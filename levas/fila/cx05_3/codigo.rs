// ---------- cx05_3: the Asian Cup stages: groups B and C, the knockout and the stage advance (asia_nations.cpp) ----------

/// The round table of stage `stage` through the competition's virtual `comp_vtable::CREATE_SCHEDULE` (push 0; push
/// &id; push &rounds; push stage; mov ecx -> thiscall, ret 0x10): (the table, the rounds, the stage id).
unsafe fn nations_stage_schedule(this: League, stage: i32) -> (usize, i16, i16) {
    let create: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut id) = (0i16, 0i16);
    let table = create(this.addr(), stage, &mut rounds, &mut id, 0);
    (table, rounds, id)
}

/// The group stage `g` of a national-team tournament, from the `team_count` teams already in the vector `teams`: the
/// round table (`nations_stage_schedule`), the stage (`exe::crt_operator_new`(0xee); League_Stage::League_Stage
/// `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58: the tournament, `team_count`, the vector, 1,
/// `.competition()`, the round table, the rounds, `.win_points()`, `.draw_points()`, `.match_category()`,
/// &`league::TIEBREAKS`, &`league::PROMOTED_COUNT`, `.year()`, g, the stage id, `.importance()`, 4, 0,
/// `.league_flags()`, −1, 0, 2; the original pushes the words and bytes with garbage in the high bytes, which the
/// constructor does not read) into `.stages()`[g]; the vector and the round table freed (`exe::crt_free`). No stage:
/// the box at line `line` of `file` and 0. Otherwise `.set_current_stage(g)`, the group draw news
/// (`exe::league_send_group_draw_news` on the stage) and 1.
pub(crate) unsafe fn nations_group_stage(this: League, g: i32, team_count: u32, teams: usize, file: usize, line: u32) -> u32 {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let (table, rounds, id) = nations_stage_schedule(this, g);
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        construct(p, this.addr(), team_count, teams, 1, this.competition().addr(), table, rounds as u16 as u32, this.win_points() as u32, this.draw_points() as u32,
            this.match_category() as u32, this.addr() + league::TIEBREAKS, this.addr() + league::PROMOTED_COUNT, this.year() as u16 as u32, g as u32, id as u16 as u32,
            this.importance() as u32, 4, 0, this.league_flags() as u16 as u32, -1, 0, 2)
    } else {
        0
    };
    write_usize(this.stages() + g as usize * 4, stage);
    exe::crt_free(teams);
    exe::crt_free(table);
    let f = read_usize(this.stages() + g as usize * 4);
    if f == 0 {
        show_error_at(file, line);
        return 0;
    }
    this.set_current_stage(g as usize);
    exe::league_send_group_draw_news(f);
    1
}

/// The knockout stage `stage` of a national-team tournament: eight teams in the vector `teams` (freed here), the
/// round table (`nations_stage_schedule`) and the stage (`exe::crt_operator_new`(0xb2); Cup_Stage::Cup_Stage
/// `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret 0x3c: the tournament, 8, the vector, the rounds, `.competition()`, the round
/// table, `.year()`, `stage`, 4, the stage id, 0x14, `seeded`, 0, 0, no seeds) into `.stages()`[stage]; the vector and
/// the round table freed. Returns the stage (0 when the new failed).
pub(crate) unsafe fn nations_knockout_stage(this: League, stage: u32, teams: usize, seeded: u32) -> usize {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let (table, rounds, id) = nations_stage_schedule(this, stage as i32);
    let p = exe::crt_operator_new(0xb2);
    let knockout = if p != 0 {
        construct(p, this.addr(), 8, teams, rounds as u16 as u32, this.competition().addr(), table, this.year() as u16 as u32, stage, 4, id as u16 as u32, 0x14, seeded, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_usize(this.stages() + stage as usize * 4, knockout);
    exe::crt_free(teams);
    exe::crt_free(table);
    knockout
}

/// The club of row `row` of a league table (`exe::league_get_table_row`, thiscall, ret 4).
unsafe fn table_row_club(league: usize, row: i32) -> usize {
    read_usize(exe::league_get_table_row(league, row) + league_table_row::CLUB)
}

/// Group g + 1 (g = 0 and 1, groups B and C) of the Asian Cup as a League_Stage: a vector of 4 (`exe::crt_malloc`(0x10);
/// out of memory: the box at line 0x2cf and 0) with drawn teams 4(g + 1) to 4(g + 1) + 3 (the list +0xee holds the
/// groups one after the other), then `nations_group_stage` with 4 teams (no stage: the box at line 0x2e5).
/// Original: `AsiaNations::create_group_stage` (0x40f8a0, asia_nations.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn asia_nations_create_group_stage(this: usize, g: i32) -> u32 {
    let teams = exe::crt_malloc(0x10);
    if teams == 0 {
        show_error_at(0x987ba0, 0x2cf);
        return 0;
    }
    covered!("cx05_3 asian cup group stage");
    let first = (g + 1) as usize * 4; // cx05_3: the group's first drawn team
    for k in 0..4usize {
        write_usize(teams + k * 4, read_usize(read_usize(this + 0xee) + (first + k) * 4));
    }
    nations_group_stage(League(this), g, 4, teams, 0x987ba0, 0x2e5)
}

/// Comparer for `asia_nations_create_group_stage`: only the original (the stage allocates, the news goes out); then,
/// when it returns 1, the plan: the stage at [+0xc][g], +0x30 = g, four teams, the cup's year and, in its table, the
/// drawn 4(g + 1) to 4(g + 1) + 3 (in any order: the constructor sorts the table).
pub(crate) unsafe extern "thiscall" fn compare_asia_nations_create_group_stage(this: usize, g: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> u32>(%%I0%%);
    let list = read_usize(this + 0xee);
    if list == 0 || !(0..2).contains(&g) {
        return o(this, g);
    }
    let mut plan: Vec<usize> = (0..4).map(|k| read_usize(list + ((g + 1) as usize * 4 + k) * 4)).collect();
    let r = o(this, g);
    if r == 1 {
        let league = League(this);
        let f = League(read_usize(league.stages() + g as usize * 4));
        let t = if !f.is_null() { f.table() } else { 0 };
        let mut read: Vec<usize> = if t != 0 { (0..4).map(|k| read_usize(t + k * league_table_row::SIZE)).collect() } else { Vec::new() };
        read.sort_unstable();
        plan.sort_unstable();
        let ok = !f.is_null() && league.current_stage() as i32 == g && f.club_count() == 4 && f.year() == league.year() && read == plan;
        SCOREBOARD.record(ok, || format!("asia_nations_create_group_stage({}): the stage does not match the plan ({:x?} read {:x?})", g, plan, read));
    }
    r
}

/// The Asian Cup knockout (stage 2): the eight quarter-finalists are the three group winners, the three runners-up and
/// the two best thirds.
///
/// A vector of 8 (`exe::crt_malloc`(0x20); out of memory: the box at line 0x305 and nothing more). The thirds are
/// ranked in a temporary League_Stage of 3 teams (`exe::crt_operator_new`(0xee); League_Stage::League_Stage: the cup,
/// 3, no list, 0, `.competition()`, no table, 0, `.win_points()`, `.draw_points()`, 0, &`league::TIEBREAKS`,
/// &`league::PROMOTED_COUNT`, `.year()`, −2, 0, 0, 1, 0, 2, −1, 0, 2; the original does not check the new): its row k
/// is a copy of row 2 (+0x76) of group k's table (group A is the cup's own table, groups B and C the stages 0 and 1),
/// with the head-to-head and position-history pointers cleared, and the table is sorted (`exe::league_sort_table`(stage,
/// 0)). The vector: A's winner, the best third, B's winner, the second third, C's winner, A's, B's and C's runners-up
/// (`exe::league_get_table_row`). Then `nations_knockout_stage`(2, not seeded) and the temporary stage destroyed
/// (virtual 0 with 1). No knockout: the box at line 0x33b. Otherwise the first round is set up
/// (`exe::cup_set_up_round`(knockout, 0)) and `.set_current_stage(2)`.
/// Original: `AsiaNations::create_knockout_stage` (0x40fa90, asia_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_nations_create_knockout_stage(this: usize) {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let league = League(this);
    let teams = exe::crt_malloc(0x20);
    if teams == 0 {
        return show_error_at(0x987ba0, 0x305);
    }
    let p = exe::crt_operator_new(0xee);
    let thirds = if p != 0 {
        construct(p, this, 3, 0, 0, league.competition().addr(), 0, 0, league.win_points() as u32, league.draw_points() as u32, 0, this + league::TIEBREAKS,
            this + league::PROMOTED_COUNT, league.year() as u16 as u32, (-2i32) as u32, 0, 0, 1, 0, 2, -1, 0, 2)
    } else {
        0
    };
    // the original reads the temporary stage's table without checking the new (a null stage faults, as here)
    for k in 0..3usize {
        let group_table = if k == 0 { league.table() } else { League(read_usize(league.stages() + (k - 1) * 4)).table() };
        let row = League(thirds).table() + k * league_table_row::SIZE;
        core::ptr::copy_nonoverlapping((group_table + 2 * league_table_row::SIZE) as *const u8, row as *mut u8, league_table_row::SIZE); // cx05_3: the third of each group
        write_usize(row + league_table_row::POSITION_HISTORY, 0);
        write_usize(row + league_table_row::H2H_ENTRIES, 0);
    }
    exe::league_sort_table(thirds, 0);
    let stage = |k: usize| read_usize(league.stages() + k * 4);
    let thirds_table = League(thirds).table();
    write_usize(teams, table_row_club(this, 0));
    write_usize(teams + 4, read_usize(thirds_table));
    write_usize(teams + 8, table_row_club(stage(0), 0));
    write_usize(teams + 0xc, read_usize(thirds_table + league_table_row::SIZE));
    write_usize(teams + 0x10, table_row_club(stage(1), 0));
    write_usize(teams + 0x14, table_row_club(this, 1));
    write_usize(teams + 0x18, table_row_club(stage(0), 1));
    write_usize(teams + 0x1c, table_row_club(stage(1), 1));
    nations_knockout_stage(league, 2, teams, 0);
    if thirds != 0 {
        let destroy: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(read_usize(read_usize(thirds) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
        destroy(thirds, 1);
    }
    let knockout = stage(2);
    if knockout == 0 {
        return show_error_at(0x987ba0, 0x33b);
    }
    covered!("cx05_3 asian cup knockout");
    exe::cup_set_up_round(knockout, 0);
    league.set_current_stage(2);
}

/// Comparer for `asia_nations_create_knockout_stage`: only the original (the stages allocate, the first round is
/// drawn); then the plan: the knockout at [+0xc][2] and +0x30 = 2.
pub(crate) unsafe extern "thiscall" fn compare_asia_nations_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    o(this);
    let league = League(this);
    let ok = league.stages() != 0 && read_usize(league.stages() + 8) != 0 && league.current_stage() as i32 == 2;
    SCOREBOARD.record(ok, || format!("asia_nations_create_knockout_stage({:#x}): stage {}", this, league.current_stage() as i32));
}

/// rule: the Asian Cup moves on one stage when the current one ends; on reaching stage 2 (after the three groups) it
/// builds the knockout; going past the last stage is an error (the box at line 0x2ad)
/// Original: `AsiaNations::advance_stage` (0x40f810, asia_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_nations_advance_stage(this: usize) {
    let knockout_at = 2; // cx05_3: the knockout after the groups
    con_merc_cup_advance_stage_cqs11_next_stage(League(this), 0x987ba0, 0x2ad, knockout_at, 0x40fa90, "cx05_3 asian cup next stage");
}

/// Comparer for `asia_nations_advance_stage`: only the original (the knockout allocates and draws), then +0x30 one up
/// (unchanged on the last stage).
pub(crate) unsafe extern "thiscall" fn compare_asia_nations_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    con_merc_cup_compare_advance_stage("asia_nations_advance_stage", League(this), || o(this))
}
