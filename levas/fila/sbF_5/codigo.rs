// ---------- sbF_5: the Korean League Cup group B and knockout stages (kor_league_cup.cpp) ----------

/// The Korean League Cup group B: five clubs, the local tie-break 2, 2 as the 4th argument and 0x82 as the 18th of
/// League_Stage::League_Stage, the boxes at lines 0x245 and 0x25b of kor_league_cup.cpp.
pub(crate) const KOREAN_LEAGUE_CUP_GROUP_B: CupGroupStage = CupGroupStage { clubs: 5, tiebreak: 2, arg4: 2, arg18: 0x82, file: KOR_LEAGUE_CUP_FILE, rows: (0x245, 0x25b) };

/// Rule: the Korean League Cup group B is a league stage of the five odd seeds (list places 1, 3, 5, 7 and 9), which
/// leave the list, played over the cup's group schedule; the group draw news follows.
///
/// `build_cup_group_stage` for stage 0 with `KOREAN_LEAGUE_CUP_GROUP_B`, club k from list place 1 + 2k, zeroed (checked
/// instruction by instruction against WelPrmCup::create_group_stage: the stage is always 0, the argument is not read;
/// five clubs with stride 2 and the 18th argument 0x82).
/// Original: `KorLeagueCup::create_group_b_stage` (0x66cb60, kor_league_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn kor_league_cup_create_group_b_stage(this: usize, _g: u32) -> u32 {
    let this = League(this);
    build_cup_group_stage(this, 0, &KOREAN_LEAGUE_CUP_GROUP_B, |k| {
        let e = cup_pool_entry(this, 1 + 2 * k); // sbF5: the odd seeds
        let c = read_usize(e);
        write_usize(e, 0);
        c
    }, "sbF_5 korean league cup group b")
}

/// Comparer for `kor_league_cup_create_group_b_stage`: the plan (the odd seeds), then only the original (it builds a
/// stage with its fixtures and sends the news); returns 1, the stage in slot 0 with five clubs and the cup's year, the
/// current stage 0, the stage table holding the planned clubs (in any order: the constructor sorts it) and their list
/// places zeroed.
pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_create_group_b_stage(this: usize, g: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I0%%);
    let t = League(this);
    let list = read_usize(this + ita_cup_fields::ENTRANT_CLUBS);
    let mut plan: Vec<usize> = (0..5).map(|k| read_usize(list + 4 * (1 + 2 * k))).collect();
    let r = o(this, g);
    let f = League(read_usize(t.stages()));
    let mut ok = r == 1 && !f.is_null() && t.current_stage() == 0 && f.club_count() == 5 && f.year() == t.year() && (0..5).all(|k| read_usize(list + 4 * (1 + 2 * k)) == 0);
    if ok && f.table() != 0 {
        let mut found: Vec<usize> = (0..5).map(|k| read_usize(f.table() + k * league_table_row::SIZE)).collect();
        found.sort_unstable();
        plan.sort_unstable();
        ok = found == plan;
    }
    SCOREBOARD.record(ok, || format!("kor_league_cup_create_group_b_stage({:#x}, {}) returned {}, stage {:#x}, plan {:x?}", this, g, r, f.addr(), plan));
    r
}

/// Rule: the Korean League Cup semi-finals pair each group winner with the other group's runner-up: group A's winner
/// against group B's runner-up and group A's runner-up against group B's winner; the first draw is made at once.
///
/// The club vector (`exe::crt_malloc` of 0x10; out of memory, the box at line 0x279 and nothing more): slot 0 the first
/// row of the cup's own table (group A), slot 1 the second row of the group B stage's table (stage slot 0, read
/// directly), slot 2 group A's second row, slot 3 group B's first row; then `create_knockout_stage` (4 clubs, stage 1,
/// no seeds, the box at line 0x290): the round table of stage 1, the Cup_Stage into stage slot 1, the vector and the
/// round table freed and the first round drawn (checked instruction by instruction against
/// NirLgeCup::create_knockout_stage: the vector filled by hand and the stage, the lines and the file change).
/// Original: `KorLeagueCup::create_knockout_stage` (0x66cd50, kor_league_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_league_cup_create_knockout_stage(this: usize) {
    let this = League(this);
    let teams = exe::crt_malloc(0x10);
    if teams == 0 {
        return show_error_at(KOR_LEAGUE_CUP_FILE, 0x279);
    }
    covered!("sbF_5 korean league cup knockout");
    let group_b = League(read_usize(this.stages())).table();
    write_usize(teams, read_usize(this.table()));
    write_usize(teams + 4, read_usize(group_b + league_table_row::SIZE)); // sbF5: group B's runner-up meets group A's winner
    write_usize(teams + 8, read_usize(this.table() + league_table_row::SIZE));
    write_usize(teams + 0xc, read_usize(group_b));
    create_knockout_stage(this, 4, teams, 1, core::ptr::null(), KOR_LEAGUE_CUP_FILE, 0x290);
}

/// Comparer for `kor_league_cup_create_knockout_stage`: the plan (the two winners and runners-up), then only the
/// original (it builds the stage and draws its first round); the stage in slot 1 with four clubs, the planned ones (as a
/// set: the draw places them).
pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let t = League(this);
    let group_b = League(read_usize(t.stages())).table();
    let row = |table: usize, r: usize| read_usize(table + r * league_table_row::SIZE);
    let mut plan = vec![row(t.table(), 0), row(group_b, 1), row(t.table(), 1), row(group_b, 0)];
    o(this);
    let s = Cup(read_usize(t.stages() + 4));
    let mut ok = !s.is_null() && s.team_count() == 4 && s.team_table() != 0;
    if ok {
        let mut teams: Vec<usize> = (0..4).map(|k| read_usize(s.team_table() + k * 6)).collect();
        teams.sort_unstable();
        plan.sort_unstable();
        ok = teams == plan;
    }
    SCOREBOARD.record(ok, || format!("kor_league_cup_create_knockout_stage({:#x}) stage {:#x} plan {:x?}", this, s.addr(), plan));
}
