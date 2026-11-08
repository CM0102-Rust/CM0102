use super::*;

/// The dry run of the comparers.
///
/// The copies run without the histories (file), without the news (`0x75c390`), without ending the call-ups (`0x774f30`)
/// and without the renewal (`0x4cda70`), noting the contracts the renewal would touch.
///
/// [pt] O ensaio dos comparadores: as cópias sem os históricos (arquivo), sem as notícias (0x75c390), sem acabar as
/// convocações (0x774f30) e sem a renovação (0x4cda70), anotando os contratos que a renovação mexeria.
pub(crate) static mut STAFF_HISTORY_DRY_RUN: bool = false;
pub(crate) static mut DRY_RUN_RENEWED_CONTRACTS: Vec<usize> = Vec::new();
/// After the copy's last day count (0x54cdb0), the rehearsal skipped a call of the original routine that also counts
/// days and leaves its static scratch date (0xae2470) in another year: the renewal (0x4cda70, by age), the end of the
/// call-ups (0x774f30) or the praise news item (0x75c390). That date is then left out of the check.
///
/// [pt] Depois da última conta de dias (0x54cdb0) da cópia, o ensaio pulou uma chamada da original que também conta dias
/// e deixa a data de rascunho estática dela (0xae2470) noutro ano: a renovação (0x4cda70, pela idade), o fim das
/// convocações (0x774f30) ou a notícia do elogio (0x75c390). Essa data então fica fora da conta.
pub(crate) static mut DRY_RUN_STRAY_SCRATCH_DATE: bool = false;

/// Raises club reputation (club::REPUTATION) toward weight × 500, at most 750 at once, clamped to 1..10000.
///
/// [pt] A reputação do clube (+0x80) sobe até peso × 500, no máximo 750 de uma vez (entre 1 e 10000; soma em 16 bits).
pub(crate) unsafe fn raise_champion_reputation(club: usize, weight: i16, champion_of_league: bool) {
    let target = (weight as i32).wrapping_mul(500);
    let rep = read_i16(club + club::REPUTATION) as i32;
    if rep < target {
        let foul = target - rep;
        let sum = if foul > 0x2ee { 0x2ee } else { foul }; // cqd13: the rise cap
        let v = read_i16(club + club::REPUTATION).wrapping_add(sum as i16);
        let v = if v < 1 { 1 } else if v > 10000 { 10000 } else { v };
        if champion_of_league {
            crate::comparison::branch("cqd_13 injury_word_0c do campeao da liga");
        } else {
            crate::comparison::branch("cqd_13 injury_word_0c do campeao da copa");
        }
        core::ptr::write_unaligned((club + club::REPUTATION) as *mut i16, v); // cqd13: the reputation of the champion
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
pub(crate) unsafe fn offer_renewal_if_due(club: usize, p: usize, k: usize) {
    if read_usize(G_TRANSFER_MANAGER) == 0 {
        return;
    }
    DRY_RUN_STRAY_SCRATCH_DATE = false;
    if days_between(G_CURRENT_DATE, k + contract::DATE_STARTED) <= 300 {
        return;
    }
    let slot_of: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    // 0x849ce0: push 0; push 0; push p; push clube; mov ecx, [[0xde1f54]+0x856] -> thiscall (ret 0x10)
    let v = slot_of(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), club, p, 0, 0);
    if v == 0 || read_i32(v + shortlist_slot::TARGET_STAFF_ID) != read_i32(p) {
        return;
    }
    if STAFF_HISTORY_DRY_RUN {
        (*core::ptr::addr_of_mut!(DRY_RUN_RENEWED_CONTRACTS)).push(k);
        DRY_RUN_STRAY_SCRATCH_DATE = true;
        return;
    }
    let oferecer: unsafe extern "thiscall" fn(usize, usize, u32, u32, i32, f64, u32) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_OFFER_CONTRACT);
    // 0x4cda70: push 1; push 0x3ff00000; push 0; push −1; push 0; push 0; push p; mov ecx, [0xae1050] -> thiscall
    // (p, 0, 0, −1, 1.0 in two dwords, 1)
    oferecer(read_usize(G_CONTRACT_MANAGER), p, 0, 0, -1, 1.0, 1);
}

/// Adds history entry e for each of the 20 lineup players (+0x190, 0x49 bytes, −1 empty) except non-players and the
/// manager.
///
/// [pt] Os históricos da entrada `e` para o técnico (já feito por quem chama), o presidente (+0xbf) e os 20 da escalação
/// (+0x190 do lado, 0x49 bytes, −1 vazio; pessoas válidas e fora o técnico; nula: a caixa da linha 0x1010).
pub(crate) unsafe fn add_lineup_histories(this: usize, club: usize, escalacao: usize, e: &[u8; 0x40]) {
    for j in 0..0x14usize {
        let id = read_i32(escalacao + 0x49 * j);
        if id == -1 {
            continue;
        }
        let p = read_usize(G_STAFF_TABLE).wrapping_add((id as u32).wrapping_mul(0x6e) as usize);
        if p != 0 && read_i32(p) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        if p == read_usize(club + club::MANAGER) {
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
pub(crate) unsafe fn add_club_history(this: usize, club: usize, kind: u8, comp: usize) {
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[8] = kind;
    put_i32(&mut e, 9, read_i32(comp));
    if !STAFF_HISTORY_DRY_RUN {
        let f: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) -> usize = core::mem::transmute(FN_FILE_LLIST_ADD_NODE);
        let p = read_usize(this + staff_records_fields::CLUB_HISTORY_LISTS).wrapping_add(read_usize(club).wrapping_mul(8));
        f(read_usize(this + staff_records_fields::CLUB_HISTORY_FILE), p, p + 4, e.as_ptr());
    }
}

/// Queues the trophy praise media story (0x75c390 on G_MEDIA); in rehearsal only marks the static date as loose.
///
/// [pt] A notícia do elogio do título (0x75c390 em [0xb6b7c8]; fora do ensaio).
pub(crate) unsafe fn post_trophy_praise(club: usize, comp: usize, mark: u32) {
    if STAFF_HISTORY_DRY_RUN {
        DRY_RUN_STRAY_SCRATCH_DATE = true;
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_MEDIA_QUEUE_TROPHY_PRAISE_STORY);
    // 0x75c390: push marca; push comp; push clube; mov ecx, [0xb6b7c8] -> thiscall (ret 0xc)
    f(read_usize(G_MEDIA), club, comp, mark);
}

/// Manager rating increments (k1, k2) by competition stage: 400/400, or 200 (foreign manager) / 200 in a stage-1
/// competition (+0x42 = 1).
///
/// [pt] As notas k1/k2 do técnico pela fase: na competição de fase 1 ([0xadadfc] + 4 × id, +0x42 = 1) 200 na terceira e 200
/// na segunda se o técnico for estrangeiro (0x53d980 com a nação +0x53 e 1), senão 400.
pub(crate) unsafe fn manager_rating_gains(t: usize, club: usize, id_comp: usize) -> (i32, i32) {
    let c = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * id_comp));
    if *((c + league::CLUB_SIDES) as *const i8) != 1 {
        (400, 400)
    } else {
        let estrangeiro: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
        // 0x53d980: 3 pushes; add esp, 0xc -> cdecl
        (if estrangeiro(t, read_usize(club + club::NATION), 1) != 0 { 200 } else { 400 }, 200)
    }
}

/// `Staff_Records::add_league_champions_record(club, comp)`: records the league champion. SEH frame only for the local
/// dates. Called by the league champions at the end of each league (25 callers, one per league; 4 calls up to
/// 2002-06-01).
///
/// Null club or competition: the error box (line 0x21c); the competition without an object in [0xadadfc] (+ 4 × id):
/// the error box (0x224). The reputation (up to weight × 500, +750 at most); with +0x57 and the club's +0x69 block:
/// +0x7b goes up weight × 20 (below 150000 and below +0x73 + 500) and the block's +0x44 by 25 × (weight + 20) (below
/// 120000). The players' threshold: ftol((+0x3e − 1) × the object's +0x3c × 0.33), in 16 bits.
///
/// With a manager (+0xcf): his three ratings (+0xa weight × 400, +8 weight × k1, +0xc weight × k2 in the +0x69 record,
/// if any) and the history (type 5: the club and the competition); his record (0x86f5b0 with create): +1 in +0x8c and
/// weight × 5000 in +0xb6, +0xca and +0xba. The chairman (+0xbf).
///
/// The 50 in the squad (+0xd7) with a player sheet, valid, of the club (+0x39) and other than the manager: the average
/// 2/1 (0x7c1e30 with 6.5) above the threshold gives the history, morale +20 (0x544710) and +100 on the contract
/// (0x4cf5f0) with the renewal; otherwise morale +10 and +50 on the contract. The club's history (type 5); with the
/// media ([0xb6b7c8]) and weight above 3, the praise (0x75c390 with 1) and, above 12, with 3.
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
pub(crate) unsafe extern "thiscall" fn add_league_champions_record(this: usize, club: usize, comp: usize) {
    if club == 0 || comp == 0 {
        return show_error_at(0xa83b78, 0x21c);
    }
    let obj = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_usize(comp)));
    if obj == 0 {
        return show_error_at(0xa83b78, 0x224);
    }
    let weight = read_i16(comp + competition::REPUTATION);
    raise_champion_reputation(club, weight, true); // cqd13: the reputation of the league champion
    if read_usize(club + club::DIVISION) != 0 {
        let b = read_usize(club + club::STADIUM);
        if b != 0 {
            let x = read_i32(club + club::MAX_ATTENDANCE);
            if x < 150000 && x < read_i32(club + club::ATTENDANCE).wrapping_add(500) {
                write_i32(club + club::MAX_ATTENDANCE, x.wrapping_add((weight as i32).wrapping_mul(20)));
            }
            let y = read_i32(b + stadium::EXPANSION_CAPACITY);
            if y < 120000 {
                write_i32(b + stadium::EXPANSION_CAPACITY, y.wrapping_add(((weight as i32).wrapping_add(20)).wrapping_mul(25)));
            }
        }
    }
    let n = ((read_i16(obj + league::CLUB_COUNT) as i32) - 1).wrapping_mul(read_i16(obj + league::LEGS) as i32);
    let threshold = ftol(n as f64 * read_f64_at(0x968f78)) as i16; // the ftol stays in the word (movsx word afterwards); 0x968f78 = 0.33
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 5;
    put_i32(&mut e, 8, read_i32(club));
    put_i32(&mut e, 0xd, read_i32(comp));
    let t = read_usize(club + club::MANAGER);
    if t != 0 {
        let np = read_usize(t + staff::NON_PLAYER);
        if np != 0 {
            let (k1, k2) = manager_rating_gains(t, club, read_usize(comp));
            raise_side_rating(np + non_player::CURRENT_REPUTATION, weight, 400);
            raise_side_rating(np + non_player::HOME_REPUTATION, weight, k1);
            raise_side_rating(np + non_player::WORLD_REPUTATION, weight, k2);
        }
        append_person_history_by_id(!STAFF_HISTORY_DRY_RUN, this, read_usize(t), &e);
        let record: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
        // 0x86f5b0: push 1; push manager; mov ecx, edi (the this) -> thiscall (ret 8)
        let r = record(this, read_usize(club + club::MANAGER), 1);
        if r != 0 {
            core::ptr::write_unaligned((r + manager_record::LEAGUES_WON) as *mut i16, read_i16(r + manager_record::LEAGUES_WON).wrapping_add(1));
            let v = (weight as i32).wrapping_mul(5000);
            write_i32(r + manager_record::YEAR_POINTS, read_i32(r + manager_record::YEAR_POINTS).wrapping_add(v));
            write_i32(r + manager_record::MONTH_POINTS, read_i32(r + manager_record::MONTH_POINTS).wrapping_add(v));
            write_i32(r + manager_record::SEASON_POINTS, read_i32(r + manager_record::SEASON_POINTS).wrapping_add(v));
        }
    }
    let chairman = read_usize(club + club::CHAIRMAN);
    if chairman != 0 {
        append_person_history_by_id(!STAFF_HISTORY_DRY_RUN, this, read_usize(chairman), &e);
    }
    let average: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
    let morale: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_STAFF_ADJUST_MORALE);
    let contract: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    // 0x7c1e30: push 6,5f; push 1; push 2; push ficha; mov ecx, [0xb74350] -> thiscall (ret 0x10, x87); 0x544710: 3
    // pushes; add esp, 0xc -> cdecl; 0x4cf5f0: push clube; push p; mov ecx, [0xae1050] -> thiscall (ret 8)
    for j in 0..50usize {
        let p = read_usize(club + club::SQUAD + 4 * j);
        if p == 0 || read_usize(p + staff::PLAYER) == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) || read_usize(p + staff::CLUB_JOB) != club
            || p == read_usize(club + club::MANAGER)
        {
            continue;
        }
        let f = average(read_usize(G_PLAYER_STATS), read_usize(p + staff::PLAYER), 2, 1, 6.5);
        if f > threshold as f64 { // cqd13: acima do limiar
            crate::comparison::branch("cqd_13 jogador destacado do campeao");
            append_person_history_by_id(!STAFF_HISTORY_DRY_RUN, this, read_usize(p), &e);
            morale(p, 0x14, 1); // cqd13: the morale of the standout player
            let k = contract(read_usize(G_CONTRACT_MANAGER), p, club);
            if k == 0 {
                continue;
            }
            crate::comparison::branch("cqd_13 contrato do destacado");
            add_contract_morale(k, 100); // cqd13: the contract of the standout player
            offer_renewal_if_due(club, p, k);
        } else {
            morale(p, 0xa, 1);
            let k = contract(read_usize(G_CONTRACT_MANAGER), p, club);
            if k != 0 {
                add_contract_morale(k, 50);
            }
        }
    }
    add_club_history(this, club, 5, comp);
    if read_usize(G_MEDIA) != 0 && weight > 3 {
        post_trophy_praise(club, comp, 1);
        if weight > 12 {
            post_trophy_praise(club, comp, 3);
        }
    }
}

/// `Staff_Records::add_cup_winners_record(club, fixture, match)`: records the cup winner. SEH frame only for the local
/// dates. Called by the cup finals (29 callers; 11 calls up to 2002-06-01).
///
/// Nulls: the error box (line 0x51e). The reputation (as in the league); with a manager (+0xcf) who has a non-player
/// record (+0x69): the three ratings of the club's side in the match (+0x188/+0x18a/+0x18c, side 0x625 × away; weight ×
/// 400, × k1, × k2); the manager's history (type 0: the club and the competition of the fixture's +0x14); his record
/// (0x86f5b0 with create): +1 in +0x8a and ftol(weight^4 × 0.5) (CRT pow 0x945ff0) in +0xb6, +0xca and, for a national
/// team (0x53b900), +0xbe, otherwise +0xba. The chairman (+0xbf) and the 20 of the line-up.
///
/// The 50 in the squad while the reputation does not exceed weight × 400: with a player sheet, valid and other than the
/// manager, morale +15 (0x544710) and +100 on the contract; with contract status (+0x4f >> 4) 1-3 or 5, the renewal.
/// The call-ups of the stored competition (0x76d940) end (0x774f30); the club's history (type 0); with the media and
/// weight above 10, the praise (0x75c390 with 1) and, with weight above 12 or reputation below 2500, with 3.
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
pub(crate) unsafe extern "thiscall" fn add_cup_winners_record(this: usize, club: usize, game: usize, partida: usize) {
    if club == 0 || game == 0 || partida == 0 {
        return show_error_at(0xa83b78, 0x51e);
    }
    let comp = read_usize(game + fixture::COMPETITION);
    let weight = read_i16(comp + competition::REPUTATION);
    raise_champion_reputation(club, weight, false);
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0;
    put_i32(&mut e, 8, read_i32(club));
    put_i32(&mut e, 0xd, read_i32(comp));
    let t = read_usize(club + club::MANAGER);
    if t != 0 {
        if read_usize(t + staff::NON_PLAYER) != 0 {
            let (k1, k2) = manager_rating_gains(t, club, read_usize(game));
            let side = partida + match_record::HOME_MANAGER_CURRENT_REPUTATION + 0x625 * (read_usize(game + fixture::AWAY_CLUB) == club) as usize;
            raise_side_rating(side, weight, 400);
            raise_side_rating(side + 2, weight, k1);
            raise_side_rating(side + 4, weight, k2);
        }
        append_person_history_by_id(!STAFF_HISTORY_DRY_RUN, this, read_usize(read_usize(club + club::MANAGER)), &e);
        let record: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
        let r = record(this, read_usize(club + club::MANAGER), 1);
        if r != 0 {
            core::ptr::write_unaligned((r + manager_record::CUPS_WON) as *mut i16, read_i16(r + manager_record::CUPS_WON).wrapping_add(1));
            // fild weight; fld 4.0; _CIpow (base in st1); fmul 0.5; ftol
            let v = ftol(game_pow(read_i16(comp + competition::REPUTATION) as f64, read_f64_at(0x968938)) * read_f64_at(0x968968)); // cqd13: the title weight; 0x968938 = 4, 0x968968 = 0.5
            write_i32(r + manager_record::YEAR_POINTS, read_i32(r + manager_record::YEAR_POINTS).wrapping_add(v));
            write_i32(r + manager_record::MONTH_POINTS, read_i32(r + manager_record::MONTH_POINTS).wrapping_add(v));
            if is_national_team(club) != 0 {
                write_i32(r + manager_record::NATIONAL_SEASON_POINTS, read_i32(r + manager_record::NATIONAL_SEASON_POINTS).wrapping_add(v));
            } else {
                write_i32(r + manager_record::SEASON_POINTS, read_i32(r + manager_record::SEASON_POINTS).wrapping_add(v));
            }
        }
    }
    let chairman = read_usize(club + club::CHAIRMAN);
    if chairman != 0 {
        append_person_history_by_id(!STAFF_HISTORY_DRY_RUN, this, read_usize(chairman), &e);
    }
    let escalacao = partida + match_record::ROW_STAFF_ID + 0x625 * (club != read_usize(game + fixture::HOME_CLUB)) as usize;
    add_lineup_histories(this, club, escalacao, &e);
    let morale: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_STAFF_ADJUST_MORALE);
    let contract: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    for j in 0..50usize {
        if (read_i16(comp + competition::REPUTATION) as i32).wrapping_mul(400) < read_i16(club + club::REPUTATION) as i32 {
            break;
        }
        let p = read_usize(club + club::SQUAD + 4 * j);
        if p == 0 || read_usize(p + staff::PLAYER) == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) || p == read_usize(club + club::MANAGER) {
            continue;
        }
        crate::comparison::branch("cqd_13 elenco do campeao da copa");
        morale(p, 0xf, 1); // cqd13: the morale of the cup winner player
        let k = contract(read_usize(G_CONTRACT_MANAGER), p, club);
        if k == 0 {
            continue;
        }
        crate::comparison::branch("cqd_13 moral do elenco campeao da copa");
        add_contract_morale(k, 100); // cqd13: the contract morale of the cup winner
        let s = *((k + contract::SQUAD_STATUS) as *const u8) >> 4;
        if s == 0 || (s > 3 && s != 5) {
            continue;
        }
        offer_renewal_if_due(club, p, k);
    }
    let stored: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NATIONAL_TEAMS_IS_FINALS_COMP);
    // 0x76d940 and 0x774f30: push comp; mov ecx, [0xb74318] (0x76d940 does not read ecx and only compares the id; ret 4 in both)
    if stored(read_usize(G_NATIONAL_TEAMS), read_usize(game + fixture::COMPETITION)) != 0 {
        if STAFF_HISTORY_DRY_RUN {
            DRY_RUN_STRAY_SCRATCH_DATE = true;
        } else {
            let acabar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_END_DUTY_FOR_COMPETITION);
            acabar(read_usize(G_NATIONAL_TEAMS), read_usize(game + fixture::COMPETITION));
        }
    }
    add_club_history(this, club, 0, comp);
    if read_usize(G_MEDIA) != 0 && weight > 10 {
        post_trophy_praise(club, comp, 1);
        if read_i16(comp + competition::REPUTATION) > 12 || read_i16(club + club::REPUTATION) < 0x9c4 {
            post_trophy_praise(club, comp, 3);
        }
    }
}

/// The dry-run comparer of the two champions (league and cup).
///
/// The manager's record is created beforehand (`0x86f5b0` with create, as the original routine does), the copy runs as
/// a dry run (without histories, news, call-ups and renewal; the morale random draws `0x544710` not counted) and the
/// original routine from the same state.
///
/// It compares: the club's reputation, the manager's record (0xd2 bytes), the ratings (the side's three in the match,
/// or `+8`..`+0xd` of the manager's non-player record), the club's `+0x7b` and `+0x44` of block `+0x69` (in the
/// league), the morale (`+0x45` of the player sheet) and the contract's morale (`+0x44`) of the 50 in the squad, and
/// the static date of `0x54cdb0`.
///
/// The morale of the contracts the renewal would touch (the original renews) and the random-draw state (the original's
/// news makes random draws) are not counted; the random draw state is put back to the previous one for the original.
/// The original's renewal also makes random draws and counts days (`0x54cdb0`): the morale of the player sheets after
/// the first renewed one is left out of the equality and checked against the range of `0x544710`; the static date is
/// left out when the dry run skipped, after the copy's last day count, an original call that counts days
/// (DRY_RUN_STRAY_SCRATCH_DATE). Seen in ENG in May 2002 and in the 8-league audit.
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
pub(crate) unsafe fn compare_champion_update(name: &str, this: usize, club: usize, notes: (usize, usize), league: bool, original: impl FnOnce(), copy: impl FnOnce()) {
    let mut fixas: Vec<(usize, usize)> = vec![(club + club::REPUTATION, 2), (G_DAYS_SINCE_SCRATCH_DATE, 8), notes];
    let t = read_usize(club + club::MANAGER);
    if t != 0 {
        let record: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
        let r = record(this, t, 1);
        if r != 0 {
            fixas.push((r, 0xd2));
        }
    }
    if league && read_usize(club + club::DIVISION) != 0 && read_usize(club + club::STADIUM) != 0 {
        fixas.push((club + club::MAX_ATTENDANCE, 4));
        fixas.push((read_usize(club + club::STADIUM) + 0x44, 4));
    }
    let contract: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let mut contratos = Vec::new();
    let mut sheets = Vec::new(); // (morale from the record, contract or 0, previous morale), in squad order
    for j in 0..50usize {
        let p = read_usize(club + club::SQUAD + 4 * j);
        if p == 0 || read_usize(p + staff::PLAYER) == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        let k = contract(read_usize(G_CONTRACT_MANAGER), p, club);
        let m = read_usize(p + staff::PLAYER) + 0x45;
        sheets.push((m, k, *(m as *const i8)));
        if k != 0 {
            contratos.push(k);
        }
    }
    let mut todas = fixas.clone();
    todas.extend(sheets.iter().map(|&(m, _, _)| (m, 1)));
    todas.extend(contratos.iter().map(|&k| (k + 0x44, 1)));
    todas.extend(crate::rng::STATE_REGIONS.iter().copied());
    let before = crate::comparison::MemorySnapshot::read(&todas);
    (*core::ptr::addr_of_mut!(DRY_RUN_RENEWED_CONTRACTS)).clear();
    DRY_RUN_STRAY_SCRATCH_DATE = false;
    STAFF_HISTORY_DRY_RUN = true;
    crate::rng::without_counting(copy);
    STAFF_HISTORY_DRY_RUN = false;
    let renovados = core::mem::take(&mut *core::ptr::addr_of_mut!(DRY_RUN_RENEWED_CONTRACTS));
    let mut count = fixas;
    if DRY_RUN_STRAY_SCRATCH_DATE {
        count.retain(|&(a, _)| a != G_DAYS_SINCE_SCRATCH_DATE);
    }
    // The original’s renewal rolls (0x4cda70): the players’ morale after the first renewed one (0x544710 rolls)
    // comes from another point of the random generator; his and the previous ones enter the count.
    let cut = renovados.first().and_then(|r| sheets.iter().position(|&(_, k, _)| k == *r)).unwrap_or(sheets.len());
    count.extend(sheets.iter().take(cut + 1).map(|&(m, _, _)| (m, 1)));
    count.extend(contratos.iter().filter(|k| !renovados.contains(k)).map(|&k| (k + 0x44, 1)));
    // The later ones, by the range of 0x544710: morale m rises from 0 to 20 (+20 or +10 drawn), clamped to 1..20.
    let band = |&(m, _, m0): &(usize, usize, i8)| -> bool {
        let (m0, v) = (m0 as i32, *(m as *const i8) as i32);
        v >= m0.clamp(1, 20) && v <= (m0 + 20).clamp(1, 20)
    };
    let mut in_band = sheets.iter().skip(cut + 1).all(band);
    let mr = crate::comparison::MemorySnapshot::read(&count);
    before.write_back();
    original();
    in_band &= sheets.iter().skip(cut + 1).all(band);
    let mo = crate::comparison::MemorySnapshot::read(&count);
    SCOREBOARD.record(mo == mr && in_band, || format!("{}(clube {:#x}) renovados {} (o primeiro na ficha {} de {}; fora da band: {}) original={:x?} rust={:x?}",
        name, club, renovados.len(), cut, sheets.len(), !in_band, mo, mr));
}

/// `Staff_Records::clear_human_manager_record(person)`: clears a human manager's record. Called by the human manager's
/// retirement (0x6b2b40) and by setup (0x82b9xx).
///
/// A person among the 16 humans (id from [0xae2364] − 16 to [0xae2364]): his record in vector +8 (slot id − [0xae2364]
/// + [0xae2368] + 16, 0xd2 bytes); otherwise, as in the original, the person itself. Zeroes the 0xd2 bytes and puts −1
/// in +0x86, +0xc2, +0xc6, +0x4a, +0x5e and +0x72.
/// Original: `Staff_Records::clear_human_manager_record` (0x86fc70, staff_records.cpp, thiscall).
///
/// [pt] { 0x86fc70 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::clear_human_manager_record(pessoa)`, pela
/// aposentadoria do técnico humano (0x6b2b40) e pelo setup (0x82b9xx). Pessoa entre os 16 humanos (id de [0xae2364] − 16
/// até [0xae2364]): o registro dele no vetor +8 (vaga id − [0xae2364] + [0xae2368] + 16, 0xd2 bytes); senão, como a
/// original, a própria pessoa. Zera os 0xd2 bytes e põe −1 em +0x86, +0xc2, +0xc6, +0x4a, +0x5e e +0x72.
pub(crate) unsafe extern "thiscall" fn clear_human_manager_record(this: usize, person: usize) {
    let id = read_i32(person);
    let lim = read_i32(G_STAFF_COUNT);
    let r = if id >= lim.wrapping_sub(0x10) && id < lim {
        let slot = id.wrapping_sub(lim).wrapping_add(read_i32(G_NON_PLAYER_COUNT)).wrapping_add(0x10);
        read_usize(this + staff_records_fields::MANAGER_RECORDS).wrapping_add((slot as usize).wrapping_mul(0xd2))
    } else {
        person
    };
    core::ptr::write_bytes(r as *mut u8, 0, 0xd2);
    for o in [0x86usize, 0xc2, 0xc6, 0x4a, 0x5e, 0x72] {
        write_i32(r + o, -1);
    }
}

/// Adds 1 to the i16 at address a.
pub(crate) unsafe fn increment_i16(a: usize) {
    core::ptr::write_unaligned(a as *mut i16, read_i16(a).wrapping_add(1));
}

/// Returns the club manager's points record (FN_STAFF_RECORDS_GET_MANAGER_RECORD with create), or 0 without a manager.
///
/// [pt] O registro de pontos do técnico do clube (+0xcf, lido sem conferir o clube; 0x86f5b0 com criar), ou 0 sem técnico.
pub(crate) unsafe fn club_manager_record(this: usize, club: usize) -> usize {
    let t = read_usize(club + club::MANAGER);
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

/// The match rating for the manager.
///
/// The importance (0x48e190 of the match's competition, with `lado` and the output in `saida`; only the byte, signed) ×
/// (opponent reputation / 500) × the competition weight (+0x69 of [+0x14]) × (points + bonus) × 50, divided by 100000
/// (signed divisions truncated, like the original's magic multiplications; products in 32 bits).
///
/// [pt] A nota do jogo para o técnico: a importância (0x48e190 da competição do jogo, com `lado` e a saída em `saida`; só o
/// byte, com sinal) × (reputação do adversário / 500) × o peso da competição (+0x69 de [+0x14]) × (pontos + bônus) × 50,
/// dividido por 100000 (divisões com sinal truncadas, como as multiplicações mágicas da original; produtos em 32 bits).
pub(crate) unsafe fn match_rating_for_manager(p: usize, side: u32, out: &mut u32, opponent: usize, points: i32) -> i32 {
    let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_usize(p)));
    let importancia: unsafe extern "thiscall" fn(usize, usize, u32, *mut u32) -> u32 = core::mem::transmute(FN_COMP_GET_FIXTURE_MATCH_IMPORTANCE);
    let i = importancia(comp, p, side, out as *mut u32) as u8 as i8 as i32;
    let x = i
        .wrapping_mul(read_i16(opponent + club::REPUTATION) as i32 / 500)
        .wrapping_mul(read_i16(read_usize(p + fixture::COMPETITION) + 0x69) as i32)
        .wrapping_mul(points)
        .wrapping_mul(50);
    x / 100000
}

/// `Staff_Records::record_match_result(fixture, match)`: records a match in both managers' records. Called by the
/// closing of the day's match (0x6b5f00), for each game.
///
/// Match not played (+0x3a = 1): nothing. The records of the two managers (+0xcf of the home side +0x1c and of the away
/// side +0x20; 0x86f5b0 with create): +1 game (+0x8e); goals for (+0x96) and against (+0x9a) from the score
/// (+0x47/+0x48, or +0x43/+0x44 with +0x47 = −1); win (+0x90, bonus 900), draw (+0x92, 300 for both) or loss (+0x94).
/// Each one's rating (0x48e190 with 0 / 1, the opponent's reputation, the match points at +0x11f.. / +0x744..;
/// importance × opponent reputation × weight × points) goes to +0xb6 and +0xca (month and year); in the club's league
/// (+0x57 or +0x60 = the competition +0x14) also to the month's points (+0xce); for a national team (0x53b900) to
/// +0xbe, otherwise +0xba (season).
///
/// Defect of the original, reproduced: for the away manager, +0xbe/+0xba receive the routine's [esp+0x10], which is the
/// home manager's rating (or his bonus, without a record).
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
    if *((p + fixture::CATEGORY) as *const u8) == 1 {
        return;
    }
    let (casa, fora) = (read_usize(p + fixture::HOME_CLUB), read_usize(p + fixture::AWAY_CLUB));
    let a = club_manager_record(this, casa);
    let v = club_manager_record(this, fora);
    if a != 0 {
        increment_i16(a + manager_record::GAMES);
    }
    if v != 0 {
        increment_i16(v + manager_record::GAMES);
    }
    let (ga, gv) = if *((p + fixture::HOME_GOALS_EXTRA_TIME) as *const u8) == 0xff {
        (read_i8_as_i32(p + fixture::HOME_GOALS), read_i8_as_i32(p + fixture::AWAY_GOALS))
    } else {
        (read_i8_as_i32(p + fixture::HOME_GOALS_EXTRA_TIME), read_i8_as_i32(p + fixture::AWAY_GOALS_EXTRA_TIME))
    };
    if a != 0 {
        add_to_i32(a + manager_record::GOALS_FOR, ga);
        add_to_i32(a + manager_record::GOALS_AGAINST, gv);
    }
    if v != 0 {
        add_to_i32(v + manager_record::GOALS_FOR, gv);
        add_to_i32(v + manager_record::GOALS_AGAINST, ga);
    }
    let (mut ba, mut bv) = (0i32, 0i32);
    if ga > gv {
        ba = 900; // cqa11: the win bonus of the home team
        if a != 0 {
            crate::comparison::branch("cqa_11 vitoria do mandante");
            increment_i16(a + manager_record::WINS);
        }
        if v != 0 {
            increment_i16(v + manager_record::LOSSES);
        }
    } else if ga < gv {
        bv = 900;
        if v != 0 {
            increment_i16(v + manager_record::WINS);
        }
        if a != 0 {
            increment_i16(a + manager_record::LOSSES);
        }
    } else {
        ba = 300;
        bv = 300;
        if a != 0 {
            increment_i16(a + manager_record::DRAWS);
        }
        if v != 0 {
            increment_i16(v + manager_record::DRAWS);
        }
    }
    // the output of 0x48e190 goes in the slot of the original’s `jogo` argument, which starts with the match itself
    let mut out = p as u32;
    let mut stored = ba; // the original’s [esp+0x10]: the home side’s bonus, then its rating
    let comp = read_usize(p + fixture::COMPETITION);
    if a != 0 {
        crate::comparison::branch("cqa_11 nota do mandante");
        let q = match_rating_for_manager(p, 0, &mut out, fora, side_match_points(m, 0x11f, 0x128, 0x127, 0x129).wrapping_add(ba));
        add_to_i32(a + manager_record::YEAR_POINTS, q);
        add_to_i32(a + manager_record::MONTH_POINTS, q);
        stored = q;
        if comp == read_usize(casa + club::DIVISION) || comp == read_usize(casa + club::RESERVE_DIVISION) {
            crate::comparison::branch("cqa_11 pontos do mes");
            add_to_i32(a + manager_record::MONTH_LEAGUE_POINTS, q); // cqa11: the month points of the home team
        }
        if is_national_team(casa) != 0 {
            add_to_i32(a + manager_record::NATIONAL_SEASON_POINTS, q);
        } else {
            add_to_i32(a + manager_record::SEASON_POINTS, q);
        }
    }
    if v != 0 {
        let q = match_rating_for_manager(p, 1, &mut out, casa, side_match_points(m, 0x744, 0x74d, 0x74c, 0x74e).wrapping_add(bv));
        add_to_i32(v + manager_record::YEAR_POINTS, q);
        add_to_i32(v + manager_record::MONTH_POINTS, q);
        if comp == read_usize(fora + club::DIVISION) || comp == read_usize(fora + club::RESERVE_DIVISION) {
            add_to_i32(v + manager_record::MONTH_LEAGUE_POINTS, q);
        }
        if is_national_team(fora) != 0 {
            add_to_i32(v + manager_record::NATIONAL_SEASON_POINTS, stored);
        } else {
            add_to_i32(v + manager_record::SEASON_POINTS, stored);
        }
    }
}

/// `Staff_Records::add_league_finish_records(league)`: at the end of the league, each club's final position in its
/// manager's history. SEH frame only for the local date. Called by the completed league (0x689ad0).
///
/// For each row of the table (0x6826d0, up to +0x3e), the history of the club's manager (+0xcf; type 0xa: the club id,
/// the competition of +4, the year +0x3a and the position k + 1), in the archive (0x58e340).
/// Original: `Staff_Records::add_league_finish_records` (0x86a070, staff_records.cpp, thiscall).
///
/// [pt] { 0x86a070 } (thiscall, ret 4, staff_records.cpp, SEH só da data local): `Staff_Records::add_league_finish_records(
/// liga)`, pela liga completa (0x689ad0), no fim da liga. Para cada linha da tabela (0x6826d0, até +0x3e), o histórico do
/// técnico do clube (+0xcf; tipo 0xa: o id do clube, a competição de +4, o ano +0x3a e a posição k + 1), no arquivo
/// (0x58e340).
pub(crate) unsafe extern "thiscall" fn add_league_finish_records(this: usize, league: usize) {
    crate::comparison::branch("cqa_11 fim da liga");
    let row: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FN_LEAGUE_GET_TABLE_ROW);
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0xa;
    put_i32(&mut e, 0x11, read_i16(league + league::STAGE_ID) as i32);
    put_i32(&mut e, 0xd, read_i32(read_usize(league + 4)));
    let mut k: i16 = 0;
    while k < read_i16(league + league::CLUB_COUNT) {
        let club = read_usize(row(league, k as i32));
        put_i32(&mut e, 0x15, k as i32 + 1);
        put_i32(&mut e, 8, read_i32(club));
        let t = read_usize(club + club::MANAGER);
        if t != 0 {
            append_person_history_by_id(true, this, read_usize(t), &e);
        }
        k += 1;
    }
}

// simplification: only writes the history to the file (does not run twice): original only; the copy takes the branch.

/// O ensaio do comparador da 0x86dc30: a cópia sem o histórico (arquivo).
pub(crate) static mut SUPPORTERS_PLAYER_OF_YEAR_DRY_RUN: bool = false;

/// `Staff_Records::add_supporters_player_of_year_record(person, club, a, b)`: the supporters' player of the year. SEH
/// frame only for the local date. Called by the end of season of the records (0x447880).
///
/// The person's history (type 0x35: the club id, `a` as unsigned 16 bits and `b`), in the archive (0x58e340); then, if
/// draw(21 − the person's +0x59) < draw(10), the person likes the club (0x547d90 with draw(2)) and the club likes him
/// (0x547870 with draw(2)). Null person: the original (the error box and the read of the null person's +0x59).
/// Original: `Staff_Records::add_supporters_player_of_year_record` (0x86dc30, staff_records.cpp, thiscall).
///
/// [pt] { 0x86dc30 } (thiscall, ret 0x10, staff_records.cpp, SEH só da data local): `Staff_Records::
/// add_supporters_player_of_year_record(pessoa, clube, a, b)`, pelo fim da temporada dos recordes (0x447880). O histórico
/// da pessoa (tipo 0x35: o id do clube, `a` em 16 bits sem sinal e `b`), no arquivo (0x58e340); depois, se sorteio(21 −
/// +0x59 da pessoa) < sorteio(10), a pessoa gosta do clube (0x547d90 com sorteio(2)) e o clube dela (0x547870 com
/// sorteio(2)). Pessoa nula: a original (a caixa e a leitura de +0x59 da pessoa nula).
pub(crate) unsafe extern "thiscall" fn add_supporters_player_of_year_record(this: usize, person: usize, club: usize, a: u32, b: u32) {
    if person == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_ADD_SUPPORTERS_PLAYER_OF_YEAR_RECORD)(this, person, club, a, b);
    }
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x35;
    put_i32(&mut e, 8, read_i32(club));
    put_i32(&mut e, 0xd, (a & 0xffff) as i32);
    put_i32(&mut e, 0x11, b as i32);
    append_person_history_by_id(!SUPPORTERS_PLAYER_OF_YEAR_DRY_RUN, this, read_usize(person), &e);
    let r1 = friendly_random(0x15 - read_i8_as_i32(person + staff::LOYALTY));
    if r1 < friendly_random(10) {
        crate::comparison::branch("cqa_11 goal_crowd_reaction favorita");
        let gosta: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_FAVOURED_CLUB);
        let favorita: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_CLUB_ADD_FAVOURED_STAFF);
        gosta(person, club, friendly_random(2) as u32);
        favorita(club, person, friendly_random(2) as u32);
    }
}

/// The comparers' rehearsal: the copies run without the history (file) and without ending the call-ups.
///
/// [pt] O ensaio dos comparadores: as cópias sem o histórico (arquivo) e sem acabar as convocações.
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
    let p = read_usize(this + staff_records_fields::CLUB_HISTORY_LISTS).wrapping_add(id.wrapping_mul(8));
    f(read_usize(this + staff_records_fields::CLUB_HISTORY_FILE), p, p + 4, e.as_ptr());
}

/// Raises a side rating word by ftol(weight × k × [0x96ff88]), floored at weight × k and capped at 10000.
///
/// [pt] Uma nota do lado do clube na partida (palavra em `a`): soma ftol(peso × k × [0x96ff88]) (produto em 32 bits) e fica
/// no piso peso × k (16 bits) quando está abaixo dele, senão no máximo 10000 (comparações com sinal em 16 bits).
pub(crate) unsafe fn raise_side_rating(a: usize, weight: i16, k: i32) {
    let sum = ftol((weight as i32).wrapping_mul(k) as f64 * read_f64_at(0x96ff88)); // 0x96ff88 = 0.1639344262
    let x = read_i16(a).wrapping_add(sum as i16);
    write_i16(a, x);
    let floor = weight.wrapping_mul(k as i16);
    write_i16(a, if floor > x { floor } else if x > 10000 { 10000 } else { x });
}

/// The cup runner-up (type 1: threshold 500, cap 350, ratings 300/150) and third place (type 2: 500, 200, 200/100; the
/// 500 threshold verified at 0x86b774).
///
/// The club's reputation (+0x80) rises up to the competition's weight (+0x69 of [match+0x14]) × threshold, at most
/// `teto` at a time (between 1 and 10000); with a manager (+0xcf) who has a non-player record (+0x69), the three
/// ratings of the club's side in the match (+0x188/+0x18a/+0x18c, side 0x625 × away): weight × k, and in a stage-1
/// competition ([0xadadfc] +0x42 = 1) weight × half in the third and in the second if the manager is foreign (0x53d980
/// with nation +0x53 and 1).
///
/// The histories (date, type, club id, competition): of the manager, the chairman (+0xbf) and the 20 of the side's
/// line-up (+0x190, 0x49 bytes, −1 empty; valid persons and excluding the manager); for third place, the call-ups end
/// in the stored competitions (0x76d940, 0x774f90 with both clubs); finally the club history (type at +8, competition
/// at +9).
///
/// [pt] O vice (tipo 1: limiar 500, teto 350, notas 300/150) e o terceiro (tipo 2: 500, 200, 200/100; o limiar 500 conferido em 0x86b774) da copa: a reputação do
/// clube (+0x80) sobe até peso da competição (+0x69 de [jogo+0x14]) × limiar, no máximo `teto` de uma vez (entre 1 e
/// 10000); com técnico (+0xcf) que tem registro de não jogador (+0x69), as três notas do lado do clube na partida
/// (+0x188/+0x18a/+0x18c, lado 0x625 × visitante): peso × k, e na competição de fase 1 ([0xadadfc] +0x42 = 1) peso ×
/// metade na terceira e na segunda se o técnico for estrangeiro (0x53d980 com a nação +0x53 e 1). Os históricos (data,
/// tipo, id do clube, competição): do técnico, do presidente (+0xbf) e dos 20 da escalação do lado (+0x190, 0x49 bytes,
/// −1 vazio; pessoas válidas e fora o técnico); no terceiro, as convocações acabam nas competições guardadas (0x76d940,
/// 0x774f90 com os dois clubes); por fim o histórico do clube (tipo no +8, competição no +9).
pub(crate) unsafe fn update_cup_placed_club(this: usize, club: usize, game: usize, partida: usize, kind: u8) {
    let (limiar, teto, k, metade) = if kind == 1 { (500, 350, 300, 150) } else { (500, 200, 200, 100) };
    let comp = read_usize(game + fixture::COMPETITION);
    let weight = read_i16(comp + competition::REPUTATION);
    let target = (weight as i32).wrapping_mul(limiar); // cqa12: the reputation threshold
    let rep = read_i16(club + club::REPUTATION) as i32;
    if rep < target {
        let foul = target - rep;
        let sum = if foul > teto { teto } else { foul }; // cqa12: the rise cap
        let v = read_i16(club + club::REPUTATION).wrapping_add(sum as i16);
        let v = if v < 1 { 1 } else if v > 10000 { 10000 } else { v };
        if kind == 1 {
            crate::comparison::branch("cqa_12 injury_word_0c do vice");
        } else {
            crate::comparison::branch("cqa_12 injury_word_0c do terceiro");
        }
        write_i16(club + club::REPUTATION, v); // cqa12: the reputation of the placed team
    }
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = kind;
    put_i32(&mut e, 8, read_i32(club));
    put_i32(&mut e, 0xd, read_i32(comp));
    let t = read_usize(club + club::MANAGER);
    if t != 0 {
        if read_usize(t + staff::NON_PLAYER) != 0 {
            let c = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_usize(game)));
            let (k1, k2) = if *((c + league::CLUB_SIDES) as *const i8) != 1 {
                (k, k)
            } else {
                let estrangeiro: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
                (if estrangeiro(t, read_usize(club + club::NATION), 1) != 0 { metade } else { k }, metade)
            };
            let side = partida + match_record::HOME_MANAGER_CURRENT_REPUTATION + 0x625 * (read_usize(game + fixture::AWAY_CLUB) == club) as usize;
            raise_side_rating(side, weight, k);
            raise_side_rating(side + 2, weight, k1);
            raise_side_rating(side + 4, weight, k2);
        }
        append_person_history_by_id(!CUP_PLACED_CLUB_DRY_RUN, this, read_usize(t), &e);
    }
    let chairman = read_usize(club + club::CHAIRMAN);
    if chairman != 0 {
        append_person_history_by_id(!CUP_PLACED_CLUB_DRY_RUN, this, read_usize(chairman), &e);
    }
    let escalacao = partida + match_record::ROW_STAFF_ID + 0x625 * (club != read_usize(game + fixture::HOME_CLUB)) as usize;
    for j in 0..0x14usize {
        let id = read_i32(escalacao + 0x49 * j);
        if id == -1 {
            continue;
        }
        let p = read_usize(G_STAFF_TABLE).wrapping_add((id as u32).wrapping_mul(0x6e) as usize);
        if p != 0 && read_i32(p) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        if p == read_usize(club + club::MANAGER) {
            continue;
        }
        if p == 0 {
            show_error_at(0xa83b78, 0x1010);
            continue;
        }
        append_person_history_by_id(!CUP_PLACED_CLUB_DRY_RUN, this, read_usize(p), &e);
    }
    if kind == 2 && !CUP_PLACED_CLUB_DRY_RUN {
        let stored: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NATIONAL_TEAMS_IS_FINALS_COMP);
        if stored(read_usize(game + fixture::COMPETITION)) != 0 {
            let acabar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_END_DUTY_FOR_NATION_IN_COMPETITION);
            acabar(read_usize(G_NATIONAL_TEAMS), read_usize(game + fixture::HOME_CLUB), read_usize(game + fixture::COMPETITION));
            acabar(read_usize(G_NATIONAL_TEAMS), read_usize(game + fixture::AWAY_CLUB), read_usize(game + fixture::COMPETITION));
        }
    }
    let mut e2 = history_entry();
    stamp_game_date(e2.as_mut_ptr());
    e2[8] = kind;
    put_i32(&mut e2, 9, read_i32(comp));
    append_club_history_by_id(this, read_usize(club), &e2);
}

/// the cup runner-up: the club reputation rises up to weight × 500 (at most 350 at once), its side's three match
/// ratings by the weight, and the manager, chairman, line-up and club histories
/// Original: `Staff_Records::add_cup_runners_up_record` (0x86b0b0, staff_records.cpp, thiscall).
///
/// [pt] { 0x86b0b0 } (thiscall, ret 0xc, staff_records.cpp, SEH só das datas locais): `Staff_Records::
/// add_cup_runners_up_record(clube, jogo, partida)`, pela final das copas (Cup 0x519890, Liga dos Campeões 0x584ae0 e
/// as copas de cada nação): `update_cup_placed_club` com o tipo 1. Clube, jogo ou partida nulos: a caixa (linha 0x5f6).
pub(crate) unsafe extern "thiscall" fn add_cup_runners_up_record(this: usize, club: usize, game: usize, partida: usize) {
    if club == 0 || game == 0 || partida == 0 {
        return show_error_at(0xa83b78, 0x5f6);
    }
    update_cup_placed_club(this, club, game, partida, 1);
}

/// the cup third place: the same with weight × 200 (at most 200) and smaller ratings; ends the call-ups in the stored
/// competitions
/// Original: `Staff_Records::add_cup_third_place_record` (0x86b710, staff_records.cpp, thiscall).
///
/// [pt] { 0x86b710 } (thiscall, ret 0xc, staff_records.cpp, SEH só das datas locais): `Staff_Records::
/// add_cup_third_place_record(clube, jogo, partida)`, pelas copas com disputa de terceiro: `update_cup_placed_club` com o tipo
/// 2. Nulos: a caixa (linha 0x67b).
pub(crate) unsafe extern "thiscall" fn add_cup_third_place_record(this: usize, club: usize, game: usize, partida: usize) {
    if club == 0 || game == 0 || partida == 0 {
        return show_error_at(0xa83b78, 0x67b);
    }
    update_cup_placed_club(this, club, game, partida, 2);
}

/// Rehearsal comparer for cup runner-up/third: copy without histories vs original from the same state, comparing club
/// reputation and the side's three ratings.
///
/// [pt] Comparador em ensaio dos colocados: a cópia sem os históricos e sem acabar as convocações, e a original a partir do
/// mesmo estado (a reputação do clube e as três notas do lado dele na partida). Nulos: só a original.
pub(crate) unsafe fn compare_cup_placed(name: &str, club: usize, game: usize, partida: usize, original: impl FnOnce(), copy: impl FnOnce()) {
    if club == 0 || game == 0 || partida == 0 {
        return original();
    }
    let reg = [(club + club::REPUTATION, 2), (partida + match_record::HOME_MANAGER_CURRENT_REPUTATION + 0x625 * (read_usize(game + fixture::AWAY_CLUB) == club) as usize, 6)];
    let before = crate::comparison::MemorySnapshot::read(&reg);
    CUP_PLACED_CLUB_DRY_RUN = true;
    copy();
    CUP_PLACED_CLUB_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&reg);
    before.write_back();
    original();
    let mo = crate::comparison::MemorySnapshot::read(&reg);
    SCOREBOARD.record(mo == mr, || format!("{}(clube {:#x}, jogo {:#x}) original={:x?} rust={:x?}", name, club, game, mo, mr));
}

/// Copies date src into dst (0x417c10).
///
/// [pt] A data em `src` copiada para `dst` (0x417c10; a cópia de retorno vai para uma data local).
pub(crate) unsafe fn staff_records_copy_date(dst: usize, src: usize) {
    let copy: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut t = [0u8; 8];
    copy(dst, t.as_mut_ptr(), src);
}

/// Days of date a minus date b (0x54cdb0, b by value; writes the static date 0xae2470).
///
/// [pt] Os dias da data `a` menos a `b` (0x54cdb0, `b` por valor; grava a data estática 0xae2470).
pub(crate) unsafe fn days_between(a: usize, b: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    f(a, read_usize(b) as u32, read_usize(b + 4) as u32)
}

/// `Staff_Records::record_manager_job_change(manager, club, hired)`: closes the manager's current spell, opens the new
/// one when hired and counts the spells. SEH frame only for the local dates. Called by the manager's departure
/// (0x69b790) and by the new manager (0x694c80).
///
/// Outside a national team (0x53b900), in the manager's record (0x86f5b0 with create), when the game start ([0xdd6ec8])
/// is not after today: the current spell (+0x4a club, +0x4e start) closes; the shortest (+0x72, +0x76 start, +0x7e end)
/// is replaced by the current one if there is none or if the current one is shorter (0x54cdb0), or only has its end
/// renewed if it is the same; the longest (+0x5e, +0x62, +0x6a) is replaced if the current one is longer (the end
/// becomes +0x56) or if there is none (the end becomes today). Then +0x4a = −1 and, with hired = 1, the new spell (the
/// club id, today, +0x56 = 13/11/1975 from 0x549f70) and +1 in +0xb4. For a national team, hired = 1: +1 in +0xb5.
///
/// The histories: the manager's (type 0x96: club, hired), the club chairman's (+0xbf; type 0x97 with the manager and
/// the three keys) and the club's (type 0x96 in +8, the manager, the keys and hired). Null manager or club: the error
/// box (line 0x3b1).
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
pub(crate) unsafe extern "thiscall" fn record_manager_job_change(this: usize, manager: usize, club: usize, contratado: u32) {
    if club == 0 || manager == 0 {
        return show_error_at(0xa83b78, 0x3b1);
    }
    let contratado = contratado as u8 as i8;
    let record: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
    if is_national_team(club) == 0 {
        let r = record(this, manager, 1);
        if r != 0 {
            let today = G_CURRENT_DATE;
            let (ai, ah) = (*((G_GAME_START_DATE + 2) as *const u16), *((today + 2) as *const u16));
            let after = ai > ah || (ai == ah && read_i16(G_GAME_START_DATE) > read_i16(today));
            if !after {
                crate::comparison::branch("cqa_12 passagem fechada");
                if read_i32(r + manager_record::SHORTEST_CLUB) != -1 && read_i32(r + manager_record::CURRENT_CLUB) != -1 {
                    if read_i32(r + manager_record::SHORTEST_CLUB) == read_i32(r + manager_record::CURRENT_CLUB) && read_i16(r + manager_record::CURRENT_CLUB_SINCE_YEAR) == read_i16(r + manager_record::SHORTEST_START_YEAR) && read_i16(r + manager_record::CURRENT_CLUB_SINCE) == read_i16(r + manager_record::SHORTEST_START) {
                        staff_records_copy_date(r + manager_record::SHORTEST_END, today);
                    } else {
                        let current = days_between(today, r + manager_record::CURRENT_CLUB_SINCE);
                        if current < days_between(r + manager_record::SHORTEST_END, r + manager_record::SHORTEST_START) {
                            write_i32(r + manager_record::SHORTEST_CLUB, read_i32(r + manager_record::CURRENT_CLUB));
                            staff_records_copy_date(r + manager_record::SHORTEST_START, r + manager_record::CURRENT_CLUB_SINCE);
                            staff_records_copy_date(r + manager_record::SHORTEST_END, today);
                        }
                    }
                } else {
                    write_i32(r + manager_record::SHORTEST_CLUB, read_i32(r + manager_record::CURRENT_CLUB));
                    staff_records_copy_date(r + manager_record::SHORTEST_START, r + manager_record::CURRENT_CLUB_SINCE);
                    staff_records_copy_date(r + manager_record::SHORTEST_END, today);
                }
                if read_i32(r + manager_record::LONGEST_CLUB) != -1 && read_i32(r + manager_record::CURRENT_CLUB) != -1 {
                    let current = days_between(today, r + manager_record::CURRENT_CLUB_SINCE);
                    if current > days_between(r + manager_record::LONGEST_END, r + manager_record::LONGEST_START) {
                        write_i32(r + manager_record::LONGEST_CLUB, read_i32(r + manager_record::CURRENT_CLUB));
                        staff_records_copy_date(r + manager_record::LONGEST_START, r + manager_record::CURRENT_CLUB_SINCE);
                        staff_records_copy_date(r + manager_record::LONGEST_END, r + manager_record::CURRENT_CLUB_END);
                    }
                } else {
                    write_i32(r + manager_record::LONGEST_CLUB, read_i32(r + manager_record::CURRENT_CLUB));
                    staff_records_copy_date(r + manager_record::LONGEST_START, r + manager_record::CURRENT_CLUB_SINCE);
                    staff_records_copy_date(r + manager_record::LONGEST_END, today);
                }
            }
            write_i32(r + manager_record::CURRENT_CLUB, -1);
            if contratado == 1 {
                crate::comparison::branch("cqa_12 passagem nova");
                write_i32(r + manager_record::CURRENT_CLUB, read_i32(club));
                staff_records_copy_date(r + manager_record::CURRENT_CLUB_SINCE, today);
                let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
                let mut t = [0u8; 8];
                let d = data(t.as_mut_ptr(), 0xd, 0xb, 0x7b7, u32::MAX);
                staff_records_copy_date(r + manager_record::CURRENT_CLUB_END, d);
                *((r + manager_record::CLUB_JOBS) as *mut u8) = (*((r + manager_record::CLUB_JOBS) as *const u8)).wrapping_add(1); // cqa12: as passagens
            }
        }
    } else if contratado == 1 {
        let r = record(this, manager, 1);
        if r != 0 {
            *((r + manager_record::NATIONAL_JOBS) as *mut u8) = (*((r + manager_record::NATIONAL_JOBS) as *const u8)).wrapping_add(1);
        }
    }
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x96;
    put_i32(&mut e, 8, read_i32(club));
    put_i32(&mut e, 0xd, contratado as i32);
    append_person_history_by_id(!CUP_PLACED_CLUB_DRY_RUN, this, read_usize(manager), &e);
    if read_usize(club + club::CHAIRMAN) != 0 {
        let mut e2 = history_entry();
        stamp_game_date(e2.as_mut_ptr());
        e2[0xc] = 0x97;
        put_i32(&mut e2, 8, read_i32(club));
        put_i32(&mut e2, 0xd, contratado as i32);
        put_person_keys(&mut e2, 0x11, manager);
        append_person_history_by_id(!CUP_PLACED_CLUB_DRY_RUN, this, read_usize(read_usize(club + club::CHAIRMAN)), &e2);
    }
    let mut e3 = history_entry();
    stamp_game_date(e3.as_mut_ptr());
    e3[8] = 0x96;
    put_person_keys(&mut e3, 9, manager);
    put_i32(&mut e3, 0x19, contratado as i32);
    append_club_history_by_id(this, read_usize(club), &e3);
}

/// `Staff_Records::add_team_of_year_record(person, club, award, a, b)`: the team of the year entry in the person's
/// history. SEH frame only for the local date, with an empty destructor. Called by TEAM_AWARD::record_winners 0x8affb0.
///
/// Null person or award: the "Error" box (line 0x7ee). Otherwise the person's history (type 0x32: the club id or −1,
/// the award id, `a` as unsigned 16 bits and `b` as a signed byte), in the archive (0x58e340).
/// Original: `Staff_Records::add_team_of_year_record` (0x86c730, staff_records.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x86c730 } (thiscall, ret 0x14, staff_records.cpp, SEH só da data local, de destrutor vazio):
/// `Staff_Records::add_team_of_year_record(pessoa, clube, prêmio, a, b)` (pela TEAM_AWARD::record_winners 0x8affb0):
/// pessoa ou prêmio nulos, a caixa "Error" (linha 0x7ee). Senão o histórico da pessoa (tipo 0x32: o id do clube ou −1,
/// o id do prêmio, `a` em 16 bits sem sinal e `b` como byte com sinal), no arquivo (0x58e340).
pub(crate) unsafe extern "thiscall" fn add_team_of_year_record(this: usize, person: usize, club: usize, prize: usize, a: u32, b: u32) {
    crate::comparison::branch("cps_10 selecao do ano");
    if person == 0 || prize == 0 {
        show_error_at(0xa83b78, 0x7ee);
        return;
    }
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x32;
    put_i32(&mut e, 8, staff_records_id_or_minus_one(club));
    put_i32(&mut e, 0xd, read_i32(prize));
    put_i32(&mut e, 0x11, (a & 0xffff) as i32);
    put_i32(&mut e, 0x15, b as u8 as i8 as i32);
    append_person_history_by_id(true, this, read_u32_as_usize(person), &e);
}

// simplification: the history goes to the file (does not run twice) and the copy writes nothing else: original only; the
// copy takes the branch.

/// Returns the club manager's points record from G_STAFF_RECORDS (0x86f5b0, creating it), or 0 without a manager (uses
/// G_STAFF_RECORDS instead of this).
///
/// [pt] O registro de pontos do técnico do clube (+0xcf; 0x86f5b0 do [0xdd7f00], criando), ou 0 sem técnico.
pub(crate) unsafe fn club_manager_record_global(club: usize) -> usize {
    let manager = read_u32_as_usize(club + club::MANAGER);
    if manager == 0 {
        return 0;
    }
    let record: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
    record(read_u32_as_usize(G_STAFF_RECORDS), manager, 1)
}

/// `staff_records_compare_clubs_by_manager_points(a, b)`: the qsort order of the manager of the year candidates
/// (0x86fab0): orders two clubs by their manager's league season points, highest first.
///
/// The records of the two clubs' managers (club_manager_record_global); neither, 0; only `a`'s missing, 1; only `b`'s
/// missing, −1; with both, `b`'s league season points (+0xba) minus `a`'s (more points first).
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
    crate::comparison::branch("cps_10 pontos dos dois");
    read_i32(rb + manager_record::SEASON_POINTS).wrapping_sub(read_i32(ra + manager_record::SEASON_POINTS)) // cps10: mais pontos primeiro
}

// simplification: 0x86f5b0 creates the missing record; the original runs first and the copy only finds the ones it created.

/// Adds v to the i32 at address a.
pub(crate) unsafe fn staff_records_add_i32(a: usize, v: i32) {
    write_i32(a, read_i32(a).wrapping_add(v));
}

/// A new 0x40-byte history entry on the stack: the date (0x54a110, Date()) with the current game date copied in
/// (0x417c10).
///
/// Simplification: the bytes the original routine does not write (its stack garbage) come out zeroed here; they only go
/// to the history file (FILE_LLIST), which the career never reads back.
///
/// [pt] Uma entrada de histórico na pilha: a data (0x54a110) com a data do jogo copiada (0x417c10).
/// simplification: os bytes que a original não grava (lixo da pilha dela) saem zerados aqui; só vão para o arquivo do
/// histórico (FILE_LLIST), que a carreira não lê de volta.
pub(crate) unsafe fn dated_history_entry() -> [u8; 0x40] {
    let mut e = [0u8; 0x40];
    let new: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let copy: unsafe extern "thiscall" fn(*mut u8, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut t = [0u8; 8];
    new(e.as_mut_ptr());
    copy(e.as_mut_ptr(), t.as_mut_ptr(), G_CURRENT_DATE);
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
pub(crate) unsafe fn append_history(write: bool, arq: usize, base: usize, id: usize, e: &[u8; 0x40]) {
    if !write {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) -> usize = core::mem::transmute(FN_FILE_LLIST_ADD_NODE);
    let p = base.wrapping_add(id.wrapping_mul(8));
    f(arq, p, p + 4, e.as_ptr());
}

/// Appends entry e to the person's history (file [this], lists [this+0xc]).
///
/// [pt] O histórico da pessoa (o arquivo [this], as listas em [this+0xc]).
pub(crate) unsafe fn append_person_history(write: bool, this: usize, person: usize, e: &[u8; 0x40]) {
    append_history(write, read_usize(this), read_usize(this + staff_records_fields::PERSON_HISTORY_LISTS), read_usize(person), e);
}

/// Appends a dated club history entry with type and competition id (file [this+4], lists [this+0x10]).
///
/// [pt] O histórico do clube (o arquivo [this+4], as listas em [this+0x10]) com o tipo e a competição.
pub(crate) unsafe fn append_club_history(write: bool, this: usize, club: usize, kind: u8, comp: usize) {
    let mut e = dated_history_entry();
    e[8] = kind;
    put_u32(&mut e, 9, read_usize(comp) as u32);
    append_history(write, read_usize(this + staff_records_fields::CLUB_HISTORY_FILE), read_usize(this + staff_records_fields::CLUB_HISTORY_LISTS), read_usize(club), &e);
}

/// Builds a dated person history entry with club id (+8), type (+0xc) and competition (+0xd).
///
/// [pt] A entrada da pessoa: o clube (+8), o tipo (+0xc) e a competição (+0xd).
pub(crate) unsafe fn person_history_entry(club: usize, kind: u8, comp: u32) -> [u8; 0x40] {
    let mut e = dated_history_entry();
    put_u32(&mut e, 8, read_usize(club) as u32);
    e[0xc] = kind;
    put_u32(&mut e, 0xd, comp);
    e
}

/// Iterates the club's 50 squad slots yielding players with a player record (+0x61), id below [0xae2364] and not the
/// manager.
///
/// [pt] Os jogadores do elenco (50 vagas em +0xd7) que contam nos registros: com registro de jogador (+0x61), id abaixo de
/// [0xae2364] e que não são o técnico.
pub(crate) unsafe fn counted_squad_players(club: usize) -> impl Iterator<Item = usize> {
    (0..50usize).filter_map(move |k| {
        let p = read_usize(club + club::SQUAD + 4 * k);
        if p == 0 || read_usize(p + staff::PLAYER) == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) || p == read_usize(club + club::MANAGER) {
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
pub(crate) unsafe fn will_renew_contract(club: usize, p: usize, c: usize) -> bool {
    let gt = read_usize(G_TRANSFER_MANAGER);
    if gt == 0 {
        return false;
    }
    let days: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    if days(G_CURRENT_DATE, read_usize(c + contract::DATE_STARTED), read_usize(c + 0x29)) <= 0x12c {
        return false;
    }
    let entry: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    let e = entry(read_usize(gt + transfer_manager_fields::SHORTLIST_MANAGER), club, p, 0, 0);
    e != 0 && read_usize(e + shortlist_slot::TARGET_STAFF_ID) == read_usize(p)
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
pub(crate) unsafe fn raise_promotion_rating(a: usize, x: i32, weight: i32) {
    write_i16(a, read_i16(a).wrapping_add(ftol(x.wrapping_mul(weight) as f64 * read_f64(0x96ff88)) as i16));
    let minimum = (x as i16).wrapping_mul(weight as i16);
    let v = read_i16(a);
    write_i16(a, if minimum > v { minimum } else if v > 0x2710 { 0x2710 } else { v });
}

/// Promotion of a club: reputation +500, max attendance, manager ratings, histories for manager, chairman and squad,
/// squad reputations and morale, renewals; no file histories when not writing.
///
/// [pt] O acesso; `gravar` falso só na cópia pelo comparador (sem os históricos em arquivo).
pub(crate) unsafe fn apply_promotion(this: usize, club: usize, comp: usize, weight: u32, write: bool) {
    if club == 0 || comp == 0 {
        show_error_at(0xa83b78, 0x301);
        return;
    }
    let e = person_history_entry(club, 8, read_usize(comp) as u32);
    write_i16(club + club::REPUTATION, read_i16(club + club::REPUTATION).wrapping_add(500));
    if read_i16(club + club::REPUTATION) > 0x2710 {
        write_i16(club + club::REPUTATION, 0x2710);
    }
    let v = read_i32(club + club::MAX_ATTENDANCE);
    if v < 0x249f0 && v < read_i32(club + club::ATTENDANCE).wrapping_add(500) {
        write_i32(club + club::MAX_ATTENDANCE, v.wrapping_add(read_i16(comp + competition::REPUTATION) as i32 * 10));
    }
    if read_usize(club + club::MANAGER) != 0 {
        let manager = read_usize(club + club::MANAGER);
        if read_usize(manager + staff::NON_PLAYER) != 0 {
            let object = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(comp).wrapping_mul(4)));
            let mut pela = comp;
            if object != 0 && read_i32(object + league::UPPER_DIVISION_ID) > -1 {
                let cc = read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_i32(object + league::UPPER_DIVISION_ID) as usize).wrapping_mul(0x6b));
                pela = if cc != 0 { cc } else { comp };
            }
            let (p1, p2) = if read_u8(object + league::CLUB_SIDES) as i8 != 1 {
                (400, 400)
            } else {
                let foreign: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
                (200, if foreign(manager, read_usize(club + club::NATION), 1) != 0 { 200 } else { 400 })
            };
            let x = read_i16(pela + competition::REPUTATION) as i32;
            let nj = || read_usize(read_usize(club + club::MANAGER) + 0x69);
            crate::comparison::branch("cpq_3 acesso do tecnico");
            raise_promotion_rating(nj() + 0xa, x, 400);
            raise_promotion_rating(nj() + 8, x, p2);
            raise_promotion_rating(nj() + 0xc, x, p1);
        }
        append_person_history(write, this, read_usize(club + club::MANAGER), &e);
        let record: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
        let r = record(this, read_usize(club + club::MANAGER), 1);
        if r != 0 {
            let points = (read_i16(comp + competition::REPUTATION) as i32).wrapping_mul(weight as u8 as i8 as i32).wrapping_mul(2500) / 100;
            staff_records_add_i32(r + manager_record::YEAR_POINTS, points);
            staff_records_add_i32(r + manager_record::SEASON_POINTS, points);
            staff_records_add_i32(r + manager_record::MONTH_POINTS, points);
        }
    }
    if read_usize(club + club::CHAIRMAN) != 0 {
        append_person_history(write, this, read_usize(club + club::CHAIRMAN), &e);
    }
    for p in counted_squad_players(club) {
        adjust_morale(p, 0xf);
        let c = game_contract_for_club(p, club);
        if c == 0 {
            continue;
        }
        let v = read_u8(c + 0x44) as i8;
        let s = v as i32 + 0x4b;
        *((c + 0x44) as *mut u8) = if s > 0x64 { 0x64 } else if s < -0x64 { 0x9c } else { v.wrapping_add(0x4b) as u8 };
        let kind = read_u8(c + contract::SQUAD_STATUS) >> 4;
        if (kind == 0 || (kind > 3 && kind != 5)) || !will_renew_contract(club, p, c) {
            continue;
        }
        crate::comparison::branch("cpq_3 acesso renova");
        offer_renewal(p);
    }
    append_club_history(write, this, club, 8, comp);
}

/// `Staff_Records::add_promotion_record(club, comp, weight)`: records a promotion. SEH frame without a destructor.
///
/// The club's reputation (+0x80) goes up 500 (up to 10000) and +0x7b goes up 10 × the competition's importance (+0x69)
/// if it is below 150000 and below +0x73 + 500. The manager (+0xcf) with a non-player record: ratings +0xa, +8 and +0xc
/// go up by the competition's importance (that of its club competition record +0x1c, if any) × 400, × 200/400
/// (foreigner in the league with +0x42 = 1, 0x53d980) and × 200/400 (league with +0x42 = 1) × 10/61, with a minimum of
/// importance × weight.
///
/// The manager's and chairman's history (type 8), the manager's points (0x86f5b0: +0xb6, +0xba, +0xca add importance ×
/// weight × 25), morale +15 for each squad player (0x544710), the contract bond (+0x44) +75 (−100..100) and the renewal
/// (0x4cda70) of the expiring contracts of types 1, 2, 3 and 5; finally the club's history (type 8). Null club or
/// competition: the "Error" box (line 0x301).
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
pub(crate) unsafe extern "thiscall" fn add_promotion_record(this: usize, club: usize, comp: usize, weight: u32) {
    crate::comparison::branch("cpq_3 acesso");
    apply_promotion(this, club, comp, weight, true)
}

/// The memory regions that promotion or relegation touch (without the histories); `None` if any contract would be
/// renewed.
///
/// The random generator state (morale uses random draws), the club, the manager's non-player record, his points record
/// (created beforehand, as the original would), and the morale (+0x45) and reputations (+9..+0xe) of each squad player
/// and the link (+0x44) of his contract. `None` if any contract goes to renewal (0x4cda70 makes the offer: it does not
/// run twice; on promotion types 1, 2, 3 and 5, on relegation 1 and 2).
///
/// [pt] As regiões que o acesso ou o rebaixamento mexem (sem os históricos): o estado do sorteador (o moral sorteia), o clube,
/// o não jogador do técnico, o registro de pontos dele (criado antes, como a original faria), o moral (+0x45) e as
/// reputações (+9..+0xe) de cada jogador do elenco e o vínculo (+0x44) do contrato dele. `None` se algum contrato vai a
/// renovação (a 0x4cda70 faz a oferta: não roda duas vezes; no acesso os tipos 1, 2, 3 e 5, no rebaixamento 1 e 2).
pub(crate) unsafe fn promotion_regions(this: usize, club: usize, promotion: bool) -> Option<Vec<(usize, usize)>> {
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((club, 0x245));
    let manager = read_usize(club + club::MANAGER);
    if manager != 0 {
        if read_usize(manager + staff::NON_PLAYER) != 0 {
            reg.push((read_usize(manager + staff::NON_PLAYER), 0x10));
        }
        if promotion {
            let record: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
            let r = record(this, manager, 1);
            if r != 0 {
                reg.push((r, 0xd2));
            }
        }
    }
    for p in counted_squad_players(club) {
        let j = read_usize(p + staff::PLAYER);
        reg.extend([(j + player_sheet::HOME_REPUTATION, 6), (j + player_sheet::PLAYER_MORALE, 1)]);
        let c = game_contract_for_club(p, club);
        if c != 0 {
            let kind = read_u8(c + contract::SQUAD_STATUS) >> 4;
            let renova = if promotion { kind != 0 && (kind <= 3 || kind == 5) } else { kind == 1 || kind == 2 };
            if renova && will_renew_contract(club, p, c) {
                return None;
            }
            reg.push((c + 0x44, 1));
        }
    }
    Some(reg)
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
        write_i16(j + o, ftol(read_i16(j + o) as i32 as f64 * k) as i16);
    }
}

/// Relegation of a club: reputation −500 (min 750), manager ratings down, histories for manager, chairman and squad,
/// squad reputations and morale.
pub(crate) unsafe fn apply_relegation(this: usize, club: usize, comp: usize, write: bool) {
    if club == 0 || comp == 0 {
        show_error_at(0xa83b78, 0x476);
        return;
    }
    let e = person_history_entry(club, 9, read_usize(comp) as u32);
    write_i16(club + club::REPUTATION, read_i16(club + club::REPUTATION).wrapping_sub(500));
    if read_i16(club + club::REPUTATION) < 0x2ee {
        write_i16(club + club::REPUTATION, 0x2ee);
    }
    if read_usize(club + club::MANAGER) != 0 {
        let manager = read_usize(club + club::MANAGER);
        if read_usize(manager + staff::NON_PLAYER) != 0 {
            let foreign: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
            let factor: i16 = if foreign(manager, read_usize(club + club::NATION), 1) != 0 { -8 } else { -16 };
            let nj = || read_usize(read_usize(club + club::MANAGER) + 0x69);
            let x = || read_i16(comp + competition::REPUTATION);
            crate::comparison::branch("cpq_3 rebaixamento do tecnico");
            add_rating_clamped(nj() + 0xa, x().wrapping_mul(-16));
            add_rating_clamped(nj() + 8, x().wrapping_mul(factor));
            add_rating_clamped(nj() + 0xc, x().wrapping_mul(-8));
        }
        append_person_history(write, this, read_usize(club + club::MANAGER), &e);
    }
    if read_usize(club + club::CHAIRMAN) != 0 {
        append_person_history(write, this, read_usize(club + club::CHAIRMAN), &e);
    }
    for p in counted_squad_players(club) {
        adjust_morale(p, -0xf);
        let c = game_contract_for_club(p, club);
        if c == 0 {
            continue;
        }
        let j = read_usize(p + staff::PLAYER);
        match read_u8(c + contract::SQUAD_STATUS) >> 4 {
            1 | 2 => {
                if j != 0 {
                    scale_player_reputations(j, read_f64(0x9689d8)); // 0,9
                }
                if will_renew_contract(club, p, c) {
                    crate::comparison::branch("cpq_3 rebaixamento renova");
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
    append_club_history(write, this, club, 9, comp);
}

/// `Staff_Records::add_relegation_record(club, comp)`: records a relegation. SEH frame without a destructor.
///
/// The club's reputation (+0x80) drops 500 (minimum 750). The manager with a non-player record: ratings +0xa (−16), +8
/// (−8 if a foreigner, 0x53d980, or −16) and +0xc (−8) × the competition's importance (+0x69), clamped to 1..10000; the
/// manager's and chairman's history (type 9). Each squad player: morale −15 (0x544710) and, by contract type (+0x4f >>
/// 4): 1 and 2, reputations × 0.9 and the renewal (0x4cda70) if expiring; 3 to 5, reputations × 0.95. Finally the
/// club's history (type 9). Null club or competition: the "Error" box (line 0x476).
/// Original: `Staff_Records::add_relegation_record` (0x86a1c0, staff_records.cpp, thiscall, ret 8).
///
/// [pt] { 0x86a1c0 } (thiscall, ret 8, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_relegation_record(clube,
/// comp)`: a reputação do clube (+0x80) cai 500 (mínimo 750). O técnico com não jogador: as notas +0xa (−16), +8 (−8
/// estrangeiro, 0x53d980, ou −16) e +0xc (−8) × a importância da competição (+0x69), presas a 1..10000; o histórico do
/// técnico e do presidente (tipo 9). Cada jogador do elenco: moral −15 (0x544710) e, pelo tipo do contrato (+0x4f >> 4):
/// 1 e 2, reputações × 0,9 e a renovação (0x4cda70) se vence; 3 a 5, reputações × 0,95. Por fim o histórico do clube
/// (tipo 9). Clube ou competição nulos: a caixa "Error" (linha 0x476).
pub(crate) unsafe extern "thiscall" fn add_relegation_record(this: usize, club: usize, comp: usize) {
    crate::comparison::branch("cpq_3 rebaixamento");
    apply_relegation(this, club, comp, true)
}

/// Qualification of a club: person history entry (type 3, stage, base round) for the manager and adds reputation² ×
/// weight points to his record (national team or club totals).
pub(crate) unsafe fn record_qualification(this: usize, club: usize, comp: usize, stage: u32, round: u32, weight: u32, write: bool) {
    if club == 0 {
        show_error_at(0xa83b78, 0x703);
        return;
    }
    let mut e = person_history_entry(club, 3, if comp != 0 { read_usize(comp) as u32 } else { u32::MAX });
    put_u32(&mut e, 0x11, stage as u16 as i16 as i32 as u32);
    let base: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_COMP_UTIL_GET_BASE_ROUND_ID);
    put_u32(&mut e, 0x15, base(round) as u16 as i16 as i32 as u32);
    if read_usize(club + club::MANAGER) == 0 {
        return;
    }
    append_person_history(write, this, read_usize(club + club::MANAGER), &e);
    let record: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
    let r = record(this, read_usize(club + club::MANAGER), 1);
    if r == 0 {
        return;
    }
    // simplification: the original does pow(x, 2.0) (0x945ff0): x × x, exact for the importance (16 bits)
    let x = read_i16(comp + competition::REPUTATION) as f64;
    let points = ftol(x * x * (weight as u8 as i8 as i32) as f64);
    staff_records_add_i32(r + manager_record::YEAR_POINTS, points);
    staff_records_add_i32(r + manager_record::MONTH_POINTS, points);
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if national_team(club) != 0 {
        crate::comparison::branch("cpq_3 classificado selecao");
        staff_records_add_i32(r + manager_record::NATIONAL_SEASON_POINTS, points);
    } else {
        crate::comparison::branch("cpq_3 classificado clube");
        staff_records_add_i32(r + manager_record::SEASON_POINTS, points);
    }
}

/// `Staff_Records::add_qualified_for_round_record(club, comp, stage, round, weight)`: SEH frame without a destructor.
///
/// With a manager (+0xcf), his history (type 3: club, competition or −1, the signed stage and the base round 0x4b0870)
/// and the points (0x86f5b0): importance² × weight added to +0xb6 (year), +0xca (month) and, for a national team
/// (0x53b900), +0xbe (cup), or for a club +0xba (league). Null club: the "Error" box (line 0x703).
/// Original: `Staff_Records::add_qualified_for_round_record` (0x86bdd0, staff_records.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x86bdd0 } (thiscall, ret 0x14, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_qualified_for_round_record(
/// clube, comp, fase, rodada, peso)`: com técnico (+0xcf), o histórico dele (tipo 3: clube, competição ou −1, a fase com
/// sinal e a rodada base 0x4b0870) e os pontos (0x86f5b0): importância² × peso somados em +0xb6, +0xca e, na seleção
/// (0x53b900), +0xbe, ou no clube +0xba. Clube nulo: a caixa "Error" (linha 0x703).
pub(crate) unsafe extern "thiscall" fn add_qualified_for_round_record(this: usize, club: usize, comp: usize, stage: u32, round: u32, weight: u32) {
    record_qualification(this, club, comp, stage, round, weight, true)
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
    let copy: unsafe extern "thiscall" fn(*mut u8, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut t = [0u8; 8];
    copy(a, t.as_mut_ptr(), G_CURRENT_DATE);
}

/// New 0x40-byte history entry on the stack with a constructed Date (0x54a110).
///
/// [pt] Uma entrada de histórico na pilha com a data construída (0x54a110).
/// simplification: os bytes que a original não grava (lixo da pilha dela) saem zerados aqui; só vão para o arquivo do
/// histórico (FILE_LLIST), que a carreira não lê de volta.
pub(crate) unsafe fn history_entry() -> [u8; 0x40] {
    let mut e = [0u8; 0x40];
    let new: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    new(e.as_mut_ptr());
    e
}

/// Appends entry e to person id's list in the person history FILE_LLIST [this] (heads at [this+0xc]); nothing when not
/// writing (takes the person id and this directly).
///
/// [pt] A entrada `e` no fim da lista da pessoa `id` no arquivo do histórico das pessoas (FILE_LLIST [this], 0x58e340; o
/// começo e o fim em [this+0xc] + id × 8). Sem `gravar` (a cópia pelo comparador), nada.
pub(crate) unsafe fn append_person_history_by_id(write: bool, this: usize, id: usize, e: &[u8; 0x40]) {
    if !write {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) -> usize = core::mem::transmute(FN_FILE_LLIST_ADD_NODE);
    let p = read_usize(this + staff_records_fields::PERSON_HISTORY_LISTS).wrapping_add(id.wrapping_mul(8));
    f(read_usize(this), p, p + 4, e.as_ptr());
}

/// Returns the id of the person's employing club (staff::CLUB_JOB), or −1.
///
/// [pt] O id do clube contratante da pessoa (+0x39), ou −1.
pub(crate) unsafe fn club_job_id(person: usize) -> i32 {
    let c = read_usize(person + staff::CLUB_JOB);
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
    let l = read_usize(t + offer::EXCHANGE_PLAYERS);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + llist::CURSOR) as *mut u32, read_usize(l + llist::HEAD) as u32);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut e = next(read_usize(t + offer::EXCHANGE_PLAYERS));
    while e != 0 {
        f(e);
        e = next(read_usize(t + offer::EXCHANGE_PLAYERS));
    }
}

/// True if the job type is a player job (0x53b620: 11, 12, 13 or 15).
///
/// [pt] O tipo do cargo é de jogador (0x53b620: 11, 12, 13 ou 15)?
pub(crate) unsafe fn staff_records_is_player_job(job: u8) -> bool {
    let f: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    f(job as u32) != 0 // simplification: the original pushes the register with the byte in al; 0x53b620 reads only the byte
}

/// Returns the manager's points record (FN_STAFF_RECORDS_GET_MANAGER_RECORD with create).
pub(crate) unsafe fn staff_records_manager_record(this: usize, manager: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
    f(this, manager, 1)
}

/// Sides of a transfer: (person, his contract, selling club or 0, buying club).
///
/// [pt] Os lados da transferência: (pessoa, contrato da pessoa 0x4cf5a0, vendedor ou 0, comprador).
pub(crate) unsafe fn transfer_parties(t: usize) -> (usize, usize, usize, usize) {
    let person = staff_records_staff_by_id(read_usize(t + offer::TARGET_STAFF_ID));
    let contract: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract(read_usize(G_CONTRACT_MANAGER), person);
    let seller = if c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f != 5 {
        read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + contract::CLUB).wrapping_mul(club::SIZE))
    } else {
        0
    };
    (person, c, seller, read_usize(G_CLUB_TABLE).wrapping_add(read_usize(t + offer::BUYER_CLUB_ID).wrapping_mul(club::SIZE)))
}

/// Records a transfer in the staff records: biggest purchase/sale and totals in the buying and selling managers'
/// records, plus history entries for the player and clubs; no file histories when not writing.
pub(crate) unsafe fn record_transfer(this: usize, t: usize, write: bool) {
    let (pessoa, c, vendedor, comprador) = transfer_parties(t);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let mut e = history_entry();
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_TOTAL_VALUE);
    let value = total(t, 1, 0, 0, 1);
    let with_swap = || (read_usize(t + offer::EXCHANGE_PLAYERS) != 0) as i32;
    let kind = read_u8(t + offer::JOB_TYPE) as i8 as i32;
    if comprador != 0 && read_usize(comprador + club::MANAGER) != 0 {
        let manager = read_usize(comprador + club::MANAGER);
        let r = if staff_records_is_player_job(read_u8(t + offer::JOB_TYPE)) { staff_records_manager_record(this, manager) } else { 0 };
        if r != 0 {
            crate::comparison::branch("cpq_4 compra");
            if value > read_i32(r + manager_record::BIGGEST_PURCHASE_FEE) {
                crate::comparison::branch("cpq_4 maior compra");
                for k in 0..4usize {
                    write_i32(r + 4 * k, if k == 0 { read_i32(pessoa) } else { read_i32(read_usize(pessoa + staff::FIRST_NAME * k) + 0x33) });
                }
                stamp_game_date((r + manager_record::BIGGEST_PURCHASE_DATE) as *mut u8);
                write_i32(r + manager_record::BIGGEST_PURCHASE_FEE, value);
                *((r + manager_record::BIGGEST_PURCHASE_SWAP) as *mut u8) = with_swap() as u8;
                write_i32(r + manager_record::BIGGEST_PURCHASE_SELLER, staff_records_id_or_minus_one(vendedor));
                write_i32(r + manager_record::BIGGEST_PURCHASE_BUYER, read_i32(comprador));
            }
            write_i16(r + manager_record::PLAYERS_BOUGHT, read_i16(r + manager_record::PLAYERS_BOUGHT).wrapping_add(1));
            staff_records_add_i64(r + manager_record::PURCHASES_TOTAL, value);
            if read_usize(t + offer::EXCHANGE_PLAYERS) != 0 {
                write_i16(r + manager_record::PLAYERS_SOLD, read_i16(r + manager_record::PLAYERS_SOLD).wrapping_add(read_i16(read_usize(t + offer::EXCHANGE_PLAYERS))));
                for_each_swap_item(t, |x| staff_records_add_i64(r + manager_record::SALES_TOTAL, read_i32(staff_records_staff_by_id(read_usize(x)) + 0x52)));
            }
        }
        // the buying manager’s history: the purchase and each player given in the swap
        stamp_game_date(e.as_mut_ptr());
        e[0xc] = 0x64;
        put_i32(&mut e, 8, club_job_id(manager));
        put_person_keys(&mut e, 0xd, pessoa);
        put_i32(&mut e, 0x1d, value);
        put_i32(&mut e, 0x21, with_swap());
        put_i32(&mut e, 0x25, staff_records_id_or_minus_one(vendedor));
        put_i32(&mut e, 0x29, kind);
        append_person_history_by_id(write, this, read_usize(manager), &e);
        for_each_swap_item(t, |x| {
            e[0xc] = 0x65;
            put_person_keys(&mut e, 0xd, staff_records_staff_by_id(read_usize(x)));
            put_i32(&mut e, 0x1d, 0);
            put_i32(&mut e, 0x21, 1);
            put_i32(&mut e, 0x25, read_i32(vendedor));
            put_i32(&mut e, 0x29, kind);
            append_person_history_by_id(write, this, read_usize(manager), &e);
        });
    }
    if vendedor != 0 && read_usize(vendedor + club::MANAGER) != 0 {
        let manager = read_usize(vendedor + club::MANAGER);
        let r = if staff_records_is_player_job(read_u8(pessoa + staff::JOB_FOR_CLUB)) { staff_records_manager_record(this, manager) } else { 0 };
        if r != 0 {
            crate::comparison::branch("cpq_4 venda");
            if value > read_i32(r + manager_record::BIGGEST_SALE_FEE) {
                for k in 0..4usize {
                    write_i32(r + manager_record::BIGGEST_SALE_PERSON + 4 * k, if k == 0 { read_i32(pessoa) } else { read_i32(read_usize(pessoa + staff::FIRST_NAME * k) + 0x33) });
                }
                stamp_game_date((r + manager_record::BIGGEST_SALE_DATE) as *mut u8);
                write_i32(r + manager_record::BIGGEST_SALE_FEE, value);
                *((r + manager_record::BIGGEST_SALE_SWAP) as *mut u8) = with_swap() as u8;
                write_i32(r + manager_record::BIGGEST_SALE_SELLER, read_i32(vendedor));
                write_i32(r + manager_record::BIGGEST_SALE_BUYER, staff_records_id_or_minus_one(comprador));
            }
            write_i16(r + manager_record::PLAYERS_SOLD, read_i16(r + manager_record::PLAYERS_SOLD).wrapping_add(1));
            staff_records_add_i64(r + manager_record::SALES_TOTAL, value);
            if read_usize(t + offer::EXCHANGE_PLAYERS) != 0 {
                write_i16(r + manager_record::PLAYERS_BOUGHT, read_i16(r + manager_record::PLAYERS_BOUGHT).wrapping_add(read_i16(read_usize(t + offer::EXCHANGE_PLAYERS))));
                for_each_swap_item(t, |x| staff_records_add_i64(r + manager_record::PURCHASES_TOTAL, read_i32(staff_records_staff_by_id(read_usize(x)) + 0x52)));
            }
        }
        // the selling manager’s history: the sale and each player received in the swap
        stamp_game_date(e.as_mut_ptr());
        e[0xc] = 0x65;
        put_i32(&mut e, 8, club_job_id(manager));
        put_person_keys(&mut e, 0xd, pessoa);
        put_i32(&mut e, 0x1d, value);
        put_i32(&mut e, 0x21, with_swap());
        put_i32(&mut e, 0x25, staff_records_id_or_minus_one(comprador));
        put_i32(&mut e, 0x29, kind);
        append_person_history_by_id(write, this, read_usize(manager), &e);
        for_each_swap_item(t, |x| {
            e[0xc] = 0x64;
            put_person_keys(&mut e, 0xd, staff_records_staff_by_id(read_usize(x)));
            put_i32(&mut e, 0x1d, 0);
            put_i32(&mut e, 0x21, 1);
            put_i32(&mut e, 0x25, read_i32(comprador));
            put_i32(&mut e, 0x29, 0xb);
            append_person_history_by_id(write, this, read_usize(manager), &e);
        });
    }
    // the history of the transferred player and that of each player in the swap
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x66;
    put_i32(&mut e, 8, club_job_id(pessoa));
    put_i32(&mut e, 0xd, staff_records_id_or_minus_one(comprador));
    put_i32(&mut e, 0x11, value);
    put_i32(&mut e, 0x19, kind);
    put_i32(&mut e, 0x15, with_swap());
    if pessoa != 0 {
        append_person_history_by_id(write, this, read_usize(pessoa), &e);
    } else {
        show_error_at(0xa83b78, 0x1010);
    }
    for_each_swap_item(t, |x| {
        let q = staff_records_staff_by_id(read_usize(x));
        put_i32(&mut e, 8, read_i32(read_usize(q + staff::CLUB_JOB)));
        put_i32(&mut e, 0x11, 0);
        put_i32(&mut e, 0xd, read_i32(vendedor));
        put_i32(&mut e, 0x15, 1);
        put_i32(&mut e, 0x19, read_u8(x + 6) as i8 as i32);
        if q != 0 {
            append_person_history_by_id(write, this, read_usize(q), &e);
        } else {
            show_error_at(0xa83b78, 0x1010);
        }
    });
}

/// `Staff_Records::add_transfer_records(transfer)`: updates the buying and selling managers' records and the transfer
/// histories. SEH frame without a destructor.
///
/// The person's contract (0x4cf5a0) goes to [0xae1080] and gives the seller (outside state 5); the offer's total value
/// (0x8e7a70 with 1, 0, 0, 1). The buyer's manager, when the offer type is for a player (+0x11, 0x53b620): in the
/// record (0x86f5b0) the biggest signing (+0..+0x24 if the value exceeds +0x18), the signings (+0xa0), the spending
/// (+0xa4, 64 bits), the players given in exchange (+0xa2) and their value (+0x52 of each, in +0xac). The seller's
/// manager, when the person's job is player (+0x3d): the biggest sale (+0x25..+0x49 if it exceeds +0x3d), the sales
/// (+0xa2), the income (+0xac), the players received (+0xa0) and their value (+0xa4).
///
/// The histories (types 0x64 buy, 0x65 sale, 0x66 transfer) of the buyer's manager, the seller's manager, the person
/// and the exchange players, in the archive (0x58e340).
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

/// Builds a dated person history entry with club id (+8), type (+0xc), competition id or −1 (+0xd) and the signed stage
/// (+0x11).
///
/// [pt] A entrada da pessoa: o clube (+8), o tipo (+0xc), a competição ou −1 (+0xd) e a fase com sinal (+0x11).
pub(crate) unsafe fn stage_history_entry(club: usize, kind: u8, comp: usize, stage: u32) -> [u8; 0x40] {
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    put_i32(&mut e, 8, read_i32(club));
    e[0xc] = kind;
    put_i32(&mut e, 0xd, staff_records_id_or_minus_one(comp));
    put_i32(&mut e, 0x11, stage as u16 as i16 as i32);
    e
}

/// If the competition is the stored national-team finals competition (0x76d940 on G_NATIONAL_TEAMS), ends the nation's
/// call-ups for it (0x774f90).
///
/// [pt] Na competição de seleção guardada (0x76d940 de [0xb74318]), as convocações da seleção para ela acabam (0x774f90).
pub(crate) unsafe fn end_national_duty(club: usize, comp: usize) {
    let special: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NATIONAL_TEAMS_IS_FINALS_COMP);
    if special(read_usize(G_NATIONAL_TEAMS), comp) != 0 {
        crate::comparison::branch("cpq_4 fim das convocacoes");
        let encerrar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_END_DUTY_FOR_NATION_IN_COMPETITION);
        encerrar(read_usize(G_NATIONAL_TEAMS), club, comp);
    }
}

/// `Staff_Records::add_knocked_out_record(club, comp, stage, round, _)`: SEH frame without a destructor.
///
/// The manager's history (+0xcf; type 4: club, competition or −1, the signed stage and the base round 0x4b0870) and the
/// end of the national team's call-ups in the stored competition. Null club: the "Error" box (line 0x73b).
/// Original: `Staff_Records::add_knocked_out_record` (0x86c000, staff_records.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x86c000 } (thiscall, ret 0x14, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_knocked_out_record(clube,
/// comp, fase, rodada, _)`: o histórico do técnico (+0xcf; tipo 4: clube, competição ou −1, a fase com sinal e a rodada
/// base 0x4b0870) e o fim das convocações da seleção na competição guardada. Clube nulo: a caixa "Error" (linha 0x73b).
pub(crate) unsafe extern "thiscall" fn add_knocked_out_record(this: usize, club: usize, comp: usize, stage: u32, round: u32, _x: u32) {
    if club == 0 {
        return show_error_at(0xa83b78, 0x73b);
    }
    crate::comparison::branch("cpq_4 eliminado");
    let mut e = stage_history_entry(club, 4, comp, stage);
    let base: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_COMP_UTIL_GET_BASE_ROUND_ID);
    put_i32(&mut e, 0x15, base(round) as u16 as i16 as i32);
    if read_usize(club + club::MANAGER) != 0 {
        append_person_history_by_id(true, this, read_usize(read_usize(club + club::MANAGER)), &e);
    }
    end_national_duty(club, comp);
}

// simplification: the history goes to the file and the call-ups end (they do not run twice) and the copy without them does not touch
// in nothing: original only; the copy carries the branches.

/// `Staff_Records::add_failed_to_qualify_record(club, comp, stage, _)`: SEH frame without a destructor.
///
/// The manager's history (type 7: club, competition or −1, the signed stage) and the end of the national team's
/// call-ups in the stored competition. Null club: the "Error" box (line 0x768).
/// Original: `Staff_Records::add_failed_to_qualify_record` (0x86c1d0, staff_records.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x86c1d0 } (thiscall, ret 0x10, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_failed_to_qualify_record(
/// clube, comp, fase, _)`: o histórico do técnico (tipo 7: clube, competição ou −1, a fase com sinal) e o fim das
/// convocações da seleção na competição guardada. Clube nulo: a caixa "Error" (linha 0x768).
pub(crate) unsafe extern "thiscall" fn add_failed_to_qualify_record(this: usize, club: usize, comp: usize, stage: u32, _x: u32) {
    if club == 0 {
        return show_error_at(0xa83b78, 0x768);
    }
    crate::comparison::branch("cpq_4 nao classificado");
    let e = stage_history_entry(club, 7, comp, stage);
    if read_usize(club + club::MANAGER) != 0 {
        append_person_history_by_id(true, this, read_usize(read_usize(club + club::MANAGER)), &e);
    }
    end_national_duty(club, comp);
}

// simplification: like the elimination: original only.

/// The points record of each manager (Staff_Records +8): 0xd2 bytes per non-player person ([0xae2368]) and for the 16
/// humans.
///
/// +0x86 person id (−1 empty), +0xb6 points for the year, +0xba/+0xbe points in the league and in the cup (+0xc2 and
/// +0xc6 their competitions, −1 none), +0xca/+0xce those for the month.
///
/// [pt] O registro de pontos de cada técnico (Staff_Records +8): 0xd2 bytes por pessoa que não é jogador ([0xae2368]) e
/// pelos 16 humanos. +0x86 id da pessoa (−1 vazio), +0xb6 pontos do ano, +0xba/+0xbe pontos na liga e na copa (+0xc2 e
/// +0xc6 as competições deles, −1 sem), +0xca/+0xce os do mês.
pub(crate) const MANAGER_RECORD_SIZE: usize = 0xd2;

/// Number of manager records: [0xae2368] (non-player count) + 16 human slots.
pub(crate) unsafe fn manager_record_count() -> i32 {
    read_i32(G_NON_PLAYER_COUNT).wrapping_add(0x10)
}

/// Returns manager record k in the records array [this+8] (MANAGER_RECORD_SIZE bytes each).
pub(crate) unsafe fn manager_record_at(this: usize, k: i32) -> usize {
    read_usize(this + staff_records_fields::MANAGER_RECORDS).wrapping_add((k as usize).wrapping_mul(MANAGER_RECORD_SIZE))
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
    let month: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_DATE_GET_MONTH);
    month(data) as u8 as i8 as i16
}

/// `Staff_Records::daily_update(date)`: if the date's month (0x54cfb0) or year (+2) are not the stored ones (+0x14
/// month, +0x16 year), zeroes the month's points (+0xca, +0xce) of every record and stores the new month and year.
///
/// On the first day of the year (day +0 = 0) passes the year's points (+0xb6) of each record with a person (+0x86) to
/// the hall of fame (0x5ef270 on [0xb5ded8]) and zeroes them.
/// Original: `Staff_Records::daily_update` (0x86dda0, staff_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x86dda0 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::daily_update(data)`: se o mês (0x54cfb0) ou o ano
/// (+2) da data não são os guardados (+0x14 mês, +0x16 ano), zera os pontos do mês (+0xca, +0xce) de todos os
/// registros e guarda o mês e o ano novos. No primeiro dia do ano (dia +0 = 0) passa os pontos do ano (+0xb6) de cada
/// registro com pessoa (+0x86) ao hall da fama (0x5ef270 em [0xb5ded8]) e os zera.
pub(crate) unsafe extern "thiscall" fn staff_records_daily_update(this: usize, data: usize) {
    if date_month(data) as i32 != read_i16(this + staff_records_fields::CURRENT_MONTH) as u16 as i32 || read_i16(data + 2) != read_i16(this + staff_records_fields::CURRENT_YEAR) {
        crate::comparison::branch("cpo_3 virada do mes");
        let base = read_usize(this + staff_records_fields::MANAGER_RECORDS);
        let n = manager_record_count();
        for k in 0..n.max(0) as usize {
            let e = base.wrapping_add(k * MANAGER_RECORD_SIZE);
            write_usize(e + manager_record::MONTH_POINTS, 0);
            write_usize(e + manager_record::MONTH_LEAGUE_POINTS, 0); // the month’s points
        }
        let m = date_month(data);
        core::ptr::write_unaligned((this + staff_records_fields::CURRENT_MONTH) as *mut i16, m);
        core::ptr::write_unaligned((this + staff_records_fields::CURRENT_YEAR) as *mut i16, read_i16(data + 2));
    }
    if read_i16(data) == 0 {
        let base = read_usize(this + staff_records_fields::MANAGER_RECORDS);
        let hall: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_HALLOFFAME_ADD_CALENDAR_YEAR_ENTRY);
        let mut k = 0i32;
        while k < manager_record_count() {
            let e = base.wrapping_add((k as usize).wrapping_mul(MANAGER_RECORD_SIZE));
            let id = read_usize(e + manager_record::OWNER);
            if id as i32 != -1 {
                hall(read_usize(G_HALL_OF_FAME), staff_records_staff_by_id(id), read_usize(e + manager_record::YEAR_POINTS));
            }
            write_usize(e + manager_record::YEAR_POINTS, 0);
            k += 1;
        }
    }
}

/// `Staff_Records::end_season_for_nation(entry)`: at the end of the competition's season passes its managers' league
/// and cup points to the hall of fame and clears them.
///
/// With the entry's competition (+0, id at its +0; no competition −1; no competition and +4 not equal to [0x9cfa10],
/// nothing), for each record: league (+0xc2) of the competition: adds the cup (+0xbe) if it also belongs to that
/// competition (+0xc6 = −1, +0xbe = 0), passes the points (+0xba) to the hall of fame (0x5ef330) if the competition
/// keeps them (+0x11c), zeroes them and +0xc2 = −1; only the competition's cup: passes +0xbe, zeroes it and +0xc6 = −1.
/// Original: `Staff_Records::end_season_for_nation` (0x86d6e0, staff_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x86d6e0 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::end_season_for_nation(entrada)`: com a competição
/// da entrada (+0, id em +0 dela; sem competição −1; sem competição e sem +4 igual a [0x9cfa10], nada), para cada
/// registro: liga (+0xc2) da competição: soma a copa (+0xbe) se ela também é dessa competição (+0xc6 = −1, +0xbe = 0),
/// passa os pontos (+0xba) ao hall da fama (0x5ef330) se a competição guarda (+0x11c), zera e +0xc2 = −1; só a copa da
/// competição: passa +0xbe, zera e +0xc6 = −1.
pub(crate) unsafe extern "thiscall" fn staff_records_end_season_for_nation(this: usize, entry: usize) {
    let p = read_usize(entry + 4);
    let pela_global = p != 0 && read_usize(p) == read_usize(KEY_CONTINENT_EUROPE);
    if !pela_global && read_usize(entry) == 0 {
        return;
    }
    let comp = read_usize(entry);
    let id = if comp != 0 { read_i32(comp) } else { -1 };
    let hall: unsafe extern "thiscall" fn(usize, usize, i32, usize) = core::mem::transmute(FN_HALLOFFAME_ADD_LEAGUE_ENTRY);
    let guarda = |c: usize| c != 0 && *((c + nation::LEAGUE_SELECTED) as *const u8) != 0;
    let base = read_usize(this + staff_records_fields::MANAGER_RECORDS);
    let mut k = 0i32;
    while k < manager_record_count() {
        let s = base.wrapping_add((k as usize).wrapping_mul(MANAGER_RECORD_SIZE)) + 0xba;
        let cup = read_i32(s + 0xc);
        if read_i32(s + 8) == id {
            crate::comparison::branch("cpo_3 fim da liga");
            if cup == id {
                write_usize(s, read_usize(s).wrapping_add(read_usize(s + 4)));
                write_usize(s + 0xc, usize::MAX);
                write_usize(s + 4, 0);
            }
            if guarda(read_usize(entry)) {
                hall(read_usize(G_HALL_OF_FAME), staff_records_staff_by_id(read_usize(s - 0x34)), id, read_usize(s));
            }
            write_usize(s, 0);
            write_usize(s + 8, usize::MAX);
        } else if cup == id {
            if guarda(read_usize(entry)) {
                hall(read_usize(G_HALL_OF_FAME), staff_records_staff_by_id(read_usize(s - 0x34)), id, read_usize(s + 4));
            }
            write_usize(s + 4, 0);
            write_usize(s + 0xc, usize::MAX);
        }
        k += 1;
    }
}

/// `Staff_Records::find_manager_of_the_month(index)`: the league manager with the most points this month.
///
/// In the league ([0xadadfc] by the index; +0x43 must be 2, otherwise the "Error" box and 0), among the clubs of the
/// table (+0xb1, 0x3b bytes, count +0x3e) with a manager (+0xcf) at the club (+0x39) since two weeks ago (0x54cab0 of
/// the game date; year +0x40 unsigned, day +0x3e signed), the one with the most points in the month (+0xce of the
/// record 0x86f5b0, created if missing), only above 0. Returns the manager or 0.
/// Original: `Staff_Records::find_manager_of_the_month` (0x86f950, staff_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x86f950 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::find_manager_of_the_month(indice)`: na liga
/// ([0xadadfc] pelo índice; +0x43 tem de ser 2, senão a caixa "Error" e 0), entre os clubes da tabela (+0xb1, 0x3b
/// bytes, conta +0x3e) com técnico (+0xcf) no clube (+0x39) desde duas semanas atrás (0x54cab0 da data do jogo; ano
/// +0x40 sem sinal, dia +0x3e com sinal), o de mais pontos no mês (+0xce do registro 0x86f5b0, criado se falta), só
/// acima de 0. Devolve o técnico ou 0.
pub(crate) unsafe extern "thiscall" fn find_manager_of_the_month(this: usize, index: i32) -> usize {
    let league = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((index as usize).wrapping_mul(4)));
    if *((league + league::COMP_TYPE) as *const u8) != 2 {
        show_error_at(0xa83b78, 0x104e);
        return 0;
    }
    let before: unsafe extern "thiscall" fn(usize, *mut u32, i32) -> usize = core::mem::transmute(FN_DATE_SUB_WEEKS);
    let record: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
    let mut best_points = 0i32;
    let mut best = 0usize;
    let mut k = 0i32;
    while k < read_i16(league + league::CLUB_COUNT) as i32 {
        let club = read_usize(read_usize(league + league::TABLE).wrapping_add((k as usize).wrapping_mul(0x3b)));
        let manager = read_usize(club + club::MANAGER);
        if manager != 0 && read_usize(manager + staff::CLUB_JOB) != 0 {
            let mut d = [0u32; 2];
            let q = before(G_CURRENT_DATE, d.as_mut_ptr(), 2); // two weeks ago
            let lo = read_usize(q) as u32;
            let year = (lo >> 16) as u16;
            let year_manager = read_i16(manager + 0x40) as u16;
            let recente = year < year_manager || (year == year_manager && (lo as u16 as i16) < read_i16(manager + staff::DATE_JOINED_CLUB));
            if recente {
                crate::comparison::branch("cpo_3 tecnico recente");
            } else {
                let r = record(this, manager, 1);
                if r != 0 {
                    let points = read_i32(r + manager_record::MONTH_LEAGUE_POINTS);
                    if points > best_points { // the best of the month
                        crate::comparison::branch("cpo_3 melhor do mes");
                        best_points = points;
                        best = read_usize(club + club::MANAGER);
                    }
                }
            }
        }
        k += 1;
    }
    best
}

/// `Staff_Records::get_manager_of_year_candidates(clubs, n)` (called by 0x768940; SEH frame without an object with a
/// destructor): sorts the clubs and keeps up to three.
///
/// The qsort of the `n` clubs (0x870220) and, in order, up to three whose manager (+0xcf) arrived (+0x3e) before today
/// − 28 weeks (0x54cab0; year unsigned, day signed) and has a record (0x86f5b0 with 1) with games (+0x8e), copied to
/// the start of the vector itself. Returns how many.
/// Original: `Staff_Records::get_manager_of_year_candidates` (0x86fab0, staff_records.cpp, thiscall, ret 8).
///
/// [pt] { 0x86fab0 } (thiscall, ret 8, staff_records.cpp, SEH sem objeto com destrutor):
/// `Staff_Records::get_manager_of_year_candidates(clubes, n)` (pela 0x768940): o qsort dos `n` clubes (0x870220) e, na
/// ordem, até três cujo técnico (+0xcf) chegou (+0x3e) antes de hoje − 28 semanas (0x54cab0; ano sem sinal, dia com
/// sinal) e tem registro (0x86f5b0 com 1) com jogos (+0x8e), copiados para o começo do próprio vetor. Devolve quantos.
pub(crate) unsafe extern "thiscall" fn get_manager_of_year_candidates(this: usize, clubs: usize, n: u32) -> u8 {
    let weeks: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_SUB_WEEKS);
    let mut lim = [0u8; 8];
    weeks(G_CURRENT_DATE, lim.as_mut_ptr(), 0x1c);
    let n = n as u16 as i16 as i32;
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(clubs, n as u32, 4, 0x870220);
    let (ld, la) = (i16::from_le_bytes([lim[0], lim[1]]), u16::from_le_bytes([lim[2], lim[3]]));
    let record: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
    let mut k = 0u8;
    for i in 0..n.max(0) as usize {
        if k as i8 >= 3 {
            break;
        }
        let manager = read_u32_as_usize(read_u32_as_usize(clubs + 4 * i) + 0xcf);
        if manager == 0 {
            continue;
        }
        let ta = read_u16(manager + 0x40);
        if !(la > ta || (la == ta && ld > read_i16(manager + staff::DATE_JOINED_CLUB))) {
            continue;
        }
        let r = record(this, manager, 1);
        if r != 0 && read_i16(r + manager_record::GAMES) != 0 {
            core::ptr::write_unaligned((clubs + 4 * k as usize) as *mut u32, read_u32_as_usize(clubs + 4 * i) as u32);
            k += 1;
        }
    }
    k
}

/// `Staff_Records::get_manager_record(person, create)`: finds (or creates, or takes over) the manager's 210-byte record
/// (vector +8).
///
/// Null person: the error box at line 0xf6e and 0. Free slot (+0x86 = −1): without `create`, 0; with it, the slot takes
/// the id. Someone else's slot: zeroes the record of that person (−1 in +0x86, +0xc2, +0xc6, +0x4a, +0x5e, +0x72) and
/// looks again.
///
/// Updates the nations of the club (+0x39) and of the national team (+0x24) in +0xc2/+0xc6, zeroing the points
/// (+0xba/+0xbe) when they change. With the date of [0xdd6ec8] up to today: no spell (+0x4a = −1), opens the current
/// club's one (since the person's +0x3e; +0x56 = the date 13/11/1975 from 0x549f70); the longest (+0x5e, +0x62..+0x6a)
/// and the shortest (+0x72, +0x76..+0x7e) through 0x54cdb0, today's in +0x6a / +0x7e. Returns the record.
/// Original: `Staff_Records::get_manager_record` (0x86f5b0, staff_records.cpp, thiscall, ret 8).
///
/// [pt] { 0x86f5b0 } (thiscall, ret 8, staff_records.cpp): `Staff_Records::get_manager_record(pessoa, criar)`: o registro de
/// 210 bytes do técnico (vetor +8). Pessoa nula: caixa da linha 0xf6e e 0. Vaga livre (+0x86 = −1): sem `criar`, 0;
/// com ele, a vaga fica com o id. Vaga de outro: zera o registro da ficha dele (−1 em +0x86, +0xc2, +0xc6, +0x4a, +0x5e,
/// +0x72) e olha de novo. Atualiza as nações do clube (+0x39) e da seleção (+0x24) em +0xc2/+0xc6, zerando os pontos
/// (+0xba/+0xbe) quando mudam. Com a data de [0xdd6ec8] até hoje: sem passagem (+0x4a = −1), abre a do clube atual
/// (desde +0x3e da pessoa; +0x56 = a data 13/11/1975 da 0x549f70); a mais longa (+0x5e, +0x62..+0x6a) e a mais curta
/// (+0x72, +0x76..+0x7e) pela 0x54cdb0, a de hoje em +0x6a / +0x7e. Devolve o registro.
pub(crate) unsafe extern "thiscall" fn get_manager_record(this: usize, person: usize, create: u32) -> usize {
    if person == 0 {
        let split: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
        let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
        let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
        let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
        split(0xa83b78, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
        let v = version(nome.as_mut_ptr(), ext.as_mut_ptr(), 0xf6e);
        format(0xae24d0, 0x9870e8, v, nome.as_ptr(), ext.as_ptr(), 0xf6eu32);
        caixa(0x9870e0, 0xae24d0, 0);
        write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
        return 0;
    }
    let r = loop {
        let Some(r) = manager_manager_manager_record(this, person) else {
            return 0;
        };
        let owner = read_u32(r + manager_record::OWNER) as i32;
        if owner == -1 {
            if create == 0 {
                return 0;
            }
            crate::comparison::branch("cpp_3 criar registro");
            write_u32(r + manager_record::OWNER, read_u32(person)); // the slot stays with him
            break r;
        }
        if owner == read_u32(person) as i32 {
            break r;
        }
        // the slot belongs to someone else: zero his profile record and look again
        let z = (read_u32(this + staff_records_fields::MANAGER_RECORDS) as usize).wrapping_add((read_u32(read_u32(person + staff::NON_PLAYER) as usize) as usize).wrapping_mul(0xd2));
        core::ptr::write_bytes(z as *mut u8, 0, 0xd2);
        for o in [0x86, 0xc2, 0xc6, 0x4a, 0x5e, 0x72] {
            write_u32(z + o, u32::MAX);
        }
    };
    let club = read_u32(person + staff::CLUB_JOB) as usize;
    if club != 0 {
        let nation = read_u32(club + club::NATION) as usize;
        if nation != 0 && read_u32(nation) != read_u32(r + manager_record::SEASON_NATION) {
            write_u32(r + manager_record::SEASON_NATION, read_u32(nation));
            write_u32(r + manager_record::SEASON_POINTS, 0);
        }
    }
    let national_team = read_u32(person + staff::NATIONAL_JOB) as usize;
    if national_team != 0 {
        let nation = read_u32(national_team + club::NATION) as usize;
        if nation != 0 && read_u32(nation) != read_u32(r + manager_record::NATIONAL_NATION) {
            write_u32(r + manager_record::NATIONAL_NATION, read_u32(nation));
            write_u32(r + manager_record::NATIONAL_SEASON_POINTS, 0);
        }
    }
    let (dia, ano) = (read_i16(G_GAME_START_DATE), core::ptr::read_unaligned(G_GAME_START_DATE_YEAR as *const u16));
    let today_year = core::ptr::read_unaligned(G_CURRENT_DATE_YEAR as *const u16);
    if ano > today_year || (ano == today_year && dia > read_i16(G_CURRENT_DATE)) {
        return r; // the date in [0xdd6ec8] has not come yet
    }
    if read_u32(r + manager_record::CURRENT_CLUB) as i32 == -1 {
        let club = read_u32(person + staff::CLUB_JOB) as usize;
        if club != 0 {
            crate::comparison::branch("cpp_3 abrir passagem");
            write_u32(r + manager_record::CURRENT_CLUB, read_u32(club));
            contract_manager_assign_date(r + manager_record::CURRENT_CLUB_SINCE, person + staff::DATE_JOINED_CLUB);
            let mut local = [0u8; 8];
            let construct: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32, u32, i32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
            let p = construct(&mut local, 0xd, 0xb, 0x7b7, -1);
            contract_manager_assign_date(r + manager_record::CURRENT_CLUB_END, p);
        }
    }
    let days: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    if read_u32(r + manager_record::LONGEST_CLUB) as i32 == -1 {
        write_u32(r + manager_record::LONGEST_CLUB, read_u32(r + manager_record::CURRENT_CLUB));
        contract_manager_assign_date(r + manager_record::LONGEST_START, r + manager_record::CURRENT_CLUB_SINCE);
        contract_manager_assign_date(r + manager_record::LONGEST_END, G_CURRENT_DATE);
    } else if read_u32(r + manager_record::CURRENT_CLUB) as i32 != -1 {
        let a = days(r + manager_record::LONGEST_END, read_u32(r + manager_record::LONGEST_START), read_u32(r + manager_record::LONGEST_START_HIGH));
        let b = days(G_CURRENT_DATE, read_u32(r + manager_record::CURRENT_CLUB_SINCE), read_u32(r + manager_record::CURRENT_CLUB_SINCE_HIGH));
        if b > a {
            write_u32(r + manager_record::LONGEST_CLUB, read_u32(r + manager_record::CURRENT_CLUB)); // a mais longa
            contract_manager_assign_date(r + manager_record::LONGEST_START, r + manager_record::CURRENT_CLUB_SINCE);
            contract_manager_assign_date(r + manager_record::LONGEST_END, G_CURRENT_DATE);
        }
    }
    if read_u32(r + manager_record::SHORTEST_CLUB) as i32 == -1 {
        write_u32(r + manager_record::SHORTEST_CLUB, read_u32(r + manager_record::CURRENT_CLUB));
        contract_manager_assign_date(r + manager_record::SHORTEST_START, r + manager_record::CURRENT_CLUB_SINCE);
        contract_manager_assign_date(r + manager_record::SHORTEST_END, G_CURRENT_DATE);
        return r;
    }
    let current = read_u32(r + manager_record::CURRENT_CLUB);
    if read_u32(r + manager_record::SHORTEST_CLUB) == current && read_i16(r + manager_record::CURRENT_CLUB_SINCE_YEAR) == read_i16(r + manager_record::SHORTEST_START_YEAR) && read_i16(r + manager_record::CURRENT_CLUB_SINCE) == read_i16(r + manager_record::SHORTEST_START) {
        contract_manager_assign_date(r + manager_record::SHORTEST_END, G_CURRENT_DATE); // the shortest is the current one: runs until today
        return r;
    }
    if current as i32 == -1 {
        return r;
    }
    let a = days(r + manager_record::SHORTEST_END, read_u32(r + manager_record::SHORTEST_START), read_u32(r + manager_record::SHORTEST_START_HIGH));
    let b = days(G_CURRENT_DATE, read_u32(r + manager_record::CURRENT_CLUB_SINCE), read_u32(r + manager_record::CURRENT_CLUB_SINCE_HIGH));
    if b < a {
        write_u32(r + manager_record::SHORTEST_CLUB, read_u32(r + manager_record::CURRENT_CLUB)); // a mais curta
        contract_manager_assign_date(r + manager_record::SHORTEST_START, r + manager_record::CURRENT_CLUB_SINCE);
        contract_manager_assign_date(r + manager_record::SHORTEST_END, G_CURRENT_DATE);
    }
    r
}

/// `Staff_Records::add_free_release_records(person)`: the free release histories. SEH frame only for the local date,
/// with an empty destructor.
///
/// The person's contract (0x4cf5a0) goes to [0xae1080] and gives the club (0x245-byte record through +4). With a
/// manager at the club (+0xcf), his history (type 0x68: the club, the person and her three ids +0x33 of +4, +8 and
/// +0xc); then the person's (type 0x67: the club or −1), in the archive (0x58e340). Null person: the "Error" box (line
/// 0x1010).
/// Original: `Staff_Records::add_free_release_records` (0x86d800, staff_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x86d800 } (thiscall, ret 4, staff_records.cpp, SEH só da data local, de destrutor vazio):
/// `Staff_Records::add_free_release_records(pessoa)`: o contrato da pessoa (0x4cf5a0) vai para [0xae1080] e dá o clube
/// (registro de 0x245 bytes pelo +4). Com técnico no clube (+0xcf), o histórico dele (tipo 0x68: o clube, a pessoa e os
/// três ids +0x33 de +4, +8 e +0xc dela); depois o da pessoa (tipo 0x67: o clube ou −1), no arquivo (0x58e340).
/// Pessoa nula: a caixa "Error" (linha 0x1010).
pub(crate) unsafe extern "thiscall" fn add_free_release_records(this: usize, person: usize) {
    crate::comparison::branch("cpr_9 liberado");
    let mut e = history_entry();
    let de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = de(read_u32_as_usize(G_CONTRACT_MANAGER), person);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let club = if c != 0 { read_u32_as_usize(G_CLUB_TABLE).wrapping_add(read_u32_as_usize(c + contract::CLUB).wrapping_mul(club::SIZE)) } else { 0 };
    if club != 0 && read_u32_as_usize(club + club::MANAGER) != 0 {
        stamp_game_date(e.as_mut_ptr());
        e[0xc] = 0x68;
        put_i32(&mut e, 8, read_i32(club));
        put_person_keys(&mut e, 0xd, person);
        append_person_history_by_id(true, this, read_u32_as_usize(read_u32_as_usize(club + club::MANAGER)), &e);
    }
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x67;
    put_i32(&mut e, 8, staff_records_id_or_minus_one(club));
    if person != 0 {
        append_person_history_by_id(true, this, read_u32_as_usize(person), &e);
    } else {
        show_error_at(0xa83b78, 0x1010);
    }
}

/// `Staff_Records::add_loan_records(person, from, club)` (called by 0x8c2180, the accepted transfer): the loan
/// histories. SEH frame only for the local date, with an empty destructor.
///
/// With the club and its manager (+0xcf), the manager's history (type 0x69: the club, the person and her three ids
/// +0x33 of +4, +8 and +0xc, and the id of `from`); then, in the same entry, the person's (type 0x6a: the club and the
/// id of `from`), in the archive (0x58e340). The club is read before the person check, as in the original; null person:
/// the "Error" box (line 0x1010).
/// Original: `Staff_Records::add_loan_records` (0x86da20, staff_records.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x86da20 } (thiscall, ret 0xc, staff_records.cpp, SEH só da data local, de destrutor vazio):
/// `Staff_Records::add_loan_records(pessoa, de, clube)` (pela 0x8c2180, a transferência aceita): com o clube e o técnico
/// dele (+0xcf), o histórico do técnico (tipo 0x69: o clube, a pessoa e os três ids +0x33 de +4, +8 e +0xc dela, e o id
/// de `de`); depois, na mesma entrada, o da pessoa (tipo 0x6a: o clube e o id de `de`), no arquivo (0x58e340). O clube
/// é lido antes do teste da pessoa, como na original; pessoa nula: a caixa "Error" (linha 0x1010).
pub(crate) unsafe extern "thiscall" fn add_loan_records(this: usize, person: usize, de: usize, club: usize) {
    crate::comparison::branch("cps_9 emprestimo");
    let mut e = history_entry();
    if club != 0 && read_u32_as_usize(club + club::MANAGER) != 0 {
        let manager = read_u32_as_usize(club + club::MANAGER);
        stamp_game_date(e.as_mut_ptr());
        e[0xc] = 0x69;
        put_i32(&mut e, 8, read_i32(club));
        put_person_keys(&mut e, 0xd, person);
        put_i32(&mut e, 0x1d, read_i32(de));
        append_person_history_by_id(true, this, read_u32_as_usize(manager), &e);
    }
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x6a;
    put_i32(&mut e, 8, read_i32(club));
    put_i32(&mut e, 0xd, read_i32(de));
    if person != 0 {
        append_person_history_by_id(true, this, read_u32_as_usize(person), &e);
    } else {
        show_error_at(0xa83b78, 0x1010);
    }
}

/// `Staff_Records::rewind_staff_history(person)`: the person's history list (the 8-byte pair at [+0xc] + 8 × id) drops
/// the last entry (0x58f110 on [this]: the second offset, +4 of the pair, = −1).
///
/// Null person: the original (fatal box: the game ends).
/// Original: `Staff_Records::rewind_staff_history` (0x86fb80, staff_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x86fb80 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::rewind_staff_history(pessoa)`: a lista do
/// histórico da pessoa (o par de 8 bytes em [+0xc] + 8 × id) fica sem o último (0x58f110 sobre [this]: o segundo
/// deslocamento = −1). Pessoa nula: a original (caixa fatal: o jogo termina).
pub(crate) unsafe extern "thiscall" fn rewind_staff_history(this: usize, person: usize) {
    if person == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REWIND_STAFF_HISTORY)(this, person); // fatal box: original only
    }
    crate::comparison::branch("cpk_7 recomecar historico");
    let par = read_usize(this + staff_records_fields::PERSON_HISTORY_LISTS).wrapping_add(read_usize(person).wrapping_mul(8));
    let without_last: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_FILE_LLIST_RESET_LAST_OFFSET);
    without_last(read_usize(this), par, par + 4); // the last offset
}

/// `Staff_Records::add_year_award_record(p, club, award, a4, a5, credit)`: records a year award. SEH frame without a
/// destructor. Called by the recording of the year award winners (YEAR_AWARD::record_winners 0x9302f0: the first, the
/// second and the third).
///
/// Entry 0x34 of the person's history (game date, club or −1, the award id, a4 in the word, a5 and the credit as signed
/// bytes; 0x58e340). Manager (+0x3d or +0x28 = 5 or 0xc): +1 in the word +0x9e of his record (0x86f5b0).
///
/// A player with a player sheet (+0x61) and a contract (0x4cf5a0, other than type 5) with the club (0x4cf5f0): with the
/// transfer manager, a contract of more than 300 days and the player on the club's list (0x849ce0), the renewal
/// (0x4cda70); then, only with credit 1, without the contract's +0x45 flag, with +0x59 above 15 and +0x5b above 10, if
/// draw(+0x59) is above 5 (a draw by loyalty): the news of thanks to the manager (0x58a070). No person or no award: the
/// original ("Error" box).
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
pub(crate) unsafe extern "thiscall" fn add_year_award_record(this: usize, p: usize, club: usize, prize: usize, a4: u32, a5: u32, credito: u32) {
    if p == 0 || prize == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32)>(IDX_ADD_YEAR_AWARD_RECORD)(this, p, club, prize, a4, a5, credito);
    }
    add_year_award(this, p, club, prize, a4, a5, credito, true)
}

/// `Staff_Records::add_first_cap_record(person, opponent)` (called by the international result 0x76b600): the first cap
/// history. SEH frame only for the local date, with an empty destructor.
///
/// The person's history (type 6: the contracting club +0x39 or −1, the opponent's id in the match and the age +0x18,
/// signed), in the archive (0x58e340). The club is read before the person check, as in the original; null person: the
/// "Error" box (line 0x1010).
/// Original: `Staff_Records::add_first_cap_record` (0x86cc50, staff_records.cpp, thiscall, ret 8).
///
/// [pt] { 0x86cc50 } (thiscall, ret 8, staff_records.cpp, SEH só da data local, de destrutor vazio):
/// `Staff_Records::add_first_cap_record(pessoa, adversário)` (pelo resultado internacional 0x76b600): o histórico da
/// pessoa (tipo 6: o clube contratante +0x39 ou −1, o id do adversário na partida e a idade +0x18 com sinal), no arquivo (0x58e340).
/// O clube é lido antes do teste da pessoa, como na original; pessoa nula: a caixa "Error" (linha 0x1010).
pub(crate) unsafe extern "thiscall" fn add_first_cap_record(this: usize, person: usize, opponent: usize) {
    crate::comparison::branch("cps_8 primeira convocacao");
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 6;
    put_i32(&mut e, 8, club_job_id(person));
    put_i32(&mut e, 0xd, read_i32(opponent));
    put_i32(&mut e, 0x11, *((person + staff::AGE) as *const i8) as i32);
    if person != 0 {
        append_person_history_by_id(true, this, read_u32_as_usize(person), &e);
    } else {
        show_error_at(0xa83b78, 0x1010);
    }
}

/// `Staff_Records::add_month_award_record(p, club, award, a4, a5)`: writes the winner's 0x33 history entry. SEH frame
/// without a destructor.
///
/// Entry 0x33 of the person's history (game date, club or −1, the award id, a4 in the byte, a5 in the word; 0x58e340).
/// Manager (+0x3d or +0x28 = 5 or 0xc): +1 in the word +0x9e of his record (0x86f5b0).
///
/// A player with a player sheet (+0x61) and a contract (0x4cf5a0, other than type 5) with the club (0x4cf5f0): with the
/// transfer manager, a contract of more than 300 days and the player on the club's list (0x849ce0), the renewal
/// (0x4cda70); then, without the contract's +0x45 flag, with +0x59 above 15 and +0x5b above 10, if draw(+0x59) is above
/// 5: the news of thanks to the manager (0x58a070). No person or no award: the original ("Error" box).
/// Original: `Staff_Records::add_month_award_record` (0x86c8c0, staff_records.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x86c8c0 } (thiscall, ret 0x14, staff_records.cpp, quadro SEH sem destrutor): `Staff_Records::add_month_award_record(
/// p, clube, premio, a4, a5)`: a entrada 0x33 do histórico da pessoa (data do jogo, clube ou −1, o id do prêmio, a4 no
/// byte, a5 na palavra; 0x58e340). Técnico (+0x3d ou +0x28 = 5 ou 0xc): +1 na palavra +0x9e do registro dele
/// (0x86f5b0). Jogador com ficha (+0x61) e contrato (0x4cf5a0, fora do tipo 5) com o clube (0x4cf5f0): com o gerente de
/// transferências, contrato de mais de 300 dias e o jogador na lista do clube (0x849ce0), a renovação (0x4cda70); depois,
/// sem a marca +0x45 do contrato, com +0x59 acima de 15 e +0x5b acima de 10, sorteio(+0x59) acima de 5: a notícia do
/// agradecimento ao técnico (0x58a070). Sem pessoa ou sem prêmio: a original (caixa "Error").
pub(crate) unsafe extern "thiscall" fn add_month_award_record(this: usize, p: usize, club: usize, prize: usize, a4: u32, a5: u32) {
    if p == 0 || prize == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32)>(IDX_ADD_MONTH_AWARD_RECORD)(this, p, club, prize, a4, a5);
    }
    add_month_award_record_body(this, p, club, prize, a4, a5, true)
}

/// clears the manager record and sets the owner, league and club ids to −1
/// Original: `Staff_Records::clear_manager_record` (0x86fc20, staff_records.cpp, thiscall).
///
/// [pt] { 0x86fc20 } (thiscall, ret 4, staff_records.cpp): zera o registro de técnico da entrada e põe −1
/// nos ids do dono, da liga e dos clubes das passagens (+0x4a, +0x5e, +0x72, +0x86, +0xc2, +0xc6).
pub(crate) unsafe extern "thiscall" fn clear_manager_record(records: usize, id: *const i32) {
    let r = friendly_manager_record(records, id);
    core::ptr::write_bytes(r as *mut u8, 0, 210);
    for o in [0x86, 0xc2, 0xc6, 0x4a, 0x5e, 0x72] {
        core::ptr::write_unaligned((r + o) as *mut i32, -1);
    }
}

