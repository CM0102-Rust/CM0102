// ---------- batch %%LEVA%%: the 2. Bundesliga: the constructor, the rules and the table (ger_second.cpp) ----------

pub(crate) const GER_SECOND_DIVISION: NationDivision = NationDivision {
    vtable: 0x96b304,
    bench: None,
    discipline: discipline_rule::GERMANY_LEAGUE, // mwc8: the 2. Bundesliga discipline rule
    stages: 0,
    set_up_rules: 0x5df3e0,
    create_table: 0x5df2b0,
    source_file: 0x9cd4c4,
    box_lines: [0x43, 0x4a, 0x51, 0x5a, 0x61],
};

/// The 2. Bundesliga: discipline rule 13 (`GERMANY_LEAGUE`), only the league; with no saved state, the rules, the
/// table, the fixtures, the order, the statistics and the predicted positions (`construct_nation_division`).
/// Original: `GerSecond::GerSecond` (0x5de2e0, ger_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ger_second(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &GER_SECOND_DIVISION, || {})
}

/// Comparer for `construct_ger_second`: only the original runs (`compare_nation_division_ctor`).
unsafe extern "thiscall" fn compare_construct_ger_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_nation_division_ctor("construct_ger_second", League(this), year, record, &GER_SECOND_DIVISION, || o(this, year, record))
}

/// The rule writes of the 2. Bundesliga (offset, size, value).
pub(crate) unsafe fn ger_second_rules(this: usize) -> Vec<(usize, u8, u32)> {
    // the Regionalliga below only when the nation has its lower division playable (bit 4 of `nation::LEAGUE_SELECTED`)
    let nation = Nation(read_usize(read_usize(this + league::COMPETITION) + competition::NATION));
    let below = if nation.league_selected() & 4 != 0 { read_usize(KEY_CLUB_COMP_GERMAN_REGIONAL) as u32 } else { 0xffff_ffff };
    let mut v = vec![(0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 3), (0xbe, 1, 3), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 4), (0x1c, 4, read_usize(KEY_CLUB_COMP_GERMAN_FIRST_DIVISION) as u32), (0x20, 4, below), (0x52, 1, 2), (0x4a, 1, 3), (0x49, 1, 6)];
    v.push((0xc2, 1, 3)); // mwc8: the 2. Bundesliga points for a win
    v
}

/// Sets up the rules of the 2. Bundesliga: 2 rounds, 3 points for a win and 1 for a draw; 3 go up, 0 to the promotion
/// play-off, 0 to the relegation play-off and 4 go down; above: `KEY_CLUB_COMP_GERMAN_FIRST_DIVISION`; below:
/// `KEY_CLUB_COMP_GERMAN_REGIONAL` when the German lower division is playable (bit 4 of the nation's
/// `LEAGUE_SELECTED`), else none; tie-breaker codes 1, 2, 3 (`+0xc5`..); 6 substitutes named and 3 substitutions; then
/// builds the main-stage round table through virtual +0x3c into `league::ROUND_DATES` (`fra_first_apply_league_rules`).
/// Returns 1.
/// Original: `GerSecond::set_up_rules` (0x5df3e0, ger_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_second_set_up_rules(this: usize) -> u32 {
    covered!("mwc_8 ger second rules");
    fra_first_apply_league_rules(League(this), &ger_second_rules(this))
}

/// Comparer for `ger_second_set_up_rules`: a dry run (`compare_nation_league_rules`).
unsafe extern "thiscall" fn compare_ger_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_nation_league_rules("ger_second_set_up_rules", League(this), ger_second_rules(this), || o(this), || ger_second_set_up_rules(this))
}

/// The 2. Bundesliga has 18 clubs; prize fund by the share of 50,000 per place; TV pays 1,300,000 to each club.
///
/// `GerSecond::create_table()`: `+0x3e` = 18, the table (`exe::crt_malloc` of 18 x 0x3b; out of memory, the error box
/// of line 0x19f and 0) with the clubs of `KEY_CLUB_COMP_GERMAN_SECOND_DIVISION` in database order
/// (League::init_table_row); not full: 0. Full: the fund `+0x28` (League::calc_prize_fund, share 0xc350), the prize
/// mode `+0xe1` = 1, the TV money (League::pay_tv_rights, 0x13d620 to every club) and 1.
/// Original: `GerSecond::create_table` (0x5df2b0, ger_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_second_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x12, 0x9cd4c4, 0x19f, KEY_CLUB_COMP_GERMAN_SECOND_DIVISION, 0xc350, || {
        eng_prm_pay_tv_rights(this, 0x13d620, 0); // mwc8: the TV money of each club
    })
}

/// Comparer for `ger_second_create_table`: only the original runs, against the plan (`compare_nation_league_table`).
unsafe extern "thiscall" fn compare_ger_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_nation_league_table("ger_second_create_table", League(this), 0x12, KEY_CLUB_COMP_GERMAN_SECOND_DIVISION, Some(0xc350), || o(this))
}
