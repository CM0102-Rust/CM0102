use super::*;

/// Rule: the Finnish Cup at career creation: discipline rule 11, 7 substitutes and 3 substitutions and, with no saved
/// state, the 76 entrants, a 200,000 prize fund, the seven rounds, the fixture tree, the statistics and the predicted
/// positions (`construct_east_nation_cup`, the CroCup template, checked instruction by instruction).
/// Original: `FinCup::FinCup` (0x592250, fin_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_fin_cup(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &FINNISH_CUP, "mwe_18 finnish cup new")
}

/// Rule: the Finnish Cup has seven single-leg rounds, the tie settled on the day (tie-break code 3): the first round (64
/// teams, the Second Division and lower clubs, code 0x1e) drawn 15/4 and played 29/4; the second round (32) drawn 3/5
/// and played 16/5; the third round (24 teams: the 8 First Division clubs come in at entrant 64) drawn 5/6 and played
/// 21/6; the fourth round (16: the 4 Premier Division clubs of the last places come in at entrant 72) drawn 22/6 and
/// played 25/7; the quarter-finals drawn 27/7 and played 19/9; the semi-finals drawn 21/9 and played 17/10; the final
/// drawn 18/10 and played 10/11 at venue 4 (category 6 from the quarter-finals); no prize per round (a share of the
/// fund). Outside stage −1 it writes nothing and returns 0; out of memory, the box at line 0xa4 and 0
/// (`fra_super_create_single_stage_round_table`).
/// Original: `FinCup::create_round_table` (0x592510, fin_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn fin_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    fra_super_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &FINNISH_CUP_ROUNDS, FIN_CUP_FILE, 0xa4, "mwe_18 finnish cup rounds", |_| {})
}

/// Rule: the Finnish Cup takes 76 clubs. First the Second Division and lower clubs (the four Second Division groups,
/// `KEY_CLUB_COMP_FINNISH_SECOND_DIVISION_WEST`/`_EAST`/`_NORTH`/`_SOUTH`, and the lower division), looked at among the
/// first [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`] clubs (without the national teams [inference]), up to 80 in a local
/// list (the 81st stops the search); the list is sorted by reputation and shuffled
/// (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 44), cdecl) and its first 44 go in. Then the First Division
/// clubs (`KEY_CLUB_COMP_FINNISH_FIRST_DIVISION`, among all the clubs of the database), then the Premier Division clubs
/// that finished below 5th last season (`club::LAST_POSITION` > 4, signed) and then those that finished in the top 5,
/// each while the list has room. Seed byte (+4): 1 for the Premier Division clubs, 0 for the others; status (+5) 0.
///
/// `team_count` = 76; the list (`exe::crt_malloc` of 76 × 6 into `cup::TEAM_TABLE`; out of memory: the box at line
/// 0x148 and −1). Returns 1 when the list is full, otherwise 0. Defect of the original: the 44 are copied even when
/// fewer than 44 lower clubs were found (the rest is stack garbage; here, null clubs).
/// Original: `FinCup::collect_entrants` (0x592a20, fin_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    this.set_team_count(0x4c);
    let l = exe::crt_malloc(0x1c8);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(FIN_CUP_FILE, 0x148);
        return -1;
    }
    let lower = [KEY_CLUB_COMP_FINNISH_SECOND_DIVISION_WEST, KEY_CLUB_COMP_FINNISH_SECOND_DIVISION_EAST, KEY_CLUB_COMP_FINNISH_SECOND_DIVISION_NORTH,
        KEY_CLUB_COMP_FINNISH_SECOND_DIVISION_SOUTH, KEY_CLUB_COMP_FINNISH_LOWER_DIVISION];
    let mut pool = [0usize; 80];
    let mut n: i16 = 0;
    let limit = (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < limit {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && lower.iter().any(|&k| read_usize(d) == read_usize(k)) {
            if n == 0x50 {
                break;
            }
            pool[n as usize] = club;
            n += 1;
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    exe::comp_util_sort_and_shuffle_top_clubs(pool.as_mut_ptr() as usize, n as u16 as u32, 0x2c); // mwe18: the 44 lower clubs drawn
    for k in 0..0x2c {
        write_usize(this.team_table() + 6 * k, pool[k]);
        write_u8(this.team_table() + 6 * k + 4, 0);
        write_u8(this.team_table() + 6 * k + 5, 0);
    }
    covered!("mwe_18 finnish cup entrants");
    let mut k: i32 = 0x2c;
    // the First Division, then the Premier Division below 5th, then the Premier Division top 5
    for (key, seed, take) in [(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION, 0u8, 0u8), (KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION, 1, 1), (KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION, 1, 2)] {
        let mut club = read_usize(G_CLUB_TABLE);
        let mut i: i16 = 0;
        while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
            let d = read_usize(club + club::DIVISION);
            let p = read_u8(club + club::LAST_POSITION) as i8;
            let wanted = match take {
                1 => p > 4,
                2 => p < 5,
                _ => true,
            };
            if d != 0 && read_usize(d) == read_usize(key) && wanted && k < this.team_count() as i32 {
                write_usize(this.team_table() + 6 * k as usize, club);
                write_u8(this.team_table() + 6 * k as usize + 4, seed); // mwe18: the Premier Division seed
                write_u8(this.team_table() + 6 * k as usize + 5, 0);
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

/// Comparer for `construct_fin_cup` (only the original runs; the fields checked).
pub(crate) unsafe extern "thiscall" fn compare_construct_fin_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_FIN_CUP);
    compare_east_nation_cup_ctor("construct_fin_cup", Cup(this), year, record, &FINNISH_CUP, || o(this, year, record))
}

/// Comparer for `fin_cup_create_round_table` (side by side, the written rounds compared).
pub(crate) unsafe extern "thiscall" fn compare_fin_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_FIN_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("fin_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| fin_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `fin_cup_collect_entrants` (side by side with the random generator restored: the club, seed and status
/// of every entry).
pub(crate) unsafe extern "thiscall" fn compare_fin_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_FIN_CUP_COLLECT_ENTRANTS);
    compare_entrants_with_seeds("fin_cup_collect_entrants", Cup(this), || o(this), || fin_cup_collect_entrants(this), |_| true)
}

/// The Finnish Cup turnover: last season's entrant, round and tie tables freed, the season archived in the statistics,
/// the year + 1; then the entrants (FinCup::collect_entrants), the rounds, the tie tree and the predicted positions.
///
/// `FinCup::start_new_season()`: the body of `start_new_cup_season` (checked instruction by instruction: only the
/// entrants routine 0x592a20, the source fin_cup.cpp (0x9c8158) and the box lines 0x206, 0x20d, 0x214 change): +0x4c =
/// 0; `cup::TEAM_TABLE` and the round list freed and zeroed; the tie arrays (one per round, `.round_count()` of them)
/// and their vector freed and zeroed; `exe::comp_stats_archive_season_and_reset`(1); the year + 1; the current round
/// and the stage flag 0; the entrants (direct, thiscall, ret 0), the rounds (virtual `comp_vtable::SET_UP_RULES`) and
/// the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`), each with its box when it returns a negative value (and 0
/// returned); last the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax is returned.
/// Original: `FinCup::start_new_season` (0x592d00, fin_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x592a20, 0x9c8158, [0x206, 0x20d, 0x214], 1, "mwe_16 finnish cup turnover") // mwe16: the Finnish Cup next year
}

/// Comparer for `fin_cup_start_new_season`: only the original runs (it frees and allocates); the year must go up by one
/// (`compare_cup_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_fin_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FIN_CUP_START_NEW_SEASON);
    compare_cup_new_season("fin_cup_start_new_season", Cup(this), || o(this))
}

/// The Finnish Cup: discipline rule 11 (RbFinlandCup), a prize fund of 200,000.
pub(crate) const FINNISH_CUP: EastNationCup = EastNationCup {
    vtable: 0x96a360,
    discipline: discipline_rule::FINLAND_CUP,
    entrants: 0x592a20,
    prize: Some(0x30d40),
    late_substitutes: false,
    stage_vector: false,
    rank: true,
    file: FIN_CUP_FILE,
    rows: [0x4b, 0x59, 0x60],
};

/// The Finnish Cup: seven single-leg rounds from March to October (the tie-break code 3 on the day); no prize per round
/// won (−1 = a share of the fund).
pub(crate) const FINNISH_CUP_ROUNDS: [FraSuperCupRound; 7] = [
    cql12r((0xf, 3, 0, 5), (0x1d, 3, 0, 5, 1, 0), 0x1e, (0, 3, 0), 4, (0x40, 0x20, 0x40, 0), (0, 1, 0), (0, 0xffffffff, 0)), // mwe18: the first round
    cql12r((3, 4, 0, 3), (0x10, 4, 0, 2, 2, 0), 0x28, (0, 3, 0), 4, (0x20, 0x10, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((5, 5, 0, 1), (0x15, 5, 0, 3, 2, 0), 0x32, (0, 3, 0), 4, (0x18, 0xc, 8, 0x40), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((0x16, 5, 0, 4), (0x19, 6, 0, 2, 2, 0), 0x3c, (0, 3, 0), 4, (0x10, 8, 4, 0x48), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((0x1b, 6, 0, 4), (0x13, 8, 0, 2, 2, 0), 0x78, (0, 3, 0), 6, (8, 4, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((0x15, 8, 0, 4), (0x11, 9, 0, 2, 2, 0), 0x82, (0, 3, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((0x12, 9, 0, 3), (0xa, 0xa, 0, 5, 1, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
];

/// The source file of the Finnish Cup's error boxes.
pub(crate) const FIN_CUP_FILE: usize = 0x9c8158;
