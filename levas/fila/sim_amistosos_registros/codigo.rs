// ---------- %%LEVA%%a leva: a competição do amistoso, a data internacional e o registro de técnico (friendly.cpp, staff_records.cpp) ----------

/// { 0x5c2990 } (thiscall, ret 4, friendly.cpp): `competicao` é a competição dos amistosos (+4)?
unsafe extern "thiscall" fn is_own_comp_record(amistoso: usize, competicao: usize) -> u32 {
    (competicao == *((amistoso + 4) as *const usize)) as u32
}

/// { 0x5cb9b0 } (thiscall, ret 0x18, friendly.cpp): preenche a data internacional de 20 bytes: dia,
/// mês e duração (+0..+2), o ano a somar (+0xb), o continente (+0xc) e a competição (+0x10).
unsafe extern "thiscall" fn friendly_international_date_set(data: usize, dia: u8, mes: u8, duracao: u8, ano: u8, continente: u32, competicao: u32) {
    let b = data as *mut u8;
    *b = dia;
    *b.add(1) = mes;
    *b.add(2) = duracao;
    *b.add(0xb) = ano;
    core::ptr::write_unaligned(b.add(0xc) as *mut u32, continente);
    core::ptr::write_unaligned(b.add(0x10) as *mut u32, competicao);
}

/// O registro de técnico de 210 bytes da entrada `*id` ([registros+8]).
unsafe fn friendly_manager_record(registros: usize, id: *const i32) -> usize {
    (*((registros + 8) as *const usize) as isize + *id as isize * 210) as usize
}

/// { 0x86fc20 } (thiscall, ret 4, staff_records.cpp): zera o registro de técnico da entrada e põe −1
/// nos ids do dono, da liga e dos clubes das passagens (+0x4a, +0x5e, +0x72, +0x86, +0xc2, +0xc6).
unsafe extern "thiscall" fn clear_manager_record(registros: usize, id: *const i32) {
    let r = friendly_manager_record(registros, id);
    core::ptr::write_bytes(r as *mut u8, 0, 210);
    for o in [0x86, 0xc2, 0xc6, 0x4a, 0x5e, 0x72] {
        core::ptr::write_unaligned((r + o) as *mut i32, -1);
    }
}

unsafe extern "thiscall" fn comparar_is_own_comp_record(amistoso: usize, competicao: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(amistoso, competicao);
    let n = is_own_comp_record(amistoso, competicao);
    SCOREBOARD.record(r == n, || format!("is_own_comp_record({:#x}) original={} rust={}", competicao, r, n));
    r
}

unsafe extern "thiscall" fn comparar_friendly_international_date_set(data: usize, dia: u8, mes: u8, duracao: u8, ano: u8, continente: u32, competicao: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u8, u8, u8, u8, u32, u32)>(%%I1%%);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(data, 0x14)], || o(data, dia, mes, duracao, ano, continente, competicao),
        || friendly_international_date_set(data, dia, mes, duracao, ano, continente, competicao));
    SCOREBOARD.record(igual, || format!("friendly_international_date_set {:x?} x {:x?}", mo, mr));
}

unsafe extern "thiscall" fn comparar_clear_manager_record(registros: usize, id: *const i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32)>(%%I2%%);
    let r = friendly_manager_record(registros, id);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(r, 210)], || o(registros, id), || clear_manager_record(registros, id));
    SCOREBOARD.record(igual, || format!("clear_manager_record({}) {:x?} x {:x?}", *id, mo, mr));
}
