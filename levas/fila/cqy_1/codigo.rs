// ---------- batch %%LEVA%%: the Série C schedule (the dates and matches of each round of the group and knockout stages) (bra_nat_third.cpp) ----------

/// The Brazilian Série C, the first-round groups (stages −128 to 6, league): 14 rounds (5 Jul to 7 Sep), 910 bytes, 28 operations.
const SERIE_C_SCHEDULE_FIRST_ROUND_OPS: [AsiaClubChampScheduleOp; 28] = [
    Cqp11D(0, 5, 7, 0, 6, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0xc, 7, 0, 6, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0xf, 7, 0, 2, 2, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 0x13, 7, 0, 6, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 0x1a, 7, 0, 6, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 0x1d, 7, 0, 2, 2, 0), Cqp11L(5, 0, -1, -1, -1, 0),
    Cqp11D(6, 2, 8, 0, 6, 1, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 5, 8, 0, 2, 2, 0),
    Cqp11L(7, 0, -1, -1, -1, 0), Cqp11D(8, 9, 8, 0, 6, 1, 0), Cqp11L(8, 0, -1, -1, -1, 0),
    Cqp11D(9, 0x10, 8, 0, 6, 1, 0), Cqp11L(9, 0, -1, -1, -1, 0), Cqp11D(0xa, 0x13, 8, 0, 2, 2, 0),
    Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 0x17, 8, 0, 6, 1, 0), Cqp11L(0xb, 0, -1, -1, -1, 0),
    Cqp11D(0xc, 0x1e, 8, 0, 6, 1, 0), Cqp11L(0xc, 0, -1, -1, -1, 0), Cqp11D(0xd, 7, 9, 0, 6, 1, 0),
    Cqp11L(0xd, 0, -1, -1, -1, 0),
];

/// The Brazilian Série C, stage 7 (knockout): 1 round (draw 8 Sep, match 14 Sep), 104 bytes, 19 operations.
const SERIE_C_SCHEDULE_SECOND_ROUND_OPS: [AsiaClubChampScheduleOp; 19] = [
    Cqp11S(0, 8, 9, 0, 0), Cqp11J(0, 0xe, 9, 0, 6, 1, 0), Cqp11W(0x7, 0x14), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 5), Cqp11W(0x18, 0x10), Cqp11W(0x1a, 8), Cqp11W(0x1c, 0x10), Cqp11W(0xf, 1),
    Cqp11W(0x11, 0), Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0),
    Cqp11X(0x60, 0), Cqp11X(0x64, 0),
];

/// The Brazilian Série C, the third-round groups (stages 8 and 9, league): 6 rounds (28 Sep to 2 Nov), 390 bytes, 12 operations.
const SERIE_C_SCHEDULE_THIRD_ROUND_OPS: [AsiaClubChampScheduleOp; 12] = [
    Cqp11D(0, 0x1c, 9, 0, 6, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 4, 0xa, 0, 6, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0xb, 0xa, 0, 6, 1, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 0x12, 0xa, 0, 6, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 0x19, 0xa, 0, 6, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 2, 0xb, 0, 6, 1, 0), Cqp11L(5, 0, -1, -1, -1, 0),
];

/// The Brazilian Série C, stage 10, the final: 1 round (draw 3 Nov, match 9 Nov), 104 bytes, 19 operations.
const SERIE_C_SCHEDULE_FINAL_OPS: [AsiaClubChampScheduleOp; 19] = [
    Cqp11S(0, 3, 0xb, 0, 0), Cqp11J(0, 9, 0xb, 0, 6, 1, 0), Cqp11W(0x7, 0x96), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0), Cqp11B(0x17, 5), Cqp11W(0x18, 2), Cqp11W(0x1a, 1), Cqp11W(0x1c, 2), Cqp11W(0xf, 1),
    Cqp11W(0x11, 1), Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0),
    Cqp11X(0x60, 0), Cqp11X(0x64, 0),
];

/// The stages of the round table of the Brazilian Série C (virtual `comp_vtable::CREATE_SCHEDULE`); the stage is the
/// signed byte of the argument.
///
/// Stages −128 to 6 (the first-round groups, league): 14 rounds from 5 Jul to 7 Sep, extra = stage + 0x420 (the word of
/// the sign-extended byte); stage 7 (knockout): 1 round, draw 8 Sep and match 14 Sep, extra 0; stages 8 and 9 (the
/// third-round groups, league): 6 rounds from 28 Sep to 2 Nov, extra = stage + 0x476; stage 10 (the final): 1 round, draw
/// 3 Nov and match 9 Nov, extra 0; any stage above 10 has no table (None). is_league is written only through a
/// non-null pointer; the rounds and the extra always.
fn serie_c_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    let s = stage as u8 as i8;
    match s {
        -128..=6 => Some(ScheduleStage { league: 1, rounds: 14, extra: (s as i16).wrapping_add(0x420), extra_check: false, size: 0x38e,
            row: 0xbb, ops: &SERIE_C_SCHEDULE_FIRST_ROUND_OPS, name: "cqy_1 serie c first round" }),
        7 => Some(ScheduleStage { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0x10d,
            ops: &SERIE_C_SCHEDULE_SECOND_ROUND_OPS, name: "cqy_1 serie c stage 7" }),
        8 | 9 => Some(ScheduleStage { league: 1, rounds: 6, extra: (s as i16).wrapping_add(0x476), extra_check: false, size: 0x186,
            row: 0x136, ops: &SERIE_C_SCHEDULE_THIRD_ROUND_OPS, name: "cqy_1 serie c third round" }),
        10 => Some(ScheduleStage { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0x15d,
            ops: &SERIE_C_SCHEDULE_FINAL_OPS, name: "cqy_1 serie c final" }),
        _ => None,
    }
}

/// The schedule of the Brazilian Série C: the first-round groups (stages up to 6) play 14 rounds from 5 Jul to 7 Sep;
/// stage 7 is a one-round knockout on 14 Sep; the third-round groups (stages 8 and 9) play 6 rounds from 28 Sep to
/// 2 Nov; stage 10 is the one-round final on 9 Nov; any other stage returns 0.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures, the group stages and the final (`bra_nat_third_create_final_stage`). For a stage with a
/// table (`serie_c_schedule_stage`) it writes the outputs (is_league when that pointer is not null, the rounds and the
/// extra: stage + 0x420 in the first-round groups, stage + 0x476 in the third-round ones, 0 in the knockouts),
/// allocates the round table (`exe::crt_malloc`, 0x41 bytes per league round, 0x68 per knockout round; out of memory,
/// the error box of bra_nat_third.cpp, lines 0xbb, 0x10d, 0x136 and 0x15d, and 0) and writes the date (day, month,
/// years after the season year `year()`, weekday and code) and the matches of each round, in the original's order
/// (`create_round_table`, the cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table,
/// or 0 without writing anything for a stage above 10.
/// Original: `BraNatThird::create_schedule` (0x42d620, bra_nat_third.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_nat_third_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), serie_c_schedule_stage(stage), rounds, extra, league, 0x98aba4) // cqy1: the Série C schedule
}

/// Comparer for `bra_nat_third_create_schedule`: side by side (`asia_club_champ_compare_round_table`), the outputs and
/// the bytes written to the two tables.
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_third_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    asia_club_champ_compare_round_table("bra_nat_third_create_schedule", stage, serie_c_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_nat_third_create_schedule(this, stage, r, e, l))
}
