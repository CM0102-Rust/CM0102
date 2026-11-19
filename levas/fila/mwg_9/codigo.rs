// ---------- batch %%LEVA%%: the season turnovers of the European Super Cup, the European Cup and the UEFA Cup (eur_super_cup.cpp, european_cup.cpp, uefa_cup.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_cup_new_season("uefa_cup_start_new_season", Cup(this), || o(this))
}
