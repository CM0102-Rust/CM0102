use super::*;

// The comparers of the routines in ../manager_manager.rs.
// [pt] Os comparadores das rotinas de ../manager_manager.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `update_board_confidence_after_match`.
pub(crate) unsafe extern "thiscall" fn compare_update_board_confidence_after_match(this: usize, club: usize, partida: usize, game: usize, home: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32)>(IDX_UPDATE_BOARD_CONFIDENCE_AFTER_MATCH)(this, club, partida, game, home)
}

/// Comparer for `post_board_match_reaction_news`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_post_board_match_reaction_news(this: usize, club: usize, other: usize, partida: usize, marks: u32, a: i32, b: i32, c: i32, d: i32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, i32, i32, i32, i32)>(IDX_POST_BOARD_MATCH_REACTION_NEWS)(this, club, other, partida, marks, a, b, c, d)
}

/// Comparer for `post_fa_performance_news`.
pub(crate) unsafe extern "thiscall" fn compare_post_fa_performance_news(this: usize, comp: usize, national_team: usize, rating: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_POST_FA_PERFORMANCE_NEWS)(this, comp, national_team, rating)
}

/// Comparer for `set_season_expectations`.
pub(crate) unsafe extern "thiscall" fn compare_set_season_expectations(this: usize, club: usize, news: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_SET_SEASON_EXPECTATIONS)(this, club, news)
}

/// Comparer for `add_job_vacancy`.
pub(crate) unsafe extern "thiscall" fn compare_add_job_vacancy(this: usize, club: usize, left: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_ADD_JOB_VACANCY)(this, club, left)
}

/// Comparer for `post_job_offer_news`.
pub(crate) unsafe extern "thiscall" fn compare_post_job_offer_news(this: usize, person: usize, club: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_POST_JOB_OFFER_NEWS)(this, person, club)
}

/// Comparer for `board_named_players_after_match`.
///
/// Side by side from the same state of the random generator, [0xae1080], the club's board record, the 3 ids and the 5
/// outputs; the return value too (the manager's list, the league table and the match record are only read). The copy
/// runs without counting the random draws.
///
/// [pt] Comparador: lado a lado a partir do mesmo estado do sorteador, do [0xae1080], do registro da diretoria do clube, dos
/// 3 ids e das 5 saídas; a devolução também (a lista do técnico, a tabela da liga e o registro da partida só leem). A
/// cópia sem contar os sorteios.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_board_named_players_after_match(this: usize, game: usize, club: usize, ids: *mut i32, partida: u32, elogio: *const u8,
                                                                  a6: *mut u8, a7: *mut u8, a8: *mut u8, a9: *mut u8, a10: *mut u8) -> u8 {
    type F = unsafe extern "thiscall" fn(usize, usize, usize, *mut i32, u32, *const u8, *mut u8, *mut u8, *mut u8, *mut u8, *mut u8) -> u8;
    let o = original_fn::<F>(IDX_BOARD_NAMED_PLAYERS_AFTER_MATCH);
    let mut regions = crate::rng::STATE_REGIONS.to_vec();
    regions.push((G_CONTRACT_LOOKUP_RESULT, 4));
    if club != 0 {
        regions.push((club_board_row(this, Club(club)), 0x49));
    }
    if !ids.is_null() {
        regions.push((ids as usize, 12));
    }
    for s in [a6, a7, a8, a9, a10] {
        regions.push((s as usize, 1));
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, game, club, ids, partida, elogio, a6, a7, a8, a9, a10),
        || crate::rng::without_counting(|| board_named_players_after_match(this, game, club, ids, partida, elogio, a6, a7, a8, a9, a10)));
    SCOREBOARD.record(igual && r == n, || format!("board_named_players_after_match(clube {:x}, elogio {}) original={} rust={} {:x?} x {:x?}", club, *elogio, r, n, mo, mr));
    r
}

/// Comparer for `get_player_match_rating_verdict`.
///
/// [pt] Comparador: pelo valor (o registro é lido do arquivo temporário e o vetor local se constrói e destrói nos dois).
pub(crate) unsafe extern "thiscall" fn compare_get_player_match_rating_verdict(this: usize, person: usize, partida: u32) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(IDX_GET_PLAYER_MATCH_RATING_VERDICT)(this, person, partida);
    let n = get_player_match_rating_verdict(this, person, partida);
    SCOREBOARD.record(r == n, || format!("get_player_match_rating_verdict({:x}, partida {}) original={} rust={}", person, partida as i32, r as i8, n as i8));
    r
}

/// Comparer for `build_board_player_performance_text`.
///
/// [pt] Comparador: lado a lado nos primeiros 0x200 bytes da saída (o único chamador, 0x6a69c0, passa um texto de 1500).
pub(crate) unsafe extern "stdcall" fn compare_build_board_player_performance_text(ids: *const i32, win: u32, out: usize, variant: u32) {
    let o = original_fn::<unsafe extern "stdcall" fn(*const i32, u32, usize, u32)>(IDX_BUILD_BOARD_PLAYER_PERFORMANCE_TEXT);
    if ids.is_null() || (*ids == -1 && (*ids.add(1) != -1 || *ids.add(2) != -1)) {
        return o(ids, win, out, variant);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(out, 0x200)], || o(ids, win, out, variant),
        || build_board_player_performance_text(ids, win, out, variant));
    SCOREBOARD.record(igual, || format!("build_board_player_performance_text(vitória {}, variante {}) {:?} x {:?}", win as u8, variant,
        String::from_utf8_lossy(&mo.0[0].1[..0x40]), String::from_utf8_lossy(&mr.0[0].1[..0x40])));
}

/// Comparer: side by side from the same state of the club's board record, of the contract stored at [0xae1080] and of
/// the random generator; the return value and all of these must match (the CRT rand, only without the clock seed).
///
/// When buying, the first thing both do is the injury (`exe::injury_get_player_injury`), which first brings the player's injury up to date
/// (`update_player_fitness`: once a day, it may draw at random and writes the date to the record); the original routine, running
/// first, brought it up to date and the copy then skipped a draw (one draw fewer).
///
/// The comparer performs that same update beforehand (the game's own, at the same point: nothing before it draws), and
/// both start from the injury already up to date.
///
/// [pt] Comparador: lado a lado a partir do mesmo estado do registro da diretoria do clube, do contrato guardado em
/// [0xae1080] e do sorteio; a devolução e tudo isso têm de bater (o rand da CRT, só sem a semente do relógio).
/// Comprando, a primeira coisa das duas é a lesão (0x62ed70), que antes põe em dia a lesão do jogador (0x62c680: uma
/// vez por dia, pode sortear e grava a data no registro); a original, rodando primeiro, deixava-a em dia e a cópia não
/// sorteava (um sorteio a menos). O comparador faz antes essa mesma atualização (a do jogo, no mesmo ponto: nada antes
/// dela sorteia), e as duas partem da lesão já em dia.
pub(crate) unsafe extern "thiscall" fn compare_check_board_transfer_veto(this: usize, j: usize, club: usize, value: i32, comprando: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32, u32) -> u32>(IDX_CHECK_BOARD_TRANSFER_VETO);
    if comprando as u8 != 0 && j != 0 {
        // ecx = [0xb63d28]; push person -> thiscall, ret 4 (as in 0x62edfc); checks id and record inside
        exe::injury_update_player_fitness(read_usize(G_INJURY), j);
    }
    let regions = [(club_board_row(this, Club(club)), 0x49), (G_CONTRACT_LOOKUP_RESULT, 4), (0xde1f5c, 8), (0xad6c30, 4)];
    let (r, n, _, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, j, club, value, comprando),
        || crate::rng::without_counting(|| check_board_transfer_veto(this, j, club, value, comprando)));
    let partes = if BOARD_VETO_SEEDED_RAND { 3 } else { 4 };
    let same = (0..partes).all(|i| mo.0[i] == mr.0[i]);
    SCOREBOARD.record(same && r == n, || format!("check_board_transfer_veto(clube {:x}, valor {}, comprando {}) original={:#x} rust={:#x} {:x?} x {:x?}", club, value, comprando as u8, r, n, mo, mr));
    r
}

/// Comparer for `post_board_ultimatum_news`.
pub(crate) unsafe extern "thiscall" fn compare_post_board_ultimatum_news(this: usize, club: usize, opponent: usize, comp: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_POST_BOARD_ULTIMATUM_NEWS)(this, club, opponent, comp)
}

/// Comparer for `news_board_ultimatum_text`.
///
/// [pt] Comparador: o texto vai para o objeto de saída (0x4ae660/0x4ae8a0 acrescentam e alocam): roda só a original.
pub(crate) unsafe extern "stdcall" fn compare_news_board_ultimatum_text(news: usize, body: u32, out: usize) {
    // simplification: the text object cannot receive the text twice; only the original runs here
    original_fn::<unsafe extern "stdcall" fn(usize, u32, usize)>(IDX_NEWS_BOARD_ULTIMATUM_TEXT)(news, body, out);
}

/// Comparer for `post_manager_under_pressure_news`.
pub(crate) unsafe extern "thiscall" fn compare_post_manager_under_pressure_news(this: usize, club: usize, kind: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_POST_MANAGER_UNDER_PRESSURE_NEWS);
    let regions = if club != 0 { vec![(board_row_of(this, Club(club)), 0x49)] } else { Vec::new() };
    compare_manager_news("post_manager_under_pressure_news", regions, || o(this, club, kind), || post_manager_under_pressure_news(this, club, kind));
}

/// Comparer for `post_manager_departure_news`.
pub(crate) unsafe extern "thiscall" fn compare_post_manager_departure_news(this: usize, manager: usize, kind: u32, reason: u32, club: usize, job: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32)>(IDX_POST_MANAGER_DEPARTURE_NEWS);
    compare_manager_news("post_manager_departure_news", Vec::new(), || o(this, manager, kind, reason, club, job), || post_manager_departure_news(this, manager, kind, reason, club, job));
}

/// Comparer for `post_manager_appointment_news`.
pub(crate) unsafe extern "thiscall" fn compare_post_manager_appointment_news(this: usize, manager: usize, previous: usize, club: usize, job: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32)>(IDX_POST_MANAGER_APPOINTMENT_NEWS);
    compare_manager_news("post_manager_appointment_news", Vec::new(), || o(this, manager, previous, club, job), || post_manager_appointment_news(this, manager, previous, club, job));
}

/// Comparer for `get_league_confidence_band`.
pub(crate) unsafe extern "thiscall" fn compare_get_league_confidence_band(this: usize, club: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_GET_LEAGUE_CONFIDENCE_BAND)(this, club);
    let n = get_league_confidence_band(this, club);
    SCOREBOARD.record(r == n, || format!("get_league_confidence_band(clube {:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `set_board_ratings`.
pub(crate) unsafe extern "thiscall" fn compare_set_board_ratings(this: usize, club: usize, value: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_SET_BOARD_RATINGS);
    let r = read_usize(this).wrapping_add((read_i32(club) as u32).wrapping_mul(0x49) as usize);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(r, 0x49)], || o(this, club, value), || set_board_ratings(this, club, value));
    SCOREBOARD.record(igual, || format!("set_board_ratings(clube {:x}, {:x}) {:x?} x {:x?}", club, value, mo, mr));
}

/// Comparer for `post_board_confidence_update_news`.
pub(crate) unsafe extern "thiscall" fn compare_post_board_confidence_update_news(this: usize, club: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_BOARD_CONFIDENCE_UPDATE_NEWS)(this, club)
}

/// Comparer for `post_board_expectations_news`.
pub(crate) unsafe extern "thiscall" fn compare_post_board_expectations_news(this: usize, club: usize, expectations: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_POST_BOARD_EXPECTATIONS_NEWS)(this, club, expectations)
}

/// Comparer for `post_fa_expectations_news`.
pub(crate) unsafe extern "thiscall" fn compare_post_fa_expectations_news(this: usize, national_team: usize, expectativa: u32, competition: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(IDX_POST_FA_EXPECTATIONS_NEWS)(this, national_team, expectativa, competition)
}

/// Comparer for `post_supporter_protest_news`.
pub(crate) unsafe extern "thiscall" fn compare_post_supporter_protest_news(this: usize, club: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_SUPPORTER_PROTEST_NEWS)(this, club)
}

/// Comparer for `init_club_board_records`.
pub(crate) unsafe extern "thiscall" fn compare_init_club_board_records(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_INIT_CLUB_BOARD_RECORDS)(this)
}

/// Comparer for `process_ban_appeals`.
pub(crate) unsafe extern "thiscall" fn compare_process_ban_appeals(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PROCESS_BAN_APPEALS)(this)
}

/// Comparer for `consider_sacking_manager`.
pub(crate) unsafe extern "thiscall" fn compare_consider_sacking_manager(this: usize, club: usize, check: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_CONSIDER_SACKING_MANAGER)(this, club, check)
}

/// Comparer for `issue_board_ultimatum`.
pub(crate) unsafe extern "thiscall" fn compare_issue_board_ultimatum(this: usize, club: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_ISSUE_BOARD_ULTIMATUM);
    if read_usize(this + manager_manager_fields::ULTIMATUM_LIST) == 0 {
        return o(this, club);
    }
    let before = read_i16(read_usize(this + manager_manager_fields::ULTIMATUM_LIST)) as i32;
    let r = o(this, club);
    let after = read_i16(read_usize(this + manager_manager_fields::ULTIMATUM_LIST)) as i32;
    SCOREBOARD.record(after == before + (r != 0) as i32, || {
        format!("issue_board_ultimatum(clube {:x}) devolveu {}, ultimatos {} -> {}", club, r, before, after)
    });
    r
}

/// Comparer for `remove_club_manager`.
pub(crate) unsafe extern "thiscall" fn compare_remove_club_manager(this: usize, club: usize, kind: u32, reason: u32, keep: u32) {
    let club = Club(club);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(IDX_REMOVE_CLUB_MANAGER);
    let tira = manager_and_role(club).is_some();
    o(this, club.addr(), kind, reason, keep);
    if !tira {
        return;
    }
    let mut sobra = 0u32;
    if exe::database_is_national_team(club) == 0 {
        for k in 0..50usize {
            let p = Staff(club.squad(k));
            if p.is_null() || p.player().is_null() {
                continue;
            }
            let ct = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
            if ct != 0 {
                // 0x10000000 is read at +0x40 (0x8676f0) and cleared at +0x45 (0x8674e0): cannot be checked
                sobra |= (exe::staff_contract_get_flags(ct) & 0x180) | (read_usize(ct + 0x45) as u32 & 0x889_2608);
            }
        }
    }
    let conf = read_i16(club_board_row(this, club) + 6);
    SCOREBOARD.record(sobra == 0 && (keep != 0 || conf == 0xdac), || {
        format!("remove_club_manager(clube {:x}, tipo {}, manter {}) marcas que sobraram {:#x}, confiança {}", club.addr(), kind as u8, keep, sobra, conf)
    });
}

/// Comparer for `review_national_team_performance`.
pub(crate) unsafe extern "thiscall" fn compare_review_national_team_performance(this: usize, comp: usize, national_team: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_REVIEW_NATIONAL_TEAM_PERFORMANCE)(this, comp, national_team)
}

/// Comparer for `is_club_in_financial_trouble`.
pub(crate) unsafe extern "thiscall" fn compare_is_club_in_financial_trouble(this: usize, club: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_IS_CLUB_IN_FINANCIAL_TROUBLE)(this, club);
    let n = is_club_in_financial_trouble(this, club);
    SCOREBOARD.record(r == n, || format!("is_club_in_financial_trouble(clube {:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `get_board_confidence_band`.
pub(crate) unsafe extern "thiscall" fn compare_get_board_confidence_band(this: usize, club: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_GET_BOARD_CONFIDENCE_BAND)(this, club);
    let n = get_board_confidence_band(this, club);
    SCOREBOARD.record(r == n, || format!("get_board_confidence_band(clube {:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `reset_board_for_new_manager`.
pub(crate) unsafe extern "thiscall" fn compare_reset_board_for_new_manager(this: usize, club: usize) {
    // simplification: the expectations (0x6a3770) post news and the record (0x869aa0) writes the history to a file: they do not
    // run twice; original only, the copy proves itself by the career in Rust mode (the branch)
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_RESET_BOARD_FOR_NEW_MANAGER)(this, club)
}

/// Comparer for `get_departure_reason`.
///
/// [pt] Comparador: lado a lado a partir do mesmo estado do sorteador (a rotina sorteia em vários caminhos), pela devolução;
/// a cópia sem contar os sorteios.
pub(crate) unsafe extern "thiscall" fn compare_get_departure_reason(this: usize, id: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u8>(IDX_GET_DEPARTURE_REASON);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&crate::rng::STATE_REGIONS, || o(this, id),
        || crate::rng::without_counting(|| get_departure_reason(this, id)));
    SCOREBOARD.record(igual && r == n, || format!("get_departure_reason(clube {}) original={} rust={} {:x?} x {:x?}", id, r, n, mo, mr));
    r
}

/// Comparer for `assess_transfer_deal`.
///
/// [pt] Comparador: a avaliação pede o valor do jogador (0x4d0c60, que sorteia): lado a lado com o estado do sorteador, as
/// duas palavras de marcas e os gastos líquidos do comprador e do vendedor; a cópia sem contar os sorteios.
pub(crate) unsafe extern "thiscall" fn compare_assess_transfer_deal(this: usize, transf: usize, f1: usize, f2: usize) {
    let transf = Offer(transf);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_ASSESS_TRANSFER_DEAL);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(f1, 2), (f2, 2)]);
    let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add(transf.target_staff_id().wrapping_mul(staff::SIZE)));
    let buyer = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(transf.addr() + 8).wrapping_mul(club::SIZE));
    reg.push((board_row_at(this, read_usize(buyer)) + 0x38, 8));
    let seller = person.club_job();
    if !seller.is_null() {
        reg.push((board_row_at(this, read_usize(seller.addr())) + 0x38, 8));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, transf.addr(), f1, f2),
        || crate::rng::without_counting(|| assess_transfer_deal(this, transf.addr(), f1, f2)));
    SCOREBOARD.record(igual, || format!("assess_transfer_deal({:x}) {:x?} x {:x?}", transf.addr(), mo, mr));
}

/// Comparer for `apply_transfer_confidence_changes`.
///
/// [pt] Comparador: lado a lado com o estado do sorteador (a avaliação 0x696570 sorteia), as linhas da diretoria do
/// comprador e do vendedor e as três pessoas favoritas do presidente do vendedor (0x547c10); a cópia sem contar.
pub(crate) unsafe extern "thiscall" fn compare_apply_transfer_confidence_changes(this: usize, transf: usize) {
    let transf = Offer(transf);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_APPLY_TRANSFER_CONFIDENCE_CHANGES);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add(transf.target_staff_id().wrapping_mul(staff::SIZE)));
    reg.push((board_row_at(this, read_usize(transf.addr() + 8)), 0x49));
    let seller = person.club_job();
    if !seller.is_null() {
        reg.push((board_row_at(this, read_usize(seller.addr())), 0x49));
        let pref = staff_preferences(seller.chairman());
        if pref != 0 {
            reg.push((pref + preferences::STAFF_FAVOURITE_STAFF1, 12));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, transf.addr()),
        || crate::rng::without_counting(|| apply_transfer_confidence_changes(this, transf.addr())));
    SCOREBOARD.record(igual, || format!("apply_transfer_confidence_changes({:x}) {:x?} x {:x?}", transf.addr(), mo, mr));
}

/// Comparer for `resolve_board_ultimatum`.
///
/// [pt] Comparador: a original tira o item da lista (e o libera) e acrescenta a saída: roda só ela e confere o plano: a
/// lista das saídas (+4) cresce de 1 quando devolve 1 (e a linha da diretoria fica com o bit 2) e fica igual com 0.
pub(crate) unsafe extern "thiscall" fn compare_resolve_board_ultimatum(this: usize, club: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_RESOLVE_BOARD_ULTIMATUM);
    let outs = read_usize(this + manager_manager_fields::DEPARTURE_LIST);
    if outs == 0 || read_usize(this + manager_manager_fields::ULTIMATUM_LIST) == 0 {
        return o(this, club);
    }
    let before = read_i16(outs) as i32;
    let r = o(this, club);
    let after = read_i16(read_usize(this + manager_manager_fields::DEPARTURE_LIST)) as i32;
    let ok = after == before + (r != 0) as i32 && (r == 0 || read_u8(board_row_at(this, read_usize(club)) + 2) & 2 != 0);
    SCOREBOARD.record(ok, || format!("resolve_board_ultimatum(clube {:x}) devolveu {}, saídas {} -> {}", club, r, before, after));
    r
}

/// Comparer for `process_pending_departures`.
pub(crate) unsafe extern "thiscall" fn compare_process_pending_departures(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PROCESS_PENDING_DEPARTURES)(this)
}

/// Comparer for `manager_manager_remove_player_from_squad`.
///
/// [pt] Comparador: a original tira a pessoa das listas, da mídia e das táticas (não pode rodar duas vezes): roda só ela e
/// confere o plano: a função e o clube (e a seleção) da pessoa e ela fora das 50 vagas do elenco.
pub(crate) unsafe extern "thiscall" fn compare_manager_manager_remove_player_from_squad(this: usize, person: usize, club: usize) -> usize {
    let person = Staff(person);
    let club = Club(club);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(IDX_MANAGER_MANAGER_REMOVE_PLAYER_FROM_SQUAD);
    let in_club = person.club_job() == club;
    let in_national_team = person.national_job() == club;
    let (mut f39, mut c39) = (person.job_for_club(), person.club_job());
    let (mut f28, mut c24) = (person.job_for_nation(), person.national_job());
    if in_club {
        let (f, fica) = role_after_leaving(f39);
        f39 = f;
        if !fica {
            c39 = Club(0);
        }
    }
    if c24 == club {
        let (f, fica) = role_after_leaving(f28);
        f28 = f;
        if !fica {
            c24 = Club(0);
        }
    }
    let r = o(this, person.addr(), club.addr());
    let away = !(in_club || in_national_team) || (0..0x32usize).all(|k| club.squad(k) != person.addr());
    let ok = away && person.job_for_club() == f39 && person.club_job() == c39 && person.job_for_nation() == f28 && person.national_job() == c24;
    SCOREBOARD.record(ok, || format!("manager_manager_remove_player_from_squad(pessoa {:x}, clube {:x}): função {:x}/{:x} clube {:x}/{:x} (esperava {:x}/{:x} {:x}/{:x}), fora das vagas {}",
        read_usize(person.addr()), read_usize(club.addr()), person.job_for_club(), person.job_for_nation(), person.club_job(), person.national_job(), f39, f28, c39, c24, away));
    r
}

/// Comparer for `remove_staff_from_club` (comparison mode: see the header of motor.rs): the original routine removes
/// the contract and removes the person from the media and from the human's lists (it cannot run twice).
///
/// Only it runs, and the plan made beforehand with the body (`detach_staff_from_club`) over copies of the person's
/// fields and the club's slots is checked. A human person: only the original routine (the human's lists may touch the
/// club).
///
/// [pt] Comparador: a original tira o contrato e tira a pessoa da mídia e das listas do humano (não pode rodar duas vezes):
/// roda só ela e confere o plano feito antes com o corpo (`detach_staff_from_club`) sobre cópias dos campos da pessoa e das vagas
/// do clube. Pessoa humana: só a original (as listas do humano podem mexer no clube).
pub(crate) unsafe extern "thiscall" fn compare_remove_staff_from_club(this: usize, person: usize, club: usize) -> usize {
    let person = Staff(person);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(IDX_REMOVE_STAFF_FROM_CLUB);
    if person.is_null() || club == 0 || read_i32(person.addr()) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10) {
        return o(this, person.addr(), club);
    }
    // the plan: the fields in copies with the same offsets
    let mut s = [0u8; 0x40];
    let mut c = [0u8; 0x1e0];
    core::ptr::copy_nonoverlapping(person.addr() as *const u8, s.as_mut_ptr(), 0x40);
    core::ptr::copy_nonoverlapping(club as *const u8, c.as_mut_ptr(), 0x1e0);
    detach_staff_from_club(person.addr(), Staff(s.as_mut_ptr() as usize), club, Club(c.as_mut_ptr() as usize));
    let expected = staff_job_fields(Staff(s.as_ptr() as usize), Club(c.as_ptr() as usize));
    let r = o(this, person.addr(), club);
    let after = staff_job_fields(person, Club(club));
    SCOREBOARD.record(after == expected, || format!("remove_staff_from_club(pessoa {:x}, função {}, clube {:x}): pessoa {:x?}{:x?} esperava {:x?}{:x?}",
        read_usize(person.addr()), person.job_for_club() as i8, read_usize(club), after.0, after.1, expected.0, expected.1));
    r
}

/// Comparer for `fix_missing_jobs`.
///
/// [pt] Comparador: sem vaga a abrir (o plano pelo mesmo teste, com o cursor da lista devolvido), lado a lado pelo cursor
/// da lista; com vaga a abrir (ou a caixa), só a original.
pub(crate) unsafe extern "thiscall" fn compare_fix_missing_jobs(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_FIX_MISSING_JOBS);
    let l = Llist(read_usize(this + manager_manager_fields::VACANCY_LIST));
    let cursor = l.cursor();
    let mut eh_a = 0u32;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut empty = true;
    for _ in 0..(read_usize(G_CLUB_COUNT) as i32).max(0) {
        if club_needs_vacancy(this, club, &mut eh_a) != Some(false) {
            empty = false;
            break;
        }
        club += club::SIZE;
    }
    l.set_cursor(cursor);
    if !empty {
        // simplification: 0x69ba50 opens the slot (allocates, draws the candidate): does not run twice; Rust mode and the career prove
        return o(this);
    }
    let reg = [(l.addr() + llist::CURSOR, 4)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this), || fix_missing_jobs(this));
    SCOREBOARD.record(igual, || format!("fix_missing_jobs (nenhuma a abrir) {:x?} x {:x?}", mo, mr));
}

/// Comparer for `best_manager_candidate_score`.
///
/// [pt] Comparador: lado a lado a partir do mesmo estado do sorteio (a 0x69d1b0 e a 0x69f810 podem sortear), pela nota e
/// pelo cursor da lista de vagas (a 0x69fc60 o devolve).
pub(crate) unsafe extern "thiscall" fn compare_best_manager_candidate_score(this: usize, club: usize, current: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(IDX_BEST_MANAGER_CANDIDATE_SCORE);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((read_usize(this + manager_manager_fields::VACANCY_LIST) + 0xa, 4));
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club, current), || crate::rng::without_counting(|| best_manager_candidate_score(this, club, current)));
    SCOREBOARD.record(igual && r == n, || format!("best_manager_candidate_score(clube {}, atual {:x}) original={} rust={} {:x?} x {:x?}", read_usize(club), current, r, n, mo, mr));
    r
}

/// Comparer for `update_board_after_fixture`.
///
/// [pt] Comparador: a 0x695240 mexe na diretoria (e pode dar notícia): só a original (a prova é a carreira e o modo Rust).
pub(crate) unsafe extern "thiscall" fn compare_update_board_after_fixture(this: usize, partida: usize, game: usize) {
    crate::comparison::branch("update_board_after_fixture");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_UPDATE_BOARD_AFTER_FIXTURE)(this, partida, game)
}

/// Comparer for `is_staff_suitable_for_job`.
///
/// [pt] Comparador: a rotina só lê (o interesse 0x844780 sem a saída das razões): pela devolução.
pub(crate) unsafe extern "thiscall" fn compare_is_staff_suitable_for_job(this: usize, person: usize, club: usize, rating_minimum: f64) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, f64) -> u8>(IDX_IS_STAFF_SUITABLE_FOR_JOB)(this, person, club, rating_minimum);
    let n = is_staff_suitable_for_job(this, person, club, rating_minimum);
    SCOREBOARD.record(r == n, || format!("is_staff_suitable_for_job(pessoa {:x}, clube {:x}, {}) original={} rust={}", person, club, rating_minimum, r, n));
    r
}

/// Comparer for `manager_manager_remove`.
///
/// [pt] Comparador: a 0x68d1f0 libera o nó e o item (não roda duas vezes): o plano antes (o nó achado pela cópia, a cabeça,
/// a cauda, a conta, o cursor e os vizinhos); a original tira; confere a devolução, o cabeçalho (14 bytes) e os elos
/// dos vizinhos.
pub(crate) unsafe extern "thiscall" fn compare_manager_manager_remove(this: usize, given: usize) -> u32 {
    let this = Llist(this);
    let in_the = find_list_node_by_bytes(this, given);
    let before: [u8; 14] = core::ptr::read_unaligned(this.addr() as *const [u8; 14]);
    let (prox, ant) = if in_the != 0 { (read_usize(in_the + llist_node::NEXT), read_usize(in_the + llist_node::PREV)) } else { (0, 0) };
    let (cabeca, cauda, cursor) = (this.head(), this.tail(), this.cursor());
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_MANAGER_MANAGER_REMOVE)(this.addr(), given);
    let after: [u8; 14] = core::ptr::read_unaligned(this.addr() as *const [u8; 14]);
    let ok = if in_the == 0 {
        r == 0 && after == before
    } else {
        let count = core::ptr::read_unaligned(before.as_ptr() as *const i16).wrapping_sub(1);
        r == 1
            && core::ptr::read_unaligned(this.addr() as *const i16) == count
            && this.head() == if in_the == cabeca { prox } else { cabeca }
            && this.tail() == if in_the == cauda { ant } else { cauda }
            && this.cursor() == if in_the == cursor { prox } else { cursor }
            && (in_the == cabeca || read_usize(ant + 6) == prox)
            && (in_the == cauda || read_usize(prox + llist_node::PREV) == ant)
    };
    SCOREBOARD.record(ok, || format!("manager_manager_remove({:#x}, {:#x}) original={} nó {:#x} cabeçalho {:x?} -> {:x?}", this.addr(), given, r, in_the, before, after));
    r
}

/// Comparer for `manager_manager_clear`.
///
/// [pt] Comparador: libera os nós (não roda duas vezes): só a original; confere o cabeçalho zerado (14 bytes).
pub(crate) unsafe extern "thiscall" fn compare_manager_manager_clear(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_MANAGER_MANAGER_CLEAR)(this);
    let after: [u8; 14] = core::ptr::read_unaligned(this as *const [u8; 14]);
    SCOREBOARD.record(after == [0u8; 14], || format!("manager_manager_clear({:#x}) cabeçalho {:x?}", this, after));
}

/// Comparer for `get_current`.
pub(crate) unsafe extern "thiscall" fn compare_get_current(this: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_GET_CURRENT)(this);
    let n = get_current(this);
    SCOREBOARD.record(r == n, || format!("get_current({:#x}) original={:#x} rust={:#x}", this, r, n));
    r
}

/// Comparer for `get_current_run`.
///
/// [pt] Comparador: a original grava nas saídas de verdade, a cópia em locais; a 0x4a4850 pode reordenar as estatísticas
/// (a original primeiro; a cópia acha a tabela já em ordem e a mesma linha).
pub(crate) unsafe extern "thiscall" fn compare_get_current_run(this: usize, club: usize, kind: *mut u8, size: *mut u8) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8)>(IDX_GET_CURRENT_RUN)(this, club, kind, size);
    let (mut t, mut n) = (0u8, 0u8);
    get_current_run(this, club, &mut t, &mut n);
    SCOREBOARD.record(*kind == t && *size == n, || format!("get_current_run({:x}) original={}/{} rust={}/{}", club, *kind, *size, t, n));
}

/// Comparer for `post_job_rumour_news`.
pub(crate) unsafe extern "thiscall" fn compare_post_job_rumour_news(this: usize, person: usize, club: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_POST_JOB_RUMOUR_NEWS)(this, person, club)
}
