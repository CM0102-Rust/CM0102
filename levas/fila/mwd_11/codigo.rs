// ---------- batch %%LEVA%%: the constructors of the Greek and Russian divisions (gre_prm.cpp, gre_second.cpp, rus_first.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_division_league_ctor("construct_gre_prm", League(this), year, record, &GREEK_FIRST_DIVISION, || o(this, year, record))
}

/// Comparer for `construct_gre_second`.
pub(crate) unsafe extern "thiscall" fn compare_construct_gre_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    compare_division_league_ctor("construct_gre_second", League(this), year, record, &GREEK_SECOND_DIVISION, || o(this, year, record))
}

/// Comparer for `construct_rus_first`.
pub(crate) unsafe extern "thiscall" fn compare_construct_rus_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    compare_division_league_ctor("construct_rus_first", League(this), year, record, &RUSSIAN_FIRST_DIVISION, || o(this, year, record))
}
