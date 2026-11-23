use super::*;

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

/// rule: the Welsh Premier Cup's own table holds the first group of four: pool entries 0, 2, 4 and 6, taken out of the
/// pool; then the group draw news; out of memory, the box of line 0x181 and 0 (`welsh_cup_group_table`, mwb_18)
/// Original: `WelPrmCup::create_group_table` (0x928060, wel_prm_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_create_group_table(this: usize) -> u32 {
    welsh_cup_group_table(League(this), 4, 2, 0xad572c, 0x181, 0, "mwb_20 welsh premier cup table") // mwb20: the Premier Cup first group
}

/// Comparer for `wel_prm_cup_create_group_table` (the plan, original only).
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_create_group_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WEL_PRM_CUP_CREATE_GROUP_TABLE);
    compare_welsh_cup_group_table("wel_prm_cup_create_group_table", League(this), 4, 2, || o(this))
}

/// rule: the Welsh Premier Cup groups play 2 rounds, 3 points for a win and 1 for a draw, category 4; the top 2 of each
/// group go through; tie-breaks codes 1, 2 and 4 [inference: goal difference, goals for, head-to-head]; a prize fund of
/// 100,000; builds the round schedule of the main stage (virtual +0x3c); returns 1
/// Original: `WelPrmCup::set_up_rules` (0x928160, wel_prm_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_set_up_rules(this: usize) -> u32 {
    covered!("mwb_20 welsh premier cup rules");
    eng_prm_apply_league_rules(League(this), &welsh_premier_cup_rules())
}

/// Comparer for `wel_prm_cup_set_up_rules` (side by side on the rule bytes, the copy without the round table).
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WEL_PRM_CUP_SET_UP_RULES);
    eng_prm_compare_league_rules("wel_prm_cup_set_up_rules", League(this), welsh_premier_cup_rules(), || o(this), || wel_prm_cup_set_up_rules(this))
}

/// rule: the Welsh Premier Cup moves from the first group (its own table) to the second (stage 0) and, on reaching
/// stage 1, builds the knockout stage of the eight (WelPrmCup::create_knockout_stage 0x928650); past the last stage,
/// the box of line 0x220 (`welsh_cup_advance_stage`, mwb_18)
/// Original: `WelPrmCup::advance_stage` (0x9283c0, wel_prm_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_advance_stage(this: usize) {
    welsh_cup_advance_stage(League(this), 0xad572c, 0x220, 1, 0x928650)
}

/// Comparer for `wel_prm_cup_advance_stage`: only the original runs (the knockout builds a stage); the current stage
/// stays at the last stage, otherwise goes up by one.
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WEL_PRM_CUP_ADVANCE_STAGE);
    compare_state_league_advance_stage("wel_prm_cup_advance_stage", League(this), || o(this))
}

/// rule: in the two Welsh Premier Cup groups, a club going through gets the qualification record (weight 40) and one
/// going out the failed-to-qualify record (weight 40); in the knockout (stage 1) a club going through gets the
/// qualification by round and one going out the knocked-out record (weight 50 to 100 over 3 rounds), and the winner
/// and the runner-up their final records (`welsh_cup_staff_records`, mwb_19)
/// Original: `WelPrmCup::update_staff_records_for_status` (0x928af0, wel_prm_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, a6: u32) -> u32 {
    covered!("mwb_21 welsh premier cup staff records");
    welsh_cup_staff_records(League(this), club, outcome, stage, game, a6, 1, 3) // mwb21: the Premier Cup knockout over 3 rounds
}

/// Comparer for `wel_prm_cup_update_staff_records_for_status` (original only, the return 0).
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_WEL_PRM_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, a6);
    SCOREBOARD.record(r == 0, || format!("wel_prm_cup_update_staff_records_for_status({:#x}) returned {}", this, r));
    r
}

/// rule: the second Welsh Premier Cup group (stage 0): four clubs, pool entries 1, 3, 5 and 7, in a league stage of
/// their own with tie-break list (2, 0, 0, 0); then the group draw news; out of memory the box of line 0x247, without
/// the stage that of 0x25d (and 0) (`welsh_cup_group_stage`, mwb_19)
/// Original: `WelPrmCup::create_group_stage` (0x928450, wel_prm_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_create_group_stage(this: usize, g: u32) -> u32 {
    welsh_cup_group_stage(League(this), g, 4, 2, 2, 0xad572c, (0x247, 0x25d), "mwb_21 welsh premier cup group")
}

/// Comparer for `wel_prm_cup_create_group_stage` (original only, the plan).
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_WEL_PRM_CUP_CREATE_GROUP_STAGE);
    compare_welsh_cup_group_stage("wel_prm_cup_create_group_stage", League(this), g, 4, 2, || o(this, g))
}

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

/// The own table of a Welsh group cup (the league object's table holds one group): `league::CLUB_COUNT` = `n`, the
/// table (`exe::crt_malloc` of n × 0x3b; out of memory, the box of `row` and 0), row k gets pool entry k × `stride`
/// (`exe::league_init_table_row(this, k, club, 0, 0)`) and that pool entry is zeroed; then the group draw news
/// (League::send_group_draw_news 0x684230, thiscall, ret 0); returns 1 (`rotate` is 0 in the original: row k gets the
/// k-th club; it exists only for the mutations).
pub(crate) unsafe fn welsh_cup_group_table(this: League, n: i16, stride: usize, file: usize, row: u32, rotate: u8, ramo: &'static str) -> u32 {
    this.set_club_count(n);
    let t = exe::crt_malloc((n as i32 as u32).wrapping_mul(0x3b));
    this.set_table(t);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    covered!(ramo);
    let mut k: u8 = 0;
    while (k as i8 as i16) < this.club_count() {
        let e = read_usize(this.addr() + WELSH_CUP_POOL) + k as i8 as usize * stride * 4;
        // simplification: the original pushes the row as a stack dword with stale high bytes; 0x687430 reads the byte
        exe::league_init_table_row(this, ((k as u32 + rotate as u32) % n as u32) as u8 as u32, read_usize(e), 0, 0);
        write_usize(e, 0);
        k = k.wrapping_add(1);
    }
    exe::league_send_group_draw_news(this);
    1
}

/// The next stage of a Welsh group cup: at the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1,
/// signed) the error box and nothing else; otherwise the current stage goes up by one and, on reaching `at`, the
/// knockout stage is built (`knockout`, thiscall, ret 0).
pub(crate) unsafe fn welsh_cup_advance_stage(this: League, file: usize, row: u32, at: i32, knockout: usize) {
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(file, row);
    }
    let stage = stage + 1;
    this.set_current_stage(stage as usize);
    if stage == at {
        covered!("mwb_18 welsh cup knockout stage");
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(knockout);
        f(this.addr());
    }
}

/// Builds group `g` of a Welsh group cup as a League_Stage, in the original routine's order.
///
/// The round table of stage `g` (virtual +0x3c: the round count and the stage label into locals, no league flag); the
/// club list (`exe::crt_malloc` of 4 × `n`; null: the box of the first line and 0) with pool entries g + 1 + k × `stride`
/// (k = 0..n), each zeroed in the pool; a League_Stage (operator new of 0xee and League_Stage::League_Stage 0x68b100:
/// this cup as the parent, `n` clubs, the list, 2, the record, the round table and its count, the win, draw and
/// category bytes, the tie-breaks (+0xc5), a local tie-break list (`first_tiebreak`, 0, 0, 0), the year, `g`, the stage
/// label, the importance (+0x51), 1, 0, the league flags (+0xd9), −1, 0, 2) into stage slot `g` (0 without memory); the
/// list and the round table freed; no stage: the box of the second line and 0; otherwise the group draw news of the
/// stage (League::send_group_draw_news), the current stage = `g` and 1.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn welsh_cup_group_stage(this: League, g: u32, n: u32, stride: usize, first_tiebreak: u8, file: usize, rows: (u32, u32), ramo: &'static str) -> u32 {
    let t = this;
    let tiebreaks: [u8; 4] = [first_tiebreak, 0, 0, 0];
    let (mut rounds, mut label) = (0i16, 0i16);
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(t.addr()) + comp_vtable::CREATE_SCHEDULE));
    let table = schedule(t.addr(), g, &mut rounds, &mut label, 0);
    let clubs = exe::crt_malloc(4 * n);
    if clubs == 0 {
        show_error_at(file, rows.0);
        return 0;
    }
    covered!(ramo);
    for k in 0..n as usize {
        let e = read_usize(t.addr() + WELSH_CUP_POOL) + (g as usize + 1 + k * stride) * 4; // mwb19: the group clubs in pool order
        write_usize(clubs + 4 * k, read_usize(e));
        write_usize(e, 0);
    }
    let p = exe::crt_operator_new(0xee);
    let mut s = 0usize;
    if p != 0 {
        type LeagueStageCtor = unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize;
        let ctor: LeagueStageCtor = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        // simplification: the original pushes the round count, the stage label and the flags as stack or register
        // dwords with stale high bytes; League_Stage::League_Stage reads the words and bytes
        s = ctor(p, t.addr(), n, clubs, 2, t.competition().addr(), table, rounds as u16 as u32, t.win_points() as u32,
            t.draw_points() as u32, t.match_category() as u32, t.addr() + league::TIEBREAKS, tiebreaks.as_ptr() as usize,
            this.year() as u16 as u32, g, label as u16 as u32, t.importance() as u32, 1, 0, (t.league_flags() as u16) as u32, 0xffff_ffff, 0, 2);
    }
    write_usize(this.stages() + 4 * g as usize, s);
    exe::crt_free(clubs);
    exe::crt_free(table);
    let s = read_usize(this.stages() + 4 * g as usize);
    if s == 0 {
        show_error_at(file, rows.1);
        return 0;
    }
    exe::league_send_group_draw_news(s);
    this.set_current_stage(g as usize);
    1
}

/// The Welsh Premier Cup rule writes: the top 2 of each group go through, tie-breaks 1, 2, 4 [inference: goal
/// difference, goals for, head-to-head].
pub(crate) fn welsh_premier_cup_rules() -> Vec<(usize, u8, u32)> {
    welsh_cup_rules(2, [1, 2, 4]) // mwb20: the Premier Cup group rules
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

/// The Welsh Premier Cup's constructor: the vtable 0x9710e8, no discipline rule (`discipline_rule::NONE`), two stages
/// (the second group and the knockout), WelPrmCup's build_entrant_list (0x928860), set_predicted_positions (0x928e00),
/// set_up_rules (0x928160), create_group_table (0x928060) and create_group_stage (0x928450), wel_prm_cup.cpp.
pub(crate) const WELSH_PREMIER_CUP_CONSTRUCTOR: IrishLeagueCupConstructor = IrishLeagueCupConstructor { vtable: 0x9710e8, discipline: discipline_rule::NONE, stages: 2, groups: 1,
    entrants: 0x928860, predicted: 0x928e00, rules: 0x928160, first_group: 0x928060, group_stage: 0x928450, file: 0xad572c };

/// The four clubs that enter the Welsh Premier Cup from the English league system, in the order the original adds
/// them: Cardiff City, Swansea City, Wrexham and Merthyr Tydfil (their key ids, index.cpp).
pub(crate) const WELSH_PREMIER_CUP_GUESTS: [usize; 4] = [0x9d0434, 0x9d0430, 0x9d043c, 0x9d0438];

/// A group matchday with its year offset: (day, month from 0, years after the cup's year, weekday, code).
pub(crate) type WelshGroupRound = (u32, u32, u32, u32, u32);
