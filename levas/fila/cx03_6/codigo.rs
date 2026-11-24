// ---------- cx03_6: the round set-up of the Polish Cup and League Cup, and the Polish League Cup round table (pol_cup.cpp, pol_lge_cup.cpp) ----------

/// The rounds of the Polish Cup and the Turkish Cup (the same virtual `comp_vtable::SET_UP_RULES`): a prize fund of
/// 600,000 and the round table (`set_up_prize_rounds`). Returns 1.
/// Original: `PolCup::set_up_rounds` (0x7c8a50, pol_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_cup_set_up_rounds(this: usize) -> i32 {
    covered!("cx03_6 cup rounds with a 600,000 prize fund");
    set_up_prize_rounds(Cup(this), 600_000)
}

/// The rounds of the Polish League Cup: a prize fund of 400,000 and the round table (`set_up_prize_rounds`), then 7
/// substitutes named and 3 substitutions allowed. Returns 1.
/// Original: `PolLgeCup::set_up_rounds` (0x7cb190, pol_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_lge_cup_set_up_rounds(this: usize) -> i32 {
    let cup = Cup(this);
    let r = set_up_prize_rounds(cup, 400_000);
    covered!("cx03_6 polish league cup rounds");
    cup.set_substitutes_named(7);
    cup.set_substitutions_allowed(3);
    r
}

/// The Polish League Cup of 2001: an opening round of 4 clubs, then the six rounds of the other years shifted by one.
pub(crate) const POLISH_LEAGUE_CUP_2001_ROUNDS: [FraSuperCupRound; 7] = [
    cql12r((5, 6, 0, 2), (0xc, 6, 0, 2, 2, 0), 0xa, (0, 0, 1), 4, (4, 2, 4, 0), (0, 2, 3), (0, 0xfa0, 0x7d0)),
    cql12r((0x10, 6, 0, 6), (0x13, 6, 0, 2, 2, 0), 0x14, (0, 0, 1), 4, (0x12, 9, 0x10, 4), (0, 2, 3), (0, 0x1770, 0xbb8)),
    cql12r((0x17, 6, 0, 6), (0x1a, 6, 0, 2, 2, 0), 0x1e, (0, 0, 1), 4, (0x12, 9, 9, 0x14), (0, 2, 7), (0, 0x2710, 0x1388)),
    cql12r((6, 7, 0, 6), (0x1e, 7, 0, 2, 2, 0), 0x28, (0, 0, 1), 4, (0x10, 8, 7, 0x1d), (0, 2, 7), (0, 0xafc8, 0x61a8)),
    cql12r((0xa, 8, 0, 6), (0x15, 1, 1, 2, 2, 0), 0x78, (0, 0, 1), 6, (8, 4, 0, 0), (0, 2, 7), (0, 0xd6d8, 0x7530)),
    cql12r((4, 2, 1, 6), (0x15, 2, 1, 2, 2, 0), 0x82, (0, 0, 1), 6, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xfde8, 0x88b8)),
    cql12r((5, 3, 1, 3), (9, 4, 1, 2, 2, 0), 0x96, (0, 0, 1), 6, (2, 1, 0, 0), (0, 2, 8), (0, 0x1d4c0, 0xea60)),
];

/// The Polish League Cup of the other years: six two-legged rounds from June to April.
pub(crate) const POLISH_LEAGUE_CUP_ROUNDS: [FraSuperCupRound; 6] = [
    cql12r((0x10, 6, 0, 6), (0x13, 6, 0, 2, 2, 0), 0xa, (0, 0, 1), 4, (0x12, 9, 0x12, 0), (0, 2, 3), (0, 0x1770, 0xbb8)),
    cql12r((0x17, 6, 0, 6), (0x1a, 6, 0, 2, 2, 0), 0x14, (0, 0, 1), 4, (0x12, 9, 9, 0x12), (0, 2, 7), (0, 0x2710, 0x1388)),
    cql12r((6, 7, 0, 6), (0x1e, 7, 0, 2, 2, 0), 0x1e, (0, 0, 1), 4, (0x10, 8, 7, 0x1b), (0, 2, 7), (0, 0xafc8, 0x61a8)),
    cql12r((0xa, 8, 0, 6), (0x15, 1, 1, 2, 2, 0), 0x78, (0, 0, 1), 6, (8, 4, 0, 0), (0, 2, 7), (0, 0xd6d8, 0x7530)),
    cql12r((4, 2, 1, 6), (0x15, 2, 1, 2, 2, 0), 0x82, (0, 0, 1), 6, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xfde8, 0x88b8)),
    cql12r((5, 3, 1, 3), (9, 4, 1, 2, 2, 0), 0x96, (0, 0, 1), 6, (2, 1, 0, 0), (0, 2, 8), (0, 0x1d4c0, 0xea60)),
];

/// rule: the Polish League Cup has six two-legged rounds, all played on weekday 2: two rounds of 18 clubs in July (19/7
/// with all 18, 26/7 with 9 new ones), the round of 16 on 30/8 (7 new ones), the quarter-finals on 21/2, the
/// semi-finals on 21/3 and the final on 9/5 of the next year; in 2001 an opening round of 4 clubs on 12/7 comes first
/// and the first round has 16 new clubs; prizes from 4,000/2,000 per tie in the opening round to 120,000/60,000 in the
/// final; category 4 up to the round of 16, 6 after.
///
/// Outside stage −1 (the byte), 0; at −1 (`fra_super_create_single_stage_round_table`): `out` (if requested) = 0, the
/// rounds = 6, or 7 when the cup year (`cup::YEAR`) is 2001, the extra = 0, the table (`exe::crt_malloc` of 0x68 per
/// round; out of memory, the box of line 0x9c of pol_lge_cup.cpp and 0) with the rounds of
/// `POLISH_LEAGUE_CUP_2001_ROUNDS` or `POLISH_LEAGUE_CUP_ROUNDS`; then the first match day of the 26/7 round (index
/// rounds − 5: `exe::cup_set_round_match_day`(table, round, slot 0, any weekday, time and code, venue 0), cdecl). The
/// original writes each round's fields after its dates and patches the code and the new-club counts of the earlier
/// round while building the next one: the same bytes in the end.
/// Original: `PolLgeCup::create_round_table` (0x7ca7a0, pol_lge_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn pol_lge_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let cup = Cup(this);
    let table: &[FraSuperCupRound] = if cup.year() == 2001 { &POLISH_LEAGUE_CUP_2001_ROUNDS } else { &POLISH_LEAGUE_CUP_ROUNDS };
    let t = fra_super_create_single_stage_round_table(cup, stage, rounds, extra, out, table, 0xa785d4, 0x9c, "cx03_6 polish league cup round table", |_| {});
    if t != 0 {
        exe::cup_set_round_match_day(t, table.len() as i32 - 5, 0, -1, -1, -1, 0); // cx03_6: the round of 16's match day
    }
    t
}

pub(crate) unsafe extern "thiscall" fn compare_pol_cup_set_up_rounds(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    compare_set_up_prize_rounds("pol_cup_set_up_rounds", Cup(this), || o(this), || pol_cup_set_up_rounds(this))
}

pub(crate) unsafe extern "thiscall" fn compare_pol_lge_cup_set_up_rounds(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    compare_set_up_prize_rounds("pol_lge_cup_set_up_rounds", Cup(this), || o(this), || pol_lge_cup_set_up_rounds(this))
}

/// Comparer for `pol_lge_cup_create_round_table` (side by side, the written rounds compared).
pub(crate) unsafe extern "thiscall" fn compare_pol_lge_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    compare_cup_round_table("pol_lge_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| pol_lge_cup_create_round_table(this, stage, r, e, s))
}
