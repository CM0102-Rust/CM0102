use super::*;

// The comparers of the routines in ../comp/league.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/league.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `set_club_season_expectations` (comparison mode: see the header of motor.rs).
///
/// [pt] Só a original (as expectativas mudam a diretoria e mandam notícias; não rodam duas vezes): a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_set_club_season_expectations(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SET_CLUB_SEASON_EXPECTATIONS)(this)
}

/// Comparer for `relegate_club_to_division` (comparison mode: see the header of motor.rs).
///
/// [pt] Só a original (cláusulas de contrato e o técnico que sai não rodam duas vezes), conferindo o que ela grava: +0x57 =
/// a divisão e +0x5b = a competição de antes.
pub(crate) unsafe extern "thiscall" fn comparar_relegate_club_to_division(this: usize, clube: usize, divisao: usize, x: u32) {
    let antiga = read_u32_as_usize(clube + club::DIVISION);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_RELEGATE_CLUB_TO_DIVISION)(this, clube, divisao, x);
    PLACAR.registrar(read_u32_as_usize(clube + club::DIVISION) == divisao && read_u32_as_usize(clube + club::LAST_DIVISION) == antiga, || format!("relegate_club_to_division: +0x57 {:#x} +0x5b {:#x}", read_u32_as_usize(clube + club::DIVISION), read_u32_as_usize(clube + club::LAST_DIVISION)));
}

/// Comparer for `move_fixtures_for_tv` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x685640: só a original (os jogos saem do calendário e voltam pela 0x5aa680, que aloca, as notas
/// sorteiam e as notícias vão aos técnicos). O plano, feito antes: pela entrada da data, os jogos da liga no dia e fatia
/// dela (0x5abd90 numa lista provisória) e as vagas válidas (fatia 0..2) antes da primeira vazia; cada vaga válida leva
/// um jogo enquanto a lista tiver (toda nota passa do −1 inicial), e a vaga fica 0xff: as vagas 0xff do bloco depois da
/// original são min(válidas, jogos).
pub(crate) unsafe extern "thiscall" fn comparar_move_fixtures_for_tv(this: usize, data: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_MOVE_FIXTURES_FOR_TV);
    let e = match tv_date_entry(this, data) {
        Some(e) => e,
        None => return o(this, data),
    };
    let mut bloco = 0usize;
    let mut validas = 0usize;
    while bloco < 8 && read_u8(e + bloco * 7 + 0xb) != 0xff {
        if (0..=2).contains(&(read_u8(e + bloco * 7 + 0xa) as i8)) {
            validas += 1;
        }
        bloco += 1;
    }
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, usize) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_COLLECT_COMP_FIXTURES_ON_DAY);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    criar(l);
    let n = jogos(read_u32_as_usize(G_FIXTURE_MANAGER), read_i16(data + 2) as u16 as u32, read_i16(data) as u16 as u32, read_u8(e + 4) as u32,
        read_u32_as_usize(read_u32_as_usize(this + 4)) as u32, u32::MAX, read_u8(this + 0x44) as u32, l) as usize & 0xffff;
    destruir(l);
    o(this, data);
    let esperado = if n == 0 { 0 } else { validas.min(n) };
    let movidas = (0..bloco).filter(|k| read_u8(e + k * 7 + 0xb) == 0xff).count();
    PLACAR.registrar(movidas == esperado, || format!("move_fixtures_for_tv(data {:x}) vagas {} válidas {} jogos {}: movidas {} (esperado {})",
        data, bloco, validas, n, movidas, esperado));
}

/// Comparer for `send_group_draw_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x684230: só a original (aloca a notícia e a entrega aos técnicos).
pub(crate) unsafe extern "thiscall" fn comparar_send_group_draw_news(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SEND_GROUP_DRAW_NEWS)(this) // simplification: só a notícia
}

/// Comparer for `send_fixture_list_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x683ee0: só a original (as notícias vão aos técnicos e a lista dos avisados cresce, alocando).
pub(crate) unsafe extern "thiscall" fn comparar_send_fixture_list_news(this: usize, lista: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SEND_FIXTURE_LIST_NEWS)(this, lista) // simplification: só as notícias
}

/// Comparer for `league_update_for_date` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador do dia da liga: só a original (simplification: chama os métodos de cada fase e da liga, as notícias, os
/// recordes e os jogos para a TV, que mexem no calendário: não roda duas vezes). Confere o que a rotina garante: 0 só
/// com a liga parada ou sem clubes e, completo, +0xd1 com a data.
pub(crate) unsafe extern "thiscall" fn comparar_league_update_for_date(this: usize, data: usize, completo: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_LEAGUE_UPDATE_FOR_DATE);
    let parada = read_u8(this + 0xb5) != 0 || read_u32_as_usize(this + 0x45) == 0;
    let r = o(this, data, completo);
    let ok = if parada { r as u8 == 0 } else { r as u8 == 1 && (completo == 0 || same_date(this + 0xd1, data)) };
    PLACAR.registrar(ok, || format!("league_update_for_date(completo {}): parada {} devolveu {} +0xd1 {:x} data {:x}",
        completo, parada, r as u8, read_u32_as_usize(this + 0xd1), read_u32_as_usize(data)));
    r
}

/// Comparer for `create_fixtures` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da criação dos jogos: só a original (simplification: aloca a grade, sorteia a ordem dos clubes e põe os
/// jogos no calendário, que aloca: não roda duas vezes). Confere o que a rotina garante: com a lista própria do ano, ela
/// é liberada (+0xea = 0) quando devolve 1.
pub(crate) unsafe extern "thiscall" fn comparar_create_fixtures(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CREATE_FIXTURES);
    let lista = read_u32_as_usize(this + 0xea);
    let propria = lista != 0 && read_i16(this + 0x40) as i32 == read_i16(lista + 2) as u16 as i32;
    let r = o(this);
    let ok = !(propria && r == 1) || read_u32_as_usize(this + 0xea) == 0;
    PLACAR.registrar(ok, || format!("create_fixtures: lista própria {} devolveu {} +0xea {:x}", propria, r, read_u32_as_usize(this + 0xea)));
    r
}

/// Comparer for `league_promote_and_relegate` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da promoção e do rebaixamento: só a original (simplification: os contratos, o técnico demitido e as notícias
/// não rodam duas vezes); a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_league_promote_and_relegate(ecx: usize, acima: usize, abaixo: usize, contratos: u32, aos_pares: u32, sobem: u32, descem: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32)>(IDX_LEAGUE_PROMOTE_AND_RELEGATE)(ecx, acima, abaixo, contratos, aos_pares, sobem, descem)
}

/// Comparer for `update_positions_and_status` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x684cd0, em ensaio (modelo RARE_FINANCE_EVENT_DRY_RUN): a original roda inteira primeiro, gravando a fita dos
/// anúncios (STATUS_ANNOUNCE_TAPE); a memória volta e a cópia roda sem as virtuais +0x40 e +0x68 nem as cláusulas (que mandam
/// notícia, gravam registros e sorteiam o moral), repondo da fita o que a +0x44 de cada anúncio gravou na tabela. As
/// duas a partir do mesmo estado da tabela, de +0xa7 e das vagas europeias dos clubes da tabela, que têm de sair
/// iguais; no fim fica o estado da original. Sem linhas ou com +0xdd: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_update_positions_and_status(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_UPDATE_POSITIONS_AND_STATUS);
    if read_usize(this + 0xdd) != 0 || read_i16(this + 0x3e) <= 0 || read_usize(this + 0xb1) == 0 {
        return o(this);
    }
    let regioes = table_status_regions(this);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    *core::ptr::addr_of_mut!(STATUS_ANNOUNCE_TAPE) = Vec::new();
    STATUS_ANNOUNCE_TAPE_POS = 0;
    *core::ptr::addr_of_mut!(STATUS_ANNOUNCE_RECORD) = Some(regioes.clone());
    o(this);
    *core::ptr::addr_of_mut!(STATUS_ANNOUNCE_RECORD) = None;
    let mo = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    POSITIONS_STATUS_DRY_RUN = true;
    crate::aleatorio::pausar_rastro(|| update_positions_and_status(this));
    POSITIONS_STATUS_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    mo.escrever();
    PLACAR.registrar(mo == mr, || format!("update_positions_and_status({:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// Comparer for `pay_prize_money` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: com +0xe1 = 3 (a virtual da competição paga por conta própria) ou o divisor zero: só a original. Nos
/// outros casos, o ensaio: a cópia sem a notícia do prêmio (0x48e3e0 aloca) e a original a partir do mesmo estado, nos
/// registros de finanças (0x167 bytes) de todos os clubes da tabela.
pub(crate) unsafe extern "thiscall" fn comparar_pay_prize_money(liga: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PAY_PRIZE_MONEY);
    let n = read_i16(liga + 0x3e);
    let e1 = read_u8(liga + 0xe1);
    if e1 == 3 || sum_of_positions(n) == 0 {
        return o(liga); // simplification: a virtual ou a divisão por zero, só a original
    }
    let mut regioes: Vec<(usize, usize)> = Vec::new();
    if e1 != 2 {
        for i in 0..n.max(0) as usize {
            let fin = european_cup_club_finance(read_usize(read_usize(liga + 0xb1) + 0x3b * i));
            if fin != 0 && !regioes.iter().any(|r| r.0 == fin) {
                regioes.push((fin, 0x167));
            }
        }
    }
    let antes = crate::comparacao::Memoria::ler(&regioes);
    LEAGUE_PRIZE_MONEY_DRY_RUN = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| pay_prize_money(liga)));
    LEAGUE_PRIZE_MONEY_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(liga);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("pay_prize_money({:x}) +0xe1 {} n {} original={:x?} rust={:x?}", liga, e1, n, mo, mr));
}

/// Comparer for `league_update_staff_records_for_status` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_league_update_staff_records_for_status(liga: usize, clube: usize, situacao: u32, fase: u32, saida: usize, a: u32, b: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32) -> u32>(IDX_LEAGUE_UPDATE_STAFF_RECORDS_FOR_STATUS)(liga, clube, situacao, fase, saida, a, b)
}

/// Comparer for `announce_team_status` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_announce_team_status(liga: usize, clube: usize, situacao: u32, fase: u32, a: u32, b: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32>(IDX_ANNOUNCE_TEAM_STATUS);
    let Some(regioes) = (*core::ptr::addr_of!(STATUS_ANNOUNCE_RECORD)).clone() else {
        return o(liga, clube, situacao, fase, a, b);
    };
    let antes = crate::comparacao::Memoria::ler(&regioes);
    let r = o(liga, clube, situacao, fase, a, b);
    let depois = crate::comparacao::Memoria::ler(&regioes);
    let mut bytes = Vec::new();
    for ((base, x), (_, y)) in antes.0.iter().zip(depois.0.iter()) {
        bytes.extend((0..x.len()).filter(|&k| x[k] != y[k]).map(|k| (base + k, y[k])));
    }
    (*core::ptr::addr_of_mut!(STATUS_ANNOUNCE_TAPE)).push((clube, situacao as u8, bytes));
    r
}

/// Comparer for `league_add_result` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a partida nula e a da fase (a virtual da fase recebe a partida e compara por conta própria): só a
/// original. Nos outros casos, o ensaio: a cópia sem a estatística (0x49f570 acha ou cria registros) e a original a
/// partir do mesmo estado: a data em +0xc9, as linhas da tabela com os confrontos (as regiões da 0x685f60), o +0x3d/+0x3e
/// da partida e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_league_add_result(liga: usize, p: usize, jogo: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(IDX_LEAGUE_ADD_RESULT);
    if p == 0 || (*((liga + 0x44) as *const u8) == 0xff && *((p + 0x42) as *const u8) != 0xff) {
        return o(liga, p, jogo); // simplification: a caixa ou a fase, só a original
    }
    let n = read_i16(liga + 0x3e) as u16 as u32;
    let mut regioes = vec![(liga + 0xc9, 8)];
    regioes.extend(result_update_regions(p, read_usize(liga + 0xb1), n, core::ptr::null_mut(), core::ptr::null_mut()));
    let antes = crate::comparacao::Memoria::ler(&regioes);
    LEAGUE_ADD_RESULT_DRY_RUN = true;
    let rr = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| league_add_result(liga, p, jogo)));
    LEAGUE_ADD_RESULT_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    let ro = o(liga, p, jogo);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(ro == rr && mo == mr, || format!("league_add_result({:x}, partida {:x}) original={} {:x?} rust={} {:x?}", liga, p, ro, mo, rr, mr));
    ro
}

/// Comparer for `copy_table` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: aloca (não roda duas vezes): só a original, com o plano completo conferido depois: o +0xa7 esperado
/// (calculado antes) e a cópia devolvida igual à tabela, linha a linha (fora os ponteiros dos vetores novos), com os
/// vetores de confrontos e de forma iguais aos da tabela.
pub(crate) unsafe extern "thiscall" fn comparar_copy_table(liga: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_COPY_TABLE);
    let tab = read_usize(liga + 0xb1);
    let n = read_i16(liga + 0x3e);
    if tab == 0 || n <= 0 || n > 0x32 {
        return o(liga); // simplification: nada a copiar ou a caixa: só a original
    }
    let a7 = if read_i16(liga + 0xa7) == -1 { n.wrapping_sub(1).wrapping_mul(read_i16(liga + 0x3c)) } else { read_i16(liga + 0xa7) };
    let c = o(liga);
    let mut erros = Vec::new();
    if read_i16(liga + 0xa7) != a7 {
        erros.push(format!("+0xa7 {} esperado {}", read_i16(liga + 0xa7), a7));
    }
    if c != 0 {
        let (h2h, forma) = (read_usize(tab + 5) != 0, read_usize(tab + 9) != 0);
        let n = n as usize;
        for i in 0..n {
            let (lt, lc) = (tab + 0x3b * i, c + 0x3b * i);
            for b in 0..0x3b {
                if (h2h && (5..9).contains(&b)) || (forma && (9..13).contains(&b)) {
                    continue;
                }
                if *((lt + b) as *const u8) != *((lc + b) as *const u8) {
                    erros.push(format!("linha {} +{:#x}", i, b));
                    break;
                }
            }
            let iguais = |o_: usize, t: usize| {
                let (a, b) = (read_usize(lt + o_), read_usize(lc + o_));
                b != 0 && core::slice::from_raw_parts(a as *const u8, t) == core::slice::from_raw_parts(b as *const u8, t)
            };
            if h2h && n > 1 && !iguais(5, 14 * (n - 1)) {
                erros.push(format!("linha {} confrontos", i));
            }
            if forma && a7 > 0 && !iguais(9, a7 as usize) {
                erros.push(format!("linha {} forma", i));
            }
        }
    }
    PLACAR.registrar(c != 0 && erros.is_empty(), || format!("copy_table({:x}) n {} copia {:x}: {}", liga, n, c, erros.join("; ")));
    c
}

/// Comparer for `league_set_predicted_positions` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: grava e pode criar registros na estatística (não roda duas vezes): o plano sai antes, sobre uma cópia
/// do vetor ordenada pelo núcleo da 0x4c08f0 (chaves num Vec); depois, o registro de cada clube (a última vez que ele
/// aparece) tem de ter em +0x72 a posição planejada.
pub(crate) unsafe extern "thiscall" fn comparar_league_set_predicted_positions(liga: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_LEAGUE_SET_PREDICTED_POSITIONS);
    let n = read_i16(liga + 0x3e) as i32;
    if read_usize(liga + 8) == 0 || n <= 0 {
        return o(liga); // simplification: a caixa ou nada a ordenar, só a original
    }
    let mut v: Vec<u32> = (0..n as usize).map(|i| read_usize(read_usize(liga + 0xb1) + 0x3b * i) as u32).collect();
    let mut k = vec![0i32; n as usize];
    sort_clubs_by_strength(v.as_mut_ptr(), k.as_mut_ptr(), n);
    o(liga);
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let mut erros = Vec::new();
    for (i, &c) in v.iter().enumerate() {
        if c == 0 || v[i + 1..].contains(&c) {
            continue;
        }
        let p = (i as u8).wrapping_add(1);
        let e = buscar(read_usize(liga + 8), c as usize);
        if e == 0 || *((e + 0x72) as *const u8) != p {
            erros.push(format!("clube {:x} esperado {} obtido {}", c, p, if e == 0 { -1 } else { *((e + 0x72) as *const u8) as i32 }));
        }
    }
    PLACAR.registrar(erros.is_empty(), || format!("league_set_predicted_positions({:x}) {} clubes: {}", liga, n, erros.join("; ")));
}

/// Comparer for `compare_projection_by_reputation_desc` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn comparar_compare_projection_by_reputation_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_PROJECTION_BY_REPUTATION_DESC)(a, b);
    let n = compare_projection_by_reputation_desc(a, b);
    PLACAR.registrar(r == n, || format!("compare_projection_by_reputation_desc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `compare_projection_by_points_desc` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn comparar_compare_projection_by_points_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_PROJECTION_BY_POINTS_DESC)(a, b);
    let n = compare_projection_by_points_desc(a, b);
    PLACAR.registrar(r == n, || format!("compare_projection_by_points_desc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `update_projected_positions` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o bloco da projeção é alocado e liberado dentro (não fica nada dele); o efeito é a virtual +0x68, que
/// grava os bytes +0x73..+0x75 do registro do clube no Comp_Stats da liga (+8, achado pela 0x4a4850, que dentro da
/// 0x4ae300 devolve o registro provisório dela): lado a lado nesses 3 bytes e na devolução. As duas versões que repassam
/// a outra competição (0x48e380, 0x7ef100) gravam fora da região: aí só a devolução pesa.
pub(crate) unsafe extern "thiscall" fn comparar_update_projected_positions(this: usize, clube: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_UPDATE_PROJECTED_POSITIONS);
    let stats = read_usize(this + 8);
    let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let reg = if stats != 0 { busca(stats, clube) } else { 0 };
    if reg == 0 {
        return o(this, clube); // simplification: sem registro para olhar, só a original
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(reg + 0x73, 3)], || o(this, clube),
        || update_projected_positions(this, clube));
    PLACAR.registrar(igual && r == n, || format!("update_projected_positions(liga {:x}, clube {:x}) original={} rust={} {:x?} x {:x?}",
        this, clube, r, n, mo, mr));
    r
}

/// Comparer for `init_table_row` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original aloca (histórico e confrontos): roda só ela e confere a linha com o plano feito antes (o
/// total de rodadas, a situação do clube, os campos, os blocos zerados, os vetores presentes e, na última linha, os
/// adversários de cada linha).
pub(crate) unsafe extern "thiscall" fn comparar_init_table_row(this: usize, i: u32, clube: usize, marca: u32, linhas: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32, usize)>(IDX_INIT_TABLE_ROW);
    let a7 = read_i16(this + 0xa7);
    let n = read_i16(this + 0x3e);
    let a7_esperado = if a7 == -1 && n > 0 { (n - 1).wrapping_mul(read_i16(this + 0x3c)) } else { a7 };
    let situacao = if clube != 0 { read_u8(clube + club::PROFESSIONAL_STATUS) } else { 0 };
    let situacao_esperada = if situacao == 3 { 2 } else { situacao };
    let tab = if linhas == 0 { read_usize(this + 0xb1) } else { linhas };
    let confrontos = uses_head_to_head_rule(this);
    o(this, i, clube, marca, linhas);
    let idx = i as u8 as i8 as isize;
    let l = tab.wrapping_add((idx * TABLE_ROW_SIZE as isize) as usize);
    let mut ok = read_usize(l) == clube && read_u8(l + 4) == i as u8 && read_u8(l + 0x37) == 0xff && read_u8(l + 0x38) == marca as u8
        && read_u8(l + 0x39) == 1 && read_u8(l + 0x3a) == n as u8 && read_i16(this + 0xa7) == a7_esperado
        && (clube == 0 || read_u8(clube + club::PROFESSIONAL_STATUS) == situacao_esperada)
        && (read_usize(l + 9) != 0) == (read_i16(this + 0x3c) > 0) && (read_usize(l + 5) != 0) == confrontos;
    for k in 0..3 {
        let e = l + 0x11 + 14 * k;
        ok &= (e - 4..e + 10).all(|a| read_u8(a) == 0);
    }
    let ultima = idx == n as isize - 1 && read_usize(l + 5) != 0;
    if read_usize(l + 5) != 0 && !ultima {
        let p = read_usize(l + 5);
        ok &= (0..(n as usize).saturating_sub(1)).all(|k| (p + 14 * k + 4..p + 14 * k + 14).all(|a| read_u8(a) == 0));
    }
    if ultima {
        for a in 0..n as usize {
            let p = read_usize(tab + a * TABLE_ROW_SIZE + 5);
            let outros = (0..n as usize).filter(|&j| j != a);
            ok &= outros.enumerate().all(|(k, j)| read_usize(p + 14 * k) == read_usize(tab + j * TABLE_ROW_SIZE));
        }
    }
    PLACAR.registrar(ok, || format!("init_table_row(linha {}, clube {:x}, marca {}) clubes {}, rodadas {} (esperava {}), confrontos {} ({:x}), histórico {:x}",
        idx, clube, marca as u8, n, read_i16(this + 0xa7), a7_esperado, confrontos, read_usize(l + 5), read_usize(l + 9)));
}

/// Comparer for `league_get_club_array` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original aloca o vetor: roda só ela e confere a conta e os clubes, na ordem da tabela.
pub(crate) unsafe extern "thiscall" fn comparar_league_get_club_array(this: usize, saida: *mut usize, n: *mut i16) {
    original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, *mut i16)>(IDX_LEAGUE_GET_CLUB_ARRAY)(this, saida, n);
    let p = *saida;
    if p == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = read_i16(this + 0x3e);
    let ok = *n == conta && (0..conta.max(0) as usize).all(|i| read_usize(p + 4 * i) == read_usize(read_usize(this + 0xb1) + i * TABLE_ROW_SIZE));
    PLACAR.registrar(ok, || format!("league_get_club_array conta {} (tabela {})", *n, conta));
}

/// Comparer for `record_final_positions` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela divisão da última temporada e pela posição (+0x5b..+0x5f) de cada clube da tabela.
pub(crate) unsafe extern "thiscall" fn comparar_record_final_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_RECORD_FINAL_POSITIONS);
    let reg: Vec<(usize, usize)> = (0..read_i16(this + 0x3e).max(0) as usize)
        .map(|i| (read_usize(read_usize(this + 0xb1) + i * TABLE_ROW_SIZE) + 0x5b, 5))
        .collect();
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || record_final_positions(this));
    PLACAR.registrar(igual, || format!("record_final_positions {:x?} x {:x?}", mo, mr));
}

/// Comparer for `apply_result_to_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_apply_result_to_table(liga: usize, p: usize, tab: usize, n: u32, marcar: u32, pts_casa: *mut i16, pts_fora: *mut i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, *mut i16, *mut i16)>(IDX_APPLY_RESULT_TO_TABLE);
    let reg = result_update_regions(p, tab, n, pts_casa, pts_fora);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(liga, p, tab, n, marcar, pts_casa, pts_fora),
        || apply_result_to_table(liga, p, tab, n, marcar, pts_casa, pts_fora));
    PLACAR.registrar(igual, || format!("apply_result_to_table({:x}, partida {:x}, {:x}, n {}, marcar {}) {:x?} x {:x?}",
        liga, p, tab, n as i16, marcar, mo, mr));
}

/// Comparer for `league_compare_table_rows_h2h` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_league_compare_table_rows_h2h(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_LEAGUE_COMPARE_TABLE_ROWS_H2H)(a, b);
    let n = league_compare_table_rows_h2h(a, b);
    PLACAR.registrar(r == n, || format!("league_compare_table_rows_h2h({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `league_compare_dates` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_league_compare_dates(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_LEAGUE_COMPARE_DATES)(a, b);
    let n = league_compare_dates(a, b);
    PLACAR.registrar(r == n, || format!("league_compare_dates({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `sort_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_sort_table(liga: usize, linhas: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SORT_TABLE);
    let reg = sort_regions(liga, linhas);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(liga, linhas), || sort_table(liga, linhas));
    PLACAR.registrar(igual && r == n, || format!("sort_table({:x}, {:x}) {}/{} {:x?} x {:x?}", liga, linhas, r, n, mo, mr));
    r
}

/// Comparer for `build_h2h_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_build_h2h_table(liga: usize, linhas: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_BUILD_H2H_TABLE);
    if linhas == 0 {
        return o(liga, linhas);
    }
    let reg = [(linhas, LEAGUE_TABLE_ROW_SIZE * read_i16(liga + 0x3e).max(0) as usize)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(liga, linhas), || build_h2h_table(liga, linhas));
    PLACAR.registrar(igual, || format!("build_h2h_table({:x}, {:x}) {:x?} x {:x?}", liga, linhas, mo, mr));
}

/// Comparer for `free_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_free_table(liga: usize, tabela: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FREE_TABLE)(liga, tabela)
}

/// Comparer for `league_check_complete` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_league_check_complete(liga: usize, fases: u32, avisar: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(IDX_LEAGUE_CHECK_COMPLETE);
    if !is_complete_check_read_only(liga, fases) {
        return o(liga, fases, avisar); // fim de liga (registros, prêmios, fases, aviso): só a original
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(liga + 0xa7, 2)], || o(liga, fases, avisar), || league_check_complete(liga, fases, avisar));
    PLACAR.registrar(igual && r == n, || format!("league_check_complete({:x}, {}, {}) {}/{} {:x?} x {:x?}", liga, fases, avisar, r, n, mo, mr));
    r
}

/// Comparer for `pay_tv_rights` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: as finanças (0x167 bytes) de cada clube da tabela; a cópia roda sem a notícia (a da original fica, uma
/// vez só). Clube nulo na tabela (a caixa da 0x5a0590): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_pay_tv_rights(liga: usize, valor: i32, filtro: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, usize)>(IDX_PAY_TV_RIGHTS);
    let financas = *(G_FINANCE_TABLE as *const usize);
    let n = read_i16(liga + 0x3e).max(0) as usize;
    let clubes: Vec<usize> = (0..n).map(|i| read_usize(read_usize(liga + 0xb1) + 0x3b * i)).collect();
    if financas == 0 || clubes.contains(&0) {
        return o(liga, valor, filtro);
    }
    let get_club_finance: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let reg: Vec<(usize, usize)> = clubes.iter().map(|&c| (get_club_finance(financas, c), 0x167)).collect();
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(liga, valor, filtro), || pay_tv_money(liga, valor, filtro, false));
    PLACAR.registrar(igual, || format!("pay_tv_rights({:x}, {}, {:x}) {:x?} x {:x?}", liga, valor, filtro, mo, mr));
}

/// Comparer for `league_record_season_honours` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: os clubes (o +0x5b) e o histórico de competições. Sem linha livre no histórico (a 0x4afce0 faria
/// realloc): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_league_record_season_honours(liga: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_LEAGUE_RECORD_SEASON_HONOURS);
    if free_comp_history_row() == 0 {
        return o(liga);
    }
    let mut reg = comp_history_regions();
    let quantos = (*(G_CLUB_COUNT as *const i32)).wrapping_sub(*(G_NATION_COUNT as *const i32)).max(0) as usize;
    reg.push((*(G_CLUB_TABLE as *const usize), 0x245 * quantos));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(liga), || league_record_season_honours(liga));
    PLACAR.registrar(igual && r == n, || format!("league_record_season_honours({:x}) {:x}/{:x} {:x?} x {:x?}", liga, r, n, mo, mr));
    r
}

/// Comparer for `has_relegation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_has_relegation(liga: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HAS_RELEGATION)(liga);
    let n = has_relegation(liga);
    PLACAR.registrar(r == n, || format!("has_relegation original={} rust={}", r, n));
    r
}

/// Comparer for `league_round_set_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_league_round_set_date(rodadas: usize, i: u32, dia: u32, mes: u32, anos: u32, semana: u32, codigo: u32, base: u32, valor: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize>(IDX_LEAGUE_ROUND_SET_DATE);
    let r = rodadas.wrapping_add((i as i16 as isize * 0x41) as usize);
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r, 0x41)], || o(rodadas, i, dia, mes, anos, semana, codigo, base, valor),
        || league_round_set_date(rodadas, i, dia, mes, anos, semana, codigo, base, valor));
    PLACAR.registrar(igual && a == b, || format!("league_round_set_date({}, {}/{} +{}) {:x?} x {:x?}", i as i16, dia, mes, anos, mo, mr));
    a
}

/// Comparer for `get_match_importance` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_match_importance(liga: usize, partida: usize, lado: u32, saida: *mut u8) -> u8 {
    // a 0x68a4d0 só lê a liga e grava na local e na `saida`: pode rodar nos dois lados
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut u8) -> u8>(IDX_GET_MATCH_IMPORTANCE);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(saida as usize, 1)], || o(liga, partida, lado, saida),
        || get_match_importance(liga, partida, lado, saida));
    PLACAR.registrar(igual && r == n, || format!("get_match_importance original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

/// Comparer for `league_compare_table_rows` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_league_compare_table_rows(a: usize, b: usize) -> i32 {
    // a comparação particular de [0xb67a00] (se houver) roda nos dois lados: só lê as linhas
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_LEAGUE_COMPARE_TABLE_ROWS)(a, b);
    let n = league_compare_table_rows(a, b);
    PLACAR.registrar(r == n, || format!("league_compare_table_rows original={} rust={}", r, n));
    r
}

/// Comparer for `league_build_round_robin_grid` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_league_build_round_robin_grid(grade: *const *mut i32, n: i32) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(*const *mut i32, i32) -> u32>(IDX_LEAGUE_BUILD_ROUND_ROBIN_GRID);
    let regioes: Vec<(usize, usize)> = (1..=n.max(0)).map(|i| (*grade.offset(i as isize) as usize + 4, (n.max(1) as usize - 1) * 4)).collect();
    let (r, s, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(grade, n), || league_build_round_robin_grid(grade, n));
    PLACAR.registrar(igual && r == s, || format!("league_build_round_robin_grid({}) {:x?} x {:x?}", n, mo, mr));
    r
}

/// Comparer for `final_day_margin_needed` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_final_day_margin_needed(liga: usize, linha: usize, saida: *mut u8) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u8>(IDX_FINAL_DAY_MARGIN_NEEDED);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(saida as usize, 1)], || o(liga, linha, saida),
        || final_day_margin_needed(liga, linha, saida));
    PLACAR.registrar(igual && r == n, || format!("final_day_margin_needed original={:#x} rust={:#x} {:x?} x {:x?}", r, n, mo, mr));
    r
}

/// Comparer for `league_compare_h2h_rows` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_league_compare_h2h_rows(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_LEAGUE_COMPARE_H2H_ROWS)(a, b);
    let n = league_compare_h2h_rows(a, b);
    PLACAR.registrar(r == n, || format!("league_compare_h2h_rows original={} rust={}", r, n));
    r
}

/// Comparer for `update_head_to_head` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_update_head_to_head(liga: usize, tabela: usize, linha: i32, adversario: i32, pro: i8, contra: i8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, i32, i8, i8)>(IDX_UPDATE_HEAD_TO_HEAD);
    if linha < 0 || adversario < 0 {
        return o(liga, tabela, linha, adversario, pro, contra);
    }
    let e = head_to_head_record(tabela, linha, adversario);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(e, 14)], || o(liga, tabela, linha, adversario, pro, contra),
        || update_head_to_head(liga, tabela, linha, adversario, pro, contra));
    PLACAR.registrar(igual, || format!("update_head_to_head({}, {}, {}x{}) {:x?} x {:x?}", linha, adversario, pro, contra, mo, mr));
}

/// Comparer for `rotate_round` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_rotate_round(pos: i32, step_towards: *mut i8, liga: i32, rodadas: i16, turnos: i16, n: i16, marcas: u8) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(i32, *mut i8, i32, i16, i16, i16, u8) -> i32>(IDX_ROTATE_ROUND);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((step_towards as usize, 1));
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pos, step_towards, liga, rodadas, turnos, n, marcas), || crate::aleatorio::sem_contar(|| rotate_round(pos, step_towards, liga, rodadas, turnos, n, marcas)));
    PLACAR.registrar(igual && r == v, || format!("rotate_round({}, {}, {}, {}, {}, {:#x}) original={} {:x?} rust={} {:x?}", pos, liga, rodadas, turnos, n, marcas, r, mo, v, mr));
    r
}

/// Comparer for `store_round_match` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_store_round_match(base: usize, rodada: i16, j: i16, a: i16, b: i8, c: i8, valor: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i16, i16, i16, i8, i8, u32)>(IDX_STORE_ROUND_MATCH);
    let e = base.wrapping_add((rodada as isize * 0x41 + j as isize * 7) as usize);
    let reg = [(e + 5, 7)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(base, rodada, j, a, b, c, valor), || store_round_match(base, rodada, j, a, b, c, valor));
    PLACAR.registrar(igual, || format!("store_round_match({:#x}, {}, {}, {}, {}, {}, {:#x}) {:x?} x {:x?}", base, rodada, j, a, b, c, valor, mo, mr));
}

/// Comparer for `calc_prize_fund` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_calc_prize_fund(liga: usize, cota: i32) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> i32>(IDX_CALC_PRIZE_FUND)(liga, cota);
    if *((liga + 0x3e) as *const i16) == 0 {
        return r;
    }
    let n = calc_prize_fund(liga, cota);
    PLACAR.registrar(r == n, || format!("calc_prize_fund({}) original={} rust={}", cota, r, n));
    r
}

/// Comparer for `league_process_promotion_relegation_chain` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_league_process_promotion_relegation_chain(this: usize, aos_pares: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_LEAGUE_PROCESS_PROMOTION_RELEGATION_CHAIN)(this, aos_pares)
}

/// Comparer for `move_club_to_division` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a mudança chama as notícias, a vaga de técnico e a obra (não podem rodar duas vezes): só a original (a
/// prova é a carreira e o modo Rust).
pub(crate) unsafe extern "thiscall" fn comparar_move_club_to_division(this: usize, clube: usize, divisao: usize, vaga: u32) {
    crate::comparacao::ramo("move_club_to_division");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_MOVE_CLUB_TO_DIVISION)(this, clube, divisao, vaga)
}

/// Comparer for `draw_team_order` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: embaralha com o sorteio e o rand, aloca e chama a tabela virtual: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_draw_team_order(this: usize, clubes: i32) {
    // simplification: sorteia e aloca (não roda duas vezes); a cópia se prova pela carreira (a grade de jogos de toda liga)
    original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_DRAW_TEAM_ORDER)(this, clubes)
}

/// Comparer for `limit_club_division_tier` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: grava só o byte +0x64 dos clubes da tabela: lado a lado nesses bytes.
pub(crate) unsafe extern "thiscall" fn comparar_limit_club_division_tier(this: usize, valor: u32, fora: u32, dentro: u32, baixar: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32)>(IDX_LIMIT_CLUB_DIVISION_TIER);
    let n = *((this + 0x3e) as *const i16);
    let tabela = read_usize(this + 0xb1);
    let reg: Vec<(usize, usize)> = (0..n.max(0) as usize).map(|i| (read_usize(tabela + i * 0x3b) + 0x64, 1)).collect();
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, valor, fora, dentro, baixar),
        || limit_club_division_tier(this, valor, fora, dentro, baixar));
    PLACAR.registrar(igual, || format!("limit_club_division_tier(valor {}, fora {:x}, dentro {:x}, baixar {}) {:x?} x {:x?}",
        valor as u8 as i8, fora as u8, dentro as u8, baixar, mo, mr));
}

/// Comparer for `narrate_qualification` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_narrate_qualification(this: usize, lado: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_NARRATE_QUALIFICATION);
    if is_recording_top_level() {
        call_recording(FN_CM3_MATCH_ENGINE_POST_PERIOD_END_EVENTS, this, [lado, 0, 0, 0, 0], || {
            o(this, lado);
            0
        });
        return;
    }
    let (igual, n) = comparar_com_comentarios(|| o(this, lado), |c| comment_standing_with(this, lado, c));
    PLACAR.registrar(igual, || format!("narrate_qualification({}) {} chamadas", lado as u8, n));
}
