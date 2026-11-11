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
            exe::month_ratings_add_player_rating(this, e.addr());
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

