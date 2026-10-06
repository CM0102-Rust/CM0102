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
unsafe fn isolar(destino: usize) -> usize {
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
    core::ptr::copy_nonoverlapping(p.bytes.as_ptr(), tramp, n);
    escrever_jmp(tramp.add(n), p.endereco + n);
    let destino = if destino == PASSAGEM { tramp as usize } else { destino };
    let destino = if ISOLAR.load(std::sync::atomic::Ordering::Relaxed) { isolar(destino) } else { destino };

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
        }
        return;
    }
    // CM_AUTOTESTE=1 liga o modo comparacao; os autotestes em si rodam depois (ver `autotestes`).
    let comparar = win::ambiente("CM_COMPARAR") || win::ambiente("CM_AUTOTESTE");
    aleatorio::instalar(if comparar { aleatorio::Modo::Comparar } else { aleatorio::Modo::Rust });
    data::instalar(comparar);
}

/// CM_AUTOTESTE=1: cada rotina reescrita se compara com a original em entradas sinteticas
/// (resultado em ddraw.log). Chamado na primeira DirectDrawCreate: a essa altura o WinMain ja
/// esta rodando e a CRT do jogo esta inicializada (algumas rotinas chamam atexit, malloc...).
pub unsafe fn autotestes() {
    static FEITO: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    if !win::ambiente("CM_AUTOTESTE") || FEITO.swap(true, std::sync::atomic::Ordering::Relaxed) {
        return;
    }
    aleatorio::autoteste(5_000_000);
    data::autoteste();
}
