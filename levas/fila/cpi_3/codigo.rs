// ---------- %%LEVA%%a leva: os recordes do clube: a construção dos recordes de jogos sem vencer, de gols numa partida e do jogador mais novo ----------

/// { 0x450bc0 } (thiscall, club_records.cpp): `winless_run_record_construct`: constrói o recorde de jogos sem vencer:
/// as datas do começo (+4) e do fim (+0xc) (0x54a110). O quadro SEH do original não é preciso (a 0x54a110 não falha).
/// Devolve `this`.
unsafe extern "thiscall" fn winless_run_record_construct(this: usize) -> usize {
    init_default_date(this + 4);
    init_default_date(this + 0xc); // o fim dos jogos sem vencer
    this
}

/// { 0x450c10 } (thiscall, club_records.cpp): `goals_in_match_record_construct`: constrói o recorde de gols numa
/// partida: a data do recordista (+0x11) e a da partida (+0x19) (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn goals_in_match_record_construct(this: usize) -> usize {
    init_default_date(this + 0x11);
    init_default_date(this + 0x19); // a data da partida
    this
}

/// { 0x450c70 } (thiscall, club_records.cpp): `youngest_player_record_construct`: constrói o recorde do jogador mais
/// novo: a idade (+0), a data do recordista (+0x18) e a da partida (+0x20) (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn youngest_player_record_construct(this: usize) -> usize {
    init_default_date(this); // a idade do mais novo
    init_default_date(this + 0x18);
    init_default_date(this + 0x20);
    this
}

unsafe extern "thiscall" fn compare_winless_run_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x14)], || o(this), || winless_run_record_construct(this));
    SCOREBOARD.record(igual && r == n, || format!("winless_run_record_construct {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn compare_goals_in_match_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I1%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x21)], || o(this), || goals_in_match_record_construct(this));
    SCOREBOARD.record(igual && r == n, || format!("goals_in_match_record_construct {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn compare_youngest_player_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x28)], || o(this), || youngest_player_record_construct(this));
    SCOREBOARD.record(igual && r == n, || format!("youngest_player_record_construct {:x?} x {:x?}", mo, mr));
    r
}
