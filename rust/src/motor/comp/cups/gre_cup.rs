use super::*;

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

/// Comparer for `gre_cup_collect_entrants`: side by side with the random generator restored; +0xf2, the return and the
/// 44 entrant pointers compared; the copy's list is freed and the original's kept.
pub(crate) unsafe extern "thiscall" fn compare_gre_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_GRE_CUP_COLLECT_ENTRANTS);
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
