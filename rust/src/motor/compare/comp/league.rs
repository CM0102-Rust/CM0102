use super::*;

// The comparers of the routines in ../comp/league.rs.
// [pt] Os comparadores das rotinas de ../comp/league.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `set_club_season_expectations`.
///
/// [pt] Só a original (as expectativas mudam a diretoria e mandam notícias; não rodam duas vezes): a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn compare_set_club_season_expectations(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SET_CLUB_SEASON_EXPECTATIONS)(this)
}

/// Comparer for `relegate_club_to_division`.
///
/// [pt] Só a original (cláusulas de contrato e o técnico que sai não rodam duas vezes), conferindo o que ela grava: +0x57 =
/// a divisão e +0x5b = a competição de antes.
pub(crate) unsafe extern "thiscall" fn compare_relegate_club_to_division(this: usize, club: usize, division: usize, x: u32) {
    let club = Club(club);
    let old = club.division().addr();
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_RELEGATE_CLUB_TO_DIVISION)(this, club.addr(), division, x);
    SCOREBOARD.record(club.division().addr() == division && club.last_division().addr() == old, || format!("relegate_club_to_division: +0x57 {:#x} +0x5b {:#x}", club.division().addr(), club.last_division().addr()));
}

/// Comparer for `move_fixtures_for_tv`.
///
/// Comparer of `move_fixtures_for_tv`: only the original routine runs (the matches leave the calendar and come back through `add_fixture`,
/// which allocates, the ratings draw at random and the news items go to the managers).
///
/// The plan, made beforehand: from the date's entry, the league's matches on its day and slice (`collect_comp_fixtures_on_day` into a
/// temporary list) and the valid slots (slice 0..2) before the first empty one; each valid slot takes one match while
/// the list has any (every rating gets past the initial −1), and the slot becomes 0xff: the 0xff slots of the block
/// after the original are min(valid slots, matches).
///
/// [pt] Comparador da 0x685640: só a original (os jogos saem do calendário e voltam pela 0x5aa680, que aloca, as notas
/// sorteiam e as notícias vão aos técnicos). O plano, feito antes: pela entrada da data, os jogos da liga no dia e fatia
/// dela (0x5abd90 numa lista provisória) e as vagas válidas (fatia 0..2) antes da primeira vazia; cada vaga válida leva
/// um jogo enquanto a lista tiver (toda nota passa do −1 inicial), e a vaga fica 0xff: as vagas 0xff do bloco depois da
/// original são min(válidas, jogos).
pub(crate) unsafe extern "thiscall" fn compare_move_fixtures_for_tv(this: usize, data: usize) {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_MOVE_FIXTURES_FOR_TV);
    let e = match tv_date_entry(this, data) {
        Some(e) => e,
        None => return o(this.addr(), data),
    };
    let mut block = 0usize;
    let mut validas = 0usize;
    while block < 8 && read_u8(e + block * 7 + 0xb) != 0xff {
        if (0..=2).contains(&(read_u8(e + block * 7 + 0xa) as i8)) {
            validas += 1;
        }
        block += 1;
    }
    let mut list = [0u8; 0x10];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    let n = (exe::fixture_manager_collect_comp_fixtures_on_day(read_usize(G_FIXTURE_MANAGER), read_i16(data + 2) as u16 as u32, read_i16(data) as u16 as u32, read_u8(e + 4) as u32,
        read_usize(this.competition().addr()) as u32, u32::MAX, this.stage_index() as u32, l) as u32) as usize & 0xffff;
    exe::drop_llist(l);
    o(this.addr(), data);
    let expected = if n == 0 { 0 } else { validas.min(n) };
    let movidas = (0..block).filter(|k| read_u8(e + k * 7 + 0xb) == 0xff).count();
    SCOREBOARD.record(movidas == expected, || format!("move_fixtures_for_tv(date {:x}) slots {} valid {} games {}: movidas {} (expected {})",
        data, block, validas, n, movidas, expected));
}

/// Comparer for `send_group_draw_news`.
///
/// [pt] Comparador da 0x684230: só a original (aloca a notícia e a entrega aos técnicos).
pub(crate) unsafe extern "thiscall" fn compare_send_group_draw_news(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SEND_GROUP_DRAW_NEWS)(this) // simplification: only the news item
}

/// Comparer for `send_fixture_list_news`.
///
/// [pt] Comparador da 0x683ee0: só a original (as notícias vão aos técnicos e a lista dos avisados cresce, alocando).
pub(crate) unsafe extern "thiscall" fn compare_send_fixture_list_news(this: usize, list: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SEND_FIXTURE_LIST_NEWS)(this, list) // simplification: only the news
}

/// Comparer for `league_update_for_date`.
///
/// The league-day comparer: only the original routine runs (simplification: it calls the methods of each stage and of
/// the league, the news items, the records and the TV matches, which touch the calendar: it does not run twice).
///
/// It checks what the routine guarantees: 0 only with the league stopped or without clubs and, when complete, +0xd1
/// with the date.
///
/// [pt] O comparador do dia da liga: só a original (simplification: chama os métodos de cada fase e da liga, as notícias, os
/// recordes e os jogos para a TV, que mexem no calendário: não roda duas vezes). Confere o que a rotina garante: 0 só
/// com a liga parada ou sem clubes e, completo, +0xd1 com a data.
pub(crate) unsafe extern "thiscall" fn compare_league_update_for_date(this: usize, data: usize, full: u32) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_LEAGUE_UPDATE_FOR_DATE);
    let parada = this.stopped() != 0 || this.active() == 0;
    let r = o(this.addr(), data, full);
    let ok = if parada { r as u8 == 0 } else { r as u8 == 1 && (full == 0 || same_date(this.addr() + league::LAST_UPDATE_DATE, data)) };
    SCOREBOARD.record(ok, || format!("league_update_for_date(full {}): stopped {} returned {} +0xd1 {:x} date {:x}",
        full, parada, r as u8, this.last_update_date(), read_usize(data)));
    r
}

/// Comparer for `create_fixtures`.
///
/// Only the original runs (simplification: it allocates the grid, draws the club order and puts the matches in the
/// calendar, which allocates: it does not run twice). It checks what the routine guarantees: with the year's own list,
/// that list is freed (+0xea = 0) when it returns 1.
///
/// [pt] O comparador da criação dos jogos: só a original (simplification: aloca a grade, sorteia a ordem dos clubes e põe os
/// jogos no calendário, que aloca: não roda duas vezes). Confere o que a rotina garante: com a lista própria do ano, ela
/// é liberada (+0xea = 0) quando devolve 1.
pub(crate) unsafe extern "thiscall" fn compare_create_fixtures(this: usize) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CREATE_FIXTURES);
    let list = this.fixed_fixtures();
    let own = list != 0 && this.year() as i32 == read_i16(list + 2) as u16 as i32;
    let r = o(this.addr());
    let ok = !(own && r == 1) || this.fixed_fixtures() == 0;
    SCOREBOARD.record(ok, || format!("create_fixtures: list own {} returned {} +0xea {:x}", own, r, this.fixed_fixtures()));
    r
}

/// Comparer for `league_promote_and_relegate`.
///
/// [pt] O comparador da promoção e do rebaixamento: só a original (simplification: os contratos, o técnico demitido e as notícias
/// não rodam duas vezes); a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn compare_league_promote_and_relegate(ecx: usize, above: usize, below: usize, contracts: u32, aos_pairs: u32, sobem: u32, descem: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32)>(IDX_LEAGUE_PROMOTE_AND_RELEGATE)(ecx, above, below, contracts, aos_pairs, sobem, descem)
}

/// Comparer for `update_positions_and_status` (comparison mode: see the header of motor.rs): the comparer of
/// `0x684cd0`, as a dry run (model RARE_FINANCE_EVENT_DRY_RUN).
///
/// The original routine runs fully first, recording the tape of the announcements (STATUS_ANNOUNCE_TAPE); memory is
/// restored and the copy runs without virtuals `+0x40` and `+0x68` and without the clauses (which send news, write
/// records and make random morale draws), putting back from the tape what each announcement's `+0x44` wrote into the
/// table.
///
/// Both start from the same state of the table, of `+0xa7` and of the European places of the table's clubs, which must
/// come out equal; at the end the original's state is kept. Without rows or with `+0xdd`: only the original routine.
///
/// [pt] Comparador da 0x684cd0, em ensaio (modelo RARE_FINANCE_EVENT_DRY_RUN): a original roda inteira primeiro, gravando a fita dos
/// anúncios (STATUS_ANNOUNCE_TAPE); a memória volta e a cópia roda sem as virtuais +0x40 e +0x68 nem as cláusulas (que mandam
/// notícia, gravam registros e sorteiam o moral), repondo da fita o que a +0x44 de cada anúncio gravou na tabela. As
/// duas a partir do mesmo estado da tabela, de +0xa7 e das vagas europeias dos clubes da tabela, que têm de sair
/// iguais; no fim fica o estado da original. Sem linhas ou com +0xdd: só a original.
pub(crate) unsafe extern "thiscall" fn compare_update_positions_and_status(this: usize) {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_UPDATE_POSITIONS_AND_STATUS);
    if this.complete() != 0 || this.club_count() <= 0 || this.table() == 0 {
        return o(this.addr());
    }
    let regions = table_status_regions(this);
    let before = crate::comparison::MemorySnapshot::read(&regions);
    *core::ptr::addr_of_mut!(STATUS_ANNOUNCE_TAPE) = Vec::new();
    STATUS_ANNOUNCE_TAPE_POS.store(0, Relaxed);
    *core::ptr::addr_of_mut!(STATUS_ANNOUNCE_RECORD) = Some(regions.clone());
    o(this.addr());
    *core::ptr::addr_of_mut!(STATUS_ANNOUNCE_RECORD) = None;
    let mo = crate::comparison::MemorySnapshot::read(&regions);
    before.write_back();
    POSITIONS_STATUS_DRY_RUN.store(true, Relaxed);
    crate::rng::pause_trace(|| update_positions_and_status(this.addr()));
    POSITIONS_STATUS_DRY_RUN.store(false, Relaxed);
    let mr = crate::comparison::MemorySnapshot::read(&regions);
    mo.write_back();
    SCOREBOARD.record(mo == mr, || format!("update_positions_and_status({:#x}) original={:x?} rust={:x?}", this.addr(), mo, mr));
}

/// Comparer for `pay_prize_money`.
///
/// With +0xe1 = 3 (the competition's virtual pays on its own) or a zero divisor: only the original routine.
///
/// Otherwise, the dry run: the copy without the prize news item (`send_prize_money_news` allocates) and the original from the same
/// state, on the finance records (0x167 bytes) of all the clubs in the table.
///
/// [pt] Comparador: com +0xe1 = 3 (a virtual da competição paga por conta própria) ou o divisor zero: só a original. Nos
/// outros casos, o ensaio: a cópia sem a notícia do prêmio (0x48e3e0 aloca) e a original a partir do mesmo estado, nos
/// registros de finanças (0x167 bytes) de todos os clubes da tabela.
pub(crate) unsafe extern "thiscall" fn compare_pay_prize_money(league: usize) {
    let league = League(league);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PAY_PRIZE_MONEY);
    let n = league.club_count();
    let e1 = league.prize_mode();
    if e1 == 3 || sum_of_positions(n) == 0 {
        return o(league.addr()); // simplification: the virtual or the division by zero, original only
    }
    let mut regions: Vec<(usize, usize)> = Vec::new();
    if e1 != 2 {
        for i in 0..n.max(0) as usize {
            let fin = european_cup_club_finance(read_usize(league.table() + 0x3b * i));
            if fin != 0 && !regions.iter().any(|r| r.0 == fin) {
                regions.push((fin, 0x167));
            }
        }
    }
    let before = crate::comparison::MemorySnapshot::read(&regions);
    LEAGUE_PRIZE_MONEY_DRY_RUN.store(true, Relaxed);
    crate::rng::pause_trace(|| crate::rng::without_counting(|| pay_prize_money(league.addr())));
    LEAGUE_PRIZE_MONEY_DRY_RUN.store(false, Relaxed);
    let mr = crate::comparison::MemorySnapshot::read(&regions);
    before.write_back();
    o(league.addr());
    let mo = crate::comparison::MemorySnapshot::read(&regions);
    SCOREBOARD.record(mo == mr, || format!("pay_prize_money({:x}) +0xe1 {} n {} original={:x?} rust={:x?}", league.addr(), e1, n, mo, mr));
}

/// Comparer for `league_update_staff_records_for_status`.
pub(crate) unsafe extern "thiscall" fn compare_league_update_staff_records_for_status(league: usize, club: usize, status: u32, stage: u32, out: usize, a: u32, b: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32) -> u32>(IDX_LEAGUE_UPDATE_STAFF_RECORDS_FOR_STATUS)(league, club, status, stage, out, a, b)
}

/// Comparer for `announce_team_status`.
pub(crate) unsafe extern "thiscall" fn compare_announce_team_status(league: usize, club: usize, status: u32, stage: u32, a: u32, b: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32>(IDX_ANNOUNCE_TEAM_STATUS);
    let Some(regioes) = (*core::ptr::addr_of!(STATUS_ANNOUNCE_RECORD)).clone() else {
        return o(league, club, status, stage, a, b);
    };
    let before = crate::comparison::MemorySnapshot::read(&regioes);
    let r = o(league, club, status, stage, a, b);
    let after = crate::comparison::MemorySnapshot::read(&regioes);
    let mut bytes = Vec::new();
    for ((base, x), (_, y)) in before.0.iter().zip(after.0.iter()) {
        bytes.extend((0..x.len()).filter(|&k| x[k] != y[k]).map(|k| (base + k, y[k])));
    }
    (*core::ptr::addr_of_mut!(STATUS_ANNOUNCE_TAPE)).push((club, status as u8, bytes));
    r
}

/// Comparer for `league_add_result`.
///
/// The null match and the stage match (the stage's virtual receives the match and compares on its own): only the
/// original. In the other cases, a dry run: the copy without the stats (`exe::comp_stats_record_match_result` finds or creates records) and the
/// original from the same state: the date at +0xc9, the table rows with the head-to-heads (the regions of `apply_result_to_table`),
/// the match's +0x3d/+0x3e and the return value.
///
/// [pt] Comparador: a partida nula e a da fase (a virtual da fase recebe a partida e compara por conta própria): só a
/// original. Nos outros casos, o ensaio: a cópia sem a estatística (0x49f570 acha ou cria registros) e a original a
/// partir do mesmo estado: a data em +0xc9, as linhas da tabela com os confrontos (as regiões da 0x685f60), o +0x3d/+0x3e
/// da partida e a devolução.
pub(crate) unsafe extern "thiscall" fn compare_league_add_result(league: usize, p: usize, game: usize) -> i32 {
    let p = Fixture(p);
    let league = League(league);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(IDX_LEAGUE_ADD_RESULT);
    if p.is_null() || (league.stage_index() == 0xff && p.stage_index() != 0xff) {
        return o(league.addr(), p.addr(), game); // simplification: the box or the stage, original only
    }
    let n = league.club_count() as u16 as u32;
    let mut regions = vec![(league.addr() + league::LAST_RESULT_DATE, 8)];
    regions.extend(result_update_regions(p, league.table(), n, core::ptr::null_mut(), core::ptr::null_mut()));
    let before = crate::comparison::MemorySnapshot::read(&regions);
    LEAGUE_ADD_RESULT_DRY_RUN.store(true, Relaxed);
    let rr = crate::rng::pause_trace(|| crate::rng::without_counting(|| league_add_result(league.addr(), p.addr(), game)));
    LEAGUE_ADD_RESULT_DRY_RUN.store(false, Relaxed);
    let mr = crate::comparison::MemorySnapshot::read(&regions);
    before.write_back();
    let ro = o(league.addr(), p.addr(), game);
    let mo = crate::comparison::MemorySnapshot::read(&regions);
    SCOREBOARD.record(ro == rr && mo == mr, || format!("league_add_result({:x}, match {:x}) original={} {:x?} rust={} {:x?}", league.addr(), p.addr(), ro, mo, rr, mr));
    ro
}

/// Comparer for `copy_table`.
///
/// It allocates (it cannot run twice), so only the original routine runs, with the full plan checked afterwards: the
/// expected +0xa7 (computed beforehand) and the returned copy equal to the table, row by row (except the pointers of
/// the new vectors), with the head-to-head and form vectors equal to the table's.
///
/// [pt] Comparador: aloca (não roda duas vezes): só a original, com o plano completo conferido depois: o +0xa7 esperado
/// (calculado antes) e a cópia devolvida igual à tabela, linha a linha (fora os ponteiros dos vetores novos), com os
/// vetores de confrontos e de forma iguais aos da tabela.
pub(crate) unsafe extern "thiscall" fn compare_copy_table(league: usize) -> usize {
    let league = League(league);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_COPY_TABLE);
    let table = league.table();
    let n = league.club_count();
    if table == 0 || n <= 0 || n > 0x32 {
        return o(league.addr()); // simplification: nothing to copy or the message box: original only
    }
    let a7 = if league.games_per_club() == -1 { n.wrapping_sub(1).wrapping_mul(league.legs()) } else { league.games_per_club() };
    let c = o(league.addr());
    let mut erros = Vec::new();
    if league.games_per_club() != a7 {
        erros.push(format!("+0xa7 {} expected {}", league.games_per_club(), a7));
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
                if read_u8(lt + b) != read_u8(lc + b) {
                    erros.push(format!("row {} +{:#x}", i, b));
                    break;
                }
            }
            let equal = |o_: usize, t: usize| {
                let (a, b) = (read_usize(lt + o_), read_usize(lc + o_));
                b != 0 && core::slice::from_raw_parts(a as *const u8, t) == core::slice::from_raw_parts(b as *const u8, t)
            };
            if h2h && n > 1 && !equal(5, 14 * (n - 1)) {
                erros.push(format!("row {} confrontos", i));
            }
            if forma && a7 > 0 && !equal(9, a7 as usize) {
                erros.push(format!("row {} form", i));
            }
        }
    }
    SCOREBOARD.record(c != 0 && erros.is_empty(), || format!("copy_table({:x}) n {} copy {:x}: {}", league.addr(), n, c, erros.join("; ")));
    c
}

/// Comparer for `league_set_predicted_positions`.
///
/// The routine writes and may create records in the stats (it does not run twice): the plan is made first, over a copy
/// of the vector sorted by the core of `comp_util_sort_clubs_by_strength` (keys in a Vec); afterwards, each club's record (the last time it
/// appears) must have the planned position in +0x72.
///
/// [pt] Comparador: grava e pode criar registros na estatística (não roda duas vezes): o plano sai antes, sobre uma cópia
/// do vetor ordenada pelo núcleo da 0x4c08f0 (chaves num Vec); depois, o registro de cada clube (a última vez que ele
/// aparece) tem de ter em +0x72 a posição planejada.
pub(crate) unsafe extern "thiscall" fn compare_league_set_predicted_positions(league: usize) {
    let league = League(league);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_LEAGUE_SET_PREDICTED_POSITIONS);
    let n = league.club_count() as i32;
    if league.comp_stats() == 0 || n <= 0 {
        return o(league.addr()); // simplification: the box or nothing to sort, original only
    }
    let mut v: Vec<u32> = (0..n as usize).map(|i| read_usize(league.table() + 0x3b * i) as u32).collect();
    let mut k = vec![0i32; n as usize];
    sort_clubs_by_strength(v.as_mut_ptr(), k.as_mut_ptr(), n);
    o(league.addr());
    let mut erros = Vec::new();
    for (i, &c) in v.iter().enumerate() {
        if c == 0 || v[i + 1..].contains(&c) {
            continue;
        }
        let p = (i as u8).wrapping_add(1);
        let e = CompStatsClub(exe::comp_stats_find_club_entry(league.comp_stats(), c as usize));
        if e.is_null() || e.danger_position() != p {
            erros.push(format!("club {:x} expected {} got {}", c, p, if e.is_null() { -1 } else { e.danger_position() as i32 }));
        }
    }
    SCOREBOARD.record(erros.is_empty(), || format!("league_set_predicted_positions({:x}) {} clubs: {}", league.addr(), n, erros.join("; ")));
}

/// Comparer for `compare_projection_by_reputation_desc`.
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn compare_compare_projection_by_reputation_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_PROJECTION_BY_REPUTATION_DESC)(a, b);
    let n = compare_projection_by_reputation_desc(a, b);
    SCOREBOARD.record(r == n, || format!("compare_projection_by_reputation_desc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `compare_projection_by_points_desc`.
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn compare_compare_projection_by_points_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_PROJECTION_BY_POINTS_DESC)(a, b);
    let n = compare_projection_by_points_desc(a, b);
    SCOREBOARD.record(r == n, || format!("compare_projection_by_points_desc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `update_projected_positions`.
///
/// The projection block is allocated and freed inside (nothing of it remains); the effect is virtual `+0x68`, which
/// writes bytes `+0x73`..`+0x75` of the club's record in the league's Comp_Stats (`+8`, found via `0x4a4850`, which
/// inside `0x4ae300` returns its temporary record): compared side by side on those 3 bytes and on the return value. The
/// two versions that forward to the other competition (`0x48e380`, `0x7ef100`) write outside the region: there only the
/// return value counts.
///
/// [pt] Comparador: o bloco da projeção é alocado e liberado dentro (não fica nada dele); o efeito é a virtual +0x68, que
/// grava os bytes +0x73..+0x75 do registro do clube no Comp_Stats da liga (+8, achado pela 0x4a4850, que dentro da
/// 0x4ae300 devolve o registro provisório dela): lado a lado nesses 3 bytes e na devolução. As duas versões que repassam
/// a outra competição (0x48e380, 0x7ef100) gravam fora da região: aí só a devolução pesa.
pub(crate) unsafe extern "thiscall" fn compare_update_projected_positions(this: usize, club: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_UPDATE_PROJECTED_POSITIONS);
    let stats = read_usize(this + 8);
    let reg = CompStatsClub(if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 });
    if reg.is_null() {
        return o(this, club); // simplification: no record to look at, original only
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(reg.addr() + comp_stats_club::CURRENT_POSITION, 3)], || o(this, club),
        || update_projected_positions(this, club));
    SCOREBOARD.record(igual && r == n, || format!("update_projected_positions(league {:x}, club {:x}) original={} rust={} {:x?} x {:x?}",
        this, club, r, n, mo, mr));
    r
}

/// Comparer for `init_table_row`.
///
/// The original allocates (history and head-to-heads): only it runs, and the row is checked against the plan made
/// beforehand (the total rounds, the club's status, the fields, the zeroed blocks, the arrays present and, on the last
/// row, each row's opponents).
///
/// [pt] Comparador: a original aloca (histórico e confrontos): roda só ela e confere a linha com o plano feito antes (o
/// total de rodadas, a situação do clube, os campos, os blocos zerados, os vetores presentes e, na última linha, os
/// adversários de cada linha).
pub(crate) unsafe extern "thiscall" fn compare_init_table_row(this: usize, i: u32, club: usize, mark: u32, rows: usize) {
    let this = League(this);
    let club = Club(club);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32, usize)>(IDX_INIT_TABLE_ROW);
    let a7 = this.games_per_club();
    let n = this.club_count();
    let a7_expected = if a7 == -1 && n > 0 { (n - 1).wrapping_mul(this.legs()) } else { a7 };
    let status = if !club.is_null() { club.professional_status() } else { 0 };
    let status_expected = if status == 3 { 2 } else { status };
    let table = if rows == 0 { this.table() } else { rows };
    let confrontos = uses_head_to_head_rule(this);
    o(this.addr(), i, club.addr(), mark, rows);
    let idx = i as u8 as i8 as isize;
    let l = LeagueTableRow(table.wrapping_add((idx * TABLE_ROW_SIZE as isize) as usize));
    let mut ok = read_usize(l.addr()) == club.addr() && (l.position() as u8) == i as u8 && l.status() == 0xff && l.mark() == mark as u8
        && l.best_possible_position() == 1 && l.worst_possible_position() == n as u8 && this.games_per_club() == a7_expected
        && (club.is_null() || club.professional_status() == status_expected)
        && (l.position_history() != 0) == (this.legs() > 0) && (l.h2h_entries() != 0) == confrontos;
    for k in 0..3 {
        let e = l.addr() + league_table_row::PLAYED + 14 * k;
        ok &= (e - 4..e + 10).all(|a| read_u8(a) == 0);
    }
    let last = idx == n as isize - 1 && l.h2h_entries() != 0;
    if l.h2h_entries() != 0 && !last {
        let p = l.h2h_entries();
        ok &= (0..(n as usize).saturating_sub(1)).all(|k| (p + 14 * k + 4..p + 14 * k + 14).all(|a| read_u8(a) == 0));
    }
    if last {
        for a in 0..n as usize {
            let p = read_usize(table + a * TABLE_ROW_SIZE + 5);
            let outros = (0..n as usize).filter(|&j| j != a);
            ok &= outros.enumerate().all(|(k, j)| read_usize(p + 14 * k) == read_usize(table + j * TABLE_ROW_SIZE));
        }
    }
    SCOREBOARD.record(ok, || format!("init_table_row(row {}, club {:x}, mark {}) clubs {}, rounds {} (esperava {}), confrontos {} ({:x}), history {:x}",
        idx, club.addr(), mark as u8, n, this.games_per_club(), a7_expected, confrontos, l.h2h_entries(), l.position_history()));
}

/// Comparer for `league_get_club_array`.
///
/// [pt] Comparador: a original aloca o vetor: roda só ela e confere a conta e os clubes, na ordem da tabela.
pub(crate) unsafe extern "thiscall" fn compare_league_get_club_array(this: usize, out: *mut usize, n: *mut i16) {
    let this = League(this);
    original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, *mut i16)>(IDX_LEAGUE_GET_CLUB_ARRAY)(this.addr(), out, n);
    let p = *out;
    if p == 0 {
        return; // out of memory: the box has already shown
    }
    let count = this.club_count();
    let ok = *n == count && (0..count.max(0) as usize).all(|i| read_usize(p + 4 * i) == read_usize(this.table() + i * TABLE_ROW_SIZE));
    SCOREBOARD.record(ok, || format!("league_get_club_array count {} (table {})", *n, count));
}

/// Comparer for `record_final_positions`.
///
/// [pt] Comparador: lado a lado pela divisão da última temporada e pela posição (+0x5b..+0x5f) de cada clube da tabela.
pub(crate) unsafe extern "thiscall" fn compare_record_final_positions(this: usize) {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_RECORD_FINAL_POSITIONS);
    let reg: Vec<(usize, usize)> = (0..this.club_count().max(0) as usize)
        .map(|i| (read_usize(this.table() + i * TABLE_ROW_SIZE) + 0x5b, 5))
        .collect();
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this.addr()), || record_final_positions(this.addr()));
    SCOREBOARD.record(igual, || format!("record_final_positions {:x?} x {:x?}", mo, mr));
}

/// Comparer for `apply_result_to_table`.
pub(crate) unsafe extern "thiscall" fn compare_apply_result_to_table(league: usize, p: usize, table: usize, n: u32, mark: u32, pts_home: *mut i16, pts_away: *mut i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, *mut i16, *mut i16)>(IDX_APPLY_RESULT_TO_TABLE);
    let reg = result_update_regions(Fixture(p), table, n, pts_home, pts_away);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(league, p, table, n, mark, pts_home, pts_away),
        || apply_result_to_table(league, p, table, n, mark, pts_home, pts_away));
    SCOREBOARD.record(igual, || format!("apply_result_to_table({:x}, match {:x}, {:x}, n {}, mark {}) {:x?} x {:x?}",
        league, p, table, n as i16, mark, mo, mr));
}

/// Comparer for `league_compare_table_rows_h2h`.
pub(crate) unsafe extern "C" fn compare_league_compare_table_rows_h2h(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_LEAGUE_COMPARE_TABLE_ROWS_H2H)(a, b);
    let n = league_compare_table_rows_h2h(a, b);
    SCOREBOARD.record(r == n, || format!("league_compare_table_rows_h2h({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `league_compare_dates`.
pub(crate) unsafe extern "C" fn compare_league_compare_dates(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_LEAGUE_COMPARE_DATES)(a, b);
    let n = league_compare_dates(a, b);
    SCOREBOARD.record(r == n, || format!("league_compare_dates({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `sort_table`.
pub(crate) unsafe extern "thiscall" fn compare_sort_table(league: usize, rows: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SORT_TABLE);
    let reg = sort_regions(League(league), rows);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(league, rows), || sort_table(league, rows));
    SCOREBOARD.record(igual && r == n, || format!("sort_table({:x}, {:x}) {}/{} {:x?} x {:x?}", league, rows, r, n, mo, mr));
    r
}

/// Comparer for `build_h2h_table`.
pub(crate) unsafe extern "thiscall" fn compare_build_h2h_table(league: usize, rows: usize) {
    let league = League(league);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_BUILD_H2H_TABLE);
    if rows == 0 {
        return o(league.addr(), rows);
    }
    let reg = [(rows, LEAGUE_TABLE_ROW_SIZE * league.club_count().max(0) as usize)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(league.addr(), rows), || build_h2h_table(league.addr(), rows));
    SCOREBOARD.record(igual, || format!("build_h2h_table({:x}, {:x}) {:x?} x {:x?}", league.addr(), rows, mo, mr));
}

/// Comparer for `free_table`.
pub(crate) unsafe extern "thiscall" fn compare_free_table(league: usize, table: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FREE_TABLE)(league, table)
}

/// Comparer for `league_check_complete`.
pub(crate) unsafe extern "thiscall" fn compare_league_check_complete(league: usize, stages: u32, warn: u32) -> u8 {
    let league = League(league);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(IDX_LEAGUE_CHECK_COMPLETE);
    if !is_complete_check_read_only(league, stages) {
        return o(league.addr(), stages, warn); // end of league (records, prizes, stages, notice): original only
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(league.addr() + league::GAMES_PER_CLUB, 2)], || o(league.addr(), stages, warn), || league_check_complete(league.addr(), stages, warn));
    SCOREBOARD.record(igual && r == n, || format!("league_check_complete({:x}, {}, {}) {}/{} {:x?} x {:x?}", league.addr(), stages, warn, r, n, mo, mr));
    r
}

/// Comparer for `pay_tv_rights`.
///
/// [pt] Comparador: as finanças (0x167 bytes) de cada clube da tabela; a cópia roda sem a notícia (a da original fica, uma
/// vez só). Clube nulo na tabela (a caixa da 0x5a0590): só a original.
pub(crate) unsafe extern "thiscall" fn compare_pay_tv_rights(league: usize, value: i32, filter: usize) {
    let league = League(league);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, usize)>(IDX_PAY_TV_RIGHTS);
    let finances = read_usize(G_FINANCE_TABLE);
    let n = league.club_count().max(0) as usize;
    let clubs: Vec<usize> = (0..n).map(|i| read_usize(league.table() + 0x3b * i)).collect();
    if finances == 0 || clubs.contains(&0) {
        return o(league.addr(), value, filter);
    }
    let reg: Vec<(usize, usize)> = clubs.iter().map(|&c| (exe::financetable_get_club_finance(finances, c), 0x167)).collect();
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(league.addr(), value, filter), || pay_tv_money(league, value, filter, false));
    SCOREBOARD.record(igual, || format!("pay_tv_rights({:x}, {}, {:x}) {:x?} x {:x?}", league.addr(), value, filter, mo, mr));
}

/// Comparer for `league_record_season_honours`.
///
/// [pt] Comparador: os clubes (o +0x5b) e o histórico de competições. Sem linha livre no histórico (a 0x4afce0 faria
/// realloc): só a original.
pub(crate) unsafe extern "thiscall" fn compare_league_record_season_honours(league: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_LEAGUE_RECORD_SEASON_HONOURS);
    if free_comp_history_row() == 0 {
        return o(league);
    }
    let mut reg = comp_history_regions();
    let how_many = read_i32(G_CLUB_COUNT).wrapping_sub(read_i32(G_NATION_COUNT)).max(0) as usize;
    reg.push((read_usize(G_CLUB_TABLE), 0x245 * how_many));
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(league), || league_record_season_honours(league));
    SCOREBOARD.record(igual && r == n, || format!("league_record_season_honours({:x}) {:x}/{:x} {:x?} x {:x?}", league, r, n, mo, mr));
    r
}

/// Comparer for `has_relegation`.
pub(crate) unsafe extern "thiscall" fn compare_has_relegation(league: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HAS_RELEGATION)(league);
    let n = has_relegation(league);
    SCOREBOARD.record(r == n, || format!("has_relegation original={} rust={}", r, n));
    r
}

/// Comparer for `league_round_set_date`.
pub(crate) unsafe extern "C" fn compare_league_round_set_date(rounds: usize, i: u32, day: u32, month: u32, years: u32, week: u32, code: u32, base: u32, value: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize>(IDX_LEAGUE_ROUND_SET_DATE);
    let r = rounds.wrapping_add((i as i16 as isize * 0x41) as usize);
    let (a, b, igual, mo, mr) = crate::comparison::side_by_side_light(&[(r, 0x41)], || o(rounds, i, day, month, years, week, code, base, value),
        || league_round_set_date(rounds, i, day, month, years, week, code, base, value));
    SCOREBOARD.record(igual && a == b, || format!("league_round_set_date({}, {}/{} +{}) {:x?} x {:x?}", i as i16, day, month, years, mo, mr));
    a
}

/// Comparer for `get_match_importance`.
pub(crate) unsafe extern "thiscall" fn compare_get_match_importance(league: usize, fixture: usize, side: u32, out: *mut u8) -> u8 {
    // 0x68a4d0 only reads the league and writes to the local and to `saida`: it can run on both sides
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut u8) -> u8>(IDX_GET_MATCH_IMPORTANCE);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(out as usize, 1)], || o(league, fixture, side, out),
        || get_match_importance(league, fixture, side, out));
    SCOREBOARD.record(igual && r == n, || format!("get_match_importance original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

/// Comparer for `league_compare_table_rows`.
pub(crate) unsafe extern "C" fn compare_league_compare_table_rows(a: usize, b: usize) -> i32 {
    // the private comparison of [0xb67a00] (if any) runs on both sides: it only reads the rows
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_LEAGUE_COMPARE_TABLE_ROWS)(a, b);
    let n = league_compare_table_rows(a, b);
    SCOREBOARD.record(r == n, || format!("league_compare_table_rows original={} rust={}", r, n));
    r
}

/// Comparer for `league_build_round_robin_grid`.
pub(crate) unsafe extern "stdcall" fn compare_league_build_round_robin_grid(grid: *const *mut i32, n: i32) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(*const *mut i32, i32) -> u32>(IDX_LEAGUE_BUILD_ROUND_ROBIN_GRID);
    let regions: Vec<(usize, usize)> = (1..=n.max(0)).map(|i| (*grid.offset(i as isize) as usize + 4, (n.max(1) as usize - 1) * 4)).collect();
    let (r, s, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(grid, n), || league_build_round_robin_grid(grid, n));
    SCOREBOARD.record(igual && r == s, || format!("league_build_round_robin_grid({}) {:x?} x {:x?}", n, mo, mr));
    r
}

/// Comparer for `final_day_margin_needed`.
pub(crate) unsafe extern "thiscall" fn compare_final_day_margin_needed(league: usize, row: usize, out: *mut u8) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u8>(IDX_FINAL_DAY_MARGIN_NEEDED);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(out as usize, 1)], || o(league, row, out),
        || final_day_margin_needed(league, row, out));
    SCOREBOARD.record(igual && r == n, || format!("final_day_margin_needed original={:#x} rust={:#x} {:x?} x {:x?}", r, n, mo, mr));
    r
}

/// Comparer for `league_compare_h2h_rows`.
pub(crate) unsafe extern "C" fn compare_league_compare_h2h_rows(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_LEAGUE_COMPARE_H2H_ROWS)(a, b);
    let n = league_compare_h2h_rows(a, b);
    SCOREBOARD.record(r == n, || format!("league_compare_h2h_rows original={} rust={}", r, n));
    r
}

/// Comparer for `update_head_to_head`.
pub(crate) unsafe extern "thiscall" fn compare_update_head_to_head(league: usize, table: usize, row: i32, opponent: i32, pro: i8, contra: i8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, i32, i8, i8)>(IDX_UPDATE_HEAD_TO_HEAD);
    if row < 0 || opponent < 0 {
        return o(league, table, row, opponent, pro, contra);
    }
    let e = head_to_head_record(table, row, opponent);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(e, 14)], || o(league, table, row, opponent, pro, contra),
        || update_head_to_head(league, table, row, opponent, pro, contra));
    SCOREBOARD.record(igual, || format!("update_head_to_head({}, {}, {}x{}) {:x?} x {:x?}", row, opponent, pro, contra, mo, mr));
}

/// Comparer for `rotate_round`.
pub(crate) unsafe extern "C" fn compare_rotate_round(pos: i32, step_towards: *mut i8, league: i32, rounds: i16, turnos: i16, n: i16, marks: u8) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(i32, *mut i8, i32, i16, i16, i16, u8) -> i32>(IDX_ROTATE_ROUND);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((step_towards as usize, 1));
    let (r, v, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(pos, step_towards, league, rounds, turnos, n, marks), || crate::rng::without_counting(|| rotate_round(pos, step_towards, league, rounds, turnos, n, marks)));
    SCOREBOARD.record(igual && r == v, || format!("rotate_round({}, {}, {}, {}, {}, {:#x}) original={} {:x?} rust={} {:x?}", pos, league, rounds, turnos, n, marks, r, mo, v, mr));
    r
}

/// Comparer for `store_round_match`.
pub(crate) unsafe extern "C" fn compare_store_round_match(base: usize, round: i16, j: i16, a: i16, b: i8, c: i8, value: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i16, i16, i16, i8, i8, u32)>(IDX_STORE_ROUND_MATCH);
    let e = base.wrapping_add((round as isize * 0x41 + j as isize * 7) as usize);
    let reg = [(e + 5, 7)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(base, round, j, a, b, c, value), || store_round_match(base, round, j, a, b, c, value));
    SCOREBOARD.record(igual, || format!("store_round_match({:#x}, {}, {}, {}, {}, {}, {:#x}) {:x?} x {:x?}", base, round, j, a, b, c, value, mo, mr));
}

/// Comparer for `calc_prize_fund`.
pub(crate) unsafe extern "thiscall" fn compare_calc_prize_fund(league: usize, quota: i32) -> i32 {
    let league = League(league);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> i32>(IDX_CALC_PRIZE_FUND)(league.addr(), quota);
    if league.club_count() == 0 {
        return r;
    }
    let n = calc_prize_fund(league.addr(), quota);
    SCOREBOARD.record(r == n, || format!("calc_prize_fund({}) original={} rust={}", quota, r, n));
    r
}

/// Comparer for `league_process_promotion_relegation_chain`.
pub(crate) unsafe extern "thiscall" fn compare_league_process_promotion_relegation_chain(this: usize, aos_pairs: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_LEAGUE_PROCESS_PROMOTION_RELEGATION_CHAIN)(this, aos_pairs)
}

/// Comparer for `move_club_to_division`.
///
/// [pt] Comparador: a mudança chama as notícias, a vaga de técnico e a obra (não podem rodar duas vezes): só a original (a
/// prova é a carreira e o modo Rust).
pub(crate) unsafe extern "thiscall" fn compare_move_club_to_division(this: usize, club: usize, division: usize, slot: u32) {
    crate::comparison::branch("move_club_to_division");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_MOVE_CLUB_TO_DIVISION)(this, club, division, slot)
}

/// Comparer for `draw_team_order`.
///
/// [pt] Comparador: embaralha com o sorteio e o rand, aloca e chama a tabela virtual: só a original.
pub(crate) unsafe extern "thiscall" fn compare_draw_team_order(this: usize, clubs: i32) {
    // simplification: rolls and allocates (does not run twice); the copy is proven by the career (the fixture grid of every league)
    original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_DRAW_TEAM_ORDER)(this, clubs)
}

/// Comparer for `limit_club_division_tier`.
///
/// [pt] Comparador: grava só o byte +0x64 dos clubes da tabela: lado a lado nesses bytes.
pub(crate) unsafe extern "thiscall" fn compare_limit_club_division_tier(this: usize, value: u32, away: u32, inside: u32, baixar: u32) {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32)>(IDX_LIMIT_CLUB_DIVISION_TIER);
    let n = this.club_count();
    let table = this.table();
    let reg: Vec<(usize, usize)> = (0..n.max(0) as usize).map(|i| (read_usize(table + i * 0x3b) + 0x64, 1)).collect();
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this.addr(), value, away, inside, baixar),
        || limit_club_division_tier(this.addr(), value, away, inside, baixar));
    SCOREBOARD.record(igual, || format!("limit_club_division_tier(value {}, away {:x}, inside {:x}, baixar {}) {:x?} x {:x?}",
        value as u8 as i8, away as u8, inside as u8, baixar, mo, mr));
}

/// Comparer for `narrate_qualification`.
pub(crate) unsafe extern "thiscall" fn compare_narrate_qualification(this: usize, side: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_NARRATE_QUALIFICATION);
    if is_recording_top_level() {
        call_recording(FN_CM3_MATCH_ENGINE_POST_PERIOD_END_EVENTS, this, [side, 0, 0, 0, 0], || {
            o(this, side);
            0
        });
        return;
    }
    let (igual, n) = compare_with_commentary(|| o(this, side), |c| comment_standing_with(Engine(this), side, c));
    SCOREBOARD.record(igual, || format!("narrate_qualification({}) {} chamadas", side as u8, n));
}
