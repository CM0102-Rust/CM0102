// ---------- batch %%LEVA%%: the Turkish Cup rounds and the Greek Super Cup (tur_cup.cpp, gre_super.cpp) ----------

/// The Turkish Cup: seven single-leg rounds from October to May, tie rule 0x83; a share of the fund per round won,
/// except in the round of 16 (nothing to the winners, 3,000 to each loser).
pub(crate) const TURKISH_CUP_ROUNDS: [FraSuperCupRound; 7] = [
    cql12r((0x11, 9, 0, 1), (0x19, 9, 0, 2, 2, 0), 0xa, (0, 0x83, 0), 4, (0x24, 0x12, 0x24, 0), (0, 1, 4), (0, 0xffffffff, 0)),
    cql12r((0x1f, 9, 0, 1), (8, 0xa, 0, 2, 2, 0), 0x14, (0, 0x83, 0), 4, (0x1c, 0xe, 0xa, 0x24), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((5, 0xb, 0, 1), (0xd, 0xb, 0, 2, 2, 0), 0x1e, (0, 0x83, 0), 4, (0x20, 0x10, 0x12, 0x2e), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((9, 0, 1, 1), (0x11, 0, 1, 2, 2, 0), 0x28, (0, 0x83, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0, 0xbb8)),
    cql12r((0xa, 0, 1, 2), (0x1f, 0, 1, 2, 2, 0), 0x78, (0, 0x83, 0), 6, (8, 4, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((6, 1, 1, 1), (0xe, 1, 1, 2, 2, 0), 0x82, (0, 0x83, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((0xf, 1, 1, 3), (2, 4, 1, 2, 2, 4), 0x96, (0, 0x83, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
];

/// Rule: the Turkish Cup has seven single-leg rounds (tie rule 0x83): the first round (36 teams, code 0xa; +0x22 = 4)
/// drawn 17/10, played 25/10 (weekday 2); the second round (28 teams, 10 new from entry 36) drawn 31/10, played 8/11;
/// the third round (32 teams, 18 new from entry 46) drawn 5/12, played 13/12; the round of 16 drawn 9/1, played 17/1 of
/// the next year, nothing to the winners and 3,000 to each loser; the quarter-finals (category 6) drawn 10/1, played
/// 31/1; the semi-finals drawn 6/2, played 14/2; the final drawn 15/2, played 2/5 at venue 4 [inference: a neutral
/// ground].
/// Original: `TurCup::create_round_table` (0x8fc670, tur_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn tur_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    fra_super_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &TURKISH_CUP_ROUNDS, 0xaa180c, 0x9d, "mwd_20 turkish cup rounds", |_| {})
}

/// The Greek Super Cup: one match between 2 clubs, drawn on 5/7 (weekday 5) and played on 2/8 (weekday 2, leg 2) at
/// venue 4 [inference: a neutral ground], tie rule 3, category 6, no prizes.
pub(crate) const GREEK_SUPER_CUP_ROUNDS: [FraSuperCupRound; 1] = [
    cql12r((5, 6, 0, 5), (2, 7, 0, 2, 2, 4), 0, (0, 3, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mwd20: the Greek Super Cup match
];

/// The one-round table of a national super cup (`fra_super_create_single_stage_round_table`) plus the round's word
/// +0x11 = 0, which the Greek and Polish super cups also write.
pub(crate) unsafe fn super_cup_round_table(this: Cup, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32, table: &[FraSuperCupRound; 1], file: usize, row: u32, branch: &'static str) -> usize {
    let t = fra_super_create_single_stage_round_table(this, stage, rounds, extra, out, table, file, row, branch, |_| {});
    if t != 0 {
        write_u16(t + 0x11, 0);
    }
    t
}

/// The Greek Super Cup: discipline rule 0x20, no prize fund, no predicted positions; the two clubs from
/// `GreSuper::select_teams` (0x5ec430).
pub(crate) const GREEK_SUPER_CUP: EastNationCup = EastNationCup {
    vtable: 0x96b6d0,
    discipline: 0x20,
    entrants: 0x5ec430,
    prize: None,
    late_substitutes: false,
    stage_vector: false,
    rank: false,
    file: 0x9cdb58,
    rows: [0x41, 0x4f, 0x56],
};

/// Rule: the Greek Super Cup at career creation: discipline rule 0x20, 7 substitutes and 3 substitutions and, with no
/// saved state, its two clubs (the league champion and the cup winner), the round, the fixture tree and the statistics
/// (`construct_east_nation_cup`, without prize fund nor predicted positions).
/// Original: `GreSuper::GreSuper` (0x5ec040, gre_super.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_gre_super(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &GREEK_SUPER_CUP, "mwd_20 greek super cup new")
}

/// Rule: the Greek Super Cup is a single match between 2 clubs, drawn on 5/7 and played on 2/8 at a neutral ground
/// [inference: venue 4], tie rule 3; no prizes; category 6.
/// Original: `GreSuper::create_round_table` (0x5ec2f0, gre_super.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn gre_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    super_cup_round_table(Cup(this), stage, rounds, extra, out, &GREEK_SUPER_CUP_ROUNDS, 0x9cdb58, 0x97, "mwd_20 greek super cup round")
}

/// Comparer for `tur_cup_create_round_table` (side by side, the written rounds compared).
pub(crate) unsafe extern "thiscall" fn compare_tur_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_cup_round_table("tur_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| tur_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `construct_gre_super` (only the original runs).
pub(crate) unsafe extern "thiscall" fn compare_construct_gre_super(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    compare_east_nation_cup_ctor("construct_gre_super", Cup(this), year, record, &GREEK_SUPER_CUP, || o(this, year, record))
}

/// Comparer for `gre_super_create_round_table` (side by side, the written round compared).
pub(crate) unsafe extern "thiscall" fn compare_gre_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    compare_cup_round_table("gre_super_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| gre_super_create_round_table(this, stage, r, e, s))
}
