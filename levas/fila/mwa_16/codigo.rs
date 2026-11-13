// ---------- batch %%LEVA%%: the Irish League Cup: the constructor, the first group's table and the rules (ire_lge_cup.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
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
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this);
    check_cup_league_rules("ire_lge_cup_set_up_rules", League(this), &IRISH_LEAGUE_CUP_GROUP_RULES, r);
    r
}
