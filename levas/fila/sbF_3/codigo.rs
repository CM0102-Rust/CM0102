// ---------- sbF_3: the Greek Cup constructor and knockout stage (gre_cup.cpp) ----------

/// Rule: the Greek Cup is a cup played as a league with eleven groups of four (the cup's own group and ten group
/// stages, one stage slot more for the knockout), with 7 substitutes on the bench and 3 substitutions under the Greek
/// discipline rules; with nothing saved, its 44 entrants, the statistics, the predicted positions, the rules, the first
/// group, its fixtures and the ten group stages are made.
///
/// Over League::League (`exe::new_league`, thiscall, ret 0): `league::COMPETITION` = the record, the vtable 0x96b4b4,
/// `league::YEAR`, 3 substitutions allowed, 7 substitutes named, `discipline_rule::GREECE`, importance 0xc. If the saved
/// state loads (`exe::league_load_from_temp_file`(this, 1) ≠ 0) it returns this. Otherwise, in order:
/// `league::LEAGUE_FLAGS` = 0x28, `league::STAGE_INDEX` = `stage_index::MAIN`, `league::CURRENT_STAGE` = −1, 11 stages
/// and their array (`exe::crt_malloc` of 0x2c, unchecked); the entrants (GreCup::collect_entrants 0x5e9a80, thiscall,
/// ret 0, unchecked); the statistics (`ita_cups_create_comp_stats`; without them the box at line 0x56); the predicted
/// positions (GreCup::set_predicted_positions 0x5ea060, unchecked); the rules (GreCup::set_up_rules 0x5e9350; 0: line
/// 0x60); the first group's table (GreCup::create_table 0x5e9260; 0: line 0x67); the fixtures
/// (`exe::league_create_fixtures`; 0: line 0x6d); the order (`exe::league_sort_table`(this, 0); 0: line 0x74); the
/// stages of groups 0 to 9 (GreCup::create_group_stage 0x5e9650, push g; mov ecx -> thiscall, ret 4). Each box returns
/// this (the same body as `construct_irish_league_cup`, checked instruction by instruction against
/// NirLgeCup::NirLgeCup: only the vtable, the substitutes, the discipline rule, the stage count, the routines, the group
/// count and the box lines change). Returns this.
/// Original: `GreCup::GreCup` (0x5e88a0, gre_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_gre_cup(this: usize, year: u32, record: usize) -> usize {
    let this = League(this);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), GREEK_CUP_VTABLE);
    this.set_year(year as u16 as i16);
    this.set_substitutions_allowed(3);
    this.set_substitutes_named(7);
    this.set_discipline_rule(discipline_rule::GREECE);
    this.set_importance(0xc);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!("sbF_3 greek cup new");
    this.set_league_flags(0x28);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    this.set_stage_count(0xb); // sbF3: the ten groups and the knockout
    this.set_stages(exe::crt_malloc(0x2c));
    let call = |f: usize| {
        let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(f);
        f(this.addr())
    };
    call(0x5e9a80); // GreCup::collect_entrants
    if !ita_cups_create_comp_stats(this) {
        show_error_at(0x9cdab0, 0x56);
        return this.addr();
    }
    call(0x5ea060); // GreCup::set_predicted_positions
    if call(0x5e9350) == 0 {
        // GreCup::set_up_rules
        show_error_at(0x9cdab0, 0x60);
        return this.addr();
    }
    if call(0x5e9260) == 0 {
        // GreCup::create_table
        show_error_at(0x9cdab0, 0x67);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(0x9cdab0, 0x6d);
        return this.addr();
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(0x9cdab0, 0x74);
        return this.addr();
    }
    let group: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x5e9650usize); // GreCup::create_group_stage
    for g in 0..0xa {
        group(this.addr(), g);
    }
    this.addr()
}

/// The Greek Cup vtable.
pub(crate) const GREEK_CUP_VTABLE: usize = 0x96b4b4;

/// Only the original (it allocates, draws and reads the save file), checking the plan (`ita_cups_check_constructor_plan`:
/// this, the vtable, the record, the year and, on a new cup, the fixed fields and the statistics).
pub(crate) unsafe extern "thiscall" fn compare_construct_gre_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, year, record);
    let fields = [(league::STAGE_COUNT, 4u8, 0xb), (league::STAGE_INDEX, 1, 0xff), (league::SUBSTITUTES_NAMED, 1, 7), (league::SUBSTITUTIONS_ALLOWED, 1, 3),
        (league::DISCIPLINE_RULE, 1, discipline_rule::GREECE as u32), (league::IMPORTANCE, 1, 0xc), (league::LEAGUE_FLAGS, 2, 0x28)];
    let error = ita_cups_check_constructor_plan(this, r, year, record, GREEK_CUP_VTABLE, &fields, false);
    SCOREBOARD.record(error.is_none(), || format!("construct_gre_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// Rule: the Greek Cup knockout (stage 10) has 22 clubs: the eleven group runners-up in group order and the eleven
/// group winners shuffled (the round table: twelve clubs in the 1st round, ten more in the 2nd).
///
/// The club vector (`exe::crt_malloc` of 0x58; out of memory, the box at line 0x2a5 of gre_cup.cpp and nothing more);
/// slots 0 to 10 the second row of each group's table (the cup's own table `league::TABLE` + one row, then stages 0 to
/// 9 by League::get_table_row(stage, 1), thiscall, ret 4), slots 11 to 21 the first rows in the same order; slots 11
/// to 21 shuffled (`exe::comp_util_shuffle_pointer_array`(vector + 0x2c, 11), cdecl); then `create_knockout_stage`
/// (22 clubs, stage 10, no seeds, the box at line 0x2d3): the round table of stage 10, the Cup_Stage into stage slot
/// 10, the vector and the round table freed and the first round drawn.
/// Original: `GreCup::create_knockout_stage` (0x5e9850, gre_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_cup_create_knockout_stage(this: usize) {
    let this = League(this);
    let teams = exe::crt_malloc(0x58);
    if teams == 0 {
        return show_error_at(0x9cdab0, 0x2a5);
    }
    covered!("sbF_3 greek cup knockout");
    for (half, row) in [(0usize, 1i32), (GREEK_CUP_GROUPS, 0)] {
        // sbF3: the runners-up first, then the winners
        write_usize(teams + 4 * half, read_usize(this.table() + row as usize * league_table_row::SIZE));
        for g in 0..0xausize {
            let stage = read_usize(this.stages() + 4 * g);
            write_usize(teams + 4 * (half + g + 1), read_usize(exe::league_get_table_row(stage, row)));
        }
    }
    exe::comp_util_shuffle_pointer_array(teams + 4 * GREEK_CUP_GROUPS, GREEK_CUP_GROUPS as u32);
    create_knockout_stage(this, 0x16, teams, 0xa, core::ptr::null(), 0x9cdab0, 0x2d3);
}

/// Comparer for `gre_cup_create_knockout_stage`: only the original (it builds the stage and draws its first round);
/// then the stage in slot 10 with 22 clubs, the eleven runners-up and the eleven winners (as a set: the winners are
/// shuffled).
pub(crate) unsafe extern "thiscall" fn compare_gre_cup_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let t = League(this);
    let group = |g: usize, row: i32| if g == 0 { read_usize(t.table() + row as usize * league_table_row::SIZE) } else { read_usize(exe::league_get_table_row(read_usize(t.stages() + 4 * (g - 1)), row)) };
    let mut plan: Vec<usize> = (0..GREEK_CUP_GROUPS).flat_map(|g| [group(g, 1), group(g, 0)]).collect();
    o(this);
    let s = Cup(read_usize(t.stages() + 4 * 0xa));
    let mut ok = !s.is_null() && s.team_count() == 0x16 && s.team_table() != 0;
    if ok {
        let mut teams: Vec<usize> = (0..0x16).map(|k| read_usize(s.team_table() + k * 6)).collect();
        teams.sort_unstable();
        plan.sort_unstable();
        ok = teams == plan;
    }
    SCOREBOARD.record(ok, || format!("gre_cup_create_knockout_stage({:#x}) stage {:#x} plan {:x?}", this, s.addr(), plan));
}
