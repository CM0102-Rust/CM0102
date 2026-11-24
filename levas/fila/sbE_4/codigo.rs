// ---------- sbE_4: the Eerste Divisie schedule, its period stages and the period winners (hol_first.cpp) ----------

/// Rule: the Eerste Divisie schedule: the season (the main stage) in 34 rounds from 14 August to 14 May of the next
/// year, mostly on weekday 5 with one TV slot (0/2, one match), five midweek rounds and the last two on weekday 6; each
/// of the two play-off groups (stages 4 and 5) in 6 rounds from 17 May to 3 June of the next year.
///
/// `HolFirst::create_schedule(stage, &rounds, &extra, &is_league)`: at −1 is_league (if asked) = 1, rounds = 34 and
/// extra = 0, the table of 34 × 0x41 bytes (`exe::crt_malloc`; none: the hol_first.cpp (0x9cee08) box at line 0xad and
/// 0); at 4 and 5 is_league = 1, rounds = 6 and extra = 0x418 (stage 4) or 0x419 (stage 5), the table of 6 rounds (the
/// box at line 0x171); the dates and slots by `fra_third_write_league_rounds`. Any other stage: 0, nothing written.
/// Defect of the original: round 13 is dated 30 November (month 10 for 9), before 6 November.
/// Original: `HolFirst::create_schedule` (0x5f5790, hol_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn hol_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let l = League(this);
    let (n, code, row, table): (i16, i16, u32, &[FraThirdLeagueRound]) = match stage as u8 {
        0xff => (0x22, 0, 0xad, &HOL_FIRST_ROUNDS),
        4 | 5 => (6, 0x418 + (stage as u8 != 4) as i16, 0x171, &HOL_FIRST_PLAY_OFF_ROUNDS), // sbE_4: the play-off group labels
        _ => return 0,
    };
    if !league.is_null() {
        *league = 1;
    }
    *rounds = n;
    *extra = code;
    let t = exe::crt_malloc(n as u32 * 0x41);
    if t == 0 {
        show_error_at(0x9cee08, row);
        return 0;
    }
    covered!("sbE_4 eerste divisie schedule");
    fra_third_write_league_rounds(t, (l.year() as u16) as u32, 0, table);
    t
}

/// Comparer for `hol_first_create_schedule`: side by side (`compare_staged_schedule`, league rounds).
pub(crate) unsafe extern "thiscall" fn compare_hol_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_staged_schedule("hol_first_create_schedule", stage, rounds, extra, league, ScheduleTable::LeagueRounds,
        || o(this, stage, rounds, extra, league), |r, e, lg| hol_first_create_schedule(this, stage, r, e, lg))
}

/// The Eerste Divisie's period winner flags: one byte per period (+0xee to +0xf1), 1 once that period's winner is
/// known (set_up_rules clears them; `hol_first_update_positions_and_status` sets them in turn).
pub(crate) const HOL_FIRST_PERIOD_WON: usize = 0xee;

/// Rule: each of the Eerste Divisie's four periods is a stage of its own with all 18 clubs (8 rounds, 10 in the third);
/// creating it makes it the current stage, and the earlier period winners are marked as out of it (0xfe).
///
/// `HolFirst::create_period_stage(period)`: the list (`exe::crt_malloc` of 4 × `league::CLUB_COUNT`; none: the
/// hol_first.cpp box at line 0x300 and nothing else) with the table's clubs in row order; a League_Stage
/// (`exe::crt_operator_new`(0xee); League_Stage::League_Stage 0x68b100, thiscall, ret 0x58: this league, the club count,
/// the list, 1 leg, the competition, no round table and 0 rounds, the win, draw and category bytes, the tie-breaks, the
/// places (0, 1, 0, 0), the year, the period, the label 0x439 + period (0x43c from the fourth on), importance 0, 1, 0,
/// rule bits 0x22, 10 for the third period and 8 for the others in the −1 slot, prize fund 0 and prize mode 2) into
/// stage slot `period` (0 without memory, unchecked); then, for each league row with status 1 (16-bit index, the count
/// read again), the first row of the new stage (League::get_table_row, up to the league's club count) with the same
/// club gets the status 0xfe; `league::CURRENT_STAGE` = the period; the list freed.
/// Original: `HolFirst::create_period_stage` (0x5f6ad0, hol_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn hol_first_create_period_stage(this: usize, period: u32) {
    let l = League(this);
    let list = exe::crt_malloc((l.club_count() as i32 as u32).wrapping_mul(4));
    if list == 0 {
        return show_error_at(0x9cee08, 0x300);
    }
    let mut k: i16 = 0;
    while k < l.club_count() {
        write_usize(list + 4 * k as usize, LeagueTableRow(l.table() + k as usize * league_table_row::SIZE).club() as usize);
        k += 1;
    }
    let (label, slot): (u32, i32) = match period as u8 {
        0 => (0x439, 8),
        1 => (0x43a, 8),
        2 => (0x43b, 10),
        _ => (0x43c, 8),
    };
    covered!("sbE_4 eerste divisie period stage");
    let places: [u8; 4] = [0, 1, 0, 0];
    let p = exe::crt_operator_new(0xee);
    let s = if p != 0 {
        let ctor: LeagueStageCtorFn = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        ctor(p, this, l.club_count() as u16 as u32, list, 1, l.competition().addr(), 0, 0, l.win_points() as u32, l.draw_points() as u32,
            l.match_category() as u32, this + league::TIEBREAKS, places.as_ptr() as usize, l.year() as u16 as u32, period, label, 0, 1, 0, 0x22, slot, 0, 2)
    } else {
        0
    };
    let g = period as u8 as i8 as i32;
    write_usize(l.stages() + 4 * g as usize, s);
    let mut k: i16 = 0;
    while k < l.club_count() {
        let row = LeagueTableRow(l.table() + k as usize * league_table_row::SIZE);
        if row.status() == 1 {
            let mut j: i16 = 0;
            while j < l.club_count() {
                if read_usize(exe::league_get_table_row(s, j as i32)) == row.club() as usize {
                    write_u8(exe::league_get_table_row(s, j as i32) + league_table_row::STATUS, 0xfe); // sbE_4: an earlier winner is out
                    break;
                }
                j += 1;
            }
        }
        k += 1;
    }
    l.set_current_stage(g as usize);
    exe::crt_free(list);
}

/// Comparer for `hol_first_create_period_stage`: only the original (it builds the stage); afterwards the period's slot
/// must hold a stage and be the current stage.
pub(crate) unsafe extern "thiscall" fn compare_hol_first_create_period_stage(this: usize, period: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I1%%)(this, period);
    let (l, g) = (League(this), period as u8 as i8 as i32);
    let s = read_usize(l.stages() + 4 * g as usize);
    SCOREBOARD.record(s != 0 && l.current_stage() as i32 == g, || format!("hol_first_create_period_stage({:#x}, {}): stage {:#x}, current {}", this, g, s, l.current_stage() as i32));
}

/// Rule: after the usual table update, the winner of the next period still undecided, once that period's stage is
/// reached and its table has a row marked champion (1), qualifies for the play-off: its first row in the Eerste Divisie
/// table not decided otherwise (status other than 0 and 5) gets 1, with the news and its progress; it is marked out
/// (0xfe) of the later period stages already built; then that period counts as won.
///
/// `HolFirst::update_positions_and_status()`: League::update_positions_and_status (0x684cd0, through its detour); the
/// first period p (1 to 4) whose flag (`HOL_FIRST_PERIOD_WON` + p − 1) is 0 (none: nothing else); with
/// `league::CURRENT_STAGE` at or past p − 1, the first row with status 1 in the table of stage p − 1 (up to the league's
/// club count), and its club not null: the first league row of that club with a status other than 0 and 5 gets 1,
/// HolFirst::add_period_winner_news (0x5f7330, thiscall(club, p), ret 8) and `belgian_progress_virtual` (virtual
/// `comp_vtable::SET_CLUB_PROGRESS`(club, `league::STAGE_INDEX`, 2, 2, 7)); then in each stage from p to the current one
/// the first row (League::get_table_row, up to the league's club count) whose club has the winner's id gets 0xfe; last,
/// the first period flag still at 0 becomes 1.
/// Original: `HolFirst::update_positions_and_status` (0x5f6d20, hol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_first_update_positions_and_status(this: usize) {
    let l = League(this);
    let base: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684cd0usize); // League::update_positions_and_status
    base(this);
    let Some(p) = (0..4).find(|&k| read_u8(this + HOL_FIRST_PERIOD_WON + k) == 0).map(|k| k as i32 + 1) else {
        return;
    };
    if (l.current_stage() as i32) < p - 1 {
        return;
    }
    let n = l.club_count() as i32;
    let period_table = League(read_usize(l.stages() + 4 * (p - 1) as usize)).table();
    let Some(k) = (0..n.max(0) as usize).find(|&k| read_u8(period_table + k * league_table_row::SIZE + league_table_row::STATUS) == 1) else {
        return;
    };
    let winner = read_usize(period_table + k * league_table_row::SIZE);
    if winner == 0 {
        return;
    }
    if let Some(row) = (0..n.max(0) as usize).map(|j| LeagueTableRow(l.table() + j * league_table_row::SIZE))
        .find(|r| r.club() as usize == winner && r.status() != 0 && r.status() != 5) {
        covered!("sbE_4 eerste divisie period winner");
        row.set_status(1); // sbE_4: the period winner goes to the play-off
        let news: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x5f7330usize); // HolFirst::add_period_winner_news
        news(this, row.club() as usize, p as u8 as u32);
        belgian_progress_virtual(l, row.club() as usize);
    }
    let mut g = p;
    while g <= l.current_stage() as i32 {
        let stage = read_usize(l.stages() + 4 * g as usize);
        let mut j = 0i32;
        while j < l.club_count() as i32 {
            if read_i32(read_usize(exe::league_get_table_row(stage, j))) == read_i32(winner) {
                write_u8(exe::league_get_table_row(stage, j) + league_table_row::STATUS, 0xfe);
                break;
            }
            j += 1;
        }
        g += 1;
    }
    let first_open = (0..4).find(|&k| read_u8(this + HOL_FIRST_PERIOD_WON + k) == 0).unwrap_or(3);
    write_u8(this + HOL_FIRST_PERIOD_WON + first_open, 1);
}

/// Comparer for `hol_first_update_positions_and_status`: only the original (it updates the table and sends news).
pub(crate) unsafe extern "thiscall" fn compare_hol_first_update_positions_and_status(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

/// The Eerste Divisie season: 34 rounds (date, TV slots).
pub(crate) const HOL_FIRST_ROUNDS: [FraThirdLeagueRound; 34] = [
    (14, 7, 0, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 0: 14 Aug
    (21, 7, 0, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 1: 21 Aug
    (28, 7, 0, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 2: 28 Aug
    (11, 8, 0, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 3: 11 Sep
    (15, 8, 0, 2, 2, &[(-1, -1, -1)]), // 4: 15 Sep
    (18, 8, 0, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 5: 18 Sep
    (25, 8, 0, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 6: 25 Sep
    (29, 8, 0, 2, 2, &[(-1, -1, -1)]), // 7: 29 Sep
    (2, 9, 0, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 8: 2 Oct
    (13, 9, 0, 2, 2, &[(-1, -1, -1)]), // 9: 13 Oct
    (16, 9, 0, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 10: 16 Oct
    (23, 9, 0, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 11: 23 Oct
    (30, 10, 0, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 12: 30 Nov (the original's month 10 for 9)
    (6, 10, 0, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 13: 6 Nov
    (20, 10, 0, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 14: 20 Nov
    (27, 10, 0, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 15: 27 Nov
    (4, 11, 0, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 16: 4 Dec
    (11, 11, 0, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 17: 11 Dec
    (18, 11, 0, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 18: 18 Dec
    (5, 1, 1, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 19: 5 Feb (+1 year)
    (12, 1, 1, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 20: 12 Feb (+1 year)
    (19, 1, 1, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 21: 19 Feb (+1 year)
    (26, 1, 1, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 22: 26 Feb (+1 year)
    (4, 2, 1, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 23: 4 Mar (+1 year)
    (11, 2, 1, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 24: 11 Mar (+1 year)
    (18, 2, 1, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 25: 18 Mar (+1 year)
    (25, 2, 1, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 26: 25 Mar (+1 year)
    (1, 3, 1, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 27: 1 Apr (+1 year)
    (8, 3, 1, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 28: 8 Apr (+1 year)
    (15, 3, 1, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 29: 15 Apr (+1 year)
    (22, 3, 1, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 30: 22 Apr (+1 year)
    (29, 3, 1, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 31: 29 Apr (+1 year)
    (7, 4, 1, 6, 2, &[(-1, -1, -1)]), // 32: 7 May (+1 year)
    (14, 4, 1, 6, 2, &[(-1, -1, -1)]), // 33: 14 May (+1 year)
];

/// Each Eerste Divisie play-off group (stages 4 and 5): 6 rounds (date, TV slots).
pub(crate) const HOL_FIRST_PLAY_OFF_ROUNDS: [FraThirdLeagueRound; 6] = [
    (17, 4, 1, 2, 2, &[(-1, -1, -1)]), // 0: 17 May (+1 year)
    (20, 4, 1, 5, 2, &[(-1, -1, -1)]), // 1: 20 May (+1 year)
    (24, 4, 1, 2, 2, &[(-1, -1, -1)]), // 2: 24 May (+1 year)
    (27, 4, 1, 5, 2, &[(-1, -1, -1)]), // 3: 27 May (+1 year)
    (31, 4, 1, 2, 2, &[(-1, -1, -1)]), // 4: 31 May (+1 year)
    (3, 5, 1, 5, 2, &[(-1, -1, -1)]), // 5: 3 Jun (+1 year)
];
