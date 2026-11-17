// ---------- batch 302: the rules of the Russian Premier League and the Turkish divisions (rus_prm.cpp, tur_first.cpp, tur_second.cpp) ----------

/// The id global of the division below the Turkish top flight (0x9cf8d0, unnamed in the community map:
/// `g_key_club_comp_turkish_unmatched_a`, docs/chaves_uso.csv).
pub(crate) const KEY_CLUB_COMP_TURKISH_UNMATCHED_A: usize = 0x9cf8d0;

/// The rules of the Russian Premier League: 2 legs, 3 points a win and 1 a draw; tie-breaks 3, 4, 1, 2 (wins, head to
/// head, goal difference, goals for); nobody goes up, 2 go down (+0xc1) to the First Division
/// (`KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION`); +0x52 = 2; 7 substitutes, 3 substitutions; points ties settled by
/// tie-break matches (+0xe6 = 1); the rule bit 0x40 (+0xd9, listed for the comparer).
pub(crate) unsafe fn rus_prm_rules() -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 3), (0xc6, 1, 4), (0xc7, 1, 1),
        (0xc8, 1, 2), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 2), (0x1c, 4, 0xffff_ffff), (0x52, 1, 2), (0x4a, 1, 3),
        (0x20, 4, read_usize(KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION) as u32), (0x49, 1, 7), (0xe6, 4, 1), (league::LEAGUE_FLAGS, 1, 0)]
}

/// The rules of the two Turkish divisions: 2 legs, 3 points a win and 1 a draw; tie-breaks 4, 1, 2, 3 (head to head,
/// goal difference, goals for, wins); `up` go up directly (+0xbe), none to play-offs, 3 go down (+0xc1); the given
/// leagues above and below; +0x52 = 2; 7 substitutes, 3 substitutions; no rule bits set.
pub(crate) unsafe fn turkish_league_rules(up: u32, above: u32, below: u32) -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 4), (0xc6, 1, 1), (0xc7, 1, 2),
        (0xc8, 1, 3), (0xbe, 1, up), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 3), (0x1c, 4, above), (0x4a, 1, 3), (0x52, 1, 2), (0x20, 4, below), (0x49, 1, 7)]
}

/// The Turkish First Division: nobody goes up, nothing above, the division of id [0x9cf8d0] below.
pub(crate) unsafe fn tur_first_rules() -> Vec<(usize, u8, u32)> {
    turkish_league_rules(0, 0xffff_ffff, read_usize(KEY_CLUB_COMP_TURKISH_UNMATCHED_A) as u32)
}

/// The Turkish Second Division: 3 go up to the top flight (`KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION`), the second
/// division category B (`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B`) below.
pub(crate) unsafe fn tur_second_rules() -> Vec<(usize, u8, u32)> {
    turkish_league_rules(3, read_usize(KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION) as u32, read_usize(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B) as u32) // mwd14: three go up
}

/// Rule: the Russian Premier League: 2 legs, 3 points a win and 1 a draw; tie-breaks wins, head to head, goal
/// difference, goals for; points ties across a band go to tie-break matches; nobody goes up, 2 go down; rule bit 0x40
/// (`rus_prm_rules`).
/// Original: `RusPrm::set_up_rules` (0x7eb740, rus_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwd_14 russian premier league rules");
    apply_league_rules_with_flags(League(this), 0x40, &rus_prm_rules())
}

/// Rule: the Turkish First Division: 2 legs, 3 points a win and 1 a draw; tie-breaks head to head, goal difference,
/// goals for, wins; nobody goes up, 3 go down (`turkish_league_rules`).
/// Original: `TurFirst::set_up_rules` (0x8fdf90, tur_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_first_set_up_rules(this: usize) -> u32 {
    covered!("mwd_14 turkish first division rules");
    eng_prm_apply_league_rules(League(this), &tur_first_rules())
}

/// Rule: the Turkish Second Division: as the First Division, with 3 going up to the top flight and 3 down to the
/// second division category B.
/// Original: `TurSecond::set_up_rules` (0x8ff280, tur_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_set_up_rules(this: usize) -> u32 {
    covered!("mwd_14 turkish second division rules");
    eng_prm_apply_league_rules(League(this), &tur_second_rules())
}

/// Comparer for `rus_prm_set_up_rules` (the rehearsal of `eng_prm_compare_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_rus_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_RUS_PRM_SET_UP_RULES);
    eng_prm_compare_league_rules("rus_prm_set_up_rules", League(this), rus_prm_rules(), || o(this), || rus_prm_set_up_rules(this))
}

/// Comparer for `tur_first_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_tur_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_TUR_FIRST_SET_UP_RULES);
    eng_prm_compare_league_rules("tur_first_set_up_rules", League(this), tur_first_rules(), || o(this), || tur_first_set_up_rules(this))
}

/// Comparer for `tur_second_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_TUR_SECOND_SET_UP_RULES);
    eng_prm_compare_league_rules("tur_second_set_up_rules", League(this), tur_second_rules(), || o(this), || tur_second_set_up_rules(this))
}

// ---------- batch 302: the tables of the Greek divisions and the Russian Premier League (gre_prm.cpp, gre_second.cpp, rus_prm.cpp) ----------

/// Rule: the Greek First Division has 14 clubs, those whose national division is it
/// (`KEY_CLUB_COMP_GREEK_PREMIER_DIVISION`), in database order; when full, the prize fund (100,000 a share) and TV
/// pays 500,000 to each club (`eng_prm_fill_league_table`: the table of 14 × 0x3b, the box at line 0x163; League::
/// calc_prize_fund push 100000; League::pay_tv_rights push 0; push 500000).
/// Original: `GrePrm::create_table` (0x5ead60, gre_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_prm_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0xe, 0x9cdae4, 0x163, KEY_CLUB_COMP_GREEK_PREMIER_DIVISION, 0x186a0, || {
        covered!("mwd_15 greek first division tv money");
        eng_prm_pay_tv_rights(this, 0x7a120, 0) // mwd15: the Greek TV money
    })
}

/// Rule: the Greek Second Division has 14 clubs (`KEY_CLUB_COMP_GREEK_SECOND_DIVISION`); when full, the same prize
/// fund (100,000 a share) and TV money (500,000) as the First Division (the box at line 0x169).
/// Original: `GreSecond::create_table` (0x5ebcb0, gre_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_second_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0xe, 0x9cdb1c, 0x169, KEY_CLUB_COMP_GREEK_SECOND_DIVISION, 0x186a0, || {
        covered!("mwd_15 greek second division tv money");
        eng_prm_pay_tv_rights(this, 0x7a120, 0)
    })
}

/// Rule: the Russian Premier League has 16 clubs (`KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION`); when full, the prize fund
/// (50,000 a share) and TV pays 300,000 to each club (the box at line 0x13c).
/// Original: `RusPrm::create_table` (0x7eb610, rus_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_prm_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x10, 0xa7acc8, 0x13c, KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION, 0xc350, || {
        covered!("mwd_15 russian premier league tv money");
        eng_prm_pay_tv_rights(this, 0x493e0, 0)
    })
}

/// Comparer for `gre_prm_create_table` (only the original runs; the planned clubs and the prize fund checked).
pub(crate) unsafe extern "thiscall" fn compare_gre_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GRE_PRM_CREATE_TABLE);
    eng_prm_compare_league_table("gre_prm_create_table", League(this), 0xe, KEY_CLUB_COMP_GREEK_PREMIER_DIVISION, 0x186a0, || o(this))
}

/// Comparer for `gre_second_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_gre_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GRE_SECOND_CREATE_TABLE);
    eng_prm_compare_league_table("gre_second_create_table", League(this), 0xe, KEY_CLUB_COMP_GREEK_SECOND_DIVISION, 0x186a0, || o(this))
}

/// Comparer for `rus_prm_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_rus_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_RUS_PRM_CREATE_TABLE);
    eng_prm_compare_league_table("rus_prm_create_table", League(this), 0x10, KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION, 0xc350, || o(this))
}

// ---------- batch 302: the tables of the Russian First Division, the Polish Second Division and the Turkish Second Division ----------

/// Rule: the Russian First Division has 18 clubs: the first 18 clubs of the division
/// (`KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION`) in database order (any more are left out of the table), sorted by
/// reputation and shuffled (all of them), in that order; with fewer than 18, no table (0). No prize fund nor TV money.
///
/// `club_count` = 18; the table (`exe::crt_malloc` of 18 × 0x3b; out of memory: the box at line 0xf0 and 0); a local
/// list of at most 18 club pointers (16-bit database index); fewer than `club_count`: 0 (the table stays allocated);
/// otherwise `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, `club_count`) (cdecl; the original pushes n and
/// `club_count` with stack garbage above the words, which the routine does not read) and each club i in row i
/// (League::init_table_row(this, i, club, 0, 0)); a club beyond `club_count` (never, the list is capped at 18) would be
/// moved to the south zone of the second division (`KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_SOUTH`, `club::DIVISION`).
/// Returns 1.
/// Original: `RusFirst::create_table` (0x7ea6c0, rus_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_first_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0x12);
    let t = exe::crt_malloc(0x426);
    this.set_table(t);
    if t == 0 {
        show_error_at(0xa7ac8c, 0xf0);
        return 0;
    }
    let mut list = [0usize; 0x12];
    let mut n: u8 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION) && (n as i8) < 0x12 {
            list[n as i8 as usize] = club;
            n = n.wrapping_add(1);
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    let n = n as i8 as i16;
    if n < this.club_count() {
        return 0;
    }
    covered!("mwd_16 russian first division table");
    exe::comp_util_sort_and_shuffle_top_clubs(list.as_ptr() as usize, n as u16 as u32, this.club_count() as u16 as u32); // mwd16: sorted and shuffled
    let mut k: i16 = 0;
    while k < n {
        let c = list[k as usize];
        if k < this.club_count() {
            exe::league_init_table_row(this, k as u16 as u32, c, 0, 0);
        } else {
            write_usize(c + club::DIVISION, comp_record_of(KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_SOUTH));
        }
        k += 1;
    }
    1
}

/// Rule: the Polish Second Division has 20 clubs in 2001 and 18 afterwards, those of the division
/// (`KEY_CLUB_COMP_POLISH_SECOND_DIVISION`) in database order; when full, the prize fund (20,000 a share); no TV money.
///
/// `club_count` = 20, games per club not computed (`league::GAMES_PER_CLUB` = −1), `club_count` = 18 unless the year is
/// 2001; then `eng_prm_fill_league_table` (the box at line 0x199; League::calc_prize_fund push 20000, `prize_mode` 1)
/// with no TV payment.
/// Original: `PolSecond::create_table` (0x7cbdf0, pol_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_second_create_table(this: usize) -> u32 {
    let l = League(this);
    l.set_club_count(0x14);
    write_u16(this + league::GAMES_PER_CLUB, 0xffff);
    if l.year() != 0x7d1 {
        l.set_club_count(0x12);
    }
    let n = l.club_count();
    eng_prm_fill_league_table(l, n, 0xa7860c, 0x199, KEY_CLUB_COMP_POLISH_SECOND_DIVISION, 0x4e20, || { covered!("mwd_16 polish second division table"); }) // mwd16: the Polish prize share
}

/// Fills a league table with the clubs of the competition of id [`g`] in database order until it is full, as
/// `eng_prm_fill_league_table` but without the prize fund: `club_count` = n, the table (out of memory: the box at
/// `row` and false), League::init_table_row(this, k, club, 0, 0) for each club; true when full.
pub(crate) unsafe fn fill_league_table_rows(this: League, n: i16, file: usize, row: u32, g: usize) -> bool {
    this.set_club_count(n);
    let t = exe::crt_malloc((n as i32 as u32).wrapping_mul(0x3b));
    this.set_table(t);
    if t == 0 {
        show_error_at(file, row);
        return false;
    }
    let mut k: u8 = 0;
    let mut i: i32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let comp = read_usize(club + club::DIVISION);
        if comp != 0 && read_usize(comp) == read_usize(g) {
            exe::league_init_table_row(this, k as u32, club, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == this.club_count() {
                break;
            }
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    k as i8 as i16 == this.club_count()
}

/// Rule: the Turkish Second Division has 20 clubs, those of the second division category A
/// (`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A`) in database order; when full, no prize fund (+0x28 = 0, prize mode
/// 2 = nothing) and TV pays 100,000 to each club (League::pay_tv_rights: push 0; push 100000).
/// Original: `TurSecond::create_table` (0x8ff150, tur_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_create_table(this: usize) -> u32 {
    let l = League(this);
    if !fill_league_table_rows(l, 0x14, 0xaa187c, 0x159, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A) {
        return 0;
    }
    covered!("mwd_16 turkish second division table");
    l.set_prize_fund(0);
    l.set_prize_mode(2);
    eng_prm_pay_tv_rights(this, 0x186a0, 0); // mwd16: the Turkish Second Division TV money
    1
}

/// Comparer for `rus_first_create_table`: only the original runs (it draws); the table must hold 18 clubs of the
/// division when there are at least 18 (any order), and the return 1; otherwise 0.
pub(crate) unsafe extern "thiscall" fn compare_rus_first_create_table(this: usize) -> u32 {
    let l = League(this);
    let plan = division_clubs_plan(0x12, 0x57, KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION);
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_RUS_FIRST_CREATE_TABLE)(this);
    let mut found: Vec<usize> = Vec::new();
    let mut ok = r == (plan.len() == 0x12) as u32 && l.club_count() == 0x12 && l.table() != 0;
    if ok && r == 1 {
        found = (0..0x12).map(|k| read_usize(l.table() + k * 0x3b)).collect();
        let (mut a, mut b) = (plan.clone(), found.clone());
        a.sort_unstable();
        b.sort_unstable();
        ok = a == b;
    }
    SCOREBOARD.record(ok, || format!("rus_first_create_table({:#x}) original={} plan={:x?} table={:x?}", this, r, plan, found));
    r
}

/// Comparer for `pol_second_create_table` (only the original runs; the planned clubs and the prize fund checked).
pub(crate) unsafe extern "thiscall" fn compare_pol_second_create_table(this: usize) -> u32 {
    let n = if League(this).year() == 0x7d1 { 0x14 } else { 0x12 };
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POL_SECOND_CREATE_TABLE);
    eng_prm_compare_league_table("pol_second_create_table", League(this), n, KEY_CLUB_COMP_POLISH_SECOND_DIVISION, 0x4e20, || o(this))
}

/// Comparer for `tur_second_create_table`: only the original runs; the planned clubs (`compare_division_table`) and,
/// when full, no prize fund with prize mode 2.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_TUR_SECOND_CREATE_TABLE);
    let r = compare_division_table("tur_second_create_table", League(this), 0x14, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A, || o(this));
    if r == 1 {
        let l = League(this);
        SCOREBOARD.record(l.prize_fund() == 0 && l.prize_mode() == 2, || format!("tur_second_create_table({:#x}) prize fund {} mode {}", this, l.prize_fund(), l.prize_mode()));
    }
    r
}

// ---------- batch 302: the schedules of the Greek divisions and the Turkish Second Division (gre_prm.cpp, gre_second.cpp, tur_second.cpp) ----------

/// One stage of a league schedule: its rounds (written with `spa_second_set_league_rounds`), the extra output (the
/// stage code), the league output (1 = a league stage) and the line of the out-of-memory box.
pub(crate) struct LeagueStageRounds {
    pub(crate) rounds: &'static [SpaSecondScheduleRound],
    pub(crate) extra: i16,
    pub(crate) league: i32,
    pub(crate) row: u32,
}

/// The body of a league `create_schedule` whose stages are fixed tables (thiscall(this, stage, &rounds, &extra,
/// &league), ret 0x10): for stage −1 (`main`) or stage 0 (`stage0`, when the league has one), the league output (if
/// given), the round count and the extra; the table (`exe::crt_malloc` of 0x41 per round; out of memory: the box at
/// the stage's line and 0) with the stage's rounds (League::set_round_date / set_round_match_slot, cdecl; the year
/// pushed as the word +0x40). Other stages: 0, nothing written.
pub(crate) unsafe fn fixed_league_schedule(this: League, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32, file: usize, main: &LeagueStageRounds,
    stage0: Option<&LeagueStageRounds>, branch: &'static str) -> usize {
    let s = match (stage as u8, stage0) {
        (0xff, _) => main,
        (0, Some(s)) => s,
        _ => return 0,
    };
    if !league.is_null() {
        *league = s.league;
    }
    *rounds = s.rounds.len() as i16;
    *extra = s.extra;
    let t = exe::crt_malloc((s.rounds.len() * 0x41) as u32);
    if t == 0 {
        show_error_at(file, s.row);
        return 0;
    }
    covered!(branch);
    spa_second_set_league_rounds(t, (this.year() as u16) as u32, 0, s.rounds);
    t
}

/// The Greek First Division: 26 rounds (14 clubs, 2 legs) on Sundays (weekday 6), 17 September to 6 May, the winter
/// break from 17 December to 28 January.
pub(crate) const GREEK_FIRST_DIVISION_ROUNDS: [SpaSecondScheduleRound; 26] = [
    (17, 8, 0, 6, 1, &[(-1, -1, -1)]), // 0: 17/set
    (24, 8, 0, 6, 1, &[(-1, -1, -1)]), // 1: 24/set
    (1, 9, 0, 6, 1, &[(-1, -1, -1)]), // 2: 1/out
    (15, 9, 0, 6, 1, &[(-1, -1, -1)]), // 3: 15/out
    (22, 9, 0, 6, 1, &[(-1, -1, -1)]), // 4: 22/out
    (29, 9, 0, 6, 1, &[(-1, -1, -1)]), // 5: 29/out
    (5, 10, 0, 6, 1, &[(-1, -1, -1)]), // 6: 5/nov
    (12, 10, 0, 6, 1, &[(-1, -1, -1)]), // 7: 12/nov
    (19, 10, 0, 6, 1, &[(-1, -1, -1)]), // 8: 19/nov
    (26, 10, 0, 6, 1, &[(-1, -1, -1)]), // 9: 26/nov
    (3, 11, 0, 6, 1, &[(-1, -1, -1)]), // 10: 3/dez
    (10, 11, 0, 6, 1, &[(-1, -1, -1)]), // 11: 10/dez
    (17, 11, 0, 6, 1, &[(-1, -1, -1)]), // 12: 17/dez
    (28, 0, 1, 6, 1, &[(-1, -1, -1)]), // 13: 28/jan (+1 ano)
    (4, 1, 1, 6, 1, &[(-1, -1, -1)]), // 14: 4/fev (+1 ano)
    (11, 1, 1, 6, 1, &[(-1, -1, -1)]), // 15: 11/fev (+1 ano)
    (18, 1, 1, 6, 1, &[(-1, -1, -1)]), // 16: 18/fev (+1 ano)
    (25, 1, 1, 6, 1, &[(-1, -1, -1)]), // 17: 25/fev (+1 ano)
    (4, 2, 1, 6, 1, &[(-1, -1, -1)]), // 18: 4/mar (+1 ano)
    (11, 2, 1, 6, 1, &[(-1, -1, -1)]), // 19: 11/mar (+1 ano)
    (18, 2, 1, 6, 1, &[(-1, -1, -1)]), // 20: 18/mar (+1 ano)
    (1, 3, 1, 6, 1, &[(-1, -1, -1)]), // 21: 1/abr (+1 ano)
    (8, 3, 1, 6, 1, &[(-1, -1, -1)]), // 22: 8/abr (+1 ano)
    (22, 3, 1, 6, 1, &[(-1, -1, -1)]), // 23: 22/abr (+1 ano)
    (29, 3, 1, 6, 1, &[(-1, -1, -1)]), // 24: 29/abr (+1 ano)
    (6, 4, 1, 6, 1, &[(-1, -1, -1)]), // 25: 6/mai (+1 ano)
];

/// The Greek Second Division: 30 rounds on Saturdays (weekday 5), 16 September to 28 April, the break from 16
/// December to 6 January.
pub(crate) const GREEK_SECOND_DIVISION_ROUNDS: [SpaSecondScheduleRound; 30] = [
    (16, 8, 0, 5, 1, &[(-1, -1, -1)]), // 0: 16/set
    (23, 8, 0, 5, 1, &[(-1, -1, -1)]), // 1: 23/set
    (30, 8, 0, 5, 1, &[(-1, -1, -1)]), // 2: 30/set
    (7, 9, 0, 5, 1, &[(-1, -1, -1)]), // 3: 7/out
    (14, 9, 0, 5, 1, &[(-1, -1, -1)]), // 4: 14/out
    (21, 9, 0, 5, 1, &[(-1, -1, -1)]), // 5: 21/out
    (28, 9, 0, 5, 1, &[(-1, -1, -1)]), // 6: 28/out
    (4, 10, 0, 5, 1, &[(-1, -1, -1)]), // 7: 4/nov
    (11, 10, 0, 5, 1, &[(-1, -1, -1)]), // 8: 11/nov
    (18, 10, 0, 5, 1, &[(-1, -1, -1)]), // 9: 18/nov
    (25, 10, 0, 5, 1, &[(-1, -1, -1)]), // 10: 25/nov
    (2, 11, 0, 5, 1, &[(-1, -1, -1)]), // 11: 2/dez
    (9, 11, 0, 5, 1, &[(-1, -1, -1)]), // 12: 9/dez
    (16, 11, 0, 5, 1, &[(-1, -1, -1)]), // 13: 16/dez
    (6, 0, 1, 5, 1, &[(-1, -1, -1)]), // 14: 6/jan (+1 ano)
    (13, 0, 1, 5, 1, &[(-1, -1, -1)]), // 15: 13/jan (+1 ano)
    (20, 0, 1, 5, 1, &[(-1, -1, -1)]), // 16: 20/jan (+1 ano)
    (27, 0, 1, 5, 1, &[(-1, -1, -1)]), // 17: 27/jan (+1 ano)
    (3, 1, 1, 5, 1, &[(-1, -1, -1)]), // 18: 3/fev (+1 ano)
    (10, 1, 1, 5, 1, &[(-1, -1, -1)]), // 19: 10/fev (+1 ano)
    (17, 1, 1, 5, 1, &[(-1, -1, -1)]), // 20: 17/fev (+1 ano)
    (24, 1, 1, 5, 1, &[(-1, -1, -1)]), // 21: 24/fev (+1 ano)
    (3, 2, 1, 5, 1, &[(-1, -1, -1)]), // 22: 3/mar (+1 ano)
    (10, 2, 1, 5, 1, &[(-1, -1, -1)]), // 23: 10/mar (+1 ano)
    (17, 2, 1, 5, 1, &[(-1, -1, -1)]), // 24: 17/mar (+1 ano)
    (24, 2, 1, 5, 1, &[(-1, -1, -1)]), // 25: 24/mar (+1 ano)
    (31, 2, 1, 5, 1, &[(-1, -1, -1)]), // 26: 31/mar (+1 ano)
    (7, 3, 1, 5, 1, &[(-1, -1, -1)]), // 27: 7/abr (+1 ano)
    (21, 3, 1, 5, 1, &[(-1, -1, -1)]), // 28: 21/abr (+1 ano)
    (28, 3, 1, 5, 1, &[(-1, -1, -1)]), // 29: 28/abr (+1 ano)
];

/// The stage 0 of both Greek divisions: a three-round league stage (code 0x44c) on the Sundays 13, 20 and 27 May.
pub(crate) const GREEK_PLAYOFF_ROUNDS: [SpaSecondScheduleRound; 3] = [
    (13, 4, 1, 6, 1, &[(-1, -1, -1)]), // 0: 13/mai (+1 ano)
    (20, 4, 1, 6, 1, &[(-1, -1, -1)]), // 1: 20/mai (+1 ano)
    (27, 4, 1, 6, 1, &[(-1, -1, -1)]), // 2: 27/mai (+1 ano)
];

/// The Turkish Second Division: 38 rounds (20 clubs, 2 legs) on Sundays, 13 August to 27 May, the break from 17
/// December to 21 January; round 17 (10 December) with code 2.
pub(crate) const TURKISH_SECOND_DIVISION_ROUNDS: [SpaSecondScheduleRound; 38] = [
    (13, 7, 0, 6, 1, &[(-1, -1, -1)]), // 0: 13/ago
    (20, 7, 0, 6, 1, &[(-1, -1, -1)]), // 1: 20/ago
    (27, 7, 0, 6, 1, &[(-1, -1, -1)]), // 2: 27/ago
    (3, 8, 0, 6, 1, &[(-1, -1, -1)]), // 3: 3/set
    (10, 8, 0, 6, 1, &[(-1, -1, -1)]), // 4: 10/set
    (17, 8, 0, 6, 1, &[(-1, -1, -1)]), // 5: 17/set
    (24, 8, 0, 6, 1, &[(-1, -1, -1)]), // 6: 24/set
    (1, 9, 0, 6, 1, &[(-1, -1, -1)]), // 7: 1/out
    (8, 9, 0, 6, 1, &[(-1, -1, -1)]), // 8: 8/out
    (15, 9, 0, 6, 1, &[(-1, -1, -1)]), // 9: 15/out
    (22, 9, 0, 6, 1, &[(-1, -1, -1)]), // 10: 22/out
    (29, 9, 0, 6, 1, &[(-1, -1, -1)]), // 11: 29/out
    (5, 10, 0, 6, 1, &[(-1, -1, -1)]), // 12: 5/nov
    (12, 10, 0, 6, 1, &[(-1, -1, -1)]), // 13: 12/nov
    (19, 10, 0, 6, 1, &[(-1, -1, -1)]), // 14: 19/nov
    (26, 10, 0, 6, 1, &[(-1, -1, -1)]), // 15: 26/nov
    (3, 11, 0, 6, 1, &[(-1, -1, -1)]), // 16: 3/dez
    (10, 11, 0, 6, 2, &[(-1, -1, -1)]), // 17: 10/dez
    (17, 11, 0, 6, 1, &[(-1, -1, -1)]), // 18: 17/dez
    (21, 0, 1, 6, 1, &[(-1, -1, -1)]), // 19: 21/jan (+1 ano)
    (28, 0, 1, 6, 1, &[(-1, -1, -1)]), // 20: 28/jan (+1 ano)
    (4, 1, 1, 6, 1, &[(-1, -1, -1)]), // 21: 4/fev (+1 ano)
    (11, 1, 1, 6, 1, &[(-1, -1, -1)]), // 22: 11/fev (+1 ano)
    (18, 1, 1, 6, 1, &[(-1, -1, -1)]), // 23: 18/fev (+1 ano)
    (25, 1, 1, 6, 1, &[(-1, -1, -1)]), // 24: 25/fev (+1 ano)
    (4, 2, 1, 6, 1, &[(-1, -1, -1)]), // 25: 4/mar (+1 ano)
    (11, 2, 1, 6, 1, &[(-1, -1, -1)]), // 26: 11/mar (+1 ano)
    (18, 2, 1, 6, 1, &[(-1, -1, -1)]), // 27: 18/mar (+1 ano)
    (25, 2, 1, 6, 1, &[(-1, -1, -1)]), // 28: 25/mar (+1 ano)
    (1, 3, 1, 6, 1, &[(-1, -1, -1)]), // 29: 1/abr (+1 ano)
    (8, 3, 1, 6, 1, &[(-1, -1, -1)]), // 30: 8/abr (+1 ano)
    (15, 3, 1, 6, 1, &[(-1, -1, -1)]), // 31: 15/abr (+1 ano)
    (22, 3, 1, 6, 1, &[(-1, -1, -1)]), // 32: 22/abr (+1 ano)
    (29, 3, 1, 6, 1, &[(-1, -1, -1)]), // 33: 29/abr (+1 ano)
    (6, 4, 1, 6, 1, &[(-1, -1, -1)]), // 34: 6/mai (+1 ano)
    (13, 4, 1, 6, 1, &[(-1, -1, -1)]), // 35: 13/mai (+1 ano)
    (20, 4, 1, 6, 1, &[(-1, -1, -1)]), // 36: 20/mai (+1 ano)
    (27, 4, 1, 6, 1, &[(-1, -1, -1)]), // 37: 27/mai (+1 ano)
];

/// Rule: the Greek First Division schedule: 26 rounds on Sundays from 17 September to 6 May (winter break from 17
/// December to 28 January); stage 0, a three-round league stage on 13, 20 and 27 May (code 0x44c), which the league
/// never reaches (it has no extra stage).
/// Original: `GrePrm::create_schedule` (0x5ea6d0, gre_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn gre_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    fixed_league_schedule(League(this), stage, rounds, extra, league, 0x9cdae4, &LeagueStageRounds { rounds: &GREEK_FIRST_DIVISION_ROUNDS, extra: 0, league: 1, row: 0xa4 },
        Some(&LeagueStageRounds { rounds: &GREEK_PLAYOFF_ROUNDS, extra: 0x44c, league: 1, row: 0x13f }), "mwd_17 greek first division calendar")
}

/// Rule: the Greek Second Division schedule: 30 rounds on Saturdays from 16 September to 28 April (break from 16
/// December to 6 January); stage 0 as the First Division's.
/// Original: `GreSecond::create_schedule` (0x5eb570, gre_second.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn gre_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    fixed_league_schedule(League(this), stage, rounds, extra, league, 0x9cdb1c, &LeagueStageRounds { rounds: &GREEK_SECOND_DIVISION_ROUNDS, extra: 0, league: 1, row: 0xa8 },
        Some(&LeagueStageRounds { rounds: &GREEK_PLAYOFF_ROUNDS, extra: 0x44c, league: 1, row: 0x143 }), "mwd_17 greek second division calendar")
}

/// Rule: the Turkish Second Division schedule: 38 rounds on Sundays from 13 August to 27 May (break from 17 December
/// to 21 January); no other stage.
/// Original: `TurSecond::create_schedule` (0x8fe9b0, tur_second.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn tur_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    fixed_league_schedule(League(this), stage, rounds, extra, league, 0xaa187c, &LeagueStageRounds { rounds: &TURKISH_SECOND_DIVISION_ROUNDS, extra: 0, league: 1, row: 0xa2 },
        None, "mwd_17 turkish second division calendar")
}

/// Side-by-side comparer for a league `create_schedule` whose every stage is a league round table (0x41-byte rounds):
/// as `compare_league_round_table`, but the written rounds compared as league rounds (`written_league_round_bytes`) at
/// every stage.
pub(crate) unsafe fn compare_league_stage_rounds(name: &str, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32,
    original: impl FnOnce() -> usize, copy: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let pl2 = if league.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, same, _, _) = crate::comparison::side_by_side(&[(0xb67a34usize, 4usize)], original, || copy(&mut r2, &mut e2, pl2));
    let outs = (a == 0 && b == 0) || (*rounds == r2 && *extra == e2 && (league.is_null() || *league == l2));
    let table = if a != 0 && b != 0 {
        let n = *rounds as usize;
        written_league_round_bytes(a, a, n) == written_league_round_bytes(b, a, n)
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(same && outs && table, || format!("{}(stage {}): outs {} table {}", name, stage as u8 as i8, outs, table));
    a
}

/// Comparer for `gre_prm_create_schedule` (side by side: the outputs and the written table).
pub(crate) unsafe extern "thiscall" fn compare_gre_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_GRE_PRM_CREATE_SCHEDULE);
    compare_league_stage_rounds("gre_prm_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| gre_prm_create_schedule(this, stage, r, e, l))
}

/// Comparer for `gre_second_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_gre_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_GRE_SECOND_CREATE_SCHEDULE);
    compare_league_stage_rounds("gre_second_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| gre_second_create_schedule(this, stage, r, e, l))
}

/// Comparer for `tur_second_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_TUR_SECOND_CREATE_SCHEDULE);
    compare_league_round_table("tur_second_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| tur_second_create_schedule(this, stage, r, e, l))
}

// ---------- batch 302: the Polish Cup and the Polish League Cup constructors (pol_cup.cpp, pol_lge_cup.cpp) ----------

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
/// The Polish League Cup: discipline rule 0x1f, 400,000 prize fund, the substitutes written after the round table.
pub(crate) const POLISH_LEAGUE_CUP: EastNationCup = EastNationCup {
    vtable: 0x96e414,
    discipline: 0x1f,
    entrants: 0x7cad30,
    prize: Some(0x61a80),
    late_substitutes: true,
    stage_vector: false,
    rank: true,
    file: 0xa785d4,
    rows: [0x3b, 0x4a, 0x51],
};

/// Rule: the Polish Cup at career creation: discipline rule 0x1f, 7 substitutes and 3 substitutions and, with no saved
/// state, the 52 entrants, a 600,000 prize fund, the seven rounds, the fixture tree, the statistics and the predicted
/// positions (`construct_east_nation_cup`).
/// Original: `PolCup::PolCup` (0x7c7d80, pol_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_pol_cup(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &POLISH_CUP, "mwd_18 polish cup new")
}

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

/// Rule: the Polish League Cup at career creation: discipline rule 0x1f and, with no saved state, its entrants, a
/// 400,000 prize fund, the rounds (7 substitutes and 3 substitutions set after them), the fixture tree, the statistics
/// and the predicted positions (`construct_east_nation_cup`).
/// Original: `PolLgeCup::PolLgeCup` (0x7ca4e0, pol_lge_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_pol_lge_cup(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &POLISH_LEAGUE_CUP, "mwd_18 polish league cup new")
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

/// Comparer for `construct_pol_lge_cup`.
pub(crate) unsafe extern "thiscall" fn compare_construct_pol_lge_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_POL_LGE_CUP);
    compare_east_nation_cup_ctor("construct_pol_lge_cup", Cup(this), year, record, &POLISH_LEAGUE_CUP, || o(this, year, record))
}

// ---------- batch 302: the Russian and Turkish cups: constructors and the Russian round table (rus_cup.cpp, tur_cup.cpp) ----------

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

/// Rule: the Russian Cup at career creation: discipline rule 0x27, 7 substitutes and 3 substitutions and, with no saved
/// state, no stage vector (+0x2c is 0), the 48 entrants, an 800,000 prize fund, the six rounds, the fixture tree, the
/// statistics and the predicted positions (`construct_east_nation_cup`).
/// Original: `RusCup::RusCup` (0x7e9210, rus_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_rus_cup(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &RUSSIAN_CUP, "mwd_19 russian cup new")
}

/// The Russian Cup: six single-leg rounds from June to May, a share of the fund per round won.
pub(crate) const RUSSIAN_CUP_ROUNDS: [FraSuperCupRound; 6] = [
    cql12r((0x17, 5, 0, 2), (0x1e, 5, 0, 2, 2, 0), 0x14, (0, 3, 0), 4, (0x20, 0x10, 0x20, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((0xa, 6, 0, 5), (0xc, 9, 0, 1, 2, 0), 0x1e, (8, 0x403, 0), 4, (0x20, 0x10, 0x10, 0x20), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((0xd, 9, 0, 2), (0xd, 0xa, 0, 5, 1, 0), 0x28, (0, 0x403, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((0xe, 0xa, 0, 6), (4, 3, 1, 1, 2, 0), 0x78, (0, 0x403, 0), 6, (8, 4, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((5, 3, 1, 2), (3, 4, 1, 2, 2, 0), 0x82, (0, 0x403, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((4, 4, 1, 3), (0x15, 4, 1, 1, 2, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
];

/// Rule: the Russian Cup has six single-leg rounds: the first round (32 teams, code 0x14) drawn 23/6, played 30/6; the
/// second round (32 teams, 16 new from entry 32: the Premier League [inference]; draw bits 8, rule 0x403) drawn 10/7,
/// played 12/10 (weekday 1); the round of 16 drawn 13/10, played 13/11 (weekday 5); the quarter-finals (category 6)
/// drawn 14/11, played 4/4 of the next year; the semi-finals drawn 5/4, played 3/5; the final drawn 4/5, played 21/5
/// at venue 4 [inference: a neutral ground]; a share of the fund per round won.
/// Original: `RusCup::create_round_table` (0x7e9530, rus_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rus_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    fra_super_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &RUSSIAN_CUP_ROUNDS, 0xa7ac58, 0xb1, "mwd_19 russian cup rounds", |_| {})
}

/// Rule: the Turkish Cup at career creation: discipline rule 0x26, 7 substitutes and 3 substitutions and, with no saved
/// state, the 64 entrants, a 600,000 prize fund, the seven rounds, the fixture tree, the statistics and the predicted
/// positions (`construct_east_nation_cup`).
/// Original: `TurCup::TurCup` (0x8fc3b0, tur_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_tur_cup(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &TURKISH_CUP, "mwd_19 turkish cup new")
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

/// Comparer for `construct_tur_cup`.
pub(crate) unsafe extern "thiscall" fn compare_construct_tur_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_TUR_CUP);
    compare_east_nation_cup_ctor("construct_tur_cup", Cup(this), year, record, &TURKISH_CUP, || o(this, year, record))
}

// ---------- batch 302: the Turkish Cup rounds and the Greek Super Cup (tur_cup.cpp, gre_super.cpp) ----------

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

/// The Greek Super Cup: one match between 2 clubs, drawn on 5/7 (weekday 5) and played on 2/8 (weekday 2, leg 2) at
/// venue 4 [inference: a neutral ground], tie rule 3, category 6, no prizes.
pub(crate) const GREEK_SUPER_CUP_ROUNDS: [FraSuperCupRound; 1] = [
    cql12r((5, 6, 0, 5), (2, 7, 0, 2, 2, 4), 0, (0, 3, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mwd20: the Greek Super Cup match
];

/// The one-round table of a national super cup (`fra_super_create_single_stage_round_table`) plus the round's word
/// +0x11 = 0, which the Greek and Polish super cups also write.
pub(crate) unsafe fn super_cup_round_table(this: Cup, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32, table: &[FraSuperCupRound; 1], file: usize, row: u32, branch: &'static str) -> usize {
    let t = fra_super_create_single_stage_round_table(this, stage, rounds, extra, out, table, file, row, branch, |_| {});
    if t != 0 {
        write_u16(t + 0x11, 0);
    }
    t
}

/// The Greek Super Cup: discipline rule 0x20, no prize fund, no predicted positions; the two clubs from
/// `GreSuper::select_teams` (0x5ec430).
pub(crate) const GREEK_SUPER_CUP: EastNationCup = EastNationCup {
    vtable: 0x96b6d0,
    discipline: 0x20,
    entrants: 0x5ec430,
    prize: None,
    late_substitutes: false,
    stage_vector: false,
    rank: false,
    file: 0x9cdb58,
    rows: [0x41, 0x4f, 0x56],
};

/// Rule: the Greek Super Cup at career creation: discipline rule 0x20, 7 substitutes and 3 substitutions and, with no
/// saved state, its two clubs (the league champion and the cup winner), the round, the fixture tree and the statistics
/// (`construct_east_nation_cup`, without prize fund nor predicted positions).
/// Original: `GreSuper::GreSuper` (0x5ec040, gre_super.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_gre_super(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &GREEK_SUPER_CUP, "mwd_20 greek super cup new")
}

/// Rule: the Greek Super Cup is a single match between 2 clubs, drawn on 5/7 and played on 2/8 at a neutral ground
/// [inference: venue 4], tie rule 3; no prizes; category 6.
/// Original: `GreSuper::create_round_table` (0x5ec2f0, gre_super.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn gre_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    super_cup_round_table(Cup(this), stage, rounds, extra, out, &GREEK_SUPER_CUP_ROUNDS, 0x9cdb58, 0x97, "mwd_20 greek super cup round")
}

/// Comparer for `tur_cup_create_round_table` (side by side, the written rounds compared).
pub(crate) unsafe extern "thiscall" fn compare_tur_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_TUR_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("tur_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| tur_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `construct_gre_super` (only the original runs).
pub(crate) unsafe extern "thiscall" fn compare_construct_gre_super(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_GRE_SUPER);
    compare_east_nation_cup_ctor("construct_gre_super", Cup(this), year, record, &GREEK_SUPER_CUP, || o(this, year, record))
}

/// Comparer for `gre_super_create_round_table` (side by side, the written round compared).
pub(crate) unsafe extern "thiscall" fn compare_gre_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_GRE_SUPER_CREATE_ROUND_TABLE);
    compare_cup_round_table("gre_super_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| gre_super_create_round_table(this, stage, r, e, s))
}

// ---------- batch 302: the Polish Super Cup (pol_super.cpp) ----------

/// The Polish Super Cup: discipline rule 0x1f, no prize fund, no predicted positions; the two clubs from
/// `PolSuper::select_teams` (0x7ccd10).
pub(crate) const POLISH_SUPER_CUP: EastNationCup = EastNationCup {
    vtable: 0x96e568,
    discipline: 0x1f,
    entrants: 0x7ccd10,
    prize: None,
    late_substitutes: false,
    stage_vector: false,
    rank: false,
    file: 0xa78648,
    rows: [0x41, 0x4f, 0x56],
};

/// The Polish Super Cup: one match between 2 clubs, drawn on 5/7 (weekday 5) and played on 18/7 (weekday 1, leg 2) at
/// venue 4 [inference: a neutral ground], tie rule 3, category 6, no prizes.
pub(crate) const POLISH_SUPER_CUP_ROUNDS: [FraSuperCupRound; 1] = [
    cql12r((5, 6, 0, 5), (0x12, 6, 0, 1, 2, 4), 0, (0, 3, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mwd21: the Polish Super Cup match
];

/// Rule: the Polish Super Cup at career creation: discipline rule 0x1f, 7 substitutes and 3 substitutions and, with no
/// saved state, its two clubs (the league champion and the cup winner), the round, the fixture tree and the statistics
/// (`construct_east_nation_cup`, without prize fund nor predicted positions).
/// Original: `PolSuper::PolSuper` (0x7cc920, pol_super.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_pol_super(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &POLISH_SUPER_CUP, "mwd_21 polish super cup new")
}

/// Rule: the Polish Super Cup is a single match between 2 clubs, drawn on 5/7 and played on 18/7 at a neutral ground
/// [inference: venue 4], tie rule 3; no prizes; category 6 (`super_cup_round_table`, with the word +0x11 = 0).
/// Original: `PolSuper::create_round_table` (0x7ccbd0, pol_super.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn pol_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    super_cup_round_table(Cup(this), stage, rounds, extra, out, &POLISH_SUPER_CUP_ROUNDS, 0xa78648, 0x97, "mwd_21 polish super cup round")
}

/// Rule: the Polish Super Cup season turnover: the finished edition's tables freed, its statistics archived, the year
/// + 1, the two clubs (`PolSuper::select_teams`), the round and the fixture tree; no predicted positions
/// (`usa_mls_all_stars_cup_start_new_season`; lines 0x135, 0x13c, 0x143 of pol_super.cpp).
/// Original: `PolSuper::start_new_season` (0x7ccf90, pol_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x7ccd10, 0xa78648, [0x135, 0x13c, 0x143], 1, false, "mwd_21 polish super cup turnover")
}

/// Comparer for `construct_pol_super` (only the original runs).
pub(crate) unsafe extern "thiscall" fn compare_construct_pol_super(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_POL_SUPER);
    compare_east_nation_cup_ctor("construct_pol_super", Cup(this), year, record, &POLISH_SUPER_CUP, || o(this, year, record))
}

/// Comparer for `pol_super_create_round_table` (side by side, the written round compared).
pub(crate) unsafe extern "thiscall" fn compare_pol_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_POL_SUPER_CREATE_ROUND_TABLE);
    compare_cup_round_table("pol_super_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| pol_super_create_round_table(this, stage, r, e, s))
}

/// Comparer for `pol_super_start_new_season` (only the original; the year went up by one).
pub(crate) unsafe extern "thiscall" fn compare_pol_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POL_SUPER_START_NEW_SEASON);
    compare_cup_new_season("pol_super_start_new_season", Cup(this), || o(this))
}

// ---------- batch 302: the Greek season turnover (gre_prm.cpp, gre_second.cpp) ----------

/// Rule: the Greek turnover, led by the First Division: the season reopened (+0x4c = 0); promotion and relegation along
/// the chain (virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN` with 0: push 0 -> thiscall, ret 4); the two
/// best Third Division clubs up and the Second Division's relegated down (`gre_prm_swap_second_and_third_division_clubs`,
/// ecx = this); the First Division rebuilt for the next year (`rebuild_croatian_league` without reopening the season:
/// lines 0x1dc, 0x1e3, 0x1e9, 0x1f0); the Second Division turned over (virtual `comp_vtable::START_NEW_SEASON`); then
/// the predicted positions, the board expectations and the officials (`finish_croatian_turnover`).
/// Original: `GrePrm::start_new_season` (0x5eaf50, gre_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_prm_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    chain(this.addr(), 0);
    let swap: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5eb120usize);
    swap(this.addr());
    covered!("mwd_22 greek first division turnover");
    if !rebuild_croatian_league(this, 0x5ead60, 0x9cdae4, [0x1dc, 0x1e3, 0x1e9, 0x1f0], false) {
        return 0;
    }
    bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_GREEK_SECOND_DIVISION), comp_vtable::START_NEW_SEASON); // mwd22: the Second Division turnover
    finish_croatian_turnover(this)
}

/// Rule: the Greek Second Division turnover (called by the First Division's): reopened, rebuilt for the next year
/// (`rebuild_croatian_league` without reopening: lines 0x1dc, 0x1e3, 0x1e9, 0x1f0 of gre_second.cpp), the predicted
/// positions, the board expectations and the officials.
/// Original: `GreSecond::start_new_season` (0x5ebe90, gre_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    covered!("mwd_22 greek second division turnover");
    if !rebuild_croatian_league(this, 0x5ebcb0, 0x9cdb1c, [0x1dc, 0x1e3, 0x1e9, 0x1f0], false) {
        return 0;
    }
    finish_croatian_turnover(this)
}

/// Rule: at the Greek turnover, the two best clubs of the Third Division (`KEY_CLUB_COMP_GREEK_LOWER_DIVISION`, sorted
/// by strength with the best nine shuffled) go up to the Second Division, and the Second Division's relegated clubs
/// (status 3) go down to the Third; with fewer than 2 candidates nobody moves.
///
/// A list of `G_CLUB_COUNT` pointers (`exe::crt_malloc`; out of memory: the box at line 0x2d5 and return) with the clubs
/// of the Third Division in database order (a 16-bit index; no nation check). With at least 2:
/// `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 3) (cdecl), the first two moved to the Second Division
/// (League::move_club_to_division(this, club, the Second Division's record, 1)); then each row i of the Second Division
/// (its club count read again on each step) with status 3 goes down (League::get_table_row, called twice as the
/// original, and League::relegate_club_to_division(this, club, the Third Division's record, 1)). The list is freed.
/// Original: `gre_prm_swap_second_and_third_division_clubs` (0x5eb120, gre_prm.cpp, thiscall: ecx = the First Division, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_prm_swap_second_and_third_division_clubs(this: usize) {
    let second = comp_by_global(KEY_CLUB_COMP_GREEK_SECOND_DIVISION);
    let list = exe::crt_malloc((read_usize(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0x9cdae4, 0x2d5);
        return;
    }
    let mut n: u32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GREEK_LOWER_DIVISION) {
            write_usize(list + (n as u16 as i16 as isize as usize).wrapping_mul(4), club);
            n = n.wrapping_add(1);
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    if (n as u16 as i16) >= 2 {
        covered!("mwd_22 greek third division up");
        exe::comp_util_sort_and_shuffle_top_clubs(list, n, 3); // mwd22: the best nine shuffled
        for k in 0..2 {
            exe::league_move_club_to_division(this, read_usize(list + 4 * k), comp_record_of(KEY_CLUB_COMP_GREEK_SECOND_DIVISION), 1);
        }
        let mut i = 0i32;
        while i < League(second).club_count() as i32 {
            if read_u8(exe::league_get_table_row(second, i) + league_table_row::STATUS) == 3 {
                let relegate: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x6831a0usize);
                let club = read_usize(exe::league_get_table_row(second, i));
                relegate(this, club, comp_record_of(KEY_CLUB_COMP_GREEK_LOWER_DIVISION), 1);
            }
            i += 1;
        }
    }
    exe::crt_free(list);
}

/// Comparer for `gre_prm_start_new_season` (only the original; the year went up by one).
pub(crate) unsafe extern "thiscall" fn compare_gre_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GRE_PRM_START_NEW_SEASON);
    compare_state_league_new_season("gre_prm_start_new_season", League(this), || o(this))
}

/// Comparer for `gre_second_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_gre_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GRE_SECOND_START_NEW_SEASON);
    compare_state_league_new_season("gre_second_start_new_season", League(this), || o(this))
}

/// Comparer for `gre_prm_swap_second_and_third_division_clubs`: only the original runs (it draws); with at least 2
/// Third Division clubs, the Second Division's clubs of status 3 must end in the Third Division.
pub(crate) unsafe extern "thiscall" fn compare_gre_prm_swap_second_and_third_division_clubs(this: usize) {
    let lower = comp_record_of(KEY_CLUB_COMP_GREEK_LOWER_DIVISION);
    let l = League(comp_by_global(KEY_CLUB_COMP_GREEK_SECOND_DIVISION));
    let candidates = (0..read_usize(G_CLUB_COUNT)).filter(|&i| {
        let d = read_usize(read_usize(G_CLUB_TABLE) + i * club::SIZE + club::DIVISION);
        d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GREEK_LOWER_DIVISION)
    }).count();
    let mut planned: Vec<usize> = Vec::new();
    if l.table() != 0 && candidates >= 2 {
        for i in 0..l.club_count() as i32 {
            let row = l.table() + i as usize * league_table_row::SIZE;
            if read_u8(row + league_table_row::STATUS) == 3 {
                planned.push(read_usize(row));
            }
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_GRE_PRM_SWAP_SECOND_AND_THIRD_DIVISION_CLUBS)(this);
    let wrong: Vec<usize> = planned.iter().copied().filter(|&c| read_usize(c + club::DIVISION) != lower).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("gre_prm_swap_second_and_third_division_clubs({:#x}): relegated clubs not in the Third Division {:x?}", this, wrong));
}

// ---------- batch 302: the Russian season turnover (rus_first.cpp, rus_prm.cpp) ----------

/// Rule: the Russian First Division turnover (called by the Premier League's): the season reopened (+0x4c = 0), the
/// table, the dates and the stage objects freed, the statistics archived, the next year, the rules, the table, the
/// fixtures, the order and the predicted positions (`rebuild_season`, lines 0x168, 0x16f, 0x175, 0x17c of
/// rus_first.cpp); then the board expectations and the officials (`set_expectations_and_renew_officials`).
/// Original: `RusFirst::start_new_season` (0x7ea8c0, rus_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    covered!("mwd_23 russian first division turnover");
    if !rebuild_season(this, 0x7ea6c0, 0xa7ac8c, [0x168, 0x16f, 0x175, 0x17c], 1) {
        return 0;
    }
    set_expectations_and_renew_officials(this)
}

/// Rule: the Russian turnover, led by the Premier League: the season reopened; promotion and relegation along the chain
/// (virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`: push 1); the exchange between the First Division and
/// the six second division zones (`rus_prm_exchange_first_and_second_division_clubs`, ecx = this); the Premier League
/// rebuilt for the next year (`rebuild_croatian_league`, reopening: lines 0x1b4, 0x1bb, 0x1c1, 0x1c8); the First
/// Division's division tier capped (League::limit_club_division_tier(1, 3, −3, 1)) and its turnover (virtual
/// `comp_vtable::START_NEW_SEASON`); then the predicted positions, the board expectations and the officials.
/// Original: `RusPrm::start_new_season` (0x7eb800, rus_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_prm_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    chain(this.addr(), 1);
    let exchange: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7ebe20usize);
    exchange(this.addr()); // mwd23: the First and second division exchange
    covered!("mwd_23 russian premier league turnover");
    if !rebuild_croatian_league(this, 0x7eb610, 0xa7acc8, [0x1b4, 0x1bb, 0x1c1, 0x1c8], true) {
        return 0;
    }
    let first = comp_by_global(KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION);
    exe::league_limit_club_division_tier(first, 1, 3, -3, 1);
    bra_nat_first_call_virtual_no_args(first, comp_vtable::START_NEW_SEASON);
    finish_croatian_turnover(this)
}

/// The six Russian second division zones, as pairs whose best club goes up: west and central, south and Volga, east and
/// Ural (each pair checked in that order).
pub(crate) const RUSSIAN_ZONE_PAIRS: [[usize; 2]; 3] = [
    [KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_WEST, KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_CENTRAL],
    [KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_SOUTH, KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_VOLGA],
    [KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_EAST, KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_URAL],
];

/// Rule: at the Russian turnover, the best club of each pair of second division zones (west/central, south/Volga,
/// east/Ural) goes up to the First Division, and the First Division's last three go down: the last to the central
/// zone, the second last to the south zone, the third last to the Ural zone; with fewer than 3 candidates nobody moves.
///
/// The zone records are `G_CLUB_COMP_TABLE` + 0x6b × id (`comp_record_of`). A list of `G_CLUB_COUNT` pointers
/// (`exe::crt_malloc`; out of memory: the box at line 0x2bd and return) with the clubs (the first [`G_CLUB_COUNT`] − 2 ×
/// [`G_NATION_COUNT`]) whose division pointer (`club::DIVISION`) is one of the six zone records, in database order
/// (16-bit index). Fewer than 3: the list freed and return. Otherwise `exe::comp_util_sort_and_shuffle_top_clubs`(list,
/// n, 3) and, along the list until all three pairs have a club, the first club of each pair is moved to the First
/// Division (League::move_club_to_division(this, club, the First Division's record, 1)); then the First Division's rows
/// `club_count` − 1, − 2 and − 3 (League::get_table_row) go down (League::relegate_club_to_division(this, club, zone, 1))
/// to the central, south and Ural zones, whatever their status; the list is freed.
/// Original: `rus_prm_exchange_first_and_second_division_clubs` (0x7ebe20, rus_prm.cpp, thiscall: ecx = the Premier League, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_prm_exchange_first_and_second_division_clubs(this: usize) {
    let first = comp_by_global(KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION);
    let list = exe::crt_malloc((read_usize(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0xa7acc8, 0x2bd);
        return;
    }
    let zones: [[usize; 2]; 3] = RUSSIAN_ZONE_PAIRS.map(|p| p.map(|g| unsafe { comp_record_of(g) }));
    let limit = || (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let mut n: u32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < limit() {
        let d = read_usize(club + club::DIVISION);
        if zones.iter().any(|p| d == p[0] || d == p[1]) {
            write_usize(list + (n as u16 as i16 as isize as usize).wrapping_mul(4), club);
            n = n.wrapping_add(1);
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    if (n as u16 as i16) < 3 {
        exe::crt_free(list);
        return;
    }
    covered!("mwd_23 russian zones up");
    exe::comp_util_sort_and_shuffle_top_clubs(list, n, 3); // mwd23: the best nine shuffled
    let mut taken = [false; 3];
    let m = n as u16 as i16 as i32;
    let mut k = 0i32;
    while k < m {
        if taken.iter().all(|&t| t) {
            break;
        }
        let c = read_usize(list + 4 * k as usize);
        let d = read_usize(c + club::DIVISION);
        if let Some(z) = zones.iter().position(|p| d == p[0] || d == p[1]) {
            if !taken[z] {
                taken[z] = true;
                exe::league_move_club_to_division(this, c, comp_record_of(KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION), 1); // mwd23: the best of the pair up
            }
        } else {
            exe::league_move_club_to_division(this, c, comp_record_of(KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION), 1);
        }
        k += 1;
    }
    let relegate: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x6831a0usize);
    for (back, zone) in [(1i32, zones[0][1]), (2, zones[1][0]), (3, zones[2][1])] {
        let row = exe::league_get_table_row(first, League(first).club_count() as i32 - back);
        relegate(this, read_usize(row), zone, 1);
    }
    exe::crt_free(list);
}

/// Comparer for `rus_first_start_new_season` (only the original; the year went up by one).
pub(crate) unsafe extern "thiscall" fn compare_rus_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_RUS_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("rus_first_start_new_season", League(this), || o(this))
}

/// Comparer for `rus_prm_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_rus_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_RUS_PRM_START_NEW_SEASON);
    compare_state_league_new_season("rus_prm_start_new_season", League(this), || o(this))
}

/// Comparer for `rus_prm_exchange_first_and_second_division_clubs`: only the original runs (it draws); with at least 3
/// candidates, the First Division's last three clubs must end in the central, south and Ural zones.
pub(crate) unsafe extern "thiscall" fn compare_rus_prm_exchange_first_and_second_division_clubs(this: usize) {
    let zones: [[usize; 2]; 3] = RUSSIAN_ZONE_PAIRS.map(|p| p.map(|g| unsafe { comp_record_of(g) }));
    let limit = (read_usize(G_CLUB_COUNT) as i32 - 2 * read_usize(G_NATION_COUNT) as i32).max(0) as usize;
    let candidates = (0..limit).filter(|&i| {
        let d = read_usize(read_usize(G_CLUB_TABLE) + i * club::SIZE + club::DIVISION);
        zones.iter().any(|p| d == p[0] || d == p[1])
    }).count();
    let first = League(comp_by_global(KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION));
    let n = first.club_count() as usize;
    let plan: Vec<(usize, usize)> = if candidates >= 3 && first.table() != 0 && n >= 3 {
        vec![(read_usize(first.table() + (n - 1) * 0x3b), zones[0][1]), (read_usize(first.table() + (n - 2) * 0x3b), zones[1][0]), (read_usize(first.table() + (n - 3) * 0x3b), zones[2][1])]
    } else {
        Vec::new()
    };
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_RUS_PRM_EXCHANGE_FIRST_AND_SECOND_DIVISION_CLUBS)(this);
    let wrong: Vec<(usize, usize)> = plan.iter().copied().filter(|&(c, z)| read_usize(c + club::DIVISION) != z).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("rus_prm_exchange_first_and_second_division_clubs({:#x}): relegated clubs not in their zone {:x?}", this, wrong));
}

// ---------- batch 302: the Turkish season turnover (tur_first.cpp, tur_second.cpp) ----------

/// The rebuild of the Turkish turnovers, which keep the stage fields: League::free_table(this, 0); the round dates
/// (+0xba) freed and cleared; the statistics archived (push 1) when there are any; the year (+0x40) + 1; then, each
/// with the box of its line (`rows`) and stopping when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`),
/// the table (`table`, direct call), the fixtures (League::create_fixtures) and the order (League::sort_table: push 0).
/// Unlike the Croatian rebuild, +0xc, +0x2c, +0x30 and +0x4c are not touched.
pub(crate) unsafe fn rebuild_turkish_league(this: League, table: usize, file: usize, rows: [u32; 4]) -> bool {
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    if this.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(this.comp_stats(), 1);
    }
    this.set_year(this.year().wrapping_add(1));
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, rows[0]);
        return false;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(table);
    if table(this.addr()) == 0 {
        show_error_at(file, rows[1]);
        return false;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, rows[2]);
        return false;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, rows[3]);
        return false;
    }
    true
}

/// Rule: the Turkish turnover, led by the First Division: the season reopened (+0x4c = 0); promotion and relegation
/// (virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`: push 1, `tur_first_process_promotion_relegation_chain`);
/// the ten best clubs of the lowest division up into the second division category B and its relegated clubs down
/// (`tur_first_promote_lower_clubs_to_third_tier`, ecx = this); the third tier reshuffle (0x8fe580, ecx = this, not
/// ported); the First Division rebuilt for the next year (`rebuild_turkish_league`, lines 0x213, 0x21a, 0x220, 0x227);
/// the division tier of the clubs of both second divisions capped (League::limit_club_division_tier(1, 3, −3, 1),
/// category A then B) and both turned over (virtual `comp_vtable::START_NEW_SEASON`); then the predicted positions, the
/// board expectations and the officials (`finish_croatian_turnover`).
/// Original: `TurFirst::start_new_season` (0x8fe040, tur_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    chain(this.addr(), 1);
    for f in [0x8fe3a0usize, 0x8fe580] {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
        f(this.addr());
    }
    covered!("mwd_24 turkish first division turnover");
    if !rebuild_turkish_league(this, 0x8fddc0, 0xaa1840, [0x213, 0x21a, 0x220, 0x227]) {
        return 0;
    }
    let (a, b) = (comp_by_global(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A), comp_by_global(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B));
    exe::league_limit_club_division_tier(a, 1, 3, -3, 1);
    exe::league_limit_club_division_tier(b, 1, 3, -3, 1);
    bra_nat_first_call_virtual_no_args(a, comp_vtable::START_NEW_SEASON);
    bra_nat_first_call_virtual_no_args(b, comp_vtable::START_NEW_SEASON);
    finish_croatian_turnover(this)
}

/// Rule: the Turkish Second Division turnover (called by the First Division's): the season reopened, rebuilt for the
/// next year (`rebuild_turkish_league`, lines 0x1bf, 0x1c6, 0x1cc, 0x1d3 of tur_second.cpp), the predicted positions,
/// the board expectations and the officials.
/// Original: `TurSecond::start_new_season` (0x8ff330, tur_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    covered!("mwd_24 turkish second division turnover");
    if !rebuild_turkish_league(this, 0x8ff150, 0xaa187c, [0x1bf, 0x1c6, 0x1cc, 0x1d3]) {
        return 0;
    }
    finish_croatian_turnover(this)
}

/// Rule: Turkish promotion and relegation: the final positions of the First Division and both second divisions are
/// recorded (virtual `league_vtable::RECORD_FINAL_POSITIONS`); then the marked clubs are swapped between the First
/// Division and the second division category A (in pairs), and between category A and category B and each of
/// category B's four groups (its stages 0..3; as a block).
///
/// League::promote_and_relegate (`exe::league_promote_and_relegate`(this, above, below, 1, pairs, −1, −1): 6 pushes ->
/// thiscall, ret 0x18): First Division / category A with pairs 1, category A / category B with 0, then category A /
/// each of category B's stages `[+0xc][0..3]` with 0. Returns the last call's eax; the argument is not read.
/// Original: `TurFirst::process_promotion_relegation_chain` (0x8fe250, tur_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn tur_first_process_promotion_relegation_chain(this: usize, _pairs: u32) -> u32 {
    let (a, b) = (comp_by_global(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A), comp_by_global(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B));
    bra_nat_first_call_virtual_no_args(this, league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(a, league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(b, league_vtable::RECORD_FINAL_POSITIONS);
    covered!("mwd_24 turkish promotion and relegation");
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION), a, 1, 1, -1, -1); // mwd24: the top flight and category A
    let mut r = exe::league_promote_and_relegate(this, a, b, 1, 0, -1, -1);
    for k in 0..4 { // mwd24: the four groups of category B
        r = exe::league_promote_and_relegate(this, a, read_usize(read_usize(b + league::STAGES) + 4 * k), 1, 0, -1, -1);
    }
    r
}

/// Comparer for `tur_first_start_new_season` (only the original; the year went up by one).
pub(crate) unsafe extern "thiscall" fn compare_tur_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_TUR_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("tur_first_start_new_season", League(this), || o(this))
}

/// Comparer for `tur_second_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_TUR_SECOND_START_NEW_SEASON);
    compare_state_league_new_season("tur_second_start_new_season", League(this), || o(this))
}

/// Comparer for `tur_first_process_promotion_relegation_chain`: only the original (it moves clubs, as the Brazilian
/// chain's comparer).
pub(crate) unsafe extern "thiscall" fn compare_tur_first_process_promotion_relegation_chain(this: usize, pairs: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_TUR_FIRST_PROCESS_PROMOTION_RELEGATION_CHAIN)(this, pairs)
}

// ---------- batch 302: the Turkish First Division table and the lowest division's promotion (tur_first.cpp) ----------

/// The five groups of the Turkish second division category B in the database (`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G1`
/// to `_G5`).
pub(crate) const TURKISH_CATEGORY_B_GROUPS: [usize; 5] = [KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G1, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G2,
    KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G3, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G4, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G5];

/// Rule: the Turkish First Division has 18 clubs, those of the top flight (`KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION`) in
/// database order; when full, no prize fund (prize mode 2) and TV pays 1,000,000 to each club. In 2001 it also moves
/// the clubs of the five category B groups into category B itself, keeping the group in the club's reserve division.
///
/// `club_count` = 18; in 2001, for the first [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`] clubs whose national division
/// (`club::DIVISION`) has the id of one of `TURKISH_CATEGORY_B_GROUPS`: `club::RESERVE_DIVISION` = that division and
/// `club::DIVISION` = the record of category B (`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B`). Then the table
/// (`fill_league_table_rows`: the box at line 0x1a4); full: +0x28 = 0, `prize_mode` = 2 and League::pay_tv_rights (push
/// 0; push 1000000) and 1; otherwise 0.
/// Original: `TurFirst::create_table` (0x8fddc0, tur_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_first_create_table(this: usize) -> u32 {
    let l = League(this);
    l.set_club_count(0x12);
    if l.year() == 0x7d1 {
        let mut club = read_usize(G_CLUB_TABLE);
        let mut i = 0i32;
        while i < (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2)) {
            let d = read_usize(club + club::DIVISION);
            if d != 0 && TURKISH_CATEGORY_B_GROUPS.iter().any(|&g| read_usize(d) == read_usize(g)) {
                covered!("mwd_25 turkish category b groups merged");
                write_usize(club + club::RESERVE_DIVISION, d);
                write_usize(club + club::DIVISION, comp_record_of(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B)); // mwd25: into category B
            }
            i += 1;
            club = club.wrapping_add(club::SIZE);
        }
    }
    if !fill_league_table_rows(l, 0x12, 0xaa1840, 0x1a4, KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION) {
        return 0;
    }
    l.set_prize_fund(0);
    l.set_prize_mode(2);
    eng_prm_pay_tv_rights(this, 0xf4240, 0); // mwd25: the Turkish top flight TV money
    1
}

/// Rule: at the Turkish turnover, the ten best clubs of the lowest division (`KEY_CLUB_COMP_TURKISH_LOWER_DIVISION`,
/// sorted by strength with the best 30 shuffled) go up into the second division category B, and the clubs of status 3
/// in category B and in its four group stages go down to the lowest division, their reserve division cleared; with
/// fewer than 10 candidates nobody moves.
///
/// A list of `G_CLUB_COUNT` pointers (`exe::crt_malloc`; out of memory: the box at line 0x27c and return) with the clubs
/// that have a nation (`club::NATION` set, not compared) and whose division is the lowest one, in database order
/// (16-bit index). Fewer than 10: the list freed and return. Otherwise `exe::comp_util_sort_and_shuffle_top_clubs`(list,
/// n, 10), the first ten moved to category B (League::move_club_to_division(this, club, its record, 1)); then for
/// category B (g = −1) and its stages g = 0..3 (`[+0xc][g]`), each row of status 3 goes down
/// (League::relegate_club_to_division(this, club, the lowest division's record, 1)) and that club's
/// `club::RESERVE_DIVISION` = 0. The list is freed.
/// Original: `tur_first_promote_lower_clubs_to_third_tier` (0x8fe3a0, tur_first.cpp, thiscall: ecx = the First Division, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_first_promote_lower_clubs_to_third_tier(this: usize) {
    let list = exe::crt_malloc((read_usize(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0xaa1840, 0x27c);
        return;
    }
    let mut n: u32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(club + club::NATION) != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_TURKISH_LOWER_DIVISION) {
            write_usize(list + (n as u16 as i16 as isize as usize).wrapping_mul(4), club);
            n = n.wrapping_add(1);
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    if (n as u16 as i16) < 10 {
        exe::crt_free(list);
        return;
    }
    covered!("mwd_25 turkish lowest division up");
    exe::comp_util_sort_and_shuffle_top_clubs(list, n, 10); // mwd25: the best thirty shuffled
    for k in 0..10 {
        exe::league_move_club_to_division(this, read_usize(list + 4 * k), comp_record_of(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B), 1);
    }
    let relegate: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x6831a0usize);
    let b = comp_by_global(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B);
    for g in -1i8..4 {
        let stage = League(if g == -1 { b } else { read_usize(read_usize(b + league::STAGES) + 4 * g as usize) });
        let mut i = 0i32;
        while i < stage.club_count() as i32 {
            let row = stage.table() + i as usize * league_table_row::SIZE;
            if read_u8(row + league_table_row::STATUS) == 3 {
                relegate(this, read_usize(row), comp_record_of(KEY_CLUB_COMP_TURKISH_LOWER_DIVISION), 1);
                write_usize(read_usize(stage.table() + i as usize * league_table_row::SIZE) + club::RESERVE_DIVISION, 0);
            }
            i += 1;
        }
    }
    exe::crt_free(list);
}

/// Comparer for `tur_first_create_table`: only the original runs (it moves clubs in 2001); the planned top-flight clubs
/// (`compare_division_table`) and, when full, prize mode 2 with no fund.
pub(crate) unsafe extern "thiscall" fn compare_tur_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_TUR_FIRST_CREATE_TABLE);
    let r = compare_division_table("tur_first_create_table", League(this), 0x12, KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION, || o(this));
    if r == 1 {
        let l = League(this);
        SCOREBOARD.record(l.prize_fund() == 0 && l.prize_mode() == 2, || format!("tur_first_create_table({:#x}) prize fund {} mode {}", this, l.prize_fund(), l.prize_mode()));
    }
    r
}

/// Comparer for `tur_first_promote_lower_clubs_to_third_tier`: only the original runs (it draws); with at least 10
/// candidates, the clubs of status 3 in category B and its four groups must end in the lowest division with no reserve
/// division.
pub(crate) unsafe extern "thiscall" fn compare_tur_first_promote_lower_clubs_to_third_tier(this: usize) {
    let candidates = (0..read_usize(G_CLUB_COUNT)).filter(|&i| {
        let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        let d = read_usize(c + club::DIVISION);
        d != 0 && read_usize(c + club::NATION) != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_TURKISH_LOWER_DIVISION)
    }).count();
    let mut planned: Vec<usize> = Vec::new();
    let b = comp_by_global(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B);
    if candidates >= 10 {
        for g in -1i32..4 {
            let s = League(if g == -1 { b } else { read_usize(read_usize(b + league::STAGES) + 4 * g as usize) });
            if s.addr() != 0 && s.table() != 0 {
                for i in 0..s.club_count() as usize {
                    let row = s.table() + i * league_table_row::SIZE;
                    if read_u8(row + league_table_row::STATUS) == 3 {
                        planned.push(read_usize(row));
                    }
                }
            }
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TUR_FIRST_PROMOTE_LOWER_CLUBS_TO_THIRD_TIER)(this);
    let lower = comp_record_of(KEY_CLUB_COMP_TURKISH_LOWER_DIVISION);
    let wrong: Vec<usize> = planned.iter().copied().filter(|&c| read_usize(c + club::DIVISION) != lower || read_usize(c + club::RESERVE_DIVISION) != 0).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("tur_first_promote_lower_clubs_to_third_tier({:#x}): relegated clubs not in the lowest division {:x?}", this, wrong));
}

// ---------- batch 302: the Greek and Polish super cup clubs (gre_super.cpp, pol_super.cpp) ----------

/// The first club (in database order, `club::SIZE` records of `G_CLUB_TABLE`) whose nation pointer (`club::NATION`) is
/// `nation` and whose database index is not `skip` (the id of an already chosen club); 0 when there is none.
pub(crate) unsafe fn first_club_of_nation(nation: usize, skip: Option<u32>) -> usize {
    let n = read_usize(G_CLUB_COUNT) as i32;
    let mut i = 0i32;
    while i < n {
        let c = read_usize(G_CLUB_TABLE) + i as usize * club::SIZE;
        if read_usize(c + club::NATION) == nation && skip.map_or(true, |s| i as u32 != s) {
            return c;
        }
        i += 1;
    }
    0
}

/// The body of `GreSuper::select_teams` and `PolSuper::select_teams` (thiscall, ret 0): the league champion and the cup
/// winner.
///
/// `team_count` = 2; the list (`exe::crt_malloc` of 12 into `cup::TEAM_TABLE`; out of memory: the box at `rows[0]` and
/// −1). Entry 0 = the latest champion of `league` (comp_util_get_latest_placed_club `exe::comp_util_get_latest_placed_club`(record,
/// 0): 2 pushes; add esp, 8 -> cdecl); none: the first club of the competition's nation (the record's +0x5d;
/// `first_club_of_nation`). Entry 1 = the latest winner of `cup`; none: the first club of the nation whose index is not
/// entry 0's id (entry 0's record +0, read even when entry 0 is null, as the original). The same club in both: entry 1
/// = the latest runner-up (place 1) of `double`; none: the box at `rows[1]` and 0. The class and status bytes (+4, +5,
/// +0xa, +0xb) = 0. Returns 1.
pub(crate) unsafe fn select_league_and_cup_winners(this: Cup, league: usize, cup: usize, double: usize, file: usize, rows: [u32; 2]) -> (i32, i32) {
    this.set_team_count(2);
    let l = exe::crt_malloc(0xc);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(file, rows[0]);
        return (-1, 0);
    }
    let nation = read_usize(read_usize(this.addr() + 4) + competition::NATION);
    write_usize(l, exe::comp_util_get_latest_placed_club(comp_record_of(league), 0)); // mwd26: the league champion
    if read_usize(l) == 0 {
        let c = first_club_of_nation(nation, None);
        if c != 0 {
            write_usize(l, c);
        }
    }
    write_usize(l + 6, exe::comp_util_get_latest_placed_club(comp_record_of(cup), 0));
    if read_usize(l + 6) == 0 {
        let c = first_club_of_nation(nation, Some(read_u32(read_usize(l))));
        if c != 0 {
            write_usize(l + 6, c);
        }
    }
    if read_usize(l) == read_usize(l + 6) {
        covered!("mwd_26 super cup double");
        write_usize(l + 6, exe::comp_util_get_latest_placed_club(comp_record_of(double), 1));
        if read_usize(l + 6) == 0 {
            show_error_at(file, rows[1]);
            return (0, 0);
        }
    }
    covered!("mwd_26 super cup clubs");
    for o in [4usize, 5, 0xa, 0xb] {
        write_u8(l + o, 0);
    }
    (1, 0)
}

/// Rule: the Greek Super Cup is between the Greek First Division champion (`KEY_CLUB_COMP_GREEK_PREMIER_DIVISION`)
/// and the Greek Cup winner (`KEY_CLUB_COMP_GREEK_CUP`); a missing one is replaced by the first club of the nation; if
/// the same club won both, the First Division runner-up comes in; if there is none, the error box
/// (`select_league_and_cup_winners`, lines 0xc3 and 0xf1).
/// Original: `GreSuper::select_teams` (0x5ec430, gre_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_super_select_teams(this: usize) -> i32 {
    select_league_and_cup_winners(Cup(this), KEY_CLUB_COMP_GREEK_PREMIER_DIVISION, KEY_CLUB_COMP_GREEK_CUP, KEY_CLUB_COMP_GREEK_PREMIER_DIVISION, 0x9cdb58, [0xc3, 0xf1]).0
}

/// Rule: the Polish Super Cup is between the Polish First Division champion (`KEY_CLUB_COMP_POLISH_FIRST_DIVISION`)
/// and the Polish Cup winner (`KEY_CLUB_COMP_POLISH_FA_CUP`); a missing one is replaced by the first club of the nation;
/// if the same club won both, the Polish Cup runner-up comes in; if there is none, the error box (lines 0xc2 and 0xf0).
/// Original: `PolSuper::select_teams` (0x7ccd10, pol_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_super_select_teams(this: usize) -> i32 {
    select_league_and_cup_winners(Cup(this), KEY_CLUB_COMP_POLISH_FIRST_DIVISION, KEY_CLUB_COMP_POLISH_FA_CUP, KEY_CLUB_COMP_POLISH_FA_CUP, 0xa78648, [0xc2, 0xf0]).0
}

/// Comparer for `gre_super_select_teams` (side by side: `compare_team_list`).
pub(crate) unsafe extern "thiscall" fn compare_gre_super_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_GRE_SUPER_SELECT_TEAMS);
    compare_team_list("gre_super_select_teams", Cup(this), || o(this), || select_league_and_cup_winners(Cup(this), KEY_CLUB_COMP_GREEK_PREMIER_DIVISION, KEY_CLUB_COMP_GREEK_CUP,
        KEY_CLUB_COMP_GREEK_PREMIER_DIVISION, 0x9cdb58, [0xc3, 0xf1]))
}

/// Comparer for `pol_super_select_teams`.
pub(crate) unsafe extern "thiscall" fn compare_pol_super_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_POL_SUPER_SELECT_TEAMS);
    compare_team_list("pol_super_select_teams", Cup(this), || o(this), || select_league_and_cup_winners(Cup(this), KEY_CLUB_COMP_POLISH_FIRST_DIVISION, KEY_CLUB_COMP_POLISH_FA_CUP,
        KEY_CLUB_COMP_POLISH_FA_CUP, 0xa78648, [0xc2, 0xf0]))
}
