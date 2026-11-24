// ---------- cx11_3: the Serie C2/A table and rules (ita_ser_c2a.cpp) ----------

/// Serie C2/A has 18 clubs, those whose division is `KEY_CLUB_COMP_ITALIAN_SERIE_C2_A`, in database order; no prize
/// fund and no TV money (`ita_ser_a_fill_league_table`: out of memory the ita_ser_c2a.cpp box of line 0x1bd). Returns 1
/// when the table filled up, otherwise 0.
/// Original: `ItaSerC2a::create_table` (0x653a90, ita_ser_c2a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2a_create_table(this: usize) -> u32 {
    covered!("cx11_3 serie c2 a table");
    ita_ser_a_fill_league_table(League(this), 0x12, 0x9ea654, 0x1bd, KEY_CLUB_COMP_ITALIAN_SERIE_C2_A).unwrap_or(false) as u32
}

/// Comparer for `ita_ser_c2a_create_table`: only the original runs (it allocates), against the plan
/// (`ita_ser_a_compare_league_table`).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2a_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    ita_ser_a_compare_league_table("ita_ser_c2a_create_table", League(this), 0x12, KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, None, || o(this))
}

/// The rule writes of a Serie C2 group (offset, size, value): Serie C1/A's (`serie_c1a_rules`, checked instruction by
/// instruction) with the division above read from `above` and the one below from `below`: the common Italian rules
/// (`italian_common_rules`), 2 legs, 3 points for a win, tie-breaks 1 and 2 (the third code 0 ends the list), 1
/// promoted, 4 in the promotion play-off, 4 in the relegation play-out and 1 relegated.
pub(crate) unsafe fn serie_c2_rules(this: usize, above: usize, below: usize) -> Vec<(usize, u8, u32)> {
    let mut v = italian_common_rules(League(this));
    v.extend_from_slice(&[(league::LEGS, 2, 2), (league::WIN_POINTS, 1, 3)]); // cx11_3: the C2 gives 3 points per win
    v.extend_from_slice(&[(league::TIEBREAKS + 2, 1, 0), (league::PROMOTED_COUNT, 1, 1), (league::PROMOTION_PLAYOFF_COUNT, 1, 4),
                          (league::RELEGATION_PLAYOFF_COUNT, 1, 4), (league::RELEGATED_COUNT, 1, 1)]);
    v.extend_from_slice(&[(league::UPPER_DIVISION_ID, 4, read_usize(above) as u32), (league::LOWER_DIVISION_ID, 4, read_usize(below) as u32)]);
    v
}

/// Sets up the Serie C2/A rules (`serie_c2_rules`) and builds the main-stage round table through virtual +0x3c into
/// +0xba (`ita_ser_a_apply_league_rules`); returns 1. Above it the id of `KEY_CLUB_COMP_ITALIAN_SERIE_C1_B` and below
/// it the id of `KEY_CLUB_COMP_ITALIAN_SERIE_C2_B`: the Italian divisions are linked as one list (A, B, C1/A, C1/B,
/// C2/A, C2/B, C2/C; `league::LOWER_DIVISION_ID` the next, `league::UPPER_DIVISION_ID` the previous), which
/// `all_stages_complete` walks from the top.
/// Original: `ItaSerC2a::set_up_rules` (0x653b90, ita_ser_c2a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2a_set_up_rules(this: usize) -> u32 {
    covered!("cx11_3 serie c2 rules");
    ita_ser_a_apply_league_rules(League(this), &serie_c2_rules(this, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B))
}

/// Comparer for `ita_ser_c2a_set_up_rules`: a dry run side by side on the written fields (`ita_ser_a_compare_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2a_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let rules = serie_c2_rules(this, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B);
    ita_ser_a_compare_league_rules("ita_ser_c2a_set_up_rules", League(this), rules, || o(this), || ita_ser_c2a_set_up_rules(this))
}
