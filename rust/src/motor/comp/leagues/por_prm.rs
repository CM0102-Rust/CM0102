use super::*;

/// The Portuguese promotion and relegation chain: the final positions of the Premier Division and the Liga de Honra are
/// recorded and the marked clubs swapped between them; when the Second Division B is played, its final positions and
/// those of its two other zones are recorded and the Liga de Honra swaps clubs with each of the three zones.
///
/// `PorPrm::process_promotion_relegation_chain(pairs)` (`pairs` is not read): virtual
/// `league_vtable::RECORD_FINAL_POSITIONS` (thiscall, ret 0) of the Premier Division and of the Liga de Honra
/// (`G_COMP_OBJECT_TABLE` by `KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION` and `KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION`);
/// League::promote_and_relegate (`exe::league_promote_and_relegate`: push −1; push −1; push 1; push 1; push lower; push
/// upper; mov ecx, this -> thiscall, ret 0x18) between them. With the Second Division B object
/// (`KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B`): its RECORD_FINAL_POSITIONS and those of its stages 0 and 1 (the
/// other two zones, `league::STAGES`), then promote_and_relegate between the Liga de Honra and the Second Division B,
/// and between the Liga de Honra and each of the two stages; the last call's eax is returned. Without it, the
/// original returns the value of `G_COMP_OBJECT_TABLE` (the register left from the lookup).
/// Original: `PorPrm::process_promotion_relegation_chain` (0x7cfbf0, por_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn por_prm_process_promotion_relegation_chain(this: usize, _pairs: u32) -> u32 {
    let (premier, second) = (comp_by_global(KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION), comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION));
    bra_nat_first_call_virtual_no_args(premier, league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(second, league_vtable::RECORD_FINAL_POSITIONS);
    covered!("mwe_15 portuguese promotion and relegation");
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION), comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION), 1, 1, -1, -1); // mwe15: the Premier Division and the Liga de Honra
    let b = comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B);
    if b == 0 {
        return read_usize(G_COMP_OBJECT_TABLE) as u32;
    }
    covered!("mwe_15 portuguese second division b");
    bra_nat_first_call_virtual_no_args(b, league_vtable::RECORD_FINAL_POSITIONS);
    let zone = |k: usize| read_usize(read_usize(comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B) + league::STAGES) + 4 * k);
    bra_nat_first_call_virtual_no_args(zone(0), league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(zone(1), league_vtable::RECORD_FINAL_POSITIONS);
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION), comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B), 1, 1, -1, -1);
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION), zone(0), 1, 1, -1, -1);
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION), zone(1), 1, 1, -1, -1) as u32
}

/// The Portuguese turnover, led by the Premier Division: the B team checks when the Second Division B is played, the
/// promotion and relegation chain (with 1); with the Second Division B two unnamed steps, otherwise the marked Liga de
/// Honra clubs exchanged with the regional zones (`PorPrm::exchange_second_division_with_zones`); the Premier Division
/// rebuilt for the next year (rules, the 18-club table, fixtures, order); then the Liga de Honra turnover and the Second
/// Division B's (when it is played), the predicted positions, the board expectations and the referee renewal.
///
/// `PorPrm::start_new_season()`: `league::SEASON_CLOSED` = 0; with the Second Division B object
/// (`G_COMP_OBJECT_TABLE` by `KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B`), the step 0x7d0320 [inference: the B team
/// checks]; virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(1); with the Second Division B, the steps
/// 0x7cff70 and 0x7d0210, otherwise 0x7cfd30 (all direct, thiscall, ret 0); then the body of `top_division_turnover`
/// (the table `PorPrm::create_table` 0x7cf800, the boxes of por_prm.cpp (0xa78724) at 0x227, 0x22e, 0x234, 0x23b) with,
/// after the rebuild, the Liga de Honra's start_new_season (no null check) and the Second Division B's (when it exists).
/// Original: `PorPrm::start_new_season` (0x7cf9e0, por_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_prm_start_new_season(this: usize) -> u32 {
    let this = League(this);
    portuguese_top_division_turnover(this, || {
        covered!("mwe_15 portuguese turnover");
        if comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B) != 0 {
            portuguese_step(0x7d0320, this);
        }
        let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
        chain(this.addr(), 1);
        if comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B) != 0 {
            portuguese_step(0x7cff70, this);
            portuguese_step(0x7d0210, this);
        } else {
            portuguese_step(0x7cfd30, this); // mwe15: the Liga de Honra exchanged with the zones
        }
    })
}

/// Comparer for `por_prm_process_promotion_relegation_chain`: only the original runs (it moves the clubs; the same as
/// the Brazilian chain's comparer).
pub(crate) unsafe extern "thiscall" fn compare_por_prm_process_promotion_relegation_chain(this: usize, pairs: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_POR_PRM_PROCESS_PROMOTION_RELEGATION_CHAIN)(this, pairs)
}

/// Comparer for `por_prm_start_new_season`: only the original runs (it frees, allocates and moves clubs); the year must
/// go up by one (`compare_state_league_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_por_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POR_PRM_START_NEW_SEASON);
    compare_state_league_new_season("por_prm_start_new_season", League(this), || o(this))
}

/// Builds the Portuguese Premier Division: discipline rule 23 (the Portuguese one), the league only; with no saved
/// state, the rules, the table, the fixtures, the order, the stats and the predicted positions.
///
/// `PorPrm::PorPrm(year, record)`: the same body as `construct_nor_prm` (vtable 0x96e6f0, discipline 0x17,
/// `PorPrm::set_up_rules` and `PorPrm::create_table`, por_prm.cpp lines 0x45, 0x4c, 0x53, 0x5c, 0x63).
/// Original: `PorPrm::PorPrm` (0x7ce720, por_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_por_prm(this: usize, year: u32, record: usize) -> usize {
    covered!("mwe_1 portuguese premier division");
    construct_division_league(League(this), year, record, &PORTUGUESE_PREMIER_DIVISION)
}

/// Comparer for `construct_por_prm`.
pub(crate) unsafe extern "thiscall" fn compare_construct_por_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_POR_PRM);
    compare_division_league_ctor("construct_por_prm", League(this), year, record, &PORTUGUESE_PREMIER_DIVISION, || o(this, year, record))
}

/// Sets up the rules of the Portuguese Premier Division (18 clubs): two legs, 3 points for a win, ties broken head to
/// head first, then goal difference and goals for; nobody goes up and the bottom 3 go down to the Liga de Honra; 7
/// substitutes and 3 substitutions.
///
/// `PorPrm::set_up_rules()`: the writes of `portuguese_premier_rules` and the main-stage round table (virtual
/// `comp_vtable::CREATE_SCHEDULE`) into `league::ROUND_DATES`. Returns 1.
/// Original: `PorPrm::set_up_rules` (0x7cf930, por_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwe_4 portuguese premier rules");
    eng_prm_apply_league_rules(League(this), &portuguese_premier_rules())
}

/// Comparer for `por_prm_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_por_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POR_PRM_SET_UP_RULES);
    eng_prm_compare_league_rules("por_prm_set_up_rules", League(this), portuguese_premier_rules(), || o(this), || por_prm_set_up_rules(this))
}

/// Creates the Portuguese Premier Division table: 18 clubs, those of the division in database order; the prize fund by
/// a share of 100,000 per place and 1,000,000 of TV money to every club.
///
/// `PorPrm::create_table()`: the body of `eng_prm_fill_league_table` (checked instruction by instruction):
/// `.set_club_count()` = 18, the table (`exe::crt_malloc` of 0x426; out of memory: the box of por_prm.cpp (0xa78724)
/// line 0x1a8 and 0); the clubs of `KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION` in database order
/// (`exe::league_init_table_row`) until full; not full: 0. Full: the prize fund (`exe::league_calc_prize_fund`, push
/// 100,000), `.set_prize_mode()` = 1, the TV money (`exe::league_pay_tv_rights`: push 0; push 1,000,000 -> thiscall,
/// ret 8, to every club) and 1.
/// Original: `PorPrm::create_table` (0x7cf800, por_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_prm_create_table(this: usize) -> u32 {
    covered!("mwe_8 portuguese premier table");
    eng_prm_fill_league_table(League(this), 0x12, 0xa78724, 0x1a8, KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION, 0x186a0, || {
        eng_prm_pay_tv_rights(this, 0xf4240, 0); // mwe8: the Portuguese Premier TV money
    })
}

/// Comparer for `por_prm_create_table`: only the original runs (it allocates and pays), against the plan of
/// `eng_prm_compare_league_table`.
pub(crate) unsafe extern "thiscall" fn compare_por_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POR_PRM_CREATE_TABLE);
    eng_prm_compare_league_table("por_prm_create_table", League(this), 0x12, KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION, 0x186a0, || o(this))
}

/// The Portuguese Premier Division schedule: 34 rounds (18 clubs, two legs), from 22 August to 14 May of the next year, on Sundays, with three matches moved to Friday, Saturday and Monday in most rounds [inference: the televised matches]; no play-offs (main stage only).
///
/// `PorPrm::create_schedule(stage, &rounds, &extra, &league)`: at stage −1 (0xff), `*league` = 1 (when given), 34 rounds,
/// extra 0 and the table (`exe::crt_malloc` of 0x8a2 = 34 × 0x41; out of memory: the box of por_prm.cpp (0xa78724) line 0xa0 and 0),
/// filled round by round from `PORTUGUESE_PREMIER_ROUNDS` (`eng_first_set_league_rounds`: League round date `exe::league_round_set_date`(t,
/// i, day, month, years, weekday, code, year, 0) and the TV slots `exe::league_round_set_match_slot`(t, i, j, a, b,
/// c, 0), the list closed by (−1, −1, −1); both cdecl); the year is the league's (`.year()`, the original pushes the
/// word with garbage above it: the date only reads the word). Any other stage: 0 and nothing written. 
/// Original: `PorPrm::create_schedule` (0x7ce9e0, por_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn por_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
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
        show_error_at(0xa78724, 0xa0);
        return 0;
    }
    covered!("mwe_12 portuguese premier schedule");
    let year = (League(this).year() as u16) as u32;
    eng_first_set_league_rounds(t, year, 0, &PORTUGUESE_PREMIER_ROUNDS); // mwe12: the Portuguese Premier rounds
    t
}

/// Comparer for `por_prm_create_schedule`: side by side (`compare_league_round_table`: the original with the caller's outputs, the copy
/// with its own; the outputs and the written round bytes equal; the copy's table freed).
pub(crate) unsafe extern "thiscall" fn compare_por_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_POR_PRM_CREATE_SCHEDULE);
    compare_league_round_table("por_prm_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| por_prm_create_schedule(this, stage, r, e, l))
}

/// A direct call with no arguments on the league (thiscall, ret 0).
pub(crate) unsafe fn portuguese_step(f: usize, this: League) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
    f(this.addr());
}

/// The rest of the Portuguese turnover after `pre`: the table and round dates freed, the stats archived, the year + 1,
/// the rules, the table, the fixtures and the order (each with its box of por_prm.cpp), the Liga de Honra's and the
/// Second Division B's start_new_season, the predicted positions, the expectations and the referees (see
/// `por_prm_start_new_season`).
pub(crate) unsafe fn portuguese_top_division_turnover(this: League, pre: impl FnOnce()) -> u32 {
    this.set_season_closed(0);
    pre();
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    this.set_year(this.year().wrapping_add(1)); // mwe15: the Premier Division next year
    let file = 0xa78724;
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, 0x227);
        return 0;
    }
    if exe::porprm_create_table(this) == 0 {
        show_error_at(file, 0x22e);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, 0x234);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, 0x23b);
        return 0;
    }
    bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION), comp_vtable::START_NEW_SEASON);
    let b = comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B);
    if b != 0 {
        bra_nat_first_call_virtual_no_args(b, comp_vtable::START_NEW_SEASON);
    }
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// The rule writes of the Portuguese Premier Division (18 clubs), in the original's order.
///
/// Two legs, 3 points for a win and 1 for a draw, league matches, no match rule flags and no round draw rules, club
/// sides; tie-breaks head-to-head (code 4), goal difference, goals for and code 3; nobody goes up, no play-offs, 3
/// relegated; no division above, 3 substitutions, the squad flags 2, the Second Division below and 7 substitutes named.
pub(crate) unsafe fn portuguese_premier_rules() -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 2), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0), (league::ROUND_DRAW_RULES, 2, 0), (league::CLUB_SIDES, 1, 1)];
    v.push((league::TIEBREAKS, 1, 4)); // mwe4: the Portuguese Premier breaks ties head to head first
    v.extend_from_slice(&[(league::TIEBREAKS + 1, 1, 1), (league::TIEBREAKS + 2, 1, 2), (league::TIEBREAKS + 3, 1, 3)]);
    v.extend_from_slice(&[(league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 3)]);
    v.extend_from_slice(&[(league::UPPER_DIVISION_ID, 4, 0xffff_ffff), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SQUAD_FLAGS, 1, 2)]);
    v.extend_from_slice(&[(league::LOWER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION) as u32), (league::SUBSTITUTES_NAMED, 1, 7)]);
    v
}

/// The Portuguese Premier Division: discipline rule 23 (the Portuguese one), rules 0x7cf930, table 0x7cf800, source
/// por_prm.cpp (0xa78724), the box lines 0x45, 0x4c, 0x53, 0x5c, 0x63.
pub(crate) const PORTUGUESE_PREMIER_DIVISION: LeagueDivision = LeagueDivision {
    vtabela: 0x96e6f0,
    discipline: discipline_rule::PORTUGAL, // mwe1: the Portuguese discipline rule
    rules: 0x7cf930,
    table: 0x7cf800,
    file: 0xa78724,
    rows: [0x45, 0x4c, 0x53, 0x5c, 0x63],
    low: false,
};

/// The Portuguese Premier Division: the 34 rounds, from late August to mid-May of the next year, on Sundays (weekday 6)
/// with three TV slots (one match each on Friday, Saturday and Monday: days 4, 5 and 0, slot 2), two Saturday rounds (4
/// March and 22 April) with only the Friday and Saturday slots, the last round without slots.
pub(crate) const PORTUGUESE_PREMIER_ROUNDS: [EngFirstScheduleRound; 34] = [
    (22, 7, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 0: 22/ago
    (29, 7, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 1: 29/ago
    (12, 8, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 2: 12/set
    (19, 8, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 3: 19/set
    (26, 8, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 4: 26/set
    (3, 9, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 5: 3/out
    (17, 9, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 6: 17/out
    (24, 9, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 7: 24/out
    (31, 9, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 8: 31/out
    (7, 10, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 9: 7/nov (mwe12: the Portuguese November round)
    (21, 10, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 10: 21/nov
    (28, 10, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 11: 28/nov
    (5, 11, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 12: 5/dez
    (12, 11, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 13: 12/dez
    (19, 11, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 14: 19/dez
    (2, 0, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 15: 2/jan (+1 ano)
    (16, 0, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 16: 16/jan (+1 ano)
    (23, 0, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 17: 23/jan (+1 ano)
    (30, 0, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 18: 30/jan (+1 ano)
    (6, 1, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 19: 6/fev (+1 ano)
    (13, 1, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 20: 13/fev (+1 ano)
    (20, 1, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 21: 20/fev (+1 ano)
    (27, 1, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 22: 27/fev (+1 ano)
    (4, 2, 1, 5, 1, &[(4, 2, 1), (5, 2, 1), (-1, -1, -1)]), // 23: 4/mar (+1 ano)
    (12, 2, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 24: 12/mar (+1 ano)
    (19, 2, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 25: 19/mar (+1 ano)
    (26, 2, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 26: 26/mar (+1 ano)
    (2, 3, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 27: 2/abr (+1 ano)
    (9, 3, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 28: 9/abr (+1 ano)
    (16, 3, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 29: 16/abr (+1 ano)
    (22, 3, 1, 5, 1, &[(4, 2, 1), (5, 2, 1), (-1, -1, -1)]), // 30: 22/abr (+1 ano)
    (30, 3, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 31: 30/abr (+1 ano)
    (7, 4, 1, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 32: 7/mai (+1 ano)
    (14, 4, 1, 6, 1, &[(-1, -1, -1)]), // 33: 14/mai (+1 ano)
];
