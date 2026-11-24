// ---------- cx15_9: the MLS schedule (usa_mls.cpp) ----------

/// The stages of the MLS round table (virtual +0x3c): -3, -2 and -1 the league of 12, 22 or 32 rounds (a league, 0x41
/// bytes per round), 2 the two-round play-off stage and 3 the one-round MLS Cup final (cups, 0x68 bytes per round, with
/// the extra 0xa0 and 0x417); every other stage has no table (`None`: returns 0 without writing anything).
pub(crate) fn usa_mls_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -3 => Some(ScheduleStage { league: 1, rounds: 12, extra: 0, extra_check: false, size: 0x30c, row: 0xb3, ops: &MLS_12_ROUNDS, name: "cx15_9 mls 12 rounds" }),
        -2 => Some(ScheduleStage { league: 1, rounds: 22, extra: 0, extra_check: false, size: 0x596, row: 0xeb, ops: &MLS_22_ROUNDS, name: "cx15_9 mls 22 rounds" }),
        -1 => Some(ScheduleStage { league: 1, rounds: 32, extra: 0, extra_check: false, size: 0x820, row: 0x141, ops: &MLS_32_ROUNDS, name: "cx15_9 mls 32 rounds" }),
        2 => Some(ScheduleStage { league: 0, rounds: 2, extra: 0xa0, extra_check: false, size: 0xd0, row: 0x1b6, ops: &MLS_PLAY_OFF_ROUNDS, name: "cx15_9 mls play-offs" }),
        3 => Some(ScheduleStage { league: 0, rounds: 1, extra: 0x417, extra_check: false, size: 0x68, row: 0x1f2, ops: &MLS_CUP_FINAL_ROUND, name: "cx15_9 mls cup final" }),
        _ => None,
    }
}

/// Builds the date template of an MLS stage (`usa_mls_schedule_stage` run by `create_round_table`): the outputs (the
/// league flag only through a non-null pointer, the rounds, the extra), the table (`exe::crt_malloc`; out of memory, the
/// error box of usa_mls.cpp at the stage's line and 0) and its dates and slots, in the year of the league
/// (`league::YEAR`). The MLS matches themselves are built by hand (`usa_mls_create_season_fixtures`): this table gives
/// the round dates.
/// Original: `UsaMls::create_schedule` (0x9072d0, usa_mls.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn usa_mls_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), usa_mls_schedule_stage(stage), rounds, extra, league, 0xaa1fb4) // "...\comp\leagues\usa_mls.cpp"
}

/// Comparer for `usa_mls_create_schedule`: side by side (`asia_club_champ_compare_round_table`).
pub(crate) unsafe extern "thiscall" fn compare_usa_mls_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    asia_club_champ_compare_round_table("usa_mls_create_schedule", stage, usa_mls_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| usa_mls_create_schedule(this, stage, r, e, l))
}

/// The 12-round MLS league stage (-3): from 29 February to 30 July, one date per round (weekday 5 and code 1; 10 May
/// and 20 July on weekday 2), each round with one open match slot.
pub(crate) const MLS_12_ROUNDS: [AsiaClubChampScheduleOp; 24] = [
    Cqp11D(0, 0x1d, 2, 0, 5, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x13, 3, 0, 5, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 3, 4, 0, 5, 1, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 0x11, 4, 0, 5, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 0x1f, 4, 0, 5, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 0xa, 5, 0, 2, 2, 0), Cqp11L(5, 0, -1, -1, -1, 0),
    Cqp11D(6, 0x15, 5, 0, 5, 1, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0xc, 6, 0, 5, 1, 0),
    Cqp11L(7, 0, -1, -1, -1, 0), Cqp11D(8, 2, 7, 0, 5, 1, 0), Cqp11L(8, 0, -1, -1, -1, 0),
    Cqp11D(9, 0x10, 7, 0, 5, 1, 0), Cqp11L(9, 0, -1, -1, -1, 0), Cqp11D(0xa, 0x14, 7, 0, 2, 1, 0),
    Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 0x1e, 7, 0, 5, 1, 0), Cqp11L(0xb, 0, -1, -1, -1, 0)
];

/// The 22-round MLS league stage (-2): from 22 February to 6 August (weekday 5 and code 1, the midweek rounds on weekday
/// 2 with code 2), each round with one open match slot.
pub(crate) const MLS_22_ROUNDS: [AsiaClubChampScheduleOp; 44] = [
    Cqp11D(0, 0x16, 2, 0, 5, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 5, 3, 0, 5, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0xc, 3, 0, 5, 1, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 0x1a, 3, 0, 5, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 0xa, 4, 0, 5, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 0x16, 4, 0, 2, 2, 0), Cqp11L(5, 0, -1, -1, -1, 0),
    Cqp11D(6, 0x18, 4, 0, 5, 1, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0x1c, 4, 0, 2, 2, 0),
    Cqp11L(7, 0, -1, -1, -1, 0), Cqp11D(8, 7, 5, 0, 5, 1, 0), Cqp11L(8, 0, -1, -1, -1, 0),
    Cqp11D(9, 0xe, 5, 0, 5, 1, 0), Cqp11L(9, 0, -1, -1, -1, 0), Cqp11D(0xa, 0x1c, 5, 0, 5, 1, 0),
    Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 2, 6, 0, 2, 2, 0), Cqp11L(0xb, 0, -1, -1, -1, 0),
    Cqp11D(0xc, 6, 6, 0, 5, 1, 0), Cqp11L(0xc, 0, -1, -1, -1, 0), Cqp11D(0xd, 0xa, 6, 0, 2, 2, 0),
    Cqp11L(0xd, 0, -1, -1, -1, 0), Cqp11D(0xe, 0x13, 6, 0, 5, 1, 0), Cqp11L(0xe, 0, -1, -1, -1, 0),
    Cqp11D(0xf, 0x17, 6, 0, 2, 2, 0), Cqp11L(0xf, 0, -1, -1, -1, 0), Cqp11D(0x10, 0x1a, 6, 0, 5, 1, 0),
    Cqp11L(0x10, 0, -1, -1, -1, 0), Cqp11D(0x11, 0x1d, 6, 0, 2, 2, 0), Cqp11L(0x11, 0, -1, -1, -1, 0),
    Cqp11D(0x12, 6, 7, 0, 2, 2, 0), Cqp11L(0x12, 0, -1, -1, -1, 0), Cqp11D(0x13, 9, 7, 0, 5, 1, 0),
    Cqp11L(0x13, 0, -1, -1, -1, 0), Cqp11D(0x14, 0x17, 7, 0, 5, 1, 0), Cqp11L(0x14, 0, -1, -1, -1, 0),
    Cqp11D(0x15, 6, 8, 0, 5, 1, 0), Cqp11L(0x15, 0, -1, -1, -1, 0)
];

/// The 32-round MLS league stage (-1), the regular season: from 22 February to 6 August (weekday 5 and code 1, the
/// midweek rounds on weekday 2 with code 2), each round with one open match slot.
pub(crate) const MLS_32_ROUNDS: [AsiaClubChampScheduleOp; 64] = [
    Cqp11D(0, 0x16, 2, 0, 5, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x1d, 2, 0, 5, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 5, 3, 0, 5, 1, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 0xc, 3, 0, 5, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 0x13, 3, 0, 5, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 0x1a, 3, 0, 5, 1, 0), Cqp11L(5, 0, -1, -1, -1, 0),
    Cqp11D(6, 3, 4, 0, 5, 1, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0xa, 4, 0, 5, 1, 0),
    Cqp11L(7, 0, -1, -1, -1, 0), Cqp11D(8, 0x11, 4, 0, 5, 1, 0), Cqp11L(8, 0, -1, -1, -1, 0),
    Cqp11D(9, 0x16, 4, 0, 2, 2, 0), Cqp11L(9, 0, -1, -1, -1, 0), Cqp11D(0xa, 0x18, 4, 0, 5, 1, 0),
    Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 0x1c, 4, 0, 2, 2, 0), Cqp11L(0xb, 0, -1, -1, -1, 0),
    Cqp11D(0xc, 0x1f, 4, 0, 5, 1, 0), Cqp11L(0xc, 0, -1, -1, -1, 0), Cqp11D(0xd, 7, 5, 0, 5, 1, 0),
    Cqp11L(0xd, 0, -1, -1, -1, 0), Cqp11D(0xe, 0xa, 5, 0, 2, 2, 0), Cqp11L(0xe, 0, -1, -1, -1, 0),
    Cqp11D(0xf, 0xe, 5, 0, 5, 1, 0), Cqp11L(0xf, 0, -1, -1, -1, 0), Cqp11D(0x10, 0x15, 5, 0, 5, 1, 0),
    Cqp11L(0x10, 0, -1, -1, -1, 0), Cqp11D(0x11, 0x1c, 5, 0, 5, 1, 0), Cqp11L(0x11, 0, -1, -1, -1, 0),
    Cqp11D(0x12, 2, 6, 0, 2, 2, 0), Cqp11L(0x12, 0, -1, -1, -1, 0), Cqp11D(0x13, 6, 6, 0, 5, 1, 0),
    Cqp11L(0x13, 0, -1, -1, -1, 0), Cqp11D(0x14, 0xc, 6, 0, 5, 1, 0), Cqp11L(0x14, 0, -1, -1, -1, 0),
    Cqp11D(0x15, 0x13, 6, 0, 5, 1, 0), Cqp11L(0x15, 0, -1, -1, -1, 0), Cqp11D(0x16, 0x1a, 6, 0, 5, 1, 0),
    Cqp11L(0x16, 0, -1, -1, -1, 0), Cqp11D(0x17, 0x1d, 6, 0, 2, 2, 0), Cqp11L(0x17, 0, -1, -1, -1, 0),
    Cqp11D(0x18, 2, 7, 0, 5, 1, 0), Cqp11L(0x18, 0, -1, -1, -1, 0), Cqp11D(0x19, 6, 7, 0, 2, 2, 0),
    Cqp11L(0x19, 0, -1, -1, -1, 0), Cqp11D(0x1a, 9, 7, 0, 5, 1, 0), Cqp11L(0x1a, 0, -1, -1, -1, 0),
    Cqp11D(0x1b, 0x10, 7, 0, 5, 1, 0), Cqp11L(0x1b, 0, -1, -1, -1, 0), Cqp11D(0x1c, 0x14, 7, 0, 2, 2, 0),
    Cqp11L(0x1c, 0, -1, -1, -1, 0), Cqp11D(0x1d, 0x17, 7, 0, 5, 1, 0), Cqp11L(0x1d, 0, -1, -1, -1, 0),
    Cqp11D(0x1e, 0x1e, 7, 0, 5, 1, 0), Cqp11L(0x1e, 0, -1, -1, -1, 0), Cqp11D(0x1f, 6, 8, 0, 5, 1, 0),
    Cqp11L(0x1f, 0, -1, -1, -1, 0)
];

/// The two-round MLS play-off stage (2): drawn on 7 and 21 August, played from 16 and 30 August, with the round fields
/// of the cup table (0x68 bytes per round).
pub(crate) const MLS_PLAY_OFF_ROUNDS: [AsiaClubChampScheduleOp; 38] = [
    Cqp11S(0, 7, 8, 0, 6), Cqp11J(0, 0x10, 8, 0, 5, 1, 0), Cqp11W(7, 0x78), Cqp11W(9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x230), Cqp11B(0x17, 5), Cqp11W(0x18, 8), Cqp11W(0x1a, 4), Cqp11W(0x1c, 8), Cqp11W(0xf, 0x30),
    Cqp11W(0x11, 0x20), Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 4), Cqp11X(0x5c, 0),
    Cqp11X(0x60, 0), Cqp11X(0x64, 0), Cqp11S(1, 0x15, 8, 0, 3), Cqp11J(1, 0x1e, 8, 0, 5, 1, 0), Cqp11W(0x6f, 0x82),
    Cqp11W(0x71, 1), Cqp11W(0x73, 0), Cqp11W(0x75, 0x230), Cqp11B(0x7f, 5), Cqp11W(0x80, 4), Cqp11W(0x82, 2),
    Cqp11W(0x84, 0), Cqp11W(0x77, 0x30), Cqp11W(0x79, 0x20), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2),
    Cqp11B(0x8a, 7), Cqp11X(0xc4, 0), Cqp11X(0xc8, 0), Cqp11X(0xcc, 0)
];

/// The one-round MLS Cup final (3): drawn on 8 September, played on 15 September.
pub(crate) const MLS_CUP_FINAL_ROUND: [AsiaClubChampScheduleOp; 18] = [
    Cqp11S(0, 8, 9, 0, 6), Cqp11J(0, 0xf, 9, 0, 6, 2, 4), Cqp11W(0x18, 2), Cqp11W(0x1c, 2), Cqp11W(7, 0x96),
    Cqp11W(9, 0), Cqp11W(0xb, 0), Cqp11W(0xd, 0x220), Cqp11B(0x17, 5), Cqp11W(0x1a, 1), Cqp11W(0xf, 0), Cqp11W(0x1e, 0),
    Cqp11B(0x20, 0), Cqp11B(0x21, 1), Cqp11B(0x22, 0), Cqp11X(0x5c, 0), Cqp11X(0x60, 0), Cqp11X(0x64, 0)
];
