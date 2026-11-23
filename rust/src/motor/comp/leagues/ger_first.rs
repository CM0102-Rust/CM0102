use super::*;

/// Rule: the German turnover, led by the Bundesliga: the season reopened; with the Regionalliga loaded, its own step
/// first; promotion and relegation along the chain; the clubs exchanged between the 2. Bundesliga and the Regionalliga
/// (with it loaded by GerFirst's 0x5db980, without it by `ger_first_exchange_regional_clubs_unplayed`); the
/// Regionalliga clubs split north and south; the Bundesliga rebuilt for the next year (rules, table, fixtures, order);
/// the division tier of the 2. Bundesliga clubs capped and the 2. Bundesliga (and the Regionalliga, when loaded) turned
/// over; the predicted positions, the board expectations and the officials.
///
/// `top_division_turnover` with, before: 0x5dbf50 (thiscall, ret 0, not ported) when the object of
/// `KEY_CLUB_COMP_GERMAN_REGIONAL` exists; virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(1)
/// (`ger_first_process_promotion_relegation_chain`); 0x5db980 (not ported) or `ger_first_exchange_regional_clubs_unplayed`
/// by that object again; `ger_first_split_regional_north_south` (0x5dbbd0, not ported; called with ecx = this, it does
/// not read it); the table GerFirst::create_table (0x5db230) and the boxes of ger_first.cpp (0x9cd414) at 0x2ec, 0x2f3,
/// 0x2f9, 0x300; after: League::limit_club_division_tier(1, 3, −3, 1) and the start_new_season (virtual
/// `comp_vtable::START_NEW_SEASON`, no null check) of the 2. Bundesliga, then the Regionalliga's when it exists. After a
/// box: 0.
/// Original: `GerFirst::start_new_season` (0x5db4a0, ger_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    top_division_turnover(this, || {
        if comp_by_global(KEY_CLUB_COMP_GERMAN_REGIONAL) != 0 {
            call_on_league(0x5dbf50, this);
        }
        promotion_relegation_chain(this, 1);
        if comp_by_global(KEY_CLUB_COMP_GERMAN_REGIONAL) != 0 {
            call_on_league(0x5db980, this);
        } else {
            call_on_league(0x5db7b0, this);
        }
        call_on_league(0x5dbbd0, this);
    }, 0x5db230, 0x9cd414, [0x2ec, 0x2f3, 0x2f9, 0x300], || {
        covered!("mwc_21 german bundesliga turnover");
        exe::league_limit_club_division_tier(comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION), 1, 3, -3, 1);
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION), comp_vtable::START_NEW_SEASON);
        start_new_season_of(KEY_CLUB_COMP_GERMAN_REGIONAL);
    })
}

/// Rule: German promotion and relegation: the final positions of the Bundesliga and the 2. Bundesliga recorded and
/// their marked clubs swapped; with the Regionalliga loaded, its final positions and its first group's recorded and
/// the marked clubs swapped between the 2. Bundesliga and the Regionalliga, and between the 2. Bundesliga and that
/// group.
///
/// Virtual `league_vtable::RECORD_FINAL_POSITIONS` on the objects of `KEY_CLUB_COMP_GERMAN_FIRST_DIVISION` and
/// `KEY_CLUB_COMP_GERMAN_SECOND_DIVISION`; League::promote_and_relegate (`exe::league_promote_and_relegate`(this,
/// above, below, 1, 1, −1, −1): 6 pushes -> thiscall, ret 0x18) Bundesliga / 2. Bundesliga; when the object of
/// `KEY_CLUB_COMP_GERMAN_REGIONAL` exists: RECORD_FINAL_POSITIONS on it and on its stage 0 (`league::STAGES`[0]), then
/// promote_and_relegate 2. Bundesliga / Regionalliga and 2. Bundesliga / stage 0. Returns the last call's eax (without
/// the Regionalliga, the value of `G_COMP_OBJECT_TABLE` the original leaves in eax); the argument is not read.
/// Original: `GerFirst::process_promotion_relegation_chain` (0x5db6c0, ger_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ger_first_process_promotion_relegation_chain(this: usize, _pairs: u32) -> u32 {
    bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_GERMAN_FIRST_DIVISION), league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION), league_vtable::RECORD_FINAL_POSITIONS);
    covered!("mwc_21 german promotion and relegation");
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_GERMAN_FIRST_DIVISION), comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION), 1, 1, -1, -1); // mwc21: the Bundesliga and the 2. Bundesliga
    let regional = comp_by_global(KEY_CLUB_COMP_GERMAN_REGIONAL);
    if regional == 0 {
        return read_usize(G_COMP_OBJECT_TABLE) as u32;
    }
    bra_nat_first_call_virtual_no_args(regional, league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(read_usize(read_usize(comp_by_global(KEY_CLUB_COMP_GERMAN_REGIONAL) + league::STAGES)), league_vtable::RECORD_FINAL_POSITIONS);
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION), comp_by_global(KEY_CLUB_COMP_GERMAN_REGIONAL), 1, 1, -1, -1);
    let stage = read_usize(read_usize(comp_by_global(KEY_CLUB_COMP_GERMAN_REGIONAL) + league::STAGES));
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION), stage, 1, 1, -1, -1)
}

/// Rule: at the German turnover without the Regionalliga loaded, the four best clubs of the Regionalliga division
/// (sorted by strength with the best twelve shuffled; B teams left out) go up to the 2. Bundesliga, and its relegated
/// clubs (status 3) go down to the Regionalliga division; with no candidate nobody moves.
///
/// A list of `G_CLUB_COUNT` pointers (`exe::crt_malloc`; out of memory: the box of ger_first.cpp at line 0x354 and
/// return) with the clubs in database order whose division (`club::DIVISION`) is `KEY_CLUB_COMP_GERMAN_REGIONAL`'s,
/// with a nation (`club::NATION` not null) and not a B team (`exe::club_get_linked_b_team`(club, &flag, 1), cdecl: a
/// partner with the flag 0 leaves the club out; the flag is one local shared by every call, as the original's stack
/// slot) (a 16-bit index). With at least one: `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 4) and the first four
/// moved to the 2. Bundesliga (League::move_club_to_division(this, club, its record, 1)); with fewer than four the
/// original moves the list's unwritten slots as well (a defect of the original, kept); then each row i of the 2.
/// Bundesliga (its club count read again on each step) with status 3 goes down (League::get_table_row, called twice,
/// and League::relegate_club_to_division(this, club, the Regionalliga's record, 1)). The list is freed.
/// Original: `ger_first_exchange_regional_clubs_unplayed` (0x5db7b0, ger_first.cpp, thiscall: ecx = the Bundesliga, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_first_exchange_regional_clubs_unplayed(this: usize) {
    let second = comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION);
    let list = exe::crt_malloc((read_usize(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0x9cd414, 0x354);
        return;
    }
    let mut flag: u32 = 0;
    let mut n: u32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(club + club::NATION) != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GERMAN_REGIONAL) {
            if exe::club_get_linked_b_team(club, &mut flag, 1) == 0 || flag != 0 {
                write_usize(list + (n as u16 as i16 as isize as usize).wrapping_mul(4), club);
                n = n.wrapping_add(1);
            }
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    if n as u16 != 0 {
        covered!("mwc_21 german regional division up");
        exe::comp_util_sort_and_shuffle_top_clubs(list, n, 4); // mwc21: the best twelve shuffled
        for k in 0..4 {
            exe::league_move_club_to_division(this, read_usize(list + 4 * k), comp_record_of(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION), 1);
        }
        let mut i = 0i32;
        while i < League(second).club_count() as i32 {
            if read_u8(exe::league_get_table_row(second, i) + league_table_row::STATUS) == 3 {
                let record = comp_record_of(KEY_CLUB_COMP_GERMAN_REGIONAL);
                let club = read_usize(exe::league_get_table_row(second, i));
                exe::league_relegate_club_to_division(this, club, record, 1);
            }
            i += 1;
        }
    }
    exe::crt_free(list);
}

/// Comparer for `ger_first_start_new_season`: only the original runs (it draws and rebuilds); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_ger_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GER_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("ger_first_start_new_season", League(this), || o(this))
}

/// Comparer for `ger_first_process_promotion_relegation_chain`: only the original runs (it moves clubs between the
/// divisions; as the Turkish chain's, `tur_first_process_promotion_relegation_chain`).
pub(crate) unsafe extern "thiscall" fn compare_ger_first_process_promotion_relegation_chain(this: usize, pairs: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_GER_FIRST_PROCESS_PROMOTION_RELEGATION_CHAIN)(this, pairs)
}

/// Comparer for `ger_first_exchange_regional_clubs_unplayed`: only the original runs (it draws); with at least 4
/// candidates, the 2. Bundesliga's clubs of status 3 must end in the Regionalliga division.
pub(crate) unsafe extern "thiscall" fn compare_ger_first_exchange_regional_clubs_unplayed(this: usize) {
    let mut flag: u32 = 0;
    let mut candidates = 0;
    for i in 0..read_usize(G_CLUB_COUNT) {
        let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        let d = read_usize(c + club::DIVISION);
        if d != 0 && read_usize(c + club::NATION) != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GERMAN_REGIONAL) && (exe::club_get_linked_b_team(c, &mut flag, 1) == 0 || flag != 0) {
            candidates += 1;
        }
    }
    let l = League(comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION));
    let mut planned: Vec<usize> = Vec::new();
    if candidates >= 4 && l.table() != 0 {
        for i in 0..l.club_count() as usize {
            let row = l.table() + i * league_table_row::SIZE;
            if read_u8(row + league_table_row::STATUS) == 3 {
                planned.push(read_usize(row));
            }
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_GER_FIRST_EXCHANGE_REGIONAL_CLUBS_UNPLAYED)(this);
    let lower = comp_record_of(KEY_CLUB_COMP_GERMAN_REGIONAL);
    let wrong: Vec<usize> = planned.iter().copied().filter(|&c| read_usize(c + club::DIVISION) != lower).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("ger_first_exchange_regional_clubs_unplayed({:#x}): relegated clubs not in the Regionalliga division {:x?}", this, wrong));
}

/// The Bundesliga: discipline rule 13 (`GERMANY_LEAGUE`), only the league; with no saved state, the rules, the table,
/// the fixtures, the order, the statistics and the predicted positions (`construct_nation_division`).
/// Original: `GerFirst::GerFirst` (0x5d99b0, ger_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ger_first(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &GER_FIRST_DIVISION, || {})
}

/// Comparer for `construct_ger_first`: only the original runs (`compare_nation_division_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_ger_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_GER_FIRST);
    compare_nation_division_ctor("construct_ger_first", League(this), year, record, &GER_FIRST_DIVISION, || o(this, year, record))
}

/// Sets up the rules of the Bundesliga: 2 rounds, 3 points for a win and 1 for a draw; 0 go up, 0 to the promotion
/// play-off, 0 to the relegation play-off and 3 go down; above: none; below: `KEY_CLUB_COMP_GERMAN_SECOND_DIVISION`;
/// tie-breaker codes 1, 2, 3 (`+0xc5`..); 6 substitutes named and 3 substitutions; then builds the main-stage round
/// table through virtual +0x3c into `league::ROUND_DATES` (`fra_first_apply_league_rules`). Returns 1.
/// Original: `GerFirst::set_up_rules` (0x5db400, ger_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_first_set_up_rules(this: usize) -> u32 {
    covered!("mwc_11 ger first rules");
    fra_first_apply_league_rules(League(this), &ger_first_rules(this))
}

/// Comparer for `ger_first_set_up_rules`: a dry run (`compare_nation_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_ger_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GER_FIRST_SET_UP_RULES);
    compare_nation_league_rules("ger_first_set_up_rules", League(this), ger_first_rules(this), || o(this), || ger_first_set_up_rules(this))
}

/// The Bundesliga schedule: 34 rounds, from 29 Jul to 5 May; 30 with matches in TV slots; in 2001/02 its own table, in
/// the following seasons another.
///
/// `GerFirst::create_schedule(stage, &rounds, &extra, &is_league)` (`create_league_schedule`): only the main stage (-1;
/// the others return 0 without writing); is_league = 1, rounds = 34, extra = 0 and the table of 34 x 0x41 bytes
/// (`exe::crt_malloc`; out of memory, the error box of line 0xa1 and 0) with each round's date and TV slots
/// (`league_round_set_date`, `league_round_set_match_slot`, cdecl).
/// Original: `GerFirst::create_schedule` (0x5d9c70, ger_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ger_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let year = League(this).year();
    let p = if year as u16 == 0x7d1 { &GER_FIRST_2001 } else { &GER_FIRST_LATER };
    create_league_schedule(League(this), stage, rounds, extra, league, p)
}

/// Comparer for `ger_first_create_schedule`: side by side, the copy's table freed (`compare_cfa_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_ger_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_GER_FIRST_CREATE_SCHEDULE);
    compare_cfa_schedule("ger_first_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| ger_first_create_schedule(this, stage, r, e, l))
}

/// Calls the start_new_season virtual (`comp_vtable::START_NEW_SEASON`, mov ecx -> thiscall, ret 0) of the
/// competition object of key `g` (`G_COMP_OBJECT_TABLE` by its id), when there is one.
pub(crate) unsafe fn start_new_season_of(g: usize) {
    let c = comp_by_global(g);
    if c != 0 {
        bra_nat_first_call_virtual_no_args(c, comp_vtable::START_NEW_SEASON);
    }
}

/// A direct call with no arguments on the league (thiscall, ret 0).
pub(crate) unsafe fn call_on_league(f: usize, this: League) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
    f(this.addr());
}

/// The rule writes of the Bundesliga (offset, size, value).
pub(crate) unsafe fn ger_first_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let _ = this;
    let mut v = vec![(0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 3), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 3), (0x1c, 4, 0xffff_ffff), (0x4a, 1, 3), (0x52, 1, 2), (0x20, 4, read_usize(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION) as u32), (0x49, 1, 6)];
    v.push((0xc2, 1, 3)); // mwc11: the Bundesliga points for a win
    v
}

pub(crate) const GER_FIRST_DIVISION: NationDivision = NationDivision {
    vtable: 0x96b0e4,
    bench: None,
    discipline: discipline_rule::GERMANY_LEAGUE, // mwc11: the Bundesliga discipline rule
    stages: 0,
    set_up_rules: 0x5db400,
    create_table: 0x5db230,
    source_file: 0x9cd414,
    box_lines: [0x47, 0x4e, 0x55, 0x5e, 0x65],
};

pub(crate) const GER_FIRST_2001: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 34, file: 0x9cd414, row: 0xa1, table: &GER_FIRST_2001_ROUNDS, branch: "mwc_11 ger first schedule" };

pub(crate) const GER_FIRST_LATER: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 34, file: 0x9cd414, row: 0xa1, table: &GER_FIRST_LATER_ROUNDS, branch: "mwc_11 ger first schedule" };

/// The Bundesliga 2001/02: 34 rounds (the first date with the literal year 2001).
pub(crate) const GER_FIRST_2001_ROUNDS: [FraThirdLeagueRound; 34] = [
    (29, 6, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 0: 29 Jul
    (5, 7, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 1: 5 Aug
    (12, 7, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 2: 12 Aug
    (19, 7, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 3: 19 Aug
    (6, 8, 0, 2, 2, &[(-1, -1, -1)]), // 4: 6 Sep
    (9, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 5: 9 Sep (mwc11 ger_first)
    (16, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 6: 16 Sep
    (23, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 7: 23 Sep
    (30, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 8: 30 Sep
    (14, 9, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 9: 14 Oct
    (21, 9, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 10: 21 Oct
    (28, 9, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 11: 28 Oct
    (4, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 12: 4 Nov
    (11, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 13: 11 Nov
    (18, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 14: 18 Nov
    (25, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 15: 25 Nov
    (2, 11, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 16: 2 Dec
    (9, 11, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 17: 9 Dec
    (13, 11, 0, 2, 2, &[(-1, -1, -1)]), // 18: 13 Dec
    (16, 11, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 19: 16 Dec
    (27, 0, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 20: 27 Jan (+1 year)
    (3, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 21: 3 Feb (+1 year)
    (10, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 22: 10 Feb (+1 year)
    (17, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 23: 17 Feb (+1 year)
    (24, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 24: 24 Feb (+1 year)
    (3, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 25: 3 Mar (+1 year)
    (10, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 26: 10 Mar (+1 year)
    (17, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 27: 17 Mar (+1 year)
    (31, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 28: 31 Mar (+1 year)
    (7, 3, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 29: 7 Apr (+1 year)
    (14, 3, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 30: 14 Apr (+1 year)
    (21, 3, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 31: 21 Apr (+1 year)
    (28, 3, 1, 5, 1, &[(-1, -1, -1)]), // 32: 28 Apr (+1 year)
    (5, 4, 1, 5, 1, &[(-1, -1, -1)]), // 33: 5 May (+1 year)
];

/// The Bundesliga from 2002/03: 34 rounds.
pub(crate) const GER_FIRST_LATER_ROUNDS: [FraThirdLeagueRound; 34] = [
    (12, 7, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 0: 12 Aug
    (19, 7, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 1: 19 Aug
    (6, 8, 0, 2, 2, &[(-1, -1, -1)]), // 2: 6 Sep
    (9, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 3: 9 Sep
    (16, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 4: 16 Sep
    (23, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 5: 23 Sep
    (30, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 6: 30 Sep
    (14, 9, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 7: 14 Oct
    (21, 9, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 8: 21 Oct
    (28, 9, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 9: 28 Oct
    (4, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 10: 4 Nov
    (11, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 11: 11 Nov
    (18, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 12: 18 Nov
    (25, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 13: 25 Nov
    (2, 11, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 14: 2 Dec
    (9, 11, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 15: 9 Dec
    (13, 11, 0, 2, 2, &[(-1, -1, -1)]), // 16: 13 Dec
    (16, 11, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 17: 16 Dec
    (27, 0, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 18: 27 Jan (+1 year)
    (3, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 19: 3 Feb (+1 year)
    (10, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 20: 10 Feb (+1 year)
    (17, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 21: 17 Feb (+1 year)
    (24, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 22: 24 Feb (+1 year)
    (3, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 23: 3 Mar (+1 year)
    (10, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 24: 10 Mar (+1 year)
    (17, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 25: 17 Mar (+1 year)
    (31, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 26: 31 Mar (+1 year)
    (7, 3, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 27: 7 Apr (+1 year)
    (14, 3, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 28: 14 Apr (+1 year)
    (21, 3, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 29: 21 Apr (+1 year)
    (28, 3, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 30: 28 Apr (+1 year)
    (5, 4, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 31: 5 May (+1 year)
    (12, 4, 1, 5, 1, &[(-1, -1, -1)]), // 32: 12 May (+1 year)
    (19, 4, 1, 5, 1, &[(-1, -1, -1)]), // 33: 19 May (+1 year)
];
