// ---------- mwb_24: the constructors of the two Welsh group cups (wel_lge_cup.cpp, wel_prm_cup.cpp) ----------

/// The body of the Welsh group cups' constructors (thiscall, ret 8: this, year, the competition record): the Irish
/// league cups' frame (`construct_irish_league_cup`, IreLgeCup::IreLgeCup read instruction by instruction) with the
/// league flags 8 instead of 0x28 and the error box lines `rows` (0x53, 0x5d, 0x64, 0x6a and 0x71 in both files).
///
/// Over League::League (`exe::new_league`, thiscall, ret 0): `league::COMPETITION` = the record, the vtable,
/// `league::YEAR`, 3 substitutions allowed, 5 substitutes named, the discipline rule and importance 0xc. If the saved
/// state loads (`exe::league_load_from_temp_file`(this, 1); ≠ 0) it returns this. Otherwise, in order:
/// `league::LEAGUE_FLAGS` = 8, `league::STAGE_INDEX` = `stage_index::MAIN`, `league::CURRENT_STAGE` = −1, the stage
/// count and its array (`exe::crt_malloc` of 4 × stages, unchecked); the entrants (thiscall, ret 0, unchecked); the
/// statistics (`ita_cups_create_comp_stats`; without them, the box of `rows[0]`); the predicted positions (unchecked);
/// the rules (failing on 0: `rows[1]`); the first group's table (failing on 0: `rows[2]`); the fixtures
/// (`exe::league_create_fixtures`; `rows[3]`); the order (`exe::league_sort_table`(this, 0); `rows[4]`); the stage of
/// each other group (push g; mov ecx, this -> thiscall, ret 4), g from 0. Each box returns this. Returns this.
pub(crate) unsafe fn construct_welsh_group_cup(this: League, year: u32, record: usize, k: &IrishLeagueCupConstructor, rows: [u32; 5], branch: &'static str) -> usize {
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), k.vtable);
    this.set_year(year as u16 as i16); // mwb24: the cup's year
    this.set_substitutions_allowed(3);
    this.set_substitutes_named(5);
    this.set_discipline_rule(k.discipline);
    this.set_importance(0xc);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!(branch);
    this.set_league_flags(8);
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
        show_error_at(k.file, rows[0]);
        return this.addr();
    }
    call(k.predicted);
    if call(k.rules) == 0 {
        show_error_at(k.file, rows[1]);
        return this.addr();
    }
    if call(k.first_group) == 0 {
        show_error_at(k.file, rows[2]);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(k.file, rows[3]);
        return this.addr();
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(k.file, rows[4]);
        return this.addr();
    }
    let group: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(k.group_stage);
    for g in 0..k.groups {
        group(this.addr(), g);
    }
    this.addr()
}

/// The comparer of the Welsh group cups' constructors: only the original runs, and the plan is checked
/// (`ita_cups_check_constructor_plan`: this, the vtable, the record, the year and, on a new cup, the fixed fields with
/// the league flags 8 and the statistics).
pub(crate) unsafe fn compare_welsh_group_cup_constructor(name: &str, this: usize, year: u32, record: usize, k: &IrishLeagueCupConstructor, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let fields = [(league::STAGE_COUNT, 4u8, k.stages), (league::STAGE_INDEX, 1, 0xff), (league::SUBSTITUTES_NAMED, 1, 5), (league::SUBSTITUTIONS_ALLOWED, 1, 3),
        (league::DISCIPLINE_RULE, 1, k.discipline as u32), (league::IMPORTANCE, 1, 0xc), (league::LEAGUE_FLAGS, 2, 8)];
    let error = ita_cups_check_constructor_plan(this, r, year, record, k.vtable, &fields, false);
    SCOREBOARD.record(error.is_none(), || format!("{}({:#x}, {}, {:#x}): {}", name, this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The error box lines of both Welsh group cups' constructors (the statistics, the rules, the first group, the fixtures
/// and the order).
pub(crate) const WELSH_GROUP_CUP_CONSTRUCTOR_ROWS: [u32; 5] = [0x53, 0x5d, 0x64, 0x6a, 0x71];

/// The Welsh League Cup's constructor: the vtable 0x971034, discipline rule `discipline_rule::WALES`, six stages (five
/// groups and the knockout), WelLgeCup's collect_entrants (0x927010), set_predicted_positions (0x9274f0), set_up_rules
/// (0x926800), create_table (0x926700) and create_group_stage (0x926b00), wel_lge_cup.cpp.
pub(crate) const WELSH_LEAGUE_CUP_CONSTRUCTOR: IrishLeagueCupConstructor = IrishLeagueCupConstructor { vtable: 0x971034, discipline: discipline_rule::WALES, stages: 6, groups: 5,
    entrants: 0x927010, predicted: 0x9274f0, rules: 0x926800, first_group: 0x926700, group_stage: 0x926b00, file: 0xad56f4 };

/// Rule: the Welsh League Cup at career creation: the year and the database competition, 5 substitutes on the bench
/// and 3 substitutions, the Welsh discipline rules, importance 12; with nothing saved, six stages (five groups and the
/// knockout), the 18 entrants, the statistics, the predicted positions, the rules, the first group's table (the cup is
/// itself the league of the first group), its fixtures and order and the stages of the five other groups.
///
/// `construct_welsh_group_cup` with `WELSH_LEAGUE_CUP_CONSTRUCTOR` (read instruction by instruction against
/// IreLgeCup::IreLgeCup: only the vtable, the discipline rule, the league flags, the stage and group counts, the
/// routines, the file and the lines change).
/// Original: `WelLgeCup::WelLgeCup` (0x925e90, wel_lge_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_wel_lge_cup(this: usize, year: u32, record: usize) -> usize {
    construct_welsh_group_cup(League(this), year, record, &WELSH_LEAGUE_CUP_CONSTRUCTOR, WELSH_GROUP_CUP_CONSTRUCTOR_ROWS, "mwb_24 welsh league cup new")
}

/// Comparer for `construct_wel_lge_cup` (original only, the plan: `compare_welsh_group_cup_constructor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_wel_lge_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_welsh_group_cup_constructor("construct_wel_lge_cup", this, year, record, &WELSH_LEAGUE_CUP_CONSTRUCTOR, || o(this, year, record))
}

/// The Welsh Premier Cup's constructor: the vtable 0x9710e8, no discipline rule (`discipline_rule::NONE`), two stages
/// (the second group and the knockout), WelPrmCup's build_entrant_list (0x928860), set_predicted_positions (0x928e00),
/// set_up_rules (0x928160), create_group_table (0x928060) and create_group_stage (0x928450), wel_prm_cup.cpp.
pub(crate) const WELSH_PREMIER_CUP_CONSTRUCTOR: IrishLeagueCupConstructor = IrishLeagueCupConstructor { vtable: 0x9710e8, discipline: discipline_rule::NONE, stages: 2, groups: 1,
    entrants: 0x928860, predicted: 0x928e00, rules: 0x928160, first_group: 0x928060, group_stage: 0x928450, file: 0xad572c };

/// Rule: the Welsh Premier Cup at career creation: the year and the database competition, 5 substitutes on the bench
/// and 3 substitutions, no discipline rule of its own, importance 12; with nothing saved, two stages (the second group
/// and the knockout), the twelve entrants, the statistics, the predicted positions, the rules, the first group's table
/// (the cup is itself the league of the first group), its fixtures and order and the stage of the second group.
///
/// `construct_welsh_group_cup` with `WELSH_PREMIER_CUP_CONSTRUCTOR` (read instruction by instruction against
/// WelLgeCup::WelLgeCup: the registers differ and the group loop is a single call; only the vtable, the discipline
/// rule, the stage and group counts, the routines and the file change).
/// Original: `WelPrmCup::WelPrmCup` (0x9277f0, wel_prm_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_wel_prm_cup(this: usize, year: u32, record: usize) -> usize {
    construct_welsh_group_cup(League(this), year, record, &WELSH_PREMIER_CUP_CONSTRUCTOR, WELSH_GROUP_CUP_CONSTRUCTOR_ROWS, "mwb_24 welsh premier cup new")
}

/// Comparer for `construct_wel_prm_cup` (original only, the plan: `compare_welsh_group_cup_constructor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_wel_prm_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    compare_welsh_group_cup_constructor("construct_wel_prm_cup", this, year, record, &WELSH_PREMIER_CUP_CONSTRUCTOR, || o(this, year, record))
}
