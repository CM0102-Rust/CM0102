// ---------- mwb_25: the Welsh Premier Cup (wel_prm_cup.cpp): the entrants and the predicted positions ----------

/// The four clubs that enter the Welsh Premier Cup from the English league system, in the order the original adds
/// them: Cardiff City, Swansea City, Wrexham and Merthyr Tydfil (their key ids, index.cpp).
pub(crate) const WELSH_PREMIER_CUP_GUESTS: [usize; 4] = [0x9d0434, 0x9d0430, 0x9d043c, 0x9d0438];

/// The body of `wel_prm_cup_build_entrant_list`; returns (the routine's return, the entries to compare).
///
/// `ita_cup_fields::ENTRANT_CLUB_COUNT` = 12; the pool (`exe::crt_calloc`(4, 12) into `ita_cup_fields::ENTRANT_CLUBS`;
/// none: the box of line 0x2b3 and 0). The holder of the Welsh Cup (`exe::comp_util_get_latest_placed_club`(the record
/// of [`KEY_CLUB_COMP_WELSH_CUP`], 0), cdecl) takes entry 0 when there is one; the cut-off place is 8 when the holder
/// finished below ninth in its league last season (`club::LAST_POSITION` < 9, a signed byte), otherwise (or with no
/// holder) 7. Then, while fewer than eight are in, three passes over the clubs (`G_CLUB_COUNT`, a signed word index, in
/// table order), the holder left out: the clubs whose `club::LAST_DIVISION` is the League of Wales
/// ([`KEY_CLUB_COMP_WELSH_PREMIER_DIVISION`]) with `club::LAST_POSITION` up to the cut-off, then those placed below it,
/// then the clubs whose `club::DIVISION` is the League of Wales (no check against those already in, as the original).
/// The entries so far are shuffled (`exe::comp_util_shuffle_pointer_array`(pool, n)); the next four are the clubs of
/// `WELSH_PREMIER_CUP_GUESTS` (`club_at`([key])), and entries 8 to 11 are shuffled (whatever n was). Returns 1.
pub(crate) unsafe fn build_welsh_premier_cup_entrants(this: League) -> (i32, i32) {
    let t = this.addr();
    write_i32(t + ita_cup_fields::ENTRANT_CLUB_COUNT, 0xc);
    let v = exe::crt_calloc(4, 0xc);
    write_usize(t + ita_cup_fields::ENTRANT_CLUBS, v);
    if v == 0 {
        show_error_at(0xad572c, 0x2b3);
        return (0, 0);
    }
    let holder = exe::comp_util_get_latest_placed_club(comp_record_of(KEY_CLUB_COMP_WELSH_CUP), 0);
    let cut = if holder != 0 && (Club(holder).last_position() as i8) < 9 { 8i8 } else { 7 }; // mwb25: the cut-off place
    let mut n = 0usize;
    if holder != 0 {
        write_usize(read_usize(t + ita_cup_fields::ENTRANT_CLUBS), holder);
        n = 1;
    }
    let wales = || read_usize(KEY_CLUB_COMP_WELSH_PREMIER_DIVISION);
    let in_league = |d: Competition| !d.is_null() && read_usize(d.addr()) == wales();
    let passes: [&dyn Fn(Club) -> bool; 3] = [
        &|c: Club| in_league(c.last_division()) && (c.last_position() as i8) <= cut,
        &|c: Club| in_league(c.last_division()) && (c.last_position() as i8) > cut,
        &|c: Club| in_league(c.division()),
    ];
    covered!("mwb_25 welsh premier cup entrants");
    for pass in passes.iter() {
        let mut i = 0i16;
        while (i as i32) < read_i32(G_CLUB_COUNT) && n < 8 {
            let c = club_at(i as isize);
            if pass(c) && c.addr() != holder {
                write_usize(read_usize(t + ita_cup_fields::ENTRANT_CLUBS) + 4 * n, c.addr());
                n += 1;
            }
            i = i.wrapping_add(1);
        }
    }
    exe::comp_util_shuffle_pointer_array(read_usize(t + ita_cup_fields::ENTRANT_CLUBS), n as u32);
    for (k, &key) in WELSH_PREMIER_CUP_GUESTS.iter().enumerate() {
        write_usize(read_usize(t + ita_cup_fields::ENTRANT_CLUBS) + 4 * (n + k), club_at(read_usize(key) as i32 as isize).addr()); // mwb25: the four guests
    }
    exe::comp_util_shuffle_pointer_array(read_usize(t + ita_cup_fields::ENTRANT_CLUBS) + 0x20, 4);
    (1, 0xc)
}

/// Rule: the Welsh Premier Cup takes twelve clubs: the Welsh Cup holder, then League of Wales clubs by last season's
/// place (those up to 8th, or up to 7th when the holder itself finished 9th or lower or there is no holder, then the
/// rest, then this season's League of Wales clubs) until there are eight, shuffled; and the four Welsh clubs of the
/// English league (Cardiff City, Swansea City, Wrexham and Merthyr Tydfil), shuffled among themselves.
/// Original: `WelPrmCup::build_entrant_list` (0x928860, wel_prm_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_build_entrant_list(this: usize) -> i32 {
    build_welsh_premier_cup_entrants(League(this)).0
}

/// Comparer for `wel_prm_cup_build_entrant_list` (side by side with the random generator restored:
/// `compare_select_entrants`). The holder comes from `comp_util_get_latest_placed_club` (0x4afd80), which re-sorts the
/// global competition history (`[0xae23d4]`, `[0xae237c]` records of 0x1a bytes) with the CRT's unstable qsort on every
/// call: the copy runs on the history as the original saw it, and the original's history is put back after it (as
/// `compare_ger_lge_cup_collect_entrants`).
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_build_entrant_list(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    let hist = || (read_usize(0xae23d4), read_usize(0xae237c) * 0x1a);
    let snap = |(p, n): (usize, usize)| if p == 0 { Vec::new() } else { core::slice::from_raw_parts(p as *const u8, n).to_vec() };
    let put = |p: usize, v: &[u8]| if p != 0 && !v.is_empty() { core::ptr::copy_nonoverlapping(v.as_ptr(), p as *mut u8, v.len()) };
    let before = core::cell::RefCell::new(Vec::new());
    let after = core::cell::RefCell::new(Vec::new());
    compare_select_entrants("wel_prm_cup_build_entrant_list", this, || {
        *before.borrow_mut() = snap(hist());
        let r = o(this);
        *after.borrow_mut() = snap(hist());
        r
    }, || {
        put(hist().0, &before.borrow());
        let r = build_welsh_premier_cup_entrants(League(this));
        put(hist().0, &after.borrow());
        r
    })
}

/// Rule: the Welsh Premier Cup's predicted positions (at creation and at the turnover): the twelve entrants by
/// reputation; the knockout (stage 1) gives 1 to the winner and 2 to the runner-up, then a shared place to the losers
/// of each earlier round; the rest in bands of 2; the board and press expectation of each club [inference].
///
/// `italian_cup_predicted_positions` with the knockout stage 1, bands of 2 and the boxes at lines 0x3db and 0x3e3 of
/// wel_prm_cup.cpp (read instruction by instruction against ItaCCup::set_predicted_positions: the frame and the
/// registers differ, the same steps; only the file, the lines, the stage and the band change).
/// Original: `WelPrmCup::set_predicted_positions` (0x928e00, wel_prm_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_set_predicted_positions(this: usize) {
    italian_cup_predicted_positions(this, 0xad572c, (0x3db, 0x3e3), 1, 2, "mwb_25 welsh premier cup predicted") // mwb25: the Premier Cup bands of 2
}

/// Comparer for `wel_prm_cup_set_predicted_positions` (`compare_italian_cup_predicted`, side by side).
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_italian_cup_predicted("wel_prm_cup_set_predicted_positions", this, wel_prm_cup_set_predicted_positions, || o(this));
}
