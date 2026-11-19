// ---------- batch %%LEVA%%: the Polish and Greek cup entrants (pol_cup.cpp, pol_lge_cup.cpp, gre_cup.cpp) ----------

/// Rule: the Polish Cup takes 52 clubs: the Second Division clubs placed 13th to 20th last season, then those placed up
/// to 12th (class 2), then lowest-division clubs (the promoted ones first, then the others sorted by strength with the
/// best ones shuffled) until 36 are in (class 3), then the First Division clubs of last season (class 1); returns 1.
///
/// `team_count` = 52; the list (`exe::crt_malloc` of 0x138 into `cup::TEAM_TABLE`; out of memory: the box of
/// pol_cup.cpp at line 0x142 and −1). Over the first [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`] clubs (the limit read
/// once for the first two passes): a local list of at most 70, first the clubs whose `club::LAST_DIVISION` has the id of
/// `KEY_CLUB_COMP_POLISH_LOWER_DIVISION` and whose division is set and another one (promoted), then those whose division
/// is that same last division (still there); `exe::comp_util_sort_and_shuffle_top_clubs` on the second part only
/// (list + first, n − first, 36 − first). Then, the limit read again each step: the clubs whose last division has the id of
/// `KEY_CLUB_COMP_POLISH_SECOND_DIVISION` and whose `club::LAST_POSITION` (a signed byte) is 13..20, then ≤ 12, each at
/// the next entry with class 2. While fewer than 36 are in, the local list's clubs (up to its count) at the next entries
/// with class 3. Then the clubs whose last division has the id of `KEY_CLUB_COMP_POLISH_FIRST_DIVISION`, while the
/// entry offset is below 0x138, with class 1 (the count is not kept). Status 0 for all. Returns 1.
/// Original: `PolCup::collect_entrants` (0x7c8550, pol_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let mut lower = [0usize; 0x46];
    let mut n: i16 = 0;
    let mut k = 0i32;
    this.set_team_count(0x34);
    let l = exe::crt_malloc(0x138);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0xa78564, 0x142);
        return -1;
    }
    let limit = || (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let count = limit();
    let mut first = 0i32;
    for promoted in [true, false] {
        if !promoted {
            first = n as i32;
        }
        for i in 0..count.max(0) as usize {
            let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
            let (last, d) = (read_usize(c + club::LAST_DIVISION), read_usize(c + club::DIVISION));
            let fits = if promoted { d != 0 && d != last } else { d == last };
            if last != 0 && n < 0x46 && read_usize(last) == read_usize(KEY_CLUB_COMP_POLISH_LOWER_DIVISION) && fits {
                lower[n as usize] = c;
                n += 1;
            }
        }
    }
    covered!("mwd_30 polish cup entrants");
    exe::comp_util_sort_and_shuffle_top_clubs(lower.as_mut_ptr() as usize + 4 * first as usize, (n as i32 - first) as u32, (0x24 - first) as u32); // mwd30: the lowest division clubs shuffled
    for upper in [false, true] {
        let mut i = 0i32;
        while i < limit() {
            let c = read_usize(G_CLUB_TABLE) + i as usize * club::SIZE;
            let last = read_usize(c + club::LAST_DIVISION);
            if last != 0 && read_usize(last) == read_usize(KEY_CLUB_COMP_POLISH_SECOND_DIVISION) {
                let pos = read_i8(c + club::LAST_POSITION);
                if if upper { pos <= 0xc } else { (0xd..0x15).contains(&pos) } {
                    set_cup_entry(this, k as usize, c, 2);
                    k += 1;
                }
            }
            i += 1;
        }
    }
    if k < 0x24 {
        let mut j = 0i32;
        while j < n as i32 {
            set_cup_entry(this, k as usize, lower[j as usize], 3); // mwd30: the lowest division clubs class 3
            k += 1;
            j += 1;
            if k * 6 >= 0xd8 {
                break;
            }
        }
    }
    let mut e = k * 6;
    let mut i = 0i32;
    while i < limit() {
        let c = read_usize(G_CLUB_TABLE) + i as usize * club::SIZE;
        let last = read_usize(c + club::LAST_DIVISION);
        if last != 0 && e < 0x138 && read_usize(last) == read_usize(KEY_CLUB_COMP_POLISH_FIRST_DIVISION) {
            set_cup_entry(this, (e / 6) as usize, c, 1);
            e += 6;
        }
        i += 1;
    }
    1
}

/// Rule: the Polish League Cup takes 36 clubs in 2001 and 34 afterwards, all class 0: the Second Division clubs that
/// came up from below (or have no last division), then the other Second Division clubs, then the First Division clubs
/// placed below 7th last season, then those placed up to 7th, each in database order until the list is full; returns
/// 1 when it is full.
///
/// `team_count` = 36 when the cup's year (+0x40) is 2001, else 34; the list (`exe::crt_malloc` of 6 × team_count; out
/// of memory: the box of pol_lge_cup.cpp at line 0x168 and −1). Four passes over the clubs (a 16-bit index up to
/// [`G_CLUB_COUNT`]), each adding at the next entry while the count is below `team_count`: division id of
/// `KEY_CLUB_COMP_POLISH_SECOND_DIVISION` with no `club::LAST_DIVISION` or one whose id is neither the First nor the
/// Second Division's; the same division with a last division that is one of them; division id of
/// `KEY_CLUB_COMP_POLISH_FIRST_DIVISION` with `club::LAST_POSITION` (a signed byte) above 7; and at most 7. Class and
/// status 0. Returns count == `team_count`.
/// Original: `PolLgeCup::collect_entrants` (0x7cad30, pol_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_lge_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let mut k = 0i32;
    this.set_team_count(if read_i16(this.addr() + 0x40) == 0x7d1 { 0x24 } else { 0x22 });
    let l = exe::crt_malloc((this.team_count() as i32 * 6) as u32);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0xa785d4, 0x168);
        return -1;
    }
    covered!("mwd_30 polish league cup entrants");
    for pass in 0..4 {
        let mut club = read_usize(G_CLUB_TABLE);
        let mut i = 0i16;
        while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
            let d = read_usize(club + club::DIVISION);
            let fits = if pass < 2 {
                d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_POLISH_SECOND_DIVISION) && k < this.team_count() as i32 && {
                    let last = read_usize(club + club::LAST_DIVISION);
                    let upper = last != 0 && (read_usize(last) == read_usize(KEY_CLUB_COMP_POLISH_FIRST_DIVISION) || read_usize(last) == read_usize(KEY_CLUB_COMP_POLISH_SECOND_DIVISION));
                    if pass == 0 { !upper } else { upper }
                }
            } else {
                d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_POLISH_FIRST_DIVISION) && k < this.team_count() as i32 && {
                    let pos = read_i8(club + club::LAST_POSITION);
                    if pass == 2 { pos > 7 } else { pos < 8 } // mwd30: the First Division, the lower half first
                }
            };
            if fits {
                set_cup_entry(this, k as usize, club, 0);
                k += 1;
            }
            i = i.wrapping_add(1);
            club = club.wrapping_add(club::SIZE);
        }
    }
    (k == this.team_count() as i32) as i32
}

/// Rule: the Greek Cup takes 44 clubs: 16 Third Division clubs (sorted by strength, the best 48 shuffled), then the
/// Second Division clubs and the First Division clubs in database order until the list is full; short of 44, an error
/// box. Returns 1.
///
/// The entrant list (+0xf2 = 44; +0xee = `exe::crt_calloc`(4, 44), cdecl; out of memory: the box of gre_cup.cpp at line
/// 0x2f2 and 0). Over the clubs (a 16-bit index up to [`G_CLUB_COUNT`] read once): the division id of
/// `KEY_CLUB_COMP_GREEK_LOWER_DIVISION` into a local list of at most 100 (a signed byte count);
/// `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 16) and its first 16 into the entrant list (the original copies
/// 16 whatever the count: with fewer it reads stack garbage; the copy reads zeros). Then the clubs of
/// `KEY_CLUB_COMP_GREEK_SECOND_DIVISION` and of `KEY_CLUB_COMP_GREEK_PREMIER_DIVISION`, each while the count is below
/// +0xf2. Count ≠ +0xf2: the box at line 0x322. Returns 1.
/// Original: `GreCup::collect_entrants` (0x5e9a80, gre_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_cup_collect_entrants(this: usize) -> i32 {
    let mut third = [0usize; 100];
    let mut n: i8 = 0;
    write_u32(this + 0xf2, 0x2c);
    let list = exe::crt_calloc(4, 0x2c);
    write_usize(this + 0xee, list);
    if list == 0 {
        show_error_at(0x9cdab0, 0x2f2);
        return 0;
    }
    let count = read_usize(G_CLUB_COUNT) as i32;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i16;
    while (i as i32) < count {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GREEK_LOWER_DIVISION) && n < 0x64 {
            third[n as usize] = club;
            n += 1;
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    covered!("mwd_30 greek cup entrants");
    exe::comp_util_sort_and_shuffle_top_clubs(third.as_mut_ptr() as usize, n as i16 as u16 as u32, 0x10); // mwd30: the best 48 third division clubs shuffled
    for k in 0..0x10usize {
        write_usize(read_usize(this + 0xee) + 4 * k, third[k]);
    }
    let mut k = 0x10i32;
    for g in [KEY_CLUB_COMP_GREEK_SECOND_DIVISION, KEY_CLUB_COMP_GREEK_PREMIER_DIVISION] {
        let mut club = read_usize(G_CLUB_TABLE);
        let mut i = 0i16;
        while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
            let d = read_usize(club + club::DIVISION);
            if d != 0 && read_usize(d) == read_usize(g) && k < read_i32(this + 0xf2) {
                k += 1;
                write_usize(read_usize(this + 0xee) + 4 * (k - 1) as usize, club);
            }
            i = i.wrapping_add(1);
            club = club.wrapping_add(club::SIZE);
        }
    }
    if k != read_i32(this + 0xf2) {
        show_error_at(0x9cdab0, 0x322);
    }
    1
}

/// Comparer for `pol_cup_collect_entrants` (side by side with the random generator restored, every entry compared).
pub(crate) unsafe extern "thiscall" fn compare_pol_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    compare_entrants_with_seeds("pol_cup_collect_entrants", Cup(this), || o(this), || pol_cup_collect_entrants(this), |_| true)
}

/// Comparer for `pol_lge_cup_collect_entrants` (side by side, every entry compared).
pub(crate) unsafe extern "thiscall" fn compare_pol_lge_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    compare_entrants_with_seeds("pol_lge_cup_collect_entrants", Cup(this), || o(this), || pol_lge_cup_collect_entrants(this), |_| true)
}

/// Comparer for `gre_cup_collect_entrants`: side by side with the random generator restored; +0xf2, the return and the
/// 44 entrant pointers compared; the copy's list is freed and the original's kept.
pub(crate) unsafe extern "thiscall" fn compare_gre_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I2%%);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this + 0xf2, 4usize), (G_MOUSE_LEFT_BUTTON_DOWN, 4), crate::rng::STATE_REGIONS[0], crate::rng::STATE_REGIONS[1]];
    let (a, b, same, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        lo.set(read_usize(this + 0xee));
        r
    }, || {
        let r = gre_cup_collect_entrants(this);
        lr.set(read_usize(this + 0xee));
        write_usize(this + 0xee, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let list = if lo != 0 && lr != 0 { (0..0x2c).all(|k| read_usize(lo + 4 * k) == read_usize(lr + 4 * k)) } else { lo == 0 && lr == 0 };
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(same && a == b && list, || format!("gre_cup_collect_entrants({:#x}) returned {} x {} memory {} list {}", this, a, b, same, list));
    a
}
