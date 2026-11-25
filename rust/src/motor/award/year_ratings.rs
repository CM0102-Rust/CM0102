use super::*;

/// Adds the first eleven of both sides of a match to the year ratings, for the players the filter accepts.
///
/// `YEAR_RATINGS::record_match_ratings(fixture, record, filter)`. Not for a friendly (`fixture::CATEGORY` 1). For each
/// side (the home club `fixture::HOME_CLUB` with the record's first block, the away club with the block
/// `match_record::SIDE_SIZE` on), the first 11 of the 20 rows: an empty one (staff id −1 or position
/// `row_slot::EMPTY`) and a virtual player (staff id at or above `G_STAFF_COUNT`) are skipped; the rest go to
/// `add_player_rating` when `filter(person, fixture)` (cdecl) accepts them.
/// Original: `YEAR_RATINGS::record_match_ratings` (0x9315d0, award/year_ratings.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn year_record_match_ratings(this: usize, fixture: usize, record: usize, filter: usize) {
    let fixture = Fixture(fixture);
    if fixture.category() == 1 {
        return;
    }
    let accepts: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(filter);
    for side in 0..2usize {
        let club = read_usize(fixture.addr() + if side == 0 { fixture::HOME_CLUB } else { fixture::AWAY_CLUB });
        for k in 0..20usize {
            let row = record + 0x190 + side * 0x625 + k * match_record_row::SIZE;
            let id = read_i32(row + match_record_row::STAFF_ID);
            if id == -1 || read_u8(row + match_record_row::POSITION) == row_slot::EMPTY || k >= 11 { // mwg5: the first eleven
                continue;
            }
            let person = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
            if person != 0 && read_i32(person) >= read_i32(G_STAFF_COUNT) {
                continue;
            }
            if accepts(person, fixture.addr()) != 0 {
                if side == 1 {
                    covered!("mwg_5 away side rating");
                }
                exe::year_ratings_add_player_rating(this, row, fixture, club);
            }
        }
    }
}

/// Comparer for `year_record_match_ratings`.
///
/// Side by side over the table's header and its entries plus room for 22 new ones; when the table could grow (less
/// room, or none yet), only the original runs (each added rating has its own comparer).
pub(crate) unsafe extern "thiscall" fn compare_year_record_match_ratings(this: usize, fixture: usize, record: usize, filter: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_YEAR_RECORD_MATCH_RATINGS);
    let (tab, n, cap) = (read_usize(this), read_i32(this + 4), read_i32(this + 8));
    if tab == 0 || n < 0 || cap.wrapping_sub(n) < 22 {
        return o(this, fixture, record, filter);
    }
    let regions = [(this, 12), (tab, (n as usize + 22) * year_ratings_consts::ENTRY_SIZE)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, fixture, record, filter), || year_record_match_ratings(this, fixture, record, filter));
    SCOREBOARD.record(equal, || format!("year_record_match_ratings(fixture {:x}) original={:x?} rust={:x?}", fixture, mo, mr));
}

/// Finds or appends a player's entry in the year ratings and adds a match to it.
///
/// `YEAR_RATINGS::add_player_rating(row, fixture, club)`. The entry of the row's staff id is searched in the table;
/// missing, it is appended (the table grows by 400 entries, `exe::crt_malloc` / `exe::crt_realloc`; without memory the
/// game's error box, line 0x113, and nothing more) with the person's nation id (`staff::NATION`, −1 none) and zeroed
/// sums. Then the three weighted rating sums (`year_rating_sums`), one more match, the row's goals and assists, and
/// yellow cards + 2 × red cards.
/// Original: `YEAR_RATINGS::add_player_rating` (0x931690, award/year_ratings.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn year_add_player_rating(this: usize, row: usize, fixture: usize, club: usize) {
    let fixture = Fixture(fixture);
    use year_ratings_consts as yr;
    let comp_id = fixture.comp_id();
    let comp_obj = if comp_id >= 0 { read_usize(read_usize(G_COMP_OBJECT_TABLE) + comp_id as usize * 4) } else { 0 };
    let id = read_i32(row + match_record_row::STAFF_ID);
    let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE)));
    let n = read_i32(this + 4);
    let mut table = read_usize(this);
    let found = (0..n.max(0) as usize).map(|k| table + k * yr::ENTRY_SIZE).find(|&e| read_i32(e) == id);
    let e = match found {
        Some(e) => e,
        None => {
            if n == read_i32(this + 8) {
                let cap = n.wrapping_add(yr::GROWTH);
                write_i32(this + 8, cap);
                let bytes = (cap as u32).wrapping_mul(yr::ENTRY_SIZE as u32);
                table = if table == 0 { exe::crt_malloc(bytes) } else { exe::crt_realloc(table, bytes) };
                write_usize(this, table);
                if table == 0 {
                    show_error_at(yr::YEAR_RATINGS_CPP, 0x113);
                    return;
                }
            }
            covered!("mwg_5 new player in the year");
            let e = table + n as usize * yr::ENTRY_SIZE;
            write_i32(e, id);
            let nation = person.nation();
            write_i32(e + 4, if nation.is_null() { -1 } else { read_i32(nation.addr()) });
            for k in [8, 0xa, 0xc, 0xf, 0x11, 0x13] {
                write_u16(e + k, 0);
            }
            write_u8(e + 0xe, 0);
            write_i32(this + 4, n.wrapping_add(1));
            e
        }
    };
    let grid = read_u8(row + match_record_row::MATCH_GRADE) as i32;
    let (a, b, c) = year_rating_sums(fixture.addr(), Club(club), comp_obj, grid);
    let add16 = |k: usize, v: u16| write_u16(e + k, read_u16(e + k).wrapping_add(v));
    add16(8, a as i16 as u16);
    add16(0xa, b as i16 as u16);
    add16(0xc, c as i16 as u16);
    write_u8(e + 0xe, read_u8(e + 0xe).wrapping_add(1));
    add16(0xf, read_u8(row + match_record_row::GOALS) as u16);
    add16(0x11, read_u8(row + match_record_row::ASSISTS) as u16);
    add16(0x13, read_u8(row + match_record_row::YELLOW_CARDS) as u16 + 2 * read_u8(row + match_record_row::RED_CARD) as u16); // mwg5: a red card counts twice
}

/// Comparer for `year_add_player_rating`.
///
/// Side by side over the header and the entries (one more when it appends with room); when the table grows, only the
/// original runs.
pub(crate) unsafe extern "thiscall" fn compare_year_add_player_rating(this: usize, row: usize, fixture: usize, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_YEAR_ADD_PLAYER_RATING);
    let (tab, n, cap) = (read_usize(this), read_i32(this + 4), read_i32(this + 8));
    let size = year_ratings_consts::ENTRY_SIZE;
    let found = tab != 0 && (0..n.max(0) as usize).any(|k| read_i32(tab + k * size) == read_i32(row));
    if tab == 0 || n < 0 || (n >= cap && !found) {
        return o(this, row, fixture, club);
    }
    let slots = if n < cap { n as usize + 1 } else { n as usize };
    let regions = [(this, 12), (tab, slots * size)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, row, fixture, club), || year_add_player_rating(this, row, fixture, club));
    SCOREBOARD.record(equal, || format!("year_add_player_rating(person {}, fixture {:x}) original={:x?} rust={:x?}", read_i32(row), fixture, mo, mr));
}

/// The three weighted rating sums a match adds to a year ratings entry (the +8, +0xa and +0xc words).
///
/// The weight: with the fixture's competition (`fixture::COMPETITION`), the competition's weight word × (club
/// reputation / 500) / 20 + (club reputation − 5000) / 1700, added as bytes and kept between 1 and 20 (without it the
/// original reads a stack byte it never wrote; the copy uses 0 [inference: no competition object then]). Without a
/// competition object for the fixture's id, the match grade × (club reputation / 500, as a byte, × 16 when the
/// nation's reputation is above 7000, × 8 above 4000, × 4 otherwise, / 20, as a byte) gives /20, /60 and /80; with
/// one, the grade × the weight gives /20, /20, /40 for competition type 2, /20 three times for type 4, and /20, /60,
/// /80 otherwise.
pub(crate) unsafe fn year_rating_sums(fixture: usize, club: Club, comp_obj: usize, grid: i32) -> (i8, i8, i8) {
    let fixture = Fixture(fixture);
    use year_ratings_consts as yr;
    let mut weight = 0i8;
    let competition = fixture.competition().addr();
    if competition != 0 {
        let rep = club.reputation() as i32;
        let a = (read_i16(competition + yr::COMP_WEIGHT) as i32).wrapping_mul(rep / 500) / 20;
        let b = (rep - 5000) / 1700;
        weight = (a as u8).wrapping_add(b as u8) as i8;
        weight = weight.clamp(1, 20); // mwg5: the competition weight within 1..20
    }
    let wide = |v: i32| ((v / 20) as i8, (v / 60) as i8, (v / 80) as i8);
    if comp_obj == 0 {
        covered!("mwg_5 fixture without a competition object");
        let rep = club.reputation() as i32;
        let by_rep = (rep / 500) as i8 as i32;
        let nation_rep = club.nation().reputation();
        let scaled = if nation_rep > 7000 { by_rep << 4 } else if nation_rep > 4000 { by_rep << 3 } else { by_rep << 2 };
        return wide(grid * (scaled / 20) as i8 as i32);
    }
    let v = grid * weight as i32;
    match read_i8(comp_obj + yr::COMP_TYPE) {
        2 => ((v / 20) as i8, (v / 20) as i8, (v / 40) as i8),
        4 => ((v / 20) as i8, (v / 20) as i8, (v / 20) as i8),
        _ => wide(v),
    }
}

/// Orders two year ratings entries for rating mode 0: the score of `year_rating_score` with the +8 rating sum doubled
/// (`year_entries_order`: at least 3 matches, the higher score first).
/// Original: `year_ratings_compare_mode_0` (0x931f90, award/year_ratings.cpp, cdecl).
pub(crate) unsafe extern "C" fn year_ratings_compare_mode_0(a: usize, b: usize) -> i32 {
    year_entries_order(YearEntry(a), YearEntry(b), |e| year_rating_score(e, 0))
}

/// Orders two year ratings entries for rating mode 1: the score of `year_rating_score` with the +0xa rating sum
/// doubled.
/// Original: `year_ratings_compare_mode_1` (0x9320e0, award/year_ratings.cpp, cdecl).
pub(crate) unsafe extern "C" fn year_ratings_compare_mode_1(a: usize, b: usize) -> i32 {
    year_entries_order(YearEntry(a), YearEntry(b), |e| year_rating_score(e, 1))
}

/// Orders two year ratings entries for rating mode 2: the score of `year_rating_score` with the +0xc rating sum
/// doubled, then × the player's current ability × 0.00625 (when he has a player sheet).
/// Original: `year_ratings_compare_mode_2` (0x932230, award/year_ratings.cpp, cdecl).
pub(crate) unsafe extern "C" fn year_ratings_compare_mode_2(a: usize, b: usize) -> i32 {
    year_entries_order(YearEntry(a), YearEntry(b), |e| {
        let score = year_rating_score(e, 2);
        let sheet = e.person().player();
        if sheet.is_null() {
            return score;
        }
        covered!("sx05_2 mode 2 by ability");
        score * sheet.current_ability() as f64 * (0.00625f32 as f64)
    })
}

pub(crate) unsafe extern "C" fn compare_year_ratings_compare_mode_0(a: usize, b: usize) -> i32 {
    compare_year_order(IDX_YEAR_RATINGS_COMPARE_MODE_0, "year_ratings_compare_mode_0", a, b, year_ratings_compare_mode_0)
}

pub(crate) unsafe extern "C" fn compare_year_ratings_compare_mode_1(a: usize, b: usize) -> i32 {
    compare_year_order(IDX_YEAR_RATINGS_COMPARE_MODE_1, "year_ratings_compare_mode_1", a, b, year_ratings_compare_mode_1)
}

pub(crate) unsafe extern "C" fn compare_year_ratings_compare_mode_2(a: usize, b: usize) -> i32 {
    compare_year_order(IDX_YEAR_RATINGS_COMPARE_MODE_2, "year_ratings_compare_mode_2", a, b, year_ratings_compare_mode_2)
}

/// Orders two year ratings entries for mode 3: the goals score (`year_goals_score`) × the reputation of the person's
/// club × 0.0002 (when he has a club) × his current ability × 0.02 (when he has a player sheet); at least 3 matches,
/// the higher score first (`year_entries_order`).
/// Original: `year_ratings_compare_mode_3` (0x9323b0, award/year_ratings.cpp, cdecl).
pub(crate) unsafe extern "C" fn year_ratings_compare_mode_3(a: usize, b: usize) -> i32 {
    year_entries_order(YearEntry(a), YearEntry(b), |e| {
        let mut score = year_goals_score(e);
        let club = e.person().club_job();
        if !club.is_null() {
            covered!("sx05_3 mode 3 by club reputation");
            score = score * club.reputation() as f64 * (0.0002f32 as f64);
        }
        let sheet = e.person().player();
        if !sheet.is_null() {
            score = score * sheet.current_ability() as f64 * (0.02f32 as f64);
        }
        score
    })
}

/// Orders two year ratings entries for mode 4: cards (yellow + 2 × red) per match, the most first.
/// Original: `year_ratings_compare_mode_4` (0x932500, award/year_ratings.cpp, cdecl).
pub(crate) unsafe extern "C" fn year_ratings_compare_mode_4(a: usize, b: usize) -> i32 {
    year_entries_order(YearEntry(a), YearEntry(b), |e| {
        covered!("sx05_3 mode 4 cards");
        e.cards() as f64 / e.matches() as f64
    })
}

/// Orders two year ratings entries for mode 5: the goals score (`year_goals_score`) × (current reputation × 0.0002 +
/// 4) when the person has a player sheet, halved when his determination, loyalty or sportsmanship is below 10.
/// Original: `year_ratings_compare_mode_5` (0x932590, award/year_ratings.cpp, cdecl).
pub(crate) unsafe extern "C" fn year_ratings_compare_mode_5(a: usize, b: usize) -> i32 {
    year_entries_order(YearEntry(a), YearEntry(b), |e| {
        let person = e.person();
        let mut score = year_goals_score(e);
        let sheet = person.player();
        if !sheet.is_null() {
            score *= sheet.current_reputation() as f64 * (0.0002f32 as f64) + 4.0;
        }
        if person.determination() < 10 || person.loyalty() < 10 || person.sportsmanship() < 10 {
            covered!("sx05_3 mode 5 poor character");
            score *= 0.5;
        }
        score
    })
}

pub(crate) unsafe extern "C" fn compare_year_ratings_compare_mode_3(a: usize, b: usize) -> i32 {
    compare_year_order(IDX_YEAR_RATINGS_COMPARE_MODE_3, "year_ratings_compare_mode_3", a, b, year_ratings_compare_mode_3)
}

pub(crate) unsafe extern "C" fn compare_year_ratings_compare_mode_4(a: usize, b: usize) -> i32 {
    compare_year_order(IDX_YEAR_RATINGS_COMPARE_MODE_4, "year_ratings_compare_mode_4", a, b, year_ratings_compare_mode_4)
}

pub(crate) unsafe extern "C" fn compare_year_ratings_compare_mode_5(a: usize, b: usize) -> i32 {
    compare_year_order(IDX_YEAR_RATINGS_COMPARE_MODE_5, "year_ratings_compare_mode_5", a, b, year_ratings_compare_mode_5)
}

/// Frees the year ratings and empties the table when `year` is the table's year.
/// Original: `YEAR_RATINGS::reset_if_year` (0x931aa0, award/year_ratings.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn year_ratings_reset_if_year(this: usize, year: *const i16) {
    if *year as i32 != read_u16(this + 0xc) as i32 {
        return;
    }
    covered!("sx05_4 year ratings reset");
    if read_usize(this) != 0 {
        exe::crt_free(read_usize(this));
        write_usize(this, 0);
    }
    write_i32(this + 4, 0);
    write_i32(this + 8, 0);
}

/// Sorts the year ratings by `mode` (0 to 5) and collects up to `max` entries into `out` (see `year_collect`; from 25
/// matches, 15 in calendar year 2001 since the table's year is always above 360) whose person passes the filters
/// (-1 or 0xff for none): his nationality is `nation`; his club's nation is `club_nation`; his nationality is not
/// `excluded_nation`; his club's nation and his nationality are both `home_nation`; his age is at most `max_age`; his
/// club's division is `division`. A filter on the club skips a person without a club or without that pointer.
/// Returns how many were collected.
/// Original: `YEAR_RATINGS::collect_best_players` (0x931ae0, award/year_ratings.cpp, thiscall, ret 0x24).
pub(crate) unsafe extern "thiscall" fn year_ratings_collect_best_players(this: usize, out: *mut usize, max: i16, nation: i32, club_nation: i32,
    excluded_nation: i32, home_nation: i32, division: i32, mode: u8, max_age: u8) -> i16 {
    let table = YearRatings(this);
    let min_matches = if read_u16(G_CURRENT_DATE_YEAR) == 2001 && table.year() > 0x168 { 15 } else { 25 };
    table.sort_by_mode(mode);
    let club_nation_id = |e: YearEntry| {
        let club = e.person().club_job();
        if club.is_null() { None } else { id_at(club.nation().addr()) }
    };
    year_collect(table, out, max, min_matches, |e| {
        if nation != -1 && e.nation_id() != nation {
            return false;
        }
        if club_nation != -1 && club_nation_id(e) != Some(club_nation) {
            return false;
        }
        if excluded_nation != -1 && e.nation_id() == excluded_nation {
            return false;
        }
        if home_nation != -1 && (club_nation_id(e) != Some(home_nation) || e.nation_id() != home_nation) {
            return false;
        }
        if max_age != 0xff && e.person().age() as i8 > max_age as i8 {
            covered!("sx05_4 too old");
            return false;
        }
        if division != -1 {
            let club = e.person().club_job();
            if club.is_null() || id_at(club.division().addr()) != Some(division) {
                return false;
            }
        }
        true
    })
}

/// Sorts the year ratings by `mode` (0 to 5) and collects up to `max` entries into `out` (see `year_collect`, from 25
/// matches) whose person: with a `continent` (-1 none), is of a nation of that continent (a person without a
/// nationality passes), and for South America also plays for a club of a South American nation; with a
/// `position_mask`, has a player sheet only if his natural positions (`exe::staff_get_natural_position_mask`) share a
/// bit with it; is not in the `excluded` list (by staff id; its iterator is rewound first). Returns how many were
/// collected.
/// Original: `YEAR_RATINGS::collect_candidates` (0x931d50, award/year_ratings.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn year_ratings_collect_candidates(this: usize, out: *mut usize, max: i16, continent: i32, position_mask: u16,
    mode: u8, excluded: usize) -> i16 {
    let table = YearRatings(this);
    table.sort_by_mode(mode);
    year_collect(table, out, max, 25, |e| {
        let person = e.person();
        if continent != -1 {
            if continent == read_i32(KEY_CONTINENT_SOUTH_AMERICA) {
                let club = person.club_job();
                let club_nation = if club.is_null() { Nation(0) } else { club.nation() };
                if club_nation.is_null() || id_at(club_nation.continent()) != Some(continent) {
                    return false;
                }
            }
            let nation_id = e.nation_id();
            if nation_id > -1 {
                let nation = Nation(read_usize(G_NATION_TABLE) + nation_id as usize * nation::SIZE);
                if nation.continent() != read_usize(G_CONTINENT_TABLE) + continent as usize * CONTINENT_SIZE {
                    return false;
                }
            }
        }
        if position_mask != 0 && !person.player().is_null() && exe::staff_get_natural_position_mask(person) as u16 & position_mask == 0 {
            covered!("sx05_4 wrong position");
            return false;
        }
        if excluded != 0 {
            write_u32(excluded + 0xa, read_u32(excluded + 2));
            loop {
                let node = exe::llist_get_next(excluded);
                if node == 0 {
                    break;
                }
                if read_i32(read_usize(node)) == read_i32(e.0) {
                    covered!("sx05_4 excluded candidate");
                    return false;
                }
            }
        }
        true
    })
}

/// Comparer for `year_ratings_reset_if_year`: only the original runs (it frees the table); then the header is checked
/// against the plan (emptied when the year matched, untouched otherwise).
pub(crate) unsafe extern "thiscall" fn compare_year_ratings_reset_if_year(this: usize, year: *const i16) {
    let before = [read_usize(this), read_usize(this + 4), read_usize(this + 8)];
    let reset = *year as i32 == read_u16(this + 0xc) as i32;
    original_fn::<unsafe extern "thiscall" fn(usize, *const i16)>(IDX_YEAR_RATINGS_RESET_IF_YEAR)(this, year);
    let after = [read_usize(this), read_usize(this + 4), read_usize(this + 8)];
    let plan = if reset { [0, 0, 0] } else { before };
    SCOREBOARD.record(after == plan, || format!("year_ratings_reset_if_year(year {}) original={:x?} plan={:x?}", *year, after, plan));
}

/// Comparer for `year_ratings_collect_best_players`: the sorted table and the output side by side, and the count.
pub(crate) unsafe extern "thiscall" fn compare_year_ratings_collect_best_players(this: usize, out: *mut usize, max: i16, nation: i32, club_nation: i32,
    excluded_nation: i32, home_nation: i32, division: i32, mode: u8, max_age: u8) -> i16 {
    type F = unsafe extern "thiscall" fn(usize, *mut usize, i16, i32, i32, i32, i32, i32, u8, u8) -> i16;
    let o = original_fn::<F>(IDX_YEAR_RATINGS_COLLECT_BEST_PLAYERS);
    if YearRatings(this).entries() == 0 || out.is_null() {
        return o(this, out, max, nation, club_nation, excluded_nation, home_nation, division, mode, max_age);
    }
    let regions = year_collect_regions(this, out, max, 0);
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&regions,
        || o(this, out, max, nation, club_nation, excluded_nation, home_nation, division, mode, max_age),
        || year_ratings_collect_best_players(this, out, max, nation, club_nation, excluded_nation, home_nation, division, mode, max_age));
    SCOREBOARD.record(equal && r == v, || format!("year_ratings_collect_best_players(mode {}, max {}) original={} rust={} {:?}", mode, max, r, v, first_differences(&a, &b)));
    r
}

/// Comparer for `year_ratings_collect_candidates`: the sorted table, the output and the list's iterator side by side,
/// and the count.
pub(crate) unsafe extern "thiscall" fn compare_year_ratings_collect_candidates(this: usize, out: *mut usize, max: i16, continent: i32, position_mask: u16,
    mode: u8, excluded: usize) -> i16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, i16, i32, u16, u8, usize) -> i16>(IDX_YEAR_RATINGS_COLLECT_CANDIDATES);
    if YearRatings(this).entries() == 0 || out.is_null() {
        return o(this, out, max, continent, position_mask, mode, excluded);
    }
    let regions = year_collect_regions(this, out, max, excluded);
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&regions,
        || o(this, out, max, continent, position_mask, mode, excluded),
        || year_ratings_collect_candidates(this, out, max, continent, position_mask, mode, excluded));
    SCOREBOARD.record(equal && r == v, || format!("year_ratings_collect_candidates(mode {}, max {}) original={} rust={} {:?}", mode, max, r, v, first_differences(&a, &b)));
    r
}

/// The qsort order of two year ratings entries by their scores: an entry with fewer than 3 matches goes after one
/// with 3 or more (two such are equal); otherwise the higher score first (-1), the lower (or an unordered pair) after
/// (1), equal 0.
pub(crate) unsafe fn year_entries_order(a: YearEntry, b: YearEntry, score: unsafe fn(YearEntry) -> f64) -> i32 {
    if a.matches() < 3 {
        return (b.matches() >= 3) as i32;
    }
    if b.matches() < 3 {
        return -1;
    }
    covered!("sx05_2 year order by score");
    let (sa, sb) = (score(a), score(b));
    if sa > sb {
        -1
    } else if !(sa >= sb) {
        1
    } else {
        0
    }
}

/// The average-rating score of sort modes 0 to 2: (goals + 2 × the mode's rating sum + the other two sums) / matches,
/// plus 2 × matches × 0.02; × 1.2 for a player rated above 14 as an attacker, else × 1.05 above 14 as an attacking
/// midfielder.
pub(crate) unsafe fn year_rating_score(e: YearEntry, doubled: usize) -> f64 {
    let n = e.matches() as f64;
    let total = e.goals() + e.rating_sum(0) + e.rating_sum(1) + e.rating_sum(2) + e.rating_sum(doubled);
    let mut score = total as f64 / n + (n * 0.02 + n * 0.02);
    let sheet = e.person().player();
    if !sheet.is_null() {
        if sheet.attacker() > 14 {
            covered!("sx05_2 attacker bonus");
            score *= 1.2f32 as f64;
        } else if sheet.attacking_midfielder() > 14 {
            score *= 1.05f32 as f64;
        }
    }
    score
}

/// A comparer of the year ratings sort modes: both orders side by side, the original's returned.
pub(crate) unsafe fn compare_year_order(index: usize, name: &str, a: usize, b: usize, rust: unsafe extern "C" fn(usize, usize) -> i32) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(index)(a, b);
    let v = rust(a, b);
    SCOREBOARD.record(r == v, || format!("{}(person {}, person {}) original={} rust={}", name, read_i32(a), read_i32(b), r, v));
    r
}

/// The goals score of sort modes 3 and 5: (5 × goals + 3 × assists + the +0xc rating sum) / matches.
pub(crate) unsafe fn year_goals_score(e: YearEntry) -> f64 {
    (5 * e.goals() + 3 * e.assists() + e.rating_sum(2)) as f64 / e.matches() as f64
}

/// Fills `out` with up to `max` year ratings entries in sort order: passes from `min_matches` matches down in steps of
/// 5 (while above 0), each taking, in order, the entries with at least that many matches that are not already among
/// the `max` slots of `out` and that `accepts`, until `max` are taken. Returns how many.
/// Defect of the original: the "already taken" check reads all `max` slots of `out`, not only the filled ones, so a
/// stale pointer left by the caller in a later slot keeps that entry out.
pub(crate) unsafe fn year_collect(table: YearRatings, out: *mut usize, max: i16, mut min_matches: i16, mut accepts: impl FnMut(YearEntry) -> bool) -> i16 {
    let max_slots = max as i32;
    let mut found = 0i32;
    while min_matches > 0 {
        let mut k = 0;
        while k < table.count() && found < max_slots {
            let e = table.entry(k as usize);
            k += 1;
            if (e.matches() as i16) < min_matches {
                continue;
            }
            if (0..max_slots.max(0) as usize).any(|s| *out.add(s) == e.0) {
                continue;
            }
            if accepts(e) {
                *out.add(found as usize) = e.0;
                found += 1;
            }
        }
        if found == max_slots {
            break;
        }
        covered!("sx05_4 fewer matches pass");
        min_matches -= 5;
    }
    found as i16
}

/// The regions a collector changes: the sorted entries, the `max` output slots and the excluded list's iterator.
pub(crate) unsafe fn year_collect_regions(this: usize, out: *mut usize, max: i16, excluded: usize) -> Vec<(usize, usize)> {
    let table = YearRatings(this);
    let mut regions = vec![(table.entries(), table.count().max(0) as usize * year_ratings_consts::ENTRY_SIZE), (out as usize, max.max(0) as usize * 4)];
    if excluded != 0 {
        regions.push((excluded + 0xa, 4));
    }
    regions
}

/// The size of a continent record (`G_CONTINENT_TABLE`).
pub(crate) const CONTINENT_SIZE: usize = 0xc6;

/// The year ratings table (`YEAR_RATINGS`: +0 the entries, +4 count, +8 capacity, +0xc the year).
#[derive(Clone, Copy)]
pub(crate) struct YearRatings(pub(crate) usize);

impl YearRatings {
    pub(crate) unsafe fn entries(self) -> usize {
        read_usize(self.0)
    }
    pub(crate) unsafe fn count(self) -> i32 {
        read_i32(self.0 + 4)
    }
    pub(crate) unsafe fn year(self) -> u16 {
        read_u16(self.0 + 0xc)
    }
    pub(crate) unsafe fn entry(self, k: usize) -> YearEntry {
        YearEntry(self.entries() + k * year_ratings_consts::ENTRY_SIZE)
    }
    /// Sorts the entries in place by sort mode 0 to 5 (the CRT qsort with the mode's order function, at its original
    /// address); any other mode leaves them as they are.
    pub(crate) unsafe fn sort_by_mode(self, mode: u8) {
        const ORDERS: [usize; 6] = [0x931f90, 0x9320e0, 0x932230, 0x9323b0, 0x932500, 0x932590];
        if let Some(&order) = ORDERS.get(mode as usize) {
            exe::crt_qsort(self.entries(), self.count() as u32, year_ratings_consts::ENTRY_SIZE as u32, order);
        }
    }
}

/// A year ratings entry (`year_ratings_consts::ENTRY_SIZE` bytes: +0 staff id, +4 nation id, +8/+0xa/+0xc the three
/// weighted rating sums, +0xe matches, +0xf goals, +0x11 assists, +0x13 cards), as the sort modes read it.
#[derive(Clone, Copy)]
pub(crate) struct YearEntry(pub(crate) usize);

impl YearEntry {
    pub(crate) unsafe fn person(self) -> Staff {
        staff_at(read_i32(self.0) as isize)
    }
    pub(crate) unsafe fn nation_id(self) -> i32 {
        read_i32(self.0 + 4)
    }
    /// The weighted rating sum `k` (0, 1 or 2: the +8, +0xa or +0xc word).
    pub(crate) unsafe fn rating_sum(self, k: usize) -> i32 {
        read_i16(self.0 + 8 + 2 * k) as i32
    }
    pub(crate) unsafe fn matches(self) -> i8 {
        read_i8(self.0 + 0xe)
    }
    pub(crate) unsafe fn goals(self) -> i32 {
        read_i16(self.0 + 0xf) as i32
    }
    pub(crate) unsafe fn assists(self) -> i32 {
        read_i16(self.0 + 0x11) as i32
    }
    /// Yellow cards + 2 × red cards.
    pub(crate) unsafe fn cards(self) -> i32 {
        read_i16(self.0 + 0x13) as i32
    }
}
