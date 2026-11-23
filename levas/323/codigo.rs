// ---------- mwb_22: the Welsh League Cup (wel_lge_cup.cpp): the entrants, the predicted positions and the season turnover ----------

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

// ---------- mwb_23: the two Welsh group cups (wel_lge_cup.cpp, wel_prm_cup.cpp): the schedules and the Premier Cup's season turnover ----------

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

/// A group matchday with its year offset: (day, month from 0, years after the cup's year, weekday, code).
pub(crate) type WelshGroupRound = (u32, u32, u32, u32, u32);

/// `create_schedule` with group matchdays that may fall in the next year (the Welsh Premier Cup's last one): the same
/// frame, the group rounds written by `exe::league_round_set_date`(table, r, day, month, years, weekday, code, year, 0)
/// with their own year offset.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn create_schedule_with_years(this: League, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32, limit: i8, id: i16, groups: &[WelshGroupRound],
    knockout: &[EngCcCupCupRound], file: usize, rows: (u32, u32), branches: (&'static str, &'static str)) -> usize {
    let f = stage as u8 as i8;
    if f < limit {
        if !league.is_null() {
            *league = 1;
        }
        *rounds = groups.len() as i16;
        *extra = (f as i16).wrapping_add(id);
        let t = exe::crt_malloc(groups.len() as u32 * 0x41);
        if t == 0 {
            show_error_at(file, rows.0);
            return 0;
        }
        covered!(branches.0);
        // the year goes with garbage in the high bytes in the original; 0x68a160 only uses the low 16 bits
        let year = (this.year() as u16) as u32;
        for (r, &(day, month, years, weekday, code)) in groups.iter().enumerate() {
            exe::league_round_set_date(t, r as i32, day as i32, month as i32, years as i32, weekday as i32, code as i32, year, 0);
            exe::league_round_set_match_slot(t, r as i32, 0, -1, -1, -1, 0);
        }
        return t;
    }
    if f != limit {
        return 0;
    }
    if !league.is_null() {
        *league = 0;
    }
    *rounds = knockout.len() as i16;
    *extra = 0;
    let t = exe::crt_malloc(knockout.len() as u32 * 0x68);
    if t == 0 {
        show_error_at(file, rows.1);
        return 0;
    }
    covered!(branches.1);
    let year = this.year() as i32 as u32;
    for (i, r) in knockout.iter().enumerate() {
        eng_cc_cup_write_cup_round(t, i as u32, r, year);
    }
    t
}

/// The Welsh Premier Cup's six group matchdays (weekday 1, code 2): 5 September, 3 and 10 October, 14 November, 12
/// December and 9 January of the next year.
pub(crate) const WELSH_PREMIER_CUP_GROUP_ROUNDS: [WelshGroupRound; 6] = [(5, 8, 0, 1, 2), (3, 9, 0, 1, 2), (0xa, 9, 0, 1, 2), (0xe, 0xa, 0, 1, 2), (0xc, 0xb, 0, 1, 2), (9, 0, 1, 1, 2)]; // mwb23: the Premier Cup group matchdays

/// The Welsh Premier Cup's knockout, all in the next year: quarter-finals drawn on 10 January and played on 13
/// February (draw rule 8, tie rules 0x203, category 4; 15,000 to each club [inference: the +0x5c prize]); semi-finals
/// over two legs seven days apart, drawn on 14 February and first played on 10 April (two-leg rule 3, category 4;
/// 10,000); the final drawn on 18 April and played on 13 May (weekday 6, code 1; category 6; 100,000 to the winner and
/// 50,000 to the loser).
pub(crate) const WELSH_PREMIER_CUP_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 3] = [
    cql10r((0xa, 0, 1, 2), (0xd, 1, 1, 1, 2, 0), 0x78, (8, 0x203, 0), 4, (8, 4, 8, 0), (0, 1, 0), (0x3a98, 0, 0)),
    cql10r((0xe, 1, 1, 2), (0xa, 3, 1, 1, 2, 0), 0x82, (0, 0, 3), 4, (4, 2, 0, 0), (0, 2, 7), (0x2710, 0, 0)),
    cql10r((0x12, 3, 1, 2), (0xd, 4, 1, 6, 1, 0), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0x186a0, 0xc350)),
];

/// Rule: the Welsh Premier Cup's two groups of four (the cup's own table and stage 0) play six matchdays from 5
/// September to 9 January (a double round robin), each group a league stage (is_league = 1, label 0x420 + stage); the
/// knockout (stage 1) has eight clubs in the next year: quarter-finals on 13 February, two-legged semi-finals from 10
/// April and the final on 13 May; prize money in every round.
///
/// `create_schedule_with_years` with the knockout at stage 1, the label 0x420, `WELSH_PREMIER_CUP_GROUP_ROUNDS`,
/// `WELSH_PREMIER_CUP_KNOCKOUT_ROUNDS` and the boxes at lines 0xbf and 0xea of wel_prm_cup.cpp (the frame of
/// WelLgeCup::create_schedule, read instruction by instruction; the last group matchday has a year offset). Above stage
/// 1: 0.
/// Original: `WelPrmCup::create_schedule` (0x927b30, wel_prm_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_schedule_with_years(League(this), stage, rounds, extra, league, 1, 0x420, &WELSH_PREMIER_CUP_GROUP_ROUNDS, &WELSH_PREMIER_CUP_KNOCKOUT_ROUNDS, 0xad572c, (0xbf, 0xea),
        ("mwb_23 welsh premier cup groups", "mwb_23 welsh premier cup knockout rounds"))
}

/// Comparer for `wel_prm_cup_create_schedule` (side by side: `ita_cup_compare_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_WEL_PRM_CUP_CREATE_SCHEDULE);
    ita_cup_compare_schedule("wel_prm_cup_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| wel_prm_cup_create_schedule(this, stage, r, e, l), 1)
}

/// Rule: at the season turn the Welsh Premier Cup is rebuilt: the table, the round table, the entrants and the stages of
/// the last edition are freed, the statistics archived, the year moves on; then the twelve entrants, the predicted
/// positions, the rules, the first group's table, its fixtures and order and the stage of the second group.
///
/// `start_irish_league_cup_season` with WelPrmCup::build_entrant_list (0x928860), create_group_table (0x928060),
/// create_group_stage (0x928450), one group and the error box lines 0x1f3, 0x1fa, 0x200 and 0x207 of wel_prm_cup.cpp
/// (read instruction by instruction against IreLgeCup::start_new_season: the registers differ; the group loop is a
/// single call, whose return the routine passes on).
/// Original: `WelPrmCup::start_new_season` (0x9281f0, wel_prm_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_start_new_season(this: usize) -> u32 {
    start_irish_league_cup_season(League(this), 0x928860, 0x928060, 0x928450, 1, 0xad572c, [0x1f3, 0x1fa, 0x200, 0x207], 1) // mwb23: the Premier Cup turnover
}

/// Comparer for `wel_prm_cup_start_new_season` (original only: `compare_state_league_new_season`, the year up by one).
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WEL_PRM_CUP_START_NEW_SEASON);
    compare_state_league_new_season("wel_prm_cup_start_new_season", League(this), || o(this))
}

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_WEL_LGE_CUP);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_WEL_PRM_CUP);
    compare_welsh_group_cup_constructor("construct_wel_prm_cup", this, year, record, &WELSH_PREMIER_CUP_CONSTRUCTOR, || o(this, year, record))
}

// ---------- mwb_25: the Welsh Premier Cup (wel_prm_cup.cpp): the entrants and the predicted positions ----------

/// The four clubs that enter the Welsh Premier Cup from the English league system, in the order the original adds
/// them: Cardiff City, Swansea City, Wrexham and Merthyr Tydfil (their key ids, index.cpp).
pub(crate) const WELSH_PREMIER_CUP_GUESTS: [usize; 4] = [0x9d0434, 0x9d0430, 0x9d043c, 0x9d0438];

/// The body of `wel_prm_cup_build_entrant_list`; returns (the routine's return, the entries to compare).
///
/// `ita_cup_fields::ENTRANT_CLUB_COUNT` = 12; the pool (`exe::crt_calloc`(4, 12) into `ita_cup_fields::ENTRANT_CLUBS`;
/// none: the box of line 0x2b3 and 0). The holder of the Welsh Cup (`exe::comp_util_get_latest_placed_club`(the record
/// of [`KEY_CLUB_COMP_WELSH_CUP`], 0), cdecl) takes entry 0 when there is one; the cut-off place is 8 when the holder
/// finished below ninth in its league last season (`club::LAST_POSITION` < 9, a signed byte), otherwise (or with no
/// holder) 7. Then, while fewer than eight are in, three passes over the clubs (`G_CLUB_COUNT`, a signed word index, in
/// table order), the holder left out: the clubs whose `club::LAST_DIVISION` is the League of Wales
/// ([`KEY_CLUB_COMP_WELSH_PREMIER_DIVISION`]) with `club::LAST_POSITION` up to the cut-off, then those placed below it,
/// then the clubs whose `club::DIVISION` is the League of Wales (no check against those already in, as the original).
/// The entries so far are shuffled (`exe::comp_util_shuffle_pointer_array`(pool, n)); the next four are the clubs of
/// `WELSH_PREMIER_CUP_GUESTS` (`club_at`([key])), and entries 8 to 11 are shuffled (whatever n was). Returns 1.
pub(crate) unsafe fn build_welsh_premier_cup_entrants(this: League) -> (i32, i32) {
    let t = this.addr();
    write_i32(t + ita_cup_fields::ENTRANT_CLUB_COUNT, 0xc);
    let v = exe::crt_calloc(4, 0xc);
    write_usize(t + ita_cup_fields::ENTRANT_CLUBS, v);
    if v == 0 {
        show_error_at(0xad572c, 0x2b3);
        return (0, 0);
    }
    let holder = exe::comp_util_get_latest_placed_club(comp_record_of(KEY_CLUB_COMP_WELSH_CUP), 0);
    let cut = if holder != 0 && (Club(holder).last_position() as i8) < 9 { 8i8 } else { 7 }; // mwb25: the cut-off place
    let mut n = 0usize;
    if holder != 0 {
        write_usize(read_usize(t + ita_cup_fields::ENTRANT_CLUBS), holder);
        n = 1;
    }
    let wales = || read_usize(KEY_CLUB_COMP_WELSH_PREMIER_DIVISION);
    let in_league = |d: Competition| !d.is_null() && read_usize(d.addr()) == wales();
    let passes: [&dyn Fn(Club) -> bool; 3] = [
        &|c: Club| in_league(c.last_division()) && (c.last_position() as i8) <= cut,
        &|c: Club| in_league(c.last_division()) && (c.last_position() as i8) > cut,
        &|c: Club| in_league(c.division()),
    ];
    covered!("mwb_25 welsh premier cup entrants");
    for pass in passes.iter() {
        let mut i = 0i16;
        while (i as i32) < read_i32(G_CLUB_COUNT) && n < 8 {
            let c = club_at(i as isize);
            if pass(c) && c.addr() != holder {
                write_usize(read_usize(t + ita_cup_fields::ENTRANT_CLUBS) + 4 * n, c.addr());
                n += 1;
            }
            i = i.wrapping_add(1);
        }
    }
    exe::comp_util_shuffle_pointer_array(read_usize(t + ita_cup_fields::ENTRANT_CLUBS), n as u32);
    for (k, &key) in WELSH_PREMIER_CUP_GUESTS.iter().enumerate() {
        write_usize(read_usize(t + ita_cup_fields::ENTRANT_CLUBS) + 4 * (n + k), club_at(read_usize(key) as i32 as isize).addr()); // mwb25: the four guests
    }
    exe::comp_util_shuffle_pointer_array(read_usize(t + ita_cup_fields::ENTRANT_CLUBS) + 0x20, 4);
    (1, 0xc)
}

/// Rule: the Welsh Premier Cup takes twelve clubs: the Welsh Cup holder, then League of Wales clubs by last season's
/// place (those up to 8th, or up to 7th when the holder itself finished 9th or lower or there is no holder, then the
/// rest, then this season's League of Wales clubs) until there are eight, shuffled; and the four Welsh clubs of the
/// English league (Cardiff City, Swansea City, Wrexham and Merthyr Tydfil), shuffled among themselves.
/// Original: `WelPrmCup::build_entrant_list` (0x928860, wel_prm_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_build_entrant_list(this: usize) -> i32 {
    build_welsh_premier_cup_entrants(League(this)).0
}

/// Comparer for `wel_prm_cup_build_entrant_list` (side by side with the random generator restored:
/// `compare_select_entrants`). The holder comes from `comp_util_get_latest_placed_club` (0x4afd80), which re-sorts the
/// global competition history (`[0xae23d4]`, `[0xae237c]` records of 0x1a bytes) with the CRT's unstable qsort on every
/// call: the copy runs on the history as the original saw it, and the original's history is put back after it (as
/// `compare_ger_lge_cup_collect_entrants`).
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_build_entrant_list(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_WEL_PRM_CUP_BUILD_ENTRANT_LIST);
    let hist = || (read_usize(0xae23d4), read_usize(0xae237c) * 0x1a);
    let snap = |(p, n): (usize, usize)| if p == 0 { Vec::new() } else { core::slice::from_raw_parts(p as *const u8, n).to_vec() };
    let put = |p: usize, v: &[u8]| if p != 0 && !v.is_empty() { core::ptr::copy_nonoverlapping(v.as_ptr(), p as *mut u8, v.len()) };
    let before = core::cell::RefCell::new(Vec::new());
    let after = core::cell::RefCell::new(Vec::new());
    compare_select_entrants("wel_prm_cup_build_entrant_list", this, || {
        *before.borrow_mut() = snap(hist());
        let r = o(this);
        *after.borrow_mut() = snap(hist());
        r
    }, || {
        put(hist().0, &before.borrow());
        let r = build_welsh_premier_cup_entrants(League(this));
        put(hist().0, &after.borrow());
        r
    })
}

/// Rule: the Welsh Premier Cup's predicted positions (at creation and at the turnover): the twelve entrants by
/// reputation; the knockout (stage 1) gives 1 to the winner and 2 to the runner-up, then a shared place to the losers
/// of each earlier round; the rest in bands of 2; the board and press expectation of each club [inference].
///
/// `italian_cup_predicted_positions` with the knockout stage 1, bands of 2 and the boxes at lines 0x3db and 0x3e3 of
/// wel_prm_cup.cpp (read instruction by instruction against ItaCCup::set_predicted_positions: the frame and the
/// registers differ, the same steps; only the file, the lines, the stage and the band change).
/// Original: `WelPrmCup::set_predicted_positions` (0x928e00, wel_prm_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_set_predicted_positions(this: usize) {
    italian_cup_predicted_positions(this, 0xad572c, (0x3db, 0x3e3), 1, 2, "mwb_25 welsh premier cup predicted") // mwb25: the Premier Cup bands of 2
}

/// Comparer for `wel_prm_cup_set_predicted_positions` (`compare_italian_cup_predicted`, side by side).
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WEL_PRM_CUP_SET_PREDICTED_POSITIONS);
    compare_italian_cup_predicted("wel_prm_cup_set_predicted_positions", this, wel_prm_cup_set_predicted_positions, || o(this));
}
