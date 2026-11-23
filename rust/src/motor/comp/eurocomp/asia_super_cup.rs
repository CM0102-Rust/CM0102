use super::*;

/// rule: the Asian Super Cup is two-legged, the first match on 4 August (drawn on 4 July) and the return 14 days later,
/// category 6, two-leg tie-break 0x83, no prizes
/// Original: `AsiaSuperCup::create_round_table` (0x410910, asia_super_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn asia_super_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &ASIAN_SUPER_CUP, 0x987be0, 0x97, "cqp_10 tabela da supercopa asiatica")
}

/// rule: the Asian Super Cup is between the Asian Club Championship winner and the Asian Cup Winners' Cup winner; if
/// the same club won both, the Club Championship runner-up plays
/// Original: `AsiaSuperCup::select_teams` (0x410a50, asia_super_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_super_cup_select_teams(this: usize) -> i32 {
    select_asian_super_cup_teams(this).0
}

/// Rule: the Asian Super Cup turns over only in June (month 5, 0-based): the finished cup is cleared, its statistics
/// archived and the year moves on; then the two entrants, the rules, the tie tree, the first round and the predicted
/// positions are made again.
///
/// `AsiaSuperCup::start_new_season()` (virtual +8). The month of today (Date::get_month `exe::date_get_month`
/// ([`G_CURRENT_DATE`]), thiscall, ret 0: 0 to 11, or −1) other than 5: nothing (it returns the month). Otherwise the
/// first half (`cup_turnover_free_season`); the entrants (AsiaSuperCup 0x410a50: mov ecx -> thiscall, ret 0; negative:
/// the error box at line 0x11f and return), the rules (virtual `comp_vtable::SET_UP_RULES`; negative: the box at line
/// 0x126), the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`; negative: the box at line 0x12d), the first round
/// (Cup::set_up_round `exe::cup_set_up_round`(this, 0): push 0; mov ecx -> thiscall, ret 4) and the predicted positions
/// (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose return it passes on.
/// Original: `AsiaSuperCup::start_new_season` (0x410c50, asia_super_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_super_cup_start_new_season(this: usize) -> u32 {
    let cup = Cup(this);
    let month = exe::date_get_month(G_CURRENT_DATE);
    // simplification: the original returns the whole eax of get_month (the month, or −1); the caller does not read it
    if month != 5 { // cqx9: the Asian Super Cup turns over in June
        return month as i8 as i32 as u32;
    }
    covered!("cqx_9 asian super cup turnover");
    cup_turnover_free_season(cup);
    let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_ASIASUPERCUP_SELECT_TEAMS);
    // simplification: in the boxes the original returns the box's eax; the caller does not read it
    if entrants(this) < 0 {
        show_error_at(0x987be0, 0x11f);
        return 0;
    }
    if cup_turnover_virtual(cup, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(0x987be0, 0x126);
        return 0;
    }
    if cup_turnover_virtual(cup, cup_vtable::MAP_FIXTURE_TREE) < 0 {
        show_error_at(0x987be0, 0x12d);
        return 0;
    }
    exe::cup_set_up_round(cup, 0); // cqx9: the first round of the new Super Cup
    cup_turnover_virtual(cup, comp_vtable::SET_PREDICTED_POSITIONS) as u32
}

pub(crate) unsafe extern "thiscall" fn compare_asia_super_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ASIA_SUPER_CUP_START_NEW_SEASON);
    if exe::date_get_month(G_CURRENT_DATE) != 5 {
        let year = Cup(this).year();
        let r = o(this);
        SCOREBOARD.record(Cup(this).year() == year, || format!("asia_super_cup_start_new_season({:#x}) outside June: year {} -> {}", this, year, Cup(this).year()));
        return r;
    }
    compare_cup_new_season("asia_super_cup_start_new_season", Cup(this), || o(this))
}

/// The Asian Super Cup at career creation: the year and the database competition; a new cup has two clubs of several
/// nations, 7 substitutes on the bench and 3 substitutions, discipline rule 41, no prize fund of its own, the rounds,
/// the tie tree, the statistics and the final ranking (see `build_asian_super_cup`).
/// Original: `AsiaSuperCup::AsiaSuperCup` (0x410650, asia_super_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_asia_super_cup(this: usize, year: u32, record: usize) -> usize {
    build_asian_super_cup(Cup(this), year, record) // cqw5: the Asian Super Cup
}

/// Comparer for `construct_asia_super_cup`: only the original runs (it allocates, draws and reads the save file),
/// then the plan is checked (`cup_constructor_plan_mismatch`).
pub(crate) unsafe extern "thiscall" fn compare_construct_asia_super_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ASIA_SUPER_CUP)(this, year, record);
    let error = cup_constructor_plan_mismatch(this, r, year, record, 0x9676c8, &ASIAN_SUPER_CUP_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_asia_super_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The first half of a cup's season turnover (the same block in AsiaSuperCup and ConChamp): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`: push; add esp, 4
/// -> cdecl) and zeroed; with the tie tables (`cup::TIE_TABLES`), each of the `cup::ROUND_COUNT` rounds' (a signed
/// word) freed and zeroed, then the array freed and zeroed; the season archived in the statistics (+8, when there:
/// Comp_Stats::archive_season_and_reset `exe::comp_stats_archive_season_and_reset`(stats, 1), thiscall, ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` and `cup::RUN_BY_STAGES` = 0.
pub(crate) unsafe fn cup_turnover_free_season(cup: Cup) {
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
    let stats = read_usize(cup.addr() + 8);
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    cup.set_year(cup.year().wrapping_add(1)); // cqx9: the next season's cup
    cup.set_current_round(0);
    cup.set_run_by_stages(0);
}

/// A cup virtual (thiscall, ret 0) at slot offset `o`; its signed return.
pub(crate) unsafe fn cup_turnover_virtual(cup: Cup, o: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(read_usize(read_usize(cup.addr()) + o));
    f(cup.addr())
}

/// The constructor of the Asian Super Cup (`AsiaSuperCup::AsiaSuperCup(year, record)`), run once, at career creation.
///
/// On the Cup base (Cup::Cup `exe::new_cup`, thiscall, ret 0): the competition record at +4, the vtable 0x9676c8, the
/// year (`set_year`), `current_round` 0, `stage_index` 0xff (the main competition), `stage_count` 0, `current_stage` −1,
/// `club_sides` 2 (clubs of several nations), 7 `substitutes_named`, 3 `substitutions_allowed`, `discipline_index` 41
/// and `run_by_stages` 0; the prize fund (+0x28) stays as the base left it. If the saved state loads
/// (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this -> thiscall, ret 4; returns nonzero), returns this.
///
/// On a new cup, in order, each step with its error box (`show_error_at`, source 0x987be0) and returning this when it
/// fails: the two clubs (AsiaSuperCup::select_teams 0x410a50, thiscall, ret 0; the box of line 0x3e when it returns a
/// negative value); the round table (`build_cup_round_table_by_vtable`); the tie tree (Cup::map_fixture_tree 0x518790,
/// thiscall, ret 0; the box of line 0x4c when negative); the statistics at +8 (`create_comp_stats_at_8`; the box of
/// line 0x53 without them); the final ranking (Cup::rank_teams 0x5223a0, thiscall, ret 0, unchecked). Returns this.
pub(crate) unsafe fn build_asian_super_cup(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_u32(this.addr() + 4, record as u32);
    write_u32(this.addr(), 0x9676c8);
    this.set_year(year as u16 as i16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_u32(this.addr() + cup::STAGE_COUNT, 0);
    write_u32(this.addr() + cup::CURRENT_STAGE, 0xffff_ffff);
    this.set_club_sides(2);
    this.set_substitutes_named(7);
    this.set_substitutions_allowed(3);
    this.set_discipline_index(0x29);
    this.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_CUP_LOAD_FROM_DISK); // Cup::load_from_disk
    if load_from_disk(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqw_5 asian super cup new");
    let select_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_ASIASUPERCUP_SELECT_TEAMS); // AsiaSuperCup::select_teams
    if select_teams(this.addr()) < 0 {
        show_error_at(0x987be0, 0x3e);
        return this.addr();
    }
    build_cup_round_table_by_vtable(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_CUP_MAP_FIXTURE_TREE); // Cup::map_fixture_tree
    if map_fixture_tree(this.addr()) < 0 {
        show_error_at(0x987be0, 0x4c);
        return this.addr();
    }
    if !create_comp_stats_at_8(this.addr()) {
        show_error_at(0x987be0, 0x53);
        return this.addr();
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_CUP_RANK_TEAMS); // Cup::rank_teams
    rank_teams(this.addr());
    this.addr()
}

/// Creates the competition's statistics and stores them at +8 (the Comp base field, `league::COMP_STATS`); returns
/// whether they were created.
///
/// Operator new `exe::crt_operator_new` of 0x5ce bytes (push; add esp, 4 -> cdecl) and, when it is not null,
/// Comp_Stats::Comp_Stats `exe::new_comp_stats` (push this; mov ecx, new -> thiscall, ret 4). The original's SEH frame
/// covers only this `new`.
pub(crate) unsafe fn create_comp_stats_at_8(this: usize) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    write_u32(this + 8, s as u32);
    s != 0
}

/// The fixed fields the Asian Super Cup constructor writes (offset, width, value), checked by the comparer.
pub(crate) const ASIAN_SUPER_CUP_CTOR_FIELDS: [(usize, u8, u32); 9] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x2), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x29), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];
