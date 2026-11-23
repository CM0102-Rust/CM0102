use super::*;

/// Builds the Portuguese Second Division (the Liga de Honra): discipline rule 23, the league only; with no saved
/// state, the rules, the table, the fixtures, the order, the stats and the predicted positions.
///
/// `PorSecond::PorSecond(year, record)`: the same body as `construct_nor_prm` (vtable 0x96e7a4, discipline 0x17,
/// `PorSecond::set_up_rules` and `PorSecond::create_table`, por_second.cpp lines 0x43, 0x4a, 0x51, 0x5a, 0x61).
/// Original: `PorSecond::PorSecond` (0x7d0950, por_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_por_second(this: usize, year: u32, record: usize) -> usize {
    covered!("mwe_1 portuguese second division");
    construct_division_league(League(this), year, record, &PORTUGUESE_SECOND_DIVISION)
}

/// Comparer for `construct_por_second`.
pub(crate) unsafe extern "thiscall" fn compare_construct_por_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_POR_SECOND);
    compare_division_league_ctor("construct_por_second", League(this), year, record, &PORTUGUESE_SECOND_DIVISION, || o(this, year, record))
}

/// Sets up the rules of the Portuguese Second Division (the Liga de Honra, 18 clubs): two legs, 3 points for a win,
/// ties broken head to head first; the top 3 go up and the bottom 3 go down (to the Second Division B when it is
/// playable); 7 substitutes and 3 substitutions.
///
/// `PorSecond::set_up_rules()`: the writes of `portuguese_second_rules` (with 34 written into `league::ROUND_COUNT`
/// before the round table) and the main-stage round table (virtual `comp_vtable::CREATE_SCHEDULE`) into
/// `league::ROUND_DATES`. Returns 1.
/// Original: `PorSecond::set_up_rules` (0x7d16a0, por_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_second_set_up_rules(this: usize) -> u32 {
    covered!("mwe_4 liga de honra rules");
    eng_prm_apply_league_rules(League(this), &portuguese_second_rules(this))
}

/// Comparer for `por_second_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_por_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POR_SECOND_SET_UP_RULES);
    eng_prm_compare_league_rules("por_second_set_up_rules", League(this), portuguese_second_rules(this), || o(this), || por_second_set_up_rules(this))
}

/// Creates the Liga de Honra table: 18 clubs, those of the division in database order; the B team of a Premier
/// Division club cannot go up (status −4); the prize fund by a share of 50,000 per place; no TV money.
///
/// `PorSecond::create_table()`: the body of `spa_second_create_table` (checked instruction by instruction):
/// `.set_club_count()` = 18, the table (`exe::crt_malloc` of 0x426; out of memory: the box of por_second.cpp
/// (0xa7875c) line 0x16c and 0); in database order (`G_CLUB_TABLE`, a signed word counter up to `G_CLUB_COUNT`), the
/// clubs of `KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION` (`spa_lower_is_in_comp`), each into the next row
/// (`exe::league_init_table_row`: push 0; push 0; push club; push row, a stack dword with the row byte (the routine
/// reads the byte) -> thiscall, ret 0x10); a B team of a Premier Division club (`is_b_team_of_comp_club`:
/// `exe::club_get_linked_b_team`(club, &flag, 1), cdecl, linked and flag 0, the linked club in
/// `KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION`) gets `league_table_row::STATUS` = 0xfc
/// (`table_status::PROMOTION_INELIGIBLE`); until full. Not full: 0. Full: the prize fund
/// (`exe::league_calc_prize_fund`, push 50,000), `.set_prize_mode()` = 1 and 1.
/// Original: `PorSecond::create_table` (0x7d1520, por_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_second_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0x12);
    let t = exe::crt_malloc(0x426);
    this.set_table(t);
    if t == 0 {
        show_error_at(0xa7875c, 0x16c);
        return 0;
    }
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let base = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let c = base + (i as u16 as usize) * club::SIZE;
        if spa_lower_is_in_comp(Club(c), KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION) {
            exe::league_init_table_row(this, k as u32, c, 0, 0);
            if is_b_team_of_comp_club(c, KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION) { // mwe8: a Premier Division B team
                covered!("mwe_8 b team in the liga de honra");
                write_u8(this.table() + (k as i8 as isize as usize).wrapping_mul(league_table_row::SIZE) + league_table_row::STATUS, 0xfc);
            }
            k = k.wrapping_add(1);
            if k as i8 as i16 == this.club_count() {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    if k as i8 as i16 != this.club_count() {
        return 0;
    }
    covered!("mwe_8 liga de honra table");
    let f = exe::league_calc_prize_fund(this, 0xc350); // mwe8: the Liga de Honra prize share
    this.set_prize_fund(f);
    this.set_prize_mode(1);
    1
}

/// The Liga de Honra turnover: archives the season in the statistics, clears the table, the round dates and the
/// stages, moves to the next year and rebuilds the rules, the table, the fixtures and the order; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// `PorSecond::start_new_season()`: `league::SEASON_CLOSED` = 0 and the body of `rebuild_season` (checked instruction
/// by instruction: `exe::league_free_table`(0), the round dates freed, `exe::comp_stats_archive_season_and_reset`(1),
/// the stages 0..=current destroyed by their virtual 0 with 1, the year + 1, the current stage −1, the rules by virtual
/// `comp_vtable::SET_UP_RULES`, `PorSecond::create_table` (0x7d1520), `exe::league_create_fixtures`,
/// `exe::league_sort_table`(0), each with the box of por_second.cpp (0xa7875c) at 0x1e4, 0x1eb, 0x1f1, 0x1f8, and the
/// predicted positions by virtual `comp_vtable::SET_PREDICTED_POSITIONS`); then `set_expectations_and_renew_officials`
/// (League::set_club_season_expectations and OfficialsManager::renew_officials(record), whose eax is returned).
/// After a box: 0.
/// Original: `PorSecond::start_new_season` (0x7d1760, por_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x7d1520, 0xa7875c, [0x1e4, 0x1eb, 0x1f1, 0x1f8], 1) { // mwe8: the Liga de Honra next year
        return 0;
    }
    covered!("mwe_8 liga de honra turnover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `por_second_create_table`: only the original runs (it allocates); the plan (the division's clubs in
/// database order, up to 18, with the B team mark) checked against the rows, the count and the return value, and the
/// prize fund when full.
pub(crate) unsafe extern "thiscall" fn compare_por_second_create_table(this: usize) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POR_SECOND_CREATE_TABLE);
    let base = read_usize(G_CLUB_TABLE);
    let mut plan: Vec<(usize, bool)> = Vec::new();
    for i in 0..(read_usize(G_CLUB_COUNT) as i32).max(0) as usize {
        let c = base + i * club::SIZE;
        if spa_lower_is_in_comp(Club(c), KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION) {
            plan.push((c, is_b_team_of_comp_club(c, KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION)));
            if plan.len() == 0x12 {
                break;
            }
        }
    }
    let r = o(this.addr());
    let t = this.table();
    let mut ok = r == (plan.len() == 0x12) as u32 && this.club_count() == 0x12 && t != 0;
    if ok {
        for (k, &(c, b)) in plan.iter().enumerate() {
            let l = LeagueTableRow(t + k * league_table_row::SIZE);
            ok = ok && read_usize(l.addr()) == c && (!b || l.status() == table_status::PROMOTION_INELIGIBLE);
        }
        if r == 1 {
            ok = ok && this.prize_mode() == 1 && this.prize_fund() == exe::league_calc_prize_fund(this, 0xc350);
        }
    }
    SCOREBOARD.record(ok, || format!("por_second_create_table({:#x}): returned {}, plan {:x?}", this.addr(), r, plan));
    r
}

/// Comparer for `por_second_start_new_season`: only the original runs (it frees and allocates); the year must go up
/// by one (`compare_state_league_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_por_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POR_SECOND_START_NEW_SEASON);
    compare_state_league_new_season("por_second_start_new_season", League(this), || o(this))
}

/// The Liga de Honra schedule: 34 rounds (18 clubs, two legs), the Premier Division dates (22 August to 14 May), with
/// one match of most rounds moved to the Sunday slot (day 6, slot 2) [inference: the televised match]; main stage
/// only.
///
/// `PorSecond::create_schedule(stage, &rounds, &extra, &league)`: at stage −1 (0xff), `*league` = 1 (when given), 34 rounds,
/// extra 0 and the table (`exe::crt_malloc` of 0x8a2 = 34 × 0x41; out of memory: the box of por_second.cpp (0xa7875c) line 0xa8 and 0),
/// filled round by round from `LIGA_DE_HONRA_ROUNDS` (`eng_first_set_league_rounds`: League round date `exe::league_round_set_date`(t,
/// i, day, month, years, weekday, code, year, 0) and the TV slots `exe::league_round_set_match_slot`(t, i, j, a, b,
/// c, 0), the list closed by (−1, −1, −1); both cdecl); the year is the league's (`.year()`, the original pushes the
/// word with garbage above it: the date only reads the word). Any other stage: 0 and nothing written. 
/// Original: `PorSecond::create_schedule` (0x7d0c40, por_second.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn por_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
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
        show_error_at(0xa7875c, 0xa8);
        return 0;
    }
    covered!("mwe_13 liga de honra schedule");
    let year = (League(this).year() as u16) as u32;
    eng_first_set_league_rounds(t, year, 0, &LIGA_DE_HONRA_ROUNDS); // mwe13: the Liga de Honra rounds
    t
}

/// Comparer for `por_second_create_schedule`: side by side (`compare_league_round_table`: the original with the caller's outputs, the copy
/// with its own; the outputs and the written round bytes equal; the copy's table freed).
pub(crate) unsafe extern "thiscall" fn compare_por_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_POR_SECOND_CREATE_SCHEDULE);
    compare_league_round_table("por_second_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| por_second_create_schedule(this, stage, r, e, l))
}

/// The rule writes of the Portuguese Second Division (the Liga de Honra, 18 clubs), in the original's order.
///
/// Two legs, 3 points for a win, 34 rounds written beforehand in `league::ROUND_COUNT` (the round table then writes
/// its own count there), 1 point for a draw, league matches, club sides; tie-breaks head-to-head, goal difference,
/// goals for and code 3; 3 promoted, no play-offs, 3 relegated; the Premier Division above and the Second Division B
/// below when the Portuguese lower leagues are playable (bit 4), otherwise −1; 3 substitutions, the squad flags 2 and 7
/// substitutes named. It does not touch the match rule flags nor the round draw rules.
pub(crate) unsafe fn portuguese_second_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 2), (league::WIN_POINTS, 1, 3), (league::ROUND_COUNT, 2, 0x22), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::CLUB_SIDES, 1, 1)];
    v.extend_from_slice(&[(league::TIEBREAKS, 1, 4), (league::TIEBREAKS + 1, 1, 1), (league::TIEBREAKS + 2, 1, 2), (league::TIEBREAKS + 3, 1, 3)]);
    v.push((league::PROMOTED_COUNT, 1, 3)); // mwe4: the Liga de Honra promotes 3
    v.extend_from_slice(&[(league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 3)]);
    v.extend_from_slice(&[(league::UPPER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION) as u32), (league::LOWER_DIVISION_ID, 4, lower_division_when_playable(this, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B))]);
    v.extend_from_slice(&[(league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SQUAD_FLAGS, 1, 2), (league::SUBSTITUTES_NAMED, 1, 7)]);
    v
}

/// The division below when the nation has its lower leagues playable: the nation of the league's competition
/// (`competition::NATION`) with bit 4 of `.league_selected()` gives the id of global `below`, otherwise −1.
pub(crate) unsafe fn lower_division_when_playable(this: usize, below: usize) -> u32 {
    let this = League(this);
    let nation = Nation(read_usize(this.competition().addr() + competition::NATION));
    if nation.league_selected() & 4 != 0 { read_usize(below) as u32 } else { 0xffff_ffff }
}

/// The Portuguese Second Division (the Liga de Honra): discipline rule 23, rules 0x7d16a0, table 0x7d1520, source
/// por_second.cpp (0xa7875c), the box lines 0x43, 0x4a, 0x51, 0x5a, 0x61.
pub(crate) const PORTUGUESE_SECOND_DIVISION: LeagueDivision = LeagueDivision {
    vtabela: 0x96e7a4,
    discipline: discipline_rule::PORTUGAL, // mwe1: the Liga de Honra discipline rule
    rules: 0x7d16a0,
    table: 0x7d1520,
    file: 0xa7875c,
    rows: [0x43, 0x4a, 0x51, 0x5a, 0x61],
    low: false,
};

/// The Liga de Honra: the 34 rounds, the same dates as the Premier Division (Sundays, weekday 6), most with one match in
/// the Sunday slot (day 6, slot 2) [inference: the televised match]; three Saturday rounds and the last three rounds
/// without a slot.
pub(crate) const LIGA_DE_HONRA_ROUNDS: [EngFirstScheduleRound; 34] = [
    (22, 7, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 0: 22/ago
    (29, 7, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 1: 29/ago
    (12, 8, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 2: 12/set
    (19, 8, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 3: 19/set
    (26, 8, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 4: 26/set
    (2, 9, 0, 5, 1, &[(-1, -1, -1)]), // 5: 2/out
    (17, 9, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 6: 17/out
    (24, 9, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 7: 24/out
    (31, 9, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 8: 31/out
    (7, 10, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 9: 7/nov (mwe13: the Liga de Honra November round)
    (21, 10, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 10: 21/nov
    (28, 10, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 11: 28/nov
    (5, 11, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 12: 5/dez
    (12, 11, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 13: 12/dez
    (19, 11, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 14: 19/dez
    (2, 0, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 15: 2/jan (+1 ano)
    (16, 0, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 16: 16/jan (+1 ano)
    (23, 0, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 17: 23/jan (+1 ano)
    (30, 0, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 18: 30/jan (+1 ano)
    (6, 1, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 19: 6/fev (+1 ano)
    (13, 1, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 20: 13/fev (+1 ano)
    (20, 1, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 21: 20/fev (+1 ano)
    (27, 1, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 22: 27/fev (+1 ano)
    (4, 2, 1, 5, 1, &[(-1, -1, -1)]), // 23: 4/mar (+1 ano)
    (12, 2, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 24: 12/mar (+1 ano)
    (19, 2, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 25: 19/mar (+1 ano)
    (26, 2, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 26: 26/mar (+1 ano)
    (2, 3, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 27: 2/abr (+1 ano)
    (9, 3, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 28: 9/abr (+1 ano)
    (16, 3, 1, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 29: 16/abr (+1 ano)
    (22, 3, 1, 5, 1, &[(-1, -1, -1)]), // 30: 22/abr (+1 ano)
    (30, 3, 1, 6, 1, &[(-1, -1, -1)]), // 31: 30/abr (+1 ano)
    (7, 4, 1, 6, 1, &[(-1, -1, -1)]), // 32: 7/mai (+1 ano)
    (14, 4, 1, 6, 1, &[(-1, -1, -1)]), // 33: 14/mai (+1 ano)
];
