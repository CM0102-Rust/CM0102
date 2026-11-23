use super::*;

/// Rule: the Croatian Second Division (south group) at career creation, as the First Division
/// (`construct_tiebreak_league`).
/// Original: `CroA2b::CroA2b` (0x516430, cro_a2b.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_cro_a2b(this: usize, year: u32, record: usize) -> usize {
    construct_tiebreak_league(League(this), year, record, &CROATIAN_SECOND_DIVISION_SOUTH, "mwd_1 croatian second division south new")
}

/// Comparer for `construct_cro_a2b`.
pub(crate) unsafe extern "thiscall" fn compare_construct_cro_a2b(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_CRO_A2B);
    compare_tiebreak_league_ctor("construct_cro_a2b", League(this), year, record, &CROATIAN_SECOND_DIVISION_SOUTH, || o(this, year, record))
}

/// Rule: the Croatian Second Division south group: as the First Division, with the north group above and nothing
/// below.
/// Original: `CroA2b::set_up_rules` (0x516f00, cro_a2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2b_set_up_rules(this: usize) -> u32 {
    covered!("mwd_2 croatian second division south rules");
    apply_league_rules_with_flags(League(this), 0x40, &cro_a2b_rules())
}

/// Comparer for `cro_a2b_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2b_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CRO_A2B_SET_UP_RULES);
    eng_prm_compare_league_rules("cro_a2b_set_up_rules", League(this), cro_a2b_rules(), || o(this), || cro_a2b_set_up_rules(this))
}

/// Rule: the Croatian Second Division south group, as the north group (16 clubs, 10,000 a share, 20,000 TV money),
/// with the competition `KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH`.
/// Original: `CroA2b::create_table` (0x516dd0, cro_a2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2b_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x10, 0x9b5430, 0x145, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH, 0x2710, || {
        covered!("mwd_3 croatian second division south tv money");
        eng_prm_pay_tv_rights(this, 0x4e20, 0)
    })
}

/// Comparer for `cro_a2b_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2b_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CRO_A2B_CREATE_TABLE);
    eng_prm_compare_league_table("cro_a2b_create_table", League(this), 0x10, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH, 0x2710, || o(this))
}

/// Rule: the Croatian Second Division south group schedule: the same 30 rounds and tie-break round as the First
/// Division.
/// Original: `CroA2b::create_schedule` (0x516700, cro_a2b.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn cro_a2b_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    croatian_league_schedule(League(this), stage, rounds, extra, league, 0x9b5430, "mwd_4 croatian second division south calendar")
}

/// Comparer for `cro_a2b_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2b_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_CRO_A2B_CREATE_SCHEDULE);
    compare_league_round_table("cro_a2b_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| cro_a2b_create_schedule(this, stage, r, e, l))
}

/// Rule: the Croatian Second Division south group turnover, as the north group's (lines 0x1b7, 0x1be, 0x1c4, 0x1cb of
/// cro_a2b.cpp).
/// Original: `CroA2b::start_new_season` (0x516fc0, cro_a2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2b_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    covered!("mwd_5 croatian second division south turnover");
    if !rebuild_croatian_league(this, 0x516dd0, 0x9b5430, [0x1b7, 0x1be, 0x1c4, 0x1cb], true) {
        return 0;
    }
    finish_croatian_turnover(this)
}

/// Comparer for `cro_a2b_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2b_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CRO_A2B_START_NEW_SEASON);
    compare_state_league_new_season("cro_a2b_start_new_season", League(this), || o(this))
}

/// Rule: the Croatian Second Division south group's next stage, as the First Division's (line 0x1e9 of cro_a2b.cpp,
/// the play-off `cro_a2b_create_tiebreak_playoff_stage`).
/// Original: `CroA2b::advance_stage` (0x517180, cro_a2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2b_advance_stage(this: usize) {
    covered!("mwd_6 croatian second division south next stage");
    state_league_advance_stage(League(this), 0x9b5430, 0x1e9, 0x517210);
}

/// Comparer for `cro_a2b_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2b_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CRO_A2B_ADVANCE_STAGE);
    ita_ser_c1a_compare_advance_stage("cro_a2b_advance_stage", League(this), || o(this), || cro_a2b_advance_stage(this))
}

/// Rule: the same tie-break play-off at the end of the Croatian Second Division south group (line 0x23a of
/// cro_a2b.cpp).
/// Original: `CroA2b::create_tiebreak_playoff_stage` (0x517210, cro_a2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2b_create_tiebreak_playoff_stage(this: usize) {
    create_croatian_tiebreak_playoff(League(this), 0x9b5430, 0x23a);
}

/// Comparer for `cro_a2b_create_tiebreak_playoff_stage`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2b_create_tiebreak_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CRO_A2B_CREATE_TIEBREAK_PLAYOFF_STAGE);
    compare_croatian_tiebreak_playoff("cro_a2b_create_tiebreak_playoff_stage", League(this), || o(this))
}

/// The rules of the Croatian Second Division south group: the north group above (as the original: the south group's
/// upper league is the north group, not the First Division), nothing below.
pub(crate) unsafe fn cro_a2b_rules() -> Vec<(usize, u8, u32)> {
    croatian_league_rules(read_usize(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH) as u32, 0xffff_ffff)
}

/// The Croatian Second Division, south group.
pub(crate) const CROATIAN_SECOND_DIVISION_SOUTH: LeagueDivision = LeagueDivision {
    vtabela: 0x96922c,
    discipline: 0x21,
    rules: 0x516f00,
    table: 0x516dd0,
    file: 0x9b5430,
    rows: [0x50, 0x57, 0x5e, 0x67, 0x6e],
    low: false,
};
