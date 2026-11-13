// ---------- mwb_6: the Scottish First Division (sco_first.cpp): the schedule, the turnover and the club progress; the moulds of the Scottish schedules and turnovers ----------

/// The main-stage schedule of a Scottish division: the rounds before the turn of the year, the reference round, the
/// eight cases of the year-end round chosen by the weekday (and the default one), the rounds after it, the source file
/// and the error-box lines (out of memory, the case out of range).
pub(crate) struct ScottishSchedule {
    pub(crate) before: &'static [EngFirstScheduleRound],
    pub(crate) reference: usize,
    pub(crate) cases: [&'static [EngFirstScheduleRound]; 8],
    pub(crate) default: &'static [EngFirstScheduleRound],
    pub(crate) after: &'static [EngFirstScheduleRound],
    pub(crate) file: usize,
    pub(crate) rows: (u32, u32),
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
    write_u32(this.addr() + league::SEASON_CLOSED, 0);
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

/// The year-end round of the Scottish divisions by case (the day of the reference round minus 1 December, minus 15):
/// 26 December with code 2 in cases 0, 1, 2 and 7, with code 1 in 3 and 4, 27 December in 5, 28 December (code 2) in 6
/// (the weekday left as it is: −1); out of range, 26 December moved to Saturday.
pub(crate) const SCOTTISH_BOXING_DAY_A: [EngFirstScheduleRound; 1] = [(26, 11, 0, -1, 2, &[(-1, -1, -1)])];
pub(crate) const SCOTTISH_BOXING_DAY_B: [EngFirstScheduleRound; 1] = [(26, 11, 0, -1, 1, &[(-1, -1, -1)])];
pub(crate) const SCOTTISH_DECEMBER_27: [EngFirstScheduleRound; 1] = [(27, 11, 0, -1, 1, &[(-1, -1, -1)])];
pub(crate) const SCOTTISH_DECEMBER_28: [EngFirstScheduleRound; 1] = [(28, 11, 0, -1, 2, &[(-1, -1, -1)])];
pub(crate) const SCOTTISH_BOXING_DAY_SATURDAY: [EngFirstScheduleRound; 1] = [(26, 11, 0, 5, 1, &[(-1, -1, -1)])];
pub(crate) const SCOTTISH_YEAR_END_CASES: [&[EngFirstScheduleRound]; 8] = [&SCOTTISH_BOXING_DAY_A, &SCOTTISH_BOXING_DAY_A, &SCOTTISH_BOXING_DAY_A,
    &SCOTTISH_BOXING_DAY_B, &SCOTTISH_BOXING_DAY_B, &SCOTTISH_DECEMBER_27, &SCOTTISH_DECEMBER_28, &SCOTTISH_BOXING_DAY_A];

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("sco_first_start_new_season", League(this), || o(this))
}

/// Maps a progress code of the 1999 play-off: 1 → `first`, 2 and 3 → `rest`, the others as they are.
fn scottish_playoff_code(v: u8, first: u8, rest: u8) -> u8 {
    match v {
        1 => first,
        2 | 3 => rest,
        _ => v,
    }
}

/// rule: outside 1999 the club's progress codes go to the common routine (Comp::set_club_progress 0x48e380, the same
/// this and arguments: thiscall, ret 0x14). In 1999 they go into the club's Comp_Stats entry (current, best, worst):
/// in the league (stage 0xff) a best of 2 becomes 1; in the play-off (stage 0) the Premier club's codes go into the
/// Premier's statistics entry with 1 → 8 and 2, 3 → 10, and a First Division club's own with 1 → 1 and 2, 3 → 3; any
/// other stage as they came (the entry is not checked for null)
/// Original: `ScoFirst::set_club_progress` (0x7ef100, sco_first.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn sco_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let league = League(this);
    if league.year() != 0x7cf {
        covered!("mwb_6 first division progress");
        let common: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x48e380usize);
        return common(this, club, stage, current, best, worst); // mwb6: the common progress codes
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
            let d = read_usize(club + club::DIVISION);
            if read_usize(d) == read_usize(KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION) {
                let premier = comp_by_global(KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION);
                e = exe::comp_stats_find_club_entry(read_usize(premier + league::COMP_STATS), club);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    if League(this).year() == 0x7cf && stage as u8 == 0 {
        return o(this, club, stage, current, best, worst);
    }
    compare_club_stage_status("sco_first_set_club_progress", this, club, || o(this, club, stage, current, best, worst), || sco_first_set_club_progress(this, club, stage, current, best, worst))
}
