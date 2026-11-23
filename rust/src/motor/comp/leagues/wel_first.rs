use super::*;

/// The League of Wales at career creation: discipline rule 0x28, no stages beyond the league, the rules, the table with
/// its 18 clubs, the fixtures, the order, the statistics and the predicted positions (`construct_british_division`, the
/// mould of mwb_5).
/// Original: `WelFirst::WelFirst` (0x924e30, wel_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_wel_first(this: usize, year: u32, record: usize) -> usize {
    construct_british_division(League(this), year, record, &LEAGUE_OF_WALES, "mwb_11 new league of wales")
}

/// Comparer for `construct_wel_first` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_wel_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_WEL_FIRST);
    compare_british_division_ctor("construct_wel_first", League(this), year, record, &LEAGUE_OF_WALES, || o(this, year, record))
}

/// rule: the League of Wales (18 clubs) plays 2 rounds, 3 points for a win and 1 for a draw, nobody goes up or down
/// (there is no division above or below in the game); tie-breaks goal difference, goals for and position [inference
/// from codes 1, 2, 3]; 5 substitutes named and 3 allowed; builds the round schedule; returns 1
/// Original: `WelFirst::set_up_rules` (0x925c50, wel_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_first_set_up_rules(this: usize) -> u32 {
    covered!("mwb_11 league of wales rules");
    eng_prm_apply_league_rules(League(this), &league_of_wales_rules())
}

/// Comparer for `wel_first_set_up_rules` (side by side on the rule bytes, the copy without the round table).
pub(crate) unsafe extern "thiscall" fn compare_wel_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WEL_FIRST_SET_UP_RULES);
    eng_prm_compare_league_rules("wel_first_set_up_rules", League(this), league_of_wales_rules(), || o(this), || wel_first_set_up_rules(this))
}

/// rule: at the turnover the League of Wales records the final positions (virtual +0xa4), archives the season, frees the
/// table and the rounds, moves to the next year and rebuilds the rules, the table, the fixtures and the order; then the
/// predicted positions, the board expectations and the referees renewed (`british_division_new_season` with the Welsh
/// steps, mwb_6; the error boxes of lines 0x1f9, 0x200, 0x206 and 0x20d)
/// Original: `WelFirst::start_new_season` (0x925cf0, wel_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_first_start_new_season(this: usize) -> u32 {
    british_division_new_season(League(this), &LEAGUE_OF_WALES, [0x1f9, 0x200, 0x206, 0x20d], true, 1, "mwb_11 league of wales turnover") // mwb11: the next League of Wales season
}

/// Comparer for `wel_first_start_new_season`: only the original runs; the year went up by one.
pub(crate) unsafe extern "thiscall" fn compare_wel_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WEL_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("wel_first_start_new_season", League(this), || o(this))
}

/// rule: the League of Wales plays 34 rounds (18 clubs, 2 rounds) from 19 August to 28 April of the next year, mostly on
/// Saturdays; around the year end two rounds chosen by the day the 16th round falls on (26 to 31 December and 2
/// January); no TV slots; only the main stage (`league` = 1, `extra` = 0; out of memory, the box of line 0xa1 and 0;
/// any other stage: nothing and 0; `eng_first_set_christmas_rounds`)
/// Original: `WelFirst::create_schedule` (0x9250f0, wel_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn wel_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
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
        show_error_at(0xad56b8, 0xa1);
        return 0;
    }
    covered!("mwb_12 league of wales calendar");
    eng_first_set_christmas_rounds(t, (this.year() as u16) as u32, &LEAGUE_OF_WALES_SCHEDULE, 0xad56b8);
    t
}

/// Comparer for `wel_first_create_schedule` (side by side, the written rounds).
pub(crate) unsafe extern "thiscall" fn compare_wel_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_WEL_FIRST_CREATE_SCHEDULE);
    compare_league_round_table("wel_first_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| wel_first_create_schedule(this, stage, r, e, l))
}

/// The League of Wales rule writes: match rules and round rules 0 (`league::MATCH_RULES`, `league::ROUND_DRAW_RULES`),
/// nobody promoted or relegated (no division above or below: −1), squad flags 0, 3 points for a win and 1 for a draw,
/// tie-breaks 1, 2 and 3, 2 rounds, category 2, club sides, 3 substitutions and 5 substitutes named.
pub(crate) fn league_of_wales_rules() -> Vec<(usize, u8, u32)> {
    vec![(league::MATCH_RULES, 2, 0), (league::ROUND_DRAW_RULES, 2, 0), (league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0),
         (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 0), (league::SQUAD_FLAGS, 1, 0),
         (league::WIN_POINTS, 1, 3), // mwb11: the League of Wales points for a win
         (league::TIEBREAKS + 2, 1, 3), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::LEGS, 2, 2), (league::MATCH_CATEGORY, 1, 2), (league::TIEBREAKS + 1, 1, 2),
         (league::DRAW_POINTS, 1, 1), (league::CLUB_SIDES, 1, 1), (league::TIEBREAKS, 1, 1), (league::UPPER_DIVISION_ID, 4, 0xffff_ffff),
         (league::LOWER_DIVISION_ID, 4, 0xffff_ffff), (league::SUBSTITUTES_NAMED, 1, 5)]
}

/// The League of Wales: discipline rule 0x28, no stadium requirement, no stages beyond the league; the table is
/// WelFirst::create_table (0x925b30, still in the original).
pub(crate) const LEAGUE_OF_WALES: BritishDivisionSpec = BritishDivisionSpec {
    vtable: 0x970f80,
    stadium: None,
    discipline: 0x28, // mwb11: the League of Wales discipline rule
    playoff_in_1999: false,
    split_compare: None,
    rules: 0x925c50,
    table: 0x925b30,
    file: 0xad56b8,
    rows: [0x47, 0x4e, 0x55, 0x5e, 0x65],
};

/// The League of Wales calendar with its year-end block chosen by the day the 16th round (index 15) falls on (k = day
/// − 1 December + 1 − 13: 0 to 6; the default after the box of line 0x12e).
pub(crate) const LEAGUE_OF_WALES_SCHEDULE: EngFirstChristmasSchedule = EngFirstChristmasSchedule {
    comeco: &LEAGUE_OF_WALES_BEFORE,
    reference: 15,
    adjust: 0xd,
    cases: [&LEAGUE_OF_WALES_CHRISTMAS_0, &LEAGUE_OF_WALES_CHRISTMAS_1, &LEAGUE_OF_WALES_CHRISTMAS_2, &LEAGUE_OF_WALES_CHRISTMAS_3,
            &LEAGUE_OF_WALES_CHRISTMAS_4, &LEAGUE_OF_WALES_CHRISTMAS_5, &LEAGUE_OF_WALES_CHRISTMAS_6, &LEAGUE_OF_WALES_CHRISTMAS_DEFAULT],
    end: &LEAGUE_OF_WALES_AFTER,
    row: 0x12e,
};

/// The League of Wales before the year end: 16 rounds from 19 August to 16 December (4 October on a Wednesday, code 2;
/// none on 7 and 14 October and 18 November).
pub(crate) const LEAGUE_OF_WALES_BEFORE: [EngFirstScheduleRound; 16] = [
    (19, 7, 0, 5, 1, &[(-1, -1, -1)]), // 0: 19/ago
    (26, 7, 0, 5, 1, &[(-1, -1, -1)]), // 1: 26/ago
    (2, 8, 0, 5, 1, &[(-1, -1, -1)]), // 2: 2/set
    (9, 8, 0, 5, 1, &[(-1, -1, -1)]), // 3: 9/set
    (16, 8, 0, 5, 1, &[(-1, -1, -1)]), // 4: 16/set
    (23, 8, 0, 5, 1, &[(-1, -1, -1)]), // 5: 23/set
    (30, 8, 0, 5, 1, &[(-1, -1, -1)]), // 6: 30/set
    (4, 9, 0, 2, 2, &[(-1, -1, -1)]), // 7: 4/out
    (21, 9, 0, 5, 1, &[(-1, -1, -1)]), // 8: 21/out
    (28, 9, 0, 5, 1, &[(-1, -1, -1)]), // 9: 28/out
    (4, 10, 0, 5, 1, &[(-1, -1, -1)]), // 10: 4/nov
    (11, 10, 0, 5, 1, &[(-1, -1, -1)]), // 11: 11/nov
    (25, 10, 0, 5, 1, &[(-1, -1, -1)]), // 12: 25/nov, mwb12: a League of Wales round
    (2, 11, 0, 5, 1, &[(-1, -1, -1)]), // 13: 2/dez
    (9, 11, 0, 5, 1, &[(-1, -1, -1)]), // 14: 9/dez
    (16, 11, 0, 5, 1, &[(-1, -1, -1)]), // 15: 16/dez
];

/// The League of Wales year-end rounds when the reference round falls on 13 December (case 0).
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_0: [EngFirstScheduleRound; 2] = [
    (27, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 27/dez
    (30, 11, 0, -1, 2, &[(-1, -1, -1)]), // 17: 30/dez
];

/// The League of Wales year-end rounds when the reference round falls on 14 December (case 1).
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_1: [EngFirstScheduleRound; 2] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 26/dez
    (28, 11, 0, -1, 1, &[(-1, -1, -1)]), // 17: 28/dez
];

/// The League of Wales year-end rounds when the reference round falls on 15 December (case 2).
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_2: [EngFirstScheduleRound; 2] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 26/dez
    (29, 11, 0, -1, 1, &[(-1, -1, -1)]), // 17: 29/dez
];

/// The League of Wales year-end rounds when the reference round falls on 16 December (case 3).
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_3: [EngFirstScheduleRound; 2] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 26/dez
    (30, 11, 0, -1, 1, &[(-1, -1, -1)]), // 17: 30/dez
];

/// The League of Wales year-end rounds when the reference round falls on 17 December (case 4).
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_4: [EngFirstScheduleRound; 2] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 26/dez
    (31, 11, 0, -1, 1, &[(-1, -1, -1)]), // 17: 31/dez
];

/// The League of Wales year-end rounds when the reference round falls on 18 December (case 5).
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_5: [EngFirstScheduleRound; 2] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 17: 28/dez
];

/// The League of Wales year-end rounds when the reference round falls on 19 December (case 6).
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_6: [EngFirstScheduleRound; 2] = [
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 16: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 17: 2/jan
];

/// The League of Wales year-end rounds out of range (after the error box): 28 December on Sunday and 2 January.
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_DEFAULT: [EngFirstScheduleRound; 2] = [
    (28, 11, 0, 0, 2, &[(-1, -1, -1)]), // 16: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 17: 2/jan
];

/// The League of Wales after the year end: 16 rounds from 6 January to 28 April of the next year (21 February and 17
/// April midweek, code 2).
pub(crate) const LEAGUE_OF_WALES_AFTER: [EngFirstScheduleRound; 16] = [
    (6, 0, 1, 5, 1, &[(-1, -1, -1)]), // 18: 6/jan
    (13, 0, 1, 5, 1, &[(-1, -1, -1)]), // 19: 13/jan
    (20, 0, 1, 5, 1, &[(-1, -1, -1)]), // 20: 20/jan
    (27, 0, 1, 5, 1, &[(-1, -1, -1)]), // 21: 27/jan
    (3, 1, 1, 5, 1, &[(-1, -1, -1)]), // 22: 3/fev
    (17, 1, 1, 5, 1, &[(-1, -1, -1)]), // 23: 17/fev
    (21, 1, 1, 2, 2, &[(-1, -1, -1)]), // 24: 21/fev
    (24, 1, 1, 5, 1, &[(-1, -1, -1)]), // 25: 24/fev
    (3, 2, 1, 5, 1, &[(-1, -1, -1)]), // 26: 3/mar
    (17, 2, 1, 5, 1, &[(-1, -1, -1)]), // 27: 17/mar
    (24, 2, 1, 5, 1, &[(-1, -1, -1)]), // 28: 24/mar
    (31, 2, 1, 5, 1, &[(-1, -1, -1)]), // 29: 31/mar
    (14, 3, 1, 5, 1, &[(-1, -1, -1)]), // 30: 14/abr
    (17, 3, 1, 1, 2, &[(-1, -1, -1)]), // 31: 17/abr
    (21, 3, 1, 5, 1, &[(-1, -1, -1)]), // 32: 21/abr
    (28, 3, 1, 5, 1, &[(-1, -1, -1)]), // 33: 28/abr
];
