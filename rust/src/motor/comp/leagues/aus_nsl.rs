use super::*;

/// Club in row i of the table (+0xb1, 0x3b-byte rows) of league or stage t.
pub(crate) unsafe fn aus_nsl_table_row_club(t: League, i: usize) -> usize {
    read_usize(t.table() + i * league_table_row::SIZE)
}

/// The NSL has the regular season (round robin), the finals series (stage 0) and the grand final (stage 1): the next
/// stage moves the current stage on and builds the finals series at the end of the regular season and the grand final
/// at the end of the finals series; past the last stage it is an error (the box)
/// Original: `AusNsl::advance_stage` (0x411f70, aus_nsl.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn aus_nsl_advance_stage(this: usize) {
    let this = League(this);
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x987c20, 0x2f0);
    }
    let stage = stage + 1;
    this.set_current_stage((stage) as usize);
    if stage == 0 {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_AUSNSL_CREATE_FINALS_SERIES_STAGE);
        f(this.addr());
    } else if stage == 1 {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_AUSNSL_CREATE_GRAND_FINAL_STAGE);
        f(this.addr());
    }
}

/// The NSL finals series: the top six of the regular season play their own round-robin stage (with the league points
/// and tie-breakers) and its top two go to the grand final; sends the draw news.
///
/// `AusNsl::create_finals_series_stage()` (SEH frame only for the new), called at the end of the regular season. 1) The
/// top six of the league table (rows 0 to 5, in order, `aus_nsl_table_row_club`) in a local vector of 6. 2) The round
/// table through virtual `comp_vtable::CREATE_SCHEDULE` (create_round_table: thiscall(this, 0, &rounds, &extra, 0), ret
/// 0x10). 3) The stage: operator new `exe::crt_operator_new`(0xee) and the League_Stage constructor
/// `FN_LEAGUE_STAGE_LEAGUE_STAGE` (thiscall, ret 0x58, 22 arguments, in the original's order: the league, 6, the
/// vector, 1, `competition()`, the round table, the rounds (word), the points `win_points()` and `draw_points()`, the
/// category `match_category()`, &`league::TIEBREAKS`, &{2,0,0,0} (two top places: the two who go to the grand final),
/// the year `year()` (word), 0, the extra (word), `importance()`, 1, 0, `league_flags()` (word), −1, 0, 2; the original
/// pushes the words and bytes with garbage in the high bytes, which the constructor does not read: the copy passes
/// clean values). Out of memory: 0. 4) `stages()`[0] = the stage and the round table freed (free `exe::crt_free`). No
/// stage: the error box at line 0x328 and 0. With it: the draw news (`exe::league_send_group_draw_news` on the stage,
/// thiscall, ret 0), `set_current_stage(0)` and 1.
///
/// [inference] The finals series is a short league of the six; its top two go to the grand final. The real NSL used a
/// double-chance knockout.
/// Original: `AusNsl::create_finals_series_stage` (0x412010, aus_nsl.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn create_finals_series_stage(this: usize) -> u32 {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, *const usize, u32, usize, usize, u32, u32, u32, u32, usize, *const u8, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let slots: [u8; 4] = [2, 0, 0, 0];
    let mut clubs = [0usize; 6];
    for (i, c) in clubs.iter_mut().enumerate() {
        *c = aus_nsl_table_row_club(this, i); // cqr3: the top six of the table
    }
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 0, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        construct(p, this.addr(), 6, clubs.as_ptr(), 1, this.competition().addr(), t, rodadas as u16 as u32, this.win_points() as u32,
                  this.draw_points() as u32, this.match_category() as u32, this.addr() + league::TIEBREAKS, slots.as_ptr(), this.year() as u16 as u32, 0,
                  extra as u16 as u32, this.importance() as u32, 1, 0, this.league_flags() as u16 as u32, -1, 0, 2)
    } else {
        0
    };
    write_usize(this.stages(), stage);
    exe::crt_free(t);
    let stage = read_usize(this.stages());
    if stage == 0 {
        show_error_at(0x987c20, 0x328);
        return 0;
    }
    covered!("cqr_3 final series");
    exe::league_send_group_draw_news(stage);
    this.set_current_stage((0) as usize);
    1
}

/// The NSL grand final: the top two of the finals series in a two-club cup stage; the winner is the Australian
/// champion.
///
/// `AusNsl::create_grand_final_stage()` (SEH frame only for the new), called at the end of the finals series: the top
/// two of the finals series table (`stages()`[0], rows 0 and 1, in that order, `aus_nsl_table_row_club`) in a list of 2
/// (malloc `exe::crt_malloc` of 8; out of memory: the error box at line 0x348 and return); the round table through
/// virtual `comp_vtable::CREATE_SCHEDULE` (thiscall(this, 1, &rounds, &extra, 0), ret 0x10); the stage (operator new
/// `exe::crt_operator_new`(0xb2) and the Cup_Stage constructor `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret 0x3c: the
/// league, 2, the list, the rounds, `competition()`, the table, the year `year()`, 1, 1, the extra, 0x14, 0, 0, 0 and
/// null seeds; the copy passes clean words) into `stages()`[1] and the table freed (free `exe::crt_free`). No stage:
/// the error box at line 0x35c and return (without freeing the list). With it: Cup::set_up_round
/// (`exe::cup_set_up_round`, thiscall(stage, 0), ret 4) and the list freed. The current stage (+0x30 = 1) was already
/// written by the next stage.
///
/// [inference] The grand final is a single game, with the finals series winner as the home side.
/// Original: `AusNsl::create_grand_final_stage` (0x4121c0, aus_nsl.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn create_grand_final_stage(this: usize) {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(0x987c20, 0x348);
    }
    let series = read_usize(this.stages());
    write_usize(list, aus_nsl_table_row_club(League(series), 0)); // cqr3: the first of the final series
    write_usize(list + 4, aus_nsl_table_row_club(League(series), 1)); // cqr3: the second of the final series
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 1, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        construct(p, this.addr(), 2, list, rodadas as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 1, 1,
                  extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(this.stages() + 4, stage);
    exe::crt_free(t);
    let stage = read_usize(this.stages() + 4);
    if stage == 0 {
        return show_error_at(0x987c20, 0x35c);
    }
    covered!("cqr_3 grand final");
    exe::cup_set_up_round(stage, 0);
    exe::crt_free(list);
}

/// Writes the NSL rule fields (3 rounds, points, tiebreaks, 6 finalists...) before the schedule.
pub(crate) unsafe fn set_nsl_rules(o: usize) {
    write_u16_at(o, 0xab, 0);
    write_u8_at(o, 0xc8, 0);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 0);
    write_u8_at(o, 0xc4, 2);
    write_u8_at(o, 0xc6, 2);
    write_u8_at(o, 0x52, 2);
    write_u16_at(o, 0x3c, 3); // three shifts
    write_u8_at(o, 0xc2, 3); // cqj8: the NSL points per win
    write_u8_at(o, 0x4a, 3);
    write_u8_at(o, 0xc3, 1);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc5, 1); // tiebreakers: goal difference, goals for, away goals
    write_u8_at(o, 0xc7, 4);
    write_u8_at(o, 0xbe, 6); // the six of the final stage
    write_u32_at(o, 0x1c, 0xffff_ffff);
    write_u32_at(o, 0x20, 0xffff_ffff);
    write_u16_at(o, 0xd9, 0x28); // 0x8 fase seguinte, 0x20 eliminados marcados
    write_u8_at(o, 0x49, 4);
}

/// Writes the MLS rule fields (no schedule; 32 games, fixed round order...).
pub(crate) unsafe fn set_mls_rules(o: usize) {
    write_u8_at(o, 0xc4, 2);
    write_u8_at(o, 0xc6, 2);
    write_u8_at(o, 0xc2, 3);
    write_u8_at(o, 0xc8, 0);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 0);
    write_u8_at(o, 0xc3, 1); // cqj8: the MLS points per draw
    write_u16_at(o, 0xab, 0xc2);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc5, 1); // tiebreakers: goal difference, goals scored, head-to-head
    write_u8_at(o, 0xc7, 3);
    write_u8_at(o, 0xbe, 1);
    write_u32_at(o, 0x1c, 0xffff_ffff);
    write_u32_at(o, 0x20, 0xffff_ffff);
    write_u16_at(o, 0xd9, 0x48); // 0x8 next stage, 0x40 rounds in fixed order
    write_u16_at(o, 0x3c, 4);
    write_u16_at(o, 0xa7, 0x20); // 32 jogos por time
}

/// NSL: three round robins; win 3, draw 1; tie-break by goal difference, goals scored and away goals; the top 6 go to
/// the finals series (rule word 0x28: 0x8 next stage, no champion band, and 0x20, clubs without a chance marked
/// eliminated); nobody goes up or down; 4 bench substitutes and 3 substitutions; fixed squad numbers. Then the schedule
/// Original: `AusNsl::set_up_rules` (0x411ce0, aus_nsl.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn aus_nsl_set_up_rules(this: usize) -> u32 {
    let this = League(this);
    set_nsl_rules(this.addr());
    covered!("cqj_8 nsl rules");
    let vt = read_usize(this.addr());
    let f: unsafe extern "thiscall" fn(usize, i32, usize, usize, usize) -> usize = core::mem::transmute(read_usize(vt + 0x3c));
    let r = f(this.addr(), -1, this.addr() + league::ROUND_COUNT, this.addr() + league::STAGE_ID, 0);
    write_u32_at(this.addr(), 0xba, r as u32);
    1
}

/// Calls record_season_honours (vtable +0x30) on the first stage [[this+0xc]].
pub(crate) unsafe fn record_first_stage_honours(this: League) -> u32 {
    let e = read_usize(this.stages());
    let vt = read_usize(e);
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(vt + 0x30));
    f(e)
}

/// NSL: at the end of the regular season, the six clubs through to the finals series get the qualification record
/// (stage 0x435, weight 0x28); in the finals, those who go through get the record (round 0x96) and those who fall the
/// failed-to-qualify one; the champion (grand final) gets the champion record. No fate written to the table
/// Original: `AusNsl::update_staff_records_for_status` (0x411980, aus_nsl.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn aus_nsl_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let _ = (a4, a5, a6);
    nsl_end_of_stage(this, club, status, stage, true).0
}

/// `AusNsl::create_table()`: the Australian NSL has 13 clubs (`is_in_division`: +0x57 =
/// [`KEY_CLUB_COMP_AUSTRALIAN_NATIONAL_SOCCER_LEAGUE`]), those of the division in database order, until it is full (the
/// counter in an int, compared as a word).
///
/// Not full: the error box at line 0x252 and 0; full: 1, with no prize fund nor TV money. Source 0x987c20 (the
/// out-of-memory box at line 0x241, `alloc_league_table`).
/// Original: `AusNsl::create_table` (0x411b70, aus_nsl.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn aus_nsl_create_table(this: usize) -> u32 {
    let this = League(this);
    if alloc_league_table(this, 0xd, 0x987c20, 0x241) == 0 {
        return 0;
    }
    let mut k: u32 = 0;
    let mut i: i16 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if (k as i16) >= this.club_count() {
            break;
        }
        if is_in_division(Club(club), KEY_CLUB_COMP_AUSTRALIAN_NATIONAL_SOCCER_LEAGUE) {
            jap_j1_init_table_row(this, k as u8, this.club_count(), 0, club); // cqm9: the NSL row
            k = k.wrapping_add(1);
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    if k as i16 != this.club_count() {
        show_error_at(0x987c20, 0x252);
        return 0;
    }
    covered!("cqm_9 nsl full");
    1
}

/// The NSL honours: the grand final winner as champion, its runner-up second and the regular season leader third in
/// the competition history.
///
/// The grand final is stage 1 (`.stages()`[1], a Cup_Stage): in its team table (`cup::TEAM_TABLE`, 6-byte entries,
/// `.team_count()` of them, a signed word) the club with status (+5) 1 is the winner and the one with status 2 the
/// runner-up (the last of each, 0 when none). The history row (`exe::comp_util_add_history_record`: push 0; push
/// leader; push runner-up; push winner; push this; add esp, 0x14 -> cdecl) takes the winner, the runner-up, the club
/// in the first row of the league table (`.table()`) and 0; its eax is returned.
/// Original: `AusNsl::record_season_honours` (0x411d90, aus_nsl.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn aus_nsl_record_season_honours(this: usize) -> usize {
    let this = League(this);
    let fin = Cup(read_usize(this.stages() + 4));
    let (mut winner, mut runner_up) = (0usize, 0usize);
    let mut k = 0i16;
    while k < fin.team_count() {
        let e = fin.team_table() + k as usize * 6;
        let status = read_u8(e + 5);
        if status == 1 {
            winner = read_usize(e);
        }
        if status == 2 {
            runner_up = read_usize(e);
        }
        k += 1;
    }
    covered!("mwf_12 nsl honours");
    exe::comp_util_add_history_record(this, winner, runner_up, read_usize(this.table()), 0) // mwf12: the NSL champion, runner-up and leader
}

/// Comparer for `aus_nsl_record_season_honours`: side by side over the competition history (count, pointer and rows),
/// when there is a free row; when the history has to grow (realloc), only the original.
pub(crate) unsafe extern "thiscall" fn compare_aus_nsl_record_season_honours(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_AUS_NSL_RECORD_SEASON_HONOURS);
    if free_comp_history_row() == 0 {
        return o(this); // simplification: the history grows, original only
    }
    let reg = comp_history_regions();
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || aus_nsl_record_season_honours(this));
    SCOREBOARD.record(same && r == n, || format!("aus_nsl_record_season_honours({:x}) {:x}/{:x} {:x?} x {:x?}", this, r, n, mo, mr));
    r
}

/// The NSL season turnover: the season archived, the table and the dates cleared, the year plus one, then the rules,
/// the 13 Australian clubs, the fixtures and the order rebuilt; then the predicted positions, the board expectations
/// and the referee renewal. Unlike the sisters it neither destroys the finals stages nor resets
/// `league::CURRENT_STAGE` [inference: the NSL rules rebuild the stages].
///
/// `league::SEASON_CLOSED` = 0; the table freed (League::free_table `exe::league_free_table`, thiscall(this, 0), ret 4);
/// `.round_dates()` freed (`exe::crt_free`) and zeroed; with statistics, Comp_Stats::archive_season_and_reset
/// (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the year up by one. Then, each with its own
/// box (aus_nsl.cpp, lines 0x2bd, 0x2c4, 0x2ca and 0x2d1) when it returns 0: the rules (virtual
/// `comp_vtable::SET_UP_RULES`), the table (AusNsl::create_table 0x411b70, thiscall, ret 0: the 13 clubs of Australia),
/// the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table
/// `exe::league_sort_table`, thiscall(this, 0), ret 4); then the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`) and `set_expectations_and_renew_officials`, whose eax (the referee
/// renewal's) it returns. After a box the copy returns 0 (the original returns the box's eax; the turnover's caller
/// does not read it).
/// Original: `AusNsl::start_new_season` (0x411de0, aus_nsl.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn aus_nsl_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
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
    let a = this.year();
    this.set_year(a.wrapping_add(1)); // mwf12: the year of the NSL
    covered!("mwf_12 nsl rollover");
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(0x987c20, 0x2bd);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_AUSNSL_CREATE_TABLE);
    if table(this.addr()) == 0 {
        show_error_at(0x987c20, 0x2c4);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(0x987c20, 0x2ca);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(0x987c20, 0x2d1);
        return 0;
    }
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// Comparer for `aus_nsl_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go
/// up by one.
pub(crate) unsafe extern "thiscall" fn compare_aus_nsl_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_AUS_NSL_START_NEW_SEASON);
    compare_state_league_new_season("aus_nsl_start_new_season", League(this), || o(this))
}

/// The schedule of the Australian NSL: stage −1, the round-robin league stage: 24 rounds from 5 Sep to 15 Feb of the following year, stage id 0x434; stage 0, a second league stage: 10 rounds from 29 Feb of the following year to 29 May of the following year, stage id 0x435; any other stage, the final: 1 round from 1 Jun of the following year to 7 Jun of the following year, stage id 0x96.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures and the season turns. For a stage with a table (`nsl_schedule_stage`) it writes the outputs (is_league
/// when that pointer is not null, the rounds and the extra), allocates the round table (`exe::crt_malloc`, 0x41 bytes
/// per round; out of memory, the error box and 0) and writes the date (day, month, years after the season year
/// `year()`, weekday and code) and the matches of each round, in the original's order (`create_round_table`, the
/// cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table, or 0 without writing
/// anything for a stage without one.
/// Original: `AusNsl::create_schedule` (0x4111a0, aus_nsl.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn aus_nsl_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), nsl_schedule_stage(stage), rounds, extra, league, 0x987c20)
}

/// Comparer for `aus_nsl_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_aus_nsl_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_AUS_NSL_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("aus_nsl_create_schedule", stage, nsl_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| aus_nsl_create_schedule(this, stage, r, e, l))
}

/// The Australian NSL at career creation: the year and the database competition, discipline rule 0x22
/// (`discipline_rule::AUSTRALIA`) and importance 10 (written before loading the saved state) and, with no saved state,
/// the finals series and the grand final (two stages besides the league), the rules (among them the substitute
/// counts [inference: map]), the table (the NSL clubs), the fixtures, the order, the statistics and the predicted
/// positions by club strength (`bra_nat_first_construct_league` with `AUS_NSL_LEAGUE`).
/// Original: `AusNsl::AusNsl` (0x410e90, aus_nsl.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_aus_nsl(this: usize, year: u32, record: usize) -> usize {
    bra_nat_first_construct_league(League(this), year, record, &AUS_NSL_LEAGUE, discipline_rule::AUSTRALIA, "cqu_1 aus_nsl new") // cqu1: the discipline of the NSL
}

/// Comparer for `construct_aus_nsl`.
pub(crate) unsafe extern "thiscall" fn compare_construct_aus_nsl(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_AUS_NSL);
    bra_nat_first_compare_league_ctor("construct_aus_nsl", League(this), year, record, &AUS_NSL_LEAGUE, discipline_rule::AUSTRALIA, || o(this, year, record))
}

/// The stages of the round table of the Australian NSL (virtual `comp_vtable::CREATE_SCHEDULE`): stage −1, the round-robin league stage: 24 rounds from 5 Sep to 15 Feb of the following year, stage id 0x434; stage 0, a second league stage: 10 rounds from 29 Feb of the following year to 29 May of the following year, stage id 0x435; any other stage, the final: 1 round from 1 Jun of the following year to 7 Jun of the following year, stage id 0x96.
pub(crate) fn nsl_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 24, extra: 0x434, extra_check: false, size: 0x618, row: 0xa7,
            ops: &NSL_SCHEDULE_LEAGUE_OPS, name: "cqw_4 nsl league" }),
        0 => Some(ScheduleStage { league: 1, rounds: 10, extra: 0x435, extra_check: false, size: 0x28a, row: 0x123,
            ops: &NSL_SCHEDULE_STAGE_0_OPS, name: "cqw_4 nsl league" }),
        _ => Some(ScheduleStage { league: 0, rounds: 1, extra: 0x96, extra_check: false, size: 0x68, row: 0x164,
            ops: &NSL_SCHEDULE_OTHER_STAGES_OPS, name: "cqw_4 nsl other stages" }),
    }
}

/// The constructor of a national league (the same 544-byte template as the state leagues, checked instruction by
/// instruction: only the vtable, the discipline rule, the importance and when it is written, the stages, the rules, the
/// table, the source file and the lines change; the predicted positions are League's).
///
/// League::League (`exe::new_league`, thiscall, ret 0); `league::COMPETITION` = the competition record; the vtable; the
/// year (`league::YEAR`, word) = `year`; the discipline rule (`league::DISCIPLINE_RULE`) = `discipline` (and, in the NSL,
/// the importance `league::IMPORTANCE`). If the saved state loads (League::load_from_temp_file
/// `exe::league_load_from_temp_file`, thiscall(this, 1), ret 4; returns ≠ 0), returns this.
///
/// Otherwise (new career): the importance (outside the NSL), `league::STAGE_INDEX` = `stage_index::MAIN` (0xff),
/// `league::CURRENT_STAGE` = −1, `league::STAGE_COUNT` = the stages and their array `league::STAGES` (malloc
/// `exe::crt_malloc`, unchecked); then, each with its own error box (returning this) if it returns 0: the rules, the
/// table, the fixtures (League::create_fixtures `exe::league_create_fixtures`, thiscall, ret 0) and the order
/// (League::sort_table `exe::league_sort_table`, thiscall(this, 0), ret 4); the statistics (operator new
/// `exe::crt_operator_new` of 0x5ce and Comp_Stats::Comp_Stats `exe::new_comp_stats`, thiscall(new, this), ret 4; the
/// original's SEH frame covers only that new) at `league::COMP_STATS` (without them: the error box); finally the
/// predicted positions (League::set_predicted_positions `exe::league_set_predicted_positions`, thiscall, ret 0).
pub(crate) unsafe fn bra_nat_first_construct_league(this: League, year: u32, record: usize, e: &BraNatFirstNationalLeague, discipline: u8, branch: &'static str) -> usize {
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.rules);
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.table);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), e.vtable);
    write_u16(this.addr() + league::YEAR, year as u16);
    this.set_discipline_rule(discipline);
    if e.importance_before_load {
        this.set_importance(e.importance);
    }
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!(branch);
    if !e.importance_before_load {
        this.set_importance(e.importance);
    }
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    write_u32(this.addr() + league::STAGE_COUNT, e.stages);
    let v = exe::crt_malloc(e.stages * 4);
    this.set_stages(v);
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
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(e.file, e.rows[3]);
        return this.addr();
    }
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(e.file, e.rows[4]);
        return this.addr();
    }
    exe::league_set_predicted_positions(this);
    this.addr()
}

/// Comparer of the four national league constructors: they allocate and draw the fixtures, so only the original runs,
/// and what it writes is checked: the return value, the vtable, the year, the record and the discipline rule; the
/// importance when it is always written (the NSL); on a new league (`stage_index::MAIN`, with the statistics), the
/// stage count, the stage array and the importance.
pub(crate) unsafe fn bra_nat_first_compare_league_ctor(name: &str, this: League, year: u32, record: usize, e: &BraNatFirstNationalLeague, discipline: u8, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == e.vtable && built_year_ok(this.year() as u16, year as u16)
        && this.competition().addr() == record && this.discipline_rule() == discipline;
    if e.importance_before_load {
        ok = ok && this.importance() == e.importance;
    }
    if this.stage_index() == stage_index::MAIN && this.comp_stats() != 0 {
        ok = ok && this.stage_count() == e.stages as usize && this.stages() != 0 && this.importance() == e.importance;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}): returned {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}, +0x51 = {}", name, this.addr(), r, read_usize(this.addr()), this.stage_count(), this.stages(), this.comp_stats(), this.importance()));
    r
}

/// The Australian NSL, every stage other than −1 and 0 (the final): 1 round (1 Jun of the following year to 7 Jun of the following year), 104 bytes, 19 operations.
pub(crate) const NSL_SCHEDULE_OTHER_STAGES_OPS: [AsiaClubChampScheduleOp; 19] = [
    Cqp11S(0, 1, 6, 1, 0), Cqp11J(0, 7, 6, 1, 5, 1, 0), Cqp11W(0x7, 0), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x83), Cqp11B(0x17, 5), Cqp11W(0x18, 2), Cqp11W(0x1a, 1), Cqp11W(0x1c, 2), Cqp11W(0xf, 0),
    Cqp11W(0x11, 0), Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 1), Cqp11B(0x22, 0), Cqp11X(0x5c, 0),
    Cqp11X(0x60, 0x2ee0), Cqp11X(0x64, 0x1388),
];

/// The Australian NSL, the round-robin stage (−1): 24 rounds (5 Sep to 15 Feb of the following year), 1560 bytes, 48 operations.
pub(crate) const NSL_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 48] = [
    Cqp11D(0, 5, 9, 0, 5, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0xc, 9, 0, 5, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0x13, 9, 0, 5, 1, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 0x1a, 9, 0, 5, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 2, 0xa, 0, 5, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 9, 0xa, 0, 5, 1, 0), Cqp11L(5, 0, -1, -1, -1, 0),
    Cqp11D(6, 0x10, 0xa, 0, 2, 2, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0x17, 0xa, 0, 5, 1, 0),
    Cqp11L(7, 0, -1, -1, -1, 0), Cqp11D(8, 0x1e, 0xa, 0, 5, 1, 0), Cqp11L(8, 0, -1, -1, -1, 0),
    Cqp11D(9, 7, 0xb, 0, 5, 1, 0), Cqp11L(9, 0, -1, -1, -1, 0), Cqp11D(0xa, 0xe, 0xb, 0, 5, 1, 0),
    Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 0x15, 0xb, 0, 5, 1, 0), Cqp11L(0xb, 0, -1, -1, -1, 0),
    Cqp11D(0xc, 0x1c, 0xb, 0, 5, 1, 0), Cqp11L(0xc, 0, -1, -1, -1, 0), Cqp11D(0xd, 4, 0, 1, 5, 1, 0),
    Cqp11L(0xd, 0, -1, -1, -1, 0), Cqp11D(0xe, 0xb, 0, 1, 5, 1, 0), Cqp11L(0xe, 0, -1, -1, -1, 0),
    Cqp11D(0xf, 0x12, 0, 1, 5, 1, 0), Cqp11L(0xf, 0, -1, -1, -1, 0), Cqp11D(0x10, 0x19, 0, 1, 5, 1, 0),
    Cqp11L(0x10, 0, -1, -1, -1, 0), Cqp11D(0x11, 1, 1, 1, 5, 1, 0), Cqp11L(0x11, 0, -1, -1, -1, 0),
    Cqp11D(0x12, 8, 1, 1, 5, 1, 0), Cqp11L(0x12, 0, -1, -1, -1, 0), Cqp11D(0x13, 0xf, 1, 1, 5, 1, 0),
    Cqp11L(0x13, 0, -1, -1, -1, 0), Cqp11D(0x14, 0x16, 1, 1, 5, 1, 0), Cqp11L(0x14, 0, -1, -1, -1, 0),
    Cqp11D(0x15, 1, 2, 1, 5, 1, 0), Cqp11L(0x15, 0, -1, -1, -1, 0), Cqp11D(0x16, 8, 2, 1, 5, 1, 0),
    Cqp11L(0x16, 0, -1, -1, -1, 0), Cqp11D(0x17, 0xf, 2, 1, 5, 1, 0), Cqp11L(0x17, 0, -1, -1, -1, 0),
];

/// The Australian NSL, stage 0 (league): 10 rounds (29 Feb of the following year to 29 May of the following year), 650 bytes, 20 operations.
pub(crate) const NSL_SCHEDULE_STAGE_0_OPS: [AsiaClubChampScheduleOp; 20] = [
    Cqp11D(0, 0x1d, 2, 1, 5, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 5, 3, 1, 5, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0xc, 3, 1, 5, 1, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 0x11, 3, 1, 5, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 0x18, 3, 1, 5, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 1, 5, 1, 5, 1, 0), Cqp11L(5, 0, -1, -1, -1, 0),
    Cqp11D(6, 8, 5, 1, 5, 1, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0xf, 5, 1, 5, 1, 0),
    Cqp11L(7, 0, -1, -1, -1, 0), Cqp11D(8, 0x16, 5, 1, 5, 1, 0), Cqp11L(8, 0, -1, -1, -1, 0),
    Cqp11D(9, 0x1d, 5, 1, 5, 1, 0), Cqp11L(9, 0, -1, -1, -1, 0),
];

pub(crate) const AUS_NSL_LEAGUE: BraNatFirstNationalLeague = BraNatFirstNationalLeague {
    vtable: 0x967768,
    importance: 0xa,
    importance_before_load: true,
    stages: 2,
    rules: 0x411ce0,
    table: 0x411b70,
    file: 0x987c20,
    rows: [0x46, 0x4d, 0x53, 0x5a, 0x61],
};

/// Rule: the news of a club's fate in the Australian league: through from the regular season, "{club} through to
/// playoff phase"; through from the finals series (stage 0), "{club} through to final"; in the grand final, the cup
/// lines of a tie.
///
/// `AusNsl::build_fate_news_text(club, outcome, stage, a4, fixture, a6, a7, a8, body, text)`: at the stage byte 0xff
/// or 0 only outcome 6 writes: with `body` the group news (`exe::comp_util_write_group_qualify_news`(club, a4, the
/// competition record, 6, body, text)), without it `group_qualify_text` with 0x987c88 (0xff) or 0x987c58 (0). Any other
/// stage: with `body` `exe::comp_util_write_cup_tie_result_news`(club, fixture, a8, outcome, a6, text), without it
/// `exe::comp_util_write_cup_win_or_defeat_news`(club, fixture, outcome, text).
/// Original: `AusNsl::build_fate_news_text` (0x411a30, aus_nsl.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn aus_nsl_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, _a7: u32, a8: u32, body: u32, text: usize) {
    let this = League(this);
    let format = match stage as u8 {
        0xff => 0x987c88, // "{club} through to playoff phase"
        0 => 0x987c58, // "{club} through to final"
        _ => {
            if body != 0 {
                return exe::comp_util_write_cup_tie_result_news(club_id, fixture as u32, a8, outcome, a6, text);
            }
            return exe::comp_util_write_cup_win_or_defeat_news(club_id, fixture as u32, outcome, text);
        }
    };
    if outcome as u8 != 6 {
        return;
    }
    if body != 0 {
        return exe::comp_util_write_group_qualify_news(club_id, a4, this.competition().addr(), 6, body, text);
    }
    covered!("cx08_13 australian qualify text");
    group_qualify_text(club_id, format, text);
}

/// Comparer for `aus_nsl_build_fate_news_text`: the text-only paths (outcome 6 without `body` at stage −1 or 0) side by
/// side (`compare_text_writer`); the news writers only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_aus_nsl_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a7: u32, a8: u32, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(IDX_AUS_NSL_BUILD_FATE_NEWS_TEXT);
    if text == 0 || body != 0 || outcome as u8 != 6 || !matches!(stage as u8, 0xff | 0) {
        return o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, body, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, body, text),
        |t| aus_nsl_build_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, body, t),
        || format!("aus_nsl_build_fate_news_text(club {}, outcome {}, stage {})", club_id, outcome as u8, stage as u8 as i8));
}

/// Rule: the Australian league's progress codes: in the regular season the top two become 1 (best and current) or 7
/// (worst) and third 9; in the finals series (stages below 1) places 1 and 2 become 1 (current, best) or 3 (worst), 3
/// becomes 5 and 4 becomes 7; in the grand final as they come.
///
/// `AusNsl::set_club_progress(club, stage, current, best, worst)`: the club's entry (`exe::comp_stats_find_club_entry`,
/// unchecked). Stage byte 0xff: current 1, 2 → 1 and 3 → 9; best 1, 2 → 1 and 3 → 9; worst 1, 2 → 7 and 3 → 9. Other
/// stages below 1 (signed; the jump tables 0x4124ac, 0x4124bc, 0x4124cc): current 1, 2 → 1, 3 → 5, 4 → 7; best the same;
/// worst 1, 2 → 3, 3 → 5, 4 → 7. Any other code, and every code from stage 1, stays. The bytes go to
/// `comp_stats_club::BEST_POSITION`, `CURRENT_POSITION` and `WORST_POSITION`.
/// Original: `AusNsl::set_club_progress` (0x412380, aus_nsl.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn aus_nsl_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let e = exe::comp_stats_find_club_entry(League(this).comp_stats(), club);
    let s = stage as u8;
    let code = |v: u32, table: [u8; 4]| -> u8 {
        let v = v as u8;
        if (1..=4).contains(&v) && table[v as usize - 1] != 0 { table[v as usize - 1] } else { v }
    };
    let (c, b, w) = if s == 0xff {
        covered!("cx08_13 australian season progress");
        (code(current, [1, 1, 9, 0]), code(best, [1, 1, 9, 0]), code(worst, [7, 7, 9, 0]))
    } else if (s as i8) < 1 {
        covered!("cx08_13 australian finals progress");
        (code(current, [1, 1, 5, 7]), code(best, [1, 1, 5, 7]), code(worst, [3, 3, 5, 7]))
    } else {
        (current as u8, best as u8, worst as u8)
    };
    write_u8(e + comp_stats_club::BEST_POSITION, b);
    write_u8(e + comp_stats_club::CURRENT_POSITION, c);
    write_u8(e + comp_stats_club::WORST_POSITION, w);
}

/// Comparer for `aus_nsl_set_club_progress`: side by side on the 3 bytes (`compare_group_championship_progress`).
pub(crate) unsafe extern "thiscall" fn compare_aus_nsl_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_AUS_NSL_SET_CLUB_PROGRESS);
    compare_group_championship_progress("aus_nsl_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || aus_nsl_set_club_progress(this, club, stage, current, best, worst));
}

/// Rule: the Australian league's bench: a fixture of a stage (the finals series and the grand final) names 5
/// substitutes and allows 3; a league fixture takes the league's numbers.
///
/// `AusNsl::set_fixture_substitutes(fixture)`: with the fixture's `fixture::STAGE_INDEX` (signed) at 0 or above, the
/// low byte of `fixture::SQUAD_RULES` becomes 0x35 (the word & 0xff35 | 0x35); otherwise the league's
/// `league::SUBSTITUTES_NAMED` (& 0xf) into the low nibble and `league::SUBSTITUTIONS_ALLOWED` (& 0xf) into the next,
/// the other bits kept.
/// Original: `AusNsl::set_fixture_substitutes` (0x4124e0, aus_nsl.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn aus_nsl_set_fixture_substitutes(this: usize, fixture: usize) {
    let (l, fixture) = (League(this), Fixture(fixture));
    if fixture.stage_index() as i8 >= 0 {
        covered!("cx08_13 australian stage bench");
        return fixture.set_squad_rules((fixture.squad_rules() & 0xff35) | 0x35);
    }
    fixture.set_squad_rules((fixture.squad_rules() & 0xfff0) | (l.substitutes_named() & 0xf) as u16);
    fixture.set_squad_rules((fixture.squad_rules() & 0xff0f) | ((l.substitutions_allowed() & 0xf) as u16) << 4);
}

/// Comparer for `aus_nsl_set_fixture_substitutes`: the fixture's `SQUAD_RULES` word side by side.
pub(crate) unsafe extern "thiscall" fn compare_aus_nsl_set_fixture_substitutes(this: usize, fixture: usize) {
    let fixture = Fixture(fixture);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_AUS_NSL_SET_FIXTURE_SUBSTITUTES);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(fixture.addr() + fixture::SQUAD_RULES, 2)], || o(this, fixture.addr()), || aus_nsl_set_fixture_substitutes(this, fixture.addr()));
    SCOREBOARD.record(same, || format!("aus_nsl_set_fixture_substitutes({:#x}): original {:x?} rust {:x?}", fixture.addr(), mo, mr));
}
