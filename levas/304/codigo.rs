// ---------- batch 304: the constructors of the Norwegian Premier Division and of the Portuguese Premier and Second Divisions (nor_prm.cpp, por_prm.cpp, por_second.cpp) ----------

/// The Norwegian Premier Division in the common division constructor: its vtable, discipline rule 21 (the Norwegian
/// league one), its rules (`nor_prm_set_up_rules`, 0x792700) and table (`nor_prm_create_table`, 0x7925e0), source
/// nor_prm.cpp (0xa6977c) and the box lines 0x45, 0x4c, 0x53, 0x5c, 0x63.
pub(crate) const NORWEGIAN_PREMIER_DIVISION: LeagueDivision = LeagueDivision {
    vtabela: 0x96ded4,
    discipline: discipline_rule::NORWAY_LEAGUE, // mwe1: the Norwegian league discipline rule
    rules: 0x792700,
    table: 0x7925e0,
    file: 0xa6977c,
    rows: [0x45, 0x4c, 0x53, 0x5c, 0x63],
    low: false,
};

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

/// Builds the Norwegian Premier Division: discipline rule 21 (the Norwegian league one), the league only (no
/// play-off stage); with no saved state, the rules, the table, the fixtures, the order, the stats and the predicted
/// positions.
///
/// `NorPrm::NorPrm(year, record)`: the body of `construct_division_league` (checked instruction by instruction against
/// the French and Spanish constructors: only the vtable 0x96ded4, the discipline byte +0x50 = 0x15, the two calls and
/// the source and lines change): League::League (`exe::new_league`), the record (`league::COMPETITION`), the vtable,
/// the year (`.set_year()`, word) and the discipline rule; the saved state (`exe::league_load_from_temp_file`, push 1)
/// ends it. Otherwise stage index 0xff, current stage −1, no extra stages (`+0x2c` = 0, `+0xc` = 0), then
/// `NorPrm::set_up_rules` and `NorPrm::create_table` (direct calls, thiscall, ret 0), `exe::league_create_fixtures`,
/// `exe::league_sort_table`(0), the Comp_Stats (operator new of 0x5ce and `exe::new_comp_stats`) into `+8` and
/// `exe::league_set_predicted_positions`; each step that fails shows the box of its line (nor_prm.cpp 0x45, 0x4c, 0x53,
/// 0x5c, 0x63) and returns. Returns this.
/// Original: `NorPrm::NorPrm` (0x791500, nor_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_nor_prm(this: usize, year: u32, record: usize) -> usize {
    covered!("mwe_1 norwegian premier division");
    construct_division_league(League(this), year, record, &NORWEGIAN_PREMIER_DIVISION)
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

/// Comparer for `construct_nor_prm`: only the original runs (it allocates the league), then the fields
/// (`compare_division_league_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_nor_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_NOR_PRM);
    compare_division_league_ctor("construct_nor_prm", League(this), year, record, &NORWEGIAN_PREMIER_DIVISION, || o(this, year, record))
}

/// Comparer for `construct_por_prm`.
pub(crate) unsafe extern "thiscall" fn compare_construct_por_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_POR_PRM);
    compare_division_league_ctor("construct_por_prm", League(this), year, record, &PORTUGUESE_PREMIER_DIVISION, || o(this, year, record))
}

/// Comparer for `construct_por_second`.
pub(crate) unsafe extern "thiscall" fn compare_construct_por_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_POR_SECOND);
    compare_division_league_ctor("construct_por_second", League(this), year, record, &PORTUGUESE_SECOND_DIVISION, || o(this, year, record))
}

// ---------- batch 304: the constructors of the Norwegian, Swedish and Finnish First Divisions (nor_first.cpp, swe_first.cpp, fin_first.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_NOR_FIRST);
    compare_league_with_stages_ctor("construct_nor_first", League(this), year, record, &NORWEGIAN_FIRST_DIVISION, || o(this, year, record))
}

/// Comparer for `construct_swe_first`.
pub(crate) unsafe extern "thiscall" fn compare_construct_swe_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SWE_FIRST);
    compare_league_with_stages_ctor("construct_swe_first", League(this), year, record, &SWEDISH_FIRST_DIVISION, || o(this, year, record))
}

/// Comparer for `construct_fin_first`.
pub(crate) unsafe extern "thiscall" fn compare_construct_fin_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_FIN_FIRST);
    compare_league_with_stages_ctor("construct_fin_first", League(this), year, record, &FINNISH_FIRST_DIVISION, || o(this, year, record))
}

// ---------- batch 304: the league rules of the Norwegian Premier and First Divisions and of the Swedish Premier Division (nor_prm.cpp, nor_first.cpp, swe_prm.cpp) ----------

/// The rule writes of the Norwegian Premier Division (Tippeligaen, 14 clubs), in the original's order.
///
/// Two legs (`league::LEGS`), 3 points for a win and 1 for a draw, league matches (category 2), no match rule flags
/// and no round draw rules, club sides; tie-breaks goal difference, goals for and code 3; nobody goes up (no division
/// above: −1), no promotion play-off, 1 club to the relegation play-off against the First Division (the 12th) and 2
/// relegated directly; the First Division below; 3 substitutions, the squad flags 2 and 7 substitutes named.
pub(crate) unsafe fn norwegian_premier_rules() -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 2), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0), (league::ROUND_DRAW_RULES, 2, 0), (league::CLUB_SIDES, 1, 1)];
    v.extend_from_slice(&[(league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 3)]);
    v.extend_from_slice(&[(league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0)]);
    v.push((league::RELEGATION_PLAYOFF_COUNT, 1, 1)); // mwe3: the Norwegian Premier relegation play-off place
    v.push((league::RELEGATED_COUNT, 1, 2)); // mwe3: the Norwegian Premier relegates 2
    v.extend_from_slice(&[(league::UPPER_DIVISION_ID, 4, 0xffff_ffff), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SQUAD_FLAGS, 1, 2)]);
    v.extend_from_slice(&[(league::LOWER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION) as u32), (league::SUBSTITUTES_NAMED, 1, 7)]);
    v
}

/// The rule writes of the Norwegian First Division (Adeccoligaen, 16 clubs; 14 when the database has only 14), in the
/// original's order.
///
/// Two legs, 3 points for a win and 1 for a draw, league matches, club sides; tie-breaks goal difference, goals for and
/// code 3; 2 promoted directly and 1 to the promotion play-off (the 3rd against the Premier Division's 12th), no
/// relegation play-off and 4 relegated (no division below: −1); the Premier Division above; 3 substitutions, the squad
/// flags 2 and 7 substitutes named. It does not touch the match rule flags nor the round draw rules.
pub(crate) unsafe fn norwegian_first_rules() -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 2), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::CLUB_SIDES, 1, 1)];
    v.extend_from_slice(&[(league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 3)]);
    v.push((league::PROMOTED_COUNT, 1, 2)); // mwe3: the Norwegian First Division promotes 2
    v.extend_from_slice(&[(league::PROMOTION_PLAYOFF_COUNT, 1, 1), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 4)]);
    v.extend_from_slice(&[(league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::UPPER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION) as u32), (league::SQUAD_FLAGS, 1, 2)]);
    v.extend_from_slice(&[(league::LOWER_DIVISION_ID, 4, 0xffff_ffff), (league::SUBSTITUTES_NAMED, 1, 7)]);
    v
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

/// Sets up the rules of the Norwegian Premier Division (14 clubs): two legs, 3 points for a win; nobody goes up, the
/// 12th goes to the relegation play-off and the bottom 2 go down to the First Division; 7 substitutes and 3
/// substitutions.
///
/// `NorPrm::set_up_rules()`: the writes of `norwegian_premier_rules` (legs, points, category, flags, tie-breaks 1, 2,
/// 3, the bands 0 / 0 / 1 / 2, no division above, the First Division below, substitutions, squad flags 2 and
/// substitutes) and the main-stage round table (virtual `comp_vtable::CREATE_SCHEDULE`: push 0; push &+0x3a; push
/// &+0xa9; push −1; mov ecx -> thiscall, ret 0x10) into `league::ROUND_DATES`. Returns 1.
/// Original: `NorPrm::set_up_rules` (0x792700, nor_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwe_3 norwegian premier rules");
    eng_prm_apply_league_rules(League(this), &norwegian_premier_rules())
}

/// Sets up the rules of the Norwegian First Division (16 clubs): two legs, 3 points for a win; the top 2 go up, the
/// 3rd plays the promotion play-off against the Premier Division's 12th and the bottom 4 go down; 7 substitutes and 3
/// substitutions.
///
/// `NorFirst::set_up_rules()`: the writes of `norwegian_first_rules` (the bands 2 / 1 / 0 / 4, the Premier Division
/// above, nothing below) and the main-stage round table (virtual `comp_vtable::CREATE_SCHEDULE`) into
/// `league::ROUND_DATES`. Returns 1.
/// Original: `NorFirst::set_up_rules` (0x790b00, nor_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_first_set_up_rules(this: usize) -> u32 {
    covered!("mwe_3 norwegian first division rules");
    eng_prm_apply_league_rules(League(this), &norwegian_first_rules())
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

/// Comparer for `nor_prm_set_up_rules`: the dry run of `eng_prm_compare_league_rules` (the original builds the round
/// table; the copy writes the rules only).
pub(crate) unsafe extern "thiscall" fn compare_nor_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NOR_PRM_SET_UP_RULES);
    eng_prm_compare_league_rules("nor_prm_set_up_rules", League(this), norwegian_premier_rules(), || o(this), || nor_prm_set_up_rules(this))
}

/// Comparer for `nor_first_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_nor_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NOR_FIRST_SET_UP_RULES);
    eng_prm_compare_league_rules("nor_first_set_up_rules", League(this), norwegian_first_rules(), || o(this), || nor_first_set_up_rules(this))
}

/// Comparer for `swe_prm_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_swe_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SWE_PRM_SET_UP_RULES);
    eng_prm_compare_league_rules("swe_prm_set_up_rules", League(this), swedish_premier_rules(), || o(this), || swe_prm_set_up_rules(this))
}

// ---------- batch 304: the league rules of the Swedish First Division and of the Portuguese Premier and Second Divisions (swe_first.cpp, por_prm.cpp, por_second.cpp) ----------

/// The division below when the nation has its lower leagues playable: the nation of the league's competition
/// (`competition::NATION`) with bit 4 of `.league_selected()` gives the id of global `below`, otherwise −1.
pub(crate) unsafe fn lower_division_when_playable(this: usize, below: usize) -> u32 {
    let nation = Nation(read_usize(read_usize(this + league::COMPETITION) + competition::NATION));
    if nation.league_selected() & 4 != 0 { read_usize(below) as u32 } else { 0xffff_ffff }
}

/// The rule writes of the Swedish First Division (Superettan, 16 clubs), in the original's order.
///
/// Two legs, 3 points for a win and 1 for a draw, league matches, no match rule flags and no round draw rules, club
/// sides; tie-breaks goal difference and goals for (the third code 0 ends the list); 2 promoted directly, 1 to the
/// promotion play-off (the 3rd, against the Allsvenskan's 12th), no relegation play-off and 3 relegated; the
/// Allsvenskan above and the Second Division below when the Swedish lower leagues are playable (bit 4), otherwise −1;
/// 3 substitutions, the squad flags 2 and 5 substitutes named.
pub(crate) unsafe fn swedish_first_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 2), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0), (league::ROUND_DRAW_RULES, 2, 0), (league::CLUB_SIDES, 1, 1)];
    v.extend_from_slice(&[(league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 0)]);
    v.extend_from_slice(&[(league::PROMOTED_COUNT, 1, 2), (league::PROMOTION_PLAYOFF_COUNT, 1, 1), (league::RELEGATION_PLAYOFF_COUNT, 1, 0)]);
    v.push((league::RELEGATED_COUNT, 1, 3)); // mwe4: the Superettan relegates 3
    v.extend_from_slice(&[(league::UPPER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION) as u32), (league::LOWER_DIVISION_ID, 4, lower_division_when_playable(this, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION))]);
    v.extend_from_slice(&[(league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SQUAD_FLAGS, 1, 2), (league::SUBSTITUTES_NAMED, 1, 5)]);
    v
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

/// Sets up the rules of the Swedish First Division (Superettan, 16 clubs): two legs, 3 points for a win, tie-breaks
/// goal difference and goals for; the top 2 go up, the 3rd plays the promotion play-off against the Allsvenskan's 12th
/// and the bottom 3 go down (to the Second Division when it is playable); 5 substitutes and 3 substitutions.
///
/// `SweFirst::set_up_rules()`: the writes of `swedish_first_rules` and the main-stage round table (virtual
/// `comp_vtable::CREATE_SCHEDULE`: push 0; push &+0x3a; push &+0xa9; push −1; mov ecx -> thiscall, ret 0x10) into
/// `league::ROUND_DATES`. Returns 1.
/// Original: `SweFirst::set_up_rules` (0x88f090, swe_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_first_set_up_rules(this: usize) -> u32 {
    covered!("mwe_4 superettan rules");
    eng_prm_apply_league_rules(League(this), &swedish_first_rules(this))
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

/// Comparer for `swe_first_set_up_rules`: the dry run of `eng_prm_compare_league_rules`.
pub(crate) unsafe extern "thiscall" fn compare_swe_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SWE_FIRST_SET_UP_RULES);
    eng_prm_compare_league_rules("swe_first_set_up_rules", League(this), swedish_first_rules(this), || o(this), || swe_first_set_up_rules(this))
}

/// Comparer for `por_prm_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_por_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POR_PRM_SET_UP_RULES);
    eng_prm_compare_league_rules("por_prm_set_up_rules", League(this), portuguese_premier_rules(), || o(this), || por_prm_set_up_rules(this))
}

/// Comparer for `por_second_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_por_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POR_SECOND_SET_UP_RULES);
    eng_prm_compare_league_rules("por_second_set_up_rules", League(this), portuguese_second_rules(this), || o(this), || por_second_set_up_rules(this))
}

// ---------- batch 304: the Allsvenskan constructor and the league rules of the two Finnish divisions (swe_prm.cpp, fin_prm.cpp, fin_first.cpp) ----------

/// The six regional sections of the Swedish Second Division (Norrland, East and West Svealand, East, West and South
/// Götaland), in the original's comparison order.
pub(crate) const SWEDISH_SECOND_DIVISION_SECTIONS: [usize; 6] = [KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_NORTH, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_EAST_SVEALAND,
    KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_WEST_SVEALAND, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_EAST_GOTALAND, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_WEST_GOTALAND,
    KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_SOUTH_GOTALAND];

/// Where the Allsvenskan constructor files club `c`: Some((new division, new reserve division)) or None (untouched).
///
/// A club in one of the six Second Division sections keeps the section in `club::RESERVE_DIVISION` and goes into the
/// Second Division (`KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION`); a Swedish club (`club::NATION` with the id of
/// `KEY_NATION_SWEDEN`) outside the Allsvenskan, the Superettan and the Second Division goes into the lower division
/// (`KEY_CLUB_COMP_SWEDISH_LOWER_DIVISION`); a club without division, or without nation, is untouched.
pub(crate) unsafe fn swedish_club_filing(c: usize) -> Option<(usize, Option<usize>)> {
    let comp = read_usize(c + club::DIVISION);
    if comp == 0 {
        return None;
    }
    let id = read_usize(comp);
    if SWEDISH_SECOND_DIVISION_SECTIONS.iter().any(|&g| read_usize(g) == id) {
        return Some((comp_record_of(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION), Some(comp))); // mwe5: a section club goes into the Second Division
    }
    let nation = read_usize(c + club::NATION);
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

/// A direct thiscall with no arguments on the league (set_up_rules, create_table), returning eax.
pub(crate) unsafe fn swe_prm_call(f: usize, this: League) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(f);
    f(this.addr())
}

/// The rule writes of the Finnish Premier Division (Veikkausliiga, 12 clubs), in the original's order.
///
/// Three legs (33 rounds), 3 points for a win and 1 for a draw, league matches, no match rule flags and no round draw
/// rules, club sides; tie-breaks goal difference, goals for and code 3; nobody goes up, no promotion play-off, 1 club
/// to the relegation play-off (the 11th, against the First Division) and 1 relegated; `league::LEAGUE_FLAGS` = 3 (the
/// champion status and promotion status 0); 7 substitutes named, 3 substitutions, no division above, the First
/// Division below and the squad flags 2.
pub(crate) unsafe fn finnish_premier_rules() -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 3)];
    v.extend_from_slice(&[(league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0), (league::ROUND_DRAW_RULES, 2, 0), (league::CLUB_SIDES, 1, 1)]);
    v.extend_from_slice(&[(league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 3)]);
    v.extend_from_slice(&[(league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 1)]);
    v.push((league::RELEGATED_COUNT, 1, 1)); // mwe5: the Veikkausliiga relegates 1
    v.extend_from_slice(&[(league::LEAGUE_FLAGS, 2, 3), (league::SUBSTITUTES_NAMED, 1, 7), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::UPPER_DIVISION_ID, 4, 0xffff_ffff)]);
    v.extend_from_slice(&[(league::LOWER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION) as u32), (league::SQUAD_FLAGS, 1, 2)]);
    v
}

/// The rule writes of the Finnish First Division (Ykkönen, the North group of 10 in the league stage), in the
/// original's order.
///
/// Three legs, 3 points for a win and 1 for a draw, league matches, club sides; tie-breaks goal difference, goals for
/// and code 3; 2 promoted, no promotion play-off, 1 to the relegation play-off and 2 relegated (no division below:
/// −1); 3 substitutions, the Premier Division above, the squad flags 2, `league::LEAGUE_FLAGS` = 8 (promotion status
/// 6) and 7 substitutes named. It does not touch the match rule flags nor the round draw rules.
pub(crate) unsafe fn finnish_first_rules() -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 3), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::CLUB_SIDES, 1, 1)];
    v.extend_from_slice(&[(league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 3)]);
    v.push((league::PROMOTED_COUNT, 1, 2)); // mwe5: the Ykkonen promotes 2
    v.extend_from_slice(&[(league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 1), (league::RELEGATED_COUNT, 1, 2)]);
    v.extend_from_slice(&[(league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::UPPER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION) as u32), (league::SQUAD_FLAGS, 1, 2)]);
    v.extend_from_slice(&[(league::LOWER_DIVISION_ID, 4, 0xffff_ffff), (league::LEAGUE_FLAGS, 2, 8), (league::SUBSTITUTES_NAMED, 1, 7)]);
    v
}

/// Sets up the rules of the Finnish Premier Division (Veikkausliiga, 12 clubs): three legs (33 rounds), 3 points for a
/// win; nobody goes up, the 11th plays the relegation play-off and the 12th goes down to the Ykkönen; 7 substitutes and
/// 3 substitutions.
///
/// `FinPrm::set_up_rules()`: the writes of `finnish_premier_rules` and the main-stage round table (virtual
/// `comp_vtable::CREATE_SCHEDULE`: push 0; push &+0x3a; push &+0xa9; push −1; mov ecx -> thiscall, ret 0x10) into
/// `league::ROUND_DATES`. Returns 1.
/// Original: `FinPrm::set_up_rules` (0x595c90, fin_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwe_5 veikkausliiga rules");
    eng_prm_apply_league_rules(League(this), &finnish_premier_rules())
}

/// Sets up the rules of the Finnish First Division (Ykkönen): three legs, 3 points for a win; 2 go up, 1 plays the
/// relegation play-off and 2 go down; 7 substitutes and 3 substitutions.
///
/// `FinFirst::set_up_rules()`: the writes of `finnish_first_rules` and the main-stage round table (virtual
/// `comp_vtable::CREATE_SCHEDULE`) into `league::ROUND_DATES`. Returns 1.
/// Original: `FinFirst::set_up_rules` (0x593b80, fin_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_first_set_up_rules(this: usize) -> u32 {
    covered!("mwe_5 ykkonen rules");
    eng_prm_apply_league_rules(League(this), &finnish_first_rules())
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

/// Comparer for `fin_prm_set_up_rules`: the dry run of `eng_prm_compare_league_rules`.
pub(crate) unsafe extern "thiscall" fn compare_fin_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FIN_PRM_SET_UP_RULES);
    eng_prm_compare_league_rules("fin_prm_set_up_rules", League(this), finnish_premier_rules(), || o(this), || fin_prm_set_up_rules(this))
}

/// Comparer for `fin_first_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_fin_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FIN_FIRST_SET_UP_RULES);
    eng_prm_compare_league_rules("fin_first_set_up_rules", League(this), finnish_first_rules(), || o(this), || fin_first_set_up_rules(this))
}

// ---------- batch 304: the tables of the Norwegian, Swedish and Finnish Premier Divisions (nor_prm.cpp, swe_prm.cpp, fin_prm.cpp) ----------

/// Creates the Norwegian Premier Division table: 14 clubs, those of the division in database order; the prize fund
/// by a share of 50,000 per place; no TV money.
///
/// `NorPrm::create_table()`: the body of `eng_prm_fill_league_table` (checked instruction by instruction):
/// `.set_club_count()` = 14, the table (`exe::crt_malloc` of 14 × 0x3b; out of memory: the box of nor_prm.cpp
/// (0xa6977c) line 0x1a7 and 0); the clubs whose division (`club::DIVISION`) has the id of
/// `KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION`, in database order, each into the next row (`exe::league_init_table_row`:
/// push 0; push 0; push club; push row -> thiscall, ret 0x10) until full; not full: 0. Full: the prize fund
/// (`exe::league_calc_prize_fund`, push 50,000), `.set_prize_mode()` = 1 and 1.
/// Original: `NorPrm::create_table` (0x7925e0, nor_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_prm_create_table(this: usize) -> u32 {
    covered!("mwe_6 tippeligaen table");
    eng_prm_fill_league_table(League(this), 0xe, 0xa6977c, 0x1a7, KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION, 0xc350, || ()) // mwe6: the Tippeligaen prize share
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

/// Creates the Finnish Premier Division table: 12 clubs, those of the division in database order; the prize fund by a
/// share of 35,000 per place; no TV money.
///
/// `FinPrm::create_table()`: the same body (`eng_prm_fill_league_table`) with 12 rows,
/// `KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION`, the box of fin_prm.cpp (0x9c8254) line 0x157 and the share 35,000.
/// Original: `FinPrm::create_table` (0x595b70, fin_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_prm_create_table(this: usize) -> u32 {
    covered!("mwe_6 veikkausliiga table");
    eng_prm_fill_league_table(League(this), 0xc, 0x9c8254, 0x157, KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION, 0x88b8, || ()) // mwe6: the Veikkausliiga prize share
}

/// Comparer for `nor_prm_create_table`: only the original runs (it allocates the table), against the plan of
/// `eng_prm_compare_league_table` (the division's clubs in database order, the count, the return value and the prize
/// fund).
pub(crate) unsafe extern "thiscall" fn compare_nor_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NOR_PRM_CREATE_TABLE);
    eng_prm_compare_league_table("nor_prm_create_table", League(this), 0xe, KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION, 0xc350, || o(this))
}

/// Comparer for `swe_prm_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_swe_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SWE_PRM_CREATE_TABLE);
    eng_prm_compare_league_table("swe_prm_create_table", League(this), 0xe, KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION, 0xc350, || o(this))
}

/// Comparer for `fin_prm_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_fin_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FIN_PRM_CREATE_TABLE);
    eng_prm_compare_league_table("fin_prm_create_table", League(this), 0xc, KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION, 0x88b8, || o(this))
}

// ---------- batch 304: the tables of the Swedish, Finnish and Norwegian First Divisions (swe_first.cpp, fin_first.cpp, nor_first.cpp) ----------

/// Allocates a league table of `bytes` bytes into `league::TABLE`; out of memory, the box of `file` at `row` and
/// false.
pub(crate) unsafe fn allocate_first_division_table(this: League, bytes: u32, file: usize, row: u32) -> bool {
    let t = exe::crt_malloc(bytes);
    this.set_table(t);
    if t == 0 {
        show_error_at(file, row);
        return false;
    }
    true
}

/// Fills the table rows with the clubs that `belongs` accepts, in database order, until `.club_count()` rows; returns
/// whether it filled.
///
/// The clubs of `G_CLUB_TABLE` (`club::SIZE` bytes, `G_CLUB_COUNT` of them, a signed word counter), each accepted one
/// into the next row (`exe::league_init_table_row`: push 0; push 0; push club; push row -> thiscall, ret 0x10; the row
/// is a byte), until the signed byte count equals `.club_count()`.
pub(crate) unsafe fn fill_first_division_rows(this: League, belongs: impl Fn(usize) -> bool) -> bool {
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut c = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if belongs(c) {
            exe::league_init_table_row(this, k as u32, c, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == this.club_count() {
                break;
            }
        }
        i = i.wrapping_add(1);
        c = c.wrapping_add(club::SIZE);
    }
    k as i8 as i16 == this.club_count()
}

/// True if the club's division (`club::DIVISION`) is the competition of id `g`.
pub(crate) unsafe fn club_in_division(c: usize, g: usize) -> bool {
    let comp = read_usize(c + club::DIVISION);
    comp != 0 && read_usize(comp) == read_usize(g)
}

/// True if the club is in the Finnish First Division and its reserve division (`club::RESERVE_DIVISION`, here the
/// regional group) is the North group (`KEY_CLUB_COMP_FINNISH_FIRST_DIVISION_NORTH`); a club without either is out.
pub(crate) unsafe fn club_in_finnish_north_group(c: usize) -> bool {
    let comp = read_usize(c + club::DIVISION);
    let group = read_usize(c + club::RESERVE_DIVISION);
    comp != 0 && group != 0 && read_usize(comp) == read_usize(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION) && read_usize(group) == read_usize(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION_NORTH)
}

/// How many clubs of the Norwegian First Division the database has (a byte counter over all the clubs).
pub(crate) unsafe fn norwegian_first_division_count() -> u8 {
    let mut n: u8 = 0;
    let mut i: i16 = 0;
    let mut c = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if club_in_division(c, KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION) {
            n = n.wrapping_add(1);
        }
        i = i.wrapping_add(1);
        c = c.wrapping_add(club::SIZE);
    }
    n
}

/// Creates the Swedish First Division (Superettan) table: 16 clubs, those of the division in database order; no prize
/// fund nor TV money.
///
/// `SweFirst::create_table()`: `.set_club_count()` = 16, the table (`exe::crt_malloc` of 0x3b0; out of memory: the box
/// of swe_first.cpp (0xa8b53c) line 0x144 and 0), the clubs of `KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION` in database order
/// (`fill_first_division_rows`); returns 1 if it filled, otherwise 0.
/// Original: `SweFirst::create_table` (0x88ef90, swe_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_first_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0x10);
    if !allocate_first_division_table(this, 0x3b0, 0xa8b53c, 0x144) {
        return 0;
    }
    covered!("mwe_7 superettan table");
    fill_first_division_rows(this, |c| club_in_division(c, KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION)) as u32 // mwe7: the Superettan clubs
}

/// Creates the Finnish First Division table of the league stage: the North group, 10 clubs (those of the division
/// whose regional group is the North one, in database order); no prize fund nor TV money.
///
/// `FinFirst::create_table()`: `.set_club_count()` = 10, the table (`exe::crt_malloc` of 0x24e; out of memory: the box
/// of fin_first.cpp (0x9c818c) line 0x1ad and 0), the clubs of `club_in_finnish_north_group` in database order;
/// returns 1 if it filled, otherwise 0. (The South group is a separate stage, FinFirst::create_south_group_stage.)
/// Original: `FinFirst::create_table` (0x593a70, fin_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_first_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0xa);
    if !allocate_first_division_table(this, 0x24e, 0x9c818c, 0x1ad) {
        return 0;
    }
    covered!("mwe_7 ykkonen north table");
    fill_first_division_rows(this, |c| club_in_finnish_north_group(c)) as u32 // mwe7: the North group clubs
}

/// Creates the Norwegian First Division table: 16 clubs; when the database has exactly 14 clubs in the division, 14
/// clubs and only 2 relegated; those of the division in database order; no prize fund nor TV money.
///
/// `NorFirst::create_table()`: `.set_club_count()` = 16 and the table (`exe::crt_malloc` of 0x3b0, the size of 16
/// rows in any case; out of memory: the box of nor_first.cpp (0xa69740) line 0x251 and 0); the division's clubs counted
/// over the whole database (`norwegian_first_division_count`, a byte); with exactly 14: `.set_club_count()` = 14,
/// `league::RELEGATED_COUNT` = 2 and `league::GAMES_PER_CLUB` = −1 (to be recomputed); then the clubs of
/// `KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION` in database order; returns 1 if it filled, otherwise 0.
/// Original: `NorFirst::create_table` (0x7909a0, nor_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_first_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0x10);
    if !allocate_first_division_table(this, 0x3b0, 0xa69740, 0x251) {
        return 0;
    }
    if norwegian_first_division_count() == 0xe { // mwe7: the 14-club Norwegian First Division
        covered!("mwe_7 norwegian first division of 14");
        this.set_club_count(0xe);
        write_u8(this.addr() + league::RELEGATED_COUNT, 2);
        write_u16(this.addr() + league::GAMES_PER_CLUB, 0xffff);
    }
    covered!("mwe_7 norwegian first division table");
    fill_first_division_rows(this, |c| club_in_division(c, KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION)) as u32
}

/// The comparer of the three tables: only the original runs (it allocates the table); checked against a plan made
/// beforehand.
///
/// The plan: the accepted clubs in database order, up to `n`; afterwards the count `.club_count()` = `n`, the return
/// value 1 if the plan filled (otherwise 0) and the rows with the planned clubs, in order.
pub(crate) unsafe fn compare_first_division_table(name: &str, this: League, n: i16, belongs: impl Fn(usize) -> bool, original: impl FnOnce() -> u32) -> u32 {
    let mut plan: Vec<usize> = Vec::new();
    let mut c = read_usize(G_CLUB_TABLE);
    for _ in 0..(read_usize(G_CLUB_COUNT) as i32).max(0) {
        if (plan.len() as i16) < n && belongs(c) {
            plan.push(c);
        }
        c += club::SIZE;
    }
    let r = original();
    let t = this.table();
    let mut found: Vec<usize> = Vec::new();
    let mut same = r == (plan.len() as i16 == n) as u32 && this.club_count() == n && t != 0;
    if same {
        found = (0..plan.len()).map(|k| read_usize(t + k * league_table_row::SIZE)).collect();
        same = found == plan;
    }
    SCOREBOARD.record(same, || format!("{}({:#x}) original={} n={} plan={:x?} table={:x?}", name, this.addr(), r, this.club_count(), plan, found));
    r
}

/// Comparer for `swe_first_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_swe_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SWE_FIRST_CREATE_TABLE);
    compare_first_division_table("swe_first_create_table", League(this), 0x10, |c| club_in_division(c, KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION), || o(this))
}

/// Comparer for `fin_first_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_fin_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FIN_FIRST_CREATE_TABLE);
    compare_first_division_table("fin_first_create_table", League(this), 0xa, |c| club_in_finnish_north_group(c), || o(this))
}

/// Comparer for `nor_first_create_table`: the size by the count of the division's clubs (14 when exactly 14,
/// otherwise 16), and with 14 also `league::RELEGATED_COUNT` = 2.
pub(crate) unsafe extern "thiscall" fn compare_nor_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NOR_FIRST_CREATE_TABLE);
    let fourteen = norwegian_first_division_count() == 0xe;
    let r = compare_first_division_table("nor_first_create_table", League(this), if fourteen { 0xe } else { 0x10 }, |c| club_in_division(c, KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION), || o(this));
    if fourteen {
        let ok = read_u8(this + league::RELEGATED_COUNT) == 2;
        SCOREBOARD.record(ok, || format!("nor_first_create_table({:#x}) 14 clubs, relegated {}", this, read_u8(this + league::RELEGATED_COUNT)));
    }
    r
}

// ---------- batch 304: the Portuguese tables and the Liga de Honra turnover (por_prm.cpp, por_second.cpp) ----------

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

/// Comparer for `por_prm_create_table`: only the original runs (it allocates and pays), against the plan of
/// `eng_prm_compare_league_table`.
pub(crate) unsafe extern "thiscall" fn compare_por_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POR_PRM_CREATE_TABLE);
    eng_prm_compare_league_table("por_prm_create_table", League(this), 0x12, KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION, 0x186a0, || o(this))
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

// ---------- batch 304: the season turnovers of the Swedish, Norwegian and Finnish First Divisions (swe_first.cpp, nor_first.cpp, fin_first.cpp) ----------

/// The Superettan turnover: archives the season in the statistics, clears the table, the round dates and the stages,
/// moves to the next year and rebuilds the rules, the 16-club table, the fixtures and the order; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// `SweFirst::start_new_season()`: `league::SEASON_CLOSED` = 0, the body of `rebuild_season` (checked instruction by
/// instruction; the table is `SweFirst::create_table` 0x88ef90 and the boxes of swe_first.cpp (0xa8b53c) at 0x1aa,
/// 0x1b1, 0x1b7, 0x1be) and `set_expectations_and_renew_officials`, whose eax is returned. After a box: 0.
/// Original: `SweFirst::start_new_season` (0x88f150, swe_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x88ef90, 0xa8b53c, [0x1aa, 0x1b1, 0x1b7, 0x1be], 1) { // mwe9: the Superettan next year
        return 0;
    }
    covered!("mwe_9 superettan turnover");
    set_expectations_and_renew_officials(this)
}

/// The Norwegian First Division turnover: the same as the Superettan's, with `NorFirst::create_table` (0x7909a0) and the
/// boxes of nor_first.cpp (0xa69740) at 0x2c9, 0x2d0, 0x2d6, 0x2dd.
/// Original: `NorFirst::start_new_season` (0x790b90, nor_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x7909a0, 0xa69740, [0x2c9, 0x2d0, 0x2d6, 0x2dd], 1) { // mwe9: the Norwegian First Division next year
        return 0;
    }
    covered!("mwe_9 norwegian first division turnover");
    set_expectations_and_renew_officials(this)
}

/// The Ykkönen turnover: archives the season, clears the table, the round dates and the stages (and their vector, which
/// is allocated again), moves to the next year and rebuilds the rules, the North group table, the fixtures and the
/// order; then the board expectations, the referee renewal, the South group stage and the predicted positions.
///
/// `FinFirst::start_new_season()`: `league::SEASON_CLOSED` = 0, `exe::league_free_table`(0), the round dates freed
/// and zeroed, `exe::comp_stats_archive_season_and_reset`(1) when there are statistics, the stages 0..=current (when
/// current ≥ 0) destroyed by their virtual 0 with 1; the current stage −1, the stage vector freed (`exe::crt_free`),
/// the year + 1 and a new vector of 4 × `.stage_count()` bytes (`exe::crt_malloc`; 0 when there are no stages). Then,
/// each with the box of fin_first.cpp (0x9c818c) when it returns 0 (lines 0x21b, 0x222, 0x228, 0x22f, and 0 is
/// returned): the rules (virtual `comp_vtable::SET_UP_RULES`), `FinFirst::create_table` (0x593a70),
/// `exe::league_create_fixtures` and `exe::league_sort_table`(0). Then League::set_club_season_expectations,
/// OfficialsManager::renew_officials(record) on `G_OFFICIALS_MANAGER`, FinFirst::create_south_group_stage (0x594cd0,
/// thiscall, ret 0) and the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax is
/// returned.
/// Original: `FinFirst::start_new_season` (0x593c20, fin_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
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
    if (this.current_stage() as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(this.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    exe::crt_free(this.stages());
    this.set_year(this.year().wrapping_add(1)); // mwe9: the Ykkonen next year
    let n = this.stage_count() as i32;
    let v = if n > 0 { exe::crt_malloc((n as u32).wrapping_mul(4)) } else { 0 };
    this.set_stages(v);
    let file = 0x9c818c;
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, 0x21b);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x593a70usize);
    if table(this.addr()) == 0 {
        show_error_at(file, 0x222);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, 0x228);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, 0x22f);
        return 0;
    }
    covered!("mwe_9 ykkonen turnover");
    let expectations: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SET_CLUB_SEASON_EXPECTATIONS);
    expectations(this.addr());
    exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), this.competition().addr());
    let south: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x594cd0usize);
    south(this.addr());
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS)
}

/// Comparer for `swe_first_start_new_season`: only the original runs (it frees and allocates); the year must go up by
/// one (`compare_state_league_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_swe_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SWE_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("swe_first_start_new_season", League(this), || o(this))
}

/// Comparer for `nor_first_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_nor_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NOR_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("nor_first_start_new_season", League(this), || o(this))
}

/// Comparer for `fin_first_start_new_season`: the year + 1, and the stage vector allocated again when there are
/// stages.
pub(crate) unsafe extern "thiscall" fn compare_fin_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FIN_FIRST_START_NEW_SEASON);
    let r = compare_state_league_new_season("fin_first_start_new_season", League(this), || o(this));
    let l = League(this);
    let ok = (l.stages() != 0) == (l.stage_count() as i32 > 0);
    SCOREBOARD.record(ok, || format!("fin_first_start_new_season({:#x}) stages {} vector {:#x}", this, l.stage_count(), l.stages()));
    r
}

// ---------- batch 304: the stage advance of the Swedish and Norwegian First Divisions and the Superettan promotion play-off (swe_first.cpp, nor_first.cpp) ----------

/// The Premier Division object of key `premier` (`G_COMP_OBJECT_TABLE` by its id) is complete: virtual
/// `comp_vtable::CHECK_COMPLETE` with (0, 1) (push 1; push 0; mov ecx -> thiscall, ret 8; the byte in al).
pub(crate) unsafe fn premier_division_complete(premier: usize) -> bool {
    let p = comp_by_global(premier);
    let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(p) + comp_vtable::CHECK_COMPLETE));
    complete(p, 0, 1) != 0
}

/// Brings the table positions and statuses of the Premier Division of key `premier` up to date: virtual
/// `league_vtable::UPDATE_POSITIONS_AND_STATUS` (mov ecx -> thiscall, ret 0).
pub(crate) unsafe fn update_premier_division_status(premier: usize) {
    let p = comp_by_global(premier);
    let update: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(p) + league_vtable::UPDATE_POSITIONS_AND_STATUS));
    update(p);
}

/// The stage advance of a First Division whose promotion play-off is against the Premier Division's play-off club.
///
/// At the last stage (`.current_stage()` ≥ `.stage_count()` − 1, signed), the error box of `file` at `row` and
/// nothing else. Otherwise, only when the Premier Division (key `premier`) is complete
/// (`premier_division_complete`): its positions and statuses are updated (`update_premier_division_status`), the
/// current stage goes up by one and, on reaching 0, the play-off is built (`play_off`, a direct call, thiscall, ret
/// 0). With the Premier Division still playing, nothing (the league asks again later).
pub(crate) unsafe fn advance_first_division_stage(this: League, file: usize, row: u32, premier: usize, play_off: usize) {
    if (this.current_stage() as i32) >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(file, row);
    }
    if !premier_division_complete(premier) { // mwe10: waits for the Premier Division
        return;
    }
    update_premier_division_status(premier); // mwe10: the Premier Division statuses brought up to date
    let stage = (this.current_stage() as i32).wrapping_add(1);
    this.set_current_stage(stage as usize);
    if stage == 0 {
        covered!("mwe_10 first division play-off reached");
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(play_off);
        f(this.addr());
    }
}

/// Moves the Superettan to the next stage once the Allsvenskan is complete: the Allsvenskan's statuses are updated and,
/// on reaching stage 0, the promotion play-off is built; at the last stage, the error box (line 0x1da).
///
/// `SweFirst::advance_stage()`: `advance_first_division_stage` with swe_first.cpp (0xa8b53c), line 0x1da, the key
/// `KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION` and `SweFirst::create_play_off_stage` (0x88f3e0).
/// Original: `SweFirst::advance_stage` (0x88f310, swe_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_first_advance_stage(this: usize) {
    advance_first_division_stage(League(this), 0xa8b53c, 0x1da, KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION, 0x88f3e0);
}

/// Moves the Norwegian First Division to the next stage once the Tippeligaen is complete: the same as the Superettan's
/// (nor_first.cpp line 0x2fa, `KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION`, `NorFirst::create_play_off_stage` 0x790e20).
/// Original: `NorFirst::advance_stage` (0x790d50, nor_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_first_advance_stage(this: usize) {
    advance_first_division_stage(League(this), 0xa69740, 0x2fa, KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION, 0x790e20);
}

/// Builds the Cup_Stage of the play-off between the two clubs of `list` into the first stage slot and frees the list
/// and the round table; returns the stage left in the slot.
///
/// The round table of stage 0 (virtual `comp_vtable::CREATE_SCHEDULE`: push 0; push &extra; push &rounds; push 0;
/// mov ecx -> thiscall, ret 0x10); the stage (operator new `exe::crt_operator_new` of 0xb2; Cup_Stage::Cup_Stage
/// `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret 0x3c: the league, 2, the list, the rounds, the competition, the round
/// table, the year, 0, 1, the extra, 0x14, 0, 0, 0, 0; 0 when out of memory) into the first slot of `.stages()`; the
/// list and the round table freed (`exe::crt_free`).
pub(crate) unsafe fn build_two_club_play_off(this: League, list: usize) -> usize {
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 0, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    // simplification: the original pushes the rounds, the extra and the year as dwords with garbage above the word;
    // the constructor only reads the words.
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 2, list, rounds as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(this.stages(), stage);
    exe::crt_free(list);
    exe::crt_free(t);
    read_usize(this.stages())
}

/// The club of the first table row (in table order, a signed word count) of `league` whose status is `status`.
pub(crate) unsafe fn first_row_club_with_status(league: League, status: u8) -> Option<usize> {
    let t = league.table();
    (0..(league.club_count() as i32).max(0) as usize).map(|i| t + i * league_table_row::SIZE).find(|&r| read_u8(r + league_table_row::STATUS) == status).map(|r| read_usize(r))
}

/// The club of the first row of the Premier Division of key `premier` (through League::get_table_row
/// `exe::league_get_table_row`, thiscall, ret 4) whose status is `status`.
pub(crate) unsafe fn premier_row_club_with_status(premier: usize, status: u8) -> Option<usize> {
    let p = League(comp_by_global(premier));
    for i in 0..(p.club_count() as i32).max(0) {
        if read_u8(exe::league_get_table_row(p, i) + league_table_row::STATUS) == status {
            return Some(read_usize(exe::league_get_table_row(p, i)));
        }
    }
    None
}

/// Builds the Superettan promotion play-off (stage 0): the Superettan club with the promotion play-off status (the
/// 3rd) against the Allsvenskan club with the relegation play-off status (the 12th), as a two-legged cup tie.
///
/// `SweFirst::create_play_off_stage()`: the list of 2 (`exe::crt_malloc` of 8; out of memory: the box of
/// swe_first.cpp (0xa8b53c) line 0x206 and return); slot 0 = the club of the first Superettan row with status
/// `table_status::PROMOTION_PLAYOFF`, slot 1 = the club of the first Allsvenskan row (`G_COMP_OBJECT_TABLE` by the id
/// of `KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION`) with status `table_status::RELEGATION_PLAYOFF` (a slot without such a
/// row keeps what malloc gave); the stage (`build_two_club_play_off`), then Cup::set_up_round (`exe::cup_set_up_round`
/// with 0) on the first slot, called before the slot is checked (a defect of the original: with no stage it runs on
/// a null object); with no stage, the box of line 0x22e; otherwise `.current_stage()` = 0.
/// Original: `SweFirst::create_play_off_stage` (0x88f3e0, swe_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_first_create_play_off_stage(this: usize) {
    let this = League(this);
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(0xa8b53c, 0x206);
    }
    if let Some(c) = first_row_club_with_status(this, table_status::PROMOTION_PLAYOFF) { // mwe10: the Superettan play-off club
        write_usize(list, c);
    }
    if let Some(c) = premier_row_club_with_status(KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION, table_status::RELEGATION_PLAYOFF) {
        write_usize(list + 4, c);
    }
    covered!("mwe_10 superettan play-off");
    let stage = build_two_club_play_off(this, list);
    exe::cup_set_up_round(stage, 0);
    if read_usize(this.stages()) == 0 {
        return show_error_at(0xa8b53c, 0x22e);
    }
    this.set_current_stage(0);
}

/// The comparer of the two stage advances: only the original runs (it updates the Premier Division and may build the
/// play-off); the current stage must stay (at the last stage, or with the Premier Division still playing) or go up by
/// one, and on reaching 0 the first stage slot must hold a stage.
pub(crate) unsafe fn compare_first_division_advance(name: &str, this: League, original: impl FnOnce()) {
    let before = this.current_stage() as i32;
    let last = before >= (this.stage_count() as i32).wrapping_sub(1);
    original();
    let after = this.current_stage() as i32;
    let ok = if last { after == before } else { (after == before || after == before + 1) && (after != 0 || before == 0 || read_usize(this.stages()) != 0) };
    SCOREBOARD.record(ok, || format!("{}({:#x}) before={} after={} stages={}", name, this.addr(), before, after, this.stage_count()));
}

/// Comparer for `swe_first_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_swe_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SWE_FIRST_ADVANCE_STAGE);
    compare_first_division_advance("swe_first_advance_stage", League(this), || o(this))
}

/// Comparer for `nor_first_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_nor_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_NOR_FIRST_ADVANCE_STAGE);
    compare_first_division_advance("nor_first_advance_stage", League(this), || o(this))
}

/// Comparer for `swe_first_create_play_off_stage`: only the original runs (it allocates and draws); the plan (the two
/// clubs, made beforehand) against the stage in the first slot: two clubs, the same as the plan (as a set), the
/// competition and `.current_stage()` = 0.
pub(crate) unsafe extern "thiscall" fn compare_swe_first_create_play_off_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SWE_FIRST_CREATE_PLAY_OFF_STAGE);
    let l = League(this);
    let plan = [first_row_club_with_status(l, table_status::PROMOTION_PLAYOFF), premier_row_club_with_status(KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION, table_status::RELEGATION_PLAYOFF)];
    o(this);
    compare_two_club_play_off("swe_first_create_play_off_stage", l, plan);
}

/// Checks the two-club play-off stage of the first slot against the plan (as a set, the round draw may swap them).
pub(crate) unsafe fn compare_two_club_play_off(name: &str, this: League, plan: [Option<usize>; 2]) {
    let stage = Cup(read_usize(this.stages()));
    if stage.is_null() {
        SCOREBOARD.record(false, || format!("{}({:#x}) without stage", name, this.addr()));
        return;
    }
    let list = stage.team_table();
    let mut found = [read_usize(list), read_usize(list + 6)];
    let mut want = [plan[0].unwrap_or(0), plan[1].unwrap_or(0)];
    found.sort_unstable();
    want.sort_unstable();
    let ok = plan.iter().all(|c| c.is_some()) && found == want && stage.team_count() == 2 && read_usize(stage.addr() + 4) == this.competition().addr() && this.current_stage() == 0;
    SCOREBOARD.record(ok, || format!("{}({:#x}) plan={:x?} stage={:#x} list={:x?} +0x3e={}", name, this.addr(), plan, stage.addr(), found, stage.team_count()));
}

// ---------- batch 304: the Norwegian promotion play-off and the Ykkönen stage advance and club progress (nor_first.cpp, fin_first.cpp) ----------

/// The Norwegian promotion play-off (stage 0): the Norwegian First Division club with the promotion play-off status
/// (the 3rd) against the Tippeligaen club with the relegation play-off status (the 12th), as a cup tie.
///
/// `NorFirst::create_play_off_stage()`: the list of 2 (`exe::crt_malloc` of 8; out of memory: the box of nor_first.cpp
/// (0xa69740) line 0x326 and return). Slot 0 = the club of the LAST First Division row with status
/// `table_status::PROMOTION_PLAYOFF` (the loop does not stop at the first; the count re-read each turn); none: the box
/// of line 0x336 and return (the list is not freed). Slot 1 = the club of the last Tippeligaen row
/// (`G_COMP_OBJECT_TABLE` by the id of `KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION`, League::get_table_row
/// `exe::league_get_table_row`, called twice for the chosen row) with status `table_status::RELEGATION_PLAYOFF`; a
/// Tippeligaen without clubs, or with no such row, ends it silently (the list is not freed). Then the stage
/// (`build_two_club_play_off`: the round table of stage 0, the Cup_Stage of the 2 clubs into the first slot, the list
/// and the round table freed); with no stage, the box of line 0x357; otherwise Cup::set_up_round
/// (`exe::cup_set_up_round` with 0). Unlike the Superettan's, it does not write `.current_stage()` (the advance already
/// set it to 0).
/// Original: `NorFirst::create_play_off_stage` (0x790e20, nor_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_first_create_play_off_stage(this: usize) {
    let this = League(this);
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(0xa69740, 0x326);
    }
    let Some(c) = last_row_club_with_status(this, table_status::PROMOTION_PLAYOFF) else { // mwe11: the Norwegian play-off club
        return show_error_at(0xa69740, 0x336);
    };
    write_usize(list, c);
    let Some(c) = last_premier_row_club_with_status(KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION, table_status::RELEGATION_PLAYOFF) else {
        return;
    };
    write_usize(list + 4, c);
    covered!("mwe_11 norwegian play-off");
    let stage = build_norwegian_play_off(this, list);
    if stage == 0 {
        return show_error_at(0xa69740, 0x357);
    }
    exe::cup_set_up_round(stage, 0);
}

/// The club of the last table row (a signed word count, re-read each turn) of `league` whose status is `status`.
pub(crate) unsafe fn last_row_club_with_status(league: League, status: u8) -> Option<usize> {
    let mut found = None;
    let mut i = 0i32;
    while i < league.club_count() as i32 {
        let r = league.table() + i as usize * league_table_row::SIZE;
        if read_u8(r + league_table_row::STATUS) == status {
            found = Some(read_usize(r));
        }
        i += 1;
    }
    found
}

/// The club of the last row of the Premier Division of key `premier` (League::get_table_row, twice for the chosen row)
/// whose status is `status`; None without clubs or without such a row.
pub(crate) unsafe fn last_premier_row_club_with_status(premier: usize, status: u8) -> Option<usize> {
    let p = League(comp_by_global(premier));
    let mut found = None;
    let mut i = 0i32;
    while i < p.club_count() as i32 {
        if read_u8(exe::league_get_table_row(p, i) + league_table_row::STATUS) == status {
            found = Some(read_usize(exe::league_get_table_row(p, i)));
        }
        i += 1;
    }
    found
}

/// The Cup_Stage of the two clubs of `list` into the first stage slot (the same steps as the Superettan's: the round
/// table of stage 0 by virtual `comp_vtable::CREATE_SCHEDULE`, operator new of 0xb2, Cup_Stage::Cup_Stage(league, 2,
/// list, rounds, competition, round table, year, 0, 1, extra, 0x14, 0, 0, 0, 0), the slot written, the list and the
/// round table freed); returns the slot.
pub(crate) unsafe fn build_norwegian_play_off(this: League, list: usize) -> usize {
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 0, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    // simplification: the original pushes the rounds, the extra and the year as dwords with garbage above the word;
    // the constructor only reads the words.
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 2, list, rounds as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(this.stages(), stage);
    exe::crt_free(list);
    exe::crt_free(t);
    read_usize(this.stages())
}

/// Moves the Ykkönen to its next stage: after the North group (and the South group stage 0) the play-off stages are
/// built at stage 1; stage 2 waits for the Veikkausliiga to finish and then the Premier Division play-off is built at
/// stage 3; at the last stage, the error box (line 0x2ce).
///
/// `FinFirst::advance_stage()`: at the last stage (`.current_stage()` ≥ `.stage_count()` − 1, signed), the box of
/// fin_first.cpp (0x9c818c) line 0x2ce and nothing else. At stage 2, only when the Veikkausliiga
/// (`KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION`) is complete (virtual `comp_vtable::CHECK_COMPLETE` with (0, 1)), after
/// its virtual `league_vtable::UPDATE_POSITIONS_AND_STATUS`; otherwise nothing. Then the current stage goes up by one: on
/// reaching 1, FinFirst::create_play_off_stages (0x594080); on reaching 3, FinFirst::create_premier_play_off_stage
/// (0x594610) (direct calls, thiscall, ret 0).
/// Original: `FinFirst::advance_stage` (0x593f90, fin_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_first_advance_stage(this: usize) {
    let this = League(this);
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x9c818c, 0x2ce);
    }
    if stage == 2 {
        let p = comp_by_global(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION);
        let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(p) + comp_vtable::CHECK_COMPLETE));
        if complete(p, 0, 1) == 0 {
            return;
        }
        let update: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(p) + league_vtable::UPDATE_POSITIONS_AND_STATUS));
        update(p); // mwe11: the Veikkausliiga statuses brought up to date
    }
    let stage = stage.wrapping_add(1);
    this.set_current_stage(stage as usize);
    let next: usize = match stage {
        1 => 0x594080,
        3 => 0x594610,
        _ => return,
    };
    covered!("mwe_11 ykkonen next stage built");
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(next);
    f(this.addr());
}

/// The Ykkönen progress code for one value at one stage: at stage 3 (the Premier Division play-off, for a Veikkausliiga
/// club) 1 → 11 and 2 → 12; at stage 2 (the play-offs, for an Ykkönen club) 1 → 15 and 3 → 17; otherwise the value.
pub(crate) fn finnish_progress_code(stage: i8, v: u8) -> u8 {
    match (stage, v) {
        (3, 1) => 0xb,
        (3, 2) => 0xc,
        (2, 1) => 0xf, // mwe11: the play-off winner's code
        (2, 3) => 0x11,
        _ => v,
    }
}

/// Writes a club's current, best and worst progress codes in its Comp_Stats entry, interleaving the two groups and
/// mapping the play-off results to the codes after the league places.
///
/// `FinFirst::set_club_progress(club, stage, current, best, worst)`: the entry of the club in the league's Comp_Stats
/// (`exe::comp_stats_find_club_entry`, thiscall, ret 4). Stage (a signed byte) below 1 (the groups): each value v
/// becomes 2v − 1 (a byte; the North and South places interleave). Stage 1: as they come. Stage 3: for a club whose
/// division (`club::DIVISION`, read without a null check) is the Veikkausliiga, the entry of the Veikkausliiga's
/// Comp_Stats (`G_COMP_OBJECT_TABLE` by `KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION`, its `league::COMP_STATS`) with 1 → 11
/// and 2 → 12; any other club, as they come in the Ykkönen entry. Stage 2: for an Ykkönen club
/// (`KEY_CLUB_COMP_FINNISH_FIRST_DIVISION`), 1 → 15 and 3 → 17; any other club, nothing is written. Other stages: as
/// they come. The bytes go to `comp_stats_club::CURRENT_POSITION`, `::BEST_POSITION` and `::WORST_POSITION` (the entry
/// is not checked for null).
/// Original: `FinFirst::set_club_progress` (0x593e10, fin_first.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn fin_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let mut e = exe::comp_stats_find_club_entry(read_usize(this + league::COMP_STATS), club);
    let s = stage as u8 as i8;
    let (mut c, mut b, mut w) = (current as u8, best as u8, worst as u8);
    if s < 1 {
        covered!("mwe_11 ykkonen group progress");
        c = c.wrapping_shl(1).wrapping_sub(1); // mwe11: the group places interleaved
        b = b.wrapping_shl(1).wrapping_sub(1);
        w = w.wrapping_shl(1).wrapping_sub(1);
    } else if s == 3 {
        if read_usize(read_usize(club + club::DIVISION)) == read_usize(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION) {
            e = exe::comp_stats_find_club_entry(read_usize(comp_by_global(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION) + league::COMP_STATS), club);
            c = finnish_progress_code(3, c);
            b = finnish_progress_code(3, b);
            w = finnish_progress_code(3, w);
        }
    } else if s == 2 {
        if read_usize(read_usize(club + club::DIVISION)) != read_usize(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION) {
            return;
        }
        c = finnish_progress_code(2, c);
        b = finnish_progress_code(2, b);
        w = finnish_progress_code(2, w);
    }
    write_u8(e + comp_stats_club::BEST_POSITION, b);
    write_u8(e + comp_stats_club::CURRENT_POSITION, c);
    write_u8(e + comp_stats_club::WORST_POSITION, w);
}

/// Comparer for `nor_first_create_play_off_stage`: only the original runs (it allocates and draws); the planned
/// clubs (the last rows with each status) against the stage, when both exist.
pub(crate) unsafe extern "thiscall" fn compare_nor_first_create_play_off_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_NOR_FIRST_CREATE_PLAY_OFF_STAGE);
    let l = League(this);
    let plan = [last_row_club_with_status(l, table_status::PROMOTION_PLAYOFF), last_premier_row_club_with_status(KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION, table_status::RELEGATION_PLAYOFF)];
    o(this);
    if plan.iter().all(|c| c.is_some()) {
        let stage = Cup(read_usize(l.stages()));
        let ok = !stage.is_null() && stage.team_count() == 2 && {
            let list = stage.team_table();
            let mut found = [read_usize(list), read_usize(list + 6)];
            let mut want = [plan[0].unwrap(), plan[1].unwrap()];
            found.sort_unstable();
            want.sort_unstable();
            found == want
        };
        SCOREBOARD.record(ok, || format!("nor_first_create_play_off_stage({:#x}) plan={:x?} stage={:#x}", this, plan, stage.addr()));
    }
}

/// Comparer for `fin_first_advance_stage`: only the original runs (it builds stages); the current stage stays (at the
/// last stage, or at stage 2 with the Veikkausliiga still playing) or goes up by one.
pub(crate) unsafe extern "thiscall" fn compare_fin_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_FIN_FIRST_ADVANCE_STAGE);
    let l = League(this);
    let before = l.current_stage() as i32;
    let last = before >= (l.stage_count() as i32).wrapping_sub(1);
    o(this);
    let after = l.current_stage() as i32;
    // A fase sobe 1 e, quando a nova e 1 ou 3, a rotina chama o construtor da fase seguinte (0x594080 / 0x594610),
    // que avanca de novo: de 0 o original para em 2 (visto no jogo real da 304). Entao a invariante verdadeira e "a
    // fase subiu ao menos 1", nao "subiu exatamente 1" (17/11/2026).
    let ok = if last {
        after == before
    } else if before == 2 {
        after == 2 || after >= 3
    } else {
        after >= before + 1 && after <= (l.stage_count() as i32).wrapping_sub(1)
    };
    SCOREBOARD.record(ok, || format!("fin_first_advance_stage({:#x}) before={} after={}", this, before, after));
}

/// Comparer for `fin_first_set_club_progress`: side by side on the 3 bytes of the entry the routine writes (the
/// Veikkausliiga's at stage 3 for a Veikkausliiga club); only the original without an entry.
pub(crate) unsafe extern "thiscall" fn compare_fin_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_FIN_FIRST_SET_CLUB_PROGRESS);
    let premier_club = read_usize(club + club::DIVISION) != 0 && read_usize(read_usize(club + club::DIVISION)) == read_usize(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION);
    let stats = if stage as u8 as i8 == 3 && premier_club { read_usize(comp_by_global(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION) + league::COMP_STATS) } else { read_usize(this + league::COMP_STATS) };
    let e = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if e == 0 || read_usize(club + club::DIVISION) == 0 {
        return o(this, club, stage, current, best, worst);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(e + comp_stats_club::CURRENT_POSITION, 3)], || o(this, club, stage, current, best, worst), || fin_first_set_club_progress(this, club, stage, current, best, worst));
    SCOREBOARD.record(same, || format!("fin_first_set_club_progress({:#x}, {:#x}, stage {}, {} {} {}) original={:x?} rust={:x?}", this, club, stage as u8 as i8, current as u8, best as u8, worst as u8, mo, mr));
}

// ---------- batch 304: the schedules of the Allsvenskan, the Veikkausliiga and the Portuguese Premier Division (swe_prm.cpp, fin_prm.cpp, por_prm.cpp) ----------

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

/// The Veikkausliiga: the 33 rounds (three legs of 12 clubs), from late April to late October, on Sundays (weekday 6),
/// Wednesdays (2) and Mondays (0) (the midweek ones with code 2); the first and the last on a Saturday (5); no TV slots.
pub(crate) const VEIKKAUSLIIGA_ROUNDS: [EngFirstScheduleRound; 33] = [
    (29, 3, 0, 5, 1, &[(-1, -1, -1)]), // 0: 29/abr
    (3, 4, 0, 2, 2, &[(-1, -1, -1)]), // 1: 3/mai
    (10, 4, 0, 2, 2, &[(-1, -1, -1)]), // 2: 10/mai
    (15, 4, 0, 0, 2, &[(-1, -1, -1)]), // 3: 15/mai
    (21, 4, 0, 6, 1, &[(-1, -1, -1)]), // 4: 21/mai
    (24, 4, 0, 2, 2, &[(-1, -1, -1)]), // 5: 24/mai
    (28, 4, 0, 6, 1, &[(-1, -1, -1)]), // 6: 28/mai
    (31, 4, 0, 2, 2, &[(-1, -1, -1)]), // 7: 31/mai
    (7, 5, 0, 2, 2, &[(-1, -1, -1)]), // 8: 7/jun
    (14, 5, 0, 2, 2, &[(-1, -1, -1)]), // 9: 14/jun
    (19, 5, 0, 0, 2, &[(-1, -1, -1)]), // 10: 19/jun
    (26, 5, 0, 0, 2, &[(-1, -1, -1)]), // 11: 26/jun
    (3, 6, 0, 0, 2, &[(-1, -1, -1)]), // 12: 3/jul
    (9, 6, 0, 6, 1, &[(-1, -1, -1)]), // 13: 9/jul
    (12, 6, 0, 2, 2, &[(-1, -1, -1)]), // 14: 12/jul
    (16, 6, 0, 6, 1, &[(-1, -1, -1)]), // 15: 16/jul
    (19, 6, 0, 2, 2, &[(-1, -1, -1)]), // 16: 19/jul
    (26, 6, 0, 2, 2, &[(-1, -1, -1)]), // 17: 26/jul
    (30, 6, 0, 6, 1, &[(-1, -1, -1)]), // 18: 30/jul
    (2, 7, 0, 2, 2, &[(-1, -1, -1)]), // 19: 2/ago
    (6, 7, 0, 6, 1, &[(-1, -1, -1)]), // 20: 6/ago
    (13, 7, 0, 6, 1, &[(-1, -1, -1)]), // 21: 13/ago
    (20, 7, 0, 6, 1, &[(-1, -1, -1)]), // 22: 20/ago
    (27, 7, 0, 6, 1, &[(-1, -1, -1)]), // 23: 27/ago
    (6, 8, 0, 2, 2, &[(-1, -1, -1)]), // 24: 6/set
    (10, 8, 0, 6, 1, &[(-1, -1, -1)]), // 25: 10/set
    (17, 8, 0, 6, 1, &[(-1, -1, -1)]), // 26: 17/set
    (20, 8, 0, 2, 2, &[(-1, -1, -1)]), // 27: 20/set
    (1, 9, 0, 6, 1, &[(-1, -1, -1)]), // 28: 1/out
    (15, 9, 0, 6, 1, &[(-1, -1, -1)]), // 29: 15/out
    (22, 9, 0, 6, 1, &[(-1, -1, -1)]), // 30: 22/out
    (25, 9, 0, 2, 2, &[(-1, -1, -1)]), // 31: 25/out
    (28, 9, 0, 5, 1, &[(-1, -1, -1)]), // 32: 28/out (mwe12: the Veikkausliiga last round)
];

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

/// The Veikkausliiga schedule: 33 rounds (12 clubs, three legs), from 29 April to 28 October, on Sundays, Wednesdays and Mondays; no TV slots and no play-offs (main stage only).
///
/// `FinPrm::create_schedule(stage, &rounds, &extra, &league)`: at stage −1 (0xff), `*league` = 1 (when given), 33 rounds,
/// extra 0 and the table (`exe::crt_malloc` of 0x861 = 33 × 0x41; out of memory: the box of fin_prm.cpp (0x9c8254) line 0xb3 and 0),
/// filled round by round from `VEIKKAUSLIIGA_ROUNDS` (`eng_first_set_league_rounds`: League round date `exe::league_round_set_date`(t,
/// i, day, month, years, weekday, code, year, 0) and the TV slots `exe::league_round_set_match_slot`(t, i, j, a, b,
/// c, 0), the list closed by (−1, −1, −1); both cdecl); the year is the league's (`.year()`, the original pushes the
/// word with garbage above it: the date only reads the word). Any other stage: 0 and nothing written. 
/// Original: `FinPrm::create_schedule` (0x5954b0, fin_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn fin_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    if !league.is_null() {
        *league = 1;
    }
    *rounds = 0x21;
    *extra = 0;
    let t = exe::crt_malloc(0x861);
    if t == 0 {
        show_error_at(0x9c8254, 0xb3);
        return 0;
    }
    covered!("mwe_12 veikkausliiga schedule");
    let year = (League(this).year() as u16) as u32;
    eng_first_set_league_rounds(t, year, 0, &VEIKKAUSLIIGA_ROUNDS); // mwe12: the Veikkausliiga rounds
    t
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

/// Comparer for `swe_prm_create_schedule`: side by side (`compare_league_round_table`: the original with the caller's outputs, the copy
/// with its own; the outputs and the written round bytes equal; the copy's table freed).
pub(crate) unsafe extern "thiscall" fn compare_swe_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SWE_PRM_CREATE_SCHEDULE);
    compare_league_round_table("swe_prm_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| swe_prm_create_schedule(this, stage, r, e, l))
}

/// Comparer for `fin_prm_create_schedule`: side by side (`compare_league_round_table`: the original with the caller's outputs, the copy
/// with its own; the outputs and the written round bytes equal; the copy's table freed).
pub(crate) unsafe extern "thiscall" fn compare_fin_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_FIN_PRM_CREATE_SCHEDULE);
    compare_league_round_table("fin_prm_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| fin_prm_create_schedule(this, stage, r, e, l))
}

/// Comparer for `por_prm_create_schedule`: side by side (`compare_league_round_table`: the original with the caller's outputs, the copy
/// with its own; the outputs and the written round bytes equal; the copy's table freed).
pub(crate) unsafe extern "thiscall" fn compare_por_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_POR_PRM_CREATE_SCHEDULE);
    compare_league_round_table("por_prm_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| por_prm_create_schedule(this, stage, r, e, l))
}

// ---------- batch 304: the schedules of the Liga de Honra, the Superettan and the Ykkonen (por_second.cpp, swe_first.cpp, fin_first.cpp) ----------

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

/// The Superettan: the 30 rounds, the Allsvenskan dates (Sundays, weekday 6) with four Wednesday rounds (weekday 2,
/// code 2) in August, September and October; no TV slots.
pub(crate) const SUPERETTAN_ROUNDS: [EngFirstScheduleRound; 30] = [
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
    (19, 6, 0, 6, 1, &[(-1, -1, -1)]), // 12: 19/jul (mwe13: the Superettan July round)
    (2, 7, 0, 6, 1, &[(-1, -1, -1)]), // 13: 2/ago
    (9, 7, 0, 6, 1, &[(-1, -1, -1)]), // 14: 9/ago
    (16, 7, 0, 6, 1, &[(-1, -1, -1)]), // 15: 16/ago
    (23, 7, 0, 6, 1, &[(-1, -1, -1)]), // 16: 23/ago
    (26, 7, 0, 2, 2, &[(-1, -1, -1)]), // 17: 26/ago
    (30, 7, 0, 6, 1, &[(-1, -1, -1)]), // 18: 30/ago
    (13, 8, 0, 6, 1, &[(-1, -1, -1)]), // 19: 13/set
    (16, 8, 0, 2, 2, &[(-1, -1, -1)]), // 20: 16/set
    (20, 8, 0, 6, 1, &[(-1, -1, -1)]), // 21: 20/set
    (27, 8, 0, 6, 1, &[(-1, -1, -1)]), // 22: 27/set
    (30, 8, 0, 2, 2, &[(-1, -1, -1)]), // 23: 30/set
    (4, 9, 0, 6, 1, &[(-1, -1, -1)]), // 24: 4/out
    (18, 9, 0, 6, 1, &[(-1, -1, -1)]), // 25: 18/out
    (21, 9, 0, 2, 2, &[(-1, -1, -1)]), // 26: 21/out
    (25, 9, 0, 6, 1, &[(-1, -1, -1)]), // 27: 25/out
    (31, 9, 0, 5, 1, &[(-1, -1, -1)]), // 28: 31/out
    (8, 10, 0, 6, 1, &[(-1, -1, -1)]), // 29: 8/nov
];

/// The Ykkonen groups: the 27 rounds (three legs of 10 clubs), from 29 April to 6 October, on Sundays (weekday 6),
/// with midweek rounds on Wednesdays, Thursdays or a Monday (weekdays 2, 3 and 0, code 2) and two Saturdays; no TV
/// slots.
pub(crate) const YKKONEN_ROUNDS: [EngFirstScheduleRound; 27] = [
    (29, 3, 0, 6, 1, &[(-1, -1, -1)]), // 0: 29/abr
    (6, 4, 0, 6, 1, &[(-1, -1, -1)]), // 1: 6/mai
    (12, 4, 0, 5, 1, &[(-1, -1, -1)]), // 2: 12/mai
    (20, 4, 0, 6, 1, &[(-1, -1, -1)]), // 3: 20/mai
    (24, 4, 0, 3, 2, &[(-1, -1, -1)]), // 4: 24/mai
    (27, 4, 0, 6, 1, &[(-1, -1, -1)]), // 5: 27/mai
    (3, 5, 0, 6, 1, &[(-1, -1, -1)]), // 6: 3/jun
    (6, 5, 0, 2, 2, &[(-1, -1, -1)]), // 7: 6/jun
    (10, 5, 0, 6, 1, &[(-1, -1, -1)]), // 8: 10/jun
    (13, 5, 0, 2, 2, &[(-1, -1, -1)]), // 9: 13/jun
    (17, 5, 0, 6, 1, &[(-1, -1, -1)]), // 10: 17/jun (mwe13: the Ykkonen June round)
    (8, 6, 0, 6, 1, &[(-1, -1, -1)]), // 11: 8/jul
    (11, 6, 0, 2, 2, &[(-1, -1, -1)]), // 12: 11/jul
    (15, 6, 0, 6, 1, &[(-1, -1, -1)]), // 13: 15/jul
    (22, 6, 0, 6, 1, &[(-1, -1, -1)]), // 14: 22/jul
    (29, 6, 0, 6, 1, &[(-1, -1, -1)]), // 15: 29/jul
    (2, 7, 0, 3, 2, &[(-1, -1, -1)]), // 16: 2/ago
    (6, 7, 0, 0, 2, &[(-1, -1, -1)]), // 17: 6/ago
    (12, 7, 0, 6, 1, &[(-1, -1, -1)]), // 18: 12/ago
    (19, 7, 0, 6, 1, &[(-1, -1, -1)]), // 19: 19/ago
    (26, 7, 0, 6, 1, &[(-1, -1, -1)]), // 20: 26/ago
    (2, 8, 0, 6, 1, &[(-1, -1, -1)]), // 21: 2/set
    (9, 8, 0, 6, 1, &[(-1, -1, -1)]), // 22: 9/set
    (16, 8, 0, 6, 1, &[(-1, -1, -1)]), // 23: 16/set
    (23, 8, 0, 6, 1, &[(-1, -1, -1)]), // 24: 23/set
    (29, 8, 0, 5, 1, &[(-1, -1, -1)]), // 25: 29/set
    (6, 9, 0, 6, 1, &[(-1, -1, -1)]), // 26: 6/out
];

/// The Superettan promotion play-off round (stage 0): draw on 9 November, one tie (two clubs, two legs, 7 days apart)
/// from 15 November (Sunday), code 0xa0, category 5, the two-leg tie rules 3, no prizes.
pub(crate) const SUPERETTAN_PLAY_OFF: EngCcCupCupRound = cql10r((9, 0xa, 0, 0), (0xf, 0xa, 0, 6, 1, 0), 0xa0, (0, 0, 3), 5, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)); // mwe13: the Superettan play-off dates
/// The Ykkonen play-off stage (stage 1), round 0: draw on 30 September, two ties (four clubs, two legs, 3 days apart)
/// from 5 October, code 0x82, tie rules 0x200, category 5.
pub(crate) const YKKONEN_PLAY_OFF_SEMI: EngCcCupCupRound = cql10r((0x1e, 9, 0, 0), (5, 0xa, 0, 6, 1, 0), 0x82, (0, 0x200, 3), 5, (4, 2, 4, 0), (0, 2, 3), (0, 0, 0));
/// The Ykkonen play-off stage (stage 1), round 1: draw on 9 October (weekday 3), one tie (two legs, 3 days apart) from
/// 12 October, code 0x96, no tie rules, category 5.
pub(crate) const YKKONEN_PLAY_OFF_FINAL: EngCcCupCupRound = cql10r((9, 0xa, 0, 3), (0xc, 0xa, 0, 6, 1, 0), 0x96, (0, 0, 3), 5, (2, 1, 0, 0), (0, 2, 3), (0, 0, 0));
/// The Ykkonen relegation play-off round (stage 2): draw on 30 September, two ties (four clubs, two legs, 7 days
/// apart) from 5 October, code 0, tie rules 0x200, category 5.
pub(crate) const YKKONEN_RELEGATION_PLAY_OFF: EngCcCupCupRound = cql10r((0x1e, 9, 0, 0), (5, 0xa, 0, 6, 1, 0), 0, (0, 0x200, 3), 5, (4, 2, 4, 0), (0, 2, 7), (0, 0, 0));
/// The play-off against the Veikkausliiga (stage 3): draw on 16 October (weekday 3), one tie (two clubs, two legs, 4 days
/// apart) from 19 October, code 0xa0, tie rules 0x200, category 5.
pub(crate) const YKKONEN_PREMIER_PLAY_OFF: EngCcCupCupRound = cql10r((0x10, 0xa, 0, 3), (0x13, 0xa, 0, 6, 1, 0), 0xa0, (0, 0x200, 3), 5, (2, 1, 2, 0), (0, 2, 4), (0, 0, 0));

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

/// Writes cup round `i` of table `t` (the draw and match dates and the round fields, `eng_cc_cup_write_cup_round`) with
/// the league's year sign-extended (the original's movsx of `.year()`).
pub(crate) unsafe fn write_play_off_round(this: League, t: usize, i: u32, r: &EngCcCupCupRound) {
    eng_cc_cup_write_cup_round(t, i, r, this.year() as i32 as u32);
}

/// The Superettan schedule: the league (stage −1) has 30 rounds (16 clubs, two legs), from 6 April to 8 November, the
/// Allsvenskan dates plus four Wednesday rounds; the promotion play-off (stage 0) is one two-legged tie from 15 November
/// (draw on 9 November).
///
/// `SweFirst::create_schedule(stage, &rounds, &extra, &league)`: stage −1 (0xff): `*league` = 1 (when given), 30 rounds,
/// extra 0, the table (`exe::crt_malloc` of 0x79e; out of memory: the box of swe_first.cpp (0xa8b53c) line 0xa8 and 0)
/// filled from `SUPERETTAN_ROUNDS` (`eng_first_set_league_rounds`). Stage 0: `*league` = 0 (when given), 1 round,
/// extra 0, the cup round table (`exe::crt_malloc` of 0x68; out of memory: the box of line 0x116 and 0) with round 0 =
/// `SUPERETTAN_PLAY_OFF` (Cup round draw date `exe::cup_set_round_draw_date` and match date
/// `exe::cup_set_round_match_date`, cdecl, with the year sign-extended; then the round fields). Any other stage: 0.
/// Original: `SweFirst::create_schedule` (0x88e8e0, swe_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn swe_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
    match stage as u8 {
        0xff => {
            if !league.is_null() {
                *league = 1;
            }
            *rounds = 0x1e;
            *extra = 0;
            let t = exe::crt_malloc(0x79e);
            if t == 0 {
                show_error_at(0xa8b53c, 0xa8);
                return 0;
            }
            covered!("mwe_13 superettan schedule");
            eng_first_set_league_rounds(t, (this.year() as u16) as u32, 0, &SUPERETTAN_ROUNDS);
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
                show_error_at(0xa8b53c, 0x116);
                return 0;
            }
            covered!("mwe_13 superettan play-off round");
            write_play_off_round(this, t, 0, &SUPERETTAN_PLAY_OFF);
            t
        }
        _ => 0,
    }
}

/// The Ykkonen schedule: the two groups (stages −1 and 0) have the same 27 rounds (three legs of 10 clubs), from 29 April
/// to 6 October, with the stage code 0x41d for the North group and 0x41e for the South one; stage 1 is the promotion
/// play-off (two-legged semi-finals of four clubs from 5 October and a two-legged final from 12 October); stage 2 the
/// relegation play-off (two two-legged ties from 5 October); stage 3 the play-off against the Veikkausliiga (one
/// two-legged tie from 19 October).
///
/// `FinFirst::create_schedule(stage, &rounds, &extra, &league)`: stage below 1 (a signed byte: −1 or 0): `*league` = 1
/// (when given), 27 rounds, extra 0x41d (stage −1) or 0x41e (any other, i.e. 0), the table (`exe::crt_malloc` of
/// 0x6db; out of memory: the box of fin_first.cpp (0x9c818c) line 0xb5 and 0) filled from `YKKONEN_ROUNDS`
/// (`eng_first_set_league_rounds`). Stage 1: `*league` = 0, 2 rounds, extra 0x458, malloc 0xd0 (box line 0x11d), the
/// rounds `YKKONEN_PLAY_OFF_SEMI` and `YKKONEN_PLAY_OFF_FINAL`. Stage 2: `*league` = 0, 1 round, extra 0x452, malloc
/// 0x68 (box 0x158), `YKKONEN_RELEGATION_PLAY_OFF`. Stage 3: `*league` = 0, 1 round, extra 0, malloc 0x68 (box 0x180),
/// `YKKONEN_PREMIER_PLAY_OFF`. The cup rounds by `write_play_off_round` (the dates with the year sign-extended).
/// Any other stage: 0.
/// Original: `FinFirst::create_schedule` (0x5931c0, fin_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn fin_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
    let s = stage as u8 as i8;
    if s < 1 {
        if !league.is_null() {
            *league = 1;
        }
        *rounds = 0x1b;
        *extra = if s as u8 == 0xff { 0x41d } else { 0x41e }; // mwe13: the North and South group codes
        let t = exe::crt_malloc(0x6db);
        if t == 0 {
            show_error_at(0x9c818c, 0xb5);
            return 0;
        }
        covered!("mwe_13 ykkonen group schedule");
        eng_first_set_league_rounds(t, (this.year() as u16) as u32, 0, &YKKONEN_ROUNDS);
        return t;
    }
    let (n, code, bytes, row, table): (i16, i16, u32, u32, &[&EngCcCupCupRound]) = match s {
        1 => (2, 0x458, 0xd0, 0x11d, &[&YKKONEN_PLAY_OFF_SEMI, &YKKONEN_PLAY_OFF_FINAL]),
        2 => (1, 0x452, 0x68, 0x158, &[&YKKONEN_RELEGATION_PLAY_OFF]),
        3 => (1, 0, 0x68, 0x180, &[&YKKONEN_PREMIER_PLAY_OFF]),
        _ => return 0,
    };
    if !league.is_null() {
        *league = 0;
    }
    *rounds = n;
    *extra = code;
    let t = exe::crt_malloc(bytes);
    if t == 0 {
        show_error_at(0x9c818c, row);
        return 0;
    }
    covered!("mwe_13 ykkonen play-off rounds");
    for (i, r) in table.iter().enumerate() {
        write_play_off_round(this, t, i as u32, r);
    }
    t
}

/// Comparer for `por_second_create_schedule`: side by side (`compare_league_round_table`: the original with the caller's outputs, the copy
/// with its own; the outputs and the written round bytes equal; the copy's table freed).
pub(crate) unsafe extern "thiscall" fn compare_por_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_POR_SECOND_CREATE_SCHEDULE);
    compare_league_round_table("por_second_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| por_second_create_schedule(this, stage, r, e, l))
}

/// Comparer for `swe_first_create_schedule`: side by side (`compare_league_round_table`: the original with the caller's outputs, the copy
/// with its own; the outputs and the written round bytes equal; the copy's table freed).
pub(crate) unsafe extern "thiscall" fn compare_swe_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SWE_FIRST_CREATE_SCHEDULE);
    compare_league_round_table("swe_first_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| swe_first_create_schedule(this, stage, r, e, l))
}

/// Comparer for `fin_first_create_schedule`: side by side as `compare_league_round_table`, but the league bytes are
/// compared at both group stages (−1 and 0) and the cup round bytes at stages 1 to 3.
pub(crate) unsafe extern "thiscall" fn compare_fin_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_FIN_FIRST_CREATE_SCHEDULE);
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let pl2 = if league.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, same, _, _) = crate::comparison::side_by_side(&[(0xb67a34usize, 4usize)], || o(this, stage, rounds, extra, league), || fin_first_create_schedule(this, stage, &mut r2, &mut e2, pl2));
    let outs = (a == 0 && b == 0) || (*rounds == r2 && *extra == e2 && (league.is_null() || *league == l2));
    let table = if a != 0 && b != 0 {
        if (stage as u8 as i8) < 1 {
            written_league_round_bytes(a, a, *rounds as usize) == written_league_round_bytes(b, a, *rounds as usize)
        } else {
            eng_cc_cup_written_round_bytes(a, *rounds) == eng_cc_cup_written_round_bytes(b, *rounds)
        }
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(same && outs && table, || format!("fin_first_create_schedule(stage {}): outs {} table {}", stage as u8 as i8, outs, table));
    a
}
