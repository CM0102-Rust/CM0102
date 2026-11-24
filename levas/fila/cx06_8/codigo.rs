// ---------- cx06_8: the Confederations Cup's stage advance, staff records and club progress (fifa_confed.cpp) ----------

/// Moves the Confederations Cup to its next stage: past the last one (`.current_stage()` ≥ `.stage_count()` − 1, signed)
/// the fifa_confed.cpp box at line 0x291 and nothing else; otherwise `.current_stage()` + 1 and, entering stage 1, the
/// semi-finals (FifaConfed::create_knockout_stage 0x58c2d0).
/// Original: `FifaConfed::advance_stage` (0x58c070, fifa_confed.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fifa_confed_advance_stage(this: usize) {
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x9c7474, 0x291); // fifa_confed.cpp
    }
    let stage = stage + 1;
    l.set_current_stage(stage as usize);
    if stage == 1 {
        covered!("cx06_8 confed knockout stage");
        let create_knockout_stage: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x58c2d0usize); // FifaConfed::create_knockout_stage
        create_knockout_stage(this);
    }
}

/// Writes the staff history records of a nation's fate in the Confederations Cup (virtual +0x44); returns 0.
///
/// In the groups (`stage`, signed byte, below 1): `outcome` 6 (through), qualified for the round (Staff_Records, the
/// record `european_cup_add_qualified_record`: stage 0, round 0x96, weight 0x19); 0xfe (out), failed to qualify (round
/// 0x41c, weight 0x19). In the knockout (stage 1): 0 (through), qualified with the fixture's `round_prefix`, the round
/// code of its round (`round_number`) in the knockout's round table and the weight `scale_by_round`(0x32, 0x64, round,
/// 2); 0xff (out), knocked out with the fixture's `round_prefix` and `round_name` and `scale_by_round`(0x32, 0x64, round,
/// 5); 1, 2 and 3 the winners, runners-up and third-placed records (`african_nations_record_final_result` with 0x86a800,
/// 0x86b0b0, 0x86b710), and for the winners `left` gets 4 (not checked for null); 4, nothing.
/// Original: `FifaConfed::update_staff_records_for_status` (0x58be70, fifa_confed.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn fifa_confed_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, left: usize, game: usize, a6: u32) -> u32 {
    let l = League(this);
    let (outcome, stage) = (outcome as u8, stage as u8 as i8);
    let comp = l.competition().addr();
    if stage < 1 {
        covered!("cx06_8 confed group records");
        match outcome {
            6 => european_cup_add_qualified_record(club, comp, 0, 0x96, 0x19),
            0xfe => european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x19),
            _ => {}
        }
        return 0;
    }
    if stage != 1 {
        return 0;
    }
    let game = Fixture(game);
    match outcome {
        0 => {
            let round = game.round_number();
            let weight = scale_by_round(0x32, 0x64, round, 2);
            let knockout = Cup(read_usize(l.stages() + 4));
            let code = read_i16(knockout.round_table().wrapping_add((round as isize * cup_round::SIZE as isize) as usize) + cup_round::ROUND_CODE + 0x68) as u16 as u32;
            european_cup_add_qualified_record(club, comp, game.round_prefix() as u16 as u32, code, weight);
        }
        0xff => {
            covered!("cx06_8 confed knocked out");
            let weight = scale_by_round(0x32, 0x64, game.round_number(), 5);
            european_cup_add_knocked_out_record(club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
        }
        1 => {
            covered!("cx06_8 confed winners");
            african_nations_record_final_result(0x86a800, club, game.addr(), a6); // Staff_Records::add_cup_winners_record
            write_u8(left, 4);
        }
        2 => african_nations_record_final_result(0x86b0b0, club, game.addr(), a6), // Staff_Records::add_cup_runners_up_record
        3 => african_nations_record_final_result(0x86b710, club, game.addr(), a6), // Staff_Records::add_cup_third_place_record
        _ => {}
    }
    0
}

/// Stores a nation's progress in the Confederations Cup (`group_championship_progress`): in the groups (stage below 1)
/// current 1→1, 2→3, 3→5, 4→7; best 1→1, 2→3, 3→5, 4→7; worst 1→2, 2→3, 3→5, 4→7.
/// Original: `FifaConfed::set_club_progress` (0x58cba0, fifa_confed.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn fifa_confed_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx06_8 confed club progress");
    group_championship_progress(this, club, stage, current, best, worst, 1, [[1, 3, 5, 7], [1, 3, 5, 7], [2, 3, 5, 7]]);
}

/// Comparer for `fifa_confed_advance_stage`: the semi-finals allocate and draw (they cannot run twice), so only the
/// original runs; the plan: `.current_stage()` one up (unchanged past the last stage).
pub(crate) unsafe extern "thiscall" fn compare_fifa_confed_advance_stage(this: usize) {
    let l = League(this);
    let before = l.current_stage() as i32;
    let last = (l.stage_count() as i32).wrapping_sub(1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let want = if before >= last { before } else { before + 1 };
    SCOREBOARD.record(l.current_stage() as i32 == want, || format!("fifa_confed_advance_stage({:#x}): stage {} -> {}", this, before, l.current_stage() as i32));
}

/// Comparer for `fifa_confed_update_staff_records_for_status`: the records go to the managers' histories and end the
/// call-ups (they cannot run twice), so only the original runs; the plan: it returns 0 and the winners' `left` is 4.
pub(crate) unsafe extern "thiscall" fn compare_fifa_confed_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, left: usize, game: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(%%I1%%)(this, club, outcome, stage, left, game, a6);
    let winners = stage as u8 == 1 && outcome as u8 == 1;
    SCOREBOARD.record(r == 0 && (!winners || read_u8(left) == 4), || format!("fifa_confed_update_staff_records_for_status(outcome {}, stage {}) returned {}", outcome as u8, stage as u8 as i8, r));
    r
}

pub(crate) unsafe extern "thiscall" fn compare_fifa_confed_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    compare_group_championship_progress("fifa_confed_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || fifa_confed_set_club_progress(this, club, stage, current, best, worst));
}
