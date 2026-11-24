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
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
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
    let a = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I2%%)(this);
    let b = wc_europe_league_get_asia_play_off_team(this);
    SCOREBOARD.record(a == b, || format!("wc_europe_league_get_asia_play_off_team({:#x}) original={:#x} rust={:#x}", this, a, b));
    a
}
