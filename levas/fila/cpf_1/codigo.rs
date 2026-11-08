// ---------- %%LEVA%%a leva: as cópias dos recordes de nota, de melhor em campo e de disciplina dos recordes do clube (club_records.cpp) ----------

/// O recordista (0x18 bytes) de `origem` em `destino`, pela 0x448490 (`record_holder_assign`, já em Rust).
unsafe fn assign_record_holder(destino: usize, origem: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x448490usize);
    copiar(destino, origem);
}

/// { 0x448230 } (thiscall, ret 4, club_records.cpp): `rating_record_assign`: copia o recorde de nota (0x1f bytes) de
/// `origem`: a nota (+0, dupla), o recordista em +4, o byte +0x1c e a palavra +0x1d. Devolve `this`.
unsafe extern "thiscall" fn rating_record_assign(this: usize, origem: usize) -> usize {
    core::ptr::write_unaligned(this as *mut u32, core::ptr::read_unaligned(origem as *const u32)); // a nota
    assign_record_holder(this + 4, origem + 4);
    *((this + 0x1c) as *mut u8) = *((origem + 0x1c) as *const u8); // o byte do recorde de nota
    core::ptr::write_unaligned((this + 0x1d) as *mut u16, core::ptr::read_unaligned((origem + 0x1d) as *const u16));
    this
}

/// { 0x4484d0 } (thiscall, ret 4, club_records.cpp): `man_of_match_record_assign`: copia o recorde de melhor em campo
/// (0x1b bytes) de `origem`: as vezes (+0, byte), o recordista em +1 e o ano da temporada (+0x19). Devolve `this`.
unsafe extern "thiscall" fn man_of_match_record_assign(this: usize, origem: usize) -> usize {
    *(this as *mut u8) = *(origem as *const u8); // as vezes de melhor em campo
    assign_record_holder(this + 1, origem + 1);
    core::ptr::write_unaligned((this + 0x19) as *mut u16, core::ptr::read_unaligned((origem + 0x19) as *const u16));
    this
}

/// { 0x448500 } (thiscall, ret 4, club_records.cpp): `discipline_record_assign`: copia o recorde de disciplina (0x1c
/// bytes) de `origem`: os amarelos (+0) e vermelhos (+1), o recordista em +2 e o ano da temporada (+0x1a). Devolve `this`.
unsafe extern "thiscall" fn discipline_record_assign(this: usize, origem: usize) -> usize {
    *(this as *mut u8) = *(origem as *const u8);
    *((this + 1) as *mut u8) = *((origem + 1) as *const u8); // os vermelhos
    assign_record_holder(this + 2, origem + 2);
    core::ptr::write_unaligned((this + 0x1a) as *mut u16, core::ptr::read_unaligned((origem + 0x1a) as *const u16));
    this
}

unsafe extern "thiscall" fn compare_rating_record_assign(this: usize, origem: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x1f)], || o(this, origem), || rating_record_assign(this, origem));
    SCOREBOARD.record(igual && r == n, || format!("rating_record_assign {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn compare_man_of_match_record_assign(this: usize, origem: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x1b)], || o(this, origem), || man_of_match_record_assign(this, origem));
    SCOREBOARD.record(igual && r == n, || format!("man_of_match_record_assign {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn compare_discipline_record_assign(this: usize, origem: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x1c)], || o(this, origem), || discipline_record_assign(this, origem));
    SCOREBOARD.record(igual && r == n, || format!("discipline_record_assign {:x?} x {:x?}", mo, mr));
    r
}
