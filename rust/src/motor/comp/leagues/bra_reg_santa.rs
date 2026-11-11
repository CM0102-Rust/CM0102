use super::*;

/// Bahia and Santa Catarina (the same routine in the two vtables): double round robin; 4 top places (the 4-club
/// semi-final); 1 relegated; win 3, draw 1; tie-break by head-to-head, then goal difference; no play-offs in the table;
/// no league above or below (the relegated go to the non-playable pool [inference]); rule word 8 (the top places lead
/// to the next stage; no champion band: the champion comes from the final stage); 7 bench substitutes and 3
/// substitutions; category 2 (league) matches. Then the main stage schedule
/// Original: `BraRegSanta::set_up_rules` (0x43bc70, bra_reg_santa.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn bra_reg_santa_set_up_rules(this: usize) -> u32 {
    catarinense_rules(this);
    covered!("cqi_8 catarinense rules");
    create_main_schedule(League(this));
    1
}

/// The Santa Catarina turnover: archives the season in the statistics, clears the previous table and stages, moves to
/// the next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table
/// order; then the predicted positions and the renewal of the state referees
/// Original: `BraRegSanta::start_new_season` (0x43bd10, bra_reg_santa.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_santa_start_new_season(this: usize) -> u32 {
    state_league_new_season(League(this), 0x43bb70, 0x98b050, [0x1ca, 0x1d1, 0x1d7, 0x1de], 1, "cqp_9 virada do catarinense") // cqp9: the next year of the Catarinense
}

/// The Santa Catarina next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to
/// the semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box)
/// Original: `BraRegSanta::advance_stage` (0x43bec0, bra_reg_santa.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_santa_advance_stage(this: usize) {
    state_league_advance_stage(League(this), 0x98b050, 0x1f8, 0x43bf50);
}

/// Santa Catarina: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is
/// the seed; the ties follow the seeds [inference: two legs, the decider at the better placed club])
/// Original: `BraRegSanta::create_semi_final_stage` (0x43bf50, bra_reg_santa.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_santa_create_semi_final_stage(this: usize) {
    create_knockout_from_table(League(this), &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98b050, 0x21b, 0x22f, "cqn_8 catarinense"); // cqn8: 1º × 4º, 2º × 3º Catarinense
}

/// The Santa Catarina state league at career creation: the year and the database competition, disciplinary rule 7 and,
/// with no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the
/// table with the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegSanta::BraRegSanta` (0x43b130, bra_reg_santa.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_santa(this: usize, year: u32, record: usize) -> usize {
    bra_reg_rio_construct_state_league(League(this), year, record, &CATARINENSE_STATE_CHAMPIONSHIP, 7, true, "cqt_4 catarinense novo") // cqt4: the discipline and expected positions of the Catarinense
}

/// the Santa Catarina league has 10 clubs: those with it as their reserve division in the database (the field that
/// holds the state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegSanta::create_table` (0x43bb70, bra_reg_santa.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_santa_create_table(this: usize) -> u32 {
    fill_state_league_table(League(this), 0xa, 0x98b050, 0x167, KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS, 0, "cqm_8 catarinense cheio")
}
