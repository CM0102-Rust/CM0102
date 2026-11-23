use super::*;

/// Rule: the Dutch Eerste Divisie turnover (called by the Eredivisie's): the season reopened, last season's stages
/// destroyed and the league rebuilt for the next year (`rebuild_season` with HolFirst::create_table at 0x5f6240 and the
/// boxes of hol_first.cpp at lines 0x1ff, 0x206, 0x20c, 0x213; the predicted positions), then the board expectations and
/// the officials (`set_expectations_and_renew_officials`), whose eax is returned. After a box: 0.
/// Original: `HolFirst::start_new_season` (0x5f63f0, hol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x5f6240, 0x9cee08, [0x1ff, 0x206, 0x20c, 0x213], 1) {
        return 0;
    }
    covered!("mwc_19 dutch eerste divisie turnover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `hol_first_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_hol_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HOL_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("hol_first_start_new_season", League(this), || o(this))
}

/// Rule: an Eerste Divisie result also counts in its period: rounds 1..7 the first period, 8..15 the second, 16..25 the
/// third, 26 on the fourth; `add_result_by_period` with the bounds 8, 16 and 26 and HolFirst::create_period_stage
/// (0x5f6ad0).
/// Original: `HolFirst::add_result` (0x5f65b0, hol_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn hol_first_add_result(this: usize, fixture: usize, arg: u32) -> u32 {
    covered!("mwc_24 eerste divisie result");
    add_result_by_period(this, fixture, arg, &[8, 16, 26], 0x5f6ad0) // mwc24: the Dutch periods
}

/// Comparer for `hol_first_add_result` (`compare_add_result_by_period`).
pub(crate) unsafe extern "thiscall" fn compare_hol_first_add_result(this: usize, fixture: usize, arg: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_HOL_FIRST_ADD_RESULT);
    compare_add_result_by_period("hol_first_add_result", this, fixture, &[8, 16, 26], || o(this, fixture, arg))
}

/// Rule: the next stage of the Eerste Divisie: past the last stage, the error box; otherwise only when the Eredivisie is
/// complete: its positions and statuses are updated and the current stage goes up by one; on reaching stage 4 (after
/// the four periods) the play-off groups are built. With the Eredivisie still playing, nothing (it asks again later).
///
/// At the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the box of hol_first.cpp at line
/// 0x263. Otherwise `premier_division_complete`(`KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION`) (virtual
/// `comp_vtable::CHECK_COMPLETE` with (0, 1)); when complete, `update_premier_division_status` (virtual
/// `league_vtable::UPDATE_POSITIONS_AND_STATUS`), the current stage (read again) + 1 and, at 4,
/// HolFirst::create_play_off_groups (0x5f6720, thiscall, ret 0, not ported).
/// Original: `HolFirst::advance_stage` (0x5f6650, hol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_first_advance_stage(this: usize) {
    let l = League(this);
    if (l.current_stage() as i32) >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x9cee08, 0x263);
    }
    if !premier_division_complete(KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION) {
        return;
    }
    update_premier_division_status(KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION);
    let stage = (l.current_stage() as i32).wrapping_add(1);
    l.set_current_stage(stage as usize);
    covered!("mwc_25 eerste divisie next stage");
    if stage == 4 { // mwc25: the play-off groups after the four periods
        call_on_league(0x5f6720, l);
    }
}

/// Comparer for `hol_first_advance_stage`: only the original runs (it asks the Eredivisie and updates it). Without the
/// play-off it is `compare_first_division_advance`; on reaching stage 4, `HolFirst::create_play_off_groups` (0x5f6720)
/// adds stages 4 and 5 (the virtual `+0x3c` with 4 and with 5) and the original ends on 5: in the 310th batch's real
/// game `before=3 after=5`, a false alarm of the one-step rule.
pub(crate) unsafe extern "thiscall" fn compare_hol_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_HOL_FIRST_ADVANCE_STAGE);
    let l = League(this);
    let before = l.current_stage() as i32;
    if before + 1 != 4 || before >= (l.stage_count() as i32).wrapping_sub(1) {
        return compare_first_division_advance("hol_first_advance_stage", l, || o(this));
    }
    o(this);
    let after = l.current_stage() as i32;
    SCOREBOARD.record(after == before || after == 5, || format!("hol_first_advance_stage({:#x}) before={} after={} (play-off: 5 expected)", this, before, after));
}

/// The Dutch Eerste Divisie: discipline rule 15 (`HOLLAND`), 6 stages besides the league (the stage vector, malloc of
/// 24); with no saved state, the rules, the table, the fixtures, the order, the statistics and the predicted positions
/// (`construct_nation_division`).
/// Original: `HolFirst::HolFirst` (0x5f5490, hol_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_hol_first(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &HOL_FIRST_DIVISION, || {})
}

/// Comparer for `construct_hol_first`: only the original runs (`compare_nation_division_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_hol_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_HOL_FIRST);
    compare_nation_division_ctor("construct_hol_first", League(this), year, record, &HOL_FIRST_DIVISION, || o(this, year, record))
}

/// Sets up the rules of the Dutch Eerste Divisie: 2 rounds, 3 points for a win and 1 for a draw; 1 go up, 0 to the
/// promotion play-off, 0 to the relegation play-off and 0 go down; above: `KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION`;
/// below: none; tie-breaker codes 1, 2, 0 (`+0xc5`..); 7 substitutes named and 3 substitutions; zeroes the class fields
/// `+0xee`, `+0xef`, `+0xf0`, `+0xf1` [inference: the period winners]; then builds the main-stage round table through
/// virtual +0x3c into `league::ROUND_DATES` (`fra_first_apply_league_rules`). Returns 1.
/// Original: `HolFirst::set_up_rules` (0x5f6340, hol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_first_set_up_rules(this: usize) -> u32 {
    covered!("mwc_10 hol first rules");
    fra_first_apply_league_rules(League(this), &hol_first_rules(this))
}

/// Comparer for `hol_first_set_up_rules`: a dry run (`compare_nation_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_hol_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HOL_FIRST_SET_UP_RULES);
    compare_nation_league_rules("hol_first_set_up_rules", League(this), hol_first_rules(this), || o(this), || hol_first_set_up_rules(this))
}

/// The Dutch Eerste Divisie has 18 clubs, with no prize fund and no TV money.
///
/// `HolFirst::create_table()`: `+0x3e` = 18, the table (`exe::crt_malloc` of 18 x 0x3b; out of memory, the error box of
/// line 0x19d and 0) with the clubs of `KEY_CLUB_COMP_DUTCH_TOTO_DIVISION` in database order (League::init_table_row);
/// not full: 0. Full: 1 (nothing else is written).
/// Original: `HolFirst::create_table` (0x5f6240, hol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_first_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0x12); // mwc10: 18 clubs
    (fra_first_fill_league_table(this, 0x9cee08, 0x19d, KEY_CLUB_COMP_DUTCH_TOTO_DIVISION) == Some(true)) as u32
}

/// Comparer for `hol_first_create_table`: only the original runs, against the plan (`compare_nation_league_table`).
pub(crate) unsafe extern "thiscall" fn compare_hol_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HOL_FIRST_CREATE_TABLE);
    compare_nation_league_table("hol_first_create_table", League(this), 0x12, KEY_CLUB_COMP_DUTCH_TOTO_DIVISION, None, || o(this))
}

/// The Premier Division object of key `premier` (`G_COMP_OBJECT_TABLE` by its id) is complete: virtual
/// `comp_vtable::CHECK_COMPLETE` with (0, 1) (push 1; push 0; mov ecx -> thiscall, ret 8; the byte in al).
pub(crate) unsafe fn premier_division_complete(premier: usize) -> bool {
    let p = comp_by_global(premier);
    let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(p) + comp_vtable::CHECK_COMPLETE));
    complete(p, 0, 1) != 0
}

/// Brings the table positions and statuses of the Premier Division of key `premier` up to date: virtual
/// `league_vtable::UPDATE_POSITIONS_AND_STATUS` (mov ecx -> thiscall, ret 0).
pub(crate) unsafe fn update_premier_division_status(premier: usize) {
    let p = comp_by_global(premier);
    let update: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(p) + league_vtable::UPDATE_POSITIONS_AND_STATUS));
    update(p);
}

/// The comparer of the two stage advances: only the original runs (it updates the Premier Division and may build the
/// play-off); the current stage must stay (at the last stage, or with the Premier Division still playing) or go up by
/// one, and on reaching 0 the first stage slot must hold a stage.
pub(crate) unsafe fn compare_first_division_advance(name: &str, this: League, original: impl FnOnce()) {
    let before = this.current_stage() as i32;
    let last = before >= (this.stage_count() as i32).wrapping_sub(1);
    original();
    let after = this.current_stage() as i32;
    let ok = if last { after == before } else { (after == before || after == before + 1) && (after != 0 || before == 0 || read_usize(this.stages()) != 0) };
    SCOREBOARD.record(ok, || format!("{}({:#x}) before={} after={} stages={}", name, this.addr(), before, after, this.stage_count()));
}

/// The rule writes of the Dutch Eerste Divisie (offset, size, value).
pub(crate) unsafe fn hol_first_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let _ = this;
    let mut v = vec![(0x3c, 2, 2), (0xc4, 1, 2), (0xc6, 1, 2), (0xc3, 1, 1), (0x42, 1, 1), (0xc5, 1, 1), (0xc7, 1, 0), (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0xf1, 1, 0), (0xf0, 1, 0), (0xef, 1, 0), (0xee, 1, 0), (0x4a, 1, 3), (0x1c, 4, read_usize(KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION) as u32), (0x20, 4, 0xffff_ffff), (0x49, 1, 7)];
    v.push((0xc2, 1, 3)); // mwc10: the Dutch Eerste Divisie points for a win
    v
}

pub(crate) const HOL_FIRST_DIVISION: NationDivision = NationDivision {
    vtable: 0x96b86c,
    bench: None,
    discipline: discipline_rule::HOLLAND, // mwc10: the Dutch Eerste Divisie discipline rule
    stages: 6,
    set_up_rules: 0x5f6340,
    create_table: 0x5f6240,
    source_file: 0x9cee08,
    box_lines: [0x49, 0x50, 0x57, 0x60, 0x67],
};
