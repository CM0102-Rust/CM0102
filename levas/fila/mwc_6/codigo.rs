// ---------- batch %%LEVA%%: the Danish First Division: the constructor, the rules and the table (den_first.cpp) ----------

pub(crate) const DEN_FIRST_DIVISION: NationDivision = NationDivision {
    vtable: 0x969630,
    bench: None,
    discipline: discipline_rule::DENMARK, // mwc6: the Danish First Division discipline rule
    stages: 0,
    set_up_rules: 0x5523b0,
    create_table: 0x552290,
    source_file: 0x9b8e30,
    box_lines: [0x42, 0x49, 0x50, 0x59, 0x60],
};

/// The Danish First Division: discipline rule 8 (`DENMARK`), only the league; with no saved state, the rules, the
/// table, the fixtures, the order, the statistics and the predicted positions (`construct_nation_division`).
/// Original: `DenFirst::DenFirst` (0x5516f0, den_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_den_first(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &DEN_FIRST_DIVISION, || {})
}

/// Comparer for `construct_den_first`: only the original runs (`compare_nation_division_ctor`).
unsafe extern "thiscall" fn compare_construct_den_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_nation_division_ctor("construct_den_first", League(this), year, record, &DEN_FIRST_DIVISION, || o(this, year, record))
}

/// The rule writes of the Danish First Division (offset, size, value).
pub(crate) unsafe fn den_first_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let _ = this;
    let mut v = vec![(0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 0), (0xbe, 1, 2), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 3), (0x1c, 4, read_usize(KEY_CLUB_COMP_DANISH_PREMIER_DIVISION) as u32), (0x20, 4, read_usize(KEY_CLUB_COMP_DANISH_SECOND_DIVISION) as u32), (0x4a, 1, 3), (0x49, 1, 4)];
    v.push((0xc2, 1, 3)); // mwc6: the Danish First Division points for a win
    v
}

/// Sets up the rules of the Danish First Division: 2 rounds, 3 points for a win and 1 for a draw; 2 go up, 0 to the
/// promotion play-off, 0 to the relegation play-off and 3 go down; above: `KEY_CLUB_COMP_DANISH_PREMIER_DIVISION`;
/// below: `KEY_CLUB_COMP_DANISH_SECOND_DIVISION`; tie-breaker codes 1, 2, 0 (`+0xc5`..); 4 substitutes named and 3
/// substitutions; then builds the main-stage round table through virtual +0x3c into `league::ROUND_DATES`
/// (`fra_first_apply_league_rules`). Returns 1.
/// Original: `DenFirst::set_up_rules` (0x5523b0, den_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_first_set_up_rules(this: usize) -> u32 {
    covered!("mwc_6 den first rules");
    fra_first_apply_league_rules(League(this), &den_first_rules(this))
}

/// Comparer for `den_first_set_up_rules`: a dry run (`compare_nation_league_rules`).
unsafe extern "thiscall" fn compare_den_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_nation_league_rules("den_first_set_up_rules", League(this), den_first_rules(this), || o(this), || den_first_set_up_rules(this))
}

/// The Danish First Division has 16 clubs; prize fund by the share of 10,000 per place; no TV money.
///
/// `DenFirst::create_table()`: `+0x3e` = 16, the table (`exe::crt_malloc` of 16 x 0x3b; out of memory, the error box of
/// line 0x142 and 0) with the clubs of `KEY_CLUB_COMP_DANISH_FIRST_DIVISION` in database order
/// (League::init_table_row); not full: 0. Full: the fund `+0x28` (League::calc_prize_fund, share 0x2710), the prize
/// mode `+0xe1` = 1 and 1.
/// Original: `DenFirst::create_table` (0x552290, den_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_first_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x10, 0x9b8e30, 0x142, KEY_CLUB_COMP_DANISH_FIRST_DIVISION, 0x2710, || {
        // no TV money
    })
}

/// Comparer for `den_first_create_table`: only the original runs, against the plan (`compare_nation_league_table`).
unsafe extern "thiscall" fn compare_den_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_nation_league_table("den_first_create_table", League(this), 0x10, KEY_CLUB_COMP_DANISH_FIRST_DIVISION, Some(0x2710), || o(this))
}
