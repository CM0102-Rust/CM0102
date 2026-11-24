// ---------- cx08_16: the Belgian Second Division periods and promotion play-off group (bel_second.cpp) ----------

/// The Belgian Second Division period winner flags (bytes at +0xf6, +0xf7 and +0xf8: 1 once the first, second or third
/// period has its winner) [inference: set_up_rules clears them, update_positions_and_status sets them in turn].
pub(crate) const BEL_SECOND_PERIOD_WON: usize = 0xf6;

/// Rule: the promotion play-off group of the Belgian Second Division: the three period winners (rows already marked 1)
/// are completed to four with the best-placed clubs not yet decided, each marked, announced and given its progress; the
/// four meet in a home-and-away group (stage 3).
///
/// `BelSecond::create_playoff_group()`: the rows with `league_table_row::STATUS` 1 are counted; while fewer than 4, each
/// row in table order with status 0xff gets 1, the status announced (virtual `comp_vtable::ANNOUNCE_TEAM_STATUS`(club,
/// 1, `league::STAGE_INDEX`, 0, 0)) and its progress set (virtual `comp_vtable::SET_CLUB_PROGRESS`(club, the stage
/// index, 2, 2, 7)). The list (`exe::crt_malloc` of 0x10; none: the bel_second.cpp box of line 0x2ac and nothing else)
/// with the first four clubs with status 1; the round table of stage 3 (virtual `comp_vtable::CREATE_SCHEDULE`(3,
/// &rounds, &label, 0)); a League_Stage (`exe::crt_operator_new`(0xee); League_Stage::League_Stage 0x68b100, thiscall,
/// ret 0x58: this league, 4 clubs, the list, 2 legs, the competition, the table and its count, the win, draw and
/// category bytes, the tie-breaks, the places (1, 0, 0, 0), the year, 3, the label, importance 0x14, 1, 0, rule bits
/// 0x22, −1, prize fund 0 and prize mode 2) into stage slot 3; the list and the round table freed; no stage: the box of
/// line 0x2c5.
/// Original: `BelSecond::create_playoff_group` (0x420230, bel_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_second_create_playoff_group(this: usize) {
    let this = League(this);
    let row = |k: i32| LeagueTableRow(this.table() + k as usize * league_table_row::SIZE);
    let mut qualified = (0..this.club_count() as i32).filter(|&k| row(k).status() == 1).count() as i32;
    let mut k = 0i32;
    while k < this.club_count() as i32 && qualified < 4 {
        let r = row(k);
        if r.status() == 0xff {
            r.set_status(1);
            qualified += 1;
            let announce: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::ANNOUNCE_TEAM_STATUS));
            announce(this.addr(), r.club() as usize, 1, this.stage_index() as u32, 0, 0);
            belgian_progress_virtual(this, r.club() as usize);
        }
        k += 1;
    }
    let list = exe::crt_malloc(0x10);
    if list == 0 {
        return show_error_at(0x989b54, 0x2ac);
    }
    covered!("cx08_16 belgian play-off group");
    let (mut n, mut k) = (0usize, 0i32);
    while k < this.club_count() as i32 && n < 4 {
        if row(k).status() == 1 {
            write_usize(list + 4 * n, row(k).club() as usize);
            n += 1;
        }
        k += 1;
    }
    let places: [u8; 4] = [1, 0, 0, 0];
    let (mut rounds, mut label) = (0i16, 0i16);
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let table = schedule(this.addr(), 3, &mut rounds, &mut label, 0);
    let p = exe::crt_operator_new(0xee);
    let s = if p != 0 {
        let ctor: LeagueStageCtorFn = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        ctor(p, this.addr(), 4, list, 2, this.competition().addr(), table, rounds as u16 as u32, this.win_points() as u32, this.draw_points() as u32,
            this.match_category() as u32, this.addr() + league::TIEBREAKS, places.as_ptr() as usize, this.year() as u16 as u32, 3, label as u16 as u32, 0x14, 1, 0, 0x22, -1, 0, 2)
    } else {
        0
    };
    write_usize(this.stages() + 0xc, s);
    exe::crt_free(list);
    exe::crt_free(table);
    if read_usize(this.stages() + 0xc) == 0 {
        show_error_at(0x989b54, 0x2c5);
    }
}

/// A Belgian Second Division club's progress at a period or play-off place: virtual `comp_vtable::SET_CLUB_PROGRESS`
/// (club, `league::STAGE_INDEX`, 2, 2, 7).
pub(crate) unsafe fn belgian_progress_virtual(this: League, club: usize) {
    let progress: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_CLUB_PROGRESS));
    progress(this.addr(), club, this.stage_index() as u32, 2, 2, 7);
}

/// Comparer for `bel_second_create_playoff_group`: only the original (it marks, announces and builds the stage);
/// afterwards stage slot 3 must hold a stage of 4 clubs.
pub(crate) unsafe extern "thiscall" fn compare_bel_second_create_playoff_group(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let s = read_usize(League(this).stages() + 0xc);
    SCOREBOARD.record(s != 0 && League(s).club_count() == 4, || format!("bel_second_create_playoff_group({:#x}): stage {:#x}", this, s));
}

/// Rule: each third of the Belgian Second Division season is a stage of its own with all 18 clubs: the first period
/// (10 rounds), the second and the third (12 each); creating it makes it the current stage.
///
/// `BelSecond::create_period_stage(period)`: the list (`exe::crt_malloc` of 4 × `league::CLUB_COUNT`; none: the
/// bel_second.cpp box of line 0x346 and nothing else) with the table's clubs in row order; a League_Stage
/// (`exe::crt_operator_new`(0xee); League_Stage::League_Stage 0x68b100: this league, the club count, the list, 1 leg,
/// the competition, no round table and 0 rounds, the win, draw and category bytes, the tie-breaks, the places (0, 1, 0,
/// 0), the year, the period, the label 0x439 + period (0x43b from the third on), importance 0, 1, 0, rule bits 2, 10
/// for the first period and 12 for the others in the −1 slot, prize fund 0 and prize mode 2) into stage slot `period`
/// (0 without memory, unchecked); `league::CURRENT_STAGE` = the period; the list freed.
/// Original: `BelSecond::create_period_stage` (0x4204d0, bel_second.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn bel_second_create_period_stage(this: usize, period: u32) {
    let this = League(this);
    let list = exe::crt_malloc((this.club_count() as i32 as u32).wrapping_mul(4));
    if list == 0 {
        return show_error_at(0x989b54, 0x346);
    }
    let mut k: i16 = 0;
    while k < this.club_count() {
        write_usize(list + 4 * k as usize, LeagueTableRow(this.table() + k as usize * league_table_row::SIZE).club() as usize);
        k += 1;
    }
    let (label, rounds) = match period as u8 {
        0 => (0x439u32, 10i32),
        1 => (0x43a, 12),
        _ => (0x43b, 12),
    };
    covered!("cx08_16 belgian period stage");
    let places: [u8; 4] = [0, 1, 0, 0];
    let p = exe::crt_operator_new(0xee);
    let s = if p != 0 {
        let ctor: LeagueStageCtorFn = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        ctor(p, this.addr(), this.club_count() as u16 as u32, list, 1, this.competition().addr(), 0, 0, this.win_points() as u32, this.draw_points() as u32,
            this.match_category() as u32, this.addr() + league::TIEBREAKS, places.as_ptr() as usize, this.year() as u16 as u32, period, label, 0, 1, 0, 2, rounds, 0, 2)
    } else {
        0
    };
    let g = period as u8 as i8 as i32;
    write_usize(this.stages() + 4 * g as usize, s);
    this.set_current_stage(g as usize);
    exe::crt_free(list);
}

/// Comparer for `bel_second_create_period_stage`: only the original (it builds the stage); afterwards the period's slot
/// must hold a stage and be the current stage.
pub(crate) unsafe extern "thiscall" fn compare_bel_second_create_period_stage(this: usize, period: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I1%%)(this, period);
    let (l, g) = (League(this), period as u8 as i8 as i32);
    let s = read_usize(l.stages() + 4 * g as usize);
    SCOREBOARD.record(s != 0 && l.current_stage() as i32 == g, || format!("bel_second_create_period_stage({:#x}, {}): stage {:#x}, current {}", this, g, s, l.current_stage() as i32));
}

/// Rule: after the usual table update, the winner of the next period still undecided, once that period's stage is
/// reached and its table leader is marked champion (1), qualifies for the play-off: its row in the league table, if
/// still undecided, is marked 1, the news sent and its progress set; then that period counts as won.
///
/// `BelSecond::update_positions_and_status()`: League::update_positions_and_status (0x684cd0, through its detour); the
/// first period whose flag (`BEL_SECOND_PERIOD_WON` + 0..2) is 0 (none: nothing else); with `league::CURRENT_STAGE` at
/// or past it (period − 1) and the first row of that period stage's table with status 1, its club: each league table row
/// of that club with status 0xff gets 1, BelSecond::send_period_winner_news (0x420ca0, thiscall(club, period), ret 8)
/// and `belgian_progress_virtual`; then the flag of the first period still at 0 becomes 1.
/// Original: `BelSecond::update_positions_and_status` (0x420680, bel_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_second_update_positions_and_status(this: usize) {
    let l = League(this);
    let base: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684cd0usize);
    base(this);
    let Some(period) = (0..3).find(|&p| read_u8(this + BEL_SECOND_PERIOD_WON + p) == 0) else {
        return;
    };
    if (l.current_stage() as i32) < period as i32 {
        return;
    }
    let leader = LeagueTableRow(League(read_usize(l.stages() + 4 * period)).table());
    if leader.status() != 1 {
        return;
    }
    covered!("cx08_16 belgian period winner");
    let club = leader.club() as usize;
    let news: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x420ca0usize);
    let mut k = 0i32;
    while k < l.club_count() as i32 {
        let row = LeagueTableRow(l.table() + k as usize * league_table_row::SIZE);
        if row.club() as usize == club && row.status() == 0xff {
            row.set_status(1);
            news(this, row.club() as usize, period as u32 + 1);
            belgian_progress_virtual(l, row.club() as usize);
        }
        k += 1;
    }
    let first_open = (0..3).find(|&p| read_u8(this + BEL_SECOND_PERIOD_WON + p) == 0).unwrap_or(2);
    write_u8(this + BEL_SECOND_PERIOD_WON + first_open, 1);
}

/// Comparer for `bel_second_update_positions_and_status`: only the original (it updates the table and sends news).
pub(crate) unsafe extern "thiscall" fn compare_bel_second_update_positions_and_status(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}
