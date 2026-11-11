use super::*;

/// rule: the Italian Super Cup is a single match (2 teams) on 19 August, drawn on 5 July, category 6, tie-break 0x203,
/// no prizes
/// Original: `ItaSuper::create_round_table` (0x65f920, ita_super.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ita_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &ITALIAN_SUPER_CUP, 0x9eac34, 0x99, "cqo_10 tabela da supercopa")
}

/// rule: the Italian Super Cup is between the Serie A champion (the home side) and the Coppa Italia winner; if the same
/// club won both, the Cup runner-up plays
/// Original: `ItaSuper::init_team_list` (0x65fa60, ita_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_super_init_team_list(this: usize) -> i32 {
    init_supercoppa_teams(this).0
}
