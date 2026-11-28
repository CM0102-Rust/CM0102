use super::*;

/// Series B has the round robin (stage −1) and the play-off (stage 0): when the round robin ends, the stage becomes 0
/// and the play-off is built; past the last stage it is an error (the box)
/// Original: `BraNatSecond::advance_stage` (0x42cdd0, bra_nat_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_nat_second_advance_stage(this: usize) {
    advance_series_stage(League(this), 0x98ab64, 0x23d, 0x42ce60);
}

/// The Series B play-off: the top 4 of the round robin, 1st v 4th and 2nd v 3rd, in a knockout (no seeds) that decides
/// the Series B champion [inference: the promotions to Series A come from it]
/// Original: `BraNatSecond::create_play_off_stage` (0x42ce60, bra_nat_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_nat_second_create_play_off_stage(this: usize) {
    create_play_off_stage(League(this), &[0, 3, 1, 2], 0x98ab64, 0x25f, 0x276, "cqq_7 serie b"); // cqq7: 1º × 4º, 2º × 3º Série B
}

/// The Series B turnover: archives the season in the statistics, clears the table and stages, moves to the next year
/// and rebuilds the rules, the table with the 28 Series B clubs, the fixtures and the order; then the predicted
/// positions, the board expectations and the referee renewal
/// Original: `BraNatSecond::start_new_season` (0x42cc10, bra_nat_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_nat_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed((0) as usize);
    if !rebuild_season(this, 0x42ca70, 0x98ab64, [0x20c, 0x213, 0x219, 0x220], 1) {
        return 0;
    }
    covered!("cqq_8 serie b rollover");
    set_expectations_and_renew_officials(this)
}

/// Series B has 28 clubs, those of the division in database order; no TV money nor prize fund
/// Original: `BraNatSecond::create_table` (0x42ca70, bra_nat_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_nat_second_create_table(this: usize) -> u32 {
    create_division_table(League(this), 0x1c, 0x98ab64, 0x1aa, 0x57, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION, 0, "cql_7 serie b cheia")
}

/// Series B: single round robin; win 3, draw 1; head-to-head and goal difference; promotes to Series A and relegates to
/// Series C; 4 go up (rule word 2: the top places are promotion) and 4 go down; 7 substitutes, 3 substitutions, fixed
/// numbers. Then the schedule
/// Original: `BraNatSecond::set_up_rules` (0x42cb70, bra_nat_second.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn bra_nat_second_set_up_rules(this: usize) -> u32 {
    set_serie_b_rules(this);
    covered!("cqi_7 serie b rules");
    create_main_schedule(League(this));
    1
}

/// The schedule of the Brazilian Série B: stage −1, the round-robin league stage: 27 rounds from 5 Jul to 2 Nov; stage 0, the knockout or final stage: 2 rounds from 3 Nov to 12 Nov; any other stage returns 0.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures and the season turns. For a stage with a table (`brazil_serie_b_schedule_stage`) it writes the outputs (is_league
/// when that pointer is not null, the rounds and the extra), allocates the round table (`exe::crt_malloc`, 0x41 bytes
/// per round; out of memory, the error box and 0) and writes the date (day, month, years after the season year
/// `year()`, weekday and code) and the matches of each round, in the original's order (`create_round_table`, the
/// cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table, or 0 without writing
/// anything for a stage without one.
/// Original: `BraNatSecond::create_schedule` (0x42c140, bra_nat_second.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_nat_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), brazil_serie_b_schedule_stage(stage), rounds, extra, league, 0x98ab64)
}

/// Comparer for `bra_nat_second_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_NAT_SECOND_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_nat_second_create_schedule", stage, brazil_serie_b_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_nat_second_create_schedule(this, stage, r, e, l))
}

/// The Brazilian second division at career creation: the year and the database competition, discipline rule 6
/// (`discipline_rule::BRAZIL_NATIONAL`) and, with no saved state, importance 10, a round robin and the knockout (one
/// stage besides the league), the rules (the first division above and the third below, two relegated [inference:
/// map]), the table (the national second division clubs), the fixtures, the order, the statistics and the predicted
/// positions by club strength (`bra_nat_first_construct_league` with `BRA_NAT_SECOND_LEAGUE`).
/// Original: `BraNatSecond::BraNatSecond` (0x42be40, bra_nat_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bra_nat_second(this: usize, year: u32, record: usize) -> usize {
    bra_nat_first_construct_league(League(this), year, record, &BRA_NAT_SECOND_LEAGUE, discipline_rule::BRAZIL_NATIONAL, "cqu_1 bra_nat_second new") // cqu1: the discipline of the second division
}

/// Comparer for `construct_bra_nat_second`.
pub(crate) unsafe extern "thiscall" fn compare_construct_bra_nat_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_NAT_SECOND);
    bra_nat_first_compare_league_ctor("construct_bra_nat_second", League(this), year, record, &BRA_NAT_SECOND_LEAGUE, discipline_rule::BRAZIL_NATIONAL, || o(this, year, record))
}

/// The stages of the round table of the Brazilian Série B (virtual `comp_vtable::CREATE_SCHEDULE`): stage −1, the round-robin league stage: 27 rounds from 5 Jul to 2 Nov; stage 0, the knockout or final stage: 2 rounds from 3 Nov to 12 Nov; any other stage has no table (None).
pub(crate) fn brazil_serie_b_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 27, extra: 0, extra_check: false, size: 0x6db, row: 0xb0,
            ops: &BRAZIL_SERIE_B_SCHEDULE_LEAGUE_OPS, name: "cqw_4 serie b league" }),
        0 => Some(ScheduleStage { league: 0, rounds: 2, extra: 0, extra_check: false, size: 0xd0, row: 0x159,
            ops: &BRAZIL_SERIE_B_SCHEDULE_STAGE_0_OPS, name: "cqw_4 serie b stage 0" }),
        _ => None,
    }
}

/// The Brazilian Série B, the round-robin stage (−1): 27 rounds (5 Jul to 2 Nov), 1755 bytes, 88 operations.
pub(crate) const BRAZIL_SERIE_B_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 88] = [
    Cqp11D(0, 5, 7, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 8, 7, 0, 2, 2, 0), Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0xc, 7, 0, 6, 1, 0), Cqp11L(2, 0, 5, 1, 1, 0),
    Cqp11L(2, 1, 5, 1, 1, 0), Cqp11L(2, 2, -1, -1, -1, 0), Cqp11D(3, 0xf, 7, 0, 2, 2, 0), Cqp11L(3, 0, -1, -1, -1, 0),
    Cqp11D(4, 0x13, 7, 0, 6, 1, 0), Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0),
    Cqp11D(5, 0x1a, 7, 0, 6, 1, 0), Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0),
    Cqp11D(6, 0x1d, 7, 0, 2, 2, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 2, 8, 0, 6, 1, 0),
    Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0), Cqp11L(7, 2, -1, -1, -1, 0), Cqp11D(8, 5, 8, 0, 2, 2, 0),
    Cqp11L(8, 0, -1, -1, -1, 0), Cqp11D(9, 9, 8, 0, 6, 1, 0), Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0),
    Cqp11L(9, 2, -1, -1, -1, 0), Cqp11D(0xa, 0x10, 8, 0, 6, 1, 0), Cqp11L(0xa, 0, 5, 1, 1, 0),
    Cqp11L(0xa, 1, 5, 1, 1, 0), Cqp11L(0xa, 2, -1, -1, -1, 0), Cqp11D(0xb, 0x13, 8, 0, 2, 2, 0),
    Cqp11L(0xb, 0, -1, -1, -1, 0), Cqp11D(0xc, 0x17, 8, 0, 6, 1, 0), Cqp11L(0xc, 0, 5, 1, 1, 0),
    Cqp11L(0xc, 1, 5, 1, 1, 0), Cqp11L(0xc, 2, -1, -1, -1, 0), Cqp11D(0xd, 0x1e, 8, 0, 6, 1, 0),
    Cqp11L(0xd, 0, 5, 1, 1, 0), Cqp11L(0xd, 1, 5, 1, 1, 0), Cqp11L(0xd, 2, -1, -1, -1, 0),
    Cqp11D(0xe, 3, 9, 0, 2, 2, 0), Cqp11L(0xe, 0, -1, -1, -1, 0), Cqp11D(0xf, 7, 9, 0, 6, 1, 0),
    Cqp11L(0xf, 0, 5, 1, 1, 0), Cqp11L(0xf, 1, 5, 1, 1, 0), Cqp11L(0xf, 2, -1, -1, -1, 0),
    Cqp11D(0x10, 0xa, 9, 0, 2, 2, 0), Cqp11L(0x10, 0, -1, -1, -1, 0), Cqp11D(0x11, 0xe, 9, 0, 6, 1, 0),
    Cqp11L(0x11, 0, 5, 1, 1, 0), Cqp11L(0x11, 1, 5, 1, 1, 0), Cqp11L(0x11, 2, -1, -1, -1, 0),
    Cqp11D(0x12, 0x15, 9, 0, 6, 1, 0), Cqp11L(0x12, 0, 5, 1, 1, 0), Cqp11L(0x12, 1, 5, 1, 1, 0),
    Cqp11L(0x12, 2, -1, -1, -1, 0), Cqp11D(0x13, 0x1c, 9, 0, 6, 1, 0), Cqp11L(0x13, 0, 5, 1, 1, 0),
    Cqp11L(0x13, 1, 5, 1, 1, 0), Cqp11L(0x13, 2, -1, -1, -1, 0), Cqp11D(0x14, 4, 0xa, 0, 6, 1, 0),
    Cqp11L(0x14, 0, 5, 1, 1, 0), Cqp11L(0x14, 1, 5, 1, 1, 0), Cqp11L(0x14, 2, -1, -1, -1, 0),
    Cqp11D(0x15, 7, 0xa, 0, 2, 2, 0), Cqp11L(0x15, 0, -1, -1, -1, 0), Cqp11D(0x16, 0xb, 0xa, 0, 6, 1, 0),
    Cqp11L(0x16, 0, 5, 1, 1, 0), Cqp11L(0x16, 1, 5, 1, 1, 0), Cqp11L(0x16, 2, -1, -1, -1, 0),
    Cqp11D(0x17, 0xe, 0xa, 0, 2, 2, 0), Cqp11L(0x17, 0, -1, -1, -1, 0), Cqp11D(0x18, 0x12, 0xa, 0, 6, 1, 0),
    Cqp11L(0x18, 0, 5, 1, 1, 0), Cqp11L(0x18, 1, 5, 1, 1, 0), Cqp11L(0x18, 2, -1, -1, -1, 0),
    Cqp11D(0x19, 0x19, 0xa, 0, 6, 1, 0), Cqp11L(0x19, 0, 5, 1, 1, 0), Cqp11L(0x19, 1, 5, 1, 1, 0),
    Cqp11L(0x19, 2, -1, -1, -1, 0), Cqp11D(0x1a, 2, 0xb, 0, 6, 1, 0), Cqp11L(0x1a, 0, -1, -1, -1, 0),
];

/// The Brazilian Série B, stage 0: 2 rounds (3 Nov to 12 Nov), 208 bytes, 38 operations.
pub(crate) const BRAZIL_SERIE_B_SCHEDULE_STAGE_0_OPS: [AsiaClubChampScheduleOp; 38] = [
    Cqp11S(0, 3, 0xb, 0, 0), Cqp11J(0, 5, 0xb, 0, 2, 2, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 8),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 5), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 1),
    Cqp11W(0x11, 0), Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 4), Cqp11X(0x5c, 0),
    Cqp11X(0x60, 0), Cqp11X(0x64, 0), Cqp11S(1, 6, 0xb, 0, 3), Cqp11J(1, 0xc, 0xb, 0, 2, 2, 0), Cqp11W(0x6f, 0x96),
    Cqp11W(0x71, 1), Cqp11W(0x73, 8), Cqp11W(0x75, 0x200), Cqp11B(0x7f, 5), Cqp11W(0x80, 2), Cqp11W(0x82, 1),
    Cqp11W(0x84, 0), Cqp11W(0x77, 1), Cqp11W(0x79, 0), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2),
    Cqp11B(0x8a, 4), Cqp11X(0xc4, 0), Cqp11X(0xc8, 0x7a120), Cqp11X(0xcc, 0),
];

pub(crate) const BRA_NAT_SECOND_LEAGUE: BraNatFirstNationalLeague = BraNatFirstNationalLeague {
    vtable: 0x967e4c,
    importance: 0xa,
    importance_before_load: false,
    stages: 1,
    rules: 0x42cb70,
    table: 0x42ca70,
    file: 0x98ab64,
    rows: [0x50, 0x57, 0x5e, 0x67, 0x6e],
};

/// Updates the staff records by a club's outcome in the Brazilian Serie B, on the Staff_Records object
/// (`G_STAFF_RECORDS`) with this competition. In the league (stage 0xff): status 0 promotion (weight 0x64), status 3
/// relegation. In the play-off (stage 0), status 1: the club's row (League::find_club_row `exe::league_find_club_row`,
/// not checked: a club without a row would crash the original) and the champions record, then the row's status 5.
/// Returns 0.
/// Original: `BraNatSecond::update_staff_records_for_status` (0x42d040, bra_nat_second.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn bra_nat_second_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, _a5: u32, _a6: u32) -> u32 {
    let l = League(this);
    let (records, comp) = (read_usize(G_STAFF_RECORDS), l.competition().addr());
    match (stage as u8, status as u8) {
        (0xff, 0) => {
            covered!("cx09_16 serie b promotion");
            exe::staff_records_add_promotion_record(records, club, comp, 0x64);
        }
        (0xff, 3) => exe::staff_records_add_relegation_record(records, club, comp),
        (0, 1) => {
            covered!("cx09_16 serie b champions");
            let row = exe::league_find_club_row(this, club, 0);
            exe::staff_records_add_league_champions_record(records, club, comp);
            LeagueTableRow(row).set_status(5);
        }
        _ => {}
    }
    0
}

/// Stores a club's progress in the Brazilian Serie B (`league_stage_progress`: in the league a best code below 5 becomes
/// 1 and a worst code below 5 becomes 4).
/// Original: `BraNatSecond::set_club_progress` (0x42d260, bra_nat_second.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn bra_nat_second_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) -> usize {
    covered!("cx09_16 serie b progress");
    league_stage_progress(this, club, stage, current, best, worst, 5, 4)
}

/// Comparer for `bra_nat_second_update_staff_records_for_status`: only the original (it adds the records); it must
/// return 0 and, for the play-off champion, leave the row with status 5.
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_second_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(IDX_BRA_NAT_SECOND_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, status, stage, a4, a5, a6);
    let row_ok = (stage as u8, status as u8) != (0, 1) || LeagueTableRow(exe::league_find_club_row(this, club, 0)).status() == 5;
    SCOREBOARD.record(r == 0 && row_ok, || format!("bra_nat_second_update_staff_records_for_status({:#x}, {:#x}, status {}, stage {:#x}): returned {}, row ok {}", this, club, status as u8, stage as u8, r, row_ok));
    r
}

pub(crate) unsafe extern "thiscall" fn compare_bra_nat_second_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(IDX_BRA_NAT_SECOND_SET_CLUB_PROGRESS);
    compare_league_stage_progress("bra_nat_second_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || bra_nat_second_set_club_progress(this, club, stage, current, best, worst))
}

/// The progress codes of the Brazilian leagues that only rewrite them in the league stage: the club's Comp_Stats entry
/// (`bra_reg_minas_gerais_comp_stats_club_entry`, unchecked); at stage 0xff a best code below `limit` becomes 1 and a
/// worst code below `limit` becomes `low_worst` (unsigned bytes); then the three codes (`write_progress_codes`).
/// Returns the entry.
pub(crate) unsafe fn league_stage_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32, limit: u8, low_worst: u8) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, club);
    let (c, mut b, mut w) = (current as u8, best as u8, worst as u8);
    if stage as u8 == 0xff {
        if b < limit {
            b = 1;
        }
        if w < limit {
            w = low_worst;
        }
    }
    write_progress_codes(e, c, b, w);
    e
}

/// The comparer of `league_stage_progress`: side by side on the entry's 3 bytes (`compare_group_championship_progress`),
/// returning the original's value.
pub(crate) unsafe fn compare_league_stage_progress(name: &str, this: usize, club: usize, original: impl FnOnce() -> usize, copy: impl FnOnce() -> usize) -> usize {
    let mut r = 0;
    compare_group_championship_progress(name, this, club, || r = original(), || {
        copy();
    });
    r
}
