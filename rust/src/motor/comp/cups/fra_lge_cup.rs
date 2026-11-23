use super::*;

/// rule: the French League Cup has 7 single-leg rounds: preliminary (code 0xe6, 16 teams) drawn 1/9, played 5/10; 1st
/// round 15/10 and 16/11 (28 teams, 20 new); 2nd 26/11 and 8/1 (32 teams, 18 new); 3rd 14/1 and 29/1; quarter-finals
/// 3/2 and 19/2; semi-final 4/3 and 1/4; final 2/4 and 22/4 at a neutral ground [inference: venue 4]; 300 thousand for
/// winning each round and 1.5 million to the winner; category 4 up to the quarter-finals and 6 after
/// Original: `FraLgeCup::create_round_table` (0x5bcc40, fra_lge_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn fra_lge_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    single_stage_round_table(Cup(this), stage, rounds, extra, out, &FRENCH_LEAGUE_CUP_ROUNDS, 0x9ccb98, 0xa2, "cql_11 tabela da copa da liga francesa")
}

/// rule: the French League Cup takes 54 clubs, in this order (the first 16 in the preliminary round, the next 20 in the
/// 1st round and the last 18 in the 2nd): those of division `KEY_CLUB_COMP_FRENCH_NATIONAL` that did not come from
/// `KEY_CLUB_COMP_FRENCH_CFA` (up to 16 in 2001 and 14 in other years), topped up to 16 with those of
/// `KEY_CLUB_COMP_FRENCH_NATIONAL` that came from `KEY_CLUB_COMP_FRENCH_CFA` (the promoted ones [inference]); the
/// Second and the First Division; with an incomplete list it returns 0
/// Original: `FraLgeCup::collect_entrants` (0x5bd160, fra_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fra_lge_cup_collect_entrants(this: usize) -> i32 {
    french_league_cup_collect_entrants(Cup(this)).0
}

/// The season turnover of the French League Cup: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; finally the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax it returns.
///
/// The cup turnover template (`start_new_cup_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (fra_lge_cup.cpp, lines 0x1f9, 0x200 and 0x207) when it returns a negative value: the entrants (FraLgeCup::collect_entrants at 0x5bd160, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `FraLgeCup::start_new_season` (0x5bd420, fra_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fra_lge_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x5bd160, 0x9ccb98, [0x1f9, 0x200, 0x207], 1, "mwf_1 French League Cup rollover") // mwf1: the turnover of the French League Cup
}

/// Comparer for `fra_lge_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_fra_lge_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FRA_LGE_CUP_START_NEW_SEASON);
    compare_cup_new_season("fra_lge_cup_start_new_season", Cup(this), || o(this))
}

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
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_FRA_LGE_CUP)(this, year, record);
    let error = fra_cup_check_constructor_plan(Cup(this), r, year, record, 0x96aad8, &FRENCH_LEAGUE_CUP_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_fra_lge_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
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
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_CUP_LOAD_FROM_DISK); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_7 french league cup new");
    let collect_entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_FRALGECUP_COLLECT_ENTRANTS); // FraLgeCup::collect_entrants
    let r = collect_entrants(this.addr());
    if r == 0 {
        show_error_at(0x9ccb98, 0x49);
        return this.addr();
    }
    this.set_prize_fund(0x7a120);
    fra_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_CUP_MAP_FIXTURE_TREE); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r == 0 {
        show_error_at(0x9ccb98, 0x57);
        return this.addr();
    }
    if !fra_cup_create_comp_stats(this) {
        show_error_at(0x9ccb98, 0x5e);
        return this.addr();
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_CUP_RANK_TEAMS); // Cup::rank_teams
    rank_teams(this.addr());
    this.addr()
}

/// The fixed fields `construct_fra_lge_cup` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const FRENCH_LEAGUE_CUP_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 10] = [(cup::PRIZE_FUND, 4, 0x7a120), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x3), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0xc), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];
