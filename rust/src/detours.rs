//! Detours in game routines: the start of the original routine becomes a `jmp` to the Rust version.
//!
//! The displaced bytes go to a trampoline (executable memory of ours) followed by a `jmp` back
//! to the rest of the routine; calling the trampoline is calling the original routine.
//! The prologue table comes from `engenharia-reversa/desvios.py`, which checks that the copied bytes
//! have no relative instruction and no jump from the game into the middle of them. Before writing, the core
//! checks the bytes in the loaded exe and refuses the detour if they do not match (exe different from expected).
//!
//! Environment variables: `CM_SEM_DESVIOS=1` installs none; `CM_COMPARAR=1` runs the original and the
//! Rust version side by side and logs divergences (the game goes on with the original's result).
//!
//! [pt] Desvios em rotinas do jogo: o inicio da rotina original vira um `jmp` para a versao em Rust.
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
use crate::{rng, date, win};

pub struct Prologo {
    pub endereco: usize,
    pub bytes: &'static [u8],
}

// ---------- private stack (only in the simulation) ----------
//
// The game reads uninitialized local variables: it inherits what the routine called before left below
// the top of the stack. The original leaves its scratch, the Rust version other values, the probes others
// still; with the same game and the same seed the career changed with the mode (day 2001-093,
// identical with the trace on and different without it). In the simulation every detoured routine runs on a
// stack of ours: the game's stack stays untouched and the garbage it finds is the same in all modes.

/// Return address of the game's last call to a detoured routine (written on entry).
///
/// [pt] Endereco de retorno da ultima chamada do jogo a uma rotina desviada (gravado na entrada).
pub static mut RETORNO: u32 = 0;
static mut GAME_ESP: u32 = 0;
/// The game's ECX on entry: third-party patch code (NickPatcher) relies on ECX being preserved after
/// calling a routine that preserves it (0x90cfc0 does push ecx/pop ecx); the exit stub used ECX and the
/// patch's 9-iteration loop at 0x602610 never reached 0 (CLZ, 25/10/2026).
///
/// [pt] O ECX do jogo na entrada: o código de patch de terceiros (NickPatcher) conta com o ECX preservado depois de
/// chamar uma rotina que o preserva (a 0x90cfc0 faz push ecx/pop ecx); o trecho de saída usava o ECX e o laço de
/// 9 voltas do patch em 0x602610 nunca chegava a 0 (CLZ, 25/10/2026).
static mut GAME_ECX: u32 = 0;
const STACK: usize = 1 << 20;
const STACK_BASE: usize = 0x0ff0_0000;
const ARG_DWORDS: u32 = 16; // dwords copied from the game's stack (enough for any detoured routine)

/// Switches on the private stack for detours installed from here on.
///
/// [pt] Liga a pilha particular para os desvios instalados daqui em diante.
pub static ISOLAR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Garbage detector (diagnosis only, the chain's third Rust career; `jogo\veneno.txt`, read by
/// `simulacao::instalar`): when nonzero, every entry from outside into a detoured routine fills the POISON_SPAN bytes
/// at the top of the private stack with that value before calling the routine. An original that reads a local variable
/// without a value then reads the poison, and the career comes out different from the two normal ones. It is not part of
/// the product and decides nothing: it only flags (see docs/COMO_CONTINUAR.md).
///
/// [pt] Detector de lixo (so diagnostico, a terceira carreira Rust da cadeia; `jogo\veneno.txt`, lido por
/// `simulacao::instalar`): diferente de 0, toda entrada vinda de fora numa rotina desviada enche os VENENO_FAIXA bytes
/// do alto da pilha particular com esse valor antes de chamar a rotina. Uma original que le variavel local sem valor
/// passa a ler o veneno, e a carreira sai diferente das duas normais. Nao entra no produto nem decide nada: so acusa
/// (ver docs/COMO_CONTINUAR.md).
pub static VENENO: AtomicU32 = AtomicU32::new(0);
static mut SAVED_EDI: u32 = 0;
/// The career with the engine uses up to 33 KB of the private stack (measured up to 2001-09-01) and enters it ~158 million
/// times up to 2002-06-01: filling 1 MB on every entry made the career 6 times slower. simplification: fixed span; if the
/// line "pilha particular usou ate" goes past it, increase it.
///
/// [pt] A carreira com motor usa ate 33 KB da pilha particular (medido ate 2001-09-01) e entra nela ~158 milhoes de vezes ate
/// 2002-06-01: encher 1 MB a cada entrada deixava a carreira 6 vezes mais lenta. simplification: faixa fixa; se a linha
/// "pilha particular usou ate" passar dela, aumentar.
const POISON_SPAN: u32 = 64 << 10;

// ---------- executable memory for the stubs ----------
//
// Trampolines and stubs (16 to 256 bytes each) come from blocks of BLOCK bytes reserved at once. Before, each one was
// a VirtualAlloc(NULL), and Windows reserves 64 KB per call: with ~1,570 detours (2 per routine in the simulation) that was
// ~200 MB of address space, from 0x03440000 up to near the private stack (01/11/2026).
const BLOCK: usize = 2 << 20; // ~7,000 detours in the simulation (288 bytes each); when used up, reserve another
static STUBS: std::sync::Mutex<(usize, usize, usize, usize)> = std::sync::Mutex::new((0, 0, 0, 0)); // (free, end, blocks, used)

/// `n` bytes of executable memory (aligned to 16; 0 if Windows refuses).
///
/// [pt] `n` bytes de memoria executavel (alinhados em 16; 0 se o Windows recusar).
unsafe fn executable(n: usize) -> usize {
    let n = (n + 15) & !15;
    let mut g = STUBS.lock().unwrap_or_else(|e| e.into_inner());
    if g.1 - g.0 < n {
        let b = win::VirtualAlloc(core::ptr::null_mut(), BLOCK, win::MEM_COMMIT | win::MEM_RESERVE, win::PAGE_EXECUTE_READWRITE) as usize;
        if b == 0 {
            return 0;
        }
        (g.0, g.1, g.2) = (b, b + BLOCK, g.2 + 1);
    }
    let p = g.0;
    g.0 += n;
    g.3 += n;
    p
}

/// The stubs line in ddraw.log: how many blocks, how much used and where the last one is.
///
/// [pt] A linha dos trechos no ddraw.log: quantos blocos, quanto usado e onde esta o ultimo.
pub fn resumo_trechos() {
    let g = *STUBS.lock().unwrap_or_else(|e| e.into_inner());
    if ISOLAR.load(std::sync::atomic::Ordering::Relaxed) {
        // the private stack starts zeroed: the lowest nonzero dword is the deepest ever written (with the detector,
        // the poisoned span); past POISON_SPAN, increase the span
        let mut p = STACK_BASE;
        while p < STACK_BASE + STACK && unsafe { *(p as *const u32) } == 0 {
            p += 4;
        }
        linha(&format!("desvios: pilha particular usou ate {} KB (detector enche {} KB)", (STACK_BASE + STACK - p) >> 10, POISON_SPAN >> 10));
    }
    linha(&format!("desvios: trechos em {} bloco(s) de {} KB, {} KB usados, ultimo bloco em {:#x}..{:#x}",
        g.2, BLOCK >> 10, g.3 >> 10, g.1.saturating_sub(BLOCK), g.1));
}

/// Entry stub: writes the return address; if already on the private stack (nested call), it just
/// jumps; otherwise switches stacks, copies the arguments, calls `target` and returns discounting the
/// arguments the routine popped (works for cdecl, stdcall and thiscall; ecx = this is preserved).
///
/// [pt] Trecho de entrada: grava o retorno; se ja estiver na pilha particular (chamada aninhada), so
/// salta; senao troca de pilha, copia os argumentos, chama `destino` e volta descontando os
/// argumentos que a rotina retirou (vale para cdecl, stdcall e thiscall; ecx = this e preservado).
pub unsafe fn isolar(target: usize) -> usize {
    isolate_with(target, true)
}

/// The same for Windows functions (IAT entries): the garbage detector does not fill the stack before them, because
/// Windows code does not read the game's garbage and the next detoured routine coming from outside poisons again.
///
/// [pt] O mesmo para as funcoes do Windows (entradas da IAT): o detector de lixo nao enche a pilha antes delas, porque o
/// codigo do Windows nao le o lixo do jogo e a proxima rotina desviada vinda de fora envenena de novo.
pub unsafe fn isolar_importacao(target: usize) -> usize {
    isolate_with(target, false)
}

unsafe fn isolate_with(target: usize, poison: bool) -> usize {
    static TOP: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
    let top = *TOP.get_or_init(|| {
        // Fixed address, just below the allocator's regions: stack addresses must not vary.
        let p = win::VirtualAlloc(STACK_BASE as *mut c_void, STACK, win::MEM_COMMIT | win::MEM_RESERVE, win::PAGE_READWRITE) as usize;
        if p == 0 { 0 } else { (p + STACK - 16) as u32 }
    });
    let code = executable(256);
    if top == 0 || code == 0 {
        linha("desvios: AVISO sem pilha particular");
        return target;
    }
    let base = top + 16 - STACK as u32;
    let le = |v: u32| v.to_le_bytes();
    let (ret_slot, game_esp) = (&raw mut RETORNO as u32, &raw mut GAME_ESP as u32);
    let mut c: Vec<u8> = Vec::new();
    let rel = |c: &Vec<u8>, target: usize| ((target as isize - (code + c.len() + 4) as isize) as i32).to_le_bytes();
    c.extend([0x8b, 0x04, 0x24, 0xa3]); // mov eax, [esp]; mov [RETORNO], eax
    c.extend(le(ret_slot));
    c.extend([0x81, 0xfc]); // cmp esp, base
    c.extend(le(base));
    c.extend([0x72, 0x00]); // jb outside
    let jb = c.len() - 1;
    c.extend([0x81, 0xfc]); // cmp esp, top
    c.extend(le(top + 16));
    c.extend([0x73, 0x00]); // jae outside
    let jae = c.len() - 1;
    c.push(0xe9); // jmp target (nested)
    let r = rel(&c, target);
    c.extend(r);
    let outside = c.len();
    c[jb] = (outside - jb - 1) as u8;
    c[jae] = (outside - jae - 1) as u8;
    c.push(0x89); // mov [GAME_ECX], ecx
    c.push(0x0d);
    c.extend(le(&raw mut GAME_ECX as u32));
    c.extend([0x89, 0xe0, 0xa3]); // mov eax, esp; mov [GAME_ESP], eax
    c.extend(le(game_esp));
    let poison_value = if poison { VENENO.load(std::sync::atomic::Ordering::Relaxed) } else { 0 };
    if poison_value != 0 {
        // detector: mov [SAVED_EDI], edi; mov edi, top - POISON_SPAN; mov ecx, POISON_SPAN/4; mov eax, poison; cld;
        // rep stosd; mov edi, [SAVED_EDI]; mov ecx, [GAME_ECX]; mov eax, [GAME_ESP]
        c.extend([0x89, 0x3d]);
        c.extend(le(&raw mut SAVED_EDI as u32));
        c.push(0xbf);
        c.extend(le(top - POISON_SPAN));
        c.push(0xb9);
        c.extend(le(POISON_SPAN / 4));
        c.push(0xb8);
        c.extend(le(poison_value));
        c.extend([0xfc, 0xf3, 0xab, 0x8b, 0x3d]);
        c.extend(le(&raw mut SAVED_EDI as u32));
        c.extend([0x8b, 0x0d]);
        c.extend(le(&raw mut GAME_ECX as u32));
        c.push(0xa1);
        c.extend(le(game_esp));
    }
    c.push(0xbc); // mov esp, top
    c.extend(le(top));
    for i in (1..=ARG_DWORDS).rev() {
        c.extend([0xff, 0x70, (4 * i) as u8]); // push dword [eax + 4i]
    }
    c.push(0xe8); // call target
    let r = rel(&c, target);
    c.extend(r);
    c.extend([0x89, 0xe1, 0x81, 0xe9]); // mov ecx, esp; sub ecx, top - 4*ARG_DWORDS  (= bytes popped)
    c.extend(le(top - 4 * ARG_DWORDS));
    c.extend([0x03, 0x0d]); // add ecx, [GAME_ESP]
    c.extend(le(game_esp));
    c.extend([0x89, 0xcc, 0x8b, 0x0d]); // mov esp, ecx; mov ecx, [GAME_ESP]
    c.extend(le(game_esp));
    c.extend([0x8b, 0x09, 0x89, 0x0c, 0x24, 0x8b, 0x0d]); // mov ecx, [ecx]; mov [esp], ecx; mov ecx, [GAME_ECX]
    c.extend(le(&raw mut GAME_ECX as u32));
    c.push(0xc3); // ret
    assert!(c.len() <= 256);
    core::ptr::copy_nonoverlapping(c.as_ptr(), code as *mut u8, c.len());
    code
}

// Stack of the ECX values and return addresses saved by the `preservar_ecx` stub (nested calls).
// simplification: a single stack, for the main thread (the game calls the detoured routines on it); 512 levels.
static mut ECX_STACK: [u32; 1024] = [0; 1024];
static mut ECX_TOP: u32 = 0;
static mut SAVED_EAX: u32 = 0;

/// Stub around `target` that gives ECX back as it was at the call (the Rust copies and the exit path
/// do not preserve it; the game's compiled code does not rely on that, but the third-party patch does). Swaps the
/// return address for ours, saves ECX and the return address on ECX_STACK and, on the way back, restores both; EAX (the
/// result) passes untouched. The depth of the game's stack does not change.
///
/// [pt] Trecho em volta de `destino` que devolve o ECX como estava na chamada (as cópias Rust e o caminho de saída
/// não o preservam; o código compilado do jogo não conta com isso, mas o patch de terceiros conta). Troca o
/// endereço de retorno pelo nosso, guarda ECX e retorno na PILHA_ECX e, na volta, restaura os dois; EAX (o
/// resultado) passa intacto. A profundidade da pilha do jogo não muda.
pub unsafe fn preservar_ecx(target: usize) -> usize {
    let code = executable(96);
    if code == 0 {
        return target;
    }
    if ECX_TOP == 0 {
        ECX_TOP = &raw mut ECX_STACK as u32;
    }
    let le = |v: u32| v.to_le_bytes();
    let (top, saved) = (&raw mut ECX_TOP as u32, &raw mut SAVED_EAX as u32);
    let mut c: Vec<u8> = Vec::new();
    c.push(0xa1); c.extend(le(top)); // mov eax, [ECX_TOP]
    c.extend([0x89, 0x08, 0x8b, 0x0c, 0x24, 0x89, 0x48, 0x04]); // mov [eax], ecx; mov ecx, [esp]; mov [eax+4], ecx
    c.extend([0x83, 0xc0, 0x08, 0xa3]); c.extend(le(top)); // add eax, 8; mov [ECX_TOP], eax
    c.extend([0xc7, 0x04, 0x24]); // mov dword [esp], after
    let after_at = c.len();
    c.extend(le(0));
    c.extend([0x8b, 0x48, 0xf8]); // mov ecx, [eax-8] (the this of thiscalls)
    c.push(0xe9); // jmp target
    let r = ((target as isize - (code + c.len() + 4) as isize) as i32).to_le_bytes();
    c.extend(r);
    let after = code + c.len();
    c[after_at..after_at + 4].copy_from_slice(&le(after as u32));
    c.push(0xa3); c.extend(le(saved)); // mov [SAVED_EAX], eax
    c.push(0xa1); c.extend(le(top)); // mov eax, [ECX_TOP]
    c.extend([0x83, 0xe8, 0x08, 0xa3]); c.extend(le(top)); // sub eax, 8; mov [ECX_TOP], eax
    c.extend([0x8b, 0x08, 0xff, 0x70, 0x04]); // mov ecx, [eax]; push dword [eax+4]
    c.push(0xa1); c.extend(le(saved)); // mov eax, [SAVED_EAX]
    c.push(0xc3); // ret
    assert!(c.len() <= 96);
    core::ptr::copy_nonoverlapping(c.as_ptr(), code as *mut u8, c.len());
    code
}

/// The same for a cdecl routine with `n` arguments, without swapping the return address: push ecx; copy the
/// arguments; call; add esp; pop ecx; ret. Keeps calls and returns matched (the return swap of
/// `preservar_ecx` defeats the processor's return prediction and doubled the career's time, with the draw
/// 0x90cfc0 called billions of times).
///
/// [pt] O mesmo para uma rotina cdecl de `n` argumentos, sem trocar o endereço de retorno: push ecx; copia os
/// argumentos; call; add esp; pop ecx; ret. Fica com as chamadas e retornos casados (a troca do retorno do
/// `preservar_ecx` desfaz a previsão de retorno do processador e dobrava o tempo da carreira, com o sorteio
/// 0x90cfc0 chamado bilhões de vezes).
pub unsafe fn preservar_ecx_cdecl(target: usize, n: u8) -> usize {
    let code = executable(64);
    if code == 0 {
        return target;
    }
    let mut c: Vec<u8> = vec![0x51]; // push ecx
    for _ in 0..n {
        c.extend([0xff, 0x74, 0x24, 4 * n + 4]); // push dword [esp + 4n + 4] (the last argument first)
    }
    c.push(0xe8); // call target
    let r = ((target as isize - (code + c.len() + 4) as isize) as i32).to_le_bytes();
    c.extend(r);
    c.extend([0x83, 0xc4, 4 * n, 0x59, 0xc3]); // add esp, 4n; pop ecx; ret
    core::ptr::copy_nonoverlapping(c.as_ptr(), code as *mut u8, c.len());
    code
}

/// Special `target`: the original routine itself (only makes sense with the private stack, so the
/// original runs isolated like the rewritten ones and the simulation modes are comparable).
///
/// [pt] `destino` especial: a propria rotina original (so faz sentido com a pilha particular, para a
/// original rodar isolada como as reescritas e os modos da simulacao serem comparaveis).
pub const PASSAGEM: usize = 0;

/// Detours `p.endereco` to `target`. Returns the trampoline (the original routine), or None if refused.
///
/// [pt] Desvia `p.endereco` para `destino`. Devolve o trampolim (a rotina original), ou None se recusou.
pub unsafe fn desviar(name: &str, p: &Prologo, target: usize) -> Option<usize> {
    hook_with(name, p, |tramp| {
        let target = if target == PASSAGEM { tramp } else { target };
        // only in the patched exe (third-party code relies on ECX; the game's does not, and the generic stub doubles the
        // career's time); the draw (1 argument, cdecl, the most called) gets the cheap stub
        let target = if !exe_com_patch() { target }
            else if p.endereco == 0x90cfc0 { preservar_ecx_cdecl(target, 1) } else { preservar_ecx(target) };
        if ISOLAR.load(std::sync::atomic::Ordering::Relaxed) { isolar(target) } else { target }
    })
}

/// Stack probe detour (`CM_SONDA_PILHA`, diagnosis): the original runs on the game's stack between the probe stubs.
///
/// [pt] Desvio da sonda de pilha (`CM_SONDA_PILHA`, diagnóstico): a original roda na pilha do jogo entre os trechos da sonda.
pub unsafe fn desviar_sonda(name: &str, p: &Prologo, inside: bool) -> Option<usize> {
    hook_with(name, p, |tramp| crate::stack_probe::montar(tramp, inside))
}

/// Call counter (coverage): `inc dword [counter]; jmp original`. Touches neither stack nor
/// registers (only the flags, which do not survive a call), so the career does not change.
///
/// [pt] Contador de chamadas (cobertura): `inc dword [contador]; jmp original`. Nao toca pilha nem
/// registradores (so os flags, que nao sobrevivem a um call), entao a carreira nao muda.
pub unsafe fn contar(name: &str, p: &Prologo, counter: *mut u32) -> bool {
    hook_with(name, p, |tramp| {
        let c = executable(16) as *mut u8;
        *c = 0xff;
        *c.add(1) = 0x05;
        core::ptr::write_unaligned(c.add(2) as *mut u32, counter as u32);
        write_jmp(c.add(6), tramp);
        c as usize
    })
    .is_some()
}

// ---------- signatures: the whole routine, not just the prologue ----------
//
// A routine of the ASSINATURAS table (the engine's) whose whole range in memory is neither the original exe's nor
// the cm0102-1080.exe's stays with the exe: another patch (NickPatcher, CLZ...) changed the code and its code runs.

static CHECKED: AtomicU32 = AtomicU32::new(0);
static LEFT_TO_EXE: AtomicU32 = AtomicU32::new(0);
static MICROS: AtomicU32 = AtomicU32::new(0);
use std::sync::atomic::AtomicU32;

// The verdicts come from a snapshot taken at the start of the installation (`conferir_assinaturas`), before any detour
// of ours: a port installed before the engine changed the range's bytes and our own check refused the routine
// (0x5afe00 in comparing mode, the post-match port; 25/10/2026).
static mut VERDICTS: Vec<bool> = Vec::new();

/// The snapshot: the verdict of each ASSINATURAS line with the current bytes (call before detouring anything).
///
/// [pt] A foto: o veredicto de cada linha de ASSINATURAS com os bytes de agora (chamar antes de desviar qualquer coisa).
pub unsafe fn conferir_assinaturas() {
    use crate::assinaturas::ASSINATURAS;
    let v: Vec<bool> = ASSINATURAS.iter().map(|a| check_range(a.0)).collect();
    VERDICTS = v;
}

/// false = the routine is in the table and the bytes match neither of the two versions.
///
/// [pt] false = a rotina esta na tabela e os bytes nao batem com nenhuma das duas versoes.
unsafe fn signature_matches(address: usize) -> bool {
    use crate::assinaturas::ASSINATURAS;
    let Ok(k) = ASSINATURAS.binary_search_by_key(&address, |a| a.0) else { return true };
    let v = &*core::ptr::addr_of!(VERDICTS);
    if k < v.len() { v[k] } else { check_range(address) }
}

unsafe fn check_range(address: usize) -> bool {
    use crate::assinaturas::ASSINATURAS;
    use std::sync::atomic::Ordering::Relaxed;
    let Ok(k) = ASSINATURAS.binary_search_by_key(&address, |a| a.0) else { return true };
    let t = std::time::Instant::now();
    let (_, n, original, r1080) = ASSINATURAS[k];
    let mut h: u64 = 0xcbf29ce484222325; // FNV-1a 64 (the same as assinaturas.py)
    for &b in core::slice::from_raw_parts(address as *const u8, n) {
        h = (h ^ b as u64).wrapping_mul(0x100000001b3);
    }
    MICROS.fetch_add(t.elapsed().as_micros() as u32, Relaxed);
    CHECKED.fetch_add(1, Relaxed);
    let ok = h == original || h == r1080;
    if !ok {
        LEFT_TO_EXE.fetch_add(1, Relaxed);
    }
    ok
}

/// The final line of the check in ddraw.log.
///
/// [pt] A linha final da conferencia no ddraw.log.
pub fn resumo_assinaturas() {
    use std::sync::atomic::Ordering::Relaxed;
    linha(&format!("assinaturas: {} conferidas, {} ficam com o exe ({} us)",
        CHECKED.load(Relaxed), LEFT_TO_EXE.load(Relaxed), MICROS.load(Relaxed)));
}

/// Core of the detours: checks the routine and the bytes, builds the trampoline and jumps to `target(trampoline)`.
///
/// [pt] Miolo dos desvios: confere a rotina e os bytes, monta o trampolim e salta para `destino(trampolim)`.
unsafe fn hook_with(name: &str, p: &Prologo, target: impl FnOnce(usize) -> usize) -> Option<usize> {
    if !signature_matches(p.endereco) {
        linha(&format!("rotina {:#x} fica com o exe (alterada)", p.endereco));
        return None;
    }
    let current = core::slice::from_raw_parts(p.endereco as *const u8, p.bytes.len());
    if current != p.bytes {
        linha(&format!("desvio {} recusado: bytes em {:#x} sao {:02x?}, esperava {:02x?}", name, p.endereco, current, p.bytes));
        return None;
    }
    let n = p.bytes.len();
    let tramp = executable(32) as *mut u8;
    if tramp.is_null() {
        linha(&format!("desvio {} recusado: sem memoria executavel", name));
        return None;
    }
    let m = build_trampoline(p, tramp);
    write_jmp(tramp.add(m), p.endereco + n);
    let target = target(tramp as usize);

    let mut old_protect = 0u32;
    win::VirtualProtect(p.endereco as *mut c_void, n, win::PAGE_EXECUTE_READWRITE, &mut old_protect);
    write_jmp(p.endereco as *mut u8, target);
    for i in 5..n {
        *(p.endereco as *mut u8).add(i) = 0xcc; // never executed (the jmp leaves before)
    }
    win::VirtualProtect(p.endereco as *mut c_void, n, old_protect, &mut old_protect);
    win::FlushInstructionCache(win::GetCurrentProcess(), p.endereco as *const c_void, n);
    linha(&format!("desvio {}: {:#x} -> {:#x} (original em {:#x})", name, p.endereco, target, tramp as usize));
    record_installed(p);
    Some(tramp as usize)
}

/// Prologues that end in a relative instruction (`call rel32` or `jcc rel8`), with its position
/// in the bytes: the trampoline redoes the instruction with the absolute target. `engenharia-reversa/prologos.py`
/// reads this table and accepts the relative instruction only at that position.
///
/// [pt] Prologos que terminam numa instrucao relativa (`call rel32` ou `jcc rel8`), com a posicao dela
/// nos bytes: o trampolim refaz a instrucao com o destino absoluto. `engenharia-reversa/prologos.py`
/// le esta tabela e aceita a instrucao relativa so nessa posicao.
pub const RELATIVAS: &[(usize, usize)] = &[(0x714d00, 4), (0x6f55c0, 4)];

/// Copies the prologue to the trampoline, redoing the relative instruction from RELATIVAS. Returns the size
/// written (the `jmp` back comes after).
///
/// [pt] Copia o prologo para o trampolim, refazendo a instrucao relativa de RELATIVAS. Devolve o tamanho
/// escrito (o `jmp` de volta vem depois).
unsafe fn build_trampoline(p: &Prologo, tramp: *mut u8) -> usize {
    let n = p.bytes.len();
    let Some(&(_, k)) = RELATIVAS.iter().find(|&&(a, _)| a == p.endereco) else {
        core::ptr::copy_nonoverlapping(p.bytes.as_ptr(), tramp, n);
        return n;
    };
    core::ptr::copy_nonoverlapping(p.bytes.as_ptr(), tramp, k);
    let op = p.bytes[k];
    if op == 0xe8 {
        let rel = i32::from_le_bytes([p.bytes[k + 1], p.bytes[k + 2], p.bytes[k + 3], p.bytes[k + 4]]);
        let target = (p.endereco as isize + k as isize + 5 + rel as isize) as usize;
        *tramp.add(k) = 0xe8;
        core::ptr::write_unaligned(tramp.add(k + 1) as *mut i32, (target as isize - (tramp as isize + k as isize + 5)) as i32);
        core::ptr::copy_nonoverlapping(p.bytes.as_ptr().add(k + 5), tramp.add(k + 5), n - k - 5);
        n
    } else {
        // jcc rel8 becomes jcc rel32 (0f 8x): 4 more bytes
        assert!((0x70..0x80).contains(&op));
        let target = (p.endereco as isize + k as isize + 2 + p.bytes[k + 1] as i8 as isize) as usize;
        *tramp.add(k) = 0x0f;
        *tramp.add(k + 1) = 0x80 + (op - 0x70);
        core::ptr::write_unaligned(tramp.add(k + 2) as *mut i32, (target as isize - (tramp as isize + k as isize + 6)) as i32);
        core::ptr::copy_nonoverlapping(p.bytes.as_ptr().add(k + 2), tramp.add(k + 6), n - k - 2);
        n + 4
    }
}

unsafe fn write_jmp(at: *mut u8, to: usize) {
    *at = 0xe9;
    let rel = (to as isize - (at as isize + 5)) as i32;
    core::ptr::write_unaligned(at.add(1) as *mut i32, rel);
}

// ---------- resolution in memory (experiment, CM_RESOLUCAO=1080) ----------

/// With `CM_RESOLUCAO=1080`, applies in memory the swaps of `resolucao.rs` (generated by `resolucao.py --rust`:
/// patch_portatil.py + resolucao.py 1920x1080), and the original cm0102.exe becomes equal to cm0102-1080.exe.
/// Runs in DllMain before everything (before WinMain and the exe's CRT, so the swaps in .data hold from the
/// start, and before the detours, which then see the same as in cm0102-1080.exe). A swap whose bytes are not the
/// "before" ones (nor already the "after" ones) is left out and goes to the log (exe with a third-party patch).
///
/// [pt] Com `CM_RESOLUCAO=1080`, aplica em memoria as trocas de `resolucao.rs` (geradas por `resolucao.py --rust`:
/// patch_portatil.py + resolucao.py 1920x1080), e o cm0102.exe original fica igual ao cm0102-1080.exe.
/// Roda no DllMain antes de tudo (antes do WinMain e da CRT do exe, entao as trocas em .data valem desde o
/// inicio, e antes dos desvios, que entao veem o mesmo que no cm0102-1080.exe). Troca cujos bytes nao sao os
/// "antes" (nem ja os "depois") fica de fora e vai para o log (exe com patch de terceiros).
pub unsafe fn aplicar_resolucao() {
    if win::texto_ambiente("CM_RESOLUCAO") != "1080" {
        return;
    }
    // remove the variable from the environment before the exe's CRT copies it to the heap (the CRT runs after DllMain):
    // one extra variable shifts the heap and changes the career (see simular.py, rodar)
    let name: Vec<u16> = "CM_RESOLUCAO ".encode_utf16().collect();
    win::SetEnvironmentVariableW(name.as_ptr(), core::ptr::null());
    let (mut applied, mut already, mut left_out) = (0, 0, 0);
    for &(va, before, after) in crate::resolution::TROCAS.iter() {
        let current = core::slice::from_raw_parts(va as *const u8, before.len());
        if current == after {
            already += 1;
            continue;
        }
        if current != before {
            linha(&format!("resolucao: troca em {:#x} fica de fora: bytes {:02x?}, esperava {:02x?}", va, current, before));
            left_out += 1;
            continue;
        }
        let mut old_protect = 0u32;
        win::VirtualProtect(va as *mut c_void, before.len(), win::PAGE_EXECUTE_READWRITE, &mut old_protect);
        core::ptr::copy_nonoverlapping(after.as_ptr(), va as *mut u8, after.len());
        win::VirtualProtect(va as *mut c_void, before.len(), old_protect, &mut old_protect);
        win::FlushInstructionCache(win::GetCurrentProcess(), va as *const c_void, after.len());
        applied += 1;
    }
    linha(&format!("resolucao 1080: {} trocas aplicadas, {} ja estavam, {} de fora (de {})", applied, already, left_out, crate::resolution::TROCAS.len()));
    if win::ambiente("CM_RESOLUCAO_CONFERIR") {
        check_resolution();
    }
}

/// Diagnosis (`CM_RESOLUCAO_CONFERIR=1`): compares each section of the in-memory image with the cm0102-1080.exe next
/// to the exe (except the IAT, which the loader fills). Uses the heap: only in the proofs, never in the career.
///
/// [pt] Diagnostico (`CM_RESOLUCAO_CONFERIR=1`): compara cada secao da imagem em memoria com o cm0102-1080.exe ao
/// lado do exe (fora a IAT, que o carregador preenche). Usa a heap: so nas provas, nunca na carreira.
unsafe fn check_resolution() {
    let Ok(disk) = std::env::current_exe().and_then(|p| std::fs::read(p.with_file_name("cm0102-1080.exe"))) else {
        linha("resolucao: conferencia sem o cm0102-1080.exe");
        return;
    };
    let base = 0x400000usize;
    let rd = |o: usize| core::ptr::read_unaligned((base + o) as *const u32) as usize;
    let pe = rd(0x3c);
    let nsec = core::ptr::read_unaligned((base + pe + 6) as *const u16) as usize;
    let opt = pe + 24;
    let (iat, iat_n) = (rd(opt + 96 + 12 * 8), rd(opt + 96 + 12 * 8 + 4));
    let table = opt + core::ptr::read_unaligned((base + pe + 20) as *const u16) as usize;
    let mut total = 0;
    for k in 0..nsec {
        let s = table + 40 * k;
        let name = String::from_utf8_lossy(core::slice::from_raw_parts((base + s) as *const u8, 8)).trim_end_matches('\0').to_string();
        let (vsize, rva, rsize, raw) = (rd(s + 8), rd(s + 12), rd(s + 16), rd(s + 20));
        let n = vsize.min(rsize).min(disk.len().saturating_sub(raw));
        let mem = core::slice::from_raw_parts((base + rva) as *const u8, n);
        let diffs: Vec<usize> = (0..n).filter(|&i| mem[i] != disk[raw + i] && !(iat..iat + iat_n).contains(&(rva + i))).collect();
        let examples: Vec<String> = diffs.iter().take(8).map(|&i| format!("{:#x}", base + rva + i)).collect();
        linha(&format!("resolucao: conferencia {} ({} bytes): {} diferem {:?}", name, n, diffs.len(), examples));
        total += diffs.len();
    }
    linha(&format!("resolucao: conferencia com o cm0102-1080.exe: {} bytes diferem (IAT de fora)", total));
}

// ---------- exe with a third-party patch: deferred installation ----------
//
// A patcher may change routines in memory after our DllMain (in CLZ, NickDLL.dll, loaded by a detour at the
// start of the CRT, and the no-CD, loaded at the exe's entry, change 15 of our routines). Generic criterion, with no DLL
// or section name: are the whole .text and the entry point those of an exe of ours? Install as always (reference
// career untouched). Otherwise, the signature snapshot and the engine detours wait for the first DirectDrawCreate
// (exported by this DLL: the game only calls it in WinMain, after loading the game's DLLs and the CRT).

/// FNV-1a 64 of 0x401000..0x401000+TEXT_LEN in cm0102.exe, cm0102-portatil.exe and cm0102-1080.exe (the exes never
/// change; CM_RESOLUCAO=1080 takes the first to the third before this check).
///
/// [pt] FNV-1a 64 de 0x401000..0x401000+TEXTO no cm0102.exe, no cm0102-portatil.exe e no cm0102-1080.exe (os exes nunca
/// mudam; o CM_RESOLUCAO=1080 leva o primeiro ao terceiro antes desta conferencia).
const TEXT_LEN: usize = 0x5650e5;
const OUR_TEXTS: [u64; 3] = [0x7af14da1f595ae30, 0x62afeb6b1b407585, 0xfd25058c59d683d5];
const OUR_ENTRY: u32 = 0x547a13; // AddressOfEntryPoint (RVA; VA 0x947a13)

static DEFERRED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// Only the deferred box (CM_SEM_DESVIOS in a patched exe): the engine detours stay off.
///
/// [pt] Só a caixa adiada (CM_SEM_DESVIOS num exe com patch): os desvios do motor ficam desligados.
static BOX_DEFERRED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// Permanent (DEFERRED is only "installation pending" and goes back to false on installation; checking it after that
/// switched off the preserved ECX and the system video in the patched exe, 25/10/2026).
///
/// [pt] Permanente (o ADIADO é só "instalação pendente" e volta a falso na instalação; consultá-lo depois dela
/// desligava o ECX preservado e o vídeo do sistema no exe com patch, 25/10/2026).
static PATCHED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// The exe has a third-party patch (the .text or the entry are not ours): deferred installation.
///
/// [pt] O exe tem patch de terceiros (o .text ou a entrada não são os nossos): instalação adiada.
pub fn exe_com_patch() -> bool {
    PATCHED.load(std::sync::atomic::Ordering::Relaxed)
}

fn fnv(b: &[u8]) -> u64 {
    b.iter().fold(0xcbf29ce484222325u64, |h, &x| (h ^ x as u64).wrapping_mul(0x100000001b3))
}

unsafe fn our_exe() -> bool {
    let pe = core::ptr::read_unaligned(0x40003c as *const u32) as usize;
    let entry = core::ptr::read_unaligned((0x400000 + pe + 0x28) as *const u32);
    let h = fnv(core::slice::from_raw_parts(0x401000 as *const u8, TEXT_LEN));
    let ours = entry == OUR_ENTRY && OUR_TEXTS.contains(&h);
    if !ours {
        linha(&format!("desvios: exe com patch (.text {:016x}, entrada {:#x}): foto das assinaturas e desvios do motor adiados para a 1a DirectDrawCreate", h, 0x400000 + entry));
    }
    ours
}

// ---------- the game's message boxes in ddraw.log (observe only) ----------
//
// gui_show_message_box (0x5e8290, cdecl: title, text, allow exit) is drawn by the game itself, it is not a Windows
// box: with the game hidden (CM_OCULTO), an error ("Database..cpp 7784") or the CD request leaves the game stopped waiting
// for OK and the test only ended by timeout. Every box becomes the line `msgbox AAAA-DDD title | text` before showing, and
// simular.py stops right away (02/11/2026). It is not an engine routine (outside PROLOGOS and ASSINATURAS): it runs on the
// game's stack, only formats when there is a box, and goes in after the engine detours (their trampolines do not change
// address). In all modes, including CM_ORIGINAL and CM_SEM_DESVIOS (observe only). The ported routines call the box by the
// game's address, so they also go through here.

/// The original box (or the jump of another DLL that detoured it before), through the trampoline.
///
/// [pt] A caixa original (ou o salto de outra DLL que a desviou antes), pelo trampolim.
static mut ORIGINAL_BOX: usize = 0;

unsafe extern "C" fn message_box(title: *const u8, text: *const u8, allow_exit: u32) -> u32 {
    // the game's ANSI text, one byte per character; line breaks and tabs become spaces (a single line in the log)
    let read = |p: *const u8| -> String {
        if p.is_null() {
            return String::new();
        }
        let n = (0..1024).position(|k| *p.add(k) == 0).unwrap_or(1024);
        core::slice::from_raw_parts(p, n).iter().map(|&c| if matches!(c, b'\n' | b'\r' | b'\t') { ' ' } else { c as char }).collect()
    };
    linha(&format!("msgbox {}-{:03} {} | {}", *(0xae2c92 as *const u16), *(0xae2c90 as *const u16), read(title), read(text)));
    let original: unsafe extern "C" fn(*const u8, *const u8, u32) -> u32 = core::mem::transmute(ORIGINAL_BOX);
    original(title, text, allow_exit)
}

/// Detours 0x5e8290 to `message_box`. The original entry (`sub esp,0x6c; mov ecx,[0xb67a30]`, 9 bytes with no
/// relative instruction) goes to the trampoline; a `jmp` from another DLL there (CLZ's LaMOHatesCD swaps those 9 bytes) is
/// chained to its target; anything else, leave it alone and say so in the log. Only once (again, it would chain to itself).
///
/// [pt] Desvia a 0x5e8290 para `caixa_de_mensagem`. A entrada original (`sub esp,0x6c; mov ecx,[0xb67a30]`, 9 bytes sem
/// instrucao relativa) vai para o trampolim; um `jmp` de outra DLL ali (a LaMOHatesCD do CLZ troca esses 9 bytes) e
/// encadeado no destino dele; outra coisa, nao mexe e diz no log. Uma vez so (de novo, encadearia em si mesma).
unsafe fn install_box() {
    static DONE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    if DONE.swap(true, std::sync::atomic::Ordering::Relaxed) {
        return;
    }
    const E: usize = 0x5e8290;
    const ENTRY_BYTES: [u8; 9] = [0x83, 0xec, 0x6c, 0x8b, 0x0d, 0x30, 0x7a, 0xb6, 0x00];
    let current = *(E as *const [u8; 9]);
    if current != ENTRY_BYTES && current[0] != 0xe9 {
        linha(&format!("msgbox: caixas do jogo fora do log (entrada de {:#x} mudada: {:02x?})", E, current));
        return;
    }
    let t = executable(16) as *mut u8;
    if t.is_null() {
        linha("msgbox: caixas do jogo fora do log (sem memoria executavel)");
        return;
    }
    let how = if current == ENTRY_BYTES {
        core::ptr::copy_nonoverlapping(ENTRY_BYTES.as_ptr(), t, 9);
        write_jmp(t.add(9), E + 9); // the E9 at t+9 ends at t+14
        "a do exe"
    } else {
        write_jmp(t, (E + 5).wrapping_add(core::ptr::read_unaligned((E + 1) as *const i32) as usize));
        "encadeada no salto de outra DLL"
    };
    ORIGINAL_BOX = t as usize;
    let mut old_protect = 0u32;
    win::VirtualProtect(E as *mut c_void, 5, win::PAGE_EXECUTE_READWRITE, &mut old_protect);
    write_jmp(E as *mut u8, message_box as *const () as usize);
    win::VirtualProtect(E as *mut c_void, 5, old_protect, &mut old_protect);
    win::FlushInstructionCache(win::GetCurrentProcess(), E as *const c_void, 5);
    linha(&format!("msgbox: as caixas do jogo vao para o log ({})", how));
}

/// Installs all the detours. Called when the DLL loads, before the game's WinMain.
///
/// [pt] Instala todos os desvios. Chamado no carregamento da DLL, antes do WinMain do jogo.
pub unsafe fn instalar() {
    if win::ambiente("CM_SEM_DESVIOS") {
        linha("desvios: desligados (CM_SEM_DESVIOS)");
        // observe only; in a patched exe its DLL (CLZ's LaMOHatesCD) swaps the entry after DllMain: the box
        // waits for the 1st DirectDrawCreate to chain to its jump (finding from the CLZ session, 02/11/2026)
        if our_exe() {
            install_box();
        } else {
            BOX_DEFERRED.store(true, std::sync::atomic::Ordering::Relaxed);
        }
        return;
    }
    if !our_exe() {
        // the simulation (allocator, clock, CRT draw) must be in place before the CRT; none of it falls in the ranges
        crate::simulation::instalar();
        DEFERRED.store(true, std::sync::atomic::Ordering::Relaxed);
        PATCHED.store(true, std::sync::atomic::Ordering::Relaxed);
        return;
    }
    conferir_assinaturas();
    // Batch simulation (CM_PARAR_EM): fixed clock, so the simulation is reproducible.
    crate::simulation::instalar();
    install_engine();
    install_box();
}

/// First DirectDrawCreate: the deferred installation of the patched exe (nothing in our exes).
///
/// [pt] Primeira DirectDrawCreate: a instalacao adiada do exe com patch (nada nos exes nossos).
pub unsafe fn instalar_adiado() {
    if BOX_DEFERRED.swap(false, std::sync::atomic::Ordering::Relaxed) {
        install_box();
    }
    if DEFERRED.swap(false, std::sync::atomic::Ordering::Relaxed) {
        linha(&format!("desvios: instalacao adiada (data do jogo {:?})", *(0xae2c90 as *const [u16; 2])));
        conferir_assinaturas();
        install_engine();
        install_box();
    }
}

unsafe fn install_engine() {
    // CM_ORIGINAL=1: no routine replaced; the draw is only observed (count and stop of the
    // simulation), to compare a simulation with only the originals against another with the Rust.
    if win::ambiente("CM_ORIGINAL") {
        rng::instalar(rng::Modo::Observar);
        if ISOLAR.load(std::sync::atomic::Ordering::Relaxed) {
            date::instalar_passagem();
            crate::motor::instalar_passagem();
        }
        crate::stack_probe::instalar_lista();
        snapshot_bodies();
        resumo_assinaturas();
        return;
    }
    // CM_AUTOTESTE=1 switches on comparison mode; the self-tests themselves run later (see `autotestes`).
    // A crash in the self-test is logged in ddraw.log (address and registers).
    if win::ambiente("CM_AUTOTESTE") {
        win::AddVectoredExceptionHandler(1, crate::simulation::registrar_excecao);
    }
    let compare = win::ambiente("CM_COMPARAR") || win::ambiente("CM_AUTOTESTE");
    rng::instalar(if compare { rng::Modo::Comparar } else { rng::Modo::Rust });
    date::instalar(compare);
    crate::motor::instalar(compare);
    snapshot_bodies();
    resumo_assinaturas();
}

// ---------- daily recheck of the bodies ----------
//
// Patcher that changes code after installation: once per game day (through the draw, which runs in all modes),
// the body of each detoured routine (the ASSINATURAS range without the prologue, where our jmp is) is compared with the
// snapshot from the end of installation. Changed body: the original prologue comes back (detour undone) and the log
// records it. ~670 KB of hashing per day (~2 ms). simplification: only the ASSINATURAS routines (date, draw and
// simulation have no range).

/// (start, original prologue, end of range, body hash, our jmp); hash 0 = no snapshot or already undone.
///
/// [pt] (inicio, prologo original, fim da faixa, hash do corpo, o nosso jmp); hash 0 = sem foto ou ja desfeito.
static mut INSTALLED: Vec<(usize, &'static [u8], usize, u64, [u8; 5])> = Vec::new();
static mut CHECKED_DAY: [u16; 2] = [0; 2];

unsafe fn record_installed(p: &Prologo) {
    use crate::assinaturas::ASSINATURAS;
    if let Ok(k) = ASSINATURAS.binary_search_by_key(&p.endereco, |a| a.0) {
        (*core::ptr::addr_of_mut!(INSTALLED)).push((p.endereco, p.bytes, p.endereco + ASSINATURAS[k].1, 0, *(p.endereco as *const [u8; 5])));
    }
}

unsafe fn body_hash(e: &(usize, &'static [u8], usize, u64, [u8; 5])) -> u64 {
    let start = e.0 + e.1.len();
    if e.2 <= start { 1 } else { fnv(core::slice::from_raw_parts(start as *const u8, e.2 - start)) }
}

unsafe fn snapshot_bodies() {
    for e in (*core::ptr::addr_of_mut!(INSTALLED)).iter_mut() {
        e.3 = body_hash(e);
    }
    CHECKED_DAY = *(0xae2c90 as *const [u16; 2]);
}

/// Called on every draw: only works when the game day changes. Inline and without a stack frame on the common path (the
/// game reads stack leftovers; one more frame on every draw would change what it finds outside the private stack).
///
/// [pt] Chamado a cada sorteio: so trabalha quando o dia do jogo muda. Inline e sem quadro de pilha no caminho comum (o
/// jogo le sobras de pilha; um quadro a mais em cada sorteio mudaria o que ele encontra fora da pilha particular).
#[inline(always)]
pub unsafe fn reconferir_no_dia() {
    let today = *(0xae2c90 as *const [u16; 2]);
    if today != CHECKED_DAY {
        recheck_bodies(today);
    }
}

#[inline(never)]
#[cold]
unsafe fn recheck_bodies(today: [u16; 2]) {
    CHECKED_DAY = today;
    for e in (*core::ptr::addr_of_mut!(INSTALLED)).iter_mut() {
        if e.3 == 0 || body_hash(e) == e.3 {
            continue;
        }
        e.3 = 0;
        let p = e.0 as *mut u8;
        if *(p as *const [u8; 5]) != e.4 {
            linha(&format!("rotina {:#x}: corpo e entrada mudaram depois do desvio (dia {:?}); fica com o exe", e.0, today));
            continue;
        }
        let mut old_protect = 0u32;
        win::VirtualProtect(p as *mut c_void, e.1.len(), win::PAGE_EXECUTE_READWRITE, &mut old_protect);
        core::ptr::copy_nonoverlapping(e.1.as_ptr(), p, e.1.len());
        win::VirtualProtect(p as *mut c_void, e.1.len(), old_protect, &mut old_protect);
        win::FlushInstructionCache(win::GetCurrentProcess(), p as *const c_void, e.1.len());
        linha(&format!("rotina {:#x}: corpo mudou depois do desvio (dia {:?}); desvio desfeito, fica com o exe", e.0, today));
    }
}

/// CM_AUTOTESTE=1: each rewritten routine compares itself with the original on synthetic inputs
/// (result in ddraw.log). `CM_AUTOTESTE=motor:N` runs only the engine, with N cases (the mutations of
/// an engine batch do not touch the rng nor the date). Called on the first DirectDrawCreate: by then WinMain is already
/// running and the game's CRT is initialized (some routines call atexit, malloc...).
///
/// [pt] CM_AUTOTESTE=1: cada rotina reescrita se compara com a original em entradas sinteticas
/// (resultado em ddraw.log). `CM_AUTOTESTE=motor:N` roda so o motor, com N casos (as mutacoes de
/// uma leva do motor nao mexem no aleatorio nem na data). Chamado na primeira DirectDrawCreate: a essa altura o WinMain ja
/// esta rodando e a CRT do jogo esta inicializada (algumas rotinas chamam atexit, malloc...).
pub unsafe fn autotestes() {
    static DONE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    if !win::ambiente("CM_AUTOTESTE") || DONE.swap(true, std::sync::atomic::Ordering::Relaxed) {
        return;
    }
    let value = win::texto_ambiente("CM_AUTOTESTE");
    if let Some(n) = value.strip_prefix("motor:") {
        crate::motor::autoteste(n.parse().unwrap_or(2_000_000));
        return;
    }
    // CM_AUTOTESTE=banco: only the database self-tests (mutacoes.py --autoteste-banco); the engine with 1 case closes
    // with the "motor: AUTOTESTE" line that mutacoes.py expects
    if value == "banco" {
        crate::motor::autoteste_dbj_1();
        crate::motor::autoteste_dbj_2();
        crate::motor::autoteste_dbm_1();
        crate::motor::autoteste_dbm_2();
        crate::motor::autoteste(1);
        return;
    }
    // the version 1 converters and the English names (232nd): neither the career nor the save proof go through them
    crate::motor::autoteste_dbj_1();
    crate::motor::autoteste_dbj_2();
    crate::motor::autoteste_dbm_1();
    crate::motor::autoteste_dbm_2();
    rng::autoteste(5_000_000);
    date::autoteste();
    crate::motor::autoteste(2_000_000);
}
