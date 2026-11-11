// ---------- %%LEVA%%a leva: the constructors of the Italian super cups and of the US Open Cup (ita_c1_super.cpp, ita_super.cpp, usa_open_cup.cpp) ----------

/// The cup's rounds through its `comp_vtable::CREATE_SCHEDULE` method (push 0; push &`cup::STAGE_LABEL`; push
/// &`cup::ROUND_COUNT`; push −1; mov ecx, this -> thiscall, ret 0x10): the round table, stored in `round_table()`.
unsafe fn ita_super_ctor_create_rounds(this: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let t = f(this, 0xffff_ffff, this + cup::ROUND_COUNT, this + cup::STAGE_LABEL, 0);
    Cup(this).set_round_table(t);
}

/// The statistics (`exe::crt_operator_new`(0x5ce), cdecl; `exe::new_comp_stats`(new, this), thiscall, ret 4) at +8; the
/// original's SEH frame covers only that new. Returns whether they were created.
unsafe fn ita_super_ctor_create_stats(this: usize) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    write_usize(this + 8, s);
    s != 0
}

/// The plan the comparer checks after the original: the return value (this), the vtable, the competition record (+4)
/// and the year (`cup::YEAR`); on a new cup (`stage_index()` = 0xff), each fixed field the constructor writes
/// (offset, width in bytes, value) and the round table and statistics it created. Returns the first difference.
unsafe fn ita_super_ctor_plan_error(this: usize, r: usize, year: u32, record: usize, vtable: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    if r != this || read_usize(this) != vtable || read_usize(this + 4) != record || read_u16(this + cup::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this), read_usize(this + 4), read_u16(this + cup::YEAR)));
    }
    if read_u8(this + cup::STAGE_INDEX) != 0xff {
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

const ITA_SUPER_VTABLE: usize = 0x96c8b8;
const ITA_C1_SUPER_VTABLE: usize = 0x96c1c4;
const USA_OPEN_CUP_VTABLE: usize = 0x970998;

/// The body of the Italian Super Cup constructor (see `construct_ita_super`).
unsafe fn construct_ita_super_body(this: usize, year: u32, record: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    write_usize(this + 4, record);
    write_usize(this, ITA_SUPER_VTABLE);
    cup.set_year(year as i16);
    cup.set_current_round(0);
    cup.set_stage_index(0xff);
    cup.set_stage_count(0);
    cup.set_current_stage(0xffff_ffff);
    cup.set_club_sides(1);
    cup.set_substitutions_allowed(3);
    cup.set_substitutes_named(7);
    cup.set_discipline_index(0x11);
    cup.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("cqv_5 italian super cup new");
    let init_team_list: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x65fa60usize);
    if init_team_list(this) < 0 {
        show_error_at(0x9ea934, 0x43);
        return this;
    }
    ita_super_ctor_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    if map_fixture_tree(this) < 0 {
        show_error_at(0x9ea934, 0x51);
        return this;
    }
    if !ita_super_ctor_create_stats(this) {
        show_error_at(0x9ea934, 0x58);
        return this;
    }
    this
}

/// The fixed fields the Italian Super Cup constructor writes on a new cup (offset, width, value).
const ITA_SUPER_CTOR_FIELDS: [(usize, u8, u32); 9] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x11), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The Italian Super Cup constructor `ItaSuper::ItaSuper(year, record)`, run once at career creation. On the Cup::Cup
/// base (`exe::new_cup`): the competition record at +4, the vtable (0x96c8b8), the year (`cup::YEAR`),
/// `current_round()` = 0, `stage_index()` = 0xff, `stage_count()` = 0, `current_stage()` = −1, `club_sides()` = 1
/// (clubs of one nation), 3 substitutions allowed, 7 substitutes named, discipline rule 17 and `run_by_stages()` = 0;
/// no prize fund (`cup::PRIZE_FUND` stays as the base left it). If the saved state loads (Cup::load_from_disk
/// 0x51fc00, thiscall(this, 1), ret 4; returns ≠ 0), it returns this. On a new cup, in order: the two clubs
/// (ItaSuper::init_team_list 0x65fa60, thiscall, ret 0; the error box of line 0x43 if it returns a negative value); the
/// rounds (`comp_vtable::CREATE_SCHEDULE`, into `round_table()`); the tie tree (Cup::map_fixture_tree 0x518790,
/// thiscall, ret 0; the error box of line 0x51 if it returns a negative value); the statistics at +8 (the error box of
/// line 0x58 without them). Every failure returns this. [the bench, substitutions, discipline and type fields read
/// from the competitions reference, §2.1]
/// Original: `ItaSuper::ItaSuper` (0x65f670, ita_super.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn construct_ita_super(this: usize, year: u32, record: usize) -> usize {
    construct_ita_super_body(this, year, record) // cqv5: the Italian Super Cup
}

/// Runs only the original (it allocates, draws and reads the save file), checking the plan (`ita_super_ctor_plan_error`).
unsafe extern "thiscall" fn compare_construct_ita_super(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, year, record);
    let error = ita_super_ctor_plan_error(this, r, year, record, ITA_SUPER_VTABLE, &ITA_SUPER_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_ita_super({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of the Serie C1 Super Cup constructor (see `construct_ita_c1_super`).
unsafe fn construct_ita_c1_super_body(this: usize, year: u32, record: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    write_usize(this + 4, record);
    write_usize(this, ITA_C1_SUPER_VTABLE);
    cup.set_year(year as i16);
    cup.set_current_round(0);
    cup.set_stage_index(0xff);
    cup.set_stage_count(0);
    cup.set_current_stage(0xffff_ffff);
    cup.set_club_sides(1);
    cup.set_substitutions_allowed(3);
    cup.set_substitutes_named(7);
    cup.set_discipline_index(0x11);
    cup.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("cqv_5 serie c1 super cup new");
    let init_team_list: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63c9d0usize);
    if init_team_list(this) < 0 {
        show_error_at(0x9ea1c8, 0x43);
        return this;
    }
    ita_super_ctor_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    if map_fixture_tree(this) < 0 {
        show_error_at(0x9ea1c8, 0x51);
        return this;
    }
    if !ita_super_ctor_create_stats(this) {
        show_error_at(0x9ea1c8, 0x58);
        return this;
    }
    this
}

/// The fixed fields the Serie C1 Super Cup constructor writes on a new cup (offset, width, value).
const ITA_C1_SUPER_CTOR_FIELDS: [(usize, u8, u32); 9] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x11), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The Serie C1 Super Cup constructor `ItaC1Super::ItaC1Super(year, record)`, run once at career creation. On the
/// Cup::Cup base (`exe::new_cup`): the competition record at +4, the vtable (0x96c1c4), the year (`cup::YEAR`),
/// `current_round()` = 0, `stage_index()` = 0xff, `stage_count()` = 0, `current_stage()` = −1, `club_sides()` = 1
/// (clubs of one nation), 3 substitutions allowed, 7 substitutes named, discipline rule 17 and `run_by_stages()` = 0;
/// no prize fund (`cup::PRIZE_FUND` stays as the base left it). If the saved state loads (Cup::load_from_disk
/// 0x51fc00, thiscall(this, 1), ret 4; returns ≠ 0), it returns this. On a new cup, in order: the two-club list
/// (ItaC1Super::init_team_list 0x63c9d0, thiscall, ret 0; the error box of line 0x43 if it returns a negative value);
/// the rounds (`comp_vtable::CREATE_SCHEDULE`, into `round_table()`); the tie tree (Cup::map_fixture_tree 0x518790,
/// thiscall, ret 0; the error box of line 0x51 if it returns a negative value); the statistics at +8 (the error box of
/// line 0x58 without them). Every failure returns this.
/// Original: `ItaC1Super::ItaC1Super` (0x63c5e0, ita_c1_super.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn construct_ita_c1_super(this: usize, year: u32, record: usize) -> usize {
    construct_ita_c1_super_body(this, year, record) // cqv5: the Serie C1 Super Cup
}

/// Runs only the original (it allocates, draws and reads the save file), checking the plan (`ita_super_ctor_plan_error`).
unsafe extern "thiscall" fn compare_construct_ita_c1_super(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%)(this, year, record);
    let error = ita_super_ctor_plan_error(this, r, year, record, ITA_C1_SUPER_VTABLE, &ITA_C1_SUPER_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_ita_c1_super({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of the US Open Cup constructor (see `construct_usa_open_cup`).
unsafe fn construct_usa_open_cup_body(this: usize, year: u32, record: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    write_usize(this + 4, record);
    write_usize(this, USA_OPEN_CUP_VTABLE);
    cup.set_year(year as i16);
    cup.set_current_round(0);
    cup.set_stage_index(0xff);
    cup.set_stage_count(0);
    cup.set_current_stage(0xffff_ffff);
    cup.set_club_sides(1);
    cup.set_discipline_index(0x1e);
    cup.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("cqv_5 us open cup new");
    let collect_entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x90b530usize);
    if collect_entrants(this) < 0 {
        show_error_at(0xaa20e0, 0x43);
        return this;
    }
    cup.set_prize_fund(0xf4240);
    ita_super_ctor_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    if map_fixture_tree(this) < 0 {
        show_error_at(0xaa20e0, 0x51);
        return this;
    }
    if !ita_super_ctor_create_stats(this) {
        show_error_at(0xaa20e0, 0x58);
        return this;
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    rank_teams(this);
    this
}

/// The fixed fields the US Open Cup constructor writes on a new cup (offset, width, value).
const USA_OPEN_CUP_CTOR_FIELDS: [(usize, u8, u32); 8] = [(cup::PRIZE_FUND, 4, 0xf4240), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::DISCIPLINE_INDEX, 1, 0x1e), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The US Open Cup constructor `UsaOpenCup::UsaOpenCup(year, record)`, run once at career creation. On the Cup::Cup
/// base (`exe::new_cup`): the competition record at +4, the vtable (0x970998), the year (`cup::YEAR`),
/// `current_round()` = 0, `stage_index()` = 0xff, `stage_count()` = 0, `current_stage()` = −1, `club_sides()` = 1
/// (clubs of one nation), discipline rule 30 and `run_by_stages()` = 0; the bench and substitutions stay as the base
/// left them. If the saved state loads (Cup::load_from_disk 0x51fc00, thiscall(this, 1), ret 4; returns ≠ 0), it
/// returns this. On a new cup, in order: the entrants (UsaOpenCup::collect_entrants 0x90b530, thiscall, ret 0; the
/// error box of line 0x43 if it returns a negative value); the prize fund (`cup::PRIZE_FUND`) = 1,000,000; the rounds
/// (`comp_vtable::CREATE_SCHEDULE`, into `round_table()`); the tie tree (Cup::map_fixture_tree 0x518790, thiscall, ret
/// 0; the error box of line 0x51 if it returns a negative value); the statistics at +8 (the error box of line 0x58
/// without them); the final ranking (Cup::rank_teams 0x5223a0, thiscall, ret 0, unchecked). Every failure returns this.
/// Original: `UsaOpenCup::UsaOpenCup` (0x90aeb0, usa_open_cup.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn construct_usa_open_cup(this: usize, year: u32, record: usize) -> usize {
    construct_usa_open_cup_body(this, year, record) // cqv5: the US Open Cup
}

/// Runs only the original (it allocates, draws and reads the save file), checking the plan (`ita_super_ctor_plan_error`).
unsafe extern "thiscall" fn compare_construct_usa_open_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%)(this, year, record);
    let error = ita_super_ctor_plan_error(this, r, year, record, USA_OPEN_CUP_VTABLE, &USA_OPEN_CUP_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_usa_open_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}
