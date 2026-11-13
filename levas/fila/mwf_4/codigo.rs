// ---------- mwf_4: the season turnovers of the Korean and Spanish Super Cups ----------

/// The season turnover of the Korean Super Cup: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; no predicted positions: it returns the eax of the tie tree.
///
/// The cup turnover template (`usa_mls_all_stars_cup_start_new_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (kor_super_cup.cpp, lines 0x134, 0x13b and 0x142) when it returns a negative value: the entrants (KorSuperCup::create_table at 0x66dc10, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `KorSuperCup::start_new_season` (0x66de90, kor_super_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_super_cup_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x66dc10, 0x9eb198, [0x134, 0x13b, 0x142], 1, false, "mwf_4 Korean Super Cup rollover") // mwf4: the turnover of the Korean Super Cup
}

/// Comparer for `kor_super_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_kor_super_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_cup_new_season("kor_super_cup_start_new_season", Cup(this), || o(this))
}

/// The season turnover of the Spanish Super Cup: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; no predicted positions: it returns the eax of the tie tree.
///
/// The cup turnover template (`usa_mls_all_stars_cup_start_new_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (spa_super.cpp, lines 0x125, 0x12c and 0x133) when it returns a negative value: the entrants (SpaSuper::select_teams at 0x858aa0, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `SpaSuper::start_new_season` (0x858cd0, spa_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x858aa0, 0xa82a58, [0x125, 0x12c, 0x133], 1, false, "mwf_4 Spanish Super Cup rollover") // mwf4: the turnover of the Spanish Super Cup
}

/// Comparer for `spa_super_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_spa_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_cup_new_season("spa_super_start_new_season", Cup(this), || o(this))
}
