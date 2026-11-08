use super::*;






/// The Pernambuco turnover: archives the season in the statistics, clears the previous table and stages, moves to the
/// next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order;
/// then the predicted positions and the renewal of the state referees
/// Original: `BraRegPern::start_new_season` (0x4394d0, bra_reg_pern.cpp, thiscall, ret 0).
///
/// [pt] { 0x4394d0 } (thiscall, ret 0, bra_reg_pern.cpp): `BraRegPern::start_new_season()`, a virada da temporada do Pernambucano (a tabela:
/// 0x439330; caixas nas linhas 0x1ca, 0x1d1, 0x1d7 e 0x1de).
pub(crate) unsafe extern "thiscall" fn bra_reg_pern_start_new_season(this: usize) -> u32 {
    state_league_new_season(this, 0x439330, 0x98afd8, [0x1ca, 0x1d1, 0x1d7, 0x1de], 1, "cqp_9 virada do pernambucano") // cqp9: o ano seguinte do Pernambucano
}

/// The Pernambuco next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the
/// semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box)
/// Original: `BraRegPern::advance_stage` (0x439680, bra_reg_pern.cpp, thiscall, ret 0).
///
/// [pt] { 0x439680 } (thiscall, ret 0, bra_reg_pern.cpp): `BraRegPern::advance_stage()`, a fase seguinte do Pernambucano: no fim dos pontos corridos,
/// o mata-mata de 4 (0x439710); a caixa da linha 0x1f8.
pub(crate) unsafe extern "thiscall" fn bra_reg_pern_advance_stage(this: usize) {
    state_league_advance_stage(this, 0x98afd8, 0x1f8, 0x439710);
}















/// Pernambuco: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the
/// seed; the ties follow the seeds [inference: two legs, the decider at the better placed club])
/// Original: `BraRegPern::create_semi_final_stage` (0x439710, bra_reg_pern.cpp, thiscall, ret 0).
///
/// [pt] { 0x439710 } (thiscall, ret 0, bra_reg_pern.cpp): `BraRegPern::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98afd8, linhas 0x21b (sem lista) e 0x22f (sem fase).
pub(crate) unsafe extern "thiscall" fn bra_reg_pern_create_semi_final_stage(this: usize) {
    create_knockout_from_table(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98afd8, 0x21b, 0x22f, "cqn_8 pernambucano"); // cqn8: 1º × 4º, 2º × 3º Pernambucano
}



/// Comparer: runs only the original and checks the new stage exists with n teams, the competition record and the
/// planned clubs (as a set), and +0x30 = 0.
///
/// [pt] O comparador dos três: a original aloca a lista, a tabela de rodadas e a fase e monta a rodada (só ela roda). O
/// plano, feito antes: os clubes das linhas. Depois, a fase da primeira vaga tem de existir, com +0x3e = n, +4 = o +4 da
/// liga e os n primeiros clubes da lista dela (+0xa7, 6 bytes) iguais aos do plano (como conjunto: o sorteio da rodada
/// 0x51c800 pode reordenar as chaves), e +0x30 = 0.
pub(crate) unsafe fn bra_reg_pern_compare_knockout_from_table(name: &str, this: usize, rows: &[usize], original: impl FnOnce()) {
    let mut plan: Vec<usize> = rows.iter().map(|&l| bra_reg_bahia_table_row_club(this, l)).collect();
    original();
    let stage = read_usize(read_usize(this + league::STAGES));
    if stage == 0 {
        SCOREBOARD.record(false, || format!("{}({:#x}) sem fase", name, this));
        return;
    }
    let list = read_usize(stage + cup::TEAM_TABLE);
    let mut found: Vec<usize> = (0..rows.len()).map(|k| read_usize(list + 6 * k)).collect();
    let (pl, ac) = (plan.clone(), found.clone());
    plan.sort_unstable();
    found.sort_unstable();
    let same = plan == found && read_i16(stage + cup::TEAM_COUNT) as usize == rows.len() && read_usize(stage + 4) == read_usize(this + 4) && read_usize(this + league::CURRENT_STAGE) == 0;
    SCOREBOARD.record(same, || format!("{}({:#x}) plano={:x?} fase={:#x} lista={:x?} +0x3e={}", name, this, pl, stage, ac, read_i16(stage + cup::TEAM_COUNT)));
}




/// The Pernambuco state league at career creation: the year and the database competition, disciplinary rule 7 and, with
/// no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table
/// with the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegPern::BraRegPern` (0x4388f0, bra_reg_pern.cpp, thiscall, ret 8).
///
/// [pt] { 0x4388f0 } (thiscall, ret 8, bra_reg_pern.cpp): `BraRegPern::BraRegPern(ano, registro)`: o Pernambucano (uma fase além da liga).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_pern(this: usize, year: u32, record: usize) -> usize {
    bra_reg_northeast_construct_state_league(this, year, record, &PERNAMBUCANO_STATE_CHAMPIONSHIP, 7, true, "cqt_3 pernambucano novo") // cqt3: a disciplina e as posições previstas do Pernambucano
}


/// the Pernambuco league has 10 clubs: those with it as their reserve division in the database (the field that holds
/// the state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegPern::create_table` (0x439330, bra_reg_pern.cpp, thiscall, ret 0).
///
/// [pt] { 0x439330 } (thiscall, ret 0, bra_reg_pern.cpp): `BraRegPern::create_table()`: o Pernambucano tem 10 clubes (divisão reserva +0x60 = [0x9cf9bc]).
/// Fonte 0x98afd8, linha 0x167.
pub(crate) unsafe extern "thiscall" fn bra_reg_pern_create_table(this: usize) -> u32 {
    fill_state_league_table(this, 0xa, 0x98afd8, 0x167, KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP, 0, "cqm_7 pernambucano cheio")
}


/// North and Pernambuco (the same routine in the two vtables): 4 top places (the final stage of 4); 2 relegated; double
/// round robin; win 3, draw 1; tie-break by head-to-head, then goal difference; no play-offs in the table; no league
/// above or below (the relegated go to the non-playable pool [inference]); rule word 8 (the top places lead to the next
/// stage; no champion band); 7 bench substitutes and 3 substitutions; category 2 (league) matches. Then the main stage
/// schedule
/// Original: `BraRegPern::set_up_rules` (0x439430, bra_reg_pern.cpp, thiscall).
///
/// [pt] { 0x439430 } (thiscall, bra_reg_pern.cpp): `BraRegPern::set_up_rules`, nas vtables do Norte (0x968338) e do
/// Pernambucano (0x968554); devolve 1.
pub(crate) unsafe extern "thiscall" fn bra_reg_pern_set_up_rules(this: usize) -> u32 {
    pernambucano_rules(this);
    crate::comparison::branch("cqi_9 regras do pernambucano");
    create_main_schedule(this);
    1
}

