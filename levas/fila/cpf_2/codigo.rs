// ---------- %%LEVA%%a leva: a cópia do recorde de contagem e do torcedor e a construção dos recordes de contagem (club_records.cpp) ----------

/// { 0x448530 } (thiscall, ret 4, club_records.cpp): `count_record_assign`: copia o recorde de contagem (0x1a bytes) de
/// `origem`: a contagem (+0, palavra) e o recordista em +2 (pela 0x448490, já em Rust). Devolve `this`.
unsafe extern "thiscall" fn count_record_assign(this: usize, origem: usize) -> usize {
    core::ptr::write_unaligned(this as *mut u16, core::ptr::read_unaligned(origem as *const u16)); // a contagem
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x448490usize);
    copiar(this + 2, origem + 2);
    this
}

/// { 0x4485b0 } (thiscall, ret 4, club_records.cpp): `fans_player_record_assign`: copia o recorde de jogador da
/// torcida (0x1a bytes) de `origem`: o recordista em +0 (pela 0x448490) e o ano da temporada (+0x18). Devolve `this`.
unsafe extern "thiscall" fn fans_player_record_assign(this: usize, origem: usize) -> usize {
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x448490usize);
    copiar(this, origem);
    core::ptr::write_unaligned((this + 0x18) as *mut u16, core::ptr::read_unaligned((origem + 0x18) as *const u16)); // o ano do torcedor
    this
}

/// A data do recordista em `a` pela 0x54a110 (o construtor da data: dia 40, ano 1972, +4 = 1).
unsafe fn init_default_date(a: usize) {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    data_padrao(a);
}

/// { 0x450c60 } (thiscall, club_records.cpp): `tally_record_construct`: constrói o recorde de ocorrências (contagem em
/// byte + recordista): só a data do recordista em +0x11 (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn tally_record_construct(this: usize) -> usize {
    init_default_date(this + 0x11); // a data das ocorrências
    this
}

/// { 0x450d30 } (thiscall, club_records.cpp): `count_record_construct`: constrói o recorde de contagem (palavra +
/// recordista): só a data do recordista em +0x12 (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn count_record_construct(this: usize) -> usize {
    init_default_date(this + 0x12); // a data da contagem
    this
}

unsafe extern "thiscall" fn comparar_count_record_assign(this: usize, origem: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x1a)], || o(this, origem), || count_record_assign(this, origem));
    SCOREBOARD.record(igual && r == n, || format!("count_record_assign {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_fans_player_record_assign(this: usize, origem: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x1a)], || o(this, origem), || fans_player_record_assign(this, origem));
    SCOREBOARD.record(igual && r == n, || format!("fans_player_record_assign {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_tally_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x19)], || o(this), || tally_record_construct(this));
    SCOREBOARD.record(igual && r == n, || format!("tally_record_construct {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_count_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I3%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x1a)], || o(this), || count_record_construct(this));
    SCOREBOARD.record(igual && r == n, || format!("count_record_construct {:x?} x {:x?}", mo, mr));
    r
}
