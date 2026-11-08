use super::*;



/// CRT fread of one record of tam bytes; returns records read.
///
/// [pt] fread (0x945bfe, cdecl): `n` registros de `tam` bytes; devolve quantos leu.
pub(crate) unsafe fn crt_fread_one(buf: usize, tam: u32, f: usize) -> u32 {
    let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
    fread(buf, tam, 1, f)
}

/// CRT fclose.
///
/// [pt] fclose (0x945a09, cdecl).
pub(crate) unsafe fn cash_crt_fclose(f: usize) {
    let fclose: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
    fclose(f);
}

/// Error box of formation.cpp at the given line.
///
/// [pt] A caixa de erro de formation.cpp (0x9cc3dc) na linha `linha`. A original também formata a mensagem num texto local
/// (sprintf 0x944c9f) que não sai dali.
pub(crate) unsafe fn formation_error_box(linha: u32) {
    show_error_at(0x9cc3dc, linha);
}

/// Byte swap of a dword.
///
/// [pt] A troca dos bytes de um dword (a original faz em linha).
pub(crate) fn swap_bytes_u32(v: u32) -> u32 {
    v.swap_bytes()
}

/// Failed read: box unless quiet, closes the file and returns 0.
///
/// [pt] A leitura que falha e encerra: sem `quieto`, a caixa; o arquivo fecha e devolve 0.
pub(crate) unsafe fn fail_read(quieto: u32, linha: u32, f: usize) -> u32 {
    if quieto == 0 {
        formation_error_box(linha);
    }
    cash_crt_fclose(f);
    0
}



/// swaps the two bytes of each word of an array
/// Original: `swap_bytes_16_array` (0x43f2d0, cash.cpp, cdecl).
///
/// [pt] { 0x43f2d0 } (cdecl, cash.cpp): `swap_bytes_16_array(palavras, conta)`: troca os dois bytes de cada palavra.
pub(crate) unsafe extern "C" fn swap_bytes_16_array(p: usize, conta: u32) {
    for k in 0..conta as usize {
        let a = (p + 2 * k) as *mut u16;
        core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).swap_bytes());
    }
}


/// reverses the four bytes of each dword of an array
/// Original: `swap_bytes_32_array` (0x43f300, cash.cpp, cdecl).
///
/// [pt] { 0x43f300 } (cdecl, cash.cpp): `swap_bytes_32_array(dwords, conta)`: inverte os quatro bytes de cada dword.
pub(crate) unsafe extern "C" fn swap_bytes_32_array(p: usize, conta: u32) {
    for k in 0..conta as usize {
        write_u32(p + 4 * k, swap_bytes_u32(read_u32(p + 4 * k)));
    }
}


