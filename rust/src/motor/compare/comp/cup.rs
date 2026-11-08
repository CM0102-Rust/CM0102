use super::*;

// The comparers of the routines in ../comp/cup.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `create_tie_fixtures` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x51cb90: só a original (a partida vai ao calendário pela 0x5aa680, que aloca, e as notícias aos
/// técnicos). O plano, feito antes: o confronto escolhido (o −1 procura a marca 1 ou 2), a devolução, a marca (2 ao
/// adiar, 3 ao criar) e, criando com os dois times, os jogos do mandante na competição sobem 1 (2 na de duas mãos com a
/// copa ainda ativa).
pub(crate) unsafe extern "thiscall" fn comparar_create_tie_fixtures(this: usize, round: u32, tie: u32, adiar: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(IDX_CREATE_TIE_FIXTURES);
    let ri = round as i16 as i32;
    let id = read_u32_as_usize(read_u32_as_usize(this + 4)) as u32;
    let (mut k, mut ad) = (tie as i16 as i32, adiar);
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
            let r = o(this, round, tie, adiar);
            PLACAR.registrar(r == 0, || format!("create_tie_fixtures(rodada {}) sem confronto marcado: {}", ri, r));
            return r;
        }
    }
    let t = tie_of_round(this, ri, k);
    let full = read_i16(t) != -1 && read_i16(t + 2) != -1;
    let e = round_entry(this, ri);
    let duas = read_u8(e + 0x21) == 2;
    let home = if full { read_u32_as_usize(cup_team_entry(this, read_i16(t))) } else { 0 };
    let ic = if home != 0 { read_u32_as_usize(home) as u32 } else { u32::MAX };
    let check = ad == 0 && full && ic != u32::MAX;
    // the end of the window: the round’s date (+ the days until the second leg), tomorrow or the next round’s date (+0x6b/+0x6d), the
    // that comes next, + 14 days. On a clash, 0x5aa680 reschedules the match (0x5ab510): cup (type 2) up to the date of the
    // next round (without it, 2 weeks), the other types up to 2 weeks later; with more leagues it happens (the 1st round
    // of cup 101, 2001-225, went more than 7 days)
    let first_leg = (read_u8(e + 5) as i8 as i32 + read_i16(this + 0x40) as i32, read_i16(e + 3) as i32);
    let last = if duas { add_days_to_day_year(first_leg, read_u8(e + 0x22) as i32) } else { first_leg };
    let amanha = add_days_to_day_year((read_i16(G_CURRENT_DATE_YEAR) as u16 as i32, read_i16(G_CURRENT_DATE) as i32), 1);
    let next = if ri < read_i16(this + 0x3c) as i32 - 1 {
        (read_u8(e + 0x6d) as i8 as i32 + read_i16(this + 0x40) as i32, read_i16(e + 0x6b) as i32)
    } else {
        last
    };
    let end = add_days_to_day_year(last.max(amanha).max(next), 14);
    let before = if check { club_fixtures_until(ic, id, end) } else { 0 };
    let r = o(this, round, tie, adiar);
    let mark = read_u8(t + 0x34);
    let mut ok = r == 1 && mark == if ad != 0 { 2 } else { 3 };
    let mut expected = 0;
    let mut after = 0;
    if check && read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + id as usize * 4) != 0 {
        expected = 1 + duas as u32;
        after = club_fixtures_until(ic, id, end);
        ok = ok && after == before + expected;
    }
    PLACAR.registrar(ok, || format!("create_tie_fixtures(rodada {}, confronto {}, adiar {}) devolveu {} marca {} jogos {} -> {} (esperado +{})",
        ri, k, ad, r, mark, before, after, expected));
    r
}

/// Comparer for `post_draw_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x522950: só a original (a notícia vai aos técnicos e não tem outro efeito).
pub(crate) unsafe extern "thiscall" fn comparar_post_draw_news(this: usize, round: u32, tie: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_POST_DRAW_NEWS)(this, round, tie) // simplification: only the news item
}

/// Comparer for `sort_round_teams` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sorteia (0x90cfc0) e só usa vetores temporários: lado a lado pelo estado do sorteador e pelos
/// confrontos de todas as rodadas (os vínculos vão às rodadas seguintes), a cópia em sem_contar.
pub(crate) unsafe extern "thiscall" fn comparar_sort_round_teams(cup: usize, round: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SORT_ROUND_TEAMS);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend(all_ties_regions(cup));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(cup, round),
        || crate::aleatorio::sem_contar(|| sort_round_teams(cup, round)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("sort_round_teams(rodada {}) diferencas em {:x?}", round as i16, dif)
    });
}

/// Comparer for `check_round_teams_for_nation` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: como o da 0x51d2d0 (sorteia; as contas e o vetor são temporários).
pub(crate) unsafe extern "thiscall" fn comparar_check_round_teams_for_nation(cup: usize, round: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_CHECK_ROUND_TEAMS_FOR_NATION);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend(all_ties_regions(cup));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(cup, round),
        || crate::aleatorio::sem_contar(|| check_round_teams_for_nation(cup, round)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("check_round_teams_for_nation(rodada {}) diferencas em {:x?}", round as i16, dif)
    });
}

/// Comparador: lado a lado no byte +0xb1.
pub(crate) unsafe extern "thiscall" fn comparar_cup_start_new_season(cup: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CUP_START_NEW_SEASON);
    let reg = [(cup + cup::RUN_BY_STAGES, 1)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(cup), || cup_start_new_season(cup));
    PLACAR.registrar(igual, || format!("cup_start_new_season({:#x}) {:x?} x {:x?}", cup, mo, mr));
}

/// Comparer for `pay_round_prize_money` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x522240, em ensaio: a cópia sem a notícia e a original inteira, a partir do mesmo estado (o fundo e
/// os registros de finanças dos dois clubes, restaurados depois do ensaio); confere o fundo e as finanças. Sem clube,
/// sem finanças ou com a divisão que cai: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_pay_round_prize_money(cup: usize, winner: usize, loser: usize, round: u32, partida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize)>(IDX_PAY_ROUND_PRIZE_MONEY);
    let (cv, cp) = (read_usize(winner), read_usize(loser));
    if cv == 0 || cp == 0 || cv == cp || round_prizes(cup, round as i16 as i32, false).is_none() {
        return o(cup, winner, loser, round, partida); // simplification: with no club or with the falling division, original only
    }
    let (fv, fp) = (european_cup_club_finance(cv), european_cup_club_finance(cp));
    if fv == 0 || fp == 0 {
        return o(cup, winner, loser, round, partida); // simplification: without finances, original only
    }
    let regions = [(cup + cup::PRIZE_FUND, 4), (fv, 0x167), (fp, 0x167)];
    let before = crate::comparacao::Memoria::ler(&regions);
    ROUND_PRIZE_MONEY_DRY_RUN = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| pay_round_prize_money(cup, winner, loser, round, partida)));
    ROUND_PRIZE_MONEY_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regions);
    before.escrever();
    o(cup, winner, loser, round, partida);
    let mo = crate::comparacao::Memoria::ler(&regions);
    PLACAR.registrar(mo == mr, || format!("pay_round_prize_money(rodada {}, vencedor {:x}, perdedor {:x}) o fundo e as finanças diferem", round as i16, cv, cp));
}

/// Comparer for `set_home_teams_for_round` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x51e870: lado a lado nos confrontos percorridos (a virtual +0x90 só consulta e roda nos dois lados).
pub(crate) unsafe extern "thiscall" fn comparar_set_home_teams_for_round(cup: usize, round: u32, laco: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_SET_HOME_TEAMS_FOR_ROUND);
    let (ini, fim) = ties_range(cup, round, laco);
    let mut reg = Vec::new();
    if ini < fim {
        reg.push((round_tie(cup, round as i16 as i32, ini as i32), (fim as i32 - ini as i32) as usize * 0x35));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(cup, round, laco), || set_home_teams_for_round(cup, round, laco));
    PLACAR.registrar(igual, || format!("set_home_teams_for_round(rodada {}, laco {}) original={:x?} rust={:x?}", round as i16, laco as i16, mo, mr));
}

/// Comparer for `record_team_progress` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x521b30: lado a lado nos 3 bytes de progresso (+0x73..+0x75) do registro do clube no Comp_Stats da
/// copa (+8, achado pela 0x4a4850: push clube; ecx = as estatísticas; thiscall, ret 4), que a +0x68 grava; as versões
/// que repassam a outra competição gravam fora da região (aí a comparação só confere que nada mudou nela). Sem
/// estatísticas, sem clube ou sem registro: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_record_team_progress(cup: usize, team: u32, round: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_RECORD_TEAM_PROGRESS);
    let stats = read_usize(cup + 8);
    let clube = read_usize(team_entry(cup, team as i16 as i32));
    let search: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let reg = if stats != 0 && clube != 0 { search(stats, clube) } else { 0 };
    if reg == 0 {
        return o(cup, team, round); // simplification: no record to look at, original only
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(reg + 0x73, 3)], || o(cup, team, round), || record_team_progress(cup, team, round));
    PLACAR.registrar(igual, || format!("record_team_progress(time {}, rodada {}) original={:x?} rust={:x?}", team as i16, round as i16, mo, mr));
}

/// Comparer for `draw_round` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x51c410: só a original (os sorteios sorteiam, a notícia aloca). O plano, feito antes: sem sorteio
/// (sem +0x45 ou já sorteada), nada muda nos confrontos nem nas datas; com time em aberto, as datas previstas das
/// rodadas (a desta mais 1 dia, as seguintes no mínimo essa); sorteada, todos os confrontos com +0x34 = 1 e +0xac com
/// a soma prevista (as consultas da notícia não mudam com o sorteio).
pub(crate) unsafe extern "thiscall" fn comparar_draw_round(cup: usize, round: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_DRAW_ROUND);
    if read_usize(cup + cup::SEASON_SET_UP) == 0 || read_u8(round_ties_array(cup, round) + 0x34) != 0 {
        return o(cup, round); // simplification: nothing to draw, the original does nothing
    }
    let n = read_i16(cup + cup::ROUND_COUNT);
    if round_has_open_team(cup, round) {
        // the date plan: (record, day word, year byte) of each round from here on
        let mut a = [0u8; 8];
        let (mut dia, mut ano) = (read_i16(round_record_by_dword(cup, round)), (read_u8(round_record_by_dword(cup, round) + 2) as i8 as i16).wrapping_add(read_i16(cup + cup::YEAR)) as u16);
        // 0x54c5b0 (plus 1): normalize forward, 365 days in the leap year (crate::data::bissexto)
        dia = dia.wrapping_add(1);
        let size = |y: u16| if crate::data::bissexto(y) == 1 { 365i16 } else { 364 };
        while dia > size(ano) {
            dia = dia.wrapping_sub(size(ano) + 1);
            ano = ano.wrapping_add(1);
        }
        a[0..2].copy_from_slice(&dia.to_le_bytes());
        a[2..4].copy_from_slice(&ano.to_le_bytes());
        let mut plan = vec![(round_record_by_dword(cup, round), dia, a[2].wrapping_sub(read_u8(cup + cup::YEAR)))];
        let mut k = round.wrapping_add(1);
        while (k as i16) < n {
            let rk = round_record_by_dword(cup, k as i16 as i32 as u32);
            let mut b = [0u8; 8];
            let anob = (read_u8(rk + 2) as i8 as i16).wrapping_add(read_i16(cup + cup::YEAR)) as u16;
            b[0..2].copy_from_slice(&read_i16(rk).to_le_bytes());
            b[2..4].copy_from_slice(&anob.to_le_bytes());
            plan.push(if cup_date_is_after(&a, &b) { (rk, dia, a[2].wrapping_sub(read_u8(cup + cup::YEAR))) } else { (rk, read_i16(rk), read_u8(rk + 2)) });
            k = k.wrapping_add(1);
        }
        let r = o(cup, round);
        let ok = plan.iter().all(|&(rk, d, y)| read_i16(rk) == d && read_u8(rk + 2) == y);
        PLACAR.registrar(ok, || format!("draw_round(rodada {}) as datas adiadas diferem do plano", round as i32));
        return r;
    }
    let before_ac = read_u8(cup + cup::NEXT_DRAW_ROUND);
    let r = o(cup, round);
    let mut ok = r == 1;
    let reg = round_record_by_dword(cup, round);
    for k in 0..read_i16(reg + 0x1a).max(0) as usize {
        ok &= read_u8(round_ties_array(cup, round) + k * 0x35 + 0x34) == 1;
    }
    let sum = if !skips_draw_news(cup, round) && read_i16(reg + 7) != 0x8c { 1 } else { 0 };
    ok &= read_u8(cup + cup::NEXT_DRAW_ROUND) == before_ac.wrapping_add(sum);
    PLACAR.registrar(ok, || format!("draw_round(rodada {}) marcas dos confrontos ou +0xac ({} -> {}) diferem do plano", round as i32, before_ac, read_u8(cup + cup::NEXT_DRAW_ROUND)));
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
pub(crate) unsafe extern "thiscall" fn comparar_cup_update_staff_records_for_status(cup: usize, clube: usize, outcome: u32, a3: u32, a4: u32, jogo: usize, final_: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS);
    if clube == 0 || jogo == 0 {
        return o(cup, clube, outcome, a3, a4, jogo, final_); // simplification: without a club or match, original only
    }
    let state = club_state_record(clube);
    let before = read_i32(state);
    let previsto = if read_u8(jogo + 0x3a) != 1 && read_usize(cup + cup::STAGE_COUNT) == 0 && read_u8(cup + cup::STAGE_INDEX) == 0xff && outcome as u8 == 1
        && wins_uefa_cup_place(read_usize(cup + 4), clube) { read_i32(KEY_CLUB_COMP_UEFA_CUP) } else { before };
    let r = o(cup, clube, outcome, a3, a4, jogo, final_);
    let after = read_i32(state);
    PLACAR.registrar(r == 0 && after == previsto, || format!("cup_update_staff_records_for_status(desfecho {}) estado {:x} -> {:x}, previsto {:x}, devolveu {}", outcome as u8, before, after, previsto, r));
    r
}

/// Comparer for `cup_record_season_honours` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x519a90: só a original (a 0x4afce0 pode crescer o histórico e a 0x519fc0 tem o seu comparador). O
/// plano, feito antes: a linha livre do histórico (free_comp_history_row) recebe a competição (+4), o ano [0xae2c92] e os
/// quatro de season_honours. Sem linha livre (o histórico cresce) ou sem a competição: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_cup_record_season_honours(cup: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CUP_RECORD_SEASON_HONOURS);
    let row = free_comp_history_row();
    if row == 0 || read_usize(cup + 4) == 0 {
        return o(cup); // simplification: the history grows (realloc) or there is no competition: original only
    }
    let (campeao, vice, terceiro, anfitria) = season_honours(cup);
    let year = core::ptr::read_unaligned(G_CURRENT_DATE_YEAR as *const u16);
    o(cup);
    let lido = (read_usize(row + 4), core::ptr::read_unaligned((row + 8) as *const u16), read_usize(row + 0xa), read_usize(row + 0xe), read_usize(row + 0x12), read_usize(row + 0x16));
    let previsto = (read_usize(cup + 4), year, campeao, vice, terceiro, anfitria);
    PLACAR.registrar(lido == previsto, || format!("cup_record_season_honours: linha {:x?} prevista {:x?}", lido, previsto));
}

/// Comparer for `register_semi_final_losers_for_uefa` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x519fc0: lado a lado na tabela da semeadura (+0x440, 50 vagas de 12 bytes, onde a 0x905790 grava; ela
/// não aloca: acha a vaga da competição ou a primeira livre). Sem a semeadura: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_register_semi_final_losers_for_uefa(cup: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_REGISTER_SEMI_FINAL_LOSERS_FOR_UEFA);
    let s = read_usize(G_UEFA_SEEDING);
    if s == 0 {
        return o(cup); // simplification: without the seeding, original only
    }
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&[(s + 0x440, 50 * 12)], || o(cup), || register_semi_final_losers_for_uefa(cup));
    PLACAR.registrar(igual, || format!("register_semi_final_losers_for_uefa(copa {:x}) a semeadura difere ({:?})", cup, semi_final_losers(cup)));
}

/// Comparer for `get_fixture_side_teams` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o esperado sai do núcleo sobre uma conta própria e uma saída própria antes da original; depois, a
/// devolução, a conta global e a saída têm de bater.
pub(crate) unsafe extern "thiscall" fn comparar_get_fixture_side_teams(this: usize, round: i32, jogo: i32, side: u32, out: *mut i16) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, u32, *mut i16) -> i32>(IDX_GET_FIXTURE_SIDE_TEAMS);
    if out.is_null() || round < 0 || round > read_i16(this + 0x3c) as i32 || read_u32_as_usize(this + 0xa7) == 0 || jogo == -1 {
        return o(this, round, jogo, side, out); // simplification: the box, original only
    }
    let reg = read_u32_as_usize(read_u32_as_usize(this + 0xad) + round as usize * 4) + (jogo as usize).wrapping_mul(0x35);
    let (c, r, j) = if side as u8 != 0 { (read_i16(reg), reg + 0xe, reg + 0xa) } else { (read_i16(reg + 2), reg + 0x10, reg + 0xc) };
    let mut expected = vec![0i16; 0x100];
    let mut n: i32 = 0;
    if c != -1 {
        expected[0] = c;
        n = 1;
    } else {
        collect_fixture_teams(this, read_i16(r) as i32, read_i16(j) as i32, expected.as_mut_ptr(), &mut n, false);
    }
    let res = o(this, round, jogo, side, out);
    let obtido = core::slice::from_raw_parts(out, res.clamp(0, 0x100) as usize);
    let global = *(G_CUP_POSSIBLE_TEAM_COUNT as *const i32);
    PLACAR.registrar(res == n && global == n && obtido == &expected[..n.clamp(0, 0x100) as usize], || format!("get_fixture_side_teams({:x}, {}, {}, {}) {}/{} global {} {:?} x {:?}", this, round, jogo, side as u8, res, n, global, obtido, &expected[..n.clamp(0, 0x100) as usize]));
    res
}

/// Comparer for `collect_possible_teams` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o esperado sai do núcleo sobre uma cópia da conta e uma saída própria antes da original; depois, a
/// devolução, a conta e os clubes acrescentados têm de bater.
pub(crate) unsafe extern "thiscall" fn comparar_collect_possible_teams(this: usize, round: i32, jogo: i32, out: *mut i16, count: *mut i32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, *mut i16, *mut i32) -> i32>(IDX_COLLECT_POSSIBLE_TEAMS);
    if out.is_null() || count.is_null() || round < 0 || jogo == -1 || !(0..0x100).contains(&*count) {
        return o(this, round, jogo, out, count); // simplification: the message box or an unexpected count, original only
    }
    let c0 = *count;
    let mut expected = vec![0i16; 0x200];
    let mut n = c0;
    collect_fixture_teams(this, round, jogo, expected.as_mut_ptr(), &mut n, false);
    let res = o(this, round, jogo, out, count);
    let end = (*count).clamp(c0, 0x200);
    let obtido = core::slice::from_raw_parts(out.offset(c0 as isize), (end - c0) as usize);
    let ok = res == n && *count == n && obtido == &expected[c0 as usize..n.clamp(c0, 0x200) as usize];
    PLACAR.registrar(ok, || format!("collect_possible_teams({:x}, {}, {}) {}/{} conta {} {:?} x {:?}", this, round, jogo, res, n, *count, obtido, &expected[c0 as usize..n.clamp(c0, 0x200) as usize]));
    res
}

/// Comparer for `cup_update_for_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_CUP_UPDATE_FOR_DATE)(this, today, stage)
}

/// Comparer for `set_up_round` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_up_round(this: usize, round: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_SET_UP_ROUND)(this, round)
}

/// Comparer for `randomize_round_teams` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sorteia (0x90cfc0) e só usa um vetor temporário: lado a lado pelo estado do sorteador e pelos
/// confrontos de todas as rodadas (os vínculos vão às rodadas seguintes), a cópia em sem_contar.
pub(crate) unsafe extern "thiscall" fn comparar_randomize_round_teams(cup: usize, round: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_RANDOMIZE_ROUND_TEAMS);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend(all_ties_regions(cup));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(cup, round),
        || crate::aleatorio::sem_contar(|| randomize_round_teams(cup, round)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("randomize_round_teams(rodada {}) diferencas em {:x?}", round as i16, dif)
    });
}

/// Comparer for `separate_shared_stadium_ties` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: só grava confrontos (da rodada e das seguintes): lado a lado pelos confrontos de todas as rodadas.
pub(crate) unsafe extern "thiscall" fn comparar_separate_shared_stadium_ties(cup: usize, round: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SEPARATE_SHARED_STADIUM_TIES);
    let reg = all_ties_regions(cup);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(cup, round), || separate_shared_stadium_ties(cup, round));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("separate_shared_stadium_ties(rodada {}) diferencas em {:x?}", round as i16, dif)
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
pub(crate) unsafe extern "thiscall" fn comparar_decide_tie_winner(this: usize, tie: usize, jogo: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_DECIDE_TIE_WINNER);
    let reg = [(jogo + 0x3d, 2)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || Some(o(this, tie, jogo)), || tie_winner_side(tie, jogo));
    let r = r.unwrap_or(0);
    if let Some(n) = n {
        PLACAR.registrar(igual && r == n, || format!("decide_tie_winner(regra {:x}, jogo {}) original={} rust={} {:x?} x {:x?}",
            read_i16(jogo + 0x38) as u16, read_i8(tie + 0x33), r, n, mo, mr));
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
pub(crate) unsafe extern "C" fn comparar_cup_set_round_draw_date(rounds: usize, round: u32, day: u32, month: u32, years: u32, week: u32, base: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize>(IDX_CUP_SET_ROUND_DRAW_DATE);
    let r = round_record_from_base(rounds, round);
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r, 3)], || o(rounds, round, day, month, years, week, base),
        || cup_set_round_draw_date(rounds, round, day, month, years, week, base));
    PLACAR.registrar(igual && a == b, || format!("cup_set_round_draw_date({}, {}/{} +{}) {:x?} x {:x?}", round as i16, day, month, years, mo, mr));
    a
}

/// Comparer for `cup_set_round_match_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_cup_set_round_match_date(rounds: usize, round: u32, day: u32, month: u32, years: u32, week: u32, hora: u32, base: u32, mando: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize>(IDX_CUP_SET_ROUND_MATCH_DATE);
    let r = round_record_from_base(rounds, round);
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r + 3, 0x14)], || o(rounds, round, day, month, years, week, hora, base, mando),
        || cup_set_round_match_date(rounds, round, day, month, years, week, hora, base, mando));
    PLACAR.registrar(igual && a == b, || format!("cup_set_round_match_date({}, {}/{} +{}) {:x?} x {:x?}", round as i16, day, month, years, mo, mr));
    a
}

/// Comparer for `apply_round_rules_to_match` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_apply_round_rules_to_match(cup: usize, jogo: usize, key: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32)>(IDX_APPLY_ROUND_RULES_TO_MATCH);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(jogo + 0x38, 2)], || o(cup, jogo, key),
        || apply_round_rules_to_match(cup, jogo, key));
    PLACAR.registrar(igual, || format!("apply_round_rules_to_match(chave {}) {:x?} x {:x?}", key, mo, mr));
}

/// Comparer for `find_tree_slot_for_fixture` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_find_tree_slot_for_fixture(cup: usize, jogo: usize) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i32>(IDX_FIND_TREE_SLOT_FOR_FIXTURE)(cup, jogo);
    if jogo == 0 {
        return r;
    }
    let n = find_tree_slot_for_fixture(cup, jogo);
    PLACAR.registrar(r == n, || format!("find_tree_slot_for_fixture original={} rust={}", r, n));
    r
}

/// Comparer for `cup_match_importance` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_cup_match_importance(cup: usize, partida: usize, second: u8, margem: *mut u8) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u8, *mut u8) -> u8>(IDX_CUP_MATCH_IMPORTANCE);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(margem as usize, 1)], || o(cup, partida, second, margem), || cup_match_importance(cup, partida, second, margem));
    PLACAR.registrar(igual && r == n, || format!("cup_match_importance(lado {}) original={} rust={} {:x?} x {:x?}", second, r, n, mo, mr));
    r
}

/// Comparer for `add_team_result_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_team_result_news(cup: usize, clube: usize, outcome: u32, a3: u32, p: usize, a5: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32>(IDX_ADD_TEAM_RESULT_NEWS)(cup, clube, outcome, a3, p, a5)
}

/// Comparer for `cup_check_complete` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (o método das fases só lê).
pub(crate) unsafe extern "thiscall" fn comparar_cup_check_complete(this: usize, stages: u32, x: u32) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(IDX_CUP_CHECK_COMPLETE)(this, stages, x);
    let n = cup_check_complete(this, stages, x);
    PLACAR.registrar(r == n, || format!("cup_check_complete({:x}, {}) original={} rust={}", this, stages, r, n));
    r
}

/// Comparer for `move_round_fixtures_to_set_dates` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x51ebe0: só a original (os jogos saem do calendário e voltam pela 0x5aa680, que aloca, e as notícias
/// vão aos técnicos). O plano, feito antes: os jogos da copa na data e fatia (0x5abd90 numa lista provisória), na
/// ordem da lista; cada um do tipo da liga ocupa a vaga seguinte (de 7 em 7 bytes, até 0x38 e até a primeira 0xff), e
/// a vaga do que está na data da rodada fica 0xff; as outras vagas não mudam. Sem a copa em [0xadadfc] depois: nada a
/// conferir.
pub(crate) unsafe extern "thiscall" fn comparar_move_round_fixtures_to_set_dates(this: usize, data: usize, round: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_MOVE_ROUND_FIXTURES_TO_SET_DATES);
    let e = cup_round_entry(this, round);
    if read_i16(e + 0xd) as u16 & 0x100 == 0 || read_u8(e + 0x29) == 0xff {
        return o(this, data, round);
    }
    let id = read_u32_as_usize(read_u32_as_usize(this + 4));
    let create: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut list = [0u8; 0x10];
    let l = list.as_mut_ptr() as usize;
    create(l);
    let mut expected = [0u8; 8];
    for k in 0..8 {
        expected[k] = read_u8(e + k * 7 + 0x29);
    }
    if collect_round_fixtures(this, data, round, l) != 0 {
        write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
        let mut j = 0usize;
        let mut f = next(l);
        while f != 0 {
            if read_u8(this + 0x44) == read_u8(f + 0x42) && j < 0x38 && expected[j / 7] != 0xff {
                if fixture_on_round_date(this, e, f) {
                    expected[j / 7] = 0xff;
                }
                j += 7;
            }
            f = next(l);
        }
    }
    destroy(l);
    o(this, data, round);
    if read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + id * 4) == 0 {
        return; // simplification: the cup has left the game; nothing to check
    }
    let e = cup_round_entry(this, round);
    let after: [u8; 8] = core::array::from_fn(|k| read_u8(e + k * 7 + 0x29));
    PLACAR.registrar(after == expected, || format!("move_round_fixtures_to_set_dates(rodada {}) vagas {:02x?} (esperado {:02x?})",
        round, after, expected));
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
    let mut error: Option<(i32, i32)> = None;
    'fora: for rd in 0..(m - 1).max(0) {
        for p in 0..read_i16(comp_cup_round(this, rd) + 0x1a) as i32 {
            let c = cup_tie(this, rd, p);
            let target = read_i16(c + 0x12) as i32;
            let a = read_i16(c + 8) == 1;
            let ok = (target == rd + 1 || target == rd + 2) && {
                let v = cup_tie(this, target, read_i16(c + 6) as i32);
                read_i16(v + if a { 0xa } else { 0xc }) as i32 == p && read_i16(v + if a { 0xe } else { 0x10 }) as i32 == rd
            };
            if !ok {
                error = Some((rd, p));
                break 'fora;
            }
        }
    }
    PLACAR.registrar(error.is_none(), || format!("map_fixture_tree: o confronto (rodada, índice) {:?} sem a vaga de volta", error));
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
        return o(this); // simplification: the box or nothing to sort, original only
    }
    let mut v: Vec<u32> = (0..n as usize).map(|i| read_u32_as_usize(read_u32_as_usize(this + 0xa7) + i * 6) as u32).collect();
    let mut k = vec![0i32; n as usize];
    sort_clubs_by_strength(v.as_mut_ptr(), k.as_mut_ptr(), n);
    // simplification: with more slots in the rounds than clubs, the original reads past the vector (block leftover); the plan reads 0
    v.extend(core::iter::repeat(0).take(0x100));
    let plan = set_final_positions_by_round(this, v.as_ptr(), 0);
    o(this);
    let search: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let mut erros = Vec::new();
    for (i, &(c, p)) in plan.iter().enumerate() {
        if c == 0 || plan[i + 1..].iter().any(|x| x.0 == c) {
            continue;
        }
        let e = search(read_u32_as_usize(this + 8), c);
        if e == 0 || *((e + 0x72) as *const u8) != p {
            erros.push(format!("clube {:x} esperado {} obtido {}", c, p, if e == 0 { -1 } else { *((e + 0x72) as *const u8) as i32 }));
        }
    }
    PLACAR.registrar(erros.is_empty(), || format!("cup_rank_teams({:x}) {} posições: {}", this, plan.len(), erros.join("; ")));
}

/// Comparer for `cup_set_round_match_day` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_cup_set_round_match_day(rounds: usize, round: i16, slot: i16, week: i16, hora: i8, code: i8, mando: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, i16, i16, i16, i8, i8, u32) -> u32>(IDX_CUP_SET_ROUND_MATCH_DAY);
    if !(0..8).contains(&slot) || !(-1..=6).contains(&week) || !(-1..3).contains(&hora) || !(-1..=3).contains(&code) {
        return o(rounds, round, slot, week, hora, code, mando);
    }
    let e = rounds.wrapping_add((round as isize * 104 + slot as isize * 7) as usize);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(e + 0x23, 7)], || o(rounds, round, slot, week, hora, code, mando),
        || cup_set_round_match_day(rounds, round, slot, week, hora, code, mando));
    PLACAR.registrar(igual && r == n, || format!("cup_set_round_match_day({}, {}) {:x?} x {:x?}", round, slot, mo, mr));
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
