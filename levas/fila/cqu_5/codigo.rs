// ---------- batch %%LEVA%%: the English cup constructors: the FA Cup, the League Cup and the Charity Shield (eng_fa_cup.cpp, eng_cc_cup.cpp, eng_charity.cpp) ----------

/// Builds the cup's round table through its own virtual `comp_vtable::CREATE_SCHEDULE` (create_round_table(−1,
/// &`cup::ROUND_COUNT`, &`cup::STAGE_LABEL`, null): push 0; push &+0x3a; push &+0x3c; push −1; mov ecx, this ->
/// thiscall, ret 0x10) and stores it in `cup::ROUND_TABLE`.
pub(crate) unsafe fn eng_fa_cup_create_rounds(this: Cup) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = f(this.addr(), 0xffff_ffff, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0);
    this.set_round_table(t);
}

/// Creates the cup's statistics at Comp +8 (operator new `exe::crt_operator_new` of 0x5ce, cdecl, and
/// Comp_Stats::Comp_Stats `exe::new_comp_stats`: push this; mov ecx, new -> thiscall, ret 4; the original's SEH frame
/// covers only that new; 0 without memory). Returns whether they were created.
pub(crate) unsafe fn eng_fa_cup_create_comp_stats(this: Cup) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    write_usize(this.addr() + 8, s); // Comp +8: the Comp_Stats object
    s != 0
}

/// The plan the comparer checks after the original constructor: the return value (this), the vtable, the competition
/// record (Comp +4) and `cup::YEAR`; on a new cup (`cup::STAGE_INDEX` = `stage_index::MAIN`), every fixed field the
/// constructor writes (offset, width, value) and the round table and statistics created. `None` = as planned.
pub(crate) unsafe fn eng_fa_cup_check_constructor_plan(this: Cup, r: usize, year: u32, record: usize, vtable: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
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

/// The body of `construct_eng_fa_cup` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_eng_fa_cup_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x969b28); // the vtable
    write_u16(this.addr() + cup::YEAR, year as u16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    this.set_stage_count(0);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_club_sides(1);
    this.set_discipline_index(discipline_rule::ENGLAND);
    this.set_run_by_stages(0);
    this.set_season_set_up(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_5 fa cup new");
    write_u8(this.addr() + eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED, 0);
    write_u8(this.addr() + eng_fa_cup::FIFTH_ROUND_ENTRY, 1);
    write_u8(this.addr() + eng_fa_cup::EXEMPTION_COUNT, 0);
    this.set_team_count(0x98);
    let v = exe::crt_calloc(0x98, 6);
    this.set_team_table(v);
    if v == 0 {
        show_error_at(0x9c42cc, 0x63);
        return this.addr();
    }
    this.set_prize_fund(0x1e8480);
    eng_fa_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r < 0 {
        show_error_at(0x9c42cc, 0x71);
        return this.addr();
    }
    if !eng_fa_cup_create_comp_stats(this) {
        show_error_at(0x9c42cc, 0x79);
        return this.addr();
    }
    this.addr()
}

/// The fixed fields `construct_eng_fa_cup` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const FA_CUP_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 13] = [(cup::PRIZE_FUND, 4, 0x1e8480), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::TEAM_COUNT, 2, 0x98), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SEASON_SET_UP, 4, 0x0), (cup::DISCIPLINE_INDEX, 1, 0x9), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0), (eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED, 1, 0x0), (eng_fa_cup::EXEMPTION_COUNT, 1, 0x0), (eng_fa_cup::FIFTH_ROUND_ENTRY, 1, 0x1)];

/// The constructor of the FA Cup: with a table of 152 entrants (6-byte records, zeroed); type 1 (clubs of one nation),
/// 152 teams, prize fund of 2,000,000, bench and substitutions from the base, discipline rule 9; on a new cup, in
/// order: the rounds (the cup's own virtual `comp_vtable::CREATE_SCHEDULE`, into `cup::ROUND_TABLE`); the tie tree
/// (Cup::map_fixture_tree, `map_fixture_tree`, 0x518790, thiscall, ret 0; the error box of line 0x71 if it returns a
/// negative value); the statistics at Comp +8 (the error box of line 0x79 without them). Over the base Cup::Cup
/// (`exe::new_cup`, thiscall, ret 0) it writes the fixed fields in the original's order (the vtable 0x969b28, the
/// competition record at Comp +4, `cup::YEAR` and those of `FA_CUP_CONSTRUCTOR_FIELDS`); if the saved state loads
/// (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this -> thiscall, ret 4; returns ≠ 0) it returns this. Each step
/// that fails shows its own error box (`show_error_at`) and returns this. Returns this.
/// Original: `EngFaCup::EngFaCup` (0x56f040, eng_fa_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_eng_fa_cup(this: usize, year: u32, record: usize) -> usize {
    construct_eng_fa_cup_body(Cup(this), year, record) // cqu5: the FA Cup
}

/// Comparer for `construct_eng_fa_cup`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`eng_fa_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_eng_fa_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, year, record);
    let error = eng_fa_cup_check_constructor_plan(Cup(this), r, year, record, 0x969b28, &FA_CUP_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_eng_fa_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of `construct_eng_cc_cup` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_eng_cc_cup_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x969934); // the vtable
    write_u16(this.addr() + cup::YEAR, year as u16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    this.set_stage_count(0);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_club_sides(1);
    this.set_discipline_index(discipline_rule::ENGLAND);
    this.set_run_by_stages(0);
    this.set_season_set_up(0);
    write_u8(this.addr() + eng_cc_cup::EUROPEAN_ENTRANT_COUNT, 0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_5 league cup new");
    this.set_team_count(0x5c);
    let v = exe::crt_calloc(0x5c, 6);
    this.set_team_table(v);
    if v == 0 {
        show_error_at(0x9c4224, 0x50);
        return this.addr();
    }
    this.set_prize_fund(0xf4240);
    eng_fa_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r < 0 {
        show_error_at(0x9c4224, 0x5e);
        return this.addr();
    }
    if !eng_fa_cup_create_comp_stats(this) {
        show_error_at(0x9c4224, 0x65);
        return this.addr();
    }
    this.addr()
}

/// The fixed fields `construct_eng_cc_cup` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const LEAGUE_CUP_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 11] = [(cup::PRIZE_FUND, 4, 0xf4240), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::TEAM_COUNT, 2, 0x5c), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SEASON_SET_UP, 4, 0x0), (cup::DISCIPLINE_INDEX, 1, 0x9), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0), (eng_cc_cup::EUROPEAN_ENTRANT_COUNT, 1, 0x0)];

/// The constructor of the English League Cup: with a table of 92 entrants (6-byte records, zeroed); type 1 (clubs of
/// one nation), 92 teams, prize fund of 1,000,000, bench and substitutions from the base, discipline rule 9; on a new
/// cup, in order: the rounds (the cup's own virtual `comp_vtable::CREATE_SCHEDULE`, into `cup::ROUND_TABLE`); the tie
/// tree (Cup::map_fixture_tree, `map_fixture_tree`, 0x518790, thiscall, ret 0; the error box of line 0x5e if it returns
/// a negative value); the statistics at Comp +8 (the error box of line 0x65 without them). Over the base Cup::Cup
/// (`exe::new_cup`, thiscall, ret 0) it writes the fixed fields in the original's order (the vtable 0x969934, the
/// competition record at Comp +4, `cup::YEAR` and those of `LEAGUE_CUP_CONSTRUCTOR_FIELDS`); if the saved state loads
/// (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this -> thiscall, ret 4; returns ≠ 0) it returns this. Each step
/// that fails shows its own error box (`show_error_at`) and returns this. Returns this.
/// Original: `EngCcCup::EngCcCup` (0x56c2a0, eng_cc_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_eng_cc_cup(this: usize, year: u32, record: usize) -> usize {
    construct_eng_cc_cup_body(Cup(this), year, record) // cqu5: the League Cup
}

/// Comparer for `construct_eng_cc_cup`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`eng_fa_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_eng_cc_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%)(this, year, record);
    let error = eng_fa_cup_check_constructor_plan(Cup(this), r, year, record, 0x969934, &LEAGUE_CUP_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_eng_cc_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of `construct_eng_charity` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_eng_charity_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x9699d4); // the vtable
    write_u16(this.addr() + cup::YEAR, year as u16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    this.set_stage_count(0);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_club_sides(1);
    this.set_substitutes_named(7);
    this.set_substitutions_allowed(5);
    this.set_discipline_index(discipline_rule::ENGLAND);
    this.set_run_by_stages(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_5 charity shield new");
    let select_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x56d780usize); // EngCharity::select_teams
    let r = select_teams(this.addr());
    if r < 0 {
        show_error_at(0x9c425c, 0x43);
        return this.addr();
    }
    this.set_prize_fund(0x493e0);
    eng_fa_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r < 0 {
        show_error_at(0x9c425c, 0x51);
        return this.addr();
    }
    if !eng_fa_cup_create_comp_stats(this) {
        show_error_at(0x9c425c, 0x58);
        return this.addr();
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize); // Cup::rank_teams
    rank_teams(this.addr());
    this.addr()
}

/// The fixed fields `construct_eng_charity` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const CHARITY_SHIELD_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 10] = [(cup::PRIZE_FUND, 4, 0x493e0), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x5), (cup::DISCIPLINE_INDEX, 1, 0x9), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The constructor of the Charity Shield: with no entrant table of its own; type 1 (clubs of one nation), prize fund of
/// 300,000, 7 substitutes on the bench and 5 substitutions, discipline rule 9; on a new cup, in order: the two clubs
/// (EngCharity::select_teams, `eng_charity_select_teams`, 0x56d780, thiscall, ret 0; the error box of line 0x43 if it
/// returns a negative value); the rounds (the cup's own virtual `comp_vtable::CREATE_SCHEDULE`, into
/// `cup::ROUND_TABLE`); the tie tree (Cup::map_fixture_tree, `map_fixture_tree`, 0x518790, thiscall, ret 0; the error
/// box of line 0x51 if it returns a negative value); the statistics at Comp +8 (the error box of line 0x58 without
/// them); the final ranking (Cup::rank_teams, `cup_rank_teams`, 0x5223a0, thiscall, ret 0, unchecked). Over the base
/// Cup::Cup (`exe::new_cup`, thiscall, ret 0) it writes the fixed fields in the original's order (the vtable 0x9699d4,
/// the competition record at Comp +4, `cup::YEAR` and those of `CHARITY_SHIELD_CONSTRUCTOR_FIELDS`); if the saved state
/// loads (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this -> thiscall, ret 4; returns ≠ 0) it returns this. Each
/// step that fails shows its own error box (`show_error_at`) and returns this. Returns this.
/// Original: `EngCharity::EngCharity` (0x56d380, eng_charity.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_eng_charity(this: usize, year: u32, record: usize) -> usize {
    construct_eng_charity_body(Cup(this), year, record) // cqu5: the Charity Shield
}

/// Comparer for `construct_eng_charity`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`eng_fa_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_eng_charity(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%)(this, year, record);
    let error = eng_fa_cup_check_constructor_plan(Cup(this), r, year, record, 0x9699d4, &CHARITY_SHIELD_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_eng_charity({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}
