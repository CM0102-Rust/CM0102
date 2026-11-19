// ---------- batch %%LEVA%%: the Danish and German lower divisions' season turnover (den_first.cpp, den_second.cpp, ger_second.cpp) ----------

/// Rule: the Danish First Division turnover (called by the Superliga's): the season reopened (`league::SEASON_CLOSED`
/// = 0), rebuilt for the next year with the stage fields kept (`rebuild_turkish_league`: the table, the round dates and
/// the statistics; the year + 1; the rules, DenFirst::create_table at 0x552290, the fixtures and the order, each with
/// the box of den_first.cpp at lines 0x1a3, 0x1aa, 0x1b0, 0x1b7), then the predicted positions, the board expectations
/// and the officials (`finish_croatian_turnover`), whose eax is returned. After a box the copy returns 0 (the original
/// returns the box's eax; the caller does not read it).
/// Original: `DenFirst::start_new_season` (0x552450, den_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    covered!("mwc_18 danish first division turnover");
    if !rebuild_turkish_league(this, 0x552290, 0x9b8e30, [0x1a3, 0x1aa, 0x1b0, 0x1b7]) {
        return 0;
    }
    finish_croatian_turnover(this)
}

/// Rule: the Danish Second Division turnover (called by the Superliga's): the season reopened, the stages of the last
/// season destroyed and the league rebuilt for the next year (`rebuild_season`: the table, the round dates, the
/// statistics, the stages 0..=current by their virtual destructor with 1, the year + 1, the current stage −1; the
/// rules, DenSecond::create_table at 0x554a40, the fixtures and the order, each with the box of den_second.cpp at lines
/// 0x1af, 0x1b6, 0x1bc, 0x1c3; the predicted positions), then the board expectations and the officials
/// (`set_expectations_and_renew_officials`), whose eax is returned. After a box: 0.
/// Original: `DenSecond::start_new_season` (0x554c00, den_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x554a40, 0x9b8ea4, [0x1af, 0x1b6, 0x1bc, 0x1c3], 1) {
        return 0;
    }
    covered!("mwc_18 danish second division turnover");
    set_expectations_and_renew_officials(this)
}

/// Rule: the German 2. Bundesliga turnover (called by the Bundesliga's): the same template as the Danish Second
/// Division (`rebuild_season` with GerSecond::create_table at 0x5df2b0 and the boxes of ger_second.cpp at lines 0x20a,
/// 0x211, 0x217, 0x21e; then `set_expectations_and_renew_officials`). After a box: 0.
/// Original: `GerSecond::start_new_season` (0x5df490, ger_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x5df2b0, 0x9cd4c4, [0x20a, 0x211, 0x217, 0x21e], 1) {
        return 0;
    }
    covered!("mwc_18 german second division turnover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `den_first_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_den_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("den_first_start_new_season", League(this), || o(this))
}

/// Comparer for `den_second_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_den_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("den_second_start_new_season", League(this), || o(this))
}

/// Comparer for `ger_second_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_ger_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_state_league_new_season("ger_second_start_new_season", League(this), || o(this))
}
