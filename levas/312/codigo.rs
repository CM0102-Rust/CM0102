// ---------- batch 312: the Finnish Cup (fin_cup.cpp): constructor, round table and entrants ----------

/// The source file of the Finnish Cup's error boxes.
pub(crate) const FIN_CUP_FILE: usize = 0x9c8158;

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

/// Rule: the Finnish Cup at career creation: discipline rule 11, 7 substitutes and 3 substitutions and, with no saved
/// state, the 76 entrants, a 200,000 prize fund, the seven rounds, the fixture tree, the statistics and the predicted
/// positions (`construct_east_nation_cup`, the CroCup template, checked instruction by instruction).
/// Original: `FinCup::FinCup` (0x592250, fin_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_fin_cup(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &FINNISH_CUP, "mwe_18 finnish cup new")
}

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

// ---------- batch 312: the Norwegian Cup (nor_cup.cpp): constructor, round table and entrants ----------

/// The source file of the Norwegian Cup's error boxes.
pub(crate) const NOR_CUP_FILE: usize = 0xa69708;

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

/// Rule: the Norwegian Cup at career creation: discipline rule 22, 7 substitutes and 3 substitutions and, with no saved
/// state, the 128 entrants, an 800,000 prize fund, the seven rounds, the fixture tree, the statistics and the predicted
/// positions (`construct_east_nation_cup`, the CroCup template, checked instruction by instruction).
/// Original: `NorCup::NorCup` (0x78f020, nor_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_nor_cup(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &NORWEGIAN_CUP, "mwe_19 norwegian cup new")
}

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

// ---------- batch 312: the Portuguese Cup (por_cup.cpp): constructor, round table and entrants ----------

/// The source file of the Portuguese Cup's error boxes.
pub(crate) const POR_CUP_FILE: usize = 0xa786f0;

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

/// Rule: the Portuguese Cup at career creation: discipline rule 23 and, with no saved state, the 164 entrants, an
/// 800,000 prize fund, the eight rounds, the fixture tree, the statistics and the predicted positions
/// (`construct_british_cup`, the WelCup template, checked instruction by instruction: it does not write the bench).
/// Original: `PorCup::PorCup` (0x7cd980, por_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_por_cup(this: usize, year: u32, record: usize) -> usize {
    construct_british_cup(Cup(this), year, record, &PORTUGUESE_CUP, "mwe_20 portuguese cup new")
}

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

/// Whether a club takes a place in the lower part of the Portuguese Cup: club_get_linked_b_team
/// (`exe::club_get_linked_b_team`(club, &parent, 1), cdecl) left out the B teams it links to no club (nonzero with a null
/// parent); `parent` keeps its value from one call to the next, as the original's local.
unsafe fn portuguese_cup_takes(club: usize, parent: &mut u32) -> bool {
    exe::club_get_linked_b_team(club, parent, 1) == 0 || *parent != 0
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

// ---------- batch 312: the Portuguese Super Cup constructor (por_super.cpp) ----------

pub(crate) const POR_SUPER_CUP_PARAMS: NationCup = NationCup {
    vtable: 0x96e90c,
    bench: None,
    discipline: discipline_rule::PORTUGAL, // mwe21: the Portuguese Super Cup discipline rule
    entrants: 0x7d3260,
    zero_fails: false,
    predicted: false,
    source_file: 0xa787d4,
    box_lines: [0x3f, 0x4d, 0x54],
};

/// Rule: the Portuguese Super Cup (Supertaça) is born with discipline rule 23; with no saved state, its two clubs
/// (PorSuper::select_teams), the round table, the tie tree (a failure is a negative return) and the statistics, with no
/// predicted positions (`construct_nation_cup`, the HolSuper body checked instruction by instruction: only the
/// discipline rule, the vtable, the clubs routine and the file change).
/// Original: `PorSuper::PorSuper` (0x7d2e40, por_super.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_por_super(this: usize, year: u32, record: usize) -> usize {
    construct_nation_cup(Cup(this), year, record, &POR_SUPER_CUP_PARAMS)
}

/// Comparer for `construct_por_super`: only the original runs (`compare_nation_cup_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_por_super(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_POR_SUPER);
    compare_nation_cup_ctor("construct_por_super", Cup(this), year, record, &POR_SUPER_CUP_PARAMS, || o(this, year, record))
}
