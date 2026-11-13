// ---------- batch %%LEVA%%: the round tables of the Dutch Super Cup and the German Cup (hol_super.cpp, ger_cup.cpp) ----------

/// The Dutch Super Cup: one match.
pub(crate) const HOL_SUPER_ROUNDS: [FraCupCupRound; 1] = [
    cql11r((5, 6, 0, 5), (8, 7, 0, 6, 1, 4), 0, (0, 0x83, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mwc13 hol_super round 0
];

/// The Dutch Super Cup has 1 round(s): code 0: draw 5 Jul, match 8 Aug, 2 teams (2 new).
///
/// `HolSuper::create_round_table(stage, &rounds, &extra, &out)` (`single_stage_round_table`): only stage -1 (the others
/// return 0 without writing); out = 0, rounds = the table size, extra = 0 and the table of 0x68-byte rounds
/// (`exe::crt_malloc`; out of memory, the error box of line 0x95 and 0), each with its draw and match dates
/// (`cup_set_round_draw_date`, `cup_set_round_match_date`, cdecl) and its fields (`fra_cup_write_cup_round`).
/// Original: `HolSuper::create_round_table` (0x5f8ad0, hol_super.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn hol_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    single_stage_round_table(Cup(this), stage, rounds, extra, out, &HOL_SUPER_ROUNDS, 0x9cefdc, 0x95, "mwc_13 hol super round table")
}

/// Comparer for `hol_super_create_round_table`: side by side (`compare_cup_round_table`).
unsafe extern "thiscall" fn compare_hol_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_cup_round_table("hol_super_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| hol_super_create_round_table(this, stage, r, e, s))
}

/// The German Cup 2001/02 (the final on 11 May 2002).
pub(crate) const GER_CUP_2001_ROUNDS: [FraCupCupRound; 6] = [
    cql11r((0xa, 6, 0, 5), (0x1a, 7, 0, 5, 1, 0), 0xa, (1, 0x403, 0), 4, (0x40, 0x20, 0x40, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x1b, 7, 0, 6), (0xb, 9, 0, 2, 2, 0), 0x14, (1, 0x403, 0), 4, (0x20, 0x10, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0xc, 0xa, 0, 6), (0x1d, 0xa, 0, 2, 2, 0), 0x1e, (1, 0x403, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x1e, 0xa, 0, 3), (0x14, 0xb, 0, 2, 2, 0), 0x78, (1, 3, 0), 4, (8, 4, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)), // mwc13 ger_cup round 3
    cql11r((0x15, 0xb, 0, 3), (7, 1, 1, 2, 2, 0), 0x82, (1, 3, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((8, 1, 1, 3), (0xb, 4, 1, 5, 1, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
];

/// The German Cup from 2002/03 (the final on 26 May).
pub(crate) const GER_CUP_LATER_ROUNDS: [FraCupCupRound; 6] = [
    cql11r((0xa, 6, 0, 5), (0x1a, 7, 0, 5, 1, 0), 0xa, (1, 0x403, 0), 4, (0x40, 0x20, 0x40, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x1b, 7, 0, 6), (0xb, 9, 0, 2, 2, 0), 0x14, (1, 0x403, 0), 4, (0x20, 0x10, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0xc, 0xa, 0, 6), (0x1d, 0xa, 0, 2, 2, 0), 0x1e, (1, 0x403, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x1e, 0xa, 0, 3), (0x14, 0xb, 0, 2, 2, 0), 0x78, (1, 3, 0), 4, (8, 4, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x15, 0xb, 0, 3), (7, 1, 1, 2, 2, 0), 0x82, (1, 3, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((8, 1, 1, 3), (0x1a, 4, 1, 5, 1, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
];

/// The German Cup has 6 round(s): code 10: draw 10 Jul, match 26 Aug, 64 teams (64 new); code 20: draw 27 Aug, match 11
/// Oct, 32 teams (0 new); code 30: draw 12 Nov, match 29 Nov, 16 teams (0 new); code 120: draw 30 Nov, match 20 Dec, 8
/// teams (0 new); code 130: draw 21 Dec, match 7 Feb (+1 year), 4 teams (0 new); code 150: draw 8 Feb (+1 year), match
/// 11 May (+1 year), 2 teams (0 new); in 2001/02 the final is on 11 May, in the following seasons on 26 May.
///
/// `GerCup::create_round_table(stage, &rounds, &extra, &out)` (`single_stage_round_table`): only stage -1 (the others
/// return 0 without writing); out = 0, rounds = the table size, extra = 0 and the table of 0x68-byte rounds
/// (`exe::crt_malloc`; out of memory, the error box of line 0xa6 and 0), each with its draw and match dates
/// (`cup_set_round_draw_date`, `cup_set_round_match_date`, cdecl) and its fields (`fra_cup_write_cup_round`).
/// Original: `GerCup::create_round_table` (0x5d8ee0, ger_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ger_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let table: &[FraCupCupRound] = if Cup(this).year() as u16 == 0x7d1 { &GER_CUP_2001_ROUNDS } else { &GER_CUP_LATER_ROUNDS };
    single_stage_round_table(Cup(this), stage, rounds, extra, out, table, 0x9cd3e0, 0xa6, "mwc_13 ger cup round table")
}

/// Comparer for `ger_cup_create_round_table`: side by side (`compare_cup_round_table`).
unsafe extern "thiscall" fn compare_ger_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_cup_round_table("ger_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| ger_cup_create_round_table(this, stage, r, e, s))
}
