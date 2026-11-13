// ---------- batch %%LEVA%%: the constructors of the Turkish divisions and the Russian Premier League (tur_first.cpp, tur_second.cpp, rus_prm.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_division_league_ctor("construct_tur_first", League(this), year, record, &TURKISH_FIRST_DIVISION, || o(this, year, record))
}

/// Comparer for `construct_tur_second`.
pub(crate) unsafe extern "thiscall" fn compare_construct_tur_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    compare_division_league_ctor("construct_tur_second", League(this), year, record, &TURKISH_SECOND_DIVISION, || o(this, year, record))
}

/// Comparer for `construct_rus_prm` (`compare_tiebreak_league_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_rus_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    compare_tiebreak_league_ctor("construct_rus_prm", League(this), year, record, &RUSSIAN_PREMIER_LEAGUE, || o(this, year, record))
}
