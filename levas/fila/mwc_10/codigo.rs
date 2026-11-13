// ---------- batch %%LEVA%%: the Eerste Divisie: the constructor, the rules and the table (hol_first.cpp) ----------

pub(crate) const HOL_FIRST_DIVISION: NationDivision = NationDivision {
    vtable: 0x96b86c,
    bench: None,
    discipline: discipline_rule::HOLLAND, // mwc10: the Dutch Eerste Divisie discipline rule
    stages: 6,
    set_up_rules: 0x5f6340,
    create_table: 0x5f6240,
    source_file: 0x9cee08,
    box_lines: [0x49, 0x50, 0x57, 0x60, 0x67],
};

/// The Dutch Eerste Divisie: discipline rule 15 (`HOLLAND`), 6 stages besides the league (the stage vector, malloc of
/// 24); with no saved state, the rules, the table, the fixtures, the order, the statistics and the predicted positions
/// (`construct_nation_division`).
/// Original: `HolFirst::HolFirst` (0x5f5490, hol_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_hol_first(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &HOL_FIRST_DIVISION, || {})
}

/// Comparer for `construct_hol_first`: only the original runs (`compare_nation_division_ctor`).
unsafe extern "thiscall" fn compare_construct_hol_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_nation_division_ctor("construct_hol_first", League(this), year, record, &HOL_FIRST_DIVISION, || o(this, year, record))
}

/// The rule writes of the Dutch Eerste Divisie (offset, size, value).
pub(crate) unsafe fn hol_first_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let _ = this;
    let mut v = vec![(0x3c, 2, 2), (0xc4, 1, 2), (0xc6, 1, 2), (0xc3, 1, 1), (0x42, 1, 1), (0xc5, 1, 1), (0xc7, 1, 0), (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0xf1, 1, 0), (0xf0, 1, 0), (0xef, 1, 0), (0xee, 1, 0), (0x4a, 1, 3), (0x1c, 4, read_usize(KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION) as u32), (0x20, 4, 0xffff_ffff), (0x49, 1, 7)];
    v.push((0xc2, 1, 3)); // mwc10: the Dutch Eerste Divisie points for a win
    v
}

/// Sets up the rules of the Dutch Eerste Divisie: 2 rounds, 3 points for a win and 1 for a draw; 1 go up, 0 to the
/// promotion play-off, 0 to the relegation play-off and 0 go down; above: `KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION`;
/// below: none; tie-breaker codes 1, 2, 0 (`+0xc5`..); 7 substitutes named and 3 substitutions; zeroes the class fields
/// `+0xee`, `+0xef`, `+0xf0`, `+0xf1` [inference: the period winners]; then builds the main-stage round table through
/// virtual +0x3c into `league::ROUND_DATES` (`fra_first_apply_league_rules`). Returns 1.
/// Original: `HolFirst::set_up_rules` (0x5f6340, hol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_first_set_up_rules(this: usize) -> u32 {
    covered!("mwc_10 hol first rules");
    fra_first_apply_league_rules(League(this), &hol_first_rules(this))
}

/// Comparer for `hol_first_set_up_rules`: a dry run (`compare_nation_league_rules`).
unsafe extern "thiscall" fn compare_hol_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_nation_league_rules("hol_first_set_up_rules", League(this), hol_first_rules(this), || o(this), || hol_first_set_up_rules(this))
}

/// The Dutch Eerste Divisie has 18 clubs, with no prize fund and no TV money.
///
/// `HolFirst::create_table()`: `+0x3e` = 18, the table (`exe::crt_malloc` of 18 x 0x3b; out of memory, the error box of
/// line 0x19d and 0) with the clubs of `KEY_CLUB_COMP_DUTCH_TOTO_DIVISION` in database order (League::init_table_row);
/// not full: 0. Full: 1 (nothing else is written).
/// Original: `HolFirst::create_table` (0x5f6240, hol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_first_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0x12); // mwc10: 18 clubs
    (fra_first_fill_league_table(this, 0x9cee08, 0x19d, KEY_CLUB_COMP_DUTCH_TOTO_DIVISION) == Some(true)) as u32
}

/// Comparer for `hol_first_create_table`: only the original runs, against the plan (`compare_nation_league_table`).
unsafe extern "thiscall" fn compare_hol_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_nation_league_table("hol_first_create_table", League(this), 0x12, KEY_CLUB_COMP_DUTCH_TOTO_DIVISION, None, || o(this))
}
