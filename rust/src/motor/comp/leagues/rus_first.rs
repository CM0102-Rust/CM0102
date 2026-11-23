use super::*;

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
        let c = Club(list[k as usize]);
        if k < this.club_count() {
            exe::league_init_table_row(this, k as u16 as u32, c, 0, 0);
        } else {
            write_usize(c.addr() + club::DIVISION, comp_record_of(KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_SOUTH));
        }
        k += 1;
    }
    1
}

/// Comparer for `rus_first_create_table`: only the original runs (it draws); the table must hold 18 clubs of the
/// division when there are at least 18 (any order), and the return 1; otherwise 0.
pub(crate) unsafe extern "thiscall" fn compare_rus_first_create_table(this: usize) -> u32 {
    let l = League(this);
    let plan = division_clubs_plan(0x12, 0x57, KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION);
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_RUS_FIRST_CREATE_TABLE)(this);
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

/// Rule: the Russian First Division turnover (called by the Premier League's): the season reopened (+0x4c = 0), the
/// table, the dates and the stage objects freed, the statistics archived, the next year, the rules, the table, the
/// fixtures, the order and the predicted positions (`rebuild_season`, lines 0x168, 0x16f, 0x175, 0x17c of
/// rus_first.cpp); then the board expectations and the officials (`set_expectations_and_renew_officials`).
/// Original: `RusFirst::start_new_season` (0x7ea8c0, rus_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    covered!("mwd_23 russian first division turnover");
    if !rebuild_season(this, 0x7ea6c0, 0xa7ac8c, [0x168, 0x16f, 0x175, 0x17c], 1) {
        return 0;
    }
    set_expectations_and_renew_officials(this)
}

/// Comparer for `rus_first_start_new_season` (only the original; the year went up by one).
pub(crate) unsafe extern "thiscall" fn compare_rus_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_RUS_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("rus_first_start_new_season", League(this), || o(this))
}

/// Rule: the Russian First Division at career creation: discipline rule 0x27 and the division constructor template
/// (`construct_division_league`).
/// Original: `RusFirst::RusFirst` (0x7e9f30, rus_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_rus_first(this: usize, year: u32, record: usize) -> usize {
    covered!("mwd_11 russian first division");
    construct_division_league(League(this), year, record, &RUSSIAN_FIRST_DIVISION)
}

/// Comparer for `construct_rus_first`.
pub(crate) unsafe extern "thiscall" fn compare_construct_rus_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_RUS_FIRST);
    compare_division_league_ctor("construct_rus_first", League(this), year, record, &RUSSIAN_FIRST_DIVISION, || o(this, year, record))
}

/// Rule: the Russian First Division: 2 legs, 3 points a win and 1 a draw; tie-breaks wins, head to head, goal
/// difference, goals for; 2 go up and 3 go down (`rus_first_rules`).
/// Original: `RusFirst::set_up_rules` (0x7ea810, rus_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_first_set_up_rules(this: usize) -> u32 {
    covered!("mwd_13 russian first division rules");
    eng_prm_apply_league_rules(League(this), &rus_first_rules())
}

/// Comparer for `rus_first_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_rus_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_RUS_FIRST_SET_UP_RULES);
    eng_prm_compare_league_rules("rus_first_set_up_rules", League(this), rus_first_rules(), || o(this), || rus_first_set_up_rules(this))
}

/// The rules of the Russian First Division: 2 legs, 3 points a win and 1 a draw; tie-breaks 3, 4, 1, 2 (wins, head to
/// head, goal difference, goals for); 2 go up (+0xbe), 3 go down (+0xc1); the Premier League above
/// (`KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION`), nothing below; +0x52 = 2; 7 substitutes, 3 substitutions; +0xa7 = −1;
/// no rule bits set.
pub(crate) unsafe fn rus_first_rules() -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 3), (0xc6, 1, 4), (0xc7, 1, 1),
        (0xc8, 1, 2), (0xbe, 1, 2), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 3), (0x4a, 1, 3), (0x1c, 4, read_usize(KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION) as u32),
        (0x52, 1, 2), (0x20, 4, 0xffff_ffff), (0x49, 1, 7), (0xa7, 2, 0xffff)]
}

/// The Russian First Division: discipline rule 0x27 (the Russian one).
pub(crate) const RUSSIAN_FIRST_DIVISION: LeagueDivision = LeagueDivision {
    vtabela: 0x96f1c4,
    discipline: 0x27,
    rules: 0x7ea810,
    table: 0x7ea6c0,
    file: 0xa7ac8c,
    rows: [0x4a, 0x51, 0x58, 0x5f, 0x66],
    low: false,
};
