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

/// Rule: the Croatian Second Division north group's next stage, as the south group's (line 0x1e9 of cro_a2a.cpp, the
/// play-off `cro_a2a_create_tiebreak_playoff_stage`, through its detour).
/// Original: `CroA2a::advance_stage` (0x516060, cro_a2a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2a_advance_stage(this: usize) {
    covered!("cx09_18 croatian second division north next stage");
    state_league_advance_stage(League(this), CRO_A2A_SOURCE, 0x1e9, 0x5160f0);
}

/// Rule: the tie-break play-off at the end of the Croatian Second Division north group, as the south group's (line
/// 0x23a of cro_a2a.cpp; `create_croatian_tiebreak_playoff`).
/// Original: `CroA2a::create_tiebreak_playoff_stage` (0x5160f0, cro_a2a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2a_create_tiebreak_playoff_stage(this: usize) {
    create_croatian_tiebreak_playoff(League(this), CRO_A2A_SOURCE, 0x23a);
}

/// The Croatian Second Division north group's day: the league day (`exe::league_update_for_date`(today, full)), whose
/// byte is returned. At the end of the season (`league::COMPLETE`), with no stage yet (`league::STAGE_COUNT` = 0),
/// with `full` and with `league::TIEBREAK_MATCHES`: `league::SEASON_CLOSED` = 0 and, from the top, the first pair of
/// neighbouring rows that needs a play-off (`croatian_tie_needs_playoff`) makes one stage (`league::STAGE_COUNT` = 1,
/// `league::STAGES` = malloc of 4) and the next stage (virtual `comp_vtable::ADVANCE_STAGE`); with none,
/// `league::COMPLETE` = 0, `league::TIEBREAK_MATCHES` = 0 and virtual `league_vtable::UPDATE_POSITIONS_AND_STATUS`.
/// Original: `CroA2a::update_for_date` (0x516360, cro_a2a.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn cro_a2a_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let l = League(this);
    let r = exe::league_update_for_date(this, today, full) as u8 as u32;
    if l.complete() == 0 || l.stage_count() != 0 || full == 0 || l.tiebreak_matches() == 0 {
        return r;
    }
    l.set_season_closed(0);
    let row = |k: i32| LeagueTableRow(l.table() + k as usize * league_table_row::SIZE);
    let mut i: i16 = 0;
    while (i as i32) < l.club_count() as i32 - 1 {
        if croatian_tie_needs_playoff(row(i as i32), row(i as i32 + 1)) {
            covered!("cx09_18 croatian north tie-break");
            l.set_stage_count(1);
            l.set_stages(exe::crt_malloc(4));
            bra_nat_first_call_virtual_no_args(this, comp_vtable::ADVANCE_STAGE);
            return r;
        }
        i = i.wrapping_add(1);
    }
    l.set_complete(0);
    l.set_tiebreak_matches(0);
    let status: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + league_vtable::UPDATE_POSITIONS_AND_STATUS));
    status(this);
    r
}

/// Comparer for `cro_a2a_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2a_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CRO_A2A_ADVANCE_STAGE);
    ita_ser_c1a_compare_advance_stage("cro_a2a_advance_stage", League(this), || o(this), || cro_a2a_advance_stage(this))
}

/// Comparer for `cro_a2a_create_tiebreak_playoff_stage`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2a_create_tiebreak_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CRO_A2A_CREATE_TIEBREAK_PLAYOFF_STAGE);
    compare_croatian_tiebreak_playoff("cro_a2a_create_tiebreak_playoff_stage", League(this), || o(this))
}

/// Comparer for `cro_a2a_update_for_date`: only the original (the league day cannot run twice); in the main stage with
/// `full`, a stage created means a tied pair needing a play-off, and the season end is never left pending.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2a_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let l = League(this);
    let stages = l.stage_count();
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_CRO_A2A_UPDATE_FOR_DATE)(this, today, full);
    if full != 0 && stages == 0 {
        let row = |k: i32| LeagueTableRow(l.table() + k as usize * league_table_row::SIZE);
        let tie = (0..l.club_count() as i32 - 1).any(|k| croatian_tie_needs_playoff(row(k), row(k + 1)));
        let ok = if l.stage_count() != 0 { tie && l.stages() != 0 } else { !(l.complete() != 0 && l.tiebreak_matches() != 0) };
        SCOREBOARD.record(ok, || format!("cro_a2a_update_for_date({:#x}): stages {} -> {}, tie {}, complete {}, tie-break {}", this, stages, l.stage_count(), tie, l.complete(), l.tiebreak_matches()));
    }
    r
}

/// Whether two neighbouring rows of a Croatian table call for a tie-break play-off: the same points
/// (`league_table_row::POINTS`) unless both are relegated (`league_table_row::STATUS` 3).
pub(crate) unsafe fn croatian_tie_needs_playoff(upper: LeagueTableRow, lower: LeagueTableRow) -> bool {
    upper.points() == lower.points() && (upper.status() != 3 || lower.status() != 3)
}

/// The cro_a2a.cpp source path, for the error boxes.
pub(crate) const CRO_A2A_SOURCE: usize = 0x9b53f8;
