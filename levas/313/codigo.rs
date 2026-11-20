// ---------- batch 313: the Korean FA Cup (kor_fa_cup.cpp): constructor, round table and entrants ----------

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

// ---------- batch 313: the Korean Super Cup (kor_super_cup.cpp): constructor, round table and clubs ----------

/// The source file of the Korean Super Cup's error boxes.
pub(crate) const KOR_SUPER_CUP_FILE: usize = 0x9eb198;

/// The Korean Super Cup: 5 substitutes named and 3 substitutions, discipline rule 42 (Korea), no predicted positions.
pub(crate) const KOR_SUPER_CUP_PARAMS: NationCup = NationCup {
    vtable: 0x96cf38,
    bench: Some((5, 3)), // mwf21: the Korean Super Cup bench
    discipline: discipline_rule::KOREA,
    entrants: 0x66dc10,
    zero_fails: false,
    predicted: false,
    source_file: KOR_SUPER_CUP_FILE,
    box_lines: [0x40, 0x4e, 0x55],
};

/// Rule: the Korean Super Cup is born with discipline rule 42 and the bench (5 substitutes named, 3 substitutions); with
/// no saved state, its two clubs (KorSuperCup::create_table), the round table, the tie tree (a failure is a negative
/// return), the statistics and no predicted positions (`construct_nation_cup`, checked instruction by instruction: the
/// FraSuper/HolSuper body with the bench written).
/// Original: `KorSuperCup::KorSuperCup` (0x66d820, kor_super_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_kor_super_cup(this: usize, year: u32, record: usize) -> usize {
    construct_nation_cup(Cup(this), year, record, &KOR_SUPER_CUP_PARAMS)
}

/// The Korean Super Cup: one match in March, no prize.
pub(crate) const KOREAN_SUPER_CUP_ROUNDS: [FraSuperCupRound; 1] = [
    cql12r((1, 1, 0, 5), (0xa, 2, 0, 6, 1, 0), 0, (0, 0x83, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mwf21: the Korean Super Cup match
];

/// Rule: the Korean Super Cup is a single match between its 2 clubs (tie rules 0x83), drawn on 1 February (weekday 5)
/// and played on 10 March (moved to weekday 6) at venue 0, category 6; no prize money. Outside stage −1 it writes
/// nothing and returns 0; out of memory, the box at line 0x96 and 0 (`fra_super_create_single_stage_round_table`). The
/// original also writes 0 in the round's +0x11 word [inference: a field the other cups leave to the round's date].
/// Original: `KorSuperCup::create_round_table` (0x66dad0, kor_super_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn kor_super_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let t = fra_super_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &KOREAN_SUPER_CUP_ROUNDS, KOR_SUPER_CUP_FILE, 0x96, "mwf_21 korean super cup rounds", |_| {});
    if t != 0 {
        write_u16(t + 0x11, 0);
    }
    t
}

/// Rule: the Korean Super Cup is between the K League champion (`KEY_CLUB_COMP_KOREAN_LEAGUE`) and the Korean FA Cup
/// winner (`KEY_CLUB_COMP_KOREAN_FA_CUP`); a missing one is replaced by the first club of the nation; if the same club
/// won both, the FA Cup runner-up comes in; if there is none, the error box (`select_league_and_cup_winners`, the
/// GreSuper/PolSuper body checked instruction by instruction, lines 0xc1 and 0xef). Returns 1, 0 without a second club
/// and −1 out of memory.
/// Original: `KorSuperCup::create_table` (0x66dc10, kor_super_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_super_cup_create_table(this: usize) -> i32 {
    korean_super_cup_clubs(Cup(this)).0
}

/// The body of `kor_super_cup_create_table` (and of its comparer's copy).
pub(crate) unsafe fn korean_super_cup_clubs(this: Cup) -> (i32, i32) {
    covered!("mwf_21 korean super cup clubs");
    select_league_and_cup_winners(this, KEY_CLUB_COMP_KOREAN_LEAGUE, KEY_CLUB_COMP_KOREAN_FA_CUP, KEY_CLUB_COMP_KOREAN_FA_CUP, KOR_SUPER_CUP_FILE, [0xc1, 0xef]) // mwf21: champion and cup winner
}

/// Comparer for `construct_kor_super_cup`: only the original runs (`compare_nation_cup_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_kor_super_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_KOR_SUPER_CUP);
    compare_nation_cup_ctor("construct_kor_super_cup", Cup(this), year, record, &KOR_SUPER_CUP_PARAMS, || o(this, year, record))
}

/// Comparer for `kor_super_cup_create_round_table` (side by side, the written rounds compared).
pub(crate) unsafe extern "thiscall" fn compare_kor_super_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_KOR_SUPER_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("kor_super_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| kor_super_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `kor_super_cup_create_table` (side by side: `compare_team_list`).
pub(crate) unsafe extern "thiscall" fn compare_kor_super_cup_create_table(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_KOR_SUPER_CUP_CREATE_TABLE);
    compare_team_list("kor_super_cup_create_table", Cup(this), || o(this), || korean_super_cup_clubs(Cup(this)))
}

// ---------- batch 313: the constructors of the K League and the Korean League Cup (kor_league.cpp, kor_league_cup.cpp) ----------

pub(crate) const KOR_LEAGUE_DIVISION: NationDivision = NationDivision {
    vtable: 0x96cdd0,
    bench: None,
    discipline: discipline_rule::KOREA, // mwf22: the K League discipline rule
    stages: 0,
    set_up_rules: 0x66bcc0,
    create_table: 0x66bba0,
    source_file: 0x9eb120,
    box_lines: [0x47, 0x4e, 0x55, 0x5e, 0x65],
};

/// Rule: the K League at career creation: discipline rule 42, only the league (no stages); with no saved state, the
/// rules, the table with its ten clubs, the fixtures, the order, the statistics and the predicted positions
/// (`construct_nation_division`, the GerFirst body checked instruction by instruction: only the discipline rule, the
/// vtable, the rules and table routines and the file change).
/// Original: `KorLeague::KorLeague` (0x66ae70, kor_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_kor_league(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &KOR_LEAGUE_DIVISION, || {})
}

/// Comparer for `construct_kor_league`: only the original runs (`compare_nation_division_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_kor_league(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_KOR_LEAGUE);
    compare_nation_division_ctor("construct_kor_league", League(this), year, record, &KOR_LEAGUE_DIVISION, || o(this, year, record))
}

/// The source file of the Korean League Cup's error boxes.
pub(crate) const KOR_LEAGUE_CUP_FILE: usize = 0x9eb15c;

/// The fixed fields the Korean League Cup constructor writes on a new cup (offset, width, value).
const KOREAN_LEAGUE_CUP_FIELDS: [(usize, u8, u32); 7] = [(league::STAGE_COUNT, 4, 2), (league::STAGE_INDEX, 1, 0xff), (league::SUBSTITUTES_NAMED, 1, 5), (league::SUBSTITUTIONS_ALLOWED, 1, 3),
    (league::DISCIPLINE_RULE, 1, discipline_rule::KOREA as u32), (league::IMPORTANCE, 1, 0xc), (league::LEAGUE_FLAGS, 2, 8)];

/// Rule: the Korean League Cup at career creation: the year and the database competition, 5 substitutes on the bench
/// and 3 substitutions, discipline rule 42, importance 12; with nothing saved, flags 8, two stages (group B and the
/// knockout), the ten clubs, the statistics, the predicted positions, the rules, group A's table (the cup is itself the
/// league of group A), its fixtures and order, and group B's stage.
///
/// The ItaCCup/IreLgeCup body (`construct_irish_league_cup`), checked instruction by instruction: over League::League
/// (`exe::new_league`, thiscall, ret 0) the record, the vtable 0x96ce84, the year, the bench, the discipline rule and
/// importance 0xc; the saved state (`exe::league_load_from_temp_file`(this, 1); nonzero: returns this). Otherwise
/// `league::LEAGUE_FLAGS` = 8 (0x28 in the Irish and Italian cups), the main stage, current stage −1, two stages and
/// their array (`exe::crt_malloc` of 8, unchecked); KorLeagueCup::select_teams (0x66cf20, unchecked); the statistics
/// (`ita_cups_create_comp_stats`; the box at line 0x53); KorLeagueCup::set_predicted_positions (0x66d3a0, unchecked);
/// KorLeagueCup::set_up_rules (0x66c870; 0: line 0x5d); KorLeagueCup::create_group_a_table (0x66c770; 0: line 0x64);
/// the fixtures (`exe::league_create_fixtures`; 0: line 0x6a); the order (`exe::league_sort_table`(this, 0); 0: line
/// 0x71); KorLeagueCup::create_group_b_stage(0) (0x66cb60: push 0; mov ecx, this -> thiscall, ret 4). Every routine
/// called is thiscall, ret 0 unless said. Each box returns this. Returns this.
/// Original: `KorLeagueCup::KorLeagueCup` (0x66bf00, kor_league_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_kor_league_cup(this: usize, year: u32, record: usize) -> usize {
    let this = League(this);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), 0x96ce84);
    this.set_year(year as u16 as i16);
    this.set_substitutions_allowed(3);
    this.set_substitutes_named(5);
    this.set_discipline_rule(discipline_rule::KOREA);
    this.set_importance(0xc);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!("mwf_22 korean league cup new");
    this.set_league_flags(8); // mwf22: the Korean League Cup flags
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    this.set_stage_count(2);
    this.set_stages(exe::crt_malloc(8));
    let call = |f: usize| {
        let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(f);
        f(this.addr())
    };
    call(0x66cf20);
    if !ita_cups_create_comp_stats(this) {
        show_error_at(KOR_LEAGUE_CUP_FILE, 0x53);
        return this.addr();
    }
    call(0x66d3a0);
    if call(0x66c870) == 0 {
        show_error_at(KOR_LEAGUE_CUP_FILE, 0x5d);
        return this.addr();
    }
    if call(0x66c770) == 0 {
        show_error_at(KOR_LEAGUE_CUP_FILE, 0x64);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(KOR_LEAGUE_CUP_FILE, 0x6a);
        return this.addr();
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(KOR_LEAGUE_CUP_FILE, 0x71);
        return this.addr();
    }
    let group_b: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x66cb60usize);
    group_b(this.addr(), 0);
    this.addr()
}

/// Comparer for `construct_kor_league_cup`: only the original runs (it allocates, draws and reads the save file); the
/// plan is checked (`ita_cups_check_constructor_plan`: this, the vtable, the record, the year and, on a new cup, the
/// fixed fields and the statistics).
pub(crate) unsafe extern "thiscall" fn compare_construct_kor_league_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_KOR_LEAGUE_CUP)(this, year, record);
    let error = ita_cups_check_constructor_plan(this, r, year, record, 0x96ce84, &KOREAN_LEAGUE_CUP_FIELDS, false);
    SCOREBOARD.record(error.is_none(), || format!("construct_kor_league_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

// ---------- batch 313: the constructors of the Argentine Premier and Second Divisions (arg_prm.cpp, arg_second.cpp) ----------

/// The fixed part of the Argentine league constructors (thiscall(this, year, record), ret 8): League::League
/// (`exe::new_league`, thiscall, ret 0), the record (+4), the vtable, the year (`league::YEAR`, word) and discipline rule
/// 3 (`discipline_rule::ARGENTINA`); the saved state (`exe::league_load_from_temp_file`(this, 1), thiscall, ret 4).
/// Returns true when it loaded. Otherwise importance 0xf, the main stage, current stage −1 and `stages` stages with their
/// array (`exe::crt_malloc` of 4 × stages, cdecl, unchecked).
unsafe fn argentine_league_start(this: League, year: u32, record: usize, vtable: usize, stages: u32) -> bool {
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), vtable);
    write_u16(this.addr() + league::YEAR, year as u16);
    this.set_discipline_rule(discipline_rule::ARGENTINA);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return true;
    }
    this.set_importance(0xf);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    write_u32(this.addr() + league::STAGE_COUNT, stages);
    this.set_stages(exe::crt_malloc(stages * 4));
    false
}

/// The statistics of a new league (operator new `exe::crt_operator_new` of 0x5ce, cdecl; Comp_Stats::Comp_Stats
/// `exe::new_comp_stats`, push league -> thiscall, ret 4; 0 when out of memory) into `league::COMP_STATS`; false
/// without them.
unsafe fn argentine_league_stats(this: League) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    this.set_comp_stats(s);
    s != 0
}

/// Calls a routine of the league (thiscall, ret 0) and returns its eax.
unsafe fn argentine_league_call(this: League, f: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(f);
    f(this.addr())
}

/// The source files of the Argentine leagues' error boxes.
pub(crate) const ARG_PRM_FILE: usize = 0x9873e8;
pub(crate) const ARG_SECOND_FILE: usize = 0x9874ec;

/// Rule: the Argentine Premier Division at career creation: discipline rule 3 and, with no saved state, importance 15,
/// two stages (the Apertura and the Clausura), the rules (ArgPrm::set_up_rules 0x405670; 0: the box at line 0x4e), the
/// table (ArgPrm::create_table 0x405540; line 0x55), the fixtures (`exe::league_create_fixtures`; line 0x5b), the
/// statistics (line 0x62), the two half-season stages (ArgPrm::create_stages 0x405950, unchecked), the order
/// (`exe::league_sort_table`(this, 0); line 0x6c), the relegation averages (ArgPrm::init_relegation_averages 0x405ef0)
/// and the predicted positions (`exe::league_set_predicted_positions`). Every box returns this; returns this.
/// Original: `ArgPrm::ArgPrm` (0x404250, arg_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_arg_prm(this: usize, year: u32, record: usize) -> usize {
    let this = League(this);
    if argentine_league_start(this, year, record, 0x967324, 2) {
        return this.addr();
    }
    covered!("mwf_23 argentine premier new");
    if argentine_league_call(this, 0x405670) == 0 {
        show_error_at(ARG_PRM_FILE, 0x4e);
        return this.addr();
    }
    if argentine_league_call(this, 0x405540) == 0 {
        show_error_at(ARG_PRM_FILE, 0x55);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(ARG_PRM_FILE, 0x5b);
        return this.addr();
    }
    if !argentine_league_stats(this) {
        show_error_at(ARG_PRM_FILE, 0x62);
        return this.addr();
    }
    argentine_league_call(this, 0x405950);
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(ARG_PRM_FILE, 0x6c);
        return this.addr();
    }
    argentine_league_call(this, 0x405ef0);
    exe::league_set_predicted_positions(this);
    this.addr()
}

/// Rule: the Argentine Second Division (Nacional B) at career creation: discipline rule 3 and, with no saved state,
/// importance 15, six stages (the three zones, the promotion cup and the play-offs), the rules (ArgSecond::set_up_rules
/// 0x407e50; 0: the box at line 0x4e), the table (ArgSecond::fill_league_table 0x407cd0; line 0x55), the fixtures
/// (`exe::league_create_fixtures`; line 0x5b), the order (`exe::league_sort_table`(this, 0); line 0x62), the statistics
/// (line 0x69), the relegation averages (ArgSecond::init_relegation_averages 0x4093f0) and its own predicted positions
/// (ArgSecond::set_predicted_positions 0x409fe0). The Brazilian state leagues' body (`bra_reg_goias_construct_state_league`)
/// with discipline rule 3 and the averages before the predicted positions. Every box returns this; returns this.
/// Original: `ArgSecond::ArgSecond` (0x406bf0, arg_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_arg_second(this: usize, year: u32, record: usize) -> usize {
    let this = League(this);
    if argentine_league_start(this, year, record, 0x9673d8, 6) {
        return this.addr();
    }
    covered!("mwf_23 argentine second new");
    if argentine_league_call(this, 0x407e50) == 0 {
        show_error_at(ARG_SECOND_FILE, 0x4e);
        return this.addr();
    }
    if argentine_league_call(this, 0x407cd0) == 0 {
        show_error_at(ARG_SECOND_FILE, 0x55);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(ARG_SECOND_FILE, 0x5b);
        return this.addr();
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(ARG_SECOND_FILE, 0x62);
        return this.addr();
    }
    if !argentine_league_stats(this) {
        show_error_at(ARG_SECOND_FILE, 0x69);
        return this.addr();
    }
    argentine_league_call(this, 0x4093f0);
    argentine_league_call(this, 0x409fe0);
    this.addr()
}

/// The comparer of the Argentine league constructors: only the original runs (it allocates and reads the save file);
/// checks the return, the vtable, the year, the record, the discipline rule and, for a new league with statistics, the
/// stage count, the stage array and the importance.
unsafe fn compare_argentine_league_ctor(name: &str, this: League, year: u32, record: usize, vtable: usize, stages: u32, original: impl FnOnce() -> usize) -> usize {
    let loaded = league_will_load_saved_state(record);
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == vtable && (this.year() as u16) == year as u16 && this.competition().addr() == record
        && this.discipline_rule() == discipline_rule::ARGENTINA;
    if !loaded && this.comp_stats() != 0 {
        ok = ok && this.stage_count() == stages as usize && this.stages() != 0 && this.importance() == 0xf;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}) [year {}, record {:#x}, loaded {}]: returned {:#x}, vtable {:#x}, discipline {}, stages {}, importance {}", name, this.addr(), year as u16,
        record, loaded, r, read_usize(this.addr()), this.discipline_rule(), this.stage_count(), this.importance()));
    r
}

/// Comparer for `construct_arg_prm`.
pub(crate) unsafe extern "thiscall" fn compare_construct_arg_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ARG_PRM);
    compare_argentine_league_ctor("construct_arg_prm", League(this), year, record, 0x967324, 2, || o(this, year, record))
}

/// Comparer for `construct_arg_second`.
pub(crate) unsafe extern "thiscall" fn compare_construct_arg_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ARG_SECOND);
    compare_argentine_league_ctor("construct_arg_second", League(this), year, record, 0x9673d8, 6, || o(this, year, record))
}
