// ---------- batch %%LEVA%%: the season turnovers of the Veikkausliiga, the Tippeligaen and the Allsvenskan (fin_prm.cpp, nor_prm.cpp, swe_prm.cpp) ----------

/// Calls the start_new_season virtual (`comp_vtable::START_NEW_SEASON`, mov ecx -> thiscall, ret 0) of the
/// competition object of key `g` (`G_COMP_OBJECT_TABLE` by its id), when there is one.
pub(crate) unsafe fn start_new_season_of(g: usize) {
    let c = comp_by_global(g);
    if c != 0 {
        bra_nat_first_call_virtual_no_args(c, comp_vtable::START_NEW_SEASON);
    }
}

/// The promotion and relegation chain of the league (virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`, push
/// `pairs`; mov ecx -> thiscall, ret 4).
pub(crate) unsafe fn promotion_relegation_chain(this: League, pairs: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    f(this.addr(), pairs);
}

/// A direct call with no arguments on the league (thiscall, ret 0).
pub(crate) unsafe fn call_on_league(f: usize, this: League) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
    f(this.addr());
}

/// The common body of the top-division turnovers (no extra stages): `pre` (the promotion and relegation steps), then
/// the table freed (`exe::league_free_table`(0)), the round dates freed and zeroed, the season archived in the
/// statistics (`exe::comp_stats_archive_season_and_reset`(1), when there are statistics), the year + 1; then, each with
/// the box of `file` at its line when it returns 0 (and 0 returned): the rules (virtual `comp_vtable::SET_UP_RULES`), the
/// table (`table`, direct), `exe::league_create_fixtures` and `exe::league_sort_table`(0); then `post` (the lower
/// divisions' turnovers), the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), the board
/// expectations (League::set_club_season_expectations) and the referee renewal (OfficialsManager::renew_officials on
/// `G_OFFICIALS_MANAGER` with the record), whose eax is returned. Unlike the lower divisions' rebuild, the stages are not
/// destroyed and the current stage is not reset (these leagues have none).
pub(crate) unsafe fn top_division_turnover(this: League, pre: impl FnOnce(), table: usize, file: usize, rows: [u32; 4], post: impl FnOnce()) -> u32 {
    this.set_season_closed(0);
    pre();
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
    this.set_year(this.year().wrapping_add(1)); // mwe14: the top division next year
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, rows[0]);
        return 0;
    }
    let t: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(table);
    if t(this.addr()) == 0 {
        show_error_at(file, rows[1]);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, rows[2]);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, rows[3]);
        return 0;
    }
    post();
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// The Finnish turnover, led by the Veikkausliiga: the promotion and relegation chain (with 0), the two unnamed Finnish
/// steps 0x595f00 and 0x596050 [inference: the First Division groups refreshed and regrouped], the Veikkausliiga rebuilt
/// for the next year (rules, the 12-club table, fixtures, order), then the Ykkönen turnover, the predicted positions,
/// the board expectations and the referee renewal.
///
/// `FinPrm::start_new_season()`: `top_division_turnover` with, before, virtual
/// `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(0) and the direct calls 0x595f00 and 0x596050 (thiscall, ret 0);
/// the table `FinPrm::create_table` (0x595b70) and the boxes of fin_prm.cpp (0x9c8254) at 0x1c7, 0x1ce, 0x1d4, 0x1db;
/// after, the Ykkönen's start_new_season (`KEY_CLUB_COMP_FINNISH_FIRST_DIVISION`, called without a null check).
/// Original: `FinPrm::start_new_season` (0x595d40, fin_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_prm_start_new_season(this: usize) -> u32 {
    let this = League(this);
    top_division_turnover(this, || {
        covered!("mwe_14 finnish turnover");
        promotion_relegation_chain(this, 0); // mwe14: the Finnish chain
        call_on_league(0x595f00, this);
        call_on_league(0x596050, this);
    }, 0x595b70, 0x9c8254, [0x1c7, 0x1ce, 0x1d4, 0x1db], || {
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION), comp_vtable::START_NEW_SEASON); // mwe14: the Ykkonen turnover
    })
}

/// The Norwegian turnover, led by the Tippeligaen: the promotion and relegation chain (with 1), the First and Second
/// Division clubs swapped (`nor_prm_swap_first_and_second_division_clubs` 0x792960) and the unnamed step 0x792b40
/// [inference: the Second Division groups regrouped], the Tippeligaen rebuilt for the next year, then the First
/// Division turnover, the predicted positions, the board expectations and the referee renewal.
///
/// `NorPrm::start_new_season()`: `top_division_turnover` with, before, virtual
/// `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(1) and the direct calls 0x792960 and 0x792b40 (thiscall, ret 0);
/// the table `NorPrm::create_table` (0x7925e0) and the boxes of nor_prm.cpp (0xa6977c) at 0x212, 0x219, 0x21f, 0x226;
/// after, the Norwegian First Division's start_new_season (`KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION`, no null check).
/// Original: `NorPrm::start_new_season` (0x7927a0, nor_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_prm_start_new_season(this: usize) -> u32 {
    let this = League(this);
    top_division_turnover(this, || {
        covered!("mwe_14 norwegian turnover");
        promotion_relegation_chain(this, 1); // mwe14: the Norwegian chain
        call_on_league(0x792960, this); // mwe14: the Norwegian First and Second Division clubs swapped
        call_on_league(0x792b40, this);
    }, 0x7925e0, 0xa6977c, [0x212, 0x219, 0x21f, 0x226], || {
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION), comp_vtable::START_NEW_SEASON);
    })
}

/// The Swedish turnover, led by the Allsvenskan: the promotion and relegation chain (with 0); with a playable Second
/// Division the unnamed step 0x890b40, otherwise three regional clubs promoted to the Superettan
/// (`swe_prm_promote_regional_clubs_to_first_division` 0x890980); the division tiers limited
/// (League::limit_club_division_tier(1, 3, −3, 1)) and the Second Division sections dealt again
/// (`swe_prm_assign_second_division_sections` 0x890c70); the Allsvenskan rebuilt for the next year; then the
/// Superettan turnover and the Second Division's (when it exists), the predicted positions, the board expectations and
/// the referee renewal.
///
/// `SwePrm::start_new_season()`: `top_division_turnover` with, before, virtual
/// `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(0), 0x890b40 or 0x890980 by the Second Division object
/// (`G_COMP_OBJECT_TABLE` by `KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION`), `exe::league_limit_club_division_tier`(1, 3, −3,
/// 1) and 0x890c70 (all thiscall); the table `SwePrm::create_table` (0x890430) and the boxes of swe_prm.cpp (0xa8b578)
/// at 0x1ba, 0x1c1, 0x1c7, 0x1ce; after, the Superettan's start_new_season (no null check) and the Second Division's
/// (only when it exists).
/// Original: `SwePrm::start_new_season` (0x8905f0, swe_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_prm_start_new_season(this: usize) -> u32 {
    let this = League(this);
    top_division_turnover(this, || {
        covered!("mwe_14 swedish turnover");
        promotion_relegation_chain(this, 0);
        if comp_by_global(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION) != 0 {
            covered!("mwe_14 swedish second division playable");
            call_on_league(0x890b40, this);
        } else {
            call_on_league(0x890980, this); // mwe14: the regional clubs promoted
        }
        exe::league_limit_club_division_tier(this, 1, 3, -3, 1);
        call_on_league(0x890c70, this);
    }, 0x890430, 0xa8b578, [0x1ba, 0x1c1, 0x1c7, 0x1ce], || {
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION), comp_vtable::START_NEW_SEASON);
        start_new_season_of(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION);
    })
}

/// Comparer for `fin_prm_start_new_season`: only the original runs (it frees, allocates and moves clubs); the year must
/// go up by one (`compare_state_league_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_fin_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("fin_prm_start_new_season", League(this), || o(this))
}

/// Comparer for `nor_prm_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_nor_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("nor_prm_start_new_season", League(this), || o(this))
}

/// Comparer for `swe_prm_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_swe_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_state_league_new_season("swe_prm_start_new_season", League(this), || o(this))
}
