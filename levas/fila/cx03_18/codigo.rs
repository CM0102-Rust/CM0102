// ---------- cx03_18: the season turnover of the two Asian club cups (asia_club_champ.cpp, asia_cup_winner.cpp) ----------

/// AsiaClubChamp::select_qualified_clubs (thiscall, ret 0; 0 = failure).
const FN_ASIACLUBCHAMP_SELECT_QUALIFIED_CLUBS: usize = 0x40b950;
/// AsiaClubChamp::build_team_table (thiscall, ret 0; negative = failure).
const FN_ASIACLUBCHAMP_BUILD_TEAM_TABLE: usize = 0x40b620;
/// AsiaClubChamp::create_second_zone_cup_stage (thiscall, ret 0).
const FN_ASIACLUBCHAMP_CREATE_SECOND_ZONE_CUP_STAGE: usize = 0x40bec0;
/// AsiaCupWinner::select_qualified_clubs (thiscall, ret 0; 0 = failure).
const FN_ASIACUPWINNER_SELECT_QUALIFIED_CLUBS: usize = 0x40d830;
/// AsiaCupWinner::create_team_entries (thiscall, ret 0; negative = failure).
const FN_ASIACUPWINNER_CREATE_TEAM_ENTRIES: usize = 0x40d4d0;
/// AsiaCupWinner::create_zone_stage (thiscall, ret 0).
const FN_ASIACUPWINNER_CREATE_ZONE_STAGE: usize = 0x40ddf0;

/// The routines and the boxes that make one Asian club cup's turnover: the club selection, the team entries, the first
/// stage, the source file and the box lines of the four failures.
pub(crate) struct AsianClubCupTurnover {
    pub(crate) select: usize,
    pub(crate) entries: usize,
    pub(crate) first_stage: usize,
    pub(crate) file: usize,
    pub(crate) rows: [u32; 4],
}

/// The Asian Club Championship turnover: lines 0x21c, 0x226, 0x22d and 0x234 of asia_club_champ.cpp.
pub(crate) const ASIAN_CLUB_CHAMPIONSHIP_TURNOVER: AsianClubCupTurnover = AsianClubCupTurnover {
    select: FN_ASIACLUBCHAMP_SELECT_QUALIFIED_CLUBS, entries: FN_ASIACLUBCHAMP_BUILD_TEAM_TABLE,
    first_stage: FN_ASIACLUBCHAMP_CREATE_SECOND_ZONE_CUP_STAGE, file: 0x987a40, rows: [0x21c, 0x226, 0x22d, 0x234],
};

/// The Asian Cup Winners' Cup turnover: lines 0x1d4, 0x1de, 0x1e5 and 0x1ec of asia_cup_winner.cpp.
pub(crate) const ASIAN_CUP_WINNERS_CUP_TURNOVER: AsianClubCupTurnover = AsianClubCupTurnover {
    select: FN_ASIACUPWINNER_SELECT_QUALIFIED_CLUBS, entries: FN_ASIACUPWINNER_CREATE_TEAM_ENTRIES,
    first_stage: FN_ASIACUPWINNER_CREATE_ZONE_STAGE, file: 0x987b60, rows: [0x1d4, 0x1de, 0x1e5, 0x1ec],
};

/// The season turnover of the two Asian club cups (the same body in both, checked instruction by instruction: only the
/// routines, the source file and the box lines change, `AsianClubCupTurnover`).
///
/// `cup::SEASON_CLOSED` = 0; the team table, the round table and the club list +0xb2 freed and cleared; with the tie
/// tables, each round's (`cup::ROUND_COUNT`, a signed word) freed and cleared, then the array freed and cleared; each
/// stage 0..=`cup::CURRENT_STAGE` destroyed (virtual 0 with 1; the slots and the stage vector are left as they are) and
/// `cup::CURRENT_STAGE` = −1; the season archived in the statistics (`exe::comp_stats_archive_season_and_reset`(stats,
/// 1), when there are statistics); the year up by one and `cup::CURRENT_ROUND` = 0. Then, each thiscall, ret 0: the
/// clubs (`select`; 0: the box `rows[0]` and 0), `cup::RUN_BY_STAGES` = 0, the team entries (`entries`; negative: the
/// box `rows[1]` and 0), the rules (virtual `comp_vtable::SET_UP_RULES`; negative: `rows[2]` and 0), the tie tree
/// (virtual `cup_vtable::MAP_FIXTURE_TREE`; negative: `rows[3]` and 0) and the first stage (`first_stage`); the club
/// list freed and cleared again; the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax it
/// returns. In the boxes the original returns what the box left in eax; the copy returns 0.
pub(crate) unsafe fn asian_club_cup_new_season(this: Cup, t: &AsianClubCupTurnover, branch: &'static str) -> u32 {
    let club_list = this.addr() + 0xb2; // the qualified club list
    this.set_season_closed(0);
    if this.team_table() != 0 {
        exe::crt_free(this.team_table());
        this.set_team_table(0);
    }
    if this.round_table() != 0 {
        exe::crt_free(this.round_table());
        this.set_round_table(0);
    }
    if read_usize(club_list) != 0 {
        exe::crt_free(read_usize(club_list));
        write_usize(club_list, 0);
    }
    if this.tie_tables() != 0 {
        let mut r = 0i32;
        while r < this.round_count() as i32 {
            let slot = this.tie_tables() + r as usize * 4;
            if read_usize(slot) != 0 {
                exe::crt_free(read_usize(slot));
                write_usize(slot, 0);
            }
            r += 1;
        }
        exe::crt_free(this.tie_tables());
        this.set_tie_tables(0);
    }
    let mut k = 0i32;
    while k <= this.current_stage() as i32 {
        let stage = read_usize(this.stages() + k as usize * 4);
        if stage != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(stage)));
            destroy(stage, 1);
        }
        k += 1;
    }
    this.set_current_stage(0xffff_ffff);
    let stats = League(this.addr()).comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    this.set_year(this.year().wrapping_add(1)); // the next edition
    this.set_current_round(0);
    let call = |f: usize| -> i32 {
        let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(f);
        f(this.addr())
    };
    if call(t.select) == 0 {
        show_error_at(t.file, t.rows[0]);
        return 0;
    }
    this.set_run_by_stages(0);
    if call(t.entries) < 0 {
        show_error_at(t.file, t.rows[1]);
        return 0;
    }
    if call_cup_virtual(this, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(t.file, t.rows[2]);
        return 0;
    }
    if call_cup_virtual(this, cup_vtable::MAP_FIXTURE_TREE) < 0 {
        show_error_at(t.file, t.rows[3]);
        return 0;
    }
    covered!(branch);
    call(t.first_stage);
    if read_usize(club_list) != 0 {
        exe::crt_free(read_usize(club_list));
        write_usize(club_list, 0);
    }
    call_cup_virtual(this, comp_vtable::SET_PREDICTED_POSITIONS) as u32
}

/// rule: at the turnover the Asian Club Championship is cleared (teams, rounds, ties, the qualified clubs and its
/// stages), its statistics archived and the year moves on; then the qualified clubs, the team entries, the rules, the
/// tie tree, the second zone's cup stage and the predicted positions again (`asian_club_cup_new_season`).
/// Original: `AsiaClubChamp::start_new_season` (0x40b700, asia_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_club_champ_start_new_season(this: usize) -> u32 {
    asian_club_cup_new_season(Cup(this), &ASIAN_CLUB_CHAMPIONSHIP_TURNOVER, "cx03_18 asian club championship turnover")
}

/// rule: the Asian Cup Winners' Cup turnover, as the Asian Club Championship's, with its own clubs, team entries and zone
/// stage (`asian_club_cup_new_season`).
/// Original: `AsiaCupWinner::start_new_season` (0x40d5b0, asia_cup_winner.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_cup_winner_start_new_season(this: usize) -> u32 {
    asian_club_cup_new_season(Cup(this), &ASIAN_CUP_WINNERS_CUP_TURNOVER, "cx03_18 asian cup winners turnover")
}

/// Comparer for `asia_club_champ_start_new_season`: only the original runs (it frees, allocates and draws); the plan:
/// the year up by one (`compare_cup_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_asia_club_champ_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_cup_new_season("asia_club_champ_start_new_season", Cup(this), || o(this))
}

/// Comparer for `asia_cup_winner_start_new_season` (as above).
pub(crate) unsafe extern "thiscall" fn compare_asia_cup_winner_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_cup_new_season("asia_cup_winner_start_new_season", Cup(this), || o(this))
}
