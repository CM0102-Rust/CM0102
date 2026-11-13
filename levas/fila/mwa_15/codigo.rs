// ---------- batch %%LEVA%%: the season turnover of the Northern Irish and the Irish Premier Divisions (nir_prm.cpp, ire_prm.cpp) ----------

/// The turnover of an Irish Premier Division, which leads its nation's league turnover.
///
/// `league::SEASON_CLOSED` = 0; the promotion and relegation chain (the virtual
/// `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(`chain`), thiscall, ret 4: 1 in Northern Ireland, 0 in Ireland);
/// the table freed (League::free_table `exe::league_free_table`(this, 0)); the round table (`league::ROUND_DATES`) freed
/// (`exe::crt_free`) and zeroed; the season archived in the statistics (Comp_Stats::archive_season_and_reset
/// `exe::comp_stats_archive_season_and_reset`(stats, 1), when there); the year (`league::YEAR`) up by one. Then, each
/// with its own error box (returning 0) when it returns 0: the rules (`comp_vtable::SET_UP_RULES`), the table (`table`,
/// thiscall, ret 0), the fixtures (`exe::league_create_fixtures`) and the order (`exe::league_sort_table`(this, 0)).
/// Then the First Division's turnover (the competition object of the key global `first`, its
/// `comp_vtable::START_NEW_SEASON`), the predicted positions (`comp_vtable::SET_PREDICTED_POSITIONS`), the board
/// expectations and the referee renewal (`set_expectations_and_renew_officials`, whose return it passes on).
pub(crate) unsafe fn start_irish_premier_season(this: League, chain: u32, table: usize, first: usize, file: usize, rows: [u32; 4], year: i16) -> u32 {
    this.set_season_closed(0);
    let promote: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    promote(this.addr(), chain);
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
    this.set_year(this.year().wrapping_add(year));
    // simplification: in the boxes the original returns the box's eax; the caller of the turnover does not read it
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, rows[0]);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(table);
    if table(this.addr()) == 0 {
        show_error_at(file, rows[1]);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, rows[2]);
        return 0;
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(file, rows[3]);
        return 0;
    }
    covered!("mwa_15 irish premier turnover");
    bra_nat_first_call_virtual_no_args(comp_by_global(first), comp_vtable::START_NEW_SEASON); // mwa15: the First Division turns over with the Premier
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// Rule: the Northern Irish turnover is led by the Premier Division: promotion and relegation, the statistics archived,
/// the next year, the rules, the ten-club table, the fixtures and the order rebuilt; then the First Division turns over,
/// and the predicted positions, the board expectations and the referee renewal follow.
///
/// `start_irish_premier_season` with the chain argument 1, NirPrm::create_table (0x78ec60), the Northern Irish First
/// Division and the error box lines 0x1c0, 0x1c7, 0x1cd and 0x1d4 of nir_prm.cpp.
/// Original: `NirPrm::start_new_season` (0x78ee30, nir_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_prm_start_new_season(this: usize) -> u32 {
    start_irish_premier_season(League(this), 1, 0x78ec60, KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION, 0xa696d0, [0x1c0, 0x1c7, 0x1cd, 0x1d4], 1) // mwa15: the Northern Irish turnover
}

unsafe extern "thiscall" fn compare_nir_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("nir_prm_start_new_season", League(this), || o(this))
}

/// Rule: the Irish turnover is led by the Premier Division: promotion and relegation, the statistics archived, the next
/// year, the rules, the table, the fixtures and the order rebuilt; then the First Division turns over, and the predicted
/// positions, the board expectations and the referee renewal follow.
///
/// `start_irish_premier_season` with the chain argument 0, IrePrm::create_table (0x63a740), the Irish First Division and
/// the error box lines 0x1f3, 0x1fa, 0x200 and 0x207 of ire_prm.cpp (checked instruction by instruction against
/// NirPrm's).
/// Original: `IrePrm::start_new_season` (0x63a9b0, ire_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_prm_start_new_season(this: usize) -> u32 {
    start_irish_premier_season(League(this), 0, 0x63a740, KEY_CLUB_COMP_IRISH_FIRST_DIVISION, 0x9ea080, [0x1f3, 0x1fa, 0x200, 0x207], 1) // mwa15: the Irish turnover
}

unsafe extern "thiscall" fn compare_ire_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("ire_prm_start_new_season", League(this), || o(this))
}
