// ---------- %%LEVA%%a leva: o match_wrap_up do motor (1): o gol da bola, a partida abandonada e as vagas do lado ----------

/// { 0x6d58f0 } (thiscall, match_eng.cpp): a bola numa área (casas centrais +0x8ea7 de 2 a 6): 1 na
/// área de cima (linha +0x8ea8 0 ou 1), −1 na de baixo (10 ou 11), senão 0. Devolve só `al`.
unsafe extern "thiscall" fn ball_goal_end(m: usize) -> i8 {
    let linha = *((m + 0x8ea8) as *const i8);
    let coluna = *((m + 0x8ea7) as *const i8);
    if (2..=6).contains(&coluna) {
        if (0..=1).contains(&linha) {
            return 1;
        }
        if (10..=11).contains(&linha) {
            return -1;
        }
    }
    0
}

/// { 0x6d5930 } (thiscall, match_eng.cpp): a partida foi abandonada? O minuto do abandono (+0xf638,
/// −1 sem abandono) × 11 abaixo do relógio (+0x8ed0), em 16 bits. Devolve só `al`.
unsafe extern "thiscall" fn match_abandoned(m: usize) -> u8 {
    let minuto = *((m + 0xf638) as *const i8);
    if minuto == -1 {
        return 0;
    }
    ((minuto as i16).wrapping_mul(11) < core::ptr::read_unaligned((m + 0x8ed0) as *const i16)) as u8
}

/// { 0x6d5af0 } (thiscall, ret 4, match_eng.cpp): as vagas do lado `lado` sem conflito (0x5b8960 na
/// formação +0xaac5, ou +0x91e2 com lado 0).
unsafe extern "thiscall" fn side_slots_without_conflict(m: usize, lado: i8) -> u8 {
    let f: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5b8960usize);
    f(m + if lado != 0 { 0xaac5 } else { 0x91e2 })
}

comparar_valor!(comparar_ball_goal_end, ball_goal_end, %%I0%%, () -> i8);
comparar_valor!(comparar_match_abandoned, match_abandoned, %%I1%%, () -> u8);
comparar_valor!(comparar_side_slots_without_conflict, side_slots_without_conflict, %%I2%%, (lado: i8) -> u8);
