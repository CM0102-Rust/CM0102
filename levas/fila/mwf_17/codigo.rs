// ---------- mwf_17: the Italian season turnover (ita_ser_a.cpp) ----------

/// The Italian season turnover, led by Serie A: promotion and relegation, the exchange with the level below (Serie D
/// and the C2 regrouping when Serie C2 is loaded, otherwise the C2 exchange), the C1 regrouping, then Serie A rebuilt
/// (next year, rules, 18 clubs, fixtures, order); the division tier capped in the lowest loaded groups; Serie B, C1 A,
/// C1 B (and C2 A, B and C) turned over; then the predicted positions, the board expectations and the referee renewal.
///
/// `league::SEASON_CLOSED` = 0; the chain (virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`, thiscall(this,
/// 1), ret 4); with the Serie C2 A object ([`KEY_CLUB_COMP_ITALIAN_SERIE_C2_A`] in `G_COMP_OBJECT_TABLE`),
/// ItaSerA::exchange_serie_d_clubs (0x6421c0) and ita_ser_a_regroup_serie_c2_clubs (0x642790), otherwise
/// ItaSerA::exchange_serie_c2_clubs_unplayed (0x641fc0); then ita_ser_a_regroup_serie_c1_clubs (0x642630); all called
/// with ecx = this and no arguments. The table freed (League::free_table `exe::league_free_table`, thiscall(this, 0),
/// ret 4); `.round_dates()` freed and zeroed; with statistics, Comp_Stats::archive_season_and_reset
/// (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the stage array freed when there is one
/// (without destroying stages) and zeroed, `league::STAGE_COUNT` = 0; the year up by one; `league::CURRENT_STAGE` = −1;
/// and `league::SEASON_CLOSED` = 1 (as in Serie B; the sisters leave 0).
///
/// Then, each with its own box (ita_ser_a.cpp, lines 0x2da, 0x2e1, 0x2e7 and 0x2ee) when it returns 0: the rules
/// (virtual `comp_vtable::SET_UP_RULES`), the table (ItaSerA::create_table 0x641510), the fixtures
/// (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table `exe::league_sort_table`,
/// thiscall(this, 0), ret 4). Then League::limit_club_division_tier (`exe::league_limit_club_division_tier`, thiscall(
/// group, 1, 3, −3, 1), ret 0x10) on C2 A, C2 B and C2 C when C2 A exists, otherwise on C1 A and C1 B; the turnovers
/// (virtual `comp_vtable::START_NEW_SEASON`) of Serie B, C1 A and C1 B (none checked for null) and, when C2 A exists,
/// of C2 A, C2 B and C2 C; the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`) and
/// `set_expectations_and_renew_officials`, whose eax (the referee renewal's) it returns. After a box the copy returns 0
/// (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `ItaSerA::start_new_season` (0x641700, ita_ser_a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_a_start_new_season(this: usize) -> u32 {
    let this = League(this);
    let call = |f: usize| {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
        f(this.addr());
    };
    this.set_season_closed(0);
    let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    chain(this.addr(), 1);
    if comp_by_global(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A) != 0 {
        call(0x6421c0);
        call(0x642790);
    } else {
        call(0x641fc0);
    }
    call(0x642630); // the Serie C1 regrouping
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
    if this.stages() != 0 {
        exe::crt_free(this.stages());
    }
    let a = this.year();
    this.set_year(a.wrapping_add(1)); // mwf17: the year of Serie A
    this.set_stages(0);
    write_u32(this.addr() + league::STAGE_COUNT, 0);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    this.set_season_closed(1);
    covered!("mwf_17 serie a rollover");
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(0x9ea2a8, 0x2da);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x641510usize);
    if table(this.addr()) == 0 {
        show_error_at(0x9ea2a8, 0x2e1);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(0x9ea2a8, 0x2e7);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(0x9ea2a8, 0x2ee);
        return 0;
    }
    let c2 = comp_by_global(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A) != 0;
    let capped: &[usize] = if c2 {
        &[KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C]
    } else {
        &[KEY_CLUB_COMP_ITALIAN_SERIE_C1_A, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B]
    };
    for &k in capped {
        exe::league_limit_club_division_tier(comp_by_global(k), 1, 3, -3, 1);
    }
    for k in [KEY_CLUB_COMP_ITALIAN_SERIE_B, KEY_CLUB_COMP_ITALIAN_SERIE_C1_A, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B] {
        bra_nat_first_call_virtual_no_args(comp_by_global(k), comp_vtable::START_NEW_SEASON);
    }
    if comp_by_global(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A) != 0 {
        for k in [KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C] {
            bra_nat_first_call_virtual_no_args(comp_by_global(k), comp_vtable::START_NEW_SEASON);
        }
    }
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// Comparer for `ita_ser_a_start_new_season`: only the original runs (it moves clubs and rebuilds the Italian
/// leagues); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_a_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("ita_ser_a_start_new_season", League(this), || o(this))
}
