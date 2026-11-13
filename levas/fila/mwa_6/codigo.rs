// ---------- batch %%LEVA%%: the Northern Irish Cup: the round table, the entrants and the season turnover (nir_cup.cpp) ----------

/// The Northern Irish Cup (Irish Cup): five single-leg rounds with replays for 32 clubs, from January to a final in
/// May; draw bits 8 in the first three rounds [inference]; category 4; loser prizes of 250, 1,500, 3,000 and 5,500 and
/// 10,000 to the winner of the final.
pub(crate) const NORTHERN_IRISH_CUP_ROUNDS: [EngCcCupCupRound; 5] = [
    cql10r((0xa, 0xb, 0, 6), (0x14, 0, 1, 5, 1, 0), 0x32, (8, 0, 3), 4, (0x20, 0x10, 0x20, 0), (1, 1, 4), (0, 0, 0)), // mwa6: the fifth round of the Irish Cup
    cql10r((0x15, 0, 1, 6), (0x11, 1, 1, 5, 1, 0), 0x3c, (8, 0, 3), 4, (0x10, 8, 0, 0), (1, 1, 0), (0, 0, 0xfa)),
    cql10r((0x12, 1, 1, 6), (0xa, 2, 1, 5, 1, 0), 0x78, (8, 0, 3), 4, (8, 4, 0, 0), (1, 1, 0), (0, 0, 0x5dc)),
    cql10r((0xb, 2, 1, 6), (7, 3, 1, 5, 1, 1), 0x82, (0, 0x100, 3), 4, (4, 2, 0, 0), (1, 1, 0), (0, 0, 0xbb8)),
    cql10r((8, 3, 1, 6), (5, 4, 1, 5, 1, 4), 0x96, (0, 2, 3), 4, (2, 1, 0, 0), (1, 1, 0), (0, 0x2710, 0x157c)),
];

/// The body of `nir_cup_create_round_table`: the single-stage table and, in the semi-finals (round 3), three match-day
/// slots (`exe::cup_set_round_match_day`, cdecl): slot 0 on weekday 4, time 2, code 1, home side 0x40; slot 1 on
/// weekday 5, time 1, code 1, home side 0x80; slot 2 empty (−1, −1, −1, 0). The original sets the slots between the
/// dates and the fields of round 3; they do not overlap (+0x23 on).
pub(crate) unsafe fn build_northern_irish_cup_round_table(this: Cup, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let t = eng_cc_cup_create_single_stage_round_table(this, stage, rounds, extra, out, &NORTHERN_IRISH_CUP_ROUNDS, 0xa69628, 0x9c, "mwa_6 irish cup rounds");
    if t != 0 {
        exe::cup_set_round_match_day(t, 3, 0, 4, 2, 1, 0x40); // mwa6: the semi-finals on two days
        exe::cup_set_round_match_day(t, 3, 1, 5, 1, 1, 0x80);
        exe::cup_set_round_match_day(t, 3, 2, -1, -1, -1, 0);
    }
    t
}

/// Rule: the (Northern) Irish Cup has five single-leg rounds with replays for 32 clubs: the fifth round drawn on 10/12
/// and played on 20/1 of the next year (32 clubs, 16 ties; replay 4 days later), the sixth drawn 21/1 and played 17/2,
/// the quarter-finals drawn 18/2 and played 10/3, the semi-finals drawn 11/3 and played 7/4 over two days (Thursday
/// and Friday slots [inference: weekdays 4 and 5]) at a neutral ground [inference: home side code 1] and the final drawn
/// 8/4 and played on 5/5 [inference: home side code 4], each moved to its weekday; draw bits 8 in the first three
/// rounds, tie rules 0x100 (no fixture news [inference]) in the semi-finals and 2 in the final; category 4; the losers
/// get 250, 1,500, 3,000 and 5,500 from the sixth round on, the winner of the final 10,000.
///
/// The cup's virtual `comp_vtable::CREATE_SCHEDULE` (create_round_table(stage, &rounds, &extra, &out)): only for stage
/// −1; `out` (when given) = 0, `rounds` = 5, `extra` = 0; the table (`exe::crt_malloc` of 5 × 0x68; out of memory, the
/// error box at line 0x9c and 0) with `NORTHERN_IRISH_CUP_ROUNDS` and the semi-final match days
/// (`build_northern_irish_cup_round_table`). Returns the table.
/// Original: `NirCup::create_round_table` (0x78a870, nir_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn nir_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    build_northern_irish_cup_round_table(Cup(this), stage, rounds, extra, out)
}

/// Side by side like `compare_cup_round_table`, also comparing the three semi-final match-day slots (round 3, 21 bytes
/// from `cup_round::SLOT_VENUE`).
unsafe extern "thiscall" fn compare_nir_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    if stage as u8 != 0xff {
        let a = o(this, stage, rounds, extra, out);
        SCOREBOARD.record(a == 0, || format!("nir_cup_create_round_table(stage {}) returned {:#x}", stage as u8 as i8, a));
        return a;
    }
    let (mut r2, mut e2) = (*rounds, *extra);
    let mut s2 = if out.is_null() { 0 } else { *out };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, equal, _, _) = crate::comparison::side_by_side(&reg, || o(this, stage, rounds, extra, out),
        || build_northern_irish_cup_round_table(Cup(this), stage, &mut r2, &mut e2, if out.is_null() { core::ptr::null_mut() } else { &mut s2 }));
    let outs = *rounds == r2 && *extra == e2 && (out.is_null() || *out == s2);
    let slots = |t: usize| core::slice::from_raw_parts((t + 3 * cup_round::SIZE + cup_round::SLOT_VENUE) as *const u8, 21).to_vec();
    let table = if a != 0 && b != 0 { eng_cc_cup_written_round_bytes(a, *rounds) == eng_cc_cup_written_round_bytes(b, r2) && slots(a) == slots(b) } else { a == 0 && b == 0 };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(equal && outs && table, || format!("nir_cup_create_round_table(stage {}): outs {} ({} x {}) table {}", stage as u8 as i8, outs, *rounds, r2, table));
    a
}

/// The body of `nir_cup_collect_entrants`; returns (the routine's return, the entries it writes).
///
/// `cup::TEAM_COUNT` = 32; the entrant table (`exe::crt_malloc` of 0xc0 into `cup::TEAM_TABLE`; out of memory, the
/// error box at line 0x11d and −1). One pass over the clubs (`irish_cup_club_count`): a club of the Northern Irish
/// lower division (`KEY_CLUB_COMP_NORTHERN_IRELAND_LOWER_DIVISION`) goes into a local pool while it holds fewer than
/// 30; otherwise a Premier or First Division club takes the next entry from entry 12 on (club, status 0 and class 1;
/// no bound check: the 20 senior clubs fill entries 12 to 31). Then the pool is sorted and its best shuffled
/// (`exe::comp_util_sort_and_shuffle_top_clubs`(pool, size, 12)) and its first twelve take entries 0 to 11 (status 0,
/// class 2). Returns 1.
pub(crate) unsafe fn collect_northern_irish_cup_entrants(this: Cup) -> (i32, i32) {
    this.set_team_count(0x20);
    let t = exe::crt_malloc(0xc0);
    this.set_team_table(t);
    if t == 0 {
        show_error_at(0xa69628, 0x11d);
        return (-1, 0);
    }
    // simplification: the original's pool is the uninitialised stack; with fewer than twelve lower division clubs the
    // copy would take nulls where the original took leftovers (the career's division has more)
    let mut lower = [0usize; 30];
    let mut n = 0i16;
    let mut senior = 0usize;
    let mut i = 0i32;
    while i < irish_cup_club_count() {
        let c = club_at(i as isize);
        if let Some(id) = irish_club_division_id(c) {
            if n < 0x1e && id == read_usize(KEY_CLUB_COMP_NORTHERN_IRELAND_LOWER_DIVISION) {
                lower[n as usize] = c.addr();
                n += 1;
            } else if id == read_usize(KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION) || id == read_usize(KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION) {
                senior += 6;
                let e = this.team_table() + senior + 0x42;
                write_usize(e, c.addr());
                write_u8(e + 5, 0);
                write_u8(e + 4, 1); // mwa6: the senior clubs' class
            }
        }
        i += 1;
    }
    covered!("mwa_6 irish cup entrants");
    exe::comp_util_sort_and_shuffle_top_clubs(lower.as_mut_ptr() as usize, n as i32 as u32, 0xc);
    for (k, &club) in lower.iter().take(12).enumerate() { // mwa6: the twelve best of the lower division
        let e = this.team_table() + k * 6;
        write_usize(e, club);
        write_u8(e + 5, 0);
        write_u8(e + 4, 2);
    }
    (1, 12 + (senior / 6) as i32)
}

/// Rule: the (Northern) Irish Cup takes 32 clubs: twelve of the lower division (the best, shuffled among the top 30),
/// entries 0 to 11 with class 2, and the 20 clubs of the Premier and First Divisions, entries 12 to 31 with class 1.
/// Original: `NirCup::collect_entrants` (0x78ac90, nir_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_cup_collect_entrants(this: usize) -> i32 {
    collect_northern_irish_cup_entrants(Cup(this)).0
}

unsafe extern "thiscall" fn compare_nir_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    compare_irish_cup_entrants("nir_cup_collect_entrants", Cup(this), true, || o(this), || collect_northern_irish_cup_entrants(Cup(this)))
}

/// Rule: at the season turn the (Northern) Irish Cup is rebuilt: last edition's tables are freed, the statistics
/// archived, the year goes up by one and the entrants, the rules, the tie tree and the predicted positions are made
/// again.
///
/// The knockout cups' turnover (`start_new_cup_season`, checked instruction by instruction against BraCup's: only the
/// entrant routine, NirCup::collect_entrants 0x78ac90, the file and the error box lines 0x17e, 0x185 and 0x18c change).
/// Original: `NirCup::start_new_season` (0x78ae30, nir_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x78ac90, 0xa69628, [0x17e, 0x185, 0x18c], 1, "mwa_6 irish cup turnover")
}

unsafe extern "thiscall" fn compare_nir_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_cup_new_season("nir_cup_start_new_season", Cup(this), || o(this))
}
