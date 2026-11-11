use super::*;

/// rule: the same turn in the Emperor's Cup (Japan): tables freed, history archived, year +1, the entrants
/// (JapEmpCup::collect_entrants), the rounds, the tree and the predicted positions
/// Original: `JapEmpCup::start_new_season` (0x662950, jap_emp_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn jap_emp_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x6624f0, 0x9eab90, [0x217, 0x21e, 0x225], 1, "cqr_5 virada da copa do imperador") // cqr5: the Emperor’s Cup
}

/// rule: the Emperor's Cup has 7 single-leg rounds with a seeded draw (+0xd = 0x83): 1st round (64 teams) drawn 1/11,
/// played 29/11; 2nd 30/11 and 6/12; 3rd (16 enter) 7/12 and 13/12; 4th 14/12 and 20/12; quarter-finals 21/12 and
/// 23/12; semi-final 24/12 and 27/12; final drawn 28/12, played on 2/1 of the next year at a neutral ground [inference:
/// venue 4]; no prize per round (−1)
/// Original: `JapEmpCup::create_round_table` (0x661ff0, jap_emp_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn jap_emp_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    build_single_stage_cup_rounds(Cup(this), stage, rounds, extra, out, &EMPERORS_CUP_ROUNDS, 0x9eab90, 0xa0, "cqn_11 tabela da copa do imperador")
}

/// rule: the Emperor's Cup takes 80 clubs: 47 from the regional divisions ([0x9cf678], sorted by reputation and
/// shuffled), 2 from the university league ([0x9cf674] [inference]), 3 from the football league ([0x9cf670]
/// [inference]) — those drawn from other Japanese divisions (up to 6) do not enter —, the J2 clubs (class 0) and the J1
/// clubs with class = last season's position; the last 16 entries are sorted by class; if the lower divisions lack
/// clubs, the error box
/// Original: `JapEmpCup::collect_entrants` (0x6624f0, jap_emp_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn jap_emp_cup_collect_entrants(this: usize) -> i32 {
    collect_emperors_cup_entrants(Cup(this)).0
}
