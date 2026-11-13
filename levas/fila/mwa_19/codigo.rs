// ---------- batch %%LEVA%%: the Northern Irish League Cup: the next stage and the season turnover (nir_lge_cup.cpp) ----------

/// Rule: the Northern Irish League Cup moves stage by stage through its three groups; when the groups are over (stage
/// 3) the eight-club knockout is built; past the last stage, the error box.
///
/// `advance_cup_stage` (checked instruction by instruction against ItaCup::advance_stage: only the file, the line
/// 0x21a, the end of the groups 3 and the knockout routine NirLgeCup::create_knockout_stage 0x78d600 change).
/// Original: `NirLgeCup::advance_stage` (0x78d330, nir_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_advance_stage(this: usize) {
    advance_cup_stage(League(this), 0xa69698, 0x21a, 3, 0x78d600) // mwa19: the Northern Irish League Cup knockout after the three groups
}

unsafe extern "thiscall" fn compare_nir_lge_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    ita_cup_compare_advance_stage("nir_lge_cup_advance_stage", League(this), || o(this))
}

/// Rule: at the season turn the Northern Irish League Cup is rebuilt: the table, the round table, the entrants and the
/// stages of the last edition are freed, the statistics archived, the year moves on; then the 20 entrants, the
/// predicted positions, the rules, the first group's table, its fixtures and order and the stages of the three groups.
///
/// `start_irish_league_cup_season` with NirLgeCup::collect_entrants (0x78d800), create_table (0x78cfd0),
/// create_group_stage (0x78d3c0), three groups and the error box lines 0x1ec, 0x1f3, 0x1f9 and 0x200 of
/// nir_lge_cup.cpp (checked instruction by instruction against IreLgeCup::start_new_season).
/// Original: `NirLgeCup::start_new_season` (0x78d160, nir_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_start_new_season(this: usize) -> u32 {
    start_irish_league_cup_season(League(this), 0x78d800, 0x78cfd0, 0x78d3c0, 3, 0xa69698, [0x1ec, 0x1f3, 0x1f9, 0x200], 1) // mwa19: the Northern Irish League Cup turnover
}

unsafe extern "thiscall" fn compare_nir_lge_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("nir_lge_cup_start_new_season", League(this), || o(this))
}
