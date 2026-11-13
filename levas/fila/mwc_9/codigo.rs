// ---------- batch %%LEVA%%: the Eredivisie: the constructor, the rules and the table (hol_prm.cpp) ----------

pub(crate) const HOL_PRM_DIVISION: NationDivision = NationDivision {
    vtable: 0x96b920,
    bench: None,
    discipline: discipline_rule::HOLLAND, // mwc9: the Dutch Eredivisie discipline rule
    stages: 0,
    set_up_rules: 0x5f8560,
    create_table: 0x5f8430,
    source_file: 0x9cefa4,
    box_lines: [0x43, 0x4a, 0x51, 0x5a, 0x61],
};

/// The Dutch Eredivisie: discipline rule 15 (`HOLLAND`), only the league; with no saved state, the rules, the table,
/// the fixtures, the order, the statistics and the predicted positions (`construct_nation_division`).
/// Original: `HolPrm::HolPrm` (0x5f7610, hol_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_hol_prm(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &HOL_PRM_DIVISION, || {})
}

/// Comparer for `construct_hol_prm`: only the original runs (`compare_nation_division_ctor`).
unsafe extern "thiscall" fn compare_construct_hol_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_nation_division_ctor("construct_hol_prm", League(this), year, record, &HOL_PRM_DIVISION, || o(this, year, record))
}

/// The rule writes of the Dutch Eredivisie (offset, size, value).
pub(crate) unsafe fn hol_prm_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let _ = this;
    let mut v = vec![(0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 0), (0xc8, 1, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 2), (0xc1, 1, 1), (0x1c, 4, 0xffff_ffff), (0x52, 1, 2), (0x4a, 1, 3), (0x20, 4, read_usize(KEY_CLUB_COMP_DUTCH_TOTO_DIVISION) as u32), (0x49, 1, 7)];
    v.push((0xc2, 1, 3)); // mwc9: the Dutch Eredivisie points for a win
    v
}

/// Sets up the rules of the Dutch Eredivisie: 2 rounds, 3 points for a win and 1 for a draw; 0 go up, 0 to the
/// promotion play-off, 2 to the relegation play-off and 1 go down; above: none; below:
/// `KEY_CLUB_COMP_DUTCH_TOTO_DIVISION`; tie-breaker codes 1, 2, 0, 0 (`+0xc5`..); 7 substitutes named and 3
/// substitutions; then builds the main-stage round table through virtual +0x3c into `league::ROUND_DATES`
/// (`fra_first_apply_league_rules`). Returns 1.
/// Original: `HolPrm::set_up_rules` (0x5f8560, hol_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwc_9 hol prm rules");
    fra_first_apply_league_rules(League(this), &hol_prm_rules(this))
}

/// Comparer for `hol_prm_set_up_rules`: a dry run (`compare_nation_league_rules`).
unsafe extern "thiscall" fn compare_hol_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_nation_league_rules("hol_prm_set_up_rules", League(this), hol_prm_rules(this), || o(this), || hol_prm_set_up_rules(this))
}

/// The Dutch Eredivisie has 18 clubs; prize fund by the share of 50,000 per place; TV pays 1,000,000 to each club.
///
/// `HolPrm::create_table()`: `+0x3e` = 18, the table (`exe::crt_malloc` of 18 x 0x3b; out of memory, the error box of
/// line 0x182 and 0) with the clubs of `KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION` in database order
/// (League::init_table_row); not full: 0. Full: the fund `+0x28` (League::calc_prize_fund, share 0xc350), the prize
/// mode `+0xe1` = 1, the TV money (League::pay_tv_rights, 0xf4240 to every club) and 1.
/// Original: `HolPrm::create_table` (0x5f8430, hol_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_prm_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x12, 0x9cefa4, 0x182, KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION, 0xc350, || {
        eng_prm_pay_tv_rights(this, 0xf4240, 0); // mwc9: the TV money of each club
    })
}

/// Comparer for `hol_prm_create_table`: only the original runs, against the plan (`compare_nation_league_table`).
unsafe extern "thiscall" fn compare_hol_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_nation_league_table("hol_prm_create_table", League(this), 0x12, KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION, Some(0xc350), || o(this))
}
