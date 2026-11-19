// ---------- batch %%LEVA%%: the World Cup qualifying turnovers of Europe, Africa and CONCACAF (wc_europe_league.cpp, wc_african_cup.cpp, wc_concacaf_cup.cpp) ----------

/// Constants of the European, African and CONCACAF qualifying turnovers.
pub(crate) mod wc_cup_turnover_consts {
    /// "E:\dev\CM3\cm3 00-01\cm3\code\comp\worldcup\wc_europe_league.cpp" (the error boxes).
    pub(crate) const WC_EUROPE_LEAGUE_CPP: usize = 0xad4a6c;
    /// "E:\dev\CM3\cm3 00-01\cm3\code\comp\worldcup\wc_african_cup.cpp".
    pub(crate) const WC_AFRICAN_CUP_CPP: usize = 0xad443c;
    /// "E:\dev\CM3\cm3 00-01\cm3\code\comp\worldcup\wc_concacaf_cup.cpp".
    pub(crate) const WC_CONCACAF_CUP_CPP: usize = 0xad4850;
    /// The European section's list object (+0x10, a 0xe-byte LLIST): deleted and made again at the turnover [inference:
    /// its users are not ported].
    pub(crate) const EUROPE_LIST: usize = 0x10;
    /// A European section dword cleared at the turnover (+0xfb) [inference: its reader is not ported].
    pub(crate) const EUROPE_FB: usize = 0xfb;
    /// The African and CONCACAF sections' entrant list (+0xb2, a pointer past the Cup body), freed at the turnover.
    pub(crate) const CUP_ENTRANT_LIST: usize = 0xb2;
    /// The African section's format byte (+0xba): 0 or 3 give 5 stages, 1 gives 4, any other 7 [inference: set by
    /// the entrant routine].
    pub(crate) const AFRICAN_FORMAT: usize = 0xba;
    /// The routines the turnovers call (thiscall, ret 0).
    pub(crate) const FN_WC_EUROPE_LEAGUE_BUILD_ENTRY_POOL: usize = 0x919e30;
    pub(crate) const FN_WC_AFRICAN_CUP_COLLECT_ENTRANTS: usize = 0x9106a0;
    pub(crate) const FN_WC_AFRICAN_CUP_SET_UP_FIRST_ROUND: usize = 0x910120;
    pub(crate) const FN_WC_CONCACAF_CUP_COLLECT_ENTRANTS: usize = 0x916250;
    pub(crate) const FN_WC_CONCACAF_CUP_SET_UP_SEEDS: usize = 0x915d40;
    pub(crate) const FN_WC_CONCACAF_CUP_CREATE_CENTRAL_AMERICAN_GROUPS: usize = 0x916cd0;
}

/// A no-argument thiscall routine (ret 0) of a qualifying section; its signed eax.
unsafe fn call_section_routine(routine: usize, this: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(routine);
    f(this)
}

/// The season turnover of the European World Cup qualifying: last campaign's table, round dates, list and stages freed,
/// the season archived, the next campaign's year, the entry pool again, as many stages as its format needs, the rules
/// and a new list.
///
/// The season reopened (`.set_season_closed(0)`), the table freed (`exe::league_free_table`(this, 0), thiscall, ret 4),
/// the round dates (`.round_dates()`) freed and zeroed; the list (`EUROPE_LIST`), when there, destroyed (LLIST::~LLIST
/// `exe::drop_llist`, thiscall) and deleted (`exe::crt_operator_delete`, cdecl); the season archived
/// (`exe::comp_stats_archive_season_and_reset`(`.comp_stats()`, 1)); the year up by four until it reaches the game's
/// (`advance_qualifying_campaign`); `.set_active(0)` and `EUROPE_FB` = 0; the entry pool
/// (WcEuropeLeague::build_entry_pool, thiscall, ret 0; its return not read); the stages destroyed
/// (`destroy_comp_stages`) and their vector freed (not zeroed); the stage count by the exempt count
/// (`WC_EUROPE_EXEMPT`): 0 → 11, 1 or 2 → 10, 3 → 9, any other kept; a new vector (`exe::crt_malloc`(4 × count),
/// cdecl), the current stage −1; the rules (virtual `comp_vtable::SET_UP_RULES`; 0: the box of line 0x36a and the copy
/// returns 0, the original the box's eax); then a new list (`exe::crt_operator_new(0xe)` and LLIST::LLIST
/// `exe::new_llist`, or 0 when out of memory) in `EUROPE_LIST`, which it returns (the original keeps the list's
/// construction under an SEH frame; nothing here throws).
/// Original: `WcEuropeLeague::start_new_season` (0x919430, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_start_new_season(this: usize) -> u32 {
    use wc_cup_turnover_consts as wt;
    let this = League(this);
    qualifying_turnover_free_table(this);
    let list = read_usize(this.addr() + wt::EUROPE_LIST);
    if list != 0 {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
    }
    if this.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(this.comp_stats(), 1);
    }
    advance_qualifying_campaign(this.addr() + league::YEAR);
    covered!("mwg_13 european qualifying rollover");
    this.set_active(0);
    write_u32(this.addr() + wt::EUROPE_FB, 0);
    call_section_routine(wt::FN_WC_EUROPE_LEAGUE_BUILD_ENTRY_POOL, this.addr());
    destroy_comp_stages(this);
    exe::crt_free(this.stages());
    match read_u8(this.addr() + WC_EUROPE_EXEMPT) {
        0 => this.set_stage_count(0xb),
        1 | 2 => this.set_stage_count(0xa),
        3 => this.set_stage_count(9),
        _ => {}
    }
    this.set_stages(exe::crt_malloc((this.stage_count() as u32) << 2));
    this.set_current_stage(usize::MAX);
    if call_section_routine(read_usize(read_usize(this.addr()) + comp_vtable::SET_UP_RULES), this.addr()) == 0 {
        show_error_at(wt::WC_EUROPE_LEAGUE_CPP, 0x36a);
        return 0;
    }
    let p = exe::crt_operator_new(0xe);
    let list = if p != 0 { exe::new_llist(p) } else { 0 };
    write_usize(this.addr() + wt::EUROPE_LIST, list);
    list as u32
}

/// Comparer for `wc_europe_league_start_new_season` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_qualifying_turnover("wc_europe_league_start_new_season", League(this), || o(this))
}

/// The common first half of the African and CONCACAF qualifying turnovers (Cup objects): the team, round and tie tables
/// freed and zeroed (as the cup turnover template), the entrant list (`CUP_ENTRANT_LIST`) freed (not zeroed; the
/// African one even when null: `free(0)` does nothing) and the stages destroyed (`destroy_comp_stages`).
unsafe fn qualifying_cup_free_tables(cup: Cup) {
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_qualifying_turnover("wc_african_cup_start_new_season", League(this), || o(this))
}

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_qualifying_turnover("wc_concacaf_cup_start_new_season", League(this), || o(this))
}
