// --- sx09c_1: the Fine manager: its daily pass, its construction and its release (Fine.cpp) ---

/// `Fine::load` (0x5a5180, Fine.cpp; thiscall(fine, save), ret 4, the answer in al): reads fine.dat (sx09c_2).
const FN_FINE_LOAD: usize = 0x5a5180;

/// Drops the first expired fine and, when a coach feedback is due, posts it.
///
/// `Fine::daily_update()`: the fine list (the 14-byte records at `[this]`) from its head: the first of the first
/// `count` records whose date (+6) today is past (`exe::date_operator_3e` on `G_CURRENT_DATE`) leaves the list
/// (`exe::llist_remove`); a record walk that runs out of nodes ends the whole pass. Then the coach feedback list (the
/// 17-byte records at `[this + 4]`) from its head: the first of the first `count` records whose date (+9) today is
/// past: when the person of +0 (the club's assistant manager when the fine was recorded) joined his club more than 30
/// days ago (`exe::date_days_since` on `date_joined_club()`) and his injury status (`exe::injury_get_player_injury_status`
/// on `G_INJURY`) is not 2, `post_coach_fine_feedback_news(+0, the player of +4, the outcome byte +8)`; then the
/// feedback list removes the node whose data equals, byte for byte, the original's stale stack slot (see below), and
/// the pass ends.
///
/// Defect of the original: the removal of the feedback record passes a stack slot that still holds the last fine the
/// first walk looked at (freed already when it was the expired one), or the Fine object itself when the fine list was
/// empty, instead of the feedback record. `LLIST::remove` compares node data byte for byte, so the feedback record
/// stays and its news is posted again every day while the conditions hold. The copy passes the same slot.
/// Original: `Fine::daily_update` (0x5a5820, Fine.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn fine_daily_update(this: usize) {
    let fines = Llist(read_usize(this));
    fines.set_cursor(fines.head());
    let mut last_fine = this; // the original's stack slot: `push ecx` (this), then each fine the walk reads
    if fines.count() > 0 {
        let mut k = 0i32;
        loop {
            let fine = exe::llist_get_next(fines);
            last_fine = fine;
            if fine == 0 {
                return;
            }
            if exe::date_operator_3e(G_CURRENT_DATE, read_u32(fine + 6), read_u32(fine + 0xa)) != 0 { // +6: the fine's end date
                covered!("sx09c_1 expired fine dropped");
                exe::llist_remove(fines, fine);
                break;
            }
            k += 1;
            if k >= fines.count() as i32 {
                break;
            }
        }
    }
    let feedback = Llist(read_usize(this + 4));
    feedback.set_cursor(feedback.head());
    if feedback.count() <= 0 {
        return;
    }
    let mut k = 0i32;
    loop {
        let entry = exe::llist_get_next(feedback);
        if entry == 0 {
            return;
        }
        if exe::date_operator_3e(G_CURRENT_DATE, read_u32(entry + 9), read_u32(entry + 0xd)) != 0 { // +9: the report date
            let first = staff_at(read_i32(entry) as isize); // +0: the club's assistant manager at the fine
            let player = staff_at(read_i32(entry + 4) as isize); // +4: the fined player
            if exe::date_days_since(G_CURRENT_DATE, first.date_joined_club() as u32, first.date_joined_club_leap_year() as u32) > 0x1e
                && exe::injury_get_player_injury_status(read_usize(G_INJURY), first) != 2 { // sx09c_1: the report is posted
                covered!("sx09c_1 coach feedback posted");
                post_coach_fine_feedback_news(first.addr(), player.addr(), read_u8(entry + 8) as u32);
            }
            exe::llist_remove(feedback, last_fine); // the original's defect: the stale slot, not `entry`
            return;
        }
        k += 1;
        if k >= feedback.count() as i32 {
            return;
        }
    }
}

/// Comparer for `fine_daily_update`: it frees list nodes and posts news, so only the original runs; the plan is
/// checked afterwards: the fine list loses one node exactly when the walk finds an expired fine (counted before), and
/// the feedback list keeps its nodes when none is due.
pub(crate) unsafe extern "thiscall" fn compare_fine_daily_update(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let due = |list: Llist, date: usize| {
        let mut node = list.head();
        let mut k = 0i32;
        while node != 0 && k < list.count() as i32 {
            let d = read_usize(node + llist_node::DATA);
            if exe::date_operator_3e(G_CURRENT_DATE, read_u32(d + date), read_u32(d + date + 4)) != 0 {
                return true;
            }
            node = read_usize(node + llist_node::NEXT);
            k += 1;
        }
        false
    };
    let (fines, feedback) = (Llist(read_usize(this)), Llist(read_usize(this + 4)));
    let (n1, n2) = (fines.count(), feedback.count());
    let (expired, report) = (due(fines, 6), due(feedback, 9));
    o(this);
    let ok = fines.count() == n1 - expired as i16 && (report || feedback.count() == n2);
    SCOREBOARD.record(ok, || format!("fine_daily_update: fines {} -> {} (expired {}), feedback {} -> {} (due {})",
        n1, fines.count(), expired, n2, feedback.count(), report));
}

/// Builds the Fine manager: the fine list and the coach feedback list, from the save's fine.dat (`Fine::load`) or new
/// and empty, and the per-club fine count table.
///
/// `Fine::Fine(save)`: both list pointers null; with a save, `Fine::load` (`FN_FINE_LOAD`); when it answers, nothing else (it built the
/// lists and the table). Otherwise (no save, or the load failed): with a save, the lists the load may have left are
/// destroyed (`LLIST::~LLIST` and `operator delete`, the pointers kept), and in both cases two new empty lists
/// (`operator new(0xe)` and `LLIST::LLIST`, null when the allocation fails) and the fine count table: one zeroed byte
/// per club (`calloc(G_CLUB_COUNT, 1)`) in +8. Returns the object.
/// Original: `Fine::Fine` (0x5a4fc0, Fine.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_fine(this: usize, save: usize) -> usize {
    write_usize(this, 0);
    write_usize(this + 4, 0);
    if save != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_FINE_LOAD);
        if load(this, save) != 0 {
            return this;
        }
        covered!("sx09c_1 fine.dat not loaded");
        for field in [0usize, 4] {
            let list = read_usize(this + field);
            if list != 0 {
                exe::drop_llist(list);
                exe::crt_operator_delete(list);
            }
        }
    }
    write_usize(this, club_history_new_list()); // the fine list
    write_usize(this + 4, club_history_new_list()); // the coach feedback list
    write_usize(this + 8, exe::crt_calloc(read_u32(G_CLUB_COUNT), 1)); // sx09c_1: the per-club fine counts
    this
}

/// Comparer for `new_fine`: it allocates (and, with a save, reads the save item), so it cannot run twice; only the
/// original runs. The copy is proven by the new career (Rust mode).
pub(crate) unsafe extern "thiscall" fn compare_new_fine(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(this, save)
}

/// Frees the Fine manager's data: each list present (`LLIST::~LLIST` and `operator delete`) and the fine count table
/// (`free`), each pointer set to null.
/// Original: `Fine::free_data` (0x5a5120, Fine.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn fine_free_data(this: usize) {
    for field in [0usize, 4] {
        let list = read_usize(this + field);
        if list != 0 {
            exe::drop_llist(list);
            exe::crt_operator_delete(list);
            write_usize(this + field, 0);
        }
    }
    let counts = read_usize(this + 8);
    if counts != 0 {
        exe::crt_free(counts);
        write_usize(this + 8, 0);
    }
}

/// Comparer for `fine_free_data`: it frees, so it cannot run twice; only the original runs, and the three pointers must
/// end null.
pub(crate) unsafe extern "thiscall" fn compare_fine_free_data(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    SCOREBOARD.record(read_usize(this) == 0 && read_usize(this + 4) == 0 && read_usize(this + 8) == 0,
        || format!("fine_free_data({:#x}): pointers left {:#x} {:#x} {:#x}", this, read_usize(this), read_usize(this + 4), read_usize(this + 8)));
}
