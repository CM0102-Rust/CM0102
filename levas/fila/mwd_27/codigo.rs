// ---------- batch %%LEVA%%: the Turkish and Russian cup entrants (tur_cup.cpp, rus_cup.cpp) ----------

/// Writes entry `k` of a cup's 6-byte team list (`cup::TEAM_TABLE`): the club, status 0 (+5) and class `class` (+4).
pub(crate) unsafe fn set_cup_entry(this: Cup, k: usize, club: usize, class: u8) {
    write_usize(this.team_table() + 6 * k, club);
    write_u8(this.team_table() + 6 * k + 5, 0);
    write_u8(this.team_table() + 6 * k + 4, class);
}

/// Rule: the Turkish Cup takes 64 clubs, all class 1: 26 of the second division category B (the first 30 in database
/// order, sorted by their last division, `compare_clubs_by_last_division`, taken from the 26th backwards), then the
/// first 20 of category A in reverse database order, and the top-flight clubs from the end of the list backwards.
///
/// `team_count` = 64; the list (`exe::crt_malloc` of 0x180 into `cup::TEAM_TABLE`; out of memory: the box at line 0x142
/// and −1). Over the first [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`] clubs with a division (`club::DIVISION`): category
/// B (`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B`) into a local list of at most 30; else category A
/// (`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A`) into a local list of at most 20; else the top flight
/// (`KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION`) straight into the entries 63, 62, ... The category B list sorted (qsort
/// `exe::crt_qsort`(list, n, 4, 0x4c0a40)) and its entries 25..0 copied to the entries 0..25 (the original copies 26
/// whatever the count: with fewer it reads stack garbage; the copy reads zeros), then the category A list from its last
/// to its first into the entries 26 onwards. Returns 1.
/// Original: `TurCup::collect_entrants` (0x8fcb90, tur_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let mut lower = [0usize; 30];
    let mut second = [0usize; 20];
    let (mut nl, mut ns, mut np) = (0i16, 0i16, 0i32);
    this.set_team_count(0x40);
    let l = exe::crt_malloc(0x180);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0xaa180c, 0x142);
        return -1;
    }
    let limit = || (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < limit() {
        let d = read_usize(club + club::DIVISION);
        if d != 0 {
            if nl < 0x1e && read_usize(d) == read_usize(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B) {
                lower[nl as usize] = club;
                nl += 1;
            } else if read_usize(d) == read_usize(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A) && ns < 0x14 {
                second[ns as usize] = club;
                ns += 1;
            } else if read_usize(d) == read_usize(KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION) {
                np += 1;
                set_cup_entry(this, (this.team_count() as i32 - np) as usize, club, 1);
            }
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    covered!("mwd_27 turkish cup entrants");
    exe::crt_qsort(lower.as_mut_ptr() as usize, nl as i32 as u32, 4, 0x4c0a40usize);
    for k in 0..0x1a {
        set_cup_entry(this, k, lower[0x19 - k], 1); // mwd27: category B from the 26th backwards
    }
    let mut j = 0usize;
    let mut k = ns as i32 - 1;
    while k >= 0 {
        set_cup_entry(this, 0x1a + j, second[k as usize], 1);
        j += 1;
        k -= 1;
    }
    1
}

/// Rule: the Russian Cup takes 48 clubs: the First Division clubs (class 2; the cup holder among them class 1) from
/// entry 31 backwards, then the second division zone clubs (sorted by strength, the best 42 shuffled) until 32 are in,
/// class 2; then the Premier League clubs (class 1) from entry 32 on; returns 1 when the list is full.
///
/// `team_count` = 48; the list (`exe::crt_malloc` of 0x120; out of memory: the box at line 0x148 and −1). The cup holder
/// is the latest winner of this competition (comp_util_get_latest_placed_club(+4, 0)). The clubs looked at are the first
/// [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`]. The zone clubs (division pointer = the record of one of the six zones,
/// `RUSSIAN_ZONE_PAIRS`) into a local list of at most 100 (a signed byte count), sorted and shuffled
/// (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 14)). Each First Division club (division pointer = its record):
/// k + 1 → entry 32 − k, class 2 (1 for the holder). Then up to 14 zone clubs, while k < 32, the same way. Then each
/// Premier League club in entry k, k + 1, ... with class 1. Returns k ≥ `team_count`.
/// Original: `RusCup::select_entrants` (0x7e99a0, rus_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_cup_select_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let zones: [[usize; 2]; 3] = RUSSIAN_ZONE_PAIRS.map(|p| p.map(|g| unsafe { comp_record_of(g) }));
    let (first, premier) = (comp_record_of(KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION), comp_record_of(KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION));
    let mut zone_clubs = [0usize; 100];
    let mut nz: i8 = 0;
    let mut k: i8 = 0;
    this.set_team_count(0x30);
    let l = exe::crt_malloc(0x120);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0xa7ac58, 0x148);
        return -1;
    }
    let holder = exe::comp_util_get_latest_placed_club(read_usize(this.addr() + 4), 0);
    let limit = || (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let count = limit();
    for i in 0..count.max(0) as usize {
        let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        let d = read_usize(c + club::DIVISION);
        if zones.iter().any(|p| d == p[0] || d == p[1]) && nz < 0x64 {
            zone_clubs[nz as usize] = c;
            nz += 1;
        }
    }
    exe::comp_util_sort_and_shuffle_top_clubs(zone_clubs.as_ptr() as usize, nz as i16 as u16 as u32, 0xe); // mwd27: the best 42 zone clubs shuffled
    let mut i = 0i32;
    while i < limit() {
        let c = read_usize(G_CLUB_TABLE) + i as usize * club::SIZE;
        if read_usize(c + club::DIVISION) == first {
            k += 1;
            set_cup_entry(this, (0x20 - k as i32) as usize, c, if c == holder { 1 } else { 2 });
        }
        i += 1;
    }
    covered!("mwd_27 russian cup entrants");
    for &c in zone_clubs.iter().take(0xe) {
        if k >= 0x20 {
            break;
        }
        k += 1;
        set_cup_entry(this, (0x20 - k as i32) as usize, c, 2);
    }
    let mut i = 0i32;
    while i < limit() {
        let c = read_usize(G_CLUB_TABLE) + i as usize * club::SIZE;
        if read_usize(c + club::DIVISION) == premier {
            set_cup_entry(this, k as usize, c, 1);
            k = k.wrapping_add(1);
        }
        i += 1;
    }
    (k as i16 >= this.team_count()) as i32
}

/// Comparer for `tur_cup_collect_entrants` (side by side, every entry compared).
pub(crate) unsafe extern "thiscall" fn compare_tur_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    compare_entrants_with_seeds("tur_cup_collect_entrants", Cup(this), || o(this), || tur_cup_collect_entrants(this), |_| true)
}

/// Comparer for `rus_cup_select_entrants` (side by side with the random generator restored, every entry compared).
pub(crate) unsafe extern "thiscall" fn compare_rus_cup_select_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    compare_entrants_with_seeds("rus_cup_select_entrants", Cup(this), || o(this), || rus_cup_select_entrants(this), |_| true)
}
