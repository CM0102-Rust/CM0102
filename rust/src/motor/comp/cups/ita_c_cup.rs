use super::*;

/// rule: the Serie C Cup starts with five-round groups in August (12, 19, 22, 26 and 29; id 0x420 + stage); the
/// knockout (stage 0xf) over two legs: 1st round drawn on 30/8 and played on 11/10 (32 teams), 2nd 26/10 and 1/11,
/// quarter-finals 9/11 and 15/11, semi-final 30/11 and 11 April, final draw 26/4 and match on 9 May; 14 days between
/// legs, 7 in the 2nd round and the final; category 4 up to the 2nd round and 6 after
/// Original: `ItaCCup::create_schedule` (0x63cf90, ita_c_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ita_c_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_schedule(League(this), stage, rounds, extra, league, 0xf, 0x420, &SERIE_C_CUP_GROUP_ROUNDS, &SERIE_C_CUP_KNOCKOUT_ROUNDS, 0x9ea204, 0xbc, 0xe3,
        "cqo_10 grupos da copa da serie c", "cqo_10 mata-mata da copa da serie c")
}

/// rule: the Serie C Cup takes the 90 Italian clubs of C1 and C2 (the two C1 groups and the three C2 ones). Those that
/// came down from Serie B and those that came from C1 in 4th, 5th or 6th go to the last places (from the 81st); the
/// first 80 are shuffled, sorted by the city's longitude and each of their 8 tens shuffled (the last 10 places stay in
/// order): the groups are regional [inference]
/// Original: `ItaCCup::select_entrants` (0x63dfe0, ita_c_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_c_cup_select_entrants(this: usize) -> i32 {
    select_serie_c_cup_entrants(this).0
}

/// rule: the Serie C Italian Cup groups follow the Italian Cup rules (single round robin, 3/1, tiebreak 4, 1, 2, one
/// qualifier per group); prize fund of 300,000
/// Original: `ItaCCup::set_up_rules` (0x63d6e0, ita_c_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_c_cup_set_up_rules(this: usize) -> u32 {
    covered!("cqi_12 serie c cup rules");
    let rounds = 1i16; // cqi12: the single round-robin of the Serie C groups
    set_cup_league_rules(League(this), &SERIE_C_CUP_RULES, rounds)
}
