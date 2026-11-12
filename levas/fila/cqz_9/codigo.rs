// ---------- cqz_9: the Asian Cup honours, the Oceania Nations Cup predictions and the Oceania qualifying stages (asia_nations.cpp, oceania_nations.cpp, wc_oceania_league.cpp) ----------

/// rule: the season honours of the Asian Cup (winner, runner-up, third) are those of its knockout, the third stage
///
/// A tail call: the stage `.stages()`[2] (the knockout) does its own `comp_vtable::RECORD_SEASON_HONOURS` (thiscall,
/// ret 0).
/// Original: `AsiaNations::record_season_honours` (0x40f590, asia_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_nations_record_season_honours(this: usize) {
    let this = League(this);
    covered!("cqz_9 asian cup honours");
    let stage = read_usize(this.stages() + 8); // cqz9: the knockout is the third stage
    let h: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(stage) + comp_vtable::RECORD_SEASON_HONOURS));
    h(stage);
}

// simplification: the honours go to the history: they cannot run twice, and the routine writes nothing beyond the jump. Only the
// original.
pub(crate) unsafe extern "thiscall" fn compare_asia_nations_record_season_honours(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

/// The predicted final places of the six Oceania Nations Cup nations, in reputation order.
const OCEANIA_NATIONS_PREDICTED_PLACES: [u32; 6] = [1, 2, 3, 3, 5, 5];

/// rule: the Oceania Nations Cup predictions: the six drawn nations by reputation get the places 1, 2, 3, 3, 5 and 5
///
/// The virtual `comp_vtable::SET_PREDICTED_POSITIONS`. Without statistics (`league::COMP_STATS`), the error box of line
/// 0x436 and return; a copy of the drawn list +0xee with the count +0xf2 (signed word) × 4 bytes (`exe::crt_malloc`:
/// push; add esp -> cdecl; out of memory, the box of line 0x43e and return), sorted by reputation (qsort
/// `exe::crt_qsort`(copy, count, 4, 0x4b07d0): 4 pushes -> cdecl), and the first six get their predicted place in the
/// statistics (`exe::comp_stats_set_club_final_position`: push place; push team; mov ecx, statistics -> thiscall, ret
/// 8), whatever the count; the copy freed (`exe::crt_free`).
/// Original: `OceaniaNations::set_predicted_positions` (0x79a320, oceania_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn oceania_nations_set_predicted_positions(this: usize) {
    if read_usize(this + league::COMP_STATS) == 0 {
        return show_error_at(0xa6a0ec, 0x436);
    }
    let n = read_i16(this + 0xf2) as i32;
    let v = exe::crt_malloc((n << 2) as u32);
    if v == 0 {
        return show_error_at(0xa6a0ec, 0x43e);
    }
    core::ptr::copy_nonoverlapping(read_usize(this + 0xee) as *const u8, v as *mut u8, ((n << 2) as u32) as usize);
    exe::crt_qsort(v, n as u32, 4, 0x4b07d0);
    covered!("cqz_9 oceania nations predictions");
    for (k, &p) in OCEANIA_NATIONS_PREDICTED_PLACES.iter().enumerate() {
        exe::comp_stats_set_club_final_position(read_usize(this + league::COMP_STATS), read_usize(v + k * 4), p); // cqz9: the place by reputation
    }
    exe::crt_free(v);
}

// simplification: the statistics find or create each nation's entry (allocates): it cannot run twice. Only the original; the plan:
// the list sorted by reputation first, then the place (`CompStatsClub::final_position`) of each of the six equal to the
// band's.
pub(crate) unsafe extern "thiscall" fn compare_oceania_nations_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    if read_usize(this + league::COMP_STATS) == 0 || read_usize(this + 0xee) == 0 || read_i16(this + 0xf2) != 6 {
        return o(this);
    }
    let mut v: Vec<usize> = (0..6).map(|i| read_usize(read_usize(this + 0xee) + i * 4)).collect();
    exe::crt_qsort((v.as_mut_ptr()) as usize, 6, 4, 0x4b07d0);
    o(this);
    let wrong = (0..6).find(|&k| {
        let e = CompStatsClub(if v[k] != 0 { exe::comp_stats_find_club_entry(read_usize(this + league::COMP_STATS), v[k]) } else { 0 });
        e.is_null() || e.final_position() as u32 != OCEANIA_NATIONS_PREDICTED_PLACES[k]
    });
    SCOREBOARD.record(wrong.is_none(), || format!("oceania_nations_set_predicted_positions: the predicted place of slot {:?} differs", wrong));
}

/// rule: the Oceania World Cup qualifying moves on stage by stage: from the groups to the tie of the group winners (stage
/// 1), and from it to the inter-continental play-off (stage 2) only once the South American qualifying is complete
/// (the play-off opponent comes from it); past the last stage it is an error
///
/// On stage 1 (`league::CURRENT_STAGE` = 1): the South American section (the competition object of
/// `KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT` in [`G_COMP_OBJECT_TABLE`]) must be complete (its
/// `comp_vtable::CHECK_COMPLETE` with 1, 1: push 1; push 1; mov ecx -> thiscall, ret 8; al 0 returns). At or past the
/// last stage (`stage_count` − 1, signed): the error box of line 0x329. Otherwise the stage goes up one: to 1, the tie of
/// the group winners (WcOceaniaLeague::create_final_stage 0x91f640, thiscall, ret 0); to 2, the play-off
/// (WcOceaniaLeague::create_playoff_stage 0x91f800, thiscall, ret 0).
/// Original: `WcOceaniaLeague::advance_stage` (0x91f0d0, wc_oceania_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_advance_stage(this: usize) {
    let league = League(this);
    if read_u32(this + league::CURRENT_STAGE) == 1 {
        let south = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT).wrapping_mul(4)));
        let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(south) + comp_vtable::CHECK_COMPLETE));
        if complete(south, 1, 1) == 0 { // cqz9: the play-off waits for the South American qualifying
            return;
        }
    }
    let stage = read_u32(this + league::CURRENT_STAGE) as i32;
    if stage >= (league.stage_count() as i32).wrapping_sub(1) {
        show_error_at(0xad4afc, 0x329);
        return;
    }
    let stage = stage + 1;
    write_u32(this + league::CURRENT_STAGE, stage as u32);
    covered!("cqz_9 oceania qualifying next stage");
    match stage {
        1 => {
            let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x91f640usize);
            f(this);
        }
        2 => {
            let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x91f800usize);
            f(this);
        }
        _ => {}
    }
}

// simplification: the next stage is allocated and drawn: it cannot run twice. Only the original; the plan: the stage went up one
// (when not at the last one and, from stage 1, with the South American qualifying complete), or stayed.
pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let before = read_u32(this + league::CURRENT_STAGE) as i32;
    o(this);
    let after = read_u32(this + league::CURRENT_STAGE) as i32;
    SCOREBOARD.record(after == before || after == before + 1, || format!("wc_oceania_league_advance_stage({:#x}): stage {} -> {}", this, before, after));
}
