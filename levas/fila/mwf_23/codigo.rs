// ---------- batch %%LEVA%%: the constructors of the Argentine Premier and Second Divisions (arg_prm.cpp, arg_second.cpp) ----------

/// The fixed part of the Argentine league constructors (thiscall(this, year, record), ret 8): League::League
/// (`exe::new_league`, thiscall, ret 0), the record (+4), the vtable, the year (`league::YEAR`, word) and discipline rule
/// 3 (`discipline_rule::ARGENTINA`); the saved state (`exe::league_load_from_temp_file`(this, 1), thiscall, ret 4).
/// Returns true when it loaded. Otherwise importance 0xf, the main stage, current stage −1 and `stages` stages with their
/// array (`exe::crt_malloc` of 4 × stages, cdecl, unchecked).
unsafe fn argentine_league_start(this: League, year: u32, record: usize, vtable: usize, stages: u32) -> bool {
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), vtable);
    write_u16(this.addr() + league::YEAR, year as u16);
    this.set_discipline_rule(discipline_rule::ARGENTINA);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return true;
    }
    this.set_importance(0xf);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    write_u32(this.addr() + league::STAGE_COUNT, stages);
    this.set_stages(exe::crt_malloc(stages * 4));
    false
}

/// The statistics of a new league (operator new `exe::crt_operator_new` of 0x5ce, cdecl; Comp_Stats::Comp_Stats
/// `exe::new_comp_stats`, push league -> thiscall, ret 4; 0 when out of memory) into `league::COMP_STATS`; false
/// without them.
unsafe fn argentine_league_stats(this: League) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    this.set_comp_stats(s);
    s != 0
}

/// Calls a routine of the league (thiscall, ret 0) and returns its eax.
unsafe fn argentine_league_call(this: League, f: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(f);
    f(this.addr())
}

/// The source files of the Argentine leagues' error boxes.
pub(crate) const ARG_PRM_FILE: usize = 0x9873e8;
pub(crate) const ARG_SECOND_FILE: usize = 0x9874ec;

/// Rule: the Argentine Premier Division at career creation: discipline rule 3 and, with no saved state, importance 15,
/// two stages (the Apertura and the Clausura), the rules (ArgPrm::set_up_rules 0x405670; 0: the box at line 0x4e), the
/// table (ArgPrm::create_table 0x405540; line 0x55), the fixtures (`exe::league_create_fixtures`; line 0x5b), the
/// statistics (line 0x62), the two half-season stages (ArgPrm::create_stages 0x405950, unchecked), the order
/// (`exe::league_sort_table`(this, 0); line 0x6c), the relegation averages (ArgPrm::init_relegation_averages 0x405ef0)
/// and the predicted positions (`exe::league_set_predicted_positions`). Every box returns this; returns this.
/// Original: `ArgPrm::ArgPrm` (0x404250, arg_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_arg_prm(this: usize, year: u32, record: usize) -> usize {
    let this = League(this);
    if argentine_league_start(this, year, record, 0x967324, 2) {
        return this.addr();
    }
    covered!("mwf_23 argentine premier new");
    if argentine_league_call(this, 0x405670) == 0 {
        show_error_at(ARG_PRM_FILE, 0x4e);
        return this.addr();
    }
    if argentine_league_call(this, 0x405540) == 0 {
        show_error_at(ARG_PRM_FILE, 0x55);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(ARG_PRM_FILE, 0x5b);
        return this.addr();
    }
    if !argentine_league_stats(this) {
        show_error_at(ARG_PRM_FILE, 0x62);
        return this.addr();
    }
    argentine_league_call(this, 0x405950);
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(ARG_PRM_FILE, 0x6c);
        return this.addr();
    }
    argentine_league_call(this, 0x405ef0);
    exe::league_set_predicted_positions(this);
    this.addr()
}

/// Rule: the Argentine Second Division (Nacional B) at career creation: discipline rule 3 and, with no saved state,
/// importance 15, six stages (the three zones, the promotion cup and the play-offs), the rules (ArgSecond::set_up_rules
/// 0x407e50; 0: the box at line 0x4e), the table (ArgSecond::fill_league_table 0x407cd0; line 0x55), the fixtures
/// (`exe::league_create_fixtures`; line 0x5b), the order (`exe::league_sort_table`(this, 0); line 0x62), the statistics
/// (line 0x69), the relegation averages (ArgSecond::init_relegation_averages 0x4093f0) and its own predicted positions
/// (ArgSecond::set_predicted_positions 0x409fe0). The Brazilian state leagues' body (`bra_reg_goias_construct_state_league`)
/// with discipline rule 3 and the averages before the predicted positions. Every box returns this; returns this.
/// Original: `ArgSecond::ArgSecond` (0x406bf0, arg_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_arg_second(this: usize, year: u32, record: usize) -> usize {
    let this = League(this);
    if argentine_league_start(this, year, record, 0x9673d8, 6) {
        return this.addr();
    }
    covered!("mwf_23 argentine second new");
    if argentine_league_call(this, 0x407e50) == 0 {
        show_error_at(ARG_SECOND_FILE, 0x4e);
        return this.addr();
    }
    if argentine_league_call(this, 0x407cd0) == 0 {
        show_error_at(ARG_SECOND_FILE, 0x55);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(ARG_SECOND_FILE, 0x5b);
        return this.addr();
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(ARG_SECOND_FILE, 0x62);
        return this.addr();
    }
    if !argentine_league_stats(this) {
        show_error_at(ARG_SECOND_FILE, 0x69);
        return this.addr();
    }
    argentine_league_call(this, 0x4093f0);
    argentine_league_call(this, 0x409fe0);
    this.addr()
}

/// The comparer of the Argentine league constructors: only the original runs (it allocates and reads the save file);
/// checks the return, the vtable, the year, the record, the discipline rule and, for a new league with statistics, the
/// stage count, the stage array and the importance.
unsafe fn compare_argentine_league_ctor(name: &str, this: League, year: u32, record: usize, vtable: usize, stages: u32, original: impl FnOnce() -> usize) -> usize {
    let loaded = league_will_load_saved_state(record);
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == vtable && (this.year() as u16) == year as u16 && this.competition().addr() == record
        && this.discipline_rule() == discipline_rule::ARGENTINA;
    if !loaded && this.comp_stats() != 0 {
        ok = ok && this.stage_count() == stages as usize && this.stages() != 0 && this.importance() == 0xf;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}) [year {}, record {:#x}, loaded {}]: returned {:#x}, vtable {:#x}, discipline {}, stages {}, importance {}", name, this.addr(), year as u16,
        record, loaded, r, read_usize(this.addr()), this.discipline_rule(), this.stage_count(), this.importance()));
    r
}

/// Comparer for `construct_arg_prm`.
pub(crate) unsafe extern "thiscall" fn compare_construct_arg_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ARG_PRM);
    compare_argentine_league_ctor("construct_arg_prm", League(this), year, record, 0x967324, 2, || o(this, year, record))
}

/// Comparer for `construct_arg_second`.
pub(crate) unsafe extern "thiscall" fn compare_construct_arg_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ARG_SECOND);
    compare_argentine_league_ctor("construct_arg_second", League(this), year, record, 0x9673d8, 6, || o(this, year, record))
}
