// ---------- 181a leva: as notícias do jogador na mídia e a importância da partida de copa ----------

/// O registro de notícias da mídia (`this`: +0 os registros de 0x17 bytes, +8 quantos): algum com o
/// jogador em +8 que `aceita`?
unsafe fn alguma_noticia(midia: usize, id: i32, aceita: impl Fn(usize) -> bool) -> bool {
    let n = *((midia + 8) as *const i32);
    let base = *(midia as *const usize);
    (0..n.max(0) as usize).any(|k| {
        let r = base + k * 0x17;
        core::ptr::read_unaligned((r + 8) as *const i32) == id && aceita(r)
    })
}

/// { 0x735780 } (thiscall, ret 4, media.cpp): o jogador `*id` tem uma notícia com a resposta 2 (+0x10)?
unsafe extern "thiscall" fn noticia_com_resposta_2(midia: usize, id: *const i32) -> u32 {
    alguma_noticia(midia, *id, |r| *((r + 0x10) as *const u8) == 2) as u32
}

/// { 0x735840 } (thiscall, ret 4, media.cpp): o jogador `*id` tem alguma notícia? Devolve só `al`.
unsafe extern "thiscall" fn tem_noticia(midia: usize, id: *const i32) -> u8 {
    alguma_noticia(midia, *id, |_| true) as u8
}

/// { 0x5221f0 } (thiscall, ret 0xc, Cup.cpp): a importância da partida da copa (a fase +0x34 da
/// partida menos +0x3c mais +0x51 da copa, pelo menos 1) e em `margem` a diferença do placar da ida
/// (+0x4b/+0x4c, 0xff sem ida) para o lado `segundo`, mais 1 (0xff abaixo de 1). Devolve só `al`.
unsafe extern "thiscall" fn importancia_da_partida(copa: usize, partida: usize, segundo: u8, margem: *mut u8) -> u8 {
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

unsafe extern "thiscall" fn comparar_noticia_com_resposta_2(midia: usize, id: *const i32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, *const i32) -> u32>(415)(midia, id);
    let n = noticia_com_resposta_2(midia, id);
    PLACAR.registrar(r == n, || format!("noticia_com_resposta_2({}) original={} rust={}", *id, r, n));
    r
}

unsafe extern "thiscall" fn comparar_tem_noticia(midia: usize, id: *const i32) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, *const i32) -> u8>(416)(midia, id);
    let n = tem_noticia(midia, id);
    PLACAR.registrar(r == n, || format!("tem_noticia({}) original={} rust={}", *id, r, n));
    r
}

unsafe extern "thiscall" fn comparar_importancia_da_partida(copa: usize, partida: usize, segundo: u8, margem: *mut u8) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u8, *mut u8) -> u8>(417);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(margem as usize, 1)], || o(copa, partida, segundo, margem), || importancia_da_partida(copa, partida, segundo, margem));
    PLACAR.registrar(igual && r == n, || format!("importancia_da_partida(lado {}) original={} rust={} {:x?} x {:x?}", segundo, r, n, mo, mr));
    r
}
