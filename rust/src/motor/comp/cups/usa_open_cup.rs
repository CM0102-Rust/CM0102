use super::*;

/// rule: the US Open Cup has 5 single-leg rounds (+0x21 = 0): 2nd round (32 teams, code 0x14) drawn 1/6, played 14/6,
/// with bit 0x400 in the draw; 3rd 15/6 and 25/6; quarter-finals 26/6 and 9/8; semi-final 10/8 and 23/8; final drawn
/// 24/8, played 22/10 at a neutral ground [inference: venue 4]; no prize per round (−1); category 4 in the first two
/// and 6 after
/// Original: `UsaOpenCup::create_round_table` (0x90b170, usa_open_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn usa_open_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    build_single_stage_cup_rounds(Cup(this), stage, rounds, extra, out, &US_OPEN_CUP_ROUNDS, 0xaa20e0, 0x9c, "cqn_11 tabela da copa dos eua")
}

/// rule: the US Open Cup takes 32 clubs: 10 from each of the two lower leagues ([0x9cf594] and [0x9cf598], up to 50
/// looked at in each, sorted by reputation and shuffled among the best), with class 2, and all MLS clubs with class 1
/// Original: `UsaOpenCup::collect_entrants` (0x90b530, usa_open_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn usa_open_cup_collect_entrants(this: usize) -> i32 {
    collect_us_open_cup_entrants(Cup(this)).0
}

/// rule: the same turn in the US Open Cup: tables freed, history archived, year +1, the entrants
/// (UsaOpenCup::collect_entrants), the rounds, the tree and the predicted positions
/// Original: `UsaOpenCup::start_new_season` (0x90b790, usa_open_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn usa_open_cup_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x90b530, 0xaa20e0, [0x198, 0x19f, 0x1a6], 1, true, "cqr_6 virada da copa dos eua") // cqr6: the US Cup
}
