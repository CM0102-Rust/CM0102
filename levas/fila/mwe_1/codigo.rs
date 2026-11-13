// ---------- batch %%LEVA%%: the constructors of the Norwegian Premier Division and of the Portuguese Premier and Second Divisions (nor_prm.cpp, por_prm.cpp, por_second.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_division_league_ctor("construct_nor_prm", League(this), year, record, &NORWEGIAN_PREMIER_DIVISION, || o(this, year, record))
}

/// Comparer for `construct_por_prm`.
pub(crate) unsafe extern "thiscall" fn compare_construct_por_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    compare_division_league_ctor("construct_por_prm", League(this), year, record, &PORTUGUESE_PREMIER_DIVISION, || o(this, year, record))
}

/// Comparer for `construct_por_second`.
pub(crate) unsafe extern "thiscall" fn compare_construct_por_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    compare_division_league_ctor("construct_por_second", League(this), year, record, &PORTUGUESE_SECOND_DIVISION, || o(this, year, record))
}
