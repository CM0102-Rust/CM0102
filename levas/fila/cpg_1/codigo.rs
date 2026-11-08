// ---------- %%LEVA%%a leva: as cópias da notícia (news.cpp) e das transferências passada e futura (transfer_manager.cpp) ----------

/// Date::operator= (0x417c10): copia a data de `origem` para `destino`, com o retorno numa local (gravada pela própria).
unsafe fn cpg1_data(destino: usize, origem: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut local = [0u8; 8];
    copiar(destino, &mut local, origem);
}

/// { 0x786010 } (thiscall, ret 4, news.cpp): `NewsItem::operator=`: copia a notícia (0xdf bytes) de `origem`: +0
/// (dupla), +4 (byte), as 50 fichas de +5 (duplas, na ordem), a data em +0xcd, +0xd5 (byte), +0xd6 e +0xda (duplas) e
/// +0xde (byte). Devolve `this`.
unsafe extern "thiscall" fn news_operator(this: usize, origem: usize) -> usize {
    let d = |o: usize| core::ptr::write_unaligned((this + o) as *mut u32, core::ptr::read_unaligned((origem + o) as *const u32));
    let b = |o: usize| *((this + o) as *mut u8) = *((origem + o) as *const u8);
    d(0);
    b(4);
    for k in 0..50 {
        d(5 + 4 * k); // as fichas da notícia
    }
    cpg1_data(this + 0xcd, origem + 0xcd);
    b(0xd5);
    d(0xd6);
    d(0xda);
    b(0xde);
    this
}

/// { 0x8cb940 } (thiscall, ret 4, transfer_manager.cpp): `past_transfer_copy_construct`: copia a transferência passada
/// (0x25 bytes) de `origem`: sete duplas (+0..+0x18), o byte +0x1c e a data em +0x1d. Devolve `this`.
unsafe extern "thiscall" fn past_transfer_copy_construct(this: usize, origem: usize) -> usize {
    for k in 0..7 {
        core::ptr::write_unaligned((this + 4 * k) as *mut u32, core::ptr::read_unaligned((origem + 4 * k) as *const u32)); // as duplas da transferência
    }
    *((this + 0x1c) as *mut u8) = *((origem + 0x1c) as *const u8);
    cpg1_data(this + 0x1d, origem + 0x1d);
    this
}

/// { 0x8cbf40 } (thiscall, ret 4, transfer_manager.cpp): `future_transfer_assign`: copia a transferência futura (0xc
/// bytes) de `origem`: a data em +0 e o índice da oferta (+8). Devolve `this`.
unsafe extern "thiscall" fn future_transfer_assign(this: usize, origem: usize) -> usize {
    cpg1_data(this, origem);
    core::ptr::write_unaligned((this + 8) as *mut u32, core::ptr::read_unaligned((origem + 8) as *const u32)); // o índice da oferta
    this
}

unsafe extern "thiscall" fn comparar_news_operator(this: usize, origem: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0xdf)], || o(this, origem), || news_operator(this, origem));
    PLACAR.registrar(igual && r == n, || format!("news_operator {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_past_transfer_copy_construct(this: usize, origem: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x25)], || o(this, origem), || past_transfer_copy_construct(this, origem));
    PLACAR.registrar(igual && r == n, || format!("past_transfer_copy_construct {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_future_transfer_assign(this: usize, origem: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0xc)], || o(this, origem), || future_transfer_assign(this, origem));
    PLACAR.registrar(igual && r == n, || format!("future_transfer_assign {:x?} x {:x?}", mo, mr));
    r
}
