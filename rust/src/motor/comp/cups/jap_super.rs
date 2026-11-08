use super::*;

/// rule: the Japanese Super Cup (Xerox Super Cup) is a single match between 2 clubs, drawn on 1/2 and played on 4/3
/// (moved to weekday 5) at a neutral ground [inference: venue 4], decided on penalties [inference: tiebreak 1]; 150
/// thousand to the winner; category 6
/// Original: `JapSuper::create_round_table` (0x666020, jap_super.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x666020 } (thiscall, ret 0x10, jap_super.cpp; a virtual +0x3c da Supercopa do Japão): `JapSuper::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única; a caixa da linha 0x98 sem memória.
pub(crate) unsafe extern "thiscall" fn jap_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    fra_super_create_single_stage_round_table(this, stage, rounds, extra, out, &JAPAN_SUPER_CUP_ROUNDS, 0x9eaf4c, 0x98, "cql_12 tabela da supercopa do japao", |_| {})
}

/// rule: the Japanese Super Cup is between the J1 champion and the Emperor's Cup winner; a winner with no nation (+0x53
/// = 0, a club that no longer exists [inference]) gives the place to the runner-up; if it is the same club, the
/// Emperor's Cup runner-up comes in and, if they still coincide, the J1 runner-up; if one is missing, the error box
/// Original: `JapSuper::select_teams` (0x666160, jap_super.cpp, thiscall, ret 0).
///
/// [pt] regra: a Supercopa do Japão é entre o campeão da J1 e o da Copa do Imperador; um campeão sem nação (+0x53 = 0, clube
/// que não existe mais [inferência]) dá a vaga ao vice; se é o mesmo clube, entra o vice da Copa do Imperador e, se
/// ainda coincidir, o vice da J1; sem algum deles, a caixa de erro
pub(crate) unsafe extern "thiscall" fn jap_super_select_teams(this: usize) -> i32 {
    select_japanese_super_cup_teams(this).0
}

/// rule: at the season turn the Japanese Super Cup is rebuilt: last edition's tables freed, history archived, year +1,
/// the two clubs (JapSuper::select_teams: the J1 champion and the Emperor's Cup winner [inference]), the rounds, the
/// tree and the predicted positions
/// Original: `JapSuper::start_new_season` (0x6664e0, jap_super.cpp, thiscall, ret 0).
///
/// [pt] { 0x6664e0 } (thiscall, ret 0, jap_super.cpp; a virtual +0x08 da Supercopa japonesa): `JapSuper::start_new_season()`,
/// a virada (`usa_mls_all_stars_cup_start_new_season`) com os dois clubes de JapSuper::select_teams (0x666160) e as caixas do jap_super.cpp nas
/// linhas 0x14b, 0x152 e 0x159.
pub(crate) unsafe extern "thiscall" fn jap_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(this, 0x666160, 0x9eaf4c, [0x14b, 0x152, 0x159], 1, true, "cqr_6 virada da supercopa japonesa") // cqr6: a Supercopa japonesa
}
