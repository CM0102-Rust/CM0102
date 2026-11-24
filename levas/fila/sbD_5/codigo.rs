// ---------- sbD_5: the Belgian Third Division schedule (bel_third.cpp) ----------

/// The Belgian Third Division league rounds (stages −1 and 0): thirty rounds on weekday 6 [inference: Saturday], code 1, one empty
/// TV slot each, from 3 September to 17 December and from 7 January to 6 May of the next year (no round on 5 November
/// nor on 11 February and 11 March, nor on 1 April).
pub(crate) const BEL_THIRD_LEAGUE_ROUNDS: [SpaSecondScheduleRound; 30] = [
    (3, 8, 0, 6, 1, &[(-1, -1, -1)]),   // 0: 3 Sep
    (10, 8, 0, 6, 1, &[(-1, -1, -1)]),  // 1: 10 Sep
    (17, 8, 0, 6, 1, &[(-1, -1, -1)]),  // 2: 17 Sep
    (24, 8, 0, 6, 1, &[(-1, -1, -1)]),  // 3: 24 Sep
    (1, 9, 0, 6, 1, &[(-1, -1, -1)]),   // 4: 1 Oct
    (8, 9, 0, 6, 1, &[(-1, -1, -1)]),   // 5: 8 Oct
    (15, 9, 0, 6, 1, &[(-1, -1, -1)]),  // 6: 15 Oct
    (22, 9, 0, 6, 1, &[(-1, -1, -1)]),  // 7: 22 Oct
    (29, 9, 0, 6, 1, &[(-1, -1, -1)]),  // 8: 29 Oct
    (12, 10, 0, 6, 1, &[(-1, -1, -1)]), // 9: 12 Nov
    (19, 10, 0, 6, 1, &[(-1, -1, -1)]), // 10: 19 Nov
    (26, 10, 0, 6, 1, &[(-1, -1, -1)]), // 11: 26 Nov
    (3, 11, 0, 6, 1, &[(-1, -1, -1)]),  // 12: 3 Dec
    (10, 11, 0, 6, 1, &[(-1, -1, -1)]), // 13: 10 Dec
    (17, 11, 0, 6, 1, &[(-1, -1, -1)]), // 14: 17 Dec
    (7, 0, 1, 6, 1, &[(-1, -1, -1)]),   // 15: 7 Jan +1
    (14, 0, 1, 6, 1, &[(-1, -1, -1)]),  // 16: 14 Jan +1
    (21, 0, 1, 6, 1, &[(-1, -1, -1)]),  // 17: 21 Jan +1
    (28, 0, 1, 6, 1, &[(-1, -1, -1)]),  // 18: 28 Jan +1
    (4, 1, 1, 6, 1, &[(-1, -1, -1)]),   // 19: 4 Feb +1
    (18, 1, 1, 6, 1, &[(-1, -1, -1)]),  // 20: 18 Feb +1
    (25, 1, 1, 6, 1, &[(-1, -1, -1)]),  // 21: 25 Feb +1
    (4, 2, 1, 6, 1, &[(-1, -1, -1)]),   // 22: 4 Mar +1
    (18, 2, 1, 6, 1, &[(-1, -1, -1)]),  // 23: 18 Mar +1
    (25, 2, 1, 6, 1, &[(-1, -1, -1)]),  // 24: 25 Mar +1
    (8, 3, 1, 6, 1, &[(-1, -1, -1)]),   // 25: 8 Apr +1
    (15, 3, 1, 6, 1, &[(-1, -1, -1)]),  // 26: 15 Apr +1
    (22, 3, 1, 6, 1, &[(-1, -1, -1)]),  // 27: 22 Apr +1
    (29, 3, 1, 6, 1, &[(-1, -1, -1)]),  // 28: 29 Apr +1
    (6, 4, 1, 6, 1, &[(-1, -1, -1)]),   // 29: 6 May +1
];

/// Stage 7: a single two-legged round [inference: the final between the two group winners], drawn on 7 May and
/// played on 13 May of the next year, legs 4 days apart; code 0x96, two-leg tie-break 3, category 5.
pub(crate) const BEL_THIRD_FINAL_ROUNDS: [EngCcCupCupRound; 1] = [
    cql10r((7, 4, 1, 0), (0xd, 4, 1, 6, 2, 0), 0x96, (0, 0, 3), 5, (2, 1, 2, 0), (0, 2, 4), (0, 0, 0)),
];

/// Stage 8: the first promotion play-off, three rounds in May of the next year: two two-legged rounds (six new clubs;
/// then four, the six already in counted), then a single match on 31 May with 0x28 as the match value; category 5,
/// two-leg tie-break 3 (the last round: single-match tie-break 3).
pub(crate) const BEL_THIRD_PROMOTION_PLAY_OFF_ROUNDS: [EngCcCupCupRound; 3] = [
    cql10r((7, 4, 1, 0), (0xd, 4, 1, 6, 2, 0), 0x78, (0, 0, 3), 5, (6, 3, 6, 0), (0, 2, 4), (0, 0, 0)),
    cql10r((0xe, 4, 1, 0), (0x14, 4, 1, 6, 2, 0), 0x82, (0, 0, 3), 5, (4, 2, 1, 6), (0, 2, 4), (0, 0, 0)),
    cql10r((0x15, 4, 1, 0), (0x1f, 4, 1, 2, 2, 0x28), 0x96, (0, 3, 0), 5, (2, 1, 0, 0), (0, 1, 0), (0, 0, 0)),
];

/// Stage 9: the second promotion play-off, three single-match rounds in May of the next year (twelve clubs, then
/// eight with two new and twelve already in, then four); category 5, single-match tie-breaks 3 and 0x403.
pub(crate) const BEL_THIRD_SECOND_PLAY_OFF_ROUNDS: [EngCcCupCupRound; 3] = [
    cql10r((7, 4, 1, 0), (0xd, 4, 1, 6, 2, 0), 0xa, (0, 3, 0), 5, (0xc, 6, 0xc, 0), (0, 1, 0), (0, 0, 0)),
    cql10r((0xe, 4, 1, 0), (0x14, 4, 1, 6, 2, 0), 0x78, (0, 0x403, 0), 5, (8, 4, 2, 0xc), (0, 1, 0), (0, 0, 0)),
    cql10r((0x15, 4, 1, 0), (0x1b, 4, 1, 6, 2, 0), 0x82, (0, 0x403, 0), 5, (4, 2, 0, 0), (0, 1, 0), (0, 0, 0)),
];

/// The play-off rounds, extra id and box line of a cup stage of the Belgian Third Division (7, 8 and 9).
fn bel_third_play_off_stage(stage: i8) -> Option<(&'static [EngCcCupCupRound], i16, u32)> {
    match stage {
        7 => Some((&BEL_THIRD_FINAL_ROUNDS, 0, 0x14a)),
        8 => Some((&BEL_THIRD_PROMOTION_PLAY_OFF_ROUNDS, 0x458, 0x172)),
        9 => Some((&BEL_THIRD_SECOND_PLAY_OFF_ROUNDS, 0x452, 0x1c0)),
        _ => None,
    }
}

/// The schedule of the Belgian Third Division (`create_schedule(stage, &rounds, &extra, &is_league)`):
/// - stages below 1 (the two groups, −1 and 0): is_league (if asked) = 1, 30 rounds, extra 0x41f for stage −1 and
///   0x420 otherwise; the table (`exe::crt_malloc` of 30 × 0x41; out of memory, the box at line 0xb8 and 0) with
///   `BEL_THIRD_LEAGUE_ROUNDS` (the year as the word +0x40);
/// - stages 7, 8 and 9 (the final and the two promotion play-offs): is_league = 0, the round count, the extra id (0,
///   0x458, 0x452); the table of 0x68-byte rounds (out of memory, the box at line 0x14a, 0x172 or 0x1c0 and 0) with the
///   cup rounds (`eng_cc_cup_write_cup_round`, the year sign-extended);
/// - other stages: 0, nothing written.
/// Original: `BelThird::create_schedule` (0x421a00, bel_third.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bel_third_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
    let stage = stage as u8 as i8;
    let file = 0x989d04; // bel_third.cpp
    if stage < 1 {
        if !league.is_null() {
            *league = 1;
        }
        *rounds = 30;
        *extra = 0x41f + (stage != -1) as i16;
        let t = exe::crt_malloc(30 * 0x41);
        if t == 0 {
            show_error_at(file, 0xb8);
            return 0;
        }
        covered!("sbD_5 belgian third division league schedule");
        spa_second_set_league_rounds(t, (this.year() as u16) as u32, 0, &BEL_THIRD_LEAGUE_ROUNDS);
        return t;
    }
    let Some((table, id, row)) = bel_third_play_off_stage(stage) else {
        return 0;
    };
    if !league.is_null() {
        *league = 0;
    }
    *rounds = table.len() as i16;
    *extra = id;
    let t = exe::crt_malloc(table.len() as u32 * 0x68);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    covered!("sbD_5 belgian third division play-off schedule");
    let year = this.year() as i32 as u32;
    for (i, r) in table.iter().enumerate() {
        eng_cc_cup_write_cup_round(t, i as u32, r, year);
    }
    t
}

/// Comparer for `bel_third_create_schedule`: side by side from the same [0xb67a34], the outputs and the written bytes
/// of both tables (`compare_stage_schedule`: the league rounds or the cup rounds by the stage); the copy's table freed.
pub(crate) unsafe extern "thiscall" fn compare_bel_third_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    let f = stage as u8 as i8;
    let plan = if f < 1 {
        BraChampCupStagePlan::Grupos(&[], 0, 0, "")
    } else if let Some((table, ..)) = bel_third_play_off_stage(f) {
        BraChampCupStagePlan::Mata(table, &[], 0, "")
    } else {
        BraChampCupStagePlan::Nada
    };
    compare_stage_schedule("bel_third_create_schedule", stage, plan, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| bel_third_create_schedule(this, stage, r, e, l))
}
