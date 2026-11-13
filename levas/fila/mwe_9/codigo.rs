// ---------- batch %%LEVA%%: the season turnovers of the Swedish, Norwegian and Finnish First Divisions (swe_first.cpp, nor_first.cpp, fin_first.cpp) ----------

/// The Superettan turnover: archives the season in the statistics, clears the table, the round dates and the stages,
/// moves to the next year and rebuilds the rules, the 16-club table, the fixtures and the order; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// `SweFirst::start_new_season()`: `league::SEASON_CLOSED` = 0, the body of `rebuild_season` (checked instruction by
/// instruction; the table is `SweFirst::create_table` 0x88ef90 and the boxes of swe_first.cpp (0xa8b53c) at 0x1aa,
/// 0x1b1, 0x1b7, 0x1be) and `set_expectations_and_renew_officials`, whose eax is returned. After a box: 0.
/// Original: `SweFirst::start_new_season` (0x88f150, swe_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x88ef90, 0xa8b53c, [0x1aa, 0x1b1, 0x1b7, 0x1be], 1) { // mwe9: the Superettan next year
        return 0;
    }
    covered!("mwe_9 superettan turnover");
    set_expectations_and_renew_officials(this)
}

/// The Norwegian First Division turnover: the same as the Superettan's, with `NorFirst::create_table` (0x7909a0) and the
/// boxes of nor_first.cpp (0xa69740) at 0x2c9, 0x2d0, 0x2d6, 0x2dd.
/// Original: `NorFirst::start_new_season` (0x790b90, nor_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x7909a0, 0xa69740, [0x2c9, 0x2d0, 0x2d6, 0x2dd], 1) { // mwe9: the Norwegian First Division next year
        return 0;
    }
    covered!("mwe_9 norwegian first division turnover");
    set_expectations_and_renew_officials(this)
}

/// The Ykkönen turnover: archives the season, clears the table, the round dates and the stages (and their vector, which
/// is allocated again), moves to the next year and rebuilds the rules, the North group table, the fixtures and the
/// order; then the board expectations, the referee renewal, the South group stage and the predicted positions.
///
/// `FinFirst::start_new_season()`: `league::SEASON_CLOSED` = 0, `exe::league_free_table`(0), the round dates freed
/// and zeroed, `exe::comp_stats_archive_season_and_reset`(1) when there are statistics, the stages 0..=current (when
/// current ≥ 0) destroyed by their virtual 0 with 1; the current stage −1, the stage vector freed (`exe::crt_free`),
/// the year + 1 and a new vector of 4 × `.stage_count()` bytes (`exe::crt_malloc`; 0 when there are no stages). Then,
/// each with the box of fin_first.cpp (0x9c818c) when it returns 0 (lines 0x21b, 0x222, 0x228, 0x22f, and 0 is
/// returned): the rules (virtual `comp_vtable::SET_UP_RULES`), `FinFirst::create_table` (0x593a70),
/// `exe::league_create_fixtures` and `exe::league_sort_table`(0). Then League::set_club_season_expectations,
/// OfficialsManager::renew_officials(record) on `G_OFFICIALS_MANAGER`, FinFirst::create_south_group_stage (0x594cd0,
/// thiscall, ret 0) and the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax is
/// returned.
/// Original: `FinFirst::start_new_season` (0x593c20, fin_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_first_start_new_season(this: usize) -> u32 {
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
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    exe::crt_free(this.stages());
    this.set_year(this.year().wrapping_add(1)); // mwe9: the Ykkonen next year
    let n = this.stage_count() as i32;
    let v = if n > 0 { exe::crt_malloc((n as u32).wrapping_mul(4)) } else { 0 };
    this.set_stages(v);
    let file = 0x9c818c;
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, 0x21b);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x593a70usize);
    if table(this.addr()) == 0 {
        show_error_at(file, 0x222);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, 0x228);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, 0x22f);
        return 0;
    }
    covered!("mwe_9 ykkonen turnover");
    let expectations: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SET_CLUB_SEASON_EXPECTATIONS);
    expectations(this.addr());
    exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), this.competition().addr());
    let south: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x594cd0usize);
    south(this.addr());
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS)
}

/// Comparer for `swe_first_start_new_season`: only the original runs (it frees and allocates); the year must go up by
/// one (`compare_state_league_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_swe_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("swe_first_start_new_season", League(this), || o(this))
}

/// Comparer for `nor_first_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_nor_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("nor_first_start_new_season", League(this), || o(this))
}

/// Comparer for `fin_first_start_new_season`: the year + 1, and the stage vector allocated again when there are
/// stages.
pub(crate) unsafe extern "thiscall" fn compare_fin_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    let r = compare_state_league_new_season("fin_first_start_new_season", League(this), || o(this));
    let l = League(this);
    let ok = (l.stages() != 0) == (l.stage_count() as i32 > 0);
    SCOREBOARD.record(ok, || format!("fin_first_start_new_season({:#x}) stages {} vector {:#x}", this, l.stage_count(), l.stages()));
    r
}
