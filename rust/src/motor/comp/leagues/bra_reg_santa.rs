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

/// The predicted positions of the Santa Catarina state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, 3, 3, and the others their place in the order (`state_league_predicted_positions`
/// with `SANTA_CATARINA_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x307 (no statistics) and 0x30f (no memory).
/// Original: `BraRegSanta::set_predicted_positions` (0x43c110, bra_reg_santa.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_santa_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &SANTA_CATARINA_PREDICTED)
}

/// Comparer for `bra_reg_santa_set_predicted_positions` (`compare_state_league_predicted`, side by side).
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_santa_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_SANTA_SET_PREDICTED_POSITIONS);
    compare_state_league_predicted("bra_reg_santa_set_predicted_positions", this, &SANTA_CATARINA_PREDICTED, || o(this));
}

/// The schedule of the Santa Catarina state league: stage −1, the round-robin league stage: 18 rounds from 11 Jan to 20 Apr; stage 0, the knockout or final stage: 2 rounds from 21 Apr to 10 May; any other stage returns 0.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures and the season turns. For a stage with a table (`santa_catarina_schedule_stage`) it writes the outputs (is_league
/// when that pointer is not null, the rounds and the extra), allocates the round table (`exe::crt_malloc`, 0x41 bytes
/// per round; out of memory, the error box and 0) and writes the date (day, month, years after the season year
/// `year()`, weekday and code) and the matches of each round, in the original's order (`create_round_table`, the
/// cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table, or 0 without writing
/// anything for a stage without one.
/// Original: `BraRegSanta::create_schedule` (0x43b430, bra_reg_santa.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_reg_santa_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), santa_catarina_schedule_stage(stage), rounds, extra, league, 0x98b050)
}

/// Comparer for `bra_reg_santa_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_santa_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_REG_SANTA_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_reg_santa_create_schedule", stage, santa_catarina_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_reg_santa_create_schedule(this, stage, r, e, l))
}

/// The stages of the round table of the Santa Catarina state league (virtual `comp_vtable::CREATE_SCHEDULE`): stage −1, the round-robin league stage: 18 rounds from 11 Jan to 20 Apr; stage 0, the knockout or final stage: 2 rounds from 21 Apr to 10 May; any other stage has no table (None).
pub(crate) fn santa_catarina_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 18, extra: 0, extra_check: false, size: 0x492, row: 0xa8,
            ops: &SANTA_CATARINA_SCHEDULE_LEAGUE_OPS, name: "cqw_3 santa catarina league" }),
        0 => Some(ScheduleStage { league: 0, rounds: 2, extra: 0, extra_check: false, size: 0xd0, row: 0x126,
            ops: &SANTA_CATARINA_SCHEDULE_STAGE_0_OPS, name: "cqw_3 santa catarina stage 0" }),
        _ => None,
    }
}

/// The Santa Catarina state league: the fixed places 1, 2, 3, 3 (the semi-finals).
pub(crate) const SANTA_CATARINA_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98b050, lines: [0x307, 0x30f], places: &[1, 2, 3, 3], label: "cqx_6 santa catarina predicted" }; // cqx6: the santa catarina places

/// The Santa Catarina state league, the round-robin stage (−1): 18 rounds (11 Jan to 20 Apr), 1170 bytes, 64 operations.
pub(crate) const SANTA_CATARINA_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 64] = [
    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 0x12, 1, 0, 6, 2, 0), Cqp11L(1, 0, 5, 1, 1, 0), Cqp11L(1, 1, 5, 1, 1, 0), Cqp11L(1, 2, -1, -1, -1, 0),
    Cqp11D(2, 0x15, 1, 0, 2, 2, 0), Cqp11L(2, 0, -1, -1, -1, 0), Cqp11D(3, 0x19, 1, 0, 6, 1, 0),
    Cqp11L(3, 0, 5, 1, 1, 0), Cqp11L(3, 1, 5, 1, 1, 0), Cqp11L(3, 2, -1, -1, -1, 0), Cqp11D(4, 4, 2, 0, 6, 1, 0),
    Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0), Cqp11D(5, 0xb, 2, 0, 6, 1, 0),
    Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0), Cqp11D(6, 0xe, 2, 0, 2, 2, 0),
    Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0x12, 2, 0, 6, 1, 0), Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0),
    Cqp11L(7, 2, -1, -1, -1, 0), Cqp11D(8, 0x19, 2, 0, 6, 1, 0), Cqp11L(8, 0, 5, 1, 1, 0), Cqp11L(8, 1, 5, 1, 1, 0),
    Cqp11L(8, 2, -1, -1, -1, 0), Cqp11D(9, 1, 3, 0, 6, 1, 0), Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0),
    Cqp11L(9, 2, -1, -1, -1, 0), Cqp11D(0xa, 8, 3, 0, 6, 1, 0), Cqp11L(0xa, 0, 5, 1, 1, 0),
    Cqp11L(0xa, 1, 5, 1, 1, 0), Cqp11L(0xa, 2, -1, -1, -1, 0), Cqp11D(0xb, 0xf, 3, 0, 6, 1, 0),
    Cqp11L(0xb, 0, 5, 1, 1, 0), Cqp11L(0xb, 1, 5, 1, 1, 0), Cqp11L(0xb, 2, -1, -1, -1, 0),
    Cqp11D(0xc, 0x16, 3, 0, 6, 1, 0), Cqp11L(0xc, 0, 5, 1, 1, 0), Cqp11L(0xc, 1, 5, 1, 1, 0),
    Cqp11L(0xc, 2, -1, -1, -1, 0), Cqp11D(0xd, 0x1d, 3, 0, 6, 1, 0), Cqp11L(0xd, 0, 5, 1, 1, 0),
    Cqp11L(0xd, 1, 5, 1, 1, 0), Cqp11L(0xd, 2, -1, -1, -1, 0), Cqp11D(0xe, 6, 4, 0, 6, 1, 0),
    Cqp11L(0xe, 0, 5, 1, 1, 0), Cqp11L(0xe, 1, 5, 1, 1, 0), Cqp11L(0xe, 2, -1, -1, -1, 0),
    Cqp11D(0xf, 0xd, 4, 0, 6, 1, 0), Cqp11L(0xf, 0, 5, 1, 1, 0), Cqp11L(0xf, 1, 5, 1, 1, 0),
    Cqp11L(0xf, 2, -1, -1, -1, 0), Cqp11D(0x10, 0x10, 4, 0, 2, 2, 0), Cqp11L(0x10, 0, -1, -1, -1, 0),
    Cqp11D(0x11, 0x14, 4, 0, 6, 1, 0), Cqp11L(0x11, 0, -1, -1, -1, 0),
];

/// The Santa Catarina state league, stage 0: 2 rounds (21 Apr to 10 May), 208 bytes, 36 operations.
pub(crate) const SANTA_CATARINA_SCHEDULE_STAGE_0_OPS: [AsiaClubChampScheduleOp; 36] = [
    Cqp11S(0, 0x15, 4, 0, 6), Cqp11J(0, 0x1b, 4, 0, 6, 1, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 8), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 0x804),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 4, 5, 0, 0), Cqp11J(1, 0xa, 5, 0, 6, 1, 0), Cqp11W(0x71, 1), Cqp11W(0x82, 1),
    Cqp11W(0x6f, 0x96), Cqp11W(0x73, 0), Cqp11W(0x75, 0), Cqp11B(0x7f, 8), Cqp11W(0x80, 2), Cqp11W(0x84, 0),
    Cqp11W(0x77, 0x804), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0x1e848), Cqp11X(0xcc, 0xc350),
];
