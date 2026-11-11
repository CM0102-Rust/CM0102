// ---------- batch %%LEVA%%: the J2 schedule (the dates and the games of each round of the round-robin stage) (jap_j2.cpp) ----------

/// J2, the league: 44 rounds (from 12 February to 26 October), 2860 bytes, 88 operations.
pub(crate) const J2_LEAGUE_ROUNDS: [AsiaClubChampScheduleOp; 88] = [
    Cqp11D(0, 0xc, 2, 0, 6, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x13, 2, 0, 6, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0x1a, 2, 0, 6, 1, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 2, 3, 0, 6, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 9, 3, 0, 6, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 0x10, 3, 0, 6, 1, 0), Cqp11L(5, 0, -1, -1, -1, 0),
    Cqp11D(6, 0x17, 3, 0, 6, 1, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0x1b, 3, 0, 3, 2, 0),
    Cqp11L(7, 0, -1, -1, -1, 0), Cqp11D(8, 0x1f, 3, 0, 6, 1, 0), Cqp11L(8, 0, -1, -1, -1, 0),
    Cqp11D(9, 3, 4, 0, 2, 2, 0), Cqp11L(9, 0, -1, -1, -1, 0), Cqp11D(0xa, 7, 4, 0, 6, 1, 0),
    Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 0xe, 4, 0, 6, 1, 0), Cqp11L(0xb, 0, -1, -1, -1, 0),
    Cqp11D(0xc, 0x15, 4, 0, 6, 1, 0), Cqp11L(0xc, 0, -1, -1, -1, 0), Cqp11D(0xd, 0x1b, 4, 0, 5, 1, 0),
    Cqp11L(0xd, 0, -1, -1, -1, 0), Cqp11D(0xe, 0x19, 5, 0, 6, 1, 0), Cqp11L(0xe, 0, -1, -1, -1, 0),
    Cqp11D(0xf, 2, 6, 0, 6, 1, 0), Cqp11L(0xf, 0, -1, -1, -1, 0), Cqp11D(0x10, 9, 6, 0, 6, 1, 0),
    Cqp11L(0x10, 0, -1, -1, -1, 0), Cqp11D(0x11, 0xf, 6, 0, 5, 1, 0), Cqp11L(0x11, 0, -1, -1, -1, 0),
    Cqp11D(0x12, 0x1e, 6, 0, 6, 1, 0), Cqp11L(0x12, 0, -1, -1, -1, 0), Cqp11D(0x13, 2, 7, 0, 2, 2, 0),
    Cqp11L(0x13, 0, -1, -1, -1, 0), Cqp11D(0x14, 6, 7, 0, 6, 1, 0), Cqp11L(0x14, 0, -1, -1, -1, 0),
    Cqp11D(0x15, 0xd, 7, 0, 6, 1, 0), Cqp11L(0x15, 0, -1, -1, -1, 0), Cqp11D(0x16, 0x14, 7, 0, 6, 1, 0),
    Cqp11L(0x16, 0, -1, -1, -1, 0), Cqp11D(0x17, 0x17, 7, 0, 2, 2, 0), Cqp11L(0x17, 0, -1, -1, -1, 0),
    Cqp11D(0x18, 0x1b, 7, 0, 6, 1, 0), Cqp11L(0x18, 0, -1, -1, -1, 0), Cqp11D(0x19, 3, 8, 0, 6, 1, 0),
    Cqp11L(0x19, 0, -1, -1, -1, 0), Cqp11D(0x1a, 6, 8, 0, 2, 2, 0), Cqp11L(0x1a, 0, -1, -1, -1, 0),
    Cqp11D(0x1b, 0xa, 8, 0, 6, 1, 0), Cqp11L(0x1b, 0, -1, -1, -1, 0), Cqp11D(0x1c, 0xd, 8, 0, 2, 2, 0),
    Cqp11L(0x1c, 0, -1, -1, -1, 0), Cqp11D(0x1d, 0x11, 8, 0, 6, 1, 0), Cqp11L(0x1d, 0, -1, -1, -1, 0),
    Cqp11D(0x1e, 0x18, 8, 0, 6, 1, 0), Cqp11L(0x1e, 0, -1, -1, -1, 0), Cqp11D(0x1f, 1, 9, 0, 6, 1, 0),
    Cqp11L(0x1f, 0, -1, -1, -1, 0), Cqp11D(0x20, 4, 9, 0, 2, 2, 0), Cqp11L(0x20, 0, -1, -1, -1, 0),
    Cqp11D(0x21, 7, 9, 0, 5, 1, 0), Cqp11L(0x21, 0, -1, -1, -1, 0), Cqp11D(0x22, 0xb, 9, 0, 2, 2, 0),
    Cqp11L(0x22, 0, -1, -1, -1, 0), Cqp11D(0x23, 0xf, 9, 0, 6, 1, 0), Cqp11L(0x23, 0, -1, -1, -1, 0),
    Cqp11D(0x24, 0x16, 9, 0, 6, 1, 0), Cqp11L(0x24, 0, -1, -1, -1, 0), Cqp11D(0x25, 0x1d, 9, 0, 6, 1, 0),
    Cqp11L(0x25, 0, -1, -1, -1, 0), Cqp11D(0x26, 1, 0xa, 0, 2, 2, 0), Cqp11L(0x26, 0, -1, -1, -1, 0),
    Cqp11D(0x27, 5, 0xa, 0, 6, 1, 0), Cqp11L(0x27, 0, -1, -1, -1, 0), Cqp11D(0x28, 8, 0xa, 0, 2, 2, 0),
    Cqp11L(0x28, 0, -1, -1, -1, 0), Cqp11D(0x29, 0xc, 0xa, 0, 6, 1, 0), Cqp11L(0x29, 0, -1, -1, -1, 0),
    Cqp11D(0x2a, 0x13, 0xa, 0, 6, 1, 0), Cqp11L(0x2a, 0, -1, -1, -1, 0), Cqp11D(0x2b, 0x1a, 0xa, 0, 6, 1, 0),
    Cqp11L(0x2b, 0, -1, -1, -1, 0),
];

/// The stages of the J2 round table: the round-robin stage (−1) (a league): 44 rounds, from 12 February to 26 October;
/// every other stage has no table (`None`: returns 0 without writing anything).
pub(crate) fn jap_j2_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 44, extra: 0, extra_check: false, size: 0xb2c, row: 0x9d,
            ops: &J2_LEAGUE_ROUNDS, name: "cqu_4 j2 league" }),
        _ => None,
    }
}

/// The J2 schedule: the round-robin stage (−1) (a league): 44 rounds, from 12 February to 26 October; each round has
/// the date (day, month, years after the season one, weekday and code) and the games, and the rest of the table is left
/// as allocated. Virtual `comp_vtable::CREATE_SCHEDULE` (`create_schedule(stage, &rounds, &extra, &is_league)`): the
/// stage of `jap_j2_schedule_stage` through the cup schedule machine (`create_round_table`: the outputs, the table of
/// 0x41 bytes per round from `exe::crt_malloc`, the error box of line 0x9d of jap_j2.cpp without memory, and the dates
/// and games of each round; the league's year at `league::YEAR` is read as `cup::YEAR`, the same word +0x40).
/// Original: `JapJ2::create_schedule` (0x664780, jap_j2.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn jap_j2_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), jap_j2_schedule_stage(stage), rounds, extra, league, 0x9eaedc)
}

/// Comparer for `jap_j2_create_schedule`: side by side (`asia_club_champ_compare_round_table`).
pub(crate) unsafe extern "thiscall" fn compare_jap_j2_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    asia_club_champ_compare_round_table("jap_j2_create_schedule", stage, jap_j2_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| jap_j2_create_schedule(this, stage, r, e, l))
}
