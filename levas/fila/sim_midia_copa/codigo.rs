// ---------- %%LEVA%%a leva: as notícias do jogador na mídia e a importância da partida de copa ----------

/// O registro de notícias da mídia (`this`: +0 os registros de 0x17 bytes, +8 quantos): algum com o
/// jogador em +8 que `aceita`?
unsafe fn any_news_for_player(midia: usize, id: i32, aceita: impl Fn(usize) -> bool) -> bool {
    let n = *((midia + 8) as *const i32);
    let base = *(midia as *const usize);
    (0..n.max(0) as usize).any(|k| {
        let r = base + k * 0x17;
        core::ptr::read_unaligned((r + 8) as *const i32) == id && aceita(r)
    })
}

/// { 0x735780 } (thiscall, ret 4, media.cpp): o jogador `*id` tem uma notícia com a resposta 2 (+0x10)?
unsafe extern "thiscall" fn story_with_response_two(midia: usize, id: *const i32) -> u32 {
    any_news_for_player(midia, *id, |r| *((r + 0x10) as *const u8) == 2) as u32
}

/// { 0x735840 } (thiscall, ret 4, media.cpp): o jogador `*id` tem alguma notícia? Devolve só `al`.
unsafe extern "thiscall" fn has_story(midia: usize, id: *const i32) -> u8 {
    any_news_for_player(midia, *id, |_| true) as u8
}

/// { 0x5221f0 } (thiscall, ret 0xc, Cup.cpp): a importância da partida da copa (a fase +0x34 da
/// partida menos +0x3c mais +0x51 da copa, pelo menos 1) e em `margem` a diferença do placar da ida
/// (+0x4b/+0x4c, 0xff sem ida) para o lado `segundo`, mais 1 (0xff abaixo de 1). Devolve só `al`.
unsafe extern "thiscall" fn cup_match_importance(copa: usize, partida: usize, segundo: u8, margem: *mut u8) -> u8 {
    let b = |p: usize, o: usize| *((p + o) as *const u8);
    let mut al = b(partida, 0x34).wrapping_sub(b(copa, 0x3c)).wrapping_add(b(copa, 0x51));
    if (al as i8) < 1 {
        al = 1;
    }
    let ida = b(partida, 0x4b);
    if ida != 0xff {
        let volta = b(partida, 0x4c);
        *margem = if segundo == 0 { volta.wrapping_sub(ida) } else { ida.wrapping_sub(volta) }.wrapping_add(1);
        if (*margem as i8) < 1 {
            *margem = 0xff;
        }
    }
    al
}

unsafe extern "thiscall" fn comparar_story_with_response_two(midia: usize, id: *const i32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const i32) -> u32>(%%I0%%)(midia, id);
    let n = story_with_response_two(midia, id);
    SCOREBOARD.record(r == n, || format!("story_with_response_two({}) original={} rust={}", *id, r, n));
    r
}

unsafe extern "thiscall" fn comparar_has_story(midia: usize, id: *const i32) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const i32) -> u8>(%%I1%%)(midia, id);
    let n = has_story(midia, id);
    SCOREBOARD.record(r == n, || format!("has_story({}) original={} rust={}", *id, r, n));
    r
}

unsafe extern "thiscall" fn comparar_cup_match_importance(copa: usize, partida: usize, segundo: u8, margem: *mut u8) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u8, *mut u8) -> u8>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(margem as usize, 1)], || o(copa, partida, segundo, margem), || cup_match_importance(copa, partida, segundo, margem));
    SCOREBOARD.record(igual && r == n, || format!("cup_match_importance(lado {}) original={} rust={} {:x?} x {:x?}", segundo, r, n, mo, mr));
    r
}
