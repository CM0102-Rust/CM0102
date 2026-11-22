// ---------- batch %%LEVA%%: the key nation competitions, part 3: Norway to Wales (Setup.cpp) ----------

// (uses the mould of tel13_2: KeyNationComps, key_nation_create_comps, key_nation_check_comps)

/// Creates Norway's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x358c and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 12 April of
/// the start date year (weekday 6) to 31 October of the next year (weekday 6). 3 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): Norwegian Premier Division, Norwegian
/// First Division, Norwegian Cup; every constructor gets the start date year. A failed allocation: the "Memory
/// Allocation Error" box (Setup.cpp 0x35a8, "create_norwegian_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(27, 10, 2001, -1) (27 November 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the start date year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original: `create_norwegian_comps` (0x836050, Setup.cpp,
/// cdecl).
pub(crate) unsafe extern "C" fn create_norwegian_comps(entry: usize) -> u32 {
    covered!("tel13_4 norwegian");
    key_nation_create_comps(entry, &NORWEGIAN_COMPS)
}

/// The entry of `create_norwegian_comps`.
const NORWEGIAN_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x358c, 0x35a8, 0xa81f44), // "create_norwegian_comps() : unable to allocate memory"
    start: (12, 3, EntryYear::Start(0), 0x6),
    end: (31, 9, EntryYear::Current(1), 0x6),
    count: EntryCount::Fixed(3),
    calloc: None,
    comps: &[
        (0xee, 0x791500, KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION, EntryYear::Start(0), false), // NorPrm::NorPrm
        (0xee, 0x78fd40, KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION, EntryYear::Start(0), false), // NorFirst::NorFirst
        (0xb2, 0x78f020, KEY_CLUB_COMP_NORWEGIAN_CUP, EntryYear::Start(0), false), // NorCup::NorCup
    ],
    end_day: (27, 10),
    season_year: EntryYear::Start(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates Poland's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x35cf and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 17 July of the
/// game year (weekday 5) to 27 May of the next year (weekday 5). 5 competitions (`KEY_NATION_RECORD_COMP_COUNT`, the
/// array in `KEY_NATION_RECORD_COMPS`): Polish First Division, Polish Second Division, Polish FA Cup, Polish League
/// Cup, Polish Super Cup; every constructor gets the game year. A failed allocation: the "Memory Allocation Error" box
/// (Setup.cpp 0x35eb, "create_polish_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` =
/// the day of Date::Date(20, 5, 2001, -1) (20 June 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the game year,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` is not written; returns 1. ///
/// Original: `create_polish_comps` (0x836350, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_polish_comps(entry: usize) -> u32 {
    covered!("tel13_4 polish");
    key_nation_create_comps(entry, &POLISH_COMPS)
}

/// The entry of `create_polish_comps`.
const POLISH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x35cf, 0x35eb, 0xa821c0), // "create_polish_comps() : unable to allocate memory"
    start: (17, 6, EntryYear::Current(0), 0x5),
    end: (27, 4, EntryYear::Current(1), 0x5),
    count: EntryCount::Fixed(5),
    calloc: None,
    comps: &[
        (0xf2, 0x7c8a80, KEY_CLUB_COMP_POLISH_FIRST_DIVISION, EntryYear::Current(0), false), // PolFirst::PolFirst
        (0xee, 0x7cb1d0, KEY_CLUB_COMP_POLISH_SECOND_DIVISION, EntryYear::Current(0), false), // PolSecond::PolSecond
        (0xb2, 0x7c7d80, KEY_CLUB_COMP_POLISH_FA_CUP, EntryYear::Current(0), false), // PolCup::PolCup
        (0xb2, 0x7ca4e0, KEY_CLUB_COMP_POLISH_LEAGUE_CUP, EntryYear::Current(0), false), // PolLgeCup::PolLgeCup
        (0xb2, 0x7cc920, KEY_CLUB_COMP_POLISH_SUPER_CUP, EntryYear::Current(0), false), // PolSuper::PolSuper
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Untouched,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates Portugal's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x3612 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 25 August of
/// the game year (weekday 6) to 8 June of the next year (weekday 6). 4 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): Portuguese Premier Division, Portuguese Second Division, Portuguese Cup,
/// Portuguese Super Cup; with the nation's extra division (`nation::LEAGUE_SELECTED` bit 2) 5, adding Portuguese Second
/// Division B (created when the nation has it); every constructor gets the game year. A failed allocation: the "Memory
/// Allocation Error" box (Setup.cpp 0x3631, "create_portuguese_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(20, 5, 2001, -1) (20 June 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the game year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = the Portuguese Super Cup record; returns 1. /// Original:
/// `create_portuguese_comps` (0x8366e0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_portuguese_comps(entry: usize) -> u32 {
    covered!("tel13_4 portuguese");
    key_nation_create_comps(entry, &PORTUGUESE_COMPS)
}

/// The entry of `create_portuguese_comps`.
const PORTUGUESE_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x3612, 0x3631, 0xa821f4), // "create_portuguese_comps() : unable to allocate memory"
    start: (25, 7, EntryYear::Current(0), 0x6),
    end: (8, 5, EntryYear::Current(1), 0x6),
    count: EntryCount::ExtraDivision(4, 5, ExtraGuard::Flag),
    calloc: None,
    comps: &[
        (0xee, 0x7ce720, KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION, EntryYear::Current(0), false), // PorPrm::PorPrm
        (0xee, 0x7d0950, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION, EntryYear::Current(0), false), // PorSecond::PorSecond
        (0xee, 0x7d1920, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B, EntryYear::Current(0), true), // PorSecondB::PorSecondB
        (0xb2, 0x7cd980, KEY_CLUB_COMP_PORTUGUESE_CUP, EntryYear::Current(0), false), // PorCup::PorCup
        (0xb2, 0x7d2e40, KEY_CLUB_COMP_PORTUGUESE_SUPER_CUP, EntryYear::Current(0), false), // PorSuper::PorSuper
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Comp(KEY_CLUB_COMP_PORTUGUESE_SUPER_CUP),
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates Russia's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x365a and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 25 March of
/// the start date year (weekday 5) to 12 November of the start date year (weekday 6). 3 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): Russian Premier Division, Russian First
/// Division, Russian Cup; the constructors get the game year (the start date year: Russian Premier Division, Russian
/// First Division). A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x3679, "create_russian_comps() :
/// unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of Date::Date(27, 11, 2001, -1) (27
/// December 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the start date year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original: `create_russian_comps` (0x836ac0, Setup.cpp,
/// cdecl).
pub(crate) unsafe extern "C" fn create_russian_comps(entry: usize) -> u32 {
    covered!("tel13_4 russian");
    key_nation_create_comps(entry, &RUSSIAN_COMPS)
}

/// The entry of `create_russian_comps`.
const RUSSIAN_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x365a, 0x3679, 0xa8222c), // "create_russian_comps() : unable to allocate memory"
    start: (25, 2, EntryYear::Start(0), 0x5),
    end: (12, 10, EntryYear::Start(0), 0x6),
    count: EntryCount::Fixed(3),
    calloc: None,
    comps: &[
        (0xef, 0x7eaa80, KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION, EntryYear::Start(0), false), // RusPrm::RusPrm
        (0xee, 0x7e9f30, KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION, EntryYear::Start(0), false), // RusFirst::RusFirst
        (0xb2, 0x7e9210, KEY_CLUB_COMP_RUSSIAN_CUP, EntryYear::Current(0), false), // RusCup::RusCup
    ],
    end_day: (27, 11),
    season_year: EntryYear::Start(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates Scotland's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x36a1 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 2 August of
/// the game year (weekday 5) to 24 May of the next year (weekday 5). 7 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): Scottish Premier Division, Scottish First Division, Scottish Second
/// Division, Scottish Third Division, Scottish Cup, Scottish League Cup, Scottish League Challenge Cup; every
/// constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x36bd,
/// "create_scottish_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(20, 5, 2001, -1) (20 June 2001), `KEY_NATION_RECORD_SEASON_YEAR` = 2001,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original:
/// `create_scottish_comps` (0x836dc0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_scottish_comps(entry: usize) -> u32 {
    covered!("tel13_4 scottish");
    key_nation_create_comps(entry, &SCOTTISH_COMPS)
}

/// The entry of `create_scottish_comps`.
const SCOTTISH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x36a1, 0x36bd, 0xa82260), // "create_scottish_comps() : unable to allocate memory"
    start: (2, 7, EntryYear::Current(0), 0x5),
    end: (24, 4, EntryYear::Current(1), 0x5),
    count: EntryCount::Fixed(7),
    calloc: None,
    comps: &[
        (0xee, 0x7f0b80, KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION, EntryYear::Current(0), false), // ScoPrm::ScoPrm
        (0xee, 0x7edfc0, KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION, EntryYear::Current(0), false), // ScoFirst::ScoFirst
        (0xee, 0x7f3250, KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION, EntryYear::Current(0), false), // ScoSecond::ScoSecond
        (0xee, 0x7f41f0, KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION, EntryYear::Current(0), false), // ScoThird::ScoThird
        (0xb2, 0x7ed310, KEY_CLUB_COMP_SCOTTISH_CUP, EntryYear::Current(0), false), // ScoFaCup::ScoFaCup
        (0xb3, 0x7ef9e0, KEY_CLUB_COMP_SCOTTISH_LEAGUE_CUP, EntryYear::Current(0), false), // ScoLgeCup::ScoLgeCup
        (0xb2, 0x7ec980, KEY_CLUB_COMP_SCOTTISH_LEAGUE_CHALLENGE_CUP, EntryYear::Current(0), false), // ScoChalCup::ScoChalCup
    ],
    end_day: (20, 5),
    season_year: EntryYear::Fixed(0x7d1),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates South Korea's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp
/// "Error" box 0x36e9 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 25
/// March of the start date year (weekday 6) to 25 November of the start date year (weekday 5). 4 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): Korean League, Korean FA Cup, Korean
/// League Cup, Korean Super Cup; every constructor gets the start date year. A failed allocation: the "Memory
/// Allocation Error" box (Setup.cpp 0x3705, "create_spanish_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(20, 0, 2001, -1) (20 January 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the start date year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = the Korean Super Cup record; returns 1. The original's allocation message names
/// another routine (create_spanish_comps()). /// Original: `create_south_korean_comps` (0x837200, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_south_korean_comps(entry: usize) -> u32 {
    covered!("tel13_4 south korean");
    key_nation_create_comps(entry, &SOUTH_KOREAN_COMPS)
}

/// The entry of `create_south_korean_comps`.
const SOUTH_KOREAN_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x36e9, 0x3705, 0xa82294), // "create_spanish_comps() : unable to allocate memory"
    start: (25, 2, EntryYear::Start(0), 0x6),
    end: (25, 10, EntryYear::Start(0), 0x5),
    count: EntryCount::Fixed(4),
    calloc: None,
    comps: &[
        (0xee, 0x66ae70, KEY_CLUB_COMP_KOREAN_LEAGUE, EntryYear::Start(0), false), // KorLeague::KorLeague
        (0xb2, 0x66a460, KEY_CLUB_COMP_KOREAN_FA_CUP, EntryYear::Start(0), false), // KorFaCup::KorFaCup
        (0xf6, 0x66bf00, KEY_CLUB_COMP_KOREAN_LEAGUE_CUP, EntryYear::Start(0), false), // KorLeagueCup::KorLeagueCup
        (0xb2, 0x66d820, KEY_CLUB_COMP_KOREAN_SUPER_CUP, EntryYear::Start(0), false), // KorSuperCup::KorSuperCup
    ],
    end_day: (20, 0),
    season_year: EntryYear::Start(0),
    spans: 1,
    league: EntryLink::Comp(KEY_CLUB_COMP_KOREAN_SUPER_CUP),
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates Spain's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x372e and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 1 September of
/// the game year (weekday 6) to 14 June of the next year (weekday 5). 4 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): Spanish First Division, Spanish Second Division, Spanish Cup, Spanish Super
/// Cup; with the nation's extra division (`nation::LEAGUE_SELECTED` bit 2) 5, adding Spanish Second Division B (created
/// when the count is 5); every constructor gets the game year. A failed allocation: the "Memory Allocation Error" box
/// (Setup.cpp 0x374d, "create_spanish_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY`
/// = the day of Date::Date(20, 5, 2001, -1) (20 June 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the game year,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = the Spanish Super Cup record; returns
/// 1. /// Original: `create_spanish_comps` (0x837560, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_spanish_comps(entry: usize) -> u32 {
    covered!("tel13_4 spanish");
    key_nation_create_comps(entry, &SPANISH_COMPS)
}

/// The entry of `create_spanish_comps`.
const SPANISH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x372e, 0x374d, 0xa82294), // "create_spanish_comps() : unable to allocate memory"
    start: (1, 8, EntryYear::Current(0), 0x6),
    end: (14, 5, EntryYear::Current(1), 0x5),
    count: EntryCount::ExtraDivision(4, 5, ExtraGuard::CountIsLarger),
    calloc: None,
    comps: &[
        (0xf2, 0x84e130, KEY_CLUB_COMP_SPANISH_FIRST_DIVISION, EntryYear::Current(0), false), // SpaFirst::SpaFirst
        (0xee, 0x8539a0, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION, EntryYear::Current(0), false), // SpaSecond::SpaSecond
        (0xee, 0x855f40, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B, EntryYear::Current(0), true), // SpaSecondB::SpaSecondB
        (0xb2, 0x84cf50, KEY_CLUB_COMP_SPANISH_CUP, EntryYear::Current(0), false), // SpaCup::SpaCup
        (0xb2, 0x8586b0, KEY_CLUB_COMP_SPANISH_SUPER_CUP, EntryYear::Current(0), false), // SpaSuper::SpaSuper
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Comp(KEY_CLUB_COMP_SPANISH_SUPER_CUP),
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates Sweden's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x377d and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 6 April of the
/// start date year (weekday 6) to 26 October of the start date year (weekday 6). 3 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): Swedish Premier Division, Swedish First
/// Division, Swedish Cup; with the nation's extra division (`nation::LEAGUE_SELECTED` bit 2) 4, adding Swedish Second
/// Division (created when the count is 4); the constructors get the game year (the start date year: Swedish Premier
/// Division, Swedish First Division, Swedish Second Division). A failed allocation: the "Memory Allocation Error" box
/// (Setup.cpp 0x379f, "create_swedish_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY`
/// = the day of Date::Date(27, 11, 2001, -1) (27 December 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the start date year,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original:
/// `create_swedish_comps` (0x8379f0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_swedish_comps(entry: usize) -> u32 {
    covered!("tel13_4 swedish");
    key_nation_create_comps(entry, &SWEDISH_COMPS)
}

/// The entry of `create_swedish_comps`.
const SWEDISH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x377d, 0x379f, 0xa822c8), // "create_swedish_comps() : unable to allocate memory"
    start: (6, 3, EntryYear::Start(0), 0x6),
    end: (26, 9, EntryYear::Start(0), 0x6),
    count: EntryCount::ExtraDivision(3, 4, ExtraGuard::CountIsLarger),
    calloc: None,
    comps: &[
        (0xee, 0x88fb10, KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION, EntryYear::Start(0), false), // SwePrm::SwePrm
        (0xee, 0x88e5e0, KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION, EntryYear::Start(0), false), // SweFirst::SweFirst
        (0x11e, 0x8916f0, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION, EntryYear::Start(0), true), // SweSecond::SweSecond
        (0xb8, 0x88cbb0, KEY_CLUB_COMP_SWEDISH_CUP, EntryYear::Current(0), false), // SweCup::SweCup
    ],
    end_day: (27, 11),
    season_year: EntryYear::Start(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates Turkey's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x37c9 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 8 August of
/// the game year (weekday 6) to 21 May of the next year (weekday 6). 4 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): Turkish Premier Division, Turkish 2 Division Category A, Turkish 2 Division
/// Category B, Turkish FA Cup; every constructor gets the game year. A failed allocation: the "Memory Allocation Error"
/// box (Setup.cpp 0x37e5, "create_spanish_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(20, 5, 2001, -1) (20 June 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the game year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = null; returns 1. The original's allocation message names another routine
/// (create_spanish_comps()). /// Original: `create_turkish_comps` (0x837d70, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_turkish_comps(entry: usize) -> u32 {
    covered!("tel13_4 turkish");
    key_nation_create_comps(entry, &TURKISH_COMPS)
}

/// The entry of `create_turkish_comps`.
const TURKISH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x37c9, 0x37e5, 0xa82294), // "create_spanish_comps() : unable to allocate memory"
    start: (8, 7, EntryYear::Current(0), 0x6),
    end: (21, 4, EntryYear::Current(1), 0x6),
    count: EntryCount::Fixed(4),
    calloc: None,
    comps: &[
        (0xee, 0x8fcf60, KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION, EntryYear::Current(0), false), // TurFirst::TurFirst
        (0xee, 0x8fe6f0, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A, EntryYear::Current(0), false), // TurSecond::TurSecond
        (0xee, 0x8ff4c0, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B, EntryYear::Current(0), false), // TurSecondB::TurSecondB
        (0xb2, 0x8fc3b0, KEY_CLUB_COMP_TURKISH_FA_CUP, EntryYear::Current(0), false), // TurCup::TurCup
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates the United States on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error" box
/// 0x380b and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 22 March of the
/// start date year (weekday 5) to 6 January of the year after the start date (weekday 2). 3 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): American Major League, US Open Cup,
/// American MLS All Stars; every constructor gets the start date year. A failed allocation: the "Memory Allocation
/// Error" box (Setup.cpp 0x3827, "create_united_states_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(20, 1, 2001, -1) (20 February 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the start date year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original: `create_united_states_comps` (0x8380c0, Setup.cpp,
/// cdecl).
pub(crate) unsafe extern "C" fn create_united_states_comps(entry: usize) -> u32 {
    covered!("tel13_4 united states");
    key_nation_create_comps(entry, &UNITED_STATES_COMPS)
}

/// The entry of `create_united_states_comps`.
const UNITED_STATES_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x380b, 0x3827, 0xa822fc), // "create_united_states_comps() : unable to allocate memory"
    start: (22, 2, EntryYear::Start(0), 0x5),
    end: (6, 0, EntryYear::Start(1), 0x2),
    count: EntryCount::Fixed(3),
    calloc: None,
    comps: &[
        (0xee, 0x906f80, KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE, EntryYear::Start(0), false), // UsaMls::UsaMls
        (0xb2, 0x90aeb0, KEY_CLUB_COMP_US_OPEN_CUP, EntryYear::Start(0), false), // UsaOpenCup::UsaOpenCup
        (0xb2, 0x90a150, KEY_CLUB_COMP_AMERICAN_MLS_ALL_STARS, EntryYear::Start(0), false), // UsaMlsAllStars::UsaMlsAllStars
    ],
    end_day: (20, 1),
    season_year: EntryYear::Start(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates Wales's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x384c and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 19 August of
/// the game year (weekday 5) to 6 May of the next year (weekday 5). 4 competitions (`KEY_NATION_RECORD_COMP_COUNT`, the
/// array in `KEY_NATION_RECORD_COMPS`): Welsh Premier Division, Welsh Cup, Welsh League Cup, Welsh Premier Cup; every
/// constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x3868,
/// "create_spanish_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(20, 5, 2001, -1) (20 June 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the game year,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` is not written; returns 1. The
/// original's allocation message names another routine (create_spanish_comps()). /// Original: `create_welsh_comps`
/// (0x8383b0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_welsh_comps(entry: usize) -> u32 {
    covered!("tel13_4 welsh");
    key_nation_create_comps(entry, &WELSH_COMPS)
}

/// The entry of `create_welsh_comps`.
const WELSH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x384c, 0x3868, 0xa82294), // "create_spanish_comps() : unable to allocate memory"
    start: (19, 7, EntryYear::Current(0), 0x5),
    end: (6, 4, EntryYear::Current(1), 0x5),
    count: EntryCount::Fixed(4),
    calloc: None,
    comps: &[
        (0xee, 0x924e30, KEY_CLUB_COMP_WELSH_PREMIER_DIVISION, EntryYear::Current(0), false), // WelFirst::WelFirst
        (0xb2, 0x924420, KEY_CLUB_COMP_WELSH_CUP, EntryYear::Current(0), false), // WelCup::WelCup
        (0xf6, 0x925e90, KEY_CLUB_COMP_WELSH_LEAGUE_CUP, EntryYear::Current(0), false), // WelLgeCup::WelLgeCup
        (0xf6, 0x9277f0, KEY_CLUB_COMP_WELSH_PREMIER_CUP, EntryYear::Current(0), false), // WelPrmCup::WelPrmCup
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Untouched,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Comparer of `create_norwegian_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_norwegian_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I0%%)(entry);
    key_nation_check_comps("create_norwegian_comps", entry, r, &NORWEGIAN_COMPS);
    r
}

/// Comparer of `create_polish_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_polish_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I1%%)(entry);
    key_nation_check_comps("create_polish_comps", entry, r, &POLISH_COMPS);
    r
}

/// Comparer of `create_portuguese_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_portuguese_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I2%%)(entry);
    key_nation_check_comps("create_portuguese_comps", entry, r, &PORTUGUESE_COMPS);
    r
}

/// Comparer of `create_russian_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_russian_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I3%%)(entry);
    key_nation_check_comps("create_russian_comps", entry, r, &RUSSIAN_COMPS);
    r
}

/// Comparer of `create_scottish_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_scottish_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I4%%)(entry);
    key_nation_check_comps("create_scottish_comps", entry, r, &SCOTTISH_COMPS);
    r
}

/// Comparer of `create_south_korean_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_south_korean_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I5%%)(entry);
    key_nation_check_comps("create_south_korean_comps", entry, r, &SOUTH_KOREAN_COMPS);
    r
}

/// Comparer of `create_spanish_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_spanish_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I6%%)(entry);
    key_nation_check_comps("create_spanish_comps", entry, r, &SPANISH_COMPS);
    r
}

/// Comparer of `create_swedish_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_swedish_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I7%%)(entry);
    key_nation_check_comps("create_swedish_comps", entry, r, &SWEDISH_COMPS);
    r
}

/// Comparer of `create_turkish_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_turkish_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I8%%)(entry);
    key_nation_check_comps("create_turkish_comps", entry, r, &TURKISH_COMPS);
    r
}

/// Comparer of `create_united_states_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_united_states_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I9%%)(entry);
    key_nation_check_comps("create_united_states_comps", entry, r, &UNITED_STATES_COMPS);
    r
}

/// Comparer of `create_welsh_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_welsh_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I10%%)(entry);
    key_nation_check_comps("create_welsh_comps", entry, r, &WELSH_COMPS);
    r
}
