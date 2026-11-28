use super::*;

/// Builds a knockout stage from table rows (the same template in the semi-finals and in the state leagues' knockouts;
/// only the source file, the box lines and the arrangement change).
///
/// The list of the `linhas.len()` teams (malloc `0x944e46` of 4 × n; out of memory: the box `linha_sem_lista` and
/// nothing else) with the clubs of the table's `linhas`, in this order (the ties: 0 × 1, 2 × 3...); virtual `+0x3c`
/// (create_round_table: thiscall(this, stage 0, &rodadas, &extra, null), ret 0x10) builds the round table; the new
/// stage (operator new `0x944cf1` of 0xb2 bytes; Cup_Stage::Cup_Stage `0x522e00`, thiscall with 15 arguments, ret 0x3c:
/// the league, n, the list, the rounds, `+4`, the round table, the year `+0x40`, 0, 1, the virtual's extra, 0x14, 0, 1,
/// 0 and the seeds, one byte per team, which the constructor writes into `+4` of the stage's list: in the original
/// routine, each team's row) goes into the first stage slot ([`+0xc`]).
///
/// Without the stage: the box `linha_sem_fase` and return (without freeing). With it: Cup::set_up_round (`0x51c800`,
/// thiscall(stage, 0), ret 4), the round table and the list are freed (free `0x9452ca`, in that order) and `+0x30` = 0.
/// The original routine's SEH frame only covers the constructor (no destructor).
pub(crate) unsafe fn create_knockout_from_table(this: League, rows: &[usize], seeds: &[u8], file: usize, row_without_list: u32, row_without_stage: u32, ramo: &'static str) {
    let n = rows.len();
    // the seeds: the original writes the bytes on the stack before allocating (same as the rows, but separate constants)
    let mut s = [0u8; 8];
    s[..seeds.len()].copy_from_slice(seeds);
    let list = exe::crt_malloc((n * 4) as u32);
    if list == 0 {
        return show_error_at(file, row_without_list);
    }
    for (k, &l) in rows.iter().enumerate() {
        write_usize(list + 4 * k, bra_reg_bahia_table_row_club(this, l));
    }
    covered!(ramo);
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 0, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    // simplification: the original pushes the rounds, the extra and the year in dwords with garbage in the high 16 bits (the outputs of the
    // virtual are words on the stack; the year goes in cx); the constructor only reads the words.
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), n as u32, list, rodadas as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 1, 0, s.as_ptr())
    } else {
        0
    };
    write_usize(this.stages(), stage);
    let stage = read_usize(this.stages());
    if stage == 0 {
        return show_error_at(file, row_without_stage);
    }
    exe::cup_set_up_round(stage, 0);
    exe::crt_free(t);
    exe::crt_free(list);
    this.set_current_stage((0) as usize);
}

/// Rio Grande do Sul: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed
/// is the seed; the ties follow the seeds [inference: two legs, the decider at the better placed club])
/// Original: `BraRegGaucho::create_knockout_stage` (0x4325b0, bra_reg_gaucho.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_gaucho_create_knockout_stage(this: usize) {
    create_knockout_from_table(League(this), &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98ae50, 0x217, 0x22b, "cqn_9 gaucho"); // cqn9: 1º × 4º, 2º × 3º Gaúcho
}

/// End of a state league: records the honours of the first stage (the table) and calls `exe::bra_nat_first_fill_champions_cup_entrants`, which, once the twelve
/// state leagues are over, puts the champions into the Champions Cup.
///
/// `BraRegGaucho::record_season_honours` (slot 13 of nine state leagues: Baiano, Centro-Oeste, Gaúcho, Goiano, Norte,
/// Nordeste, Pernambucano, Catarinense and Paulista): the history of the first stage and then, as a tail jump, `exe::bra_nat_first_fill_champions_cup_entrants`
/// (`bra_nat_first_fill_champions_cup_entrants`, cdecl with no arguments: when the twelve state leagues are over, it
/// puts the champions into the Champions Cup); returns its eax.
/// Original: `BraRegGaucho::record_season_honours` (0x432970, bra_reg_gaucho.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn bra_reg_gaucho_record_season_honours(this: usize) -> u32 {
    record_first_stage_honours(League(this)); // cqj8: the state championship history
    covered!("cqj_8 state league history");
    exe::bra_nat_first_fill_champions_cup_entrants()
}

/// The Rio Grande do Sul state league at career creation: the year and the database competition, disciplinary rule 7
/// and, with no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules,
/// the table with the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegGaucho::BraRegGaucho` (0x4317c0, bra_reg_gaucho.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_gaucho(this: usize, year: u32, record: usize) -> usize {
    bra_reg_bahia_construct_state_league(League(this), year, record, &GAUCHO_STATE_LEAGUE, 7, true, "cqt_1 gaucho novo") // cqt1: the discipline and expected positions of the Gaúcho
}

/// The Rio Grande do Sul turnover: archives the season in the statistics, clears the previous table and stages, moves
/// to the next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table
/// order; then the predicted positions and the renewal of the state referees
/// Original: `BraRegGaucho::start_new_season` (0x432370, bra_reg_gaucho.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_gaucho_start_new_season(this: usize) -> u32 {
    state_league_new_season(League(this), 0x4321d0, 0x98ae50, [0x1c6, 0x1cd, 0x1d3, 0x1da], 1, "cqp_7 virada do gaucho") // cqp7: the next year of the Gaúcho
}

/// The Rio Grande do Sul next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to
/// the semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box)
/// Original: `BraRegGaucho::advance_stage` (0x432520, bra_reg_gaucho.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_gaucho_advance_stage(this: usize) {
    state_league_advance_stage(League(this), 0x98ae50, 0x1f4, 0x4325b0);
}

/// the Rio Grande do Sul league has 17 clubs: those with it as their reserve division in the database (the field that
/// holds the state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegGaucho::load_region_clubs` (0x4321d0, bra_reg_gaucho.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_gaucho_load_region_clubs(this: usize) -> u32 {
    fill_state_league_table(League(this), 0x11, 0x98ae50, 0x163, KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP, 0, "cql_8 gaucho cheio")
}

/// Gaucho, Northeast Cup and Paulista (the same routine in the three vtables): single round robin; 4 top places (the
/// 4-club knockout); 2 relegated; win 3, draw 1; tie-break by head-to-head, then goal difference; no play-offs in the
/// table; no league above or below (the relegated go to the non-playable pool [inference]); rule word 8 (the top places
/// lead to the next stage; no champion band: the champion comes from the final stage); 7 bench substitutes and 3
/// substitutions; category 2 (league) matches. Then the main stage schedule
/// Original: `BraRegGaucho::set_up_rules` (0x4322d0, bra_reg_gaucho.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn bra_reg_gaucho_set_up_rules(this: usize) -> u32 {
    gaucho_rules(this);
    covered!("cqi_8 gaucho rules");
    create_main_schedule(League(this));
    1
}

/// The predicted positions of the Rio Grande do Sul state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, 3, 3, and the others their place in the order (`state_league_predicted_positions`
/// with `GAUCHO_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x303 (no statistics) and 0x30b (no memory).
/// Original: `BraRegGaucho::set_predicted_positions` (0x432980, bra_reg_gaucho.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_gaucho_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &GAUCHO_PREDICTED)
}

/// Comparer for `bra_reg_gaucho_set_predicted_positions` (`compare_state_league_predicted`, side by side).
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_gaucho_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_GAUCHO_SET_PREDICTED_POSITIONS);
    compare_state_league_predicted("bra_reg_gaucho_set_predicted_positions", this, &GAUCHO_PREDICTED, || o(this));
}

/// The Rio Grande do Sul (Gaúcho) state league schedule (the virtual `comp_vtable::CREATE_SCHEDULE`): the stage from
/// `bra_reg_gaucho_schedule_stage`, run by the schedule machine of cqp_11 (`create_round_table`: the outputs, the table of
/// 0x41 bytes per round and, for each round, its date (day, month, years after the season's, weekday and code) and its
/// fixtures; the rest of the table is left as allocated); a stage without a table returns 0 and writes nothing.
/// Original: `BraRegGaucho::create_schedule` (0x431ac0, bra_reg_gaucho.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_reg_gaucho_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), bra_reg_gaucho_schedule_stage(stage), rounds, extra, league, 0x98ae50)
}

pub(crate) unsafe extern "thiscall" fn compare_bra_reg_gaucho_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_REG_GAUCHO_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_reg_gaucho_create_schedule", stage, bra_reg_gaucho_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_reg_gaucho_create_schedule(this, stage, r, e, l))
}

/// The stages of the Rio Grande do Sul (Gaúcho) round table: the round-robin stage (-1) (a league): 17 rounds, from 11 January to 20 April; stage 0
/// (knockout or final stage): 2 rounds, from 21 April to 10 May.
pub(crate) fn bra_reg_gaucho_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 17, extra: 0, extra_check: false, size: 0x451, row: 0xa8,
            ops: &GAUCHO_LEAGUE_STAGE_OPS, name: "cqw_1 gaucho league" }),
        0 => Some(ScheduleStage { league: 0, rounds: 2, extra: 0, extra_check: false, size: 0xd0, row: 0x122,
            ops: &GAUCHO_STAGE_0_OPS, name: "cqw_1 gaucho stage 0" }),
        _ => None,
    }
}

/// The Rio Grande do Sul state league: the fixed places 1, 2, 3, 3 (the semi-finals).
pub(crate) const GAUCHO_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98ae50, lines: [0x303, 0x30b], places: &[1, 2, 3, 3], label: "cqx_3 rio grande do sul predicted" }; // cqx3: the rio grande do sul places

/// The Rio Grande do Sul (Gaúcho) state league, the round-robin stage: 17 rounds (from 11 January to 20 April), 1105 bytes, 62 operations.
pub(crate) const GAUCHO_LEAGUE_STAGE_OPS: [AsiaClubChampScheduleOp; 62] = [
    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 0x12, 1, 0, 6, 2, 0), Cqp11L(1, 0, 5, 1, 1, 0), Cqp11L(1, 1, 5, 1, 1, 0), Cqp11L(1, 2, -1, -1, -1, 0),
    Cqp11D(2, 0x15, 1, 0, 2, 2, 0), Cqp11L(2, 0, -1, -1, -1, 0), Cqp11D(3, 0x19, 1, 0, 6, 1, 0),
    Cqp11L(3, 0, 5, 1, 1, 0), Cqp11L(3, 1, 5, 1, 1, 0), Cqp11L(3, 2, -1, -1, -1, 0), Cqp11D(4, 4, 2, 0, 6, 1, 0),
    Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0), Cqp11D(5, 0xb, 2, 0, 6, 1, 0),
    Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0), Cqp11D(6, 0x12, 2, 0, 6, 1, 0),
    Cqp11L(6, 0, 5, 1, 1, 0), Cqp11L(6, 1, 5, 1, 1, 0), Cqp11L(6, 2, -1, -1, -1, 0), Cqp11D(7, 0x19, 2, 0, 6, 1, 0),
    Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0), Cqp11L(7, 2, -1, -1, -1, 0), Cqp11D(8, 1, 3, 0, 6, 1, 0),
    Cqp11L(8, 0, 5, 1, 1, 0), Cqp11L(8, 1, 5, 1, 1, 0), Cqp11L(8, 2, -1, -1, -1, 0), Cqp11D(9, 8, 3, 0, 6, 1, 0),
    Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0), Cqp11L(9, 2, -1, -1, -1, 0), Cqp11D(0xa, 0xf, 3, 0, 6, 1, 0),
    Cqp11L(0xa, 0, 5, 1, 1, 0), Cqp11L(0xa, 1, 5, 1, 1, 0), Cqp11L(0xa, 2, -1, -1, -1, 0),
    Cqp11D(0xb, 0x16, 3, 0, 6, 1, 0), Cqp11L(0xb, 0, 5, 1, 1, 0), Cqp11L(0xb, 1, 5, 1, 1, 0),
    Cqp11L(0xb, 2, -1, -1, -1, 0), Cqp11D(0xc, 0x19, 3, 0, 6, 1, 0), Cqp11L(0xc, 0, -1, -1, -1, 0),
    Cqp11D(0xd, 0x1d, 3, 0, 6, 1, 0), Cqp11L(0xd, 0, 5, 1, 1, 0), Cqp11L(0xd, 1, 5, 1, 1, 0),
    Cqp11L(0xd, 2, -1, -1, -1, 0), Cqp11D(0xe, 6, 4, 0, 6, 1, 0), Cqp11L(0xe, 0, 5, 1, 1, 0),
    Cqp11L(0xe, 1, 5, 1, 1, 0), Cqp11L(0xe, 2, -1, -1, -1, 0), Cqp11D(0xf, 0xd, 4, 0, 6, 1, 0),
    Cqp11L(0xf, 0, 5, 1, 1, 0), Cqp11L(0xf, 1, 5, 1, 1, 0), Cqp11L(0xf, 2, -1, -1, -1, 0),
    Cqp11D(0x10, 0x14, 4, 0, 6, 1, 0), Cqp11L(0x10, 0, -1, -1, -1, 0),
];

/// The Rio Grande do Sul (Gaúcho) state league, stage 0: 2 rounds (from 21 April to 10 May), 208 bytes, 36 operations.
pub(crate) const GAUCHO_STAGE_0_OPS: [AsiaClubChampScheduleOp; 36] = [
    Cqp11S(0, 0x15, 4, 0, 6), Cqp11J(0, 0x1b, 4, 0, 6, 1, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 8), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 0x804),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 4, 5, 0, 0), Cqp11J(1, 0xa, 5, 0, 6, 1, 0), Cqp11W(0x71, 1), Cqp11W(0x82, 1),
    Cqp11W(0x6f, 0x96), Cqp11W(0x73, 0), Cqp11W(0x75, 0), Cqp11B(0x7f, 8), Cqp11W(0x80, 2), Cqp11W(0x84, 0),
    Cqp11W(0x77, 0x804), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0x3d090), Cqp11X(0xcc, 0x186a0),
];

/// Stores a club's progress in the Rio Grande do Sul state championship (`league_stage_progress`: in the league a best
/// code below 5 becomes 1 and a worst code below 5 becomes 3).
/// Original: `BraRegGaucho::set_club_progress` (0x432af0, bra_reg_gaucho.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn bra_reg_gaucho_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) -> usize {
    covered!("cx09_17 gaucho progress");
    league_stage_progress(this, club, stage, current, best, worst, 5, 3)
}

pub(crate) unsafe extern "thiscall" fn compare_bra_reg_gaucho_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(IDX_BRA_REG_GAUCHO_SET_CLUB_PROGRESS);
    compare_league_stage_progress("bra_reg_gaucho_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || bra_reg_gaucho_set_club_progress(this, club, stage, current, best, worst))
}
