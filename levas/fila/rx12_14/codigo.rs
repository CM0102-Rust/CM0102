// ---------- rx12_14: the Serie A and Serie B tie-break play-off: the next stage and the Serie A play-off (ita_ser_a.cpp, ita_ser_b.cpp) ----------

/// The next stage of an Italian league (Serie A or B, as `state_league_advance_stage`): past the last stage the error
/// box of `ita_ser_a.cpp` line 0x328; otherwise the current stage goes up by one and, on reaching 0 (the league just
/// closed with clubs level on points across a band boundary), the tie-break play-off is built
/// (`ItaSerA::create_tiebreak_playoffs`, called with the league's ecx).
/// Original: `ItaSerA::advance_stage` (0x6419e0, ita_ser_a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_a_advance_stage(this: usize) {
    covered!("rx12_14 serie a next stage");
    state_league_advance_stage(League(this), 0x9ea2a8, 0x328, 0x641a70);
}

/// Comparer for `ita_ser_a_advance_stage`: side by side on the current stage when no play-off is built; else only the
/// original.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_a_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    ita_ser_c1a_compare_advance_stage("ita_ser_a_advance_stage", League(this), || o(this), || ita_ser_a_advance_stage(this))
}

/// Whether rows `i` and `i + 1` of an Italian league table are a tie-break pair: level on points
/// (`league_table_row::POINTS`, as words) and in different bands (`zones`, see `serie_a_zones`/`serie_b_zones`).
unsafe fn is_italian_tiebreak_pair(this: League, zones: &[u8; 64], i: i16) -> bool {
    let row = this.table() + (i as i32 as usize).wrapping_mul(league_table_row::SIZE);
    (i as i32) < this.club_count() as i32 - 1
        && read_u16(row + league_table_row::POINTS) == read_u16(row + league_table_row::SIZE + league_table_row::POINTS)
        && zones[(i as usize).min(63)] != zones[(i as usize + 1).min(63)] // rx12_14: only across a band boundary
}

/// The body of the Serie A and Serie B tie-break play-offs (`create_tiebreak_playoffs`, thiscall, ret 0).
///
/// The table scan (16-bit row index i, while i < `club_count`): when rows i and i + 1 are a pair
/// (`is_italian_tiebreak_pair`), both clubs go into the list (a local array), row i's worst possible position
/// (`league_table_row::WORST_POSSIBLE_POSITION`) becomes i + 2 and row i + 1's best (`BEST_POSSIBLE_POSITION`) i + 1,
/// both statuses 1, and the scan skips row i + 1; otherwise, when i > 0 and row i is level on points with row i − 1, its
/// worst and best possible positions become i + 1. The tie count (the list count / 2, truncated) goes into
/// `italian_league::TIEBREAK_TIE_COUNT`.
///
/// No tie: no extra stage (`league::STAGE_COUNT` = 0, current stage −1), no tie-break matches
/// (`league::TIEBREAK_MATCHES` = 0) and, when `recompute` (Serie A only), the positions and statuses recomputed (virtual
/// `league_vtable::UPDATE_POSITIONS_AND_STATUS`). Otherwise: the stage 0 round (virtual `comp_vtable::CREATE_SCHEDULE`
/// (0, &rounds, &extra, 0): the play-off match of `italian_schedule`), the stage (`exe::crt_operator_new`(0xb2) and
/// Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`(league, count, list, rounds, competition, round, year, 0, 1, extra,
/// 0x14, 0, 0, 0, 0), thiscall, ret 0x3c; the rounds, extra and year pushed as dwords with stack garbage above the
/// words, which the constructor does not read) into the first slot of the stage vector; no stage: the error box of
/// `file` at `row` and return; otherwise the round freed, the first round set up (`exe::cup_set_up_round`(stage, 0)),
/// current stage 0 and the first tie-break code 6 (`league::TIEBREAKS`, sort only).
pub(crate) unsafe fn create_italian_tiebreak_playoff(this: League, zones: &[u8; 64], file: usize, row: u32, recompute: bool) {
    let mut list = [0usize; 0x40];
    let mut count: u32 = 0;
    let n = this.club_count();
    let mut i: i16 = 0;
    while i < n {
        let r = this.table() + (i as i32 as usize).wrapping_mul(league_table_row::SIZE);
        let next = r + league_table_row::SIZE;
        if is_italian_tiebreak_pair(this, zones, i) {
            list[(count as u16 as i16 as usize) & 0x3f] = read_usize(r);
            list[((count + 1) as u16 as i16 as usize) & 0x3f] = read_usize(next);
            count += 2;
            write_u8(r + league_table_row::WORST_POSSIBLE_POSITION, (i as u8).wrapping_add(2));
            write_u8(next + league_table_row::BEST_POSSIBLE_POSITION, (i as u8).wrapping_add(1));
            write_u8(r + league_table_row::STATUS, 1); // rx12_14: the pair waits for the play-off
            write_u8(next + league_table_row::STATUS, 1);
            i += 1;
        } else if i > 0 && read_u16(r + league_table_row::POINTS) == read_u16(r - league_table_row::SIZE + league_table_row::POINTS) {
            write_u8(r + league_table_row::WORST_POSSIBLE_POSITION, (i as u8).wrapping_add(1));
            write_u8(r + league_table_row::BEST_POSSIBLE_POSITION, (i as u8).wrapping_add(1));
        }
        i += 1;
    }
    let ties = ((count as u16 as i16 as i32) / 2) as u8;
    write_u8(this.addr() + italian_league::TIEBREAK_TIE_COUNT, ties);
    if ties == 0 {
        this.set_stage_count(0);
        this.set_current_stage(0xffff_ffff);
        this.set_tiebreak_matches(0);
        if recompute {
            bra_nat_first_call_virtual_no_args(this.addr(), league_vtable::UPDATE_POSITIONS_AND_STATUS);
        }
        return;
    }
    covered!("rx12_14 italian tie-break play-off");
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let t = schedule(this.addr(), 0, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, *const usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), count, list.as_ptr(), rounds as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(this.stages(), stage);
    if read_usize(this.stages()) == 0 {
        show_error_at(file, row);
        return;
    }
    exe::crt_free(t);
    exe::cup_set_up_round(read_usize(this.stages()), 0);
    this.set_current_stage(0);
    write_u8(this.addr() + league::TIEBREAKS, 6);
}

/// The clubs of an Italian tie-break play-off, in the order of the stage's list (the scan of
/// `create_italian_tiebreak_playoff` without its writes).
pub(crate) unsafe fn italian_tiebreak_plan(this: League, zones: &[u8; 64]) -> Vec<usize> {
    let mut v = Vec::new();
    let mut i: i16 = 0;
    while i < this.club_count() {
        if is_italian_tiebreak_pair(this, zones, i) {
            let row = this.table() + i as usize * league_table_row::SIZE;
            v.push(read_usize(row));
            v.push(read_usize(row + league_table_row::SIZE));
            i += 1;
        }
        i += 1;
    }
    v
}

/// Comparer for the Italian tie-break play-offs: only the original runs (it builds a stage); the tie count must be
/// half the planned clubs (`italian_tiebreak_plan`) and, with ties, the stage's team list must hold the planned clubs.
pub(crate) unsafe fn compare_italian_tiebreak_playoff(name: &str, this: League, zones: &[u8; 64], original: impl FnOnce()) {
    let plan = if this.table() != 0 { italian_tiebreak_plan(this, zones) } else { Vec::new() };
    original();
    let ties = read_u8(this.addr() + italian_league::TIEBREAK_TIE_COUNT);
    let mut ok = ties as usize == plan.len() / 2;
    let mut found: Vec<usize> = Vec::new();
    if ok && ties != 0 {
        let stage = if this.stages() != 0 { Cup(read_usize(this.stages())) } else { Cup(0) };
        ok = !stage.is_null() && stage.team_table() != 0;
        if ok {
            found = (0..plan.len()).map(|k| read_usize(stage.team_table() + 6 * k)).collect();
            let (mut a, mut b) = (plan.clone(), found.clone());
            a.sort_unstable();
            b.sort_unstable();
            ok = a == b;
        }
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}): ties {} plan {:x?} stage list {:x?}", name, this.addr(), ties, plan, found));
}

/// The Serie A tie-break play-off (spareggio): two clubs level on points in different bands (the title, Italy's
/// Champions League places, the UEFA Cup places, the next two, relegation: `serie_a_zones`) play off for the place,
/// as `create_italian_tiebreak_playoff` (the box at line 0x38e; with no tie, the positions recomputed).
/// Original: `ItaSerA::create_tiebreak_playoffs` (0x641a70, ita_ser_a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_a_create_tiebreak_playoffs(this: usize) {
    let this = League(this);
    create_italian_tiebreak_playoff(this, &serie_a_zones(this), 0x9ea2a8, 0x38e, true);
}

/// Comparer for `ita_ser_a_create_tiebreak_playoffs` (`compare_italian_tiebreak_playoff` with the Serie A bands).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_a_create_tiebreak_playoffs(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let league = League(this);
    compare_italian_tiebreak_playoff("ita_ser_a_create_tiebreak_playoffs", league, &serie_a_zones(league), || o(this));
}

/// The next stage of Serie B: as Serie A's (`state_league_advance_stage`), with the box of `ita_ser_b.cpp` line 0x23d
/// and the Serie B play-off (`ItaSerB::create_tiebreak_playoff_stage`).
/// Original: `ItaSerB::advance_stage` (0x646b20, ita_ser_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_b_advance_stage(this: usize) {
    covered!("rx12_14 serie b next stage");
    state_league_advance_stage(League(this), 0x9ea384, 0x23d, 0x646bb0);
}

/// Comparer for `ita_ser_b_advance_stage`: side by side on the current stage when no play-off is built; else only the
/// original.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_b_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    ita_ser_c1a_compare_advance_stage("ita_ser_b_advance_stage", League(this), || o(this), || ita_ser_b_advance_stage(this))
}
