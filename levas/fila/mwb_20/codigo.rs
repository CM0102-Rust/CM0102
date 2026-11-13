// ---------- mwb_20: the Welsh Premier Cup (wel_prm_cup.cpp): the group table, the rules and the next stage ----------

/// rule: the Welsh Premier Cup's own table holds the first group of four: pool entries 0, 2, 4 and 6, taken out of the
/// pool; then the group draw news; out of memory, the box of line 0x181 and 0 (`welsh_cup_group_table`, mwb_18)
/// Original: `WelPrmCup::create_group_table` (0x928060, wel_prm_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_create_group_table(this: usize) -> u32 {
    welsh_cup_group_table(League(this), 4, 2, 0xad572c, 0x181, 0, "mwb_20 welsh premier cup table") // mwb20: the Premier Cup first group
}

/// Comparer for `wel_prm_cup_create_group_table` (the plan, original only).
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_create_group_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_welsh_cup_group_table("wel_prm_cup_create_group_table", League(this), 4, 2, || o(this))
}

/// The Welsh Premier Cup rule writes: the top 2 of each group go through, tie-breaks 1, 2, 4 [inference: goal
/// difference, goals for, head-to-head].
pub(crate) fn welsh_premier_cup_rules() -> Vec<(usize, u8, u32)> {
    welsh_cup_rules(2, [1, 2, 4]) // mwb20: the Premier Cup group rules
}

/// rule: the Welsh Premier Cup groups play 2 rounds, 3 points for a win and 1 for a draw, category 4; the top 2 of each
/// group go through; tie-breaks codes 1, 2 and 4 [inference: goal difference, goals for, head-to-head]; a prize fund of
/// 100,000; builds the round schedule of the main stage (virtual +0x3c); returns 1
/// Original: `WelPrmCup::set_up_rules` (0x928160, wel_prm_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_set_up_rules(this: usize) -> u32 {
    covered!("mwb_20 welsh premier cup rules");
    eng_prm_apply_league_rules(League(this), &welsh_premier_cup_rules())
}

/// Comparer for `wel_prm_cup_set_up_rules` (side by side on the rule bytes, the copy without the round table).
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    eng_prm_compare_league_rules("wel_prm_cup_set_up_rules", League(this), welsh_premier_cup_rules(), || o(this), || wel_prm_cup_set_up_rules(this))
}

/// rule: the Welsh Premier Cup moves from the first group (its own table) to the second (stage 0) and, on reaching
/// stage 1, builds the knockout stage of the eight (WelPrmCup::create_knockout_stage 0x928650); past the last stage,
/// the box of line 0x220 (`welsh_cup_advance_stage`, mwb_18)
/// Original: `WelPrmCup::advance_stage` (0x9283c0, wel_prm_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_advance_stage(this: usize) {
    welsh_cup_advance_stage(League(this), 0xad572c, 0x220, 1, 0x928650)
}

/// Comparer for `wel_prm_cup_advance_stage`: only the original runs (the knockout builds a stage); the current stage
/// stays at the last stage, otherwise goes up by one.
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    compare_state_league_advance_stage("wel_prm_cup_advance_stage", League(this), || o(this))
}
