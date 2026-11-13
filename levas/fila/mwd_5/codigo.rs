// ---------- batch %%LEVA%%: the Croatian leagues (cro_a1.cpp, cro_a2a.cpp, cro_a2b.cpp): the season turnover ----------

/// The rebuild common to the three Croatian turnovers (after the First Division's promotion and relegation), in the
/// original's order; returns false after an error box.
///
/// League::free_table (`exe::league_free_table`: push 0 -> thiscall, ret 4); the round dates (+0xba) freed
/// (`exe::crt_free`, cdecl) and cleared; the statistics archived (Comp_Stats::archive_season_and_reset
/// `exe::comp_stats_archive_season_and_reset`: push 1 -> thiscall, ret 4) when there are any; the stage vector (+0xc)
/// freed when there is one (the stage objects are not destroyed); the year (+0x40) + 1; +0xc = 0, +0x2c = 0, current
/// stage −1, `season_closed` = 1 when `reopen` (the Croatian and Russian Premier turnovers; not the Greek ones); then, each with the box of its line (`rows`) and stopping when it returns 0: the
/// rules (virtual `comp_vtable::SET_UP_RULES`), the table (`table`, direct call, thiscall, ret 0), the fixtures
/// (League::create_fixtures) and the order (League::sort_table: push 0).
pub(crate) unsafe fn rebuild_croatian_league(this: League, table: usize, file: usize, rows: [u32; 4], reopen: bool) -> bool {
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    if this.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(this.comp_stats(), 1);
    }
    if this.stages() != 0 {
        exe::crt_free(this.stages());
    }
    this.set_year(this.year().wrapping_add(1)); // mwd5: the next year
    this.set_stages(0);
    this.set_stage_count(0);
    this.set_current_stage(0xffff_ffff);
    if reopen {
        this.set_season_closed(1);
    }
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, rows[0]);
        return false;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(table);
    if table(this.addr()) == 0 {
        show_error_at(file, rows[1]);
        return false;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, rows[2]);
        return false;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, rows[3]);
        return false;
    }
    true
}

/// The end common to the Croatian turnovers: the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`),
/// the board expectations of each club (League::set_club_season_expectations, thiscall, ret 0) and the renewal of the
/// officials (OfficialsManager::renew_officials(+4): push; mov ecx, [0xb74340] -> thiscall, ret 4), whose eax is
/// returned.
pub(crate) unsafe fn finish_croatian_turnover(this: League) -> u32 {
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// Rule: the Croatian turnover, led by the First Division: the season is reopened (+0x4c = 0); promotion and relegation
/// along the chain First Division -> north group -> south group (virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`:
/// push 1 -> thiscall, ret 4); the four best third-tier clubs up into the second division and its relegated clubs down
/// (`cro_a1_swap_second_and_third_tier_clubs`, ecx = this); the 32 second division clubs regrouped north and south by
/// the latitude of their city (0x515190, cdecl); the First Division rebuilt (`rebuild_croatian_league`, lines 0x1c0,
/// 0x1c7, 0x1cd, 0x1d4); the division tier of the clubs of both second division groups capped (League::limit_club_division_tier
/// `exe::league_limit_club_division_tier`(group, 1, 3, −3, 1): 4 pushes -> thiscall, ret 0x10); both groups turned over
/// (virtual `comp_vtable::START_NEW_SEASON`); then the predicted positions, the board expectations and the officials
/// (`finish_croatian_turnover`).
/// Original: `CroA1::start_new_season` (0x5149a0, cro_a1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a1_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    chain(this.addr(), 1);
    let swap: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x514ed0usize);
    swap(this.addr()); // mwd5: third tier up, second tier down
    let realign: unsafe extern "C" fn() = core::mem::transmute(0x515190usize);
    realign();
    covered!("mwd_5 croatian first division turnover");
    if !rebuild_croatian_league(this, 0x5147b0, 0x9b53c0, [0x1c0, 0x1c7, 0x1cd, 0x1d4], true) {
        return 0;
    }
    let (north, south) = (comp_by_global(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH), comp_by_global(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH));
    exe::league_limit_club_division_tier(north, 1, 3, -3, 1);
    exe::league_limit_club_division_tier(south, 1, 3, -3, 1);
    bra_nat_first_call_virtual_no_args(north, comp_vtable::START_NEW_SEASON); // mwd5: the north group turnover
    bra_nat_first_call_virtual_no_args(south, comp_vtable::START_NEW_SEASON);
    finish_croatian_turnover(this)
}

/// Rule: the Croatian Second Division north group turnover (called by the First Division's): reopened, rebuilt for the
/// next year (`rebuild_croatian_league`, lines 0x1b7, 0x1be, 0x1c4, 0x1cb), the predicted positions, the board
/// expectations and the officials.
/// Original: `CroA2a::start_new_season` (0x515ea0, cro_a2a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2a_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    covered!("mwd_5 croatian second division north turnover");
    if !rebuild_croatian_league(this, 0x515cb0, 0x9b53f8, [0x1b7, 0x1be, 0x1c4, 0x1cb], true) {
        return 0;
    }
    finish_croatian_turnover(this)
}

/// Rule: the Croatian Second Division south group turnover, as the north group's (lines 0x1b7, 0x1be, 0x1c4, 0x1cb of
/// cro_a2b.cpp).
/// Original: `CroA2b::start_new_season` (0x516fc0, cro_a2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2b_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    covered!("mwd_5 croatian second division south turnover");
    if !rebuild_croatian_league(this, 0x516dd0, 0x9b5430, [0x1b7, 0x1be, 0x1c4, 0x1cb], true) {
        return 0;
    }
    finish_croatian_turnover(this)
}

/// Comparer for `cro_a1_start_new_season` (only the original; the year went up by one).
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("cro_a1_start_new_season", League(this), || o(this))
}

/// Comparer for `cro_a2a_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2a_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("cro_a2a_start_new_season", League(this), || o(this))
}

/// Comparer for `cro_a2b_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2b_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_state_league_new_season("cro_a2b_start_new_season", League(this), || o(this))
}
