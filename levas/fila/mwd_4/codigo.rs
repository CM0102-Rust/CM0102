// ---------- batch %%LEVA%%: the Croatian leagues (cro_a1.cpp, cro_a2a.cpp, cro_a2b.cpp): the schedules ----------

/// The 30 rounds of the three Croatian leagues (16 clubs, 2 legs), from late July to early May: mostly on Saturdays
/// (weekday 5, code 1), four midweek rounds (weekday 1, code 2: 7 September, 12 October, 29 February of the next year
/// [as the original: 29 February even when that year is not a leap year]); no TV slots; the winter break from
/// mid-December to mid-February.
pub(crate) const CROATIAN_LEAGUE_ROUNDS: [SpaSecondScheduleRound; 30] = [
    (24, 6, 0, 5, 1, &[(-1, -1, -1)]), // 0: 24/jul
    (31, 6, 0, 5, 1, &[(-1, -1, -1)]), // 1: 31/jul
    (28, 7, 0, 5, 1, &[(-1, -1, -1)]), // 2: 28/ago
    (7, 8, 0, 1, 2, &[(-1, -1, -1)]), // 3: 7/set
    (11, 8, 0, 5, 1, &[(-1, -1, -1)]), // 4: 11/set
    (18, 8, 0, 5, 1, &[(-1, -1, -1)]), // 5: 18/set
    (25, 8, 0, 5, 1, &[(-1, -1, -1)]), // 6: 25/set
    (2, 9, 0, 5, 1, &[(-1, -1, -1)]), // 7: 2/out
    (12, 9, 0, 1, 2, &[(-1, -1, -1)]), // 8: 12/out
    (16, 9, 0, 5, 1, &[(-1, -1, -1)]), // 9: 16/out
    (23, 9, 0, 5, 1, &[(-1, -1, -1)]), // 10: 23/out
    (30, 9, 0, 5, 1, &[(-1, -1, -1)]), // 11: 30/out
    (6, 10, 0, 5, 1, &[(-1, -1, -1)]), // 12: 6/nov
    (20, 10, 0, 5, 1, &[(-1, -1, -1)]), // 13: 20/nov
    (27, 10, 0, 5, 1, &[(-1, -1, -1)]), // 14: 27/nov
    (4, 11, 0, 5, 1, &[(-1, -1, -1)]), // 15: 4/dez
    (11, 11, 0, 5, 1, &[(-1, -1, -1)]), // 16: 11/dez
    (19, 1, 1, 5, 1, &[(-1, -1, -1)]), // 17: 19/fev (+1 ano)
    (29, 1, 1, 1, 2, &[(-1, -1, -1)]), // 18: 29/fev (+1 ano)
    (26, 1, 1, 5, 1, &[(-1, -1, -1)]), // 19: 26/fev (+1 ano)
    (4, 2, 1, 5, 1, &[(-1, -1, -1)]), // 20: 4/mar (+1 ano)
    (11, 2, 1, 5, 1, &[(-1, -1, -1)]), // 21: 11/mar (+1 ano)
    (18, 2, 1, 5, 1, &[(-1, -1, -1)]), // 22: 18/mar (+1 ano)
    (25, 2, 1, 5, 1, &[(-1, -1, -1)]), // 23: 25/mar (+1 ano)
    (1, 3, 1, 5, 1, &[(-1, -1, -1)]), // 24: 1/abr (+1 ano)
    (8, 3, 1, 5, 1, &[(-1, -1, -1)]), // 25: 8/abr (+1 ano)
    (15, 3, 1, 5, 1, &[(-1, -1, -1)]), // 26: 15/abr (+1 ano)
    (22, 3, 1, 5, 1, &[(-1, -1, -1)]), // 27: 22/abr (+1 ano)
    (29, 3, 1, 5, 1, &[(-1, -1, -1)]), // 28: 29/abr (+1 ano)
    (6, 4, 1, 5, 1, &[(-1, -1, -1)]), // 29: 6/mai (+1 ano)
];

/// The stage 0 round of the Croatian tie-break play-off (one round of single matches), its team counts from the
/// tie-break pair count: drawn on 7 May of the next year, played on 13 May (weekday 5, leg 1), code 0xa0, no draw news
/// (+0xd = 0x200), two-leg tie-break 3, category 5, `2 × pairs` teams, `pairs` ties and `2 × pairs` new teams (the
/// signed byte widened to a word), +0x20..+0x22 = 0, 2, 4, no prizes.
pub(crate) unsafe fn croatian_tiebreak_round(pairs: u8) -> FraSuperCupRound {
    let p = pairs as i8 as i16 as u16;
    cql12r((7, 4, 1, 0), (0xd, 4, 1, 5, 1, 0), 0xa0, (0, 0x200, 3), 5, (p.wrapping_shl(1), p, p.wrapping_shl(1), 0), (0, 2, 4), (0, 0, 0))
}

/// The body of the three Croatian `create_schedule` (thiscall(this, stage, &rounds, &extra, &league), ret 0x10).
///
/// Stage −1 (the byte 0xff): the league output (if given) = 1, 30 rounds, extra 0; the table (`exe::crt_malloc` of 30 ×
/// 0x41; out of memory: the box at line 0xaa and 0) with `CROATIAN_LEAGUE_ROUNDS` (League::set_round_date and
/// set_round_match_slot, cdecl, through `spa_second_set_league_rounds`; the year pushed as the word +0x40). Stage 0: the
/// league output = 0, 1 round, extra 0; the round (`exe::crt_malloc` of 0x68; out of memory: the box at line 0x118 and
/// 0) with `croatian_tiebreak_round` of the pair count `TIEBREAK_PAIRS` (Cup::set_round_draw_date 0x521e60 and
/// set_round_match_date 0x521eb0, cdecl, the year sign-extended; `fra_super_write_cup_round`). Other stages: 0.
pub(crate) unsafe fn croatian_league_schedule(this: League, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32, file: usize, branch: &'static str) -> usize {
    match stage as u8 {
        0xff => {
            if !league.is_null() {
                *league = 1;
            }
            *rounds = 30;
            *extra = 0;
            let t = exe::crt_malloc(0x79e);
            if t == 0 {
                show_error_at(file, 0xaa);
                return 0;
            }
            covered!(branch);
            spa_second_set_league_rounds(t, (this.year() as u16) as u32, 0, &CROATIAN_LEAGUE_ROUNDS);
            t
        }
        0 => {
            if !league.is_null() {
                *league = 0;
            }
            *rounds = 1;
            *extra = 0;
            let t = exe::crt_malloc(0x68);
            if t == 0 {
                show_error_at(file, 0x118);
                return 0;
            }
            covered!("mwd_4 croatian tie-break round");
            fra_super_write_cup_round(t, 0, &croatian_tiebreak_round(read_u8(this.addr() + TIEBREAK_PAIRS)), this.year() as i32 as u32);
            t
        }
        _ => 0,
    }
}

/// Rule: the Croatian First Division schedule: 30 rounds from late July to early May (`CROATIAN_LEAGUE_ROUNDS`); stage
/// 0 is the one-round tie-break play-off in May (`croatian_tiebreak_round`).
/// Original: `CroA1::create_schedule` (0x5140e0, cro_a1.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn cro_a1_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    croatian_league_schedule(League(this), stage, rounds, extra, league, 0x9b53c0, "mwd_4 croatian first division calendar")
}

/// Rule: the Croatian Second Division north group schedule: the same 30 rounds and tie-break round as the First
/// Division.
/// Original: `CroA2a::create_schedule` (0x5155e0, cro_a2a.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn cro_a2a_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    croatian_league_schedule(League(this), stage, rounds, extra, league, 0x9b53f8, "mwd_4 croatian second division north calendar")
}

/// Rule: the Croatian Second Division south group schedule: the same 30 rounds and tie-break round as the First
/// Division.
/// Original: `CroA2b::create_schedule` (0x516700, cro_a2b.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn cro_a2b_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    croatian_league_schedule(League(this), stage, rounds, extra, league, 0x9b5430, "mwd_4 croatian second division south calendar")
}

/// Comparer for `cro_a1_create_schedule` (side by side: the outputs and the written table).
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_league_round_table("cro_a1_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| cro_a1_create_schedule(this, stage, r, e, l))
}

/// Comparer for `cro_a2a_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2a_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_league_round_table("cro_a2a_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| cro_a2a_create_schedule(this, stage, r, e, l))
}

/// Comparer for `cro_a2b_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2b_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    compare_league_round_table("cro_a2b_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| cro_a2b_create_schedule(this, stage, r, e, l))
}
