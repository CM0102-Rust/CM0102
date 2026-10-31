//! Sonda de pilha (diagnóstico): prova se o resíduo de pilha de uma faixa é lido e muda a carreira.
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
static mut TMP_ENT: u32 = 0;
static mut ESP_SALVO: u32 = 0;
static mut ALVO: u32 = 0;
static mut CHAMADAS: u32 = 0;
static mut RETORNOS: [u32; 1024] = [0; 1024]; // (retorno do jogo, entrada) por nível
static mut TOPO: u32 = 0;

/// Alvo da sonda (0 = desligada), se envenena na entrada (`dentro`) e, para rotina fora da tabela de prólogos, o
/// tamanho do prólogo a copiar (`dentro:5ae380:7`: instruções inteiras, sem salto relativo; ver com o desmonta.py).
pub fn alvo() -> (usize, bool, usize) {
    let t = win::texto_ambiente("CM_SONDA_PILHA");
    let (dentro, e) = match t.split_once(':') {
        Some(("dentro", e)) => (true, e),
        Some(("depois", e)) => (false, e),
        _ => return (0, false, 0),
    };
    let (e, n) = e.split_once(':').unwrap_or((e, "0"));
    (usize::from_str_radix(e.trim_start_matches("0x"), 16).unwrap_or(0), dentro, n.parse().unwrap_or(0))
}

fn hex(t: &str) -> u32 {
    u32::from_str_radix(t.trim().trim_start_matches("0x"), 16).unwrap_or(0)
}

/// A cadeia viva acima da entrada: dwords que apontam logo depois de um call do exe.
unsafe extern "C" fn mostrar(entrada: u32) {
    let mut ret = Vec::new();
    for d in 0..0x1000u32 {
        let r = *((entrada + 4 * d) as *const u32);
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
    linha(&format!("sonda_pilha: chamada {} data {:?} entrada {:#x} retornos {}", CHAMADAS, *(0xae2c90 as *const [u16; 2]), entrada, ret.join(" ")));
}

/// Monta os trechos em volta do trampolim `tramp` da rotina; devolve o destino do desvio.
pub unsafe fn montar(tramp: usize, dentro: bool) -> usize {
    // CM_SONDA_FAIXA=ini-fim[/de-ate]: as chamadas vão na mesma variável (uma variável a mais desloca a heap e a carreira)
    let texto = win::texto_ambiente("CM_SONDA_FAIXA");
    let (faixa, chamadas) = texto.split_once('/').unwrap_or((&texto, ""));
    let (ini, fim) = faixa.split_once('-').map(|(a, b)| (hex(a) & !3, hex(b) & !3)).unwrap_or((0, 0));
    let veneno = hex(&win::texto_ambiente("CM_SONDA_VENENO"));
    let (de, ate) = chamadas.split_once('-').map(|(a, b)| (a.parse().unwrap_or(1u32), b.parse().unwrap_or(u32::MAX))).unwrap_or((1, u32::MAX));
    let mostra = win::ambiente("CM_SONDA_MOSTRAR");
    let pilha = win::VirtualAlloc(core::ptr::null_mut(), 1 << 16, win::MEM_COMMIT | win::MEM_RESERVE, win::PAGE_READWRITE) as u32 + (1 << 16) - 16;
    let codigo = win::VirtualAlloc(core::ptr::null_mut(), 512, win::MEM_COMMIT | win::MEM_RESERVE, win::PAGE_EXECUTE_READWRITE) as usize;
    TOPO = &raw mut RETORNOS as u32;
    let a = |v: *mut u32| (v as u32).to_le_bytes();
    let (eax, ecx, edi, topo) = (&raw mut TMP_EAX, &raw mut TMP_ECX, &raw mut TMP_EDI, &raw mut TOPO);
    let mut c: Vec<u8> = Vec::new();
    // nas chamadas de..=ate: envenenar [base − fim, base − ini) (base = esp na entrada, ecx na volta) e mostrar a cadeia;
    // eax e ecx já guardados
    let envenenar = |c: &mut Vec<u8>, pela_pilha: bool| {
        if fim <= ini && !mostra {
            return;
        }
        let mut v: Vec<u8> = Vec::new();
        if mostra {
            // mov [TMP_EDX], edx; mov [ESP_SALVO], esp; mov esp, pilha; push ecx (ou a entrada); call mostrar; mov esp, [ESP_SALVO]; mov edx, [TMP_EDX]
            v.extend([0x89, 0x15]); v.extend(a(&raw mut TMP_EDX));
            v.extend([0x89, 0x25]); v.extend(a(&raw mut ESP_SALVO));
            if pela_pilha {
                v.extend([0x89, 0xe1]); // mov ecx, esp (a entrada)
            }
            v.extend([0x89, 0x0d]); v.extend(a(&raw mut TMP_ENT)); // mov [TMP_ENT], ecx
            v.push(0xbc); v.extend(pilha.to_le_bytes());
            v.push(0x51);
            v.push(0xb8); v.extend((mostrar as usize as u32).to_le_bytes());
            v.extend([0xff, 0xd0]); // call eax
            v.extend([0x8b, 0x25]); v.extend(a(&raw mut ESP_SALVO));
            v.extend([0x8b, 0x15]); v.extend(a(&raw mut TMP_EDX));
            v.extend([0x8b, 0x0d]); v.extend(a(&raw mut TMP_ENT)); // mov ecx, [TMP_ENT]
        }
        if fim > ini {
            // mov [TMP_EDI], edi; lea edi, [base − fim]; mov ecx, n; mov eax, veneno; cld; rep stosd; mov edi, [TMP_EDI]
            v.extend([0x89, 0x3d]); v.extend(a(edi));
            v.extend(if pela_pilha { &[0x8d, 0xbc, 0x24][..] } else { &[0x8d, 0xb9][..] });
            v.extend((fim as i32).wrapping_neg().to_le_bytes());
            v.push(0xb9); v.extend(((fim - ini) / 4).to_le_bytes());
            v.push(0xb8); v.extend(veneno.to_le_bytes());
            v.extend([0xfc, 0xf3, 0xab]);
            v.extend([0x8b, 0x3d]); v.extend(a(edi));
        }
        // inc [CHAMADAS]; cmp [CHAMADAS], de; jb fim; cmp [CHAMADAS], ate; ja fim; <v>
        c.extend([0xff, 0x05]); c.extend(a(&raw mut CHAMADAS));
        c.extend([0x81, 0x3d]); c.extend(a(&raw mut CHAMADAS)); c.extend(de.to_le_bytes());
        c.extend([0x0f, 0x82]); c.extend((v.len() as u32 + 16).to_le_bytes());
        c.extend([0x81, 0x3d]); c.extend(a(&raw mut CHAMADAS)); c.extend(ate.to_le_bytes());
        c.extend([0x0f, 0x87]); c.extend((v.len() as u32).to_le_bytes());
        c.extend(v);
    };
    // entrada: guarda eax/ecx; empilha (retorno do jogo, esp) na RETORNOS
    c.push(0xa3); c.extend(a(eax));
    c.extend([0x89, 0x0d]); c.extend(a(ecx));
    c.push(0xa1); c.extend(a(topo));
    c.extend([0x8b, 0x0c, 0x24, 0x89, 0x08, 0x89, 0x60, 0x04]); // mov ecx, [esp]; mov [eax], ecx; mov [eax+4], esp
    c.extend([0x83, 0xc0, 0x08, 0xa3]); c.extend(a(topo));
    if dentro {
        envenenar(&mut c, true);
    }
    c.extend([0x8b, 0x0d]); c.extend(a(ecx));
    c.push(0xa1); c.extend(a(eax));
    c.extend([0x83, 0xc4, 0x04, 0xe8]); // add esp, 4 (tira o retorno do jogo); call trampolim (põe o nosso no mesmo lugar)
    let rel = (tramp as isize - (codigo + c.len() + 4) as isize) as i32;
    c.extend(rel.to_le_bytes());
    // volta: desempilha; o retorno do jogo volta ao seu dword (o resíduo fica o de sem desvio)
    c.push(0xa3); c.extend(a(eax));
    c.extend([0x89, 0x0d]); c.extend(a(ecx));
    c.push(0xa1); c.extend(a(topo));
    c.extend([0x83, 0xe8, 0x08, 0xa3]); c.extend(a(topo));
    c.extend([0x8b, 0x48, 0x04]); // mov ecx, [eax+4] (a entrada)
    c.extend([0x8b, 0x00, 0x89, 0x01, 0xa3]); c.extend(a(&raw mut ALVO)); // mov eax, [eax]; mov [ecx], eax; mov [ALVO], eax
    if !dentro {
        envenenar(&mut c, false);
    }
    c.extend([0x8b, 0x0d]); c.extend(a(ecx));
    c.push(0xa1); c.extend(a(eax));
    c.extend([0xff, 0x25]); c.extend(a(&raw mut ALVO));
    assert!(c.len() <= 512);
    core::ptr::copy_nonoverlapping(c.as_ptr(), codigo as *mut u8, c.len());
    linha(&format!("sonda_pilha: {}, faixa {:#x}..{:#x} abaixo da entrada com {:#x}, chamadas {}..{}",
        if dentro { "dentro" } else { "depois" }, ini, fim, veneno, de, ate));
    codigo
}

// ---------- varredura das leitoras (CM_SONDA_LISTA=1, engenharia-reversa/leitoras.py) ----------

/// Lê `sonda_lista.txt` (endereço, bytes do prólogo, quadro, marcada; hex/dec/hex/0-1) e desvia cada rotina marcada para
/// um trecho que envenena o próprio quadro na entrada ([esp − quadro, esp), com 0x5a5a5a5a) e segue para a original.
/// Sem marca não há desvio (o trecho é transparente, provado com a sonda de uma rotina). Tudo num bloco só (milhares de
/// rotinas: um VirtualAlloc por trampolim gastaria 64 KB de endereços cada).
pub unsafe fn instalar_lista() {
    if !win::ambiente("CM_SONDA_LISTA") {
        return;
    }
    let texto = std::fs::read_to_string("sonda_lista.txt").unwrap_or_default();
    let marcadas: Vec<(u32, usize, u32)> = texto
        .lines()
        .filter_map(|l| {
            let c: Vec<&str> = l.split_whitespace().collect();
            (c.len() == 4 && c[3] == "1").then(|| (hex(c[0]), c[1].parse().unwrap_or(0), hex(c[2]) & !3))
        })
        .filter(|&(a, n, q)| a != 0 && n >= 5 && q != 0)
        .collect();
    let bloco = win::VirtualAlloc(core::ptr::null_mut(), 96 * marcadas.len() + 64, win::MEM_COMMIT | win::MEM_RESERVE, win::PAGE_EXECUTE_READWRITE) as usize;
    if bloco == 0 {
        linha("sonda_pilha: lista sem memória");
        return;
    }
    let a = |v: *mut u32| (v as u32).to_le_bytes();
    let mut pos = bloco;
    for &(end, n, quadro) in &marcadas {
        let original = core::slice::from_raw_parts(end as *const u8, n).to_vec();
        // trampolim: o prólogo e o jmp de volta
        let tramp = pos;
        let mut c: Vec<u8> = original.clone();
        c.push(0xe9);
        c.extend(((end as usize + n) as i32).wrapping_sub((tramp + c.len() + 4) as i32).to_le_bytes());
        // trecho: guarda eax/ecx/edi; lea edi, [esp − quadro]; mov ecx, quadro/4; mov eax, veneno; cld; rep stosd; volta; jmp trampolim
        let trecho = tramp + c.len();
        c.push(0xa3); c.extend(a(&raw mut TMP_EAX));
        c.extend([0x89, 0x0d]); c.extend(a(&raw mut TMP_ECX));
        c.extend([0x89, 0x3d]); c.extend(a(&raw mut TMP_EDI));
        c.extend([0x8d, 0xbc, 0x24]); c.extend((quadro as i32).wrapping_neg().to_le_bytes());
        c.push(0xb9); c.extend((quadro / 4).to_le_bytes());
        c.push(0xb8); c.extend(0x5a5a_5a5au32.to_le_bytes());
        c.extend([0xfc, 0xf3, 0xab]);
        c.extend([0x8b, 0x3d]); c.extend(a(&raw mut TMP_EDI));
        c.extend([0x8b, 0x0d]); c.extend(a(&raw mut TMP_ECX));
        c.push(0xa1); c.extend(a(&raw mut TMP_EAX));
        c.push(0xe9);
        c.extend((tramp as i32).wrapping_sub((bloco + (c.len() + (tramp - bloco)) + 4) as i32).to_le_bytes());
        assert!(c.len() <= 96);
        core::ptr::copy_nonoverlapping(c.as_ptr(), tramp as *mut u8, c.len());
        // a entrada vira jmp trecho
        let mut antiga = 0u32;
        win::VirtualProtect(end as *mut core::ffi::c_void, 5, win::PAGE_EXECUTE_READWRITE, &mut antiga);
        *(end as *mut u8) = 0xe9;
        core::ptr::write_unaligned((end + 1) as *mut i32, (trecho as i32).wrapping_sub(end as i32 + 5));
        win::VirtualProtect(end as *mut core::ffi::c_void, 5, antiga, &mut antiga);
        pos += 96;
    }
    win::FlushInstructionCache(win::GetCurrentProcess(), core::ptr::null(), 0);
    linha(&format!("sonda_pilha: lista com {} rotinas marcadas", marcadas.len()));
}
