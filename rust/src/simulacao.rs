//! Simulacao em lote (ver `docs/VERSAO_SEM_TELA.md` e `engenharia-reversa/simular.py`).
//!
//! `CM_PARAR_EM=AAAA-MM-DD`: na primeira chamada do sorteio depois de a data atual do jogo
//! (`Data` em 0xae2c90) alcancar o alvo, grava a impressao digital em `ddraw.log` e encerra o
//! processo. O ponto de parada e fixo na sequencia do jogo, entao duas execucoes com a mesma
//! semente (`-seed`) e as mesmas rotinas tem de dar a mesma impressao.
//!
//! Impressao digital: quantidade de sorteios desde o inicio + estado completo do gerador
//! (ponteiro da tabela, deslocamento, estado do rand() da CRT). Qualquer diferenca de
//! comportamento que mude o numero ou a ordem dos sorteios aparece aqui.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;

use crate::registro::linha;

const DATA_ATUAL: *const [u16; 2] = 0xae2c90 as *const [u16; 2]; // (dia do ano, ano)

static SORTEIOS: AtomicU64 = AtomicU64::new(0);

fn data_do_ambiente(nome: &str) -> Option<(u16, u16)> {
    let n: Vec<u16> = nome.encode_utf16().chain([0]).collect();
    let mut buf = [0u16; 32];
    let k = unsafe { crate::win::GetEnvironmentVariableW(n.as_ptr(), buf.as_mut_ptr(), buf.len() as u32) } as usize;
    let s = String::from_utf16_lossy(&buf[..k.min(32)]);
    let a = interpretar(&s);
    if let Some((ano, dia)) = a {
        linha(&format!("simulacao: {} = {} (ano {}, dia {})", nome, s, ano, dia));
    }
    a
}

/// Alvo (ano, dia do ano a partir de 0) lido de CM_PARAR_EM.
fn alvo() -> Option<(u16, u16)> {
    static ALVO: OnceLock<Option<(u16, u16)>> = OnceLock::new();
    *ALVO.get_or_init(|| data_do_ambiente("CM_PARAR_EM"))
}

/// `CM_RASTRO=AAAA-MM-DD`: cada sorteio desse dia vai para `rastro.log` (n, quem chamou, estado do
/// gerador), para achar a primeira chamada em que duas execucoes divergem.
pub fn rastro() -> Option<(u16, u16)> {
    static RASTRO: OnceLock<Option<(u16, u16)>> = OnceLock::new();
    *RASTRO.get_or_init(|| data_do_ambiente("CM_RASTRO"))
}

static ARQ_RASTRO: std::sync::Mutex<Option<std::io::BufWriter<std::fs::File>>> = std::sync::Mutex::new(None);

fn escrever_rastro(texto: &str) {
    use std::io::Write;
    let mut g = ARQ_RASTRO.lock().unwrap_or_else(|e| e.into_inner());
    if g.is_none() {
        let caminho = std::env::current_exe().map(|p| p.with_file_name("rastro.log")).unwrap_or_else(|_| "rastro.log".into());
        *g = std::fs::File::create(caminho).ok().map(std::io::BufWriter::new);
    }
    if let Some(f) = g.as_mut() {
        let _ = writeln!(f, "{}", texto);
    }
}

fn fechar_rastro() {
    use std::io::Write;
    if let Some(f) = ARQ_RASTRO.lock().unwrap_or_else(|e| e.into_inner()).as_mut() {
        let _ = f.flush();
    }
}

/// "AAAA-MM-DD" -> (ano, dia do ano a partir de 0), com a regra de bissexto do jogo.
pub fn interpretar(s: &str) -> Option<(u16, u16)> {
    let mut p = s.trim().split('-').map(|x| x.parse::<u32>().ok());
    let (a, m, d) = (p.next()??, p.next()??, p.next()??);
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) || a > 65535 {
        return None;
    }
    let acumulado = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    let extra = (m > 2 && crate::data::bissexto(a as u16) == 1) as u32;
    Some((a as u16, (acumulado[m as usize - 1] + extra + d - 1) as u16))
}

/// Chamado no inicio de cada sorteio (versao Rust e modo comparacao), com o estado do gerador antes da chamada.
/// Thread que fez o primeiro sorteio; sorteios de outras threads tornam a simulacao dependente
/// do agendamento do Windows e sao registrados.
static THREAD_PRINCIPAL: AtomicU64 = AtomicU64::new(0);
static OUTRAS_THREADS: AtomicU64 = AtomicU64::new(0);

unsafe fn conferir_thread(origem: &str) {
    let t = crate::win::GetCurrentThreadId() as u64;
    let p = match THREAD_PRINCIPAL.compare_exchange(0, t, Ordering::Relaxed, Ordering::Relaxed) {
        Ok(_) => t,
        Err(p) => p,
    };
    if t != p {
        let n = OUTRAS_THREADS.fetch_add(1, Ordering::Relaxed);
        if n < 20 || (n + 1).is_power_of_two() {
            let [d, a] = *DATA_ATUAL;
            linha(&format!("simulacao: {} na thread {} (principal {}), data {}-{:03}, {} vezes", origem, t, p, a, d, n + 1));
        }
    }
}

/// FNV-1a da tabela de pessoas (registros de 70 bytes em [0xae23c4], quantidade em [0xae236c]).
unsafe fn hash_pessoas() -> u64 {
    let (base, qtd) = (*(0xae23c4 as *const usize), *(0xae236c as *const i32));
    if base == 0 || qtd <= 0 {
        return 0;
    }
    core::slice::from_raw_parts(base as *const u8, qtd as usize * 70)
        .iter()
        .fold(0xcbf29ce484222325u64, |h, &b| (h ^ b as u64).wrapping_mul(0x100000001b3))
}

/// Mapa diario da memoria em jogo\memoria.log (com o rastro ligado): uma linha por dia com o hash
/// de cada bloco de 4 KB da .data e de 64 KB da heap. Comparando execucoes, separa o ruido (blocos
/// que variam ate entre execucoes iguais) do primeiro dado que diverge de verdade.
unsafe fn mapa_da_memoria() {
    use std::io::Write;
    static ARQ: std::sync::Mutex<Option<std::io::BufWriter<std::fs::File>>> = std::sync::Mutex::new(None);
    let mut g = ARQ.lock().unwrap_or_else(|e| e.into_inner());
    if g.is_none() {
        let caminho = std::env::current_exe().map(|p| p.with_file_name("memoria.log")).unwrap_or_default();
        *g = std::fs::File::create(caminho).ok().map(std::io::BufWriter::new);
    }
    let Some(f) = g.as_mut() else { return };
    let [d, a] = *DATA_ATUAL;
    let mut linha = format!("{}-{:03}", a, d);
    let mut b = 0x987000;
    while b < 0xde4000 {
        linha.push_str(&format!(" d{:x}:{:x}", b, hash_memoria(b, b + 0x1000) as u32));
        b += 0x1000;
    }
    let (ini, topo) = crate::alocador::regiao_do_jogo();
    let mut b = ini;
    while ini != 0 && b + 0x10000 <= topo {
        linha.push_str(&format!(" h{:x}:{:x}", b, hash_memoria(b, b + 0x10000) as u32));
        b += 0x10000;
    }
    let _ = writeln!(f, "{}", linha);
    let _ = f.flush();
}

/// Hash rapido de um trecho de memoria (palavras de 8 bytes).
unsafe fn hash_memoria(inicio: usize, fim: usize) -> u64 {
    core::slice::from_raw_parts(inicio as *const u64, (fim - inicio) / 8)
        .iter()
        .fold(0xcbf29ce484222325u64, |h, &w| (h ^ w).wrapping_mul(0x100000001b3).rotate_left(17))
}

/// Impressao das partidas: o hash de cada motor de partida ao ser liberado (o estado de fim de jogo
/// que nao chega ao sorteio, como o desgaste dos jogadores), encadeado na ordem das partidas.
static PARTIDAS: AtomicU64 = AtomicU64::new(0);
static QUANTAS: AtomicU64 = AtomicU64::new(0);

pub unsafe fn partida_terminou(motor: usize) {
    let h = hash_memoria(motor, motor + (0xf63b & !7));
    let v = PARTIDAS.load(Ordering::Relaxed);
    PARTIDAS.store(v.rotate_left(5) ^ h, Ordering::Relaxed);
    QUANTAS.fetch_add(1, Ordering::Relaxed);
}

/// A memoria do jogo, encadeada dia a dia: os blocos do banco de dados. A heap inteira mistura a
/// interface (buffers de imagem, telas), que o jogo desenha em momentos que dependem da velocidade e
/// varia entre o Rust e o original sem mudar o jogo (13/10/2026). Entram os blocos apontados pela
/// tabela de ponteiros do banco (0xae2300..0xae2400: pessoas, clubes, ...), a tabela em [[0xb67a44]]
/// e os blocos do GlobalAlloc.
static MEMORIA: AtomicU64 = AtomicU64::new(0);

pub unsafe fn blocos_do_jogo() -> Vec<(usize, usize)> {
    let mut v = crate::alocador::blocos_do_banco();
    let mut ponteiros: Vec<usize> = (0xae2300..0xae2400).step_by(4).map(|g| *(g as *const usize)).collect();
    if *(0xb67a44 as *const usize) != 0 {
        ponteiros.push(**(0xb67a44 as *const *const usize));
    }
    for p in ponteiros {
        if let Some(n) = crate::alocador::bloco(p) {
            if !v.iter().any(|b| b.0 == p) {
                v.push((p, n));
            }
        }
    }
    v
}

unsafe fn hash_da_memoria() -> u64 {
    blocos_do_jogo().iter().fold(0u64, |h, &(p, n)| h.rotate_left(7) ^ hash_memoria(p, p + (n & !7)))
}

/// O hash encadeado ate agora, com a memoria deste momento.
unsafe fn memoria_do_dia() -> u64 {
    let v = MEMORIA.load(Ordering::Relaxed).rotate_left(5) ^ hash_da_memoria();
    MEMORIA.store(v, Ordering::Relaxed);
    v
}

/// Com o rastro ligado, o diario ganha o hash de toda a memoria do jogo (heap fixa e .data do exe,
/// 0x987000..0xde463c): acha o primeiro dia em que qualquer dado diverge, antes de chegar ao sorteio.
unsafe fn hash_do_jogo() -> String {
    if rastro().is_none() {
        return String::new();
    }
    mapa_da_memoria();
    let (ini, topo) = crate::alocador::regiao_do_jogo();
    format!(" heap={:016x} dados={:016x}", if ini == 0 { 0 } else { hash_memoria(ini, topo & !7) }, hash_memoria(0x987000, 0xde4638))
}

pub unsafe fn sorteio(pedido: i32, ponteiro: u32, deslocamento: i32, rand: u32) {
    let n = SORTEIOS.fetch_add(1, Ordering::Relaxed);
    let Some((ano, dia)) = alvo() else { return };
    conferir_thread("aleatorio");
    let [d, a] = *DATA_ATUAL;
    if let Some((ra, rd)) = rastro() {
        // Primeira chamada do dia do rastro: retrato da memoria (.data inteira e hash por bloco da heap).
        static RETRATO: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        if (a, d) == (ra, rd) && !RETRATO.swap(true, Ordering::Relaxed) {
            let pasta = std::env::current_exe().map(|p| p.with_file_name("dados.bin")).unwrap_or_default();
            let _ = std::fs::write(pasta, core::slice::from_raw_parts(0x987000 as *const u8, 0xde4638 - 0x987000));
            let (ini, topo) = crate::alocador::regiao_do_jogo();
            if ini != 0 {
                let heap = std::env::current_exe().map(|p| p.with_file_name("heap.bin")).unwrap_or_default();
                let _ = std::fs::write(heap, core::slice::from_raw_parts(ini as *const u8, topo - ini));
            }
        }
        if (a, d) == (ra, rd) {
            escrever_rastro(&format!("{} n={} chamador={:#x} ponteiro={:#x} deslocamento={:#x} rand={:#x}", n, pedido, { crate::desvios::RETORNO }, ponteiro, deslocamento, rand));
        } else if (a, d) > (ra, rd) {
            fechar_rastro();
        }
    }
    // Diario: uma linha por dia do jogo, para achar onde duas execucoes comecam a divergir.
    static ULTIMO_DIA: AtomicU64 = AtomicU64::new(0);
    let hoje = (a as u64) << 16 | d as u64;
    if ULTIMO_DIA.swap(hoje, Ordering::Relaxed) != hoje {
        let mem = memoria_do_dia();
        linha(&format!("dia {}-{:03} sorteios={} ponteiro={:#x} deslocamento={:#x} rand={:#x} pessoas={:016x} memoria={:016x} relogios={:?}{}", a, d, n, ponteiro, deslocamento, rand, hash_pessoas(), mem,
            RELOGIOS.iter().map(|r| r.load(Ordering::Relaxed)).collect::<Vec<_>>(), hash_do_jogo()));
    }
    if (a, d) < (ano, dia) || a < 1900 {
        return;
    }
    linha(&format!(
        "SIMULACAO parou em {}-{:03} sorteios={} ponteiro={:#x} deslocamento={:#x} rand={:#x}",
        a, d, n, ponteiro, deslocamento, rand
    ));
    linha(&format!("partidas: {} motores, impressao {:016x}", QUANTAS.load(Ordering::Relaxed), PARTIDAS.load(Ordering::Relaxed)));
    {
        let (ini, topo) = crate::alocador::regiao_do_jogo();
        // diagnostico: com jogo\heap_fim.txt (o caminho de saida dentro), despeja a heap do jogo no fim
        // (um arquivo, e nao uma variavel de ambiente, que deslocaria a heap e mudaria a carreira)
        let marcador = std::env::current_exe().map(|p| p.with_file_name("heap_fim.txt")).unwrap_or_default();
        if let Ok(arq) = std::fs::read_to_string(marcador).map(|t| t.trim().to_string()) {
            if ini != 0 {
                let _ = std::fs::write(&arq, core::slice::from_raw_parts(ini as *const u8, topo - ini));
                linha(&format!("memoria: heap em {:#x}", ini));
            }
        }
        let _ = ini;
        let blocos = blocos_do_jogo();
        linha(&format!("memoria: banco {:016x} ({} blocos, {} KB)", memoria_do_dia(), blocos.len(), blocos.iter().map(|b| b.1).sum::<usize>() >> 10));
    }
    linha(&format!("alocador: pico {} MB", crate::alocador::pico() >> 20));
    despejar_cobertura();
    fechar_rastro();
    // Encerra sem rodar finalizacoes do jogo (nada de gravar preferencias).
    crate::win::TerminateProcess(crate::win::GetCurrentProcess(), 0);
}

// ---------- relogio fixo ----------

/// Instante devolvido pelo time() do jogo durante a simulacao (9/9/2001 01:46:40 UTC).
const RELOGIO_FIXO: u32 = 1_000_000_000;

// 0x946ebb: push ebp; mov ebp, esp; sub esp, 0xcc
const PROLOGO_TIME: crate::desvios::Prologo =
    crate::desvios::Prologo { endereco: 0x946ebb, bytes: &[0x55, 0x8b, 0xec, 0x81, 0xec, 0xcc, 0x00, 0x00, 0x00] };

/// time() da CRT { 0x946ebb } na simulacao: sempre o mesmo instante. O jogo ressemeia o rand()
/// com o relogio (manager_manager.cpp, 0x696d4b), o que tornaria a simulacao irreproduzivel.
unsafe extern "C" fn time_fixo(saida: *mut u32) -> u32 {
    if !saida.is_null() {
        *saida = RELOGIO_FIXO;
    }
    RELOGIO_FIXO
}

// 0x946a04: mov eax, dword ptr [0xad6c30]
const PROLOGO_RAND: crate::desvios::Prologo =
    crate::desvios::Prologo { endereco: 0x946a04, bytes: &[0xa1, 0x30, 0x6c, 0xad, 0x00] };
static mut RAND_ORIGINAL: usize = 0;

/// rand() da CRT observado na simulacao: so confere de que thread vem e chama o original.
unsafe extern "C" fn rand_observado() -> i32 {
    conferir_thread("rand");
    let f: unsafe extern "C" fn() -> i32 = core::mem::transmute(RAND_ORIGINAL);
    f()
}

// 0x947299: push ebp; mov ebp, esp; push ecx; push ecx
const PROLOGO_CLOCK: crate::desvios::Prologo =
    crate::desvios::Prologo { endereco: 0x947299, bytes: &[0x55, 0x8b, 0xec, 0x51, 0x51] };
/// Funcoes do jogo que chamam clock() (via 0x90c510; enderecos de retorno conferidos com refs.py).
const ROTINAS_COM_RELOGIO: [u32; 13] = [
    0x497050, 0x569610, 0x5cccd0, 0x68d520, 0x6b55c0, 0x6d7b00, 0x6d8c70, 0x71b1b0, 0x724b53, 0x725860,
    0x75db00, 0x7615d0, 0x81b890,
];
static RELOGIOS: [AtomicU64; 14] = [const { AtomicU64::new(0) }; 14];

/// clock() da CRT { 0x947299 } na simulacao: um relogio por rotina chamadora, que avanca 1 ms a
/// cada consulta dela. Um relogio unico (ou o real) faz as esperas do dia de jogo (match_day.cpp,
/// "volta quando clock() passar de X") dependerem de quantas vezes a barra de menus e as telas
/// consultaram o relogio, o que varia com as mensagens do Windows.
unsafe extern "C" fn clock_fixo() -> i32 {
    let r = crate::desvios::RETORNO;
    let i = ROTINAS_COM_RELOGIO.iter().rposition(|&f| f <= r).unwrap_or(13);
    let v = RELOGIOS[i].fetch_add(1, Ordering::Relaxed) as i32;
    let [d, a] = *DATA_ATUAL;
    if rastro() == Some((a, d)) {
        escrever_rastro(&format!("clock de {:#x} (rotina {:#x}) = {}", r, ROTINAS_COM_RELOGIO.get(i).copied().unwrap_or(0), v));
    }
    v
}

// 0x7a16d0 (RAM total, MB) e 0x7a1710 (memoria livre, MB): sub esp, 0x20; mov eax, 0x20
const PROLOGO_RAM_TOTAL: crate::desvios::Prologo =
    crate::desvios::Prologo { endereco: 0x7a16d0, bytes: &[0x83, 0xec, 0x20, 0xb8, 0x20, 0x00, 0x00, 0x00] };
const PROLOGO_RAM_LIVRE: crate::desvios::Prologo =
    crate::desvios::Prologo { endereco: 0x7a1710, bytes: &[0x83, 0xec, 0x20, 0xb8, 0x20, 0x00, 0x00, 0x00] };

// 0x7a1820 (os.cpp, espaco livre em disco em MB, saturado em 4096; 136 chamadas): sub esp, 0x2bc
const PROLOGO_DISCO: crate::desvios::Prologo =
    crate::desvios::Prologo { endereco: 0x7a1820, bytes: &[0x81, 0xec, 0xbc, 0x02, 0x00, 0x00] };

unsafe extern "C" fn disco_fixo(_unidade: usize) -> i32 {
    4096
}

/// Unidades vistas pelas 33 checagens de CD: so C:\ (pen drives e afins nao mudam nada).
unsafe extern "system" fn unidades_fixas(tam: u32, buf: *mut u8) -> u32 {
    let s = b"C:\\\0\0";
    if (tam as usize) < s.len() {
        return s.len() as u32;
    }
    core::ptr::copy_nonoverlapping(s.as_ptr(), buf, s.len());
    (s.len() - 1) as u32
}

/// Memoria vista pelo jogo na simulacao: fixa (o teto do GlobalMemoryStatus), para nao depender da maquina.
unsafe extern "C" fn memoria_fixa() -> i32 {
    2047
}

// ---------- qsort no rastro ----------

const PROLOGO_QSORT: crate::desvios::Prologo =
    crate::desvios::Prologo { endereco: 0x945333, bytes: &[0x55, 0x8b, 0xec, 0x81, 0xec, 0xf8, 0x00, 0x00, 0x00] };
static mut QSORT_ORIGINAL: usize = 0;

/// Hash do vetor com ponteiros para a tabela de pessoas trocados pelo indice (o endereco da tabela varia).
unsafe fn hash_vetor(base: *const u8, bytes: usize) -> u64 {
    let (pessoas, qtd) = (*(0xae23c4 as *const u32), *(0xae236c as *const i32) as u32);
    let fim = pessoas.wrapping_add(qtd.wrapping_mul(70));
    let mut h = 0xcbf29ce484222325u64;
    for i in 0..bytes / 4 {
        let mut v = *(base as *const u32).add(i);
        if v >= pessoas && v < fim {
            v = 0x8000_0000 | (v - pessoas);
        }
        h = (h ^ v as u64).wrapping_mul(0x100000001b3);
    }
    h
}

unsafe extern "C" fn qsort_rastreado(base: *mut u8, n: usize, largura: usize, cmp: usize) {
    let f: unsafe extern "C" fn(*mut u8, usize, usize, usize) = core::mem::transmute(QSORT_ORIGINAL);
    let [d, a] = *DATA_ATUAL;
    let no_dia = rastro() == Some((a, d));
    let antes = if no_dia { hash_vetor(base, n * largura) } else { 0 };
    f(base, n, largura, cmp);
    if no_dia {
        escrever_rastro(&format!("qsort cmp={:#x} n={} largura={} entrada={:016x} saida={:016x}", cmp, n, largura, antes, hash_vetor(base, n * largura)));
    }
}

// ---------- cobertura ----------

/// (endereco, arquivo, contador) de cada funcao contada (`CM_COBERTURA=1`, lista em cobertura.txt).
static COBERTURA: std::sync::Mutex<Vec<(usize, String, &'static mut u32)>> = std::sync::Mutex::new(Vec::new());

/// Le jogo\cobertura.txt (gerado por engenharia-reversa\cobertura.py) e poe um contador em cada funcao.
unsafe fn instalar_cobertura() {
    let caminho = std::env::current_exe().map(|p| p.with_file_name("cobertura.txt")).unwrap_or_default();
    let Ok(texto) = std::fs::read_to_string(&caminho) else {
        linha("cobertura: AVISO cobertura.txt nao encontrado (rode cobertura.py)");
        return;
    };
    let mut lista = COBERTURA.lock().unwrap_or_else(|e| e.into_inner());
    for l in texto.lines() {
        let mut c = l.split_whitespace();
        let (Some(end), Some(hex), arq) = (c.next(), c.next(), c.next().unwrap_or("")) else { continue };
        let Ok(endereco) = usize::from_str_radix(end, 16) else { continue };
        let bytes: Vec<u8> = (0..hex.len() / 2).filter_map(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).ok()).collect();
        let p = crate::desvios::Prologo { endereco, bytes: Box::leak(bytes.into_boxed_slice()) };
        let contador: &'static mut u32 = Box::leak(Box::new(0));
        if crate::desvios::contar(&format!("cobertura {:#x}", endereco), &p, contador) {
            lista.push((endereco, arq.to_string(), contador));
        }
    }
    linha(&format!("cobertura: {} funcoes com contador", lista.len()));
}

fn despejar_cobertura() {
    for (e, arq, n) in COBERTURA.lock().unwrap_or_else(|e| e.into_inner()).iter() {
        linha(&format!("cobertura {:x} {} {}", e, arq, n));
    }
}

// ---------- mensagens ----------

/// PeekMessageA do jogo na simulacao: esvazia a fila de verdade (a janela segue respondendo ao
/// Windows, inclusive a mensagens enviadas por outros programas) e responde sempre "nenhuma".
/// Roda na pilha particular (instalada pela IAT atras de `desvios::isolar`): o codigo do Windows
/// que trata as mensagens deixava lixo na pilha do jogo em momentos que dependem do relogio, e o
/// jogo le variaveis nao inicializadas (26/12/2001, treino: carreira mudava em 1 de cada 4-6
/// execucoes com o gerador identico ate o ponto da divergencia).
unsafe extern "system" fn peek_isolado(_msg: *mut [u32; 7], _hwnd: usize, _min: u32, _max: u32, _remover: u32) -> i32 {
    let mut m = [0u32; 7];
    while crate::win::PeekMessageA(&mut m, core::ptr::null_mut(), 0, 0, 1) != 0 {
        crate::win::TranslateMessage(&m);
        crate::win::DispatchMessageA(&m);
    }
    0
}

/// Toda funcao do Windows chamada pelo jogo roda na pilha particular (`desvios::isolar` em cada
/// entrada da IAT, inclusive as ja trocadas acima). O codigo do Windows deixava na pilha do jogo
/// valores que mudam a cada execucao (enderecos da heap do processo, alcas) e o jogo le variaveis
/// nao inicializadas: a carreira com o motor ligado mudava em metade das execucoes (26/12/2001,
/// treino). Assim a pilha do jogo so ve o que o proprio jogo escreve. Ficam de fora as funcoes de
/// excecao e de encerramento, que precisam da pilha real.
unsafe fn isolar_importacoes() {
    const FORA: [&str; 6] = ["RaiseException", "RtlUnwind", "ExitProcess", "TerminateProcess", "UnhandledExceptionFilter", "SetUnhandledExceptionFilter"];
    let n = crate::ganchos::para_cada_importacao(|_dll, nome, atual| {
        if nome.is_some_and(|n| FORA.contains(&n)) {
            None
        } else {
            Some(crate::desvios::isolar(atual))
        }
    });
    linha(&format!("simulacao: {} funcoes do Windows isoladas na pilha particular", n));
}

// ---------- excecoes ----------

/// Registra no ddraw.log as excecoes do processo (codigo, endereco, registradores, pilha) e deixa
/// o tratamento seguir normal. Na simulacao o jogo roda oculto e o Windows nem sempre registra a
/// queda; assim o endereco fica sempre a mao. So as 8 primeiras (excecoes tratadas tambem passam aqui).
pub unsafe extern "system" fn registrar_excecao(info: *const [usize; 2]) -> i32 {
    static VISTAS: AtomicU64 = AtomicU64::new(0);
    if VISTAS.fetch_add(1, Ordering::Relaxed) < 8 {
        let (reg, ctx) = ((*info)[0], (*info)[1]);
        let codigo = *(reg as *const u32);
        let endereco = *((reg + 12) as *const usize);
        // CONTEXT x86: edi 0x9c, esi 0xa0, ebx 0xa4, edx 0xa8, ecx 0xac, eax 0xb0, ebp 0xb4, eip 0xb8, esp 0xc4
        let r = |o: usize| *((ctx + o) as *const u32);
        let esp = r(0xc4) as usize;
        let pilha: Vec<String> = (0..12).map(|i| format!("{:x}", *((esp + 4 * i) as *const u32))).collect();
        extern "C" {
            static __ImageBase: u8;
        }
        let dll = core::ptr::addr_of!(__ImageBase) as usize;
        let onde = if endereco >= dll && endereco - dll < 0x0100_0000 { format!(" (ddraw.dll+{:#x})", endereco - dll) } else { String::new() };
        linha(&format!(
            "EXCECAO {:#x} em {:#x}{} (data {:?}) eax={:x} ebx={:x} ecx={:x} edx={:x} esi={:x} edi={:x} ebp={:x} esp={:x} pilha: {}",
            codigo, endereco, onde, *DATA_ATUAL, r(0xb0), r(0xa4), r(0xac), r(0xa8), r(0xa0), r(0x9c), r(0xb4), esp, pilha.join(" ")
        ));
    }
    0 // EXCEPTION_CONTINUE_SEARCH
}

const PROLOGO_RESTAURAR_HUMANOS: crate::desvios::Prologo =
    crate::desvios::Prologo { endereco: 0x8387e0, bytes: &[0x51, 0x53, 0x55, 0x33, 0xed] };
static mut RESTAURAR_ORIGINAL: usize = 0;

/// CM_FERIAS=1 (prova do save, etapa de carregar): o jogo nao deixa gravar com o tecnico de ferias, e um save
/// carregado com -load espera o Continuar. Na entrada da 0x8387e0 (Setup.cpp, os humanos restaurados; so o
/// -load e a rede passam por ela, depois do load_game_data) cada humano ativo (+0x5f = 1, entre as 16 ultimas
/// pessoas) sai de ferias como o ON_HOLIDAY=YES do auto_setup (0x827cc9: set_on_holiday 0x5fd2f0 com 1), e o
/// carimbo das ferias (+0x9a65, setter 0x5fdd30) volta a 0, como depois do reset 0x5fb810 da carreira
/// automatica: com o carimbo do relogio real do save, o clock fixo da simulacao nunca passaria dele + 2000 e a
/// barra lateral (0x7615d0) nao andaria sozinha. Igual nos dois modos (nao e rotina portada).
unsafe extern "C" fn restaurar_humanos_de_ferias() -> i32 {
    let hm = *(0xb63c98 as *const usize);
    if hm != 0 {
        let ferias: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x5fd2f0usize);
        let carimbo: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x5fdd30usize);
        let (base, qtd) = (*(0xae23bc as *const usize), *(0xae2364 as *const i32) as usize);
        for i in 0..16usize.min(qtd) {
            let t = base + (qtd - 1 - i) * 0x6e;
            if *((t + 0x5f) as *const u8) == 1 {
                ferias(hm, 1, t);
                carimbo(hm, 0, t);
                crate::registro::linha(&format!("CM_FERIAS: humano {:#x} de ferias depois do -load", t));
            }
        }
    }
    let f: unsafe extern "C" fn() -> i32 = core::mem::transmute(RESTAURAR_ORIGINAL);
    f()
}

/// Chamado no carregamento da DLL: so age em modo simulacao (CM_PARAR_EM definido).
pub unsafe fn instalar() {
    if alvo().is_some() {
        // Toda rotina desviada daqui em diante roda na pilha particular e grava quem chamou.
        crate::desvios::ISOLAR.store(true, Ordering::Relaxed);
        crate::win::AddVectoredExceptionHandler(1, registrar_excecao);
        crate::ganchos::trocar_importacao("USER32.dll", "PeekMessageA", peek_isolado as *const () as usize);
        crate::alocador::instalar();
        if crate::win::ambiente("CM_FERIAS") {
            if let Some(t) = crate::desvios::desviar("ferias depois do -load (CM_FERIAS)", &PROLOGO_RESTAURAR_HUMANOS,
                                                     restaurar_humanos_de_ferias as *const () as usize) {
                RESTAURAR_ORIGINAL = t;
            }
        }
        if crate::win::ambiente("CM_COBERTURA") {
            instalar_cobertura();
        }
        if rastro().is_some() {
            if let Some(t) = crate::desvios::desviar("qsort (rastro)", &PROLOGO_QSORT, qsort_rastreado as *const () as usize) {
                QSORT_ORIGINAL = t;
            }
        }
        crate::desvios::desviar("time (relogio fixo da simulacao)", &PROLOGO_TIME, time_fixo as *const () as usize);
        crate::desvios::desviar("clock (relogio da simulacao)", &PROLOGO_CLOCK, clock_fixo as *const () as usize);
        crate::desvios::desviar("disco livre (fixo na simulacao)", &PROLOGO_DISCO, disco_fixo as *const () as usize);
        crate::ganchos::trocar_importacao("KERNEL32.dll", "GetLogicalDriveStringsA", unidades_fixas as *const () as usize);
        crate::desvios::desviar("RAM total (fixa na simulacao)", &PROLOGO_RAM_TOTAL, memoria_fixa as *const () as usize);
        crate::desvios::desviar("RAM livre (fixa na simulacao)", &PROLOGO_RAM_LIVRE, memoria_fixa as *const () as usize);
        if let Some(t) = crate::desvios::desviar("rand (observado na simulacao)", &PROLOGO_RAND, rand_observado as *const () as usize) {
            RAND_ORIGINAL = t;
        }
        isolar_importacoes();
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn datas() {
        assert_eq!(interpretar("2001-01-01"), Some((2001, 0)));
        assert_eq!(interpretar("2001-08-09"), Some((2001, 220))); // a data vista na memoria do jogo
        assert_eq!(interpretar("2004-03-01"), Some((2004, 60)));
        assert_eq!(interpretar("2001-13-01"), None);
        assert_eq!(interpretar("lixo"), None);
    }
}
