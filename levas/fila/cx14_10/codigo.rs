// ---------- cx14_10: the schedules of the Turkish First Division and of the second division category B (tur_first.cpp, tur_second_b.cpp) ----------

/// The schedule of the Turkish First Division: stage −1 only, 34 rounds on Sundays from 13 August to 20 May (winter
/// break), most with TV slots (`TURKISH_FIRST_DIVISION_ROUNDS`, through `fixed_league_schedule`: `*league` = 1 when
/// given, 34 rounds, extra 0, the table of 0x8a2 bytes; out of memory: the box of tur_first.cpp (0xaa1840) line 0xa2
/// and 0). Any other stage: 0, nothing written.
/// Original: `TurFirst::create_schedule` (0x8fd220, tur_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn tur_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    fixed_league_schedule(League(this), stage, rounds, extra, league, 0xaa1840, &LeagueStageRounds { rounds: &TURKISH_FIRST_DIVISION_ROUNDS, extra: 0, league: 1, row: 0xa2 },
        None, "cx14_10 turkish first division calendar")
}

/// Comparer for `tur_first_create_schedule` (`compare_league_round_table`: the league rounds at stage −1).
pub(crate) unsafe extern "thiscall" fn compare_tur_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_league_round_table("tur_first_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| tur_first_create_schedule(this, stage, r, e, l))
}

/// The round table of a first phase group of category B (stage −1 to 3): the dates of `CATEGORY_B_GROUP_DATES` (the
/// midweek ones only when the group has 22 rounds, group 2 = stage 0), then each round's TV slots: (5, 1, 1) and the
/// end mark when (stage + round + 1) is a multiple of 5, otherwise only the end mark; with 22 rounds, rounds 1, 5, 9
/// and 15 never get the slot (a defect of the original: the midweek rounds are 3, 5, 11 and 15, so the Sunday rounds
/// 1 and 9 lose it). Returns the number of rounds written.
pub(crate) unsafe fn category_b_group_rounds(t: usize, stage: i8, count: i16, year: u32) -> i16 {
    let mut n = 0i32;
    for &(day, month, years, week, code, midweek) in CATEGORY_B_GROUP_DATES.iter() {
        if midweek && count != 0x16 {
            continue;
        }
        exe::league_round_set_date(t, n, day as i32, month as i32, years as i32, week as i32, code as i32, year, 0);
        n += 1;
    }
    for k in 0..n {
        let no_tv = matches!(k, 1 | 5 | 9 | 15) && n == 0x16;
        if !no_tv && (stage as i32 + k + 1) % 5 == 0 { // cx14_10: the group's TV rounds
            exe::league_round_set_match_slot(t, k, 0, 5, 1, 1, 0);
            exe::league_round_set_match_slot(t, k, 1, -1, -1, -1, 0);
        } else {
            exe::league_round_set_match_slot(t, k, 0, -1, -1, -1, 0);
        }
    }
    n as i16
}

/// The schedule of the Turkish second division category B, by stage (signed byte):
/// - a first phase group (below 4): `*league` = 1 (when given); a table of 18 rounds (22 for group 2, stage 0;
///   `exe::crt_malloc` of 0x41 a round; out of memory: the box of line 0xb4 and 0, the outputs not written); the extra
///   1001 + stage; the rounds (`category_b_group_rounds`, from 13 August to 10 December); then `*rounds`;
/// - the promotion group (4): `*league` = 1, 18 rounds, extra 0x458 (before the table, 0x492 bytes; box 0xfd); the
///   rounds of `CATEGORY_B_PROMOTION_GROUP_ROUNDS` (14 January to 20 May, every one with a TV slot);
/// - a classification group (5 to 9): `*league` = 1, 14 rounds, extra 0x470 + stage (table 0x38e; box 0x134); the
///   rounds of `CATEGORY_B_CLASSIFICATION_GROUP_ROUNDS` (11 February to 20 May, no TV slots);
/// - the play-off (10): `*league` = 0, 3 rounds, extra 0xa0 (table 0x138; box 0x16c); the cup rounds of
///   `CATEGORY_B_PLAY_OFF` (`write_play_off_round`);
/// - later stages: 0, nothing written.
/// The year is pushed with stack leftovers above the word in the original; the callees read the word.
/// Original: `TurSecondB::create_schedule` (0x8ff7e0, tur_second_b.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn tur_second_b_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let l = League(this);
    let s = stage as u8 as i8;
    let year = (l.year() as u16) as u32;
    if s < 4 {
        let count: i16 = if s == 0 { 0x16 } else { 0x12 };
        if !league.is_null() {
            *league = 1;
        }
        let t = exe::crt_malloc((count as i32 as u32).wrapping_mul(0x41));
        if t == 0 {
            show_error_at(TURKISH_CATEGORY_B_SOURCE, 0xb4);
            return 0;
        }
        *extra = (s as i16).wrapping_add(0x3e9);
        covered!("cx14_10 category b group calendar");
        *rounds = category_b_group_rounds(t, s, count, year);
        return t;
    }
    let (n, e, size, line, is_league): (i16, i16, u32, u32, bool) = match s {
        4 => (0x12, 0x458, 0x492, 0xfd, true),
        5..=9 => (0xe, (s as i16).wrapping_add(0x470), 0x38e, 0x134, true),
        10 => (3, 0xa0, 0x138, 0x16c, false),
        _ => return 0,
    };
    if !league.is_null() {
        *league = is_league as i32;
    }
    *rounds = n;
    *extra = e;
    let t = exe::crt_malloc(size);
    if t == 0 {
        show_error_at(TURKISH_CATEGORY_B_SOURCE, line);
        return 0;
    }
    match s {
        4 => {
            covered!("cx14_10 category b promotion group calendar");
            spa_second_set_league_rounds(t, year, 0, &CATEGORY_B_PROMOTION_GROUP_ROUNDS);
        }
        10 => {
            covered!("cx14_10 category b play-off calendar");
            for (i, r) in CATEGORY_B_PLAY_OFF.iter().enumerate() {
                write_play_off_round(l, t, i as u32, r);
            }
        }
        _ => {
            covered!("cx14_10 category b classification calendar");
            spa_second_set_league_rounds(t, year, 0, &CATEGORY_B_CLASSIFICATION_GROUP_ROUNDS);
        }
    }
    t
}

/// The side-by-side comparer of a `create_schedule` whose stages mix league and cup round tables: as
/// `compare_league_round_table`, but the league bytes (`written_league_round_bytes`) where `league_rounds(stage)` and
/// the cup bytes (`eng_cc_cup_written_round_bytes`) elsewhere; the copy's table freed.
pub(crate) unsafe fn compare_mixed_round_table(name: &str, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32, league_rounds: impl Fn(i8) -> bool,
    original: impl FnOnce() -> usize, copy: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let pl2 = if league.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, same, _, _) = crate::comparison::side_by_side(&[(0xb67a34usize, 4usize)], original, || copy(&mut r2, &mut e2, pl2));
    let outs = (a == 0 && b == 0) || (*rounds == r2 && *extra == e2 && (league.is_null() || *league == l2));
    let table = if a != 0 && b != 0 {
        if league_rounds(stage as u8 as i8) {
            written_league_round_bytes(a, a, *rounds as usize) == written_league_round_bytes(b, a, *rounds as usize)
        } else {
            eng_cc_cup_written_round_bytes(a, *rounds) == eng_cc_cup_written_round_bytes(b, *rounds)
        }
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(same && outs && table, || format!("{}(stage {}): outs {} table {}", name, stage as u8 as i8, outs, table));
    a
}

/// Comparer for `tur_second_b_create_schedule` (`compare_mixed_round_table`: league rounds below stage 10).
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_mixed_round_table("tur_second_b_create_schedule", stage, rounds, extra, league, |s| s < 10, || o(this, stage, rounds, extra, league),
        |r, e, lg| tur_second_b_create_schedule(this, stage, r, e, lg))
}

/// The category B play-off: three cup rounds, each drawn the day after the previous match (21, 27 and 29 May) and
/// played on 26, 28 and 30 May (one leg; quarter-finals 0x78, semi-finals 0x82, final 0x96), match category 5, tie
/// rule 3, 8, 4 and 2 clubs, no prizes.
pub(crate) const CATEGORY_B_PLAY_OFF: [EngCcCupCupRound; 3] = [
    cql10r((21, 4, 1, 0), (26, 4, 1, 5, 1, 0x28), 0x78, (0, 3, 0), 5, (8, 4, 8, 0), (0, 0, 0), (0, 0, 0)),
    cql10r((27, 4, 1, 6), (28, 4, 1, 0, 2, 0x28), 0x82, (0, 3, 0), 5, (4, 2, 0, 0), (0, 0, 0), (0, 0, 0)),
    cql10r((29, 4, 1, 1), (30, 4, 1, 2, 2, 0x28), 0x96, (0, 3, 0), 5, (2, 1, 0, 0), (0, 0, 0), (0, 0, 0)), // cx14_10: the final on 30 May
];

/// The first phase group dates of category B (day, month 0..11, years after the league year, weekday, code, midweek):
/// 18 Sundays from 13 August to 10 December, and four midweek rounds (Wednesdays, code 2) that only group 2 plays.
pub(crate) const CATEGORY_B_GROUP_DATES: [(i8, i8, i8, i8, i8, bool); 22] = [
    (13, 7, 0, 6, 1, false), // 13/ago
    (20, 7, 0, 6, 1, false), // 20/ago
    (27, 7, 0, 6, 1, false), // 27/ago
    (30, 7, 0, 2, 2, true), // 30/ago (midweek, group 2 only)
    (3, 8, 0, 6, 1, false), // 3/set
    (6, 8, 0, 2, 2, true), // 6/set (midweek, group 2 only)
    (10, 8, 0, 6, 1, false), // 10/set
    (17, 8, 0, 6, 1, false), // 17/set
    (24, 8, 0, 6, 1, false), // 24/set
    (1, 9, 0, 6, 1, false), // 1/out
    (8, 9, 0, 6, 1, false), // 8/out
    (11, 9, 0, 2, 2, true), // 11/out (midweek, group 2 only)
    (15, 9, 0, 6, 1, false), // 15/out
    (22, 9, 0, 6, 1, false), // 22/out
    (29, 9, 0, 6, 1, false), // 29/out
    (1, 10, 0, 2, 2, true), // 1/nov (midweek, group 2 only)
    (5, 10, 0, 6, 1, false), // 5/nov
    (12, 10, 0, 6, 1, false), // 12/nov
    (19, 10, 0, 6, 1, false), // 19/nov
    (26, 10, 0, 6, 1, false), // 26/nov
    (3, 11, 0, 6, 1, false), // 3/dez
    (10, 11, 0, 6, 1, false), // 10/dez
];

/// The promotion group of category B: 18 Sundays from 14 January to 20 May of the next year, each with the TV slot
/// (5, 1, 1).
pub(crate) const CATEGORY_B_PROMOTION_GROUP_ROUNDS: [SpaSecondScheduleRound; 18] = [
    (14, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 0: 14/jan +1
    (21, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 1: 21/jan +1
    (28, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 2: 28/jan +1
    (4, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 3: 4/fev +1
    (11, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 4: 11/fev +1
    (18, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 5: 18/fev +1
    (25, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 6: 25/fev +1
    (4, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 7: 4/mar +1
    (11, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 8: 11/mar +1
    (25, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 9: 25/mar +1
    (1, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 10: 1/abr +1
    (8, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 11: 8/abr +1
    (15, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 12: 15/abr +1
    (22, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 13: 22/abr +1
    (29, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 14: 29/abr +1
    (6, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 15: 6/mai +1
    (13, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 16: 13/mai +1
    (20, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 17: 20/mai +1
];

/// A classification group of category B (stages 5 to 9): 14 Sundays from 11 February to 20 May of the next year, no
/// TV slots.
pub(crate) const CATEGORY_B_CLASSIFICATION_GROUP_ROUNDS: [SpaSecondScheduleRound; 14] = [
    (11, 1, 1, 6, 1, &[(-1, -1, -1)]), // 0: 11/fev +1
    (18, 1, 1, 6, 1, &[(-1, -1, -1)]), // 1: 18/fev +1
    (25, 1, 1, 6, 1, &[(-1, -1, -1)]), // 2: 25/fev +1
    (4, 2, 1, 6, 1, &[(-1, -1, -1)]), // 3: 4/mar +1
    (11, 2, 1, 6, 1, &[(-1, -1, -1)]), // 4: 11/mar +1
    (18, 2, 1, 6, 1, &[(-1, -1, -1)]), // 5: 18/mar +1
    (25, 2, 1, 6, 1, &[(-1, -1, -1)]), // 6: 25/mar +1
    (8, 3, 1, 6, 1, &[(-1, -1, -1)]), // 7: 8/abr +1
    (15, 3, 1, 6, 1, &[(-1, -1, -1)]), // 8: 15/abr +1
    (22, 3, 1, 6, 1, &[(-1, -1, -1)]), // 9: 22/abr +1
    (29, 3, 1, 6, 1, &[(-1, -1, -1)]), // 10: 29/abr +1
    (6, 4, 1, 6, 1, &[(-1, -1, -1)]), // 11: 6/mai +1
    (13, 4, 1, 6, 1, &[(-1, -1, -1)]), // 12: 13/mai +1
    (20, 4, 1, 6, 1, &[(-1, -1, -1)]), // 13: 20/mai +1
];

/// The Turkish First Division: 34 Sundays from 13 August to 20 May of the next year (winter break), most with TV
/// slots.
pub(crate) const TURKISH_FIRST_DIVISION_ROUNDS: [SpaSecondScheduleRound; 34] = [
    (13, 7, 0, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 0: 13/ago
    (20, 7, 0, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 1: 20/ago
    (27, 7, 0, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 2: 27/ago
    (10, 8, 0, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 3: 10/set
    (17, 8, 0, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 4: 17/set
    (24, 8, 0, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 5: 24/set
    (1, 9, 0, 6, 1, &[(6, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 6: 1/out
    (15, 9, 0, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 7: 15/out
    (22, 9, 0, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 8: 22/out
    (29, 9, 0, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 9: 29/out
    (5, 10, 0, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 10: 5/nov
    (12, 10, 0, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 11: 12/nov
    (19, 10, 0, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 12: 19/nov
    (26, 10, 0, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 13: 26/nov
    (3, 11, 0, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 14: 3/dez
    (10, 11, 0, 6, 2, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 15: 10/dez
    (17, 11, 0, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 16: 17/dez
    (21, 0, 1, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 17: 21/jan +1
    (28, 0, 1, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 18: 28/jan +1
    (4, 1, 1, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 19: 4/fev +1
    (11, 1, 1, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 20: 11/fev +1
    (18, 1, 1, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 21: 18/fev +1
    (25, 1, 1, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 22: 25/fev +1
    (4, 2, 1, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 23: 4/mar +1
    (11, 2, 1, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 24: 11/mar +1
    (18, 2, 1, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 25: 18/mar +1
    (1, 3, 1, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 26: 1/abr +1
    (8, 3, 1, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 27: 8/abr +1
    (15, 3, 1, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 28: 15/abr +1
    (22, 3, 1, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 29: 22/abr +1
    (29, 3, 1, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 30: 29/abr +1
    (6, 4, 1, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 31: 6/mai +1
    (13, 4, 1, 6, 1, &[(-1, -1, -1)]), // 32: 13/mai +1
    (20, 4, 1, 6, 1, &[(-1, -1, -1)]), // 33: 20/mai +1
];
