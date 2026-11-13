// ---------- batch %%LEVA%%: the round tables of the Belgian and Danish cups and the Belgian Super Cup (bel_fa_cup.cpp, den_cup.cpp, bel_super.cpp) ----------

/// The Belgian Cup: 8 rounds.
pub(crate) const BEL_FA_CUP_ROUNDS: [FraCupCupRound; 8] = [
    cql11r((6, 7, 0, 6), (0xc, 7, 0, 5, 2, 0), 0x1e, (0, 0x401, 0), 4, (0x4c, 0x26, 0x4c, 0), (0, 1, 0), (0, 0, 0)),
    cql11r((0xd, 7, 0, 6), (0x13, 7, 0, 5, 2, 0), 0x28, (0, 0x403, 0), 4, (0x38, 0x1c, 0x12, 0x4c), (0, 1, 0), (0, 0, 0)),
    cql11r((0x14, 7, 0, 6), (0x1a, 7, 0, 5, 2, 0), 0x32, (0, 0x403, 0), 4, (0x1c, 0xe, 0, 0), (0, 1, 0), (0, 0, 0)),
    cql11r((0xf, 8, 0, 6), (5, 0xa, 0, 6, 2, 0), 0x3c, (0, 3, 0), 4, (0x20, 0x10, 0x12, 0x5e), (0, 1, 0), (0, 0xffffffff, 0)), // mwc12 bel_fa_cup round 3
    cql11r((6, 0xa, 0, 0), (0x1d, 0xa, 0, 2, 2, 0), 0x46, (0, 3, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x1e, 0xa, 0, 3), (0x18, 0, 1, 2, 2, 0), 0x78, (0, 2, 3), 6, (8, 4, 0, 0), (1, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x19, 0, 1, 3), (0x1c, 2, 1, 2, 2, 0), 0x82, (0, 0, 3), 6, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffffffff, 0)),
    cql11r((0xc, 3, 1, 3), (0x1b, 4, 1, 6, 2, 4), 0x96, (0, 2, 3), 6, (2, 1, 0, 0), (1, 1, 6), (0, 0xffffffff, 0)),
];

/// The Belgian Cup has 8 round(s): code 30: draw 6 Aug, match 12 Aug, 76 teams (76 new); code 40: draw 13 Aug, match 19
/// Aug, 56 teams (18 new); code 50: draw 20 Aug, match 26 Aug, 28 teams (0 new); code 60: draw 15 Sep, match 5 Nov, 32
/// teams (18 new); code 70: draw 6 Nov, match 29 Nov, 16 teams (0 new); code 120: draw 30 Nov, match 24 Jan (+1 year),
/// 8 teams (0 new); code 130: draw 25 Jan (+1 year), match 28 Mar (+1 year), 4 teams (0 new), two legs; code 150: draw
/// 12 Apr (+1 year), match 27 May (+1 year), 2 teams (0 new).
///
/// `BelFaCup::create_round_table(stage, &rounds, &extra, &out)` (`single_stage_round_table`): only stage -1 (the others
/// return 0 without writing); out = 0, rounds = the table size, extra = 0 and the table of 0x68-byte rounds
/// (`exe::crt_malloc`; out of memory, the error box of line 0xa9 and 0), each with its draw and match dates
/// (`cup_set_round_draw_date`, `cup_set_round_match_date`, cdecl) and its fields (`fra_cup_write_cup_round`).
/// Original: `BelFaCup::create_round_table` (0x41d310, bel_fa_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bel_fa_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    single_stage_round_table(Cup(this), stage, rounds, extra, out, &BEL_FA_CUP_ROUNDS, 0x989ae0, 0xa9, "mwc_12 bel fa cup round table")
}

/// Comparer for `bel_fa_cup_create_round_table`: side by side (`compare_cup_round_table`).
unsafe extern "thiscall" fn compare_bel_fa_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_cup_round_table("bel_fa_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| bel_fa_cup_create_round_table(this, stage, r, e, s))
}

/// The Danish Cup: 8 rounds.
pub(crate) const DEN_CUP_ROUNDS: [FraCupCupRound; 8] = [
    cql11r((1, 7, 0, 4), (0xc, 7, 0, 2, 2, 0), 0xa, (1, 3, 0), 4, (0x40, 0x20, 0x40, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0xe, 7, 0, 4), (0x13, 7, 0, 2, 2, 0), 0x14, (1, 3, 0), 4, (0x28, 0x14, 8, 0x40), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x15, 7, 0, 4), (0x1a, 7, 0, 2, 2, 0), 0x1e, (1, 3, 0), 4, (0x1c, 0xe, 8, 0x48), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x1c, 7, 0, 4), (5, 8, 0, 2, 2, 0), 0x28, (1, 3, 0), 4, (0x14, 0xa, 6, 0x50), (0, 1, 0), (0, 0xffffffff, 0)), // mwc12 den_cup round 3
    cql11r((7, 8, 0, 4), (0xc, 8, 0, 2, 2, 0), 0x32, (1, 3, 0), 4, (0x10, 8, 6, 0x56), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0xe, 8, 0, 4), (0x13, 8, 0, 2, 2, 0), 0x78, (1, 3, 0), 6, (8, 4, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x15, 8, 0, 4), (0x14, 2, 1, 2, 2, 0), 0x82, (0, 0, 3), 6, (4, 2, 0, 0), (0, 2, 7), (0, 0xffffffff, 0)),
    cql11r((0x1d, 2, 1, 4), (1, 5, 1, 3, 2, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
];

/// The Danish Cup has 8 round(s): code 10: draw 1 Aug, match 12 Aug, 64 teams (64 new); code 20: draw 14 Aug, match 19
/// Aug, 40 teams (8 new); code 30: draw 21 Aug, match 26 Aug, 28 teams (8 new); code 40: draw 28 Aug, match 5 Sep, 20
/// teams (6 new); code 50: draw 7 Sep, match 12 Sep, 16 teams (6 new); code 120: draw 14 Sep, match 19 Sep, 8 teams (0
/// new); code 130: draw 21 Sep, match 20 Mar (+1 year), 4 teams (0 new), two legs; code 150: draw 29 Mar (+1 year),
/// match 1 Jun (+1 year), 2 teams (0 new).
///
/// `DenCup::create_round_table(stage, &rounds, &extra, &out)` (`single_stage_round_table`): only stage -1 (the others
/// return 0 without writing); out = 0, rounds = the table size, extra = 0 and the table of 0x68-byte rounds
/// (`exe::crt_malloc`; out of memory, the error box of line 0xa5 and 0), each with its draw and match dates
/// (`cup_set_round_draw_date`, `cup_set_round_match_date`, cdecl) and its fields (`fra_cup_write_cup_round`).
/// Original: `DenCup::create_round_table` (0x550c70, den_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn den_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    single_stage_round_table(Cup(this), stage, rounds, extra, out, &DEN_CUP_ROUNDS, 0x9b8dfc, 0xa5, "mwc_12 den cup round table")
}

/// Comparer for `den_cup_create_round_table`: side by side (`compare_cup_round_table`).
unsafe extern "thiscall" fn compare_den_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_cup_round_table("den_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| den_cup_create_round_table(this, stage, r, e, s))
}

/// The Belgian Super Cup: one match.
pub(crate) const BEL_SUPER_ROUNDS: [FraCupCupRound; 1] = [
    cql11r((5, 6, 0, 5), (5, 7, 0, 5, 2, 0), 0, (8, 0x201, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mwc12 bel_super round 0
];

/// The Belgian Super Cup has 1 round(s): code 0: draw 5 Jul, match 5 Aug, 2 teams (2 new).
///
/// `BelSuper::create_round_table(stage, &rounds, &extra, &out)` (`single_stage_round_table`): only stage -1 (the others
/// return 0 without writing); out = 0, rounds = the table size, extra = 0 and the table of 0x68-byte rounds
/// (`exe::crt_malloc`; out of memory, the error box of line 0x9d and 0), each with its draw and match dates
/// (`cup_set_round_draw_date`, `cup_set_round_match_date`, cdecl) and its fields (`fra_cup_write_cup_round`).
/// Original: `BelSuper::create_round_table` (0x4211b0, bel_super.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bel_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    single_stage_round_table(Cup(this), stage, rounds, extra, out, &BEL_SUPER_ROUNDS, 0x989ccc, 0x9d, "mwc_12 bel super round table")
}

/// Comparer for `bel_super_create_round_table`: side by side (`compare_cup_round_table`).
unsafe extern "thiscall" fn compare_bel_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    compare_cup_round_table("bel_super_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| bel_super_create_round_table(this, stage, r, e, s))
}
