// ---------- batch %%LEVA%%: the Irish League Cup: the next stage and the season turnover (ire_lge_cup.cpp) ----------

/// Rule: the Irish League Cup moves stage by stage through its seven groups; when the groups are over (stage 7) the
/// knockout of the eight group winners is built; past the last stage, the error box.
///
/// `advance_cup_stage` (the Italian cups' template, checked instruction by instruction against ItaCup::advance_stage
/// 0x63f5b0: only the file, the line 0x212 and the knockout routine IreLgeCup::create_knockout_stage 0x637c00 change).
/// Original: `IreLgeCup::advance_stage` (0x637930, ire_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_advance_stage(this: usize) {
    advance_cup_stage(League(this), 0x9e9f4c, 0x212, 7, 0x637c00) // mwa18: the Irish League Cup knockout after the seven groups
}

unsafe extern "thiscall" fn compare_ire_lge_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    ita_cup_compare_advance_stage("ire_lge_cup_advance_stage", League(this), || o(this))
}

/// The turnover of an Irish league cup (the cup is the league of its first group).
///
/// `league::SEASON_CLOSED` = 0; the table freed (`exe::league_free_table`(this, 0)); the round table freed and zeroed;
/// the season archived in the statistics (when there); the entrant vector (`ita_cup_fields::ENTRANT_CLUBS`) freed (not
/// zeroed: the entrant routine replaces it); with the current stage ≥ 0, the stages 0 to the current one destroyed
/// (their virtual 0 with 1; the slots are not zeroed); the year up by `year`; `league::CURRENT_STAGE` = −1. Then the
/// entrants (thiscall, ret 0, unchecked), the predicted positions (`comp_vtable::SET_PREDICTED_POSITIONS`) and, each
/// with its own error box (returning 0) when it returns 0: the rules (`comp_vtable::SET_UP_RULES`), the first group's
/// table (`table`), the fixtures and the order; finally the stage of each group (push g -> thiscall, ret 4), g from 0.
pub(crate) unsafe fn start_irish_league_cup_season(this: League, entrants: usize, table: usize, group_stage: usize, groups: u32, file: usize, rows: [u32; 4], year: i16) -> u32 {
    this.set_season_closed(0);
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    let v = read_usize(this.addr() + ita_cup_fields::ENTRANT_CLUBS);
    if v != 0 {
        exe::crt_free(v);
    }
    if (this.current_stage() as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(this.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
    this.set_year(this.year().wrapping_add(year));
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(entrants);
    entrants(this.addr());
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    // simplification: in the boxes the original returns the box's eax; the caller of the turnover does not read it
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, rows[0]);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(table);
    if table(this.addr()) == 0 {
        show_error_at(file, rows[1]);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, rows[2]);
        return 0;
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(file, rows[3]);
        return 0;
    }
    covered!("mwa_18 irish league cup turnover");
    let group: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(group_stage);
    let mut r = 0;
    for g in 0..groups {
        r = group(this.addr(), g) as u32;
    }
    r
}

/// Rule: at the season turn the Irish League Cup is rebuilt: the table, the round table, the entrants and the stages of
/// the last edition are freed, the statistics archived, the year moves on; then the 26 entrants, the predicted
/// positions, the rules, the first group's table, its fixtures and order and the stages of the seven groups.
///
/// `start_irish_league_cup_season` with IreLgeCup::collect_entrants (0x637dd0), create_table (0x6375c0),
/// create_group_stage (0x6379c0), seven groups and the error box lines 0x1e4, 0x1eb, 0x1f1 and 0x1f8 of
/// ire_lge_cup.cpp.
/// Original: `IreLgeCup::start_new_season` (0x637760, ire_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_start_new_season(this: usize) -> u32 {
    start_irish_league_cup_season(League(this), 0x637dd0, 0x6375c0, 0x6379c0, 7, 0x9e9f4c, [0x1e4, 0x1eb, 0x1f1, 0x1f8], 1) // mwa18: the Irish League Cup turnover
}

unsafe extern "thiscall" fn compare_ire_lge_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("ire_lge_cup_start_new_season", League(this), || o(this))
}
