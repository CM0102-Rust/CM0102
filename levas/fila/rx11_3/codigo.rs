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
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
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
