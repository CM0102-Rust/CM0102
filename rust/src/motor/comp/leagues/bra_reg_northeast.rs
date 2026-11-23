use super::*;

/// Describes one state league.
///
/// It holds the vtable, the stages beyond the league (+0x2c: 1 in the round-robin ones with a knockout, 3 in the ones
/// with two rounds and a final; the vector [+0xc] = malloc of 4 × stages), the rules (thiscall, ret 0), the table
/// (thiscall, ret 0), the predicted positions (thiscall, ret 0), the source file and the error-box lines (rules, table,
/// fixtures, order, statistics).
pub(crate) struct StateLeague {
    pub(crate) vtabela: usize,
    pub(crate) stages: u32,
    pub(crate) rules: usize,
    pub(crate) table: usize,
    pub(crate) predicted: usize,
    pub(crate) file: usize,
    pub(crate) rows: [u32; 5],
}

/// The constructor of a Brazilian state championship (the same 544-byte template in all twelve, checked instruction by
/// instruction: only the vtable, the stages, the three own routines, the source file and the lines change).
///
/// League::League (`exe::new_league`, thiscall, ret 0); +4 = the competition record; the vtable; the year (+0x40, word) = `ano`;
/// the discipline rule (+0x50) = 7. If the saved state loads (League::load_from_temp_file `exe::league_load_from_temp_file`, thiscall(this, 1),
/// ret 4; returns ≠ 0), returns this.
///
/// Otherwise (a new career): +0x51 = 0xf (the importance), +0x44 = 0xff, +0x30 = −1, +0x2c = the stages and their
/// vector (malloc `exe::crt_malloc`, unchecked); then, each with its own box (and returning this) if it returns 0: the rules,
/// the table, the fixtures (League::create_fixtures `create_fixtures`, thiscall, ret 0) and the order (League::sort_table
/// `sort_table`, thiscall(this, 0), ret 4); the stats (operator new `exe::crt_operator_new` of 0x5ce and Comp_Stats::Comp_Stats `exe::new_comp_stats`,
/// thiscall(new, this), ret 4; the original's SEH frame covers only that new) in +8 (without them: the box); finally
/// the predicted positions.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn bra_reg_northeast_construct_state_league(this: League, year: u32, registro: usize, e: &StateLeague, discipline: u8, predicted_yes: bool, ramo: &'static str) -> usize {
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.rules);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.table);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(e.predicted);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, registro);
    write_usize(this.addr(), e.vtabela);
    write_u16(this.addr() + league::YEAR, year as u16);
    this.set_discipline_rule(discipline);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!(ramo);
    this.set_importance(0xf);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    write_u32(this.addr() + league::STAGE_COUNT, e.stages);
    let v = exe::crt_malloc(e.stages * 4);
    this.set_stages(v);
    if regras(this.addr()) == 0 {
        show_error_at(e.file, e.rows[0]);
        return this.addr();
    }
    if tabela(this.addr()) == 0 {
        show_error_at(e.file, e.rows[1]);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(e.file, e.rows[2]);
        return this.addr();
    }
    if exe::league_sort_table(this, (0) as u32 as usize) == 0 {
        show_error_at(e.file, e.rows[3]);
        return this.addr();
    }
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(e.file, e.rows[4]);
        return this.addr();
    }
    if predicted_yes {
        previstas(this.addr());
    }
    this.addr()
}

/// Comparer of the state league constructor: runs only the original and checks return, vtable, year, record, discipline
/// and new-league fields.
pub(crate) unsafe fn bra_reg_northeast_compare_state_league_ctor(name: &str, this: League, year: u32, record: usize, e: &StateLeague, original: impl FnOnce() -> usize) -> usize {
    let loaded = league_will_load_saved_state(record);
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == e.vtabela && built_year_ok(this.year() as u16, year as u16)
        && this.competition().addr() == record && this.discipline_rule() == discipline_rule::BRAZIL_REGIONAL;
    if this.stage_index() == stage_index::MAIN && this.comp_stats() != 0 && !loaded {
        ok = ok && this.stage_count() == e.stages as usize && this.stages() != 0 && this.importance() == 0xf;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}) [year {} arg {}, record {:#x} arg {:#x}, discipline {}, importance {}, stage index {}]: returned {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", name, this.addr(), this.year(), year as u16, this.competition().addr(), record, this.discipline_rule(), this.importance(), this.stage_index() as i32, r, read_usize(this.addr()), this.stage_count(), this.stages(), this.comp_stats()));
    r
}

pub(crate) const NORDESTE_STATE_LEAGUE: StateLeague = StateLeague {
    vtabela: 0x9683ec,
    stages: 1,
    rules: 0x4322d0,
    table: 0x436d90,
    predicted: 0x437290,
    file: 0x98af54,
    rows: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

pub(crate) const PARANA_CHAMPIONSHIP: StateLeague = StateLeague {
    vtabela: 0x9684a0,
    stages: 3,
    rules: 0x437fc0,
    table: 0x437ec0,
    predicted: 0x438590,
    file: 0x98af98,
    rows: [0x4e, 0x55, 0x5b, 0x62, 0x69],
};

pub(crate) const PERNAMBUCANO_STATE_CHAMPIONSHIP: StateLeague = StateLeague {
    vtabela: 0x968554,
    stages: 1,
    rules: 0x439430,
    table: 0x439330,
    predicted: 0x4398d0,
    file: 0x98afd8,
    rows: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

/// The Northeast state league at career creation: the year and the database competition, disciplinary rule 7 and, with
/// no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table
/// with the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegNortheast::BraRegNortheast` (0x4363f0, bra_reg_northeast.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_northeast(this: usize, year: u32, record: usize) -> usize {
    bra_reg_northeast_construct_state_league(League(this), year, record, &NORDESTE_STATE_LEAGUE, 7, true, "cqt_3 nordeste novo") // cqt3: the discipline and expected positions of the Nordeste
}

/// the Northeast league has 16 clubs: those with it as their reserve division in the database (the field that holds the
/// state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegNortheast::create_table` (0x436d90, bra_reg_northeast.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_northeast_create_table(this: usize) -> u32 {
    fill_state_league_table(League(this), 0x10, 0x98af54, 0x159, KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, 0, "cqm_7 nordeste cheio")
}

/// Northeast: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the
/// seed; the ties follow the seeds [inference: two legs, the decider at the better placed club])
/// Original: `BraRegNortheast::create_semi_final_stage` (0x4370d0, bra_reg_northeast.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_northeast_create_semi_final_stage(this: usize) {
    create_knockout_from_table(League(this), &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98af54, 0x20d, 0x221, "cqn_7 nordeste"); // cqn7: 1º × 4º, 2º × 3º Nordeste
}

/// The Northeast turnover: archives the season in the statistics, clears the previous table and stages, moves to the
/// next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order;
/// then the predicted positions and the renewal of the state referees
/// Original: `BraRegNortheast::start_new_season` (0x436e90, bra_reg_northeast.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_northeast_start_new_season(this: usize) -> u32 {
    state_league_new_season(League(this), 0x436d90, 0x98af54, [0x1bc, 0x1c3, 0x1c9, 0x1d0], 1, "cqp_8 virada do nordeste") // cqp8: the next year of the Nordeste
}

/// The Northeast next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the
/// semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box)
/// Original: `BraRegNortheast::advance_stage` (0x437040, bra_reg_northeast.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_northeast_advance_stage(this: usize) {
    state_league_advance_stage(League(this), 0x98af54, 0x1ea, 0x4370d0);
}

/// The predicted positions of the Northeast league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, 3, 3, and the others their place in the order (`state_league_predicted_positions`
/// with `NORTHEAST_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x2f9 (no statistics) and 0x301 (no memory).
/// Original: `BraRegNortheast::set_predicted_positions` (0x437290, bra_reg_northeast.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_northeast_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &NORTHEAST_PREDICTED)
}

/// Comparer for `bra_reg_northeast_set_predicted_positions` (`compare_state_league_predicted`, side by side).
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_northeast_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_NORTHEAST_SET_PREDICTED_POSITIONS);
    compare_state_league_predicted("bra_reg_northeast_set_predicted_positions", this, &NORTHEAST_PREDICTED, || o(this));
}

/// The schedule of the Northeast regional league: stage −1, the round-robin league stage: 15 rounds from 18 Jan to 20 Apr; stage 0, the knockout or final stage: 2 rounds from 21 Apr to 10 May; any other stage returns 0.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures and the season turns. For a stage with a table (`northeast_region_schedule_stage`) it writes the outputs (is_league
/// when that pointer is not null, the rounds and the extra), allocates the round table (`exe::crt_malloc`, 0x41 bytes
/// per round; out of memory, the error box and 0) and writes the date (day, month, years after the season year
/// `year()`, weekday and code) and the matches of each round, in the original's order (`create_round_table`, the
/// cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table, or 0 without writing
/// anything for a stage without one.
/// Original: `BraRegNortheast::create_schedule` (0x4366f0, bra_reg_northeast.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_reg_northeast_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), northeast_region_schedule_stage(stage), rounds, extra, league, 0x98af54)
}

/// Comparer for `bra_reg_northeast_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_northeast_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_REG_NORTHEAST_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_reg_northeast_create_schedule", stage, northeast_region_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_reg_northeast_create_schedule(this, stage, r, e, l))
}

/// The stages of the round table of the Northeast regional league (virtual `comp_vtable::CREATE_SCHEDULE`): stage −1, the round-robin league stage: 15 rounds from 18 Jan to 20 Apr; stage 0, the knockout or final stage: 2 rounds from 21 Apr to 10 May; any other stage has no table (None).
pub(crate) fn northeast_region_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 15, extra: 0, extra_check: false, size: 0x3cf, row: 0xa8,
            ops: &NORTHEAST_REGION_SCHEDULE_LEAGUE_OPS, name: "cqw_2 northeast league" }),
        0 => Some(ScheduleStage { league: 0, rounds: 2, extra: 0, extra_check: false, size: 0xd0, row: 0x118,
            ops: &NORTHEAST_REGION_SCHEDULE_STAGE_0_OPS, name: "cqw_2 northeast stage 0" }),
        _ => None,
    }
}

/// The Northeast league: the fixed places 1, 2, 3, 3 (the semi-finals).
pub(crate) const NORTHEAST_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98af54, lines: [0x2f9, 0x301], places: &[1, 2, 3, 3], label: "cqx_5 northeast predicted" }; // cqx5: the northeast places

/// The Northeast regional league, the round-robin stage (−1): 15 rounds (18 Jan to 20 Apr), 975 bytes, 56 operations.
pub(crate) const NORTHEAST_REGION_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 56] = [
    Cqp11D(0, 0x12, 1, 0, 6, 2, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 0x19, 1, 0, 6, 1, 0), Cqp11L(1, 0, 5, 1, 1, 0), Cqp11L(1, 1, 5, 1, 1, 0), Cqp11L(1, 2, -1, -1, -1, 0),
    Cqp11D(2, 4, 2, 0, 6, 1, 0), Cqp11L(2, 0, 5, 1, 1, 0), Cqp11L(2, 1, 5, 1, 1, 0), Cqp11L(2, 2, -1, -1, -1, 0),
    Cqp11D(3, 0xb, 2, 0, 6, 1, 0), Cqp11L(3, 0, 5, 1, 1, 0), Cqp11L(3, 1, 5, 1, 1, 0), Cqp11L(3, 2, -1, -1, -1, 0),
    Cqp11D(4, 0x12, 2, 0, 6, 1, 0), Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0),
    Cqp11D(5, 0x19, 2, 0, 6, 1, 0), Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0),
    Cqp11D(6, 1, 3, 0, 6, 1, 0), Cqp11L(6, 0, 5, 1, 1, 0), Cqp11L(6, 1, 5, 1, 1, 0), Cqp11L(6, 2, -1, -1, -1, 0),
    Cqp11D(7, 8, 3, 0, 6, 1, 0), Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0), Cqp11L(7, 2, -1, -1, -1, 0),
    Cqp11D(8, 0xf, 3, 0, 6, 1, 0), Cqp11L(8, 0, 5, 1, 1, 0), Cqp11L(8, 1, 5, 1, 1, 0), Cqp11L(8, 2, -1, -1, -1, 0),
    Cqp11D(9, 0x16, 3, 0, 6, 1, 0), Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0), Cqp11L(9, 2, -1, -1, -1, 0),
    Cqp11D(0xa, 0x19, 3, 0, 6, 1, 0), Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 0x1d, 3, 0, 6, 1, 0),
    Cqp11L(0xb, 0, 5, 1, 1, 0), Cqp11L(0xb, 1, 5, 1, 1, 0), Cqp11L(0xb, 2, -1, -1, -1, 0),
    Cqp11D(0xc, 6, 4, 0, 6, 1, 0), Cqp11L(0xc, 0, 5, 1, 1, 0), Cqp11L(0xc, 1, 5, 1, 1, 0),
    Cqp11L(0xc, 2, -1, -1, -1, 0), Cqp11D(0xd, 0xd, 4, 0, 6, 1, 0), Cqp11L(0xd, 0, 5, 1, 1, 0),
    Cqp11L(0xd, 1, 5, 1, 1, 0), Cqp11L(0xd, 2, -1, -1, -1, 0), Cqp11D(0xe, 0x14, 4, 0, 6, 1, 0),
    Cqp11L(0xe, 0, -1, -1, -1, 0),
];

/// The Northeast regional league, stage 0: 2 rounds (21 Apr to 10 May), 208 bytes, 36 operations.
pub(crate) const NORTHEAST_REGION_SCHEDULE_STAGE_0_OPS: [AsiaClubChampScheduleOp; 36] = [
    Cqp11S(0, 0x15, 4, 0, 6), Cqp11J(0, 0x1b, 4, 0, 6, 1, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 8), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 0x804),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 4, 5, 0, 0), Cqp11J(1, 0xa, 5, 0, 6, 1, 0), Cqp11W(0x71, 1), Cqp11W(0x82, 1),
    Cqp11W(0x6f, 0x96), Cqp11W(0x73, 0), Cqp11W(0x75, 0), Cqp11B(0x7f, 8), Cqp11W(0x80, 2), Cqp11W(0x84, 0),
    Cqp11W(0x77, 0x804), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0x249f0), Cqp11X(0xcc, 0x186a0),
];
