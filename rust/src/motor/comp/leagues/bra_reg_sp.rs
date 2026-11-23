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

/// The predicted positions of the São Paulo state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, 3, 3, and the others their place in the order (`state_league_predicted_positions`
/// with `SAO_PAULO_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x2f9 (no statistics) and 0x301 (no memory).
/// Original: `BraRegSp::set_predicted_positions` (0x43d140, bra_reg_sp.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_sp_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &SAO_PAULO_PREDICTED)
}

/// Comparer for `bra_reg_sp_set_predicted_positions` (`compare_state_league_predicted`, side by side).
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_sp_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_SP_SET_PREDICTED_POSITIONS);
    compare_state_league_predicted("bra_reg_sp_set_predicted_positions", this, &SAO_PAULO_PREDICTED, || o(this));
}

/// Rule: the São Paulo state league gives the league points for a result (the win and draw points of its rules),
/// passing the query to the League base.
///
/// `BraRegSp::points_for_result(club, fixture)` (the virtual of the points query): League::points_for_result
/// (0x68aad0, thiscall(this, club, fixture), ret 8; this goes through in ecx untouched), whose return it passes on.
/// Original: `BraRegSp::points_for_result` (0x43c4a0, bra_reg_sp.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn bra_reg_sp_points_for_result(this: usize, club: usize, fixture: usize) -> u32 {
    covered!("cqx_10 paulista points");
    exe::league_points_for_result(this, club, fixture) as u32 // cqx10: the League's points
}

/// Comparer for `bra_reg_sp_points_for_result`: side by side (the query only reads); the returned byte (the caller
/// reads al) must be equal.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_sp_points_for_result(this: usize, club: usize, fixture: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_BRA_REG_SP_POINTS_FOR_RESULT)(this, club, fixture);
    let c = bra_reg_sp_points_for_result(this, club, fixture);
    SCOREBOARD.record(r as u8 == c as u8, || format!("bra_reg_sp_points_for_result({:#x}, {:#x}) original {} copy {}", club, fixture, r as u8, c as u8));
    r
}

/// The schedule of the São Paulo state league (Paulista): stage −1, the round-robin league stage: 15 rounds from 18 Jan to 20 Apr; stage 0, the knockout or final stage: 2 rounds from 21 Apr to 10 May; any other stage returns 0.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures and the season turns. For a stage with a table (`sao_paulo_schedule_stage`) it writes the outputs (is_league
/// when that pointer is not null, the rounds and the extra), allocates the round table (`exe::crt_malloc`, 0x41 bytes
/// per round; out of memory, the error box and 0) and writes the date (day, month, years after the season year
/// `year()`, weekday and code) and the matches of each round, in the original's order (`create_round_table`, the
/// cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table, or 0 without writing
/// anything for a stage without one.
/// Original: `BraRegSp::create_schedule` (0x43c5a0, bra_reg_sp.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_reg_sp_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), sao_paulo_schedule_stage(stage), rounds, extra, league, 0x98b090)
}

/// Comparer for `bra_reg_sp_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_sp_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_REG_SP_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_reg_sp_create_schedule", stage, sao_paulo_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_reg_sp_create_schedule(this, stage, r, e, l))
}

/// The stages of the round table of the São Paulo state league (Paulista) (virtual `comp_vtable::CREATE_SCHEDULE`): stage −1, the round-robin league stage: 15 rounds from 18 Jan to 20 Apr; stage 0, the knockout or final stage: 2 rounds from 21 Apr to 10 May; any other stage has no table (None).
pub(crate) fn sao_paulo_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 15, extra: 0, extra_check: false, size: 0x3cf, row: 0xa8,
            ops: &SAO_PAULO_SCHEDULE_LEAGUE_OPS, name: "cqw_3 sao paulo league" }),
        0 => Some(ScheduleStage { league: 0, rounds: 2, extra: 0, extra_check: false, size: 0xd0, row: 0x118,
            ops: &SAO_PAULO_SCHEDULE_STAGE_0_OPS, name: "cqw_3 sao paulo stage 0" }),
        _ => None,
    }
}

/// The São Paulo state league: the fixed places 1, 2, 3, 3 (the semi-finals).
pub(crate) const SAO_PAULO_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98b090, lines: [0x2f9, 0x301], places: &[1, 2, 3, 3], label: "cqx_6 são paulo predicted" }; // cqx6: the são paulo places

/// The São Paulo state league (Paulista), the round-robin stage (−1): 15 rounds (18 Jan to 20 Apr), 975 bytes, 56 operations.
pub(crate) const SAO_PAULO_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 56] = [
    Cqp11D(0, 0x12, 1, 0, 6, 2, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 0x19, 1, 0, 6, 1, 0), Cqp11L(1, 0, 5, 1, 1, 0), Cqp11L(1, 1, 5, 1, 1, 0), Cqp11L(1, 2, -1, -1, -1, 0),
    Cqp11D(2, 4, 2, 0, 6, 1, 0), Cqp11L(2, 0, 5, 1, 1, 0), Cqp11L(2, 1, 5, 1, 1, 0), Cqp11L(2, 2, -1, -1, -1, 0),
    Cqp11D(3, 0xb, 2, 0, 6, 1, 0), Cqp11L(3, 0, 5, 1, 1, 0), Cqp11L(3, 1, 5, 1, 1, 0), Cqp11L(3, 2, -1, -1, -1, 0),
    Cqp11D(4, 0x12, 2, 0, 6, 1, 0), Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0),
    Cqp11D(5, 0x19, 2, 0, 6, 1, 0), Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0),
    Cqp11D(6, 1, 3, 0, 6, 1, 0), Cqp11L(6, 0, 5, 1, 1, 0), Cqp11L(6, 1, 5, 1, 1, 0), Cqp11L(6, 2, -1, -1, -1, 0),
    Cqp11D(7, 8, 3, 0, 6, 1, 0), Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0), Cqp11L(7, 2, -1, -1, -1, 0),
    Cqp11D(8, 0xf, 3, 0, 6, 1, 0), Cqp11L(8, 0, 5, 1, 1, 0), Cqp11L(8, 1, 5, 1, 1, 0), Cqp11L(8, 2, -1, -1, -1, 0),
    Cqp11D(9, 0x16, 3, 0, 6, 1, 0), Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0), Cqp11L(9, 2, -1, -1, -1, 0),
    Cqp11D(0xa, 0x19, 3, 0, 6, 1, 0), Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 0x1d, 3, 0, 6, 1, 0),
    Cqp11L(0xb, 0, 5, 1, 1, 0), Cqp11L(0xb, 1, 5, 1, 1, 0), Cqp11L(0xb, 2, -1, -1, -1, 0),
    Cqp11D(0xc, 6, 4, 0, 6, 1, 0), Cqp11L(0xc, 0, 5, 1, 1, 0), Cqp11L(0xc, 1, 5, 1, 1, 0),
    Cqp11L(0xc, 2, -1, -1, -1, 0), Cqp11D(0xd, 0xd, 4, 0, 6, 1, 0), Cqp11L(0xd, 0, 5, 1, 1, 0),
    Cqp11L(0xd, 1, 5, 1, 1, 0), Cqp11L(0xd, 2, -1, -1, -1, 0), Cqp11D(0xe, 0x14, 4, 0, 6, 1, 0),
    Cqp11L(0xe, 0, -1, -1, -1, 0),
];

/// The São Paulo state league (Paulista), stage 0: 2 rounds (21 Apr to 10 May), 208 bytes, 36 operations.
pub(crate) const SAO_PAULO_SCHEDULE_STAGE_0_OPS: [AsiaClubChampScheduleOp; 36] = [
    Cqp11S(0, 0x15, 4, 0, 6), Cqp11J(0, 0x1b, 4, 0, 6, 1, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 8), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 0x804),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 4, 5, 0, 0), Cqp11J(1, 0xa, 5, 0, 6, 1, 0), Cqp11W(0x71, 1), Cqp11W(0x82, 1),
    Cqp11W(0x6f, 0x96), Cqp11W(0x73, 0), Cqp11W(0x75, 0), Cqp11B(0x7f, 8), Cqp11W(0x80, 2), Cqp11W(0x84, 0),
    Cqp11W(0x77, 0x804), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0xa4cb8), Cqp11X(0xcc, 0x2ab98),
];
