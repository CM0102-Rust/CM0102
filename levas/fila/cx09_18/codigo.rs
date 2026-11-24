// ---------- cx09_18: the Croatian Second Division north group: the stage advance, the tie-break play-off and the day (cro_a2a.cpp) ----------

/// The cro_a2a.cpp source path, for the error boxes.
const CRO_A2A_SOURCE: usize = 0x9b53f8;

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

/// Whether two neighbouring rows of a Croatian table call for a tie-break play-off: the same points
/// (`league_table_row::POINTS`) unless both are relegated (`league_table_row::STATUS` 3).
unsafe fn croatian_tie_needs_playoff(upper: LeagueTableRow, lower: LeagueTableRow) -> bool {
    upper.points() == lower.points() && (upper.status() != 3 || lower.status() != 3)
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    ita_ser_c1a_compare_advance_stage("cro_a2a_advance_stage", League(this), || o(this), || cro_a2a_advance_stage(this))
}

/// Comparer for `cro_a2a_create_tiebreak_playoff_stage`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2a_create_tiebreak_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_croatian_tiebreak_playoff("cro_a2a_create_tiebreak_playoff_stage", League(this), || o(this))
}

/// Comparer for `cro_a2a_update_for_date`: only the original (the league day cannot run twice); in the main stage with
/// `full`, a stage created means a tied pair needing a play-off, and the season end is never left pending.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2a_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let l = League(this);
    let stages = l.stage_count();
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I2%%)(this, today, full);
    if full != 0 && stages == 0 {
        let row = |k: i32| LeagueTableRow(l.table() + k as usize * league_table_row::SIZE);
        let tie = (0..l.club_count() as i32 - 1).any(|k| croatian_tie_needs_playoff(row(k), row(k + 1)));
        let ok = if l.stage_count() != 0 { tie && l.stages() != 0 } else { !(l.complete() != 0 && l.tiebreak_matches() != 0) };
        SCOREBOARD.record(ok, || format!("cro_a2a_update_for_date({:#x}): stages {} -> {}, tie {}, complete {}, tie-break {}", this, stages, l.stage_count(), tie, l.complete(), l.tiebreak_matches()));
    }
    r
}
