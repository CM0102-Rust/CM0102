use super::*;

/// Rule: the Dutch turnover, led by the Eredivisie: the season reopened; promotion and relegation along the chain; the
/// Eredivisie rebuilt for the next year (rules, table, fixtures, order); the division tier of the Eerste Divisie clubs
/// capped and the Eerste Divisie turned over; the predicted positions, the board expectations and the officials.
///
/// `top_division_turnover` with, before, virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(1); the table
/// HolPrm::create_table (0x5f8430) and the boxes of hol_prm.cpp (0x9cefa4) at 0x1eb, 0x1f2, 0x1f8, 0x1ff; after,
/// League::limit_club_division_tier(1, 3, −3, 1) and the start_new_season (virtual `comp_vtable::START_NEW_SEASON`, no
/// null check) of the object of `KEY_CLUB_COMP_DUTCH_TOTO_DIVISION` (the Eerste Divisie). After a box: 0.
/// Original: `HolPrm::start_new_season` (0x5f8610, hol_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_prm_start_new_season(this: usize) -> u32 {
    let this = League(this);
    top_division_turnover(this, || {
        promotion_relegation_chain(this, 1);
    }, 0x5f8430, 0x9cefa4, [0x1eb, 0x1f2, 0x1f8, 0x1ff], || {
        covered!("mwc_20 dutch eredivisie turnover");
        exe::league_limit_club_division_tier(comp_by_global(KEY_CLUB_COMP_DUTCH_TOTO_DIVISION), 1, 3, -3, 1);
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_DUTCH_TOTO_DIVISION), comp_vtable::START_NEW_SEASON);
    })
}

/// Comparer for `hol_prm_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_hol_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HOL_PRM_START_NEW_SEASON);
    compare_state_league_new_season("hol_prm_start_new_season", League(this), || o(this))
}

/// The Dutch Eredivisie schedule: 34 rounds, from 14 Aug to 14 May; 32 with matches in TV slots.
///
/// `HolPrm::create_schedule(stage, &rounds, &extra, &is_league)` (`create_league_schedule`): only the main stage (-1;
/// the others return 0 without writing); is_league = 1, rounds = 34, extra = 0 and the table of 34 x 0x41 bytes
/// (`exe::crt_malloc`; out of memory, the error box of line 0x9d and 0) with each round's date and TV slots
/// (`league_round_set_date`, `league_round_set_match_slot`, cdecl).
/// Original: `HolPrm::create_schedule` (0x5f78d0, hol_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn hol_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_league_schedule(League(this), stage, rounds, extra, league, &HOL_PRM_SCHEDULE)
}

/// Comparer for `hol_prm_create_schedule`: side by side, the copy's table freed (`compare_cfa_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_hol_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_HOL_PRM_CREATE_SCHEDULE);
    compare_cfa_schedule("hol_prm_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| hol_prm_create_schedule(this, stage, r, e, l))
}

/// The Dutch Eredivisie: discipline rule 15 (`HOLLAND`), only the league; with no saved state, the rules, the table,
/// the fixtures, the order, the statistics and the predicted positions (`construct_nation_division`).
/// Original: `HolPrm::HolPrm` (0x5f7610, hol_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_hol_prm(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &HOL_PRM_DIVISION, || {})
}

/// Comparer for `construct_hol_prm`: only the original runs (`compare_nation_division_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_hol_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_HOL_PRM);
    compare_nation_division_ctor("construct_hol_prm", League(this), year, record, &HOL_PRM_DIVISION, || o(this, year, record))
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
pub(crate) unsafe extern "thiscall" fn compare_hol_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HOL_PRM_SET_UP_RULES);
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
pub(crate) unsafe extern "thiscall" fn compare_hol_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HOL_PRM_CREATE_TABLE);
    compare_nation_league_table("hol_prm_create_table", League(this), 0x12, KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION, Some(0xc350), || o(this))
}

/// The rule writes of the Dutch Eredivisie (offset, size, value).
pub(crate) unsafe fn hol_prm_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let _ = this;
    let mut v = vec![(0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 0), (0xc8, 1, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 2), (0xc1, 1, 1), (0x1c, 4, 0xffff_ffff), (0x52, 1, 2), (0x4a, 1, 3), (0x20, 4, read_usize(KEY_CLUB_COMP_DUTCH_TOTO_DIVISION) as u32), (0x49, 1, 7)];
    v.push((0xc2, 1, 3)); // mwc9: the Dutch Eredivisie points for a win
    v
}

pub(crate) const HOL_PRM_SCHEDULE: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 34, file: 0x9cefa4, row: 0x9d, table: &HOL_PRM_SCHEDULE_ROUNDS, branch: "mwc_2 hol prm schedule" };

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

/// The Eredivisie: 34 rounds (date, TV slots).
pub(crate) const HOL_PRM_SCHEDULE_ROUNDS: [FraThirdLeagueRound; 34] = [
    (14, 7, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 0: 14 Aug
    (21, 7, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 1: 21 Aug
    (28, 7, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 2: 28 Aug
    (11, 8, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 3: 11 Sep
    (15, 8, 0, 2, 2, &[(1, 2, 1), (-1, -1, -1)]), // 4: 15 Sep
    (18, 8, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 5: 18 Sep (mwc2 hol_prm)
    (25, 8, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 6: 25 Sep
    (29, 8, 0, 2, 2, &[(1, 2, 1), (-1, -1, -1)]), // 7: 29 Sep
    (2, 9, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 8: 2 Oct
    (13, 9, 0, 2, 2, &[(1, 2, 1), (-1, -1, -1)]), // 9: 13 Oct
    (16, 9, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 10: 16 Oct
    (23, 9, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 11: 23 Oct
    (30, 10, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 12: 30 Nov
    (6, 10, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 13: 6 Nov
    (20, 10, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 14: 20 Nov
    (27, 10, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 15: 27 Nov
    (4, 11, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 16: 4 Dec
    (11, 11, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 17: 11 Dec
    (18, 11, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 18: 18 Dec
    (5, 1, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 19: 5 Feb (+1 year)
    (12, 1, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 20: 12 Feb (+1 year)
    (19, 1, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 21: 19 Feb (+1 year)
    (26, 1, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 22: 26 Feb (+1 year)
    (4, 2, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 23: 4 Mar (+1 year)
    (11, 2, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 24: 11 Mar (+1 year)
    (18, 2, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 25: 18 Mar (+1 year)
    (25, 2, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 26: 25 Mar (+1 year)
    (1, 3, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 27: 1 Apr (+1 year)
    (8, 3, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 28: 8 Apr (+1 year)
    (15, 3, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 29: 15 Apr (+1 year)
    (22, 3, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 30: 22 Apr (+1 year)
    (29, 3, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 31: 29 Apr (+1 year)
    (7, 4, 1, 6, 2, &[(-1, -1, -1)]), // 32: 7 May (+1 year)
    (14, 4, 1, 6, 2, &[(-1, -1, -1)]), // 33: 14 May (+1 year)
];
