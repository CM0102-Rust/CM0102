// ---------- batch 287: the Série A, Série B, J1 and NSL schedules (the dates and matches of each round of the round-robin and knockout stages) (bra_nat_first.cpp, bra_nat_second.cpp, jap_j1.cpp, aus_nsl.cpp) ----------

/// The Brazilian Série A, the round-robin stage (−1): 27 rounds (5 Jul to 2 Nov), 1755 bytes, 88 operations.
const BRAZIL_SERIE_A_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 88] = [
    Cqp11D(0, 5, 7, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 8, 7, 0, 2, 2, 0), Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0xc, 7, 0, 6, 1, 0), Cqp11L(2, 0, 5, 1, 1, 0),
    Cqp11L(2, 1, 5, 1, 1, 0), Cqp11L(2, 2, -1, -1, -1, 0), Cqp11D(3, 0xf, 7, 0, 2, 2, 0), Cqp11L(3, 0, -1, -1, -1, 0),
    Cqp11D(4, 0x13, 7, 0, 6, 1, 0), Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0),
    Cqp11D(5, 0x1a, 7, 0, 6, 1, 0), Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0),
    Cqp11D(6, 0x1d, 7, 0, 2, 2, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 2, 8, 0, 6, 1, 0),
    Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0), Cqp11L(7, 2, -1, -1, -1, 0), Cqp11D(8, 5, 8, 0, 2, 2, 0),
    Cqp11L(8, 0, -1, -1, -1, 0), Cqp11D(9, 9, 8, 0, 6, 1, 0), Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0),
    Cqp11L(9, 2, -1, -1, -1, 0), Cqp11D(0xa, 0x10, 8, 0, 6, 1, 0), Cqp11L(0xa, 0, 5, 1, 1, 0),
    Cqp11L(0xa, 1, 5, 1, 1, 0), Cqp11L(0xa, 2, -1, -1, -1, 0), Cqp11D(0xb, 0x13, 8, 0, 2, 2, 0),
    Cqp11L(0xb, 0, -1, -1, -1, 0), Cqp11D(0xc, 0x17, 8, 0, 6, 1, 0), Cqp11L(0xc, 0, 5, 1, 1, 0),
    Cqp11L(0xc, 1, 5, 1, 1, 0), Cqp11L(0xc, 2, -1, -1, -1, 0), Cqp11D(0xd, 0x1e, 8, 0, 6, 1, 0),
    Cqp11L(0xd, 0, 5, 1, 1, 0), Cqp11L(0xd, 1, 5, 1, 1, 0), Cqp11L(0xd, 2, -1, -1, -1, 0),
    Cqp11D(0xe, 3, 9, 0, 2, 2, 0), Cqp11L(0xe, 0, -1, -1, -1, 0), Cqp11D(0xf, 7, 9, 0, 6, 1, 0),
    Cqp11L(0xf, 0, 5, 1, 1, 0), Cqp11L(0xf, 1, 5, 1, 1, 0), Cqp11L(0xf, 2, -1, -1, -1, 0),
    Cqp11D(0x10, 0xa, 9, 0, 2, 2, 0), Cqp11L(0x10, 0, -1, -1, -1, 0), Cqp11D(0x11, 0xe, 9, 0, 6, 1, 0),
    Cqp11L(0x11, 0, 5, 1, 1, 0), Cqp11L(0x11, 1, 5, 1, 1, 0), Cqp11L(0x11, 2, -1, -1, -1, 0),
    Cqp11D(0x12, 0x15, 9, 0, 6, 1, 0), Cqp11L(0x12, 0, 5, 1, 1, 0), Cqp11L(0x12, 1, 5, 1, 1, 0),
    Cqp11L(0x12, 2, -1, -1, -1, 0), Cqp11D(0x13, 0x1c, 9, 0, 6, 1, 0), Cqp11L(0x13, 0, 5, 1, 1, 0),
    Cqp11L(0x13, 1, 5, 1, 1, 0), Cqp11L(0x13, 2, -1, -1, -1, 0), Cqp11D(0x14, 4, 0xa, 0, 6, 1, 0),
    Cqp11L(0x14, 0, 5, 1, 1, 0), Cqp11L(0x14, 1, 5, 1, 1, 0), Cqp11L(0x14, 2, -1, -1, -1, 0),
    Cqp11D(0x15, 7, 0xa, 0, 2, 2, 0), Cqp11L(0x15, 0, -1, -1, -1, 0), Cqp11D(0x16, 0xb, 0xa, 0, 6, 1, 0),
    Cqp11L(0x16, 0, 5, 1, 1, 0), Cqp11L(0x16, 1, 5, 1, 1, 0), Cqp11L(0x16, 2, -1, -1, -1, 0),
    Cqp11D(0x17, 0xe, 0xa, 0, 2, 2, 0), Cqp11L(0x17, 0, -1, -1, -1, 0), Cqp11D(0x18, 0x12, 0xa, 0, 6, 1, 0),
    Cqp11L(0x18, 0, 5, 1, 1, 0), Cqp11L(0x18, 1, 5, 1, 1, 0), Cqp11L(0x18, 2, -1, -1, -1, 0),
    Cqp11D(0x19, 0x19, 0xa, 0, 6, 1, 0), Cqp11L(0x19, 0, 5, 1, 1, 0), Cqp11L(0x19, 1, 5, 1, 1, 0),
    Cqp11L(0x19, 2, -1, -1, -1, 0), Cqp11D(0x1a, 2, 0xb, 0, 6, 1, 0), Cqp11L(0x1a, 0, -1, -1, -1, 0),
];

/// The Brazilian Série A, stage 0: 2 rounds (3 Nov to 12 Nov), 208 bytes, 38 operations.
const BRAZIL_SERIE_A_SCHEDULE_STAGE_0_OPS: [AsiaClubChampScheduleOp; 38] = [
    Cqp11S(0, 3, 0xb, 0, 0), Cqp11J(0, 5, 0xb, 0, 2, 2, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 8),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 5), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 1),
    Cqp11W(0x11, 0), Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 4), Cqp11X(0x5c, 0),
    Cqp11X(0x60, 0), Cqp11X(0x64, 0), Cqp11S(1, 6, 0xb, 0, 3), Cqp11J(1, 0xc, 0xb, 0, 2, 2, 0), Cqp11W(0x6f, 0x96),
    Cqp11W(0x71, 1), Cqp11W(0x73, 8), Cqp11W(0x75, 0x200), Cqp11B(0x7f, 5), Cqp11W(0x80, 2), Cqp11W(0x82, 1),
    Cqp11W(0x84, 0), Cqp11W(0x77, 1), Cqp11W(0x79, 0), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2),
    Cqp11B(0x8a, 4), Cqp11X(0xc4, 0), Cqp11X(0xc8, 0xf4240), Cqp11X(0xcc, 0),
];

/// The stages of the round table of the Brazilian Série A (virtual `comp_vtable::CREATE_SCHEDULE`): stage −1, the round-robin league stage: 27 rounds from 5 Jul to 2 Nov; stage 0, the knockout or final stage: 2 rounds from 3 Nov to 12 Nov; any other stage has no table (None).
fn brazil_serie_a_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 27, extra: 0, extra_check: false, size: 0x6db, row: 0xb6,
            ops: &BRAZIL_SERIE_A_SCHEDULE_LEAGUE_OPS, name: "cqw_4 serie a league" }),
        0 => Some(ScheduleStage { league: 0, rounds: 2, extra: 0, extra_check: false, size: 0xd0, row: 0x15f,
            ops: &BRAZIL_SERIE_A_SCHEDULE_STAGE_0_OPS, name: "cqw_4 serie a stage 0" }),
        _ => None,
    }
}

/// The schedule of the Brazilian Série A: stage −1, the round-robin league stage: 27 rounds from 5 Jul to 2 Nov; stage 0, the knockout or final stage: 2 rounds from 3 Nov to 12 Nov; any other stage returns 0.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures and the season turns. For a stage with a table (`brazil_serie_a_schedule_stage`) it writes the outputs (is_league
/// when that pointer is not null, the rounds and the extra), allocates the round table (`exe::crt_malloc`, 0x41 bytes
/// per round; out of memory, the error box and 0) and writes the date (day, month, years after the season year
/// `year()`, weekday and code) and the matches of each round, in the original's order (`create_round_table`, the
/// cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table, or 0 without writing
/// anything for a stage without one.
/// Original: `BraNatFirst::create_schedule` (0x429b50, bra_nat_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_nat_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), brazil_serie_a_schedule_stage(stage), rounds, extra, league, 0x98aa30)
}

/// Comparer for `bra_nat_first_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_NAT_FIRST_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_nat_first_create_schedule", stage, brazil_serie_a_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_nat_first_create_schedule(this, stage, r, e, l))
}

/// The Brazilian Série B, the round-robin stage (−1): 27 rounds (5 Jul to 2 Nov), 1755 bytes, 88 operations.
const BRAZIL_SERIE_B_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 88] = [
    Cqp11D(0, 5, 7, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 8, 7, 0, 2, 2, 0), Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0xc, 7, 0, 6, 1, 0), Cqp11L(2, 0, 5, 1, 1, 0),
    Cqp11L(2, 1, 5, 1, 1, 0), Cqp11L(2, 2, -1, -1, -1, 0), Cqp11D(3, 0xf, 7, 0, 2, 2, 0), Cqp11L(3, 0, -1, -1, -1, 0),
    Cqp11D(4, 0x13, 7, 0, 6, 1, 0), Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0),
    Cqp11D(5, 0x1a, 7, 0, 6, 1, 0), Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0),
    Cqp11D(6, 0x1d, 7, 0, 2, 2, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 2, 8, 0, 6, 1, 0),
    Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0), Cqp11L(7, 2, -1, -1, -1, 0), Cqp11D(8, 5, 8, 0, 2, 2, 0),
    Cqp11L(8, 0, -1, -1, -1, 0), Cqp11D(9, 9, 8, 0, 6, 1, 0), Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0),
    Cqp11L(9, 2, -1, -1, -1, 0), Cqp11D(0xa, 0x10, 8, 0, 6, 1, 0), Cqp11L(0xa, 0, 5, 1, 1, 0),
    Cqp11L(0xa, 1, 5, 1, 1, 0), Cqp11L(0xa, 2, -1, -1, -1, 0), Cqp11D(0xb, 0x13, 8, 0, 2, 2, 0),
    Cqp11L(0xb, 0, -1, -1, -1, 0), Cqp11D(0xc, 0x17, 8, 0, 6, 1, 0), Cqp11L(0xc, 0, 5, 1, 1, 0),
    Cqp11L(0xc, 1, 5, 1, 1, 0), Cqp11L(0xc, 2, -1, -1, -1, 0), Cqp11D(0xd, 0x1e, 8, 0, 6, 1, 0),
    Cqp11L(0xd, 0, 5, 1, 1, 0), Cqp11L(0xd, 1, 5, 1, 1, 0), Cqp11L(0xd, 2, -1, -1, -1, 0),
    Cqp11D(0xe, 3, 9, 0, 2, 2, 0), Cqp11L(0xe, 0, -1, -1, -1, 0), Cqp11D(0xf, 7, 9, 0, 6, 1, 0),
    Cqp11L(0xf, 0, 5, 1, 1, 0), Cqp11L(0xf, 1, 5, 1, 1, 0), Cqp11L(0xf, 2, -1, -1, -1, 0),
    Cqp11D(0x10, 0xa, 9, 0, 2, 2, 0), Cqp11L(0x10, 0, -1, -1, -1, 0), Cqp11D(0x11, 0xe, 9, 0, 6, 1, 0),
    Cqp11L(0x11, 0, 5, 1, 1, 0), Cqp11L(0x11, 1, 5, 1, 1, 0), Cqp11L(0x11, 2, -1, -1, -1, 0),
    Cqp11D(0x12, 0x15, 9, 0, 6, 1, 0), Cqp11L(0x12, 0, 5, 1, 1, 0), Cqp11L(0x12, 1, 5, 1, 1, 0),
    Cqp11L(0x12, 2, -1, -1, -1, 0), Cqp11D(0x13, 0x1c, 9, 0, 6, 1, 0), Cqp11L(0x13, 0, 5, 1, 1, 0),
    Cqp11L(0x13, 1, 5, 1, 1, 0), Cqp11L(0x13, 2, -1, -1, -1, 0), Cqp11D(0x14, 4, 0xa, 0, 6, 1, 0),
    Cqp11L(0x14, 0, 5, 1, 1, 0), Cqp11L(0x14, 1, 5, 1, 1, 0), Cqp11L(0x14, 2, -1, -1, -1, 0),
    Cqp11D(0x15, 7, 0xa, 0, 2, 2, 0), Cqp11L(0x15, 0, -1, -1, -1, 0), Cqp11D(0x16, 0xb, 0xa, 0, 6, 1, 0),
    Cqp11L(0x16, 0, 5, 1, 1, 0), Cqp11L(0x16, 1, 5, 1, 1, 0), Cqp11L(0x16, 2, -1, -1, -1, 0),
    Cqp11D(0x17, 0xe, 0xa, 0, 2, 2, 0), Cqp11L(0x17, 0, -1, -1, -1, 0), Cqp11D(0x18, 0x12, 0xa, 0, 6, 1, 0),
    Cqp11L(0x18, 0, 5, 1, 1, 0), Cqp11L(0x18, 1, 5, 1, 1, 0), Cqp11L(0x18, 2, -1, -1, -1, 0),
    Cqp11D(0x19, 0x19, 0xa, 0, 6, 1, 0), Cqp11L(0x19, 0, 5, 1, 1, 0), Cqp11L(0x19, 1, 5, 1, 1, 0),
    Cqp11L(0x19, 2, -1, -1, -1, 0), Cqp11D(0x1a, 2, 0xb, 0, 6, 1, 0), Cqp11L(0x1a, 0, -1, -1, -1, 0),
];

/// The Brazilian Série B, stage 0: 2 rounds (3 Nov to 12 Nov), 208 bytes, 38 operations.
const BRAZIL_SERIE_B_SCHEDULE_STAGE_0_OPS: [AsiaClubChampScheduleOp; 38] = [
    Cqp11S(0, 3, 0xb, 0, 0), Cqp11J(0, 5, 0xb, 0, 2, 2, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 8),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 5), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 1),
    Cqp11W(0x11, 0), Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 4), Cqp11X(0x5c, 0),
    Cqp11X(0x60, 0), Cqp11X(0x64, 0), Cqp11S(1, 6, 0xb, 0, 3), Cqp11J(1, 0xc, 0xb, 0, 2, 2, 0), Cqp11W(0x6f, 0x96),
    Cqp11W(0x71, 1), Cqp11W(0x73, 8), Cqp11W(0x75, 0x200), Cqp11B(0x7f, 5), Cqp11W(0x80, 2), Cqp11W(0x82, 1),
    Cqp11W(0x84, 0), Cqp11W(0x77, 1), Cqp11W(0x79, 0), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2),
    Cqp11B(0x8a, 4), Cqp11X(0xc4, 0), Cqp11X(0xc8, 0x7a120), Cqp11X(0xcc, 0),
];

/// The stages of the round table of the Brazilian Série B (virtual `comp_vtable::CREATE_SCHEDULE`): stage −1, the round-robin league stage: 27 rounds from 5 Jul to 2 Nov; stage 0, the knockout or final stage: 2 rounds from 3 Nov to 12 Nov; any other stage has no table (None).
fn brazil_serie_b_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 27, extra: 0, extra_check: false, size: 0x6db, row: 0xb0,
            ops: &BRAZIL_SERIE_B_SCHEDULE_LEAGUE_OPS, name: "cqw_4 serie b league" }),
        0 => Some(ScheduleStage { league: 0, rounds: 2, extra: 0, extra_check: false, size: 0xd0, row: 0x159,
            ops: &BRAZIL_SERIE_B_SCHEDULE_STAGE_0_OPS, name: "cqw_4 serie b stage 0" }),
        _ => None,
    }
}

/// The schedule of the Brazilian Série B: stage −1, the round-robin league stage: 27 rounds from 5 Jul to 2 Nov; stage 0, the knockout or final stage: 2 rounds from 3 Nov to 12 Nov; any other stage returns 0.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures and the season turns. For a stage with a table (`brazil_serie_b_schedule_stage`) it writes the outputs (is_league
/// when that pointer is not null, the rounds and the extra), allocates the round table (`exe::crt_malloc`, 0x41 bytes
/// per round; out of memory, the error box and 0) and writes the date (day, month, years after the season year
/// `year()`, weekday and code) and the matches of each round, in the original's order (`create_round_table`, the
/// cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table, or 0 without writing
/// anything for a stage without one.
/// Original: `BraNatSecond::create_schedule` (0x42c140, bra_nat_second.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_nat_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), brazil_serie_b_schedule_stage(stage), rounds, extra, league, 0x98ab64)
}

/// Comparer for `bra_nat_second_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_NAT_SECOND_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_nat_second_create_schedule", stage, brazil_serie_b_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_nat_second_create_schedule(this, stage, r, e, l))
}

/// The Japanese J1, the round-robin stage (−1): 30 rounds (11 Feb to 26 Oct), 1950 bytes, 60 operations.
const J1_LEAGUE_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 60] = [
    Cqp11D(0, 0xb, 2, 0, 5, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x12, 2, 0, 5, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0x19, 2, 0, 5, 1, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 1, 3, 0, 5, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 5, 3, 0, 2, 2, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 8, 3, 0, 5, 1, 0), Cqp11L(5, 0, -1, -1, -1, 0),
    Cqp11D(6, 0xf, 3, 0, 5, 1, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0x16, 3, 0, 5, 1, 0),
    Cqp11L(7, 0, -1, -1, -1, 0), Cqp11D(8, 0x1d, 3, 0, 5, 1, 0), Cqp11L(8, 0, -1, -1, -1, 0),
    Cqp11D(9, 3, 4, 0, 2, 2, 0), Cqp11L(9, 0, -1, -1, -1, 0), Cqp11D(0xa, 6, 4, 0, 5, 1, 0),
    Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 0xd, 4, 0, 5, 1, 0), Cqp11L(0xb, 0, -1, -1, -1, 0),
    Cqp11D(0xc, 0x11, 4, 0, 2, 2, 0), Cqp11L(0xc, 0, -1, -1, -1, 0), Cqp11D(0xd, 0x14, 4, 0, 5, 1, 0),
    Cqp11L(0xd, 0, -1, -1, -1, 0), Cqp11D(0xe, 0x1b, 4, 0, 5, 1, 0), Cqp11L(0xe, 0, -1, -1, -1, 0),
    Cqp11D(0xf, 0x18, 5, 0, 5, 1, 0), Cqp11L(0xf, 0, -1, -1, -1, 0), Cqp11D(0x10, 1, 6, 0, 5, 1, 0),
    Cqp11L(0x10, 0, -1, -1, -1, 0), Cqp11D(0x11, 8, 6, 0, 5, 1, 0), Cqp11L(0x11, 0, -1, -1, -1, 0),
    Cqp11D(0x12, 0xf, 6, 0, 5, 1, 0), Cqp11L(0x12, 0, -1, -1, -1, 0), Cqp11D(0x13, 0x16, 6, 0, 5, 1, 0),
    Cqp11L(0x13, 0, -1, -1, -1, 0), Cqp11D(0x14, 0x1a, 6, 0, 2, 2, 0), Cqp11L(0x14, 0, -1, -1, -1, 0),
    Cqp11D(0x15, 0x1d, 6, 0, 5, 1, 0), Cqp11L(0x15, 0, -1, -1, -1, 0), Cqp11D(0x16, 5, 7, 0, 5, 1, 0),
    Cqp11L(0x16, 0, -1, -1, -1, 0), Cqp11D(0x17, 0xc, 7, 0, 5, 1, 0), Cqp11L(0x17, 0, -1, -1, -1, 0),
    Cqp11D(0x18, 0x13, 7, 0, 5, 1, 0), Cqp11L(0x18, 0, -1, -1, -1, 0), Cqp11D(0x19, 8, 0xa, 0, 2, 2, 0),
    Cqp11L(0x19, 0, -1, -1, -1, 0), Cqp11D(0x1a, 0xb, 0xa, 0, 5, 1, 0), Cqp11L(0x1a, 0, -1, -1, -1, 0),
    Cqp11D(0x1b, 0x12, 0xa, 0, 5, 1, 0), Cqp11L(0x1b, 0, -1, -1, -1, 0), Cqp11D(0x1c, 0x17, 0xa, 0, 3, 2, 0),
    Cqp11L(0x1c, 0, -1, -1, -1, 0), Cqp11D(0x1d, 0x1a, 0xa, 0, 6, 1, 0), Cqp11L(0x1d, 0, -1, -1, -1, 0),
];

/// The Japanese J1, stage 2: 1 round (27 Oct to 2 Nov), 104 bytes, 18 operations.
const J1_LEAGUE_SCHEDULE_STAGE_2_OPS: [AsiaClubChampScheduleOp; 18] = [
    Cqp11S(0, 0x1b, 0xa, 0, 0), Cqp11J(0, 2, 0xb, 0, 5, 1, 0), Cqp11W(0x18, 2), Cqp11W(0x1c, 2), Cqp11W(0x7, 0x96),
    Cqp11W(0x9, 0), Cqp11W(0xb, 0), Cqp11W(0xd, 0), Cqp11B(0x17, 5), Cqp11W(0x1a, 1), Cqp11W(0xf, 0x83),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0),
];

/// The stages of the round table of the Japanese J1 (virtual `comp_vtable::CREATE_SCHEDULE`): stage −1, the round-robin league stage: 30 rounds from 11 Feb to 26 Oct; stage 2, the knockout or final stage: 1 round from 27 Oct to 2 Nov; any other stage has no table (None).
fn j1_league_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 30, extra: 0, extra_check: false, size: 0x79e, row: 0xab,
            ops: &J1_LEAGUE_SCHEDULE_LEAGUE_OPS, name: "cqw_4 j1 league" }),
        2 => Some(ScheduleStage { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0x11a,
            ops: &J1_LEAGUE_SCHEDULE_STAGE_2_OPS, name: "cqw_4 j1 stage 2" }),
        _ => None,
    }
}

/// The schedule of the Japanese J1: stage −1, the round-robin league stage: 30 rounds from 11 Feb to 26 Oct; stage 2, the knockout or final stage: 1 round from 27 Oct to 2 Nov; any other stage returns 0.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures and the season turns. For a stage with a table (`j1_league_schedule_stage`) it writes the outputs (is_league
/// when that pointer is not null, the rounds and the extra), allocates the round table (`exe::crt_malloc`, 0x41 bytes
/// per round; out of memory, the error box and 0) and writes the date (day, month, years after the season year
/// `year()`, weekday and code) and the matches of each round, in the original's order (`create_round_table`, the
/// cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table, or 0 without writing
/// anything for a stage without one.
/// Original: `JapJ1::create_schedule` (0x662e30, jap_j1.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn jap_j1_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), j1_league_schedule_stage(stage), rounds, extra, league, 0x9eabc8)
}

/// Comparer for `jap_j1_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_JAP_J1_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("jap_j1_create_schedule", stage, j1_league_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| jap_j1_create_schedule(this, stage, r, e, l))
}

/// The Australian NSL, every stage other than −1 and 0 (the final): 1 round (1 Jun of the following year to 7 Jun of the following year), 104 bytes, 19 operations.
const NSL_SCHEDULE_OTHER_STAGES_OPS: [AsiaClubChampScheduleOp; 19] = [
    Cqp11S(0, 1, 6, 1, 0), Cqp11J(0, 7, 6, 1, 5, 1, 0), Cqp11W(0x7, 0), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x83), Cqp11B(0x17, 5), Cqp11W(0x18, 2), Cqp11W(0x1a, 1), Cqp11W(0x1c, 2), Cqp11W(0xf, 0),
    Cqp11W(0x11, 0), Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 1), Cqp11B(0x22, 0), Cqp11X(0x5c, 0),
    Cqp11X(0x60, 0x2ee0), Cqp11X(0x64, 0x1388),
];

/// The Australian NSL, the round-robin stage (−1): 24 rounds (5 Sep to 15 Feb of the following year), 1560 bytes, 48 operations.
const NSL_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 48] = [
    Cqp11D(0, 5, 9, 0, 5, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0xc, 9, 0, 5, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0x13, 9, 0, 5, 1, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 0x1a, 9, 0, 5, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 2, 0xa, 0, 5, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 9, 0xa, 0, 5, 1, 0), Cqp11L(5, 0, -1, -1, -1, 0),
    Cqp11D(6, 0x10, 0xa, 0, 2, 2, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0x17, 0xa, 0, 5, 1, 0),
    Cqp11L(7, 0, -1, -1, -1, 0), Cqp11D(8, 0x1e, 0xa, 0, 5, 1, 0), Cqp11L(8, 0, -1, -1, -1, 0),
    Cqp11D(9, 7, 0xb, 0, 5, 1, 0), Cqp11L(9, 0, -1, -1, -1, 0), Cqp11D(0xa, 0xe, 0xb, 0, 5, 1, 0),
    Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 0x15, 0xb, 0, 5, 1, 0), Cqp11L(0xb, 0, -1, -1, -1, 0),
    Cqp11D(0xc, 0x1c, 0xb, 0, 5, 1, 0), Cqp11L(0xc, 0, -1, -1, -1, 0), Cqp11D(0xd, 4, 0, 1, 5, 1, 0),
    Cqp11L(0xd, 0, -1, -1, -1, 0), Cqp11D(0xe, 0xb, 0, 1, 5, 1, 0), Cqp11L(0xe, 0, -1, -1, -1, 0),
    Cqp11D(0xf, 0x12, 0, 1, 5, 1, 0), Cqp11L(0xf, 0, -1, -1, -1, 0), Cqp11D(0x10, 0x19, 0, 1, 5, 1, 0),
    Cqp11L(0x10, 0, -1, -1, -1, 0), Cqp11D(0x11, 1, 1, 1, 5, 1, 0), Cqp11L(0x11, 0, -1, -1, -1, 0),
    Cqp11D(0x12, 8, 1, 1, 5, 1, 0), Cqp11L(0x12, 0, -1, -1, -1, 0), Cqp11D(0x13, 0xf, 1, 1, 5, 1, 0),
    Cqp11L(0x13, 0, -1, -1, -1, 0), Cqp11D(0x14, 0x16, 1, 1, 5, 1, 0), Cqp11L(0x14, 0, -1, -1, -1, 0),
    Cqp11D(0x15, 1, 2, 1, 5, 1, 0), Cqp11L(0x15, 0, -1, -1, -1, 0), Cqp11D(0x16, 8, 2, 1, 5, 1, 0),
    Cqp11L(0x16, 0, -1, -1, -1, 0), Cqp11D(0x17, 0xf, 2, 1, 5, 1, 0), Cqp11L(0x17, 0, -1, -1, -1, 0),
];

/// The Australian NSL, stage 0 (league): 10 rounds (29 Feb of the following year to 29 May of the following year), 650 bytes, 20 operations.
const NSL_SCHEDULE_STAGE_0_OPS: [AsiaClubChampScheduleOp; 20] = [
    Cqp11D(0, 0x1d, 2, 1, 5, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 5, 3, 1, 5, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0xc, 3, 1, 5, 1, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 0x11, 3, 1, 5, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 0x18, 3, 1, 5, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 1, 5, 1, 5, 1, 0), Cqp11L(5, 0, -1, -1, -1, 0),
    Cqp11D(6, 8, 5, 1, 5, 1, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0xf, 5, 1, 5, 1, 0),
    Cqp11L(7, 0, -1, -1, -1, 0), Cqp11D(8, 0x16, 5, 1, 5, 1, 0), Cqp11L(8, 0, -1, -1, -1, 0),
    Cqp11D(9, 0x1d, 5, 1, 5, 1, 0), Cqp11L(9, 0, -1, -1, -1, 0),
];

/// The stages of the round table of the Australian NSL (virtual `comp_vtable::CREATE_SCHEDULE`): stage −1, the round-robin league stage: 24 rounds from 5 Sep to 15 Feb of the following year, stage id 0x434; stage 0, a second league stage: 10 rounds from 29 Feb of the following year to 29 May of the following year, stage id 0x435; any other stage, the final: 1 round from 1 Jun of the following year to 7 Jun of the following year, stage id 0x96.
fn nsl_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 24, extra: 0x434, extra_check: false, size: 0x618, row: 0xa7,
            ops: &NSL_SCHEDULE_LEAGUE_OPS, name: "cqw_4 nsl league" }),
        0 => Some(ScheduleStage { league: 1, rounds: 10, extra: 0x435, extra_check: false, size: 0x28a, row: 0x123,
            ops: &NSL_SCHEDULE_STAGE_0_OPS, name: "cqw_4 nsl league" }),
        _ => Some(ScheduleStage { league: 0, rounds: 1, extra: 0x96, extra_check: false, size: 0x68, row: 0x164,
            ops: &NSL_SCHEDULE_OTHER_STAGES_OPS, name: "cqw_4 nsl other stages" }),
    }
}

/// The schedule of the Australian NSL: stage −1, the round-robin league stage: 24 rounds from 5 Sep to 15 Feb of the following year, stage id 0x434; stage 0, a second league stage: 10 rounds from 29 Feb of the following year to 29 May of the following year, stage id 0x435; any other stage, the final: 1 round from 1 Jun of the following year to 7 Jun of the following year, stage id 0x96.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures and the season turns. For a stage with a table (`nsl_schedule_stage`) it writes the outputs (is_league
/// when that pointer is not null, the rounds and the extra), allocates the round table (`exe::crt_malloc`, 0x41 bytes
/// per round; out of memory, the error box and 0) and writes the date (day, month, years after the season year
/// `year()`, weekday and code) and the matches of each round, in the original's order (`create_round_table`, the
/// cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table, or 0 without writing
/// anything for a stage without one.
/// Original: `AusNsl::create_schedule` (0x4111a0, aus_nsl.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn aus_nsl_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), nsl_schedule_stage(stage), rounds, extra, league, 0x987c20)
}

/// Comparer for `aus_nsl_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_aus_nsl_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_AUS_NSL_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("aus_nsl_create_schedule", stage, nsl_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| aus_nsl_create_schedule(this, stage, r, e, l))
}

// ---------- batch 287: the constructors of the Asian Super Cup, the Interamerican Cup, the Mercosur Cup and the MLS All-Star Game (asia_super_cup.cpp, con_merc_cup.cpp, inter_amer_cup.cpp, usa_mls_all_stars.cpp) ----------

/// Builds a new cup's round table through virtual `comp_vtable::CREATE_SCHEDULE` and stores it in `round_table()`.
///
/// The call is `create_schedule(−1, &round_count, &stage_label, 0)` (push 0; push &+0x3a; push &+0x3c; push −1; mov
/// ecx, this -> thiscall, ret 0x10): stage −1, the rounds into `cup::ROUND_COUNT` and the extra into
/// `cup::STAGE_LABEL`, no is_league output.
unsafe fn build_cup_round_table_by_vtable(this: Cup) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = f(this.addr(), 0xffff_ffff, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0);
    write_u32(this.addr() + cup::ROUND_TABLE, t as u32);
}

/// Creates the competition's statistics and stores them at +8 (the Comp base field, `league::COMP_STATS`); returns
/// whether they were created.
///
/// Operator new `exe::crt_operator_new` of 0x5ce bytes (push; add esp, 4 -> cdecl) and, when it is not null,
/// Comp_Stats::Comp_Stats `exe::new_comp_stats` (push this; mov ecx, new -> thiscall, ret 4). The original's SEH frame
/// covers only this `new`.
unsafe fn create_comp_stats_at_8(this: usize) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    write_u32(this + 8, s as u32);
    s != 0
}

/// The plan the comparer checks after the original constructor ran; `None` when it holds, otherwise the difference.
///
/// Always: the return value is `this`, the vtable, the competition record at +4 and the year (`cup::YEAR`, word). On a
/// new cup (`cup::STAGE_INDEX` = 0xff), every fixed field the constructor writes (`fields`: offset, width in bytes,
/// value), the round table (`cup::ROUND_TABLE`, when `rounds`) and the statistics at +8 must be set.
unsafe fn cup_constructor_plan_mismatch(this: usize, r: usize, year: u32, record: usize, vt: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    if r != this || read_usize(this) != vt || read_usize(this + 4) != record || read_u16(this + cup::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this), read_usize(this + 4), read_u16(this + cup::YEAR)));
    }
    if read_u8(this + cup::STAGE_INDEX) != stage_index::MAIN {
        return None;
    }
    for &(o, t, v) in fields {
        let x = match t {
            1 => read_u8(this + o) as u32,
            2 => read_u16(this + o) as u32,
            _ => read_u32(this + o),
        };
        if x != v {
            return Some(format!("+{:#x} = {:#x}, the constructor writes {:#x}", o, x, v));
        }
    }
    if (rounds && read_usize(this + cup::ROUND_TABLE) == 0) || read_usize(this + 8) == 0 {
        return Some(format!("rounds {:#x}, statistics {:#x}", read_usize(this + cup::ROUND_TABLE), read_usize(this + 8)));
    }
    None
}

/// The constructor of the Asian Super Cup (`AsiaSuperCup::AsiaSuperCup(year, record)`), run once, at career creation.
///
/// On the Cup base (Cup::Cup `exe::new_cup`, thiscall, ret 0): the competition record at +4, the vtable 0x9676c8, the
/// year (`set_year`), `current_round` 0, `stage_index` 0xff (the main competition), `stage_count` 0, `current_stage` −1,
/// `club_sides` 2 (clubs of several nations), 7 `substitutes_named`, 3 `substitutions_allowed`, `discipline_index` 41
/// and `run_by_stages` 0; the prize fund (+0x28) stays as the base left it. If the saved state loads
/// (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this -> thiscall, ret 4; returns nonzero), returns this.
///
/// On a new cup, in order, each step with its error box (`show_error_at`, source 0x987be0) and returning this when it
/// fails: the two clubs (AsiaSuperCup::select_teams 0x410a50, thiscall, ret 0; the box of line 0x3e when it returns a
/// negative value); the round table (`build_cup_round_table_by_vtable`); the tie tree (Cup::map_fixture_tree 0x518790,
/// thiscall, ret 0; the box of line 0x4c when negative); the statistics at +8 (`create_comp_stats_at_8`; the box of
/// line 0x53 without them); the final ranking (Cup::rank_teams 0x5223a0, thiscall, ret 0, unchecked). Returns this.
unsafe fn build_asian_super_cup(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_u32(this.addr() + 4, record as u32);
    write_u32(this.addr(), 0x9676c8);
    this.set_year(year as u16 as i16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_u32(this.addr() + cup::STAGE_COUNT, 0);
    write_u32(this.addr() + cup::CURRENT_STAGE, 0xffff_ffff);
    this.set_club_sides(2);
    this.set_substitutes_named(7);
    this.set_substitutions_allowed(3);
    this.set_discipline_index(0x29);
    this.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load_from_disk(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqw_5 asian super cup new");
    let select_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x410a50usize); // AsiaSuperCup::select_teams
    if select_teams(this.addr()) < 0 {
        show_error_at(0x987be0, 0x3e);
        return this.addr();
    }
    build_cup_round_table_by_vtable(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    if map_fixture_tree(this.addr()) < 0 {
        show_error_at(0x987be0, 0x4c);
        return this.addr();
    }
    if !create_comp_stats_at_8(this.addr()) {
        show_error_at(0x987be0, 0x53);
        return this.addr();
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize); // Cup::rank_teams
    rank_teams(this.addr());
    this.addr()
}

/// The fixed fields the Asian Super Cup constructor writes (offset, width, value), checked by the comparer.
const ASIAN_SUPER_CUP_CTOR_FIELDS: [(usize, u8, u32); 9] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x2), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x29), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The Asian Super Cup at career creation: the year and the database competition; a new cup has two clubs of several
/// nations, 7 substitutes on the bench and 3 substitutions, discipline rule 41, no prize fund of its own, the rounds,
/// the tie tree, the statistics and the final ranking (see `build_asian_super_cup`).
/// Original: `AsiaSuperCup::AsiaSuperCup` (0x410650, asia_super_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_asia_super_cup(this: usize, year: u32, record: usize) -> usize {
    build_asian_super_cup(Cup(this), year, record) // cqw5: the Asian Super Cup
}

/// Comparer for `construct_asia_super_cup`: only the original runs (it allocates, draws and reads the save file),
/// then the plan is checked (`cup_constructor_plan_mismatch`).
pub(crate) unsafe extern "thiscall" fn compare_construct_asia_super_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ASIA_SUPER_CUP)(this, year, record);
    let error = cup_constructor_plan_mismatch(this, r, year, record, 0x9676c8, &ASIAN_SUPER_CUP_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_asia_super_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The constructor of the Interamerican Cup (`InterAmerCup::InterAmerCup(year, record)`), run once, at career creation.
///
/// On the Cup base (Cup::Cup `exe::new_cup`, thiscall, ret 0): the year (`set_year`), the competition record at +4,
/// the vtable 0x96bad8, `current_round` 0, `stage_index` 0xff, `stage_count` 0, `current_stage` −1, `club_sides` 2
/// (clubs of several nations), +0x52 = 2, 7 `substitutes_named`, 3 `substitutions_allowed`, `discipline_index` 0xff and
/// `run_by_stages` 0; the prize fund (+0x28) stays as the base left it. If the saved state loads (Cup::load_from_disk
/// 0x51fc00: push 1; mov ecx, this -> thiscall, ret 4; returns nonzero), returns this.
///
/// On a new cup, in order, each step with its error box (source 0x9e9d94) and returning this when it fails: the two
/// finalists (InterAmerCup::select_teams 0x632480, thiscall, ret 0; the box of line 0x41 when negative); the round
/// table (`build_cup_round_table_by_vtable`); the tie tree (Cup::map_fixture_tree 0x518790; the box of line 0x4f when
/// negative); the statistics at +8 (the box of line 0x56 without them); the final ranking (Cup::rank_teams 0x5223a0,
/// unchecked). Returns this.
unsafe fn build_interamerican_cup(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    this.set_year(year as u16 as i16);
    write_u32(this.addr() + 4, record as u32);
    write_u32(this.addr(), 0x96bad8);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_u32(this.addr() + cup::STAGE_COUNT, 0);
    write_u32(this.addr() + cup::CURRENT_STAGE, 0xffff_ffff);
    this.set_club_sides(2);
    write_u8(this.addr() + 0x52, 2);
    this.set_substitutes_named(7);
    this.set_substitutions_allowed(3);
    this.set_discipline_index(0xff);
    this.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load_from_disk(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqw_5 interamerican cup new");
    let select_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x632480usize); // InterAmerCup::select_teams
    if select_teams(this.addr()) < 0 {
        show_error_at(0x9e9d94, 0x41);
        return this.addr();
    }
    build_cup_round_table_by_vtable(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    if map_fixture_tree(this.addr()) < 0 {
        show_error_at(0x9e9d94, 0x4f);
        return this.addr();
    }
    if !create_comp_stats_at_8(this.addr()) {
        show_error_at(0x9e9d94, 0x56);
        return this.addr();
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize); // Cup::rank_teams
    rank_teams(this.addr());
    this.addr()
}

/// The fixed fields the Interamerican Cup constructor writes (offset, width, value), checked by the comparer.
const INTERAMERICAN_CUP_CTOR_FIELDS: [(usize, u8, u32); 10] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x2), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0xff), (0x52, 1, 0x2), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The Interamerican Cup at career creation: the year and the database competition; a new cup has the two finalists
/// (clubs of several nations), 7 substitutes on the bench and 3 substitutions, discipline rule 255, no prize fund of its
/// own, the rounds, the tie tree, the statistics and the final ranking (see `build_interamerican_cup`).
/// Original: `InterAmerCup::InterAmerCup` (0x632080, inter_amer_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_inter_amer_cup(this: usize, year: u32, record: usize) -> usize {
    build_interamerican_cup(Cup(this), year, record) // cqw5: the Interamerican Cup
}

/// Comparer for `construct_inter_amer_cup`: only the original runs (it allocates, draws and reads the save file),
/// then the plan is checked (`cup_constructor_plan_mismatch`).
pub(crate) unsafe extern "thiscall" fn compare_construct_inter_amer_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_INTER_AMER_CUP)(this, year, record);
    let error = cup_constructor_plan_mismatch(this, r, year, record, 0x96bad8, &INTERAMERICAN_CUP_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_inter_amer_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The constructor of the Mercosur Cup (`ConMercCup::ConMercCup(year, record)`), run once, at career creation.
///
/// On the League base (League::League `exe::new_league`, thiscall, ret 0): the competition record (`league::COMPETITION`),
/// the vtable 0x968b40, the year (`set_year`), `discipline_rule` 0xff, the drawn club list `con_merc_cup::DRAWN_CLUBS` = 0 and `importance` 0xf;
/// substitutes, substitutions and the prize fund stay as the base left them. If the saved state loads
/// (`exe::league_load_from_temp_file`: push 1; mov ecx, this -> thiscall, ret 4; returns nonzero), returns this.
///
/// On a new cup, in order: `league_flags` 0x28, `stage_index` 0xff, `current_stage` −1, `stage_count` 4 and the stage
/// vector (`exe::crt_malloc` of 0x10: push; add esp, 4 -> cdecl, unchecked); the rules (ConMercCup::set_up_rules
/// 0x4c1ed0, thiscall, ret 0; the error box of line 0x4d, source 0x9a7d6c, and this when it returns 0); `club_count` 0
/// and `table` 0; the statistics (`create_comp_stats_at_8`; the box of line 0x59 and this without them); `active` 0.
/// Returns this.
unsafe fn build_mercosur_cup(this: League, year: u32, record: usize) -> usize {
    exe::new_league(this);
    write_u32(this.addr() + league::COMPETITION, record as u32);
    write_u32(this.addr(), 0x968b40);
    this.set_year(year as u16 as i16);
    this.set_discipline_rule(0xff);
    write_u32(this.addr() + con_merc_cup::DRAWN_CLUBS, 0);
    this.set_importance(0xf);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!("cqw_5 mercosur cup new");
    this.set_league_flags(0x28);
    this.set_stage_index(stage_index::MAIN);
    write_u32(this.addr() + league::CURRENT_STAGE, 0xffff_ffff);
    write_u32(this.addr() + league::STAGE_COUNT, 0x4);
    write_u32(this.addr() + league::STAGES, exe::crt_malloc(0x10) as u32);
    let set_up_rules: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x4c1ed0usize); // ConMercCup::set_up_rules
    if set_up_rules(this.addr()) == 0 {
        show_error_at(0x9a7d6c, 0x4d);
        return this.addr();
    }
    this.set_club_count(0);
    write_u32(this.addr() + league::TABLE, 0);
    if !create_comp_stats_at_8(this.addr()) {
        show_error_at(0x9a7d6c, 0x59);
        return this.addr();
    }
    write_u32(this.addr() + league::ACTIVE, 0);
    this.addr()
}

/// The fixed fields the Mercosur Cup constructor writes (offset, width, value), checked by the comparer.
const MERCOSUR_CUP_CTOR_FIELDS: [(usize, u8, u32); 10] = [(league::STAGE_COUNT, 4, 0x4), (league::CURRENT_STAGE, 4, 0xffffffff), (league::CLUB_COUNT, 2, 0x0), (league::STAGE_INDEX, 1, 0xff), (league::ACTIVE, 4, 0x0), (league::DISCIPLINE_RULE, 1, 0xff), (league::IMPORTANCE, 1, 0xf), (league::TABLE, 4, 0x0), (league::LEAGUE_FLAGS, 2, 0x28), (con_merc_cup::DRAWN_CLUBS, 4, 0x0)];

/// The Mercosur Cup at career creation: the year and the database competition, discipline rule 255 and importance 15;
/// a new cup has league flags 0x28, four stages, the rules and the statistics (see `build_mercosur_cup`).
/// Original: `ConMercCup::ConMercCup` (0x4c16f0, con_merc_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_con_merc_cup(this: usize, year: u32, record: usize) -> usize {
    build_mercosur_cup(League(this), year, record) // cqw5: the Mercosur Cup
}

/// Comparer for `construct_con_merc_cup`: only the original runs (it allocates and reads the save file), then the plan
/// is checked (`cup_constructor_plan_mismatch`, without the round table).
pub(crate) unsafe extern "thiscall" fn compare_construct_con_merc_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_CON_MERC_CUP)(this, year, record);
    let error = cup_constructor_plan_mismatch(this, r, year, record, 0x968b40, &MERCOSUR_CUP_CTOR_FIELDS, false);
    SCOREBOARD.record(error.is_none(), || format!("construct_con_merc_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The constructor of the MLS All-Star Game (`UsaMlsAllStars::UsaMlsAllStars(year, record)`), run once, at career
/// creation.
///
/// On the Cup base (Cup::Cup `exe::new_cup`, thiscall, ret 0): the competition record at +4, the vtable 0x9708f8, the
/// year (`set_year`), `current_round` 0, `stage_index` 0xff, `stage_count` 0, `current_stage` −1, `club_sides` 1 (clubs
/// of one nation), `discipline_index` 0xff and `run_by_stages` 0; substitutes, substitutions and the prize fund stay as
/// the base left them. If the saved state loads (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this -> thiscall, ret
/// 4; returns nonzero), returns this.
///
/// On a new cup, in order, each step with its error box (source 0xaa20a0) and returning this when it fails: the two
/// all-star teams (UsaMlsAllStars::set_up_teams 0x90a540, thiscall, ret 0; the box of line 0x44 when negative); the
/// round table (`build_cup_round_table_by_vtable`); the tie tree (Cup::map_fixture_tree 0x518790; the box of line 0x52
/// when negative); the statistics at +8 (the box of line 0x59 without them). No final ranking. Returns this.
unsafe fn build_mls_all_star_game(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_u32(this.addr() + 4, record as u32);
    write_u32(this.addr(), 0x9708f8);
    this.set_year(year as u16 as i16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_u32(this.addr() + cup::STAGE_COUNT, 0);
    write_u32(this.addr() + cup::CURRENT_STAGE, 0xffff_ffff);
    this.set_club_sides(1);
    this.set_discipline_index(0xff);
    this.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load_from_disk(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqw_5 mls all-star game new");
    let set_up_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x90a540usize); // UsaMlsAllStars::set_up_teams
    if set_up_teams(this.addr()) < 0 {
        show_error_at(0xaa20a0, 0x44);
        return this.addr();
    }
    build_cup_round_table_by_vtable(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    if map_fixture_tree(this.addr()) < 0 {
        show_error_at(0xaa20a0, 0x52);
        return this.addr();
    }
    if !create_comp_stats_at_8(this.addr()) {
        show_error_at(0xaa20a0, 0x59);
        return this.addr();
    }
    this.addr()
}

/// The fixed fields the MLS All-Star Game constructor writes (offset, width, value), checked by the comparer.
const MLS_ALL_STAR_GAME_CTOR_FIELDS: [(usize, u8, u32); 7] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::DISCIPLINE_INDEX, 1, 0xff), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The MLS All-Star Game at career creation: the year and the database competition; a new game has the two all-star
/// teams (clubs of one nation), discipline rule 255, no prize fund of its own, the rounds, the tie tree and the
/// statistics (see `build_mls_all_star_game`).
/// Original: `UsaMlsAllStars::UsaMlsAllStars` (0x90a150, usa_mls_all_stars.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_usa_mls_all_stars(this: usize, year: u32, record: usize) -> usize {
    build_mls_all_star_game(Cup(this), year, record) // cqw5: the MLS All-Star Game
}

/// Comparer for `construct_usa_mls_all_stars`: only the original runs (it allocates, draws and reads the save file),
/// then the plan is checked (`cup_constructor_plan_mismatch`).
pub(crate) unsafe extern "thiscall" fn compare_construct_usa_mls_all_stars(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_USA_MLS_ALL_STARS)(this, year, record);
    let error = cup_constructor_plan_mismatch(this, r, year, record, 0x9708f8, &MLS_ALL_STAR_GAME_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_usa_mls_all_stars({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

// ---------- batch 287: the constructors of the national team tournaments: the African Nations Cup, the Copa América and the Oceania Nations Cup (african_nations.cpp, goldcup.cpp, oceania_nations.cpp) ----------

/// The remainder of (year − 1996) by 2, signed (as the original's `and 0x80000001` with its sign adjustment).
fn nations_cup_year_parity(year: i16) -> i32 {
    (year as i32 - 0x7cc) % 2
}

/// The edition year: from `start`, goes up one year at a time while the remainder of (year − 1996) by 2 is not
/// `remainder`.
fn nations_cup_edition(start: i16, remainder: i32) -> i16 {
    let mut year = start;
    while nations_cup_year_parity(year) != remainder {
        year = year.wrapping_add(1);
    }
    year
}

/// A national team tournament built on the League base (the same body in the three constructors).
///
/// League::League (`exe::new_league`, mov ecx -> thiscall, ret 0); the competition record (`league::COMPETITION`), the
/// vtable, `club_sides` 4, `upper_division_id` and `lower_division_id` −1, `discipline_rule` 0, the edition `year`
/// (word) and `importance` 0xf. If the saved state loads (`exe::league_load_from_temp_file(this, 1)`: push 1; mov ecx ->
/// thiscall, ret 4; returns nonzero), returns this.
///
/// Otherwise: `league_flags` 40, `active` 0, `stage_index` 0xff, `current_stage` −1, `stage_count` = `stages` and the
/// stage vector (`exe::crt_malloc` of 4 × stages: push; add esp, 4 -> cdecl, unchecked); the rules (the routine
/// `rules`, thiscall, ret 0; when it returns 0, the error box `rows.0` of source `file` and this); the statistics
/// (operator new `exe::crt_operator_new` of 0x5ce and Comp_Stats::Comp_Stats `exe::new_comp_stats(new, this)`: push
/// this; mov ecx, new -> thiscall, ret 4) in `comp_stats` (null: the box `rows.1`). Returns this.
#[allow(clippy::too_many_arguments)]
unsafe fn build_nations_tournament(this: League, year: i16, record: usize, vtable: usize, stages: u32, rules: usize, file: usize, rows: (u32, u32)) -> usize {
    let set_up_rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(rules);
    exe::new_league(this);
    write_u32(this.addr() + league::COMPETITION, record as u32);
    write_u32(this.addr(), vtable as u32);
    this.set_club_sides(4);
    write_u32(this.addr() + league::UPPER_DIVISION_ID, 0xffff_ffff);
    write_u32(this.addr() + league::LOWER_DIVISION_ID, 0xffff_ffff);
    this.set_discipline_rule(0);
    this.set_year(year);
    this.set_importance(0xf);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    this.set_league_flags(0x28);
    write_u32(this.addr() + league::ACTIVE, 0);
    this.set_stage_index(stage_index::MAIN);
    write_u32(this.addr() + league::CURRENT_STAGE, 0xffff_ffff);
    write_u32(this.addr() + league::STAGE_COUNT, stages);
    write_u32(this.addr() + league::STAGES, exe::crt_malloc(stages * 4) as u32);
    if set_up_rules(this.addr()) == 0 {
        show_error_at(file, rows.0);
        return this.addr();
    }
    let p = exe::crt_operator_new(0x5ce);
    let e = if p != 0 { exe::new_comp_stats(p, this) } else { 0 };
    write_u32(this.addr() + league::COMP_STATS, e as u32);
    if e == 0 {
        show_error_at(file, rows.1);
    }
    this.addr()
}

/// The African Nations Cup at career creation: the edition is the year after the game's, moved to the first even year
/// (every two years from 1996: in a 2001 game, 2002); vtable 0x967270, 4 stages, the rules
/// AfricanNations::set_up_rules (0x401ab0) and the error boxes of lines 0x58 and 0x5f (`build_nations_tournament`).
/// The original's SEH frame only matters for exceptions in the statistics constructor.
/// Original: `AfricanNations::AfricanNations` (0x401000, african_nations.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_african_nations(this: usize, year: u32, record: usize) -> usize {
    covered!("cqw_10 african nations cup");
    let edition = nations_cup_edition((year as i16).wrapping_add(1), 0); // cqw10: the African Nations Cup of the next year, even
    build_nations_tournament(League(this), edition, record, 0x967270, 4, 0x401ab0, 0x9870f8, (0x58, 0x5f))
}

/// Shared comparer of the three constructors. They allocate the stages and the statistics, so they cannot run twice:
/// only the original runs, then the plan is checked: it returns this, the vtable, the record at +4
/// (`league::COMPETITION`), the edition year and `importance` 0xf.
unsafe fn compare_nations_tournament_ctor(name: &str, this: League, record: usize, vtable: usize, year: i16, o: impl FnOnce() -> usize) -> usize {
    let r = o();
    let read = this.year();
    let ok = r == this.addr() && read_usize(this.addr()) == vtable && read_usize(this.addr() + league::COMPETITION) == record && read == year && this.importance() == 0xf;
    SCOREBOARD.record(ok, || format!("{}({:#x}): year {} expected {}, vtable {:#x}", name, this.addr(), read, year, read_usize(this.addr())));
    r
}

/// Comparer for `construct_african_nations`.
pub(crate) unsafe extern "thiscall" fn compare_construct_african_nations(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_AFRICAN_NATIONS);
    compare_nations_tournament_ctor("construct_african_nations", League(this), record, 0x967270, nations_cup_edition((year as i16).wrapping_add(1), 0), || o(this, year, record))
}

/// The Copa América at career creation: the edition is the game's own year, moved to the first odd year (1997, 1999,
/// 2001...: in a 2001 game, July 2001); vtable 0x96b400, 3 stages, the rules GoldCup::set_up_rules (0x5e12d0) and the
/// error boxes of lines 0x56 and 0x5d (`build_nations_tournament`).
/// Original: `GoldCup::GoldCup` (0x5e0660, goldcup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_goldcup(this: usize, year: u32, record: usize) -> usize {
    covered!("cqw_10 copa america");
    let edition = nations_cup_edition(year as i16, 1); // cqw10: the Copa América in an odd year
    build_nations_tournament(League(this), edition, record, 0x96b400, 3, 0x5e12d0, 0x9cd640, (0x56, 0x5d))
}

/// Comparer for `construct_goldcup`.
pub(crate) unsafe extern "thiscall" fn compare_construct_goldcup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_GOLDCUP);
    compare_nations_tournament_ctor("construct_goldcup", League(this), record, 0x96b400, nations_cup_edition(year as i16, 1), || o(this, year, record))
}

/// The Oceania Nations Cup at career creation: the edition is the game's year, never before 2002, moved to the first
/// even year; vtable 0x96e0cc, 2 stages, the rules OceaniaNations::set_up_rules (0x799690) and the error boxes of lines
/// 0x57 and 0x5e (`build_nations_tournament`).
/// Original: `OceaniaNations::OceaniaNations` (0x7988b0, oceania_nations.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_oceania_nations(this: usize, year: u32, record: usize) -> usize {
    covered!("cqw_10 oceania nations cup");
    let edition = nations_cup_edition((year as i16).max(0x7d2), 0); // cqw10: the Oceania Nations Cup from 2002 on
    build_nations_tournament(League(this), edition, record, 0x96e0cc, 2, 0x799690, 0xa6a0ec, (0x57, 0x5e))
}

/// Comparer for `construct_oceania_nations`.
pub(crate) unsafe extern "thiscall" fn compare_construct_oceania_nations(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_OCEANIA_NATIONS);
    compare_nations_tournament_ctor("construct_oceania_nations", League(this), record, 0x96e0cc, nations_cup_edition((year as i16).max(0x7d2), 0), || o(this, year, record))
}

// ---------- batch 287: the Oceania World Cup qualifying: the ten nations and the final between the group winners; the second group of the Oceania Nations Cup (wc_oceania_league.cpp, oceania_nations.cpp) ----------

/// Whether the nation with id `id` belongs to Oceania: its record (`G_NATION_TABLE` + `nation::SIZE` × id) has a
/// continent (`nation::CONTINENT`) whose id is the Oceania key's (`KEY_CONTINENT_OCEANIA`).
unsafe fn nation_id_is_oceanian(id: usize) -> bool {
    let c = Nation(read_usize(G_NATION_TABLE).wrapping_add(id.wrapping_mul(nation::SIZE))).continent();
    c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_OCEANIA)
}

/// The Oceania World Cup qualifying entrants: ten places minus the exempt nations, filled with the Oceania national
/// teams in database order and sorted by reputation.
///
/// `WcOceaniaLeague::build_entrant_pool()`. +0xf3 (the pool count, dword) = 10, `wc_oceania_league_fields::EXEMPT_COUNT`
/// = 0, `games_per_club` = −1. The hosts of the World Cup two years later (HostCountry::find_hosts
/// `exe::hostcountry_find_hosts`([`G_HOST_COUNTRY`], [`KEY_CLUB_COMP_FIFA_WORLD_CUP`], `year()` + 2, &host0, &host1, 1):
/// 5 pushes; mov ecx -> thiscall, ret 0x14; without [`G_HOST_COUNTRY`], the error box of line 0x452 and no hosts): each
/// host from Oceania (`nation_id_is_oceanian`) is exempt (EXEMPT_COUNT + 1), up to the first −1. The last World Cup winner
/// (`exe::comp_util_get_latest_placed_club`(World Cup competition record, 0): 2 pushes; add esp, 8 -> cdecl), when its
/// nation is from Oceania and is not one of the hosts, is exempt too.
///
/// +0xf3 = 10 − exempt, and the pool list +0xef (`exe::crt_malloc` of 4 × n: push; add esp, 4 -> cdecl; out of memory, the
/// error box of line 0x481 and 0). Over the nations (`G_NATION_COUNT`), the Oceania ones except New Caledonia
/// (`KEY_NATION_NEW_CALEDONIA`) and Papua New Guinea (`KEY_NATION_PAPUA_NEW_GUINEA`), while there are places: the main
/// national team (`exe::database_get_national_team`: push; add esp, 4 -> cdecl) that is neither a host nor the winner.
/// Short of filling the places it returns 0 (unsorted); full, the list sorted by reputation (qsort `exe::crt_qsort`(list,
/// n, 4, 0x4b07d0): 4 pushes -> cdecl) and 1.
/// Original: `WcOceaniaLeague::build_entrant_pool` (0x91fa80, wc_oceania_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_build_entrant_pool(this: usize) -> u32 {
    let this = League(this);
    let mut hosts = [-1i32; 2];
    write_u32(this.addr() + 0xf3, 10);
    write_u8(this.addr() + wc_oceania_league_fields::EXEMPT_COUNT, 0);
    this.set_games_per_club(-1);
    let host_country = read_usize(G_HOST_COUNTRY);
    if host_country == 0 {
        show_error_at(0xad4afc, 0x452);
    } else {
        let year = (this.year() as u16).wrapping_add(2);
        let (a, b) = hosts.split_at_mut(1);
        let n = exe::hostcountry_find_hosts(host_country, read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP), year as u32, &mut a[0], &mut b[0], 1) as i8;
        let mut k = 0i8;
        while k < n {
            let s = hosts[k as usize];
            if s == -1 {
                break;
            }
            if nation_id_is_oceanian(s as usize) {
                let p = this.addr() + wc_oceania_league_fields::EXEMPT_COUNT;
                write_u8(p, read_u8(p).wrapping_add(1));
            }
            k += 1;
        }
    }
    let world_cup = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(0x6b));
    let champion = exe::comp_util_get_latest_placed_club(world_cup, 0);
    if champion != 0 {
        let nation = Club(champion).nation();
        if !nation.is_null() {
            let c = nation.continent();
            let id = read_usize(nation.addr()) as i32;
            if c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_OCEANIA) && id != hosts[0] && id != hosts[1] {
                let p = this.addr() + wc_oceania_league_fields::EXEMPT_COUNT;
                write_u8(p, read_u8(p).wrapping_add(1));
            }
        }
    }
    let n = 10 - read_u8(this.addr() + wc_oceania_league_fields::EXEMPT_COUNT) as i8 as i32;
    write_u32(this.addr() + 0xf3, n as u32);
    let list = exe::crt_malloc((n as u32).wrapping_mul(4));
    write_u32(this.addr() + 0xef, list as u32);
    if list == 0 {
        show_error_at(0xad4afc, 0x481);
        return 0;
    }
    let mut k = 0i32;
    let mut i = 0i16;
    while (i as i32) < read_usize(G_NATION_COUNT) as i32 {
        let nation = Nation(read_usize(G_NATION_TABLE).wrapping_add((i as isize as usize).wrapping_mul(nation::SIZE)));
        let continent = nation.continent();
        let id = read_usize(nation.addr()) as i32;
        let excluded = id == read_usize(KEY_NATION_NEW_CALEDONIA) as i32 || id == read_usize(KEY_NATION_PAPUA_NEW_GUINEA) as i32; // cqw11: without New Caledonia and Papua New Guinea
        if continent != 0 && read_usize(continent) == read_usize(KEY_CONTINENT_OCEANIA) && !excluded && k < read_usize(this.addr() + 0xf3) as i32 {
            let team = exe::database_get_national_team(nation);
            let id = read_usize(nation.addr()) as i32;
            if id != hosts[0] && id != hosts[1] && team != champion {
                covered!("cqw_11 qualifying nation");
                write_u32(read_usize(this.addr() + 0xef) + k as usize * 4, team as u32);
                k += 1;
            }
        }
        i = i.wrapping_add(1);
    }
    if k != read_usize(this.addr() + 0xf3) as i32 {
        return 0;
    }
    exe::crt_qsort(read_usize(this.addr() + 0xef), k as u32, 4, 0x4b07d0); // cqw11: the ten by reputation
    1
}

/// Comparer for `wc_oceania_league_build_entrant_pool`. The list is allocated, so it cannot run twice: only the
/// original runs, then the plan is checked: with 1, +0xf3 = 10 − EXEMPT_COUNT and the list holds Oceania national
/// teams (without New Caledonia and Papua New Guinea), with no repeats, by decreasing reputation.
pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_build_entrant_pool(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_OCEANIA_LEAGUE_BUILD_ENTRANT_POOL);
    let r = o(this);
    if r == 1 {
        let n = read_usize(this + 0xf3) as i32;
        let list = read_usize(this + 0xef);
        let v: Vec<usize> = (0..n.max(0) as usize).map(|k| read_usize(list + k * 4)).collect();
        let mut u = v.clone();
        u.sort_unstable();
        u.dedup();
        let ok = n == 10 - read_u8(this + wc_oceania_league_fields::EXEMPT_COUNT) as i8 as i32
            && u.len() == v.len()
            && v.iter().all(|&c| {
                let id = read_usize(Club(c).nation().addr());
                nation_id_is_oceanian(id) && id != read_usize(KEY_NATION_NEW_CALEDONIA) && id != read_usize(KEY_NATION_PAPUA_NEW_GUINEA)
            })
            && v.windows(2).all(|w| Club(w[0]).reputation() >= Club(w[1]).reputation());
        SCOREBOARD.record(ok, || format!("wc_oceania_league_build_entrant_pool({:#x}): {} nations {:x?}", this, n, v));
    }
    r
}

/// The Oceania qualifying final between the winners of the two groups, the first group's winner first.
///
/// `WcOceaniaLeague::create_final_stage()`. A list of 2 (`exe::crt_malloc` of 8; out of memory, the error box of line
/// 0x3ce and return) with the top club of the qualifying table (the first group; League::get_table_row
/// `exe::league_get_table_row(this, 0)`: push; mov ecx -> thiscall, ret 4) and that of stage 0 (the second group); the
/// round table of stage 1 (virtual `comp_vtable::CREATE_SCHEDULE`: push 0; push &extra; push &rounds; push 1; mov ecx
/// -> thiscall, ret 0x10); the stage (operator new `exe::crt_operator_new` of 0xb2 and Cup_Stage::Cup_Stage
/// `FN_CUP_STAGE_CUP_STAGE`, ret 0x3c: the qualifying, 2, the list, the rounds, the competition record, the table, the
/// year, 1, 4, the extra, 0x14, 0, 0, 0, 0; words zero-extended) in `stages()`[1]; the list and the table freed
/// (`exe::crt_free`); without a stage, the error box of line 0x3e3; with it, its first round set up (Cup::set_up_round
/// `exe::cup_set_up_round(stage, 0)`: push 0; mov ecx -> thiscall, ret 4). The original's SEH frame covers only the
/// stage's `new`.
/// Original: `WcOceaniaLeague::create_final_stage` (0x91f640, wc_oceania_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_create_final_stage(this: usize) {
    let this = League(this);
    let v = exe::crt_malloc(8);
    if v == 0 {
        return show_error_at(0xad4afc, 0x3ce);
    }
    covered!("cqw_11 oceania final");
    write_u32(v, read_u32(exe::league_get_table_row(this, 0))); // cqw11: the first group winner is listed first
    write_u32(v + 4, read_u32(exe::league_get_table_row(read_usize(this.stages()), 0)));
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = create_schedule(this.addr(), 1, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 2, v, rounds as u16 as u32, this.competition().addr(), table, this.year() as u16 as u32, 1, 4, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_u32(this.stages() + 4, stage as u32);
    exe::crt_free(v);
    exe::crt_free(table);
    let stage = read_usize(this.stages() + 4);
    if stage == 0 {
        return show_error_at(0xad4afc, 0x3e3);
    }
    exe::cup_set_up_round(stage, 0);
}

/// Comparer for `wc_oceania_league_create_final_stage`. The stage is allocated and its round set up, so it cannot run
/// twice: only the original runs, then the plan is checked: the stage in `stages()`[1] has the two group winners (the
/// first group's and stage 0's) in its team table (`cup::TEAM_TABLE`, 6-byte rows), in that order.
pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_create_final_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_OCEANIA_LEAGUE_CREATE_FINAL_STAGE);
    let league = League(this);
    let stages = league.stages();
    if stages == 0 || read_usize(stages) == 0 || league.table() == 0 || League(read_usize(stages)).table() == 0 {
        return o(this);
    }
    let plan = [read_usize(league.table()), read_usize(League(read_usize(stages)).table())];
    o(this);
    let stage = read_usize(league.stages() + 4);
    if stage == 0 {
        return;
    }
    let t = Cup(stage).team_table();
    let read = if t != 0 { [read_usize(t), read_usize(t + 6)] } else { [0, 0] };
    SCOREBOARD.record(read == plan, || format!("wc_oceania_league_create_final_stage({:#x}): {:x?} plan {:x?}", this, read, plan));
}

/// The second group of the Oceania Nations Cup: a league stage with the nations of places 3, 4 and 5 of the list.
///
/// `OceaniaNations::create_group_stage(stage)`. The nations 3, 4 and 5 of the list at +0xee in a vector
/// (`exe::crt_malloc` of 0xc; out of memory, the error box of line 0x331 and 0); the round table of `stage` (virtual
/// `comp_vtable::CREATE_SCHEDULE`); the stage (operator new `exe::crt_operator_new` of 0xee and
/// League_Stage::League_Stage `FN_LEAGUE_STAGE_LEAGUE_STAGE`, ret 0x58: the cup, 3, the vector, 1, the competition
/// record, the table, the rounds, `win_points`, `draw_points`, `match_category`, &`league::TIEBREAKS`,
/// &`league::PROMOTED_COUNT`, the year, the stage, the id, `importance`, 4, 0, `league_flags`, −1, 0, 2; words and bytes
/// zero-extended) in `stages()`[stage]; the vector and the table freed (`exe::crt_free`); without a stage, the error box
/// of line 0x347 and 0; with it, `current_stage` = stage, the group draw news (League::send_group_draw_news
/// `exe::league_send_group_draw_news` on the stage) and 1. The original's SEH frame covers only the stage's `new`.
/// Original: `OceaniaNations::create_group_stage` (0x799a60, oceania_nations.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn oceania_nations_create_group_stage(this: usize, stage: u32) -> u32 {
    let this = League(this);
    let teams = exe::crt_malloc(0xc);
    if teams == 0 {
        show_error_at(0xa6a0ec, 0x331);
        return 0;
    }
    for i in 0..3usize {
        write_u32(teams + i * 4, read_u32(read_usize(this.addr() + 0xee) + 0xc + i * 4));
    }
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut id) = (0i16, 0i16);
    let table = create_schedule(this.addr(), stage, &mut rounds, &mut id, 0);
    let p = exe::crt_operator_new(0xee);
    let group = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
            core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        construct(p, this.addr(), 3, teams, 1, this.competition().addr(), table, rounds as u16 as u32, this.win_points() as u32, this.draw_points() as u32, this.match_category() as u32,
                  this.addr() + league::TIEBREAKS, this.addr() + league::PROMOTED_COUNT, this.year() as u16 as u32, stage, id as u16 as u32, this.importance() as u32, 4, 0,
                  this.league_flags() as u16 as u32, -1, 0, 2)
    } else {
        0
    };
    write_u32(this.stages() + stage as usize * 4, group as u32);
    exe::crt_free(teams);
    exe::crt_free(table);
    let group = read_usize(this.stages() + stage as usize * 4);
    if group == 0 {
        show_error_at(0xa6a0ec, 0x347);
        return 0;
    }
    write_u32(this.addr() + league::CURRENT_STAGE, stage);
    exe::league_send_group_draw_news(group);
    1
}

/// Comparer for `oceania_nations_create_group_stage`. The stage is allocated and the news goes to the managers, so it
/// cannot run twice: only the original runs, then the plan is checked: it returns 1, the stage is in
/// `stages()`[stage], `current_stage` = stage and the nations 3, 4 and 5 of the list are in its table (in any order).
pub(crate) unsafe extern "thiscall" fn compare_oceania_nations_create_group_stage(this: usize, stage: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_OCEANIA_NATIONS_CREATE_GROUP_STAGE);
    let league = League(this);
    let list = read_usize(this + 0xee);
    if list == 0 || stage > 8 {
        return o(this, stage);
    }
    let mut plan: Vec<usize> = (3..6).map(|k| read_usize(list + k * 4)).collect();
    let r = o(this, stage);
    if r == 1 {
        let group = read_usize(league.stages() + stage as usize * 4);
        let t = if group != 0 { League(group).table() } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..3).map(|k| read_usize(t + k * league_table_row::SIZE)).collect() } else { Vec::new() };
        a.sort_unstable();
        plan.sort_unstable();
        SCOREBOARD.record(group != 0 && league.current_stage() == stage as usize && a == plan, || format!("oceania_nations_create_group_stage: {:x?} plan {:x?}", a, plan));
    }
    r
}

// ---------- batch 287: the constructors of the inter-continental cups: the CONCACAF Champions Cup and the Intercontinental Cup (con_champ.cpp, world_club_cup.cpp) ----------

/// The fields every inter-continental knockout cup sets: `current_round` 0, `stage_index` 0xff (the main
/// competition), `stage_count` 0 and `current_stage` −1.
unsafe fn intercontinental_cup_common_fields(this: Cup) {
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_u32(this.addr() + cup::STAGE_COUNT, 0);
    write_u32(this.addr() + cup::CURRENT_STAGE, 0xffff_ffff);
}

/// The common end of a new inter-continental cup; returns whether it got to the end with the statistics.
///
/// The round table (virtual `comp_vtable::CREATE_SCHEDULE`(−1, &`cup::ROUND_COUNT`, &`cup::STAGE_LABEL`, 0): push 0;
/// push; push; push −1; mov ecx -> thiscall, ret 0x10) in `round_table()`; the tie tree (Cup::map_fixture_tree
/// 0x518790, thiscall, ret 0; negative: the error box of line 0x4f of source `file` and false); the statistics
/// (operator new `exe::crt_operator_new` of 0x5ce and Comp_Stats::Comp_Stats `exe::new_comp_stats(new, this)`: push
/// this; mov ecx -> thiscall, ret 4) at +8 (null: the box of line 0x56 and false).
unsafe fn finish_intercontinental_cup(this: Cup, file: usize) -> bool {
    let create_schedule: unsafe extern "thiscall" fn(usize, i32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    write_u32(this.addr() + cup::ROUND_TABLE, create_schedule(this.addr(), -1, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0) as u32);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    if map_fixture_tree(this.addr()) < 0 {
        show_error_at(file, 0x4f);
        return false;
    }
    let p = exe::crt_operator_new(0x5ce);
    let e = if p != 0 { exe::new_comp_stats(p, this) } else { 0 };
    write_u32(this.addr() + 8, e as u32);
    if e == 0 {
        show_error_at(file, 0x56);
        return false;
    }
    true
}

/// Whether the saved state of the cup loads (Cup::load_from_disk 0x51fc00(this, 1): push 1; mov ecx -> thiscall, ret 4;
/// nonzero).
unsafe fn intercontinental_cup_loads(this: Cup) -> bool {
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    load_from_disk(this.addr(), 1) != 0
}

/// The CONCACAF Champions Cup of the game's year, at career creation: 8 clubs that come in later (the table starts
/// empty) and a 2,000,000 prize fund.
///
/// Cup::Cup (`exe::new_cup`, mov ecx -> thiscall, ret 0); the competition record at +4, the vtable 0x968aa0, the year
/// (`set_year`), the common fields (`intercontinental_cup_common_fields`), `club_sides` 2, `discipline_index` 0xff and
/// `run_by_stages` 0. If the saved state loads (`intercontinental_cup_loads`), returns this. Otherwise 8 teams
/// (`team_count`) in a zeroed team table of 6 bytes each (calloc `exe::crt_calloc(8, 6)`: push 6; push 8; add esp, 8 ->
/// cdecl; out of memory, the error box of line 0x41, source 0x9a7d30, and this), the prize fund 2,000,000 and the common
/// end (`finish_intercontinental_cup`). Returns this. The original's SEH frame only matters for exceptions.
/// Original: `ConChamp::ConChamp` (0x4c0c40, con_champ.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_con_champ(this: usize, year: u32, record: usize) -> usize {
    let this = Cup(this);
    exe::new_cup(this);
    covered!("cqw_12 concacaf champions cup");
    write_u32(this.addr() + 4, record as u32);
    write_u32(this.addr(), 0x968aa0);
    this.set_year(year as i16); // cqw12: the CONCACAF cup of the game's year
    intercontinental_cup_common_fields(this);
    this.set_club_sides(2);
    this.set_discipline_index(0xff);
    this.set_run_by_stages(0);
    if intercontinental_cup_loads(this) {
        return this.addr();
    }
    this.set_team_count(8);
    let t = exe::crt_calloc(8, 6);
    write_u32(this.addr() + cup::TEAM_TABLE, t as u32);
    if t == 0 {
        show_error_at(0x9a7d30, 0x41);
        return this.addr();
    }
    this.set_prize_fund(0x1e8480);
    finish_intercontinental_cup(this, 0x9a7d30);
    this.addr()
}

/// Shared comparer of the inter-continental cup constructors. They allocate the table, the tree and the statistics,
/// so they cannot run twice: only the original runs, then the plan is checked: it returns this, the vtable, the record
/// at +4 and the year (`cup::YEAR`).
unsafe fn compare_intercontinental_cup_ctor(name: &str, this: usize, year: u32, record: usize, vtable: usize, o: impl FnOnce() -> usize) -> usize {
    let r = o();
    let read = Cup(this).year();
    let ok = r == this && read_usize(this) == vtable && read_usize(this + 4) == record && read == year as i16;
    SCOREBOARD.record(ok, || format!("{}({:#x}): year {} requested {}, vtable {:#x}", name, this, read, year as i16, read_usize(this)));
    r
}

/// Comparer for `construct_con_champ`.
pub(crate) unsafe extern "thiscall" fn compare_construct_con_champ(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_CON_CHAMP);
    compare_intercontinental_cup_ctor("construct_con_champ", this, year, record, 0x968aa0, || o(this, year, record))
}

/// The body of a two-club inter-continental cup (the Intercontinental Cup; the Interamerican Cup, 0x632080, has the same
/// shape).
///
/// Cup::Cup (`exe::new_cup`, thiscall, ret 0); the year (`set_year`), the competition record at +4, the vtable, the
/// common fields (`intercontinental_cup_common_fields`), `club_sides` 2, +0x52 = `b52`, 7 `substitutes_named`,
/// `substitutions_allowed` = `substitutions`, `discipline_index` 0xff and `run_by_stages` 0. If the saved state loads,
/// returns this. Otherwise the two clubs (the routine `select_teams`, WorldClubCup::pick_qualifiers in the Intercontinental Cup, thiscall, ret 0; negative: the error box of line
/// 0x41 of source `file` and this), the common end (`finish_intercontinental_cup`) and, when it got to the end, the
/// predicted positions (Cup::rank_teams 0x5223a0, thiscall, ret 0). Returns this.
#[allow(clippy::too_many_arguments)]
unsafe fn build_two_club_cup(this: Cup, year: u32, record: usize, vtable: usize, b52: u8, substitutions: u8, select_teams: usize, file: usize) -> usize {
    let select_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(select_teams);
    let rank_teams: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5223a0usize); // Cup::rank_teams
    exe::new_cup(this);
    this.set_year(year as i16);
    write_u32(this.addr() + 4, record as u32);
    write_u32(this.addr(), vtable as u32);
    intercontinental_cup_common_fields(this);
    this.set_club_sides(2);
    write_u8(this.addr() + 0x52, b52);
    this.set_substitutes_named(7);
    this.set_substitutions_allowed(substitutions);
    this.set_discipline_index(0xff);
    this.set_run_by_stages(0);
    if intercontinental_cup_loads(this) {
        return this.addr();
    }
    if select_teams(this.addr()) < 0 {
        show_error_at(file, 0x41);
        return this.addr();
    }
    if finish_intercontinental_cup(this, file) {
        rank_teams(this.addr());
    }
    this.addr()
}

/// The Intercontinental Cup of the game's year, at career creation, with the two clubs (the European and the South
/// American winners) picked at once by WorldClubCup::pick_qualifiers (0x92b8b0): vtable 0x971250, +0x52 = 3, 3
/// substitutions, error boxes of source 0xad5848 (`build_two_club_cup`). The original's SEH frame only matters for
/// exceptions.
/// Original: `WorldClubCup::WorldClubCup` (0x92b4b0, world_club_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_world_club_cup(this: usize, year: u32, record: usize) -> usize {
    covered!("cqw_12 intercontinental cup");
    let year = year; // cqw12: the Intercontinental Cup of the game's year
    build_two_club_cup(Cup(this), year, record, 0x971250, 3, 3, 0x92b8b0, 0xad5848)
}

/// Comparer for `construct_world_club_cup`.
pub(crate) unsafe extern "thiscall" fn compare_construct_world_club_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_WORLD_CLUB_CUP);
    compare_intercontinental_cup_ctor("construct_world_club_cup", this, year, record, 0x971250, || o(this, year, record))
}

// ---------- batch 287: the constructors of the Asian club cups: the Asian Club Championship and the Asian Cup Winners' Cup (asia_club_champ.cpp, asia_cup_winner.cpp) ----------

/// Creates the cup's statistics at +8 (operator new `exe::crt_operator_new` of 0x5ce and Comp_Stats::Comp_Stats
/// `exe::new_comp_stats(new, this)`: push this; mov ecx, new -> thiscall, ret 4); when null, the error box `row` of
/// source `file` and false.
unsafe fn asian_club_cup_comp_stats(this: Cup, file: usize, row: u32) -> bool {
    let p = exe::crt_operator_new(0x5ce);
    let e = if p != 0 { exe::new_comp_stats(p, this) } else { 0 };
    write_u32(this.addr() + 8, e as u32);
    if e == 0 {
        show_error_at(file, row);
        return false;
    }
    true
}

/// The cup's round table (virtual `comp_vtable::CREATE_SCHEDULE`(−1, &`cup::ROUND_COUNT`, &`cup::STAGE_LABEL`, 0):
/// push 0; push; push; push −1; mov ecx -> thiscall, ret 0x10) in `round_table()`.
unsafe fn asian_club_cup_round_table(this: Cup) {
    let create_schedule: unsafe extern "thiscall" fn(usize, i32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    write_u32(this.addr() + cup::ROUND_TABLE, create_schedule(this.addr(), -1, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0) as u32);
}

/// The routines and the error-box lines of one of the two 32-club Asian cups.
struct AsianClubCupParts {
    vtable: usize,
    stages: u32,
    file: usize,
    select_qualified: usize,
    build_team_table: usize,
    create_stage: usize,
    predicted: usize,
    rows: [u32; 4],
}

/// The body of the two 32-club Asian cups (0x40aae0 and 0x40cbe0).
///
/// Cup::Cup (`exe::new_cup`, thiscall, ret 0); the competition record at +4, the vtable, the year (`set_year`),
/// `current_round` 0, `stage_index` 0xff, `current_stage` −1, `stage_count` = stages, `comp_type` 1 (cup), `club_sides`
/// 2, +0x1c = +0x20 = −1 (the upper and lower division ids of the Comp base), the qualified club list
/// (`asia_club_champ_fields::QUALIFIED_CLUBS`) 0, `discipline_index` 0x29, 5 `substitutes_named` and 3
/// `substitutions_allowed`. If the saved state loads (Cup::load_from_disk 0x51fc00(this, 1): push 1; mov ecx ->
/// thiscall, ret 4; nonzero), returns this.
///
/// Otherwise: the stage vector (`exe::crt_malloc` of 4 × stages; null when stages ≤ 0), `importance_bias` 0xf,
/// `run_by_stages` 0, the 32 qualified clubs (`select_qualified`; 0: the box `rows[0]`), the team table
/// (`build_team_table`; 0: the box `rows[1]`), the prize fund 4,000,000, the round table (`asian_club_cup_round_table`),
/// the statistics (the box `rows[2]`), the zone cup stage (`create_stage`), the qualified list freed (`exe::crt_free`)
/// and zeroed, the tie tree (Cup::map_fixture_tree 0x518790; 0: the box `rows[3]`) and the predicted positions
/// (`predicted`). Each failing step returns this; all the routines are thiscall, ret 0. Returns this.
unsafe fn build_asian_club_cup(this: Cup, year: i16, record: usize, c: &AsianClubCupParts) -> usize {
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    let routine = |a: usize| -> unsafe extern "thiscall" fn(usize) -> u32 { core::mem::transmute(a) };
    exe::new_cup(this);
    write_u32(this.addr() + 4, record as u32);
    write_u32(this.addr(), c.vtable as u32);
    this.set_year(year);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_u32(this.addr() + cup::CURRENT_STAGE, 0xffff_ffff);
    write_u32(this.addr() + cup::STAGE_COUNT, c.stages);
    this.set_comp_type(1);
    this.set_club_sides(2);
    write_u32(this.addr() + 0x1c, 0xffff_ffff);
    write_u32(this.addr() + 0x20, 0xffff_ffff);
    write_u32(this.addr() + asia_club_champ_fields::QUALIFIED_CLUBS, 0);
    this.set_discipline_index(0x29);
    this.set_substitutes_named(5);
    this.set_substitutions_allowed(3);
    if load_from_disk(this.addr(), 1) != 0 {
        return this.addr();
    }
    let n = read_usize(this.addr() + cup::STAGE_COUNT) as i32;
    write_u32(this.addr() + cup::STAGES, if n > 0 { exe::crt_malloc((n as u32).wrapping_mul(4)) as u32 } else { 0 });
    this.set_importance_bias(0xf);
    this.set_run_by_stages(0);
    if routine(c.select_qualified)(this.addr()) == 0 {
        show_error_at(c.file, c.rows[0]);
        return this.addr();
    }
    if routine(c.build_team_table)(this.addr()) == 0 {
        show_error_at(c.file, c.rows[1]);
        return this.addr();
    }
    this.set_prize_fund(0x3d0900);
    asian_club_cup_round_table(this);
    if !asian_club_cup_comp_stats(this, c.file, c.rows[2]) {
        return this.addr();
    }
    routine(c.create_stage)(this.addr());
    let l = read_usize(this.addr() + asia_club_champ_fields::QUALIFIED_CLUBS);
    if l != 0 {
        exe::crt_free(l);
        write_u32(this.addr() + asia_club_champ_fields::QUALIFIED_CLUBS, 0);
    }
    if map_fixture_tree(this.addr()) == 0 {
        show_error_at(c.file, c.rows[3]);
        return this.addr();
    }
    routine(c.predicted)(this.addr());
    this.addr()
}

/// The Asian Club Championship: vtable 0x9674d4, 4 stages; AsiaClubChamp::select_qualified_clubs (0x40b950),
/// build_team_table (0x40b620), create_second_zone_cup_stage (0x40bec0) and rank_teams (0x40c720, the predicted
/// positions); error boxes of source 0x987a40 at lines 0x4d, 0x54, 0x63 and 0x73.
const ASIAN_CLUB_CHAMPIONSHIP_PARTS: AsianClubCupParts = AsianClubCupParts { vtable: 0x9674d4, stages: 4, file: 0x987a40, select_qualified: 0x40b950, build_team_table: 0x40b620, create_stage: 0x40bec0, predicted: 0x40c720, rows: [0x4d, 0x54, 0x63, 0x73] };
/// The Asian Cup Winners' Cup: vtable 0x967574, 2 stages; AsiaCupWinner::select_qualified_clubs (0x40d830),
/// create_team_entries (0x40d4d0), create_zone_stage (0x40ddf0) and rank_teams (0x40e3b0); error boxes of source
/// 0x987b60 at lines 0x4e, 0x55, 0x64 and 0x74.
const ASIAN_CUP_WINNERS_CUP_PARTS: AsianClubCupParts = AsianClubCupParts { vtable: 0x967574, stages: 2, file: 0x987b60, select_qualified: 0x40d830, build_team_table: 0x40d4d0, create_stage: 0x40ddf0, predicted: 0x40e3b0, rows: [0x4e, 0x55, 0x64, 0x74] };

/// The Asian Club Championship of the game's year, at career creation: 4 stages and a 4,000,000 prize fund; with nothing
/// saved, the 32 qualifiers, the team table, the rounds, the statistics, the second zone cup stage, the tie tree and the
/// predicted positions (`build_asian_club_cup` with `ASIAN_CLUB_CHAMPIONSHIP_PARTS`). The original's SEH frame only
/// matters for exceptions.
/// Original: `AsiaClubChamp::AsiaClubChamp` (0x40aae0, asia_club_champ.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_asia_club_champ(this: usize, year: u32, record: usize) -> usize {
    covered!("cqw_13 asian club championship");
    let year = year as i16; // cqw13: the Asian Club Championship of the game's year
    build_asian_club_cup(Cup(this), year, record, &ASIAN_CLUB_CHAMPIONSHIP_PARTS)
}

/// Shared comparer of the Asian club cup constructors. They allocate, draw and create stages, so they cannot run
/// twice: only the original runs, then the plan is checked: it returns this, the vtable, the record at +4 and the year
/// (`cup::YEAR`).
unsafe fn compare_asian_club_cup_ctor(name: &str, this: usize, year: u32, record: usize, vtable: usize, o: impl FnOnce() -> usize) -> usize {
    let r = o();
    let read = Cup(this).year();
    let ok = r == this && read_usize(this) == vtable && read_usize(this + 4) == record && read == year as i16;
    SCOREBOARD.record(ok, || format!("{}({:#x}): year {} requested {}, vtable {:#x}", name, this, read, year as i16, read_usize(this)));
    r
}

/// Comparer for `construct_asia_club_champ`.
pub(crate) unsafe extern "thiscall" fn compare_construct_asia_club_champ(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ASIA_CLUB_CHAMP);
    compare_asian_club_cup_ctor("construct_asia_club_champ", this, year, record, 0x9674d4, || o(this, year, record))
}

/// The Asian Cup Winners' Cup of the game's year, at career creation: 2 stages and a 4,000,000 prize fund, built like
/// the Asian Club Championship (`build_asian_club_cup` with `ASIAN_CUP_WINNERS_CUP_PARTS`). The original's SEH frame
/// only matters for exceptions.
/// Original: `AsiaCupWinner::AsiaCupWinner` (0x40cbe0, asia_cup_winner.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_asia_cup_winner(this: usize, year: u32, record: usize) -> usize {
    covered!("cqw_13 asian cup winners cup");
    let year = year as i16; // cqw13: the Asian Cup Winners' Cup of the game's year
    build_asian_club_cup(Cup(this), year, record, &ASIAN_CUP_WINNERS_CUP_PARTS)
}

/// Comparer for `construct_asia_cup_winner`.
pub(crate) unsafe extern "thiscall" fn compare_construct_asia_cup_winner(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ASIA_CUP_WINNER);
    compare_asian_club_cup_ctor("construct_asia_cup_winner", this, year, record, 0x967574, || o(this, year, record))
}

// ---------- 287a leva: ending a loan in France and Italy, the Australian loan deadline (france_rules.cpp, italy_rules.cpp, australia_rules.cpp) ----------

/// The refusal text of a loan that cannot end before the deadline of nation `key` (`exe::nation_get_nationality_adjective`
/// of `[G_NATION_TABLE] + [key] · nation::SIZE`, 100 bytes, form 10: push; add esp, 0x10 -> cdecl): the club that
/// asks is the player's own (`club_job()`): "cannot be sent back" (0x9cd030); otherwise "cannot be recalled" (0x9ccfbc).
unsafe fn loan_end_refusal_text(key: usize, club: usize, player: usize, msg: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut adjective = [0u8; 100];
    let nation = read_usize(G_NATION_TABLE).wrapping_add(read_usize(key).wrapping_mul(nation::SIZE));
    exe::nation_get_nationality_adjective(nation, adjective.as_mut_ptr() as usize, 100, 10);
    let format = if Staff(player).club_job().addr() == club { 0x9cd030 } else { 0x9ccfbc };
    text(msg, format, 100, 100, adjective.as_ptr());
}

/// Tells whether a loan with a French club can end (sent back or recalled): only on a date inside a French transfer
/// window.
///
/// `FRANCE_RULES::can_end_loan(club, player, msg, date, flag)`, the vtable slot `transfer_rules_vtable::CAN_END_LOAN`,
/// from `can_end_loan` (the loan screen, the AI loan request) and from `transfer_manager_can_loan_player` (each month
/// of the loan of a player owned by a French club, flag 1). Without a date, today (`G_CURRENT_DATE`). A French window
/// open on it (`exe::transfer_rules_find_open_window`, push 0; push date; ecx = rules): 1. Otherwise 0 and, with a
/// message, the text with the French adjective (`KEY_NATION_FRANCE`, `loan_end_refusal_text`). `flag` is not read.
/// Original: `FRANCE_RULES::can_end_loan` (0x5c1ed0, france_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn france_rules_can_end_loan(this: usize, club: usize, player: usize, msg: usize, date: usize, _flag: u32) -> u32 {
    let date = if date == 0 { G_CURRENT_DATE } else { date };
    if exe::transfer_rules_find_open_window(this, date, 0) != 0 {
        return 1; // mkc2: France, the window open
    }
    covered!("mkc_2 france window closed");
    if msg != 0 {
        loan_end_refusal_text(KEY_NATION_FRANCE, club, player, msg);
    }
    0
}

/// Tells whether a loan with an Italian club can end: in June (month 5) when asked by the loan check (flag), or on a date
/// inside one of the first two Italian windows (the third, free agents only, does not count).
///
/// `ITALY_RULES::can_end_loan(club, player, msg, date, flag)`, the vtable slot `transfer_rules_vtable::CAN_END_LOAN`,
/// from `can_end_loan` and from `transfer_manager_can_loan_player` (flag 1). Without a date, today
/// (`G_CURRENT_DATE`). With `flag` and the date's month (`exe::date_get_month`) June: 1. The date inside window 0 or
/// window 1 of the table at +4 (`exe::transfer_window_contains_date`: push window; push date; mov ecx, this): 1.
/// Otherwise 0 and, with a message, the text with the Italian adjective (`KEY_NATION_ITALY`, `loan_end_refusal_text`).
/// Original: `ITALY_RULES::can_end_loan` (0x6618f0, italy_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn italy_rules_can_end_loan(this: usize, club: usize, player: usize, msg: usize, date: usize, flag: u32) -> u32 {
    let date = if date == 0 { G_CURRENT_DATE } else { date };
    if flag != 0 && exe::date_get_month(date) == 5 {
        covered!("mkc_2 italy june");
        return 1; // mkc2: Italy, June
    }
    if exe::transfer_window_contains_date(this, date, read_usize(this + 4)) != 0 || exe::transfer_window_contains_date(this, date, read_usize(this + 4) + 0xc) != 0 {
        return 1;
    }
    covered!("mkc_2 italy windows closed");
    if msg != 0 {
        loan_end_refusal_text(KEY_NATION_ITALY, club, player, msg);
    }
    0
}

/// Side-by-side comparer of a `can_end_loan`: the return and the message text (the copy starts on an emptied message and
/// the original's text is put back). Not named `compare_can_end_loan`: that is the 0x8d0530 comparer, glob-imported into
/// `motor.rs`, and a local function of the same name shadows it in the `PROLOGOS` table (the detour of 0x8d0530 then
/// lands here with the wrong arguments and calls a garbage `o`).
unsafe fn rules_compare_can_end_loan(o: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32, r: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32, name: &str, this: usize, club: usize, player: usize, msg: usize, date: usize, flag: u32) -> u32 {
    let ro = o(this, club, player, msg, date, flag);
    let to = message_text(msg);
    if msg != 0 {
        write_u8(msg, 0);
    }
    let rr = r(this, club, player, msg, date, flag);
    let tr = message_text(msg);
    if msg != 0 {
        core::ptr::copy_nonoverlapping(to.as_ptr(), msg as *mut u8, to.len());
    }
    SCOREBOARD.record(ro == rr && to == tr, || format!("{}(rules {:#x}, club {:#x}, player {:#x}, flag {}) original={} {:?} rust={} {:?}", name, this, club, player, flag, ro, String::from_utf8_lossy(&to), rr, String::from_utf8_lossy(&tr)));
    ro
}

unsafe extern "thiscall" fn compare_france_rules_can_end_loan(this: usize, club: usize, player: usize, msg: usize, date: usize, flag: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32>(IDX_FRANCE_RULES_CAN_END_LOAN);
    rules_compare_can_end_loan(o, france_rules_can_end_loan, "france_rules_can_end_loan", this, club, player, msg, date, flag)
}

unsafe extern "thiscall" fn compare_italy_rules_can_end_loan(this: usize, club: usize, player: usize, msg: usize, date: usize, flag: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32>(IDX_ITALY_RULES_CAN_END_LOAN);
    rules_compare_can_end_loan(o, italy_rules_can_end_loan, "italy_rules_can_end_loan", this, club, player, msg, date, flag)
}

/// Tells whether an Australian club can take a player on loan: only today inside an Australian transfer window
/// (`AUSTRALIA_TRANSFER_WINDOWS`, 11 June to 17 April).
///
/// `AUSTRALIA_RULES::can_loan_player(club, player, msg, reason)`, the vtable slot `transfer_rules_vtable::CAN_LOAN_PLAYER`,
/// from `transfer_manager_can_loan_player` and `handle_player_accepted_terms` with the borrowing club's nation. A window
/// open today (`exe::transfer_rules_find_open_window`, push 0; push `G_CURRENT_DATE`; mov ecx, this): 1. Otherwise, with
/// a message, "The deadline for loans to <adjective> clubs has passed." (0x987d38, the adjective of the rules' nation
/// `transfer_rules::NATION_ID`, form 2); the reason 9 (written even without a message) and 0.
/// Original: `AUSTRALIA_RULES::can_loan_player` (0x412dd0, australia_rules.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn australia_rules_can_loan_player(this: usize, _club: usize, _player: usize, msg: usize, reason: *mut u16) -> u32 {
    if exe::transfer_rules_find_open_window(this, G_CURRENT_DATE, 0) != 0 {
        return 1; // mkc2: Australia, the window open
    }
    covered!("mkc_2 australia deadline");
    if msg != 0 {
        let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        let mut adjective = [0u8; 100];
        exe::nation_get_nationality_adjective(read_usize(this + transfer_rules::NATION_ID), adjective.as_mut_ptr() as usize, 100, 2);
        text(msg, 0x987d38, 100, 100, adjective.as_ptr());
    }
    if !reason.is_null() {
        *reason = 9;
    }
    0
}

unsafe extern "thiscall" fn compare_australia_rules_can_loan_player(this: usize, club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(IDX_AUSTRALIA_RULES_CAN_LOAN_PLAYER);
    england_rules_compare_can_loan(o, australia_rules_can_loan_player, "australia_rules_can_loan_player", this, club, player, msg, reason)
}

// ---------- 287a leva: the loan periods of the Welsh and Russian rules, the Turkish loan deadline (wales_rules.cpp, russia_rules.cpp, turkey_rules.cpp) ----------

/// Returns the loan period of the rules whose loan lasts exactly to the season end: start and end both in the season end
/// month of the club's nation.
///
/// `WALES_RULES::get_loan_period(club, a2, start_month, start_year, end_month, end_year, player)`, the vtable slot
/// `transfer_rules_vtable::GET_LOAN_PERIOD` of 11 nations (Australia, Belgium, Croatia, France, Germany, Greece,
/// Holland, Republic of Ireland, Korea, Turkey and Wales), from `transfer_manager_get_loan_period` with the borrowing
/// club's nation. As the base period: the start is today's month (`exe::date_get_month` of `G_CURRENT_DATE`) + 1 of
/// this year (`G_CURRENT_DATE_YEAR`), past December 12 months back and a year on; the end month is the season end month
/// of the club's nation (`exe::contract_manager_season_end_month`: push club; mov ecx, `[G_CONTRACT_MANAGER]` ->
/// thiscall, ret 4); the start month (signed) below it: the end year is the start year, otherwise today's year + 1.
/// Then the start becomes the end (month and year): start = end, which `transfer_manager_can_loan_player` accepts at
/// once (without asking the owner's `CAN_END_LOAN`). `a2` and `player` are not read.
/// Original: `WALES_RULES::get_loan_period` (0x90f1a0, wales_rules.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn wales_rules_get_loan_period(_this: usize, club: usize, _a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, _player: usize) {
    *start_month = (exe::date_get_month(G_CURRENT_DATE) as i8).wrapping_add(1);
    *start_year = read_i16(G_CURRENT_DATE_YEAR);
    if *start_month > 0xb {
        *start_month = (*start_month).wrapping_sub(0xc);
        *start_year = (*start_year).wrapping_add(1);
    }
    let end = exe::contract_manager_season_end_month(read_usize(G_CONTRACT_MANAGER), club) as i8;
    *end_month = end;
    *end_year = if *start_month < end { *start_year } else { read_i16(G_CURRENT_DATE_YEAR).wrapping_add(1) };
    covered!("mkc_3 wales period");
    *start_month = *end_month; // mkc3: Wales, the start is the end
    *start_year = *end_year;
}

/// Returns the loan period of the rules whose loan starts three months ahead: from today's month + 3 to the season end
/// month of the club's nation.
///
/// `RUSSIA_RULES::get_loan_period(club, a2, start_month, start_year, end_month, end_year, player)`, the vtable slot
/// `transfer_rules_vtable::GET_LOAN_PERIOD` of Brazil, Poland and Russia, from `transfer_manager_get_loan_period` with
/// the borrowing club's nation. The start is today's month (`exe::date_get_month` of `G_CURRENT_DATE`) + 3 of this year
/// (`G_CURRENT_DATE_YEAR`), past December 12 months back and a year on; the end month is the season end month of the
/// club's nation (`exe::contract_manager_season_end_month`); the start month (signed) below it: the end year is the start
/// year, otherwise today's year + 1. A defect of the original: when the start wraps into next year (October to
/// December) and the season ends in an earlier month (January or February), the end lands before the start (the
/// Northern Irish copy of this routine clamps it). `a2` and `player` are not read.
/// Original: `RUSSIA_RULES::get_loan_period` (0x7ec8e0, russia_rules.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn russia_rules_get_loan_period(_this: usize, club: usize, _a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, _player: usize) {
    *start_month = (exe::date_get_month(G_CURRENT_DATE) as i8).wrapping_add(3); // mkc3: Russia, three months ahead
    *start_year = read_i16(G_CURRENT_DATE_YEAR);
    if *start_month > 0xb {
        *start_month = (*start_month).wrapping_sub(0xc);
        *start_year = (*start_year).wrapping_add(1);
    }
    let end = exe::contract_manager_season_end_month(read_usize(G_CONTRACT_MANAGER), club) as i8;
    *end_month = end;
    covered!("mkc_3 russia period");
    *end_year = if *start_month < end { *start_year } else { read_i16(G_CURRENT_DATE_YEAR).wrapping_add(1) };
}

/// Side-by-side comparer of a nation's `get_loan_period`: the four outputs.
unsafe fn nation_rules_compare_loan_period(o: unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize), r: unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize), name: &str, this: usize, club: usize, a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, player: usize) {
    let regions = [(start_month as usize, 1usize), (start_year as usize, 2), (end_month as usize, 1), (end_year as usize, 2)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club, a2, start_month, start_year, end_month, end_year, player), || r(this, club, a2, start_month, start_year, end_month, end_year, player));
    SCOREBOARD.record(equal, || format!("{}(rules {:#x}, club {:#x}) original={:x?} rust={:x?}", name, this, club, mo, mr));
}

unsafe extern "thiscall" fn compare_wales_rules_get_loan_period(this: usize, club: usize, a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, player: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize)>(IDX_WALES_RULES_GET_LOAN_PERIOD);
    nation_rules_compare_loan_period(o, wales_rules_get_loan_period, "wales_rules_get_loan_period", this, club, a2, start_month, start_year, end_month, end_year, player)
}

unsafe extern "thiscall" fn compare_russia_rules_get_loan_period(this: usize, club: usize, a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, player: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize)>(IDX_RUSSIA_RULES_GET_LOAN_PERIOD);
    nation_rules_compare_loan_period(o, russia_rules_get_loan_period, "russia_rules_get_loan_period", this, club, a2, start_month, start_year, end_month, end_year, player)
}

/// Tells whether a club under the Turkish rules can take a player on loan: only today inside one of the rules' transfer
/// windows.
///
/// `TURKEY_RULES::can_loan_player(club, player, msg, reason)`, the vtable slot `transfer_rules_vtable::CAN_LOAN_PLAYER` of
/// Turkey and the USA, from `transfer_manager_can_loan_player` and `handle_player_accepted_terms` with the borrowing
/// club's nation. A window open today (`exe::transfer_rules_find_open_window`, push 0; push `G_CURRENT_DATE`; mov ecx,
/// this): the player's main contract is looked up and kept in `G_CONTRACT_LOOKUP_RESULT`
/// (`transfer_manager_get_contract_stored`; the club it points to is computed and not used) and 1. Otherwise, with a
/// message, "The deadline for loans to <adjective> clubs has passed." (0x987d38, the adjective of the rules' nation
/// `transfer_rules::NATION_ID`, form 2); the reason 9 (written even without a message) and 0. `club` is not read.
/// Original: `TURKEY_RULES::can_loan_player` (0x902240, turkey_rules.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn turkey_rules_can_loan_player(this: usize, _club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    if exe::transfer_rules_find_open_window(this, G_CURRENT_DATE, 0) != 0 {
        transfer_manager_get_contract_stored(player);
        return 1; // mkc3: Turkey, the window open
    }
    covered!("mkc_3 turkey deadline");
    if msg != 0 {
        let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        let mut adjective = [0u8; 100];
        exe::nation_get_nationality_adjective(read_usize(this + transfer_rules::NATION_ID), adjective.as_mut_ptr() as usize, 100, 2);
        text(msg, 0x987d38, 100, 100, adjective.as_ptr());
    }
    if !reason.is_null() {
        *reason = 9;
    }
    0
}

unsafe extern "thiscall" fn compare_turkey_rules_can_loan_player(this: usize, club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(IDX_TURKEY_RULES_CAN_LOAN_PLAYER);
    england_rules_compare_can_loan(o, turkey_rules_can_loan_player, "turkey_rules_can_loan_player", this, club, player, msg, reason)
}
