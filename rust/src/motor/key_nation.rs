use super::*;

/// Returns the first row of the 35-row key nation table holding the given nation, or 4.
///
/// `key_nation_index_of_nation(nation)`: the first row of the table of the 35 key nations (`G_LEAGUE_TABLE`, 72 bytes,
/// nation at `+0`) with the given nation (without checking for null), or 4. Called by the match morale `0x4cb250`, the
/// retirements `0x7a7310`, the free friendly `0x5c6b70`, the prizes of the nations' day and the UEFA places. TapeCall
/// for every player: no counter.
/// Original: `key_nation_index_of_nation` (0x669340, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_index_of_nation(nation: usize) -> u8 {
    for k in 0..0x23usize {
        if read_usize(G_LEAGUE_TABLE + 72 * k) == nation {
            return k as u8; // cpu1: the row of the given nation
        }
    }
    4
}

/// Returns the key nation row of the club: the row of its chosen nation; for a national team, 1; for a nation not
/// chosen, the row of its continent, or 3.
///
/// `key_nation_index_of_club(club)`. A club that is not a national team (`exe::database_is_national_team`), with a
/// nation (`.nation()`) chosen to be played (`nation::LEAGUE_SELECTED` of the nation): the nation's row in the table
/// (`G_LEAGUE_TABLE`), or the "Error" box (line `0x26d`) and 0. Otherwise (the original calls
/// `exe::database_is_national_team` again): national team, 1; the first row whose continent
/// (`G_LEAGUE_WORLD_CUP_CONTINENT`) is that of the club's nation (`nation::CONTINENT`), or 3 (with no nation, 3).
/// Called by the retirements `0x7a7310`, the weather postponement `0x923630`, the days until the season `0x59b4c0`, the
/// desired contract `0x85fc50`, the season year `0x669950` and others. No counter.
/// Original: `key_nation_index_of_club` (0x669360, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_index_of_club(club: usize) -> u8 {
    let club = Club(club);
    if exe::database_is_national_team(club) == 0 {
        let nation = club.nation().addr();
        if nation != 0 && *((nation + nation::LEAGUE_SELECTED) as *const u8) != 0 {
            for k in 0..0x23usize {
                if read_usize(G_LEAGUE_TABLE + 72 * k) == nation {
                    return k as u8; // cpu1: the row of the chosen nation
                }
            }
            show_error_at(0x9eaff4, 0x26d);
            return 0;
        }
    }
    if exe::database_is_national_team(club) != 0 {
        return 1;
    }
    let nation = club.nation().addr(); // cpu1: the nation of the continent
    for k in 0..0x23usize {
        if nation != 0 && read_usize(G_LEAGUE_WORLD_CUP_CONTINENT + 72 * k) == read_usize(nation + nation::CONTINENT) {
            return k as u8; // cpu1: the continent row
        }
    }
    3
}

/// Would the club hit the `key_nation_index_of_club` "Error" box (not a national team, nation selected but not in the league table)?
pub(crate) unsafe fn hits_key_nation_error(club: Club) -> bool {
    let nation = club.nation();
    exe::database_is_national_team(club) == 0
        && !nation.is_null()
        && nation.league_selected() != 0
        && !(0..0x23usize).any(|k| read_usize(G_LEAGUE_TABLE + 72 * k) == nation.addr())
}

/// Returns the club's season year: the previous year when today is before the season end day of its row.
///
/// `key_nation_get_club_season_year(club)`: today's year (`G_CURRENT_DATE_YEAR`); for a club that is not a national
/// team (`exe::database_is_national_team`), by the club's row (`exe::key_nation_index_of_club`): with the row's dword
/// `key_nation_record::SPLIT_SEASON_FLAG` not zero or the end day (`key_nation_record::SEASON_END_DAY`, unsigned) below
/// 60, and today's day (`G_CURRENT_DATE`, signed) before the end day, the previous year. The original returns only the
/// low 16 bits meaningfully (the high ones are left over from the pushed slot). Called by the club records
/// `0x444930`/`0x447430`/`0x448ad0` and the player history `0x7c3e40`/`0x7c5fe0`.
/// Original: `key_nation_get_club_season_year` (0x669950, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_get_club_season_year(club: usize) -> u16 {
    let year = read_u16(G_CURRENT_DATE_YEAR);
    if club != 0 && exe::database_is_national_team(club) == 0 {
        let k = exe::key_nation_index_of_club(club) as i8 as isize;
        let e = (0xb63d60isize + 72 * k) as usize;
        let start = read_u16(e + key_nation_record::SEASON_END_DAY);
        if (read_usize(e + key_nation_record::SPLIT_SEASON_FLAG) != 0 || start < 0x3c) && (read_i16(G_CURRENT_DATE) as i32) < start as i32 {
            return year.wrapping_sub(1); // cpu1: the previous year
        }
    }
    year
}

/// Writes to dst (if not null) the date built by 0x549ef0 from day and year (with leap flag).
pub(crate) unsafe fn write_date(dst: usize, day: i16, year: u16) {
    if dst == 0 {
        return;
    }
    let mut b = [0u8; 8];
    exe::new_date((b.as_mut_ptr()) as *mut u8 as usize, day as u16 as u32, year as u32);
    core::ptr::copy_nonoverlapping(b.as_ptr(), dst as *mut u8, 8);
}

/// By the start and end days of the club's nation row and today, tells whether the club is in season and gives the
/// start and end dates with the right year.
///
/// Convention: cdecl, `key_nation.cpp`, SEH frame only for the local dates. `key_nation_club_in_season(club, &start,
/// &end)`. The row of the club's nation (`.nation()`) in the table of the 35 key nations (`G_LEAGUE_TABLE`, 72 bytes;
/// with no nation or outside the table, row 3 `G_LEAGUE_EUROPE_NATION` and the end day of the default date `0x54a110`,
/// 40). `a` = day `key_nation_record::START_DATE` of the row (start), `c` = the day of date
/// `key_nation_record::END_DATE` (end; the original also copies the next 7 bytes and, with club `+0x1db` ≠ −1 and byte
/// `+0x43` 3, 4 or 5, swaps two of them, but does not use them), `t` today's day (`G_CURRENT_DATE`) and `y` the year
/// (`G_CURRENT_DATE_YEAR`); signed 16-bit comparisons.
///
/// With a > c (a season that crosses the year): t > a, (a, y) and (c, y+1), 1; t < c, (a, y−1) and (c, y), 1; otherwise
/// (a, y) and (c, y), 0. With a ≤ c: t < a, (a, y) and (c, y), 0; t > c, (a, y+1) and (c, y+1), 1; otherwise (a, y) and
/// (c, y), 1. The dates go to `start` and `end` when not null.
///
/// Called by the club in season `0x547770`, the fitness `0x62a920`, `0x62c680` and `0x62ce30`, the month of the season
/// end `0x4d45f0` and the transfer date `0x8cee70`. TapeCall for every player: no counter.
/// Original: `key_nation_club_in_season` (0x669bd0, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_club_in_season(club: usize, start: usize, end: usize) -> u32 {
    let club = Club(club);
    let mut row = G_LEAGUE_EUROPE_NATION;
    let mut c: i16 = 0x28;
    let nation = club.nation().addr();
    if nation != 0 {
        if let Some(k) = (0..0x23usize).find(|&k| read_usize(G_LEAGUE_TABLE + 72 * k) == nation) {
            row = G_LEAGUE_TABLE + 72 * k;
            c = read_i16(row + key_nation_record::END_DATE);
        }
    }
    let a = read_i16(row + key_nation_record::START_DATE);
    let t = read_i16(G_CURRENT_DATE);
    let y = read_u16(G_CURRENT_DATE_YEAR);
    if a > c {
        if t > a {
            write_date(start, a, y);
            if end != 0 {
                write_date(end, c, y.wrapping_add(1)); // cpu2: the end in the following year
            }
            return 1;
        }
        if t < c {
            write_date(start, a, y.wrapping_sub(1));
            write_date(end, c, y);
            return 1; // cpu2: before the end, within the season
        }
        write_date(start, a, y);
        write_date(end, c, y);
        return 0; // cpu2: between the end and the start, outside
    }
    if t < a {
        write_date(start, a, y);
        write_date(end, c, y);
        return 0;
    }
    if t > c {
        write_date(start, a, y.wrapping_add(1));
        write_date(end, c, y.wrapping_add(1));
        return 1;
    }
    write_date(start, a, y);
    write_date(end, c, y);
    1 // cpu2: between the start and the end of the year
}

/// the continent of the human's nation, or that of the first main nation of the table
/// Original: `key_nation_get_main_continent` (0x6695c0, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_get_main_continent() -> usize {
    let h = Nation(read_usize(G_MAIN_NATION));
    if !h.is_null() && h.continent() != 0 {
        return h.continent();
    }
    for k in 0..0x23usize {
        let n = Nation(read_usize(G_LEAGUE_TABLE + 72 * k));
        if !n.is_null() && n.league_selected() & 2 != 0 {
            return n.continent();
        }
    }
    0
}

/// the human's nation, or the first main nation of the table
/// Original: `key_nation_get_main_nation` (0x669600, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_get_main_nation() -> usize {
    let h = read_usize(G_MAIN_NATION);
    if h != 0 {
        return h;
    }
    for k in 0..0x23usize {
        let n = Nation(read_usize(G_LEAGUE_TABLE + 72 * k));
        if !n.is_null() && n.league_selected() & 2 != 0 {
            return n.addr();
        }
    }
    0
}

/// The World Cup competition ids: the finals and the six qualifying competitions (Europe [0x9cf780], Africa, Asia,
/// CONCACAF, Oceania, South America).
pub(crate) const WORLD_CUP_COMPS: [usize; 7] = [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT];

/// The ids of the national-team tournaments outside the World Cup: Olympics, European Championship, European
/// Championship qualifiers, African Cup of Nations [0x9cf7ac], Copa América, B team, under-21, Oceania Nations Cup,
/// Confederations Cup and Asian Cup.
pub(crate) const INTERNATIONAL_TOURNAMENTS: [usize; 10] = [KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_EUROPEAN_CHAMPIONSHIP_QUALIFYING, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_B_INTERNATIONAL, KEY_CLUB_COMP_UNDER_21_INTERNATIONAL, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_CONFEDERATIONS_CUP, KEY_CLUB_COMP_ASIAN_CUP];

/// 1 when the competition id is the World Cup or one of its six qualifying sections
/// Original: `key_nation_is_world_cup_comp` (0x669a10, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_is_world_cup_comp(comp: usize) -> u32 {
    if comp == 0 {
        show_error_at(0x9eaff4, 0x3d7);
        return 0;
    }
    let id = read_usize(comp);
    WORLD_CUP_COMPS.iter().any(|&g| read_usize(g) == id) as u32
}

/// 1 when the competition id is one of the ten national side tournaments outside the World Cup
/// Original: `key_nation_is_international_comp` (0x669ae0, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_is_international_comp(comp: usize) -> u32 {
    if comp == 0 {
        show_error_at(0x9eaff4, 0x3ef);
        return 0;
    }
    let id = read_usize(comp);
    INTERNATIONAL_TOURNAMENTS.iter().any(|&g| read_usize(g) == id) as u32
}

/// Returns the competition's key nation row: the nation's row, 0 for the World Cup, 1 for national team tournaments, 2
/// for the club world cups, the continent's row or 3.
///
/// `key_nation_index_of_comp(comp)` (competition), called by the scope of the match rules `0x8d2fa0`, the league day
/// `0x6847c0`, the cup records `0x519890` and the match key nation `0x48dd70` of cpt_3. A nation competition
/// (`.nation()`): its row in the table of the 35 key nations (`G_LEAGUE_TABLE`, 72 bytes), or 4. With no nation: the
/// World Cup and its qualifiers (`KEY_CLUB_COMP_FIFA_WORLD_CUP` and the `KEY_CLUB_COMP_WORLD_CUP_*_QUALIFYING_SECTION`
/// keys), 0; national team tournament (`exe::key_nation_is_international_comp`), 1; the Club World Cup
/// (`KEY_CLUB_COMP_WORLD_CLUB_CUP`) and the FIFA Club World Championship
/// (`KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP`), 2; otherwise the continent row (nation 0) with the competition's
/// continent (`.continent()`, against `G_LEAGUE_WORLD_CUP_CONTINENT`), or 3.
///
/// The original reads `.nation()` before checking for a null competition (the error case never reaches the box).
/// TapeCall for every match: no counter.
/// Original: `key_nation_index_of_comp` (0x669470, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_index_of_comp(comp: usize) -> u8 {
    let comp = Competition(comp);
    let nation = comp.nation();
    if !nation.is_null() {
        let row = (0..0x23usize).find(|&k| read_usize(G_LEAGUE_TABLE + 72 * k) == nation.addr()); // cpt4: the nation search
        return match row {
            Some(k) => k as u8, // cpt4: the nation row
            None => 4,
        };
    }
    let id = read_usize(comp.addr());
    let cup = [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT]; // cpt4: the World Cup and the qualifiers
    if cup.iter().any(|&g| read_usize(g) == id) {
        return 0;
    }
    if exe::key_nation_is_international_comp(comp) != 0 {
        return 1;
    }
    if id == read_usize(KEY_CLUB_COMP_WORLD_CLUB_CUP) || id == read_usize(KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP) {
        return 2;
    }
    for k in 0..0x23usize {
        let c = comp.continent();
        if read_usize(G_LEAGUE_TABLE + 72 * k) == 0 && c != 0 && c == read_usize(G_LEAGUE_WORLD_CUP_CONTINENT + 72 * k) {
            return k as u8; // cpt4: the continent row
        }
    }
    3
}

/// Returns 1 when the key nation table holds a nation of the given continent chosen to be played, else 0.
///
/// `key_nation_continent_has_selected_nation(continent)`: over the 35 rows of `G_LEAGUE_TABLE` (72 bytes, nation at
/// +0), the first row whose nation is not null, has `.continent()` equal to the given one and `.league_selected()` not
/// zero gives 1. Called by the awards (0x4135b0), the discipline (0x55d420), the continental cups and the setup.
/// Original: `key_nation_continent_has_selected_nation` (0x6699d0, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_continent_has_selected_nation(continent: usize) -> u32 {
    for k in 0..0x23usize {
        let n = Nation(read_usize(G_LEAGUE_TABLE + 72 * k));
        if !n.is_null() && n.continent() == continent && n.league_selected() != 0 {
            covered!("sx12_1 continent has a selected nation");
            return 1; // sx12_1: a chosen nation of the continent
        }
    }
    0
}

/// Numbers the active rows of the key nation table and stores how many there are; returns 1.
///
/// `key_nation_number_active_entries()`: clears the count (`G_KEY_NATION_ACTIVE_COUNT`, a byte); a row of
/// `G_LEAGUE_TABLE` is active when its nation (+0) is null (the world and continent rows) or chosen to be played
/// (`.league_selected()` not zero); each active row gets the running count at `key_nation_record::ACTIVE_INDEX`, and
/// the count rises. The inactive rows keep their old index. Called by the game setup (0x828110, 0x82a9c0).
/// Original: `key_nation_number_active_entries` (0x669140, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_number_active_entries() -> u32 {
    let mut count: u8 = 0;
    write_u8(G_KEY_NATION_ACTIVE_COUNT, 0);
    for k in 0..0x23usize {
        let row = KeyNationRecord(G_LEAGUE_TABLE + 72 * k);
        let n = Nation(read_usize(row.addr())); // sx12_1: the row's nation
        if n.is_null() || n.league_selected() != 0 {
            covered!("sx12_1 active key nation row");
            row.set_active_index(count);
            count = count.wrapping_add(1); // sx12_1: the next active index
            write_u8(G_KEY_NATION_ACTIVE_COUNT, count);
        }
    }
    1
}

/// Comparer for `key_nation_continent_has_selected_nation`: the return, side by side (the routine only reads).
pub(crate) unsafe extern "C" fn compare_key_nation_continent_has_selected_nation(continent: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_KEY_NATION_CONTINENT_HAS_SELECTED_NATION)(continent);
    let n = key_nation_continent_has_selected_nation(continent);
    SCOREBOARD.record(r == n, || format!("key_nation_continent_has_selected_nation({:#x}) original {} rust {}", continent, r, n));
    r
}

/// Comparer for `key_nation_number_active_entries`: the 35 rows of the table and the count, side by side.
pub(crate) unsafe extern "C" fn compare_key_nation_number_active_entries() -> u32 {
    let o = original_fn::<unsafe extern "C" fn() -> u32>(IDX_KEY_NATION_NUMBER_ACTIVE_ENTRIES);
    let regions = [(G_LEAGUE_TABLE, 0x23 * 72), (G_KEY_NATION_ACTIVE_COUNT, 1)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(), || key_nation_number_active_entries());
    SCOREBOARD.record(equal && r == n, || format!("key_nation_number_active_entries() original {} rust {}: {:?}", r, n, first_differences(&mo, &mr)));
    r
}

/// Sorts the 35 key nation row indices into menu order and returns the static index array (0xb63d38).
///
/// `key_nation_sorted_order(use_short)`: writes 0..34 into the byte array at 0xb63d38, stores the flag in the dword at
/// 0xb63d5c (read by `key_nation_compare_entries`: compare the nations by their short name) and sorts the array with
/// the C runtime's qsort (`exe::crt_qsort`, 35 one-byte elements) and `key_nation_compare_entries` (0x66a100). Called
/// by the menu bar (0x75db00, every time it is built), the manager, search, setup and transfer screens.
/// Original: `key_nation_sorted_order` (0x66a0c0, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_sorted_order(use_short: u32) -> usize {
    for k in 0..0x23u8 {
        write_u8(0xb63d38 + k as usize, k); // the static index array
    }
    write_u32(0xb63d5c, use_short); // the flag of the comparison
    covered!("sx12_2 key nation order");
    exe::crt_qsort(0xb63d38usize, 0x23, 1, 0x66a100usize); // sx12_2: key_nation_compare_entries
    0xb63d38
}

/// Orders the three world rows first by their index, then those of the continents by the continent's name, then the
/// nations by name.
///
/// `key_nation_compare_entries(a, b)` (qsort, one-byte row indices of `G_LEAGUE_TABLE`, nation at +0 and continent at
/// +4 (`G_LEAGUE_WORLD_CUP_CONTINENT`)): a null a gives 1 and a null b -1. A row with neither nation nor continent (a
/// world row) goes before any other; two of them by the index (0 before 1 before 2; other indices: equal). A row with
/// a continent goes before a row with neither: with both continents, `exe::utils_string_compare` of their names (+4); a
/// with a continent and b without: -1; a without (a nation row) and b with: 1. Two nation rows: by the nations' names
/// (+4), or their short names (`nation::SHORT_NAME`) when the flag at 0xb63d5c is set.
/// Original: `key_nation_compare_entries` (0x66a100, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_compare_entries(a: *const i8, b: *const i8) -> i32 {
    if a.is_null() {
        return 1;
    }
    if b.is_null() {
        return -1;
    }
    let (ia, ib) = (*a as isize, *b as isize);
    let row = |i: isize| (i * 72) as usize;
    let (na, ca) = (read_usize(G_LEAGUE_TABLE.wrapping_add(row(ia))), read_usize(G_LEAGUE_WORLD_CUP_CONTINENT.wrapping_add(row(ia))));
    let (nb, cb) = (read_usize(G_LEAGUE_TABLE.wrapping_add(row(ib))), read_usize(G_LEAGUE_WORLD_CUP_CONTINENT.wrapping_add(row(ib))));
    if na == 0 && ca == 0 {
        if nb != 0 || cb != 0 {
            return -1;
        }
        covered!("sx12_2 world rows");
        // the world rows by their index (only 0, 1 and 2 are ordered)
        return if (0..=2).contains(&ia) && (0..=2).contains(&ib) { (ia - ib).signum() as i32 } else { 0 };
    }
    if nb == 0 && cb == 0 {
        return 1;
    }
    if ca != 0 {
        if cb == 0 {
            return -1;
        }
        covered!("sx12_2 continent rows");
        return exe::utils_string_compare((ca + 4) as *const u8, (cb + 4) as *const u8, 1, 0); // the continents' names
    }
    if cb != 0 {
        return 1;
    }
    if na == 0 || nb == 0 {
        return 0;
    }
    covered!("sx12_2 nation rows");
    if read_u32(0xb63d5c) != 0 {
        exe::utils_string_compare((na + nation::SHORT_NAME) as *const u8, (nb + nation::SHORT_NAME) as *const u8, 1, 0)
    } else {
        exe::utils_string_compare((na + 4) as *const u8, (nb + 4) as *const u8, 1, 0) // sx12_2: the nations' names
    }
}

/// Returns 2 when a nation menu holds more than 16 nations and is shown in two parts, else 1.
///
/// `NATION_MENU_SPLIT::part_count()`: the signed byte count at +9 above 16 gives 2. Only al is the return value (the
/// original increments the whole eax, whose high bytes are left over).
/// Original: `NATION_MENU_SPLIT::part_count` (0x66a350, key_nation.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn nation_menu_split_part_count(this: usize) -> u8 {
    covered!("sx12_2 menu parts");
    (read_i8(this + 9) > 16) as u8 + 1 // sx12_2: more than 16 nations
}

/// Comparer for `key_nation_sorted_order`: the index array and the flag (0xb63d38, 0x28 bytes) and the return, side by
/// side.
pub(crate) unsafe extern "C" fn compare_key_nation_sorted_order(use_short: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32) -> usize>(IDX_KEY_NATION_SORTED_ORDER);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(0xb63d38, 0x28)], || o(use_short), || key_nation_sorted_order(use_short));
    SCOREBOARD.record(equal && r == n, || format!("key_nation_sorted_order({}) original {:#x} rust {:#x}: {:?}", use_short, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `key_nation_compare_entries`: the return, side by side (the routine only reads).
pub(crate) unsafe extern "C" fn compare_key_nation_compare_entries(a: *const i8, b: *const i8) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(*const i8, *const i8) -> i32>(IDX_KEY_NATION_COMPARE_ENTRIES)(a, b);
    let n = key_nation_compare_entries(a, b);
    SCOREBOARD.record(r == n, || format!("key_nation_compare_entries({:?}, {:?}) original {} rust {}",
        if a.is_null() { None } else { Some(*a) }, if b.is_null() { None } else { Some(*b) }, r, n));
    r
}

/// Comparer for `nation_menu_split_part_count`: al, side by side.
pub(crate) unsafe extern "thiscall" fn compare_nation_menu_split_part_count(this: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_NATION_MENU_SPLIT_PART_COUNT)(this);
    let n = nation_menu_split_part_count(this);
    SCOREBOARD.record(r == n, || format!("NATION_MENU_SPLIT::part_count(count {}) original {} rust {}", read_i8(this + 9), r, n));
    r
}

/// Tells every competition of each key nation row to save itself to its temporary file; returns 1.
///
/// `key_nation_save_all_comps()`: for each of the 35 rows of `G_LEAGUE_TABLE` with a competition array (+0x10, count at
/// +0xc), each competition that is not null gets its virtual `comp_vtable::SAVE_TO_TEMP_FILE` with 1. Called by the
/// game save (step 0x19 of 0x82e430).
/// Original: `key_nation_save_all_comps` (0x6692f0, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_save_all_comps() -> u32 {
    for k in 0..0x23usize {
        let row = G_LEAGUE_TABLE + 72 * k;
        if read_usize(row + 0x10) == 0 {
            continue; // no competition array
        }
        let mut i = 0;
        while i < read_i32(row + 0xc) {
            let comp = read_usize(read_usize(row + 0x10) + 4 * i as usize);
            if comp != 0 {
                covered!("sx12_5 save competition");
                let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(read_usize(read_usize(comp) + comp_vtable::SAVE_TO_TEMP_FILE));
                f(comp, 1); // sx12_5: the competition's temporary file
            }
            i += 1;
        }
    }
    1
}

/// Comparer for `key_nation_save_all_comps`: only the original runs (each competition writes its temporary file);
/// proven by the save proof.
pub(crate) unsafe extern "C" fn compare_key_nation_save_all_comps() -> u32 {
    original_fn::<unsafe extern "C" fn() -> u32>(IDX_KEY_NATION_SAVE_ALL_COMPS)()
}

/// Builds a nation menu split: how many nations the menu holds and the letter where its second part starts.
///
/// `NATION_MENU_SPLIT::NATION_MENU_SPLIT(selected_only, order, use_short)` (0xe bytes: +0 the split letter, +1 the
/// order array, +5 the selected-only flag, +9 the nation count, +0xa the short-name flag): clears the letter and stores
/// the three arguments. The count is the rows of `G_LEAGUE_TABLE` whose nation is not null and chosen to be played
/// (`.league_selected()` not zero; with `selected_only`, its bit 2 set). Then, over the 35 indices of `order` (signed
/// bytes), the same nations in that order: the one whose running number equals (count + 1) / 2 (signed, rounded toward
/// zero) gives the split letter, the first character of its short name (`nation::SHORT_NAME`) with `use_short`, else of
/// its name (+4). Returns `this`.
/// Original: `NATION_MENU_SPLIT::NATION_MENU_SPLIT` (0x66a280, key_nation.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn nation_menu_split_construct(this: usize, selected_only: u32, order: usize, use_short: u32) -> usize {
    write_u32(this + 5, selected_only);
    write_usize(this + 1, order);
    write_u8(this, 0);
    write_u32(this + 0xa, use_short);
    let shown = |n: Nation| {
        let s = if n.is_null() { 0 } else { n.league_selected() };
        s != 0 && (selected_only == 0 || s & 2 != 0)
    };
    let mut count: u8 = 0;
    for k in 0..0x23usize {
        if shown(Nation(read_usize(G_LEAGUE_TABLE + 72 * k))) {
            count = count.wrapping_add(1);
        }
    }
    write_u8(this + 9, count);
    let middle = (count as i8 as i32 + 1) / 2; // sx12_3: the middle nation
    let mut seen: i8 = 0;
    for k in 0..0x23usize {
        let n = Nation(read_usize(G_LEAGUE_TABLE.wrapping_add((read_i8(order + k) as isize * 72) as usize)));
        if !shown(n) {
            continue;
        }
        seen = seen.wrapping_add(1);
        if seen as i32 == middle {
            covered!("sx12_3 split letter");
            write_u8(this, if use_short != 0 { n.short_name() } else { read_u8(n.addr() + 4) }); // the name's first letter
        }
    }
    this
}

/// Writes the header of one part of a split nation menu: "Nations A-{letter}" for the first part, "Nations
/// {next letter}-Z" for the second.
///
/// `NATION_MENU_SPLIT::format_header(text, size, second)`: a size below 20 shows the "Error" box (key_nation.cpp line
/// 0x57f) and writes nothing. Otherwise the game's text formatter (`FN_LANGLIB_SPRINTF`) with "Nations A-<%c -
/// Character>" and the split letter (+0, signed), or, when the low byte of `second` is set, "Nations <%c -
/// Character>-Z" and the letter plus one.
/// Original: `NATION_MENU_SPLIT::format_header` (0x66a360, key_nation.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn nation_menu_split_format_header(this: usize, text: usize, size: i32, second: u32) {
    if size < 0x14 {
        show_error_at(0x9eaff4, 0x57f); // "...\key_nation.cpp"
        return;
    }
    let letter = read_i8(this) as i32;
    if second as u8 == 0 {
        covered!("sx12_3 first part header");
        story_lang!(text, 0x9eb0cc, letter); // "Nations A-<%c - Character>"
    } else {
        covered!("sx12_3 second part header");
        story_lang!(text, 0x9eb0b0, letter + 1); // sx12_3: "Nations <%c - Character>-Z"
    }
}

/// Tells whether a nation goes in the second part of a split menu: the first character of its short name
/// (`nation::SHORT_NAME`, with the short-name flag at +0xa) or of its name (+4) above the split letter (+0), signed.
/// Only al is the return value.
/// Original: `NATION_MENU_SPLIT::in_second_part` (0x66a430, key_nation.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn nation_menu_split_in_second_part(this: usize, nation: usize) -> u8 {
    let n = Nation(nation);
    let first = if read_u32(this + 0xa) != 0 { n.short_name() } else { read_u8(n.addr() + 4) };
    covered!("sx12_3 second part");
    (first as i8 > read_i8(this)) as u8 // sx12_3: after the split letter
}

/// Comparer for `nation_menu_split_construct`: the 0xe bytes of the object and the return, side by side.
pub(crate) unsafe extern "thiscall" fn compare_nation_menu_split_construct(this: usize, selected_only: u32, order: usize, use_short: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32) -> usize>(IDX_NATION_MENU_SPLIT_CONSTRUCT);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(this, 0xe)], || o(this, selected_only, order, use_short),
        || nation_menu_split_construct(this, selected_only, order, use_short));
    SCOREBOARD.record(equal && r == n, || format!("NATION_MENU_SPLIT::NATION_MENU_SPLIT({}, {:#x}, {}): {:?}", selected_only, order, use_short, first_differences(&mo, &mr)));
    r
}

/// Comparer for `nation_menu_split_format_header`: the original writes into the caller's text and the copy into a
/// local one; the two strings side by side. A size below 20 (the "Error" box): only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_nation_menu_split_format_header(this: usize, text: usize, size: i32, second: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32)>(IDX_NATION_MENU_SPLIT_FORMAT_HEADER);
    o(this, text, size, second);
    if size < 0x14 {
        return;
    }
    let mut copy = [0u8; 0x100];
    nation_menu_split_format_header(this, copy.as_mut_ptr() as usize, size, second);
    let a = core::ffi::CStr::from_ptr(text as *const core::ffi::c_char).to_bytes();
    let b = core::ffi::CStr::from_ptr(copy.as_ptr() as *const core::ffi::c_char).to_bytes();
    SCOREBOARD.record(a == b, || format!("NATION_MENU_SPLIT::format_header(second {}) original {:?} rust {:?}", second as u8,
        String::from_utf8_lossy(a), String::from_utf8_lossy(b)));
}

/// Comparer for `nation_menu_split_in_second_part`: al, side by side (the routine only reads).
pub(crate) unsafe extern "thiscall" fn compare_nation_menu_split_in_second_part(this: usize, nation: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_NATION_MENU_SPLIT_IN_SECOND_PART)(this, nation);
    let n = nation_menu_split_in_second_part(this, nation);
    SCOREBOARD.record(r == n, || format!("NATION_MENU_SPLIT::in_second_part(nation {:#x}) original {} rust {}", nation, r, n));
    r
}

/// Builds the competitions of every active key nation row and sets each row's season dates; returns 1, or 0 after the
/// "Error" box when a row's creation fails.
///
/// `key_nation_create_all_comps()`: for each of the 35 rows of `G_LEAGUE_TABLE` whose nation (+0) is null or chosen
/// to be played (`.league_selected()`) and which has a creation function (`G_LEAGUE_WORLD_CUP_SETUP_FUNC`, +8, cdecl with
/// the row): the function; returning 0: the box (key_nation.cpp line 0x1dc) and 0. Then the row's season start
/// (`key_nation_record::START_DATE`) becomes the date of day +0x32 (word), month +0x34 (byte), year +0x35 and weekday
/// +0x37 (words, `exe::date_date_549f70`), and the season end (`key_nation_record::END_DATE`) the date of day +0x41,
/// month +0x43, year +0x44 and weekday +0x46. Called by the game setup (0x828110, 0x82a9c0: load step 0x1f).
/// Original: `key_nation_create_all_comps` (0x669180, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_create_all_comps() -> u32 {
    for k in 0..0x23usize {
        let row = G_LEAGUE_TABLE + 72 * k;
        let n = Nation(read_usize(row));
        if !n.is_null() && n.league_selected() == 0 {
            continue;
        }
        let create = read_usize(row + 8); // G_LEAGUE_WORLD_CUP_SETUP_FUNC of the row
        if create == 0 {
            continue;
        }
        let create: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(create);
        if create(row) == 0 {
            show_error_at(0x9eaff4, 0x1dc); // "...\key_nation.cpp"
            return 0;
        }
        covered!("sx12_6 key nation competitions");
        set_key_nation_season_dates(row); // sx12_6: the season dates
    }
    1
}

/// Comparer for `key_nation_create_all_comps`: the creation functions build the competitions once, so only the original
/// runs; afterwards each active row's season dates are checked against the copy's (`set_key_nation_season_dates` on a
/// copy of the row).
pub(crate) unsafe extern "C" fn compare_key_nation_create_all_comps() -> u32 {
    let r = original_fn::<unsafe extern "C" fn() -> u32>(IDX_KEY_NATION_CREATE_ALL_COMPS)();
    if r == 0 {
        return r;
    }
    for k in 0..0x23usize {
        let row = G_LEAGUE_TABLE + 72 * k;
        let n = Nation(read_usize(row));
        if (!n.is_null() && n.league_selected() == 0) || read_usize(row + 8) == 0 {
            continue;
        }
        let mut copy = [0u8; 72];
        core::ptr::copy_nonoverlapping(row as *const u8, copy.as_mut_ptr(), 72);
        set_key_nation_season_dates(copy.as_mut_ptr() as usize);
        let (a, b) = (core::slice::from_raw_parts(row as *const u8, 72), &copy[..]);
        SCOREBOARD.record(a == b, || format!("key_nation_create_all_comps row {}: original {:02x?} rust {:02x?}", k, a, b));
    }
    r
}

/// Sets a key nation row's season start and end dates from their day, month, year and weekday fields (see
/// `key_nation_create_all_comps`).
pub(crate) unsafe fn set_key_nation_season_dates(row: usize) {
    let mut t = [0u8; 8];
    for (date, parts) in [(key_nation_record::START_DATE, 0x32), (key_nation_record::END_DATE, 0x41)] {
        let mut d = [0u16; 4];
        let built = exe::date_date_549f70(&mut d, read_u16(row + parts) as u32, read_u8(row + parts + 2) as u32,
            read_u16(row + parts + 3) as u32, read_u16(row + parts + 5) as u32);
        exe::date_operator_3d(row + date, t.as_mut_ptr() as usize, built);
    }
}

/// The day a nation's month award moves on to the next season: today's day word (signed) is the season end day of the
/// nation's key nation record (`key_nation_record::SEASON_END_DAY` of `G_LEAGUE_TABLE` at
/// `exe::key_nation_index_of_nation`).
pub(crate) unsafe fn is_season_end_day(nation: usize, today: usize) -> bool {
    let k = exe::key_nation_index_of_nation(nation) as isize;
    read_i16(today) as i32 == read_u16((G_LEAGUE_TABLE as isize + k * 0x48) as usize + key_nation_record::SEASON_END_DAY) as i32
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
    let o = original_fn::<unsafe extern "C" fn() -> u32>(IDX_KEY_NATION_BUILD_TABLE);
    if read_usize(G_NATION_TABLE) == 0 {
        return o();
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(G_LEAGUE_TABLE, 0x23 * 72)], || o(), || key_nation_build_table());
    SCOREBOARD.record(equal && r == n, || format!("key_nation_build_table original {} rust {}: {:?}", r, n, first_differences(&mo, &mr)));
    r
}

/// Builds one key nation entry (72 bytes): only its season start (`key_nation_record::START_DATE`) and end
/// (`key_nation_record::END_DATE`) dates (`Date::Date` empty). Returns the entry.
/// Original: `KEY_NATION::KEY_NATION` (0x667100, key_nation.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn construct_key_nation(this: usize) -> usize {
    exe::date_date_54a110(this + key_nation_record::START_DATE);
    exe::date_date_54a110(this + key_nation_record::END_DATE); // sx12b_3: the end date
    covered!("sx12b_3 key nation entry");
    this
}

/// Comparer for `construct_key_nation`: the entry, side by side.
pub(crate) unsafe extern "thiscall" fn compare_construct_key_nation(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_CONSTRUCT_KEY_NATION);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side_light(&[(this, 72)], || o(this), || construct_key_nation(this));
    SCOREBOARD.record(equal && r == n, || format!("KEY_NATION::KEY_NATION({:#x}): {:?}", this, first_differences(&mo, &mr)));
    r
}

/// The record of a fixed key in a database table of `size`-byte records (`table` holds the table's address): table +
/// key x size, in 32-bit arithmetic as the original.
pub(crate) unsafe fn keyed_record(table: usize, key: usize, size: usize) -> usize {
    read_usize(table).wrapping_add((read_i32(key) as usize).wrapping_mul(size))
}

/// Writes a season date into a key nation row: `Date::Date` (0x549f70) of day, month, year and weekday into a
/// temporary, then `Date::operator=` into the row.
pub(crate) unsafe fn write_key_nation_date(at: usize, d: KeyNationDate) {
    let year = read_u16(G_CURRENT_DATE_YEAR).wrapping_add(d.next_year as u16);
    let (mut built, mut tmp) = ([0u16; 4], [0u8; 8]);
    let p = exe::date_date_549f70(&mut built, d.day, d.month, year as u32, d.weekday as u32);
    exe::date_operator_3d(at, tmp.as_mut_ptr() as usize, p);
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
pub(crate) const KEY_NATION_CUP: usize = 0x1e;

pub(crate) const KEY_NATION_LEAGUE_CUP: usize = 0x22;

/// One of the 35 rows `key_nation_build_table` writes: the nation (`KEY_NATION_*`, 0 = none) or continent
/// (`KEY_CONTINENT_*`), the competition creation function (an exe address), the season end (day and month of 2001),
/// the split season flag, the cup and league cup (`KEY_CLUB_COMP_*`, 0 = none) and the season start and end dates.
pub(crate) struct KeyNationRow {
    pub(crate) nation: usize,
    pub(crate) continent: usize,
    pub(crate) create: usize,
    pub(crate) season_end: Option<(u32, u32)>,
    pub(crate) split_season: bool,
    pub(crate) cup: usize,
    pub(crate) league_cup: usize,
    pub(crate) start: KeyNationDate,
    pub(crate) end: KeyNationDate,
}
