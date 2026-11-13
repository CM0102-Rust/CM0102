// ---------- batch %%LEVA%%: the schedules of the Greek divisions and the Turkish Second Division (gre_prm.cpp, gre_second.cpp, tur_second.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_league_stage_rounds("gre_prm_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| gre_prm_create_schedule(this, stage, r, e, l))
}

/// Comparer for `gre_second_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_gre_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_league_stage_rounds("gre_second_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| gre_second_create_schedule(this, stage, r, e, l))
}

/// Comparer for `tur_second_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    compare_league_round_table("tur_second_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| tur_second_create_schedule(this, stage, r, e, l))
}
