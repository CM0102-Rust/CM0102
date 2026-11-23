use super::*;

/// Rule: the Russian Cup takes 48 clubs: the First Division clubs (class 2; the cup holder among them class 1) from
/// entry 31 backwards, then the second division zone clubs (sorted by strength, the best 42 shuffled) until 32 are in,
/// class 2; then the Premier League clubs (class 1) from entry 32 on; returns 1 when the list is full.
///
/// `team_count` = 48; the list (`exe::crt_malloc` of 0x120; out of memory: the box at line 0x148 and −1). The cup holder
/// is the latest winner of this competition (comp_util_get_latest_placed_club(+4, 0)). The clubs looked at are the first
/// [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`]. The zone clubs (division pointer = the record of one of the six zones,
/// `RUSSIAN_ZONE_PAIRS`) into a local list of at most 100 (a signed byte count), sorted and shuffled
/// (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 14)). Each First Division club (division pointer = its record):
/// k + 1 → entry 32 − k, class 2 (1 for the holder). Then up to 14 zone clubs, while k < 32, the same way. Then each
/// Premier League club in entry k, k + 1, ... with class 1. Returns k ≥ `team_count`.
/// Original: `RusCup::select_entrants` (0x7e99a0, rus_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_cup_select_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let zones: [[usize; 2]; 3] = RUSSIAN_ZONE_PAIRS.map(|p| p.map(|g| unsafe { comp_record_of(g) }));
    let (first, premier) = (comp_record_of(KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION), comp_record_of(KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION));
    let mut zone_clubs = [0usize; 100];
    let mut nz: i8 = 0;
    let mut k: i8 = 0;
    this.set_team_count(0x30);
    let l = exe::crt_malloc(0x120);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0xa7ac58, 0x148);
        return -1;
    }
    let holder = exe::comp_util_get_latest_placed_club(read_usize(this.addr() + 4), 0);
    let limit = || (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let count = limit();
    for i in 0..count.max(0) as usize {
        let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        let d = read_usize(c + club::DIVISION);
        if zones.iter().any(|p| d == p[0] || d == p[1]) && nz < 0x64 {
            zone_clubs[nz as usize] = c;
            nz += 1;
        }
    }
    exe::comp_util_sort_and_shuffle_top_clubs(zone_clubs.as_ptr() as usize, nz as i16 as u16 as u32, 0xe); // mwd27: the best 42 zone clubs shuffled
    let mut i = 0i32;
    while i < limit() {
        let c = read_usize(G_CLUB_TABLE) + i as usize * club::SIZE;
        if read_usize(c + club::DIVISION) == first {
            k += 1;
            set_cup_entry(this, (0x20 - k as i32) as usize, c, if c == holder { 1 } else { 2 });
        }
        i += 1;
    }
    covered!("mwd_27 russian cup entrants");
    for &c in zone_clubs.iter().take(0xe) {
        if k >= 0x20 {
            break;
        }
        k += 1;
        set_cup_entry(this, (0x20 - k as i32) as usize, c, 2);
    }
    let mut i = 0i32;
    while i < limit() {
        let c = read_usize(G_CLUB_TABLE) + i as usize * club::SIZE;
        if read_usize(c + club::DIVISION) == premier {
            set_cup_entry(this, k as usize, c, 1);
            k = k.wrapping_add(1);
        }
        i += 1;
    }
    (k as i16 >= this.team_count()) as i32
}

/// Comparer for `rus_cup_select_entrants` (side by side with the random generator restored, every entry compared).
pub(crate) unsafe extern "thiscall" fn compare_rus_cup_select_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_RUS_CUP_SELECT_ENTRANTS);
    compare_entrants_with_seeds("rus_cup_select_entrants", Cup(this), || o(this), || rus_cup_select_entrants(this), |_| true)
}

/// Rule: the Russian Cup at career creation: discipline rule 0x27, 7 substitutes and 3 substitutions and, with no saved
/// state, no stage vector (+0x2c is 0), the 48 entrants, an 800,000 prize fund, the six rounds, the fixture tree, the
/// statistics and the predicted positions (`construct_east_nation_cup`).
/// Original: `RusCup::RusCup` (0x7e9210, rus_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_rus_cup(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &RUSSIAN_CUP, "mwd_19 russian cup new")
}

/// Rule: the Russian Cup has six single-leg rounds: the first round (32 teams, code 0x14) drawn 23/6, played 30/6; the
/// second round (32 teams, 16 new from entry 32: the Premier League [inference]; draw bits 8, rule 0x403) drawn 10/7,
/// played 12/10 (weekday 1); the round of 16 drawn 13/10, played 13/11 (weekday 5); the quarter-finals (category 6)
/// drawn 14/11, played 4/4 of the next year; the semi-finals drawn 5/4, played 3/5; the final drawn 4/5, played 21/5
/// at venue 4 [inference: a neutral ground]; a share of the fund per round won.
/// Original: `RusCup::create_round_table` (0x7e9530, rus_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rus_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    fra_super_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &RUSSIAN_CUP_ROUNDS, 0xa7ac58, 0xb1, "mwd_19 russian cup rounds", |_| {})
}

/// Comparer for `construct_rus_cup` (only the original runs).
pub(crate) unsafe extern "thiscall" fn compare_construct_rus_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_RUS_CUP);
    compare_east_nation_cup_ctor("construct_rus_cup", Cup(this), year, record, &RUSSIAN_CUP, || o(this, year, record))
}

/// Comparer for `rus_cup_create_round_table` (side by side, the written rounds compared).
pub(crate) unsafe extern "thiscall" fn compare_rus_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_RUS_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("rus_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| rus_cup_create_round_table(this, stage, r, e, s))
}

/// Rule: the Russian Cup season turnover: last edition's tables freed, its statistics archived, the year + 1, the 48
/// entrants (`RusCup::select_entrants` 0x7e99a0), the rounds, the fixture tree and the predicted positions
/// (`start_new_cup_season`; lines 0x1ce, 0x1d5, 0x1dc of rus_cup.cpp).
/// Original: `RusCup::start_new_season` (0x7e9d10, rus_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x7e99a0, 0xa7ac58, [0x1ce, 0x1d5, 0x1dc], 1, "mwd_10 russian cup turnover")
}

/// Comparer for `rus_cup_start_new_season` (only the original; the year went up by one).
pub(crate) unsafe extern "thiscall" fn compare_rus_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_RUS_CUP_START_NEW_SEASON);
    compare_cup_new_season("rus_cup_start_new_season", Cup(this), || o(this))
}

/// The Russian Cup: discipline rule 0x27, 800,000 prize fund, the stage vector allocated from +0x2c.
pub(crate) const RUSSIAN_CUP: EastNationCup = EastNationCup {
    vtable: 0x96f124,
    discipline: 0x27,
    entrants: 0x7e99a0,
    prize: Some(0xc3500),
    late_substitutes: false,
    stage_vector: true,
    rank: true,
    file: 0xa7ac58,
    rows: [0x4e, 0x5c, 0x63],
};

/// The Russian Cup: six single-leg rounds from June to May, a share of the fund per round won.
pub(crate) const RUSSIAN_CUP_ROUNDS: [FraSuperCupRound; 6] = [
    cql12r((0x17, 5, 0, 2), (0x1e, 5, 0, 2, 2, 0), 0x14, (0, 3, 0), 4, (0x20, 0x10, 0x20, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((0xa, 6, 0, 5), (0xc, 9, 0, 1, 2, 0), 0x1e, (8, 0x403, 0), 4, (0x20, 0x10, 0x10, 0x20), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((0xd, 9, 0, 2), (0xd, 0xa, 0, 5, 1, 0), 0x28, (0, 0x403, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((0xe, 0xa, 0, 6), (4, 3, 1, 1, 2, 0), 0x78, (0, 0x403, 0), 6, (8, 4, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((5, 3, 1, 2), (3, 4, 1, 2, 2, 0), 0x82, (0, 0x403, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((4, 4, 1, 3), (0x15, 4, 1, 1, 2, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
];
