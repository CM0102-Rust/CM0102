use super::*;

/// Rule: the German Cup takes 64 clubs: up to 4 amateur clubs relegated from the 2. Bundesliga last season (class 2),
/// then the other amateur clubs (sorted by strength, the best 72 shuffled) until 28 are in (class 2), then the 2.
/// Bundesliga clubs (class 2 for those promoted from below, 1 for the others) and the Bundesliga's (class 1) until the
/// list is full; returns 1 when it is full. The amateur clubs: those of the Regionalliga division with a reserve
/// division, and the German clubs of the lowest division; B teams left out.
///
/// `team_count` = 64; the list (`exe::crt_malloc` of 0x180; out of memory: the box of ger_cup.cpp at line 0x13c and
/// −1); a local list of 700 pointers (`exe::crt_malloc` of 0xaf0; out of memory: the box at line 0x144 and 0, the team
/// list left allocated). Over the first [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`] clubs (a 16-bit index, the limit read
/// again each step) with a division: an amateur club is one with `club::RESERVE_DIVISION` set and the division id of
/// `KEY_CLUB_COMP_GERMAN_REGIONAL`, or else with a nation whose id is Germany's (`KEY_NATION_GERMANY`) and the division
/// id of `KEY_CLUB_COMP_A_LOWER_DIVISION`; B teams (`exe::club_get_linked_b_team`(club, &flag, 1), cdecl: a partner
/// with the flag 0; one flag shared by every call) are left out. An amateur club whose `club::LAST_DIVISION` has the id
/// of `KEY_CLUB_COMP_GERMAN_SECOND_DIVISION` goes straight in (class 2) while fewer than 4 are in; the others go to the
/// local list (at most 700). `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 24); then, while the count is below
/// 28, the list's clubs in order (class 2), each slot zeroed after it is copied; and a second pass over the list from
/// its start while still below 28 (it copies the zeroed slots: with too few amateur clubs the original fills the rest
/// with null clubs; a defect of the original, kept). The local list is freed. Then a pass over the clubs (16-bit index
/// up to `G_CLUB_COUNT`) for the 2. Bundesliga (division id of `KEY_CLUB_COMP_GERMAN_SECOND_DIVISION`) while below
/// `team_count`: class 2 when the last division is set and is neither the 2. Bundesliga nor the Bundesliga, else 1;
/// then the Bundesliga (`KEY_CLUB_COMP_GERMAN_FIRST_DIVISION`, class 1); both stop when the count reaches
/// `team_count`. Returns count == `team_count`.
/// Original: `GerCup::collect_entrants` (0x5d9370, ger_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    this.set_team_count(0x40);
    let l = exe::crt_malloc(0x180);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0x9cd3e0, 0x13c);
        return -1;
    }
    let list = exe::crt_malloc(0xaf0);
    if list == 0 {
        show_error_at(0x9cd3e0, 0x144);
        return 0;
    }
    let mut k = 0i32;
    let mut n: u32 = 0;
    let mut flag: u32 = 0;
    let limit = || (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i16;
    while (i as i32) < limit() {
        let d = read_usize(club + club::DIVISION);
        if d != 0 {
            let amateur = (read_usize(club + club::RESERVE_DIVISION) != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GERMAN_REGIONAL)) || {
                let nation = read_usize(club + club::NATION);
                nation != 0 && read_usize(nation) == read_usize(KEY_NATION_GERMANY) && read_usize(d) == read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION)
            };
            if amateur && (exe::club_get_linked_b_team(club, &mut flag, 1) == 0 || flag != 0) {
                let last = read_usize(club + club::LAST_DIVISION);
                if last != 0 && read_usize(last) == read_usize(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION) && k * 6 < 0x18 { // mwc22: up to 4 relegated amateur clubs
                    set_cup_entry(this, k as usize, club, 2);
                    k += 1;
                } else if (n as u16 as i16) < 0x2bc {
                    write_usize(list + (n as u16 as i16 as isize as usize).wrapping_mul(4), club);
                    n = n.wrapping_add(1);
                }
            }
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    covered!("mwc_22 german cup entrants");
    exe::comp_util_sort_and_shuffle_top_clubs(list, n, 0x18); // mwc22: the best 72 amateur clubs shuffled
    let count = n as u16 as i16;
    if count > 0 {
        let mut j = 0i16;
        while k * 6 < 0xa8 {
            let slot = list + 4 * j as usize;
            set_cup_entry(this, k as usize, read_usize(slot), 2);
            write_usize(slot, 0);
            k += 1;
            j += 1;
            if j >= count {
                break;
            }
        }
    }
    if count > 0 {
        let mut j = 0i16;
        while k * 6 < 0xa8 {
            set_cup_entry(this, k as usize, read_usize(list + 4 * j as usize), 2);
            k += 1;
            j += 1;
            if j >= count {
                break;
            }
        }
    }
    exe::crt_free(list);
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i16;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION) && k < this.team_count() as i32 {
            let last = read_usize(club + club::LAST_DIVISION);
            let up = last != 0 && read_usize(last) != read_usize(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION) && read_usize(last) != read_usize(KEY_CLUB_COMP_GERMAN_FIRST_DIVISION);
            set_cup_entry(this, k as usize, club, if up { 2 } else { 1 });
            k += 1;
            if k == this.team_count() as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i16;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GERMAN_FIRST_DIVISION) && k < this.team_count() as i32 {
            set_cup_entry(this, k as usize, club, 1);
            k += 1;
            if k == this.team_count() as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    (k == this.team_count() as i32) as i32
}

/// Comparer for `ger_cup_collect_entrants` (side by side with the random generator restored, every entry compared).
pub(crate) unsafe extern "thiscall" fn compare_ger_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_GER_CUP_COLLECT_ENTRANTS);
    compare_entrants_with_seeds("ger_cup_collect_entrants", Cup(this), || o(this), || ger_cup_collect_entrants(this), |_| true)
}

/// At the season turn the German Cup is rebuilt: last edition's tables freed, the history archived, the year +1, the
/// entrants (GerCup::collect_entrants), the rounds, the tie tree and the predicted positions.
///
/// The knockout-cup turnover template (`usa_mls_all_stars_cup_start_new_season`), checked instruction by instruction
/// against `JapSuper::start_new_season`: only the entrants routine (`GerCup::collect_entrants`, thiscall, ret 0), the
/// source file and the box lines (0x220, 0x227, 0x22e) change.
/// Original: `GerCup::start_new_season` (0x5d9780, ger_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_cup_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x5d9370, 0x9cd3e0, [0x220, 0x227, 0x22e], 1, true, "mwc_14 ger cup turnover") // mwc14: the German Cup
}

/// Comparer for `ger_cup_start_new_season`: only the original runs; the year goes up by one (`compare_cup_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_ger_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GER_CUP_START_NEW_SEASON);
    compare_cup_new_season("ger_cup_start_new_season", Cup(this), || o(this))
}

/// The German Cup is born with discipline rule 14, the bench (6 substitutes named, 3 substitutions); with no saved
/// state, the entrants (GerCup::collect_entrants), the round table, the tie tree (a failure is a 0 return), the
/// statistics and the predicted positions (`construct_nation_cup`).
/// Original: `GerCup::GerCup` (0x5d8c20, ger_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ger_cup(this: usize, year: u32, record: usize) -> usize {
    construct_nation_cup(Cup(this), year, record, &GER_CUP_CUP_PARAMS)
}

/// Comparer for `construct_ger_cup`: only the original runs (`compare_nation_cup_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_ger_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_GER_CUP);
    compare_nation_cup_ctor("construct_ger_cup", Cup(this), year, record, &GER_CUP_CUP_PARAMS, || o(this, year, record))
}

/// The German Cup has 6 round(s): code 10: draw 10 Jul, match 26 Aug, 64 teams (64 new); code 20: draw 27 Aug, match 11
/// Oct, 32 teams (0 new); code 30: draw 12 Nov, match 29 Nov, 16 teams (0 new); code 120: draw 30 Nov, match 20 Dec, 8
/// teams (0 new); code 130: draw 21 Dec, match 7 Feb (+1 year), 4 teams (0 new); code 150: draw 8 Feb (+1 year), match
/// 11 May (+1 year), 2 teams (0 new); in 2001/02 the final is on 11 May, in the following seasons on 26 May.
///
/// `GerCup::create_round_table(stage, &rounds, &extra, &out)` (`single_stage_round_table`): only stage -1 (the others
/// return 0 without writing); out = 0, rounds = the table size, extra = 0 and the table of 0x68-byte rounds
/// (`exe::crt_malloc`; out of memory, the error box of line 0xa6 and 0), each with its draw and match dates
/// (`cup_set_round_draw_date`, `cup_set_round_match_date`, cdecl) and its fields (`fra_cup_write_cup_round`).
/// Original: `GerCup::create_round_table` (0x5d8ee0, ger_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ger_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let table: &[FraCupCupRound] = if Cup(this).year() as u16 == 0x7d1 { &GER_CUP_2001_ROUNDS } else { &GER_CUP_LATER_ROUNDS };
    single_stage_round_table(Cup(this), stage, rounds, extra, out, table, 0x9cd3e0, 0xa6, "mwc_13 ger cup round table")
}

/// Comparer for `ger_cup_create_round_table`: side by side (`compare_cup_round_table`).
pub(crate) unsafe extern "thiscall" fn compare_ger_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_GER_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("ger_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| ger_cup_create_round_table(this, stage, r, e, s))
}

/// Writes entry `k` of a cup's 6-byte team list (`cup::TEAM_TABLE`): the club, status 0 (+5) and class `class` (+4).
pub(crate) unsafe fn set_cup_entry(this: Cup, k: usize, club: usize, class: u8) {
    write_usize(this.team_table() + 6 * k, club);
    write_u8(this.team_table() + 6 * k + 5, 0);
    write_u8(this.team_table() + 6 * k + 4, class);
}

pub(crate) const GER_CUP_CUP_PARAMS: NationCup = NationCup {
    vtable: 0x96b044,
    bench: Some((6, 3)),
    discipline: discipline_rule::GERMANY_CUP, // mwc16: the German Cup discipline rule
    entrants: 0x5d9370,
    zero_fails: true,
    predicted: true,
    source_file: 0x9cd3e0,
    box_lines: [0x4d, 0x5b, 0x62],
};

/// The German Cup 2001/02 (the final on 11 May 2002).
pub(crate) const GER_CUP_2001_ROUNDS: [FraCupCupRound; 6] = [
    cql11r((0xa, 6, 0, 5), (0x1a, 7, 0, 5, 1, 0), 0xa, (1, 0x403, 0), 4, (0x40, 0x20, 0x40, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x1b, 7, 0, 6), (0xb, 9, 0, 2, 2, 0), 0x14, (1, 0x403, 0), 4, (0x20, 0x10, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0xc, 0xa, 0, 6), (0x1d, 0xa, 0, 2, 2, 0), 0x1e, (1, 0x403, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x1e, 0xa, 0, 3), (0x14, 0xb, 0, 2, 2, 0), 0x78, (1, 3, 0), 4, (8, 4, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)), // mwc13 ger_cup round 3
    cql11r((0x15, 0xb, 0, 3), (7, 1, 1, 2, 2, 0), 0x82, (1, 3, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((8, 1, 1, 3), (0xb, 4, 1, 5, 1, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
];

/// The German Cup from 2002/03 (the final on 26 May).
pub(crate) const GER_CUP_LATER_ROUNDS: [FraCupCupRound; 6] = [
    cql11r((0xa, 6, 0, 5), (0x1a, 7, 0, 5, 1, 0), 0xa, (1, 0x403, 0), 4, (0x40, 0x20, 0x40, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x1b, 7, 0, 6), (0xb, 9, 0, 2, 2, 0), 0x14, (1, 0x403, 0), 4, (0x20, 0x10, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0xc, 0xa, 0, 6), (0x1d, 0xa, 0, 2, 2, 0), 0x1e, (1, 0x403, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x1e, 0xa, 0, 3), (0x14, 0xb, 0, 2, 2, 0), 0x78, (1, 3, 0), 4, (8, 4, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x15, 0xb, 0, 3), (7, 1, 1, 2, 2, 0), 0x82, (1, 3, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((8, 1, 1, 3), (0x1a, 4, 1, 5, 1, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
];
