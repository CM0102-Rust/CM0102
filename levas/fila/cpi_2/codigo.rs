// ---------- %%LEVA%%a leva: os recordes do clube: a construção dos recordes de vitórias, derrotas e invencibilidade seguidas ----------

/// { 0x450ad0 } (thiscall, club_records.cpp): `won_run_record_construct`: constrói o recorde de vitórias seguidas: as
/// datas do começo (+4) e do fim (+0xc) (0x54a110). O quadro SEH do original não é preciso (a 0x54a110 não falha).
/// Devolve `this`.
unsafe extern "thiscall" fn construir_recorde_de_vitorias_seguidas(this: usize) -> usize {
    cpf2_data_padrao(this + 4); // o começo das vitórias
    cpf2_data_padrao(this + 0xc);
    this
}

/// { 0x450b20 } (thiscall, club_records.cpp): `lost_run_record_construct`: constrói o recorde de derrotas seguidas: as
/// datas do começo (+4) e do fim (+0xc) (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn construir_recorde_de_derrotas_seguidas(this: usize) -> usize {
    cpf2_data_padrao(this + 4);
    cpf2_data_padrao(this + 0xc); // o fim das derrotas
    this
}

/// { 0x450b70 } (thiscall, club_records.cpp): `unbeaten_run_record_construct`: constrói o recorde de jogos sem perder:
/// as datas do começo (+4) e do fim (+0xc) (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn construir_recorde_de_invencibilidade(this: usize) -> usize {
    cpf2_data_padrao(this + 4); // o começo da invencibilidade
    cpf2_data_padrao(this + 0xc);
    this
}

unsafe extern "thiscall" fn comparar_construir_recorde_de_vitorias_seguidas(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || construir_recorde_de_vitorias_seguidas(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_de_vitorias_seguidas {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_construir_recorde_de_derrotas_seguidas(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(%%I1%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || construir_recorde_de_derrotas_seguidas(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_de_derrotas_seguidas {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_construir_recorde_de_invencibilidade(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || construir_recorde_de_invencibilidade(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_de_invencibilidade {:x?} x {:x?}", mo, mr));
    r
}
