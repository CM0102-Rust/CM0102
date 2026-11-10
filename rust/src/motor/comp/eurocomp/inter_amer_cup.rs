use super::*;

/// rule: the Inter-American Cup is two-legged from 26 January of the following year (7 days), drawn on 10 October,
/// category 8, two-leg tie-break 3
/// Original: `InterAmerCup::create_round_table` (0x632340, inter_amer_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x632340 } (thiscall, ret 0x10, inter_amer_cup.cpp; a virtual +0x3c): `InterAmerCup::create_round_table(...)`:
/// na fase −1, a rodada única; a caixa da linha 0x9a sem memória.
pub(crate) unsafe extern "thiscall" fn inter_amer_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &INTERAMERICAN_CUP, 0x9e9d94, 0x9a, "cqp_13 tabela da interamericana")
}
