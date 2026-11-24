// ---------- cx07_2: the Copa América knockout and the progress codes (goldcup.cpp) ----------

/// The file name of goldcup.cpp for the error boxes.
const GOLDCUP_SOURCE: usize = 0x9cd640;

/// The knockout stage `stage` of a national-team tournament from the `count` teams in the vector `teams`: the round
/// table through the virtual `comp_vtable::CREATE_SCHEDULE` (push 0; push &id; push &rounds; push stage; mov ecx ->
/// thiscall, ret 0x10), the stage (`exe::crt_operator_new`(0xb2); Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`,
/// thiscall, ret 0x3c: the tournament, `count`, the vector, the rounds, `.competition()`, the round table, `.year()`,
/// `stage`, 4, the stage id, 0x14, 0, 0, 0, 0; the words with garbage in the high bytes, which the constructor does not
/// read) into `.stages()`[stage]; the vector and the round table freed (`exe::crt_free`). No stage: the box at line
/// `line` of `file`. Otherwise its first round is set up (`exe::cup_set_up_round`(stage, 0)) and
/// `.set_current_stage(stage)`.
pub(crate) unsafe fn nations_knockout_from(this: League, stage: u32, count: u32, teams: usize, file: usize, line: u32) {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let create: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut id) = (0i16, 0i16);
    let table = create(this.addr(), stage, &mut rounds, &mut id, 0);
    let p = exe::crt_operator_new(0xb2);
    let knockout = if p != 0 {
        construct(p, this.addr(), count, teams, rounds as u16 as u32, this.competition().addr(), table, this.year() as u16 as u32, stage, 4, id as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(this.stages() + stage as usize * 4, knockout);
    exe::crt_free(teams);
    exe::crt_free(table);
    let knockout = read_usize(this.stages() + stage as usize * 4);
    if knockout == 0 {
        return show_error_at(file, line);
    }
    exe::cup_set_up_round(knockout, 0);
    this.set_current_stage(stage as usize);
}

/// The plan of a knockout builder that only the original runs: the stage at `.stages()`[stage] and
/// `.current_stage()` = stage.
pub(crate) unsafe fn check_nations_knockout(name: &str, this: League, stage: usize) {
    let ok = this.stages() != 0 && read_usize(this.stages() + stage * 4) != 0 && this.current_stage() == stage;
    SCOREBOARD.record(ok, || format!("{}({:#x}): stage {} (knockout expected at {})", name, this.addr(), this.current_stage() as i32, stage));
}

/// The table of group `g` of the Copa América: group A (−1) is the cup's own table, B and C the stages 0 and 1.
unsafe fn copa_america_group(this: League, g: i8) -> usize {
    if g == -1 { this.addr() } else { read_usize(this.stages() + g as usize * 4) }
}

/// The Copa América knockout (stage 2): the eight quarter-finalists are the three group winners, the three
/// runners-up and the two best thirds, in a shuffled draw.
///
/// The thirds are ranked in a temporary League_Stage of 3 teams (`exe::crt_operator_new`(0xee); League_Stage::
/// League_Stage `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58: the cup, 3, no list, 0, `.competition()`, no table,
/// 0, `.win_points()`, `.draw_points()`, 0, &`league::TIEBREAKS`, &`league::PROMOTED_COUNT`, `.year()`, −2, 0, 0, 1, 0,
/// 2, −1, 0, 2; the new is not checked): its row g + 1 is a copy of row 2 of group g's table with the position history
/// pointer cleared, and the table is sorted (`exe::league_sort_table`(stage, 0)). A vector of 8 (`exe::crt_malloc`(0x20);
/// out of memory: the box at line 0x30b and nothing more): A's winner and runner-up, B's, C's, then the two best thirds.
/// The worst third goes out of the statistics (the cup's virtual `comp_vtable::SET_CLUB_PROGRESS`(club, −1, 3, 3, 3)).
/// Each group then updates its positions and statuses (its virtual `league_vtable::UPDATE_POSITIONS_AND_STATUS`,
/// thiscall, ret 0) and announces through its own virtual `comp_vtable::ANNOUNCE_TEAM_STATUS` (5 pushes, thiscall):
/// its third, when one of the two best, through (6, one more place in its `league::PROMOTED_COUNT`), otherwise out
/// (−2); its fourth always out (−2). The temporary stage is destroyed (virtual 0 with 1), the vector shuffled
/// (`exe::comp_util_shuffle_pointer_array`(vector, 8), cdecl) and the knockout built (`nations_knockout_from`, stage
/// 2, 8 teams; no stage: the box at line 0x35e).
/// Original: `GoldCup::create_knockout_stage` (0x5e18a0, goldcup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn goldcup_create_knockout_stage(this: usize) {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let league = League(this);
    let p = exe::crt_operator_new(0xee);
    let thirds = if p != 0 {
        construct(p, this, 3, 0, 0, league.competition().addr(), 0, 0, league.win_points() as u32, league.draw_points() as u32, 0, this + league::TIEBREAKS,
            this + league::PROMOTED_COUNT, league.year() as u16 as u32, (-2i32) as u32, 0, 0, 1, 0, 2, -1, 0, 2)
    } else {
        0
    };
    // the original reads the temporary stage's table without checking the new (a null stage faults, as here)
    for g in -1i8..2 {
        let row = League(thirds).table() + (g + 1) as usize * league_table_row::SIZE;
        let third = League(copa_america_group(league, g)).table() + 2 * league_table_row::SIZE; // cx07_2: the third of each group
        core::ptr::copy_nonoverlapping(third as *const u8, row as *mut u8, league_table_row::SIZE);
        write_usize(row + league_table_row::POSITION_HISTORY, 0);
    }
    exe::league_sort_table(thirds, 0);
    let teams = exe::crt_malloc(0x20);
    if teams == 0 {
        return show_error_at(GOLDCUP_SOURCE, 0x30b);
    }
    let mut k = 0usize;
    for g in -1i8..2 {
        let group = copa_america_group(league, g);
        for r in 0..2 {
            write_usize(teams + k * 4, table_row_club(group, r));
            k += 1;
        }
    }
    let best_thirds = 2; // cx07_2: the best thirds that go through
    for r in 0..best_thirds {
        write_usize(teams + (6 + r as usize) * 4, table_row_club(thirds, r));
    }
    covered!("cx07_2 copa america quarter-finalists");
    let progress: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_CLUB_PROGRESS));
    progress(this, table_row_club(thirds, 2), -1, 3, 3, 3);
    for g in -1i8..2 {
        let group = copa_america_group(league, g);
        let update: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(group) + league_vtable::UPDATE_POSITIONS_AND_STATUS));
        update(group);
        let announce: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(group) + comp_vtable::ANNOUNCE_TEAM_STATUS));
        // simplification: the original pushes a dword whose low byte is the group and whose high bytes are stack
        // leftovers; the announcement reads the byte
        let third = table_row_club(group, 2);
        if third != table_row_club(thirds, 0) && third != table_row_club(thirds, 1) {
            announce(group, third, 0xfe, g as u8 as u32, 0, 0);
        } else {
            covered!("cx07_2 third through");
            write_u8(group + league::PROMOTED_COUNT, read_u8(group + league::PROMOTED_COUNT).wrapping_add(1));
            announce(group, third, 6, g as u8 as u32, 0, 0);
        }
        announce(group, table_row_club(group, 3), 0xfe, g as u8 as u32, 0, 0);
    }
    if thirds != 0 {
        let destroy: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(read_usize(read_usize(thirds) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
        destroy(thirds, 1);
    }
    exe::comp_util_shuffle_pointer_array(teams, 8);
    nations_knockout_from(league, 2, 8, teams, GOLDCUP_SOURCE, 0x35e);
}

/// Comparer for `goldcup_create_knockout_stage`: only the original (the stages allocate, the draw is shuffled and the
/// first round drawn, the statuses announced); then the plan (`check_nations_knockout`).
pub(crate) unsafe extern "thiscall" fn compare_goldcup_create_knockout_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    check_nations_knockout("goldcup_create_knockout_stage", League(this), 2);
}

/// Stores a nation's progress in the Copa América statistics (`group_championship_progress` of cx04_5): in the group
/// stages (below 2) current 1, 2→5, 3→9, 4→10; best 1, 2, 3→1, 4→10; worst 1, 2→5, 3→9, 4→10; afterwards as they
/// come.
/// Original: `GoldCup::set_club_progress` (0x5e2500, goldcup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn goldcup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx07_2 copa america progress");
    group_championship_progress(this, club, stage, current, best, worst, 2, [[5, 5, 9, 0xa], [1, 1, 1, 0xa], [5, 5, 9, 0xa]]); // cx07_2: the Copa América codes
}

/// Comparer for `goldcup_set_club_progress`: side by side on the 3 bytes of the nation's entry
/// (`compare_group_championship_progress` of cx04_5).
pub(crate) unsafe extern "thiscall" fn compare_goldcup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I1%%);
    compare_group_championship_progress("goldcup_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || goldcup_set_club_progress(this, club, stage, current, best, worst));
}
