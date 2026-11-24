// ---------- cx12_3: the Serie C2/C constructor, table and rules (ita_ser_c2c.cpp) ----------

/// What the Serie C2/C constructor passes to `construct_italian_division`: the vtable, the league and 2 stages (the
/// promotion play-off and the relegation play-out), its rules, table and real 2001/02 calendar, the source file and the
/// box lines (the same as Serie C1/A's).
pub(crate) const SERIE_C2C_DIVISION: ItalianDivision = ItalianDivision {
    vtabela: 0x96c804,
    series: false,
    stages: 2,
    rules: 0x65c1e0,
    table: 0x65c0e0,
    fixed: 0x65c8e0, // cx12_3: the real 2001/02 C2/C calendar
    file: 0x9ea83c,
    rows: [0x49, 0x50, 0x5a, 0x63, 0x6a],
};

/// Builds the Italian Serie C2/C (`construct_italian_division`), as C2/A (`construct_ita_ser_c2a`): the league base, the
/// database record, the vtable, the year and discipline rule 0x11; with no saved state, stage 0xff, current stage −1, 2
/// stages, the rules, the table, the real 2001/02 calendar by club names, the fixtures, the order, the statistics and the
/// predicted positions. Each step that returns 0 shows its error box (ita_ser_c2c.cpp, lines 0x49, 0x50, 0x5a, 0x63 and
/// 0x6a) and stops. Returns this.
/// Original: `ItaSerC2c::ItaSerC2c` (0x65b4e0, ita_ser_c2c.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ita_ser_c2c(this: usize, year: u32, record: usize) -> usize {
    covered!("cx12_3 serie c2 c built");
    construct_italian_division(League(this), year, record, &SERIE_C2C_DIVISION)
}

/// Serie C2/C has 18 clubs, those whose division is `KEY_CLUB_COMP_ITALIAN_SERIE_C2_C`, in database order; no prize
/// fund and no TV money (`ita_ser_a_fill_league_table`: out of memory the ita_ser_c2c.cpp box of line 0x1bd). Returns 1
/// when the table filled up, otherwise 0.
/// Original: `ItaSerC2c::init_team_list` (0x65c0e0, ita_ser_c2c.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2c_create_table(this: usize) -> u32 {
    ita_ser_a_fill_league_table(League(this), 0x12, 0x9ea83c, 0x1bd, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C).unwrap_or(false) as u32
}

/// The rule writes of Serie C2/C: those of the other C2 groups (`serie_c2_rules`) with Serie C2/B above and nothing
/// below (`league::LOWER_DIVISION_ID` = −1): C2/C is the last of the Italian divisions.
unsafe fn serie_c2c_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = serie_c2_rules(this, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B);
    *v.last_mut().unwrap() = (league::LOWER_DIVISION_ID, 4, 0xffff_ffff); // cx12_3: nothing below Serie C2/C
    v
}

/// Sets up the Serie C2/C rules (`serie_c2c_rules`: 2 legs, 3 points for a win and 1 for a draw, tie-breaks 1 and 2,
/// 1 promoted, 4 in the promotion play-off, 4 in the relegation play-out and 1 relegated, below Serie C2/B and above
/// nothing) and builds the main-stage round table through virtual +0x3c into +0xba (`ita_ser_a_apply_league_rules`);
/// returns 1.
/// Original: `ItaSerC2c::set_up_rules` (0x65c1e0, ita_ser_c2c.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2c_set_up_rules(this: usize) -> u32 {
    covered!("cx12_3 serie c2 c rules");
    ita_ser_a_apply_league_rules(League(this), &serie_c2c_rules(this))
}

/// Comparer for `construct_ita_ser_c2c`: only the original runs (it allocates), checked by `compare_italian_constructor`.
pub(crate) unsafe extern "thiscall" fn compare_construct_ita_ser_c2c(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_italian_constructor("construct_ita_ser_c2c", League(this), year, record, &SERIE_C2C_DIVISION, || o(this, year, record))
}

/// Comparer for `ita_ser_c2c_create_table`: only the original runs (it allocates), against the plan
/// (`ita_ser_a_compare_league_table`).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2c_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    ita_ser_a_compare_league_table("ita_ser_c2c_create_table", League(this), 0x12, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C, None, || o(this))
}

/// Comparer for `ita_ser_c2c_set_up_rules`: a dry run side by side on the written fields (`ita_ser_a_compare_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2c_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    ita_ser_a_compare_league_rules("ita_ser_c2c_set_up_rules", League(this), serie_c2c_rules(this), || o(this), || ita_ser_c2c_set_up_rules(this))
}
