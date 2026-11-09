use super::*;

// The comparers of the routines in ../national_teams.rs.
// [pt] Os comparadores das rotinas de ../national_teams.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `send_club_international_report_news`.
///
/// Comparer of `send_club_international_report_news`: the original routine delivers the news items (it allocates in the inboxes), so only it runs,
/// with the random draws traced and the squad saved beforehand.
///
/// Then the squad is put back as it was, the copy redoes the work in rehearsal mode (draws from the tape, without
/// delivering) and the squad must come out equal to the original's, with the rehearsal's draws equal to the tape's
/// (same n, all used).
///
/// [pt] Comparador da 0x7706c0: a original entrega as notícias (aloca nas caixas): só ela roda, com os sorteios rastreados e o
/// elenco guardado antes. Depois o elenco volta ao de antes, a cópia refaz em ensaio (sorteios da fita, sem entregar) e o
/// elenco tem de sair igual ao da original, com os sorteios do ensaio iguais aos da fita (mesmos n, todos usados).
pub(crate) unsafe extern "thiscall" fn compare_send_club_international_report_news(this: usize, club: usize, game: usize, result: usize, highlight: usize, squad: *mut usize, count: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, *mut usize, u32)>(IDX_SEND_CLUB_INTERNATIONAL_REPORT_NEWS);
    let q = (count as u8 as i8).max(0) as usize;
    let before: Vec<usize> = (0..q).map(|k| *squad.add(k)).collect();
    let ((), tape_lock) = crate::rng::trace(|| o(this, club, game, result, highlight, squad, count));
    let after: Vec<usize> = (0..q).map(|k| *squad.add(k)).collect();
    let n = tape_lock.len();
    for k in 0..q {
        *squad.add(k) = before[k];
    }
    *core::ptr::addr_of_mut!(INTERNATIONAL_REPORT_TAPE) = Some((tape_lock, 0, false));
    crate::rng::without_counting(|| send_club_international_report_news(this, club, game, result, highlight, squad, count));
    let (k, falhou) = match &*core::ptr::addr_of!(INTERNATIONAL_REPORT_TAPE) {
        Some((_, k, f)) => (*k, *f),
        None => (0, true),
    };
    *core::ptr::addr_of_mut!(INTERNATIONAL_REPORT_TAPE) = None;
    let after_r: Vec<usize> = (0..q).map(|k| *squad.add(k)).collect();
    for k in 0..q {
        *squad.add(k) = after[k];
    }
    SCOREBOARD.record(!falhou && k == n && after == after_r, || format!("send_club_international_report_news(clube {:#x}, jogo {:#x}, conta {}) tape_lock de {} sorteios, ensaio usou {} (n diferente: {}); elenco original {:x?} copia {:x?}",
        club, game, q, n, k, falhou, after, after_r));
}

/// Comparer for `national_teams_post_call_up_news`.
///
/// Comparer of `national_teams_post_call_up_news`: it only builds and delivers news items (allocating in the humans' inboxes) and builds and
/// frees the opponents list: it cannot run twice and writes nothing of the game besides the inboxes.
///
/// Only the original routine runs (simplification: the effect is the human's inbox; the opponents list is checked
/// through `national_teams_collect_fixture_opponents`).
///
/// [pt] Comparador da 0x76e0b0: ela só monta e entrega notícias (aloca nas caixas dos humanos) e monta e libera a lista dos
/// adversários: não pode rodar duas vezes e não grava nada do jogo além das caixas. Só a original roda (simplification: o
/// efeito é a caixa de entrada do humano; a lista dos adversários é conferida pela 0x76d560).
pub(crate) unsafe extern "thiscall" fn compare_national_teams_post_call_up_news(this: usize, national_team: usize, second: u32, game: usize, kind: u32, sem_adversarios: u32, adversarios: usize, count: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, usize, u32)>(IDX_NATIONAL_TEAMS_POST_CALL_UP_NEWS)(this, national_team, second, game, kind, sem_adversarios, adversarios, count);
}

/// Comparer for `national_teams_collect_fixture_opponents`.
///
/// [pt] Comparador da 0x76d560: a original aloca a lista nova (não pode rodar duas vezes): só ela roda; antes, a cópia em
/// ensaio diz os valores que vão na lista (sem alocar), e a lista da original tem de ter exatamente esses, na ordem.
pub(crate) unsafe extern "thiscall" fn compare_national_teams_collect_fixture_opponents(this: usize, national_team: usize, game: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(IDX_NATIONAL_TEAMS_COLLECT_FIXTURE_OPPONENTS);
    *core::ptr::addr_of_mut!(FIXTURE_OPPONENTS_PLAN) = Some(Vec::new());
    national_teams_collect_fixture_opponents(this, national_team, game);
    let plan = (*core::ptr::addr_of_mut!(FIXTURE_OPPONENTS_PLAN)).take().unwrap_or_default();
    let r = o(this, national_team, game);
    let now = list_item_values(r);
    SCOREBOARD.record(now == plan, || format!("national_teams_collect_fixture_opponents(seleção {:#x}, jogo {:#x}) plano {:x?} original {:x?}", national_team, game, plan, now));
    r
}

/// Comparer for `select_squad`.
///
/// The original allocates (`exe::new_llist`/0x68d0d0), changes the squads and the tactic slots and sends news (`national_teams_post_call_up_news`,
/// `exe::national_teams_post_squad_request_news`): it cannot run twice. Only it runs, with a tape recording open on the squad_selection_regions: the lineup
/// picker (0x6e2000) enters it as a unit, with the input arrays (the list and the codes) and the state of the regions
/// just before. Afterwards, with that state restored, the Rust copy builds the list (collect_squad_candidates) and the
/// codes (squad_position_codes), which must match the original's, together with the count, the mode and the team; the
/// original's after-state is restored at the end.
///
/// In the squads of 23 and 25 the original passes stack leftover in the slots from the 22nd on: those are left out
/// (known fix, the branch); the first 22 and the rest are checked. With no lineup pick (human manager), nothing to
/// compare. Inside an outer recording, only the original runs.
///
/// [pt] Comparador: a original aloca (0x68d020/0x68d0d0), mexe nos elencos, nas vagas da tática e manda notícias (0x76e0b0,
/// 0x76f1a0): não pode rodar duas vezes. Só ela roda, com uma gravação da fita aberta nas regiões de squad_selection_regions:
/// a escala (0x6e2000) entra nela como unidade com os vetores de entrada (a lista e os códigos) e o estado das
/// regiões logo antes. Depois, com esse estado de volta, a cópia Rust monta a lista (collect_squad_candidates) e os códigos
/// (squad_position_codes), que têm de bater com os da original, junto com a quantidade, o modo e o time; o estado de depois
/// da original volta no fim. Nos elencos de 23 e 25 a original passa a sobra da pilha nas vagas a partir da 22ª: elas
/// ficam fora (correção conhecida, o ramo); as 22 primeiras e o resto conferem. Sem escala (técnico humano), nada a
/// comparar. Dentro de uma gravação de fora, só a original.
pub(crate) unsafe extern "thiscall" fn compare_select_squad(this: usize, nation: usize, kind: u32, data: usize, p: usize, forcado: u32) {
    use std::sync::atomic::Ordering::Relaxed;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, u32)>(IDX_SELECT_SQUAD);
    if is_recording() {
        return o(this, nation, kind, data, p, forcado);
    }
    let t = kind as u8 as i8;
    let regions = squad_selection_regions(this, nation, t, p);
    let externa = tape_lock().take();
    *tape_lock() = Some((regions.clone(), Vec::new()));
    let level = RECORDABLE_DEPTH.swap(0, Relaxed);
    o(this, nation, kind, data, p, forcado);
    let gravadas = tape_lock().take().map_or(Vec::new(), |f| f.1);
    RECORDABLE_DEPTH.store(level, Relaxed);
    *tape_lock() = externa;
    let Some((_, _, a, _, antes, _, (ent, _))) = gravadas.iter().find(|c| c.0 == 0x6e2000) else {
        return;
    };
    let after = crate::comparison::MemorySnapshot::read(&regions);
    let sorteio = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    antes.write_back();
    let reg = squad_nation_record(this, nation);
    let r = reg.wrapping_add((t as i32 * 0x14) as usize);
    let size = read_u8(r + 0x42);
    let (elenco, _, _) = squad_team_for_type(nation, t);
    let mode: u32 = if t == 1 { 0x2100914 } else if t != 2 { 0x2000814 } else { 0x814 };
    let mut cand = [0usize; 200];
    let mut cursor = 0usize;
    let (n, codigos) = crate::rng::pause_trace(|| crate::rng::without_counting(|| {
        let n = collect_squad_candidates(this, nation, t, data, p, elenco, reg, reputation_limits(nation, Fixture(p)), &mut cand, &mut cursor);
        (n, squad_position_codes(size))
    }));
    sorteio.write_back();
    after.write_back();
    let n1 = (a[1] as u8 as i8).max(0) as usize;
    let dw = |k: usize| u32::from_le_bytes([ent[k], ent[k + 1], ent[k + 2], ent[k + 3]]) as usize;
    let list: Vec<usize> = (0..n1.min(ent.len() / 4)).map(|k| dw(4 * k)).collect();
    let code: Vec<u16> = ent.get(4 * n1..).unwrap_or(&[]).chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
    let corrigido = size == 23 || size == 25;
    let lim = if corrigido { 22.min(code.len()) } else { code.len() };
    let same = a[1] as u8 == n && a[2] == mode && a[3] as usize == elenco && code.len() == size as usize
        && list[..] == cand[..n1.min(200)] && code[..lim] == codigos[..lim];
    SCOREBOARD.record(same, || format!(
        "select_squad(nação {:x}, tipo {}, tam {}) original: n {} modo {:#x} time {:x} lista {:x?} códigos {:x?}; rust: n {} modo {:#x} time {:x} lista {:x?} códigos {:x?}",
        nation, t, size, a[1] as u8, a[2], a[3], list, code, n, mode, elenco, &cand[..(n as i8).max(0) as usize], &codigos[..size as usize]));
}

/// Comparer for `get_international_duty_club`.
///
/// [pt] Comparador da 0x7757d0 (2,1 milhões de chamadas até 2002-06-01: o lado a lado leve, sem alocar até 256 bytes): cada
/// uma com a sua lista provisória (como a original); no texto de 0xde1f64 (0x80) e no do comentário (0x40) e na devolução.
pub(crate) unsafe extern "thiscall" fn compare_get_international_duty_club(this: usize, player: usize, data: usize, club: usize, comentario: usize,
                                                               opponent: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize) -> usize>(IDX_GET_INTERNATIONAL_DUTY_CLUB);
    let regions = [(0xde1f64usize, 0x80usize), (comentario, 0x40)];
    let reg = if comentario != 0 { &regions[..] } else { &regions[..1] };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(reg,|| o(this, player, data, club, comentario, opponent),
        || get_international_duty_club(this, player, data, club, comentario, opponent));
    SCOREBOARD.record(igual && r == n, || format!("get_international_duty_club(jogador {:x}, data {:x}, clube {:x}, adversario {:x}) original={:x} rust={:x} {:x?} x {:x?}",
        player, data, club, opponent, r, n, mo, mr));
    r
}

/// Comparer for `national_teams_player_is_eligible`.
///
/// [pt] Comparador da 0x76d9b0: pela devolução (só lê).
pub(crate) unsafe extern "stdcall" fn compare_national_teams_player_is_eligible(player: usize, manager: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> u32>(IDX_NATIONAL_TEAMS_PLAYER_IS_ELIGIBLE)(player, manager);
    let n = national_teams_player_is_eligible(player, manager);
    SCOREBOARD.record(r == n, || format!("national_teams_player_is_eligible(jogador {:x}, tecnico {:x}) original={} rust={}", player, manager, r, n));
    r
}

/// Comparer for `collect_duty_competitions`.
pub(crate) unsafe extern "thiscall" fn compare_collect_duty_competitions(this: usize, out: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_COLLECT_DUTY_COMPETITIONS);
    let conv = read_usize(this);
    if conv == 0 || out == 0 {
        let r = o(this, out);
        SCOREBOARD.record(r == 0, || format!("collect_duty_competitions(saida {:x}) sem lista original={}", out, r));
        return r;
    }
    let mut has: Vec<usize> = national_teams_list_items(out).iter().map(|&x| read_usize(x)).collect();
    let before = has.len();
    for e in national_teams_list_items(conv) {
        let c = read_usize(e + 4);
        if !has.contains(&c) {
            has.push(c);
        }
    }
    let r = o(this, out);
    let after: Vec<usize> = national_teams_list_items(out).iter().map(|&x| read_usize(x)).collect();
    let same = r == (has.len() - before) as u8 && after == has;
    SCOREBOARD.record(same, || format!("collect_duty_competitions(saida {:x}) original={} plano={:x?} depois={:x?}", out, r, has, after));
    r
}

/// Comparer for `select_squads_for_fixture`.
///
/// The original routine runs first; the copy afterwards, as a dry run, from the same records of the two nations (+0x32
/// to +0x6c: today's marks, squad dates and call-up marks).
///
/// If the copy requested any squad (`select_squad` allocates and touches the squads), the comparison does not count and the
/// original stands; otherwise the records must match.
///
/// [pt] Comparador: a original roda antes; a cópia depois, em ensaio, a partir dos mesmos registros das duas nações (+0x32
/// a +0x6c: marcas de hoje, datas dos elencos e marcas de convocado). Se a cópia pediu algum elenco (0x76c120 aloca e
/// mexe nos elencos), a comparação não vale e fica a original; senão os registros têm de bater.
pub(crate) unsafe extern "thiscall" fn compare_select_squads_for_fixture(this: usize, p: usize) {
    let p = Fixture(p);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SELECT_SQUADS_FOR_FIXTURE);
    let regions = [(club_nation_record(this, Club(p.home_club())) + 0x32, 0x3a), (club_nation_record(this, Club(p.away_club())) + 0x32, 0x3a)];
    let before = crate::comparison::MemorySnapshot::read(&regions);
    o(this, p.addr());
    let mo = crate::comparison::MemorySnapshot::read(&regions);
    before.write_back();
    SELECT_SQUADS_DRY_RUN = true;
    SELECT_SQUADS_REQUESTED_SQUAD = false;
    crate::rng::pause_trace(|| select_squads_for_fixture(this, p.addr()));
    SELECT_SQUADS_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&regions);
    mo.write_back();
    if !SELECT_SQUADS_REQUESTED_SQUAD {
        SCOREBOARD.record(mo == mr, || format!("select_squads_for_fixture({:x}, {:x}) original={:x?} rust={:x?}", this, p.addr(), mo, mr));
    }
}

/// Comparer for `national_teams_post_player_called_up_news`.
///
/// [pt] Comparador: a cópia em ensaio (sem entregar) e a original a partir do mesmo [0xae1080] (o último contrato lido, que
/// as duas gravam).
pub(crate) unsafe extern "stdcall" fn compare_national_teams_post_player_called_up_news(player: usize, nation: usize, slot: u32) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, u32)>(IDX_NATIONAL_TEAMS_POST_PLAYER_CALLED_UP_NEWS);
    let regions = [(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    let before = crate::comparison::MemorySnapshot::read(&regions);
    CALLED_UP_NEWS_DRY_RUN = true;
    crate::rng::pause_trace(|| national_teams_post_player_called_up_news(player, nation, slot));
    CALLED_UP_NEWS_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&regions);
    before.write_back();
    o(player, nation, slot);
    let mo = crate::comparison::MemorySnapshot::read(&regions);
    SCOREBOARD.record(mo == mr, || format!("national_teams_post_player_called_up_news({:x}, {:x}, {}) original={:x?} rust={:x?}", player, nation, slot, mo, mr));
}

/// Comparer for `record_international_result`.
///
/// The rehearsal (the copy without the ranking, the first call-up, the transfer record and the news item, which
/// allocate, go to the file or write elsewhere) and the original routine from the same state, checked over the regions
/// the routine writes on its own (caps_update_regions).
///
/// Outside a match between national teams: only the original.
///
/// [pt] Comparador: o ensaio (a cópia sem o ranking, a primeira convocação, o registro de transferências e a notícia, que
/// alocam, vão para o arquivo ou gravam fora) e a original a partir do mesmo estado, conferidos nas regiões que a
/// rotina grava por conta própria (caps_update_regions). Fora de jogo entre seleções: só a original.
pub(crate) unsafe extern "thiscall" fn compare_record_international_result(this: usize, p: usize, m: usize) {
    let p = Fixture(p);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_RECORD_INTERNATIONAL_RESULT);
    if exe::database_is_national_team(p.home_club()) == 0 || exe::database_is_national_team(p.away_club()) == 0 {
        return o(this, p.addr(), m); // simplification: writes nothing, original only
    }
    let regions = caps_update_regions(this, p, m);
    let before = crate::comparison::MemorySnapshot::read(&regions);
    INTERNATIONAL_RESULT_DRY_RUN = true;
    crate::rng::pause_trace(|| crate::rng::without_counting(|| record_international_result(this, p.addr(), m)));
    INTERNATIONAL_RESULT_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&regions);
    before.write_back();
    o(this, p.addr(), m);
    let mo = crate::comparison::MemorySnapshot::read(&regions);
    SCOREBOARD.record(mo == mr, || format!("record_international_result(partida {:x}, resumo {:x}) original={:x?} rust={:x?}", p.addr(), m, mo, mr));
}

/// Comparer for `format_squad_label`.
///
/// [pt] Comparador: lado a lado no começo da saída (até 64 bytes) e na devolução. A caixa: só a original.
pub(crate) unsafe extern "thiscall" fn compare_format_squad_label(this: usize, kind: u32, sub23: u32, out: usize, size: i32, minusculo: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize, i32, u32) -> u32>(IDX_FORMAT_SQUAD_LABEL);
    if size < 0x14 || out == 0 {
        return o(this, kind, sub23, out, size, minusculo); // simplification: the box, original only
    }
    let reg = [(out, (size as usize).min(0x40))];
    let (ro, rr, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, kind, sub23, out, size, minusculo),
        || format_squad_label(this, kind, sub23, out, size, minusculo));
    SCOREBOARD.record(igual && ro == rr, || format!("format_squad_label({}, {}, {}) original={:x} {:x?} rust={:x} {:x?}", kind as u8, sub23, minusculo as u8, ro, mo, rr, mr));
    ro
}

/// Comparer for `withdraw_injured_player` (comparison mode: see the header of motor.rs): the withdrawal calls the news
/// (allocates), the removal from the tactics and the replacement (original routines with side effects), so only the
/// original routine runs.
///
/// The path is decided beforehand, read-only (release_plan), and checked afterwards: without withdrawal, the national
/// team's club (`+0x24`) and the call-up mark (`+0x28`) remain; with withdrawal, the player who was in one of the
/// team's 50 slots leaves them (`+0x24` zeroed by `0x7762f0`). Without a club (the box): only the original routine.
///
/// [pt] Comparador: a retirada chama a notícia (aloca), a saída da tática e o substituto (originais com efeitos): só a
/// original roda. O caminho é decidido antes, só lendo (release_plan), e conferido depois: sem retirada, o clube da
/// seleção (+0x24) e a marca de convocado (+0x28) ficam; com retirada, o jogador que estava numa das 50 vagas do time
/// sai delas (+0x24 zerado pela 0x7762f0). Sem clube (a caixa): só a original.
pub(crate) unsafe extern "thiscall" fn compare_withdraw_injured_player(this: usize, p: usize, days: u32) {
    let p = Staff(p);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_WITHDRAW_INJURED_PLAYER);
    let club = Club(p.national_job());
    if (days as u16 as i16) < 0xe || club.is_null() {
        return o(this, p.addr(), days);
    }
    let plan = release_plan(this, p, days);
    let team = Club(national_team_for_type(club.nation().addr(), call_up_type(club, p)));
    let estava = !team.is_null() && (0..50usize).any(|c| team.squad(c) == p.addr());
    let before = (club.addr(), p.job_for_nation());
    o(this, p.addr(), days);
    let after = (p.national_job(), p.job_for_nation());
    let ok = match plan {
        0 => after == before,
        _ => !estava || after.0 == 0,
    };
    SCOREBOARD.record(ok, || format!("withdraw_injured_player({:x}, {}) plano {} estava {} antes {:x?} depois {:x?}", p.addr(), days as u16 as i16, plan, estava, before, after));
}

/// Comparer for `national_teams_post_player_withdrawn_news`.
///
/// [pt] Comparador: a notícia aloca e vai para a caixa de entrada: só a original.
pub(crate) unsafe extern "thiscall" fn compare_national_teams_post_player_withdrawn_news(this: usize, p: usize, nation: usize, kind: u32) {
    crate::comparison::branch("national_teams_post_player_withdrawn_news");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_NATIONAL_TEAMS_POST_PLAYER_WITHDRAWN_NEWS)(this, p, nation, kind)
}

/// Comparer for `national_teams_post_replacement_needed_news`.
///
/// [pt] Comparador: a notícia aloca e vai para a caixa de entrada: só a original.
pub(crate) unsafe extern "thiscall" fn compare_national_teams_post_replacement_needed_news(this: usize, p: usize, nation: usize, kind: u32, level: u32) {
    crate::comparison::branch("national_teams_post_replacement_needed_news");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_NATIONAL_TEAMS_POST_REPLACEMENT_NEEDED_NEWS)(this, p, nation, kind, level)
}

/// Comparer for `national_teams_can_change_squad`.
///
/// Side by side on the return value and the reason (the original routine in the destination, the copy in its own
/// destination of the same size). Both move the cursor of list [this] the same way; the search for the day's matches
/// builds and frees a local list (no lasting state).
///
/// No nation or a short destination (the box): only the original.
///
/// [pt] Comparador: lado a lado pela devolução e pelo motivo (a original no destino, a cópia num destino próprio do mesmo
/// tamanho). As duas andam o cursor da lista [this] do mesmo jeito; a busca de jogos do dia monta e libera uma lista
/// local (sem estado que fique). Sem nação ou destino curto (a caixa): só a original.
pub(crate) unsafe extern "thiscall" fn compare_national_teams_can_change_squad(this: usize, nation: usize, kind: u32, p: usize, entrando: u32, msg: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, i32) -> u32>(IDX_NATIONAL_TEAMS_CAN_CHANGE_SQUAD);
    if nation == 0 || (msg != 0 && size < 0x32) {
        return o(this, nation, kind, p, entrando, msg, size);
    }
    let r = o(this, nation, kind, p, entrando, msg, size);
    let mut my: Vec<u8> = if msg != 0 { vec![0u8; size as usize] } else { Vec::new() };
    let n = national_teams_can_change_squad(this, nation, kind, p, entrando, if msg != 0 { my.as_mut_ptr() as usize } else { 0 }, size);
    let text = |a: *const u8, m: usize| -> Vec<u8> { (0..m).map(|i| *a.add(i)).take_while(|&c| c != 0).collect() };
    let (to, tr) = if msg != 0 { (text(msg as *const u8, size as usize), text(my.as_ptr(), size as usize)) } else { (Vec::new(), Vec::new()) };
    SCOREBOARD.record(r == n && to == tr, || format!("national_teams_can_change_squad({:x}, {}, {:x}, {}) {}/{} {:?} x {:?}", nation, kind as u8, p, entrando, r, n,
        String::from_utf8_lossy(&to), String::from_utf8_lossy(&tr)));
    r
}

/// Comparador: pura.
pub(crate) unsafe extern "stdcall" fn compare_national_teams_player_in_squad(p: usize, nation: usize, kind: u32) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, usize, u32) -> u32>(IDX_NATIONAL_TEAMS_PLAYER_IN_SQUAD)(p, nation, kind);
    let n = national_teams_player_in_squad(p, nation, kind);
    SCOREBOARD.record(r == n, || format!("national_teams_player_in_squad({:x}, {:x}, {}) original={} rust={}", p, nation, kind as u8, r, n));
    r
}

/// Comparer for `can_schedule_club_fixture`.
///
/// [pt] Comparador: pelo valor (só lê; as datas são locais).
pub(crate) unsafe extern "thiscall" fn compare_can_schedule_club_fixture(this: usize, game: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_CAN_SCHEDULE_CLUB_FIXTURE)(this, game);
    let n = can_schedule_club_fixture(this, game);
    SCOREBOARD.record(r == n, || format!("can_schedule_club_fixture({:x}) original={} rust={}", game, r, n));
    r
}

/// Comparer for `compare_call_up_records_by_date`.
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn compare_compare_call_up_records_by_date(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_CALL_UP_RECORDS_BY_DATE)(a, b);
    let n = compare_call_up_records_by_date(a, b);
    SCOREBOARD.record(r == n, || format!("compare_call_up_records_by_date({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `add_qualified_nation`.
///
/// [pt] Comparador: a original aloca o nó (não roda duas vezes): só ela, e confere o plano: a conta da lista (+0x28, palavra)
/// sobe 1 com as duas não nulas e fica igual senão.
pub(crate) unsafe extern "thiscall" fn compare_add_qualified_nation(this: usize, nation: usize, stage_final: usize) {
    let before = *((this + national_teams_fields::NEW_QUALIFIERS) as *const i16) as i32;
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_ADD_QUALIFIED_NATION)(this, nation, stage_final);
    let after = *((this + national_teams_fields::NEW_QUALIFIERS) as *const i16) as i32;
    let expected = before + (nation != 0 && stage_final != 0) as i32;
    SCOREBOARD.record(after == expected, || format!("add_qualified_nation({:x}, {:x}) conta {} -> {}", nation, stage_final, before, after));
}

/// Comparer for `set_current_competition`.
///
/// [pt] Comparador: a 0x6a3770 (o aviso) não pode rodar duas vezes: o plano antes, a original roda; confere a competição
/// atual e a da fila. Nação ou competição nula: só a original (caixa).
pub(crate) unsafe extern "thiscall" fn compare_set_current_competition(this: usize, nation: usize, comp: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_SET_CURRENT_COMPETITION);
    if nation == 0 || comp == 0 {
        return o(this, nation, comp);
    }
    let p = current_comp_plan(this, nation, comp);
    let (atual, fila) = (read_usize(p.slot) as u32, read_usize(p.queue) as u32);
    o(this, nation, comp);
    let (atual2, fila2) = (read_usize(p.slot) as u32, read_usize(p.queue) as u32);
    let ok = atual2 == p.new.unwrap_or(atual) && fila2 == p.in_queue.unwrap_or(fila);
    SCOREBOARD.record(ok, || format!("set_current_competition(nação {}, comp {}) atual {:#x} -> {:#x} (esperava {:x?}), fila {:#x} -> {:#x} (esperava {:x?})",
        read_usize(nation), read_usize(comp), atual, atual2, p.new, fila, fila2, p.in_queue));
}

/// Comparer for `mark_clubs_with_fixtures`.
///
/// [pt] Comparador: na primeira (sem tabela: calloc) só a original; depois, lado a lado na tabela (n bytes). A lista do
/// dia e o amistoso só leem (o cursor da lista volta ao começo a cada uso).
pub(crate) unsafe extern "thiscall" fn compare_mark_clubs_with_fixtures(this: usize, data: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_MARK_CLUBS_WITH_FIXTURES);
    let n = (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    if read_usize(this + national_teams_fields::CLUB_MARKS) == 0 || n <= 0 {
        return o(this, data);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(read_usize(this + national_teams_fields::CLUB_MARKS), n as usize)], || o(this, data), || mark_clubs_with_fixtures(this, data));
    SCOREBOARD.record(igual, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let dif: Vec<(usize, u8, u8)> = (0..a.len()).filter(|&i| a[i] != b[i]).take(16).map(|i| (i, a[i], b[i])).collect();
        format!("mark_clubs_with_fixtures({:x}) clube, original, rust: {:?}", core::ptr::read_unaligned(data as *const u32), dif)
    });
}

/// Comparer for `end_duty_for_competition`.
pub(crate) unsafe extern "thiscall" fn compare_end_duty_for_competition(this: usize, comp: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_END_DUTY_FOR_COMPETITION);
    let reg = end_call_ups_regions(this);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, comp), || end_duty_for_competition(this, comp));
    SCOREBOARD.record(igual, || format!("end_duty_for_competition({:x}, {:x}) {:x?} x {:x?}", this, comp, mo, mr));
}

/// Comparer for `end_duty_for_nation_in_competition`.
pub(crate) unsafe extern "thiscall" fn compare_end_duty_for_nation_in_competition(this: usize, nation: usize, comp: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_END_DUTY_FOR_NATION_IN_COMPETITION);
    let reg = end_call_ups_regions(this);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, nation, comp), || end_duty_for_nation_in_competition(this, nation, comp));
    SCOREBOARD.record(igual, || format!("end_duty_for_nation_in_competition({:x}, {:x}, {:x}) {:x?} x {:x?}", this, nation, comp, mo, mr));
}

/// Comparer for `prune_club_call_up_records`.
///
/// [pt] Comparador: os registros (6 bytes cada) e os vetores de entradas. Algum vetor a soltar (0x9452ca; soltar duas
/// vezes): só a original.
pub(crate) unsafe extern "thiscall" fn compare_prune_club_call_up_records(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PRUNE_CLUB_CALL_UP_RECORDS);
    let n = club_call_up_record_count().max(0) as usize;
    let recs = read_usize(this + national_teams_fields::CLUB_CALL_UP_RECORDS);
    if n == 0 || recs == 0 || (0..n).any(|i| would_free_call_up_record(recs + 6 * i)) {
        return o(this);
    }
    let mut reg = vec![(recs, 6 * n)];
    for i in 0..n {
        let (v, k) = (read_usize(recs + 6 * i), *((recs + 6 * i + 5) as *const i8));
        if v != 0 && k > 0 {
            reg.push((v, 9 * k as usize));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || prune_club_call_up_records(this));
    SCOREBOARD.record(igual, || format!("prune_club_call_up_records({:x}) {:x?} x {:x?}", this, mo, mr));
}

/// Comparer for `add_player_to_squad`.
pub(crate) unsafe extern "thiscall" fn compare_add_player_to_squad(this: usize, nation: usize, kind: u32, p: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_ADD_PLAYER_TO_SQUAD);
    let team = national_team_for_type(nation, kind as u8);
    if team == 0 || p == 0 {
        return o(this, nation, kind, p);
    }
    let reg = squad_change_regions(Club(team), Staff(p));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, nation, kind, p), || add_player_to_squad(this, nation, kind, p));
    SCOREBOARD.record(igual, || format!("add_player_to_squad({:x}, {}, {:x}) {:x?} x {:x?}", nation, kind as u8, p, mo, mr));
}

/// Comparer for `national_teams_remove_player_from_squad`.
///
/// [pt] Comparador: as vagas e a pessoa. A 0x896c60 tira a pessoa da tática: na segunda vez (a cópia) já não a acha e não
/// muda nada, então a tática fica fora das regiões. Time ou pessoa nulos (a caixa da 0x896c60): só a original.
pub(crate) unsafe extern "thiscall" fn compare_national_teams_remove_player_from_squad(this: usize, nation: usize, kind: u32, p: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_NATIONAL_TEAMS_REMOVE_PLAYER_FROM_SQUAD);
    let team = national_team_for_type(nation, kind as u8);
    if team == 0 || p == 0 {
        return o(this, nation, kind, p);
    }
    let reg = squad_change_regions(Club(team), Staff(p));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, nation, kind, p), || national_teams_remove_player_from_squad(this, nation, kind, p));
    SCOREBOARD.record(igual, || format!("national_teams_remove_player_from_squad({:x}, {}, {:x}) {:x?} x {:x?}", nation, kind as u8, p, mo, mr));
}

/// Comparer: byte +0x6b of the record, the under-21 slots and their persons (+0x24 to +0x5e).
///
/// The copy's `remove_player_from_selection` no longer finds the persons that the original routine has already removed (it stays outside the
/// regions, as in `national_teams_remove_player_from_squad`).
///
/// Null under-21 team: only the original.
///
/// [pt] Comparador: o byte +0x6b do registro, as vagas do sub-21 e as pessoas delas (+0x24 a +0x5e). A 0x896c60 da
/// cópia não acha mais as pessoas que a original já tirou (fica fora das regiões, como na 0x7762f0). Sub-21 nulo:
/// só a original.
pub(crate) unsafe extern "thiscall" fn compare_release_overage_u21_players(this: usize, nation: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_RELEASE_OVERAGE_U21_PLAYERS);
    let team = Club(exe::database_get_national_u21_team(nation));
    if team.is_null() {
        return o(this, nation);
    }
    let mut reg = vec![(nation_record_by_id(this, nation) + 0x6b, 1), (team.addr() + club::SQUAD, 200)];
    reg.extend((0..50usize).map(|c| team.squad(c)).filter(|&p| p != 0).map(|p| (p + staff::NATIONAL_JOB, 0x3b)));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, nation), || release_overage_u21_players(this, nation));
    SCOREBOARD.record(igual, || format!("release_overage_u21_players({:x}, {:x}) {:x?} x {:x?}", this, nation, mo, mr));
}

/// Comparer for `clear_current_competition`.
///
/// [pt] Comparador: a competição atual e a seguinte; com a seguinte (o aviso 0x6a3770 sorteia e mexe nos técnicos), só a
/// original.
pub(crate) unsafe extern "thiscall" fn compare_clear_current_competition(this: usize, nation: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_CLEAR_CURRENT_COMPETITION);
    let id = *(nation as *const i32);
    let next = read_usize(this + national_teams_fields::NEXT_COMPETITIONS).wrapping_add((id as usize).wrapping_mul(4));
    if *(next as *const i32) != -1 {
        return o(this, nation);
    }
    let reg = [(read_usize(this + national_teams_fields::NATION_RECORDS).wrapping_add(id.wrapping_mul(0x6e) as usize) + 4, 4), (next, 4)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, nation), || clear_current_competition(this, nation));
    SCOREBOARD.record(igual, || format!("clear_current_competition({:x}, {:x}) {:x?} x {:x?}", this, nation, mo, mr));
}

/// Comparer for `release_competitions_of_continent`.
pub(crate) unsafe extern "thiscall" fn compare_release_competitions_of_continent(this: usize, continent: usize, end_of_stage: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_RELEASE_COMPETITIONS_OF_CONTINENT)(this, continent, end_of_stage)
}

/// Comparer for `call_up_replacement`.
pub(crate) unsafe extern "thiscall" fn compare_call_up_replacement(this: usize, nation: usize, kind: u32, level: u32, saindo: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize)>(IDX_CALL_UP_REPLACEMENT)(this, nation, kind, level, saindo)
}

/// Comparer for `build_selection_pools`.
///
/// [pt] Comparador: os registros das nações e, em cada reserva (principal e sub-21), o trecho dos 4 níveis (a soma das
/// vagas +0xe a partir do começo +8).
pub(crate) unsafe extern "thiscall" fn compare_build_selection_pools(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BUILD_SELECTION_POOLS);
    let n = (*(G_NATION_COUNT as *const i32)).max(0) as usize;
    let recs = read_usize(this + national_teams_fields::NATION_RECORDS);
    let mut reg = vec![(recs, 0x6e * n)];
    for i in 0..n {
        let rec = recs + 0x6e * i;
        for sub in 0..2usize {
            let o = 0x15 * sub;
            let total: usize = (0..4).map(|k| read_u8(rec + o + 0xe + 4 * k) as usize).sum();
            if total > 0 {
                reg.push((read_usize(this + national_teams_fields::RESERVE_POOLS + 4 * sub).wrapping_add(4 * read_usize(rec + o + 8)), 4 * total));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || build_selection_pools(this));
    SCOREBOARD.record(igual, || format!("build_selection_pools({:x}) {:x?} x {:x?}", this, mo, mr));
}

/// Comparer for `compare_by_rating`.
pub(crate) unsafe extern "C" fn compare_compare_by_rating(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_BY_RATING)(a, b);
    let n = compare_by_rating(a, b);
    SCOREBOARD.record(r == n, || format!("compare_by_rating({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `compare_by_club`.
pub(crate) unsafe extern "C" fn compare_compare_by_club(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_BY_CLUB)(a, b);
    let n = compare_by_club(a, b);
    SCOREBOARD.record(r == n, || format!("compare_by_club({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `national_teams_remove_player`.
pub(crate) unsafe extern "thiscall" fn compare_national_teams_remove_player(this: usize, player: usize) {
    let player = Staff(player);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_NATIONAL_TEAMS_REMOVE_PLAYER);
    let u = |p: usize| core::ptr::read_unaligned(p as *const usize);
    let nation = u(player.addr() + staff::NATION);
    if nation == 0 || u(player.addr() + staff::PLAYER) == 0 {
        return o(this, player.addr());
    }
    // the regions: the player, the nation record, the level band in the list and the national team (the level,
    // 0x76d810, only reads)
    let reg = u(this + national_teams_fields::NATION_RECORDS).wrapping_add((*(nation as *const u32) as usize).wrapping_mul(110));
    let n = exe::national_teams_get_position_group(player, (reg as *const i16) as usize, (0) as u32) as i8 as isize;
    let mut regions = [(player.addr(), 110), (reg, 110), (player.addr(), 0), (player.addr(), 0)];
    if (0..=3).contains(&n) {
        let start = (0..n as usize).fold(u(reg + 8) as i32, |s, k| s.wrapping_add(*((reg + 0xe + 4 * k) as *const u8) as i32));
        let size = *((reg + national_squad_record::RESERVE_GROUPS + 4 * n as usize) as *const u8) as usize;
        if start >= 0 && size > 0 {
            regions[2] = (u(this + national_teams_fields::RESERVE_POOLS) + 4 * start as usize, 4 * size);
        }
    }
    if u(player.addr() + staff::NATIONAL_JOB) != 0 {
        regions[3] = (u(player.addr() + staff::NATIONAL_JOB) + 0xd7, 200);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, player.addr()), || national_teams_remove_player(this, player.addr()));
    SCOREBOARD.record(igual, || format!("national_teams_remove_player(nivel {}) {:x?} x {:x?}", n, mo, mr));
}

/// Comparer for `national_teams_remove_duplicate_squad_entries`.
pub(crate) unsafe extern "stdcall" fn compare_national_teams_remove_duplicate_squad_entries(nation: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize)>(IDX_NATIONAL_TEAMS_REMOVE_DUPLICATE_SQUAD_ENTRIES);
    if nation == 0 {
        return o(nation);
    }
    let [a, b] = national_squad_slots(nation);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(a, 200), (b, 200)], || o(nation), || national_teams_remove_duplicate_squad_entries(nation));
    SCOREBOARD.record(igual, || format!("national_teams_remove_duplicate_squad_entries {:x?} x {:x?}", mo, mr));
}

/// Comparer for `national_team_level`.
pub(crate) unsafe extern "stdcall" fn compare_national_team_level(j: usize, rep: *const i16, base: u8) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, *const i16, u8) -> u8>(IDX_NATIONAL_TEAM_LEVEL)(j, rep, base);
    let n = national_team_level(j, rep, base);
    SCOREBOARD.record(r == n, || format!("national_team_level({:#x}, {}, {}) original={} rust={}", j, *rep, base, r, n));
    r
}

/// Comparer for `national_teams_daily_update` (comparison mode: see the header of motor.rs): the original routine calls
/// up players (`0x76bab0`: news, squads), reschedules matches (`0x7777e0`), allocates the table (`0x7775c0`) and frees
/// items (`0x68d1f0`), so it cannot run twice.
///
/// Only it runs, checking the plan of what the routine itself writes: no item for today is left in the list at `+0`.
///
/// [pt] Comparador: a original convoca (0x76bab0: notícias, elencos), remarca jogos (0x7777e0), aloca a tabela (0x7775c0)
/// e libera itens (0x68d1f0): não pode rodar duas vezes. Só ela roda, conferindo o plano do que a própria rotina grava:
/// nenhum item do dia de hoje sobra na lista em +0.
pub(crate) unsafe extern "thiscall" fn compare_national_teams_daily_update(this: usize, today: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_NATIONAL_TEAMS_DAILY_UPDATE);
    let day = read_i16(today);
    let before = count_items_on_day(this, day);
    o(this, today);
    let sobra = count_items_on_day(this, day);
    SCOREBOARD.record(sobra == 0, || format!("national_teams_daily_update(dia {}) itens do dia antes {} depois {}", day, before, sobra));
}

/// Comparer for `national_team_ages`.
pub(crate) unsafe extern "stdcall" fn compare_national_team_ages(comp: usize, kind: i8, minimum: *mut u8, maxima: *mut u8) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, i8, *mut u8, *mut u8)>(IDX_NATIONAL_TEAM_AGES);
    let reg = if minimum.is_null() || maxima.is_null() { vec![] } else { vec![(minimum as usize, 1), (maxima as usize, 1)] };
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(comp, kind, minimum, maxima), || national_team_ages(comp, kind, minimum, maxima));
    SCOREBOARD.record(igual, || format!("national_team_ages({:#x}, {}) {:x?} x {:x?}", comp, kind, mo, mr));
}

/// Comparer for `record_club_call_ups`.
///
/// Side by side on the team's squad (50 slots) and, for each club of the squad that counts, on the record (6 bytes) and
/// on the whole entries vector (capacity × 9).
///
/// When some vector may grow (`exe::crt_realloc` releases the old one; it cannot run twice): only the original routine, then
/// checking the squad sorted by club (the same qsort on a copy, done beforehand).
///
/// [pt] Comparador: lado a lado no elenco do time (50 vagas) e, de cada clube do elenco que conta, no registro (6 bytes) e
/// no vetor de entradas inteiro (capacidade × 9). Quando algum vetor pode crescer (0x945667 solta o velho; não roda
/// duas vezes): só a original, conferindo depois o elenco ordenado pelo clube (o mesmo qsort sobre uma cópia, antes).
pub(crate) unsafe extern "thiscall" fn compare_record_club_call_ups(this: usize, team: usize, b: u32, p: usize) {
    let team = Club(team);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_RECORD_CLUB_CALL_UPS);
    if p != 0 && callup_array_full(this, team) {
        let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
        let mut expected: Vec<u32> = (0..0x32usize).map(|i| team.squad(i) as u32).collect();
        // the CRT’s own qsort (the order of equals is its own), outside the original’s trace
        crate::rng::pause_trace(|| qsort(expected.as_mut_ptr() as usize, 0x32, 4, 0x778ef0));
        o(this, team.addr(), b, p);
        let obtido: Vec<u32> = (0..0x32usize).map(|i| team.squad(i) as u32).collect();
        SCOREBOARD.record(obtido == expected, || format!("record_club_call_ups({:x}) elenco original={:x?} esperado={:x?}", team.addr(), obtido, expected));
        return;
    }
    let mut reg = vec![(team.addr() + club::SQUAD, 0xc8)];
    let mut vistos = Vec::new();
    for i in 0..0x32usize {
        let s = Staff(team.squad(i));
        if s.is_null() || read_usize(s.addr()) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
            continue;
        }
        let c = s.club_job();
        if c == 0 || vistos.contains(&c) {
            continue;
        }
        vistos.push(c);
        if let Some(rec) = club_callup_record(this, Club(c)) {
            reg.push((rec, 6));
            let cap = *((rec + 4) as *const i8);
            if read_usize(rec) != 0 && cap > 0 {
                reg.push((read_usize(rec), 9 * cap as usize));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, team.addr(), b, p), || record_club_call_ups(this, team.addr(), b, p));
    SCOREBOARD.record(igual, || format!("record_club_call_ups({:x}, b {}, partida {:x}) {:x?} x {:x?}", team.addr(), b, p, mo, mr));
}

/// Comparer for `set_person_44`.
pub(crate) unsafe extern "thiscall" fn compare_set_person_44(this: usize, person: usize, k: i8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8)>(IDX_SET_PERSON_44);
    let reg = [(person_block(this, person, k) + 0x44, 1)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, person, k), || set_person_44(this, person, k));
    SCOREBOARD.record(igual, || format!("set_person_44 original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `class_by_national_side`.
pub(crate) unsafe extern "stdcall" fn compare_class_by_national_side(person: usize) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u8>(IDX_CLASS_BY_NATIONAL_SIDE)(person);
    let v = class_by_national_side(person);
    SCOREBOARD.record(r == v, || format!("class_by_national_side({:#x}) original={} rust={}", person, r, v));
    r
}

/// Comparer for `national_or_special`.
pub(crate) unsafe extern "stdcall" fn compare_national_or_special(club: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_NATIONAL_OR_SPECIAL)(club);
    let v = national_or_special(club);
    SCOREBOARD.record(r == v, || format!("national_or_special({:#x}) original={} rust={}", club, r, v));
    r
}

/// Comparer for `dislikes_national_side`.
pub(crate) unsafe extern "stdcall" fn compare_dislikes_national_side(person: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_DISLIKES_NATIONAL_SIDE)(person);
    let v = dislikes_national_side(person);
    SCOREBOARD.record(r == v, || format!("dislikes_national_side({:#x}) original={} rust={}", person, r, v));
    r
}

/// Comparer for `copy_date_and_mark`.
pub(crate) unsafe extern "thiscall" fn compare_copy_date_and_mark(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_COPY_DATE_AND_MARK);
    let (r, v, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 9)], || o(this, source), || copy_date_and_mark(this, source));
    SCOREBOARD.record(igual && r == v, || format!("copy_date_and_mark original={:#x} {:x?} rust={:#x} {:x?}", r, mo, v, mr));
    r
}
