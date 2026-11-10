use super::*;

// The comparers of the routines in ../news.rs.
// [pt] Os comparadores das rotinas de ../news.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `mark_item_read`.
///
/// [pt] Comparador: lado a lado no vetor das notícias (News +0, +4 vagas), nas entradas dos 16 humanos (News +8) e no texto
/// do botão (0xde1f64; a 0x782b70 só consulta). Caixa "Error": só a original.
pub(crate) unsafe extern "thiscall" fn compare_mark_item_read(this: usize, manager: usize, id: u32, sincronizar: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(IDX_MARK_ITEM_READ);
    if manager == 0 || id == 0 || read_i32(manager).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10) < 0 || read_usize(this) == 0 {
        return o(this, manager, id, sincronizar);
    }
    let regions = [
        (read_usize(this), read_i32(this + news_fields::SLOT_COUNT).max(0) as usize * STORED_NEWS_SIZE),
        (read_usize(this + news_fields::HUMAN_INBOXES), 0x10 * 12),
        (0xde1f64, 0x7d0),
    ];
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&regions, || o(this, manager, id, sincronizar), || mark_item_read(this, manager, id, sincronizar));
    SCOREBOARD.record(igual && r == n, || format!("mark_item_read(id {}, sincronizar {}) original={} rust={} memoria igual {}", id, sincronizar, r, n, igual));
    r
}

/// Comparer for `find_item_by_id`.
///
/// [pt] Comparador: pela notícia copiada em `saida` (0xdf bytes) e pela devolução. Caixa "Error": só a original.
pub(crate) unsafe extern "thiscall" fn compare_find_item_by_id(this: usize, manager: usize, id: u32, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(IDX_FIND_ITEM_BY_ID);
    if id == 0 || out == 0 || (manager != 0 && read_i32(manager).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10) < 0) {
        return o(this, manager, id, out);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(out, STORED_NEWS_SIZE)], || o(this, manager, id, out), || find_item_by_id(this, manager, id, out));
    SCOREBOARD.record(igual && r == n, || format!("find_item_by_id(id {}) original={} rust={} {:x?} x {:x?}", id, r, n, mo.0[0].1, mr.0[0].1));
    r
}

/// Comparer for `send_by_scope`.
///
/// [pt] Comparador: a entrega pode cair na caixa do humano (realloc, ids, resposta automática): roda só a original e confere
/// o alcance; alcance fora de 0..5 devolve 0.
pub(crate) unsafe extern "thiscall" fn compare_send_by_scope(this: usize, news: usize, club: usize, competition: usize, nation: usize, continent: usize, reach: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32) -> u32>(IDX_SEND_BY_SCOPE)(this, news, club, competition, nation, continent, reach);
    let away = !(0..=5).contains(&(reach as u8 as i8));
    SCOREBOARD.record(!away || r == 0, || format!("send_by_scope alcance {} devolveu {}", reach as u8 as i8, r));
    r
}

/// Comparer for `send_to_club_manager`.
///
/// [pt] Comparador: roda só a original (a entrega não pode rodar duas vezes) e confere o plano: o alcance zerado e, sem técnico
/// humano, a devolução 0 (com humano, a entrega devolve 1 ou 0 sem memória).
pub(crate) unsafe extern "thiscall" fn compare_send_to_club_manager(this: usize, news: usize, club: usize) -> u32 {
    let club = Club(club);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SEND_TO_CLUB_MANAGER);
    if news == 0 || club.is_null() {
        return o(this, news, club.addr());
    }
    let manager = club.manager().addr();
    let human = manager != 0 && read_i32(manager) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10);
    let r = o(this, news, club.addr());
    let ok = *((news + news_item::REACH) as *const u8) == 0 && (human || r == 0) && r <= 1;
    SCOREBOARD.record(ok, || format!("send_to_club_manager(clube {:x}, técnico {:x}, humano {}) devolveu {}", club.addr(), manager, human, r));
    r
}

/// Comparer for `deliver_to_manager`.
///
/// [pt] Comparador: a original realoca o vetor, numera e responde: roda só ela e confere o plano feito antes: o id da notícia,
/// o contador de ids (News +0x10), o anel do humano (início, próxima vaga, conta), a vaga escrita com o id e +0xda = 0.
pub(crate) unsafe extern "thiscall" fn compare_deliver_to_manager(this: usize, news: usize, manager: usize, free: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_DELIVER_TO_MANAGER);
    if news == 0 || manager == 0 || news_human_manager_index(manager) < 0 {
        return o(this, news, manager, free);
    }
    let idx = news_human_manager_index(manager) as usize;
    let e = read_usize(this + news_fields::HUMAN_INBOXES) + idx * 12;
    // o plano
    let new = read_i32(e) == -1;
    let (inicio, atual, conta) = if new { (read_i32(this + news_fields::SLOT_COUNT), read_i32(this + news_fields::SLOT_COUNT), read_i32(e + 8)) } else { (read_i32(e), read_i32(e + 4), read_i32(e + 8)) };
    let counter = read_usize(this + news_fields::LAST_ID);
    let (id, contador_depois) = if read_usize(news + news_item::ID) == 0 { (counter.wrapping_add(1), counter.wrapping_add(1)) } else { (read_usize(news + news_item::ID), counter) };
    let mut previous = atual - 1;
    if previous < inicio {
        previous = inicio + 0x63;
    }
    let (repetida, livre_atual) = if new {
        (id == 0, true)
    } else {
        let v = |k: i32| read_usize(this).wrapping_add((k as usize).wrapping_mul(NEWS_NEWS_RECORD_SIZE));
        (read_usize(v(previous) + 0xd6) == id, read_usize(v(atual) + 0xda) != 0)
    };
    let (prox, conta_depois) = if repetida {
        (atual, conta)
    } else {
        let p = if atual + 1 == inicio + 0x64 { inicio } else { atual + 1 };
        (p, conta + if livre_atual { 1 } else { 0 })
    };
    // with the manager on holiday the automatic reply (0x784100) may deliver another news item to the same human: the ring moves
    // more slots and the count together (found on 31/10/2026, when the routine left DESLIGADAS)
    let responde = !repetida && (*(G_NEWS_SCREEN_AUTO_OPEN_BLOCKED_FLAG as *const i16) != 0 || exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER), manager) != 0);
    let r = o(this, news, manager, free);
    if r == 0 {
        SCOREBOARD.record(new, || format!("deliver_to_manager devolveu 0 sem anel novo")); // only without memory
        return r;
    }
    let extra = if responde { (read_i32(e + 4) - prox).rem_euclid(0x64) } else { 0 };
    let slot = read_usize(this).wrapping_add((atual as usize).wrapping_mul(NEWS_NEWS_RECORD_SIZE));
    let ok = r == 1 && read_i32(e) == inicio && read_i32(e + 4) == if extra == 0 { prox } else { read_i32(e + 4) }
        && read_i32(e + 8) >= conta_depois && read_i32(e + 8) <= conta_depois + extra
        && (read_usize(this + news_fields::LAST_ID) == contador_depois || (extra > 0 && read_usize(this + news_fields::LAST_ID) > contador_depois))
        && read_usize(news + news_item::ID) == id && read_usize(news + news_item::READ) == 0
        && (repetida || read_usize(slot + news_item::ID) == id);
    SCOREBOARD.record(ok, || format!("deliver_to_manager(humano {}) id {} (esperava {}), anel {}/{}/{} (esperava {}/{}/{}), repetida {}",
        idx, read_usize(news + news_item::ID), id, read_i32(e), read_i32(e + 4), read_i32(e + 8), inicio, prox, conta_depois, repetida));
    r
}

/// Comparer for `auto_respond_all`.
///
/// [pt] Comparador: a original responde (convites, propostas, emprego): roda só ela e confere a devolução (0 só sem técnico
/// ou com técnico da máquina; senão 1).
pub(crate) unsafe extern "thiscall" fn compare_auto_respond_all(this: usize, manager: usize, club: usize) -> u32 {
    let human = manager != 0 && read_i32(manager).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10) >= 0;
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_AUTO_RESPOND_ALL)(this, manager, club);
    SCOREBOARD.record(r == human as u32, || format!("auto_respond_all(técnico {:x}, humano {}) devolveu {}", manager, human, r));
    r
}

/// Comparer for `has_urgent_items`.
///
/// [pt] Comparador: lado a lado pela devolução e pelo texto montado em 0xde1f64 (a 0x782b70 não sorteia; a caixa não muda).
pub(crate) unsafe extern "thiscall" fn compare_has_urgent_items(this: usize, manager: usize, so_urgentes: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_HAS_URGENT_ITEMS);
    if so_urgentes != 0 && read_i32(manager).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10) < 0 {
        return o(this, manager, so_urgentes); // a caixa "Error"
    }
    let reg = [(0xde1f64usize, 0x7d0usize)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, manager, so_urgentes), || has_urgent_items(this, manager, so_urgentes));
    SCOREBOARD.record(igual && r == n, || format!("has_urgent_items(urgentes {}) {} x {} {:x?} x {:x?}", so_urgentes, r, n, mo, mr));
    r
}

/// Comparer for `set_slot_string`.
///
/// [pt] Comparador: lado a lado pelos campos da notícia (+5..+0xcc) e pelo próximo campo.
pub(crate) unsafe extern "thiscall" fn compare_set_slot_string(this: usize, field: u32, text: *const u8, next: *mut u8) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *const u8, *mut u8) -> u32>(IDX_SET_SLOT_STRING);
    if !(0..0x32).contains(&(field as u8 as i8)) || text.is_null() {
        return o(this, field, text, next); // a caixa "Error"
    }
    let mut reg = vec![(this + 5, 0xc8usize)];
    if !next.is_null() {
        reg.push((next as usize, 1));
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, field, text, next), || set_slot_string(this, field, text, next));
    SCOREBOARD.record(igual && r == n, || format!("set_slot_string(campo {}) {:x?} x {:x?}", field as u8 as i8, mo, mr));
    r
}

/// Comparer for `send_to_continent_managers`.
///
/// [pt] Comparadores: a entrega (0x785d40) põe a notícia nas caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
pub(crate) unsafe extern "thiscall" fn compare_send_to_continent_managers(this: usize, news: usize, continent: usize) -> u32 {
    crate::comparison::branch("send_to_continent_managers");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SEND_TO_CONTINENT_MANAGERS)(this, news, continent)
}

/// Comparer for `send_to_comp_region`.
pub(crate) unsafe extern "thiscall" fn compare_send_to_comp_region(this: usize, news: usize, comp: usize) -> u32 {
    crate::comparison::branch("send_to_comp_region");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SEND_TO_COMP_REGION)(this, news, comp)
}

/// Comparer for `is_important_for`.
///
/// [pt] Comparador: pelo valor (a 0x785670 só lê).
pub(crate) unsafe extern "thiscall" fn compare_is_important_for(news: usize, person: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_IS_IMPORTANT_FOR)(news, person);
    let t = read_usize(news) as i32;
    if person == 0 || !(1..=0x1790).contains(&t) {
        return r;
    }
    let n = is_important_for(news, person);
    SCOREBOARD.record(r == n, || format!("is_important_for(tipo {:#x}, {:x}) original={} rust={}", t, person, r, n));
    r
}

/// Comparador: lado a lado pelos `n` bytes do destino.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_string(news: usize, given: u32, target: *mut u8, n: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut u8, i32) -> u32>(IDX_GET_SLOT_STRING);
    let k = given as u8 as i8;
    if !(0..0x32).contains(&k) || target.is_null() || n < 1 {
        return o(news, given, target, n);
    }
    let reg = [(target as usize, n as usize)];
    let (r, m, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(news, given, target, n), || get_slot_string(news, given, target, n));
    SCOREBOARD.record(igual && r == m, || format!("get_slot_string({}, {}) {:x?} x {:x?}", k, n, mo, mr));
    r
}

/// Comparer for `send_to_all_managers`.
///
/// [pt] Comparadores: a entrega (0x785d40) põe a notícia nas caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
pub(crate) unsafe extern "thiscall" fn compare_send_to_all_managers(this: usize, news: usize) -> u32 {
    crate::comparison::branch("send_to_all_managers");
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SEND_TO_ALL_MANAGERS)(this, news)
}

/// Comparer for `send_to_division_managers`.
pub(crate) unsafe extern "thiscall" fn compare_send_to_division_managers(this: usize, news: usize, division: usize) -> u32 {
    crate::comparison::branch("send_to_division_managers");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SEND_TO_DIVISION_MANAGERS)(this, news, division)
}

/// Comparer for `send_to_nation_managers`.
pub(crate) unsafe extern "thiscall" fn compare_send_to_nation_managers(this: usize, news: usize, nation: usize) -> u32 {
    crate::comparison::branch("send_to_nation_managers");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SEND_TO_NATION_MANAGERS)(this, news, nation)
}

/// Comparer for `send_to_nation_club_managers`.
///
/// [pt] Comparador: a entrega (0x785d40) põe a notícia nas caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
pub(crate) unsafe extern "thiscall" fn compare_send_to_nation_club_managers(this: usize, news: usize, nation: usize) -> u32 {
    crate::comparison::branch("send_to_nation_club_managers");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SEND_TO_NATION_CLUB_MANAGERS)(this, news, nation)
}

/// Comparer for `count_unread`.
pub(crate) unsafe extern "thiscall" fn compare_count_unread(news: usize, manager: usize, category: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(IDX_COUNT_UNREAD);
    let entry = if manager == 0 { None } else { manager_inbox_entry(news, manager) };
    let Some(entry) = entry else {
        return o(news, manager, category);
    };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(entry + 8, 4)], || o(news, manager, category),
        || count_unread(news, manager, category));
    SCOREBOARD.record(igual && r == n, || format!("count_unread(categoria {}) original={} rust={} {:x?} x {:x?}", category as u8, r, n, mo, mr));
    r
}

/// Comparer for `set_item_slot`.
pub(crate) unsafe extern "thiscall" fn compare_set_item_slot(news: usize, manager: usize, id: u32, slot: i32, value: u32, copy: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, usize) -> u32>(IDX_SET_ITEM_SLOT);
    let Ok(alvo) = set_slot_target(news, manager, id, slot, copy) else {
        return o(news, manager, id, slot, value, copy);
    };
    let mut regions = vec![];
    if copy != 0 {
        regions.push((copy + 5 + 4 * slot as usize, 4));
    }
    if let Some(a) = alvo {
        regions.push((a + 5 + 4 * slot as usize, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(news, manager, id, slot, value, copy),
        || set_item_slot(news, manager, id, slot, value, copy));
    SCOREBOARD.record(igual && r == n, || format!("set_item_slot(vaga {}) original={} rust={} {:x?} x {:x?}", slot, r, n, mo, mr));
    r
}

/// Comparer for `news_club_has_human_manager`.
pub(crate) unsafe extern "stdcall" fn compare_news_club_has_human_manager(club: usize) -> u32 {
    // queries only: both run again with no effect
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_NEWS_CLUB_HAS_HUMAN_MANAGER)(club);
    let n = news_club_has_human_manager(club);
    SCOREBOARD.record(r == n, || format!("news_club_has_human_manager original={} rust={}", r, n));
    r
}

/// Comparer for `clear_inbox`.
///
/// [pt] Comparador: lado a lado na caixa e nas 100 notícias; com resposta padrão a dar (clube ou seleção), só a original,
/// conferindo o plano depois dela (+4 = +0, +8 = 0 e os dois campos das 100 notícias). Erros: a original.
pub(crate) unsafe extern "thiscall" fn compare_clear_inbox(this: usize, manager: usize) {
    let manager = Staff(manager);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_CLEAR_INBOX);
    if manager.is_null() {
        return o(this, manager.addr());
    }
    let h = read_i32(manager.addr()).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10);
    if h < 0 {
        return o(this, manager.addr());
    }
    let caixa = (read_i32(this + news_fields::HUMAN_INBOXES) as u32).wrapping_add((h as u32).wrapping_mul(12)) as usize;
    if (manager.club_job().addr() as i32) != 0 || (manager.national_job().addr() as i32) != 0 {
        o(this, manager.addr());
        let first = read_i32(caixa);
        let news = news_box_regions(this, caixa);
        let ok = read_i32(caixa + 4) == first && read_i32(caixa + 8) == 0
            && news[1..].iter().all(|&(n, _)| read_i32(n) == 0 && read_i32(n + 4) == 1);
        SCOREBOARD.record(ok, || format!("clear_inbox({:x}) plano: caixa {:x?}", manager.addr(), (first, read_i32(caixa + 4), read_i32(caixa + 8))));
        return;
    }
    let reg = news_box_regions(this, caixa);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, manager.addr()), || clear_inbox(this, manager.addr()));
    SCOREBOARD.record(igual, || format!("clear_inbox({:x}) {:x?} x {:x?}", manager.addr(), mo, mr));
}

/// Comparer for `construct_news_item`.
///
/// [pt] Comparador: pela memória da notícia (0xde bytes; a original usa só uma temporária na pilha) e pela devolução.
pub(crate) unsafe extern "thiscall" fn compare_construct_news_item(this: usize, kind: u32, importante: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(IDX_CONSTRUCT_NEWS_ITEM);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0xde)], || o(this, kind, importante), || construct_news_item(this, kind, importante));
    SCOREBOARD.record(igual && r == n, || format!("construct_news_item(tipo {:#x}, {}) {:x?} x {:x?}", kind, importante as u8, mo.0[0].1, mr.0[0].1));
    r
}

/// Comparer for `get_inbox_items`.
///
/// [pt] Comparador: a original aloca o vetor: roda só ela e confere o plano feito antes (a conta e os ids, na ordem).
pub(crate) unsafe extern "thiscall" fn compare_get_inbox_items(this: usize, manager: usize, count: *mut i32, category: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i32, u32) -> usize>(IDX_GET_INBOX_ITEMS);
    if manager == 0 || read_i32(manager).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10) < 0 {
        return o(this, manager, count, category);
    }
    let e = read_usize(this + news_fields::HUMAN_INBOXES) + (read_i32(manager).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10)) as usize * 12;
    let mut ids = Vec::new();
    if read_i32(e) != -1 {
        let mut k = read_i32(e + 4) - 1;
        loop {
            if k < read_i32(e) {
                k = read_i32(e) + 0x63;
            }
            let slot = read_usize(this).wrapping_add((k as usize).wrapping_mul(MEDIA_NEWS_ITEM_SIZE));
            if read_usize(slot + news_item::ID) == 0 {
                break;
            }
            if category as u8 == 0 || (exe::newsitem_get_category(slot) as u32) as u8 == category as u8 {
                ids.push(read_usize(slot + news_item::ID));
            }
            k -= 1;
            if k == read_i32(e + 4) - 1 {
                break;
            }
        }
    }
    let r = o(this, manager, count, category);
    let ok = if ids.is_empty() {
        r == 0
    } else {
        r == 0 || (*count as usize == ids.len() && ids.iter().enumerate().all(|(i, &id)| read_usize(r + i * MEDIA_NEWS_ITEM_SIZE + 0xd6) == id))
    };
    SCOREBOARD.record(ok, || format!("get_inbox_items(categoria {}) {} notícias, esperava {:?}", category as u8, if r == 0 { 0 } else { *count }, ids));
    r
}

/// Comparer for `get_category`.
pub(crate) unsafe extern "thiscall" fn compare_get_category(news: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GET_CATEGORY);
    let t = *(news as *const i32);
    if !(1..=0x1790).contains(&t) {
        return o(news);
    }
    let r = o(news);
    let n = get_category(news);
    SCOREBOARD.record(r == n, || format!("get_category({}) original={:x} rust={:x}", t, r, n));
    r
}

/// Comparer for `get_slot`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot(news: usize, slot: i8) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(IDX_GET_SLOT)(news, slot);
    if !(0..0x32).contains(&slot) {
        return r;
    }
    let n = get_slot(news, slot);
    SCOREBOARD.record(r == n, || format!("get_slot({}) original={:#x} rust={:#x}", slot, r, n));
    r
}

/// Comparer for `set_news_field`.
pub(crate) unsafe extern "thiscall" fn compare_set_news_field(t: usize, k: i8, value: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, u32) -> u32>(IDX_SET_NEWS_FIELD);
    let reg = if (0..0x32).contains(&k) { vec![(t + 5 + 4 * k as usize, 4)] } else { vec![] };
    let (r, v, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(t, k, value), || set_news_field(t, k, value));
    SCOREBOARD.record(igual && r == v, || format!("set_news_field({:#x}, {}, {:#x}) original={} {:x?} rust={} {:x?}", t, k, value, r, mo, v, mr));
    r
}

/// Comparer for `news_operator`.
pub(crate) unsafe extern "thiscall" fn compare_news_operator(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_NEWS_OPERATOR);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0xdf)], || o(this, source), || news_operator(this, source));
    SCOREBOARD.record(igual && r == n, || format!("news_operator {:x?} x {:x?}", mo, mr));
    r
}
