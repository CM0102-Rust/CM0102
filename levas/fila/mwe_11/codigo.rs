// ---------- batch %%LEVA%%: the Norwegian promotion play-off and the Ykkönen stage advance and club progress (nor_first.cpp, fin_first.cpp) ----------

/// The Norwegian promotion play-off (stage 0): the Norwegian First Division club with the promotion play-off status
/// (the 3rd) against the Tippeligaen club with the relegation play-off status (the 12th), as a cup tie.
///
/// `NorFirst::create_play_off_stage()`: the list of 2 (`exe::crt_malloc` of 8; out of memory: the box of nor_first.cpp
/// (0xa69740) line 0x326 and return). Slot 0 = the club of the LAST First Division row with status
/// `table_status::PROMOTION_PLAYOFF` (the loop does not stop at the first; the count re-read each turn); none: the box
/// of line 0x336 and return (the list is not freed). Slot 1 = the club of the last Tippeligaen row
/// (`G_COMP_OBJECT_TABLE` by the id of `KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION`, League::get_table_row
/// `exe::league_get_table_row`, called twice for the chosen row) with status `table_status::RELEGATION_PLAYOFF`; a
/// Tippeligaen without clubs, or with no such row, ends it silently (the list is not freed). Then the stage
/// (`build_two_club_play_off`: the round table of stage 0, the Cup_Stage of the 2 clubs into the first slot, the list
/// and the round table freed); with no stage, the box of line 0x357; otherwise Cup::set_up_round
/// (`exe::cup_set_up_round` with 0). Unlike the Superettan's, it does not write `.current_stage()` (the advance already
/// set it to 0).
/// Original: `NorFirst::create_play_off_stage` (0x790e20, nor_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_first_create_play_off_stage(this: usize) {
    let this = League(this);
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(0xa69740, 0x326);
    }
    let Some(c) = last_row_club_with_status(this, table_status::PROMOTION_PLAYOFF) else { // mwe11: the Norwegian play-off club
        return show_error_at(0xa69740, 0x336);
    };
    write_usize(list, c);
    let Some(c) = last_premier_row_club_with_status(KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION, table_status::RELEGATION_PLAYOFF) else {
        return;
    };
    write_usize(list + 4, c);
    covered!("mwe_11 norwegian play-off");
    let stage = build_norwegian_play_off(this, list);
    if stage == 0 {
        return show_error_at(0xa69740, 0x357);
    }
    exe::cup_set_up_round(stage, 0);
}

/// The club of the last table row (a signed word count, re-read each turn) of `league` whose status is `status`.
pub(crate) unsafe fn last_row_club_with_status(league: League, status: u8) -> Option<usize> {
    let mut found = None;
    let mut i = 0i32;
    while i < league.club_count() as i32 {
        let r = league.table() + i as usize * league_table_row::SIZE;
        if read_u8(r + league_table_row::STATUS) == status {
            found = Some(read_usize(r));
        }
        i += 1;
    }
    found
}

/// The club of the last row of the Premier Division of key `premier` (League::get_table_row, twice for the chosen row)
/// whose status is `status`; None without clubs or without such a row.
pub(crate) unsafe fn last_premier_row_club_with_status(premier: usize, status: u8) -> Option<usize> {
    let p = League(comp_by_global(premier));
    let mut found = None;
    let mut i = 0i32;
    while i < p.club_count() as i32 {
        if read_u8(exe::league_get_table_row(p, i) + league_table_row::STATUS) == status {
            found = Some(read_usize(exe::league_get_table_row(p, i)));
        }
        i += 1;
    }
    found
}

/// The Cup_Stage of the two clubs of `list` into the first stage slot (the same steps as the Superettan's: the round
/// table of stage 0 by virtual `comp_vtable::CREATE_SCHEDULE`, operator new of 0xb2, Cup_Stage::Cup_Stage(league, 2,
/// list, rounds, competition, round table, year, 0, 1, extra, 0x14, 0, 0, 0, 0), the slot written, the list and the
/// round table freed); returns the slot.
pub(crate) unsafe fn build_norwegian_play_off(this: League, list: usize) -> usize {
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 0, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    // simplification: the original pushes the rounds, the extra and the year as dwords with garbage above the word;
    // the constructor only reads the words.
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 2, list, rounds as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(this.stages(), stage);
    exe::crt_free(list);
    exe::crt_free(t);
    read_usize(this.stages())
}

/// Moves the Ykkönen to its next stage: after the North group (and the South group stage 0) the play-off stages are
/// built at stage 1; stage 2 waits for the Veikkausliiga to finish and then the Premier Division play-off is built at
/// stage 3; at the last stage, the error box (line 0x2ce).
///
/// `FinFirst::advance_stage()`: at the last stage (`.current_stage()` ≥ `.stage_count()` − 1, signed), the box of
/// fin_first.cpp (0x9c818c) line 0x2ce and nothing else. At stage 2, only when the Veikkausliiga
/// (`KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION`) is complete (virtual `comp_vtable::CHECK_COMPLETE` with (0, 1)), after
/// its virtual `league_vtable::UPDATE_POSITIONS_AND_STATUS`; otherwise nothing. Then the current stage goes up by one: on
/// reaching 1, FinFirst::create_play_off_stages (0x594080); on reaching 3, FinFirst::create_premier_play_off_stage
/// (0x594610) (direct calls, thiscall, ret 0).
/// Original: `FinFirst::advance_stage` (0x593f90, fin_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_first_advance_stage(this: usize) {
    let this = League(this);
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x9c818c, 0x2ce);
    }
    if stage == 2 {
        let p = comp_by_global(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION);
        let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(p) + comp_vtable::CHECK_COMPLETE));
        if complete(p, 0, 1) == 0 {
            return;
        }
        let update: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(p) + league_vtable::UPDATE_POSITIONS_AND_STATUS));
        update(p); // mwe11: the Veikkausliiga statuses brought up to date
    }
    let stage = stage.wrapping_add(1);
    this.set_current_stage(stage as usize);
    let next: usize = match stage {
        1 => 0x594080,
        3 => 0x594610,
        _ => return,
    };
    covered!("mwe_11 ykkonen next stage built");
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(next);
    f(this.addr());
}

/// The Ykkönen progress code for one value at one stage: at stage 3 (the Premier Division play-off, for a Veikkausliiga
/// club) 1 → 11 and 2 → 12; at stage 2 (the play-offs, for an Ykkönen club) 1 → 15 and 3 → 17; otherwise the value.
pub(crate) fn finnish_progress_code(stage: i8, v: u8) -> u8 {
    match (stage, v) {
        (3, 1) => 0xb,
        (3, 2) => 0xc,
        (2, 1) => 0xf, // mwe11: the play-off winner's code
        (2, 3) => 0x11,
        _ => v,
    }
}

/// Writes a club's current, best and worst progress codes in its Comp_Stats entry, interleaving the two groups and
/// mapping the play-off results to the codes after the league places.
///
/// `FinFirst::set_club_progress(club, stage, current, best, worst)`: the entry of the club in the league's Comp_Stats
/// (`exe::comp_stats_find_club_entry`, thiscall, ret 4). Stage (a signed byte) below 1 (the groups): each value v
/// becomes 2v − 1 (a byte; the North and South places interleave). Stage 1: as they come. Stage 3: for a club whose
/// division (`club::DIVISION`, read without a null check) is the Veikkausliiga, the entry of the Veikkausliiga's
/// Comp_Stats (`G_COMP_OBJECT_TABLE` by `KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION`, its `league::COMP_STATS`) with 1 → 11
/// and 2 → 12; any other club, as they come in the Ykkönen entry. Stage 2: for an Ykkönen club
/// (`KEY_CLUB_COMP_FINNISH_FIRST_DIVISION`), 1 → 15 and 3 → 17; any other club, nothing is written. Other stages: as
/// they come. The bytes go to `comp_stats_club::CURRENT_POSITION`, `::BEST_POSITION` and `::WORST_POSITION` (the entry
/// is not checked for null).
/// Original: `FinFirst::set_club_progress` (0x593e10, fin_first.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn fin_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let mut e = exe::comp_stats_find_club_entry(read_usize(this + league::COMP_STATS), club);
    let s = stage as u8 as i8;
    let (mut c, mut b, mut w) = (current as u8, best as u8, worst as u8);
    if s < 1 {
        covered!("mwe_11 ykkonen group progress");
        c = c.wrapping_shl(1).wrapping_sub(1); // mwe11: the group places interleaved
        b = b.wrapping_shl(1).wrapping_sub(1);
        w = w.wrapping_shl(1).wrapping_sub(1);
    } else if s == 3 {
        if read_usize(read_usize(club + club::DIVISION)) == read_usize(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION) {
            e = exe::comp_stats_find_club_entry(read_usize(comp_by_global(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION) + league::COMP_STATS), club);
            c = finnish_progress_code(3, c);
            b = finnish_progress_code(3, b);
            w = finnish_progress_code(3, w);
        }
    } else if s == 2 {
        if read_usize(read_usize(club + club::DIVISION)) != read_usize(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION) {
            return;
        }
        c = finnish_progress_code(2, c);
        b = finnish_progress_code(2, b);
        w = finnish_progress_code(2, w);
    }
    write_u8(e + comp_stats_club::BEST_POSITION, b);
    write_u8(e + comp_stats_club::CURRENT_POSITION, c);
    write_u8(e + comp_stats_club::WORST_POSITION, w);
}

/// Comparer for `nor_first_create_play_off_stage`: only the original runs (it allocates and draws); the planned
/// clubs (the last rows with each status) against the stage, when both exist.
pub(crate) unsafe extern "thiscall" fn compare_nor_first_create_play_off_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let l = League(this);
    let plan = [last_row_club_with_status(l, table_status::PROMOTION_PLAYOFF), last_premier_row_club_with_status(KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION, table_status::RELEGATION_PLAYOFF)];
    o(this);
    if plan.iter().all(|c| c.is_some()) {
        let stage = Cup(read_usize(l.stages()));
        let ok = !stage.is_null() && stage.team_count() == 2 && {
            let list = stage.team_table();
            let mut found = [read_usize(list), read_usize(list + 6)];
            let mut want = [plan[0].unwrap(), plan[1].unwrap()];
            found.sort_unstable();
            want.sort_unstable();
            found == want
        };
        SCOREBOARD.record(ok, || format!("nor_first_create_play_off_stage({:#x}) plan={:x?} stage={:#x}", this, plan, stage.addr()));
    }
}

/// Comparer for `fin_first_advance_stage`: only the original runs (it builds stages); the current stage stays (at the
/// last stage, or at stage 2 with the Veikkausliiga still playing) or goes up by one.
pub(crate) unsafe extern "thiscall" fn compare_fin_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let l = League(this);
    let before = l.current_stage() as i32;
    let last = before >= (l.stage_count() as i32).wrapping_sub(1);
    o(this);
    let after = l.current_stage() as i32;
    let ok = if last { after == before } else if before == 2 { after == 2 || after == 3 } else { after == before + 1 };
    SCOREBOARD.record(ok, || format!("fin_first_advance_stage({:#x}) before={} after={}", this, before, after));
}

/// Comparer for `fin_first_set_club_progress`: side by side on the 3 bytes of the entry the routine writes (the
/// Veikkausliiga's at stage 3 for a Veikkausliiga club); only the original without an entry.
pub(crate) unsafe extern "thiscall" fn compare_fin_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    let premier_club = read_usize(club + club::DIVISION) != 0 && read_usize(read_usize(club + club::DIVISION)) == read_usize(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION);
    let stats = if stage as u8 as i8 == 3 && premier_club { read_usize(comp_by_global(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION) + league::COMP_STATS) } else { read_usize(this + league::COMP_STATS) };
    let e = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if e == 0 || read_usize(club + club::DIVISION) == 0 {
        return o(this, club, stage, current, best, worst);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(e + comp_stats_club::CURRENT_POSITION, 3)], || o(this, club, stage, current, best, worst), || fin_first_set_club_progress(this, club, stage, current, best, worst));
    SCOREBOARD.record(same, || format!("fin_first_set_club_progress({:#x}, {:#x}, stage {}, {} {} {}) original={:x?} rust={:x?}", this, club, stage as u8 as i8, current as u8, best as u8, worst as u8, mo, mr));
}
