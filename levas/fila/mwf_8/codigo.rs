// ---------- mwf_8: the season turnovers of Serie C1 B, the Spanish Second Division and the Spanish lower leagues (ita_ser_c1b.cpp, spa_second.cpp, spa_lower.cpp) ----------

/// The season turnover of Serie C1 B: the season archived in the statistics, the table, the round dates and the stages
/// cleared, the year moved on by one and the rules, the table, the fixtures and the order rebuilt; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// The league turnover template of Brazil's Series B (`bra_nat_second_start_new_season`, checked instruction by
/// instruction): `league::SEASON_CLOSED` = 0, then `rebuild_season`: the table freed (League::free_table
/// `exe::league_free_table`, thiscall(this, 0), ret 4); `.round_dates()` freed (`exe::crt_free`) and zeroed; with
/// statistics, Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1),
/// ret 4); with a current stage ≥ 0, the stages 0..=`.current_stage()` destroyed (virtual 0 with 1; the stage array
/// itself is kept); the year up by one and `league::CURRENT_STAGE` = −1. Then, each with its own box (ita_ser_c1b.cpp, lines 0x224, 0x22b,
/// 0x231 and 0x238) when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table (ItaSerC1b::create_table at 0x64f820, thiscall, ret 0),
/// the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table
/// `exe::league_sort_table`, thiscall(this, 0), ret 4); then the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`), the board expectations (League::set_club_season_expectations) and the
/// referee renewal (OfficialsManager::renew_officials `exe::officialsmanager_renew_officials`,
/// thiscall([`G_OFFICIALS_MANAGER`], `.competition()`), ret 4), whose eax it returns. After a box the copy returns 0
/// (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `ItaSerC1b::start_new_season` (0x64f9e0, ita_ser_c1b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c1b_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x64f820, 0x9ea558, [0x224, 0x22b, 0x231, 0x238], 1) { // mwf8: the turnover of Serie C1 B
        return 0;
    }
    covered!("mwf_8 Serie C1 B rollover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `ita_ser_c1b_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c1b_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("ita_ser_c1b_start_new_season", League(this), || o(this))
}

/// The season turnover of the Spanish Second Division: the season archived in the statistics, the table, the round dates and the stages
/// cleared, the year moved on by one and the rules, the table, the fixtures and the order rebuilt; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// The league turnover template of Brazil's Series B (`bra_nat_second_start_new_season`, checked instruction by
/// instruction): `league::SEASON_CLOSED` = 0, then `rebuild_season`: the table freed (League::free_table
/// `exe::league_free_table`, thiscall(this, 0), ret 4); `.round_dates()` freed (`exe::crt_free`) and zeroed; with
/// statistics, Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1),
/// ret 4); with a current stage ≥ 0, the stages 0..=`.current_stage()` destroyed (virtual 0 with 1; the stage array
/// itself is kept); the year up by one and `league::CURRENT_STAGE` = −1. Then, each with its own box (spa_second.cpp, lines 0x312, 0x319,
/// 0x31f and 0x326) when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table (SpaSecond::create_table at 0x855b40, thiscall, ret 0),
/// the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table
/// `exe::league_sort_table`, thiscall(this, 0), ret 4); then the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`), the board expectations (League::set_club_season_expectations) and the
/// referee renewal (OfficialsManager::renew_officials `exe::officialsmanager_renew_officials`,
/// thiscall([`G_OFFICIALS_MANAGER`], `.competition()`), ret 4), whose eax it returns. After a box the copy returns 0
/// (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `SpaSecond::start_new_season` (0x855d80, spa_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x855b40, 0xa828bc, [0x312, 0x319, 0x31f, 0x326], 1) { // mwf8: the turnover of the Spanish Second Division
        return 0;
    }
    covered!("mwf_8 Spanish Second Division rollover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `spa_second_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_spa_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("spa_second_start_new_season", League(this), || o(this))
}

/// The front of a league turnover without `rebuild_season` (checked instruction by instruction):
/// `league::SEASON_CLOSED` = 0; the table freed (League::free_table `exe::league_free_table`, thiscall(this, 0), ret 4);
/// `.round_dates()` freed (`exe::crt_free`) and zeroed; with statistics, Comp_Stats::archive_season_and_reset
/// (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); with a current stage ≥ 0, the stages
/// 0..=`.current_stage()` destroyed (virtual 0 with 1; the stage array is kept); the year up by `year` and
/// `league::CURRENT_STAGE` = −1.
pub(crate) unsafe fn spa_lower_clear_season(this: League, year: i16) {
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
    let a = this.year();
    this.set_year(a.wrapping_add(year));
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
}

/// The season turnover of a Spanish lower league (the Segunda B groups [inference]): the season archived, the table,
/// the dates and the stages cleared, the year plus one, then the rules, the table and the fixtures rebuilt and the
/// referees renewed; unlike the upper divisions there is no table order, no predicted positions and no board
/// expectations.
///
/// `spa_lower_clear_season` (one year); then, each with its own box (spa_lower.cpp, lines 0x36c, 0x373 and 0x379) when
/// it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table (SpaLower::create_table 0x8533d0, thiscall,
/// ret 0: the B teams of the First and Second Division clubs, then the best other lower clubs) and the fixtures
/// (League::create_fixtures `exe::league_create_fixtures`); finally the referee renewal
/// (OfficialsManager::renew_officials `exe::officialsmanager_renew_officials`, thiscall([`G_OFFICIALS_MANAGER`],
/// `.competition()`), ret 4), whose eax it returns. After a box the copy returns 0 (the original returns the box's eax;
/// the turnover's caller does not read it).
/// Original: `SpaLower::start_new_season` (0x853830, spa_lower.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_lower_start_new_season(this: usize) -> u32 {
    let this = League(this);
    let year = 1; // mwf8: the year of the Spanish lower league
    spa_lower_clear_season(this, year);
    covered!("mwf_8 spanish lower league rollover");
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(0xa82880, 0x36c);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x8533d0usize);
    if table(this.addr()) == 0 {
        show_error_at(0xa82880, 0x373);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(0xa82880, 0x379);
        return 0;
    }
    exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), this.competition().addr()) as u32
}

/// Comparer for `spa_lower_start_new_season`: only the original runs (it frees and rebuilds the league); the year must
/// go up by one.
pub(crate) unsafe extern "thiscall" fn compare_spa_lower_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_state_league_new_season("spa_lower_start_new_season", League(this), || o(this))
}
