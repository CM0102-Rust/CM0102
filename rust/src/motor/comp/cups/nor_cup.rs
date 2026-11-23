use super::*;

/// Rule: the Norwegian Cup at career creation: discipline rule 22, 7 substitutes and 3 substitutions and, with no saved
/// state, the 128 entrants, an 800,000 prize fund, the seven rounds, the fixture tree, the statistics and the predicted
/// positions (`construct_east_nation_cup`, the CroCup template, checked instruction by instruction).
/// Original: `NorCup::NorCup` (0x78f020, nor_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_nor_cup(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &NORWEGIAN_CUP, "mwe_19 norwegian cup new")
}

/// Rule: the Norwegian Cup has seven rounds: the first round (128 teams, code 0xa, tie rules 0x403) drawn 7/4 and
/// played 5/5; the second round (64) drawn 6/5 and played 26/5; the third round (32) drawn 27/5 and played 2/6; the
/// fourth round (16, a replay on a draw, tie rules 2 and 3) drawn 3/6 and played 30/6; the quarter-finals (category 6)
/// drawn 1/7 and played 4/8; the semi-finals drawn 5/8 and played 25/9 (3 days between the matches); the final drawn
/// 29/9, played on 24 or 31 October (`norwegian_cup_final_day`) at venue 4, with a replay 3 days later; no prize per
/// round (a share of the fund). Outside stage −1 it writes nothing and returns 0; out of memory, the box at line 0xa5
/// and 0 (`fra_super_create_single_stage_round_table`; the Date work runs before the allocation here, after it in the
/// original: it only reads).
/// Original: `NorCup::create_round_table` (0x78f2e0, nor_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn nor_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let this = Cup(this);
    let mut table = NORWEGIAN_CUP_ROUNDS;
    if stage as u8 == 0xff {
        table[6].game.0 = norwegian_cup_final_day(this);
    }
    fra_super_create_single_stage_round_table(this, stage, rounds, extra, out, &table, NOR_CUP_FILE, 0xa5, "mwe_19 norwegian cup rounds", |_| {})
}

/// Rule: the Norwegian Cup takes 128 clubs, all without a "2" in the short name (the reserve sides): first the Premier
/// Division (seed 1), First Division and the eight Second Division groups (seed 0) clubs that have a nation, then the
/// Third Division clubs (`KEY_CLUB_COMP_NORWEGIAN_THIRD_DIVISION`) with a nation and last, among the first
/// [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`] clubs, the Norwegian clubs (`KEY_NATION_NORWAY`) without a division, each
/// while the list has room. A club whose division is the cup's own competition record (+4) has its division cleared
/// first [inference: a repair of bad data]. Status (+5) 0.
///
/// `team_count` = 128; the list (`exe::crt_malloc` of 128 × 6 into `cup::TEAM_TABLE`; out of memory: the box at line
/// 0x14a and −1); the "2" test is `exe::crt_strstr`(short name, "2") (cdecl). Returns 1 when the list is full,
/// otherwise 0.
/// Original: `NorCup::collect_entrants` (0x78f890, nor_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    this.set_team_count(0x80);
    let l = exe::crt_malloc(0x300);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(NOR_CUP_FILE, 0x14a);
        return -1;
    }
    const TWO: usize = 0xa6973c; // "2"
    let leagues = [KEY_CLUB_COMP_NORWEGIAN_SECOND_DIVISION_GROUP_1, KEY_CLUB_COMP_NORWEGIAN_SECOND_DIVISION_GROUP_2, KEY_CLUB_COMP_NORWEGIAN_SECOND_DIVISION_GROUP_3,
        KEY_CLUB_COMP_NORWEGIAN_SECOND_DIVISION_GROUP_4, KEY_CLUB_COMP_NORWEGIAN_SECOND_DIVISION_GROUP_5, KEY_CLUB_COMP_NORWEGIAN_SECOND_DIVISION_GROUP_6,
        KEY_CLUB_COMP_NORWEGIAN_SECOND_DIVISION_GROUP_7, KEY_CLUB_COMP_NORWEGIAN_SECOND_DIVISION_GROUP_8, KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION,
        KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION];
    let mut k: i32 = 0;
    let add = |k: &mut i32, club: usize, seed: u8| {
        write_usize(this.team_table() + 6 * *k as usize, club);
        write_u8(this.team_table() + 6 * *k as usize + 4, seed);
        write_u8(this.team_table() + 6 * *k as usize + 5, 0);
        *k += 1;
    };
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if read_usize(club + club::DIVISION) == read_usize(this.addr() + 4) {
            write_usize(club + club::DIVISION, 0);
        }
        let d = read_usize(club + club::DIVISION);
        if d != 0 && leagues.iter().any(|&key| read_usize(d) == read_usize(key)) && k < this.team_count() as i32 && read_usize(club + club::NATION) != 0
            && exe::crt_strstr(club + club::SHORT_NAME, TWO) == 0 { // mwe19: the reserve sides left out
            let premier = read_usize(read_usize(club + club::DIVISION)) == read_usize(KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION);
            add(&mut k, club, premier as u8); // mwe19: the Premier Division seed
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    covered!("mwe_19 norwegian cup entrants");
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 && k < this.team_count() as i32 {
        let d = read_usize(club + club::DIVISION);
        if read_usize(club + club::NATION) != 0 && d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_NORWEGIAN_THIRD_DIVISION)
            && exe::crt_strstr(club + club::SHORT_NAME, TWO) == 0 {
            add(&mut k, club, 0);
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    let limit = || (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < limit() && k < this.team_count() as i32 {
        let n = read_usize(club + club::NATION);
        if n != 0 && read_usize(n) == read_usize(KEY_NATION_NORWAY) && read_usize(club + club::DIVISION) == 0 && exe::crt_strstr(club + club::SHORT_NAME, TWO) == 0 {
            add(&mut k, club, 0); // mwe19: the Norwegian clubs without a division
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    (k == this.team_count() as i32) as i32
}

/// Comparer for `construct_nor_cup` (only the original runs; the fields checked).
pub(crate) unsafe extern "thiscall" fn compare_construct_nor_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_NOR_CUP);
    compare_east_nation_cup_ctor("construct_nor_cup", Cup(this), year, record, &NORWEGIAN_CUP, || o(this, year, record))
}

/// Comparer for `nor_cup_create_round_table` (side by side, the written rounds compared).
pub(crate) unsafe extern "thiscall" fn compare_nor_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_NOR_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("nor_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| nor_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `nor_cup_collect_entrants` (side by side: the club, seed and status of every entry).
pub(crate) unsafe extern "thiscall" fn compare_nor_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_NOR_CUP_COLLECT_ENTRANTS);
    compare_entrants_with_seeds("nor_cup_collect_entrants", Cup(this), || o(this), || nor_cup_collect_entrants(this), |_| true)
}

/// The Norwegian Cup turnover: the same as the Finnish Cup's, with NorCup::collect_entrants (0x78f890), nor_cup.cpp
/// (0xa69708) and the box lines 0x1e8, 0x1ef, 0x1f6.
/// Original: `NorCup::start_new_season` (0x78fb90, nor_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x78f890, 0xa69708, [0x1e8, 0x1ef, 0x1f6], 1, "mwe_16 norwegian cup turnover") // mwe16: the Norwegian Cup next year
}

/// Comparer for `nor_cup_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_nor_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NOR_CUP_START_NEW_SEASON);
    compare_cup_new_season("nor_cup_start_new_season", Cup(this), || o(this))
}

/// The day of the Norwegian Cup final: 31 October moved to weekday 6 (Date::Date `exe::date_date_549f70`(date, 0x1f, 9,
/// year, 6): lea ecx, local; 4 pushes -> thiscall, ret 0x10) is compared with 1 November (Date::Date(1, 0xa, year, −1));
/// when 1 November comes first (by the word at +2, unsigned, then the word at +0, signed) the final is on the 24th
/// (0x18), otherwise on the 31st (0x1f) [inference: the final stays in October].
pub(crate) unsafe fn norwegian_cup_final_day(this: Cup) -> u32 {
    let year = this.year() as i32 as u32;
    let mut first = [0u16; 4];
    let mut snapped = [0u16; 4];
    exe::date_date_549f70(&mut first, 1, 0xa, year, 0xffff_ffff);
    exe::date_date_549f70(&mut snapped, 0x1f, 9, year, 6);
    let earlier = if first[1] != snapped[1] { first[1] < snapped[1] } else { (first[0] as i16) < (snapped[0] as i16) };
    if earlier { 0x18 } else { 0x1f } // mwe19: the final's day
}

/// The Norwegian Cup: discipline rule 22 (RbNorwayCup), a prize fund of 800,000.
pub(crate) const NORWEGIAN_CUP: EastNationCup = EastNationCup {
    vtable: 0x96dd80,
    discipline: discipline_rule::NORWAY_CUP,
    entrants: 0x78f890,
    prize: Some(0xc3500),
    late_substitutes: false,
    stage_vector: false,
    rank: true,
    file: NOR_CUP_FILE,
    rows: [0x4a, 0x58, 0x5f],
};

/// The Norwegian Cup: seven rounds from April to October; no prize per round won (−1 = a share of the fund). The final's
/// match day is set at run time (`norwegian_cup_final_day`).
pub(crate) const NORWEGIAN_CUP_ROUNDS: [FraSuperCupRound; 7] = [
    cql12r((7, 3, 0, 2), (5, 4, 0, 2, 2, 0), 0xa, (0, 0x403, 0), 4, (0x80, 0x40, 0x80, 0), (0, 1, 0), (0, 0xffffffff, 0)), // mwe19: the first round
    cql12r((6, 4, 0, 3), (0x1a, 4, 0, 2, 2, 0), 0x14, (0, 0x403, 0), 4, (0x40, 0x20, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((0x1b, 4, 0, 3), (2, 5, 0, 2, 2, 0), 0x1e, (0, 3, 0), 4, (0x20, 0x10, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((3, 5, 0, 3), (0x1e, 5, 0, 2, 2, 0), 0x28, (0, 2, 3), 4, (0x10, 8, 0, 0), (1, 0, 0), (0, 0xffffffff, 0)),
    cql12r((1, 6, 0, 3), (4, 7, 0, 2, 2, 0), 0x78, (0, 2, 3), 6, (8, 4, 0, 0), (1, 1, 0), (0, 0xffffffff, 0)),
    cql12r((5, 7, 0, 3), (0x19, 8, 0, 5, 1, 0), 0x82, (0, 2, 3), 6, (4, 2, 0, 0), (1, 1, 3), (0, 0xffffffff, 0)),
    cql12r((0x1d, 8, 0, 2), (0x1f, 9, 0, 6, 1, 4), 0x96, (0, 2, 3), 6, (2, 1, 0, 0), (1, 0, 3), (0, 0xffffffff, 0)),
];

/// The source file of the Norwegian Cup's error boxes.
pub(crate) const NOR_CUP_FILE: usize = 0xa69708;
