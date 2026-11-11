use super::*;

/// The Sao Paulo turnover: archives the season in the statistics, clears the previous table and stages, moves to the
/// next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order;
/// then the predicted positions and the renewal of the state referees
/// Original: `BraRegSp::start_new_season` (0x43cd40, bra_reg_sp.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_sp_start_new_season(this: usize) -> u32 {
    state_league_new_season(League(this), 0x43cc40, 0x98b090, [0x1bc, 0x1c3, 0x1c9, 0x1d0], 1, "cqp_9 virada do paulista") // cqp9: the next year of the Paulista
}

/// The Sao Paulo next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the
/// semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box)
/// Original: `BraRegSp::advance_stage` (0x43cef0, bra_reg_sp.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_sp_advance_stage(this: usize) {
    state_league_advance_stage(League(this), 0x98b090, 0x1ea, 0x43cf80);
}

/// Sao Paulo: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the
/// seed; the ties follow the seeds [inference: two legs, the decider at the better placed club])
/// Original: `BraRegSp::create_semi_final_stage` (0x43cf80, bra_reg_sp.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_sp_create_semi_final_stage(this: usize) {
    create_knockout_from_table(League(this), &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98b090, 0x20d, 0x221, "cqn_8 paulista"); // cqn8: 1º × 4º, 2º × 3º Paulista
}

/// The Sao Paulo state league at career creation: the year and the database competition, disciplinary rule 7 and, with
/// no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table
/// with the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegSp::BraRegSp` (0x43c280, bra_reg_sp.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_sp(this: usize, year: u32, record: usize) -> usize {
    bra_reg_rio_construct_state_league(League(this), year, record, &PAULISTA_STATE_LEAGUE, 7, true, "cqt_4 paulista novo") // cqt4: the discipline and expected positions of the Paulista
}

/// the Sao Paulo league has 16 clubs: those with it as their reserve division in the database (the field that holds the
/// state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegSp::create_table` (0x43cc40, bra_reg_sp.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_sp_create_table(this: usize) -> u32 {
    fill_state_league_table(League(this), 0x10, 0x98b090, 0x159, KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP, 0, "cqm_8 paulista cheio")
}
