// ---------- cx08_5: the Argentine Premier Division relegation averages and club progress (arg_prm.cpp) ----------

/// The relegation averages row (`ARG_AVERAGE_ROW_SIZE` bytes): the club id +0 (−1 empty), the points average +4 (a
/// float), the year of the last season +8 (word) and three season columns from +0xa, 3 bytes apart: the points (word)
/// and the games (byte), the oldest first.
pub(crate) const AVERAGE_ROW_AVERAGE: usize = 0x4;
pub(crate) const AVERAGE_ROW_YEAR: usize = 0x8;
pub(crate) const AVERAGE_ROW_SEASONS: usize = 0xa;

/// The two seasons before 2001/02 of the Argentine Premier Division: the club (its global), the points and games of
/// the older season, then of the newer (each with 38 games but for Huracán's, promoted for the newer one only).
const ARG_PRM_SEEDED_AVERAGES: [(usize, i16, u8, i16, u8); 20] = [
    (0x9d00ac, 86, 38, 78, 38), // River Plate
    (0x9d00d8, 69, 38, 81, 38), // San Lorenzo de Almagro
    (0x9d00d0, 74, 38, 71, 38), // Boca Juniors
    (0x9d0108, 58, 38, 61, 38), // Talleres de Córdoba
    (0x9d00b0, 61, 38, 56, 38), // Vélez Sarsfield
    (0x9d0104, 0, 0, 55, 38), // Huracán
    (0x9d00e0, 66, 38, 41, 38), // Rosario Central
    (0x9d00ec, 55, 38, 49, 38), // Colón de Santa Fé
    (0x9d00d4, 49, 38, 55, 38), // Gimnasia y Esgrima de La Plata
    (0x9d00b4, 61, 38, 42, 38), // Independiente
    (0x9d00e4, 55, 38, 48, 38), // Newell's Old Boys
    (0x9d01f4, 45, 38, 56, 38), // Chacarita Juniors
    (0x9d0100, 50, 38, 46, 38), // Unión de Santa Fé
    (0x9d00f8, 39, 38, 56, 38), // Estudiantes de La Plata
    (0x9d00cc, 48, 38, 43, 38), // Lanús
    (0x9d00e8, 45, 38, 40, 38), // Racing Club
    (0x9d00dc, 39, 38, 43, 38), // Argentinos Juniors
    (0x9d010c, 39, 38, 37, 38), // Belgrano de Córdoba
    (0x9d01e4, 0, 0, 0, 0), // Banfield
    (0x9d0200, 0, 0, 0, 0), // Nueva Chicago
];

/// Rule: the Argentine relegation averages start from the real 1999/2000 and 2000/01 seasons of the twenty clubs; if
/// any of them is not in the Premier Division in the database the table starts empty.
///
/// `ArgPrm::init_relegation_averages()`: `ARG_AVERAGE_COUNT` = `league::CLUB_COUNT`; the rows (`exe::crt_calloc`(count,
/// 0x13) into `ARG_AVERAGES`; none: the arg_prm.cpp box of line 0x36e and nothing else) seeded with
/// `ARG_PRM_SEEDED_AVERAGES` (the id at each club's global, the year `league::YEAR` and the first two season columns;
/// the 20 rows are written whatever the count). Then each row below the count (read once): the club of its id
/// (`club_at`, unchecked) must have a division (`club::DIVISION`) whose id is that of
/// `KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION`; at the first that has not, the rows are freed and a new zeroed block
/// allocated (none: the box of line 0x3e7) whose rows get the id −1 and the year.
/// Defect of the original: a club missing from the database (id −1) is looked up 0x245 bytes before the club table.
/// Original: `ArgPrm::init_relegation_averages` (0x405ef0, arg_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn arg_prm_init_relegation_averages(this: usize) {
    let this = League(this);
    write_i16(this.addr() + ARG_AVERAGE_COUNT, this.club_count());
    let rows = exe::crt_calloc(this.club_count() as i32 as u32, ARG_AVERAGE_ROW_SIZE as u32);
    write_usize(this.addr() + ARG_AVERAGES, rows);
    if rows == 0 {
        return show_error_at(ARG_PRM_FILE, 0x36e);
    }
    for (k, &(global, p1, g1, p2, g2)) in ARG_PRM_SEEDED_AVERAGES.iter().enumerate() {
        let row = rows + k * ARG_AVERAGE_ROW_SIZE;
        write_u32(row, read_u32(global));
        write_i16(row + AVERAGE_ROW_SEASONS, p1);
        write_u8(row + AVERAGE_ROW_SEASONS + 2, g1);
        write_i16(row + AVERAGE_ROW_SEASONS + 3, p2);
        write_u8(row + AVERAGE_ROW_SEASONS + 5, g2);
        write_i16(row + AVERAGE_ROW_YEAR, this.year());
    }
    let count = this_average_count(this);
    for k in 0..count {
        let club = club_at(arg_average_row_club(this, k) as isize);
        let division = club.division().addr();
        if division == 0 || read_usize(division) != read_usize(KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION) {
            covered!("cx08_5 argentine averages start empty");
            exe::crt_free(rows);
            reset_relegation_averages(this, ARG_PRM_FILE, 0x3e7);
            return;
        }
    }
    covered!("cx08_5 argentine averages seeded");
}

/// A new, empty averages table: a zeroed block of `ARG_AVERAGE_COUNT` rows (`exe::crt_calloc`, into `ARG_AVERAGES`;
/// none: the box of `file` at `row`), each with the id −1 and the year `league::YEAR` (the count read again each row).
pub(crate) unsafe fn reset_relegation_averages(this: League, file: usize, row: u32) {
    let rows = exe::crt_calloc(this_average_count(this) as u32, ARG_AVERAGE_ROW_SIZE as u32);
    write_usize(this.addr() + ARG_AVERAGES, rows);
    if rows == 0 {
        return show_error_at(file, row);
    }
    let mut k = 0i32;
    while k < this_average_count(this) {
        let r = read_usize(this.addr() + ARG_AVERAGES) + k as usize * ARG_AVERAGE_ROW_SIZE;
        write_i32(r, -1);
        write_i16(r + AVERAGE_ROW_YEAR, this.year());
        k += 1;
    }
}

/// Rule: the Argentine averages take each club's points and games of this season from the overall stage; on a new
/// season the columns roll (the oldest dropped, the year + 1); the average is the points of the three seasons over
/// their games; the rows are sorted by the average; on a new season the relegated clubs and those no longer in the
/// division leave the table.
///
/// `ArgPrm::update_relegation_averages(roll)`: for each row k below `league::CLUB_COUNT` of stage 1's table, the
/// averages row of its club's id (else the first row with −1, which takes the id; else nothing) gets the row's points
/// (`league_table_row::POINTS`) and games (`league_table_row::PLAYED`) in the third column. Then each averages row (the
/// count read again each time): with `roll`, the second column to the first, the third to the second, the third zeroed
/// and the year + 1 (the last row's year kept); the average (`AVERAGE_ROW_AVERAGE`) = the three points words over the
/// three games bytes (signed; 0 without games; a float). The rows sorted (`exe::crt_qsort` with
/// `comp_compare_average_rows`, 0x48f410). With `roll`, each club of the league table with status 3 or whose division is
/// no longer this competition (`club::DIVISION` against `league::COMPETITION`) has its averages row zeroed, with the id
/// −1 and the kept year.
/// Original: `ArgPrm::update_relegation_averages` (0x406570, arg_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn arg_prm_update_relegation_averages(this: usize, roll: u32) {
    let this = League(this);
    covered!("cx08_5 argentine averages updated");
    take_season_points(this, League(read_usize(this.stages() + 4)));
    let year = average_seasons(this, roll);
    if roll != 0 {
        covered!("cx08_5 argentine averages roll");
        drop_departed_clubs(this, year);
    }
}

/// The averages row index of club id `id` (−1 when none).
pub(crate) unsafe fn find_average_row(this: League, id: i32) -> i32 {
    let mut j: i16 = 0;
    while (j as i32) < this_average_count(this) {
        if arg_average_row_club(this, j as i32) == id {
            return j as i32;
        }
        j += 1;
    }
    -1
}

/// This season's points and games of each club of `source`'s table (rows below this league's `league::CLUB_COUNT`) into
/// the third column of its averages row, a new row (the first with id −1) for a club not yet in the table, none when
/// the table is full.
pub(crate) unsafe fn take_season_points(this: League, source: League) {
    let mut k: i16 = 0;
    while k < this.club_count() {
        let row = LeagueTableRow(source.table() + k as usize * league_table_row::SIZE);
        let id = read_i32(row.club() as usize);
        let mut j = find_average_row(this, id);
        if j < 0 {
            j = find_average_row(this, -1);
            if j >= 0 {
                write_i32(read_usize(this.addr() + ARG_AVERAGES) + j as usize * ARG_AVERAGE_ROW_SIZE, id);
            }
        }
        if j >= 0 {
            let a = read_usize(this.addr() + ARG_AVERAGES) + j as usize * ARG_AVERAGE_ROW_SIZE;
            write_i16(a + AVERAGE_ROW_SEASONS + 6, read_i16(row.addr() + league_table_row::POINTS));
            write_u8(a + AVERAGE_ROW_SEASONS + 8, read_u8(row.addr() + league_table_row::PLAYED));
        }
        k += 1;
    }
}

/// The columns rolled (with `roll`), the averages worked out and the rows sorted; returns the last rolled year (0 when
/// none rolled: the original's local is then never read).
pub(crate) unsafe fn average_seasons(this: League, roll: u32) -> i16 {
    let mut year = 0i16;
    let mut j = 0i32;
    while j < this_average_count(this) {
        let a = read_usize(this.addr() + ARG_AVERAGES) + j as usize * ARG_AVERAGE_ROW_SIZE;
        let s = a + AVERAGE_ROW_SEASONS;
        if roll != 0 {
            core::ptr::copy((s + 3) as *const u8, s as *mut u8, 3);
            core::ptr::copy((s + 6) as *const u8, (s + 3) as *mut u8, 3);
            write_i16(s + 6, 0);
            write_u8(s + 8, 0);
            year = read_i16(a + AVERAGE_ROW_YEAR).wrapping_add(1);
            write_i16(a + AVERAGE_ROW_YEAR, year);
        }
        let games = read_i8(s + 8) as i32 + read_i8(s + 5) as i32 + read_i8(s + 2) as i32;
        if games > 0 {
            let points = read_i16(s + 6) as i32 + read_i16(s + 3) as i32 + read_i16(s) as i32;
            write_f32(a + AVERAGE_ROW_AVERAGE, (points as f64 / games as f64) as f32);
        } else {
            write_u32(a + AVERAGE_ROW_AVERAGE, 0);
        }
        j += 1;
    }
    exe::crt_qsort(read_usize(this.addr() + ARG_AVERAGES), this_average_count(this) as u32, ARG_AVERAGE_ROW_SIZE as u32, 0x48f410usize);
    year
}

/// On a new season, the averages row of each club of the league table that went down (status 3) or left the division
/// is cleared: zeroed, the id −1 and `year`.
pub(crate) unsafe fn drop_departed_clubs(this: League, year: i16) {
    let mut k: i16 = 0;
    while k < this.club_count() {
        let row = LeagueTableRow(this.table() + k as usize * league_table_row::SIZE);
        let club = Club(row.club() as usize);
        if row.status() == 3 || club.division().addr() != this.competition().addr() {
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

/// Stores a club's progress in the Argentine Premier Division: in the league and its two stages (−1, 0 and 1) the codes
/// above 17 become 18; in other stages as they come.
///
/// `ArgPrm::set_club_progress(club, stage, current, best, worst)`: the club's entry (`exe::comp_stats_find_club_entry`
/// on `league::COMP_STATS`, unchecked); with the stage byte 0xff, 0 or 1, each code at or above 0x12 (unsigned) is 0x12;
/// the bytes go to `comp_stats_club::BEST_POSITION`, `CURRENT_POSITION` and `WORST_POSITION`.
/// Original: `ArgPrm::set_club_progress` (0x406b80, arg_prm.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn arg_prm_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let e = exe::comp_stats_find_club_entry(League(this).comp_stats(), club);
    let (mut c, mut b, mut w) = (current as u8, best as u8, worst as u8);
    if matches!(stage as u8, 0xff | 0 | 1) {
        covered!("cx08_5 argentine progress capped");
        (c, b, w) = (c.min(0x12), b.min(0x12), w.min(0x12));
    }
    write_u8(e + comp_stats_club::BEST_POSITION, b);
    write_u8(e + comp_stats_club::CURRENT_POSITION, c);
    write_u8(e + comp_stats_club::WORST_POSITION, w);
}

/// Comparer for `arg_prm_init_relegation_averages`: only the original runs (it allocates the rows); afterwards the
/// count must be the club count and the rows either the seeded ones (ids and season columns) or all empty.
pub(crate) unsafe extern "thiscall" fn compare_arg_prm_init_relegation_averages(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let l = League(this);
    let n = this_average_count(l);
    let rows = read_usize(this + ARG_AVERAGES);
    let seeded = rows != 0 && (0..n.min(20) as usize).all(|k| {
        let (g, p1, g1, p2, g2) = ARG_PRM_SEEDED_AVERAGES[k];
        let r = rows + k * ARG_AVERAGE_ROW_SIZE;
        read_u32(r) == read_u32(g) && read_i16(r + AVERAGE_ROW_SEASONS) == p1 && read_u8(r + AVERAGE_ROW_SEASONS + 2) == g1
            && read_i16(r + AVERAGE_ROW_SEASONS + 3) == p2 && read_u8(r + AVERAGE_ROW_SEASONS + 5) == g2
    });
    let empty = rows != 0 && (0..n).all(|k| arg_average_row_club(l, k) == -1);
    SCOREBOARD.record(n == l.club_count() as i32 && (seeded || empty), || format!("arg_prm_init_relegation_averages({:#x}): count {} rows {:#x} seeded {} empty {}", this, n, rows, seeded, empty));
}

/// Comparer of the Argentine averages updates: side by side over the averages rows (`ARG_AVERAGE_COUNT` × 0x13 bytes);
/// with `roll` only the original (the rows of departed clubs are cleared along with the season's move).
pub(crate) unsafe fn compare_relegation_averages_update(name: &str, this: usize, roll: u32, original: impl FnOnce(), copy: impl FnOnce()) {
    let rows = read_usize(this + ARG_AVERAGES);
    let n = this_average_count(League(this)).max(0) as usize;
    if rows == 0 || roll != 0 {
        return original();
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(rows, n * ARG_AVERAGE_ROW_SIZE)], original, copy);
    SCOREBOARD.record(same, || format!("{}({:#x}, roll {}): rows differ, original {:x?} rust {:x?}", name, this, roll, mo, mr));
}

pub(crate) unsafe extern "thiscall" fn compare_arg_prm_update_relegation_averages(this: usize, roll: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I1%%);
    compare_relegation_averages_update("arg_prm_update_relegation_averages", this, roll, || o(this, roll), || arg_prm_update_relegation_averages(this, roll))
}

/// Comparer for `arg_prm_set_club_progress`: side by side on the 3 bytes of the entry (`compare_group_championship_progress`).
pub(crate) unsafe extern "thiscall" fn compare_arg_prm_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    compare_group_championship_progress("arg_prm_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || arg_prm_set_club_progress(this, club, stage, current, best, worst));
}
