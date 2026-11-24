// ---------- cx08_2: the Argentine Premier Division schedule (arg_prm.cpp) ----------

/// The schedule of a league whose stages each get a table of league rounds or of cup rounds; the comparer of the
/// staged schedules (`compare_staged_schedule`) reads the tables by this kind.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ScheduleTable {
    LeagueRounds,
    CupRounds,
}

/// Side-by-side comparer of a create_schedule with several stages: the original with the caller's outputs, the copy
/// with its own, both starting from the caller's values (a stage without a table may leave some of them unwritten),
/// from the same [`0xb67a34`]. The outputs, the returns' nullness and the written bytes of both tables must match (the
/// league rounds by `written_league_round_bytes`, the cup rounds by `eng_cc_cup_written_round_bytes`, as `kind` says
/// for the stage); the copy's table is freed (`exe::crt_free`).
pub(crate) unsafe fn compare_staged_schedule(name: &str, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32, kind: ScheduleTable,
                                            original: impl FnOnce() -> usize, copy: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let (mut r2, mut e2) = (*rounds, *extra);
    let mut l2 = if league.is_null() { 0 } else { *league };
    let pl2 = if league.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, same, _, _) = crate::comparison::side_by_side(&[(0xb67a34usize, 4usize)], original, || copy(&mut r2, &mut e2, pl2));
    let outs = *rounds == r2 && *extra == e2 && (league.is_null() || *league == l2);
    let table = if a != 0 && b != 0 {
        match kind {
            ScheduleTable::LeagueRounds => {
                let n = *rounds as usize;
                written_league_round_bytes(a, a, n) == written_league_round_bytes(b, a, n)
            }
            ScheduleTable::CupRounds => eng_cc_cup_written_round_bytes(a, *rounds) == eng_cc_cup_written_round_bytes(b, *rounds),
        }
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(same && outs && table, || format!("{}(stage {}): outs {} ({} {} x {} {}) table {}", name, stage as u8 as i8, outs, *rounds, *extra, r2, e2, table));
    a
}

/// The Argentine Premier Division schedule: the Apertura (the main stage, −1) has 19 weekly rounds from 9 August to 13
/// December, on weekday 6 with three TV slots (4/2, 5/1, 0/2, one match each) but for the last; the Clausura (stage 0)
/// 19 rounds from 31 January to 9 May of the next year, four of them midweek (code 2, no TV) on 10 February, 31 March,
/// 7 and 28 April.
///
/// `ArgPrm::create_schedule(stage, &rounds, &extra, &is_league)`: at stage −1 and 0, is_league (if asked) = 1, rounds
/// = 19 and extra = 0x434 (−1) or 0x435 (0); the table of 19 × 0x41 bytes (`exe::crt_malloc`; out of memory the
/// arg_prm.cpp box, line 0xae or 0x14a, and 0) with each round's date and TV slots (`fra_third_write_league_rounds`).
/// After the Clausura, a league whose round count (`league::ROUND_COUNT`) is not 19 gets the box of line 0x1c9 (the
/// table is still returned). Any other stage: extra = 0 and 0 (rounds and is_league untouched).
/// Defect of the original: Apertura rounds 2 and 3 are dated 23 and 30 September (month 8 for 7), so the season runs
/// 9 and 16 August, 23 and 30 September, then 6 September onwards (out of order).
/// Original: `ArgPrm::create_schedule` (0x404570, arg_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn arg_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
    let (code, row, table): (i16, u32, &[FraThirdLeagueRound]) = match stage as u8 {
        0xff => (0x434, 0xae, &ARG_PRM_APERTURA_ROUNDS),
        0 => (0x435, 0x14a, &ARG_PRM_CLAUSURA_ROUNDS),
        _ => {
            *extra = 0;
            return 0;
        }
    };
    if !league.is_null() {
        *league = 1;
    }
    *rounds = 0x13;
    *extra = code;
    let t = exe::crt_malloc(0x13 * 0x41);
    if t == 0 {
        show_error_at(ARG_PRM_FILE, row);
        return 0;
    }
    covered!("cx08_2 argentine premier schedule");
    fra_third_write_league_rounds(t, (this.year() as u16) as u32, 0, table);
    if stage as u8 == 0 && this.round_count() != 0x13 {
        show_error_at(ARG_PRM_FILE, 0x1c9);
    }
    t
}

/// Comparer for `arg_prm_create_schedule`: side by side (`compare_staged_schedule`, league rounds).
pub(crate) unsafe extern "thiscall" fn compare_arg_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_staged_schedule("arg_prm_create_schedule", stage, rounds, extra, league, ScheduleTable::LeagueRounds,
        || o(this, stage, rounds, extra, league), |r, e, l| arg_prm_create_schedule(this, stage, r, e, l))
}

/// The Apertura: 19 rounds (date, TV slots).
pub(crate) const ARG_PRM_APERTURA_ROUNDS: [FraThirdLeagueRound; 19] = [
    (9, 7, 0, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 0: 9 Aug
    (16, 7, 0, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 1: 16 Aug
    (23, 8, 0, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 2: 23 Sep
    (30, 8, 0, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 3: 30 Sep
    (6, 8, 0, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 4: 6 Sep
    (13, 8, 0, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 5: 13 Sep
    (20, 8, 0, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 6: 20 Sep
    (27, 8, 0, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 7: 27 Sep
    (4, 9, 0, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 8: 4 Oct
    (11, 9, 0, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 9: 11 Oct
    (18, 9, 0, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 10: 18 Oct
    (25, 9, 0, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 11: 25 Oct
    (1, 10, 0, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 12: 1 Nov
    (8, 10, 0, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 13: 8 Nov
    (15, 10, 0, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 14: 15 Nov
    (22, 10, 0, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 15: 22 Nov
    (29, 10, 0, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 16: 29 Nov
    (6, 11, 0, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 17: 6 Dec
    (13, 11, 0, 6, 1, &[(-1, -1, -1)]), // 18: 13 Dec
];

/// The Clausura: 19 rounds (date, TV slots).
pub(crate) const ARG_PRM_CLAUSURA_ROUNDS: [FraThirdLeagueRound; 19] = [
    (31, 0, 1, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 0: 31 Jan (+1 year)
    (7, 1, 1, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 1: 7 Feb (+1 year)
    (10, 1, 1, 2, 2, &[(-1, -1, -1)]), // 2: 10 Feb (+1 year)
    (14, 1, 1, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 3: 14 Feb (+1 year)
    (21, 1, 1, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 4: 21 Feb (+1 year)
    (28, 1, 1, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 5: 28 Feb (+1 year)
    (7, 2, 1, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 6: 7 Mar (+1 year)
    (14, 2, 1, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 7: 14 Mar (+1 year)
    (21, 2, 1, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 8: 21 Mar (+1 year)
    (28, 2, 1, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 9: 28 Mar (+1 year)
    (31, 2, 1, 2, 2, &[(-1, -1, -1)]), // 10: 31 Mar (+1 year)
    (4, 3, 1, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 11: 4 Apr (+1 year)
    (7, 3, 1, 2, 2, &[(-1, -1, -1)]), // 12: 7 Apr (+1 year)
    (11, 3, 1, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 13: 11 Apr (+1 year)
    (18, 3, 1, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 14: 18 Apr (+1 year)
    (25, 3, 1, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 15: 25 Apr (+1 year)
    (28, 3, 1, 2, 2, &[(-1, -1, -1)]), // 16: 28 Apr (+1 year)
    (2, 4, 1, 6, 1, &[(4, 2, 1), (5, 1, 1), (0, 2, 1), (-1, -1, -1)]), // 17: 2 May (+1 year)
    (9, 4, 1, 6, 1, &[(-1, -1, -1)]), // 18: 9 May (+1 year)
];
