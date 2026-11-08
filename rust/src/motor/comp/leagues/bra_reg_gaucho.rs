use super::*;

// ---------- 278a leva: o mata-mata dos estaduais e regionais: Gaúcho, Centro-Oeste e Goiano (bra_reg_gaucho.cpp, bra_reg_central.cpp, bra_reg_goias.cpp) ----------




/// O mata-mata montado da tabela (o mesmo molde nas semifinais e nos mata-matas dos estaduais; só mudam o fonte, as
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
pub(crate) unsafe fn cqn9_mata_mata(this: usize, linhas: &[usize], sementes: &[u8], arquivo: usize, linha_sem_lista: u32, linha_sem_fase: u32, ramo: &'static str) {
    let n = linhas.len();
    // as sementes: a original grava os bytes na pilha antes de alocar (iguais às linhas, mas constantes à parte)
    let mut s = [0u8; 8];
    s[..sementes.len()].copy_from_slice(sementes);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let lista = alocar((n * 4) as u32);
    if lista == 0 {
        return aviso_em(arquivo, linha_sem_lista);
    }
    for (k, &l) in linhas.iter().enumerate() {
        core::ptr::write_unaligned((lista + 4 * k) as *mut usize, cqn7_clube(this, l));
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
        return aviso_em(arquivo, linha_sem_fase);
    }
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_SET_UP_ROUND);
    montar(fase, 0);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    liberar(t);
    liberar(lista);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 0);
}

/// Rio Grande do Sul: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed
/// is the seed; the ties follow the seeds [inference: two legs, the decider at the better placed club])
/// Original: `BraRegGaucho::create_knockout_stage` (0x4325b0, bra_reg_gaucho.cpp, thiscall, ret 0).
///
/// [pt] { 0x4325b0 } (thiscall, ret 0, bra_reg_gaucho.cpp): `BraRegGaucho::create_knockout_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98ae50, linhas 0x217 (sem lista) e 0x22b (sem fase).
pub(crate) unsafe extern "thiscall" fn cqn9_mata_mata_do_gaucho(this: usize) {
    cqn9_mata_mata(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98ae50, 0x217, 0x22b, "cqn_9 gaucho"); // cqn9: 1º × 4º, 2º × 3º Gaúcho
}



/// O comparador dos três: a original aloca a lista, a tabela de rodadas e a fase e monta a rodada (só ela roda). O
/// plano, feito antes: os clubes das linhas. Depois, a fase da primeira vaga tem de existir, com +0x3e = n, +4 = o +4 da
/// liga e os n primeiros clubes da lista dela (+0xa7, 6 bytes) iguais aos do plano (como conjunto: o sorteio da rodada
/// 0x51c800 pode reordenar as chaves), e +0x30 = 0.
pub(crate) unsafe fn cqn9_comparar(nome: &str, this: usize, linhas: &[usize], original: impl FnOnce()) {
    let mut plano: Vec<usize> = linhas.iter().map(|&l| cqn7_clube(this, l)).collect();
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

pub(crate) unsafe extern "thiscall" fn comparar_cqn9_mata_mata_do_gaucho(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1849);
    cqn9_comparar("cqn9_mata_mata_do_gaucho", this, &[0, 3, 1, 2], || o(this))
}

/// end of a state league (nine of them): records the honours of the first stage (the table) and calls 0x42bba0, which,
/// once the twelve state leagues are over, puts the champions into the Champions Cup
/// Original: `BraRegGaucho::record_season_honours` (0x432970, bra_reg_gaucho.cpp, thiscall).
///
/// [pt] { 0x432970 } (thiscall, bra_reg_gaucho.cpp): `BraRegGaucho::record_season_honours` (slot 13 de nove estaduais: Baiano,
/// Centro-Oeste, Gaúcho, Goiano, Norte, Nordeste, Pernambucano, Catarinense e Paulista): o histórico do primeiro estágio
/// e, em salto final, a 0x42bba0 (`bra_nat_first_fill_champions_cup_entrants`, cdecl sem argumentos: quando os doze
/// estaduais terminam, põe os campeões na Copa dos Campeões); devolve o eax dela.
pub(crate) unsafe extern "thiscall" fn cqj8_historico_do_estadual(this: usize) -> u32 {
    cqj8_historico_do_estagio(this); // cqj8: o histórico do estadual
    crate::comparacao::ramo("cqj_8 historico do estadual");
    let campeoes: unsafe extern "C" fn() -> u32 = core::mem::transmute(FN_BRA_NAT_FIRST_FILL_CHAMPIONS_CUP_ENTRANTS);
    campeoes()
}

/// Comparador: só a original (ponytail: grava o histórico, aloca o registro dele e pode montar a Copa dos Campeões; não
/// pode rodar duas vezes). A cópia roda no modo Rust (contador "cqj_8 historico do estadual") e é provada pelas
/// mutações e pelas duas carreiras Rust.
pub(crate) unsafe extern "thiscall" fn comparar_cqj8_historico_do_estadual(this: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize) -> u32>(1726)(this)
}

/// The Rio Grande do Sul state league at career creation: the year and the database competition, disciplinary rule 7
/// and, with no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules,
/// the table with the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegGaucho::BraRegGaucho` (0x4317c0, bra_reg_gaucho.cpp, thiscall, ret 8).
///
/// [pt] { 0x4317c0 } (thiscall, ret 8, bra_reg_gaucho.cpp): `BraRegGaucho::BraRegGaucho(ano, registro)`: o Gaúcho (uma fase além da liga).
pub(crate) unsafe extern "thiscall" fn cqt1_construir_gaucho(this: usize, ano: u32, registro: usize) -> usize {
    cqt1_construir(this, ano, registro, &CQT1_GAUCHO, 7, true, "cqt_1 gaucho novo") // cqt1: a disciplina e as posições previstas do Gaúcho
}

pub(crate) unsafe extern "thiscall" fn comparar_cqt1_construir_gaucho(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2048);
    cqt1_comparar("cqt1_construir_gaucho", this, ano, registro, &CQT1_GAUCHO, || o(this, ano, registro))
}

/// The Rio Grande do Sul turnover: archives the season in the statistics, clears the previous table and stages, moves
/// to the next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table
/// order; then the predicted positions and the renewal of the state referees
/// Original: `BraRegGaucho::start_new_season` (0x432370, bra_reg_gaucho.cpp, thiscall, ret 0).
///
/// [pt] { 0x432370 } (thiscall, ret 0, bra_reg_gaucho.cpp): `BraRegGaucho::start_new_season()`, a virada da temporada do Gaúcho (a tabela:
/// 0x4321d0; caixas nas linhas 0x1c6, 0x1cd, 0x1d3 e 0x1da).
pub(crate) unsafe extern "thiscall" fn cqp7_virada_do_gaucho(this: usize) -> u32 {
    cqp7_virada(this, 0x4321d0, 0x98ae50, [0x1c6, 0x1cd, 0x1d3, 0x1da], 1, "cqp_7 virada do gaucho") // cqp7: o ano seguinte do Gaúcho
}

pub(crate) unsafe extern "thiscall" fn comparar_cqp7_virada_do_gaucho(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1930);
    cqp7_comparar_virada("cqp7_virada_do_gaucho", this, || o(this))
}

/// The Rio Grande do Sul next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to
/// the semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box)
/// Original: `BraRegGaucho::advance_stage` (0x432520, bra_reg_gaucho.cpp, thiscall, ret 0).
///
/// [pt] { 0x432520 } (thiscall, ret 0, bra_reg_gaucho.cpp): `BraRegGaucho::advance_stage()`, a fase seguinte do Gaúcho: no fim dos pontos corridos,
/// o mata-mata de 4 (0x4325b0); a caixa da linha 0x1f4.
pub(crate) unsafe extern "thiscall" fn cqp7_fase_seguinte_do_gaucho(this: usize) {
    cqp7_fase_seguinte(this, 0x98ae50, 0x1f4, 0x4325b0);
}

pub(crate) unsafe extern "thiscall" fn comparar_cqp7_fase_seguinte_do_gaucho(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1931);
    cqp7_comparar_fase_seguinte("cqp7_fase_seguinte_do_gaucho", this, || o(this))
}

/// the Rio Grande do Sul league has 17 clubs: those with it as their reserve division in the database (the field that
/// holds the state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegGaucho::load_region_clubs` (0x4321d0, bra_reg_gaucho.cpp, thiscall, ret 0).
///
/// [pt] { 0x4321d0 } (thiscall, ret 0, bra_reg_gaucho.cpp): `BraRegGaucho::load_region_clubs()`: o Gaúcho tem 17 clubes (divisão reserva +0x60 = [0x9cf9c8]).
/// Fonte 0x98ae50, linha 0x163.
pub(crate) unsafe extern "thiscall" fn cql8_tabela_do_gaucho(this: usize) -> u32 {
    cql8_tabela(this, 0x11, 0x98ae50, 0x163, KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP, 0, "cql_8 gaucho cheio")
}

pub(crate) unsafe extern "thiscall" fn comparar_cql8_tabela_do_gaucho(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1830);
    cql8_comparar("cql8_tabela_do_gaucho", this, 0x11, KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP, || o(this))
}

/// Gaucho, Northeast Cup and Paulista (the same routine in the three vtables): single round robin; 4 top places (the
/// 4-club knockout); 2 relegated; win 3, draw 1; tie-break by head-to-head, then goal difference; no play-offs in the
/// table; no league above or below (the relegated go to the non-playable pool [inference]); rule word 8 (the top places
/// lead to the next stage; no champion band: the champion comes from the final stage); 7 bench substitutes and 3
/// substitutions; category 2 (league) matches. Then the main stage schedule
/// Original: `BraRegGaucho::set_up_rules` (0x4322d0, bra_reg_gaucho.cpp, thiscall).
///
/// [pt] { 0x4322d0 } (thiscall, bra_reg_gaucho.cpp): `BraRegGaucho::set_up_rules`, nas vtables do Gaúcho (0x96811c), do
/// Nordeste (0x9683ec) e do Paulista (0x968770); devolve 1.
pub(crate) unsafe extern "thiscall" fn cqi8_regras_do_gaucho(this: usize) -> u32 {
    cqi8_regras_gaucho(this);
    crate::comparacao::ramo("cqi_8 regras do gaucho");
    cqi7_calendario(this);
    1
}

/// Comparador: como o do Centro-Oeste.
pub(crate) unsafe extern "thiscall" fn comparar_cqi8_regras_do_gaucho(this: usize) -> u32 {
    let reg = cqi7_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1716)(this), || {
        cqi8_regras_gaucho(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqi8_regras_do_gaucho({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}
