// ---------- mwf_1: the season turnovers of the Charity Shield, the Coupe de France and the French League Cup ----------

/// The season turnover of the Charity Shield: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; finally the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax it returns.
///
/// The cup turnover template (`start_new_cup_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (eng_charity.cpp, lines 0x128, 0x12f and 0x136) when it returns a negative value: the entrants (EngCharity::select_teams at 0x56d780, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `EngCharity::start_new_season` (0x56d9b0, eng_charity.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_charity_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x56d780, 0x9c425c, [0x128, 0x12f, 0x136], 1, "mwf_1 Charity Shield rollover") // mwf1: the turnover of the Charity Shield
}

/// Comparer for `eng_charity_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_eng_charity_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_cup_new_season("eng_charity_start_new_season", Cup(this), || o(this))
}

/// The season turnover of the Coupe de France: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; finally the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax it returns.
///
/// The cup turnover template (`start_new_cup_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (fra_cup.cpp, lines 0x23a, 0x241 and 0x248) when it returns a negative value: the entrants (FraCup::collect_entrants at 0x5baa50, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `FraCup::start_new_season` (0x5bae50, fra_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fra_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x5baa50, 0x9ccb28, [0x23a, 0x241, 0x248], 1, "mwf_1 Coupe de France rollover") // mwf1: the turnover of the Coupe de France
}

/// Comparer for `fra_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_fra_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_cup_new_season("fra_cup_start_new_season", Cup(this), || o(this))
}

/// The season turnover of the French League Cup: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; finally the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax it returns.
///
/// The cup turnover template (`start_new_cup_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (fra_lge_cup.cpp, lines 0x1f9, 0x200 and 0x207) when it returns a negative value: the entrants (FraLgeCup::collect_entrants at 0x5bd160, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `FraLgeCup::start_new_season` (0x5bd420, fra_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fra_lge_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x5bd160, 0x9ccb98, [0x1f9, 0x200, 0x207], 1, "mwf_1 French League Cup rollover") // mwf1: the turnover of the French League Cup
}

/// Comparer for `fra_lge_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_fra_lge_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_cup_new_season("fra_lge_cup_start_new_season", Cup(this), || o(this))
}
