// ---------- batch %%LEVA%%: the constructors of the national team tournaments: the African Nations Cup, the Copa América and the Oceania Nations Cup (african_nations.cpp, goldcup.cpp, oceania_nations.cpp) ----------

/// The remainder of (year − 1996) by 2, signed (as the original's `and 0x80000001` with its sign adjustment).
fn nations_cup_year_parity(year: i16) -> i32 {
    (year as i32 - 0x7cc) % 2
}

/// The edition year: from `start`, goes up one year at a time while the remainder of (year − 1996) by 2 is not
/// `remainder`.
fn nations_cup_edition(start: i16, remainder: i32) -> i16 {
    let mut year = start;
    while nations_cup_year_parity(year) != remainder {
        year = year.wrapping_add(1);
    }
    year
}

/// A national team tournament built on the League base (the same body in the three constructors).
///
/// League::League (`exe::new_league`, mov ecx -> thiscall, ret 0); the competition record (`league::COMPETITION`), the
/// vtable, `club_sides` 4, `upper_division_id` and `lower_division_id` −1, `discipline_rule` 0, the edition `year`
/// (word) and `importance` 0xf. If the saved state loads (`exe::league_load_from_temp_file(this, 1)`: push 1; mov ecx ->
/// thiscall, ret 4; returns nonzero), returns this.
///
/// Otherwise: `league_flags` 40, `active` 0, `stage_index` 0xff, `current_stage` −1, `stage_count` = `stages` and the
/// stage vector (`exe::crt_malloc` of 4 × stages: push; add esp, 4 -> cdecl, unchecked); the rules (the routine
/// `rules`, thiscall, ret 0; when it returns 0, the error box `rows.0` of source `file` and this); the statistics
/// (operator new `exe::crt_operator_new` of 0x5ce and Comp_Stats::Comp_Stats `exe::new_comp_stats(new, this)`: push
/// this; mov ecx, new -> thiscall, ret 4) in `comp_stats` (null: the box `rows.1`). Returns this.
#[allow(clippy::too_many_arguments)]
unsafe fn build_nations_tournament(this: League, year: i16, record: usize, vtable: usize, stages: u32, rules: usize, file: usize, rows: (u32, u32)) -> usize {
    let set_up_rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(rules);
    exe::new_league(this);
    write_u32(this.addr() + league::COMPETITION, record as u32);
    write_u32(this.addr(), vtable as u32);
    this.set_club_sides(4);
    write_u32(this.addr() + league::UPPER_DIVISION_ID, 0xffff_ffff);
    write_u32(this.addr() + league::LOWER_DIVISION_ID, 0xffff_ffff);
    this.set_discipline_rule(0);
    this.set_year(year);
    this.set_importance(0xf);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    this.set_league_flags(0x28);
    write_u32(this.addr() + league::ACTIVE, 0);
    this.set_stage_index(stage_index::MAIN);
    write_u32(this.addr() + league::CURRENT_STAGE, 0xffff_ffff);
    write_u32(this.addr() + league::STAGE_COUNT, stages);
    write_u32(this.addr() + league::STAGES, exe::crt_malloc(stages * 4) as u32);
    if set_up_rules(this.addr()) == 0 {
        show_error_at(file, rows.0);
        return this.addr();
    }
    let p = exe::crt_operator_new(0x5ce);
    let e = if p != 0 { exe::new_comp_stats(p, this) } else { 0 };
    write_u32(this.addr() + league::COMP_STATS, e as u32);
    if e == 0 {
        show_error_at(file, rows.1);
    }
    this.addr()
}

/// The African Nations Cup at career creation: the edition is the year after the game's, moved to the first even year
/// (every two years from 1996: in a 2001 game, 2002); vtable 0x967270, 4 stages, the rules
/// AfricanNations::set_up_rules (0x401ab0) and the error boxes of lines 0x58 and 0x5f (`build_nations_tournament`).
/// The original's SEH frame only matters for exceptions in the statistics constructor.
/// Original: `AfricanNations::AfricanNations` (0x401000, african_nations.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_african_nations(this: usize, year: u32, record: usize) -> usize {
    covered!("cqw_10 african nations cup");
    let edition = nations_cup_edition((year as i16).wrapping_add(1), 0); // cqw10: the African Nations Cup of the next year, even
    build_nations_tournament(League(this), edition, record, 0x967270, 4, 0x401ab0, 0x9870f8, (0x58, 0x5f))
}

/// Shared comparer of the three constructors. They allocate the stages and the statistics, so they cannot run twice:
/// only the original runs, then the plan is checked: it returns this, the vtable, the record at +4
/// (`league::COMPETITION`), the edition year and `importance` 0xf.
unsafe fn compare_nations_tournament_ctor(name: &str, this: League, record: usize, vtable: usize, year: i16, o: impl FnOnce() -> usize) -> usize {
    let r = o();
    let read = this.year();
    let ok = r == this.addr() && read_usize(this.addr()) == vtable && read_usize(this.addr() + league::COMPETITION) == record && read == year && this.importance() == 0xf;
    SCOREBOARD.record(ok, || format!("{}({:#x}): year {} expected {}, vtable {:#x}", name, this.addr(), read, year, read_usize(this.addr())));
    r
}

/// Comparer for `construct_african_nations`.
pub(crate) unsafe extern "thiscall" fn compare_construct_african_nations(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_nations_tournament_ctor("construct_african_nations", League(this), record, 0x967270, nations_cup_edition((year as i16).wrapping_add(1), 0), || o(this, year, record))
}

/// The Copa América at career creation: the edition is the game's own year, moved to the first odd year (1997, 1999,
/// 2001...: in a 2001 game, July 2001); vtable 0x96b400, 3 stages, the rules GoldCup::set_up_rules (0x5e12d0) and the
/// error boxes of lines 0x56 and 0x5d (`build_nations_tournament`).
/// Original: `GoldCup::GoldCup` (0x5e0660, goldcup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_goldcup(this: usize, year: u32, record: usize) -> usize {
    covered!("cqw_10 copa america");
    let edition = nations_cup_edition(year as i16, 1); // cqw10: the Copa América in an odd year
    build_nations_tournament(League(this), edition, record, 0x96b400, 3, 0x5e12d0, 0x9cd640, (0x56, 0x5d))
}

/// Comparer for `construct_goldcup`.
pub(crate) unsafe extern "thiscall" fn compare_construct_goldcup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    compare_nations_tournament_ctor("construct_goldcup", League(this), record, 0x96b400, nations_cup_edition(year as i16, 1), || o(this, year, record))
}

/// The Oceania Nations Cup at career creation: the edition is the game's year, never before 2002, moved to the first
/// even year; vtable 0x96e0cc, 2 stages, the rules OceaniaNations::set_up_rules (0x799690) and the error boxes of lines
/// 0x57 and 0x5e (`build_nations_tournament`).
/// Original: `OceaniaNations::OceaniaNations` (0x7988b0, oceania_nations.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_oceania_nations(this: usize, year: u32, record: usize) -> usize {
    covered!("cqw_10 oceania nations cup");
    let edition = nations_cup_edition((year as i16).max(0x7d2), 0); // cqw10: the Oceania Nations Cup from 2002 on
    build_nations_tournament(League(this), edition, record, 0x96e0cc, 2, 0x799690, 0xa6a0ec, (0x57, 0x5e))
}

/// Comparer for `construct_oceania_nations`.
pub(crate) unsafe extern "thiscall" fn compare_construct_oceania_nations(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    compare_nations_tournament_ctor("construct_oceania_nations", League(this), record, 0x96e0cc, nations_cup_edition((year as i16).max(0x7d2), 0), || o(this, year, record))
}
