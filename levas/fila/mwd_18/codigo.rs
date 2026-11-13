// ---------- batch %%LEVA%%: the Polish Cup and the Polish League Cup constructors (pol_cup.cpp, pol_lge_cup.cpp) ----------

/// The Polish Cup: discipline rule 0x1f (the Polish one), 600,000 prize fund.
pub(crate) const POLISH_CUP: EastNationCup = EastNationCup {
    vtable: 0x96e2c0,
    discipline: 0x1f,
    entrants: 0x7c8550,
    prize: Some(0x927c0), // mwd18: the Polish Cup prize fund
    late_substitutes: false,
    stage_vector: false,
    rank: true,
    file: 0xa78564,
    rows: [0x44, 0x52, 0x59],
};
/// The Polish League Cup: discipline rule 0x1f, 400,000 prize fund, the substitutes written after the round table.
pub(crate) const POLISH_LEAGUE_CUP: EastNationCup = EastNationCup {
    vtable: 0x96e414,
    discipline: 0x1f,
    entrants: 0x7cad30,
    prize: Some(0x61a80),
    late_substitutes: true,
    stage_vector: false,
    rank: true,
    file: 0xa785d4,
    rows: [0x3b, 0x4a, 0x51],
};

/// Rule: the Polish Cup at career creation: discipline rule 0x1f, 7 substitutes and 3 substitutions and, with no saved
/// state, the 52 entrants, a 600,000 prize fund, the seven rounds, the fixture tree, the statistics and the predicted
/// positions (`construct_east_nation_cup`).
/// Original: `PolCup::PolCup` (0x7c7d80, pol_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_pol_cup(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &POLISH_CUP, "mwd_18 polish cup new")
}

/// The Polish Cup: seven single-leg rounds from July to April, the quarter-finals and semi-finals over two legs 14
/// days apart and the final over two legs 7 days apart; 500,000 to the winner of the final.
pub(crate) const POLISH_CUP_ROUNDS: [FraSuperCupRound; 7] = [
    cql12r((6, 7, 0, 4), (9, 7, 0, 2, 2, 0), 0xa, (0, 0x403, 0), 4, (8, 4, 8, 0), (0, 1, 0), (0, 0, 0)),
    cql12r((0xb, 7, 0, 4), (0xd, 8, 0, 2, 2, 0), 0x14, (0, 3, 0), 4, (0x20, 0x10, 0x1c, 8), (0, 1, 0), (0, 0, 0)),
    cql12r((0xf, 8, 0, 4), (0x17, 8, 0, 5, 1, 0), 0x1e, (8, 0x403, 0), 4, (0x20, 0x10, 0x10, 0x24), (0, 1, 0), (0, 0, 0)),
    cql12r((0x1d, 8, 0, 4), (8, 0xa, 0, 2, 2, 0), 0x28, (0, 0, 3), 4, (0x10, 8, 0, 0), (0, 2, 0xe), (0, 0, 0)),
    cql12r((3, 2, 1, 4), (0xe, 2, 1, 2, 2, 0), 0x78, (0, 0, 3), 4, (8, 4, 0, 0), (0, 2, 0xe), (0, 0, 0)),
    cql12r((1, 3, 1, 4), (0xb, 3, 1, 2, 2, 0), 0x82, (0, 0, 3), 6, (4, 2, 0, 0), (0, 2, 7), (0, 0, 0)),
    cql12r((0x13, 3, 1, 3), (0x10, 4, 1, 1, 2, 0), 0x96, (0, 0, 3), 6, (2, 1, 0, 0), (0, 2, 7), (0, 0x7a120, 0)),
];

/// Rule: the Polish Cup has seven rounds: the preliminary round (8 teams, code 0xa, draw rule 0x403) drawn 6/8,
/// played 9/8 (weekday 2); the first round (32 teams, 28 new from entry 8) drawn 11/8, played 13/9; the second round
/// (32 teams, 16 new from entry 36, draw bits 8) drawn 15/9, played 23/9 (weekday 5); the round of 16 over two legs 14
/// days apart drawn 29/9, played 8/11; the quarter-finals over two legs drawn 3/3, played 14/3 of the next year; the
/// semi-finals over two legs drawn 1/4, played 11/4 (category 6); the final over two legs 7 days apart drawn 19/4,
/// played 16/5, 500,000 to the winner.
/// Original: `PolCup::create_round_table` (0x7c8040, pol_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn pol_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    fra_super_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &POLISH_CUP_ROUNDS, 0xa78564, 0x9d, "mwd_18 polish cup rounds", |_| {})
}

/// Rule: the Polish League Cup at career creation: discipline rule 0x1f and, with no saved state, its entrants, a
/// 400,000 prize fund, the rounds (7 substitutes and 3 substitutions set after them), the fixture tree, the statistics
/// and the predicted positions (`construct_east_nation_cup`).
/// Original: `PolLgeCup::PolLgeCup` (0x7ca4e0, pol_lge_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_pol_lge_cup(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &POLISH_LEAGUE_CUP, "mwd_18 polish league cup new")
}

/// Comparer for `construct_pol_cup` (only the original runs).
pub(crate) unsafe extern "thiscall" fn compare_construct_pol_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_east_nation_cup_ctor("construct_pol_cup", Cup(this), year, record, &POLISH_CUP, || o(this, year, record))
}

/// Comparer for `pol_cup_create_round_table` (side by side, the written rounds compared).
pub(crate) unsafe extern "thiscall" fn compare_pol_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_cup_round_table("pol_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| pol_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `construct_pol_lge_cup`.
pub(crate) unsafe extern "thiscall" fn compare_construct_pol_lge_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    compare_east_nation_cup_ctor("construct_pol_lge_cup", Cup(this), year, record, &POLISH_LEAGUE_CUP, || o(this, year, record))
}
