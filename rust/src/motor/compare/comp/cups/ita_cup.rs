use super::*;

// The comparers of the routines in ../comp/cups/ita_cup.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/cups/ita_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `ita_cup_create_first_group_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_cup_create_first_group_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_CUP_CREATE_FIRST_GROUP_TABLE);
    compare_first_group_table("ita_cup_create_first_group_table", this, 4, || o(this))
}

/// Comparer for `init_first_group` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_init_first_group(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_INIT_FIRST_GROUP);
    compare_first_group_table("init_first_group", this, 5, || o(this))
}

/// Comparer for `ita_cup_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ITA_CUP_ADVANCE_STAGE);
    ita_cup_compare_advance_stage("ita_cup_advance_stage", this, || o(this))
}

/// Comparer for `ita_cup_advance_stage_63d950` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_cup_advance_stage_63d950(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ITA_CUP_ADVANCE_STAGE_63D950);
    ita_cup_compare_advance_stage("ita_cup_advance_stage_63d950", this, || o(this))
}

/// Comparer for `ita_cup_create_group_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_ITA_CUP_CREATE_GROUP_STAGE);
    compare_group_stage("ita_cup_create_group_stage", this, g, || o(this, g))
}

/// Comparer for `ita_cup_create_group_stage_63d9e0` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_cup_create_group_stage_63d9e0(this: usize, g: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_ITA_CUP_CREATE_GROUP_STAGE_63D9E0);
    compare_group_stage("ita_cup_create_group_stage_63d9e0", this, g, || o(this, g))
}

/// Comparer for `ita_cup_create_knockout_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_cup_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ITA_CUP_CREATE_KNOCKOUT_STAGE);
    // simplification: allocates the stage and draws the first round; cannot run twice: original only, with the plan
    ita_cup_compare_knockout_stage("ita_cup_create_knockout_stage", this, 7, 16, || o(this))
}

/// Comparer for `ita_cup_create_knockout_stage_63dc10` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_cup_create_knockout_stage_63dc10(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ITA_CUP_CREATE_KNOCKOUT_STAGE_63DC10);
    // simplification: builds and destroys the provisional league, allocates the stage and draws; cannot run twice: original only
    ita_cup_compare_knockout_stage("ita_cup_create_knockout_stage_63dc10", this, 0xf, 10, || o(this))
}

/// Comparer for `ita_cup_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ITA_CUP_CREATE_SCHEDULE);
    ita_cup_compare_schedule("ita_cup_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| ita_cup_create_schedule(this, stage, r, e, l), 7)
}

/// Comparer for `ita_cup_create_schedule_63cf90` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_cup_create_schedule_63cf90(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ITA_CUP_CREATE_SCHEDULE_63CF90);
    ita_cup_compare_schedule("ita_cup_create_schedule_63cf90", stage, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| ita_cup_create_schedule_63cf90(this, stage, r, e, l), 0xf)
}

/// Comparer for `ita_cup_create_round_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ITA_CUP_CREATE_ROUND_TABLE);
    eng_cc_cup_compare_round_table("ita_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| ita_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `ita_cup_create_round_table_63c890` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_cup_create_round_table_63c890(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ITA_CUP_CREATE_ROUND_TABLE_63C890);
    eng_cc_cup_compare_round_table("ita_cup_create_round_table_63c890", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| ita_cup_create_round_table_63c890(this, stage, r, e, s))
}

/// Comparer for `ita_cup_select_entrants` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_cup_select_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_ITA_CUP_SELECT_ENTRANTS);
    compare_select_entrants("ita_cup_select_entrants", this, || o(this), || select_coppa_italia_entrants(this))
}

/// Comparer for `ita_cup_select_entrants_63dfe0` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_cup_select_entrants_63dfe0(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_ITA_CUP_SELECT_ENTRANTS_63DFE0);
    compare_select_entrants("ita_cup_select_entrants_63dfe0", this, || o(this), || select_serie_c_cup_entrants(this))
}

/// Comparer for `ita_cup_init_team_list` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_cup_init_team_list(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_ITA_CUP_INIT_TEAM_LIST);
    compare_team_list("ita_cup_init_team_list", this, || o(this), || init_supercoppa_teams(this))
}

/// Comparer for `ita_cup_set_up_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_cup_set_up_rules(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_CUP_SET_UP_RULES)(this);
    check_cup_league_rules("ita_cup_set_up_rules", this, &ITALIAN_CUP_GROUP_RULES, r);
    r
}

/// Comparer for `ita_cup_set_up_rules_63d6e0` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ita_cup_set_up_rules_63d6e0(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_CUP_SET_UP_RULES_63D6E0)(this);
    check_cup_league_rules("ita_cup_set_up_rules_63d6e0", this, &SERIE_C_CUP_RULES, r);
    r
}
