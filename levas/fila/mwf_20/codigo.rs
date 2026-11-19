// ---------- batch %%LEVA%%: the Korean FA Cup (kor_fa_cup.cpp): constructor, round table and entrants ----------

/// The source file of the Korean FA Cup's error boxes.
pub(crate) const KOR_FA_CUP_FILE: usize = 0x9eb0e8;

/// The Korean FA Cup: the Cup bench defaults (not written), discipline rule 42 (Korea), a prize fund of 200,000.
pub(crate) const KOREAN_FA_CUP: BritishCupSpec = BritishCupSpec {
    vtable: 0x96cd30,
    bench: None,
    discipline: discipline_rule::KOREA,
    prize_fund: 0x30d40,
    entrants: 0x66aaf0,
    file: KOR_FA_CUP_FILE,
    rows: [0x45, 0x53, 0x5a],
};

/// Rule: the Korean FA Cup at career creation: discipline rule 42 and, with no saved state, the 32 entrants, a 200,000
/// prize fund, the five rounds, the fixture tree, the statistics and the predicted positions (`construct_british_cup`,
/// the WelCup template, checked instruction by instruction: it does not write the bench).
/// Original: `KorFaCup::KorFaCup` (0x66a460, kor_fa_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_kor_fa_cup(this: usize, year: u32, record: usize) -> usize {
    construct_british_cup(Cup(this), year, record, &KOREAN_FA_CUP, "mwf_20 korean fa cup new")
}

/// The Korean FA Cup: five single-match rounds in October and November; no prize per round won (−1 = a share of the
/// fund).
pub(crate) const KOREAN_FA_CUP_ROUNDS: [FraSuperCupRound; 5] = [
    cql12r((0x15, 9, 0, 0), (0x1e, 9, 0, 2, 2, 0), 0x1e, (0, 0x83, 0), 4, (0x20, 0x10, 0x20, 0), (0, 1, 3), (0, 0xffffffff, 0)), // mwf20: the first round
    cql12r((0x1f, 9, 0, 3), (3, 0xa, 0, 5, 1, 0), 0x28, (0, 0x83, 0), 4, (0x10, 8, 0, 0), (0, 1, 8), (0, 0xffffffff, 0)),
    cql12r((4, 0xa, 0, 6), (7, 0xa, 0, 2, 2, 0), 0x78, (0, 0x83, 3), 6, (8, 4, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((8, 0xa, 0, 3), (0x11, 0xa, 0, 5, 1, 0), 0x82, (0, 0x83, 3), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((0x12, 0xa, 0, 6), (0x19, 0xa, 0, 6, 1, 4), 0x96, (0, 0x83, 0), 6, (2, 1, 0, 0), (0, 1, 7), (0, 0xffffffff, 0)),
];

/// Rule: the Korean FA Cup has five single-match rounds, tie rules 0x83: the first round (32 teams, all entering) drawn
/// 21/10 and played 30/10; the round of 16 drawn 31/10 and played 3/11; the quarter-finals (category 6) drawn 4/11 and
/// played 7/11; the semi-finals drawn 8/11 and played 17/11; the final drawn 18/11 and played 25/11 at venue 4; no
/// prize per round (a share of the fund). Outside stage −1 it writes nothing and returns 0; out of memory, the box at
/// line 0x9e and 0 (`fra_super_create_single_stage_round_table`).
/// Original: `KorFaCup::create_round_table` (0x66a720, kor_fa_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn kor_fa_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    fra_super_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &KOREAN_FA_CUP_ROUNDS, KOR_FA_CUP_FILE, 0x9e, "mwf_20 korean fa cup rounds", |_| {})
}

/// Rule: the Korean FA Cup takes 32 clubs: 22 clubs of the competition's nation (the record's `competition::NATION`)
/// outside the K League (`KEY_CLUB_COMP_KOREAN_LEAGUE`), up to 100 found among the first [`G_CLUB_COUNT`] − 2 ×
/// [`G_NATION_COUNT`] clubs, sorted by reputation and shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n,
/// 22), cdecl), then the K League clubs while the list has room. Seed (+4) and status (+5) 0.
///
/// `team_count` = 32; the list (`exe::crt_malloc` of 32 × 6 into `cup::TEAM_TABLE`; out of memory: the box at line
/// 0x11c and −1). Returns 1 when the list is full, otherwise 0. Defect of the original: the 22 are copied even when
/// fewer were found (the rest is stack garbage; here, null clubs).
/// Original: `KorFaCup::collect_entrants` (0x66aaf0, kor_fa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_fa_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    this.set_team_count(0x20);
    let l = exe::crt_malloc(0xc0);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(KOR_FA_CUP_FILE, 0x11c);
        return -1;
    }
    let nation = read_usize(read_usize(this.addr() + 4) + competition::NATION);
    let mut pool = [0usize; 100];
    let mut n: i8 = 0;
    let limit = (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < limit {
        let cn = read_usize(club + club::NATION);
        let d = read_usize(club + club::DIVISION);
        if cn != 0 && !(d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_KOREAN_LEAGUE)) && cn == nation && n < 0x64 {
            pool[n as usize] = club;
            n += 1;
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    exe::comp_util_sort_and_shuffle_top_clubs(pool.as_mut_ptr() as usize, n as i16 as u16 as u32, 0x16); // mwf20: the 22 non-league clubs drawn
    for k in 0..0x16 {
        write_usize(this.team_table() + 6 * k, pool[k]);
        write_u8(this.team_table() + 6 * k + 4, 0);
        write_u8(this.team_table() + 6 * k + 5, 0);
    }
    covered!("mwf_20 korean fa cup entrants");
    let mut k: i32 = 0x16;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_KOREAN_LEAGUE) && k < this.team_count() as i32 {
            write_usize(this.team_table() + 6 * k as usize, club);
            write_u8(this.team_table() + 6 * k as usize + 4, 0); // mwf20: the K League seed
            write_u8(this.team_table() + 6 * k as usize + 5, 0);
            k += 1;
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    (k == this.team_count() as i32) as i32
}

/// Comparer for `construct_kor_fa_cup` (only the original runs; then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_kor_fa_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_KOR_FA_CUP);
    compare_british_cup_ctor("construct_kor_fa_cup", Cup(this), year, record, &KOREAN_FA_CUP, || o(this, year, record))
}

/// Comparer for `kor_fa_cup_create_round_table` (side by side, the written rounds compared).
pub(crate) unsafe extern "thiscall" fn compare_kor_fa_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_KOR_FA_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("kor_fa_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| kor_fa_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `kor_fa_cup_collect_entrants` (side by side with the random generator restored: the club, seed and
/// status of every entry).
pub(crate) unsafe extern "thiscall" fn compare_kor_fa_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_KOR_FA_CUP_COLLECT_ENTRANTS);
    compare_entrants_with_seeds("kor_fa_cup_collect_entrants", Cup(this), || o(this), || kor_fa_cup_collect_entrants(this), |_| true)
}
