// ---------- cx11_8: the Serie C2/B stage advance and end-of-league stages (ita_ser_c2b.cpp) ----------

/// The Serie C2/B stage advance, as C2/A's (`ita_ser_c1a_advance_stage_cql5_advance_stage`): the ita_ser_c2b.cpp box of
/// line 0x256 at the last stage; on reaching stage 0, the play-off and the play-out
/// (`ita_ser_c2b_create_playoff_stages`, 0x658350).
/// Original: `ItaSerC2b::advance_stage` (0x6582c0, ita_ser_c2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2b_advance_stage(this: usize) {
    ita_ser_c1a_advance_stage_cql5_advance_stage(League(this), 0x9ea75c, 0x256, 0x658350)
}

/// The Serie C2/B end-of-league stages, as C2/A's (`create_serie_c2_playoff_stages`, ita_ser_c2b.cpp box lines 0x27b,
/// 0x28f and 0x2b2).
/// Original: `ItaSerC2b::create_playoff_stages` (0x658350, ita_ser_c2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2b_create_playoff_stages(this: usize) {
    covered!("cx11_8 serie c2 b end stages");
    create_serie_c2_playoff_stages(League(this), 0x9ea75c, [0x27b, 0x28f, 0x2b2]) // cx11_8: the C2/B end stages
}

/// Comparer for `ita_ser_c2b_advance_stage`: side by side on `league::CURRENT_STAGE` when no stage is created, else
/// only the original (`ita_ser_c1a_compare_advance_stage`).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2b_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    ita_ser_c1a_compare_advance_stage("ita_ser_c2b_advance_stage", League(this), || o(this), || ita_ser_c2b_advance_stage(this))
}

/// Comparer for `ita_ser_c2b_create_playoff_stages`: only the original (`compare_serie_c2_playoff_stages`).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2b_create_playoff_stages(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_serie_c2_playoff_stages("ita_ser_c2b_create_playoff_stages", League(this), || o(this))
}
