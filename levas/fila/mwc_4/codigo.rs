// ---------- batch %%LEVA%%: the Belgian Second Division: the constructor, the rules and the table (bel_second.cpp) ----------

pub(crate) const BEL_SECOND_DIVISION: NationDivision = NationDivision {
    vtable: 0x9679f4,
    bench: Some((4, 3)),
    discipline: discipline_rule::BELGIUM_LEAGUE, // mwc4: the Belgian Second Division discipline rule
    stages: 4,
    set_up_rules: 0x41fea0,
    create_table: 0x41fd70,
    source_file: 0x989b54,
    box_lines: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

/// The Belgian Second Division: discipline rule 4 (`BELGIUM_LEAGUE`), the bench (4 substitutes named, 3 substitutions),
/// 4 stages besides the league (the stage vector, malloc of 16); with no saved state, the rules, the table, the
/// fixtures, the order, the statistics and the predicted positions (`construct_nation_division`).
/// Original: `BelSecond::BelSecond` (0x41f030, bel_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bel_second(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &BEL_SECOND_DIVISION, || {})
}

/// Comparer for `construct_bel_second`: only the original runs (`compare_nation_division_ctor`).
unsafe extern "thiscall" fn compare_construct_bel_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_nation_division_ctor("construct_bel_second", League(this), year, record, &BEL_SECOND_DIVISION, || o(this, year, record))
}

/// The rule writes of the Belgian Second Division (offset, size, value).
pub(crate) unsafe fn bel_second_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let _ = this;
    let mut v = vec![(0xc5, 1, 3), (0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc6, 1, 1), (0xc7, 1, 2), (0xc8, 1, 0), (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 1), (0xc1, 1, 2), (0x1c, 4, read_usize(KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION) as u32), (0xf8, 1, 0), (0xf7, 1, 0), (0xf6, 1, 0), (0x52, 1, 2), (0x20, 4, read_usize(KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION) as u32), (0xd9, 2, 0x43)];
    v.push((0xc2, 1, 3)); // mwc4: the Belgian Second Division points for a win
    v
}

/// Sets up the rules of the Belgian Second Division: 2 rounds, 3 points for a win and 1 for a draw; 1 go up, 0 to the
/// promotion play-off, 1 to the relegation play-off and 2 go down; above: `KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION`;
/// below: `KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION`; tie-breaker codes 3, 1, 2, 0 (`+0xc5`..); zeroes the class fields
/// `+0xf6`, `+0xf7`, `+0xf8` [inference: the period winners]; rule bits `+0xd9` = 0x43; then builds the main-stage
/// round table through virtual +0x3c into `league::ROUND_DATES` (`fra_first_apply_league_rules`). Returns 1.
/// Original: `BelSecond::set_up_rules` (0x41fea0, bel_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_second_set_up_rules(this: usize) -> u32 {
    covered!("mwc_4 bel second rules");
    fra_first_apply_league_rules(League(this), &bel_second_rules(this))
}

/// Comparer for `bel_second_set_up_rules`: a dry run (`compare_nation_league_rules`).
unsafe extern "thiscall" fn compare_bel_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_nation_league_rules("bel_second_set_up_rules", League(this), bel_second_rules(this), || o(this), || bel_second_set_up_rules(this))
}

/// The Belgian Second Division has 18 clubs; prize fund by the share of 10,000 per place; TV pays 40,000 to each club
/// (before the fund).
///
/// `BelSecond::create_table()`: `+0x3e` = 18, the table (`exe::crt_malloc` of 18 x 0x3b; out of memory, the error box
/// of line 0x1a4 and 0) with the clubs of `KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION` in database order
/// (League::init_table_row); not full: 0. Full: the TV money (League::pay_tv_rights, 0x9c40 to every club), the fund
/// `+0x28` (League::calc_prize_fund, share 0x2710), the prize mode `+0xe1` = 1 and 1.
/// Original: `BelSecond::create_table` (0x41fd70, bel_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_second_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0x12);
    if fra_first_fill_league_table(this, 0x989b54, 0x1a4, KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION) != Some(true) {
        return 0;
    }
    covered!("mwc_4 full table");
    eng_prm_pay_tv_rights(this.addr(), 0x9c40, 0); // mwc4: the TV money of each club
    let fund = exe::league_calc_prize_fund(this, 0x2710);
    this.set_prize_fund(fund);
    this.set_prize_mode(1);
    1
}

/// Comparer for `bel_second_create_table`: only the original runs, against the plan (`compare_nation_league_table`).
unsafe extern "thiscall" fn compare_bel_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_nation_league_table("bel_second_create_table", League(this), 0x12, KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION, Some(0x2710), || o(this))
}
