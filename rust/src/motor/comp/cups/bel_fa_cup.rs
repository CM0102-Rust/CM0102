use super::*;

/// Rule: the Belgian Cup takes 112 clubs: 44 Belgian clubs of the four Fourth Division groups (sorted by strength, the
/// best 132 shuffled) with class 3, then the Third Division clubs (the national one and groups A and B) with class 2,
/// the Second Division's with class 1 and the First Division's with class 0, each group in database order until the
/// list is full; returns 1 when it is full.
///
/// `team_count` = 112; the list (`exe::crt_malloc` of 0x2a0 into `cup::TEAM_TABLE`; out of memory: the box of
/// bel_fa_cup.cpp at line 0x161 and −1). Over the clubs (a 16-bit index up to [`G_CLUB_COUNT`]) whose division
/// (`club::DIVISION`) has the id of `KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_A` to `_D` and whose nation (`club::NATION`)
/// is Belgium's record (`KEY_NATION_BELGIUM`): a local list of at most 150. `exe::comp_util_sort_and_shuffle_top_clubs`
/// (list, n, 44) and its first 44 into the entries 0..43 (the original copies 44 whatever the count: with fewer it reads
/// stack garbage; the copy reads zeros). Then three passes over the clubs, each adding at the next entry and stopping
/// when the count reaches `team_count` (checked only after adding): the division id of `KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION`,
/// `_A` or `_B` (class 2), `KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION` (1) and `KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION` (0).
/// Returns count == `team_count`.
/// Original: `BelFaCup::collect_entrants` (0x41d8d0, bel_fa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_fa_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let mut lower = [0usize; 150];
    let mut n: i16 = 0;
    this.set_team_count(0x70);
    let l = exe::crt_malloc(0x2a0);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0x989ae0, 0x161);
        return -1;
    }
    let fourth = [KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_A, KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_B, KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_C, KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_D];
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i16;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && fourth.iter().any(|&g| read_usize(d) == read_usize(g)) && read_usize(club + club::NATION) == oceania_nations_nation_record(KEY_NATION_BELGIUM) && n < 0x96 {
            lower[n as usize] = club;
            n += 1;
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    covered!("mwc_22 belgian cup entrants");
    exe::comp_util_sort_and_shuffle_top_clubs(lower.as_mut_ptr() as usize, n as u32, 0x2c); // mwc22: the best 132 shuffled
    for k in 0..0x2c {
        set_cup_entry(this, k, lower[k], 3);
    }
    let mut k = 0x2ci32;
    let passes: [(&[usize], u8); 3] = [(&[KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION, KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION_A, KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION_B], 2),
        (&[KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION], 1), (&[KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION], 0)];
    for (keys, class) in passes {
        let mut club = read_usize(G_CLUB_TABLE);
        let mut i = 0i16;
        while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
            let d = read_usize(club + club::DIVISION);
            if d != 0 && keys.iter().any(|&g| read_usize(d) == read_usize(g)) {
                set_cup_entry(this, k as usize, club, class);
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

/// Comparer for `bel_fa_cup_collect_entrants` (side by side with the random generator restored, every entry compared).
pub(crate) unsafe extern "thiscall" fn compare_bel_fa_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_BEL_FA_CUP_COLLECT_ENTRANTS);
    compare_entrants_with_seeds("bel_fa_cup_collect_entrants", Cup(this), || o(this), || bel_fa_cup_collect_entrants(this), |_| true)
}

/// At the season turn the Belgian Cup is rebuilt: last edition's tables freed, the history archived, the year +1, the
/// entrants (BelFaCup::collect_entrants), the rounds, the tie tree and the predicted positions.
///
/// The knockout-cup turnover template (`usa_mls_all_stars_cup_start_new_season`), checked instruction by instruction
/// against `JapSuper::start_new_season`: only the entrants routine (`BelFaCup::collect_entrants`, thiscall, ret 0), the
/// source file and the box lines (0x216, 0x21d, 0x224) change.
/// Original: `BelFaCup::start_new_season` (0x41db90, bel_fa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_fa_cup_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x41d8d0, 0x989ae0, [0x216, 0x21d, 0x224], 1, true, "mwc_14 bel fa cup turnover") // mwc14: the Belgian Cup
}

/// Comparer for `bel_fa_cup_start_new_season`: only the original runs; the year goes up by one
/// (`compare_cup_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_bel_fa_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BEL_FA_CUP_START_NEW_SEASON);
    compare_cup_new_season("bel_fa_cup_start_new_season", Cup(this), || o(this))
}

/// The Belgian Cup is born with discipline rule 5, the bench (5 substitutes named, 3 substitutions); with no saved
/// state, the entrants (BelFaCup::collect_entrants), the round table, the tie tree (a failure is a negative return),
/// the statistics and the predicted positions (`construct_nation_cup`).
/// Original: `BelFaCup::BelFaCup` (0x41d050, bel_fa_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bel_fa_cup(this: usize, year: u32, record: usize) -> usize {
    construct_nation_cup(Cup(this), year, record, &BEL_FA_CUP_CUP_PARAMS)
}

/// Comparer for `construct_bel_fa_cup`: only the original runs (`compare_nation_cup_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_bel_fa_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BEL_FA_CUP);
    compare_nation_cup_ctor("construct_bel_fa_cup", Cup(this), year, record, &BEL_FA_CUP_CUP_PARAMS, || o(this, year, record))
}

/// The Belgian Cup has 8 round(s): code 30: draw 6 Aug, match 12 Aug, 76 teams (76 new); code 40: draw 13 Aug, match 19
/// Aug, 56 teams (18 new); code 50: draw 20 Aug, match 26 Aug, 28 teams (0 new); code 60: draw 15 Sep, match 5 Nov, 32
/// teams (18 new); code 70: draw 6 Nov, match 29 Nov, 16 teams (0 new); code 120: draw 30 Nov, match 24 Jan (+1 year),
/// 8 teams (0 new); code 130: draw 25 Jan (+1 year), match 28 Mar (+1 year), 4 teams (0 new), two legs; code 150: draw
/// 12 Apr (+1 year), match 27 May (+1 year), 2 teams (0 new).
///
/// `BelFaCup::create_round_table(stage, &rounds, &extra, &out)` (`single_stage_round_table`): only stage -1 (the others
/// return 0 without writing); out = 0, rounds = the table size, extra = 0 and the table of 0x68-byte rounds
/// (`exe::crt_malloc`; out of memory, the error box of line 0xa9 and 0), each with its draw and match dates
/// (`cup_set_round_draw_date`, `cup_set_round_match_date`, cdecl) and its fields (`fra_cup_write_cup_round`).
/// Original: `BelFaCup::create_round_table` (0x41d310, bel_fa_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bel_fa_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    single_stage_round_table(Cup(this), stage, rounds, extra, out, &BEL_FA_CUP_ROUNDS, 0x989ae0, 0xa9, "mwc_12 bel fa cup round table")
}

/// Comparer for `bel_fa_cup_create_round_table`: side by side (`compare_cup_round_table`).
pub(crate) unsafe extern "thiscall" fn compare_bel_fa_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BEL_FA_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("bel_fa_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| bel_fa_cup_create_round_table(this, stage, r, e, s))
}

/// Side by side for an entrant list whose seeds are written only for some entries: the original and the copy (the
/// random generator restored between them), then each entry's club and status compared, and the seed only where
/// `seeded(entry index)`; the copy's list is freed.
pub(crate) unsafe fn compare_entrants_with_seeds(name: &str, this: Cup, o: impl FnOnce() -> i32, c: impl FnOnce() -> i32, seeded: impl Fn(usize) -> bool) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    // the draw cursor too (crate::rng::STATE_REGIONS): utils_random (0x90cfc0) walks [0xde1f60] over the fixed table and
    // caches in [0xde1f5c], so without restoring them the copy's seeds are the 32 draws after the original's.
    let reg = [(this.addr() + cup::TEAM_COUNT, 2usize), (G_MOUSE_LEFT_BUTTON_DOWN, 4), crate::rng::STATE_REGIONS[0], crate::rng::STATE_REGIONS[1]];
    let (a, b, same, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o();
        lo.set(this.team_table());
        r
    }, || {
        let r = c();
        lr.set(this.team_table());
        this.set_team_table(lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let n = if a == 1 { this.team_count().max(0) as usize } else { 0 };
    let list = if lo != 0 && lr != 0 {
        (0..n).all(|k| read_usize(lo + 6 * k) == read_usize(lr + 6 * k) && read_u8(lo + 6 * k + 5) == read_u8(lr + 6 * k + 5) && (!seeded(k) || read_u8(lo + 6 * k + 4) == read_u8(lr + 6 * k + 4)))
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(same && a == b && list, || format!("{}({:#x}) returned {} x {} memory {} list {} ({} entries)", name, this.addr(), a, b, same, list, n));
    a
}

/// The body of the cup constructors of Belgium, Denmark, Germany and Holland (thiscall(this, year, record), ret 8, SEH
/// frame only for the operator new); returns this.
///
/// The base (Cup::Cup `exe::new_cup`: mov ecx -> thiscall, ret 0), the year (`+0x40`), the competition record (`+4`),
/// the vtable, `cup::CURRENT_ROUND` = 0, stage 0xff (`+0x44`), current stage -1 (`+0x30`), no stages (`+0x2c` = 0),
/// played by clubs (`cup::CLUB_SIDES` = 1), the bench when the cup writes it, the discipline rule and
/// `cup::RUN_BY_STAGES` = 0; then it tries to load the saved state (Cup::load_from_disk `0x51fc00`: push 1; mov ecx ->
/// thiscall, ret 4). Loaded: done.
///
/// Otherwise the new cup: the entrants (direct call), the round table (virtual +0x3c with stage -1, &`+0x3c`, &`+0x3a`
/// and 0; into `cup::ROUND_TABLE`, unchecked), the tie tree (Cup::map_fixture_tree `0x518790`, thiscall, ret 0), the
/// statistics (operator new of 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats::Comp_Stats `exe::new_comp_stats`: push
/// cup; mov ecx, new -> thiscall, ret 4) into `+8` and, in the cups that have them, the predicted positions
/// (Cup::rank_teams `0x5223a0`, thiscall, ret 0). The entrants and the tree that fail (0 or negative, as `zero_fails`)
/// and the statistics out of memory show the error box of their line and end.
pub(crate) unsafe fn construct_nation_cup(this: Cup, year: u32, record: usize, p: &NationCup) -> usize {
    let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(p.entrants);
    let tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_CUP_MAP_FIXTURE_TREE);
    let failed = |r: i32| if p.zero_fails { r == 0 } else { r < 0 };
    exe::new_cup(this);
    this.set_year((year as u16) as i16);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), p.vtable);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    this.set_current_stage(0xffff_ffff);
    this.set_stage_count(0);
    this.set_club_sides(1);
    if let Some((named, allowed)) = p.bench {
        this.set_substitutes_named(named);
        this.set_substitutions_allowed(allowed);
    }
    this.set_discipline_index(p.discipline);
    this.set_run_by_stages(0);
    if exe::cup_load_from_disk(this, 1) != 0 {
        return this.addr();
    }
    covered!("mwc_16 new cup");
    if failed(entrants(this.addr())) {
        show_error_at(p.source_file, p.box_lines[0]);
        return this.addr();
    }
    let rounds: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = rounds(this.addr(), 0xffff_ffff, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0);
    this.set_round_table(t);
    if failed(tree(this.addr())) {
        show_error_at(p.source_file, p.box_lines[1]);
        return this.addr();
    }
    let e = exe::crt_operator_new(0x5ce);
    let s = if e != 0 { exe::new_comp_stats(e, this) } else { 0 };
    write_usize(this.addr() + 8, s);
    if s == 0 {
        show_error_at(p.source_file, p.box_lines[2]);
        return this.addr();
    }
    if p.predicted {
        exe::cup_rank_teams(this);
    }
    this.addr()
}

/// The comparer of these cup constructors: the original builds the cup (it allocates, loads files and draws), so only
/// it runs. Checks the return value, vtable, year, record, discipline rule and, for a new cup, the bench, the club
/// sides and the statistics.
pub(crate) unsafe fn compare_nation_cup_ctor(name: &str, this: Cup, year: u32, record: usize, p: &NationCup, original: impl FnOnce() -> usize) -> usize {
    let loaded = league_will_load_saved_state(record);
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == p.vtable && built_year_ok(this.year() as u16, year as u16)
        && read_usize(this.addr() + league::COMPETITION) == record && this.discipline_index() == p.discipline && this.stage_index() == stage_index::MAIN;
    if !loaded {
        ok = ok && this.club_sides() == 1 && read_usize(this.addr() + 8) != 0;
        if let Some((named, allowed)) = p.bench {
            ok = ok && this.substitutes_named() == named && this.substitutions_allowed() == allowed;
        }
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}) [year {} arg {}, record {:#x} arg {:#x}, discipline {}, bench {}/{}]: returned {:#x}, vtable {:#x}, +8 = {:#x}", name, this.addr(), this.year(), year as u16, read_usize(this.addr() + league::COMPETITION), record, this.discipline_index(), this.substitutes_named(), this.substitutions_allowed(), r, read_usize(this.addr()), read_usize(this.addr() + 8)));
    r
}

pub(crate) const BEL_FA_CUP_CUP_PARAMS: NationCup = NationCup {
    vtable: 0x9678a0,
    bench: Some((5, 3)),
    discipline: discipline_rule::BELGIUM_CUP, // mwc16: the Belgian Cup discipline rule
    entrants: 0x41d8d0,
    zero_fails: false,
    predicted: true,
    source_file: 0x989ae0,
    box_lines: [0x50, 0x5e, 0x65],
};

/// The Belgian Cup: 8 rounds.
pub(crate) const BEL_FA_CUP_ROUNDS: [FraCupCupRound; 8] = [
    cql11r((6, 7, 0, 6), (0xc, 7, 0, 5, 2, 0), 0x1e, (0, 0x401, 0), 4, (0x4c, 0x26, 0x4c, 0), (0, 1, 0), (0, 0, 0)),
    cql11r((0xd, 7, 0, 6), (0x13, 7, 0, 5, 2, 0), 0x28, (0, 0x403, 0), 4, (0x38, 0x1c, 0x12, 0x4c), (0, 1, 0), (0, 0, 0)),
    cql11r((0x14, 7, 0, 6), (0x1a, 7, 0, 5, 2, 0), 0x32, (0, 0x403, 0), 4, (0x1c, 0xe, 0, 0), (0, 1, 0), (0, 0, 0)),
    cql11r((0xf, 8, 0, 6), (5, 0xa, 0, 6, 2, 0), 0x3c, (0, 3, 0), 4, (0x20, 0x10, 0x12, 0x5e), (0, 1, 0), (0, 0xffffffff, 0)), // mwc12 bel_fa_cup round 3
    cql11r((6, 0xa, 0, 0), (0x1d, 0xa, 0, 2, 2, 0), 0x46, (0, 3, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x1e, 0xa, 0, 3), (0x18, 0, 1, 2, 2, 0), 0x78, (0, 2, 3), 6, (8, 4, 0, 0), (1, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x19, 0, 1, 3), (0x1c, 2, 1, 2, 2, 0), 0x82, (0, 0, 3), 6, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffffffff, 0)),
    cql11r((0xc, 3, 1, 3), (0x1b, 4, 1, 6, 2, 4), 0x96, (0, 2, 3), 6, (2, 1, 0, 0), (1, 1, 6), (0, 0xffffffff, 0)),
];
