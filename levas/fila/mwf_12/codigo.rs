// ---------- mwf_12: the NSL season honours and turnover (aus_nsl.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%);
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
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x411b70usize);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("aus_nsl_start_new_season", League(this), || o(this))
}
