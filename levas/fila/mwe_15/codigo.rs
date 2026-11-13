// ---------- batch %%LEVA%%: the Portuguese turnover and its promotion and relegation chain (por_prm.cpp) ----------

/// The Portuguese promotion and relegation chain: the final positions of the Premier Division and the Liga de Honra are
/// recorded and the marked clubs swapped between them; when the Second Division B is played, its final positions and
/// those of its two other zones are recorded and the Liga de Honra swaps clubs with each of the three zones.
///
/// `PorPrm::process_promotion_relegation_chain(pairs)` (`pairs` is not read): virtual
/// `league_vtable::RECORD_FINAL_POSITIONS` (thiscall, ret 0) of the Premier Division and of the Liga de Honra
/// (`G_COMP_OBJECT_TABLE` by `KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION` and `KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION`);
/// League::promote_and_relegate (`exe::league_promote_and_relegate`: push −1; push −1; push 1; push 1; push lower; push
/// upper; mov ecx, this -> thiscall, ret 0x18) between them. With the Second Division B object
/// (`KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B`): its RECORD_FINAL_POSITIONS and those of its stages 0 and 1 (the
/// other two zones, `league::STAGES`), then promote_and_relegate between the Liga de Honra and the Second Division B,
/// and between the Liga de Honra and each of the two stages; the last call's eax is returned. Without it, the
/// original returns the value of `G_COMP_OBJECT_TABLE` (the register left from the lookup).
/// Original: `PorPrm::process_promotion_relegation_chain` (0x7cfbf0, por_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn por_prm_process_promotion_relegation_chain(this: usize, _pairs: u32) -> u32 {
    let (premier, second) = (comp_by_global(KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION), comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION));
    bra_nat_first_call_virtual_no_args(premier, league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(second, league_vtable::RECORD_FINAL_POSITIONS);
    covered!("mwe_15 portuguese promotion and relegation");
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION), comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION), 1, 1, -1, -1); // mwe15: the Premier Division and the Liga de Honra
    let b = comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B);
    if b == 0 {
        return read_usize(G_COMP_OBJECT_TABLE) as u32;
    }
    covered!("mwe_15 portuguese second division b");
    bra_nat_first_call_virtual_no_args(b, league_vtable::RECORD_FINAL_POSITIONS);
    let zone = |k: usize| read_usize(read_usize(comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B) + league::STAGES) + 4 * k);
    bra_nat_first_call_virtual_no_args(zone(0), league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(zone(1), league_vtable::RECORD_FINAL_POSITIONS);
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION), comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B), 1, 1, -1, -1);
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION), zone(0), 1, 1, -1, -1);
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION), zone(1), 1, 1, -1, -1) as u32
}

/// The Portuguese turnover, led by the Premier Division: the B team checks when the Second Division B is played, the
/// promotion and relegation chain (with 1); with the Second Division B two unnamed steps, otherwise the marked Liga de
/// Honra clubs exchanged with the regional zones (`PorPrm::exchange_second_division_with_zones`); the Premier Division
/// rebuilt for the next year (rules, the 18-club table, fixtures, order); then the Liga de Honra turnover and the Second
/// Division B's (when it is played), the predicted positions, the board expectations and the referee renewal.
///
/// `PorPrm::start_new_season()`: `league::SEASON_CLOSED` = 0; with the Second Division B object
/// (`G_COMP_OBJECT_TABLE` by `KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B`), the step 0x7d0320 [inference: the B team
/// checks]; virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(1); with the Second Division B, the steps
/// 0x7cff70 and 0x7d0210, otherwise 0x7cfd30 (all direct, thiscall, ret 0); then the body of `top_division_turnover`
/// (the table `PorPrm::create_table` 0x7cf800, the boxes of por_prm.cpp (0xa78724) at 0x227, 0x22e, 0x234, 0x23b) with,
/// after the rebuild, the Liga de Honra's start_new_season (no null check) and the Second Division B's (when it exists).
/// Original: `PorPrm::start_new_season` (0x7cf9e0, por_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_prm_start_new_season(this: usize) -> u32 {
    let this = League(this);
    portuguese_top_division_turnover(this, || {
        covered!("mwe_15 portuguese turnover");
        if comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B) != 0 {
            portuguese_step(0x7d0320, this);
        }
        let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
        chain(this.addr(), 1);
        if comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B) != 0 {
            portuguese_step(0x7cff70, this);
            portuguese_step(0x7d0210, this);
        } else {
            portuguese_step(0x7cfd30, this); // mwe15: the Liga de Honra exchanged with the zones
        }
    })
}

/// A direct call with no arguments on the league (thiscall, ret 0).
pub(crate) unsafe fn portuguese_step(f: usize, this: League) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
    f(this.addr());
}

/// The rest of the Portuguese turnover after `pre`: the table and round dates freed, the stats archived, the year + 1,
/// the rules, the table, the fixtures and the order (each with its box of por_prm.cpp), the Liga de Honra's and the
/// Second Division B's start_new_season, the predicted positions, the expectations and the referees (see
/// `por_prm_start_new_season`).
pub(crate) unsafe fn portuguese_top_division_turnover(this: League, pre: impl FnOnce()) -> u32 {
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
    this.set_year(this.year().wrapping_add(1)); // mwe15: the Premier Division next year
    let file = 0xa78724;
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, 0x227);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x7cf800usize);
    if table(this.addr()) == 0 {
        show_error_at(file, 0x22e);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, 0x234);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, 0x23b);
        return 0;
    }
    bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION), comp_vtable::START_NEW_SEASON);
    let b = comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B);
    if b != 0 {
        bra_nat_first_call_virtual_no_args(b, comp_vtable::START_NEW_SEASON);
    }
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// Comparer for `por_prm_process_promotion_relegation_chain`: only the original runs (it moves the clubs; the same as
/// the Brazilian chain's comparer).
pub(crate) unsafe extern "thiscall" fn compare_por_prm_process_promotion_relegation_chain(this: usize, pairs: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I0%%)(this, pairs)
}

/// Comparer for `por_prm_start_new_season`: only the original runs (it frees, allocates and moves clubs); the year must
/// go up by one (`compare_state_league_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_por_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("por_prm_start_new_season", League(this), || o(this))
}
