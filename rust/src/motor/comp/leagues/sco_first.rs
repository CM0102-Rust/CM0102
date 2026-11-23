use super::*;

/// The Scottish First Division at career creation: discipline rule 0x18, the 1999 promotion play-off, the rules, the
/// table with its 10 clubs, the fixtures, the order, the statistics and the predicted positions
/// (`construct_british_division`).
/// Original: `ScoFirst::ScoFirst` (0x7edfc0, sco_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_sco_first(this: usize, year: u32, record: usize) -> usize {
    construct_british_division(League(this), year, record, &SCOTTISH_FIRST_DIVISION, "mwb_5 new scottish first")
}

/// Comparer for `construct_sco_first` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_sco_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SCO_FIRST);
    compare_british_division_ctor("construct_sco_first", League(this), year, record, &SCOTTISH_FIRST_DIVISION, || o(this, year, record))
}

/// The First Division table: 10 clubs, those whose division is the Scottish First, in database order; the prize fund of
/// 25,000 per place (League::calc_prize_fund) and no TV money; out of memory, the box of line 0x183
/// Original: `ScoFirst::create_table` (0x7eed30, sco_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_first_create_table(this: usize) -> u32 {
    fill_british_division_table(League(this), 0xa, 0xa7ade4, 0x183, KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION, Some(0x61a8), None, 0, "mwb_5 scottish first table") // mwb5: the First Division rows
}

/// Comparer for `sco_first_create_table` (the plan, original only).
pub(crate) unsafe extern "thiscall" fn compare_sco_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_FIRST_CREATE_TABLE);
    compare_british_division_table("sco_first_create_table", League(this), 0xa, KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION, Some(0x61a8), || o(this))
}

/// rule: the Scottish First Division (10 clubs) plays 4 rounds, 3 points for a win and 1 for a draw; the champion goes
/// up to the Premier and the bottom 2 go down to the Second (in 1999: 2 to the promotion play-off and nobody down);
/// tie-breaks goal difference, goals for and position [inference from codes 1, 2, 3]; 5 substitutes named and 3
/// allowed; builds the round schedule (virtual +0x3c into `league::ROUND_DATES`); returns 1
/// Original: `ScoFirst::set_up_rules` (0x7eee50, sco_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_first_set_up_rules(this: usize) -> u32 {
    covered!("mwb_5 scottish first rules");
    eng_prm_apply_league_rules(League(this), &scottish_first_rules(League(this)))
}

/// Comparer for `sco_first_set_up_rules` (side by side on the rule bytes, the copy without the round table).
pub(crate) unsafe extern "thiscall" fn compare_sco_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_FIRST_SET_UP_RULES);
    eng_prm_compare_league_rules("sco_first_set_up_rules", League(this), scottish_first_rules(League(this)), || o(this), || sco_first_set_up_rules(this))
}

/// rule: the Scottish First Division plays 36 rounds (10 clubs, 4 rounds), Saturdays from 7 August to 6 May of the next
/// year: 19 before the year end (none on 9 October), the year-end round on 26, 27 or 28 December by the weekday and 16
/// after (3 January on its own day); no TV slots. In 1999 the promotion play-off (stage 0): 3 rounds, 27 April, 31 May
/// and 4 June, with stage label 0xa0 and the date argument 0x28. Any other stage: nothing and 0
/// Original: `ScoFirst::create_schedule` (0x7ee2e0, sco_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn sco_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
    match stage as u8 {
        0xff => scottish_league_round_table(this, rounds, extra, league, &SCOTTISH_FIRST_SCHEDULE, "mwb_6 scottish first calendar"),
        0 => {
            if !league.is_null() {
                *league = 1;
            }
            *rounds = 3;
            *extra = 0xa0;
            let t = exe::crt_malloc(0xc3);
            if t == 0 {
                show_error_at(0xa7ade4, 0x15f);
                return 0;
            }
            let year = (this.year() as u16) as u32;
            for (i, &(day, month, years, week, code)) in SCOTTISH_FIRST_PLAYOFF.iter().enumerate() {
                exe::league_round_set_date(t, i as i32, day, month, years, week, code, year, 0x28);
                exe::league_round_set_match_slot(t, i as i32, 0, -1, -1, -1, 0);
            }
            t
        }
        _ => 0,
    }
}

/// Comparer for `sco_first_create_schedule` (side by side, the written rounds).
pub(crate) unsafe extern "thiscall" fn compare_sco_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SCO_FIRST_CREATE_SCHEDULE);
    compare_league_round_table("sco_first_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| sco_first_create_schedule(this, stage, r, e, l))
}

/// rule: at the turnover the Scottish First Division is rebuilt: the season archived in the statistics, the old table,
/// rounds and stages freed, the next year (with the play-off stage only in 1999), the rules, the table, the fixtures and
/// the order; then the predicted positions, the board expectations and the referees renewed
/// (`british_division_new_season`; the error boxes of lines 0x1fe, 0x205, 0x20b and 0x212)
/// Original: `ScoFirst::start_new_season` (0x7eef00, sco_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_first_start_new_season(this: usize) -> u32 {
    british_division_new_season(League(this), &SCOTTISH_FIRST_DIVISION, [0x1fe, 0x205, 0x20b, 0x212], false, 1, "mwb_6 scottish first turnover") // mwb6: the next First Division season
}

/// Comparer for `sco_first_start_new_season`: only the original runs; the year went up by one.
pub(crate) unsafe extern "thiscall" fn compare_sco_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("sco_first_start_new_season", League(this), || o(this))
}

/// rule: outside 1999 the club's progress codes go to the common routine (Comp::set_club_progress 0x48e380, the same
/// this and arguments: thiscall, ret 0x14). In 1999 they go into the club's Comp_Stats entry (current, best, worst):
/// in the league (stage 0xff) a best of 2 becomes 1; in the play-off (stage 0) the Premier club's codes go into the
/// Premier's statistics entry with 1 → 8 and 2, 3 → 10, and a First Division club's own with 1 → 1 and 2, 3 → 3; any
/// other stage as they came (the entry is not checked for null)
/// Original: `ScoFirst::set_club_progress` (0x7ef100, sco_first.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn sco_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let club = Club(club);
    let league = League(this);
    if league.year() != 0x7cf {
        covered!("mwb_6 first division progress");
        return exe::comp_set_club_progress(this, club, stage, current, best, worst); // mwb6: the common progress codes
    }
    covered!("mwb_6 first division progress 1999");
    let mut e = exe::comp_stats_find_club_entry(league.comp_stats(), club);
    let (mut a, mut m, mut p) = (current as u8, best as u8, worst as u8);
    match stage as u8 {
        0xff => {
            if m == 2 {
                m = 1;
            }
        }
        0 => {
            let d = club.division().addr();
            if read_usize(d) == read_usize(KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION) {
                let premier = League(comp_by_global(KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION));
                e = exe::comp_stats_find_club_entry(premier.comp_stats(), club);
                m = scottish_playoff_code(m, 8, 0xa);
                p = scottish_playoff_code(p, 8, 0xa);
                a = scottish_playoff_code(a, 8, 0xa);
            } else {
                m = scottish_playoff_code(m, 1, 3);
                p = scottish_playoff_code(p, 1, 3);
                a = scottish_playoff_code(a, 1, 3);
            }
        }
        _ => {}
    }
    let e = CompStatsClub(e);
    e.set_best_position(m);
    e.set_current_position(a);
    e.set_worst_position(p);
}

/// Comparer for `sco_first_set_club_progress`: side by side on the three codes of the club's statistics entry (only the
/// original without an entry, and in the 1999 play-off, where the Premier's entry is written).
pub(crate) unsafe extern "thiscall" fn compare_sco_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_SCO_FIRST_SET_CLUB_PROGRESS);
    if League(this).year() == 0x7cf && stage as u8 == 0 {
        return o(this, club, stage, current, best, worst);
    }
    compare_club_stage_status("sco_first_set_club_progress", this, club, || o(this, club, stage, current, best, worst), || sco_first_set_club_progress(this, club, stage, current, best, worst))
}

/// The constructor shared by the Scottish Premier, First, Second and Third Divisions and the League of Wales (the same
/// template, checked instruction by instruction: only the vtable, the stadium requirement, the 1999 play-off, the
/// Premier's split compare, the two routines, the source file and the lines change).
///
/// League::League (`exe::new_league`, mov ecx -> thiscall, ret 0); the record (`league::COMPETITION`), the vtable, the
/// year (word) = `year`, the stadium requirement (`league::REQUIRED_CAPACITY`, `league::REQUIRED_SEATS`) and the
/// discipline rule. If the saved state loads (`exe::league_load_from_temp_file(this, 1)`, nonzero), the Premier gets
/// its split compare back (`league::CUSTOM_COMPARE`, when the season is played in four rounds and the year is not 1999)
/// and it returns this.
///
/// Otherwise (a new career): `league::STAGE_INDEX` = 0xff, `league::CURRENT_STAGE` = −1, the stages beyond the league
/// (`british_division_stage_count`) and their vector (`exe::crt_malloc` of 4 per stage, unchecked; 0 without stages);
/// then, each with its own error box (and returning this) when it returns 0: the rules, the table, the fixtures
/// (`exe::league_create_fixtures`) and the order (`exe::league_sort_table(this, 0)`); the statistics (operator new
/// `exe::crt_operator_new(0x5ce)` and `exe::new_comp_stats(new, this)`, under the original's SEH frame) into
/// `league::COMP_STATS` (null: the box); finally the predicted positions (`exe::league_set_predicted_positions`).
pub(crate) unsafe fn construct_british_division(this: League, year: u32, record: usize, e: &BritishDivisionSpec, ramo: &'static str) -> usize {
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.rules);
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.table);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), e.vtable);
    this.set_year(year as u16 as i16);
    if let Some((capacity, seats)) = e.stadium {
        this.set_required_capacity(capacity);
        this.set_required_seats(seats);
    }
    this.set_discipline_rule(e.discipline);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        if let Some(f) = e.split_compare {
            if this.legs() == 4 && this.year() != 0x7cf {
                this.set_custom_compare(f);
            }
        }
        return this.addr();
    }
    covered!(ramo);
    this.set_stage_index(0xff);
    this.set_current_stage(0xffff_ffff);
    let n = british_division_stage_count(this, e);
    this.set_stage_count(n as usize);
    if n as i32 > 0 {
        let v = exe::crt_malloc(n * 4);
        this.set_stages(v);
    } else {
        this.set_stages(0);
    }
    if rules(this.addr()) == 0 {
        show_error_at(e.file, e.rows[0]);
        return this.addr();
    }
    if table(this.addr()) == 0 {
        show_error_at(e.file, e.rows[1]);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(e.file, e.rows[2]);
        return this.addr();
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(e.file, e.rows[3]);
        return this.addr();
    }
    let p = exe::crt_operator_new(0x5ce);
    let s = if p != 0 { exe::new_comp_stats(p, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(e.file, e.rows[4]);
        return this.addr();
    }
    exe::league_set_predicted_positions(this);
    this.addr()
}

/// Comparer for the division constructors: only the original runs (it allocates the table, the fixtures and the
/// statistics); then the plan: the returned this, the vtable, the year, the record, the discipline rule, the stadium
/// requirement and, in a new career with statistics, the stages and the table.
pub(crate) unsafe fn compare_british_division_ctor(name: &str, this: League, year: u32, record: usize, e: &BritishDivisionSpec, original: impl FnOnce() -> usize) -> usize {
    let loaded = league_will_load_saved_state(record);
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == e.vtable && built_year_ok(this.year() as u16, year as u16)
        && this.competition().addr() == record && this.discipline_rule() == e.discipline;
    if let Some((c, s)) = e.stadium {
        ok = ok && this.required_capacity() == c && this.required_seats() == s;
    }
    if !loaded && this.comp_stats() != 0 {
        let n = british_division_stage_count(this, e) as usize;
        ok = ok && this.stage_index() == 0xff && this.stage_count() == n && (this.stages() != 0) == (n != 0) && this.table() != 0;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}) [year arg {}, record arg {:#x}, loaded {}]: returned {:#x}, vtable {:#x}, year {}, record {:#x}, discipline {}, stages {} at {:#x}, table {:#x}, stats {:#x}",
        name, this.addr(), year as u16, record, loaded, r, read_usize(this.addr()), this.year(), this.competition().addr(), this.discipline_rule(), this.stage_count(), this.stages(), this.table(), this.comp_stats()));
    r
}

/// The table of a Scottish or Welsh division: `league::CLUB_COUNT` = `n`, the table (`exe::crt_malloc` of n × 0x3b;
/// out of memory, the box of `row` and 0), the clubs whose division (`club::DIVISION`) is the competition of global `g`,
/// in database order, each into the next row (`exe::league_init_table_row(this, row, club, 0, 0)`), until full
/// (`rotate` is 0 in the original: row k gets the k-th club; it exists only for the mutations).
///
/// Not full: 0. Full: with a prize fund, `league::PRIZE_FUND` = League::calc_prize_fund(`prize`) and `league::PRIZE_MODE`
/// = 1, and with TV money, League::pay_tv_rights(`tv`, 0) to every club (the Premier); returns 1 (the Second and Third
/// Divisions, the Welsh League and the Premier from 2000 on have no prize fund or TV money of their own here: only
/// those given).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn fill_british_division_table(this: League, n: i16, file: usize, row: u32, g: usize, prize: Option<i32>, tv: Option<i32>, rotate: u8, ramo: &'static str) -> u32 {
    this.set_club_count(n);
    let t = exe::crt_malloc((n as i32 as u32).wrapping_mul(0x3b));
    this.set_table(t);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut c = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(c + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(g) {
            // simplification: the original pushes the row in eax with the high bits of the division id; 0x687430 only
            // reads the byte
            let l = ((k as u32 + rotate as u32) % n as u32) as u8;
            exe::league_init_table_row(this, l as u32, c, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == this.club_count() {
                break;
            }
        }
        i = i.wrapping_add(1);
        c = c.wrapping_add(club::SIZE);
    }
    if k as i8 as i16 != this.club_count() {
        return 0;
    }
    covered!(ramo);
    if let Some(q) = prize {
        let f = exe::league_calc_prize_fund(this, q);
        this.set_prize_fund(f);
        this.set_prize_mode(1);
    }
    if let Some(v) = tv {
        exe::league_pay_tv_rights(this, v, 0usize);
    }
    1
}

/// Comparer for the division tables: the division's clubs planned beforehand (database order, up to `n`); only the
/// original runs (it allocates and pays); then the count, the return (1 when the plan filled up), the rows and, when full
/// and with a prize fund, the fund and the mode.
pub(crate) unsafe fn compare_british_division_table(name: &str, this: League, n: i16, g: usize, prize: Option<i32>, original: impl FnOnce() -> u32) -> u32 {
    let mut plan: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(G_CLUB_COUNT) as i32 && (plan.len() as i16) < n {
        let c = Club(read_usize(G_CLUB_TABLE) + (i as usize) * club::SIZE);
        let d = c.division().addr();
        if d != 0 && read_usize(d) == read_usize(g) {
            plan.push(c.addr());
        }
        i += 1;
    }
    let r = original();
    let t = this.table();
    let expected = (plan.len() as i16 == n) as u32;
    let mut same = r == expected && this.club_count() == n && t != 0;
    let mut found: Vec<usize> = Vec::new();
    if same {
        found = (0..plan.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        same = found == plan;
        if let (1, Some(q)) = (r, prize) {
            same = same && this.prize_mode() == 1 && this.prize_fund() == exe::league_calc_prize_fund(this, q);
        }
    }
    SCOREBOARD.record(same, || format!("{}({:#x}) original={} plan={:x?} table={:x?}", name, this.addr(), r, plan, found));
    r
}

/// The First Division rule writes (`scottish_division_rules`): the champion goes up to the Premier; in 1999 2 to the
/// promotion play-off and nobody down, from 2000 2 down to the Second.
pub(crate) unsafe fn scottish_first_rules(this: League) -> Vec<(usize, u8, u32)> {
    scottish_division_rules(this, 3, 1, 2, 2, KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION, KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION) // mwb5: the First Division rules
}

/// The main stage (stage −1) of a Scottish division's `create_schedule`: `league` (when given) = 1, `rounds` = the
/// number of rounds, `extra` = 0; the table (`exe::crt_malloc` of 0x41 per round; out of memory, the box of the first
/// line and 0) filled by `set_scottish_league_rounds`; returns the table.
pub(crate) unsafe fn scottish_league_round_table(this: League, rounds: *mut i16, extra: *mut i16, league: *mut i32, s: &ScottishSchedule, ramo: &'static str) -> usize {
    if !league.is_null() {
        *league = 1;
    }
    let n = s.before.len() + s.cases[0].len() + s.after.len();
    *rounds = n as i16;
    *extra = 0;
    let t = exe::crt_malloc(n as u32 * 0x41);
    if t == 0 {
        show_error_at(s.file, s.rows.0);
        return 0;
    }
    covered!(ramo);
    set_scottish_league_rounds(t, (this.year() as u16) as u32, s);
    t
}

/// The turnover of a Scottish division (First, Second, Third) or of the League of Wales (`welsh`), in the original
/// routine's order.
///
/// `league::SEASON_CLOSED` = 0; the League of Wales first records the final positions (virtual +0xa4); the table freed
/// (`exe::league_free_table(this, 0)`), the round dates freed and zeroed, the season archived in the statistics
/// (`exe::comp_stats_archive_season_and_reset(stats, 1)`); the Scottish divisions destroy the stages 0..=current
/// (virtual 0 with 1) and set the current stage to −1; the First also frees the stage vector (always, even null) and,
/// after the year, rebuilds the stage count and vector of `british_division_stage_count`. The year goes up by `step` (1).
///
/// Then, each with its error box (and 0) when it returns 0: the rules (virtual +0x8c), the table (`e.table`), the
/// fixtures and the order; finally the predicted positions (virtual +0x5c), the clubs' board expectations
/// (`exe::league_set_club_season_expectations`) and the renewal of the officials, whose eax it returns
/// (`set_expectations_and_renew_officials`).
pub(crate) unsafe fn british_division_new_season(this: League, e: &BritishDivisionSpec, rows: [u32; 4], welsh: bool, step: i16, ramo: &'static str) -> u32 {
    let call = |slot: usize| {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + slot));
        f(this.addr())
    };
    this.set_season_closed((0) as usize);
    if welsh {
        call(league_vtable::RECORD_FINAL_POSITIONS);
    }
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
    if !welsh {
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
        this.set_current_stage(0xffff_ffff);
    }
    if e.playoff_in_1999 {
        exe::crt_free(this.stages());
    }
    this.set_year(this.year().wrapping_add(step));
    if e.playoff_in_1999 {
        let n = british_division_stage_count(this, e);
        this.set_stage_count(n as usize);
        if n as i32 > 0 {
            let v = exe::crt_malloc(n * 4);
            this.set_stages(v);
        } else {
            this.set_stages(0);
        }
    }
    covered!(ramo);
    if call(comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(e.file, rows[0]);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.table);
    if table(this.addr()) == 0 {
        show_error_at(e.file, rows[1]);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(e.file, rows[2]);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(e.file, rows[3]);
        return 0;
    }
    call(comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// Maps a progress code of the 1999 play-off: 1 → `first`, 2 and 3 → `rest`, the others as they are.
pub(crate) fn scottish_playoff_code(v: u8, first: u8, rest: u8) -> u8 {
    match v {
        1 => first,
        2 | 3 => rest,
        _ => v,
    }
}

/// The number of stages beyond the league: 1 in the 1999 season of a division with a play-off then, 0 otherwise.
pub(crate) unsafe fn british_division_stage_count(this: League, e: &BritishDivisionSpec) -> u32 {
    if e.playoff_in_1999 && this.year() == 0x7cf { 1 } else { 0 }
}

/// The rules of a Scottish division (the Premier has its own): 4 rounds (`league::LEGS`), 3 points for a win and 1 for a
/// draw, category 2, club sides, tie-breaks 1, 2 and 3 (goal difference, goals scored, position [inference from the
/// English codes]), `promoted` promoted directly; in 1999 `playoff_1999` to the promotion play-off and nobody relegated,
/// from 2000 `relegated` relegated directly; the divisions above and below (`upper` and `lower`: the globals of the ids,
/// 0 = none, −1), 3 substitutions and 5 substitutes named.
pub(crate) unsafe fn scottish_division_rules(this: League, win: u8, promoted: u8, playoff_1999: u8, relegated: u8, upper: usize, lower: usize) -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 4), (league::WIN_POINTS, 1, win as u32), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::CLUB_SIDES, 1, 1),
                     (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 3), (league::PROMOTED_COUNT, 1, promoted as u32)];
    if this.year() == 0x7cf {
        v.extend_from_slice(&[(league::PROMOTION_PLAYOFF_COUNT, 1, playoff_1999 as u32), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 0)]);
    } else {
        v.extend_from_slice(&[(league::RELEGATED_COUNT, 1, relegated as u32), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 0)]);
    }
    let id = |g: usize| if g == 0 { 0xffff_ffff } else { read_usize(g) as u32 };
    v.extend_from_slice(&[(league::UPPER_DIVISION_ID, 4, id(upper)), (league::LOWER_DIVISION_ID, 4, id(lower)), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SUBSTITUTES_NAMED, 1, 5)]);
    v
}

/// Writes the main-stage rounds of a Scottish division into table `t`, in the original routine's order.
///
/// The rounds before (`eng_first_set_league_rounds`: League round date and its TV slots, cdecl); then the date of 1
/// December of the year (Date::Date `exe::date_date_549f70`) and k = (the day of the reference round, word at +0 of its
/// 0x41-byte entry, − the day of that date + 1, in 16 bits) − 15: from 0 to 7 the year-end round of case k; outside, the
/// error box of the second line and the default round (moved to Saturday instead of the weekday the cases leave); then
/// the rounds after.
pub(crate) unsafe fn set_scottish_league_rounds(t: usize, year: u32, s: &ScottishSchedule) {
    eng_first_set_league_rounds(t, year, 0, s.before);
    let mut d = [0u16; 4];
    exe::date_date_549f70(&mut d, 1, 0xb, year, 0xffff_ffff);
    let day = read_u16(t + s.reference * 0x41);
    let k = (day.wrapping_sub(d[0]).wrapping_add(1) as i16 as i32).wrapping_sub(0xf);
    let middle = if k as u32 > 7 {
        show_error_at(s.file, s.rows.1);
        s.default
    } else {
        s.cases[k as usize]
    };
    eng_first_set_league_rounds(t, year, s.before.len() as i32, middle);
    eng_first_set_league_rounds(t, year, (s.before.len() + middle.len()) as i32, s.after);
}

/// The Scottish First Division: discipline rule 0x18, no stadium requirement, the promotion play-off in 1999.
pub(crate) const SCOTTISH_FIRST_DIVISION: BritishDivisionSpec = BritishDivisionSpec {
    vtable: 0x96f4b4,
    stadium: None,
    discipline: 0x18, // mwb5: the Scottish discipline rule
    playoff_in_1999: true,
    split_compare: None,
    rules: 0x7eee50,
    table: 0x7eed30,
    file: 0xa7ade4,
    rows: [0x4c, 0x53, 0x5a, 0x63, 0x6a],
};

pub(crate) const SCOTTISH_FIRST_SCHEDULE: ScottishSchedule = ScottishSchedule {
    before: &SCOTTISH_FIRST_BEFORE,
    reference: 18,
    cases: SCOTTISH_YEAR_END_CASES,
    default: &SCOTTISH_BOXING_DAY_SATURDAY,
    after: &SCOTTISH_FIRST_AFTER,
    file: 0xa7ade4,
    rows: (0xb1, 0x118),
};

/// The 1999 promotion play-off of the First Division (stage 0): 27 April, 31 May (code 2) and 4 June of the next year.
pub(crate) const SCOTTISH_FIRST_PLAYOFF: [(i32, i32, i32, i32, i32); 3] = [(27, 4, 1, 5, 1), (31, 4, 1, 2, 2), (4, 5, 1, 5, 1)];

/// The Scottish First Division before the turn of the year: 19 Saturday rounds from 7 August to 18 December (none on 9
/// October).
pub(crate) const SCOTTISH_FIRST_BEFORE: [EngFirstScheduleRound; 19] = [
    (7, 7, 0, 5, 1, &[(-1, -1, -1)]), // 0: 7/ago
    (14, 7, 0, 5, 1, &[(-1, -1, -1)]), // 1: 14/ago
    (21, 7, 0, 5, 1, &[(-1, -1, -1)]), // 2: 21/ago
    (28, 7, 0, 5, 1, &[(-1, -1, -1)]), // 3: 28/ago
    (4, 8, 0, 5, 1, &[(-1, -1, -1)]), // 4: 4/set
    (11, 8, 0, 5, 1, &[(-1, -1, -1)]), // 5: 11/set
    (18, 8, 0, 5, 1, &[(-1, -1, -1)]), // 6: 18/set
    (25, 8, 0, 5, 1, &[(-1, -1, -1)]), // 7: 25/set
    (2, 9, 0, 5, 1, &[(-1, -1, -1)]), // 8: 2/out, mwb6: a First Division round
    (16, 9, 0, 5, 1, &[(-1, -1, -1)]), // 9: 16/out
    (23, 9, 0, 5, 1, &[(-1, -1, -1)]), // 10: 23/out
    (30, 9, 0, 5, 1, &[(-1, -1, -1)]), // 11: 30/out
    (6, 10, 0, 5, 1, &[(-1, -1, -1)]), // 12: 6/nov
    (13, 10, 0, 5, 1, &[(-1, -1, -1)]), // 13: 13/nov
    (20, 10, 0, 5, 1, &[(-1, -1, -1)]), // 14: 20/nov
    (27, 10, 0, 5, 1, &[(-1, -1, -1)]), // 15: 27/nov
    (4, 11, 0, 5, 1, &[(-1, -1, -1)]), // 16: 4/dez
    (11, 11, 0, 5, 1, &[(-1, -1, -1)]), // 17: 11/dez
    (18, 11, 0, 5, 1, &[(-1, -1, -1)]), // 18: 18/dez
];

/// The Scottish First Division after the turn of the year: 3 January (on its own weekday, code 2), then Saturdays to 6
/// May of the next year.
pub(crate) const SCOTTISH_FIRST_AFTER: [EngFirstScheduleRound; 16] = [
    (3, 0, 1, 0, 2, &[(-1, -1, -1)]), // 20: 3/jan
    (8, 0, 1, 5, 1, &[(-1, -1, -1)]), // 21: 8/jan
    (15, 0, 1, 5, 1, &[(-1, -1, -1)]), // 22: 15/jan
    (22, 0, 1, 5, 1, &[(-1, -1, -1)]), // 23: 22/jan
    (5, 1, 1, 5, 1, &[(-1, -1, -1)]), // 24: 5/fev
    (12, 1, 1, 5, 1, &[(-1, -1, -1)]), // 25: 12/fev
    (26, 1, 1, 5, 1, &[(-1, -1, -1)]), // 26: 26/fev
    (4, 2, 1, 5, 1, &[(-1, -1, -1)]), // 27: 4/mar
    (18, 2, 1, 5, 1, &[(-1, -1, -1)]), // 28: 18/mar
    (25, 2, 1, 5, 1, &[(-1, -1, -1)]), // 29: 25/mar
    (1, 3, 1, 5, 1, &[(-1, -1, -1)]), // 30: 1/abr
    (8, 3, 1, 5, 1, &[(-1, -1, -1)]), // 31: 8/abr
    (15, 3, 1, 5, 1, &[(-1, -1, -1)]), // 32: 15/abr
    (22, 3, 1, 5, 1, &[(-1, -1, -1)]), // 33: 22/abr
    (29, 3, 1, 5, 1, &[(-1, -1, -1)]), // 34: 29/abr
    (6, 4, 1, 5, 1, &[(-1, -1, -1)]), // 35: 6/mai
];
