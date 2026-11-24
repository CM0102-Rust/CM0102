// ---------- cx02_25: the Greek Cup next stage (gre_cup.cpp) ----------

/// The Greek Cup moves on to its next stage: past the last stage the box at line 0x24a of gre_cup.cpp and nothing
/// else; otherwise the current stage goes up by one and, on reaching stage 10, the knockout stage is built
/// (GreCup::create_knockout_stage 0x5e9850) (`welsh_cup_advance_stage`, checked instruction by instruction against
/// WelPrmCup::advance_stage: only the file, the line and the stage change).
/// Original: `GreCup::advance_stage` (0x5e95c0, gre_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_cup_advance_stage(this: usize) {
    welsh_cup_advance_stage(League(this), 0x9cdab0, 0x24a, 10, 0x5e9850)
}

/// Comparer for `gre_cup_advance_stage`: only the original runs (the knockout builds a stage); the current stage stays
/// at the last stage, otherwise goes up by one.
pub(crate) unsafe extern "thiscall" fn compare_gre_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    compare_state_league_advance_stage("gre_cup_advance_stage", League(this), || o(this))
}
