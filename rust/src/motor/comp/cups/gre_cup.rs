use super::*;

/// Rule: the Greek Cup takes 44 clubs: 16 Third Division clubs (sorted by strength, the best 48 shuffled), then the
/// Second Division clubs and the First Division clubs in database order until the list is full; short of 44, an error
/// box. Returns 1.
///
/// The entrant list (+0xf2 = 44; +0xee = `exe::crt_calloc`(4, 44), cdecl; out of memory: the box of gre_cup.cpp at line
/// 0x2f2 and 0). Over the clubs (a 16-bit index up to [`G_CLUB_COUNT`] read once): the division id of
/// `KEY_CLUB_COMP_GREEK_LOWER_DIVISION` into a local list of at most 100 (a signed byte count);
/// `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 16) and its first 16 into the entrant list (the original copies
/// 16 whatever the count: with fewer it reads stack garbage; the copy reads zeros). Then the clubs of
/// `KEY_CLUB_COMP_GREEK_SECOND_DIVISION` and of `KEY_CLUB_COMP_GREEK_PREMIER_DIVISION`, each while the count is below
/// +0xf2. Count ≠ +0xf2: the box at line 0x322. Returns 1.
/// Original: `GreCup::collect_entrants` (0x5e9a80, gre_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_cup_collect_entrants(this: usize) -> i32 {
    let mut third = [0usize; 100];
    let mut n: i8 = 0;
    write_u32(this + 0xf2, 0x2c);
    let list = exe::crt_calloc(4, 0x2c);
    write_usize(this + 0xee, list);
    if list == 0 {
        show_error_at(0x9cdab0, 0x2f2);
        return 0;
    }
    let count = read_usize(G_CLUB_COUNT) as i32;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i16;
    while (i as i32) < count {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GREEK_LOWER_DIVISION) && n < 0x64 {
            third[n as usize] = club;
            n += 1;
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    covered!("mwd_30 greek cup entrants");
    exe::comp_util_sort_and_shuffle_top_clubs(third.as_mut_ptr() as usize, n as i16 as u16 as u32, 0x10); // mwd30: the best 48 third division clubs shuffled
    for k in 0..0x10usize {
        write_usize(read_usize(this + 0xee) + 4 * k, third[k]);
    }
    let mut k = 0x10i32;
    for g in [KEY_CLUB_COMP_GREEK_SECOND_DIVISION, KEY_CLUB_COMP_GREEK_PREMIER_DIVISION] {
        let mut club = read_usize(G_CLUB_TABLE);
        let mut i = 0i16;
        while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
            let d = read_usize(club + club::DIVISION);
            if d != 0 && read_usize(d) == read_usize(g) && k < read_i32(this + 0xf2) {
                k += 1;
                write_usize(read_usize(this + 0xee) + 4 * (k - 1) as usize, club);
            }
            i = i.wrapping_add(1);
            club = club.wrapping_add(club::SIZE);
        }
    }
    if k != read_i32(this + 0xf2) {
        show_error_at(0x9cdab0, 0x322);
    }
    1
}

/// Comparer for `gre_cup_collect_entrants`: side by side with the random generator restored; +0xf2, the return and the
/// 44 entrant pointers compared; the copy's list is freed and the original's kept.
pub(crate) unsafe extern "thiscall" fn compare_gre_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_GRE_CUP_COLLECT_ENTRANTS);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this + 0xf2, 4usize), (G_MOUSE_LEFT_BUTTON_DOWN, 4), crate::rng::STATE_REGIONS[0], crate::rng::STATE_REGIONS[1]];
    let (a, b, same, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        lo.set(read_usize(this + 0xee));
        r
    }, || {
        let r = gre_cup_collect_entrants(this);
        lr.set(read_usize(this + 0xee));
        write_usize(this + 0xee, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let list = if lo != 0 && lr != 0 { (0..0x2c).all(|k| read_usize(lo + 4 * k) == read_usize(lr + 4 * k)) } else { lo == 0 && lr == 0 };
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(same && a == b && list, || format!("gre_cup_collect_entrants({:#x}) returned {} x {} memory {} list {}", this, a, b, same, list));
    a
}

/// Rule: the Greek Cup's eleven groups of four (the cup's own group and stages 0 to 9) play six matchdays from 11
/// August to 8 September, each group a league stage (is_league = 1, label 0x3e9 + stage); the knockout (stage 10) has
/// 22 clubs: 12 in the 1st round (December), 10 more in the 2nd (January), then two-legged quarter- and semi-finals
/// and the final on 10 May.
///
/// `create_schedule` with the knockout at stage 10, the label 0x3e9, `GREEK_CUP_GROUP_ROUNDS`,
/// `GREEK_CUP_KNOCKOUT_ROUNDS` and the boxes at lines 0xc3 and 0xee of gre_cup.cpp (checked by emulation of the
/// original for every stage −128..127 in an odd and an even year: the same calls and the same 16 fields per round as
/// ItaCup::create_schedule). Above stage 10: 0.
/// Original: `GreCup::create_schedule` (0x5e8bf0, gre_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn gre_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_schedule(League(this), stage, rounds, extra, league, 0xa, 0x3e9, &GREEK_CUP_GROUP_ROUNDS, &GREEK_CUP_KNOCKOUT_ROUNDS, 0x9cdab0, 0xc3, 0xee,
        "sbF_2 greek cup groups", "sbF_2 greek cup knockout rounds")
}

/// Comparer for `gre_cup_create_schedule` (side by side: `ita_cup_compare_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_gre_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_GRE_CUP_CREATE_SCHEDULE);
    ita_cup_compare_schedule("gre_cup_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| gre_cup_create_schedule(this, stage, r, e, l), 0xa)
}

/// Rule: the Greek Cup's own group (the cup is the league of its first group) takes the four clubs of column 0 of the
/// entrant pool; the group draw news follows.
///
/// `league::CLUB_COUNT` = 4; the table (`exe::crt_malloc` of 0xec; out of memory, the box at line 0x1ab of gre_cup.cpp
/// and 0); row k gets pool entry 11 × k (`exe::league_init_table_row`(this, k, club, 0, 0); the original pushes the
/// dword of the local byte with stack garbage above it, the routine reads only the byte; the index is a signed byte
/// compared with the word `league::CLUB_COUNT`); unlike the Welsh and Irish cups the pool entry is not cleared. Then
/// the group draw news (League::send_group_draw_news, thiscall, ret 0). Returns 1.
/// Original: `GreCup::create_table` (0x5e9260, gre_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_cup_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(4);
    let t = exe::crt_malloc(0xec);
    this.set_table(t);
    if t == 0 {
        show_error_at(0x9cdab0, 0x1ab);
        return 0;
    }
    covered!("sbF_2 greek cup first group");
    let mut k: i8 = 0;
    while (k as i16) < this.club_count() {
        exe::league_init_table_row(this, k as u8 as u32, greek_cup_pool_club(this, 0, k as usize), 0, 0);
        k = k.wrapping_add(1);
    }
    exe::league_send_group_draw_news(this);
    1
}

/// Comparer for `gre_cup_create_table`: the plan (column 0 of the pool), then only the original (`init_table_row`
/// allocates and the news goes to the managers); returns 1, four clubs, the table rows in pool order and the pool
/// untouched.
pub(crate) unsafe extern "thiscall" fn compare_gre_cup_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GRE_CUP_CREATE_TABLE);
    let t = League(this);
    let plan: Vec<usize> = (0..4).map(|k| greek_cup_pool_club(t, 0, k)).collect();
    let r = o(this);
    let found: Vec<usize> = if t.table() != 0 { (0..4).map(|k| read_usize(t.table() + k * league_table_row::SIZE)).collect() } else { Vec::new() };
    let ok = r == 1 && t.club_count() == 4 && found == plan && (0..4).all(|k| greek_cup_pool_club(t, 0, k) == plan[k]);
    SCOREBOARD.record(ok, || format!("gre_cup_create_table({:#x}) returned {}; plan {:x?} table {:x?}", this, r, plan, found));
    r
}

/// Rule: Greek Cup group stage g (0 to 9) is a league stage of the four clubs of pool column g + 1 (they stay in the
/// pool), played over the cup's group schedule; the group draw news follows and the current stage becomes g.
///
/// `build_cup_group_stage` with `GREEK_CUP_GROUP_STAGE`, club k from pool entry g + 1 + 11 × k, not cleared (against
/// WelPrmCup::create_group_stage the pool stride is 11, the entries are not cleared and the 4th argument of
/// League_Stage::League_Stage is 1).
/// Original: `GreCup::create_group_stage` (0x5e9650, gre_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn gre_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let this = League(this);
    build_cup_group_stage(this, g, &GREEK_CUP_GROUP_STAGE, |k| read_usize(cup_pool_entry(this, g as usize + 1 + k * GREEK_CUP_GROUPS)), // sbF2: the Greek group columns
        "sbF_2 greek cup group stage")
}

/// Comparer for `gre_cup_create_group_stage`: only the original (it builds a stage with its fixtures and sends the
/// news); then the return 1, the stage in slot g with four clubs and the cup's year, the current stage g, the stage
/// table holding pool column g + 1 (in any order: the constructor sorts it) and the pool untouched.
pub(crate) unsafe extern "thiscall" fn compare_gre_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_GRE_CUP_CREATE_GROUP_STAGE);
    let t = League(this);
    let mut plan: Vec<usize> = (0..4).map(|k| greek_cup_pool_club(t, g as usize + 1, k)).collect();
    let r = o(this, g);
    let f = League(read_usize(t.stages() + 4 * g as usize));
    let mut ok = r == 1 && !f.is_null() && t.current_stage() as u32 == g && f.club_count() == 4 && f.year() == t.year()
        && (0..4).all(|k| greek_cup_pool_club(t, g as usize + 1, k) == plan[k]);
    if ok && f.table() != 0 {
        let mut found: Vec<usize> = (0..4).map(|k| read_usize(f.table() + k * league_table_row::SIZE)).collect();
        found.sort_unstable();
        plan.sort_unstable();
        ok = found == plan;
    }
    SCOREBOARD.record(ok, || format!("gre_cup_create_group_stage({:#x}, {}) returned {}, stage {:#x}, plan {:x?}", this, g, r, f.addr(), plan));
    r
}

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
    let group: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(FN_GRECUP_CREATE_GROUP_STAGE); // GreCup::create_group_stage
    for g in 0..0xa {
        group(this.addr(), g);
    }
    this.addr()
}

/// Only the original (it allocates, draws and reads the save file), checking the plan (`ita_cups_check_constructor_plan`:
/// this, the vtable, the record, the year and, on a new cup, the fixed fields and the statistics).
pub(crate) unsafe extern "thiscall" fn compare_construct_gre_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_GRE_CUP)(this, year, record);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_GRE_CUP_CREATE_KNOCKOUT_STAGE);
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

/// The Greek Cup rules: `set_cup_league_rules` with `GREEK_CUP_GROUP_RULES` and one meeting (checked instruction by
/// instruction against NirLgeCup::set_up_rules: only the tie-break order and the prize change), then the round table.
/// Returns 1.
/// Original: `GreCup::set_up_rules` (0x5e9350, gre_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_cup_set_up_rules(this: usize) -> u32 {
    covered!("cx02_18 greek cup rules");
    set_cup_league_rules(League(this), &GREEK_CUP_GROUP_RULES, 1)
}

/// The Greek Cup turnover: `start_irish_league_cup_season` with GreCup::collect_entrants (0x5e9a80), GreCup::create_table
/// (0x5e9260), GreCup::create_group_stage (0x5e9650) for the ten groups, the year up by one and the boxes at lines
/// 0x21c, 0x223, 0x229 and 0x230 of gre_cup.cpp (checked instruction by instruction against
/// NirLgeCup::start_new_season: only the routines, the group count and the lines change).
/// Original: `GreCup::start_new_season` (0x5e93f0, gre_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_cup_start_new_season(this: usize) -> u32 {
    covered!("cx02_18 greek cup turnover");
    start_irish_league_cup_season(League(this), 0x5e9a80, 0x5e9260, 0x5e9650, 10, 0x9cdab0, [0x21c, 0x223, 0x229, 0x230], 1)
}

/// The Greek Cup predicted positions: `italian_cup_predicted_positions` with the knockout at stage 10, bands of 11 and
/// the boxes at lines 0x3f0 and 0x3f8 of gre_cup.cpp (checked instruction by instruction against
/// ItaCup::set_predicted_positions: only the file, the lines, the stage and the band change).
/// Original: `GreCup::set_predicted_positions` (0x5ea060, gre_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_cup_set_predicted_positions(this: usize) {
    italian_cup_predicted_positions(this, 0x9cdab0, (0x3f0, 0x3f8), 0xa, 0xb, "cx02_18 greek cup predicted")
}

pub(crate) unsafe extern "thiscall" fn compare_gre_cup_set_up_rules(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GRE_CUP_SET_UP_RULES)(this);
    check_cup_league_rules("gre_cup_set_up_rules", League(this), &GREEK_CUP_GROUP_RULES, r);
    r
}

pub(crate) unsafe extern "thiscall" fn compare_gre_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GRE_CUP_START_NEW_SEASON);
    compare_state_league_new_season("gre_cup_start_new_season", League(this), || o(this))
}

pub(crate) unsafe extern "thiscall" fn compare_gre_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_GRE_CUP_SET_PREDICTED_POSITIONS);
    compare_italian_cup_predicted("gre_cup_set_predicted_positions", this, gre_cup_set_predicted_positions, || o(this));
}

/// The Greek Cup moves on to its next stage: past the last stage the box at line 0x24a of gre_cup.cpp and nothing
/// else; otherwise the current stage goes up by one and, on reaching stage 10, the knockout stage is built
/// (GreCup::create_knockout_stage 0x5e9850) (`welsh_cup_advance_stage`, checked instruction by instruction against
/// WelPrmCup::advance_stage: only the file, the line and the stage change).
/// Original: `GreCup::advance_stage` (0x5e95c0, gre_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_cup_advance_stage(this: usize) {
    welsh_cup_advance_stage(League(this), 0x9cdab0, 0x24a, 10, 0x5e9850)
}

/// Comparer for `gre_cup_advance_stage`: only the original runs (the knockout builds a stage); the current stage stays
/// at the last stage, otherwise goes up by one.
pub(crate) unsafe extern "thiscall" fn compare_gre_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_GRE_CUP_ADVANCE_STAGE);
    compare_state_league_advance_stage("gre_cup_advance_stage", League(this), || o(this))
}

/// Pool entry `k` of Greek Cup group `group` (the entrant vector `ita_cup_fields::ENTRANT_CLUBS`, column `group`).
pub(crate) unsafe fn greek_cup_pool_club(this: League, group: usize, k: usize) -> usize {
    read_usize(cup_pool_entry(this, group + k * GREEK_CUP_GROUPS))
}

/// Builds group stage `g` of a cup played as a league, in the original routine's order (the body of
/// WelPrmCup::create_group_stage, GreCup::create_group_stage, HolCup::create_group_stage and
/// KorLeagueCup::create_group_b_stage, compared instruction by instruction: only the values of `CupGroupStage` and how
/// the clubs leave the entrant pool change).
///
/// The round table of the stage (virtual `comp_vtable::CREATE_SCHEDULE`(g, &rounds, &label, 0)); the club list
/// (`exe::crt_malloc` of 4 × clubs; null: the box of the first line and 0) with `take`(k) for k = 0.. clubs − 1 (the
/// club of slot k, taken from the pool by the caller's rule); a League_Stage (`exe::crt_operator_new`(0xee) and
/// League_Stage::League_Stage 0x68b100, thiscall, ret 0x58: this cup as the parent, the club count, the list, `arg4`,
/// `league::COMPETITION`, the round table and its count, the win, draw and category bytes, the tie-breaks
/// (`league::TIEBREAKS`), a local tie-break list (`tiebreak`, 0, 0, 0), the year, g, the label, the importance, 1,
/// `arg18`, the league flags, −1, 0, 2) into stage slot g (0 without memory); the list and the round table freed; no
/// stage: the box of the second line and 0; otherwise the group draw news of the stage (League::send_group_draw_news),
/// the current stage = g and 1. The original pushes the round count, the label, the year and the flags as dwords with
/// stale high bytes; League_Stage::League_Stage reads the words and bytes.
pub(crate) unsafe fn build_cup_group_stage(this: League, g: u32, p: &CupGroupStage, mut take: impl FnMut(usize) -> usize, ramo: &'static str) -> u32 {
    let tiebreaks: [u8; 4] = [p.tiebreak, 0, 0, 0];
    let (mut rounds, mut label) = (0i16, 0i16);
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let table = schedule(this.addr(), g, &mut rounds, &mut label, 0);
    let clubs = exe::crt_malloc(4 * p.clubs);
    if clubs == 0 {
        show_error_at(p.file, p.rows.0);
        return 0;
    }
    covered!(ramo);
    for k in 0..p.clubs as usize {
        write_usize(clubs + 4 * k, take(k));
    }
    let n = exe::crt_operator_new(0xee);
    let mut s = 0usize;
    if n != 0 {
        type LeagueStageCtor = unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize;
        let ctor: LeagueStageCtor = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        s = ctor(n, this.addr(), p.clubs, clubs, p.arg4, this.competition().addr(), table, rounds as u16 as u32, this.win_points() as u32, this.draw_points() as u32,
            this.match_category() as u32, this.addr() + league::TIEBREAKS, tiebreaks.as_ptr() as usize, this.year() as u16 as u32, g, label as u16 as u32,
            this.importance() as u32, 1, p.arg18, (this.league_flags() as u16) as u32, 0xffff_ffff, 0, 2);
    }
    write_usize(this.stages() + 4 * g as usize, s);
    exe::crt_free(clubs);
    exe::crt_free(table);
    let s = read_usize(this.stages() + 4 * g as usize);
    if s == 0 {
        show_error_at(p.file, p.rows.1);
        return 0;
    }
    exe::league_send_group_draw_news(s);
    this.set_current_stage(g as usize);
    1
}

/// The address of entrant pool entry `i` (`ita_cup_fields::ENTRANT_CLUBS`, a vector of club pointers).
pub(crate) unsafe fn cup_pool_entry(this: League, i: usize) -> usize {
    read_usize(this.addr() + ita_cup_fields::ENTRANT_CLUBS) + 4 * i
}

/// The Greek Cup groups: six matchdays from 11 August to 8 September (11, 18, 22, 25 and 29 August and 8 September;
/// weekday 2 with code 2, or weekday 6 with code 1).
pub(crate) const GREEK_CUP_GROUP_ROUNDS: [ItaCupGroupRound; 6] = [(0xb, 7, 2, 2), (0x12, 7, 2, 2), (0x16, 7, 6, 1), (0x19, 7, 2, 2), (0x1d, 7, 6, 1), (8, 8, 2, 2)]; // sbF2: the Greek Cup group matchdays

/// The Greek Cup knockout: the 1st round drawn on 16 September and played on 15 December (12 clubs, 6 ties, all new),
/// the 2nd drawn 16 December and played 13 January (16 clubs, 8 ties, 10 new from entry 12), the quarter-finals drawn
/// 14 January and played over two legs 14 days apart from 24 February, the semi-finals drawn 8 March and played over
/// two legs from 5 April, the final drawn 20 April and played on 10 May at a neutral ground [inference: venue 4];
/// single-match tie rules 3; category 4 up to the quarter-finals and 6 after; no prize per round (−1).
pub(crate) const GREEK_CUP_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 5] = [
    cql10r((0x10, 8, 0, 3), (0xf, 11, 0, 2, 2, 0), 0x14, (0, 3, 0), 4, (0xc, 6, 0xc, 0), (0, 1, 0), (0, 0xffff_ffff, 0)), // sbF2: the Greek Cup 1st round
    cql10r((0x10, 11, 0, 3), (0xd, 0, 1, 2, 2, 0), 0x1e, (0, 3, 0), 4, (0x10, 8, 0xa, 0xc), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0xe, 0, 1, 3), (0x18, 1, 1, 2, 2, 0), 0x78, (0, 0, 3), 4, (8, 4, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((8, 2, 1, 3), (5, 3, 1, 2, 2, 0), 0x82, (0, 0, 3), 6, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x14, 3, 1, 3), (0xa, 4, 1, 2, 2, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// The Greek Cup entrant pool: 44 clubs in eleven columns of four (entry `group + 11 × k` is club k of group `group`;
/// group 0 is the cup's own group, group g + 1 the group stage g). The pool is read, never cleared: each column is used
/// once.
pub(crate) const GREEK_CUP_GROUPS: usize = 0xb;

/// The Greek Cup group stages: four clubs, the local tie-break 2, 1 as the 4th argument, the boxes at lines 0x271 and
/// 0x286 of gre_cup.cpp.
pub(crate) const GREEK_CUP_GROUP_STAGE: CupGroupStage = CupGroupStage { clubs: 4, tiebreak: 2, arg4: 1, arg18: 0, file: 0x9cdab0, rows: (0x271, 0x286) };

/// The Greek Cup vtable.
pub(crate) const GREEK_CUP_VTABLE: usize = 0x96b4b4;

/// The Greek Cup groups: 3 points for a win and 1 for a draw, category 4 (national cup), the tie-breaks 4, 1, 2 (the
/// head-to-head first [inference]), two places through per group, a 600,000 prize fund.
pub(crate) const GREEK_CUP_GROUP_RULES: GroupStageRules = GroupStageRules { win: 3, draw: 1, category: 4, tiebreak: [4, 1, 2, 0], slots: 2, prize: Some(0x927c0), bits: None };

/// How a cup played as a league builds one of its group stages (`build_cup_group_stage`): the club count, the first
/// local tie-break, the 4th and the 18th arguments of League_Stage::League_Stage (1 or 2, and 0 or 0x82 [inference:
/// the places through and the stage's round name]), the source file and the two error-box lines.
pub(crate) struct CupGroupStage {
    pub(crate) clubs: u32,
    pub(crate) tiebreak: u8,
    pub(crate) arg4: u32,
    pub(crate) arg18: u32,
    pub(crate) file: usize,
    pub(crate) rows: (u32, u32),
}
