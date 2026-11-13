// ---------- mwf_13: the Auto Windscreens Shield season turnover and rounds (eng_auto_cup.cpp) ----------

/// The season turnover of the Auto Windscreens Shield (the Football League Trophy): last season's tables, selected
/// clubs and section stages dropped, the season archived, the year plus one, then the 56 clubs selected again, the
/// entrants, the round table, the tie tree and the first section stage rebuilt; the selected list is freed again and
/// the predicted positions close it.
///
/// `cup::SEASON_CLOSED` = 0; the team table (`cup::TEAM_TABLE`), the round table (`cup::ROUND_TABLE`) and the selected
/// clubs (`eng_auto_cup::SELECTED_CLUBS`) freed (`exe::crt_free`) and zeroed; when there are tie tables
/// (`cup::TIE_TABLES`), each round's (`cup::ROUND_COUNT` rounds, a signed word) freed and zeroed and the array freed and
/// zeroed; with a current stage (`cup::CURRENT_STAGE`) ≥ 0, the stages 0..=current destroyed (virtual 0 with 1) and
/// then `cup::CURRENT_STAGE` = −1; with statistics, Comp_Stats::archive_season_and_reset
/// (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the year up by one and
/// `cup::CURRENT_ROUND` = 0.
///
/// Then, each with its own box (eng_auto_cup.cpp): the 56 section clubs (EngAutoCup::select_section_clubs 0x56b730,
/// thiscall, ret 0; box 0x1ee when it returns 0); `cup::RUN_BY_STAGES` = 0; the entrants (EngAutoCup::collect_entrants
/// 0x56b3c0, box 0x1f8 when negative); the round table (virtual `comp_vtable::SET_UP_RULES`, box 0x1ff when negative)
/// and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`, box 0x206 when negative). Then the first section stage
/// (EngAutoCup::create_section_stage 0x56bab0, thiscall, ret 0), the selected clubs freed and zeroed again, and the
/// predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax it returns. After a box the copy
/// returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `EngAutoCup::start_new_season` (0x56b4b0, eng_auto_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_auto_cup_start_new_season(this: usize) -> u32 {
    let this = Cup(this);
    this.set_season_closed(0);
    for o in [cup::TEAM_TABLE, cup::ROUND_TABLE, eng_auto_cup::SELECTED_CLUBS] {
        let l = read_usize(this.addr() + o);
        if l != 0 {
            exe::crt_free(l);
            write_usize(this.addr() + o, 0);
        }
    }
    if this.tie_tables() != 0 {
        let mut i = 0i32;
        while i < this.round_count() as i32 {
            let p = read_usize(this.tie_tables() + i as usize * 4);
            if p != 0 {
                exe::crt_free(p);
                write_usize(this.tie_tables() + i as usize * 4, 0);
            }
            i += 1;
        }
        exe::crt_free(this.tie_tables());
        this.set_tie_tables(0);
    }
    if read_i32(this.addr() + cup::CURRENT_STAGE) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(read_usize(this.addr() + cup::STAGES) + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > read_i32(this.addr() + cup::CURRENT_STAGE) {
                break;
            }
        }
    }
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    let stats = read_usize(this.addr() + 8);
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    let a = this.year();
    this.set_year(a.wrapping_add(1)); // mwf13: the year of the Auto Windscreens Shield
    this.set_current_round(0);
    covered!("mwf_13 auto windscreens rollover");
    let select: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x56b730usize);
    if select(this.addr()) == 0 {
        show_error_at(0x9c4180, 0x1ee);
        return 0;
    }
    this.set_run_by_stages(0);
    let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x56b3c0usize);
    if entrants(this.addr()) < 0 {
        show_error_at(0x9c4180, 0x1f8);
        return 0;
    }
    if call_cup_virtual(this, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(0x9c4180, 0x1ff);
        return 0;
    }
    if call_cup_virtual(this, cup_vtable::MAP_FIXTURE_TREE) < 0 {
        show_error_at(0x9c4180, 0x206);
        return 0;
    }
    let section: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x56bab0usize);
    section(this.addr());
    let l = read_usize(this.addr() + eng_auto_cup::SELECTED_CLUBS);
    if l != 0 {
        exe::crt_free(l);
        write_usize(this.addr() + eng_auto_cup::SELECTED_CLUBS, 0);
    }
    call_cup_virtual(this, comp_vtable::SET_PREDICTED_POSITIONS) as u32
}

/// Comparer for `eng_auto_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must
/// go up by one.
pub(crate) unsafe extern "thiscall" fn compare_eng_auto_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_cup_new_season("eng_auto_cup_start_new_season", Cup(this), || o(this))
}

/// The Auto Windscreens Shield rounds: a prize fund of 400,000 and the round table.
///
/// `cup::PRIZE_FUND` = 400,000 (0x61a80); the round table of stage −1 (virtual `comp_vtable::CREATE_SCHEDULE`: push 0;
/// push &`cup::STAGE_LABEL`; push &`cup::ROUND_COUNT`; push −1 -> thiscall, ret 0x10) into `cup::ROUND_TABLE`. The
/// substitutes and changes are not set here. Returns 1.
/// Original: `EngAutoCup::set_up_rounds` (0x56b700, eng_auto_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_auto_cup_set_up_rounds(this: usize) -> i32 {
    let this = Cup(this);
    covered!("mwf_13 auto windscreens rounds");
    this.set_prize_fund(0x61a80); // mwf13: the Auto Windscreens Shield prize fund
    let table: unsafe extern "thiscall" fn(usize, u32, usize, usize, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = table(this.addr(), 0xffff_ffff, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0);
    this.set_round_table(t);
    1
}

/// Comparer for `eng_auto_cup_set_up_rounds`: side by side (as the Copa do Brasil's): the prize fund, the round count
/// and the stage label, and the bytes the round table writes (`eng_cc_cup_written_round_bytes`); the copy's table is
/// freed and the original's kept.
pub(crate) unsafe extern "thiscall" fn compare_eng_auto_cup_set_up_rounds(this: usize) -> i32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    let to = core::cell::Cell::new(0usize);
    let tr = core::cell::Cell::new(0usize);
    let before = this.round_table();
    let reg = [(this.addr() + cup::PRIZE_FUND, 4usize), (this.addr() + cup::STAGE_LABEL, 4), (G_MOUSE_LEFT_BUTTON_DOWN, 4)];
    let (a, b, same, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this.addr());
        to.set(this.round_table());
        this.set_round_table(before);
        r
    }, || {
        let r = eng_auto_cup_set_up_rounds(this.addr());
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
    SCOREBOARD.record(same && a == b && table, || format!("eng_auto_cup_set_up_rounds({:#x}) returned {} x {} memory {} table {}", this.addr(), a, b, same, table));
    a
}
