// ---------- mwf_2: the season turnover of the Korean FA Cup ----------

/// The season turnover of the Korean FA Cup: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; finally the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax it returns.
///
/// The cup turnover template (`start_new_cup_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (kor_fa_cup.cpp, lines 0x185, 0x18c and 0x193) when it returns a negative value: the entrants (KorFaCup::collect_entrants at 0x66aaf0, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `KorFaCup::start_new_season` (0x66acc0, kor_fa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_fa_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x66aaf0, 0x9eb0e8, [0x185, 0x18c, 0x193], 1, "mwf_2 Korean FA Cup rollover") // mwf2: the turnover of the Korean FA Cup
}

/// Comparer for `kor_fa_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_kor_fa_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_cup_new_season("kor_fa_cup_start_new_season", Cup(this), || o(this))
}
