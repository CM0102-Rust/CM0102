use super::*;

/// Contract squad status (high nibble of +0x4f).
///
/// [pt] O tipo do contrato (+0x4f, nibble alto).
pub(crate) unsafe fn contract_squad_status(contract: Contract) -> u8 {
    contract.squad_status() & 0xf0
}

/// A notícia guardada: 0xdf bytes (+0xd6 id, +0xda lida).
pub(crate) const STORED_NEWS_SIZE: usize = 0xdf;

/// Marks read the first unread news item with the id in the human's ring, unless it needs a reply, takes it off the
/// count and, when asked, repeats it for the humans on the same network machine.
///
/// `News::mark_item_read(manager, id, sync)`: in the human's ring of 100 slots (News +8, by id − [0xae2364] + 16), the
/// first slot with the `id` not yet read (+0xda = 0) whose button (`exe::newsitem_get_action_button` on 0xde1f64) does not ask for a reply (2)
/// becomes read and leaves the count (+8 of the entry). No ring: 0. Otherwise (even when not found), with `sync`, it
/// repeats (without sync) for the other humans (+0x5f = 1) on the same network machine (`exe::screenmanager_find_manager_channel`) as the current human;
/// returns 1. No manager, no id, or a manager of the machine: the "Error" box and 0.
/// Original: `News::mark_item_read` (0x787140, news.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x787140 } (thiscall, ret 0xc, news.cpp): `News::mark_item_read(tecnico, id, sincronizar)`: no anel de 100 vagas
/// do humano (News +8, pelo id − [0xae2364] + 16), a primeira vaga com o `id` ainda não lida (+0xda = 0) cujo botão
/// (0x782b70 em 0xde1f64) não pede resposta (2) fica lida e sai da conta (+8 da entrada). Sem anel: 0. Senão (mesmo
/// sem achar), com `sincronizar`, repete (sem sincronizar) para os outros humanos (+0x5f = 1) da mesma máquina da rede
/// (0x7fffa0) que o humano da vez; devolve 1. Sem técnico, sem id ou técnico da máquina: a caixa "Error" e 0.
pub(crate) unsafe extern "thiscall" fn mark_item_read(this: usize, manager: usize, id: u32, sincronizar: u32) -> u32 {
    if manager == 0 {
        show_error_at(0xa693ec, 0x2358);
        return 0;
    }
    if id == 0 {
        show_error_at(0xa693ec, 0x2360);
        return 0;
    }
    let idx = read_i32(manager).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10);
    if idx < 0 {
        show_error_at(0xa693ec, 0x236b);
        return 0;
    }
    let e = read_usize(this + news_fields::HUMAN_INBOXES) + idx as usize * 12;
    if read_i32(e) == -1 {
        return 0;
    }
    let mut v = 0usize;
    for k in 0..0x64usize {
        v = read_usize(this).wrapping_add((read_usize(e).wrapping_add(k)).wrapping_mul(STORED_NEWS_SIZE));
        if read_usize(v + news_item::ID) as u32 == id && read_usize(v + news_item::READ) == 0 {
            let mut tratador = 0u32;
            if exe::newsitem_get_action_button(v, manager, 0xde1f64, 0x7d0, &mut tratador) != 2 {
                covered!("cpo_6 news read");
                write_u32(v + news_item::READ, 1);
                write_i32(e + 8, read_i32(e + 8) - 1);
                break;
            }
        }
    }
    if v == 0 {
        return 0;
    }
    if sincronizar != 0 {
        let of_team = read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize);
        let m = (exe::screenmanager_find_manager_channel(G_SCREEN_MANAGER, of_team) as u32) as u16;
        for j in 0..0x10i32 {
            let p = Staff(read_usize(G_STAFF_TABLE).wrapping_add((read_i32(G_STAFF_COUNT) - j - 1).wrapping_mul((staff::SIZE as i32)) as usize));
            if p.classification() == 1 && p.addr() != manager && (exe::screenmanager_find_manager_channel(G_SCREEN_MANAGER, p) as u32) as u16 == m {
                exe::news_mark_item_read(this, p, id, 0);
            }
        }
    }
    1
}

/// Copies the news item with the id to the output: from the human manager's 100-slot ring or, without a manager, from
/// all slots.
///
/// `News::find_item_by_id(manager, id, out)`: copies (`news_operator`) into `out` the news item with the `id`: with a manager,
/// among the 100 slots of his ring (no ring: 0); without a manager, among all slots (News +4). Found: 1; otherwise 0.
/// No id, no output, or a manager of the machine: the "Error" box and 0.
/// Original: `News::find_item_by_id` (0x786b70, news.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x786b70 } (thiscall, ret 0xc, news.cpp): `News::find_item_by_id(tecnico, id, saida)`: copia (0x786010) em `saida`
/// a notícia com o `id`: com técnico, entre as 100 vagas do anel dele (sem anel: 0); sem técnico, entre todas as vagas
/// (News +4). Achou: 1; senão 0. Sem id, sem saída ou técnico da máquina: a caixa "Error" e 0.
pub(crate) unsafe extern "thiscall" fn find_item_by_id(this: usize, manager: usize, id: u32, out: usize) -> u32 {
    if id == 0 {
        show_error_at(0xa693ec, 0x2252);
        return 0;
    }
    if out == 0 {
        show_error_at(0xa693ec, 0x225a);
        return 0;
    }
    let (inicio, n) = if manager == 0 {
        (0usize, read_i32(this + news_fields::SLOT_COUNT).max(0) as usize)
    } else {
        let idx = read_i32(manager).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10);
        if idx < 0 {
            show_error_at(0xa693ec, 0x2268);
            return 0;
        }
        let start = read_i32(read_usize(this + news_fields::HUMAN_INBOXES) + idx as usize * 12);
        if start == -1 {
            return 0;
        }
        (start as usize, 0x64)
    };
    for k in 0..n {
        let v = read_usize(this).wrapping_add(inicio.wrapping_add(k).wrapping_mul(STORED_NEWS_SIZE));
        if read_usize(v + news_item::ID) as u32 == id {
            exe::newsitem_operator_3d(out, v);
            return 1;
        }
    }
    0
}

/// A notícia guardada na caixa: 0xdf bytes (+4 importante, +0xd6 id, +0xda livre/lida, +0xde alcance).
pub(crate) const NEWS_NEWS_RECORD_SIZE: usize = 0xdf;

/// Index of a human manager in News' inbox table: person id - staff count + 16 (negative = not human).
///
/// [pt] A entrada do técnico humano na tabela das caixas (News +8, 12 bytes: início do anel, a próxima vaga, a conta):
/// índice = id da pessoa − [0xae2364] + 16 (os humanos são as 16 últimas pessoas). Negativo: não é humano.
pub(crate) unsafe fn news_human_manager_index(manager: usize) -> i32 {
    read_i32(manager).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10)
}

/// Delivers the news item by scope: 0 the club manager, 1 the division's managers, 2 the country's, 3 the continent's,
/// 4 all, 5 the competition region's; anything else shows the "Error" box.
///
/// `News::send_by_scope(news, club, competition, nation, continent, scope)`: the scope (char) picks the delivery: 0 the
/// club's manager (`send_to_club_manager`), 1 those of the division (`send_to_division_managers`, by the competition), 2 those of the country (`send_to_nation_managers`,
/// by the nation), 3 those of the continent (`send_to_continent_managers`), 4 all (`send_to_all_managers`), 5 those of the competition's region
/// (`send_to_comp_region`). Anything else: the "Error" box and 0. Returns what the delivery returns.
/// Original: `News::send_by_scope` (0x786090, news.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x786090 } (thiscall, ret 0x18, news.cpp): `News::send_by_scope(noticia, clube, competicao, nacao, continente,
/// alcance)`: o alcance (char) escolhe a entrega: 0 o técnico do clube (0x7861e0), 1 os da divisão (0x7862d0, pela
/// competição), 2 os do país (0x7863f0, pela nação), 3 os do continente (0x786640), 4 todos (0x786780), 5 os da região
/// da competição (0x786860). Fora disso, a caixa "Error" e 0. Devolve o que a entrega devolve.
pub(crate) unsafe extern "thiscall" fn send_by_scope(this: usize, news: usize, club: usize, competition: usize, nation: usize, continent: usize, reach: u32) -> u32 {
    let two: unsafe extern "thiscall" fn(usize, usize, usize) -> u32;
    let target = match reach as u8 as i8 {
        0 => {
            covered!("cpn_1 club reach");
            two = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
            club // the club’s manager
        }
        1 => {
            two = core::mem::transmute(FN_NEWS_SEND_TO_DIVISION_MANAGERS);
            competition
        }
        2 => {
            two = core::mem::transmute(FN_NEWS_SEND_TO_NATION_MANAGERS);
            nation
        }
        3 => {
            two = core::mem::transmute(FN_NEWS_SEND_TO_CONTINENT_MANAGERS);
            continent
        }
        4 => {
            return exe::news_send_to_all_managers(this, news);
        }
        5 => {
            two = core::mem::transmute(FN_NEWS_SEND_TO_COMP_REGION);
            competition
        }
        _ => {
            show_error_at(0xa693ec, 0x1fd3);
            return 0;
        }
    };
    two(this, news, target)
}

/// marks the scope as club (+0xde = 0) and delivers the item to the club manager (+0xcf) when he is human (among the
/// last 16 people); turned off (DESLIGADAS) from 2026-10-29 to 2026-10-31, when the test criterion changed and the list was emptied (the detour, even passing through, changes the stack residue a later
/// original reads: the career left be54a138); the copy stays in the code and does not run
/// Original: `News::send_to_club_manager` (0x7861e0, news.cpp, thiscall, ret 8).
///
/// [pt] { 0x7861e0 } (thiscall, ret 8, news.cpp): `News::send_to_club_manager(noticia, clube)`: marca o alcance da notícia
/// como clube (+0xde = 0) e a entrega (0x785d40) ao técnico do clube (+0xcf) se ele for humano (id entre as 16 últimas
/// pessoas). Sem notícia ou sem clube: a caixa "Error" e 0; sem técnico ou técnico da máquina: 0.
pub(crate) unsafe extern "thiscall" fn send_to_club_manager(this: usize, news: usize, club: usize) -> u32 {
    let club = Club(club);
    if news == 0 {
        show_error_at(0xa693ec, 0x2026);
        return 0;
    }
    if club.is_null() {
        show_error_at(0xa693ec, 0x202e);
        return 0;
    }
    let manager = club.manager().addr();
    write_u8(news + news_item::REACH, 0);
    if manager == 0 {
        return 0;
    }
    if read_i32(manager) < read_i32(G_STAFF_COUNT).wrapping_sub(0x10) { // the machine
        return 0;
    }
    covered!("cpn_1 news to the human manager");
    exe::news_deliver_to_manager(this, news, manager, 0)
}

/// copies the item into the human's 100-slot ring (created on the first, reallocating the array), gives it an id, skips
/// a repeat of the last one and, with the manager on holiday, applies the automatic reply for the club and the national
/// team; turned off (DESLIGADAS) from 2026-10-29 to 2026-10-31, when the test criterion changed and the list was emptied (the detour, even passing through, changes the stack residue a later original
/// reads: the career left be54a138); the copy stays in the code and does not run
/// Original: `News::deliver_to_manager` (0x785d40, news.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x785d40 } (thiscall, ret 0xc, news.cpp): `News::deliver_to_manager(noticia, tecnico, _)`: grava se a notícia é
/// importante para ele (+4, 0x784eb0) e zera +0xda; acha a entrada do humano (News +8); na primeira notícia dele, o anel
/// de 100 vagas no fim do vetor das notícias (News +0, conta de vagas em +4, realloc 0x945667; sem memória, caixa e 0)
/// com as vagas zeradas (+0xd6 = 0, +0xda = 1). Notícia sem id (+0xd6) recebe o próximo (News +0x10). Se a vaga anterior
/// tem o mesmo id, não repete (devolve 1). Senão copia a notícia (0x786010) na próxima vaga (a vaga ainda não lida, +0xda
/// = 0, sai da conta), avança o anel e, com todos de férias ([0xdd6bec]) ou o técnico de férias (0x5fd210), dá a
/// resposta automática (0x784100) pelo clube (+0x39) e pela seleção (+0x24). Soma 1 na conta e devolve 1.
pub(crate) unsafe extern "thiscall" fn deliver_to_manager(this: usize, news: usize, manager: usize, _free: u32) -> u32 {
    let manager = Staff(manager);
    if news == 0 {
        show_error_at(0xa693ec, 0x1f3f);
        return 0;
    }
    if manager.is_null() {
        show_error_at(0xa693ec, 0x1f47);
        return 0;
    }
    write_u8(news + news_item::IMPORTANT, (exe::newsitem_is_important_for(news, manager) as u32) as u8);
    write_usize(news + news_item::READ, 0);
    let idx = news_human_manager_index(manager.addr());
    if idx < 0 {
        show_error_at(0xa693ec, 0x1f58);
        return 0;
    }
    let e = read_usize(this + news_fields::HUMAN_INBOXES) + idx as usize * 12;
    if read_i32(e) == -1 {
        write_usize(e, read_usize(this + news_fields::SLOT_COUNT));
        write_usize(e + 4, read_usize(this + news_fields::SLOT_COUNT));
        let total = read_usize(this + news_fields::SLOT_COUNT).wrapping_add(0x64);
        write_usize(this + news_fields::SLOT_COUNT, total);
        let p = exe::crt_realloc(read_usize(this), total.wrapping_mul(NEWS_NEWS_RECORD_SIZE) as u32);
        write_usize(this, p);
        if p == 0 {
            show_error_at(0xa693ec, 0x1f6d);
            return 0;
        }
        for k in 0..0x64usize {
            let v = read_usize(this) + (read_usize(e) + k).wrapping_mul(NEWS_NEWS_RECORD_SIZE);
            write_usize(v + news_item::ID, 0);
            write_usize(v + news_item::READ, 1);
        }
    }
    if read_usize(news + news_item::ID) == 0 {
        let id = read_usize(this + news_fields::LAST_ID).wrapping_add(1);
        write_usize(this + news_fields::LAST_ID, id);
        write_usize(news + news_item::ID, id);
    }
    let current = read_i32(e + 4);
    let mut previous = current.wrapping_sub(1);
    if previous < read_i32(e) {
        previous = read_i32(e) + 0x63;
    }
    let slot = |k: i32| read_usize(this).wrapping_add((k as usize).wrapping_mul(NEWS_NEWS_RECORD_SIZE));
    if read_usize(slot(previous) + 0xd6) == read_usize(news + news_item::ID) {
        return 1; // the same news again
    }
    if read_usize(slot(current) + 0xda) == 0 {
        write_usize(e + 8, read_usize(e + 8).wrapping_sub(1));
    }
    exe::newsitem_operator_3d(slot(current), news);
    let current = read_i32(e + 4);
    let start = read_i32(e);
    write_usize(e + 4, current.wrapping_add(1) as usize);
    if current.wrapping_add(1) == start.wrapping_add(0x64) {
        write_usize(e + 4, start as usize);
    }
    if read_i16(G_NEWS_SCREEN_AUTO_OPEN_BLOCKED_FLAG) != 0 || exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER), manager) != 0 {
        covered!("cpn_1 automatic reply");
        exe::newsitem_auto_respond(slot(current), manager, manager.club_job()); // the club
        let national_team = manager.national_job();
        if !national_team.is_null() {
            exe::newsitem_auto_respond(slot(current), manager, national_team);
        }
    }
    write_usize(e + 8, read_usize(e + 8).wrapping_add(1));
    1
}

/// A notícia guardada na caixa: 0xdf bytes (+4 importante, +5 os 50 campos de 4 bytes, +0xcc fim do texto, +0xd6 id,
/// +0xda lida).
pub(crate) const NEWS_NEWS_ITEM_SIZE: usize = 0xdf;

/// Human manager's 12-byte inbox entry (News +8: ring start, next slot, count), or None with the error box when not one
/// of the last 16 people.
///
/// [pt] A entrada do humano na tabela das caixas (News +8, 12 bytes: início do anel, próxima vaga, conta), ou None (com a
/// caixa "Error" da `linha`) se a pessoa não está entre as 16 últimas.
pub(crate) unsafe fn human_inbox_entry(this: usize, manager: usize, row: u32) -> Option<usize> {
    let idx = read_i32(manager).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10);
    if idx < 0 {
        show_error_at(0xa693ec, row);
        return None;
    }
    Some(read_usize(this + news_fields::HUMAN_INBOXES) + idx as usize * 12)
}

/// Wraps a ring slot k below inicio to inicio + 99 (100-slot inbox ring).
///
/// [pt] A vaga anterior a `k` no anel que começa em `inicio` (100 vagas).
pub(crate) fn ring_prev_wrap(k: i32, start: i32) -> i32 {
    if k < start { start + 0x63 } else { k }
}

/// Applies the automatic reply for the club to the human's inbox, from the newest item to the oldest, up to the empty
/// slot.
///
/// `News::auto_respond_all(manager, club)`: from the newest to the oldest news item of the human's ring, up to the
/// first empty slot (+0xd6 = 0), gives the automatic reply (`exe::newsitem_auto_respond`) for the `club`. No manager or a manager of the
/// machine: the "Error" box and 0; no ring or at the end: 1. TapeCall when going on holiday, when sacked and when
/// hired.
/// Original: `News::auto_respond_all` (0x7874b0, news.cpp, thiscall, ret 8).
///
/// [pt] { 0x7874b0 } (thiscall, ret 8, news.cpp): `News::auto_respond_all(tecnico, clube)`: da notícia mais nova para a mais
/// antiga do anel do humano, até a primeira vaga vazia (+0xd6 = 0), dá a resposta automática (0x784100) pelo `clube`.
/// Sem técnico ou técnico da máquina: a caixa "Error" e 0; sem anel ou no fim: 1. TapeCall ao sair de férias, ao ser
/// demitido e ao ser contratado.
pub(crate) unsafe extern "thiscall" fn auto_respond_all(this: usize, manager: usize, club: usize) -> u32 {
    if manager == 0 {
        show_error_at(0xa693ec, 0x2460);
        return 0;
    }
    let e = match human_inbox_entry(this, manager, 0x246b) {
        Some(e) => e,
        None => return 0,
    };
    if read_i32(e) == -1 {
        return 1;
    }
    let mut k = read_i32(e + 4) - 1;
    loop {
        k = ring_prev_wrap(k, read_i32(e));
        let slot = read_usize(this).wrapping_add((k as usize).wrapping_mul(NEWS_NEWS_ITEM_SIZE));
        if read_usize(slot + news_item::ID) == 0 {
            return 1;
        }
        covered!("cpn_2 respond all");
        exe::newsitem_auto_respond(slot, manager, club); // the automatic reply
        k -= 1;
        if k == read_i32(e + 4) - 1 {
            return 1;
        }
    }
}

/// Without "urgent only", tells whether there is an unread item; with it, whether an item needs a reply or is important
/// and unread.
///
/// `News::has_urgent_items(manager, urgent_only)`: without `urgent_only`, whether there is an unread news item
/// (`count_unread` > 0). With it, from the newest to the oldest of the human's ring, up to the first empty slot: 1 if one
/// needs a reply (the text `exe::newsitem_get_action_button` on 0xde1f64 returns 2) or is important (+4 = 1) and unread (+0xda = 0). A manager
/// of the machine: the "Error" box and 0.
/// Original: `News::has_urgent_items` (0x787350, news.cpp, thiscall, ret 8).
///
/// [pt] { 0x787350 } (thiscall, ret 8, news.cpp): `News::has_urgent_items(tecnico, so_urgentes)`: sem `so_urgentes`, se há
/// notícia não lida (0x786fe0 > 0). Com ele, da mais nova para a mais antiga do anel do humano, até a primeira vaga vazia:
/// 1 se uma pede resposta (o texto 0x782b70 em 0xde1f64 devolve 2) ou é importante (+4 = 1) e não lida (+0xda = 0).
/// Técnico da máquina: a caixa "Error" e 0.
pub(crate) unsafe extern "thiscall" fn has_urgent_items(this: usize, manager: usize, so_urgentes: u32) -> u32 {
    if so_urgentes == 0 {
        return (exe::news_count_unread(this, manager, 0) > 0) as u32;
    }
    let e = match human_inbox_entry(this, manager, 0x241e) {
        Some(e) => e,
        None => return 0,
    };
    if read_i32(e) == -1 {
        return 0;
    }
    let mut k = read_i32(e + 4) - 1;
    loop {
        k = ring_prev_wrap(k, read_i32(e));
        let slot = read_usize(this).wrapping_add((k as usize).wrapping_mul(NEWS_NEWS_ITEM_SIZE));
        if read_usize(slot + news_item::ID) == 0 {
            return 0;
        }
        let mut out = 0u32;
        if (exe::newsitem_get_action_button(slot, manager, 0xde1f64, 0x7d0, &mut out) as u32) as u8 == 2 {
            return 1; // pede resposta
        }
        let slot = read_usize(this).wrapping_add((k as usize).wrapping_mul(NEWS_NEWS_ITEM_SIZE));
        if read_u8(slot + news_item::IMPORTANT) == 1 && read_usize(slot + news_item::READ) == 0 {
            return 1; // important and unread
        }
        k -= 1;
        if k == read_i32(e + 4) - 1 {
            return 0;
        }
    }
}

/// Copies the text into the news item's slots from the given slot and returns the first free slot after it (at most
/// 50).
///
/// `slot.set_slot_string(text, next)`: copies the `text` into the news item's slots from `slot` (strncpy
/// `exe::at_9466d0` at +5 + 4·slot, up to 200 − 4·slot bytes), terminates the area (+0xcc = 0) and, with `next`, writes the
/// first free slot after the text (the length with the terminating zero rounded up to 4, in slots, plus `slot`; at most
/// 50). Slot outside 0..49 or no text: the "Error" box and 0; otherwise 1.
/// Original: `NewsItem::set_slot_string` (0x7858c0, news.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x7858c0 } (thiscall, ret 0xc, news.cpp): `campo.set_slot_string(texto, proximo)`: copia o `texto` nos
/// campos da notícia a partir do `campo` (strncpy 0x9466d0 em +5 + 4·campo, até 200 − 4·campo bytes), termina a área
/// (+0xcc = 0) e, com `proximo`, grava o primeiro campo livre depois do texto (o tamanho com o zero arredondado para
/// 4, em campos, mais o `campo`; no máximo 50). Campo fora de 0..49 ou sem texto: a caixa "Error" e 0; senão 1.
pub(crate) unsafe extern "thiscall" fn set_slot_string(this: usize, field: u32, text: *const u8, next: *mut u8) -> u32 {
    let field = field as u8 as i8;
    if !(0..0x32).contains(&field) {
        show_error_at(0xa693ec, 0x1e8d);
        return 0;
    }
    if text.is_null() {
        show_error_at(0xa693ec, 0x1e95);
        return 0;
    }
    let d = 4 * field as i32;
    exe::at_9466d0(this + news_item::DATA + d as usize, text, (0xc8 - d) as u32);
    write_u8(this + news_item::TEXT_END, 0);
    if !next.is_null() {
        let mut n = 0i32;
        while *text.add(n as usize) != 0 {
            n += 1;
        }
        let quartos = ((n + 1 + 3) & !3) / 4; // the size including the zero, in 4-byte fields
        let mut p = (quartos as u8).wrapping_add(field as u8);
        covered!("cpn_2 next text field");
        if p as i8 > 0x32 {
            p = 0x32;
        }
        *next = p; // the next free field
    }
    1
}

/// Sets the continent scope (+0xde = 3) and delivers the news to every active human manager whose club or nation is in
/// the continent.
///
/// `News::send_to_continent_managers(news, continent)`: continent scope (+0xde = 3) and delivery (`deliver_to_manager` with 0) to
/// each active human manager whose club (+0x39) has its nation (+0x53) in the `continent` (+0x71) or whose nation
/// (+0x1a) is in it. Returns 1, or 0 on the first refusal. Null news or continent: as in the original ("Error" box).
/// Original: `News::send_to_continent_managers` (0x786640, news.cpp, thiscall, ret 8).
///
/// [pt] { 0x786640 } (thiscall, ret 8, news.cpp): `News::send_to_continent_managers(noticia, continente)`: alcance do
/// continente (+0xde = 3) e a entrega (0x785d40 com 0) a cada técnico humano ativo cujo clube (+0x39) tem a nação
/// (+0x53) no `continente` (+0x71) ou cuja nação (+0x1a) está nele. Devolve 1, ou 0 na primeira recusada. Notícia ou
/// continente nulos: a original (caixa "Error").
pub(crate) unsafe extern "thiscall" fn send_to_continent_managers(this: usize, news: usize, continent: usize) -> u32 {
    if news == 0 || continent == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SEND_TO_CONTINENT_MANAGERS)(this, news, continent); // message box: original only
    }
    deliver_to_human_managers(this, news, 3, |p| {
        let c = Club(read_usize(p + staff::CLUB_JOB));
        if !c.is_null() {
            let n = c.nation();
            if !n.is_null() && n.continent() == continent { // the continent of the club’s nation
                covered!("cpk_5 continent by club");
                return true;
            }
        }
        let s = Nation(read_usize(p + staff::NATION));
        !s.is_null() && s.continent() == continent // the continent of the person’s nation
    }) as u32
}

/// Delivers the news to the managers of the competition's nation (`send_to_nation_club_managers`), else of its continent (`send_to_continent_managers`), else to
/// all (`send_to_all_managers`).
///
/// `News::send_to_comp_region(news, comp)`: with the competition's nation (+0x5d), to the nation's managers (`send_to_nation_club_managers`);
/// otherwise, with the continent (+0x59), to the continent's (`send_to_continent_managers`); otherwise to all (`send_to_all_managers`). Returns what the
/// delivery returns. Null news or competition: as in the original ("Error" box).
/// Original: `News::send_to_comp_region` (0x786860, news.cpp, thiscall, ret 8).
///
/// [pt] { 0x786860 } (thiscall, ret 8, news.cpp): `News::send_to_comp_region(noticia, comp)`: com a nação da competição
/// (+0x5d), para os técnicos da nação (0x786520); senão, com o continente (+0x59), para os do continente (0x786640);
/// senão para todos (0x786780). Devolve o que a entrega devolve. Notícia ou competição nulas: a original (caixa
/// "Error").
pub(crate) unsafe extern "thiscall" fn send_to_comp_region(this: usize, news: usize, comp: usize) -> u32 {
    let comp = Competition(comp);
    if news == 0 || comp.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SEND_TO_COMP_REGION)(this, news, comp.addr()); // message box: original only
    }
    let nation = comp.nation();
    if !nation.is_null() {
        covered!("cpk_5 news of the competition nation");
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_NATION_CLUB_MANAGERS);
        return f(this, news, nation.addr());
    }
    let continent = comp.continent();
    if continent != 0 {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CONTINENT_MANAGERS);
        return f(this, news, continent);
    }
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_ALL_MANAGERS);
    f(this, news)
}

/// Manager (+0xcf) of the club whose id is in the news data at offset o.
///
/// [pt] O técnico (+0xcf) do clube cujo id está no dado da notícia em `o` ([0xae23b4], 0x245 bytes por clube).
pub(crate) unsafe fn manager_of_news_club(news: usize, o: usize) -> usize {
    read_usize(read_usize(G_CLUB_TABLE).wrapping_add(read_usize(news + o).wrapping_mul(club::SIZE)) + 0xcf)
}

/// Tells whether the news item is important for the person, by category and type (bids on the person's club, the
/// humans...).
///
/// `person.is_important_for()`: is the news important for the person (the byte +4 that the delivery `deliver_to_manager`
/// writes)? By the category (`get_category`) and the type (+0): category 2, types 7 to 0xb, 0x27 and 0x28; category 4, 0xbb8,
/// 0xbb9 and 0xbc4; category 5, a fixed list, 0xfa1 if the person is the manager of the club in data 5 (+0x19) and
/// 0xfb6 if he is that of data 4 (+0x15) or of data 5; category 6, 0x1770 if data 0 (+5) is one of the 16 human
/// managers; category 1, 0x1388; the rest, 0. The original calls `get_category` up to six times (it only reads): the port
/// calls it once. Null person or type outside 1..0x1790 (`get_category`'s box): as in the original.
/// Original: `NewsItem::is_important_for` (0x784eb0, news.cpp, thiscall, ret 4).
///
/// [pt] { 0x784eb0 } (thiscall, ret 4, news.cpp): `pessoa.is_important_for()`: a notícia é importante para a
/// pessoa (o byte +4 que a entrega 0x785d40 grava)? Pela categoria (0x785670) e o tipo (+0): categoria 2, os tipos 7 a
/// 0xb, 0x27 e 0x28; categoria 4, 0xbb8, 0xbb9 e 0xbc4; categoria 5, uma lista fixa, 0xfa1 se a pessoa é o técnico do
/// clube do dado 5 (+0x19) e 0xfb6 se é o do dado 4 (+0x15) ou do 5; categoria 6, 0x1770 se o dado 0 (+5) é um dos
/// 16 técnicos humanos; categoria 1, 0x1388; o resto, 0. A original chama a 0x785670 até seis vezes (só lê): a cópia,
/// uma. Pessoa nula ou tipo fora de 1..0x1790 (caixa da 0x785670): a original.
pub(crate) unsafe extern "thiscall" fn is_important_for(news: usize, person: usize) -> u8 {
    let t = read_usize(news) as i32;
    if person == 0 || !(1..=0x1790).contains(&t) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_IS_IMPORTANT_FOR)(news, person); // message box: original only
    }
    let r = match (exe::newsitem_get_category(news) as u32) as u8 {
        2 => matches!(t, 7..=0xb | 0x27 | 0x28),
        4 => matches!(t, 0xbb8 | 0xbb9 | 0xbc4),
        5 => match t {
            0xfa0 | 0xfa4 | 0xfad | 0xfae | 0xfaf | 0xfb0 | 0xfb2 | 0xfb4 | 0xfb7 | 0xfb8 | 0xfbb | 0xfbf | 0xfc0 => true,
            0xfa1 => {
                covered!("cpk_5 important offer by club");
                manager_of_news_club(news, 0x19) == person // the manager of the offering club
            }
            0xfb6 => manager_of_news_club(news, 0x15) == person || manager_of_news_club(news, 0x19) == person,
            _ => false,
        },
        6 => t == 0x1770 && read_usize(news + news_item::DATA) as i32 >= (read_usize(G_STAFF_COUNT) as i32).wrapping_sub(0x10),
        1 => t == 0x1388,
        _ => false, // 3 and 7: none
    };
    r as u8
}

/// Copies the text stored from the slot (+5 + 4 × slot) into the buffer (strncpy) and clears the last byte; returns 1.
///
/// `slot.get_slot_string(dest, n)`: copies the text stored from slot `slot` (+5 + 4 × slot) into `dest` like
/// strncpy (`exe::at_9466d0`: up to the zero, padding with zeros up to `n`) and zeroes the last byte (`dest[n − 1]`). Returns
/// 1. Slot outside 0..49, null destination or `n` < 1: as in the original ("Error" box and 0).
/// Original: `NewsItem::get_slot_string` (0x785a20, news.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x785a20 } (thiscall, ret 0xc, news.cpp): `dado.get_slot_string(destino, n)`: copia o texto guardado a
/// partir do dado `dado` (+5 + 4 × dado) para `destino` como o strncpy (0x9466d0: até o zero, completando com zeros
/// até `n`) e zera o último byte (`destino[n − 1]`). Devolve 1. Dado fora de 0..49, destino nulo ou `n` < 1: a
/// original (caixa "Error" e 0).
pub(crate) unsafe extern "thiscall" fn get_slot_string(news: usize, given: u32, target: *mut u8, n: i32) -> u32 {
    let k = given as u8 as i8;
    if !(0..0x32).contains(&k) || target.is_null() || n < 1 {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut u8, i32) -> u32>(IDX_GET_SLOT_STRING)(news, given, target, n); // message box: original only
    }
    covered!("cpk_6 data text");
    let source = (news + 5 + 4 * k as usize) as *const u8; // o dado
    let mut end = false;
    for i in 0..n as usize {
        let c = if end { 0 } else { *source.add(i) };
        end |= c == 0;
        *target.add(i) = c;
    }
    *target.add(n as usize - 1) = 0;
    1
}

/// The given club, or the current human manager's club when null.
///
/// [pt] O clube que a 0x7f9190 usa: o `clube`, ou o do técnico humano da vez ([0xb74c26 + 0x300 × [0xb77c7a]] + 0x39).
pub(crate) unsafe fn club_or_current_human(club: usize) -> usize {
    if club != 0 {
        return club;
    }
    read_usize(read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize) + 0x39)
}

/// Human manager i (0..15, last 16 people of G_STAFF_TABLE) if active (+0x5f = 1).
///
/// [pt] O técnico humano `i` (0 a 15: os últimos 16 de [0xae23bc], 0x6e bytes), se ativo (+0x5f = 1).
pub(crate) unsafe fn active_human_manager(i: usize) -> Option<usize> {
    let p = Staff(read_usize(G_STAFF_TABLE).wrapping_add(read_usize(G_STAFF_COUNT).wrapping_sub(0x10).wrapping_add(i).wrapping_mul(staff::SIZE)));
    (p.classification() == 1).then_some(p.addr())
}

/// Delivers the news to every active human manager that passa accepts, with reach alcance in +0xde; false at the first
/// refusal.
///
/// [pt] Entrega a notícia a cada técnico humano ativo que `passa` (0x785d40 com 0), com o alcance `alcance` em +0xde.
/// Devolve se todas as entregas foram aceitas (a primeira recusada encerra).
pub(crate) unsafe fn deliver_to_human_managers(this: usize, news: usize, reach: u8, passes: impl Fn(usize) -> bool) -> bool {
    write_u8(news + news_item::REACH, reach);
    for i in 0..0x10 {
        if let Some(p) = active_human_manager(i) {
            if passes(p) && exe::news_deliver_to_manager(this, news, p, 0) == 0 {
                return false; // a entrega recusada encerra
            }
        }
    }
    true
}

/// all scope (+0xde = 4) and delivery (`deliver_to_manager`) to every active human manager; 1, or 0 on the first refusal
/// Original: `News::send_to_all_managers` (0x786780, news.cpp, thiscall, ret 4).
///
/// [pt] { 0x786780 } (thiscall, ret 4, news.cpp): `News::send_to_all_managers(noticia)`: alcance de todos (+0xde = 4) e a
/// entrega (0x785d40) a cada técnico humano ativo. Devolve 1, ou 0 na primeira recusada. Notícia nula: a original
/// (caixa "Error").
pub(crate) unsafe extern "thiscall" fn send_to_all_managers(this: usize, news: usize) -> u32 {
    if news == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SEND_TO_ALL_MANAGERS)(this, news); // message box: original only
    }
    deliver_to_human_managers(this, news, 4, |_| true) as u32
}

/// Sets the division scope (+0xde = 1) and delivers the news to every active human manager with a club in the division
/// (+0x57); returns 1, or 0 on the first refusal.
///
/// `News::send_to_division_managers(news, division)`: division scope (+0xde = 1) and delivery to each active human
/// manager with a club (+0x39) in the `division` (+0x57). Returns 1, or 0 on the first refusal. Null news or division:
/// as in the original ("Error" box).
/// Original: `News::send_to_division_managers` (0x7862d0, news.cpp, thiscall, ret 8).
///
/// [pt] { 0x7862d0 } (thiscall, ret 8, news.cpp): `News::send_to_division_managers(noticia, divisao)`: alcance da divisão
/// (+0xde = 1) e a entrega a cada técnico humano ativo com clube (+0x39) na `divisao` (+0x57). Devolve 1, ou 0 na
/// primeira recusada. Notícia ou divisão nulas: a original (caixa "Error").
pub(crate) unsafe extern "thiscall" fn send_to_division_managers(this: usize, news: usize, division: usize) -> u32 {
    if news == 0 || division == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SEND_TO_DIVISION_MANAGERS)(this, news, division); // message box: original only
    }
    deliver_to_human_managers(this, news, 1, |p| {
        let c = Club(read_usize(p + staff::CLUB_JOB));
        !c.is_null() && c.division().addr() == division // the club’s division
    }) as u32
}

/// Sets the nation scope (+0xde = 2) and delivers the news to every active human manager whose club or national team
/// belongs to the nation (+0x53); returns 0.
///
/// `News::send_to_nation_managers(news, nation)`: nation scope (+0xde = 2) and delivery to each active human manager
/// whose club (+0x39) or national team (+0x24) belongs to the `nation` (+0x53). Returns 0. Null news or nation: as in
/// the original ("Error" box).
/// Original: `News::send_to_nation_managers` (0x7863f0, news.cpp, thiscall, ret 8).
///
/// [pt] { 0x7863f0 } (thiscall, ret 8, news.cpp): `News::send_to_nation_managers(noticia, nacao)`: alcance da nação (+0xde
/// = 2) e a entrega a cada técnico humano ativo cujo clube (+0x39) ou seleção (+0x24) é da `nacao` (+0x53). Devolve 0.
/// Notícia ou nação nulas: a original (caixa "Error").
pub(crate) unsafe extern "thiscall" fn send_to_nation_managers(this: usize, news: usize, nation: usize) -> u32 {
    if news == 0 || nation == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SEND_TO_NATION_MANAGERS)(this, news, nation); // message box: original only
    }
    deliver_to_human_managers(this, news, 2, |p| {
        let (c, s) = (read_usize(p + staff::CLUB_JOB), read_usize(p + staff::NATIONAL_JOB));
        (c != 0 && read_usize(c + club::NATION) == nation) || (s != 0 && read_usize(s + club::NATION) == nation) // the club or the national team
    });
    0
}

/// Sets the nation scope (+0xde = 2) and delivers the news (`deliver_to_manager`) to every active human manager with a club of the
/// nation; the first refusal stops.
///
/// `News::send_to_nation_club_managers(news, nation)`: the news gets the nation scope (+0xde = 2) and goes (`deliver_to_manager`
/// with 0) to each of the 16 human managers (the last 16 of [0xae23bc], 0x6e bytes) who are active (+0x5f = 1) with a
/// club (+0x39) of the `nation` (+0x53); the first refused delivery (0) stops it. Returns 0. Null news or nation: as in
/// the original ("Error" box).
/// Original: `News::send_to_nation_club_managers` (0x786520, news.cpp, thiscall, ret 8).
///
/// [pt] { 0x786520 } (thiscall, ret 8, news.cpp): `News::send_to_nation_club_managers(noticia, nacao)`: a notícia fica com
/// o alcance da nação (+0xde = 2) e vai (0x785d40 com 0) para cada um dos 16 técnicos humanos (os últimos 16 de
/// [0xae23bc], 0x6e bytes) ativos (+0x5f = 1) com clube (+0x39) da `nacao` (+0x53); a primeira entrega recusada (0)
/// encerra. Devolve 0. Notícia ou nação nulas: a original (caixa "Error").
pub(crate) unsafe extern "thiscall" fn send_to_nation_club_managers(this: usize, news: usize, nation: usize) -> u32 {
    if news == 0 || nation == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SEND_TO_NATION_CLUB_MANAGERS)(this, news, nation); // message box: original only
    }
    write_u8(news + news_item::REACH, 2); // the nation’s reach
    for i in 0..0x10usize {
        let p = Staff(read_usize(G_STAFF_TABLE).wrapping_add(read_usize(G_STAFF_COUNT).wrapping_sub(0x10).wrapping_add(i).wrapping_mul(staff::SIZE)));
        if p.classification() != 1 {
            continue;
        }
        let club = p.club_job();
        if club.is_null() || club.nation().addr() != nation {
            continue;
        }
        if exe::news_deliver_to_manager(this, news, p, 0) == 0 {
            break; // a entrega recusada encerra
        }
    }
    0
}

/// Returns the scout of a search if it still checks out (type 4, index ≥ 0, names/date match), else None (the original
/// shows a box).
///
/// [pt] O olheiro da busca confere (o tipo +0xa é 4, o índice +0x180 > −1 e a pessoa ainda confere com os nomes e a data
/// guardados pela 0x53e990)? Devolve a pessoa, ou nenhuma (a original mostra a caixa).
pub(crate) unsafe fn check_search_scout(this: usize) -> Option<usize> {
    let i = read_usize(this + player_search_fields::SCOUT_ID) as i32;
    if read_u8(this + player_search_fields::LIST_TYPE) != 4 || i <= -1 {
        return None;
    }
    let p = read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE));
    if exe::database_staff_matches_names(p, read_usize(this + player_search_fields::SCOUT_NAME_REF) as u32, read_usize(this + 0x178) as u32, read_usize(this + 0x17c) as u32, this + 0x184) == 0 || p == 0 {
        return None;
    }
    Some(p)
}

/// Human manager's 12-byte inbox entry: [news+8] + 12 × (id - staff count + 16); None when negative.
///
/// [pt] A entrada (12 bytes) da caixa de notícias do técnico humano: [noticias+8] + 12 × (id − [0xae2364] +
/// 0x10); None quando o índice fica negativo (a original abre a caixa de erro).
pub(crate) unsafe fn manager_inbox_entry(news: usize, manager: usize) -> Option<usize> {
    let i = read_i32(manager).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10);
    if i < 0 {
        return None;
    }
    Some(read_usize(news + 8).wrapping_add((i as usize).wrapping_mul(12)))
}

/// News item k of the inbox starting at inicio: [news] + 0xdf × (inicio + k).
///
/// [pt] A notícia `k` da caixa que começa em `inicio`: [noticias] + 0xdf × (inicio + k).
pub(crate) unsafe fn inbox_news_item(news: usize, start: i32, k: i32) -> usize {
    read_usize(news).wrapping_add((start.wrapping_add(k) as usize).wrapping_mul(0xdf))
}

/// Counts the unread news in a human manager's inbox, in total or for one category.
///
/// Without a category (0): the entry's total (+8, negative becomes 0). With a category: of the 100 news items in the
/// inbox (start at entry +0; −1 = empty, 0), the occupied ones (+0xd6) of the category (`get_category`) not yet read (+0xda =
/// 0). Null manager or out of range: as in the original (error box).
/// Original: `News::count_unread` (0x786fe0, news.cpp, thiscall).
///
/// [pt] { 0x786fe0 } (thiscall, ret 8, news.cpp): as notícias não lidas do técnico humano. Sem categoria (0):
/// o total da entrada (+8, negativo vira 0). Com categoria: das 100 notícias da caixa (início na entrada
/// +0; −1 = vazia, 0), as ocupadas (+0xd6) da categoria (0x785670) ainda não lidas (+0xda = 0). Técnico
/// nulo ou fora da faixa: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn count_unread(news: usize, manager: usize, category: u32) -> i32 {
    let entry = match if manager == 0 { None } else { manager_inbox_entry(news, manager) } {
        Some(e) => e,
        None => return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(IDX_COUNT_UNREAD)(news, manager, category),
    };
    let total = (entry + 8) as *mut i32;
    if *total < 0 {
        *total = 0;
    }
    if category as u8 == 0 {
        return *total; // message box total
    }
    let start = read_i32(entry);
    if start == -1 {
        return 0;
    }
    let mut count = 0;
    for k in 0..100 {
        let n = inbox_news_item(news, start, k);
        if read_u32(n + news_item::ID) != 0
            && (exe::newsitem_get_category(n) as u32) as u8 == category as u8
            && read_u32(n + news_item::READ) == 0
        {
            count += 1;
        }
    }
    count
}

/// Where `set_item_slot` writes: Err on its error-box cases, Ok(None) for an empty inbox, Ok(Some(item)) when found.
///
/// [pt] Onde a 0x786d30 grava: Err nos casos da caixa de erro (técnico nulo, id 0, vaga fora de 0..49, cópia
/// de outro id, índice negativo, id fora das 100 da caixa); Ok(None) com a caixa vazia (início −1);
/// Ok(Some(notícia)) achada.
pub(crate) unsafe fn set_slot_target(news: usize, manager: usize, id: u32, slot: i32, copy: usize) -> Result<Option<usize>, ()> {
    if manager == 0 || id == 0 || !(0..0x32).contains(&slot) {
        return Err(());
    }
    if copy != 0 && read_u32(copy + news_item::ID) != id {
        return Err(());
    }
    let entry = manager_inbox_entry(news, manager).ok_or(())?;
    let start = read_i32(entry);
    if start == -1 {
        return Ok(None);
    }
    (0..100)
        .map(|k| inbox_news_item(news, start, k))
        .find(|&n| read_u32(n + news_item::ID) == id)
        .map(Some)
        .ok_or(())
}

/// Writes a value into a slot of a news item in the human manager's inbox and into its local copy.
///
/// Writes `value` into slot `slot` (dword +5 + 4 × slot) of news item `id` (+0xd6) in the human manager's inbox, first
/// into the `copy` (if any) and then into the inbox one; 1 if found. Empty inbox: only the copy, 0. Error cases: as in
/// the original.
/// Original: `News::set_item_slot` (0x786d30, news.cpp, thiscall).
///
/// [pt] { 0x786d30 } (thiscall, ret 0x14, news.cpp): grava `valor` na vaga `vaga` (dword +5 + 4 × vaga) da
/// notícia `id` (+0xd6) da caixa do técnico humano, primeiro na `copia` (se houver) e depois na da caixa;
/// 1 se achou. Caixa vazia: só a cópia, 0. Casos de erro: a original.
pub(crate) unsafe extern "thiscall" fn set_item_slot(news: usize, manager: usize, id: u32, slot: i32, value: u32, copy: usize) -> u32 {
    let target = match set_slot_target(news, manager, id, slot, copy) {
        Ok(a) => a,
        Err(()) => return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, usize) -> u32>(IDX_SET_ITEM_SLOT)(news, manager, id, slot, value, copy),
    };
    if copy != 0 {
        write_u32(copy + 5 + 4 * slot as usize, value);
    }
    match target {
        Some(n) => {
            write_u32(n + 5 + 4 * slot as usize, value); // mark in the box
            1
        }
        None => 0,
    }
}

/// whether a human manager runs the club, ignoring holidays
/// Original: `news_club_has_human_manager` (0x7875e0, news.cpp, stdcall).
///
/// [pt] { 0x7875e0 } (stdcall, ret 4, news.cpp): 1 se um humano dirige o `clube` (0x600a50 com férias
/// ignoradas); antes de devolver, consulta as férias (0x5fd210 em [0xb63c98]) do técnico do clube
/// (0x53ca90), sem usar a resposta. Clube nulo: 0.
pub(crate) unsafe extern "stdcall" fn news_club_has_human_manager(club: usize) -> u32 {
    if club == 0 {
        return 0;
    }
    let r = exe::human_manager_is_human_club(club, 1, 1, 0, 0);
    if r == 0 {
        return r;
    }
    let t = exe::database_get_club_manager(club);
    exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER), t);
    1 // humano
}

/// Gives the default replies for the manager's club and national team (`auto_respond_all`) and empties his inbox (the 100 news
/// items with +0xd6 = 0 and +0xda = 1).
///
/// `News::clear_inbox(manager)`, from 0x6b28b0 and from loading the game (`load_game_data`): the default replies to the news of
/// the manager's club (+0x39) and national team (+0x24) (`auto_respond_all`); then the human's inbox (12-byte record at [this+8],
/// by id − [0xae2364] + 16): +4 = +0 and +8 = 0; with the first news item (+0, −1 none), the 100 news items of 0xdf
/// bytes from it ([this]) with +0xd6 = 0 and +0xda = 1. No manager or a negative index: the error box (lines 0x24c1 and
/// 0x24cc).
/// Original: `News::clear_inbox` (0x787630, news.cpp, thiscall, ret 4).
///
/// [pt] { 0x787630 } (thiscall, ret 4, news.cpp): `News::clear_inbox(tecnico)`, pela 0x6b28b0 e pela carga do jogo
/// (0x82a9c0): as respostas padrão das notícias do clube (+0x39) e da seleção (+0x24) do técnico (0x7874b0); depois a
/// caixa do humano (registro de 12 bytes em [this+8], pelo id − [0xae2364] + 16): +4 = +0 e +8 = 0; com a primeira
/// notícia (+0, −1 nenhuma), as 100 notícias de 0xdf bytes a partir dela ([this]) com +0xd6 = 0 e +0xda = 1. Sem técnico
/// ou com índice negativo: a caixa de erro (linhas 0x24c1 e 0x24cc).
pub(crate) unsafe extern "thiscall" fn clear_inbox(this: usize, manager: usize) {
    if manager == 0 {
        show_error_at(0xa693ec, 0x24c1);
        return;
    }
    let h = read_i32(manager).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10);
    if h < 0 {
        show_error_at(0xa693ec, 0x24cc);
        return;
    }
    let caixa = (read_i32(this + news_fields::HUMAN_INBOXES) as u32).wrapping_add((h as u32).wrapping_mul(12)) as usize;
    for o in [0x39usize, 0x24] {
        let c = read_i32(manager + o) as usize;
        if c != 0 {
            exe::news_auto_respond_all(this, manager, c);
        }
    }
    let first = read_i32(caixa);
    write_i32(caixa + 8, 0);
    write_i32(caixa + 4, first);
    if first != -1 {
        covered!("clear_inbox");
        for k in 0..0x64u32 {
            let n = (read_i32(this) as u32).wrapping_add(k.wrapping_add(read_i32(caixa) as u32).wrapping_mul(0xdf)) as usize;
            write_i32(n + news_item::ID, 0);
            write_i32(n + news_item::READ, 1);
        }
    }
}

/// builds the news item: type (+0), important byte (+4), the 50 data slots cleared, today's date (+0xcd), byte
/// [0xae2c88] at +0xd5, id and +0xda cleared; turned off (DESLIGADAS) from 2026-10-29 to 2026-10-31, when the test criterion changed and the list was emptied: with the detour the career in Rust mode
/// left the reference be54a138 (bisection of batch 251's 39 routines; with `construct_news_item`, the news constructor, the game
/// later crashed in `player_stats_reset_player` on a broken index); the cause (stack residue or a difference in the copy) is not isolated
/// yet; the copy stays in the code and does not run
/// Original: `NewsItem::NewsItem` (0x77bbe0, news.cpp, thiscall, ret 8).
///
/// [pt] { 0x77bbe0 } (thiscall, ret 8, news.cpp, quadro SEH só da data, destrutor vazio 0x5a53c0): `NewsItem::NewsItem(tipo,
/// importante)`: inicia a data (+0xcd, 0x54a110), grava o tipo (+0), o byte importante (+4) e zera os 50 dados (+5);
/// a data de hoje ([0xae2c90], pela atribuição 0x417c10 com retorno numa temporária), o byte [0xae2c88] em +0xd5, o id
/// (+0xd6) e +0xda zerados. Devolve a notícia.
pub(crate) unsafe extern "thiscall" fn construct_news_item(this: usize, kind: u32, importante: u32) -> usize {
    exe::date_date_54a110(this + news_item::DATE);
    write_usize(this, kind as usize);
    write_u8(this + news_item::IMPORTANT, importante as u8);
    for k in 0..50 {
        write_usize(this + news_item::DATA + 4 * k, 0);
    }
    let mut temporary = [0u32; 2];
    exe::date_operator_3d(this + news_item::DATE, temporary.as_mut_ptr() as usize, G_CURRENT_DATE);
    write_u8(this + news_item::DAY_SLICE, read_u8(G_CURRENT_DAY_SLICE));
    write_usize(this + news_item::ID, 0);
    write_usize(this + news_item::READ, 0);
    this
}

/// Returns a new array with the human's inbox items, newest first, only those of one category when asked, and the
/// count.
///
/// `News::get_inbox_items(manager, count, category)`: a new vector of 100 news items (malloc `exe::crt_malloc` of 0x571c, on the
/// first one found; `*count` = 0) with the news of the human's ring, from the newest to the oldest up to the empty
/// slot, only those of the `category` (`get_category`) if it is not 0, copied (`news_operator`); `*count` adds one per news item.
/// Returns the vector (0 if none; `*count` untouched). No manager or a manager of the machine: the "Error" box and 0.
/// Original: `News::get_inbox_items` (0x786960, news.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x786960 } (thiscall, ret 0xc, news.cpp): `News::get_inbox_items(tecnico, conta, categoria)`: um vetor novo de 100
/// notícias (malloc 0x944e46 de 0x571c, na primeira achada; `*conta` = 0) com as notícias do anel do humano, da mais nova
/// para a mais antiga até a vaga vazia, só as da `categoria` (0x785670) se ela não for 0, copiadas (0x786010); `*conta`
/// soma uma por notícia. Devolve o vetor (0 sem nenhuma; `*conta` intocada). Sem técnico ou técnico da máquina: a caixa
/// "Error" e 0.
pub(crate) unsafe extern "thiscall" fn get_inbox_items(this: usize, manager: usize, count: *mut i32, category: u32) -> usize {
    if manager == 0 {
        show_error_at(0xa693ec, 0x21fa);
        return 0;
    }
    let idx = read_i32(manager).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10);
    if idx < 0 {
        show_error_at(0xa693ec, 0x2205);
        return 0;
    }
    let e = read_usize(this + news_fields::HUMAN_INBOXES) + idx as usize * 12;
    if read_i32(e) == -1 {
        return 0;
    }
    let mut array = 0usize;
    let mut k = read_i32(e + 4) - 1;
    loop {
        if k < read_i32(e) {
            k = read_i32(e) + 0x63;
        }
        let slot = read_usize(this).wrapping_add((k as usize).wrapping_mul(MEDIA_NEWS_ITEM_SIZE));
        if read_usize(slot + news_item::ID) == 0 {
            return array;
        }
        if category as u8 == 0 || (exe::newsitem_get_category(slot) as u32) as u8 == category as u8 {
            if array == 0 {
                array = exe::crt_malloc(0x571c);
                if array == 0 {
                    show_error_at(0xa693ec, 0x222a);
                    return 0;
                }
                *count = 0;
            }
            let n = *count;
            *count = n + 1;
            exe::newsitem_operator_3d(array.wrapping_add((n as usize).wrapping_mul(MEDIA_NEWS_ITEM_SIZE)), read_usize(this).wrapping_add((k as usize).wrapping_mul(MEDIA_NEWS_ITEM_SIZE)));
        }
        k -= 1;
        if k == read_i32(e + 4) - 1 {
            return array;
        }
    }
}

/// Returns the news category by the thousands range of its type.
///
/// (thiscall) The news category by the thousands of the type (+0): 1..999 → 2, 1000.. → 7, 2000.. → 3, 3000.. → 4,
/// 4000.. → 5, 5000.. → 1, 6000..0x1790 → 6. Anything else: as in the original (error box). Returns the whole eax like
/// the original (the type with al replaced).
/// Original: `NewsItem::get_category` (0x785670, news.cpp, thiscall).
///
/// [pt] { 0x785670 } (thiscall, news.cpp): a categoria da notícia pelo milhar do tipo (+0): 1..999 → 2,
/// 1000.. → 7, 2000.. → 3, 3000.. → 4, 4000.. → 5, 5000.. → 1, 6000..0x1790 → 6. Fora disso: a original
/// (caixa de erro). Devolve o eax inteiro como a original (o tipo com o al trocado).
pub(crate) unsafe extern "thiscall" fn get_category(news: usize) -> u32 {
    let t = read_i32(news);
    let c = match t {
        1..=999 => 2,
        1000..=1999 => 7,
        2000..=2999 => 3,
        3000..=3999 => 4,
        4000..=4999 => 5,
        5000..=5999 => 1,
        6000..=0x1790 => 6,
        _ => return original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GET_CATEGORY)(news),
    };
    (t as u32 & 0xffff_ff00) | c
}

/// the value of one of the 50 news item slots
/// Original: `NewsItem::get_slot` (0x785830, news.cpp, thiscall).
///
/// [pt] { 0x785830 } (thiscall, ret 4, news.cpp): o valor da vaga `vaga` (0 a 49) da notícia (+5, 4 bytes
/// cada). Fora da faixa: a original (caixa de erro, devolve 0).
pub(crate) unsafe extern "thiscall" fn get_slot(news: usize, slot: i8) -> u32 {
    if !(0..0x32).contains(&slot) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(IDX_GET_SLOT)(news, slot);
    }
    read_u32(news + 5 + slot as usize * 4)
}

/// stores news field 0..49
/// Original: `set_news_field` (0x785790, news.cpp, thiscall).
///
/// [pt] { 0x785790 } (thiscall, ret 8, news.cpp): grava `valor` no dado `k` 0..49 da notícia (+5, 4 bytes
/// cada). Devolve 1. `k` fora: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn set_news_field(t: usize, k: i8, value: u32) -> u32 {
    if !(0..0x32).contains(&k) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i8, u32) -> u32>(IDX_SET_NEWS_FIELD)(t, k, value);
    }
    write_u32(t + 5 + 4 * k as usize, value);
    1
}

/// Copies a news item (0xdf bytes): +0, +4, the 50 tokens (+5), the date (+0xcd), +0xd5, +0xd6, +0xda and +0xde.
///
/// `NewsItem::operator=`: copies the news item (0xdf bytes) from `source`: +0 (dword), +4 (byte), the 50 tokens from +5
/// (dwords, in order), the date at +0xcd, +0xd5 (byte), +0xd6 and +0xda (dwords) and +0xde (byte). Returns `this`.
/// Original: `NewsItem::operator=` (0x786010, news.cpp, thiscall, ret 4).
///
/// [pt] { 0x786010 } (thiscall, ret 4, news.cpp): `NewsItem::operator=`: copia a notícia (0xdf bytes) de `origem`: +0
/// (dupla), +4 (byte), as 50 fichas de +5 (duplas, na ordem), a data em +0xcd, +0xd5 (byte), +0xd6 e +0xda (duplas) e
/// +0xde (byte). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn news_operator(this: usize, source: usize) -> usize {
    let d = |o: usize| write_u32(this + o, read_u32(source + o));
    let b = |o: usize| write_u8(this + o, read_u8(source + o));
    d(0);
    b(4);
    for k in 0..50 {
        d(5 + 4 * k); // the news item’s records
    }
    copy_record_date(this + news_item::DATE, source + news_item::DATE);
    b(news_item::DAY_SLICE);
    d(news_item::ID);
    d(news_item::READ);
    b(news_item::REACH);
    this
}

