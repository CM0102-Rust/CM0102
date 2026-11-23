use super::*;

/// Rule: the Danish Cup takes 92 clubs, all class 0: up to 48 Danish clubs of the lowest division, then the Danish
/// Second Division clubs, the First Division's and the Superliga's, each in database order until the list is full;
/// returns 1 when it is full.
///
/// `team_count` = 92; the list (`exe::crt_malloc` of 0x228; out of memory: the box of den_cup.cpp at line 0x15a and −1).
/// Four passes over the clubs (a 16-bit index up to [`G_CLUB_COUNT`]), each adding at the next entry with class 0: the
/// division id of `KEY_CLUB_COMP_A_LOWER_DIVISION` with Denmark's nation record (`KEY_NATION_DENMARK`) while fewer than
/// 48 are in; `KEY_CLUB_COMP_DANISH_SECOND_DIVISION` with Denmark's nation record while the count is below
/// `team_count` (the pass does not stop); `KEY_CLUB_COMP_DANISH_FIRST_DIVISION` and
/// `KEY_CLUB_COMP_DANISH_PREMIER_DIVISION` (no nation check) while below `team_count`, stopping when it is reached.
/// Returns count == `team_count`.
/// Original: `DenCup::collect_entrants` (0x551240, den_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    this.set_team_count(0x5c);
    let l = exe::crt_malloc(0x228);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0x9b8dfc, 0x15a);
        return -1;
    }
    let mut k = 0i32;
    let denmark = || oceania_nations_nation_record(KEY_NATION_DENMARK);
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i16;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION) && read_usize(club + club::NATION) == denmark() && k * 6 < 0x120 { // mwc22: up to 48 from the lowest division
            set_cup_entry(this, k as usize, club, 0);
            k += 1;
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    covered!("mwc_22 danish cup entrants");
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i16;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_DANISH_SECOND_DIVISION) && read_usize(club + club::NATION) == denmark() && k < this.team_count() as i32 {
            set_cup_entry(this, k as usize, club, 0);
            k += 1;
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    for g in [KEY_CLUB_COMP_DANISH_FIRST_DIVISION, KEY_CLUB_COMP_DANISH_PREMIER_DIVISION] {
        let mut club = read_usize(G_CLUB_TABLE);
        let mut i = 0i16;
        while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
            let d = read_usize(club + club::DIVISION);
            if d != 0 && read_usize(d) == read_usize(g) && k < this.team_count() as i32 {
                set_cup_entry(this, k as usize, club, 0);
                k += 1;
                if k == this.team_count() as i32 {
                    break;
                }
            }
            i = i.wrapping_add(1);
            club = club.wrapping_add(club::SIZE);
        }
    }
    (k == this.team_count() as i32) as i32
}

/// Comparer for `den_cup_collect_entrants` (side by side, every entry compared).
pub(crate) unsafe extern "thiscall" fn compare_den_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_DEN_CUP_COLLECT_ENTRANTS);
    compare_entrants_with_seeds("den_cup_collect_entrants", Cup(this), || o(this), || den_cup_collect_entrants(this), |_| true)
}

/// At the season turn the Danish Cup is rebuilt: last edition's tables freed, the history archived, the year +1, the
/// entrants (DenCup::collect_entrants), the rounds, the tie tree and the predicted positions.
///
/// The knockout-cup turnover template (`usa_mls_all_stars_cup_start_new_season`), checked instruction by instruction
/// against `JapSuper::start_new_season`: only the entrants routine (`DenCup::collect_entrants`, thiscall, ret 0), the
/// source file and the box lines (0x1fe, 0x205, 0x20c) change.
/// Original: `DenCup::start_new_season` (0x551500, den_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_cup_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x551240, 0x9b8dfc, [0x1fe, 0x205, 0x20c], 1, true, "mwc_14 den cup turnover") // mwc14: the Danish Cup
}

/// Comparer for `den_cup_start_new_season`: only the original runs; the year goes up by one (`compare_cup_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_den_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_DEN_CUP_START_NEW_SEASON);
    compare_cup_new_season("den_cup_start_new_season", Cup(this), || o(this))
}

/// The Danish Cup is born with discipline rule 8; with no saved state, the entrants (DenCup::collect_entrants), the
/// round table, the tie tree (a failure is a 0 return), the statistics and the predicted positions
/// (`construct_nation_cup`).
/// Original: `DenCup::DenCup` (0x5509b0, den_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_den_cup(this: usize, year: u32, record: usize) -> usize {
    construct_nation_cup(Cup(this), year, record, &DEN_CUP_CUP_PARAMS)
}

/// Comparer for `construct_den_cup`: only the original runs (`compare_nation_cup_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_den_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_DEN_CUP);
    compare_nation_cup_ctor("construct_den_cup", Cup(this), year, record, &DEN_CUP_CUP_PARAMS, || o(this, year, record))
}

/// The Danish Cup has 8 round(s): code 10: draw 1 Aug, match 12 Aug, 64 teams (64 new); code 20: draw 14 Aug, match 19
/// Aug, 40 teams (8 new); code 30: draw 21 Aug, match 26 Aug, 28 teams (8 new); code 40: draw 28 Aug, match 5 Sep, 20
/// teams (6 new); code 50: draw 7 Sep, match 12 Sep, 16 teams (6 new); code 120: draw 14 Sep, match 19 Sep, 8 teams (0
/// new); code 130: draw 21 Sep, match 20 Mar (+1 year), 4 teams (0 new), two legs; code 150: draw 29 Mar (+1 year),
/// match 1 Jun (+1 year), 2 teams (0 new).
///
/// `DenCup::create_round_table(stage, &rounds, &extra, &out)` (`single_stage_round_table`): only stage -1 (the others
/// return 0 without writing); out = 0, rounds = the table size, extra = 0 and the table of 0x68-byte rounds
/// (`exe::crt_malloc`; out of memory, the error box of line 0xa5 and 0), each with its draw and match dates
/// (`cup_set_round_draw_date`, `cup_set_round_match_date`, cdecl) and its fields (`fra_cup_write_cup_round`).
/// Original: `DenCup::create_round_table` (0x550c70, den_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn den_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    single_stage_round_table(Cup(this), stage, rounds, extra, out, &DEN_CUP_ROUNDS, 0x9b8dfc, 0xa5, "mwc_12 den cup round table")
}

/// Comparer for `den_cup_create_round_table`: side by side (`compare_cup_round_table`).
pub(crate) unsafe extern "thiscall" fn compare_den_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_DEN_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("den_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| den_cup_create_round_table(this, stage, r, e, s))
}

pub(crate) const DEN_CUP_CUP_PARAMS: NationCup = NationCup {
    vtable: 0x969590,
    bench: None,
    discipline: discipline_rule::DENMARK, // mwc16: the Danish Cup discipline rule
    entrants: 0x551240,
    zero_fails: true,
    predicted: true,
    source_file: 0x9b8dfc,
    box_lines: [0x4b, 0x5a, 0x61],
};

/// The Danish Cup: 8 rounds.
pub(crate) const DEN_CUP_ROUNDS: [FraCupCupRound; 8] = [
    cql11r((1, 7, 0, 4), (0xc, 7, 0, 2, 2, 0), 0xa, (1, 3, 0), 4, (0x40, 0x20, 0x40, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0xe, 7, 0, 4), (0x13, 7, 0, 2, 2, 0), 0x14, (1, 3, 0), 4, (0x28, 0x14, 8, 0x40), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x15, 7, 0, 4), (0x1a, 7, 0, 2, 2, 0), 0x1e, (1, 3, 0), 4, (0x1c, 0xe, 8, 0x48), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x1c, 7, 0, 4), (5, 8, 0, 2, 2, 0), 0x28, (1, 3, 0), 4, (0x14, 0xa, 6, 0x50), (0, 1, 0), (0, 0xffffffff, 0)), // mwc12 den_cup round 3
    cql11r((7, 8, 0, 4), (0xc, 8, 0, 2, 2, 0), 0x32, (1, 3, 0), 4, (0x10, 8, 6, 0x56), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0xe, 8, 0, 4), (0x13, 8, 0, 2, 2, 0), 0x78, (1, 3, 0), 6, (8, 4, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x15, 8, 0, 4), (0x14, 2, 1, 2, 2, 0), 0x82, (0, 0, 3), 6, (4, 2, 0, 0), (0, 2, 7), (0, 0xffffffff, 0)),
    cql11r((0x1d, 2, 1, 4), (1, 5, 1, 3, 2, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
];
