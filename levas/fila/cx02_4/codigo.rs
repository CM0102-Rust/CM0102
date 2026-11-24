// ---------- cx02_4: the round set-up of the League Cup and the Charity Shield, and the Auto Windscreens Shield progress (fra_cup.cpp, eng_fa_trophy.cpp, eng_auto_cup.cpp) ----------

/// The body of the cups' `set_up_rounds` (virtual `comp_vtable::SET_UP_RULES`) that only set the prize fund: the prize
/// fund (`cup::PRIZE_FUND`) first, then the round table of stage −1 built through the class's own virtual
/// `comp_vtable::CREATE_SCHEDULE` into `cup::ROUND_TABLE` (`build_cup_round_table_by_vtable`; the table is not
/// checked). The bench is not touched. Returns 1.
pub(crate) unsafe fn set_up_prize_rounds(this: Cup, prize: i32) -> i32 {
    this.set_prize_fund(prize);
    build_cup_round_table_by_vtable(this);
    1
}

/// Comparer of the `set_up_rounds` that build the round table: side by side over the prize fund, the stage label and
/// round count (the two words the virtual writes) and the bench bytes, the returns and the bytes the round tables
/// write (`eng_cc_cup_written_round_bytes`); the copy's table is freed and the original's kept.
pub(crate) unsafe fn compare_set_up_prize_rounds(name: &str, this: Cup, original: impl FnOnce() -> i32, copy: impl FnOnce() -> i32) -> i32 {
    let (to, tr) = (core::cell::Cell::new(0usize), core::cell::Cell::new(0usize));
    let before = this.round_table();
    let regions = [(this.addr() + cup::PRIZE_FUND, 4usize), (this.addr() + cup::STAGE_LABEL, 4), (this.addr() + cup::SUBSTITUTES_NAMED, 2)];
    let (a, b, same, _, _) = crate::comparison::side_by_side(&regions, || {
        let r = original();
        to.set(this.round_table());
        this.set_round_table(before);
        r
    }, || {
        let r = copy();
        tr.set(this.round_table());
        this.set_round_table(to.get());
        r
    });
    let (to, tr) = (to.get(), tr.get());
    let n = this.round_count();
    let table = if to != 0 && tr != 0 { eng_cc_cup_written_round_bytes(to, n) == eng_cc_cup_written_round_bytes(tr, n) } else { to == 0 && tr == 0 };
    if tr != 0 && tr != to {
        exe::crt_free(tr);
    }
    SCOREBOARD.record(same && a == b && table, || format!("{}({:#x}) returned {} x {} memory {} table {}", name, this.addr(), a, b, same, table));
    a
}

/// The rounds of the cups that share this virtual (the Coupe de France, the English League Cup, the German Cup, the
/// Scottish FA and League Cups, the Spanish Cup and the US Open Cup): a prize fund of 1,000,000 and the round table
/// (`set_up_prize_rounds`). Returns 1.
/// Original: `FraCup::set_up_rounds` (0x5bb000, fra_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fra_cup_set_up_rounds(this: usize) -> i32 {
    covered!("cx02_4 cup rounds with a 1,000,000 prize fund");
    set_up_prize_rounds(Cup(this), 1_000_000)
}

/// The rounds of the FA Trophy and the Charity Shield (the same virtual): a prize fund of 300,000 and the round table
/// (`set_up_prize_rounds`). Returns 1.
/// Original: `EngFaTrophy::set_up_rounds` (0x571650, eng_fa_trophy.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_fa_trophy_set_up_rounds(this: usize) -> i32 {
    covered!("cx02_4 cup rounds with a 300,000 prize fund");
    set_up_prize_rounds(Cup(this), 300_000)
}

/// Stores a club's progress codes in its Auto Windscreens Shield statistics entry (`Comp_Stats::find_club_entry`, not
/// checked): in the sections (stage, a signed byte, below 1) each code n becomes 2n − 1 (byte arithmetic), the
/// section's place on the cup-wide scale [inference]; in the final the codes as they come. The current code goes to
/// `comp_stats_club::CURRENT_POSITION`, the best to `BEST_POSITION`, the worst to `WORST_POSITION`.
/// Original: `EngAutoCup::set_club_progress` (0x56c240, eng_auto_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn eng_auto_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let entry = CompStatsClub(exe::comp_stats_find_club_entry(League(this).comp_stats(), club));
    let (mut current, mut best, mut worst) = (current as u8, best as u8, worst as u8);
    if (stage as u8 as i8) < 1 {
        covered!("cx02_4 auto windscreens section progress");
        current = (current << 1).wrapping_sub(1);
        best = (best << 1).wrapping_sub(1);
        worst = (worst << 1).wrapping_sub(1);
    }
    entry.set_best_position(best);
    entry.set_current_position(current);
    entry.set_worst_position(worst);
}

pub(crate) unsafe extern "thiscall" fn compare_fra_cup_set_up_rounds(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    compare_set_up_prize_rounds("fra_cup_set_up_rounds", Cup(this), || o(this), || fra_cup_set_up_rounds(this))
}

pub(crate) unsafe extern "thiscall" fn compare_eng_fa_trophy_set_up_rounds(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    compare_set_up_prize_rounds("eng_fa_trophy_set_up_rounds", Cup(this), || o(this), || eng_fa_trophy_set_up_rounds(this))
}

/// Comparer of the `set_club_progress` routines: side by side over the club's three progress bytes in the statistics
/// (the entry looked up before; without statistics or entry, only the original).
pub(crate) unsafe fn compare_club_progress(name: &str, this: usize, club: usize, stage: u32, original: impl FnOnce(), copy: impl FnOnce()) {
    let stats = League(this).comp_stats();
    let entry = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if entry == 0 {
        return original();
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(entry + comp_stats_club::CURRENT_POSITION, 3)], original, copy);
    SCOREBOARD.record(same, || format!("{}({:#x}, stage {}): original {:x?} copy {:x?}", name, club, stage as u8 as i8, mo, mr));
}

pub(crate) unsafe extern "thiscall" fn compare_eng_auto_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    compare_club_progress("eng_auto_cup_set_club_progress", this, club, stage, || o(this, club, stage, current, best, worst),
        || eng_auto_cup_set_club_progress(this, club, stage, current, best, worst));
}
