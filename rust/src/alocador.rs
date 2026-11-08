//! Heap reproduzivel para a simulacao em lote (so com CM_PARAR_EM).
//!
//! O Windows muda a cada execucao o endereco da heap e, com o alocador de baixa fragmentacao, a
//! ordem dos blocos pequenos. O jogo desempata algumas decisoes pela ordem dos ponteiros (no dia
//! 2001-093 a escalacao mudava com o gerador identico), entao a mesma semente nao dava a mesma
//! carreira. Aqui as funcoes Heap* do exe (IAT) passam a usar uma regiao de endereco fixo com um
//! alocador deterministico: mesma sequencia de pedidos => mesmos enderecos.
//!
//! A memoria da DLL que o jogo enxerga (objetos DirectDraw, superficies e seus pixels) vai para
//! uma segunda regiao fixa (`visivel`): o jogo guarda ponteiros para ela e o endereco na heap do
//! processo variava (dia 2001-271). O resto da DLL vai para uma terceira regiao fixa, so dela (`REGIAO_DLL`):
//! longe da do jogo, para que comparacoes, placares e registro (que mudam com o modo) nao desloquem o que o
//! jogo ve, e em endereco fixo, porque os ponteiros dela ficam na pilha particular (01/11/2026).
//!
//! Blocos: cabecalho de 16 bytes {capacidade, tamanho pedido} + dados alinhados em 16.
//! Ate 4 KB: classes de 16 em 16 bytes; acima: classes por pagina (ate 256 MB). Cada classe tem
//! uma lista de livres encadeada dentro dos proprios blocos (o ultimo liberado sai primeiro; nada
//! e alocado para controle, o que permite servir de alocador global); sem livre, avanca o topo.
//! Os livres vizinhos so se juntam quando a regiao esgota (`alocar_juntando`).

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::Mutex;

use crate::registro::linha;

const CABECALHO: usize = 16;
const PEQUENO: usize = 4096;
const REGIAO: usize = 0x4000_0000; // 1 GB reservado e comprometido
const BASES: [usize; 1] = [0x1800_0000];

const HEAP_ZERO_MEMORY: u32 = 0x08;
const HEAP_REALLOC_IN_PLACE_ONLY: u32 = 0x10;

pub struct Alocador {
    inicio: usize,
    topo: usize,
    fim: usize,
    livres: [usize; CLASSES], // cabeca da lista de livres de cada classe (0 = vazia)
    pico: usize,
    /// (livres, topo) do ultimo `juntar`, para quem chamou registrar depois de soltar a trava (registrar aloca, e o
    /// alocador da DLL nao pode alocar com a propria trava na mao: o Mutex nao e reentrante)
    juntou: Option<(usize, usize)>,
}

const CLASSES: usize = PEQUENO / 16 + 65536 + 1;

fn capacidade(n: usize) -> usize {
    let n = n.max(1);
    if n <= PEQUENO { (n + 15) & !15 } else { (n + 4095) & !4095 }
}

/// Classe de uma capacidade (None: grande demais para reaproveitar).
fn classe(cap: usize) -> Option<usize> {
    let c = if cap <= PEQUENO { cap / 16 } else { PEQUENO / 16 + cap / 4096 };
    (c < CLASSES).then_some(c)
}

impl Alocador {
    pub const fn novo(inicio: usize, tamanho: usize) -> Self {
        Alocador { inicio, topo: inicio, fim: inicio + tamanho, livres: [0; CLASSES], pico: 0, juntou: None }
    }

    /// Passa a usar a regiao dada (sem mover a estrutura, que e grande para a pilha).
    fn ativar(&mut self, inicio: usize, tamanho: usize) {
        (self.inicio, self.topo, self.fim) = (inicio, inicio, inicio + tamanho);
    }

    fn ativo(&self) -> bool {
        self.fim != 0
    }

    pub unsafe fn alocar(&mut self, n: usize, zerar: bool) -> usize {
        let cap = capacidade(n);
        let livre = classe(cap).map_or(0, |c| self.livres[c]);
        let p = match livre {
            p if p != 0 => {
                self.livres[classe(cap).unwrap()] = *(p as *const usize);
                p
            }
            _ => {
                if self.fim - self.topo < CABECALHO + cap {
                    // so com a regiao esgotada (as carreiras que cabem nela nao mudam): junta os livres e tenta de novo
                    return self.alocar_juntando(n, cap, zerar);
                }
                let p = self.topo + CABECALHO;
                self.topo += CABECALHO + cap;
                if self.topo - self.inicio > self.pico {
                    self.pico = self.topo - self.inicio;
                }
                p
            }
        };
        *((p - 16) as *mut usize) = cap;
        *((p - 12) as *mut usize) = n;
        if zerar {
            core::ptr::write_bytes(p as *mut u8, 0, n);
        }
        p
    }

    /// Regiao esgotada: junta os blocos livres vizinhos (os do fim voltam ao topo) e serve o pedido pelo topo ou
    /// pelo menor livre que caiba, partindo o resto. Deterministico (pela ordem dos enderecos). A carreira ampla
    /// (8 ligas) esgotava 1 GB em abril de 2002: a lista de estatisticas dos jogadores cresce de 100 em 100 e as
    /// copias antigas nunca serviam de novo (27/10/2026).
    /// ponytail: varre todas as listas a cada esgotamento; se ficar lento, juntar so depois de N bytes liberados.
    unsafe fn alocar_juntando(&mut self, n: usize, cap: usize, zerar: bool) -> usize {
        let mut p = self.menor_livre(cap);
        if p == 0 {
            self.juntar();
            if self.fim - self.topo >= CABECALHO + cap {
                p = self.topo + CABECALHO;
                self.topo += CABECALHO + cap;
                *((p - 16) as *mut usize) = cap;
            } else {
                p = self.menor_livre(cap);
            }
        }
        if p != 0 {
            *((p - 12) as *mut usize) = n;
            if zerar {
                core::ptr::write_bytes(p as *mut u8, 0, n);
            }
        }
        p
    }

    /// O menor livre de capacidade >= `cap` (o primeiro de cada lista), partido se sobrar; 0 se nenhum.
    unsafe fn menor_livre(&mut self, cap: usize) -> usize {
        let Some(c0) = classe(cap) else { return 0 };
        let Some(c) = (c0..CLASSES).find(|&c| self.livres[c] != 0 && *((self.livres[c] - 16) as *const usize) >= cap) else {
            return 0;
        };
        let p = self.livres[c];
        self.livres[c] = *(p as *const usize);
        let total = *((p - 16) as *const usize);
        if total - cap >= CABECALHO + 16 {
            let resto = p + cap + CABECALHO;
            *((resto - 16) as *mut usize) = total - cap - CABECALHO;
            self.liberar(resto);
            *((p - 16) as *mut usize) = cap;
        }
        p
    }

    /// Junta os livres vizinhos; os que chegam ao topo voltam para ele. Nao passa pelo alocador global (a lista vai
    /// direto na heap do Windows) nem registra: serve tambem a regiao da DLL, que roda com a propria trava na mao.
    unsafe fn juntar(&mut self) {
        let mut n = 0;
        for c in 0..CLASSES {
            let mut p = self.livres[c];
            while p != 0 {
                n += 1;
                p = *(p as *const usize);
            }
        }
        let layout = Layout::array::<usize>(n.max(1)).unwrap();
        let buf = System.alloc(layout) as *mut usize;
        if buf.is_null() {
            return;
        }
        let livres = core::slice::from_raw_parts_mut(buf, n);
        let mut i = 0;
        for c in 0..CLASSES {
            let mut p = self.livres[c];
            while p != 0 {
                livres[i] = p;
                i += 1;
                p = *(p as *const usize);
            }
            self.livres[c] = 0;
        }
        livres.sort_unstable();
        // vizinhos juntos no lugar: o primeiro de cada sequencia fica com a capacidade somada
        let mut k = 0;
        for i in 0..n {
            let (p, c) = (livres[i], *((livres[i] - 16) as *const usize));
            if k > 0 {
                let q = livres[k - 1];
                let cq = *((q - 16) as *const usize);
                if q + cq == p - CABECALHO {
                    *((q - 16) as *mut usize) = cq + CABECALHO + c;
                    continue;
                }
            }
            livres[k] = p;
            k += 1;
        }
        if k > 0 && livres[k - 1] + *((livres[k - 1] - 16) as *const usize) == self.topo {
            self.topo = livres[k - 1] - CABECALHO;
            k -= 1;
        }
        for i in (0..k).rev() {
            self.liberar(livres[i]);
        }
        System.dealloc(buf as *mut u8, layout);
        self.juntou = Some((k, self.topo));
    }

    /// Registra o aviso do ultimo `juntar` (chamar depois de soltar a trava).
    fn registrar_juntou(aviso: Option<(usize, usize)>, qual: &str) {
        if let Some((k, topo)) = aviso {
            linha(&format!("alocador: regiao {} esgotada; {} livres depois de juntar, topo em {:#x}", qual, k, topo));
        }
    }

    pub unsafe fn liberar(&mut self, p: usize) {
        if let Some(c) = classe(*((p - 16) as *const usize)) {
            *(p as *mut usize) = self.livres[c];
            self.livres[c] = p;
        }
    }

    pub unsafe fn tamanho(&self, p: usize) -> usize {
        *((p - 12) as *const usize)
    }

    pub unsafe fn realocar(&mut self, p: usize, n: usize, flags: u32) -> usize {
        let (cap, antigo) = (*((p - 16) as *const usize), self.tamanho(p));
        if capacidade(n) <= cap {
            // cabe no mesmo bloco
            if flags & HEAP_ZERO_MEMORY != 0 && n > antigo {
                core::ptr::write_bytes((p + antigo) as *mut u8, 0, n - antigo);
            }
            *((p - 12) as *mut usize) = n;
            return p;
        }
        if flags & HEAP_REALLOC_IN_PLACE_ONLY != 0 {
            return 0;
        }
        let q = self.alocar(n, flags & HEAP_ZERO_MEMORY != 0);
        if q != 0 {
            core::ptr::copy_nonoverlapping(p as *const u8, q as *mut u8, antigo.min(n));
            self.liberar(p);
        }
        q
    }

    pub fn contem(&self, p: usize) -> bool {
        p >= self.inicio && p < self.fim
    }
}

static HEAP: Mutex<Alocador> = Mutex::new(Alocador::novo(0, 0));
static mut ORIGINAIS: [usize; 6] = [0; 6]; // Create, Alloc, Free, ReAlloc, Size, Destroy
static mut ALCA: usize = 0; // "alca" devolvida ao HeapCreate (= inicio da regiao)

fn com<R>(f: impl FnOnce(&mut Alocador) -> R) -> R {
    let mut g = HEAP.lock().unwrap_or_else(|e| e.into_inner());
    let r = f(&mut g);
    let aviso = g.juntou.take();
    drop(g);
    Alocador::registrar_juntou(aviso, "do jogo");
    r
}

unsafe fn nosso(h: usize) -> bool {
    h == ALCA && ALCA != 0
}

unsafe extern "system" fn heap_create(_flags: u32, _inicial: usize, _maximo: usize) -> usize {
    ALCA
}
unsafe extern "system" fn heap_alloc(h: usize, flags: u32, n: usize) -> usize {
    if !nosso(h) {
        let f: unsafe extern "system" fn(usize, u32, usize) -> usize = core::mem::transmute(ORIGINAIS[1]);
        return f(h, flags, n);
    }
    let p = com(|a| a.alocar(n, flags & HEAP_ZERO_MEMORY != 0));
    if p == 0 {
        linha(&format!("alocador: regiao esgotada pedindo {} bytes", n));
    }
    p
}
unsafe extern "system" fn heap_free(h: usize, flags: u32, p: usize) -> i32 {
    if !nosso(h) {
        let f: unsafe extern "system" fn(usize, u32, usize) -> i32 = core::mem::transmute(ORIGINAIS[2]);
        return f(h, flags, p);
    }
    if p != 0 {
        // o motor de partida (0xf63b bytes, 0xf640 com o arredondamento do CRT; jogadores em campo inclusive) entra na impressao das partidas
        if com(|a| a.tamanho(p)) == 0xf640 {
            crate::simulacao::partida_terminou(p);
        }
        com(|a| a.liberar(p));
    }
    1
}
unsafe extern "system" fn heap_realloc(h: usize, flags: u32, p: usize, n: usize) -> usize {
    if !nosso(h) {
        let f: unsafe extern "system" fn(usize, u32, usize, usize) -> usize = core::mem::transmute(ORIGINAIS[3]);
        return f(h, flags, p, n);
    }
    com(|a| a.realocar(p, n, flags))
}
unsafe extern "system" fn heap_size(h: usize, flags: u32, p: usize) -> usize {
    if !nosso(h) {
        let f: unsafe extern "system" fn(usize, u32, usize) -> usize = core::mem::transmute(ORIGINAIS[4]);
        return f(h, flags, p);
    }
    com(|a| a.tamanho(p))
}
unsafe extern "system" fn heap_destroy(h: usize) -> i32 {
    if !nosso(h) {
        let f: unsafe extern "system" fn(usize) -> i32 = core::mem::transmute(ORIGINAIS[5]);
        return f(h);
    }
    1
}

/// RoleBitEffect usada da regiao do jogo (inicio, topo), para o hash diario da simulacao.
pub fn regiao_do_jogo() -> (usize, usize) {
    let a = HEAP.lock().unwrap_or_else(|e| e.into_inner());
    (a.inicio, a.topo)
}

/// Pico de uso da regiao (bytes), para o registro da simulacao.
pub fn pico() -> usize {
    HEAP.lock().unwrap_or_else(|e| e.into_inner()).pico
}

// ---------- alocador global da DLL ----------

const REGIAO_VISIVEL: usize = 0x0800_0000; // 128 MB, logo abaixo da regiao do jogo
const BASES_VISIVEL: [usize; 1] = [0x1000_0000];
static VISIVEL: Mutex<Alocador> = Mutex::new(Alocador::novo(0, 0));
static NA_VISIVEL: AtomicBool = AtomicBool::new(false);

/// Aloca o que `f` criar na regiao visivel ao jogo (quando a simulacao a ligou).
/// The `visivel` pair for the hook of `cm0102-base` (the video layer's allocations).
pub fn entrar_visivel() -> bool {
    NA_VISIVEL.swap(true, Ordering::Relaxed)
}

pub fn sair_visivel(antes: bool) {
    NA_VISIVEL.store(antes, Ordering::Relaxed);
}

#[allow(dead_code)]
pub fn visivel<R>(f: impl FnOnce() -> R) -> R {
    let antes = NA_VISIVEL.swap(true, Ordering::Relaxed);
    let r = f();
    NA_VISIVEL.store(antes, Ordering::Relaxed);
    r
}

// O resto da memoria da DLL (Vec, String, format!, Box: registro, diario, comparacoes, fitas) tambem vai para uma
// regiao fixa na simulacao, so dela e da thread que carregou a DLL (a do jogo). Na heap do processo, que muda de
// endereco a cada execucao, esses ponteiros ficavam na pilha particular e uma rotina original que le variavel sem
// valor os encontrava: a carreira mudava entre execucoes iguais (item cqc_6, 01/11/2026). Como a heap do jogo e a
// regiao visivel, nao escreve nada na pilha: so fixa onde a memoria da DLL fica. Fora da simulacao, nada muda. O que
// nao couber (regiao esgotada mesmo depois de juntar) vai para a heap do Windows, com AVISO.
// O modo comparar fica na heap do Windows: as fotos e fitas dele chegam a 143 MB vivos ate 2002-01-01 (a carreira Rust,
// 96 KB), nao ha espaco baixo livre com folga para isso, e o criterio dele e a divergencia, nao a impressao.
const REGIAO_DLL: usize = 0x0400_0000; // 64 MB; a carreira ate 2002-06-01 usa 145 KB
const BASES_DLL: [usize; 1] = [0x0b00_0000]; // ate 0x0f000000, abaixo da pilha particular (0x0ff00000)
static DLL: Mutex<Alocador> = Mutex::new(Alocador::novo(0, 0));
static DLL_FAIXA: [AtomicUsize; 2] = [AtomicUsize::new(0), AtomicUsize::new(0)]; // (inicio, fim): o dealloc confere sem trava
static DLL_THREAD: AtomicU32 = AtomicU32::new(0);
static DLL_ESGOTOU: AtomicBool = AtomicBool::new(false);
static DLL_VIVOS: [AtomicUsize; 2] = [AtomicUsize::new(0), AtomicUsize::new(0)]; // (bytes vivos agora, maximo)

/// Alocador global da DLL: heap do processo; na simulacao, a regiao visivel dentro de `visivel(...)` e a regiao da
/// DLL no resto.
pub struct Global;

unsafe fn na_visivel(l: Layout, zerar: bool) -> Option<*mut u8> {
    if !NA_VISIVEL.load(Ordering::Relaxed) || l.align() > 16 {
        return None;
    }
    let mut a = VISIVEL.lock().unwrap_or_else(|e| e.into_inner());
    a.ativo().then(|| a.alocar(l.size(), zerar) as *mut u8)
}

unsafe fn na_dll(l: Layout, zerar: bool) -> Option<*mut u8> {
    let t = DLL_THREAD.load(Ordering::Relaxed);
    if t == 0 || l.align() > 16 || crate::win::GetCurrentThreadId() != t {
        return None;
    }
    let mut a = DLL.lock().unwrap_or_else(|e| e.into_inner());
    let p = a.alocar(l.size(), zerar);
    let aviso = a.juntou.take();
    drop(a);
    Alocador::registrar_juntou(aviso, "da DLL");
    if p == 0 && !DLL_ESGOTOU.swap(true, Ordering::Relaxed) {
        linha(&format!("alocador: AVISO regiao da DLL esgotada pedindo {} bytes; o que nao couber vai para a heap do Windows", l.size()));
    }
    if p == 0 {
        return None;
    }
    let vivos = DLL_VIVOS[0].fetch_add(l.size(), Ordering::Relaxed) + l.size();
    DLL_VIVOS[1].fetch_max(vivos, Ordering::Relaxed);
    Some(p as *mut u8)
}

unsafe impl GlobalAlloc for Global {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        na_visivel(l, false).or_else(|| na_dll(l, false)).unwrap_or_else(|| System.alloc(l))
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        na_visivel(l, true).or_else(|| na_dll(l, true)).unwrap_or_else(|| System.alloc_zeroed(l))
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        let mut a = VISIVEL.lock().unwrap_or_else(|e| e.into_inner());
        if a.contem(p as usize) {
            a.liberar(p as usize);
            return;
        }
        drop(a);
        if (DLL_FAIXA[0].load(Ordering::Relaxed)..DLL_FAIXA[1].load(Ordering::Relaxed)).contains(&(p as usize)) {
            DLL.lock().unwrap_or_else(|e| e.into_inner()).liberar(p as usize);
            DLL_VIVOS[0].fetch_sub(l.size(), Ordering::Relaxed);
        } else {
            System.dealloc(p, l)
        }
    }
}

/// Pico de uso da regiao da DLL e maximo de bytes vivos nela (0 sem ela), para o registro da simulacao. O pico passa dos
/// vivos porque os livres so se juntam com a regiao esgotada.
pub fn pico_dll() -> (usize, usize) {
    (DLL.lock().unwrap_or_else(|e| e.into_inner()).pico, DLL_VIVOS[1].load(Ordering::Relaxed))
}

unsafe fn reservar(bases: &[usize], tamanho: usize) -> usize {
    for &b in bases {
        let p = crate::win::VirtualAlloc(b as *mut _, tamanho, crate::win::MEM_COMMIT | crate::win::MEM_RESERVE, crate::win::PAGE_READWRITE);
        if !p.is_null() {
            if p as usize != bases[0] {
                linha(&format!("alocador: AVISO regiao em {:#x} e nao em {:#x}", p as usize, bases[0]));
            }
            return p as usize;
        }
    }
    0
}

// ---------- VirtualAlloc do exe ----------
//
// O alocador de blocos pequenos da CRT do VC6 (SBH) reserva regioes de 1 MB com VirtualAlloc(NULL)
// e o Windows escolhe o endereco (dia 2001-254: ponteiros em 0x045xxxxx mudando entre execucoes).
// Reservas sem endereco vao para enderecos sequenciais numa faixa fixa.

const FAIXA_VIRTUAL: (usize, usize) = (0x5800_0000, 0x5f00_0000);
static PROXIMO_VIRTUAL: Mutex<usize> = Mutex::new(FAIXA_VIRTUAL.0);
static mut VIRTUAL_ALLOC_ORIGINAL: usize = 0;

unsafe extern "system" fn virtual_alloc(end: usize, tam: usize, tipo: u32, prot: u32) -> usize {
    let f: unsafe extern "system" fn(usize, usize, u32, u32) -> usize = core::mem::transmute(VIRTUAL_ALLOC_ORIGINAL);
    if end != 0 || tipo & crate::win::MEM_RESERVE == 0 {
        return f(end, tam, tipo, prot);
    }
    let mut prox = PROXIMO_VIRTUAL.lock().unwrap_or_else(|e| e.into_inner());
    while *prox + tam <= FAIXA_VIRTUAL.1 {
        let candidato = *prox;
        *prox += (tam + 0xffff) & !0xffff;
        let r = f(candidato, tam, tipo, prot);
        if r != 0 {
            return r;
        }
    }
    linha("alocador: AVISO band fixa do VirtualAlloc esgotada; endereco do Windows");
    f(0, tam, tipo, prot)
}

// ---------- GlobalAlloc do exe ----------
//
// Database.cpp aloca o banco de dados inteiro com GlobalAlloc (0x529cdf) e o jogo guarda ponteiros
// para dentro dele (dia 2001-254). O exe so importa GlobalAlloc/GlobalFree (sem GlobalLock), ou
// seja, sempre GMEM_FIXED: a alca e o proprio ponteiro.

const GMEM_ZEROINIT: u32 = 0x40;

/// Os blocos vivos do GlobalAlloc (o banco de dados do jogo), para a impressao da memoria.
static BANCO: Mutex<Vec<(usize, usize)>> = Mutex::new(Vec::new());

/// O tamanho do bloco que comeca em `p` na regiao fixa, se `p` parece mesmo o inicio de um bloco
/// (cabecalho coerente: capacidade de classe, nao menor que a do tamanho pedido; o realocar mantem
/// a capacidade ao encolher).
pub unsafe fn bloco(p: usize) -> Option<usize> {
    let (ini, topo) = regiao_do_jogo();
    if p < ini + CABECALHO || p >= topo || p & 15 != 0 {
        return None;
    }
    let (cap, n) = (*((p - 16) as *const usize), *((p - 12) as *const usize));
    (n > 0 && cap >= capacidade(n) && capacidade(cap) == cap && p + cap <= topo).then_some(n)
}

pub fn blocos_do_banco() -> Vec<(usize, usize)> {
    BANCO.lock().unwrap_or_else(|e| e.into_inner()).clone()
}

unsafe extern "system" fn global_alloc(flags: u32, n: usize) -> usize {
    let p = com(|a| a.alocar(n, flags & GMEM_ZEROINIT != 0));
    if p != 0 {
        BANCO.lock().unwrap_or_else(|e| e.into_inner()).push((p, n));
    }
    p
}
unsafe extern "system" fn global_free(p: usize) -> usize {
    if p != 0 {
        BANCO.lock().unwrap_or_else(|e| e.into_inner()).retain(|b| b.0 != p);
        com(|a| a.liberar(p));
    }
    0
}

/// Reserva a regiao fixa e troca as Heap* do exe. Chamado no carregamento da DLL, antes da CRT do jogo.
pub unsafe fn instalar() {
    let inicio = reservar(&BASES, REGIAO);
    if inicio == 0 {
        linha("alocador: AVISO nenhuma regiao fixa livre; heap do Windows (simulacao nao reproduzivel)");
        return;
    }
    let vis = reservar(&BASES_VISIVEL, REGIAO_VISIVEL);
    if vis != 0 {
        VISIVEL.lock().unwrap_or_else(|e| e.into_inner()).ativar(vis, REGIAO_VISIVEL);
        linha(&format!("alocador: memoria visivel da DLL em regiao fixa {:#x}..{:#x}", vis, vis + REGIAO_VISIVEL));
    }
    let comparar = crate::win::ambiente("CM_COMPARAR");
    let dll = if comparar { 0 } else { reservar(&BASES_DLL, REGIAO_DLL) };
    if comparar {
        linha("alocador: modo comparar, o resto da memoria da DLL fica na heap do Windows");
    } else if dll != 0 {
        DLL.lock().unwrap_or_else(|e| e.into_inner()).ativar(dll, REGIAO_DLL);
        DLL_FAIXA[0].store(dll, Ordering::Relaxed);
        DLL_FAIXA[1].store(dll + REGIAO_DLL, Ordering::Relaxed);
        DLL_THREAD.store(crate::win::GetCurrentThreadId(), Ordering::Relaxed);
        linha(&format!("alocador: resto da memoria da DLL em regiao fixa {:#x}..{:#x}", dll, dll + REGIAO_DLL));
    } else {
        linha("alocador: AVISO regiao da DLL ocupada; o resto da memoria da DLL fica na heap do Windows");
    }
    HEAP.lock().unwrap_or_else(|e| e.into_inner()).ativar(inicio, REGIAO);
    ALCA = inicio;
    let trocas: [(&str, usize); 6] = [
        ("HeapCreate", heap_create as *const () as usize),
        ("HeapAlloc", heap_alloc as *const () as usize),
        ("HeapFree", heap_free as *const () as usize),
        ("HeapReAlloc", heap_realloc as *const () as usize),
        ("HeapSize", heap_size as *const () as usize),
        ("HeapDestroy", heap_destroy as *const () as usize),
    ];
    crate::ganchos::trocar_importacao("KERNEL32.dll", "GlobalAlloc", global_alloc as *const () as usize);
    crate::ganchos::trocar_importacao("KERNEL32.dll", "GlobalFree", global_free as *const () as usize);
    match crate::ganchos::trocar_importacao("KERNEL32.dll", "VirtualAlloc", virtual_alloc as *const () as usize) {
        Some(a) => VIRTUAL_ALLOC_ORIGINAL = a,
        None => linha("alocador: AVISO VirtualAlloc nao trocado"),
    }
    for (i, (nome, nova)) in trocas.iter().enumerate() {
        match crate::ganchos::trocar_importacao("KERNEL32.dll", nome, *nova) {
            Some(a) => ORIGINAIS[i] = a,
            None => linha(&format!("alocador: {} nao trocado", nome)),
        }
    }
    linha(&format!("alocador: regiao fixa {:#x}..{:#x}", inicio, inicio + REGIAO));
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn reproduzivel_e_reaproveita() {
        let mut mem = vec![0u8; 1 << 20];
        let base = (mem.as_mut_ptr() as usize + 15) & !15;
        let rodar = |base: usize| unsafe {
            let mut a = Alocador::novo(base, (1 << 20) - 16);
            let x = a.alocar(10, false);
            let y = a.alocar(5000, true);
            assert_eq!((x % 16, a.tamanho(x), a.tamanho(y)), (0, 10, 5000));
            a.liberar(x);
            let z = a.alocar(16, false); // mesma classe de x: reaproveita
            assert_eq!(z, x);
            *(z as *mut u8) = 7;
            let w = a.realocar(z, 12, 0); // cabe: mesmo bloco
            assert_eq!(w, z);
            let v = a.realocar(w, 100, HEAP_ZERO_MEMORY); // nao cabe: copia
            assert_ne!(v, w);
            assert_eq!(*(v as *const u8), 7);
            assert_eq!(*((v + 99) as *const u8), 0);
            assert_eq!(a.realocar(v, 5000, HEAP_REALLOC_IN_PLACE_ONLY), 0);
            assert_eq!(a.alocar(1 << 21, false), 0); // esgotada
            vec![x - base, y - base, z - base, v - base]
        };
        assert_eq!(rodar(base), rodar(base));
    }

    #[test]
    fn esgotada_junta_os_livres() {
        let mut mem = vec![0u8; 1 << 20];
        let base = (mem.as_mut_ptr() as usize + 15) & !15;
        unsafe {
            let mut a = Alocador::novo(base, (1 << 20) - 16);
            let fixo = a.alocar(100, false); // no comeco, nunca liberado
            // a lista que cresce de 3200 em 3200 bytes (como a das estatisticas dos jogadores): sem juntar, esgota
            let mut p = a.alocar(3200, false);
            *(p as *mut u32) = 0x1234;
            for k in 2..300 {
                p = a.realocar(p, 3200 * k, 0);
                assert_ne!(p, 0, "esgotou em {}", k);
                assert_eq!(*(p as *const u32), 0x1234);
                if 3200 * (k + 1) > (1 << 18) { // sem juntar, esgota perto de k = 25
                    break;
                }
            }
            assert_eq!(a.tamanho(fixo), 100);
        }
    }
}
