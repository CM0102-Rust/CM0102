//! Desvios em rotinas do jogo: o inicio da rotina original vira um `jmp` para a versao em Rust.
//!
//! Os bytes deslocados vao para um trampolim (memoria executavel nossa) seguido de um `jmp` de
//! volta para o resto da rotina; chamar o trampolim e chamar a rotina original.
//! A tabela de prologos vem de `engenharia-reversa/desvios.py`, que confere que os bytes copiados
//! nao tem instrucao relativa nem salto do jogo para o meio deles. Antes de escrever, o nucleo
//! confere os bytes no exe carregado e recusa o desvio se nao baterem (exe diferente do esperado).
//!
//! Variaveis de ambiente: `CM_SEM_DESVIOS=1` nao instala nenhum; `CM_COMPARAR=1` roda a original e a
//! versao Rust lado a lado e registra divergencias (o jogo segue com o resultado da original).

use core::ffi::c_void;

use crate::registro::linha;
use crate::{aleatorio, data, win};

pub struct Prologo {
    pub endereco: usize,
    pub bytes: &'static [u8],
}

// ---------- pilha particular (so na simulacao) ----------
//
// O jogo le variaveis locais nao inicializadas: herda o que a rotina chamada antes deixou abaixo
// do topo da pilha. A original deixa os rascunhos dela, a versao Rust outros, as sondas outros
// ainda; com o mesmo jogo e a mesma semente a carreira mudava conforme o modo (dia 2001-093,
// identica com o rastro ligado e diferente sem ele). Na simulacao toda rotina desviada roda numa
// pilha nossa: a pilha do jogo fica intocada e o lixo que ele encontra e o mesmo em todos os modos.

/// Endereco de retorno da ultima chamada do jogo a uma rotina desviada (gravado na entrada).
pub static mut RETORNO: u32 = 0;
static mut ESP_DO_JOGO: u32 = 0;
const PILHA: usize = 1 << 20;
const ARGUMENTOS: u32 = 16; // dwords copiados da pilha do jogo (sobra para qualquer rotina desviada)

/// Liga a pilha particular para os desvios instalados daqui em diante.
pub static ISOLAR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Trecho de entrada: grava o retorno; se ja estiver na pilha particular (chamada aninhada), so
/// salta; senao troca de pilha, copia os argumentos, chama `destino` e volta descontando os
/// argumentos que a rotina retirou (vale para cdecl, stdcall e thiscall; ecx = this e preservado).
pub unsafe fn isolar(destino: usize) -> usize {
    static TOPO: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
    let topo = *TOPO.get_or_init(|| {
        // Endereco fixo, logo abaixo das regioes do alocador: enderecos de pilha nao podem variar.
        let p = win::VirtualAlloc(0x0ff0_0000 as *mut c_void, PILHA, win::MEM_COMMIT | win::MEM_RESERVE, win::PAGE_READWRITE) as usize;
        if p == 0 { 0 } else { (p + PILHA - 16) as u32 }
    });
    let codigo = win::VirtualAlloc(core::ptr::null_mut(), 256, win::MEM_COMMIT | win::MEM_RESERVE, win::PAGE_EXECUTE_READWRITE) as usize;
    if topo == 0 || codigo == 0 {
        linha("desvios: AVISO sem pilha particular");
        return destino;
    }
    let base = topo + 16 - PILHA as u32;
    let le = |v: u32| v.to_le_bytes();
    let (retorno, esp_jogo) = (&raw mut RETORNO as u32, &raw mut ESP_DO_JOGO as u32);
    let mut c: Vec<u8> = Vec::new();
    let rel = |c: &Vec<u8>, alvo: usize| ((alvo as isize - (codigo + c.len() + 4) as isize) as i32).to_le_bytes();
    c.extend([0x8b, 0x04, 0x24, 0xa3]); // mov eax, [esp]; mov [RETORNO], eax
    c.extend(le(retorno));
    c.extend([0x81, 0xfc]); // cmp esp, base
    c.extend(le(base));
    c.extend([0x72, 0x00]); // jb fora
    let jb = c.len() - 1;
    c.extend([0x81, 0xfc]); // cmp esp, topo
    c.extend(le(topo + 16));
    c.extend([0x73, 0x00]); // jae fora
    let jae = c.len() - 1;
    c.push(0xe9); // jmp destino (aninhada)
    let r = rel(&c, destino);
    c.extend(r);
    let fora = c.len();
    c[jb] = (fora - jb - 1) as u8;
    c[jae] = (fora - jae - 1) as u8;
    c.extend([0x89, 0xe0, 0xa3]); // mov eax, esp; mov [ESP_DO_JOGO], eax
    c.extend(le(esp_jogo));
    c.push(0xbc); // mov esp, topo
    c.extend(le(topo));
    for i in (1..=ARGUMENTOS).rev() {
        c.extend([0xff, 0x70, (4 * i) as u8]); // push dword [eax + 4i]
    }
    c.push(0xe8); // call destino
    let r = rel(&c, destino);
    c.extend(r);
    c.extend([0x89, 0xe1, 0x81, 0xe9]); // mov ecx, esp; sub ecx, topo - 4*ARGUMENTOS  (= bytes retirados)
    c.extend(le(topo - 4 * ARGUMENTOS));
    c.extend([0x03, 0x0d]); // add ecx, [ESP_DO_JOGO]
    c.extend(le(esp_jogo));
    c.extend([0x89, 0xcc, 0x8b, 0x0d]); // mov esp, ecx; mov ecx, [ESP_DO_JOGO]
    c.extend(le(esp_jogo));
    c.extend([0x8b, 0x09, 0x89, 0x0c, 0x24, 0xc3]); // mov ecx, [ecx]; mov [esp], ecx; ret
    assert!(c.len() <= 256);
    core::ptr::copy_nonoverlapping(c.as_ptr(), codigo as *mut u8, c.len());
    codigo
}

/// `destino` especial: a propria rotina original (so faz sentido com a pilha particular, para a
/// original rodar isolada como as reescritas e os modos da simulacao serem comparaveis).
pub const PASSAGEM: usize = 0;

/// Desvia `p.endereco` para `destino`. Devolve o trampolim (a rotina original), ou None se recusou.
pub unsafe fn desviar(nome: &str, p: &Prologo, destino: usize) -> Option<usize> {
    desviar_para(nome, p, |tramp| {
        let destino = if destino == PASSAGEM { tramp } else { destino };
        if ISOLAR.load(std::sync::atomic::Ordering::Relaxed) { isolar(destino) } else { destino }
    })
}

/// Desvia `p.endereco` para `destino` (ou a propria original, com PASSAGEM) passando antes por um
/// trecho que zera `n` bytes da pilha logo abaixo da entrada. Original determinizado (15/10/2026): as
/// rotinas que leem variavel local sem valor passam a ler zero em todos os modos (a sobra da pilha
/// variava com o modo). Na simulacao o trecho roda ja na pilha particular.
pub unsafe fn desviar_zerando(nome: &str, p: &Prologo, n: u32, destino: usize) -> Option<usize> {
    desviar_para(nome, p, |tramp| {
        let destino = if destino == PASSAGEM { tramp } else { destino };
        let z = zerar_e_saltar(n, destino);
        if ISOLAR.load(std::sync::atomic::Ordering::Relaxed) { isolar(z) } else { z }
    })
}

/// `push edi; push ecx; push eax; lea edi, [esp - 12 - n]; mov ecx, n/4; xor eax, eax; rep stosd;
/// pop eax; pop ecx; pop edi; jmp destino`: zera [esp_entrada - n - 12, esp_entrada - 12) sem tocar
/// registradores nem argumentos.
unsafe fn zerar_e_saltar(n: u32, destino: usize) -> usize {
    let codigo = win::VirtualAlloc(core::ptr::null_mut(), 64, win::MEM_COMMIT | win::MEM_RESERVE, win::PAGE_EXECUTE_READWRITE) as usize;
    let mut c: Vec<u8> = vec![0x57, 0x51, 0x50, 0x8d, 0xbc, 0x24];
    c.extend((-(n as i32)).to_le_bytes()); // lea edi, [esp - n] (esp ja desceu 12)
    c.push(0xb9);
    c.extend((n / 4).to_le_bytes());
    c.extend([0x31, 0xc0, 0xfc, 0xf3, 0xab, 0x58, 0x59, 0x5f]); // xor eax, eax; cld; rep stosd; pop; pop; pop
    // os 12 bytes logo abaixo da entrada guardaram edi, ecx e eax durante a zeragem: zera-os tambem
    // (mov dword [esp-4/-8/-12], 0), senao ficam os registradores de quem chamou (a save_database grava esse
    // trecho no index.dat; prova do save da 219a, 23/10/2026)
    for d in [0xfcu8, 0xf8, 0xf4] {
        c.extend([0xc7, 0x44, 0x24, d, 0, 0, 0, 0]);
    }
    c.push(0xe9); // jmp
    let rel = (destino as isize - (codigo + c.len() + 4) as isize) as i32;
    c.extend(rel.to_le_bytes());
    core::ptr::copy_nonoverlapping(c.as_ptr(), codigo as *mut u8, c.len());
    codigo
}

/// Contador de chamadas (cobertura): `inc dword [contador]; jmp original`. Nao toca pilha nem
/// registradores (so os flags, que nao sobrevivem a um call), entao a carreira nao muda.
pub unsafe fn contar(nome: &str, p: &Prologo, contador: *mut u32) -> bool {
    desviar_para(nome, p, |tramp| {
        let c = win::VirtualAlloc(core::ptr::null_mut(), 16, win::MEM_COMMIT | win::MEM_RESERVE, win::PAGE_EXECUTE_READWRITE) as *mut u8;
        *c = 0xff;
        *c.add(1) = 0x05;
        core::ptr::write_unaligned(c.add(2) as *mut u32, contador as u32);
        escrever_jmp(c.add(6), tramp);
        c as usize
    })
    .is_some()
}

// ---------- assinaturas: a rotina inteira, nao so o prologo ----------
//
// Rotina da tabela ASSINATURAS (as do motor) cuja faixa inteira em memoria nao e a do exe original nem
// a do cm0102-1080.exe fica com o exe: outro patch (NickPatcher, CLZ...) mudou o codigo e o dele roda.

static CONFERIDAS: AtomicU32 = AtomicU32::new(0);
static COM_O_EXE: AtomicU32 = AtomicU32::new(0);
static MICROS: AtomicU32 = AtomicU32::new(0);
use std::sync::atomic::AtomicU32;

/// false = a rotina esta na tabela e os bytes nao batem com nenhuma das duas versoes.
unsafe fn assinatura_confere(endereco: usize) -> bool {
    use crate::assinaturas::ASSINATURAS;
    use std::sync::atomic::Ordering::Relaxed;
    let Ok(k) = ASSINATURAS.binary_search_by_key(&endereco, |a| a.0) else { return true };
    let t = std::time::Instant::now();
    let (_, n, original, r1080) = ASSINATURAS[k];
    let mut h: u64 = 0xcbf29ce484222325; // FNV-1a 64 (o mesmo do assinaturas.py)
    for &b in core::slice::from_raw_parts(endereco as *const u8, n) {
        h = (h ^ b as u64).wrapping_mul(0x100000001b3);
    }
    MICROS.fetch_add(t.elapsed().as_micros() as u32, Relaxed);
    CONFERIDAS.fetch_add(1, Relaxed);
    let ok = h == original || h == r1080;
    if !ok {
        COM_O_EXE.fetch_add(1, Relaxed);
    }
    ok
}

/// A linha final da conferencia no ddraw.log.
pub fn resumo_assinaturas() {
    use std::sync::atomic::Ordering::Relaxed;
    linha(&format!("assinaturas: {} conferidas, {} ficam com o exe ({} us)",
        CONFERIDAS.load(Relaxed), COM_O_EXE.load(Relaxed), MICROS.load(Relaxed)));
}

/// Miolo dos desvios: confere a rotina e os bytes, monta o trampolim e salta para `destino(trampolim)`.
unsafe fn desviar_para(nome: &str, p: &Prologo, destino: impl FnOnce(usize) -> usize) -> Option<usize> {
    if !assinatura_confere(p.endereco) {
        linha(&format!("rotina {:#x} fica com o exe (alterada)", p.endereco));
        return None;
    }
    let atual = core::slice::from_raw_parts(p.endereco as *const u8, p.bytes.len());
    if atual != p.bytes {
        linha(&format!("desvio {} recusado: bytes em {:#x} sao {:02x?}, esperava {:02x?}", nome, p.endereco, atual, p.bytes));
        return None;
    }
    let n = p.bytes.len();
    let tramp = win::VirtualAlloc(core::ptr::null_mut(), 32, win::MEM_COMMIT | win::MEM_RESERVE, win::PAGE_EXECUTE_READWRITE) as *mut u8;
    if tramp.is_null() {
        linha(&format!("desvio {} recusado: VirtualAlloc falhou", nome));
        return None;
    }
    let m = montar_trampolim(p, tramp);
    escrever_jmp(tramp.add(m), p.endereco + n);
    let destino = destino(tramp as usize);

    let mut antiga = 0u32;
    win::VirtualProtect(p.endereco as *mut c_void, n, win::PAGE_EXECUTE_READWRITE, &mut antiga);
    escrever_jmp(p.endereco as *mut u8, destino);
    for i in 5..n {
        *(p.endereco as *mut u8).add(i) = 0xcc; // nunca executados (o jmp sai antes)
    }
    win::VirtualProtect(p.endereco as *mut c_void, n, antiga, &mut antiga);
    win::FlushInstructionCache(win::GetCurrentProcess(), p.endereco as *const c_void, n);
    linha(&format!("desvio {}: {:#x} -> {:#x} (original em {:#x})", nome, p.endereco, destino, tramp as usize));
    Some(tramp as usize)
}

/// Prologos que terminam numa instrucao relativa (`call rel32` ou `jcc rel8`), com a posicao dela
/// nos bytes: o trampolim refaz a instrucao com o destino absoluto. `engenharia-reversa/prologos.py`
/// le esta tabela e aceita a instrucao relativa so nessa posicao.
pub const RELATIVAS: &[(usize, usize)] = &[(0x714d00, 4), (0x6f55c0, 4)];

/// Copia o prologo para o trampolim, refazendo a instrucao relativa de RELATIVAS. Devolve o tamanho
/// escrito (o `jmp` de volta vem depois).
unsafe fn montar_trampolim(p: &Prologo, tramp: *mut u8) -> usize {
    let n = p.bytes.len();
    let Some(&(_, k)) = RELATIVAS.iter().find(|&&(a, _)| a == p.endereco) else {
        core::ptr::copy_nonoverlapping(p.bytes.as_ptr(), tramp, n);
        return n;
    };
    core::ptr::copy_nonoverlapping(p.bytes.as_ptr(), tramp, k);
    let op = p.bytes[k];
    if op == 0xe8 {
        let rel = i32::from_le_bytes([p.bytes[k + 1], p.bytes[k + 2], p.bytes[k + 3], p.bytes[k + 4]]);
        let alvo = (p.endereco as isize + k as isize + 5 + rel as isize) as usize;
        *tramp.add(k) = 0xe8;
        core::ptr::write_unaligned(tramp.add(k + 1) as *mut i32, (alvo as isize - (tramp as isize + k as isize + 5)) as i32);
        core::ptr::copy_nonoverlapping(p.bytes.as_ptr().add(k + 5), tramp.add(k + 5), n - k - 5);
        n
    } else {
        // jcc rel8 vira jcc rel32 (0f 8x): 4 bytes a mais
        assert!((0x70..0x80).contains(&op));
        let alvo = (p.endereco as isize + k as isize + 2 + p.bytes[k + 1] as i8 as isize) as usize;
        *tramp.add(k) = 0x0f;
        *tramp.add(k + 1) = 0x80 + (op - 0x70);
        core::ptr::write_unaligned(tramp.add(k + 2) as *mut i32, (alvo as isize - (tramp as isize + k as isize + 6)) as i32);
        core::ptr::copy_nonoverlapping(p.bytes.as_ptr().add(k + 2), tramp.add(k + 6), n - k - 2);
        n + 4
    }
}

unsafe fn escrever_jmp(onde: *mut u8, para: usize) {
    *onde = 0xe9;
    let rel = (para as isize - (onde as isize + 5)) as i32;
    core::ptr::write_unaligned(onde.add(1) as *mut i32, rel);
}

/// Instala todos os desvios. Chamado no carregamento da DLL, antes do WinMain do jogo.
pub unsafe fn instalar() {
    if win::ambiente("CM_SEM_DESVIOS") {
        linha("desvios: desligados (CM_SEM_DESVIOS)");
        return;
    }
    // Simulacao em lote (CM_PARAR_EM): relogio fixo, para a simulacao ser reproduzivel.
    crate::simulacao::instalar();
    // CM_ORIGINAL=1: nenhuma rotina substituida; o sorteio so e observado (contagem e parada da
    // simulacao), para comparar uma simulacao so com as originais contra outra com o Rust.
    if win::ambiente("CM_ORIGINAL") {
        aleatorio::instalar(aleatorio::Modo::Observar);
        if ISOLAR.load(std::sync::atomic::Ordering::Relaxed) {
            data::instalar_passagem();
            crate::motor::instalar_passagem();
        }
        crate::motor::instalar_zeradas();
        resumo_assinaturas();
        return;
    }
    // CM_AUTOTESTE=1 liga o modo comparacao; os autotestes em si rodam depois (ver `autotestes`).
    // Uma queda no autoteste fica registrada no ddraw.log (endereco e registradores).
    if win::ambiente("CM_AUTOTESTE") {
        win::AddVectoredExceptionHandler(1, crate::simulacao::registrar_excecao);
    }
    let comparar = win::ambiente("CM_COMPARAR") || win::ambiente("CM_AUTOTESTE");
    aleatorio::instalar(if comparar { aleatorio::Modo::Comparar } else { aleatorio::Modo::Rust });
    data::instalar(comparar);
    crate::motor::instalar(comparar);
    crate::motor::instalar_zeradas();
    resumo_assinaturas();
}

/// CM_AUTOTESTE=1: cada rotina reescrita se compara com a original em entradas sinteticas
/// (resultado em ddraw.log). `CM_AUTOTESTE=motor:N` roda so o motor, com N casos (as mutacoes de
/// uma leva do motor nao mexem no aleatorio nem na data). Chamado na primeira DirectDrawCreate: a essa altura o WinMain ja
/// esta rodando e a CRT do jogo esta inicializada (algumas rotinas chamam atexit, malloc...).
pub unsafe fn autotestes() {
    static FEITO: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    if !win::ambiente("CM_AUTOTESTE") || FEITO.swap(true, std::sync::atomic::Ordering::Relaxed) {
        return;
    }
    let valor = win::texto_ambiente("CM_AUTOTESTE");
    if let Some(n) = valor.strip_prefix("motor:") {
        crate::motor::autoteste(n.parse().unwrap_or(2_000_000));
        return;
    }
    // CM_AUTOTESTE=banco: só os autotestes do banco (mutacoes.py --autoteste-banco); o motor com 1 caso fecha
    // com a linha "motor: AUTOTESTE" que o mutacoes.py espera
    if valor == "banco" {
        crate::motor::autoteste_dbj_1();
        crate::motor::autoteste_dbj_2();
        crate::motor::autoteste_dbm_1();
        crate::motor::autoteste_dbm_2();
        crate::motor::autoteste(1);
        return;
    }
    // os conversores da versão 1 e os nomes em inglês (232ª): nem a carreira nem a prova do save passam por eles
    crate::motor::autoteste_dbj_1();
    crate::motor::autoteste_dbj_2();
    crate::motor::autoteste_dbm_1();
    crate::motor::autoteste_dbm_2();
    aleatorio::autoteste(5_000_000);
    data::autoteste();
    crate::motor::autoteste(2_000_000);
}
