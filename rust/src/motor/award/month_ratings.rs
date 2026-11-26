use super::*;

/// Staff record of id (G_STAFF_TABLE, 0x6e bytes).
pub(crate) unsafe fn month_ratings_staff_by_id(id: i32) -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE))
}

/// Records every player's rating from a match into the month ratings.
///
/// `MONTH_RATINGS::record_match_ratings(game, fixture)`, called by 0x7683a0 for every match of the leagues that have a
/// monthly award. Both sides of the match (+0x190 and +0x190 + 0x625), 20 slots of `match_record_row::SIZE` bytes each:
/// except the slot whose `.position()` is `row_slot::EMPTY` and, from the 12th slot on, the substitute who did not come
/// on (`.minute_on()` = 0xff), the person (id at +0) below the person count `G_STAFF_COUNT` goes to the month ratings
/// (`exe::month_ratings_add_player_rating`).
/// Original: `MONTH_RATINGS::record_match_ratings` (0x767ee0, month_ratings.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn record_match_ratings(this: usize, game: usize, fixture: usize) {
    let _ = game;
    let mut side = fixture.wrapping_add(0x190);
    for l in 0..2 { // cpw5: os dois lados
        for k in 0..0x14usize {
            let e = MatchRecordRow(side + k * match_record_row::SIZE);
            if e.position() == row_slot::EMPTY {
                continue;
            }
            if k >= 0xb && (e.minute_on() as u8) == 0xff {
                continue;
            }
            let p = month_ratings_staff_by_id(read_i32(e.addr()));
            if p != 0 && read_i32(p) >= read_i32(G_STAFF_COUNT) {
                continue;
            }
            if l == 1 {
                covered!("cpw_5 away side ratings");
            }
            exe::month_ratings_add_player_rating(this, e);
        }
        side += 0x625;
    }
}

/// Adds a slot's match rating (+0x35) to word +8 of a 12-byte record and counts one more game at +0xa.
pub(crate) unsafe fn add_match_rating(r: usize, e: usize) {
    let e = MatchRecordRow(e);
    let s = (r + 8) as *mut u16;
    core::ptr::write_unaligned(s, core::ptr::read_unaligned(s).wrapping_add(e.match_grade() as u16));
    write_u8(r + month_rating_entry::MATCHES, read_u8(r + month_rating_entry::MATCHES).wrapping_add(1));
}

/// Finds or appends the person's record in the month ratings table, adds the rating and counts one match.
///
/// `MONTH_RATINGS::add_player_rating(e)`. The table at [this] (count at +4, capacity at +8) has 12-byte records: +0
/// person, +4 club's league (or −1), +8 sum of the ratings, `month_rating_entry::MATCHES` matches,
/// `month_rating_entry::AGE` age. It finds the slot's person (+0) or appends it (growing by 300 with malloc
/// `exe::crt_malloc` or realloc `exe::crt_realloc` when full; the league from the person's `.club_job()` and that
/// club's `.division()`; the age from the person's `.age()`) and adds the rating (`add_match_rating`).
/// Original: `MONTH_RATINGS::add_player_rating` (0x767fa0, month_ratings.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn add_player_rating(this: usize, e: usize) {
    let id = read_i32(e);
    let n = read_i32(this + 4);
    let mut table = read_usize(this);
    for k in 0..n.max(0) as usize {
        let r = table + k * 12;
        if read_i32(r) == id {
            return add_match_rating(r, e);
        }
    }
    let cap = read_i32(this + 8);
    if n == cap {
        let new = cap.wrapping_add(300);
        write_u32(this + 8, new as u32);
        let bytes = (new as u32).wrapping_mul(12);
        table = if table == 0 {
            exe::crt_malloc(bytes)
        } else {
            exe::crt_realloc(table, bytes)
        };
        if table == 0 {
            // simplification: out of memory (the original’s ”Error” box): restores the capacity and lets the original redo it
            write_u32(this + 8, cap as u32);
            return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_PLAYER_RATING)(this, e);
        }
        write_u32(this, table as u32);
    }
    let p = Staff(month_ratings_staff_by_id(id));
    let club = p.club_job();
    let mut league = -1i32;
    if !club.is_null() {
        let l = club.division().addr();
        if l != 0 {
            league = read_i32(l);
        }
    }
    let r = table + (n as usize) * 12;
    write_u32(r, id as u32);
    write_u32(r + 4, league as u32);
    write_u16(r + 8, 0);
    write_u8(r + month_rating_entry::AGE, p.age() as u8);
    covered!("cpw_5 new player in the month");
    write_u8(r + month_rating_entry::MATCHES, 0); // cpw5: starts with no matches
    write_u32(this + 4, n.wrapping_add(1) as u32);
    add_match_rating(r, e);
}

/// Sorts the month ratings by average and returns the best player of a league, with an optional age filter.
///
/// `MONTH_RATINGS::find_best_player(league, age)`, used by the monthly award (`award_best_rated_player`). It sorts the
/// table by average (qsort `exe::crt_qsort` with 0x768200: those with fewer than 3 matches go to the end) and returns
/// the first record of the league (−1: any) with at least 3 `month_rating_entry::MATCHES` (then 2, 1 and 0); with an
/// age (not 0xff), those whose `month_rating_entry::AGE` is above it are only counted and the youngest only qualifies
/// after 3 have been counted; with a league, the current league of the person's club (`staff::CLUB_JOB`, `.division()`)
/// must also be the requested one. None: 0.
/// Original: `MONTH_RATINGS::find_best_player` (0x768130, month_ratings.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn find_best_player(this: usize, league: i32, age: u32) -> usize {
    covered!("cpw_5 sorted the ratings");
    exe::crt_qsort(read_usize(this), read_usize(this + 4) as u32, 0xc, 0x768200); // cpw5: sorts by the average
    let table = read_usize(this);
    let n = read_i32(this + 4);
    let age = age as u8;
    let mut contados = 0i32;
    let mut minimum = 3i8;
    loop {
        for k in 0..n.max(0) as usize {
            let r = table + k * 12;
            if league != -1 && read_i32(r + 4) != league {
                continue;
            }
            if (read_u8(r + month_rating_entry::MATCHES) as i8) < minimum {
                continue;
            }
            if age != 0xff {
                if read_u8(r + month_rating_entry::AGE) as i8 > age as i8 {
                    contados += 1;
                    continue;
                }
                if contados < 3 {
                    continue;
                }
            }
            if league == -1 {
                return r;
            }
            let c = Club(read_usize(month_ratings_staff_by_id(read_i32(r)) + staff::CLUB_JOB));
            if !c.is_null() {
                let l = c.division().addr();
                if l != 0 && read_i32(l) == league {
                    return r;
                }
            }
        }
        minimum -= 1;
        if minimum < 0 {
            return 0;
        }
    }
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
    original_fn::<unsafe extern "thiscall" fn(usize, u8, i16)>(IDX_MONTH_RATINGS_RESET)(this, month, year);
    let ok = read_usize(this) == 0 && read_u32(this + 4) == 0 && read_u32(this + 8) == 0
        && read_u8(this + month_ratings_fields::MONTH) == month && read_i16(this + month_ratings_fields::YEAR) == year;
    SCOREBOARD.record(ok, || format!("month_ratings_reset({}, {}): table {:x} count {} capacity {} month {} year {}",
        month, year, read_usize(this), read_u32(this + 4), read_u32(this + 8), read_u8(this + month_ratings_fields::MONTH), read_i16(this + month_ratings_fields::YEAR)));
}

/// The qsort order of the month ratings (12-byte `month_rating_entry` records): the best average rating first, with
/// those of fewer than 3 matches after all the others. `a` with fewer than 3 matches: 1 when `b` has at least 3, else 0;
/// `b` with fewer than 3 (and `a` not): −1; otherwise the averages (the rating sum, i16 at +8, / `month_rating_entry::MATCHES`, signed)
/// decide: −1 when `a`'s is higher, 1 when lower, 0 when equal. Used by `find_best_player`.
/// Original: `month_ratings_compare_average` (0x768200, month_ratings.cpp, cdecl).
pub(crate) unsafe extern "C" fn month_ratings_compare_average(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    let (ma, mb) = (read_i8(a + month_rating_entry::MATCHES), read_i8(b + month_rating_entry::MATCHES));
    if ma < 3 { // sx04_1: fewer than three matches go last
        return (mb >= 3) as i32;
    }
    if mb < 3 {
        return -1;
    }
    covered!("sx04_1 averages compared");
    let average = |e: usize, m: i8| read_i16(e + 8) as i32 as f64 / m as i32 as f64; // +8: the rating sum
    let (va, vb) = (average(a, ma), average(b, mb));
    if va > vb {
        -1
    } else if va < vb {
        1
    } else {
        0
    }
}

/// Reads the month ratings from the saved game: the entry count (4 bytes at `MONTH_RATINGS_COUNT`); with entries, the
/// capacity becomes the count, a block of count × 12 bytes is allocated (`exe::crt_malloc`) for the entries and they
/// are read; then the month (`month_ratings_fields::MONTH`, a byte) and the year (`month_ratings_fields::YEAR`, 2
/// bytes). Returns 1, or 0 after the box of the first failure: Load Game Error (month_ratings.cpp 86 "cannot read
/// number of players", 105 "cannot read players", 113 "cannot read current month", 120 "cannot read current year")
/// or Memory Allocation Error (98 "cannot malloc players array").
/// Original: `MONTH_RATINGS::load_from_disk` (0x767b20, month_ratings.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn month_ratings_load_from_disk(this: usize, file: usize) -> u32 {
    if exe::zread(this + MONTH_RATINGS_COUNT, 4, 1, file) != 1 {
        award_load_error_box(MONTH_RATINGS_CPP, 0x56, 0xa5b1f4, LOAD_ERROR);
        return 0;
    }
    let count = read_u32(this + MONTH_RATINGS_COUNT);
    if count != 0 {
        write_u32(this + MONTH_RATINGS_CAPACITY, count);
        let entries = exe::crt_malloc(count.wrapping_mul(MONTH_RATING_ENTRY_SIZE));
        write_usize(this + MONTH_RATINGS_ENTRIES, entries);
        if entries == 0 {
            award_load_error_box(MONTH_RATINGS_CPP, 0x62, 0xa5b1b4, MEMORY_ERROR);
            return 0;
        }
        if exe::zread(entries, MONTH_RATING_ENTRY_SIZE, read_u32(this + MONTH_RATINGS_COUNT), file) != read_u32(this + MONTH_RATINGS_COUNT) {
            award_load_error_box(MONTH_RATINGS_CPP, 0x69, 0xa5b17c, LOAD_ERROR);
            return 0;
        }
    }
    if exe::zread(this + month_ratings_fields::MONTH, 1, 1, file) != 1 { // sx04b_6: the ratings' month
        award_load_error_box(MONTH_RATINGS_CPP, 0x71, 0xa5b140, LOAD_ERROR);
        return 0;
    }
    if exe::zread(this + month_ratings_fields::YEAR, 2, 1, file) != 1 {
        award_load_error_box(MONTH_RATINGS_CPP, 0x78, 0xa5b104, LOAD_ERROR);
        return 0;
    }
    covered!("sx04b_6 month ratings loaded");
    1
}

/// Comparer for `month_ratings_load_from_disk`: the load reads the file once and allocates; only the original runs,
/// and loaded ratings must have their capacity at least their count.
pub(crate) unsafe extern "thiscall" fn compare_month_ratings_load_from_disk(this: usize, file: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_MONTH_RATINGS_LOAD_FROM_DISK)(this, file);
    if r == 1 {
        let (n, cap) = (read_u32(this + MONTH_RATINGS_COUNT), read_u32(this + MONTH_RATINGS_CAPACITY));
        SCOREBOARD.record(n == 0 || (cap == n && read_usize(this + MONTH_RATINGS_ENTRIES) != 0), || format!("month_ratings_load_from_disk({:#x}): count {} capacity {}", this, n, cap));
    }
    r
}

/// The box of a failed load from a saved game: "v<version> <file>.<ext> <line> (<message>)" at 0xae24d0, the `box`'s
/// title (0xae28bc) and body with that text (0xae26c4), both translated, and the message box.
pub(crate) unsafe fn award_load_error_box(cpp: usize, line: u32, message: usize, (title, body): (usize, usize)) {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(cpp, 0usize, 0usize, name.as_mut_ptr(), ext.as_mut_ptr());
    let v = version(name.as_mut_ptr(), ext.as_mut_ptr(), line, message);
    comp_crt_format()(0xae24d0, 0x9a8120, v, name.as_ptr(), ext.as_ptr(), line, message); // "v%s %s.%s %d (%s)"
    comp_lang_format()(0xae28bc, title);
    comp_lang_format()(0xae26c4, body, 0xae24d0);
    exe::gui_show_message_box(0xae28bc, 0xae26c4, 0);
}

pub(crate) const MONTH_RATINGS_CPP: usize = 0xa5b234;

/// The month ratings table (`MONTH_RATINGS`): the 12-byte entries (+0), their count (+4) and capacity (+8).
pub(crate) const MONTH_RATINGS_ENTRIES: usize = 0x0;

pub(crate) const MONTH_RATINGS_COUNT: usize = 0x4;

pub(crate) const MONTH_RATINGS_CAPACITY: usize = 0x8;

pub(crate) const MONTH_RATING_ENTRY_SIZE: u32 = 0xc;

/// Builds an empty month ratings table: no entries (+0 pointer, +4 count, +8 capacity), month −1
/// (`month_ratings_fields::MONTH`) and year 0 (`month_ratings_fields::YEAR`), so that the first day of play closes no
/// month; with a save file, the table is then read from it (`MONTH_RATINGS::load_from_disk`). Returns `this`.
/// Original: `MONTH_RATINGS::MONTH_RATINGS` (0x767af0, month_ratings.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn construct_month_ratings(this: usize, file: usize) -> usize {
    covered!("sx04b_2 month ratings built");
    write_u8(this + month_ratings_fields::MONTH, 0xff); // sx04b_2: no month yet
    write_u16(this + month_ratings_fields::YEAR, 0);
    write_u32(this, 0); // the entries
    write_u32(this + 4, 0); // the count
    write_u32(this + 8, 0); // the capacity
    if file != 0 {
        covered!("sx04b_2 month ratings loaded");
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_MONTH_RATINGS_LOAD_FROM_DISK);
        load(this, file);
    }
    this
}

/// Comparer for `construct_month_ratings`: side by side over the 0xf bytes of the table; with a save file (the load
/// allocates) only the original.
pub(crate) unsafe extern "thiscall" fn compare_construct_month_ratings(this: usize, file: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CONSTRUCT_MONTH_RATINGS);
    if file != 0 {
        return o(this, file);
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(this, 0xf)], || o(this, file), || construct_month_ratings(this, file));
    SCOREBOARD.record(equal && r == n, || format!("construct_month_ratings original={:x?} rust={:x?}", mo, mr));
    r
}

/// `MONTH_RATINGS::load_from_disk(file)` (0x767b20, month_ratings.cpp, thiscall, ret 4).
pub(crate) const FN_MONTH_RATINGS_LOAD_FROM_DISK: usize = 0x767b20;
