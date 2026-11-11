use super::*;

/// North: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the seed;
/// the ties follow the seeds [inference: two legs, the decider at the better placed club])
/// Original: `BraRegNorth::create_semi_final_stage` (0x4360c0, bra_reg_north.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_north_create_semi_final_stage(this: usize) {
    create_knockout_from_table(League(this), &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98af14, 0x21b, 0x22f, "cqn_7 norte"); // cqn7: 1º × 4º, 2º × 3º Norte
}

/// The North state league at career creation: the year and the database competition, disciplinary rule 7 and, with no
/// saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table with
/// the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegNorth::BraRegNorth` (0x435340, bra_reg_north.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_north(this: usize, year: u32, record: usize) -> usize {
    bra_reg_goias_construct_state_league(League(this), year, record, &NORTH_CHAMPIONSHIP, 7, true, "cqt_2 norte novo") // cqt2: the discipline and expected positions of the Norte
}

/// The North turnover: archives the season in the statistics, clears the previous table and stages, moves to the next
/// year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order; then
/// the predicted positions and the renewal of the state referees
/// Original: `BraRegNorth::start_new_season` (0x435e80, bra_reg_north.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_north_start_new_season(this: usize) -> u32 {
    state_league_new_season(League(this), 0x435d80, 0x98af14, [0x1ca, 0x1d1, 0x1d7, 0x1de], 1, "cqp_8 virada do norte") // cqp8: the next year of the Norte
}

/// The North next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the
/// semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box)
/// Original: `BraRegNorth::advance_stage` (0x436030, bra_reg_north.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_north_advance_stage(this: usize) {
    state_league_advance_stage(League(this), 0x98af14, 0x1f8, 0x4360c0);
}

/// the North league has 10 clubs: those with it as their reserve division in the database (the field that holds the
/// state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegNorth::create_table` (0x435d80, bra_reg_north.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_north_create_table(this: usize) -> u32 {
    fill_state_league_table(League(this), 0xa, 0x98af14, 0x167, KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, 0, "cql_9 norte cheio")
}
