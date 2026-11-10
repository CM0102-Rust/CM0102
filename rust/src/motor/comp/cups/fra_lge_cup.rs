use super::*;

/// rule: the French League Cup has 7 single-leg rounds: preliminary (code 0xe6, 16 teams) drawn 1/9, played 5/10; 1st
/// round 15/10 and 16/11 (28 teams, 20 new); 2nd 26/11 and 8/1 (32 teams, 18 new); 3rd 14/1 and 29/1; quarter-finals
/// 3/2 and 19/2; semi-final 4/3 and 1/4; final 2/4 and 22/4 at a neutral ground [inference: venue 4]; 300 thousand for
/// winning each round and 1.5 million to the winner; category 4 up to the quarter-finals and 6 after
/// Original: `FraLgeCup::create_round_table` (0x5bcc40, fra_lge_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5bcc40 } (thiscall, ret 0x10, fra_lge_cup.cpp; a virtual +0x3c da Copa da Liga francesa): `FraLgeCup::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, as 7 rodadas; a caixa da linha 0xa2 sem memória.
pub(crate) unsafe extern "thiscall" fn fra_lge_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    single_stage_round_table(Cup(this), stage, rounds, extra, out, &FRENCH_LEAGUE_CUP_ROUNDS, 0x9ccb98, 0xa2, "cql_11 tabela da copa da liga francesa")
}

/// rule: the French League Cup takes 54 clubs, in this order (the first 16 in the preliminary round, the next 20 in the
/// 1st round and the last 18 in the 2nd): those of division [0x9cf5d4] that did not come from [0x9cf5d8] (up to 16 in
/// 2001 and 14 in other years), topped up to 16 with those of [0x9cf5d4] that came from [0x9cf5d8] (the promoted ones
/// [inference]); the Second and the First Division; with an incomplete list it returns 0
/// Original: `FraLgeCup::collect_entrants` (0x5bd160, fra_lge_cup.cpp, thiscall, ret 0).
///
/// [pt] regra: entram na Copa da Liga francesa 54 clubes, nesta ordem (os 16 primeiros na preliminar, os 20 seguintes na 1ª
/// rodada e os 18 últimos na 2ª): os da divisão [0x9cf5d4] que não vieram da [0x9cf5d8] (até 16 em 2001 e 14 nos outros
/// anos), completados até 16 com os da [0x9cf5d4] que vieram da [0x9cf5d8] (os promovidos [inferência]); a Segunda e a
/// Primeira; com a lista incompleta, devolve 0
pub(crate) unsafe extern "thiscall" fn fra_lge_cup_collect_entrants(this: usize) -> i32 {
    french_league_cup_collect_entrants(Cup(this)).0
}
