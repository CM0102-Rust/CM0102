// ---------- rx10_14: the Club World Championship second and third groups (world_club_champ.cpp) ----------

/// Builds group stage `k` (0 or 1) of the Club World Championship from the four clubs in `list`: the round table
/// (virtual `comp_vtable::CREATE_SCHEDULE`(k, &rounds, &extra, 0)), a League_Stage (`exe::crt_operator_new`(0xee);
/// League_Stage::League_Stage: the championship, 4, the list, 1 leg, the competition, the round table and its rounds,
/// the win, draw and category bytes, the tie-breaks, the places (`league::PROMOTED_COUNT`), the year, stage k, the
/// extra as its label, the importance, 2, 0, the rule bits `league::LEAGUE_FLAGS`, −1, 0 and 2) into stage slot k, and
/// the round table freed. Returns the stage (0 when none).
pub(crate) unsafe fn world_club_champ_group_stage(this: usize, k: u32, list: usize) -> usize {
    let l = League(this);
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = schedule(this, k, &mut rounds, &mut extra, 0);
    let e = exe::crt_operator_new(0xee);
    let s = if e != 0 {
        let ctor: LeagueStageCtorFn = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        ctor(e, this, 4, list, 1, l.competition().addr(), table, rounds as u16 as u32, l.win_points() as u32, l.draw_points() as u32, l.match_category() as u32,
            this + league::TIEBREAKS, this + league::PROMOTED_COUNT, l.year() as u16 as u32, k, extra as u16 as u32, l.importance() as u32, 2, 0,
            l.league_flags() as u16 as u32, -1, 0, 2)
    } else {
        0
    };
    write_usize(l.stages() + 4 * k as usize, s);
    exe::crt_free(table);
    s
}

/// Rule: the Club World Championship's second and third groups are the second and the third club of each of the four
/// seeding pots (entrants 1, 4, 7, 10 and 2, 5, 8, 11), single round robin, each with its draw news.
///
/// `WorldClubChamp::create_second_and_third_groups()` (SEH frame only for the stages' new). The list
/// (`exe::crt_malloc`(0x10); none: the world_club_champ.cpp box at line 0x302 and 0) with entrants 3k + 1 of the list
/// +0xee; group stage 0 (`world_club_champ_group_stage`); none: the box at line 0x318 and 0 (the list is not freed);
/// otherwise `league::CURRENT_STAGE` = 0 and League::send_group_draw_news on the stage
/// (`exe::league_send_group_draw_news`). Then the same list with entrants 3k + 2 and group stage 1 (box at line 0x332),
/// `league::CURRENT_STAGE` = 1, its news, the list freed and 1. (The first group is the championship's own table:
/// WorldClubChamp::draw_first_group, 0x929aa0.)
/// Original: `WorldClubChamp::create_second_and_third_groups` (0x929e60, world_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_club_champ_create_second_and_third_groups(this: usize) -> u32 {
    let l = League(this);
    let list = exe::crt_malloc(0x10);
    if list == 0 {
        show_error_at(0xad5764, 0x302);
        return 0;
    }
    covered!("rx10_14 club world championship other groups");
    for (k, line) in [(0u32, 0x318u32), (1, 0x332)] {
        for pot in 0..4usize {
            write_usize(list + 4 * pot, read_usize(read_usize(this + 0xee) + 4 * (3 * pot + 1 + k as usize))); // rx10_14: the k-th next club of each pot
        }
        let s = world_club_champ_group_stage(this, k, list);
        if s == 0 {
            show_error_at(0xad5764, line);
            return 0;
        }
        l.set_current_stage(k as usize);
        exe::league_send_group_draw_news(s);
    }
    exe::crt_free(list);
    1
}

/// Comparer for `world_club_champ_create_second_and_third_groups`: only the original (it allocates the stages and sends
/// news), with the plan: 1, stage slots 0 and 1 hold four clubs each, entrants 3k + 1 and 3k + 2 of the list (in any
/// order), and the current stage is 1.
pub(crate) unsafe extern "thiscall" fn compare_world_club_champ_create_second_and_third_groups(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    let l = League(this);
    let list = read_usize(this + 0xee);
    let group = |k: usize| -> bool {
        let s = read_usize(l.stages() + 4 * k);
        if s == 0 || League(s).club_count() != 4 {
            return false;
        }
        let mut got: Vec<usize> = (0..4).map(|row| read_usize(League(s).table() + row * league_table_row::SIZE)).collect();
        let mut plan: Vec<usize> = (0..4).map(|pot| read_usize(list + 4 * (3 * pot + 1 + k))).collect();
        got.sort_unstable();
        plan.sort_unstable();
        got == plan
    };
    let ok = r == 0 || (group(0) && group(1) && l.current_stage() as i32 == 1);
    SCOREBOARD.record(ok, || format!("world_club_champ_create_second_and_third_groups({:#x}): returned {}, current {}", this, r, l.current_stage() as i32));
    r
}
