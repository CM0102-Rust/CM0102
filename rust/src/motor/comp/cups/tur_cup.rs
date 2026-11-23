use super::*;

/// Rule: the Turkish Cup takes 64 clubs, all class 1: 26 of the second division category B (the first 30 in database
/// order, sorted by their last division, `compare_clubs_by_last_division`, taken from the 26th backwards), then the
/// first 20 of category A in reverse database order, and the top-flight clubs from the end of the list backwards.
///
/// `team_count` = 64; the list (`exe::crt_malloc` of 0x180 into `cup::TEAM_TABLE`; out of memory: the box at line 0x142
/// and −1). Over the first [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`] clubs with a division (`club::DIVISION`): category
/// B (`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B`) into a local list of at most 30; else category A
/// (`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A`) into a local list of at most 20; else the top flight
/// (`KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION`) straight into the entries 63, 62, ... The category B list sorted (qsort
/// `exe::crt_qsort`(list, n, 4, 0x4c0a40)) and its entries 25..0 copied to the entries 0..25 (the original copies 26
/// whatever the count: with fewer it reads stack garbage; the copy reads zeros), then the category A list from its last
/// to its first into the entries 26 onwards. Returns 1.
/// Original: `TurCup::collect_entrants` (0x8fcb90, tur_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let mut lower = [0usize; 30];
    let mut second = [0usize; 20];
    let (mut nl, mut ns, mut np) = (0i16, 0i16, 0i32);
    this.set_team_count(0x40);
    let l = exe::crt_malloc(0x180);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0xaa180c, 0x142);
        return -1;
    }
    let limit = || (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < limit() {
        let d = read_usize(club + club::DIVISION);
        if d != 0 {
            if nl < 0x1e && read_usize(d) == read_usize(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B) {
                lower[nl as usize] = club;
                nl += 1;
            } else if read_usize(d) == read_usize(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A) && ns < 0x14 {
                second[ns as usize] = club;
                ns += 1;
            } else if read_usize(d) == read_usize(KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION) {
                np += 1;
                set_cup_entry(this, (this.team_count() as i32 - np) as usize, club, 1);
            }
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    covered!("mwd_27 turkish cup entrants");
    exe::crt_qsort(lower.as_mut_ptr() as usize, nl as i32 as u32, 4, 0x4c0a40usize);
    for k in 0..0x1a {
        set_cup_entry(this, k, lower[0x19 - k], 1); // mwd27: category B from the 26th backwards
    }
    let mut j = 0usize;
    let mut k = ns as i32 - 1;
    while k >= 0 {
        set_cup_entry(this, 0x1a + j, second[k as usize], 1);
        j += 1;
        k -= 1;
    }
    1
}

/// Comparer for `tur_cup_collect_entrants` (side by side, every entry compared).
pub(crate) unsafe extern "thiscall" fn compare_tur_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_TUR_CUP_COLLECT_ENTRANTS);
    compare_entrants_with_seeds("tur_cup_collect_entrants", Cup(this), || o(this), || tur_cup_collect_entrants(this), |_| true)
}

/// Rule: the Turkish Cup at career creation: discipline rule 0x26, 7 substitutes and 3 substitutions and, with no saved
/// state, the 64 entrants, a 600,000 prize fund, the seven rounds, the fixture tree, the statistics and the predicted
/// positions (`construct_east_nation_cup`).
/// Original: `TurCup::TurCup` (0x8fc3b0, tur_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_tur_cup(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &TURKISH_CUP, "mwd_19 turkish cup new")
}

/// Comparer for `construct_tur_cup`.
pub(crate) unsafe extern "thiscall" fn compare_construct_tur_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_TUR_CUP);
    compare_east_nation_cup_ctor("construct_tur_cup", Cup(this), year, record, &TURKISH_CUP, || o(this, year, record))
}

/// Rule: the Turkish Cup has seven single-leg rounds (tie rule 0x83): the first round (36 teams, code 0xa; +0x22 = 4)
/// drawn 17/10, played 25/10 (weekday 2); the second round (28 teams, 10 new from entry 36) drawn 31/10, played 8/11;
/// the third round (32 teams, 18 new from entry 46) drawn 5/12, played 13/12; the round of 16 drawn 9/1, played 17/1 of
/// the next year, nothing to the winners and 3,000 to each loser; the quarter-finals (category 6) drawn 10/1, played
/// 31/1; the semi-finals drawn 6/2, played 14/2; the final drawn 15/2, played 2/5 at venue 4 [inference: a neutral
/// ground].
/// Original: `TurCup::create_round_table` (0x8fc670, tur_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn tur_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    fra_super_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &TURKISH_CUP_ROUNDS, 0xaa180c, 0x9d, "mwd_20 turkish cup rounds", |_| {})
}

/// Comparer for `tur_cup_create_round_table` (side by side, the written rounds compared).
pub(crate) unsafe extern "thiscall" fn compare_tur_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_TUR_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("tur_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| tur_cup_create_round_table(this, stage, r, e, s))
}

/// Rule: the Turkish Cup season turnover, as the Russian Cup's, with its 64 entrants (`TurCup::collect_entrants`
/// 0x8fcb90; lines 0x200, 0x207, 0x20e of tur_cup.cpp).
/// Original: `TurCup::start_new_season` (0x8fcdb0, tur_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x8fcb90, 0xaa180c, [0x200, 0x207, 0x20e], 1, "mwd_10 turkish cup turnover") // mwd10: the Turkish Cup turnover
}

/// Comparer for `tur_cup_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_tur_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_TUR_CUP_START_NEW_SEASON);
    compare_cup_new_season("tur_cup_start_new_season", Cup(this), || o(this))
}

/// The Turkish Cup: discipline rule 0x26, 600,000 prize fund.
pub(crate) const TURKISH_CUP: EastNationCup = EastNationCup {
    vtable: 0x97048c,
    discipline: 0x26, // mwd19: the Turkish Cup discipline rule
    entrants: 0x8fcb90,
    prize: Some(0x927c0),
    late_substitutes: false,
    stage_vector: false,
    rank: true,
    file: 0xaa180c,
    rows: [0x44, 0x52, 0x59],
};

/// The Turkish Cup: seven single-leg rounds from October to May, tie rule 0x83; a share of the fund per round won,
/// except in the round of 16 (nothing to the winners, 3,000 to each loser).
pub(crate) const TURKISH_CUP_ROUNDS: [FraSuperCupRound; 7] = [
    cql12r((0x11, 9, 0, 1), (0x19, 9, 0, 2, 2, 0), 0xa, (0, 0x83, 0), 4, (0x24, 0x12, 0x24, 0), (0, 1, 4), (0, 0xffffffff, 0)),
    cql12r((0x1f, 9, 0, 1), (8, 0xa, 0, 2, 2, 0), 0x14, (0, 0x83, 0), 4, (0x1c, 0xe, 0xa, 0x24), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((5, 0xb, 0, 1), (0xd, 0xb, 0, 2, 2, 0), 0x1e, (0, 0x83, 0), 4, (0x20, 0x10, 0x12, 0x2e), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((9, 0, 1, 1), (0x11, 0, 1, 2, 2, 0), 0x28, (0, 0x83, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0, 0xbb8)),
    cql12r((0xa, 0, 1, 2), (0x1f, 0, 1, 2, 2, 0), 0x78, (0, 0x83, 0), 6, (8, 4, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((6, 1, 1, 1), (0xe, 1, 1, 2, 2, 0), 0x82, (0, 0x83, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((0xf, 1, 1, 3), (2, 4, 1, 2, 2, 4), 0x96, (0, 0x83, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
];
