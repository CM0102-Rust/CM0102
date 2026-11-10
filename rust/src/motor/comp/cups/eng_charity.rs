use super::*;

/// rule: the Charity Shield is a single match between 2 clubs, drawn on 5/7 and played on 13/8 (moved to weekday 6) at
/// a neutral ground [inference: venue 4], decided on penalties [inference: tiebreak 1]; category 6; no prize
/// Original: `EngCharity::create_round_table` (0x56d640, eng_charity.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x56d640 } (thiscall, ret 0x10, eng_charity.cpp; a virtual +0x3c da Supercopa): `EngCharity::create_round_table(
/// fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única; a caixa da linha 0x9c sem memória.
pub(crate) unsafe extern "thiscall" fn eng_charity_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &CHARITY_SHIELD_ROUNDS, 0x9c425c, 0x9c, "cql_10 tabela da supercopa inglesa")
}

/// rule: the Charity Shield is between the Premier champion and the FA Cup winner; if the same club won both, the
/// Premier runner-up takes its place; if one is missing, the error box
/// Original: `EngCharity::select_teams` (0x56d780, eng_charity.cpp, thiscall, ret 0).
///
/// [pt] regra: a Supercopa inglesa (Charity Shield) é entre o campeão da Premier e o da Copa da Inglaterra; se o mesmo clube
/// ganhou as duas, entra o vice da Premier; sem algum deles, a caixa de erro
pub(crate) unsafe extern "thiscall" fn eng_charity_select_teams(this: usize) -> i32 {
    select_charity_shield_teams(Cup(this)).0
}
