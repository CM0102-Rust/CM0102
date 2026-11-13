// ---------- mwf_6: the season turnovers of the English First, Second and Third Divisions ----------

/// The season turnover of the English First Division: the season archived in the statistics, the table, the round dates and the stages
/// cleared, the year moved on by one and the rules, the table, the fixtures and the order rebuilt; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// The league turnover template of Brazil's Series B (`bra_nat_second_start_new_season`, checked instruction by
/// instruction): `league::SEASON_CLOSED` = 0, then `rebuild_season`: the table freed (League::free_table
/// `exe::league_free_table`, thiscall(this, 0), ret 4); `.round_dates()` freed (`exe::crt_free`) and zeroed; with
/// statistics, Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1),
/// ret 4); with a current stage ≥ 0, the stages 0..=`.current_stage()` destroyed (virtual 0 with 1; the stage array
/// itself is kept); the year up by one and `league::CURRENT_STAGE` = −1. Then, each with its own box (eng_first.cpp, lines 0x29e, 0x2a5,
/// 0x2ab and 0x2b2) when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table (EngFirst::create_table at 0x572bf0, thiscall, ret 0),
/// the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table
/// `exe::league_sort_table`, thiscall(this, 0), ret 4); then the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`), the board expectations (League::set_club_season_expectations) and the
/// referee renewal (OfficialsManager::renew_officials `exe::officialsmanager_renew_officials`,
/// thiscall([`G_OFFICIALS_MANAGER`], `.competition()`), ret 4), whose eax it returns. After a box the copy returns 0
/// (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `EngFirst::start_new_season` (0x572e30, eng_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x572bf0, 0x9c4374, [0x29e, 0x2a5, 0x2ab, 0x2b2], 1) { // mwf6: the turnover of the English First Division
        return 0;
    }
    covered!("mwf_6 English First Division rollover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `eng_first_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_eng_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("eng_first_start_new_season", League(this), || o(this))
}

/// The season turnover of the English Second Division: the season archived in the statistics, the table, the round dates and the stages
/// cleared, the year moved on by one and the rules, the table, the fixtures and the order rebuilt; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// The league turnover template of Brazil's Series B (`bra_nat_second_start_new_season`, checked instruction by
/// instruction): `league::SEASON_CLOSED` = 0, then `rebuild_season`: the table freed (League::free_table
/// `exe::league_free_table`, thiscall(this, 0), ret 4); `.round_dates()` freed (`exe::crt_free`) and zeroed; with
/// statistics, Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1),
/// ret 4); with a current stage ≥ 0, the stages 0..=`.current_stage()` destroyed (virtual 0 with 1; the stage array
/// itself is kept); the year up by one and `league::CURRENT_STAGE` = −1. Then, each with its own box (eng_second.cpp, lines 0x264, 0x26b,
/// 0x271 and 0x278) when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table (EngSecond::create_table at 0x576630, thiscall, ret 0),
/// the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table
/// `exe::league_sort_table`, thiscall(this, 0), ret 4); then the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`), the board expectations (League::set_club_season_expectations) and the
/// referee renewal (OfficialsManager::renew_officials `exe::officialsmanager_renew_officials`,
/// thiscall([`G_OFFICIALS_MANAGER`], `.competition()`), ret 4), whose eax it returns. After a box the copy returns 0
/// (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `EngSecond::start_new_season` (0x576820, eng_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x576630, 0x9c43e8, [0x264, 0x26b, 0x271, 0x278], 1) { // mwf6: the turnover of the English Second Division
        return 0;
    }
    covered!("mwf_6 English Second Division rollover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `eng_second_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_eng_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("eng_second_start_new_season", League(this), || o(this))
}

/// The season turnover of the English Third Division: the season archived in the statistics, the table, the round dates and the stages
/// cleared, the year moved on by one and the rules, the table, the fixtures and the order rebuilt; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// The league turnover template of Brazil's Series B (`bra_nat_second_start_new_season`, checked instruction by
/// instruction): `league::SEASON_CLOSED` = 0, then `rebuild_season`: the table freed (League::free_table
/// `exe::league_free_table`, thiscall(this, 0), ret 4); `.round_dates()` freed (`exe::crt_free`) and zeroed; with
/// statistics, Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1),
/// ret 4); with a current stage ≥ 0, the stages 0..=`.current_stage()` destroyed (virtual 0 with 1; the stage array
/// itself is kept); the year up by one and `league::CURRENT_STAGE` = −1. Then, each with its own box (eng_third.cpp, lines 0x26c, 0x273,
/// 0x279 and 0x280) when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table (EngThird::create_table at 0x577f70, thiscall, ret 0),
/// the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table
/// `exe::league_sort_table`, thiscall(this, 0), ret 4); then the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`), the board expectations (League::set_club_season_expectations) and the
/// referee renewal (OfficialsManager::renew_officials `exe::officialsmanager_renew_officials`,
/// thiscall([`G_OFFICIALS_MANAGER`], `.competition()`), ret 4), whose eax it returns. After a box the copy returns 0
/// (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `EngThird::start_new_season` (0x578170, eng_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_third_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x577f70, 0x9c4424, [0x26c, 0x273, 0x279, 0x280], 1) { // mwf6: the turnover of the English Third Division
        return 0;
    }
    covered!("mwf_6 English Third Division rollover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `eng_third_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_eng_third_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_state_league_new_season("eng_third_start_new_season", League(this), || o(this))
}
