// ---------- cx11_6: the season turnover of the Serie C2 groups A and B (ita_ser_c2a.cpp, ita_ser_c2b.cpp) ----------

/// The season turnover of Serie C2/A: the season archived in the statistics, the table, the round dates and the stages
/// cleared, the year moved on by one and the rules, the table, the fixtures and the order rebuilt; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// Serie C1/A's turnover (`ita_ser_c1a_start_new_season`, checked instruction by instruction: only the table routine
/// and the source file change; the box lines 0x220, 0x227, 0x22d and 0x234 are the same): `league::SEASON_CLOSED` = 0,
/// `rebuild_season` with ItaSerC2a::create_table (0x653a90) and `set_expectations_and_renew_officials`, whose eax it
/// returns. After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `ItaSerC2a::start_new_season` (0x653c40, ita_ser_c2a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2a_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x653a90, 0x9ea654, [0x220, 0x227, 0x22d, 0x234], 1) { // cx11_6: the turnover of Serie C2 A
        return 0;
    }
    covered!("cx11_6 Serie C2 rollover");
    set_expectations_and_renew_officials(this)
}

/// The season turnover of Serie C2/B, as C2/A's with ItaSerC2b::init_team_list (0x657f50) and the ita_ser_c2b.cpp box
/// lines 0x226, 0x22d, 0x233 and 0x23a.
/// Original: `ItaSerC2b::start_new_season` (0x658100, ita_ser_c2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2b_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x657f50, 0x9ea75c, [0x226, 0x22d, 0x233, 0x23a], 1) { // cx11_6: the turnover of Serie C2 B
        return 0;
    }
    set_expectations_and_renew_officials(this)
}

/// Comparer for `ita_ser_c2a_start_new_season`: only the original runs (it frees and rebuilds the league); the year
/// must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2a_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("ita_ser_c2a_start_new_season", League(this), || o(this))
}

/// Comparer for `ita_ser_c2b_start_new_season`: the same.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2b_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("ita_ser_c2b_start_new_season", League(this), || o(this))
}
