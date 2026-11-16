// ---------- batch 301: the Croatian leagues (cro_a1.cpp, cro_a2a.cpp, cro_a2b.cpp): the constructors ----------

/// The tie-break pair count of a league that settles a points tie with play-off matches (CroA1, CroA2b, RusPrm): a
/// byte at +0xee, written by `create_playoff_stage`, read by `create_schedule` (stage 0) and saved by `load_extra_data`.
pub(crate) const TIEBREAK_PAIRS: usize = 0xee;

/// The body of the league constructors that settle points ties with play-off matches (thiscall(this, year, record),
/// ret 8; the SEH frame only covers the operator new of the statistics).
///
/// League::League (`exe::new_league`, thiscall, ret 0); the competition record (+4); the vtable; the year (+0x40, word)
/// and the discipline rule (+0x50); the saved state (League::load_from_temp_file `exe::league_load_from_temp_file`:
/// push 1 -> thiscall, ret 4): loaded, returns this. Otherwise the new league: `TIEBREAK_PAIRS` = 0, `season_closed`
/// (+0x4c) = 1, the main stage (+0x44 = 0xff), current stage −1 (+0x30), no extra stages (+0x2c = 0, +0xc = 0); then,
/// each with the error box of its line (`p.rows`) and returning this when it returns 0: the rules (`p.rules`, direct
/// call, thiscall, ret 0), the table (`p.table`), the fixtures (League::create_fixtures `exe::league_create_fixtures`),
/// the order (League::sort_table `exe::league_sort_table`: push 0 -> thiscall, ret 4) and the statistics (operator new
/// `exe::crt_operator_new` of 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats::Comp_Stats `exe::new_comp_stats`: push
/// league -> thiscall, ret 4; 0 when out of memory) into +8; last, the predicted positions (League::set_predicted_positions
/// `exe::league_set_predicted_positions`, thiscall, ret 0). Returns this.
pub(crate) unsafe fn construct_tiebreak_league(this: League, year: u32, record: usize, p: &LeagueDivision, branch: &'static str) -> usize {
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.rules);
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.table);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), p.vtabela);
    this.set_year((year as u16) as i16);
    this.set_discipline_rule(p.discipline); // mwd1: the discipline rule
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!(branch);
    write_u8(this.addr() + TIEBREAK_PAIRS, 0);
    this.set_season_closed(1);
    this.set_stage_index(stage_index::MAIN);
    this.set_current_stage(0xffff_ffff);
    this.set_stage_count(0);
    this.set_stages(0);
    if rules(this.addr()) == 0 {
        show_error_at(p.file, p.rows[0]);
        return this.addr();
    }
    if table(this.addr()) == 0 {
        show_error_at(p.file, p.rows[1]);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(p.file, p.rows[2]);
        return this.addr();
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(p.file, p.rows[3]);
        return this.addr();
    }
    let e = exe::crt_operator_new(0x5ce);
    let s = if e != 0 { exe::new_comp_stats(e, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(p.file, p.rows[4]);
        return this.addr();
    }
    exe::league_set_predicted_positions(this);
    this.addr()
}

/// Comparer for the tie-break league constructors: only the original runs; checks the return, vtable, year, record and
/// discipline rule and, for a new league (not loaded), the stage fields, `TIEBREAK_PAIRS` = 0 and `season_closed` = 1.
pub(crate) unsafe fn compare_tiebreak_league_ctor(name: &str, this: League, year: u32, record: usize, p: &LeagueDivision, original: impl FnOnce() -> usize) -> usize {
    let loaded = league_will_load_saved_state(record);
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == p.vtabela && (this.year() as u16) == year as u16
        && this.competition().addr() == record && this.discipline_rule() == p.discipline;
    if this.stage_index() == stage_index::MAIN && this.comp_stats() != 0 && !loaded {
        ok = ok && this.stage_count() == 0 && this.stages() == 0 && read_u8(this.addr() + TIEBREAK_PAIRS) == 0 && this.season_closed() == 1;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}) [year {} arg {}, record {:#x} arg {:#x}, discipline {}]: returned {:#x}, vtable {:#x}, +0x2c = {}, +0xee = {}, +0x4c = {}, +8 = {:#x}", name, this.addr(), this.year(), year as u16, this.competition().addr(), record, this.discipline_rule(), r, read_usize(this.addr()), this.stage_count(), read_u8(this.addr() + TIEBREAK_PAIRS), this.season_closed(), this.comp_stats()));
    r
}

/// The Croatian First Division (1. HNL): discipline rule 0x21 (the Croatian one), the rules, table and fixtures.
pub(crate) const CROATIAN_FIRST_DIVISION: LeagueDivision = LeagueDivision {
    vtabela: 0x9690c4,
    discipline: 0x21,
    rules: 0x5148e0,
    table: 0x5147b0,
    file: 0x9b53c0,
    rows: [0x50, 0x57, 0x5e, 0x67, 0x6e],
    low: false,
};
/// The Croatian Second Division, north group.
pub(crate) const CROATIAN_SECOND_DIVISION_NORTH: LeagueDivision = LeagueDivision {
    vtabela: 0x969178,
    discipline: 0x21,
    rules: 0x515de0,
    table: 0x515cb0,
    file: 0x9b53f8,
    rows: [0x50, 0x57, 0x5e, 0x67, 0x6e],
    low: false,
};
/// The Croatian Second Division, south group.
pub(crate) const CROATIAN_SECOND_DIVISION_SOUTH: LeagueDivision = LeagueDivision {
    vtabela: 0x96922c,
    discipline: 0x21,
    rules: 0x516f00,
    table: 0x516dd0,
    file: 0x9b5430,
    rows: [0x50, 0x57, 0x5e, 0x67, 0x6e],
    low: false,
};

/// Rule: the Croatian First Division at career creation: discipline rule 0x21 and, with no saved state, no tie-break
/// pairs, the main stage only, the rules, the table, the fixtures, the order, the statistics and the predicted
/// positions (`construct_tiebreak_league`).
/// Original: `CroA1::CroA1` (0x513e10, cro_a1.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_cro_a1(this: usize, year: u32, record: usize) -> usize {
    construct_tiebreak_league(League(this), year, record, &CROATIAN_FIRST_DIVISION, "mwd_1 croatian first division new")
}

/// Rule: the Croatian Second Division (north group) at career creation, as the First Division
/// (`construct_tiebreak_league`).
/// Original: `CroA2a::CroA2a` (0x515310, cro_a2a.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_cro_a2a(this: usize, year: u32, record: usize) -> usize {
    construct_tiebreak_league(League(this), year, record, &CROATIAN_SECOND_DIVISION_NORTH, "mwd_1 croatian second division north new")
}

/// Rule: the Croatian Second Division (south group) at career creation, as the First Division
/// (`construct_tiebreak_league`).
/// Original: `CroA2b::CroA2b` (0x516430, cro_a2b.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_cro_a2b(this: usize, year: u32, record: usize) -> usize {
    construct_tiebreak_league(League(this), year, record, &CROATIAN_SECOND_DIVISION_SOUTH, "mwd_1 croatian second division south new")
}

/// Comparer for `construct_cro_a1`.
pub(crate) unsafe extern "thiscall" fn compare_construct_cro_a1(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_CRO_A1);
    compare_tiebreak_league_ctor("construct_cro_a1", League(this), year, record, &CROATIAN_FIRST_DIVISION, || o(this, year, record))
}

/// Comparer for `construct_cro_a2a`.
pub(crate) unsafe extern "thiscall" fn compare_construct_cro_a2a(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_CRO_A2A);
    compare_tiebreak_league_ctor("construct_cro_a2a", League(this), year, record, &CROATIAN_SECOND_DIVISION_NORTH, || o(this, year, record))
}

/// Comparer for `construct_cro_a2b`.
pub(crate) unsafe extern "thiscall" fn compare_construct_cro_a2b(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_CRO_A2B);
    compare_tiebreak_league_ctor("construct_cro_a2b", League(this), year, record, &CROATIAN_SECOND_DIVISION_SOUTH, || o(this, year, record))
}

// ---------- batch 301: the Croatian leagues (cro_a1.cpp, cro_a2a.cpp, cro_a2b.cpp): the rules ----------

/// Sets the bits `flags` in the rule bits (`league::LEAGUE_FLAGS`, byte +0xd9: `or byte ptr [esi + 0xd9], flags`), then
/// writes the (offset, size, value) rules and builds the main stage round table (`eng_prm_apply_league_rules`: virtual
/// +0x3c(−1, &+0xa9, &+0x3a, 0) into +0xba, skipped in the comparer's rehearsal). Returns 1.
pub(crate) unsafe fn apply_league_rules_with_flags(this: League, flags: u8, written: &[(usize, u8, u32)]) -> u32 {
    write_u8(this.addr() + league::LEAGUE_FLAGS, read_u8(this.addr() + league::LEAGUE_FLAGS) | flags);
    eng_prm_apply_league_rules(this, written)
}

/// The rules of the three Croatian leagues (16 clubs each), given the ids of the league above (+0x1c) and below
/// (+0x20).
///
/// 2 legs (+0x3c), 3 points a win (+0xc2) and 1 a draw (+0xc3), league matches (+0xc4 = 2), +0xab = 0 and +0xdb = 0
/// (words), national clubs (+0x42 = 1), tie-breaks 4, 1, 2 (+0xc5..+0xc7: head to head, goal difference, goals for;
/// +0xc8 = 0 ends), nobody goes up directly nor to the play-offs (+0xbe, +0xbf, +0xc0 = 0), 2 go down (+0xc1), +0x52 =
/// 2, 3 substitutions (+0x4a) of 7 substitutes (+0x49) and points ties settled by tie-break matches (+0xe6 = 1); the
/// rule bit 0x40 is set separately (`apply_league_rules_with_flags`). The list also carries +0xd9 for the comparer.
pub(crate) unsafe fn croatian_league_rules(above: u32, below: u32) -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 4), (0xc6, 1, 1), // mwd2: the Croatian points
        (0xc7, 1, 2), (0xc8, 1, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 2), (0x1c, 4, above), (0x52, 1, 2), (0x4a, 1, 3),
        (0x20, 4, below), (0x49, 1, 7), (0xe6, 4, 1), (league::LEAGUE_FLAGS, 1, 0)]
}

/// The rules of the Croatian First Division: nothing above, the Second Division north group
/// (`KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH`) below.
pub(crate) unsafe fn cro_a1_rules() -> Vec<(usize, u8, u32)> {
    croatian_league_rules(0xffff_ffff, read_usize(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH) as u32)
}

/// The rules of the Croatian Second Division north group: the First Division above, the south group below.
pub(crate) unsafe fn cro_a2a_rules() -> Vec<(usize, u8, u32)> {
    croatian_league_rules(read_usize(KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4) as u32, read_usize(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH) as u32)
}

/// The rules of the Croatian Second Division south group: the north group above (as the original: the south group's
/// upper league is the north group, not the First Division), nothing below.
pub(crate) unsafe fn cro_a2b_rules() -> Vec<(usize, u8, u32)> {
    croatian_league_rules(read_usize(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH) as u32, 0xffff_ffff)
}

/// Rule: the Croatian First Division: 16 clubs, 2 legs, 3 points a win and 1 a draw; tie-breaks head to head, goal
/// difference, goals for; points ties across a band go to tie-break matches; nobody goes up, 2 go down to the Second
/// Division (north group); 7 substitutes, 3 substitutions; rule bit 0x40 (`croatian_league_rules`).
/// Original: `CroA1::set_up_rules` (0x5148e0, cro_a1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a1_set_up_rules(this: usize) -> u32 {
    covered!("mwd_2 croatian first division rules");
    apply_league_rules_with_flags(League(this), 0x40, &cro_a1_rules())
}

/// Rule: the Croatian Second Division north group: as the First Division, with the First Division above and the south
/// group below.
/// Original: `CroA2a::set_up_rules` (0x515de0, cro_a2a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2a_set_up_rules(this: usize) -> u32 {
    covered!("mwd_2 croatian second division north rules");
    apply_league_rules_with_flags(League(this), 0x40, &cro_a2a_rules())
}

/// Rule: the Croatian Second Division south group: as the First Division, with the north group above and nothing
/// below.
/// Original: `CroA2b::set_up_rules` (0x516f00, cro_a2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2b_set_up_rules(this: usize) -> u32 {
    covered!("mwd_2 croatian second division south rules");
    apply_league_rules_with_flags(League(this), 0x40, &cro_a2b_rules())
}

/// Comparer for `cro_a1_set_up_rules` (the rehearsal of `eng_prm_compare_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CRO_A1_SET_UP_RULES);
    eng_prm_compare_league_rules("cro_a1_set_up_rules", League(this), cro_a1_rules(), || o(this), || cro_a1_set_up_rules(this))
}

/// Comparer for `cro_a2a_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2a_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CRO_A2A_SET_UP_RULES);
    eng_prm_compare_league_rules("cro_a2a_set_up_rules", League(this), cro_a2a_rules(), || o(this), || cro_a2a_set_up_rules(this))
}

/// Comparer for `cro_a2b_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2b_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CRO_A2B_SET_UP_RULES);
    eng_prm_compare_league_rules("cro_a2b_set_up_rules", League(this), cro_a2b_rules(), || o(this), || cro_a2b_set_up_rules(this))
}

// ---------- batch 301: the Croatian leagues (cro_a1.cpp, cro_a2a.cpp, cro_a2b.cpp): the tables ----------

/// Rule: the Croatian First Division has 16 clubs, those whose national division is it, in database order; when full,
/// the prize fund (30,000 a share) and TV pays 100,000 to each club.
///
/// `club_count` = 16; the table (`exe::crt_malloc` of 16 × 0x3b; out of memory, the box at line 0x145 and 0); the
/// clubs of the competition of id `KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4` until full
/// (`eng_prm_fill_league_table`); full: the prize fund (League::calc_prize_fund, push 30000 -> thiscall, ret 4),
/// `prize_mode` = 1 and the TV money (League::pay_tv_rights `exe::league_pay_tv_rights`: push 0; push 100000 ->
/// thiscall, ret 8) and 1; otherwise 0.
/// Original: `CroA1::create_table` (0x5147b0, cro_a1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a1_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x10, 0x9b53c0, 0x145, KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4, 0x7530, || {
        covered!("mwd_3 croatian first division tv money");
        eng_prm_pay_tv_rights(this, 0x186a0, 0) // mwd3: the First Division TV money
    })
}

/// Rule: the Croatian Second Division north group has 16 clubs (its own, in database order); when full, the prize fund
/// (10,000 a share) and TV pays 20,000 to each club. As `cro_a1_create_table`, with the competition
/// `KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH`.
/// Original: `CroA2a::create_table` (0x515cb0, cro_a2a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2a_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x10, 0x9b53f8, 0x145, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH, 0x2710, || {
        covered!("mwd_3 croatian second division north tv money");
        eng_prm_pay_tv_rights(this, 0x4e20, 0)
    })
}

/// Rule: the Croatian Second Division south group, as the north group (16 clubs, 10,000 a share, 20,000 TV money),
/// with the competition `KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH`.
/// Original: `CroA2b::create_table` (0x516dd0, cro_a2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2b_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x10, 0x9b5430, 0x145, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH, 0x2710, || {
        covered!("mwd_3 croatian second division south tv money");
        eng_prm_pay_tv_rights(this, 0x4e20, 0)
    })
}

/// Comparer for `cro_a1_create_table` (only the original runs; the planned clubs and the prize fund checked).
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CRO_A1_CREATE_TABLE);
    eng_prm_compare_league_table("cro_a1_create_table", League(this), 0x10, KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4, 0x7530, || o(this))
}

/// Comparer for `cro_a2a_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2a_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CRO_A2A_CREATE_TABLE);
    eng_prm_compare_league_table("cro_a2a_create_table", League(this), 0x10, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH, 0x2710, || o(this))
}

/// Comparer for `cro_a2b_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2b_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CRO_A2B_CREATE_TABLE);
    eng_prm_compare_league_table("cro_a2b_create_table", League(this), 0x10, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH, 0x2710, || o(this))
}

// ---------- batch 301: the Croatian leagues (cro_a1.cpp, cro_a2a.cpp, cro_a2b.cpp): the schedules ----------

/// The 30 rounds of the three Croatian leagues (16 clubs, 2 legs), from late July to early May: mostly on Saturdays
/// (weekday 5, code 1), four midweek rounds (weekday 1, code 2: 7 September, 12 October, 29 February of the next year
/// [as the original: 29 February even when that year is not a leap year]); no TV slots; the winter break from
/// mid-December to mid-February.
pub(crate) const CROATIAN_LEAGUE_ROUNDS: [SpaSecondScheduleRound; 30] = [
    (24, 6, 0, 5, 1, &[(-1, -1, -1)]), // 0: 24/jul
    (31, 6, 0, 5, 1, &[(-1, -1, -1)]), // 1: 31/jul
    (28, 7, 0, 5, 1, &[(-1, -1, -1)]), // 2: 28/ago
    (7, 8, 0, 1, 2, &[(-1, -1, -1)]), // 3: 7/set
    (11, 8, 0, 5, 1, &[(-1, -1, -1)]), // 4: 11/set
    (18, 8, 0, 5, 1, &[(-1, -1, -1)]), // 5: 18/set
    (25, 8, 0, 5, 1, &[(-1, -1, -1)]), // 6: 25/set
    (2, 9, 0, 5, 1, &[(-1, -1, -1)]), // 7: 2/out
    (12, 9, 0, 1, 2, &[(-1, -1, -1)]), // 8: 12/out
    (16, 9, 0, 5, 1, &[(-1, -1, -1)]), // 9: 16/out
    (23, 9, 0, 5, 1, &[(-1, -1, -1)]), // 10: 23/out
    (30, 9, 0, 5, 1, &[(-1, -1, -1)]), // 11: 30/out
    (6, 10, 0, 5, 1, &[(-1, -1, -1)]), // 12: 6/nov
    (20, 10, 0, 5, 1, &[(-1, -1, -1)]), // 13: 20/nov
    (27, 10, 0, 5, 1, &[(-1, -1, -1)]), // 14: 27/nov
    (4, 11, 0, 5, 1, &[(-1, -1, -1)]), // 15: 4/dez
    (11, 11, 0, 5, 1, &[(-1, -1, -1)]), // 16: 11/dez
    (19, 1, 1, 5, 1, &[(-1, -1, -1)]), // 17: 19/fev (+1 ano)
    (29, 1, 1, 1, 2, &[(-1, -1, -1)]), // 18: 29/fev (+1 ano)
    (26, 1, 1, 5, 1, &[(-1, -1, -1)]), // 19: 26/fev (+1 ano)
    (4, 2, 1, 5, 1, &[(-1, -1, -1)]), // 20: 4/mar (+1 ano)
    (11, 2, 1, 5, 1, &[(-1, -1, -1)]), // 21: 11/mar (+1 ano)
    (18, 2, 1, 5, 1, &[(-1, -1, -1)]), // 22: 18/mar (+1 ano)
    (25, 2, 1, 5, 1, &[(-1, -1, -1)]), // 23: 25/mar (+1 ano)
    (1, 3, 1, 5, 1, &[(-1, -1, -1)]), // 24: 1/abr (+1 ano)
    (8, 3, 1, 5, 1, &[(-1, -1, -1)]), // 25: 8/abr (+1 ano)
    (15, 3, 1, 5, 1, &[(-1, -1, -1)]), // 26: 15/abr (+1 ano)
    (22, 3, 1, 5, 1, &[(-1, -1, -1)]), // 27: 22/abr (+1 ano)
    (29, 3, 1, 5, 1, &[(-1, -1, -1)]), // 28: 29/abr (+1 ano)
    (6, 4, 1, 5, 1, &[(-1, -1, -1)]), // 29: 6/mai (+1 ano)
];

/// The stage 0 round of the Croatian tie-break play-off (one round of single matches), its team counts from the
/// tie-break pair count: drawn on 7 May of the next year, played on 13 May (weekday 5, leg 1), code 0xa0, no draw news
/// (+0xd = 0x200), two-leg tie-break 3, category 5, `2 × pairs` teams, `pairs` ties and `2 × pairs` new teams (the
/// signed byte widened to a word), +0x20..+0x22 = 0, 2, 4, no prizes.
pub(crate) unsafe fn croatian_tiebreak_round(pairs: u8) -> FraSuperCupRound {
    let p = pairs as i8 as i16 as u16;
    cql12r((7, 4, 1, 0), (0xd, 4, 1, 5, 1, 0), 0xa0, (0, 0x200, 3), 5, (p.wrapping_shl(1), p, p.wrapping_shl(1), 0), (0, 2, 4), (0, 0, 0))
}

/// The body of the three Croatian `create_schedule` (thiscall(this, stage, &rounds, &extra, &league), ret 0x10).
///
/// Stage −1 (the byte 0xff): the league output (if given) = 1, 30 rounds, extra 0; the table (`exe::crt_malloc` of 30 ×
/// 0x41; out of memory: the box at line 0xaa and 0) with `CROATIAN_LEAGUE_ROUNDS` (League::set_round_date and
/// set_round_match_slot, cdecl, through `spa_second_set_league_rounds`; the year pushed as the word +0x40). Stage 0: the
/// league output = 0, 1 round, extra 0; the round (`exe::crt_malloc` of 0x68; out of memory: the box at line 0x118 and
/// 0) with `croatian_tiebreak_round` of the pair count `TIEBREAK_PAIRS` (Cup::set_round_draw_date 0x521e60 and
/// set_round_match_date 0x521eb0, cdecl, the year sign-extended; `fra_super_write_cup_round`). Other stages: 0.
pub(crate) unsafe fn croatian_league_schedule(this: League, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32, file: usize, branch: &'static str) -> usize {
    match stage as u8 {
        0xff => {
            if !league.is_null() {
                *league = 1;
            }
            *rounds = 30;
            *extra = 0;
            let t = exe::crt_malloc(0x79e);
            if t == 0 {
                show_error_at(file, 0xaa);
                return 0;
            }
            covered!(branch);
            spa_second_set_league_rounds(t, (this.year() as u16) as u32, 0, &CROATIAN_LEAGUE_ROUNDS);
            t
        }
        0 => {
            if !league.is_null() {
                *league = 0;
            }
            *rounds = 1;
            *extra = 0;
            let t = exe::crt_malloc(0x68);
            if t == 0 {
                show_error_at(file, 0x118);
                return 0;
            }
            covered!("mwd_4 croatian tie-break round");
            fra_super_write_cup_round(t, 0, &croatian_tiebreak_round(read_u8(this.addr() + TIEBREAK_PAIRS)), this.year() as i32 as u32);
            t
        }
        _ => 0,
    }
}

/// Rule: the Croatian First Division schedule: 30 rounds from late July to early May (`CROATIAN_LEAGUE_ROUNDS`); stage
/// 0 is the one-round tie-break play-off in May (`croatian_tiebreak_round`).
/// Original: `CroA1::create_schedule` (0x5140e0, cro_a1.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn cro_a1_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    croatian_league_schedule(League(this), stage, rounds, extra, league, 0x9b53c0, "mwd_4 croatian first division calendar")
}

/// Rule: the Croatian Second Division north group schedule: the same 30 rounds and tie-break round as the First
/// Division.
/// Original: `CroA2a::create_schedule` (0x5155e0, cro_a2a.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn cro_a2a_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    croatian_league_schedule(League(this), stage, rounds, extra, league, 0x9b53f8, "mwd_4 croatian second division north calendar")
}

/// Rule: the Croatian Second Division south group schedule: the same 30 rounds and tie-break round as the First
/// Division.
/// Original: `CroA2b::create_schedule` (0x516700, cro_a2b.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn cro_a2b_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    croatian_league_schedule(League(this), stage, rounds, extra, league, 0x9b5430, "mwd_4 croatian second division south calendar")
}

/// Comparer for `cro_a1_create_schedule` (side by side: the outputs and the written table).
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_CRO_A1_CREATE_SCHEDULE);
    compare_league_round_table("cro_a1_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| cro_a1_create_schedule(this, stage, r, e, l))
}

/// Comparer for `cro_a2a_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2a_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_CRO_A2A_CREATE_SCHEDULE);
    compare_league_round_table("cro_a2a_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| cro_a2a_create_schedule(this, stage, r, e, l))
}

/// Comparer for `cro_a2b_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2b_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_CRO_A2B_CREATE_SCHEDULE);
    compare_league_round_table("cro_a2b_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| cro_a2b_create_schedule(this, stage, r, e, l))
}

// ---------- batch 301: the Croatian leagues (cro_a1.cpp, cro_a2a.cpp, cro_a2b.cpp): the season turnover ----------

/// The rebuild common to the three Croatian turnovers (after the First Division's promotion and relegation), in the
/// original's order; returns false after an error box.
///
/// League::free_table (`exe::league_free_table`: push 0 -> thiscall, ret 4); the round dates (+0xba) freed
/// (`exe::crt_free`, cdecl) and cleared; the statistics archived (Comp_Stats::archive_season_and_reset
/// `exe::comp_stats_archive_season_and_reset`: push 1 -> thiscall, ret 4) when there are any; the stage vector (+0xc)
/// freed when there is one (the stage objects are not destroyed); the year (+0x40) + 1; +0xc = 0, +0x2c = 0, current
/// stage −1, `season_closed` = 1 when `reopen` (the Croatian and Russian Premier turnovers; not the Greek ones); then, each with the box of its line (`rows`) and stopping when it returns 0: the
/// rules (virtual `comp_vtable::SET_UP_RULES`), the table (`table`, direct call, thiscall, ret 0), the fixtures
/// (League::create_fixtures) and the order (League::sort_table: push 0).
pub(crate) unsafe fn rebuild_croatian_league(this: League, table: usize, file: usize, rows: [u32; 4], reopen: bool) -> bool {
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    if this.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(this.comp_stats(), 1);
    }
    if this.stages() != 0 {
        exe::crt_free(this.stages());
    }
    this.set_year(this.year().wrapping_add(1)); // mwd5: the next year
    this.set_stages(0);
    this.set_stage_count(0);
    this.set_current_stage(0xffff_ffff);
    if reopen {
        this.set_season_closed(1);
    }
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, rows[0]);
        return false;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(table);
    if table(this.addr()) == 0 {
        show_error_at(file, rows[1]);
        return false;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, rows[2]);
        return false;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, rows[3]);
        return false;
    }
    true
}

/// The end common to the Croatian turnovers: the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`),
/// the board expectations of each club (League::set_club_season_expectations, thiscall, ret 0) and the renewal of the
/// officials (OfficialsManager::renew_officials(+4): push; mov ecx, [0xb74340] -> thiscall, ret 4), whose eax is
/// returned.
pub(crate) unsafe fn finish_croatian_turnover(this: League) -> u32 {
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// Rule: the Croatian turnover, led by the First Division: the season is reopened (+0x4c = 0); promotion and relegation
/// along the chain First Division -> north group -> south group (virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`:
/// push 1 -> thiscall, ret 4); the four best third-tier clubs up into the second division and its relegated clubs down
/// (`cro_a1_swap_second_and_third_tier_clubs`, ecx = this); the 32 second division clubs regrouped north and south by
/// the latitude of their city (0x515190, cdecl); the First Division rebuilt (`rebuild_croatian_league`, lines 0x1c0,
/// 0x1c7, 0x1cd, 0x1d4); the division tier of the clubs of both second division groups capped (League::limit_club_division_tier
/// `exe::league_limit_club_division_tier`(group, 1, 3, −3, 1): 4 pushes -> thiscall, ret 0x10); both groups turned over
/// (virtual `comp_vtable::START_NEW_SEASON`); then the predicted positions, the board expectations and the officials
/// (`finish_croatian_turnover`).
/// Original: `CroA1::start_new_season` (0x5149a0, cro_a1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a1_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    chain(this.addr(), 1);
    let swap: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x514ed0usize);
    swap(this.addr()); // mwd5: third tier up, second tier down
    let realign: unsafe extern "C" fn() = core::mem::transmute(0x515190usize);
    realign();
    covered!("mwd_5 croatian first division turnover");
    if !rebuild_croatian_league(this, 0x5147b0, 0x9b53c0, [0x1c0, 0x1c7, 0x1cd, 0x1d4], true) {
        return 0;
    }
    let (north, south) = (comp_by_global(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH), comp_by_global(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH));
    exe::league_limit_club_division_tier(north, 1, 3, -3, 1);
    exe::league_limit_club_division_tier(south, 1, 3, -3, 1);
    bra_nat_first_call_virtual_no_args(north, comp_vtable::START_NEW_SEASON); // mwd5: the north group turnover
    bra_nat_first_call_virtual_no_args(south, comp_vtable::START_NEW_SEASON);
    finish_croatian_turnover(this)
}

/// Rule: the Croatian Second Division north group turnover (called by the First Division's): reopened, rebuilt for the
/// next year (`rebuild_croatian_league`, lines 0x1b7, 0x1be, 0x1c4, 0x1cb), the predicted positions, the board
/// expectations and the officials.
/// Original: `CroA2a::start_new_season` (0x515ea0, cro_a2a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2a_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    covered!("mwd_5 croatian second division north turnover");
    if !rebuild_croatian_league(this, 0x515cb0, 0x9b53f8, [0x1b7, 0x1be, 0x1c4, 0x1cb], true) {
        return 0;
    }
    finish_croatian_turnover(this)
}

/// Rule: the Croatian Second Division south group turnover, as the north group's (lines 0x1b7, 0x1be, 0x1c4, 0x1cb of
/// cro_a2b.cpp).
/// Original: `CroA2b::start_new_season` (0x516fc0, cro_a2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2b_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    covered!("mwd_5 croatian second division south turnover");
    if !rebuild_croatian_league(this, 0x516dd0, 0x9b5430, [0x1b7, 0x1be, 0x1c4, 0x1cb], true) {
        return 0;
    }
    finish_croatian_turnover(this)
}

/// Comparer for `cro_a1_start_new_season` (only the original; the year went up by one).
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CRO_A1_START_NEW_SEASON);
    compare_state_league_new_season("cro_a1_start_new_season", League(this), || o(this))
}

/// Comparer for `cro_a2a_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2a_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CRO_A2A_START_NEW_SEASON);
    compare_state_league_new_season("cro_a2a_start_new_season", League(this), || o(this))
}

/// Comparer for `cro_a2b_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2b_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CRO_A2B_START_NEW_SEASON);
    compare_state_league_new_season("cro_a2b_start_new_season", League(this), || o(this))
}

// ---------- batch 301: the Croatian leagues (cro_a1.cpp, cro_a2b.cpp): the next stage and the third tier ----------

/// Rule: the Croatian First Division's next stage: at the last stage (+0x30 ≥ +0x2c − 1, signed) the error box (line
/// 0x1fa) and nothing else; otherwise the current stage goes up by one and, on reaching 0 (the league is over), the
/// tie-break play-off is built (`cro_a1_create_playoff_stage`, direct call with ecx = this).
/// Original: `CroA1::advance_stage` (0x514bd0, cro_a1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a1_advance_stage(this: usize) {
    covered!("mwd_6 croatian first division next stage");
    state_league_advance_stage(League(this), 0x9b53c0, 0x1fa, 0x514c60);
}

/// Rule: the Croatian Second Division south group's next stage, as the First Division's (line 0x1e9 of cro_a2b.cpp,
/// the play-off `cro_a2b_create_tiebreak_playoff_stage`).
/// Original: `CroA2b::advance_stage` (0x517180, cro_a2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2b_advance_stage(this: usize) {
    covered!("mwd_6 croatian second division south next stage");
    state_league_advance_stage(League(this), 0x9b5430, 0x1e9, 0x517210);
}

/// Is the club in one of the Croatian divisions below the second (the lower division `KEY_CLUB_COMP_A_LOWER_DIVISION`
/// or the four third division groups: north, east, south, west, compared in that order)?
pub(crate) unsafe fn is_croatian_third_tier_club(club: usize) -> bool {
    let d = read_usize(club + club::DIVISION);
    if d == 0 {
        return false;
    }
    let id = read_usize(d);
    [KEY_CLUB_COMP_A_LOWER_DIVISION, KEY_CLUB_COMP_CROATIAN_THIRD_DIVISION_NORTH, KEY_CLUB_COMP_CROATIAN_THIRD_DIVISION_EAST,
        KEY_CLUB_COMP_CROATIAN_THIRD_DIVISION_SOUTH, KEY_CLUB_COMP_CROATIAN_THIRD_DIVISION_WEST].iter().any(|&g| id == read_usize(g))
}

/// Relegates the club of row `i` of a second division group to the lower division (`KEY_CLUB_COMP_A_LOWER_DIVISION`)
/// when its final status is 3 (relegated): League::get_table_row (`exe::league_get_table_row`: push i -> thiscall, ret
/// 4; called again for the club, as the original) and League::relegate_club_to_division (0x6831a0: push 1; push
/// division; push club; mov ecx, this -> thiscall, ret 0xc).
pub(crate) unsafe fn relegate_croatian_row(this: usize, group: usize, i: i32) {
    if read_u8(exe::league_get_table_row(group, i) + league_table_row::STATUS) == 3 {
        let relegate: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x6831a0usize);
        let club = read_usize(exe::league_get_table_row(group, i));
        relegate(this, club, comp_record_of(KEY_CLUB_COMP_A_LOWER_DIVISION), 1); // mwd6: down to the lower division
    }
}

/// Rule: at the Croatian turnover, the four best clubs of the divisions below the second (the lower division and the
/// four third division groups) go up into the Second Division (north group: the regrouping that follows splits them),
/// and the clubs relegated from both second division groups go down to the lower division.
///
/// A list of `G_CLUB_COUNT` pointers (`exe::crt_malloc`; out of memory: the box at line 0x2d2 and return) with the
/// Croatian clubs (`club::NATION` id = [`KEY_NATION_CROATIA`]) of those divisions (`is_croatian_third_tier_club`), in
/// database order (a 16-bit index). With at least 4: the list sorted by strength and the best shuffled
/// (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 4): 3 pushes; add esp, 0xc -> cdecl) and its first four moved
/// to the north group (League::move_club_to_division `exe::league_move_club_to_division`(this, club, the north group's
/// record, 1): thiscall, ret 0xc); then, for each row i below the north group's club count, the relegated rows of the
/// north group and then of the south group (that loop also bounded by the north group's count, as the original) go down
/// (`relegate_croatian_row`). The list is freed in every case.
/// Original: `cro_a1_swap_second_and_third_tier_clubs` (0x514ed0, cro_a1.cpp, thiscall: ecx = the First Division, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a1_swap_second_and_third_tier_clubs(this: usize) {
    let north = comp_by_global(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH);
    let south = comp_by_global(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH);
    let list = exe::crt_malloc((read_usize(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0x9b53c0, 0x2d2);
        return;
    }
    let mut n: u32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let nation = read_usize(club + club::NATION);
        if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_CROATIA) && is_croatian_third_tier_club(club) {
            write_usize(list + (n as u16 as i16 as isize as usize).wrapping_mul(4), club);
            n = n.wrapping_add(1);
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    if (n as u16 as i16) >= 4 {
        covered!("mwd_6 croatian third tier up");
        exe::comp_util_sort_and_shuffle_top_clubs(list, n, 4); // mwd6: the best four, shuffled
        for k in 0..4 {
            exe::league_move_club_to_division(this, read_usize(list + 4 * k), comp_record_of(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH), 1); // mwd6: up to the north group
        }
        let count = League(north).club_count() as i32;
        for i in 0..count {
            relegate_croatian_row(this, north, i);
            relegate_croatian_row(this, south, i);
        }
    }
    exe::crt_free(list);
}

/// Comparer for `cro_a1_advance_stage` (side by side on the current stage, only the original when the play-off would
/// be built: `ita_ser_c1a_compare_advance_stage`).
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CRO_A1_ADVANCE_STAGE);
    ita_ser_c1a_compare_advance_stage("cro_a1_advance_stage", League(this), || o(this), || cro_a1_advance_stage(this))
}

/// Comparer for `cro_a2b_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2b_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CRO_A2B_ADVANCE_STAGE);
    ita_ser_c1a_compare_advance_stage("cro_a2b_advance_stage", League(this), || o(this), || cro_a2b_advance_stage(this))
}

/// Comparer for `cro_a1_swap_second_and_third_tier_clubs`: only the original runs (it moves clubs and draws); with at
/// least 4 candidates, the clubs of both second division groups whose final status is 3 must end in the lower
/// division (the south group's rows only below the north group's club count, as the original).
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_swap_second_and_third_tier_clubs(this: usize) {
    let lower = comp_record_of(KEY_CLUB_COMP_A_LOWER_DIVISION);
    let candidates = (0..read_usize(G_CLUB_COUNT)).filter(|&i| {
        let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        let nation = read_usize(c + club::NATION);
        nation != 0 && read_usize(nation) == read_usize(KEY_NATION_CROATIA) && is_croatian_third_tier_club(c)
    }).count();
    let mut planned: Vec<usize> = Vec::new();
    for g in [KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH] {
        let l = League(comp_by_global(g));
        if l.table() != 0 && candidates >= 4 {
            for i in 0..l.club_count() as i32 {
                let row = l.table() + i as usize * league_table_row::SIZE;
                if read_u8(row + league_table_row::STATUS) == 3 {
                    planned.push(read_usize(row));
                }
            }
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CRO_A1_SWAP_SECOND_AND_THIRD_TIER_CLUBS)(this);
    let wrong: Vec<usize> = planned.iter().copied().filter(|&c| read_usize(c + club::DIVISION) != lower).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("cro_a1_swap_second_and_third_tier_clubs({:#x}): relegated clubs not in the lower division {:x?}", this, wrong));
}

// ---------- batch 301: the Croatian leagues (cro_a1.cpp, cro_a2b.cpp): the tie-break play-off ----------

/// The clubs that go to the tie-break play-off, in the order of the stage's list: scanning the table rows i from the
/// top, a row and the next one form a pair when they are level on points (`league_table_row::POINTS`) and neither has
/// the final status 3 (relegated); the pair takes both rows and the scan goes on two rows below.
pub(crate) unsafe fn croatian_tiebreak_plan(this: League) -> Vec<usize> {
    let mut v = Vec::new();
    let n = this.club_count() as i32;
    let mut i = 0i32;
    while i < n {
        let row = this.table() + i as usize * league_table_row::SIZE;
        let next = row + league_table_row::SIZE;
        if i < n - 1 && read_u16(row + league_table_row::POINTS) == read_u16(next + league_table_row::POINTS)
            && read_u8(row + league_table_row::STATUS) != 3 && read_u8(next + league_table_row::STATUS) != 3 {
            v.push(read_usize(row));
            v.push(read_usize(next));
            i += 1;
        }
        i += 1;
    }
    v
}

/// The body of the two Croatian tie-break play-offs (`create_playoff_stage`, thiscall, ret 0).
///
/// The table scan (16-bit row index i, while i < `club_count`): when i is not the last row and rows i and i + 1 are
/// level on points with neither of status 3, both clubs go into the list (a local array), row i's worst possible
/// position (`league_table_row::WORST_POSSIBLE_POSITION`) becomes i + 2 and row i + 1's best (`BEST_POSSIBLE_POSITION`)
/// i + 1, both statuses 1, and the scan skips row i + 1; otherwise, when i > 0 and row i is level on points with row i
/// − 1, its worst and best possible positions become i + 1. The pair count (the list count / 2, truncated) goes into
/// `TIEBREAK_PAIRS`.
///
/// No pair: no extra stage (+0x2c = 0, current stage −1), no tie-break matches (`league::TIEBREAK_MATCHES` = 0) and the
/// positions and statuses recomputed (virtual `league_vtable::UPDATE_POSITIONS_AND_STATUS`, thiscall, ret 0).
/// Otherwise: the stage 0 round (virtual `comp_vtable::CREATE_SCHEDULE`: push 0; push &extra; push &rounds; push 0 ->
/// thiscall, ret 0x10; `croatian_tiebreak_round`), the stage (operator new `exe::crt_operator_new` of 0xb2 and
/// Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`(league, count, list, rounds, competition, round, year, 0, 1, extra, 0x14,
/// 0, 0, 0, 0): 15 pushes -> thiscall, ret 0x3c; the rounds, extra and year pushed as dwords with stack garbage above
/// the words, which the constructor does not read) into the first slot of the stage vector (+0xc); no stage: the box
/// at line `row` and return; otherwise the round freed (`exe::crt_free`), the first round set up (Cup::set_up_round
/// `exe::cup_set_up_round`: push 0 -> thiscall, ret 4), current stage 0 and the first tie-break code 6
/// (`league::TIEBREAKS`).
pub(crate) unsafe fn create_croatian_tiebreak_playoff(this: League, file: usize, row: u32) {
    let mut list = [0usize; 0x40];
    let mut count: u32 = 0;
    let n = this.club_count();
    let mut i: i16 = 0;
    while i < n {
        let r = this.table() + (i as i32 as usize).wrapping_mul(league_table_row::SIZE);
        let next = r + league_table_row::SIZE;
        if (i as i32) < n as i32 - 1 && read_u16(r + league_table_row::POINTS) == read_u16(next + league_table_row::POINTS)
            && read_u8(r + league_table_row::STATUS) != 3 && read_u8(next + league_table_row::STATUS) != 3 {
            list[(count as u16 as i16 as usize) & 0x3f] = read_usize(r);
            list[((count + 1) as u16 as i16 as usize) & 0x3f] = read_usize(next);
            count += 2;
            write_u8(r + league_table_row::WORST_POSSIBLE_POSITION, (i as u8).wrapping_add(2));
            write_u8(next + league_table_row::BEST_POSSIBLE_POSITION, (i as u8).wrapping_add(1));
            write_u8(r + league_table_row::STATUS, 1);
            write_u8(next + league_table_row::STATUS, 1);
            i += 1;
        } else if i > 0 && read_u16(r + league_table_row::POINTS) == read_u16(r - league_table_row::SIZE + league_table_row::POINTS) {
            write_u8(r + league_table_row::WORST_POSSIBLE_POSITION, (i as u8).wrapping_add(1));
            write_u8(r + league_table_row::BEST_POSSIBLE_POSITION, (i as u8).wrapping_add(1));
        }
        i += 1;
    }
    let pairs = ((count as u16 as i16 as i32) / 2) as u8;
    write_u8(this.addr() + TIEBREAK_PAIRS, pairs);
    if pairs == 0 { // mwd7: no level clubs
        this.set_stage_count(0);
        this.set_current_stage(0xffff_ffff);
        this.set_tiebreak_matches(0);
        bra_nat_first_call_virtual_no_args(this.addr(), league_vtable::UPDATE_POSITIONS_AND_STATUS);
        return;
    }
    covered!("mwd_7 croatian tie-break play-off");
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let t = schedule(this.addr(), 0, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, *const usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), count, list.as_ptr(), rounds as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(this.stages(), stage);
    if read_usize(this.stages()) == 0 {
        show_error_at(file, row);
        return;
    }
    exe::crt_free(t);
    exe::cup_set_up_round(read_usize(this.stages()), 0);
    this.set_current_stage(0);
    write_u8(this.addr() + league::TIEBREAKS, 6);
}

/// Rule: at the end of the Croatian First Division, the clubs level on points with the club right below them (neither
/// relegated) play a one-round tie-break play-off (stage 0, in May): each pair of level rows goes into the play-off and
/// the positions they can still reach are narrowed; with no level pair, the season ends without tie-breaks.
/// Original: `CroA1::create_playoff_stage` (0x514c60, cro_a1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a1_create_playoff_stage(this: usize) {
    create_croatian_tiebreak_playoff(League(this), 0x9b53c0, 0x24b);
}

/// Rule: the same tie-break play-off at the end of the Croatian Second Division south group (line 0x23a of
/// cro_a2b.cpp).
/// Original: `CroA2b::create_tiebreak_playoff_stage` (0x517210, cro_a2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2b_create_tiebreak_playoff_stage(this: usize) {
    create_croatian_tiebreak_playoff(League(this), 0x9b5430, 0x23a);
}

/// Comparer for the Croatian tie-break play-offs: only the original runs (it builds a stage); the pair count must be
/// half the planned clubs (`croatian_tiebreak_plan`) and, with pairs, the stage's team list must hold the planned clubs
/// in order.
pub(crate) unsafe fn compare_croatian_tiebreak_playoff(name: &str, this: League, original: impl FnOnce()) {
    let plan = if this.table() != 0 { croatian_tiebreak_plan(this) } else { Vec::new() };
    original();
    let pairs = read_u8(this.addr() + TIEBREAK_PAIRS);
    let mut ok = pairs as usize == plan.len() / 2;
    let mut found: Vec<usize> = Vec::new();
    if ok && pairs != 0 {
        let stage = if this.stages() != 0 { Cup(read_usize(this.stages())) } else { Cup(0) };
        ok = !stage.is_null() && stage.team_table() != 0;
        if ok {
            found = (0..plan.len()).map(|k| read_usize(stage.team_table() + 6 * k)).collect();
            let (mut a, mut b) = (plan.clone(), found.clone());
            a.sort_unstable();
            b.sort_unstable();
            ok = a == b;
        }
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}): pairs {} plan {:x?} stage list {:x?}", name, this.addr(), pairs, plan, found));
}

/// Comparer for `cro_a1_create_playoff_stage`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_create_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CRO_A1_CREATE_PLAYOFF_STAGE);
    compare_croatian_tiebreak_playoff("cro_a1_create_playoff_stage", League(this), || o(this))
}

/// Comparer for `cro_a2b_create_tiebreak_playoff_stage`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2b_create_tiebreak_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CRO_A2B_CREATE_TIEBREAK_PLAYOFF_STAGE);
    compare_croatian_tiebreak_playoff("cro_a2b_create_tiebreak_playoff_stage", League(this), || o(this))
}

// ---------- batch 301: the Croatian Cup (cro_cup.cpp) ----------

/// What changes from one national cup constructor to another (the CroCup template, shared by the Polish, Russian and
/// Turkish cups and the Greek and Polish super cups).
pub(crate) struct EastNationCup {
    pub(crate) vtable: usize,
    /// the discipline rule (+0x50)
    pub(crate) discipline: u8,
    /// `collect_entrants` (or `select_teams` in a super cup), direct call, thiscall, ret 0
    pub(crate) entrants: usize,
    /// the prize fund (+0x28) written before the round table; `None` in the super cups (not written)
    pub(crate) prize: Option<u32>,
    /// +0x49 = 7 and +0x4a = 3 written after the round table instead of before the load (the Polish League Cup)
    pub(crate) late_substitutes: bool,
    /// the stage vector (+0xc) allocated from +0x2c (the Russian Cup)
    pub(crate) stage_vector: bool,
    /// Cup::rank_teams at the end (not in the super cups)
    pub(crate) rank: bool,
    pub(crate) file: usize,
    /// the boxes: entrants, fixture tree, statistics
    pub(crate) rows: [u32; 3],
}

/// Calls the exe routine at `f` as thiscall(c) with no arguments (ret 0) and returns its eax.
pub(crate) unsafe fn call_thiscall_no_args(f: usize, c: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(f);
    f(c)
}

/// The body of the national cup constructors (thiscall(this, year, record), ret 8; the SEH frame only covers the
/// operator new of the statistics).
///
/// Cup::Cup (`exe::new_cup`, thiscall, ret 0); the competition record (+4); the vtable; the year (+0x40, word); the
/// current round (+0xab) = 0; the main stage (+0x44 = 0xff), current stage −1 (+0x30), no stages (+0x2c = 0); club
/// sides (+0x42 = 1); 7 substitutes (+0x49) and 3 substitutions (+0x4a) (here or later, `late_substitutes`); the
/// discipline rule (+0x50); +0xb1 = 0. The saved state (Cup::load_from_disk 0x51fc00: push 1 -> thiscall, ret 4):
/// loaded, returns this. Otherwise: the stage vector when `stage_vector` (+0xc = malloc(+0x2c × 4) when +0x2c > 0,
/// else 0); the entrants (`entrants`; 0: the box of line `rows[0]` and this; note that the −1 of a failed allocation
/// passes as success, as the original); the prize fund; the round table (virtual `comp_vtable::CREATE_SCHEDULE`: push 0;
/// push &+0x3a; push &+0x3c; push −1 -> thiscall, ret 0x10) into `cup::ROUND_TABLE`; the fixture tree
/// (Cup::map_fixture_tree 0x518790, thiscall, ret 0; 0: the box `rows[1]` and this); the statistics (operator new
/// `exe::crt_operator_new` of 0x5ce and Comp_Stats::Comp_Stats `exe::new_comp_stats`(this); 0: the box `rows[2]` and
/// this) into +8; last, when `rank`, the predicted positions (Cup::rank_teams 0x5223a0, thiscall, ret 0). Returns
/// this.
pub(crate) unsafe fn construct_east_nation_cup(this: Cup, year: u32, record: usize, p: &EastNationCup, branch: &'static str) -> usize {
    let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(p.entrants);
    exe::new_cup(this);
    write_usize(this.addr() + 4, record);
    write_usize(this.addr(), p.vtable);
    this.set_year((year as u16) as i16);
    write_u8(this.addr() + cup::CURRENT_ROUND, 0);
    write_u8(this.addr() + cup::STAGE_INDEX, 0xff);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    write_u32(this.addr() + cup::STAGE_COUNT, 0);
    write_u8(this.addr() + cup::CLUB_SIDES, 1);
    if !p.late_substitutes {
        write_u8(this.addr() + cup::SUBSTITUTES_NAMED, 7);
        write_u8(this.addr() + cup::SUBSTITUTIONS_ALLOWED, 3);
    }
    write_u8(this.addr() + cup::DISCIPLINE_INDEX, p.discipline); // mwd8: the discipline rule
    write_u8(this.addr() + 0xb1, 0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!(branch);
    if p.stage_vector {
        let n = read_i32(this.addr() + cup::STAGE_COUNT);
        let v = if n > 0 { exe::crt_malloc((n as u32).wrapping_mul(4)) } else { 0 };
        write_usize(this.addr() + cup::STAGES, v);
    }
    if entrants(this.addr()) == 0 {
        show_error_at(p.file, p.rows[0]);
        return this.addr();
    }
    if let Some(v) = p.prize {
        write_u32(this.addr() + cup::PRIZE_FUND, v);
    }
    let schedule: unsafe extern "thiscall" fn(usize, u32, usize, usize, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = schedule(this.addr(), 0xffff_ffff, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0);
    write_usize(this.addr() + cup::ROUND_TABLE, t);
    if p.late_substitutes {
        write_u8(this.addr() + cup::SUBSTITUTES_NAMED, 7);
        write_u8(this.addr() + cup::SUBSTITUTIONS_ALLOWED, 3);
    }
    if call_thiscall_no_args(0x518790, this.addr()) == 0 {
        show_error_at(p.file, p.rows[1]);
        return this.addr();
    }
    let e = exe::crt_operator_new(0x5ce);
    let s = if e != 0 { exe::new_comp_stats(e, this) } else { 0 };
    write_usize(this.addr() + 8, s);
    if s == 0 {
        show_error_at(p.file, p.rows[2]);
        return this.addr();
    }
    if p.rank {
        call_thiscall_no_args(0x5223a0, this.addr());
    }
    this.addr()
}

/// Comparer for the national cup constructors: only the original runs; checks the return, vtable, year, record,
/// discipline rule, substitutes and, for a new cup (not loaded, with statistics), the team and round tables and the
/// prize fund.
pub(crate) unsafe fn compare_east_nation_cup_ctor(name: &str, this: Cup, year: u32, record: usize, p: &EastNationCup, original: impl FnOnce() -> usize) -> usize {
    let loaded = league_will_load_saved_state(record);
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == p.vtable && (this.year() as u16) == year as u16 && read_usize(this.addr() + 4) == record
        && this.discipline_index() == p.discipline && read_u8(this.addr() + cup::SUBSTITUTES_NAMED) == 7 && read_u8(this.addr() + cup::SUBSTITUTIONS_ALLOWED) == 3;
    if !loaded && read_usize(this.addr() + 8) != 0 {
        ok = ok && this.team_table() != 0 && this.round_table() != 0 && p.prize.map_or(true, |v| this.prize_fund() == v as i32);
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}) [year {} arg {}, record {:#x} arg {:#x}]: returned {:#x}, vtable {:#x}, discipline {}, teams {:#x}, rounds {:#x}, prize {}", name, this.addr(), this.year(), year as u16,
        read_usize(this.addr() + 4), record, r, read_usize(this.addr()), this.discipline_index(), this.team_table(), this.round_table(), this.prize_fund()));
    r
}

/// The Croatian Cup: discipline rule 0x21, 100,000 prize fund.
pub(crate) const CROATIAN_CUP: EastNationCup = EastNationCup {
    vtable: 0x9692e0,
    discipline: 0x21,
    entrants: 0x517bb0,
    prize: Some(0x186a0),
    late_substitutes: false,
    stage_vector: false,
    rank: true,
    file: 0x9b5468,
    rows: [0x40, 0x4e, 0x55],
};

/// Rule: the Croatian Cup at career creation: discipline rule 0x21, 7 substitutes and 3 substitutions and, with no
/// saved state, the 48 entrants, a 100,000 prize fund, the six rounds, the fixture tree, the statistics and the
/// predicted positions (`construct_east_nation_cup`).
/// Original: `CroCup::CroCup` (0x517480, cro_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_cro_cup(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &CROATIAN_CUP, "mwd_8 croatian cup new")
}

/// The Croatian Cup: six single-leg rounds from June to April, no prize per round won (−1 = a share of the fund).
pub(crate) const CROATIAN_CUP_ROUNDS: [FraSuperCupRound; 6] = [
    cql12r((0x10, 6, 0, 4), (0xf, 7, 0, 6, 1, 0), 0xe6, (0, 1, 0), 4, (0x20, 0x10, 0x20, 0), (0, 1, 0), (0, 0xffffffff, 0)), // mwd8: the preliminary round
    cql12r((0x14, 7, 0, 4), (0x16, 8, 0, 2, 2, 0), 0xa, (0, 1, 0), 4, (0x20, 0x10, 0x10, 0x20), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((0x18, 8, 0, 4), (0x1b, 9, 0, 2, 2, 0), 0x14, (0, 1, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((0x1d, 9, 0, 4), (7, 0xa, 0, 2, 2, 0), 0x78, (0, 0, 1), 4, (8, 4, 0, 0), (0, 2, 0xe), (0, 0xffffffff, 0)),
    cql12r((0x17, 0xa, 0, 4), (0x14, 2, 1, 2, 2, 0), 0x82, (0, 0, 3), 4, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffffffff, 0)),
    cql12r((4, 3, 1, 3), (0x18, 4, 1, 2, 2, 0), 0x96, (0, 0, 1), 6, (2, 1, 0, 0), (0, 2, 7), (0, 0xffffffff, 0)),
];

/// Rule: the Croatian Cup has six rounds: a preliminary round (code 0xe6, 32 teams: the 32 second division clubs) drawn
/// 16/7, played 15/8 (weekday 6); the first round (16 new teams: the First Division) drawn 20/8, played 22/9 (weekday
/// 2); the round of 16 drawn 24/9, played 27/10; the quarter-finals, two legs 14 days apart, drawn 29/10, played 7/11;
/// the semi-finals, two legs 14 days apart (two-leg tie-break 3), drawn 23/11, played 20/3 of the next year; the
/// final, two legs 7 days apart, drawn 4/4, played 24/5 (category 6); no prize per round won (a share of the fund).
/// Original: `CroCup::create_round_table` (0x517740, cro_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn cro_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    fra_super_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &CROATIAN_CUP_ROUNDS, 0x9b5468, 0x99, "mwd_8 croatian cup rounds", |_| {})
}

/// Rule: the Croatian Cup takes 48 clubs: first the 32 clubs of both second division groups
/// (`KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH`/`_SOUTH`), each with a random seed 0..49 (`exe::utils_random`(50)),
/// then the 16 First Division clubs (`KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4`), in database order, status 0.
///
/// `team_count` = 48; the list (`exe::crt_malloc` of 48 × 6 into `cup::TEAM_TABLE`; out of memory: the box at line
/// 0x12b and −1); the clubs looked at are the first [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`] (without the national
/// teams [inference]); there is no bound on the list (the database fills it exactly). The First Division clubs' seed
/// byte (+4) is not written: it keeps what the allocation had (as the original). Returns 1.
/// Original: `CroCup::collect_entrants` (0x517bb0, cro_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    this.set_team_count(0x30);
    let l = exe::crt_malloc(0x120);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0x9b5468, 0x12b);
        return -1;
    }
    let limit = || (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let mut k = 0usize;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < limit() {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && (read_usize(d) == read_usize(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH) || read_usize(d) == read_usize(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH)) {
            let e = this.team_table() + k;
            write_usize(e, club);
            write_u8(e + 5, 0);
            write_u8(this.team_table() + k + 4, exe::utils_random(0x32) as u8); // mwd8: the second division seed
            k += 6;
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    covered!("mwd_8 croatian cup entrants");
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < limit() {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4) {
            write_usize(this.team_table() + k, club);
            write_u8(this.team_table() + k + 5, 0);
            k += 6;
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    1
}

/// Comparer for `construct_cro_cup`.
pub(crate) unsafe extern "thiscall" fn compare_construct_cro_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_CRO_CUP);
    compare_east_nation_cup_ctor("construct_cro_cup", Cup(this), year, record, &CROATIAN_CUP, || o(this, year, record))
}

/// Comparer for `cro_cup_create_round_table` (side by side, the written rounds compared).
pub(crate) unsafe extern "thiscall" fn compare_cro_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_CRO_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("cro_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| cro_cup_create_round_table(this, stage, r, e, s))
}

/// Side by side for an entrant list whose seeds are written only for some entries: the original and the copy (the
/// random generator restored between them), then each entry's club and status compared, and the seed only where
/// `seeded(entry index)`; the copy's list is freed.
pub(crate) unsafe fn compare_entrants_with_seeds(name: &str, this: Cup, o: impl FnOnce() -> i32, c: impl FnOnce() -> i32, seeded: impl Fn(usize) -> bool) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this.addr() + cup::TEAM_COUNT, 2usize), (G_MOUSE_LEFT_BUTTON_DOWN, 4)];
    let (a, b, same, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o();
        lo.set(this.team_table());
        r
    }, || {
        let r = c();
        lr.set(this.team_table());
        this.set_team_table(lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let n = if a == 1 { this.team_count().max(0) as usize } else { 0 };
    let list = if lo != 0 && lr != 0 {
        (0..n).all(|k| read_usize(lo + 6 * k) == read_usize(lr + 6 * k) && read_u8(lo + 6 * k + 5) == read_u8(lr + 6 * k + 5) && (!seeded(k) || read_u8(lo + 6 * k + 4) == read_u8(lr + 6 * k + 4)))
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(same && a == b && list, || format!("{}({:#x}) returned {} x {} memory {} list {} ({} entries)", name, this.addr(), a, b, same, list, n));
    a
}

/// Comparer for `cro_cup_collect_entrants` (the seeds compared only for the second division clubs).
pub(crate) unsafe extern "thiscall" fn compare_cro_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_CRO_CUP_COLLECT_ENTRANTS);
    let c = Cup(this);
    let first = KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4;
    compare_entrants_with_seeds("cro_cup_collect_entrants", c, || o(this), || cro_cup_collect_entrants(this), |k| {
        let club = read_usize(c.team_table() + 6 * k);
        let d = if club != 0 { read_usize(club + club::DIVISION) } else { 0 };
        d != 0 && read_usize(d) != read_usize(first)
    })
}

// ---------- batch 301: the season turnover of the Croatian, Polish and Polish League cups ----------

/// Rule: the Croatian Cup season turnover: last edition's tables freed, its statistics archived, the year + 1, the 48
/// entrants (`cro_cup_collect_entrants`), the rounds, the fixture tree and the predicted positions
/// (`start_new_cup_season`, the template of the knockout cups; lines 0x1a5, 0x1ac, 0x1b3 of cro_cup.cpp).
/// Original: `CroCup::start_new_season` (0x517d60, cro_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x517bb0, 0x9b5468, [0x1a5, 0x1ac, 0x1b3], 1, "mwd_9 croatian cup turnover") // mwd9: the Croatian Cup turnover
}

/// Rule: the Polish Cup season turnover, as the Croatian Cup's, with the Polish Cup entrants (`PolCup::collect_entrants`
/// 0x7c8550; lines 0x1de, 0x1e5, 0x1ec of pol_cup.cpp).
/// Original: `PolCup::start_new_season` (0x7c88a0, pol_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x7c8550, 0xa78564, [0x1de, 0x1e5, 0x1ec], 1, "mwd_9 polish cup turnover")
}

/// Rule: the Polish League Cup season turnover, as the Croatian Cup's, with its entrants (`PolLgeCup::collect_entrants`
/// 0x7cad30; lines 0x1f3, 0x1fa, 0x201 of pol_lge_cup.cpp).
/// Original: `PolLgeCup::start_new_season` (0x7cafe0, pol_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_lge_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x7cad30, 0xa785d4, [0x1f3, 0x1fa, 0x201], 1, "mwd_9 polish league cup turnover")
}

/// Comparer for `cro_cup_start_new_season` (only the original; the year went up by one).
pub(crate) unsafe extern "thiscall" fn compare_cro_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CRO_CUP_START_NEW_SEASON);
    compare_cup_new_season("cro_cup_start_new_season", Cup(this), || o(this))
}

/// Comparer for `pol_cup_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_pol_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POL_CUP_START_NEW_SEASON);
    compare_cup_new_season("pol_cup_start_new_season", Cup(this), || o(this))
}

/// Comparer for `pol_lge_cup_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_pol_lge_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POL_LGE_CUP_START_NEW_SEASON);
    compare_cup_new_season("pol_lge_cup_start_new_season", Cup(this), || o(this))
}

// ---------- batch 301: the season turnover of the Russian and Turkish cups and the Greek Super Cup ----------

/// Rule: the Russian Cup season turnover: last edition's tables freed, its statistics archived, the year + 1, the 48
/// entrants (`RusCup::select_entrants` 0x7e99a0), the rounds, the fixture tree and the predicted positions
/// (`start_new_cup_season`; lines 0x1ce, 0x1d5, 0x1dc of rus_cup.cpp).
/// Original: `RusCup::start_new_season` (0x7e9d10, rus_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x7e99a0, 0xa7ac58, [0x1ce, 0x1d5, 0x1dc], 1, "mwd_10 russian cup turnover")
}

/// Rule: the Turkish Cup season turnover, as the Russian Cup's, with its 64 entrants (`TurCup::collect_entrants`
/// 0x8fcb90; lines 0x200, 0x207, 0x20e of tur_cup.cpp).
/// Original: `TurCup::start_new_season` (0x8fcdb0, tur_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x8fcb90, 0xaa180c, [0x200, 0x207, 0x20e], 1, "mwd_10 turkish cup turnover") // mwd10: the Turkish Cup turnover
}

/// Rule: the Greek Super Cup season turnover: the finished edition's tables freed, its statistics archived, the year
/// + 1, the two clubs (`GreSuper::select_teams` 0x5ec430: the league champion and the cup winner), the round and the
/// fixture tree; no predicted positions (`usa_mls_all_stars_cup_start_new_season`; lines 0x136, 0x13d, 0x144 of
/// gre_super.cpp).
/// Original: `GreSuper::start_new_season` (0x5ec6b0, gre_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x5ec430, 0x9cdb58, [0x136, 0x13d, 0x144], 1, false, "mwd_10 greek super cup turnover")
}

/// Comparer for `rus_cup_start_new_season` (only the original; the year went up by one).
pub(crate) unsafe extern "thiscall" fn compare_rus_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_RUS_CUP_START_NEW_SEASON);
    compare_cup_new_season("rus_cup_start_new_season", Cup(this), || o(this))
}

/// Comparer for `tur_cup_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_tur_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_TUR_CUP_START_NEW_SEASON);
    compare_cup_new_season("tur_cup_start_new_season", Cup(this), || o(this))
}

/// Comparer for `gre_super_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_gre_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GRE_SUPER_START_NEW_SEASON);
    compare_cup_new_season("gre_super_start_new_season", Cup(this), || o(this))
}

// ---------- batch 301: the constructors of the Greek and Russian divisions (gre_prm.cpp, gre_second.cpp, rus_first.cpp) ----------

/// The Greek First Division (Alpha Ethniki): discipline rule 0x20 (the Greek one).
pub(crate) const GREEK_FIRST_DIVISION: LeagueDivision = LeagueDivision {
    vtabela: 0x96b568,
    discipline: 0x20, // mwd11: the Greek discipline rule
    rules: 0x5eae90,
    table: 0x5ead60,
    file: 0x9cdae4,
    rows: [0x49, 0x50, 0x57, 0x60, 0x67],
    low: false,
};
/// The Greek Second Division (Beta Ethniki).
pub(crate) const GREEK_SECOND_DIVISION: LeagueDivision = LeagueDivision {
    vtabela: 0x96b61c,
    discipline: 0x20,
    rules: 0x5ebde0,
    table: 0x5ebcb0,
    file: 0x9cdb1c,
    rows: [0x49, 0x50, 0x57, 0x60, 0x67],
    low: false,
};
/// The Russian First Division: discipline rule 0x27 (the Russian one).
pub(crate) const RUSSIAN_FIRST_DIVISION: LeagueDivision = LeagueDivision {
    vtabela: 0x96f1c4,
    discipline: 0x27,
    rules: 0x7ea810,
    table: 0x7ea6c0,
    file: 0xa7ac8c,
    rows: [0x4a, 0x51, 0x58, 0x5f, 0x66],
    low: false,
};

/// Rule: the Greek First Division at career creation: discipline rule 0x20 and, with no saved state, the main stage
/// only, the rules, the table, the fixtures, the order, the statistics and the predicted positions (the division
/// constructor template, `construct_division_league`).
/// Original: `GrePrm::GrePrm` (0x5ea410, gre_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_gre_prm(this: usize, year: u32, record: usize) -> usize {
    covered!("mwd_11 greek first division");
    construct_division_league(League(this), year, record, &GREEK_FIRST_DIVISION)
}

/// Rule: the Greek Second Division at career creation, as the First Division (`construct_division_league`).
/// Original: `GreSecond::GreSecond` (0x5eb2b0, gre_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_gre_second(this: usize, year: u32, record: usize) -> usize {
    covered!("mwd_11 greek second division");
    construct_division_league(League(this), year, record, &GREEK_SECOND_DIVISION)
}

/// Rule: the Russian First Division at career creation: discipline rule 0x27 and the division constructor template
/// (`construct_division_league`).
/// Original: `RusFirst::RusFirst` (0x7e9f30, rus_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_rus_first(this: usize, year: u32, record: usize) -> usize {
    covered!("mwd_11 russian first division");
    construct_division_league(League(this), year, record, &RUSSIAN_FIRST_DIVISION)
}

/// Comparer for `construct_gre_prm` (only the original runs; `compare_division_league_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_gre_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_GRE_PRM);
    compare_division_league_ctor("construct_gre_prm", League(this), year, record, &GREEK_FIRST_DIVISION, || o(this, year, record))
}

/// Comparer for `construct_gre_second`.
pub(crate) unsafe extern "thiscall" fn compare_construct_gre_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_GRE_SECOND);
    compare_division_league_ctor("construct_gre_second", League(this), year, record, &GREEK_SECOND_DIVISION, || o(this, year, record))
}

/// Comparer for `construct_rus_first`.
pub(crate) unsafe extern "thiscall" fn compare_construct_rus_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_RUS_FIRST);
    compare_division_league_ctor("construct_rus_first", League(this), year, record, &RUSSIAN_FIRST_DIVISION, || o(this, year, record))
}

// ---------- batch 301: the constructors of the Turkish divisions and the Russian Premier League (tur_first.cpp, tur_second.cpp, rus_prm.cpp) ----------

/// The Turkish First Division (the top flight): discipline rule 0x25 (the Turkish one).
pub(crate) const TURKISH_FIRST_DIVISION: LeagueDivision = LeagueDivision {
    vtabela: 0x97052c,
    discipline: 0x25, // mwd12: the Turkish discipline rule
    rules: 0x8fdf90,
    table: 0x8fddc0,
    file: 0xaa1840,
    rows: [0x48, 0x4f, 0x56, 0x5f, 0x66],
    low: false,
};
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
/// The Russian Premier League (Top Division): discipline rule 0x27; points ties settled by tie-break matches.
pub(crate) const RUSSIAN_PREMIER_LEAGUE: LeagueDivision = LeagueDivision {
    vtabela: 0x96f278,
    discipline: 0x27,
    rules: 0x7eb740,
    table: 0x7eb610,
    file: 0xa7acc8,
    rows: [0x50, 0x57, 0x5e, 0x67, 0x6e],
    low: false,
};

/// Rule: the Turkish First Division at career creation: discipline rule 0x25 and the division constructor template
/// (`construct_division_league`).
/// Original: `TurFirst::TurFirst` (0x8fcf60, tur_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_tur_first(this: usize, year: u32, record: usize) -> usize {
    covered!("mwd_12 turkish first division");
    construct_division_league(League(this), year, record, &TURKISH_FIRST_DIVISION)
}

/// Rule: the Turkish Second Division at career creation, as the First Division (`construct_division_league`).
/// Original: `TurSecond::TurSecond` (0x8fe6f0, tur_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_tur_second(this: usize, year: u32, record: usize) -> usize {
    covered!("mwd_12 turkish second division");
    construct_division_league(League(this), year, record, &TURKISH_SECOND_DIVISION)
}

/// Rule: the Russian Premier League at career creation: discipline rule 0x27 and, with no saved state, no tie-break
/// pairs, the season open and the rest of the division template (`construct_tiebreak_league`, the Croatian template).
/// Original: `RusPrm::RusPrm` (0x7eaa80, rus_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_rus_prm(this: usize, year: u32, record: usize) -> usize {
    construct_tiebreak_league(League(this), year, record, &RUSSIAN_PREMIER_LEAGUE, "mwd_12 russian premier league new")
}

/// Comparer for `construct_tur_first` (only the original runs; `compare_division_league_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_tur_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_TUR_FIRST);
    compare_division_league_ctor("construct_tur_first", League(this), year, record, &TURKISH_FIRST_DIVISION, || o(this, year, record))
}

/// Comparer for `construct_tur_second`.
pub(crate) unsafe extern "thiscall" fn compare_construct_tur_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_TUR_SECOND);
    compare_division_league_ctor("construct_tur_second", League(this), year, record, &TURKISH_SECOND_DIVISION, || o(this, year, record))
}

/// Comparer for `construct_rus_prm` (`compare_tiebreak_league_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_rus_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_RUS_PRM);
    compare_tiebreak_league_ctor("construct_rus_prm", League(this), year, record, &RUSSIAN_PREMIER_LEAGUE, || o(this, year, record))
}

// ---------- batch 301: the rules of the Greek divisions and the Russian First Division (gre_prm.cpp, gre_second.cpp, rus_first.cpp) ----------

/// The rules of the Greek First Division (16 clubs): 2 legs, 3 points a win and 1 a draw, league matches (+0xc4 = 2),
/// +0xab = +0xdb = 0, national clubs; tie-breaks 1, 2, 3 (goal difference, goals for, wins; +0xc8 = 0 ends); nobody
/// goes up nor to the play-offs, 2 go down (+0xc1) to the Second Division (`KEY_CLUB_COMP_GREEK_SECOND_DIVISION`,
/// +0x20); nothing above; +0x52 = 2; 7 substitutes, 3 substitutions; games per club not computed yet (+0xa7 = −1); the
/// rule bit 0x40 (+0xd9, listed for the comparer).
pub(crate) unsafe fn gre_prm_rules() -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 3),
        (0xc8, 1, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 2), (0x1c, 4, 0xffff_ffff), (0x52, 1, 2), (0x4a, 1, 3),
        (0x20, 4, read_usize(KEY_CLUB_COMP_GREEK_SECOND_DIVISION) as u32), (0x49, 1, 7), (0xa7, 2, 0xffff), (league::LEAGUE_FLAGS, 1, 0)]
}

/// The rules of the Greek Second Division: as the First Division, with 2 going up (+0xbe) and 2 down, the First
/// Division above (`KEY_CLUB_COMP_GREEK_PREMIER_DIVISION`, +0x1c) and nothing below.
pub(crate) unsafe fn gre_second_rules() -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 3),
        (0xc8, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0), (0xbe, 1, 2), (0xc1, 1, 2), (0x4a, 1, 3), (0x1c, 4, read_usize(KEY_CLUB_COMP_GREEK_PREMIER_DIVISION) as u32),
        (0x52, 1, 2), (0x20, 4, 0xffff_ffff), (0x49, 1, 7), (0xa7, 2, 0xffff), (league::LEAGUE_FLAGS, 1, 0)]
}

/// The rules of the Russian First Division: 2 legs, 3 points a win and 1 a draw; tie-breaks 3, 4, 1, 2 (wins, head to
/// head, goal difference, goals for); 2 go up (+0xbe), 3 go down (+0xc1); the Premier League above
/// (`KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION`), nothing below; +0x52 = 2; 7 substitutes, 3 substitutions; +0xa7 = −1;
/// no rule bits set.
pub(crate) unsafe fn rus_first_rules() -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 3), (0xc6, 1, 4), (0xc7, 1, 1),
        (0xc8, 1, 2), (0xbe, 1, 2), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 3), (0x4a, 1, 3), (0x1c, 4, read_usize(KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION) as u32),
        (0x52, 1, 2), (0x20, 4, 0xffff_ffff), (0x49, 1, 7), (0xa7, 2, 0xffff)]
}

/// Rule: the Greek First Division: 16 clubs, 2 legs, 3 points a win and 1 a draw; tie-breaks goal difference, goals
/// for, wins; 2 go down to the Second Division; 7 substitutes, 3 substitutions; rule bit 0x40 (`gre_prm_rules`).
/// Original: `GrePrm::set_up_rules` (0x5eae90, gre_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwd_13 greek first division rules");
    apply_league_rules_with_flags(League(this), 0x40, &gre_prm_rules())
}

/// Rule: the Greek Second Division: as the First Division, 2 go up and 2 go down (`gre_second_rules`).
/// Original: `GreSecond::set_up_rules` (0x5ebde0, gre_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_second_set_up_rules(this: usize) -> u32 {
    covered!("mwd_13 greek second division rules");
    apply_league_rules_with_flags(League(this), 0x40, &gre_second_rules())
}

/// Rule: the Russian First Division: 2 legs, 3 points a win and 1 a draw; tie-breaks wins, head to head, goal
/// difference, goals for; 2 go up and 3 go down (`rus_first_rules`).
/// Original: `RusFirst::set_up_rules` (0x7ea810, rus_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_first_set_up_rules(this: usize) -> u32 {
    covered!("mwd_13 russian first division rules");
    eng_prm_apply_league_rules(League(this), &rus_first_rules())
}

/// Comparer for `gre_prm_set_up_rules` (the rehearsal of `eng_prm_compare_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_gre_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GRE_PRM_SET_UP_RULES);
    eng_prm_compare_league_rules("gre_prm_set_up_rules", League(this), gre_prm_rules(), || o(this), || gre_prm_set_up_rules(this))
}

/// Comparer for `gre_second_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_gre_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GRE_SECOND_SET_UP_RULES);
    eng_prm_compare_league_rules("gre_second_set_up_rules", League(this), gre_second_rules(), || o(this), || gre_second_set_up_rules(this))
}

/// Comparer for `rus_first_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_rus_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_RUS_FIRST_SET_UP_RULES);
    eng_prm_compare_league_rules("rus_first_set_up_rules", League(this), rus_first_rules(), || o(this), || rus_first_set_up_rules(this))
}
