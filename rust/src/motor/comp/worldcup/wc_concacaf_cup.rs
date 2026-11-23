use super::*;

/// The season turnover of the CONCACAF World Cup qualifying: last campaign's tables, entrant list and stages freed (the
/// stage vector kept), seven stages, the season archived, the next campaign's year, the entrants again, the predicted
/// positions, the seeds, the round table, the tie tree and the two Central American groups.
///
/// `qualifying_cup_free_tables`; `cup::STAGE_COUNT` = 7 and `cup::CURRENT_STAGE` = −1; the season archived
/// (`exe::comp_stats_archive_season_and_reset`, +8); `cup::SEASON_SET_UP` = 1; the year up by four until it reaches the
/// game's (`advance_qualifying_campaign`); `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own
/// box (wc_concacaf_cup.cpp) when it returns 0: the entrants (WcConcacafCup 0x916250, thiscall, ret 0; line 0x41f); the
/// predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`); the seeds (WcConcacafCup 0x915d40; line
/// 0x429), the round table (virtual `comp_vtable::SET_UP_RULES`; line 0x430) and the tie tree (virtual
/// `cup_vtable::MAP_FIXTURE_TREE`; line 0x437); finally the Central American groups
/// (WcConcacafCup::create_central_american_groups, thiscall, ret 0), whose eax it returns. After a box the copy returns
/// 0 (the original the box's eax).
/// Original: `WcConcacafCup::start_new_season` (0x915ff0, wc_concacaf_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_start_new_season(this: usize) -> u32 {
    use wc_cup_turnover_consts as wt;
    let cup = Cup(this);
    qualifying_cup_free_tables(cup);
    cup.set_stage_count(7);
    cup.set_current_stage(usize::MAX);
    let stats = read_usize(this + 8);
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    cup.set_season_set_up(1);
    advance_qualifying_campaign(this + cup::YEAR);
    covered!("mwg_13 concacaf qualifying rollover");
    cup.set_current_round(0);
    cup.set_run_by_stages(0);
    if call_section_routine(wt::FN_WC_CONCACAF_CUP_COLLECT_ENTRANTS, this) == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x41f);
        return 0;
    }
    call_cup_virtual(cup, comp_vtable::SET_PREDICTED_POSITIONS);
    if call_section_routine(wt::FN_WC_CONCACAF_CUP_SET_UP_SEEDS, this) == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x429);
        return 0;
    }
    if call_cup_virtual(cup, comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x430);
        return 0;
    }
    if call_cup_virtual(cup, cup_vtable::MAP_FIXTURE_TREE) == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x437);
        return 0;
    }
    call_section_routine(wt::FN_WC_CONCACAF_CUP_CREATE_CENTRAL_AMERICAN_GROUPS, this) as u32
}

/// Comparer for `wc_concacaf_cup_start_new_season` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_CONCACAF_CUP_START_NEW_SEASON);
    compare_qualifying_turnover("wc_concacaf_cup_start_new_season", League(this), || o(this))
}

/// A no-argument thiscall routine (ret 0) of a qualifying section; its signed eax.
pub(crate) unsafe fn call_section_routine(routine: usize, this: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(routine);
    f(this)
}
