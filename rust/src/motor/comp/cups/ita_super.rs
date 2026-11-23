use super::*;

/// rule: the Italian Super Cup is a single match (2 teams) on 19 August, drawn on 5 July, category 6, tie-break 0x203,
/// no prizes
/// Original: `ItaSuper::create_round_table` (0x65f920, ita_super.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ita_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &ITALIAN_SUPER_CUP, 0x9eac34, 0x99, "cqo_10 tabela da supercopa")
}

/// rule: the Italian Super Cup is between the Serie A champion (the home side) and the Coppa Italia winner; if the same
/// club won both, the Cup runner-up plays
/// Original: `ItaSuper::init_team_list` (0x65fa60, ita_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_super_init_team_list(this: usize) -> i32 {
    init_supercoppa_teams(this).0
}

/// The season turnover of the Italian Super Cup: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; no predicted positions: it returns the eax of the tie tree.
///
/// The cup turnover template (`usa_mls_all_stars_cup_start_new_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (ita_super.cpp, lines 0x125, 0x12c and 0x133) when it returns a negative value: the entrants (ItaSuper::init_team_list at 0x65fa60, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `ItaSuper::start_new_season` (0x65fc90, ita_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x65fa60, 0x9ea934, [0x125, 0x12c, 0x133], 1, false, "mwf_3 Italian Super Cup rollover") // mwf3: the turnover of the Italian Super Cup
}

/// Comparer for `ita_super_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_ita_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_SUPER_START_NEW_SEASON);
    compare_cup_new_season("ita_super_start_new_season", Cup(this), || o(this))
}

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
pub(crate) unsafe extern "thiscall" fn construct_ita_super(this: usize, year: u32, record: usize) -> usize {
    construct_ita_super_body(this, year, record) // cqv5: the Italian Super Cup
}

/// Runs only the original (it allocates, draws and reads the save file), checking the plan (`ita_super_ctor_plan_error`).
pub(crate) unsafe extern "thiscall" fn compare_construct_ita_super(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ITA_SUPER)(this, year, record);
    let error = ita_super_ctor_plan_error(this, r, year, record, ITA_SUPER_VTABLE, &ITA_SUPER_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_ita_super({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of the Italian Super Cup constructor (see `construct_ita_super`).
pub(crate) unsafe fn construct_ita_super_body(this: usize, year: u32, record: usize) -> usize {
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
    if exe::cup_load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("cqv_5 italian super cup new");
    if exe::itasuper_init_team_list(this) < 0 {
        show_error_at(0x9ea934, 0x43);
        return this;
    }
    ita_super_ctor_create_rounds(this);
    if exe::cup_map_fixture_tree(this) < 0 {
        show_error_at(0x9ea934, 0x51);
        return this;
    }
    if !ita_super_ctor_create_stats(this) {
        show_error_at(0x9ea934, 0x58);
        return this;
    }
    this
}

pub(crate) const ITA_SUPER_VTABLE: usize = 0x96c8b8;

/// The fixed fields the Italian Super Cup constructor writes on a new cup (offset, width, value).
pub(crate) const ITA_SUPER_CTOR_FIELDS: [(usize, u8, u32); 9] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x11), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];
