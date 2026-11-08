// ---------- %%LEVA%%a leva: os recordes do clube: a construção dos recordes de vitórias, derrotas e invencibilidade seguidas ----------

/// { 0x450ad0 } (thiscall, club_records.cpp): `won_run_record_construct`: constrói o recorde de vitórias seguidas: as
/// datas do começo (+4) e do fim (+0xc) (0x54a110). O quadro SEH do original não é preciso (a 0x54a110 não falha).
/// Devolve `this`.
unsafe extern "thiscall" fn won_run_record_construct(this: usize) -> usize {
    cpf2_data_padrao(this + 4); // o começo das vitórias
    cpf2_data_padrao(this + 0xc);
    this
}

/// { 0x450b20 } (thiscall, club_records.cpp): `lost_run_record_construct`: constrói o recorde de derrotas seguidas: as
/// datas do começo (+4) e do fim (+0xc) (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn lost_run_record_construct(this: usize) -> usize {
    cpf2_data_padrao(this + 4);
    cpf2_data_padrao(this + 0xc); // o fim das derrotas
    this
}

/// { 0x450b70 } (thiscall, club_records.cpp): `unbeaten_run_record_construct`: constrói o recorde de jogos sem perder:
/// as datas do começo (+4) e do fim (+0xc) (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn unbeaten_run_record_construct(this: usize) -> usize {
    cpf2_data_padrao(this + 4); // o começo da invencibilidade
    cpf2_data_padrao(this + 0xc);
    this
}

unsafe extern "thiscall" fn comparar_won_run_record_construct(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || won_run_record_construct(this));
    PLACAR.registrar(igual && r == n, || format!("won_run_record_construct {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_lost_run_record_construct(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(%%I1%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || lost_run_record_construct(this));
    PLACAR.registrar(igual && r == n, || format!("lost_run_record_construct {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_unbeaten_run_record_construct(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || unbeaten_run_record_construct(this));
    PLACAR.registrar(igual && r == n, || format!("unbeaten_run_record_construct {:x?} x {:x?}", mo, mr));
    r
}
