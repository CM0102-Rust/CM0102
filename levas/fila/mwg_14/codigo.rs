// ---------- batch %%LEVA%%: the World Cup's knockout stage: moving on to it, building it and its season honours (world_cup.cpp) ----------

/// Constants of the World Cup's knockout stage.
pub(crate) mod world_cup_knockout_consts {
    /// "E:\dev\CM3\cm3 00-01\cm3\code\comp\worldcup\world_cup.cpp" (the error boxes).
    pub(crate) const WORLD_CUP_CPP: usize = 0xad5888;
    /// The knockout stage's index in the stage vector (the groups are the cup itself and stages 0..=6).
    pub(crate) const KNOCKOUT_STAGE: usize = 7;
    /// NationalTeams::clear_competition_for_comp_clubs (thiscall, ret 4): clears the current competition of every
    /// national team in a competition's club list.
    /// WorldCup::create_knockout_stage (thiscall, ret 0), called directly by advance_stage (hooked: the copy below).
    pub(crate) const FN_WORLD_CUP_CREATE_KNOCKOUT_STAGE: usize = 0x92d770;
    pub(crate) const FN_NATIONALTEAMS_CLEAR_COMPETITION_FOR_COMP_CLUBS: usize = 0x775420;
    /// The 16 places of the round of 16, in the order of the stage's list (the stage pairs them): (group, place), group 0 the cup's own table (group A), 1..=7 the stages 0..=6
    /// (groups B..H), place 0 the winner, 1 the runner-up.
    pub(crate) const ROUND_OF_16: [(usize, i32); 16] = [
        (0, 0), (1, 1), (3, 0), (2, 1), (4, 0), (5, 1), (7, 0), (6, 1), // mwg14: the group winners and runners-up paired
        (1, 0), (0, 1), (2, 0), (3, 1), (5, 0), (4, 1), (6, 0), (7, 1),
    ];
}

/// The club in `place` of World Cup group `group` (0 the cup itself, k the stage k − 1): League::get_table_row
/// (`exe::league_get_table_row`, thiscall, ret 4) and the row's club.
pub(crate) unsafe fn world_cup_group_club(this: League, group: usize, place: i32) -> usize {
    let league = if group == 0 { this.addr() } else { read_usize(this.stages() + (group - 1) * 4) };
    read_usize(exe::league_get_table_row(league, place))
}

/// Moves the World Cup on to its next stage; entering stage 7 builds the knockout stage. Past the last stage it is an
/// error (the box).
///
/// At the last stage (`.current_stage()` ≥ `.stage_count()` − 1, signed) the box of line 0x345 and nothing else;
/// otherwise `league::CURRENT_STAGE` goes up by one and, when it becomes 7, `world_cup_create_knockout_stage` (direct
/// call, thiscall, ret 0).
/// Original: `WorldCup::advance_stage` (0x92d4b0, world_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_cup_advance_stage(this: usize) {
    use world_cup_knockout_consts as wk;
    let league = League(this);
    let stage = league.current_stage() as i32;
    if stage >= (league.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(wk::WORLD_CUP_CPP, 0x345);
    }
    league.set_current_stage((stage + 1) as usize);
    if stage + 1 == wk::KNOCKOUT_STAGE as i32 {
        covered!("mwg_14 world cup knockout");
        let create: unsafe extern "thiscall" fn(usize) = core::mem::transmute(wk::FN_WORLD_CUP_CREATE_KNOCKOUT_STAGE);
        create(this);
    }
}

/// Comparer for `world_cup_advance_stage`: only the original (it may build the knockout stage); the stage must stay at
/// the last one or go up by one.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    compare_state_league_advance_stage("world_cup_advance_stage", League(this), || o(this))
}

/// Builds the World Cup's round of 16: the eight group winners and eight runners-up in a sixteen-team cup stage (stage
/// 7), with the bookings wiped and the first round set up at once.
///
/// The bookings of the competition's ruling body cleared (the ruling body of `league::DISCIPLINE_RULE` (signed byte)
/// in [`G_DISCIPLINE`]'s table: RulingBody::reset_bookings_for_club(0), `exe::rulingbody_reset_bookings_for_club`,
/// thiscall, ret 4). The list of the 16 (`exe::crt_malloc(0x40)`, cdecl; out of memory, the box of line 0x3a0 and
/// return) in the order of `ROUND_OF_16` (A1, B2, D1, C2, E1, F2, H1, G2, then B1, A2, C1, D2, F1, E2, G1, H2). The round table through `comp_vtable::CREATE_SCHEDULE` (push 0; push &id; push &rounds; push 7;
/// mov ecx, this -> thiscall, ret 0x10); the stage (`exe::crt_operator_new`(0xb2); Cup_Stage::Cup_Stage
/// `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret 0x3c: the cup, 16, the list, the rounds, `.competition()`, the table, the year
/// `.year()`, 7, 4, the id, 0x14, 1, 0, 0 and no seed; the original pushes the rounds, the id and the year with garbage in
/// the high 16 bits, which the constructor does not read; the stage's new under an SEH frame, nothing here throws) into
/// `.stages()`[7]; the list and the table freed (`exe::crt_free`, in that order). No stage: the box of line 0x3c4 and
/// return. With it: its first round (`exe::cup_set_up_round` Cup::set_up_round(0), mov ecx, stage -> thiscall, ret 4)
/// and `league::CURRENT_STAGE` = 7.
/// Original: `WorldCup::create_knockout_stage` (0x92d770, world_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_cup_create_knockout_stage(this: usize) {
    use world_cup_knockout_consts as wk;
    let league = League(this);
    let body = read_usize(read_usize(read_usize(G_DISCIPLINE)).wrapping_add((read_i8(this + league::DISCIPLINE_RULE) as isize * 4) as usize));
    exe::rulingbody_reset_bookings_for_club(body, 0usize);
    let list = exe::crt_malloc(0x40);
    if list == 0 {
        return show_error_at(wk::WORLD_CUP_CPP, 0x3a0);
    }
    covered!("mwg_14 world cup round of 16");
    for (k, &(group, place)) in wk::ROUND_OF_16.iter().enumerate() {
        write_usize(list + k * 4, world_cup_group_club(league, group, place));
    }
    let round_table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut id) = (0i16, 0i16);
    let table = round_table(this, wk::KNOCKOUT_STAGE as u32, &mut rounds, &mut id, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, 0x10, list, rounds as u16 as u32, league.competition().addr(), table, league.year() as u16 as u32, 7, 4, id as u16 as u32, 0x14, 1, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_usize(league.stages() + wk::KNOCKOUT_STAGE * 4, stage);
    exe::crt_free(list);
    exe::crt_free(table);
    let stage = read_usize(league.stages() + wk::KNOCKOUT_STAGE * 4);
    if stage == 0 {
        return show_error_at(wk::WORLD_CUP_CPP, 0x3c4);
    }
    exe::cup_set_up_round(stage, 0);
    league.set_current_stage(wk::KNOCKOUT_STAGE);
}

// simplification: allocates the stage and builds its first round: it does not run twice. Original only; the plan: the
// stage at [+0xc][7] with 16 teams, the cup's +4 and the 16 group places (read before) in its list (+0xa7, 6 bytes; as a
// set: the round may reorder), and the current stage 7.
/// Comparer for `world_cup_create_knockout_stage`.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_create_knockout_stage(this: usize) {
    use world_cup_knockout_consts as wk;
    let league = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    if league.table() == 0 || league.stages() == 0 || (0..7).any(|k| read_usize(league.stages() + k * 4) == 0) {
        return o(this);
    }
    let mut plan: Vec<usize> = wk::ROUND_OF_16.iter().map(|&(g, p)| world_cup_group_club(league, g, p)).collect();
    o(this);
    let stage = Cup(read_usize(league.stages() + wk::KNOCKOUT_STAGE * 4));
    if stage.is_null() {
        SCOREBOARD.record(false, || format!("world_cup_create_knockout_stage({:#x}) without stage", this));
        return;
    }
    let mut found: Vec<usize> = (0..16).map(|k| read_usize(stage.team_table() + 6 * k)).collect();
    let (pl, ac) = (plan.clone(), found.clone());
    plan.sort_unstable();
    found.sort_unstable();
    let same = plan == found && stage.team_count() == 16 && read_usize(stage.addr() + 4) == league.competition().addr()
        && league.current_stage() == wk::KNOCKOUT_STAGE;
    SCOREBOARD.record(same, || format!("world_cup_create_knockout_stage({:#x}) plan={:x?} list={:x?} stage {}", this, pl, ac, league.current_stage() as i32));
}

/// The World Cup's season honours: those of the knockout stage, then the national teams leave the competition.
///
/// The knockout stage (`.stages()`[7]) records its honours (virtual `comp_vtable::RECORD_SEASON_HONOURS`, thiscall, ret
/// 0; the original does not check the stage); then NationalTeams::clear_competition_for_comp_clubs(competition record)
/// on [`G_NATIONAL_TEAMS`] (`FN_NATIONALTEAMS_CLEAR_COMPETITION_FOR_COMP_CLUBS`, thiscall, ret 4), whose eax it returns.
/// Original: `WorldCup::record_season_honours` (0x92d310, world_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_cup_record_season_honours(this: usize) -> u32 {
    use world_cup_knockout_consts as wk;
    let league = League(this);
    let stage = read_usize(league.stages() + wk::KNOCKOUT_STAGE * 4);
    let honours: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(stage) + comp_vtable::RECORD_SEASON_HONOURS));
    honours(stage);
    covered!("mwg_14 world cup honours");
    let clear: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(wk::FN_NATIONALTEAMS_CLEAR_COMPETITION_FOR_COMP_CLUBS);
    clear(read_usize(G_NATIONAL_TEAMS), league.competition().addr()) // mwg14: the nations leave the World Cup
}

// simplification: the honours go to the history: they cannot run twice. Original only; the plan: the knockout stage
// existed (the original reads it without checking).
/// Comparer for `world_cup_record_season_honours`.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_record_season_honours(this: usize) -> u32 {
    let league = League(this);
    let had = league.stages() != 0 && read_usize(league.stages() + world_cup_knockout_consts::KNOCKOUT_STAGE * 4) != 0;
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this);
    SCOREBOARD.record(had, || format!("world_cup_record_season_honours({:#x}) without the knockout stage", this));
    r
}
