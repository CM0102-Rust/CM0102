use super::*;

/// The season turnover of the African World Cup qualifying: last campaign's tables, entrant list and stages freed, the
/// season archived, the next campaign's year, the entrants again, as many stages as their format needs, the predicted
/// positions, the first round, the round table and the tie tree.
///
/// `qualifying_cup_free_tables`, the stage vector (`cup::STAGES`) freed (not zeroed) and `cup::CURRENT_STAGE` = −1; the
/// season archived (`exe::comp_stats_archive_season_and_reset`, +8, thiscall, ret 4); the year up by four until it
/// reaches the game's (`advance_qualifying_campaign`); `cup::CURRENT_ROUND` = 0, `cup::SEASON_SET_UP` = 1 and
/// `cup::RUN_BY_STAGES` = 0. Then, each with its own box (wc_african_cup.cpp): the entrants (WcAfricanCup 0x9106a0,
/// thiscall, ret 0) when 0 (line 0x36a); the stage count by `AFRICAN_FORMAT` (0 or 3 → 5, 1 → 4, other → 7) in a new
/// vector (`exe::crt_malloc`(4 × count)); the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`); the
/// first round (WcAfricanCup 0x910120, thiscall, ret 0; negative: line 0x37f), the round table (virtual
/// `comp_vtable::SET_UP_RULES`; negative: 0x386) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`; negative:
/// 0x38d), whose eax it returns. After a box the copy returns 0 (the original the box's eax).
/// Original: `WcAfricanCup::start_new_season` (0x910410, wc_african_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_start_new_season(this: usize) -> u32 {
    use wc_cup_turnover_consts as wt;
    let cup = Cup(this);
    qualifying_cup_free_tables(cup);
    exe::crt_free(cup.stages());
    cup.set_current_stage(usize::MAX);
    let stats = read_usize(this + 8);
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    advance_qualifying_campaign(this + cup::YEAR);
    covered!("mwg_13 african qualifying rollover");
    cup.set_current_round(0);
    cup.set_season_set_up(1);
    cup.set_run_by_stages(0);
    if call_section_routine(wt::FN_WC_AFRICAN_CUP_COLLECT_ENTRANTS, this) == 0 {
        show_error_at(wt::WC_AFRICAN_CUP_CPP, 0x36a);
        return 0;
    }
    let stages: usize = match read_u8(this + wt::AFRICAN_FORMAT) {
        0 | 3 => 5,
        1 => 4,
        _ => 7,
    };
    cup.set_stage_count(stages);
    cup.set_stages(exe::crt_malloc((stages as u32) << 2));
    call_cup_virtual(cup, comp_vtable::SET_PREDICTED_POSITIONS);
    if call_section_routine(wt::FN_WC_AFRICAN_CUP_SET_UP_FIRST_ROUND, this) < 0 {
        show_error_at(wt::WC_AFRICAN_CUP_CPP, 0x37f);
        return 0;
    }
    if call_cup_virtual(cup, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(wt::WC_AFRICAN_CUP_CPP, 0x386);
        return 0;
    }
    let r = call_cup_virtual(cup, cup_vtable::MAP_FIXTURE_TREE);
    if r < 0 {
        show_error_at(wt::WC_AFRICAN_CUP_CPP, 0x38d);
        return 0;
    }
    r as u32
}

/// Comparer for `wc_african_cup_start_new_season` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_AFRICAN_CUP_START_NEW_SEASON);
    compare_qualifying_turnover("wc_african_cup_start_new_season", League(this), || o(this))
}

/// A World Cup qualifying section's next campaign: the year goes up in steps of four until it is no longer before the
/// game's year (the section's year a signed word, the game's [`G_CURRENT_DATE_YEAR`] an unsigned one, read again on
/// each step).
pub(crate) unsafe fn advance_qualifying_campaign(year_at: usize) {
    while (read_i16(year_at) as i32) < read_u16(G_CURRENT_DATE_YEAR) as i32 {
        write_i16(year_at, read_i16(year_at).wrapping_add(4));
    }
}

/// The plan of a qualifying turnover (only the original runs: it frees and rebuilds): the year went up by a multiple
/// of four and is no longer before the game's year, and the season is open.
pub(crate) unsafe fn compare_qualifying_turnover(name: &str, this: League, original: impl FnOnce() -> u32) -> u32 {
    let before = this.year() as i32;
    let r = original();
    let after = this.year() as i32;
    let ok = after >= before && (after - before) % 4 == 0 && after >= read_u16(G_CURRENT_DATE_YEAR) as i32 && this.season_closed() == 0;
    SCOREBOARD.record(ok, || format!("{}({:#x}): year {} -> {}, +0x4c {}", name, this.addr(), before, after, this.season_closed()));
    r
}

/// The common first half of the African and CONCACAF qualifying turnovers (Cup objects): the team, round and tie tables
/// freed and zeroed (as the cup turnover template), the entrant list (`CUP_ENTRANT_LIST`) freed (not zeroed; the
/// African one even when null: `free(0)` does nothing) and the stages destroyed (`destroy_comp_stages`).
pub(crate) unsafe fn qualifying_cup_free_tables(cup: Cup) {
    cup.set_season_closed(0);
    if cup.team_table() != 0 {
        exe::crt_free(cup.team_table());
        cup.set_team_table(0);
    }
    if cup.round_table() != 0 {
        exe::crt_free(cup.round_table());
        cup.set_round_table(0);
    }
    if cup.tie_tables() != 0 {
        let mut i = 0i32;
        while i < cup.round_count() as i32 {
            let p = read_usize(cup.tie_tables() + i as usize * 4);
            if p != 0 {
                exe::crt_free(p);
                write_usize(cup.tie_tables() + i as usize * 4, 0);
            }
            i += 1;
        }
        exe::crt_free(cup.tie_tables());
        cup.set_tie_tables(0);
    }
    let list = read_usize(cup.addr() + wc_cup_turnover_consts::CUP_ENTRANT_LIST);
    if list != 0 {
        exe::crt_free(list);
    }
    destroy_comp_stages(League(cup.addr()));
}
