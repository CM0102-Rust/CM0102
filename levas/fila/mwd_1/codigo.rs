// ---------- batch %%LEVA%%: the Croatian leagues (cro_a1.cpp, cro_a2a.cpp, cro_a2b.cpp): the constructors ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_tiebreak_league_ctor("construct_cro_a1", League(this), year, record, &CROATIAN_FIRST_DIVISION, || o(this, year, record))
}

/// Comparer for `construct_cro_a2a`.
pub(crate) unsafe extern "thiscall" fn compare_construct_cro_a2a(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    compare_tiebreak_league_ctor("construct_cro_a2a", League(this), year, record, &CROATIAN_SECOND_DIVISION_NORTH, || o(this, year, record))
}

/// Comparer for `construct_cro_a2b`.
pub(crate) unsafe extern "thiscall" fn compare_construct_cro_a2b(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    compare_tiebreak_league_ctor("construct_cro_a2b", League(this), year, record, &CROATIAN_SECOND_DIVISION_SOUTH, || o(this, year, record))
}
