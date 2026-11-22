// ---------- batch %%LEVA%%: the key nation competitions, part 2: Croatia to Northern Ireland (Setup.cpp) ----------

// (uses the mould of tel13_2: KeyNationComps, key_nation_create_comps, key_nation_check_comps)

/// Creates Croatia's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x3239 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 24 July of the
/// game year (weekday 5) to 14 May of the next year (weekday 5). 4 competitions (`KEY_NATION_RECORD_COMP_COUNT`, the
/// array in `KEY_NATION_RECORD_COMPS`): Croatian First Division, Croatian Second Division North, Croatian Second
/// Division South, Croatian Cup; every constructor gets the game year. A failed allocation: the "Memory Allocation
/// Error" box (Setup.cpp 0x3256, "create_croatian_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(20, 5, 2001, -1) (20 June 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the game year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original: `create_croatian_comps` (0x8330b0, Setup.cpp,
/// cdecl).
pub(crate) unsafe extern "C" fn create_croatian_comps(entry: usize) -> u32 {
    covered!("tel13_3 croatian");
    key_nation_create_comps(entry, &CROATIAN_COMPS)
}

/// The entry of `create_croatian_comps`.
const CROATIAN_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x3239, 0x3256, 0xa81f7c), // "create_croatian_comps() : unable to allocate memory"
    start: (24, 6, EntryYear::Current(0), 0x5),
    end: (14, 4, EntryYear::Current(1), 0x5),
    count: EntryCount::Fixed(4),
    calloc: None,
    comps: &[
        (0xef, 0x513e10, KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4, EntryYear::Current(0), false), // CroA1::CroA1
        (0xef, 0x515310, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH, EntryYear::Current(0), false), // CroA2a::CroA2a
        (0xef, 0x516430, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH, EntryYear::Current(0), false), // CroA2b::CroA2b
        (0xb2, 0x517480, KEY_CLUB_COMP_CROATIAN_CUP, EntryYear::Current(0), false), // CroCup::CroCup
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates Denmark's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x327e and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 27 July of the
/// game year (weekday 6) to 14 June of the next year (weekday 6). 4 competitions (`KEY_NATION_RECORD_COMP_COUNT`, the
/// array in `KEY_NATION_RECORD_COMPS`): Danish Premier Division, Danish First Division, Danish Second Division, Danish
/// Cup; every constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x329a,
/// "create_danish_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(20, 5, 2001, -1) (20 June 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the game year,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original:
/// `create_danish_comps` (0x833400, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_danish_comps(entry: usize) -> u32 {
    covered!("tel13_3 danish");
    key_nation_create_comps(entry, &DANISH_COMPS)
}

/// The entry of `create_danish_comps`.
const DANISH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x327e, 0x329a, 0xa81fb0), // "create_danish_comps() : unable to allocate memory"
    start: (27, 6, EntryYear::Current(0), 0x6),
    end: (14, 5, EntryYear::Current(1), 0x6),
    count: EntryCount::Fixed(4),
    calloc: None,
    comps: &[
        (0xee, 0x5525e0, KEY_CLUB_COMP_DANISH_PREMIER_DIVISION, EntryYear::Current(0), false), // DenPrm::DenPrm
        (0xee, 0x5516f0, KEY_CLUB_COMP_DANISH_FIRST_DIVISION, EntryYear::Current(0), false), // DenFirst::DenFirst
        (0xee, 0x553e80, KEY_CLUB_COMP_DANISH_SECOND_DIVISION, EntryYear::Current(0), false), // DenSecond::DenSecond
        (0xb2, 0x5509b0, KEY_CLUB_COMP_DANISH_CUP, EntryYear::Current(0), false), // DenCup::DenCup
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates England's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x32c1 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 10 August of
/// the game year (weekday 5) to 24 May of the next year (weekday 5). 8 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): English Premier Division, English First Division, English Second Division,
/// English Third Division, English FA Cup, English League Cup, English Autoglass Trophy, English Charity Shield; with
/// the nation's extra division (`nation::LEAGUE_SELECTED` bit 2) 10, adding English Conference, English FA Trophy
/// (created when the count is 10); every constructor gets the game year. A failed allocation: the "Memory Allocation
/// Error" box (Setup.cpp 0x32e0, "create_english_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(20, 5, 2001, -1) (20 June 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the game year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = the English Charity Shield record; returns 1. /// Original:
/// `create_english_comps` (0x833750, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_english_comps(entry: usize) -> u32 {
    covered!("tel13_3 english");
    key_nation_create_comps(entry, &ENGLISH_COMPS)
}

/// The entry of `create_english_comps`.
const ENGLISH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x32c1, 0x32e0, 0xa81fe4), // "create_english_comps() : unable to allocate memory"
    start: (10, 7, EntryYear::Current(0), 0x5),
    end: (24, 4, EntryYear::Current(1), 0x5),
    count: EntryCount::ExtraDivision(8, 10, ExtraGuard::CountIsLarger),
    calloc: None,
    comps: &[
        (0xee, 0x573380, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, EntryYear::Current(0), false), // EngPrm::EngPrm
        (0xee, 0x571680, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, EntryYear::Current(0), false), // EngFirst::EngFirst
        (0xee, 0x5754a0, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, EntryYear::Current(0), false), // EngSecond::EngSecond
        (0xee, 0x576dd0, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, EntryYear::Current(0), false), // EngThird::EngThird
        (0xee, 0x56db60, KEY_CLUB_COMP_ENGLISH_CONFERENCE, EntryYear::Current(0), true), // EngConf::EngConf
        (0xb5, 0x56f040, KEY_CLUB_COMP_ENGLISH_FA_CUP, EntryYear::Current(0), false), // EngFaCup::EngFaCup
        (0xb3, 0x56c2a0, KEY_CLUB_COMP_ENGLISH_LEAGUE_CUP, EntryYear::Current(0), false), // EngCcCup::EngCcCup
        (0xba, 0x56aa30, KEY_CLUB_COMP_ENGLISH_AUTOGLASS_TROPHY, EntryYear::Current(0), false), // EngAutoCup::EngAutoCup
        (0xb2, 0x56d380, KEY_CLUB_COMP_ENGLISH_CHARITY_SHIELD, EntryYear::Current(0), false), // EngCharity::EngCharity
        (0xb2, 0x570c00, KEY_CLUB_COMP_ENGLISH_FA_TROPHY, EntryYear::Current(0), true), // EngFaTrophy::EngFaTrophy
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Comp(KEY_CLUB_COMP_ENGLISH_CHARITY_SHIELD),
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates Finland's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x3311 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 29 April of
/// the start date year (weekday 5) to 12 November of the next year (weekday 6). 3 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): Finnish Premier Division, Finnish First
/// Division, Finnish Cup; every constructor gets the start date year. A failed allocation: the "Memory Allocation
/// Error" box (Setup.cpp 0x332d, "create_finnish_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(27, 11, 2001, -1) (27 December 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the start date year, `key_nation_record::SEASON_SPANS_YEAR` = 0;
/// `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original: `create_finnish_comps` (0x833cf0, Setup.cpp,
/// cdecl).
pub(crate) unsafe extern "C" fn create_finnish_comps(entry: usize) -> u32 {
    covered!("tel13_3 finnish");
    key_nation_create_comps(entry, &FINNISH_COMPS)
}

/// The entry of `create_finnish_comps`.
const FINNISH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x3311, 0x332d, 0xa82018), // "create_finnish_comps() : unable to allocate memory"
    start: (29, 3, EntryYear::Start(0), 0x5),
    end: (12, 10, EntryYear::Current(1), 0x6),
    count: EntryCount::Fixed(3),
    calloc: None,
    comps: &[
        (0xee, 0x595170, KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION, EntryYear::Start(0), false), // FinPrm::FinPrm
        (0xfe, 0x592eb0, KEY_CLUB_COMP_FINNISH_FIRST_DIVISION, EntryYear::Start(0), false), // FinFirst::FinFirst
        (0xb2, 0x592250, KEY_CLUB_COMP_FINNISH_CUP, EntryYear::Start(0), false), // FinCup::FinCup
    ],
    end_day: (27, 11),
    season_year: EntryYear::Start(0),
    spans: 0,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates France's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x3352 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 9 August of
/// the game year (weekday 5) to 24 May of the next year (weekday 5). 6 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): French First Division, French Second Division, French National, French Cup,
/// French League Cup, French Champions Trophy; every constructor gets the game year. A failed allocation: the "Memory
/// Allocation Error" box (Setup.cpp 0x336e, "create_french_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(20, 5, 2001, -1) (20 June 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the game year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = the French Champions Trophy record; returns 1. /// Original:
/// `create_french_comps` (0x833ff0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_french_comps(entry: usize) -> u32 {
    covered!("tel13_3 french");
    key_nation_create_comps(entry, &FRENCH_COMPS)
}

/// The entry of `create_french_comps`.
const FRENCH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x3352, 0x336e, 0xa8204c), // "create_french_comps() : unable to allocate memory"
    start: (9, 7, EntryYear::Current(0), 0x5),
    end: (24, 4, EntryYear::Current(1), 0x5),
    count: EntryCount::Fixed(6),
    calloc: None,
    comps: &[
        (0xf6, 0x5bb160, KEY_CLUB_COMP_FRENCH_FIRST_DIVISION, EntryYear::Current(0), false), // FraFirst::FraFirst
        (0xee, 0x5be330, KEY_CLUB_COMP_FRENCH_SECOND_DIVISION, EntryYear::Current(0), false), // FraSecond::FraSecond
        (0xee, 0x5bfb40, KEY_CLUB_COMP_FRENCH_NATIONAL, EntryYear::Current(0), false), // FraThird::FraThird
        (0xb2, 0x5ba130, KEY_CLUB_COMP_FRENCH_CUP, EntryYear::Current(0), false), // FraCup::FraCup
        (0xb2, 0x5bc980, KEY_CLUB_COMP_FRENCH_LEAGUE_CUP, EntryYear::Current(0), false), // FraLgeCup::FraLgeCup
        (0xb2, 0x5bf320, KEY_CLUB_COMP_FRENCH_CHAMPIONS_TROPHY, EntryYear::Current(0), false), // FraSuper::FraSuper
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Comp(KEY_CLUB_COMP_FRENCH_CHAMPIONS_TROPHY),
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates Germany's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x3397 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 10 August of
/// the game year (weekday 5) to 14 June of the next year (weekday 5). 4 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): German First Division, German Second Division, German Cup, German League
/// Cup; with the nation's extra division (`nation::LEAGUE_SELECTED` bit 2) 5, adding German Regional (created when the
/// count is 5); every constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp
/// 0x33b6, "create_german_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(20, 5, 2001, -1) (20 June 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the game year,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original:
/// `create_german_comps` (0x8343f0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_german_comps(entry: usize) -> u32 {
    covered!("tel13_3 german");
    key_nation_create_comps(entry, &GERMAN_COMPS)
}

/// The entry of `create_german_comps`.
const GERMAN_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x3397, 0x33b6, 0xa82080), // "create_german_comps() : unable to allocate memory"
    start: (10, 7, EntryYear::Current(0), 0x5),
    end: (14, 5, EntryYear::Current(1), 0x5),
    count: EntryCount::ExtraDivision(4, 5, ExtraGuard::CountIsLarger),
    calloc: None,
    comps: &[
        (0xee, 0x5d99b0, KEY_CLUB_COMP_GERMAN_FIRST_DIVISION, EntryYear::Current(0), false), // GerFirst::GerFirst
        (0xee, 0x5de2e0, KEY_CLUB_COMP_GERMAN_SECOND_DIVISION, EntryYear::Current(0), false), // GerSecond::GerSecond
        (0xee, 0x5dcbd0, KEY_CLUB_COMP_GERMAN_REGIONAL, EntryYear::Current(0), true), // GerRegional::GerRegional
        (0xb2, 0x5d8c20, KEY_CLUB_COMP_GERMAN_CUP, EntryYear::Current(0), false), // GerCup::GerCup
        (0xb2, 0x5dc0e0, KEY_CLUB_COMP_GERMAN_LEAGUE_CUP, EntryYear::Current(0), false), // GerLgeCup::GerLgeCup
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates Greece's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x33e1 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 15 August of
/// the game year (weekday 5) to 21 May of the next year (weekday 5). 4 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): Greek Premier Division, Greek Second Division, Greek Cup, Greek Super Cup;
/// every constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x33fd,
/// "create_german_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(20, 5, 2001, -1) (20 June 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the game year,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = null; returns 1. The original's
/// allocation message names another routine (create_german_comps()). /// Original: `create_greek_comps` (0x8347c0,
/// Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_greek_comps(entry: usize) -> u32 {
    covered!("tel13_3 greek");
    key_nation_create_comps(entry, &GREEK_COMPS)
}

/// The entry of `create_greek_comps`.
const GREEK_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x33e1, 0x33fd, 0xa82080), // "create_german_comps() : unable to allocate memory"
    start: (15, 7, EntryYear::Current(0), 0x5),
    end: (21, 4, EntryYear::Current(1), 0x5),
    count: EntryCount::Fixed(4),
    calloc: None,
    comps: &[
        (0xee, 0x5ea410, KEY_CLUB_COMP_GREEK_PREMIER_DIVISION, EntryYear::Current(0), false), // GrePrm::GrePrm
        (0xee, 0x5eb2b0, KEY_CLUB_COMP_GREEK_SECOND_DIVISION, EntryYear::Current(0), false), // GreSecond::GreSecond
        (0xf6, 0x5e88a0, KEY_CLUB_COMP_GREEK_CUP, EntryYear::Current(0), false), // GreCup::GreCup
        (0xb2, 0x5ec040, KEY_CLUB_COMP_GREEK_SUPER_CUP, EntryYear::Current(0), false), // GreSuper::GreSuper
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates Holland's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x3423 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 16 August of
/// the game year (weekday 5) to 7 June of the next year (weekday 5). 4 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): Dutch KPN Telecompetition, Dutch Toto Division, Dutch Amstel Cup, Dutch
/// Super Cup; every constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp
/// 0x343f, "create_dutch_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(20, 5, 2001, -1) (20 June 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the game year,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original:
/// `create_dutch_comps` (0x834b10, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_dutch_comps(entry: usize) -> u32 {
    covered!("tel13_3 dutch");
    key_nation_create_comps(entry, &DUTCH_COMPS)
}

/// The entry of `create_dutch_comps`.
const DUTCH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x3423, 0x343f, 0xa820b4), // "create_dutch_comps() : unable to allocate memory"
    start: (16, 7, EntryYear::Current(0), 0x5),
    end: (7, 5, EntryYear::Current(1), 0x5),
    count: EntryCount::Fixed(4),
    calloc: None,
    comps: &[
        (0xee, 0x5f7610, KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION, EntryYear::Current(0), false), // HolPrm::HolPrm
        (0xf2, 0x5f5490, KEY_CLUB_COMP_DUTCH_TOTO_DIVISION, EntryYear::Current(0), false), // HolFirst::HolFirst
        (0xf6, 0x5f3450, KEY_CLUB_COMP_DUTCH_AMSTEL_CUP, EntryYear::Current(0), false), // HolCup::HolCup
        (0xb2, 0x5f8820, KEY_CLUB_COMP_DUTCH_SUPER_CUP, EntryYear::Current(0), false), // HolSuper::HolSuper
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates the Republic of Ireland on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x3466 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 1 August of
/// the game year (weekday 6) to 23 April of the next year (weekday 6). 8 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): Irish Premier Division, Irish First Division, Irish Senior Challenge Cup,
/// Irish League Cup, Irish Super Cup, Irish Presidents Cup, Irish Munster Senior Cup, Irish Leinster Senior Cup; every
/// constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x3482,
/// "create_irish_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(20, 5, 2001, -1) (20 June 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the game year,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = the Irish Super Cup record;
/// `KEY_NATION_RECORD_CUP_COMP` = the Irish Senior Challenge Cup record; returns 1. /// Original: `create_irish_comps`
/// (0x834e60, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_irish_comps(entry: usize) -> u32 {
    covered!("tel13_3 irish");
    key_nation_create_comps(entry, &IRISH_COMPS)
}

/// The entry of `create_irish_comps`.
const IRISH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x3466, 0x3482, 0xa820e8), // "create_irish_comps() : unable to allocate memory"
    start: (1, 7, EntryYear::Current(0), 0x6),
    end: (23, 3, EntryYear::Current(1), 0x6),
    count: EntryCount::Fixed(8),
    calloc: None,
    comps: &[
        (0x120, 0x639bb0, KEY_CLUB_COMP_IRISH_PREMIER_DIVISION, EntryYear::Current(0), false), // IrePrm::IrePrm
        (0xee, 0x634c40, KEY_CLUB_COMP_IRISH_FIRST_DIVISION, EntryYear::Current(0), false), // IreFirst::IreFirst
        (0xb2, 0x6340a0, KEY_CLUB_COMP_IRISH_SENIOR_CHALLENGE_CUP, EntryYear::Current(0), false), // IreChalCup::IreChalCup
        (0xf6, 0x636dd0, KEY_CLUB_COMP_IRISH_LEAGUE_CUP, EntryYear::Current(0), false), // IreLgeCup::IreLgeCup
        (0xee, 0x63acd0, KEY_CLUB_COMP_IRISH_SUPER_CUP, EntryYear::Current(0), false), // IreSuperCup::IreSuperCup
        (0xb2, 0x639280, KEY_CLUB_COMP_IRISH_PRESIDENTS_CUP, EntryYear::Current(0), false), // IrePresCup::IrePresCup
        (0xb2, 0x6387a0, KEY_CLUB_COMP_IRISH_MUNSTER_SENIOR_CUP, EntryYear::Current(0), false), // IreMunsterCup::IreMunsterCup
        (0xb2, 0x6364a0, KEY_CLUB_COMP_IRISH_LEINSTER_SENIOR_CUP, EntryYear::Current(0), false), // IreLeinsterCup::IreLeinsterCup
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Comp(KEY_CLUB_COMP_IRISH_SUPER_CUP),
    cup: EntryLink::Comp(KEY_CLUB_COMP_IRISH_SENIOR_CHALLENGE_CUP),
    weeks: false,
};

/// Creates Italy's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x34af and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 6 September of
/// the game year (weekday 6) to 8 June of the next year (weekday 6). 8 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): Italian Serie A, Italian Serie B, Italian Serie C1 A, Italian Serie C1 B,
/// Italian Cup, Italian Serie C Cup, Italian Super Cup, Italian C1 Super Cup; with the nation's extra division
/// (`nation::LEAGUE_SELECTED` bit 2) 11, adding Italian Serie C2 A, Italian Serie C2 B, Italian Serie C2 C (created
/// when the nation has it); every constructor gets the game year. A failed allocation: the "Memory Allocation Error"
/// box (Setup.cpp 0x34ce, "create_italian_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(20, 5, 2001, -1) (20 June 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the game year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = the Italian Super Cup record; returns 1. /// Original: `create_italian_comps`
/// (0x835320, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_italian_comps(entry: usize) -> u32 {
    covered!("tel13_3 italian");
    key_nation_create_comps(entry, &ITALIAN_COMPS)
}

/// The entry of `create_italian_comps`.
const ITALIAN_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x34af, 0x34ce, 0xa8211c), // "create_italian_comps() : unable to allocate memory"
    start: (6, 8, EntryYear::Current(0), 0x6),
    end: (8, 5, EntryYear::Current(1), 0x6),
    count: EntryCount::ExtraDivision(8, 11, ExtraGuard::Flag),
    calloc: None,
    comps: &[
        (0xef, 0x6406d0, KEY_CLUB_COMP_ITALIAN_SERIE_A, EntryYear::Current(0), false), // ItaSerA::ItaSerA
        (0xef, 0x645810, KEY_CLUB_COMP_ITALIAN_SERIE_B, EntryYear::Current(0), false), // ItaSerB::ItaSerB
        (0xee, 0x64aa70, KEY_CLUB_COMP_ITALIAN_SERIE_C1_A, EntryYear::Current(0), false), // ItaSerC1a::ItaSerC1a
        (0xee, 0x64ec20, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B, EntryYear::Current(0), false), // ItaSerC1b::ItaSerC1b
        (0xee, 0x652e90, KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, EntryYear::Current(0), true), // ItaSerC2a::ItaSerC2a
        (0xee, 0x657350, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B, EntryYear::Current(0), true), // ItaSerC2b::ItaSerC2b
        (0xee, 0x65b4e0, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C, EntryYear::Current(0), true), // ItaSerC2c::ItaSerC2c
        (0xf6, 0x63e900, KEY_CLUB_COMP_ITALIAN_CUP, EntryYear::Current(0), false), // ItaCup::ItaCup
        (0xf6, 0x63cc40, KEY_CLUB_COMP_ITALIAN_SERIE_C_CUP, EntryYear::Current(0), false), // ItaCCup::ItaCCup
        (0xb2, 0x65f670, KEY_CLUB_COMP_ITALIAN_SUPER_CUP, EntryYear::Current(0), false), // ItaSuper::ItaSuper
        (0xb2, 0x63c5e0, KEY_CLUB_COMP_ITALIAN_C1_SUPER_CUP, EntryYear::Current(0), false), // ItaC1Super::ItaC1Super
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Comp(KEY_CLUB_COMP_ITALIAN_SUPER_CUP),
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates Japan's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x3502 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 14 March of
/// the start date year (weekday 5) to 28 November of the start date year (weekday 5). 5 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): Japanese J League 1, Japanese J League 2,
/// Japanese Emperor's Cup, Japanese Yamazaki Nabisco Cup, Japanese Xerox Super Cup; every constructor gets the start
/// date year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x351e, "create_japanese_comps() :
/// unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of Date::Date(25, 0, 2001, -1) (25
/// January 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the start date year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original: `create_japanese_comps` (0x835900, Setup.cpp,
/// cdecl).
pub(crate) unsafe extern "C" fn create_japanese_comps(entry: usize) -> u32 {
    covered!("tel13_3 japanese");
    key_nation_create_comps(entry, &JAPANESE_COMPS)
}

/// The entry of `create_japanese_comps`.
const JAPANESE_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x3502, 0x351e, 0xa82150), // "create_japanese_comps() : unable to allocate memory"
    start: (14, 2, EntryYear::Start(0), 0x5),
    end: (28, 10, EntryYear::Start(0), 0x5),
    count: EntryCount::Fixed(5),
    calloc: None,
    comps: &[
        (0xee, 0x662b30, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1, EntryYear::Start(0), false), // JapJ1::JapJ1
        (0xee, 0x6644c0, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2, EntryYear::Start(0), false), // JapJ2::JapJ2
        (0xb2, 0x661d30, KEY_CLUB_COMP_JAPANESE_EMPEROR_S_CUP, EntryYear::Start(0), false), // JapEmpCup::JapEmpCup
        (0xb2, 0x665380, KEY_CLUB_COMP_JAPANESE_YAMAZAKI_NABISCO_CUP, EntryYear::Start(0), false), // JapJCup::JapJCup
        (0xb2, 0x665d70, KEY_CLUB_COMP_JAPANESE_XEROX_SUPER_CUP, EntryYear::Start(0), false), // JapSuper::JapSuper
    ],
    end_day: (25, 0),
    season_year: EntryYear::Start(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates Northern Ireland's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp
/// "Error" box 0x3546 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 12
/// August of the game year (weekday 5) to 5 May of the next year (weekday 5). 5 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): Northern Ireland Premier Division,
/// Northern Ireland First Division, Northern Irish Cup, Northern Irish League Cup, Northern Irish Charity Shield; every
/// constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x3562,
/// "create_northern_irish_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(20, 5, 2001, -1) (20 June 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the game year,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = the Northern Irish Charity Shield
/// record; `KEY_NATION_RECORD_CUP_COMP` = the Northern Irish Cup record; returns 1. /// Original:
/// `create_northern_irish_comps` (0x835c90, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_northern_irish_comps(entry: usize) -> u32 {
    covered!("tel13_3 northern irish");
    key_nation_create_comps(entry, &NORTHERN_IRISH_COMPS)
}

/// The entry of `create_northern_irish_comps`.
const NORTHERN_IRISH_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x3546, 0x3562, 0xa82184), // "create_northern_irish_comps() : unable to allocate memory"
    start: (12, 7, EntryYear::Current(0), 0x5),
    end: (5, 4, EntryYear::Current(1), 0x5),
    count: EntryCount::Fixed(5),
    calloc: None,
    comps: &[
        (0xee, 0x78e070, KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION, EntryYear::Current(0), false), // NirPrm::NirPrm
        (0xee, 0x78b010, KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION, EntryYear::Current(0), false), // NirFirst::NirFirst
        (0xb2, 0x78a5b0, KEY_CLUB_COMP_NORTHERN_IRISH_CUP, EntryYear::Current(0), false), // NirCup::NirCup
        (0xf6, 0x78c780, KEY_CLUB_COMP_NORTHERN_IRISH_LEAGUE_CUP, EntryYear::Current(0), false), // NirLgeCup::NirLgeCup
        (0xb2, 0x789df0, KEY_CLUB_COMP_NORTHERN_IRISH_CHARITY_SHIELD, EntryYear::Current(0), false), // NirCharity::NirCharity
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Comp(KEY_CLUB_COMP_NORTHERN_IRISH_CHARITY_SHIELD),
    cup: EntryLink::Comp(KEY_CLUB_COMP_NORTHERN_IRISH_CUP),
    weeks: false,
};

/// Comparer of `create_croatian_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_croatian_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I0%%)(entry);
    key_nation_check_comps("create_croatian_comps", entry, r, &CROATIAN_COMPS);
    r
}

/// Comparer of `create_danish_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_danish_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I1%%)(entry);
    key_nation_check_comps("create_danish_comps", entry, r, &DANISH_COMPS);
    r
}

/// Comparer of `create_english_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_english_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I2%%)(entry);
    key_nation_check_comps("create_english_comps", entry, r, &ENGLISH_COMPS);
    r
}

/// Comparer of `create_finnish_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_finnish_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I3%%)(entry);
    key_nation_check_comps("create_finnish_comps", entry, r, &FINNISH_COMPS);
    r
}

/// Comparer of `create_french_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_french_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I4%%)(entry);
    key_nation_check_comps("create_french_comps", entry, r, &FRENCH_COMPS);
    r
}

/// Comparer of `create_german_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_german_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I5%%)(entry);
    key_nation_check_comps("create_german_comps", entry, r, &GERMAN_COMPS);
    r
}

/// Comparer of `create_greek_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_greek_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I6%%)(entry);
    key_nation_check_comps("create_greek_comps", entry, r, &GREEK_COMPS);
    r
}

/// Comparer of `create_dutch_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_dutch_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I7%%)(entry);
    key_nation_check_comps("create_dutch_comps", entry, r, &DUTCH_COMPS);
    r
}

/// Comparer of `create_irish_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_irish_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I8%%)(entry);
    key_nation_check_comps("create_irish_comps", entry, r, &IRISH_COMPS);
    r
}

/// Comparer of `create_italian_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_italian_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I9%%)(entry);
    key_nation_check_comps("create_italian_comps", entry, r, &ITALIAN_COMPS);
    r
}

/// Comparer of `create_japanese_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_japanese_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I10%%)(entry);
    key_nation_check_comps("create_japanese_comps", entry, r, &JAPANESE_COMPS);
    r
}

/// Comparer of `create_northern_irish_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_northern_irish_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I11%%)(entry);
    key_nation_check_comps("create_northern_irish_comps", entry, r, &NORTHERN_IRISH_COMPS);
    r
}
