// ---------- batch 314: the season turnovers of the European Super Cup, the European Cup and the UEFA Cup (eur_super_cup.cpp, european_cup.cpp, uefa_cup.cpp) ----------

/// Constants of the European cups' season turnovers.
pub(crate) mod european_turnover_consts {
    /// "E:\dev\CM3\cm3 00-01\cm3\code\comp\eurocomp\eur_super_cup.cpp" (the error boxes).
    pub(crate) const EUR_SUPER_CUP_CPP: usize = 0x9c45f8;
    /// "E:\dev\CM3\cm3 00-01\cm3\code\comp\eurocomp\european_cup.cpp".
    pub(crate) const EUROPEAN_CUP_CPP: usize = 0x9c4900;
    /// "E:\dev\CM3\cm3 00-01\cm3\code\comp\eurocomp\uefa_cup.cpp".
    pub(crate) const UEFA_CUP_CPP: usize = 0xaa19e0;
}

/// The first half of a European cup's season turnover: last season's tables freed, the stages destroyed (when asked),
/// the season archived and the year moved on by one.
///
/// `cup::SEASON_CLOSED` = 0; the team table (`cup::TEAM_TABLE`) freed (`exe::crt_free`, cdecl) and zeroed; with
/// `entrant_book`, the entrant book (`cup::ENTRANT_BOOK`) freed and zeroed; the round table (`cup::ROUND_TABLE`) freed
/// and zeroed; with tie tables (`cup::TIE_TABLES`), each of the `cup::ROUND_COUNT` rounds' (a signed word, read again on
/// each pass) freed and zeroed, then the array freed and zeroed. With `stages`, the stages 0..=`cup::CURRENT_STAGE`
/// (signed, read again on each pass) of `cup::STAGES` destroyed (virtual 0 with 1: the destructor that frees) and
/// `cup::CURRENT_STAGE` = −1. The season archived in the statistics (+8, when there:
/// `exe::comp_stats_archive_season_and_reset`(stats, 1), thiscall, ret 4); the year (`cup::YEAR`) up by one;
/// `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. The frees keep the original's order (the allocator is
/// reproducible).
pub(crate) unsafe fn european_cup_free_season(cup: Cup, entrant_book: bool, stages: bool) {
    cup.set_season_closed(0);
    if cup.team_table() != 0 {
        exe::crt_free(cup.team_table());
        cup.set_team_table(0);
    }
    if entrant_book && cup.entrant_book() != 0 {
        exe::crt_free(cup.entrant_book());
        cup.set_entrant_book(0);
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
    if stages {
        let mut i = 0i32;
        while (cup.current_stage() as i32) >= 0 && i <= cup.current_stage() as i32 {
            let f = read_usize(cup.stages() + i as usize * 4);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
        }
        cup.set_current_stage(usize::MAX);
    }
    let stats = read_usize(cup.addr() + 8);
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    cup.set_year(cup.year().wrapping_add(1)); // mwg9: the next season's European cup
    cup.set_current_round(0);
    cup.set_run_by_stages(0);
}

/// The UEFA seeding's season update (when the seeding object exists): UefaSeeding::update_season(0)
/// (`exe::uefaseeding_update_season`, push 0; mov ecx, [`G_UEFA_SEEDING`] -> thiscall, ret 4).
pub(crate) unsafe fn european_turnover_update_seeding() {
    let seeding = read_usize(G_UEFA_SEEDING);
    if seeding != 0 {
        exe::uefaseeding_update_season(seeding, 0); // mwg9: the seeding's new season
    }
}

/// The season turnover of the European Super Cup: last season's tables freed, the season archived, the year moved on by
/// one, the two clubs picked, the round table and the tie tree rebuilt, the first round set up and the predicted
/// positions.
///
/// `european_cup_free_season` without the entrant book and the stages. Then, each with its own box (eur_super_cup.cpp,
/// lines 0x118, 0x11f and 0x126) when it returns a negative value: the two clubs (`exe::eursupercup_select_teams`
/// EurSuperCup::select_teams, thiscall, ret 0), the round table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree
/// (virtual `cup_vtable::MAP_FIXTURE_TREE`). Then the first round (`exe::cup_set_up_round` Cup::set_up_round(0), push
/// 0; mov ecx -> thiscall, ret 4) and the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax
/// it returns. After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read
/// it).
/// Original: `EurSuperCup::start_new_season` (0x57a6d0, eur_super_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eur_super_cup_start_new_season(this: usize) -> u32 {
    use european_turnover_consts as et;
    let this = Cup(this);
    european_cup_free_season(this, false, false);
    covered!("mwg_9 european super cup rollover");
    if exe::eursupercup_select_teams(this) < 0 {
        show_error_at(et::EUR_SUPER_CUP_CPP, 0x118);
        return 0;
    }
    if call_cup_virtual(this, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(et::EUR_SUPER_CUP_CPP, 0x11f);
        return 0;
    }
    if call_cup_virtual(this, cup_vtable::MAP_FIXTURE_TREE) < 0 {
        show_error_at(et::EUR_SUPER_CUP_CPP, 0x126);
        return 0;
    }
    exe::cup_set_up_round(this, 0);
    call_cup_virtual(this, comp_vtable::SET_PREDICTED_POSITIONS) as u32
}

/// Comparer for `eur_super_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must
/// go up by one.
pub(crate) unsafe extern "thiscall" fn compare_eur_super_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_EUR_SUPER_CUP_START_NEW_SEASON);
    compare_cup_new_season("eur_super_cup_start_new_season", Cup(this), || o(this))
}

/// The season turnover of the European Cup (the Champions League): last season's tables, entrant book and stages freed,
/// the season archived, the year moved on by one, the UEFA seeding's new season, the entrants and the qualifying entry
/// table collected, the round table and the tie tree rebuilt and the predicted positions.
///
/// `european_cup_free_season` with the entrant book (`cup::ENTRANT_BOOK`) and the stages (destroyed, then
/// `cup::CURRENT_STAGE` = −1); then `european_turnover_update_seeding`. Then, each with its own box (european_cup.cpp):
/// the entrants (`exe::europeancup_collect_entrants` EuropeanCup::collect_entrants, thiscall, ret 0) when it returns 0
/// (line 0x40c: here zero is the failure, not a negative value), the qualifying entry table
/// (`exe::europeancup_build_qualifying_entry_table`, thiscall, ret 0) when negative (line 0x413), the round table
/// (virtual `comp_vtable::SET_UP_RULES`, line 0x41a) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`, line
/// 0x421) when negative. Finally the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax it
/// returns. After a box the copy returns 0 (the original returns the box's eax; the caller does not read it).
/// Original: `EuropeanCup::start_new_season` (0x584d60, european_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn european_cup_start_new_season(this: usize) -> u32 {
    use european_turnover_consts as et;
    let this = Cup(this);
    european_cup_free_season(this, true, true);
    european_turnover_update_seeding();
    covered!("mwg_9 european cup rollover");
    if exe::europeancup_collect_entrants(this) == 0 {
        show_error_at(et::EUROPEAN_CUP_CPP, 0x40c);
        return 0;
    }
    if exe::europeancup_build_qualifying_entry_table(this) < 0 {
        show_error_at(et::EUROPEAN_CUP_CPP, 0x413);
        return 0;
    }
    if call_cup_virtual(this, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(et::EUROPEAN_CUP_CPP, 0x41a);
        return 0;
    }
    if call_cup_virtual(this, cup_vtable::MAP_FIXTURE_TREE) < 0 {
        show_error_at(et::EUROPEAN_CUP_CPP, 0x421);
        return 0;
    }
    call_cup_virtual(this, comp_vtable::SET_PREDICTED_POSITIONS) as u32
}

/// Comparer for `european_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must
/// go up by one.
pub(crate) unsafe extern "thiscall" fn compare_european_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_EUROPEAN_CUP_START_NEW_SEASON);
    compare_cup_new_season("european_cup_start_new_season", Cup(this), || o(this))
}

/// The season turnover of the UEFA Cup: last season's tables and entrant book freed, the season archived, the year moved
/// on by one, the UEFA seeding's new season, the entrants built, the round table and the tie tree rebuilt and the
/// predicted positions.
///
/// `european_cup_free_season` with the entrant book (`cup::ENTRANT_BOOK`), without the stages; then
/// `european_turnover_update_seeding`. Then, each with its own box (uefa_cup.cpp, lines 0x273, 0x27a and 0x281) when it
/// returns a negative value: the entrants (`exe::uefacup_build_entrants` UefaCup::build_entrants, thiscall, ret 0), the
/// round table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`). Finally
/// the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax it returns. After a box the copy
/// returns 0 (the original returns the box's eax; the caller does not read it).
/// Original: `UefaCup::start_new_season` (0x902ee0, uefa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn uefa_cup_start_new_season(this: usize) -> u32 {
    use european_turnover_consts as et;
    let this = Cup(this);
    european_cup_free_season(this, true, false);
    european_turnover_update_seeding();
    covered!("mwg_9 uefa cup rollover");
    if exe::uefacup_build_entrants(this) < 0 {
        show_error_at(et::UEFA_CUP_CPP, 0x273);
        return 0;
    }
    if call_cup_virtual(this, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(et::UEFA_CUP_CPP, 0x27a);
        return 0;
    }
    if call_cup_virtual(this, cup_vtable::MAP_FIXTURE_TREE) < 0 {
        show_error_at(et::UEFA_CUP_CPP, 0x281);
        return 0;
    }
    call_cup_virtual(this, comp_vtable::SET_PREDICTED_POSITIONS) as u32
}

/// Comparer for `uefa_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go
/// up by one.
pub(crate) unsafe extern "thiscall" fn compare_uefa_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_UEFA_CUP_START_NEW_SEASON);
    compare_cup_new_season("uefa_cup_start_new_season", Cup(this), || o(this))
}

// ---------- batch 314: the season turnovers of the Intertoto Cup and the World Club Cup; the UEFA Cup's rounds (intertoto_cup.cpp, uefa_cup.cpp, world_club_cup.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\comp\eurocomp\intertoto_cup.cpp" (the error boxes).
pub(crate) const INTERTOTO_CUP_CPP: usize = 0x9e9e14;
/// "E:\dev\CM3\cm3 00-01\cm3\code\comp\eurocomp\world_club_cup.cpp".
pub(crate) const WORLD_CLUB_CUP_CPP: usize = 0xad5848;
/// WorldClubCup::select_teams (thiscall, ret 0): the World Club Cup's clubs.
pub(crate) const FN_WORLDCLUBCUP_SELECT_TEAMS: usize = 0x92b8b0;
/// The UEFA Cup's prize fund (3,000,000).
pub(crate) const UEFA_CUP_PRIZE_FUND: i32 = 3_000_000;

/// The season turnover of the Intertoto Cup: last season's tables freed, the season archived, the year moved on by one,
/// the winners not yet sent to the UEFA Cup, the UEFA seeding's new season, the entrants collected, the round table and
/// the tie tree rebuilt, the predicted positions and the first round set up.
///
/// The first half of the cup turnover (`cup_turnover_free_season`: the team, round and tie tables freed and zeroed, the
/// season archived, the year up by one, `cup::CURRENT_ROUND` and `cup::RUN_BY_STAGES` = 0), then
/// `intertoto_cup::WINNERS_SENT_TO_UEFA` = 0 (the original writes it between the two bytes) and, with the UEFA seeding
/// object ([`G_UEFA_SEEDING`]), UefaSeeding::update_season(0) (`exe::uefaseeding_update_season`, thiscall, ret 4). Then, each
/// with its own box (intertoto_cup.cpp, lines 0x1e9, 0x1f0 and 0x1f7) when it returns a negative value: the entrants
/// (`exe::intertotocup_collect_entrants` IntertotoCup::collect_entrants, thiscall, ret 0), the round table (virtual
/// `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`). Then the predicted positions
/// (virtual `comp_vtable::SET_PREDICTED_POSITIONS`) and the first round (`exe::cup_set_up_round` Cup::set_up_round(0),
/// push 0; mov ecx -> thiscall, ret 4), whose eax it returns. After a box the copy returns 0 (the original returns the
/// box's eax; the caller does not read it).
/// Original: `IntertotoCup::start_new_season` (0x633920, intertoto_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn intertoto_cup_start_new_season(this: usize) -> u32 {
    let this = Cup(this);
    cup_turnover_free_season(this);
    write_u32(this.addr() + intertoto_cup::WINNERS_SENT_TO_UEFA, 0); // mwg10: the winners not yet sent on
    let seeding = read_usize(G_UEFA_SEEDING);
    if seeding != 0 {
        exe::uefaseeding_update_season(seeding, 0);
    }
    covered!("mwg_10 intertoto rollover");
    if exe::intertotocup_collect_entrants(this) < 0 {
        show_error_at(INTERTOTO_CUP_CPP, 0x1e9);
        return 0;
    }
    if call_cup_virtual(this, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(INTERTOTO_CUP_CPP, 0x1f0);
        return 0;
    }
    if call_cup_virtual(this, cup_vtable::MAP_FIXTURE_TREE) < 0 {
        show_error_at(INTERTOTO_CUP_CPP, 0x1f7);
        return 0;
    }
    call_cup_virtual(this, comp_vtable::SET_PREDICTED_POSITIONS);
    exe::cup_set_up_round(this, 0) // mwg10: the first round set up
}

/// Comparer for `intertoto_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must
/// go up by one.
pub(crate) unsafe extern "thiscall" fn compare_intertoto_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_INTERTOTO_CUP_START_NEW_SEASON);
    compare_cup_new_season("intertoto_cup_start_new_season", Cup(this), || o(this))
}

/// Sets up the rounds of the UEFA Cup. Rule: the prize fund is 3,000,000; the round table comes from
/// `comp_vtable::CREATE_SCHEDULE`.
///
/// Ret 0; virtual +0x8c: `UefaCup::set_up_rounds()`; returns 1. The prize fund (`.set_prize_fund()`) of 3,000,000 and the
/// round table (`.set_round_table()`, the old one not freed) from `comp_vtable::CREATE_SCHEDULE` (push 0; push
/// &`cup::STAGE_LABEL`; push &`cup::ROUND_COUNT`; push −1; mov ecx, this -> thiscall, ret 0x10; the original writes the
/// fund after pushing the arguments, before the call). The same routine as the CONCACAF Champions' Cup's with another
/// fund.
/// Original: `UefaCup::set_up_rounds` (0x9030b0, uefa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn uefa_cup_set_up_rounds(this: usize) -> u32 {
    let this = Cup(this);
    this.set_prize_fund(UEFA_CUP_PRIZE_FUND); // mwg10: the UEFA Cup's prize fund
    let table: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = table(this.addr(), 0xffff_ffff, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0);
    this.set_round_table(t);
    covered!("mwg_10 uefa cup rounds");
    1
}

// simplification: virtual +0x3c allocates the round table: it does not run twice. Original only; the plan: the prize
// fund (+0x28) of 3,000,000 and the return 1.
/// Comparer for `uefa_cup_set_up_rounds`.
pub(crate) unsafe extern "thiscall" fn compare_uefa_cup_set_up_rounds(this: usize) -> u32 {
    let this = Cup(this);
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_UEFA_CUP_SET_UP_ROUNDS)(this.addr());
    SCOREBOARD.record(r == 1 && this.prize_fund() == UEFA_CUP_PRIZE_FUND, || format!("uefa_cup_set_up_rounds({:#x}) returned {} prize fund {}", this.addr(), r, this.prize_fund()));
    r
}

/// The season turnover of the World Club Cup: last season's tables freed, the season archived, the year moved on by one
/// and the clubs, the round table and the tie tree rebuilt; finally the predicted positions.
///
/// The cup turnover template (`start_new_cup_season`, checked instruction by instruction): the clubs from
/// WorldClubCup::select_teams (`FN_WORLDCLUBCUP_SELECT_TEAMS`, thiscall, ret 0) and the world_club_cup.cpp boxes at
/// lines 0x176, 0x17d and 0x184.
/// Original: `WorldClubCup::start_new_season` (0x92bda0, world_club_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_club_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), FN_WORLDCLUBCUP_SELECT_TEAMS, WORLD_CLUB_CUP_CPP, [0x176, 0x17d, 0x184], 1, "mwg_10 world club cup rollover")
}

/// Comparer for `world_club_cup_start_new_season`: only the original runs; the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_world_club_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WORLD_CLUB_CUP_START_NEW_SEASON);
    compare_cup_new_season("world_club_cup_start_new_season", Cup(this), || o(this))
}

// ---------- batch 314: the World Cup qualifying turnovers of Asia, South America and Oceania (wc_asia_league.cpp, wc_south_american_league.cpp, wc_oceania_league.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_ASIA_LEAGUE_START_NEW_SEASON);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_SOUTH_AMERICAN_LEAGUE_START_NEW_SEASON);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_OCEANIA_LEAGUE_START_NEW_SEASON);
    compare_qualifying_turnover("wc_oceania_league_start_new_season", League(this), || o(this))
}

// ---------- batch 314: the World Cup qualifying turnovers of Europe, Africa and CONCACAF (wc_europe_league.cpp, wc_african_cup.cpp, wc_concacaf_cup.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_EUROPE_LEAGUE_START_NEW_SEASON);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_AFRICAN_CUP_START_NEW_SEASON);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_CONCACAF_CUP_START_NEW_SEASON);
    compare_qualifying_turnover("wc_concacaf_cup_start_new_season", League(this), || o(this))
}

// ---------- batch 314: the World Cup's knockout stage: moving on to it, building it and its season honours (world_cup.cpp) ----------

/// Constants of the World Cup's knockout stage.
pub(crate) mod world_cup_knockout_consts {
    /// "E:\dev\CM3\cm3 00-01\cm3\code\comp\worldcup\world_cup.cpp" (the error boxes).
    pub(crate) const WORLD_CUP_CPP: usize = 0xad5888;
    /// The knockout stage's index in the stage vector (the groups are the cup itself and stages 0..=6).
    pub(crate) const KNOCKOUT_STAGE: usize = 7;
    /// NationalTeams::clear_competition_for_comp_clubs (thiscall, ret 4): clears the current competition of every
    /// national team in a competition's club list.
    /// WorldCup::create_knockout_stage (thiscall, ret 0), called directly by advance_stage (hooked: the copy below).
    pub(crate) const FN_WORLD_CUP_CREATE_KNOCKOUT_STAGE: usize = 0x92d770;
    pub(crate) const FN_NATIONALTEAMS_CLEAR_COMPETITION_FOR_COMP_CLUBS: usize = 0x775420;
    /// The 16 places of the round of 16, in the order of the stage's list (the stage pairs them): (group, place), group 0 the cup's own table (group A), 1..=7 the stages 0..=6
    /// (groups B..H), place 0 the winner, 1 the runner-up.
    pub(crate) const ROUND_OF_16: [(usize, i32); 16] = [
        (0, 0), (1, 1), (3, 0), (2, 1), (4, 0), (5, 1), (7, 0), (6, 1), // mwg14: the group winners and runners-up paired
        (1, 0), (0, 1), (2, 0), (3, 1), (5, 0), (4, 1), (6, 0), (7, 1),
    ];
}

/// The club in `place` of World Cup group `group` (0 the cup itself, k the stage k − 1): League::get_table_row
/// (`exe::league_get_table_row`, thiscall, ret 4) and the row's club.
pub(crate) unsafe fn world_cup_group_club(this: League, group: usize, place: i32) -> usize {
    let league = if group == 0 { this.addr() } else { read_usize(this.stages() + (group - 1) * 4) };
    read_usize(exe::league_get_table_row(league, place))
}

/// Moves the World Cup on to its next stage; entering stage 7 builds the knockout stage. Past the last stage it is an
/// error (the box).
///
/// At the last stage (`.current_stage()` ≥ `.stage_count()` − 1, signed) the box of line 0x345 and nothing else;
/// otherwise `league::CURRENT_STAGE` goes up by one and, when it becomes 7, `world_cup_create_knockout_stage` (direct
/// call, thiscall, ret 0).
/// Original: `WorldCup::advance_stage` (0x92d4b0, world_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_cup_advance_stage(this: usize) {
    use world_cup_knockout_consts as wk;
    let league = League(this);
    let stage = league.current_stage() as i32;
    if stage >= (league.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(wk::WORLD_CUP_CPP, 0x345);
    }
    league.set_current_stage((stage + 1) as usize);
    if stage + 1 == wk::KNOCKOUT_STAGE as i32 {
        covered!("mwg_14 world cup knockout");
        let create: unsafe extern "thiscall" fn(usize) = core::mem::transmute(wk::FN_WORLD_CUP_CREATE_KNOCKOUT_STAGE);
        create(this);
    }
}

/// Comparer for `world_cup_advance_stage`: only the original (it may build the knockout stage); the stage must stay at
/// the last one or go up by one.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WORLD_CUP_ADVANCE_STAGE);
    compare_state_league_advance_stage("world_cup_advance_stage", League(this), || o(this))
}

/// Builds the World Cup's round of 16: the eight group winners and eight runners-up in a sixteen-team cup stage (stage
/// 7), with the bookings wiped and the first round set up at once.
///
/// The bookings of the competition's ruling body cleared (the ruling body of `league::DISCIPLINE_RULE` (signed byte)
/// in [`G_DISCIPLINE`]'s table: RulingBody::reset_bookings_for_club(0), `exe::rulingbody_reset_bookings_for_club`,
/// thiscall, ret 4). The list of the 16 (`exe::crt_malloc(0x40)`, cdecl; out of memory, the box of line 0x3a0 and
/// return) in the order of `ROUND_OF_16` (A1, B2, D1, C2, E1, F2, H1, G2, then B1, A2, C1, D2, F1, E2, G1, H2). The round table through `comp_vtable::CREATE_SCHEDULE` (push 0; push &id; push &rounds; push 7;
/// mov ecx, this -> thiscall, ret 0x10); the stage (`exe::crt_operator_new`(0xb2); Cup_Stage::Cup_Stage
/// `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret 0x3c: the cup, 16, the list, the rounds, `.competition()`, the table, the year
/// `.year()`, 7, 4, the id, 0x14, 1, 0, 0 and no seed; the original pushes the rounds, the id and the year with garbage in
/// the high 16 bits, which the constructor does not read; the stage's new under an SEH frame, nothing here throws) into
/// `.stages()`[7]; the list and the table freed (`exe::crt_free`, in that order). No stage: the box of line 0x3c4 and
/// return. With it: its first round (`exe::cup_set_up_round` Cup::set_up_round(0), mov ecx, stage -> thiscall, ret 4)
/// and `league::CURRENT_STAGE` = 7.
/// Original: `WorldCup::create_knockout_stage` (0x92d770, world_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_cup_create_knockout_stage(this: usize) {
    use world_cup_knockout_consts as wk;
    let league = League(this);
    let body = read_usize(read_usize(read_usize(G_DISCIPLINE)).wrapping_add((read_i8(this + league::DISCIPLINE_RULE) as isize * 4) as usize));
    exe::rulingbody_reset_bookings_for_club(body, 0usize);
    let list = exe::crt_malloc(0x40);
    if list == 0 {
        return show_error_at(wk::WORLD_CUP_CPP, 0x3a0);
    }
    covered!("mwg_14 world cup round of 16");
    for (k, &(group, place)) in wk::ROUND_OF_16.iter().enumerate() {
        write_usize(list + k * 4, world_cup_group_club(league, group, place));
    }
    let round_table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut id) = (0i16, 0i16);
    let table = round_table(this, wk::KNOCKOUT_STAGE as u32, &mut rounds, &mut id, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, 0x10, list, rounds as u16 as u32, league.competition().addr(), table, league.year() as u16 as u32, 7, 4, id as u16 as u32, 0x14, 1, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_usize(league.stages() + wk::KNOCKOUT_STAGE * 4, stage);
    exe::crt_free(list);
    exe::crt_free(table);
    let stage = read_usize(league.stages() + wk::KNOCKOUT_STAGE * 4);
    if stage == 0 {
        return show_error_at(wk::WORLD_CUP_CPP, 0x3c4);
    }
    exe::cup_set_up_round(stage, 0);
    league.set_current_stage(wk::KNOCKOUT_STAGE);
}

// simplification: allocates the stage and builds its first round: it does not run twice. Original only; the plan: the
// stage at [+0xc][7] with 16 teams, the cup's +4 and the 16 group places (read before) in its list (+0xa7, 6 bytes; as a
// set: the round may reorder), and the current stage 7.
/// Comparer for `world_cup_create_knockout_stage`.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_create_knockout_stage(this: usize) {
    use world_cup_knockout_consts as wk;
    let league = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WORLD_CUP_CREATE_KNOCKOUT_STAGE);
    if league.table() == 0 || league.stages() == 0 || (0..7).any(|k| read_usize(league.stages() + k * 4) == 0) {
        return o(this);
    }
    let mut plan: Vec<usize> = wk::ROUND_OF_16.iter().map(|&(g, p)| world_cup_group_club(league, g, p)).collect();
    o(this);
    let stage = Cup(read_usize(league.stages() + wk::KNOCKOUT_STAGE * 4));
    if stage.is_null() {
        SCOREBOARD.record(false, || format!("world_cup_create_knockout_stage({:#x}) without stage", this));
        return;
    }
    let mut found: Vec<usize> = (0..16).map(|k| read_usize(stage.team_table() + 6 * k)).collect();
    let (pl, ac) = (plan.clone(), found.clone());
    plan.sort_unstable();
    found.sort_unstable();
    let same = plan == found && stage.team_count() == 16 && read_usize(stage.addr() + 4) == league.competition().addr()
        && league.current_stage() == wk::KNOCKOUT_STAGE;
    SCOREBOARD.record(same, || format!("world_cup_create_knockout_stage({:#x}) plan={:x?} list={:x?} stage {}", this, pl, ac, league.current_stage() as i32));
}

/// The World Cup's season honours: those of the knockout stage, then the national teams leave the competition.
///
/// The knockout stage (`.stages()`[7]) records its honours (virtual `comp_vtable::RECORD_SEASON_HONOURS`, thiscall, ret
/// 0; the original does not check the stage); then NationalTeams::clear_competition_for_comp_clubs(competition record)
/// on [`G_NATIONAL_TEAMS`] (`FN_NATIONALTEAMS_CLEAR_COMPETITION_FOR_COMP_CLUBS`, thiscall, ret 4), whose eax it returns.
/// Original: `WorldCup::record_season_honours` (0x92d310, world_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_cup_record_season_honours(this: usize) -> u32 {
    use world_cup_knockout_consts as wk;
    let league = League(this);
    let stage = read_usize(league.stages() + wk::KNOCKOUT_STAGE * 4);
    let honours: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(stage) + comp_vtable::RECORD_SEASON_HONOURS));
    honours(stage);
    covered!("mwg_14 world cup honours");
    let clear: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(wk::FN_NATIONALTEAMS_CLEAR_COMPETITION_FOR_COMP_CLUBS);
    clear(read_usize(G_NATIONAL_TEAMS), league.competition().addr()) // mwg14: the nations leave the World Cup
}

// simplification: the honours go to the history: they cannot run twice. Original only; the plan: the knockout stage
// existed (the original reads it without checking).
/// Comparer for `world_cup_record_season_honours`.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_record_season_honours(this: usize) -> u32 {
    let league = League(this);
    let had = league.stages() != 0 && read_usize(league.stages() + world_cup_knockout_consts::KNOCKOUT_STAGE * 4) != 0;
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WORLD_CUP_RECORD_SEASON_HONOURS)(this);
    SCOREBOARD.record(had, || format!("world_cup_record_season_honours({:#x}) without the knockout stage", this));
    r
}

// ---------- batch 314: a nation's World Cup fate in the news; the award news and the award shortlists (world_cup.cpp, award_manager.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\award_manager.cpp" (the error box of add_shortlist).
pub(crate) const AWARD_MANAGER_CPP: usize = 0x987e24;

/// Fields of the award manager (`AWARD_MANAGER`, the object at [`G_AWARD_MANAGER`]) the shortlist routines use.
pub(crate) mod award_manager_fields {
    /// The nation award objects' table (+8: [+8][nation] is a NationAwards, whose +4 is its award table).
    pub(crate) const NATION_AWARDS: usize = 0x8;
    /// The growable array of the open shortlists (+0x14), its count (+0x18, signed word) and capacity (+0x1a, word,
    /// grown by 5).
    pub(crate) const SHORTLISTS: usize = 0x14;
    pub(crate) const SHORTLIST_COUNT: usize = 0x18;
    pub(crate) const SHORTLIST_CAPACITY: usize = 0x1a;
}

/// Rule: the news text of a nation's World Cup fate: in the group stages (stage below 7) only the round-of-16
/// qualification (fate 6) and the group exit (fate −2) have a text, the shared group sentence; in the knockout (stage 7)
/// the win, the runners-up place, the third and fourth places, the elimination ("knocked out of") and any other fate
/// ("through to next round in") of the competition; past stage 7 there is no text.
///
/// The arguments as `goldcup_build_fate_news_text` (the club index `club_index` in [`G_CLUB_TABLE`], the fate byte
/// `fate`, the stage byte `stage` (signed), four values passed on, `full_sentence` and the Text `text`). Stage below 7:
/// fate 6 or 0xfe (−2) write the shared group sentence 0x4b4590 (club_index, a3, the competition record, the fate as
/// 6 or −2, full_sentence, text: 6 pushes; add esp, 0x18 -> cdecl), with or without `full_sentence`; any other fate,
/// nothing. Stage 7: with `full_sentence`, the shared knockout sentence 0x4b0b80 (club_index, a4, a7, fate, a5, text:
/// cdecl); otherwise the template of `copa_america_fate_template` (the same six texts) with the club's gender twice, the
/// competition's gender twice, the club's and the competition's short names (`langlib_sprintf` into 0xde1f64), set as
/// the text (`exe::text_set_string`) with the club's link (0x7d5) and the competition's (0x7d0).
/// Original: `WorldCup::build_fate_news_text` (0x92cea0, world_cup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn world_cup_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, full_sentence: u32, text: usize) {
    let this = League(this);
    let _ = a6;
    let comp = this.competition().addr();
    let s = stage as u8 as i8;
    // simplification: on the paths without text the original leaves al with the stage or the fate byte; the callers do
    // not read it
    if s < 7 {
        let group_fate: i32 = match fate as u8 {
            6 => 6,
            0xfe => -2, // mwg15: the group exit has its text
            _ => return,
        };
        covered!("mwg_15 world cup group fate");
        let sentence: unsafe extern "C" fn(u32, u32, usize, i32, u32, usize) = core::mem::transmute(FN_COMP_UTIL_WRITE_GROUP_QUALIFY_NEWS);
        sentence(club_index, a3, comp, group_fate, full_sentence, text);
        return;
    }
    if s != 7 {
        return;
    }
    covered!("mwg_15 world cup knockout fate");
    if full_sentence != 0 {
        let sentence: unsafe extern "C" fn(u32, u32, u32, u32, u32, usize) = core::mem::transmute(FN_COMP_UTIL_WRITE_CUP_TIE_RESULT_NEWS);
        sentence(club_index, a4, a7, fate, a5, text);
        return;
    }
    let club = Club(read_usize(G_CLUB_TABLE).wrapping_add((club_index as usize).wrapping_mul(club::SIZE)));
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let club_gender = club.short_gender_name() as i8 as i32;
    let comp_gender = read_u8(comp + competition::SHORT_GENDER_NAME) as i8 as i32;
    sprintf(0xde1f64, copa_america_fate_template(fate as u8), club_gender, club_gender, comp_gender, comp_gender, club.addr() + club::SHORT_NAME, comp + competition::SHORT_NAME);
    exe::text_set_string(text, 0xde1f64 as *const u8);
    exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
    exe::text_add_link(text, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
}

/// Comparer for `world_cup_build_fate_news_text`: side by side into two Texts (`compare_text_writer`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_world_cup_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, full_sentence: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(IDX_WORLD_CUP_BUILD_FATE_NEWS_TEXT);
    compare_text_writer(text, || o(this, club_index, fate, stage, a3, a4, a5, a6, a7, full_sentence, text),
        |t| world_cup_build_fate_news_text(this, club_index, fate, stage, a3, a4, a5, a6, a7, full_sentence, t),
        || format!("world_cup_build_fate_news_text(club {}, fate {}, stage {}, full {})", club_index, fate as u8, stage as u8 as i8, full_sentence));
}

/// Passes a news text request to one award of a nation: the award `award` of the nation awards `nation` writes the text
/// of `item` into `text` (headline or body by `body`).
///
/// The award object is [[`award_manager_fields::NATION_AWARDS`][nation] + 4][award] (no checks); its virtual +0xc
/// (`write_news`(item, text, body): YEAR_AWARD::write_news, TEAM_AWARD::write_news..., thiscall, ret 0xc), whose eax it
/// returns.
/// Original: `AWARD_MANAGER::write_award_news` (0x4143f0, award_manager.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn write_award_news(this: usize, nation: i32, award: i32, item: usize, text: usize, body: u32) -> u32 {
    let awards = read_usize(read_usize(this + award_manager_fields::NATION_AWARDS).wrapping_add((nation as isize * 4) as usize));
    let object = read_usize(read_usize(awards + 4).wrapping_add((award as isize * 4) as usize));
    covered!("mwg_15 award news");
    let write: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(object) + 0xc));
    write(object, item, text, body) // mwg15: headline or body
}

/// Comparer for `write_award_news`: side by side into two Texts (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_award_news(this: usize, nation: i32, award: i32, item: usize, text: usize, body: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, usize, usize, u32) -> u32>(IDX_WRITE_AWARD_NEWS);
    let mut r = 0;
    compare_text_writer(text, || r = o(this, nation, award, item, text, body), |t| {
        write_award_news(this, nation, award, item, t, body);
    }, || format!("write_award_news(nation {}, award {}, body {})", nation, award, body));
    r
}

/// Appends a shortlist to the award manager's growable array and gives its index.
///
/// When the count (`SHORTLIST_COUNT`, signed word) is not below the capacity (`SHORTLIST_CAPACITY`), the capacity goes up
/// by 5 and the array (`SHORTLISTS`) is allocated (`exe::crt_malloc`, first time) or grown (`exe::crt_realloc`) to 4 ×
/// capacity bytes; out of memory, the array becomes null, the error box (line 0x39a) and −1 (in ax; the original's high
/// word is the box's). Otherwise the shortlist goes into slot `count`, the count goes up by one and the result is the new
/// count − 1 in ax (the original computes it from esi, whose high word is still `this`'s: kept).
/// Original: `AWARD_MANAGER::add_shortlist` (0x4147f0, award_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn add_shortlist(this: usize, shortlist: usize) -> u32 {
    use award_manager_fields as am;
    let capacity = read_u16(this + am::SHORTLIST_CAPACITY);
    if read_i16(this + am::SHORTLIST_COUNT) >= capacity as i16 {
        let capacity = capacity.wrapping_add(5);
        write_u16(this + am::SHORTLIST_CAPACITY, capacity);
        let bytes = ((capacity as i16 as i32) << 2) as u32;
        let array = read_usize(this + am::SHORTLISTS);
        let array = if array == 0 { exe::crt_malloc(bytes) } else { exe::crt_realloc(array, bytes) };
        write_usize(this + am::SHORTLISTS, array);
        if array == 0 {
            show_error_at(AWARD_MANAGER_CPP, 0x39a);
            return 0xffff;
        }
        covered!("mwg_15 shortlists grown");
    }
    let count = read_i16(this + am::SHORTLIST_COUNT);
    write_usize(read_usize(this + am::SHORTLISTS).wrapping_add((count as isize * 4) as usize), shortlist);
    let count = count.wrapping_add(1);
    write_i16(this + am::SHORTLIST_COUNT, count);
    ((this as u32 & 0xffff_0000) | count as u16 as u32).wrapping_sub(1)
}

// simplification: the array may be (re)allocated: it does not run twice. Original only; the plan: the count up by one, the
// shortlist in the last slot and the old count in ax.
/// Comparer for `add_shortlist`.
pub(crate) unsafe extern "thiscall" fn compare_add_shortlist(this: usize, shortlist: usize) -> u32 {
    use award_manager_fields as am;
    let before = read_i16(this + am::SHORTLIST_COUNT);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_ADD_SHORTLIST)(this, shortlist);
    let after = read_i16(this + am::SHORTLIST_COUNT);
    let ok = r as u16 == 0xffff || (after == before.wrapping_add(1) && r as u16 as i16 == before
        && read_usize(read_usize(this + am::SHORTLISTS) + before as usize * 4) == shortlist);
    SCOREBOARD.record(ok, || format!("add_shortlist({:#x}) count {} -> {}, returned {:#x}", shortlist, before, after, r));
    r
}

// ---------- batch 314: the award shortlists: building one, adding a candidate and announcing it (award_shortlist.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\award_shortlist.cpp" (the error boxes).
pub(crate) const AWARD_SHORTLIST_CPP: usize = 0x9880b0;

/// Fields of an award shortlist (`AWARD_SHORTLIST`) and of its 14-byte candidate entries.
pub(crate) mod award_shortlist_fields {
    /// The candidate entries (+0), their count (+4) and capacity (+6), signed words.
    pub(crate) const ENTRIES: usize = 0x0;
    pub(crate) const COUNT: usize = 0x4;
    pub(crate) const CAPACITY: usize = 0x6;
    /// The 14-byte scope record copied from the constructor's argument (+0x28): +0 the scope type (0 a division, 1 a
    /// nation, 2 a continent by the club's nation, 3 a continent by the national team's nation, 4 every club manager, 5
    /// every national manager, other every manager), +1 the division's id, +5 the nation's, +9 the continent's, +0xd the
    /// capacity (signed byte).
    pub(crate) const SCOPE: usize = 0x28;
    pub(crate) const SCOPE_DIVISION: usize = 0x29;
    pub(crate) const SCOPE_NATION: usize = 0x2d;
    pub(crate) const SCOPE_CONTINENT: usize = 0x31;
    pub(crate) const SCOPE_SIZE: usize = 0xe;
    /// The shortlist's index in the award manager (+0x36, word; −1 until announced) and the announcement date (+0x38).
    pub(crate) const MANAGER_INDEX: usize = 0x36;
    pub(crate) const DATE: usize = 0x38;
    /// An entry: the player's id (+0), the club's id or −1 (+4), the score (+8, float), the votes (+0xc, word).
    pub(crate) const ENTRY_SIZE: usize = 0xe;
    pub(crate) const ENTRY_CLUB_ID: usize = 0x4;
    pub(crate) const ENTRY_SCORE: usize = 0x8;
    pub(crate) const ENTRY_VOTES: usize = 0xc;
    /// AWARD_SHORTLIST::load (thiscall, ret 4) and the qsort comparer by name (cdecl).
    pub(crate) const FN_AWARD_SHORTLIST_LOAD: usize = 0x417590;
    pub(crate) const FN_AWARD_SHORTLIST_COMPARE_BY_NAME: usize = 0x418340;
    /// The news item type of a shortlist announcement.
    pub(crate) const SHORTLIST_NEWS: u32 = 0x1c;
}

/// Builds an award shortlist from its 14-byte scope record (and loads it from a saved game when given the file).
///
/// The date (`DATE`) built empty (Date::Date `exe::date_date_54a110`, thiscall). Without a scope, the error box (line
/// 0x30) and nothing else. Otherwise `MANAGER_INDEX` = −1, the capacity = the scope's +0xd (signed byte, as a word), the
/// count 0 and the entries (`exe::crt_malloc`(14 × capacity), cdecl; out of memory, the box of line 0x3e and nothing
/// more); the scope copied (three dwords and a word, `SCOPE`); with a file, AWARD_SHORTLIST::load(file) (thiscall, ret
/// 4; its return not read). Returns this (the original builds the date under an SEH frame; nothing here throws).
/// Original: `AWARD_SHORTLIST::AWARD_SHORTLIST` (0x417440, award_shortlist.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_award_shortlist(this: usize, scope: usize, file: usize) -> usize {
    use award_shortlist_fields as asl;
    exe::date_date_54a110(this + asl::DATE);
    if scope == 0 {
        show_error_at(AWARD_SHORTLIST_CPP, 0x30);
        return this;
    }
    write_i16(this + asl::MANAGER_INDEX, -1);
    let capacity = read_i8(scope + 0xd) as i16; // mwg16: the shortlist's size from its scope
    write_i16(this + asl::CAPACITY, capacity);
    write_i16(this + asl::COUNT, 0);
    let entries = exe::crt_malloc((capacity as i32 * asl::ENTRY_SIZE as i32) as u32);
    write_usize(this + asl::ENTRIES, entries);
    if entries == 0 {
        show_error_at(AWARD_SHORTLIST_CPP, 0x3e);
        return this;
    }
    covered!("mwg_16 shortlist built");
    for k in (0..12).step_by(4) {
        write_u32(this + asl::SCOPE + k, read_u32(scope + k));
    }
    write_u16(this + asl::SCOPE + 12, read_u16(scope + 12));
    if file != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(asl::FN_AWARD_SHORTLIST_LOAD);
        load(this, file);
    }
    this
}

// simplification: allocates the entries: it does not run twice. Original only; the plan: the capacity from the scope,
// count 0, the index −1, the scope copied and the entries there.
/// Comparer for `construct_award_shortlist`.
pub(crate) unsafe extern "thiscall" fn compare_construct_award_shortlist(this: usize, scope: usize, file: usize) -> usize {
    use award_shortlist_fields as asl;
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(IDX_CONSTRUCT_AWARD_SHORTLIST)(this, scope, file);
    if scope == 0 || file != 0 || read_usize(this + asl::ENTRIES) == 0 {
        return r;
    }
    let ok = r == this && read_i16(this + asl::CAPACITY) == read_i8(scope + 0xd) as i16 && read_i16(this + asl::COUNT) == 0
        && read_i16(this + asl::MANAGER_INDEX) == -1 && (0..asl::SCOPE_SIZE).all(|k| read_u8(this + asl::SCOPE + k) == read_u8(scope + k));
    SCOREBOARD.record(ok, || format!("construct_award_shortlist({:#x}): capacity {} count {} index {}", scope, read_i16(this + asl::CAPACITY), read_i16(this + asl::COUNT), read_i16(this + asl::MANAGER_INDEX)));
    r
}

/// Appends a candidate to a shortlist: the player's id, the club's id (or −1), the score and no votes; whether the list
/// is now full.
///
/// A full list (count ≥ capacity, signed words): the error box (line 0xbf) and 1. Without the player: the box (line
/// 0xc6) and 0. Otherwise the entry at the count: the player's id, the club's id or −1 without a club, the score (the
/// original copies it through the x87 stack: a signalling NaN comes out quiet, as here) and 0 votes; the count goes up
/// by one and the result is whether it reached the capacity.
/// Original: `AWARD_SHORTLIST::add_entry` (0x4178a0, award_shortlist.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn award_shortlist_add_entry(this: usize, player: usize, club: usize, score: u32) -> u32 {
    use award_shortlist_fields as asl;
    let count = read_i16(this + asl::COUNT);
    if count >= read_i16(this + asl::CAPACITY) {
        show_error_at(AWARD_SHORTLIST_CPP, 0xbf);
        return 1;
    }
    if player == 0 {
        show_error_at(AWARD_SHORTLIST_CPP, 0xc6);
        return 0;
    }
    covered!("mwg_16 shortlist candidate");
    let e = read_usize(this + asl::ENTRIES).wrapping_add((count as isize * asl::ENTRY_SIZE as isize) as usize);
    write_u32(e, read_u32(player));
    write_u32(e + asl::ENTRY_CLUB_ID, if club != 0 { read_u32(club) } else { u32::MAX });
    let quiet = if score & 0x7f80_0000 == 0x7f80_0000 && score & 0x007f_ffff != 0 { score | 0x0040_0000 } else { score };
    write_u32(e + asl::ENTRY_SCORE, quiet);
    write_u16(e + asl::ENTRY_VOTES, 0); // mwg16: a new candidate without votes
    let count = count.wrapping_add(1);
    write_i16(this + asl::COUNT, count);
    (count == read_i16(this + asl::CAPACITY)) as u32
}

/// Comparer for `award_shortlist_add_entry`: side by side over the count and the new entry.
pub(crate) unsafe extern "thiscall" fn compare_award_shortlist_add_entry(this: usize, player: usize, club: usize, score: u32) -> u32 {
    use award_shortlist_fields as asl;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_AWARD_SHORTLIST_ADD_ENTRY);
    let count = read_i16(this + asl::COUNT);
    let slot = read_usize(this + asl::ENTRIES).wrapping_add((count.max(0) as usize) * asl::ENTRY_SIZE);
    let full = count >= read_i16(this + asl::CAPACITY) || player == 0;
    let regions = if full { vec![(this + asl::COUNT, 2)] } else { vec![(this + asl::COUNT, 2), (slot, asl::ENTRY_SIZE)] };
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, player, club, score), || award_shortlist_add_entry(this, player, club, score));
    SCOREBOARD.record(equal && r == n, || format!("award_shortlist_add_entry(player {:#x}, club {:#x}) original={} {:x?} rust={} {:x?}", player, club, r, mo, n, mr));
    r
}

/// Whether a human manager (a staff record) is in a shortlist's scope (the announcement goes to him): by the scope type
/// (`SCOPE`): 0 his club's division is the scope's (`SCOPE_DIVISION`), 1 his club's nation (`SCOPE_NATION`), 2 his club's
/// nation's continent (`SCOPE_CONTINENT`), 3 his national team's nation's continent, 4 he has a club, 5 he has a national
/// team, any other everyone. A missing link on the way, not in scope.
pub(crate) unsafe fn shortlist_scope_has_manager(this: usize, s: usize) -> bool {
    use award_shortlist_fields as asl;
    let club = read_usize(s + staff::CLUB_JOB);
    let national = read_usize(s + staff::NATIONAL_JOB);
    let continent_of = |team: usize| -> Option<u32> {
        if team == 0 {
            return None;
        }
        let nation = read_usize(team + club::NATION);
        if nation == 0 {
            return None;
        }
        let continent = read_usize(nation + nation::CONTINENT);
        if continent == 0 { None } else { Some(read_u32(continent)) }
    };
    match read_u8(this + asl::SCOPE) {
        0 => club != 0 && read_usize(club + club::DIVISION) != 0 && read_u32(read_usize(club + club::DIVISION)) == read_u32(this + asl::SCOPE_DIVISION),
        1 => club != 0 && read_usize(club + club::NATION) != 0 && read_u32(read_usize(club + club::NATION)) == read_u32(this + asl::SCOPE_NATION),
        2 => continent_of(club) == Some(read_u32(this + asl::SCOPE_CONTINENT)),
        3 => continent_of(national) == Some(read_u32(this + asl::SCOPE_CONTINENT)),
        4 => club != 0,
        5 => national != 0,
        _ => true,
    }
}

/// Announces a shortlist: the candidates sorted by name, the shortlist registered with the award manager, its date set
/// and the shortlist news sent to each human manager in its scope.
///
/// The entries sorted by name (`exe::crt_qsort`(entries, count, 14, `FN_AWARD_SHORTLIST_COMPARE_BY_NAME`)); the index
/// from the award manager (AWARD_MANAGER::add_shortlist on [`G_AWARD_MANAGER`], thiscall, ret 4) into `MANAGER_INDEX`;
/// the date (`DATE`) = `date` (Date::operator= `exe::date_operator_3d`). A news item of type `SHORTLIST_NEWS`
/// (NewsItem::NewsItem `exe::new_newsitem`(item, 0x1c, 0)) with slot 0 `a`, slot 1 `b`, slot 2 the date's day (signed
/// word), slot 3 its year (word) and slot 4 0 (`exe::newsitem_set_slot`). Then the 16 human manager rows (the last 16
/// of the staff table, from the end: `staff_at`(count − 1 − k)) in use (`staff::CLASSIFICATION` ≠ 0) and in the scope
/// (`shortlist_scope_has_manager`) get the news (News::send_to_club_manager `exe::news_send_to_club_manager` on
/// [`G_NEWS`], with his club, or his national team without one). The original keeps the news item under an SEH frame;
/// nothing here throws.
/// Original: `AWARD_SHORTLIST::announce` (0x417a20, award_shortlist.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn award_shortlist_announce(this: usize, date: usize, a: u32, b: u32) {
    use award_shortlist_fields as asl;
    exe::crt_qsort(read_usize(this + asl::ENTRIES), read_i16(this + asl::COUNT) as i32 as u32, asl::ENTRY_SIZE as u32, asl::FN_AWARD_SHORTLIST_COMPARE_BY_NAME);
    // AWARD_MANAGER::add_shortlist returns a 16-bit index (some paths write only ax)
    let add: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(0x4147f0usize);
    let index = add(read_usize(G_AWARD_MANAGER), this);
    write_u16(this + asl::MANAGER_INDEX, index);
    let mut copy = [0u32; 2];
    exe::date_operator_3d(this + asl::DATE, copy.as_mut_ptr() as usize, date);
    let mut news = [0u32; 0x38];
    let item = news.as_mut_ptr() as usize;
    exe::new_newsitem(item, asl::SHORTLIST_NEWS, 0);
    exe::newsitem_set_slot(item, 0, a);
    exe::newsitem_set_slot(item, 1, b);
    exe::newsitem_set_slot(item, 2, read_i16(date) as i32 as u32);
    exe::newsitem_set_slot(item, 3, read_u16(date + 2) as u32);
    exe::newsitem_set_slot(item, 4, 0);
    covered!("mwg_16 shortlist announced");
    for k in 0..16isize {
        let s = staff_at(read_i32(G_STAFF_COUNT) as isize - k - 1).addr();
        if read_u8(s + staff::CLASSIFICATION) == 0 || !shortlist_scope_has_manager(this, s) {
            continue;
        }
        let club = read_usize(s + staff::CLUB_JOB);
        let target = if club != 0 { club } else { read_usize(s + staff::NATIONAL_JOB) };
        exe::news_send_to_club_manager(read_usize(G_NEWS), item, target);
    }
}

// simplification: registers the shortlist and sends news: it does not run twice. Original only; the plan: the index set
// and the date copied.
/// Comparer for `award_shortlist_announce`.
pub(crate) unsafe extern "thiscall" fn compare_award_shortlist_announce(this: usize, date: usize, a: u32, b: u32) {
    use award_shortlist_fields as asl;
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(IDX_AWARD_SHORTLIST_ANNOUNCE)(this, date, a, b);
    let ok = read_i16(this + asl::MANAGER_INDEX) >= 0 && read_u32(this + asl::DATE) == read_u32(date);
    SCOREBOARD.record(ok, || format!("award_shortlist_announce({:#x}): index {} date {:#x}", this, read_i16(this + asl::MANAGER_INDEX), read_u32(this + asl::DATE)));
}

// ---------- batch 314: the daily fog of war update, the due note reminders and the ruling body's base constructor (fog_of_war.cpp, notes.cpp, ruling_body.cpp) ----------

/// FOG_OF_WAR_MAP::daily_update (thiscall, ret 0): one human manager's map's daily knowledge update.
pub(crate) const FN_FOG_OF_WAR_MAP_DAILY_UPDATE: usize = 0x5b0260;
/// Notes::post_reminder_news (thiscall, ret 8): the reminder news of one note of manager slot k.
pub(crate) const FN_NOTES_POST_REMINDER_NEWS: usize = 0x795e20;
/// The note's due date (+0x10b of a note record).
pub(crate) const NOTE_DUE_DATE: usize = 0x10b;
/// RulingBody's own vtable, RulingBody::load (thiscall, ret 4) and the ruling body's list (+7, a 0xe-byte LLIST).
pub(crate) const RULING_BODY_VTABLE: u32 = 0x96f104;
pub(crate) const FN_RULING_BODY_LOAD: usize = 0x7e78a0;
pub(crate) const RULING_BODY_LIST: usize = 0x7;
/// "E:\dev\CM3\cm3 00-01\cm3\code\ruling_body\ruling_body.cpp" (the error box).
pub(crate) const RULING_BODY_CPP: usize = 0xa7a858;

/// The daily fog of war update: each of the sixteen human manager maps that exists runs its daily knowledge update.
///
/// `this` holds 16 map pointers; each non-null one gets FOG_OF_WAR_MAP::daily_update (`FN_FOG_OF_WAR_MAP_DAILY_UPDATE`,
/// thiscall, ret 0), in slot order.
/// Original: `FOG_OF_WAR::daily_update` (0x5afce0, fog_of_war.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fog_of_war_daily_update(this: usize) {
    let update: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FOG_OF_WAR_MAP_DAILY_UPDATE);
    for k in 0..16 {
        let map = read_usize(this + k * 4);
        if map != 0 {
            covered!("mwg_17 fog of war map");
            update(map);
        }
    }
}

// simplification: each map's update changes its knowledge records: it does not run twice. Original only; the plan: the 16
// slots unchanged (the update does not replace the maps).
/// Comparer for `fog_of_war_daily_update`.
pub(crate) unsafe extern "thiscall" fn compare_fog_of_war_daily_update(this: usize) {
    let before: Vec<usize> = (0..16).map(|k| read_usize(this + k * 4)).collect();
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_FOG_OF_WAR_DAILY_UPDATE)(this);
    let same = (0..16).all(|k| read_usize(this + k * 4) == before[k]);
    SCOREBOARD.record(same, || format!("fog_of_war_daily_update({:#x}): the maps changed", this));
}

/// The due note reminders: every note of the sixteen manager slots whose due date is today gets its reminder news.
///
/// `this` holds 16 slots of 8 bytes whose +0 is the slot's note list (LLIST). For each slot k: the list's cursor
/// (`llist::CURSOR`) back to its head (`llist::HEAD`); then, as many times as the list's count (`llist::COUNT`, a signed
/// word read again on each pass), the next note (LLIST::get_next `exe::llist_get_next`, thiscall) and, when its due date
/// (`NOTE_DUE_DATE`) equals today (Date::operator== `exe::date_operator_3d_3d` with [`G_CURRENT_DATE`] by value),
/// Notes::post_reminder_news(k, note) (`FN_NOTES_POST_REMINDER_NEWS`, thiscall, ret 8).
/// Original: `Notes::post_due_reminders` (0x795360, notes.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn notes_post_due_reminders(this: usize) {
    let post: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_NOTES_POST_REMINDER_NEWS);
    for k in 0..16usize {
        let list = read_usize(this + k * 8);
        write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
        let mut i = 0i32;
        while i < read_i16(read_usize(this + k * 8) + llist::COUNT) as i32 {
            let note = exe::llist_get_next(read_usize(this + k * 8));
            if exe::date_operator_3d_3d(note + NOTE_DUE_DATE, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP)) != 0 {
                covered!("mwg_17 note due today");
                post(this, k as u32, note);
            }
            i += 1;
        }
    }
}

// simplification: the reminders are news: they do not run twice. Original only; the plan: each list's cursor ends at
// the end of the list (or at the head of an empty one).
/// Comparer for `notes_post_due_reminders`.
pub(crate) unsafe extern "thiscall" fn compare_notes_post_due_reminders(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_NOTES_POST_DUE_REMINDERS)(this);
    let wrong = (0..16usize).find(|&k| {
        let list = read_usize(this + k * 8);
        read_i16(list + llist::COUNT) > 0 && read_usize(list + llist::CURSOR) != 0
    });
    SCOREBOARD.record(wrong.is_none(), || format!("notes_post_due_reminders({:#x}): slot {:?} not walked to the end", this, wrong));
}

/// The base constructor of every ruling body: its vtable, a new list, no entries; from a saved game, its saved state.
///
/// [this] = `RULING_BODY_VTABLE`; `RULING_BODY_LIST` = a new LLIST (`exe::crt_operator_new(0xe)` and LLIST::LLIST
/// `exe::new_llist`, or 0 out of memory); no entries (`ruling_body::ENTRIES`, `ENTRY_CAPACITY` and `ENTRY_COUNT` = 0).
/// With a saved game (`file`), RulingBody::load(file) (`FN_RULING_BODY_LOAD`, thiscall, ret 4): 0 shows the error box
/// (line 0x36). Returns this (the original builds the list under an SEH frame; nothing here throws).
/// Original: `RulingBody::RulingBody` (0x7e7760, ruling_body.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn construct_ruling_body(this: usize, file: usize) -> usize {
    write_u32(this, RULING_BODY_VTABLE);
    let p = exe::crt_operator_new(0xe);
    let list = if p != 0 { exe::new_llist(p) } else { 0 };
    write_usize(this + RULING_BODY_LIST, list);
    covered!("mwg_17 ruling body built");
    write_u32(this + ruling_body::ENTRIES, 0);
    write_u32(this + ruling_body::ENTRY_CAPACITY, 0);
    write_u32(this + ruling_body::ENTRY_COUNT, 0);
    if file != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_RULING_BODY_LOAD);
        if load(this, file) == 0 {
            show_error_at(RULING_BODY_CPP, 0x36);
        }
    }
    this
}

// simplification: allocates the list: it does not run twice. Original only; the plan: the vtable, a list and no entries
// (without a saved game).
/// Comparer for `construct_ruling_body`.
pub(crate) unsafe extern "thiscall" fn compare_construct_ruling_body(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CONSTRUCT_RULING_BODY)(this, file);
    let ok = file != 0 || (r == this && read_u32(this) == RULING_BODY_VTABLE && read_usize(this + RULING_BODY_LIST) != 0
        && read_u32(this + ruling_body::ENTRIES) == 0 && read_u32(this + ruling_body::ENTRY_COUNT) == 0);
    SCOREBOARD.record(ok, || format!("construct_ruling_body({:#x}): vtable {:#x} list {:#x}", this, read_u32(this), read_usize(this + RULING_BODY_LIST)));
    r
}
