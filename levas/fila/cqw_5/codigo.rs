// ---------- batch %%LEVA%%: the constructors of the Asian Super Cup, the Interamerican Cup, the Mercosur Cup and the MLS All-Star Game (asia_super_cup.cpp, con_merc_cup.cpp, inter_amer_cup.cpp, usa_mls_all_stars.cpp) ----------

/// Builds a new cup's round table through virtual `comp_vtable::CREATE_SCHEDULE` and stores it in `round_table()`.
///
/// The call is `create_schedule(−1, &round_count, &stage_label, 0)` (push 0; push &+0x3a; push &+0x3c; push −1; mov
/// ecx, this -> thiscall, ret 0x10): stage −1, the rounds into `cup::ROUND_COUNT` and the extra into
/// `cup::STAGE_LABEL`, no is_league output.
unsafe fn build_cup_round_table_by_vtable(this: Cup) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = f(this.addr(), 0xffff_ffff, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0);
    write_u32(this.addr() + cup::ROUND_TABLE, t as u32);
}

/// Creates the competition's statistics and stores them at +8 (the Comp base field, `league::COMP_STATS`); returns
/// whether they were created.
///
/// Operator new `exe::crt_operator_new` of 0x5ce bytes (push; add esp, 4 -> cdecl) and, when it is not null,
/// Comp_Stats::Comp_Stats `exe::new_comp_stats` (push this; mov ecx, new -> thiscall, ret 4). The original's SEH frame
/// covers only this `new`.
unsafe fn create_comp_stats_at_8(this: usize) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    write_u32(this + 8, s as u32);
    s != 0
}

/// The plan the comparer checks after the original constructor ran; `None` when it holds, otherwise the difference.
///
/// Always: the return value is `this`, the vtable, the competition record at +4 and the year (`cup::YEAR`, word). On a
/// new cup (`cup::STAGE_INDEX` = 0xff), every fixed field the constructor writes (`fields`: offset, width in bytes,
/// value), the round table (`cup::ROUND_TABLE`, when `rounds`) and the statistics at +8 must be set.
unsafe fn cup_constructor_plan_mismatch(this: usize, r: usize, year: u32, record: usize, vt: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    if r != this || read_usize(this) != vt || read_usize(this + 4) != record || read_u16(this + cup::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this), read_usize(this + 4), read_u16(this + cup::YEAR)));
    }
    if read_u8(this + cup::STAGE_INDEX) != stage_index::MAIN {
        return None;
    }
    for &(o, t, v) in fields {
        let x = match t {
            1 => read_u8(this + o) as u32,
            2 => read_u16(this + o) as u32,
            _ => read_u32(this + o),
        };
        if x != v {
            return Some(format!("+{:#x} = {:#x}, the constructor writes {:#x}", o, x, v));
        }
    }
    if (rounds && read_usize(this + cup::ROUND_TABLE) == 0) || read_usize(this + 8) == 0 {
        return Some(format!("rounds {:#x}, statistics {:#x}", read_usize(this + cup::ROUND_TABLE), read_usize(this + 8)));
    }
    None
}

/// The constructor of the Asian Super Cup (`AsiaSuperCup::AsiaSuperCup(year, record)`), run once, at career creation.
///
/// On the Cup base (Cup::Cup `exe::new_cup`, thiscall, ret 0): the competition record at +4, the vtable 0x9676c8, the
/// year (`set_year`), `current_round` 0, `stage_index` 0xff (the main competition), `stage_count` 0, `current_stage` −1,
/// `club_sides` 2 (clubs of several nations), 7 `substitutes_named`, 3 `substitutions_allowed`, `discipline_index` 41
/// and `run_by_stages` 0; the prize fund (+0x28) stays as the base left it. If the saved state loads
/// (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this -> thiscall, ret 4; returns nonzero), returns this.
///
/// On a new cup, in order, each step with its error box (`show_error_at`, source 0x987be0) and returning this when it
/// fails: the two clubs (AsiaSuperCup::select_teams 0x410a50, thiscall, ret 0; the box of line 0x3e when it returns a
/// negative value); the round table (`build_cup_round_table_by_vtable`); the tie tree (Cup::map_fixture_tree 0x518790,
/// thiscall, ret 0; the box of line 0x4c when negative); the statistics at +8 (`create_comp_stats_at_8`; the box of
/// line 0x53 without them); the final ranking (Cup::rank_teams 0x5223a0, thiscall, ret 0, unchecked). Returns this.
unsafe fn build_asian_super_cup(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_u32(this.addr() + 4, record as u32);
    write_u32(this.addr(), 0x9676c8);
    this.set_year(year as u16 as i16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_u32(this.addr() + cup::STAGE_COUNT, 0);
    write_u32(this.addr() + cup::CURRENT_STAGE, 0xffff_ffff);
    this.set_club_sides(2);
    this.set_substitutes_named(7);
    this.set_substitutions_allowed(3);
    this.set_discipline_index(0x29);
    this.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load_from_disk(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqw_5 asian super cup new");
    let select_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x410a50usize); // AsiaSuperCup::select_teams
    if select_teams(this.addr()) < 0 {
        show_error_at(0x987be0, 0x3e);
        return this.addr();
    }
    build_cup_round_table_by_vtable(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    if map_fixture_tree(this.addr()) < 0 {
        show_error_at(0x987be0, 0x4c);
        return this.addr();
    }
    if !create_comp_stats_at_8(this.addr()) {
        show_error_at(0x987be0, 0x53);
        return this.addr();
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize); // Cup::rank_teams
    rank_teams(this.addr());
    this.addr()
}

/// The fixed fields the Asian Super Cup constructor writes (offset, width, value), checked by the comparer.
const ASIAN_SUPER_CUP_CTOR_FIELDS: [(usize, u8, u32); 9] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x2), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x29), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The Asian Super Cup at career creation: the year and the database competition; a new cup has two clubs of several
/// nations, 7 substitutes on the bench and 3 substitutions, discipline rule 41, no prize fund of its own, the rounds,
/// the tie tree, the statistics and the final ranking (see `build_asian_super_cup`).
/// Original: `AsiaSuperCup::AsiaSuperCup` (0x410650, asia_super_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_asia_super_cup(this: usize, year: u32, record: usize) -> usize {
    build_asian_super_cup(Cup(this), year, record) // cqw5: the Asian Super Cup
}

/// Comparer for `construct_asia_super_cup`: only the original runs (it allocates, draws and reads the save file),
/// then the plan is checked (`cup_constructor_plan_mismatch`).
pub(crate) unsafe extern "thiscall" fn compare_construct_asia_super_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, year, record);
    let error = cup_constructor_plan_mismatch(this, r, year, record, 0x9676c8, &ASIAN_SUPER_CUP_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_asia_super_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The constructor of the Interamerican Cup (`InterAmerCup::InterAmerCup(year, record)`), run once, at career creation.
///
/// On the Cup base (Cup::Cup `exe::new_cup`, thiscall, ret 0): the year (`set_year`), the competition record at +4,
/// the vtable 0x96bad8, `current_round` 0, `stage_index` 0xff, `stage_count` 0, `current_stage` −1, `club_sides` 2
/// (clubs of several nations), +0x52 = 2, 7 `substitutes_named`, 3 `substitutions_allowed`, `discipline_index` 0xff and
/// `run_by_stages` 0; the prize fund (+0x28) stays as the base left it. If the saved state loads (Cup::load_from_disk
/// 0x51fc00: push 1; mov ecx, this -> thiscall, ret 4; returns nonzero), returns this.
///
/// On a new cup, in order, each step with its error box (source 0x9e9d94) and returning this when it fails: the two
/// finalists (InterAmerCup::select_teams 0x632480, thiscall, ret 0; the box of line 0x41 when negative); the round
/// table (`build_cup_round_table_by_vtable`); the tie tree (Cup::map_fixture_tree 0x518790; the box of line 0x4f when
/// negative); the statistics at +8 (the box of line 0x56 without them); the final ranking (Cup::rank_teams 0x5223a0,
/// unchecked). Returns this.
unsafe fn build_interamerican_cup(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    this.set_year(year as u16 as i16);
    write_u32(this.addr() + 4, record as u32);
    write_u32(this.addr(), 0x96bad8);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_u32(this.addr() + cup::STAGE_COUNT, 0);
    write_u32(this.addr() + cup::CURRENT_STAGE, 0xffff_ffff);
    this.set_club_sides(2);
    write_u8(this.addr() + 0x52, 2);
    this.set_substitutes_named(7);
    this.set_substitutions_allowed(3);
    this.set_discipline_index(0xff);
    this.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load_from_disk(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqw_5 interamerican cup new");
    let select_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x632480usize); // InterAmerCup::select_teams
    if select_teams(this.addr()) < 0 {
        show_error_at(0x9e9d94, 0x41);
        return this.addr();
    }
    build_cup_round_table_by_vtable(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    if map_fixture_tree(this.addr()) < 0 {
        show_error_at(0x9e9d94, 0x4f);
        return this.addr();
    }
    if !create_comp_stats_at_8(this.addr()) {
        show_error_at(0x9e9d94, 0x56);
        return this.addr();
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize); // Cup::rank_teams
    rank_teams(this.addr());
    this.addr()
}

/// The fixed fields the Interamerican Cup constructor writes (offset, width, value), checked by the comparer.
const INTERAMERICAN_CUP_CTOR_FIELDS: [(usize, u8, u32); 10] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x2), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0xff), (0x52, 1, 0x2), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The Interamerican Cup at career creation: the year and the database competition; a new cup has the two finalists
/// (clubs of several nations), 7 substitutes on the bench and 3 substitutions, discipline rule 255, no prize fund of its
/// own, the rounds, the tie tree, the statistics and the final ranking (see `build_interamerican_cup`).
/// Original: `InterAmerCup::InterAmerCup` (0x632080, inter_amer_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_inter_amer_cup(this: usize, year: u32, record: usize) -> usize {
    build_interamerican_cup(Cup(this), year, record) // cqw5: the Interamerican Cup
}

/// Comparer for `construct_inter_amer_cup`: only the original runs (it allocates, draws and reads the save file),
/// then the plan is checked (`cup_constructor_plan_mismatch`).
pub(crate) unsafe extern "thiscall" fn compare_construct_inter_amer_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%)(this, year, record);
    let error = cup_constructor_plan_mismatch(this, r, year, record, 0x96bad8, &INTERAMERICAN_CUP_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_inter_amer_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The constructor of the Mercosur Cup (`ConMercCup::ConMercCup(year, record)`), run once, at career creation.
///
/// On the League base (League::League `exe::new_league`, thiscall, ret 0): the competition record (`league::COMPETITION`),
/// the vtable 0x968b40, the year (`set_year`), `discipline_rule` 0xff, the drawn club list `con_merc_cup::DRAWN_CLUBS` = 0 and `importance` 0xf;
/// substitutes, substitutions and the prize fund stay as the base left them. If the saved state loads
/// (`exe::league_load_from_temp_file`: push 1; mov ecx, this -> thiscall, ret 4; returns nonzero), returns this.
///
/// On a new cup, in order: `league_flags` 0x28, `stage_index` 0xff, `current_stage` −1, `stage_count` 4 and the stage
/// vector (`exe::crt_malloc` of 0x10: push; add esp, 4 -> cdecl, unchecked); the rules (ConMercCup::set_up_rules
/// 0x4c1ed0, thiscall, ret 0; the error box of line 0x4d, source 0x9a7d6c, and this when it returns 0); `club_count` 0
/// and `table` 0; the statistics (`create_comp_stats_at_8`; the box of line 0x59 and this without them); `active` 0.
/// Returns this.
unsafe fn build_mercosur_cup(this: League, year: u32, record: usize) -> usize {
    exe::new_league(this);
    write_u32(this.addr() + league::COMPETITION, record as u32);
    write_u32(this.addr(), 0x968b40);
    this.set_year(year as u16 as i16);
    this.set_discipline_rule(0xff);
    write_u32(this.addr() + con_merc_cup::DRAWN_CLUBS, 0);
    this.set_importance(0xf);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!("cqw_5 mercosur cup new");
    this.set_league_flags(0x28);
    this.set_stage_index(stage_index::MAIN);
    write_u32(this.addr() + league::CURRENT_STAGE, 0xffff_ffff);
    write_u32(this.addr() + league::STAGE_COUNT, 0x4);
    write_u32(this.addr() + league::STAGES, exe::crt_malloc(0x10) as u32);
    let set_up_rules: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x4c1ed0usize); // ConMercCup::set_up_rules
    if set_up_rules(this.addr()) == 0 {
        show_error_at(0x9a7d6c, 0x4d);
        return this.addr();
    }
    this.set_club_count(0);
    write_u32(this.addr() + league::TABLE, 0);
    if !create_comp_stats_at_8(this.addr()) {
        show_error_at(0x9a7d6c, 0x59);
        return this.addr();
    }
    write_u32(this.addr() + league::ACTIVE, 0);
    this.addr()
}

/// The fixed fields the Mercosur Cup constructor writes (offset, width, value), checked by the comparer.
const MERCOSUR_CUP_CTOR_FIELDS: [(usize, u8, u32); 10] = [(league::STAGE_COUNT, 4, 0x4), (league::CURRENT_STAGE, 4, 0xffffffff), (league::CLUB_COUNT, 2, 0x0), (league::STAGE_INDEX, 1, 0xff), (league::ACTIVE, 4, 0x0), (league::DISCIPLINE_RULE, 1, 0xff), (league::IMPORTANCE, 1, 0xf), (league::TABLE, 4, 0x0), (league::LEAGUE_FLAGS, 2, 0x28), (con_merc_cup::DRAWN_CLUBS, 4, 0x0)];

/// The Mercosur Cup at career creation: the year and the database competition, discipline rule 255 and importance 15;
/// a new cup has league flags 0x28, four stages, the rules and the statistics (see `build_mercosur_cup`).
/// Original: `ConMercCup::ConMercCup` (0x4c16f0, con_merc_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_con_merc_cup(this: usize, year: u32, record: usize) -> usize {
    build_mercosur_cup(League(this), year, record) // cqw5: the Mercosur Cup
}

/// Comparer for `construct_con_merc_cup`: only the original runs (it allocates and reads the save file), then the plan
/// is checked (`cup_constructor_plan_mismatch`, without the round table).
pub(crate) unsafe extern "thiscall" fn compare_construct_con_merc_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%)(this, year, record);
    let error = cup_constructor_plan_mismatch(this, r, year, record, 0x968b40, &MERCOSUR_CUP_CTOR_FIELDS, false);
    SCOREBOARD.record(error.is_none(), || format!("construct_con_merc_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The constructor of the MLS All-Star Game (`UsaMlsAllStars::UsaMlsAllStars(year, record)`), run once, at career
/// creation.
///
/// On the Cup base (Cup::Cup `exe::new_cup`, thiscall, ret 0): the competition record at +4, the vtable 0x9708f8, the
/// year (`set_year`), `current_round` 0, `stage_index` 0xff, `stage_count` 0, `current_stage` −1, `club_sides` 1 (clubs
/// of one nation), `discipline_index` 0xff and `run_by_stages` 0; substitutes, substitutions and the prize fund stay as
/// the base left them. If the saved state loads (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this -> thiscall, ret
/// 4; returns nonzero), returns this.
///
/// On a new cup, in order, each step with its error box (source 0xaa20a0) and returning this when it fails: the two
/// all-star teams (UsaMlsAllStars::set_up_teams 0x90a540, thiscall, ret 0; the box of line 0x44 when negative); the
/// round table (`build_cup_round_table_by_vtable`); the tie tree (Cup::map_fixture_tree 0x518790; the box of line 0x52
/// when negative); the statistics at +8 (the box of line 0x59 without them). No final ranking. Returns this.
unsafe fn build_mls_all_star_game(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_u32(this.addr() + 4, record as u32);
    write_u32(this.addr(), 0x9708f8);
    this.set_year(year as u16 as i16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_u32(this.addr() + cup::STAGE_COUNT, 0);
    write_u32(this.addr() + cup::CURRENT_STAGE, 0xffff_ffff);
    this.set_club_sides(1);
    this.set_discipline_index(0xff);
    this.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load_from_disk(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqw_5 mls all-star game new");
    let set_up_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x90a540usize); // UsaMlsAllStars::set_up_teams
    if set_up_teams(this.addr()) < 0 {
        show_error_at(0xaa20a0, 0x44);
        return this.addr();
    }
    build_cup_round_table_by_vtable(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    if map_fixture_tree(this.addr()) < 0 {
        show_error_at(0xaa20a0, 0x52);
        return this.addr();
    }
    if !create_comp_stats_at_8(this.addr()) {
        show_error_at(0xaa20a0, 0x59);
        return this.addr();
    }
    this.addr()
}

/// The fixed fields the MLS All-Star Game constructor writes (offset, width, value), checked by the comparer.
const MLS_ALL_STAR_GAME_CTOR_FIELDS: [(usize, u8, u32); 7] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::DISCIPLINE_INDEX, 1, 0xff), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The MLS All-Star Game at career creation: the year and the database competition; a new game has the two all-star
/// teams (clubs of one nation), discipline rule 255, no prize fund of its own, the rounds, the tie tree and the
/// statistics (see `build_mls_all_star_game`).
/// Original: `UsaMlsAllStars::UsaMlsAllStars` (0x90a150, usa_mls_all_stars.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_usa_mls_all_stars(this: usize, year: u32, record: usize) -> usize {
    build_mls_all_star_game(Cup(this), year, record) // cqw5: the MLS All-Star Game
}

/// Comparer for `construct_usa_mls_all_stars`: only the original runs (it allocates, draws and reads the save file),
/// then the plan is checked (`cup_constructor_plan_mismatch`).
pub(crate) unsafe extern "thiscall" fn compare_construct_usa_mls_all_stars(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I3%%)(this, year, record);
    let error = cup_constructor_plan_mismatch(this, r, year, record, 0x9708f8, &MLS_ALL_STAR_GAME_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_usa_mls_all_stars({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}
