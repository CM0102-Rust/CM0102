// ---------- batch %%LEVA%%: the Danish Superliga: the constructor, the rules and the table (den_prm.cpp) ----------

pub(crate) const DEN_PRM_DIVISION: NationDivision = NationDivision {
    vtable: 0x9696e4,
    bench: None,
    discipline: discipline_rule::DENMARK, // mwc5: the Danish Superliga discipline rule
    stages: 0,
    set_up_rules: 0x553a00,
    create_table: 0x5538d0,
    source_file: 0x9b8e6c,
    box_lines: [0x50, 0x57, 0x5e, 0x67, 0x6e],
};

/// The Danish Superliga: discipline rule 8 (`DENMARK`), only the league; before the load, the Kvalifikationsraekken
/// clubs move to the generic lower division (`KEY_CLUB_COMP_A_LOWER_DIVISION`); with no saved state, the rules, the
/// table, the fixtures, the order, the statistics and the predicted positions (`construct_nation_division`).
/// Original: `DenPrm::DenPrm` (0x5525e0, den_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_den_prm(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &DEN_PRM_DIVISION, || {
        // mwc5: the Kvalifikationsraekken clubs go to the lower division before the load
        let kval = read_usize(KEY_CLUB_COMP_DANISH_KVALIFIKATIONSRAEKKEN);
        let mut i = 0i32;
        while i < read_usize(G_CLUB_COUNT) as i32 {
            let club = Club(read_usize(G_CLUB_TABLE) + i as usize * club::SIZE);
            let comp = read_usize(club.addr() + club::DIVISION);
            if comp != 0 && read_usize(comp) == kval {
                covered!("mwc_5 kvalifikationsraekken club moved");
                write_usize(club.addr() + club::DIVISION, comp_record_of(KEY_CLUB_COMP_A_LOWER_DIVISION));
            }
            i += 1;
        }
    })
}

/// Comparer for `construct_den_prm`: only the original runs (`compare_nation_division_ctor`).
unsafe extern "thiscall" fn compare_construct_den_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_nation_division_ctor("construct_den_prm", League(this), year, record, &DEN_PRM_DIVISION, || o(this, year, record))
}

/// The rule writes of the Danish Superliga (offset, size, value).
pub(crate) unsafe fn den_prm_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let _ = this;
    let mut v = vec![(0x3c, 2, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 2), (0x1c, 4, 0xffff_ffff), (0x52, 1, 2), (0x4a, 1, 3), (0x20, 4, read_usize(KEY_CLUB_COMP_DANISH_FIRST_DIVISION) as u32), (0x49, 1, 4)];
    v.push((0xc2, 1, 3)); // mwc5: the Danish Superliga points for a win
    v
}

/// Sets up the rules of the Danish Superliga: 3 rounds, 3 points for a win and 1 for a draw; 0 go up, 0 to the
/// promotion play-off, 0 to the relegation play-off and 2 go down; above: none; below:
/// `KEY_CLUB_COMP_DANISH_FIRST_DIVISION`; tie-breaker codes 1, 2, 0 (`+0xc5`..); 4 substitutes named and 3
/// substitutions; then builds the main-stage round table through virtual +0x3c into `league::ROUND_DATES`
/// (`fra_first_apply_league_rules`). Returns 1.
/// Original: `DenPrm::set_up_rules` (0x553a00, den_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwc_5 den prm rules");
    fra_first_apply_league_rules(League(this), &den_prm_rules(this))
}

/// Comparer for `den_prm_set_up_rules`: a dry run (`compare_nation_league_rules`).
unsafe extern "thiscall" fn compare_den_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_nation_league_rules("den_prm_set_up_rules", League(this), den_prm_rules(this), || o(this), || den_prm_set_up_rules(this))
}

/// The Danish Superliga has 12 clubs; prize fund by the share of 50,000 per place; TV pays 100,000 to each club.
///
/// `DenPrm::create_table()`: `+0x3e` = 12, the table (`exe::crt_malloc` of 12 x 0x3b; out of memory, the error box of
/// line 0x1ab and 0) with the clubs of `KEY_CLUB_COMP_DANISH_PREMIER_DIVISION` in database order
/// (League::init_table_row); not full: 0. Full: the fund `+0x28` (League::calc_prize_fund, share 0xc350), the prize
/// mode `+0xe1` = 1, the TV money (League::pay_tv_rights, 0x186a0 to every club) and 1.
/// Original: `DenPrm::create_table` (0x5538d0, den_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_prm_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0xc, 0x9b8e6c, 0x1ab, KEY_CLUB_COMP_DANISH_PREMIER_DIVISION, 0xc350, || {
        eng_prm_pay_tv_rights(this, 0x186a0, 0); // mwc5: the TV money of each club
    })
}

/// Comparer for `den_prm_create_table`: only the original runs, against the plan (`compare_nation_league_table`).
unsafe extern "thiscall" fn compare_den_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_nation_league_table("den_prm_create_table", League(this), 0xc, KEY_CLUB_COMP_DANISH_PREMIER_DIVISION, Some(0xc350), || o(this))
}
