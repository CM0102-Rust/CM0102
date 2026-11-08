// ---------- %%LEVA%%a leva: a media do jogo, o valor do contrato e o nivel para a selecao ----------

/// { 0x5d5a60 } (thiscall, ret 4, game.cpp): a média +0x20/+0x28 (com `segunda`) ou +0x10/+0x18 (f64,
/// como a original deixa no x87); com divisor zero (ou não comparável), 6.8.
unsafe extern "thiscall" fn game_average(t: usize, segunda: u8) -> f64 {
    let d = |o: usize| core::ptr::read_unaligned((t + o) as *const f64);
    let (n, q) = if segunda != 0 { (d(0x20), d(0x28)) } else { (d(0x10), d(0x18)) };
    if q == read_f64_at(0x9689a0) || q.is_nan() {
        return read_f64_at(0x968998);
    }
    n / q
}

/// { 0x4d0c20 } (thiscall, ret 4, contract_manager.cpp): o dword +0xc da ficha de contrato (0x50 bytes
/// a partir de [this]) da pessoa `*p`, pelo índice em +0 do registro de 79 bytes em [0xae2d0c]; 0 sem
/// pessoa, com id fora de [0xae2364], índice negativo ou ficha no endereço 0.
unsafe extern "thiscall" fn contract_value(t: usize, p: *const i32) -> u32 {
    if p.is_null() || *p >= *(0xae2364 as *const i32) {
        return 0;
    }
    let k = core::ptr::read_unaligned((*(0xae2d0c as *const usize)).wrapping_add((*p as usize).wrapping_mul(79)) as *const i32);
    if k < 0 {
        return 0;
    }
    let ficha = (*(t as *const usize)).wrapping_add(k as usize * 0x50);
    if ficha == 0 {
        return 0;
    }
    core::ptr::read_unaligned((ficha + 0xc) as *const u32)
}

/// { 0x76d810 } (stdcall, ret 0xc, national_teams.cpp): o nível 1..3 do jogador `j` para a seleção
/// (0 com reputação `*rep` abaixo de 100, ou 50 com `base`, ou goleiro de nota 15): 1 para líbero ou
/// zagueiro, 2 para volante, meia ou meia-atacante (nota 15 em +0x10..+0x14 da ficha), senão 3.
/// Devolve só `al`.
unsafe extern "stdcall" fn national_team_level(j: usize, rep: *const i16, base: u8) -> u8 {
    if *rep < if base != 0 { 0x32 } else { 0x64 } {
        return 0;
    }
    let ficha = core::ptr::read_unaligned((j + 0x61) as *const usize);
    let n = |o: usize| *((ficha + o) as *const i8) >= 0xf;
    if n(0xf) {
        0
    } else if n(0x10) || n(0x11) {
        1
    } else if n(0x12) || n(0x13) || n(0x14) {
        2
    } else {
        3
    }
}

unsafe extern "thiscall" fn comparar_game_average(t: usize, segunda: u8) -> f64 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u8) -> f64>(%%I0%%)(t, segunda);
    let n = game_average(t, segunda);
    PLACAR.registrar(o.to_bits() == n.to_bits(), || format!("game_average({:#x}, {}) original={} rust={}", t, segunda, o, n));
    o
}

comparar_valor!(comparar_contract_value, contract_value, %%I1%%, (p: *const i32) -> u32);

unsafe extern "stdcall" fn comparar_national_team_level(j: usize, rep: *const i16, base: u8) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, *const i16, u8) -> u8>(%%I2%%)(j, rep, base);
    let n = national_team_level(j, rep, base);
    PLACAR.registrar(r == n, || format!("national_team_level({:#x}, {}, {}) original={} rust={}", j, *rep, base, r, n));
    r
}
