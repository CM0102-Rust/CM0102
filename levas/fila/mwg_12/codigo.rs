// ---------- batch %%LEVA%%: the World Cup qualifying turnovers of Asia, South America and Oceania (wc_asia_league.cpp, wc_south_american_league.cpp, wc_oceania_league.cpp) ----------

/// Constants of the World Cup qualifying turnovers.
pub(crate) mod wc_turnover_consts {
    /// "E:\dev\CM3\cm3 00-01\cm3\code\comp\worldcup\wc_asia_league.cpp" (the error boxes).
    pub(crate) const WC_ASIA_LEAGUE_CPP: usize = 0xad4680;
    /// "E:\dev\CM3\cm3 00-01\cm3\code\comp\worldcup\wc_south_american_league.cpp".
    pub(crate) const WC_SOUTH_AMERICAN_LEAGUE_CPP: usize = 0xad4ce0;
    /// "E:\dev\CM3\cm3 00-01\cm3\code\comp\worldcup\wc_oceania_league.cpp".
    pub(crate) const WC_OCEANIA_LEAGUE_CPP: usize = 0xad4afc;
    /// The national-team list of the Asian and Oceania sections (+0xef, a pointer; built by their entrant routines).
    pub(crate) const ENTRANT_LIST: usize = 0xef;
    /// The routines the turnovers call (thiscall, ret 0; already Rust, called through the exe).
    pub(crate) const FN_WC_ASIA_LEAGUE_BUILD_ENTRANT_LIST: usize = 0x913f60;
    pub(crate) const FN_WC_SOUTH_AMERICAN_LEAGUE_COUNT_EXEMPT_NATIONS: usize = 0x920d10;
    pub(crate) const FN_WC_SOUTH_AMERICAN_LEAGUE_BUILD_TEAM_SLOTS: usize = 0x920eb0;
    pub(crate) const FN_WC_OCEANIA_LEAGUE_BUILD_ENTRANT_POOL: usize = 0x91fa80;
    pub(crate) const FN_WC_OCEANIA_LEAGUE_INIT_GROUP_TABLE: usize = 0x91ec00;
}

/// A World Cup qualifying section's next campaign: the year goes up in steps of four until it is no longer before the
/// game's year (the section's year a signed word, the game's [`G_CURRENT_DATE_YEAR`] an unsigned one, read again on
/// each step).
pub(crate) unsafe fn advance_qualifying_campaign(year_at: usize) {
    while (read_i16(year_at) as i32) < read_u16(G_CURRENT_DATE_YEAR) as i32 {
        write_i16(year_at, read_i16(year_at).wrapping_add(4));
    }
}

/// Destroys the stages 0..=`current_stage` (signed, read again on each pass) of a competition (virtual 0 with 1: the
/// destructor that frees), as the turnovers do; the stage vector itself stays.
pub(crate) unsafe fn destroy_comp_stages(this: League) {
    let mut i = 0i32;
    while (this.current_stage() as i32) >= 0 && i <= this.current_stage() as i32 {
        let f = read_usize(this.stages() + i as usize * 4);
        if f != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
            destroy(f, 1);
        }
        i += 1;
    }
}

/// The start of a qualifying section's turnover: the season reopened (`.set_season_closed(0)`), the table freed (`exe::league_free_table` League::free_table: push 0; mov ecx, this -> thiscall, ret 4) and the
/// round dates (`.round_dates()`) freed (`exe::crt_free`) and zeroed.
pub(crate) unsafe fn qualifying_turnover_free_table(this: League) {
    this.set_season_closed(0);
    exe::league_free_table(this, 0);
    if this.round_dates() != 0 {
        exe::crt_free(this.round_dates());
        this.set_round_dates(0);
    }
}

/// The season turnover of the Asian World Cup qualifying: last campaign's table, round dates, national-team list and
/// stages freed, the season archived, the next campaign's year, the national teams again and the rules.
///
/// `qualifying_turnover_free_table`; the national-team list (`ENTRANT_LIST`, +0xef) freed when there (not zeroed: the
/// entrant routine writes it again); the stages destroyed (`destroy_comp_stages`) and `.set_current_stage()` = −1; the
/// season archived in the statistics (`exe::comp_stats_archive_season_and_reset`(`.comp_stats()`, 1), thiscall, ret 4);
/// the year up by four until it reaches the game's (`advance_qualifying_campaign`); `.set_active(0)` and the current
/// stage −1 again; the national teams (WcAsiaLeague::build_entrant_list, thiscall, ret 0; its return not read) and the
/// rules (virtual `comp_vtable::SET_UP_RULES`): 0 shows the box (wc_asia_league.cpp, line 0x45e) and the copy returns
/// 0 (the original the box's eax); otherwise its eax.
/// Original: `WcAsiaLeague::start_new_season` (0x913360, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_start_new_season(this: usize) -> u32 {
    use wc_turnover_consts as wt;
    let this = League(this);
    qualifying_turnover_free_table(this);
    let list = read_usize(this.addr() + wt::ENTRANT_LIST);
    if list != 0 {
        exe::crt_free(list);
    }
    destroy_comp_stages(this);
    this.set_current_stage(usize::MAX);
    if this.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(this.comp_stats(), 1);
    }
    advance_qualifying_campaign(this.addr() + league::YEAR);
    covered!("mwg_12 asian qualifying rollover");
    this.set_active(0);
    this.set_current_stage(usize::MAX);
    call_qualifier_routine(wt::FN_WC_ASIA_LEAGUE_BUILD_ENTRANT_LIST, this.addr());
    let r = call_qualifier_routine(read_usize(read_usize(this.addr()) + comp_vtable::SET_UP_RULES), this.addr());
    if r == 0 {
        show_error_at(wt::WC_ASIA_LEAGUE_CPP, 0x45e);
        return 0;
    }
    r
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

/// Comparer for `wc_asia_league_start_new_season` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_qualifying_turnover("wc_asia_league_start_new_season", League(this), || o(this))
}

/// The season turnover of the South American World Cup qualifying: last campaign's table and round dates freed, the
/// season archived, the next campaign's year, the exempt nations counted again, the rules and the team slots.
///
/// `qualifying_turnover_free_table` (no stages: the South American section is one league); the season archived
/// (`exe::comp_stats_archive_season_and_reset`(`.comp_stats()`, 1)); the year up by four until it reaches the game's
/// (`advance_qualifying_campaign`); `.set_active(0)`; the exempt nations (WcSouthAmericanLeague::count_exempt_nations,
/// thiscall, ret 0; its return not read); the rules (virtual `comp_vtable::SET_UP_RULES`; 0: the box of line 0x2e1) and
/// the team slots (WcSouthAmericanLeague::build_team_slots, thiscall, ret 0; 0: the box of line 0x2e8), whose eax it
/// returns. After a box the copy returns 0 (the original the box's eax).
/// Original: `WcSouthAmericanLeague::start_new_season` (0x921470, wc_south_american_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_start_new_season(this: usize) -> u32 {
    use wc_turnover_consts as wt;
    let this = League(this);
    qualifying_turnover_free_table(this);
    if this.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(this.comp_stats(), 1);
    }
    advance_qualifying_campaign(this.addr() + league::YEAR);
    covered!("mwg_12 south american qualifying rollover");
    this.set_active(0);
    call_qualifier_routine(wt::FN_WC_SOUTH_AMERICAN_LEAGUE_COUNT_EXEMPT_NATIONS, this.addr());
    if call_qualifier_routine(read_usize(read_usize(this.addr()) + comp_vtable::SET_UP_RULES), this.addr()) == 0 {
        show_error_at(wt::WC_SOUTH_AMERICAN_LEAGUE_CPP, 0x2e1);
        return 0;
    }
    let r = call_qualifier_routine(wt::FN_WC_SOUTH_AMERICAN_LEAGUE_BUILD_TEAM_SLOTS, this.addr());
    if r == 0 {
        show_error_at(wt::WC_SOUTH_AMERICAN_LEAGUE_CPP, 0x2e8);
        return 0;
    }
    r
}

/// Comparer for `wc_south_american_league_start_new_season` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_qualifying_turnover("wc_south_american_league_start_new_season", League(this), || o(this))
}

/// The season turnover of the Oceania World Cup qualifying: last campaign's table, round dates, national-team pool and
/// stages freed, the season archived, the next campaign's year, the pool and the predicted positions again, three new
/// stages, the rules and the group table.
///
/// `qualifying_turnover_free_table`; the national-team pool (`ENTRANT_LIST`, +0xef) freed when there (not zeroed); the
/// stages destroyed (`destroy_comp_stages`) and the stage vector (`.stages()`) freed (not zeroed); the current stage −1;
/// the season archived (`exe::comp_stats_archive_season_and_reset`(`.comp_stats()`, 1)); the year up by four until it
/// reaches the game's (`advance_qualifying_campaign`); `.set_active(0)` and the current stage −1 again; the pool
/// (WcOceaniaLeague::build_entrant_pool, thiscall, ret 0; its return not read), the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`), 3 stages (`.set_stage_count(3)`) in a new vector (`exe::crt_malloc(0xc)`,
/// cdecl); the rules (virtual `comp_vtable::SET_UP_RULES`; 0: the box of line 0x305) and the group table
/// (WcOceaniaLeague::init_group_table, thiscall, ret 0; 0: the box of line 0x30c), whose eax it returns. After a box
/// the copy returns 0 (the original the box's eax).
/// Original: `WcOceaniaLeague::start_new_season` (0x91ef40, wc_oceania_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_start_new_season(this: usize) -> u32 {
    use wc_turnover_consts as wt;
    let this = League(this);
    qualifying_turnover_free_table(this);
    let pool = read_usize(this.addr() + wt::ENTRANT_LIST);
    if pool != 0 {
        exe::crt_free(pool);
    }
    destroy_comp_stages(this);
    exe::crt_free(this.stages());
    this.set_current_stage(usize::MAX);
    if this.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(this.comp_stats(), 1);
    }
    advance_qualifying_campaign(this.addr() + league::YEAR);
    covered!("mwg_12 oceania qualifying rollover");
    this.set_active(0);
    this.set_current_stage(usize::MAX);
    call_qualifier_routine(wt::FN_WC_OCEANIA_LEAGUE_BUILD_ENTRANT_POOL, this.addr());
    call_qualifier_routine(read_usize(read_usize(this.addr()) + comp_vtable::SET_PREDICTED_POSITIONS), this.addr());
    this.set_stage_count(3);
    this.set_stages(exe::crt_malloc(0xc) as usize);
    if call_qualifier_routine(read_usize(read_usize(this.addr()) + comp_vtable::SET_UP_RULES), this.addr()) == 0 {
        show_error_at(wt::WC_OCEANIA_LEAGUE_CPP, 0x305);
        return 0;
    }
    let r = call_qualifier_routine(wt::FN_WC_OCEANIA_LEAGUE_INIT_GROUP_TABLE, this.addr());
    if r == 0 {
        show_error_at(wt::WC_OCEANIA_LEAGUE_CPP, 0x30c);
        return 0;
    }
    r
}

/// Comparer for `wc_oceania_league_start_new_season` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_qualifying_turnover("wc_oceania_league_start_new_season", League(this), || o(this))
}
