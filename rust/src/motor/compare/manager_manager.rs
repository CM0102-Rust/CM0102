use super::*;

// The comparers of the routines in ../manager_manager.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../manager_manager.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `update_board_confidence_after_match` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_update_board_confidence_after_match(this: usize, clube: usize, partida: usize, jogo: usize, home: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32)>(IDX_UPDATE_BOARD_CONFIDENCE_AFTER_MATCH)(this, clube, partida, jogo, home)
}

/// Comparer for `post_board_match_reaction_news` (comparison mode: see the header of motor.rs).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_post_board_match_reaction_news(this: usize, clube: usize, other: usize, partida: usize, marks: u32, a: i32, b: i32, c: i32, d: i32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, i32, i32, i32, i32)>(IDX_POST_BOARD_MATCH_REACTION_NEWS)(this, clube, other, partida, marks, a, b, c, d)
}

/// Comparer for `post_fa_performance_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_fa_performance_news(this: usize, comp: usize, national_team: usize, rating: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_POST_FA_PERFORMANCE_NEWS)(this, comp, national_team, rating)
}

/// Comparer for `set_season_expectations` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_season_expectations(this: usize, clube: usize, noticia: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_SET_SEASON_EXPECTATIONS)(this, clube, noticia)
}

/// Comparer for `add_job_vacancy` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_job_vacancy(this: usize, clube: usize, saiu: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_ADD_JOB_VACANCY)(this, clube, saiu)
}

/// Comparer for `post_job_offer_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_job_offer_news(this: usize, person: usize, clube: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_POST_JOB_OFFER_NEWS)(this, person, clube)
}

/// Comparer for `board_named_players_after_match` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado a partir do mesmo estado do sorteador, do [0xae1080], do registro da diretoria do clube, dos
/// 3 ids e das 5 saídas; a devolução também (a lista do técnico, a tabela da liga e o registro da partida só leem). A
/// cópia sem contar os sorteios.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_board_named_players_after_match(this: usize, jogo: usize, clube: usize, ids: *mut i32, partida: u32, elogio: *const u8,
                                                                  a6: *mut u8, a7: *mut u8, a8: *mut u8, a9: *mut u8, a10: *mut u8) -> u8 {
    type F = unsafe extern "thiscall" fn(usize, usize, usize, *mut i32, u32, *const u8, *mut u8, *mut u8, *mut u8, *mut u8, *mut u8) -> u8;
    let o = original_fn::<F>(IDX_BOARD_NAMED_PLAYERS_AFTER_MATCH);
    let mut regions = crate::rng::STATE_REGIONS.to_vec();
    regions.push((G_CONTRACT_LOOKUP_RESULT, 4));
    if clube != 0 {
        regions.push((club_board_row(this, clube), 0x49));
    }
    if !ids.is_null() {
        regions.push((ids as usize, 12));
    }
    for s in [a6, a7, a8, a9, a10] {
        regions.push((s as usize, 1));
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, jogo, clube, ids, partida, elogio, a6, a7, a8, a9, a10),
        || crate::rng::without_counting(|| board_named_players_after_match(this, jogo, clube, ids, partida, elogio, a6, a7, a8, a9, a10)));
    PLACAR.record(igual && r == n, || format!("board_named_players_after_match(clube {:x}, elogio {}) original={} rust={} {:x?} x {:x?}", clube, *elogio, r, n, mo, mr));
    r
}

/// Comparer for `get_player_match_rating_verdict` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (o registro é lido do arquivo temporário e o vetor local se constrói e destrói nos dois).
pub(crate) unsafe extern "thiscall" fn comparar_get_player_match_rating_verdict(this: usize, person: usize, partida: u32) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(IDX_GET_PLAYER_MATCH_RATING_VERDICT)(this, person, partida);
    let n = get_player_match_rating_verdict(this, person, partida);
    PLACAR.record(r == n, || format!("get_player_match_rating_verdict({:x}, partida {}) original={} rust={}", person, partida as i32, r as i8, n as i8));
    r
}

/// Comparer for `build_board_player_performance_text` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado nos primeiros 0x200 bytes da saída (o único chamador, 0x6a69c0, passa um texto de 1500).
pub(crate) unsafe extern "stdcall" fn comparar_build_board_player_performance_text(ids: *const i32, win: u32, out: usize, variant: u32) {
    let o = original_fn::<unsafe extern "stdcall" fn(*const i32, u32, usize, u32)>(IDX_BUILD_BOARD_PLAYER_PERFORMANCE_TEXT);
    if ids.is_null() || (*ids == -1 && (*ids.add(1) != -1 || *ids.add(2) != -1)) {
        return o(ids, win, out, variant);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(out, 0x200)], || o(ids, win, out, variant),
        || build_board_player_performance_text(ids, win, out, variant));
    PLACAR.record(igual, || format!("build_board_player_performance_text(vitória {}, variante {}) {:?} x {:?}", win as u8, variant,
        String::from_utf8_lossy(&mo.0[0].1[..0x40]), String::from_utf8_lossy(&mr.0[0].1[..0x40])));
}

/// Comparador: lado a lado a partir do mesmo estado do registro da diretoria do clube, do contrato guardado em
/// [0xae1080] e do sorteio; a devolução e tudo isso têm de bater (o rand da CRT, só sem a semente do relógio).
/// Comprando, a primeira coisa das duas é a lesão (0x62ed70), que antes põe em dia a lesão do jogador (0x62c680: uma
/// vez por dia, pode sortear e grava a data no registro); a original, rodando primeiro, deixava-a em dia e a cópia não
/// sorteava (um sorteio a menos). O comparador faz antes essa mesma atualização (a do jogo, no mesmo ponto: nada antes
/// dela sorteia), e as duas partem da lesão já em dia.
pub(crate) unsafe extern "thiscall" fn comparar_check_board_transfer_veto(this: usize, j: usize, clube: usize, value: i32, comprando: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32, u32) -> u32>(IDX_CHECK_BOARD_TRANSFER_VETO);
    if comprando as u8 != 0 && j != 0 {
        // ecx = [0xb63d28]; push person -> thiscall, ret 4 (as in 0x62edfc); checks id and record inside
        let in_day: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_INJURY_UPDATE_PLAYER_FITNESS);
        in_day(read_usize(G_INJURY), j);
    }
    let regions = [(club_board_row(this, clube), 0x49), (G_CONTRACT_LOOKUP_RESULT, 4), (0xde1f5c, 8), (0xad6c30, 4)];
    let (r, n, _, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, j, clube, value, comprando),
        || crate::rng::without_counting(|| check_board_transfer_veto(this, j, clube, value, comprando)));
    let partes = if BOARD_VETO_SEEDED_RAND { 3 } else { 4 };
    let same = (0..partes).all(|i| mo.0[i] == mr.0[i]);
    PLACAR.record(same && r == n, || format!("check_board_transfer_veto(clube {:x}, valor {}, comprando {}) original={:#x} rust={:#x} {:x?} x {:x?}", clube, value, comprando as u8, r, n, mo, mr));
    r
}

/// Comparer for `post_board_ultimatum_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_board_ultimatum_news(this: usize, clube: usize, opponent: usize, comp: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_POST_BOARD_ULTIMATUM_NEWS)(this, clube, opponent, comp)
}

/// Comparer for `news_board_ultimatum_text` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o texto vai para o objeto de saída (0x4ae660/0x4ae8a0 acrescentam e alocam): roda só a original.
pub(crate) unsafe extern "stdcall" fn comparar_news_board_ultimatum_text(noticia: usize, corpo: u32, out: usize) {
    // simplification: the text object cannot receive the text twice; only the original runs here
    original_fn::<unsafe extern "stdcall" fn(usize, u32, usize)>(IDX_NEWS_BOARD_ULTIMATUM_TEXT)(noticia, corpo, out);
}

/// Comparer for `post_manager_under_pressure_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_manager_under_pressure_news(this: usize, clube: usize, kind: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_POST_MANAGER_UNDER_PRESSURE_NEWS);
    let regions = if clube != 0 { vec![(board_row_of(this, clube), 0x49)] } else { Vec::new() };
    compare_manager_news("post_manager_under_pressure_news", regions, || o(this, clube, kind), || post_manager_under_pressure_news(this, clube, kind));
}

/// Comparer for `post_manager_departure_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_manager_departure_news(this: usize, manager: usize, kind: u32, reason: u32, clube: usize, job: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32)>(IDX_POST_MANAGER_DEPARTURE_NEWS);
    compare_manager_news("post_manager_departure_news", Vec::new(), || o(this, manager, kind, reason, clube, job), || post_manager_departure_news(this, manager, kind, reason, clube, job));
}

/// Comparer for `post_manager_appointment_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_manager_appointment_news(this: usize, manager: usize, previous: usize, clube: usize, job: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32)>(IDX_POST_MANAGER_APPOINTMENT_NEWS);
    compare_manager_news("post_manager_appointment_news", Vec::new(), || o(this, manager, previous, clube, job), || post_manager_appointment_news(this, manager, previous, clube, job));
}

/// Comparer for `get_league_confidence_band` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_league_confidence_band(this: usize, clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_GET_LEAGUE_CONFIDENCE_BAND)(this, clube);
    let n = get_league_confidence_band(this, clube);
    PLACAR.record(r == n, || format!("get_league_confidence_band(clube {:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `set_board_ratings` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_board_ratings(this: usize, clube: usize, value: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_SET_BOARD_RATINGS);
    let r = read_usize(this).wrapping_add((read_i32(clube) as u32).wrapping_mul(0x49) as usize);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(r, 0x49)], || o(this, clube, value), || set_board_ratings(this, clube, value));
    PLACAR.record(igual, || format!("set_board_ratings(clube {:x}, {:x}) {:x?} x {:x?}", clube, value, mo, mr));
}

/// Comparer for `post_board_confidence_update_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_board_confidence_update_news(this: usize, clube: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_BOARD_CONFIDENCE_UPDATE_NEWS)(this, clube)
}

/// Comparer for `post_board_expectations_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_board_expectations_news(this: usize, clube: usize, expectativas: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_POST_BOARD_EXPECTATIONS_NEWS)(this, clube, expectativas)
}

/// Comparer for `post_fa_expectations_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_fa_expectations_news(this: usize, national_team: usize, expectativa: u32, competicao: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(IDX_POST_FA_EXPECTATIONS_NEWS)(this, national_team, expectativa, competicao)
}

/// Comparer for `post_supporter_protest_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_supporter_protest_news(this: usize, clube: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_SUPPORTER_PROTEST_NEWS)(this, clube)
}

/// Comparer for `init_club_board_records` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_init_club_board_records(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_INIT_CLUB_BOARD_RECORDS)(this)
}

/// Comparer for `process_ban_appeals` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_process_ban_appeals(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PROCESS_BAN_APPEALS)(this)
}

/// Comparer for `consider_sacking_manager` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_consider_sacking_manager(this: usize, clube: usize, check: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_CONSIDER_SACKING_MANAGER)(this, clube, check)
}

/// Comparer for `issue_board_ultimatum` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_issue_board_ultimatum(this: usize, clube: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_ISSUE_BOARD_ULTIMATUM);
    if read_usize(this + 0x10) == 0 {
        return o(this, clube);
    }
    let before = read_i16(read_usize(this + 0x10)) as i32;
    let r = o(this, clube);
    let after = read_i16(read_usize(this + 0x10)) as i32;
    PLACAR.record(after == before + (r != 0) as i32, || {
        format!("issue_board_ultimatum(clube {:x}) devolveu {}, ultimatos {} -> {}", clube, r, before, after)
    });
    r
}

/// Comparer for `remove_club_manager` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_remove_club_manager(this: usize, clube: usize, kind: u32, reason: u32, keep: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(IDX_REMOVE_CLUB_MANAGER);
    let tira = manager_and_role(clube).is_some();
    o(this, clube, kind, reason, keep);
    if !tira {
        return;
    }
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let marks: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_GET_FLAGS);
    let mut sobra = 0u32;
    if national_team(clube) == 0 {
        for k in 0..50usize {
            let p = read_usize(clube + club::SQUAD + 4 * k);
            if p == 0 || read_usize(p + staff::PLAYER) == 0 {
                continue;
            }
            let ct = contrato(read_usize(G_CONTRACT_MANAGER), p);
            if ct != 0 {
                // 0x10000000 is read at +0x40 (0x8676f0) and cleared at +0x45 (0x8674e0): cannot be checked
                sobra |= (marks(ct) & 0x180) | (read_usize(ct + 0x45) as u32 & 0x889_2608);
            }
        }
    }
    let conf = read_i16(club_board_row(this, clube) + 6);
    PLACAR.record(sobra == 0 && (keep != 0 || conf == 0xdac), || {
        format!("remove_club_manager(clube {:x}, tipo {}, manter {}) marcas que sobraram {:#x}, confiança {}", clube, kind as u8, keep, sobra, conf)
    });
}

/// Comparer for `review_national_team_performance` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_review_national_team_performance(this: usize, comp: usize, national_team: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_REVIEW_NATIONAL_TEAM_PERFORMANCE)(this, comp, national_team)
}

/// Comparer for `is_club_in_financial_trouble` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_is_club_in_financial_trouble(this: usize, clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_IS_CLUB_IN_FINANCIAL_TROUBLE)(this, clube);
    let n = is_club_in_financial_trouble(this, clube);
    PLACAR.record(r == n, || format!("is_club_in_financial_trouble(clube {:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `get_board_confidence_band` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_board_confidence_band(this: usize, clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_GET_BOARD_CONFIDENCE_BAND)(this, clube);
    let n = get_board_confidence_band(this, clube);
    PLACAR.record(r == n, || format!("get_board_confidence_band(clube {:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `reset_board_for_new_manager` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_reset_board_for_new_manager(this: usize, clube: usize) {
    // simplification: the expectations (0x6a3770) post news and the record (0x869aa0) writes the history to a file: they do not
    // run twice; original only, the copy proves itself by the career in Rust mode (the branch)
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_RESET_BOARD_FOR_NEW_MANAGER)(this, clube)
}

/// Comparer for `get_departure_reason` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado a partir do mesmo estado do sorteador (a rotina sorteia em vários caminhos), pela devolução;
/// a cópia sem contar os sorteios.
pub(crate) unsafe extern "thiscall" fn comparar_get_departure_reason(this: usize, id: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u8>(IDX_GET_DEPARTURE_REASON);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&crate::rng::STATE_REGIONS, || o(this, id),
        || crate::rng::without_counting(|| get_departure_reason(this, id)));
    PLACAR.record(igual && r == n, || format!("get_departure_reason(clube {}) original={} rust={} {:x?} x {:x?}", id, r, n, mo, mr));
    r
}

/// Comparer for `assess_transfer_deal` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a avaliação pede o valor do jogador (0x4d0c60, que sorteia): lado a lado com o estado do sorteador, as
/// duas palavras de marcas e os gastos líquidos do comprador e do vendedor; a cópia sem contar os sorteios.
pub(crate) unsafe extern "thiscall" fn comparar_assess_transfer_deal(this: usize, transf: usize, f1: usize, f2: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_ASSESS_TRANSFER_DEAL);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(f1, 2), (f2, 2)]);
    let person = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(transf + 0xc).wrapping_mul(staff::SIZE));
    let buyer = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(transf + 8).wrapping_mul(club::SIZE));
    reg.push((board_row_at(this, read_usize(buyer)) + 0x38, 8));
    let seller = read_usize(person + staff::CLUB_JOB);
    if seller != 0 {
        reg.push((board_row_at(this, read_usize(seller)) + 0x38, 8));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, transf, f1, f2),
        || crate::rng::without_counting(|| assess_transfer_deal(this, transf, f1, f2)));
    PLACAR.record(igual, || format!("assess_transfer_deal({:x}) {:x?} x {:x?}", transf, mo, mr));
}

/// Comparer for `apply_transfer_confidence_changes` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado com o estado do sorteador (a avaliação 0x696570 sorteia), as linhas da diretoria do
/// comprador e do vendedor e as três pessoas favoritas do presidente do vendedor (0x547c10); a cópia sem contar.
pub(crate) unsafe extern "thiscall" fn comparar_apply_transfer_confidence_changes(this: usize, transf: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_APPLY_TRANSFER_CONFIDENCE_CHANGES);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    let person = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(transf + 0xc).wrapping_mul(staff::SIZE));
    reg.push((board_row_at(this, read_usize(transf + 8)), 0x49));
    let seller = read_usize(person + staff::CLUB_JOB);
    if seller != 0 {
        reg.push((board_row_at(this, read_usize(seller)), 0x49));
        let pref = staff_preferences(read_usize(seller + club::CHAIRMAN));
        if pref != 0 {
            reg.push((pref + preferences::STAFF_FAVOURITE_STAFF1, 12));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, transf),
        || crate::rng::without_counting(|| apply_transfer_confidence_changes(this, transf)));
    PLACAR.record(igual, || format!("apply_transfer_confidence_changes({:x}) {:x?} x {:x?}", transf, mo, mr));
}

/// Comparer for `resolve_board_ultimatum` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original tira o item da lista (e o libera) e acrescenta a saída: roda só ela e confere o plano: a
/// lista das saídas (+4) cresce de 1 quando devolve 1 (e a linha da diretoria fica com o bit 2) e fica igual com 0.
pub(crate) unsafe extern "thiscall" fn comparar_resolve_board_ultimatum(this: usize, clube: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_RESOLVE_BOARD_ULTIMATUM);
    let outs = read_usize(this + 4);
    if outs == 0 || read_usize(this + 0x10) == 0 {
        return o(this, clube);
    }
    let before = read_i16(outs) as i32;
    let r = o(this, clube);
    let after = read_i16(read_usize(this + 4)) as i32;
    let ok = after == before + (r != 0) as i32 && (r == 0 || *((board_row_at(this, read_usize(clube)) + 2) as *const u8) & 2 != 0);
    PLACAR.record(ok, || format!("resolve_board_ultimatum(clube {:x}) devolveu {}, saídas {} -> {}", clube, r, before, after));
    r
}

/// Comparer for `process_pending_departures` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_process_pending_departures(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PROCESS_PENDING_DEPARTURES)(this)
}

/// Comparer for `manager_manager_remove_player_from_squad` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original tira a pessoa das listas, da mídia e das táticas (não pode rodar duas vezes): roda só ela e
/// confere o plano: a função e o clube (e a seleção) da pessoa e ela fora das 50 vagas do elenco.
pub(crate) unsafe extern "thiscall" fn comparar_manager_manager_remove_player_from_squad(this: usize, person: usize, clube: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(IDX_MANAGER_MANAGER_REMOVE_PLAYER_FROM_SQUAD);
    let in_club = read_usize(person + staff::CLUB_JOB) == clube;
    let in_national_team = read_usize(person + staff::NATIONAL_JOB) == clube;
    let (mut f39, mut c39) = (read_u8(person + staff::JOB_FOR_CLUB), read_usize(person + staff::CLUB_JOB));
    let (mut f28, mut c24) = (read_u8(person + staff::JOB_FOR_NATION), read_usize(person + staff::NATIONAL_JOB));
    if in_club {
        let (f, fica) = role_after_leaving(f39);
        f39 = f;
        if !fica {
            c39 = 0;
        }
    }
    if c24 == clube {
        let (f, fica) = role_after_leaving(f28);
        f28 = f;
        if !fica {
            c24 = 0;
        }
    }
    let r = o(this, person, clube);
    let away = !(in_club || in_national_team) || (0..0x32usize).all(|k| read_usize(clube + club::SQUAD + 4 * k) != person);
    let ok = away && read_u8(person + staff::JOB_FOR_CLUB) == f39 && read_usize(person + staff::CLUB_JOB) == c39 && read_u8(person + staff::JOB_FOR_NATION) == f28 && read_usize(person + staff::NATIONAL_JOB) == c24;
    PLACAR.record(ok, || format!("manager_manager_remove_player_from_squad(pessoa {:x}, clube {:x}): função {:x}/{:x} clube {:x}/{:x} (esperava {:x}/{:x} {:x}/{:x}), fora das vagas {}",
        read_usize(person), read_usize(clube), read_u8(person + staff::JOB_FOR_CLUB), read_u8(person + staff::JOB_FOR_NATION), read_usize(person + staff::CLUB_JOB), read_usize(person + staff::NATIONAL_JOB), f39, f28, c39, c24, away));
    r
}

/// Comparer for `remove_staff_from_club` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original tira o contrato e tira a pessoa da mídia e das listas do humano (não pode rodar duas vezes):
/// roda só ela e confere o plano feito antes com o corpo (`detach_staff_from_club`) sobre cópias dos campos da pessoa e das vagas
/// do clube. Pessoa humana: só a original (as listas do humano podem mexer no clube).
pub(crate) unsafe extern "thiscall" fn comparar_remove_staff_from_club(this: usize, person: usize, clube: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(IDX_REMOVE_STAFF_FROM_CLUB);
    if person == 0 || clube == 0 || read_i32(person) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10) {
        return o(this, person, clube);
    }
    // the plan: the fields in copies with the same offsets
    let mut s = [0u8; 0x40];
    let mut c = [0u8; 0x1e0];
    core::ptr::copy_nonoverlapping(person as *const u8, s.as_mut_ptr(), 0x40);
    core::ptr::copy_nonoverlapping(clube as *const u8, c.as_mut_ptr(), 0x1e0);
    detach_staff_from_club(person, s.as_mut_ptr() as usize, clube, c.as_mut_ptr() as usize);
    let expected = staff_job_fields(s.as_ptr() as usize, c.as_ptr() as usize);
    let r = o(this, person, clube);
    let after = staff_job_fields(person, clube);
    PLACAR.record(after == expected, || format!("remove_staff_from_club(pessoa {:x}, função {}, clube {:x}): pessoa {:x?}{:x?} esperava {:x?}{:x?}",
        read_usize(person), read_u8(person + staff::JOB_FOR_CLUB) as i8, read_usize(clube), after.0, after.1, expected.0, expected.1));
    r
}

/// Comparer for `fix_missing_jobs` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sem vaga a abrir (o plano pelo mesmo teste, com o cursor da lista devolvido), lado a lado pelo cursor
/// da lista; com vaga a abrir (ou a caixa), só a original.
pub(crate) unsafe extern "thiscall" fn comparar_fix_missing_jobs(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_FIX_MISSING_JOBS);
    let l = read_usize(this + 8);
    let cursor = read_usize(l + 0xa);
    let mut eh_a = 0u32;
    let mut clube = read_usize(G_CLUB_TABLE);
    let mut empty = true;
    for _ in 0..(read_usize(G_CLUB_COUNT) as i32).max(0) {
        if club_needs_vacancy(this, clube, &mut eh_a) != Some(false) {
            empty = false;
            break;
        }
        clube += 0x245;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cursor);
    if !empty {
        // simplification: 0x69ba50 opens the slot (allocates, draws the candidate): does not run twice; Rust mode and the career prove
        return o(this);
    }
    let reg = [(l + 0xa, 4)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this), || fix_missing_jobs(this));
    PLACAR.record(igual, || format!("fix_missing_jobs (nenhuma a abrir) {:x?} x {:x?}", mo, mr));
}

/// Comparer for `best_manager_candidate_score` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado a partir do mesmo estado do sorteio (a 0x69d1b0 e a 0x69f810 podem sortear), pela nota e
/// pelo cursor da lista de vagas (a 0x69fc60 o devolve).
pub(crate) unsafe extern "thiscall" fn comparar_best_manager_candidate_score(this: usize, clube: usize, current: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(IDX_BEST_MANAGER_CANDIDATE_SCORE);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((read_usize(this + 8) + 0xa, 4));
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, clube, current), || crate::rng::without_counting(|| best_manager_candidate_score(this, clube, current)));
    PLACAR.record(igual && r == n, || format!("best_manager_candidate_score(clube {}, atual {:x}) original={} rust={} {:x?} x {:x?}", read_usize(clube), current, r, n, mo, mr));
    r
}

/// Comparer for `update_board_after_fixture` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a 0x695240 mexe na diretoria (e pode dar notícia): só a original (a prova é a carreira e o modo Rust).
pub(crate) unsafe extern "thiscall" fn comparar_update_board_after_fixture(this: usize, partida: usize, jogo: usize) {
    crate::comparison::branch("update_board_after_fixture");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_UPDATE_BOARD_AFTER_FIXTURE)(this, partida, jogo)
}

/// Comparer for `is_staff_suitable_for_job` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a rotina só lê (o interesse 0x844780 sem a saída das razões): pela devolução.
pub(crate) unsafe extern "thiscall" fn comparar_is_staff_suitable_for_job(this: usize, person: usize, clube: usize, nota_minima: f64) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, f64) -> u8>(IDX_IS_STAFF_SUITABLE_FOR_JOB)(this, person, clube, nota_minima);
    let n = is_staff_suitable_for_job(this, person, clube, nota_minima);
    PLACAR.record(r == n, || format!("is_staff_suitable_for_job(pessoa {:x}, clube {:x}, {}) original={} rust={}", person, clube, nota_minima, r, n));
    r
}

/// Comparer for `manager_manager_remove` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a 0x68d1f0 libera o nó e o item (não roda duas vezes): o plano antes (o nó achado pela cópia, a cabeça,
/// a cauda, a conta, o cursor e os vizinhos); a original tira; confere a devolução, o cabeçalho (14 bytes) e os elos
/// dos vizinhos.
pub(crate) unsafe extern "thiscall" fn comparar_manager_manager_remove(this: usize, given: usize) -> u32 {
    let no = find_list_node_by_bytes(this, given);
    let before: [u8; 14] = core::ptr::read_unaligned(this as *const [u8; 14]);
    let (prox, ant) = if no != 0 { (read_usize(no + 6), read_usize(no + 0xa)) } else { (0, 0) };
    let (cabeca, cauda, cursor) = (read_usize(this + 2), read_usize(this + 6), read_usize(this + 0xa));
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_MANAGER_MANAGER_REMOVE)(this, given);
    let after: [u8; 14] = core::ptr::read_unaligned(this as *const [u8; 14]);
    let ok = if no == 0 {
        r == 0 && after == before
    } else {
        let count = core::ptr::read_unaligned(before.as_ptr() as *const i16).wrapping_sub(1);
        r == 1
            && core::ptr::read_unaligned(this as *const i16) == count
            && read_usize(this + 2) == if no == cabeca { prox } else { cabeca }
            && read_usize(this + 6) == if no == cauda { ant } else { cauda }
            && read_usize(this + 0xa) == if no == cursor { prox } else { cursor }
            && (no == cabeca || read_usize(ant + 6) == prox)
            && (no == cauda || read_usize(prox + 0xa) == ant)
    };
    PLACAR.record(ok, || format!("manager_manager_remove({:#x}, {:#x}) original={} nó {:#x} cabeçalho {:x?} -> {:x?}", this, given, r, no, before, after));
    r
}

/// Comparer for `manager_manager_clear` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: libera os nós (não roda duas vezes): só a original; confere o cabeçalho zerado (14 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_manager_manager_clear(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_MANAGER_MANAGER_CLEAR)(this);
    let after: [u8; 14] = core::ptr::read_unaligned(this as *const [u8; 14]);
    PLACAR.record(after == [0u8; 14], || format!("manager_manager_clear({:#x}) cabeçalho {:x?}", this, after));
}

/// Comparer for `get_current` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_current(this: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_GET_CURRENT)(this);
    let n = get_current(this);
    PLACAR.record(r == n, || format!("get_current({:#x}) original={:#x} rust={:#x}", this, r, n));
    r
}

/// Comparer for `get_current_run` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original grava nas saídas de verdade, a cópia em locais; a 0x4a4850 pode reordenar as estatísticas
/// (a original primeiro; a cópia acha a tabela já em ordem e a mesma linha).
pub(crate) unsafe extern "thiscall" fn comparar_get_current_run(this: usize, clube: usize, kind: *mut u8, size: *mut u8) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8)>(IDX_GET_CURRENT_RUN)(this, clube, kind, size);
    let (mut t, mut n) = (0u8, 0u8);
    get_current_run(this, clube, &mut t, &mut n);
    PLACAR.record(*kind == t && *size == n, || format!("get_current_run({:x}) original={}/{} rust={}/{}", clube, *kind, *size, t, n));
}

/// Comparer for `post_job_rumour_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_job_rumour_news(this: usize, person: usize, clube: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_POST_JOB_RUMOUR_NEWS)(this, person, clube)
}
