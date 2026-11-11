use super::*;

/// rule: the Serie C1 Super Cup is a single match on 21 May of the following year, drawn on 15 May, category 6, no
/// prizes
/// Original: `ItaC1Super::create_round_table` (0x63c890, ita_c1_super.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ita_c1_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &SERIE_C1_SUPER_CUP_ROUNDS, 0x9ea1c8, 0x99, "cqo_10 tabela da supercopa c1")
}

/// rule: the Serie C1 Super Cup starts with both places empty; the clubs enter at the end of the season [inference: the
/// champions of the two C1 groups]
/// Original: `ItaC1Super::init_team_list` (0x63c9d0, ita_c1_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_c1_super_init_team_list(this: usize) -> i32 {
    init_c1_super_cup_list(this).0
}
