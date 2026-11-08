use super::*;

// The comparers of the routines in ../comp/friendly.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/friendly.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `on_friendly_fixture_cancelled` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5c6570: a original entrega notícias, cancela torneios e tira jogos do calendário (libera): só ela
/// roda. Antes, a cópia acha a vaga do amistoso (0x5c6740, só leitura); depois a vaga tem de estar livre (+0x10 = 0 e
/// +0x18 = 0).
pub(crate) unsafe extern "thiscall" fn comparar_on_friendly_fixture_cancelled(this: usize, jogo: usize, who: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_ON_FRIENDLY_FIXTURE_CANCELLED);
    let mut slot = 0usize;
    if read_usize(jogo + 0x14) == read_usize(this + 4) && read_usize(this + 0x34cc) != 0 {
        let slot_of: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_FRIENDLY_FIND_SCHEDULED_FRIENDLY_SLOT);
        slot = slot_of(this, replace_low_word(read_usize(jogo + 0x1c), read_i16(jogo + 0x2a)), replace_low_word(read_usize(jogo + 0x14), read_i16(jogo + 0x28)),
            read_usize(jogo + 0x20) as u32, read_usize(jogo + 0x1c) as u32);
    }
    o(this, jogo, who);
    let free = slot == 0 || (read_usize(slot + 0x10) == 0 && read_u8(slot + 0x18) == 0);
    PLACAR.registrar(free, || format!("on_friendly_fixture_cancelled(jogo {:#x}) a vaga {:#x} não ficou livre", jogo, slot));
}

/// Comparer for `friendly_cancel_national_fixture` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5cb9e0: a original tira jogos do calendário (libera): só ela roda (simplification: a cópia não tem o que
/// conferir sem refazer a busca no calendário já mudado; a contagem do ramo prova no modo Rust).
pub(crate) unsafe extern "thiscall" fn comparar_friendly_cancel_national_fixture(this: usize, jogo: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FRIENDLY_CANCEL_NATIONAL_FIXTURE)(this, jogo);
}

/// Comparer for `cancel_tournament` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5c8260: a original entrega notícias, tira jogos do calendário e destrói a competição: só ela roda,
/// com o plano conferido depois: o dono (+0) e a competição (+0x65) zerados, a vaga da tabela de competições zerada e
/// o torneio fora da lista dos pendentes.
pub(crate) unsafe extern "thiscall" fn comparar_cancel_tournament(this: usize, tournament: usize, without_owner: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_CANCEL_TOURNAMENT);
    if tournament == 0 {
        o(this, tournament, without_owner);
        return;
    }
    let k = (read_i16(this + 0xaf) as i32 + read_u8(tournament + 0x64) as i8 as i32) as usize;
    let slot = read_usize(G_COMP_OBJECT_TABLE).wrapping_add(k.wrapping_mul(4));
    o(this, tournament, without_owner);
    let mut in_list = false;
    let p = read_usize(this + 0x34d0);
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
    let ok = read_usize(tournament) == 0 && read_usize(tournament + 0x65) == 0 && read_usize(slot) == 0 && !in_list;
    PLACAR.registrar(ok, || format!("cancel_tournament(torneio {:#x}) dono {:#x} competição {:#x} vaga {:#x} na lista {}",
        tournament, read_usize(tournament), read_usize(tournament + 0x65), read_usize(slot), in_list));
}

/// Comparer for `arrange_international_friendlies` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original cria amistosos (0x5c6f00, 0x5c5f40), acrescenta às listas (0x68d0d0) e sorteia: só ela roda,
/// com o plano em ensaio. A primeira parte do ensaio roda antes dela (depois, o calendário já teria os amistosos
/// novos); a segunda, depois dela, pela fita dos sorteios que ela fez (aleatorio::rastrear: n e resposta, os de dentro
/// das rotinas do amistoso também), decide cada amistoso pelo que a original fez. Confere a sequência dos sorteios (os
/// do ensaio são os da original, salvo os de dentro das rotinas do amistoso, e a fita acaba junto) e, lista a lista de
/// nações, que os itens novos da original são os que o ensaio acrescentaria. (Lado a lado pelo estado do sorteador não
/// servia: a 0x5c6f00 e a 0x5c5f40 sorteiam por dentro na original e o ensaio não as chama.)
pub(crate) unsafe extern "thiscall" fn comparar_arrange_international_friendlies(this: usize, datas: usize, n: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_ARRANGE_INTERNATIONAL_FRIENDLIES);
    let nacoes = read_usize(G_NATION_COUNT) as i32;
    let k = nacoes.max(read_usize(this + 0x34fe) as i32).max(0) as usize;
    let antes: Vec<Vec<u32>> = (0..k).map(|q| list_item_ids(nation_friendly_list(this, q))).collect();
    let mut e = FriendlyComparerTrial { usadas: vec![0u8; nacoes.max(0) as usize], antes: std::rc::Rc::new(antes), novos: vec![Vec::new(); k] };
    link_nations_near_date(this, datas, 0, Some(&mut e));
    let ((), tape_lock) = crate::aleatorio::rastrear(|| o(this, datas, n));
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
    PLACAR.registrar(igual && dif.is_empty(), || {
        format!("arrange_international_friendlies(datas {:x}, {}) tape_lock de {} sorteios, o ensaio parou em {} (n diferente: {}); listas (indice, original, ensaio) {:x?}",
            datas, n as i16, quantos, pos, falhou, dif.iter().take(4).collect::<Vec<_>>())
    });
}

/// Comparer for `friendly_check_match_invitation` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo texto e a devolução (as listas locais nascem e morrem nas duas); com a caixa, só a
/// original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_friendly_check_match_invitation(this: usize, a: usize, b: usize, clube: usize, day: u32,
    year: u32, data: usize, text: usize, size: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, usize, usize, u32) -> u32>(IDX_FRIENDLY_CHECK_MATCH_INVITATION);
    if text != 0 && (size as i16) < 100 {
        return o(this, a, b, clube, day, year, data, text, size);
    }
    let reg = news_text_regions(text, size);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, a, b, clube, day, year, data, text, size),
        || friendly_check_match_invitation(this, a, b, clube, day, year, data, text, size));
    PLACAR.registrar(igual && r == n, || format!("friendly_check_match_invitation(a {:x}, b {:x}, clube {:x}, data {:x}) original={} rust={} {:x?} x {:x?}",
        a, b, clube, data, r, n, mo, mr));
    r
}

/// Comparer for `friendly_check_tournament_invitation` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo texto e a devolução; com a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_friendly_check_tournament_invitation(this: usize, clube: usize, tournament: usize, text: usize, size: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(IDX_FRIENDLY_CHECK_TOURNAMENT_INVITATION);
    if text != 0 && (size as i16) < 100 {
        return o(this, clube, tournament, text, size);
    }
    let reg = news_text_regions(text, size);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, tournament, text, size),
        || friendly_check_tournament_invitation(this, clube, tournament, text, size));
    PLACAR.registrar(igual && r == n, || format!("friendly_check_tournament_invitation(clube {:x}, torneio {:x}) original={} rust={} {:x?} x {:x?}",
        clube, tournament, r, n, mo, mr));
    r
}

/// Comparer for `build_international_date_table` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5ca4f0: a cópia primeiro (aloca a tabela dela, que sai copiada e liberada: free 0x9452ca, cdecl),
/// depois a original a partir dos mesmos +0x34ec..+0x34f5; confere a conta, o último dia feito e as 110 datas. Roda
/// uma vez, no construtor dos amistosos.
pub(crate) unsafe extern "thiscall" fn comparar_build_international_date_table(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BUILD_INTERNATIONAL_DATE_TABLE);
    let campos = crate::comparacao::Memoria::ler(&[(this + 0x34ec, 10)]);
    build_international_date_table(this);
    let (t, n) = (read_usize(this + 0x34ec), read_i32(this + 0x34f0));
    if t == 0 || n != 0x6e {
        campos.escrever();
        return o(this); // simplification: sem memória na cópia, só a original
    }
    let mr = crate::comparacao::Memoria::ler(&[(t, 0x898), (this + 0x34f0, 6)]);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    free(t);
    campos.escrever();
    o(this);
    let t = read_usize(this + 0x34ec);
    if t == 0 {
        return PLACAR.registrar(false, || "build_international_date_table: a original ficou sem tabela".to_string());
    }
    let mo = crate::comparacao::Memoria::ler(&[(t, 0x898), (this + 0x34f0, 6)]);
    PLACAR.registrar(mo.0[0].1 == mr.0[0].1 && mo.0[1].1 == mr.0[1].1, || format!("build_international_date_table original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `resolve_international_dates` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5cc510: lado a lado nas datas da tabela (só grava nelas; o qsort e a 0x54a130 não sorteiam).
pub(crate) unsafe extern "thiscall" fn comparar_resolve_international_dates(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_RESOLVE_INTERNATIONAL_DATES);
    let (t, n) = (read_usize(this + 0x34ec), read_i32(this + 0x34f0));
    let reg = if t != 0 && n > 0 { vec![(t, n as usize * 0x14)] } else { Vec::new() };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || resolve_international_dates(this));
    PLACAR.registrar(igual, || format!("resolve_international_dates(conta {}) original={:x?} rust={:x?}", n, mo, mr));
}

/// Comparer for `plan_club_friendlies` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5c4770: só a original (simplification: ela libera e aloca o plano, e a 0x5c4380 aloca as listas de datas e
/// de adversários dos candidatos; não roda duas vezes), conferindo o plano que ela deixa: o plano alocado, a capacidade
/// (+0x34e4) não menor que a conta (+0x34e0), o +0x34dc zerado e o plano em ordem de data (a 0x778e90 entre vizinhos).
pub(crate) unsafe extern "thiscall" fn comparar_plan_club_friendlies(this: usize, entradas: usize, ordenados: usize, count: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_PLAN_CLUB_FRIENDLIES)(this, entradas, ordenados, count);
    let (p, n, cap, z) = (read_usize(this + 0x34d8), read_i32(this + 0x34e0), read_i32(this + 0x34e4), read_i32(this + 0x34dc));
    if p == 0 {
        return; // sem memória: a caixa
    }
    let order: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(FN_COMPARE_CALL_UP_RECORDS_BY_DATE);
    let ordenado = (1..n.max(0) as usize).all(|i| order(p + (i - 1) * 0x14, p + i * 0x14) <= 0);
    PLACAR.registrar(n >= 0 && cap >= n && z == 0 && ordenado, || format!("plan_club_friendlies: plano {:#x} conta {} capacidade {} +0x34dc {} em ordem {}", p, n, cap, z, ordenado));
}

/// Comparer for `add_tournament` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5c6e00: lado a lado no torneio livre (0x69 bytes, mais o que um nome comprido passar) e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_add_tournament(this: usize, anfitriao: usize, kind: u32, start: usize, a: u32, b: u32, c: u32, d: u32, e: u32, convidados: usize, name: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32, u32, u32, usize, usize) -> usize>(IDX_ADD_TOURNAMENT);
    let t = free_tournament_slot(this);
    let mut size = 0usize;
    while name != 0 && *((name + size) as *const u8) != 0 {
        size += 1;
    }
    let reg = if t != 0 { vec![(t, 0x69usize.max(0x31 + size + 1))] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, anfitriao, kind, start, a, b, c, d, e, convidados, name), || add_tournament(this, anfitriao, kind, start, a, b, c, d, e, convidados, name));
    PLACAR.registrar(igual && r == n, || format!("add_tournament(anfitriao {:x}) original={:#x} {:x?} rust={:#x} {:x?}", anfitriao, r, mo, n, mr));
    r
}

/// Comparer for `find_human_club_open_invitation` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5c68b0: lado a lado no cursor da lista (+0xa) e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_find_human_club_open_invitation(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_FIND_HUMAN_CLUB_OPEN_INVITATION);
    let list = read_usize(this + 0x34c8);
    let reg = if list != 0 { vec![(list + 0xa, 4)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || find_human_club_open_invitation(this));
    PLACAR.registrar(igual && r == n, || format!("find_human_club_open_invitation original={:#x} {:x?} rust={:#x} {:x?}", r, mo, n, mr));
    r
}

/// Comparer for `process_friendly_reply` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5c5f40: só a original (a partida vai ao calendário e a notícia ao técnico, que alocam, e o pedido é
/// liberado pela 0x5c87b0). O plano, feito antes: a decisão (friendly_reply) e a vaga do amistoso (0x5c6740). Com 3 ou 4,
/// a vaga fica com +0x18 = 3, ou com +0x10 = 0 e +0x18 = 0; sem decisão, o pedido continua com a mesma situação e sai
/// da lista das respostas (+0x34d4). Sem quem pede ou sem nação num dos dois: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_process_friendly_reply(this: usize, m: usize, force: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_PROCESS_FRIENDLY_REPLY);
    LAST_FRIENDLY_REPLY = m;
    let a = if m != 0 { read_usize(m + 0xc) } else { 0 };
    let other = if m != 0 { other_friendly_club(m) } else { 0 };
    if a == 0 || (other != 0 && (read_usize(a + 0x53) == 0 || read_usize(other + 0x53) == 0)) {
        return o(this, m, force); // simplification: sem quem pede ou sem nação, só a original
    }
    let s = if friendly_needs_decision(m, force) { friendly_reply(m, false) } else { read_u8(m + 0x18) };
    let slot: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) -> usize = core::mem::transmute(FN_FRIENDLY_FIND_SCHEDULED_FRIENDLY_SLOT);
    let v = if s == 3 || s == 4 { slot(this, read_i16(m) as u16 as u32, read_i16(m + 2) as u16 as u32, read_usize(m + 0x14), read_usize(m + 0x10)) } else { 0 };
    o(this, m, force);
    let ok = if s == 3 || s == 4 {
        // o pedido foi liberado: só a vaga
        v == 0 || if s == 3 { read_u8(v + 0x18) == 3 } else { read_usize(v + 0x10) == 0 && read_u8(v + 0x18) == 0 }
    } else {
        read_u8(m + 0x18) == s && !list_holds(read_usize(this + 0x34d4), m)
    };
    PLACAR.registrar(ok, || format!("process_friendly_reply(pedido {:x}) situação prevista {} vaga {:x}", m, s, v));
}

/// Comparer for `send_friendly_request` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5c5570: só a original (o nó da lista e a notícia alocam). O plano, feito antes: a lista das
/// respostas fica com um nó a mais e guarda o pedido; a situação vai a 2 quando o outro clube, na situação 1, é de
/// técnico humano; senão não muda. A notícia 6 ao humano de férias é respondida na hora pelo assistente (0x784100:
/// situação 1 e a 0x5c5f40 com `forcar`, conferida pelo comparador dela): então o pedido já saiu da lista (a lista fica
/// com os nós de antes) e pode ter sido liberado (a situação não se lê).
pub(crate) unsafe extern "thiscall" fn comparar_send_friendly_request(this: usize, m: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SEND_FRIENDLY_REQUEST);
    let other = other_friendly_club(m);
    let before = list_len(read_usize(this + 0x34d4));
    let s = if other != 0 && read_u8(m + 0x18) == 1 && club_has_human_manager(other) { 2 } else { read_u8(m + 0x18) };
    LAST_FRIENDLY_REPLY = 0;
    o(this, m);
    let l = read_usize(this + 0x34d4);
    let respondido = s == 2 && LAST_FRIENDLY_REPLY == m;
    if respondido {
        crate::comparacao::ramo("cqa_2 pedido respondido pelo assistente");
    }
    let ok = l == 0 || if respondido {
        list_len(l) == before && !list_holds(l, m)
    } else {
        list_len(l) == before + 1 && list_holds(l, m) && read_u8(m + 0x18) == s
    };
    PLACAR.registrar(ok, || format!("send_friendly_request(pedido {:x}) nós {} -> {} situação {} prevista {}", m, before, list_len(l), read_u8(m + 0x18), s));
}

/// Comparer for `build_nation_list` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5ca3e0: só a original (aloca a lista). O plano, feito antes numa lista local com o mesmo qsort e a
/// mesma ordem: a conta (+0x34fe) e as nações, na ordem.
pub(crate) unsafe extern "thiscall" fn comparar_build_nation_list(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BUILD_NATION_LIST);
    let mut plan = nations_with_continent();
    sort_by_reputation(plan.as_mut_ptr() as usize, plan.len() as u32);
    o(this);
    let p = read_usize(this + 0x34fa);
    let n = read_i32(this + 0x34fe);
    let ok = p == 0 || (n as usize == plan.len() && (0..plan.len()).all(|k| read_usize(p + 4 * k) == plan[k]));
    PLACAR.registrar(ok, || format!("build_nation_list: conta {} prevista {}", n, plan.len()));
}

/// Comparador: pura.
pub(crate) unsafe extern "stdcall" fn comparar_friendly_club_reputation_tier(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u8>(IDX_FRIENDLY_CLUB_REPUTATION_TIER)(clube);
    let n = friendly_club_reputation_tier(clube);
    PLACAR.registrar(r == n, || format!("friendly_club_reputation_tier({:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `friendly_get_rest_days_for_kind` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pura; os tipos da caixa, só a original.
pub(crate) unsafe extern "stdcall" fn comparar_friendly_get_rest_days_for_kind(kind: u32) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(u32) -> u8>(IDX_FRIENDLY_GET_REST_DAYS_FOR_KIND)(kind);
    if !matches!(kind as u8 as i8, 0..=4 | 10 | 11) {
        return r;
    }
    let n = friendly_get_rest_days_for_kind(kind);
    PLACAR.registrar(r == n, || format!("friendly_get_rest_days_for_kind({}) original={} rust={}", kind as u8 as i8, r, n));
    r
}

/// Comparer for `friendly_update_for_date` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: anda o calendário e planeja (aloca, sorteia): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_friendly_update_for_date(this: usize, today: usize, active: u32) -> u32 {
    // simplification: cancela torneios, tira pedidos e cria amistosos (não roda duas vezes); a cópia se prova pela carreira
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_FRIENDLY_UPDATE_FOR_DATE)(this, today, active)
}

/// Comparer for `plan_ai_preseason_friendlies` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: monta e planeja os amistosos da IA (aloca, sorteia): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_plan_ai_preseason_friendlies(this: usize) {
    // simplification: os torneios, as excursões e os amistosos sorteiam e alocam; a cópia se prova pela carreira
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PLAN_AI_PRESEASON_FRIENDLIES)(this)
}

/// Comparer for `set_preseason_windows` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: grava as janelas e cria listas (não roda duas vezes): o plano antes (sem gravar), a original grava;
/// cada candidato planejado tem de ter a janela e, quando é a ponta da tabela, uma lista em +0x12.
pub(crate) unsafe extern "thiscall" fn comparar_set_preseason_windows(this: usize, ptrs: usize, n: i32) {
    let plan = set_friendly_windows(ptrs, n, false);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32)>(IDX_SET_PRESEASON_WINDOWS)(this, ptrs, n);
    let erros: Vec<String> = plan.iter()
        .filter(|&&(c, ini, fim, nova)| {
            core::ptr::read_unaligned((c + 0xe) as *const i16) != ini || core::ptr::read_unaligned((c + 0x10) as *const i16) != fim
                || (nova && read_u32_as_usize(c + 0x12) == 0)
        })
        .map(|&(c, ini, fim, nova)| format!("candidato {:x}: esperado {}..{} lista {} obtido {}..{} lista {:x}", c, ini, fim, nova,
            core::ptr::read_unaligned((c + 0xe) as *const i16), core::ptr::read_unaligned((c + 0x10) as *const i16), read_u32_as_usize(c + 0x12)))
        .collect();
    PLACAR.registrar(erros.is_empty(), || format!("set_preseason_windows(n {}) {} candidatos: {}", n, plan.len(), erros.join("; ")));
}

/// Comparer for `schedule_tour_matches` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sorteia, monta amistosos e cria listas (não roda duas vezes): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_schedule_tour_matches(this: usize, cand: usize, adv: usize, n: u32) -> u32 {
    // simplification: o sorteio e os amistosos novos não rodam duas vezes; a cópia se prova pela carreira
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_SCHEDULE_TOUR_MATCHES)(this, cand, adv, n)
}

/// Comparer for `compare_friendly_candidates_by_band` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: rotina pura, pela devolução.
pub(crate) unsafe extern "C" fn comparar_compare_friendly_candidates_by_band(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_FRIENDLY_CANDIDATES_BY_BAND)(a, b);
    let n = compare_friendly_candidates_by_band(a, b);
    PLACAR.registrar(r == n, || format!("compare_friendly_candidates_by_band({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `compare_friendly_candidate_ptrs_by_nation` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: rotina pura, pela devolução.
pub(crate) unsafe extern "C" fn comparar_compare_friendly_candidate_ptrs_by_nation(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_FRIENDLY_CANDIDATE_PTRS_BY_NATION)(a, b);
    let n = compare_friendly_candidate_ptrs_by_nation(a, b);
    PLACAR.registrar(r == n, || format!("compare_friendly_candidate_ptrs_by_nation({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `create_due_planned_friendlies` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_create_due_planned_friendlies(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_DUE_PLANNED_FRIENDLIES)(this)
}

/// Comparer for `add_friendly_match` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: aloca e acrescenta à lista (não roda duas vezes): a original faz e a cópia confere o resultado com o
/// estádio de quem joga em casa visto antes (0x5ae870, só lê): ocupado → 0; senão o amistoso devolvido tem a data,
/// +0xc o clube, +8 = 0, +0x18 = 1 e o mandante e o visitante pelo `em_casa`.
pub(crate) unsafe extern "thiscall" fn comparar_add_friendly_match(this: usize, clube: usize, data: usize, opponent: usize, in_home: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> usize>(IDX_ADD_FRIENDLY_MATCH);
    let ocupada = home_stadium_busy_on(clube, data, opponent, in_home);
    let (dia, ano) = (read_u16(data), read_u16(data + 2));
    let r = o(this, clube, data, opponent, in_home);
    let (casa, fora) = if in_home != 0 { (clube, opponent) } else { (opponent, clube) };
    let same = if ocupada {
        r == 0
    } else {
        // simplification: r = 0 sem estar ocupada só sem memória (a caixa já saiu): não confere
        r == 0 || (read_u16(r) == dia && read_u16(r + 2) == ano && read_u32_as_usize(r + 0xc) == clube && read_u32_as_usize(r + 8) == 0
            && *((r + 0x18) as *const u8) == 1 && read_u32_as_usize(r + 0x10) == casa && read_u32_as_usize(r + 0x14) == fora)
    };
    PLACAR.registrar(same, || format!("add_friendly_match(clube {:x}, {}/{}, adversario {:x}, em casa {}) ocupada {} original={:x}",
        clube, dia, ano, opponent, in_home, ocupada, r));
    r
}

/// Comparer for `plan_friendly_on_free_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_plan_friendly_on_free_date(this: usize, anfitriao: usize, home: usize, away: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(IDX_PLAN_FRIENDLY_ON_FREE_DATE)(this, anfitriao, home, away)
}

/// Comparer for `friendly_is_club_free_on_date` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a cópia roda com a lista provisória dela, como a original: pela devolução.
pub(crate) unsafe extern "stdcall" fn comparar_friendly_is_club_free_on_date(data: usize, clube: usize, kind: u32) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, usize, u32) -> u32>(IDX_FRIENDLY_IS_CLUB_FREE_ON_DATE)(data, clube, kind);
    let n = friendly_is_club_free_on_date(data, clube, kind);
    PLACAR.registrar(r == n, || format!("friendly_is_club_free_on_date(data {:x}, clube {:x}, tipo {}) original={} rust={}", data, clube, kind as u8 as i8, r, n));
    r
}

/// Comparer for `is_banned_nation_pair` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "thiscall" fn comparar_is_banned_nation_pair(this: usize, a: i32, b: i32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32) -> u32>(IDX_IS_BANNED_NATION_PAIR)(this, a, b);
    let n = is_banned_nation_pair(this, a, b);
    PLACAR.registrar(r == n, || format!("is_banned_nation_pair({}, {}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `compare_nations_by_reputation_desc` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn comparar_compare_nations_by_reputation_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_NATIONS_BY_REPUTATION_DESC)(a, b);
    let n = compare_nations_by_reputation_desc(a, b);
    PLACAR.registrar(r == n, || format!("compare_nations_by_reputation_desc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `compare_fixture_records_by_date` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn comparar_compare_fixture_records_by_date(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_FIXTURE_RECORDS_BY_DATE)(a, b);
    let n = compare_fixture_records_by_date(a, b);
    PLACAR.registrar(r == n, || format!("compare_fixture_records_by_date({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `remove_friendly_match` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a 0x68d1f0 libera o que tira: o plano antes (os itens que ficam nas duas listas e a vaga do amistoso);
/// a original roda; confere as listas, o cursor da +0x34d4 no começo e a vaga zerada. Jogo nulo ou sem a lista
/// +0x34c8: só a original (a caixa).
pub(crate) unsafe extern "thiscall" fn comparar_remove_friendly_match(this: usize, jogo: usize, clear_slot: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_REMOVE_FRIENDLY_MATCH);
    if jogo == 0 || read_usize(this + 0x34c8) == 0 {
        return o(this, jogo, clear_slot);
    }
    // simplification: a 0x68d1f0 acha o nó pelo conteúdo do item; o plano supõe itens de conteúdo distinto (o normal)
    let l4 = read_usize(this + 0x34d4);
    let esperado4: Vec<usize> = if l4 != 0 { list_items_capped(l4).into_iter().filter(|&e| read_usize(e) != jogo).collect() } else { Vec::new() };
    let mut esperado8 = list_items_capped(read_usize(this + 0x34c8));
    if let Some(k) = esperado8.iter().position(|&e| e == jogo) {
        esperado8.remove(k);
    }
    let v = if clear_slot != 0 {
        let slot: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) -> usize = core::mem::transmute(FN_FRIENDLY_FIND_SCHEDULED_FRIENDLY_SLOT);
        slot(this, *(jogo as *const u16) as u32, *((jogo + 2) as *const u16) as u32, read_usize(jogo + 0x14), read_usize(jogo + 0x10))
    } else {
        0
    };
    let r = o(this, jogo, clear_slot);
    let l4 = read_usize(this + 0x34d4);
    let itens4 = if l4 != 0 { list_items_capped(l4) } else { Vec::new() };
    let itens8 = list_items_capped(read_usize(this + 0x34c8));
    let ok = itens4 == esperado4
        && (l4 == 0 || read_usize(l4 + 0xa) == read_usize(l4 + 2))
        && itens8 == esperado8
        && (v == 0 || (read_usize(v + 0x10) == 0 && read_u8(v + 0x18) == 0));
    PLACAR.registrar(ok, || format!("remove_friendly_match({:#x}, {}) itens {} e {} (esperava {} e {}), vaga {:#x}", jogo, clear_slot, itens4.len(), itens8.len(), esperado4.len(), esperado8.len(), v));
    r
}

/// Comparer for `init_banned_nation_pairs` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o malloc não pode rodar duas vezes: a original roda e confere o vetor e a conta com o plano.
pub(crate) unsafe extern "thiscall" fn comparar_init_banned_nation_pairs(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_INIT_BANNED_NATION_PAIRS)(this);
    let v = read_usize(this + 0x3502);
    let ok = if v == 0 {
        read_u8(this + 0x3506) == 0
    } else {
        let expected = [read_usize(KEY_NATION_IRAN), read_usize(KEY_NATION_UNITED_STATES), read_usize(KEY_NATION_IRAQ), usize::MAX, read_usize(KEY_NATION_VIETNAM), read_usize(KEY_NATION_UNITED_STATES), read_usize(KEY_NATION_NORTH_KOREA), read_usize(KEY_NATION_SOUTH_KOREA)];
        read_u8(this + 0x3506) == 4 && (0..8).all(|k| read_usize(v + 4 * k) == expected[k])
    };
    PLACAR.registrar(ok, || format!("init_banned_nation_pairs: conta {} vetor {:#x}", read_u8(this + 0x3506), v));
}

/// Comparer for `is_comp_current_on_date` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (as datas e a vaga só são lidas). TapeCall pelas copas e ligas a cada data: sem contador.
pub(crate) unsafe extern "thiscall" fn comparar_is_comp_current_on_date(this: usize, comp: usize, data: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_IS_COMP_CURRENT_ON_DATE)(this, comp, data);
    let n = is_comp_current_on_date(this, comp, data);
    PLACAR.registrar(r == n, || format!("is_comp_current_on_date({}, {:?}) original={} rust={}", read_usize(comp), friendly_date_day_year(data), r, n));
    r
}

/// Comparer for `candidate_has_nearby_date` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo cursor da lista (+0xa) e o valor.
pub(crate) unsafe extern "thiscall" fn comparar_candidate_has_nearby_date(this: usize, clube: usize, data: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_CANDIDATE_HAS_NEARBY_DATE);
    let l = read_usize(clube + 0x12);
    let reg: &[(usize, usize)] = if l == 0 { &[] } else { &[(l + 0xa, 4)] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(reg, || o(this, clube, data), || candidate_has_nearby_date(this, clube, data));
    PLACAR.registrar(igual && r == n, || format!("candidate_has_nearby_date({:?}) original={} rust={} {:x?} x {:x?}", friendly_date_day_year(data), r, n, mo, mr));
    r
}

/// Comparer for `add_planned_friendly` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original acrescenta (pode realocar, não roda duas vezes): o plano antes (a conta, os ids e a data),
/// a original roda, confere o item novo e a conta.
pub(crate) unsafe extern "thiscall" fn comparar_add_planned_friendly(this: usize, anfitriao: usize, home: usize, away: usize, data: usize) {
    let n = read_usize(this + 0x34e0);
    let expected = [read_usize(data), read_usize(data + 4), 0, read_usize(anfitriao), read_usize(home), read_usize(away)];
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize)>(IDX_ADD_PLANNED_FRIENDLY)(this, anfitriao, home, away, data);
    let v = read_usize(this + 0x34d8);
    if v == 0 {
        return; // sem memória: a caixa já saiu
    }
    let item = v.wrapping_add(n.wrapping_mul(0x14));
    let done = [read_usize(item), read_usize(item + 4), 0, read_usize(item + 8), read_usize(item + 0xc), read_usize(item + 0x10)];
    PLACAR.registrar(done == expected && read_usize(this + 0x34e0) == n.wrapping_add(1), || {
        format!("add_planned_friendly conta {} -> {}, item {:x?} (esperava {:x?})", n, read_usize(this + 0x34e0), done, expected)
    });
}

/// Comparer for `find_friendly_opponent` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_find_friendly_opponent(this: usize, clube: usize, list: usize, n: u32, band: u32, excluir: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize) -> usize>(IDX_FIND_FRIENDLY_OPPONENT);
    let reg = candidate_regions((0..(n as i32).max(0) as usize).map(|i| list + 0x16 * i));
    let (r, m, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, list, n, band, excluir),
        || crate::aleatorio::sem_contar(|| find_friendly_opponent(this, clube, list, n, band, excluir)));
    PLACAR.registrar(igual && r == m, || format!("find_friendly_opponent({:x}, {:x}, n {}, band {}) original={:x} rust={:x} {:x?} x {:x?}",
        clube, list, n, band as i8, r, m, mo, mr));
    r
}

/// Comparer for `find_tour_opponent` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_find_tour_opponent(this: usize, clube: usize, ordenados: usize, n: u32, faixa_max: u32, nacao: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize) -> usize>(IDX_FIND_TOUR_OPPONENT);
    let reg = candidate_regions((0..(n as i32).max(0) as usize).map(|i| read_usize(ordenados + 4 * i)));
    let (r, m, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, ordenados, n, faixa_max, nacao),
        || crate::aleatorio::sem_contar(|| find_tour_opponent(this, clube, ordenados, n, faixa_max, nacao)));
    PLACAR.registrar(igual && r == m, || format!("find_tour_opponent({:x}, {:x}, n {}, band {}, nacao {:x}) original={:x} rust={:x} {:x?} x {:x?}",
        clube, ordenados, n, faixa_max as i8, nacao, r, m, mo, mr));
    r
}

/// Comparer for `is_own_comp_record` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_is_own_comp_record(friendly: usize, competicao: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_IS_OWN_COMP_RECORD)(friendly, competicao);
    let n = is_own_comp_record(friendly, competicao);
    PLACAR.registrar(r == n, || format!("is_own_comp_record({:#x}) original={} rust={}", competicao, r, n));
    r
}

/// Comparer for `friendly_international_date_set` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_friendly_international_date_set(data: usize, day: u8, month: u8, duracao: u8, year: u8, continent: u32, competicao: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u8, u8, u8, u8, u32, u32)>(IDX_FRIENDLY_INTERNATIONAL_DATE_SET);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(data, 0x14)], || o(data, day, month, duracao, year, continent, competicao),
        || friendly_international_date_set(data, day, month, duracao, year, continent, competicao));
    PLACAR.registrar(igual, || format!("friendly_international_date_set {:x?} x {:x?}", mo, mr));
}

/// Comparer for `schedule_assign_opponent` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelas vagas da agenda (até a última possível, 25 · (n + 1) + 4 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_schedule_assign_opponent(this: usize, agenda: usize, opponent: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_SCHEDULE_ASSIGN_OPPONENT);
    let n = read_u8(agenda + 8) as i8;
    if n <= 0 {
        return o(this, agenda, opponent);
    }
    let reg = [(agenda, 25 * (n as usize + 1) + 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, agenda, opponent), || schedule_assign_opponent(this, agenda, opponent));
    PLACAR.registrar(igual, || format!("schedule_assign_opponent({:x}, {:x}) {:x?} x {:x?}", agenda, opponent, mo, mr));
}

/// Comparer for `register_comp_objects` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_register_comp_objects(friendly: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_REGISTER_COMP_OBJECTS);
    let table = *(G_COMP_OBJECT_TABLE as *const usize) as isize;
    let base = core::ptr::read_unaligned((friendly + 0xaf) as *const i16) as isize;
    let reg = [((table + *(KEY_CLUB_COMP_FRIENDLY as *const i32) as isize * 4) as usize, 4), ((table + base * 4) as usize, 0x7f * 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(friendly), || register_comp_objects(friendly));
    PLACAR.registrar(igual, || format!("register_comp_objects {:x?} x {:x?}", mo, mr));
}

/// Comparer for `update_international_friendlies` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: no primeiro dia do ano (as listas se esvaziam, liberando) e quando há amistosos a marcar (a 0x5cbdf0
/// cria jogos e sorteia), só a original, conferindo o plano: o dia marcado em +0x34f4. Nos outros dias, lado a lado
/// pelo dia marcado e pelas datas estáticas.
pub(crate) unsafe extern "thiscall" fn comparar_update_international_friendlies(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_UPDATE_INTERNATIONAL_FRIENDLIES);
    if read_u16(G_CURRENT_DATE) == 0 {
        return o(this);
    }
    if let Some((primeira, quantas)) = update_friendlies_body(this, true) {
        o(this);
        let day = read_u16(this + 0x34f4);
        PLACAR.registrar(day == read_u16(G_CURRENT_DATE), || format!("update_international_friendlies(primeira {:x}, quantas {}) dia marcado={} hoje={}", primeira, quantas, day, read_u16(G_CURRENT_DATE)));
        return;
    }
    let mut reg = SCOUT_MANAGER_DATE_STATICS.to_vec();
    reg.push((this + 0x34f4, 2));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || update_international_friendlies(this));
    PLACAR.registrar(igual, || format!("update_international_friendlies(sem amistosos) {:x?} x {:x?}", mo, mr));
}

/// Comparer for `find_scheduled_friendly_slot` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (as duas voltam o cursor da lista ao começo e param no mesmo registro).
pub(crate) unsafe extern "thiscall" fn comparar_find_scheduled_friendly_slot(this: usize, day: u32, year: u32, clube: usize, opponent: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) -> usize>(IDX_FIND_SCHEDULED_FRIENDLY_SLOT)(this, day, year, clube, opponent);
    let n = find_scheduled_friendly_slot(this, day, year, clube, opponent);
    PLACAR.registrar(r == n, || format!("find_scheduled_friendly_slot({}/{}, {:x}, {:x}) original={:#x} rust={:#x}", day as u16, year as u16, clube, opponent, r, n));
    r
}

/// Comparer for `remove_club_from_ai_plans` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_remove_club_from_ai_plans(amistosos: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REMOVE_CLUB_FROM_AI_PLANS);
    let mut regions = vec![];
    let array = *((amistosos + 0x34d8) as *const usize);
    let n = *((amistosos + 0x34e0) as *const i32);
    if array != 0 && n > 0 {
        regions.push((array, n as usize * 0x14));
    }
    let list = *((amistosos + 0x34c8) as *const usize);
    if list != 0 {
        // o cursor e o ponteiro +0xc de cada amistoso da lista (percorrida aqui antes; as duas
        // rotinas reiniciam o cursor)
        regions.push((list + 0xa, 4));
        let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
        core::ptr::write_unaligned((list + 0xa) as *mut u32, core::ptr::read_unaligned((list + 2) as *const u32));
        loop {
            let e = next(list);
            if e == 0 {
                break;
            }
            regions.push((e + 0xc, 4));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regions, || o(amistosos, clube),
        || remove_club_from_ai_plans(amistosos, clube));
    PLACAR.registrar(igual, || format!("remove_club_from_ai_plans({:#x}) {:x?} x {:x?}", clube, mo, mr));
}

/// Comparer for `pick_tour_nation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_pick_tour_nation(this: usize, clube: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_PICK_TOUR_NATION);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube),
        || crate::aleatorio::sem_contar(|| pick_tour_nation(this, clube)));
    PLACAR.registrar(igual && r == n, || format!("pick_tour_nation({:#x}) original={:#x} rust={:#x} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}

/// Comparer for `friendly_international_date_resolve` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_friendly_international_date_resolve(this: usize, year: u32) -> u32 {
    let o = original_fn::<usize>(IDX_FRIENDLY_INTERNATIONAL_DATE_RESOLVE);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 11)], || call_original_with_year(o, this, year),
        || friendly_international_date_resolve(this, year));
    PLACAR.registrar(igual /* o eax é lixo de pilha: só a memória */, || format!("friendly_international_date_resolve({:x}, {:x}) original={:x} rust={:x} {:x?} x {:x?}",
        this, year, r, n, mo, mr));
    r
}
