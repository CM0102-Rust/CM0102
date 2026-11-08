use super::*;

// The comparers of the routines in ../comp/cup.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `create_tie_fixtures` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x51cb90: só a original (a partida vai ao calendário pela 0x5aa680, que aloca, e as notícias aos
/// técnicos). O plano, feito antes: o confronto escolhido (o −1 procura a marca 1 ou 2), a devolução, a marca (2 ao
/// adiar, 3 ao criar) e, criando com os dois times, os jogos do mandante na competição sobem 1 (2 na de duas mãos com a
/// copa ainda ativa).
pub(crate) unsafe extern "thiscall" fn comparar_create_tie_fixtures(this: usize, rodada: u32, confronto: u32, adiar: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(IDX_CREATE_TIE_FIXTURES);
    let ri = rodada as i16 as i32;
    let id = read_u32_as_usize(read_u32_as_usize(this + 4)) as u32;
    let (mut k, mut ad) = (confronto as i16 as i32, adiar);
    if k == -1 {
        let n = read_i16(round_entry(this, ri) + 0x1a) as i32;
        k = -1;
        for i in 0..n.max(0) {
            let m = read_u8(tie_of_round(this, ri, i) + 0x34);
            if m == 1 || m == 2 {
                k = i;
                ad = (m == 1) as u32;
                break;
            }
        }
        if k == -1 {
            let r = o(this, rodada, confronto, adiar);
            PLACAR.registrar(r == 0, || format!("create_tie_fixtures(rodada {}) sem confronto marcado: {}", ri, r));
            return r;
        }
    }
    let t = tie_of_round(this, ri, k);
    let completo = read_i16(t) != -1 && read_i16(t + 2) != -1;
    let e = round_entry(this, ri);
    let duas = read_u8(e + 0x21) == 2;
    let casa = if completo { read_u32_as_usize(cup_team_entry(this, read_i16(t))) } else { 0 };
    let ic = if casa != 0 { read_u32_as_usize(casa) as u32 } else { u32::MAX };
    let conferir = ad == 0 && completo && ic != u32::MAX;
    // o fim da janela: a data da rodada (+ os dias até a volta), amanhã ou a data da rodada seguinte (+0x6b/+0x6d), o
    // que vier depois, + 14 dias. Com conflito, a 0x5aa680 remarca o jogo (0x5ab510): copa (tipo 2) até a data da
    // rodada seguinte (sem ela, 2 semanas), os outros tipos até 2 semanas depois; com mais ligas acontece (a 1a rodada
    // da copa 101, 2001-225, foi a mais de 7 dias)
    let ida = (read_u8(e + 5) as i8 as i32 + read_i16(this + 0x40) as i32, read_i16(e + 3) as i32);
    let ultimo = if duas { add_days_to_day_year(ida, read_u8(e + 0x22) as i32) } else { ida };
    let amanha = add_days_to_day_year((read_i16(G_CURRENT_DATE_YEAR) as u16 as i32, read_i16(G_CURRENT_DATE) as i32), 1);
    let seguinte = if ri < read_i16(this + 0x3c) as i32 - 1 {
        (read_u8(e + 0x6d) as i8 as i32 + read_i16(this + 0x40) as i32, read_i16(e + 0x6b) as i32)
    } else {
        ultimo
    };
    let fim = add_days_to_day_year(ultimo.max(amanha).max(seguinte), 14);
    let antes = if conferir { club_fixtures_until(ic, id, fim) } else { 0 };
    let r = o(this, rodada, confronto, adiar);
    let marca = read_u8(t + 0x34);
    let mut ok = r == 1 && marca == if ad != 0 { 2 } else { 3 };
    let mut esperado = 0;
    let mut depois = 0;
    if conferir && read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + id as usize * 4) != 0 {
        esperado = 1 + duas as u32;
        depois = club_fixtures_until(ic, id, fim);
        ok = ok && depois == antes + esperado;
    }
    PLACAR.registrar(ok, || format!("create_tie_fixtures(rodada {}, confronto {}, adiar {}) devolveu {} marca {} jogos {} -> {} (esperado +{})",
        ri, k, ad, r, marca, antes, depois, esperado));
    r
}

/// Comparer for `post_draw_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x522950: só a original (a notícia vai aos técnicos e não tem outro efeito).
pub(crate) unsafe extern "thiscall" fn comparar_post_draw_news(this: usize, rodada: u32, confronto: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_POST_DRAW_NEWS)(this, rodada, confronto) // simplification: só a notícia
}

/// Comparer for `sort_round_teams` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sorteia (0x90cfc0) e só usa vetores temporários: lado a lado pelo estado do sorteador e pelos
/// confrontos de todas as rodadas (os vínculos vão às rodadas seguintes), a cópia em sem_contar.
pub(crate) unsafe extern "thiscall" fn comparar_sort_round_teams(copa: usize, rodada: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SORT_ROUND_TEAMS);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend(all_ties_regions(copa));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada),
        || crate::aleatorio::sem_contar(|| sort_round_teams(copa, rodada)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("sort_round_teams(rodada {}) diferencas em {:x?}", rodada as i16, dif)
    });
}

/// Comparer for `check_round_teams_for_nation` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: como o da 0x51d2d0 (sorteia; as contas e o vetor são temporários).
pub(crate) unsafe extern "thiscall" fn comparar_check_round_teams_for_nation(copa: usize, rodada: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_CHECK_ROUND_TEAMS_FOR_NATION);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend(all_ties_regions(copa));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada),
        || crate::aleatorio::sem_contar(|| check_round_teams_for_nation(copa, rodada)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("check_round_teams_for_nation(rodada {}) diferencas em {:x?}", rodada as i16, dif)
    });
}

/// Comparador: lado a lado no byte +0xb1.
pub(crate) unsafe extern "thiscall" fn comparar_cup_start_new_season(copa: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CUP_START_NEW_SEASON);
    let reg = [(copa + 0xb1, 1)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa), || cup_start_new_season(copa));
    PLACAR.registrar(igual, || format!("cup_start_new_season({:#x}) {:x?} x {:x?}", copa, mo, mr));
}

/// Comparer for `pay_round_prize_money` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x522240, em ensaio: a cópia sem a notícia e a original inteira, a partir do mesmo estado (o fundo e
/// os registros de finanças dos dois clubes, restaurados depois do ensaio); confere o fundo e as finanças. Sem clube,
/// sem finanças ou com a divisão que cai: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_pay_round_prize_money(copa: usize, vencedor: usize, perdedor: usize, rodada: u32, partida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize)>(IDX_PAY_ROUND_PRIZE_MONEY);
    let (cv, cp) = (read_usize(vencedor), read_usize(perdedor));
    if cv == 0 || cp == 0 || cv == cp || round_prizes(copa, rodada as i16 as i32, false).is_none() {
        return o(copa, vencedor, perdedor, rodada, partida); // simplification: sem clube ou com a divisão que cai, só a original
    }
    let (fv, fp) = (european_cup_club_finance(cv), european_cup_club_finance(cp));
    if fv == 0 || fp == 0 {
        return o(copa, vencedor, perdedor, rodada, partida); // simplification: sem finanças, só a original
    }
    let regioes = [(copa + 0x28, 4), (fv, 0x167), (fp, 0x167)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    ROUND_PRIZE_MONEY_DRY_RUN = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| pay_round_prize_money(copa, vencedor, perdedor, rodada, partida)));
    ROUND_PRIZE_MONEY_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(copa, vencedor, perdedor, rodada, partida);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("pay_round_prize_money(rodada {}, vencedor {:x}, perdedor {:x}) o fundo e as finanças diferem", rodada as i16, cv, cp));
}

/// Comparer for `set_home_teams_for_round` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x51e870: lado a lado nos confrontos percorridos (a virtual +0x90 só consulta e roda nos dois lados).
pub(crate) unsafe extern "thiscall" fn comparar_set_home_teams_for_round(copa: usize, rodada: u32, laco: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_SET_HOME_TEAMS_FOR_ROUND);
    let (ini, fim) = ties_range(copa, rodada, laco);
    let mut reg = Vec::new();
    if ini < fim {
        reg.push((round_tie(copa, rodada as i16 as i32, ini as i32), (fim as i32 - ini as i32) as usize * 0x35));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada, laco), || set_home_teams_for_round(copa, rodada, laco));
    PLACAR.registrar(igual, || format!("set_home_teams_for_round(rodada {}, laco {}) original={:x?} rust={:x?}", rodada as i16, laco as i16, mo, mr));
}

/// Comparer for `record_team_progress` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x521b30: lado a lado nos 3 bytes de progresso (+0x73..+0x75) do registro do clube no Comp_Stats da
/// copa (+8, achado pela 0x4a4850: push clube; ecx = as estatísticas; thiscall, ret 4), que a +0x68 grava; as versões
/// que repassam a outra competição gravam fora da região (aí a comparação só confere que nada mudou nela). Sem
/// estatísticas, sem clube ou sem registro: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_record_team_progress(copa: usize, time: u32, rodada: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_RECORD_TEAM_PROGRESS);
    let stats = read_usize(copa + 8);
    let clube = read_usize(team_entry(copa, time as i16 as i32));
    let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let reg = if stats != 0 && clube != 0 { busca(stats, clube) } else { 0 };
    if reg == 0 {
        return o(copa, time, rodada); // simplification: sem registro para olhar, só a original
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(reg + 0x73, 3)], || o(copa, time, rodada), || record_team_progress(copa, time, rodada));
    PLACAR.registrar(igual, || format!("record_team_progress(time {}, rodada {}) original={:x?} rust={:x?}", time as i16, rodada as i16, mo, mr));
}

/// Comparer for `draw_round` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x51c410: só a original (os sorteios sorteiam, a notícia aloca). O plano, feito antes: sem sorteio
/// (sem +0x45 ou já sorteada), nada muda nos confrontos nem nas datas; com time em aberto, as datas previstas das
/// rodadas (a desta mais 1 dia, as seguintes no mínimo essa); sorteada, todos os confrontos com +0x34 = 1 e +0xac com
/// a soma prevista (as consultas da notícia não mudam com o sorteio).
pub(crate) unsafe extern "thiscall" fn comparar_draw_round(copa: usize, rodada: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_DRAW_ROUND);
    if read_usize(copa + 0x45) == 0 || read_u8(round_ties_array(copa, rodada) + 0x34) != 0 {
        return o(copa, rodada); // simplification: nada a sortear, a original não faz nada
    }
    let n = read_i16(copa + 0x3c);
    if round_has_open_team(copa, rodada) {
        // o plano das datas: (registro, palavra do dia, byte do ano) de cada rodada daqui em diante
        let mut a = [0u8; 8];
        let (mut dia, mut ano) = (read_i16(round_record_by_dword(copa, rodada)), (read_u8(round_record_by_dword(copa, rodada) + 2) as i8 as i16).wrapping_add(read_i16(copa + 0x40)) as u16);
        // a 0x54c5b0 (mais 1): normalizar para a frente, 365 dias no bissexto (crate::data::bissexto)
        dia = dia.wrapping_add(1);
        let tam = |y: u16| if crate::data::bissexto(y) == 1 { 365i16 } else { 364 };
        while dia > tam(ano) {
            dia = dia.wrapping_sub(tam(ano) + 1);
            ano = ano.wrapping_add(1);
        }
        a[0..2].copy_from_slice(&dia.to_le_bytes());
        a[2..4].copy_from_slice(&ano.to_le_bytes());
        let mut plano = vec![(round_record_by_dword(copa, rodada), dia, a[2].wrapping_sub(read_u8(copa + 0x40)))];
        let mut k = rodada.wrapping_add(1);
        while (k as i16) < n {
            let rk = round_record_by_dword(copa, k as i16 as i32 as u32);
            let mut b = [0u8; 8];
            let anob = (read_u8(rk + 2) as i8 as i16).wrapping_add(read_i16(copa + 0x40)) as u16;
            b[0..2].copy_from_slice(&read_i16(rk).to_le_bytes());
            b[2..4].copy_from_slice(&anob.to_le_bytes());
            plano.push(if cup_date_is_after(&a, &b) { (rk, dia, a[2].wrapping_sub(read_u8(copa + 0x40))) } else { (rk, read_i16(rk), read_u8(rk + 2)) });
            k = k.wrapping_add(1);
        }
        let r = o(copa, rodada);
        let ok = plano.iter().all(|&(rk, d, y)| read_i16(rk) == d && read_u8(rk + 2) == y);
        PLACAR.registrar(ok, || format!("draw_round(rodada {}) as datas adiadas diferem do plano", rodada as i32));
        return r;
    }
    let antes_ac = read_u8(copa + 0xac);
    let r = o(copa, rodada);
    let mut ok = r == 1;
    let reg = round_record_by_dword(copa, rodada);
    for k in 0..read_i16(reg + 0x1a).max(0) as usize {
        ok &= read_u8(round_ties_array(copa, rodada) + k * 0x35 + 0x34) == 1;
    }
    let soma = if !skips_draw_news(copa, rodada) && read_i16(reg + 7) != 0x8c { 1 } else { 0 };
    ok &= read_u8(copa + 0xac) == antes_ac.wrapping_add(soma);
    PLACAR.registrar(ok, || format!("draw_round(rodada {}) marcas dos confrontos ou +0xac ({} -> {}) diferem do plano", rodada as i32, antes_ac, read_u8(copa + 0xac)));
    r
}

/// Comparer for `club_in_higher_division` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x522360: pelo valor (só lê).
pub(crate) unsafe extern "thiscall" fn comparar_club_in_higher_division(this: usize, a: usize, b: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(IDX_CLUB_IN_HIGHER_DIVISION)(this, a, b);
    let n = club_in_higher_division(this, a, b);
    PLACAR.registrar(r == n, || format!("club_in_higher_division({:x}, {:x}) original={:x} rust={:x}", a, b, r, n));
    r
}

/// Comparer for `cup_update_staff_records_for_status` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x519890: só a original (os registros vão ao histórico das pessoas e mexem nas reputações e no moral,
/// não podem rodar duas vezes). O plano, feito antes: o estado do clube (+0 do registro de 6 bytes) passa à Copa da
/// UEFA quando o campeão ganha a vaga; senão não muda. Sem clube: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_cup_update_staff_records_for_status(copa: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, final_: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS);
    if clube == 0 || jogo == 0 {
        return o(copa, clube, desfecho, a3, a4, jogo, final_); // simplification: sem clube ou jogo, só a original
    }
    let estado = club_state_record(clube);
    let antes = read_i32(estado);
    let previsto = if read_u8(jogo + 0x3a) != 1 && read_usize(copa + 0x2c) == 0 && read_u8(copa + 0x44) == 0xff && desfecho as u8 == 1
        && wins_uefa_cup_place(read_usize(copa + 4), clube) { read_i32(KEY_CLUB_COMP_UEFA_CUP) } else { antes };
    let r = o(copa, clube, desfecho, a3, a4, jogo, final_);
    let depois = read_i32(estado);
    PLACAR.registrar(r == 0 && depois == previsto, || format!("cup_update_staff_records_for_status(desfecho {}) estado {:x} -> {:x}, previsto {:x}, devolveu {}", desfecho as u8, antes, depois, previsto, r));
    r
}

/// Comparer for `cup_record_season_honours` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x519a90: só a original (a 0x4afce0 pode crescer o histórico e a 0x519fc0 tem o seu comparador). O
/// plano, feito antes: a linha livre do histórico (free_comp_history_row) recebe a competição (+4), o ano [0xae2c92] e os
/// quatro de season_honours. Sem linha livre (o histórico cresce) ou sem a competição: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_cup_record_season_honours(copa: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CUP_RECORD_SEASON_HONOURS);
    let linha = free_comp_history_row();
    if linha == 0 || read_usize(copa + 4) == 0 {
        return o(copa); // simplification: o histórico cresce (realloc) ou não há competição: só a original
    }
    let (campeao, vice, terceiro, anfitria) = season_honours(copa);
    let ano = core::ptr::read_unaligned(G_CURRENT_DATE_YEAR as *const u16);
    o(copa);
    let lido = (read_usize(linha + 4), core::ptr::read_unaligned((linha + 8) as *const u16), read_usize(linha + 0xa), read_usize(linha + 0xe), read_usize(linha + 0x12), read_usize(linha + 0x16));
    let previsto = (read_usize(copa + 4), ano, campeao, vice, terceiro, anfitria);
    PLACAR.registrar(lido == previsto, || format!("cup_record_season_honours: linha {:x?} prevista {:x?}", lido, previsto));
}

/// Comparer for `register_semi_final_losers_for_uefa` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x519fc0: lado a lado na tabela da semeadura (+0x440, 50 vagas de 12 bytes, onde a 0x905790 grava; ela
/// não aloca: acha a vaga da competição ou a primeira livre). Sem a semeadura: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_register_semi_final_losers_for_uefa(copa: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_REGISTER_SEMI_FINAL_LOSERS_FOR_UEFA);
    let s = read_usize(G_UEFA_SEEDING);
    if s == 0 {
        return o(copa); // simplification: sem a semeadura, só a original
    }
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&[(s + 0x440, 50 * 12)], || o(copa), || register_semi_final_losers_for_uefa(copa));
    PLACAR.registrar(igual, || format!("register_semi_final_losers_for_uefa(copa {:x}) a semeadura difere ({:?})", copa, semi_final_losers(copa)));
}

/// Comparer for `get_fixture_side_teams` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o esperado sai do núcleo sobre uma conta própria e uma saída própria antes da original; depois, a
/// devolução, a conta global e a saída têm de bater.
pub(crate) unsafe extern "thiscall" fn comparar_get_fixture_side_teams(this: usize, rodada: i32, jogo: i32, lado: u32, saida: *mut i16) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, u32, *mut i16) -> i32>(IDX_GET_FIXTURE_SIDE_TEAMS);
    if saida.is_null() || rodada < 0 || rodada > read_i16(this + 0x3c) as i32 || read_u32_as_usize(this + 0xa7) == 0 || jogo == -1 {
        return o(this, rodada, jogo, lado, saida); // simplification: a caixa, só a original
    }
    let reg = read_u32_as_usize(read_u32_as_usize(this + 0xad) + rodada as usize * 4) + (jogo as usize).wrapping_mul(0x35);
    let (c, r, j) = if lado as u8 != 0 { (read_i16(reg), reg + 0xe, reg + 0xa) } else { (read_i16(reg + 2), reg + 0x10, reg + 0xc) };
    let mut esperado = vec![0i16; 0x100];
    let mut n: i32 = 0;
    if c != -1 {
        esperado[0] = c;
        n = 1;
    } else {
        collect_fixture_teams(this, read_i16(r) as i32, read_i16(j) as i32, esperado.as_mut_ptr(), &mut n, false);
    }
    let res = o(this, rodada, jogo, lado, saida);
    let obtido = core::slice::from_raw_parts(saida, res.clamp(0, 0x100) as usize);
    let global = *(G_CUP_POSSIBLE_TEAM_COUNT as *const i32);
    PLACAR.registrar(res == n && global == n && obtido == &esperado[..n.clamp(0, 0x100) as usize], || format!("get_fixture_side_teams({:x}, {}, {}, {}) {}/{} global {} {:?} x {:?}", this, rodada, jogo, lado as u8, res, n, global, obtido, &esperado[..n.clamp(0, 0x100) as usize]));
    res
}

/// Comparer for `collect_possible_teams` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o esperado sai do núcleo sobre uma cópia da conta e uma saída própria antes da original; depois, a
/// devolução, a conta e os clubes acrescentados têm de bater.
pub(crate) unsafe extern "thiscall" fn comparar_collect_possible_teams(this: usize, rodada: i32, jogo: i32, saida: *mut i16, conta: *mut i32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, *mut i16, *mut i32) -> i32>(IDX_COLLECT_POSSIBLE_TEAMS);
    if saida.is_null() || conta.is_null() || rodada < 0 || jogo == -1 || !(0..0x100).contains(&*conta) {
        return o(this, rodada, jogo, saida, conta); // simplification: a caixa ou conta fora do esperado, só a original
    }
    let c0 = *conta;
    let mut esperado = vec![0i16; 0x200];
    let mut n = c0;
    collect_fixture_teams(this, rodada, jogo, esperado.as_mut_ptr(), &mut n, false);
    let res = o(this, rodada, jogo, saida, conta);
    let fim = (*conta).clamp(c0, 0x200);
    let obtido = core::slice::from_raw_parts(saida.offset(c0 as isize), (fim - c0) as usize);
    let ok = res == n && *conta == n && obtido == &esperado[c0 as usize..n.clamp(c0, 0x200) as usize];
    PLACAR.registrar(ok, || format!("collect_possible_teams({:x}, {}, {}) {}/{} conta {} {:?} x {:?}", this, rodada, jogo, res, n, *conta, obtido, &esperado[c0 as usize..n.clamp(c0, 0x200) as usize]));
    res
}

/// Comparer for `cup_update_for_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_cup_update_for_date(this: usize, hoje: usize, fase: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_CUP_UPDATE_FOR_DATE)(this, hoje, fase)
}

/// Comparer for `set_up_round` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_up_round(this: usize, rodada: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_SET_UP_ROUND)(this, rodada)
}

/// Comparer for `randomize_round_teams` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sorteia (0x90cfc0) e só usa um vetor temporário: lado a lado pelo estado do sorteador e pelos
/// confrontos de todas as rodadas (os vínculos vão às rodadas seguintes), a cópia em sem_contar.
pub(crate) unsafe extern "thiscall" fn comparar_randomize_round_teams(copa: usize, rodada: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_RANDOMIZE_ROUND_TEAMS);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend(all_ties_regions(copa));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada),
        || crate::aleatorio::sem_contar(|| randomize_round_teams(copa, rodada)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("randomize_round_teams(rodada {}) diferencas em {:x?}", rodada as i16, dif)
    });
}

/// Comparer for `separate_shared_stadium_ties` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: só grava confrontos (da rodada e das seguintes): lado a lado pelos confrontos de todas as rodadas.
pub(crate) unsafe extern "thiscall" fn comparar_separate_shared_stadium_ties(copa: usize, rodada: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SEPARATE_SHARED_STADIUM_TIES);
    let reg = all_ties_regions(copa);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada), || separate_shared_stadium_ties(copa, rodada));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("separate_shared_stadium_ties(rodada {}) diferencas em {:x?}", rodada as i16, dif)
    });
}

/// Comparer for `cup_compare_entries_by_level_desc` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn comparar_cup_compare_entries_by_level_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_CUP_COMPARE_ENTRIES_BY_LEVEL_DESC)(a, b);
    let n = cup_compare_entries_by_level_desc(a, b);
    PLACAR.registrar(r == n, || format!("cup_compare_entries_by_level_desc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `cup_compare_entries_by_level_asc` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn comparar_cup_compare_entries_by_level_asc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_CUP_COMPARE_ENTRIES_BY_LEVEL_ASC)(a, b);
    let n = cup_compare_entries_by_level_asc(a, b);
    PLACAR.registrar(r == n, || format!("cup_compare_entries_by_level_asc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `decide_fixture_winner` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelos códigos (+0x3d/+0x3e) e pelo lado devolvido; jogo nulo ou empate nos pênaltis (a
/// caixa): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_decide_fixture_winner(this: usize, jogo: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_DECIDE_FIXTURE_WINNER);
    if jogo == 0 {
        return o(this, jogo);
    }
    let reg = [(jogo + 0x3d, 2)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || Some(o(this, jogo)), || fixture_winner_side(this, jogo));
    let r = r.unwrap_or(0);
    if let Some(n) = n {
        PLACAR.registrar(igual && r == n, || format!("decide_fixture_winner(regra {:x}) original={} rust={} {:x?} x {:x?}", read_i16(jogo + 0x38) as u16, r, n, mo, mr));
    }
    r
}

/// Comparer for `decide_tie_winner` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelos códigos (+0x3d/+0x3e) e pelo lado devolvido; empate nos pênaltis (a caixa): só a
/// original.
pub(crate) unsafe extern "thiscall" fn comparar_decide_tie_winner(this: usize, confronto: usize, jogo: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_DECIDE_TIE_WINNER);
    let reg = [(jogo + 0x3d, 2)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || Some(o(this, confronto, jogo)), || tie_winner_side(confronto, jogo));
    let r = r.unwrap_or(0);
    if let Some(n) = n {
        PLACAR.registrar(igual && r == n, || format!("decide_tie_winner(regra {:x}, jogo {}) original={} rust={} {:x?} x {:x?}",
            read_i16(jogo + 0x38) as u16, read_i8(confronto + 0x33), r, n, mo, mr));
    }
    r
}

/// Comparer for `cup_compare_draw_entries_by_seed` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "C" fn comparar_cup_compare_draw_entries_by_seed(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_CUP_COMPARE_DRAW_ENTRIES_BY_SEED)(a, b);
    let n = cup_compare_draw_entries_by_seed(a, b);
    PLACAR.registrar(r == n, || format!("cup_compare_draw_entries_by_seed({}, {}) original={} rust={}", read_i8(a + 6), read_i8(b + 6), r, n));
    r
}

/// Comparer for `cup_set_round_draw_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_cup_set_round_draw_date(rodadas: usize, rodada: u32, dia: u32, mes: u32, anos: u32, semana: u32, base: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize>(IDX_CUP_SET_ROUND_DRAW_DATE);
    let r = round_record_from_base(rodadas, rodada);
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r, 3)], || o(rodadas, rodada, dia, mes, anos, semana, base),
        || cup_set_round_draw_date(rodadas, rodada, dia, mes, anos, semana, base));
    PLACAR.registrar(igual && a == b, || format!("cup_set_round_draw_date({}, {}/{} +{}) {:x?} x {:x?}", rodada as i16, dia, mes, anos, mo, mr));
    a
}

/// Comparer for `cup_set_round_match_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_cup_set_round_match_date(rodadas: usize, rodada: u32, dia: u32, mes: u32, anos: u32, semana: u32, hora: u32, base: u32, mando: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize>(IDX_CUP_SET_ROUND_MATCH_DATE);
    let r = round_record_from_base(rodadas, rodada);
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r + 3, 0x14)], || o(rodadas, rodada, dia, mes, anos, semana, hora, base, mando),
        || cup_set_round_match_date(rodadas, rodada, dia, mes, anos, semana, hora, base, mando));
    PLACAR.registrar(igual && a == b, || format!("cup_set_round_match_date({}, {}/{} +{}) {:x?} x {:x?}", rodada as i16, dia, mes, anos, mo, mr));
    a
}

/// Comparer for `apply_round_rules_to_match` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_apply_round_rules_to_match(copa: usize, jogo: usize, chave: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32)>(IDX_APPLY_ROUND_RULES_TO_MATCH);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(jogo + 0x38, 2)], || o(copa, jogo, chave),
        || apply_round_rules_to_match(copa, jogo, chave));
    PLACAR.registrar(igual, || format!("apply_round_rules_to_match(chave {}) {:x?} x {:x?}", chave, mo, mr));
}

/// Comparer for `find_tree_slot_for_fixture` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_find_tree_slot_for_fixture(copa: usize, jogo: usize) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i32>(IDX_FIND_TREE_SLOT_FOR_FIXTURE)(copa, jogo);
    if jogo == 0 {
        return r;
    }
    let n = find_tree_slot_for_fixture(copa, jogo);
    PLACAR.registrar(r == n, || format!("find_tree_slot_for_fixture original={} rust={}", r, n));
    r
}

/// Comparer for `cup_match_importance` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_cup_match_importance(copa: usize, partida: usize, segundo: u8, margem: *mut u8) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u8, *mut u8) -> u8>(IDX_CUP_MATCH_IMPORTANCE);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(margem as usize, 1)], || o(copa, partida, segundo, margem), || cup_match_importance(copa, partida, segundo, margem));
    PLACAR.registrar(igual && r == n, || format!("cup_match_importance(lado {}) original={} rust={} {:x?} x {:x?}", segundo, r, n, mo, mr));
    r
}

/// Comparer for `add_team_result_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_team_result_news(copa: usize, clube: usize, desfecho: u32, a3: u32, p: usize, a5: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32>(IDX_ADD_TEAM_RESULT_NEWS)(copa, clube, desfecho, a3, p, a5)
}

/// Comparer for `cup_check_complete` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (o método das fases só lê).
pub(crate) unsafe extern "thiscall" fn comparar_cup_check_complete(this: usize, fases: u32, x: u32) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(IDX_CUP_CHECK_COMPLETE)(this, fases, x);
    let n = cup_check_complete(this, fases, x);
    PLACAR.registrar(r == n, || format!("cup_check_complete({:x}, {}) original={} rust={}", this, fases, r, n));
    r
}

/// Comparer for `move_round_fixtures_to_set_dates` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x51ebe0: só a original (os jogos saem do calendário e voltam pela 0x5aa680, que aloca, e as notícias
/// vão aos técnicos). O plano, feito antes: os jogos da copa na data e fatia (0x5abd90 numa lista provisória), na
/// ordem da lista; cada um do tipo da liga ocupa a vaga seguinte (de 7 em 7 bytes, até 0x38 e até a primeira 0xff), e
/// a vaga do que está na data da rodada fica 0xff; as outras vagas não mudam. Sem a copa em [0xadadfc] depois: nada a
/// conferir.
pub(crate) unsafe extern "thiscall" fn comparar_move_round_fixtures_to_set_dates(this: usize, data: usize, rodada: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_MOVE_ROUND_FIXTURES_TO_SET_DATES);
    let e = cup_round_entry(this, rodada);
    if read_i16(e + 0xd) as u16 & 0x100 == 0 || read_u8(e + 0x29) == 0xff {
        return o(this, data, rodada);
    }
    let id = read_u32_as_usize(read_u32_as_usize(this + 4));
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    criar(l);
    let mut esperado = [0u8; 8];
    for k in 0..8 {
        esperado[k] = read_u8(e + k * 7 + 0x29);
    }
    if collect_round_fixtures(this, data, rodada, l) != 0 {
        write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
        let mut j = 0usize;
        let mut f = proximo(l);
        while f != 0 {
            if read_u8(this + 0x44) == read_u8(f + 0x42) && j < 0x38 && esperado[j / 7] != 0xff {
                if fixture_on_round_date(this, e, f) {
                    esperado[j / 7] = 0xff;
                }
                j += 7;
            }
            f = proximo(l);
        }
    }
    destruir(l);
    o(this, data, rodada);
    if read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + id * 4) == 0 {
        return; // simplification: a copa saiu do jogo; nada a conferir
    }
    let e = cup_round_entry(this, rodada);
    let depois: [u8; 8] = core::array::from_fn(|k| read_u8(e + k * 7 + 0x29));
    PLACAR.registrar(depois == esperado, || format!("move_round_fixtures_to_set_dates(rodada {}) vagas {:02x?} (esperado {:02x?})",
        rodada, depois, esperado));
}

/// Comparer for `map_fixture_tree` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da árvore dos confrontos: só a original (simplification: aloca os vetores dos confrontos e sorteia; não
/// roda duas vezes). Confere o que ela garante quando devolve 1 com clubes: cada confronto das rodadas mapeadas aponta
/// (+6 vaga, +8 lado, +0x12 rodada uma ou duas adiante) uma vaga que aponta de volta para ele (+0xa/+0xe mandante,
/// +0xc/+0x10 visitante).
pub(crate) unsafe extern "thiscall" fn comparar_map_fixture_tree(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_MAP_FIXTURE_TREE);
    let r = o(this);
    if r != 1 || read_u32_as_usize(this + 0x45) == 0 {
        return r;
    }
    let nr = read_i16(this + 0x3c);
    let m = (if read_i16(comp_cup_round(this, nr as i32).wrapping_sub(0x61)) == 0x8c { nr.wrapping_sub(1) } else { nr }) as i32;
    let mut erro: Option<(i32, i32)> = None;
    'fora: for rd in 0..(m - 1).max(0) {
        for p in 0..read_i16(comp_cup_round(this, rd) + 0x1a) as i32 {
            let c = cup_tie(this, rd, p);
            let destino = read_i16(c + 0x12) as i32;
            let a = read_i16(c + 8) == 1;
            let ok = (destino == rd + 1 || destino == rd + 2) && {
                let v = cup_tie(this, destino, read_i16(c + 6) as i32);
                read_i16(v + if a { 0xa } else { 0xc }) as i32 == p && read_i16(v + if a { 0xe } else { 0x10 }) as i32 == rd
            };
            if !ok {
                erro = Some((rd, p));
                break 'fora;
            }
        }
    }
    PLACAR.registrar(erro.is_none(), || format!("map_fixture_tree: o confronto (rodada, índice) {:?} sem a vaga de volta", erro));
    r
}

/// Comparer for `cup_rank_teams` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: grava e pode realocar a estatística (não roda duas vezes): antes da original, as posições são
/// planejadas sobre uma cópia ordenada do vetor (as chaves num Vec); depois, o registro de cada clube planejado tem
/// de ter em +0x72 a última posição dele.
pub(crate) unsafe extern "thiscall" fn comparar_cup_rank_teams(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CUP_RANK_TEAMS);
    let n = read_i16(this + 0x3e) as i32;
    if read_u32_as_usize(this + 8) == 0 || n <= 0 {
        return o(this); // simplification: a caixa ou nada a ordenar, só a original
    }
    let mut v: Vec<u32> = (0..n as usize).map(|i| read_u32_as_usize(read_u32_as_usize(this + 0xa7) + i * 6) as u32).collect();
    let mut k = vec![0i32; n as usize];
    sort_clubs_by_strength(v.as_mut_ptr(), k.as_mut_ptr(), n);
    // simplification: com mais vagas nas rodadas que clubes, a original lê além do vetor (sobra do bloco); o plano lê 0
    v.extend(core::iter::repeat(0).take(0x100));
    let plano = set_final_positions_by_round(this, v.as_ptr(), 0);
    o(this);
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let mut erros = Vec::new();
    for (i, &(c, p)) in plano.iter().enumerate() {
        if c == 0 || plano[i + 1..].iter().any(|x| x.0 == c) {
            continue;
        }
        let e = buscar(read_u32_as_usize(this + 8), c);
        if e == 0 || *((e + 0x72) as *const u8) != p {
            erros.push(format!("clube {:x} esperado {} obtido {}", c, p, if e == 0 { -1 } else { *((e + 0x72) as *const u8) as i32 }));
        }
    }
    PLACAR.registrar(erros.is_empty(), || format!("cup_rank_teams({:x}) {} posições: {}", this, plano.len(), erros.join("; ")));
}

/// Comparer for `cup_set_round_match_day` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_cup_set_round_match_day(rodadas: usize, rodada: i16, vaga: i16, semana: i16, hora: i8, codigo: i8, mando: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, i16, i16, i16, i8, i8, u32) -> u32>(IDX_CUP_SET_ROUND_MATCH_DAY);
    if !(0..8).contains(&vaga) || !(-1..=6).contains(&semana) || !(-1..3).contains(&hora) || !(-1..=3).contains(&codigo) {
        return o(rodadas, rodada, vaga, semana, hora, codigo, mando);
    }
    let e = rodadas.wrapping_add((rodada as isize * 104 + vaga as isize * 7) as usize);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(e + 0x23, 7)], || o(rodadas, rodada, vaga, semana, hora, codigo, mando),
        || cup_set_round_match_day(rodadas, rodada, vaga, semana, hora, codigo, mando));
    PLACAR.registrar(igual && r == n, || format!("cup_set_round_match_day({}, {}) {:x?} x {:x?}", rodada, vaga, mo, mr));
    r
}

/// Comparer for `has_club` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "thiscall" fn comparar_has_club(this: usize, clube: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_HAS_CLUB)(this, clube);
    let n = has_club(this, clube);
    PLACAR.registrar(r == n, || format!("has_club(copa {:x}, clube {:x}) original={} rust={}", this, clube, r, n));
    r
}
