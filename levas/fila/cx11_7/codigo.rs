// ---------- cx11_7: the Serie C2/A stage advance and end-of-league stages (ita_ser_c2a.cpp) ----------

/// Builds the two end-of-league stages of a Serie C2 group: the promotion play-off and the relegation play-out, each a
/// Cup_Stage with its first-round draw. Serie C1's `create_end_of_league_stages` without the Super Cup step (the C2
/// champion goes nowhere else), with the lists always freed list first. The body of
/// `ita_ser_c2a_create_play_off_stages` and `ita_ser_c2b_create_playoff_stages`, identical but for the source file and
/// the box lines.
///
/// 1) The 4-club list (`exe::crt_malloc` of 0x10; out of memory the box for line `rows[0]` and return) with the clubs of
/// the play-off rows (`PROMOTION_PLAYOFF_ROWS`: 5th, 2nd, 4th, 3rd, for 2nd v 5th and 3rd v 4th).
///
/// 2) The stage 0 schedule (virtual `comp_vtable::CREATE_SCHEDULE`(0, &rounds, &extra, 0); returns the template;
/// `rounds` and `extra` are words).
///
/// 3) The stage (`exe::crt_operator_new` of 0xb2 and Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`, 15 arguments,
/// thiscall, ret 0x3c: the league, the clubs (`league::PROMOTION_PLAYOFF_COUNT`, signed byte), the list, the rounds,
/// the competition record, the template, the year, 0, 1, the `extra`, 0x14, 0, 0, 0 and the table positions of the 4
/// (local 4-byte record: 4, 1, 3, 2)) goes to stage slot 0; no stage: the box for line `rows[1]` and return; otherwise
/// the first-round draw (`exe::cup_set_up_round`(stage, 0)) and the template freed.
///
/// 4) The same with stage 1 and the play-out clubs (`PLAYOUT_ROWS`: 17th, 14th, 16th, 15th; positions 16, 13, 15, 14
/// and 1, 1 in the two bytes) into stage slot 1; the list and the template freed; no stage: the box for line
/// `rows[2]`; otherwise the first-round draw and `league::CURRENT_STAGE` = 1.
///
/// The high bits of the words the original pushes as dwords (the clubs, the rounds, the year and the `extra`) are
/// garbage from the stack or other registers; the constructor reads only the word, and the copy passes zero there.
pub(crate) unsafe fn create_serie_c2_playoff_stages(this: League, file: usize, rows: [u32; 3]) {
    let new_cup_stage: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut u32, *mut u32, u32) -> usize =
        core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let list = exe::crt_malloc(0x10);
    if list == 0 {
        show_error_at(file, rows[0]);
        return;
    }
    for (k, &i) in PROMOTION_PLAYOFF_ROWS.iter().enumerate() {
        write_usize(list + k * 4, ita_ser_c1a_league_row_club(this, i));
    }
    let clubs = (this.promotion_playoff_count() as i8) as i16 as u16 as u32;
    let year = (this.year() as u16) as u32;
    let record = this.competition().addr();
    let (mut rounds, mut extra) = (0u32, 0u32);
    let template = schedule(this.addr(), 0, &mut rounds, &mut extra, 0);
    let positions: [u8; 4] = [4, 1, 3, 2]; // cx11_7: the play-off positions
    let mut stage = exe::crt_operator_new(0xb2);
    if stage != 0 {
        stage = new_cup_stage(stage, this.addr(), clubs, list, rounds & 0xffff, record, template, year, 0, 1, extra & 0xffff, 0x14, 0, 0, 0, positions.as_ptr());
    }
    write_usize(this.stages(), stage);
    if read_usize(this.stages()) == 0 {
        show_error_at(file, rows[1]);
        return;
    }
    exe::cup_set_up_round(read_usize(this.stages()), 0);
    exe::crt_free(template);
    let (mut rounds, mut extra) = (0u32, 0u32);
    let template = schedule(this.addr(), 1, &mut rounds, &mut extra, 0);
    for (k, &i) in PLAYOUT_ROWS.iter().enumerate() {
        write_usize(list + k * 4, ita_ser_c1a_league_row_club(this, i));
    }
    let positions: [u8; 4] = [0x10, 0xd, 0xf, 0xe]; // cx11_7: the play-out positions
    let mut stage = exe::crt_operator_new(0xb2);
    if stage != 0 {
        stage = new_cup_stage(stage, this.addr(), clubs, list, rounds & 0xffff, record, template, year, 1, 1, extra & 0xffff, 0x14, 0, 0, 0, positions.as_ptr());
    }
    write_usize(this.stages() + 4, stage);
    exe::crt_free(list);
    exe::crt_free(template);
    if read_usize(this.stages() + 4) == 0 {
        show_error_at(file, rows[2]);
        return;
    }
    covered!("cx11_7 serie c2 play-off and play-out");
    exe::cup_set_up_round(read_usize(this.stages() + 4), 0);
    this.set_current_stage(1);
}

/// The comparer of the Serie C2 end-of-league stages: they allocate and draw, so only the original runs; afterwards
/// both stage slots must hold a stage and `league::CURRENT_STAGE` must be 1.
pub(crate) unsafe fn compare_serie_c2_playoff_stages(name: &str, this: League, original: impl FnOnce()) {
    original();
    let stages = this.stages();
    let ok = stages != 0 && read_usize(stages) != 0 && read_usize(stages + 4) != 0 && this.current_stage() == 1;
    SCOREBOARD.record(ok, || format!("{}({:#x}): stages {:#x}, +0x30 = {}", name, this.addr(), stages, this.current_stage() as i32));
}

/// The Serie C2/A stage advance (`ita_ser_c1a_advance_stage_cql5_advance_stage`, Serie C1's body, checked instruction
/// by instruction): at the last stage the ita_ser_c2a.cpp box of line 0x251; otherwise the next stage and, on reaching
/// stage 0 (the league is over), the play-off and the play-out (`ita_ser_c2a_create_play_off_stages`, 0x653e90).
/// Original: `ItaSerC2a::advance_stage` (0x653e00, ita_ser_c2a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2a_advance_stage(this: usize) {
    ita_ser_c1a_advance_stage_cql5_advance_stage(League(this), 0x9ea654, 0x251, 0x653e90)
}

/// The Serie C2/A end-of-league stages (`create_serie_c2_playoff_stages`, ita_ser_c2a.cpp box lines 0x276, 0x28a and
/// 0x2ad): 2nd v 5th and 3rd v 4th play the promotion play-off; 14th v 17th and 15th v 16th play the relegation play-out.
/// Original: `ItaSerC2a::create_play_off_stages` (0x653e90, ita_ser_c2a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2a_create_play_off_stages(this: usize) {
    create_serie_c2_playoff_stages(League(this), 0x9ea654, [0x276, 0x28a, 0x2ad])
}

/// Comparer for `ita_ser_c2a_advance_stage`: side by side on `league::CURRENT_STAGE` when no stage is created, else
/// only the original (`ita_ser_c1a_compare_advance_stage`).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2a_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    ita_ser_c1a_compare_advance_stage("ita_ser_c2a_advance_stage", League(this), || o(this), || ita_ser_c2a_advance_stage(this))
}

/// Comparer for `ita_ser_c2a_create_play_off_stages`: only the original (`compare_serie_c2_playoff_stages`).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2a_create_play_off_stages(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_serie_c2_playoff_stages("ita_ser_c2a_create_play_off_stages", League(this), || o(this))
}
