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
pub(crate) unsafe extern "thiscall" fn comparar_relegate_club_to_division(this: usize, clube: usize, division: usize, x: u32) {
    let antiga = read_u32_as_usize(clube + club::DIVISION);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_RELEGATE_CLUB_TO_DIVISION)(this, clube, division, x);
    PLACAR.registrar(read_u32_as_usize(clube + club::DIVISION) == division && read_u32_as_usize(clube + club::LAST_DIVISION) == antiga, || format!("relegate_club_to_division: +0x57 {:#x} +0x5b {:#x}", read_u32_as_usize(clube + club::DIVISION), read_u32_as_usize(clube + club::LAST_DIVISION)));
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
    let mut block = 0usize;
    let mut validas = 0usize;
    while block < 8 && read_u8(e + block * 7 + 0xb) != 0xff {
        if (0..=2).contains(&(read_u8(e + block * 7 + 0xa) as i8)) {
            validas += 1;
        }
        block += 1;
    }
    let create: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let games: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, usize) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_COLLECT_COMP_FIXTURES_ON_DAY);
    let mut list = [0u8; 0x10];
    let l = list.as_mut_ptr() as usize;
    create(l);
    let n = games(read_u32_as_usize(G_FIXTURE_MANAGER), read_i16(data + 2) as u16 as u32, read_i16(data) as u16 as u32, read_u8(e + 4) as u32,
        read_u32_as_usize(read_u32_as_usize(this + 4)) as u32, u32::MAX, read_u8(this + 0x44) as u32, l) as usize & 0xffff;
    destroy(l);
    o(this, data);
    let expected = if n == 0 { 0 } else { validas.min(n) };
    let movidas = (0..block).filter(|k| read_u8(e + k * 7 + 0xb) == 0xff).count();
    PLACAR.registrar(movidas == expected, || format!("move_fixtures_for_tv(data {:x}) vagas {} válidas {} jogos {}: movidas {} (esperado {})",
        data, block, validas, n, movidas, expected));
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
pub(crate) unsafe extern "thiscall" fn comparar_send_fixture_list_news(this: usize, list: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SEND_FIXTURE_LIST_NEWS)(this, list) // simplification: só as notícias
}

/// Comparer for `league_update_for_date` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador do dia da liga: só a original (simplification: chama os métodos de cada fase e da liga, as notícias, os
/// recordes e os jogos para a TV, que mexem no calendário: não roda duas vezes). Confere o que a rotina garante: 0 só
/// com a liga parada ou sem clubes e, completo, +0xd1 com a data.
pub(crate) unsafe extern "thiscall" fn comparar_league_update_for_date(this: usize, data: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_LEAGUE_UPDATE_FOR_DATE);
    let parada = read_u8(this + 0xb5) != 0 || read_u32_as_usize(this + 0x45) == 0;
    let r = o(this, data, full);
    let ok = if parada { r as u8 == 0 } else { r as u8 == 1 && (full == 0 || same_date(this + 0xd1, data)) };
    PLACAR.registrar(ok, || format!("league_update_for_date(completo {}): parada {} devolveu {} +0xd1 {:x} data {:x}",
        full, parada, r as u8, read_u32_as_usize(this + 0xd1), read_u32_as_usize(data)));
    r
}

/// Comparer for `create_fixtures` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da criação dos jogos: só a original (simplification: aloca a grade, sorteia a ordem dos clubes e põe os
/// jogos no calendário, que aloca: não roda duas vezes). Confere o que a rotina garante: com a lista própria do ano, ela
/// é liberada (+0xea = 0) quando devolve 1.
pub(crate) unsafe extern "thiscall" fn comparar_create_fixtures(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CREATE_FIXTURES);
    let list = read_u32_as_usize(this + 0xea);
    let propria = list != 0 && read_i16(this + 0x40) as i32 == read_i16(list + 2) as u16 as i32;
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
    let regions = table_status_regions(this);
    let before = crate::comparacao::Memoria::ler(&regions);
    *core::ptr::addr_of_mut!(STATUS_ANNOUNCE_TAPE) = Vec::new();
    STATUS_ANNOUNCE_TAPE_POS = 0;
    *core::ptr::addr_of_mut!(STATUS_ANNOUNCE_RECORD) = Some(regions.clone());
    o(this);
    *core::ptr::addr_of_mut!(STATUS_ANNOUNCE_RECORD) = None;
    let mo = crate::comparacao::Memoria::ler(&regions);
    before.escrever();
    POSITIONS_STATUS_DRY_RUN = true;
    crate::aleatorio::pausar_rastro(|| update_positions_and_status(this));
    POSITIONS_STATUS_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regions);
    mo.escrever();
    PLACAR.registrar(mo == mr, || format!("update_positions_and_status({:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// Comparer for `pay_prize_money` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: com +0xe1 = 3 (a virtual da competição paga por conta própria) ou o divisor zero: só a original. Nos
/// outros casos, o ensaio: a cópia sem a notícia do prêmio (0x48e3e0 aloca) e a original a partir do mesmo estado, nos
/// registros de finanças (0x167 bytes) de todos os clubes da tabela.
pub(crate) unsafe extern "thiscall" fn comparar_pay_prize_money(league: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PAY_PRIZE_MONEY);
    let n = read_i16(league + 0x3e);
    let e1 = read_u8(league + 0xe1);
    if e1 == 3 || sum_of_positions(n) == 0 {
        return o(league); // simplification: a virtual ou a divisão por zero, só a original
    }
    let mut regions: Vec<(usize, usize)> = Vec::new();
    if e1 != 2 {
        for i in 0..n.max(0) as usize {
            let fin = european_cup_club_finance(read_usize(read_usize(league + 0xb1) + 0x3b * i));
            if fin != 0 && !regions.iter().any(|r| r.0 == fin) {
                regions.push((fin, 0x167));
            }
        }
    }
    let before = crate::comparacao::Memoria::ler(&regions);
    LEAGUE_PRIZE_MONEY_DRY_RUN = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| pay_prize_money(league)));
    LEAGUE_PRIZE_MONEY_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regions);
    before.escrever();
    o(league);
    let mo = crate::comparacao::Memoria::ler(&regions);
    PLACAR.registrar(mo == mr, || format!("pay_prize_money({:x}) +0xe1 {} n {} original={:x?} rust={:x?}", league, e1, n, mo, mr));
}

/// Comparer for `league_update_staff_records_for_status` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_league_update_staff_records_for_status(league: usize, clube: usize, status: u32, stage: u32, out: usize, a: u32, b: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32) -> u32>(IDX_LEAGUE_UPDATE_STAFF_RECORDS_FOR_STATUS)(league, clube, status, stage, out, a, b)
}

/// Comparer for `announce_team_status` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_announce_team_status(league: usize, clube: usize, status: u32, stage: u32, a: u32, b: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32>(IDX_ANNOUNCE_TEAM_STATUS);
    let Some(regioes) = (*core::ptr::addr_of!(STATUS_ANNOUNCE_RECORD)).clone() else {
        return o(league, clube, status, stage, a, b);
    };
    let before = crate::comparacao::Memoria::ler(&regioes);
    let r = o(league, clube, status, stage, a, b);
    let after = crate::comparacao::Memoria::ler(&regioes);
    let mut bytes = Vec::new();
    for ((base, x), (_, y)) in before.0.iter().zip(after.0.iter()) {
        bytes.extend((0..x.len()).filter(|&k| x[k] != y[k]).map(|k| (base + k, y[k])));
    }
    (*core::ptr::addr_of_mut!(STATUS_ANNOUNCE_TAPE)).push((clube, status as u8, bytes));
    r
}

/// Comparer for `league_add_result` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a partida nula e a da fase (a virtual da fase recebe a partida e compara por conta própria): só a
/// original. Nos outros casos, o ensaio: a cópia sem a estatística (0x49f570 acha ou cria registros) e a original a
/// partir do mesmo estado: a data em +0xc9, as linhas da tabela com os confrontos (as regiões da 0x685f60), o +0x3d/+0x3e
/// da partida e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_league_add_result(league: usize, p: usize, jogo: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(IDX_LEAGUE_ADD_RESULT);
    if p == 0 || (*((league + 0x44) as *const u8) == 0xff && *((p + 0x42) as *const u8) != 0xff) {
        return o(league, p, jogo); // simplification: a caixa ou a fase, só a original
    }
    let n = read_i16(league + 0x3e) as u16 as u32;
    let mut regions = vec![(league + 0xc9, 8)];
    regions.extend(result_update_regions(p, read_usize(league + 0xb1), n, core::ptr::null_mut(), core::ptr::null_mut()));
    let before = crate::comparacao::Memoria::ler(&regions);
    LEAGUE_ADD_RESULT_DRY_RUN = true;
    let rr = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| league_add_result(league, p, jogo)));
    LEAGUE_ADD_RESULT_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regions);
    before.escrever();
    let ro = o(league, p, jogo);
    let mo = crate::comparacao::Memoria::ler(&regions);
    PLACAR.registrar(ro == rr && mo == mr, || format!("league_add_result({:x}, partida {:x}) original={} {:x?} rust={} {:x?}", league, p, ro, mo, rr, mr));
    ro
}

/// Comparer for `copy_table` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: aloca (não roda duas vezes): só a original, com o plano completo conferido depois: o +0xa7 esperado
/// (calculado antes) e a cópia devolvida igual à tabela, linha a linha (fora os ponteiros dos vetores novos), com os
/// vetores de confrontos e de forma iguais aos da tabela.
pub(crate) unsafe extern "thiscall" fn comparar_copy_table(league: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_COPY_TABLE);
    let table = read_usize(league + 0xb1);
    let n = read_i16(league + 0x3e);
    if table == 0 || n <= 0 || n > 0x32 {
        return o(league); // simplification: nada a copiar ou a caixa: só a original
    }
    let a7 = if read_i16(league + 0xa7) == -1 { n.wrapping_sub(1).wrapping_mul(read_i16(league + 0x3c)) } else { read_i16(league + 0xa7) };
    let c = o(league);
    let mut erros = Vec::new();
    if read_i16(league + 0xa7) != a7 {
        erros.push(format!("+0xa7 {} esperado {}", read_i16(league + 0xa7), a7));
    }
    if c != 0 {
        let (h2h, forma) = (read_usize(table + 5) != 0, read_usize(table + 9) != 0);
        let n = n as usize;
        for i in 0..n {
            let (lt, lc) = (table + 0x3b * i, c + 0x3b * i);
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
    PLACAR.registrar(c != 0 && erros.is_empty(), || format!("copy_table({:x}) n {} copia {:x}: {}", league, n, c, erros.join("; ")));
    c
}

/// Comparer for `league_set_predicted_positions` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: grava e pode criar registros na estatística (não roda duas vezes): o plano sai antes, sobre uma cópia
/// do vetor ordenada pelo núcleo da 0x4c08f0 (chaves num Vec); depois, o registro de cada clube (a última vez que ele
/// aparece) tem de ter em +0x72 a posição planejada.
pub(crate) unsafe extern "thiscall" fn comparar_league_set_predicted_positions(league: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_LEAGUE_SET_PREDICTED_POSITIONS);
    let n = read_i16(league + 0x3e) as i32;
    if read_usize(league + 8) == 0 || n <= 0 {
        return o(league); // simplification: a caixa ou nada a ordenar, só a original
    }
    let mut v: Vec<u32> = (0..n as usize).map(|i| read_usize(read_usize(league + 0xb1) + 0x3b * i) as u32).collect();
    let mut k = vec![0i32; n as usize];
    sort_clubs_by_strength(v.as_mut_ptr(), k.as_mut_ptr(), n);
    o(league);
    let search: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let mut erros = Vec::new();
    for (i, &c) in v.iter().enumerate() {
        if c == 0 || v[i + 1..].contains(&c) {
            continue;
        }
        let p = (i as u8).wrapping_add(1);
        let e = search(read_usize(league + 8), c as usize);
        if e == 0 || *((e + 0x72) as *const u8) != p {
            erros.push(format!("clube {:x} esperado {} obtido {}", c, p, if e == 0 { -1 } else { *((e + 0x72) as *const u8) as i32 }));
        }
    }
    PLACAR.registrar(erros.is_empty(), || format!("league_set_predicted_positions({:x}) {} clubes: {}", league, n, erros.join("; ")));
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
    let search: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let reg = if stats != 0 { search(stats, clube) } else { 0 };
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
pub(crate) unsafe extern "thiscall" fn comparar_init_table_row(this: usize, i: u32, clube: usize, mark: u32, rows: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32, usize)>(IDX_INIT_TABLE_ROW);
    let a7 = read_i16(this + 0xa7);
    let n = read_i16(this + 0x3e);
    let a7_expected = if a7 == -1 && n > 0 { (n - 1).wrapping_mul(read_i16(this + 0x3c)) } else { a7 };
    let status = if clube != 0 { read_u8(clube + club::PROFESSIONAL_STATUS) } else { 0 };
    let situacao_esperada = if status == 3 { 2 } else { status };
    let table = if rows == 0 { read_usize(this + 0xb1) } else { rows };
    let confrontos = uses_head_to_head_rule(this);
    o(this, i, clube, mark, rows);
    let idx = i as u8 as i8 as isize;
    let l = table.wrapping_add((idx * TABLE_ROW_SIZE as isize) as usize);
    let mut ok = read_usize(l) == clube && read_u8(l + 4) == i as u8 && read_u8(l + 0x37) == 0xff && read_u8(l + 0x38) == mark as u8
        && read_u8(l + 0x39) == 1 && read_u8(l + 0x3a) == n as u8 && read_i16(this + 0xa7) == a7_expected
        && (clube == 0 || read_u8(clube + club::PROFESSIONAL_STATUS) == situacao_esperada)
        && (read_usize(l + 9) != 0) == (read_i16(this + 0x3c) > 0) && (read_usize(l + 5) != 0) == confrontos;
    for k in 0..3 {
        let e = l + 0x11 + 14 * k;
        ok &= (e - 4..e + 10).all(|a| read_u8(a) == 0);
    }
    let last = idx == n as isize - 1 && read_usize(l + 5) != 0;
    if read_usize(l + 5) != 0 && !last {
        let p = read_usize(l + 5);
        ok &= (0..(n as usize).saturating_sub(1)).all(|k| (p + 14 * k + 4..p + 14 * k + 14).all(|a| read_u8(a) == 0));
    }
    if last {
        for a in 0..n as usize {
            let p = read_usize(table + a * TABLE_ROW_SIZE + 5);
            let outros = (0..n as usize).filter(|&j| j != a);
            ok &= outros.enumerate().all(|(k, j)| read_usize(p + 14 * k) == read_usize(table + j * TABLE_ROW_SIZE));
        }
    }
    PLACAR.registrar(ok, || format!("init_table_row(linha {}, clube {:x}, marca {}) clubes {}, rodadas {} (esperava {}), confrontos {} ({:x}), histórico {:x}",
        idx, clube, mark as u8, n, read_i16(this + 0xa7), a7_expected, confrontos, read_usize(l + 5), read_usize(l + 9)));
}

/// Comparer for `league_get_club_array` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original aloca o vetor: roda só ela e confere a conta e os clubes, na ordem da tabela.
pub(crate) unsafe extern "thiscall" fn comparar_league_get_club_array(this: usize, out: *mut usize, n: *mut i16) {
    original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, *mut i16)>(IDX_LEAGUE_GET_CLUB_ARRAY)(this, out, n);
    let p = *out;
    if p == 0 {
        return; // sem memória: a caixa já saiu
    }
    let count = read_i16(this + 0x3e);
    let ok = *n == count && (0..count.max(0) as usize).all(|i| read_usize(p + 4 * i) == read_usize(read_usize(this + 0xb1) + i * TABLE_ROW_SIZE));
    PLACAR.registrar(ok, || format!("league_get_club_array conta {} (tabela {})", *n, count));
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
pub(crate) unsafe extern "thiscall" fn comparar_apply_result_to_table(league: usize, p: usize, table: usize, n: u32, mark: u32, pts_home: *mut i16, pts_away: *mut i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, *mut i16, *mut i16)>(IDX_APPLY_RESULT_TO_TABLE);
    let reg = result_update_regions(p, table, n, pts_home, pts_away);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(league, p, table, n, mark, pts_home, pts_away),
        || apply_result_to_table(league, p, table, n, mark, pts_home, pts_away));
    PLACAR.registrar(igual, || format!("apply_result_to_table({:x}, partida {:x}, {:x}, n {}, marcar {}) {:x?} x {:x?}",
        league, p, table, n as i16, mark, mo, mr));
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
pub(crate) unsafe extern "thiscall" fn comparar_sort_table(league: usize, rows: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SORT_TABLE);
    let reg = sort_regions(league, rows);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(league, rows), || sort_table(league, rows));
    PLACAR.registrar(igual && r == n, || format!("sort_table({:x}, {:x}) {}/{} {:x?} x {:x?}", league, rows, r, n, mo, mr));
    r
}

/// Comparer for `build_h2h_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_build_h2h_table(league: usize, rows: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_BUILD_H2H_TABLE);
    if rows == 0 {
        return o(league, rows);
    }
    let reg = [(rows, LEAGUE_TABLE_ROW_SIZE * read_i16(league + 0x3e).max(0) as usize)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(league, rows), || build_h2h_table(league, rows));
    PLACAR.registrar(igual, || format!("build_h2h_table({:x}, {:x}) {:x?} x {:x?}", league, rows, mo, mr));
}

/// Comparer for `free_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_free_table(league: usize, table: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FREE_TABLE)(league, table)
}

/// Comparer for `league_check_complete` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_league_check_complete(league: usize, stages: u32, warn: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(IDX_LEAGUE_CHECK_COMPLETE);
    if !is_complete_check_read_only(league, stages) {
        return o(league, stages, warn); // fim de liga (registros, prêmios, fases, aviso): só a original
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(league + 0xa7, 2)], || o(league, stages, warn), || league_check_complete(league, stages, warn));
    PLACAR.registrar(igual && r == n, || format!("league_check_complete({:x}, {}, {}) {}/{} {:x?} x {:x?}", league, stages, warn, r, n, mo, mr));
    r
}

/// Comparer for `pay_tv_rights` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: as finanças (0x167 bytes) de cada clube da tabela; a cópia roda sem a notícia (a da original fica, uma
/// vez só). Clube nulo na tabela (a caixa da 0x5a0590): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_pay_tv_rights(league: usize, value: i32, filtro: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, usize)>(IDX_PAY_TV_RIGHTS);
    let finances = *(G_FINANCE_TABLE as *const usize);
    let n = read_i16(league + 0x3e).max(0) as usize;
    let clubs: Vec<usize> = (0..n).map(|i| read_usize(read_usize(league + 0xb1) + 0x3b * i)).collect();
    if finances == 0 || clubs.contains(&0) {
        return o(league, value, filtro);
    }
    let get_club_finance: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let reg: Vec<(usize, usize)> = clubs.iter().map(|&c| (get_club_finance(finances, c), 0x167)).collect();
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(league, value, filtro), || pay_tv_money(league, value, filtro, false));
    PLACAR.registrar(igual, || format!("pay_tv_rights({:x}, {}, {:x}) {:x?} x {:x?}", league, value, filtro, mo, mr));
}

/// Comparer for `league_record_season_honours` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: os clubes (o +0x5b) e o histórico de competições. Sem linha livre no histórico (a 0x4afce0 faria
/// realloc): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_league_record_season_honours(league: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_LEAGUE_RECORD_SEASON_HONOURS);
    if free_comp_history_row() == 0 {
        return o(league);
    }
    let mut reg = comp_history_regions();
    let how_many = (*(G_CLUB_COUNT as *const i32)).wrapping_sub(*(G_NATION_COUNT as *const i32)).max(0) as usize;
    reg.push((*(G_CLUB_TABLE as *const usize), 0x245 * how_many));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(league), || league_record_season_honours(league));
    PLACAR.registrar(igual && r == n, || format!("league_record_season_honours({:x}) {:x}/{:x} {:x?} x {:x?}", league, r, n, mo, mr));
    r
}

/// Comparer for `has_relegation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_has_relegation(league: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HAS_RELEGATION)(league);
    let n = has_relegation(league);
    PLACAR.registrar(r == n, || format!("has_relegation original={} rust={}", r, n));
    r
}

/// Comparer for `league_round_set_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_league_round_set_date(rounds: usize, i: u32, day: u32, month: u32, years: u32, week: u32, code: u32, base: u32, value: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize>(IDX_LEAGUE_ROUND_SET_DATE);
    let r = rounds.wrapping_add((i as i16 as isize * 0x41) as usize);
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r, 0x41)], || o(rounds, i, day, month, years, week, code, base, value),
        || league_round_set_date(rounds, i, day, month, years, week, code, base, value));
    PLACAR.registrar(igual && a == b, || format!("league_round_set_date({}, {}/{} +{}) {:x?} x {:x?}", i as i16, day, month, years, mo, mr));
    a
}

/// Comparer for `get_match_importance` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_match_importance(league: usize, partida: usize, side: u32, out: *mut u8) -> u8 {
    // a 0x68a4d0 só lê a liga e grava na local e na `saida`: pode rodar nos dois lados
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut u8) -> u8>(IDX_GET_MATCH_IMPORTANCE);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(out as usize, 1)], || o(league, partida, side, out),
        || get_match_importance(league, partida, side, out));
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
    let regions: Vec<(usize, usize)> = (1..=n.max(0)).map(|i| (*grade.offset(i as isize) as usize + 4, (n.max(1) as usize - 1) * 4)).collect();
    let (r, s, igual, mo, mr) = crate::comparacao::lado_a_lado(&regions, || o(grade, n), || league_build_round_robin_grid(grade, n));
    PLACAR.registrar(igual && r == s, || format!("league_build_round_robin_grid({}) {:x?} x {:x?}", n, mo, mr));
    r
}

/// Comparer for `final_day_margin_needed` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_final_day_margin_needed(league: usize, row: usize, out: *mut u8) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u8>(IDX_FINAL_DAY_MARGIN_NEEDED);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(out as usize, 1)], || o(league, row, out),
        || final_day_margin_needed(league, row, out));
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
pub(crate) unsafe extern "thiscall" fn comparar_update_head_to_head(league: usize, table: usize, row: i32, opponent: i32, pro: i8, contra: i8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, i32, i8, i8)>(IDX_UPDATE_HEAD_TO_HEAD);
    if row < 0 || opponent < 0 {
        return o(league, table, row, opponent, pro, contra);
    }
    let e = head_to_head_record(table, row, opponent);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(e, 14)], || o(league, table, row, opponent, pro, contra),
        || update_head_to_head(league, table, row, opponent, pro, contra));
    PLACAR.registrar(igual, || format!("update_head_to_head({}, {}, {}x{}) {:x?} x {:x?}", row, opponent, pro, contra, mo, mr));
}

/// Comparer for `rotate_round` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_rotate_round(pos: i32, step_towards: *mut i8, league: i32, rounds: i16, turnos: i16, n: i16, marks: u8) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(i32, *mut i8, i32, i16, i16, i16, u8) -> i32>(IDX_ROTATE_ROUND);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((step_towards as usize, 1));
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pos, step_towards, league, rounds, turnos, n, marks), || crate::aleatorio::sem_contar(|| rotate_round(pos, step_towards, league, rounds, turnos, n, marks)));
    PLACAR.registrar(igual && r == v, || format!("rotate_round({}, {}, {}, {}, {}, {:#x}) original={} {:x?} rust={} {:x?}", pos, league, rounds, turnos, n, marks, r, mo, v, mr));
    r
}

/// Comparer for `store_round_match` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_store_round_match(base: usize, round: i16, j: i16, a: i16, b: i8, c: i8, value: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i16, i16, i16, i8, i8, u32)>(IDX_STORE_ROUND_MATCH);
    let e = base.wrapping_add((round as isize * 0x41 + j as isize * 7) as usize);
    let reg = [(e + 5, 7)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(base, round, j, a, b, c, value), || store_round_match(base, round, j, a, b, c, value));
    PLACAR.registrar(igual, || format!("store_round_match({:#x}, {}, {}, {}, {}, {}, {:#x}) {:x?} x {:x?}", base, round, j, a, b, c, value, mo, mr));
}

/// Comparer for `calc_prize_fund` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_calc_prize_fund(league: usize, cota: i32) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> i32>(IDX_CALC_PRIZE_FUND)(league, cota);
    if *((league + 0x3e) as *const i16) == 0 {
        return r;
    }
    let n = calc_prize_fund(league, cota);
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
pub(crate) unsafe extern "thiscall" fn comparar_move_club_to_division(this: usize, clube: usize, division: usize, slot: u32) {
    crate::comparacao::ramo("move_club_to_division");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_MOVE_CLUB_TO_DIVISION)(this, clube, division, slot)
}

/// Comparer for `draw_team_order` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: embaralha com o sorteio e o rand, aloca e chama a tabela virtual: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_draw_team_order(this: usize, clubs: i32) {
    // simplification: sorteia e aloca (não roda duas vezes); a cópia se prova pela carreira (a grade de jogos de toda liga)
    original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_DRAW_TEAM_ORDER)(this, clubs)
}

/// Comparer for `limit_club_division_tier` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: grava só o byte +0x64 dos clubes da tabela: lado a lado nesses bytes.
pub(crate) unsafe extern "thiscall" fn comparar_limit_club_division_tier(this: usize, value: u32, away: u32, inside: u32, baixar: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32)>(IDX_LIMIT_CLUB_DIVISION_TIER);
    let n = *((this + 0x3e) as *const i16);
    let table = read_usize(this + 0xb1);
    let reg: Vec<(usize, usize)> = (0..n.max(0) as usize).map(|i| (read_usize(table + i * 0x3b) + 0x64, 1)).collect();
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, value, away, inside, baixar),
        || limit_club_division_tier(this, value, away, inside, baixar));
    PLACAR.registrar(igual, || format!("limit_club_division_tier(valor {}, fora {:x}, dentro {:x}, baixar {}) {:x?} x {:x?}",
        value as u8 as i8, away as u8, inside as u8, baixar, mo, mr));
}

/// Comparer for `narrate_qualification` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_narrate_qualification(this: usize, side: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_NARRATE_QUALIFICATION);
    if is_recording_top_level() {
        call_recording(FN_CM3_MATCH_ENGINE_POST_PERIOD_END_EVENTS, this, [side, 0, 0, 0, 0], || {
            o(this, side);
            0
        });
        return;
    }
    let (igual, n) = comparar_com_comentarios(|| o(this, side), |c| comment_standing_with(this, side, c));
    PLACAR.registrar(igual, || format!("narrate_qualification({}) {} chamadas", side as u8, n));
}
