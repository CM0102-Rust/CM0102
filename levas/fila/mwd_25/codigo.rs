// ---------- batch %%LEVA%%: the Turkish First Division table and the lowest division's promotion (tur_first.cpp) ----------

/// The five groups of the Turkish second division category B in the database (`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G1`
/// to `_G5`).
pub(crate) const TURKISH_CATEGORY_B_GROUPS: [usize; 5] = [KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G1, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G2,
    KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G3, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G4, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G5];

/// Rule: the Turkish First Division has 18 clubs, those of the top flight (`KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION`) in
/// database order; when full, no prize fund (prize mode 2) and TV pays 1,000,000 to each club. In 2001 it also moves
/// the clubs of the five category B groups into category B itself, keeping the group in the club's reserve division.
///
/// `club_count` = 18; in 2001, for the first [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`] clubs whose national division
/// (`club::DIVISION`) has the id of one of `TURKISH_CATEGORY_B_GROUPS`: `club::RESERVE_DIVISION` = that division and
/// `club::DIVISION` = the record of category B (`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B`). Then the table
/// (`fill_league_table_rows`: the box at line 0x1a4); full: +0x28 = 0, `prize_mode` = 2 and League::pay_tv_rights (push
/// 0; push 1000000) and 1; otherwise 0.
/// Original: `TurFirst::create_table` (0x8fddc0, tur_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_first_create_table(this: usize) -> u32 {
    let l = League(this);
    l.set_club_count(0x12);
    if l.year() == 0x7d1 {
        let mut club = read_usize(G_CLUB_TABLE);
        let mut i = 0i32;
        while i < (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2)) {
            let d = read_usize(club + club::DIVISION);
            if d != 0 && TURKISH_CATEGORY_B_GROUPS.iter().any(|&g| read_usize(d) == read_usize(g)) {
                covered!("mwd_25 turkish category b groups merged");
                write_usize(club + club::RESERVE_DIVISION, d);
                write_usize(club + club::DIVISION, comp_record_of(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B)); // mwd25: into category B
            }
            i += 1;
            club = club.wrapping_add(club::SIZE);
        }
    }
    if !fill_league_table_rows(l, 0x12, 0xaa1840, 0x1a4, KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION) {
        return 0;
    }
    l.set_prize_fund(0);
    l.set_prize_mode(2);
    eng_prm_pay_tv_rights(this, 0xf4240, 0); // mwd25: the Turkish top flight TV money
    1
}

/// Rule: at the Turkish turnover, the ten best clubs of the lowest division (`KEY_CLUB_COMP_TURKISH_LOWER_DIVISION`,
/// sorted by strength with the best 30 shuffled) go up into the second division category B, and the clubs of status 3
/// in category B and in its four group stages go down to the lowest division, their reserve division cleared; with
/// fewer than 10 candidates nobody moves.
///
/// A list of `G_CLUB_COUNT` pointers (`exe::crt_malloc`; out of memory: the box at line 0x27c and return) with the clubs
/// that have a nation (`club::NATION` set, not compared) and whose division is the lowest one, in database order
/// (16-bit index). Fewer than 10: the list freed and return. Otherwise `exe::comp_util_sort_and_shuffle_top_clubs`(list,
/// n, 10), the first ten moved to category B (League::move_club_to_division(this, club, its record, 1)); then for
/// category B (g = −1) and its stages g = 0..3 (`[+0xc][g]`), each row of status 3 goes down
/// (League::relegate_club_to_division(this, club, the lowest division's record, 1)) and that club's
/// `club::RESERVE_DIVISION` = 0. The list is freed.
/// Original: `tur_first_promote_lower_clubs_to_third_tier` (0x8fe3a0, tur_first.cpp, thiscall: ecx = the First Division, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_first_promote_lower_clubs_to_third_tier(this: usize) {
    let list = exe::crt_malloc((read_usize(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0xaa1840, 0x27c);
        return;
    }
    let mut n: u32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(club + club::NATION) != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_TURKISH_LOWER_DIVISION) {
            write_usize(list + (n as u16 as i16 as isize as usize).wrapping_mul(4), club);
            n = n.wrapping_add(1);
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    if (n as u16 as i16) < 10 {
        exe::crt_free(list);
        return;
    }
    covered!("mwd_25 turkish lowest division up");
    exe::comp_util_sort_and_shuffle_top_clubs(list, n, 10); // mwd25: the best thirty shuffled
    for k in 0..10 {
        exe::league_move_club_to_division(this, read_usize(list + 4 * k), comp_record_of(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B), 1);
    }
    let relegate: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x6831a0usize);
    let b = comp_by_global(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B);
    for g in -1i8..4 {
        let stage = League(if g == -1 { b } else { read_usize(read_usize(b + league::STAGES) + 4 * g as usize) });
        let mut i = 0i32;
        while i < stage.club_count() as i32 {
            let row = stage.table() + i as usize * league_table_row::SIZE;
            if read_u8(row + league_table_row::STATUS) == 3 {
                relegate(this, read_usize(row), comp_record_of(KEY_CLUB_COMP_TURKISH_LOWER_DIVISION), 1);
                write_usize(read_usize(stage.table() + i as usize * league_table_row::SIZE) + club::RESERVE_DIVISION, 0);
            }
            i += 1;
        }
    }
    exe::crt_free(list);
}

/// Comparer for `tur_first_create_table`: only the original runs (it moves clubs in 2001); the planned top-flight clubs
/// (`compare_division_table`) and, when full, prize mode 2 with no fund.
pub(crate) unsafe extern "thiscall" fn compare_tur_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let r = compare_division_table("tur_first_create_table", League(this), 0x12, KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION, || o(this));
    if r == 1 {
        let l = League(this);
        SCOREBOARD.record(l.prize_fund() == 0 && l.prize_mode() == 2, || format!("tur_first_create_table({:#x}) prize fund {} mode {}", this, l.prize_fund(), l.prize_mode()));
    }
    r
}

/// Comparer for `tur_first_promote_lower_clubs_to_third_tier`: only the original runs (it draws); with at least 10
/// candidates, the clubs of status 3 in category B and its four groups must end in the lowest division with no reserve
/// division.
pub(crate) unsafe extern "thiscall" fn compare_tur_first_promote_lower_clubs_to_third_tier(this: usize) {
    let candidates = (0..read_usize(G_CLUB_COUNT)).filter(|&i| {
        let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        let d = read_usize(c + club::DIVISION);
        d != 0 && read_usize(c + club::NATION) != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_TURKISH_LOWER_DIVISION)
    }).count();
    let mut planned: Vec<usize> = Vec::new();
    let b = comp_by_global(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B);
    if candidates >= 10 {
        for g in -1i32..4 {
            let s = League(if g == -1 { b } else { read_usize(read_usize(b + league::STAGES) + 4 * g as usize) });
            if s.addr() != 0 && s.table() != 0 {
                for i in 0..s.club_count() as usize {
                    let row = s.table() + i * league_table_row::SIZE;
                    if read_u8(row + league_table_row::STATUS) == 3 {
                        planned.push(read_usize(row));
                    }
                }
            }
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let lower = comp_record_of(KEY_CLUB_COMP_TURKISH_LOWER_DIVISION);
    let wrong: Vec<usize> = planned.iter().copied().filter(|&c| read_usize(c + club::DIVISION) != lower || read_usize(c + club::RESERVE_DIVISION) != 0).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("tur_first_promote_lower_clubs_to_third_tier({:#x}): relegated clubs not in the lowest division {:x?}", this, wrong));
}
