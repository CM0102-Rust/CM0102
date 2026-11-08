use super::*;

// ---------- 279a leva: o Paranaense: a fase seguinte, a final entre os campeões dos dois turnos e o fim da temporada (bra_reg_parana.cpp) ----------




/// The Parana state league has two halves (stages 0 and 1, each a round-robin league) and, if needed, a final (stage
/// 2): the next stage moves the current stage on and, at the end of the second half, builds the final; past the last
/// stage it is an error (the box)
/// Original: `BraRegParana::advance_stage` (0x438230, bra_reg_parana.cpp, thiscall, ret 0).
///
/// [pt] { 0x438230 } (thiscall, ret 0, bra_reg_parana.cpp): `BraRegParana::advance_stage()`, a fase seguinte do Paranaense (dois turnos, as fases 0
/// e 1, cada um um League_Stage, e a final, a fase 2). Na última fase (+0x30 ≥ +0x2c − 1, com sinal): a caixa de
/// erro da linha 0x202 e nada mais. Senão +0x30 sobe um e, ao chegar a 2 (o fim do segundo turno), monta a final
/// (0x4382c0, call direto com o ecx da liga).
pub(crate) unsafe extern "thiscall" fn bra_reg_parana_advance_stage(this: usize) {
    let fase = read_usize(this + 0x30) as i32;
    if fase >= (read_usize(this + 0x2c) as i32).wrapping_sub(1) {
        return show_error_at(0x98af98, 0x202);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    if fase == 2 {
        let final_: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_BRAREGPARANA_CREATE_FINAL_STAGE);
        final_(this); // cqo8: a final depois do segundo turno
    }
}

/// The Parana final: the winners of the first and second halves meet over two games [inference: home and away], the
/// first-half winner as seed 0; if the same club wins both halves, it is champion without a final (the season ends with
/// the two stages and it gets status 5, champion, in the three tables)
/// Original: `BraRegParana::create_final_stage` (0x4382c0, bra_reg_parana.cpp, thiscall, ret 0).
///
/// [pt] { 0x4382c0 } (thiscall, ret 0, bra_reg_parana.cpp): `BraRegParana::create_final_stage()`, da fase seguinte quando o segundo turno
/// acaba. Os dois campeões dos turnos (o clube da primeira linha da tabela de cada League_Stage) vão para uma lista
/// de 2 (malloc 0x944e46 de 8; sem memória: a caixa da linha 0x226 e volta).
/// - O mesmo clube nos dois turnos: é o campeão sem final. +0x30 = 1, +0x2c = 2 (a temporada acaba com as duas fases),
///   a situação 5 (+0x37, campeão) na primeira linha da tabela de cada turno e na linha dele da tabela da liga
///   (+0xb1, as +0x3e linhas). A lista não é liberada (a original a deixa vazar; a cópia também).
/// - Campeões diferentes: a final, em dois jogos [inferência: ida e volta]. A virtual +0x3c (create_round_table:
///   thiscall(this, fase 2, &rodadas, &extra, 0), ret 0x10) monta a tabela de rodadas; a fase nova (operator new
///   0x944cf1 de 0xb2; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 2, a lista, as rodadas, +4, a tabela,
///   o ano +0x40, 2, 1, o extra, 0x14, 0, 1, 0 e as sementes {0, 1}: o campeão do primeiro turno é a semente 0) vai
///   para a terceira vaga das fases ([+0xc]+8; sem fase: a caixa da linha 0x250 e volta, sem liberar). Com ela:
///   Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4), a tabela e a lista liberadas (free 0x9452ca, nessa
///   ordem) e +0x30 = 2. O quadro SEH da original só cobre o construtor (sem destrutor).
pub(crate) unsafe extern "thiscall" fn bra_reg_parana_create_final_stage(this: usize) {
    let sementes: [u8; 4] = [0, 1, 0, 0];
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let lista = alocar(8);
    if lista == 0 {
        return show_error_at(0x98af98, 0x226);
    }
    let a = half_champion(this, 0);
    core::ptr::write_unaligned(lista as *mut usize, a);
    let b = half_champion(this, 1);
    core::ptr::write_unaligned((lista + 4) as *mut usize, b);
    if a == b {
        crate::comparacao::ramo("cqo_8 mesmo campeao");
        core::ptr::write_unaligned((this + 0x30) as *mut u32, 1);
        core::ptr::write_unaligned((this + 0x2c) as *mut u32, 2);
        for k in 0..2 {
            *((read_usize(read_usize(read_usize(this + 0xc) + 4 * k) + 0xb1) + 0x37) as *mut u8) = 5; // cqo8: campeão dos dois turnos
        }
        let mut i: i16 = 0;
        while i < read_i16(this + 0x3e) {
            let linha = read_usize(this + 0xb1) + i as usize * 0x3b;
            if read_usize(linha) == read_usize(lista) {
                *((linha + 0x37) as *mut u8) = 5;
            }
            i += 1;
        }
        // ponytail: a lista de 8 bytes fica sem liberar, como na original.
        return;
    }
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 2, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let p = novo(0xb2);
    // ponytail: a original empurra as rodadas, o extra e o ano em dwords com lixo nos 16 bits altos (as saídas da
    // virtual são words na pilha; o ano vai no dx); o construtor só lê as palavras.
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construir(p, this, 2, lista, rodadas as u16 as u32, read_usize(this + 4), t, read_i16(this + 0x40) as u16 as u32, 2, 1, extra as u16 as u32, 0x14, 0, 1, 0, sementes.as_ptr())
    } else {
        0
    };
    core::ptr::write_unaligned((read_usize(this + 0xc) + 8) as *mut usize, fase);
    let fase = read_usize(read_usize(this + 0xc) + 8);
    if fase == 0 {
        return show_error_at(0x98af98, 0x250);
    }
    crate::comparacao::ramo("cqo_8 final");
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_SET_UP_ROUND);
    montar(fase, 0);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    liberar(t);
    liberar(lista);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 2);
}

/// The end of the Parana season: with a final, the honours come from the final; without one, from the league, and the
/// champion gets 175,000 and the runner-up 100,000 in prize money (to the balance, with the prize money news). Then,
/// once the twelve state leagues are over, the champions go into the Champions Cup
/// Original: `BraRegParana::record_season_honours` (0x4384e0, bra_reg_parana.cpp, thiscall, ret 0).
///
/// [pt] { 0x4384e0 } (thiscall, ret 0, bra_reg_parana.cpp): `BraRegParana::record_season_honours()` (slot 13), o fim da temporada do Paranaense.
/// - Com final (+0x30 = 2): o histórico da final (o slot 13, +0x30, da fase [+0xc]+8) e, em salto final, a 0x42bba0
///   (`bra_nat_first_fill_champions_cup_entrants`, cdecl sem argumentos: quando os doze estaduais terminam, põe os
///   campeões na Copa dos Campeões); devolve o eax dela.
/// - Sem final (o mesmo campeão nos dois turnos): o histórico da liga (League::record_season_honours 0x684640,
///   thiscall, ret 0); os prêmios, 175.000 ao campeão (primeira linha da tabela +0xb1) e 100.000 ao vice (segunda linha),
///   cada um no saldo (0x5a0590 em [0xae2a38] -> as finanças do clube, thiscall, ret 4; 0x5999a0 Finance::add_prize_money,
///   thiscall, ret 4) e na notícia do prêmio (0x48e3e0 Comp::send_prize_money_news: thiscall(this, clube, valor em 64
///   bits, posição 1 ou 2, rodada 0, partida 0, tipo −2), ret 0x1c); depois a 0x42bba0, como acima.
pub(crate) unsafe extern "thiscall" fn bra_reg_parana_record_season_honours(this: usize) -> u32 {
    let campeoes: unsafe extern "C" fn() -> u32 = core::mem::transmute(FN_BRA_NAT_FIRST_FILL_CHAMPIONS_CUP_ENTRANTS);
    if read_usize(this + 0x30) == 2 {
        let f = read_usize(read_usize(this + 0xc) + 8);
        let historico: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(f) + 0x30));
        historico(f);
        crate::comparacao::ramo("cqo_8 historico da final");
        return campeoes();
    }
    let historico: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_RECORD_SEASON_HONOURS);
    historico(this);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let premio: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_FINANCE_ADD_PRIZE_MONEY);
    let noticia: unsafe extern "thiscall" fn(usize, usize, i32, i32, i32, u32, usize, i32) = core::mem::transmute(FN_COMP_SEND_PRIZE_MONEY_NEWS);
    for (linha, valor, posicao) in [(0usize, 175000i32, 1i32), (0x3b, 100000, 2)] {
        let c = read_usize(read_usize(this + 0xb1) + linha);
        premio(financas(read_usize(G_FINANCE_TABLE), c), valor);
        noticia(this, read_usize(read_usize(this + 0xb1) + linha), valor, 0, posicao, 0, 0, -2);
    }
    crate::comparacao::ramo("cqo_8 premios");
    campeoes()
}

/// O comparador da fase seguinte: só a original (ponytail: ao chegar a 2 monta a final, que aloca e sorteia; não pode
/// rodar duas vezes), conferindo o plano: na última fase, +0x30 igual; senão +0x30 + 1, ou, ao chegar a 2, 2 (com
/// final) ou 1 (o mesmo campeão nos dois turnos, que a final grava).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_parana_advance_stage(this: usize) {
    let antes = read_usize(this + 0x30) as i32;
    let ultima = antes >= (read_usize(this + 0x2c) as i32).wrapping_sub(1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_PARANA_ADVANCE_STAGE)(this);
    let depois = read_usize(this + 0x30) as i32;
    let igual = if ultima { depois == antes } else if antes + 1 == 2 { depois == 2 || depois == 1 } else { depois == antes + 1 };
    PLACAR.registrar(igual, || format!("bra_reg_parana_advance_stage({:#x}) antes={} depois={}", this, antes, depois));
}

/// O comparador da final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), conferindo o plano feito
/// antes: os campeões dos turnos. O mesmo nos dois: +0x30 = 1, +0x2c = 2 e a situação 5 na primeira linha de cada turno
/// e na linha dele da tabela da liga. Diferentes: a fase da terceira vaga existe, com +0x3e = 2, +4 = o +4 da liga e os
/// dois campeões na lista dela (+0xa7, 6 bytes; como conjunto: o sorteio da rodada pode reordenar), e +0x30 = 2.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_parana_create_final_stage(this: usize) {
    let (a, b) = (half_champion(this, 0), half_champion(this, 1));
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_PARANA_CREATE_FINAL_STAGE)(this);
    let igual = if a == b {
        let marca = |k: usize| *((read_usize(read_usize(read_usize(this + 0xc) + 4 * k) + 0xb1) + 0x37) as *const u8) == 5;
        let na_liga = (0..read_i16(this + 0x3e).max(0) as usize).all(|i| {
            let linha = read_usize(this + 0xb1) + i * 0x3b;
            read_usize(linha) != a || *((linha + 0x37) as *const u8) == 5
        });
        read_usize(this + 0x30) == 1 && read_usize(this + 0x2c) == 2 && marca(0) && marca(1) && na_liga
    } else {
        let fase = read_usize(read_usize(this + 0xc) + 8);
        if fase == 0 {
            false
        } else {
            let l = read_usize(fase + 0xa7);
            let (x, y) = (read_usize(l), read_usize(l + 6));
            read_i16(fase + 0x3e) == 2 && read_usize(fase + 4) == read_usize(this + 4) && ((x, y) == (a, b) || (x, y) == (b, a)) && read_usize(this + 0x30) == 2
        }
    };
    PLACAR.registrar(igual, || format!("bra_reg_parana_create_final_stage({:#x}) campeoes={:#x},{:#x} +0x30={} +0x2c={}", this, a, b, read_usize(this + 0x30), read_usize(this + 0x2c)));
}

/// Comparador do fim da temporada: só a original (ponytail: grava o histórico, que aloca o registro, paga os prêmios e
/// manda a notícia, e pode montar a Copa dos Campeões; não pode rodar duas vezes). A cópia roda no modo Rust (contadores
/// "cqo_8 historico da final" e "cqo_8 premios") e é provada pelas mutações e pelas duas carreiras Rust.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_parana_record_season_honours(this: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_PARANA_RECORD_SEASON_HONOURS)(this)
}

// ---------- 274a leva: as regras dos estaduais Paranaense, Pernambucano (Norte), Carioca e Mineiro (bra_reg_parana.cpp, bra_reg_pern.cpp, bra_reg_rio.cpp) ----------




/// Writes the rule set common to these Brazilian state leagues (two rounds, 3 pts win, 1 draw, tiebreakers, 7 subs, 3
/// substitutions), with top places (+0xbe) and relegated (+0xc1) as parameters.
///
/// [pt] As regras comuns destes estaduais (as três rotinas gravam o mesmo conjunto; mudam as vagas de cima +0xbe e os
/// rebaixados +0xc1; os três têm dois turnos): vitória 3, empate 1, categoria 2 (liga), clube nacional, desempates
/// confronto direto e saldo, sem play-offs na tabela, sem liga acima nem abaixo, palavra de regras 8, 7 reservas e
/// 3 substituições, e as palavras +0xab e +0xdb em 0.
pub(crate) unsafe fn bra_reg_parana_set_state_league_rules(o: usize, cima: u8, rebaixados: u8) {
    write_u16_at(o, 0x3c, 2);
    write_u8_at(o, 0xc4, 2);
    write_u16_at(o, 0xab, 0);
    write_u16_at(o, 0xdb, 0);
    write_u8_at(o, 0xc7, 0);
    write_u8_at(o, 0xbe, cima);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc2, 3);
    write_u8_at(o, 0xc5, 3);
    write_u8_at(o, 0x4a, 3);
    write_u8_at(o, 0xc3, 1);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc6, 1);
    write_u8_at(o, 0xc1, rebaixados);
    write_u32_at(o, 0x1c, 0xffff_ffff);
    write_u32_at(o, 0x20, 0xffff_ffff);
    write_u8_at(o, 0x49, 7);
    write_u16_at(o, 0xd9, 8);
}

/// State league rules for Paranaense (0x437fc0): no top places, 4 relegated.
///
/// [pt] O Paranaense (0x437fc0): nenhuma vaga de cima na tabela geral (os turnos têm estágios próprios), 4 rebaixados.
pub(crate) unsafe fn parana_rules(o: usize) {
    bra_reg_parana_set_state_league_rules(o, 0, 4); // cqi9: o Paranaense
}

/// State league rules for Pernambucano (0x439430, also Norte): 4 top places, 2 relegated.
///
/// [pt] O Pernambucano (0x439430, também o Norte): 4 vagas de cima, 2 rebaixados.
pub(crate) unsafe fn pernambucano_rules(o: usize) {
    bra_reg_parana_set_state_league_rules(o, 4, 2); // cqi9: o Pernambucano
}

/// State league rules for Carioca (0x43a600, also Mineiro): no top places, 2 relegated.
///
/// [pt] O Carioca (0x43a600, também o Mineiro): nenhuma vaga de cima na tabela geral, 2 rebaixados.
pub(crate) unsafe fn carioca_rules(o: usize) {
    bra_reg_parana_set_state_league_rules(o, 0, 2); // cqi9: o Carioca
}



/// Parana: no top places in the overall table (each half is a league stage of its own, created by add_result 0x4386e0,
/// and decides the title [inference]); 4 relegated; double round robin; win 3, draw 1; tie-break by head-to-head, then
/// goal difference; no play-offs in the table; no league above or below (the relegated go to the non-playable pool
/// [inference]); rule word 8 (the top places lead to the next stage; no champion band); 7 bench substitutes and 3
/// substitutions; category 2 (league) matches. Then the main stage schedule
/// Original: `BraRegParana::set_up_rules` (0x437fc0, bra_reg_parana.cpp, thiscall).
///
/// [pt] { 0x437fc0 } (thiscall, bra_reg_parana.cpp): `BraRegParana::set_up_rules` (slot 36), as regras e o calendário;
/// devolve 1.
pub(crate) unsafe extern "thiscall" fn bra_reg_parana_set_up_rules(this: usize) -> u32 {
    parana_rules(this);
    crate::comparacao::ramo("cqi_9 regras do paranaense");
    create_main_schedule(this);
    1
}

/// Comparador: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca), sobre o objeto menos as
/// saídas do calendário; a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_parana_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_PARANA_SET_UP_RULES)(this), || {
        parana_rules(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("bra_reg_parana_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// The Parana state league at career creation: the year and the database competition, disciplinary rule 7 and, with no
/// saved state, importance 15, two halves and the final (three stages besides the league), the rules, the table with
/// the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegParana::BraRegParana` (0x437400, bra_reg_parana.cpp, thiscall, ret 8).
///
/// [pt] { 0x437400 } (thiscall, ret 8, bra_reg_parana.cpp): `BraRegParana::BraRegParana(ano, registro)`: o Paranaense (três fases além da liga).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_parana(this: usize, ano: u32, registro: usize) -> usize {
    bra_reg_northeast_construct_state_league(this, ano, registro, &CQT3_PARANAENSE, 7, true, "cqt_3 paranaense novo") // cqt3: a disciplina e as posições previstas do Paranaense
}

pub(crate) unsafe extern "thiscall" fn comparar_construct_bra_reg_parana(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_REG_PARANA);
    bra_reg_northeast_compare_state_league_ctor("construct_bra_reg_parana", this, ano, registro, &CQT3_PARANAENSE, || o(this, ano, registro))
}

/// the Parana league has 12 clubs: those with it as their reserve division in the database (the field that holds the
/// state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegParana::create_table` (0x437ec0, bra_reg_parana.cpp, thiscall, ret 0).
///
/// [pt] { 0x437ec0 } (thiscall, ret 0, bra_reg_parana.cpp): `BraRegParana::create_table()`: o Paranaense tem 12 clubes (divisão reserva +0x60 = [0x9cf9d0]).
/// Fonte 0x98af98, linha 0x169.
pub(crate) unsafe extern "thiscall" fn bra_reg_parana_create_table(this: usize) -> u32 {
    build_state_league_table(this, 0xc, 0x98af98, 0x169, KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, 0, "cqm_7 paranaense cheio")
}

pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_parana_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_PARANA_CREATE_TABLE);
    bra_reg_northeast_compare_state_league_table("bra_reg_parana_create_table", this, 0xc, KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, || o(this))
}

/// One half of the Parana state league: when the first game of a half is processed, it builds that half's round-robin
/// stage (a league with all the clubs of the table, the same points for a win and a draw and the same tie-breakers as
/// the league, and one top place, the half winner's); the first half goes to stage 0 and the second (from round 12, by
/// the match +0x34) to stage 1; each half winner (the top of the stage table) goes to the final
/// Original: `BraRegParana::create_half_stage` (0x438760, bra_reg_parana.cpp, thiscall, ret 4).
///
/// [pt] { 0x438760 } (thiscall, ret 4, bra_reg_parana.cpp): `BraRegParana::create_half_stage(k)`, um turno do Paranaense (a
/// caixa da linha 0x3d5).
pub(crate) unsafe extern "thiscall" fn create_half_stage(this: usize, k: u32) {
    create_half_season_stage(this, k, 0x98af98, 0x3d5, 0xb, 0xc2, "cqr_1 turno do paranaense") // cqr1: os pontos por vitória no Paranaense
}

pub(crate) unsafe extern "thiscall" fn comparar_create_half_stage(this: usize, k: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_CREATE_HALF_STAGE);
    compare_half_season_stage("create_half_stage", this, k, || o(this, k))
}

/// The Parana state league turnover: archives the season in the statistics, clears the table, the schedule and the
/// stages (the two halves and the final) and prepares three new stage slots; moves to the next year and rebuilds the
/// rules, the table with the twelve state clubs, the fixtures and the order; then the predicted positions and the
/// referee renewal. The halves are only built at each one's first game (cqr_1)
/// Original: `BraRegParana::start_new_season` (0x438060, bra_reg_parana.cpp, thiscall, ret 0).
///
/// [pt] { 0x438060 } (thiscall, ret 0, bra_reg_parana.cpp): `BraRegParana::start_new_season()`, a virada do Paranaense (a tabela: 0x437ec0).
pub(crate) unsafe extern "thiscall" fn bra_reg_parana_start_new_season(this: usize) -> u32 {
    start_new_season_two_halves(this, 0x437ec0, 0x98af98, 1, "cqr_2 virada do paranaense") // cqr2: o ano seguinte do Paranaense
}

pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_parana_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_PARANA_START_NEW_SEASON);
    compare_new_season("bra_reg_parana_start_new_season", this, || o(this))
}
