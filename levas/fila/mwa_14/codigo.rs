// ---------- batch %%LEVA%%: the Irish First Division: the season turnover, the next stage and the promotion play-off (ire_first.cpp) ----------

/// Rule: the Irish First Division turnover: the statistics archived, the table and the round table cleared, the year
/// moved on and the rules, the table, the fixtures and the order rebuilt; then the predicted positions, the board
/// expectations and the referee renewal.
///
/// BraNatSecond::start_new_season's template (checked instruction by instruction: only the table routine,
/// IreFirst::create_table 0x635910, the file and the error box lines 0x217, 0x21e, 0x224 and 0x22b change).
/// Original: `IreFirst::start_new_season` (0x635b30, ire_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x635910, 0x9e9ed0, [0x217, 0x21e, 0x224, 0x22b], 1) { // mwa14: the next season of the Irish First
        return 0;
    }
    covered!("mwa_14 irish first turnover");
    set_expectations_and_renew_officials(this)
}

unsafe extern "thiscall" fn compare_ire_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("ire_first_start_new_season", League(this), || o(this))
}

/// Rule: the Irish First Division's league stage ends only when the Premier Division above has finished: then the
/// Premier's positions are updated and the promotion play-off is built; past the last stage, the error box.
///
/// `advance_irish_first_stage` with the Irish Premier Division (`KEY_CLUB_COMP_IRISH_PREMIER_DIVISION`), the error box at
/// line 0x248 of ire_first.cpp and IreFirst::create_promotion_playoff_stage (0x635dc0) (checked instruction by
/// instruction against NirFirst::advance_stage: only the key, the file and the line change).
/// Original: `IreFirst::advance_stage` (0x635cf0, ire_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_first_advance_stage(this: usize) {
    advance_irish_first_stage(League(this), KEY_CLUB_COMP_IRISH_PREMIER_DIVISION, 0x9e9ed0, 0x248, 0x635dc0) // mwa14: the Irish play-off waits for the Premier
}

unsafe extern "thiscall" fn compare_ire_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_irish_first_advance_stage("ire_first_advance_stage", League(this), || o(this))
}

/// Rule: the Irish promotion play-off: the First Division runner-up against the Premier Division's second last (the
/// clubs with the play-off status in each table), in a two-club stage; without one of them, no play-off.
///
/// `create_irish_promotion_play_off` with the Irish Premier Division and the error box lines 0x274, 0x284 and 0x2a5 of
/// ire_first.cpp (checked instruction by instruction against NirFirst::create_playoff_stage).
/// Original: `IreFirst::create_promotion_playoff_stage` (0x635dc0, ire_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_first_create_promotion_playoff_stage(this: usize) {
    create_irish_promotion_play_off(League(this), KEY_CLUB_COMP_IRISH_PREMIER_DIVISION, 0x9e9ed0, [0x274, 0x284, 0x2a5], "mwa_14 irish promotion play-off")
}

unsafe extern "thiscall" fn compare_ire_first_create_promotion_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    compare_irish_promotion_play_off("ire_first_create_promotion_playoff_stage", League(this), KEY_CLUB_COMP_IRISH_PREMIER_DIVISION, || o(this))
}
