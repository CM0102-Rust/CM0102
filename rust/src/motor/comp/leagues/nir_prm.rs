use super::*;

/// Rule: the Northern Irish turnover is led by the Premier Division: promotion and relegation, the statistics archived,
/// the next year, the rules, the ten-club table, the fixtures and the order rebuilt; then the First Division turns over,
/// and the predicted positions, the board expectations and the referee renewal follow.
///
/// `start_irish_premier_season` with the chain argument 1, NirPrm::create_table (0x78ec60), the Northern Irish First
/// Division and the error box lines 0x1c0, 0x1c7, 0x1cd and 0x1d4 of nir_prm.cpp.
/// Original: `NirPrm::start_new_season` (0x78ee30, nir_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_prm_start_new_season(this: usize) -> u32 {
    start_irish_premier_season(League(this), 1, 0x78ec60, KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION, 0xa696d0, [0x1c0, 0x1c7, 0x1cd, 0x1d4], 1) // mwa15: the Northern Irish turnover
}

pub(crate) unsafe extern "thiscall" fn compare_nir_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NIR_PRM_START_NEW_SEASON);
    compare_state_league_new_season("nir_prm_start_new_season", League(this), || o(this))
}

/// Rule: the Northern Irish Premier Division plays 36 matchdays (ten clubs meeting four times), from 12 August to 28
/// April of the next year, mostly on Saturdays with a few midweek rounds; the two Christmas rounds depend on the
/// weekday of the 16 December round; only the league stage (no play-off).
///
/// Only stage −1 (the byte; otherwise 0 with nothing written): `northern_irish_league_schedule` with
/// `NORTHERN_IRISH_PREMIER_END` and the error box at line 0xa0 of nir_prm.cpp.
/// Original: `NirPrm::create_schedule` (0x78e330, nir_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn nir_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    northern_irish_league_schedule(League(this), rounds, extra, out, &NORTHERN_IRISH_PREMIER_END, 0xa696d0, 0xa0, "mwa_20 northern irish premier schedule")
}

pub(crate) unsafe extern "thiscall" fn compare_nir_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_NIR_PRM_CREATE_SCHEDULE);
    compare_c1_schedule("nir_prm_create_schedule", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, l| nir_prm_create_schedule(this, stage, r, e, l))
}

/// Rule: the Northern Irish Premier Division at career creation: the year and the database competition, the Northern
/// Irish discipline rules and, with nothing saved, a plain league (no extra stage): the rules, the table, the fixtures,
/// the order, the statistics and the predicted positions.
///
/// `construct_irish_league` with `NORTHERN_IRISH_PREMIER_CONSTRUCTOR` (checked instruction by instruction against
/// JapJ2::JapJ2 0x6644c0: only the vtable, the discipline rule, the two routines, the file and the lines change).
/// Original: `NirPrm::NirPrm` (0x78e070, nir_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_nir_prm(this: usize, year: u32, record: usize) -> usize {
    construct_irish_league(League(this), year, record, &NORTHERN_IRISH_PREMIER_CONSTRUCTOR, "mwa_9 northern irish premier new") // mwa9: the Northern Irish Premier
}

pub(crate) unsafe extern "thiscall" fn compare_construct_nir_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_NIR_PRM);
    compare_irish_league_constructor("construct_nir_prm", League(this), year, record, &NORTHERN_IRISH_PREMIER_CONSTRUCTOR, || o(this, year, record))
}

/// Rule: the Northern Irish Premier Division: ten clubs meeting four times, 3 points for a win and 1 for a draw, ties
/// split by goal difference and goals for; the last goes down to the First Division and the second last plays the
/// promotion/relegation play-off; 3 substitutes on the bench and 3 substitutions.
///
/// `set_up_irish_league_rules` with `NORTHERN_IRISH_PREMIER_RULES` (the fields, then the round table of the main stage
/// into `league::ROUND_DATES`). Returns 1.
/// Original: `NirPrm::set_up_rules` (0x78ed80, nir_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwa_9 northern irish premier rules");
    set_up_irish_league_rules(League(this), &NORTHERN_IRISH_PREMIER_RULES)
}

pub(crate) unsafe extern "thiscall" fn compare_nir_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NIR_PRM_SET_UP_RULES);
    compare_irish_league_rules("nir_prm_set_up_rules", this, &NORTHERN_IRISH_PREMIER_RULES, || o(this))
}

/// Rule: the Northern Irish Premier Division table has ten clubs, the Premier Division clubs in database order; when
/// full, the prize fund for a 500 share per place, paid by table position at the season's end.
///
/// `ita_ser_a_fill_league_table`(10 clubs, `KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION`; out of memory, the error
/// box at line 0x15a of nir_prm.cpp) (ItaSerA::create_table's template, checked instruction by instruction: the end
/// without the TV money); not full, 0; full, the prize fund (League::calc_prize_fund `exe::league_calc_prize_fund`(this,
/// 500), thiscall, ret 4) in `league::PRIZE_FUND`, `league::PRIZE_MODE` = 1 and 1.
/// Original: `NirPrm::create_table` (0x78ec60, nir_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_prm_create_table(this: usize) -> u32 {
    if ita_ser_a_fill_league_table(League(this), 0xa, 0xa696d0, 0x15a, KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION) != Some(true) {
        return 0;
    }
    covered!("mwa_9 northern irish premier table");
    set_prize_fund(League(this), 0x1f4); // mwa9: the Northern Irish Premier quota
    1
}

pub(crate) unsafe extern "thiscall" fn compare_nir_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NIR_PRM_CREATE_TABLE);
    ita_ser_a_compare_league_table("nir_prm_create_table", League(this), 0xa, KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION, Some(0x1f4), || o(this))
}

/// The Northern Irish Premier Division's last fourteen matchdays (rounds 22 to 35), from 6 January to 28 April of the
/// next year, on weekday 5 (one midweek round on 11 April).
pub(crate) const NORTHERN_IRISH_PREMIER_END: [FraThirdLeagueRound; 14] = [
    (6, 0, 1, 5, 1, &[(-1, -1, -1)]), // 22: 6 jan (+1)
    (13, 0, 1, 5, 1, &[(-1, -1, -1)]), // 23: 13 jan (+1)
    (17, 0, 1, 5, 1, &[(-1, -1, -1)]), // 24: 17 jan (+1)
    (27, 0, 1, 5, 1, &[(-1, -1, -1)]), // 25: 27 jan (+1)
    (3, 1, 1, 5, 1, &[(-1, -1, -1)]), // 26: 3 feb (+1)
    (10, 1, 1, 5, 1, &[(-1, -1, -1)]), // 27: 10 feb (+1)
    (24, 1, 1, 5, 1, &[(-1, -1, -1)]), // 28: 24 feb (+1)
    (3, 2, 1, 5, 1, &[(-1, -1, -1)]), // 29: 3 mar (+1)
    (17, 2, 1, 5, 1, &[(-1, -1, -1)]), // 30: 17 mar (+1)
    (31, 2, 1, 5, 1, &[(-1, -1, -1)]), // 31: 31 mar (+1)
    (11, 3, 1, 2, 2, &[(-1, -1, -1)]), // 32: 11 apr (+1)
    (14, 3, 1, 5, 1, &[(-1, -1, -1)]), // 33: 14 apr (+1)
    (21, 3, 1, 5, 1, &[(-1, -1, -1)]), // 34: 21 apr (+1)
    (28, 3, 1, 5, 1, &[(-1, -1, -1)]), // 35: 28 apr (+1)
];

/// The Northern Irish Premier Division's constructor: the vtable 0x96dccc, discipline rule
/// `discipline_rule::NORTHERN_IRELAND`, no extra stage, NirPrm::set_up_rules (0x78ed80) and NirPrm::create_table
/// (0x78ec60), the lines 0x45, 0x4c, 0x53, 0x5c and 0x63 of nir_prm.cpp.
pub(crate) const NORTHERN_IRISH_PREMIER_CONSTRUCTOR: IrishLeagueConstructor = IrishLeagueConstructor { vtable: 0x96dccc, discipline: discipline_rule::NORTHERN_IRELAND,
    play_off_stage: false, reset_ten_slots: false, rules: 0x78ed80, table: 0x78ec60, file: 0xa696d0, rows: [0x45, 0x4c, 0x53, 0x5c, 0x63] };

/// The Northern Irish Premier Division's rules: four meetings (`league::LEGS` = 4), 3 points for a win and 1 for a
/// draw, match category 2, `league::MATCH_RULES` and `league::ROUND_DRAW_RULES` 0, club sides, the tie-breaks 1, 2
/// (goal difference, goals for) and 4, no promotion, one club to the relegation play-off and one relegated, the squad
/// flags 0, 3 substitutes on the bench and 3 substitutions, `league::LEAGUE_FLAGS` 3 (champion and promotion status
/// bits [inference]); no division above, the First Division below.
pub(crate) const NORTHERN_IRISH_PREMIER_RULES: IrishLeagueRules = IrishLeagueRules {
    writes: &[(league::LEGS, 2, 4), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0),
        (league::ROUND_DRAW_RULES, 2, 0), (league::CLUB_SIDES, 1, 1), (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 4),
        (league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 1), (league::RELEGATED_COUNT, 1, 1), // mwa9: one relegated, one in the play-off
        (league::SQUAD_FLAGS, 1, 0), (league::SUBSTITUTES_NAMED, 1, 3), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::LEAGUE_FLAGS, 2, 3)],
    upper: None,
    lower: Some(KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION),
};
