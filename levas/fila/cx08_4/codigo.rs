// ---------- cx08_4: the Argentine Premier Division turnover, results and season honours (arg_prm.cpp) ----------

/// The virtual slot of the Argentine leagues that brings the relegation averages up to date (ArgPrm and ArgSecond
/// update_relegation_averages, thiscall(roll), ret 4; 1 also rolls the seasons) [inference: the averages screen calls it
/// with 0 before drawing].
pub(crate) const UPDATE_RELEGATION_AVERAGES_SLOT: usize = 0x34;

/// Calls a league's update_relegation_averages (virtual `UPDATE_RELEGATION_AVERAGES_SLOT`, push `roll` -> thiscall).
pub(crate) unsafe fn update_relegation_averages_virtual(this: League, roll: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(read_usize(read_usize(this.addr()) + UPDATE_RELEGATION_AVERAGES_SLOT));
    f(this.addr(), roll);
}

/// The relegation averages table of the Argentine leagues: a pointer at +0x18 to rows of 0x13 bytes, as many as the
/// word at +0x34 (`comp_compare_average_rows` reads the rows).
pub(crate) const ARG_AVERAGES: usize = 0x18;
pub(crate) const ARG_AVERAGE_COUNT: usize = 0x34;
pub(crate) const ARG_AVERAGE_ROW_SIZE: usize = 0x13;

/// The club id of averages row `k` (−1 = an empty row).
pub(crate) unsafe fn arg_average_row_club(this: League, k: i32) -> i32 {
    read_i32(read_usize(this.addr() + ARG_AVERAGES) + (k as usize).wrapping_mul(ARG_AVERAGE_ROW_SIZE))
}

/// Rule: the Argentine Premier Division turnover. The averages roll a season; promotion and relegation with the Second
/// Division and the swap between the Second and Third Divisions; the division tier of the clubs of this league, of its
/// first stage and of the Second Division capped (1, 3, −3, 1); the season reopened; the table, the round dates, the
/// statistics' season and the stages cleared; the year + 1 and the current stage −1; the rules, the table, the fixtures
/// and the order rebuilt; the two half-season stages; the Second Division's own turnover; the predicted positions, the
/// board expectations and the officials.
///
/// `ArgPrm::start_new_season()`: `update_relegation_averages_virtual`(1); the chain (virtual
/// `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(1)); arg_prm_swap_second_and_third_division_clubs (0x4068d0,
/// thiscall, ret 0); League::limit_club_division_tier (`exe::league_limit_club_division_tier`(1, 3, −3, 1)) on this, on
/// stage 0 and on `KEY_CLUB_COMP_ARGENTINE_SECOND_DIVISION`'s object; `league::SEASON_CLOSED` = 0; the table freed
/// (`exe::league_free_table` with 0), the round dates freed and cleared, with statistics
/// `exe::comp_stats_archive_season_and_reset`(1), the stages 0..=`league::CURRENT_STAGE` destroyed (virtual 0 with 1;
/// none when it is negative); the year + 1, the current stage −1; then, each with the arg_prm.cpp box of its line when it
/// returns 0 (and 0 returned): the rules (virtual `comp_vtable::SET_UP_RULES`, 0x263), ArgPrm::create_table (0x405540,
/// 0x26a), `exe::league_create_fixtures` (0x270) and `exe::league_sort_table`(0) (0x277); then ArgPrm::create_stages
/// (0x405950), the Second Division's virtual `comp_vtable::START_NEW_SEASON` (no null check), the predicted positions
/// (virtual `comp_vtable::SET_PREDICTED_POSITIONS`) and `set_expectations_and_renew_officials`, whose eax is returned.
/// After a box the copy returns 0 (the original returns the box's eax; the caller does not read it).
/// Original: `ArgPrm::start_new_season` (0x405720, arg_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn arg_prm_start_new_season(this: usize) -> u32 {
    let this = League(this);
    update_relegation_averages_virtual(this, 1);
    promotion_relegation_chain(this, 1);
    call_on_league(0x4068d0, this);
    let second = comp_by_global(KEY_CLUB_COMP_ARGENTINE_SECOND_DIVISION);
    for l in [this.addr(), read_usize(this.stages()), second] {
        exe::league_limit_club_division_tier(l, 1, 3, -3, 1);
    }
    this.set_season_closed(0);
    exe::league_free_table(this, 0);
    if this.round_dates() != 0 {
        exe::crt_free(this.round_dates());
        this.set_round_dates(0);
    }
    if this.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(this.comp_stats(), 1);
    }
    let mut k = 0i32;
    while k <= this.current_stage() as i32 {
        let stage = read_usize(this.stages() + 4 * k as usize);
        if stage != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(stage) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
            destroy(stage, 1);
        }
        k += 1;
    }
    this.set_year(this.year().wrapping_add(1)); // cx08_4: the Argentine season moves on a year
    this.set_current_stage(0xffff_ffff);
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(ARG_PRM_FILE, 0x263);
        return 0;
    }
    if argentine_league_call(this, 0x405540) == 0 {
        show_error_at(ARG_PRM_FILE, 0x26a);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(ARG_PRM_FILE, 0x270);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(ARG_PRM_FILE, 0x277);
        return 0;
    }
    covered!("cx08_4 argentine premier turnover");
    call_on_league(0x405950, this);
    bra_nat_first_call_virtual_no_args(second, comp_vtable::START_NEW_SEASON);
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// Comparer for `arg_prm_start_new_season`: only the original runs (it moves clubs and rebuilds); the year must go up
/// by one.
pub(crate) unsafe extern "thiscall" fn compare_arg_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("arg_prm_start_new_season", League(this), || o(this))
}

/// Rule: an Argentine Premier Division result counts in the league table and again in the overall half-season stage
/// (stage 1), without the statistics.
///
/// `ArgPrm::add_result(fixture, arg)`: League::add_result (`exe::league_add_result`(this, fixture, arg)); then, with the
/// statistics (`league::COMP_STATS`) set aside and the fixture's stage (`fixture::STAGE_INDEX`) at 1, League::add_result
/// again; the fixture's stage and the statistics are put back. Returns 1.
/// Original: `ArgPrm::add_result` (0x405c50, arg_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn arg_prm_add_result(this: usize, fixture: usize, arg: u32) -> u32 {
    let this = League(this);
    let fixture = Fixture(fixture);
    exe::league_add_result(this, fixture, arg);
    let stats = this.comp_stats();
    this.set_comp_stats(0);
    let stage = fixture.stage_index();
    fixture.set_stage_index(1); // cx08_4: the result also goes to the overall stage
    covered!("cx08_4 argentine result in both tables");
    exe::league_add_result(this, fixture, arg);
    fixture.set_stage_index(stage);
    this.set_comp_stats(stats);
    1
}

/// Comparer for `arg_prm_add_result`: only the original runs (it adds the result twice); afterwards the fixture's stage
/// and the league's statistics must be as they were and the return 1.
pub(crate) unsafe extern "thiscall" fn compare_arg_prm_add_result(this: usize, fixture: usize, arg: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%);
    let (l, f) = (League(this), Fixture(fixture));
    let (stage, stats) = (f.stage_index(), l.comp_stats());
    let r = o(this, fixture, arg);
    SCOREBOARD.record(r == 1 && f.stage_index() == stage && l.comp_stats() == stats,
        || format!("arg_prm_add_result({:#x}, fixture {:#x}): returned {}, stage {} -> {}, stats {:#x} -> {:#x}", this, fixture, r, stage, f.stage_index(), stats, l.comp_stats()));
    r
}

/// Rule: at the end of the Argentine season the two clubs with the lowest points average over three seasons go down:
/// each is announced as relegated and its progress set, and it is marked relegated in the league table and in both
/// half-season stages; then the season honours of the league and of the first stage.
///
/// `ArgPrm::record_season_honours()`: the averages up to date (`update_relegation_averages_virtual`(0)); the club of the
/// second-last averages row (by `ARG_AVERAGE_COUNT`): not −1, the status announced (virtual
/// `comp_vtable::ANNOUNCE_TEAM_STATUS`(club, 3, −1, 0, 0)) and its progress set (virtual
/// `comp_vtable::SET_CLUB_PROGRESS`(club, −2, 0x13, 0x13, 0x13)); −1, the arg_prm.cpp box of line 0x31c. The last row
/// the same with 0x14 codes (box 0x32b). Then, for each row k below `league::CLUB_COUNT`, the rows k of the league table
/// and of stages 0 and 1 whose club's id is one of the two get `league_table_row::STATUS` = 3; then League::
/// record_season_honours (`exe::league_record_season_honours`) and stage 0's virtual
/// `comp_vtable::RECORD_SEASON_HONOURS`.
/// Original: `ArgPrm::record_season_honours` (0x405ca0, arg_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn arg_prm_record_season_honours(this: usize) {
    let this = League(this);
    update_relegation_averages_virtual(this, 0);
    covered!("cx08_4 argentine relegation by averages");
    let mut relegated = [0i32; 2];
    for (k, (code, row)) in [(0x13u32, 0x31cu32), (0x14, 0x32b)].into_iter().enumerate() {
        let id = arg_average_row_club(this, this_average_count(this) - 2 + k as i32);
        relegated[k] = id;
        if id == -1 {
            show_error_at(ARG_PRM_FILE, row);
            continue;
        }
        let club = club_at(id as isize).addr();
        let announce: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::ANNOUNCE_TEAM_STATUS));
        announce(this.addr(), club, 3, 0xffff_ffff, 0, 0);
        let progress: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_CLUB_PROGRESS));
        progress(this.addr(), club, 0xffff_fffe, code, code, code);
    }
    let mut k: i16 = 0;
    while k < this.club_count() {
        let tables = [this.table(), League(read_usize(this.stages())).table(), League(read_usize(this.stages() + 4)).table()];
        for t in tables {
            for id in relegated {
                let row = LeagueTableRow(t + k as usize * league_table_row::SIZE);
                if read_i32(row.club() as usize) == id {
                    row.set_status(3);
                }
            }
        }
        k += 1;
    }
    exe::league_record_season_honours(this);
    bra_nat_first_call_virtual_no_args(read_usize(this.stages()), comp_vtable::RECORD_SEASON_HONOURS);
}

/// The averages row count of an Argentine league (`ARG_AVERAGE_COUNT`, a signed word).
pub(crate) unsafe fn this_average_count(this: League) -> i32 {
    read_i16(this.addr() + ARG_AVERAGE_COUNT) as i32
}

/// Comparer for `arg_prm_record_season_honours`: only the original runs (it announces and records); afterwards the
/// league table rows of the clubs of the last two averages rows must carry status 3.
pub(crate) unsafe extern "thiscall" fn compare_arg_prm_record_season_honours(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let l = League(this);
    o(this);
    let n = this_average_count(l);
    let ids = [arg_average_row_club(l, n - 2), arg_average_row_club(l, n - 1)];
    let marked = (0..l.club_count() as usize).map(|k| LeagueTableRow(l.table() + k * league_table_row::SIZE))
        .filter(|r| ids.contains(&read_i32(r.club() as usize))).all(|r| r.status() == 3);
    SCOREBOARD.record(marked, || format!("arg_prm_record_season_honours({:#x}): relegated ids {:?} not all marked", this, ids));
}
