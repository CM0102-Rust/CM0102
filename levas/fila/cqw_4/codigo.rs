// ---------- batch %%LEVA%%: the Série A, Série B, J1 and NSL schedules (the dates and matches of each round of the round-robin and knockout stages) (bra_nat_first.cpp, bra_nat_second.cpp, jap_j1.cpp, aus_nsl.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I3%%);
    asia_club_champ_compare_round_table("aus_nsl_create_schedule", stage, nsl_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| aus_nsl_create_schedule(this, stage, r, e, l))
}
