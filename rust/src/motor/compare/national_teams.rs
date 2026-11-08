use super::*;

// The comparers of the routines in ../national_teams.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../national_teams.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `send_club_international_report_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x7706c0: a original entrega as notícias (aloca nas caixas): só ela roda, com os sorteios rastreados e o
/// elenco guardado antes. Depois o elenco volta ao de antes, a cópia refaz em ensaio (sorteios da fita, sem entregar) e o
/// elenco tem de sair igual ao da original, com os sorteios do ensaio iguais aos da fita (mesmos n, todos usados).
pub(crate) unsafe extern "thiscall" fn comparar_send_club_international_report_news(this: usize, clube: usize, jogo: usize, resultado: usize, destaque: usize, elenco: *mut usize, conta: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, *mut usize, u32)>(IDX_SEND_CLUB_INTERNATIONAL_REPORT_NEWS);
    let q = (conta as u8 as i8).max(0) as usize;
    let antes: Vec<usize> = (0..q).map(|k| *elenco.add(k)).collect();
    let ((), tape_lock) = crate::aleatorio::rastrear(|| o(this, clube, jogo, resultado, destaque, elenco, conta));
    let depois_o: Vec<usize> = (0..q).map(|k| *elenco.add(k)).collect();
    let n = tape_lock.len();
    for k in 0..q {
        *elenco.add(k) = antes[k];
    }
    *core::ptr::addr_of_mut!(INTERNATIONAL_REPORT_TAPE) = Some((tape_lock, 0, false));
    crate::aleatorio::sem_contar(|| send_club_international_report_news(this, clube, jogo, resultado, destaque, elenco, conta));
    let (k, falhou) = match &*core::ptr::addr_of!(INTERNATIONAL_REPORT_TAPE) {
        Some((_, k, f)) => (*k, *f),
        None => (0, true),
    };
    *core::ptr::addr_of_mut!(INTERNATIONAL_REPORT_TAPE) = None;
    let depois_r: Vec<usize> = (0..q).map(|k| *elenco.add(k)).collect();
    for k in 0..q {
        *elenco.add(k) = depois_o[k];
    }
    PLACAR.registrar(!falhou && k == n && depois_o == depois_r, || format!("send_club_international_report_news(clube {:#x}, jogo {:#x}, conta {}) tape_lock de {} sorteios, ensaio usou {} (n diferente: {}); elenco original {:x?} copia {:x?}",
        clube, jogo, q, n, k, falhou, depois_o, depois_r));
}

/// Comparer for `national_teams_post_call_up_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x76e0b0: ela só monta e entrega notícias (aloca nas caixas dos humanos) e monta e libera a lista dos
/// adversários: não pode rodar duas vezes e não grava nada do jogo além das caixas. Só a original roda (simplification: o
/// efeito é a caixa de entrada do humano; a lista dos adversários é conferida pela 0x76d560).
pub(crate) unsafe extern "thiscall" fn comparar_national_teams_post_call_up_news(this: usize, selecao: usize, segunda: u32, jogo: usize, tipo: u32, sem_adversarios: u32, adversarios: usize, conta: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, usize, u32)>(IDX_NATIONAL_TEAMS_POST_CALL_UP_NEWS)(this, selecao, segunda, jogo, tipo, sem_adversarios, adversarios, conta);
}

/// Comparer for `national_teams_collect_fixture_opponents` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x76d560: a original aloca a lista nova (não pode rodar duas vezes): só ela roda; antes, a cópia em
/// ensaio diz os valores que vão na lista (sem alocar), e a lista da original tem de ter exatamente esses, na ordem.
pub(crate) unsafe extern "thiscall" fn comparar_national_teams_collect_fixture_opponents(this: usize, selecao: usize, jogo: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(IDX_NATIONAL_TEAMS_COLLECT_FIXTURE_OPPONENTS);
    *core::ptr::addr_of_mut!(FIXTURE_OPPONENTS_PLAN) = Some(Vec::new());
    national_teams_collect_fixture_opponents(this, selecao, jogo);
    let plano = (*core::ptr::addr_of_mut!(FIXTURE_OPPONENTS_PLAN)).take().unwrap_or_default();
    let r = o(this, selecao, jogo);
    let agora = list_item_values(r);
    PLACAR.registrar(agora == plano, || format!("national_teams_collect_fixture_opponents(seleção {:#x}, jogo {:#x}) plano {:x?} original {:x?}", selecao, jogo, plano, agora));
    r
}

/// Comparer for `select_squad` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original aloca (0x68d020/0x68d0d0), mexe nos elencos, nas vagas da tática e manda notícias (0x76e0b0,
/// 0x76f1a0): não pode rodar duas vezes. Só ela roda, com uma gravação da fita aberta nas regiões de squad_selection_regions:
/// a escala (0x6e2000) entra nela como unidade com os vetores de entrada (a lista e os códigos) e o estado das
/// regiões logo antes. Depois, com esse estado de volta, a cópia Rust monta a lista (collect_squad_candidates) e os códigos
/// (squad_position_codes), que têm de bater com os da original, junto com a quantidade, o modo e o time; o estado de depois
/// da original volta no fim. Nos elencos de 23 e 25 a original passa a sobra da pilha nas vagas a partir da 22ª: elas
/// ficam fora (correção conhecida, o ramo); as 22 primeiras e o resto conferem. Sem escala (técnico humano), nada a
/// comparar. Dentro de uma gravação de fora, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_select_squad(this: usize, nacao: usize, tipo: u32, data: usize, p: usize, forcado: u32) {
    use std::sync::atomic::Ordering::Relaxed;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, u32)>(IDX_SELECT_SQUAD);
    if is_recording() {
        return o(this, nacao, tipo, data, p, forcado);
    }
    let t = tipo as u8 as i8;
    let regioes = squad_selection_regions(this, nacao, t, p);
    let externa = tape_lock().take();
    *tape_lock() = Some((regioes.clone(), Vec::new()));
    let nivel = RECORDABLE_DEPTH.swap(0, Relaxed);
    o(this, nacao, tipo, data, p, forcado);
    let gravadas = tape_lock().take().map_or(Vec::new(), |f| f.1);
    RECORDABLE_DEPTH.store(nivel, Relaxed);
    *tape_lock() = externa;
    let Some((_, _, a, _, antes, _, (ent, _))) = gravadas.iter().find(|c| c.0 == 0x6e2000) else {
        return;
    };
    let depois = crate::comparacao::Memoria::ler(&regioes);
    let sorteio = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    antes.escrever();
    let reg = squad_nation_record(this, nacao);
    let r = reg.wrapping_add((t as i32 * 0x14) as usize);
    let tam = read_u8(r + 0x42);
    let (elenco, _, _) = squad_team_for_type(nacao, t);
    let modo: u32 = if t == 1 { 0x2100914 } else if t != 2 { 0x2000814 } else { 0x814 };
    let mut cand = [0usize; 200];
    let mut cursor = 0usize;
    let (n, codigos) = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| {
        let n = collect_squad_candidates(this, nacao, t, data, p, elenco, reg, reputation_limits(nacao, p), &mut cand, &mut cursor);
        (n, squad_position_codes(tam))
    }));
    sorteio.escrever();
    depois.escrever();
    let n1 = (a[1] as u8 as i8).max(0) as usize;
    let dw = |k: usize| u32::from_le_bytes([ent[k], ent[k + 1], ent[k + 2], ent[k + 3]]) as usize;
    let lista_o: Vec<usize> = (0..n1.min(ent.len() / 4)).map(|k| dw(4 * k)).collect();
    let cod_o: Vec<u16> = ent.get(4 * n1..).unwrap_or(&[]).chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
    let corrigido = tam == 23 || tam == 25;
    let lim = if corrigido { 22.min(cod_o.len()) } else { cod_o.len() };
    let igual = a[1] as u8 == n && a[2] == modo && a[3] as usize == elenco && cod_o.len() == tam as usize
        && lista_o[..] == cand[..n1.min(200)] && cod_o[..lim] == codigos[..lim];
    PLACAR.registrar(igual, || format!(
        "select_squad(nação {:x}, tipo {}, tam {}) original: n {} modo {:#x} time {:x} lista {:x?} códigos {:x?}; rust: n {} modo {:#x} time {:x} lista {:x?} códigos {:x?}",
        nacao, t, tam, a[1] as u8, a[2], a[3], lista_o, cod_o, n, modo, elenco, &cand[..(n as i8).max(0) as usize], &codigos[..tam as usize]));
}

/// Comparer for `get_international_duty_club` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x7757d0 (2,1 milhões de chamadas até 2002-06-01: o lado a lado leve, sem alocar até 256 bytes): cada
/// uma com a sua lista provisória (como a original); no texto de 0xde1f64 (0x80) e no do comentário (0x40) e na devolução.
pub(crate) unsafe extern "thiscall" fn comparar_get_international_duty_club(this: usize, jogador: usize, data: usize, clube: usize, comentario: usize,
                                                               adversario: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize) -> usize>(IDX_GET_INTERNATIONAL_DUTY_CLUB);
    let regioes = [(0xde1f64usize, 0x80usize), (comentario, 0x40)];
    let reg = if comentario != 0 { &regioes[..] } else { &regioes[..1] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(reg,|| o(this, jogador, data, clube, comentario, adversario),
        || get_international_duty_club(this, jogador, data, clube, comentario, adversario));
    PLACAR.registrar(igual && r == n, || format!("get_international_duty_club(jogador {:x}, data {:x}, clube {:x}, adversario {:x}) original={:x} rust={:x} {:x?} x {:x?}",
        jogador, data, clube, adversario, r, n, mo, mr));
    r
}

/// Comparer for `national_teams_player_is_eligible` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x76d9b0: pela devolução (só lê).
pub(crate) unsafe extern "stdcall" fn comparar_national_teams_player_is_eligible(jogador: usize, tecnico: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> u32>(IDX_NATIONAL_TEAMS_PLAYER_IS_ELIGIBLE)(jogador, tecnico);
    let n = national_teams_player_is_eligible(jogador, tecnico);
    PLACAR.registrar(r == n, || format!("national_teams_player_is_eligible(jogador {:x}, tecnico {:x}) original={} rust={}", jogador, tecnico, r, n));
    r
}

/// Comparer for `collect_duty_competitions` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_collect_duty_competitions(this: usize, saida: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_COLLECT_DUTY_COMPETITIONS);
    let conv = read_usize(this);
    if conv == 0 || saida == 0 {
        let r = o(this, saida);
        PLACAR.registrar(r == 0, || format!("collect_duty_competitions(saida {:x}) sem lista original={}", saida, r));
        return r;
    }
    let mut tem: Vec<usize> = national_teams_list_items(saida).iter().map(|&x| read_usize(x)).collect();
    let antes = tem.len();
    for e in national_teams_list_items(conv) {
        let c = read_usize(e + 4);
        if !tem.contains(&c) {
            tem.push(c);
        }
    }
    let r = o(this, saida);
    let depois: Vec<usize> = national_teams_list_items(saida).iter().map(|&x| read_usize(x)).collect();
    let igual = r == (tem.len() - antes) as u8 && depois == tem;
    PLACAR.registrar(igual, || format!("collect_duty_competitions(saida {:x}) original={} plano={:x?} depois={:x?}", saida, r, tem, depois));
    r
}

/// Comparer for `select_squads_for_fixture` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original roda antes; a cópia depois, em ensaio, a partir dos mesmos registros das duas nações (+0x32
/// a +0x6c: marcas de hoje, datas dos elencos e marcas de convocado). Se a cópia pediu algum elenco (0x76c120 aloca e
/// mexe nos elencos), a comparação não vale e fica a original; senão os registros têm de bater.
pub(crate) unsafe extern "thiscall" fn comparar_select_squads_for_fixture(this: usize, p: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SELECT_SQUADS_FOR_FIXTURE);
    let regioes = [(club_nation_record(this, read_usize(p + 0x1c)) + 0x32, 0x3a), (club_nation_record(this, read_usize(p + 0x20)) + 0x32, 0x3a)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    o(this, p);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    SELECT_SQUADS_DRY_RUN = true;
    SELECT_SQUADS_REQUESTED_SQUAD = false;
    crate::aleatorio::pausar_rastro(|| select_squads_for_fixture(this, p));
    SELECT_SQUADS_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    mo.escrever();
    if !SELECT_SQUADS_REQUESTED_SQUAD {
        PLACAR.registrar(mo == mr, || format!("select_squads_for_fixture({:x}, {:x}) original={:x?} rust={:x?}", this, p, mo, mr));
    }
}

/// Comparer for `national_teams_post_player_called_up_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a cópia em ensaio (sem entregar) e a original a partir do mesmo [0xae1080] (o último contrato lido, que
/// as duas gravam).
pub(crate) unsafe extern "stdcall" fn comparar_national_teams_post_player_called_up_news(jogador: usize, nacao: usize, vaga: u32) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, u32)>(IDX_NATIONAL_TEAMS_POST_PLAYER_CALLED_UP_NEWS);
    let regioes = [(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CALLED_UP_NEWS_DRY_RUN = true;
    crate::aleatorio::pausar_rastro(|| national_teams_post_player_called_up_news(jogador, nacao, vaga));
    CALLED_UP_NEWS_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(jogador, nacao, vaga);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("national_teams_post_player_called_up_news({:x}, {:x}, {}) original={:x?} rust={:x?}", jogador, nacao, vaga, mo, mr));
}

/// Comparer for `record_international_result` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o ensaio (a cópia sem o ranking, a primeira convocação, o registro de transferências e a notícia, que
/// alocam, vão para o arquivo ou gravam fora) e a original a partir do mesmo estado, conferidos nas regiões que a
/// rotina grava por conta própria (caps_update_regions). Fora de jogo entre seleções: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_record_international_result(this: usize, p: usize, m: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_RECORD_INTERNATIONAL_RESULT);
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if is_national_team(read_usize(p + 0x1c)) == 0 || is_national_team(read_usize(p + 0x20)) == 0 {
        return o(this, p, m); // simplification: não grava nada, só a original
    }
    let regioes = caps_update_regions(this, p, m);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    INTERNATIONAL_RESULT_DRY_RUN = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| record_international_result(this, p, m)));
    INTERNATIONAL_RESULT_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(this, p, m);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("record_international_result(partida {:x}, resumo {:x}) original={:x?} rust={:x?}", p, m, mo, mr));
}

/// Comparer for `format_squad_label` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no começo da saída (até 64 bytes) e na devolução. A caixa: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_format_squad_label(this: usize, tipo: u32, sub23: u32, saida: usize, tamanho: i32, minusculo: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize, i32, u32) -> u32>(IDX_FORMAT_SQUAD_LABEL);
    if tamanho < 0x14 || saida == 0 {
        return o(this, tipo, sub23, saida, tamanho, minusculo); // simplification: a caixa, só a original
    }
    let reg = [(saida, (tamanho as usize).min(0x40))];
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, tipo, sub23, saida, tamanho, minusculo),
        || format_squad_label(this, tipo, sub23, saida, tamanho, minusculo));
    PLACAR.registrar(igual && ro == rr, || format!("format_squad_label({}, {}, {}) original={:x} {:x?} rust={:x} {:x?}", tipo as u8, sub23, minusculo as u8, ro, mo, rr, mr));
    ro
}

/// Comparer for `withdraw_injured_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a retirada chama a notícia (aloca), a saída da tática e o substituto (originais com efeitos): só a
/// original roda. O caminho é decidido antes, só lendo (release_plan), e conferido depois: sem retirada, o clube da
/// seleção (+0x24) e a marca de convocado (+0x28) ficam; com retirada, o jogador que estava numa das 50 vagas do time
/// sai delas (+0x24 zerado pela 0x7762f0). Sem clube (a caixa): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_withdraw_injured_player(this: usize, p: usize, dias: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_WITHDRAW_INJURED_PLAYER);
    let clube = read_usize(p + 0x24);
    if (dias as u16 as i16) < 0xe || clube == 0 {
        return o(this, p, dias);
    }
    let plano = release_plan(this, p, dias);
    let time = national_team_for_type(read_usize(clube + club::NATION), call_up_type(clube, p));
    let estava = time != 0 && (0..50usize).any(|c| read_usize(time + 0xd7 + 4 * c) == p);
    let antes = (clube, read_u8(p + 0x28));
    o(this, p, dias);
    let depois = (read_usize(p + 0x24), read_u8(p + 0x28));
    let ok = match plano {
        0 => depois == antes,
        _ => !estava || depois.0 == 0,
    };
    PLACAR.registrar(ok, || format!("withdraw_injured_player({:x}, {}) plano {} estava {} antes {:x?} depois {:x?}", p, dias as u16 as i16, plano, estava, antes, depois));
}

/// Comparer for `national_teams_post_player_withdrawn_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a notícia aloca e vai para a caixa de entrada: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_national_teams_post_player_withdrawn_news(this: usize, p: usize, nacao: usize, tipo: u32) {
    crate::comparacao::ramo("national_teams_post_player_withdrawn_news");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_NATIONAL_TEAMS_POST_PLAYER_WITHDRAWN_NEWS)(this, p, nacao, tipo)
}

/// Comparer for `national_teams_post_replacement_needed_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a notícia aloca e vai para a caixa de entrada: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_national_teams_post_replacement_needed_news(this: usize, p: usize, nacao: usize, tipo: u32, nivel: u32) {
    crate::comparacao::ramo("national_teams_post_replacement_needed_news");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_NATIONAL_TEAMS_POST_REPLACEMENT_NEEDED_NEWS)(this, p, nacao, tipo, nivel)
}

/// Comparer for `national_teams_can_change_squad` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela devolução e pelo motivo (a original no destino, a cópia num destino próprio do mesmo
/// tamanho). As duas andam o cursor da lista [this] do mesmo jeito; a busca de jogos do dia monta e libera uma lista
/// local (sem estado que fique). Sem nação ou destino curto (a caixa): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_national_teams_can_change_squad(this: usize, nacao: usize, tipo: u32, p: usize, entrando: u32, msg: usize, tam: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, i32) -> u32>(IDX_NATIONAL_TEAMS_CAN_CHANGE_SQUAD);
    if nacao == 0 || (msg != 0 && tam < 0x32) {
        return o(this, nacao, tipo, p, entrando, msg, tam);
    }
    let r = o(this, nacao, tipo, p, entrando, msg, tam);
    let mut meu: Vec<u8> = if msg != 0 { vec![0u8; tam as usize] } else { Vec::new() };
    let n = national_teams_can_change_squad(this, nacao, tipo, p, entrando, if msg != 0 { meu.as_mut_ptr() as usize } else { 0 }, tam);
    let texto = |a: *const u8, m: usize| -> Vec<u8> { (0..m).map(|i| *a.add(i)).take_while(|&c| c != 0).collect() };
    let (to, tr) = if msg != 0 { (texto(msg as *const u8, tam as usize), texto(meu.as_ptr(), tam as usize)) } else { (Vec::new(), Vec::new()) };
    PLACAR.registrar(r == n && to == tr, || format!("national_teams_can_change_squad({:x}, {}, {:x}, {}) {}/{} {:?} x {:?}", nacao, tipo as u8, p, entrando, r, n,
        String::from_utf8_lossy(&to), String::from_utf8_lossy(&tr)));
    r
}

/// Comparador: pura.
pub(crate) unsafe extern "stdcall" fn comparar_national_teams_player_in_squad(p: usize, nacao: usize, tipo: u32) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, usize, u32) -> u32>(IDX_NATIONAL_TEAMS_PLAYER_IN_SQUAD)(p, nacao, tipo);
    let n = national_teams_player_in_squad(p, nacao, tipo);
    PLACAR.registrar(r == n, || format!("national_teams_player_in_squad({:x}, {:x}, {}) original={} rust={}", p, nacao, tipo as u8, r, n));
    r
}

/// Comparer for `can_schedule_club_fixture` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê; as datas são locais).
pub(crate) unsafe extern "thiscall" fn comparar_can_schedule_club_fixture(this: usize, jogo: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_CAN_SCHEDULE_CLUB_FIXTURE)(this, jogo);
    let n = can_schedule_club_fixture(this, jogo);
    PLACAR.registrar(r == n, || format!("can_schedule_club_fixture({:x}) original={} rust={}", jogo, r, n));
    r
}

/// Comparer for `compare_call_up_records_by_date` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn comparar_compare_call_up_records_by_date(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_CALL_UP_RECORDS_BY_DATE)(a, b);
    let n = compare_call_up_records_by_date(a, b);
    PLACAR.registrar(r == n, || format!("compare_call_up_records_by_date({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `add_qualified_nation` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original aloca o nó (não roda duas vezes): só ela, e confere o plano: a conta da lista (+0x28, palavra)
/// sobe 1 com as duas não nulas e fica igual senão.
pub(crate) unsafe extern "thiscall" fn comparar_add_qualified_nation(this: usize, nacao: usize, fase_final: usize) {
    let antes = *((this + 0x28) as *const i16) as i32;
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_ADD_QUALIFIED_NATION)(this, nacao, fase_final);
    let depois = *((this + 0x28) as *const i16) as i32;
    let esperado = antes + (nacao != 0 && fase_final != 0) as i32;
    PLACAR.registrar(depois == esperado, || format!("add_qualified_nation({:x}, {:x}) conta {} -> {}", nacao, fase_final, antes, depois));
}

/// Comparer for `set_current_competition` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a 0x6a3770 (o aviso) não pode rodar duas vezes: o plano antes, a original roda; confere a competição
/// atual e a da fila. Nação ou competição nula: só a original (caixa).
pub(crate) unsafe extern "thiscall" fn comparar_set_current_competition(this: usize, nacao: usize, comp: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_SET_CURRENT_COMPETITION);
    if nacao == 0 || comp == 0 {
        return o(this, nacao, comp);
    }
    let p = current_comp_plan(this, nacao, comp);
    let (atual, fila) = (read_u32_as_usize(p.vaga) as u32, read_u32_as_usize(p.fila) as u32);
    o(this, nacao, comp);
    let (atual2, fila2) = (read_u32_as_usize(p.vaga) as u32, read_u32_as_usize(p.fila) as u32);
    let ok = atual2 == p.nova.unwrap_or(atual) && fila2 == p.na_fila.unwrap_or(fila);
    PLACAR.registrar(ok, || format!("set_current_competition(nação {}, comp {}) atual {:#x} -> {:#x} (esperava {:x?}), fila {:#x} -> {:#x} (esperava {:x?})",
        read_u32_as_usize(nacao), read_u32_as_usize(comp), atual, atual2, p.nova, fila, fila2, p.na_fila));
}

/// Comparer for `mark_clubs_with_fixtures` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: na primeira (sem tabela: calloc) só a original; depois, lado a lado na tabela (n bytes). A lista do
/// dia e o amistoso só leem (o cursor da lista volta ao começo a cada uso).
pub(crate) unsafe extern "thiscall" fn comparar_mark_clubs_with_fixtures(this: usize, data: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_MARK_CLUBS_WITH_FIXTURES);
    let n = (read_u32_as_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_u32_as_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    if read_u32_as_usize(this + 0x24) == 0 || n <= 0 {
        return o(this, data);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(read_u32_as_usize(this + 0x24), n as usize)], || o(this, data), || mark_clubs_with_fixtures(this, data));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let dif: Vec<(usize, u8, u8)> = (0..a.len()).filter(|&i| a[i] != b[i]).take(16).map(|i| (i, a[i], b[i])).collect();
        format!("mark_clubs_with_fixtures({:x}) clube, original, rust: {:?}", core::ptr::read_unaligned(data as *const u32), dif)
    });
}

/// Comparer for `end_duty_for_competition` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_end_duty_for_competition(this: usize, comp: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_END_DUTY_FOR_COMPETITION);
    let reg = end_call_ups_regions(this);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, comp), || end_duty_for_competition(this, comp));
    PLACAR.registrar(igual, || format!("end_duty_for_competition({:x}, {:x}) {:x?} x {:x?}", this, comp, mo, mr));
}

/// Comparer for `end_duty_for_nation_in_competition` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_end_duty_for_nation_in_competition(this: usize, nacao: usize, comp: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_END_DUTY_FOR_NATION_IN_COMPETITION);
    let reg = end_call_ups_regions(this);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, nacao, comp), || end_duty_for_nation_in_competition(this, nacao, comp));
    PLACAR.registrar(igual, || format!("end_duty_for_nation_in_competition({:x}, {:x}, {:x}) {:x?} x {:x?}", this, nacao, comp, mo, mr));
}

/// Comparer for `prune_club_call_up_records` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: os registros (6 bytes cada) e os vetores de entradas. Algum vetor a soltar (0x9452ca; soltar duas
/// vezes): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_prune_club_call_up_records(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PRUNE_CLUB_CALL_UP_RECORDS);
    let n = club_call_up_record_count().max(0) as usize;
    let recs = read_usize(this + 0x20);
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
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || prune_club_call_up_records(this));
    PLACAR.registrar(igual, || format!("prune_club_call_up_records({:x}) {:x?} x {:x?}", this, mo, mr));
}

/// Comparer for `add_player_to_squad` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_player_to_squad(this: usize, nacao: usize, tipo: u32, p: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_ADD_PLAYER_TO_SQUAD);
    let time = national_team_for_type(nacao, tipo as u8);
    if time == 0 || p == 0 {
        return o(this, nacao, tipo, p);
    }
    let reg = squad_change_regions(time, p);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, nacao, tipo, p), || add_player_to_squad(this, nacao, tipo, p));
    PLACAR.registrar(igual, || format!("add_player_to_squad({:x}, {}, {:x}) {:x?} x {:x?}", nacao, tipo as u8, p, mo, mr));
}

/// Comparer for `national_teams_remove_player_from_squad` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: as vagas e a pessoa. A 0x896c60 tira a pessoa da tática: na segunda vez (a cópia) já não a acha e não
/// muda nada, então a tática fica fora das regiões. Time ou pessoa nulos (a caixa da 0x896c60): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_national_teams_remove_player_from_squad(this: usize, nacao: usize, tipo: u32, p: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_NATIONAL_TEAMS_REMOVE_PLAYER_FROM_SQUAD);
    let time = national_team_for_type(nacao, tipo as u8);
    if time == 0 || p == 0 {
        return o(this, nacao, tipo, p);
    }
    let reg = squad_change_regions(time, p);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, nacao, tipo, p), || national_teams_remove_player_from_squad(this, nacao, tipo, p));
    PLACAR.registrar(igual, || format!("national_teams_remove_player_from_squad({:x}, {}, {:x}) {:x?} x {:x?}", nacao, tipo as u8, p, mo, mr));
}

/// Comparador: o byte +0x6b do registro, as vagas do sub-21 e as pessoas delas (+0x24 a +0x5e). A 0x896c60 da
/// cópia não acha mais as pessoas que a original já tirou (fica fora das regiões, como na 0x7762f0). Sub-21 nulo:
/// só a original.
pub(crate) unsafe extern "thiscall" fn comparar_release_overage_u21_players(this: usize, nacao: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_RELEASE_OVERAGE_U21_PLAYERS);
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    let time = sub21(nacao);
    if time == 0 {
        return o(this, nacao);
    }
    let mut reg = vec![(nation_record_by_id(this, nacao) + 0x6b, 1), (time + 0xd7, 200)];
    reg.extend((0..50usize).map(|c| read_usize(time + 0xd7 + 4 * c)).filter(|&p| p != 0).map(|p| (p + 0x24, 0x3b)));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, nacao), || release_overage_u21_players(this, nacao));
    PLACAR.registrar(igual, || format!("release_overage_u21_players({:x}, {:x}) {:x?} x {:x?}", this, nacao, mo, mr));
}

/// Comparer for `clear_current_competition` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a competição atual e a seguinte; com a seguinte (o aviso 0x6a3770 sorteia e mexe nos técnicos), só a
/// original.
pub(crate) unsafe extern "thiscall" fn comparar_clear_current_competition(this: usize, nacao: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_CLEAR_CURRENT_COMPETITION);
    let id = *(nacao as *const i32);
    let seguinte = read_usize(this + 8).wrapping_add((id as usize).wrapping_mul(4));
    if *(seguinte as *const i32) != -1 {
        return o(this, nacao);
    }
    let reg = [(read_usize(this + 4).wrapping_add(id.wrapping_mul(0x6e) as usize) + 4, 4), (seguinte, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, nacao), || clear_current_competition(this, nacao));
    PLACAR.registrar(igual, || format!("clear_current_competition({:x}, {:x}) {:x?} x {:x?}", this, nacao, mo, mr));
}

/// Comparer for `release_competitions_of_continent` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_release_competitions_of_continent(this: usize, continente: usize, fim_de_fase: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_RELEASE_COMPETITIONS_OF_CONTINENT)(this, continente, fim_de_fase)
}

/// Comparer for `call_up_replacement` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_call_up_replacement(this: usize, nacao: usize, tipo: u32, nivel: u32, saindo: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize)>(IDX_CALL_UP_REPLACEMENT)(this, nacao, tipo, nivel, saindo)
}

/// Comparer for `build_selection_pools` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: os registros das nações e, em cada reserva (principal e sub-21), o trecho dos 4 níveis (a soma das
/// vagas +0xe a partir do começo +8).
pub(crate) unsafe extern "thiscall" fn comparar_build_selection_pools(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BUILD_SELECTION_POOLS);
    let n = (*(G_NATION_COUNT as *const i32)).max(0) as usize;
    let recs = read_usize(this + 4);
    let mut reg = vec![(recs, 0x6e * n)];
    for i in 0..n {
        let rec = recs + 0x6e * i;
        for sub in 0..2usize {
            let o = 0x15 * sub;
            let total: usize = (0..4).map(|k| read_u8(rec + o + 0xe + 4 * k) as usize).sum();
            if total > 0 {
                reg.push((read_usize(this + 0xc + 4 * sub).wrapping_add(4 * read_usize(rec + o + 8)), 4 * total));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || build_selection_pools(this));
    PLACAR.registrar(igual, || format!("build_selection_pools({:x}) {:x?} x {:x?}", this, mo, mr));
}

/// Comparer for `compare_by_rating` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_compare_by_rating(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_BY_RATING)(a, b);
    let n = compare_by_rating(a, b);
    PLACAR.registrar(r == n, || format!("compare_by_rating({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `compare_by_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_compare_by_club(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_BY_CLUB)(a, b);
    let n = compare_by_club(a, b);
    PLACAR.registrar(r == n, || format!("compare_by_club({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `national_teams_remove_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_national_teams_remove_player(this: usize, jogador: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_NATIONAL_TEAMS_REMOVE_PLAYER);
    let u = |p: usize| core::ptr::read_unaligned(p as *const usize);
    let nacao = u(jogador + 0x1a);
    if nacao == 0 || u(jogador + 0x61) == 0 {
        return o(this, jogador);
    }
    // as regiões: o jogador, o registro da nação, a faixa do nível na lista e a seleção (o nível,
    // 0x76d810, só lê)
    let nivel: unsafe extern "stdcall" fn(usize, *const i16, u8) -> u8 = core::mem::transmute(FN_NATIONAL_TEAMS_GET_POSITION_GROUP);
    let reg = u(this + 4).wrapping_add((*(nacao as *const u32) as usize).wrapping_mul(110));
    let n = nivel(jogador, reg as *const i16, 0) as i8 as isize;
    let mut regioes = [(jogador, 110), (reg, 110), (jogador, 0), (jogador, 0)];
    if (0..=3).contains(&n) {
        let inicio = (0..n as usize).fold(u(reg + 8) as i32, |s, k| s.wrapping_add(*((reg + 0xe + 4 * k) as *const u8) as i32));
        let tamanho = *((reg + 0xd + 4 * n as usize) as *const u8) as usize;
        if inicio >= 0 && tamanho > 0 {
            regioes[2] = (u(this + 0xc) + 4 * inicio as usize, 4 * tamanho);
        }
    }
    if u(jogador + 0x24) != 0 {
        regioes[3] = (u(jogador + 0x24) + 0xd7, 200);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, jogador), || national_teams_remove_player(this, jogador));
    PLACAR.registrar(igual, || format!("national_teams_remove_player(nivel {}) {:x?} x {:x?}", n, mo, mr));
}

/// Comparer for `national_teams_remove_duplicate_squad_entries` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_national_teams_remove_duplicate_squad_entries(nacao: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize)>(IDX_NATIONAL_TEAMS_REMOVE_DUPLICATE_SQUAD_ENTRIES);
    if nacao == 0 {
        return o(nacao);
    }
    let [a, b] = national_squad_slots(nacao);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(a, 200), (b, 200)], || o(nacao), || national_teams_remove_duplicate_squad_entries(nacao));
    PLACAR.registrar(igual, || format!("national_teams_remove_duplicate_squad_entries {:x?} x {:x?}", mo, mr));
}

/// Comparer for `national_team_level` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_national_team_level(j: usize, rep: *const i16, base: u8) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, *const i16, u8) -> u8>(IDX_NATIONAL_TEAM_LEVEL)(j, rep, base);
    let n = national_team_level(j, rep, base);
    PLACAR.registrar(r == n, || format!("national_team_level({:#x}, {}, {}) original={} rust={}", j, *rep, base, r, n));
    r
}

/// Comparer for `national_teams_daily_update` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original convoca (0x76bab0: notícias, elencos), remarca jogos (0x7777e0), aloca a tabela (0x7775c0)
/// e libera itens (0x68d1f0): não pode rodar duas vezes. Só ela roda, conferindo o plano do que a própria rotina grava:
/// nenhum item do dia de hoje sobra na lista em +0.
pub(crate) unsafe extern "thiscall" fn comparar_national_teams_daily_update(this: usize, hoje: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_NATIONAL_TEAMS_DAILY_UPDATE);
    let dia = read_i16(hoje);
    let antes = count_items_on_day(this, dia);
    o(this, hoje);
    let sobra = count_items_on_day(this, dia);
    PLACAR.registrar(sobra == 0, || format!("national_teams_daily_update(dia {}) itens do dia antes {} depois {}", dia, antes, sobra));
}

/// Comparer for `national_team_ages` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_national_team_ages(comp: usize, tipo: i8, minima: *mut u8, maxima: *mut u8) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, i8, *mut u8, *mut u8)>(IDX_NATIONAL_TEAM_AGES);
    let reg = if minima.is_null() || maxima.is_null() { vec![] } else { vec![(minima as usize, 1), (maxima as usize, 1)] };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(comp, tipo, minima, maxima), || national_team_ages(comp, tipo, minima, maxima));
    PLACAR.registrar(igual, || format!("national_team_ages({:#x}, {}) {:x?} x {:x?}", comp, tipo, mo, mr));
}

/// Comparer for `record_club_call_ups` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no elenco do time (50 vagas) e, de cada clube do elenco que conta, no registro (6 bytes) e
/// no vetor de entradas inteiro (capacidade × 9). Quando algum vetor pode crescer (0x945667 solta o velho; não roda
/// duas vezes): só a original, conferindo depois o elenco ordenado pelo clube (o mesmo qsort sobre uma cópia, antes).
pub(crate) unsafe extern "thiscall" fn comparar_record_club_call_ups(this: usize, time: usize, b: u32, p: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_RECORD_CLUB_CALL_UPS);
    if p != 0 && callup_array_full(this, time) {
        let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
        let mut esperado: Vec<u32> = (0..0x32usize).map(|i| read_usize(time + 0xd7 + 4 * i) as u32).collect();
        // o mesmo qsort da CRT (a ordem dos iguais é a dele), fora do rastro da original
        crate::aleatorio::pausar_rastro(|| qsort(esperado.as_mut_ptr() as usize, 0x32, 4, 0x778ef0));
        o(this, time, b, p);
        let obtido: Vec<u32> = (0..0x32usize).map(|i| read_usize(time + 0xd7 + 4 * i) as u32).collect();
        PLACAR.registrar(obtido == esperado, || format!("record_club_call_ups({:x}) elenco original={:x?} esperado={:x?}", time, obtido, esperado));
        return;
    }
    let mut reg = vec![(time + 0xd7, 0xc8)];
    let mut vistos = Vec::new();
    for i in 0..0x32usize {
        let s = read_usize(time + 0xd7 + 4 * i);
        if s == 0 || read_usize(s) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
            continue;
        }
        let c = read_usize(s + 0x39);
        if c == 0 || vistos.contains(&c) {
            continue;
        }
        vistos.push(c);
        if let Some(rec) = club_callup_record(this, c) {
            reg.push((rec, 6));
            let cap = *((rec + 4) as *const i8);
            if read_usize(rec) != 0 && cap > 0 {
                reg.push((read_usize(rec), 9 * cap as usize));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, time, b, p), || record_club_call_ups(this, time, b, p));
    PLACAR.registrar(igual, || format!("record_club_call_ups({:x}, b {}, partida {:x}) {:x?} x {:x?}", time, b, p, mo, mr));
}

/// Comparer for `set_person_44` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_person_44(this: usize, pessoa: usize, k: i8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8)>(IDX_SET_PERSON_44);
    let reg = [(person_block(this, pessoa, k) + 0x44, 1)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, k), || set_person_44(this, pessoa, k));
    PLACAR.registrar(igual, || format!("set_person_44 original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `class_by_national_side` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_class_by_national_side(pessoa: usize) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u8>(IDX_CLASS_BY_NATIONAL_SIDE)(pessoa);
    let v = class_by_national_side(pessoa);
    PLACAR.registrar(r == v, || format!("class_by_national_side({:#x}) original={} rust={}", pessoa, r, v));
    r
}

/// Comparer for `national_or_special` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_national_or_special(clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_NATIONAL_OR_SPECIAL)(clube);
    let v = national_or_special(clube);
    PLACAR.registrar(r == v, || format!("national_or_special({:#x}) original={} rust={}", clube, r, v));
    r
}

/// Comparer for `dislikes_national_side` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_dislikes_national_side(pessoa: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_DISLIKES_NATIONAL_SIDE)(pessoa);
    let v = dislikes_national_side(pessoa);
    PLACAR.registrar(r == v, || format!("dislikes_national_side({:#x}) original={} rust={}", pessoa, r, v));
    r
}

/// Comparer for `copy_date_and_mark` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_copy_date_and_mark(this: usize, origem: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_COPY_DATE_AND_MARK);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 9)], || o(this, origem), || copy_date_and_mark(this, origem));
    PLACAR.registrar(igual && r == v, || format!("copy_date_and_mark original={:#x} {:x?} rust={:#x} {:x?}", r, mo, v, mr));
    r
}
