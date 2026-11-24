// ---------- cx11_4: the Serie C2/B table and rules (ita_ser_c2b.cpp) ----------

/// Serie C2/B has 18 clubs, those whose division is `KEY_CLUB_COMP_ITALIAN_SERIE_C2_B`, in database order; no prize
/// fund and no TV money (`ita_ser_a_fill_league_table`: out of memory the ita_ser_c2b.cpp box of line 0x1bd). Returns 1
/// when the table filled up, otherwise 0.
/// Original: `ItaSerC2b::init_team_list` (0x657f50, ita_ser_c2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2b_create_table(this: usize) -> u32 {
    ita_ser_a_fill_league_table(League(this), 0x12, 0x9ea75c, 0x1bd, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B).unwrap_or(false) as u32
}

/// Comparer for `ita_ser_c2b_create_table`: only the original runs (it allocates), against the plan
/// (`ita_ser_a_compare_league_table`).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2b_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    ita_ser_a_compare_league_table("ita_ser_c2b_create_table", League(this), 0x12, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B, None, || o(this))
}

/// Sets up the Serie C2/B rules (`serie_c2_rules`) and builds the main-stage round table through virtual +0x3c into
/// +0xba (`ita_ser_a_apply_league_rules`); returns 1. In the list of Italian divisions (`ita_ser_c2a_set_up_rules`)
/// the previous one is Serie C2/A and the next one Serie C2/C.
/// Original: `ItaSerC2b::set_up_rules` (0x658050, ita_ser_c2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2b_set_up_rules(this: usize) -> u32 {
    covered!("cx11_4 serie c2 b rules");
    let rules = serie_c2_rules(this, KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C); // cx11_4: the C2/B links
    ita_ser_a_apply_league_rules(League(this), &rules)
}

/// Comparer for `ita_ser_c2b_set_up_rules`: a dry run side by side on the written fields (`ita_ser_a_compare_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2b_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let rules = serie_c2_rules(this, KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C);
    ita_ser_a_compare_league_rules("ita_ser_c2b_set_up_rules", League(this), rules, || o(this), || ita_ser_c2b_set_up_rules(this))
}
