// ---------- batch %%LEVA%%: the key nation competitions, part 1: the international ones, Argentina, Australia, Belgium and Brazil (Setup.cpp) ----------

// (uses setup_warning and save_error_box of setup.rs and the key nation entry names of tel13_1)
// The 35 create_*_comps routines (one per G_LEAGUE_TABLE row; key_nation_create_all_comps calls the row's
// KEY_NATION_RECORD_SETUP_FUNC for an international row or a nation with a selected league) are one mould with
// different data: KeyNationComps holds a routine's data and key_nation_create_comps is the mould. Checked routine by
// routine against the original by emulating it (every write to the entry, every allocation and constructor call with
// its arguments, with and without the extra division and the continent's selected nation).

/// The year a key nation entry value takes: the game year (`G_CURRENT_DATE_YEAR`) or the game start date's
/// (`G_GAME_START_DATE_YEAR`) plus an offset, the start date moved 4 weeks on (`Date::add_weeks`, Brazil: in a
/// constructor's dword the year and the date's next word), or a fixed year.
#[derive(Clone, Copy)]
enum EntryYear {
    Current(u16),
    Start(u16),
    StartPlus4Weeks,
    Fixed(u16),
}

/// What the original tests before a run of extra-division competitions: the count it wrote is the larger one, or the
/// nation's bit again.
#[derive(Clone, Copy)]
enum ExtraGuard {
    CountIsLarger,
    Flag,
}

/// How many competitions an entry gets.
#[derive(Clone, Copy)]
enum EntryCount {
    Fixed(u32),
    /// Only when a nation of the entry's continent (`KEY_NATION_RECORD_CONTINENT`) has a selected league
    /// (`key_nation_continent_has_selected_nation`); otherwise the count and the array are 0.
    Continental(u32),
    /// (without, with) the nation's extra division (`nation::LEAGUE_SELECTED` bit 2); the list's extra competitions are
    /// only created when the guard holds.
    ExtraDivision(u32, u32, ExtraGuard),
}

/// A pointer field of the entry: not written, null, or the competition record of a key.
#[derive(Clone, Copy)]
enum EntryLink {
    Untouched,
    Null,
    Comp(usize),
}

/// One competition object: (operator new size, constructor thiscall(this, year dword, competition record) ret 8, the
/// competition key, the constructor's year, extra division only).
type EntryComp = (u32, usize, usize, EntryYear, bool);

/// A season bound as the Date::Date arguments the entry keeps: (day, month from 0, year, weekday).
type SeasonBound = (u16, u8, EntryYear, u16);

/// What one `create_*_comps` routine writes on its key nation entry.
struct KeyNationComps {
    /// the Setup.cpp rows of the null entry box and of the allocation box, and the allocation box's text
    rows: (u32, u32, usize),
    /// the season start (`KEY_NATION_RECORD_START_DAY`, `key_nation_record::START_MONTH`, `.._START_YEAR`,
    /// `.._START_WEEKDAY`; key_nation_create_all_comps turns them into `key_nation_record::START_DATE`)
    start: SeasonBound,
    /// the season end (`KEY_NATION_RECORD_END_DAY`.. -> `key_nation_record::END_DATE`)
    end: SeasonBound,
    count: EntryCount,
    /// the calloc arguments when they are not (count × 4, 1)
    calloc: Option<(u32, u32)>,
    comps: &'static [EntryComp],
    /// Date::Date(day, month, 2001, −1), whose day goes to `key_nation_record::SEASON_END_DAY`
    end_day: (u32, u32),
    /// `KEY_NATION_RECORD_SEASON_YEAR`
    season_year: EntryYear,
    /// `key_nation_record::SEASON_SPANS_YEAR`
    spans: u8,
    /// `key_nation_record::LEAGUE_COMP`
    league: EntryLink,
    /// `KEY_NATION_RECORD_CUP_COMP`
    cup: EntryLink,
    /// the routine first moves the start date 4 weeks on (Brazil)
    weeks: bool,
}

/// The competition record of a key: `G_CLUB_COMP_TABLE` + [key] × 0x6b.
unsafe fn key_comp_record(key: usize) -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_u32(key) as usize).wrapping_mul(0x6b))
}

/// The value of an `EntryYear` as the dword a constructor gets (the original's high word is stack leftover, here 0,
/// except for the 4 weeks date, read as a dword from its year on); the 16-bit fields take its low word.
unsafe fn entry_year(year: EntryYear, weeks: &[u8; 8]) -> u32 {
    match year {
        EntryYear::Current(d) => read_u16(G_CURRENT_DATE_YEAR).wrapping_add(d) as u32,
        EntryYear::Start(d) => read_u16(G_GAME_START_DATE_YEAR).wrapping_add(d) as u32,
        EntryYear::StartPlus4Weeks => read_u32(weeks.as_ptr() as usize + cm_date::YEAR),
        EntryYear::Fixed(y) => y as u32,
    }
}

/// The value an `EntryLink` writes, or None when it writes nothing.
unsafe fn entry_link(link: EntryLink) -> Option<usize> {
    match link {
        EntryLink::Untouched => None,
        EntryLink::Null => Some(0),
        EntryLink::Comp(key) => Some(key_comp_record(key)),
    }
}

/// Whether the entry's nation has its extra division (bit 2 of `nation::LEAGUE_SELECTED`).
unsafe fn key_nation_extra_division(entry: usize) -> bool {
    Nation(read_usize(entry + KEY_NATION_RECORD_NATION)).league_selected() & 4 != 0
}

/// The competition count an entry gets, or None (a continental entry without a selected nation).
unsafe fn key_nation_comp_count(entry: usize, spec: &KeyNationComps) -> Option<u32> {
    match spec.count {
        EntryCount::Fixed(n) => Some(n),
        EntryCount::Continental(n) => {
            (exe::key_nation_continent_has_selected_nation(read_usize(entry + KEY_NATION_RECORD_CONTINENT)) != 0).then_some(n)
        }
        EntryCount::ExtraDivision(small, large, _) => Some(if key_nation_extra_division(entry) { large } else { small }),
    }
}

/// Whether a run of extra-division competitions is created (tested once before the run, as the original).
unsafe fn key_nation_extra_guard(entry: usize, spec: &KeyNationComps) -> bool {
    match spec.count {
        EntryCount::ExtraDivision(_, large, ExtraGuard::CountIsLarger) => read_u32(entry + KEY_NATION_RECORD_COMP_COUNT) == large,
        EntryCount::ExtraDivision(_, _, ExtraGuard::Flag) => key_nation_extra_division(entry),
        _ => true,
    }
}

/// The 4 weeks date of an entry that uses it (`Date::add_weeks`(G_GAME_START_DATE, 4)), zeroes otherwise.
unsafe fn key_nation_weeks(spec: &KeyNationComps) -> [u8; 8] {
    let mut weeks = [0u8; 8];
    if spec.weeks {
        exe::date_add_weeks(G_GAME_START_DATE, &mut weeks, 4);
    }
    weeks
}

/// The day of Date::Date(day, month, 2001, −1) (the season end of every entry is a 2001 date).
unsafe fn key_nation_end_day(spec: &KeyNationComps) -> u16 {
    let mut d = [0u16; 4];
    exe::date_date_549f70(&mut d, spec.end_day.0, spec.end_day.1, 0x7d1, 0xffff_ffff);
    d[0]
}

/// The mould of the `create_*_comps` routines: fills the key nation entry `entry` from `spec`; returns 1, or 0 with a
/// null entry (the Setup.cpp "Error" box) or when the array cannot be allocated (the "Memory Allocation Error" box).
///
/// The season bounds are written; the count (`key_nation_comp_count`) goes to `KEY_NATION_RECORD_COMP_COUNT` and the
/// array (calloc) to `KEY_NATION_RECORD_COMPS` (a continental entry without a selected nation gets 0 in both and no
/// competition); each competition of the list (the extra ones only when `key_nation_extra_guard`) is `operator new`
/// and its constructor (year, the key's competition record), a failed `new` leaving the slot null, in consecutive
/// slots. Then `key_nation_record::SEASON_END_DAY`, `KEY_NATION_RECORD_SEASON_YEAR`,
/// `key_nation_record::SEASON_SPANS_YEAR` and the links. The original's SEH frame (which deletes an object whose
/// constructor throws) has no counterpart: the constructors do not throw.
unsafe fn key_nation_create_comps(entry: usize, spec: &KeyNationComps) -> u32 {
    if entry == 0 {
        setup_warning(spec.rows.0);
        return 0;
    }
    let weeks = key_nation_weeks(spec);
    let year = |y: EntryYear| entry_year(y, &weeks);
    let (start, end) = (spec.start, spec.end);
    write_u16(entry + KEY_NATION_RECORD_START_DAY, start.0);
    write_u8(entry + key_nation_record::START_MONTH, start.1);
    write_u16(entry + KEY_NATION_RECORD_START_YEAR, year(start.2) as u16);
    write_u16(entry + KEY_NATION_RECORD_START_WEEKDAY, start.3);
    write_u16(entry + KEY_NATION_RECORD_END_DAY, end.0);
    write_u8(entry + KEY_NATION_RECORD_END_MONTH, end.1);
    write_u16(entry + KEY_NATION_RECORD_END_YEAR, year(end.2) as u16);
    write_u16(entry + KEY_NATION_RECORD_END_WEEKDAY, end.3);
    match key_nation_comp_count(entry, spec) {
        None => {
            write_u32(entry + KEY_NATION_RECORD_COMP_COUNT, 0);
            write_usize(entry + KEY_NATION_RECORD_COMPS, 0);
        }
        Some(count) => {
            write_u32(entry + KEY_NATION_RECORD_COMP_COUNT, count);
            let (size, number) = spec.calloc.unwrap_or((read_u32(entry + KEY_NATION_RECORD_COMP_COUNT).wrapping_mul(4), 1));
            let comps = exe::crt_calloc(size, number);
            write_usize(entry + KEY_NATION_RECORD_COMPS, comps);
            if comps == 0 {
                save_error_box(spec.rows.1, spec.rows.2, 0x9a8108, 0x9a7ff8); // "Memory Allocation Error"
                return 0;
            }
            let (mut slot, mut guard) = (0usize, None::<bool>);
            for &(size, constructor, key, comp_year, extra) in spec.comps {
                if extra {
                    if !*guard.get_or_insert_with(|| key_nation_extra_guard(entry, spec)) {
                        continue;
                    }
                } else {
                    guard = None;
                }
                let p = exe::crt_operator_new(size);
                let object = if p == 0 {
                    0
                } else {
                    let construct: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(constructor);
                    construct(p, year(comp_year), key_comp_record(key))
                };
                write_usize(read_usize(entry + KEY_NATION_RECORD_COMPS) + slot * 4, object);
                slot += 1;
            }
        }
    }
    let e = KeyNationRecord(entry);
    e.set_season_end_day(key_nation_end_day(spec));
    write_u16(entry + KEY_NATION_RECORD_SEASON_YEAR, year(spec.season_year) as u16);
    e.set_season_spans_year(spec.spans);
    if let Some(v) = entry_link(spec.league) {
        e.set_league_comp(v);
    }
    if let Some(v) = entry_link(spec.cup) {
        write_usize(entry + KEY_NATION_RECORD_CUP_COMP, v);
    }
    1
}

/// The plan of a `create_*_comps` routine checked on the entry after the original ran (`result` its result; nothing to
/// check with a null entry or a failed allocation): the season bounds, the count, each slot's competition (a non-null
/// object's `league::COMPETITION` is the key's record; the extra ones by the same guard), the season end day, the
/// season year, the spans flag and the links.
unsafe fn key_nation_check_comps(name: &str, entry: usize, result: u32, spec: &KeyNationComps) {
    if entry == 0 || result == 0 {
        return;
    }
    let weeks = key_nation_weeks(spec);
    let year = |y: EntryYear| entry_year(y, &weeks) as u16 as u32;
    let mut wrong: Vec<String> = Vec::new();
    let mut check = |what: &str, got: u32, plan: u32| {
        if got != plan {
            wrong.push(format!("{} {:#x} (plan {:#x})", what, got, plan));
        }
    };
    let (start, end) = (spec.start, spec.end);
    check("start day", read_u16(entry + KEY_NATION_RECORD_START_DAY) as u32, start.0 as u32);
    check("start month", read_u8(entry + key_nation_record::START_MONTH) as u32, start.1 as u32);
    check("start year", read_u16(entry + KEY_NATION_RECORD_START_YEAR) as u32, year(start.2));
    check("start weekday", read_u16(entry + KEY_NATION_RECORD_START_WEEKDAY) as u32, start.3 as u32);
    check("end day", read_u16(entry + KEY_NATION_RECORD_END_DAY) as u32, end.0 as u32);
    check("end month", read_u8(entry + KEY_NATION_RECORD_END_MONTH) as u32, end.1 as u32);
    check("end year", read_u16(entry + KEY_NATION_RECORD_END_YEAR) as u32, year(end.2));
    check("end weekday", read_u16(entry + KEY_NATION_RECORD_END_WEEKDAY) as u32, end.3 as u32);
    let count = key_nation_comp_count(entry, spec);
    check("count", read_u32(entry + KEY_NATION_RECORD_COMP_COUNT), count.unwrap_or(0));
    let comps = read_usize(entry + KEY_NATION_RECORD_COMPS);
    if count.is_none() {
        check("array", comps as u32, 0);
    } else if comps != 0 {
        let (mut slot, mut guard) = (0usize, None::<bool>);
        for &(_, _, key, _, extra) in spec.comps {
            if extra {
                if !*guard.get_or_insert_with(|| key_nation_extra_guard(entry, spec)) {
                    continue;
                }
            } else {
                guard = None;
            }
            let object = read_usize(comps + slot * 4);
            if object != 0 {
                check("competition of a slot", read_usize(object + league::COMPETITION) as u32, key_comp_record(key) as u32);
            }
            slot += 1;
        }
    }
    check("season end day", KeyNationRecord(entry).season_end_day() as u32, key_nation_end_day(spec) as u32);
    check("season year", read_u16(entry + KEY_NATION_RECORD_SEASON_YEAR) as u32, year(spec.season_year));
    check("spans year", KeyNationRecord(entry).season_spans_year() as u32, spec.spans as u32);
    if let Some(v) = entry_link(spec.league) {
        check("league comp", KeyNationRecord(entry).league_comp() as u32, v as u32);
    }
    if let Some(v) = entry_link(spec.cup) {
        check("cup comp", read_usize(entry + KEY_NATION_RECORD_CUP_COMP) as u32, v as u32);
    }
    SCOREBOARD.record(wrong.is_empty(), || format!("{}(entry {:#x}): {}", name, entry, wrong.join(", ")));
}

/// Creates the World Cup and its qualifying sections on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the
/// Setup.cpp "Error" box 0x2ede and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go
/// from 1 July of the game year (weekday 5) to 1 July of the game year (weekday 5). 7 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): FIFA World Cup, World Cup African
/// Qualifying Section, World Cup Asian Qualifying Section, World Cup CONCACAF Qualifying Section, World Cup European
/// Qualifying Section, World Cup Oceania Qualifying Section, World Cup South American Qualifying Sect; every
/// constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x2efb,
/// "create_world_cup_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(25, 6, 2001, -1) (25 July 2001), `KEY_NATION_RECORD_SEASON_YEAR` = 1996,
/// `key_nation_record::SEASON_SPANS_YEAR` = 2; `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original:
/// `create_world_cup_qualifying_comps` (0x830620, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_world_cup_qualifying_comps(entry: usize) -> u32 {
    covered!("tel13_2 world cup qualifying");
    key_nation_create_comps(entry, &WORLD_CUP_QUALIFYING_COMPS)
}

/// The entry of `create_world_cup_qualifying_comps`.
const WORLD_CUP_QUALIFYING_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x2ede, 0x2efb, 0xa81d04), // "create_world_cup_comps() : unable to allocate memory"
    start: (1, 6, EntryYear::Current(0), 0x5),
    end: (1, 6, EntryYear::Current(0), 0x5),
    count: EntryCount::Fixed(7),
    calloc: None,
    comps: &[
        (0x192, 0x92bf50, KEY_CLUB_COMP_FIFA_WORLD_CUP, EntryYear::Current(0), false), // WorldCup::WorldCup
        (0xbb, 0x90f210, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION, EntryYear::Current(0), false), // WcAfricanCup::WcAfricanCup
        (0xfb, 0x911bf0, KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION, EntryYear::Current(0), false), // WcAsiaLeague::WcAsiaLeague
        (0xbd, 0x9149f0, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION, EntryYear::Current(0), false), // WcConcacafCup::WcConcacafCup
        (0x102, 0x918200, KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION, EntryYear::Current(0), false), // WcEuropeLeague::WcEuropeLeague
        (0xf7, 0x91df90, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION, EntryYear::Current(0), false), // WcOceaniaLeague::WcOceaniaLeague
        (0xf0, 0x9202b0, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT, EntryYear::Current(0), false), // WcSouthAmericanLeague::WcSouthAmericanLeague
    ],
    end_day: (25, 6),
    season_year: EntryYear::Fixed(0x7cc),
    spans: 2,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates the other international tournaments on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the
/// Setup.cpp "Error" box 0x2f22 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go
/// from 1 July of the game year (weekday 5) to 1 July of the game year (weekday 5). 8 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): Football Tournament Of The Olympic Games,
/// European Football Championship, European Championship Qualifying, African Cup Of Nations, Copa America, Oceania
/// Nations Cup, Asian Cup, Confederations Cup; every constructor gets the game year. A failed allocation: the "Memory
/// Allocation Error" box (Setup.cpp 0x2f3e, "create_other_int_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(15, 7, 2001, -1) (15 August 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = 2001, `key_nation_record::SEASON_SPANS_YEAR` = 2; `key_nation_record::LEAGUE_COMP`
/// = null; returns 1. /// Original: `create_other_int_comps` (0x830ac0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_other_int_comps(entry: usize) -> u32 {
    covered!("tel13_2 other int");
    key_nation_create_comps(entry, &OTHER_INT_COMPS)
}

/// The entry of `create_other_int_comps`.
const OTHER_INT_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x2f22, 0x2f3e, 0xa81d3c), // "create_other_int_comps() : unable to allocate memory"
    start: (1, 6, EntryYear::Current(0), 0x5),
    end: (1, 6, EntryYear::Current(0), 0x5),
    count: EntryCount::Fixed(8),
    calloc: None,
    comps: &[
        (0xf6, 0x79f530, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, EntryYear::Current(0), false), // Olympics::Olympics
        (0x142, 0x57a890, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, EntryYear::Current(0), false), // EuroChamp::EuroChamp
        (0x102, 0x57ce10, KEY_CLUB_COMP_EUROPEAN_CHAMPIONSHIP_QUALIFYING, EntryYear::Current(0), false), // EuroChampQual::EuroChampQual
        (0xf4, 0x401000, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, EntryYear::Current(0), false), // AfricanNations::AfricanNations
        (0xf6, 0x5e0660, KEY_CLUB_COMP_COPA_AMERICA, EntryYear::Current(0), false), // GoldCup::GoldCup
        (0xf4, 0x7988b0, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, EntryYear::Current(0), false), // OceaniaNations::OceaniaNations
        (0xf4, 0x40e860, KEY_CLUB_COMP_ASIAN_CUP, EntryYear::Current(0), false), // AsiaNations::AsiaNations
        (0xf6, 0x58b310, KEY_CLUB_COMP_CONFEDERATIONS_CUP, EntryYear::Current(0), false), // FifaConfed::FifaConfed
    ],
    end_day: (15, 7),
    season_year: EntryYear::Fixed(0x7d1),
    spans: 2,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates the world club tournaments on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp
/// "Error" box 0x2f69 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 1 July
/// of the game year (weekday 5) to 1 July of the game year (weekday 5). 3 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): FIFA Club World Championship, World Club Cup, Inter American Cup; every
/// constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x2f88,
/// "create_world_club_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(20, 5, 2001, -1) (20 June 2001), `KEY_NATION_RECORD_SEASON_YEAR` = 2001,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original:
/// `create_world_club_comps` (0x830fe0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_world_club_comps(entry: usize) -> u32 {
    covered!("tel13_2 world club");
    key_nation_create_comps(entry, &WORLD_CLUB_COMPS)
}

/// The entry of `create_world_club_comps`.
const WORLD_CLUB_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x2f69, 0x2f88, 0xa81d74), // "create_world_club_comps() : unable to allocate memory"
    start: (1, 6, EntryYear::Current(0), 0x5),
    end: (1, 6, EntryYear::Current(0), 0x5),
    count: EntryCount::Fixed(3),
    calloc: None,
    comps: &[
        (0xf6, 0x929140, KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP, EntryYear::Current(0), false), // WorldClubChamp::WorldClubChamp
        (0xb2, 0x92b4b0, KEY_CLUB_COMP_WORLD_CLUB_CUP, EntryYear::Current(0), false), // WorldClubCup::WorldClubCup
        (0xb2, 0x632080, KEY_CLUB_COMP_INTER_AMERICAN_CUP, EntryYear::Current(0), false), // InterAmerCup::InterAmerCup
    ],
    end_day: (20, 5),
    season_year: EntryYear::Fixed(0x7d1),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates the European club cups on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x2fac and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 23 June of the
/// game year (weekday -1) to 20 May of the game year (weekday 5). 4 competitions (`KEY_NATION_RECORD_COMP_COUNT`, the
/// array in `KEY_NATION_RECORD_COMPS`): UEFA European Champions League, UEFA Cup, Inter Toto Cup, European Super Cup
/// only when a nation of the entry's continent has a selected league (`key_nation_continent_has_selected_nation` of
/// `KEY_NATION_RECORD_CONTINENT`); otherwise the count and the array stay 0; every constructor gets the game year. A
/// failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x2fcd, "create_european_club_comps() : unable to
/// allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of Date::Date(21, 5, 2001, -1) (21 June
/// 2001), `KEY_NATION_RECORD_SEASON_YEAR` = 2001, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original: `create_european_club_comps` (0x8312e0, Setup.cpp,
/// cdecl).
pub(crate) unsafe extern "C" fn create_european_club_comps(entry: usize) -> u32 {
    covered!("tel13_2 european club");
    key_nation_create_comps(entry, &EUROPEAN_CLUB_COMPS)
}

/// The entry of `create_european_club_comps`.
const EUROPEAN_CLUB_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x2fac, 0x2fcd, 0xa81dac), // "create_european_club_comps() : unable to allocate memory"
    start: (23, 5, EntryYear::Current(0), 0xffff),
    end: (20, 4, EntryYear::Current(0), 0x5),
    count: EntryCount::Continental(4),
    calloc: None,
    comps: &[
        (0xb2, 0x5837c0, KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, EntryYear::Current(0), false), // EuropeanCup::EuropeanCup
        (0xb3, 0x9022e0, KEY_CLUB_COMP_UEFA_CUP, EntryYear::Current(0), false), // UefaCup::UefaCup
        (0xb6, 0x632fb0, KEY_CLUB_COMP_INTER_TOTO_CUP, EntryYear::Current(0), false), // IntertotoCup::IntertotoCup
        (0xb2, 0x57a100, KEY_CLUB_COMP_EUROPEAN_SUPER_CUP, EntryYear::Current(0), false), // EurSuperCup::EurSuperCup
    ],
    end_day: (21, 5),
    season_year: EntryYear::Fixed(0x7d1),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates the CONCACAF club cups on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x2ff9 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 1 July of the
/// game year (weekday 5) to 1 July of the game year (weekday 5). 2 competitions (`KEY_NATION_RECORD_COMP_COUNT`, the
/// array in `KEY_NATION_RECORD_COMPS`): CONCACAF Champions Cup, CONCACAF Merconorte Cup only when a nation of the
/// entry's continent has a selected league (`key_nation_continent_has_selected_nation` of
/// `KEY_NATION_RECORD_CONTINENT`); otherwise the count and the array stay 0; every constructor gets the start date
/// year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x301d, "create_north_american_club_comps()
/// : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of Date::Date(21, 1, 2001, -1)
/// (21 February 2001), `KEY_NATION_RECORD_SEASON_YEAR` = 2001, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original: `create_north_american_club_comps` (0x831650,
/// Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_north_american_club_comps(entry: usize) -> u32 {
    covered!("tel13_2 north american club");
    key_nation_create_comps(entry, &NORTH_AMERICAN_CLUB_COMPS)
}

/// The entry of `create_north_american_club_comps`.
const NORTH_AMERICAN_CLUB_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x2ff9, 0x301d, 0xa81de8), // "create_north_american_club_comps() : unable to allocate memory"
    start: (1, 6, EntryYear::Current(0), 0x5),
    end: (1, 6, EntryYear::Current(0), 0x5),
    count: EntryCount::Continental(2),
    calloc: None,
    comps: &[
        (0xb2, 0x4c0c40, KEY_CLUB_COMP_CONCACAF_CHAMPIONS_CUP, EntryYear::Start(0), false), // ConChamp::ConChamp
        (0xf6, 0x4c16f0, KEY_CLUB_COMP_CONCACAF_MERCONORTE_CUP, EntryYear::Start(0), false), // ConMercCup::ConMercCup
    ],
    end_day: (21, 1),
    season_year: EntryYear::Fixed(0x7d1),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates the South American club cups on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp
/// "Error" box 0x3045 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 1 July
/// of the game year (weekday 5) to 1 July of the game year (weekday 5). 2 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS` (allocated as calloc(4, 2))): South American Copa Libertadores, South
/// American Copa Mercosur only when a nation of the entry's continent has a selected league
/// (`key_nation_continent_has_selected_nation` of `KEY_NATION_RECORD_CONTINENT`); otherwise the count and the array
/// stay 0; every constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp
/// 0x3065, "create_south_american_club_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY`
/// = the day of Date::Date(10, 0, 2001, -1) (10 January 2001), `KEY_NATION_RECORD_SEASON_YEAR` = 2001,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original:
/// `create_south_american_club_comps` (0x8318f0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_south_american_club_comps(entry: usize) -> u32 {
    covered!("tel13_2 south american club");
    key_nation_create_comps(entry, &SOUTH_AMERICAN_CLUB_COMPS)
}

/// The entry of `create_south_american_club_comps`.
const SOUTH_AMERICAN_CLUB_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x3045, 0x3065, 0xa81e28), // "create_south_american_club_comps() : unable to allocate memory"
    start: (1, 6, EntryYear::Current(0), 0x5),
    end: (1, 6, EntryYear::Current(0), 0x5),
    count: EntryCount::Continental(2),
    calloc: Some((4, 2)),
    comps: &[
        (0xf6, 0x4c2f50, KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES, EntryYear::Current(0), false), // ConmebolLiber::ConmebolLiber
        (0xf6, 0x4c46c0, KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_MERCOSUR, EntryYear::Current(0), false), // ConmebolMerc::ConmebolMerc
    ],
    end_day: (10, 0),
    season_year: EntryYear::Fixed(0x7d1),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates the Asian club cups on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error" box
/// 0x308d and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 1 July of the game
/// year (weekday 5) to 1 July of the game year (weekday 5). 3 competitions (`KEY_NATION_RECORD_COMP_COUNT`, the array
/// in `KEY_NATION_RECORD_COMPS`): Asian Champions Cup, Asian Cup Winners Cup, Asian Super Cup only when a nation of the
/// entry's continent has a selected league (`key_nation_continent_has_selected_nation` of
/// `KEY_NATION_RECORD_CONTINENT`); otherwise the count and the array stay 0; every constructor gets the game year. A
/// failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x30ad, "create_asian_club_comps() : unable to
/// allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of Date::Date(28, 7, 2001, -1) (28 August
/// 2001), `KEY_NATION_RECORD_SEASON_YEAR` = 2001, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original: `create_asian_club_comps` (0x831ba0, Setup.cpp,
/// cdecl).
pub(crate) unsafe extern "C" fn create_asian_club_comps(entry: usize) -> u32 {
    covered!("tel13_2 asian club");
    key_nation_create_comps(entry, &ASIAN_CLUB_COMPS)
}

/// The entry of `create_asian_club_comps`.
const ASIAN_CLUB_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x308d, 0x30ad, 0xa81e68), // "create_asian_club_comps() : unable to allocate memory"
    start: (1, 6, EntryYear::Current(0), 0x5),
    end: (1, 6, EntryYear::Current(0), 0x5),
    count: EntryCount::Continental(3),
    calloc: None,
    comps: &[
        (0xba, 0x40aae0, KEY_CLUB_COMP_ASIAN_CHAMPIONS_CUP, EntryYear::Current(0), false), // AsiaClubChamp::AsiaClubChamp
        (0xba, 0x40cbe0, KEY_CLUB_COMP_ASIAN_CUP_WINNERS_CUP, EntryYear::Current(0), false), // AsiaCupWinner::AsiaCupWinner
        (0xb2, 0x410650, KEY_CLUB_COMP_ASIAN_SUPER_CUP, EntryYear::Current(0), false), // AsiaSuperCup::AsiaSuperCup
    ],
    end_day: (28, 7),
    season_year: EntryYear::Fixed(0x7d1),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates the Oceania club championship on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp
/// "Error" box 0x30d6 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 1 July
/// of the game year (weekday 5) to 1 July of the game year (weekday 5). 1 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): Oceania Club Championship only when a nation of the entry's continent has a
/// selected league (`key_nation_continent_has_selected_nation` of `KEY_NATION_RECORD_CONTINENT`); otherwise the count
/// and the array stay 0; every constructor gets the game year. A failed allocation: the "Memory Allocation Error" box
/// (Setup.cpp 0x30f6, "create_asian_club_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(21, 6, 2001, -1) (21 July 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = 2001, `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP`
/// = null; returns 1. The original's allocation message names another routine (create_asian_club_comps()). ///
/// Original: `create_oceania_club_comps` (0x831eb0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_oceania_club_comps(entry: usize) -> u32 {
    covered!("tel13_2 oceania club");
    key_nation_create_comps(entry, &OCEANIA_CLUB_COMPS)
}

/// The entry of `create_oceania_club_comps`.
const OCEANIA_CLUB_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x30d6, 0x30f6, 0xa81e68), // "create_asian_club_comps() : unable to allocate memory"
    start: (1, 6, EntryYear::Current(0), 0x5),
    end: (1, 6, EntryYear::Current(0), 0x5),
    count: EntryCount::Continental(1),
    calloc: None,
    comps: &[
        (0xf6, 0x796fd0, KEY_CLUB_COMP_OCEANIA_CLUB_CHAMPIONSHIP, EntryYear::Current(0), false), // OceaniaClubChamp::OceaniaClubChamp
    ],
    end_day: (21, 6),
    season_year: EntryYear::Fixed(0x7d1),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates Argentina's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp
/// "Error" box 0x311f and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 23
/// August of the game year (weekday 5) to 19 July of the next year (weekday 6). 2 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): Argentine Premier Division, Argentine
/// Second Division; every constructor gets the game year. A failed allocation: the "Memory Allocation Error" box
/// (Setup.cpp 0x313c, "create_argentine_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(1, 7, 2001, -1) (1 August 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the game year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original: `create_argentine_comps` (0x832100, Setup.cpp,
/// cdecl).
pub(crate) unsafe extern "C" fn create_argentine_comps(entry: usize) -> u32 {
    covered!("tel13_2 argentine");
    key_nation_create_comps(entry, &ARGENTINE_COMPS)
}

/// The entry of `create_argentine_comps`.
const ARGENTINE_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x311f, 0x313c, 0xa81ea0), // "create_argentine_comps() : unable to allocate memory"
    start: (23, 7, EntryYear::Current(0), 0x5),
    end: (19, 6, EntryYear::Current(1), 0x6),
    count: EntryCount::Fixed(2),
    calloc: None,
    comps: &[
        (0xee, 0x404250, KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION, EntryYear::Current(0), false), // ArgPrm::ArgPrm
        (0xf2, 0x406bf0, KEY_CLUB_COMP_ARGENTINE_SECOND_DIVISION, EntryYear::Current(0), false), // ArgSecond::ArgSecond
    ],
    end_day: (1, 7),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates Australia's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp
/// "Error" box 0x3160 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 2
/// October of the game year (weekday 5) to 10 June of the next year (weekday 5). 1 competitions
/// (`KEY_NATION_RECORD_COMP_COUNT`, the array in `KEY_NATION_RECORD_COMPS`): Australian National Soccer League; every
/// constructor gets the game year. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x317d,
/// "create_australian_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(20, 6, 2001, -1) (20 July 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the game year,
/// `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = null; returns 1. /// Original:
/// `create_australian_comps` (0x832380, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_australian_comps(entry: usize) -> u32 {
    covered!("tel13_2 australian");
    key_nation_create_comps(entry, &AUSTRALIAN_COMPS)
}

/// The entry of `create_australian_comps`.
const AUSTRALIAN_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x3160, 0x317d, 0xa81ed8), // "create_australian_comps() : unable to allocate memory"
    start: (2, 9, EntryYear::Current(0), 0x5),
    end: (10, 5, EntryYear::Current(1), 0x5),
    count: EntryCount::Fixed(1),
    calloc: None,
    comps: &[
        (0xee, 0x410e90, KEY_CLUB_COMP_AUSTRALIAN_NATIONAL_SOCCER_LEAGUE, EntryYear::Current(0), false), // AusNsl::AusNsl
    ],
    end_day: (20, 6),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates Belgium's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x31a1 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 3 August of
/// the game year (weekday 5) to 1 June of the next year (weekday 6). 5 competitions (`KEY_NATION_RECORD_COMP_COUNT`,
/// the array in `KEY_NATION_RECORD_COMPS`): Belgian First Division, Belgian Second Division, Belgian Third Division,
/// Belgian Cup, Belgian Super Cup; every constructor gets the game year. A failed allocation: the "Memory Allocation
/// Error" box (Setup.cpp 0x31be, "create_belgian_comps() : unable to allocate memory") and 0.
/// `key_nation_record::SEASON_END_DAY` = the day of Date::Date(20, 5, 2001, -1) (20 June 2001),
/// `KEY_NATION_RECORD_SEASON_YEAR` = the game year, `key_nation_record::SEASON_SPANS_YEAR` = 1;
/// `key_nation_record::LEAGUE_COMP` = the Belgian Super Cup record; returns 1. /// Original: `create_belgian_comps`
/// (0x8325d0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_belgian_comps(entry: usize) -> u32 {
    covered!("tel13_2 belgian");
    key_nation_create_comps(entry, &BELGIAN_COMPS)
}

/// The entry of `create_belgian_comps`.
const BELGIAN_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x31a1, 0x31be, 0xa81f10), // "create_belgian_comps() : unable to allocate memory"
    start: (3, 7, EntryYear::Current(0), 0x5),
    end: (1, 5, EntryYear::Current(1), 0x6),
    count: EntryCount::Fixed(5),
    calloc: None,
    comps: &[
        (0xee, 0x41dd40, KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION, EntryYear::Current(0), false), // BelFirst::BelFirst
        (0xf9, 0x41f030, KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION, EntryYear::Current(0), false), // BelSecond::BelSecond
        (0x104, 0x4216f0, KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION, EntryYear::Current(0), false), // BelThird::BelThird
        (0xb2, 0x41d050, KEY_CLUB_COMP_BELGIAN_CUP, EntryYear::Current(0), false), // BelFaCup::BelFaCup
        (0xb2, 0x420ef0, KEY_CLUB_COMP_BELGIAN_SUPER_CUP, EntryYear::Current(0), false), // BelSuper::BelSuper
    ],
    end_day: (20, 5),
    season_year: EntryYear::Current(0),
    spans: 1,
    league: EntryLink::Comp(KEY_CLUB_COMP_BELGIAN_SUPER_CUP),
    cup: EntryLink::Untouched,
    weeks: false,
};

/// Creates Brazil's competitions on the key nation entry `entry` (a `G_LEAGUE_TABLE` row; null: the Setup.cpp "Error"
/// box 0x31e6 and 0). The season bounds (Date::Date arguments: day, month from 0, year, weekday) go from 18 January of
/// the year 4 weeks after the start date (weekday 6) to 14 December of the year 4 weeks after the start date (weekday
/// 6) (Brazil reads its year from the start date moved 4 weeks on, `Date::add_weeks`; its constructors get the dword
/// from that year on: the year and the next word). 17 competitions (`KEY_NATION_RECORD_COMP_COUNT`, the array in
/// `KEY_NATION_RECORD_COMPS`): Brazilian National First Division, Brazilian National Second Division, Brazilian
/// National Third Division, Brazilian Bahia State Championship, Brazilian Central State Championship, Brazilian Gaucho
/// State Championship, Brazilian Goias State Championship, Brazilian Minas Gerais State Championshi, Brazilian North
/// State Championship, Brazilian Northeast State Championship, Brazilian Parana State Championship, Brazilian
/// Pernambuco State Championship, Brazilian Rio De Janeiro State Champions, Brazilian Santa Catarina State Champions,
/// Brazilian Sao Paulo State Champ, Brazilian Cup, Brazilian Champions Cup; every constructor gets the year 4 weeks
/// after the start date. A failed allocation: the "Memory Allocation Error" box (Setup.cpp 0x3205,
/// "create_norwegian_comps() : unable to allocate memory") and 0. `key_nation_record::SEASON_END_DAY` = the day of
/// Date::Date(12, 0, 2001, -1) (12 January 2001), `KEY_NATION_RECORD_SEASON_YEAR` = the year 4 weeks after the start
/// date, `key_nation_record::SEASON_SPANS_YEAR` = 1; `key_nation_record::LEAGUE_COMP` = null; returns 1. The original's
/// allocation message names another routine (create_norwegian_comps()). /// Original: `create_brazilian_comps`
/// (0x832980, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_brazilian_comps(entry: usize) -> u32 {
    covered!("tel13_2 brazilian");
    key_nation_create_comps(entry, &BRAZILIAN_COMPS)
}

/// The entry of `create_brazilian_comps`.
const BRAZILIAN_COMPS: KeyNationComps = KeyNationComps {
    rows: (0x31e6, 0x3205, 0xa81f44), // "create_norwegian_comps() : unable to allocate memory"
    start: (18, 0, EntryYear::StartPlus4Weeks, 0x6),
    end: (14, 11, EntryYear::StartPlus4Weeks, 0x6),
    count: EntryCount::Fixed(17),
    calloc: None,
    comps: &[
        (0xee, 0x429850, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION, EntryYear::StartPlus4Weeks, false), // BraNatFirst::BraNatFirst
        (0xee, 0x42be40, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION, EntryYear::StartPlus4Weeks, false), // BraNatSecond::BraNatSecond
        (0xf2, 0x42d2c0, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_THIRD_DIVISION, EntryYear::StartPlus4Weeks, false), // BraNatThird::BraNatThird
        (0xee, 0x42f2d0, KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP, EntryYear::StartPlus4Weeks, false), // BraRegBahia::BraRegBahia
        (0xee, 0x4304b0, KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, EntryYear::StartPlus4Weeks, false), // BraRegCentral::BraRegCentral
        (0xee, 0x4317c0, KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP, EntryYear::StartPlus4Weeks, false), // BraRegGaucho::BraRegGaucho
        (0xee, 0x432b50, KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, EntryYear::StartPlus4Weeks, false), // BraRegGoias::BraRegGoias
        (0xee, 0x433d50, KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI, EntryYear::StartPlus4Weeks, false), // BraRegMinasGerais::BraRegMinasGerais
        (0xee, 0x435340, KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, EntryYear::StartPlus4Weeks, false), // BraRegNorth::BraRegNorth
        (0xee, 0x4363f0, KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, EntryYear::StartPlus4Weeks, false), // BraRegNortheast::BraRegNortheast
        (0xee, 0x437400, KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, EntryYear::StartPlus4Weeks, false), // BraRegParana::BraRegParana
        (0xee, 0x4388f0, KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP, EntryYear::StartPlus4Weeks, false), // BraRegPern::BraRegPern
        (0xee, 0x439a40, KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS, EntryYear::StartPlus4Weeks, false), // BraRegRio::BraRegRio
        (0xee, 0x43b130, KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS, EntryYear::StartPlus4Weeks, false), // BraRegSanta::BraRegSanta
        (0xee, 0x43c280, KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP, EntryYear::StartPlus4Weeks, false), // BraRegSp::BraRegSp
        (0xb2, 0x427430, KEY_CLUB_COMP_BRAZILIAN_CUP, EntryYear::StartPlus4Weeks, false), // BraCup::BraCup
        (0xee, 0x4259a0, KEY_CLUB_COMP_BRAZILIAN_CHAMPIONS_CUP, EntryYear::StartPlus4Weeks, false), // BraChampCup::BraChampCup
    ],
    end_day: (12, 0),
    season_year: EntryYear::StartPlus4Weeks,
    spans: 1,
    league: EntryLink::Null,
    cup: EntryLink::Untouched,
    weeks: true,
};

/// Comparer of `create_world_cup_qualifying_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_world_cup_qualifying_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I0%%)(entry);
    key_nation_check_comps("create_world_cup_qualifying_comps", entry, r, &WORLD_CUP_QUALIFYING_COMPS);
    r
}

/// Comparer of `create_other_int_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_other_int_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I1%%)(entry);
    key_nation_check_comps("create_other_int_comps", entry, r, &OTHER_INT_COMPS);
    r
}

/// Comparer of `create_world_club_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_world_club_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I2%%)(entry);
    key_nation_check_comps("create_world_club_comps", entry, r, &WORLD_CLUB_COMPS);
    r
}

/// Comparer of `create_european_club_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_european_club_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I3%%)(entry);
    key_nation_check_comps("create_european_club_comps", entry, r, &EUROPEAN_CLUB_COMPS);
    r
}

/// Comparer of `create_north_american_club_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_north_american_club_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I4%%)(entry);
    key_nation_check_comps("create_north_american_club_comps", entry, r, &NORTH_AMERICAN_CLUB_COMPS);
    r
}

/// Comparer of `create_south_american_club_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_south_american_club_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I5%%)(entry);
    key_nation_check_comps("create_south_american_club_comps", entry, r, &SOUTH_AMERICAN_CLUB_COMPS);
    r
}

/// Comparer of `create_asian_club_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_asian_club_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I6%%)(entry);
    key_nation_check_comps("create_asian_club_comps", entry, r, &ASIAN_CLUB_COMPS);
    r
}

/// Comparer of `create_oceania_club_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_oceania_club_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I7%%)(entry);
    key_nation_check_comps("create_oceania_club_comps", entry, r, &OCEANIA_CLUB_COMPS);
    r
}

/// Comparer of `create_argentine_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_argentine_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I8%%)(entry);
    key_nation_check_comps("create_argentine_comps", entry, r, &ARGENTINE_COMPS);
    r
}

/// Comparer of `create_australian_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_australian_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I9%%)(entry);
    key_nation_check_comps("create_australian_comps", entry, r, &AUSTRALIAN_COMPS);
    r
}

/// Comparer of `create_belgian_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_belgian_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I10%%)(entry);
    key_nation_check_comps("create_belgian_comps", entry, r, &BELGIAN_COMPS);
    r
}

/// Comparer of `create_brazilian_comps`: only the original runs (it allocates the competitions: it cannot run twice); then the entry is
/// checked against the plan (`key_nation_check_comps`).
pub(crate) unsafe extern "C" fn compare_create_brazilian_comps(entry: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I11%%)(entry);
    key_nation_check_comps("create_brazilian_comps", entry, r, &BRAZILIAN_COMPS);
    r
}
