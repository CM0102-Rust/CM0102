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

/// The Santa Catarina turnover: archives the season in the statistics, clears the previous table and stages, moves to
/// the next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table
/// order; then the predicted positions and the renewal of the state referees
/// Original: `BraRegSanta::start_new_season` (0x43bd10, bra_reg_santa.cpp, thiscall, ret 0).
///
/// [pt] { 0x43bd10 } (thiscall, ret 0, bra_reg_santa.cpp): `BraRegSanta::start_new_season()`, a virada da temporada do Catarinense (a tabela:
/// 0x43bb70; caixas nas linhas 0x1ca, 0x1d1, 0x1d7 e 0x1de).
pub(crate) unsafe extern "thiscall" fn bra_reg_pern_start_new_season_43bd10(this: usize) -> u32 {
    state_league_new_season(this, 0x43bb70, 0x98b050, [0x1ca, 0x1d1, 0x1d7, 0x1de], 1, "cqp_9 virada do catarinense") // cqp9: o ano seguinte do Catarinense
}

/// The Santa Catarina next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to
/// the semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box)
/// Original: `BraRegSanta::advance_stage` (0x43bec0, bra_reg_santa.cpp, thiscall, ret 0).
///
/// [pt] { 0x43bec0 } (thiscall, ret 0, bra_reg_santa.cpp): `BraRegSanta::advance_stage()`, a fase seguinte do Catarinense: no fim dos pontos corridos,
/// o mata-mata de 4 (0x43bf50); a caixa da linha 0x1f8.
pub(crate) unsafe extern "thiscall" fn bra_reg_pern_advance_stage_43bec0(this: usize) {
    state_league_advance_stage(this, 0x98b050, 0x1f8, 0x43bf50);
}

/// The Sao Paulo turnover: archives the season in the statistics, clears the previous table and stages, moves to the
/// next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order;
/// then the predicted positions and the renewal of the state referees
/// Original: `BraRegSp::start_new_season` (0x43cd40, bra_reg_sp.cpp, thiscall, ret 0).
///
/// [pt] { 0x43cd40 } (thiscall, ret 0, bra_reg_sp.cpp): `BraRegSp::start_new_season()`, a virada da temporada do Paulista (a tabela:
/// 0x43cc40; caixas nas linhas 0x1bc, 0x1c3, 0x1c9 e 0x1d0).
pub(crate) unsafe extern "thiscall" fn bra_reg_pern_start_new_season_43cd40(this: usize) -> u32 {
    state_league_new_season(this, 0x43cc40, 0x98b090, [0x1bc, 0x1c3, 0x1c9, 0x1d0], 1, "cqp_9 virada do paulista") // cqp9: o ano seguinte do Paulista
}

/// The Sao Paulo next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the
/// semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box)
/// Original: `BraRegSp::advance_stage` (0x43cef0, bra_reg_sp.cpp, thiscall, ret 0).
///
/// [pt] { 0x43cef0 } (thiscall, ret 0, bra_reg_sp.cpp): `BraRegSp::advance_stage()`, a fase seguinte do Paulista: no fim dos pontos corridos,
/// o mata-mata de 4 (0x43cf80); a caixa da linha 0x1ea.
pub(crate) unsafe extern "thiscall" fn bra_reg_pern_advance_stage_43cef0(this: usize) {
    state_league_advance_stage(this, 0x98b090, 0x1ea, 0x43cf80);
}

/// Comparer for `bra_reg_pern_start_new_season` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_pern_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_PERN_START_NEW_SEASON);
    compare_state_league_new_season("bra_reg_pern_start_new_season", this, || o(this))
}

/// Comparer for `bra_reg_pern_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_pern_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_PERN_ADVANCE_STAGE);
    compare_state_league_advance_stage("bra_reg_pern_advance_stage", this, || o(this))
}

/// Comparer for `bra_reg_pern_start_new_season_43bd10` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_pern_start_new_season_43bd10(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_PERN_START_NEW_SEASON_43BD10);
    compare_state_league_new_season("bra_reg_pern_start_new_season_43bd10", this, || o(this))
}

/// Comparer for `bra_reg_pern_advance_stage_43bec0` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_pern_advance_stage_43bec0(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_PERN_ADVANCE_STAGE_43BEC0);
    compare_state_league_advance_stage("bra_reg_pern_advance_stage_43bec0", this, || o(this))
}

/// Comparer for `bra_reg_pern_start_new_season_43cd40` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_pern_start_new_season_43cd40(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_PERN_START_NEW_SEASON_43CD40);
    compare_state_league_new_season("bra_reg_pern_start_new_season_43cd40", this, || o(this))
}

/// Comparer for `bra_reg_pern_advance_stage_43cef0` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_pern_advance_stage_43cef0(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_PERN_ADVANCE_STAGE_43CEF0);
    compare_state_league_advance_stage("bra_reg_pern_advance_stage_43cef0", this, || o(this))
}




/// Builds a knockout stage from table rows: allocates the team list, creates the round table (virtual +0x3c),
/// constructs a Cup_Stage in the first stage slot, sets up the round and frees the temporaries.
///
/// [pt] O mata-mata montado da tabela (o mesmo molde nas semifinais e nos mata-matas dos estaduais; só mudam o fonte, as
/// linhas da caixa e o arranjo): a lista dos `linhas.len()` times (malloc 0x944e46 de 4 × n; sem memória: a caixa
/// `linha_sem_lista` e nada mais) com os clubes das `linhas` da tabela, nesta ordem (as chaves: 0 × 1, 2 × 3...); a
/// virtual +0x3c (create_round_table: thiscall(this, fase 0, &rodadas, &extra, nulo), ret 0x10) monta a tabela de
/// rodadas; a fase nova (operator new 0x944cf1 de 0xb2 bytes; Cup_Stage::Cup_Stage 0x522e00, thiscall com 15
/// argumentos, ret 0x3c: a liga, n, a lista, as rodadas, +4, a tabela de rodadas, o ano +0x40, 0, 1, o extra da
/// virtual, 0x14, 0, 1, 0 e as sementes, um byte por time, que o construtor grava no +4 da lista da fase: na original,
/// a linha de cada time) vai para a
/// primeira vaga de fases ([+0xc]). Sem fase: a caixa `linha_sem_fase` e volta (sem liberar). Com ela:
/// Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4), a tabela de rodadas e a lista liberadas (free 0x9452ca,
/// nessa ordem) e +0x30 = 0. O quadro SEH da original só cobre o construtor (sem destrutor).
pub(crate) unsafe fn bra_reg_pern_build_knockout_from_table(this: usize, linhas: &[usize], sementes: &[u8], arquivo: usize, linha_sem_lista: u32, linha_sem_fase: u32, ramo: &'static str) {
    let n = linhas.len();
    // as sementes: a original grava os bytes na pilha antes de alocar (iguais às linhas, mas constantes à parte)
    let mut s = [0u8; 8];
    s[..sementes.len()].copy_from_slice(sementes);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let lista = alocar((n * 4) as u32);
    if lista == 0 {
        return show_error_at(arquivo, linha_sem_lista);
    }
    for (k, &l) in linhas.iter().enumerate() {
        core::ptr::write_unaligned((lista + 4 * k) as *mut usize, bra_reg_bahia_table_row_club(this, l));
    }
    crate::comparacao::ramo(ramo);
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 0, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let p = novo(0xb2);
    // ponytail: a original empurra as rodadas, o extra e o ano em dwords com lixo nos 16 bits altos (as saídas da
    // virtual são words na pilha; o ano vai no cx); o construtor só lê as palavras.
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construir(p, this, n as u32, lista, rodadas as u16 as u32, read_usize(this + 4), t, read_i16(this + 0x40) as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 1, 0, s.as_ptr())
    } else {
        0
    };
    core::ptr::write_unaligned(read_usize(this + 0xc) as *mut usize, fase);
    let fase = read_usize(read_usize(this + 0xc));
    if fase == 0 {
        return show_error_at(arquivo, linha_sem_fase);
    }
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_SET_UP_ROUND);
    montar(fase, 0);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    liberar(t);
    liberar(lista);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 0);
}

/// Pernambuco: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the
/// seed; the ties follow the seeds [inference: two legs, the decider at the better placed club])
/// Original: `BraRegPern::create_semi_final_stage` (0x439710, bra_reg_pern.cpp, thiscall, ret 0).
///
/// [pt] { 0x439710 } (thiscall, ret 0, bra_reg_pern.cpp): `BraRegPern::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98afd8, linhas 0x21b (sem lista) e 0x22f (sem fase).
pub(crate) unsafe extern "thiscall" fn bra_reg_pern_create_semi_final_stage(this: usize) {
    bra_reg_pern_build_knockout_from_table(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98afd8, 0x21b, 0x22f, "cqn_8 pernambucano"); // cqn8: 1º × 4º, 2º × 3º Pernambucano
}

/// Santa Catarina: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is
/// the seed; the ties follow the seeds [inference: two legs, the decider at the better placed club])
/// Original: `BraRegSanta::create_semi_final_stage` (0x43bf50, bra_reg_santa.cpp, thiscall, ret 0).
///
/// [pt] { 0x43bf50 } (thiscall, ret 0, bra_reg_santa.cpp): `BraRegSanta::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98b050, linhas 0x21b (sem lista) e 0x22f (sem fase).
pub(crate) unsafe extern "thiscall" fn bra_reg_pern_create_semi_final_stage_43bf50(this: usize) {
    bra_reg_pern_build_knockout_from_table(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98b050, 0x21b, 0x22f, "cqn_8 catarinense"); // cqn8: 1º × 4º, 2º × 3º Catarinense
}

/// Sao Paulo: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the
/// seed; the ties follow the seeds [inference: two legs, the decider at the better placed club])
/// Original: `BraRegSp::create_semi_final_stage` (0x43cf80, bra_reg_sp.cpp, thiscall, ret 0).
///
/// [pt] { 0x43cf80 } (thiscall, ret 0, bra_reg_sp.cpp): `BraRegSp::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98b090, linhas 0x20d (sem lista) e 0x221 (sem fase).
pub(crate) unsafe extern "thiscall" fn bra_reg_pern_create_semi_final_stage_43cf80(this: usize) {
    bra_reg_pern_build_knockout_from_table(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98b090, 0x20d, 0x221, "cqn_8 paulista"); // cqn8: 1º × 4º, 2º × 3º Paulista
}

/// Comparer: runs only the original and checks the new stage exists with n teams, the competition record and the
/// planned clubs (as a set), and +0x30 = 0.
///
/// [pt] O comparador dos três: a original aloca a lista, a tabela de rodadas e a fase e monta a rodada (só ela roda). O
/// plano, feito antes: os clubes das linhas. Depois, a fase da primeira vaga tem de existir, com +0x3e = n, +4 = o +4 da
/// liga e os n primeiros clubes da lista dela (+0xa7, 6 bytes) iguais aos do plano (como conjunto: o sorteio da rodada
/// 0x51c800 pode reordenar as chaves), e +0x30 = 0.
pub(crate) unsafe fn bra_reg_pern_compare_knockout_from_table(nome: &str, this: usize, linhas: &[usize], original: impl FnOnce()) {
    let mut plano: Vec<usize> = linhas.iter().map(|&l| bra_reg_bahia_table_row_club(this, l)).collect();
    original();
    let fase = read_usize(read_usize(this + 0xc));
    if fase == 0 {
        PLACAR.registrar(false, || format!("{}({:#x}) sem fase", nome, this));
        return;
    }
    let lista = read_usize(fase + 0xa7);
    let mut achou: Vec<usize> = (0..linhas.len()).map(|k| read_usize(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && read_i16(fase + 0x3e) as usize == linhas.len() && read_usize(fase + 4) == read_usize(this + 4) && read_usize(this + 0x30) == 0;
    PLACAR.registrar(igual, || format!("{}({:#x}) plano={:x?} fase={:#x} lista={:x?} +0x3e={}", nome, this, pl, fase, ac, read_i16(fase + 0x3e)));
}

/// Comparer for `bra_reg_pern_create_semi_final_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_pern_create_semi_final_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_PERN_CREATE_SEMI_FINAL_STAGE);
    bra_reg_pern_compare_knockout_from_table("bra_reg_pern_create_semi_final_stage", this, &[0, 3, 1, 2], || o(this))
}

/// Comparer for `bra_reg_pern_create_semi_final_stage_43bf50` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_pern_create_semi_final_stage_43bf50(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_PERN_CREATE_SEMI_FINAL_STAGE_43BF50);
    bra_reg_pern_compare_knockout_from_table("bra_reg_pern_create_semi_final_stage_43bf50", this, &[0, 3, 1, 2], || o(this))
}

/// Comparer for `bra_reg_pern_create_semi_final_stage_43cf80` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_pern_create_semi_final_stage_43cf80(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_PERN_CREATE_SEMI_FINAL_STAGE_43CF80);
    bra_reg_pern_compare_knockout_from_table("bra_reg_pern_create_semi_final_stage_43cf80", this, &[0, 3, 1, 2], || o(this))
}

/// The Pernambuco state league at career creation: the year and the database competition, disciplinary rule 7 and, with
/// no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table
/// with the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegPern::BraRegPern` (0x4388f0, bra_reg_pern.cpp, thiscall, ret 8).
///
/// [pt] { 0x4388f0 } (thiscall, ret 8, bra_reg_pern.cpp): `BraRegPern::BraRegPern(ano, registro)`: o Pernambucano (uma fase além da liga).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_pern(this: usize, ano: u32, registro: usize) -> usize {
    bra_reg_northeast_construct_state_league(this, ano, registro, &PERNAMBUCANO_STATE_CHAMPIONSHIP, 7, true, "cqt_3 pernambucano novo") // cqt3: a disciplina e as posições previstas do Pernambucano
}

/// Comparer for `construct_bra_reg_pern` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_bra_reg_pern(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_REG_PERN);
    bra_reg_northeast_compare_state_league_ctor("construct_bra_reg_pern", this, ano, registro, &PERNAMBUCANO_STATE_CHAMPIONSHIP, || o(this, ano, registro))
}

/// the Pernambuco league has 10 clubs: those with it as their reserve division in the database (the field that holds
/// the state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegPern::create_table` (0x439330, bra_reg_pern.cpp, thiscall, ret 0).
///
/// [pt] { 0x439330 } (thiscall, ret 0, bra_reg_pern.cpp): `BraRegPern::create_table()`: o Pernambucano tem 10 clubes (divisão reserva +0x60 = [0x9cf9bc]).
/// Fonte 0x98afd8, linha 0x167.
pub(crate) unsafe extern "thiscall" fn bra_reg_pern_create_table(this: usize) -> u32 {
    build_state_league_table(this, 0xa, 0x98afd8, 0x167, KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP, 0, "cqm_7 pernambucano cheio")
}

/// Comparer for `bra_reg_pern_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_pern_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_PERN_CREATE_TABLE);
    bra_reg_northeast_compare_state_league_table("bra_reg_pern_create_table", this, 0xa, KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP, || o(this))
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
    crate::comparacao::ramo("cqi_9 regras do pernambucano");
    create_main_schedule(this);
    1
}

/// Comparador: como o do Paranaense.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_pern_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_PERN_SET_UP_RULES)(this), || {
        pernambucano_rules(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("bra_reg_pern_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}
