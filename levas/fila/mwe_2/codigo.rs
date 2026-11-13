// ---------- batch %%LEVA%%: the constructors of the Norwegian, Swedish and Finnish First Divisions (nor_first.cpp, swe_first.cpp, fin_first.cpp) ----------

/// What changes from one constructor to another among the leagues with play-off stages after the league.
pub(crate) struct LeagueWithStages {
    /// The vtable, discipline rule, rules and table routines, source file and box lines (rules, table, fixtures,
    /// order, statistics); `low` is not used.
    pub(crate) division: LeagueDivision,
    /// How many stages besides the league (`+0x2c`); the vector at `+0xc` is malloc of 4 × that.
    pub(crate) stages: u32,
    /// The last step: None = League::set_predicted_positions; Some((a, b)) = two direct calls (thiscall, ret 0) on
    /// the league, in this order.
    pub(crate) finish: Option<(usize, usize)>,
}

/// The body of the three constructors (thiscall(this, year, record), ret 8; SEH frame only for the operator new of
/// the statistics).
///
/// League::League (`exe::new_league`), the record (`league::COMPETITION`), the vtable, the year (`.set_year()`,
/// word) and the discipline rule (`.set_discipline_rule()`); the saved state (`exe::league_load_from_temp_file`,
/// push 1 -> thiscall, ret 4) ends it. Otherwise the new league: stage index 0xff, current stage −1, `stages`
/// extra stages (`.set_stage_count()`) and their vector (`exe::crt_malloc` of 4 × stages, not checked); the rules
/// and the table (direct calls, thiscall, ret 0), `exe::league_create_fixtures`, `exe::league_sort_table`(0) and the
/// Comp_Stats (operator new of 0x5ce, `exe::new_comp_stats`) into `+8`; each step that fails shows the box of its
/// line and returns. Last, the predicted positions (`exe::league_set_predicted_positions`) or the two calls of
/// `finish`. Returns this.
pub(crate) unsafe fn construct_league_with_stages(this: League, year: u32, record: usize, p: &LeagueWithStages) -> usize {
    let d = &p.division;
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(d.rules);
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(d.table);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), d.vtabela);
    this.set_year((year as u16) as i16);
    this.set_discipline_rule(d.discipline);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!("mwe_2 new league with stages");
    this.set_stage_index(stage_index::MAIN);
    this.set_current_stage(0xffff_ffff);
    this.set_stage_count(p.stages as usize);
    let v = exe::crt_malloc(p.stages.wrapping_mul(4));
    this.set_stages(v);
    if rules(this.addr()) == 0 {
        show_error_at(d.file, d.rows[0]);
        return this.addr();
    }
    if table(this.addr()) == 0 {
        show_error_at(d.file, d.rows[1]);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(d.file, d.rows[2]);
        return this.addr();
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(d.file, d.rows[3]);
        return this.addr();
    }
    let e = exe::crt_operator_new(0x5ce);
    let s = if e != 0 { exe::new_comp_stats(e, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(d.file, d.rows[4]);
        return this.addr();
    }
    match p.finish {
        None => exe::league_set_predicted_positions(this),
        Some((a, b)) => {
            let a: unsafe extern "thiscall" fn(usize) = core::mem::transmute(a);
            let b: unsafe extern "thiscall" fn(usize) = core::mem::transmute(b);
            a(this.addr());
            b(this.addr());
        }
    }
    this.addr()
}

/// The Norwegian First Division: discipline rule 21, one stage after the league (the promotion play-off against the
/// Premier Division), nor_first.cpp (0xa69740) lines 0x49, 0x50, 0x57, 0x60, 0x67.
pub(crate) const NORWEGIAN_FIRST_DIVISION: LeagueWithStages = LeagueWithStages {
    division: LeagueDivision { vtabela: 0x96de20, discipline: discipline_rule::NORWAY_LEAGUE, rules: 0x790b00, table: 0x7909a0, file: 0xa69740, rows: [0x49, 0x50, 0x57, 0x60, 0x67], low: false },
    stages: 1, // mwe2: the Norwegian play-off stage
    finish: None,
};

/// The Swedish First Division (Superettan): discipline rule 28 (the Swedish league one), one stage after the league
/// (the promotion play-off), swe_first.cpp (0xa8b53c) lines 0x4a, 0x51, 0x58, 0x5f, 0x66.
pub(crate) const SWEDISH_FIRST_DIVISION: LeagueWithStages = LeagueWithStages {
    division: LeagueDivision { vtabela: 0x9700e4, discipline: discipline_rule::SWEDEN_LEAGUE, rules: 0x88f090, table: 0x88ef90, file: 0xa8b53c, rows: [0x4a, 0x51, 0x58, 0x5f, 0x66], low: false },
    stages: 1, // mwe2: the Swedish play-off stage
    finish: None,
};

/// The Finnish First Division (Ykkönen): discipline rule 10 (the Finnish league one), four stages after the North
/// group (the South group and the play-offs), fin_first.cpp (0x9c818c) lines 0x4a, 0x51, 0x58, 0x61, 0x68; it ends
/// with FinFirst::create_south_group_stage (0x594cd0) and FinFirst::set_predicted_positions (0x594ff0).
pub(crate) const FINNISH_FIRST_DIVISION: LeagueWithStages = LeagueWithStages {
    division: LeagueDivision { vtabela: 0x96a400, discipline: discipline_rule::FINLAND_LEAGUE, rules: 0x593b80, table: 0x593a70, file: 0x9c818c, rows: [0x4a, 0x51, 0x58, 0x61, 0x68], low: false },
    stages: 4,
    finish: Some((0x594cd0, 0x594ff0)), // mwe2: the Finnish South group and predicted positions
};

/// Builds the Norwegian First Division: discipline rule 21 (the Norwegian league one) and one stage after the league
/// (the promotion play-off); with no saved state, the rules, the table, the fixtures, the order, the stats and the
/// predicted positions.
///
/// `NorFirst::NorFirst(year, record)`: `construct_league_with_stages` with vtable 0x96de20, discipline byte 0x15, one
/// extra stage (malloc of 4), `NorFirst::set_up_rules` (0x790b00) and `NorFirst::create_table` (0x7909a0); the boxes of
/// nor_first.cpp lines 0x49, 0x50, 0x57, 0x60, 0x67; League::set_predicted_positions at the end.
/// Original: `NorFirst::NorFirst` (0x78fd40, nor_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_nor_first(this: usize, year: u32, record: usize) -> usize {
    construct_league_with_stages(League(this), year, record, &NORWEGIAN_FIRST_DIVISION)
}

/// Builds the Swedish First Division: discipline rule 28 (the Swedish league one) and one stage after the league (the
/// promotion play-off); with no saved state, the rules, the 16-club table, the fixtures, the order, the stats and the
/// predicted positions.
///
/// `SweFirst::SweFirst(year, record)`: `construct_league_with_stages` with vtable 0x9700e4, discipline byte 0x1c, one
/// extra stage, `SweFirst::set_up_rules` (0x88f090) and `SweFirst::create_table` (0x88ef90); the boxes of swe_first.cpp
/// lines 0x4a, 0x51, 0x58, 0x5f, 0x66; League::set_predicted_positions at the end.
/// Original: `SweFirst::SweFirst` (0x88e5e0, swe_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_swe_first(this: usize, year: u32, record: usize) -> usize {
    construct_league_with_stages(League(this), year, record, &SWEDISH_FIRST_DIVISION)
}

/// Builds the Finnish First Division: discipline rule 10 (the Finnish league one) and four stages after the North
/// group; with no saved state, the rules, the North group table, the fixtures, the order and the stats; then the South
/// group stage and the predicted positions of both groups.
///
/// `FinFirst::FinFirst(year, record)`: `construct_league_with_stages` with vtable 0x96a400, discipline byte 0xa, four
/// extra stages (malloc of 0x10), `FinFirst::set_up_rules` (0x593b80) and `FinFirst::create_table` (0x593a70); the
/// boxes of fin_first.cpp lines 0x4a, 0x51, 0x58, 0x61, 0x68; at the end FinFirst::create_south_group_stage (0x594cd0)
/// and FinFirst::set_predicted_positions (0x594ff0), direct calls (thiscall, ret 0), instead of
/// League::set_predicted_positions.
/// Original: `FinFirst::FinFirst` (0x592eb0, fin_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_fin_first(this: usize, year: u32, record: usize) -> usize {
    construct_league_with_stages(League(this), year, record, &FINNISH_FIRST_DIVISION)
}

/// The comparer of the three constructors: only the original runs (it allocates the league, the stage vector and the
/// statistics); checks the return value, the vtable, the year, the record, the discipline and, for a new league, the
/// stage count and its vector.
pub(crate) unsafe fn compare_league_with_stages_ctor(name: &str, this: League, year: u32, record: usize, p: &LeagueWithStages, original: impl FnOnce() -> usize) -> usize {
    let loaded = league_will_load_saved_state(record);
    let r = original();
    let d = &p.division;
    let mut ok = r == this.addr() && read_usize(this.addr()) == d.vtabela && (this.year() as u16) == year as u16
        && this.competition().addr() == record && this.discipline_rule() == d.discipline;
    if this.comp_stats() != 0 && !loaded {
        ok = ok && this.stage_count() == p.stages as usize && this.stages() != 0;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}) [year {} arg {}, record {:#x} arg {:#x}, discipline {}, stage index {}]: returned {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", name, this.addr(), this.year(), year as u16, this.competition().addr(), record, this.discipline_rule(), this.stage_index() as i32, r, read_usize(this.addr()), this.stage_count(), this.stages(), this.comp_stats()));
    r
}

/// Comparer for `construct_nor_first`.
pub(crate) unsafe extern "thiscall" fn compare_construct_nor_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_league_with_stages_ctor("construct_nor_first", League(this), year, record, &NORWEGIAN_FIRST_DIVISION, || o(this, year, record))
}

/// Comparer for `construct_swe_first`.
pub(crate) unsafe extern "thiscall" fn compare_construct_swe_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    compare_league_with_stages_ctor("construct_swe_first", League(this), year, record, &SWEDISH_FIRST_DIVISION, || o(this, year, record))
}

/// Comparer for `construct_fin_first`.
pub(crate) unsafe extern "thiscall" fn compare_construct_fin_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    compare_league_with_stages_ctor("construct_fin_first", League(this), year, record, &FINNISH_FIRST_DIVISION, || o(this, year, record))
}
