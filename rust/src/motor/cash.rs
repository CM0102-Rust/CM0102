use super::*;

// ---------- 271a leva: a formação lida do arquivo .pct (as táticas da IA e as do humano) e a troca da ordem dos bytes de palavras e dwords (formation.cpp, cash.cpp) ----------



/// fread (0x945bfe, cdecl): `n` registros de `tam` bytes; devolve quantos leu.
pub(crate) unsafe fn cqg8_ler(buf: usize, tam: u32, f: usize) -> u32 {
    let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
    fread(buf, tam, 1, f)
}

/// fclose (0x945a09, cdecl).
pub(crate) unsafe fn cqg8_fechar(f: usize) {
    let fclose: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
    fclose(f);
}

/// A caixa de erro de formation.cpp (0x9cc3dc) na linha `linha`. A original também formata a mensagem num texto local
/// (sprintf 0x944c9f) que não sai dali.
pub(crate) unsafe fn cqg8_erro(linha: u32) {
    aviso_em(0x9cc3dc, linha);
}

/// A troca dos bytes de um dword (a original faz em linha).
pub(crate) fn cqg8_inverter(v: u32) -> u32 {
    v.swap_bytes()
}

/// A leitura que falha e encerra: sem `quieto`, a caixa; o arquivo fecha e devolve 0.
pub(crate) unsafe fn cqg8_falhou(quieto: u32, linha: u32, f: usize) -> u32 {
    if quieto == 0 {
        cqg8_erro(linha);
    }
    cqg8_fechar(f);
    0
}



/// swaps the two bytes of each word of an array
/// Original: `swap_bytes_16_array` (0x43f2d0, cash.cpp, cdecl).
///
/// [pt] { 0x43f2d0 } (cdecl, cash.cpp): `swap_bytes_16_array(palavras, conta)`: troca os dois bytes de cada palavra.
pub(crate) unsafe extern "C" fn trocar_bytes_das_palavras(p: usize, conta: u32) {
    for k in 0..conta as usize {
        let a = (p + 2 * k) as *mut u16;
        core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).swap_bytes());
    }
}

/// Comparador: lado a lado no vetor (até 0x420 palavras conferidas).
pub(crate) unsafe extern "C" fn comparar_trocar_bytes_das_palavras(p: usize, conta: u32) {
    let o = orig::<unsafe extern "C" fn(usize, u32)>(1633);
    if p == 0 || conta == 0 || conta > 0x1000 {
        return o(p, conta);
    }
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&[(p, 2 * conta as usize)], || o(p, conta), || trocar_bytes_das_palavras(p, conta));
    PLACAR.registrar(igual, || format!("trocar_bytes_das_palavras({:x}, {})", p, conta));
}

/// reverses the four bytes of each dword of an array
/// Original: `swap_bytes_32_array` (0x43f300, cash.cpp, cdecl).
///
/// [pt] { 0x43f300 } (cdecl, cash.cpp): `swap_bytes_32_array(dwords, conta)`: inverte os quatro bytes de cada dword.
pub(crate) unsafe extern "C" fn trocar_bytes_dos_dwords(p: usize, conta: u32) {
    for k in 0..conta as usize {
        write_u32(p + 4 * k, cqg8_inverter(read_u32(p + 4 * k)));
    }
}

/// Comparador: lado a lado no vetor.
pub(crate) unsafe extern "C" fn comparar_trocar_bytes_dos_dwords(p: usize, conta: u32) {
    let o = orig::<unsafe extern "C" fn(usize, u32)>(1634);
    if p == 0 || conta == 0 || conta > 0x1000 {
        return o(p, conta);
    }
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&[(p, 4 * conta as usize)], || o(p, conta), || trocar_bytes_dos_dwords(p, conta));
    PLACAR.registrar(igual, || format!("trocar_bytes_dos_dwords({:x}, {})", p, conta));
}

