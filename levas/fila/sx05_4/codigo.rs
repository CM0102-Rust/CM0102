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

/// The id of the record a pointer field points to (its first dword), or None when the pointer is null.
unsafe fn id_at(record: usize) -> Option<i32> {
    if record == 0 { None } else { Some(read_i32(record)) }
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
        if position_mask != 0 && !person.player().is_null() && exe::staff_get_natural_position_mask(person.addr()) as u16 & position_mask == 0 {
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
    original_fn::<unsafe extern "thiscall" fn(usize, *const i16)>(%%I0%%)(this, year);
    let after = [read_usize(this), read_usize(this + 4), read_usize(this + 8)];
    let plan = if reset { [0, 0, 0] } else { before };
    SCOREBOARD.record(after == plan, || format!("year_ratings_reset_if_year(year {}) original={:x?} plan={:x?}", *year, after, plan));
}

/// The regions a collector changes: the sorted entries, the `max` output slots and the excluded list's iterator.
unsafe fn year_collect_regions(this: usize, out: *mut usize, max: i16, excluded: usize) -> Vec<(usize, usize)> {
    let table = YearRatings(this);
    let mut regions = vec![(table.entries(), table.count().max(0) as usize * year_ratings_consts::ENTRY_SIZE), (out as usize, max.max(0) as usize * 4)];
    if excluded != 0 {
        regions.push((excluded + 0xa, 4));
    }
    regions
}

/// Comparer for `year_ratings_collect_best_players`: the sorted table and the output side by side, and the count.
pub(crate) unsafe extern "thiscall" fn compare_year_ratings_collect_best_players(this: usize, out: *mut usize, max: i16, nation: i32, club_nation: i32,
    excluded_nation: i32, home_nation: i32, division: i32, mode: u8, max_age: u8) -> i16 {
    type F = unsafe extern "thiscall" fn(usize, *mut usize, i16, i32, i32, i32, i32, i32, u8, u8) -> i16;
    let o = original_fn::<F>(%%I1%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, i16, i32, u16, u8, usize) -> i16>(%%I2%%);
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
