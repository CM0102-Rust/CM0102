// ---------- 168a leva: o fechamento do motor (2): os zagueiros do lado, o arbitro e o jogador da partida ----------

/// { 0x6d5b20 } (thiscall, ret 4, match_eng.cpp): os zagueiros do lado `lado` sem conflito (0x5b8dc0
/// na formação +0xaac5, ou +0x91e2 com lado 0).
unsafe extern "thiscall" fn zagueiros_do_lado_sem_conflito(m: usize, lado: i8) -> u8 {
    let f: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5b8dc0usize);
    f(m + if lado != 0 { 0xaac5 } else { 0x91e2 })
}

/// { 0x6e9460 } (thiscall, match_official.cpp): o construtor do árbitro: zera o motor (+0) e o oficial
/// (+4). Devolve `this`.
unsafe extern "thiscall" fn construir_arbitro(a: usize) -> usize {
    core::ptr::write_unaligned(a as *mut u32, 0);
    core::ptr::write_unaligned((a + 4) as *mut u32, 0);
    a
}

/// { 0x6ebd60 } (thiscall, match_pl.cpp): o destrutor do jogador da partida: zera o ponteiro do motor
/// (+0x19e).
unsafe extern "thiscall" fn destruir_jogador_da_partida(j: usize) {
    core::ptr::write_unaligned((j + 0x19e) as *mut u32, 0);
}

comparar_valor!(comparar_zagueiros_do_lado_sem_conflito, zagueiros_do_lado_sem_conflito, 376, (lado: i8) -> u8);

unsafe extern "thiscall" fn comparar_construir_arbitro(a: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(377);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(a, 8)], || o(a), || construir_arbitro(a));
    PLACAR.registrar(igual && r == v, || format!("construir_arbitro({:#x}) original={:#x} {:x?} rust={:#x} {:x?}", a, r, mo, v, mr));
    r
}

unsafe extern "thiscall" fn comparar_destruir_jogador_da_partida(j: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(378);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(j + 0x19e, 4)], || o(j), || destruir_jogador_da_partida(j));
    PLACAR.registrar(igual, || format!("destruir_jogador_da_partida({:#x}) {:x?} x {:x?}", j, mo, mr));
}
