use super::*;

/// rule: the Serie C1 Super Cup is a single match on 21 May of the following year, drawn on 15 May, category 6, no
/// prizes
/// Original: `ItaC1Super::create_round_table` (0x63c890, ita_c1_super.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ita_c1_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &SERIE_C1_SUPER_CUP_ROUNDS, 0x9ea1c8, 0x99, "cqo_10 tabela da supercopa c1")
}

/// rule: the Serie C1 Super Cup starts with both places empty; the clubs enter at the end of the season [inference: the
/// champions of the two C1 groups]
/// Original: `ItaC1Super::init_team_list` (0x63c9d0, ita_c1_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_c1_super_init_team_list(this: usize) -> i32 {
    init_c1_super_cup_list(this).0
}

/// The season turnover of the Serie C1 Super Cup: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; no predicted positions: it returns the eax of the tie tree.
///
/// The cup turnover template (`usa_mls_all_stars_cup_start_new_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (ita_c1_super.cpp, lines 0x10a, 0x111 and 0x118) when it returns a negative value: the entrants (ItaC1Super::init_team_list at 0x63c9d0, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `ItaC1Super::start_new_season` (0x63caa0, ita_c1_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_c1_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x63c9d0, 0x9ea1c8, [0x10a, 0x111, 0x118], 1, false, "mwf_3 Serie C1 Super Cup rollover") // mwf3: the turnover of the Serie C1 Super Cup
}

/// Comparer for `ita_c1_super_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_ita_c1_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_C1_SUPER_START_NEW_SEASON);
    compare_cup_new_season("ita_c1_super_start_new_season", Cup(this), || o(this))
}

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
pub(crate) unsafe extern "thiscall" fn construct_ita_c1_super(this: usize, year: u32, record: usize) -> usize {
    construct_ita_c1_super_body(this, year, record) // cqv5: the Serie C1 Super Cup
}

/// Runs only the original (it allocates, draws and reads the save file), checking the plan (`ita_super_ctor_plan_error`).
pub(crate) unsafe extern "thiscall" fn compare_construct_ita_c1_super(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ITA_C1_SUPER)(this, year, record);
    let error = ita_super_ctor_plan_error(this, r, year, record, ITA_C1_SUPER_VTABLE, &ITA_C1_SUPER_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_ita_c1_super({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The plan the comparer checks after the original: the return value (this), the vtable, the competition record (+4)
/// and the year (`cup::YEAR`); on a new cup (`stage_index()` = 0xff), each fixed field the constructor writes
/// (offset, width in bytes, value) and the round table and statistics it created. Returns the first difference.
pub(crate) unsafe fn ita_super_ctor_plan_error(this: usize, r: usize, year: u32, record: usize, vtable: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    if r != this || read_usize(this) != vtable || read_usize(this + 4) != record || read_u16(this + cup::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this), read_usize(this + 4), read_u16(this + cup::YEAR)));
    }
    if read_u8(this + cup::STAGE_INDEX) != 0xff || league_will_load_saved_state(record) {
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

/// The body of the Serie C1 Super Cup constructor (see `construct_ita_c1_super`).
pub(crate) unsafe fn construct_ita_c1_super_body(this: usize, year: u32, record: usize) -> usize {
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
    if exe::cup_load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("cqv_5 serie c1 super cup new");
    if exe::itac1super_init_team_list(this) < 0 {
        show_error_at(0x9ea1c8, 0x43);
        return this;
    }
    ita_super_ctor_create_rounds(this);
    if exe::cup_map_fixture_tree(this) < 0 {
        show_error_at(0x9ea1c8, 0x51);
        return this;
    }
    if !ita_super_ctor_create_stats(this) {
        show_error_at(0x9ea1c8, 0x58);
        return this;
    }
    this
}

/// The cup's rounds through its `comp_vtable::CREATE_SCHEDULE` method (push 0; push &`cup::STAGE_LABEL`; push
/// &`cup::ROUND_COUNT`; push −1; mov ecx, this -> thiscall, ret 0x10): the round table, stored in `round_table()`.
pub(crate) unsafe fn ita_super_ctor_create_rounds(this: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let t = f(this, 0xffff_ffff, this + cup::ROUND_COUNT, this + cup::STAGE_LABEL, 0);
    Cup(this).set_round_table(t);
}

/// The statistics (`exe::crt_operator_new`(0x5ce), cdecl; `exe::new_comp_stats`(new, this), thiscall, ret 4) at +8; the
/// original's SEH frame covers only that new. Returns whether they were created.
pub(crate) unsafe fn ita_super_ctor_create_stats(this: usize) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    write_usize(this + 8, s);
    s != 0
}

pub(crate) const ITA_C1_SUPER_VTABLE: usize = 0x96c1c4;

/// The fixed fields the Serie C1 Super Cup constructor writes on a new cup (offset, width, value).
pub(crate) const ITA_C1_SUPER_CTOR_FIELDS: [(usize, u8, u32); 9] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x11), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];
