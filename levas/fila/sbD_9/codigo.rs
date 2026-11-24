// ---------- sbD_9: the Argentine Second Division relegation averages (arg_second.cpp) ----------

/// The two seasons before 2001/02 of the Argentine Second Division (Primera B Nacional): the club (its global), the
/// points and games of the older season, then of the newer (the zones of 30, 32 or 34 games, 24 or 26 in the second
/// phase; 0 for a club that did not play it).
const ARG_SECOND_SEEDED_AVERAGES: [(usize, i16, u8, i16, u8); 25] = [
    (0x9d01a4, 0, 0, 71, 32),
    (0x9d01ec, 63, 34, 50, 26),
    (0x9d01e8, 63, 34, 0, 0),
    (0x9d021c, 62, 34, 0, 0),
    (0x9d0208, 55, 34, 44, 26),
    (0x9d01d8, 47, 30, 55, 32),
    (0x9d01d0, 50, 30, 51, 32),
    (0x9d01a8, 49, 30, 50, 32),
    (0x9d00fc, 0, 0, 50, 32),
    (0x9d01b0, 51, 30, 45, 32),
    (0x9d01b8, 40, 30, 50, 32),
    (0x9d00bc, 38, 30, 52, 32),
    (0x9d01dc, 46, 30, 43, 32),
    (0x9d01e0, 43, 30, 44, 32),
    (0x9d0204, 52, 34, 29, 24),
    (0x9d00c0, 41, 30, 43, 32),
    (0x9d00f0, 45, 34, 33, 24),
    (0x9d00b8, 45, 30, 35, 32),
    (0x9d022c, 45, 34, 26, 24),
    (0x9d0228, 50, 34, 20, 24),
    (0x9d01f0, 38, 34, 31, 24),
    (0x9d01c0, 41, 30, 32, 32),
    (0x9d01bc, 29, 30, 43, 32),
    (0x9d0210, 0, 0, 0, 0),
    (0x9d020c, 0, 0, 0, 0),
];

/// Rule: the Argentine Second Division relegation averages start from the real 1999/2000 and 2000/01 seasons of
/// twenty-five clubs; if any club of the table is not in the Second Division in the database, the table starts empty.
///
/// `ArgSecond::init_relegation_averages()`: `ARG_AVERAGE_COUNT` = `league::CLUB_COUNT`; the rows (`exe::crt_calloc`(count,
/// 0x13) into `ARG_AVERAGES`; none: the arg_second.cpp box of line 0x638 and nothing else), each below the count first
/// emptied (id −1, the year `league::YEAR`; the index a signed byte), then the 25 rows of `ARG_SECOND_SEEDED_AVERAGES`
/// (the id at each club's global, the first two season columns, the year) whatever the count. Then each row below the
/// count (read once): the club of its id (`club_at`, unchecked) must have a division (`club::DIVISION`) whose id is that
/// of `KEY_CLUB_COMP_ARGENTINE_SECOND_DIVISION`; at the first that has not, the rows are freed and a new empty table made
/// (`reset_relegation_averages`, the box of line 0x6d1).
/// Defects of the original: with fewer than 25 clubs the seeded rows are written past the block (the copy reserves 25
/// rows: correction, 24/11/2026); a row with id −1 is
/// looked up 0x245 bytes before the club table.
/// Original: `ArgSecond::init_relegation_averages` (0x4093f0, arg_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn arg_second_init_relegation_averages(this: usize) {
    let this = League(this);
    write_i16(this.addr() + ARG_AVERAGE_COUNT, this.club_count());
    // correction of the original (24/11/2026, Lucas): the 25 seeded rows are written whatever the count, past the block
    // with fewer than 25 clubs (a heap overflow); the copy reserves at least 25 rows (docs/DEFEITOS_DO_ORIGINAL.md)
    let rows = exe::crt_calloc((this.club_count() as i32).max(ARG_SECOND_SEEDED_AVERAGES.len() as i32) as u32, ARG_AVERAGE_ROW_SIZE as u32);
    write_usize(this.addr() + ARG_AVERAGES, rows);
    if rows == 0 {
        return show_error_at(ARG_SECOND_FILE, 0x638);
    }
    let row_at = |k: i8| rows.wrapping_add((k as isize as usize).wrapping_mul(ARG_AVERAGE_ROW_SIZE));
    if this_average_count(this) > 0 {
        let mut k: i8 = 0;
        loop {
            write_i32(row_at(k), -1);
            write_i16(row_at(k) + AVERAGE_ROW_YEAR, this.year());
            k = k.wrapping_add(1);
            if k as i32 >= this_average_count(this) {
                break;
            }
        }
    }
    for (k, &(global, p1, g1, p2, g2)) in ARG_SECOND_SEEDED_AVERAGES.iter().enumerate() {
        let row = rows + k * ARG_AVERAGE_ROW_SIZE;
        write_u32(row, read_u32(global));
        write_i16(row + AVERAGE_ROW_SEASONS, p1);
        write_u8(row + AVERAGE_ROW_SEASONS + 2, g1);
        write_i16(row + AVERAGE_ROW_SEASONS + 3, p2);
        write_u8(row + AVERAGE_ROW_SEASONS + 5, g2);
        write_i16(row + AVERAGE_ROW_YEAR, this.year());
    }
    let count = this_average_count(this);
    let mut k: i8 = 0;
    while (k as i32) < count {
        let club = club_at(arg_average_row_club(this, k as i32) as isize);
        let division = club.division().addr();
        if division == 0 || read_usize(division) != read_usize(KEY_CLUB_COMP_ARGENTINE_SECOND_DIVISION) {
            covered!("sbD_9 argentine second division averages start empty");
            exe::crt_free(rows);
            reset_relegation_averages(this, ARG_SECOND_FILE, 0x6d1);
            return;
        }
        k = k.wrapping_add(1);
    }
    covered!("sbD_9 argentine second division averages seeded");
}

/// Adds each zone stage's points and games (stages 0 up to `league::CURRENT_STAGE`, `league::STAGES`) to the third
/// column of the clubs' averages rows (a new row, the first with id −1, for a club not yet in; none when full).
/// As the original, the stage index is the register reused by the row search: after a stage with clubs, the next stage
/// looked at is the last search's index (the row found, or the count, or 0 with an empty table) plus one. Defect of the
/// original: a stage whose last club lands on an averages row below the current stage minus one sends the loop back to
/// an earlier stage, and it can loop forever (both the original and the copy, under the emulator, with made-up data).
unsafe fn add_zone_points(this: League) {
    let mut stage: i32 = 0;
    if this.current_stage() as i32 <= 0 {
        return;
    }
    loop {
        let zone = League(read_usize(this.stages() + (stage as i16 as isize as usize).wrapping_mul(4)));
        let mut k: i16 = 0;
        while k < zone.club_count() {
            let row = LeagueTableRow(zone.table() + k as usize * league_table_row::SIZE);
            let id = read_i32(row.club() as usize);
            let n = this_average_count(this);
            let found = find_average_row(this, id);
            if found >= 0 {
                let a = read_usize(this.addr() + ARG_AVERAGES) + found as usize * ARG_AVERAGE_ROW_SIZE;
                write_i16(a + AVERAGE_ROW_SEASONS + 6, read_i16(a + AVERAGE_ROW_SEASONS + 6).wrapping_add(read_i16(row.addr() + league_table_row::POINTS)));
                write_u8(a + AVERAGE_ROW_SEASONS + 8, read_u8(a + AVERAGE_ROW_SEASONS + 8).wrapping_add(read_u8(row.addr() + league_table_row::PLAYED)));
                stage = found;
            } else {
                let free = find_average_row(this, -1);
                if free >= 0 {
                    let a = read_usize(this.addr() + ARG_AVERAGES) + free as usize * ARG_AVERAGE_ROW_SIZE;
                    write_i32(a, id);
                    write_i16(a + AVERAGE_ROW_SEASONS + 6, read_i16(row.addr() + league_table_row::POINTS));
                    write_u8(a + AVERAGE_ROW_SEASONS + 8, read_u8(row.addr() + league_table_row::PLAYED));
                    stage = free;
                } else {
                    stage = n.max(0);
                }
            }
            k += 1;
        }
        stage += 1;
        if (stage as i16 as i32) >= this.current_stage() as i32 {
            break;
        }
    }
}

/// On a new season, the averages row of each club of the league table that is not staying in this division is
/// cleared (zeroed, the id −1 and `year`): status 3 (relegated), status 0 (promoted) or a division other than this
/// competition (`club::DIVISION` against `league::COMPETITION`).
unsafe fn drop_clubs_leaving_second_division(this: League, year: i16) {
    let mut k: i16 = 0;
    while k < this.club_count() {
        let row = LeagueTableRow(this.table() + k as usize * league_table_row::SIZE);
        let club = Club(row.club() as usize);
        let s = row.status();
        if s == 3 || club.division().addr() != this.competition().addr() || s == 0 {
            let j = find_average_row(this, read_i32(club.addr()));
            if j >= 0 {
                let a = read_usize(this.addr() + ARG_AVERAGES) + j as usize * ARG_AVERAGE_ROW_SIZE;
                core::ptr::write_bytes(a as *mut u8, 0, ARG_AVERAGE_ROW_SIZE);
                write_i32(a, -1);
                write_i16(a + AVERAGE_ROW_YEAR, year);
            }
        }
        k += 1;
    }
}

/// Rule: the Argentine Second Division averages take each club's points and games of this season from the overall
/// table and add those of the zone stages played; on a new season the columns roll; the average is the points of the
/// three seasons over their games; the rows are sorted by the average; on a new season the clubs relegated, promoted
/// or gone from the division leave the table.
///
/// `ArgSecond::update_relegation_averages(roll)`: this league's own table into the third column (`take_season_points`
/// with this league as the source), the zone stages added (`add_zone_points`), the columns rolled with `roll`, the
/// averages worked out and the rows sorted (`average_seasons`); with `roll` the leaving clubs' rows cleared
/// (`drop_clubs_leaving_second_division`).
/// Original: `ArgSecond::update_relegation_averages` (0x409c20, arg_second.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn arg_second_update_relegation_averages(this: usize, roll: u32) {
    let this = League(this);
    covered!("sbD_9 argentine second division averages updated");
    take_season_points(this, this);
    add_zone_points(this);
    let year = average_seasons(this, roll);
    if roll != 0 {
        covered!("sbD_9 argentine second division averages roll");
        drop_clubs_leaving_second_division(this, year);
    }
}

/// Comparer for `arg_second_init_relegation_averages`: only the original runs (it allocates the rows); afterwards the
/// count must be the club count and the rows either the seeded ones (ids and season columns) or all empty.
pub(crate) unsafe extern "thiscall" fn compare_arg_second_init_relegation_averages(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let l = League(this);
    let n = this_average_count(l);
    let rows = read_usize(this + ARG_AVERAGES);
    let seeded = rows != 0 && (0..n.min(25) as usize).all(|k| {
        let (g, p1, g1, p2, g2) = ARG_SECOND_SEEDED_AVERAGES[k];
        let r = rows + k * ARG_AVERAGE_ROW_SIZE;
        read_u32(r) == read_u32(g) && read_i16(r + AVERAGE_ROW_SEASONS) == p1 && read_u8(r + AVERAGE_ROW_SEASONS + 2) == g1
            && read_i16(r + AVERAGE_ROW_SEASONS + 3) == p2 && read_u8(r + AVERAGE_ROW_SEASONS + 5) == g2
    });
    let empty = rows != 0 && (0..n).all(|k| arg_average_row_club(l, k) == -1);
    SCOREBOARD.record(n == l.club_count() as i32 && (seeded || empty), || format!("arg_second_init_relegation_averages({:#x}): count {} rows {:#x} seeded {} empty {}", this, n, rows, seeded, empty));
}

/// Comparer for `arg_second_update_relegation_averages`: side by side over the averages rows; with `roll` only the
/// original (`compare_relegation_averages_update`).
pub(crate) unsafe extern "thiscall" fn compare_arg_second_update_relegation_averages(this: usize, roll: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I1%%);
    compare_relegation_averages_update("arg_second_update_relegation_averages", this, roll, || o(this, roll), || arg_second_update_relegation_averages(this, roll))
}
