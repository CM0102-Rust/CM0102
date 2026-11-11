// ---------- batch %%LEVA%%: the French cup constructors: the Coupe de France, the League Cup and the Super Cup (fra_cup.cpp, fra_lge_cup.cpp, fra_super.cpp) ----------

/// Builds the cup's round table through its own virtual `comp_vtable::CREATE_SCHEDULE` (create_round_table(−1,
/// &`cup::ROUND_COUNT`, &`cup::STAGE_LABEL`, null): push 0; push &+0x3a; push &+0x3c; push −1; mov ecx, this ->
/// thiscall, ret 0x10) and stores it in `cup::ROUND_TABLE`.
pub(crate) unsafe fn fra_cup_create_rounds(this: Cup) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = f(this.addr(), 0xffff_ffff, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0);
    this.set_round_table(t);
}

/// Creates the cup's statistics at Comp +8 (operator new `exe::crt_operator_new` of 0x5ce, cdecl, and
/// Comp_Stats::Comp_Stats `exe::new_comp_stats`: push this; mov ecx, new -> thiscall, ret 4; the original's SEH frame
/// covers only that new; 0 without memory). Returns whether they were created.
pub(crate) unsafe fn fra_cup_create_comp_stats(this: Cup) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    write_usize(this.addr() + 8, s); // Comp +8: the Comp_Stats object
    s != 0
}

/// The plan the comparer checks after the original constructor: the return value (this), the vtable, the competition
/// record (Comp +4) and `cup::YEAR`; on a new cup (`cup::STAGE_INDEX` = `stage_index::MAIN`), every fixed field the
/// constructor writes (offset, width, value) and the round table and statistics created. `None` = as planned.
pub(crate) unsafe fn fra_cup_check_constructor_plan(this: Cup, r: usize, year: u32, record: usize, vtable: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    if r != this.addr() || read_usize(this.addr()) != vtable || read_usize(this.addr() + 4) != record || read_u16(this.addr() + cup::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this.addr()), read_usize(this.addr() + 4), read_u16(this.addr() + cup::YEAR)));
    }
    if this.stage_index() != stage_index::MAIN {
        return None;
    }
    for &(o, w, v) in fields {
        let x = match w {
            1 => read_u8(this.addr() + o) as u32,
            2 => read_u16(this.addr() + o) as u32,
            _ => read_u32(this.addr() + o),
        };
        if x != v {
            return Some(format!("+{:#x} = {:#x}, the constructor writes {:#x}", o, x, v));
        }
    }
    if (rounds && this.round_table() == 0) || read_usize(this.addr() + 8) == 0 {
        return Some(format!("rounds {:#x}, statistics {:#x}", this.round_table(), read_usize(this.addr() + 8)));
    }
    None
}

/// The body of `construct_fra_cup` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_fra_cup_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_u16(this.addr() + cup::YEAR, year as u16);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x96a864); // the vtable
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_stage_count(0);
    this.set_club_sides(1);
    this.set_substitutes_named(3);
    this.set_substitutions_allowed(3);
    this.set_discipline_index(discipline_rule::FRANCE);
    this.set_run_by_stages(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_7 coupe de france new");
    let collect_entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5baa50usize); // FraCup::collect_entrants
    let r = collect_entrants(this.addr());
    if r == 0 {
        show_error_at(0x9ccb28, 0x4c);
        return this.addr();
    }
    this.set_prize_fund(0xf4240);
    fra_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r == 0 {
        show_error_at(0x9ccb28, 0x5b);
        return this.addr();
    }
    if !fra_cup_create_comp_stats(this) {
        show_error_at(0x9ccb28, 0x62);
        return this.addr();
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize); // Cup::rank_teams
    rank_teams(this.addr());
    this.addr()
}

/// The fixed fields `construct_fra_cup` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const COUPE_DE_FRANCE_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 10] = [(cup::PRIZE_FUND, 4, 0xf4240), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x3), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0xc), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The constructor of the Coupe de France: with no entrant table of its own; type 1 (clubs of one nation), prize fund
/// of 1,000,000, 3 substitutes on the bench and 3 substitutions, discipline rule 12; on a new cup, in order: the 222
/// entrants (FraCup::collect_entrants, `fra_cup_collect_entrants`, 0x5baa50, thiscall, ret 0; the error box of line
/// 0x4c if it returns 0); the rounds (the cup's own virtual `comp_vtable::CREATE_SCHEDULE`, into `cup::ROUND_TABLE`);
/// the tie tree (Cup::map_fixture_tree, `map_fixture_tree`, 0x518790, thiscall, ret 0; the error box of line 0x5b if it
/// returns 0); the statistics at Comp +8 (the error box of line 0x62 without them); the final ranking (Cup::rank_teams,
/// `cup_rank_teams`, 0x5223a0, thiscall, ret 0, unchecked). Over the base Cup::Cup (`exe::new_cup`, thiscall, ret 0) it
/// writes the fixed fields in the original's order (the vtable 0x96a864, the competition record at Comp +4, `cup::YEAR`
/// and those of `COUPE_DE_FRANCE_CONSTRUCTOR_FIELDS`); if the saved state loads (Cup::load_from_disk 0x51fc00: push 1;
/// mov ecx, this -> thiscall, ret 4; returns ≠ 0) it returns this. Each step that fails shows its own error box
/// (`show_error_at`) and returns this. Returns this.
/// Original: `FraCup::FraCup` (0x5ba130, fra_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_fra_cup(this: usize, year: u32, record: usize) -> usize {
    construct_fra_cup_body(Cup(this), year, record) // cqu7: the Coupe de France
}

/// Comparer for `construct_fra_cup`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`fra_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_fra_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, year, record);
    let error = fra_cup_check_constructor_plan(Cup(this), r, year, record, 0x96a864, &COUPE_DE_FRANCE_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_fra_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of `construct_fra_lge_cup` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_fra_lge_cup_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_u16(this.addr() + cup::YEAR, year as u16);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x96aad8); // the vtable
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_stage_count(0);
    this.set_club_sides(1);
    this.set_substitutes_named(3);
    this.set_substitutions_allowed(3);
    this.set_discipline_index(discipline_rule::FRANCE);
    this.set_run_by_stages(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_7 french league cup new");
    let collect_entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5bd160usize); // FraLgeCup::collect_entrants
    let r = collect_entrants(this.addr());
    if r == 0 {
        show_error_at(0x9ccb98, 0x49);
        return this.addr();
    }
    this.set_prize_fund(0x7a120);
    fra_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r == 0 {
        show_error_at(0x9ccb98, 0x57);
        return this.addr();
    }
    if !fra_cup_create_comp_stats(this) {
        show_error_at(0x9ccb98, 0x5e);
        return this.addr();
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize); // Cup::rank_teams
    rank_teams(this.addr());
    this.addr()
}

/// The fixed fields `construct_fra_lge_cup` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const FRENCH_LEAGUE_CUP_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 10] = [(cup::PRIZE_FUND, 4, 0x7a120), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x3), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0xc), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The constructor of the French League Cup: with no entrant table of its own; type 1 (clubs of one nation), prize fund
/// of 500,000, 3 substitutes on the bench and 3 substitutions, discipline rule 12; on a new cup, in order: the 54
/// entrants (FraLgeCup::collect_entrants, `fra_lge_cup_collect_entrants`, 0x5bd160, thiscall, ret 0; the error box of
/// line 0x49 if it returns 0); the rounds (the cup's own virtual `comp_vtable::CREATE_SCHEDULE`, into
/// `cup::ROUND_TABLE`); the tie tree (Cup::map_fixture_tree, `map_fixture_tree`, 0x518790, thiscall, ret 0; the error
/// box of line 0x57 if it returns 0); the statistics at Comp +8 (the error box of line 0x5e without them); the final
/// ranking (Cup::rank_teams, `cup_rank_teams`, 0x5223a0, thiscall, ret 0, unchecked). Over the base Cup::Cup
/// (`exe::new_cup`, thiscall, ret 0) it writes the fixed fields in the original's order (the vtable 0x96aad8, the
/// competition record at Comp +4, `cup::YEAR` and those of `FRENCH_LEAGUE_CUP_CONSTRUCTOR_FIELDS`); if the saved state
/// loads (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this -> thiscall, ret 4; returns ≠ 0) it returns this. Each
/// step that fails shows its own error box (`show_error_at`) and returns this. Returns this.
/// Original: `FraLgeCup::FraLgeCup` (0x5bc980, fra_lge_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_fra_lge_cup(this: usize, year: u32, record: usize) -> usize {
    construct_fra_lge_cup_body(Cup(this), year, record) // cqu7: the French League Cup
}

/// Comparer for `construct_fra_lge_cup`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`fra_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_fra_lge_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%)(this, year, record);
    let error = fra_cup_check_constructor_plan(Cup(this), r, year, record, 0x96aad8, &FRENCH_LEAGUE_CUP_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_fra_lge_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of `construct_fra_super` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_fra_super_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x96ace0); // the vtable
    write_u16(this.addr() + cup::YEAR, year as u16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    this.set_stage_count(0);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_club_sides(1);
    this.set_discipline_index(discipline_rule::FRANCE);
    this.set_run_by_stages(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_7 french super cup new");
    let select_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5bf770usize); // FraSuper::select_teams
    let r = select_teams(this.addr());
    if r < 0 {
        show_error_at(0x9ccc48, 0x3f);
        return this.addr();
    }
    fra_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r < 0 {
        show_error_at(0x9ccc48, 0x4d);
        return this.addr();
    }
    if !fra_cup_create_comp_stats(this) {
        show_error_at(0x9ccc48, 0x54);
        return this.addr();
    }
    this.addr()
}

/// The fixed fields `construct_fra_super` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const FRENCH_SUPER_CUP_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 7] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::DISCIPLINE_INDEX, 1, 0xc), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The constructor of the French Super Cup: with no entrant table of its own; type 1 (clubs of one nation), no prize
/// fund (`cup::PRIZE_FUND` stays as the base left it), bench and substitutions from the base, discipline rule 12; on a
/// new cup, in order: the two clubs (FraSuper::select_teams, `fra_super_select_teams`, 0x5bf770, thiscall, ret 0; the
/// error box of line 0x3f if it returns a negative value); the rounds (the cup's own virtual
/// `comp_vtable::CREATE_SCHEDULE`, into `cup::ROUND_TABLE`); the tie tree (Cup::map_fixture_tree, `map_fixture_tree`,
/// 0x518790, thiscall, ret 0; the error box of line 0x4d if it returns a negative value); the statistics at Comp +8
/// (the error box of line 0x54 without them). Over the base Cup::Cup (`exe::new_cup`, thiscall, ret 0) it writes the
/// fixed fields in the original's order (the vtable 0x96ace0, the competition record at Comp +4, `cup::YEAR` and those
/// of `FRENCH_SUPER_CUP_CONSTRUCTOR_FIELDS`); if the saved state loads (Cup::load_from_disk 0x51fc00: push 1; mov ecx,
/// this -> thiscall, ret 4; returns ≠ 0) it returns this. Each step that fails shows its own error box
/// (`show_error_at`) and returns this. Returns this.
/// Original: `FraSuper::FraSuper` (0x5bf320, fra_super.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_fra_super(this: usize, year: u32, record: usize) -> usize {
    construct_fra_super_body(Cup(this), year, record) // cqu7: the French Super Cup
}

/// Comparer for `construct_fra_super`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`fra_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_fra_super(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%)(this, year, record);
    let error = fra_cup_check_constructor_plan(Cup(this), r, year, record, 0x96ace0, &FRENCH_SUPER_CUP_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_fra_super({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}
