use super::*;

/// Rule: the Turkish Second Division: as the First Division, with 3 going up to the top flight and 3 down to the
/// second division category B.
/// Original: `TurSecond::set_up_rules` (0x8ff280, tur_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_set_up_rules(this: usize) -> u32 {
    covered!("mwd_14 turkish second division rules");
    eng_prm_apply_league_rules(League(this), &tur_second_rules())
}

/// Comparer for `tur_second_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_TUR_SECOND_SET_UP_RULES);
    eng_prm_compare_league_rules("tur_second_set_up_rules", League(this), tur_second_rules(), || o(this), || tur_second_set_up_rules(this))
}

/// Rule: the Turkish Second Division has 20 clubs, those of the second division category A
/// (`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A`) in database order; when full, no prize fund (+0x28 = 0, prize mode
/// 2 = nothing) and TV pays 100,000 to each club (League::pay_tv_rights: push 0; push 100000).
/// Original: `TurSecond::create_table` (0x8ff150, tur_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_create_table(this: usize) -> u32 {
    let l = League(this);
    if !fill_league_table_rows(l, 0x14, 0xaa187c, 0x159, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A) {
        return 0;
    }
    covered!("mwd_16 turkish second division table");
    l.set_prize_fund(0);
    l.set_prize_mode(2);
    eng_prm_pay_tv_rights(this, 0x186a0, 0); // mwd16: the Turkish Second Division TV money
    1
}

/// Comparer for `tur_second_create_table`: only the original runs; the planned clubs (`compare_division_table`) and,
/// when full, no prize fund with prize mode 2.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_TUR_SECOND_CREATE_TABLE);
    let r = compare_division_table("tur_second_create_table", League(this), 0x14, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A, || o(this));
    if r == 1 {
        let l = League(this);
        SCOREBOARD.record(l.prize_fund() == 0 && l.prize_mode() == 2, || format!("tur_second_create_table({:#x}) prize fund {} mode {}", this, l.prize_fund(), l.prize_mode()));
    }
    r
}

/// Rule: the Turkish Second Division schedule: 38 rounds on Sundays from 13 August to 27 May (break from 17 December
/// to 21 January); no other stage.
/// Original: `TurSecond::create_schedule` (0x8fe9b0, tur_second.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn tur_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    fixed_league_schedule(League(this), stage, rounds, extra, league, 0xaa187c, &LeagueStageRounds { rounds: &TURKISH_SECOND_DIVISION_ROUNDS, extra: 0, league: 1, row: 0xa2 },
        None, "mwd_17 turkish second division calendar")
}

/// Comparer for `tur_second_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_TUR_SECOND_CREATE_SCHEDULE);
    compare_league_round_table("tur_second_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| tur_second_create_schedule(this, stage, r, e, l))
}

/// Rule: the Turkish Second Division turnover (called by the First Division's): the season reopened, rebuilt for the
/// next year (`rebuild_turkish_league`, lines 0x1bf, 0x1c6, 0x1cc, 0x1d3 of tur_second.cpp), the predicted positions,
/// the board expectations and the officials.
/// Original: `TurSecond::start_new_season` (0x8ff330, tur_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    covered!("mwd_24 turkish second division turnover");
    if !rebuild_turkish_league(this, 0x8ff150, 0xaa187c, [0x1bf, 0x1c6, 0x1cc, 0x1d3]) {
        return 0;
    }
    finish_croatian_turnover(this)
}

/// Comparer for `tur_second_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_TUR_SECOND_START_NEW_SEASON);
    compare_state_league_new_season("tur_second_start_new_season", League(this), || o(this))
}

/// Rule: the Turkish Second Division at career creation, as the First Division (`construct_division_league`).
/// Original: `TurSecond::TurSecond` (0x8fe6f0, tur_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_tur_second(this: usize, year: u32, record: usize) -> usize {
    covered!("mwd_12 turkish second division");
    construct_division_league(League(this), year, record, &TURKISH_SECOND_DIVISION)
}

/// Comparer for `construct_tur_second`.
pub(crate) unsafe extern "thiscall" fn compare_construct_tur_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_TUR_SECOND);
    compare_division_league_ctor("construct_tur_second", League(this), year, record, &TURKISH_SECOND_DIVISION, || o(this, year, record))
}

/// The Turkish Second Division: 3 go up to the top flight (`KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION`), the second
/// division category B (`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B`) below.
pub(crate) unsafe fn tur_second_rules() -> Vec<(usize, u8, u32)> {
    turkish_league_rules(3, read_usize(KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION) as u32, read_usize(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B) as u32) // mwd14: three go up
}

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

/// The Turkish Second Division.
pub(crate) const TURKISH_SECOND_DIVISION: LeagueDivision = LeagueDivision {
    vtabela: 0x9705e0,
    discipline: 0x25,
    rules: 0x8ff280,
    table: 0x8ff150,
    file: 0xaa187c,
    rows: [0x48, 0x4f, 0x56, 0x5f, 0x66],
    low: false,
};
