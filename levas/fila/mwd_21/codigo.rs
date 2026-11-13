// ---------- batch %%LEVA%%: the Polish Super Cup (pol_super.cpp) ----------

/// The Polish Super Cup: discipline rule 0x1f, no prize fund, no predicted positions; the two clubs from
/// `PolSuper::select_teams` (0x7ccd10).
pub(crate) const POLISH_SUPER_CUP: EastNationCup = EastNationCup {
    vtable: 0x96e568,
    discipline: 0x1f,
    entrants: 0x7ccd10,
    prize: None,
    late_substitutes: false,
    stage_vector: false,
    rank: false,
    file: 0xa78648,
    rows: [0x41, 0x4f, 0x56],
};

/// The Polish Super Cup: one match between 2 clubs, drawn on 5/7 (weekday 5) and played on 18/7 (weekday 1, leg 2) at
/// venue 4 [inference: a neutral ground], tie rule 3, category 6, no prizes.
pub(crate) const POLISH_SUPER_CUP_ROUNDS: [FraSuperCupRound; 1] = [
    cql12r((5, 6, 0, 5), (0x12, 6, 0, 1, 2, 4), 0, (0, 3, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mwd21: the Polish Super Cup match
];

/// Rule: the Polish Super Cup at career creation: discipline rule 0x1f, 7 substitutes and 3 substitutions and, with no
/// saved state, its two clubs (the league champion and the cup winner), the round, the fixture tree and the statistics
/// (`construct_east_nation_cup`, without prize fund nor predicted positions).
/// Original: `PolSuper::PolSuper` (0x7cc920, pol_super.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_pol_super(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &POLISH_SUPER_CUP, "mwd_21 polish super cup new")
}

/// Rule: the Polish Super Cup is a single match between 2 clubs, drawn on 5/7 and played on 18/7 at a neutral ground
/// [inference: venue 4], tie rule 3; no prizes; category 6 (`super_cup_round_table`, with the word +0x11 = 0).
/// Original: `PolSuper::create_round_table` (0x7ccbd0, pol_super.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn pol_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    super_cup_round_table(Cup(this), stage, rounds, extra, out, &POLISH_SUPER_CUP_ROUNDS, 0xa78648, 0x97, "mwd_21 polish super cup round")
}

/// Rule: the Polish Super Cup season turnover: the finished edition's tables freed, its statistics archived, the year
/// + 1, the two clubs (`PolSuper::select_teams`), the round and the fixture tree; no predicted positions
/// (`usa_mls_all_stars_cup_start_new_season`; lines 0x135, 0x13c, 0x143 of pol_super.cpp).
/// Original: `PolSuper::start_new_season` (0x7ccf90, pol_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x7ccd10, 0xa78648, [0x135, 0x13c, 0x143], 1, false, "mwd_21 polish super cup turnover")
}

/// Comparer for `construct_pol_super` (only the original runs).
pub(crate) unsafe extern "thiscall" fn compare_construct_pol_super(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_east_nation_cup_ctor("construct_pol_super", Cup(this), year, record, &POLISH_SUPER_CUP, || o(this, year, record))
}

/// Comparer for `pol_super_create_round_table` (side by side, the written round compared).
pub(crate) unsafe extern "thiscall" fn compare_pol_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_cup_round_table("pol_super_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| pol_super_create_round_table(this, stage, r, e, s))
}

/// Comparer for `pol_super_start_new_season` (only the original; the year went up by one).
pub(crate) unsafe extern "thiscall" fn compare_pol_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_cup_new_season("pol_super_start_new_season", Cup(this), || o(this))
}
