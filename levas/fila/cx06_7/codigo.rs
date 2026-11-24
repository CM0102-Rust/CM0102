// ---------- cx06_7: the Confederations Cup's groups and semi-finals (fifa_confed.cpp) ----------

/// Builds group A of the Confederations Cup in the cup itself: 4 teams (`.set_club_count()`), the table
/// (`exe::crt_malloc`(0xec), 4 rows of `league_table_row::SIZE`; out of memory: the box at line 0x1c0 and 0); the 8
/// nations (+0xee, count +0xf2) sorted by reputation (`exe::crt_qsort` with 0x4b07d0) and every second one from the first
/// (0, 2, 4, 6) in rows 0 to 3 (`exe::league_init_table_row`(this, row, team, 0, 0)); the group draw news
/// (`exe::league_send_group_draw_news`) and 1.
/// Original: `FifaConfed::create_group_table` (0x58bcb0, fifa_confed.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fifa_confed_create_group_table(this: usize) -> u32 {
    let l = League(this);
    l.set_club_count(4);
    let table = exe::crt_malloc(0xec);
    l.set_table(table);
    if table == 0 {
        show_error_at(0x9c7474, 0x1c0); // fifa_confed.cpp
        return 0;
    }
    exe::crt_qsort(read_usize(this + 0xee), read_u32(this + 0xf2), 4, 0x4b07d0usize);
    covered!("cx06_7 confed group a");
    let mut row = 0i8;
    while (row as i16) < l.club_count() {
        let team = read_usize(read_usize(this + 0xee) + 8 * row as usize);
        exe::league_init_table_row(this, row as u8 as u32, team, 0, 0);
        row = row.wrapping_add(1);
    }
    exe::league_send_group_draw_news(this);
    1
}

/// Creates group B of the Confederations Cup as stage 0: the odd nations of the sorted list (1, 3, 5, 7) in a list of 4
/// (`exe::crt_malloc`(0x10); out of memory: the box at line 0x2b3 and 0), the round table of stage 0
/// (`comp_vtable::CREATE_SCHEDULE`: 0, &rounds, &id, 0), the stage (`exe::crt_operator_new`(0xee) and
/// League_Stage::League_Stage `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58: the cup, 4, the list, 1,
/// `.competition()`, the round table, the rounds, the win, draw and category bytes, &tie-breaks, &promoted, the year, 0,
/// the id, the importance, 2, 0, `.league_flags()`, −1, 0, 2) into `.stages()`[0]; the list and the round table freed;
/// none: the box at line 0x2ca and 0. Then `.current_stage()` 0, the group draw news on the stage and 1.
/// Original: `FifaConfed::create_group_stage` (0x58c100, fifa_confed.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fifa_confed_create_group_stage(this: usize) -> u32 {
    let l = League(this);
    let teams = exe::crt_malloc(0x10);
    if teams == 0 {
        show_error_at(0x9c7474, 0x2b3); // fifa_confed.cpp
        return 0;
    }
    for k in 0..4 {
        write_usize(teams + 4 * k, read_usize(read_usize(this + 0xee) + 4 * (2 * k + 1)));
    }
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut id) = (0i16, 0i16);
    let table = schedule(this, 0, &mut rounds, &mut id, 0);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        covered!("cx06_7 confed group b");
        construct(p, this, 4, teams, 1, l.competition().addr(), table, rounds as u16 as u32, l.win_points() as u32, l.draw_points() as u32,
            l.match_category() as u32, this + league::TIEBREAKS, this + league::PROMOTED_COUNT, l.year() as u16 as u32, 0, id as u16 as u32,
            l.importance() as u32, 2, 0, l.league_flags() as u16 as u32, -1, 0, 2)
    } else {
        0
    };
    write_usize(l.stages(), stage);
    exe::crt_free(teams);
    exe::crt_free(table);
    if read_usize(l.stages()) == 0 {
        show_error_at(0x9c7474, 0x2ca);
        return 0;
    }
    l.set_current_stage(0);
    exe::league_send_group_draw_news(read_usize(l.stages()));
    1
}

/// The 4 semi-finalists of the Confederations Cup, crossed: A runner-up, B winner, A winner, B runner-up (A is the
/// cup's own table, B the table of `.stages()`[0]).
pub(crate) unsafe fn confed_semi_finalists(this: League) -> [usize; 4] {
    let a = |row: usize| read_usize(this.table() + row * league_table_row::SIZE);
    let b = |row: usize| read_usize(League(read_usize(this.stages())).table() + row * league_table_row::SIZE);
    [a(1), b(0), a(0), b(1)]
}

/// Creates the Confederations Cup semi-finals as stage 1: the list of the 4 (`exe::crt_malloc`(0x10); out of memory: the
/// box at line 0x2e8 and return) crossed (`confed_semi_finalists`); the round table of stage 1
/// (`comp_vtable::CREATE_SCHEDULE`: 1, &rounds, &id, 0); the stage (`exe::crt_operator_new`(0xb2) and
/// Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret 0x3c: the cup, 4, the list, the rounds,
/// `.competition()`, the round table, the year, 1, 2, the id, 0x14, 0, 0, 0, no seeds) into `.stages()`[1]; the round
/// table and the list freed; none: the box at line 0x300 and return. With it: its first round set up
/// (`exe::cup_set_up_round` with 0) and `.current_stage()` 1.
/// Original: `FifaConfed::create_knockout_stage` (0x58c2d0, fifa_confed.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fifa_confed_create_knockout_stage(this: usize) {
    let l = League(this);
    let list = exe::crt_malloc(0x10);
    if list == 0 {
        return show_error_at(0x9c7474, 0x2e8); // fifa_confed.cpp
    }
    for (k, &team) in confed_semi_finalists(l).iter().enumerate() {
        write_usize(list + 4 * k, team);
    }
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut id) = (0i16, 0i16);
    let table = schedule(this, 1, &mut rounds, &mut id, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        covered!("cx06_7 confed semi-finals");
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, 4, list, rounds as u16 as u32, l.competition().addr(), table, l.year() as u16 as u32, 1, 2, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_usize(l.stages() + 4, stage);
    exe::crt_free(table);
    exe::crt_free(list);
    let stage = read_usize(l.stages() + 4);
    if stage == 0 {
        return show_error_at(0x9c7474, 0x300);
    }
    exe::cup_set_up_round(stage, 0);
    l.set_current_stage(1);
}

/// Comparer for `fifa_confed_create_group_table`: it allocates the table and sends the news (it cannot run twice), so
/// only the original runs; the plan: 4 rows with the sorted list's nations 0, 2, 4 and 6.
pub(crate) unsafe extern "thiscall" fn compare_fifa_confed_create_group_table(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    let l = League(this);
    if r != 1 {
        return r;
    }
    let list = read_usize(this + 0xee);
    let want: Vec<usize> = (0..4).map(|k| read_usize(list + 8 * k)).collect();
    let got: Vec<usize> = (0..4).map(|k| read_usize(l.table() + k * league_table_row::SIZE)).collect();
    SCOREBOARD.record(l.club_count() == 4 && got == want, || format!("fifa_confed_create_group_table({:#x}): rows {:x?} expected {:x?}", this, got, want));
    r
}

/// Comparer for `fifa_confed_create_group_stage`: it allocates the stage and sends the news (it cannot run twice), so
/// only the original runs; the plan: the stage at `.stages()`[0] with the nations 1, 3, 5 and 7 (as a set: the stage
/// sorts its table) and `.current_stage()` 0.
pub(crate) unsafe extern "thiscall" fn compare_fifa_confed_create_group_stage(this: usize) -> u32 {
    let list = read_usize(this + 0xee);
    let mut want: Vec<usize> = if list != 0 { (0..4).map(|k| read_usize(list + 4 * (2 * k + 1))).collect() } else { Vec::new() };
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(this);
    if r != 1 {
        return r;
    }
    let l = League(this);
    let stage = League(read_usize(l.stages()));
    let mut got: Vec<usize> = (0..stage.club_count().clamp(0, 4) as usize).map(|k| read_usize(stage.table() + k * league_table_row::SIZE)).collect();
    got.sort_unstable();
    want.sort_unstable();
    SCOREBOARD.record(got == want && l.current_stage() == 0, || format!("fifa_confed_create_group_stage({:#x}): teams {:x?} expected {:x?}", this, got, want));
    r
}

/// Comparer for `fifa_confed_create_knockout_stage`: it allocates the stage and builds its round (it cannot run twice),
/// so only the original runs; the plan: the stage at `.stages()`[1] with the 4 crossed semi-finalists (read before; as
/// a set: the round may reorder them) and `.current_stage()` 1.
pub(crate) unsafe extern "thiscall" fn compare_fifa_confed_create_knockout_stage(this: usize) {
    let l = League(this);
    let ready = l.table() != 0 && read_usize(l.stages()) != 0 && League(read_usize(l.stages())).table() != 0;
    let mut want: Vec<usize> = if ready { confed_semi_finalists(l).to_vec() } else { Vec::new() };
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    let stage = Cup(read_usize(l.stages() + 4));
    if !ready || stage.addr() == 0 {
        return;
    }
    let mut got: Vec<usize> = (0..stage.team_count().clamp(0, 4) as usize).map(|k| read_usize(stage.team_table() + 6 * k)).collect();
    got.sort_unstable();
    want.sort_unstable();
    SCOREBOARD.record(got == want && l.current_stage() == 1, || format!("fifa_confed_create_knockout_stage({:#x}): teams {:x?} expected {:x?}", this, got, want));
}
