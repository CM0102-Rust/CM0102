use super::*;

// The comparers of the routines in ../comp/leagues/fin_prm.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/fin_prm.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `fin_prm_end_season_for_nation` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador do fim de temporada da nação: só a original, porque ela chama o fim de temporada de cada clube
/// (0x59cfa0), que sorteia e entrega notícias ao técnico humano (a cópia segue no modo Rust, com o contador). Confere o
/// que a rotina garante e nenhuma chamada desfaz: o último clube da entrada (a ordem dos clubes) fica com os totais do
/// ano (+0x110, 0x50 bytes) zerados. Ponytail: os anteriores podem ganhar valores pela 0x59cfa0 de um clube seguinte
/// (o B que usa o registro do A, o estádio dividido); conferir todos pede o plano dessas exceções.
pub(crate) unsafe extern "thiscall" fn comparar_fin_prm_end_season_for_nation(this: usize, entrada: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FIN_PRM_END_SEASON_FOR_NATION);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    if entrada == 0 || depois(G_CURRENT_DATE, read_usize(G_GAME_START_DATE) as u32, read_usize(G_GAME_START_DATE_FLAGS) as u32) != 0 {
        return o(this, entrada);
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let linha: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_CLUB);
    let mut ultimo = None;
    let mut clube = read_usize(G_CLUB_TABLE);
    for k in 0..read_i32(G_CLUB_COUNT).max(0) as usize {
        if selecao(clube) != 0 {
            break;
        }
        let nacao = read_usize(clube + club::NATION);
        if read_usize(clube + club::DIVISION) != 0 && nacao != 0 && (read_u8(nacao + nation::NATION_LEAGUE_SELECTED) != 0 || read_i16(clube + club::REPUTATION) >= 0x1388) && (0xb63d60 + (linha(clube) as i8 as i32) * 72) as usize == entrada {
            ultimo = Some(k);
        }
        clube += 0x245;
    }
    o(this, entrada);
    if let Some(k) = ultimo {
        let f = read_usize(this) + k * 0x167 + 0x110;
        let zerado = core::slice::from_raw_parts(f as *const u8, 0x50).iter().all(|&x| x == 0);
        PLACAR.registrar(zerado, || format!("fin_prm_end_season_for_nation(entrada {:#x}) clube {} sem zerar os totais do ano", entrada, k));
    }
}

/// Comparer for `season_end_club_update` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador do fim de temporada do clube: lado a lado (a cópia sem contar os sorteios) nas finanças do parceiro e do
/// próprio clube, no clube, no estádio (78 bytes), na linha dos pedidos (0x49 bytes) e no estado do sorteio. As notícias
/// (0x59ffb0, 0x59fae0, 0x59fe10) são montadas na pilha e só são entregues ao técnico humano (0x7861e0): com técnico da
/// máquina elas não deixam nada além do sorteio(2) da recusa, que as duas rodadas fazem. Só a original: o técnico
/// humano (a notícia entra na caixa dele) e o estádio dividido (mexe noutro clube e nas finanças dele).
pub(crate) unsafe extern "thiscall" fn comparar_season_end_club_update(this: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SEASON_END_CLUB_UPDATE);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if clube == 0 || selecao(clube) != 0 || fin_prm_has_human_manager(clube) {
        return o(this, clube);
    }
    let mut a = 0u32;
    let fin = finance_of_parent_club(this, clube, &mut a);
    let estadio = read_usize(clube + club::STADIUM);
    if fin == 0 || (estadio != 0 && read_u8(clube + club::OWN_STADIUM) == 0 && read_usize(clube + club::DIVISION) != 0 && read_i32(fin) > 0x2160ec0) {
        return o(this, clube); // simplification: a compra do estádio pode mexer no dono; só a original
    }
    let proprio = read_usize(this).wrapping_add((read_i32(clube) as u32).wrapping_mul(0x167) as usize);
    let mut reg = vec![(fin, 0x167), (clube, 0x245), (board_requests_row(clube), 0x49)];
    if proprio != fin {
        reg.push((proprio, 0x167));
    }
    if estadio != 0 {
        reg.push((estadio, 0x4e));
    }
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube), || crate::aleatorio::sem_contar(|| season_end_club_update(this, clube)));
    PLACAR.registrar(igual, || format!("season_end_club_update(clube {:x}) original={:x?} rust={:x?}", clube, mo, mr));
}

/// Comparer for `news_dividend` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da notícia do dividendo: só a original (simplification: a notícia vai para a caixa do técnico humano e não
/// pode ser entregue duas vezes; com técnico da máquina não deixa nada).
pub(crate) unsafe extern "thiscall" fn comparar_news_dividend(this: usize, clube: usize, valor: i32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32)>(IDX_NEWS_DIVIDEND)(this, clube, valor)
}

/// Comparer for `get_max_wage` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador do teto salarial: pela devolução (a rotina só lê; as chamadas de dentro também).
pub(crate) unsafe extern "thiscall" fn comparar_get_max_wage(fin: usize, pessoa: usize, cargo: u32) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(IDX_GET_MAX_WAGE)(fin, pessoa, cargo);
    let n = get_max_wage(fin, pessoa, cargo);
    PLACAR.registrar(r == n, || format!("get_max_wage(fin {:x}, pessoa {:x}, cargo {}) original={} rust={}", fin, pessoa, cargo as u8 as i8, r, n));
    r
}

/// Comparer for `news_ground_works` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da notícia das obras: só a original (simplification: a notícia vai para a caixa do técnico humano e não pode
/// ser entregue duas vezes; com técnico da máquina não deixa nada).
pub(crate) unsafe extern "thiscall" fn comparar_news_ground_works(this: usize, clube: usize, valor: u32, tipo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(IDX_NEWS_GROUND_WORKS)(this, clube, valor, tipo)
}

/// Comparer for `news_request_refused` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da notícia da obra recusada: só a original (simplification: entrega ao técnico humano e sorteia; não pode
/// rodar duas vezes).
pub(crate) unsafe extern "thiscall" fn comparar_news_request_refused(this: usize, clube: usize, assunto: u32, motivo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(IDX_NEWS_REQUEST_REFUSED)(this, clube, assunto, motivo)
}

/// Comparer for `save_finance_dat` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x59b3b0: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_save_finance_dat(this: usize, save: usize) {
    // simplification: grava no contêiner do save (não pode rodar duas vezes); a cópia é provada pela prova do save
    // (o .qst gravado no modo rust com e sem a leva, item a item)
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SAVE_FINANCE_DAT)(this, save)
}

/// Comparer for `load_from_save` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x59b2a0: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_load_from_save(this: usize, save: usize) -> u32 {
    // simplification: lê do contêiner (a posição do item anda; não pode rodar duas vezes); a cópia é provada pela carga
    // do save de férias na prova do save (levas/N/CARGA)
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_LOAD_FROM_SAVE)(this, save)
}

/// Comparer for `process_match_income` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x59a5d0: lado a lado nos dois registros de finanças e no estado do sorteio (os sorteios de 10, dos
/// gastos e da manutenção). Sem resultado, sem partida, sem um dos clubes ou com seleção: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_process_match_income(tabela: usize, partida: usize, resultado: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_PROCESS_MATCH_INCOME);
    if resultado == 0 || partida == 0 || read_usize(partida + 0x1c) == 0 || read_usize(partida + 0x20) == 0 {
        return o(tabela, partida, resultado);
    }
    let (Some(fm), Some(fv)) = (match_club_finance(tabela, partida, 0x1c), match_club_finance(tabela, partida, 0x20)) else {
        return o(tabela, partida, resultado); // seleção: a original não faz nada
    };
    if fm == fv {
        return o(tabela, partida, resultado); // simplification: o mesmo registro nos dois lados, só a original
    }
    let mut reg = vec![(fm, 0x167), (fv, 0x167)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(tabela, partida, resultado), || crate::aleatorio::sem_contar(|| process_match_income(tabela, partida, resultado)));
    PLACAR.registrar(igual, || format!("process_match_income(partida {:x}) original={:x?} rust={:x?}", partida, mo, mr));
}

/// Comparer for `calc_gate_receipts` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x59af40: a devolução (só lê; a 0x540a50 pode zerar a marca +0x244 do clube, igual nas duas). Sem
/// partida: só a original (a caixa).
pub(crate) unsafe extern "thiscall" fn comparar_calc_gate_receipts(tabela: usize, partida: usize, publico: i32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32) -> i32>(IDX_CALC_GATE_RECEIPTS);
    if partida == 0 {
        return o(tabela, partida, publico);
    }
    let r = o(tabela, partida, publico);
    let n = calc_gate_receipts(tabela, partida, publico);
    PLACAR.registrar(r == n, || format!("calc_gate_receipts(partida {:x}, publico {}) original={} rust={}", partida, publico, r, n));
    r
}

/// Comparer for `pay_league_fine` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x599850: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_pay_league_fine(this: usize, valor: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAY_LEAGUE_FINE);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pay_league_fine(this, valor));
    PLACAR.registrar(igual, || format!("pay_league_fine(valor {}) original={:x?} rust={:x?}", valor, mo, mr));
}

/// Comparer for `try_finance_boost_event` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_try_finance_boost_event(this: usize, clube: usize, fin: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_TRY_FINANCE_BOOST_EVENT);
    let pres = if clube != 0 { read_usize(clube + club::CHAIRMAN) } else { 0 };
    let ficha = if pres != 0 { read_usize(pres + 0x69) } else { 0 };
    if clube == 0 || fin == 0 {
        return o(this, clube, fin); // simplification: sem clube ou sem finanças, só a original
    }
    let mut extras = vec![(clube + club::PLC, 1)];
    let mut conferir = Vec::new();
    if ficha != 0 {
        extras.push((ficha + 0x20, 1)); // o +0x20 da ficha cai em todo evento com ela
        conferir.push(1);
    }
    compare_rare_finance_event("try_finance_boost_event", fin, &extras, &conferir, || o(this, clube, fin), || try_finance_boost_event(this, clube, fin))
}

/// Comparer for `try_board_takeover` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_try_board_takeover(this: usize, clube: usize, fin: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_TRY_BOARD_TAKEOVER);
    if clube == 0 || fin == 0 {
        return o(this, clube, fin); // simplification: sem clube ou sem finanças, só a original
    }
    compare_rare_finance_event("try_board_takeover", fin, &[], &[], || o(this, clube, fin), || try_board_takeover(this, clube, fin))
}

/// Comparer for `pay_other_costs` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5999e0: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_pay_other_costs(this: usize, valor: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAY_OTHER_COSTS);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pay_other_costs(this, valor));
    PLACAR.registrar(igual, || format!("pay_other_costs(valor {}) original={:x?} rust={:x?}", valor, mo, mr));
}

/// Comparer for `update_wage_bill` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado na folha (+0xc), no maior salário (+0x10), no teto (+0x14) e no último contrato consultado
/// ([0xae1080], que o salário padrão 0x864550 grava); as outras chamadas (contratos, teto 0x5968d0) só leem.
pub(crate) unsafe extern "thiscall" fn comparar_update_wage_bill(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_UPDATE_WAGE_BILL);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0xc, 12), (G_CONTRACT_LOOKUP_RESULT, 4)], || o(this), || update_wage_bill(this));
    PLACAR.registrar(igual, || format!("update_wage_bill(fin {:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// Comparer for `refresh_all_wage_bills` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado na tabela de finanças inteira ([this], [0xae235c] × 0x167 bytes) e em [0xae1080].
pub(crate) unsafe extern "thiscall" fn comparar_refresh_all_wage_bills(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_REFRESH_ALL_WAGE_BILLS);
    let n = (read_i32(G_CLUB_COUNT).max(0) as usize).wrapping_mul(0x167);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(read_usize(this), n), (G_CONTRACT_LOOKUP_RESULT, 4)], || o(this), || refresh_all_wage_bills(this));
    PLACAR.registrar(igual, || format!("refresh_all_wage_bills({:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// Comparer for `finance_get_days_to_season_start` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sem linha da nação, só a original (a caixa); senão lado a lado na saída `passou` e na data estática da
/// 0x54cdb0 (0xae2470), com a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_finance_get_days_to_season_start(this: usize, clube: usize, passou: *mut u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u32) -> i32>(IDX_FINANCE_GET_DAYS_TO_SEASON_START);
    let linha: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_CLUB);
    if (linha(clube) as i8) < 0 {
        return o(this, clube, passou);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(passou as usize, 4), (G_DAYS_SINCE_SCRATCH_DATE, 8)], || o(this, clube, passou), || finance_get_days_to_season_start(this, clube, passou));
    PLACAR.registrar(igual && r == n, || format!("finance_get_days_to_season_start(clube {:#x}) original={} {:x?} rust={} {:x?}", clube, r, mo, n, mr));
    r
}

/// Comparer for `add_players_sold` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_add_players_sold(this: usize, baixo: u32, alto: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_ADD_PLAYERS_SOLD);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, baixo, alto), || add_players_sold(this, baixo, alto));
    PLACAR.registrar(igual, || format!("add_players_sold(fin {:#x}, {}) original={:x?} rust={:x?}", this, i64_from_halves(baixo, alto), mo, mr));
}

/// Comparer for `pay_players_bought` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_pay_players_bought(this: usize, baixo: u32, alto: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_PAY_PLAYERS_BOUGHT);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, baixo, alto), || pay_players_bought(this, baixo, alto));
    PLACAR.registrar(igual, || format!("pay_players_bought(fin {:#x}, {}) original={:x?} rust={:x?}", this, i64_from_halves(baixo, alto), mo, mr));
}

/// Comparer for `pay_sign_on_fee` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_pay_sign_on_fee(this: usize, valor: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAY_SIGN_ON_FEE);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pay_sign_on_fee(this, valor));
    PLACAR.registrar(igual, || format!("pay_sign_on_fee(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}

/// Comparer for `add_prize_money` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_add_prize_money(this: usize, valor: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_ADD_PRIZE_MONEY);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || add_prize_money(this, valor));
    PLACAR.registrar(igual, || format!("add_prize_money(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}

/// Comparer for `add_tv_revenue` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_add_tv_revenue(this: usize, valor: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_ADD_TV_REVENUE);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || add_tv_revenue(this, valor));
    PLACAR.registrar(igual, || format!("add_tv_revenue(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}

/// Comparer for `pay_wages` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_pay_wages(this: usize, valor: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAY_WAGES);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pay_wages(this, valor));
    PLACAR.registrar(igual, || format!("pay_wages(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}

/// Comparer for `add_weekly_merchandising_income` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no registro de finanças do clube e no estado do sorteio.
pub(crate) unsafe extern "thiscall" fn comparar_add_weekly_merchandising_income(this: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_WEEKLY_MERCHANDISING_INCOME);
    let mut regioes = vec![(fin_prm_club_finance(this, clube), 0x167)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, clube), || crate::aleatorio::sem_contar(|| add_weekly_merchandising_income(this, clube)));
    PLACAR.registrar(igual, || format!("add_weekly_merchandising_income(clube {:#x}) original={:x?} rust={:x?}", clube, mo, mr));
}

/// Comparer for `sell_season_tickets` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no registro de finanças (this), nos lugares do estádio (+0x3c) e no estado do sorteio
/// (clube nulo: só a original).
pub(crate) unsafe extern "thiscall" fn comparar_sell_season_tickets(this: usize, aplicar: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_SELL_SEASON_TICKETS);
    let clube = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(this + 8).wrapping_mul(club::SIZE));
    if clube == 0 {
        return o(this, aplicar);
    }
    let mut regioes = vec![(this, 0x167)];
    let est = read_usize(clube + club::STADIUM);
    if est != 0 {
        regioes.push((est + 0x3c, 4));
    }
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, aplicar), || crate::aleatorio::sem_contar(|| sell_season_tickets(this, aplicar)));
    PLACAR.registrar(igual, || format!("sell_season_tickets(fin {:#x}, {}) original={:x?} rust={:x?}", this, aplicar, mo, mr));
}

/// Comparer for `upgrade_training_facilities` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no nível do centro de treino (+0x7f) e no registro de finanças do clube.
pub(crate) unsafe extern "thiscall" fn comparar_upgrade_training_facilities(this: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_UPGRADE_TRAINING_FACILITIES);
    let regioes = [(clube + club::TRAINING, 1), (fin_prm_club_finance(this, clube), 0x167)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, clube), || upgrade_training_facilities(this, clube));
    PLACAR.registrar(igual, || format!("upgrade_training_facilities(clube {:#x}) original={:x?} rust={:x?}", clube, mo, mr));
}

/// Comparer for `process_club_finance_events` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_process_club_finance_events(this: usize, clube: usize, semana: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_PROCESS_CLUB_FINANCE_EVENTS)(this, clube, semana)
}

/// Comparer for `try_board_debt_payment` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_try_board_debt_payment(this: usize, clube: usize, fin: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_TRY_BOARD_DEBT_PAYMENT);
    compare_finance_event("try_board_debt_payment", fin, || o(this, clube, fin), || try_board_debt_payment(this, clube, fin))
}

/// Comparer for `try_cash_injection` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_try_cash_injection(this: usize, clube: usize, fin: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_TRY_CASH_INJECTION);
    compare_finance_event("try_cash_injection", fin, || o(this, clube, fin), || try_cash_injection(this, clube, fin))
}

/// Comparer for `get_max_staff_wage` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "thiscall" fn comparar_get_max_staff_wage(this: usize, cargo: i8) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> i32>(IDX_GET_MAX_STAFF_WAGE)(this, cargo);
    let n = get_max_staff_wage(this, cargo);
    PLACAR.registrar(r == n, || format!("get_max_staff_wage(fin {:#x}, cargo {}) original={} rust={}", this, cargo, r, n));
    r
}

/// Comparer for `pay_bonuses` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_pay_bonuses(this: usize, valor: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAY_BONUSES);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pay_bonuses(this, valor));
    PLACAR.registrar(igual, || format!("pay_bonuses(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}

/// Comparador: lado a lado no teto salarial (+0x14).
pub(crate) unsafe extern "thiscall" fn comparar_adjust_max_wage(this: usize, write_percent_pair: i8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8)>(IDX_ADJUST_MAX_WAGE);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x14, 4)], || o(this, write_percent_pair), || adjust_max_wage(this, write_percent_pair));
    PLACAR.registrar(igual, || format!("adjust_max_wage(fin {:#x}, {}) original={:x?} rust={:x?}", this, write_percent_pair, mo, mr));
}

/// Comparer for `get_wealth_band` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "thiscall" fn comparar_get_wealth_band(this: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_GET_WEALTH_BAND)(this);
    let n = get_wealth_band(this);
    PLACAR.registrar(r == n, || format!("get_wealth_band(financas {:#x}) original={} rust={}", this, r as i8, n as i8));
    r
}
