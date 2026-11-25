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

/// A comparer of the year ratings sort modes: both orders side by side, the original's returned.
pub(crate) unsafe fn compare_year_order(index: usize, name: &str, a: usize, b: usize, rust: unsafe extern "C" fn(usize, usize) -> i32) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(index)(a, b);
    let v = rust(a, b);
    SCOREBOARD.record(r == v, || format!("{}(person {}, person {}) original={} rust={}", name, read_i32(a), read_i32(b), r, v));
    r
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

/// The goals score of sort modes 3 and 5: (5 × goals + 3 × assists + the +0xc rating sum) / matches.
pub(crate) unsafe fn year_goals_score(e: YearEntry) -> f64 {
    (5 * e.goals() + 3 * e.assists() + e.rating_sum(2)) as f64 / e.matches() as f64
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
    original_fn::<unsafe extern "thiscall" fn(usize, *const i16)>(IDX_YEAR_RATINGS_RESET_IF_YEAR)(this, year);
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

/// The team award's selection (`TEAM_AWARD`): the position mask word of each slot at +0x26 (2 bytes a slot), the
/// 20 slots of 0x19 bytes at +0x4e (+0 staff id, +4/+8/+0xc the three name ids, +0x10 the club id, +0x14 the squad
/// number, +0x15 the rating, f32), the filled count at +0x252, the substitutes count at +0x253 (the slots are 11 +
/// it) and the formation mode at +0x25c.
#[derive(Clone, Copy)]
pub(crate) struct TeamAward(pub(crate) usize);

pub(crate) const TEAM_AWARD_SLOT_SIZE: usize = 0x19;

impl TeamAward {
    pub(crate) unsafe fn slot_count(self) -> i8 {
        (read_u8(self.0 + 0x253) as i8).wrapping_add(11)
    }
    pub(crate) unsafe fn filled(self) -> i8 {
        read_i8(self.0 + 0x252)
    }
    pub(crate) fn slot(self, k: usize) -> usize {
        self.0 + 0x4e + k * TEAM_AWARD_SLOT_SIZE
    }
    pub(crate) unsafe fn slot_mask(self, k: usize) -> u16 {
        read_u16(self.0 + 0x26 + 2 * k)
    }
    pub(crate) unsafe fn slot_id(self, k: usize) -> i32 {
        read_i32(self.slot(k))
    }
    /// The slot accepts a player whose (widened) positions are `mask`: all its own bits are in it.
    pub(crate) unsafe fn fits(self, k: usize, mask: u16) -> bool {
        self.slot_mask(k) & mask == self.slot_mask(k)
    }
    /// The selection is complete: the filled count equals the slot count.
    pub(crate) unsafe fn complete(self) -> bool {
        self.filled() == self.slot_count()
    }
}

/// A player's position bits widened for the team award slots: 2 also gives 4, 8 or 0x20 give 0x10, and 0x40 with
/// 0x80 or with 0x800 gives 0x200.
pub(crate) fn widen_award_positions(mut m: u16) -> u16 {
    if m & 2 != 0 {
        m |= 4;
    }
    if m & 8 != 0 { // sx05_6: 8 brings 0x10
        m |= 0x10;
    }
    if m & 0x20 != 0 {
        m |= 0x10;
    }
    if m & 0x40 != 0 && (m & 0x80 != 0 || m & 0x800 != 0) {
        m |= 0x200;
    }
    m
}

/// The name id of a staff name record (+0x33), -1 without one.
unsafe fn award_name_id(record: usize) -> i32 {
    if record == 0 { -1 } else { read_i32(record + 0x33) }
}

/// Writes `person` into team award slot `k`: his id, rating, club id, squad number and three name ids.
unsafe fn team_award_place(t: TeamAward, k: usize, person: Staff, club: usize, rating: f32, number: u8) {
    let s = t.slot(k);
    write_i32(s, read_i32(person.addr()));
    write_f32(s + 0x15, rating);
    write_i32(s + 0x10, read_i32(club));
    write_u8(s + 0x14, number);
    write_i32(s + 4, award_name_id(person.first_name()));
    write_i32(s + 8, award_name_id(person.second_name()));
    write_i32(s + 0xc, award_name_id(person.common_name()));
}

/// Offers a player to the team award. Without a person, a player sheet or a club, nothing. Already chosen, his rating
/// rises to `rating` when higher. Otherwise his positions (`position_mask`, or his sheet's `exe::formation_get_position_mask`
/// when 0) are widened (`widen_award_positions`) and he takes the first empty slot that fits; with none, the first
/// filled slot that fits him whose occupant fits an empty slot: the occupant moves there and he takes the slot. His
/// squad number (absolute) is kept when his club uses squad numbers. Returns 1 when the selection is complete.
/// Original: `TEAM_AWARD::add_candidate` (0x8afa40, team_award.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn team_award_add_candidate(this: usize, person: usize, club: usize, rating: f32, position_mask: u16) -> u32 {
    let t = TeamAward(this);
    let n = t.slot_count();
    let person = Staff(person);
    if person.is_null() || person.player().is_null() || club == 0 {
        return t.complete() as u32;
    }
    covered!("sx05_6 candidate");
    if let Some(k) = (0..n.max(0) as usize).find(|&k| t.slot_id(k) == read_i32(person.addr())) {
        covered!("sx05_6 chosen again");
        if rating > read_f32(t.slot(k) + 0x15) {
            write_f32(t.slot(k) + 0x15, rating);
        }
        return t.complete() as u32;
    }
    let mask = widen_award_positions(if position_mask != 0 { position_mask } else { exe::formation_get_position_mask(person.player()) });
    let mut number = 0u8;
    if exe::squad_manager_club_uses_squad_numbers(read_usize(G_SQUAD_MANAGER), club, 0) != 0 {
        number = (exe::squad_manager_get_squad_number(read_usize(G_SQUAD_MANAGER), person, club) as i32).unsigned_abs() as u8;
    }
    let slots = 0..n.max(0) as usize;
    let place = |k: usize| {
        team_award_place(t, k, person, club, rating, number);
        write_u8(this + 0x252, read_u8(this + 0x252).wrapping_add(1));
        t.complete() as u32
    };
    if let Some(k) = slots.clone().find(|&k| t.slot_id(k) == -1 && t.fits(k, mask)) {
        return place(k);
    }
    for k in slots.clone() {
        if t.slot_id(k) == -1 || !t.fits(k, mask) {
            continue;
        }
        let occupant = widen_award_positions(exe::formation_get_position_mask(staff_at(t.slot_id(k) as isize).player()));
        if let Some(j) = slots.clone().find(|&j| t.slot_id(j) == -1 && t.fits(j, occupant)) {
            covered!("sx05_6 occupant moved");
            core::ptr::copy(t.slot(k) as *const u8, t.slot(j) as *mut u8, TEAM_AWARD_SLOT_SIZE);
            return place(k);
        }
    }
    t.complete() as u32
}

/// Empties the twenty slots (id, name ids and club -1, rating 0; the squad number is left) and the filled count.
/// Original: `TEAM_AWARD::clear_selection` (0x8afd90, team_award.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn team_award_clear_selection(this: usize) {
    let t = TeamAward(this);
    for k in 0..20 {
        for f in [0, 4, 8, 0xc, 0x10] {
            write_i32(t.slot(k) + f, -1);
        }
        write_u32(t.slot(k) + 0x15, 0);
    }
    write_u8(this + 0x252, 0);
}

/// The position masks of the starting eleven by the formation mode (+0x25c), then of the nine substitutes.
const TEAM_AWARD_ELEVENS: [[u16; 11]; 4] = [
    [1, 0x804, 0x84, 0x204, 0x204, 0x210, 0x810, 0x210, 0x240, 0x240, 0x90],
    [1, 0x804, 0x84, 0x204, 0x204, 0x90, 0x210, 0x810, 0xc0, 0x240, 0x840],
    [1, 0x808, 0x88, 0x204, 0x204, 2, 0x210, 0x210, 0x240, 0x240, 0x210],
    [1, 0x804, 0x84, 2, 0x204, 0x208, 0x90, 0x810, 0x220, 0x240, 0x840],
];
const TEAM_AWARD_SUBSTITUTES: [u16; 9] = [1, 0x40, 4, 0x10, 0x40, 4, 0x10, 0x40, 4];

/// Sets the position mask of each slot: the eleven by the formation mode (0, 1 and 2 their own, any other the
/// fourth), the nine substitutes always goalkeeper then three times (0x40, 4, 0x10).
/// Original: `TEAM_AWARD::init_slot_positions` (0x8afdc0, team_award.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn team_award_init_slot_positions(this: usize) {
    let eleven = &TEAM_AWARD_ELEVENS[(read_u8(this + 0x25c) as usize).min(3)];
    for (k, &m) in eleven.iter().chain(TEAM_AWARD_SUBSTITUTES.iter()).enumerate() {
        write_u16(this + 0x26 + 2 * k, m);
    }
}

/// The team award's slot masks, slots and filled count (+0x26..+0x253).
const TEAM_AWARD_SELECTION: (usize, usize) = (0x26, 0x253 - 0x26);

pub(crate) unsafe extern "thiscall" fn compare_team_award_add_candidate(this: usize, person: usize, club: usize, rating: f32, position_mask: u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, f32, u16) -> u32>(IDX_TEAM_AWARD_ADD_CANDIDATE);
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&[(this + TEAM_AWARD_SELECTION.0, TEAM_AWARD_SELECTION.1)],
        || o(this, person, club, rating, position_mask) & 0xff, || team_award_add_candidate(this, person, club, rating, position_mask));
    SCOREBOARD.record(equal && r == v, || format!("team_award_add_candidate(person {:#x}) original={} rust={} {:?}", person, r, v, first_differences(&a, &b)));
    r
}

pub(crate) unsafe extern "thiscall" fn compare_team_award_clear_selection(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TEAM_AWARD_CLEAR_SELECTION);
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&[(this + TEAM_AWARD_SELECTION.0, TEAM_AWARD_SELECTION.1)],
        || o(this), || team_award_clear_selection(this));
    SCOREBOARD.record(equal, || format!("team_award_clear_selection({:#x}) {:?}", this, first_differences(&a, &b)));
}

pub(crate) unsafe extern "thiscall" fn compare_team_award_init_slot_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TEAM_AWARD_INIT_SLOT_POSITIONS);
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&[(this + 0x26, 2 * 20)], || o(this), || team_award_init_slot_positions(this));
    SCOREBOARD.record(equal, || format!("team_award_init_slot_positions(mode {}) {:?}", read_u8(this + 0x25c), first_differences(&a, &b)));
}
