// ---------- batch %%LEVA%%: the tables of the Swedish, Finnish and Norwegian First Divisions (swe_first.cpp, fin_first.cpp, nor_first.cpp) ----------

/// Allocates a league table of `bytes` bytes into `league::TABLE`; out of memory, the box of `file` at `row` and
/// false.
pub(crate) unsafe fn allocate_first_division_table(this: League, bytes: u32, file: usize, row: u32) -> bool {
    let t = exe::crt_malloc(bytes);
    this.set_table(t);
    if t == 0 {
        show_error_at(file, row);
        return false;
    }
    true
}

/// Fills the table rows with the clubs that `belongs` accepts, in database order, until `.club_count()` rows; returns
/// whether it filled.
///
/// The clubs of `G_CLUB_TABLE` (`club::SIZE` bytes, `G_CLUB_COUNT` of them, a signed word counter), each accepted one
/// into the next row (`exe::league_init_table_row`: push 0; push 0; push club; push row -> thiscall, ret 0x10; the row
/// is a byte), until the signed byte count equals `.club_count()`.
pub(crate) unsafe fn fill_first_division_rows(this: League, belongs: impl Fn(usize) -> bool) -> bool {
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut c = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if belongs(c) {
            exe::league_init_table_row(this, k as u32, c, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == this.club_count() {
                break;
            }
        }
        i = i.wrapping_add(1);
        c = c.wrapping_add(club::SIZE);
    }
    k as i8 as i16 == this.club_count()
}

/// True if the club's division (`club::DIVISION`) is the competition of id `g`.
pub(crate) unsafe fn club_in_division(c: usize, g: usize) -> bool {
    let comp = read_usize(c + club::DIVISION);
    comp != 0 && read_usize(comp) == read_usize(g)
}

/// True if the club is in the Finnish First Division and its reserve division (`club::RESERVE_DIVISION`, here the
/// regional group) is the North group (`KEY_CLUB_COMP_FINNISH_FIRST_DIVISION_NORTH`); a club without either is out.
pub(crate) unsafe fn club_in_finnish_north_group(c: usize) -> bool {
    let comp = read_usize(c + club::DIVISION);
    let group = read_usize(c + club::RESERVE_DIVISION);
    comp != 0 && group != 0 && read_usize(comp) == read_usize(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION) && read_usize(group) == read_usize(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION_NORTH)
}

/// How many clubs of the Norwegian First Division the database has (a byte counter over all the clubs).
pub(crate) unsafe fn norwegian_first_division_count() -> u8 {
    let mut n: u8 = 0;
    let mut i: i16 = 0;
    let mut c = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if club_in_division(c, KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION) {
            n = n.wrapping_add(1);
        }
        i = i.wrapping_add(1);
        c = c.wrapping_add(club::SIZE);
    }
    n
}

/// Creates the Swedish First Division (Superettan) table: 16 clubs, those of the division in database order; no prize
/// fund nor TV money.
///
/// `SweFirst::create_table()`: `.set_club_count()` = 16, the table (`exe::crt_malloc` of 0x3b0; out of memory: the box
/// of swe_first.cpp (0xa8b53c) line 0x144 and 0), the clubs of `KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION` in database order
/// (`fill_first_division_rows`); returns 1 if it filled, otherwise 0.
/// Original: `SweFirst::create_table` (0x88ef90, swe_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_first_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0x10);
    if !allocate_first_division_table(this, 0x3b0, 0xa8b53c, 0x144) {
        return 0;
    }
    covered!("mwe_7 superettan table");
    fill_first_division_rows(this, |c| club_in_division(c, KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION)) as u32 // mwe7: the Superettan clubs
}

/// Creates the Finnish First Division table of the league stage: the North group, 10 clubs (those of the division
/// whose regional group is the North one, in database order); no prize fund nor TV money.
///
/// `FinFirst::create_table()`: `.set_club_count()` = 10, the table (`exe::crt_malloc` of 0x24e; out of memory: the box
/// of fin_first.cpp (0x9c818c) line 0x1ad and 0), the clubs of `club_in_finnish_north_group` in database order;
/// returns 1 if it filled, otherwise 0. (The South group is a separate stage, FinFirst::create_south_group_stage.)
/// Original: `FinFirst::create_table` (0x593a70, fin_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_first_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0xa);
    if !allocate_first_division_table(this, 0x24e, 0x9c818c, 0x1ad) {
        return 0;
    }
    covered!("mwe_7 ykkonen north table");
    fill_first_division_rows(this, |c| club_in_finnish_north_group(c)) as u32 // mwe7: the North group clubs
}

/// Creates the Norwegian First Division table: 16 clubs; when the database has exactly 14 clubs in the division, 14
/// clubs and only 2 relegated; those of the division in database order; no prize fund nor TV money.
///
/// `NorFirst::create_table()`: `.set_club_count()` = 16 and the table (`exe::crt_malloc` of 0x3b0, the size of 16
/// rows in any case; out of memory: the box of nor_first.cpp (0xa69740) line 0x251 and 0); the division's clubs counted
/// over the whole database (`norwegian_first_division_count`, a byte); with exactly 14: `.set_club_count()` = 14,
/// `league::RELEGATED_COUNT` = 2 and `league::GAMES_PER_CLUB` = −1 (to be recomputed); then the clubs of
/// `KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION` in database order; returns 1 if it filled, otherwise 0.
/// Original: `NorFirst::create_table` (0x7909a0, nor_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_first_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0x10);
    if !allocate_first_division_table(this, 0x3b0, 0xa69740, 0x251) {
        return 0;
    }
    if norwegian_first_division_count() == 0xe { // mwe7: the 14-club Norwegian First Division
        covered!("mwe_7 norwegian first division of 14");
        this.set_club_count(0xe);
        write_u8(this.addr() + league::RELEGATED_COUNT, 2);
        write_u16(this.addr() + league::GAMES_PER_CLUB, 0xffff);
    }
    covered!("mwe_7 norwegian first division table");
    fill_first_division_rows(this, |c| club_in_division(c, KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION)) as u32
}

/// The comparer of the three tables: only the original runs (it allocates the table); checked against a plan made
/// beforehand.
///
/// The plan: the accepted clubs in database order, up to `n`; afterwards the count `.club_count()` = `n`, the return
/// value 1 if the plan filled (otherwise 0) and the rows with the planned clubs, in order.
pub(crate) unsafe fn compare_first_division_table(name: &str, this: League, n: i16, belongs: impl Fn(usize) -> bool, original: impl FnOnce() -> u32) -> u32 {
    let mut plan: Vec<usize> = Vec::new();
    let mut c = read_usize(G_CLUB_TABLE);
    for _ in 0..(read_usize(G_CLUB_COUNT) as i32).max(0) {
        if (plan.len() as i16) < n && belongs(c) {
            plan.push(c);
        }
        c += club::SIZE;
    }
    let r = original();
    let t = this.table();
    let mut found: Vec<usize> = Vec::new();
    let mut same = r == (plan.len() as i16 == n) as u32 && this.club_count() == n && t != 0;
    if same {
        found = (0..plan.len()).map(|k| read_usize(t + k * league_table_row::SIZE)).collect();
        same = found == plan;
    }
    SCOREBOARD.record(same, || format!("{}({:#x}) original={} n={} plan={:x?} table={:x?}", name, this.addr(), r, this.club_count(), plan, found));
    r
}

/// Comparer for `swe_first_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_swe_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_first_division_table("swe_first_create_table", League(this), 0x10, |c| club_in_division(c, KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION), || o(this))
}

/// Comparer for `fin_first_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_fin_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_first_division_table("fin_first_create_table", League(this), 0xa, |c| club_in_finnish_north_group(c), || o(this))
}

/// Comparer for `nor_first_create_table`: the size by the count of the division's clubs (14 when exactly 14,
/// otherwise 16), and with 14 also `league::RELEGATED_COUNT` = 2.
pub(crate) unsafe extern "thiscall" fn compare_nor_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    let fourteen = norwegian_first_division_count() == 0xe;
    let r = compare_first_division_table("nor_first_create_table", League(this), if fourteen { 0xe } else { 0x10 }, |c| club_in_division(c, KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION), || o(this));
    if fourteen {
        let ok = read_u8(this + league::RELEGATED_COUNT) == 2;
        SCOREBOARD.record(ok, || format!("nor_first_create_table({:#x}) 14 clubs, relegated {}", this, read_u8(this + league::RELEGATED_COUNT)));
    }
    r
}
