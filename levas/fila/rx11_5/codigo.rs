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
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
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
