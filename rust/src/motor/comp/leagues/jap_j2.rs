use super::*;

/// J2: four round robins; win 3, draw 1; tie-break by goal difference, goals scored and away goals; 2 go up to J1 (the
/// rule word stays the base one, 3: champion and promotion), no relegation, no league below; 5 bench substitutes, 3
/// substitutions. Then the schedule
/// Original: `JapJ2::set_up_rules` (0x665140, jap_j2.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn jap_j2_set_up_rules(this: usize) -> u32 {
    j2_rules(this);
    covered!("cqj_7 j2 rules");
    create_main_schedule(League(this));
    1
}

/// J2 has 12 clubs, those of the division in database order; those beyond 12 lose their division; no prize fund nor TV
/// money
/// Original: `JapJ2::create_table` (0x665030, jap_j2.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn jap_j2_create_table(this: usize) -> u32 {
    if alloc_league_table(League(this), 0xc, 0x9eaedc, 0x13a) == 0 {
        return 0;
    }
    fill_japanese_table(League(this), KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2, 0, "cqm_9 j2 cheia")
}

/// J2 at career creation: the year and the database competition, discipline rule 0x13 (`discipline_rule::JAPAN_LEAGUE`)
/// and, with no saved state, a plain round robin (no stage besides the league, no stage array), the rules, the table,
/// the fixtures, the order, the statistics and the predicted positions by club strength (`bra_nat_third_construct_league`
/// with `JAP_J2_LEAGUE`).
/// Original: `JapJ2::JapJ2` (0x6644c0, jap_j2.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_jap_j2(this: usize, year: u32, record: usize) -> usize {
    bra_nat_third_construct_league(League(this), year, record, &JAP_J2_LEAGUE, discipline_rule::JAPAN_LEAGUE, "cqu_2 jap_j2 new") // cqu2: the discipline of J2
}

/// Comparer for `construct_jap_j2`.
pub(crate) unsafe extern "thiscall" fn compare_construct_jap_j2(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_JAP_J2);
    bra_nat_third_compare_league_ctor("construct_jap_j2", League(this), year, record, &JAP_J2_LEAGUE, discipline_rule::JAPAN_LEAGUE, || o(this, year, record))
}

/// The J2 schedule: the round-robin stage (−1) (a league): 44 rounds, from 12 February to 26 October; each round has
/// the date (day, month, years after the season one, weekday and code) and the games, and the rest of the table is left
/// as allocated. Virtual `comp_vtable::CREATE_SCHEDULE` (`create_schedule(stage, &rounds, &extra, &is_league)`): the
/// stage of `jap_j2_schedule_stage` through the cup schedule machine (`create_round_table`: the outputs, the table of
/// 0x41 bytes per round from `exe::crt_malloc`, the error box of line 0x9d of jap_j2.cpp without memory, and the dates
/// and games of each round; the league's year at `league::YEAR` is read as `cup::YEAR`, the same word +0x40).
/// Original: `JapJ2::create_schedule` (0x664780, jap_j2.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn jap_j2_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), jap_j2_schedule_stage(stage), rounds, extra, league, 0x9eaedc)
}

/// Comparer for `jap_j2_create_schedule`: side by side (`asia_club_champ_compare_round_table`).
pub(crate) unsafe extern "thiscall" fn compare_jap_j2_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_JAP_J2_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("jap_j2_create_schedule", stage, jap_j2_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| jap_j2_create_schedule(this, stage, r, e, l))
}

/// The J2 season turnover. `league::SEASON_CLOSED` = 0 and the table freed (`exe::league_free_table`(this, 0)); the
/// round dates (`league::ROUND_DATES`) freed (`exe::crt_free`) and zeroed; with statistics (`league::COMP_STATS`),
/// `exe::comp_stats_archive_season_and_reset`(stats, 1); the year (`league::YEAR`, word) + 1; then, each with its own
/// error box (returning 0; the original returns the box's eax, which nobody reads) if it returns 0: the rules
/// (`comp_vtable::SET_UP_RULES`, thiscall, ret 0; line 0x196), the table (JapJ2::create_table 0x665030, thiscall, ret 0;
/// 0x19d), the fixtures (`exe::league_create_fixtures`; 0x1a3) and the sort (`exe::league_sort_table`(this, 0); 0x1aa);
/// the predicted positions (`comp_vtable::SET_PREDICTED_POSITIONS`), the board expectations
/// (`exe::league_set_club_season_expectations`) and the referee renewal (OfficialsManager::renew_officials
/// `FN_OFFICIALSMANAGER_RENEW_OFFICIALS`, thiscall([`G_OFFICIALS_MANAGER`], `league::COMPETITION`), ret 4), whose eax it
/// returns. The J2 has no stages besides the league.
/// Original: `JapJ2::start_new_season` (0x6651f0, jap_j2.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn jap_j2_start_new_season(this: usize) -> u32 {
    jap_j2_start_new_season_years(this, 1) // cqv3: the J2's next year
}

/// The turnover comparer: runs only the original (it frees the table and the round dates, allocates the new ones and
/// draws the fixtures and referees; it cannot run twice), checking what only it guarantees: the year + 1, the same
/// statistics block and, with the rules, table and fixtures built, the new table (`league::TABLE`).
pub(crate) unsafe extern "thiscall" fn compare_jap_j2_start_new_season(this: usize) -> u32 {
    let this = League(this);
    let before = this.year();
    let stats = this.comp_stats();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_JAP_J2_START_NEW_SEASON)(this.addr());
    let after = this.year();
    let same = after == before.wrapping_add(1) && this.comp_stats() == stats && this.table() != 0;
    SCOREBOARD.record(same, || format!("jap_j2_start_new_season({:#x}) year before={} after={} +8={:#x} +0xb1={:#x}", this.addr(), before, after, this.comp_stats(), this.table()));
    r
}

/// The stages of the J2 round table: the round-robin stage (−1) (a league): 44 rounds, from 12 February to 26 October;
/// every other stage has no table (`None`: returns 0 without writing anything).
pub(crate) fn jap_j2_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 44, extra: 0, extra_check: false, size: 0xb2c, row: 0x9d,
            ops: &J2_LEAGUE_ROUNDS, name: "cqu_4 j2 league" }),
        _ => None,
    }
}

/// The body of `jap_j2_start_new_season`, moving the year by `years`.
pub(crate) unsafe fn jap_j2_start_new_season_years(this: usize, years: i16) -> u32 {
    let this = League(this);
    this.set_season_closed((0) as usize);
    exe::league_free_table(this, 0usize);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    let a = this.year();
    this.set_year(a.wrapping_add(years));
    covered!("cqv_3 j2 turnover");
    let set_up_rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_UP_RULES));
    if set_up_rules(this.addr()) == 0 {
        show_error_at(0x9eaedc, 0x196);
        return 0;
    }
    if exe::japj2_create_table(this) == 0 {
        show_error_at(0x9eaedc, 0x19d);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(0x9eaedc, 0x1a3);
        return 0;
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(0x9eaedc, 0x1aa);
        return 0;
    }
    let set_predicted_positions: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_PREDICTED_POSITIONS));
    set_predicted_positions(this.addr());
    exe::league_set_club_season_expectations(this);
    // the full eax is returned, so the u8 wrapper `exe::officialsmanager_renew_officials` is not used
    exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), this.competition().addr()) as u32 // cqv3: the J2 referee renewal
}

pub(crate) const JAP_J2_LEAGUE: BraNatThirdLeagueVariant = BraNatThirdLeagueVariant {
    vtable: 0x96caf4,
    importance: None,
    stages: 0,
    writes: &[],
    rules: Some(0x665140),
    table: 0x665030,
    fixtures: FN_LEAGUE_CREATE_FIXTURES,
    predicted: FN_LEAGUE_SET_PREDICTED_POSITIONS,
    serie_c: false,
    file: 0x9eaedc,
    rows: &[0x46, 0x4d, 0x53, 0x5a, 0x61],
};

/// J2, the league: 44 rounds (from 12 February to 26 October), 2860 bytes, 88 operations.
pub(crate) const J2_LEAGUE_ROUNDS: [AsiaClubChampScheduleOp; 88] = [
    Cqp11D(0, 0xc, 2, 0, 6, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x13, 2, 0, 6, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0x1a, 2, 0, 6, 1, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 2, 3, 0, 6, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 9, 3, 0, 6, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 0x10, 3, 0, 6, 1, 0), Cqp11L(5, 0, -1, -1, -1, 0),
    Cqp11D(6, 0x17, 3, 0, 6, 1, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0x1b, 3, 0, 3, 2, 0),
    Cqp11L(7, 0, -1, -1, -1, 0), Cqp11D(8, 0x1f, 3, 0, 6, 1, 0), Cqp11L(8, 0, -1, -1, -1, 0),
    Cqp11D(9, 3, 4, 0, 2, 2, 0), Cqp11L(9, 0, -1, -1, -1, 0), Cqp11D(0xa, 7, 4, 0, 6, 1, 0),
    Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 0xe, 4, 0, 6, 1, 0), Cqp11L(0xb, 0, -1, -1, -1, 0),
    Cqp11D(0xc, 0x15, 4, 0, 6, 1, 0), Cqp11L(0xc, 0, -1, -1, -1, 0), Cqp11D(0xd, 0x1b, 4, 0, 5, 1, 0),
    Cqp11L(0xd, 0, -1, -1, -1, 0), Cqp11D(0xe, 0x19, 5, 0, 6, 1, 0), Cqp11L(0xe, 0, -1, -1, -1, 0),
    Cqp11D(0xf, 2, 6, 0, 6, 1, 0), Cqp11L(0xf, 0, -1, -1, -1, 0), Cqp11D(0x10, 9, 6, 0, 6, 1, 0),
    Cqp11L(0x10, 0, -1, -1, -1, 0), Cqp11D(0x11, 0xf, 6, 0, 5, 1, 0), Cqp11L(0x11, 0, -1, -1, -1, 0),
    Cqp11D(0x12, 0x1e, 6, 0, 6, 1, 0), Cqp11L(0x12, 0, -1, -1, -1, 0), Cqp11D(0x13, 2, 7, 0, 2, 2, 0),
    Cqp11L(0x13, 0, -1, -1, -1, 0), Cqp11D(0x14, 6, 7, 0, 6, 1, 0), Cqp11L(0x14, 0, -1, -1, -1, 0),
    Cqp11D(0x15, 0xd, 7, 0, 6, 1, 0), Cqp11L(0x15, 0, -1, -1, -1, 0), Cqp11D(0x16, 0x14, 7, 0, 6, 1, 0),
    Cqp11L(0x16, 0, -1, -1, -1, 0), Cqp11D(0x17, 0x17, 7, 0, 2, 2, 0), Cqp11L(0x17, 0, -1, -1, -1, 0),
    Cqp11D(0x18, 0x1b, 7, 0, 6, 1, 0), Cqp11L(0x18, 0, -1, -1, -1, 0), Cqp11D(0x19, 3, 8, 0, 6, 1, 0),
    Cqp11L(0x19, 0, -1, -1, -1, 0), Cqp11D(0x1a, 6, 8, 0, 2, 2, 0), Cqp11L(0x1a, 0, -1, -1, -1, 0),
    Cqp11D(0x1b, 0xa, 8, 0, 6, 1, 0), Cqp11L(0x1b, 0, -1, -1, -1, 0), Cqp11D(0x1c, 0xd, 8, 0, 2, 2, 0),
    Cqp11L(0x1c, 0, -1, -1, -1, 0), Cqp11D(0x1d, 0x11, 8, 0, 6, 1, 0), Cqp11L(0x1d, 0, -1, -1, -1, 0),
    Cqp11D(0x1e, 0x18, 8, 0, 6, 1, 0), Cqp11L(0x1e, 0, -1, -1, -1, 0), Cqp11D(0x1f, 1, 9, 0, 6, 1, 0),
    Cqp11L(0x1f, 0, -1, -1, -1, 0), Cqp11D(0x20, 4, 9, 0, 2, 2, 0), Cqp11L(0x20, 0, -1, -1, -1, 0),
    Cqp11D(0x21, 7, 9, 0, 5, 1, 0), Cqp11L(0x21, 0, -1, -1, -1, 0), Cqp11D(0x22, 0xb, 9, 0, 2, 2, 0),
    Cqp11L(0x22, 0, -1, -1, -1, 0), Cqp11D(0x23, 0xf, 9, 0, 6, 1, 0), Cqp11L(0x23, 0, -1, -1, -1, 0),
    Cqp11D(0x24, 0x16, 9, 0, 6, 1, 0), Cqp11L(0x24, 0, -1, -1, -1, 0), Cqp11D(0x25, 0x1d, 9, 0, 6, 1, 0),
    Cqp11L(0x25, 0, -1, -1, -1, 0), Cqp11D(0x26, 1, 0xa, 0, 2, 2, 0), Cqp11L(0x26, 0, -1, -1, -1, 0),
    Cqp11D(0x27, 5, 0xa, 0, 6, 1, 0), Cqp11L(0x27, 0, -1, -1, -1, 0), Cqp11D(0x28, 8, 0xa, 0, 2, 2, 0),
    Cqp11L(0x28, 0, -1, -1, -1, 0), Cqp11D(0x29, 0xc, 0xa, 0, 6, 1, 0), Cqp11L(0x29, 0, -1, -1, -1, 0),
    Cqp11D(0x2a, 0x13, 0xa, 0, 6, 1, 0), Cqp11L(0x2a, 0, -1, -1, -1, 0), Cqp11D(0x2b, 0x1a, 0xa, 0, 6, 1, 0),
    Cqp11L(0x2b, 0, -1, -1, -1, 0),
];
