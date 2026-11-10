use super::*;

// The comparers of the routines in ../comp/cup.rs.
// [pt] Os comparadores das rotinas de ../comp/cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `create_tie_fixtures` (comparison mode: see the header of motor.rs): the comparer of `0x51cb90`; only
/// the original routine runs (the match goes to the calendar via `0x5aa680`, which allocates, and the news goes to the
/// managers).
///
/// The plan, made beforehand: the chosen tie (−1 searches for mark 1 or 2), the return value, the mark (2 when
/// postponing, 3 when creating) and, when creating with both teams, the home side's matches in the competition go up by
/// 1 (2 in a two-legged one with the cup still active).
///
/// [pt] Comparador da 0x51cb90: só a original (a partida vai ao calendário pela 0x5aa680, que aloca, e as notícias aos
/// técnicos). O plano, feito antes: o confronto escolhido (o −1 procura a marca 1 ou 2), a devolução, a marca (2 ao
/// adiar, 3 ao criar) e, criando com os dois times, os jogos do mandante na competição sobem 1 (2 na de duas mãos com a
/// copa ainda ativa).
pub(crate) unsafe extern "thiscall" fn compare_create_tie_fixtures(this: usize, round: u32, tie: u32, adiar: u32) -> u32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(IDX_CREATE_TIE_FIXTURES);
    let ri = round as i16 as i32;
    let id = read_usize(read_usize(this.addr() + 4)) as u32;
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
            let r = o(this.addr(), round, tie, adiar);
            SCOREBOARD.record(r == 0, || format!("create_tie_fixtures(round {}) without tie marked: {}", ri, r));
            return r;
        }
    }
    let t = tie_of_round(this, ri, k);
    let full = read_i16(t) != -1 && read_i16(t + cup_tie::AWAY_TEAM) != -1;
    let e = round_entry(this, ri);
    let duas = read_u8(e + cup_round::LEGS) == 2;
    let home = if full { read_usize(cup_team_entry(this, read_i16(t))) } else { 0 };
    let ic = if home != 0 { read_usize(home) as u32 } else { u32::MAX };
    let check = ad == 0 && full && ic != u32::MAX;
    // the end of the window: the round’s date (+ the days until the second leg), tomorrow or the next round’s date (+0x6b/+0x6d), the
    // that comes next, + 14 days. On a clash, 0x5aa680 reschedules the match (0x5ab510): cup (type 2) up to the date of the
    // next round (without it, 2 weeks), the other types up to 2 weeks later; with more leagues it happens (the 1st round
    // of cup 101, 2001-225, went more than 7 days)
    let first_leg = (read_u8(e + cup_round::YEAR_OFFSET) as i8 as i32 + this.year() as i32, read_i16(e + cup_round::DAY) as i32);
    let last = if duas { add_days_to_day_year(first_leg, read_u8(e + cup_round::LEG_GAP_DAYS) as i32) } else { first_leg };
    let tomorrow = add_days_to_day_year((read_i16(G_CURRENT_DATE_YEAR) as u16 as i32, read_i16(G_CURRENT_DATE) as i32), 1);
    let next = if ri < this.round_count() as i32 - 1 {
        (read_u8(e + 0x6d) as i8 as i32 + this.year() as i32, read_i16(e + 0x6b) as i32)
    } else {
        last
    };
    let end = add_days_to_day_year(last.max(tomorrow).max(next), 14);
    let before = if check { club_fixtures_until(ic, id, end) } else { 0 };
    let r = o(this.addr(), round, tie, adiar);
    let mark = read_u8(t + cup_tie::FIXTURE_STATE);
    let mut ok = r == 1 && mark == if ad != 0 { 2 } else { 3 };
    let mut expected = 0;
    let mut after = 0;
    if check && read_usize(read_usize(G_COMP_OBJECT_TABLE) + id as usize * 4) != 0 {
        expected = 1 + duas as u32;
        after = club_fixtures_until(ic, id, end);
        ok = ok && after == before + expected;
    }
    SCOREBOARD.record(ok, || format!("create_tie_fixtures(round {}, tie {}, adiar {}) returned {} mark {} games {} -> {} (expected +{})",
        ri, k, ad, r, mark, before, after, expected));
    r
}

/// Comparer for `post_draw_news`.
///
/// [pt] Comparador da 0x522950: só a original (a notícia vai aos técnicos e não tem outro efeito).
pub(crate) unsafe extern "thiscall" fn compare_post_draw_news(this: usize, round: u32, tie: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_POST_DRAW_NEWS)(this, round, tie) // simplification: only the news item
}

/// Comparer for `sort_round_teams`.
///
/// [pt] Comparador: sorteia (0x90cfc0) e só usa vetores temporários: lado a lado pelo estado do sorteador e pelos
/// confrontos de todas as rodadas (os vínculos vão às rodadas seguintes), a cópia em sem_contar.
pub(crate) unsafe extern "thiscall" fn compare_sort_round_teams(cup: usize, round: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SORT_ROUND_TEAMS);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend(all_ties_regions(Cup(cup)));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(cup, round),
        || crate::rng::without_counting(|| sort_round_teams(cup, round)));
    SCOREBOARD.record(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("sort_round_teams(round {}) differences em {:x?}", round as i16, dif)
    });
}

/// Comparer for `check_round_teams_for_nation`.
///
/// [pt] Comparador: como o da 0x51d2d0 (sorteia; as contas e o vetor são temporários).
pub(crate) unsafe extern "thiscall" fn compare_check_round_teams_for_nation(cup: usize, round: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_CHECK_ROUND_TEAMS_FOR_NATION);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend(all_ties_regions(Cup(cup)));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(cup, round),
        || crate::rng::without_counting(|| check_round_teams_for_nation(cup, round)));
    SCOREBOARD.record(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("check_round_teams_for_nation(round {}) differences em {:x?}", round as i16, dif)
    });
}

/// Comparador: lado a lado no byte +0xb1.
pub(crate) unsafe extern "thiscall" fn compare_cup_start_new_season(cup: usize) {
    let cup = Cup(cup);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CUP_START_NEW_SEASON);
    let reg = [(cup.addr() + cup::RUN_BY_STAGES, 1)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(cup.addr()), || cup_start_new_season(cup.addr()));
    SCOREBOARD.record(igual, || format!("cup_start_new_season({:#x}) {:x?} x {:x?}", cup.addr(), mo, mr));
}

/// Comparer for `pay_round_prize_money`.
///
/// Comparer of `pay_round_prize_money`, as a dry run: the copy without the news item and the whole original routine, from the same
/// state (the fund and the finance records of both clubs, restored after the dry run); checks the fund and the
/// finances.
///
/// No club, no finances, or with the division that drops: only the original.
///
/// [pt] Comparador da 0x522240, em ensaio: a cópia sem a notícia e a original inteira, a partir do mesmo estado (o fundo e
/// os registros de finanças dos dois clubes, restaurados depois do ensaio); confere o fundo e as finanças. Sem clube,
/// sem finanças ou com a divisão que cai: só a original.
pub(crate) unsafe extern "thiscall" fn compare_pay_round_prize_money(cup: usize, winner: usize, loser: usize, round: u32, fixture: usize) {
    let cup = Cup(cup);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize)>(IDX_PAY_ROUND_PRIZE_MONEY);
    let (cv, cp) = (read_usize(winner), read_usize(loser));
    if cv == 0 || cp == 0 || cv == cp || round_prizes(cup, round as i16 as i32, false).is_none() {
        return o(cup.addr(), winner, loser, round, fixture); // simplification: with no club or with the falling division, original only
    }
    let (fv, fp) = (european_cup_club_finance(cv), european_cup_club_finance(cp));
    if fv == 0 || fp == 0 {
        return o(cup.addr(), winner, loser, round, fixture); // simplification: without finances, original only
    }
    let regions = [(cup.addr() + cup::PRIZE_FUND, 4), (fv, 0x167), (fp, 0x167)];
    let before = crate::comparison::MemorySnapshot::read(&regions);
    ROUND_PRIZE_MONEY_DRY_RUN.store(true, Relaxed);
    crate::rng::pause_trace(|| crate::rng::without_counting(|| pay_round_prize_money(cup.addr(), winner, loser, round, fixture)));
    ROUND_PRIZE_MONEY_DRY_RUN.store(false, Relaxed);
    let mr = crate::comparison::MemorySnapshot::read(&regions);
    before.write_back();
    o(cup.addr(), winner, loser, round, fixture);
    let mo = crate::comparison::MemorySnapshot::read(&regions);
    SCOREBOARD.record(mo == mr, || format!("pay_round_prize_money(round {}, winner {:x}, loser {:x}) o bottom and the finances diferem", round as i16, cv, cp));
}

/// Comparer for `set_home_teams_for_round`.
///
/// [pt] Comparador da 0x51e870: lado a lado nos confrontos percorridos (a virtual +0x90 só consulta e roda nos dois lados).
pub(crate) unsafe extern "thiscall" fn compare_set_home_teams_for_round(cup: usize, round: u32, laco: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_SET_HOME_TEAMS_FOR_ROUND);
    let (ini, fim) = ties_range(cup, round, laco);
    let mut reg = Vec::new();
    if ini < fim {
        reg.push((round_tie(Cup(cup), round as i16 as i32, ini as i32), (fim as i32 - ini as i32) as usize * 0x35));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(cup, round, laco), || set_home_teams_for_round(cup, round, laco));
    SCOREBOARD.record(igual, || format!("set_home_teams_for_round(round {}, loop {}) original={:x?} rust={:x?}", round as i16, laco as i16, mo, mr));
}

/// Comparer for `record_team_progress`.
///
/// Comparer of `record_team_progress`: side by side on the 3 progress bytes (+0x73..+0x75) of the club's record in the cup's
/// Comp_Stats (+8, found by `exe::comp_stats_find_club_entry`: push club; ecx = the statistics; thiscall, ret 4), which +0x68 writes. The
/// versions that pass it on to another competition write outside that region (then the comparison only checks that
/// nothing changed in it).
///
/// Without statistics, without a club or without a record: only the original.
///
/// [pt] Comparador da 0x521b30: lado a lado nos 3 bytes de progresso (+0x73..+0x75) do registro do clube no Comp_Stats da
/// copa (+8, achado pela 0x4a4850: push clube; ecx = as estatísticas; thiscall, ret 4), que a +0x68 grava; as versões
/// que repassam a outra competição gravam fora da região (aí a comparação só confere que nada mudou nela). Sem
/// estatísticas, sem clube ou sem registro: só a original.
pub(crate) unsafe extern "thiscall" fn compare_record_team_progress(cup: usize, team: u32, round: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_RECORD_TEAM_PROGRESS);
    let stats = read_usize(cup + 8);
    let club = read_usize(team_entry(Cup(cup), team as i16 as i32));
    let reg = if stats != 0 && club != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if reg == 0 {
        return o(cup, team, round); // simplification: no record to look at, original only
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(reg + comp_stats_club::CURRENT_POSITION, 3)], || o(cup, team, round), || record_team_progress(cup, team, round));
    SCOREBOARD.record(igual, || format!("record_team_progress(team {}, round {}) original={:x?} rust={:x?}", team as i16, round as i16, mo, mr));
}

/// Comparer for `draw_round` (`draw_round`; comparison mode: see the header of motor.rs).
///
/// Only the original runs (the random draws draw, the news item allocates). The plan, made beforehand: with no draw (no
/// +0x45 or already drawn), nothing changes in the ties or the dates; with a team still open, the predicted dates of
/// the rounds (this one plus 1 day, the following ones at least that); once drawn, all ties with +0x34 = 1 and +0xac
/// with the predicted sum (the news item's queries do not change with the draw).
///
/// [pt] Comparador da 0x51c410: só a original (os sorteios sorteiam, a notícia aloca). O plano, feito antes: sem sorteio
/// (sem +0x45 ou já sorteada), nada muda nos confrontos nem nas datas; com time em aberto, as datas previstas das
/// rodadas (a desta mais 1 dia, as seguintes no mínimo essa); sorteada, todos os confrontos com +0x34 = 1 e +0xac com
/// a soma prevista (as consultas da notícia não mudam com o sorteio).
pub(crate) unsafe extern "thiscall" fn compare_draw_round(cup: usize, round: u32) -> u32 {
    let cup = Cup(cup);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_DRAW_ROUND);
    if cup.season_set_up() == 0 || read_u8(round_ties_array(cup, round) + 0x34) != 0 {
        return o(cup.addr(), round); // simplification: nothing to draw, the original does nothing
    }
    let n = cup.round_count();
    if round_has_open_team(cup, round) {
        // the date plan: (record, day word, year byte) of each round from here on
        let mut a = [0u8; 8];
        let (mut dia, mut ano) = (read_i16(round_record_by_dword(cup, round)), (read_u8(round_record_by_dword(cup, round) + 2) as i8 as i16).wrapping_add(cup.year()) as u16);
        // 0x54c5b0 (plus 1): normalize forward, 365 days in the leap year (crate::date::bissexto)
        dia = dia.wrapping_add(1);
        let size = |y: u16| if crate::date::is_leap_year(y) == 1 { 365i16 } else { 364 };
        while dia > size(ano) {
            dia = dia.wrapping_sub(size(ano) + 1);
            ano = ano.wrapping_add(1);
        }
        a[0..2].copy_from_slice(&dia.to_le_bytes());
        a[2..4].copy_from_slice(&ano.to_le_bytes());
        let mut plan = vec![(round_record_by_dword(cup, round), dia, a[2].wrapping_sub((cup.year() as u8)))];
        let mut k = round.wrapping_add(1);
        while (k as i16) < n {
            let rk = round_record_by_dword(cup, k as i16 as i32 as u32);
            let mut b = [0u8; 8];
            let anob = (read_u8(rk + 2) as i8 as i16).wrapping_add(cup.year()) as u16;
            b[0..2].copy_from_slice(&read_i16(rk).to_le_bytes());
            b[2..4].copy_from_slice(&anob.to_le_bytes());
            plan.push(if cup_date_is_after(&a, &b) { (rk, dia, a[2].wrapping_sub((cup.year() as u8))) } else { (rk, read_i16(rk), read_u8(rk + 2)) });
            k = k.wrapping_add(1);
        }
        let r = o(cup.addr(), round);
        let ok = plan.iter().all(|&(rk, d, y)| read_i16(rk) == d && read_u8(rk + 2) == y);
        SCOREBOARD.record(ok, || format!("draw_round(round {}) as dates adiadas diferem of the plan", round as i32));
        return r;
    }
    let before_ac = cup.next_draw_round();
    let r = o(cup.addr(), round);
    let mut ok = r == 1;
    let reg = round_record_by_dword(cup, round);
    for k in 0..read_i16(reg + cup_round::TIES).max(0) as usize {
        ok &= read_u8(round_ties_array(cup, round) + k * 0x35 + 0x34) == 1;
    }
    let sum = if !skips_draw_news(cup, round) && read_i16(reg + cup_round::ROUND_CODE) != 0x8c { 1 } else { 0 };
    ok &= cup.next_draw_round() == before_ac.wrapping_add(sum);
    SCOREBOARD.record(ok, || format!("draw_round(round {}) marks of the confrontos ou +0xac ({} -> {}) diferem of the plan", round as i32, before_ac, cup.next_draw_round()));
    r
}

/// Comparer for `club_in_higher_division`.
///
/// [pt] Comparador da 0x522360: pelo valor (só lê).
pub(crate) unsafe extern "thiscall" fn compare_club_in_higher_division(this: usize, a: usize, b: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(IDX_CLUB_IN_HIGHER_DIVISION)(this, a, b);
    let n = club_in_higher_division(this, a, b);
    SCOREBOARD.record(r == n, || format!("club_in_higher_division({:x}, {:x}) original={:x} rust={:x}", a, b, r, n));
    r
}

/// Comparer for `cup_update_staff_records_for_status` (comparison mode: see the header of motor.rs): the comparer of
/// `0x519890`; only the original routine runs (the records go to the people's history and change reputations and
/// morale, so they cannot run twice).
///
/// The plan, made beforehand: the club's status (`+0` of the 6-byte record) changes to UEFA Cup when the winner earns
/// the place; otherwise it does not change. Without a club: only the original routine.
///
/// [pt] Comparador da 0x519890: só a original (os registros vão ao histórico das pessoas e mexem nas reputações e no moral,
/// não podem rodar duas vezes). O plano, feito antes: o estado do clube (+0 do registro de 6 bytes) passa à Copa da
/// UEFA quando o campeão ganha a vaga; senão não muda. Sem clube: só a original.
pub(crate) unsafe extern "thiscall" fn compare_cup_update_staff_records_for_status(cup: usize, club: usize, outcome: u32, a3: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let cup = Cup(cup);
    let game = Fixture(game);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS);
    if club == 0 || game.is_null() {
        return o(cup.addr(), club, outcome, a3, a4, game.addr(), final_); // simplification: without a club or match, original only
    }
    let state = club_state_record(club);
    let before = read_i32(state);
    let predicted = if game.category() != 1 && cup.stage_count() == 0 && cup.stage_index() == 0xff && outcome as u8 == 1
        && wins_uefa_cup_place(Competition(read_usize(cup.addr() + 4)), club) { read_i32(KEY_CLUB_COMP_UEFA_CUP) } else { before };
    let r = o(cup.addr(), club, outcome, a3, a4, game.addr(), final_);
    let after = read_i32(state);
    SCOREBOARD.record(r == 0 && after == predicted, || format!("cup_update_staff_records_for_status(outcome {}) state {:x} -> {:x}, predicted {:x}, returned {}", outcome as u8, before, after, predicted, r));
    r
}

/// Comparer for `cup_record_season_honours`.
///
/// Comparer of `cup_record_season_honours`: only the original routine runs (`comp_util_add_history_record` may grow the history and `register_semi_final_losers_for_uefa` has its own
/// comparer). The plan, made beforehand: the free history row (free_comp_history_row) gets the competition (+4), the
/// year [0xae2c92] and the four of season_honours.
///
/// No free row (the history grows) or no competition: only the original.
///
/// [pt] Comparador da 0x519a90: só a original (a 0x4afce0 pode crescer o histórico e a 0x519fc0 tem o seu comparador). O
/// plano, feito antes: a linha livre do histórico (free_comp_history_row) recebe a competição (+4), o ano [0xae2c92] e os
/// quatro de season_honours. Sem linha livre (o histórico cresce) ou sem a competição: só a original.
pub(crate) unsafe extern "thiscall" fn compare_cup_record_season_honours(cup: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CUP_RECORD_SEASON_HONOURS);
    let row = free_comp_history_row();
    if row == 0 || read_usize(cup + 4) == 0 {
        return o(cup); // simplification: the history grows (realloc) or there is no competition: original only
    }
    let (campeao, vice, terceiro, anfitria) = season_honours(Cup(cup));
    let year = read_u16(G_CURRENT_DATE_YEAR);
    o(cup);
    let lido = (read_usize(row + 4), read_u16(row + 8), read_usize(row + comp_history_row::CHAMPION), read_usize(row + comp_history_row::RUNNER_UP), read_usize(row + comp_history_row::THIRD_PLACE), read_usize(row + comp_history_row::HOST));
    let predicted = (read_usize(cup + 4), year, campeao, vice, terceiro, anfitria);
    SCOREBOARD.record(lido == predicted, || format!("cup_record_season_honours: row {:x?} prevista {:x?}", lido, predicted));
}

/// Comparer for `register_semi_final_losers_for_uefa`.
///
/// [pt] Comparador da 0x519fc0: lado a lado na tabela da semeadura (+0x440, 50 vagas de 12 bytes, onde a 0x905790 grava; ela
/// não aloca: acha a vaga da competição ou a primeira livre). Sem a semeadura: só a original.
pub(crate) unsafe extern "thiscall" fn compare_register_semi_final_losers_for_uefa(cup: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_REGISTER_SEMI_FINAL_LOSERS_FOR_UEFA);
    let s = read_usize(G_UEFA_SEEDING);
    if s == 0 {
        return o(cup); // simplification: without the seeding, original only
    }
    let (_, _, igual, _, _) = crate::comparison::side_by_side(&[(s + uefa_seeding::CUP_FINALS, 50 * 12)], || o(cup), || register_semi_final_losers_for_uefa(cup));
    SCOREBOARD.record(igual, || format!("register_semi_final_losers_for_uefa(cup {:x}) a semeadura difere ({:?})", cup, semi_final_losers(Cup(cup))));
}

/// Comparer for `get_fixture_side_teams`.
///
/// [pt] Comparador: o esperado sai do núcleo sobre uma conta própria e uma saída própria antes da original; depois, a
/// devolução, a conta global e a saída têm de bater.
pub(crate) unsafe extern "thiscall" fn compare_get_fixture_side_teams(this: usize, round: i32, game: i32, side: u32, out: *mut i16) -> i32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, u32, *mut i16) -> i32>(IDX_GET_FIXTURE_SIDE_TEAMS);
    if out.is_null() || round < 0 || round > this.round_count() as i32 || this.team_table() == 0 || game == -1 {
        return o(this.addr(), round, game, side, out); // simplification: the box, original only
    }
    let reg = read_usize(this.tie_tables() + round as usize * 4) + (game as usize).wrapping_mul(0x35);
    let (c, r, j) = if side as u8 != 0 { (read_i16(reg), reg + cup_tie::HOME_LINK_ROUND, reg + cup_tie::HOME_LINK_TIE) } else { (read_i16(reg + cup_tie::AWAY_TEAM), reg + cup_tie::AWAY_LINK_ROUND, reg + cup_tie::AWAY_LINK_TIE) };
    let mut expected = vec![0i16; 0x100];
    let mut n: i32 = 0;
    if c != -1 {
        expected[0] = c;
        n = 1;
    } else {
        collect_fixture_teams(this, read_i16(r) as i32, read_i16(j) as i32, expected.as_mut_ptr(), &mut n, false);
    }
    let res = o(this.addr(), round, game, side, out);
    let got = core::slice::from_raw_parts(out, res.clamp(0, 0x100) as usize);
    let global = read_i32(G_CUP_POSSIBLE_TEAM_COUNT);
    SCOREBOARD.record(res == n && global == n && got == &expected[..n.clamp(0, 0x100) as usize], || format!("get_fixture_side_teams({:x}, {}, {}, {}) {}/{} global {} {:?} x {:?}", this.addr(), round, game, side as u8, res, n, global, got, &expected[..n.clamp(0, 0x100) as usize]));
    res
}

/// Comparer for `collect_possible_teams`.
///
/// [pt] Comparador: o esperado sai do núcleo sobre uma cópia da conta e uma saída própria antes da original; depois, a
/// devolução, a conta e os clubes acrescentados têm de bater.
pub(crate) unsafe extern "thiscall" fn compare_collect_possible_teams(this: usize, round: i32, game: i32, out: *mut i16, count: *mut i32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, *mut i16, *mut i32) -> i32>(IDX_COLLECT_POSSIBLE_TEAMS);
    if out.is_null() || count.is_null() || round < 0 || game == -1 || !(0..0x100).contains(&*count) {
        return o(this, round, game, out, count); // simplification: the message box or an unexpected count, original only
    }
    let c0 = *count;
    let mut expected = vec![0i16; 0x200];
    let mut n = c0;
    collect_fixture_teams(Cup(this), round, game, expected.as_mut_ptr(), &mut n, false);
    let res = o(this, round, game, out, count);
    let end = (*count).clamp(c0, 0x200);
    let got = core::slice::from_raw_parts(out.offset(c0 as isize), (end - c0) as usize);
    let ok = res == n && *count == n && got == &expected[c0 as usize..n.clamp(c0, 0x200) as usize];
    SCOREBOARD.record(ok, || format!("collect_possible_teams({:x}, {}, {}) {}/{} count {} {:?} x {:?}", this, round, game, res, n, *count, got, &expected[c0 as usize..n.clamp(c0, 0x200) as usize]));
    res
}

/// Comparer for `cup_update_for_date`.
pub(crate) unsafe extern "thiscall" fn compare_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_CUP_UPDATE_FOR_DATE)(this, today, stage)
}

/// Comparer for `set_up_round`.
pub(crate) unsafe extern "thiscall" fn compare_set_up_round(this: usize, round: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_SET_UP_ROUND)(this, round)
}

/// Comparer for `randomize_round_teams`.
///
/// [pt] Comparador: sorteia (0x90cfc0) e só usa um vetor temporário: lado a lado pelo estado do sorteador e pelos
/// confrontos de todas as rodadas (os vínculos vão às rodadas seguintes), a cópia em sem_contar.
pub(crate) unsafe extern "thiscall" fn compare_randomize_round_teams(cup: usize, round: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_RANDOMIZE_ROUND_TEAMS);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend(all_ties_regions(Cup(cup)));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(cup, round),
        || crate::rng::without_counting(|| randomize_round_teams(cup, round)));
    SCOREBOARD.record(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("randomize_round_teams(round {}) differences em {:x?}", round as i16, dif)
    });
}

/// Comparer for `separate_shared_stadium_ties`.
///
/// [pt] Comparador: só grava confrontos (da rodada e das seguintes): lado a lado pelos confrontos de todas as rodadas.
pub(crate) unsafe extern "thiscall" fn compare_separate_shared_stadium_ties(cup: usize, round: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SEPARATE_SHARED_STADIUM_TIES);
    let reg = all_ties_regions(Cup(cup));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(cup, round), || separate_shared_stadium_ties(cup, round));
    SCOREBOARD.record(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("separate_shared_stadium_ties(round {}) differences em {:x?}", round as i16, dif)
    });
}

/// Comparer for `cup_compare_entries_by_level_desc`.
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn compare_cup_compare_entries_by_level_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_CUP_COMPARE_ENTRIES_BY_LEVEL_DESC)(a, b);
    let n = cup_compare_entries_by_level_desc(a, b);
    SCOREBOARD.record(r == n, || format!("cup_compare_entries_by_level_desc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `cup_compare_entries_by_level_asc`.
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn compare_cup_compare_entries_by_level_asc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_CUP_COMPARE_ENTRIES_BY_LEVEL_ASC)(a, b);
    let n = cup_compare_entries_by_level_asc(a, b);
    SCOREBOARD.record(r == n, || format!("cup_compare_entries_by_level_asc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `decide_fixture_winner`.
///
/// [pt] Comparador: lado a lado pelos códigos (+0x3d/+0x3e) e pelo lado devolvido; jogo nulo ou empate nos pênaltis (a
/// caixa): só a original.
pub(crate) unsafe extern "thiscall" fn compare_decide_fixture_winner(this: usize, game: usize) -> u32 {
    let game = Fixture(game);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_DECIDE_FIXTURE_WINNER);
    if game.is_null() {
        return o(this, game.addr());
    }
    let reg = [(game.addr() + fixture::HOME_RESULT_MARK, 2)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || Some(o(this, game.addr())), || fixture_winner_side(Cup(this), game));
    let r = r.unwrap_or(0);
    if let Some(n) = n {
        SCOREBOARD.record(igual && r == n, || format!("decide_fixture_winner(rule {:x}) original={} rust={} {:x?} x {:x?}", (game.tie_rules() as i16) as u16, r, n, mo, mr));
    }
    r
}

/// Comparer for `decide_tie_winner`.
///
/// [pt] Comparador: lado a lado pelos códigos (+0x3d/+0x3e) e pelo lado devolvido; empate nos pênaltis (a caixa): só a
/// original.
pub(crate) unsafe extern "thiscall" fn compare_decide_tie_winner(this: usize, tie: usize, game: usize) -> u32 {
    let game = Fixture(game);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_DECIDE_TIE_WINNER);
    let reg = [(game.addr() + fixture::HOME_RESULT_MARK, 2)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || Some(o(this, tie, game.addr())), || tie_winner_side(tie, game));
    let r = r.unwrap_or(0);
    if let Some(n) = n {
        SCOREBOARD.record(igual && r == n, || format!("decide_tie_winner(rule {:x}, game {}) original={} rust={} {:x?} x {:x?}",
            (game.tie_rules() as i16) as u16, read_i8(tie + 0x33), r, n, mo, mr));
    }
    r
}

/// Comparer for `cup_compare_draw_entries_by_seed`.
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "C" fn compare_cup_compare_draw_entries_by_seed(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_CUP_COMPARE_DRAW_ENTRIES_BY_SEED)(a, b);
    let n = cup_compare_draw_entries_by_seed(a, b);
    SCOREBOARD.record(r == n, || format!("cup_compare_draw_entries_by_seed({}, {}) original={} rust={}", read_i8(a + 6), read_i8(b + 6), r, n));
    r
}

/// Comparer for `cup_set_round_draw_date`.
pub(crate) unsafe extern "C" fn compare_cup_set_round_draw_date(rounds: usize, round: u32, day: u32, month: u32, years: u32, week: u32, base: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize>(IDX_CUP_SET_ROUND_DRAW_DATE);
    let r = round_record_from_base(rounds, round);
    let (a, b, igual, mo, mr) = crate::comparison::side_by_side_light(&[(r, 3)], || o(rounds, round, day, month, years, week, base),
        || cup_set_round_draw_date(rounds, round, day, month, years, week, base));
    SCOREBOARD.record(igual && a == b, || format!("cup_set_round_draw_date({}, {}/{} +{}) {:x?} x {:x?}", round as i16, day, month, years, mo, mr));
    a
}

/// Comparer for `cup_set_round_match_date`.
pub(crate) unsafe extern "C" fn compare_cup_set_round_match_date(rounds: usize, round: u32, day: u32, month: u32, years: u32, week: u32, hour: u32, base: u32, mando: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize>(IDX_CUP_SET_ROUND_MATCH_DATE);
    let r = round_record_from_base(rounds, round);
    let (a, b, igual, mo, mr) = crate::comparison::side_by_side_light(&[(r + 3, 0x14)], || o(rounds, round, day, month, years, week, hour, base, mando),
        || cup_set_round_match_date(rounds, round, day, month, years, week, hour, base, mando));
    SCOREBOARD.record(igual && a == b, || format!("cup_set_round_match_date({}, {}/{} +{}) {:x?} x {:x?}", round as i16, day, month, years, mo, mr));
    a
}

/// Comparer for `apply_round_rules_to_match`.
pub(crate) unsafe extern "thiscall" fn compare_apply_round_rules_to_match(cup: usize, game: usize, key: i32) {
    let game = Fixture(game);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32)>(IDX_APPLY_ROUND_RULES_TO_MATCH);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(game.addr() + fixture::TIE_RULES, 2)], || o(cup, game.addr(), key),
        || apply_round_rules_to_match(cup, game.addr(), key));
    SCOREBOARD.record(igual, || format!("apply_round_rules_to_match(key {}) {:x?} x {:x?}", key, mo, mr));
}

/// Comparer for `find_tree_slot_for_fixture`.
pub(crate) unsafe extern "thiscall" fn compare_find_tree_slot_for_fixture(cup: usize, game: usize) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i32>(IDX_FIND_TREE_SLOT_FOR_FIXTURE)(cup, game);
    if game == 0 {
        return r;
    }
    let n = find_tree_slot_for_fixture(cup, game);
    SCOREBOARD.record(r == n, || format!("find_tree_slot_for_fixture original={} rust={}", r, n));
    r
}

/// Comparer for `cup_match_importance`.
pub(crate) unsafe extern "thiscall" fn compare_cup_match_importance(cup: usize, fixture: usize, second: u8, margin: *mut u8) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u8, *mut u8) -> u8>(IDX_CUP_MATCH_IMPORTANCE);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(margin as usize, 1)], || o(cup, fixture, second, margin), || cup_match_importance(cup, fixture, second, margin));
    SCOREBOARD.record(igual && r == n, || format!("cup_match_importance(side {}) original={} rust={} {:x?} x {:x?}", second, r, n, mo, mr));
    r
}

/// Comparer for `add_team_result_news`.
pub(crate) unsafe extern "thiscall" fn compare_add_team_result_news(cup: usize, club: usize, outcome: u32, a3: u32, p: usize, a5: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32>(IDX_ADD_TEAM_RESULT_NEWS)(cup, club, outcome, a3, p, a5)
}

/// Comparer for `cup_check_complete`.
///
/// [pt] Comparador: pelo valor (o método das fases só lê).
pub(crate) unsafe extern "thiscall" fn compare_cup_check_complete(this: usize, stages: u32, x: u32) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(IDX_CUP_CHECK_COMPLETE)(this, stages, x);
    let n = cup_check_complete(this, stages, x);
    SCOREBOARD.record(r == n, || format!("cup_check_complete({:x}, {}) original={} rust={}", this, stages, r, n));
    r
}

/// Comparer for `move_round_fixtures_to_set_dates`.
///
/// Comparer of `move_round_fixtures_to_set_dates`: only the original routine runs (the matches leave the calendar and come back through `add_fixture`,
/// which allocates, and the news items go to the managers).
///
/// The plan, made beforehand: the cup's matches on the date and slice (`collect_comp_fixtures_on_day` into a temporary list), in list order;
/// each one of the league's type takes the next slot (every 7 bytes, up to 0x38 and up to the first 0xff), and the slot
/// of the one on the round's date becomes 0xff; the other slots do not change.
///
/// Without the cup in [0xadadfc] afterwards: nothing to check.
///
/// [pt] Comparador da 0x51ebe0: só a original (os jogos saem do calendário e voltam pela 0x5aa680, que aloca, e as notícias
/// vão aos técnicos). O plano, feito antes: os jogos da copa na data e fatia (0x5abd90 numa lista provisória), na
/// ordem da lista; cada um do tipo da liga ocupa a vaga seguinte (de 7 em 7 bytes, até 0x38 e até a primeira 0xff), e
/// a vaga do que está na data da rodada fica 0xff; as outras vagas não mudam. Sem a copa em [0xadadfc] depois: nada a
/// conferir.
pub(crate) unsafe extern "thiscall" fn compare_move_round_fixtures_to_set_dates(this: usize, data: usize, round: u32) {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_MOVE_ROUND_FIXTURES_TO_SET_DATES);
    let e = cup_round_entry(this, round);
    if read_i16(e + cup_round::TIE_RULES) as u16 & 0x100 == 0 || read_u8(e + cup_round::SLOT_CODE) == 0xff {
        return o(this.addr(), data, round);
    }
    let id = read_usize(read_usize(this.addr() + 4));
    let mut list = [0u8; 0x10];
    let l = Llist(list.as_mut_ptr() as usize);
    exe::new_llist(l);
    let mut expected = [0u8; 8];
    for k in 0..8 {
        expected[k] = read_u8(e + k * 7 + 0x29);
    }
    if collect_round_fixtures(this, data, round, l) != 0 {
        l.set_cursor(l.head());
        let mut j = 0usize;
        let mut f = exe::llist_get_next(l);
        while f != 0 {
            if this.stage_index() == read_u8(f + fixture::STAGE_INDEX) && j < 0x38 && expected[j / 7] != 0xff {
                if fixture_on_round_date(this, e, Fixture(f)) {
                    expected[j / 7] = 0xff;
                }
                j += 7;
            }
            f = exe::llist_get_next(l);
        }
    }
    exe::drop_llist(l);
    o(this.addr(), data, round);
    if read_usize(read_usize(G_COMP_OBJECT_TABLE) + id * 4) == 0 {
        return; // simplification: the cup has left the game; nothing to check
    }
    let e = cup_round_entry(this, round);
    let after: [u8; 8] = core::array::from_fn(|k| read_u8(e + k * 7 + 0x29));
    SCOREBOARD.record(after == expected, || format!("move_round_fixtures_to_set_dates(round {}) slots {:02x?} (expected {:02x?})",
        round, after, expected));
}

/// Comparer for `map_fixture_tree` (comparison mode: see the header of motor.rs): the comparer of the tie tree; only
/// the original routine runs (simplification: it allocates the tie vectors and makes random draws; it cannot run
/// twice).
///
/// It checks what the original guarantees when it returns 1 with clubs: each tie of the mapped rounds points (`+6`
/// slot, `+8` side, `+0x12` round one or two ahead) to a slot that points back to it (`+0xa`/`+0xe` home,
/// `+0xc`/`+0x10` away).
///
/// [pt] O comparador da árvore dos confrontos: só a original (simplification: aloca os vetores dos confrontos e sorteia; não
/// roda duas vezes). Confere o que ela garante quando devolve 1 com clubes: cada confronto das rodadas mapeadas aponta
/// (+6 vaga, +8 lado, +0x12 rodada uma ou duas adiante) uma vaga que aponta de volta para ele (+0xa/+0xe mandante,
/// +0xc/+0x10 visitante).
pub(crate) unsafe extern "thiscall" fn compare_map_fixture_tree(this: usize) -> i32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_MAP_FIXTURE_TREE);
    let r = o(this.addr());
    if r != 1 || this.season_set_up() == 0 {
        return r;
    }
    let nr = this.round_count();
    let m = (if read_i16(comp_cup_round(this, nr as i32).wrapping_sub(0x61)) == 0x8c { nr.wrapping_sub(1) } else { nr }) as i32;
    let mut error: Option<(i32, i32)> = None;
    'fora: for rd in 0..(m - 1).max(0) {
        for p in 0..read_i16(comp_cup_round(this, rd) + 0x1a) as i32 {
            let c = cup_tie(this, rd, p);
            let target = read_i16(c + cup_tie::FEEDER_ROUND) as i32;
            let a = read_i16(c + cup_tie::FEEDER_SIDE) == 1;
            let ok = (target == rd + 1 || target == rd + 2) && {
                let v = cup_tie(this, target, read_i16(c + cup_tie::FEEDER_TIE) as i32);
                read_i16(v + if a { 0xa } else { 0xc }) as i32 == p && read_i16(v + if a { 0xe } else { 0x10 }) as i32 == rd
            };
            if !ok {
                error = Some((rd, p));
                break 'fora;
            }
        }
    }
    SCOREBOARD.record(error.is_none(), || format!("map_fixture_tree: o tie (round, index) {:?} without a slot of return", error));
    r
}

/// Comparer for `cup_rank_teams`.
///
/// It writes and may reallocate the stats (it does not run twice): before the original, the positions are planned on a
/// sorted copy of the array (the keys in a Vec); afterwards, each planned club's record must have its last position at
/// +0x72.
///
/// [pt] Comparador: grava e pode realocar a estatística (não roda duas vezes): antes da original, as posições são
/// planejadas sobre uma cópia ordenada do vetor (as chaves num Vec); depois, o registro de cada clube planejado tem
/// de ter em +0x72 a última posição dele.
pub(crate) unsafe extern "thiscall" fn compare_cup_rank_teams(this: usize) {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CUP_RANK_TEAMS);
    let n = this.team_count() as i32;
    if read_usize(this.addr() + 8) == 0 || n <= 0 {
        return o(this.addr()); // simplification: the box or nothing to sort, original only
    }
    let mut v: Vec<u32> = (0..n as usize).map(|i| read_usize(this.team_table() + i * 6) as u32).collect();
    let mut k = vec![0i32; n as usize];
    sort_clubs_by_strength(v.as_mut_ptr(), k.as_mut_ptr(), n);
    // simplification: with more slots in the rounds than clubs, the original reads past the vector (block leftover); the plan reads 0
    v.extend(core::iter::repeat(0).take(0x100));
    let plan = set_final_positions_by_round(this, v.as_ptr(), 0);
    o(this.addr());
    let mut erros = Vec::new();
    for (i, &(c, p)) in plan.iter().enumerate() {
        if c == 0 || plan[i + 1..].iter().any(|x| x.0 == c) {
            continue;
        }
        let e = exe::comp_stats_find_club_entry(read_usize(this.addr() + 8), c);
        if e == 0 || read_u8(e + comp_stats_club::DANGER_POSITION) != p {
            erros.push(format!("club {:x} expected {} got {}", c, p, if e == 0 { -1 } else { read_u8(e + comp_stats_club::DANGER_POSITION) as i32 }));
        }
    }
    SCOREBOARD.record(erros.is_empty(), || format!("cup_rank_teams({:x}) {} positions: {}", this.addr(), plan.len(), erros.join("; ")));
}

/// Comparer for `cup_set_round_match_day`.
pub(crate) unsafe extern "C" fn compare_cup_set_round_match_day(rounds: usize, round: i16, slot: i16, week: i16, hour: i8, code: i8, mando: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, i16, i16, i16, i8, i8, u32) -> u32>(IDX_CUP_SET_ROUND_MATCH_DAY);
    if !(0..8).contains(&slot) || !(-1..=6).contains(&week) || !(-1..3).contains(&hour) || !(-1..=3).contains(&code) {
        return o(rounds, round, slot, week, hour, code, mando);
    }
    let e = rounds.wrapping_add((round as isize * 104 + slot as isize * 7) as usize);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(e + cup_round::SLOT_VENUE, 7)], || o(rounds, round, slot, week, hour, code, mando),
        || cup_set_round_match_day(rounds, round, slot, week, hour, code, mando));
    SCOREBOARD.record(igual && r == n, || format!("cup_set_round_match_day({}, {}) {:x?} x {:x?}", round, slot, mo, mr));
    r
}

/// Comparer for `has_club`.
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "thiscall" fn compare_has_club(this: usize, club: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_HAS_CLUB)(this, club);
    let n = has_club(this, club);
    SCOREBOARD.record(r == n, || format!("has_club(cup {:x}, club {:x}) original={} rust={}", this, club, r, n));
    r
}
