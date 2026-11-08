use super::*;

/// rule: the Intercontinental Cup (the European against the South American champion [inference]) is a single match on 1
/// December, drawn on 10 October, venue 4 (Tokyo [inference]), category 6, tie-break 3
/// Original: `WorldClubCup::create_round_table` (0x92b770, world_club_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x92b770 } (thiscall, ret 0x10, world_club_cup.cpp; a virtual +0x3c): `WorldClubCup::create_round_table(...)`:
/// na fase −1, a rodada única; a caixa da linha 0x9a sem memória.
pub(crate) unsafe extern "thiscall" fn world_club_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(this, stage, rounds, extra, out, &INTERCONTINENTAL_CUP_ROUNDS, 0xad5848, 0x9a, "cqp_13 tabela da intercontinental")
}
