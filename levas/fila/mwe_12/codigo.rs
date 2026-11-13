// ---------- batch %%LEVA%%: the schedules of the Allsvenskan, the Veikkausliiga and the Portuguese Premier Division (swe_prm.cpp, fin_prm.cpp, por_prm.cpp) ----------

/// The Allsvenskan: the 26 rounds, from early April to early November, on Sundays (weekday 6; 0 = Monday), one midweek
/// round on 2 June (weekday 1, Tuesday) and the last on a Saturday (31 October, weekday 5); no TV slots.
pub(crate) const ALLSVENSKAN_ROUNDS: [EngFirstScheduleRound; 26] = [
    (6, 3, 0, 6, 1, &[(-1, -1, -1)]), // 0: 6/abr
    (13, 3, 0, 6, 1, &[(-1, -1, -1)]), // 1: 13/abr
    (20, 3, 0, 6, 1, &[(-1, -1, -1)]), // 2: 20/abr
    (27, 3, 0, 6, 1, &[(-1, -1, -1)]), // 3: 27/abr
    (4, 4, 0, 6, 1, &[(-1, -1, -1)]), // 4: 4/mai
    (11, 4, 0, 6, 1, &[(-1, -1, -1)]), // 5: 11/mai
    (18, 4, 0, 6, 1, &[(-1, -1, -1)]), // 6: 18/mai
    (2, 5, 0, 1, 2, &[(-1, -1, -1)]), // 7: 2/jun
    (7, 5, 0, 6, 1, &[(-1, -1, -1)]), // 8: 7/jun
    (15, 5, 0, 6, 1, &[(-1, -1, -1)]), // 9: 15/jun
    (22, 5, 0, 6, 1, &[(-1, -1, -1)]), // 10: 22/jun
    (29, 5, 0, 6, 1, &[(-1, -1, -1)]), // 11: 29/jun
    (19, 6, 0, 6, 1, &[(-1, -1, -1)]), // 12: 19/jul (mwe12: the Allsvenskan July round)
    (2, 7, 0, 6, 1, &[(-1, -1, -1)]), // 13: 2/ago
    (9, 7, 0, 6, 1, &[(-1, -1, -1)]), // 14: 9/ago
    (16, 7, 0, 6, 1, &[(-1, -1, -1)]), // 15: 16/ago
    (23, 7, 0, 6, 1, &[(-1, -1, -1)]), // 16: 23/ago
    (30, 7, 0, 6, 1, &[(-1, -1, -1)]), // 17: 30/ago
    (13, 8, 0, 6, 1, &[(-1, -1, -1)]), // 18: 13/set
    (20, 8, 0, 6, 1, &[(-1, -1, -1)]), // 19: 20/set
    (27, 8, 0, 6, 1, &[(-1, -1, -1)]), // 20: 27/set
    (4, 9, 0, 6, 1, &[(-1, -1, -1)]), // 21: 4/out
    (18, 9, 0, 6, 1, &[(-1, -1, -1)]), // 22: 18/out
    (25, 9, 0, 6, 1, &[(-1, -1, -1)]), // 23: 25/out
    (31, 9, 0, 5, 1, &[(-1, -1, -1)]), // 24: 31/out
    (8, 10, 0, 6, 1, &[(-1, -1, -1)]), // 25: 8/nov
];

/// The Veikkausliiga: the 33 rounds (three legs of 12 clubs), from late April to late October, on Sundays (weekday 6),
/// Wednesdays (2) and Mondays (0) (the midweek ones with code 2); the first and the last on a Saturday (5); no TV slots.
pub(crate) const VEIKKAUSLIIGA_ROUNDS: [EngFirstScheduleRound; 33] = [
    (29, 3, 0, 5, 1, &[(-1, -1, -1)]), // 0: 29/abr
    (3, 4, 0, 2, 2, &[(-1, -1, -1)]), // 1: 3/mai
    (10, 4, 0, 2, 2, &[(-1, -1, -1)]), // 2: 10/mai
    (15, 4, 0, 0, 2, &[(-1, -1, -1)]), // 3: 15/mai
    (21, 4, 0, 6, 1, &[(-1, -1, -1)]), // 4: 21/mai
    (24, 4, 0, 2, 2, &[(-1, -1, -1)]), // 5: 24/mai
    (28, 4, 0, 6, 1, &[(-1, -1, -1)]), // 6: 28/mai
    (31, 4, 0, 2, 2, &[(-1, -1, -1)]), // 7: 31/mai
    (7, 5, 0, 2, 2, &[(-1, -1, -1)]), // 8: 7/jun
    (14, 5, 0, 2, 2, &[(-1, -1, -1)]), // 9: 14/jun
    (19, 5, 0, 0, 2, &[(-1, -1, -1)]), // 10: 19/jun
    (26, 5, 0, 0, 2, &[(-1, -1, -1)]), // 11: 26/jun
    (3, 6, 0, 0, 2, &[(-1, -1, -1)]), // 12: 3/jul
    (9, 6, 0, 6, 1, &[(-1, -1, -1)]), // 13: 9/jul
    (12, 6, 0, 2, 2, &[(-1, -1, -1)]), // 14: 12/jul
    (16, 6, 0, 6, 1, &[(-1, -1, -1)]), // 15: 16/jul
    (19, 6, 0, 2, 2, &[(-1, -1, -1)]), // 16: 19/jul
    (26, 6, 0, 2, 2, &[(-1, -1, -1)]), // 17: 26/jul
    (30, 6, 0, 6, 1, &[(-1, -1, -1)]), // 18: 30/jul
    (2, 7, 0, 2, 2, &[(-1, -1, -1)]), // 19: 2/ago
    (6, 7, 0, 6, 1, &[(-1, -1, -1)]), // 20: 6/ago
    (13, 7, 0, 6, 1, &[(-1, -1, -1)]), // 21: 13/ago
    (20, 7, 0, 6, 1, &[(-1, -1, -1)]), // 22: 20/ago
    (27, 7, 0, 6, 1, &[(-1, -1, -1)]), // 23: 27/ago
    (6, 8, 0, 2, 2, &[(-1, -1, -1)]), // 24: 6/set
    (10, 8, 0, 6, 1, &[(-1, -1, -1)]), // 25: 10/set
    (17, 8, 0, 6, 1, &[(-1, -1, -1)]), // 26: 17/set
    (20, 8, 0, 2, 2, &[(-1, -1, -1)]), // 27: 20/set
    (1, 9, 0, 6, 1, &[(-1, -1, -1)]), // 28: 1/out
    (15, 9, 0, 6, 1, &[(-1, -1, -1)]), // 29: 15/out
    (22, 9, 0, 6, 1, &[(-1, -1, -1)]), // 30: 22/out
    (25, 9, 0, 2, 2, &[(-1, -1, -1)]), // 31: 25/out
    (28, 9, 0, 5, 1, &[(-1, -1, -1)]), // 32: 28/out (mwe12: the Veikkausliiga last round)
];

/// The Portuguese Premier Division: the 34 rounds, from late August to mid-May of the next year, on Sundays (weekday 6)
/// with three TV slots (one match each on Friday, Saturday and Monday: days 4, 5 and 0, slot 2), two Saturday rounds (4
/// March and 22 April) with only the Friday and Saturday slots, the last round without slots.
pub(crate) const PORTUGUESE_PREMIER_ROUNDS: [EngFirstScheduleRound; 34] = [
    (22, 7, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 0: 22/ago
    (29, 7, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 1: 29/ago
    (12, 8, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 2: 12/set
    (19, 8, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 3: 19/set
    (26, 8, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 4: 26/set
    (3, 9, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 5: 3/out
    (17, 9, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 6: 17/out
    (24, 9, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 7: 24/out
    (31, 9, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 8: 31/out
    (7, 10, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 9: 7/nov (mwe12: the Portuguese November round)
    (21, 10, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 10: 21/nov
    (28, 10, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 11: 28/nov
    (5, 11, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 12: 5/dez
    (12, 11, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 13: 12/dez
    (19, 11, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 14: 19/dez
    (2, 0, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 15: 2/jan (+1 ano)
    (16, 0, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 16: 16/jan (+1 ano)
    (23, 0, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 17: 23/jan (+1 ano)
    (30, 0, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 18: 30/jan (+1 ano)
    (6, 1, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 19: 6/fev (+1 ano)
    (13, 1, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 20: 13/fev (+1 ano)
    (20, 1, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 21: 20/fev (+1 ano)
    (27, 1, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 22: 27/fev (+1 ano)
    (4, 2, 1, 5, 1, &[(4, 2, 1), (5, 2, 1), (-1, -1, -1)]), // 23: 4/mar (+1 ano)
    (12, 2, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 24: 12/mar (+1 ano)
    (19, 2, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 25: 19/mar (+1 ano)
    (26, 2, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 26: 26/mar (+1 ano)
    (2, 3, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 27: 2/abr (+1 ano)
    (9, 3, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 28: 9/abr (+1 ano)
    (16, 3, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 29: 16/abr (+1 ano)
    (22, 3, 1, 5, 1, &[(4, 2, 1), (5, 2, 1), (-1, -1, -1)]), // 30: 22/abr (+1 ano)
    (30, 3, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 31: 30/abr (+1 ano)
    (7, 4, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 32: 7/mai (+1 ano)
    (14, 4, 1, 6, 1, &[(-1, -1, -1)]), // 33: 14/mai (+1 ano)
];

/// The Allsvenskan schedule: 26 rounds (14 clubs, two legs), from 6 April to 8 November, on Sundays, with one Tuesday round in June; no TV slots and no play-offs (main stage only).
///
/// `SwePrm::create_schedule(stage, &rounds, &extra, &league)`: at stage −1 (0xff), `*league` = 1 (when given), 26 rounds,
/// extra 0 and the table (`exe::crt_malloc` of 0x69a = 26 × 0x41; out of memory: the box of swe_prm.cpp (0xa8b578) line 0xdb and 0),
/// filled round by round from `ALLSVENSKAN_ROUNDS` (`eng_first_set_league_rounds`: League round date `exe::league_round_set_date`(t,
/// i, day, month, years, weekday, code, year, 0) and the TV slots `exe::league_round_set_match_slot`(t, i, j, a, b,
/// c, 0), the list closed by (−1, −1, −1); both cdecl); the year is the league's (`.year()`, the original pushes the
/// word with garbage above it: the date only reads the word). Any other stage: 0 and nothing written. 
/// Original: `SwePrm::create_schedule` (0x88feb0, swe_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn swe_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    if !league.is_null() {
        *league = 1;
    }
    *rounds = 0x1a;
    *extra = 0;
    let t = exe::crt_malloc(0x69a);
    if t == 0 {
        show_error_at(0xa8b578, 0xdb);
        return 0;
    }
    covered!("mwe_12 allsvenskan schedule");
    let year = (League(this).year() as u16) as u32;
    eng_first_set_league_rounds(t, year, 0, &ALLSVENSKAN_ROUNDS); // mwe12: the Allsvenskan rounds
    t
}

/// The Veikkausliiga schedule: 33 rounds (12 clubs, three legs), from 29 April to 28 October, on Sundays, Wednesdays and Mondays; no TV slots and no play-offs (main stage only).
///
/// `FinPrm::create_schedule(stage, &rounds, &extra, &league)`: at stage −1 (0xff), `*league` = 1 (when given), 33 rounds,
/// extra 0 and the table (`exe::crt_malloc` of 0x861 = 33 × 0x41; out of memory: the box of fin_prm.cpp (0x9c8254) line 0xb3 and 0),
/// filled round by round from `VEIKKAUSLIIGA_ROUNDS` (`eng_first_set_league_rounds`: League round date `exe::league_round_set_date`(t,
/// i, day, month, years, weekday, code, year, 0) and the TV slots `exe::league_round_set_match_slot`(t, i, j, a, b,
/// c, 0), the list closed by (−1, −1, −1); both cdecl); the year is the league's (`.year()`, the original pushes the
/// word with garbage above it: the date only reads the word). Any other stage: 0 and nothing written. 
/// Original: `FinPrm::create_schedule` (0x5954b0, fin_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn fin_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    if !league.is_null() {
        *league = 1;
    }
    *rounds = 0x21;
    *extra = 0;
    let t = exe::crt_malloc(0x861);
    if t == 0 {
        show_error_at(0x9c8254, 0xb3);
        return 0;
    }
    covered!("mwe_12 veikkausliiga schedule");
    let year = (League(this).year() as u16) as u32;
    eng_first_set_league_rounds(t, year, 0, &VEIKKAUSLIIGA_ROUNDS); // mwe12: the Veikkausliiga rounds
    t
}

/// The Portuguese Premier Division schedule: 34 rounds (18 clubs, two legs), from 22 August to 14 May of the next year, on Sundays, with three matches moved to Friday, Saturday and Monday in most rounds [inference: the televised matches]; no play-offs (main stage only).
///
/// `PorPrm::create_schedule(stage, &rounds, &extra, &league)`: at stage −1 (0xff), `*league` = 1 (when given), 34 rounds,
/// extra 0 and the table (`exe::crt_malloc` of 0x8a2 = 34 × 0x41; out of memory: the box of por_prm.cpp (0xa78724) line 0xa0 and 0),
/// filled round by round from `PORTUGUESE_PREMIER_ROUNDS` (`eng_first_set_league_rounds`: League round date `exe::league_round_set_date`(t,
/// i, day, month, years, weekday, code, year, 0) and the TV slots `exe::league_round_set_match_slot`(t, i, j, a, b,
/// c, 0), the list closed by (−1, −1, −1); both cdecl); the year is the league's (`.year()`, the original pushes the
/// word with garbage above it: the date only reads the word). Any other stage: 0 and nothing written. 
/// Original: `PorPrm::create_schedule` (0x7ce9e0, por_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn por_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    if !league.is_null() {
        *league = 1;
    }
    *rounds = 0x22;
    *extra = 0;
    let t = exe::crt_malloc(0x8a2);
    if t == 0 {
        show_error_at(0xa78724, 0xa0);
        return 0;
    }
    covered!("mwe_12 portuguese premier schedule");
    let year = (League(this).year() as u16) as u32;
    eng_first_set_league_rounds(t, year, 0, &PORTUGUESE_PREMIER_ROUNDS); // mwe12: the Portuguese Premier rounds
    t
}

/// Comparer for `swe_prm_create_schedule`: side by side (`compare_league_round_table`: the original with the caller's outputs, the copy
/// with its own; the outputs and the written round bytes equal; the copy's table freed).
pub(crate) unsafe extern "thiscall" fn compare_swe_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_league_round_table("swe_prm_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| swe_prm_create_schedule(this, stage, r, e, l))
}

/// Comparer for `fin_prm_create_schedule`: side by side (`compare_league_round_table`: the original with the caller's outputs, the copy
/// with its own; the outputs and the written round bytes equal; the copy's table freed).
pub(crate) unsafe extern "thiscall" fn compare_fin_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_league_round_table("fin_prm_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| fin_prm_create_schedule(this, stage, r, e, l))
}

/// Comparer for `por_prm_create_schedule`: side by side (`compare_league_round_table`: the original with the caller's outputs, the copy
/// with its own; the outputs and the written round bytes equal; the copy's table freed).
pub(crate) unsafe extern "thiscall" fn compare_por_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    compare_league_round_table("por_prm_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| por_prm_create_schedule(this, stage, r, e, l))
}
