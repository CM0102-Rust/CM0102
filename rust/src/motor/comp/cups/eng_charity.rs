use super::*;

/// rule: the Charity Shield is a single match between 2 clubs, drawn on 5/7 and played on 13/8 (moved to weekday 6) at
/// a neutral ground [inference: venue 4], decided on penalties [inference: tiebreak 1]; category 6; no prize
/// Original: `EngCharity::create_round_table` (0x56d640, eng_charity.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn eng_charity_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &CHARITY_SHIELD_ROUNDS, 0x9c425c, 0x9c, "cql_10 tabela da supercopa inglesa")
}

/// rule: the Charity Shield is between the Premier champion and the FA Cup winner; if the same club won both, the
/// Premier runner-up takes its place; if one is missing, the error box
/// Original: `EngCharity::select_teams` (0x56d780, eng_charity.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_charity_select_teams(this: usize) -> i32 {
    select_charity_shield_teams(Cup(this)).0
}

/// The season turnover of the Charity Shield: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; finally the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax it returns.
///
/// The cup turnover template (`start_new_cup_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (eng_charity.cpp, lines 0x128, 0x12f and 0x136) when it returns a negative value: the entrants (EngCharity::select_teams at 0x56d780, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `EngCharity::start_new_season` (0x56d9b0, eng_charity.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_charity_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x56d780, 0x9c425c, [0x128, 0x12f, 0x136], 1, "mwf_1 Charity Shield rollover") // mwf1: the turnover of the Charity Shield
}

/// Comparer for `eng_charity_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_eng_charity_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_CHARITY_START_NEW_SEASON);
    compare_cup_new_season("eng_charity_start_new_season", Cup(this), || o(this))
}

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
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ENG_CHARITY)(this, year, record);
    let error = eng_fa_cup_check_constructor_plan(Cup(this), r, year, record, 0x9699d4, &CHARITY_SHIELD_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_eng_charity({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
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
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_CUP_LOAD_FROM_DISK); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_5 charity shield new");
    let select_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_ENGCHARITY_SELECT_TEAMS); // EngCharity::select_teams
    let r = select_teams(this.addr());
    if r < 0 {
        show_error_at(0x9c425c, 0x43);
        return this.addr();
    }
    this.set_prize_fund(0x493e0);
    eng_fa_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_CUP_MAP_FIXTURE_TREE); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r < 0 {
        show_error_at(0x9c425c, 0x51);
        return this.addr();
    }
    if !eng_fa_cup_create_comp_stats(this) {
        show_error_at(0x9c425c, 0x58);
        return this.addr();
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_CUP_RANK_TEAMS); // Cup::rank_teams
    rank_teams(this.addr());
    this.addr()
}

/// The fixed fields `construct_eng_charity` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const CHARITY_SHIELD_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 10] = [(cup::PRIZE_FUND, 4, 0x493e0), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x5), (cup::DISCIPLINE_INDEX, 1, 0x9), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];
