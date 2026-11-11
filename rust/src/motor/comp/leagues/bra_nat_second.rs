use super::*;

/// Series B has the round robin (stage −1) and the play-off (stage 0): when the round robin ends, the stage becomes 0
/// and the play-off is built; past the last stage it is an error (the box)
/// Original: `BraNatSecond::advance_stage` (0x42cdd0, bra_nat_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_nat_second_advance_stage(this: usize) {
    advance_series_stage(League(this), 0x98ab64, 0x23d, 0x42ce60);
}

/// The Series B play-off: the top 4 of the round robin, 1st v 4th and 2nd v 3rd, in a knockout (no seeds) that decides
/// the Series B champion [inference: the promotions to Series A come from it]
/// Original: `BraNatSecond::create_play_off_stage` (0x42ce60, bra_nat_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_nat_second_create_play_off_stage(this: usize) {
    create_play_off_stage(League(this), &[0, 3, 1, 2], 0x98ab64, 0x25f, 0x276, "cqq_7 serie b"); // cqq7: 1º × 4º, 2º × 3º Série B
}

/// The Series B turnover: archives the season in the statistics, clears the table and stages, moves to the next year
/// and rebuilds the rules, the table with the 28 Series B clubs, the fixtures and the order; then the predicted
/// positions, the board expectations and the referee renewal
/// Original: `BraNatSecond::start_new_season` (0x42cc10, bra_nat_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_nat_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed((0) as usize);
    if !rebuild_season(this, 0x42ca70, 0x98ab64, [0x20c, 0x213, 0x219, 0x220], 1) {
        return 0;
    }
    covered!("cqq_8 serie b rollover");
    set_expectations_and_renew_officials(this)
}

/// Series B has 28 clubs, those of the division in database order; no TV money nor prize fund
/// Original: `BraNatSecond::create_table` (0x42ca70, bra_nat_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_nat_second_create_table(this: usize) -> u32 {
    create_division_table(League(this), 0x1c, 0x98ab64, 0x1aa, 0x57, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION, 0, "cql_7 serie b cheia")
}

/// Series B: single round robin; win 3, draw 1; head-to-head and goal difference; promotes to Series A and relegates to
/// Series C; 4 go up (rule word 2: the top places are promotion) and 4 go down; 7 substitutes, 3 substitutions, fixed
/// numbers. Then the schedule
/// Original: `BraNatSecond::set_up_rules` (0x42cb70, bra_nat_second.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn bra_nat_second_set_up_rules(this: usize) -> u32 {
    set_serie_b_rules(this);
    covered!("cqi_7 serie b rules");
    create_main_schedule(League(this));
    1
}
