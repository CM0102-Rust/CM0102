use super::*;

/// Rule: the Croatian Second Division (north group) at career creation, as the First Division
/// (`construct_tiebreak_league`).
/// Original: `CroA2a::CroA2a` (0x515310, cro_a2a.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_cro_a2a(this: usize, year: u32, record: usize) -> usize {
    construct_tiebreak_league(League(this), year, record, &CROATIAN_SECOND_DIVISION_NORTH, "mwd_1 croatian second division north new")
}

/// Comparer for `construct_cro_a2a`.
pub(crate) unsafe extern "thiscall" fn compare_construct_cro_a2a(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_CRO_A2A);
    compare_tiebreak_league_ctor("construct_cro_a2a", League(this), year, record, &CROATIAN_SECOND_DIVISION_NORTH, || o(this, year, record))
}

/// Rule: the Croatian Second Division north group: as the First Division, with the First Division above and the south
/// group below.
/// Original: `CroA2a::set_up_rules` (0x515de0, cro_a2a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2a_set_up_rules(this: usize) -> u32 {
    covered!("mwd_2 croatian second division north rules");
    apply_league_rules_with_flags(League(this), 0x40, &cro_a2a_rules())
}

/// Comparer for `cro_a2a_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2a_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CRO_A2A_SET_UP_RULES);
    eng_prm_compare_league_rules("cro_a2a_set_up_rules", League(this), cro_a2a_rules(), || o(this), || cro_a2a_set_up_rules(this))
}

/// Rule: the Croatian Second Division north group has 16 clubs (its own, in database order); when full, the prize fund
/// (10,000 a share) and TV pays 20,000 to each club. As `cro_a1_create_table`, with the competition
/// `KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH`.
/// Original: `CroA2a::create_table` (0x515cb0, cro_a2a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2a_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x10, 0x9b53f8, 0x145, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH, 0x2710, || {
        covered!("mwd_3 croatian second division north tv money");
        eng_prm_pay_tv_rights(this, 0x4e20, 0)
    })
}

/// Comparer for `cro_a2a_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2a_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CRO_A2A_CREATE_TABLE);
    eng_prm_compare_league_table("cro_a2a_create_table", League(this), 0x10, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH, 0x2710, || o(this))
}

/// Rule: the Croatian Second Division north group schedule: the same 30 rounds and tie-break round as the First
/// Division.
/// Original: `CroA2a::create_schedule` (0x5155e0, cro_a2a.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn cro_a2a_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    croatian_league_schedule(League(this), stage, rounds, extra, league, 0x9b53f8, "mwd_4 croatian second division north calendar")
}

/// Comparer for `cro_a2a_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2a_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_CRO_A2A_CREATE_SCHEDULE);
    compare_league_round_table("cro_a2a_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| cro_a2a_create_schedule(this, stage, r, e, l))
}

/// Rule: the Croatian Second Division north group turnover (called by the First Division's): reopened, rebuilt for the
/// next year (`rebuild_croatian_league`, lines 0x1b7, 0x1be, 0x1c4, 0x1cb), the predicted positions, the board
/// expectations and the officials.
/// Original: `CroA2a::start_new_season` (0x515ea0, cro_a2a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2a_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    covered!("mwd_5 croatian second division north turnover");
    if !rebuild_croatian_league(this, 0x515cb0, 0x9b53f8, [0x1b7, 0x1be, 0x1c4, 0x1cb], true) {
        return 0;
    }
    finish_croatian_turnover(this)
}

/// Comparer for `cro_a2a_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2a_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CRO_A2A_START_NEW_SEASON);
    compare_state_league_new_season("cro_a2a_start_new_season", League(this), || o(this))
}

/// The rules of the Croatian Second Division north group: the First Division above, the south group below.
pub(crate) unsafe fn cro_a2a_rules() -> Vec<(usize, u8, u32)> {
    croatian_league_rules(read_usize(KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4) as u32, read_usize(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH) as u32)
}

/// The Croatian Second Division, north group.
pub(crate) const CROATIAN_SECOND_DIVISION_NORTH: LeagueDivision = LeagueDivision {
    vtabela: 0x969178,
    discipline: 0x21,
    rules: 0x515de0,
    table: 0x515cb0,
    file: 0x9b53f8,
    rows: [0x50, 0x57, 0x5e, 0x67, 0x6e],
    low: false,
};
