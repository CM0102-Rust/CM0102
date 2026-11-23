use super::*;

/// rule: the Japanese Super Cup (Xerox Super Cup) is a single match between 2 clubs, drawn on 1/2 and played on 4/3
/// (moved to weekday 5) at a neutral ground [inference: venue 4], decided on penalties [inference: tiebreak 1]; 150
/// thousand to the winner; category 6
/// Original: `JapSuper::create_round_table` (0x666020, jap_super.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn jap_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    fra_super_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &JAPAN_SUPER_CUP_ROUNDS, 0x9eaf4c, 0x98, "cql_12 tabela da supercopa do japao", |_| {})
}

/// rule: the Japanese Super Cup is between the J1 champion and the Emperor's Cup winner; a winner with no nation
/// (`club::NATION` = 0, a club that no longer exists [inference]) gives the place to the runner-up; if it is the same
/// club, the Emperor's Cup runner-up comes in and, if they still coincide, the J1 runner-up; if one is missing, the
/// error box
/// Original: `JapSuper::select_teams` (0x666160, jap_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn jap_super_select_teams(this: usize) -> i32 {
    select_japanese_super_cup_teams(this).0
}

/// rule: at the season turn the Japanese Super Cup is rebuilt: last edition's tables freed, history archived, year +1,
/// the two clubs (JapSuper::select_teams: the J1 champion and the Emperor's Cup winner [inference]), the rounds, the
/// tree and the predicted positions
/// Original: `JapSuper::start_new_season` (0x6664e0, jap_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn jap_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x666160, 0x9eaf4c, [0x14b, 0x152, 0x159], 1, true, "cqr_6 virada da supercopa japonesa") // cqr6: a Supercopa japonesa
}

/// The constructor of the Japanese Super Cup: with no entrant table of its own; type 1 (clubs of one nation), no prize
/// fund (`cup::PRIZE_FUND` stays as the base left it), bench and substitutions from the base, discipline rule 20; on a
/// new cup, in order: the two clubs (JapSuper::select_teams, `jap_super_select_teams`, 0x666160, thiscall, ret 0; the
/// error box of line 0x3f if it returns a negative value); the rounds (the cup's own virtual
/// `comp_vtable::CREATE_SCHEDULE`, into `cup::ROUND_TABLE`); the tie tree (Cup::map_fixture_tree, `map_fixture_tree`,
/// 0x518790, thiscall, ret 0; the error box of line 0x4d if it returns a negative value); the statistics at Comp +8
/// (the error box of line 0x54 without them); the final ranking (Cup::rank_teams, `cup_rank_teams`, 0x5223a0, thiscall,
/// ret 0, unchecked). Over the base Cup::Cup (`exe::new_cup`, thiscall, ret 0) it writes the fixed fields in the
/// original's order (the vtable 0x96cc48, the competition record at Comp +4, `cup::YEAR` and those of
/// `JAPANESE_SUPER_CUP_CONSTRUCTOR_FIELDS`); if the saved state loads (Cup::load_from_disk 0x51fc00: push 1; mov ecx,
/// this -> thiscall, ret 4; returns ≠ 0) it returns this. Each step that fails shows its own error box
/// (`show_error_at`) and returns this. Returns this.
/// Original: `JapSuper::JapSuper` (0x665d70, jap_super.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_jap_super(this: usize, year: u32, record: usize) -> usize {
    construct_jap_super_body(Cup(this), year, record) // cqu8: the Japanese Super Cup
}

/// Comparer for `construct_jap_super`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`jap_emp_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_jap_super(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_JAP_SUPER)(this, year, record);
    let error = jap_emp_cup_check_constructor_plan(Cup(this), r, year, record, 0x96cc48, &JAPANESE_SUPER_CUP_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_jap_super({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of `construct_jap_super` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_jap_super_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x96cc48); // the vtable
    write_u16(this.addr() + cup::YEAR, year as u16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    this.set_stage_count(0);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_club_sides(1);
    this.set_discipline_index(discipline_rule::JAPAN_CUP);
    this.set_run_by_stages(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_CUP_LOAD_FROM_DISK); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_8 japanese super cup new");
    let select_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_JAPSUPER_SELECT_TEAMS); // JapSuper::select_teams
    let r = select_teams(this.addr());
    if r < 0 {
        show_error_at(0x9eaf4c, 0x3f);
        return this.addr();
    }
    jap_emp_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_CUP_MAP_FIXTURE_TREE); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r < 0 {
        show_error_at(0x9eaf4c, 0x4d);
        return this.addr();
    }
    if !jap_emp_cup_create_comp_stats(this) {
        show_error_at(0x9eaf4c, 0x54);
        return this.addr();
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_CUP_RANK_TEAMS); // Cup::rank_teams
    rank_teams(this.addr());
    this.addr()
}

/// The fixed fields `construct_jap_super` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const JAPANESE_SUPER_CUP_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 7] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::DISCIPLINE_INDEX, 1, 0x14), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];
