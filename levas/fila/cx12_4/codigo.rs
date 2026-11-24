// ---------- cx12_4: the Serie C2/C schedule and season turnover (ita_ser_c2c.cpp) ----------

/// The Serie C2/C schedule, the same as C2/A's (`create_c1_schedule`, checked instruction by instruction: only the
/// source file changes; the box lines are C1/A's). By stage: −1, the league (34 rounds from 5 September to 7 May, extra
/// 0); 0, the promotion play-off (2 two-legged rounds, the semi-finals on 21 and 28 May and the final on 4 and 11 June,
/// extra 0xa0); 1, the relegation play-out (1 two-legged round on the semi-final dates, extra 0); any other stage
/// returns 0 and writes nothing. Out of memory: the ita_ser_c2c.cpp box of the stage's line (0xab, 0x155, 0x18f) and 0.
/// Original: `ItaSerC2c::create_schedule` (0x65b7f0, ita_ser_c2c.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2c_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    covered!("cx12_4 serie c2 c schedule");
    create_c1_schedule(League(this), stage, rounds, extra, league, &C1BracketVariant { file: 0x9ea83c, rows: [0xab, 0x155, 0x18f] }) // cx12_4: the C2/C schedule
}

/// The season turnover of Serie C2/C, as C2/A's (`ita_ser_c2a_start_new_season`, checked instruction by instruction:
/// only the table routine ItaSerC2c::init_team_list (0x65c0e0) and the source file change; the box lines 0x220, 0x227,
/// 0x22d and 0x234 are the same): `league::SEASON_CLOSED` = 0, `rebuild_season` and
/// `set_expectations_and_renew_officials`, whose eax it returns. After a box the copy returns 0 (the original returns the
/// box's eax; the turnover's caller does not read it).
/// Original: `ItaSerC2c::start_new_season` (0x65c290, ita_ser_c2c.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2c_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x65c0e0, 0x9ea83c, [0x220, 0x227, 0x22d, 0x234], 1) { // cx12_4: the turnover of Serie C2 C
        return 0;
    }
    covered!("cx12_4 serie c2 c rollover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `ita_ser_c2c_create_schedule`: side by side over the outputs and the written table (`compare_c1_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2c_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_c1_schedule("ita_ser_c2c_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| ita_ser_c2c_create_schedule(this, stage, r, e, l))
}

/// Comparer for `ita_ser_c2c_start_new_season`: only the original runs (it frees and rebuilds the league); the year
/// must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2c_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("ita_ser_c2c_start_new_season", League(this), || o(this))
}
