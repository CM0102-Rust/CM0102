// ---------- rx09_7: the Swedish Second Division at career creation, its rules and its round tables (swe_second.cpp) ----------

/// The source file of the Swedish Second Division (swe_second.cpp), for the error boxes.
pub(crate) const SWEDISH_SECOND_SOURCE: usize = 0xa8b5b0;

/// The rules of the Swedish Second Division (six regional sections of 12): 2 legs, 3 points a win and 1 a draw, league
/// matches, national clubs; tie-breaks 1, 2 (goal difference, goals for; +0xc7 = 0 ends); none up directly, 1 to the
/// promotion play-off, 1 to the relegation play-off, 2 down; the First Division above, nothing below; the rule bits
/// 0x42 (a word). The substitutes, the substitutions, +0xab, +0xdb and +0x52 are not written.
pub(crate) unsafe fn swedish_second_rules() -> Vec<(usize, u8, u32)> {
    vec![(league::LEGS, 2, 2), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::CLUB_SIDES, 1, 1),
        (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 0), (league::PROMOTED_COUNT, 1, 0),
        (league::PROMOTION_PLAYOFF_COUNT, 1, 1), (league::RELEGATION_PLAYOFF_COUNT, 1, 1), (league::RELEGATED_COUNT, 1, 2), // rx09_7: two go down
        (league::UPPER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION) as u32), (league::LOWER_DIVISION_ID, 4, 0xffff_ffff),
        (league::LEAGUE_FLAGS, 2, 0x42)]
}

/// The Swedish Second Division: vtable 0x9702a0, the Swedish discipline rule 0x1c, seven stages after the North
/// section (the five other sections, the promotion play-off and the relegation play-off); the rules written inline
/// (no box; SweSecond::set_up_rules 0x892380 is not called), SweSecond::create_table (0x8921b0, line 0x50), sort
/// 0x57, statistics 0x5f, fixtures 0x68; SweSecond::create_section_stage (0x892670) for the five other sections and
/// SweSecond::set_predicted_positions (0x893470).
pub(crate) const SWEDISH_SECOND: GroupedLeagueCtor = GroupedLeagueCtor {
    shape: LeagueWithStages {
        division: LeagueDivision { vtabela: 0x9702a0, discipline: 0x1c, rules: 0x892380, table: 0x8921b0, file: SWEDISH_SECOND_SOURCE, rows: [0, 0x50, 0x68, 0x57, 0x5f], low: false },
        stages: 7, // rx09_7: the seven stages of the Swedish Second Division
        finish: None,
    },
    inline_rules: Some(swedish_second_rules),
    group_stage: 0x892670,
    groups: 5,
    predicted: 0x893470,
};

/// Builds the Swedish Second Division at game creation (`construct_grouped_league` with `SWEDISH_SECOND`): the North
/// section in the league's own table, the other five as stages 0 to 4, then the predicted positions.
/// Original: `SweSecond::SweSecond` (0x8916f0, swe_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_swe_second(this: usize, year: u32, record: usize) -> usize {
    construct_grouped_league(this, year, record, &SWEDISH_SECOND, "rx09_7 new swedish second division")
}

/// Comparer for `construct_swe_second` (`compare_league_with_stages_ctor` with the shape of `SWEDISH_SECOND`): only the
/// original runs (it allocates the league, the stages and the statistics).
pub(crate) unsafe extern "thiscall" fn compare_construct_swe_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_league_with_stages_ctor("construct_swe_second", League(this), year, record, &SWEDISH_SECOND.shape, || o(this, year, record))
}

/// Rule: the Swedish Second Division: 3 points a win and 1 a draw; tie-breaks goal difference, goals for; each section
/// winner to the promotion play-off, each runner-up to the relegation play-off, the last two down; the First Division
/// above (`swedish_second_rules`, then the main round table into `league::ROUND_DATES`).
/// Original: `SweSecond::set_up_rules` (0x892380, swe_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_second_set_up_rules(this: usize) -> u32 {
    covered!("rx09_7 swedish second division rules");
    eng_prm_apply_league_rules(League(this), &swedish_second_rules())
}

/// Comparer for `swe_second_set_up_rules` (the rehearsal of `eng_prm_compare_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_swe_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    eng_prm_compare_league_rules("swe_second_set_up_rules", League(this), swedish_second_rules(), || o(this), || swe_second_set_up_rules(this))
}

/// The Swedish Second Division sections: 22 rounds on Sundays (weekday 6) from 6 April to 8 November, the summer break
/// from 7 June to 19 July and a Saturday round on 31 October; no TV slots.
pub(crate) const SWEDISH_SECOND_SECTION_ROUNDS: [SpaSecondScheduleRound; 22] = [
    (6, 3, 0, 6, 1, &[(-1, -1, -1)]), // 0: 6/abr (rx09_7: the first section round)
    (13, 3, 0, 6, 1, &[(-1, -1, -1)]), // 1: 13/abr
    (20, 3, 0, 6, 1, &[(-1, -1, -1)]), // 2: 20/abr
    (27, 3, 0, 6, 1, &[(-1, -1, -1)]), // 3: 27/abr
    (4, 4, 0, 6, 1, &[(-1, -1, -1)]), // 4: 4/mai
    (11, 4, 0, 6, 1, &[(-1, -1, -1)]), // 5: 11/mai
    (18, 4, 0, 6, 1, &[(-1, -1, -1)]), // 6: 18/mai
    (7, 5, 0, 6, 1, &[(-1, -1, -1)]), // 7: 7/jun
    (19, 6, 0, 6, 1, &[(-1, -1, -1)]), // 8: 19/jul
    (2, 7, 0, 6, 1, &[(-1, -1, -1)]), // 9: 2/ago
    (9, 7, 0, 6, 1, &[(-1, -1, -1)]), // 10: 9/ago
    (16, 7, 0, 6, 1, &[(-1, -1, -1)]), // 11: 16/ago
    (23, 7, 0, 6, 1, &[(-1, -1, -1)]), // 12: 23/ago
    (30, 7, 0, 6, 1, &[(-1, -1, -1)]), // 13: 30/ago
    (6, 8, 0, 6, 1, &[(-1, -1, -1)]), // 14: 6/set
    (13, 8, 0, 6, 1, &[(-1, -1, -1)]), // 15: 13/set
    (20, 8, 0, 6, 1, &[(-1, -1, -1)]), // 16: 20/set
    (4, 9, 0, 6, 1, &[(-1, -1, -1)]), // 17: 4/out
    (18, 9, 0, 6, 1, &[(-1, -1, -1)]), // 18: 18/out
    (25, 9, 0, 6, 1, &[(-1, -1, -1)]), // 19: 25/out
    (31, 9, 0, 5, 1, &[(-1, -1, -1)]), // 20: 31/out
    (8, 10, 0, 6, 1, &[(-1, -1, -1)]), // 21: 8/nov
];

/// The promotion play-off of the Swedish Second Division (stage 5): drawn on 9 November (weekday 0), three ties of two
/// legs 7 days apart from 15 November (weekday 6) between the six section winners, code 0xa0, category 5, two-leg
/// tie-break 3, no prizes.
pub(crate) const SWEDISH_SECOND_PROMOTION_PLAY_OFF: EngCcCupCupRound = cql10r((9, 10, 0, 0), (0xf, 10, 0, 6, 1, 0), 0xa0, (0, 0, 3), 5, (6, 3, 6, 0), (0, 2, 7), (0, 0, 0));

/// The relegation play-off of the Swedish Second Division (stage 6), two rounds: drawn on 9 November, six ties of two
/// legs from 15 November between twelve lower division clubs (code 0xa); then drawn on 23 November, six ties of two
/// legs from 29 November where the six section runners-up come in (code 0x14, tie-break 0x400); category 5, two-leg
/// tie-break 3, 7 days between the legs, no prizes.
pub(crate) const SWEDISH_SECOND_RELEGATION_PLAY_OFF: [EngCcCupCupRound; 2] = [
    cql10r((9, 10, 0, 0), (0xf, 10, 0, 6, 1, 0), 0xa, (0, 0, 3), 5, (0xc, 6, 0xc, 0), (0, 2, 7), (0, 0, 0)),
    cql10r((0x17, 10, 0, 0), (0x1d, 10, 0, 6, 1, 0), 0x14, (0, 0x400, 3), 5, (0xc, 6, 6, 0xc), (0, 2, 7), (0, 0, 0)), // rx09_7: the runners-up come in
];

/// The round tables of the Swedish Second Division by stage (the signed byte of the argument): stages up to 4 (the
/// North section at −1, the other five at 0 to 4) the 22 section rounds, extra = stage + 0x46d written only for stages
/// −1 to 4 (line 0xb9); stage 5 the promotion play-off, extra 0x438 (line 0x12b); stage 6 the relegation play-off,
/// extra 0x452 (line 0x153); later stages 0 with nothing written.
/// Original: `SweSecond::create_schedule` (0x891a60, swe_second.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn swe_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let l = League(this);
    let s = stage as u8 as i8;
    if s < 5 {
        let code = if s >= -1 { Some((s as i16).wrapping_add(0x46d)) } else { None };
        league_round_table(l, rounds, extra, league, &SWEDISH_SECOND_SECTION_ROUNDS, code, SWEDISH_SECOND_SOURCE, 0xb9, "rx09_7 swedish second section rounds")
    } else if s == 5 {
        cup_round_table(l, rounds, extra, league, &[SWEDISH_SECOND_PROMOTION_PLAY_OFF], 0x438, SWEDISH_SECOND_SOURCE, 0x12b, "rx09_7 swedish second promotion play-off round")
    } else if s == 6 {
        cup_round_table(l, rounds, extra, league, &SWEDISH_SECOND_RELEGATION_PLAY_OFF, 0x452, SWEDISH_SECOND_SOURCE, 0x153, "rx09_7 swedish second relegation play-off rounds")
    } else {
        0
    }
}

/// Comparer for `swe_second_create_schedule` (`compare_league_or_cup_schedule`; cup rounds at stages 5 and 6).
pub(crate) unsafe extern "thiscall" fn compare_swe_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    let cup = matches!(stage as u8 as i8, 5 | 6);
    compare_league_or_cup_schedule("swe_second_create_schedule", stage, cup, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| swe_second_create_schedule(this, stage, r, e, l))
}
