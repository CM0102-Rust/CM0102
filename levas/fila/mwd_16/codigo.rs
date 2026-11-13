// ---------- batch %%LEVA%%: the tables of the Russian First Division, the Polish Second Division and the Turkish Second Division ----------

/// Rule: the Russian First Division has 18 clubs: the first 18 clubs of the division
/// (`KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION`) in database order (any more are left out of the table), sorted by
/// reputation and shuffled (all of them), in that order; with fewer than 18, no table (0). No prize fund nor TV money.
///
/// `club_count` = 18; the table (`exe::crt_malloc` of 18 × 0x3b; out of memory: the box at line 0xf0 and 0); a local
/// list of at most 18 club pointers (16-bit database index); fewer than `club_count`: 0 (the table stays allocated);
/// otherwise `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, `club_count`) (cdecl; the original pushes n and
/// `club_count` with stack garbage above the words, which the routine does not read) and each club i in row i
/// (League::init_table_row(this, i, club, 0, 0)); a club beyond `club_count` (never, the list is capped at 18) would be
/// moved to the south zone of the second division (`KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_SOUTH`, `club::DIVISION`).
/// Returns 1.
/// Original: `RusFirst::create_table` (0x7ea6c0, rus_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_first_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0x12);
    let t = exe::crt_malloc(0x426);
    this.set_table(t);
    if t == 0 {
        show_error_at(0xa7ac8c, 0xf0);
        return 0;
    }
    let mut list = [0usize; 0x12];
    let mut n: u8 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION) && (n as i8) < 0x12 {
            list[n as i8 as usize] = club;
            n = n.wrapping_add(1);
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    let n = n as i8 as i16;
    if n < this.club_count() {
        return 0;
    }
    covered!("mwd_16 russian first division table");
    exe::comp_util_sort_and_shuffle_top_clubs(list.as_ptr() as usize, n as u16 as u32, this.club_count() as u16 as u32); // mwd16: sorted and shuffled
    let mut k: i16 = 0;
    while k < n {
        let c = list[k as usize];
        if k < this.club_count() {
            exe::league_init_table_row(this, k as u16 as u32, c, 0, 0);
        } else {
            write_usize(c + club::DIVISION, comp_record_of(KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_SOUTH));
        }
        k += 1;
    }
    1
}

/// Rule: the Polish Second Division has 20 clubs in 2001 and 18 afterwards, those of the division
/// (`KEY_CLUB_COMP_POLISH_SECOND_DIVISION`) in database order; when full, the prize fund (20,000 a share); no TV money.
///
/// `club_count` = 20, games per club not computed (`league::GAMES_PER_CLUB` = −1), `club_count` = 18 unless the year is
/// 2001; then `eng_prm_fill_league_table` (the box at line 0x199; League::calc_prize_fund push 20000, `prize_mode` 1)
/// with no TV payment.
/// Original: `PolSecond::create_table` (0x7cbdf0, pol_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_second_create_table(this: usize) -> u32 {
    let l = League(this);
    l.set_club_count(0x14);
    write_u16(this + league::GAMES_PER_CLUB, 0xffff);
    if l.year() != 0x7d1 {
        l.set_club_count(0x12);
    }
    let n = l.club_count();
    eng_prm_fill_league_table(l, n, 0xa7860c, 0x199, KEY_CLUB_COMP_POLISH_SECOND_DIVISION, 0x4e20, || { covered!("mwd_16 polish second division table"); }) // mwd16: the Polish prize share
}

/// Fills a league table with the clubs of the competition of id [`g`] in database order until it is full, as
/// `eng_prm_fill_league_table` but without the prize fund: `club_count` = n, the table (out of memory: the box at
/// `row` and false), League::init_table_row(this, k, club, 0, 0) for each club; true when full.
pub(crate) unsafe fn fill_league_table_rows(this: League, n: i16, file: usize, row: u32, g: usize) -> bool {
    this.set_club_count(n);
    let t = exe::crt_malloc((n as i32 as u32).wrapping_mul(0x3b));
    this.set_table(t);
    if t == 0 {
        show_error_at(file, row);
        return false;
    }
    let mut k: u8 = 0;
    let mut i: i32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let comp = read_usize(club + club::DIVISION);
        if comp != 0 && read_usize(comp) == read_usize(g) {
            exe::league_init_table_row(this, k as u32, club, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == this.club_count() {
                break;
            }
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    k as i8 as i16 == this.club_count()
}

/// Rule: the Turkish Second Division has 20 clubs, those of the second division category A
/// (`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A`) in database order; when full, no prize fund (+0x28 = 0, prize mode
/// 2 = nothing) and TV pays 100,000 to each club (League::pay_tv_rights: push 0; push 100000).
/// Original: `TurSecond::create_table` (0x8ff150, tur_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_create_table(this: usize) -> u32 {
    let l = League(this);
    if !fill_league_table_rows(l, 0x14, 0xaa187c, 0x159, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A) {
        return 0;
    }
    covered!("mwd_16 turkish second division table");
    l.set_prize_fund(0);
    l.set_prize_mode(2);
    eng_prm_pay_tv_rights(this, 0x186a0, 0); // mwd16: the Turkish Second Division TV money
    1
}

/// Comparer for `rus_first_create_table`: only the original runs (it draws); the table must hold 18 clubs of the
/// division when there are at least 18 (any order), and the return 1; otherwise 0.
pub(crate) unsafe extern "thiscall" fn compare_rus_first_create_table(this: usize) -> u32 {
    let l = League(this);
    let plan = division_clubs_plan(0x12, 0x57, KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION);
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    let mut found: Vec<usize> = Vec::new();
    let mut ok = r == (plan.len() == 0x12) as u32 && l.club_count() == 0x12 && l.table() != 0;
    if ok && r == 1 {
        found = (0..0x12).map(|k| read_usize(l.table() + k * 0x3b)).collect();
        let (mut a, mut b) = (plan.clone(), found.clone());
        a.sort_unstable();
        b.sort_unstable();
        ok = a == b;
    }
    SCOREBOARD.record(ok, || format!("rus_first_create_table({:#x}) original={} plan={:x?} table={:x?}", this, r, plan, found));
    r
}

/// Comparer for `pol_second_create_table` (only the original runs; the planned clubs and the prize fund checked).
pub(crate) unsafe extern "thiscall" fn compare_pol_second_create_table(this: usize) -> u32 {
    let n = if League(this).year() == 0x7d1 { 0x14 } else { 0x12 };
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    eng_prm_compare_league_table("pol_second_create_table", League(this), n, KEY_CLUB_COMP_POLISH_SECOND_DIVISION, 0x4e20, || o(this))
}

/// Comparer for `tur_second_create_table`: only the original runs; the planned clubs (`compare_division_table`) and,
/// when full, no prize fund with prize mode 2.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    let r = compare_division_table("tur_second_create_table", League(this), 0x14, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A, || o(this));
    if r == 1 {
        let l = League(this);
        SCOREBOARD.record(l.prize_fund() == 0 && l.prize_mode() == 2, || format!("tur_second_create_table({:#x}) prize fund {} mode {}", this, l.prize_fund(), l.prize_mode()));
    }
    r
}
