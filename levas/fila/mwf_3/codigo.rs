// ---------- mwf_3: the season turnovers of the French, Serie C1 and Italian Super Cups ----------

/// The season turnover of the French Super Cup: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; no predicted positions: it returns the eax of the tie tree.
///
/// The cup turnover template (`usa_mls_all_stars_cup_start_new_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (fra_super.cpp, lines 0x127, 0x12e and 0x135) when it returns a negative value: the entrants (FraSuper::select_teams at 0x5bf770, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `FraSuper::start_new_season` (0x5bf9a0, fra_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fra_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x5bf770, 0x9ccc48, [0x127, 0x12e, 0x135], 1, false, "mwf_3 French Super Cup rollover") // mwf3: the turnover of the French Super Cup
}

/// Comparer for `fra_super_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_fra_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_cup_new_season("fra_super_start_new_season", Cup(this), || o(this))
}

/// The season turnover of the Serie C1 Super Cup: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; no predicted positions: it returns the eax of the tie tree.
///
/// The cup turnover template (`usa_mls_all_stars_cup_start_new_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (ita_c1_super.cpp, lines 0x10a, 0x111 and 0x118) when it returns a negative value: the entrants (ItaC1Super::init_team_list at 0x63c9d0, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `ItaC1Super::start_new_season` (0x63caa0, ita_c1_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_c1_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x63c9d0, 0x9ea1c8, [0x10a, 0x111, 0x118], 1, false, "mwf_3 Serie C1 Super Cup rollover") // mwf3: the turnover of the Serie C1 Super Cup
}

/// Comparer for `ita_c1_super_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_ita_c1_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_cup_new_season("ita_c1_super_start_new_season", Cup(this), || o(this))
}

/// The season turnover of the Italian Super Cup: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; no predicted positions: it returns the eax of the tie tree.
///
/// The cup turnover template (`usa_mls_all_stars_cup_start_new_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (ita_super.cpp, lines 0x125, 0x12c and 0x133) when it returns a negative value: the entrants (ItaSuper::init_team_list at 0x65fa60, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `ItaSuper::start_new_season` (0x65fc90, ita_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x65fa60, 0x9ea934, [0x125, 0x12c, 0x133], 1, false, "mwf_3 Italian Super Cup rollover") // mwf3: the turnover of the Italian Super Cup
}

/// Comparer for `ita_super_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_ita_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_cup_new_season("ita_super_start_new_season", Cup(this), || o(this))
}
