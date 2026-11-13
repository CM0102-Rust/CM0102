// ---------- batch %%LEVA%%: the Croatian leagues (cro_a1.cpp, cro_a2b.cpp): the tie-break play-off ----------

/// The clubs that go to the tie-break play-off, in the order of the stage's list: scanning the table rows i from the
/// top, a row and the next one form a pair when they are level on points (`league_table_row::POINTS`) and neither has
/// the final status 3 (relegated); the pair takes both rows and the scan goes on two rows below.
pub(crate) unsafe fn croatian_tiebreak_plan(this: League) -> Vec<usize> {
    let mut v = Vec::new();
    let n = this.club_count() as i32;
    let mut i = 0i32;
    while i < n {
        let row = this.table() + i as usize * league_table_row::SIZE;
        let next = row + league_table_row::SIZE;
        if i < n - 1 && read_u16(row + league_table_row::POINTS) == read_u16(next + league_table_row::POINTS)
            && read_u8(row + league_table_row::STATUS) != 3 && read_u8(next + league_table_row::STATUS) != 3 {
            v.push(read_usize(row));
            v.push(read_usize(next));
            i += 1;
        }
        i += 1;
    }
    v
}

/// The body of the two Croatian tie-break play-offs (`create_playoff_stage`, thiscall, ret 0).
///
/// The table scan (16-bit row index i, while i < `club_count`): when i is not the last row and rows i and i + 1 are
/// level on points with neither of status 3, both clubs go into the list (a local array), row i's worst possible
/// position (`league_table_row::WORST_POSSIBLE_POSITION`) becomes i + 2 and row i + 1's best (`BEST_POSSIBLE_POSITION`)
/// i + 1, both statuses 1, and the scan skips row i + 1; otherwise, when i > 0 and row i is level on points with row i
/// − 1, its worst and best possible positions become i + 1. The pair count (the list count / 2, truncated) goes into
/// `TIEBREAK_PAIRS`.
///
/// No pair: no extra stage (+0x2c = 0, current stage −1), no tie-break matches (`league::TIEBREAK_MATCHES` = 0) and the
/// positions and statuses recomputed (virtual `league_vtable::UPDATE_POSITIONS_AND_STATUS`, thiscall, ret 0).
/// Otherwise: the stage 0 round (virtual `comp_vtable::CREATE_SCHEDULE`: push 0; push &extra; push &rounds; push 0 ->
/// thiscall, ret 0x10; `croatian_tiebreak_round`), the stage (operator new `exe::crt_operator_new` of 0xb2 and
/// Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`(league, count, list, rounds, competition, round, year, 0, 1, extra, 0x14,
/// 0, 0, 0, 0): 15 pushes -> thiscall, ret 0x3c; the rounds, extra and year pushed as dwords with stack garbage above
/// the words, which the constructor does not read) into the first slot of the stage vector (+0xc); no stage: the box
/// at line `row` and return; otherwise the round freed (`exe::crt_free`), the first round set up (Cup::set_up_round
/// `exe::cup_set_up_round`: push 0 -> thiscall, ret 4), current stage 0 and the first tie-break code 6
/// (`league::TIEBREAKS`).
pub(crate) unsafe fn create_croatian_tiebreak_playoff(this: League, file: usize, row: u32) {
    let mut list = [0usize; 0x40];
    let mut count: u32 = 0;
    let n = this.club_count();
    let mut i: i16 = 0;
    while i < n {
        let r = this.table() + (i as i32 as usize).wrapping_mul(league_table_row::SIZE);
        let next = r + league_table_row::SIZE;
        if (i as i32) < n as i32 - 1 && read_u16(r + league_table_row::POINTS) == read_u16(next + league_table_row::POINTS)
            && read_u8(r + league_table_row::STATUS) != 3 && read_u8(next + league_table_row::STATUS) != 3 {
            list[(count as u16 as i16 as usize) & 0x3f] = read_usize(r);
            list[((count + 1) as u16 as i16 as usize) & 0x3f] = read_usize(next);
            count += 2;
            write_u8(r + league_table_row::WORST_POSSIBLE_POSITION, (i as u8).wrapping_add(2));
            write_u8(next + league_table_row::BEST_POSSIBLE_POSITION, (i as u8).wrapping_add(1));
            write_u8(r + league_table_row::STATUS, 1);
            write_u8(next + league_table_row::STATUS, 1);
            i += 1;
        } else if i > 0 && read_u16(r + league_table_row::POINTS) == read_u16(r - league_table_row::SIZE + league_table_row::POINTS) {
            write_u8(r + league_table_row::WORST_POSSIBLE_POSITION, (i as u8).wrapping_add(1));
            write_u8(r + league_table_row::BEST_POSSIBLE_POSITION, (i as u8).wrapping_add(1));
        }
        i += 1;
    }
    let pairs = ((count as u16 as i16 as i32) / 2) as u8;
    write_u8(this.addr() + TIEBREAK_PAIRS, pairs);
    if pairs == 0 { // mwd7: no level clubs
        this.set_stage_count(0);
        this.set_current_stage(0xffff_ffff);
        this.set_tiebreak_matches(0);
        bra_nat_first_call_virtual_no_args(this.addr(), league_vtable::UPDATE_POSITIONS_AND_STATUS);
        return;
    }
    covered!("mwd_7 croatian tie-break play-off");
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

/// Rule: at the end of the Croatian First Division, the clubs level on points with the club right below them (neither
/// relegated) play a one-round tie-break play-off (stage 0, in May): each pair of level rows goes into the play-off and
/// the positions they can still reach are narrowed; with no level pair, the season ends without tie-breaks.
/// Original: `CroA1::create_playoff_stage` (0x514c60, cro_a1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a1_create_playoff_stage(this: usize) {
    create_croatian_tiebreak_playoff(League(this), 0x9b53c0, 0x24b);
}

/// Rule: the same tie-break play-off at the end of the Croatian Second Division south group (line 0x23a of
/// cro_a2b.cpp).
/// Original: `CroA2b::create_tiebreak_playoff_stage` (0x517210, cro_a2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2b_create_tiebreak_playoff_stage(this: usize) {
    create_croatian_tiebreak_playoff(League(this), 0x9b5430, 0x23a);
}

/// Comparer for the Croatian tie-break play-offs: only the original runs (it builds a stage); the pair count must be
/// half the planned clubs (`croatian_tiebreak_plan`) and, with pairs, the stage's team list must hold the planned clubs
/// in order.
pub(crate) unsafe fn compare_croatian_tiebreak_playoff(name: &str, this: League, original: impl FnOnce()) {
    let plan = if this.table() != 0 { croatian_tiebreak_plan(this) } else { Vec::new() };
    original();
    let pairs = read_u8(this.addr() + TIEBREAK_PAIRS);
    let mut ok = pairs as usize == plan.len() / 2;
    let mut found: Vec<usize> = Vec::new();
    if ok && pairs != 0 {
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
    SCOREBOARD.record(ok, || format!("{}({:#x}): pairs {} plan {:x?} stage list {:x?}", name, this.addr(), pairs, plan, found));
}

/// Comparer for `cro_a1_create_playoff_stage`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_create_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    compare_croatian_tiebreak_playoff("cro_a1_create_playoff_stage", League(this), || o(this))
}

/// Comparer for `cro_a2b_create_tiebreak_playoff_stage`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2b_create_tiebreak_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_croatian_tiebreak_playoff("cro_a2b_create_tiebreak_playoff_stage", League(this), || o(this))
}
