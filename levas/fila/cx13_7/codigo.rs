// ---------- cx13_7: the Polish Second Division's next stage and promotion play-off (pol_second.cpp) ----------

/// The next stage of the Polish Second Division: past the last stage, the pol_second.cpp box of line 0x234 and nothing
/// else. Otherwise it waits for the First Division ([`KEY_CLUB_COMP_POLISH_FIRST_DIVISION`]): while that one is not
/// complete (virtual `comp_vtable::CHECK_COMPLETE` with 0, 1) nothing happens; once it is, the First Division's final
/// stage (`league::STAGES`[1]) updates its positions and statuses (virtual
/// `league_vtable::UPDATE_POSITIONS_AND_STATUS`), the current stage goes up by one and, on reaching 0, the promotion
/// play-off is built (`pol_second_create_play_off_stage`, through its address).
/// Original: `PolSecond::advance_stage` (0x7cc1c0, pol_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_second_advance_stage(this: usize) {
    let l = League(this);
    if l.current_stage() as i32 >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0xa7860c, 0x234); // pol_second.cpp
    }
    let first = comp_by_global(KEY_CLUB_COMP_POLISH_FIRST_DIVISION);
    let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(first) + comp_vtable::CHECK_COMPLETE));
    if complete(first, 0, 1) == 0 {
        return;
    }
    covered!("cx13_7 polish second next stage");
    bra_nat_first_call_virtual_no_args(read_usize(League(first).stages() + 4), league_vtable::UPDATE_POSITIONS_AND_STATUS);
    let stage = (l.current_stage() as i32).wrapping_add(1);
    l.set_current_stage(stage as usize);
    if stage == 0 {
        call_on_league(0x7cc290, l); // cx13_7: the promotion play-off
    }
}

/// Comparer for `pol_second_advance_stage`: only the original (it updates the First Division's final stage and may
/// build the play-off); the current stage stays (at the last stage, or with the First Division still playing) or goes
/// up by one.
pub(crate) unsafe extern "thiscall" fn compare_pol_second_advance_stage(this: usize) {
    let l = League(this);
    let before = l.current_stage() as i32;
    let last = before >= (l.stage_count() as i32).wrapping_sub(1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let after = l.current_stage() as i32;
    SCOREBOARD.record(after == before || (!last && after == before + 1), || format!("pol_second_advance_stage({:#x}) before={} after={}", this, before, after));
}

/// The four clubs of the Polish promotion play-off, in the list's order: the Second Division rows with status 1 go to
/// slots 0 and 2, the First Division final stage rows with status 2 to slots 1 and 3 (the first found in each to the
/// lower slot, any later one to the upper slot, the last found winning it). Ties 0 × 1 and 2 × 3.
pub(crate) unsafe fn polish_play_off_clubs(this: League) -> [usize; 4] {
    let mut v = [0usize; 4];
    for k in 0..(this.club_count() as i16).max(0) as usize {
        let row = this.table() + k * league_table_row::SIZE;
        if read_u8(row + league_table_row::STATUS) == 1 {
            let slot = if v[0] == 0 { 0 } else { 2 }; // cx13_7: the Second Division's play-off places
            v[slot] = read_usize(row);
        }
    }
    let final_stage = League(read_usize(League(comp_by_global(KEY_CLUB_COMP_POLISH_FIRST_DIVISION)).stages() + 4));
    for k in 0..(final_stage.club_count() as i16).max(0) as i32 {
        if read_u8(exe::league_get_table_row(final_stage, k) + league_table_row::STATUS) == 2 {
            let slot = if v[1] == 0 { 1 } else { 3 };
            v[slot] = read_usize(exe::league_get_table_row(final_stage, k));
        }
    }
    v
}

/// The Polish promotion play-off: the list of its four clubs (calloc `exe::crt_calloc`(4, 4); out of memory, the box of
/// line 0x260 and nothing else) filled by `polish_play_off_clubs`; the round table of stage 0 (virtual
/// `comp_vtable::CREATE_SCHEDULE`(0, &rounds, &extra, null)); the stage (operator new `exe::crt_operator_new` of 0xb2;
/// Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`(this, 4, the list, the rounds, the competition record, the round table,
/// the year, 0, 1, the extra, 0x14, 0, 0, 0, 0)) into the first stage slot; the list and the round table freed. Without
/// a stage, the box of line 0x28b; otherwise the round is set up (Cup::set_up_round `exe::cup_set_up_round` with 0).
/// The original pushes the rounds, the extra and the year as dwords whose high halves are not set (the schedule writes
/// words); the copy passes them zero-extended (the constructor reads words).
/// Original: `PolSecond::create_play_off_stage` (0x7cc290, pol_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_second_create_play_off_stage(this: usize) {
    let l = League(this);
    let list = exe::crt_calloc(4, 4);
    if list == 0 {
        return show_error_at(0xa7860c, 0x260);
    }
    covered!("cx13_7 polish play-off");
    for (k, club) in polish_play_off_clubs(l).iter().enumerate() {
        if *club != 0 {
            write_usize(list + 4 * k, *club);
        }
    }
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = schedule(this, 0, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, 4, list, rounds as u16 as u32, l.competition().addr(), table, l.year() as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(l.stages(), stage);
    exe::crt_free(list);
    exe::crt_free(table);
    let stage = read_usize(l.stages());
    if stage == 0 {
        return show_error_at(0xa7860c, 0x28b);
    }
    exe::cup_set_up_round(stage, 0);
}

/// Comparer for `pol_second_create_play_off_stage`: only the original (it allocates the stage and draws the round);
/// the stage in the first slot with 4 clubs, the competition record and the planned clubs (as a set: the draw may
/// reorder the ties).
pub(crate) unsafe extern "thiscall" fn compare_pol_second_create_play_off_stage(this: usize) {
    let l = League(this);
    let plan = polish_play_off_clubs(l);
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let stage = Cup(read_usize(l.stages()));
    if stage.is_null() {
        SCOREBOARD.record(false, || format!("pol_second_create_play_off_stage({:#x}) without stage", this));
        return;
    }
    let mut found = [0usize; 4];
    for (k, a) in found.iter_mut().enumerate() {
        *a = read_usize(stage.team_table() + 6 * k);
    }
    let (mut x, mut y) = (plan, found);
    x.sort_unstable();
    y.sort_unstable();
    let same = x == y && stage.team_count() == 4 && read_usize(stage.addr() + 4) == l.competition().addr();
    SCOREBOARD.record(same, || format!("pol_second_create_play_off_stage({:#x}) plan={:x?} stage={:#x} list={:x?} count={}", this, plan, stage.addr(), found, stage.team_count()));
}
