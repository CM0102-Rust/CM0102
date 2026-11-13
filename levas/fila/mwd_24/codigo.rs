// ---------- batch %%LEVA%%: the Turkish season turnover (tur_first.cpp, tur_second.cpp) ----------

/// The rebuild of the Turkish turnovers, which keep the stage fields: League::free_table(this, 0); the round dates
/// (+0xba) freed and cleared; the statistics archived (push 1) when there are any; the year (+0x40) + 1; then, each
/// with the box of its line (`rows`) and stopping when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`),
/// the table (`table`, direct call), the fixtures (League::create_fixtures) and the order (League::sort_table: push 0).
/// Unlike the Croatian rebuild, +0xc, +0x2c, +0x30 and +0x4c are not touched.
pub(crate) unsafe fn rebuild_turkish_league(this: League, table: usize, file: usize, rows: [u32; 4]) -> bool {
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    if this.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(this.comp_stats(), 1);
    }
    this.set_year(this.year().wrapping_add(1));
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

/// Rule: the Turkish turnover, led by the First Division: the season reopened (+0x4c = 0); promotion and relegation
/// (virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`: push 1, `tur_first_process_promotion_relegation_chain`);
/// the ten best clubs of the lowest division up into the second division category B and its relegated clubs down
/// (`tur_first_promote_lower_clubs_to_third_tier`, ecx = this); the third tier reshuffle (0x8fe580, ecx = this, not
/// ported); the First Division rebuilt for the next year (`rebuild_turkish_league`, lines 0x213, 0x21a, 0x220, 0x227);
/// the division tier of the clubs of both second divisions capped (League::limit_club_division_tier(1, 3, −3, 1),
/// category A then B) and both turned over (virtual `comp_vtable::START_NEW_SEASON`); then the predicted positions, the
/// board expectations and the officials (`finish_croatian_turnover`).
/// Original: `TurFirst::start_new_season` (0x8fe040, tur_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    chain(this.addr(), 1);
    for f in [0x8fe3a0usize, 0x8fe580] {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
        f(this.addr());
    }
    covered!("mwd_24 turkish first division turnover");
    if !rebuild_turkish_league(this, 0x8fddc0, 0xaa1840, [0x213, 0x21a, 0x220, 0x227]) {
        return 0;
    }
    let (a, b) = (comp_by_global(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A), comp_by_global(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B));
    exe::league_limit_club_division_tier(a, 1, 3, -3, 1);
    exe::league_limit_club_division_tier(b, 1, 3, -3, 1);
    bra_nat_first_call_virtual_no_args(a, comp_vtable::START_NEW_SEASON);
    bra_nat_first_call_virtual_no_args(b, comp_vtable::START_NEW_SEASON);
    finish_croatian_turnover(this)
}

/// Rule: the Turkish Second Division turnover (called by the First Division's): the season reopened, rebuilt for the
/// next year (`rebuild_turkish_league`, lines 0x1bf, 0x1c6, 0x1cc, 0x1d3 of tur_second.cpp), the predicted positions,
/// the board expectations and the officials.
/// Original: `TurSecond::start_new_season` (0x8ff330, tur_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    covered!("mwd_24 turkish second division turnover");
    if !rebuild_turkish_league(this, 0x8ff150, 0xaa187c, [0x1bf, 0x1c6, 0x1cc, 0x1d3]) {
        return 0;
    }
    finish_croatian_turnover(this)
}

/// Rule: Turkish promotion and relegation: the final positions of the First Division and both second divisions are
/// recorded (virtual `league_vtable::RECORD_FINAL_POSITIONS`); then the marked clubs are swapped between the First
/// Division and the second division category A (in pairs), and between category A and category B and each of
/// category B's four groups (its stages 0..3; as a block).
///
/// League::promote_and_relegate (`exe::league_promote_and_relegate`(this, above, below, 1, pairs, −1, −1): 6 pushes ->
/// thiscall, ret 0x18): First Division / category A with pairs 1, category A / category B with 0, then category A /
/// each of category B's stages `[+0xc][0..3]` with 0. Returns the last call's eax; the argument is not read.
/// Original: `TurFirst::process_promotion_relegation_chain` (0x8fe250, tur_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn tur_first_process_promotion_relegation_chain(this: usize, _pairs: u32) -> u32 {
    let (a, b) = (comp_by_global(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A), comp_by_global(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B));
    bra_nat_first_call_virtual_no_args(this, league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(a, league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(b, league_vtable::RECORD_FINAL_POSITIONS);
    covered!("mwd_24 turkish promotion and relegation");
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION), a, 1, 1, -1, -1); // mwd24: the top flight and category A
    let mut r = exe::league_promote_and_relegate(this, a, b, 1, 0, -1, -1);
    for k in 0..4 { // mwd24: the four groups of category B
        r = exe::league_promote_and_relegate(this, a, read_usize(read_usize(b + league::STAGES) + 4 * k), 1, 0, -1, -1);
    }
    r
}

/// Comparer for `tur_first_start_new_season` (only the original; the year went up by one).
pub(crate) unsafe extern "thiscall" fn compare_tur_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("tur_first_start_new_season", League(this), || o(this))
}

/// Comparer for `tur_second_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("tur_second_start_new_season", League(this), || o(this))
}

/// Comparer for `tur_first_process_promotion_relegation_chain`: only the original (it moves clubs, as the Brazilian
/// chain's comparer).
pub(crate) unsafe extern "thiscall" fn compare_tur_first_process_promotion_relegation_chain(this: usize, pairs: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I2%%)(this, pairs)
}
