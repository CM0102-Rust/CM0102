use super::*;

// The comparers of the routines in ../comp/friendly.rs.
// [pt] Os comparadores das rotinas de ../comp/friendly.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `on_friendly_fixture_cancelled`.
///
/// [pt] Comparador da 0x5c6570: a original entrega notícias, cancela torneios e tira jogos do calendário (libera): só ela
/// roda. Antes, a cópia acha a vaga do amistoso (0x5c6740, só leitura); depois a vaga tem de estar livre (+0x10 = 0 e
/// +0x18 = 0).
pub(crate) unsafe extern "thiscall" fn compare_on_friendly_fixture_cancelled(this: usize, game: usize, who: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_ON_FRIENDLY_FIXTURE_CANCELLED);
    let mut slot = 0usize;
    if read_usize(game + fixture::COMPETITION) == read_usize(this + 4) && read_usize(this + friendly::SCHEDULED_SLOT_LIST) != 0 {
        let slot_of: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_FRIENDLY_FIND_SCHEDULED_FRIENDLY_SLOT);
        slot = slot_of(this, replace_low_word(read_usize(game + fixture::HOME_CLUB), read_i16(game + fixture::DAY)), replace_low_word(read_usize(game + fixture::COMPETITION), read_i16(game + fixture::YEAR)),
            read_usize(game + fixture::AWAY_CLUB) as u32, read_usize(game + fixture::HOME_CLUB) as u32);
    }
    o(this, game, who);
    let free = slot == 0 || (read_usize(slot + scheduled_friendly_slot::OPPONENT) == 0 && read_u8(slot + scheduled_friendly_slot::STATUS) == 0);
    SCOREBOARD.record(free, || format!("on_friendly_fixture_cancelled(jogo {:#x}) a vaga {:#x} não ficou livre", game, slot));
}

/// Comparer for `friendly_cancel_national_fixture`.
///
/// [pt] Comparador da 0x5cb9e0: a original tira jogos do calendário (libera): só ela roda (simplification: a cópia não tem o que
/// conferir sem refazer a busca no calendário já mudado; a contagem do ramo prova no modo Rust).
pub(crate) unsafe extern "thiscall" fn compare_friendly_cancel_national_fixture(this: usize, game: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FRIENDLY_CANCEL_NATIONAL_FIXTURE)(this, game);
}

/// Comparer for `cancel_tournament` (comparison mode: see the header of motor.rs): the comparer of `0x5c8260`.
///
/// The original routine delivers news, removes matches from the calendar and destroys the competition, so only it runs,
/// with the plan checked afterwards: the owner (`+0`) and the competition (`+0x65`) zeroed, the slot in the competition
/// table zeroed and the tournament removed from the pending list.
///
/// [pt] Comparador da 0x5c8260: a original entrega notícias, tira jogos do calendário e destrói a competição: só ela roda,
/// com o plano conferido depois: o dono (+0) e a competição (+0x65) zerados, a vaga da tabela de competições zerada e
/// o torneio fora da lista dos pendentes.
pub(crate) unsafe extern "thiscall" fn compare_cancel_tournament(this: usize, tournament: usize, without_owner: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_CANCEL_TOURNAMENT);
    if tournament == 0 {
        o(this, tournament, without_owner);
        return;
    }
    let k = (read_i16(this + friendly::FIRST_TOURNAMENT_COMP_ID) as i32 + read_u8(tournament + tournament_slot::SLOT_INDEX) as i8 as i32) as usize;
    let slot = read_usize(G_COMP_OBJECT_TABLE).wrapping_add(k.wrapping_mul(4));
    o(this, tournament, without_owner);
    let mut in_list = false;
    let p = read_usize(this + friendly::PENDING_TOURNAMENT_LIST);
    if p != 0 {
        let mut no = read_usize(p + 2);
        let mut n = 0;
        while no != 0 && n < 0x10000 {
            if read_usize(read_usize(no)) == tournament {
                in_list = true;
            }
            no = read_usize(no + 6);
            n += 1;
        }
    }
    let ok = read_usize(tournament) == 0 && read_usize(tournament + tournament_slot::COMPETITION) == 0 && read_usize(slot) == 0 && !in_list;
    SCOREBOARD.record(ok, || format!("cancel_tournament(torneio {:#x}) dono {:#x} competição {:#x} vaga {:#x} na lista {}",
        tournament, read_usize(tournament), read_usize(tournament + tournament_slot::COMPETITION), read_usize(slot), in_list));
}

/// Comparer for `arrange_international_friendlies`.
///
/// The original routine creates friendlies (0x5c6f00, 0x5c5f40), adds to the lists (0x68d0d0) and draws at random, so
/// only it runs, with the plan in rehearsal mode. The first part of the rehearsal runs before it (afterwards the
/// calendar would already have the new friendlies); the second part, after it, uses the tape of the draws it made
/// (rng::rastrear: n and the answer, including those inside the friendly's routines) and decides each friendly by what
/// the original did.
///
/// It checks the draw sequence (the rehearsal's draws are the original's, except those inside the friendly's routines,
/// and the tape ends at the same point) and, nation list by nation list, that the original's new items are the ones the
/// rehearsal would add.
///
/// (Side by side by the random generator's state did not work: 0x5c6f00 and 0x5c5f40 draw internally in the original
/// and the rehearsal does not call them.)
///
/// [pt] Comparador: a original cria amistosos (0x5c6f00, 0x5c5f40), acrescenta às listas (0x68d0d0) e sorteia: só ela roda,
/// com o plano em ensaio. A primeira parte do ensaio roda antes dela (depois, o calendário já teria os amistosos
/// novos); a segunda, depois dela, pela fita dos sorteios que ela fez (rng::rastrear: n e resposta, os de dentro
/// das rotinas do amistoso também), decide cada amistoso pelo que a original fez. Confere a sequência dos sorteios (os
/// do ensaio são os da original, salvo os de dentro das rotinas do amistoso, e a fita acaba junto) e, lista a lista de
/// nações, que os itens novos da original são os que o ensaio acrescentaria. (Lado a lado pelo estado do sorteador não
/// servia: a 0x5c6f00 e a 0x5c5f40 sorteiam por dentro na original e o ensaio não as chama.)
pub(crate) unsafe extern "thiscall" fn compare_arrange_international_friendlies(this: usize, datas: usize, n: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_ARRANGE_INTERNATIONAL_FRIENDLIES);
    let nacoes = read_usize(G_NATION_COUNT) as i32;
    let k = nacoes.max(read_usize(this + friendly::SORTED_NATION_COUNT) as i32).max(0) as usize;
    let antes: Vec<Vec<u32>> = (0..k).map(|q| list_item_ids(nation_friendly_list(this, q))).collect();
    let mut e = FriendlyComparerTrial { usadas: vec![0u8; nacoes.max(0) as usize], antes: std::rc::Rc::new(antes), novos: vec![Vec::new(); k] };
    link_nations_near_date(this, datas, 0, Some(&mut e));
    let ((), tape_lock) = crate::rng::trace(|| o(this, datas, n));
    let quantos = tape_lock.len();
    *core::ptr::addr_of_mut!(FRIENDLY_TAPE) = (tape_lock, 0, false);
    pair_nations_for_friendlies(this, datas, n, 0, Some(&mut e), 0);
    let igual = tape_fully_consumed();
    let (pos, falhou) = { let f = &*core::ptr::addr_of!(FRIENDLY_TAPE); (f.1, f.2) };
    *core::ptr::addr_of_mut!(FRIENDLY_TAPE) = (Vec::new(), 0, false);
    let mut dif = Vec::new();
    for q in 0..k.max(e.novos.len()) {
        let mut depois = if q < k { list_item_ids(nation_friendly_list(this, q)) } else { Vec::new() };
        let mut falta = false;
        for x in e.antes.get(q).map(|v| v.as_slice()).unwrap_or(&[]) {
            match depois.iter().position(|y| y == x) {
                Some(p) => {
                    depois.remove(p);
                }
                None => falta = true,
            }
        }
        let mut esperado = e.novos.get(q).cloned().unwrap_or_default();
        depois.sort_unstable();
        esperado.sort_unstable();
        if falta || depois != esperado {
            dif.push((q, depois, esperado));
        }
    }
    SCOREBOARD.record(igual && dif.is_empty(), || {
        format!("arrange_international_friendlies(datas {:x}, {}) tape_lock de {} sorteios, o ensaio parou em {} (n diferente: {}); listas (indice, original, ensaio) {:x?}",
            datas, n as i16, quantos, pos, falhou, dif.iter().take(4).collect::<Vec<_>>())
    });
}

/// Comparer for `friendly_check_match_invitation`.
///
/// [pt] Comparador: lado a lado pelo texto e a devolução (as listas locais nascem e morrem nas duas); com a caixa, só a
/// original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_friendly_check_match_invitation(this: usize, a: usize, b: usize, club: usize, day: u32,
    year: u32, data: usize, text: usize, size: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, usize, usize, u32) -> u32>(IDX_FRIENDLY_CHECK_MATCH_INVITATION);
    if text != 0 && (size as i16) < 100 {
        return o(this, a, b, club, day, year, data, text, size);
    }
    let reg = news_text_regions(text, size);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, a, b, club, day, year, data, text, size),
        || friendly_check_match_invitation(this, a, b, club, day, year, data, text, size));
    SCOREBOARD.record(igual && r == n, || format!("friendly_check_match_invitation(a {:x}, b {:x}, clube {:x}, data {:x}) original={} rust={} {:x?} x {:x?}",
        a, b, club, data, r, n, mo, mr));
    r
}

/// Comparer for `friendly_check_tournament_invitation`.
///
/// [pt] Comparador: lado a lado pelo texto e a devolução; com a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn compare_friendly_check_tournament_invitation(this: usize, club: usize, tournament: usize, text: usize, size: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(IDX_FRIENDLY_CHECK_TOURNAMENT_INVITATION);
    if text != 0 && (size as i16) < 100 {
        return o(this, club, tournament, text, size);
    }
    let reg = news_text_regions(text, size);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club, tournament, text, size),
        || friendly_check_tournament_invitation(this, club, tournament, text, size));
    SCOREBOARD.record(igual && r == n, || format!("friendly_check_tournament_invitation(clube {:x}, torneio {:x}) original={} rust={} {:x?} x {:x?}",
        club, tournament, r, n, mo, mr));
    r
}

/// Comparer for `build_international_date_table`.
///
/// Comparer of 0x5ca4f0: the copy first (it allocates its own table, which is copied out and freed: free 0x9452ca,
/// cdecl), then the original routine from the same +0x34ec..+0x34f5; checks the count, the last day done and the 110
/// dates.
///
/// Runs once, in the friendlies constructor.
///
/// [pt] Comparador da 0x5ca4f0: a cópia primeiro (aloca a tabela dela, que sai copiada e liberada: free 0x9452ca, cdecl),
/// depois a original a partir dos mesmos +0x34ec..+0x34f5; confere a conta, o último dia feito e as 110 datas. Roda
/// uma vez, no construtor dos amistosos.
pub(crate) unsafe extern "thiscall" fn compare_build_international_date_table(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BUILD_INTERNATIONAL_DATE_TABLE);
    let fields = crate::comparison::MemorySnapshot::read(&[(this + friendly::INTL_DATE_TABLE, 10)]);
    build_international_date_table(this);
    let (t, n) = (read_usize(this + friendly::INTL_DATE_TABLE), read_i32(this + friendly::INTL_DATE_COUNT));
    if t == 0 || n != 0x6e {
        fields.write_back();
        return o(this); // simplification: without memory in the copy, original only
    }
    let mr = crate::comparison::MemorySnapshot::read(&[(t, 0x898), (this + friendly::INTL_DATE_COUNT, 6)]);
    exe::crt_free(t);
    fields.write_back();
    o(this);
    let t = read_usize(this + friendly::INTL_DATE_TABLE);
    if t == 0 {
        return SCOREBOARD.record(false, || "build_international_date_table: a original ficou sem tabela".to_string());
    }
    let mo = crate::comparison::MemorySnapshot::read(&[(t, 0x898), (this + friendly::INTL_DATE_COUNT, 6)]);
    SCOREBOARD.record(mo.0[0].1 == mr.0[0].1 && mo.0[1].1 == mr.0[1].1, || format!("build_international_date_table original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `resolve_international_dates`.
///
/// [pt] Comparador da 0x5cc510: lado a lado nas datas da tabela (só grava nelas; o qsort e a 0x54a130 não sorteiam).
pub(crate) unsafe extern "thiscall" fn compare_resolve_international_dates(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_RESOLVE_INTERNATIONAL_DATES);
    let (t, n) = (read_usize(this + friendly::INTL_DATE_TABLE), read_i32(this + friendly::INTL_DATE_COUNT));
    let reg = if t != 0 && n > 0 { vec![(t, n as usize * 0x14)] } else { Vec::new() };
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || resolve_international_dates(this));
    SCOREBOARD.record(igual, || format!("resolve_international_dates(conta {}) original={:x?} rust={:x?}", n, mo, mr));
}

/// Comparer for `plan_club_friendlies` (0x5c4770; comparison mode: see the header of motor.rs).
///
/// Only the original runs (simplification: it frees and allocates the plan, and 0x5c4380 allocates the candidates' date
/// and opponent lists; it does not run twice), checking the plan it leaves: the plan allocated, the capacity (+0x34e4)
/// not lower than the count (+0x34e0), +0x34dc zeroed and the plan in date order (0x778e90 between neighbours).
///
/// [pt] Comparador da 0x5c4770: só a original (simplification: ela libera e aloca o plano, e a 0x5c4380 aloca as listas de datas e
/// de adversários dos candidatos; não roda duas vezes), conferindo o plano que ela deixa: o plano alocado, a capacidade
/// (+0x34e4) não menor que a conta (+0x34e0), o +0x34dc zerado e o plano em ordem de data (a 0x778e90 entre vizinhos).
pub(crate) unsafe extern "thiscall" fn compare_plan_club_friendlies(this: usize, entradas: usize, ordenados: usize, count: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_PLAN_CLUB_FRIENDLIES)(this, entradas, ordenados, count);
    let (p, n, cap, z) = (read_usize(this + friendly::PLANNED_FRIENDLIES), read_i32(this + friendly::PLANNED_COUNT), read_i32(this + friendly::PLANNED_CAPACITY), read_i32(this + friendly::PLANNED_NEXT_INDEX));
    if p == 0 {
        return; // out of memory: the message box
    }
    let ordenado = (1..n.max(0) as usize).all(|i| exe::compare_call_up_records_by_date(p + (i - 1) * 0x14, p + i * 0x14) <= 0);
    SCOREBOARD.record(n >= 0 && cap >= n && z == 0 && ordenado, || format!("plan_club_friendlies: plano {:#x} conta {} capacidade {} +0x34dc {} em ordem {}", p, n, cap, z, ordenado));
}

/// Comparer for `add_tournament`.
///
/// [pt] Comparador da 0x5c6e00: lado a lado no torneio livre (0x69 bytes, mais o que um nome comprido passar) e a devolução.
pub(crate) unsafe extern "thiscall" fn compare_add_tournament(this: usize, anfitriao: usize, kind: u32, start: usize, a: u32, b: u32, c: u32, d: u32, e: u32, convidados: usize, name: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32, u32, u32, usize, usize) -> usize>(IDX_ADD_TOURNAMENT);
    let t = free_tournament_slot(this);
    let mut size = 0usize;
    while name != 0 && *((name + size) as *const u8) != 0 {
        size += 1;
    }
    let reg = if t != 0 { vec![(t, 0x69usize.max(0x31 + size + 1))] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, anfitriao, kind, start, a, b, c, d, e, convidados, name), || add_tournament(this, anfitriao, kind, start, a, b, c, d, e, convidados, name));
    SCOREBOARD.record(igual && r == n, || format!("add_tournament(anfitriao {:x}) original={:#x} {:x?} rust={:#x} {:x?}", anfitriao, r, mo, n, mr));
    r
}

/// Comparer for `find_human_club_open_invitation`.
///
/// [pt] Comparador da 0x5c68b0: lado a lado no cursor da lista (+0xa) e a devolução.
pub(crate) unsafe extern "thiscall" fn compare_find_human_club_open_invitation(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_FIND_HUMAN_CLUB_OPEN_INVITATION);
    let list = read_usize(this + friendly::FRIENDLY_MATCH_LIST);
    let reg = if list != 0 { vec![(list + llist::CURSOR, 4)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || find_human_club_open_invitation(this));
    SCOREBOARD.record(igual && r == n, || format!("find_human_club_open_invitation original={:#x} {:x?} rust={:#x} {:x?}", r, mo, n, mr));
    r
}

/// Comparer for `process_friendly_reply` (comparison mode: see the header of motor.rs): the comparer of `0x5c5f40`;
/// only the original routine runs (the match goes to the calendar and the news to the manager, which allocate, and the
/// request is freed by `0x5c87b0`).
///
/// The plan, made beforehand: the decision (friendly_reply) and the friendly's slot (`0x5c6740`). With 3 or 4, the slot
/// ends with `+0x18` = 3, or with `+0x10` = 0 and `+0x18` = 0; without a decision, the request keeps the same status
/// and leaves the reply list (`+0x34d4`). Without a requester or without a nation for one of the two: only the original
/// routine.
///
/// [pt] Comparador da 0x5c5f40: só a original (a partida vai ao calendário e a notícia ao técnico, que alocam, e o pedido é
/// liberado pela 0x5c87b0). O plano, feito antes: a decisão (friendly_reply) e a vaga do amistoso (0x5c6740). Com 3 ou 4,
/// a vaga fica com +0x18 = 3, ou com +0x10 = 0 e +0x18 = 0; sem decisão, o pedido continua com a mesma situação e sai
/// da lista das respostas (+0x34d4). Sem quem pede ou sem nação num dos dois: só a original.
pub(crate) unsafe extern "thiscall" fn compare_process_friendly_reply(this: usize, m: usize, force: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_PROCESS_FRIENDLY_REPLY);
    LAST_FRIENDLY_REPLY = m;
    let a = if m != 0 { read_usize(m + friendly_match::REQUESTING_CLUB) } else { 0 };
    let other = if m != 0 { other_friendly_club(m) } else { 0 };
    if a == 0 || (other != 0 && (read_usize(a + club::NATION) == 0 || read_usize(other + club::NATION) == 0)) {
        return o(this, m, force); // simplification: with no requester or no nation, original only
    }
    let s = if friendly_needs_decision(m, force) { friendly_reply(m, false) } else { read_u8(m + friendly_match::STATUS) };
    let slot: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) -> usize = core::mem::transmute(FN_FRIENDLY_FIND_SCHEDULED_FRIENDLY_SLOT);
    let v = if s == 3 || s == 4 { slot(this, read_i16(m) as u16 as u32, read_i16(m + 2) as u16 as u32, read_usize(m + friendly_match::AWAY_CLUB), read_usize(m + friendly_match::HOME_CLUB)) } else { 0 };
    o(this, m, force);
    let ok = if s == 3 || s == 4 {
        // the request was released: only the slot
        v == 0 || if s == 3 { read_u8(v + scheduled_friendly_slot::STATUS) == 3 } else { read_usize(v + scheduled_friendly_slot::OPPONENT) == 0 && read_u8(v + scheduled_friendly_slot::STATUS) == 0 }
    } else {
        read_u8(m + friendly_match::STATUS) == s && !list_holds(read_usize(this + friendly::REQUEST_REPLY_LIST), m)
    };
    SCOREBOARD.record(ok, || format!("process_friendly_reply(pedido {:x}) situação prevista {} vaga {:x}", m, s, v));
}

/// Comparer for `send_friendly_request`.
///
/// Comparer of 0x5c5570: only the original routine runs (the list node and the news item allocate). The plan, made
/// beforehand: the reply list gets one more node and keeps the request; the status goes to 2 when the other club, in
/// status 1, has a human manager; otherwise it does not change.
///
/// News item 6 to a human on holiday is answered on the spot by the assistant (0x784100: status 1 and 0x5c5f40 with
/// `forcar`, checked by its own comparer): then the request has already left the list (the list keeps the earlier
/// nodes) and may have been freed (the status is not read).
///
/// [pt] Comparador da 0x5c5570: só a original (o nó da lista e a notícia alocam). O plano, feito antes: a lista das
/// respostas fica com um nó a mais e guarda o pedido; a situação vai a 2 quando o outro clube, na situação 1, é de
/// técnico humano; senão não muda. A notícia 6 ao humano de férias é respondida na hora pelo assistente (0x784100:
/// situação 1 e a 0x5c5f40 com `forcar`, conferida pelo comparador dela): então o pedido já saiu da lista (a lista fica
/// com os nós de antes) e pode ter sido liberado (a situação não se lê).
pub(crate) unsafe extern "thiscall" fn compare_send_friendly_request(this: usize, m: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SEND_FRIENDLY_REQUEST);
    let other = other_friendly_club(m);
    let before = list_len(read_usize(this + friendly::REQUEST_REPLY_LIST));
    let s = if other != 0 && read_u8(m + friendly_match::STATUS) == 1 && club_has_human_manager(other) { 2 } else { read_u8(m + friendly_match::STATUS) };
    LAST_FRIENDLY_REPLY = 0;
    o(this, m);
    let l = read_usize(this + friendly::REQUEST_REPLY_LIST);
    let respondido = s == 2 && LAST_FRIENDLY_REPLY == m;
    if respondido {
        crate::comparison::branch("cqa_2 pedido respondido pelo assistente");
    }
    let ok = l == 0 || if respondido {
        list_len(l) == before && !list_holds(l, m)
    } else {
        list_len(l) == before + 1 && list_holds(l, m) && read_u8(m + friendly_match::STATUS) == s
    };
    SCOREBOARD.record(ok, || format!("send_friendly_request(pedido {:x}) nós {} -> {} situação {} prevista {}", m, before, list_len(l), read_u8(m + friendly_match::STATUS), s));
}

/// Comparer for `build_nation_list`.
///
/// [pt] Comparador da 0x5ca3e0: só a original (aloca a lista). O plano, feito antes numa lista local com o mesmo qsort e a
/// mesma ordem: a conta (+0x34fe) e as nações, na ordem.
pub(crate) unsafe extern "thiscall" fn compare_build_nation_list(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BUILD_NATION_LIST);
    let mut plan = nations_with_continent();
    sort_by_reputation(plan.as_mut_ptr() as usize, plan.len() as u32);
    o(this);
    let p = read_usize(this + friendly::SORTED_NATIONS);
    let n = read_i32(this + friendly::SORTED_NATION_COUNT);
    let ok = p == 0 || (n as usize == plan.len() && (0..plan.len()).all(|k| read_usize(p + 4 * k) == plan[k]));
    SCOREBOARD.record(ok, || format!("build_nation_list: conta {} prevista {}", n, plan.len()));
}

/// Comparador: pura.
pub(crate) unsafe extern "stdcall" fn compare_friendly_club_reputation_tier(club: usize) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u8>(IDX_FRIENDLY_CLUB_REPUTATION_TIER)(club);
    let n = friendly_club_reputation_tier(club);
    SCOREBOARD.record(r == n, || format!("friendly_club_reputation_tier({:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `friendly_get_rest_days_for_kind`.
///
/// [pt] Comparador: pura; os tipos da caixa, só a original.
pub(crate) unsafe extern "stdcall" fn compare_friendly_get_rest_days_for_kind(kind: u32) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(u32) -> u8>(IDX_FRIENDLY_GET_REST_DAYS_FOR_KIND)(kind);
    if !matches!(kind as u8 as i8, 0..=4 | 10 | 11) {
        return r;
    }
    let n = friendly_get_rest_days_for_kind(kind);
    SCOREBOARD.record(r == n, || format!("friendly_get_rest_days_for_kind({}) original={} rust={}", kind as u8 as i8, r, n));
    r
}

/// Comparer for `friendly_update_for_date`.
///
/// [pt] Comparador: anda o calendário e planeja (aloca, sorteia): só a original.
pub(crate) unsafe extern "thiscall" fn compare_friendly_update_for_date(this: usize, today: usize, active: u32) -> u32 {
    // simplification: cancels tournaments, removes requests and creates friendlies (does not run twice); the copy is proven by the career
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_FRIENDLY_UPDATE_FOR_DATE)(this, today, active)
}

/// Comparer for `plan_ai_preseason_friendlies`.
///
/// [pt] Comparador: monta e planeja os amistosos da IA (aloca, sorteia): só a original.
pub(crate) unsafe extern "thiscall" fn compare_plan_ai_preseason_friendlies(this: usize) {
    // simplification: the tournaments, the tours and the friendlies roll and allocate; the copy is proved by the career
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PLAN_AI_PRESEASON_FRIENDLIES)(this)
}

/// Comparer for `set_preseason_windows`.
///
/// [pt] Comparador: grava as janelas e cria listas (não roda duas vezes): o plano antes (sem gravar), a original grava;
/// cada candidato planejado tem de ter a janela e, quando é a ponta da tabela, uma lista em +0x12.
pub(crate) unsafe extern "thiscall" fn compare_set_preseason_windows(this: usize, ptrs: usize, n: i32) {
    let plan = set_friendly_windows(ptrs, n, false);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32)>(IDX_SET_PRESEASON_WINDOWS)(this, ptrs, n);
    let erros: Vec<String> = plan.iter()
        .filter(|&&(c, ini, fim, nova)| {
            core::ptr::read_unaligned((c + preseason_candidate::WINDOW_START) as *const i16) != ini || core::ptr::read_unaligned((c + preseason_candidate::WINDOW_END) as *const i16) != fim
                || (nova && read_usize(c + preseason_candidate::PLANNED_DATES) == 0)
        })
        .map(|&(c, ini, fim, nova)| format!("candidato {:x}: esperado {}..{} lista {} obtido {}..{} lista {:x}", c, ini, fim, nova,
            core::ptr::read_unaligned((c + preseason_candidate::WINDOW_START) as *const i16), core::ptr::read_unaligned((c + preseason_candidate::WINDOW_END) as *const i16), read_usize(c + preseason_candidate::PLANNED_DATES)))
        .collect();
    SCOREBOARD.record(erros.is_empty(), || format!("set_preseason_windows(n {}) {} candidatos: {}", n, plan.len(), erros.join("; ")));
}

/// Comparer for `schedule_tour_matches`.
///
/// [pt] Comparador: sorteia, monta amistosos e cria listas (não roda duas vezes): só a original.
pub(crate) unsafe extern "thiscall" fn compare_schedule_tour_matches(this: usize, cand: usize, adv: usize, n: u32) -> u32 {
    // simplification: the draw and the new friendlies do not run twice; the copy is proven by the career
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_SCHEDULE_TOUR_MATCHES)(this, cand, adv, n)
}

/// Comparer for `compare_friendly_candidates_by_band`.
///
/// [pt] Comparador: rotina pura, pela devolução.
pub(crate) unsafe extern "C" fn compare_compare_friendly_candidates_by_band(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_FRIENDLY_CANDIDATES_BY_BAND)(a, b);
    let n = compare_friendly_candidates_by_band(a, b);
    SCOREBOARD.record(r == n, || format!("compare_friendly_candidates_by_band({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `compare_friendly_candidate_ptrs_by_nation`.
///
/// [pt] Comparador: rotina pura, pela devolução.
pub(crate) unsafe extern "C" fn compare_compare_friendly_candidate_ptrs_by_nation(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_FRIENDLY_CANDIDATE_PTRS_BY_NATION)(a, b);
    let n = compare_friendly_candidate_ptrs_by_nation(a, b);
    SCOREBOARD.record(r == n, || format!("compare_friendly_candidate_ptrs_by_nation({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `create_due_planned_friendlies`.
pub(crate) unsafe extern "thiscall" fn compare_create_due_planned_friendlies(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_DUE_PLANNED_FRIENDLIES)(this)
}

/// Comparer for `add_friendly_match`.
///
/// It allocates and appends to the list (it does not run twice): the original does it and the copy checks the result
/// against the home side's stadium, looked up beforehand (0x5ae870, read-only): occupied → 0; otherwise the returned
/// friendly has the date, +0xc the club, +8 = 0, +0x18 = 1 and home and away sides according to `em_casa`.
///
/// [pt] Comparador: aloca e acrescenta à lista (não roda duas vezes): a original faz e a cópia confere o resultado com o
/// estádio de quem joga em casa visto antes (0x5ae870, só lê): ocupado → 0; senão o amistoso devolvido tem a data,
/// +0xc o clube, +8 = 0, +0x18 = 1 e o mandante e o visitante pelo `em_casa`.
pub(crate) unsafe extern "thiscall" fn compare_add_friendly_match(this: usize, club: usize, data: usize, opponent: usize, in_home: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> usize>(IDX_ADD_FRIENDLY_MATCH);
    let ocupada = home_stadium_busy_on(club, data, opponent, in_home);
    let (dia, ano) = (read_u16(data), read_u16(data + 2));
    let r = o(this, club, data, opponent, in_home);
    let (casa, fora) = if in_home != 0 { (club, opponent) } else { (opponent, club) };
    let same = if ocupada {
        r == 0
    } else {
        // simplification: r = 0 without being occupied only when out of memory (the box has already shown): not checked
        r == 0 || (read_u16(r) == dia && read_u16(r + 2) == ano && read_usize(r + friendly_match::REQUESTING_CLUB) == club && read_usize(r + 8) == 0
            && *((r + friendly_match::STATUS) as *const u8) == 1 && read_usize(r + friendly_match::HOME_CLUB) == casa && read_usize(r + friendly_match::AWAY_CLUB) == fora)
    };
    SCOREBOARD.record(same, || format!("add_friendly_match(clube {:x}, {}/{}, adversario {:x}, em casa {}) ocupada {} original={:x}",
        club, dia, ano, opponent, in_home, ocupada, r));
    r
}

/// Comparer for `plan_friendly_on_free_date`.
pub(crate) unsafe extern "thiscall" fn compare_plan_friendly_on_free_date(this: usize, anfitriao: usize, home: usize, away: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(IDX_PLAN_FRIENDLY_ON_FREE_DATE)(this, anfitriao, home, away)
}

/// Comparer for `friendly_is_club_free_on_date`.
///
/// [pt] Comparador: a cópia roda com a lista provisória dela, como a original: pela devolução.
pub(crate) unsafe extern "stdcall" fn compare_friendly_is_club_free_on_date(data: usize, club: usize, kind: u32) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, usize, u32) -> u32>(IDX_FRIENDLY_IS_CLUB_FREE_ON_DATE)(data, club, kind);
    let n = friendly_is_club_free_on_date(data, club, kind);
    SCOREBOARD.record(r == n, || format!("friendly_is_club_free_on_date(data {:x}, clube {:x}, tipo {}) original={} rust={}", data, club, kind as u8 as i8, r, n));
    r
}

/// Comparer for `is_banned_nation_pair`.
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "thiscall" fn compare_is_banned_nation_pair(this: usize, a: i32, b: i32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32) -> u32>(IDX_IS_BANNED_NATION_PAIR)(this, a, b);
    let n = is_banned_nation_pair(this, a, b);
    SCOREBOARD.record(r == n, || format!("is_banned_nation_pair({}, {}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `compare_nations_by_reputation_desc`.
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn compare_compare_nations_by_reputation_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_NATIONS_BY_REPUTATION_DESC)(a, b);
    let n = compare_nations_by_reputation_desc(a, b);
    SCOREBOARD.record(r == n, || format!("compare_nations_by_reputation_desc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `compare_fixture_records_by_date`.
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn compare_compare_fixture_records_by_date(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_FIXTURE_RECORDS_BY_DATE)(a, b);
    let n = compare_fixture_records_by_date(a, b);
    SCOREBOARD.record(r == n, || format!("compare_fixture_records_by_date({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `remove_friendly_match` (comparison mode: see the header of motor.rs): `0x68d1f0` frees what it
/// removes, so the plan is made first.
///
/// The plan beforehand (the items that remain in both lists and the friendly's slot); the original routine runs; then
/// it checks the lists, the cursor of `+0x34d4` at the start and the slot zeroed. A null match or no list `+0x34c8`:
/// only the original routine (the box).
///
/// [pt] Comparador: a 0x68d1f0 libera o que tira: o plano antes (os itens que ficam nas duas listas e a vaga do amistoso);
/// a original roda; confere as listas, o cursor da +0x34d4 no começo e a vaga zerada. Jogo nulo ou sem a lista
/// +0x34c8: só a original (a caixa).
pub(crate) unsafe extern "thiscall" fn compare_remove_friendly_match(this: usize, game: usize, clear_slot: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_REMOVE_FRIENDLY_MATCH);
    if game == 0 || read_usize(this + friendly::FRIENDLY_MATCH_LIST) == 0 {
        return o(this, game, clear_slot);
    }
    // simplification: 0x68d1f0 finds the node by the item’s content; the plan assumes items with distinct content (the normal case)
    let l4 = read_usize(this + friendly::REQUEST_REPLY_LIST);
    let esperado4: Vec<usize> = if l4 != 0 { list_items_capped(l4).into_iter().filter(|&e| read_usize(e) != game).collect() } else { Vec::new() };
    let mut esperado8 = list_items_capped(read_usize(this + friendly::FRIENDLY_MATCH_LIST));
    if let Some(k) = esperado8.iter().position(|&e| e == game) {
        esperado8.remove(k);
    }
    let v = if clear_slot != 0 {
        let slot: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) -> usize = core::mem::transmute(FN_FRIENDLY_FIND_SCHEDULED_FRIENDLY_SLOT);
        slot(this, *(game as *const u16) as u32, *((game + 2) as *const u16) as u32, read_usize(game + friendly_match::AWAY_CLUB), read_usize(game + friendly_match::HOME_CLUB))
    } else {
        0
    };
    let r = o(this, game, clear_slot);
    let l4 = read_usize(this + friendly::REQUEST_REPLY_LIST);
    let itens4 = if l4 != 0 { list_items_capped(l4) } else { Vec::new() };
    let itens8 = list_items_capped(read_usize(this + friendly::FRIENDLY_MATCH_LIST));
    let ok = itens4 == esperado4
        && (l4 == 0 || read_usize(l4 + llist::CURSOR) == read_usize(l4 + llist::HEAD))
        && itens8 == esperado8
        && (v == 0 || (read_usize(v + scheduled_friendly_slot::OPPONENT) == 0 && read_u8(v + scheduled_friendly_slot::STATUS) == 0));
    SCOREBOARD.record(ok, || format!("remove_friendly_match({:#x}, {}) itens {} e {} (esperava {} e {}), vaga {:#x}", game, clear_slot, itens4.len(), itens8.len(), esperado4.len(), esperado8.len(), v));
    r
}

/// Comparer for `init_banned_nation_pairs`.
///
/// [pt] Comparador: o malloc não pode rodar duas vezes: a original roda e confere o vetor e a conta com o plano.
pub(crate) unsafe extern "thiscall" fn compare_init_banned_nation_pairs(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_INIT_BANNED_NATION_PAIRS)(this);
    let v = read_usize(this + friendly::BANNED_NATION_PAIRS);
    let ok = if v == 0 {
        read_u8(this + friendly::BANNED_PAIR_COUNT) == 0
    } else {
        let expected = [read_usize(KEY_NATION_IRAN), read_usize(KEY_NATION_UNITED_STATES), read_usize(KEY_NATION_IRAQ), usize::MAX, read_usize(KEY_NATION_VIETNAM), read_usize(KEY_NATION_UNITED_STATES), read_usize(KEY_NATION_NORTH_KOREA), read_usize(KEY_NATION_SOUTH_KOREA)];
        read_u8(this + friendly::BANNED_PAIR_COUNT) == 4 && (0..8).all(|k| read_usize(v + 4 * k) == expected[k])
    };
    SCOREBOARD.record(ok, || format!("init_banned_nation_pairs: conta {} vetor {:#x}", read_u8(this + friendly::BANNED_PAIR_COUNT), v));
}

/// Comparer for `is_comp_current_on_date`.
///
/// [pt] Comparador: pelo valor (as datas e a vaga só são lidas). TapeCall pelas copas e ligas a cada data: sem contador.
pub(crate) unsafe extern "thiscall" fn compare_is_comp_current_on_date(this: usize, comp: usize, data: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_IS_COMP_CURRENT_ON_DATE)(this, comp, data);
    let n = is_comp_current_on_date(this, comp, data);
    SCOREBOARD.record(r == n, || format!("is_comp_current_on_date({}, {:?}) original={} rust={}", read_usize(comp), friendly_date_day_year(data), r, n));
    r
}

/// Comparer for `candidate_has_nearby_date`.
///
/// [pt] Comparador: lado a lado pelo cursor da lista (+0xa) e o valor.
pub(crate) unsafe extern "thiscall" fn compare_candidate_has_nearby_date(this: usize, club: usize, data: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_CANDIDATE_HAS_NEARBY_DATE);
    let l = read_usize(club + preseason_candidate::PLANNED_DATES);
    let reg: &[(usize, usize)] = if l == 0 { &[] } else { &[(l + llist::CURSOR, 4)] };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(reg, || o(this, club, data), || candidate_has_nearby_date(this, club, data));
    SCOREBOARD.record(igual && r == n, || format!("candidate_has_nearby_date({:?}) original={} rust={} {:x?} x {:x?}", friendly_date_day_year(data), r, n, mo, mr));
    r
}

/// Comparer for `add_planned_friendly`.
///
/// [pt] Comparador: a original acrescenta (pode realocar, não roda duas vezes): o plano antes (a conta, os ids e a data),
/// a original roda, confere o item novo e a conta.
pub(crate) unsafe extern "thiscall" fn compare_add_planned_friendly(this: usize, anfitriao: usize, home: usize, away: usize, data: usize) {
    let n = read_usize(this + friendly::PLANNED_COUNT);
    let expected = [read_usize(data), read_usize(data + 4), 0, read_usize(anfitriao), read_usize(home), read_usize(away)];
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize)>(IDX_ADD_PLANNED_FRIENDLY)(this, anfitriao, home, away, data);
    let v = read_usize(this + friendly::PLANNED_FRIENDLIES);
    if v == 0 {
        return; // out of memory: the box has already shown
    }
    let item = v.wrapping_add(n.wrapping_mul(0x14));
    let done = [read_usize(item), read_usize(item + 4), 0, read_usize(item + planned_friendly::HOST_CLUB_ID), read_usize(item + planned_friendly::HOME_CLUB_ID), read_usize(item + planned_friendly::AWAY_CLUB_ID)];
    SCOREBOARD.record(done == expected && read_usize(this + friendly::PLANNED_COUNT) == n.wrapping_add(1), || {
        format!("add_planned_friendly conta {} -> {}, item {:x?} (esperava {:x?})", n, read_usize(this + friendly::PLANNED_COUNT), done, expected)
    });
}

/// Comparer for `find_friendly_opponent`.
pub(crate) unsafe extern "thiscall" fn compare_find_friendly_opponent(this: usize, club: usize, list: usize, n: u32, band: u32, excluir: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize) -> usize>(IDX_FIND_FRIENDLY_OPPONENT);
    let reg = candidate_regions((0..(n as i32).max(0) as usize).map(|i| list + 0x16 * i));
    let (r, m, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club, list, n, band, excluir),
        || crate::rng::without_counting(|| find_friendly_opponent(this, club, list, n, band, excluir)));
    SCOREBOARD.record(igual && r == m, || format!("find_friendly_opponent({:x}, {:x}, n {}, band {}) original={:x} rust={:x} {:x?} x {:x?}",
        club, list, n, band as i8, r, m, mo, mr));
    r
}

/// Comparer for `find_tour_opponent`.
pub(crate) unsafe extern "thiscall" fn compare_find_tour_opponent(this: usize, club: usize, ordenados: usize, n: u32, band_max: u32, nation: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize) -> usize>(IDX_FIND_TOUR_OPPONENT);
    let reg = candidate_regions((0..(n as i32).max(0) as usize).map(|i| read_usize(ordenados + 4 * i)));
    let (r, m, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club, ordenados, n, band_max, nation),
        || crate::rng::without_counting(|| find_tour_opponent(this, club, ordenados, n, band_max, nation)));
    SCOREBOARD.record(igual && r == m, || format!("find_tour_opponent({:x}, {:x}, n {}, band {}, nacao {:x}) original={:x} rust={:x} {:x?} x {:x?}",
        club, ordenados, n, band_max as i8, nation, r, m, mo, mr));
    r
}

/// Comparer for `is_own_comp_record`.
pub(crate) unsafe extern "thiscall" fn compare_is_own_comp_record(friendly: usize, competition: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_IS_OWN_COMP_RECORD)(friendly, competition);
    let n = is_own_comp_record(friendly, competition);
    SCOREBOARD.record(r == n, || format!("is_own_comp_record({:#x}) original={} rust={}", competition, r, n));
    r
}

/// Comparer for `friendly_international_date_set`.
pub(crate) unsafe extern "thiscall" fn compare_friendly_international_date_set(data: usize, day: u8, month: u8, duracao: u8, year: u8, continent: u32, competition: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u8, u8, u8, u8, u32, u32)>(IDX_FRIENDLY_INTERNATIONAL_DATE_SET);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(data, 0x14)], || o(data, day, month, duracao, year, continent, competition),
        || friendly_international_date_set(data, day, month, duracao, year, continent, competition));
    SCOREBOARD.record(igual, || format!("friendly_international_date_set {:x?} x {:x?}", mo, mr));
}

/// Comparer for `schedule_assign_opponent`.
///
/// [pt] Comparador: lado a lado pelas vagas da agenda (até a última possível, 25 · (n + 1) + 4 bytes).
pub(crate) unsafe extern "thiscall" fn compare_schedule_assign_opponent(this: usize, agenda: usize, opponent: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_SCHEDULE_ASSIGN_OPPONENT);
    let n = read_u8(agenda + 8) as i8;
    if n <= 0 {
        return o(this, agenda, opponent);
    }
    let reg = [(agenda, 25 * (n as usize + 1) + 4)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, agenda, opponent), || schedule_assign_opponent(this, agenda, opponent));
    SCOREBOARD.record(igual, || format!("schedule_assign_opponent({:x}, {:x}) {:x?} x {:x?}", agenda, opponent, mo, mr));
}

/// Comparer for `register_comp_objects`.
pub(crate) unsafe extern "thiscall" fn compare_register_comp_objects(friendly: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_REGISTER_COMP_OBJECTS);
    let table = *(G_COMP_OBJECT_TABLE as *const usize) as isize;
    let base = core::ptr::read_unaligned((friendly + friendly::FIRST_TOURNAMENT_COMP_ID) as *const i16) as isize;
    let reg = [((table + *(KEY_CLUB_COMP_FRIENDLY as *const i32) as isize * 4) as usize, 4), ((table + base * 4) as usize, 0x7f * 4)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(friendly), || register_comp_objects(friendly));
    SCOREBOARD.record(igual, || format!("register_comp_objects {:x?} x {:x?}", mo, mr));
}

/// Comparer for `update_international_friendlies`.
///
/// On the first day of the year (the lists are emptied, freeing memory) and when there are friendlies to arrange
/// (0x5cbdf0 creates matches and draws at random), only the original routine runs, checking the plan: the day set at
/// +0x34f4.
///
/// On the other days, side by side on the set day and the static dates.
///
/// [pt] Comparador: no primeiro dia do ano (as listas se esvaziam, liberando) e quando há amistosos a marcar (a 0x5cbdf0
/// cria jogos e sorteia), só a original, conferindo o plano: o dia marcado em +0x34f4. Nos outros dias, lado a lado
/// pelo dia marcado e pelas datas estáticas.
pub(crate) unsafe extern "thiscall" fn compare_update_international_friendlies(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_UPDATE_INTERNATIONAL_FRIENDLIES);
    if read_u16(G_CURRENT_DATE) == 0 {
        return o(this);
    }
    if let Some((primeira, quantas)) = update_friendlies_body(this, true) {
        o(this);
        let day = read_u16(this + friendly::INTL_LAST_DAY_DONE);
        SCOREBOARD.record(day == read_u16(G_CURRENT_DATE), || format!("update_international_friendlies(primeira {:x}, quantas {}) dia marcado={} hoje={}", primeira, quantas, day, read_u16(G_CURRENT_DATE)));
        return;
    }
    let mut reg = SCOUT_MANAGER_DATE_STATICS.to_vec();
    reg.push((this + friendly::INTL_LAST_DAY_DONE, 2));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || update_international_friendlies(this));
    SCOREBOARD.record(igual, || format!("update_international_friendlies(sem amistosos) {:x?} x {:x?}", mo, mr));
}

/// Comparer for `find_scheduled_friendly_slot`.
///
/// [pt] Comparador: pelo valor (as duas voltam o cursor da lista ao começo e param no mesmo registro).
pub(crate) unsafe extern "thiscall" fn compare_find_scheduled_friendly_slot(this: usize, day: u32, year: u32, club: usize, opponent: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) -> usize>(IDX_FIND_SCHEDULED_FRIENDLY_SLOT)(this, day, year, club, opponent);
    let n = find_scheduled_friendly_slot(this, day, year, club, opponent);
    SCOREBOARD.record(r == n, || format!("find_scheduled_friendly_slot({}/{}, {:x}, {:x}) original={:#x} rust={:#x}", day as u16, year as u16, club, opponent, r, n));
    r
}

/// Comparer for `remove_club_from_ai_plans`.
pub(crate) unsafe extern "thiscall" fn compare_remove_club_from_ai_plans(amistosos: usize, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REMOVE_CLUB_FROM_AI_PLANS);
    let mut regions = vec![];
    let array = *((amistosos + friendly::PLANNED_FRIENDLIES) as *const usize);
    let n = *((amistosos + friendly::PLANNED_COUNT) as *const i32);
    if array != 0 && n > 0 {
        regions.push((array, n as usize * 0x14));
    }
    let list = *((amistosos + friendly::FRIENDLY_MATCH_LIST) as *const usize);
    if list != 0 {
        // the cursor and the +0xc pointer of each friendly in the list (traversed here before; the two
        // rotinas reiniciam o cursor)
        regions.push((list + llist::CURSOR, 4));
        let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
        core::ptr::write_unaligned((list + llist::CURSOR) as *mut u32, core::ptr::read_unaligned((list + llist::HEAD) as *const u32));
        loop {
            let e = next(list);
            if e == 0 {
                break;
            }
            regions.push((e + friendly_match::REQUESTING_CLUB, 4));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(amistosos, club),
        || remove_club_from_ai_plans(amistosos, club));
    SCOREBOARD.record(igual, || format!("remove_club_from_ai_plans({:#x}) {:x?} x {:x?}", club, mo, mr));
}

/// Comparer for `pick_tour_nation`.
pub(crate) unsafe extern "thiscall" fn compare_pick_tour_nation(this: usize, club: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_PICK_TOUR_NATION);
    let reg = crate::rng::STATE_REGIONS.to_vec();
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club),
        || crate::rng::without_counting(|| pick_tour_nation(this, club)));
    SCOREBOARD.record(igual && r == n, || format!("pick_tour_nation({:#x}) original={:#x} rust={:#x} {:x?} x {:x?}", club, r, n, mo, mr));
    r
}

/// Comparer for `friendly_international_date_resolve`.
pub(crate) unsafe extern "thiscall" fn compare_friendly_international_date_resolve(this: usize, year: u32) -> u32 {
    let o = original_fn::<usize>(IDX_FRIENDLY_INTERNATIONAL_DATE_RESOLVE);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 11)], || call_original_with_year(o, this, year),
        || friendly_international_date_resolve(this, year));
    SCOREBOARD.record(igual /* o eax é lixo de pilha: só a memória */, || format!("friendly_international_date_resolve({:x}, {:x}) original={:x} rust={:x} {:x?} x {:x?}",
        this, year, r, n, mo, mr));
    r
}
