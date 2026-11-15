// ---------- batch 296: the Irish First Division: the season turnover, the next stage and the promotion play-off (ire_first.cpp) ----------

/// Rule: the Irish First Division turnover: the statistics archived, the table and the round table cleared, the year
/// moved on and the rules, the table, the fixtures and the order rebuilt; then the predicted positions, the board
/// expectations and the referee renewal.
///
/// BraNatSecond::start_new_season's template (checked instruction by instruction: only the table routine,
/// IreFirst::create_table 0x635910, the file and the error box lines 0x217, 0x21e, 0x224 and 0x22b change).
/// Original: `IreFirst::start_new_season` (0x635b30, ire_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x635910, 0x9e9ed0, [0x217, 0x21e, 0x224, 0x22b], 1) { // mwa14: the next season of the Irish First
        return 0;
    }
    covered!("mwa_14 irish first turnover");
    set_expectations_and_renew_officials(this)
}

unsafe extern "thiscall" fn compare_ire_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("ire_first_start_new_season", League(this), || o(this))
}

/// Rule: the Irish First Division's league stage ends only when the Premier Division above has finished: then the
/// Premier's positions are updated and the promotion play-off is built; past the last stage, the error box.
///
/// `advance_irish_first_stage` with the Irish Premier Division (`KEY_CLUB_COMP_IRISH_PREMIER_DIVISION`), the error box at
/// line 0x248 of ire_first.cpp and IreFirst::create_promotion_playoff_stage (0x635dc0) (checked instruction by
/// instruction against NirFirst::advance_stage: only the key, the file and the line change).
/// Original: `IreFirst::advance_stage` (0x635cf0, ire_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_first_advance_stage(this: usize) {
    advance_irish_first_stage(League(this), KEY_CLUB_COMP_IRISH_PREMIER_DIVISION, 0x9e9ed0, 0x248, 0x635dc0) // mwa14: the Irish play-off waits for the Premier
}

unsafe extern "thiscall" fn compare_ire_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_IRE_FIRST_ADVANCE_STAGE);
    compare_irish_first_advance_stage("ire_first_advance_stage", League(this), || o(this))
}

/// Rule: the Irish promotion play-off: the First Division runner-up against the Premier Division's second last (the
/// clubs with the play-off status in each table), in a two-club stage; without one of them, no play-off.
///
/// `create_irish_promotion_play_off` with the Irish Premier Division and the error box lines 0x274, 0x284 and 0x2a5 of
/// ire_first.cpp (checked instruction by instruction against NirFirst::create_playoff_stage).
/// Original: `IreFirst::create_promotion_playoff_stage` (0x635dc0, ire_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_first_create_promotion_playoff_stage(this: usize) {
    create_irish_promotion_play_off(League(this), KEY_CLUB_COMP_IRISH_PREMIER_DIVISION, 0x9e9ed0, [0x274, 0x284, 0x2a5], "mwa_14 irish promotion play-off")
}

unsafe extern "thiscall" fn compare_ire_first_create_promotion_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_IRE_FIRST_CREATE_PROMOTION_PLAYOFF_STAGE);
    compare_irish_promotion_play_off("ire_first_create_promotion_playoff_stage", League(this), KEY_CLUB_COMP_IRISH_PREMIER_DIVISION, || o(this))
}

// ---------- batch 296: the season turnover of the Northern Irish and the Irish Premier Divisions (nir_prm.cpp, ire_prm.cpp) ----------

/// The turnover of an Irish Premier Division, which leads its nation's league turnover.
///
/// `league::SEASON_CLOSED` = 0; the promotion and relegation chain (the virtual
/// `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(`chain`), thiscall, ret 4: 1 in Northern Ireland, 0 in Ireland);
/// the table freed (League::free_table `exe::league_free_table`(this, 0)); the round table (`league::ROUND_DATES`) freed
/// (`exe::crt_free`) and zeroed; the season archived in the statistics (Comp_Stats::archive_season_and_reset
/// `exe::comp_stats_archive_season_and_reset`(stats, 1), when there); the year (`league::YEAR`) up by one. Then, each
/// with its own error box (returning 0) when it returns 0: the rules (`comp_vtable::SET_UP_RULES`), the table (`table`,
/// thiscall, ret 0), the fixtures (`exe::league_create_fixtures`) and the order (`exe::league_sort_table`(this, 0)).
/// Then the First Division's turnover (the competition object of the key global `first`, its
/// `comp_vtable::START_NEW_SEASON`), the predicted positions (`comp_vtable::SET_PREDICTED_POSITIONS`), the board
/// expectations and the referee renewal (`set_expectations_and_renew_officials`, whose return it passes on).
pub(crate) unsafe fn start_irish_premier_season(this: League, chain: u32, table: usize, first: usize, file: usize, rows: [u32; 4], year: i16) -> u32 {
    this.set_season_closed(0);
    let promote: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    promote(this.addr(), chain);
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    this.set_year(this.year().wrapping_add(year));
    // simplification: in the boxes the original returns the box's eax; the caller of the turnover does not read it
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, rows[0]);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(table);
    if table(this.addr()) == 0 {
        show_error_at(file, rows[1]);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, rows[2]);
        return 0;
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(file, rows[3]);
        return 0;
    }
    covered!("mwa_15 irish premier turnover");
    bra_nat_first_call_virtual_no_args(comp_by_global(first), comp_vtable::START_NEW_SEASON); // mwa15: the First Division turns over with the Premier
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// Rule: the Northern Irish turnover is led by the Premier Division: promotion and relegation, the statistics archived,
/// the next year, the rules, the ten-club table, the fixtures and the order rebuilt; then the First Division turns over,
/// and the predicted positions, the board expectations and the referee renewal follow.
///
/// `start_irish_premier_season` with the chain argument 1, NirPrm::create_table (0x78ec60), the Northern Irish First
/// Division and the error box lines 0x1c0, 0x1c7, 0x1cd and 0x1d4 of nir_prm.cpp.
/// Original: `NirPrm::start_new_season` (0x78ee30, nir_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_prm_start_new_season(this: usize) -> u32 {
    start_irish_premier_season(League(this), 1, 0x78ec60, KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION, 0xa696d0, [0x1c0, 0x1c7, 0x1cd, 0x1d4], 1) // mwa15: the Northern Irish turnover
}

unsafe extern "thiscall" fn compare_nir_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NIR_PRM_START_NEW_SEASON);
    compare_state_league_new_season("nir_prm_start_new_season", League(this), || o(this))
}

/// Rule: the Irish turnover is led by the Premier Division: promotion and relegation, the statistics archived, the next
/// year, the rules, the table, the fixtures and the order rebuilt; then the First Division turns over, and the predicted
/// positions, the board expectations and the referee renewal follow.
///
/// `start_irish_premier_season` with the chain argument 0, IrePrm::create_table (0x63a740), the Irish First Division and
/// the error box lines 0x1f3, 0x1fa, 0x200 and 0x207 of ire_prm.cpp (checked instruction by instruction against
/// NirPrm's).
/// Original: `IrePrm::start_new_season` (0x63a9b0, ire_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_prm_start_new_season(this: usize) -> u32 {
    start_irish_premier_season(League(this), 0, 0x63a740, KEY_CLUB_COMP_IRISH_FIRST_DIVISION, 0x9ea080, [0x1f3, 0x1fa, 0x200, 0x207], 1) // mwa15: the Irish turnover
}

unsafe extern "thiscall" fn compare_ire_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_PRM_START_NEW_SEASON);
    compare_state_league_new_season("ire_prm_start_new_season", League(this), || o(this))
}

// ---------- batch 296: the Irish League Cup: the constructor, the first group's table and the rules (ire_lge_cup.cpp) ----------

/// What changes between the Irish league cups' constructors (ItaCCup::ItaCCup's template, read instruction by
/// instruction): the vtable, the discipline rule, the stage count (the stage array is 4 bytes each), the group count,
/// the entrant, predicted positions, rules, first group table and group stage routines, and the source file (the error
/// box lines are the same: 0x4f, 0x59, 0x60, 0x66 and 0x6d).
pub(crate) struct IrishLeagueCupConstructor {
    pub(crate) vtable: usize,
    pub(crate) discipline: u8,
    pub(crate) stages: u32,
    pub(crate) groups: u32,
    pub(crate) entrants: usize,
    pub(crate) predicted: usize,
    pub(crate) rules: usize,
    pub(crate) first_group: usize,
    pub(crate) group_stage: usize,
    pub(crate) file: usize,
}

/// The body of the Irish league cups' constructors (thiscall, ret 8: this, year, the competition record).
///
/// Over League::League (`exe::new_league`, thiscall, ret 0): `league::COMPETITION` = the record, the vtable,
/// `league::YEAR`, 3 substitutions allowed, 5 substitutes named, the discipline rule and importance 0xc. If the saved
/// state loads (`exe::league_load_from_temp_file`(this, 1); ≠ 0) it returns this. Otherwise, in order:
/// `league::LEAGUE_FLAGS` = 0x28, `league::STAGE_INDEX` = `stage_index::MAIN`, `league::CURRENT_STAGE` = −1, the stage
/// count and its array (`exe::crt_malloc` of 4 × stages, unchecked); the entrants (thiscall, ret 0, unchecked); the
/// statistics (`ita_cups_create_comp_stats`; without them, the error box at line 0x4f); the predicted positions
/// (unchecked); the rules (failing on 0: line 0x59); the first group's table (failing on 0: line 0x60); the fixtures
/// (`exe::league_create_fixtures`; line 0x66); the order (`exe::league_sort_table`(this, 0); line 0x6d); the stage of
/// each group (push g; mov ecx, this -> thiscall, ret 4), g from 0. Each box returns this. Returns this.
pub(crate) unsafe fn construct_irish_league_cup(this: League, year: u32, record: usize, k: &IrishLeagueCupConstructor, branch: &'static str) -> usize {
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), k.vtable);
    this.set_year(year as u16 as i16);
    this.set_substitutions_allowed(3);
    this.set_substitutes_named(5);
    this.set_discipline_rule(k.discipline);
    this.set_importance(0xc);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!(branch);
    this.set_league_flags(0x28);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    this.set_stage_count(k.stages as usize);
    this.set_stages(exe::crt_malloc(k.stages * 4));
    let call = |f: usize| {
        let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(f);
        f(this.addr())
    };
    call(k.entrants);
    if !ita_cups_create_comp_stats(this) {
        show_error_at(k.file, 0x4f);
        return this.addr();
    }
    call(k.predicted);
    if call(k.rules) == 0 {
        show_error_at(k.file, 0x59);
        return this.addr();
    }
    if call(k.first_group) == 0 {
        show_error_at(k.file, 0x60);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(k.file, 0x66);
        return this.addr();
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(k.file, 0x6d);
        return this.addr();
    }
    let group: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(k.group_stage);
    for g in 0..k.groups {
        group(this.addr(), g);
    }
    this.addr()
}

/// The comparer of the Irish league cups' constructors: only the original runs, and the plan is checked
/// (`ita_cups_check_constructor_plan`: this, the vtable, the record, the year and, on a new cup, the fixed fields and
/// the statistics).
pub(crate) unsafe fn compare_irish_league_cup_constructor(name: &str, this: usize, year: u32, record: usize, k: &IrishLeagueCupConstructor, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let fields = [(league::STAGE_COUNT, 4u8, k.stages), (league::STAGE_INDEX, 1, 0xff), (league::SUBSTITUTES_NAMED, 1, 5), (league::SUBSTITUTIONS_ALLOWED, 1, 3),
        (league::DISCIPLINE_RULE, 1, k.discipline as u32), (league::IMPORTANCE, 1, 0xc), (league::LEAGUE_FLAGS, 2, 0x28)];
    let error = ita_cups_check_constructor_plan(this, r, year, record, k.vtable, &fields, false);
    SCOREBOARD.record(error.is_none(), || format!("{}({:#x}, {}, {:#x}): {}", name, this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The Irish League Cup's constructor: the vtable 0x96be20, discipline rule `discipline_rule::IRELAND`, eight stages
/// (seven groups and the knockout), IreLgeCup's collect_entrants (0x637dd0), set_predicted_positions (0x638420),
/// set_up_rules (0x6376c0), create_table (0x6375c0) and create_group_stage (0x6379c0), ire_lge_cup.cpp.
pub(crate) const IRISH_LEAGUE_CUP_CONSTRUCTOR: IrishLeagueCupConstructor = IrishLeagueCupConstructor { vtable: 0x96be20, discipline: discipline_rule::IRELAND, stages: 8, groups: 7,
    entrants: 0x637dd0, predicted: 0x638420, rules: 0x6376c0, first_group: 0x6375c0, group_stage: 0x6379c0, file: 0x9e9f4c };

/// Rule: the Irish League Cup at career creation: the year and the database competition, 5 substitutes on the bench and
/// 3 substitutions, the Irish discipline rules, importance 12; with nothing saved, eight stages (seven groups and the
/// knockout), the 26 entrants, the statistics, the predicted positions, the rules, the first group's table (the cup is
/// itself the league of the first group), its fixtures and order and the stages of the seven groups.
///
/// `construct_irish_league_cup` with `IRISH_LEAGUE_CUP_CONSTRUCTOR` (checked instruction by instruction against
/// ItaCCup::ItaCCup 0x63cc40: only the vtable, the bench, the discipline rule, the stage and group counts, the routines
/// and the file change).
/// Original: `IreLgeCup::IreLgeCup` (0x636dd0, ire_lge_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ire_lge_cup(this: usize, year: u32, record: usize) -> usize {
    construct_irish_league_cup(League(this), year, record, &IRISH_LEAGUE_CUP_CONSTRUCTOR, "mwa_16 irish league cup new") // mwa16: the Irish League Cup
}

unsafe extern "thiscall" fn compare_construct_ire_lge_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_IRE_LGE_CUP);
    compare_irish_league_cup_constructor("construct_ire_lge_cup", this, year, record, &IRISH_LEAGUE_CUP_CONSTRUCTOR, || o(this, year, record))
}

/// Rule: the first group of the Irish League Cup has three clubs, the first three entrants, which leave the waiting
/// list; the group draw news follows.
///
/// `create_first_group_table`(3 clubs; out of memory, the error box at line 0x172 of ire_lge_cup.cpp) (ItaCCup's
/// init_first_group template, checked instruction by instruction: only the count, the file and the line). Returns 1.
/// Original: `IreLgeCup::create_table` (0x6375c0, ire_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_create_table(this: usize) -> u32 {
    create_first_group_table(League(this), 3, 0x9e9f4c, 0x172, "mwa_16 irish league cup first group")
}

unsafe extern "thiscall" fn compare_ire_lge_cup_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_LGE_CUP_CREATE_TABLE);
    compare_first_group_table("ire_lge_cup_create_table", League(this), 3, || o(this))
}

/// The Irish League Cup's group rules: 3 points for a win and 1 for a draw, the tie-breaks 1, 2 and 4 (goal difference,
/// goals for, head-to-head [inference]), the group winner goes through, category 4, a 100,000 prize fund.
pub(crate) const IRISH_LEAGUE_CUP_GROUP_RULES: GroupStageRules = GroupStageRules { win: 3, draw: 1, category: 4, tiebreak: [1, 2, 4, 0], slots: 1, prize: Some(0x186a0), bits: None }; // mwa16: one through per group

/// Rule: the Irish League Cup groups are single round robins: 3 points for a win and 1 for a draw, ties split by goal
/// difference, goals for and the head-to-head [inference]; the group winner goes through; category 4 (national cup); no
/// promotion or relegation; a 100,000 prize fund.
///
/// `set_cup_league_rules` with `IRISH_LEAGUE_CUP_GROUP_RULES` and one meeting (ItaCup::set_up_rules's template, checked
/// instruction by instruction: only the tie-break order and the prize change), then the round table. Returns 1.
/// Original: `IreLgeCup::set_up_rules` (0x6376c0, ire_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_set_up_rules(this: usize) -> u32 {
    covered!("mwa_16 irish league cup rules");
    set_cup_league_rules(League(this), &IRISH_LEAGUE_CUP_GROUP_RULES, 1)
}

// simplification: the virtual +0x3c allocates the round table: it does not run twice. Only the original; the plan
// checks the rules.
unsafe extern "thiscall" fn compare_ire_lge_cup_set_up_rules(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_LGE_CUP_SET_UP_RULES)(this);
    check_cup_league_rules("ire_lge_cup_set_up_rules", League(this), &IRISH_LEAGUE_CUP_GROUP_RULES, r);
    r
}

// ---------- batch 296: the Northern Irish League Cup: the constructor, the first group's table and the rules (nir_lge_cup.cpp) ----------

/// The Northern Irish League Cup's constructor: the vtable 0x96dc18, discipline rule
/// `discipline_rule::NORTHERN_IRELAND`, four stages (three groups and the knockout), NirLgeCup's collect_entrants
/// (0x78d800), set_predicted_positions (0x78dcd0), set_up_rules (0x78d0d0), create_table (0x78cfd0) and
/// create_group_stage (0x78d3c0), nir_lge_cup.cpp.
pub(crate) const NORTHERN_IRISH_LEAGUE_CUP_CONSTRUCTOR: IrishLeagueCupConstructor = IrishLeagueCupConstructor { vtable: 0x96dc18, discipline: discipline_rule::NORTHERN_IRELAND, stages: 4, groups: 3,
    entrants: 0x78d800, predicted: 0x78dcd0, rules: 0x78d0d0, first_group: 0x78cfd0, group_stage: 0x78d3c0, file: 0xa69698 };

/// Rule: the Northern Irish League Cup at career creation: the year and the database competition, 5 substitutes on the
/// bench and 3 substitutions, the Northern Irish discipline rules, importance 12; with nothing saved, four stages
/// (three groups and the knockout), the 20 entrants, the statistics, the predicted positions, the rules, the first
/// group's table (the cup is itself the league of the first group), its fixtures and order and the stages of the three
/// groups.
///
/// `construct_irish_league_cup` with `NORTHERN_IRISH_LEAGUE_CUP_CONSTRUCTOR` (checked instruction by instruction against
/// IreLgeCup::IreLgeCup: only the vtable, the discipline rule, the stage and group counts, the routines and the file).
/// Original: `NirLgeCup::NirLgeCup` (0x78c780, nir_lge_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_nir_lge_cup(this: usize, year: u32, record: usize) -> usize {
    construct_irish_league_cup(League(this), year, record, &NORTHERN_IRISH_LEAGUE_CUP_CONSTRUCTOR, "mwa_17 northern irish league cup new") // mwa17: the Northern Irish League Cup
}

unsafe extern "thiscall" fn compare_construct_nir_lge_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_NIR_LGE_CUP);
    compare_irish_league_cup_constructor("construct_nir_lge_cup", this, year, record, &NORTHERN_IRISH_LEAGUE_CUP_CONSTRUCTOR, || o(this, year, record))
}

/// Rule: the first group of the Northern Irish League Cup has five clubs, the first five entrants, which leave the
/// waiting list; the group draw news follows.
///
/// `create_first_group_table`(5 clubs; out of memory, the error box at line 0x17a of nir_lge_cup.cpp) (ItaCCup's
/// init_first_group template, checked instruction by instruction: only the file and the line). Returns 1.
/// Original: `NirLgeCup::create_table` (0x78cfd0, nir_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_create_table(this: usize) -> u32 {
    create_first_group_table(League(this), 5, 0xa69698, 0x17a, "mwa_17 northern irish league cup first group")
}

unsafe extern "thiscall" fn compare_nir_lge_cup_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NIR_LGE_CUP_CREATE_TABLE);
    compare_first_group_table("nir_lge_cup_create_table", League(this), 5, || o(this))
}

/// The Northern Irish League Cup's group rules: 3 points for a win and 1 for a draw, the tie-breaks 1, 2 and 4, the top
/// two go through, category 4, a 100,000 prize fund.
pub(crate) const NORTHERN_IRISH_LEAGUE_CUP_GROUP_RULES: GroupStageRules = GroupStageRules { win: 3, draw: 1, category: 4, tiebreak: [1, 2, 4, 0], slots: 2, prize: Some(0x186a0), bits: None }; // mwa17: two through per group

/// Rule: the Northern Irish League Cup groups are single round robins: 3 points for a win and 1 for a draw, ties split
/// by goal difference, goals for and the head-to-head [inference]; the top two go through; category 4 (national cup);
/// no promotion or relegation; a 100,000 prize fund.
///
/// `set_cup_league_rules` with `NORTHERN_IRISH_LEAGUE_CUP_GROUP_RULES` and one meeting (ItaCup::set_up_rules's
/// template), then the round table. Returns 1.
/// Original: `NirLgeCup::set_up_rules` (0x78d0d0, nir_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_set_up_rules(this: usize) -> u32 {
    covered!("mwa_17 northern irish league cup rules");
    set_cup_league_rules(League(this), &NORTHERN_IRISH_LEAGUE_CUP_GROUP_RULES, 1)
}

// simplification: the virtual +0x3c allocates the round table: it does not run twice. Only the original; the plan
// checks the rules.
unsafe extern "thiscall" fn compare_nir_lge_cup_set_up_rules(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NIR_LGE_CUP_SET_UP_RULES)(this);
    check_cup_league_rules("nir_lge_cup_set_up_rules", League(this), &NORTHERN_IRISH_LEAGUE_CUP_GROUP_RULES, r);
    r
}

// ---------- batch 296: the Irish League Cup: the next stage and the season turnover (ire_lge_cup.cpp) ----------

/// Rule: the Irish League Cup moves stage by stage through its seven groups; when the groups are over (stage 7) the
/// knockout of the eight group winners is built; past the last stage, the error box.
///
/// `advance_cup_stage` (the Italian cups' template, checked instruction by instruction against ItaCup::advance_stage
/// 0x63f5b0: only the file, the line 0x212 and the knockout routine IreLgeCup::create_knockout_stage 0x637c00 change).
/// Original: `IreLgeCup::advance_stage` (0x637930, ire_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_advance_stage(this: usize) {
    advance_cup_stage(League(this), 0x9e9f4c, 0x212, 7, 0x637c00) // mwa18: the Irish League Cup knockout after the seven groups
}

unsafe extern "thiscall" fn compare_ire_lge_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_IRE_LGE_CUP_ADVANCE_STAGE);
    ita_cup_compare_advance_stage("ire_lge_cup_advance_stage", League(this), || o(this))
}

/// The turnover of an Irish league cup (the cup is the league of its first group).
///
/// `league::SEASON_CLOSED` = 0; the table freed (`exe::league_free_table`(this, 0)); the round table freed and zeroed;
/// the season archived in the statistics (when there); the entrant vector (`ita_cup_fields::ENTRANT_CLUBS`) freed (not
/// zeroed: the entrant routine replaces it); with the current stage ≥ 0, the stages 0 to the current one destroyed
/// (their virtual 0 with 1; the slots are not zeroed); the year up by `year`; `league::CURRENT_STAGE` = −1. Then the
/// entrants (thiscall, ret 0, unchecked), the predicted positions (`comp_vtable::SET_PREDICTED_POSITIONS`) and, each
/// with its own error box (returning 0) when it returns 0: the rules (`comp_vtable::SET_UP_RULES`), the first group's
/// table (`table`), the fixtures and the order; finally the stage of each group (push g -> thiscall, ret 4), g from 0.
pub(crate) unsafe fn start_irish_league_cup_season(this: League, entrants: usize, table: usize, group_stage: usize, groups: u32, file: usize, rows: [u32; 4], year: i16) -> u32 {
    this.set_season_closed(0);
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    let v = read_usize(this.addr() + ita_cup_fields::ENTRANT_CLUBS);
    if v != 0 {
        exe::crt_free(v);
    }
    if (this.current_stage() as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(this.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
    this.set_year(this.year().wrapping_add(year));
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(entrants);
    entrants(this.addr());
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    // simplification: in the boxes the original returns the box's eax; the caller of the turnover does not read it
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, rows[0]);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(table);
    if table(this.addr()) == 0 {
        show_error_at(file, rows[1]);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, rows[2]);
        return 0;
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(file, rows[3]);
        return 0;
    }
    covered!("mwa_18 irish league cup turnover");
    let group: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(group_stage);
    let mut r = 0;
    for g in 0..groups {
        r = group(this.addr(), g) as u32;
    }
    r
}

/// Rule: at the season turn the Irish League Cup is rebuilt: the table, the round table, the entrants and the stages of
/// the last edition are freed, the statistics archived, the year moves on; then the 26 entrants, the predicted
/// positions, the rules, the first group's table, its fixtures and order and the stages of the seven groups.
///
/// `start_irish_league_cup_season` with IreLgeCup::collect_entrants (0x637dd0), create_table (0x6375c0),
/// create_group_stage (0x6379c0), seven groups and the error box lines 0x1e4, 0x1eb, 0x1f1 and 0x1f8 of
/// ire_lge_cup.cpp.
/// Original: `IreLgeCup::start_new_season` (0x637760, ire_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_start_new_season(this: usize) -> u32 {
    start_irish_league_cup_season(League(this), 0x637dd0, 0x6375c0, 0x6379c0, 7, 0x9e9f4c, [0x1e4, 0x1eb, 0x1f1, 0x1f8], 1) // mwa18: the Irish League Cup turnover
}

unsafe extern "thiscall" fn compare_ire_lge_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_LGE_CUP_START_NEW_SEASON);
    compare_state_league_new_season("ire_lge_cup_start_new_season", League(this), || o(this))
}

// ---------- batch 296: the Northern Irish League Cup: the next stage and the season turnover (nir_lge_cup.cpp) ----------

/// Rule: the Northern Irish League Cup moves stage by stage through its three groups; when the groups are over (stage
/// 3) the eight-club knockout is built; past the last stage, the error box.
///
/// `advance_cup_stage` (checked instruction by instruction against ItaCup::advance_stage: only the file, the line
/// 0x21a, the end of the groups 3 and the knockout routine NirLgeCup::create_knockout_stage 0x78d600 change).
/// Original: `NirLgeCup::advance_stage` (0x78d330, nir_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_advance_stage(this: usize) {
    advance_cup_stage(League(this), 0xa69698, 0x21a, 3, 0x78d600) // mwa19: the Northern Irish League Cup knockout after the three groups
}

unsafe extern "thiscall" fn compare_nir_lge_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_NIR_LGE_CUP_ADVANCE_STAGE);
    ita_cup_compare_advance_stage("nir_lge_cup_advance_stage", League(this), || o(this))
}

/// Rule: at the season turn the Northern Irish League Cup is rebuilt: the table, the round table, the entrants and the
/// stages of the last edition are freed, the statistics archived, the year moves on; then the 20 entrants, the
/// predicted positions, the rules, the first group's table, its fixtures and order and the stages of the three groups.
///
/// `start_irish_league_cup_season` with NirLgeCup::collect_entrants (0x78d800), create_table (0x78cfd0),
/// create_group_stage (0x78d3c0), three groups and the error box lines 0x1ec, 0x1f3, 0x1f9 and 0x200 of
/// nir_lge_cup.cpp (checked instruction by instruction against IreLgeCup::start_new_season).
/// Original: `NirLgeCup::start_new_season` (0x78d160, nir_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_start_new_season(this: usize) -> u32 {
    start_irish_league_cup_season(League(this), 0x78d800, 0x78cfd0, 0x78d3c0, 3, 0xa69698, [0x1ec, 0x1f3, 0x1f9, 0x200], 1) // mwa19: the Northern Irish League Cup turnover
}

unsafe extern "thiscall" fn compare_nir_lge_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NIR_LGE_CUP_START_NEW_SEASON);
    compare_state_league_new_season("nir_lge_cup_start_new_season", League(this), || o(this))
}

// ---------- batch %LEVA%: the schedules of the Northern Irish Premier and First Divisions (nir_prm.cpp, nir_first.cpp) ----------

/// The Northern Irish leagues' first twenty matchdays (rounds 0 to 19), from 12 August to 16 December: Saturdays
/// (weekday 5, code 1) and a few midweek rounds (weekday 2, code 2); no TV slots.
pub(crate) const NORTHERN_IRISH_LEAGUE_START: [FraThirdLeagueRound; 20] = [
    (12, 7, 0, 5, 1, &[(-1, -1, -1)]), // mwa20: the first Northern Irish matchday, 12 August
    (19, 7, 0, 5, 1, &[(-1, -1, -1)]), // 1: 19 aug
    (26, 7, 0, 5, 1, &[(-1, -1, -1)]), // 2: 26 aug
    (30, 7, 0, 2, 2, &[(-1, -1, -1)]), // 3: 30 aug
    (9, 8, 0, 5, 1, &[(-1, -1, -1)]), // 4: 9 sep
    (13, 8, 0, 2, 2, &[(-1, -1, -1)]), // 5: 13 sep
    (16, 8, 0, 5, 1, &[(-1, -1, -1)]), // 6: 16 sep
    (23, 8, 0, 5, 1, &[(-1, -1, -1)]), // 7: 23 sep
    (27, 8, 0, 2, 2, &[(-1, -1, -1)]), // 8: 27 sep
    (30, 8, 0, 5, 1, &[(-1, -1, -1)]), // 9: 30 sep
    (14, 9, 0, 5, 1, &[(-1, -1, -1)]), // 10: 14 oct
    (21, 9, 0, 5, 1, &[(-1, -1, -1)]), // 11: 21 oct
    (28, 9, 0, 5, 1, &[(-1, -1, -1)]), // 12: 28 oct
    (4, 10, 0, 5, 1, &[(-1, -1, -1)]), // 13: 4 nov
    (11, 10, 0, 5, 1, &[(-1, -1, -1)]), // 14: 11 nov
    (18, 10, 0, 5, 1, &[(-1, -1, -1)]), // 15: 18 nov
    (25, 10, 0, 5, 1, &[(-1, -1, -1)]), // 16: 25 nov
    (2, 11, 0, 5, 1, &[(-1, -1, -1)]), // 17: 2 dec
    (9, 11, 0, 5, 1, &[(-1, -1, -1)]), // 18: 9 dec
    (16, 11, 0, 5, 1, &[(-1, -1, -1)]), // 19: 16 dec
];

/// The Christmas pair (rounds 20 and 21) by how the 16 December round falls (k = its day − 1 December's + 1 − 13, so k
/// = 0 to 6 for 13 to 19 December after the move to the weekday): k 0 or 1, 23 and 30 December; k 2 or 3, 23 and 27
/// December (midweek); k 4 or 5, 20 (midweek) and 27 December (midweek); any other k, 20 (midweek) and 23 December.
pub(crate) const NORTHERN_IRISH_CHRISTMAS_0: [FraThirdLeagueRound; 2] = [(23, 11, 0, 5, 1, &[(-1, -1, -1)]), (30, 11, 0, 5, 1, &[(-1, -1, -1)])];
pub(crate) const NORTHERN_IRISH_CHRISTMAS_2: [FraThirdLeagueRound; 2] = [(23, 11, 0, 5, 1, &[(-1, -1, -1)]), (27, 11, 0, 2, 2, &[(-1, -1, -1)])];
pub(crate) const NORTHERN_IRISH_CHRISTMAS_4: [FraThirdLeagueRound; 2] = [(20, 11, 0, 2, 2, &[(-1, -1, -1)]), (27, 11, 0, 2, 2, &[(-1, -1, -1)])];
pub(crate) const NORTHERN_IRISH_CHRISTMAS_DEFAULT: [FraThirdLeagueRound; 2] = [(20, 11, 0, 2, 2, &[(-1, -1, -1)]), (23, 11, 0, 5, 1, &[(-1, -1, -1)])];

/// The Northern Irish Premier Division's last fourteen matchdays (rounds 22 to 35), from 6 January to 28 April of the
/// next year, on weekday 5 (one midweek round on 11 April).
pub(crate) const NORTHERN_IRISH_PREMIER_END: [FraThirdLeagueRound; 14] = [
    (6, 0, 1, 5, 1, &[(-1, -1, -1)]), // 22: 6 jan (+1)
    (13, 0, 1, 5, 1, &[(-1, -1, -1)]), // 23: 13 jan (+1)
    (17, 0, 1, 5, 1, &[(-1, -1, -1)]), // 24: 17 jan (+1)
    (27, 0, 1, 5, 1, &[(-1, -1, -1)]), // 25: 27 jan (+1)
    (3, 1, 1, 5, 1, &[(-1, -1, -1)]), // 26: 3 feb (+1)
    (10, 1, 1, 5, 1, &[(-1, -1, -1)]), // 27: 10 feb (+1)
    (24, 1, 1, 5, 1, &[(-1, -1, -1)]), // 28: 24 feb (+1)
    (3, 2, 1, 5, 1, &[(-1, -1, -1)]), // 29: 3 mar (+1)
    (17, 2, 1, 5, 1, &[(-1, -1, -1)]), // 30: 17 mar (+1)
    (31, 2, 1, 5, 1, &[(-1, -1, -1)]), // 31: 31 mar (+1)
    (11, 3, 1, 2, 2, &[(-1, -1, -1)]), // 32: 11 apr (+1)
    (14, 3, 1, 5, 1, &[(-1, -1, -1)]), // 33: 14 apr (+1)
    (21, 3, 1, 5, 1, &[(-1, -1, -1)]), // 34: 21 apr (+1)
    (28, 3, 1, 5, 1, &[(-1, -1, -1)]), // 35: 28 apr (+1)
];

/// The Northern Irish First Division's last fourteen matchdays: the Premier's dates moved to weekday 6 (the 11 April
/// midweek round the same).
pub(crate) const NORTHERN_IRISH_FIRST_END: [FraThirdLeagueRound; 14] = [
    (6, 0, 1, 6, 1, &[(-1, -1, -1)]), // 22: 6 jan (+1)
    (13, 0, 1, 6, 1, &[(-1, -1, -1)]), // 23: 13 jan (+1)
    (17, 0, 1, 6, 1, &[(-1, -1, -1)]), // 24: 17 jan (+1)
    (27, 0, 1, 6, 1, &[(-1, -1, -1)]), // 25: 27 jan (+1)
    (3, 1, 1, 6, 1, &[(-1, -1, -1)]), // 26: 3 feb (+1)
    (10, 1, 1, 6, 1, &[(-1, -1, -1)]), // 27: 10 feb (+1)
    (24, 1, 1, 6, 1, &[(-1, -1, -1)]), // 28: 24 feb (+1)
    (3, 2, 1, 6, 1, &[(-1, -1, -1)]), // 29: 3 mar (+1)
    (17, 2, 1, 6, 1, &[(-1, -1, -1)]), // 30: 17 mar (+1)
    (31, 2, 1, 6, 1, &[(-1, -1, -1)]), // 31: 31 mar (+1)
    (11, 3, 1, 2, 2, &[(-1, -1, -1)]), // 32: 11 apr (+1)
    (14, 3, 1, 6, 1, &[(-1, -1, -1)]), // 33: 14 apr (+1)
    (21, 3, 1, 6, 1, &[(-1, -1, -1)]), // 34: 21 apr (+1)
    (28, 3, 1, 6, 1, &[(-1, -1, -1)]), // 35: 28 apr (+1)
];

/// The 36-matchday schedule of a Northern Irish league (the league's virtual `comp_vtable::CREATE_SCHEDULE` at stage
/// −1): `out` (when given) = 1, `rounds` = 36, `extra` = 0; the table (`exe::crt_malloc` of 36 × 0x41; out of memory,
/// the error box at `row` and 0); the first twenty rounds (`fra_third_write_league_rounds`); 1 December of the league's
/// year (Date::Date `exe::date_date_549f70`(&d, 1, 11, year, −1), thiscall, ret 0x10) and the Christmas pair chosen by
/// k = (round 19's day − that day + 1, in 16 bits) − 13 (`NORTHERN_IRISH_CHRISTMAS_*`); then the last fourteen rounds
/// `end`. Returns the table.
pub(crate) unsafe fn northern_irish_league_schedule(this: League, rounds: *mut i16, extra: *mut i16, out: *mut i32, end: &[FraThirdLeagueRound], file: usize, row: u32, branch: &'static str) -> usize {
    if !out.is_null() {
        *out = 1;
    }
    *rounds = 0x24;
    *extra = 0;
    let t = exe::crt_malloc(0x924);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    covered!(branch);
    let year = this.year() as u16 as u32;
    fra_third_write_league_rounds(t, year, 0, &NORTHERN_IRISH_LEAGUE_START);
    let mut d = [0u16; 4];
    exe::date_date_549f70(&mut d, 1, 0xb, year, 0xffff_ffff);
    let k = (read_u16(t + 19 * 0x41).wrapping_sub(d[0]).wrapping_add(1) as i16 as i32).wrapping_sub(0xd); // mwa20: the Christmas case
    let christmas: &[FraThirdLeagueRound] = match k {
        0 | 1 => &NORTHERN_IRISH_CHRISTMAS_0,
        2 | 3 => &NORTHERN_IRISH_CHRISTMAS_2,
        4 | 5 => &NORTHERN_IRISH_CHRISTMAS_4,
        _ => &NORTHERN_IRISH_CHRISTMAS_DEFAULT,
    };
    fra_third_write_league_rounds(t, year, 20, christmas);
    fra_third_write_league_rounds(t, year, 22, end);
    t
}

/// Rule: the Northern Irish Premier Division plays 36 matchdays (ten clubs meeting four times), from 12 August to 28
/// April of the next year, mostly on Saturdays with a few midweek rounds; the two Christmas rounds depend on the
/// weekday of the 16 December round; only the league stage (no play-off).
///
/// Only stage −1 (the byte; otherwise 0 with nothing written): `northern_irish_league_schedule` with
/// `NORTHERN_IRISH_PREMIER_END` and the error box at line 0xa0 of nir_prm.cpp.
/// Original: `NirPrm::create_schedule` (0x78e330, nir_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn nir_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    northern_irish_league_schedule(League(this), rounds, extra, out, &NORTHERN_IRISH_PREMIER_END, 0xa696d0, 0xa0, "mwa_20 northern irish premier schedule")
}

unsafe extern "thiscall" fn compare_nir_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_NIR_PRM_CREATE_SCHEDULE);
    compare_c1_schedule("nir_prm_create_schedule", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, l| nir_prm_create_schedule(this, stage, r, e, l))
}

/// The Northern Irish promotion play-off: one round of two clubs over two legs seven days apart, drawn on 29 April and
/// first played on 5 May of the next year (weekday 3); code 0xa0, tie rule 3 [inference: away goals]; category 5; no
/// prize.
pub(crate) const NORTHERN_IRISH_PLAY_OFF: EngCcCupCupRound = cql10r((0x1d, 3, 1, 0), (5, 4, 1, 3, 1, 0), 0xa0, (0, 0, 3), 5, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)); // mwa20: the play-off on 5 May

/// Rule: the Northern Irish First Division plays 36 matchdays (ten clubs meeting four times), on the Premier's dates up
/// to Christmas and on Sundays after it [inference: weekday 6]; then the promotion play-off against the Premier's second
/// last, two legs seven days apart from 5 May.
///
/// Stage −1: `northern_irish_league_schedule` with `NORTHERN_IRISH_FIRST_END` and the error box at line 0xae of
/// nir_first.cpp. Stage 0: `out` (when given) = 0, `rounds` = 1, `extra` = 0, the table (`exe::crt_malloc` of 0x68; out of
/// memory, the box at line 0x163 and 0) with `NORTHERN_IRISH_PLAY_OFF` (`eng_cc_cup_write_cup_round`; the original writes
/// the same fields in another order). Other stages: 0.
/// Original: `NirFirst::create_schedule` (0x78b310, nir_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn nir_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let this = League(this);
    match stage as u8 {
        0xff => northern_irish_league_schedule(this, rounds, extra, out, &NORTHERN_IRISH_FIRST_END, 0xa6965c, 0xae, "mwa_20 northern irish first schedule"),
        0 => {
            if !out.is_null() {
                *out = 0;
            }
            *rounds = 1;
            *extra = 0;
            let t = exe::crt_malloc(0x68);
            if t == 0 {
                show_error_at(0xa6965c, 0x163);
                return 0;
            }
            covered!("mwa_20 northern irish play-off round");
            eng_cc_cup_write_cup_round(t, 0, &NORTHERN_IRISH_PLAY_OFF, this.year() as i32 as u32);
            t
        }
        _ => 0,
    }
}

unsafe extern "thiscall" fn compare_nir_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_NIR_FIRST_CREATE_SCHEDULE);
    compare_c1_schedule("nir_first_create_schedule", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, l| nir_first_create_schedule(this, stage, r, e, l))
}

// ---------- batch 296: the Irish Super Cup: the constructor, the rules and the schedule (ire_super_cup.cpp) ----------

/// The Irish Super Cup's league rules: a single round robin of four clubs (one meeting), 3 points for a win and 1 for a
/// draw, match category 2, the tie-breaks 4, 1, 2 and 3 (head-to-head [inference], goal difference, goals for and the
/// row byte), no promotion or relegation places; no prize fund written.
pub(crate) const IRISH_SUPER_CUP_RULES: GroupStageRules = GroupStageRules { win: 3, draw: 1, category: 2, tiebreak: [4, 1, 2, 3], slots: 0, prize: None, bits: None }; // mwa21: the Super Cup tie-breaks

/// The body of `construct_ire_super_cup`.
///
/// Over League::League (`exe::new_league`): `league::COMPETITION` = the record, the vtable 0x96c0c8, `league::YEAR`, 7
/// substitutes on the bench and 5 substitutions, discipline rule `discipline_rule::IRELAND`. If the saved state loads
/// (`exe::league_load_from_temp_file`(this, 1); ≠ 0) it returns this. Otherwise `league::STAGE_INDEX` =
/// `stage_index::MAIN`, `league::CURRENT_STAGE` = −1, no stages; the rules written inline with the round table
/// (`set_cup_league_rules` with `IRISH_SUPER_CUP_RULES` and one meeting: the same fields the original writes, then the
/// virtual `comp_vtable::CREATE_SCHEDULE` into `league::ROUND_DATES`); `league::CLUB_COUNT` = 0, `league::TABLE` = 0 and
/// `league::ACTIVE` = 0 (the clubs come later, `ire_super_cup_update_for_date`); the statistics (`ita_cups_create_comp_stats`;
/// without them, the error box at line 0x51). Returns this.
pub(crate) unsafe fn construct_irish_super_cup(this: League, year: u32, record: usize) -> usize {
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), 0x96c0c8);
    this.set_year(year as u16 as i16);
    this.set_substitutes_named(7);
    this.set_substitutions_allowed(5);
    this.set_discipline_rule(discipline_rule::IRELAND);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!("mwa_21 irish super cup new");
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    this.set_stage_count(0);
    this.set_stages(0);
    set_cup_league_rules(this, &IRISH_SUPER_CUP_RULES, 1);
    this.set_club_count(0);
    this.set_table(0);
    write_u32(this.addr() + league::ACTIVE, 0);
    if !ita_cups_create_comp_stats(this) {
        show_error_at(0x9ea0b8, 0x51);
    }
    this.addr()
}

/// Rule: the Irish Super Cup at career creation: the year and the database competition, 7 substitutes on the bench and
/// 5 substitutions, the Irish discipline rules; with nothing saved, its rules (a four-club round robin) and its three
/// dates, with no clubs yet (they come once the European places are known), and the statistics.
/// Original: `IreSuperCup::IreSuperCup` (0x63acd0, ire_super_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ire_super_cup(this: usize, year: u32, record: usize) -> usize {
    construct_irish_super_cup(League(this), year, record) // mwa21: the Irish Super Cup
}

unsafe extern "thiscall" fn compare_construct_ire_super_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_IRE_SUPER_CUP);
    let r = o(this, year, record);
    let fields = [(league::STAGE_INDEX, 1u8, 0xff), (league::SUBSTITUTES_NAMED, 1, 7), (league::SUBSTITUTIONS_ALLOWED, 1, 5), (league::DISCIPLINE_RULE, 1, 0x10),
        (league::CLUB_COUNT, 2, 0), (league::TABLE, 4, 0), (league::ACTIVE, 4, 0), (league::STAGE_COUNT, 4, 0)];
    let error = ita_cups_check_constructor_plan(this, r, year, record, 0x96c0c8, &fields, false);
    SCOREBOARD.record(error.is_none(), || format!("construct_ire_super_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// Rule: the Irish Super Cup is a single round robin of four clubs: 3 points for a win and 1 for a draw, ties split by
/// the head-to-head [inference], goal difference and goals for; category 2; no promotion or relegation.
///
/// `set_cup_league_rules` with `IRISH_SUPER_CUP_RULES` and one meeting (ItaCup::set_up_rules's template: the fields, then
/// the round table). Returns 1.
/// Original: `IreSuperCup::set_up_rules` (0x63b4c0, ire_super_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_super_cup_set_up_rules(this: usize) -> u32 {
    covered!("mwa_21 irish super cup rules");
    set_cup_league_rules(League(this), &IRISH_SUPER_CUP_RULES, 1)
}

// simplification: the virtual +0x3c allocates the round table: it does not run twice. Only the original; the plan
// checks the rules.
unsafe extern "thiscall" fn compare_ire_super_cup_set_up_rules(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_SUPER_CUP_SET_UP_RULES)(this);
    check_cup_league_rules("ire_super_cup_set_up_rules", League(this), &IRISH_SUPER_CUP_RULES, r);
    r
}

/// The Irish Super Cup's three dates: 27 June (weekday 2), 29 June (weekday 4) and 1 July (weekday 6) of the cup's year;
/// no TV slots.
pub(crate) const IRISH_SUPER_CUP_ROUNDS: [FraThirdLeagueRound; 3] = [
    (27, 5, 0, 2, 2, &[(-1, -1, -1)]), // mwa21: the Super Cup opens on 27 June
    (29, 5, 0, 4, 2, &[(-1, -1, -1)]),
    (1, 6, 0, 6, 1, &[(-1, -1, -1)]),
];

pub(crate) const IRISH_SUPER_CUP_SCHEDULE: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 3, file: 0x9ea0b8, row: 0x91, table: &IRISH_SUPER_CUP_ROUNDS, branch: "mwa_21 irish super cup schedule" };

/// Rule: the Irish Super Cup is played over three matchdays: 27 June, 29 June and 1 July of its year, each moved to its
/// weekday.
///
/// The single-stage league schedule (`create_league_schedule`, FraThird::create_schedule's template: only stage −1;
/// `out` = 1, three rounds, `extra` = 0; the table of 3 × 0x41 bytes, out of memory the error box at line 0x91) with
/// `IRISH_SUPER_CUP_ROUNDS`. Returns the table.
/// Original: `IreSuperCup::create_schedule` (0x63af60, ire_super_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ire_super_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    create_league_schedule(League(this), stage, rounds, extra, out, &IRISH_SUPER_CUP_SCHEDULE)
}

unsafe extern "thiscall" fn compare_ire_super_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_IRE_SUPER_CUP_CREATE_SCHEDULE);
    compare_cfa_schedule("ire_super_cup_create_schedule", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, l| ire_super_cup_create_schedule(this, stage, r, e, l))
}

// ---------- batch 296: the Irish Super Cup: the table, the daily update and the season turnover (ire_super_cup.cpp) ----------

/// The clubs of the Irish Super Cup table, in the original's order (the plan of the comparer too): three passes over
/// all `G_CLUB_COUNT` clubs taking the Irish clubs (`club::NATION` = `KEY_NATION_REPUBLIC_OF_IRELAND`) entered in the
/// Champions League, then the UEFA Cup, then the Intertoto Cup (`club::CONTINENTAL_COMP_ID`); then, while places
/// remain, the Premier Division clubs (`irish_cup_club_count` clubs, by division) in table order, without checking
/// whether they are already in (a defect of the original: with fewer than four clubs in Europe, a Premier club in
/// Europe enters twice). Up to `n` clubs.
pub(crate) unsafe fn irish_super_cup_clubs(n: i16) -> Vec<usize> {
    let mut v: Vec<usize> = Vec::new();
    for key in [KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, KEY_CLUB_COMP_UEFA_CUP, KEY_CLUB_COMP_INTER_TOTO_CUP] { // mwa22: the European entrants first
        let mut i = 0i16;
        while (i as i32) < read_i32(G_CLUB_COUNT) {
            let c = club_at(i as isize);
            let nation = read_usize(c.addr() + club::NATION);
            if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_REPUBLIC_OF_IRELAND) && c.continental_comp_id() == read_u32(key) && (v.len() as i16) < n {
                v.push(c.addr());
            }
            if v.len() as i16 == n {
                break;
            }
            i = i.wrapping_add(1);
        }
    }
    let mut i = 0i16;
    while (i as i32) < irish_cup_club_count() && (v.len() as i16) < n {
        let c = club_at(i as isize);
        if irish_club_division_id(c) == Some(read_usize(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION)) {
            v.push(c.addr());
        }
        i = i.wrapping_add(1);
    }
    v
}

/// The body of `ire_super_cup_create_table`: `league::CLUB_COUNT` = 4; the table (`exe::crt_malloc` of 4 × 0x3b; out of
/// memory, the error box at line 0xb5 and −1); the clubs of `irish_super_cup_clubs` in rows 0 on (League::init_table_row
/// `exe::league_init_table_row`(this, row, club, 0, 0), thiscall, ret 0x10). Returns whether the four rows were filled.
pub(crate) unsafe fn create_irish_super_cup_table(this: League) -> i32 {
    this.set_club_count(4);
    let t = exe::crt_malloc(0xec);
    this.set_table(t);
    if t == 0 {
        show_error_at(0x9ea0b8, 0xb5);
        return -1;
    }
    covered!("mwa_22 irish super cup table");
    let clubs = irish_super_cup_clubs(this.club_count());
    for (k, &c) in clubs.iter().enumerate() {
        exe::league_init_table_row(this, k as u8 as u32, c, 0, 0);
    }
    (clubs.len() as i16 == this.club_count()) as i32
}

/// Rule: the Irish Super Cup has four clubs: the Irish clubs in the Champions League, then those in the UEFA Cup, then
/// those in the Intertoto Cup, completed by the Premier Division clubs in database order.
/// Original: `IreSuperCup::create_table` (0x63b0c0, ire_super_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_super_cup_create_table(this: usize) -> i32 {
    create_irish_super_cup_table(League(this))
}

// simplification: init_table_row allocates each row's form and head-to-head: it cannot run twice. Only the original;
// the plan: the clubs of `irish_super_cup_clubs` in the rows and the return.
unsafe extern "thiscall" fn compare_ire_super_cup_create_table(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_IRE_SUPER_CUP_CREATE_TABLE);
    let plan = irish_super_cup_clubs(4);
    let r = o(this);
    let l = League(this);
    let ok = l.table() != 0 && l.club_count() == 4 && r == (plan.len() == 4) as i32 && plan.iter().enumerate().all(|(k, &c)| read_usize(l.table() + k * 0x3b) == c);
    SCOREBOARD.record(ok, || format!("ire_super_cup_create_table({:#x}) returned {}, plan {:x?}", this, r, plan));
    r
}

/// Whether the Irish Super Cup is due today: not yet active (`league::ACTIVE` = 0), the UEFA seeding already in the
/// cup's year (the word `uefa_seeding::SEASON_YEAR` of [`G_UEFA_SEEDING`], zero-extended, equal to `league::YEAR`) and
/// the day's flag set.
pub(crate) unsafe fn irish_super_cup_due(this: League, flag: u32) -> bool {
    read_u32(this.addr() + league::ACTIVE) == 0 && read_u16(read_usize(G_UEFA_SEEDING) + uefa_seeding::SEASON_YEAR) as i32 == this.year() as i32 && flag != 0
}

/// Rule: once the European places of the Super Cup's year are known (the UEFA seeding has moved to that year), the Irish
/// Super Cup takes its four clubs, draws its fixtures and order and its predicted positions, once; then the day goes on
/// as in any league.
///
/// The league's daily update (thiscall(date, flag), ret 8): when due (`irish_super_cup_due`), the table
/// (`ire_super_cup_create_table` 0x63b0c0; 0: the error box at line 0x101), the fixtures (`exe::league_create_fixtures`;
/// 0: the box at line 0x106) and the order (`exe::league_sort_table`(this, 0); 0: the box at line 0x10c), each box
/// going on to the next step; the predicted positions (`comp_vtable::SET_PREDICTED_POSITIONS`) and `league::ACTIVE` = 1.
/// Then always League::update_for_date (`exe::league_update_for_date`(this, date, flag), thiscall, ret 8), whose return
/// it passes on.
/// Original: `IreSuperCup::update_for_date` (0x63b300, ire_super_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn ire_super_cup_update_for_date(this: usize, date: usize, flag: u32) -> u8 {
    let l = League(this);
    if irish_super_cup_due(l, flag) { // mwa22: the Super Cup is set up once the seeding reaches its year
        covered!("mwa_22 irish super cup set up");
        let table: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63b0c0usize);
        if table(this) == 0 {
            show_error_at(0x9ea0b8, 0x101);
        }
        if exe::league_create_fixtures(l) == 0 {
            show_error_at(0x9ea0b8, 0x106);
        }
        if exe::league_sort_table(l, 0usize) == 0 {
            show_error_at(0x9ea0b8, 0x10c);
        }
        bra_nat_first_call_virtual_no_args(this, comp_vtable::SET_PREDICTED_POSITIONS);
        write_u32(this + league::ACTIVE, 1);
    }
    exe::league_update_for_date(l, date, flag)
}

// simplification: the set-up allocates and draws, and the base update plays the day: it cannot run twice. Only the
// original; the plan: when due, the cup becomes active with its table; otherwise `league::ACTIVE` stays.
unsafe extern "thiscall" fn compare_ire_super_cup_update_for_date(this: usize, date: usize, flag: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(IDX_IRE_SUPER_CUP_UPDATE_FOR_DATE);
    let l = League(this);
    let due = irish_super_cup_due(l, flag);
    let before = read_u32(this + league::ACTIVE);
    let r = o(this, date, flag);
    let after = read_u32(this + league::ACTIVE);
    let ok = if due { after == 1 && l.table() != 0 } else { after == before };
    SCOREBOARD.record(ok, || format!("ire_super_cup_update_for_date({:#x}) due {} active {} -> {}", this, due, before, after));
    r
}

/// Rule: at the season turn the Irish Super Cup is cleared: the table and the round table freed, the statistics
/// archived, the stages destroyed, the year moved on and the rules (with the new dates) made again; it waits inactive
/// for its new clubs.
///
/// `league::SEASON_CLOSED` = 0; the table freed (`exe::league_free_table`(this, 0)); the round table freed and zeroed;
/// the season archived (when there are statistics); with the current stage ≥ 0, the stages 0 to the current one
/// destroyed (their virtual 0 with 1); the year up by `year`; `league::CURRENT_STAGE` = −1; the rules
/// (`comp_vtable::SET_UP_RULES`; 0: the error box at line 0x15d and return); `league::ACTIVE` = 0.
pub(crate) unsafe fn start_irish_super_cup_season(this: League, year: i16) {
    this.set_season_closed(0);
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    if (this.current_stage() as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(this.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
    this.set_year(this.year().wrapping_add(year));
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        return show_error_at(0x9ea0b8, 0x15d);
    }
    covered!("mwa_22 irish super cup turnover");
    write_u32(this.addr() + league::ACTIVE, 0);
}

/// Original: `IreSuperCup::start_new_season` (0x63b550, ire_super_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_super_cup_start_new_season(this: usize) {
    start_irish_super_cup_season(League(this), 1) // mwa22: the Irish Super Cup turnover
}

unsafe extern "thiscall" fn compare_ire_super_cup_start_new_season(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_IRE_SUPER_CUP_START_NEW_SEASON);
    compare_state_league_new_season("ire_super_cup_start_new_season", League(this), || {
        o(this);
        0
    });
}
