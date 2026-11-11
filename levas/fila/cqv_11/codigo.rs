// ---------- %%LEVA%%a leva: as quartas da Mercosul (os cinco primeiros e os três melhores segundos) e as posições previstas da Libertadores e da Mercosul (conmebol_merc.cpp, conmebol_liber.cpp) ----------

/// Group `g` of the Mercosur Cup: -1 is the cup itself (group A), the others the stages `league::STAGES`[g].
unsafe fn conmebol_merc_group(this: usize, g: i8) -> usize {
    if g == -1 {
        this
    } else {
        read_usize(read_usize(this + league::STAGES) + g as isize as usize * 4)
    }
}

/// The Mercosur Cup quarter-finals, at the end of the five groups (SEH frame only for the stage `new`s, no destructor
/// on the normal path). A league stage of the runners-up (`exe::crt_operator_new`(0xee) and the League_Stage
/// constructor `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58, 22 arguments: the cup, 5, no teams, 0, the
/// competition record (`league::COMPETITION`), no table, 0, `league::WIN_POINTS`, `league::DRAW_POINTS`, 0,
/// &`league::TIEBREAKS`, &`league::PROMOTED_COUNT`, the year, -2, 0, 0, 1, 0, 2, -1, 0, 2; words and bytes clean), with
/// row 1 (the runner-up) of each group copied (`league_table_row::SIZE` bytes) into its row and the dword
/// `league_table_row::POSITION_HISTORY` zeroed (the points stay [inference]), and the table sorted
/// (`exe::league_sort_table`(stage, 0)). An array of 8 clubs (`exe::crt_malloc`(0x20); out of memory, the box for line
/// 0x248 and it returns without destroying the runners-up stage): the first `league::PROMOTED_COUNT` of group A
/// (`league::TABLE`) and of each of the groups B..E (stages 0..3, `exe::league_get_table_row`), and the first three of
/// the runners-up table. Each of those three gets one more place in its group (`league::PROMOTED_COUNT` of the group +
/// 1), its row marked (`league_table_row::STATUS` = 6) and the status (the virtual `comp_vtable::ANNOUNCE_TEAM_STATUS`(club,
/// 6, group, 0, 0): 5 pushes; mov ecx -> thiscall). Runners-up 4 and 5 stay out (the virtual
/// `comp_vtable::SET_CLUB_PROGRESS`(club, -1, 3, 3, 3)). Whoever played the groups and is not among the 8 gets the
/// failed-to-qualify record (`exe::staff_records_add_failed_to_qualify_record`(`G_STAFF_RECORDS`, club, competition
/// record, 0x41c, 0x28): 4 pushes; mov ecx -> thiscall, ret 0x10). The runners-up stage is destroyed (the virtual
/// `comp_vtable::SCALAR_DELETING_DESTRUCTOR` with 1). The quarter-finals: the round table of stage 4 (the virtual
/// `comp_vtable::CREATE_SCHEDULE`), the stage (`exe::crt_operator_new`(0xb2) and the Cup_Stage constructor
/// `FN_CUP_STAGE_CUP_STAGE`, ret 0x3c: the cup, 8, the array, the rounds, the competition record, the table, the year,
/// 4, 2, the extra, 0x14, 0, 0, 0, 0) in `league::STAGES`[4]; the array and the table freed; without a stage, the box
/// for line 0x2ab; with it, the draw of the first round (`exe::cup_draw_round`(stage, 0): push 0; mov ecx -> thiscall,
/// ret 4).
/// Original: `ConmebolMerc::create_quarter_final_stage` (0x4c53d0, conmebol_merc.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn conmebol_merc_create_quarter_final_stage(this: usize) {
    #[allow(clippy::type_complexity)]
    let league_stage: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, i32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    #[allow(clippy::type_complexity)]
    let cup_stage: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let b = |d: usize| read_u8(this + d) as u32;
    let p = exe::crt_operator_new(0xee);
    let runners_up = if p != 0 {
        league_stage(p, this, 5, 0, 0, read_usize(this + league::COMPETITION), 0, 0, b(league::WIN_POINTS), b(league::DRAW_POINTS), 0, this + league::TIEBREAKS, this + league::PROMOTED_COUNT, read_i16(this + league::YEAR) as u16 as u32, -2, 0, 0, 1, 0, 2, -1, 0, 2)
    } else {
        0
    };
    for g in 0..5usize {
        let source = read_usize(conmebol_merc_group(this, g as i8 - 1) + league::TABLE) + league_table_row::SIZE; // cqv11: the row of the group runner-up
        let target = read_usize(runners_up + league::TABLE) + g * league_table_row::SIZE;
        core::ptr::copy(source as *const u8, target as *mut u8, league_table_row::SIZE);
        write_u32(read_usize(runners_up + league::TABLE) + g * league_table_row::SIZE + league_table_row::POSITION_HISTORY, 0);
    }
    exe::league_sort_table(runners_up, 0);
    let v = exe::crt_malloc(0x20);
    if v == 0 {
        return show_error_at(0x9a7e5c, 0x248);
    }
    let mut k = 0usize;
    for i in 0..read_i8(this + league::PROMOTED_COUNT).max(0) as usize {
        write_usize(v + k * 4, read_usize(read_usize(this + league::TABLE) + i * league_table_row::SIZE));
        k += 1;
    }
    for g in 0..4usize {
        let stage = read_usize(read_usize(this + league::STAGES) + g * 4);
        for i in 0..read_i8(this + league::PROMOTED_COUNT).max(0) as i32 {
            write_usize(v + k * 4, read_usize(exe::league_get_table_row(stage, i)));
            k += 1;
        }
    }
    for s in 0..3i32 {
        covered!("cqv_11 best runners-up");
        write_usize(v + (k + s as usize) * 4, read_usize(exe::league_get_table_row(runners_up, s))); // cqv11: runner-up s into the quarter-finals
        for g in -1i8..4 {
            let group = conmebol_merc_group(this, g);
            let club = read_usize(exe::league_get_table_row(runners_up, s));
            if read_usize(exe::league_get_table_row(group, 1)) == club {
                let places = group + league::PROMOTED_COUNT;
                write_u8(places, read_u8(places).wrapping_add(1));
                write_u8(exe::league_get_table_row(group, 1) + league_table_row::STATUS, 6);
                let status: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::ANNOUNCE_TEAM_STATUS));
                // simplification: the original pushes the dword whose low byte is the group (the high ones are those of the count of 8, < 256: 0)
                status(this, read_usize(exe::league_get_table_row(runners_up, s)), 6, g as u8 as u32, 0, 0);
                break;
            }
        }
    }
    for s in 3..5i32 {
        let out: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_CLUB_PROGRESS));
        out(this, read_usize(exe::league_get_table_row(runners_up, s)), -1, 3, 3, 3);
    }
    for g in -1i8..4 {
        let group = conmebol_merc_group(this, g);
        for r in 0..read_i16(group + league::CLUB_COUNT).max(0) as usize {
            let club = read_usize(read_usize(group + league::TABLE) + r * league_table_row::SIZE);
            if !(0..8).any(|j| read_usize(v + j * 4) == club) {
                exe::staff_records_add_failed_to_qualify_record(read_usize(G_STAFF_RECORDS), club, read_usize(this + league::COMPETITION), 0x41c, 0x28);
            }
        }
    }
    if runners_up != 0 {
        let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(runners_up) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
        destroy(runners_up, 1);
    }
    let round_table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = round_table(this, 4, &mut rounds, &mut extra, 0);
    let eight = v; // cqv11: the 8 quarter-finalists in order
    let p = exe::crt_operator_new(0xb2);
    let quarter_finals = if p != 0 {
        cup_stage(p, this, 8, eight, rounds as u16 as u32, read_usize(this + league::COMPETITION), table, read_i16(this + league::YEAR) as u16 as u32, 4, 2, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(read_usize(this + league::STAGES) + 0x10, quarter_finals);
    exe::crt_free(v);
    exe::crt_free(table);
    let f = read_usize(read_usize(this + league::STAGES) + 0x10);
    if f == 0 {
        return show_error_at(0x9a7e5c, 0x2ab);
    }
    exe::cup_draw_round(f, 0);
}

// simplification: the stages are allocated, the records and the round draw cannot run twice. Only the original; the
// plan: the 8 quarter-finalists are the first `league::PROMOTED_COUNT` of each group and three runners-up, and the three
// runners-up that got in are the best placed among the five runners-up from before (the highest scoring: [inference]
// the table order is that of 0x6827d0, so it only checks that each one is a group runner-up and that no club repeats);
// the stage in `league::STAGES`[4] has the 8 in its team table (`cup::TEAM_TABLE`, 6-byte rows).
unsafe extern "thiscall" fn compare_conmebol_merc_create_quarter_final_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let q = read_i8(this + league::PROMOTED_COUNT);
    let stages = read_usize(this + league::STAGES);
    if !(1..=2).contains(&q) || stages == 0 || (0..4).any(|g| read_usize(stages + g * 4) == 0) {
        return o(this);
    }
    let mut first: Vec<usize> = Vec::new();
    let mut second: Vec<usize> = Vec::new();
    for g in -1i8..4 {
        let group = conmebol_merc_group(this, g);
        for i in 0..q as i32 {
            first.push(read_usize(exe::league_get_table_row(group, i)));
        }
        second.push(read_usize(exe::league_get_table_row(group, 1)));
    }
    o(this);
    let f = read_usize(read_usize(this + league::STAGES) + 0x10);
    if f == 0 {
        return;
    }
    let t = read_usize(f + cup::TEAM_TABLE);
    let mut read: Vec<usize> = if t != 0 { (0..8).map(|i| read_usize(t + i * 6)).collect() } else { Vec::new() };
    let got_in: Vec<usize> = read.iter().copied().filter(|c| !first.contains(c)).collect();
    let ok = read.len() == 8
        && first.iter().all(|c| read.contains(c))
        && got_in.len() + first.len() == 8
        && got_in.iter().all(|c| second.contains(c))
        && {
            read.sort_unstable();
            read.windows(2).all(|w| w[0] != w[1])
        };
    SCOREBOARD.record(ok, || format!("conmebol_merc_create_quarter_final_stage({:#x}): quarter-finals {:x?} group winners {:x?} runners-up {:x?}", this, read, first, second));
}

/// The Libertadores predicted positions: by the rounds of stage 7 (the knockout) and in blocks of 8 for those who stay
/// in the groups (box at line 0x30e); see `conmebol_set_predicted_positions`.
/// Original: `ConmebolLiber::set_predicted_positions` (0x4c4210, conmebol_liber.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn conmebol_liber_set_predicted_positions(this: usize) {
    conmebol_set_predicted_positions(this, 7, 8, 0x9a7de4, 0x30e)
}

unsafe extern "thiscall" fn compare_conmebol_liber_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    conmebol_compare_predicted_positions("conmebol_liber_set_predicted_positions", this, 7, 8, || o(this))
}

/// The Mercosur Cup predicted positions: by the rounds of stage 4 (the quarter-finals on) and in blocks of 5 for those
/// who stay in the groups (box at line 0x338); see `conmebol_set_predicted_positions`.
/// Original: `ConmebolMerc::set_predicted_positions` (0x4c5a90, conmebol_merc.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn conmebol_merc_set_predicted_positions(this: usize) {
    conmebol_set_predicted_positions(this, 4, 5, 0x9a7e5c, 0x338)
}

unsafe extern "thiscall" fn compare_conmebol_merc_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    conmebol_compare_predicted_positions("conmebol_merc_set_predicted_positions", this, 4, 5, || o(this))
}

/// The predicted positions the CONMEBOL cups give (0x4c2a90, 0x4c4210 and 0x4c5a90, the same body), in order: the list
/// of invited clubs (+0xee, +0xf2 clubs) already sorted; the rounds of the table `table` (`cup_round::SIZE`-byte
/// records; the word `cup_round::TIES` of each, those who go out in it), from the last to the first: the last gives
/// position 1 to those who go out in it and, like all of them, the current position (which goes up with each club) to
/// as many more; those left over go in blocks of `block` with the position of the start of the block. Returns (club,
/// position).
unsafe fn conmebol_predicted_positions_plan(this: usize, table: usize, rounds: i16, block: u32) -> Vec<(usize, u8)> {
    let mut v = Vec::new();
    let list = read_usize(this + 0xee);
    let mut i: i16 = 0;
    let mut bl: u8 = 1;
    let mut current: u8 = 1;
    let mut r = rounds as i32 - 1;
    while r >= 0 {
        let p = table + r as usize * cup_round::SIZE + cup_round::TIES;
        if r == rounds as i32 - 1 {
            let mut j: i16 = 0;
            while j < read_i16(p) {
                v.push((read_usize(list + i as isize as usize * 4), current));
                i = i.wrapping_add(1);
                bl = bl.wrapping_add(1);
                j += 1;
            }
        }
        current = bl;
        let mut j: i16 = 0;
        while j < read_i16(p) {
            v.push((read_usize(list + i as isize as usize * 4), current));
            i = i.wrapping_add(1);
            bl = bl.wrapping_add(1);
            j += 1;
        }
        r -= 1;
    }
    let mut pos = bl;
    while (i as i32) < read_usize(this + 0xf2) as i32 {
        v.push((read_usize(list + i as isize as usize * 4), pos));
        bl = bl.wrapping_add(1);
        if bl as u32 == pos as u32 + block {
            pos = bl;
        }
        i = i.wrapping_add(1);
    }
    v
}

/// The body of the predicted positions: without statistics (`league::COMP_STATS`), the box `row` of `file`; the list
/// +0xee of +0xf2 invited clubs sorted by reputation (`exe::crt_qsort`(list, n, 4, 0x4b07d0): 4 pushes; add esp ->
/// cdecl); the round table of stage `stage` by the virtual `comp_vtable::CREATE_SCHEDULE` (push 0; push &extra; push
/// &rounds; push stage; mov ecx, this -> thiscall, ret 0x10); each club of the plan (`conmebol_predicted_positions_plan`)
/// with its position (`exe::comp_stats_set_club_final_position`: push position; push club; mov ecx, stats -> thiscall,
/// ret 8; the original pushes the byte with leftovers in the high bytes, 0x4a2540 reads the byte); the table freed
/// (`exe::crt_free`).
unsafe fn conmebol_set_predicted_positions(this: usize, stage: u32, block: u32, file: usize, row: u32) {
    if read_usize(this + league::COMP_STATS) == 0 {
        return show_error_at(file, row);
    }
    exe::crt_qsort(read_usize(this + 0xee), read_usize(this + 0xf2) as u32, 4, 0x4b07d0);
    let round_table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = round_table(this, stage, &mut rounds, &mut extra, 0);
    for (c, p) in conmebol_predicted_positions_plan(this, table, rounds, block) {
        exe::comp_stats_set_club_final_position(read_usize(this + league::COMP_STATS), c, p as u32);
    }
    exe::crt_free(table);
}

// simplification: Comp_Stats::set_club_final_position adds the club's entry (reallocates): it cannot run twice. Only the
// original; the plan, with the list already sorted by it: each invited club with an entry and the positions
// (`comp_stats_club::FINAL_POSITION`, `exe::comp_stats_find_club_entry`) increasing in the list order, the first 1.
// (Asking again for the stage's round table changes the cup's calendar: the 07/11 comparison gave the Cup.cpp line 1278
// box; the plan calls nothing but the search.)
unsafe fn conmebol_compare_predicted_positions(name: &str, this: usize, _stage: u32, _block: u32, o: impl FnOnce()) {
    o();
    let stats = read_usize(this + league::COMP_STATS);
    let list = read_usize(this + 0xee);
    let n = read_usize(this + 0xf2) as i32;
    if stats == 0 || list == 0 || !(1..=256).contains(&n) {
        return;
    }
    let mut read: Vec<i32> = Vec::new();
    for k in 0..n as usize {
        let c = read_usize(list + k * 4);
        let e = if c != 0 { exe::comp_stats_find_club_entry(stats, c) } else { 0 };
        read.push(if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) as i32 } else { -1 });
    }
    let ok = read[0] == 1 && read.iter().all(|&x| x >= 1) && read.windows(2).all(|w| w[0] <= w[1]);
    SCOREBOARD.record(ok, || format!("{}({:#x}): positions {:?}", name, this, read));
}
