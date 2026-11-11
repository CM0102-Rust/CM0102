// ---------- %%LEVA%%a leva: the European cup constructors: the UEFA Cup, the European Super Cup and the Intertoto Cup (eur_super_cup.cpp, intertoto_cup.cpp, uefa_cup.cpp) ----------

/// The cup's rounds through its `comp_vtable::CREATE_SCHEDULE` method (push 0; push &`cup::STAGE_LABEL`; push
/// &`cup::ROUND_COUNT`; push −1; mov ecx, this -> thiscall, ret 0x10): the round table, stored in `round_table()`.
unsafe fn uefa_cup_ctor_create_rounds(this: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let t = f(this, 0xffff_ffff, this + cup::ROUND_COUNT, this + cup::STAGE_LABEL, 0);
    Cup(this).set_round_table(t);
}

/// The statistics (`exe::crt_operator_new`(0x5ce), cdecl; `exe::new_comp_stats`(new, this), thiscall, ret 4) at +8; the
/// original's SEH frame covers only that new. Returns whether they were created.
unsafe fn uefa_cup_ctor_create_stats(this: usize) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    write_usize(this + 8, s);
    s != 0
}

/// The plan the comparer checks after the original: the return value (this), the vtable, the competition record (+4)
/// and the year (`cup::YEAR`); on a new cup (`stage_index()` = 0xff), each fixed field the constructor writes
/// (offset, width in bytes, value) and the round table and statistics it created. Returns the first difference.
unsafe fn uefa_cup_ctor_plan_error(this: usize, r: usize, year: u32, record: usize, vtable: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
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

const UEFA_CUP_VTABLE: usize = 0x970790;
const EUR_SUPER_CUP_VTABLE: usize = 0x969f80;
const INTERTOTO_CUP_VTABLE: usize = 0x96bb8c;

/// The body of the UEFA Cup constructor (see `construct_uefa_cup`).
unsafe fn construct_uefa_cup_body(this: usize, year: u32, record: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    cup.set_year(year as i16);
    write_usize(this + 4, record);
    write_usize(this, UEFA_CUP_VTABLE);
    cup.set_current_round(0);
    cup.set_stage_index(0xff);
    cup.set_current_stage(0xffff_ffff);
    cup.set_stage_count(0);
    cup.set_club_sides(2);
    write_u8(this + 0x52, 3);
    cup.set_substitutes_named(7);
    cup.set_substitutions_allowed(3);
    cup.set_discipline_index(1);
    cup.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("cqu_9 uefa cup new");
    // the yearly European seeding (UefaSeeding::update_season 0x9035a0: push 0; mov ecx, [G_UEFA_SEEDING] -> thiscall, ret 4)
    let seeding = read_usize(G_UEFA_SEEDING);
    if seeding != 0 {
        let update_season: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x9035a0usize);
        update_season(seeding, 0);
    }
    let build_entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x902b90usize);
    if build_entrants(this) == 0 {
        show_error_at(0xaa19e0, 0x54);
        return this;
    }
    cup.set_prize_fund(0x2dc6c0);
    uefa_cup_ctor_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    if map_fixture_tree(this) == 0 {
        show_error_at(0xaa19e0, 0x62);
        return this;
    }
    if !uefa_cup_ctor_create_stats(this) {
        show_error_at(0xaa19e0, 0x69);
        return this;
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    rank_teams(this);
    this
}

/// The fixed fields the UEFA Cup constructor writes on a new cup (offset, width, value).
const UEFA_CUP_CTOR_FIELDS: [(usize, u8, u32); 11] = [(cup::PRIZE_FUND, 4, 0x2dc6c0), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x2), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x1), (0x52, 1, 0x3), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The UEFA Cup constructor `UefaCup::UefaCup(year, record)`, run once at career creation. On the Cup::Cup base
/// (`exe::new_cup`): the year (`cup::YEAR`), the competition record at +4, the vtable (0x970790), `current_round()` =
/// 0, `stage_index()` = 0xff, `current_stage()` = −1, `stage_count()` = 0, `club_sides()` = 2 (clubs of several
/// nations), +0x52 = 3, 7 substitutes named, 3 substitutions allowed, discipline rule 1 and `run_by_stages()` = 0. If
/// the saved state loads (Cup::load_from_disk 0x51fc00, thiscall(this, 1), ret 4; returns ≠ 0), it returns this. On a
/// new cup, in order: the yearly European seeding (UefaSeeding::update_season 0x9035a0 on [`G_UEFA_SEEDING`], if any;
/// thiscall, ret 4); the entrants (UefaCup::build_entrants 0x902b90, thiscall, ret 0; the error box of line 0x54 if it
/// returns 0); the prize fund (`cup::PRIZE_FUND`) = 3,000,000; the rounds (`comp_vtable::CREATE_SCHEDULE`, into
/// `round_table()`); the tie tree (Cup::map_fixture_tree 0x518790, thiscall, ret 0; the error box of line 0x62 if it
/// returns 0); the statistics at +8 (the error box of line 0x69 without them); the final ranking (Cup::rank_teams
/// 0x5223a0, thiscall, ret 0, unchecked). Every failure returns this. [the bench, substitutions, discipline and type
/// fields read from the competitions reference, §2.1]
/// Original: `UefaCup::UefaCup` (0x9022e0, uefa_cup.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn construct_uefa_cup(this: usize, year: u32, record: usize) -> usize {
    construct_uefa_cup_body(this, year, record) // cqu9: the UEFA Cup
}

/// Runs only the original (it allocates, draws and reads the save file), checking the plan (`uefa_cup_ctor_plan_error`).
unsafe extern "thiscall" fn compare_construct_uefa_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, year, record);
    let error = uefa_cup_ctor_plan_error(this, r, year, record, UEFA_CUP_VTABLE, &UEFA_CUP_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_uefa_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of the European Super Cup constructor (see `construct_eur_super_cup`).
unsafe fn construct_eur_super_cup_body(this: usize, year: u32, record: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    cup.set_year(year as i16);
    write_usize(this + 4, record);
    write_usize(this, EUR_SUPER_CUP_VTABLE);
    cup.set_current_round(0);
    cup.set_stage_index(0xff);
    cup.set_stage_count(0);
    cup.set_current_stage(0xffff_ffff);
    cup.set_club_sides(2);
    write_u8(this + 0x52, 3);
    cup.set_substitutes_named(7);
    cup.set_substitutions_allowed(3);
    cup.set_discipline_index(1);
    cup.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("cqu_9 european super cup new");
    let select_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x57a500usize);
    if select_teams(this) < 0 {
        show_error_at(0x9c45f8, 0x3f);
        return this;
    }
    uefa_cup_ctor_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    if map_fixture_tree(this) < 0 {
        show_error_at(0x9c45f8, 0x4d);
        return this;
    }
    if !uefa_cup_ctor_create_stats(this) {
        show_error_at(0x9c45f8, 0x54);
        return this;
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    rank_teams(this);
    this
}

/// The fixed fields the European Super Cup constructor writes on a new cup (offset, width, value).
const EUR_SUPER_CUP_CTOR_FIELDS: [(usize, u8, u32); 10] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x2), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x1), (0x52, 1, 0x3), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The European Super Cup constructor `EurSuperCup::EurSuperCup(year, record)`, run once at career creation. On the
/// Cup::Cup base (`exe::new_cup`): the year (`cup::YEAR`), the competition record at +4, the vtable (0x969f80),
/// `current_round()` = 0, `stage_index()` = 0xff, `stage_count()` = 0, `current_stage()` = −1, `club_sides()` = 2
/// (clubs of several nations), +0x52 = 3, 7 substitutes named, 3 substitutions allowed, discipline rule 1 and
/// `run_by_stages()` = 0; no prize fund (`cup::PRIZE_FUND` stays as the base left it). If the saved state loads
/// (Cup::load_from_disk 0x51fc00, thiscall(this, 1), ret 4; returns ≠ 0), it returns this. On a new cup, in order: the
/// two clubs (EurSuperCup::select_teams 0x57a500, thiscall, ret 0; the error box of line 0x3f if it returns a negative
/// value); the rounds (`comp_vtable::CREATE_SCHEDULE`, into `round_table()`); the tie tree (Cup::map_fixture_tree
/// 0x518790, thiscall, ret 0; the error box of line 0x4d if it returns a negative value); the statistics at +8 (the
/// error box of line 0x54 without them); the final ranking (Cup::rank_teams 0x5223a0, thiscall, ret 0, unchecked).
/// Every failure returns this.
/// Original: `EurSuperCup::EurSuperCup` (0x57a100, eur_super_cup.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn construct_eur_super_cup(this: usize, year: u32, record: usize) -> usize {
    construct_eur_super_cup_body(this, year, record) // cqu9: the European Super Cup
}

/// Runs only the original (it allocates, draws and reads the save file), checking the plan (`uefa_cup_ctor_plan_error`).
unsafe extern "thiscall" fn compare_construct_eur_super_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%)(this, year, record);
    let error = uefa_cup_ctor_plan_error(this, r, year, record, EUR_SUPER_CUP_VTABLE, &EUR_SUPER_CUP_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_eur_super_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of the Intertoto Cup constructor (see `construct_intertoto_cup`).
unsafe fn construct_intertoto_cup_body(this: usize, year: u32, record: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    cup.set_year(year as i16);
    write_usize(this + 4, record);
    write_usize(this, INTERTOTO_CUP_VTABLE);
    cup.set_current_round(0);
    cup.set_stage_index(0xff);
    cup.set_current_stage(0xffff_ffff);
    cup.set_stage_count(0);
    cup.set_club_sides(2);
    write_u8(this + 0x52, 3);
    cup.set_substitutes_named(7);
    cup.set_substitutions_allowed(3);
    write_u32(this + intertoto_cup::WINNERS_SENT_TO_UEFA, 0);
    cup.set_discipline_index(1);
    cup.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("cqu_9 intertoto cup new");
    // the yearly European seeding (UefaSeeding::update_season 0x9035a0: push 0; mov ecx, [G_UEFA_SEEDING] -> thiscall, ret 4)
    let seeding = read_usize(G_UEFA_SEEDING);
    if seeding != 0 {
        let update_season: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x9035a0usize);
        update_season(seeding, 0);
    }
    uefa_cup_ctor_create_rounds(this);
    let collect_entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x6336c0usize);
    if collect_entrants(this) == 0 {
        show_error_at(0x9e9e14, 0x60);
        return this;
    }
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    if map_fixture_tree(this) == 0 {
        show_error_at(0x9e9e14, 0x67);
        return this;
    }
    if !uefa_cup_ctor_create_stats(this) {
        show_error_at(0x9e9e14, 0x6e);
        return this;
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    rank_teams(this);
    this
}

/// The fixed fields the Intertoto Cup constructor writes on a new cup (offset, width, value).
const INTERTOTO_CUP_CTOR_FIELDS: [(usize, u8, u32); 11] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x2), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x1), (0x52, 1, 0x3), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0), (intertoto_cup::WINNERS_SENT_TO_UEFA, 4, 0x0)];

/// The Intertoto Cup constructor `IntertotoCup::IntertotoCup(year, record)`, run once at career creation. On the
/// Cup::Cup base (`exe::new_cup`): the year (`cup::YEAR`), the competition record at +4, the vtable (0x96bb8c),
/// `current_round()` = 0, `stage_index()` = 0xff, `current_stage()` = −1, `stage_count()` = 0, `club_sides()` = 2
/// (clubs of several nations), +0x52 = 3, 7 substitutes named, 3 substitutions allowed,
/// `intertoto_cup::WINNERS_SENT_TO_UEFA` = 0, discipline rule 1 and `run_by_stages()` = 0; no prize fund
/// (`cup::PRIZE_FUND` stays as the base left it). If the saved state loads (Cup::load_from_disk 0x51fc00,
/// thiscall(this, 1), ret 4; returns ≠ 0), it returns this. On a new cup, in order: the yearly European seeding
/// (UefaSeeding::update_season 0x9035a0 on [`G_UEFA_SEEDING`], if any; thiscall, ret 4); the rounds
/// (`comp_vtable::CREATE_SCHEDULE`, into `round_table()`); the 60 entrants (IntertotoCup::collect_entrants 0x6336c0,
/// thiscall, ret 0; the error box of line 0x60 if it returns 0); the tie tree (Cup::map_fixture_tree 0x518790,
/// thiscall, ret 0; the error box of line 0x67 if it returns 0); the statistics at +8 (the error box of line 0x6e
/// without them); the final ranking (Cup::rank_teams 0x5223a0, thiscall, ret 0, unchecked). Every failure returns this.
/// Original: `IntertotoCup::IntertotoCup` (0x632fb0, intertoto_cup.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn construct_intertoto_cup(this: usize, year: u32, record: usize) -> usize {
    construct_intertoto_cup_body(this, year, record) // cqu9: the Intertoto Cup
}

/// Runs only the original (it allocates, draws and reads the save file), checking the plan (`uefa_cup_ctor_plan_error`).
unsafe extern "thiscall" fn compare_construct_intertoto_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%)(this, year, record);
    let error = uefa_cup_ctor_plan_error(this, r, year, record, INTERTOTO_CUP_VTABLE, &INTERTOTO_CUP_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_intertoto_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}
