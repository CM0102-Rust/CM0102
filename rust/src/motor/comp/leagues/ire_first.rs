use super::*;

/// Rule: the League of Ireland First Division plays on the Premier Division's dates (36 matchdays, or 34 when the
/// clubs meet three times); then the promotion play-off against the Premier Division's play-off club, two legs from 27
/// April.
///
/// Stage −1: `irish_league_schedule` with the boxes at lines 0xb5 and 0x15e of ire_first.cpp. Stage 0: `out` (when
/// given) = 0, `rounds` = 1, `extra` = 0, the table (`exe::crt_malloc` of 0x68; out of memory, the box at line 0x171 and
/// 0) with `IRISH_PLAY_OFF` (`eng_cc_cup_write_cup_round`; the original writes the same fields in another order).
/// Other stages: 0.
/// Original: `IreFirst::create_schedule` (0x634f40, ire_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ire_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let this = League(this);
    match stage as u8 {
        0xff => irish_league_schedule(this, rounds, extra, out, 0x9e9ed0, (0xb5, 0x15e), "mwa_27 irish first schedule"),
        0 => {
            if !out.is_null() {
                *out = 0;
            }
            *rounds = 1;
            *extra = 0;
            let t = exe::crt_malloc(0x68);
            if t == 0 {
                show_error_at(0x9e9ed0, 0x171);
                return 0;
            }
            covered!("mwa_27 irish play-off round");
            eng_cc_cup_write_cup_round(t, 0, &IRISH_PLAY_OFF, this.year() as i32 as u32);
            t
        }
        _ => 0,
    }
}

/// Comparer for `ire_first_create_schedule` (side by side: `compare_c1_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_ire_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_IRE_FIRST_CREATE_SCHEDULE);
    compare_c1_schedule("ire_first_create_schedule", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, l| ire_first_create_schedule(this, stage, r, e, l))
}

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

pub(crate) unsafe extern "thiscall" fn compare_ire_first_start_new_season(this: usize) -> u32 {
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

pub(crate) unsafe extern "thiscall" fn compare_ire_first_advance_stage(this: usize) {
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

pub(crate) unsafe extern "thiscall" fn compare_ire_first_create_promotion_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_IRE_FIRST_CREATE_PROMOTION_PLAYOFF_STAGE);
    compare_irish_promotion_play_off("ire_first_create_promotion_playoff_stage", League(this), KEY_CLUB_COMP_IRISH_PREMIER_DIVISION, || o(this))
}

/// Rule: the Irish First Division at career creation: the year and the database competition, the Irish discipline rules
/// and, with nothing saved, the league and one extra stage (the promotion play-off): the rules, the table, the
/// fixtures, the order, the statistics and the predicted positions.
///
/// `construct_irish_league` with `IRISH_FIRST_CONSTRUCTOR` (checked instruction by instruction against
/// EngFirst::EngFirst 0x571680: only the vtable, the discipline rule, the two routines, the file and the last line).
/// Original: `IreFirst::IreFirst` (0x634c40, ire_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ire_first(this: usize, year: u32, record: usize) -> usize {
    construct_irish_league(League(this), year, record, &IRISH_FIRST_CONSTRUCTOR, "mwa_12 irish first new") // mwa12: the Irish First
}

pub(crate) unsafe extern "thiscall" fn compare_construct_ire_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_IRE_FIRST);
    compare_irish_league_constructor("construct_ire_first", League(this), year, record, &IRISH_FIRST_CONSTRUCTOR, || o(this, year, record))
}

/// Rule: the Irish First Division: 3 points for a win and 1 for a draw, ties split by goal difference and goals for; the
/// champion goes up to the Premier Division and the runner-up plays the promotion play-off; no relegation (no division
/// below); 5 substitutes on the bench and 3 substitutions.
///
/// `set_up_irish_league_rules` with `IRISH_FIRST_RULES`. Returns 1.
/// Original: `IreFirst::set_up_rules` (0x635aa0, ire_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_first_set_up_rules(this: usize) -> u32 {
    covered!("mwa_12 irish first rules");
    set_up_irish_league_rules(League(this), &IRISH_FIRST_RULES)
}

pub(crate) unsafe extern "thiscall" fn compare_ire_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_FIRST_SET_UP_RULES);
    compare_irish_league_rules("ire_first_set_up_rules", this, &IRISH_FIRST_RULES, || o(this))
}

/// Rule: the Irish First Division table takes the division's clubs in database order; only nine to twelve are
/// accepted: nine or ten meet four times, eleven or twelve three times; no prize fund.
/// Original: `IreFirst::create_table` (0x635910, ire_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_first_create_table(this: usize) -> u32 {
    create_irish_first_table(League(this))
}

pub(crate) unsafe extern "thiscall" fn compare_ire_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_FIRST_CREATE_TABLE);
    compare_irish_counted_table("ire_first_create_table", League(this), KEY_CLUB_COMP_IRISH_FIRST_DIVISION, irish_first_meetings, None, || o(this))
}

/// The league schedule of the League of Ireland's divisions (the virtual `comp_vtable::CREATE_SCHEDULE` at stage −1;
/// IrePrm and IreFirst write the same rounds, read instruction by instruction with the emulator of the 36 and 34 round
/// paths and both Christmas cases).
///
/// n = 36 rounds unless `league::LEGS` is 3 (then 34); `out` (when given) = 1, `rounds` = n, `extra` = 0; the table
/// (`exe::crt_malloc` of n × 0x41; out of memory, the box of `rows.0` and 0). The rounds, each with match slot 0 empty
/// (`fra_third_write_league_rounds`): `IRISH_LEAGUE_OPENING`; with 36, `IRISH_LEAGUE_EXTRA_AUGUST`;
/// `IRISH_LEAGUE_AUTUMN`; the Christmas round: Date(25 December of the league's year, no weekday) against Date(26
/// December moved to weekday 6) (Date::Date `exe::date_date_549f70`, thiscall, ret 0x10; the two words compared), and
/// if they differ the same with 24 December: equal in either, `IRISH_LEAGUE_CHRISTMAS_MIDWEEK`, otherwise
/// `IRISH_LEAGUE_CHRISTMAS`; `IRISH_LEAGUE_JANUARY`; with 36, `IRISH_LEAGUE_EXTRA_FEBRUARY`; `IRISH_LEAGUE_SPRING`.
/// When the rounds written are not n (never: the original's own check), the box of `rows.1`. Returns the table.
pub(crate) unsafe fn irish_league_schedule(this: League, rounds: *mut i16, extra: *mut i16, out: *mut i32, file: usize, rows: (u32, u32), branch: &'static str) -> usize {
    let n: i16 = if this.legs() != 3 { 0x24 } else { 0x22 };
    if !out.is_null() {
        *out = 1;
    }
    *rounds = n;
    *extra = 0;
    let t = exe::crt_malloc((n as i32 * 0x41) as u32);
    if t == 0 {
        show_error_at(file, rows.0);
        return 0;
    }
    covered!(branch);
    let year = this.year() as u16 as u32;
    let mut r = 0i32;
    let mut put = |table: &[FraThirdLeagueRound]| {
        fra_third_write_league_rounds(t, year, r, table);
        r += table.len() as i32;
    };
    put(&IRISH_LEAGUE_OPENING);
    if n == 0x24 {
        put(&IRISH_LEAGUE_EXTRA_AUGUST);
    }
    put(&IRISH_LEAGUE_AUTUMN);
    let same = |day: u32| {
        let (mut a, mut b) = ([0u16; 4], [0u16; 4]);
        exe::date_date_549f70(&mut a, day, 0xb, year, 0xffff_ffff);
        exe::date_date_549f70(&mut b, 0x1a, 0xb, year, 6);
        a[1] == b[1] && a[0] == b[0]
    };
    let midweek = same(0x19) || same(0x18); // mwa27: the Christmas case
    put(if midweek { &IRISH_LEAGUE_CHRISTMAS_MIDWEEK } else { &IRISH_LEAGUE_CHRISTMAS });
    put(&IRISH_LEAGUE_JANUARY);
    if n == 0x24 {
        put(&IRISH_LEAGUE_EXTRA_FEBRUARY);
    }
    put(&IRISH_LEAGUE_SPRING);
    if r as i16 != n {
        show_error_at(file, rows.1);
    }
    t
}

/// The body of the Irish leagues' constructors (thiscall, ret 8: this, year, the competition record).
///
/// League::League (`exe::new_league`, thiscall, ret 0); `league::COMPETITION` = the record; the vtable; `league::YEAR`
/// (word); `league::DISCIPLINE_RULE`; in the Premier Division, the ten 5-byte entries from +0xee get −1 in their first
/// dword (its own state [inference: the last seasons' places]). If the saved state loads (League::load_from_temp_file
/// `exe::league_load_from_temp_file`(this, 1), thiscall, ret 4; ≠ 0) it returns this. Otherwise `league::STAGE_INDEX` =
/// `stage_index::MAIN`, `league::CURRENT_STAGE` = −1, the stage count and array; then, each with its own error box
/// (returning this) when it returns 0: the rules, the table, the fixtures (League::create_fixtures
/// `exe::league_create_fixtures`), the order (League::sort_table `exe::league_sort_table`(this, 0)); the statistics
/// (operator new `exe::crt_operator_new` of 0x5ce and Comp_Stats::Comp_Stats `exe::new_comp_stats`) at
/// `league::COMP_STATS` (without them, the box); finally the predicted positions (League::set_predicted_positions
/// `exe::league_set_predicted_positions`). Returns this.
pub(crate) unsafe fn construct_irish_league(this: League, year: u32, record: usize, k: &IrishLeagueConstructor, branch: &'static str) -> usize {
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), k.vtable);
    this.set_year(year as u16 as i16);
    this.set_discipline_rule(k.discipline);
    if k.reset_ten_slots {
        for s in 0..10usize {
            write_i32(this.addr() + 0xee + s * 5, -1);
        }
    }
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!(branch);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    if k.play_off_stage {
        this.set_stage_count(1);
        this.set_stages(exe::crt_malloc(4));
    } else {
        this.set_stage_count(0);
        this.set_stages(0);
    }
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(k.rules);
    if rules(this.addr()) == 0 {
        show_error_at(k.file, k.rows[0]);
        return this.addr();
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(k.table);
    if table(this.addr()) == 0 {
        show_error_at(k.file, k.rows[1]);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(k.file, k.rows[2]);
        return this.addr();
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(k.file, k.rows[3]);
        return this.addr();
    }
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(k.file, k.rows[4]);
        return this.addr();
    }
    exe::league_set_predicted_positions(this);
    this.addr()
}

/// The comparer of the Irish leagues' constructors: only the original runs (it allocates, draws the fixtures and reads
/// the save file); the plan: the return (this), the vtable, the year, the record, the discipline rule and, on a new
/// league (`stage_index::MAIN`), the current stage −1, the stage count, the stage array (none without stages) and the
/// statistics.
pub(crate) unsafe fn compare_irish_league_constructor(name: &str, this: League, year: u32, record: usize, k: &IrishLeagueConstructor, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == k.vtable && built_year_ok(this.year() as u16, year as u16)
        && this.competition().addr() == record && this.discipline_rule() == k.discipline;
    if ok && this.stage_index() == stage_index::MAIN {
        ok = (this.current_stage() as i32) == -1 && (this.stage_count() as u32) == k.play_off_stage as u32
            && (this.stages() != 0) == k.play_off_stage && this.comp_stats() != 0;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}, {}, {:#x}) returned {:#x}, vtable {:#x}, year {}, stages {} {:#x}, statistics {:#x}", name, this.addr(), year as u16, record, r,
        read_usize(this.addr()), this.year(), (this.stage_count() as u32), this.stages(), this.comp_stats()));
    r
}

/// The body of the Irish leagues' set_up_rules (thiscall, ret 0): the fields (`write_irish_league_rules`) and the round
/// table of the main stage (the league's own virtual `comp_vtable::CREATE_SCHEDULE`(−1, &`league::ROUND_COUNT`,
/// &`league::STAGE_ID`, null), thiscall, ret 0x10) into `league::ROUND_DATES`. Returns 1.
pub(crate) unsafe fn set_up_irish_league_rules(this: League, k: &IrishLeagueRules) -> u32 {
    write_irish_league_rules(this, k);
    let schedule: unsafe extern "thiscall" fn(usize, u32, usize, usize, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = schedule(this.addr(), 0xffff_ffff, this.addr() + league::ROUND_COUNT, this.addr() + league::STAGE_ID, 0);
    this.set_round_dates(t);
    1
}

/// The comparer of the Irish leagues' set_up_rules: side by side over the league's fields outside the round table
/// (`league_regions_without_schedule`); the copy writes only the fields (the original's round table stays).
pub(crate) unsafe fn compare_irish_league_rules(name: &str, this: usize, k: &IrishLeagueRules, original: impl FnOnce() -> u32) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&reg, original, || {
        write_irish_league_rules(League(this), k);
        1
    });
    SCOREBOARD.record(equal && r == n, || format!("{}({:#x}) original={} rust={} {:?} {:?}", name, this, r, n, mo, mr));
    r
}

/// The comparer of the Irish tables (they allocate the table: only the original runs): the plan is the count of the
/// division's clubs; out of the accepted counts the routine must return 0; otherwise the rows, the return and the
/// prize (`ita_ser_a_compare_league_table`) and the meetings `legs`.
pub(crate) unsafe fn compare_irish_counted_table(name: &str, this: League, key: usize, legs: impl Fn(i16) -> Option<i16>, quota: Option<i32>, original: impl FnOnce() -> u32) -> u32 {
    let n = irish_division_club_count(key);
    match legs(n) {
        None => {
            let r = original();
            SCOREBOARD.record(r == 0, || format!("{}({:#x}) with {} clubs returned {}", name, this.addr(), n, r));
            r
        }
        Some(l) => {
            let r = ita_ser_a_compare_league_table(name, this, n, key, quota, original);
            SCOREBOARD.record(this.legs() == l, || format!("{}({:#x}) with {} clubs: legs {} (plan {})", name, this.addr(), n, this.legs(), l));
            r
        }
    }
}

/// The meetings of an Irish First Division of `n` clubs: nine or ten meet four times, eleven or twelve three times;
/// other counts are refused.
pub(crate) fn irish_first_meetings(n: i16) -> Option<i16> {
    match n {
        9 | 10 => Some(4), // mwa12: nine or ten clubs meet four times
        11 | 12 => Some(3),
        _ => None,
    }
}

/// The body of `ire_first_create_table`.
///
/// `league::CLUB_COUNT` = 0 and `league::GAMES_PER_CLUB` = −1; the division's clubs are counted into
/// `league::CLUB_COUNT`. Only nine to twelve are accepted (otherwise 0), with the meetings of `irish_first_meetings`.
/// The table (`ita_ser_a_fill_league_table`: n × 0x3b bytes; out of memory, the error box at line 0x1b6 of
/// ire_first.cpp and 0) with the clubs in database order; returns whether it filled up (no prize fund).
pub(crate) unsafe fn create_irish_first_table(this: League) -> u32 {
    this.set_club_count(0);
    this.set_games_per_club(-1);
    let n = irish_division_club_count(KEY_CLUB_COMP_IRISH_FIRST_DIVISION);
    this.set_club_count(n);
    let legs = match irish_first_meetings(n) {
        Some(l) => l,
        None => return 0,
    };
    this.set_legs(legs);
    covered!("mwa_12 irish first table");
    (ita_ser_a_fill_league_table(this, n, 0x9e9ed0, 0x1b6, KEY_CLUB_COMP_IRISH_FIRST_DIVISION) == Some(true)) as u32
}

/// The next stage of an Irish First Division (`advance_stage`, the comp virtual): at the last stage
/// (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the error box and nothing else. Otherwise it waits for
/// the Premier Division above (the competition object of the key global `premier`, `comp_by_global`): when its
/// `comp_vtable::CHECK_COMPLETE`(0, 1) (thiscall, ret 8, the low byte) says it is over, the Premier's positions and
/// status are updated (`league_vtable::UPDATE_POSITIONS_AND_STATUS`, thiscall, ret 0), the current stage goes up by one
/// and, on reaching 0, the promotion play-off is built (`play_off`, thiscall, ret 0).
pub(crate) unsafe fn advance_irish_first_stage(this: League, premier: usize, file: usize, row: u32, play_off: usize) {
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(file, row);
    }
    let p = comp_by_global(premier);
    let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(p) + comp_vtable::CHECK_COMPLETE));
    if complete(p, 0, 1) as u8 == 0 {
        return;
    }
    covered!("mwa_13 irish first division waits for the premier");
    bra_nat_first_call_virtual_no_args(p, league_vtable::UPDATE_POSITIONS_AND_STATUS);
    let stage = this.current_stage() as i32 + 1;
    this.set_current_stage(stage as usize);
    if stage == 0 {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(play_off);
        f(this.addr());
    }
}

/// The comparer of the Irish First Divisions' next stage: only the original runs (it may build the play-off); the
/// stage either stays or goes up by one, and never past the last stage.
pub(crate) unsafe fn compare_irish_first_advance_stage(name: &str, this: League, original: impl FnOnce()) {
    let before = this.current_stage() as i32;
    let last = before >= (this.stage_count() as i32).wrapping_sub(1);
    original();
    let after = this.current_stage() as i32;
    SCOREBOARD.record(after == before || (!last && after == before + 1), || format!("{}({:#x}) before={} after={} last={}", name, this.addr(), before, after, last));
}

/// The promotion play-off of an Irish First Division: the First Division's play-off club against the Premier Division's
/// play-off club, in a two-club Cup_Stage in the first stage slot.
///
/// The list of 2 (`exe::crt_malloc` of 8; out of memory, the box `rows[0]` and nothing else); entry 0 = the First
/// Division club (`irish_play_off_clubs`; without one, the box `rows[1]` and return: the original leaves the list
/// allocated), entry 1 = the Premier Division club (without one, return with nothing else, the list left allocated).
/// The round table of stage 0 (the league's virtual `comp_vtable::CREATE_SCHEDULE`(0, &rounds, &extra, null)); the
/// stage (operator new `exe::crt_operator_new` of 0xb2; Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret
/// 0x3c: the league, 2, the list, the rounds, the competition record, the table, the year, 0, 1, the extra, 0x14, 0, 0,
/// 0, 0) into the first stage slot; the list and the table freed; without the stage, the box `rows[2]`; otherwise
/// Cup::set_up_round(stage, 0) (`exe::cup_set_up_round`, thiscall, ret 4).
pub(crate) unsafe fn create_irish_promotion_play_off(this: League, premier: usize, file: usize, rows: [u32; 3], branch: &'static str) {
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(file, rows[0]);
    }
    let (first, second) = irish_play_off_clubs(this, premier);
    match first {
        Some(c) => write_usize(list, c),
        None => return show_error_at(file, rows[1]),
    }
    match second {
        Some(c) => write_usize(list + 4, c),
        None => return,
    }
    covered!(branch);
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 0, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    // simplification: the original pushes the rounds, the extra and the year with whatever is in the registers' high
    // halves; the constructor only reads the words
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 2, list, rounds as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(this.stages(), stage);
    exe::crt_free(list);
    exe::crt_free(t);
    let stage = read_usize(this.stages());
    if stage == 0 {
        return show_error_at(file, rows[2]);
    }
    exe::cup_set_up_round(stage, 0);
}

/// The comparer of the Irish promotion play-offs: only the original runs (it allocates and draws); the plan, made
/// beforehand: the two clubs (`irish_play_off_clubs`). With both, the first stage slot must hold a stage of two clubs
/// of the league's competition with those two (as a set).
pub(crate) unsafe fn compare_irish_promotion_play_off(name: &str, this: League, premier: usize, original: impl FnOnce()) {
    let plan = irish_play_off_clubs(this, premier);
    original();
    if let (Some(a), Some(b)) = plan {
        let stage = Cup(read_usize(this.stages()));
        let ok = !stage.is_null() && stage.team_count() == 2 && read_usize(stage.addr() + 4) == this.competition().addr() && {
            let (x, y) = (read_usize(stage.team_table()), read_usize(stage.team_table() + 6));
            (x == a && y == b) || (x == b && y == a)
        };
        SCOREBOARD.record(ok, || format!("{}({:#x}) plan {:#x} v {:#x}, stage {:#x}", name, this.addr(), a, b, stage.addr()));
    }
}

/// The fields `set_up_rules` writes before the round table: `writes`, then `league::UPPER_DIVISION_ID` and
/// `league::LOWER_DIVISION_ID` (the id of the key global, or −1).
pub(crate) unsafe fn write_irish_league_rules(this: League, k: &IrishLeagueRules) {
    for &(o, n, x) in k.writes {
        match n {
            1 => write_u8(this.addr() + o, x as u8),
            2 => write_u16(this.addr() + o, x as u16),
            _ => write_u32(this.addr() + o, x),
        }
    }
    this.set_upper_division_id((k.upper.map_or(0xffff_ffff, |g| read_u32(g))) as i32);
    this.set_lower_division_id((k.lower.map_or(0xffff_ffff, |g| read_u32(g))) as usize);
}

/// How many clubs of the division whose id is in the key global `key` (all `G_CLUB_COUNT` clubs, a 16-bit counter as
/// in the original).
pub(crate) unsafe fn irish_division_club_count(key: usize) -> i16 {
    let mut n = 0i16;
    let mut i = 0i16;
    while (i as i32) < read_i32(G_CLUB_COUNT) {
        let d = club_at(i as isize).division();
        if !d.is_null() && read_usize(d.addr()) == read_usize(key) {
            n = n.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    n
}

/// The two clubs of an Irish promotion play-off: the First Division club whose table row has status 1 (the play-off
/// place; the last such row) and the club of the Premier Division row with status 2 (League::get_table_row
/// `exe::league_get_table_row`, thiscall, ret 4; the last such row), or None for each one missing.
pub(crate) unsafe fn irish_play_off_clubs(this: League, premier: usize) -> (Option<usize>, Option<usize>) {
    let mut first = None;
    for k in 0..this.club_count().max(0) as usize {
        let row = this.table() + k * 0x3b;
        if read_u8(row + 0x37) == 1 {
            first = Some(read_usize(row));
        }
    }
    let p = League(comp_by_global(premier));
    let mut second = None;
    let mut k = 0i32;
    while k < p.club_count() as i32 {
        if read_u8(exe::league_get_table_row(p, k) + 0x37) == 2 {
            second = Some(read_usize(exe::league_get_table_row(p, k)));
        }
        k += 1;
    }
    (first, second)
}

/// The League of Ireland's opening matchday: Saturday 15 August [inference: weekday 6, code 1] (both divisions share
/// the dates); (day, month from 0, years after the league's year, weekday, code, the match slots).
pub(crate) const IRISH_LEAGUE_OPENING: [FraThirdLeagueRound; 1] = [(15, 7, 0, 6, 1, &[(-1, -1, -1)])]; // mwa27: the opening matchday

/// The extra midweek matchday of a 36-round season: 17 August (weekday 1, code 2).
pub(crate) const IRISH_LEAGUE_EXTRA_AUGUST: [FraThirdLeagueRound; 1] = [(17, 7, 0, 1, 2, &[(-1, -1, -1)])];

/// The weekly matchdays from 22 August to 19 December (weekday 6, code 1).
pub(crate) const IRISH_LEAGUE_AUTUMN: [FraThirdLeagueRound; 17] = [
    (22, 7, 0, 6, 1, &[(-1, -1, -1)]), // 22 aug
    (5, 8, 0, 6, 1, &[(-1, -1, -1)]), // 5 sep
    (12, 8, 0, 6, 1, &[(-1, -1, -1)]), // 12 sep
    (19, 8, 0, 6, 1, &[(-1, -1, -1)]), // 19 sep
    (26, 8, 0, 6, 1, &[(-1, -1, -1)]), // 26 sep
    (3, 9, 0, 6, 1, &[(-1, -1, -1)]), // 3 oct
    (10, 9, 0, 6, 1, &[(-1, -1, -1)]), // 10 oct
    (17, 9, 0, 6, 1, &[(-1, -1, -1)]), // 17 oct
    (24, 9, 0, 6, 1, &[(-1, -1, -1)]), // 24 oct
    (31, 9, 0, 6, 1, &[(-1, -1, -1)]), // 31 oct
    (7, 10, 0, 6, 1, &[(-1, -1, -1)]), // 7 nov
    (14, 10, 0, 6, 1, &[(-1, -1, -1)]), // 14 nov
    (21, 10, 0, 6, 1, &[(-1, -1, -1)]), // 21 nov
    (28, 10, 0, 6, 1, &[(-1, -1, -1)]), // 28 nov
    (5, 11, 0, 6, 1, &[(-1, -1, -1)]), // 5 dec
    (12, 11, 0, 6, 1, &[(-1, -1, -1)]), // 12 dec
    (19, 11, 0, 6, 1, &[(-1, -1, -1)]), // 19 dec
];

/// The Christmas matchday when 25 or 24 December is the weekday-6 day the 26 December round would move to: 22 December
/// midweek (weekday 2, code 2).
pub(crate) const IRISH_LEAGUE_CHRISTMAS_MIDWEEK: [FraThirdLeagueRound; 1] = [(22, 11, 0, 2, 2, &[(-1, -1, -1)])];

/// The Christmas matchday otherwise: 26 December (weekday 6, code 1).
pub(crate) const IRISH_LEAGUE_CHRISTMAS: [FraThirdLeagueRound; 1] = [(26, 11, 0, 6, 1, &[(-1, -1, -1)])];

/// The January matchdays of the next year: 3, 16, 23 and 30 (weekday 6, code 1).
pub(crate) const IRISH_LEAGUE_JANUARY: [FraThirdLeagueRound; 4] = [
    (3, 0, 1, 6, 1, &[(-1, -1, -1)]),
    (16, 0, 1, 6, 1, &[(-1, -1, -1)]),
    (23, 0, 1, 6, 1, &[(-1, -1, -1)]),
    (30, 0, 1, 6, 1, &[(-1, -1, -1)]),
];

/// The extra midweek matchday of a 36-round season in the next year: 8 February (weekday 1, code 2).
pub(crate) const IRISH_LEAGUE_EXTRA_FEBRUARY: [FraThirdLeagueRound; 1] = [(8, 1, 1, 1, 2, &[(-1, -1, -1)])];

/// The last eleven matchdays, from 13 February to 23 April of the next year (weekday 6, code 1; 7 March midweek).
pub(crate) const IRISH_LEAGUE_SPRING: [FraThirdLeagueRound; 11] = [
    (13, 1, 1, 6, 1, &[(-1, -1, -1)]), // 13 feb (+1)
    (20, 1, 1, 6, 1, &[(-1, -1, -1)]), // 20 feb (+1)
    (27, 1, 1, 6, 1, &[(-1, -1, -1)]), // 27 feb (+1)
    (7, 2, 1, 1, 2, &[(-1, -1, -1)]), // 7 mar (+1)
    (12, 2, 1, 6, 1, &[(-1, -1, -1)]), // 12 mar (+1)
    (19, 2, 1, 6, 1, &[(-1, -1, -1)]), // 19 mar (+1)
    (26, 2, 1, 6, 1, &[(-1, -1, -1)]), // 26 mar (+1)
    (2, 3, 1, 6, 1, &[(-1, -1, -1)]), // 2 apr (+1)
    (9, 3, 1, 6, 1, &[(-1, -1, -1)]), // 9 apr (+1)
    (16, 3, 1, 6, 1, &[(-1, -1, -1)]), // 16 apr (+1)
    (23, 3, 1, 6, 1, &[(-1, -1, -1)]), // 23 apr (+1)
];

/// The Irish promotion play-off: one round of two clubs over two legs two days apart [the gap byte the original
/// writes], drawn on 24 April and first played on 27 April of the next year (weekday 3, code 1); code 0xa0, tie rule 4
/// and two-leg rule 7; category 5; no prize.
pub(crate) const IRISH_PLAY_OFF: EngCcCupCupRound = cql10r((0x18, 3, 1, 0), (0x1b, 3, 1, 3, 1, 0), 0xa0, (0, 4, 7), 5, (2, 1, 2, 0), (0, 2, 2), (0, 0, 0)); // mwa27: the play-off on 27 April

/// The Irish First Division's constructor: the vtable 0x96bccc, discipline rule `discipline_rule::IRELAND`, one extra
/// stage (the promotion play-off), IreFirst::set_up_rules (0x635aa0) and IreFirst::create_table (0x635910), the lines
/// 0x49, 0x50, 0x57, 0x60 and 0x67 of ire_first.cpp.
pub(crate) const IRISH_FIRST_CONSTRUCTOR: IrishLeagueConstructor = IrishLeagueConstructor { vtable: 0x96bccc, discipline: discipline_rule::IRELAND,
    play_off_stage: true, reset_ten_slots: false, rules: 0x635aa0, table: 0x635910, file: 0x9e9ed0, rows: [0x49, 0x50, 0x57, 0x60, 0x67] };

/// The Irish First Division's rules: four meetings (the table sets 3 with eleven or twelve clubs), 3 points for a win
/// and 1 for a draw, match category 2, club sides, the tie-breaks 1 and 2, one promoted directly and one to the
/// promotion play-off, no relegation, the squad flags 2, 5 substitutes on the bench and 3 substitutions; the Premier
/// Division above, none below.
pub(crate) const IRISH_FIRST_RULES: IrishLeagueRules = IrishLeagueRules {
    writes: &[(league::LEGS, 2, 4), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::CLUB_SIDES, 1, 1),
        (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 0),
        (league::PROMOTED_COUNT, 1, 1), (league::PROMOTION_PLAYOFF_COUNT, 1, 1), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 0), // mwa12: one up, one to the play-off
        (league::SQUAD_FLAGS, 1, 2), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SUBSTITUTES_NAMED, 1, 5)],
    upper: Some(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION),
    lower: None,
};
