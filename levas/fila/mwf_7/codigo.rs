// ---------- mwf_7: the season turnovers of the French Second Division, the French National and Serie C1 A ----------

/// The season turnover of the French Second Division: the season archived in the statistics, the table, the round dates and the stages
/// cleared, the year moved on by one and the rules, the table, the fixtures and the order rebuilt; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// The league turnover template of Brazil's Series B (`bra_nat_second_start_new_season`, checked instruction by
/// instruction): `league::SEASON_CLOSED` = 0, then `rebuild_season`: the table freed (League::free_table
/// `exe::league_free_table`, thiscall(this, 0), ret 4); `.round_dates()` freed (`exe::crt_free`) and zeroed; with
/// statistics, Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1),
/// ret 4); with a current stage ≥ 0, the stages 0..=`.current_stage()` destroyed (virtual 0 with 1; the stage array
/// itself is kept); the year up by one and `league::CURRENT_STAGE` = −1. Then, each with its own box (fra_second.cpp, lines 0x1e4, 0x1ec,
/// 0x1f2 and 0x1f9) when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table (FraSecond::create_table at 0x5bef10, thiscall, ret 0),
/// the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table
/// `exe::league_sort_table`, thiscall(this, 0), ret 4); then the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`), the board expectations (League::set_club_season_expectations) and the
/// referee renewal (OfficialsManager::renew_officials `exe::officialsmanager_renew_officials`,
/// thiscall([`G_OFFICIALS_MANAGER`], `.competition()`), ret 4), whose eax it returns. After a box the copy returns 0
/// (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `FraSecond::start_new_season` (0x5bf160, fra_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fra_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x5bef10, 0x9ccc0c, [0x1e4, 0x1ec, 0x1f2, 0x1f9], 1) { // mwf7: the turnover of the French Second Division
        return 0;
    }
    covered!("mwf_7 French Second Division rollover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `fra_second_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_fra_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("fra_second_start_new_season", League(this), || o(this))
}

/// The season turnover of the French National: the season archived in the statistics, the table, the round dates and the stages
/// cleared, the year moved on by one and the rules, the table, the fixtures and the order rebuilt; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// The league turnover template of Brazil's Series B (`bra_nat_second_start_new_season`, checked instruction by
/// instruction): `league::SEASON_CLOSED` = 0, then `rebuild_season`: the table freed (League::free_table
/// `exe::league_free_table`, thiscall(this, 0), ret 4); `.round_dates()` freed (`exe::crt_free`) and zeroed; with
/// statistics, Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1),
/// ret 4); with a current stage ≥ 0, the stages 0..=`.current_stage()` destroyed (virtual 0 with 1; the stage array
/// itself is kept); the year up by one and `league::CURRENT_STAGE` = −1. Then, each with its own box (fra_third.cpp, lines 0x1ce, 0x1d5,
/// 0x1db and 0x1e2) when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table (FraThird::create_table at 0x5c05d0, thiscall, ret 0),
/// the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table
/// `exe::league_sort_table`, thiscall(this, 0), ret 4); then the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`), the board expectations (League::set_club_season_expectations) and the
/// referee renewal (OfficialsManager::renew_officials `exe::officialsmanager_renew_officials`,
/// thiscall([`G_OFFICIALS_MANAGER`], `.competition()`), ret 4), whose eax it returns. After a box the copy returns 0
/// (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `FraThird::start_new_season` (0x5c07c0, fra_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fra_third_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x5c05d0, 0x9ccc80, [0x1ce, 0x1d5, 0x1db, 0x1e2], 1) { // mwf7: the turnover of the French National
        return 0;
    }
    covered!("mwf_7 French National rollover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `fra_third_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_fra_third_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("fra_third_start_new_season", League(this), || o(this))
}

/// The season turnover of Serie C1 A: the season archived in the statistics, the table, the round dates and the stages
/// cleared, the year moved on by one and the rules, the table, the fixtures and the order rebuilt; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// The league turnover template of Brazil's Series B (`bra_nat_second_start_new_season`, checked instruction by
/// instruction): `league::SEASON_CLOSED` = 0, then `rebuild_season`: the table freed (League::free_table
/// `exe::league_free_table`, thiscall(this, 0), ret 4); `.round_dates()` freed (`exe::crt_free`) and zeroed; with
/// statistics, Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1),
/// ret 4); with a current stage ≥ 0, the stages 0..=`.current_stage()` destroyed (virtual 0 with 1; the stage array
/// itself is kept); the year up by one and `league::CURRENT_STAGE` = −1. Then, each with its own box (ita_ser_c1a.cpp, lines 0x220, 0x227,
/// 0x22d and 0x234) when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table (ItaSerC1a::create_table at 0x64b670, thiscall, ret 0),
/// the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table
/// `exe::league_sort_table`, thiscall(this, 0), ret 4); then the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`), the board expectations (League::set_club_season_expectations) and the
/// referee renewal (OfficialsManager::renew_officials `exe::officialsmanager_renew_officials`,
/// thiscall([`G_OFFICIALS_MANAGER`], `.competition()`), ret 4), whose eax it returns. After a box the copy returns 0
/// (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `ItaSerC1a::start_new_season` (0x64b820, ita_ser_c1a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c1a_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x64b670, 0x9ea474, [0x220, 0x227, 0x22d, 0x234], 1) { // mwf7: the turnover of Serie C1 A
        return 0;
    }
    covered!("mwf_7 Serie C1 A rollover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `ita_ser_c1a_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c1a_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_state_league_new_season("ita_ser_c1a_start_new_season", League(this), || o(this))
}
