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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_cup_new_season("spa_cup_start_new_season", Cup(this), || o(this))
}
