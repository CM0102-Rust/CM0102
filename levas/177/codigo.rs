// ---------- 177a leva: o bit do contrato, a marca da competicao e o registro do clube na liga ----------

/// { 0x7a7eb0 } (stdcall, ret 4, player_regen.cpp): os bits 4..5 do byte +0xb do registro de 79 bytes
/// da pessoa `*p` em [0xae2d0c]. Devolve só `al`.
unsafe extern "stdcall" fn bits_do_registro(p: *const i32) -> u8 {
    let r = (*(0xae2d0c as *const usize)).wrapping_add((*p as usize).wrapping_mul(79));
    (*((r + 0xb) as *const u8) >> 4) & 3
}

/// { 0x684790 } (thiscall, league.cpp): a competição tem o byte +0xbe e a marca 2 de +0xd9?
unsafe extern "thiscall" fn competicao_marcada(t: usize) -> u32 {
    (*((t + 0xbe) as *const u8) != 0 && *((t + 0xd9) as *const u8) & 2 != 0) as u32
}

/// { 0x6826d0 } (thiscall, ret 4, league.cpp): o registro `i` (59 bytes) da tabela [this+0xb1].
unsafe extern "thiscall" fn registro_da_liga(t: usize, i: i32) -> usize {
    core::ptr::read_unaligned((t + 0xb1) as *const usize).wrapping_add((i as usize).wrapping_mul(59))
}

comparar_valor!(comparar_competicao_marcada, competicao_marcada, 404, () -> u32);
comparar_valor!(comparar_registro_da_liga, registro_da_liga, 405, (i: i32) -> usize);

unsafe extern "stdcall" fn comparar_bits_do_registro(p: *const i32) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(*const i32) -> u8>(403)(p);
    let n = bits_do_registro(p);
    PLACAR.registrar(r == n, || format!("bits_do_registro({:p}) original={} rust={}", p, r, n));
    r
}
