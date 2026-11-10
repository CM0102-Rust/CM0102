//! Stack probe (diagnosis): proves whether the stack residue of a range is read and changes the career.
//!
//! `CM_SONDA_PILHA=depois:ADDRESS` (or `dentro:ADDRESS`): the routine runs on the game's stack, as without a detour, between
//! two stubs that do not use the stack (the game's return address goes back to its dword, so without poison the residue is
//! the one without a detour and the career does not change). With `CM_SONDA_FAIXA=ini-fim` (bytes below the entry, in hex), the range
//! [entry - fim, entry - ini) gets `CM_SONDA_VENENO` (hex) on calls a..b (decimal,
//! counted from 1; default all; they go as CM_SONDA_FAIXA=ini-fim/a-b): `depois` on the way back (what it left for whoever comes after), `dentro` on entry (what
//! it reads itself without having written). A career different from the one without poison = someone reads residue of the range on those calls;
//! `sonda_pilha.py` bisects the range and then the calls. On the poisoned calls, `CM_SONDA_MOSTRAR=1` writes to
//! ddraw.log the chain of return addresses above the entry (the live stack: who called). The debug registers did not
//! work (the exception dispatcher runs on the same stack, and the triggers did not tell reads from writes).
//!
//! [pt] Sonda de pilha (diagnóstico): prova se o resíduo de pilha de uma faixa é lido e muda a carreira.
//!
//! `CM_SONDA_PILHA=depois:ENDERECO` (ou `dentro:ENDERECO`): a rotina roda na pilha do jogo, como sem desvio, entre
//! dois trechos que não usam a pilha (o endereço de retorno do jogo volta ao seu dword, então sem veneno o resíduo é
//! o de sem desvio e a carreira não muda). Com `CM_SONDA_FAIXA=ini-fim` (bytes abaixo da entrada, em hex), a faixa
//! [entrada − fim, entrada − ini) recebe `CM_SONDA_VENENO` (hex) nas chamadas a..b (decimal,
//! contadas desde 1; padrão todas; vão como CM_SONDA_FAIXA=ini-fim/a-b): `depois` na volta (o que ela deixou para quem vem depois), `dentro` na entrada (o
//! que ela mesma lê sem ter gravado). Carreira diferente da sem veneno = alguém lê resíduo da faixa naquelas chamadas;
//! `sonda_pilha.py` bissecta a faixa e depois as chamadas. Nas chamadas envenenadas, `CM_SONDA_MOSTRAR=1` grava no
//! ddraw.log a cadeia de retornos acima da entrada (a pilha viva: quem chamou). Os registradores de depuração não
//! serviram (o despachante de exceções roda na mesma pilha, e os disparos não distinguiam leitura de gravação).

use crate::registro::linha;
use crate::win;

static mut TMP_EAX: u32 = 0;
static mut TMP_ECX: u32 = 0;
static mut TMP_EDX: u32 = 0;
static mut TMP_EDI: u32 = 0;
static mut TMP_ENTRY: u32 = 0;
static mut SAVED_ESP: u32 = 0;
static mut TARGET: u32 = 0;
static mut CALLS: u32 = 0;
static mut RETURNS: [u32; 1024] = [0; 1024]; // (game's return address, entry) per level
static mut TOP: u32 = 0;

/// Probe target (0 = off), whether it poisons on entry (`dentro`) and, for a routine outside the prologue table, the
/// size of the prologue to copy (`dentro:5ae380:7`: whole instructions, no relative jump; check with desmonta.py).
///
/// [pt] Alvo da sonda (0 = desligada), se envenena na entrada (`dentro`) e, para rotina fora da tabela de prólogos, o
/// tamanho do prólogo a copiar (`dentro:5ae380:7`: instruções inteiras, sem salto relativo; ver com o desmonta.py).
pub fn target() -> (usize, bool, usize) {
    let t = win::texto_ambiente("CM_SONDA_PILHA");
    let (inside, e) = match t.split_once(':') {
        Some(("dentro", e)) => (true, e),
        Some(("depois", e)) => (false, e),
        _ => return (0, false, 0),
    };
    let (e, n) = e.split_once(':').unwrap_or((e, "0"));
    (usize::from_str_radix(e.trim_start_matches("0x"), 16).unwrap_or(0), inside, n.parse().unwrap_or(0))
}

fn hex(t: &str) -> u32 {
    u32::from_str_radix(t.trim().trim_start_matches("0x"), 16).unwrap_or(0)
}

/// The live chain above the entry: dwords that point just after a call in the exe.
///
/// [pt] A cadeia viva acima da entrada: dwords que apontam logo depois de um call do exe.
unsafe extern "C" fn show_chain(entry: u32) {
    let mut ret = Vec::new();
    for d in 0..0x1000u32 {
        let r = *((entry + 4 * d) as *const u32);
        if (0x401006..0x970000).contains(&r) {
            let b = |k: u32| *((r - k) as *const u8);
            if b(5) == 0xe8 || b(6) == 0xff || b(3) == 0xff || b(2) == 0xff {
                ret.push(format!("+{:#x}:{:x}", 4 * d, r));
                if ret.len() == 80 {
                    break;
                }
            }
        }
    }
    linha(&format!("stack_probe: call {} date {:?} entry {:#x} returns {}", CALLS, *(0xae2c90 as *const [u16; 2]), entry, ret.join(" ")));
}

/// Builds the stubs around the routine's trampoline `tramp`; returns the detour target.
///
/// [pt] Monta os trechos em volta do trampolim `tramp` da rotina; devolve o destino do desvio.
pub unsafe fn build_stub(tramp: usize, inside: bool) -> usize {
    // CM_SONDA_FAIXA=ini-fim[/from-to]: the calls go in the same variable (one extra variable shifts the heap and the career)
    let text = win::texto_ambiente("CM_SONDA_FAIXA");
    let (band, calls) = text.split_once('/').unwrap_or((&text, ""));
    let (start, end) = band.split_once('-').map(|(a, b)| (hex(a) & !3, hex(b) & !3)).unwrap_or((0, 0));
    let poison = hex(&win::texto_ambiente("CM_SONDA_VENENO"));
    let (first, last) = calls.split_once('-').map(|(a, b)| (a.parse().unwrap_or(1u32), b.parse().unwrap_or(u32::MAX))).unwrap_or((1, u32::MAX));
    let show = win::ambiente("CM_SONDA_MOSTRAR");
    let stack = win::VirtualAlloc(core::ptr::null_mut(), 1 << 16, win::MEM_COMMIT | win::MEM_RESERVE, win::PAGE_READWRITE) as u32 + (1 << 16) - 16;
    let code = win::VirtualAlloc(core::ptr::null_mut(), 512, win::MEM_COMMIT | win::MEM_RESERVE, win::PAGE_EXECUTE_READWRITE) as usize;
    TOP = &raw mut RETURNS as u32;
    let a = |v: *mut u32| (v as u32).to_le_bytes();
    let (eax, ecx, edi, top) = (&raw mut TMP_EAX, &raw mut TMP_ECX, &raw mut TMP_EDI, &raw mut TOP);
    let mut c: Vec<u8> = Vec::new();
    // on calls first..=last: poison [base - end, base - start) (base = esp on entry, ecx on the way back) and show the chain;
    // eax and ecx already saved
    let poison_fill = |c: &mut Vec<u8>, via_stack: bool| {
        if end <= start && !show {
            return;
        }
        let mut v: Vec<u8> = Vec::new();
        if show {
            // mov [TMP_EDX], edx; mov [SAVED_ESP], esp; mov esp, stack; push ecx (or the entry); call show_chain; mov esp, [SAVED_ESP]; mov edx, [TMP_EDX]
            v.extend([0x89, 0x15]); v.extend(a(&raw mut TMP_EDX));
            v.extend([0x89, 0x25]); v.extend(a(&raw mut SAVED_ESP));
            if via_stack {
                v.extend([0x89, 0xe1]); // mov ecx, esp (the entry)
            }
            v.extend([0x89, 0x0d]); v.extend(a(&raw mut TMP_ENTRY)); // mov [TMP_ENTRY], ecx
            v.push(0xbc); v.extend(stack.to_le_bytes());
            v.push(0x51);
            v.push(0xb8); v.extend((show_chain as usize as u32).to_le_bytes());
            v.extend([0xff, 0xd0]); // call eax
            v.extend([0x8b, 0x25]); v.extend(a(&raw mut SAVED_ESP));
            v.extend([0x8b, 0x15]); v.extend(a(&raw mut TMP_EDX));
            v.extend([0x8b, 0x0d]); v.extend(a(&raw mut TMP_ENTRY)); // mov ecx, [TMP_ENTRY]
        }
        if end > start {
            // mov [TMP_EDI], edi; lea edi, [base - end]; mov ecx, n; mov eax, poison; cld; rep stosd; mov edi, [TMP_EDI]
            v.extend([0x89, 0x3d]); v.extend(a(edi));
            v.extend(if via_stack { &[0x8d, 0xbc, 0x24][..] } else { &[0x8d, 0xb9][..] });
            v.extend((end as i32).wrapping_neg().to_le_bytes());
            v.push(0xb9); v.extend(((end - start) / 4).to_le_bytes());
            v.push(0xb8); v.extend(poison.to_le_bytes());
            v.extend([0xfc, 0xf3, 0xab]);
            v.extend([0x8b, 0x3d]); v.extend(a(edi));
        }
        // inc [CALLS]; cmp [CALLS], first; jb end; cmp [CALLS], last; ja end; <v>
        c.extend([0xff, 0x05]); c.extend(a(&raw mut CALLS));
        c.extend([0x81, 0x3d]); c.extend(a(&raw mut CALLS)); c.extend(first.to_le_bytes());
        c.extend([0x0f, 0x82]); c.extend((v.len() as u32 + 16).to_le_bytes());
        c.extend([0x81, 0x3d]); c.extend(a(&raw mut CALLS)); c.extend(last.to_le_bytes());
        c.extend([0x0f, 0x87]); c.extend((v.len() as u32).to_le_bytes());
        c.extend(v);
    };
    // entry: save eax/ecx; push (game's return address, esp) onto RETURNS
    c.push(0xa3); c.extend(a(eax));
    c.extend([0x89, 0x0d]); c.extend(a(ecx));
    c.push(0xa1); c.extend(a(top));
    c.extend([0x8b, 0x0c, 0x24, 0x89, 0x08, 0x89, 0x60, 0x04]); // mov ecx, [esp]; mov [eax], ecx; mov [eax+4], esp
    c.extend([0x83, 0xc0, 0x08, 0xa3]); c.extend(a(top));
    if inside {
        poison_fill(&mut c, true);
    }
    c.extend([0x8b, 0x0d]); c.extend(a(ecx));
    c.push(0xa1); c.extend(a(eax));
    c.extend([0x83, 0xc4, 0x04, 0xe8]); // add esp, 4 (drops the game's return address); call trampoline (puts ours in the same place)
    let rel = (tramp as isize - (code + c.len() + 4) as isize) as i32;
    c.extend(rel.to_le_bytes());
    // way back: pop; the game's return address goes back to its dword (the residue stays the one without a detour)
    c.push(0xa3); c.extend(a(eax));
    c.extend([0x89, 0x0d]); c.extend(a(ecx));
    c.push(0xa1); c.extend(a(top));
    c.extend([0x83, 0xe8, 0x08, 0xa3]); c.extend(a(top));
    c.extend([0x8b, 0x48, 0x04]); // mov ecx, [eax+4] (the entry)
    c.extend([0x8b, 0x00, 0x89, 0x01, 0xa3]); c.extend(a(&raw mut TARGET)); // mov eax, [eax]; mov [ecx], eax; mov [TARGET], eax
    if !inside {
        poison_fill(&mut c, false);
    }
    c.extend([0x8b, 0x0d]); c.extend(a(ecx));
    c.push(0xa1); c.extend(a(eax));
    c.extend([0xff, 0x25]); c.extend(a(&raw mut TARGET));
    assert!(c.len() <= 512);
    core::ptr::copy_nonoverlapping(c.as_ptr(), code as *mut u8, c.len());
    linha(&format!("stack_probe: {}, band {:#x}..{:#x} below the entry with {:#x}, calls {}..{}",
        if inside { "dentro" } else { "depois" }, start, end, poison, first, last));
    code
}

// ---------- scan of the readers (CM_SONDA_LISTA=1, engenharia-reversa/leitoras.py) ----------

/// Reads `sonda_lista.txt` (address, prologue bytes, frame, marked; hex/dec/hex/0-1) and detours each marked routine to
/// a stub that poisons its own frame on entry ([esp - frame, esp), with 0x5a5a5a5a) and goes on to the original.
/// Without a mark there is no detour (the stub is transparent, proved with the probe of one routine). All in a single block (thousands of
/// routines: one VirtualAlloc per trampoline would spend 64 KB of addresses each).
///
/// [pt] Lê `sonda_lista.txt` (endereço, bytes do prólogo, quadro, marcada; hex/dec/hex/0-1) e desvia cada rotina marcada para
/// um trecho que envenena o próprio quadro na entrada ([esp − quadro, esp), com 0x5a5a5a5a) e segue para a original.
/// Sem marca não há desvio (o trecho é transparente, provado com a sonda de uma rotina). Tudo num bloco só (milhares de
/// rotinas: um VirtualAlloc por trampolim gastaria 64 KB de endereços cada).
pub unsafe fn install_list() {
    if !win::ambiente("CM_SONDA_LISTA") {
        return;
    }
    let text = std::fs::read_to_string("sonda_lista.txt").unwrap_or_default();
    let marked: Vec<(u32, usize, u32)> = text
        .lines()
        .filter_map(|l| {
            let c: Vec<&str> = l.split_whitespace().collect();
            (c.len() == 4 && c[3] == "1").then(|| (hex(c[0]), c[1].parse().unwrap_or(0), hex(c[2]) & !3))
        })
        .filter(|&(a, n, q)| a != 0 && n >= 5 && q != 0)
        .collect();
    let block = win::VirtualAlloc(core::ptr::null_mut(), 96 * marked.len() + 64, win::MEM_COMMIT | win::MEM_RESERVE, win::PAGE_EXECUTE_READWRITE) as usize;
    if block == 0 {
        linha("stack_probe: list without memory");
        return;
    }
    let mut pos = block;
    for &(end, n, frame) in &marked {
        let original = core::slice::from_raw_parts(end as *const u8, n).to_vec();
        // trampoline: the prologue and the jmp back
        let tramp = pos;
        let mut c: Vec<u8> = original.clone();
        c.push(0xe9);
        c.extend(((end as usize + n) as i32).wrapping_sub((tramp + c.len() + 4) as i32).to_le_bytes());
        // stub: push eax/ecx/edi (on its own stack: another thread may go through the same routine); lea edi, [esp - 4];
        // mov ecx, frame/4; mov eax, poison; std; rep stosd; cld (top to bottom: goes through the guard page in
        // order, like __chkstk); pop; jmp trampoline. The poisoned frame sits 12 bytes below the entry.
        let stub = tramp + c.len();
        c.extend([0x50, 0x51, 0x57, 0x8d, 0x7c, 0x24, 0xfc]);
        c.push(0xb9); c.extend((frame / 4).to_le_bytes());
        c.push(0xb8); c.extend(0x5a5a_5a5au32.to_le_bytes());
        c.extend([0xfd, 0xf3, 0xab, 0xfc, 0x5f, 0x59, 0x58]);
        c.push(0xe9);
        c.extend((tramp as i32).wrapping_sub((block + (c.len() + (tramp - block)) + 4) as i32).to_le_bytes());
        assert!(c.len() <= 96);
        core::ptr::copy_nonoverlapping(c.as_ptr(), tramp as *mut u8, c.len());
        // the entry becomes jmp stub
        let mut old_protect = 0u32;
        win::VirtualProtect(end as *mut core::ffi::c_void, 5, win::PAGE_EXECUTE_READWRITE, &mut old_protect);
        *(end as *mut u8) = 0xe9;
        core::ptr::write_unaligned((end + 1) as *mut i32, (stub as i32).wrapping_sub(end as i32 + 5));
        win::VirtualProtect(end as *mut core::ffi::c_void, 5, old_protect, &mut old_protect);
        pos += 96;
    }
    win::FlushInstructionCache(win::GetCurrentProcess(), core::ptr::null(), 0);
    linha(&format!("stack_probe: list with {} marked routines", marked.len()));
}
