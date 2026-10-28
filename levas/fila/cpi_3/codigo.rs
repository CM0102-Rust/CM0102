// ---------- %%LEVA%%a leva: os recordes do clube: a construção dos recordes de jogos sem vencer, de gols numa partida e do jogador mais novo ----------

/// { 0x450bc0 } (thiscall, club_records.cpp): `winless_run_record_construct`: constrói o recorde de jogos sem vencer:
/// as datas do começo (+4) e do fim (+0xc) (0x54a110). O quadro SEH do original não é preciso (a 0x54a110 não falha).
/// Devolve `this`.
unsafe extern "thiscall" fn construir_recorde_sem_vencer(this: usize) -> usize {
    cpf2_data_padrao(this + 4);
    cpf2_data_padrao(this + 0xc); // o fim dos jogos sem vencer
    this
}

/// { 0x450c10 } (thiscall, club_records.cpp): `goals_in_match_record_construct`: constrói o recorde de gols numa
/// partida: a data do recordista (+0x11) e a da partida (+0x19) (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn construir_recorde_de_gols_na_partida(this: usize) -> usize {
    cpf2_data_padrao(this + 0x11);
    cpf2_data_padrao(this + 0x19); // a data da partida
    this
}

/// { 0x450c70 } (thiscall, club_records.cpp): `youngest_player_record_construct`: constrói o recorde do jogador mais
/// novo: a idade (+0), a data do recordista (+0x18) e a da partida (+0x20) (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn construir_recorde_do_mais_novo(this: usize) -> usize {
    cpf2_data_padrao(this); // a idade do mais novo
    cpf2_data_padrao(this + 0x18);
    cpf2_data_padrao(this + 0x20);
    this
}

unsafe extern "thiscall" fn comparar_construir_recorde_sem_vencer(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || construir_recorde_sem_vencer(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_sem_vencer {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_construir_recorde_de_gols_na_partida(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(%%I1%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x21)], || o(this), || construir_recorde_de_gols_na_partida(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_de_gols_na_partida {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_construir_recorde_do_mais_novo(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x28)], || o(this), || construir_recorde_do_mais_novo(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_do_mais_novo {:x?} x {:x?}", mo, mr));
    r
}
