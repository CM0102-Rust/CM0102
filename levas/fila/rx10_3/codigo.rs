// ---------- rx10_3: the Football Conference season turnover (eng_conf.cpp) ----------

/// Rule: at the season turn the Football Conference archives the season in its statistics, clears the table, the
/// round dates and the stages, moves the year on by one and rebuilds the rules, the 22-club table, the fixtures and the
/// order; then the predicted positions, the board expectations and the referee renewal.
///
/// The mould of the English divisions' turnover (`eng_second_start_new_season`, checked instruction by instruction:
/// `league::SEASON_CLOSED` = 0, then `rebuild_season` with EngConf::create_table 0x56ecb0 and the eng_conf.cpp boxes at
/// lines 0x259, 0x260, 0x266 and 0x26d, then `set_expectations_and_renew_officials`). After a box the copy returns 0
/// (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `EngConf::start_new_season` (0x56ee80, eng_conf.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_conf_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x56ecb0, 0x9c4294, [0x259, 0x260, 0x266, 0x26d], 1) { // rx10_3: the Conference turnover
        return 0;
    }
    covered!("rx10_3 conference turnover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `eng_conf_start_new_season`: only the original runs (it frees and rebuilds the league); the year must
/// go up by one (`compare_state_league_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_eng_conf_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("eng_conf_start_new_season", League(this), || o(this))
}
