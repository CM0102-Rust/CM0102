use super::*;

/// Rule: the Portuguese Cup at career creation: discipline rule 23 and, with no saved state, the 164 entrants, an
/// 800,000 prize fund, the eight rounds, the fixture tree, the statistics and the predicted positions
/// (`construct_british_cup`, the WelCup template, checked instruction by instruction: it does not write the bench).
/// Original: `PorCup::PorCup` (0x7cd980, por_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_por_cup(this: usize, year: u32, record: usize) -> usize {
    construct_british_cup(Cup(this), year, record, &PORTUGUESE_CUP, "mwe_20 portuguese cup new")
}

/// Rule: the Portuguese Cup has eight rounds, single matches with a replay on a draw until the semi-finals (tie rules 2
/// and 3): the first round (128 teams, the lower clubs) drawn 10/9 and played 19/9; the second round (82 teams: the 18
/// Second Division clubs come in at entrant 128) drawn 23/9 and played 5/10; the third round (58: the 17 Premier
/// Division clubs from entrant 146) drawn 18/10 and played 14/11; the fourth round (30: the last club, entrant 163)
/// drawn 29/12 and played 9/1 of the next year; the fifth round (15 teams in 7 ties, as the original) drawn 10/1 and
/// played 26/1; the quarter-finals drawn 3/2 and played 9/2; the semi-finals drawn 20/3
/// and played 12/4; the final drawn 13/4 and played 21/5 at venue 4, settled on the day, a replay 4 days later; no prize
/// per round (a share of the fund). Outside stage −1 it writes nothing and returns 0; out of memory, the box at line
/// 0xa3 and 0 (`fra_super_create_single_stage_round_table`).
/// Original: `PorCup::create_round_table` (0x7cdc40, por_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn por_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    fra_super_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &PORTUGUESE_CUP_ROUNDS, POR_CUP_FILE, 0xa3, "mwe_20 portuguese cup rounds", |_| {})
}

/// Rule: the Portuguese Cup takes 164 clubs: first the Second Division B clubs (the South, Central and North groups and
/// the Second Division B itself), then the Third Division and the Portuguese clubs (`KEY_NATION_PORTUGAL`) of the lower
/// division (`KEY_CLUB_COMP_A_LOWER_DIVISION`), up to 128 in these two passes and without the B teams linked to no club
/// (`portuguese_cup_takes`); then the Second Division and last the Premier Division clubs, while the list has room. Seed
/// (+4) and status (+5) 0. Last, the club pointers of the last entry and of the entry `utils_random`(18) + 1 places before
/// it are swapped (`exe::utils_random`, cdecl) [inference: one Premier Division club drawn into an earlier place].
///
/// `team_count` = 164; the list (`exe::crt_malloc` of 164 × 6 into `cup::TEAM_TABLE`; out of memory: the box at line
/// 0x159 and −1). Returns 1 when the list is full, otherwise 0.
/// Original: `PorCup::collect_entrants` (0x7ce220, por_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    this.set_team_count(0xa4);
    let l = exe::crt_malloc(0x3d8);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(POR_CUP_FILE, 0x159);
        return -1;
    }
    let mut k: i32 = 0;
    let mut parent: u32 = 0;
    let add = |k: &mut i32, club: usize| {
        write_usize(this.team_table() + 6 * *k as usize, club);
        write_u8(this.team_table() + 6 * *k as usize + 4, 0);
        write_u8(this.team_table() + 6 * *k as usize + 5, 0);
        *k += 1;
    };
    let second_b = [KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_SOUTH, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_CENTRAL, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_NORTH,
        KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B];
    for pass in 0..2 {
        let mut club = read_usize(G_CLUB_TABLE);
        let mut i: i16 = 0;
        while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
            let d = read_usize(club + club::DIVISION);
            let wanted = d != 0 && if pass == 0 {
                second_b.iter().any(|&key| read_usize(d) == read_usize(key))
            } else {
                let n = read_usize(club + club::NATION);
                read_usize(d) == read_usize(KEY_CLUB_COMP_PORTUGUESE_THIRD_DIVISION)
                    || (read_usize(d) == read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION) && n != 0 && read_usize(n) == read_usize(KEY_NATION_PORTUGAL))
            };
            if wanted && 6 * k < 0x300 && portuguese_cup_takes(club, &mut parent) { // mwe20: up to 128 lower clubs
                add(&mut k, club);
            }
            i = i.wrapping_add(1);
            club = club.wrapping_add(club::SIZE);
        }
    }
    covered!("mwe_20 portuguese cup entrants");
    for key in [KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION, KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION] {
        let mut club = read_usize(G_CLUB_TABLE);
        let mut i: i16 = 0;
        while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
            let d = read_usize(club + club::DIVISION);
            if d != 0 && read_usize(d) == read_usize(key) && k < this.team_count() as i32 {
                add(&mut k, club);
                if k == this.team_count() as i32 {
                    break;
                }
            }
            i = i.wrapping_add(1);
            club = club.wrapping_add(club::SIZE);
        }
    }
    let r = exe::utils_random(0x12); // mwe20: the place of the last club drawn
    let other = this.team_table().wrapping_add((((k - r - 1) as i16) as isize * 6) as usize);
    let last = this.team_table().wrapping_add((6 * k - 6) as isize as usize);
    let c = read_usize(last);
    write_usize(last, read_usize(other));
    write_usize(other, c);
    (k == this.team_count() as i32) as i32
}

/// Comparer for `construct_por_cup` (only the original runs; then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_por_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_POR_CUP);
    compare_british_cup_ctor("construct_por_cup", Cup(this), year, record, &PORTUGUESE_CUP, || o(this, year, record))
}

/// Comparer for `por_cup_create_round_table` (side by side, the written rounds compared).
pub(crate) unsafe extern "thiscall" fn compare_por_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_POR_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("por_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| por_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `por_cup_collect_entrants` (side by side with the random generator restored: the club, seed and status
/// of every entry).
pub(crate) unsafe extern "thiscall" fn compare_por_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_POR_CUP_COLLECT_ENTRANTS);
    compare_entrants_with_seeds("por_cup_collect_entrants", Cup(this), || o(this), || por_cup_collect_entrants(this), |_| true)
}

/// The Portuguese Cup turnover: the same as the Finnish Cup's, with PorCup::collect_entrants (0x7ce220), por_cup.cpp
/// (0xa786f0) and the box lines 0x20d, 0x214, 0x21b.
/// Original: `PorCup::start_new_season` (0x7ce540, por_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x7ce220, 0xa786f0, [0x20d, 0x214, 0x21b], 1, "mwe_16 portuguese cup turnover") // mwe16: the Portuguese Cup next year
}

/// Comparer for `por_cup_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_por_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POR_CUP_START_NEW_SEASON);
    compare_cup_new_season("por_cup_start_new_season", Cup(this), || o(this))
}

/// Whether a club takes a place in the lower part of the Portuguese Cup: club_get_linked_b_team
/// (`exe::club_get_linked_b_team`(club, &parent, 1), cdecl) left out the B teams it links to no club (nonzero with a null
/// parent); `parent` keeps its value from one call to the next, as the original's local.
pub(crate) unsafe fn portuguese_cup_takes(club: usize, parent: &mut u32) -> bool {
    exe::club_get_linked_b_team(club, parent, 1) == 0 || *parent != 0
}

/// The Portuguese Cup: the Cup bench defaults (not written), discipline rule 23 (RbPortugal), a prize fund of 800,000.
pub(crate) const PORTUGUESE_CUP: BritishCupSpec = BritishCupSpec {
    vtable: 0x96e650,
    bench: None,
    discipline: discipline_rule::PORTUGAL,
    prize_fund: 0xc3500,
    entrants: 0x7ce220,
    file: POR_CUP_FILE,
    rows: [0x4a, 0x58, 0x5f],
};

/// The Portuguese Cup: eight rounds from August to May; no prize per round won (−1 = a share of the fund).
pub(crate) const PORTUGUESE_CUP_ROUNDS: [FraSuperCupRound; 8] = [
    cql12r((0xa, 8, 0, 4), (0x13, 8, 0, 6, 1, 0), 0x14, (0, 2, 3), 4, (0x80, 0x40, 0x80, 0), (1, 1, 3), (0, 0xffffffff, 0)), // mwe20: the first round
    cql12r((0x17, 8, 0, 3), (5, 9, 0, 1, 1, 0), 0x1e, (0, 2, 3), 4, (0x52, 0x29, 0x12, 0x80), (1, 1, 8), (0, 0xffffffff, 0)),
    cql12r((0x12, 9, 0, 0), (0xe, 0xa, 0, 6, 1, 0), 0x28, (0, 2, 3), 4, (0x3a, 0x1d, 0x11, 0x92), (1, 1, 0x11), (0, 0xffffffff, 0)),
    cql12r((0x1d, 0xb, 0, 0), (9, 0, 1, 6, 1, 0), 0x32, (0, 2, 3), 4, (0x1e, 0xf, 1, 0xa3), (1, 1, 0), (0, 0xffffffff, 0)),
    cql12r((0xa, 0, 1, 0), (0x1a, 0, 1, 2, 2, 0), 0x3c, (0, 2, 3), 4, (0xf, 7, 0, 0), (1, 1, 7), (0, 0xffffffff, 0)),
    cql12r((3, 1, 1, 3), (9, 1, 1, 2, 2, 0), 0x78, (0, 2, 3), 6, (8, 4, 0, 0), (1, 1, 7), (0, 0xffffffff, 0)),
    cql12r((0x14, 2, 1, 0), (0xc, 3, 1, 2, 2, 0), 0x82, (0, 2, 3), 6, (4, 2, 0, 0), (1, 1, 7), (0, 0xffffffff, 0)),
    cql12r((0xd, 3, 1, 3), (0x15, 4, 1, 6, 1, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 4), (0, 0xffffffff, 0)),
];

/// The source file of the Portuguese Cup's error boxes.
pub(crate) const POR_CUP_FILE: usize = 0xa786f0;
