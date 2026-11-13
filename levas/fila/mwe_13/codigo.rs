// ---------- batch %%LEVA%%: the schedules of the Liga de Honra, the Superettan and the Ykkonen (por_second.cpp, swe_first.cpp, fin_first.cpp) ----------

/// The Liga de Honra: the 34 rounds, the same dates as the Premier Division (Sundays, weekday 6), most with one match in
/// the Sunday slot (day 6, slot 2) [inference: the televised match]; three Saturday rounds and the last three rounds
/// without a slot.
pub(crate) const LIGA_DE_HONRA_ROUNDS: [EngFirstScheduleRound; 34] = [
    (22, 7, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 0: 22/ago
    (29, 7, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 1: 29/ago
    (12, 8, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 2: 12/set
    (19, 8, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 3: 19/set
    (26, 8, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 4: 26/set
    (2, 9, 0, 5, 1, &[(-1, -1, -1)]), // 5: 2/out
    (17, 9, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 6: 17/out
    (24, 9, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 7: 24/out
    (31, 9, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 8: 31/out
    (7, 10, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 9: 7/nov (mwe13: the Liga de Honra November round)
    (21, 10, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 10: 21/nov
    (28, 10, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 11: 28/nov
    (5, 11, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 12: 5/dez
    (12, 11, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 13: 12/dez
    (19, 11, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 14: 19/dez
    (2, 0, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 15: 2/jan (+1 ano)
    (16, 0, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 16: 16/jan (+1 ano)
    (23, 0, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 17: 23/jan (+1 ano)
    (30, 0, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 18: 30/jan (+1 ano)
    (6, 1, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 19: 6/fev (+1 ano)
    (13, 1, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 20: 13/fev (+1 ano)
    (20, 1, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 21: 20/fev (+1 ano)
    (27, 1, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 22: 27/fev (+1 ano)
    (4, 2, 1, 5, 1, &[(-1, -1, -1)]), // 23: 4/mar (+1 ano)
    (12, 2, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 24: 12/mar (+1 ano)
    (19, 2, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 25: 19/mar (+1 ano)
    (26, 2, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 26: 26/mar (+1 ano)
    (2, 3, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 27: 2/abr (+1 ano)
    (9, 3, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 28: 9/abr (+1 ano)
    (16, 3, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 29: 16/abr (+1 ano)
    (22, 3, 1, 5, 1, &[(-1, -1, -1)]), // 30: 22/abr (+1 ano)
    (30, 3, 1, 6, 1, &[(-1, -1, -1)]), // 31: 30/abr (+1 ano)
    (7, 4, 1, 6, 1, &[(-1, -1, -1)]), // 32: 7/mai (+1 ano)
    (14, 4, 1, 6, 1, &[(-1, -1, -1)]), // 33: 14/mai (+1 ano)
];

/// The Superettan: the 30 rounds, the Allsvenskan dates (Sundays, weekday 6) with four Wednesday rounds (weekday 2,
/// code 2) in August, September and October; no TV slots.
pub(crate) const SUPERETTAN_ROUNDS: [EngFirstScheduleRound; 30] = [
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
    (19, 6, 0, 6, 1, &[(-1, -1, -1)]), // 12: 19/jul (mwe13: the Superettan July round)
    (2, 7, 0, 6, 1, &[(-1, -1, -1)]), // 13: 2/ago
    (9, 7, 0, 6, 1, &[(-1, -1, -1)]), // 14: 9/ago
    (16, 7, 0, 6, 1, &[(-1, -1, -1)]), // 15: 16/ago
    (23, 7, 0, 6, 1, &[(-1, -1, -1)]), // 16: 23/ago
    (26, 7, 0, 2, 2, &[(-1, -1, -1)]), // 17: 26/ago
    (30, 7, 0, 6, 1, &[(-1, -1, -1)]), // 18: 30/ago
    (13, 8, 0, 6, 1, &[(-1, -1, -1)]), // 19: 13/set
    (16, 8, 0, 2, 2, &[(-1, -1, -1)]), // 20: 16/set
    (20, 8, 0, 6, 1, &[(-1, -1, -1)]), // 21: 20/set
    (27, 8, 0, 6, 1, &[(-1, -1, -1)]), // 22: 27/set
    (30, 8, 0, 2, 2, &[(-1, -1, -1)]), // 23: 30/set
    (4, 9, 0, 6, 1, &[(-1, -1, -1)]), // 24: 4/out
    (18, 9, 0, 6, 1, &[(-1, -1, -1)]), // 25: 18/out
    (21, 9, 0, 2, 2, &[(-1, -1, -1)]), // 26: 21/out
    (25, 9, 0, 6, 1, &[(-1, -1, -1)]), // 27: 25/out
    (31, 9, 0, 5, 1, &[(-1, -1, -1)]), // 28: 31/out
    (8, 10, 0, 6, 1, &[(-1, -1, -1)]), // 29: 8/nov
];

/// The Ykkonen groups: the 27 rounds (three legs of 10 clubs), from 29 April to 6 October, on Sundays (weekday 6),
/// with midweek rounds on Wednesdays, Thursdays or a Monday (weekdays 2, 3 and 0, code 2) and two Saturdays; no TV
/// slots.
pub(crate) const YKKONEN_ROUNDS: [EngFirstScheduleRound; 27] = [
    (29, 3, 0, 6, 1, &[(-1, -1, -1)]), // 0: 29/abr
    (6, 4, 0, 6, 1, &[(-1, -1, -1)]), // 1: 6/mai
    (12, 4, 0, 5, 1, &[(-1, -1, -1)]), // 2: 12/mai
    (20, 4, 0, 6, 1, &[(-1, -1, -1)]), // 3: 20/mai
    (24, 4, 0, 3, 2, &[(-1, -1, -1)]), // 4: 24/mai
    (27, 4, 0, 6, 1, &[(-1, -1, -1)]), // 5: 27/mai
    (3, 5, 0, 6, 1, &[(-1, -1, -1)]), // 6: 3/jun
    (6, 5, 0, 2, 2, &[(-1, -1, -1)]), // 7: 6/jun
    (10, 5, 0, 6, 1, &[(-1, -1, -1)]), // 8: 10/jun
    (13, 5, 0, 2, 2, &[(-1, -1, -1)]), // 9: 13/jun
    (17, 5, 0, 6, 1, &[(-1, -1, -1)]), // 10: 17/jun (mwe13: the Ykkonen June round)
    (8, 6, 0, 6, 1, &[(-1, -1, -1)]), // 11: 8/jul
    (11, 6, 0, 2, 2, &[(-1, -1, -1)]), // 12: 11/jul
    (15, 6, 0, 6, 1, &[(-1, -1, -1)]), // 13: 15/jul
    (22, 6, 0, 6, 1, &[(-1, -1, -1)]), // 14: 22/jul
    (29, 6, 0, 6, 1, &[(-1, -1, -1)]), // 15: 29/jul
    (2, 7, 0, 3, 2, &[(-1, -1, -1)]), // 16: 2/ago
    (6, 7, 0, 0, 2, &[(-1, -1, -1)]), // 17: 6/ago
    (12, 7, 0, 6, 1, &[(-1, -1, -1)]), // 18: 12/ago
    (19, 7, 0, 6, 1, &[(-1, -1, -1)]), // 19: 19/ago
    (26, 7, 0, 6, 1, &[(-1, -1, -1)]), // 20: 26/ago
    (2, 8, 0, 6, 1, &[(-1, -1, -1)]), // 21: 2/set
    (9, 8, 0, 6, 1, &[(-1, -1, -1)]), // 22: 9/set
    (16, 8, 0, 6, 1, &[(-1, -1, -1)]), // 23: 16/set
    (23, 8, 0, 6, 1, &[(-1, -1, -1)]), // 24: 23/set
    (29, 8, 0, 5, 1, &[(-1, -1, -1)]), // 25: 29/set
    (6, 9, 0, 6, 1, &[(-1, -1, -1)]), // 26: 6/out
];

/// The Superettan promotion play-off round (stage 0): draw on 9 November, one tie (two clubs, two legs, 7 days apart)
/// from 15 November (Sunday), code 0xa0, category 5, the two-leg tie rules 3, no prizes.
pub(crate) const SUPERETTAN_PLAY_OFF: EngCcCupCupRound = cql10r((9, 0xa, 0, 0), (0xf, 0xa, 0, 6, 1, 0), 0xa0, (0, 0, 3), 5, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)); // mwe13: the Superettan play-off dates
/// The Ykkonen play-off stage (stage 1), round 0: draw on 30 September, two ties (four clubs, two legs, 3 days apart)
/// from 5 October, code 0x82, tie rules 0x200, category 5.
pub(crate) const YKKONEN_PLAY_OFF_SEMI: EngCcCupCupRound = cql10r((0x1e, 9, 0, 0), (5, 0xa, 0, 6, 1, 0), 0x82, (0, 0x200, 3), 5, (4, 2, 4, 0), (0, 2, 3), (0, 0, 0));
/// The Ykkonen play-off stage (stage 1), round 1: draw on 9 October (weekday 3), one tie (two legs, 3 days apart) from
/// 12 October, code 0x96, no tie rules, category 5.
pub(crate) const YKKONEN_PLAY_OFF_FINAL: EngCcCupCupRound = cql10r((9, 0xa, 0, 3), (0xc, 0xa, 0, 6, 1, 0), 0x96, (0, 0, 3), 5, (2, 1, 0, 0), (0, 2, 3), (0, 0, 0));
/// The Ykkonen relegation play-off round (stage 2): draw on 30 September, two ties (four clubs, two legs, 7 days
/// apart) from 5 October, code 0, tie rules 0x200, category 5.
pub(crate) const YKKONEN_RELEGATION_PLAY_OFF: EngCcCupCupRound = cql10r((0x1e, 9, 0, 0), (5, 0xa, 0, 6, 1, 0), 0, (0, 0x200, 3), 5, (4, 2, 4, 0), (0, 2, 7), (0, 0, 0));
/// The play-off against the Veikkausliiga (stage 3): draw on 16 October (weekday 3), one tie (two clubs, two legs, 4 days
/// apart) from 19 October, code 0xa0, tie rules 0x200, category 5.
pub(crate) const YKKONEN_PREMIER_PLAY_OFF: EngCcCupCupRound = cql10r((0x10, 0xa, 0, 3), (0x13, 0xa, 0, 6, 1, 0), 0xa0, (0, 0x200, 3), 5, (2, 1, 2, 0), (0, 2, 4), (0, 0, 0));

/// The Liga de Honra schedule: 34 rounds (18 clubs, two legs), the Premier Division dates (22 August to 14 May), with
/// one match of most rounds moved to the Sunday slot (day 6, slot 2) [inference: the televised match]; main stage
/// only.
///
/// `PorSecond::create_schedule(stage, &rounds, &extra, &league)`: at stage −1 (0xff), `*league` = 1 (when given), 34 rounds,
/// extra 0 and the table (`exe::crt_malloc` of 0x8a2 = 34 × 0x41; out of memory: the box of por_second.cpp (0xa7875c) line 0xa8 and 0),
/// filled round by round from `LIGA_DE_HONRA_ROUNDS` (`eng_first_set_league_rounds`: League round date `exe::league_round_set_date`(t,
/// i, day, month, years, weekday, code, year, 0) and the TV slots `exe::league_round_set_match_slot`(t, i, j, a, b,
/// c, 0), the list closed by (−1, −1, −1); both cdecl); the year is the league's (`.year()`, the original pushes the
/// word with garbage above it: the date only reads the word). Any other stage: 0 and nothing written. 
/// Original: `PorSecond::create_schedule` (0x7d0c40, por_second.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn por_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
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
        show_error_at(0xa7875c, 0xa8);
        return 0;
    }
    covered!("mwe_13 liga de honra schedule");
    let year = (League(this).year() as u16) as u32;
    eng_first_set_league_rounds(t, year, 0, &LIGA_DE_HONRA_ROUNDS); // mwe13: the Liga de Honra rounds
    t
}

/// Writes cup round `i` of table `t` (the draw and match dates and the round fields, `eng_cc_cup_write_cup_round`) with
/// the league's year sign-extended (the original's movsx of `.year()`).
pub(crate) unsafe fn write_play_off_round(this: League, t: usize, i: u32, r: &EngCcCupCupRound) {
    eng_cc_cup_write_cup_round(t, i, r, this.year() as i32 as u32);
}

/// The Superettan schedule: the league (stage −1) has 30 rounds (16 clubs, two legs), from 6 April to 8 November, the
/// Allsvenskan dates plus four Wednesday rounds; the promotion play-off (stage 0) is one two-legged tie from 15 November
/// (draw on 9 November).
///
/// `SweFirst::create_schedule(stage, &rounds, &extra, &league)`: stage −1 (0xff): `*league` = 1 (when given), 30 rounds,
/// extra 0, the table (`exe::crt_malloc` of 0x79e; out of memory: the box of swe_first.cpp (0xa8b53c) line 0xa8 and 0)
/// filled from `SUPERETTAN_ROUNDS` (`eng_first_set_league_rounds`). Stage 0: `*league` = 0 (when given), 1 round,
/// extra 0, the cup round table (`exe::crt_malloc` of 0x68; out of memory: the box of line 0x116 and 0) with round 0 =
/// `SUPERETTAN_PLAY_OFF` (Cup round draw date `exe::cup_set_round_draw_date` and match date
/// `exe::cup_set_round_match_date`, cdecl, with the year sign-extended; then the round fields). Any other stage: 0.
/// Original: `SweFirst::create_schedule` (0x88e8e0, swe_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn swe_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
    match stage as u8 {
        0xff => {
            if !league.is_null() {
                *league = 1;
            }
            *rounds = 0x1e;
            *extra = 0;
            let t = exe::crt_malloc(0x79e);
            if t == 0 {
                show_error_at(0xa8b53c, 0xa8);
                return 0;
            }
            covered!("mwe_13 superettan schedule");
            eng_first_set_league_rounds(t, (this.year() as u16) as u32, 0, &SUPERETTAN_ROUNDS);
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
                show_error_at(0xa8b53c, 0x116);
                return 0;
            }
            covered!("mwe_13 superettan play-off round");
            write_play_off_round(this, t, 0, &SUPERETTAN_PLAY_OFF);
            t
        }
        _ => 0,
    }
}

/// The Ykkonen schedule: the two groups (stages −1 and 0) have the same 27 rounds (three legs of 10 clubs), from 29 April
/// to 6 October, with the stage code 0x41d for the North group and 0x41e for the South one; stage 1 is the promotion
/// play-off (two-legged semi-finals of four clubs from 5 October and a two-legged final from 12 October); stage 2 the
/// relegation play-off (two two-legged ties from 5 October); stage 3 the play-off against the Veikkausliiga (one
/// two-legged tie from 19 October).
///
/// `FinFirst::create_schedule(stage, &rounds, &extra, &league)`: stage below 1 (a signed byte: −1 or 0): `*league` = 1
/// (when given), 27 rounds, extra 0x41d (stage −1) or 0x41e (any other, i.e. 0), the table (`exe::crt_malloc` of
/// 0x6db; out of memory: the box of fin_first.cpp (0x9c818c) line 0xb5 and 0) filled from `YKKONEN_ROUNDS`
/// (`eng_first_set_league_rounds`). Stage 1: `*league` = 0, 2 rounds, extra 0x458, malloc 0xd0 (box line 0x11d), the
/// rounds `YKKONEN_PLAY_OFF_SEMI` and `YKKONEN_PLAY_OFF_FINAL`. Stage 2: `*league` = 0, 1 round, extra 0x452, malloc
/// 0x68 (box 0x158), `YKKONEN_RELEGATION_PLAY_OFF`. Stage 3: `*league` = 0, 1 round, extra 0, malloc 0x68 (box 0x180),
/// `YKKONEN_PREMIER_PLAY_OFF`. The cup rounds by `write_play_off_round` (the dates with the year sign-extended).
/// Any other stage: 0.
/// Original: `FinFirst::create_schedule` (0x5931c0, fin_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn fin_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
    let s = stage as u8 as i8;
    if s < 1 {
        if !league.is_null() {
            *league = 1;
        }
        *rounds = 0x1b;
        *extra = if s as u8 == 0xff { 0x41d } else { 0x41e }; // mwe13: the North and South group codes
        let t = exe::crt_malloc(0x6db);
        if t == 0 {
            show_error_at(0x9c818c, 0xb5);
            return 0;
        }
        covered!("mwe_13 ykkonen group schedule");
        eng_first_set_league_rounds(t, (this.year() as u16) as u32, 0, &YKKONEN_ROUNDS);
        return t;
    }
    let (n, code, bytes, row, table): (i16, i16, u32, u32, &[&EngCcCupCupRound]) = match s {
        1 => (2, 0x458, 0xd0, 0x11d, &[&YKKONEN_PLAY_OFF_SEMI, &YKKONEN_PLAY_OFF_FINAL]),
        2 => (1, 0x452, 0x68, 0x158, &[&YKKONEN_RELEGATION_PLAY_OFF]),
        3 => (1, 0, 0x68, 0x180, &[&YKKONEN_PREMIER_PLAY_OFF]),
        _ => return 0,
    };
    if !league.is_null() {
        *league = 0;
    }
    *rounds = n;
    *extra = code;
    let t = exe::crt_malloc(bytes);
    if t == 0 {
        show_error_at(0x9c818c, row);
        return 0;
    }
    covered!("mwe_13 ykkonen play-off rounds");
    for (i, r) in table.iter().enumerate() {
        write_play_off_round(this, t, i as u32, r);
    }
    t
}

/// Comparer for `por_second_create_schedule`: side by side (`compare_league_round_table`: the original with the caller's outputs, the copy
/// with its own; the outputs and the written round bytes equal; the copy's table freed).
pub(crate) unsafe extern "thiscall" fn compare_por_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_league_round_table("por_second_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| por_second_create_schedule(this, stage, r, e, l))
}

/// Comparer for `swe_first_create_schedule`: side by side (`compare_league_round_table`: the original with the caller's outputs, the copy
/// with its own; the outputs and the written round bytes equal; the copy's table freed).
pub(crate) unsafe extern "thiscall" fn compare_swe_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_league_round_table("swe_first_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| swe_first_create_schedule(this, stage, r, e, l))
}

/// Comparer for `fin_first_create_schedule`: side by side as `compare_league_round_table`, but the league bytes are
/// compared at both group stages (−1 and 0) and the cup round bytes at stages 1 to 3.
pub(crate) unsafe extern "thiscall" fn compare_fin_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let pl2 = if league.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, same, _, _) = crate::comparison::side_by_side(&[(0xb67a34usize, 4usize)], || o(this, stage, rounds, extra, league), || fin_first_create_schedule(this, stage, &mut r2, &mut e2, pl2));
    let outs = (a == 0 && b == 0) || (*rounds == r2 && *extra == e2 && (league.is_null() || *league == l2));
    let table = if a != 0 && b != 0 {
        if (stage as u8 as i8) < 1 {
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
    SCOREBOARD.record(same && outs && table, || format!("fin_first_create_schedule(stage {}): outs {} table {}", stage as u8 as i8, outs, table));
    a
}
