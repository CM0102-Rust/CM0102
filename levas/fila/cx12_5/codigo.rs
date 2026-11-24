// ---------- cx12_5: the Serie C2/C stage advance, end-of-league stages and club progress (ita_ser_c2c.cpp) ----------

/// The Serie C2/C stage advance (`ita_ser_c1a_advance_stage_cql5_advance_stage`, Serie C1's body, checked instruction
/// by instruction): at the last stage the ita_ser_c2c.cpp box of line 0x250; otherwise the next stage and, on reaching
/// stage 0 (the league is over), the play-off and the play-out (`ita_ser_c2c_create_playoff_stages`, 0x65c4e0).
/// Original: `ItaSerC2c::advance_stage` (0x65c450, ita_ser_c2c.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2c_advance_stage(this: usize) {
    ita_ser_c1a_advance_stage_cql5_advance_stage(League(this), 0x9ea83c, 0x250, 0x65c4e0)
}

/// The Serie C2/C end-of-league stages, as the other C2 groups' (`create_serie_c2_playoff_stages`, ita_ser_c2c.cpp box
/// lines 0x275, 0x289 and 0x2ac): 2nd v 5th and 3rd v 4th play the promotion play-off; 14th v 17th and 15th v 16th play
/// the relegation play-out.
/// Original: `ItaSerC2c::create_playoff_stages` (0x65c4e0, ita_ser_c2c.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2c_create_playoff_stages(this: usize) {
    covered!("cx12_5 serie c2 c end stages");
    create_serie_c2_playoff_stages(League(this), 0x9ea83c, [0x275, 0x289, 0x2ac]) // cx12_5: the C2/C end stages
}

/// The club's place forecast in Serie C2/C, the same rule as Serie C1's and the other C2 groups' (`set_c1_club_progress`,
/// checked instruction by instruction, jump tables included): in the league, best and worst place become bands (2 to 5,
/// the promotion play-off: best 2, worst 5; 14 to 17, the play-out: best 14, worst 17); in the play-off, the three
/// places + 1; in the play-out, best 1 becomes 14 and 2 becomes 16; written to the club's entry in the competition
/// statistics (current, best, worst).
/// Original: `ItaSerC2c::set_club_progress` (0x65c7d0, ita_ser_c2c.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2c_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx12_5 serie c2 c progress");
    set_c1_club_progress(this, club, stage as u8, current as u8, best as u8, worst as u8) // cx12_5: the C2/C progress
}

/// Comparer for `ita_ser_c2c_advance_stage`: side by side on `league::CURRENT_STAGE` when no stage is created, else
/// only the original (`ita_ser_c1a_compare_advance_stage`).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2c_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    ita_ser_c1a_compare_advance_stage("ita_ser_c2c_advance_stage", League(this), || o(this), || ita_ser_c2c_advance_stage(this))
}

/// Comparer for `ita_ser_c2c_create_playoff_stages`: only the original (`compare_serie_c2_playoff_stages`).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2c_create_playoff_stages(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_serie_c2_playoff_stages("ita_ser_c2c_create_playoff_stages", League(this), || o(this))
}

/// Comparer for `ita_ser_c2c_set_club_progress`: side by side on the 3 bytes of the club's entry
/// (`compare_c1_club_progress`).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2c_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    compare_c1_club_progress("ita_ser_c2c_set_club_progress", this, club, || o(this, club, stage, current, best, worst), || ita_ser_c2c_set_club_progress(this, club, stage, current, best, worst))
}
