// ---------- batch %%LEVA%%: the Belgian season turnover and the Eerste Divisie and Dutch Cup stages (bel_first.cpp, hol_first.cpp, hol_cup.cpp) ----------

/// Rule: the Belgian turnover, led by the First Division: the season reopened; promotion and relegation along the
/// chain; the Third Division's movements and its split into groups A and B; the division tier of the clubs capped (the
/// First Division with (1, 3, −3, 1), the Second with (2, −3, 3, 0)); the First Division rebuilt for the next year
/// (rules, table, fixtures, order); the Second and Third Divisions turned over; the predicted positions, the board
/// expectations and the officials.
///
/// `top_division_turnover` with, before, virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(1),
/// bel_first_move_third_division_clubs (0x41ec90) and bel_first_split_third_division_groups (0x41ef70) (called with ecx
/// = this, not ported) and League::limit_club_division_tier (`exe::league_limit_club_division_tier`) on this and on the
/// object of `KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION`; the table BelFirst::create_table (0x41e8b0) and the boxes of
/// bel_first.cpp (0x989b18) at 0x1af, 0x1b6, 0x1bc, 0x1c3; after, the start_new_season (virtual
/// `comp_vtable::START_NEW_SEASON`, no null check) of the Second Division and of `KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION`.
/// After a box: 0.
/// Original: `BelFirst::start_new_season` (0x41ea90, bel_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    top_division_turnover(this, || {
        promotion_relegation_chain(this, 1);
        call_on_league(0x41ec90, this);
        call_on_league(0x41ef70, this);
        exe::league_limit_club_division_tier(this, 1, 3, -3, 1);
        exe::league_limit_club_division_tier(comp_by_global(KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION), 2, -3i32 as u32, 3, 0);
    }, 0x41e8b0, 0x989b18, [0x1af, 0x1b6, 0x1bc, 0x1c3], || {
        covered!("mwc_25 belgian first division turnover");
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION), comp_vtable::START_NEW_SEASON);
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION), comp_vtable::START_NEW_SEASON);
    })
}

/// Rule: the next stage of the Eerste Divisie: past the last stage, the error box; otherwise only when the Eredivisie is
/// complete: its positions and statuses are updated and the current stage goes up by one; on reaching stage 4 (after
/// the four periods) the play-off groups are built. With the Eredivisie still playing, nothing (it asks again later).
///
/// At the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the box of hol_first.cpp at line
/// 0x263. Otherwise `premier_division_complete`(`KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION`) (virtual
/// `comp_vtable::CHECK_COMPLETE` with (0, 1)); when complete, `update_premier_division_status` (virtual
/// `league_vtable::UPDATE_POSITIONS_AND_STATUS`), the current stage (read again) + 1 and, at 4,
/// HolFirst::create_play_off_groups (0x5f6720, thiscall, ret 0, not ported).
/// Original: `HolFirst::advance_stage` (0x5f6650, hol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_first_advance_stage(this: usize) {
    let l = League(this);
    if (l.current_stage() as i32) >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x9cee08, 0x263);
    }
    if !premier_division_complete(KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION) {
        return;
    }
    update_premier_division_status(KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION);
    let stage = (l.current_stage() as i32).wrapping_add(1);
    l.set_current_stage(stage as usize);
    covered!("mwc_25 eerste divisie next stage");
    if stage == 4 { // mwc25: the play-off groups after the four periods
        call_on_league(0x5f6720, l);
    }
}

/// Rule: the next stage of the Dutch Cup: past the last stage, the error box; otherwise the current stage goes up by
/// one; on reaching stage 19 (after the group stages) the knockout stage is built.
///
/// At the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the box of hol_cup.cpp at line
/// 0x24f. Otherwise the current stage + 1; at 0x13, HolCup::create_knockout_stage (0x5f4480, thiscall, ret 0, not
/// ported).
/// Original: `HolCup::advance_stage` (0x5f4190, hol_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_cup_advance_stage(this: usize) {
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x9cedd4, 0x24f);
    }
    let stage = stage + 1;
    l.set_current_stage(stage as usize);
    covered!("mwc_25 dutch cup next stage");
    if stage == 0x13 {
        call_on_league(0x5f4480, l);
    }
}

/// Comparer for `bel_first_start_new_season`: only the original runs (it moves clubs and rebuilds); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_bel_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("bel_first_start_new_season", League(this), || o(this))
}

/// Comparer for `hol_first_advance_stage`: only the original runs (it asks the Eredivisie and updates it;
/// `compare_first_division_advance`).
pub(crate) unsafe extern "thiscall" fn compare_hol_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_first_division_advance("hol_first_advance_stage", League(this), || o(this))
}

/// Comparer for `hol_cup_advance_stage`: side by side on the current stage when nothing is built; at the last stage or
/// on reaching stage 0x13, only the original.
pub(crate) unsafe extern "thiscall" fn compare_hol_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) || stage + 1 == 0x13 {
        return o(this);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(this + league::CURRENT_STAGE, 4)], || o(this), || hol_cup_advance_stage(this));
    SCOREBOARD.record(same, || format!("hol_cup_advance_stage({:#x}) original={:x?} rust={:x?}", this, mo, mr));
}
