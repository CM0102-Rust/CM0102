// ---------- batch %%LEVA%%: building the key nation table (key_nation.cpp; sx12b_1) ----------

/// A season date of a key nation row: day and month (0-based) of the current year (`G_CURRENT_DATE_YEAR`) or of the
/// next, and the weekday the date is moved to (-1: none), as `Date::Date` (0x549f70) takes them.
#[derive(Clone, Copy)]
pub(crate) struct KeyNationDate {
    day: u32,
    month: u32,
    next_year: bool,
    weekday: i32,
}

const fn date(day: u32, month: u32, next_year: bool, weekday: i32) -> KeyNationDate {
    KeyNationDate { day, month, next_year, weekday }
}

/// One of the 35 rows `key_nation_build_table` writes: the nation (`KEY_NATION_*`, 0 = none) or continent
/// (`KEY_CONTINENT_*`), the competition creation function (an exe address), the season end (day and month of 2001),
/// the split season flag, the cup and league cup (`KEY_CLUB_COMP_*`, 0 = none) and the season start and end dates.
pub(crate) struct KeyNationRow {
    nation: usize,
    continent: usize,
    create: usize,
    season_end: Option<(u32, u32)>,
    split_season: bool,
    cup: usize,
    league_cup: usize,
    start: KeyNationDate,
    end: KeyNationDate,
}

/// The rows of the key nation table, in order (the original writes them one by one, unrolled).
pub(crate) const KEY_NATION_ROWS: [KeyNationRow; 0x23] = [
    KeyNationRow { nation: 0, continent: 0, create: 0x830620, // create_world_cup_qualifying_comps
        season_end: None, split_season: true, cup: 0, league_cup: 0,
        start: date(1, 6, false, -1), end: date(1, 6, false, -1) },
    KeyNationRow { nation: 0, continent: 0, create: 0x830ac0, // create_other_int_comps
        season_end: None, split_season: true, cup: 0, league_cup: 0,
        start: date(1, 6, false, -1), end: date(1, 6, false, -1) },
    KeyNationRow { nation: 0, continent: 0, create: 0x830fe0, // create_world_club_comps
        season_end: Some((21, 5)), split_season: true, cup: 0, league_cup: 0,
        start: date(23, 5, false, -1), end: date(20, 4, false, -1) },
    KeyNationRow { nation: 0, continent: KEY_CONTINENT_EUROPE, create: 0x8312e0, // create_european_club_comps
        season_end: Some((21, 5)), split_season: true, cup: KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, league_cup: 0,
        start: date(23, 5, false, -1), end: date(20, 4, false, -1) },
    KeyNationRow { nation: 0, continent: KEY_CONTINENT_NORTH_AMERICA, create: 0x831650, // create_north_american_club_comps
        season_end: Some((21, 1)), split_season: true, cup: KEY_CLUB_COMP_CONCACAF_CHAMPIONS_CUP, league_cup: 0,
        start: date(1, 6, false, -1), end: date(1, 6, false, -1) },
    KeyNationRow { nation: 0, continent: KEY_CONTINENT_SOUTH_AMERICA, create: 0x8318f0, // create_south_american_club_comps
        season_end: Some((21, 11)), split_season: true, cup: KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES, league_cup: 0,
        start: date(1, 6, false, -1), end: date(1, 6, false, -1) },
    KeyNationRow { nation: 0, continent: KEY_CONTINENT_ASIA, create: 0x831ba0, // create_asian_club_comps
        season_end: Some((21, 1)), split_season: true, cup: 0, league_cup: 0,
        start: date(1, 6, false, -1), end: date(1, 6, false, -1) },
    KeyNationRow { nation: 0, continent: KEY_CONTINENT_OCEANIA, create: 0x831eb0, // create_oceania_club_comps
        season_end: Some((21, 6)), split_season: true, cup: 0, league_cup: 0,
        start: date(1, 6, false, -1), end: date(1, 6, false, -1) },
    KeyNationRow { nation: KEY_NATION_ARGENTINA, continent: 0, create: 0x832100, // create_argentine_comps
        season_end: Some((1, 7)), split_season: true, cup: 0, league_cup: 0,
        start: date(9, 7, false, -1), end: date(19, 6, false, -1) },
    KeyNationRow { nation: KEY_NATION_AUSTRALIA, continent: 0, create: 0x832380, // create_australian_comps
        season_end: Some((1, 7)), split_season: true, cup: 0, league_cup: 0,
        start: date(2, 9, false, -1), end: date(10, 5, false, -1) },
    KeyNationRow { nation: KEY_NATION_BELGIUM, continent: 0, create: 0x8325d0, // create_belgian_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_BELGIAN_CUP, league_cup: 0,
        start: date(7, 7, false, 5), end: date(1, 5, false, -1) },
    KeyNationRow { nation: KEY_NATION_BRAZIL, continent: 0, create: 0x832980, // create_brazilian_comps
        season_end: Some((1, 0)), split_season: false, cup: KEY_CLUB_COMP_BRAZILIAN_CUP, league_cup: 0,
        start: date(18, 0, true, -1), end: date(14, 11, true, -1) },
    KeyNationRow { nation: KEY_NATION_CROATIA, continent: 0, create: 0x8330b0, // create_croatian_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_CROATIAN_CUP, league_cup: 0,
        start: date(24, 6, false, 5), end: date(28, 4, false, -1) },
    KeyNationRow { nation: KEY_NATION_DENMARK, continent: 0, create: 0x833400, // create_danish_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_DANISH_CUP, league_cup: 0,
        start: date(27, 6, false, 6), end: date(14, 5, false, -1) },
    KeyNationRow { nation: KEY_NATION_ENGLAND, continent: 0, create: 0x833750, // create_english_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_ENGLISH_FA_CUP, league_cup: KEY_CLUB_COMP_ENGLISH_LEAGUE_CUP,
        start: date(10, 7, false, 5), end: date(31, 4, false, -1) },
    KeyNationRow { nation: KEY_NATION_FINLAND, continent: 0, create: 0x833cf0, // create_finnish_comps
        season_end: Some((27, 11)), split_season: false, cup: KEY_CLUB_COMP_FINNISH_CUP, league_cup: 0,
        start: date(29, 3, true, 5), end: date(12, 10, true, -1) },
    KeyNationRow { nation: KEY_NATION_FRANCE, continent: 0, create: 0x833ff0, // create_french_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_FRENCH_CUP, league_cup: KEY_CLUB_COMP_FRENCH_LEAGUE_CUP,
        start: date(9, 7, false, 5), end: date(24, 4, false, -1) },
    KeyNationRow { nation: KEY_NATION_GERMANY, continent: 0, create: 0x8343f0, // create_german_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_GERMAN_CUP, league_cup: 0,
        start: date(29, 6, false, 5), end: date(26, 4, false, -1) },
    KeyNationRow { nation: KEY_NATION_GREECE, continent: 0, create: 0x8347c0, // create_greek_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_GREEK_CUP, league_cup: 0,
        start: date(15, 7, false, 5), end: date(21, 4, false, -1) },
    KeyNationRow { nation: KEY_NATION_HOLLAND, continent: 0, create: 0x834b10, // create_dutch_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_DUTCH_AMSTEL_CUP, league_cup: 0,
        start: date(16, 7, false, 5), end: date(7, 5, false, -1) },
    KeyNationRow { nation: KEY_NATION_REPUBLIC_OF_IRELAND, continent: 0, create: 0x834e60, // create_irish_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_IRISH_SENIOR_CHALLENGE_CUP, league_cup: 0,
        start: date(1, 7, false, 6), end: date(20, 4, false, 6) },
    KeyNationRow { nation: KEY_NATION_ITALY, continent: 0, create: 0x835320, // create_italian_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_ITALIAN_CUP, league_cup: 0,
        start: date(25, 7, false, 6), end: date(8, 5, false, -1) },
    KeyNationRow { nation: KEY_NATION_JAPAN, continent: 0, create: 0x835900, // create_japanese_comps
        season_end: Some((25, 0)), split_season: false, cup: 0, league_cup: 0,
        start: date(6, 2, true, 5), end: date(28, 10, false, -1) },
    KeyNationRow { nation: KEY_NATION_NORTHERN_IRELAND, continent: 0, create: 0x835c90, // create_northern_irish_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_NORTHERN_IRISH_CUP, league_cup: 0,
        start: date(12, 7, false, 5), end: date(8, 4, false, -1) },
    KeyNationRow { nation: KEY_NATION_NORWAY, continent: 0, create: 0x836050, // create_norwegian_comps
        season_end: Some((27, 10)), split_season: false, cup: KEY_CLUB_COMP_NORWEGIAN_CUP, league_cup: 0,
        start: date(12, 3, true, 6), end: date(31, 9, true, -1) },
    KeyNationRow { nation: KEY_NATION_POLAND, continent: 0, create: 0x836350, // create_polish_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_POLISH_FA_CUP, league_cup: 0,
        start: date(17, 6, false, 5), end: date(5, 5, false, -1) },
    KeyNationRow { nation: KEY_NATION_PORTUGAL, continent: 0, create: 0x8366e0, // create_portuguese_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_PORTUGUESE_CUP, league_cup: 0,
        start: date(25, 7, false, 6), end: date(8, 5, false, -1) },
    KeyNationRow { nation: KEY_NATION_RUSSIA, continent: 0, create: 0x836ac0, // create_russian_comps
        season_end: Some((27, 11)), split_season: false, cup: KEY_CLUB_COMP_RUSSIAN_CUP, league_cup: 0,
        start: date(25, 2, true, 5), end: date(12, 10, true, 6) },
    KeyNationRow { nation: KEY_NATION_SCOTLAND, continent: 0, create: 0x836dc0, // create_scottish_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_SCOTTISH_CUP, league_cup: 0,
        start: date(2, 7, false, 5), end: date(24, 4, false, -1) },
    KeyNationRow { nation: KEY_NATION_SOUTH_KOREA, continent: 0, create: 0x837200, // create_south_korean_comps
        season_end: Some((20, 0)), split_season: false, cup: KEY_CLUB_COMP_KOREAN_FA_CUP, league_cup: 0,
        start: date(25, 2, true, 6), end: date(25, 10, true, -1) },
    KeyNationRow { nation: KEY_NATION_SPAIN, continent: 0, create: 0x837560, // create_spanish_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_SPANISH_CUP, league_cup: 0,
        start: date(1, 8, false, 6), end: date(14, 5, false, -1) },
    KeyNationRow { nation: KEY_NATION_SWEDEN, continent: 0, create: 0x8379f0, // create_swedish_comps
        season_end: Some((27, 11)), split_season: false, cup: KEY_CLUB_COMP_SWEDISH_CUP, league_cup: 0,
        start: date(6, 3, true, 6), end: date(26, 9, true, -1) },
    KeyNationRow { nation: KEY_NATION_TURKEY, continent: 0, create: 0x837d70, // create_turkish_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_TURKISH_FA_CUP, league_cup: 0,
        start: date(8, 7, false, 6), end: date(21, 4, false, -1) },
    KeyNationRow { nation: KEY_NATION_UNITED_STATES, continent: 0, create: 0x8380c0, // create_united_states_comps
        season_end: Some((20, 1)), split_season: false, cup: 0, league_cup: 0,
        start: date(22, 2, true, 5), end: date(6, 0, true, -1) },
    KeyNationRow { nation: KEY_NATION_WALES, continent: 0, create: 0x8383b0, // create_welsh_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_WELSH_CUP, league_cup: 0,
        start: date(19, 7, false, 5), end: date(6, 4, false, -1) },
];

/// The cup comp pointer (+0x1e) and the league cup comp pointer (+0x22) of a key nation row.
const KEY_NATION_CUP: usize = 0x1e;
const KEY_NATION_LEAGUE_CUP: usize = 0x22;

/// The record of a fixed key in a database table of `size`-byte records (`table` holds the table's address): table +
/// key x size, in 32-bit arithmetic as the original.
unsafe fn keyed_record(table: usize, key: usize, size: usize) -> usize {
    read_usize(table).wrapping_add((read_i32(key) as usize).wrapping_mul(size))
}

/// Writes a season date into a key nation row: `Date::Date` (0x549f70) of day, month, year and weekday into a
/// temporary, then `Date::operator=` into the row.
unsafe fn write_key_nation_date(at: usize, d: KeyNationDate) {
    let year = read_u16(G_CURRENT_DATE_YEAR).wrapping_add(d.next_year as u16);
    let (mut built, mut tmp) = ([0u16; 4], [0u8; 8]);
    let p = exe::date_date_549f70(&mut built, d.day, d.month, year as u32, d.weekday as u32);
    exe::date_operator_3d(at, tmp.as_mut_ptr() as usize, p);
}

/// Fills the table of the 35 key nations (`G_LEAGUE_TABLE`, 72-byte rows) for a new game; returns 1, or 0 without the
/// nation data.
///
/// `key_nation_build_table()`: with no nation table (`G_NATION_TABLE` 0) the "Error" box (key_nation.cpp line 0x36)
/// and 0. Otherwise the whole table is cleared (0x9d8 bytes) and each row of `KEY_NATION_ROWS` written: the nation
/// record (+0, `G_NATION_TABLE` + key x `nation::SIZE`) or the continent record (+4, `G_CONTINENT_TABLE` + key x 0xc6),
/// the competition creation function (+8, called later by `key_nation_create_all_comps`), the season end day
/// (`key_nation_record::SEASON_END_DAY`, the day of the year of that day and month in 2001), the split season flag
/// (`key_nation_record::SPLIT_SEASON_FLAG`, a dword), the cup (+0x1e) and league cup (+0x22) records
/// (`G_CLUB_COMP_TABLE` + key x 0x6b) and the season start (`key_nation_record::START_DATE`) and end
/// (`key_nation_record::END_DATE`) dates of the current or the next year. The rows are: World Cup qualifying, the
/// other international competitions, the world club competitions, the five continents' club competitions (Europe,
/// North America, South America, Asia, Oceania) and the 27 league nations from Argentina to Wales. Returns 1. Called by
/// the game setup (0x81ce00, 0x827110, 0x82a9c0).
/// The original pushes each year as a dword whose high word is stack residue; `Date::Date` reads only the word.
/// Original: `key_nation_build_table` (0x667150, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_build_table() -> u32 {
    if read_usize(G_NATION_TABLE) == 0 {
        show_error_at(0x9eaff4, 0x36); // "...\key_nation.cpp"
        return 0;
    }
    core::ptr::write_bytes(G_LEAGUE_TABLE as *mut u8, 0, 0x23 * 72);
    for (k, row) in KEY_NATION_ROWS.iter().enumerate() {
        let at = G_LEAGUE_TABLE + 72 * k;
        if row.nation != 0 {
            write_usize(at, keyed_record(G_NATION_TABLE, row.nation, nation::SIZE));
        }
        if row.continent != 0 {
            write_usize(at + 4, keyed_record(G_CONTINENT_TABLE, row.continent, 0xc6));
        }
        write_usize(at + 8, row.create);
        write_u32(at + key_nation_record::SPLIT_SEASON_FLAG, row.split_season as u32); // sx12b_1: the split season flag
        for (field, cup) in [(KEY_NATION_CUP, row.cup), (KEY_NATION_LEAGUE_CUP, row.league_cup)] {
            if cup != 0 {
                write_usize(at + field, keyed_record(G_CLUB_COMP_TABLE, cup, 0x6b));
            }
        }
        if let Some((day, month)) = row.season_end {
            let mut end = [0u16; 4];
            exe::date_date_549f70(&mut end, day, month, 2001, u32::MAX);
            write_u16(at + key_nation_record::SEASON_END_DAY, end[0]); // sx12b_1: the season end day
        }
        write_key_nation_date(at + key_nation_record::START_DATE, row.start);
        write_key_nation_date(at + key_nation_record::END_DATE, row.end);
    }
    covered!("sx12b_1 key nation table built");
    1
}

/// Comparer for `key_nation_build_table`: the table (35 x 72 bytes) and the return, side by side (the routine only
/// writes the table). Without the nation data (the "Error" box): only the original.
pub(crate) unsafe extern "C" fn compare_key_nation_build_table() -> u32 {
    let o = original_fn::<unsafe extern "C" fn() -> u32>(%%I0%%);
    if read_usize(G_NATION_TABLE) == 0 {
        return o();
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(G_LEAGUE_TABLE, 0x23 * 72)], || o(), || key_nation_build_table());
    SCOREBOARD.record(equal && r == n, || format!("key_nation_build_table original {} rust {}: {:?}", r, n, first_differences(&mo, &mr)));
    r
}
