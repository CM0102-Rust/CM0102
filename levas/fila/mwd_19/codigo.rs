// ---------- batch %%LEVA%%: the Russian and Turkish cups: constructors and the Russian round table (rus_cup.cpp, tur_cup.cpp) ----------

/// The Russian Cup: discipline rule 0x27, 800,000 prize fund, the stage vector allocated from +0x2c.
pub(crate) const RUSSIAN_CUP: EastNationCup = EastNationCup {
    vtable: 0x96f124,
    discipline: 0x27,
    entrants: 0x7e99a0,
    prize: Some(0xc3500),
    late_substitutes: false,
    stage_vector: true,
    rank: true,
    file: 0xa7ac58,
    rows: [0x4e, 0x5c, 0x63],
};
/// The Turkish Cup: discipline rule 0x26, 600,000 prize fund.
pub(crate) const TURKISH_CUP: EastNationCup = EastNationCup {
    vtable: 0x97048c,
    discipline: 0x26, // mwd19: the Turkish Cup discipline rule
    entrants: 0x8fcb90,
    prize: Some(0x927c0),
    late_substitutes: false,
    stage_vector: false,
    rank: true,
    file: 0xaa180c,
    rows: [0x44, 0x52, 0x59],
};

/// Rule: the Russian Cup at career creation: discipline rule 0x27, 7 substitutes and 3 substitutions and, with no saved
/// state, no stage vector (+0x2c is 0), the 48 entrants, an 800,000 prize fund, the six rounds, the fixture tree, the
/// statistics and the predicted positions (`construct_east_nation_cup`).
/// Original: `RusCup::RusCup` (0x7e9210, rus_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_rus_cup(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &RUSSIAN_CUP, "mwd_19 russian cup new")
}

/// The Russian Cup: six single-leg rounds from June to May, a share of the fund per round won.
pub(crate) const RUSSIAN_CUP_ROUNDS: [FraSuperCupRound; 6] = [
    cql12r((0x17, 5, 0, 2), (0x1e, 5, 0, 2, 2, 0), 0x14, (0, 3, 0), 4, (0x20, 0x10, 0x20, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((0xa, 6, 0, 5), (0xc, 9, 0, 1, 2, 0), 0x1e, (8, 0x403, 0), 4, (0x20, 0x10, 0x10, 0x20), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((0xd, 9, 0, 2), (0xd, 0xa, 0, 5, 1, 0), 0x28, (0, 0x403, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((0xe, 0xa, 0, 6), (4, 3, 1, 1, 2, 0), 0x78, (0, 0x403, 0), 6, (8, 4, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((5, 3, 1, 2), (3, 4, 1, 2, 2, 0), 0x82, (0, 0x403, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((4, 4, 1, 3), (0x15, 4, 1, 1, 2, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
];

/// Rule: the Russian Cup has six single-leg rounds: the first round (32 teams, code 0x14) drawn 23/6, played 30/6; the
/// second round (32 teams, 16 new from entry 32: the Premier League [inference]; draw bits 8, rule 0x403) drawn 10/7,
/// played 12/10 (weekday 1); the round of 16 drawn 13/10, played 13/11 (weekday 5); the quarter-finals (category 6)
/// drawn 14/11, played 4/4 of the next year; the semi-finals drawn 5/4, played 3/5; the final drawn 4/5, played 21/5
/// at venue 4 [inference: a neutral ground]; a share of the fund per round won.
/// Original: `RusCup::create_round_table` (0x7e9530, rus_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rus_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    fra_super_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &RUSSIAN_CUP_ROUNDS, 0xa7ac58, 0xb1, "mwd_19 russian cup rounds", |_| {})
}

/// Rule: the Turkish Cup at career creation: discipline rule 0x26, 7 substitutes and 3 substitutions and, with no saved
/// state, the 64 entrants, a 600,000 prize fund, the seven rounds, the fixture tree, the statistics and the predicted
/// positions (`construct_east_nation_cup`).
/// Original: `TurCup::TurCup` (0x8fc3b0, tur_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_tur_cup(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &TURKISH_CUP, "mwd_19 turkish cup new")
}

/// Comparer for `construct_rus_cup` (only the original runs).
pub(crate) unsafe extern "thiscall" fn compare_construct_rus_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_east_nation_cup_ctor("construct_rus_cup", Cup(this), year, record, &RUSSIAN_CUP, || o(this, year, record))
}

/// Comparer for `rus_cup_create_round_table` (side by side, the written rounds compared).
pub(crate) unsafe extern "thiscall" fn compare_rus_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_cup_round_table("rus_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| rus_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `construct_tur_cup`.
pub(crate) unsafe extern "thiscall" fn compare_construct_tur_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    compare_east_nation_cup_ctor("construct_tur_cup", Cup(this), year, record, &TURKISH_CUP, || o(this, year, record))
}
