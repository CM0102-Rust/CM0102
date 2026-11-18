// ---------- mwf_1: the season turnovers of the Charity Shield, the Coupe de France and the French League Cup ----------

/// The season turnover of the Charity Shield: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; finally the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax it returns.
///
/// The cup turnover template (`start_new_cup_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (eng_charity.cpp, lines 0x128, 0x12f and 0x136) when it returns a negative value: the entrants (EngCharity::select_teams at 0x56d780, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `EngCharity::start_new_season` (0x56d9b0, eng_charity.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_charity_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x56d780, 0x9c425c, [0x128, 0x12f, 0x136], 1, "mwf_1 Charity Shield rollover") // mwf1: the turnover of the Charity Shield
}

/// Comparer for `eng_charity_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_eng_charity_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_CHARITY_START_NEW_SEASON);
    compare_cup_new_season("eng_charity_start_new_season", Cup(this), || o(this))
}

/// The season turnover of the Coupe de France: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; finally the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax it returns.
///
/// The cup turnover template (`start_new_cup_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (fra_cup.cpp, lines 0x23a, 0x241 and 0x248) when it returns a negative value: the entrants (FraCup::collect_entrants at 0x5baa50, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `FraCup::start_new_season` (0x5bae50, fra_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fra_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x5baa50, 0x9ccb28, [0x23a, 0x241, 0x248], 1, "mwf_1 Coupe de France rollover") // mwf1: the turnover of the Coupe de France
}

/// Comparer for `fra_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_fra_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FRA_CUP_START_NEW_SEASON);
    compare_cup_new_season("fra_cup_start_new_season", Cup(this), || o(this))
}

/// The season turnover of the French League Cup: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; finally the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax it returns.
///
/// The cup turnover template (`start_new_cup_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (fra_lge_cup.cpp, lines 0x1f9, 0x200 and 0x207) when it returns a negative value: the entrants (FraLgeCup::collect_entrants at 0x5bd160, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `FraLgeCup::start_new_season` (0x5bd420, fra_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fra_lge_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x5bd160, 0x9ccb98, [0x1f9, 0x200, 0x207], 1, "mwf_1 French League Cup rollover") // mwf1: the turnover of the French League Cup
}

/// Comparer for `fra_lge_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_fra_lge_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FRA_LGE_CUP_START_NEW_SEASON);
    compare_cup_new_season("fra_lge_cup_start_new_season", Cup(this), || o(this))
}

// ---------- mwf_2: the season turnover of the Korean FA Cup ----------

/// The season turnover of the Korean FA Cup: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; finally the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax it returns.
///
/// The cup turnover template (`start_new_cup_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (kor_fa_cup.cpp, lines 0x185, 0x18c and 0x193) when it returns a negative value: the entrants (KorFaCup::collect_entrants at 0x66aaf0, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `KorFaCup::start_new_season` (0x66acc0, kor_fa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_fa_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x66aaf0, 0x9eb0e8, [0x185, 0x18c, 0x193], 1, "mwf_2 Korean FA Cup rollover") // mwf2: the turnover of the Korean FA Cup
}

/// Comparer for `kor_fa_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_kor_fa_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_KOR_FA_CUP_START_NEW_SEASON);
    compare_cup_new_season("kor_fa_cup_start_new_season", Cup(this), || o(this))
}

// ---------- mwf_3: the season turnovers of the French, Serie C1 and Italian Super Cups ----------

/// The season turnover of the French Super Cup: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; no predicted positions: it returns the eax of the tie tree.
///
/// The cup turnover template (`usa_mls_all_stars_cup_start_new_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (fra_super.cpp, lines 0x127, 0x12e and 0x135) when it returns a negative value: the entrants (FraSuper::select_teams at 0x5bf770, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `FraSuper::start_new_season` (0x5bf9a0, fra_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fra_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x5bf770, 0x9ccc48, [0x127, 0x12e, 0x135], 1, false, "mwf_3 French Super Cup rollover") // mwf3: the turnover of the French Super Cup
}

/// Comparer for `fra_super_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_fra_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FRA_SUPER_START_NEW_SEASON);
    compare_cup_new_season("fra_super_start_new_season", Cup(this), || o(this))
}

/// The season turnover of the Serie C1 Super Cup: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; no predicted positions: it returns the eax of the tie tree.
///
/// The cup turnover template (`usa_mls_all_stars_cup_start_new_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (ita_c1_super.cpp, lines 0x10a, 0x111 and 0x118) when it returns a negative value: the entrants (ItaC1Super::init_team_list at 0x63c9d0, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `ItaC1Super::start_new_season` (0x63caa0, ita_c1_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_c1_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x63c9d0, 0x9ea1c8, [0x10a, 0x111, 0x118], 1, false, "mwf_3 Serie C1 Super Cup rollover") // mwf3: the turnover of the Serie C1 Super Cup
}

/// Comparer for `ita_c1_super_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_ita_c1_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_C1_SUPER_START_NEW_SEASON);
    compare_cup_new_season("ita_c1_super_start_new_season", Cup(this), || o(this))
}

/// The season turnover of the Italian Super Cup: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; no predicted positions: it returns the eax of the tie tree.
///
/// The cup turnover template (`usa_mls_all_stars_cup_start_new_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (ita_super.cpp, lines 0x125, 0x12c and 0x133) when it returns a negative value: the entrants (ItaSuper::init_team_list at 0x65fa60, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `ItaSuper::start_new_season` (0x65fc90, ita_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x65fa60, 0x9ea934, [0x125, 0x12c, 0x133], 1, false, "mwf_3 Italian Super Cup rollover") // mwf3: the turnover of the Italian Super Cup
}

/// Comparer for `ita_super_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_ita_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_SUPER_START_NEW_SEASON);
    compare_cup_new_season("ita_super_start_new_season", Cup(this), || o(this))
}

// ---------- mwf_4: the season turnovers of the Korean and Spanish Super Cups ----------

/// The season turnover of the Korean Super Cup: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; no predicted positions: it returns the eax of the tie tree.
///
/// The cup turnover template (`usa_mls_all_stars_cup_start_new_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (kor_super_cup.cpp, lines 0x134, 0x13b and 0x142) when it returns a negative value: the entrants (KorSuperCup::create_table at 0x66dc10, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `KorSuperCup::start_new_season` (0x66de90, kor_super_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_super_cup_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x66dc10, 0x9eb198, [0x134, 0x13b, 0x142], 1, false, "mwf_4 Korean Super Cup rollover") // mwf4: the turnover of the Korean Super Cup
}

/// Comparer for `kor_super_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_kor_super_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_KOR_SUPER_CUP_START_NEW_SEASON);
    compare_cup_new_season("kor_super_cup_start_new_season", Cup(this), || o(this))
}

/// The season turnover of the Spanish Super Cup: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; no predicted positions: it returns the eax of the tie tree.
///
/// The cup turnover template (`usa_mls_all_stars_cup_start_new_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (spa_super.cpp, lines 0x125, 0x12c and 0x133) when it returns a negative value: the entrants (SpaSuper::select_teams at 0x858aa0, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `SpaSuper::start_new_season` (0x858cd0, spa_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x858aa0, 0xa82a58, [0x125, 0x12c, 0x133], 1, false, "mwf_4 Spanish Super Cup rollover") // mwf4: the turnover of the Spanish Super Cup
}

/// Comparer for `spa_super_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_spa_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SPA_SUPER_START_NEW_SEASON);
    compare_cup_new_season("spa_super_start_new_season", Cup(this), || o(this))
}

// ---------- mwf_5: the season turnovers of the League Cup, the FA Cup and the Copa del Rey (eng_cc_cup.cpp, eng_fa_cup.cpp, spa_cup.cpp) ----------

/// The season turnover of a cup whose clubs come in round by round (the English League Cup and FA Cup, the Copa del
/// Rey): the same frees and archive as `start_new_cup_season`, but the team table is not filled by an entrants routine:
/// a zeroed one of the same size is allocated and the clubs are placed when their round is drawn [inference: from the
/// entrant book `cup::ENTRANT_BOOK`, built when the cup was created].
///
/// `cup::SEASON_CLOSED` = 0; the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed
/// (`exe::crt_free`) and zeroed; the ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds, a signed word)
/// freed and zeroed, then the array freed and zeroed; with statistics, Comp_Stats::archive_season_and_reset
/// (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the year (`cup::YEAR`) up by `year` (1);
/// `cup::CURRENT_ROUND` = 0, `cup::RUN_BY_STAGES` = 0 and `cup::SEASON_SET_UP` = 0; with `fa_cup`, also
/// `cup::NEXT_DRAW_ROUND` = 0 and `eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED` = 0 (a byte). The new team table:
/// calloc(`cup::TEAM_COUNT` (a signed word, read before the year moves), 6) (`exe::crt_calloc`, cdecl) into
/// `cup::TEAM_TABLE`; out of memory, the box of line `rows[0]`.
///
/// Then, each with its own box (lines `rows[1]` and `rows[2]`) when it returns a negative value: the round table
/// (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`); with `predicted`,
/// the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax it returns, otherwise the tie
/// tree's eax. After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read
/// it).
pub(crate) unsafe fn start_new_cup_season_empty_teams(this: Cup, file: usize, rows: [u32; 3], year: i16, fa_cup: bool, predicted: bool, ramo: &'static str) -> u32 {
    this.set_season_closed(0);
    for o in [cup::TEAM_TABLE, cup::ROUND_TABLE] {
        let l = read_usize(this.addr() + o);
        if l != 0 {
            exe::crt_free(l);
            write_usize(this.addr() + o, 0);
        }
    }
    if this.tie_tables() != 0 {
        let mut i = 0i32;
        while i < this.round_count() as i32 {
            let p = read_usize(this.tie_tables() + i as usize * 4);
            if p != 0 {
                exe::crt_free(p);
                write_usize(this.tie_tables() + i as usize * 4, 0);
            }
            i += 1;
        }
        exe::crt_free(this.tie_tables());
        this.set_tie_tables(0);
    }
    let stats = read_usize(this.addr() + 8);
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    let teams = this.team_count() as i32;
    let a = this.year();
    this.set_year(a.wrapping_add(year));
    covered!(ramo);
    if fa_cup {
        this.set_next_draw_round(0);
    }
    this.set_current_round(0);
    this.set_run_by_stages(0);
    this.set_season_set_up(0);
    if fa_cup {
        write_u8(this.addr() + eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED, 0);
    }
    let t = exe::crt_calloc(teams as u32, 6);
    this.set_team_table(t);
    if t == 0 {
        show_error_at(file, rows[0]);
        return 0;
    }
    if call_cup_virtual(this, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(file, rows[1]);
        return 0;
    }
    let r = call_cup_virtual(this, cup_vtable::MAP_FIXTURE_TREE);
    if r < 0 {
        show_error_at(file, rows[2]);
        return 0;
    }
    if !predicted {
        return r as u32;
    }
    call_cup_virtual(this, comp_vtable::SET_PREDICTED_POSITIONS) as u32
}

/// The season turnover of the League Cup: last season's tables freed, the season archived, the year plus one, an empty
/// team table of the same size, the rounds and the tie tree (no predicted positions).
///
/// `start_new_cup_season_empty_teams` with the eng_cc_cup.cpp boxes at lines 0x2ad, 0x2b4 and 0x2bb.
/// Original: `EngCcCup::start_new_season` (0x56d130, eng_cc_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_cc_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season_empty_teams(Cup(this), 0x9c4224, [0x2ad, 0x2b4, 0x2bb], 1, false, false, "mwf_5 league cup rollover") // mwf5: the League Cup turnover
}

/// Comparer for `eng_cc_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go
/// up by one.
pub(crate) unsafe extern "thiscall" fn compare_eng_cc_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_CC_CUP_START_NEW_SEASON);
    compare_cup_new_season("eng_cc_cup_start_new_season", Cup(this), || o(this))
}

/// The season turnover of the FA Cup: as the League Cup's, and it also clears the next round to draw and the question
/// about entering at the 5th round (no predicted positions).
///
/// `start_new_cup_season_empty_teams` with `fa_cup` (`cup::NEXT_DRAW_ROUND` and
/// `eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED` zeroed) and the eng_fa_cup.cpp boxes at lines 0x372, 0x379 and 0x380.
/// Original: `EngFaCup::start_new_season` (0x570390, eng_fa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_fa_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season_empty_teams(Cup(this), 0x9c42cc, [0x372, 0x379, 0x380], 1, true, false, "mwf_5 fa cup rollover") // mwf5: the FA Cup turnover
}

/// Comparer for `eng_fa_cup_start_new_season` (original only; the year plus one).
pub(crate) unsafe extern "thiscall" fn compare_eng_fa_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_FA_CUP_START_NEW_SEASON);
    compare_cup_new_season("eng_fa_cup_start_new_season", Cup(this), || o(this))
}

/// The season turnover of the Copa del Rey: as the League Cup's, then the predicted positions.
///
/// `start_new_cup_season_empty_teams` with `predicted` and the spa_cup.cpp boxes at lines 0x2de, 0x2e6 and 0x2ed.
/// Original: `SpaCup::start_new_season` (0x84df70, spa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season_empty_teams(Cup(this), 0xa82810, [0x2de, 0x2e6, 0x2ed], 1, false, true, "mwf_5 copa del rey rollover") // mwf5: the Copa del Rey turnover
}

/// Comparer for `spa_cup_start_new_season` (original only; the year plus one).
pub(crate) unsafe extern "thiscall" fn compare_spa_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SPA_CUP_START_NEW_SEASON);
    compare_cup_new_season("spa_cup_start_new_season", Cup(this), || o(this))
}

// ---------- mwf_6: the season turnovers of the English First, Second and Third Divisions ----------

/// The season turnover of the English First Division: the season archived in the statistics, the table, the round dates and the stages
/// cleared, the year moved on by one and the rules, the table, the fixtures and the order rebuilt; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// The league turnover template of Brazil's Series B (`bra_nat_second_start_new_season`, checked instruction by
/// instruction): `league::SEASON_CLOSED` = 0, then `rebuild_season`: the table freed (League::free_table
/// `exe::league_free_table`, thiscall(this, 0), ret 4); `.round_dates()` freed (`exe::crt_free`) and zeroed; with
/// statistics, Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1),
/// ret 4); with a current stage ≥ 0, the stages 0..=`.current_stage()` destroyed (virtual 0 with 1; the stage array
/// itself is kept); the year up by one and `league::CURRENT_STAGE` = −1. Then, each with its own box (eng_first.cpp, lines 0x29e, 0x2a5,
/// 0x2ab and 0x2b2) when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table (EngFirst::create_table at 0x572bf0, thiscall, ret 0),
/// the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table
/// `exe::league_sort_table`, thiscall(this, 0), ret 4); then the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`), the board expectations (League::set_club_season_expectations) and the
/// referee renewal (OfficialsManager::renew_officials `exe::officialsmanager_renew_officials`,
/// thiscall([`G_OFFICIALS_MANAGER`], `.competition()`), ret 4), whose eax it returns. After a box the copy returns 0
/// (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `EngFirst::start_new_season` (0x572e30, eng_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x572bf0, 0x9c4374, [0x29e, 0x2a5, 0x2ab, 0x2b2], 1) { // mwf6: the turnover of the English First Division
        return 0;
    }
    covered!("mwf_6 English First Division rollover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `eng_first_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_eng_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("eng_first_start_new_season", League(this), || o(this))
}

/// The season turnover of the English Second Division: the season archived in the statistics, the table, the round dates and the stages
/// cleared, the year moved on by one and the rules, the table, the fixtures and the order rebuilt; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// The league turnover template of Brazil's Series B (`bra_nat_second_start_new_season`, checked instruction by
/// instruction): `league::SEASON_CLOSED` = 0, then `rebuild_season`: the table freed (League::free_table
/// `exe::league_free_table`, thiscall(this, 0), ret 4); `.round_dates()` freed (`exe::crt_free`) and zeroed; with
/// statistics, Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1),
/// ret 4); with a current stage ≥ 0, the stages 0..=`.current_stage()` destroyed (virtual 0 with 1; the stage array
/// itself is kept); the year up by one and `league::CURRENT_STAGE` = −1. Then, each with its own box (eng_second.cpp, lines 0x264, 0x26b,
/// 0x271 and 0x278) when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table (EngSecond::create_table at 0x576630, thiscall, ret 0),
/// the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table
/// `exe::league_sort_table`, thiscall(this, 0), ret 4); then the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`), the board expectations (League::set_club_season_expectations) and the
/// referee renewal (OfficialsManager::renew_officials `exe::officialsmanager_renew_officials`,
/// thiscall([`G_OFFICIALS_MANAGER`], `.competition()`), ret 4), whose eax it returns. After a box the copy returns 0
/// (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `EngSecond::start_new_season` (0x576820, eng_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x576630, 0x9c43e8, [0x264, 0x26b, 0x271, 0x278], 1) { // mwf6: the turnover of the English Second Division
        return 0;
    }
    covered!("mwf_6 English Second Division rollover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `eng_second_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_eng_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_SECOND_START_NEW_SEASON);
    compare_state_league_new_season("eng_second_start_new_season", League(this), || o(this))
}

/// The season turnover of the English Third Division: the season archived in the statistics, the table, the round dates and the stages
/// cleared, the year moved on by one and the rules, the table, the fixtures and the order rebuilt; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// The league turnover template of Brazil's Series B (`bra_nat_second_start_new_season`, checked instruction by
/// instruction): `league::SEASON_CLOSED` = 0, then `rebuild_season`: the table freed (League::free_table
/// `exe::league_free_table`, thiscall(this, 0), ret 4); `.round_dates()` freed (`exe::crt_free`) and zeroed; with
/// statistics, Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1),
/// ret 4); with a current stage ≥ 0, the stages 0..=`.current_stage()` destroyed (virtual 0 with 1; the stage array
/// itself is kept); the year up by one and `league::CURRENT_STAGE` = −1. Then, each with its own box (eng_third.cpp, lines 0x26c, 0x273,
/// 0x279 and 0x280) when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table (EngThird::create_table at 0x577f70, thiscall, ret 0),
/// the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table
/// `exe::league_sort_table`, thiscall(this, 0), ret 4); then the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`), the board expectations (League::set_club_season_expectations) and the
/// referee renewal (OfficialsManager::renew_officials `exe::officialsmanager_renew_officials`,
/// thiscall([`G_OFFICIALS_MANAGER`], `.competition()`), ret 4), whose eax it returns. After a box the copy returns 0
/// (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `EngThird::start_new_season` (0x578170, eng_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_third_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x577f70, 0x9c4424, [0x26c, 0x273, 0x279, 0x280], 1) { // mwf6: the turnover of the English Third Division
        return 0;
    }
    covered!("mwf_6 English Third Division rollover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `eng_third_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_eng_third_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_THIRD_START_NEW_SEASON);
    compare_state_league_new_season("eng_third_start_new_season", League(this), || o(this))
}

// ---------- mwf_7: the season turnovers of the French Second Division, the French National and Serie C1 A ----------

/// The season turnover of the French Second Division: the season archived in the statistics, the table, the round dates and the stages
/// cleared, the year moved on by one and the rules, the table, the fixtures and the order rebuilt; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// The league turnover template of Brazil's Series B (`bra_nat_second_start_new_season`, checked instruction by
/// instruction): `league::SEASON_CLOSED` = 0, then `rebuild_season`: the table freed (League::free_table
/// `exe::league_free_table`, thiscall(this, 0), ret 4); `.round_dates()` freed (`exe::crt_free`) and zeroed; with
/// statistics, Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1),
/// ret 4); with a current stage ≥ 0, the stages 0..=`.current_stage()` destroyed (virtual 0 with 1; the stage array
/// itself is kept); the year up by one and `league::CURRENT_STAGE` = −1. Then, each with its own box (fra_second.cpp, lines 0x1e4, 0x1ec,
/// 0x1f2 and 0x1f9) when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table (FraSecond::create_table at 0x5bef10, thiscall, ret 0),
/// the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table
/// `exe::league_sort_table`, thiscall(this, 0), ret 4); then the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`), the board expectations (League::set_club_season_expectations) and the
/// referee renewal (OfficialsManager::renew_officials `exe::officialsmanager_renew_officials`,
/// thiscall([`G_OFFICIALS_MANAGER`], `.competition()`), ret 4), whose eax it returns. After a box the copy returns 0
/// (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `FraSecond::start_new_season` (0x5bf160, fra_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fra_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x5bef10, 0x9ccc0c, [0x1e4, 0x1ec, 0x1f2, 0x1f9], 1) { // mwf7: the turnover of the French Second Division
        return 0;
    }
    covered!("mwf_7 French Second Division rollover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `fra_second_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_fra_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FRA_SECOND_START_NEW_SEASON);
    compare_state_league_new_season("fra_second_start_new_season", League(this), || o(this))
}

/// The season turnover of the French National: the season archived in the statistics, the table, the round dates and the stages
/// cleared, the year moved on by one and the rules, the table, the fixtures and the order rebuilt; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// The league turnover template of Brazil's Series B (`bra_nat_second_start_new_season`, checked instruction by
/// instruction): `league::SEASON_CLOSED` = 0, then `rebuild_season`: the table freed (League::free_table
/// `exe::league_free_table`, thiscall(this, 0), ret 4); `.round_dates()` freed (`exe::crt_free`) and zeroed; with
/// statistics, Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1),
/// ret 4); with a current stage ≥ 0, the stages 0..=`.current_stage()` destroyed (virtual 0 with 1; the stage array
/// itself is kept); the year up by one and `league::CURRENT_STAGE` = −1. Then, each with its own box (fra_third.cpp, lines 0x1ce, 0x1d5,
/// 0x1db and 0x1e2) when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table (FraThird::create_table at 0x5c05d0, thiscall, ret 0),
/// the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table
/// `exe::league_sort_table`, thiscall(this, 0), ret 4); then the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`), the board expectations (League::set_club_season_expectations) and the
/// referee renewal (OfficialsManager::renew_officials `exe::officialsmanager_renew_officials`,
/// thiscall([`G_OFFICIALS_MANAGER`], `.competition()`), ret 4), whose eax it returns. After a box the copy returns 0
/// (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `FraThird::start_new_season` (0x5c07c0, fra_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fra_third_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x5c05d0, 0x9ccc80, [0x1ce, 0x1d5, 0x1db, 0x1e2], 1) { // mwf7: the turnover of the French National
        return 0;
    }
    covered!("mwf_7 French National rollover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `fra_third_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_fra_third_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FRA_THIRD_START_NEW_SEASON);
    compare_state_league_new_season("fra_third_start_new_season", League(this), || o(this))
}

/// The season turnover of Serie C1 A: the season archived in the statistics, the table, the round dates and the stages
/// cleared, the year moved on by one and the rules, the table, the fixtures and the order rebuilt; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// The league turnover template of Brazil's Series B (`bra_nat_second_start_new_season`, checked instruction by
/// instruction): `league::SEASON_CLOSED` = 0, then `rebuild_season`: the table freed (League::free_table
/// `exe::league_free_table`, thiscall(this, 0), ret 4); `.round_dates()` freed (`exe::crt_free`) and zeroed; with
/// statistics, Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1),
/// ret 4); with a current stage ≥ 0, the stages 0..=`.current_stage()` destroyed (virtual 0 with 1; the stage array
/// itself is kept); the year up by one and `league::CURRENT_STAGE` = −1. Then, each with its own box (ita_ser_c1a.cpp, lines 0x220, 0x227,
/// 0x22d and 0x234) when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table (ItaSerC1a::create_table at 0x64b670, thiscall, ret 0),
/// the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table
/// `exe::league_sort_table`, thiscall(this, 0), ret 4); then the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`), the board expectations (League::set_club_season_expectations) and the
/// referee renewal (OfficialsManager::renew_officials `exe::officialsmanager_renew_officials`,
/// thiscall([`G_OFFICIALS_MANAGER`], `.competition()`), ret 4), whose eax it returns. After a box the copy returns 0
/// (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `ItaSerC1a::start_new_season` (0x64b820, ita_ser_c1a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c1a_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x64b670, 0x9ea474, [0x220, 0x227, 0x22d, 0x234], 1) { // mwf7: the turnover of Serie C1 A
        return 0;
    }
    covered!("mwf_7 Serie C1 A rollover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `ita_ser_c1a_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c1a_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_SER_C1A_START_NEW_SEASON);
    compare_state_league_new_season("ita_ser_c1a_start_new_season", League(this), || o(this))
}

// ---------- mwf_8: the season turnovers of Serie C1 B, the Spanish Second Division and the Spanish lower leagues (ita_ser_c1b.cpp, spa_second.cpp, spa_lower.cpp) ----------

/// The season turnover of Serie C1 B: the season archived in the statistics, the table, the round dates and the stages
/// cleared, the year moved on by one and the rules, the table, the fixtures and the order rebuilt; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// The league turnover template of Brazil's Series B (`bra_nat_second_start_new_season`, checked instruction by
/// instruction): `league::SEASON_CLOSED` = 0, then `rebuild_season`: the table freed (League::free_table
/// `exe::league_free_table`, thiscall(this, 0), ret 4); `.round_dates()` freed (`exe::crt_free`) and zeroed; with
/// statistics, Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1),
/// ret 4); with a current stage ≥ 0, the stages 0..=`.current_stage()` destroyed (virtual 0 with 1; the stage array
/// itself is kept); the year up by one and `league::CURRENT_STAGE` = −1. Then, each with its own box (ita_ser_c1b.cpp, lines 0x224, 0x22b,
/// 0x231 and 0x238) when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table (ItaSerC1b::create_table at 0x64f820, thiscall, ret 0),
/// the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table
/// `exe::league_sort_table`, thiscall(this, 0), ret 4); then the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`), the board expectations (League::set_club_season_expectations) and the
/// referee renewal (OfficialsManager::renew_officials `exe::officialsmanager_renew_officials`,
/// thiscall([`G_OFFICIALS_MANAGER`], `.competition()`), ret 4), whose eax it returns. After a box the copy returns 0
/// (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `ItaSerC1b::start_new_season` (0x64f9e0, ita_ser_c1b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c1b_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x64f820, 0x9ea558, [0x224, 0x22b, 0x231, 0x238], 1) { // mwf8: the turnover of Serie C1 B
        return 0;
    }
    covered!("mwf_8 Serie C1 B rollover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `ita_ser_c1b_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c1b_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_SER_C1B_START_NEW_SEASON);
    compare_state_league_new_season("ita_ser_c1b_start_new_season", League(this), || o(this))
}

/// The season turnover of the Spanish Second Division: the season archived in the statistics, the table, the round dates and the stages
/// cleared, the year moved on by one and the rules, the table, the fixtures and the order rebuilt; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// The league turnover template of Brazil's Series B (`bra_nat_second_start_new_season`, checked instruction by
/// instruction): `league::SEASON_CLOSED` = 0, then `rebuild_season`: the table freed (League::free_table
/// `exe::league_free_table`, thiscall(this, 0), ret 4); `.round_dates()` freed (`exe::crt_free`) and zeroed; with
/// statistics, Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1),
/// ret 4); with a current stage ≥ 0, the stages 0..=`.current_stage()` destroyed (virtual 0 with 1; the stage array
/// itself is kept); the year up by one and `league::CURRENT_STAGE` = −1. Then, each with its own box (spa_second.cpp, lines 0x312, 0x319,
/// 0x31f and 0x326) when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table (SpaSecond::create_table at 0x855b40, thiscall, ret 0),
/// the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table
/// `exe::league_sort_table`, thiscall(this, 0), ret 4); then the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`), the board expectations (League::set_club_season_expectations) and the
/// referee renewal (OfficialsManager::renew_officials `exe::officialsmanager_renew_officials`,
/// thiscall([`G_OFFICIALS_MANAGER`], `.competition()`), ret 4), whose eax it returns. After a box the copy returns 0
/// (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `SpaSecond::start_new_season` (0x855d80, spa_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x855b40, 0xa828bc, [0x312, 0x319, 0x31f, 0x326], 1) { // mwf8: the turnover of the Spanish Second Division
        return 0;
    }
    covered!("mwf_8 Spanish Second Division rollover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `spa_second_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_spa_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SPA_SECOND_START_NEW_SEASON);
    compare_state_league_new_season("spa_second_start_new_season", League(this), || o(this))
}

/// The front of a league turnover without `rebuild_season` (checked instruction by instruction):
/// `league::SEASON_CLOSED` = 0; the table freed (League::free_table `exe::league_free_table`, thiscall(this, 0), ret 4);
/// `.round_dates()` freed (`exe::crt_free`) and zeroed; with statistics, Comp_Stats::archive_season_and_reset
/// (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); with a current stage ≥ 0, the stages
/// 0..=`.current_stage()` destroyed (virtual 0 with 1; the stage array is kept); the year up by `year` and
/// `league::CURRENT_STAGE` = −1.
pub(crate) unsafe fn spa_lower_clear_season(this: League, year: i16) {
    this.set_season_closed(0);
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    if (this.current_stage() as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(this.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
    let a = this.year();
    this.set_year(a.wrapping_add(year));
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
}

/// The season turnover of a Spanish lower league (the Segunda B groups [inference]): the season archived, the table,
/// the dates and the stages cleared, the year plus one, then the rules, the table and the fixtures rebuilt and the
/// referees renewed; unlike the upper divisions there is no table order, no predicted positions and no board
/// expectations.
///
/// `spa_lower_clear_season` (one year); then, each with its own box (spa_lower.cpp, lines 0x36c, 0x373 and 0x379) when
/// it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table (SpaLower::create_table 0x8533d0, thiscall,
/// ret 0: the B teams of the First and Second Division clubs, then the best other lower clubs) and the fixtures
/// (League::create_fixtures `exe::league_create_fixtures`); finally the referee renewal
/// (OfficialsManager::renew_officials `exe::officialsmanager_renew_officials`, thiscall([`G_OFFICIALS_MANAGER`],
/// `.competition()`), ret 4), whose eax it returns. After a box the copy returns 0 (the original returns the box's eax;
/// the turnover's caller does not read it).
/// Original: `SpaLower::start_new_season` (0x853830, spa_lower.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_lower_start_new_season(this: usize) -> u32 {
    let this = League(this);
    let year = 1; // mwf8: the year of the Spanish lower league
    spa_lower_clear_season(this, year);
    covered!("mwf_8 spanish lower league rollover");
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(0xa82880, 0x36c);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x8533d0usize);
    if table(this.addr()) == 0 {
        show_error_at(0xa82880, 0x373);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(0xa82880, 0x379);
        return 0;
    }
    exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), this.competition().addr()) as u32
}

/// Comparer for `spa_lower_start_new_season`: only the original runs (it frees and rebuilds the league); the year must
/// go up by one.
pub(crate) unsafe extern "thiscall" fn compare_spa_lower_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SPA_LOWER_START_NEW_SEASON);
    compare_state_league_new_season("spa_lower_start_new_season", League(this), || o(this))
}

// ---------- mwf_9: the season turnovers of the Serie C Italian Cup, the Coppa Italia and the Korean League Cup (ita_c_cup.cpp, ita_cup.cpp, kor_league_cup.cpp) ----------

/// The season turnover of a cup played as a league with group stages (the Coppa Italia, the Serie C cup and the Korean
/// League Cup: the same body, checked instruction by instruction for the two Italian ones and by reading for the Korean
/// one, which only uses another register as zero; only the routines, the number of groups, the source file and the box
/// lines change).
///
/// `league::SEASON_CLOSED` = 0; the table freed (League::free_table `exe::league_free_table`, thiscall(this, 0), ret 4);
/// `.round_dates()` freed (`exe::crt_free`) and zeroed; with statistics, Comp_Stats::archive_season_and_reset
/// (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the entrant list at `+0xee`
/// (`ita_cup_fields::ENTRANT_CLUBS`; the ten selected clubs in the Korean League Cup) freed when there is one, but not
/// zeroed (the entrants routine allocates a new one next); with a current stage ≥ 0, the stages 0..=`.current_stage()` destroyed (virtual 0 with 1); the year up by
/// `year` and `league::CURRENT_STAGE` = −1.
///
/// Then the entrants (`entrants`, thiscall, ret 0; its result is not checked) and the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`); then, each with its own box (lines `rows`) when it returns 0: the rules
/// (virtual `comp_vtable::SET_UP_RULES`), the first group's table (`first_group`, thiscall, ret 0: the cup itself is
/// group 1), the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table
/// `exe::league_sort_table`, thiscall(this, 0), ret 4). Finally the group stages `group_stage`(g) for g = 0 to
/// `groups` − 1 (thiscall, ret 4), the last one's eax returned. After a box the copy returns 0 (the original returns
/// the box's eax; the turnover's caller does not read it).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn ita_group_cup_start_new_season(this: League, entrants: usize, first_group: usize, group_stage: usize, groups: u32, file: usize, rows: [u32; 4], year: i16, ramo: &'static str) -> u32 {
    this.set_season_closed(0);
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    let list = read_usize(this.addr() + ita_cup_fields::ENTRANT_CLUBS);
    if list != 0 {
        exe::crt_free(list);
    }
    if (this.current_stage() as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(this.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
    let a = this.year();
    this.set_year(a.wrapping_add(year));
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    covered!(ramo);
    let entrants: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(entrants);
    entrants(this.addr());
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, rows[0]);
        return 0;
    }
    let first_group: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(first_group);
    if first_group(this.addr()) == 0 {
        show_error_at(file, rows[1]);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, rows[2]);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, rows[3]);
        return 0;
    }
    let group_stage: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(group_stage);
    let mut r = 0;
    for g in 0..groups {
        r = group_stage(this.addr(), g);
    }
    r
}

/// The season turnover of the Serie C Italian Cup: the season archived, last season's stages and entrant list dropped,
/// the year plus one, the 90 Serie C entrants drawn again (ItaCCup::select_entrants), the predicted positions, the
/// rules, the first group of 5 (ItaCCup::init_first_group), its fixtures and order, and the 15 group stages
/// (ItaCCup::create_group_stage 0..14).
///
/// `ita_group_cup_start_new_season` with 0x63dfe0, 0x63d5e0, 0x63d9e0, 15 groups and the ita_c_cup.cpp boxes at lines
/// 0x212, 0x219, 0x21f and 0x226.
/// Original: `ItaCCup::start_new_season` (0x63d780, ita_c_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_c_cup_start_new_season(this: usize) -> u32 {
    ita_group_cup_start_new_season(League(this), 0x63dfe0, 0x63d5e0, 0x63d9e0, 15, 0x9ea204, [0x212, 0x219, 0x21f, 0x226], 1, "mwf_9 serie c cup rollover") // mwf9: the Serie C cup turnover
}

/// Comparer for `ita_c_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go
/// up by one.
pub(crate) unsafe extern "thiscall" fn compare_ita_c_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_C_CUP_START_NEW_SEASON);
    compare_state_league_new_season("ita_c_cup_start_new_season", League(this), || o(this))
}

/// The season turnover of the Coppa Italia: as the Serie C cup's, with the 48 entrants (ItaCup::select_entrants), the
/// first group of 4 (ItaCup::create_first_group_table) and 7 group stages (ItaCup::create_group_stage 0..6).
///
/// `ita_group_cup_start_new_season` with 0x63faf0, 0x63f250, 0x63f640, 7 groups and the ita_cup.cpp boxes at lines
/// 0x210, 0x217, 0x21d and 0x224.
/// Original: `ItaCup::start_new_season` (0x63f3e0, ita_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_cup_start_new_season(this: usize) -> u32 {
    ita_group_cup_start_new_season(League(this), 0x63faf0, 0x63f250, 0x63f640, 7, 0x9ea23c, [0x210, 0x217, 0x21d, 0x224], 1, "mwf_9 coppa italia rollover") // mwf9: the Coppa Italia turnover
}

/// Comparer for `ita_cup_start_new_season` (original only; the year plus one).
pub(crate) unsafe extern "thiscall" fn compare_ita_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_CUP_START_NEW_SEASON);
    compare_state_league_new_season("ita_cup_start_new_season", League(this), || o(this))
}

/// The season turnover of the Korean League Cup: the season archived, last season's stages and team list dropped, the
/// year plus one, the ten K League clubs selected again (KorLeagueCup::select_teams, sorted by last season's
/// position), the predicted positions, the rules, group A (KorLeagueCup::create_group_a_table: the even-numbered seeds,
/// the cup itself), its fixtures and order, and the group B stage (KorLeagueCup::create_group_b_stage: the odd-numbered
/// seeds).
///
/// `ita_group_cup_start_new_season` with 0x66cf20, 0x66c770, 0x66cb60 (push 0: the one group stage, ret 4) and the
/// kor_league_cup.cpp boxes at lines 0x1f1, 0x1f8, 0x1fe and 0x205.
/// Original: `KorLeagueCup::start_new_season` (0x66c900, kor_league_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_league_cup_start_new_season(this: usize) -> u32 {
    ita_group_cup_start_new_season(League(this), 0x66cf20, 0x66c770, 0x66cb60, 1, 0x9eb15c, [0x1f1, 0x1f8, 0x1fe, 0x205], 1, "mwf_9 korean league cup rollover") // mwf9: the Korean League Cup turnover
}

/// Comparer for `kor_league_cup_start_new_season` (original only; the year plus one).
pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_KOR_LEAGUE_CUP_START_NEW_SEASON);
    compare_state_league_new_season("kor_league_cup_start_new_season", League(this), || o(this))
}

// ---------- mwf_10: the Serie B season turnover and the Serie C1 B final positions (ita_ser_b.cpp, ita_ser_c1b.cpp) ----------

/// The season turnover of Serie B: the season archived, the table and the dates cleared, the stage array dropped, the
/// year plus one, then the rules, the 20 Serie B clubs, the fixtures and the order rebuilt; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// `league::SEASON_CLOSED` = 0; the table freed (League::free_table `exe::league_free_table`, thiscall(this, 0), ret 4);
/// `.round_dates()` freed (`exe::crt_free`) and zeroed; with statistics, Comp_Stats::archive_season_and_reset
/// (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the stage array (`.stages()`) freed when
/// there is one, without destroying the stage objects (Serie B has none [inference]), and zeroed, `league::STAGE_COUNT`
/// = 0; the year up by one; `league::CURRENT_STAGE` = −1; and `league::SEASON_CLOSED` = 1 (as Serie A's turnover
/// also does; the other sisters leave 0; a possible defect of the original [inference: the end of the next season may
/// be taken as already closed]).
///
/// Then, each with its own box (ita_ser_b.cpp, lines 0x20c, 0x213, 0x219 and 0x220) when it returns 0: the rules
/// (virtual `comp_vtable::SET_UP_RULES`), the table (ItaSerB::create_table 0x646780, thiscall, ret 0: the 20 Serie B
/// clubs and a prize fund from 70000), the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the
/// order (League::sort_table `exe::league_sort_table`, thiscall(this, 0), ret 4); then `set_expectations_and_renew_officials`
/// after the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax (the referee renewal's)
/// it returns. After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read
/// it).
/// Original: `ItaSerB::start_new_season` (0x646960, ita_ser_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_b_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    if this.stages() != 0 {
        exe::crt_free(this.stages());
    }
    let a = this.year();
    this.set_year(a.wrapping_add(1)); // mwf10: the year of Serie B
    this.set_stages(0);
    write_u32(this.addr() + league::STAGE_COUNT, 0);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    this.set_season_closed(1);
    covered!("mwf_10 serie b rollover");
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(0x9ea384, 0x20c);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x646780usize);
    if table(this.addr()) == 0 {
        show_error_at(0x9ea384, 0x213);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(0x9ea384, 0x219);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(0x9ea384, 0x220);
        return 0;
    }
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// Comparer for `ita_ser_b_start_new_season`: only the original runs (it frees and rebuilds the league); the year must
/// go up by one.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_b_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_SER_B_START_NEW_SEASON);
    compare_state_league_new_season("ita_ser_b_start_new_season", League(this), || o(this))
}

/// The Serie C1 B final positions: as League::record_final_positions for the table, then the promotion play-off
/// overrides the places of its clubs: the winner 2nd, the runner-up 3rd and the others 4th, 5th... in the order of the
/// play-off's team table.
///
/// The play-off is stage 0 (`.stages()`[0], read first). Each club of the table rows 0..`.club_count()` (a signed
/// word) whose division (`club.division()`) is this league stores `club::LAST_DIVISION` = the league and the place
/// i + 1 (`set_last_position`). Then each entry of the play-off's team table (`cup::TEAM_TABLE`, 6-byte entries,
/// `.team_count()` of them, a signed word): status (+5) 1 gives place 2, status 2 gives place 3, any other the next
/// place from 4 on (a byte counter).
/// Original: `ItaSerC1b::record_final_positions` (0x650050, ita_ser_c1b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c1b_record_final_positions(this: usize) {
    let this = League(this);
    let playoff = Cup(read_usize(this.stages()));
    let mut i = 0i16;
    while i < this.club_count() {
        let division = this.competition().addr();
        let c = Club(read_usize(this.table() + i as usize * TABLE_ROW_SIZE));
        if c.division().addr() == division {
            write_usize(c.addr() + club::LAST_DIVISION, division);
            c.set_last_position((i as u8).wrapping_add(1));
        }
        i += 1;
    }
    covered!("mwf_10 serie c1 b play-off places");
    let mut next = 4u8;
    let mut k = 0i16;
    while k < playoff.team_count() {
        let e = playoff.team_table() + k as usize * 6;
        let c = Club(read_usize(e));
        match read_u8(e + 5) {
            1 => c.set_last_position(2), // mwf10: the play-off winner 2nd
            2 => c.set_last_position(3),
            _ => {
                c.set_last_position(next);
                next = next.wrapping_add(1);
            }
        }
        k += 1;
    }
}

/// Comparer for `ita_ser_c1b_record_final_positions`: side by side (it only writes `club::LAST_DIVISION` and
/// `club::LAST_POSITION` of the table and play-off clubs, so both can run from the same state), over those bytes.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c1b_record_final_positions(this: usize) {
    let l = League(this);
    let mut reg: Vec<(usize, usize)> = (0..l.club_count().max(0) as usize).map(|i| (read_usize(l.table() + i * TABLE_ROW_SIZE) + club::LAST_DIVISION, 5)).collect();
    let playoff = Cup(read_usize(l.stages()));
    reg.extend((0..playoff.team_count().max(0) as usize).map(|k| (read_usize(playoff.team_table() + k * 6) + club::LAST_POSITION, 1)));
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ITA_SER_C1B_RECORD_FINAL_POSITIONS);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || ita_ser_c1b_record_final_positions(this));
    SCOREBOARD.record(same, || format!("ita_ser_c1b_record_final_positions({:#x}) {:?} {:?}", this, mo, mr));
}


// ---------- mwf_11: the Korean league turnover and the rules of the Korean league and League Cup (kor_league.cpp, kor_league_cup.cpp) ----------

/// The season turnover of the Korean league (the K League): the final positions recorded first, then the season
/// archived, the table and the dates cleared, the year plus one, the rules, the ten clubs, the fixtures and the order
/// rebuilt; then the predicted positions, the board expectations and the referee renewal. Unlike the sisters it does
/// not touch the stages (the K League has none) nor `league::CURRENT_STAGE`.
///
/// `league::SEASON_CLOSED` = 0; the final positions (virtual `league_vtable::RECORD_FINAL_POSITIONS`, thiscall, ret 0);
/// the table freed (League::free_table `exe::league_free_table`, thiscall(this, 0), ret 4); `.round_dates()` freed
/// (`exe::crt_free`) and zeroed; with statistics, Comp_Stats::archive_season_and_reset
/// (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the year up by one. Then, each with its own
/// box (kor_league.cpp, lines 0x206, 0x20d, 0x213 and 0x21a) when it returns 0: the rules (virtual
/// `comp_vtable::SET_UP_RULES`), the table (KorLeague::create_table 0x66bba0, thiscall, ret 0: the ten clubs of the
/// division and the prize fund), the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order
/// (League::sort_table `exe::league_sort_table`, thiscall(this, 0), ret 4); then the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`) and `set_expectations_and_renew_officials`, whose eax (the referee
/// renewal's) it returns. After a box the copy returns 0 (the original returns the box's eax; the turnover's caller
/// does not read it).
/// Original: `KorLeague::start_new_season` (0x66bd60, kor_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_league_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    bra_nat_first_call_virtual_no_args(this.addr(), league_vtable::RECORD_FINAL_POSITIONS);
    exe::league_free_table(this, 0);
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
    this.set_year(a.wrapping_add(1)); // mwf11: the year of the K League
    covered!("mwf_11 k league rollover");
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(0x9eb120, 0x206);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x66bba0usize);
    if table(this.addr()) == 0 {
        show_error_at(0x9eb120, 0x20d);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(0x9eb120, 0x213);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(0x9eb120, 0x21a);
        return 0;
    }
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// Comparer for `kor_league_start_new_season`: only the original runs (it frees and rebuilds the league); the year must
/// go up by one.
pub(crate) unsafe extern "thiscall" fn compare_kor_league_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_KOR_LEAGUE_START_NEW_SEASON);
    compare_state_league_new_season("kor_league_start_new_season", League(this), || o(this))
}

/// Writes the K League rule fields (what `kor_league_set_up_rules` writes before the schedule).
pub(crate) unsafe fn set_k_league_rules(o: usize) {
    write_u16_at(o, 0xab, 0); // no match rule bits
    write_u16_at(o, 0xdb, 0);
    write_u8_at(o, 0xbe, 0); // nobody goes up or down
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 0);
    write_u16_at(o, 0x3c, 3); // three round robins
    write_u8_at(o, 0xc2, 3); // mwf11: the K League points per win
    write_u8_at(o, 0xc7, 3);
    write_u8_at(o, 0x4a, 3); // substitutions
    write_u8_at(o, 0xc4, 2); // match category: league
    write_u8_at(o, 0xc6, 2);
    write_u8_at(o, 0x52, 2); // fixed squad numbering
    write_u8_at(o, 0xc3, 1); // points per draw
    write_u8_at(o, 0x42, 1); // club sides
    write_u8_at(o, 0xc5, 1); // tiebreakers 1, 2, 3
    write_u32_at(o, 0x1c, 0xffff_ffff); // no league above
    write_u32_at(o, 0x20, 0xffff_ffff); // no league below
    write_u8_at(o, 0x49, 7); // substitutes on the bench
}

/// The K League rules: a triple round robin (3 legs), 3 points a win and 1 a draw, tiebreakers 1, 2 and 3, no
/// promotion or relegation (no league above or below), club sides, league category, 7 substitutes and 3 changes, fixed
/// squad numbers; then the season schedule.
///
/// Writes the bytes and words of `set_k_league_rules` (`+0xab` and `+0xdb` words 0, `+0xbe`..`+0xc1` 0, `+0x3c` word
/// 3, `+0xc2` 3, `+0xc7` 3, `+0x4a` 3, `+0xc4` 2, `+0xc6` 2, `+0x52` 2, `+0xc3` 1, `+0x42` 1, `+0xc5` 1, `+0x1c` and
/// `+0x20` −1, `+0x49` 7), then the round table of stage −1 (virtual +0x3c: push 0; push &`+0x3a`; push &`+0xa9`;
/// push −1 -> thiscall, ret 0x10) into `.round_dates()` (`create_main_schedule`). Returns 1.
/// Original: `KorLeague::set_up_rules` (0x66bcc0, kor_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_league_set_up_rules(this: usize) -> u32 {
    set_k_league_rules(this);
    covered!("mwf_11 k league rules");
    create_main_schedule(League(this));
    1
}

/// Comparer for `kor_league_set_up_rules`: side by side over the league object without the schedule outputs (+0x3a,
/// +0xa9, +0xba, which the virtual +0x3c allocates); the copy only writes the rule fields.
pub(crate) unsafe extern "thiscall" fn compare_kor_league_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_KOR_LEAGUE_SET_UP_RULES)(this), || {
        set_k_league_rules(this);
        1
    });
    SCOREBOARD.record(same && r == n, || format!("kor_league_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// Writes the Korean League Cup rule fields (what `kor_league_cup_set_up_rules` writes before the schedule).
pub(crate) unsafe fn set_korean_league_cup_rules(o: usize) {
    write_u8_at(o, 0xc7, 0);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 0);
    write_u16_at(o, 0x3c, 2); // a double round robin
    write_u8_at(o, 0xc6, 2);
    write_u8_at(o, 0xbe, 2); // the top two of each group go through
    write_u32_at(o, 0x1c, 0xffff_ffff);
    write_u32_at(o, 0x20, 0xffff_ffff);
    write_u8_at(o, 0xc2, 3); // mwf11: the League Cup points per win
    write_u8_at(o, 0xc3, 1); // points per draw
    write_u8_at(o, 0xc4, 4); // match category: cup
    write_u8_at(o, 0xc5, 1);
    write_u8_at(o, 0x42, 1); // club sides
    write_u16_at(o, 0xab, 0x83); // match rule bits
    write_u32_at(o, 0x28, 0x186a0); // prize fund 100,000
}

/// The Korean League Cup group rules: a double round robin, 3 points a win and 1 a draw, tiebreakers 1 and 2, the top
/// two of each group through, no league above or below, club sides, cup category (4), match rule bits 0x83 and a prize
/// fund of 100,000; then the group schedule. The substitutes and changes are not set here.
///
/// Writes the bytes and words of `set_korean_league_cup_rules` (`+0xc7` and `+0xbf`..`+0xc1` 0, `+0x3c` word 2,
/// `+0xc6` 2, `+0xbe` 2, `+0x1c` and `+0x20` −1, `+0xc2` 3, `+0xc3` 1, `+0xc4` 4, `+0xc5` 1, `+0x42` 1, `+0xab` word
/// 0x83, `+0x28` 100,000), then the round table of stage −1 (virtual +0x3c: push 0; push &`+0x3a`; push &`+0xa9`; push
/// −1 -> thiscall, ret 0x10) into `.round_dates()` (`create_main_schedule`). Returns 1.
/// Original: `KorLeagueCup::set_up_rules` (0x66c870, kor_league_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_league_cup_set_up_rules(this: usize) -> u32 {
    set_korean_league_cup_rules(this);
    covered!("mwf_11 korean league cup rules");
    create_main_schedule(League(this));
    1
}

/// Comparer for `kor_league_cup_set_up_rules`: side by side over the league object without the schedule outputs.
pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_KOR_LEAGUE_CUP_SET_UP_RULES)(this), || {
        set_korean_league_cup_rules(this);
        1
    });
    SCOREBOARD.record(same && r == n, || format!("kor_league_cup_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

// ---------- mwf_12: the NSL season honours and turnover (aus_nsl.cpp) ----------

/// The NSL honours: the grand final winner as champion, its runner-up second and the regular season leader third in
/// the competition history.
///
/// The grand final is stage 1 (`.stages()`[1], a Cup_Stage): in its team table (`cup::TEAM_TABLE`, 6-byte entries,
/// `.team_count()` of them, a signed word) the club with status (+5) 1 is the winner and the one with status 2 the
/// runner-up (the last of each, 0 when none). The history row (`exe::comp_util_add_history_record`: push 0; push
/// leader; push runner-up; push winner; push this; add esp, 0x14 -> cdecl) takes the winner, the runner-up, the club
/// in the first row of the league table (`.table()`) and 0; its eax is returned.
/// Original: `AusNsl::record_season_honours` (0x411d90, aus_nsl.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn aus_nsl_record_season_honours(this: usize) -> usize {
    let this = League(this);
    let fin = Cup(read_usize(this.stages() + 4));
    let (mut winner, mut runner_up) = (0usize, 0usize);
    let mut k = 0i16;
    while k < fin.team_count() {
        let e = fin.team_table() + k as usize * 6;
        let status = read_u8(e + 5);
        if status == 1 {
            winner = read_usize(e);
        }
        if status == 2 {
            runner_up = read_usize(e);
        }
        k += 1;
    }
    covered!("mwf_12 nsl honours");
    exe::comp_util_add_history_record(this, winner, runner_up, read_usize(this.table()), 0) // mwf12: the NSL champion, runner-up and leader
}

/// Comparer for `aus_nsl_record_season_honours`: side by side over the competition history (count, pointer and rows),
/// when there is a free row; when the history has to grow (realloc), only the original.
pub(crate) unsafe extern "thiscall" fn compare_aus_nsl_record_season_honours(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_AUS_NSL_RECORD_SEASON_HONOURS);
    if free_comp_history_row() == 0 {
        return o(this); // simplification: the history grows, original only
    }
    let reg = comp_history_regions();
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || aus_nsl_record_season_honours(this));
    SCOREBOARD.record(same && r == n, || format!("aus_nsl_record_season_honours({:x}) {:x}/{:x} {:x?} x {:x?}", this, r, n, mo, mr));
    r
}

/// The NSL season turnover: the season archived, the table and the dates cleared, the year plus one, then the rules,
/// the 13 Australian clubs, the fixtures and the order rebuilt; then the predicted positions, the board expectations
/// and the referee renewal. Unlike the sisters it neither destroys the finals stages nor resets
/// `league::CURRENT_STAGE` [inference: the NSL rules rebuild the stages].
///
/// `league::SEASON_CLOSED` = 0; the table freed (League::free_table `exe::league_free_table`, thiscall(this, 0), ret 4);
/// `.round_dates()` freed (`exe::crt_free`) and zeroed; with statistics, Comp_Stats::archive_season_and_reset
/// (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the year up by one. Then, each with its own
/// box (aus_nsl.cpp, lines 0x2bd, 0x2c4, 0x2ca and 0x2d1) when it returns 0: the rules (virtual
/// `comp_vtable::SET_UP_RULES`), the table (AusNsl::create_table 0x411b70, thiscall, ret 0: the 13 clubs of Australia),
/// the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table
/// `exe::league_sort_table`, thiscall(this, 0), ret 4); then the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`) and `set_expectations_and_renew_officials`, whose eax (the referee
/// renewal's) it returns. After a box the copy returns 0 (the original returns the box's eax; the turnover's caller
/// does not read it).
/// Original: `AusNsl::start_new_season` (0x411de0, aus_nsl.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn aus_nsl_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    exe::league_free_table(this, 0);
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
    this.set_year(a.wrapping_add(1)); // mwf12: the year of the NSL
    covered!("mwf_12 nsl rollover");
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(0x987c20, 0x2bd);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x411b70usize);
    if table(this.addr()) == 0 {
        show_error_at(0x987c20, 0x2c4);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(0x987c20, 0x2ca);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(0x987c20, 0x2d1);
        return 0;
    }
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// Comparer for `aus_nsl_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go
/// up by one.
pub(crate) unsafe extern "thiscall" fn compare_aus_nsl_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_AUS_NSL_START_NEW_SEASON);
    compare_state_league_new_season("aus_nsl_start_new_season", League(this), || o(this))
}

// ---------- mwf_13: the Auto Windscreens Shield season turnover and rounds (eng_auto_cup.cpp) ----------

/// The season turnover of the Auto Windscreens Shield (the Football League Trophy): last season's tables, selected
/// clubs and section stages dropped, the season archived, the year plus one, then the 56 clubs selected again, the
/// entrants, the round table, the tie tree and the first section stage rebuilt; the selected list is freed again and
/// the predicted positions close it.
///
/// `cup::SEASON_CLOSED` = 0; the team table (`cup::TEAM_TABLE`), the round table (`cup::ROUND_TABLE`) and the selected
/// clubs (`eng_auto_cup::SELECTED_CLUBS`) freed (`exe::crt_free`) and zeroed; when there are tie tables
/// (`cup::TIE_TABLES`), each round's (`cup::ROUND_COUNT` rounds, a signed word) freed and zeroed and the array freed and
/// zeroed; with a current stage (`cup::CURRENT_STAGE`) ≥ 0, the stages 0..=current destroyed (virtual 0 with 1) and
/// then `cup::CURRENT_STAGE` = −1; with statistics, Comp_Stats::archive_season_and_reset
/// (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the year up by one and
/// `cup::CURRENT_ROUND` = 0.
///
/// Then, each with its own box (eng_auto_cup.cpp): the 56 section clubs (EngAutoCup::select_section_clubs 0x56b730,
/// thiscall, ret 0; box 0x1ee when it returns 0); `cup::RUN_BY_STAGES` = 0; the entrants (EngAutoCup::collect_entrants
/// 0x56b3c0, box 0x1f8 when negative); the round table (virtual `comp_vtable::SET_UP_RULES`, box 0x1ff when negative)
/// and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`, box 0x206 when negative). Then the first section stage
/// (EngAutoCup::create_section_stage 0x56bab0, thiscall, ret 0), the selected clubs freed and zeroed again, and the
/// predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax it returns. After a box the copy
/// returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `EngAutoCup::start_new_season` (0x56b4b0, eng_auto_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_auto_cup_start_new_season(this: usize) -> u32 {
    let this = Cup(this);
    this.set_season_closed(0);
    for o in [cup::TEAM_TABLE, cup::ROUND_TABLE, eng_auto_cup::SELECTED_CLUBS] {
        let l = read_usize(this.addr() + o);
        if l != 0 {
            exe::crt_free(l);
            write_usize(this.addr() + o, 0);
        }
    }
    if this.tie_tables() != 0 {
        let mut i = 0i32;
        while i < this.round_count() as i32 {
            let p = read_usize(this.tie_tables() + i as usize * 4);
            if p != 0 {
                exe::crt_free(p);
                write_usize(this.tie_tables() + i as usize * 4, 0);
            }
            i += 1;
        }
        exe::crt_free(this.tie_tables());
        this.set_tie_tables(0);
    }
    if read_i32(this.addr() + cup::CURRENT_STAGE) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(read_usize(this.addr() + cup::STAGES) + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > read_i32(this.addr() + cup::CURRENT_STAGE) {
                break;
            }
        }
    }
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    let stats = read_usize(this.addr() + 8);
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    let a = this.year();
    this.set_year(a.wrapping_add(1)); // mwf13: the year of the Auto Windscreens Shield
    this.set_current_round(0);
    covered!("mwf_13 auto windscreens rollover");
    let select: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x56b730usize);
    if select(this.addr()) == 0 {
        show_error_at(0x9c4180, 0x1ee);
        return 0;
    }
    this.set_run_by_stages(0);
    let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x56b3c0usize);
    if entrants(this.addr()) < 0 {
        show_error_at(0x9c4180, 0x1f8);
        return 0;
    }
    if call_cup_virtual(this, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(0x9c4180, 0x1ff);
        return 0;
    }
    if call_cup_virtual(this, cup_vtable::MAP_FIXTURE_TREE) < 0 {
        show_error_at(0x9c4180, 0x206);
        return 0;
    }
    let section: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x56bab0usize);
    section(this.addr());
    let l = read_usize(this.addr() + eng_auto_cup::SELECTED_CLUBS);
    if l != 0 {
        exe::crt_free(l);
        write_usize(this.addr() + eng_auto_cup::SELECTED_CLUBS, 0);
    }
    call_cup_virtual(this, comp_vtable::SET_PREDICTED_POSITIONS) as u32
}

/// Comparer for `eng_auto_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must
/// go up by one.
pub(crate) unsafe extern "thiscall" fn compare_eng_auto_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_AUTO_CUP_START_NEW_SEASON);
    compare_cup_new_season("eng_auto_cup_start_new_season", Cup(this), || o(this))
}

/// The Auto Windscreens Shield rounds: a prize fund of 400,000 and the round table.
///
/// `cup::PRIZE_FUND` = 400,000 (0x61a80); the round table of stage −1 (virtual `comp_vtable::CREATE_SCHEDULE`: push 0;
/// push &`cup::STAGE_LABEL`; push &`cup::ROUND_COUNT`; push −1 -> thiscall, ret 0x10) into `cup::ROUND_TABLE`. The
/// substitutes and changes are not set here. Returns 1.
/// Original: `EngAutoCup::set_up_rounds` (0x56b700, eng_auto_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_auto_cup_set_up_rounds(this: usize) -> i32 {
    let this = Cup(this);
    covered!("mwf_13 auto windscreens rounds");
    this.set_prize_fund(0x61a80); // mwf13: the Auto Windscreens Shield prize fund
    let table: unsafe extern "thiscall" fn(usize, u32, usize, usize, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = table(this.addr(), 0xffff_ffff, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0);
    this.set_round_table(t);
    1
}

/// Comparer for `eng_auto_cup_set_up_rounds`: side by side (as the Copa do Brasil's): the prize fund, the round count
/// and the stage label, and the bytes the round table writes (`eng_cc_cup_written_round_bytes`); the copy's table is
/// freed and the original's kept.
pub(crate) unsafe extern "thiscall" fn compare_eng_auto_cup_set_up_rounds(this: usize) -> i32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_ENG_AUTO_CUP_SET_UP_ROUNDS);
    let to = core::cell::Cell::new(0usize);
    let tr = core::cell::Cell::new(0usize);
    let before = this.round_table();
    let reg = [(this.addr() + cup::PRIZE_FUND, 4usize), (this.addr() + cup::STAGE_LABEL, 4), (G_MOUSE_LEFT_BUTTON_DOWN, 4)];
    let (a, b, same, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this.addr());
        to.set(this.round_table());
        this.set_round_table(before);
        r
    }, || {
        let r = eng_auto_cup_set_up_rounds(this.addr());
        tr.set(this.round_table());
        this.set_round_table(to.get());
        r
    });
    let (to, tr) = (to.get(), tr.get());
    let n = this.round_count();
    let table = if to != 0 && tr != 0 { eng_cc_cup_written_round_bytes(to, n) == eng_cc_cup_written_round_bytes(tr, n) } else { to == 0 && tr == 0 };
    if tr != 0 && tr != to {
        exe::crt_free(tr);
    }
    SCOREBOARD.record(same && a == b && table, || format!("eng_auto_cup_set_up_rounds({:#x}) returned {} x {} memory {} table {}", this.addr(), a, b, same, table));
    a
}
