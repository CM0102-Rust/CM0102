// ---------- batch %%LEVA%%: the constructors of the inter-continental cups: the CONCACAF Champions Cup and the Intercontinental Cup (con_champ.cpp, world_club_cup.cpp) ----------

/// The fields every inter-continental knockout cup sets: `current_round` 0, `stage_index` 0xff (the main
/// competition), `stage_count` 0 and `current_stage` −1.
unsafe fn intercontinental_cup_common_fields(this: Cup) {
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_u32(this.addr() + cup::STAGE_COUNT, 0);
    write_u32(this.addr() + cup::CURRENT_STAGE, 0xffff_ffff);
}

/// The common end of a new inter-continental cup; returns whether it got to the end with the statistics.
///
/// The round table (virtual `comp_vtable::CREATE_SCHEDULE`(−1, &`cup::ROUND_COUNT`, &`cup::STAGE_LABEL`, 0): push 0;
/// push; push; push −1; mov ecx -> thiscall, ret 0x10) in `round_table()`; the tie tree (Cup::map_fixture_tree
/// 0x518790, thiscall, ret 0; negative: the error box of line 0x4f of source `file` and false); the statistics
/// (operator new `exe::crt_operator_new` of 0x5ce and Comp_Stats::Comp_Stats `exe::new_comp_stats(new, this)`: push
/// this; mov ecx -> thiscall, ret 4) at +8 (null: the box of line 0x56 and false).
unsafe fn finish_intercontinental_cup(this: Cup, file: usize) -> bool {
    let create_schedule: unsafe extern "thiscall" fn(usize, i32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    write_u32(this.addr() + cup::ROUND_TABLE, create_schedule(this.addr(), -1, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0) as u32);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    if map_fixture_tree(this.addr()) < 0 {
        show_error_at(file, 0x4f);
        return false;
    }
    let p = exe::crt_operator_new(0x5ce);
    let e = if p != 0 { exe::new_comp_stats(p, this) } else { 0 };
    write_u32(this.addr() + 8, e as u32);
    if e == 0 {
        show_error_at(file, 0x56);
        return false;
    }
    true
}

/// Whether the saved state of the cup loads (Cup::load_from_disk 0x51fc00(this, 1): push 1; mov ecx -> thiscall, ret 4;
/// nonzero).
unsafe fn intercontinental_cup_loads(this: Cup) -> bool {
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    load_from_disk(this.addr(), 1) != 0
}

/// The CONCACAF Champions Cup of the game's year, at career creation: 8 clubs that come in later (the table starts
/// empty) and a 2,000,000 prize fund.
///
/// Cup::Cup (`exe::new_cup`, mov ecx -> thiscall, ret 0); the competition record at +4, the vtable 0x968aa0, the year
/// (`set_year`), the common fields (`intercontinental_cup_common_fields`), `club_sides` 2, `discipline_index` 0xff and
/// `run_by_stages` 0. If the saved state loads (`intercontinental_cup_loads`), returns this. Otherwise 8 teams
/// (`team_count`) in a zeroed team table of 6 bytes each (calloc `exe::crt_calloc(8, 6)`: push 6; push 8; add esp, 8 ->
/// cdecl; out of memory, the error box of line 0x41, source 0x9a7d30, and this), the prize fund 2,000,000 and the common
/// end (`finish_intercontinental_cup`). Returns this. The original's SEH frame only matters for exceptions.
/// Original: `ConChamp::ConChamp` (0x4c0c40, con_champ.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_con_champ(this: usize, year: u32, record: usize) -> usize {
    let this = Cup(this);
    exe::new_cup(this);
    covered!("cqw_12 concacaf champions cup");
    write_u32(this.addr() + 4, record as u32);
    write_u32(this.addr(), 0x968aa0);
    this.set_year(year as i16); // cqw12: the CONCACAF cup of the game's year
    intercontinental_cup_common_fields(this);
    this.set_club_sides(2);
    this.set_discipline_index(0xff);
    this.set_run_by_stages(0);
    if intercontinental_cup_loads(this) {
        return this.addr();
    }
    this.set_team_count(8);
    let t = exe::crt_calloc(8, 6);
    write_u32(this.addr() + cup::TEAM_TABLE, t as u32);
    if t == 0 {
        show_error_at(0x9a7d30, 0x41);
        return this.addr();
    }
    this.set_prize_fund(0x1e8480);
    finish_intercontinental_cup(this, 0x9a7d30);
    this.addr()
}

/// Shared comparer of the inter-continental cup constructors. They allocate the table, the tree and the statistics,
/// so they cannot run twice: only the original runs, then the plan is checked: it returns this, the vtable, the record
/// at +4 and the year (`cup::YEAR`).
unsafe fn compare_intercontinental_cup_ctor(name: &str, this: usize, year: u32, record: usize, vtable: usize, o: impl FnOnce() -> usize) -> usize {
    let r = o();
    let read = Cup(this).year();
    let ok = r == this && read_usize(this) == vtable && read_usize(this + 4) == record && read == year as i16;
    SCOREBOARD.record(ok, || format!("{}({:#x}): year {} requested {}, vtable {:#x}", name, this, read, year as i16, read_usize(this)));
    r
}

/// Comparer for `construct_con_champ`.
pub(crate) unsafe extern "thiscall" fn compare_construct_con_champ(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_intercontinental_cup_ctor("construct_con_champ", this, year, record, 0x968aa0, || o(this, year, record))
}

/// The body of a two-club inter-continental cup (the Intercontinental Cup; the Interamerican Cup, 0x632080, has the same
/// shape).
///
/// Cup::Cup (`exe::new_cup`, thiscall, ret 0); the year (`set_year`), the competition record at +4, the vtable, the
/// common fields (`intercontinental_cup_common_fields`), `club_sides` 2, +0x52 = `b52`, 7 `substitutes_named`,
/// `substitutions_allowed` = `substitutions`, `discipline_index` 0xff and `run_by_stages` 0. If the saved state loads,
/// returns this. Otherwise the two clubs (the routine `select_teams`, WorldClubCup::pick_qualifiers in the Intercontinental Cup, thiscall, ret 0; negative: the error box of line
/// 0x41 of source `file` and this), the common end (`finish_intercontinental_cup`) and, when it got to the end, the
/// predicted positions (Cup::rank_teams 0x5223a0, thiscall, ret 0). Returns this.
#[allow(clippy::too_many_arguments)]
unsafe fn build_two_club_cup(this: Cup, year: u32, record: usize, vtable: usize, b52: u8, substitutions: u8, select_teams: usize, file: usize) -> usize {
    let select_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(select_teams);
    let rank_teams: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5223a0usize); // Cup::rank_teams
    exe::new_cup(this);
    this.set_year(year as i16);
    write_u32(this.addr() + 4, record as u32);
    write_u32(this.addr(), vtable as u32);
    intercontinental_cup_common_fields(this);
    this.set_club_sides(2);
    write_u8(this.addr() + 0x52, b52);
    this.set_substitutes_named(7);
    this.set_substitutions_allowed(substitutions);
    this.set_discipline_index(0xff);
    this.set_run_by_stages(0);
    if intercontinental_cup_loads(this) {
        return this.addr();
    }
    if select_teams(this.addr()) < 0 {
        show_error_at(file, 0x41);
        return this.addr();
    }
    if finish_intercontinental_cup(this, file) {
        rank_teams(this.addr());
    }
    this.addr()
}

/// The Intercontinental Cup of the game's year, at career creation, with the two clubs (the European and the South
/// American winners) picked at once by WorldClubCup::pick_qualifiers (0x92b8b0): vtable 0x971250, +0x52 = 3, 3
/// substitutions, error boxes of source 0xad5848 (`build_two_club_cup`). The original's SEH frame only matters for
/// exceptions.
/// Original: `WorldClubCup::WorldClubCup` (0x92b4b0, world_club_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_world_club_cup(this: usize, year: u32, record: usize) -> usize {
    covered!("cqw_12 intercontinental cup");
    let year = year; // cqw12: the Intercontinental Cup of the game's year
    build_two_club_cup(Cup(this), year, record, 0x971250, 3, 3, 0x92b8b0, 0xad5848)
}

/// Comparer for `construct_world_club_cup`.
pub(crate) unsafe extern "thiscall" fn compare_construct_world_club_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    compare_intercontinental_cup_ctor("construct_world_club_cup", this, year, record, 0x971250, || o(this, year, record))
}
