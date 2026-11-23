use super::*;

/// Rule: the Northern Irish First Division plays 36 matchdays (ten clubs meeting four times), on the Premier's dates up
/// to Christmas and on Sundays after it [inference: weekday 6]; then the promotion play-off against the Premier's second
/// last, two legs seven days apart from 5 May.
///
/// Stage −1: `northern_irish_league_schedule` with `NORTHERN_IRISH_FIRST_END` and the error box at line 0xae of
/// nir_first.cpp. Stage 0: `out` (when given) = 0, `rounds` = 1, `extra` = 0, the table (`exe::crt_malloc` of 0x68; out of
/// memory, the box at line 0x163 and 0) with `NORTHERN_IRISH_PLAY_OFF` (`eng_cc_cup_write_cup_round`; the original writes
/// the same fields in another order). Other stages: 0.
/// Original: `NirFirst::create_schedule` (0x78b310, nir_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn nir_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let this = League(this);
    match stage as u8 {
        0xff => northern_irish_league_schedule(this, rounds, extra, out, &NORTHERN_IRISH_FIRST_END, 0xa6965c, 0xae, "mwa_20 northern irish first schedule"),
        0 => {
            if !out.is_null() {
                *out = 0;
            }
            *rounds = 1;
            *extra = 0;
            let t = exe::crt_malloc(0x68);
            if t == 0 {
                show_error_at(0xa6965c, 0x163);
                return 0;
            }
            covered!("mwa_20 northern irish play-off round");
            eng_cc_cup_write_cup_round(t, 0, &NORTHERN_IRISH_PLAY_OFF, this.year() as i32 as u32);
            t
        }
        _ => 0,
    }
}

pub(crate) unsafe extern "thiscall" fn compare_nir_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_NIR_FIRST_CREATE_SCHEDULE);
    compare_c1_schedule("nir_first_create_schedule", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, l| nir_first_create_schedule(this, stage, r, e, l))
}

/// Rule: the Northern Irish First Division at career creation: the year and the database competition, the Northern
/// Irish discipline rules and, with nothing saved, the league and one extra stage (the promotion play-off): the rules,
/// the table, the fixtures, the order, the statistics and the predicted positions.
///
/// `construct_irish_league` with `NORTHERN_IRISH_FIRST_CONSTRUCTOR` (checked instruction by instruction against
/// EngFirst::EngFirst 0x571680: only the vtable, the discipline rule, the two routines, the file and the last line).
/// Original: `NirFirst::NirFirst` (0x78b010, nir_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_nir_first(this: usize, year: u32, record: usize) -> usize {
    construct_irish_league(League(this), year, record, &NORTHERN_IRISH_FIRST_CONSTRUCTOR, "mwa_10 northern irish first new") // mwa10: the Northern Irish First
}

pub(crate) unsafe extern "thiscall" fn compare_construct_nir_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_NIR_FIRST);
    compare_irish_league_constructor("construct_nir_first", League(this), year, record, &NORTHERN_IRISH_FIRST_CONSTRUCTOR, || o(this, year, record))
}

/// Rule: the Northern Irish First Division: ten clubs meeting four times, 3 points for a win and 1 for a draw, ties split
/// by goal difference and goals for; the champion goes up to the Premier Division and the runner-up plays the promotion
/// play-off; no relegation (no division below); 3 substitutes on the bench and 3 substitutions.
///
/// `set_up_irish_league_rules` with `NORTHERN_IRISH_FIRST_RULES`. Returns 1.
/// Original: `NirFirst::set_up_rules` (0x78bd70, nir_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_first_set_up_rules(this: usize) -> u32 {
    covered!("mwa_10 northern irish first rules");
    set_up_irish_league_rules(League(this), &NORTHERN_IRISH_FIRST_RULES)
}

pub(crate) unsafe extern "thiscall" fn compare_nir_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NIR_FIRST_SET_UP_RULES);
    compare_irish_league_rules("nir_first_set_up_rules", this, &NORTHERN_IRISH_FIRST_RULES, || o(this))
}

/// Rule: the Northern Irish First Division table has ten clubs, the division's in database order; when full, the prize
/// fund for a 300 share per place, paid by table position at the season's end.
///
/// `ita_ser_a_fill_league_table`(10 clubs, `KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION`; out of memory, the error box
/// at line 0x190 of nir_first.cpp); not full, 0; full, the prize fund (`exe::league_calc_prize_fund`(this, 300)) in
/// `league::PRIZE_FUND`, `league::PRIZE_MODE` = 1 and 1.
/// Original: `NirFirst::create_table` (0x78bc50, nir_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_first_create_table(this: usize) -> u32 {
    if ita_ser_a_fill_league_table(League(this), 0xa, 0xa6965c, 0x190, KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION) != Some(true) {
        return 0;
    }
    covered!("mwa_10 northern irish first table");
    set_prize_fund(League(this), 0x12c); // mwa10: the Northern Irish First quota
    1
}

pub(crate) unsafe extern "thiscall" fn compare_nir_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NIR_FIRST_CREATE_TABLE);
    ita_ser_a_compare_league_table("nir_first_create_table", League(this), 0xa, KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION, Some(0x12c), || o(this))
}

/// Rule: the Northern Irish First Division turnover: the statistics archived, the table and the round table cleared,
/// the year moved on and the rules, the table, the fixtures and the order rebuilt; then the predicted positions, the
/// board expectations and the referee renewal.
///
/// BraNatSecond::start_new_season's template (checked instruction by instruction: only the table routine,
/// NirFirst::create_table 0x78bc50, the file and the error box lines 0x1f5, 0x1fc, 0x202 and 0x209 change):
/// `league::SEASON_CLOSED` = 0, `rebuild_season` and, without a box, `set_expectations_and_renew_officials` (whose
/// return it passes on; 0 after a box).
/// Original: `NirFirst::start_new_season` (0x78be10, nir_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x78bc50, 0xa6965c, [0x1f5, 0x1fc, 0x202, 0x209], 1) { // mwa13: the next season of the Northern Irish First
        return 0;
    }
    covered!("mwa_13 northern irish first turnover");
    set_expectations_and_renew_officials(this)
}

pub(crate) unsafe extern "thiscall" fn compare_nir_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NIR_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("nir_first_start_new_season", League(this), || o(this))
}

/// Rule: the Northern Irish First Division's league stage ends only when the Premier Division above has finished: then
/// the Premier's positions are updated and the promotion play-off is built; past the last stage, the error box.
///
/// `advance_irish_first_stage` with the Northern Irish Premier Division (`KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION`),
/// the error box at line 0x226 of nir_first.cpp and NirFirst::create_playoff_stage (0x78c0a0).
/// Original: `NirFirst::advance_stage` (0x78bfd0, nir_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_first_advance_stage(this: usize) {
    advance_irish_first_stage(League(this), KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION, 0xa6965c, 0x226, 0x78c0a0) // mwa13: the Northern Irish play-off waits for the Premier
}

pub(crate) unsafe extern "thiscall" fn compare_nir_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_NIR_FIRST_ADVANCE_STAGE);
    compare_irish_first_advance_stage("nir_first_advance_stage", League(this), || o(this))
}

/// Rule: the Northern Irish promotion play-off: the First Division runner-up against the Premier Division's second last
/// (the clubs with the play-off status in each table), in a two-club stage.
///
/// `create_irish_promotion_play_off` with the Northern Irish Premier Division and the error box lines 0x252, 0x262 and
/// 0x283 of nir_first.cpp.
/// Original: `NirFirst::create_playoff_stage` (0x78c0a0, nir_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_first_create_playoff_stage(this: usize) {
    create_irish_promotion_play_off(League(this), KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION, 0xa6965c, [0x252, 0x262, 0x283], "mwa_13 northern irish promotion play-off")
}

pub(crate) unsafe extern "thiscall" fn compare_nir_first_create_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_NIR_FIRST_CREATE_PLAYOFF_STAGE);
    compare_irish_promotion_play_off("nir_first_create_playoff_stage", League(this), KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION, || o(this))
}

/// The 36-matchday schedule of a Northern Irish league (the league's virtual `comp_vtable::CREATE_SCHEDULE` at stage
/// −1): `out` (when given) = 1, `rounds` = 36, `extra` = 0; the table (`exe::crt_malloc` of 36 × 0x41; out of memory,
/// the error box at `row` and 0); the first twenty rounds (`fra_third_write_league_rounds`); 1 December of the league's
/// year (Date::Date `exe::date_date_549f70`(&d, 1, 11, year, −1), thiscall, ret 0x10) and the Christmas pair chosen by
/// k = (round 19's day − that day + 1, in 16 bits) − 13 (`NORTHERN_IRISH_CHRISTMAS_*`); then the last fourteen rounds
/// `end`. Returns the table.
pub(crate) unsafe fn northern_irish_league_schedule(this: League, rounds: *mut i16, extra: *mut i16, out: *mut i32, end: &[FraThirdLeagueRound], file: usize, row: u32, branch: &'static str) -> usize {
    if !out.is_null() {
        *out = 1;
    }
    *rounds = 0x24;
    *extra = 0;
    let t = exe::crt_malloc(0x924);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    covered!(branch);
    let year = this.year() as u16 as u32;
    fra_third_write_league_rounds(t, year, 0, &NORTHERN_IRISH_LEAGUE_START);
    let mut d = [0u16; 4];
    exe::date_date_549f70(&mut d, 1, 0xb, year, 0xffff_ffff);
    let k = (read_u16(t + 19 * 0x41).wrapping_sub(d[0]).wrapping_add(1) as i16 as i32).wrapping_sub(0xd); // mwa20: the Christmas case
    let christmas: &[FraThirdLeagueRound] = match k {
        0 | 1 => &NORTHERN_IRISH_CHRISTMAS_0,
        2 | 3 => &NORTHERN_IRISH_CHRISTMAS_2,
        4 | 5 => &NORTHERN_IRISH_CHRISTMAS_4,
        _ => &NORTHERN_IRISH_CHRISTMAS_DEFAULT,
    };
    fra_third_write_league_rounds(t, year, 20, christmas);
    fra_third_write_league_rounds(t, year, 22, end);
    t
}

/// The Northern Irish leagues' first twenty matchdays (rounds 0 to 19), from 12 August to 16 December: Saturdays
/// (weekday 5, code 1) and a few midweek rounds (weekday 2, code 2); no TV slots.
pub(crate) const NORTHERN_IRISH_LEAGUE_START: [FraThirdLeagueRound; 20] = [
    (12, 7, 0, 5, 1, &[(-1, -1, -1)]), // mwa20: the first Northern Irish matchday, 12 August
    (19, 7, 0, 5, 1, &[(-1, -1, -1)]), // 1: 19 aug
    (26, 7, 0, 5, 1, &[(-1, -1, -1)]), // 2: 26 aug
    (30, 7, 0, 2, 2, &[(-1, -1, -1)]), // 3: 30 aug
    (9, 8, 0, 5, 1, &[(-1, -1, -1)]), // 4: 9 sep
    (13, 8, 0, 2, 2, &[(-1, -1, -1)]), // 5: 13 sep
    (16, 8, 0, 5, 1, &[(-1, -1, -1)]), // 6: 16 sep
    (23, 8, 0, 5, 1, &[(-1, -1, -1)]), // 7: 23 sep
    (27, 8, 0, 2, 2, &[(-1, -1, -1)]), // 8: 27 sep
    (30, 8, 0, 5, 1, &[(-1, -1, -1)]), // 9: 30 sep
    (14, 9, 0, 5, 1, &[(-1, -1, -1)]), // 10: 14 oct
    (21, 9, 0, 5, 1, &[(-1, -1, -1)]), // 11: 21 oct
    (28, 9, 0, 5, 1, &[(-1, -1, -1)]), // 12: 28 oct
    (4, 10, 0, 5, 1, &[(-1, -1, -1)]), // 13: 4 nov
    (11, 10, 0, 5, 1, &[(-1, -1, -1)]), // 14: 11 nov
    (18, 10, 0, 5, 1, &[(-1, -1, -1)]), // 15: 18 nov
    (25, 10, 0, 5, 1, &[(-1, -1, -1)]), // 16: 25 nov
    (2, 11, 0, 5, 1, &[(-1, -1, -1)]), // 17: 2 dec
    (9, 11, 0, 5, 1, &[(-1, -1, -1)]), // 18: 9 dec
    (16, 11, 0, 5, 1, &[(-1, -1, -1)]), // 19: 16 dec
];

/// The Christmas pair (rounds 20 and 21) by how the 16 December round falls (k = its day − 1 December's + 1 − 13, so k
/// = 0 to 6 for 13 to 19 December after the move to the weekday): k 0 or 1, 23 and 30 December; k 2 or 3, 23 and 27
/// December (midweek); k 4 or 5, 20 (midweek) and 27 December (midweek); any other k, 20 (midweek) and 23 December.
pub(crate) const NORTHERN_IRISH_CHRISTMAS_0: [FraThirdLeagueRound; 2] = [(23, 11, 0, 5, 1, &[(-1, -1, -1)]), (30, 11, 0, 5, 1, &[(-1, -1, -1)])];

pub(crate) const NORTHERN_IRISH_CHRISTMAS_2: [FraThirdLeagueRound; 2] = [(23, 11, 0, 5, 1, &[(-1, -1, -1)]), (27, 11, 0, 2, 2, &[(-1, -1, -1)])];

pub(crate) const NORTHERN_IRISH_CHRISTMAS_4: [FraThirdLeagueRound; 2] = [(20, 11, 0, 2, 2, &[(-1, -1, -1)]), (27, 11, 0, 2, 2, &[(-1, -1, -1)])];

pub(crate) const NORTHERN_IRISH_CHRISTMAS_DEFAULT: [FraThirdLeagueRound; 2] = [(20, 11, 0, 2, 2, &[(-1, -1, -1)]), (23, 11, 0, 5, 1, &[(-1, -1, -1)])];

/// The Northern Irish First Division's last fourteen matchdays: the Premier's dates moved to weekday 6 (the 11 April
/// midweek round the same).
pub(crate) const NORTHERN_IRISH_FIRST_END: [FraThirdLeagueRound; 14] = [
    (6, 0, 1, 6, 1, &[(-1, -1, -1)]), // 22: 6 jan (+1)
    (13, 0, 1, 6, 1, &[(-1, -1, -1)]), // 23: 13 jan (+1)
    (17, 0, 1, 6, 1, &[(-1, -1, -1)]), // 24: 17 jan (+1)
    (27, 0, 1, 6, 1, &[(-1, -1, -1)]), // 25: 27 jan (+1)
    (3, 1, 1, 6, 1, &[(-1, -1, -1)]), // 26: 3 feb (+1)
    (10, 1, 1, 6, 1, &[(-1, -1, -1)]), // 27: 10 feb (+1)
    (24, 1, 1, 6, 1, &[(-1, -1, -1)]), // 28: 24 feb (+1)
    (3, 2, 1, 6, 1, &[(-1, -1, -1)]), // 29: 3 mar (+1)
    (17, 2, 1, 6, 1, &[(-1, -1, -1)]), // 30: 17 mar (+1)
    (31, 2, 1, 6, 1, &[(-1, -1, -1)]), // 31: 31 mar (+1)
    (11, 3, 1, 2, 2, &[(-1, -1, -1)]), // 32: 11 apr (+1)
    (14, 3, 1, 6, 1, &[(-1, -1, -1)]), // 33: 14 apr (+1)
    (21, 3, 1, 6, 1, &[(-1, -1, -1)]), // 34: 21 apr (+1)
    (28, 3, 1, 6, 1, &[(-1, -1, -1)]), // 35: 28 apr (+1)
];

/// The Northern Irish promotion play-off: one round of two clubs over two legs seven days apart, drawn on 29 April and
/// first played on 5 May of the next year (weekday 3); code 0xa0, tie rule 3 [inference: away goals]; category 5; no
/// prize.
pub(crate) const NORTHERN_IRISH_PLAY_OFF: EngCcCupCupRound = cql10r((0x1d, 3, 1, 0), (5, 4, 1, 3, 1, 0), 0xa0, (0, 0, 3), 5, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)); // mwa20: the play-off on 5 May

/// The Northern Irish First Division's constructor: the vtable 0x96db64, discipline rule
/// `discipline_rule::NORTHERN_IRELAND`, one extra stage (the promotion play-off), NirFirst::set_up_rules (0x78bd70) and
/// NirFirst::create_table (0x78bc50), the lines 0x49, 0x50, 0x57, 0x60 and 0x67 of nir_first.cpp.
pub(crate) const NORTHERN_IRISH_FIRST_CONSTRUCTOR: IrishLeagueConstructor = IrishLeagueConstructor { vtable: 0x96db64, discipline: discipline_rule::NORTHERN_IRELAND,
    play_off_stage: true, reset_ten_slots: false, rules: 0x78bd70, table: 0x78bc50, file: 0xa6965c, rows: [0x49, 0x50, 0x57, 0x60, 0x67] };

/// The Northern Irish First Division's rules: four meetings, the tie-breaks 1, 2 and 4, 3 points for a win and 1 for a
/// draw, match category 2, club sides, one promoted directly and one to the promotion play-off, no relegation, 3
/// substitutes on the bench and 3 substitutions, the squad flags 2; the Premier Division above, none below.
pub(crate) const NORTHERN_IRISH_FIRST_RULES: IrishLeagueRules = IrishLeagueRules {
    writes: &[(league::LEGS, 2, 4), (league::TIEBREAKS + 2, 1, 4), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2),
        (league::CLUB_SIDES, 1, 1), (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2),
        (league::PROMOTED_COUNT, 1, 1), (league::PROMOTION_PLAYOFF_COUNT, 1, 1), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 0), // mwa10: one up, one to the play-off
        (league::SUBSTITUTES_NAMED, 1, 3), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SQUAD_FLAGS, 1, 2)],
    upper: Some(KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION),
    lower: None,
};
