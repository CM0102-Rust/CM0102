use super::*;

/// The Swedish turnover, led by the Allsvenskan: the promotion and relegation chain (with 0); with a playable Second
/// Division the unnamed step 0x890b40, otherwise three regional clubs promoted to the Superettan
/// (`swe_prm_promote_regional_clubs_to_first_division` 0x890980); the division tiers limited
/// (League::limit_club_division_tier(1, 3, −3, 1)) and the Second Division sections dealt again
/// (`swe_prm_assign_second_division_sections` 0x890c70); the Allsvenskan rebuilt for the next year; then the
/// Superettan turnover and the Second Division's (when it exists), the predicted positions, the board expectations and
/// the referee renewal.
///
/// `SwePrm::start_new_season()`: `top_division_turnover` with, before, virtual
/// `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(0), 0x890b40 or 0x890980 by the Second Division object
/// (`G_COMP_OBJECT_TABLE` by `KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION`), `exe::league_limit_club_division_tier`(1, 3, −3,
/// 1) and 0x890c70 (all thiscall); the table `SwePrm::create_table` (0x890430) and the boxes of swe_prm.cpp (0xa8b578)
/// at 0x1ba, 0x1c1, 0x1c7, 0x1ce; after, the Superettan's start_new_season (no null check) and the Second Division's
/// (only when it exists).
/// Original: `SwePrm::start_new_season` (0x8905f0, swe_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_prm_start_new_season(this: usize) -> u32 {
    let this = League(this);
    top_division_turnover(this, || {
        covered!("mwe_14 swedish turnover");
        promotion_relegation_chain(this, 0);
        if comp_by_global(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION) != 0 {
            covered!("mwe_14 swedish second division playable");
            call_on_league(0x890b40, this);
        } else {
            call_on_league(0x890980, this); // mwe14: the regional clubs promoted
        }
        exe::league_limit_club_division_tier(this, 1, 3, -3, 1);
        call_on_league(0x890c70, this);
    }, 0x890430, 0xa8b578, [0x1ba, 0x1c1, 0x1c7, 0x1ce], || {
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION), comp_vtable::START_NEW_SEASON);
        start_new_season_of(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION);
    })
}

/// Comparer for `swe_prm_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_swe_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SWE_PRM_START_NEW_SEASON);
    compare_state_league_new_season("swe_prm_start_new_season", League(this), || o(this))
}

/// Sets up the rules of the Swedish Premier Division (Allsvenskan, 14 clubs): two legs, 3 points for a win, tie-breaks
/// goal difference and goals for; nobody goes up, the 12th plays the relegation play-off and the bottom 2 go down to
/// the Superettan; 5 substitutes and 3 substitutions.
///
/// `SwePrm::set_up_rules()`: the writes of `swedish_premier_rules` and the main-stage round table (virtual
/// `comp_vtable::CREATE_SCHEDULE`) into `league::ROUND_DATES`. Returns 1.
/// Original: `SwePrm::set_up_rules` (0x890550, swe_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwe_3 swedish premier rules");
    eng_prm_apply_league_rules(League(this), &swedish_premier_rules())
}

/// Comparer for `swe_prm_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_swe_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SWE_PRM_SET_UP_RULES);
    eng_prm_compare_league_rules("swe_prm_set_up_rules", League(this), swedish_premier_rules(), || o(this), || swe_prm_set_up_rules(this))
}

/// Builds the Swedish Premier Division (Allsvenskan): discipline rule 28 (the Swedish league one), the league only; with
/// no saved state, it first files every Second Division section club under the Second Division (the section kept as
/// the reserve division) and every other Swedish club outside the top three divisions under the lower division; then
/// the rules, the table, the fixtures, the order, the stats and the predicted positions.
///
/// `SwePrm::SwePrm(year, record)`: League::League (`exe::new_league`), the record (`league::COMPETITION`), the vtable
/// 0x970198, the year (word) and the discipline byte 0x1c; the saved state (`exe::league_load_from_temp_file`, push 1)
/// ends it. Otherwise, for each of the first `G_CLUB_COUNT` − 2 × `G_NATION_COUNT` club records (`club::SIZE` bytes
/// from `G_CLUB_TABLE`, the count re-read each turn), the filing of `swedish_club_filing` (the new division as the
/// competition record, `G_CLUB_COMP_TABLE` + id × 0x6b); then stage index 0xff, current stage −1, no extra stages,
/// `SwePrm::set_up_rules` (0x890550) and `SwePrm::create_table` (0x890430) (direct, thiscall, ret 0),
/// `exe::league_create_fixtures`, `exe::league_sort_table`(0), the Comp_Stats into `+8` and
/// `exe::league_set_predicted_positions`; each failing step shows the box of swe_prm.cpp (0xa8b578) at lines 0x82,
/// 0x89, 0x90, 0x98, 0x9f and returns. Returns this.
/// Original: `SwePrm::SwePrm` (0x88fb10, swe_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_swe_prm(this: usize, year: u32, record: usize) -> usize {
    let this = League(this);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), 0x970198);
    this.set_year((year as u16) as i16);
    this.set_discipline_rule(discipline_rule::SWEDEN_LEAGUE);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!("mwe_5 new allsvenskan");
    let mut i = 0i32;
    let mut c = read_usize(G_CLUB_TABLE);
    while i < swedish_filing_count() {
        if let Some((division, section)) = swedish_club_filing(c) {
            if let Some(s) = section {
                write_usize(c + club::RESERVE_DIVISION, s);
            }
            write_usize(c + club::DIVISION, division);
        }
        i += 1;
        c += club::SIZE;
    }
    this.set_stage_index(stage_index::MAIN);
    this.set_current_stage(0xffff_ffff);
    this.set_stage_count(0);
    this.set_stages(0);
    let file = 0xa8b578;
    if swe_prm_call(0x890550, this) == 0 {
        show_error_at(file, 0x82);
        return this.addr();
    }
    if swe_prm_call(0x890430, this) == 0 {
        show_error_at(file, 0x89);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, 0x90);
        return this.addr();
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, 0x98);
        return this.addr();
    }
    let e = exe::crt_operator_new(0x5ce);
    let s = if e != 0 { exe::new_comp_stats(e, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(file, 0x9f);
        return this.addr();
    }
    exe::league_set_predicted_positions(this);
    this.addr()
}

/// Comparer for `construct_swe_prm`: only the original runs (it allocates); the filing is planned beforehand and
/// checked club by club afterwards, with the fields of `compare_division_league_ctor`.
pub(crate) unsafe extern "thiscall" fn compare_construct_swe_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SWE_PRM);
    let loaded = league_will_load_saved_state(record);
    let mut plan: Vec<(usize, usize, usize)> = Vec::new();
    if !loaded {
        let mut c = read_usize(G_CLUB_TABLE);
        for _ in 0..swedish_filing_count().max(0) {
            if let Some((d, s)) = swedish_club_filing(c) {
                plan.push((c, d, s.unwrap_or(read_usize(c + club::RESERVE_DIVISION))));
            }
            c += club::SIZE;
        }
    }
    let p = LeagueDivision { vtabela: 0x970198, discipline: discipline_rule::SWEDEN_LEAGUE, rules: 0x890550, table: 0x890430, file: 0xa8b578, rows: [0x82, 0x89, 0x90, 0x98, 0x9f], low: false };
    let r = compare_division_league_ctor("construct_swe_prm", League(this), year, record, &p, || o(this, year, record));
    let wrong: Vec<&(usize, usize, usize)> = plan.iter().filter(|&&(c, d, s)| read_usize(c + club::DIVISION) != d || read_usize(c + club::RESERVE_DIVISION) != s).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("construct_swe_prm({:#x}) filing: {} clubs planned, wrong (club, division, reserve) = {:x?}", this, plan.len(), &wrong[..wrong.len().min(8)]));
    r
}

/// Creates the Swedish Premier Division table: 14 clubs, those of the division in database order; the prize fund by a
/// share of 50,000 per place; no TV money.
///
/// `SwePrm::create_table()`: the same body (`eng_prm_fill_league_table`) with `KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION`,
/// the box of swe_prm.cpp (0xa8b578) line 0x143 and the share 50,000.
/// Original: `SwePrm::create_table` (0x890430, swe_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_prm_create_table(this: usize) -> u32 {
    covered!("mwe_6 allsvenskan table");
    eng_prm_fill_league_table(League(this), 0xe, 0xa8b578, 0x143, KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION, 0xc350, || ()) // mwe6: the Allsvenskan prize share
}

/// Comparer for `swe_prm_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_swe_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SWE_PRM_CREATE_TABLE);
    eng_prm_compare_league_table("swe_prm_create_table", League(this), 0xe, KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION, 0xc350, || o(this))
}

/// The Allsvenskan schedule: 26 rounds (14 clubs, two legs), from 6 April to 8 November, on Sundays, with one Tuesday round in June; no TV slots and no play-offs (main stage only).
///
/// `SwePrm::create_schedule(stage, &rounds, &extra, &league)`: at stage −1 (0xff), `*league` = 1 (when given), 26 rounds,
/// extra 0 and the table (`exe::crt_malloc` of 0x69a = 26 × 0x41; out of memory: the box of swe_prm.cpp (0xa8b578) line 0xdb and 0),
/// filled round by round from `ALLSVENSKAN_ROUNDS` (`eng_first_set_league_rounds`: League round date `exe::league_round_set_date`(t,
/// i, day, month, years, weekday, code, year, 0) and the TV slots `exe::league_round_set_match_slot`(t, i, j, a, b,
/// c, 0), the list closed by (−1, −1, −1); both cdecl); the year is the league's (`.year()`, the original pushes the
/// word with garbage above it: the date only reads the word). Any other stage: 0 and nothing written. 
/// Original: `SwePrm::create_schedule` (0x88feb0, swe_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn swe_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    if !league.is_null() {
        *league = 1;
    }
    *rounds = 0x1a;
    *extra = 0;
    let t = exe::crt_malloc(0x69a);
    if t == 0 {
        show_error_at(0xa8b578, 0xdb);
        return 0;
    }
    covered!("mwe_12 allsvenskan schedule");
    let year = (League(this).year() as u16) as u32;
    eng_first_set_league_rounds(t, year, 0, &ALLSVENSKAN_ROUNDS); // mwe12: the Allsvenskan rounds
    t
}

/// Comparer for `swe_prm_create_schedule`: side by side (`compare_league_round_table`: the original with the caller's outputs, the copy
/// with its own; the outputs and the written round bytes equal; the copy's table freed).
pub(crate) unsafe extern "thiscall" fn compare_swe_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SWE_PRM_CREATE_SCHEDULE);
    compare_league_round_table("swe_prm_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| swe_prm_create_schedule(this, stage, r, e, l))
}

/// The rule writes of the Swedish Premier Division (Allsvenskan, 14 clubs), in the original's order.
///
/// Two legs, 3 points for a win and 1 for a draw, league matches, no match rule flags and no round draw rules, club
/// sides; tie-breaks goal difference and goals for (the third code 0 ends the list); nobody goes up, no promotion
/// play-off, 1 club to the relegation play-off (the 12th, against the Superettan's 3rd) and 2 relegated; the First
/// Division below; the squad flags 2, 3 substitutions and 5 substitutes named.
pub(crate) unsafe fn swedish_premier_rules() -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 2)];
    v.push((league::WIN_POINTS, 1, 3)); // mwe3: the Allsvenskan gives 3 points for a win
    v.extend_from_slice(&[(league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0), (league::ROUND_DRAW_RULES, 2, 0), (league::CLUB_SIDES, 1, 1)]);
    v.extend_from_slice(&[(league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 0)]);
    v.extend_from_slice(&[(league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 1), (league::RELEGATED_COUNT, 1, 2)]);
    v.extend_from_slice(&[(league::UPPER_DIVISION_ID, 4, 0xffff_ffff), (league::SQUAD_FLAGS, 1, 2), (league::SUBSTITUTIONS_ALLOWED, 1, 3)]);
    v.extend_from_slice(&[(league::LOWER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION) as u32), (league::SUBSTITUTES_NAMED, 1, 5)]);
    v
}

/// Where the Allsvenskan constructor files club `c`: Some((new division, new reserve division)) or None (untouched).
///
/// A club in one of the six Second Division sections keeps the section in `club::RESERVE_DIVISION` and goes into the
/// Second Division (`KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION`); a Swedish club (`club::NATION` with the id of
/// `KEY_NATION_SWEDEN`) outside the Allsvenskan, the Superettan and the Second Division goes into the lower division
/// (`KEY_CLUB_COMP_SWEDISH_LOWER_DIVISION`); a club without division, or without nation, is untouched.
pub(crate) unsafe fn swedish_club_filing(c: usize) -> Option<(usize, Option<usize>)> {
    let c = Club(c);
    let comp = c.division().addr();
    if comp == 0 {
        return None;
    }
    let id = read_usize(comp);
    if SWEDISH_SECOND_DIVISION_SECTIONS.iter().any(|&g| read_usize(g) == id) {
        return Some((comp_record_of(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION), Some(comp))); // mwe5: a section club goes into the Second Division
    }
    let nation = c.nation().addr();
    if nation == 0 || read_usize(nation) != read_usize(KEY_NATION_SWEDEN) {
        return None;
    }
    if [KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION, KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION].iter().any(|&g| read_usize(g) == id) {
        return None;
    }
    Some((comp_record_of(KEY_CLUB_COMP_SWEDISH_LOWER_DIVISION), None))
}

/// The number of club records the filing loop walks: `G_CLUB_COUNT` − 2 × `G_NATION_COUNT` (re-read each turn).
pub(crate) unsafe fn swedish_filing_count() -> i32 {
    (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2))
}

/// A direct thiscall with no arguments on the league (set_up_rules, create_table), returning eax.
pub(crate) unsafe fn swe_prm_call(f: usize, this: League) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(f);
    f(this.addr())
}

/// The six regional sections of the Swedish Second Division (Norrland, East and West Svealand, East, West and South
/// Götaland), in the original's comparison order.
pub(crate) const SWEDISH_SECOND_DIVISION_SECTIONS: [usize; 6] = [KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_NORTH, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_EAST_SVEALAND,
    KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_WEST_SVEALAND, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_EAST_GOTALAND, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_WEST_GOTALAND,
    KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_SOUTH_GOTALAND];

/// The Allsvenskan: the 26 rounds, from early April to early November, on Sundays (weekday 6; 0 = Monday), one midweek
/// round on 2 June (weekday 1, Tuesday) and the last on a Saturday (31 October, weekday 5); no TV slots.
pub(crate) const ALLSVENSKAN_ROUNDS: [EngFirstScheduleRound; 26] = [
    (6, 3, 0, 6, 1, &[(-1, -1, -1)]), // 0: 6/abr
    (13, 3, 0, 6, 1, &[(-1, -1, -1)]), // 1: 13/abr
    (20, 3, 0, 6, 1, &[(-1, -1, -1)]), // 2: 20/abr
    (27, 3, 0, 6, 1, &[(-1, -1, -1)]), // 3: 27/abr
    (4, 4, 0, 6, 1, &[(-1, -1, -1)]), // 4: 4/mai
    (11, 4, 0, 6, 1, &[(-1, -1, -1)]), // 5: 11/mai
    (18, 4, 0, 6, 1, &[(-1, -1, -1)]), // 6: 18/mai
    (2, 5, 0, 1, 2, &[(-1, -1, -1)]), // 7: 2/jun
    (7, 5, 0, 6, 1, &[(-1, -1, -1)]), // 8: 7/jun
    (15, 5, 0, 6, 1, &[(-1, -1, -1)]), // 9: 15/jun
    (22, 5, 0, 6, 1, &[(-1, -1, -1)]), // 10: 22/jun
    (29, 5, 0, 6, 1, &[(-1, -1, -1)]), // 11: 29/jun
    (19, 6, 0, 6, 1, &[(-1, -1, -1)]), // 12: 19/jul (mwe12: the Allsvenskan July round)
    (2, 7, 0, 6, 1, &[(-1, -1, -1)]), // 13: 2/ago
    (9, 7, 0, 6, 1, &[(-1, -1, -1)]), // 14: 9/ago
    (16, 7, 0, 6, 1, &[(-1, -1, -1)]), // 15: 16/ago
    (23, 7, 0, 6, 1, &[(-1, -1, -1)]), // 16: 23/ago
    (30, 7, 0, 6, 1, &[(-1, -1, -1)]), // 17: 30/ago
    (13, 8, 0, 6, 1, &[(-1, -1, -1)]), // 18: 13/set
    (20, 8, 0, 6, 1, &[(-1, -1, -1)]), // 19: 20/set
    (27, 8, 0, 6, 1, &[(-1, -1, -1)]), // 20: 27/set
    (4, 9, 0, 6, 1, &[(-1, -1, -1)]), // 21: 4/out
    (18, 9, 0, 6, 1, &[(-1, -1, -1)]), // 22: 18/out
    (25, 9, 0, 6, 1, &[(-1, -1, -1)]), // 23: 25/out
    (31, 9, 0, 5, 1, &[(-1, -1, -1)]), // 24: 31/out
    (8, 10, 0, 6, 1, &[(-1, -1, -1)]), // 25: 8/nov
];

/// rule: at the Swedish turnover with a playable Second Division, the winners of its promotion play-off go up into it,
/// the clubs relegated from its four sections go down to the lower division, and the twelve clubs it kept for promotion
/// go up
///
/// The Second Division is the object of [`KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION`]. Its stage 6 (the play-off, a cup):
/// every entrant of `cup::TEAM_TABLE` (6-byte entries, `cup::TEAM_COUNT` of them) with the outcome byte (+5) 1 whose
/// club's division is not the Second Division's record goes up (`exe::league_move_club_to_division`(this, club, record,
/// 1)). Then the Second Division and its stages 0 to 4 (`league_group`, the six sections): every row with `league_table_row::STATUS` 3
/// goes down (`exe::league_relegate_club_to_division`(this, club, the [`KEY_CLUB_COMP_SWEDISH_LOWER_DIVISION`] record,
/// 1)). Then the twelve club ids at +0xee (the clubs promoted into the Second Division) each go up to its record.
/// Original: `swe_prm_apply_second_division_moves` (0x890b40, swe_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_prm_apply_second_division_moves(this: usize) {
    let this = League(this);
    let second = League(comp_by_global(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION));
    let record = second.competition().addr();
    let play_off = read_usize(second.stages() + 0x18); // stage 6: the promotion play-off
    for k in 0..read_i16(play_off + cup::TEAM_COUNT).max(0) as usize {
        let e = read_usize(play_off + cup::TEAM_TABLE) + k * 6;
        let c = Club(read_usize(e));
        if read_u8(e + 5) == 1 && c.division().addr() != record {
            covered!("rx13_16 swedish play-off winner up");
            exe::league_move_club_to_division(this, c, record, 1);
        }
    }
    for k in -1i8..5 { // the North and the stages 0 to 4: the six sections (bl from -1 while below 5, 0x890c0d; fixed 26/11/2026)
        let g = league_group(second, k);
        for r in 0..g.club_count().max(0) as usize {
            let row = g.table() + r * 0x3b;
            if read_u8(row + league_table_row::STATUS) == 3 {
                exe::league_relegate_club_to_division(this, read_usize(row), comp_record_of_key(KEY_CLUB_COMP_SWEDISH_LOWER_DIVISION), 1);
            }
        }
    }
    covered!("rx13_16 swedish second division moves");
    for k in 0..0xc {
        let id = read_u32(second.addr() + 0xee + k * 4); // the twelve promoted club ids
        exe::league_move_club_to_division(this, club_at(id as isize), comp_record_of_key(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION), 1);
    }
}

/// Comparer for `swe_prm_apply_second_division_moves`: only the original runs (it moves clubs); afterwards the twelve
/// stored clubs must be in the Second Division.
pub(crate) unsafe extern "thiscall" fn compare_swe_prm_apply_second_division_moves(this: usize) {
    let second = comp_by_global(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION);
    let record = comp_record_of_key(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION);
    // the relegated rows of the six sections, planned before the original moves them (26/11/2026: the copy skipped
    // the sixth section and this comparer, which only looked at the twelve stored clubs, did not see it)
    let mut relegated = Vec::new();
    for k in -1i8..5 {
        let g = league_group(League(second), k);
        for r in 0..g.club_count().max(0) as usize {
            let row = g.table() + r * 0x3b;
            if read_u8(row + league_table_row::STATUS) == 3 {
                relegated.push(read_usize(row));
            }
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SWE_PRM_APPLY_SECOND_DIVISION_MOVES)(this);
    let lower = comp_record_of_key(KEY_CLUB_COMP_SWEDISH_LOWER_DIVISION);
    let not_down: Vec<usize> = relegated.into_iter().filter(|&c| read_usize(c + club::DIVISION) != lower).collect();
    let wrong: Vec<u32> = (0..0xc).map(|k| read_u32(second + 0xee + k * 4))
        .filter(|&id| read_usize(club_at(id as isize).addr() + club::DIVISION) != record).collect();
    SCOREBOARD.record(wrong.is_empty() && not_down.is_empty(), || format!(
        "swe_prm_apply_second_division_moves({:#x}) stored clubs outside the Second Division: {:?}; relegated rows not in the lower division: {:x?}", this, wrong, not_down));
}
