use super::*;

/// Rule: the Welsh League Cup takes the 18 clubs of the League of Wales, ranked by reputation and shuffled within each
/// band of three (the band is a draw pot: the six groups take one club from each of the three pots [inference]); fewer
/// than 18 clubs in the league shows the error box.
/// Original: `WelLgeCup::collect_entrants` (0x927010, wel_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_collect_entrants(this: usize) -> i32 {
    collect_welsh_league_cup_entrants(League(this)).0
}

/// Comparer for `wel_lge_cup_collect_entrants` (side by side with the random generator restored: `compare_select_entrants`).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_WEL_LGE_CUP_COLLECT_ENTRANTS);
    compare_select_entrants("wel_lge_cup_collect_entrants", this, || o(this), || collect_welsh_league_cup_entrants(League(this)))
}

/// Rule: the Welsh League Cup's predicted positions (at creation and at the turnover): the 18 entrants by reputation;
/// the knockout (stage 5) gives 1 to the winner and 2 to the runner-up, then a shared place to the losers of each
/// earlier round; the rest in bands of 6; the board and press expectation of each club [inference].
///
/// `italian_cup_predicted_positions` with the knockout stage 5, bands of 6 and the boxes at lines 0x3b5 and 0x3bd of
/// wel_lge_cup.cpp (read instruction by instruction against ItaCCup::set_predicted_positions: only the file, the lines,
/// the stage and the band change).
/// Original: `WelLgeCup::set_predicted_positions` (0x9274f0, wel_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_set_predicted_positions(this: usize) {
    italian_cup_predicted_positions(this, 0xad56f4, (0x3b5, 0x3bd), 5, 6, "mwb_22 welsh league cup predicted") // mwb22: the Welsh League Cup bands of 6
}

/// Comparer for `wel_lge_cup_set_predicted_positions` (`compare_italian_cup_predicted`, side by side).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WEL_LGE_CUP_SET_PREDICTED_POSITIONS);
    compare_italian_cup_predicted("wel_lge_cup_set_predicted_positions", this, wel_lge_cup_set_predicted_positions, || o(this));
}

/// Rule: at the season turn the Welsh League Cup is rebuilt: the table, the round table, the entrants and the stages of
/// the last edition are freed, the statistics archived, the year moves on; then the 18 entrants, the predicted
/// positions, the rules, the first group's table, its fixtures and order and the stages of the five other groups.
///
/// `start_irish_league_cup_season` with WelLgeCup::collect_entrants (0x927010), create_table (0x926700),
/// create_group_stage (0x926b00), five groups and the error box lines 0x1f4, 0x1fb, 0x201 and 0x208 of wel_lge_cup.cpp
/// (read instruction by instruction against IreLgeCup::start_new_season: only the routines, the group count, the file
/// and the lines change).
/// Original: `WelLgeCup::start_new_season` (0x9268a0, wel_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_start_new_season(this: usize) -> u32 {
    start_irish_league_cup_season(League(this), 0x927010, 0x926700, 0x926b00, 5, 0xad56f4, [0x1f4, 0x1fb, 0x201, 0x208], 1) // mwb22: the Welsh League Cup turnover
}

/// Comparer for `wel_lge_cup_start_new_season` (original only: `compare_state_league_new_season`, the year up by one).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WEL_LGE_CUP_START_NEW_SEASON);
    compare_state_league_new_season("wel_lge_cup_start_new_season", League(this), || o(this))
}

/// Rule: the Welsh League Cup's groups (stages up to 4, and the cup's own table) play six matchdays, weekly from 23
/// August to 27 September (a double round robin of three clubs), each group a league stage (is_league = 1, label 0x3e9
/// + stage); the knockout (stage 5) has eight clubs: quarter-finals on 11 October, two-legged semi-finals from 25
/// October and the final on 10 January of the next year.
///
/// `create_schedule` with the knockout at stage 5, the label 0x3e9, `WELSH_LEAGUE_CUP_GROUP_ROUNDS`,
/// `WELSH_LEAGUE_CUP_KNOCKOUT_ROUNDS` and the boxes at lines 0xc0 and 0xeb of wel_lge_cup.cpp (read instruction by
/// instruction against IreLgeCup::create_schedule: only the stage, the dates, the rounds and the lines change). Above
/// stage 5: 0.
/// Original: `WelLgeCup::create_schedule` (0x9261e0, wel_lge_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_schedule(League(this), stage, rounds, extra, league, 5, 0x3e9, &WELSH_LEAGUE_CUP_GROUP_ROUNDS, &WELSH_LEAGUE_CUP_KNOCKOUT_ROUNDS, 0xad56f4, 0xc0, 0xeb,
        "mwb_23 welsh league cup groups", "mwb_23 welsh league cup knockout rounds")
}

/// Comparer for `wel_lge_cup_create_schedule` (side by side: `ita_cup_compare_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_WEL_LGE_CUP_CREATE_SCHEDULE);
    ita_cup_compare_schedule("wel_lge_cup_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| wel_lge_cup_create_schedule(this, stage, r, e, l), 5)
}

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_WEL_LGE_CUP);
    compare_welsh_group_cup_constructor("construct_wel_lge_cup", this, year, record, &WELSH_LEAGUE_CUP_CONSTRUCTOR, || o(this, year, record))
}

/// rule: the Welsh League Cup's own table holds one group of three: the first entrant of each pool row (the pool is 6
/// columns of 3; the column of group 0 is the first), taken out of the pool; then the group draw news; out of memory,
/// the box of line 0x182 and 0
/// Original: `WelLgeCup::create_table` (0x926700, wel_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_create_table(this: usize) -> u32 {
    welsh_cup_group_table(League(this), 3, 6, 0xad56f4, 0x182, 0, "mwb_18 welsh league cup table") // mwb18: the League Cup first group
}

/// Comparer for `wel_lge_cup_create_table` (the plan, original only).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WEL_LGE_CUP_CREATE_TABLE);
    compare_welsh_cup_group_table("wel_lge_cup_create_table", League(this), 3, 6, || o(this))
}

/// rule: the Welsh League Cup groups play 2 rounds, 3 points for a win and 1 for a draw, category 4; the winner of each
/// group goes through; tie-breaks codes 4, 1 and 2 [inference: head-to-head, goal difference, goals for]; a prize fund
/// of 100,000; builds the round schedule of the main stage (virtual +0x3c); returns 1
/// Original: `WelLgeCup::set_up_rules` (0x926800, wel_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_set_up_rules(this: usize) -> u32 {
    covered!("mwb_18 welsh league cup rules");
    eng_prm_apply_league_rules(League(this), &welsh_league_cup_rules())
}

/// Comparer for `wel_lge_cup_set_up_rules` (side by side on the rule bytes, the copy without the round table).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WEL_LGE_CUP_SET_UP_RULES);
    eng_prm_compare_league_rules("wel_lge_cup_set_up_rules", League(this), welsh_league_cup_rules(), || o(this), || wel_lge_cup_set_up_rules(this))
}

/// rule: the Welsh League Cup moves from group to group (stages 0 to 4 are the other five groups) and, on reaching stage
/// 5, builds the knockout stage of the eight best (WelLgeCup::create_knockout_stage 0x926d00); past the last stage, the
/// box of line 0x222
/// Original: `WelLgeCup::advance_stage` (0x926a70, wel_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_advance_stage(this: usize) {
    welsh_cup_advance_stage(League(this), 0xad56f4, 0x222, 5, 0x926d00)
}

/// Comparer for `wel_lge_cup_advance_stage`: only the original runs (the knockout builds a stage); the current stage
/// stays at the last stage, otherwise goes up by one.
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WEL_LGE_CUP_ADVANCE_STAGE);
    compare_state_league_advance_stage("wel_lge_cup_advance_stage", League(this), || o(this))
}

/// rule: the Welsh League Cup's season honours are those of its knockout stage (the sixth stage slot): the call goes on
/// to that stage's own `record_season_honours` (virtual +0x30), whose eax it returns
/// Original: `WelLgeCup::record_season_honours` (0x926890, wel_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_record_season_honours(this: usize) -> u32 {
    covered!("mwb_19 welsh league cup honours");
    let knockout = read_usize(League(this).stages() + 4 * 5); // mwb19: the League Cup knockout slot
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(knockout) + comp_vtable::RECORD_SEASON_HONOURS));
    f(knockout)
}

/// Comparer for `wel_lge_cup_record_season_honours` (original only).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_record_season_honours(this: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WEL_LGE_CUP_RECORD_SEASON_HONOURS)(this)
}

/// rule: in the six Welsh League Cup groups, a club going through gets the qualification record (weight 40) and one
/// going out the failed-to-qualify record (weight 40); in the knockout (stage 5) a club going through gets the
/// qualification by round and one going out the knocked-out record (weight 50 to 100 over 5 rounds), and the winner
/// and the runner-up their final records
/// Original: `WelLgeCup::update_staff_records_for_status` (0x9271e0, wel_lge_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, a6: u32) -> u32 {
    covered!("mwb_19 welsh league cup staff records");
    welsh_cup_staff_records(League(this), club, outcome, stage, game, a6, 5, 5)
}

/// Comparer for `wel_lge_cup_update_staff_records_for_status` (original only, the return 0).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_WEL_LGE_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, a6);
    SCOREBOARD.record(r == 0, || format!("wel_lge_cup_update_staff_records_for_status({:#x}) returned {}", this, r));
    r
}

/// rule: Welsh League Cup group `g` (0 to 4; the sixth group is the cup's own table): three clubs, pool entries g + 1,
/// g + 7 and g + 13 (the pool's column g + 1), in a league stage of their own with tie-break list (1, 0, 0, 0); then the
/// group draw news; out of memory the box of line 0x249, without the stage that of 0x25f (and 0)
/// Original: `WelLgeCup::create_group_stage` (0x926b00, wel_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_create_group_stage(this: usize, g: u32) -> u32 {
    welsh_cup_group_stage(League(this), g, 3, 6, 1, 0xad56f4, (0x249, 0x25f), "mwb_19 welsh league cup group")
}

/// Comparer for `wel_lge_cup_create_group_stage` (original only, the plan).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_WEL_LGE_CUP_CREATE_GROUP_STAGE);
    compare_welsh_cup_group_stage("wel_lge_cup_create_group_stage", League(this), g, 3, 6, || o(this, g))
}

/// The body of `wel_lge_cup_collect_entrants`; returns (the routine's return, the entries to compare).
///
/// `ita_cup_fields::ENTRANT_CLUB_COUNT` = 18; the pool (`exe::crt_calloc`(4, 18) into `ita_cup_fields::ENTRANT_CLUBS`;
/// none: the box of line 0x2cd and 0). One pass over the clubs (`G_CLUB_COUNT`, a signed word index, in table order):
/// each club whose `club::DIVISION` is the League of Wales ([`KEY_CLUB_COMP_WELSH_PREMIER_DIVISION`]) goes into the
/// next entry (no bound check, as the original). The entries found are sorted by reputation (`exe::crt_qsort`(pool, n,
/// 4, 0x4b07d0), cdecl) and each of the six bands of three is shuffled (`exe::comp_util_shuffle_pointer_array`(band,
/// 3)); with fewer than 18 found, the box of line 0x2e7 (and still 1). Returns 1.
pub(crate) unsafe fn collect_welsh_league_cup_entrants(this: League) -> (i32, i32) {
    let t = this.addr();
    write_i32(t + ita_cup_fields::ENTRANT_CLUB_COUNT, 0x12);
    let v = exe::crt_calloc(4, 0x12);
    write_usize(t + ita_cup_fields::ENTRANT_CLUBS, v);
    if v == 0 {
        show_error_at(0xad56f4, 0x2cd);
        return (0, 0);
    }
    let mut n = 0u32;
    let mut i = 0i16;
    while (i as i32) < read_i32(G_CLUB_COUNT) {
        let c = club_at(i as isize);
        let d = c.division();
        if !d.is_null() && read_usize(d.addr()) == read_usize(KEY_CLUB_COMP_WELSH_PREMIER_DIVISION) {
            write_usize(read_usize(t + ita_cup_fields::ENTRANT_CLUBS) + 4 * n as usize, c.addr());
            n += 1;
        }
        i = i.wrapping_add(1);
    }
    covered!("mwb_22 welsh league cup entrants");
    exe::crt_qsort(read_usize(t + ita_cup_fields::ENTRANT_CLUBS), n, 4, 0x4b07d0usize); // mwb22: the entrants by reputation
    for b in 0..6usize {
        exe::comp_util_shuffle_pointer_array(read_usize(t + ita_cup_fields::ENTRANT_CLUBS) + 0xc * b, 3);
    }
    if (n as i32) < read_i32(t + ita_cup_fields::ENTRANT_CLUB_COUNT) {
        show_error_at(0xad56f4, 0x2e7);
    }
    (1, 0x12)
}

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

/// Comparer for the Welsh group tables: the pool entries planned beforehand; only the original runs (it allocates and
/// sends news); then the count, the return 1, the rows and the pool entries zeroed.
pub(crate) unsafe fn compare_welsh_cup_group_table(name: &str, this: League, n: i16, stride: usize, original: impl FnOnce() -> u32) -> u32 {
    let pool = read_usize(this.addr() + WELSH_CUP_POOL);
    let plan: Vec<usize> = (0..n.max(0) as usize).map(|k| read_usize(pool + k * stride * 4)).collect();
    let r = original();
    let t = this.table();
    let mut ok = r == 1 && this.club_count() == n && t != 0;
    let mut found: Vec<usize> = Vec::new();
    if ok {
        found = (0..plan.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        ok = found == plan && (0..plan.len()).all(|k| read_usize(pool + k * stride * 4) == 0);
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}) original={} plan={:x?} table={:x?}", name, this.addr(), r, plan, found));
    r
}

/// The Welsh League Cup rule writes: the group winner goes through, tie-breaks 4, 1, 2 [inference: head-to-head, goal
/// difference, goals for].
pub(crate) fn welsh_league_cup_rules() -> Vec<(usize, u8, u32)> {
    welsh_cup_rules(1, [4, 1, 2]) // mwb18: the League Cup group rules
}

/// Comparer for the Welsh group stages: the pool entries of the group planned beforehand; only the original runs (it
/// builds a stage with its fixtures); then the return 1, the stage in its slot, the current stage and the pool entries
/// zeroed.
pub(crate) unsafe fn compare_welsh_cup_group_stage(name: &str, this: League, g: u32, n: u32, stride: usize, original: impl FnOnce() -> u32) -> u32 {
    let pool = read_usize(this.addr() + WELSH_CUP_POOL);
    let r = original();
    let ok = r == 1 && read_usize(this.stages() + 4 * g as usize) != 0 && this.current_stage() as u32 == g
        && (0..n as usize).all(|k| read_usize(pool + (g as usize + 1 + k * stride) * 4) == 0);
    SCOREBOARD.record(ok, || format!("{}({:#x}, group {}) returned {}, stage {:#x}, current {}", name, this.addr(), g, r, read_usize(this.stages() + 4 * g as usize), this.current_stage() as i32));
    r
}

/// The rules of the two Welsh group cups: category 4, 2 rounds, no promotion play-off or relegation, no division above
/// or below (−1), 3 points for a win and 1 for a draw, club sides, a prize fund of 100,000; `promoted` go through from
/// each group (`league::PROMOTED_COUNT`) and the tie-breaks (`tiebreaks`, the fourth 0).
pub(crate) fn welsh_cup_rules(promoted: u8, tiebreaks: [u8; 3]) -> Vec<(usize, u8, u32)> {
    vec![(league::MATCH_CATEGORY, 1, 4), (league::TIEBREAKS, 1, tiebreaks[0] as u32), (league::TIEBREAKS + 1, 1, tiebreaks[1] as u32), (league::TIEBREAKS + 2, 1, tiebreaks[2] as u32),
         (league::TIEBREAKS + 3, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 0),
         (league::LEGS, 2, 2), (league::UPPER_DIVISION_ID, 4, 0xffff_ffff), (league::LOWER_DIVISION_ID, 4, 0xffff_ffff), (league::WIN_POINTS, 1, 3),
         (league::DRAW_POINTS, 1, 1), (league::PROMOTED_COUNT, 1, promoted as u32), (league::CLUB_SIDES, 1, 1), (league::PRIZE_FUND, 4, 0x186a0)]
}

/// The Welsh League Cup's six group matchdays, weekly from 23 August to 27 September (weekday 2, code 2: midweek
/// [inference]); (day, month from 0, weekday, code).
pub(crate) const WELSH_LEAGUE_CUP_GROUP_ROUNDS: [ItaCupGroupRound; 6] = [(0x17, 7, 2, 2), (0x1e, 7, 2, 2), (6, 8, 2, 2), (0xd, 8, 2, 2), (0x14, 8, 2, 2), (0x1b, 8, 2, 2)]; // mwb23: the Welsh League Cup group matchdays

/// The Welsh League Cup's knockout: quarter-finals drawn on 28 September and played on 11 October (single matches, tie
/// rule 3, category 4); semi-finals over two legs fourteen days apart, drawn on 12 October and first played on 25
/// October (two-leg rule 3, category 4); the final drawn on 9 November and played on 10 January of the next year
/// (category 6); the tie winner's prize a share of the fund (−1).
pub(crate) const WELSH_LEAGUE_CUP_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 3] = [
    cql10r((0x1c, 8, 0, 3), (0xb, 9, 0, 2, 2, 0), 0x78, (0, 3, 0), 4, (8, 4, 8, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0xc, 9, 0, 3), (0x19, 9, 0, 2, 2, 0), 0x82, (0, 0, 3), 4, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((9, 0xa, 0, 3), (0xa, 0, 1, 2, 2, 0), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// The Welsh League Cup's constructor: the vtable 0x971034, discipline rule `discipline_rule::WALES`, six stages (five
/// groups and the knockout), WelLgeCup's collect_entrants (0x927010), set_predicted_positions (0x9274f0), set_up_rules
/// (0x926800), create_table (0x926700) and create_group_stage (0x926b00), wel_lge_cup.cpp.
pub(crate) const WELSH_LEAGUE_CUP_CONSTRUCTOR: IrishLeagueCupConstructor = IrishLeagueCupConstructor { vtable: 0x971034, discipline: discipline_rule::WALES, stages: 6, groups: 5,
    entrants: 0x927010, predicted: 0x9274f0, rules: 0x926800, first_group: 0x926700, group_stage: 0x926b00, file: 0xad56f4 };

/// Reads the Welsh League Cup's entrants from its temporary file (`load_welsh_cup_entrants`, the boxes of lines 0x348,
/// 0x350 and 0x358 of wel_lge_cup.cpp).
/// Original: `WelLgeCup::load_extra_data` (0x927330, wel_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    load_welsh_cup_entrants(this, file, 0xad56f4, [0x348, 0x350, 0x358], "cx03_11 welsh league cup entrants loaded")
}

pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_WEL_LGE_CUP_LOAD_EXTRA_DATA);
    compare_welsh_cup_entrants("wel_lge_cup_load_extra_data", this, file, || o(this, file), || wel_lge_cup_load_extra_data(this, file))
}

/// The body of the Welsh league cups' `load_extra_data`: the entrant count (`ita_cup_fields::ENTRANT_CLUB_COUNT`, 4 bytes;
/// a short read: the box of line `rows[0]` and 0), the entrant list (`exe::crt_malloc` of 4 per entrant into
/// `ita_cup_fields::ENTRANT_CLUBS`; out of memory, the box of `rows[1]` and 0) read whole (a short read: the box of
/// `rows[2]` and 0), then each club index back into a club pointer (`club_at`; −1, a free slot, becomes null). Returns
/// 1.
pub(crate) unsafe fn load_welsh_cup_entrants(this: usize, file: usize, source: usize, rows: [u32; 3], branch: &'static str) -> u32 {
    let cup = ItaCupFields(this);
    if exe::crt_fread(this + ita_cup_fields::ENTRANT_CLUB_COUNT, 4, 1, file) != 1 {
        show_error_at(source, rows[0]);
        return 0;
    }
    let list = exe::crt_malloc(cup.entrant_club_count() as u32 * 4);
    cup.set_entrant_clubs(list);
    if list == 0 {
        show_error_at(source, rows[1]);
        return 0;
    }
    if exe::crt_fread(list, 4, cup.entrant_club_count() as u32, file) != cup.entrant_club_count() as u32 {
        show_error_at(source, rows[2]);
        return 0;
    }
    covered!(branch);
    let mut k = 0i32;
    while k < cup.entrant_club_count() as i32 {
        let slot = cup.entrant_clubs() + k as usize * 4;
        let index = read_i32(slot);
        write_usize(slot, if index == -1 { 0 } else { club_at(index as isize).addr() });
        k += 1;
    }
    1
}
