use super::*;





/// O ensaio dos comparadores: as cópias sem os históricos (arquivo), sem as notícias (0x75c390), sem acabar as
/// convocações (0x774f30) e sem a renovação (0x4cda70), anotando os contratos que a renovação mexeria.
pub(crate) static mut STAFF_HISTORY_DRY_RUN: bool = false;
pub(crate) static mut DRY_RUN_RENEWED_CONTRACTS: Vec<usize> = Vec::new();
/// Depois da última conta de dias (0x54cdb0) da cópia, o ensaio pulou uma chamada da original que também conta dias
/// e deixa a data de rascunho estática dela (0xae2470) noutro ano: a renovação (0x4cda70, pela idade), o fim das
/// convocações (0x774f30) ou a notícia do elogio (0x75c390). Essa data então fica fora da conta.
pub(crate) static mut DRY_RUN_STRAY_SCRATCH_DATE: bool = false;

/// Raises club reputation (club::REPUTATION) toward weight × 500, at most 750 at once, clamped to 1..10000.
///
/// [pt] A reputação do clube (+0x80) sobe até peso × 500, no máximo 750 de uma vez (entre 1 e 10000; soma em 16 bits).
pub(crate) unsafe fn raise_champion_reputation(clube: usize, peso: i16, campeao_da_liga: bool) {
    let alvo = (peso as i32).wrapping_mul(500);
    let rep = read_i16(clube + club::REPUTATION) as i32;
    if rep < alvo {
        let falta = alvo - rep;
        let soma = if falta > 0x2ee { 0x2ee } else { falta }; // cqd13: o teto da subida
        let v = read_i16(clube + club::REPUTATION).wrapping_add(soma as i16);
        let v = if v < 1 { 1 } else if v > 10000 { 10000 } else { v };
        if campeao_da_liga {
            crate::comparacao::ramo("cqd_13 injury_word_0c do campeao da liga");
        } else {
            crate::comparacao::ramo("cqd_13 injury_word_0c do campeao da copa");
        }
        core::ptr::write_unaligned((clube + club::REPUTATION) as *mut i16, v); // cqd13: a reputação do campeão
    }
}

/// Adds d to the contract morale byte (+0x44), clamped to −100..100.
///
/// [pt] O moral do contrato (+0x44, byte com sinal) sobe `d`, preso em −100..100.
pub(crate) unsafe fn add_contract_morale(k: usize, d: i32) {
    let c = *((k + 0x44) as *const i8);
    let x = c as i32 + d;
    *((k + 0x44) as *mut i8) = if x > 100 { 100 } else if x < -100 { -100 } else { c.wrapping_add(d as i8) };
}

/// Offers a contract renewal (0x4cda70, factor 1.0) to an own-club player when the transfer manager exists, the
/// contract (+0x25) is over 300 days old and he is on the club shortlist; rehearsal only notes it.
///
/// [pt] A renovação oferecida ao jogador do próprio clube (0x4cda70 com 1,0 e submeter): com o gerente de transferências
/// ([0xde1f54]), mais de 300 dias desde a data do contrato (+0x25; 0x54cdb0) e a vaga dele na lista do clube (0x849ce0
/// com 0, 0; +0xf o id). No ensaio, só anota o contrato.
pub(crate) unsafe fn offer_renewal_if_due(clube: usize, p: usize, k: usize) {
    if read_usize(G_TRANSFER_MANAGER) == 0 {
        return;
    }
    DRY_RUN_STRAY_SCRATCH_DATE = false;
    if days_between(G_CURRENT_DATE, k + 0x25) <= 300 {
        return;
    }
    let vaga_de: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    // 0x849ce0: push 0; push 0; push p; push clube; mov ecx, [[0xde1f54]+0x856] -> thiscall (ret 0x10)
    let v = vaga_de(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), clube, p, 0, 0);
    if v == 0 || read_i32(v + 0xf) != read_i32(p) {
        return;
    }
    if STAFF_HISTORY_DRY_RUN {
        (*core::ptr::addr_of_mut!(DRY_RUN_RENEWED_CONTRACTS)).push(k);
        DRY_RUN_STRAY_SCRATCH_DATE = true;
        return;
    }
    let oferecer: unsafe extern "thiscall" fn(usize, usize, u32, u32, i32, f64, u32) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_OFFER_CONTRACT);
    // 0x4cda70: push 1; push 0x3ff00000; push 0; push −1; push 0; push 0; push p; mov ecx, [0xae1050] -> thiscall
    // (p, 0, 0, −1, 1.0 em dois dwords, 1)
    oferecer(read_usize(G_CONTRACT_MANAGER), p, 0, 0, -1, 1.0, 1);
}

/// Adds history entry e for each of the 20 lineup players (+0x190, 0x49 bytes, −1 empty) except non-players and the
/// manager.
///
/// [pt] Os históricos da entrada `e` para o técnico (já feito por quem chama), o presidente (+0xbf) e os 20 da escalação
/// (+0x190 do lado, 0x49 bytes, −1 vazio; pessoas válidas e fora o técnico; nula: a caixa da linha 0x1010).
pub(crate) unsafe fn add_lineup_histories(this: usize, clube: usize, escalacao: usize, e: &[u8; 0x40]) {
    for j in 0..0x14usize {
        let id = read_i32(escalacao + 0x49 * j);
        if id == -1 {
            continue;
        }
        let p = read_usize(G_STAFF_TABLE).wrapping_add((id as u32).wrapping_mul(0x6e) as usize);
        if p != 0 && read_i32(p) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        if p == read_usize(clube + club::MANAGER) {
            continue;
        }
        if p == 0 {
            show_error_at(0xa83b78, 0x1010);
            continue;
        }
        append_person_history_by_id(!STAFF_HISTORY_DRY_RUN, this, read_usize(p), e);
    }
}

/// Adds a dated club history entry (type at +8, competition id at +9) to the club history FILE_LLIST [this+4]; skipped
/// in rehearsal.
///
/// [pt] O histórico do clube (tipo no +8, a competição no +9).
pub(crate) unsafe fn add_club_history(this: usize, clube: usize, tipo: u8, comp: usize) {
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[8] = tipo;
    put_i32(&mut e, 9, read_i32(comp));
    if !STAFF_HISTORY_DRY_RUN {
        let f: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) -> usize = core::mem::transmute(FN_FILE_LLIST_ADD_NODE);
        let p = read_usize(this + 0x10).wrapping_add(read_usize(clube).wrapping_mul(8));
        f(read_usize(this + 4), p, p + 4, e.as_ptr());
    }
}

/// Queues the trophy praise media story (0x75c390 on G_MEDIA); in rehearsal only marks the static date as loose.
///
/// [pt] A notícia do elogio do título (0x75c390 em [0xb6b7c8]; fora do ensaio).
pub(crate) unsafe fn post_trophy_praise(clube: usize, comp: usize, marca: u32) {
    if STAFF_HISTORY_DRY_RUN {
        DRY_RUN_STRAY_SCRATCH_DATE = true;
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_MEDIA_QUEUE_TROPHY_PRAISE_STORY);
    // 0x75c390: push marca; push comp; push clube; mov ecx, [0xb6b7c8] -> thiscall (ret 0xc)
    f(read_usize(G_MEDIA), clube, comp, marca);
}

/// Manager rating increments (k1, k2) by competition stage: 400/400, or 200 (foreign manager) / 200 in a stage-1
/// competition (+0x42 = 1).
///
/// [pt] As notas k1/k2 do técnico pela fase: na competição de fase 1 ([0xadadfc] + 4 × id, +0x42 = 1) 200 na terceira e 200
/// na segunda se o técnico for estrangeiro (0x53d980 com a nação +0x53 e 1), senão 400.
pub(crate) unsafe fn manager_rating_gains(t: usize, clube: usize, id_comp: usize) -> (i32, i32) {
    let c = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * id_comp));
    if *((c + 0x42) as *const i8) != 1 {
        (400, 400)
    } else {
        let estrangeiro: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
        // 0x53d980: 3 pushes; add esp, 0xc -> cdecl
        (if estrangeiro(t, read_usize(clube + club::NATION), 1) != 0 { 200 } else { 400 }, 200)
    }
}

/// the league champion: the club reputation (up to weight × 500), the +0x7b and the +0x69 block, the manager's ratings
/// and record (weight × 5000), the histories (type 5), the squad morale by the player's average (above the threshold:
/// +20 and +100 on the contract with the renewal; else +10 and +50) and the media praise
/// Original: `Staff_Records::add_league_champions_record` (0x868c50, staff_records.cpp, thiscall).
///
/// [pt] { 0x868c50 } (thiscall, ret 8, staff_records.cpp, SEH só das datas locais): `Staff_Records::
/// add_league_champions_record(clube, comp)`, pelos campeões das ligas no fim delas (25 chamadores, um por liga; 4
/// chamadas até 2002-06-01). Clube ou competição nulos: a caixa (linha 0x21c); a competição sem objeto em [0xadadfc]
/// (+ 4 × id): a caixa (0x224). A reputação (até peso × 500, +750 no máximo); com +0x57 e o bloco +0x69 do clube: o
/// +0x7b sobe peso × 20 (abaixo de 150000 e de +0x73 + 500) e o +0x44 do bloco 25 × (peso + 20) (abaixo de 120000). O
/// limiar dos jogadores: ftol((+0x3e − 1) × +0x3c do objeto × 0,33), em 16 bits. Com técnico (+0xcf): as três notas
/// dele (+0xa peso × 400, +8 peso × k1, +0xc peso × k2 no registro +0x69, se houver) e o histórico (tipo 5: o clube e a
/// competição); o registro dele (0x86f5b0 com criar): +1 em +0x8c e peso × 5000 em +0xb6, +0xca e +0xba. O presidente
/// (+0xbf). Os 50 do elenco (+0xd7) com ficha, válidos, do clube (+0x39) e fora o técnico: a média 2/1 (0x7c1e30 com
/// 6,5) acima do limiar dá o histórico, moral +20 (0x544710) e +100 no contrato (0x4cf5f0) com a renovação; senão moral
/// +10 e +50 no contrato. O histórico do clube (tipo 5); com a mídia ([0xb6b7c8]) e peso acima de 3, o elogio (0x75c390
/// com 1) e, acima de 12, com 3.
pub(crate) unsafe extern "thiscall" fn add_league_champions_record(this: usize, clube: usize, comp: usize) {
    if clube == 0 || comp == 0 {
        return show_error_at(0xa83b78, 0x21c);
    }
    let obj = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_usize(comp)));
    if obj == 0 {
        return show_error_at(0xa83b78, 0x224);
    }
    let peso = read_i16(comp + competition::CLUB_COMP_REPUTATION);
    raise_champion_reputation(clube, peso, true); // cqd13: a reputação do campeão da liga
    if read_usize(clube + club::DIVISION) != 0 {
        let b = read_usize(clube + club::STADIUM);
        if b != 0 {
            let x = read_i32(clube + club::MAX_ATTENDANCE);
            if x < 150000 && x < read_i32(clube + club::ATTENDANCE).wrapping_add(500) {
                write_i32(clube + club::MAX_ATTENDANCE, x.wrapping_add((peso as i32).wrapping_mul(20)));
            }
            let y = read_i32(b + 0x44);
            if y < 120000 {
                write_i32(b + 0x44, y.wrapping_add(((peso as i32).wrapping_add(20)).wrapping_mul(25)));
            }
        }
    }
    let n = ((read_i16(obj + 0x3e) as i32) - 1).wrapping_mul(read_i16(obj + 0x3c) as i32);
    let limiar = match_pl_ftol(n as f64 * read_f64_at(0x968f78)) as i16; // o ftol fica no word (movsx word depois); 0x968f78 = 0.33
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 5;
    put_i32(&mut e, 8, read_i32(clube));
    put_i32(&mut e, 0xd, read_i32(comp));
    let t = read_usize(clube + club::MANAGER);
    if t != 0 {
        let np = read_usize(t + 0x69);
        if np != 0 {
            let (k1, k2) = manager_rating_gains(t, clube, read_usize(comp));
            raise_side_rating(np + 0xa, peso, 400);
            raise_side_rating(np + 8, peso, k1);
            raise_side_rating(np + 0xc, peso, k2);
        }
        append_person_history_by_id(!STAFF_HISTORY_DRY_RUN, this, read_usize(t), &e);
        let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
        // 0x86f5b0: push 1; push técnico; mov ecx, edi (o this) -> thiscall (ret 8)
        let r = registro(this, read_usize(clube + club::MANAGER), 1);
        if r != 0 {
            core::ptr::write_unaligned((r + 0x8c) as *mut i16, read_i16(r + 0x8c).wrapping_add(1));
            let v = (peso as i32).wrapping_mul(5000);
            write_i32(r + 0xb6, read_i32(r + 0xb6).wrapping_add(v));
            write_i32(r + 0xca, read_i32(r + 0xca).wrapping_add(v));
            write_i32(r + 0xba, read_i32(r + 0xba).wrapping_add(v));
        }
    }
    let presidente = read_usize(clube + club::CHAIRMAN);
    if presidente != 0 {
        append_person_history_by_id(!STAFF_HISTORY_DRY_RUN, this, read_usize(presidente), &e);
    }
    let media: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
    let moral: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_STAFF_ADJUST_MORALE);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    // 0x7c1e30: push 6,5f; push 1; push 2; push ficha; mov ecx, [0xb74350] -> thiscall (ret 0x10, x87); 0x544710: 3
    // pushes; add esp, 0xc -> cdecl; 0x4cf5f0: push clube; push p; mov ecx, [0xae1050] -> thiscall (ret 8)
    for j in 0..50usize {
        let p = read_usize(clube + club::SQUAD + 4 * j);
        if p == 0 || read_usize(p + 0x61) == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) || read_usize(p + 0x39) != clube
            || p == read_usize(clube + club::MANAGER)
        {
            continue;
        }
        let f = media(read_usize(G_PLAYER_STATS), read_usize(p + 0x61), 2, 1, 6.5);
        if f > limiar as f64 { // cqd13: acima do limiar
            crate::comparacao::ramo("cqd_13 jogador destacado do campeao");
            append_person_history_by_id(!STAFF_HISTORY_DRY_RUN, this, read_usize(p), &e);
            moral(p, 0x14, 1); // cqd13: o moral do jogador destacado
            let k = contrato(read_usize(G_CONTRACT_MANAGER), p, clube);
            if k == 0 {
                continue;
            }
            crate::comparacao::ramo("cqd_13 contrato do destacado");
            add_contract_morale(k, 100); // cqd13: o contrato do jogador destacado
            offer_renewal_if_due(clube, p, k);
        } else {
            moral(p, 0xa, 1);
            let k = contrato(read_usize(G_CONTRACT_MANAGER), p, clube);
            if k != 0 {
                add_contract_morale(k, 50);
            }
        }
    }
    add_club_history(this, clube, 5, comp);
    if read_usize(G_MEDIA) != 0 && peso > 3 {
        post_trophy_praise(clube, comp, 1);
        if peso > 12 {
            post_trophy_praise(clube, comp, 3);
        }
    }
}

/// the cup winner: the club reputation, its side's ratings in the match, the manager's record (weight^4 × 0.5), the
/// histories (type 0), the squad morale (+15 and +100 on the contract, with the renewal) while the reputation stays up
/// to weight × 400, the end of the call-ups and the media praise
/// Original: `Staff_Records::add_cup_winners_record` (0x86a800, staff_records.cpp, thiscall).
///
/// [pt] { 0x86a800 } (thiscall, ret 0xc, staff_records.cpp, SEH só das datas locais): `Staff_Records::add_cup_winners_record(
/// clube, jogo, partida)`, pelas finais das copas (29 chamadores; 11 chamadas até 2002-06-01). Nulos: a caixa (linha
/// 0x51e). A reputação (como na liga); com técnico (+0xcf) que tem registro de não jogador (+0x69): as três notas do lado
/// do clube na partida (+0x188/+0x18a/+0x18c, lado 0x625 × visitante; peso × 400, × k1, × k2); o histórico do técnico
/// (tipo 0: o clube e a competição de +0x14 do jogo); o registro dele (0x86f5b0 com criar): +1 em +0x8a e ftol(peso^4 ×
/// 0,5) (pow da CRT 0x945ff0) em +0xb6, +0xca e, na seleção (0x53b900), +0xbe, senão +0xba. O presidente (+0xbf) e os
/// 20 da escalação. Os 50 do elenco enquanto a reputação não passar de peso × 400: com ficha, válidos e fora o técnico,
/// moral +15 (0x544710) e +100 no contrato; com a situação do contrato (+0x4f >> 4) 1-3 ou 5, a renovação. As
/// convocações da competição guardada (0x76d940) acabam (0x774f30); o histórico do clube (tipo 0); com a mídia e peso
/// acima de 10, o elogio (0x75c390 com 1) e, com peso acima de 12 ou reputação abaixo de 2500, com 3.
pub(crate) unsafe extern "thiscall" fn add_cup_winners_record(this: usize, clube: usize, jogo: usize, partida: usize) {
    if clube == 0 || jogo == 0 || partida == 0 {
        return show_error_at(0xa83b78, 0x51e);
    }
    let comp = read_usize(jogo + 0x14);
    let peso = read_i16(comp + competition::CLUB_COMP_REPUTATION);
    raise_champion_reputation(clube, peso, false);
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0;
    put_i32(&mut e, 8, read_i32(clube));
    put_i32(&mut e, 0xd, read_i32(comp));
    let t = read_usize(clube + club::MANAGER);
    if t != 0 {
        if read_usize(t + 0x69) != 0 {
            let (k1, k2) = manager_rating_gains(t, clube, read_usize(jogo));
            let lado = partida + 0x188 + 0x625 * (read_usize(jogo + 0x20) == clube) as usize;
            raise_side_rating(lado, peso, 400);
            raise_side_rating(lado + 2, peso, k1);
            raise_side_rating(lado + 4, peso, k2);
        }
        append_person_history_by_id(!STAFF_HISTORY_DRY_RUN, this, read_usize(read_usize(clube + club::MANAGER)), &e);
        let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
        let r = registro(this, read_usize(clube + club::MANAGER), 1);
        if r != 0 {
            core::ptr::write_unaligned((r + 0x8a) as *mut i16, read_i16(r + 0x8a).wrapping_add(1));
            // fild peso; fld 4,0; _CIpow (base em st1); fmul 0,5; ftol
            let v = match_pl_ftol(game_pow(read_i16(comp + competition::CLUB_COMP_REPUTATION) as f64, read_f64_at(0x968938)) * read_f64_at(0x968968)); // cqd13: o peso do título; 0x968938 = 4, 0x968968 = 0.5
            write_i32(r + 0xb6, read_i32(r + 0xb6).wrapping_add(v));
            write_i32(r + 0xca, read_i32(r + 0xca).wrapping_add(v));
            if is_national_team(clube) != 0 {
                write_i32(r + 0xbe, read_i32(r + 0xbe).wrapping_add(v));
            } else {
                write_i32(r + 0xba, read_i32(r + 0xba).wrapping_add(v));
            }
        }
    }
    let presidente = read_usize(clube + club::CHAIRMAN);
    if presidente != 0 {
        append_person_history_by_id(!STAFF_HISTORY_DRY_RUN, this, read_usize(presidente), &e);
    }
    let escalacao = partida + 0x190 + 0x625 * (clube != read_usize(jogo + 0x1c)) as usize;
    add_lineup_histories(this, clube, escalacao, &e);
    let moral: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_STAFF_ADJUST_MORALE);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    for j in 0..50usize {
        if (read_i16(comp + competition::CLUB_COMP_REPUTATION) as i32).wrapping_mul(400) < read_i16(clube + club::REPUTATION) as i32 {
            break;
        }
        let p = read_usize(clube + club::SQUAD + 4 * j);
        if p == 0 || read_usize(p + 0x61) == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) || p == read_usize(clube + club::MANAGER) {
            continue;
        }
        crate::comparacao::ramo("cqd_13 elenco do campeao da copa");
        moral(p, 0xf, 1); // cqd13: o moral do jogador do campeão da copa
        let k = contrato(read_usize(G_CONTRACT_MANAGER), p, clube);
        if k == 0 {
            continue;
        }
        crate::comparacao::ramo("cqd_13 moral do elenco campeao da copa");
        add_contract_morale(k, 100); // cqd13: o moral do contrato do campeão da copa
        let s = *((k + 0x4f) as *const u8) >> 4;
        if s == 0 || (s > 3 && s != 5) {
            continue;
        }
        offer_renewal_if_due(clube, p, k);
    }
    let guardada: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NATIONAL_TEAMS_IS_FINALS_COMP);
    // 0x76d940 e 0x774f30: push comp; mov ecx, [0xb74318] (a 0x76d940 não lê o ecx e só compara o id; ret 4 nas duas)
    if guardada(read_usize(G_NATIONAL_TEAMS), read_usize(jogo + 0x14)) != 0 {
        if STAFF_HISTORY_DRY_RUN {
            DRY_RUN_STRAY_SCRATCH_DATE = true;
        } else {
            let acabar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_END_DUTY_FOR_COMPETITION);
            acabar(read_usize(G_NATIONAL_TEAMS), read_usize(jogo + 0x14));
        }
    }
    add_club_history(this, clube, 0, comp);
    if read_usize(G_MEDIA) != 0 && peso > 10 {
        post_trophy_praise(clube, comp, 1);
        if read_i16(comp + competition::CLUB_COMP_REPUTATION) > 12 || read_i16(clube + club::REPUTATION) < 0x9c4 {
            post_trophy_praise(clube, comp, 3);
        }
    }
}

/// Rehearsal comparer for league and cup champions: runs copy and original from the same state, comparing club
/// reputation, manager record, ratings, attendance, squad morale and the static date.
///
/// [pt] O comparador em ensaio dos dois campeões: o registro do técnico criado antes (0x86f5b0 com criar, como a original
/// faz), a cópia em ensaio (sem históricos, notícias, convocações e renovação; os sorteios do moral 0x544710 sem contar)
/// e a original a partir do mesmo estado. Compara: a reputação do clube, o registro do técnico (0xd2 bytes), as notas
/// (as três do lado na partida, ou +8..+0xd do registro de não jogador do técnico), o +0x7b do clube e o +0x44 do bloco
/// +0x69 (na liga), o moral (+0x45 da ficha) e o do contrato (+0x44) dos 50 do elenco e a data estática da 0x54cdb0. O
/// moral dos contratos que a renovação mexeria (a original renova) e o estado do sorteador (as notícias da original
/// sorteiam) não entram na conta; o sorteador volta ao de antes para a original. A renovação da original também sorteia
/// e conta dias (0x54cdb0): o moral das fichas depois do primeiro renovado fica fora da igualdade e é conferido pela
/// faixa da 0x544710; a data estática fica fora quando o ensaio pulou, depois da última conta de dias da cópia, uma
/// chamada da original que conta dias (DRY_RUN_STRAY_SCRATCH_DATE). Visto na ENG em maio de 2002 e na auditoria das 8 ligas.
pub(crate) unsafe fn compare_champion_update(nome: &str, this: usize, clube: usize, notas: (usize, usize), liga: bool, original: impl FnOnce(), copia: impl FnOnce()) {
    let mut fixas: Vec<(usize, usize)> = vec![(clube + club::REPUTATION, 2), (G_DAYS_SINCE_SCRATCH_DATE, 8), notas];
    let t = read_usize(clube + club::MANAGER);
    if t != 0 {
        let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
        let r = registro(this, t, 1);
        if r != 0 {
            fixas.push((r, 0xd2));
        }
    }
    if liga && read_usize(clube + club::DIVISION) != 0 && read_usize(clube + club::STADIUM) != 0 {
        fixas.push((clube + club::MAX_ATTENDANCE, 4));
        fixas.push((read_usize(clube + club::STADIUM) + 0x44, 4));
    }
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let mut contratos = Vec::new();
    let mut fichas = Vec::new(); // (moral da ficha, contrato ou 0, moral de antes), na ordem do elenco
    for j in 0..50usize {
        let p = read_usize(clube + club::SQUAD + 4 * j);
        if p == 0 || read_usize(p + 0x61) == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        let k = contrato(read_usize(G_CONTRACT_MANAGER), p, clube);
        let m = read_usize(p + 0x61) + 0x45;
        fichas.push((m, k, *(m as *const i8)));
        if k != 0 {
            contratos.push(k);
        }
    }
    let mut todas = fixas.clone();
    todas.extend(fichas.iter().map(|&(m, _, _)| (m, 1)));
    todas.extend(contratos.iter().map(|&k| (k + 0x44, 1)));
    todas.extend(crate::aleatorio::REGIOES_DO_ESTADO.iter().copied());
    let antes = crate::comparacao::Memoria::ler(&todas);
    (*core::ptr::addr_of_mut!(DRY_RUN_RENEWED_CONTRACTS)).clear();
    DRY_RUN_STRAY_SCRATCH_DATE = false;
    STAFF_HISTORY_DRY_RUN = true;
    crate::aleatorio::sem_contar(copia);
    STAFF_HISTORY_DRY_RUN = false;
    let renovados = core::mem::take(&mut *core::ptr::addr_of_mut!(DRY_RUN_RENEWED_CONTRACTS));
    let mut conta = fixas;
    if DRY_RUN_STRAY_SCRATCH_DATE {
        conta.retain(|&(a, _)| a != G_DAYS_SINCE_SCRATCH_DATE);
    }
    // A renovação da original sorteia (0x4cda70): o moral dos jogadores depois do primeiro renovado (0x544710 sorteia)
    // sai de outro ponto do sorteador; o dele e os de antes entram na conta.
    let corte = renovados.first().and_then(|r| fichas.iter().position(|&(_, k, _)| k == *r)).unwrap_or(fichas.len());
    conta.extend(fichas.iter().take(corte + 1).map(|&(m, _, _)| (m, 1)));
    conta.extend(contratos.iter().filter(|k| !renovados.contains(k)).map(|&k| (k + 0x44, 1)));
    // Os de depois, pela faixa da 0x544710: o moral m sobe de 0 a 20 (+20 ou +10 sorteados), preso em 1..20.
    let band = |&(m, _, m0): &(usize, usize, i8)| -> bool {
        let (m0, v) = (m0 as i32, *(m as *const i8) as i32);
        v >= m0.clamp(1, 20) && v <= (m0 + 20).clamp(1, 20)
    };
    let mut na_faixa = fichas.iter().skip(corte + 1).all(band);
    let mr = crate::comparacao::Memoria::ler(&conta);
    antes.escrever();
    original();
    na_faixa &= fichas.iter().skip(corte + 1).all(band);
    let mo = crate::comparacao::Memoria::ler(&conta);
    PLACAR.registrar(mo == mr && na_faixa, || format!("{}(clube {:#x}) renovados {} (o primeiro na ficha {} de {}; fora da band: {}) original={:x?} rust={:x?}",
        nome, clube, renovados.len(), corte, fichas.len(), !na_faixa, mo, mr));
}

/// Comparer for `add_league_champions_record` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_league_champions_record(this: usize, clube: usize, comp: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_ADD_LEAGUE_CHAMPIONS_RECORD);
    if clube == 0 || comp == 0 || read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_usize(comp))) == 0 {
        return o(this, clube, comp);
    }
    let t = read_usize(clube + club::MANAGER);
    let notas = if t != 0 && read_usize(t + 0x69) != 0 { (read_usize(t + 0x69) + 8, 6) } else { (clube + club::REPUTATION, 2) };
    compare_champion_update("add_league_champions_record", this, clube, notas, true, || o(this, clube, comp),
        || add_league_champions_record(this, clube, comp));
}

/// Comparer for `add_cup_winners_record` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_cup_winners_record(this: usize, clube: usize, jogo: usize, partida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_ADD_CUP_WINNERS_RECORD);
    if clube == 0 || jogo == 0 || partida == 0 {
        return o(this, clube, jogo, partida);
    }
    let notas = (partida + 0x188 + 0x625 * (read_usize(jogo + 0x20) == clube) as usize, 6);
    compare_champion_update("add_cup_winners_record", this, clube, notas, false, || o(this, clube, jogo, partida),
        || add_cup_winners_record(this, clube, jogo, partida));
}

/// the human manager record (one of the 16; else the person itself, as in the original) cleared, with −1 in the slots
/// Original: `Staff_Records::clear_human_manager_record` (0x86fc70, staff_records.cpp, thiscall).
///
/// [pt] { 0x86fc70 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::clear_human_manager_record(pessoa)`, pela
/// aposentadoria do técnico humano (0x6b2b40) e pelo setup (0x82b9xx). Pessoa entre os 16 humanos (id de [0xae2364] − 16
/// até [0xae2364]): o registro dele no vetor +8 (vaga id − [0xae2364] + [0xae2368] + 16, 0xd2 bytes); senão, como a
/// original, a própria pessoa. Zera os 0xd2 bytes e põe −1 em +0x86, +0xc2, +0xc6, +0x4a, +0x5e e +0x72.
pub(crate) unsafe extern "thiscall" fn clear_human_manager_record(this: usize, pessoa: usize) {
    let id = read_i32(pessoa);
    let lim = read_i32(G_STAFF_COUNT);
    let r = if id >= lim.wrapping_sub(0x10) && id < lim {
        let vaga = id.wrapping_sub(lim).wrapping_add(read_i32(G_NON_PLAYER_COUNT)).wrapping_add(0x10);
        read_usize(this + 8).wrapping_add((vaga as usize).wrapping_mul(0xd2))
    } else {
        pessoa
    };
    core::ptr::write_bytes(r as *mut u8, 0, 0xd2);
    for o in [0x86usize, 0xc2, 0xc6, 0x4a, 0x5e, 0x72] {
        write_i32(r + o, -1);
    }
}

/// Comparer for `clear_human_manager_record` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo registro (os 0xd2 bytes que a rotina zera).
pub(crate) unsafe extern "thiscall" fn comparar_clear_human_manager_record(this: usize, pessoa: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_CLEAR_HUMAN_MANAGER_RECORD);
    let id = read_i32(pessoa);
    let lim = read_i32(G_STAFF_COUNT);
    let r = if id >= lim.wrapping_sub(0x10) && id < lim {
        let vaga = id.wrapping_sub(lim).wrapping_add(read_i32(G_NON_PLAYER_COUNT)).wrapping_add(0x10);
        read_usize(this + 8).wrapping_add((vaga as usize).wrapping_mul(0xd2))
    } else {
        pessoa
    };
    let ((), (), igual, mo, mr) = crate::comparacao::lado_a_lado(&[(r, 0xd2)], || o(this, pessoa),
        || clear_human_manager_record(this, pessoa));
    PLACAR.registrar(igual, || format!("clear_human_manager_record(pessoa {:#x}) {:x?} x {:x?}", pessoa, mo, mr));
}






/// Adds 1 to the i16 at address a.
pub(crate) unsafe fn increment_i16(a: usize) {
    core::ptr::write_unaligned(a as *mut i16, read_i16(a).wrapping_add(1));
}


/// Returns the club manager's points record (FN_STAFF_RECORDS_GET_MANAGER_RECORD with create), or 0 without a manager.
///
/// [pt] O registro de pontos do técnico do clube (+0xcf, lido sem conferir o clube; 0x86f5b0 com criar), ou 0 sem técnico.
pub(crate) unsafe fn club_manager_record(this: usize, clube: usize) -> usize {
    let t = read_usize(clube + club::MANAGER);
    if t == 0 {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
    f(this, t, 1)
}

/// Points of one side in match m from four signed bytes: (((a + b) × 5 + (c − b)) × 2 + d) × 5.
///
/// [pt] Os pontos de um lado na partida `m`: ((a + b) × 5 + (c − b, em 8 bits)) × 2 + d, vezes 5 (bytes com sinal).
pub(crate) unsafe fn side_match_points(m: usize, a: usize, b: usize, c: usize, d: usize) -> i32 {
    let x = read_i8_as_i32(m + a).wrapping_add(read_i8_as_i32(m + b));
    let y = (*((m + c) as *const u8)).wrapping_sub(*((m + b) as *const u8)) as i8 as i32;
    let x = x.wrapping_mul(5).wrapping_add(y);
    read_i8_as_i32(m + d).wrapping_add(x.wrapping_mul(2)).wrapping_mul(5)
}

/// Match rating for the manager: importance (0x48e190) × opponent reputation / 500 × competition weight (+0x69) ×
/// points × 50 / 100000.
///
/// [pt] A nota do jogo para o técnico: a importância (0x48e190 da competição do jogo, com `lado` e a saída em `saida`; só o
/// byte, com sinal) × (reputação do adversário / 500) × o peso da competição (+0x69 de [+0x14]) × (pontos + bônus) × 50,
/// dividido por 100000 (divisões com sinal truncadas, como as multiplicações mágicas da original; produtos em 32 bits).
pub(crate) unsafe fn match_rating_for_manager(p: usize, lado: u32, saida: &mut u32, adversario: usize, pontos: i32) -> i32 {
    let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_usize(p)));
    let importancia: unsafe extern "thiscall" fn(usize, usize, u32, *mut u32) -> u32 = core::mem::transmute(FN_COMP_GET_FIXTURE_MATCH_IMPORTANCE);
    let i = importancia(comp, p, lado, saida as *mut u32) as u8 as i8 as i32;
    let x = i
        .wrapping_mul(read_i16(adversario + 0x80) as i32 / 500)
        .wrapping_mul(read_i16(read_usize(p + 0x14) + 0x69) as i32)
        .wrapping_mul(pontos)
        .wrapping_mul(50);
    x / 100000
}

/// in both managers' records: the match, the goals, the win, draw or loss and the match rating (importance × opponent
/// reputation × weight × points) in the month, the year, the club's league and the club or national season; the away
/// manager gets the home rating in the season total (original bug)
/// Original: `Staff_Records::record_match_result` (0x868830, staff_records.cpp, thiscall).
///
/// [pt] { 0x868830 } (thiscall, ret 8, staff_records.cpp): `Staff_Records::record_match_result(jogo, partida)`, pelo
/// fechamento da partida do dia (0x6b5f00), a cada jogo. Jogo não disputado (+0x3a = 1): nada. Os registros dos dois
/// técnicos (+0xcf do mandante +0x1c e do visitante +0x20; 0x86f5b0 com criar): +1 jogo (+0x8e); os gols pró (+0x96) e
/// contra (+0x9a) pelo placar (+0x47/+0x48, ou +0x43/+0x44 com +0x47 = −1); vitória (+0x90, bônus 900), empate (+0x92,
/// 300 para os dois) ou derrota (+0x94). A nota de cada um (0x48e190 com 0 / 1, a reputação do adversário, os pontos da
/// partida em +0x11f.. / +0x744..) vai para +0xb6 e +0xca; na liga do clube (+0x57 ou +0x60 = a competição +0x14) também
/// para os pontos do mês (+0xce); na seleção (0x53b900) para +0xbe, senão +0xba. Defeito da original, reproduzido: no
/// visitante, +0xbe/+0xba recebem o [esp+0x10] da rotina, que é a nota do mandante (ou o bônus dele, sem registro).
pub(crate) unsafe extern "thiscall" fn record_match_result(this: usize, p: usize, m: usize) {
    if *((p + 0x3a) as *const u8) == 1 {
        return;
    }
    let (casa, fora) = (read_usize(p + 0x1c), read_usize(p + 0x20));
    let a = club_manager_record(this, casa);
    let v = club_manager_record(this, fora);
    if a != 0 {
        increment_i16(a + 0x8e);
    }
    if v != 0 {
        increment_i16(v + 0x8e);
    }
    let (ga, gv) = if *((p + 0x47) as *const u8) == 0xff {
        (read_i8_as_i32(p + 0x43), read_i8_as_i32(p + 0x44))
    } else {
        (read_i8_as_i32(p + 0x47), read_i8_as_i32(p + 0x48))
    };
    if a != 0 {
        add_to_i32(a + 0x96, ga);
        add_to_i32(a + 0x9a, gv);
    }
    if v != 0 {
        add_to_i32(v + 0x96, gv);
        add_to_i32(v + 0x9a, ga);
    }
    let (mut ba, mut bv) = (0i32, 0i32);
    if ga > gv {
        ba = 900; // cqa11: o bônus da vitória do mandante
        if a != 0 {
            crate::comparacao::ramo("cqa_11 vitoria do mandante");
            increment_i16(a + 0x90);
        }
        if v != 0 {
            increment_i16(v + 0x94);
        }
    } else if ga < gv {
        bv = 900;
        if v != 0 {
            increment_i16(v + 0x90);
        }
        if a != 0 {
            increment_i16(a + 0x94);
        }
    } else {
        ba = 300;
        bv = 300;
        if a != 0 {
            increment_i16(a + 0x92);
        }
        if v != 0 {
            increment_i16(v + 0x92);
        }
    }
    // a saída da 0x48e190 vai na vaga do argumento `jogo` da original, que começa com o próprio jogo
    let mut saida = p as u32;
    let mut guardado = ba; // o [esp+0x10] da original: o bônus do mandante, depois a nota dele
    let comp = read_usize(p + 0x14);
    if a != 0 {
        crate::comparacao::ramo("cqa_11 nota do mandante");
        let q = match_rating_for_manager(p, 0, &mut saida, fora, side_match_points(m, 0x11f, 0x128, 0x127, 0x129).wrapping_add(ba));
        add_to_i32(a + 0xb6, q);
        add_to_i32(a + 0xca, q);
        guardado = q;
        if comp == read_usize(casa + 0x57) || comp == read_usize(casa + 0x60) {
            crate::comparacao::ramo("cqa_11 pontos do mes");
            add_to_i32(a + 0xce, q); // cqa11: os pontos do mês do mandante
        }
        if is_national_team(casa) != 0 {
            add_to_i32(a + 0xbe, q);
        } else {
            add_to_i32(a + 0xba, q);
        }
    }
    if v != 0 {
        let q = match_rating_for_manager(p, 1, &mut saida, casa, side_match_points(m, 0x744, 0x74d, 0x74c, 0x74e).wrapping_add(bv));
        add_to_i32(v + 0xb6, q);
        add_to_i32(v + 0xca, q);
        if comp == read_usize(fora + 0x57) || comp == read_usize(fora + 0x60) {
            add_to_i32(v + 0xce, q);
        }
        if is_national_team(fora) != 0 {
            add_to_i32(v + 0xbe, guardado);
        } else {
            add_to_i32(v + 0xba, guardado);
        }
    }
}

/// Comparer for `record_match_result` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador lado a lado nos dois registros (0xd2 bytes), criados antes pela 0x86f5b0 como a original faria (a segunda
/// chamada acha o mesmo registro). Jogo não disputado: só a original (não cria nada).
pub(crate) unsafe extern "thiscall" fn comparar_record_match_result(this: usize, p: usize, m: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_RECORD_MATCH_RESULT);
    if *((p + 0x3a) as *const u8) == 1 {
        return o(this, p, m);
    }
    let a = club_manager_record(this, read_usize(p + 0x1c));
    let v = club_manager_record(this, read_usize(p + 0x20));
    let mut reg = Vec::new();
    for r in [a, v] {
        if r != 0 {
            reg.push((r, 0xd2));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, p, m), || record_match_result(this, p, m));
    PLACAR.registrar(igual, || format!("record_match_result(jogo {:#x}) original={:x?} rust={:x?}", p, mo, mr));
}

/// at the end of the league, each club's final position in its manager's history
/// Original: `Staff_Records::add_league_finish_records` (0x86a070, staff_records.cpp, thiscall).
///
/// [pt] { 0x86a070 } (thiscall, ret 4, staff_records.cpp, SEH só da data local): `Staff_Records::add_league_finish_records(
/// liga)`, pela liga completa (0x689ad0), no fim da liga. Para cada linha da tabela (0x6826d0, até +0x3e), o histórico do
/// técnico do clube (+0xcf; tipo 0xa: o id do clube, a competição de +4, o ano +0x3a e a posição k + 1), no arquivo
/// (0x58e340).
pub(crate) unsafe extern "thiscall" fn add_league_finish_records(this: usize, liga: usize) {
    crate::comparacao::ramo("cqa_11 fim da liga");
    let linha: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FN_LEAGUE_GET_TABLE_ROW);
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0xa;
    put_i32(&mut e, 0x11, read_i16(liga + 0x3a) as i32);
    put_i32(&mut e, 0xd, read_i32(read_usize(liga + 4)));
    let mut k: i16 = 0;
    while k < read_i16(liga + 0x3e) {
        let clube = read_usize(linha(liga, k as i32));
        put_i32(&mut e, 0x15, k as i32 + 1);
        put_i32(&mut e, 8, read_i32(clube));
        let t = read_usize(clube + club::MANAGER);
        if t != 0 {
            append_person_history_by_id(true, this, read_usize(t), &e);
        }
        k += 1;
    }
}

// ponytail: só grava o histórico no arquivo (não roda duas vezes): só a original; a cópia leva o ramo.
/// Comparer for `add_league_finish_records` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_league_finish_records(this: usize, liga: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_LEAGUE_FINISH_RECORDS)(this, liga)
}

/// O ensaio do comparador da 0x86dc30: a cópia sem o histórico (arquivo).
pub(crate) static mut SUPPORTERS_PLAYER_OF_YEAR_DRY_RUN: bool = false;

/// the supporters' player of the year in his history and, by a draw, the favourite link between him and the club
/// Original: `Staff_Records::add_supporters_player_of_year_record` (0x86dc30, staff_records.cpp, thiscall).
///
/// [pt] { 0x86dc30 } (thiscall, ret 0x10, staff_records.cpp, SEH só da data local): `Staff_Records::
/// add_supporters_player_of_year_record(pessoa, clube, a, b)`, pelo fim da temporada dos recordes (0x447880). O histórico
/// da pessoa (tipo 0x35: o id do clube, `a` em 16 bits sem sinal e `b`), no arquivo (0x58e340); depois, se sorteio(21 −
/// +0x59 da pessoa) < sorteio(10), a pessoa gosta do clube (0x547d90 com sorteio(2)) e o clube dela (0x547870 com
/// sorteio(2)). Pessoa nula: a original (a caixa e a leitura de +0x59 da pessoa nula).
pub(crate) unsafe extern "thiscall" fn add_supporters_player_of_year_record(this: usize, pessoa: usize, clube: usize, a: u32, b: u32) {
    if pessoa == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_ADD_SUPPORTERS_PLAYER_OF_YEAR_RECORD)(this, pessoa, clube, a, b);
    }
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x35;
    put_i32(&mut e, 8, read_i32(clube));
    put_i32(&mut e, 0xd, (a & 0xffff) as i32);
    put_i32(&mut e, 0x11, b as i32);
    append_person_history_by_id(!SUPPORTERS_PLAYER_OF_YEAR_DRY_RUN, this, read_usize(pessoa), &e);
    let r1 = friendly_random(0x15 - read_i8_as_i32(pessoa + staff::LOYALITY));
    if r1 < friendly_random(10) {
        crate::comparacao::ramo("cqa_11 goal_crowd_reaction favorita");
        let gosta: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_FAVOURED_CLUB);
        let favorita: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_CLUB_ADD_FAVOURED_STAFF);
        gosta(pessoa, clube, friendly_random(2) as u32);
        favorita(clube, pessoa, friendly_random(2) as u32);
    }
}

/// Comparer for `add_supporters_player_of_year_record` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador em ensaio: a cópia sem o histórico e a original a partir do mesmo estado (sorteio, as pessoas favoritas e
/// rejeitadas do clube +0x9b e os clubes favoritos e rejeitados da pessoa, preferências +0x65: +4..+0x18). Pessoa nula:
/// só a original.
pub(crate) unsafe extern "thiscall" fn comparar_add_supporters_player_of_year_record(this: usize, pessoa: usize, clube: usize, a: u32, b: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_ADD_SUPPORTERS_PLAYER_OF_YEAR_RECORD);
    if pessoa == 0 || clube == 0 {
        return o(this, pessoa, clube, a, b);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((clube + club::FAV_STAFF1, 0x18));
    let pref = read_usize(pessoa + staff::STAFF_PREFERENCES);
    if pref != 0 {
        reg.push((pref + 4, 0x18));
    }
    let antes = crate::comparacao::Memoria::ler(&reg);
    SUPPORTERS_PLAYER_OF_YEAR_DRY_RUN = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| add_supporters_player_of_year_record(this, pessoa, clube, a, b)));
    SUPPORTERS_PLAYER_OF_YEAR_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&reg);
    antes.escrever();
    o(this, pessoa, clube, a, b);
    let mo = crate::comparacao::Memoria::ler(&reg);
    PLACAR.registrar(mo == mr, || format!("add_supporters_player_of_year_record(pessoa {:#x}, clube {:#x}) original={:x?} rust={:x?}", pessoa, clube, mo, mr));
}






/// O ensaio dos comparadores: as cópias sem o histórico (arquivo) e sem acabar as convocações.
pub(crate) static mut CUP_PLACED_CLUB_DRY_RUN: bool = false;

/// Appends history entry e to club id's list in the club history FILE_LLIST [this+4]; nothing in rehearsal (takes the
/// club id; skipped in rehearsal).
///
/// [pt] A entrada `e` no fim da lista do clube `id` no arquivo do histórico dos clubes (FILE_LLIST [this+4], 0x58e340; o
/// começo e o fim em [this+0x10] + id × 8). No ensaio, nada.
pub(crate) unsafe fn append_club_history_by_id(this: usize, id: usize, e: &[u8; 0x40]) {
    if CUP_PLACED_CLUB_DRY_RUN {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) -> usize = core::mem::transmute(FN_FILE_LLIST_ADD_NODE);
    let p = read_usize(this + 0x10).wrapping_add(id.wrapping_mul(8));
    f(read_usize(this + 4), p, p + 4, e.as_ptr());
}

/// Raises a side rating word by ftol(weight × k × [0x96ff88]), floored at weight × k and capped at 10000.
///
/// [pt] Uma nota do lado do clube na partida (palavra em `a`): soma ftol(peso × k × [0x96ff88]) (produto em 32 bits) e fica
/// no piso peso × k (16 bits) quando está abaixo dele, senão no máximo 10000 (comparações com sinal em 16 bits).
pub(crate) unsafe fn raise_side_rating(a: usize, peso: i16, k: i32) {
    let soma = match_pl_ftol((peso as i32).wrapping_mul(k) as f64 * read_f64_at(0x96ff88)); // 0x96ff88 = 0.1639344262
    let x = read_i16(a).wrapping_add(soma as i16);
    write_i16(a, x);
    let piso = peso.wrapping_mul(k as i16);
    write_i16(a, if piso > x { piso } else if x > 10000 { 10000 } else { x });
}

/// Cup runner-up (type 1) or third place (type 2): raises club reputation, manager ratings, writes histories for
/// manager, chairman and lineup, ends call-ups for third, and the club history.
///
/// [pt] O vice (tipo 1: limiar 500, teto 350, notas 300/150) e o terceiro (tipo 2: 500, 200, 200/100; o limiar 500 conferido em 0x86b774) da copa: a reputação do
/// clube (+0x80) sobe até peso da competição (+0x69 de [jogo+0x14]) × limiar, no máximo `teto` de uma vez (entre 1 e
/// 10000); com técnico (+0xcf) que tem registro de não jogador (+0x69), as três notas do lado do clube na partida
/// (+0x188/+0x18a/+0x18c, lado 0x625 × visitante): peso × k, e na competição de fase 1 ([0xadadfc] +0x42 = 1) peso ×
/// metade na terceira e na segunda se o técnico for estrangeiro (0x53d980 com a nação +0x53 e 1). Os históricos (data,
/// tipo, id do clube, competição): do técnico, do presidente (+0xbf) e dos 20 da escalação do lado (+0x190, 0x49 bytes,
/// −1 vazio; pessoas válidas e fora o técnico); no terceiro, as convocações acabam nas competições guardadas (0x76d940,
/// 0x774f90 com os dois clubes); por fim o histórico do clube (tipo no +8, competição no +9).
pub(crate) unsafe fn update_cup_placed_club(this: usize, clube: usize, jogo: usize, partida: usize, tipo: u8) {
    let (limiar, teto, k, metade) = if tipo == 1 { (500, 350, 300, 150) } else { (500, 200, 200, 100) };
    let comp = read_usize(jogo + 0x14);
    let peso = read_i16(comp + competition::CLUB_COMP_REPUTATION);
    let alvo = (peso as i32).wrapping_mul(limiar); // cqa12: o limiar da reputação
    let rep = read_i16(clube + club::REPUTATION) as i32;
    if rep < alvo {
        let falta = alvo - rep;
        let soma = if falta > teto { teto } else { falta }; // cqa12: o teto da subida
        let v = read_i16(clube + club::REPUTATION).wrapping_add(soma as i16);
        let v = if v < 1 { 1 } else if v > 10000 { 10000 } else { v };
        if tipo == 1 {
            crate::comparacao::ramo("cqa_12 injury_word_0c do vice");
        } else {
            crate::comparacao::ramo("cqa_12 injury_word_0c do terceiro");
        }
        write_i16(clube + club::REPUTATION, v); // cqa12: a reputação do colocado
    }
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = tipo;
    put_i32(&mut e, 8, read_i32(clube));
    put_i32(&mut e, 0xd, read_i32(comp));
    let t = read_usize(clube + club::MANAGER);
    if t != 0 {
        if read_usize(t + 0x69) != 0 {
            let c = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_usize(jogo)));
            let (k1, k2) = if *((c + 0x42) as *const i8) != 1 {
                (k, k)
            } else {
                let estrangeiro: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
                (if estrangeiro(t, read_usize(clube + club::NATION), 1) != 0 { metade } else { k }, metade)
            };
            let lado = partida + 0x188 + 0x625 * (read_usize(jogo + 0x20) == clube) as usize;
            raise_side_rating(lado, peso, k);
            raise_side_rating(lado + 2, peso, k1);
            raise_side_rating(lado + 4, peso, k2);
        }
        append_person_history_by_id(!CUP_PLACED_CLUB_DRY_RUN, this, read_usize(t), &e);
    }
    let presidente = read_usize(clube + club::CHAIRMAN);
    if presidente != 0 {
        append_person_history_by_id(!CUP_PLACED_CLUB_DRY_RUN, this, read_usize(presidente), &e);
    }
    let escalacao = partida + 0x190 + 0x625 * (clube != read_usize(jogo + 0x1c)) as usize;
    for j in 0..0x14usize {
        let id = read_i32(escalacao + 0x49 * j);
        if id == -1 {
            continue;
        }
        let p = read_usize(G_STAFF_TABLE).wrapping_add((id as u32).wrapping_mul(0x6e) as usize);
        if p != 0 && read_i32(p) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        if p == read_usize(clube + club::MANAGER) {
            continue;
        }
        if p == 0 {
            show_error_at(0xa83b78, 0x1010);
            continue;
        }
        append_person_history_by_id(!CUP_PLACED_CLUB_DRY_RUN, this, read_usize(p), &e);
    }
    if tipo == 2 && !CUP_PLACED_CLUB_DRY_RUN {
        let guardada: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NATIONAL_TEAMS_IS_FINALS_COMP);
        if guardada(read_usize(jogo + 0x14)) != 0 {
            let acabar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_END_DUTY_FOR_NATION_IN_COMPETITION);
            acabar(read_usize(G_NATIONAL_TEAMS), read_usize(jogo + 0x1c), read_usize(jogo + 0x14));
            acabar(read_usize(G_NATIONAL_TEAMS), read_usize(jogo + 0x20), read_usize(jogo + 0x14));
        }
    }
    let mut e2 = history_entry();
    stamp_game_date(e2.as_mut_ptr());
    e2[8] = tipo;
    put_i32(&mut e2, 9, read_i32(comp));
    append_club_history_by_id(this, read_usize(clube), &e2);
}

/// the cup runner-up: the club reputation rises up to weight × 500 (at most 350 at once), its side's three match
/// ratings by the weight, and the manager, chairman, line-up and club histories
/// Original: `Staff_Records::add_cup_runners_up_record` (0x86b0b0, staff_records.cpp, thiscall).
///
/// [pt] { 0x86b0b0 } (thiscall, ret 0xc, staff_records.cpp, SEH só das datas locais): `Staff_Records::
/// add_cup_runners_up_record(clube, jogo, partida)`, pela final das copas (Cup 0x519890, Liga dos Campeões 0x584ae0 e
/// as copas de cada nação): `update_cup_placed_club` com o tipo 1. Clube, jogo ou partida nulos: a caixa (linha 0x5f6).
pub(crate) unsafe extern "thiscall" fn add_cup_runners_up_record(this: usize, clube: usize, jogo: usize, partida: usize) {
    if clube == 0 || jogo == 0 || partida == 0 {
        return show_error_at(0xa83b78, 0x5f6);
    }
    update_cup_placed_club(this, clube, jogo, partida, 1);
}

/// the cup third place: the same with weight × 200 (at most 200) and smaller ratings; ends the call-ups in the stored
/// competitions
/// Original: `Staff_Records::add_cup_third_place_record` (0x86b710, staff_records.cpp, thiscall).
///
/// [pt] { 0x86b710 } (thiscall, ret 0xc, staff_records.cpp, SEH só das datas locais): `Staff_Records::
/// add_cup_third_place_record(clube, jogo, partida)`, pelas copas com disputa de terceiro: `update_cup_placed_club` com o tipo
/// 2. Nulos: a caixa (linha 0x67b).
pub(crate) unsafe extern "thiscall" fn add_cup_third_place_record(this: usize, clube: usize, jogo: usize, partida: usize) {
    if clube == 0 || jogo == 0 || partida == 0 {
        return show_error_at(0xa83b78, 0x67b);
    }
    update_cup_placed_club(this, clube, jogo, partida, 2);
}

/// Rehearsal comparer for cup runner-up/third: copy without histories vs original from the same state, comparing club
/// reputation and the side's three ratings.
///
/// [pt] Comparador em ensaio dos colocados: a cópia sem os históricos e sem acabar as convocações, e a original a partir do
/// mesmo estado (a reputação do clube e as três notas do lado dele na partida). Nulos: só a original.
pub(crate) unsafe fn compare_cup_placed(nome: &str, clube: usize, jogo: usize, partida: usize, original: impl FnOnce(), copia: impl FnOnce()) {
    if clube == 0 || jogo == 0 || partida == 0 {
        return original();
    }
    let reg = [(clube + club::REPUTATION, 2), (partida + 0x188 + 0x625 * (read_usize(jogo + 0x20) == clube) as usize, 6)];
    let antes = crate::comparacao::Memoria::ler(&reg);
    CUP_PLACED_CLUB_DRY_RUN = true;
    copia();
    CUP_PLACED_CLUB_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&reg);
    antes.escrever();
    original();
    let mo = crate::comparacao::Memoria::ler(&reg);
    PLACAR.registrar(mo == mr, || format!("{}(clube {:#x}, jogo {:#x}) original={:x?} rust={:x?}", nome, clube, jogo, mo, mr));
}

/// Comparer for `add_cup_runners_up_record` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_cup_runners_up_record(this: usize, clube: usize, jogo: usize, partida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_ADD_CUP_RUNNERS_UP_RECORD);
    compare_cup_placed("add_cup_runners_up_record", clube, jogo, partida, || o(this, clube, jogo, partida),
        || add_cup_runners_up_record(this, clube, jogo, partida))
}

/// Comparer for `add_cup_third_place_record` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_cup_third_place_record(this: usize, clube: usize, jogo: usize, partida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_ADD_CUP_THIRD_PLACE_RECORD);
    compare_cup_placed("add_cup_third_place_record", clube, jogo, partida, || o(this, clube, jogo, partida),
        || add_cup_third_place_record(this, clube, jogo, partida))
}

/// Copies date src into dst (0x417c10).
///
/// [pt] A data em `src` copiada para `dst` (0x417c10; a cópia de retorno vai para uma data local).
pub(crate) unsafe fn staff_records_copy_date(dst: usize, src: usize) {
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut t = [0u8; 8];
    copia(dst, t.as_mut_ptr(), src);
}

/// Days of date a minus date b (0x54cdb0, b by value; writes the static date 0xae2470).
///
/// [pt] Os dias da data `a` menos a `b` (0x54cdb0, `b` por valor; grava a data estática 0xae2470).
pub(crate) unsafe fn days_between(a: usize, b: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    f(a, read_usize(b) as u32, read_usize(b + 4) as u32)
}

/// closes the manager's current spell (shortest and longest), opens the new one when hired and counts the spells; the
/// manager, chairman and club histories
/// Original: `Staff_Records::record_manager_job_change` (0x869aa0, staff_records.cpp, thiscall).
///
/// [pt] { 0x869aa0 } (thiscall, ret 0xc, staff_records.cpp, SEH só das datas locais): `Staff_Records::
/// record_manager_job_change(técnico, clube, contratado)`, pela saída do técnico (0x69b790) e pelo técnico novo
/// (0x694c80). Fora da seleção (0x53b900), no registro do técnico (0x86f5b0 com criar), quando o começo do jogo
/// ([0xdd6ec8]) não é depois de hoje: a passagem atual (+0x4a clube, +0x4e começo) fecha; a mais curta (+0x72, +0x76
/// começo, +0x7e fim) é trocada pela atual se não houver ou se a atual for mais curta (0x54cdb0), ou só tem o fim
/// renovado se for a mesma; a mais longa (+0x5e, +0x62, +0x6a) é trocada se a atual for mais longa (o fim vira o +0x56)
/// ou se não houver (o fim vira hoje). Depois +0x4a = −1 e, contratado = 1, a passagem nova (o id do clube, hoje,
/// +0x56 = 13/11/1975 da 0x549f70) e +1 em +0xb4. Na seleção, contratado = 1: +1 em +0xb5. Os históricos: do técnico
/// (tipo 0x96: clube, contratado), do presidente do clube (+0xbf; tipo 0x97 com o técnico e as três chaves) e do clube
/// (tipo 0x96 no +8, o técnico, as chaves e contratado). Técnico ou clube nulos: a caixa (linha 0x3b1).
pub(crate) unsafe extern "thiscall" fn record_manager_job_change(this: usize, tecnico: usize, clube: usize, contratado: u32) {
    if clube == 0 || tecnico == 0 {
        return show_error_at(0xa83b78, 0x3b1);
    }
    let contratado = contratado as u8 as i8;
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
    if is_national_team(clube) == 0 {
        let r = registro(this, tecnico, 1);
        if r != 0 {
            let hoje = G_CURRENT_DATE;
            let (ai, ah) = (*((G_GAME_START_DATE + 2) as *const u16), *((hoje + 2) as *const u16));
            let depois = ai > ah || (ai == ah && read_i16(G_GAME_START_DATE) > read_i16(hoje));
            if !depois {
                crate::comparacao::ramo("cqa_12 passagem fechada");
                if read_i32(r + 0x72) != -1 && read_i32(r + 0x4a) != -1 {
                    if read_i32(r + 0x72) == read_i32(r + 0x4a) && read_i16(r + 0x50) == read_i16(r + 0x78) && read_i16(r + 0x4e) == read_i16(r + 0x76) {
                        staff_records_copy_date(r + 0x7e, hoje);
                    } else {
                        let atual = days_between(hoje, r + 0x4e);
                        if atual < days_between(r + 0x7e, r + 0x76) {
                            write_i32(r + 0x72, read_i32(r + 0x4a));
                            staff_records_copy_date(r + 0x76, r + 0x4e);
                            staff_records_copy_date(r + 0x7e, hoje);
                        }
                    }
                } else {
                    write_i32(r + 0x72, read_i32(r + 0x4a));
                    staff_records_copy_date(r + 0x76, r + 0x4e);
                    staff_records_copy_date(r + 0x7e, hoje);
                }
                if read_i32(r + 0x5e) != -1 && read_i32(r + 0x4a) != -1 {
                    let atual = days_between(hoje, r + 0x4e);
                    if atual > days_between(r + 0x6a, r + 0x62) {
                        write_i32(r + 0x5e, read_i32(r + 0x4a));
                        staff_records_copy_date(r + 0x62, r + 0x4e);
                        staff_records_copy_date(r + 0x6a, r + 0x56);
                    }
                } else {
                    write_i32(r + 0x5e, read_i32(r + 0x4a));
                    staff_records_copy_date(r + 0x62, r + 0x4e);
                    staff_records_copy_date(r + 0x6a, hoje);
                }
            }
            write_i32(r + 0x4a, -1);
            if contratado == 1 {
                crate::comparacao::ramo("cqa_12 passagem nova");
                write_i32(r + 0x4a, read_i32(clube));
                staff_records_copy_date(r + 0x4e, hoje);
                let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
                let mut t = [0u8; 8];
                let d = data(t.as_mut_ptr(), 0xd, 0xb, 0x7b7, u32::MAX);
                staff_records_copy_date(r + 0x56, d);
                *((r + 0xb4) as *mut u8) = (*((r + 0xb4) as *const u8)).wrapping_add(1); // cqa12: as passagens
            }
        }
    } else if contratado == 1 {
        let r = registro(this, tecnico, 1);
        if r != 0 {
            *((r + 0xb5) as *mut u8) = (*((r + 0xb5) as *const u8)).wrapping_add(1);
        }
    }
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x96;
    put_i32(&mut e, 8, read_i32(clube));
    put_i32(&mut e, 0xd, contratado as i32);
    append_person_history_by_id(!CUP_PLACED_CLUB_DRY_RUN, this, read_usize(tecnico), &e);
    if read_usize(clube + club::CHAIRMAN) != 0 {
        let mut e2 = history_entry();
        stamp_game_date(e2.as_mut_ptr());
        e2[0xc] = 0x97;
        put_i32(&mut e2, 8, read_i32(clube));
        put_i32(&mut e2, 0xd, contratado as i32);
        put_person_keys(&mut e2, 0x11, tecnico);
        append_person_history_by_id(!CUP_PLACED_CLUB_DRY_RUN, this, read_usize(read_usize(clube + club::CHAIRMAN)), &e2);
    }
    let mut e3 = history_entry();
    stamp_game_date(e3.as_mut_ptr());
    e3[8] = 0x96;
    put_person_keys(&mut e3, 9, tecnico);
    put_i32(&mut e3, 0x19, contratado as i32);
    append_club_history_by_id(this, read_usize(clube), &e3);
}

/// Comparer for `record_manager_job_change` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador em ensaio: o registro do técnico criado antes (0x86f5b0, como a original faria nos dois caminhos que o
/// pedem), a cópia sem os históricos e a original a partir do mesmo estado (o registro, 0xd2 bytes, e a data estática da
/// 0x54cdb0). Nulos: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_record_manager_job_change(this: usize, tecnico: usize, clube: usize, contratado: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_RECORD_MANAGER_JOB_CHANGE);
    if clube == 0 || tecnico == 0 {
        return o(this, tecnico, clube, contratado);
    }
    let mut reg = vec![(G_DAYS_SINCE_SCRATCH_DATE, 8usize)];
    if is_national_team(clube) == 0 || contratado as u8 == 1 {
        let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
        let r = registro(this, tecnico, 1);
        if r != 0 {
            reg.push((r, 0xd2));
        }
    }
    let antes = crate::comparacao::Memoria::ler(&reg);
    CUP_PLACED_CLUB_DRY_RUN = true;
    record_manager_job_change(this, tecnico, clube, contratado);
    CUP_PLACED_CLUB_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&reg);
    antes.escrever();
    o(this, tecnico, clube, contratado);
    let mo = crate::comparacao::Memoria::ler(&reg);
    PLACAR.registrar(mo == mr, || format!("record_manager_job_change(tecnico {:#x}, clube {:#x}, {}) original={:x?} rust={:x?}", tecnico, clube, contratado as u8, mo, mr));
}





/// the team of the year history of the person (type 0x32: the club, the award and two values)
/// Original: `Staff_Records::add_team_of_year_record` (0x86c730, staff_records.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x86c730 } (thiscall, ret 0x14, staff_records.cpp, SEH só da data local, de destrutor vazio):
/// `Staff_Records::add_team_of_year_record(pessoa, clube, prêmio, a, b)` (pela TEAM_AWARD::record_winners 0x8affb0):
/// pessoa ou prêmio nulos, a caixa "Error" (linha 0x7ee). Senão o histórico da pessoa (tipo 0x32: o id do clube ou −1,
/// o id do prêmio, `a` em 16 bits sem sinal e `b` como byte com sinal), no arquivo (0x58e340).
pub(crate) unsafe extern "thiscall" fn add_team_of_year_record(this: usize, pessoa: usize, clube: usize, premio: usize, a: u32, b: u32) {
    crate::comparacao::ramo("cps_10 selecao do ano");
    if pessoa == 0 || premio == 0 {
        show_error_at(0xa83b78, 0x7ee);
        return;
    }
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x32;
    put_i32(&mut e, 8, staff_records_id_or_minus_one(clube));
    put_i32(&mut e, 0xd, read_i32(premio));
    put_i32(&mut e, 0x11, (a & 0xffff) as i32);
    put_i32(&mut e, 0x15, b as u8 as i8 as i32);
    append_person_history_by_id(true, this, read_u32_as_usize(pessoa), &e);
}

// ponytail: o histórico vai para o arquivo (não roda duas vezes) e a cópia não grava mais nada: só a original; a
// cópia leva o ramo.
/// Comparer for `add_team_of_year_record` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_team_of_year_record(this: usize, pessoa: usize, clube: usize, premio: usize, a: u32, b: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32)>(IDX_ADD_TEAM_OF_YEAR_RECORD)(this, pessoa, clube, premio, a, b)
}

/// Returns the club manager's points record from G_STAFF_RECORDS (0x86f5b0, creating it), or 0 without a manager (uses
/// G_STAFF_RECORDS instead of this).
///
/// [pt] O registro de pontos do técnico do clube (+0xcf; 0x86f5b0 do [0xdd7f00], criando), ou 0 sem técnico.
pub(crate) unsafe fn club_manager_record_global(clube: usize) -> usize {
    let tecnico = read_u32_as_usize(clube + club::MANAGER);
    if tecnico == 0 {
        return 0;
    }
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
    registro(read_u32_as_usize(G_STAFF_RECORDS), tecnico, 1)
}

/// orders two clubs by their manager's league season points (highest first; no record last)
/// Original: `staff_records_compare_clubs_by_manager_points` (0x870220, staff_records.cpp, cdecl).
///
/// [pt] { 0x870220 } (cdecl, staff_records.cpp): `staff_records_compare_clubs_by_manager_points(a, b)`, a ordem do qsort dos
/// candidatos a técnico do ano (0x86fab0): os registros dos técnicos dos dois clubes (club_manager_record_global); sem os dois, 0;
/// só sem o de `a`, 1; só sem o de `b`, −1; com os dois, os pontos da temporada na liga (+0xba) de `b` menos os de
/// `a` (mais pontos primeiro).
pub(crate) unsafe extern "C" fn staff_records_compare_clubs_by_manager_points(a: usize, b: usize) -> i32 {
    let (ca, cb) = (read_u32_as_usize(a), read_u32_as_usize(b));
    let ra = club_manager_record_global(ca);
    let rb = club_manager_record_global(cb);
    if ra == 0 {
        return if rb != 0 { 1 } else { 0 };
    }
    if rb == 0 {
        return -1;
    }
    crate::comparacao::ramo("cps_10 pontos dos dois");
    read_i32(rb + 0xba).wrapping_sub(read_i32(ra + 0xba)) // cps10: mais pontos primeiro
}

// ponytail: a 0x86f5b0 cria o registro que falta; a original roda antes e a cópia só acha os que ela criou.
/// Comparer for `staff_records_compare_clubs_by_manager_points` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_records_compare_clubs_by_manager_points(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_STAFF_RECORDS_COMPARE_CLUBS_BY_MANAGER_POINTS)(a, b);
    let n = staff_records_compare_clubs_by_manager_points(a, b);
    PLACAR.registrar(r == n, || format!("staff_records_compare_clubs_by_manager_points(clubes {:x}, {:x}) original={} rust={}", read_u32_as_usize(a), read_u32_as_usize(b), r, n));
    r
}







/// Adds v to the i32 at address a.
pub(crate) unsafe fn staff_records_add_i32(a: usize, v: i32) {
    write_i32(a, read_i32(a).wrapping_add(v));
}


/// New 0x40-byte history entry on the stack with Date() and the current game date copied in.
///
/// [pt] Uma entrada de histórico na pilha: a data (0x54a110) com a data do jogo copiada (0x417c10).
/// ponytail: os bytes que a original não grava (lixo da pilha dela) saem zerados aqui; só vão para o arquivo do
/// histórico (FILE_LLIST), que a carreira não lê de volta.
pub(crate) unsafe fn dated_history_entry() -> [u8; 0x40] {
    let mut e = [0u8; 0x40];
    let nova: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let copia: unsafe extern "thiscall" fn(*mut u8, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut t = [0u8; 8];
    nova(e.as_mut_ptr());
    copia(e.as_mut_ptr(), t.as_mut_ptr(), G_CURRENT_DATE);
    e
}

/// Writes u32 v at offset o of a history entry.
pub(crate) unsafe fn put_u32(e: &mut [u8; 0x40], o: usize, v: u32) {
    e[o..o + 4].copy_from_slice(&v.to_le_bytes());
}

/// Appends entry e to the list of id (head/tail at base + id × 8) in FILE_LLIST arq; nothing when not writing.
///
/// [pt] A entrada `e` no fim da lista do arquivo `arq` (FILE_LLIST, 0x58e340) da pessoa ou do clube `id` (o começo e o fim
/// em `base` + id × 8). Sem `gravar` (a cópia pelo comparador, depois da original), nada.
pub(crate) unsafe fn append_history(gravar: bool, arq: usize, base: usize, id: usize, e: &[u8; 0x40]) {
    if !gravar {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) -> usize = core::mem::transmute(FN_FILE_LLIST_ADD_NODE);
    let p = base.wrapping_add(id.wrapping_mul(8));
    f(arq, p, p + 4, e.as_ptr());
}

/// Appends entry e to the person's history (file [this], lists [this+0xc]).
///
/// [pt] O histórico da pessoa (o arquivo [this], as listas em [this+0xc]).
pub(crate) unsafe fn append_person_history(gravar: bool, this: usize, pessoa: usize, e: &[u8; 0x40]) {
    append_history(gravar, read_usize(this), read_usize(this + 0xc), read_usize(pessoa), e);
}

/// Appends a dated club history entry with type and competition id (file [this+4], lists [this+0x10]).
///
/// [pt] O histórico do clube (o arquivo [this+4], as listas em [this+0x10]) com o tipo e a competição.
pub(crate) unsafe fn append_club_history(gravar: bool, this: usize, clube: usize, tipo: u8, comp: usize) {
    let mut e = dated_history_entry();
    e[8] = tipo;
    put_u32(&mut e, 9, read_usize(comp) as u32);
    append_history(gravar, read_usize(this + 4), read_usize(this + 0x10), read_usize(clube), &e);
}

/// Builds a dated person history entry with club id (+8), type (+0xc) and competition (+0xd).
///
/// [pt] A entrada da pessoa: o clube (+8), o tipo (+0xc) e a competição (+0xd).
pub(crate) unsafe fn person_history_entry(clube: usize, tipo: u8, comp: u32) -> [u8; 0x40] {
    let mut e = dated_history_entry();
    put_u32(&mut e, 8, read_usize(clube) as u32);
    e[0xc] = tipo;
    put_u32(&mut e, 0xd, comp);
    e
}

/// Iterates the club's 50 squad slots yielding players with a player record (+0x61), id below [0xae2364] and not the
/// manager.
///
/// [pt] Os jogadores do elenco (50 vagas em +0xd7) que contam nos registros: com registro de jogador (+0x61), id abaixo de
/// [0xae2364] e que não são o técnico.
pub(crate) unsafe fn counted_squad_players(clube: usize) -> impl Iterator<Item = usize> {
    (0..50usize).filter_map(move |k| {
        let p = read_usize(clube + club::SQUAD + 4 * k);
        if p == 0 || read_usize(p + 0x61) == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) || p == read_usize(clube + club::MANAGER) {
            None
        } else {
            Some(p)
        }
    })
}


/// Read-only: true if 0x4cda70 would run, i.e. transfer manager exists, contract older than 300 days (+0x25) and the
/// player is on the club shortlist.
///
/// [pt] O contrato `c` do jogador `p` vai a renovação (a 0x4cda70 roda): com o gerente de transferências, o contrato com
/// mais de 300 dias (desde +0x25) e o jogador na lista do clube (0x849ce0, item +0xf = id dele). Só leitura.
pub(crate) unsafe fn will_renew_contract(clube: usize, p: usize, c: usize) -> bool {
    let gt = read_usize(G_TRANSFER_MANAGER);
    if gt == 0 {
        return false;
    }
    let dias: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    if dias(G_CURRENT_DATE, read_usize(c + 0x25), read_usize(c + 0x29)) <= 0x12c {
        return false;
    }
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    let e = entrada(read_usize(gt + 0x856), clube, p, 0, 0);
    e != 0 && read_usize(e + 0xf) == read_usize(p)
}

/// Offers a contract renewal (0x4cda70 on G_CONTRACT_MANAGER: player, no club or terms, slot −1, factor 1.0, mode 1).
///
/// [pt] A oferta de renovação do contrato (0x4cda70 do gerente de contratos: o jogador, sem clube nem termos, vaga −1,
/// fator 1,0, modo 1).
pub(crate) unsafe fn offer_renewal(p: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, i32, u32, u32, u32) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_OFFER_CONTRACT);
    f(read_usize(G_CONTRACT_MANAGER), p, 0, 0, -1, 0, 0x3ff0_0000, 1);
}

/// Adjusts the person's morale by delta (FN_STAFF_ADJUST_MORALE with 1).
pub(crate) unsafe fn adjust_morale(p: usize, delta: i32) {
    let f: unsafe extern "C" fn(usize, i32, u32) -> u32 = core::mem::transmute(FN_STAFF_ADJUST_MORALE);
    f(p, delta, 1);
}

/// Raises a non-player rating i16 by ftol(x × weight × [0x96ff88]), floored at x × weight and capped at 10000.
///
/// [pt] A nota do não jogador em `a` sobe ftol(x × peso × 10/61) e fica no máximo entre x × peso (16 bits) e ela mesma,
/// presa a 10000.
pub(crate) unsafe fn raise_promotion_rating(a: usize, x: i32, peso: i32) {
    write_i16(a, read_i16(a).wrapping_add(match_pl_ftol(x.wrapping_mul(peso) as f64 * read_f64(0x96ff88)) as i16));
    let minimo = (x as i16).wrapping_mul(peso as i16);
    let v = read_i16(a);
    write_i16(a, if minimo > v { minimo } else if v > 0x2710 { 0x2710 } else { v });
}

/// Promotion of a club: reputation +500, max attendance, manager ratings, histories for manager, chairman and squad,
/// squad reputations and morale, renewals; no file histories when not writing.
///
/// [pt] O acesso; `gravar` falso só na cópia pelo comparador (sem os históricos em arquivo).
pub(crate) unsafe fn apply_promotion(this: usize, clube: usize, comp: usize, peso: u32, gravar: bool) {
    if clube == 0 || comp == 0 {
        show_error_at(0xa83b78, 0x301);
        return;
    }
    let e = person_history_entry(clube, 8, read_usize(comp) as u32);
    write_i16(clube + club::REPUTATION, read_i16(clube + club::REPUTATION).wrapping_add(500));
    if read_i16(clube + club::REPUTATION) > 0x2710 {
        write_i16(clube + club::REPUTATION, 0x2710);
    }
    let v = read_i32(clube + club::MAX_ATTENDANCE);
    if v < 0x249f0 && v < read_i32(clube + club::ATTENDANCE).wrapping_add(500) {
        write_i32(clube + club::MAX_ATTENDANCE, v.wrapping_add(read_i16(comp + competition::CLUB_COMP_REPUTATION) as i32 * 10));
    }
    if read_usize(clube + club::MANAGER) != 0 {
        let tec = read_usize(clube + club::MANAGER);
        if read_usize(tec + 0x69) != 0 {
            let objeto = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(comp).wrapping_mul(4)));
            let mut pela = comp;
            if objeto != 0 && read_i32(objeto + 0x1c) > -1 {
                let cc = read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_i32(objeto + 0x1c) as usize).wrapping_mul(0x6b));
                pela = if cc != 0 { cc } else { comp };
            }
            let (p1, p2) = if read_u8(objeto + 0x42) as i8 != 1 {
                (400, 400)
            } else {
                let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
                (200, if estrangeira(tec, read_usize(clube + club::NATION), 1) != 0 { 200 } else { 400 })
            };
            let x = read_i16(pela + 0x69) as i32;
            let nj = || read_usize(read_usize(clube + club::MANAGER) + 0x69);
            crate::comparacao::ramo("cpq_3 acesso do tecnico");
            raise_promotion_rating(nj() + 0xa, x, 400);
            raise_promotion_rating(nj() + 8, x, p2);
            raise_promotion_rating(nj() + 0xc, x, p1);
        }
        append_person_history(gravar, this, read_usize(clube + club::MANAGER), &e);
        let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
        let r = registro(this, read_usize(clube + club::MANAGER), 1);
        if r != 0 {
            let pontos = (read_i16(comp + competition::CLUB_COMP_REPUTATION) as i32).wrapping_mul(peso as u8 as i8 as i32).wrapping_mul(2500) / 100;
            staff_records_add_i32(r + 0xb6, pontos);
            staff_records_add_i32(r + 0xba, pontos);
            staff_records_add_i32(r + 0xca, pontos);
        }
    }
    if read_usize(clube + club::CHAIRMAN) != 0 {
        append_person_history(gravar, this, read_usize(clube + club::CHAIRMAN), &e);
    }
    for p in counted_squad_players(clube) {
        adjust_morale(p, 0xf);
        let c = game_contract_for_club(p, clube);
        if c == 0 {
            continue;
        }
        let v = read_u8(c + 0x44) as i8;
        let s = v as i32 + 0x4b;
        *((c + 0x44) as *mut u8) = if s > 0x64 { 0x64 } else if s < -0x64 { 0x9c } else { v.wrapping_add(0x4b) as u8 };
        let tipo = read_u8(c + 0x4f) >> 4;
        if (tipo == 0 || (tipo > 3 && tipo != 5)) || !will_renew_contract(clube, p, c) {
            continue;
        }
        crate::comparacao::ramo("cpq_3 acesso renova");
        offer_renewal(p);
    }
    append_club_history(gravar, this, clube, 8, comp);
}

/// club reputation +500 (up to 10000) and +0x7b; the manager ratings by the competition importance (× 10/61, with a
/// minimum); the manager, chairman and club histories (type 8); the manager points; squad morale +15 and bond +75 and
/// renewal of the expiring contracts
/// Original: `Staff_Records::add_promotion_record` (0x869480, staff_records.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x869480 } (thiscall, ret 0xc, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_promotion_record(clube,
/// comp, peso)`: a reputação do clube (+0x80) sobe 500 (até 10000) e +0x7b sobe 10 × a importância da competição
/// (+0x69) se está abaixo de 150000 e de +0x73 + 500. O técnico (+0xcf) com não jogador: as notas +0xa, +8 e +0xc sobem
/// pela importância da competição (a do registro de competição de clube +0x1c dela, se há) × 400, × 200/400 (estrangeiro
/// na liga com +0x42 = 1, 0x53d980) e × 200/400 (liga com +0x42 = 1) × 10/61, com o mínimo de importância × peso. O
/// histórico do técnico e do presidente (tipo 8), os pontos do técnico (0x86f5b0: +0xb6, +0xba, +0xca somam importância
/// × peso × 25), o moral +15 de cada jogador do elenco (0x544710), o vínculo do contrato (+0x44) +75 (−100..100) e a
/// renovação (0x4cda70) dos contratos dos tipos 1, 2, 3 e 5 que vencem; por fim o histórico do clube (tipo 8). Clube ou
/// competição nulos: a caixa "Error" (linha 0x301).
pub(crate) unsafe extern "thiscall" fn add_promotion_record(this: usize, clube: usize, comp: usize, peso: u32) {
    crate::comparacao::ramo("cpq_3 acesso");
    apply_promotion(this, clube, comp, peso, true)
}

/// Memory regions touched by promotion or relegation (RNG state, club, manager records, squad reputations, morale,
/// contract links); None if any contract would be renewed.
///
/// [pt] As regiões que o acesso ou o rebaixamento mexem (sem os históricos): o estado do sorteador (o moral sorteia), o clube,
/// o não jogador do técnico, o registro de pontos dele (criado antes, como a original faria), o moral (+0x45) e as
/// reputações (+9..+0xe) de cada jogador do elenco e o vínculo (+0x44) do contrato dele. `None` se algum contrato vai a
/// renovação (a 0x4cda70 faz a oferta: não roda duas vezes; no acesso os tipos 1, 2, 3 e 5, no rebaixamento 1 e 2).
pub(crate) unsafe fn promotion_regions(this: usize, clube: usize, acesso: bool) -> Option<Vec<(usize, usize)>> {
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((clube, 0x245));
    let tec = read_usize(clube + club::MANAGER);
    if tec != 0 {
        if read_usize(tec + 0x69) != 0 {
            reg.push((read_usize(tec + 0x69), 0x10));
        }
        if acesso {
            let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
            let r = registro(this, tec, 1);
            if r != 0 {
                reg.push((r, 0xd2));
            }
        }
    }
    for p in counted_squad_players(clube) {
        let j = read_usize(p + 0x61);
        reg.extend([(j + 9, 6), (j + 0x45, 1)]);
        let c = game_contract_for_club(p, clube);
        if c != 0 {
            let tipo = read_u8(c + 0x4f) >> 4;
            let renova = if acesso { tipo != 0 && (tipo <= 3 || tipo == 5) } else { tipo == 1 || tipo == 2 };
            if renova && will_renew_contract(clube, p, c) {
                return None;
            }
            reg.push((c + 0x44, 1));
        }
    }
    Some(reg)
}

/// Comparer for `add_promotion_record` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelas regiões (a cópia sem os históricos em arquivo, que a original já gravou, e sem contar
/// os sorteios); com renovação de contrato ou clube/competição nulos, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_add_promotion_record(this: usize, clube: usize, comp: usize, peso: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_ADD_PROMOTION_RECORD);
    let reg = if clube != 0 && comp != 0 { promotion_regions(this, clube, true) } else { None };
    let reg = match reg {
        Some(r) => r,
        None => return o(this, clube, comp, peso),
    };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, comp, peso),
        || crate::aleatorio::sem_contar(|| apply_promotion(this, clube, comp, peso, false)));
    PLACAR.registrar(igual, || format!("add_promotion_record(clube {:x}, peso {}) {:x?} x {:x?}", clube, peso as u8 as i8, mo, mr));
}

/// Adds v to a non-player rating i16, clamped to 1..10000.
///
/// [pt] A nota do não jogador em `a` soma `v` (16 bits) e fica presa a 1..10000.
pub(crate) unsafe fn add_rating_clamped(a: usize, v: i16) {
    write_i16(a, read_i16(a).wrapping_add(v));
    let n = read_i16(a);
    write_i16(a, if n < 1 { 1 } else if n > 0x2710 { 0x2710 } else { n });
}

/// Multiplies the player record reputations (+0xb current, +9 home, +0xd world) by k.
///
/// [pt] As reputações (+0xb atual, +9 na nação, +0xd mundial) do registro de jogador `j` × `k`.
pub(crate) unsafe fn scale_player_reputations(j: usize, k: f64) {
    for o in [0xb, 9, 0xd] {
        write_i16(j + o, match_pl_ftol(read_i16(j + o) as i32 as f64 * k) as i16);
    }
}

/// Relegation of a club: reputation −500 (min 750), manager ratings down, histories for manager, chairman and squad,
/// squad reputations and morale.
pub(crate) unsafe fn apply_relegation(this: usize, clube: usize, comp: usize, gravar: bool) {
    if clube == 0 || comp == 0 {
        show_error_at(0xa83b78, 0x476);
        return;
    }
    let e = person_history_entry(clube, 9, read_usize(comp) as u32);
    write_i16(clube + club::REPUTATION, read_i16(clube + club::REPUTATION).wrapping_sub(500));
    if read_i16(clube + club::REPUTATION) < 0x2ee {
        write_i16(clube + club::REPUTATION, 0x2ee);
    }
    if read_usize(clube + club::MANAGER) != 0 {
        let tec = read_usize(clube + club::MANAGER);
        if read_usize(tec + 0x69) != 0 {
            let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
            let fator: i16 = if estrangeira(tec, read_usize(clube + club::NATION), 1) != 0 { -8 } else { -16 };
            let nj = || read_usize(read_usize(clube + club::MANAGER) + 0x69);
            let x = || read_i16(comp + competition::CLUB_COMP_REPUTATION);
            crate::comparacao::ramo("cpq_3 rebaixamento do tecnico");
            add_rating_clamped(nj() + 0xa, x().wrapping_mul(-16));
            add_rating_clamped(nj() + 8, x().wrapping_mul(fator));
            add_rating_clamped(nj() + 0xc, x().wrapping_mul(-8));
        }
        append_person_history(gravar, this, read_usize(clube + club::MANAGER), &e);
    }
    if read_usize(clube + club::CHAIRMAN) != 0 {
        append_person_history(gravar, this, read_usize(clube + club::CHAIRMAN), &e);
    }
    for p in counted_squad_players(clube) {
        adjust_morale(p, -0xf);
        let c = game_contract_for_club(p, clube);
        if c == 0 {
            continue;
        }
        let j = read_usize(p + 0x61);
        match read_u8(c + 0x4f) >> 4 {
            1 | 2 => {
                if j != 0 {
                    scale_player_reputations(j, read_f64(0x9689d8)); // 0,9
                }
                if will_renew_contract(clube, p, c) {
                    crate::comparacao::ramo("cpq_3 rebaixamento renova");
                    offer_renewal(p);
                }
            }
            3..=5 => {
                if j != 0 {
                    scale_player_reputations(j, read_f64(0x9689d0)); // 0,95
                }
            }
            _ => {}
        }
    }
    append_club_history(gravar, this, clube, 9, comp);
}

/// club reputation −500 (minimum 750); the manager ratings drop by the competition importance (1..10000); the manager,
/// chairman and club histories (type 9); squad morale −15, reputations × 0.9 (contracts 1 and 2, renewing the expiring
/// ones) or × 0.95 (3 to 5)
/// Original: `Staff_Records::add_relegation_record` (0x86a1c0, staff_records.cpp, thiscall, ret 8).
///
/// [pt] { 0x86a1c0 } (thiscall, ret 8, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_relegation_record(clube,
/// comp)`: a reputação do clube (+0x80) cai 500 (mínimo 750). O técnico com não jogador: as notas +0xa (−16), +8 (−8
/// estrangeiro, 0x53d980, ou −16) e +0xc (−8) × a importância da competição (+0x69), presas a 1..10000; o histórico do
/// técnico e do presidente (tipo 9). Cada jogador do elenco: moral −15 (0x544710) e, pelo tipo do contrato (+0x4f >> 4):
/// 1 e 2, reputações × 0,9 e a renovação (0x4cda70) se vence; 3 a 5, reputações × 0,95. Por fim o histórico do clube
/// (tipo 9). Clube ou competição nulos: a caixa "Error" (linha 0x476).
pub(crate) unsafe extern "thiscall" fn add_relegation_record(this: usize, clube: usize, comp: usize) {
    crate::comparacao::ramo("cpq_3 rebaixamento");
    apply_relegation(this, clube, comp, true)
}

/// Comparer for `add_relegation_record` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: como o do acesso (sem o registro de pontos, que o rebaixamento não usa).
pub(crate) unsafe extern "thiscall" fn comparar_add_relegation_record(this: usize, clube: usize, comp: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_ADD_RELEGATION_RECORD);
    let reg = if clube != 0 && comp != 0 { promotion_regions(this, clube, false) } else { None };
    let reg = match reg {
        Some(r) => r,
        None => return o(this, clube, comp),
    };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, comp),
        || crate::aleatorio::sem_contar(|| apply_relegation(this, clube, comp, false)));
    PLACAR.registrar(igual, || format!("add_relegation_record(clube {:x}) {:x?} x {:x?}", clube, mo, mr));
}

/// Qualification of a club: person history entry (type 3, stage, base round) for the manager and adds reputation² ×
/// weight points to his record (national team or club totals).
pub(crate) unsafe fn record_qualification(this: usize, clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32, gravar: bool) {
    if clube == 0 {
        show_error_at(0xa83b78, 0x703);
        return;
    }
    let mut e = person_history_entry(clube, 3, if comp != 0 { read_usize(comp) as u32 } else { u32::MAX });
    put_u32(&mut e, 0x11, fase as u16 as i16 as i32 as u32);
    let base: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_COMP_UTIL_GET_BASE_ROUND_ID);
    put_u32(&mut e, 0x15, base(rodada) as u16 as i16 as i32 as u32);
    if read_usize(clube + club::MANAGER) == 0 {
        return;
    }
    append_person_history(gravar, this, read_usize(clube + club::MANAGER), &e);
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
    let r = registro(this, read_usize(clube + club::MANAGER), 1);
    if r == 0 {
        return;
    }
    // ponytail: a original faz pow(x, 2,0) (0x945ff0): x × x, exato para a importância (16 bits)
    let x = read_i16(comp + competition::CLUB_COMP_REPUTATION) as f64;
    let pontos = match_pl_ftol(x * x * (peso as u8 as i8 as i32) as f64);
    staff_records_add_i32(r + 0xb6, pontos);
    staff_records_add_i32(r + 0xca, pontos);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if selecao(clube) != 0 {
        crate::comparacao::ramo("cpq_3 classificado selecao");
        staff_records_add_i32(r + 0xbe, pontos);
    } else {
        crate::comparacao::ramo("cpq_3 classificado clube");
        staff_records_add_i32(r + 0xba, pontos);
    }
}

/// the manager history (type 3, with the round and the base round) and his points: importance² × weight in the year,
/// the month and the cup (national team) or the league (club)
/// Original: `Staff_Records::add_qualified_for_round_record` (0x86bdd0, staff_records.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x86bdd0 } (thiscall, ret 0x14, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_qualified_for_round_record(
/// clube, comp, fase, rodada, peso)`: com técnico (+0xcf), o histórico dele (tipo 3: clube, competição ou −1, a fase com
/// sinal e a rodada base 0x4b0870) e os pontos (0x86f5b0): importância² × peso somados em +0xb6, +0xca e, na seleção
/// (0x53b900), +0xbe, ou no clube +0xba. Clube nulo: a caixa "Error" (linha 0x703).
pub(crate) unsafe extern "thiscall" fn add_qualified_for_round_record(this: usize, clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32) {
    record_qualification(this, clube, comp, fase, rodada, peso, true)
}

/// Comparer for `add_qualified_for_round_record` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo registro de pontos do técnico (criado antes, como a original faria), a cópia sem o
/// histórico em arquivo; sem clube ou sem técnico, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_add_qualified_for_round_record(this: usize, clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(IDX_ADD_QUALIFIED_FOR_ROUND_RECORD);
    if clube == 0 || read_usize(clube + club::MANAGER) == 0 {
        return o(this, clube, comp, fase, rodada, peso);
    }
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
    let r = registro(this, read_usize(clube + club::MANAGER), 1);
    if r == 0 {
        return o(this, clube, comp, fase, rodada, peso);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(r, 0xd2)], || o(this, clube, comp, fase, rodada, peso),
        || record_qualification(this, clube, comp, fase, rodada, peso, false));
    PLACAR.registrar(igual, || format!("add_qualified_for_round_record(clube {:x}, peso {}) {:x?} x {:x?}", clube, peso as u8 as i8, mo, mr));
}







/// Adds sign-extended v to the i64 at address a.
///
/// [pt] O inteiro de 64 bits em `a` somado de `v` (estendido com sinal: cdq, add, adc).
pub(crate) unsafe fn staff_records_add_i64(a: usize, v: i32) {
    let x = core::ptr::read_unaligned(a as *const i64);
    core::ptr::write_unaligned(a as *mut i64, x.wrapping_add(v as i64));
}

/// Writes i32 v at offset o of a history entry.
pub(crate) unsafe fn put_i32(e: &mut [u8; 0x40], o: usize, v: i32) {
    e[o..o + 4].copy_from_slice(&v.to_le_bytes());
}


/// Copies the current game date (0xae2c90) into the date at a.
///
/// [pt] A data do jogo copiada (0x417c10) em `a` (a cópia também vai para uma data local).
pub(crate) unsafe fn stamp_game_date(a: *mut u8) {
    let copia: unsafe extern "thiscall" fn(*mut u8, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut t = [0u8; 8];
    copia(a, t.as_mut_ptr(), G_CURRENT_DATE);
}

/// New 0x40-byte history entry on the stack with a constructed Date (0x54a110).
///
/// [pt] Uma entrada de histórico na pilha com a data construída (0x54a110).
/// ponytail: os bytes que a original não grava (lixo da pilha dela) saem zerados aqui; só vão para o arquivo do
/// histórico (FILE_LLIST), que a carreira não lê de volta.
pub(crate) unsafe fn history_entry() -> [u8; 0x40] {
    let mut e = [0u8; 0x40];
    let nova: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    nova(e.as_mut_ptr());
    e
}

/// Appends entry e to person id's list in the person history FILE_LLIST [this] (heads at [this+0xc]); nothing when not
/// writing (takes the person id and this directly).
///
/// [pt] A entrada `e` no fim da lista da pessoa `id` no arquivo do histórico das pessoas (FILE_LLIST [this], 0x58e340; o
/// começo e o fim em [this+0xc] + id × 8). Sem `gravar` (a cópia pelo comparador), nada.
pub(crate) unsafe fn append_person_history_by_id(gravar: bool, this: usize, id: usize, e: &[u8; 0x40]) {
    if !gravar {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) -> usize = core::mem::transmute(FN_FILE_LLIST_ADD_NODE);
    let p = read_usize(this + 0xc).wrapping_add(id.wrapping_mul(8));
    f(read_usize(this), p, p + 4, e.as_ptr());
}

/// Returns the id of the person's employing club (staff::CLUB_JOB), or −1.
///
/// [pt] O id do clube contratante da pessoa (+0x39), ou −1.
pub(crate) unsafe fn club_job_id(pessoa: usize) -> i32 {
    let c = read_usize(pessoa + staff::CLUB_JOB);
    if c == 0 {
        -1
    } else {
        read_i32(c)
    }
}

/// Returns the record's id, or −1 for a null pointer.
pub(crate) unsafe fn staff_records_id_or_minus_one(p: usize) -> i32 {
    if p == 0 {
        -1
    } else {
        read_i32(p)
    }
}

/// Writes the person's id and the +0x33 ids of its +4, +8, +0xc pointers into entry e from offset o.
///
/// [pt] O id da pessoa e os de +4, +8 e +0xc dela (+0x33 de cada) em `e` a partir de `o`.
pub(crate) unsafe fn put_person_keys(e: &mut [u8; 0x40], o: usize, p: usize) {
    put_i32(e, o, read_i32(p));
    for k in 0..3usize {
        put_i32(e, o + 4 + 4 * k, read_i32(read_usize(p + 4 + 4 * k) + 0x33));
    }
}

/// Calls f for each item of the transfer's swap list (LLIST at +4), resetting the cursor first.
///
/// [pt] Os itens da lista da troca (LLIST em +4 da transferência, relida a cada passo; o cursor reposto antes), na ordem.
pub(crate) unsafe fn for_each_swap_item(t: usize, mut f: impl FnMut(usize)) {
    let l = read_usize(t + 4);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut u32, read_usize(l + 2) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut e = proximo(read_usize(t + 4));
    while e != 0 {
        f(e);
        e = proximo(read_usize(t + 4));
    }
}

/// True if the job type is a player job (0x53b620: 11, 12, 13 or 15).
///
/// [pt] O tipo do cargo é de jogador (0x53b620: 11, 12, 13 ou 15)?
pub(crate) unsafe fn staff_records_is_player_job(cargo: u8) -> bool {
    let f: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    f(cargo as u32) != 0 // ponytail: a original empurra o registro com o byte no al; a 0x53b620 lê só o byte
}

/// Returns the manager's points record (FN_STAFF_RECORDS_GET_MANAGER_RECORD with create).
pub(crate) unsafe fn staff_records_manager_record(this: usize, tec: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
    f(this, tec, 1)
}

/// Sides of a transfer: (person, his contract, selling club or 0, buying club).
///
/// [pt] Os lados da transferência: (pessoa, contrato da pessoa 0x4cf5a0, vendedor ou 0, comprador).
pub(crate) unsafe fn transfer_parties(t: usize) -> (usize, usize, usize, usize) {
    let pessoa = staff_records_staff_by_id(read_usize(t + 0xc));
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato(read_usize(G_CONTRACT_MANAGER), pessoa);
    let vendedor = if c != 0 && read_u8(c + 0x35) & 0x3f != 5 {
        read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4).wrapping_mul(club::SIZE))
    } else {
        0
    };
    (pessoa, c, vendedor, read_usize(G_CLUB_TABLE).wrapping_add(read_usize(t + 8).wrapping_mul(club::SIZE)))
}

/// Records a transfer in the staff records: biggest purchase/sale and totals in the buying and selling managers'
/// records, plus history entries for the player and clubs; no file histories when not writing.
pub(crate) unsafe fn record_transfer(this: usize, t: usize, gravar: bool) {
    let (pessoa, c, vendedor, comprador) = transfer_parties(t);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let mut e = history_entry();
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_TOTAL_VALUE);
    let valor = total(t, 1, 0, 0, 1);
    let com_troca = || (read_usize(t + 4) != 0) as i32;
    let tipo = read_u8(t + 0x11) as i8 as i32;
    if comprador != 0 && read_usize(comprador + 0xcf) != 0 {
        let tec = read_usize(comprador + 0xcf);
        let r = if staff_records_is_player_job(read_u8(t + 0x11)) { staff_records_manager_record(this, tec) } else { 0 };
        if r != 0 {
            crate::comparacao::ramo("cpq_4 compra");
            if valor > read_i32(r + 0x18) {
                crate::comparacao::ramo("cpq_4 maior compra");
                for k in 0..4usize {
                    write_i32(r + 4 * k, if k == 0 { read_i32(pessoa) } else { read_i32(read_usize(pessoa + 4 * k) + 0x33) });
                }
                stamp_game_date((r + 0x10) as *mut u8);
                write_i32(r + 0x18, valor);
                *((r + 0x1c) as *mut u8) = com_troca() as u8;
                write_i32(r + 0x1d, staff_records_id_or_minus_one(vendedor));
                write_i32(r + 0x21, read_i32(comprador));
            }
            write_i16(r + 0xa0, read_i16(r + 0xa0).wrapping_add(1));
            staff_records_add_i64(r + 0xa4, valor);
            if read_usize(t + 4) != 0 {
                write_i16(r + 0xa2, read_i16(r + 0xa2).wrapping_add(read_i16(read_usize(t + 4))));
                for_each_swap_item(t, |x| staff_records_add_i64(r + 0xac, read_i32(staff_records_staff_by_id(read_usize(x)) + 0x52)));
            }
        }
        // o histórico do técnico comprador: a compra e cada jogador dado na troca
        stamp_game_date(e.as_mut_ptr());
        e[0xc] = 0x64;
        put_i32(&mut e, 8, club_job_id(tec));
        put_person_keys(&mut e, 0xd, pessoa);
        put_i32(&mut e, 0x1d, valor);
        put_i32(&mut e, 0x21, com_troca());
        put_i32(&mut e, 0x25, staff_records_id_or_minus_one(vendedor));
        put_i32(&mut e, 0x29, tipo);
        append_person_history_by_id(gravar, this, read_usize(tec), &e);
        for_each_swap_item(t, |x| {
            e[0xc] = 0x65;
            put_person_keys(&mut e, 0xd, staff_records_staff_by_id(read_usize(x)));
            put_i32(&mut e, 0x1d, 0);
            put_i32(&mut e, 0x21, 1);
            put_i32(&mut e, 0x25, read_i32(vendedor));
            put_i32(&mut e, 0x29, tipo);
            append_person_history_by_id(gravar, this, read_usize(tec), &e);
        });
    }
    if vendedor != 0 && read_usize(vendedor + 0xcf) != 0 {
        let tec = read_usize(vendedor + 0xcf);
        let r = if staff_records_is_player_job(read_u8(pessoa + staff::JOB_FOR_CLUB)) { staff_records_manager_record(this, tec) } else { 0 };
        if r != 0 {
            crate::comparacao::ramo("cpq_4 venda");
            if valor > read_i32(r + 0x3d) {
                for k in 0..4usize {
                    write_i32(r + 0x25 + 4 * k, if k == 0 { read_i32(pessoa) } else { read_i32(read_usize(pessoa + 4 * k) + 0x33) });
                }
                stamp_game_date((r + 0x35) as *mut u8);
                write_i32(r + 0x3d, valor);
                *((r + 0x41) as *mut u8) = com_troca() as u8;
                write_i32(r + 0x42, read_i32(vendedor));
                write_i32(r + 0x46, staff_records_id_or_minus_one(comprador));
            }
            write_i16(r + 0xa2, read_i16(r + 0xa2).wrapping_add(1));
            staff_records_add_i64(r + 0xac, valor);
            if read_usize(t + 4) != 0 {
                write_i16(r + 0xa0, read_i16(r + 0xa0).wrapping_add(read_i16(read_usize(t + 4))));
                for_each_swap_item(t, |x| staff_records_add_i64(r + 0xa4, read_i32(staff_records_staff_by_id(read_usize(x)) + 0x52)));
            }
        }
        // o histórico do técnico vendedor: a venda e cada jogador recebido na troca
        stamp_game_date(e.as_mut_ptr());
        e[0xc] = 0x65;
        put_i32(&mut e, 8, club_job_id(tec));
        put_person_keys(&mut e, 0xd, pessoa);
        put_i32(&mut e, 0x1d, valor);
        put_i32(&mut e, 0x21, com_troca());
        put_i32(&mut e, 0x25, staff_records_id_or_minus_one(comprador));
        put_i32(&mut e, 0x29, tipo);
        append_person_history_by_id(gravar, this, read_usize(tec), &e);
        for_each_swap_item(t, |x| {
            e[0xc] = 0x64;
            put_person_keys(&mut e, 0xd, staff_records_staff_by_id(read_usize(x)));
            put_i32(&mut e, 0x1d, 0);
            put_i32(&mut e, 0x21, 1);
            put_i32(&mut e, 0x25, read_i32(comprador));
            put_i32(&mut e, 0x29, 0xb);
            append_person_history_by_id(gravar, this, read_usize(tec), &e);
        });
    }
    // o histórico do jogador transferido e o de cada jogador da troca
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x66;
    put_i32(&mut e, 8, club_job_id(pessoa));
    put_i32(&mut e, 0xd, staff_records_id_or_minus_one(comprador));
    put_i32(&mut e, 0x11, valor);
    put_i32(&mut e, 0x19, tipo);
    put_i32(&mut e, 0x15, com_troca());
    if pessoa != 0 {
        append_person_history_by_id(gravar, this, read_usize(pessoa), &e);
    } else {
        show_error_at(0xa83b78, 0x1010);
    }
    for_each_swap_item(t, |x| {
        let q = staff_records_staff_by_id(read_usize(x));
        put_i32(&mut e, 8, read_i32(read_usize(q + 0x39)));
        put_i32(&mut e, 0x11, 0);
        put_i32(&mut e, 0xd, read_i32(vendedor));
        put_i32(&mut e, 0x15, 1);
        put_i32(&mut e, 0x19, read_u8(x + 6) as i8 as i32);
        if q != 0 {
            append_person_history_by_id(gravar, this, read_usize(q), &e);
        } else {
            show_error_at(0xa83b78, 0x1010);
        }
    });
}

/// in the buying and selling managers' records: the biggest signing or sale, the counts, the 64-bit totals and the
/// exchange players; the buy (0x64), sale (0x65) and transfer (0x66) histories of the managers, the person and the
/// exchange players
/// Original: `Staff_Records::add_transfer_records` (0x86cd70, staff_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x86cd70 } (thiscall, ret 4, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_transfer_records(transf)`:
/// o contrato da pessoa (0x4cf5a0) vai para [0xae1080] e dá o vendedor (fora do estado 5); o valor total da oferta
/// (0x8e7a70 com 1, 0, 0, 1). O técnico do comprador, sendo de jogador o tipo da oferta (+0x11, 0x53b620): no registro
/// (0x86f5b0) a maior compra (+0..+0x24 se o valor passa +0x18), as compras (+0xa0), o gasto (+0xa4, 64 bits), os
/// jogadores dados (+0xa2) e o valor deles (+0x52 de cada, em +0xac). O técnico do vendedor, sendo de jogador o cargo
/// da pessoa (+0x3d): a maior venda (+0x25..+0x49 se passa +0x3d), as vendas (+0xa2), a receita (+0xac), os recebidos
/// (+0xa0) e o valor deles (+0xa4). Os históricos (tipos 0x64 compra, 0x65 venda, 0x66 transferência) do técnico do
/// comprador, do vendedor, da pessoa e dos jogadores da troca, no arquivo (0x58e340).
pub(crate) unsafe extern "thiscall" fn add_transfer_records(this: usize, t: usize) {
    record_transfer(this, t, true)
}

/// Comparer for `add_transfer_records` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela global [0xae1080], o estado do sorteador (o valor da oferta pode pedir a avaliação), a
/// lista da troca (o cursor) e os registros dos dois técnicos (criados antes só quando a original os pede); a cópia
/// sem os históricos em arquivo, que a original já gravou, e sem contar os sorteios.
pub(crate) unsafe extern "thiscall" fn comparar_add_transfer_records(this: usize, t: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_TRANSFER_RECORDS);
    let (pessoa, _, vendedor, comprador) = transfer_parties(t);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((G_CONTRACT_LOOKUP_RESULT, 4));
    if read_usize(t + 4) != 0 {
        reg.push((read_usize(t + 4), 0xe));
    }
    for (clube, cargo) in [(comprador, read_u8(t + 0x11)), (vendedor, read_u8(pessoa + staff::JOB_FOR_CLUB))] {
        if clube != 0 && read_usize(clube + club::MANAGER) != 0 && staff_records_is_player_job(cargo) {
            let r = staff_records_manager_record(this, read_usize(clube + club::MANAGER));
            if r != 0 {
                reg.push((r, 0xd2));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, t),
        || crate::aleatorio::sem_contar(|| record_transfer(this, t, false)));
    PLACAR.registrar(igual, || format!("add_transfer_records(pessoa {:x}) {:x?} x {:x?}", pessoa, mo, mr));
}

/// Builds a dated person history entry with club id (+8), type (+0xc), competition id or −1 (+0xd) and the signed stage
/// (+0x11).
///
/// [pt] A entrada da pessoa: o clube (+8), o tipo (+0xc), a competição ou −1 (+0xd) e a fase com sinal (+0x11).
pub(crate) unsafe fn stage_history_entry(clube: usize, tipo: u8, comp: usize, fase: u32) -> [u8; 0x40] {
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    put_i32(&mut e, 8, read_i32(clube));
    e[0xc] = tipo;
    put_i32(&mut e, 0xd, staff_records_id_or_minus_one(comp));
    put_i32(&mut e, 0x11, fase as u16 as i16 as i32);
    e
}

/// If the competition is the stored national-team finals competition (0x76d940 on G_NATIONAL_TEAMS), ends the nation's
/// call-ups for it (0x774f90).
///
/// [pt] Na competição de seleção guardada (0x76d940 de [0xb74318]), as convocações da seleção para ela acabam (0x774f90).
pub(crate) unsafe fn end_national_duty(clube: usize, comp: usize) {
    let especial: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NATIONAL_TEAMS_IS_FINALS_COMP);
    if especial(read_usize(G_NATIONAL_TEAMS), comp) != 0 {
        crate::comparacao::ramo("cpq_4 fim das convocacoes");
        let encerrar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_END_DUTY_FOR_NATION_IN_COMPETITION);
        encerrar(read_usize(G_NATIONAL_TEAMS), clube, comp);
    }
}

/// the manager history (type 4, with the round and the base round) and the end of the national team duty in the stored
/// competition
/// Original: `Staff_Records::add_knocked_out_record` (0x86c000, staff_records.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x86c000 } (thiscall, ret 0x14, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_knocked_out_record(clube,
/// comp, fase, rodada, _)`: o histórico do técnico (+0xcf; tipo 4: clube, competição ou −1, a fase com sinal e a rodada
/// base 0x4b0870) e o fim das convocações da seleção na competição guardada. Clube nulo: a caixa "Error" (linha 0x73b).
pub(crate) unsafe extern "thiscall" fn add_knocked_out_record(this: usize, clube: usize, comp: usize, fase: u32, rodada: u32, _x: u32) {
    if clube == 0 {
        return show_error_at(0xa83b78, 0x73b);
    }
    crate::comparacao::ramo("cpq_4 eliminado");
    let mut e = stage_history_entry(clube, 4, comp, fase);
    let base: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_COMP_UTIL_GET_BASE_ROUND_ID);
    put_i32(&mut e, 0x15, base(rodada) as u16 as i16 as i32);
    if read_usize(clube + club::MANAGER) != 0 {
        append_person_history_by_id(true, this, read_usize(read_usize(clube + club::MANAGER)), &e);
    }
    end_national_duty(clube, comp);
}

// ponytail: o histórico vai para o arquivo e as convocações acabam (não rodam duas vezes) e a cópia sem eles não mexe
// em nada: só a original; a cópia leva os ramos.
/// Comparer for `add_knocked_out_record` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_knocked_out_record(this: usize, clube: usize, comp: usize, fase: u32, rodada: u32, x: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(IDX_ADD_KNOCKED_OUT_RECORD)(this, clube, comp, fase, rodada, x)
}

/// the manager history (type 7, with the round) and the end of the national team duty in the stored competition
/// Original: `Staff_Records::add_failed_to_qualify_record` (0x86c1d0, staff_records.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x86c1d0 } (thiscall, ret 0x10, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_failed_to_qualify_record(
/// clube, comp, fase, _)`: o histórico do técnico (tipo 7: clube, competição ou −1, a fase com sinal) e o fim das
/// convocações da seleção na competição guardada. Clube nulo: a caixa "Error" (linha 0x768).
pub(crate) unsafe extern "thiscall" fn add_failed_to_qualify_record(this: usize, clube: usize, comp: usize, fase: u32, _x: u32) {
    if clube == 0 {
        return show_error_at(0xa83b78, 0x768);
    }
    crate::comparacao::ramo("cpq_4 nao classificado");
    let e = stage_history_entry(clube, 7, comp, fase);
    if read_usize(clube + club::MANAGER) != 0 {
        append_person_history_by_id(true, this, read_usize(read_usize(clube + club::MANAGER)), &e);
    }
    end_national_duty(clube, comp);
}

// ponytail: como a eliminação: só a original.
/// Comparer for `add_failed_to_qualify_record` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_failed_to_qualify_record(this: usize, clube: usize, comp: usize, fase: u32, x: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_ADD_FAILED_TO_QUALIFY_RECORD)(this, clube, comp, fase, x)
}





/// O registro de pontos de cada técnico (Staff_Records +8): 0xd2 bytes por pessoa que não é jogador ([0xae2368]) e
/// pelos 16 humanos. +0x86 id da pessoa (−1 vazio), +0xb6 pontos do ano, +0xba/+0xbe pontos na liga e na copa (+0xc2 e
/// +0xc6 as competições deles, −1 sem), +0xca/+0xce os do mês.
pub(crate) const MANAGER_RECORD_SIZE: usize = 0xd2;

/// Number of manager records: [0xae2368] (non-player count) + 16 human slots.
pub(crate) unsafe fn manager_record_count() -> i32 {
    read_i32(G_NON_PLAYER_COUNT).wrapping_add(0x10)
}

/// Returns manager record k in the records array [this+8] (MANAGER_RECORD_SIZE bytes each).
pub(crate) unsafe fn manager_record_at(this: usize, k: i32) -> usize {
    read_usize(this + 8).wrapping_add((k as usize).wrapping_mul(MANAGER_RECORD_SIZE))
}

/// Returns the staff record of id ([G_STAFF_TABLE] + id × 0x6e; −1 gives the record before the table, as the original).
///
/// [pt] A pessoa pelo id (registro de 0x6e bytes em [0xae23bc]; −1 dá o registro antes da tabela, como na original).
pub(crate) unsafe fn staff_records_staff_by_id(id: usize) -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add(id.wrapping_mul(staff::SIZE))
}

/// Returns the month of a date (FN_DATE_GET_MONTH, low byte signed).
///
/// [pt] O mês da data (0x54cfb0, o al com sinal).
pub(crate) unsafe fn date_month(data: usize) -> i16 {
    let mes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_DATE_GET_MONTH);
    mes(data) as u8 as i8 as i16
}

/// at the turn of the month or year clears every manager's monthly points and stores the month and year; on the first
/// day of the year passes the yearly points to the hall of fame and clears them
/// Original: `Staff_Records::daily_update` (0x86dda0, staff_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x86dda0 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::daily_update(data)`: se o mês (0x54cfb0) ou o ano
/// (+2) da data não são os guardados (+0x14 mês, +0x16 ano), zera os pontos do mês (+0xca, +0xce) de todos os
/// registros e guarda o mês e o ano novos. No primeiro dia do ano (dia +0 = 0) passa os pontos do ano (+0xb6) de cada
/// registro com pessoa (+0x86) ao hall da fama (0x5ef270 em [0xb5ded8]) e os zera.
pub(crate) unsafe extern "thiscall" fn staff_records_daily_update(this: usize, data: usize) {
    if date_month(data) as i32 != read_i16(this + 0x14) as u16 as i32 || read_i16(data + 2) != read_i16(this + 0x16) {
        crate::comparacao::ramo("cpo_3 virada do mes");
        let base = read_usize(this + 8);
        let n = manager_record_count();
        for k in 0..n.max(0) as usize {
            let e = base.wrapping_add(k * MANAGER_RECORD_SIZE);
            write_usize(e + 0xca, 0);
            write_usize(e + 0xce, 0); // os pontos do mês
        }
        let m = date_month(data);
        core::ptr::write_unaligned((this + 0x14) as *mut i16, m);
        core::ptr::write_unaligned((this + 0x16) as *mut i16, read_i16(data + 2));
    }
    if read_i16(data) == 0 {
        let base = read_usize(this + 8);
        let hall: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_HALLOFFAME_ADD_CALENDAR_YEAR_ENTRY);
        let mut k = 0i32;
        while k < manager_record_count() {
            let e = base.wrapping_add((k as usize).wrapping_mul(MANAGER_RECORD_SIZE));
            let id = read_usize(e + 0x86);
            if id as i32 != -1 {
                hall(read_usize(G_HALL_OF_FAME), staff_records_staff_by_id(id), read_usize(e + 0xb6));
            }
            write_usize(e + 0xb6, 0);
            k += 1;
        }
    }
}

/// at the end of the competition's season passes its managers' league points (with the cup points of the same
/// competition) and cup points to the hall of fame and clears them
/// Original: `Staff_Records::end_season_for_nation` (0x86d6e0, staff_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x86d6e0 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::end_season_for_nation(entrada)`: com a competição
/// da entrada (+0, id em +0 dela; sem competição −1; sem competição e sem +4 igual a [0x9cfa10], nada), para cada
/// registro: liga (+0xc2) da competição: soma a copa (+0xbe) se ela também é dessa competição (+0xc6 = −1, +0xbe = 0),
/// passa os pontos (+0xba) ao hall da fama (0x5ef330) se a competição guarda (+0x11c), zera e +0xc2 = −1; só a copa da
/// competição: passa +0xbe, zera e +0xc6 = −1.
pub(crate) unsafe extern "thiscall" fn staff_records_end_season_for_nation(this: usize, entrada: usize) {
    let p = read_usize(entrada + 4);
    let pela_global = p != 0 && read_usize(p) == read_usize(KEY_CONTINENT_EUROPE);
    if !pela_global && read_usize(entrada) == 0 {
        return;
    }
    let comp = read_usize(entrada);
    let id = if comp != 0 { read_i32(comp) } else { -1 };
    let hall: unsafe extern "thiscall" fn(usize, usize, i32, usize) = core::mem::transmute(FN_HALLOFFAME_ADD_LEAGUE_ENTRY);
    let guarda = |c: usize| c != 0 && *((c + 0x11c) as *const u8) != 0;
    let base = read_usize(this + 8);
    let mut k = 0i32;
    while k < manager_record_count() {
        let s = base.wrapping_add((k as usize).wrapping_mul(MANAGER_RECORD_SIZE)) + 0xba;
        let copa = read_i32(s + 0xc);
        if read_i32(s + 8) == id {
            crate::comparacao::ramo("cpo_3 fim da liga");
            if copa == id {
                write_usize(s, read_usize(s).wrapping_add(read_usize(s + 4)));
                write_usize(s + 0xc, usize::MAX);
                write_usize(s + 4, 0);
            }
            if guarda(read_usize(entrada)) {
                hall(read_usize(G_HALL_OF_FAME), staff_records_staff_by_id(read_usize(s - 0x34)), id, read_usize(s));
            }
            write_usize(s, 0);
            write_usize(s + 8, usize::MAX);
        } else if copa == id {
            if guarda(read_usize(entrada)) {
                hall(read_usize(G_HALL_OF_FAME), staff_records_staff_by_id(read_usize(s - 0x34)), id, read_usize(s + 4));
            }
            write_usize(s + 4, 0);
            write_usize(s + 0xc, usize::MAX);
        }
        k += 1;
    }
}

/// the league manager with the most points this month (above 0) among those two weeks in the job; a league without a
/// table: the "Error" box and 0
/// Original: `Staff_Records::find_manager_of_the_month` (0x86f950, staff_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x86f950 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::find_manager_of_the_month(indice)`: na liga
/// ([0xadadfc] pelo índice; +0x43 tem de ser 2, senão a caixa "Error" e 0), entre os clubes da tabela (+0xb1, 0x3b
/// bytes, conta +0x3e) com técnico (+0xcf) no clube (+0x39) desde duas semanas atrás (0x54cab0 da data do jogo; ano
/// +0x40 sem sinal, dia +0x3e com sinal), o de mais pontos no mês (+0xce do registro 0x86f5b0, criado se falta), só
/// acima de 0. Devolve o técnico ou 0.
pub(crate) unsafe extern "thiscall" fn find_manager_of_the_month(this: usize, indice: i32) -> usize {
    let liga = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((indice as usize).wrapping_mul(4)));
    if *((liga + 0x43) as *const u8) != 2 {
        show_error_at(0xa83b78, 0x104e);
        return 0;
    }
    let antes: unsafe extern "thiscall" fn(usize, *mut u32, i32) -> usize = core::mem::transmute(FN_DATE_SUB_WEEKS);
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
    let mut melhor_pontos = 0i32;
    let mut melhor = 0usize;
    let mut k = 0i32;
    while k < read_i16(liga + 0x3e) as i32 {
        let clube = read_usize(read_usize(liga + 0xb1).wrapping_add((k as usize).wrapping_mul(0x3b)));
        let tecnico = read_usize(clube + club::MANAGER);
        if tecnico != 0 && read_usize(tecnico + 0x39) != 0 {
            let mut d = [0u32; 2];
            let q = antes(G_CURRENT_DATE, d.as_mut_ptr(), 2); // duas semanas atrás
            let lo = read_usize(q) as u32;
            let ano = (lo >> 16) as u16;
            let ano_tecnico = read_i16(tecnico + 0x40) as u16;
            let recente = ano < ano_tecnico || (ano == ano_tecnico && (lo as u16 as i16) < read_i16(tecnico + 0x3e));
            if recente {
                crate::comparacao::ramo("cpo_3 tecnico recente");
            } else {
                let r = registro(this, tecnico, 1);
                if r != 0 {
                    let pontos = read_i32(r + 0xce);
                    if pontos > melhor_pontos { // o melhor do mês
                        crate::comparacao::ramo("cpo_3 melhor do mes");
                        melhor_pontos = pontos;
                        melhor = read_usize(clube + club::MANAGER);
                    }
                }
            }
        }
        k += 1;
    }
    melhor
}

/// Comparer for `staff_records_daily_update` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: os pontos vão ao hall da fama (0x5ef270, soma): roda só a original e confere o plano: na virada, os pontos
/// do mês zerados e o mês e o ano guardados; no primeiro dia do ano, os pontos do ano zerados.
pub(crate) unsafe extern "thiscall" fn comparar_staff_records_daily_update(this: usize, data: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_STAFF_RECORDS_DAILY_UPDATE);
    let virada = date_month(data) as i32 != read_i16(this + 0x14) as u16 as i32 || read_i16(data + 2) != read_i16(this + 0x16);
    let ano = read_i16(data) == 0;
    let (m, a) = (date_month(data), read_i16(data + 2));
    o(this, data);
    if !virada && !ano {
        return;
    }
    let mut ok = !virada || (read_i16(this + 0x14) == m && read_i16(this + 0x16) == a);
    let mut primeiro = -1i32;
    for k in 0..manager_record_count().max(0) {
        let e = manager_record_at(this, k);
        let certo = (!virada || (read_usize(e + 0xca) == 0 && read_usize(e + 0xce) == 0)) && (!ano || read_usize(e + 0xb6) == 0);
        if !certo && primeiro < 0 {
            primeiro = k;
        }
        ok &= certo;
    }
    PLACAR.registrar(ok, || format!("staff_records_daily_update(mês {}, ano {}) virada {} ano novo {}: registro {} não zerado", m, a, virada, ano, primeiro));
}

/// Comparer for `staff_records_end_season_for_nation` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: os pontos vão ao hall da fama (0x5ef330, soma): roda só a original e confere o plano feito antes: os
/// quatro campos (+0xba, +0xbe, +0xc2, +0xc6) de cada registro.
pub(crate) unsafe extern "thiscall" fn comparar_staff_records_end_season_for_nation(this: usize, entrada: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_STAFF_RECORDS_END_SEASON_FOR_NATION);
    let p = read_usize(entrada + 4);
    if !(p != 0 && read_usize(p) == read_usize(KEY_CONTINENT_EUROPE)) && read_usize(entrada) == 0 {
        return o(this, entrada);
    }
    let comp = read_usize(entrada);
    let id = if comp != 0 { read_i32(comp) } else { -1 };
    let n = manager_record_count().max(0);
    let mut plano = Vec::with_capacity(n as usize);
    for k in 0..n {
        let s = manager_record_at(this, k) + 0xba;
        let (mut liga, mut copa, mut cl, mut cc) = (read_usize(s), read_usize(s + 4), read_i32(s + 8), read_i32(s + 0xc));
        if cl == id {
            if cc == id {
                cc = -1; // a copa somada à liga, que vai ao hall da fama e zera
                copa = 0;
            }
            liga = 0;
            cl = -1;
        } else if cc == id {
            copa = 0;
            cc = -1;
        }
        plano.push((liga, copa, cl, cc));
    }
    o(this, entrada);
    let errado = (0..n).find(|&k| {
        let s = manager_record_at(this, k) + 0xba;
        plano[k as usize] != (read_usize(s), read_usize(s + 4), read_i32(s + 8), read_i32(s + 0xc))
    });
    PLACAR.registrar(errado.is_none(), || format!("staff_records_end_season_for_nation(competição {}) registro {:?} difere do plano", id, errado));
}

/// Comparer for `find_manager_of_the_month` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (o registro 0x86f5b0 criado pela original já existe para a cópia).
pub(crate) unsafe extern "thiscall" fn comparar_find_manager_of_the_month(this: usize, indice: i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> usize>(IDX_FIND_MANAGER_OF_THE_MONTH);
    let liga = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((indice as usize).wrapping_mul(4)));
    if *((liga + 0x43) as *const u8) != 2 {
        return o(this, indice);
    }
    let r = o(this, indice);
    let n = find_manager_of_the_month(this, indice);
    PLACAR.registrar(r == n, || format!("find_manager_of_the_month({}) original={:x} rust={:x}", indice, r, n));
    r
}

/// sorts the clubs and keeps up to three whose manager has been in the job 28 weeks and has games in the record
/// Original: `Staff_Records::get_manager_of_year_candidates` (0x86fab0, staff_records.cpp, thiscall, ret 8).
///
/// [pt] { 0x86fab0 } (thiscall, ret 8, staff_records.cpp, SEH sem objeto com destrutor):
/// `Staff_Records::get_manager_of_year_candidates(clubes, n)` (pela 0x768940): o qsort dos `n` clubes (0x870220) e, na
/// ordem, até três cujo técnico (+0xcf) chegou (+0x3e) antes de hoje − 28 semanas (0x54cab0; ano sem sinal, dia com
/// sinal) e tem registro (0x86f5b0 com 1) com jogos (+0x8e), copiados para o começo do próprio vetor. Devolve quantos.
pub(crate) unsafe extern "thiscall" fn get_manager_of_year_candidates(this: usize, clubes: usize, n: u32) -> u8 {
    let semanas: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_SUB_WEEKS);
    let mut lim = [0u8; 8];
    semanas(G_CURRENT_DATE, lim.as_mut_ptr(), 0x1c);
    let n = n as u16 as i16 as i32;
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(clubes, n as u32, 4, 0x870220);
    let (ld, la) = (i16::from_le_bytes([lim[0], lim[1]]), u16::from_le_bytes([lim[2], lim[3]]));
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
    let mut k = 0u8;
    for i in 0..n.max(0) as usize {
        if k as i8 >= 3 {
            break;
        }
        let tec = read_u32_as_usize(read_u32_as_usize(clubes + 4 * i) + 0xcf);
        if tec == 0 {
            continue;
        }
        let ta = read_u16(tec + 0x40);
        if !(la > ta || (la == ta && ld > read_i16(tec + 0x3e))) {
            continue;
        }
        let r = registro(this, tec, 1);
        if r != 0 && read_i16(r + 0x8e) != 0 {
            core::ptr::write_unaligned((clubes + 4 * k as usize) as *mut u32, read_u32_as_usize(clubes + 4 * i) as u32);
            k += 1;
        }
    }
    k
}

/// Comparer for `get_manager_of_year_candidates` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo vetor dos clubes e pela data estática da 0x54cab0, e a devolução (os registros dos
/// técnicos que a 0x86f5b0 cria na original já existem para a cópia).
pub(crate) unsafe extern "thiscall" fn comparar_get_manager_of_year_candidates(this: usize, clubes: usize, n: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(IDX_GET_MANAGER_OF_YEAR_CANDIDATES);
    let k = (n as u16 as i16).max(0) as usize;
    let reg = [(clubes, 4 * k), (G_SUB_WEEKS_RESULT_DATE, 8), (0xae24aa, 1)];
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clubes, n), || get_manager_of_year_candidates(this, clubes, n));
    PLACAR.registrar(igual && r == c, || format!("get_manager_of_year_candidates({}) original={} rust={} {:x?} x {:x?}", k, r, c, mo, mr));
    r
}

/// finds (or creates, or takes over) the manager's 210-byte record and updates the nations, the current spell and the
/// longest and shortest spells
/// Original: `Staff_Records::get_manager_record` (0x86f5b0, staff_records.cpp, thiscall, ret 8).
///
/// [pt] { 0x86f5b0 } (thiscall, ret 8, staff_records.cpp): `Staff_Records::get_manager_record(pessoa, criar)`: o registro de
/// 210 bytes do técnico (vetor +8). Pessoa nula: caixa da linha 0xf6e e 0. Vaga livre (+0x86 = −1): sem `criar`, 0;
/// com ele, a vaga fica com o id. Vaga de outro: zera o registro da ficha dele (−1 em +0x86, +0xc2, +0xc6, +0x4a, +0x5e,
/// +0x72) e olha de novo. Atualiza as nações do clube (+0x39) e da seleção (+0x24) em +0xc2/+0xc6, zerando os pontos
/// (+0xba/+0xbe) quando mudam. Com a data de [0xdd6ec8] até hoje: sem passagem (+0x4a = −1), abre a do clube atual
/// (desde +0x3e da pessoa; +0x56 = a data 13/11/1975 da 0x549f70); a mais longa (+0x5e, +0x62..+0x6a) e a mais curta
/// (+0x72, +0x76..+0x7e) pela 0x54cdb0, a de hoje em +0x6a / +0x7e. Devolve o registro.
pub(crate) unsafe extern "thiscall" fn get_manager_record(this: usize, pessoa: usize, criar: u32) -> usize {
    if pessoa == 0 {
        let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
        let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
        let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
        let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
        separar(0xa83b78, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
        let v = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), 0xf6e);
        formatar(0xae24d0, 0x9870e8, v, nome.as_ptr(), ext.as_ptr(), 0xf6eu32);
        caixa(0x9870e0, 0xae24d0, 0);
        write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
        return 0;
    }
    let r = loop {
        let Some(r) = manager_manager_manager_record(this, pessoa) else {
            return 0;
        };
        let dono = read_u32(r + 0x86) as i32;
        if dono == -1 {
            if criar == 0 {
                return 0;
            }
            crate::comparacao::ramo("cpp_3 criar registro");
            write_u32(r + 0x86, read_u32(pessoa)); // a vaga fica com ele
            break r;
        }
        if dono == read_u32(pessoa) as i32 {
            break r;
        }
        // a vaga é de outro: zera o registro da ficha dele e olha de novo
        let z = (read_u32(this + 8) as usize).wrapping_add((read_u32(read_u32(pessoa + staff::NON_PLAYER) as usize) as usize).wrapping_mul(0xd2));
        core::ptr::write_bytes(z as *mut u8, 0, 0xd2);
        for o in [0x86, 0xc2, 0xc6, 0x4a, 0x5e, 0x72] {
            write_u32(z + o, u32::MAX);
        }
    };
    let clube = read_u32(pessoa + staff::CLUB_JOB) as usize;
    if clube != 0 {
        let nacao = read_u32(clube + club::NATION) as usize;
        if nacao != 0 && read_u32(nacao) != read_u32(r + 0xc2) {
            write_u32(r + 0xc2, read_u32(nacao));
            write_u32(r + 0xba, 0);
        }
    }
    let selecao = read_u32(pessoa + staff::NATIONAL_JOB) as usize;
    if selecao != 0 {
        let nacao = read_u32(selecao + 0x53) as usize;
        if nacao != 0 && read_u32(nacao) != read_u32(r + 0xc6) {
            write_u32(r + 0xc6, read_u32(nacao));
            write_u32(r + 0xbe, 0);
        }
    }
    let (dia, ano) = (read_i16(G_GAME_START_DATE), core::ptr::read_unaligned(G_GAME_START_DATE_YEAR as *const u16));
    let hoje_ano = core::ptr::read_unaligned(G_CURRENT_DATE_YEAR as *const u16);
    if ano > hoje_ano || (ano == hoje_ano && dia > read_i16(G_CURRENT_DATE)) {
        return r; // a data de [0xdd6ec8] ainda não chegou
    }
    if read_u32(r + 0x4a) as i32 == -1 {
        let clube = read_u32(pessoa + staff::CLUB_JOB) as usize;
        if clube != 0 {
            crate::comparacao::ramo("cpp_3 abrir passagem");
            write_u32(r + 0x4a, read_u32(clube));
            contract_manager_assign_date(r + 0x4e, pessoa + staff::DATE_JOINED_CLUB);
            let mut local = [0u8; 8];
            let construir: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32, u32, i32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
            let p = construir(&mut local, 0xd, 0xb, 0x7b7, -1);
            contract_manager_assign_date(r + 0x56, p);
        }
    }
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    if read_u32(r + 0x5e) as i32 == -1 {
        write_u32(r + 0x5e, read_u32(r + 0x4a));
        contract_manager_assign_date(r + 0x62, r + 0x4e);
        contract_manager_assign_date(r + 0x6a, G_CURRENT_DATE);
    } else if read_u32(r + 0x4a) as i32 != -1 {
        let a = dias(r + 0x6a, read_u32(r + 0x62), read_u32(r + 0x66));
        let b = dias(G_CURRENT_DATE, read_u32(r + 0x4e), read_u32(r + 0x52));
        if b > a {
            write_u32(r + 0x5e, read_u32(r + 0x4a)); // a mais longa
            contract_manager_assign_date(r + 0x62, r + 0x4e);
            contract_manager_assign_date(r + 0x6a, G_CURRENT_DATE);
        }
    }
    if read_u32(r + 0x72) as i32 == -1 {
        write_u32(r + 0x72, read_u32(r + 0x4a));
        contract_manager_assign_date(r + 0x76, r + 0x4e);
        contract_manager_assign_date(r + 0x7e, G_CURRENT_DATE);
        return r;
    }
    let atual = read_u32(r + 0x4a);
    if read_u32(r + 0x72) == atual && read_i16(r + 0x50) == read_i16(r + 0x78) && read_i16(r + 0x4e) == read_i16(r + 0x76) {
        contract_manager_assign_date(r + 0x7e, G_CURRENT_DATE); // a mais curta é a atual: vai até hoje
        return r;
    }
    if atual as i32 == -1 {
        return r;
    }
    let a = dias(r + 0x7e, read_u32(r + 0x76), read_u32(r + 0x7a));
    let b = dias(G_CURRENT_DATE, read_u32(r + 0x4e), read_u32(r + 0x52));
    if b < a {
        write_u32(r + 0x72, read_u32(r + 0x4a)); // a mais curta
        contract_manager_assign_date(r + 0x76, r + 0x4e);
        contract_manager_assign_date(r + 0x7e, G_CURRENT_DATE);
    }
    r
}

/// Comparer for `get_manager_record` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo registro do técnico (e, quando a vaga é de outro, pelo registro que a original zera),
/// pela devolução. Pessoa nula (a caixa) e o humano sem situação: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_get_manager_record(this: usize, pessoa: usize, criar: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_GET_MANAGER_RECORD);
    if pessoa == 0 {
        return o(this, pessoa, criar);
    }
    let Some(r) = manager_manager_manager_record(this, pessoa) else {
        return o(this, pessoa, criar);
    };
    let mut reg = vec![(r, 0xd2)];
    let dono = read_u32(r + 0x86) as i32;
    if dono != -1 && dono != read_u32(pessoa) as i32 {
        let z = (read_u32(this + 8) as usize).wrapping_add((read_u32(read_u32(pessoa + staff::NON_PLAYER) as usize) as usize).wrapping_mul(0xd2));
        if z != r {
            reg.push((z, 0xd2));
        }
    }
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa, criar), || get_manager_record(this, pessoa, criar));
    PLACAR.registrar(igual && a == b, || format!("get_manager_record(pessoa {:x}, criar {}) original={:x} rust={:x} {:x?} x {:x?}", pessoa, criar, a, b, mo, mr));
    a
}

/// the free release histories: the club manager one (type 0x68) and the person one (type 0x67)
/// Original: `Staff_Records::add_free_release_records` (0x86d800, staff_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x86d800 } (thiscall, ret 4, staff_records.cpp, SEH só da data local, de destrutor vazio):
/// `Staff_Records::add_free_release_records(pessoa)`: o contrato da pessoa (0x4cf5a0) vai para [0xae1080] e dá o clube
/// (registro de 0x245 bytes pelo +4). Com técnico no clube (+0xcf), o histórico dele (tipo 0x68: o clube, a pessoa e os
/// três ids +0x33 de +4, +8 e +0xc dela); depois o da pessoa (tipo 0x67: o clube ou −1), no arquivo (0x58e340).
/// Pessoa nula: a caixa "Error" (linha 0x1010).
pub(crate) unsafe extern "thiscall" fn add_free_release_records(this: usize, pessoa: usize) {
    crate::comparacao::ramo("cpr_9 liberado");
    let mut e = history_entry();
    let de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = de(read_u32_as_usize(G_CONTRACT_MANAGER), pessoa);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let clube = if c != 0 { read_u32_as_usize(G_CLUB_TABLE).wrapping_add(read_u32_as_usize(c + 4).wrapping_mul(club::SIZE)) } else { 0 };
    if clube != 0 && read_u32_as_usize(clube + club::MANAGER) != 0 {
        stamp_game_date(e.as_mut_ptr());
        e[0xc] = 0x68;
        put_i32(&mut e, 8, read_i32(clube));
        put_person_keys(&mut e, 0xd, pessoa);
        append_person_history_by_id(true, this, read_u32_as_usize(read_u32_as_usize(clube + club::MANAGER)), &e);
    }
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x67;
    put_i32(&mut e, 8, staff_records_id_or_minus_one(clube));
    if pessoa != 0 {
        append_person_history_by_id(true, this, read_u32_as_usize(pessoa), &e);
    } else {
        show_error_at(0xa83b78, 0x1010);
    }
}

/// Comparer for `add_free_release_records` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_free_release_records(this: usize, pessoa: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_FREE_RELEASE_RECORDS)(this, pessoa)
}

/// the loan histories: the one of the receiving club manager (type 0x69) and the one of the person (type 0x6a)
/// Original: `Staff_Records::add_loan_records` (0x86da20, staff_records.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x86da20 } (thiscall, ret 0xc, staff_records.cpp, SEH só da data local, de destrutor vazio):
/// `Staff_Records::add_loan_records(pessoa, de, clube)` (pela 0x8c2180, a transferência aceita): com o clube e o técnico
/// dele (+0xcf), o histórico do técnico (tipo 0x69: o clube, a pessoa e os três ids +0x33 de +4, +8 e +0xc dela, e o id
/// de `de`); depois, na mesma entrada, o da pessoa (tipo 0x6a: o clube e o id de `de`), no arquivo (0x58e340). O clube
/// é lido antes do teste da pessoa, como na original; pessoa nula: a caixa "Error" (linha 0x1010).
pub(crate) unsafe extern "thiscall" fn add_loan_records(this: usize, pessoa: usize, de: usize, clube: usize) {
    crate::comparacao::ramo("cps_9 emprestimo");
    let mut e = history_entry();
    if clube != 0 && read_u32_as_usize(clube + club::MANAGER) != 0 {
        let tecnico = read_u32_as_usize(clube + club::MANAGER);
        stamp_game_date(e.as_mut_ptr());
        e[0xc] = 0x69;
        put_i32(&mut e, 8, read_i32(clube));
        put_person_keys(&mut e, 0xd, pessoa);
        put_i32(&mut e, 0x1d, read_i32(de));
        append_person_history_by_id(true, this, read_u32_as_usize(tecnico), &e);
    }
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x6a;
    put_i32(&mut e, 8, read_i32(clube));
    put_i32(&mut e, 0xd, read_i32(de));
    if pessoa != 0 {
        append_person_history_by_id(true, this, read_u32_as_usize(pessoa), &e);
    } else {
        show_error_at(0xa83b78, 0x1010);
    }
}

/// Comparer for `add_loan_records` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_loan_records(this: usize, pessoa: usize, de: usize, clube: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_ADD_LOAN_RECORDS)(this, pessoa, de, clube)
}

/// the last offset of the person's history list (+4 of the pair at [+0xc] + 8 × id) = −1 (0x58f110)
/// Original: `Staff_Records::rewind_staff_history` (0x86fb80, staff_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x86fb80 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::rewind_staff_history(pessoa)`: a lista do
/// histórico da pessoa (o par de 8 bytes em [+0xc] + 8 × id) fica sem o último (0x58f110 sobre [this]: o segundo
/// deslocamento = −1). Pessoa nula: a original (caixa fatal: o jogo termina).
pub(crate) unsafe extern "thiscall" fn rewind_staff_history(this: usize, pessoa: usize) {
    if pessoa == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REWIND_STAFF_HISTORY)(this, pessoa); // caixa fatal: só a original
    }
    crate::comparacao::ramo("cpk_7 recomecar historico");
    let par = read_usize(this + 0xc).wrapping_add(read_usize(pessoa).wrapping_mul(8));
    let sem_ultimo: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_FILE_LLIST_RESET_LAST_OFFSET);
    sem_ultimo(read_usize(this), par, par + 4); // o último deslocamento
}

/// Comparer for `rewind_staff_history` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo par da pessoa (8 bytes; a 0x58f110 só grava o segundo).
pub(crate) unsafe extern "thiscall" fn comparar_rewind_staff_history(this: usize, pessoa: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REWIND_STAFF_HISTORY);
    if pessoa == 0 {
        return o(this, pessoa);
    }
    let par = read_usize(this + 0xc).wrapping_add(read_usize(pessoa).wrapping_mul(8));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(par, 8)], || o(this, pessoa), || rewind_staff_history(this, pessoa));
    PLACAR.registrar(igual, || format!("rewind_staff_history({}) {:x?} x {:x?}", read_usize(pessoa), mo, mr));
}

/// the year award entry in the person history; the manager adds the award to his record; the player with a long
/// contract on the club list gets the renewal offer and the winner may thank the manager (draw by loyalty)
/// Original: `Staff_Records::add_year_award_record` (0x86c390, staff_records.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x86c390 } (thiscall, ret 0x18, staff_records.cpp, quadro SEH sem destrutor): `Staff_Records::add_year_award_record(
/// p, clube, premio, a4, a5, credito)`, pelo registro dos vencedores do prêmio do ano (YEAR_AWARD::record_winners
/// 0x9302f0: o primeiro, o segundo e o terceiro). A entrada 0x34 do histórico da pessoa (data do jogo, clube ou −1, o id
/// do prêmio, a4 na palavra, a5 e o crédito em bytes com sinal; 0x58e340). Técnico (+0x3d ou +0x28 = 5 ou 0xc): +1 na
/// palavra +0x9e do registro dele (0x86f5b0). Jogador com ficha (+0x61) e contrato (0x4cf5a0, fora do tipo 5) com o
/// clube (0x4cf5f0): com o gerente de transferências, contrato de mais de 300 dias e o jogador na lista do clube
/// (0x849ce0), a renovação (0x4cda70); depois, só com o crédito 1, sem a marca +0x45 do contrato, com +0x59 acima de 15
/// e +0x5b acima de 10, sorteio(+0x59) acima de 5: a notícia do agradecimento ao técnico (0x58a070). Sem pessoa ou sem
/// prêmio: a original (caixa "Error").
pub(crate) unsafe extern "thiscall" fn add_year_award_record(this: usize, p: usize, clube: usize, premio: usize, a4: u32, a5: u32, credito: u32) {
    if p == 0 || premio == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32)>(IDX_ADD_YEAR_AWARD_RECORD)(this, p, clube, premio, a4, a5, credito);
    }
    add_year_award(this, p, clube, premio, a4, a5, credito, true)
}

/// Comparer for `add_year_award_record` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o técnico lado a lado pelo registro de pontos dele (criado antes pela 0x86f5b0, como a original faria;
/// a cópia sem a entrada do histórico em arquivo). O jogador sorteia, renova contrato e manda notícia (não pode rodar
/// duas vezes): só a original. Sem pessoa ou sem prêmio (a caixa): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_add_year_award_record(this: usize, p: usize, clube: usize, premio: usize, a4: u32, a5: u32, credito: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32)>(IDX_ADD_YEAR_AWARD_RECORD);
    if p == 0 || premio == 0 {
        return o(this, p, clube, premio, a4, a5, credito);
    }
    let (t3d, t28) = (read_u8(p + 0x3d), read_u8(p + 0x28));
    if !(t3d == 5 || t3d == 0xc || t28 == 5 || t28 == 0xc) {
        crate::comparacao::ramo("add_year_award_record jogador");
        return o(this, p, clube, premio, a4, a5, credito);
    }
    let r = staff_records_manager_record(this, p);
    if r == 0 {
        return o(this, p, clube, premio, a4, a5, credito);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(r, 0xd2)], || o(this, p, clube, premio, a4, a5, credito),
        || add_year_award(this, p, clube, premio, a4, a5, credito, false));
    PLACAR.registrar(igual, || format!("add_year_award_record({:x}, prêmio {:x}) {:x?} x {:x?}", p, read_usize(premio), mo, mr));
}

/// the first cap history of the person (type 6: the club, the opponent and the age)
/// Original: `Staff_Records::add_first_cap_record` (0x86cc50, staff_records.cpp, thiscall, ret 8).
///
/// [pt] { 0x86cc50 } (thiscall, ret 8, staff_records.cpp, SEH só da data local, de destrutor vazio):
/// `Staff_Records::add_first_cap_record(pessoa, adversário)` (pelo resultado internacional 0x76b600): o histórico da
/// pessoa (tipo 6: o clube contratante +0x39 ou −1, o id do adversário na partida e a idade +0x18 com sinal), no arquivo (0x58e340).
/// O clube é lido antes do teste da pessoa, como na original; pessoa nula: a caixa "Error" (linha 0x1010).
pub(crate) unsafe extern "thiscall" fn add_first_cap_record(this: usize, pessoa: usize, adversario: usize) {
    crate::comparacao::ramo("cps_8 primeira convocacao");
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 6;
    put_i32(&mut e, 8, club_job_id(pessoa));
    put_i32(&mut e, 0xd, read_i32(adversario));
    put_i32(&mut e, 0x11, *((pessoa + staff::YEAR_OF_BIRTH) as *const i8) as i32);
    if pessoa != 0 {
        append_person_history_by_id(true, this, read_u32_as_usize(pessoa), &e);
    } else {
        show_error_at(0xa83b78, 0x1010);
    }
}

/// Comparer for `add_first_cap_record` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_first_cap_record(this: usize, pessoa: usize, adversario: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_ADD_FIRST_CAP_RECORD)(this, pessoa, adversario)
}

/// writes the 0x33 history entry of the winner; a manager counts +1 in his record; a player may get a contract renewal
/// and, by a draw, thanks his manager (news)
/// Original: `Staff_Records::add_month_award_record` (0x86c8c0, staff_records.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x86c8c0 } (thiscall, ret 0x14, staff_records.cpp, quadro SEH sem destrutor): `Staff_Records::add_month_award_record(
/// p, clube, premio, a4, a5)`: a entrada 0x33 do histórico da pessoa (data do jogo, clube ou −1, o id do prêmio, a4 no
/// byte, a5 na palavra; 0x58e340). Técnico (+0x3d ou +0x28 = 5 ou 0xc): +1 na palavra +0x9e do registro dele
/// (0x86f5b0). Jogador com ficha (+0x61) e contrato (0x4cf5a0, fora do tipo 5) com o clube (0x4cf5f0): com o gerente de
/// transferências, contrato de mais de 300 dias e o jogador na lista do clube (0x849ce0), a renovação (0x4cda70); depois,
/// sem a marca +0x45 do contrato, com +0x59 acima de 15 e +0x5b acima de 10, sorteio(+0x59) acima de 5: a notícia do
/// agradecimento ao técnico (0x58a070). Sem pessoa ou sem prêmio: a original (caixa "Error").
pub(crate) unsafe extern "thiscall" fn add_month_award_record(this: usize, p: usize, clube: usize, premio: usize, a4: u32, a5: u32) {
    if p == 0 || premio == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32)>(IDX_ADD_MONTH_AWARD_RECORD)(this, p, clube, premio, a4, a5);
    }
    add_month_award_record_body(this, p, clube, premio, a4, a5, true)
}

/// Comparer for `add_month_award_record` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o técnico lado a lado pelo registro de pontos dele (criado antes pela 0x86f5b0, como a original faria;
/// a cópia sem a entrada do histórico em arquivo). O jogador sorteia, renova contrato e manda notícia (não pode rodar
/// duas vezes): só a original. Sem pessoa ou sem prêmio (a caixa): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_add_month_award_record(this: usize, p: usize, clube: usize, premio: usize, a4: u32, a5: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32)>(IDX_ADD_MONTH_AWARD_RECORD);
    if p == 0 || premio == 0 {
        return o(this, p, clube, premio, a4, a5);
    }
    let (t3d, t28) = (read_u8(p + 0x3d), read_u8(p + 0x28));
    if !(t3d == 5 || t3d == 0xc || t28 == 5 || t28 == 0xc) {
        crate::comparacao::ramo("add_month_award_record jogador");
        return o(this, p, clube, premio, a4, a5);
    }
    let r = staff_records_manager_record(this, p);
    if r == 0 {
        return o(this, p, clube, premio, a4, a5);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(r, 0xd2)], || o(this, p, clube, premio, a4, a5),
        || add_month_award_record_body(this, p, clube, premio, a4, a5, false));
    PLACAR.registrar(igual, || format!("add_month_award_record({:x}, prêmio {:x}) {:x?} x {:x?}", p, read_usize(premio), mo, mr));
}

/// clears the manager record and sets the owner, league and club ids to −1
/// Original: `Staff_Records::clear_manager_record` (0x86fc20, staff_records.cpp, thiscall).
///
/// [pt] { 0x86fc20 } (thiscall, ret 4, staff_records.cpp): zera o registro de técnico da entrada e põe −1
/// nos ids do dono, da liga e dos clubes das passagens (+0x4a, +0x5e, +0x72, +0x86, +0xc2, +0xc6).
pub(crate) unsafe extern "thiscall" fn clear_manager_record(registros: usize, id: *const i32) {
    let r = friendly_manager_record(registros, id);
    core::ptr::write_bytes(r as *mut u8, 0, 210);
    for o in [0x86, 0xc2, 0xc6, 0x4a, 0x5e, 0x72] {
        core::ptr::write_unaligned((r + o) as *mut i32, -1);
    }
}

/// Comparer for `clear_manager_record` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_clear_manager_record(registros: usize, id: *const i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32)>(IDX_CLEAR_MANAGER_RECORD);
    let r = friendly_manager_record(registros, id);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(r, 210)], || o(registros, id), || clear_manager_record(registros, id));
    PLACAR.registrar(igual, || format!("clear_manager_record({}) {:x?} x {:x?}", *id, mo, mr));
}
