// ---------- cx09_20: the Finnish First Division south group and the play-off against the Veikkausliiga (fin_first.cpp) ----------

/// Builds the Finnish First Division south group (`create_second_group_stage`): the clubs whose reserve division is
/// the South group ([`KEY_CLUB_COMP_FINNISH_FIRST_DIVISION_SOUTH`]), three legs, with the fin_first.cpp boxes of lines
/// 0x52f, 0x540, 0x54d and 0x558.
/// Original: `FinFirst::create_south_group_stage` (0x594cd0, fin_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_first_create_south_group_stage(this: usize) {
    if create_second_group_stage(League(this), KEY_CLUB_COMP_FINNISH_FIRST_DIVISION_SOUTH, 3, FIN_FIRST_SOURCE, [0x52f, 0x540, 0x54d, 0x558]) {
        covered!("cx09_20 finnish south group");
    }
}

/// The pair of the Finnish play-off against the Veikkausliiga, written into `pair` (two slots): the first entry of the
/// promotion play-off (stage 1, a cup stage) whose `cup_team_entry::STATUS` is 2, among its team count (+0x3e), and the
/// first Veikkausliiga ([`KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION`]) table row with `league_table_row::STATUS` 2. A slot
/// with nobody found is left as it was (the original's fresh malloc, uninitialised: a defect).
unsafe fn fin_first_premier_playoff_pair(l: League, pair: usize) {
    let stage = read_usize(l.stages() + 4);
    let teams = Cup(stage).team_table();
    let count = read_i16(stage + 0x3e) as i32; // the stage's team count
    if let Some(k) = (0..count).find(|&k| read_u8(teams + 6 * k as usize + cup_team_entry::STATUS) == 2) {
        write_usize(pair, read_usize(teams + 6 * k as usize + cup_team_entry::CLUB));
    }
    let premier = League(comp_by_global(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION));
    if let Some(k) = (0..premier.club_count() as i32).find(|&k| LeagueTableRow(premier.table() + k as usize * league_table_row::SIZE).status() == 2) {
        write_usize(pair + 4, LeagueTableRow(premier.table() + k as usize * league_table_row::SIZE).club() as usize);
    }
}

/// Builds the Finnish play-off against the Veikkausliiga (stage 3): a pair (malloc of 8; none: the fin_first.cpp box of
/// line 0x3bb) with the First Division's play-off winner and the Veikkausliiga's play-off club
/// (`fin_first_premier_playoff_pair`); the round table through virtual `comp_vtable::CREATE_SCHEDULE` (3, &rounds,
/// &extra, 0) and a Cup_Stage (`exe::crt_operator_new`(0xb2); 0x522e00: this league, 2 clubs, the pair, the rounds,
/// the competition, the table, the year, stage 3, 1, the extra, 0x14, 0, 0, 0, no seeds) into stage slot 3. Without a
/// stage: the box of line 0x3e0 (the table and the pair are not freed). With one: the table and the pair freed and
/// `league::CURRENT_STAGE` = 3 (no Cup::set_up_round here, unlike the other play-offs).
/// Original: `FinFirst::create_premier_play_off_stage` (0x594610, fin_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_first_create_premier_play_off_stage(this: usize) {
    let l = League(this);
    let pair = exe::crt_malloc(8);
    if pair == 0 {
        return show_error_at(FIN_FIRST_SOURCE, 0x3bb);
    }
    fin_first_premier_playoff_pair(l, pair);
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = schedule(this, 3, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        let construct: BelThirdCupStageCtor = core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, 2, pair, rounds as u16 as u32, l.competition().addr(), table, l.year() as u16 as u32, 3, 1, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(l.stages() + 4 * 3, stage);
    if stage == 0 {
        return show_error_at(FIN_FIRST_SOURCE, 0x3e0);
    }
    covered!("cx09_20 finnish play-off against the veikkausliiga");
    exe::crt_free(table);
    exe::crt_free(pair);
    l.set_current_stage(3);
}

/// Comparer for `fin_first_create_south_group_stage`: only the original (it builds the stage); afterwards stage 0 must
/// exist and be the current one.
pub(crate) unsafe extern "thiscall" fn compare_fin_first_create_south_group_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let l = League(this);
    let s = read_usize(l.stages());
    SCOREBOARD.record(s != 0 && l.current_stage() as i32 == 0, || format!("fin_first_create_south_group_stage({:#x}): stage {:#x}, current {}", this, s, l.current_stage() as i32));
}

/// Comparer for `fin_first_create_premier_play_off_stage`: the copy's pair is worked out before; only the original runs
/// (it builds the stage); afterwards stage 3 must exist, be the current one and hold the copy's pair (its team table's
/// first two clubs, in either order).
pub(crate) unsafe extern "thiscall" fn compare_fin_first_create_premier_play_off_stage(this: usize) {
    let l = League(this);
    let mut pair = [0usize; 2];
    fin_first_premier_playoff_pair(l, pair.as_mut_ptr() as usize);
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let s = read_usize(l.stages() + 4 * 3);
    let got = if s != 0 { [read_usize(Cup(s).team_table()), read_usize(Cup(s).team_table() + 6)] } else { [0, 0] };
    let (mut a, mut b) = (got, pair);
    a.sort();
    b.sort();
    SCOREBOARD.record(s != 0 && l.current_stage() as i32 == 3 && a == b, || format!("fin_first_create_premier_play_off_stage({:#x}): stage {:#x}, current {}, teams {:x?} copy {:x?}", this, s, l.current_stage() as i32, got, pair));
}
