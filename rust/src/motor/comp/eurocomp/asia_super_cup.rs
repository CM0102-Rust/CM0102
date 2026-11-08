use super::*;

/// rule: the Asian Super Cup is two-legged, the first match on 4 August (drawn on 4 July) and the return 14 days later,
/// category 6, two-leg tie-break 0x83, no prizes
/// Original: `AsiaSuperCup::create_round_table` (0x410910, asia_super_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x410910 } (thiscall, ret 0x10, asia_super_cup.cpp; a virtual +0x3c da Supercopa Asiática): `AsiaSuperCup::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única; a caixa da linha 0x97 sem memória.
pub(crate) unsafe extern "thiscall" fn asia_super_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(this, stage, rounds, extra, out, &ASIAN_SUPER_CUP, 0x987be0, 0x97, "cqp_10 tabela da supercopa asiatica")
}

/// rule: the Asian Super Cup is between the Asian Club Championship winner and the Asian Cup Winners' Cup winner; if
/// the same club won both, the Club Championship runner-up plays
/// Original: `AsiaSuperCup::select_teams` (0x410a50, asia_super_cup.cpp, thiscall, ret 0).
///
/// [pt] regra: a Supercopa Asiática é entre o campeão da Liga dos Campeões asiática e o da Recopa asiática; se o mesmo clube
/// ganhou as duas, entra o vice da Liga dos Campeões
pub(crate) unsafe extern "thiscall" fn asia_super_cup_select_teams(this: usize) -> i32 {
    select_asian_super_cup_teams(this).0
}
