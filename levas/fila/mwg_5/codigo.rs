// ---------- batch %%LEVA%%: the year ratings and the month ratings reset (award/year_ratings.cpp, award/month_ratings.cpp) ----------

/// Constants of the year ratings (`YEAR_RATINGS`: +0 pointer to the 21-byte entries, +4 count, +8 capacity).
pub(crate) mod year_ratings_consts {
    /// The entry size: +0 staff id, +4 nation id (−1 none), +8/+0xa/+0xc three weighted rating sums (i16), +0xe
    /// matches (u8), +0xf goals, +0x11 assists, +0x13 cards (yellow + 2 × red) (u16).
    pub(crate) const ENTRY_SIZE: usize = 21;
    /// How many entries the table grows by when full.
    pub(crate) const GROWTH: i32 = 400;
    /// "E:\dev\CM3\cm3 00-01\cm3\code\award\year_ratings.cpp" (the error box).
    pub(crate) const YEAR_RATINGS_CPP: usize = 0xad6a38;
    /// The competition object's weight word (+0x69) for the year ratings, and its type byte (+0x42: 2 and 4 weigh
    /// differently).
    pub(crate) const COMP_WEIGHT: usize = 0x69;
    pub(crate) const COMP_TYPE: usize = 0x42;
}

/// Adds the first eleven of both sides of a match to the year ratings, for the players the filter accepts.
///
/// `YEAR_RATINGS::record_match_ratings(fixture, record, filter)`. Not for a friendly (`fixture::CATEGORY` 1). For each
/// side (the home club `fixture::HOME_CLUB` with the record's first block, the away club with the block
/// `match_record::SIDE_SIZE` on), the first 11 of the 20 rows: an empty one (staff id −1 or position
/// `row_slot::EMPTY`) and a virtual player (staff id at or above `G_STAFF_COUNT`) are skipped; the rest go to
/// `add_player_rating` when `filter(person, fixture)` (cdecl) accepts them.
/// Original: `YEAR_RATINGS::record_match_ratings` (0x9315d0, award/year_ratings.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn year_record_match_ratings(this: usize, fixture: usize, record: usize, filter: usize) {
    if read_u8(fixture + fixture::CATEGORY) == 1 {
        return;
    }
    let accepts: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(filter);
    let add: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x931690usize);
    for side in 0..2usize {
        let club = read_usize(fixture + if side == 0 { fixture::HOME_CLUB } else { fixture::AWAY_CLUB });
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
            if accepts(person, fixture) != 0 {
                if side == 1 {
                    covered!("mwg_5 away side rating");
                }
                add(this, row, fixture, club);
            }
        }
    }
}

/// Comparer for `year_record_match_ratings`.
///
/// Side by side over the table's header and its entries plus room for 22 new ones; when the table could grow (less
/// room, or none yet), only the original runs (each added rating has its own comparer).
pub(crate) unsafe extern "thiscall" fn compare_year_record_match_ratings(this: usize, fixture: usize, record: usize, filter: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I0%%);
    let (tab, n, cap) = (read_usize(this), read_i32(this + 4), read_i32(this + 8));
    if tab == 0 || n < 0 || cap.wrapping_sub(n) < 22 {
        return o(this, fixture, record, filter);
    }
    let regions = [(this, 12), (tab, (n as usize + 22) * year_ratings_consts::ENTRY_SIZE)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, fixture, record, filter), || year_record_match_ratings(this, fixture, record, filter));
    SCOREBOARD.record(equal, || format!("year_record_match_ratings(fixture {:x}) original={:x?} rust={:x?}", fixture, mo, mr));
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
unsafe fn year_rating_sums(fixture: usize, club: Club, comp_obj: usize, grade: i32) -> (i8, i8, i8) {
    use year_ratings_consts as yr;
    let mut weight = 0i8;
    let competition = read_usize(fixture + fixture::COMPETITION);
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
        return wide(grade * (scaled / 20) as i8 as i32);
    }
    let v = grade * weight as i32;
    match read_i8(comp_obj + yr::COMP_TYPE) {
        2 => ((v / 20) as i8, (v / 20) as i8, (v / 40) as i8),
        4 => ((v / 20) as i8, (v / 20) as i8, (v / 20) as i8),
        _ => wide(v),
    }
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
    use year_ratings_consts as yr;
    let comp_id = read_i32(fixture + fixture::COMP_ID);
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
    let grade = read_u8(row + match_record_row::MATCH_GRADE) as i32;
    let (a, b, c) = year_rating_sums(fixture, Club(club), comp_obj, grade);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I1%%);
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

/// Empties the month ratings for a new month.
///
/// `MONTH_RATINGS::reset(month, year)`: the entry table is freed (`exe::crt_free`) and cleared, the count and the
/// capacity become 0, and `month_ratings_fields::MONTH` / `YEAR` the new month and year.
/// Original: `MONTH_RATINGS::reset` (0x767f60, award/month_ratings.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn month_ratings_reset(this: usize, month: u8, year: i16) {
    covered!("mwg_5 month reset");
    let table = read_usize(this); // mwg5: the old month's table
    if table != 0 {
        exe::crt_free(table);
        write_usize(this, 0);
    }
    write_u32(this + 8, 0);
    write_u32(this + 4, 0);
    write_u8(this + month_ratings_fields::MONTH, month);
    write_i16(this + month_ratings_fields::YEAR, year); // mwg5: the new month's year
}

/// Comparer for `month_ratings_reset`.
///
/// The table is freed: it cannot run twice. Only the original runs, and what it guarantees is checked: an empty table
/// with the month and year given.
pub(crate) unsafe extern "thiscall" fn compare_month_ratings_reset(this: usize, month: u8, year: i16) {
    original_fn::<unsafe extern "thiscall" fn(usize, u8, i16)>(%%I2%%)(this, month, year);
    let ok = read_usize(this) == 0 && read_u32(this + 4) == 0 && read_u32(this + 8) == 0
        && read_u8(this + month_ratings_fields::MONTH) == month && read_i16(this + month_ratings_fields::YEAR) == year;
    SCOREBOARD.record(ok, || format!("month_ratings_reset({}, {}): table {:x} count {} capacity {} month {} year {}",
        month, year, read_usize(this), read_u32(this + 4), read_u32(this + 8), read_u8(this + month_ratings_fields::MONTH), read_i16(this + month_ratings_fields::YEAR)));
}
