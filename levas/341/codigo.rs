// ---------- cx16_10: the CONCACAF qualifying stages and play-off, and the European runner-up for the Asian play-off (wc_concacaf_cup.cpp, wc_europe_league.cpp) ----------

/// WcConcacafCup routines that build a stage (thiscall, ret 0): the play-off, the semi-final groups and the final group.
pub(crate) const FN_WC_CONCACAF_CUP_CREATE_PLAY_OFF_STAGE: usize = 0x917110;
pub(crate) const FN_WC_CONCACAF_CUP_CREATE_SEMI_FINAL_GROUPS: usize = 0x917320;
pub(crate) const FN_WC_CONCACAF_CUP_CREATE_FINAL_GROUP: usize = 0x917830;

/// rule: the CONCACAF qualifying moves on one stage at a time: to stage 2 the play-off, to 3 the three semi-final
/// groups, to 6 the final group (stages 4 and 5 are the other semi-final groups, built with stage 3); past the last
/// stage it is an error
///
/// At or past the last stage (`cup::STAGE_COUNT` − 1, signed), the error box of line 0x5d7. Otherwise
/// `cup::CURRENT_STAGE` + 1 and, by the new stage, WcConcacafCup::create_play_off_stage (2),
/// create_semi_final_groups (3) or create_final_group (6), thiscall, ret 0, called directly.
/// Original: `WcConcacafCup::advance_stage` (0x917060, wc_concacaf_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_advance_stage(this: usize) {
    let cup = Cup(this);
    let current = cup.current_stage() as i32;
    if current >= (cup.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(wc_cup_turnover_consts::WC_CONCACAF_CUP_CPP, 0x5d7);
    }
    let next = current + 1;
    cup.set_current_stage(next as usize);
    covered!("cx16_10 concacaf next stage");
    let build = match next {
        2 => FN_WC_CONCACAF_CUP_CREATE_PLAY_OFF_STAGE,
        3 => FN_WC_CONCACAF_CUP_CREATE_SEMI_FINAL_GROUPS, // cx16_10: the semi-final groups at stage 3
        6 => FN_WC_CONCACAF_CUP_CREATE_FINAL_GROUP,
        _ => return,
    };
    call_section_routine(build, this);
}

/// Comparer for `wc_concacaf_cup_advance_stage`: it builds stages, so only the original runs; the stage stays or moves
/// on by one.
pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_advance_stage(this: usize) {
    let before = Cup(this).current_stage() as i32;
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_CONCACAF_CUP_ADVANCE_STAGE)(this);
    let after = Cup(this).current_stage() as i32;
    SCOREBOARD.record(after == before || after == before + 1, || format!("wc_concacaf_cup_advance_stage({:#x}): stage {} -> {}", this, before, after));
}

/// The six nations of the CONCACAF play-off, taken without side effects: the first seeded entrant left in the list's
/// last ten slots (+0xb2, slots 25 to 34; the original then empties that slot), the Caribbean knockout survivors (the
/// cup's team table rows with status 2, in order) and the runners-up of the two Central American groups (row 1 of the
/// tables of stages 0 and 1). Returns the nations and the seeded slot taken.
pub(crate) unsafe fn concacaf_play_off_nations(this: usize) -> (Vec<usize>, Option<usize>) {
    let cup = Cup(this);
    let list = read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST);
    let seeded = (0x19..0x23).find(|&s| read_usize(list + s * 4) != 0);
    let mut v: Vec<usize> = seeded.map(|s| read_usize(list + s * 4)).into_iter().collect();
    for i in 0..cup.team_count().max(0) as usize {
        let row = cup.team_table() + i * 6;
        if read_u8(row + 5) == 2 {
            v.push(read_usize(row)); // a knockout survivor
        }
    }
    for k in 0..2 {
        let group = League(read_usize(cup.stages() + k * 4));
        v.push(read_usize(group.table() + league_table_row::SIZE)); // cx16_10: the group runner-up
    }
    (v, seeded)
}

/// rule: the CONCACAF play-off is a cup stage of six: the next seeded nation, the Caribbean knockout survivors and the
/// two Central American group runners-up
///
/// A team array (`exe::crt_malloc` of 0x18, cdecl; null, the error box of line 0x5ff) with `concacaf_play_off_nations`
/// (the seeded slot emptied); the round table of stage 2 (virtual `comp_vtable::CREATE_SCHEDULE`(2, &rounds, &label, 0),
/// thiscall, ret 0x10); the stage (operator new of 0xb2 and Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`, thiscall,
/// ret 0x3c: the section, 6, the teams, the rounds, the record, the round table, the year, 2, 4, the label, the
/// importance bias `cup::IMPORTANCE_BIAS`, 0, 0, 0, 0; the original pushes the words and the byte with garbage in the
/// high bytes, which the constructor does not read: the copy passes clean values) into `cup::STAGES`[2]; the array and
/// the round table freed (`exe::crt_free`); without a stage, the box of line 0x629; with it, `cup::CURRENT_STAGE` 2 and
/// its first round set up (Cup::set_up_round `exe::cup_set_up_round`(stage, 0), thiscall, ret 4). The SEH frame covers
/// only the stage's new.
/// Original: `WcConcacafCup::create_play_off_stage` (0x917110, wc_concacaf_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_create_play_off_stage(this: usize) {
    let cpp = wc_cup_turnover_consts::WC_CONCACAF_CUP_CPP;
    let cup = Cup(this);
    let teams = exe::crt_malloc(0x18);
    if teams == 0 {
        return show_error_at(cpp, 0x5ff);
    }
    covered!("cx16_10 concacaf play-off");
    let (nations, seeded) = concacaf_play_off_nations(this);
    if let Some(s) = seeded {
        write_usize(read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST) + s * 4, 0);
    }
    for (k, &n) in nations.iter().enumerate() {
        write_usize(teams + k * 4, n);
    }
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut label) = (0i16, 0i16);
    let table = create_schedule(this, 2, &mut rounds, &mut label, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, 6, teams, rounds as u16 as u32, read_usize(this + league::COMPETITION), table, cup.year() as u16 as u32, 2, 4, label as u16 as u32, cup.importance_bias() as u32, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(cup.stages() + 8, stage);
    exe::crt_free(teams);
    exe::crt_free(table);
    if stage == 0 {
        return show_error_at(cpp, 0x629);
    }
    cup.set_current_stage(2);
    exe::cup_set_up_round(stage, 0);
}

/// Comparer for `wc_concacaf_cup_create_play_off_stage`: it builds a stage, so only the original runs; then the plan:
/// the current stage 2 and the stage's team table (6-byte rows) with the planned nations in order.
pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_create_play_off_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_CONCACAF_CUP_CREATE_PLAY_OFF_STAGE);
    let cup = Cup(this);
    if read_usize(this + 0xb2) == 0 || cup.stages() == 0 || read_usize(cup.stages()) == 0 || read_usize(cup.stages() + 4) == 0 {
        return o(this);
    }
    let (plan, _) = concacaf_play_off_nations(this);
    o(this);
    let stage = read_usize(cup.stages() + 8);
    if stage == 0 {
        return;
    }
    let t = Cup(stage).team_table();
    let read: Vec<usize> = if t != 0 { (0..plan.len()).map(|k| read_usize(t + k * 6)).collect() } else { Vec::new() };
    SCOREBOARD.record(cup.current_stage() == 2 && read == plan, || format!("wc_concacaf_cup_create_play_off_stage({:#x}): {:x?} plan {:x?}", this, read, plan));
}

/// rule: the European nation that plays the Asian section is the best-ranked group runner-up who is not in the
/// European play-off
///
/// The runners-up stage (`cup::STAGES`[+0x100], signed byte; its table `league::TABLE`, `league_table_row::SIZE`-byte
/// rows, `league::CLUB_COUNT`) in order; the first whose club is not in the play-off stage's team table
/// (`cup::STAGES`[+0x101]: `cup::TEAM_TABLE`, 6-byte rows, `cup::TEAM_COUNT`). None: 0.
/// Original: `WcEuropeLeague::get_asia_play_off_team` (0x91def0, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_get_asia_play_off_team(this: usize) -> usize {
    let stages = League(this).stages();
    let runners = League(read_usize(stages + (read_u8(this + 0x100) as i8 as isize as usize).wrapping_mul(4)));
    let play_off = Cup(read_usize(stages + (read_u8(this + 0x101) as i8 as isize as usize).wrapping_mul(4)));
    covered!("cx16_10 european asian play-off team");
    for i in 0..runners.club_count().max(0) as usize {
        let club = read_usize(runners.table() + i * league_table_row::SIZE);
        let n = play_off.team_count();
        let mut k = 0i16;
        while k < n && read_usize(play_off.team_table() + k as usize * 6) != club {
            k += 1;
        }
        if k == n {
            return club; // cx16_10: the first runner-up not in the play-off
        }
    }
    0
}

/// Comparer for `wc_europe_league_get_asia_play_off_team`: it only reads; side by side on the result.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_get_asia_play_off_team(this: usize) -> usize {
    let a = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_WC_EUROPE_LEAGUE_GET_ASIA_PLAY_OFF_TEAM)(this);
    let b = wc_europe_league_get_asia_play_off_team(this);
    SCOREBOARD.record(a == b, || format!("wc_europe_league_get_asia_play_off_team({:#x}) original={:#x} rust={:#x}", this, a, b));
    a
}

// ---------- cx16_11: the European World Cup qualifying stages: moving on, the runners-up stage and the section's own group (wc_europe_league.cpp) ----------

/// WcEuropeLeague routines called directly (thiscall): the play-off stage (ret 0) and the group size (ret 4).
pub(crate) const FN_WC_EUROPE_LEAGUE_CREATE_PLAY_OFF_STAGE: usize = 0x919910;
pub(crate) const FN_WC_EUROPE_LEAGUE_GET_GROUP_SIZE: usize = 0x918dd0;

/// rule: the European qualifying moves on one stage at a time; reaching the runners-up stage or the play-off stage
/// builds the play-off; past the last stage it is an error
///
/// At or past the last stage (`league::STAGE_COUNT` − 1, signed), the error box of line 0x37f. Otherwise
/// `league::CURRENT_STAGE` + 1 and, when the new stage is the runners-up stage (+0x100) or the play-off stage (+0x101),
/// signed bytes, WcEuropeLeague::create_play_off_stage (0x919910, thiscall, ret 0, called directly).
/// Original: `WcEuropeLeague::advance_stage` (0x919620, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_advance_stage(this: usize) {
    let league = League(this);
    let current = league.current_stage() as i32;
    if current >= (league.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x37f);
    }
    let next = current + 1;
    league.set_current_stage(next as usize);
    covered!("cx16_11 european next stage");
    if next == read_u8(this + 0x100) as i8 as i32 || next == read_u8(this + 0x101) as i8 as i32 { // cx16_11: the play-off when the runners-up stage comes
        call_section_routine(FN_WC_EUROPE_LEAGUE_CREATE_PLAY_OFF_STAGE, this);
    }
}

/// Comparer for `wc_europe_league_advance_stage`: it builds stages, so only the original runs; the stage stays or moves
/// on by one.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_advance_stage(this: usize) {
    let before = League(this).current_stage() as i32;
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_EUROPE_LEAGUE_ADVANCE_STAGE)(this);
    let after = League(this).current_stage() as i32;
    SCOREBOARD.record(after == before || after == before + 1, || format!("wc_europe_league_advance_stage({:#x}): stage {} -> {}", this, before, after));
}

/// rule: the European runners-up are ranked in a league stage of their own, one runner-up per group (the last group
/// index + 2), with the section's points, category and tie-breaks 1, 2, 5, its best placed going through
///
/// `league::PROMOTED_COUNT` 0; the stage (`exe::crt_operator_new` of 0xee and League_Stage::League_Stage
/// `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58: the section, the last group (+0xff, signed) + 2, no teams, 0, the
/// record, no round table, 0 rounds, `league::WIN_POINTS`, `league::DRAW_POINTS`, `league::MATCH_CATEGORY`, the
/// tie-breaks {1, 2, 5, 0}, the places at `league::PROMOTED_COUNT` (the section's own four bytes), the year, the
/// runners-up stage index (+0x100), the label 0x3fc, importance 0, 4, 0, flags 2, −1, 0, 2; the original pushes the count,
/// the bytes and the year with garbage in the high bytes, which the constructor does not read: the copy passes clean
/// values) into `league::STAGES`[+0x100] (null not checked); `league::PROMOTED_COUNT` 1; `league::CURRENT_STAGE` the
/// runners-up stage index; then the table filled (WcEuropeLeague::fill_runners_up_table
/// `exe::wceuropeleague_fill_runners_up_table`, thiscall, ret 0). The SEH frame covers only the stage's new.
/// Original: `WcEuropeLeague::create_runners_up_stage` (0x91a140, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_create_runners_up_stage(this: usize) {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let league = League(this);
    let tiebreaks: [u8; 4] = [1, 2, 5, 0];
    let n = (read_u8(this + 0xff) as i8 as i16).wrapping_add(2); // cx16_11: one runner-up per group
    let index = read_u8(this + 0x100) as i8;
    league.set_promoted_count(0);
    covered!("cx16_11 european runners-up stage");
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        construct(p, this, n as u16 as u32, 0, 0, league.competition().addr(), 0, 0, league.win_points() as u32, league.draw_points() as u32, league.match_category() as u32,
            tiebreaks.as_ptr() as usize, this + league::PROMOTED_COUNT, league.year() as u16 as u32, index as u8 as u32, 0x3fc, 0, 4, 0, 2, -1, 0, 2)
    } else {
        0
    };
    write_usize(league.stages().wrapping_add((index as isize as usize).wrapping_mul(4)), stage);
    league.set_promoted_count(1);
    league.set_current_stage(index as isize as usize);
    exe::wceuropeleague_fill_runners_up_table(this);
}

/// Comparer for `wc_europe_league_create_runners_up_stage`: it builds a stage, so only the original runs; then the
/// plan: the current stage is the runners-up stage, which exists with one row per group.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_create_runners_up_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_EUROPE_LEAGUE_CREATE_RUNNERS_UP_STAGE)(this);
    let league = League(this);
    let index = read_u8(this + 0x100) as i8;
    let stage = League(read_usize(league.stages().wrapping_add((index as isize as usize).wrapping_mul(4))));
    let n = (read_u8(this + 0xff) as i8 as i16).wrapping_add(2);
    let ok = league.current_stage() as i32 == index as i32 && !stage.is_null() && stage.club_count() == n;
    SCOREBOARD.record(ok, || format!("wc_europe_league_create_runners_up_stage({:#x}): stage {} index {}, {} rows of {}", this, league.current_stage() as i32, index, if stage.is_null() { -1 } else { stage.club_count() as i32 }, n));
}

/// rule: the first European qualifying group is the section itself: its size by the exempt count (group −1), its
/// nations from the first drawn slots, and the group draw news
///
/// `league::CLUB_COUNT` = WcEuropeLeague::get_group_size(−1) (0x918dd0, thiscall, ret 4); `league::TABLE` = a table of
/// that many `league_table_row::SIZE`-byte rows (`exe::crt_malloc`; out of memory, the error box of line 0x2b2 and 0);
/// `league::GAMES_PER_CLUB` = (count − 1) × `league::LEGS` (words); row i with the drawn slot i (+0xf7, a pointer to the
/// slot table) (`exe::league_init_table_row`(this, i, team, 0, 0), thiscall, ret 0x10; the original pushes i with
/// garbage in the high bytes: the routine reads the byte); the group draw news (`exe::league_send_group_draw_news`); 1.
/// Original: `WcEuropeLeague::init_own_group_table` (0x9190d0, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_init_own_group_table(this: usize) -> u32 {
    let league = League(this);
    let size: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(FN_WC_EUROPE_LEAGUE_GET_GROUP_SIZE);
    league.set_club_count(size(this, u32::MAX));
    let t = exe::crt_malloc((league.club_count() as i32 as u32).wrapping_mul(league_table_row::SIZE as u32));
    league.set_table(t);
    if t == 0 {
        show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x2b2);
        return 0;
    }
    league.set_games_per_club(league.club_count().wrapping_sub(1).wrapping_mul(league.legs()));
    covered!("cx16_11 european own group");
    let slots = read_usize(this + 0xf7); // the drawn group slots
    let mut i = 0i8;
    while (i as i16) < league.club_count() {
        exe::league_init_table_row(league, i as u8 as u32, read_usize(slots + i as isize as usize * 4), 0, 0); // cx16_11: slot i in row i
        i = i.wrapping_add(1);
    }
    exe::league_send_group_draw_news(league);
    1
}

/// Comparer for `wc_europe_league_init_own_group_table`: it allocates the table and sends news, so only the original
/// runs; then the plan: 1, the group size and each row with its drawn slot in order.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_init_own_group_table(this: usize) -> u32 {
    let slots = read_usize(this + 0xf7);
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_EUROPE_LEAGUE_INIT_OWN_GROUP_TABLE)(this);
    let league = League(this);
    if r != 1 || slots == 0 || league.table() == 0 {
        return r;
    }
    let n = league.club_count().clamp(0, 16) as usize;
    let rows: Vec<usize> = (0..n).map(|k| read_usize(league.table() + k * league_table_row::SIZE)).collect();
    let plan: Vec<usize> = (0..n).map(|k| read_usize(slots + k * 4)).collect();
    SCOREBOARD.record(rows == plan, || format!("wc_europe_league_init_own_group_table({:#x}): rows {:x?} plan {:x?}", this, rows, plan));
    r
}

// ---------- cx16_12: the European World Cup qualifying table update and its play-off qualifiers (wc_europe_league.cpp) ----------

/// WcEuropeLeague::mark_play_off_qualifiers and League::update_positions_and_status (thiscall, ret 0), called directly
/// by the table update (the second through its detour).
pub(crate) const FN_WC_EUROPE_LEAGUE_MARK_PLAY_OFF_QUALIFIERS: usize = 0x91a7e0;
pub(crate) const FN_LEAGUE_UPDATE_POSITIONS_AND_STATUS: usize = 0x684cd0;

/// rule: the European qualifying table update is the league's, and once the whole qualifying is complete the play-off
/// winners are marked as qualified (once)
///
/// League::update_positions_and_status (`FN_LEAGUE_UPDATE_POSITIONS_AND_STATUS`, through its detour); then, while the
/// done flag (`wc_cup_turnover_consts::EUROPE_FB`) is 0 and the section is complete (virtual `comp_vtable::CHECK_COMPLETE`
/// with 1, 0: push 0; push 1; mov ecx -> thiscall, ret 8; al), WcEuropeLeague::mark_play_off_qualifiers (0x91a7e0).
/// Original: `WcEuropeLeague::update_positions_and_status` (0x91a7b0, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_update_positions_and_status(this: usize) {
    call_section_routine(FN_LEAGUE_UPDATE_POSITIONS_AND_STATUS, this);
    if read_u32(this + wc_cup_turnover_consts::EUROPE_FB) != 0 {
        return;
    }
    let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CHECK_COMPLETE));
    if complete(this, 1, 0) != 0 {
        covered!("cx16_12 european qualifying complete");
        call_section_routine(FN_WC_EUROPE_LEAGUE_MARK_PLAY_OFF_QUALIFIERS, this);
    }
}

/// Comparer for `wc_europe_league_update_positions_and_status`: the table update cannot run twice; only the original.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_update_positions_and_status(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_EUROPE_LEAGUE_UPDATE_POSITIONS_AND_STATUS)(this)
}

/// The European group table of group `g` (−1 the section itself, else `league::STAGES`[g]).
pub(crate) unsafe fn wc_europe_group(this: usize, g: i32) -> League {
    if g == -1 { League(this) } else { League(read_usize(League(this).stages().wrapping_add((g as isize as usize).wrapping_mul(4)))) }
}

/// rule: when the European qualifying ends, the play-off winners get the qualified status (7) in their group tables;
/// without a play-off stage the best group runner-up qualifies instead, with its staff record
///
/// Once (the done flag `wc_cup_turnover_consts::EUROPE_FB` set to 1 on the first call; set already, nothing). Without a
/// play-off stage (+0x101 = −1): the top club of the runners-up stage's table (`league::STAGES`[+0x100], row 0) is
/// looked for in the group tables −1 to the last group (+0xff), in order; found, its row's `league_table_row::STATUS`
/// is 7 and the section's staff records for it (virtual `comp_vtable::UPDATE_STAFF_RECORDS_FOR_STATUS`(club, 7, group,
/// 0, 0, 0): thiscall, ret 0x18); not found, the error box of line 0x65f. With one: for each group, each play-off team
/// row (`cup::TEAM_TABLE`, 6-byte rows, `cup::TEAM_COUNT` of the play-off stage) whose status (+5) is 1, its row in the
/// group (League::get_table_row `exe::league_get_table_row`, thiscall, ret 4) gets the status 7.
/// Original: `WcEuropeLeague::mark_play_off_qualifiers` (0x91a7e0, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_mark_play_off_qualifiers(this: usize) {
    use wc_cup_turnover_consts as wt;
    if read_u32(this + wt::EUROPE_FB) != 0 {
        return;
    }
    write_u32(this + wt::EUROPE_FB, 1);
    let last = read_u8(this + 0xff) as i8 as i32;
    let play_off = read_u8(this + 0x101) as i8;
    if play_off == -1 {
        let runners = wc_europe_group(this, read_u8(this + 0x100) as i8 as i32);
        let best = read_usize(runners.table());
        for g in -1..=last {
            let group = wc_europe_group(this, g);
            for k in 0..group.club_count().max(0) as usize {
                let row = group.table() + k * league_table_row::SIZE;
                if read_usize(row) == best {
                    covered!("cx16_12 european best runner-up qualifies");
                    write_u8(row + league_table_row::STATUS, 7);
                    let records: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32 =
                        core::mem::transmute(read_usize(read_usize(this) + comp_vtable::UPDATE_STAFF_RECORDS_FOR_STATUS));
                    records(this, best, 7, g as u32, 0, 0, 0);
                    return;
                }
            }
        }
        return show_error_at(wt::WC_EUROPE_LEAGUE_CPP, 0x65f);
    }
    covered!("cx16_12 european play-off qualifiers");
    for g in -1..=last {
        let group = wc_europe_group(this, g);
        let mut i = 0i32;
        loop {
            let stage = Cup(wc_europe_group(this, read_u8(this + 0x101) as i8 as i32).addr());
            if i >= stage.team_count() as i32 {
                break;
            }
            let team = stage.team_table() + i as usize * 6;
            if read_u8(team + 5) == 1 { // cx16_12: a play-off winner
                let mut k = 0i32;
                while k < group.club_count() as i32 {
                    let row = exe::league_get_table_row(group, k);
                    if read_usize(row) == read_usize(team) {
                        write_u8(exe::league_get_table_row(group, k) + league_table_row::STATUS, 7);
                        break;
                    }
                    k += 1;
                }
            }
            i += 1;
        }
    }
}

/// Comparer for `wc_europe_league_mark_play_off_qualifiers`: it runs once (the flag) and adds staff records, so only the
/// original runs; then the plan: the flag set and, with a play-off stage, every winner's row in its group with status 7.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_mark_play_off_qualifiers(this: usize) {
    let first = read_u32(this + wc_cup_turnover_consts::EUROPE_FB) == 0;
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_EUROPE_LEAGUE_MARK_PLAY_OFF_QUALIFIERS)(this);
    let play_off = read_u8(this + 0x101) as i8;
    if !first || play_off == -1 {
        return;
    }
    let stage = Cup(wc_europe_group(this, play_off as i32).addr());
    let last = read_u8(this + 0xff) as i8 as i32;
    let mut missing = Vec::new();
    for i in 0..stage.team_count().max(0) as usize {
        let team = stage.team_table() + i * 6;
        if read_u8(team + 5) != 1 {
            continue;
        }
        let marked = (-1..=last).any(|g| {
            let group = wc_europe_group(this, g);
            (0..group.club_count().max(0) as usize).any(|k| {
                let row = group.table() + k * league_table_row::SIZE;
                read_usize(row) == read_usize(team) && read_u8(row + league_table_row::STATUS) == 7
            })
        });
        if !marked {
            missing.push(read_usize(team));
        }
    }
    SCOREBOARD.record(read_u32(this + wc_cup_turnover_consts::EUROPE_FB) == 1 && missing.is_empty(), || format!("wc_europe_league_mark_play_off_qualifiers({:#x}): winners not marked {:x?}", this, missing));
}

// ---------- cx16_7: the club progress codes of the CONCACAF, European and Oceania World Cup qualifying (wc_concacaf_cup.cpp, wc_europe_league.cpp, wc_oceania_league.cpp) ----------

/// The CONCACAF progress codes (current, best, worst) of a nation at `stage` (signed byte), with `exempt` (+0xba) and
/// `others` (+0xbc) the exempt counts. The knockout (stage −1, or stage 0 with fewer than two exempt non-Caribbean
/// nations): best 1 for a winner (1) else 13, current and worst 13. A Central American or semi-final group (stages
/// below 3 other than the knockout, with fewer than two exempt others; stage 3 too, then; or below 3 with exactly two):
/// 1 and 2 become 1 for the best and 6 for the current and worst, 3 becomes 7 and 4 becomes 10. The final group (stage 4
/// with fewer than two exempt others, or 3 with exactly two): codes up to 3 − exempt become 1. Any other, as they came.
pub(crate) fn wc_concacaf_progress_codes(stage: i8, exempt: i8, others: i8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    let knockout = (stage == -1 || stage == 0) && (stage == -1 || others < 2);
    if knockout {
        return (0xd, if best == 1 { 1 } else { 0xd }, 0xd);
    }
    let group = (stage != 0 && stage < 4 && others < 2) || (stage < 3 && others == 2);
    if group {
        let band = |v: u8, top: u8| match v {
            1 | 2 => top,
            3 => 7,
            4 => 0xa,
            _ => v,
        };
        return (band(current, 6), band(best, 1), band(worst, 6)); // cx16_7: the group bands
    }
    let last = (stage == 4 && others < 2) || (stage == 3 && others == 2);
    if last {
        let top = 3 - exempt as i32;
        let q = |v: u8| if v as i32 > top { v } else { 1 };
        return (q(current), q(best), q(worst));
    }
    (current, best, worst)
}

/// rule: the CONCACAF qualifying progress of a nation: a knockout winner is best 1, the rest 13; in a group the first
/// two are 1 at best and 6 now, third 7 and fourth 10; in the final group the qualifying places (three less the exempt)
/// are 1
///
/// The codes of `wc_concacaf_progress_codes` (by +0xba and +0xbc, signed bytes) into the nation's statistics entry
/// (`exe::comp_stats_find_club_entry`, not checked for null; `write_progress_codes` of the Minas Gerais state league).
/// Original: `WcConcacafCup::set_club_progress` (0x918040, wc_concacaf_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx16_7 concacaf progress");
    let e = exe::comp_stats_find_club_entry(read_usize(this + league::COMP_STATS), club);
    let (c, b, w) = wc_concacaf_progress_codes(stage as i8, read_u8(this + 0xba) as i8, read_u8(this + 0xbc) as i8, current as u8, best as u8, worst as u8);
    write_progress_codes(e, c, b, w);
}

/// The European progress codes (current, best, worst) at `stage` (signed byte). In a group (up to the last group stage,
/// +0xff): 1 → 1, 2 → 16 (the best 10), 3 → 19, 4 → 28, 5 → 37, 6 → 46. In the play-off stage (+0x101): 1 → 11, 2 → 16.
/// Any other stage, as they came. Defect of the original [inference]: a group runner-up's best becomes 10 where its
/// current and worst become 16.
pub(crate) unsafe fn wc_europe_progress_codes(this: usize, stage: i8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    if stage <= read_u8(this + 0xff) as i8 {
        let band = |v: u8, second: u8| match v {
            1 => 1,
            2 => second,
            3 => 0x13,
            4 => 0x1c,
            5 => 0x25,
            6 => 0x2e,
            _ => v,
        };
        return (band(current, 0x10), band(best, 0xa), band(worst, 0x10)); // cx16_7: the European group bands
    }
    if stage == read_u8(this + 0x101) as i8 {
        let play_off = |v: u8| match v {
            1 => 0xb,
            2 => 0x10,
            _ => v,
        };
        return (play_off(current), play_off(best), play_off(worst));
    }
    (current, best, worst)
}

/// rule: the European qualifying progress of a nation: in a group the winner is 1, the runner-up 16 (10 at best), then
/// 19, 28, 37 and 46; in the play-off the winner 11 and the loser 16
///
/// The codes of `wc_europe_progress_codes` into the nation's statistics entry (`exe::comp_stats_find_club_entry`, not
/// checked for null; `write_progress_codes` of the Minas Gerais state league).
/// Original: `WcEuropeLeague::set_club_progress` (0x91b510, wc_europe_league.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx16_7 european progress");
    let e = exe::comp_stats_find_club_entry(read_usize(this + league::COMP_STATS), club);
    let (c, b, w) = wc_europe_progress_codes(this, stage as i8, current as u8, best as u8, worst as u8);
    write_progress_codes(e, c, b, w);
}

/// The Oceania progress codes (current, best, worst) at `stage` (signed byte): from stage 1 on, as they came. In the
/// groups (below 1): the best 1, 2 → 1, 3 → 5, 4 → 7, 5 → 9, 6 → 11; the worst 1, 2 → 3, 3 → 5; the current 1 → 1, 2 → 3,
/// 3 → 8. Defects of the original [inference]: a worst of 4, 5 or 6 and a current of 4, 5 or 6 are not mapped; the
/// best becomes 7, 9 or 11 instead (the worst's case first, then the current's, the last one written winning).
pub(crate) fn wc_oceania_progress_codes(stage: i8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    if stage >= 1 {
        return (current, best, worst);
    }
    let mut b = match best {
        1 | 2 => 1,
        3 => 5,
        4 => 7,
        5 => 9,
        6 => 0xb,
        _ => best,
    };
    let mut w = worst;
    match worst {
        1 | 2 => w = 3,
        3 => w = 5,
        4 => b = 7, // the original writes the best
        5 => b = 9,
        6 => b = 0xb,
        _ => {}
    }
    let mut c = current;
    match current {
        1 => c = 1,
        2 => c = 3, // cx16_7: the Oceania group runner-up
        3 => c = 8,
        4 => b = 7,
        5 => b = 9,
        6 => b = 0xb,
        _ => {}
    }
    (c, b, w)
}

/// rule: the Oceania qualifying group progress of a nation: the best 1 for the top two, then 5, 7, 9 and 11; the
/// current 1, 3 and 8 for the first three; later stages keep their codes
///
/// The codes of `wc_oceania_progress_codes` into the nation's statistics entry (`exe::comp_stats_find_club_entry`, not
/// checked for null; `write_progress_codes` of the Minas Gerais state league).
/// Original: `WcOceaniaLeague::set_club_progress` (0x9200e0, wc_oceania_league.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx16_7 oceania progress");
    let e = exe::comp_stats_find_club_entry(read_usize(this + league::COMP_STATS), club);
    let (c, b, w) = wc_oceania_progress_codes(stage as i8, current as u8, best as u8, worst as u8);
    write_progress_codes(e, c, b, w);
}

/// Side-by-side comparer of a progress writer: the three codes of the nation's statistics entry (only the original when
/// there is no entry or no statistics).
pub(crate) unsafe fn compare_progress_codes(name: &str, this: usize, club: usize, stage: u32, codes: (u32, u32, u32), o: impl FnOnce(), c: impl FnOnce()) {
    let stats = read_usize(this + league::COMP_STATS);
    let e = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if e == 0 {
        return o();
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(e + comp_stats_club::CURRENT_POSITION, 3)], o, c);
    SCOREBOARD.record(same, || format!("{}({:#x}, {:#x}, stage {}, {:?}) original={:x?} rust={:x?}", name, this, club, stage as u8 as i8, codes, mo, mr));
}

pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_WC_CONCACAF_CUP_SET_CLUB_PROGRESS);
    compare_progress_codes("wc_concacaf_cup_set_club_progress", this, club, stage, (current, best, worst), || o(this, club, stage, current, best, worst),
        || wc_concacaf_cup_set_club_progress(this, club, stage, current, best, worst))
}

pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_WC_EUROPE_LEAGUE_SET_CLUB_PROGRESS);
    compare_progress_codes("wc_europe_league_set_club_progress", this, club, stage, (current, best, worst), || o(this, club, stage, current, best, worst),
        || wc_europe_league_set_club_progress(this, club, stage, current, best, worst))
}

pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_WC_OCEANIA_LEAGUE_SET_CLUB_PROGRESS);
    compare_progress_codes("wc_oceania_league_set_club_progress", this, club, stage, (current, best, worst), || o(this, club, stage, current, best, worst),
        || wc_oceania_league_set_club_progress(this, club, stage, current, best, worst))
}

// ---------- cx16_8: the qualification status of a nation in the European, CONCACAF and Oceania World Cup qualifying (wc_europe_league.cpp, wc_oceania_league.cpp) ----------

/// A nation's competition stat `stat` in a qualifying section's statistics (Comp_Stats::get_club_stat
/// `exe::comp_stats_get_club_stat`(`league::COMP_STATS`, club, stat): push stat; push club; mov ecx -> thiscall, ret 8,
/// a float), truncated (`ftol`), its low byte signed.
pub(crate) unsafe fn qualifying_stat_byte(this: usize, club: usize, stat: u32) -> i8 {
    ftol(exe::comp_stats_get_club_stat(read_usize(this + league::COMP_STATS), club, stat)) as u8 as i8
}

/// rule: a nation's status in the European qualifying is still open (−2) while its stat 0x12 (its current progress
/// code) is below 12, and decided (−3) from 12 on
///
/// `qualifying_stat_byte`(club, 0x12) below 12: −2, otherwise −3 (a word).
/// Original: `WcEuropeLeague::get_qualification_status` (0x91b6e0, wc_europe_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_get_qualification_status(this: usize, club: usize) -> i16 {
    covered!("cx16_8 european qualification status");
    if qualifying_stat_byte(this, club, 0x12) < 0xc { -2 } else { -3 } // cx16_8: open below 12
}

/// rule: in the European qualifying a nation's status changes down (−1) when its progress code (stat 0x12) falls below
/// 12 from 12 or more (stat 0x13, the previous code), up (1) when it rises from below 12 to 12 or more, otherwise 0
///
/// `qualifying_stat_byte` of 0x13 (before) and 0x12 (now), signed: now below 12, (before below 12) − 1; otherwise
/// before below 12 (1 or 0). The original returns the byte in al.
/// Original: `WcEuropeLeague::get_qualification_status_change` (0x91b710, wc_europe_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_get_qualification_status_change(this: usize, club: usize) -> i8 {
    covered!("cx16_8 european status change");
    let before = qualifying_stat_byte(this, club, 0x13);
    let now = qualifying_stat_byte(this, club, 0x12);
    if now < 0xc {
        (before < 0xc) as i8 - 1
    } else {
        (before < 0xc) as i8
    }
}

/// rule: in the Oceania and CONCACAF qualifying a nation's status changes up (1) when its progress code (stat 0x12)
/// leaves 1 (stat 0x13, the previous code, was 1), down (−1) when it becomes 1 from another code, otherwise 0
///
/// `qualifying_stat_byte` of 0x13 (before) and 0x12 (now): now 1, −1 when before was not 1 (else 0); otherwise 1 when
/// before was 1 (else 0). The byte in al. Shared by the CONCACAF and Oceania vtables (+0x74).
/// Original: `WcOceaniaLeague::get_qualification_status_change` (0x920260, wc_oceania_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_get_qualification_status_change(this: usize, club: usize) -> i8 {
    covered!("cx16_8 oceania status change");
    let before = qualifying_stat_byte(this, club, 0x13);
    let now = qualifying_stat_byte(this, club, 0x12);
    if now == 1 {
        -((before != 1) as i8) // cx16_8: became 1
    } else {
        (before == 1) as i8
    }
}

/// Comparers: the three only read the statistics; side by side on the result (the byte or word).
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_get_qualification_status(this: usize, club: usize) -> i16 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i16>(IDX_WC_EUROPE_LEAGUE_GET_QUALIFICATION_STATUS)(this, club);
    let b = wc_europe_league_get_qualification_status(this, club);
    SCOREBOARD.record(a == b, || format!("wc_europe_league_get_qualification_status({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}

pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_get_qualification_status_change(this: usize, club: usize) -> i8 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i8>(IDX_WC_EUROPE_LEAGUE_GET_QUALIFICATION_STATUS_CHANGE)(this, club);
    let b = wc_europe_league_get_qualification_status_change(this, club);
    SCOREBOARD.record(a == b, || format!("wc_europe_league_get_qualification_status_change({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}

pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_get_qualification_status_change(this: usize, club: usize) -> i8 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i8>(IDX_WC_OCEANIA_LEAGUE_GET_QUALIFICATION_STATUS_CHANGE)(this, club);
    let b = wc_oceania_league_get_qualification_status_change(this, club);
    SCOREBOARD.record(a == b, || format!("wc_oceania_league_get_qualification_status_change({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}

// ---------- cx16_9: the World Cup record of the qualifying sections, the CONCACAF season reset and the European group sizes (wc_oceania_league.cpp, wc_concacaf_cup.cpp, wc_europe_league.cpp) ----------

/// rule: the competition the World Cup qualifying sections feed is the FIFA World Cup
///
/// The record of [`KEY_CLUB_COMP_FIFA_WORLD_CUP`] in [`G_CLUB_COMP_TABLE`] (0x6b bytes each). Shared by the CONCACAF,
/// European and Oceania vtables (+0x78); `this` is not read.
/// Original: `WcOceaniaLeague::get_finals_comp` (0x91e1c0, wc_oceania_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_get_finals_comp(_this: usize) -> usize {
    covered!("cx16_9 finals comp");
    read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(0x6b)) // cx16_9: the World Cup record
}

/// rule: the CONCACAF qualifying clears its run-by-stages flag (`cup::RUN_BY_STAGES` 0) and returns 1
/// Original: `WcConcacafCup::skip_season_reset` (0x915fe0, wc_concacaf_cup.cpp, thiscall, ret 0; vtable +0x2c).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_skip_season_reset(this: usize) -> u32 {
    covered!("cx16_9 concacaf reset");
    Cup(this).set_run_by_stages(0); // cx16_9: the flag cleared
    1
}

/// rule: the European qualifying groups (−1 is the section's own group, 0 on its stages) have five nations, with six
/// in groups 2, 4, 6 and 7 when one nation is exempt, in 2, 4 and 7 when two are, and in all but group 2 when three are
///
/// By the exempt count (`WC_EUROPE_EXEMPT`, signed byte): 0, 5; 1, 6 for groups 2, 4, 6, 7; 2, 6 for groups 2, 4, 7;
/// 3, 5 for group 2 and 6 for the others; any other count, the error box of line 0x25f and 5. `group` is the low byte
/// of the argument. Returns a word.
/// Original: `WcEuropeLeague::get_group_size` (0x918dd0, wc_europe_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_get_group_size(this: usize, group: u32) -> i16 {
    let g = group as u8;
    covered!("cx16_9 european group size");
    match read_u8(this + WC_EUROPE_EXEMPT) as i8 {
        0 => 5,
        1 => if matches!(g, 2 | 4 | 6 | 7) { 6 } else { 5 },
        2 => if matches!(g, 2 | 4 | 7) { 6 } else { 5 }, // cx16_9: two exempt, three groups of six
        3 => 5 + (g != 2) as i16,
        _ => {
            show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x25f);
            5
        }
    }
}

/// Comparers: side by side on the result (and, for the reset, the byte it writes); the group size only off the error
/// path (a second box).
pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_get_finals_comp(this: usize) -> usize {
    let a = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_WC_OCEANIA_LEAGUE_GET_FINALS_COMP)(this);
    let b = wc_oceania_league_get_finals_comp(this);
    SCOREBOARD.record(a == b, || format!("wc_oceania_league_get_finals_comp({:#x}) original={:#x} rust={:#x}", this, a, b));
    a
}

pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_skip_season_reset(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_CONCACAF_CUP_SKIP_SEASON_RESET);
    let (a, b, same, mo, mr) = crate::comparison::side_by_side_light(&[(this + cup::RUN_BY_STAGES, 1)], || o(this), || wc_concacaf_cup_skip_season_reset(this));
    SCOREBOARD.record(same && a == b, || format!("wc_concacaf_cup_skip_season_reset({:#x}) original={} {:x?} rust={} {:x?}", this, a, mo, b, mr));
    a
}

pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_get_group_size(this: usize, group: u32) -> i16 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> i16>(IDX_WC_EUROPE_LEAGUE_GET_GROUP_SIZE)(this, group);
    if (read_u8(this + WC_EUROPE_EXEMPT) as i8) < 0 || read_u8(this + WC_EUROPE_EXEMPT) > 3 {
        return a;
    }
    let b = wc_europe_league_get_group_size(this, group);
    SCOREBOARD.record(a == b, || format!("wc_europe_league_get_group_size({:#x}, {}) exempt {} original={} rust={}", this, group as u8 as i8, read_u8(this + WC_EUROPE_EXEMPT), a, b));
    a
}

// ---------- rx08_4: the European World Cup qualifying: the staff records of a nation's fate, its news text and the group draw (wc_europe_league.cpp) ----------

/// The European qualifying's play-off stage index (+0x101, signed byte; −1 before the format is set), after the last
/// group stage (`WC_EUROPE_LAST_GROUP`, +0xff) and the runners-up stage (+0x100).
pub(crate) const WC_EUROPE_PLAY_OFF_STAGE: usize = 0x101;

/// The World Cup's record (`KEY_CLUB_COMP_FIFA_WORLD_CUP` in [`G_CLUB_COMP_TABLE`], 0x6b bytes each) and its
/// competition object ([`G_COMP_OBJECT_TABLE`]).
unsafe fn world_cup_record_and_object() -> (usize, usize) {
    let key = read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP);
    (read_usize(G_CLUB_COMP_TABLE).wrapping_add(key.wrapping_mul(0x6b)), read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(key.wrapping_mul(4))))
}

/// A nation through to the World Cup from the European qualifying: the World Cup qualification record (weight
/// `weight`), its place in the World Cup (`exe::worldcup_add_qualified_team`) and in the national teams' list of
/// qualified nations (NationalTeams::add_qualified_nation on [`G_NATIONAL_TEAMS`]).
unsafe fn wc_europe_league_qualify_nation(club: usize, weight: u32) {
    let (record, world_cup) = world_cup_record_and_object();
    european_cup_add_qualified_record(club, record, 0, 0, weight);
    exe::worldcup_add_qualified_team(world_cup, club); // rx08_4: the nation enters the World Cup
    let add: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_ADD_QUALIFIED_NATION);
    add(read_usize(G_NATIONAL_TEAMS), club, record);
}

/// rule: in the European qualifying a group winner (status 7) gets the World Cup qualification record (weight 100),
/// enters the World Cup and the list of qualified nations, and its status becomes 4; in the play-off the winner (1) gets
/// the same with weight 50 and the loser (2) the knocked-out record (weight 50) and ends its campaign; the other group
/// fates leave no record
///
/// `this.update_staff_records_for_status(club, outcome, stage, out, game, a6)` (virtual +0x44). In the groups (the play-off
/// stage `WC_EUROPE_PLAY_OFF_STAGE` not set, or the stage byte below it, signed): outcome 7, the World Cup
/// (`wc_europe_league_qualify_nation` with 0x64) and `*out` = 4 when there is `out`, −1; outcomes 1 and 0xfe, −1 without
/// any record; any other, 0. In the play-off stage itself: outcome 1, the World Cup with 0x32 (`*out` untouched), −1;
/// outcome 2, `european_cup_add_knocked_out_record`(club, `league::COMPETITION`, the fixture's `fixture::ROUND_PREFIX`,
/// its `fixture::ROUND_NAME`, 0x32) and the national team's campaign ended (`clear_national_team_competition`), 0; any
/// other, 0. Past the play-off stage, 0.
/// Original: `WcEuropeLeague::update_staff_records_for_status` (0x918980, wc_europe_league.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, _a6: u32) -> u32 {
    let play_off = read_u8(this + WC_EUROPE_PLAY_OFF_STAGE) as i8;
    let stage = stage as u8 as i8;
    if play_off == -1 || stage < play_off {
        return match outcome as u8 {
            7 => {
                covered!("rx08_4 european group winner to the world cup");
                wc_europe_league_qualify_nation(club, 0x64);
                if out != 0 {
                    write_u8(out, 4);
                }
                u32::MAX
            }
            1 | 0xfe => u32::MAX,
            _ => 0,
        };
    }
    if stage != play_off {
        return 0;
    }
    match outcome as u8 {
        1 => {
            covered!("rx08_4 european play-off winner");
            wc_europe_league_qualify_nation(club, 0x32);
            u32::MAX
        }
        2 => {
            covered!("rx08_4 european play-off loser");
            // simplification: the original pushes the two words with the high halves of the registers; the callee reads words
            european_cup_add_knocked_out_record(club, League(this).competition().addr(), read_u16(game + fixture::ROUND_PREFIX) as u32, read_u16(game + fixture::ROUND_NAME) as u32, 0x32);
            clear_national_team_competition(Club(club)); // rx08_4: the play-off loser ends its campaign
            0
        }
        _ => 0,
    }
}

/// Comparer for `wc_europe_league_update_staff_records_for_status`: the records, the World Cup place and the end of the
/// campaign do not run twice, so only the original runs; the plan: the return, `*out` = 4 for a group winner with
/// `out`, and the World Cup's entrant count one more (when it had room) for a nation through.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(IDX_WC_EUROPE_LEAGUE_UPDATE_STAFF_RECORDS_FOR_STATUS);
    let play_off = read_u8(this + WC_EUROPE_PLAY_OFF_STAGE) as i8;
    let s = stage as u8 as i8;
    let in_groups = play_off == -1 || s < play_off;
    let through = (in_groups && outcome as u8 == 7) || (!in_groups && s == play_off && outcome as u8 == 1);
    let expected = if through || (in_groups && matches!(outcome as u8, 1 | 0xfe)) { u32::MAX } else { 0 };
    let world_cup = League(world_cup_record_and_object().1);
    if through && world_cup.is_null() {
        return o(this, club, outcome, stage, out, game, a6);
    }
    let n = if through { world_cup.entrant_count() } else { 0 };
    let room = through && n < world_cup.entrant_capacity();
    let r = o(this, club, outcome, stage, out, game, a6);
    let out_ok = !(in_groups && outcome as u8 == 7 && out != 0) || read_u8(out) == 4;
    let list_ok = !through || world_cup.entrant_count() == if room { n + 1 } else { n };
    SCOREBOARD.record(r == expected && out_ok && list_ok, || format!("wc_europe_league_update_staff_records_for_status({:#x}, {}, stage {}) returned {:#x} (expected {:#x}), list {}", club, outcome as u8, s, r, expected, n));
    r
}

/// Prints one news sentence of a nation and a competition into the global buffer 0xde1f64 (`FN_LANGLIB_SPRINTF`: the
/// template, the club's gender twice, the competition's gender twice, the club's short name and the competition's
/// `name`), sets it as the text and links the club (0x7d5) and the competition `linked` (its short name, 0x7d0).
unsafe fn wc_europe_league_fate_sentence(text: usize, template: usize, club: Club, comp: usize, name: usize, linked: usize) {
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let club_gender = club.short_gender_name() as i8 as i32;
    let comp_gender = read_u8(comp + competition::SHORT_GENDER_NAME) as i8 as i32;
    let club_name = club.addr() + club::SHORT_NAME;
    sprintf(0xde1f64, template, club_gender, club_gender, comp_gender, comp_gender, club_name, name);
    exe::text_set_string(text, 0xde1f64 as *const u8);
    exe::text_add_link(text, club_name as *const u8, 0x7d5, club);
    exe::text_add_link(text, (linked + competition::SHORT_NAME) as *const u8, 0x7d0, linked);
}

/// rule: the news text of a nation's fate in the European qualifying: in the groups the winner "qualify for" the World
/// Cup (the full sentence is the shared group one) and the eliminated get the shared group sentence; the best runner-up
/// "are the best second placed team" (or "one of the three best", with one exempt nation) of the qualifying, "qualify
/// for" the World Cup in the short sentence; in the play-off the shared cup tie sentence, or in short "qualify for" the
/// World Cup for the winner and "out of" the European Championship for the loser
///
/// The arguments as `world_cup_build_fate_news_text` (the club index `club_index` in [`G_CLUB_TABLE`], the fate byte
/// `fate`, `stage`, the stage id `a3` (a signed word), the fixture `a4`, `a5`, `a6`, `a7`, `full_sentence` and the Text
/// `text`).
/// - Stage ids 1000 to 1009 (the groups): fate 7 with the full sentence, the shared group sentence
///   (`exe::comp_util_write_group_qualify_news`(club_index, a3, the competition, 7, full_sentence, text)); fate 7
///   without it, 0x9c470c with the World Cup's short name; fate 0xfe, the group sentence with −2; any other, nothing.
/// - Stage id 1020 (the runners-up) and fate 7: with the full sentence, 0x9c4800 when the exempt count is 1 (one of the
///   three best runners-up) or 0x9c4764 otherwise (the best), with the competition's long name (`competition::NAME`), the
///   club linked and the competition's long name linked (0x7d0); without it, 0xad4ab0 with the World Cup.
/// - Otherwise only with a fixture `a4` whose `fixture::ROUND_NAME` is 0xa4 (the play-off): with the full sentence, the
///   shared cup tie sentence (`exe::comp_util_write_cup_tie_result_news`(club_index, a4, a7, fate, a5, text)); without
///   it, fate 1 0xad4ab0 with the World Cup, any other fate 0x9c46b8 "out of" with the European Championship's short
///   name (`KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP`), still linked to the World Cup.
/// The short sentences go through `wc_europe_league_fate_sentence` (the club's link first, then the World Cup's).
/// Original: `WcEuropeLeague::build_fate_news_text` (0x918b00, wc_europe_league.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn wc_europe_league_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, full_sentence: u32, text: usize) {
    let _ = (stage, a6);
    let league = League(this);
    let comp = league.competition().addr();
    let club = club_at(club_index as i32 as isize);
    let (world_cup, _) = world_cup_record_and_object();
    let to_world_cup = |template: usize| wc_europe_league_fate_sentence(text, template, club, world_cup, world_cup + competition::SHORT_NAME, world_cup);
    let stage_id = a3 as u16 as i16;
    // simplification: the original leaves al with the fate byte or a callee's eax; the callers do not read it
    if (0x3e8..=0x3f1).contains(&stage_id) {
        match fate as u8 {
            7 if full_sentence != 0 => {
                covered!("rx08_4 european group winner, full sentence");
                exe::comp_util_write_group_qualify_news(club_index, a3, comp, 7, full_sentence, text);
            }
            7 => {
                covered!("rx08_4 european group winner");
                to_world_cup(0x9c470c);
            }
            0xfe => {
                covered!("rx08_4 european group out");
                exe::comp_util_write_group_qualify_news(club_index, a3, comp, -2, full_sentence, text); // rx08_4: the group exit sentence
            }
            _ => {}
        }
        return;
    }
    if stage_id == 0x3fc && fate as u8 == 7 {
        if full_sentence == 0 {
            to_world_cup(0xad4ab0);
            return;
        }
        covered!("rx08_4 european best runner-up");
        let template = if read_u8(this + WC_EUROPE_EXEMPT) == 1 { 0x9c4800 } else { 0x9c4764 }; // rx08_4: one exempt nation, three best runners-up
        let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        let club_gender = club.short_gender_name() as i8 as i32;
        let comp_gender = read_u8(comp + competition::SHORT_GENDER_NAME) as i8 as i32;
        sprintf(0xde1f64, template, club_gender, club_gender, comp_gender, comp_gender, club.addr() + club::SHORT_NAME, comp + competition::NAME);
        exe::text_set_string(text, 0xde1f64 as *const u8);
        exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
        exe::text_add_link(text, (comp + competition::NAME) as *const u8, 0x7d0, comp);
        return;
    }
    if a4 == 0 || read_u16(a4 as usize + fixture::ROUND_NAME) != 0xa4 {
        return;
    }
    covered!("rx08_4 european play-off fate");
    if full_sentence != 0 {
        exe::comp_util_write_cup_tie_result_news(club_index, a4, a7, fate, a5, text);
    } else if fate as u8 == 1 {
        to_world_cup(0xad4ab0);
    } else {
        let euro = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP).wrapping_mul(0x6b));
        wc_europe_league_fate_sentence(text, 0x9c46b8, club, euro, euro + competition::SHORT_NAME, world_cup); // rx08_4: the loser "out of" the European Championship
    }
}

/// Comparer for `wc_europe_league_build_fate_news_text`: side by side into two Texts (`compare_text_writer`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, full_sentence: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(IDX_WC_EUROPE_LEAGUE_BUILD_FATE_NEWS_TEXT);
    compare_text_writer(text, || o(this, club_index, fate, stage, a3, a4, a5, a6, a7, full_sentence, text),
        |t| wc_europe_league_build_fate_news_text(this, club_index, fate, stage, a3, a4, a5, a6, a7, full_sentence, t),
        || format!("wc_europe_league_build_fate_news_text(club {}, fate {}, stage id {}, full {})", club_index, fate as u8, a3 as u16 as i16, full_sentence));
}

/// Deals the European qualifying's entry pool (+0xee, already sorted by strength; its count +0xf2, a dword) into the
/// group slots `slots` (six per group): pot by pot (at most six), the group order shuffled (20 swaps of two drawn
/// places, `exe::utils_random`(groups) twice each), then each group in that order takes the next nation while it has
/// room (its size from WcEuropeLeague::get_group_size of the group index less one, a signed word). Returns how many
/// nations each group got.
unsafe fn wc_europe_league_deal_groups(this: usize, slots: usize, groups: i8) -> [i8; 10] {
    let size: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(FN_WC_EUROPE_LEAGUE_GET_GROUP_SIZE);
    let mut filled = [0i8; 10];
    let mut order: [i8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut next = 0i8;
    let pool_left = |next: i8| (next as i32) < read_i32(this + 0xf2);
    for _pot in 0..6 {
        if !pool_left(next) {
            break;
        }
        for _ in 0..20 {
            let a = exe::utils_random(groups as i32) as i8 as usize;
            let b = exe::utils_random(groups as i32) as i8 as usize;
            order.swap(a, b);
        }
        let mut k = 0i16;
        while k < groups as i16 && pool_left(next) {
            let g = order[k as usize];
            if g < groups {
                let had = filled[g as usize];
                if (had as i16) < size(this, g.wrapping_sub(1) as u8 as u32) { // rx08_4: the group's size by its index less one
                    let nation = read_usize(read_usize(this + 0xee) + next as usize * 4);
                    write_usize(slots + (g as usize * 6 + had as usize) * 4, nation);
                    next += 1;
                    filled[g as usize] = had + 1;
                }
            }
            k += 1;
        }
    }
    filled
}

/// rule: the European qualifying draw: the nations, strongest first, go into the groups pot by pot; in each pot the
/// groups are shuffled and each one with room takes the next nation, so every pot gives one nation to each group
///
/// The group count is the last group stage (`WC_EUROPE_LAST_GROUP`, signed byte) + 2 (the section's own group and the
/// group stages); a slot table of six per group (`exe::crt_malloc` of the signed byte groups × 6, times 4) in +0xf7
/// (without memory, the error box of line 0x27b and nothing else), dealt by `wc_europe_league_deal_groups`. The unused
/// slots keep what `malloc` left.
/// Original: `WcEuropeLeague::draw_groups` (0x918ec0, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_draw_groups(this: usize) {
    let groups = (read_u8(this + WC_EUROPE_LAST_GROUP) as i8).wrapping_add(2);
    let slots = exe::crt_malloc(((groups.wrapping_mul(6) as i32) << 2) as u32);
    write_usize(this + 0xf7, slots);
    if slots == 0 {
        show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x27b);
        return;
    }
    covered!("rx08_4 european group draw");
    wc_europe_league_deal_groups(this, slots, groups);
}

/// Comparer for `wc_europe_league_draw_groups`: the original draws into its own slot table; the generator goes back to
/// where it was and the copy deals the same draws into a table of its own (its draws not counted); the dealt slots of
/// every group must match. The generator is left as the original left it.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_draw_groups(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_EUROPE_LEAGUE_DRAW_GROUPS);
    let rng_before = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    o(this);
    let slots = read_usize(this + 0xf7);
    if slots == 0 {
        return;
    }
    let rng_after = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    rng_before.write_back();
    let groups = (read_u8(this + WC_EUROPE_LAST_GROUP) as i8).wrapping_add(2);
    let mut mine = [0usize; 60];
    let filled = crate::rng::pause_trace(|| crate::rng::without_counting(|| wc_europe_league_deal_groups(this, mine.as_mut_ptr() as usize, groups)));
    rng_after.write_back();
    let wrong = (0..groups.clamp(0, 10) as usize).flat_map(|g| (0..filled[g].clamp(0, 6) as usize).map(move |k| g * 6 + k))
        .find(|&s| read_usize(slots + s * 4) != mine[s]);
    SCOREBOARD.record(wrong.is_none(), || format!("wc_europe_league_draw_groups({:#x}): slot {:?} original {:#x} rust {:#x}", this, wrong, wrong.map_or(0, |s| read_usize(slots + s * 4)), wrong.map_or(0, |s| mine[s])));
}

// ---------- rx08_5: the European World Cup qualifying: the group stages, the runners-up table and the play-off stage (wc_europe_league.cpp) ----------

/// The section's virtual `CREATE_SCHEDULE` (+0x3c) for `stage`: the round table, with the round count and the stage id
/// written into the two words.
unsafe fn wc_europe_league_schedule(this: usize, stage: u32, rounds: &mut i16, stage_id: &mut i16) -> usize {
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    create_schedule(this, stage, rounds, stage_id, 0)
}

/// rule: each European qualifying group after the section's own is a league stage of its drawn nations (or of a given
/// list), home and away with the section's points, category and tie-breaks, and its draw news goes out
///
/// `this.create_group_stage(group, teams, count)`: the group's schedule (`wc_europe_league_schedule`). Without `teams`:
/// the group's size (WcEuropeLeague::get_group_size(group), a signed word), a list of that many (`exe::crt_malloc`;
/// none: the error box of line 0x3a7 and 0) copied from the drawn slots of group + 1 (+0xf7, six per group), and the
/// count is the size; with `teams`, the schedule is freed (`exe::crt_free`) and none is passed. The stage
/// (`exe::crt_operator_new` of 0xee and League_Stage::League_Stage `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58:
/// the section, the count, the list, 2, the record, the schedule, the rounds, `league::WIN_POINTS`,
/// `league::DRAW_POINTS`, `league::MATCH_CATEGORY`, the tie-breaks `league::TIEBREAKS`, the places
/// `league::PROMOTED_COUNT`, the year, the group, the stage id, `league::IMPORTANCE`, 4, 0, `league::LEAGUE_FLAGS`, −1,
/// 0, 2) into `league::STAGES`[group]; none: the box of line 0x3c3 and 0. Then the drawn list and the schedule freed,
/// `league::CURRENT_STAGE` = group, the group draw news (`exe::league_send_group_draw_news` of the stage) and 1.
/// Original: `WcEuropeLeague::create_group_stage` (0x9196d0, wc_europe_league.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_create_group_stage(this: usize, group: u32, teams: usize, count: u32) -> u32 {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let league = League(this);
    let cpp = wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP;
    let (mut rounds, mut stage_id) = (0i16, 0i16);
    let mut schedule = wc_europe_league_schedule(this, group, &mut rounds, &mut stage_id);
    let (list, count) = if teams == 0 {
        let size_of: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(FN_WC_EUROPE_LEAGUE_GET_GROUP_SIZE);
        let size = size_of(this, group);
        let list = exe::crt_malloc((size as i32 as u32).wrapping_mul(4));
        if list == 0 {
            show_error_at(cpp, 0x3a7);
            return 0;
        }
        let drawn = read_usize(this + 0xf7) + (group as usize + 1) * 6 * 4; // rx08_5: the group after the section's own
        for k in 0..(size as i32).max(0) as usize {
            write_usize(list + k * 4, read_usize(drawn + k * 4)); // rx08_5: the drawn nations in draw order
        }
        (list, size as u16 as u32)
    } else {
        if schedule != 0 {
            exe::crt_free(schedule);
        }
        schedule = 0;
        (teams, count)
    };
    covered!("rx08_5 european group stage");
    let p = exe::crt_operator_new(0xee);
    // simplification: the original pushes the rounds, the stage id, the bytes and the words with garbage in the high
    // halves of the registers, which the constructor does not read: the copy passes clean values
    let stage = if p != 0 {
        construct(p, this, count, list, 2, league.competition().addr(), schedule, rounds as u16 as u32, league.win_points() as u32, league.draw_points() as u32,
            league.match_category() as u32, this + league::TIEBREAKS, this + league::PROMOTED_COUNT, league.year() as u16 as u32, group, stage_id as u16 as u32,
            read_u8(this + league::IMPORTANCE) as u32, 4, 0, read_u16(this + league::LEAGUE_FLAGS) as u32, -1, 0, 2)
    } else {
        0
    };
    let slot = league.stages() + group as usize * 4;
    write_usize(slot, stage);
    if read_usize(slot) == 0 {
        show_error_at(cpp, 0x3c3);
        return 0;
    }
    if teams == 0 {
        exe::crt_free(list);
    }
    if schedule != 0 {
        exe::crt_free(schedule);
    }
    league.set_current_stage(group as usize);
    exe::league_send_group_draw_news(read_usize(slot));
    1
}

/// Comparer for `wc_europe_league_create_group_stage`: it builds the stage and sends news, so only the original runs;
/// the plan: 1, the stage in its slot with the count's rows (the group's size when drawn) and the current stage the group.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_create_group_stage(this: usize, group: u32, teams: usize, count: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32>(IDX_WC_EUROPE_LEAGUE_CREATE_GROUP_STAGE);
    let r = o(this, group, teams, count);
    let league = League(this);
    let stage = League(read_usize(league.stages() + group as usize * 4));
    if r != 1 || stage.is_null() || (teams == 0 && read_u8(this + WC_EUROPE_EXEMPT) > 3) {
        return r;
    }
    let size_of: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(FN_WC_EUROPE_LEAGUE_GET_GROUP_SIZE);
    let n = if teams == 0 { size_of(this, group) } else { count as i16 };
    let ok = stage.club_count() == n && league.current_stage() == group as usize;
    SCOREBOARD.record(ok, || format!("wc_europe_league_create_group_stage({:#x}, {}): {} rows of {}, current stage {}", this, group, stage.club_count(), n, league.current_stage() as i32));
    r
}

/// The European qualifying sections, from the section itself (−1) to the last group stage (`WC_EUROPE_LAST_GROUP`,
/// signed), in order; empty when the last group is below −1.
unsafe fn wc_europe_league_groups(this: usize) -> impl Iterator<Item = League> {
    let last = read_u8(this + WC_EUROPE_LAST_GROUP) as i8 as i32;
    (-1..=last).map(move |s| if s == -1 { League(this) } else { League(read_usize(League(this).stages() + s as usize * 4)) })
}

/// The club of row `k` of a league table (`league::TABLE`, `league_table_row::SIZE` bytes each).
unsafe fn table_club(league: League, k: usize) -> usize {
    read_usize(league.table() + k * league_table_row::SIZE)
}

/// A nation out of the European qualifying: the failed-to-qualify record (`european_cup_add_failed_to_qualify_record`
/// with the competition, 0x41c and 0x64) and the end of its campaign (`clear_national_team_competition`).
unsafe fn wc_europe_league_eliminate(this: usize, club: usize) {
    european_cup_add_failed_to_qualify_record(club, League(this).competition().addr(), 0x41c, 0x64);
    clear_national_team_competition(Club(club)); // rx08_5: out of the qualifying
}

/// rule: when the European groups end, the best runners-up go to the play-off: one of them, drawn, plays the Asian
/// section, the others play each other home and away; every other nation not through (status 7) is out
///
/// Without a runners-up stage (+0x100 = −1) or a play-off stage (`WC_EUROPE_PLAY_OFF_STAGE` = −1): every row of every
/// group (`wc_europe_league_groups`) not at status 7 (`league_table_row::STATUS`) is out (`wc_europe_league_eliminate`).
/// Otherwise the play-off size N (8 with up to one exempt nation, 6 with more) and a list of N + 1 (`exe::crt_malloc`;
/// none: the error box of line 0x444 and nothing else) with the clubs of the runners-up stage's rows 0..N
/// (`exe::league_get_table_row`); one drawn (`exe::utils_random`(N + 1)) goes to Asia and its place takes the last one
/// (which stays at the end too). Each group row: a club among the first N of the list (the first match), or the Asian
/// one, goes on (status 1) with the qualified record for round 0xa0 (`european_cup_add_qualified_record`(club, the
/// competition, 0, 0xa0, 0x1e)); the Asian section's record (`KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION`) only
/// when the list entry found is the Asian one; any other row not at status 7 is out. Then, on the last group processed
/// (the runners-up stage when there is no group), its first rows down to its row count less N get the section's
/// virtual `SET_CLUB_PROGRESS`(club, −1, 1, 1, 1). The play-off schedule (`wc_europe_league_schedule` of the play-off
/// stage), the stage (`exe::crt_operator_new` of 0xb2 and Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret
/// 0x3c: the section, N, the list, the rounds, the record, the schedule, the year, the play-off stage, 4, the stage id,
/// 0x14, 0, 0, 0, 0) into `league::STAGES`[play-off]; the list and the schedule freed; no stage: the box of line 0x49f;
/// otherwise its first round (`exe::cup_set_up_round`(stage, 0)) and `league::CURRENT_STAGE` the play-off stage.
/// Original: `WcEuropeLeague::create_play_off_stage` (0x919910, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_create_play_off_stage(this: usize) {
    let league = League(this);
    let runners_up = read_u8(this + 0x100) as i8;
    let play_off = read_u8(this + WC_EUROPE_PLAY_OFF_STAGE) as i8;
    let not_through = |l: League, k: usize| read_u8(l.table() + k * league_table_row::SIZE + league_table_row::STATUS) != 7;
    if runners_up == -1 || play_off == -1 {
        covered!("rx08_5 european groups end without a play-off");
        for group in wc_europe_league_groups(this) {
            for k in 0..group.club_count().max(0) as usize {
                if not_through(group, k) {
                    wc_europe_league_eliminate(this, table_club(group, k));
                }
            }
        }
        return;
    }
    let n: i16 = if read_u8(this + WC_EUROPE_EXEMPT) <= 1 { 8 } else { 6 }; // rx08_5: eight play-off places with up to one exempt nation
    let list = exe::crt_malloc((n as u32 + 1) * 4);
    if list == 0 {
        show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x444);
        return;
    }
    covered!("rx08_5 european play-off");
    let runners_up_stage = League(read_usize(league.stages() + runners_up as isize as usize * 4));
    for k in 0..=n as usize {
        write_usize(list + k * 4, read_usize(exe::league_get_table_row(runners_up_stage, k as i32)));
    }
    let drawn = exe::utils_random(n as i32 + 1) as usize;
    let asia = read_usize(list + drawn * 4);
    write_usize(list + drawn * 4, read_usize(list + n as usize * 4));
    let mut last = runners_up_stage;
    for group in wc_europe_league_groups(this) {
        last = group;
        for k in 0..group.club_count().max(0) as usize {
            let club = table_club(group, k);
            let found = (0..n as usize).find(|&j| read_usize(list + j * 4) == club).unwrap_or(n as usize);
            if found < n as usize || club == asia {
                write_u8(group.table() + k * league_table_row::SIZE + league_table_row::STATUS, 1);
                let comp = if read_usize(list + found * 4) == asia { club_comp_record(read_usize(KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION)) } else { league.competition().addr() };
                european_cup_add_qualified_record(club, comp, 0, 0xa0, 0x1e);
            } else if not_through(group, k) {
                wc_europe_league_eliminate(this, club);
            }
        }
    }
    let progress: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_CLUB_PROGRESS));
    for k in 0..(last.club_count() as i32 - n as i32).max(0) as usize {
        progress(this, table_club(last, k), -1, 1, 1, 1);
    }
    let (mut rounds, mut stage_id) = (0i16, 0i16);
    let schedule = wc_europe_league_schedule(this, play_off as u8 as u32, &mut rounds, &mut stage_id);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, n as u32, list, rounds as u16 as u32, league.competition().addr(), schedule, league.year() as u16 as u32, play_off as u8 as u32, 4, stage_id as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    let slot = league.stages() + play_off as isize as usize * 4;
    write_usize(slot, stage);
    exe::crt_free(list);
    exe::crt_free(schedule);
    if read_usize(slot) == 0 {
        show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x49f);
        return;
    }
    exe::cup_set_up_round(read_usize(slot), 0);
    league.set_current_stage(play_off as isize as usize);
}

/// Comparer for `wc_europe_league_create_play_off_stage`: it draws, writes records and builds the stage, so only the
/// original runs; the plan: with both stages set, the current stage is the play-off stage, which exists.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_create_play_off_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_EUROPE_LEAGUE_CREATE_PLAY_OFF_STAGE);
    let both = read_u8(this + 0x100) != 0xff && read_u8(this + WC_EUROPE_PLAY_OFF_STAGE) != 0xff;
    o(this);
    if both {
        let league = League(this);
        let play_off = read_u8(this + WC_EUROPE_PLAY_OFF_STAGE) as i8 as isize as usize;
        let stage = read_usize(league.stages() + play_off * 4);
        SCOREBOARD.record(stage != 0 && league.current_stage() == play_off, || format!("wc_europe_league_create_play_off_stage({:#x}): stage {:#x}, current {}", this, stage, league.current_stage() as i32));
    }
}

/// rule: the European runners-up are ranked by their record against the top four of their own group: each group's
/// runner-up row in the runners-up table is the one of a four-team sub-league of the group's games among its top four
///
/// The period: from the first round date to the last one (`league::ROUND_DATES`, day and year offset + the year;
/// Date::Date `exe::new_date`). For each group (`wc_europe_league_groups`): its table sorted (`exe::league_sort_table`),
/// its runner-up (row 1, `exe::league_get_table_row`) and the ids of its top four in a new list (`exe::crt_malloc`(0x10);
/// none: the error box of line 0x57c and return, without the final sort); the sub-league of their games
/// (`exe::crt_operator_new`(0xee) and SubLeague::SubLeague `FN_SUBLEAGUE_SUBLEAGUE`, thiscall, ret 0x30: 4, the ids, −1,
/// −1, the two dates, the competition id, `league::WIN_POINTS`, `league::DRAW_POINTS`, the tie-breaks, 2 × n × (n − 1)
/// with n the group's row count, and the section's stored results +0x10); when the runner-up is one of its first 4
/// rows, that row is copied into the next row of the runners-up table (+0x100; the row advances only on a copy) with its
/// head-to-head and position-history pointers cleared (the sub-league owns them); the sub-league destroyed (its virtual
/// 0 with 1) and the list freed. At the end the runners-up table sorted.
/// Original: `WcEuropeLeague::fill_runners_up_table` (0x91a240, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_fill_runners_up_table(this: usize) {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, u32, usize, i32, i32, usize, usize, i32, u32, u32, usize, u32, usize) -> usize = core::mem::transmute(FN_SUBLEAGUE_SUBLEAGUE);
    let league = League(this);
    let (mut from, mut to) = ([0u8; 8], [0u8; 8]);
    let dates = league.round_dates();
    exe::new_date(from.as_mut_ptr() as usize, read_u16(dates) as u32, read_u16(dates + 2).wrapping_add(league.year() as u16) as u32);
    let last = dates.wrapping_add((league.round_count() as i32 as usize).wrapping_mul(league_round_date::SIZE)).wrapping_sub(league_round_date::SIZE);
    exe::new_date(to.as_mut_ptr() as usize, read_u16(last) as u32, read_u16(last + 2).wrapping_add(league.year() as u16) as u32);
    let runners_up = || League(read_usize(league.stages() + read_u8(this + 0x100) as i8 as isize as usize * 4));
    let mut row = 0usize;
    for group in wc_europe_league_groups(this) {
        exe::league_sort_table(group, 0);
        let runner = read_usize(exe::league_get_table_row(group, 1));
        let ids = exe::crt_malloc(0x10);
        if ids == 0 {
            show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x57c);
            return;
        }
        for i in 0..4 {
            write_u32(ids + 4 * i as usize, read_u32(read_usize(exe::league_get_table_row(group, i))));
        }
        let p = exe::crt_operator_new(0xee);
        let n = group.club_count() as i32;
        let sub = if p != 0 {
            construct(p, 4, ids, -1, -1, from.as_ptr() as usize, to.as_ptr() as usize, read_i32(league.competition().addr()), league.win_points() as u32,
                league.draw_points() as u32, this + league::TIEBREAKS, n.wrapping_sub(1).wrapping_mul(n).wrapping_mul(2) as u32, read_usize(this + 0x10)) // rx08_5: both legs between the four
        } else {
            0
        };
        if let Some(j) = (0..4).find(|&j| read_usize(exe::league_get_table_row(sub, j)) == runner) {
            covered!("rx08_5 european runner-up row copied");
            let target = runners_up().table() + row;
            core::ptr::copy_nonoverlapping((League(sub).table() + j as usize * league_table_row::SIZE) as *const u8, target as *mut u8, league_table_row::SIZE);
            write_u32(target + league_table_row::POSITION_HISTORY, 0);
            write_u32(target + league_table_row::H2H_ENTRIES, 0);
            row += league_table_row::SIZE;
        }
        if sub != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(sub) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
            destroy(sub, 1);
        }
        exe::crt_free(ids);
    }
    exe::league_sort_table(runners_up(), 0);
}

/// Comparer for `wc_europe_league_fill_runners_up_table`: side by side on the runners-up table's rows; the groups'
/// sorts and the sub-leagues are the same on both sides.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_fill_runners_up_table(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_EUROPE_LEAGUE_FILL_RUNNERS_UP_TABLE);
    let stage = League(read_usize(League(this).stages() + read_u8(this + 0x100) as i8 as isize as usize * 4));
    if stage.is_null() || stage.table() == 0 {
        return o(this);
    }
    let regions = [(stage.table(), stage.club_count().clamp(0, 10) as usize * league_table_row::SIZE)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this), || wc_europe_league_fill_runners_up_table(this));
    SCOREBOARD.record(same, || format!("wc_europe_league_fill_runners_up_table({:#x}): {}", this, first_differences(&mo, &mr).join(", ")));
}

// ---------- rx08_6: the European World Cup qualifying's predicted positions (wc_europe_league.cpp) ----------

/// The predicted final position of the nation at `k` in the strength-sorted European pool, by the exempt count: the
/// seeding pots (the first nine or ten are pot 1, the next ones pot 2 and so on), each pot's first place as the
/// prediction. `None` for an exempt count outside 0..=3 (no prediction).
fn wc_europe_league_predicted_position(exempt: u8, k: i16) -> Option<u32> {
    let pots: &[(i16, u32)] = match exempt {
        0 => &[(10, 1), (15, 11), (19, 16), (20, 20), (30, 21), (40, 31), (i16::MAX, 41)],
        1 => &[(9, 1), (14, 10), (18, 15), (27, 19), (36, 28), (45, 37), (i16::MAX, 46)],
        2 => &[(9, 1), (13, 10), (16, 14), (18, 17), (27, 19), (36, 28), (45, 37), (i16::MAX, 46)],
        3 => &[(8, 1), (12, 9), (15, 13), (16, 16), (24, 17), (32, 25), (40, 33), (i16::MAX, 41)],
        _ => return None,
    };
    pots.iter().find(|&&(below, _)| k < below).map(|&(_, position)| position)
}

/// rule: before the European qualifying starts every nation of the pool gets its predicted final position from its
/// seeding pot (the strongest pot first), and all the European national teams leave their continental competitions to
/// play this one
///
/// Without statistics (`league::COMP_STATS`): the error box of line 0x7a0 and nothing else. The pool (+0xee, count
/// +0xf2) sorted by strength (`exe::crt_qsort`(pool, count, 4, 0x4b07d0)); the European national teams released
/// (`exe::nationalteams_release_competitions_of_continent`(national teams, the European continent record —
/// `G_CONTINENT_TABLE` + 0xc6 × `KEY_CONTINENT_EUROPE` — and 0)); then each nation k of the pool (a signed word below
/// the count): its predicted position (`wc_europe_league_predicted_position`; Comp_Stats::set_club_final_position
/// `exe::comp_stats_set_club_final_position`(stats, nation, position)) and its current competition this one
/// (`exe::nationalteams_set_current_competition`(national teams, `club::NATION`, `league::COMPETITION`)).
/// Original: `WcEuropeLeague::set_predicted_positions` (0x91b1b0, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_set_predicted_positions(this: usize) {
    let league = League(this);
    if league.comp_stats() == 0 {
        show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x7a0);
        return;
    }
    covered!("rx08_6 european predicted positions");
    let pool = read_usize(this + 0xee);
    exe::crt_qsort(pool, read_u32(this + 0xf2), 4, 0x4b07d0);
    let europe = read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(KEY_CONTINENT_EUROPE).wrapping_mul(0xc6));
    exe::nationalteams_release_competitions_of_continent(read_usize(G_NATIONAL_TEAMS), europe, 0);
    let mut k = 0i16;
    while (k as i32) < read_i32(this + 0xf2) {
        let nation = Club(read_usize(read_usize(this + 0xee) + k as isize as usize * 4));
        if let Some(position) = wc_europe_league_predicted_position(read_u8(this + WC_EUROPE_EXEMPT), k) { // rx08_6: the seeding pot's first place
            exe::comp_stats_set_club_final_position(league.comp_stats(), nation, position);
        }
        exe::nationalteams_set_current_competition(read_usize(G_NATIONAL_TEAMS), nation.nation(), league.competition().addr()); // rx08_6: the nation plays this one
        k = k.wrapping_add(1);
    }
}

/// Comparer for `wc_europe_league_set_predicted_positions`: it moves the national teams between competitions, so only
/// the original runs; the plan: the pool sorted as a copy sorted the same way before.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_EUROPE_LEAGUE_SET_PREDICTED_POSITIONS);
    let (pool, n) = (read_usize(this + 0xee), read_i32(this + 0xf2).clamp(0, 64) as usize);
    if League(this).comp_stats() == 0 || pool == 0 {
        return o(this);
    }
    let mut sorted: Vec<usize> = (0..n).map(|k| read_usize(pool + k * 4)).collect();
    exe::crt_qsort(sorted.as_mut_ptr() as usize, n as u32, 4, 0x4b07d0);
    o(this);
    let wrong = (0..n).find(|&k| read_usize(pool + k * 4) != sorted[k]);
    SCOREBOARD.record(wrong.is_none(), || format!("wc_europe_league_set_predicted_positions({:#x}): pool place {:?} differs from the sorted copy", this, wrong));
}

// ---------- rx09_16: the Asian World Cup qualifying first round draw (wc_asia_league.cpp) ----------

/// The Asian qualifying's exempt nation count (+0xee, signed byte; the hosts and the last winner when Asian).
pub(crate) const WC_ASIA_EXEMPT: usize = 0xee;
/// The Asian qualifying's entrant count (+0xf3, dword) of the list at `wc_turnover_consts::ENTRANT_LIST`.
pub(crate) const WC_ASIA_ENTRANT_COUNT: usize = 0xf3;
/// The Asian qualifying's first round draw (+0xf7, a pointer: ten groups of four national team slots).
pub(crate) const WC_ASIA_DRAW: usize = 0xf7;

/// The team count of an Asian first round group (the group −1 to 8; −1 is the league's own table), by the exempt count
/// (`WC_ASIA_EXEMPT`): none exempt, four in every group; one, three in group 1; two, three in groups 3 and 6; three,
/// three in groups 1, 3 and 5; otherwise four. Any count above 3 or negative shows the box at line 0x37f first.
/// Original: `WcAsiaLeague::get_first_round_group_size` (0x912db0, wc_asia_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_get_first_round_group_size(this: usize, group: u32) -> i16 {
    let g = group as u8;
    let three = match read_u8(this + WC_ASIA_EXEMPT) as i8 {
        0 => false,
        1 => g == 1,
        2 => g == 3 || g == 6, // rx09_16: two exempt, groups 3 and 6 of three
        3 => g == 1 || g == 3 || g == 5,
        _ => {
            show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x37f);
            false
        }
    };
    if three { 3 } else { 4 }
}

/// Comparer for `wc_asia_league_get_first_round_group_size`: the answers side by side (the box only on the original's
/// side for a bad exempt count).
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_get_first_round_group_size(this: usize, group: u32) -> i16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> i16>(IDX_WC_ASIA_LEAGUE_GET_FIRST_ROUND_GROUP_SIZE)(this, group);
    let e = read_u8(this + WC_ASIA_EXEMPT) as i8;
    if (0..=3).contains(&e) {
        let c = wc_asia_league_get_first_round_group_size(this, group);
        SCOREBOARD.record(r == c, || format!("wc_asia_league_get_first_round_group_size(exempt {}, group {}) original={} rust={}", e, group as u8 as i8, r, c));
    }
    r
}

/// The team count of Asian first round group g (−1 to 8), through the hooked routine.
pub(crate) unsafe fn wc_asia_group_size(this: usize, g: i8) -> i16 {
    let size: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(FN_WCASIALEAGUE_GET_FIRST_ROUND_GROUP_SIZE);
    size(this, g as u8 as u32)
}

/// `WcAsiaLeague::get_first_round_group_size` (0x912db0, thiscall, ret 4), through its hook.
const FN_WCASIALEAGUE_GET_FIRST_ROUND_GROUP_SIZE: usize = 0x912db0;

/// Draws the ten Asian first round groups: the group order 0..9 (a local array) shuffled by 20 swaps of two randomly
/// drawn places (`exe::utils_random`(10) twice: the first place, then the second); the draw (`exe::crt_malloc` of 0xa0,
/// ten groups of four dwords, into `WC_ASIA_DRAW`; none: the box at line 0x340 and nothing more). Then up to four pots
/// while fewer than the entrant count (`WC_ASIA_ENTRANT_COUNT`, signed byte against the dword) are placed: for each
/// group in the shuffled order, while some remain, group g takes the next entrant (`wc_turnover_consts::ENTRANT_LIST`)
/// into its next slot if it has fewer than its size (`wc_asia_group_size`(g − 1)).
/// Original: `WcAsiaLeague::draw_first_round_groups` (0x912bd0, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_draw_first_round_groups(this: usize) {
    let mut taken = [0u8; 10];
    let mut order: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for _ in 0..0x14 {
        let a = exe::utils_random(0xa) as u8 as i8 as isize as usize;
        let b = exe::utils_random(0xa) as u8 as i8 as isize as usize;
        order.swap(a, b); // rx09_16: the group order shuffled
    }
    let draw = exe::crt_malloc(0xa0);
    write_usize(this + WC_ASIA_DRAW, draw);
    if draw == 0 {
        return show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x340);
    }
    covered!("rx09_16 asian first round draw");
    let list = read_usize(this + wc_turnover_consts::ENTRANT_LIST);
    let mut placed: i8 = 0;
    for _pot in 0..4 {
        if placed as i32 >= read_i32(this + WC_ASIA_ENTRANT_COUNT) {
            break;
        }
        for &g in &order {
            if placed as i32 >= read_i32(this + WC_ASIA_ENTRANT_COUNT) {
                break;
            }
            if g >= 0xa {
                continue;
            }
            let c = taken[g as usize];
            if (c as i8 as i16) < wc_asia_group_size(this, (g as i8).wrapping_sub(1)) {
                write_usize(draw + 4 * (c as i8 as usize + 4 * g as usize), read_usize(list + 4 * placed as usize));
                placed = placed.wrapping_add(1);
                taken[g as usize] = c.wrapping_add(1);
            }
        }
    }
}

/// The national teams of the Asian draw that the plan can check: each of the first entrants (up to the slots of the
/// ten groups) must be in exactly one used slot (group g, slot c below the group's size).
pub(crate) unsafe fn compare_wc_asia_draw(name: &str, this: usize) {
    let draw = read_usize(this + WC_ASIA_DRAW);
    if draw == 0 {
        return;
    }
    let sizes: Vec<usize> = (0..10).map(|g| wc_asia_group_size(this, g as i8 - 1).max(0) as usize).collect();
    let n = (read_i32(this + WC_ASIA_ENTRANT_COUNT).max(0) as usize).min(sizes.iter().sum());
    let list = read_usize(this + wc_turnover_consts::ENTRANT_LIST);
    let slots: Vec<usize> = (0..10).flat_map(|g| (0..sizes[g]).map(move |c| (g, c))).map(|(g, c)| read_usize(draw + 4 * (c + 4 * g))).collect();
    let missing: Vec<usize> = (0..n).map(|k| read_usize(list + 4 * k)).filter(|t| slots.iter().filter(|s| *s == t).count() != 1).collect();
    SCOREBOARD.record(missing.is_empty(), || format!("{}({:#x}): {} entrants, not drawn once {:x?}", name, this, n, missing));
}

/// Comparer for `wc_asia_league_draw_first_round_groups`: only the original runs (it draws and allocates); every
/// entrant drawn once into a slot within its group's size (`compare_wc_asia_draw`).
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_draw_first_round_groups(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_ASIA_LEAGUE_DRAW_FIRST_ROUND_GROUPS)(this);
    compare_wc_asia_draw("wc_asia_league_draw_first_round_groups", this);
}

/// Builds the table of the first Asian first round group (the league's own, group −1): `club_count` = its size
/// (`wc_asia_group_size`(−1)), the table (`exe::crt_malloc` of 0x3b a team; none: the box at line 0x30d and 0) and a row
/// for each of the group's drawn national teams (League::init_table_row `exe::league_init_table_row`(this, k, slot k of
/// the draw, 0, 0)); then the group's draw news (League::send_group_draw_news `exe::league_send_group_draw_news`).
/// Returns 1.
/// Original: `WcAsiaLeague::init_first_group_table` (0x912ad0, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_init_first_group_table(this: usize) -> u32 {
    let l = League(this);
    l.set_club_count(wc_asia_group_size(this, -1));
    let t = exe::crt_malloc((l.club_count() as i32 * league_table_row::SIZE as i32) as u32);
    l.set_table(t);
    if t == 0 {
        show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x30d);
        return 0;
    }
    covered!("rx09_16 asian first group table");
    let draw = read_usize(this + WC_ASIA_DRAW);
    let mut k: u8 = 0;
    while (k as i8 as i16) < l.club_count() {
        exe::league_init_table_row(l, k as u32, read_usize(draw + 4 * k as i8 as usize), 0, 0); // rx09_16: the k-th team of the first group
        k = k.wrapping_add(1);
    }
    exe::league_send_group_draw_news(l);
    1
}

/// Comparer for `wc_asia_league_init_first_group_table`: only the original runs (it allocates the table and sends the
/// news); the club count, the result and the rows against the first group of the draw.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_init_first_group_table(this: usize) -> u32 {
    let n = wc_asia_group_size(this, -1).max(0) as usize;
    let draw = read_usize(this + WC_ASIA_DRAW);
    let plan: Vec<usize> = (0..n).map(|k| read_usize(draw + 4 * k)).collect();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_ASIA_LEAGUE_INIT_FIRST_GROUP_TABLE)(this);
    let l = League(this);
    let found: Vec<usize> = if l.table() != 0 { (0..n).map(|k| aus_nsl_table_row_club(l, k)).collect() } else { Vec::new() };
    SCOREBOARD.record(r == 1 && l.club_count() as usize == n && found == plan, || format!("wc_asia_league_init_first_group_table({:#x}) original={} plan={:x?} table={:x?}", this, r, plan, found));
    r
}

// ---------- rx09_17: the Asian World Cup qualifying first and second round groups and next stage (wc_asia_league.cpp) ----------

/// A League_Stage of the Asian qualifying (`exe::crt_operator_new` of 0xee; League_Stage::League_Stage
/// `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58; 0 when out of memory): this league, `n` teams from `list`, two
/// legs, the competition, the round table with its rounds, the win, draw and category bytes and the tie-breaks of this
/// league, the place counts at `places`, the year, the stage, the extra as its label, `importance`, 4, 0, the rule bits
/// `flags`, −1, 0, 2 (as `category_b_league_stage`, with 4 where the leagues pass 1).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn qualifying_league_stage(l: League, n: u32, list: usize, table: usize, rounds: i16, extra: i16, places: usize, stage: u32, importance: u32, flags: u32) -> usize {
    let p = exe::crt_operator_new(0xee);
    if p == 0 {
        return 0;
    }
    let ctor: LeagueStageCtorFn = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    ctor(p, l.addr(), n, list, 2, l.competition().addr(), table, rounds as u16 as u32, l.win_points() as u32, l.draw_points() as u32, l.match_category() as u32,
        l.addr() + league::TIEBREAKS, places, l.year() as u16 as u32, stage, extra as u16 as u32, importance, 4, 0, flags, -1, 0, 2)
}

/// Builds Asian first round group k (0 to 8: groups 1 to 9, after the first in the league's own table) as stage k: its
/// size (`wc_asia_group_size`(k)), a list of that many (`exe::crt_malloc`; none: the box at line 0x49a and 0) with the
/// group's drawn teams (slots of group k + 1 in `WC_ASIA_DRAW`), its round table (`stage_round_table`(k)) and the
/// League_Stage (`qualifying_league_stage`: the league's own places from `league::PROMOTED_COUNT`, importance and rule
/// bits) into slot k; the list and the round table freed. No stage: the box at line 0x4b0 and 0. Otherwise the current
/// stage = k, the group's draw news (League::send_group_draw_news on the stage) and 1.
/// Original: `WcAsiaLeague::create_first_round_group` (0x913540, wc_asia_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_create_first_round_group(this: usize, k: u32) -> u32 {
    let l = League(this);
    let size = wc_asia_group_size(this, k as u8 as i8);
    let n = size as i32;
    let list = exe::crt_malloc((n * 4) as u32);
    if list == 0 {
        show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x49a);
        return 0;
    }
    let draw = read_usize(this + WC_ASIA_DRAW);
    for i in 0..n.max(0) as usize {
        write_usize(list + 4 * i, read_usize(draw + 16 * (k as usize + 1) + 4 * i)); // rx09_17: the group's drawn teams
    }
    let (table, rounds, extra) = stage_round_table(l, k);
    let stage = qualifying_league_stage(l, size as u16 as u32, list, table, rounds, extra, this + league::PROMOTED_COUNT, k, l.importance() as u32, l.league_flags() as u16 as u32);
    let slot = l.stages() + 4 * k as usize;
    write_usize(slot, stage);
    exe::crt_free(list);
    exe::crt_free(table);
    if read_usize(slot) == 0 {
        show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x4b0);
        return 0;
    }
    covered!("rx09_17 asian first round group");
    l.set_current_stage(k as usize);
    exe::league_send_group_draw_news(read_usize(slot));
    1
}

/// Comparer for `wc_asia_league_create_first_round_group`: only the original runs (it allocates the stage and sends the
/// news); with a stage built, its teams (as a set) and the current stage against the group's draw.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_create_first_round_group(this: usize, k: u32) -> u32 {
    let l = League(this);
    let n = wc_asia_group_size(this, k as u8 as i8).max(0) as usize;
    let draw = read_usize(this + WC_ASIA_DRAW);
    let plan: Vec<usize> = (0..n).map(|i| read_usize(draw + 16 * (k as usize + 1) + 4 * i)).collect();
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_WC_ASIA_LEAGUE_CREATE_FIRST_ROUND_GROUP)(this, k);
    if r == 1 {
        compare_category_b_stage("wc_asia_league_create_first_round_group", l, k as usize, &plan);
    }
    r
}

/// Rule: the next stage of the Asian qualifying: past the last stage, the error box; otherwise the current stage goes
/// up by one; at 9 (the first round is over) the two second round groups are built, at 11 the third round tie, at 12
/// the play-off.
///
/// At the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the box at line 0x471 and nothing
/// else. Otherwise the current stage + 1; at 9, WcAsiaLeague::create_second_round_groups (0x913740); at 11,
/// WcAsiaLeague::create_third_round_tie (0x913b60); at 12, WcAsiaLeague::create_playoff_tie (0x913d50) (thiscall,
/// ret 0).
/// Original: `WcAsiaLeague::advance_stage` (0x913490, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_advance_stage(this: usize) {
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x471);
    }
    let stage = stage + 1;
    l.set_current_stage(stage as usize);
    covered!("rx09_17 asian next stage");
    match stage {
        9 => call_on_league(0x913740, l),
        11 => call_on_league(0x913b60, l),
        12 => call_on_league(0x913d50, l),
        _ => {}
    }
}

/// Comparer for `wc_asia_league_advance_stage` (`compare_advance_stage_building`: stages 9, 11 and 12 build).
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_ASIA_LEAGUE_ADVANCE_STAGE);
    compare_advance_stage_building("wc_asia_league_advance_stage", League(this), |s| s == 9 || s == 11 || s == 12, || o(this), || wc_asia_league_advance_stage(this))
}

/// The winner (table row 0, League::get_table_row) of Asian first round group k (−1 the league's own table, 0 to 8 the
/// stages).
pub(crate) unsafe fn wc_asia_first_round_winner(l: League, k: i32) -> usize {
    let group = if k > -1 { read_usize(l.stages() + 4 * k as usize) } else { l.addr() };
    read_usize(exe::league_get_table_row(group, 0))
}

/// The places of the Asian second round groups (a local array of the original): with no exempt nation, 2 qualify and 1
/// goes to the third round; otherwise 1 and 1.
pub(crate) unsafe fn wc_asia_second_round_places(this: usize) -> [u8; 4] {
    let first = if read_u8(this + WC_ASIA_EXEMPT) == 0 { 2 } else { 1 }; // rx09_17: two qualify with no exempt nation
    [first, 1, 0, 0]
}

/// Builds the two Asian second round groups (stages 9 and 10, five teams each). A list of five (`exe::crt_malloc` of
/// 0x14; none: the box at line 0x4d9 and 0) takes five first round winners drawn at random (`exe::utils_random`(10):
/// the group r − 1, drawn again when taken; `wc_asia_first_round_winner`); its round table (`stage_round_table`(9)) and
/// the League_Stage (`qualifying_league_stage`: five teams, `wc_asia_second_round_places`, stage 9, importance 0xf,
/// rule bits 4) into slot 9; the list and the round table freed; none: the box at line 0x502 and 0. Otherwise the
/// group's draw news, the current stage = 9, and the other five winners in first round order into a second list (none:
/// the box at line 0x510 and 0) for stage 10 the same way (the box at line 0x534); then the current stage = 10, its draw
/// news and 1.
/// Original: `WcAsiaLeague::create_second_round_groups` (0x913740, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_create_second_round_groups(this: usize) -> u32 {
    let l = League(this);
    let places = wc_asia_second_round_places(this);
    let mut taken = [0u8; 10];
    let first = exe::crt_malloc(0x14);
    if first == 0 {
        show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x4d9);
        return 0;
    }
    let mut placed = 0usize;
    while placed < 5 {
        let r = exe::utils_random(0xa) as usize;
        if taken[r] == 0 {
            write_usize(first + 4 * placed, wc_asia_first_round_winner(l, r as i32 - 1));
            placed += 1;
            taken[r] = 1;
        }
    }
    let build = |list: usize, stage: u32| {
        let (table, rounds, extra) = stage_round_table(l, stage);
        let p = qualifying_league_stage(l, 5, list, table, rounds, extra, places.as_ptr() as usize, stage, 0xf, 4);
        write_usize(l.stages() + 4 * stage as usize, p);
        exe::crt_free(list);
        exe::crt_free(table);
        read_usize(l.stages() + 4 * stage as usize)
    };
    let stage = build(first, 9);
    if stage == 0 {
        show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x502);
        return 0;
    }
    covered!("rx09_17 asian second round groups");
    exe::league_send_group_draw_news(stage);
    l.set_current_stage(9);
    let second = exe::crt_malloc(0x14);
    if second == 0 {
        show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x510);
        return 0;
    }
    let mut placed = 0usize;
    for k in -1i32..9 {
        if taken[(k + 1) as usize] == 0 {
            write_usize(second + 4 * placed, wc_asia_first_round_winner(l, k)); // rx09_17: the winners not drawn into the first group
            placed += 1;
        }
    }
    let stage = build(second, 10);
    if stage == 0 {
        show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x534);
        return 0;
    }
    l.set_current_stage(10);
    exe::league_send_group_draw_news(stage);
    1
}

/// Comparer for `wc_asia_league_create_second_round_groups`: only the original runs (it draws and allocates); the two
/// groups together hold the ten first round winners, five each, and the current stage is 10.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_create_second_round_groups(this: usize) -> u32 {
    let l = League(this);
    let mut winners: Vec<usize> = (-1i32..9).map(|k| wc_asia_first_round_winner(l, k)).collect();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_ASIA_LEAGUE_CREATE_SECOND_ROUND_GROUPS)(this);
    if r == 1 {
        let (a, b) = (League(read_usize(l.stages() + 4 * 9)), League(read_usize(l.stages() + 4 * 10)));
        let (ca, cb) = (category_b_stage_clubs(a), category_b_stage_clubs(b));
        let mut found: Vec<usize> = ca.iter().chain(cb.iter()).copied().collect();
        found.sort_unstable();
        winners.sort_unstable();
        SCOREBOARD.record(ca.len() == 5 && cb.len() == 5 && found == winners && l.current_stage() == 10, || format!("wc_asia_league_create_second_round_groups({:#x}) winners {:x?} found {:x?} current {}", this,
            winners, found, l.current_stage() as i32));
    }
    r
}

// ---------- rx09_18: the Asian World Cup qualifying third round, play-off and predicted positions (wc_asia_league.cpp) ----------

/// `WcEuropeLeague` 0x91def0 (thiscall, ret 0): the European nation drawn for the Asian play-off [inference: the first
/// team of the European runners-up stage, +0x100, that is not in the play-off stage, +0x101].
const FN_WC_EUROPE_LEAGUE_ASIAN_PLAY_OFF_NATION: usize = 0x91def0;

/// The two sides of the Asian third round tie: from each second round group (stages 9 and 10), the third-placed team
/// (table row 2) with no exempt nation, else the runner-up (row 1).
pub(crate) unsafe fn wc_asia_third_round_sides(this: usize) -> [usize; 2] {
    let l = League(this);
    let row = if read_u8(this + WC_ASIA_EXEMPT) == 0 { 2 } else { 1 }; // rx09_18: the third-placed sides with no exempt nation
    [9usize, 10].map(|s| read_usize(exe::league_get_table_row(read_usize(l.stages() + 4 * s), row)))
}

/// Builds the Asian third round tie as stage 11: a list of two (`exe::crt_malloc` of 8; none: the box at line 0x552)
/// with `wc_asia_third_round_sides`, the round table of stage 11 (`stage_round_table`), the Cup_Stage (`league_cup_stage`:
/// two teams, stage 11, 4) into slot 11; the list and the round table freed; none: the box at line 0x572. Otherwise its
/// round set up (Cup::set_up_round `exe::cup_set_up_round`(stage, 0)) and the current stage = 11.
/// Original: `WcAsiaLeague::create_third_round_tie` (0x913b60, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_create_third_round_tie(this: usize) {
    let l = League(this);
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x552);
    }
    let sides = wc_asia_third_round_sides(this);
    write_usize(list, sides[0]);
    write_usize(list + 4, sides[1]);
    let (table, rounds, extra) = stage_round_table(l, 0xb);
    let p = league_cup_stage(l, 2, list, table, rounds, extra, 0xb, 4, 0);
    write_usize(l.stages() + 4 * 0xb, p);
    exe::crt_free(list);
    exe::crt_free(table);
    if read_usize(l.stages() + 4 * 0xb) == 0 {
        return show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x572);
    }
    covered!("rx09_18 asian third round tie");
    exe::cup_set_up_round(read_usize(l.stages() + 4 * 0xb), 0);
    l.set_current_stage(0xb);
}

/// Comparer for `wc_asia_league_create_third_round_tie`: only the original runs (it allocates); the tie's teams and the
/// current stage against the plan.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_create_third_round_tie(this: usize) {
    let plan = wc_asia_third_round_sides(this);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_ASIA_LEAGUE_CREATE_THIRD_ROUND_TIE)(this);
    compare_league_cup_stage("wc_asia_league_create_third_round_tie", League(this), 0xb, &plan);
}

/// The Asian side of the play-off: the first entry of the third round tie's team table (`cup::TEAM_TABLE`, the count
/// re-read) whose status is 2 (lost) with one exempt nation, else 1 (won); none: None (the original leaves the slot as
/// malloc returned it).
pub(crate) unsafe fn wc_asia_play_off_side(this: usize) -> Option<usize> {
    let tie = Cup(read_usize(League(this).stages() + 4 * 0xb));
    let mut i = 0i16;
    while i < tie.team_count() {
        let want = if read_u8(this + WC_ASIA_EXEMPT) == 1 { 2 } else { 1 }; // rx09_18: the third round winner goes on
        let e = tie.team_table() + 6 * i as usize;
        if read_u8(e + cup_team_entry::STATUS) == want {
            return Some(read_usize(e + cup_team_entry::CLUB));
        }
        i += 1;
    }
    None
}

/// The European nation of the play-off (WcEuropeLeague 0x91def0 on the European section's object:
/// `KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION` in [`G_COMP_OBJECT_TABLE`]).
pub(crate) unsafe fn wc_asia_play_off_european_side() -> usize {
    let europe = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION).wrapping_mul(4)));
    let pick: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_WC_EUROPE_LEAGUE_ASIAN_PLAY_OFF_NATION);
    pick(europe)
}

/// Builds the Asian play-off as stage 12: a list of two (`exe::crt_malloc` of 8; none: the box at line 0x591) with the
/// Asian side (`wc_asia_play_off_side`; when none, the slot is left as malloc returned it) and the European one
/// (`wc_asia_play_off_european_side`); the round table of stage 12, the Cup_Stage (`league_cup_stage`: two teams, stage
/// 12, 4) into slot 12; the list and the round table freed; none: the box at line 0x5be. Otherwise its round set up
/// (`exe::cup_set_up_round`(stage, 0)) and the current stage = 12.
/// Original: `WcAsiaLeague::create_playoff_tie` (0x913d50, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_create_playoff_tie(this: usize) {
    let l = League(this);
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x591);
    }
    if let Some(side) = wc_asia_play_off_side(this) {
        write_usize(list, side);
    }
    write_usize(list + 4, wc_asia_play_off_european_side());
    let (table, rounds, extra) = stage_round_table(l, 0xc);
    let p = league_cup_stage(l, 2, list, table, rounds, extra, 0xc, 4, 0);
    write_usize(l.stages() + 4 * 0xc, p);
    exe::crt_free(list);
    exe::crt_free(table);
    if read_usize(l.stages() + 4 * 0xc) == 0 {
        return show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x5be);
    }
    covered!("rx09_18 asian play-off");
    exe::cup_set_up_round(read_usize(l.stages() + 4 * 0xc), 0);
    l.set_current_stage(0xc);
}

/// Comparer for `wc_asia_league_create_playoff_tie`: only the original runs (it allocates); with an Asian side, the
/// tie's teams and the current stage against the plan.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_create_playoff_tie(this: usize) {
    let side = wc_asia_play_off_side(this);
    let europe = wc_asia_play_off_european_side();
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_ASIA_LEAGUE_CREATE_PLAYOFF_TIE)(this);
    if let Some(side) = side {
        compare_league_cup_stage("wc_asia_league_create_playoff_tie", League(this), 0xc, &[side, europe]);
    }
}

/// The predicted place of the Asian entrant in place k (by reputation), by the exempt count: none exempt, 1 for the top
/// four, then 5, 6, 7 (to the eighth), 9 (to the tenth), 11 (to the twentieth), 21 (to the thirtieth), 31; one or two,
/// 1 for the top two, then 3, 4, 5 (to the sixth), 7 (to the eighth), 9, 11, 21, 31; three, 1, 2, 3 (to the fourth), 5,
/// 7, 9, 11, 21, 31; any other count, no place.
pub(crate) fn wc_asia_predicted_place(exempt: i8, k: i16) -> Option<u8> {
    let bands: &[(i16, u8)] = match exempt {
        0 => &[(4, 1), (5, 5), (6, 6), (8, 7), (10, 9), (20, 0xb), (30, 0x15)],
        1 | 2 => &[(2, 1), (3, 3), (4, 4), (6, 5), (8, 7), (10, 9), (20, 0xb), (30, 0x15)],
        3 => &[(1, 1), (2, 2), (4, 3), (6, 5), (8, 7), (10, 9), (20, 0xb), (30, 0x15)],
        _ => return None,
    };
    Some(bands.iter().find(|&&(limit, _)| k < limit).map_or(0x1f, |&(_, p)| p)) // rx09_18: the reputation bands
}

/// The predicted positions of the Asian qualifying: without statistics, the box at line 0x6bb and nothing else. The
/// entrants (`wc_turnover_consts::ENTRANT_LIST`, `WC_ASIA_ENTRANT_COUNT`) sorted by reputation (qsort `exe::crt_qsort`
/// with 0x4b07d0); the Asian national teams released from the continent's competitions
/// (`exe::nationalteams_release_competitions_of_continent`(the Asian continent record, 0) on [`G_NATIONAL_TEAMS`]);
/// then each entrant k gets its predicted place (`wc_asia_predicted_place` with the exempt count read again each time;
/// `exe::comp_stats_set_club_final_position`) and this competition as its nation's current one
/// (`exe::nationalteams_set_current_competition`(the entrant's nation, the record)).
/// Original: `WcAsiaLeague::set_predicted_positions` (0x9144d0, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_set_predicted_positions(this: usize) {
    let l = League(this);
    if l.comp_stats() == 0 {
        return show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x6bb);
    }
    covered!("rx09_18 asian predicted positions");
    let list = read_usize(this + wc_turnover_consts::ENTRANT_LIST);
    exe::crt_qsort(list, read_u32(this + WC_ASIA_ENTRANT_COUNT), 4, 0x4b07d0);
    let continent = read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(KEY_CONTINENT_ASIA).wrapping_mul(0xc6));
    exe::nationalteams_release_competitions_of_continent(read_usize(G_NATIONAL_TEAMS), continent, 0);
    let mut k = 0i16;
    while (k as i32) < read_i32(this + WC_ASIA_ENTRANT_COUNT) {
        let team = read_usize(list + 4 * k as usize);
        if let Some(p) = wc_asia_predicted_place(read_u8(this + WC_ASIA_EXEMPT) as i8, k) {
            exe::comp_stats_set_club_final_position(l.comp_stats(), team, p as u32);
        }
        exe::nationalteams_set_current_competition(read_usize(G_NATIONAL_TEAMS), read_usize(team + club::NATION), l.competition().addr());
        k = k.wrapping_add(1);
    }
}

/// Comparer for `wc_asia_league_set_predicted_positions`: only the original runs (it sorts the list and moves the
/// national teams); then each entrant's predicted place byte (`comp_stats_club::FINAL_POSITION`) against the plan, over
/// the sorted list.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_set_predicted_positions(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_ASIA_LEAGUE_SET_PREDICTED_POSITIONS)(this);
    let l = League(this);
    let stats = l.comp_stats();
    if stats == 0 {
        return;
    }
    let list = read_usize(this + wc_turnover_consts::ENTRANT_LIST);
    let exempt = read_u8(this + WC_ASIA_EXEMPT) as i8;
    let mut bad = Vec::new();
    for k in 0..read_i32(this + WC_ASIA_ENTRANT_COUNT).max(0) as i16 {
        let e = exe::comp_stats_find_club_entry(stats, read_usize(list + 4 * k as usize));
        if let (Some(p), true) = (wc_asia_predicted_place(exempt, k), e != 0) {
            if read_u8(e + comp_stats_club::FINAL_POSITION) != p {
                bad.push((k, p, read_u8(e + comp_stats_club::FINAL_POSITION)));
            }
        }
    }
    SCOREBOARD.record(bad.is_empty(), || format!("wc_asia_league_set_predicted_positions({:#x}) exempt {}: (place, plan, original) {:?}", this, exempt, bad));
}

// ---------- rx09_19: the Asian World Cup qualifying staff records, fate news text and progress codes (wc_asia_league.cpp) ----------

/// The World Cup's record (`KEY_CLUB_COMP_FIFA_WORLD_CUP` in [`G_CLUB_COMP_TABLE`], 0x6b bytes each).
pub(crate) unsafe fn world_cup_record() -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(0x6b))
}

/// A nation through to the World Cup from the Asian qualifying: the World Cup qualification record (weight 100), its
/// place in the World Cup (`exe::worldcup_add_qualified_team` on the World Cup object, [`G_COMP_OBJECT_TABLE`]), `*out`
/// = 4 (not checked for null) and NationalTeams::add_qualified_nation(club, record) on [`G_NATIONAL_TEAMS`].
pub(crate) unsafe fn wc_asia_qualify_nation(club: usize, out: usize) {
    european_cup_add_qualified_record(club, world_cup_record(), 0, 0, 0x64);
    let world_cup = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(4)));
    exe::worldcup_add_qualified_team(world_cup, club); // rx09_19: the Asian nation enters the World Cup
    write_u8(out, 4);
    let add: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_ADD_QUALIFIED_NATION);
    add(read_usize(G_NATIONAL_TEAMS), club, world_cup_record());
}

/// A nation out of the Asian qualifying: the failed-to-qualify record (`european_cup_add_failed_to_qualify_record` with
/// `stage` and `weight`) and the national team's campaign ended (`clear_national_team_competition`).
pub(crate) unsafe fn wc_asia_eliminate(this: usize, club: usize, stage: u32, weight: u32) {
    european_cup_add_failed_to_qualify_record(club, League(this).competition().addr(), stage, weight);
    clear_national_team_competition(Club(club));
}

/// What happens to a nation by its Asian qualifying fate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum AsianFate {
    /// The qualified-for-round record with (stage, round, weight).
    Qualified(u32, u32, u32),
    /// The failed-to-qualify record with (stage, weight) and the end of the campaign.
    Out(u32, u32),
    /// Through to the World Cup (`wc_asia_qualify_nation`); the result −1.
    WorldCup,
    Nothing,
}

/// The fate of a nation with `outcome` (its byte) at `stage` (signed) of the Asian qualifying (`exempt`, the byte at
/// `WC_ASIA_EXEMPT`):
/// - the first round (below 9): 6 through (0x14, 0, 0x1e), 0xfe out (0xa, 0x1e);
/// - the second round (9, 10): 7 to the World Cup, 1 to the third round (0x1e, 0x96, 0x3c), 0xfe out (0x14, 0x3c);
/// - the third round (11): with 1 or 3 exempt nations, 2 to the play-off (0xa0, 0, 0x46) and any other to the World
///   Cup; with none or 2, 2 out (0x1e, 0x3c) and any other to the play-off;
/// - the play-off (12): 1 to the World Cup, any other out (0xa0, 0x3c);
/// - later: nothing.
pub(crate) fn wc_asia_fate(stage: i8, outcome: u8, exempt: u8) -> AsianFate {
    match stage {
        i8::MIN..=8 => match outcome {
            6 => AsianFate::Qualified(0x14, 0, 0x1e),
            0xfe => AsianFate::Out(0xa, 0x1e),
            _ => AsianFate::Nothing,
        },
        9 | 10 => match outcome {
            7 => AsianFate::WorldCup,
            1 => AsianFate::Qualified(0x1e, 0x96, 0x3c),
            0xfe => AsianFate::Out(0x14, 0x3c),
            _ => AsianFate::Nothing,
        },
        11 if exempt != 0 && exempt != 2 => if outcome == 2 { AsianFate::Qualified(0xa0, 0, 0x46) } else { AsianFate::WorldCup },
        11 => if outcome == 2 { AsianFate::Out(0x1e, 0x3c) } else { AsianFate::Qualified(0xa0, 0, 0x46) },
        12 => if outcome == 1 { AsianFate::WorldCup } else { AsianFate::Out(0xa0, 0x3c) }, // rx09_19: the play-off winner goes to the World Cup
        _ => AsianFate::Nothing,
    }
}

/// Updates the staff records by a nation's fate in the Asian qualifying (`wc_asia_fate`; the records with this
/// league's competition). A nation through to the World Cup returns −1; any other fate returns the exempt count
/// (`WC_ASIA_EXEMPT`, sign-extended). The last two arguments are not read.
/// Original: `WcAsiaLeague::update_staff_records_for_status` (0x912500, wc_asia_league.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, _game: usize, _a6: u32) -> u32 {
    match wc_asia_fate(stage as u8 as i8, outcome as u8, read_u8(this + WC_ASIA_EXEMPT)) {
        AsianFate::Qualified(s, r, w) => european_cup_add_qualified_record(club, League(this).competition().addr(), s, r, w),
        AsianFate::Out(s, w) => wc_asia_eliminate(this, club, s, w),
        AsianFate::WorldCup => {
            covered!("rx09_19 asian nation to the world cup");
            wc_asia_qualify_nation(club, out);
            return u32::MAX;
        }
        AsianFate::Nothing => {}
    }
    read_u8(this + WC_ASIA_EXEMPT) as i8 as i32 as u32
}

/// Comparer for `wc_asia_league_update_staff_records_for_status`: the records and the World Cup place do not run twice,
/// so only the original runs; the plan: the result and, for a nation through, `*out` = 4.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, a6: u32) -> u32 {
    let fate = wc_asia_fate(stage as u8 as i8, outcome as u8, read_u8(this + WC_ASIA_EXEMPT));
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(IDX_WC_ASIA_LEAGUE_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, out, game, a6);
    let expected = if fate == AsianFate::WorldCup { u32::MAX } else { read_u8(this + WC_ASIA_EXEMPT) as i8 as i32 as u32 };
    let out_ok = fate != AsianFate::WorldCup || read_u8(out) == 4;
    SCOREBOARD.record(r == expected && out_ok, || format!("wc_asia_league_update_staff_records_for_status({:#x}, outcome {:#x}, stage {}) original={:#x} plan {:?}", club, outcome as u8,
        stage as u8 as i8, r, fate));
    r
}

/// What `wc_asia_league_build_fate_news_text` writes for a nation (the cup and group news are the shared routines').
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum AsianFateText {
    /// `exe::comp_util_write_group_qualify_news`(club, a4, the competition, the outcome or −2, the news item or 0, text).
    Group(i32, u32),
    /// `exe::comp_util_write_cup_tie_result_news`(club, a5, a8, outcome, a6, text).
    CupTie,
    /// A club text (`play_off_club_text`) with this format.
    Club(usize),
    /// A club and World Cup text (`club_and_competition_short_text` with the World Cup's record) with this format.
    WorldCup(usize),
    Nothing,
}

/// The text of a nation's fate in the Asian qualifying:
/// - the first round (below 9): with a news item the group news (outcome, news); without, 6 gives 0xad47bc "{nation}
///   qualify for second round", 0xfe the group news with −2 and 0;
/// - the second round (9, 10): with a news item the group news; without, 7 gives 0xad4738 "{nation} qualify for {World
///   Cup}", 1 0xad4780 "{nation} qualify for third round", 0xfe the group news with −2 and 0;
/// - the third round (11): with a news item the cup tie news; without, when `a7` is 1, outcome 1 "qualify for {World
///   Cup}" and any other 0xad46c0 "{nation} out of {World Cup}"; otherwise outcome 1 0xad4700 "{nation} qualify for
///   playoff" and any other "out of {World Cup}";
/// - the play-off (12): with a news item the cup tie news; without, 1 "qualify for {World Cup}", any other "out of
///   {World Cup}";
/// - later: nothing.
pub(crate) fn wc_asia_fate_text(stage: i8, outcome: u8, a7: u8, news: u32) -> AsianFateText {
    let group_out = |o: u8| if o == 0xfe { AsianFateText::Group(-2, 0) } else { AsianFateText::Nothing };
    match stage {
        i8::MIN..=8 if news != 0 => AsianFateText::Group(outcome as i8 as i32, news),
        i8::MIN..=8 => if outcome == 6 { AsianFateText::Club(0xad47bc) } else { group_out(outcome) },
        9 | 10 if news != 0 => AsianFateText::Group(outcome as i8 as i32, news),
        9 | 10 => match outcome {
            7 => AsianFateText::WorldCup(0xad4738),
            1 => AsianFateText::Club(0xad4780),
            o => group_out(o),
        },
        11 | 12 if news != 0 => AsianFateText::CupTie,
        11 if a7 != 1 => if outcome == 1 { AsianFateText::Club(0xad4700) } else { AsianFateText::WorldCup(0xad46c0) },
        11 | 12 => if outcome == 1 { AsianFateText::WorldCup(0xad4738) } else { AsianFateText::WorldCup(0xad46c0) }, // rx09_19: qualify for / out of the World Cup
        _ => AsianFateText::Nothing,
    }
}

/// The fate news text of an Asian qualifying nation (thiscall, ret 0x28; the arguments of Comp::build_fate_news_text):
/// `wc_asia_fate_text` with the outcome byte, the stage (signed) and the a7 byte. The group news keeps the full outcome
/// argument when it passes it on (the −2 is a constant).
/// Original: `WcAsiaLeague::build_fate_news_text` (0x912780, wc_asia_league.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn wc_asia_league_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let comp = League(this).competition().addr();
    match wc_asia_fate_text(stage as u8 as i8, outcome as u8, a7 as u8, news) {
        AsianFateText::Group(-2, _) => exe::comp_util_write_group_qualify_news(club_id, a4, comp, -2, 0, text),
        AsianFateText::Group(_, _) => exe::comp_util_write_group_qualify_news(club_id, a4, comp, outcome as i32, news, text),
        AsianFateText::CupTie => exe::comp_util_write_cup_tie_result_news(club_id, a5, a8, outcome, a6, text),
        AsianFateText::Club(format) => {
            covered!("rx09_19 asian round text");
            play_off_club_text(club_id, format, text);
        }
        AsianFateText::WorldCup(format) => {
            covered!("rx09_19 asian world cup text");
            club_and_competition_short_text(club_id, world_cup_record(), format, true, text);
        }
        AsianFateText::Nothing => {}
    }
}

/// Comparer for `wc_asia_league_build_fate_news_text`: the texts the routine writes itself side by side with
/// `compare_text_writer`; the shared group and cup news only the original (it writes the Text once).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(IDX_WC_ASIA_LEAGUE_BUILD_FATE_NEWS_TEXT);
    let s = stage as u8 as i8;
    let own = matches!(wc_asia_fate_text(s, outcome as u8, a7 as u8, news), AsianFateText::Club(_) | AsianFateText::WorldCup(_));
    if !own || text == 0 {
        return o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| wc_asia_league_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t),
        || format!("wc_asia_league_build_fate_news_text(club {}, outcome {:#x}, stage {}, a7 {}, news {})", club_id, outcome as u8, s, a7 as u8, news));
}

/// The three progress codes (current, best, worst) of an Asian qualifying nation at a stage (bytes; `exempt` the byte at
/// `WC_ASIA_EXEMPT`):
/// - the first round (−1 to 8): best 1, 2, 3, 4 become 1, 11, 21, 31; worst and current 1, 2, 3, 4 become 9, 11, 21, 31
///   (the group winner's current code is 9, as its worst);
/// - the second round (9, 10): best 1 and 2 become 1, then 3, 4, 5 become 5, 7, 9; worst and current 1 and 2 become 4,
///   then 5, 7, 9;
/// - the third round (11): with no exempt nation best 1, worst and current below 4 become 1; with one, best below 4 and
///   worst and current below 3 become 1; with two, best and worst and current below 3, 2 and 2 become 1; otherwise best
///   below 2 becomes 1;
/// - any other code, and the other stages (below −1, or 12 on): as it came.
pub(crate) fn wc_asia_progress(stage: i8, exempt: u8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    let map = |v: u8, table: &[u8]| if (1..=table.len()).contains(&(v as usize)) { table[v as usize - 1] } else { v };
    let below = |v: u8, n: u8| if v < n { 1 } else { v };
    match stage {
        -1..=8 => (map(current, &[9, 0xb, 0x15, 0x1f]), map(best, &[1, 0xb, 0x15, 0x1f]), map(worst, &[9, 0xb, 0x15, 0x1f])), // rx09_19: the first round codes
        9 | 10 => (map(current, &[4, 4, 5, 7, 9]), map(best, &[1, 1, 5, 7, 9]), map(worst, &[4, 4, 5, 7, 9])),
        11 => match exempt {
            0 => (below(current, 4), 1, below(worst, 4)),
            1 => (below(current, 3), below(best, 4), below(worst, 3)),
            2 => (below(current, 2), below(best, 3), below(worst, 2)),
            _ => (current, below(best, 2), worst),
        },
        _ => (current, best, worst),
    }
}

/// Stores an Asian qualifying nation's current, best and worst position codes in its statistics entry
/// (`exe::comp_stats_find_club_entry`, not checked), turned into the overall codes by `wc_asia_progress`.
/// Original: `WcAsiaLeague::set_club_progress` (0x914730, wc_asia_league.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("rx09_19 asian progress");
    let entry = CompStatsClub(exe::comp_stats_find_club_entry(League(this).comp_stats(), club));
    let (c, b, w) = wc_asia_progress(stage as u8 as i8, read_u8(this + WC_ASIA_EXEMPT), current as u8, best as u8, worst as u8);
    entry.set_best_position(b);
    entry.set_current_position(c);
    entry.set_worst_position(w);
}

/// Comparer for `wc_asia_league_set_club_progress`: the three position bytes of the nation's entry side by side.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_WC_ASIA_LEAGUE_SET_CLUB_PROGRESS);
    let stats = League(this).comp_stats();
    let entry = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if entry == 0 {
        return o(this, club, stage, current, best, worst);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(entry + comp_stats_club::CURRENT_POSITION, 3)], || o(this, club, stage, current, best, worst),
        || wc_asia_league_set_club_progress(this, club, stage, current, best, worst));
    SCOREBOARD.record(same, || format!("wc_asia_league_set_club_progress({:#x}, stage {}, {}, {}, {}): original {:x?} rust {:x?}", club, stage as u8 as i8, current as u8, best as u8,
        worst as u8, mo, mr));
}

// ---------- rx11_1: the news texts of a nation's fate in the African and CONCACAF World Cup qualifying (wc_african_cup.cpp, wc_concacaf_cup.cpp) ----------

/// The fate news templates of the African qualifying (langlib, `.rdata`).
pub(crate) mod wc_african_fate_texts {
    /// "{}<%s - Team Name (e.g. Scotland)>{} qualify for Final"
    pub(crate) const QUALIFY_FOR_FINAL: usize = 0xad45fc;
    /// "{}<%s - Team Name (e.g. Scotland)>{} qualify for {}<%s - World Cup>{}" (from a group)
    pub(crate) const GROUP_QUALIFY_FOR_WORLD_CUP: usize = 0xad45b4;
    /// "{}<%s - Team Name (e.g. South Africa)>{} qualify for 2nd round in qualifiers"
    pub(crate) const QUALIFY_FOR_SECOND_ROUND: usize = 0xad4564;
    /// "{}<%s - Team Name (e.g. South Africa)>{} knocked out in 1st round qualifier"
    pub(crate) const KNOCKED_OUT_IN_FIRST_ROUND: usize = 0xad4518;
    /// "{}<%s - Team Name (e.g. South Africa)>{} qualify for {}<%s - World Cup>{}" (from a tie)
    pub(crate) const TIE_QUALIFY_FOR_WORLD_CUP: usize = 0xad44cc;
    /// "{}<%s - Team Name (e.g. South Africa)>{} knocked out of {}<%s - World Cup>{}"
    pub(crate) const KNOCKED_OUT_OF_WORLD_CUP: usize = 0xad447c;
}

/// The fate news templates of the CONCACAF qualifying (langlib, `.rdata`).
pub(crate) mod wc_concacaf_fate_texts {
    /// "{}<%s - Team Name(e.g.Canada)>{} qualify for Semi Finals"
    pub(crate) const QUALIFY_FOR_SEMI_FINALS: usize = 0xad49e4;
    /// "{}<%s - Team Name(e.g.Canada)>{} have qualified from the semi final stage of the {}<%s - Competition Name>{}."
    pub(crate) const QUALIFIED_FROM_SEMI_FINAL_STAGE: usize = 0xad4974;
    /// "{}<%s - Team Name(e.g.Canada)>{} qualify for final"
    pub(crate) const QUALIFY_FOR_FINAL: usize = 0xad4940;
    /// "{}<%s - Team Name(e.g.Canada)>{} have qualified from the final stage of the {}<%s - Competition Name>{}."
    pub(crate) const QUALIFIED_FROM_FINAL_STAGE: usize = 0xad48d4;
    /// "{}<%s - Team Name(e.g.Canada)>{} qualify for {}<%s - World Cup>{}"
    pub(crate) const QUALIFY_FOR_WORLD_CUP: usize = 0xad4890;
}

/// The club of a club index in [`G_CLUB_TABLE`] (`club::SIZE` bytes each).
pub(crate) unsafe fn club_of_index(index: u32) -> Club {
    Club(read_usize(G_CLUB_TABLE).wrapping_add((index as usize).wrapping_mul(club::SIZE)))
}

/// One line of a qualifying fate news: `template` (`FN_LANGLIB_SPRINTF`, cdecl, into `NEWS_TEXT_BUFFER`) with the
/// club's short gender twice and, with `other` (a gender byte's address and a name), that gender twice, then the club's
/// short name and the other name; set as the text (`exe::text_set_string`). The caller adds the links.
pub(crate) unsafe fn qualifying_fate_line(text: usize, template: usize, club: Club, other: Option<(usize, usize)>) {
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let g = club.short_gender_name() as i8 as i32;
    let name = club.addr() + club::SHORT_NAME;
    match other {
        Some((gender_at, other_name)) => {
            let og = read_u8(gender_at) as i8 as i32;
            sprintf(NEWS_TEXT_BUFFER, template, g, g, og, og, name, other_name);
        }
        None => {
            sprintf(NEWS_TEXT_BUFFER, template, g, g, name);
        }
    }
    exe::text_set_string(text, NEWS_TEXT_BUFFER as *const u8);
}

/// The club's link in a fate news (its short name, kind 0x7d5).
pub(crate) unsafe fn link_fate_club(text: usize, club: Club) {
    exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
}

/// A competition record's link in a fate news (its short name, kind 0x7d0).
pub(crate) unsafe fn link_fate_competition(text: usize, record: usize) {
    exe::text_add_link(text, (record + competition::SHORT_NAME) as *const u8, 0x7d0, record);
}

/// rule: the African qualifying fate news: in the groups (stage codes 0x407 to 0x40b and the final group 0x472), the
/// shared group sentence, or, in short, "qualify for Final" (fate 6) and "qualify for the World Cup" (fate 7); in the
/// first round (stage −1) and in the ties (code 0), the shared tie sentence, or, in short, "qualify for 2nd round" or
/// "knocked out in 1st round" in the first round and "qualify for" or "knocked out of" the World Cup in the ties
///
/// `this.build_fate_news_text(club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, text)` (virtual +0x48; the
/// club in [`G_CLUB_TABLE`] by index, the fate byte, the stage byte (signed), the stage code (a signed word), four
/// values passed on, `full_sentence` and the Text).
///
/// Stage other than −1 with a code: a code outside 0x407..=0x40b and 0x472 writes nothing. With `full_sentence`, the
/// shared group sentence (`exe::comp_util_write_group_qualify_news`(club_index, code, the competition record
/// `league::COMPETITION`, the fate dword, full_sentence, text), cdecl). Otherwise fate 6: "qualify for Final" with the
/// club's link; fate 7: "qualify for the World Cup" (the World Cup record of [`KEY_CLUB_COMP_FIFA_WORLD_CUP`]: its short
/// gender and short name) with the club's link and the World Cup's; any other fate, the shared group sentence with 0
/// for `full_sentence`.
///
/// Stage −1 or code 0: with `full_sentence`, the shared tie sentence (`exe::comp_util_write_cup_tie_result_news`(
/// club_index, a5, a8, the fate dword, a6, text), cdecl). Otherwise in the first round (stage −1) fate 1 "qualify for
/// 2nd round in qualifiers", any other "knocked out in 1st round qualifier" (the club only); in a tie fate 1 "qualify
/// for", any other "knocked out of" the World Cup (its gender and short name); then the World Cup's link and the
/// club's, in that order.
/// Original: `WcAfricanCup::build_fate_news_text` (0x90fe00, wc_african_cup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn wc_african_cup_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, code: u32, a5: u32, a6: u32, a7: u32, a8: u32, full_sentence: u32, text: usize) {
    use wc_african_fate_texts as t;
    let _ = a7;
    let comp = League(this).competition().addr();
    let club = club_of_index(club_index);
    let world_cup = key_comp_record(KEY_CLUB_COMP_FIFA_WORLD_CUP);
    let wc_name = Some((world_cup + competition::SHORT_GENDER_NAME, world_cup + competition::SHORT_NAME));
    let first_round = stage as u8 as i8 == -1;
    let raw_code = code;
    let code = code as u16 as i16;
    // simplification: on the paths without text the original leaves al with a byte of the arguments; the callers do not read it
    if !first_round && code != 0 {
        if !((0x407..=0x40b).contains(&code) || code == 0x472) {
            return;
        }
        covered!("rx11_1 african group fate");
        if full_sentence != 0 {
            return exe::comp_util_write_group_qualify_news(club_index, raw_code, comp, fate as i32, full_sentence, text);
        }
        match fate as u8 {
            6 => {
                qualifying_fate_line(text, t::QUALIFY_FOR_FINAL, club, None);
                link_fate_club(text, club);
            }
            7 => {
                qualifying_fate_line(text, t::GROUP_QUALIFY_FOR_WORLD_CUP, club, wc_name);
                link_fate_club(text, club); // rx11_1: the club's link first in a group
                link_fate_competition(text, world_cup);
            }
            _ => exe::comp_util_write_group_qualify_news(club_index, raw_code, comp, fate as i32, 0, text),
        }
        return;
    }
    covered!("rx11_1 african tie fate");
    if full_sentence != 0 {
        return exe::comp_util_write_cup_tie_result_news(club_index, a5, a8, fate, a6, text);
    }
    let through = fate as u8 == 1;
    if first_round {
        qualifying_fate_line(text, if through { t::QUALIFY_FOR_SECOND_ROUND } else { t::KNOCKED_OUT_IN_FIRST_ROUND }, club, None);
    } else {
        qualifying_fate_line(text, if through { t::TIE_QUALIFY_FOR_WORLD_CUP } else { t::KNOCKED_OUT_OF_WORLD_CUP }, club, wc_name);
    }
    link_fate_competition(text, world_cup); // rx11_1: the World Cup's link first in a tie
    link_fate_club(text, club);
}

/// rule: the CONCACAF qualifying fate news: in the Caribbean knockout (code 0x40f) and the play-off (stage 2), the
/// shared tie sentence, or, in short, "qualify for Semi Finals" for a winner and the shared win-or-defeat sentence
/// otherwise; in the semi-final groups (codes 0x40c to 0x40e), "have qualified from the semi final stage" (or, in
/// short, "qualify for final") for a qualifier (6) and the group sentence for one going out (−2); in the final group
/// (code 0x96), "have qualified from the final stage" (or, in short, "qualify for the World Cup") for a qualifier (7)
/// and the group sentence for one going out (−2)
///
/// `this.build_fate_news_text(club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, text)` (virtual +0x48), as
/// the African one. Code 0x40f or stage byte 2: with `full_sentence`, `exe::comp_util_write_cup_tie_result_news`(
/// club_index, a5, a8, the fate dword, a6, text); otherwise fate 1 "qualify for Semi Finals" with the club's link, any
/// other `exe::comp_util_write_cup_win_or_defeat_news`(club_index, a5, the fate dword, text) (cdecl).
///
/// Codes 0x40c..=0x40e (signed): fate 6, with `full_sentence` "have qualified from the semi final stage of the" the
/// competition (its long name `competition::NAME` with the short gender byte), the club's link and the competition's
/// (its short name); without, "qualify for final" with the club's link. Fate 0xfe: the shared group sentence
/// (`exe::comp_util_write_group_qualify_news`(club_index, code, the competition record, −2, full_sentence, text)). Code
/// 0x96: fate 7, with `full_sentence` "have qualified from the final stage of the" competition and the two links;
/// without, "qualify for" the World Cup (the record of [`KEY_CLUB_COMP_FIFA_WORLD_CUP`]: short gender and short name)
/// with the club's link and the World Cup's; fate 0xfe, the group sentence with code 0x96. Any other code or fate,
/// nothing.
/// Original: `WcConcacafCup::build_fate_news_text` (0x9159b0, wc_concacaf_cup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, code: u32, a5: u32, a6: u32, a7: u32, a8: u32, full_sentence: u32, text: usize) {
    use wc_concacaf_fate_texts as t;
    let _ = a7;
    let comp = League(this).competition().addr();
    let club = club_of_index(club_index);
    let raw_code = code;
    let code = code as u16 as i16;
    // simplification: on the paths without text the original leaves al with a byte of the arguments; the callers do not read it
    if code == 0x40f || stage as u8 == 2 {
        covered!("rx11_1 concacaf tie fate");
        if full_sentence != 0 {
            return exe::comp_util_write_cup_tie_result_news(club_index, a5, a8, fate, a6, text);
        }
        if fate as u8 == 1 {
            qualifying_fate_line(text, t::QUALIFY_FOR_SEMI_FINALS, club, None);
            return link_fate_club(text, club);
        }
        return exe::comp_util_write_cup_win_or_defeat_news(club_index, a5, fate, text);
    }
    let (qualified, from_stage, in_short) = if (0x40c..=0x40e).contains(&code) {
        (6, t::QUALIFIED_FROM_SEMI_FINAL_STAGE, t::QUALIFY_FOR_FINAL)
    } else if code == 0x96 {
        (7, t::QUALIFIED_FROM_FINAL_STAGE, t::QUALIFY_FOR_WORLD_CUP)
    } else {
        return;
    };
    covered!("rx11_1 concacaf group fate");
    match fate as u8 {
        f if f == qualified => {
            if full_sentence != 0 {
                qualifying_fate_line(text, from_stage, club, Some((comp + competition::SHORT_GENDER_NAME, comp + competition::NAME)));
                link_fate_club(text, club);
                link_fate_competition(text, comp);
            } else if code == 0x96 {
                let world_cup = key_comp_record(KEY_CLUB_COMP_FIFA_WORLD_CUP);
                qualifying_fate_line(text, in_short, club, Some((world_cup + competition::SHORT_GENDER_NAME, world_cup + competition::SHORT_NAME)));
                link_fate_club(text, club);
                link_fate_competition(text, world_cup); // rx11_1: the World Cup from the final group
            } else {
                qualifying_fate_line(text, in_short, club, None);
                link_fate_club(text, club);
            }
        }
        0xfe => exe::comp_util_write_group_qualify_news(club_index, if code == 0x96 { 0x96 } else { raw_code }, comp, -2, full_sentence, text),
        _ => {}
    }
}

/// Comparers of the two fate texts: side by side into two Texts (`compare_text_writer`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, code: u32, a5: u32, a6: u32, a7: u32, a8: u32, full_sentence: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(IDX_WC_AFRICAN_CUP_BUILD_FATE_NEWS_TEXT);
    compare_text_writer(text, || o(this, club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, text),
        |t| wc_african_cup_build_fate_news_text(this, club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, t),
        || format!("wc_african_cup_build_fate_news_text(club {}, fate {}, stage {}, code {:#x}, full {})", club_index, fate as u8, stage as u8 as i8, code as u16, full_sentence));
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, code: u32, a5: u32, a6: u32, a7: u32, a8: u32, full_sentence: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(IDX_WC_CONCACAF_CUP_BUILD_FATE_NEWS_TEXT);
    compare_text_writer(text, || o(this, club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, text),
        |t| wc_concacaf_cup_build_fate_news_text(this, club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, t),
        || format!("wc_concacaf_cup_build_fate_news_text(club {}, fate {}, stage {}, code {:#x}, full {})", club_index, fate as u8, stage as u8 as i8, code as u16, full_sentence));
}

// ---------- rx11_2: the staff records of a nation's fate in the African and CONCACAF World Cup qualifying, and the qualifying sections' season honours (wc_african_cup.cpp, wc_concacaf_cup.cpp) ----------

/// The World Cup's records for a nation that qualifies: the qualification record (`european_cup_add_qualified_record`
/// with the World Cup record of [`KEY_CLUB_COMP_FIFA_WORLD_CUP`], stage 0, round 0 and `weight`), the place in the World
/// Cup (`exe::worldcup_add_qualified_team` on the World Cup object), `*out` = 4 and the national teams' list of
/// qualified nations (NationalTeams::add_qualified_nation(club, record) on [`G_NATIONAL_TEAMS`], thiscall, ret 8). The
/// African play-off writes `*out` after the list, the groups before it: the copy writes it first (nothing reads it in
/// between).
pub(crate) unsafe fn qualify_nation_for_world_cup(club: usize, out: usize, weight: u32) {
    let record = key_comp_record(KEY_CLUB_COMP_FIFA_WORLD_CUP);
    european_cup_add_qualified_record(club, record, 0, 0, weight);
    exe::worldcup_add_qualified_team(comp_object_of_key(KEY_CLUB_COMP_FIFA_WORLD_CUP), club);
    write_u8(out, 4);
    let add: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x7779b0usize); // NationalTeams::add_qualified_nation
    add(read_usize(G_NATIONAL_TEAMS), club, record);
}

/// A nation knocked out of a qualifying tie: the knocked-out record (`european_cup_add_knocked_out_record` with the
/// fixture's `round_prefix` and `round_name`, words, and `weight`) and the end of its campaign
/// (`clear_national_team_competition`).
pub(crate) unsafe fn knock_nation_out(club: usize, comp: usize, game: Fixture, weight: u32) {
    // simplification: the two words go in registers with garbage in the high 16 bits; 0x86c000 only uses the words
    european_cup_add_knocked_out_record(club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
    clear_national_team_competition(Club(club));
}

/// rule: in the African qualifying, a first round winner gets the qualification for round 0x14 (weight 25) and a loser
/// the knocked-out record (25) and the end of its campaign; in the groups, a group winner who qualifies for the World
/// Cup (7) gets its record (100) and place, a qualifier for the play-off (6) the qualification for round 0x96 (45), and
/// one going out (−2) the failed-to-qualify record (45) and the end of its campaign; in the play-off, a winner gets the
/// World Cup record (55) and place, a loser the knocked-out record (100) and the end of its campaign
///
/// `this.update_staff_records_for_status(club, outcome, stage, out, game, a6)` (virtual +0x44). The group stages by the
/// format byte (`wc_cup_turnover_consts::AFRICAN_FORMAT`): below stage 5 with format 0, below 4 with 1 or 3, below 6
/// with 2; the play-off: stage 6 with format 2, stage 4 with format 3. Stage −1, outcome 1:
/// `european_cup_add_qualified_record`(club, the competition `league::COMPETITION`, 0, 0x14, 0x19); outcome 2:
/// `knock_nation_out`(weight 0x19). A group, outcome 7: `qualify_nation_for_world_cup`(weight 0x64), returns −1;
/// outcome 6: the qualification (0, 0x96, 0x2d); 0xfe: `european_cup_add_failed_to_qualify_record`(0x14, 0x2d) and
/// `clear_national_team_competition`. The play-off, outcome 1: `qualify_nation_for_world_cup`(0x37), −1; outcome 2:
/// `knock_nation_out`(0x64). Anything else, 0.
/// Original: `WcAfricanCup::update_staff_records_for_status` (0x90fbf0, wc_african_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, _a6: u32) -> u32 {
    let comp = League(this).competition().addr();
    let game = Fixture(game);
    let (outcome, stage) = (outcome as u8, stage as u8 as i8);
    if stage == -1 {
        covered!("rx11_2 african first round records");
        match outcome {
            1 => european_cup_add_qualified_record(club, comp, 0, 0x14, 0x19),
            2 => knock_nation_out(club, comp, game, 0x19),
            _ => {}
        }
        return 0;
    }
    let format = read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT);
    if stage < african_groups_end(format) {
        covered!("rx11_2 african group records");
        match outcome {
            7 => {
                qualify_nation_for_world_cup(club, out, 0x64);
                return u32::MAX;
            }
            6 => european_cup_add_qualified_record(club, comp, 0, 0x96, 0x2d),
            0xfe => {
                european_cup_add_failed_to_qualify_record(club, comp, 0x14, 0x2d);
                clear_national_team_competition(Club(club)); // rx11_2: the African group's eliminated nation ends its campaign
            }
            _ => {}
        }
        return 0;
    }
    if (format == 2 && stage == 6) || (format == 3 && stage == 4) {
        covered!("rx11_2 african play-off records");
        match outcome {
            1 => {
                qualify_nation_for_world_cup(club, out, 0x37); // rx11_2: the African play-off winner to the World Cup
                return u32::MAX;
            }
            2 => knock_nation_out(club, comp, game, 0x64),
            _ => {}
        }
    }
    0
}

/// rule: in the CONCACAF qualifying, in the Caribbean knockout a tie winner gets the qualification for the next round
/// (weight 20), a loser the knocked-out record (20) and the end of its campaign, a group stage qualifier (1) the
/// qualification for round 0x82 (30); in the first Central American stages a qualifier for the semi finals (6 or 1)
/// gets its record (30) and one going out (−2) only the end of its campaign; in the play-off the winner gets the
/// semi-final qualification and the rest the knocked-out record; in the semi-final groups a qualifier (6) gets round
/// 0x96 (75) and one going out the failed-to-qualify record (75); in the final group a qualifier (7) gets the World Cup
/// record (100) and place, one going out the failed-to-qualify record (100)
///
/// `this.update_staff_records_for_status(club, outcome, stage, out, game, a6)` (virtual +0x44), stage a signed byte.
/// Stage −1: outcome 0, `european_cup_add_qualified_record`(club, comp, the fixture's `round_prefix`, the round code of
/// the round after the fixture's (`cup_round::ROUND_CODE` of entry `round_number` + 1 in the cup's `round_table`),
/// 0x14), 0; 0xff, `knock_nation_out`(0x14), 0; 1, the qualification (0x82, 0, 0x1e), 0; any other, the qualification
/// (0, 0x172, 0x1e) and −1. Stage below 2: 6, (0x82, 0, 0x1e); 1, (0, 0x172, 0x1e); 0xfe, `clear_national_team_competition`
/// only; always −1. Stage 2: 1, (0x82, 0, 0x1e); any other, `knock_nation_out`(0x14); 0. Stages 3 to 5: 6, (0x96, 0,
/// 0x4b); 0xfe, `european_cup_add_failed_to_qualify_record`(0x82, 0x4b) and the end of the campaign; 0. Stage 6: 7,
/// `qualify_nation_for_world_cup`(0x64), −1; 0xfe, the failed-to-qualify record (0x96, 0x64) and the end of the
/// campaign, 0. Later stages, 0.
/// Original: `WcConcacafCup::update_staff_records_for_status` (0x915710, wc_concacaf_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, _a6: u32) -> u32 {
    let cup = Cup(this);
    let comp = League(this).competition().addr();
    let game = Fixture(game);
    let (outcome, stage) = (outcome as u8, stage as u8 as i8);
    let qualified = |round_or_stage: (u32, u32), weight: u32| european_cup_add_qualified_record(club, comp, round_or_stage.0, round_or_stage.1, weight);
    match stage {
        -1 => {
            covered!("rx11_2 concacaf knockout records");
            match outcome {
                0 => {
                    let next = cup.round_table() + (game.round_number() as isize as usize).wrapping_mul(cup_round::SIZE) + cup_round::SIZE + cup_round::ROUND_CODE;
                    // simplification: the two words go in registers with garbage in the high 16 bits; 0x86bdd0 only uses the words
                    qualified((game.round_prefix() as u16 as u32, read_u16(next) as u32), 0x14);
                }
                0xff => knock_nation_out(club, comp, game, 0x14),
                1 => qualified((0x82, 0), 0x1e),
                _ => {
                    qualified((0, 0x172), 0x1e);
                    return u32::MAX;
                }
            }
            0
        }
        s if s < 2 => {
            covered!("rx11_2 concacaf first stage records");
            match outcome {
                6 => qualified((0x82, 0), 0x1e),
                1 => qualified((0, 0x172), 0x1e),
                0xfe => clear_national_team_competition(Club(club)), // rx11_2: the CONCACAF first stage loser has no record
                _ => {}
            }
            u32::MAX
        }
        2 => {
            covered!("rx11_2 concacaf play-off records");
            if outcome == 1 {
                qualified((0x82, 0), 0x1e);
            } else {
                knock_nation_out(club, comp, game, 0x14);
            }
            0
        }
        s if s < 6 => {
            covered!("rx11_2 concacaf semi-final group records");
            match outcome {
                6 => qualified((0x96, 0), 0x4b),
                0xfe => {
                    european_cup_add_failed_to_qualify_record(club, comp, 0x82, 0x4b);
                    clear_national_team_competition(Club(club));
                }
                _ => {}
            }
            0
        }
        6 => {
            covered!("rx11_2 concacaf final group records");
            match outcome {
                7 => {
                    qualify_nation_for_world_cup(club, out, 0x64); // rx11_2: the CONCACAF final group qualifier
                    return u32::MAX;
                }
                0xfe => {
                    european_cup_add_failed_to_qualify_record(club, comp, 0x96, 0x64);
                    clear_national_team_competition(Club(club));
                }
                _ => {}
            }
            0
        }
        _ => 0,
    }
}

/// rule: when a World Cup qualifying section closes its season, every national team entered in it leaves it
///
/// NationalTeams::clear_competition_for_comp_clubs(the competition record `league::COMPETITION`) on
/// [`G_NATIONAL_TEAMS`] (`world_cup_knockout_consts::FN_NATIONALTEAMS_CLEAR_COMPETITION_FOR_COMP_CLUBS`, thiscall, ret
/// 4), whose eax it returns. Shared by the African, Asian, CONCACAF, European and Oceania sections (virtual +0x30) and
/// one more class (vtable 0x96a0d4).
/// Original: `WcAfricanCup::record_season_honours` (0x90fbe0, wc_african_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_qualifying_record_season_honours(this: usize) -> u32 {
    covered!("rx11_2 qualifying season honours");
    let clear: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(world_cup_knockout_consts::FN_NATIONALTEAMS_CLEAR_COMPETITION_FOR_COMP_CLUBS);
    clear(read_usize(G_NATIONAL_TEAMS), League(this).competition().addr())
}

/// The plan of a qualifying staff records writer (only the original runs: the records go to the managers' histories
/// and the World Cup place and the end of a campaign do not run twice): the return value and, for a nation that
/// qualifies for the World Cup, `*out` = 4 and the World Cup's entrant count one more when it had room.
pub(crate) unsafe fn compare_qualifying_staff_records(name: &str, club: usize, outcome: u32, stage: u32, out: usize, planned: (u32, bool), original: impl FnOnce() -> u32) -> u32 {
    let (ret, to_world_cup) = planned;
    let world_cup = League(comp_object_of_key(KEY_CLUB_COMP_FIFA_WORLD_CUP));
    let watch = to_world_cup && out != 0 && !world_cup.is_null();
    let n = if watch { world_cup.entrant_count() } else { 0 };
    let room = watch && n < world_cup.entrant_capacity();
    let r = original();
    let same = r == ret && (!watch || (read_u8(out) == 4 && world_cup.entrant_count() == if room { n + 1 } else { n }));
    SCOREBOARD.record(same, || format!("{}({:#x}, outcome {}, stage {}) returned {:#x} plan {:#x}, list {}", name, club, outcome as u8, stage as u8 as i8, r, ret, n));
    r
}

/// The stage where the African groups end by the format byte (`wc_cup_turnover_consts::AFRICAN_FORMAT`): 5 for format
/// 0, 4 for 1 and 3, 6 for 2; any other format has no group stage.
pub(crate) fn african_groups_end(format: u8) -> i8 {
    match format {
        0 => 5,
        1 | 3 => 4,
        2 => 6,
        _ => i8::MIN,
    }
}

/// The planned return of the African records (and whether the nation enters the World Cup).
pub(crate) fn african_staff_records_plan(format: u8, stage: i8, outcome: u8) -> (u32, bool) {
    let group = stage != -1 && stage < african_groups_end(format);
    let play_off = matches!((format, stage), (2, 6) | (3, 4));
    if (group && outcome == 7) || (play_off && outcome == 1) { (u32::MAX, true) } else { (0, false) }
}

/// The planned return of the CONCACAF records (and whether the nation enters the World Cup).
pub(crate) fn concacaf_staff_records_plan(stage: i8, outcome: u8) -> (u32, bool) {
    match stage {
        -1 => (if matches!(outcome, 0 | 0xff | 1) { 0 } else { u32::MAX }, false),
        s if s < 2 => (u32::MAX, false),
        6 if outcome == 7 => (u32::MAX, true),
        _ => (0, false),
    }
}

pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(IDX_WC_AFRICAN_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS);
    let plan = african_staff_records_plan(read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT), stage as u8 as i8, outcome as u8);
    compare_qualifying_staff_records("wc_african_cup_update_staff_records_for_status", club, outcome, stage, out, plan, || o(this, club, outcome, stage, out, game, a6))
}

pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(IDX_WC_CONCACAF_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS);
    let plan = concacaf_staff_records_plan(stage as u8 as i8, outcome as u8);
    compare_qualifying_staff_records("wc_concacaf_cup_update_staff_records_for_status", club, outcome, stage, out, plan, || o(this, club, outcome, stage, out, game, a6))
}

/// Comparer for `wc_qualifying_record_season_honours`: the national teams leave the competition, which does not run
/// twice (and the original's club list is not freed: a second run would leak another), so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_wc_qualifying_record_season_honours(this: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_QUALIFYING_RECORD_SEASON_HONOURS)(this)
}

// ---------- rx11_3: the African World Cup qualifying stages: moving on, the groups and the play-off (wc_african_cup.cpp) ----------

/// WcAfricanCup routines that build stages (thiscall, ret 0), called directly by advance_stage.
pub(crate) const FN_WC_AFRICAN_CUP_CREATE_GROUP_STAGES: usize = 0x9109e0;
pub(crate) const FN_WC_AFRICAN_CUP_CREATE_PLAYOFF_STAGE: usize = 0x910d20;

/// The African groups by the format byte (`wc_cup_turnover_consts::AFRICAN_FORMAT`): (groups, nations per group) =
/// (5, 5) for format 0, (4, 6) for 1 and 3, (6, 4) for 2. Any other format leaves both counts uninitialised in the
/// original (stack leftovers; see DEFEITOS_DO_ORIGINAL): None.
pub(crate) fn african_group_layout(format: u8) -> Option<(usize, usize)> {
    match format {
        0 => Some((5, 5)),
        1 | 3 => Some((4, 6)), // rx11_3: four groups of six
        2 => Some((6, 4)),
        _ => None,
    }
}

/// The African play-off's size and stage index by the format byte: 6 for format 2, 4 otherwise.
pub(crate) fn african_playoff_size(format: u8) -> usize {
    if format == 2 { 6 } else { 4 }
}

/// rule: the African qualifying moves on one stage at a time: from the first round to stage 0 it builds all the
/// groups, and the play-off comes at stage 6 with format 2 or at stage 4 with format 3; past the last stage it is an
/// error
///
/// At or past the last stage (`cup::STAGE_COUNT` − 1, signed), the error box of line 0x417. Otherwise
/// `cup::CURRENT_STAGE` + 1 and, by the new stage: 0, WcAfricanCup::create_group_stages; 6 with format 2
/// (`wc_cup_turnover_consts::AFRICAN_FORMAT`) or 4 with format 3, WcAfricanCup::create_playoff_stage (thiscall, ret 0,
/// called directly).
/// Original: `WcAfricanCup::advance_stage` (0x910920, wc_african_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_advance_stage(this: usize) {
    let cup = Cup(this);
    let current = cup.current_stage() as i32;
    if current >= (cup.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(wc_cup_turnover_consts::WC_AFRICAN_CUP_CPP, 0x417);
    }
    let next = current + 1;
    cup.set_current_stage(next as usize);
    covered!("rx11_3 african next stage");
    let format = read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT);
    if next == 0 {
        call_section_routine(FN_WC_AFRICAN_CUP_CREATE_GROUP_STAGES, this);
    } else if (next == 6 && format == 2) || (next == 4 && format == 3) {
        call_section_routine(FN_WC_AFRICAN_CUP_CREATE_PLAYOFF_STAGE, this);
    }
}

/// The first round winners (the cup's team table rows with status 1, in order) placed in the empty slots of the entrant
/// list (`wc_cup_turnover_consts::CUP_ENTRANT_LIST`, +0xb6 slots), each in the first empty slot from the one after the
/// last placed; once the slots run out the rest are dropped.
pub(crate) unsafe fn place_first_round_winners(this: usize) {
    let cup = Cup(this);
    let list = read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST);
    let places = read_i32(this + 0xb6); // the entrant list's slot count
    let mut k = 0i32;
    for i in 0..cup.team_count().max(0) as usize {
        let row = cup.team_table() + i * 6;
        if read_u8(row + 5) != 1 || k >= places {
            continue;
        }
        while k < places && read_usize(list + k as usize * 4) != 0 {
            k += 1;
        }
        if k < places {
            write_usize(list + k as usize * 4, read_usize(row)); // rx11_3: a first round winner into the first free slot
            k += 1;
        }
    }
}

/// rule: the African groups are drawn by pots: the first round winners join the seeded entrants, the list is ordered by
/// reputation and group g takes entrants g, g + groups, g + 2 × groups...; each group is a round robin league stage
/// (3 points a win, 1 a draw), with its draw news
///
/// The group layout of `african_group_layout`; a team array (`exe::crt_malloc` of 4 × size; null, the error box of line
/// 0x45e). The first round winners into the list (`place_first_round_winners`), then the list sorted (`exe::crt_qsort`
/// with the reputation comparison 0x4b07d0). For each group g: the round table (virtual
/// `comp_vtable::CREATE_SCHEDULE`(g, &rounds, &label, 0)); the teams; the stage (operator new of 0xee and
/// League_Stage::League_Stage `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58: the section, size, the teams, 2, the
/// record, the round table, rounds, 3, 1, 0xa, the tie-breaks {1, 2, 3, 0}, the places {1, 0, 0, 0}, the year, g, the
/// label, 0xf, 4, 0, the code 0x24 (format below 2) or 0x28, −1, 0, 2; the original pushes the words with garbage in
/// the high bytes, which the constructor does not read: the copy passes clean values) into `cup::STAGES`[g]; without a
/// stage, the error box of line 0x48b and the routine ends (the team array and the round table are not freed); with
/// it, `cup::CURRENT_STAGE` g, the round table freed and the group draw news (`exe::league_send_group_draw_news`). Last
/// the team array freed. The SEH frame covers only the stage's new.
/// Original: `WcAfricanCup::create_group_stages` (0x9109e0, wc_african_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_create_group_stages(this: usize) {
    let cpp = wc_cup_turnover_consts::WC_AFRICAN_CUP_CPP;
    let cup = Cup(this);
    let format = read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT);
    let Some((groups, size)) = african_group_layout(format) else {
        return; // the original goes on with stack leftovers: no format of the game reaches here
    };
    let code: u32 = if (format as i8) < 2 { 0x24 } else { 0x28 };
    let teams = exe::crt_malloc((size * 4) as u32);
    if teams == 0 {
        return show_error_at(cpp, 0x45e);
    }
    covered!("rx11_3 african groups");
    place_first_round_winners(this);
    let list = read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST);
    exe::crt_qsort(list, read_u32(this + 0xb6), 4, 0x4b07d0);
    let tiebreaks: [u8; 4] = [1, 2, 3, 0];
    let places: [u8; 4] = [1, 0, 0, 0];
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    for g in 0..groups {
        let (mut rounds, mut label) = (0i16, 0i16);
        let table = create_schedule(this, g as u32, &mut rounds, &mut label, 0);
        for k in 0..size {
            write_usize(teams + k * 4, read_usize(list + (g + k * groups) * 4)); // rx11_3: the pots dealt across the groups
        }
        let p = exe::crt_operator_new(0xee);
        let stage = if p != 0 {
            construct(p, this, size as u32, teams, 2, League(this).competition().addr(), table, rounds as u16 as u32, 3, 1, 0xa, tiebreaks.as_ptr() as usize,
                places.as_ptr() as usize, cup.year() as u16 as u32, g as u32, label as u16 as u32, 0xf, 4, 0, code, -1, 0, 2)
        } else {
            0
        };
        write_usize(cup.stages() + g * 4, stage);
        if read_usize(cup.stages() + g * 4) == 0 {
            return show_error_at(cpp, 0x48b);
        }
        cup.set_current_stage(g);
        exe::crt_free(table);
        exe::league_send_group_draw_news(read_usize(cup.stages() + g * 4));
    }
    exe::crt_free(teams);
}

/// rule: the African play-off is a knockout cup stage between the group winners (four, or six with format 2), built as
/// the stage of its own index
///
/// n = `african_playoff_size`; a team array (`exe::crt_malloc` of 4n; null, the error box of line 0x4b3) with the
/// winner of each group k < n (row 0 of `cup::STAGES`[k]'s `league::TABLE`); the round table (virtual
/// `comp_vtable::CREATE_SCHEDULE`(n, &rounds, &label, 0); the original pushes n from a stack dword whose high bytes are
/// not set, read as the stage byte); the stage (operator new of 0xb2 and Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`,
/// thiscall, ret 0x3c: the section, n, the teams, rounds, the record, the round table, the year, n, 1, the label, 0x14,
/// 0, 0, 0, 0; the words clean) into `cup::STAGES`[n]; the array and the round table freed (`exe::crt_free`); without a
/// stage, the error box of line 0x4ca; with it, its first round set up (`exe::cup_set_up_round`(stage, 0)). The current
/// stage is not touched (advance_stage set it). The SEH frame covers only the stage's new.
/// Original: `WcAfricanCup::create_playoff_stage` (0x910d20, wc_african_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_create_playoff_stage(this: usize) {
    let cpp = wc_cup_turnover_consts::WC_AFRICAN_CUP_CPP;
    let cup = Cup(this);
    let n = african_playoff_size(read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT));
    let teams = exe::crt_malloc((n * 4) as u32);
    if teams == 0 {
        return show_error_at(cpp, 0x4b3);
    }
    covered!("rx11_3 african play-off");
    for k in 0..n {
        let group = League(read_usize(cup.stages() + k * 4));
        write_usize(teams + k * 4, read_usize(group.table())); // rx11_3: each group's winner
    }
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut label) = (0i16, 0i16);
    let table = create_schedule(this, n as u32, &mut rounds, &mut label, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, n as u32, teams, rounds as u16 as u32, League(this).competition().addr(), table, cup.year() as u16 as u32, n as u32, 1, label as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(cup.stages() + n * 4, stage);
    exe::crt_free(teams);
    exe::crt_free(table);
    let stage = read_usize(cup.stages() + n * 4);
    if stage == 0 {
        return show_error_at(cpp, 0x4ca);
    }
    exe::cup_set_up_round(stage, 0);
}

/// Comparer for `wc_african_cup_advance_stage`: it builds stages, so only the original runs; the stage stays (at the
/// last one) or moves on by one, except that building the groups leaves it at the last group.
pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_advance_stage(this: usize) {
    let before = Cup(this).current_stage() as i32;
    let last_group = african_group_layout(read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT)).map_or(0, |(groups, _)| groups as i32 - 1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_AFRICAN_CUP_ADVANCE_STAGE)(this);
    let after = Cup(this).current_stage() as i32;
    let ok = after == before || if before == -1 { after == 0 || after == last_group } else { after == before + 1 };
    SCOREBOARD.record(ok, || format!("wc_african_cup_advance_stage({:#x}): stage {} -> {}", this, before, after));
}

/// The clubs of a league stage's table, sorted (a set: the stage may order them).
pub(crate) unsafe fn league_stage_clubs(stage: League) -> Vec<usize> {
    let mut v: Vec<usize> = (0..stage.club_count().max(0) as usize).map(|i| read_usize(stage.table() + i * league_table_row::SIZE)).collect();
    v.sort_unstable();
    v
}

/// Comparer for `wc_african_cup_create_group_stages`: it builds stages, so only the original runs; then the plan: the
/// current stage is the last group, and each group's table holds the entrants g, g + groups... of the sorted list.
pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_create_group_stages(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_AFRICAN_CUP_CREATE_GROUP_STAGES);
    let layout = african_group_layout(read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT));
    let cup = Cup(this);
    let Some((groups, size)) = layout.filter(|_| cup.stages() != 0 && read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST) != 0) else {
        return o(this);
    };
    o(this);
    let list = read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST);
    let mut ok = cup.current_stage() == groups - 1;
    for g in 0..groups {
        let stage = League(read_usize(cup.stages() + g * 4));
        let mut plan: Vec<usize> = (0..size).map(|k| read_usize(list + (g + k * groups) * 4)).collect();
        plan.sort_unstable();
        ok &= !stage.is_null() && league_stage_clubs(stage) == plan;
    }
    SCOREBOARD.record(ok, || format!("wc_african_cup_create_group_stages({:#x}): stage {} of {} groups of {}", this, cup.current_stage() as i32, groups, size));
}

/// Comparer for `wc_african_cup_create_playoff_stage`: it builds a stage, so only the original runs; then the plan: the
/// stage at the play-off index with the group winners (read before) in its team table, in order.
pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_create_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_AFRICAN_CUP_CREATE_PLAYOFF_STAGE);
    let cup = Cup(this);
    let n = african_playoff_size(read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT));
    if cup.stages() == 0 || (0..n).any(|k| read_usize(cup.stages() + k * 4) == 0) {
        return o(this);
    }
    let plan: Vec<usize> = (0..n).map(|k| read_usize(League(read_usize(cup.stages() + k * 4)).table())).collect();
    o(this);
    let stage = read_usize(cup.stages() + n * 4);
    let t = if stage != 0 { Cup(stage).team_table() } else { 0 };
    let read: Vec<usize> = if t != 0 { (0..n).map(|k| read_usize(t + k * 6)).collect() } else { Vec::new() };
    SCOREBOARD.record(read == plan, || format!("wc_african_cup_create_playoff_stage({:#x}): {:x?} plan {:x?}", this, read, plan));
}

// ---------- rx11_4: the progress codes and the qualification status of a nation in the African World Cup qualifying (wc_african_cup.cpp) ----------

/// A progress code through a band table: code k (1-based) becomes `bands[k − 1]`; a code outside the table stays.
pub(crate) fn african_band(code: u8, bands: &[u8]) -> u8 {
    match code {
        1..=6 if (code as usize) <= bands.len() => bands[code as usize - 1],
        _ => code,
    }
}

/// The African progress codes (current, best, worst) of a nation at `stage` (signed byte) by the format byte
/// (`wc_cup_turnover_consts::AFRICAN_FORMAT`). The first round (stage −1; with format 2 its own bands): winner 1 at
/// best, the rest 25; the current and worst 21 (19 with format 2) for 1 and 25 for 2. A group stage (below 5 with
/// format 0, below 4 with 1 or 3 and others, below 5 with 2): the band tables below by position. The play-off with
/// format 2 (stage 5 and on) and with the other formats at stage 4: 1 stays 1 and 2 becomes 4 (format 2) or 3. Any
/// other stage, as they came.
pub(crate) fn african_progress_codes(format: u8, stage: i8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    let first_round = |one: u8| -> (u8, u8, u8) {
        let two = |v: u8, a: u8| match v {
            1 => a,
            2 => 0x19,
            _ => v,
        };
        (two(current, one), two(best, 1), two(worst, one))
    };
    let all = |c: &[u8], b: &[u8], w: &[u8]| (african_band(current, c), african_band(best, b), african_band(worst, w));
    let pair = |two: u8| {
        let f = |v: u8| match v {
            1 => 1,
            2 => two,
            _ => v,
        };
        (f(current), f(best), f(worst))
    };
    match format {
        0 => match stage {
            -1 => first_round(0x15),
            s if s < 5 => {
                let b = [1, 6, 0xb, 0x10, 0x15];
                all(&b, &b, &b) // rx11_4: five groups of five
            }
            _ => (current, best, worst),
        },
        1 => match stage {
            -1 => first_round(0x15),
            s if s < 4 => {
                let b = [1, 5, 9, 0xd, 0x11, 0x15];
                all(&b, &b, &b)
            }
            _ => (current, best, worst),
        },
        2 => match stage {
            -1 => first_round(0x13),
            s if s < 5 => all(&[1, 7, 0xd, 0x13], &[1, 7, 0xd, 0x13], &[4, 7, 0xd, 0x13]),
            _ => pair(4),
        },
        _ => match stage {
            -1 => first_round(0x15),
            s if s < 4 => {
                let others = [3, 5, 9, 0xd, 0x11, 0x15];
                all(&others, &[1, 5, 9, 0xd, 0x11, 0x15], &others)
            }
            4 => pair(3),
            _ => (current, best, worst),
        },
    }
}

/// rule: the African qualifying progress of a nation: in the first round the winner is 1 at best and 21 (19 with six
/// groups) now, the loser 25; in the groups the places map to bands of the final position (five groups: 1, 6, 11, 16,
/// 21; four groups: 1, 5, 9, 13, 17, 21, the winners 3 now with format 3; six groups: 1, 7, 13, 19, the winners 4 at
/// worst); in the play-off the winner 1 and the loser 3 (4 with six groups)
///
/// The codes of `african_progress_codes` into the nation's statistics entry (`exe::comp_stats_find_club_entry`, not
/// checked for null; `write_progress_codes`).
/// Original: `WcAfricanCup::set_club_progress` (0x9115a0, wc_african_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("rx11_4 african progress");
    let e = exe::comp_stats_find_club_entry(read_usize(this + league::COMP_STATS), club);
    let format = read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT);
    let (c, b, w) = african_progress_codes(format, stage as i8, current as u8, best as u8, worst as u8);
    write_progress_codes(e, c, b, w);
}

/// rule: a nation's status in the African, Asian, CONCACAF and Oceania qualifying is qualified (−2) while its progress
/// code (stat 0x12) is 1, and not (−3) otherwise
///
/// `qualifying_stat_byte`(club, 0x12) = 1: −2, otherwise −3 (a dword in eax). Shared by four vtables (+0x70).
/// Original: `WcAfricanCup::get_qualification_status` (0x911b80, wc_african_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_get_qualification_status(this: usize, club: usize) -> i32 {
    covered!("rx11_4 qualification status");
    if qualifying_stat_byte(this, club, 0x12) == 1 { -2 } else { -3 } // rx11_4: qualified at code 1
}

/// rule: in the African qualifying a nation's status changes down (−1) whenever its progress code (stat 0x12) is 1, up
/// (1) when it left 1 (stat 0x13, the previous code, was 1), otherwise 0
///
/// `qualifying_stat_byte` of 0x13 (before) and 0x12 (now): now 1, −1 (whatever before was; unlike the Oceania one, so
/// a nation still at 1 reads as falling again [inference: a slip]); otherwise 1 when before was 1, else 0. The byte in
/// al.
/// Original: `WcAfricanCup::get_qualification_status_change` (0x911ba0, wc_african_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_get_qualification_status_change(this: usize, club: usize) -> i8 {
    covered!("rx11_4 african status change");
    let before = qualifying_stat_byte(this, club, 0x13);
    let now = qualifying_stat_byte(this, club, 0x12);
    if now == 1 {
        -1
    } else {
        (before == 1) as i8 // rx11_4: left code 1
    }
}

pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_WC_AFRICAN_CUP_SET_CLUB_PROGRESS);
    compare_progress_codes("wc_african_cup_set_club_progress", this, club, stage, (current, best, worst), || o(this, club, stage, current, best, worst),
        || wc_african_cup_set_club_progress(this, club, stage, current, best, worst))
}

/// Comparers of the two status readers: they only read the statistics; side by side on the result.
pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_get_qualification_status(this: usize, club: usize) -> i32 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i32>(IDX_WC_AFRICAN_CUP_GET_QUALIFICATION_STATUS)(this, club);
    let b = wc_african_cup_get_qualification_status(this, club);
    SCOREBOARD.record(a == b, || format!("wc_african_cup_get_qualification_status({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}

pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_get_qualification_status_change(this: usize, club: usize) -> i8 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i8>(IDX_WC_AFRICAN_CUP_GET_QUALIFICATION_STATUS_CHANGE)(this, club);
    let b = wc_african_cup_get_qualification_status_change(this, club);
    SCOREBOARD.record(a == b, || format!("wc_african_cup_get_qualification_status_change({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}

// ---------- rx11_5: the CONCACAF World Cup qualifying semi-final groups and final group (wc_concacaf_cup.cpp) ----------

/// The next seeded CONCACAF nation: the first one left in the entrant list's slots 25 to 34
/// (`wc_cup_turnover_consts::CUP_ENTRANT_LIST`), taken out of it (the slot zeroed). None when they are all taken.
pub(crate) unsafe fn take_next_concacaf_seed(this: usize) -> Option<usize> {
    let list = read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST);
    let slot = (0x19..0x23).find(|&s| read_usize(list + s * 4) != 0)?;
    let nation = read_usize(list + slot * 4);
    write_usize(read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST) + slot * 4, 0);
    Some(nation)
}

/// The first three teams with status 1 (still in) of a cup's team table (6-byte rows), in order; fewer than three
/// leave the rest None (the original then reads stack leftovers; see DEFEITOS_DO_ORIGINAL).
pub(crate) unsafe fn first_three_still_in(cup: Cup) -> [Option<usize>; 3] {
    let mut v = [None; 3];
    let mut k = 0;
    for i in 0..cup.team_count().max(0) as usize {
        let row = cup.team_table() + i * 6;
        if read_u8(row + 5) == 1 && k < 3 {
            v[k] = Some(read_usize(row));
            k += 1;
        }
    }
    v
}

/// A CONCACAF group stage: the round table of stage `index` (virtual `comp_vtable::CREATE_SCHEDULE`(index, &rounds,
/// &label, 0)); operator new of 0xee and League_Stage::League_Stage `FN_LEAGUE_STAGE_LEAGUE_STAGE` (thiscall, ret
/// 0x58: the section, `size`, the teams, 2, the record, the round table, rounds, 3, 1, 0xa, the tie-breaks {1, 2, 3,
/// 0}, the places {`qualifiers`, 0, 0, 0}, the year, `index`, the label, `importance`, 4, 0, `code`, −1, 0, 2; the
/// original pushes the words and the index with garbage in the high bytes, which the constructor does not read: the
/// copy passes clean values) into `cup::STAGES`[index]. Returns the stage (or 0) and the round table. The Central
/// American groups' `concacaf_group_stage` (cx16_3) is this with places {1, 1, 0, 0}, importance 0xf and code 0x28,
/// without the store: one can replace the other when both are in the motor.
pub(crate) unsafe fn concacaf_qualifying_group_stage(this: usize, index: u8, teams: usize, size: u32, qualifiers: u8, importance: u32, code: u32) -> (usize, usize) {
    let cup = Cup(this);
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut label) = (0i16, 0i16);
    let table = create_schedule(this, index as u32, &mut rounds, &mut label, 0);
    let tiebreaks: [u8; 4] = [1, 2, 3, 0];
    let places: [u8; 4] = [qualifiers, 0, 0, 0];
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        construct(p, this, size, teams, 2, League(this).competition().addr(), table, rounds as u16 as u32, 3, 1, 0xa, tiebreaks.as_ptr() as usize,
            places.as_ptr() as usize, cup.year() as u16 as u32, index as u32, label as u16 as u32, importance, 4, 0, code, -1, 0, 2)
    } else {
        0
    };
    write_usize(cup.stages() + (index as i8 as isize as usize).wrapping_mul(4), stage);
    (read_usize(cup.stages() + (index as i8 as isize as usize).wrapping_mul(4)), table)
}

/// The four nations of each CONCACAF semi-final group g (0, 1, 2): the next seeded nation, the g-th Caribbean knockout
/// survivor (the cup's team table, status 1), the g-th play-off survivor (the play-off stage `cup::STAGES`[2], status
/// 1) and a Central American nation: the winner of group 0 (row 0 of `cup::STAGES`[0]) for group A, the entrant list's
/// slot at the cup's `team_count` for group B, the winner of group 1 for group C. The B slot is a defect of the
/// original [inference: the group 1 winner was meant, as in group C].
pub(crate) unsafe fn concacaf_semi_final_fourth(this: usize, g: usize) -> usize {
    let cup = Cup(this);
    match g {
        0 => read_usize(League(read_usize(cup.stages())).table()),
        1 => read_usize(read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST) + (cup.team_count() as isize as usize).wrapping_mul(4)), // rx11_5: the B slot
        _ => read_usize(League(read_usize(cup.stages() + 4)).table()),
    }
}

/// rule: the CONCACAF semi finals are three round robin groups of four, two qualifying from each: each takes the next
/// seeded nation, one Caribbean knockout survivor, one play-off survivor and a Central American nation; each group gets
/// its draw news
///
/// A team array (`exe::crt_malloc(0x10)`; null, the error box of line 0x65a). The Caribbean survivors
/// (`first_three_still_in` of the cup) and the play-off survivors (of `cup::STAGES`[2]). For the three groups at the
/// current stage g (`cup::CURRENT_STAGE`, its byte) and g + 1, g + 2: slot 0 `take_next_concacaf_seed` (none: the slot
/// keeps what was there), slots 1 and 2 the survivors, slot 3 `concacaf_semi_final_fourth`; the stage of 4 nations, 2
/// qualifying, importance 0xf, code 0x28 (`concacaf_qualifying_group_stage`); without it, the error box (lines 0x698, 0x6c4,
/// 0x6f3) and the routine ends (the array and the round table not freed). Groups A and B: the round table freed and
/// the group draw news (`exe::league_send_group_draw_news`). Group C: the array and the round table freed, then
/// `cup::CURRENT_STAGE` g + 2 and its draw news. The SEH frame covers only each stage's new.
/// Original: `WcConcacafCup::create_semi_final_groups` (0x917320, wc_concacaf_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_create_semi_final_groups(this: usize) {
    let cpp = wc_cup_turnover_consts::WC_CONCACAF_CUP_CPP;
    let cup = Cup(this);
    let first = cup.current_stage() as u8;
    let play_off = Cup(read_usize(cup.stages() + 8));
    let teams = exe::crt_malloc(0x10);
    if teams == 0 {
        return show_error_at(cpp, 0x65a);
    }
    covered!("rx11_5 concacaf semi-final groups");
    let caribbean = first_three_still_in(cup);
    let play_off = first_three_still_in(play_off);
    let lines = [0x698, 0x6c4, 0x6f3];
    for g in 0..3 {
        if let Some(seed) = take_next_concacaf_seed(this) {
            write_usize(teams, seed);
        }
        // simplification: with fewer than three survivors the original reads stack leftovers; the copy passes 0
        write_usize(teams + 4, caribbean[g].unwrap_or(0));
        write_usize(teams + 8, play_off[g].unwrap_or(0)); // rx11_5: one play-off survivor per group
        write_usize(teams + 0xc, concacaf_semi_final_fourth(this, g));
        let index = first.wrapping_add(g as u8);
        let (stage, table) = concacaf_qualifying_group_stage(this, index, teams, 4, 2, 0xf, 0x28);
        if g == 2 {
            exe::crt_free(teams);
            exe::crt_free(table);
        }
        if stage == 0 {
            return show_error_at(cpp, lines[g]);
        }
        if g == 2 {
            cup.set_current_stage(index as i8 as isize as usize);
        } else {
            exe::crt_free(table);
        }
        exe::league_send_group_draw_news(stage);
    }
}

/// rule: the CONCACAF final group is a round robin of six, the top two of each semi-final group, with three places
/// less the exempt nations qualifying for the World Cup
///
/// A team array (`exe::crt_malloc(0x18)`; null, the error box of line 0x723). The current stage s (`cup::CURRENT_STAGE`,
/// its byte); for the semi-final groups s − 3, s − 2 and s − 1, rows 0 and 1 (`exe::league_get_table_row`(group, row),
/// thiscall, ret 4; the club at the row's start). The stage of 6 nations at index s, with 3 − the exempt count
/// (+0xba, the byte `wc_concacaf_progress_codes` reads) qualifying, importance 0x14, code 0x24 (`concacaf_qualifying_group_stage`);
/// the array and the round table freed; without a stage, the error box of line 0x742. No draw news, and the current
/// stage is not touched (advance_stage set it).
/// Original: `WcConcacafCup::create_final_group` (0x917830, wc_concacaf_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_create_final_group(this: usize) {
    let cpp = wc_cup_turnover_consts::WC_CONCACAF_CUP_CPP;
    let cup = Cup(this);
    let index = cup.current_stage() as u8;
    let qualifiers = 3u8.wrapping_sub(read_u8(this + 0xba)); // three places less the exempt nations (+0xba)
    let teams = exe::crt_malloc(0x18);
    if teams == 0 {
        return show_error_at(cpp, 0x723);
    }
    covered!("rx11_5 concacaf final group");
    let base = index.wrapping_sub(3) as i8 as isize;
    for g in 0..3isize {
        let group = read_usize(cup.stages() + (base + g) as usize * 4);
        for row in 0..2 {
            write_usize(teams + (g as usize * 2 + row) * 4, read_usize(exe::league_get_table_row(group, row as i32))); // rx11_5: the top two of each group
        }
    }
    let (stage, table) = concacaf_qualifying_group_stage(this, index, teams, 6, qualifiers, 0x14, 0x24);
    exe::crt_free(teams);
    exe::crt_free(table);
    if stage == 0 {
        show_error_at(cpp, 0x742);
    }
}

/// Comparer for `wc_concacaf_cup_create_semi_final_groups`: it builds stages and takes the seeded nations, so only the
/// original runs; then the plan: the current stage two past the first group, and each group's table with its planned
/// four (read before: the seeds in order, the survivors, the fourth) as a set.
pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_create_semi_final_groups(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_CONCACAF_CUP_CREATE_SEMI_FINAL_GROUPS);
    let cup = Cup(this);
    let list = read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST);
    if cup.stages() == 0 || list == 0 || (0..3).any(|k| read_usize(cup.stages() + k * 4) == 0) {
        return o(this);
    }
    let first = cup.current_stage() as u8;
    let caribbean = first_three_still_in(cup);
    let play_off = first_three_still_in(Cup(read_usize(cup.stages() + 8)));
    if caribbean.iter().chain(play_off.iter()).any(|t| t.is_none()) {
        return o(this);
    }
    let seeds: Vec<usize> = (0x19..0x23).map(|s| read_usize(list + s * 4)).filter(|&n| n != 0).take(3).collect();
    if seeds.len() < 3 {
        return o(this);
    }
    let plans: Vec<Vec<usize>> = (0..3).map(|g| {
        let mut v = vec![seeds[g], caribbean[g].unwrap(), play_off[g].unwrap(), concacaf_semi_final_fourth(this, g)];
        v.sort_unstable();
        v
    }).collect();
    o(this);
    let mut ok = cup.current_stage() as i32 == first as i8 as i32 + 2;
    for (g, plan) in plans.iter().enumerate() {
        let stage = read_usize(cup.stages() + (first.wrapping_add(g as u8) as i8 as isize as usize) * 4);
        ok &= stage != 0 && league_stage_clubs(League(stage)) == *plan;
    }
    SCOREBOARD.record(ok, || format!("wc_concacaf_cup_create_semi_final_groups({:#x}): stage {} plans {:x?}", this, cup.current_stage() as i32, plans));
}

/// Comparer for `wc_concacaf_cup_create_final_group`: it builds a stage, so only the original runs; then the plan: the
/// stage at the current index with the top two of the three semi-final groups (read before) as a set.
pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_create_final_group(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_CONCACAF_CUP_CREATE_FINAL_GROUP);
    let cup = Cup(this);
    let index = cup.current_stage() as u8 as i8;
    if cup.stages() == 0 || index < 3 || (0..3).any(|g| read_usize(cup.stages() + (index as usize - 3 + g) * 4) == 0) {
        return o(this);
    }
    let mut plan: Vec<usize> = (0..3).flat_map(|g| {
        let t = League(read_usize(cup.stages() + (index as usize - 3 + g) * 4)).table();
        [read_usize(t), read_usize(t + league_table_row::SIZE)]
    }).collect();
    plan.sort_unstable();
    o(this);
    let stage = read_usize(cup.stages() + index as usize * 4);
    let ok = stage != 0 && league_stage_clubs(League(stage)) == plan;
    SCOREBOARD.record(ok, || format!("wc_concacaf_cup_create_final_group({:#x}): stage {} plan {:x?}", this, index, plan));
}

// ---------- rx12_17: the Oceania World Cup qualifying: the fate news text and the inter-continental play-off (wc_oceania_league.cpp) ----------

/// The fate news templates of the Oceania qualifying (langlib, `.rdata`).
pub(crate) mod wc_oceania_fate_texts {
    /// "{}<%s - Team Name(e.g.Australia)>{} have qualified for the semi finals of the {}<%s - Competition Name>{}."
    pub(crate) const QUALIFIED_FOR_SEMI_FINALS_OF: usize = 0xad4c48;
    /// "{}<%s - Team Name(e.g.Australia)>{} qualify for semi finals"
    pub(crate) const QUALIFY_FOR_SEMI_FINALS: usize = 0xad4c0c;
    /// "{}<%s - Team Name>{} qualify for playoff"
    pub(crate) const QUALIFY_FOR_PLAYOFF: usize = 0xad4be0;
    /// "{}<%s - Team Name>{} knocked out of World Cup qualifying"
    pub(crate) const KNOCKED_OUT_OF_QUALIFYING: usize = 0xad4ba4;
    /// "{}<%s - Team Name>{} qualify for World Cup"
    pub(crate) const QUALIFY_FOR_WORLD_CUP: usize = 0xad4b78;
    /// "{}<%s - Team Name>{} knocked out in World Cup playoff"
    pub(crate) const KNOCKED_OUT_IN_PLAYOFF: usize = 0xad4b40;
}

/// rule: the Oceania qualifying fate news: in the groups (stage below 1) a nation that goes through (fate 6) "have
/// qualified for the semi finals of" the competition (or, in short, "qualify for semi finals") and one that goes out
/// (−2) the shared group sentence; in the final (stage 1) the winner "qualify for playoff" and the loser "knocked out of
/// World Cup qualifying"; in the inter-continental play-off (code 0xa0) "qualify for World Cup" or "knocked out in World
/// Cup playoff"
///
/// `this.build_fate_news_text(club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, text)` (virtual +0x48; the
/// club in [`G_CLUB_TABLE`] by index, the fate byte, the stage byte (signed), the stage code (a word), four values
/// passed on, `full_sentence` and the Text).
///
/// Stage below 1: fate 6, with `full_sentence`, "have qualified for the semi finals of the" competition (its long name
/// `competition::NAME` with the short gender byte), the club's link and the competition's (its short name); without,
/// "qualify for semi finals" with the club's link. Fate 0xfe: the shared group sentence
/// (`exe::comp_util_write_group_qualify_news`(club_index, code, the competition record, −2, full_sentence, text)). Any
/// other fate, nothing.
///
/// Stage 1, or any other stage with code 0xa0: with `full_sentence`, the shared tie sentence
/// (`exe::comp_util_write_cup_tie_result_news`(club_index, a5, a8, the fate dword, a6, text)); otherwise fate 1 "qualify
/// for playoff" (stage 1) or "qualify for World Cup" (play-off), any other fate "knocked out of World Cup qualifying" or
/// "knocked out in World Cup playoff", with the club's link. Another stage with another code: nothing.
/// Original: `WcOceaniaLeague::build_fate_news_text` (0x91e990, wc_oceania_league.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, code: u32, a5: u32, a6: u32, a7: u32, a8: u32, full_sentence: u32, text: usize) {
    use wc_oceania_fate_texts as t;
    let _ = a7;
    let comp = League(this).competition().addr();
    let club = club_of_index(club_index);
    let stage = stage as u8 as i8;
    // simplification: on the paths without text the original leaves al with a byte of the arguments; the callers do not read it
    if stage < 1 {
        match fate as u8 {
            6 => {
                covered!("rx12_17 oceania group qualifier");
                if full_sentence != 0 {
                    qualifying_fate_line(text, t::QUALIFIED_FOR_SEMI_FINALS_OF, club, Some((comp + competition::SHORT_GENDER_NAME, comp + competition::NAME)));
                    link_fate_club(text, club);
                    link_fate_competition(text, comp); // rx12_17: the competition's link after the club's
                } else {
                    qualifying_fate_line(text, t::QUALIFY_FOR_SEMI_FINALS, club, None);
                    link_fate_club(text, club);
                }
            }
            0xfe => exe::comp_util_write_group_qualify_news(club_index, code, comp, -2, full_sentence, text),
            _ => {}
        }
        return;
    }
    let final_tie = stage == 1;
    if !final_tie && code as u16 != 0xa0 {
        return;
    }
    covered!("rx12_17 oceania tie fate");
    if full_sentence != 0 {
        return exe::comp_util_write_cup_tie_result_news(club_index, a5, a8, fate, a6, text);
    }
    let through = fate as u8 == 1;
    let template = match (final_tie, through) {
        (true, true) => t::QUALIFY_FOR_PLAYOFF,
        (true, false) => t::KNOCKED_OUT_OF_QUALIFYING,
        (false, true) => t::QUALIFY_FOR_WORLD_CUP, // rx12_17: the play-off winner
        (false, false) => t::KNOCKED_OUT_IN_PLAYOFF,
    };
    qualifying_fate_line(text, template, club, None);
    link_fate_club(text, club);
}

/// Comparer for `wc_oceania_league_build_fate_news_text`: side by side into two Texts (`compare_text_writer`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, code: u32, a5: u32, a6: u32, a7: u32, a8: u32, full_sentence: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(IDX_WC_OCEANIA_LEAGUE_BUILD_FATE_NEWS_TEXT);
    compare_text_writer(text, || o(this, club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, text),
        |t| wc_oceania_league_build_fate_news_text(this, club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, t),
        || format!("wc_oceania_league_build_fate_news_text(club {}, fate {}, stage {}, code {:#x}, full {})", club_index, fate as u8, stage as u8 as i8, code as u16, full_sentence));
}

/// The two nations of the Oceania play-off: the Oceania final's winner (the first entry of the final stage's team table,
/// `stages()`[1], with outcome 1 at +5 of its 6-byte entry) and the South American section's play-off nation (the
/// first row of its table with `league_table_row::STATUS` 1), in that order; either may be missing.
unsafe fn oceania_playoff_nations(this: League, south: League) -> Vec<usize> {
    let mut v = Vec::new();
    let final_stage = Cup(read_usize(this.stages() + 4));
    let teams = final_stage.team_table();
    if let Some(k) = (0..final_stage.team_count().max(0) as usize).find(|&k| read_u8(teams + 6 * k + 5) == 1) {
        v.push(read_usize(teams + 6 * k));
    }
    let rows = south.table();
    if let Some(k) = (0..south.club_count().max(0) as usize).find(|&k| read_u8(rows + k * league_table_row::SIZE + league_table_row::STATUS) == 1) {
        v.push(read_usize(rows + k * league_table_row::SIZE));
    }
    v
}

/// The inter-continental play-off of the Oceania qualifying: the Oceania final's winner against the South American
/// section's play-off nation (`oceania_playoff_nations`, the section the object of
/// `KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT` in [`G_COMP_OBJECT_TABLE`]), over two legs.
///
/// The list of 8 bytes (`exe::crt_malloc`; out of memory, the box of line 0x403); without both nations, the box of line
/// 0x41f and the list freed. Otherwise: the round table of the current stage (virtual `comp_vtable::CREATE_SCHEDULE`(the
/// current stage byte, &rounds, &extra, 0); the original pushes the byte with stack address bytes above it, which the
/// schedule does not read), the stage (`exe::crt_operator_new`(0xb2) and Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`(
/// the qualifying, 2, the list, the rounds, the competition record, the table, the year, the current stage byte, 4, the
/// extra, 0x14, 0, 0, 0, 0), ret 0x3c; words zero-extended) in `stages()`[current stage]; the list and the table freed
/// (`exe::crt_free`); without a stage, the box of line 0x431; with it, its first round set up (`exe::cup_set_up_round`(
/// stage, 0)).
/// Original: `WcOceaniaLeague::create_playoff_stage` (0x91f800, wc_oceania_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_create_playoff_stage(this: usize) {
    let this = League(this);
    let south = League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT).wrapping_mul(4))));
    let v = exe::crt_malloc(8);
    if v == 0 {
        return show_error_at(0xad4afc, 0x403);
    }
    let nations = oceania_playoff_nations(this, south);
    for (k, &n) in nations.iter().enumerate() {
        write_usize(v + 4 * k, n); // rx12_17: the play-off list
    }
    if nations.len() != 2 { // rx12_17: both nations are needed
        show_error_at(0xad4afc, 0x41f);
        exe::crt_free(v);
        return;
    }
    covered!("rx12_17 oceania play-off");
    let stage_byte = this.current_stage() as u8 as u32;
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = create_schedule(this.addr(), stage_byte, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 2, v, rounds as u16 as u32, this.competition().addr(), table, this.year() as u16 as u32, stage_byte, 4, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    let slot = this.stages() + (this.current_stage() as u32 as usize).wrapping_mul(4);
    write_usize(slot, stage);
    exe::crt_free(v);
    exe::crt_free(table);
    let stage = read_usize(this.stages() + (this.current_stage() as u32 as usize).wrapping_mul(4));
    if stage == 0 {
        return show_error_at(0xad4afc, 0x431);
    }
    exe::cup_set_up_round(stage, 0);
}

/// Comparer for `wc_oceania_league_create_playoff_stage`: the stage is allocated and its round set up, so only the
/// original runs; then the stage in `stages()`[current stage] must hold the planned two nations
/// (`oceania_playoff_nations`, read before) in its team table, in that order.
pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_create_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_OCEANIA_LEAGUE_CREATE_PLAYOFF_STAGE);
    let league = League(this);
    let south = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT).wrapping_mul(4)));
    if league.stages() == 0 || read_usize(league.stages() + 4) == 0 || south == 0 || League(south).table() == 0 {
        return o(this);
    }
    let plan = oceania_playoff_nations(league, League(south));
    o(this);
    if plan.len() != 2 {
        return;
    }
    let stage = read_usize(league.stages() + (league.current_stage() as u32 as usize).wrapping_mul(4));
    if stage == 0 {
        return;
    }
    let t = Cup(stage).team_table();
    let read = if t != 0 { vec![read_usize(t), read_usize(t + 6)] } else { vec![0, 0] };
    SCOREBOARD.record(read == plan, || format!("wc_oceania_league_create_playoff_stage({:#x}): {:x?} plan {:x?}", this, read, plan));
}

// ---------- cx17_2: the qualification status and the staff records of a nation in the South American World Cup qualifying (wc_south_american_league.cpp) ----------

/// The qualifying places of the South American section: six less the exempt nations
/// (`wc_south_american_league_fields::EXEMPT_COUNT`, a signed byte); a progress code (the table position) below it is
/// inside them.
pub(crate) unsafe fn south_american_qualifying_places(this: usize) -> i32 {
    6 - read_u8(this + wc_south_american_league_fields::EXEMPT_COUNT) as i8 as i32 // cx17_2: six places less the exempt
}

/// rule: in the South American qualifying a nation's status is qualified (−2) while its table position (stat 0x12) is
/// inside the qualifying places (six less the exempt nations), and not qualified (−3) otherwise
///
/// `qualifying_stat_byte`(club, 0x12) below `south_american_qualifying_places`: −2, otherwise −3 (a word).
/// Original: `WcSouthAmericanLeague::get_qualification_status` (0x921630, wc_south_american_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_get_qualification_status(this: usize, club: usize) -> i16 {
    covered!("cx17_2 south american qualification status");
    let now = qualifying_stat_byte(this, club, 0x12) as i32;
    (now < south_american_qualifying_places(this)) as i16 - 3
}

/// rule: in the South American qualifying a nation's status changes down (−1) when its table position (stat 0x12) leaves
/// the qualifying places (stat 0x13, the previous position, was inside), up (1) when it enters them, otherwise 0
///
/// `qualifying_stat_byte` of 0x13 (before) and 0x12 (now), signed, against `south_american_qualifying_places`: now
/// inside, (before inside) − 1; otherwise (before inside). The byte in al.
/// Original: `WcSouthAmericanLeague::get_qualification_status_change` (0x921670, wc_south_american_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_get_qualification_status_change(this: usize, club: usize) -> i8 {
    covered!("cx17_2 south american status change");
    let before = qualifying_stat_byte(this, club, 0x13) as i32;
    let now = qualifying_stat_byte(this, club, 0x12) as i32;
    let places = south_american_qualifying_places(this);
    if now < places {
        (before < places) as i8 - 1
    } else {
        (before < places) as i8
    }
}

/// NationalTeams::add_qualified_nation (thiscall, ret 8).
const FN_NATIONALTEAMS_ADD_QUALIFIED_NATION: usize = 0x7779b0;

/// rule: when the South American qualifying ends, a nation that qualifies (status 7) gets the World Cup qualification
/// record (weight 100), enters the World Cup and the national teams' list of qualified nations and its status becomes 4;
/// one that goes to the play-off against Oceania (1) gets the record of qualifying for the Oceania section (stage 0xa0,
/// weight 70); one that fails (−2) gets the failed-to-qualify record (weight 100) and ends its campaign
///
/// `this.update_staff_records_for_status(club, outcome, stage, out, game, a6)` (virtual +0x44; only `club`, the outcome
/// byte and `out` are read). The records go to Staff_Records ([`G_STAFF_RECORDS`]). Outcome 7: the World Cup record
/// (`KEY_CLUB_COMP_FIFA_WORLD_CUP` in [`G_CLUB_COMP_TABLE`], 0x6b bytes each) with
/// `european_cup_add_qualified_record`(club, record, 0, 0, 0x64); WorldCup::add_qualified_team on the World Cup object
/// (`exe::worldcup_add_qualified_team`); `*out` = 4; NationalTeams::add_qualified_nation(club, record) on
/// [`G_NATIONAL_TEAMS`] (`FN_NATIONALTEAMS_ADD_QUALIFIED_NATION`: 2 pushes; mov ecx -> thiscall, ret 8); returns −1.
/// Outcome 1: `european_cup_add_qualified_record`(club, the Oceania section's record, 0xa0, 0, 0x46); 0. Outcome 0xfe:
/// `european_cup_add_failed_to_qualify_record`(club, `league::COMPETITION`, 0, 0x64) and the national team's campaign
/// ended (`clear_national_team_competition`); 0. Any other outcome: 0.
/// Original: `WcSouthAmericanLeague::update_staff_records_for_status` (0x920a50, wc_south_american_league.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_update_staff_records_for_status(this: usize, club: usize, outcome: u32, _stage: u32, out: usize, _game: usize, _a6: u32) -> u32 {
    let comp_record = |key: usize| read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(key).wrapping_mul(0x6b));
    match outcome as u8 {
        7 => {
            covered!("cx17_2 south american to the world cup");
            european_cup_add_qualified_record(club, comp_record(KEY_CLUB_COMP_FIFA_WORLD_CUP), 0, 0, 0x64);
            let world_cup = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(4)));
            exe::worldcup_add_qualified_team(world_cup, club); // cx17_2: the qualified nation enters the World Cup
            write_u8(out, 4);
            let add: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_ADD_QUALIFIED_NATION);
            add(read_usize(G_NATIONAL_TEAMS), club, comp_record(KEY_CLUB_COMP_FIFA_WORLD_CUP));
            u32::MAX
        }
        1 => {
            covered!("cx17_2 south american to the oceania play-off");
            european_cup_add_qualified_record(club, comp_record(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION), 0xa0, 0, 0x46);
            0
        }
        0xfe => {
            covered!("cx17_2 south american out");
            european_cup_add_failed_to_qualify_record(club, League(this).competition().addr(), 0, 0x64);
            clear_national_team_competition(Club(club)); // cx17_2: the eliminated nation ends its campaign
            0
        }
        _ => 0,
    }
}

/// Comparers of the two status readers: they only read the statistics; side by side on the result (the word or byte).
pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_get_qualification_status(this: usize, club: usize) -> i16 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i16>(IDX_WC_SOUTH_AMERICAN_LEAGUE_GET_QUALIFICATION_STATUS)(this, club);
    let b = wc_south_american_league_get_qualification_status(this, club);
    SCOREBOARD.record(a == b, || format!("wc_south_american_league_get_qualification_status({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}

pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_get_qualification_status_change(this: usize, club: usize) -> i8 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i8>(IDX_WC_SOUTH_AMERICAN_LEAGUE_GET_QUALIFICATION_STATUS_CHANGE)(this, club);
    let b = wc_south_american_league_get_qualification_status_change(this, club);
    SCOREBOARD.record(a == b, || format!("wc_south_american_league_get_qualification_status_change({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}

/// Comparer for `wc_south_american_league_update_staff_records_for_status`: the records, the end of the national team's
/// campaign and the World Cup place do not run twice, so only the original runs; the plan: −1 for a qualified nation
/// (outcome 7) with `*out` = 4 and the World Cup's entrant count one more when it had room, 0 otherwise.
pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(IDX_WC_SOUTH_AMERICAN_LEAGUE_UPDATE_STAFF_RECORDS_FOR_STATUS);
    let qualified = outcome as u8 == 7 && out != 0;
    let world_cup = League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(4))));
    if qualified && world_cup.is_null() {
        return o(this, club, outcome, stage, out, game, a6);
    }
    let n = if qualified { world_cup.entrant_count() } else { 0 };
    let room = qualified && n < world_cup.entrant_capacity();
    let r = o(this, club, outcome, stage, out, game, a6);
    let same = if qualified { r == u32::MAX && read_u8(out) == 4 && world_cup.entrant_count() == if room { n + 1 } else { n } } else { r == 0 };
    SCOREBOARD.record(same, || format!("wc_south_american_league_update_staff_records_for_status({:#x}, {}) returned {:#x}, list {}", club, outcome as u8, r, n));
    r
}

// ---------- cx17_3: the news text of a nation's fate in the South American World Cup qualifying (wc_south_american_league.cpp), and the pot order of the World Cup draw (world_cup.cpp) ----------

/// rule: the news text of a nation's fate in the South American qualifying: a qualified nation (fate 7) "have qualified
/// from the" South American qualifying in the full sentence, "qualify for" the World Cup in the short one; a nation going
/// to the play-off (1) "qualify for Oceania playoff"; an eliminated one (−2) the shared group sentence; any other fate,
/// no text
///
/// The arguments as `world_cup_build_fate_news_text` (the club index `club_index` in [`G_CLUB_TABLE`], the fate byte
/// `fate`, six values not read here, `full_sentence` and the Text `text`). The lines are printed into the global buffer
/// 0xde1f64 (`FN_LANGLIB_SPRINTF`) and set as the text (`exe::text_set_string`); the club's gender and short name
/// (`club::SHORT_GENDER_NAME`, `club::SHORT_NAME`) go into every one.
/// - Fate 7 with `full_sentence`: 0xad4d6c with the club's gender twice, the competition record's short gender twice
///   (`competition::SHORT_GENDER_NAME`), the club's short name and the competition's long name (`competition::NAME`);
///   the links: the club (0x7d5) and the competition's long name (0x7d0).
/// - Fate 7 without it: 0xad44cc with the club's gender twice, the World Cup record's short gender twice
///   (`KEY_CLUB_COMP_FIFA_WORLD_CUP` in [`G_CLUB_COMP_TABLE`], 0x6b bytes each), the club's and the World Cup's short
///   names; the links: the World Cup (0x7d0) first, then the club (0x7d5).
/// - Fate 1: 0xad4d2c with the club's gender twice and its short name; the club's link (0x7d5).
/// - Fate 0xfe: the shared group sentence (`FN_COMP_UTIL_WRITE_GROUP_QUALIFY_NEWS`: club_index, the stage id, the
///   competition record, −2, full_sentence, text: 6 pushes; add esp, 0x18 -> cdecl). The stage id goes as the dword
///   the original builds: the word `league::STAGE_ID` over the high half of `text` (the callee reads the word).
/// Original: `WcSouthAmericanLeague::build_fate_news_text` (0x920b40, wc_south_american_league.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, full_sentence: u32, text: usize) {
    let _ = (stage, a3, a4, a5, a6, a7);
    let this = League(this);
    let club = club_at(club_index as i32 as isize);
    let club_name = club.addr() + club::SHORT_NAME;
    let club_gender = club.short_gender_name() as i8 as i32;
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    // simplification: the original leaves al with the fate byte or a callee's eax; the callers do not read it
    match fate as u8 {
        7 if full_sentence != 0 => {
            covered!("cx17_3 south american qualified, full sentence");
            let comp = this.competition().addr();
            let comp_gender = read_u8(comp + competition::SHORT_GENDER_NAME) as i8 as i32;
            sprintf(0xde1f64, 0xad4d6c, club_gender, club_gender, comp_gender, comp_gender, club_name, comp + competition::NAME);
            exe::text_set_string(text, 0xde1f64 as *const u8);
            exe::text_add_link(text, club_name as *const u8, 0x7d5, club);
            exe::text_add_link(text, (comp + competition::NAME) as *const u8, 0x7d0, comp);
        }
        7 => {
            covered!("cx17_3 south american qualified");
            let world_cup = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(0x6b));
            let cup_gender = read_u8(world_cup + competition::SHORT_GENDER_NAME) as i8 as i32;
            sprintf(0xde1f64, 0xad44cc, club_gender, club_gender, cup_gender, cup_gender, club_name, world_cup + competition::SHORT_NAME);
            exe::text_set_string(text, 0xde1f64 as *const u8);
            exe::text_add_link(text, (world_cup + competition::SHORT_NAME) as *const u8, 0x7d0, world_cup); // cx17_3: the World Cup's link first
            exe::text_add_link(text, club_name as *const u8, 0x7d5, club);
        }
        1 => {
            covered!("cx17_3 south american to the play-off");
            sprintf(0xde1f64, 0xad4d2c, club_gender, club_gender, club_name);
            exe::text_set_string(text, 0xde1f64 as *const u8);
            exe::text_add_link(text, club_name as *const u8, 0x7d5, club);
        }
        0xfe => {
            covered!("cx17_3 south american out");
            let sentence: unsafe extern "C" fn(u32, u32, usize, i32, u32, usize) = core::mem::transmute(FN_COMP_UTIL_WRITE_GROUP_QUALIFY_NEWS);
            let stage_id = (text as u32 & 0xffff_0000) | this.stage_id() as u16 as u32;
            sentence(club_index, stage_id, this.competition().addr(), -2, full_sentence, text);
        }
        _ => {}
    }
}

/// Comparer for `wc_south_american_league_build_fate_news_text`: side by side into two Texts (`compare_text_writer`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, full_sentence: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(IDX_WC_SOUTH_AMERICAN_LEAGUE_BUILD_FATE_NEWS_TEXT);
    compare_text_writer(text, || o(this, club_index, fate, stage, a3, a4, a5, a6, a7, full_sentence, text),
        |t| wc_south_american_league_build_fate_news_text(this, club_index, fate, stage, a3, a4, a5, a6, a7, full_sentence, t),
        || format!("wc_south_american_league_build_fate_news_text(club {}, fate {}, full {})", club_index, fate as u8, full_sentence));
}

/// rule: the draw entries are sorted by pot, lowest first
///
/// The qsort comparator of the draw and seeding entries: the signed pot byte at +4 of `a` less that of `b`.
/// Original: `compare_entries_by_pot` (0x92f660, world_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_entries_by_pot(a: usize, b: usize) -> i32 {
    covered!("cx17_3 pot order");
    read_i8(a + 4) as i32 - read_i8(b + 4) as i32 // cx17_3: the pot byte
}

/// Comparer for `compare_entries_by_pot`: it only reads; side by side on the result.
pub(crate) unsafe extern "C" fn compare_compare_entries_by_pot(a: usize, b: usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_ENTRIES_BY_POT)(a, b);
    let r = compare_entries_by_pot(a, b);
    SCOREBOARD.record(o == r, || format!("compare_entries_by_pot({:#x}, {:#x}) original={} rust={}", a, b, o, r));
    o
}

// ---------- cx17_5: the World Cup's staff records, progress codes and new season (world_cup.cpp) ----------

/// rule: the World Cup's career records: in the groups a nation through to the round of 16 gets the qualification (stage
/// 0x14, weight 30) and one going out the failed-to-qualify record (weight 30) and ends its campaign; in the knockout a
/// winner of a tie gets the qualification for the next round and a loser the knock-out record, both weighted 50 to 100
/// by round; the champion, the runner-up and the third get their records and end the campaign with every other nation
/// knocked out; the champion's status becomes 4
///
/// `this.update_staff_records_for_status(club, outcome, stage, out, game, final_)` (virtual +0x44). The records go to
/// Staff_Records ([`G_STAFF_RECORDS`]); the competition is `league::COMPETITION`; ending a campaign is
/// `clear_national_team_competition`. Stage below 7 (signed byte, the groups): outcome 6, `european_cup_add_qualified_record`
/// (club, competition, 0, 0x14, 0x1e); 0xfe, `european_cup_add_failed_to_qualify_record`(club, competition, 0x41c, 0x1e)
/// and the campaign ended; any other, nothing. Stage 7 (the knockout): outcome 0, the qualification with the fixture's
/// `round_prefix()`, the round code of the next round of the knockout stage (`.stages()`[7], its round table, round
/// `round_number()` + 1, `cup_round::ROUND_CODE`) and the weight `scale_by_round`(0x32, 0x64, `round_number()`, 5);
/// 0xff, `european_cup_add_knocked_out_record`(club, competition, `round_prefix()`, `round_name()`, the same weight) and
/// the campaign ended; 4, only the campaign ended; 1, 2 and 3, the winners, runners-up and third-place records
/// (`cup_record_final_result` with 0x86a800, 0x86b0b0 and 0x86b710: club, game, final_), for 1 `*out` = 4, then the
/// campaign ended; any other, nothing. Past stage 7, nothing. Returns 0.
/// Original: `WorldCup::update_staff_records_for_status` (0x92ccf0, world_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn world_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, final_: u32) -> u32 {
    let league = League(this);
    let comp = league.competition().addr();
    let stage = stage as u8 as i8;
    if stage < 7 {
        match outcome as u8 {
            6 => european_cup_add_qualified_record(club, comp, 0, 0x14, 0x1e),
            0xfe => {
                covered!("cx17_5 world cup group exit");
                european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x1e);
                clear_national_team_competition(Club(club)); // cx17_5: the group exit ends the campaign
            }
            _ => {}
        }
        return 0;
    }
    if stage != 7 {
        return 0;
    }
    covered!("cx17_5 world cup knockout records");
    let game = Fixture(game);
    // simplification: the original pushes the fixture's words in registers with garbage in the high 16 bits; the
    // records and scale_by_round only read the words
    match outcome as u8 {
        0 => {
            let weight = scale_by_round(0x32, 0x64, game.round_number(), 5);
            let rounds = Cup(read_usize(league.stages() + 7 * 4)).round_table();
            let next = read_u16(rounds.wrapping_add((game.round_number() as i32 as usize).wrapping_mul(cup_round::SIZE)) + cup_round::SIZE + cup_round::ROUND_CODE);
            european_cup_add_qualified_record(club, comp, game.round_prefix() as u16 as u32, next as u32, weight);
            return 0;
        }
        0xff => {
            let weight = scale_by_round(0x32, 0x64, game.round_number(), 5);
            european_cup_add_knocked_out_record(club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
        }
        4 => {}
        1 => {
            covered!("cx17_5 world cup winner");
            cup_record_final_result(FN_STAFF_RECORDS_ADD_CUP_WINNERS_RECORD, club, game, final_);
            write_u8(out, 4); // cx17_5: the champion's status
        }
        2 => cup_record_final_result(FN_STAFF_RECORDS_ADD_CUP_RUNNERS_UP_RECORD, club, game, final_),
        3 => cup_record_final_result(0x86b710, club, game, final_), // Staff_Records: the third place
        _ => return 0,
    }
    clear_national_team_competition(Club(club));
    0
}

/// The World Cup's progress codes (current, best, worst) at `stage` (signed byte): in the groups (below 7) a code 1 or 2
/// becomes 1 at best and 9 now and at worst, 3 becomes 17 and 4 becomes 25; in the knockout (from 7) they stay as they
/// came.
pub(crate) fn world_cup_progress_codes(stage: i8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    if stage >= 7 {
        return (current, best, worst);
    }
    let band = |v: u8, top: u8| match v {
        1 | 2 => top,
        3 => 0x11, // cx17_5: the third of a group
        4 => 0x19,
        _ => v,
    };
    (band(current, 9), band(best, 1), band(worst, 9))
}

/// rule: a nation's World Cup progress: in a group the first two are 1 at best and 9 now and at worst (the round of 16),
/// the third 17 and the fourth 25; in the knockout the codes stay as they came
///
/// The nation's statistics entry (`exe::comp_stats_find_club_entry`, not checked for null) gets the codes of
/// `world_cup_progress_codes` (`write_progress_codes`: best, current, worst, in the original's order).
/// Original: `WorldCup::set_club_progress` (0x92f9e0, world_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn world_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx17_5 world cup progress");
    let e = exe::comp_stats_find_club_entry(League(this).comp_stats(), club);
    let (c, b, w) = world_cup_progress_codes(stage as i8, current as u8, best as u8, worst as u8);
    write_progress_codes(e, c, b, w);
}

/// The six World Cup qualifying sections the World Cup restarts at its turnover, in the original's order.
const WORLD_CUP_QUALIFYING_SECTIONS: [usize; 6] = [KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION,
    KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION,
    KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION];

/// rule: at the World Cup's turnover the six qualifying sections start their new campaign, the finished World Cup is
/// cleared (table, dates, stages) with its statistics archived, the edition moves on four years and the team list and
/// the rules are built again
///
/// `league::SEASON_CLOSED` = 0; each section of `WORLD_CUP_QUALIFYING_SECTIONS` (the competition object of its key in
/// [`G_COMP_OBJECT_TABLE`], not checked for null) restarts (virtual `comp_vtable::START_NEW_SEASON`, thiscall, ret 0;
/// its eax not read). Then `exe::league_free_table(this, 0)`, `league::CLUB_COUNT` = 0, `league::ACTIVE` = 0, the round
/// dates freed and cleared; the statistics, when there, archived (`exe::comp_stats_archive_season_and_reset` with 1);
/// the stages 0..=`current_stage` destroyed (`destroy_comp_stages`; the vector stays). The year + 4 (word),
/// `league::STAGE_INDEX` = 0xff (the main competition), `current_stage` = −1, `league::ENTRANTS_ARE_QUALIFIERS` = 1; the
/// team list (`exe::worldcup_set_up_team_list`, its eax not read) and the rules (virtual `comp_vtable::SET_UP_RULES`):
/// 0, the world_cup.cpp box of line 0x332 and 0; otherwise `league::TABLE` = null and the rules' result.
/// Original: `WorldCup::start_new_season` (0x92d330, world_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_cup_start_new_season(this: usize) -> u32 {
    let league = League(this);
    league.set_season_closed(0);
    for key in WORLD_CUP_QUALIFYING_SECTIONS {
        let section = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(key).wrapping_mul(4)));
        let restart: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(section) + comp_vtable::START_NEW_SEASON));
        restart(section);
    }
    exe::league_free_table(league, 0);
    league.set_club_count(0);
    league.set_active(0);
    if league.round_dates() != 0 {
        exe::crt_free(league.round_dates());
        league.set_round_dates(0);
    }
    if league.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(league.comp_stats(), 1);
    }
    destroy_comp_stages(league);
    covered!("cx17_5 world cup turnover");
    league.set_year(league.year().wrapping_add(4)); // cx17_5: the next edition
    league.set_stage_index(stage_index::MAIN);
    league.set_current_stage(0xffff_ffff);
    league.set_entrants_are_qualifiers(1);
    exe::worldcup_set_up_team_list(league);
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_UP_RULES));
    let r = rules(this);
    if r == 0 {
        // simplification: after the box the original returns whatever the box left in eax; the caller reads 0 or not
        show_error_at(world_cup_knockout_consts::WORLD_CUP_CPP, 0x332);
        return 0;
    }
    league.set_table(0);
    r
}

/// Comparer for `world_cup_update_staff_records_for_status`: the records and the end of the campaigns do not run
/// twice, so only the original runs; the plan: 0 and, for the champion (stage 7, outcome 1), `*out` = 4.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, final_: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(IDX_WORLD_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS);
    let champion = stage as u8 == 7 && outcome as u8 == 1 && out != 0;
    let r = o(this, club, outcome, stage, out, game, final_);
    SCOREBOARD.record(r == 0 && (!champion || read_u8(out) == 4), || format!("world_cup_update_staff_records_for_status({:#x}, {}, stage {}) returned {}", club, outcome as u8, stage as u8 as i8, r));
    r
}

pub(crate) unsafe extern "thiscall" fn compare_world_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_WORLD_CUP_SET_CLUB_PROGRESS);
    compare_progress_codes("world_cup_set_club_progress", this, club, stage, (current, best, worst), || o(this, club, stage, current, best, worst),
        || world_cup_set_club_progress(this, club, stage, current, best, worst));
}

/// Comparer for `world_cup_start_new_season`: it restarts the six sections and frees the finished World Cup (it cannot
/// run twice), so only the original runs; the plan: the year four on, the season reopened, the main competition back
/// at no stage with the qualifiers' list, no clubs and inactive, and with the rules set up no table.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WORLD_CUP_START_NEW_SEASON);
    let league = League(this);
    let year = league.year();
    let r = o(this);
    let ok = league.year() == year.wrapping_add(4) && league.season_closed() == 0 && league.current_stage() == 0xffff_ffff
        && league.stage_index() == stage_index::MAIN && read_u8(this + league::ENTRANTS_ARE_QUALIFIERS) == 1 && league.club_count() == 0
        && league.active() == 0 && (r == 0 || league.table() == 0);
    SCOREBOARD.record(ok, || format!("world_cup_start_new_season({:#x}) returned {}: year {} -> {}, stage {:#x}", this, r, year, league.year(), league.current_stage()));
    r
}

// ---------- cx17_6: the World Cup qualification status and its change (world_cup.cpp) ----------

/// The World Cup status of a progress code (signed): 1 (the champion) −1, 2 (the runner-up) 150, below 5 (the semi-
/// finals) 130, below 9 (the quarter-finals) 120, below 17 (the round of 16) 20, otherwise (out in the groups) −4.
pub(crate) fn world_cup_status_of_code(code: i8) -> i16 {
    match code {
        1 => -1,
        2 => 0x96,
        c if c < 5 => 0x82,
        c if c < 9 => 0x78,
        c if c < 0x11 => 0x14,
        _ => -4, // cx17_6: out in the groups
    }
}

/// rule: a nation's World Cup status by its progress code: −1 for the champion, 150 for the runner-up, 130 for the
/// semi-finalists, 120 for the quarter-finalists, 20 for the round of 16 and −4 for those out in the groups
///
/// `qualifying_stat_byte`(club, 0x12) through `world_cup_status_of_code` (a word).
/// Original: `WorldCup::get_qualification_status` (0x92fae0, world_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn world_cup_get_qualification_status(this: usize, club: usize) -> i16 {
    covered!("cx17_6 world cup status");
    world_cup_status_of_code(qualifying_stat_byte(this, club, 0x12))
}

/// The change of a nation's World Cup status, by the band of its current code (rows: below 3, below 9, below 17, the
/// rest) and of its previous one (columns: below 5, below 9, below 17, the rest).
const WORLD_CUP_STATUS_CHANGE: [[i8; 4]; 4] = [
    [0, -1, -2, -3],
    [2, 0, -1, -2],
    [3, 2, 0, -1],
    [3, 3, 2, 0], // cx17_6: still out in the groups
];

/// rule: the change of a nation's World Cup status between its previous progress code (stat 0x13) and its current one
/// (stat 0x12): 0 within the same band, positive when it goes down the bands (worse), negative when it goes up, by the
/// number of bands crossed
///
/// The bands of the current code are below 3, below 9, below 17 and the rest; those of the previous one below 5, below
/// 9, below 17 and the rest (`WORLD_CUP_STATUS_CHANGE`; signed bytes). Defect of the original [inference]: the first
/// band of the current code ends at 3 where the previous one's ends at 5, so a nation whose code stays 3 or 4 (a semi-
/// finalist) shows a change of 2. The byte in al.
/// Original: `WorldCup::get_qualification_status_change` (0x92fb40, world_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn world_cup_get_qualification_status_change(this: usize, club: usize) -> i8 {
    covered!("cx17_6 world cup status change");
    let before = qualifying_stat_byte(this, club, 0x13);
    let now = qualifying_stat_byte(this, club, 0x12);
    let band = |code: i8, first: i8| match code {
        c if c < first => 0,
        c if c < 9 => 1,
        c if c < 0x11 => 2,
        _ => 3,
    };
    WORLD_CUP_STATUS_CHANGE[band(now, 3)][band(before, 5)]
}

/// Comparers: the two only read; side by side on the result.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_get_qualification_status(this: usize, club: usize) -> i16 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i16>(IDX_WORLD_CUP_GET_QUALIFICATION_STATUS)(this, club);
    let b = world_cup_get_qualification_status(this, club);
    SCOREBOARD.record(a == b, || format!("world_cup_get_qualification_status({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}

pub(crate) unsafe extern "thiscall" fn compare_world_cup_get_qualification_status_change(this: usize, club: usize) -> i8 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i8>(IDX_WORLD_CUP_GET_QUALIFICATION_STATUS_CHANGE)(this, club);
    let b = world_cup_get_qualification_status_change(this, club);
    SCOREBOARD.record(a == b, || format!("world_cup_get_qualification_status_change({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}
