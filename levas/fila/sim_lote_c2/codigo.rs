// ---------- %%LEVA%%a leva: as cópias do recordista, do registro de partida e do registro de sequência dos recordes do clube (club_records.cpp) ----------

/// Date::operator= (0x417c10): copia a data de `origem` para `destino`, com o retorno numa local.
unsafe fn copiar_data_do_recorde(destino: usize, origem: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut local = [0u8; 8];
    copiar(destino, &mut local, origem);
}

/// { 0x448490 } (thiscall, ret 4, club_records.cpp): copia o recordista (0x18 bytes) de `origem`:
/// quatro palavras duplas de identificação (+0..+0xc) e a data em +0x10. Devolve `this`.
unsafe extern "thiscall" fn record_holder_assign(this: usize, origem: usize) -> usize {
    let d = |o: usize| core::ptr::write_unaligned((this + o) as *mut u32, core::ptr::read_unaligned((origem + o) as *const u32));
    d(0); // identificação
    d(4);
    d(8);
    d(0xc);
    copiar_data_do_recorde(this + 0x10, origem + 0x10);
    this
}

/// { 0x448110 } (thiscall, ret 4, club_records.cpp): copia o registro de partida (0x1f bytes) de
/// `origem`: a data em +0, depois +8 (dupla), +0xc e +0xe (palavras), +0x10 (byte), +0x11 e +0x15
/// (duplas), +0x19 e +0x1a (bytes) e +0x1b (dupla). Devolve `this`.
unsafe extern "thiscall" fn match_record_assign(this: usize, origem: usize) -> usize {
    let d = |o: usize| core::ptr::write_unaligned((this + o) as *mut u32, core::ptr::read_unaligned((origem + o) as *const u32));
    let w = |o: usize| core::ptr::write_unaligned((this + o) as *mut u16, core::ptr::read_unaligned((origem + o) as *const u16));
    let b = |o: usize| *((this + o) as *mut u8) = *((origem + o) as *const u8);
    copiar_data_do_recorde(this, origem);
    d(8); // adversário
    w(0xc);
    w(0xe);
    b(0x10);
    d(0x11);
    d(0x15);
    b(0x19);
    b(0x1a);
    d(0x1b);
    this
}

/// { 0x448170 } (thiscall, ret 4, club_records.cpp): copia o registro de sequência (0x14 bytes) de
/// `origem`: o número de jogos (+0) e as datas de início (+4) e fim (+0xc). Devolve `this`.
unsafe extern "thiscall" fn run_record_assign(this: usize, origem: usize) -> usize {
    *(this as *mut u32) = *(origem as *const u32); // jogos
    copiar_data_do_recorde(this + 4, origem + 4);
    copiar_data_do_recorde(this + 0xc, origem + 0xc);
    this
}

unsafe extern "thiscall" fn comparar_record_holder_assign(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x18)], || o(this, origem), || record_holder_assign(this, origem));
    PLACAR.registrar(igual && r == n, || format!("record_holder_assign {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_match_record_assign(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1f)], || o(this, origem), || match_record_assign(this, origem));
    PLACAR.registrar(igual && r == n, || format!("match_record_assign {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_run_record_assign(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this, origem), || run_record_assign(this, origem));
    PLACAR.registrar(igual && r == n, || format!("run_record_assign {:x?} x {:x?}", mo, mr));
    r
}
