use super::*;

/// rule: the French League Cup has 7 single-leg rounds: preliminary (code 0xe6, 16 teams) drawn 1/9, played 5/10; 1st
/// round 15/10 and 16/11 (28 teams, 20 new); 2nd 26/11 and 8/1 (32 teams, 18 new); 3rd 14/1 and 29/1; quarter-finals
/// 3/2 and 19/2; semi-final 4/3 and 1/4; final 2/4 and 22/4 at a neutral ground [inference: venue 4]; 300 thousand for
/// winning each round and 1.5 million to the winner; category 4 up to the quarter-finals and 6 after
/// Original: `FraLgeCup::create_round_table` (0x5bcc40, fra_lge_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn fra_lge_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    single_stage_round_table(Cup(this), stage, rounds, extra, out, &FRENCH_LEAGUE_CUP_ROUNDS, 0x9ccb98, 0xa2, "cql_11 tabela da copa da liga francesa")
}

/// rule: the French League Cup takes 54 clubs, in this order (the first 16 in the preliminary round, the next 20 in the
/// 1st round and the last 18 in the 2nd): those of division `KEY_CLUB_COMP_FRENCH_NATIONAL` that did not come from
/// `KEY_CLUB_COMP_FRENCH_CFA` (up to 16 in 2001 and 14 in other years), topped up to 16 with those of
/// `KEY_CLUB_COMP_FRENCH_NATIONAL` that came from `KEY_CLUB_COMP_FRENCH_CFA` (the promoted ones [inference]); the
/// Second and the First Division; with an incomplete list it returns 0
/// Original: `FraLgeCup::collect_entrants` (0x5bd160, fra_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fra_lge_cup_collect_entrants(this: usize) -> i32 {
    french_league_cup_collect_entrants(Cup(this)).0
}
