use super::*;

/// Rule: the Polish Cup takes 52 clubs: the Second Division clubs placed 13th to 20th last season, then those placed up
/// to 12th (class 2), then lowest-division clubs (the promoted ones first, then the others sorted by strength with the
/// best ones shuffled) until 36 are in (class 3), then the First Division clubs of last season (class 1); returns 1.
///
/// `team_count` = 52; the list (`exe::crt_malloc` of 0x138 into `cup::TEAM_TABLE`; out of memory: the box of
/// pol_cup.cpp at line 0x142 and −1). Over the first [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`] clubs (the limit read
/// once for the first two passes): a local list of at most 70, first the clubs whose `club::LAST_DIVISION` has the id of
/// `KEY_CLUB_COMP_POLISH_LOWER_DIVISION` and whose division is set and another one (promoted), then those whose division
/// is that same last division (still there); `exe::comp_util_sort_and_shuffle_top_clubs` on the second part only
/// (list + first, n − first, 36 − first). Then, the limit read again each step: the clubs whose last division has the id of
/// `KEY_CLUB_COMP_POLISH_SECOND_DIVISION` and whose `club::LAST_POSITION` (a signed byte) is 13..20, then ≤ 12, each at
/// the next entry with class 2. While fewer than 36 are in, the local list's clubs (up to its count) at the next entries
/// with class 3. Then the clubs whose last division has the id of `KEY_CLUB_COMP_POLISH_FIRST_DIVISION`, while the
/// entry offset is below 0x138, with class 1 (the count is not kept). Status 0 for all. Returns 1.
/// Original: `PolCup::collect_entrants` (0x7c8550, pol_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let mut lower = [0usize; 0x46];
    let mut n: i16 = 0;
    let mut k = 0i32;
    this.set_team_count(0x34);
    let l = exe::crt_malloc(0x138);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0xa78564, 0x142);
        return -1;
    }
    let limit = || (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let count = limit();
    let mut first = 0i32;
    for promoted in [true, false] {
        if !promoted {
            first = n as i32;
        }
        for i in 0..count.max(0) as usize {
            let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
            let (last, d) = (read_usize(c + club::LAST_DIVISION), read_usize(c + club::DIVISION));
            let fits = if promoted { d != 0 && d != last } else { d == last };
            if last != 0 && n < 0x46 && read_usize(last) == read_usize(KEY_CLUB_COMP_POLISH_LOWER_DIVISION) && fits {
                lower[n as usize] = c;
                n += 1;
            }
        }
    }
    covered!("mwd_30 polish cup entrants");
    exe::comp_util_sort_and_shuffle_top_clubs(lower.as_mut_ptr() as usize + 4 * first as usize, (n as i32 - first) as u32, (0x24 - first) as u32); // mwd30: the lowest division clubs shuffled
    for upper in [false, true] {
        let mut i = 0i32;
        while i < limit() {
            let c = read_usize(G_CLUB_TABLE) + i as usize * club::SIZE;
            let last = read_usize(c + club::LAST_DIVISION);
            if last != 0 && read_usize(last) == read_usize(KEY_CLUB_COMP_POLISH_SECOND_DIVISION) {
                let pos = read_i8(c + club::LAST_POSITION);
                if if upper { pos <= 0xc } else { (0xd..0x15).contains(&pos) } {
                    set_cup_entry(this, k as usize, c, 2);
                    k += 1;
                }
            }
            i += 1;
        }
    }
    if k < 0x24 {
        let mut j = 0i32;
        while j < n as i32 {
            set_cup_entry(this, k as usize, lower[j as usize], 3); // mwd30: the lowest division clubs class 3
            k += 1;
            j += 1;
            if k * 6 >= 0xd8 {
                break;
            }
        }
    }
    let mut e = k * 6;
    let mut i = 0i32;
    while i < limit() {
        let c = read_usize(G_CLUB_TABLE) + i as usize * club::SIZE;
        let last = read_usize(c + club::LAST_DIVISION);
        if last != 0 && e < 0x138 && read_usize(last) == read_usize(KEY_CLUB_COMP_POLISH_FIRST_DIVISION) {
            set_cup_entry(this, (e / 6) as usize, c, 1);
            e += 6;
        }
        i += 1;
    }
    1
}

/// Comparer for `pol_cup_collect_entrants` (side by side with the random generator restored, every entry compared).
pub(crate) unsafe extern "thiscall" fn compare_pol_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_POL_CUP_COLLECT_ENTRANTS);
    compare_entrants_with_seeds("pol_cup_collect_entrants", Cup(this), || o(this), || pol_cup_collect_entrants(this), |_| true)
}

/// Rule: the Polish Cup at career creation: discipline rule 0x1f, 7 substitutes and 3 substitutions and, with no saved
/// state, the 52 entrants, a 600,000 prize fund, the seven rounds, the fixture tree, the statistics and the predicted
/// positions (`construct_east_nation_cup`).
/// Original: `PolCup::PolCup` (0x7c7d80, pol_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_pol_cup(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &POLISH_CUP, "mwd_18 polish cup new")
}

/// Rule: the Polish Cup has seven rounds: the preliminary round (8 teams, code 0xa, draw rule 0x403) drawn 6/8,
/// played 9/8 (weekday 2); the first round (32 teams, 28 new from entry 8) drawn 11/8, played 13/9; the second round
/// (32 teams, 16 new from entry 36, draw bits 8) drawn 15/9, played 23/9 (weekday 5); the round of 16 over two legs 14
/// days apart drawn 29/9, played 8/11; the quarter-finals over two legs drawn 3/3, played 14/3 of the next year; the
/// semi-finals over two legs drawn 1/4, played 11/4 (category 6); the final over two legs 7 days apart drawn 19/4,
/// played 16/5, 500,000 to the winner.
/// Original: `PolCup::create_round_table` (0x7c8040, pol_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn pol_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    fra_super_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &POLISH_CUP_ROUNDS, 0xa78564, 0x9d, "mwd_18 polish cup rounds", |_| {})
}

/// Comparer for `construct_pol_cup` (only the original runs).
pub(crate) unsafe extern "thiscall" fn compare_construct_pol_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_POL_CUP);
    compare_east_nation_cup_ctor("construct_pol_cup", Cup(this), year, record, &POLISH_CUP, || o(this, year, record))
}

/// Comparer for `pol_cup_create_round_table` (side by side, the written rounds compared).
pub(crate) unsafe extern "thiscall" fn compare_pol_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_POL_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("pol_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| pol_cup_create_round_table(this, stage, r, e, s))
}

/// Rule: the Polish Cup season turnover, as the Croatian Cup's, with the Polish Cup entrants (`PolCup::collect_entrants`
/// 0x7c8550; lines 0x1de, 0x1e5, 0x1ec of pol_cup.cpp).
/// Original: `PolCup::start_new_season` (0x7c88a0, pol_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x7c8550, 0xa78564, [0x1de, 0x1e5, 0x1ec], 1, "mwd_9 polish cup turnover")
}

/// Comparer for `pol_cup_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_pol_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POL_CUP_START_NEW_SEASON);
    compare_cup_new_season("pol_cup_start_new_season", Cup(this), || o(this))
}

/// The Polish Cup: discipline rule 0x1f (the Polish one), 600,000 prize fund.
pub(crate) const POLISH_CUP: EastNationCup = EastNationCup {
    vtable: 0x96e2c0,
    discipline: 0x1f,
    entrants: 0x7c8550,
    prize: Some(0x927c0), // mwd18: the Polish Cup prize fund
    late_substitutes: false,
    stage_vector: false,
    rank: true,
    file: 0xa78564,
    rows: [0x44, 0x52, 0x59],
};

/// The Polish Cup: seven single-leg rounds from July to April, the quarter-finals and semi-finals over two legs 14
/// days apart and the final over two legs 7 days apart; 500,000 to the winner of the final.
pub(crate) const POLISH_CUP_ROUNDS: [FraSuperCupRound; 7] = [
    cql12r((6, 7, 0, 4), (9, 7, 0, 2, 2, 0), 0xa, (0, 0x403, 0), 4, (8, 4, 8, 0), (0, 1, 0), (0, 0, 0)),
    cql12r((0xb, 7, 0, 4), (0xd, 8, 0, 2, 2, 0), 0x14, (0, 3, 0), 4, (0x20, 0x10, 0x1c, 8), (0, 1, 0), (0, 0, 0)),
    cql12r((0xf, 8, 0, 4), (0x17, 8, 0, 5, 1, 0), 0x1e, (8, 0x403, 0), 4, (0x20, 0x10, 0x10, 0x24), (0, 1, 0), (0, 0, 0)),
    cql12r((0x1d, 8, 0, 4), (8, 0xa, 0, 2, 2, 0), 0x28, (0, 0, 3), 4, (0x10, 8, 0, 0), (0, 2, 0xe), (0, 0, 0)),
    cql12r((3, 2, 1, 4), (0xe, 2, 1, 2, 2, 0), 0x78, (0, 0, 3), 4, (8, 4, 0, 0), (0, 2, 0xe), (0, 0, 0)),
    cql12r((1, 3, 1, 4), (0xb, 3, 1, 2, 2, 0), 0x82, (0, 0, 3), 6, (4, 2, 0, 0), (0, 2, 7), (0, 0, 0)),
    cql12r((0x13, 3, 1, 3), (0x10, 4, 1, 1, 2, 0), 0x96, (0, 0, 3), 6, (2, 1, 0, 0), (0, 2, 7), (0, 0x7a120, 0)),
];
