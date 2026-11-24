use super::*;

/// Contract squad status (high nibble of +0x4f).
pub(crate) unsafe fn contract_squad_status(contract: Contract) -> u8 {
    contract.squad_status() & 0xf0
}

/// A notícia guardada: 0xdf bytes (+0xd6 id, +0xda lida).
pub(crate) const STORED_NEWS_SIZE: usize = 0xdf;

/// Marks read the first unread news item with the id in the human's ring, unless it needs a reply, takes it off the
/// count and, when asked, repeats it for the humans on the same network machine.
///
/// `News::mark_item_read(manager, id, sincronizar)`: in the human's ring of 100 slots (`news_fields::HUMAN_INBOXES`, by
/// id − `G_STAFF_COUNT` + 16), the first slot with the `id` (`news_item::ID`) not yet read (`news_item::READ` = 0)
/// whose button (`exe::newsitem_get_action_button` on 0xde1f64) does not ask for a reply (2) becomes read and leaves
/// the count (+8 of the entry). No ring: 0. Otherwise (even when not found), with `sincronizar`, it repeats (without
/// sync, `exe::news_mark_item_read`) for the other humans (`.classification()` = 1) on the same network machine
/// (`exe::screenmanager_find_manager_channel`) as the current human; returns 1. No manager, no id, or a manager of the
/// machine: the "Error" box and 0.
/// Original: `News::mark_item_read` (0x787140, news.cpp, thiscall, ret 0xc).
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
            let p = Staff(read_usize(G_STAFF_TABLE).wrapping_add((read_i32(G_STAFF_COUNT) - j - 1).wrapping_mul(staff::SIZE as i32) as usize));
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

/// marks the scope as club (`news_item::REACH` = 0) and delivers the item (`exe::news_deliver_to_manager`) to the club
/// manager (`.manager()`) when he is human (among the last 16 people, `G_STAFF_COUNT`); it was turned off (DESLIGADAS)
/// from 2026-10-29 to 2026-10-31 (the detour, even passing through, changed the stack residue a later original read:
/// the career left be54a138); since the test criterion changed and the list was emptied, the copy runs again
/// Original: `News::send_to_club_manager` (0x7861e0, news.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn send_to_club_manager(this: usize, news: usize, club: usize) -> u32 {
    let _texts = news_texts_posting(news); // CM_TEXTO_NOTICIAS
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
/// reads: the career left be54a138); since then the routine is hooked again
/// Original: `News::deliver_to_manager` (0x785d40, news.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn deliver_to_manager(this: usize, news: usize, manager: usize, _free: u32) -> u32 {
    let _texts = news_texts_posting(news); // CM_TEXTO_NOTICIAS
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
pub(crate) unsafe fn human_inbox_entry(this: usize, manager: usize, row: u32) -> Option<usize> {
    let idx = read_i32(manager).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10);
    if idx < 0 {
        show_error_at(0xa693ec, row);
        return None;
    }
    Some(read_usize(this + news_fields::HUMAN_INBOXES) + idx as usize * 12)
}

/// Wraps a ring slot k below inicio to inicio + 99 (100-slot inbox ring).
pub(crate) fn ring_prev_wrap(k: i32, start: i32) -> i32 {
    if k < start { start + 0x63 } else { k }
}

/// Applies the automatic reply for the club to the human's inbox, from the newest item to the oldest, up to the empty
/// slot.
///
/// `News::auto_respond_all(manager, club)`: from the newest to the oldest news item of the human's ring, up to the
/// first empty slot (`news_item::ID` = 0), gives the automatic reply (`exe::newsitem_auto_respond`) for the `club`. No
/// manager or a manager of the machine: the "Error" box and 0; no ring or at the end: 1. TapeCall when going on
/// holiday, when sacked and when hired.
/// Original: `News::auto_respond_all` (0x7874b0, news.cpp, thiscall, ret 8).
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
/// `News::has_urgent_items(manager, so_urgentes)`: without `so_urgentes`, whether there is an unread news item
/// (`exe::news_count_unread` > 0). With it, from the newest to the oldest of the human's ring, up to the first empty
/// slot (`news_item::ID` = 0): 1 if one needs a reply (the text `exe::newsitem_get_action_button` on 0xde1f64 returns
/// 2) or is important (`news_item::IMPORTANT` = 1) and unread (`news_item::READ` = 0). A manager of the machine: the
/// "Error" box and 0.
/// Original: `News::has_urgent_items` (0x787350, news.cpp, thiscall, ret 8).
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
/// `NewsItem::set_slot_string(field, text, next)`: copies the `text` into the news item's slots from slot `field`
/// (strncpy `exe::at_9466d0` at `news_item::DATA` + 4·field, up to 200 − 4·field bytes), terminates the area
/// (`news_item::TEXT_END` = 0) and, with `next`, writes the first free slot after the text (the length with the
/// terminating zero rounded up to 4, in slots, plus `field`; at most 50). Slot outside 0..49 or no text: the "Error"
/// box and 0; otherwise 1.
/// Original: `NewsItem::set_slot_string` (0x7858c0, news.cpp, thiscall, ret 0xc).
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

/// Sets the continent scope (`news_item::REACH` = 3) and delivers the news to every active human manager whose club or
/// nation is in the continent.
///
/// `News::send_to_continent_managers(news, continent)`: continent scope (`news_item::REACH` = 3) and delivery
/// (`exe::news_deliver_to_manager` with 0) to each active human manager whose club (`staff::CLUB_JOB`) has its nation
/// (`.nation()`) in the `continent` (`.continent()`) or whose nation (`staff::NATION`) is in it. Returns 1, or 0 on the
/// first refusal. Null news or continent: as in the original ("Error" box).
/// Original: `News::send_to_continent_managers` (0x786640, news.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn send_to_continent_managers(this: usize, news: usize, continent: usize) -> u32 {
    let _texts = news_texts_posting(news); // CM_TEXTO_NOTICIAS
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

/// Delivers the news to the managers of the competition's nation (`send_to_nation_club_managers`), else of its
/// continent (`send_to_continent_managers`), else to all (`send_to_all_managers`).
///
/// `News::send_to_comp_region(news, comp)`: with the competition's nation (`.nation()`), to the nation's managers
/// (`FN_NEWS_SEND_TO_NATION_CLUB_MANAGERS`); otherwise, with the continent (`.continent()`), to the continent's
/// (`FN_NEWS_SEND_TO_CONTINENT_MANAGERS`); otherwise to all (`FN_NEWS_SEND_TO_ALL_MANAGERS`). Returns what the delivery
/// returns. Null news or competition: as in the original ("Error" box).
/// Original: `News::send_to_comp_region` (0x786860, news.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn send_to_comp_region(this: usize, news: usize, comp: usize) -> u32 {
    let _texts = news_texts_posting(news); // CM_TEXTO_NOTICIAS
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
pub(crate) unsafe fn manager_of_news_club(news: usize, o: usize) -> usize {
    read_usize(read_usize(G_CLUB_TABLE).wrapping_add(read_usize(news + o).wrapping_mul(club::SIZE)) + 0xcf)
}

/// Tells whether the news item is important for the person, by category and type (bids on the person's club, the
/// humans...).
///
/// `person.is_important_for()`: is the news important for the person (the byte +4 that the delivery
/// `deliver_to_manager` writes)? By the category (`exe::newsitem_get_category`) and the type (+0): category 2, types 7
/// to 0xb, 0x27 and 0x28; category 4, 0xbb8, 0xbb9 and 0xbc4; category 5, a fixed list, 0xfa1 if the person is the
/// manager of the club in data 5 (+0x19) and 0xfb6 if he is that of data 4 (+0x15) or of data 5; category 6, 0x1770 if
/// data 0 (`news_item::DATA`) is one of the 16 human managers (`G_STAFF_COUNT`); category 1, 0x1388; the rest, 0. The
/// original calls `exe::newsitem_get_category` up to six times (it only reads): the port calls it once. Null person or
/// type outside 1..0x1790 (`exe::newsitem_get_category`'s box): as in the original.
/// Original: `NewsItem::is_important_for` (0x784eb0, news.cpp, thiscall, ret 4).
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
pub(crate) unsafe fn club_or_current_human(club: usize) -> usize {
    if club != 0 {
        return club;
    }
    read_usize(read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize) + 0x39)
}

/// Human manager i (0..15, last 16 people of G_STAFF_TABLE) if active (+0x5f = 1).
pub(crate) unsafe fn active_human_manager(i: usize) -> Option<usize> {
    let p = Staff(read_usize(G_STAFF_TABLE).wrapping_add(read_usize(G_STAFF_COUNT).wrapping_sub(0x10).wrapping_add(i).wrapping_mul(staff::SIZE)));
    (p.classification() == 1).then_some(p.addr())
}

/// Delivers the news to every active human manager that passa accepts, with reach alcance in +0xde; false at the first
/// refusal.
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

/// all scope (`news_item::REACH` = 4) and delivery (`exe::news_deliver_to_manager`) to every active human manager; 1,
/// or 0 on the first refusal
/// Original: `News::send_to_all_managers` (0x786780, news.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn send_to_all_managers(this: usize, news: usize) -> u32 {
    let _texts = news_texts_posting(news); // CM_TEXTO_NOTICIAS
    if news == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SEND_TO_ALL_MANAGERS)(this, news); // message box: original only
    }
    deliver_to_human_managers(this, news, 4, |_| true) as u32
}

/// Sets the division scope (`news_item::REACH` = 1) and delivers the news to every active human manager with a club in
/// the division (`.division()`); returns 1, or 0 on the first refusal.
///
/// `News::send_to_division_managers(news, division)`: division scope (`news_item::REACH` = 1) and delivery to each
/// active human manager with a club (`staff::CLUB_JOB`) in the `division` (`.division()`). Returns 1, or 0 on the first
/// refusal. Null news or division: as in the original ("Error" box).
/// Original: `News::send_to_division_managers` (0x7862d0, news.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn send_to_division_managers(this: usize, news: usize, division: usize) -> u32 {
    let _texts = news_texts_posting(news); // CM_TEXTO_NOTICIAS
    if news == 0 || division == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SEND_TO_DIVISION_MANAGERS)(this, news, division); // message box: original only
    }
    deliver_to_human_managers(this, news, 1, |p| {
        let c = Club(read_usize(p + staff::CLUB_JOB));
        !c.is_null() && c.division().addr() == division // the club’s division
    }) as u32
}

/// Sets the nation scope (`news_item::REACH` = 2) and delivers the news to every active human manager whose club or
/// national team belongs to the nation (`club::NATION`); returns 0.
///
/// `News::send_to_nation_managers(news, nation)`: nation scope (`news_item::REACH` = 2) and delivery to each active
/// human manager whose club (`staff::CLUB_JOB`) or national team (`staff::NATIONAL_JOB`) belongs to the `nation`
/// (`club::NATION`). Returns 0. Null news or nation: as in the original ("Error" box).
/// Original: `News::send_to_nation_managers` (0x7863f0, news.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn send_to_nation_managers(this: usize, news: usize, nation: usize) -> u32 {
    let _texts = news_texts_posting(news); // CM_TEXTO_NOTICIAS
    if news == 0 || nation == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SEND_TO_NATION_MANAGERS)(this, news, nation); // message box: original only
    }
    deliver_to_human_managers(this, news, 2, |p| {
        let (c, s) = (read_usize(p + staff::CLUB_JOB), read_usize(p + staff::NATIONAL_JOB));
        (c != 0 && read_usize(c + club::NATION) == nation) || (s != 0 && read_usize(s + club::NATION) == nation) // the club or the national team
    });
    0
}

/// Sets the nation scope (`news_item::REACH` = 2) and delivers the news (`exe::news_deliver_to_manager`) to every
/// active human manager with a club of the nation; the first refusal stops.
///
/// `News::send_to_nation_club_managers(news, nation)`: the news gets the nation scope (`news_item::REACH` = 2) and goes
/// (`exe::news_deliver_to_manager` with 0) to each of the 16 human managers (the last 16 of `G_STAFF_TABLE`,
/// `staff::SIZE` bytes) who are active (`.classification()` = 1) with a club (`.club_job()`) of the `nation`
/// (`.nation()`); the first refused delivery (0) stops it. Returns 0. Null news or nation: as in the original ("Error"
/// box).
/// Original: `News::send_to_nation_club_managers` (0x786520, news.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn send_to_nation_club_managers(this: usize, news: usize, nation: usize) -> u32 {
    let _texts = news_texts_posting(news); // CM_TEXTO_NOTICIAS
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
pub(crate) unsafe fn manager_inbox_entry(news: usize, manager: usize) -> Option<usize> {
    let i = read_i32(manager).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10);
    if i < 0 {
        return None;
    }
    Some(read_usize(news + 8).wrapping_add((i as usize).wrapping_mul(12)))
}

/// News item k of the inbox starting at inicio: [news] + 0xdf × (inicio + k).
pub(crate) unsafe fn inbox_news_item(news: usize, start: i32, k: i32) -> usize {
    read_usize(news).wrapping_add((start.wrapping_add(k) as usize).wrapping_mul(0xdf))
}

/// Counts the unread news in a human manager's inbox, in total or for one category.
///
/// Without a category (0): the entry's total (+8, negative becomes 0). With a category: of the 100 news items in the
/// inbox (start at entry +0; −1 = empty, 0), the occupied ones (`news_item::ID`) of the category
/// (`exe::newsitem_get_category`) not yet read (`news_item::READ` = 0). Null manager or out of range: as in the
/// original (error box).
/// Original: `News::count_unread` (0x786fe0, news.cpp, thiscall).
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

/// Gives the default replies for the manager's club and national team (`exe::news_auto_respond_all`) and empties his
/// inbox (the 100 news items with `news_item::ID` = 0 and `news_item::READ` = 1).
///
/// `News::clear_inbox(manager)`, from 0x6b28b0 and from loading the game (`load_game_data`): the default replies to the
/// news of the manager's club (+0x39) and national team (+0x24) (`exe::news_auto_respond_all`); then the human's inbox
/// (12-byte record at [`news_fields::HUMAN_INBOXES`], by id − `G_STAFF_COUNT` + 16): +4 = +0 and +8 = 0; with the first
/// news item (+0, −1 none), the 100 news items of 0xdf bytes from it ([this]) with `news_item::ID` = 0 and
/// `news_item::READ` = 1. No manager or a negative index: the error box (lines 0x24c1 and 0x24cc).
/// Original: `News::clear_inbox` (0x787630, news.cpp, thiscall, ret 4).
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

/// builds the news item: type (+0), important byte (`news_item::IMPORTANT`), the 50 data slots (`news_item::DATA`)
/// cleared, today's date (`news_item::DATE`), byte `G_CURRENT_DAY_SLICE` at `news_item::DAY_SLICE`, id
/// (`news_item::ID`) and `news_item::READ` cleared; it was turned off (DESLIGADAS) from 2026-10-29 to 2026-10-31: with
/// the detour the career in Rust mode left the reference be54a138 (bisection of batch 251's 39 routines; with
/// `construct_news_item`, the news constructor, the game later crashed in `player_stats_reset_player` on a broken
/// index), and the cause (stack residue or a difference in the copy) was not isolated; since the test criterion changed
/// and the list was emptied, the copy runs again
/// Original: `NewsItem::NewsItem` (0x77bbe0, news.cpp, thiscall, ret 8).
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
/// `News::get_inbox_items(manager, count, category)`: a new vector of 100 news items (malloc `exe::crt_malloc` of
/// 0x571c, on the first one found; `*count` = 0) with the news of the human's ring (`news_fields::HUMAN_INBOXES`), from
/// the newest to the oldest up to the empty slot (`news_item::ID` = 0), only those of the `category`
/// (`exe::newsitem_get_category`) if it is not 0, copied (`exe::newsitem_operator_3d`); `*count` adds one per news
/// item. Returns the vector (0 if none; `*count` untouched). No manager or a manager of the machine: the "Error" box
/// and 0.
/// Original: `News::get_inbox_items` (0x786960, news.cpp, thiscall, ret 0xc).
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
pub(crate) unsafe extern "thiscall" fn get_slot(news: usize, slot: i8) -> u32 {
    if !(0..0x32).contains(&slot) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(IDX_GET_SLOT)(news, slot);
    }
    read_u32(news + 5 + slot as usize * 4)
}

/// stores news field 0..49
/// Original: `set_news_field` (0x785790, news.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn set_news_field(t: usize, k: i8, value: u32) -> u32 {
    if !(0..0x32).contains(&k) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i8, u32) -> u32>(IDX_SET_NEWS_FIELD)(t, k, value);
    }
    write_u32(t + 5 + 4 * k as usize, value);
    1
}

/// Copies a news item (0xdf bytes): +0, +4, the 50 tokens (+5), the date (`news_item::DATE`), `news_item::DAY_SLICE`,
/// `news_item::ID`, `news_item::READ` and `news_item::REACH`.
///
/// `NewsItem::operator=`: copies the news item (0xdf bytes) from `source`: +0 (dword), +4 (byte), the 50 tokens from +5
/// (dwords, in order), the date at `news_item::DATE`, `news_item::DAY_SLICE` (byte), `news_item::ID` and
/// `news_item::READ` (dwords) and `news_item::REACH` (byte). Returns `this`.
/// Original: `NewsItem::operator=` (0x786010, news.cpp, thiscall, ret 4).
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

/// Writes the headline (`body` 0) or the body of a news item into `out`, returning 1 (0 for a type-26 news whose
/// manager record is missing, and after an error box).
///
/// `NewsItem::write_text(news, manager, out, body, flag)`: with no manager or no `out`, the news.cpp error box (lines
/// 0x9f and 0xa7) and 0. Otherwise `out` is cleared (`exe::text_clear`) and the news is written by its category
/// (`exe::newsitem_get_category`) and type (the news `TYPE` dword, +0): most types go to the writer of their subject
/// (`NEWS_WRITERS`: the media, the coach report, the transfers, the competitions, the discipline, the records...); the
/// types the routine writes itself are the pieces below (category 2: the match moved, televised or rearranged, the host
/// bids, the finances, the season's end, the job offers...; category 3: the match report; category 7: the club
/// record; category 1: the follow-up news). Types 27, 28, 31 and 35 call their writers with other arguments (see the
/// code). An unknown type gives the error box of line 0x1226 and 0.
/// Original: `NewsItem::write_text` (0x77bc80, news.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn newsitem_write_text(news: usize, manager: usize, out: usize, body: u32, flag: u32) -> u32 {
    NEWS_LAST_PIECE = "";
    if manager == 0 {
        news_error_box(0x9f);
        return 0;
    }
    if out == 0 {
        news_error_box(0xa7);
        return 0;
    }
    let clear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_TEXT_CLEAR); // Text::clear
    clear(out);
    let cat = exe::newsitem_get_category(news);
    let ty = read_u32(news);
    let three = |f: usize, this: usize| {
        let w: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(f);
        w(this, news, body, out);
    };
    if cat == 2 {
        match ty {
            0x1b => {
                // the club record news: (this, f5, f9, news, out, body)
                let w: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize, u32) -> u32 = core::mem::transmute(FN_AWARD_MANAGER_WRITE_AWARD_NEWS);
                w(read_usize(0xad9c60), read_u32(news + 5), read_u32(news + 9), news, out, body);
                return 1;
            }
            0x1c => {
                let w: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_AWARD_MANAGER_WRITE_SHORTLIST_NEWS);
                w(read_usize(0xad9c60), news, out, body);
                return 1;
            }
            0x1f => {
                three(0x48eb00, read_usize(read_usize(0xadadfc).wrapping_add((read_u32(news + 9) as usize).wrapping_mul(4))));
                return 1;
            }
            0x23 => {
                three(0x48ee40, read_usize(read_usize(0xadadfc).wrapping_add((read_u32(news + 5) as usize).wrapping_mul(4))));
                return 1;
            }
            0x1a => {
                let coach = read_usize(read_usize(0xadadfc).wrapping_add((read_u32(news + 5) as usize).wrapping_mul(4)));
                if coach == 0 {
                    return 0;
                }
                three(0x48e590, coach);
                return 1;
            }
            1 => return news_match_rearranged(news, body, out),
            3 => return news_match_televised(news, body, out),
            4 => return news_host_bids(news, body, out),
            5 => return news_host_chosen(news, body, out),
            6 => return news_friendly_proposal(news, body, out),
            7 => return news_friendly_answered(news, body, out),
            8 => return news_friendly_cancelled(news, body, out),
            9 => return news_tournament_invitation(news, body, out),
            10 => return news_tournament_answered(news, body, out),
            11 => return news_tournament_cancelled(news, body, out),
            12 => return news_comp_report(news, body, out),
            13 => return news_fixture_list(news, body, out),
            16 => return news_comp_match_report(news, body, out),
            17 => return news_playoff_place(news, body, out, false),
            18 => return news_playoff_place(news, body, out, true),
            19 => return news_leagues_reshuffled(news, body, out),
            20 => return news_cup_draw(news, body, out),
            21 => return news_group_draw(news, body, out),
            22 => return news_intertoto_invitation(news, body, out),
            23 => return news_intertoto_chosen(news, body, out),
            0x26 => return news_cup_draw_open(news, body, out),
            0x27 => return news_cup_draw_today(news, body, out),
            0x28 => return news_world_club_bye(news, body, out),
            _ => {}
        }
    } else if cat == 7 && ty == 0x3e9 {
        return news_club_record(news, body, out);
    } else if cat == 3 && ty == 0x7d0 {
        return news_match_report(news, body, out);
    } else if cat == 1 && ty == 0x1388 {
        return news_follow_up(news, manager, out, body, flag);
    }
    if let Some(&(_, _, w)) = NEWS_WRITERS.iter().find(|e| e.0 == cat && e.1 == ty) {
        match w {
            NewsWriter::Global(g, f) => three(f, read_usize(g)),
            NewsWriter::Object(o, f) => three(f, o),
            NewsWriter::Plain(f) => {
                let w: unsafe extern "C" fn(usize, u32, usize) -> u32 = core::mem::transmute(f);
                w(news, body, out);
            }
        }
        return 1;
    }
    news_error_box(0x1226);
    0
}

/// Compares `NewsItem::write_text`: the original writes into `out` and the copy into a local text, side by side
/// (`compare_text_writer`: the text, the links and the random generator), and the return values. Without a manager or
/// `out` (the error box) only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_newsitem_write_text(news: usize, manager: usize, out: usize, body: u32, flag: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32>(IDX_NEWSITEM_WRITE_TEXT);
    if manager == 0 || out == 0 {
        return o(news, manager, out, body, flag);
    }
    let (ro, rr) = (core::cell::Cell::new(0u32), core::cell::Cell::new(0u32));
    let (cat, ty) = (exe::newsitem_get_category(news), read_u32(news));
    compare_text_writer(out, || ro.set(o(news, manager, out, body, flag)), |t| rr.set(newsitem_write_text(news, manager, t, body, flag)),
        || format!("newsitem_write_text(category {}, type {:#x}, body {}, flag {}, piece {})", cat, ty, body, flag, NEWS_LAST_PIECE));
    SCOREBOARD.record(ro.get() == rr.get(), || format!("newsitem_write_text(type {:#x}) returned {} x {}", ty, ro.get(), rr.get()));
    ro.get()
}

/// Writes the button label and the screen handler of a news item for a manager and says whether it needs a reply.
///
/// `NewsItem::get_action_button(manager, label, size, &handler)`: no manager (box of line 0x123b), no label or a size
/// below 0x33 (line 0x1243) or no handler (line 0x124e, after the label is emptied): the "Error" box and 0. Otherwise
/// the label is emptied, the handler set to 0 and the item goes to the piece of its category (`get_category`, asked
/// again before each test, as the original does): 6 the board (`board_button`), 2 the general news
/// (`general_button`), 4 the player news (`player_button`), 3 the discipline (`discipline_button`), 5 the transfers
/// (`transfer_button`) and 1, type 0x1388 only, "Reply" with handler 0x789860 and 1. A board item that gets no button
/// falls through the other tests to 0. Returns 2 when the item needs a reply (the automatic reply
/// `NewsItem::auto_respond` 0x784100, `mark_item_read` and `has_urgent_items` look for it), 1 for a button without a
/// needed reply, 0 for none. Some items are also closed on the way: the national squad (0x22) not of today and the
/// cup draw (0x27) with every fixture made get their answer slot set, and the transfer step (0xfa4) without a button
/// gets its offer slot set to −1 (`set_item_slot`).
/// Original: `NewsItem::get_action_button` (0x782b70, news.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn get_action_button(news: usize, manager: usize, label: usize, size: u32, handler: *mut u32) -> u8 {
    if manager == 0 {
        show_error_at(0xa693ec, 0x123b);
        return 0;
    }
    if label == 0 || (size as i32) < 0x33 {
        show_error_at(0xa693ec, 0x1243);
        return 0;
    }
    write_u8(label, 0);
    if handler.is_null() {
        show_error_at(0xa693ec, 0x124e);
        return 0;
    }
    *handler = 0;
    let category = || get_category(news) as u8;
    if category() == 6 {
        if let Some(r) = board_button(news, label, handler) {
            return r;
        }
    }
    if category() == 2 {
        return general_button(news, manager, label, handler);
    }
    if category() == 4 {
        return player_button(news, manager, label, handler);
    }
    if category() == 3 {
        return discipline_button(news, manager, label, handler);
    }
    if category() == 5 {
        return transfer_button(news, manager, label, handler);
    }
    if category() == 1 && read_i32(news) == 0x1388 {
        covered!("nma_8 reply button");
        return set_action_button(label, handler, 0xa69414, 0x789860, 1); // "Reply"
    }
    0
}

/// Comparer for `get_action_button`: the original and the copy side by side (the label, the handler, the contract
/// lookup global, the item and the news storage that `set_item_slot` writes); a divergence names the piece (the
/// item's category). The error boxes (no manager, label or handler, a type outside the categories) run the original
/// only.
pub(crate) unsafe extern "thiscall" fn compare_get_action_button(news: usize, manager: usize, label: usize, size: u32, handler: *mut u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u32) -> u8>(IDX_GET_ACTION_BUTTON);
    let kind = read_i32(news);
    if manager == 0 || label == 0 || (size as i32) < 0x33 || handler.is_null() || !(1..=0x1790).contains(&kind) {
        return o(news, manager, label, size, handler);
    }
    let store = read_usize(G_NEWS);
    let mut regions = vec![
        (label, (size as usize).min(0x7d0)),
        (handler as usize, 4usize),
        (G_CONTRACT_LOOKUP_RESULT, 4),
        (news, NEWS_NEWS_ITEM_SIZE),
    ];
    if store != 0 && read_usize(store) != 0 {
        regions.push((read_usize(store), read_i32(store + news_fields::SLOT_COUNT).max(0) as usize * NEWS_NEWS_ITEM_SIZE));
    }
    let (r, n, same, _, _) = crate::comparison::side_by_side(&regions,
        || o(news, manager, label, size, handler),
        || get_action_button(news, manager, label, size, handler));
    SCOREBOARD.record(same && r == n, || format!("get_action_button piece {} (type {:#x}) original={} rust={} handler {:#x} label {:?} memory equal {}",
        action_button_piece(kind), kind, r, n, *handler, (0..0x40).map(|k| read_u8(label + k)).take_while(|&b| b != 0).map(|b| b as char).collect::<String>(), same));
    r
}

/// The news.cpp "Error" message box for source line `line`: the file name and extension (`exe::crt_strtok`, the
/// C runtime splitpath, of the news.cpp path 0xa693ec), the version, "v%s %s.%s %d" at 0xae24d0, the box
/// (`exe::gui_show_message_box`) and the mouse button released.
pub(crate) unsafe fn news_error_box(line: u32) {
    let (mut name, mut ext) = ([0u8; 0x100], [0u8; 0x100]);
    exe::crt_strtok(0xa693ec, 0, 0, name.as_mut_ptr(), ext.as_mut_ptr());
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let v = version(name.as_mut_ptr(), ext.as_mut_ptr(), line);
    news_sprintf()(0xae24d0, 0x9870e8, v, name.as_ptr(), ext.as_ptr(), line); // "v%s %s.%s %d"
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0); // "Error"
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
}

/// Type 1, the match rearranged. Slots: +5 and +9 the two clubs, +0xd the competition, +0x11 and +0x15 the stage and
/// the round (words), +0x19/+0x1d the old date and +0x35/+0x39 the new one (`Date::Date` of two words each), +0x21 a
/// club, +0x25 another club and +0x29 a competition of the clash (+0x21 = −1 none).
///
/// Body: the round's label (`exe::comp_util_format_comp_stage_round_label`, form 2; "Friendly" and "Reserve" get a
/// lower-case first letter) and its gender (`exe::comp_util_get_label_gender`), both dates without the year
/// (`exe::date_get_long_date_string_no_year`); with a clash 0xa692ec (the clubs' names and genders, the label, the
/// dates, the clash clubs' and competition's short names and short genders), otherwise 0xa691c8. Links: the two clubs
/// (0x7d5), the competition on the label (0x7d0), and with a clash its two clubs' short names and its competition's.
/// Headline 0xa69184 with the clubs' short names and short genders and their links.
/// Original: part of `NewsItem::write_text` (0x77bfeb..0x77c46a).
pub(crate) unsafe fn news_match_rearranged(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "match_rearranged";
    let f = news_format();
    let (c1, c2) = (news_club(read_u32(news + 5)), news_club(read_u32(news + 9)));
    if body == 0 {
        f(NEWS_TEXT_BUFFER, 0xa69184, news_g(c1 + club::SHORT_GENDER_NAME), news_g(c1 + club::SHORT_GENDER_NAME),
          news_g(c2 + club::SHORT_GENDER_NAME), news_g(c2 + club::SHORT_GENDER_NAME), c1 + club::SHORT_NAME, c2 + club::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, c1 + club::SHORT_NAME, 0x7d5, c1);
        news_link(out, c2 + club::SHORT_NAME, 0x7d5, c2);
        return 1;
    }
    covered!("gra_1 match rearranged body");
    let comp = news_comp(read_u32(news + 0xd));
    let (stage, round) = (read_u16(news + 0x11) as u32, read_u16(news + 0x15) as u32);
    let mut label = [0u8; 0xc8];
    let lb = label.as_mut_ptr() as usize;
    exe::comp_util_format_comp_stage_round_label(comp, stage, round, 2, label.as_mut_ptr(), 0xc8);
    if news_streq(lb, 0x9db958) {
        label[0] = b'f'; // "Friendly"
    } else if news_streq(lb, 0x9b5b40) {
        label[0] = b'r'; // "Reserve"
    }
    let lg = exe::comp_util_get_label_gender(comp as u32, stage, round, 2) as i8 as i32;
    let (mut d1, mut d2) = ([0u8; 8], [0u8; 8]);
    let (mut s1, mut s2) = ([0u8; 0xc8], [0u8; 0xc8]);
    news_date(d1.as_mut_ptr() as usize, read_u16(news + 0x19), read_u16(news + 0x1d));
    exe::date_get_long_date_string_no_year(d1.as_ptr() as usize, s1.as_mut_ptr() as usize, 0xc8, 0xff);
    news_date(d2.as_mut_ptr() as usize, read_u16(news + 0x35), read_u16(news + 0x39));
    exe::date_get_long_date_string_no_year(d2.as_ptr() as usize, s2.as_mut_ptr() as usize, 0xc8, 0xff);
    let (s1, s2) = (s1.as_ptr() as usize, s2.as_ptr() as usize);
    let clash = read_i32(news + 0x21);
    let (g1, g2) = (news_g(c1 + club::GENDER_NAME), news_g(c2 + club::GENDER_NAME));
    if clash != -1 {
        let comp2 = news_comp(read_u32(news + 0x29));
        let cc = news_club(read_u32(news + 0x25));
        let cb = news_club(clash as u32);
        let (gk, gc, gb) = (news_g(comp2 + competition::SHORT_GENDER_NAME), news_g(cc + club::SHORT_GENDER_NAME), news_g(cb + club::SHORT_GENDER_NAME));
        f(NEWS_TEXT_BUFFER, 0xa692ec, g1, g1, g2, g2, lg, lg, 0x64, gb, gb, gc, gc, gk, gk, 0x64, c1 + club::NAME, c2 + club::NAME,
          lb, s1, cb + club::SHORT_NAME, cc + club::SHORT_NAME, comp2 + competition::SHORT_NAME, s2);
    } else {
        f(NEWS_TEXT_BUFFER, 0xa691c8, lg, lg, g1, g1, g2, g2, 0x64, 0x64, lb, c1 + club::NAME, c2 + club::NAME, s2, s1);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c1 + club::NAME, 0x7d5, c1);
    news_link(out, c2 + club::NAME, 0x7d5, c2);
    news_link(out, lb, 0x7d0, comp);
    if clash != -1 {
        covered!("gra_1 match rearranged clash");
        let cb = news_club(clash as u32);
        news_link(out, cb + club::SHORT_NAME, 0x7d5, cb);
        let cc = news_club(read_u32(news + 0x25));
        news_link(out, cc + club::SHORT_NAME, 0x7d5, cc);
        let comp2 = news_comp(read_u32(news + 0x29));
        news_link(out, comp2 + competition::SHORT_NAME, 0x7d0, comp2);
    }
    1
}

/// Type 3, the match chosen for live television. Slots: +5 and +9 the clubs, +0xd the competition, +0x11 and +0x15
/// the stage and round (words), +0x19/+0x29 and +0x25 the old and new date words (+0x29 the year of the new date,
/// +0x19 the day of the old one compared with the new +0x25), +0x21 and +0x2d the old and new time of day (bytes).
///
/// Body: the round's label (as in type 1, "Friendly"/"Reserve" lower-cased) and its gender; a new day: 0xa690f0 with
/// the new date without the year; the same day at another time of day: 0xa69044 with the time of day's name and
/// gender (`exe::date_get_time_of_day_name`, `exe::date_get_time_of_day_gender`); the same day and time: 0xa68fc0.
/// Links: the two clubs' names (0x7d5) and the competition on the label (0x7d0). Headline 0xa68f7c with the short names.
/// Original: part of `NewsItem::write_text` (0x77c473..0x77c846).
pub(crate) unsafe fn news_match_televised(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "match_televised";
    let f = news_format();
    let (c1, c2) = (news_club(read_u32(news + 5)), news_club(read_u32(news + 9)));
    if body == 0 {
        f(NEWS_TEXT_BUFFER, 0xa68f7c, news_g(c1 + club::SHORT_GENDER_NAME), news_g(c1 + club::SHORT_GENDER_NAME),
          news_g(c2 + club::SHORT_GENDER_NAME), news_g(c2 + club::SHORT_GENDER_NAME), c1 + club::SHORT_NAME, c2 + club::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, c1 + club::SHORT_NAME, 0x7d5, c1);
        news_link(out, c2 + club::SHORT_NAME, 0x7d5, c2);
        return 1;
    }
    covered!("gra_1 match televised body");
    let comp = news_comp(read_u32(news + 0xd));
    let (stage, round) = (read_u16(news + 0x11) as u32, read_u16(news + 0x15) as u32);
    let mut label = [0u8; 0xc8];
    let lb = label.as_mut_ptr() as usize;
    exe::comp_util_format_comp_stage_round_label(comp, stage, round, 2, label.as_mut_ptr(), 0xc8);
    if news_streq(lb, 0x9db958) {
        label[0] = b'f';
    } else if news_streq(lb, 0x9b5b40) {
        label[0] = b'r';
    }
    let lg = exe::comp_util_get_label_gender(comp as u32, stage, round, 2) as i8 as i32;
    let (g1, g2) = (news_g(c1 + club::GENDER_NAME), news_g(c2 + club::GENDER_NAME));
    let (old_time, new_time) = (read_u8(news + 0x21), read_u8(news + 0x2d));
    if read_u16(news + 0x25) != read_u16(news + 0x19) {
        covered!("gra_1 match televised new day");
        let mut d = [0u8; 8];
        let mut s = [0u8; 0xc8];
        news_date(d.as_mut_ptr() as usize, read_u16(news + 0x25), read_u16(news + 0x29));
        exe::date_get_long_date_string_no_year(d.as_ptr() as usize, s.as_mut_ptr() as usize, 0xc8, 0xff);
        f(NEWS_TEXT_BUFFER, 0xa690f0, g1, g1, g2, g2, lg, lg, 0x64, c1 + club::NAME, c2 + club::NAME, lb, s.as_ptr() as usize);
        news_set(out, NEWS_TEXT_BUFFER);
    } else {
        if old_time != new_time {
            covered!("gra_1 match televised time of day");
            let name = exe::date_get_time_of_day_name(new_time as u32, 0);
            let tg = exe::date_get_time_of_day_gender(new_time as u32) as i32;
            f(NEWS_TEXT_BUFFER, 0xa69044, g1, g1, g2, g2, lg, lg, tg, c1 + club::NAME, c2 + club::NAME, lb, name);
        } else {
            f(NEWS_TEXT_BUFFER, 0xa68fc0, g1, g1, g2, g2, lg, lg, c1 + club::NAME, c2 + club::NAME, lb);
        }
        news_set(out, NEWS_TEXT_BUFFER);
    }
    news_link(out, c1 + club::NAME, 0x7d5, c1);
    news_link(out, c2 + club::NAME, 0x7d5, c2);
    news_link(out, lb, 0x7d0, comp);
    1
}

/// Type 4, the bids to host a competition. Slots: +5 the competition, +9 the year (word), +0xd, +0x15 and +0x1d three
/// bidding nations and +0x11 the joint bidder of the first one (−1 none).
///
/// Body: first the nations' links (0x7d5 on the nation's name, to its national team, `exe::database_get_national_team`):
/// the first and its joint bidder, or each of the three that is set; then 0xa68ee0 (three nations) or 0xa68e1c (with
/// the joint bid: four), with the genders, the names, the year and the competition's name; the competition's link
/// (0x7d0). Headline 0xa68de0 with the year and the competition's short name.
/// Original: part of `NewsItem::write_text` (0x77c84f..0x77cb6c).
pub(crate) unsafe fn news_host_bids(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "host_bids";
    let f = news_format();
    let comp = news_comp(read_u32(news + 5));
    let year = read_u16(news + 9) as u32;
    if body == 0 {
        let g = news_g(comp + competition::SHORT_GENDER_NAME);
        f(NEWS_TEXT_BUFFER, 0xa68de0, g, g, year, comp + competition::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 host bids body");
    let n = [read_i32(news + 0xd), read_i32(news + 0x11), read_i32(news + 0x15), read_i32(news + 0x19), read_i32(news + 0x1d)];
    let team_link = |i: i32| {
        let nat = news_nation(i as u32);
        news_link(out, nat + 4, 0x7d5, exe::database_get_national_team(nat));
    };
    for k in 0..3 {
        if k == 0 && n[0] > -1 && n[1] > -1 {
            team_link(n[0]);
            team_link(n[1]);
        } else if n[2 * k] > -1 {
            team_link(n[2 * k]);
        }
    }
    let gc = news_g(comp + competition::GENDER_NAME);
    let nat = |i: i32| news_nation(i as u32);
    if n[1] == -1 {
        let (a, b, c) = (nat(n[0]), nat(n[2]), nat(n[4]));
        f(NEWS_TEXT_BUFFER, 0xa68ee0, news_g(a + 0x37), news_g(a + 0x37), news_g(b + 0x37), news_g(b + 0x37), news_g(c + 0x37),
          news_g(c + 0x37), gc, gc, a + 4, b + 4, c + 4, year, comp + competition::NAME);
    } else {
        covered!("gra_1 host bids joint");
        let (a, j, b, c) = (nat(n[0]), nat(n[1]), nat(n[2]), nat(n[4]));
        f(NEWS_TEXT_BUFFER, 0xa68e1c, news_g(a + 0x37), news_g(a + 0x37), news_g(j + 0x37), news_g(j + 0x37), news_g(b + 0x37),
          news_g(b + 0x37), news_g(c + 0x37), news_g(c + 0x37), gc, gc, a + 4, j + 4, b + 4, c + 4, year, comp + competition::NAME);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, comp + competition::NAME, 0x7d0, comp);
    1
}

/// Type 5, the host of a competition chosen. Slots: +5 the competition, +9 the year (word), +0xd the host nation (−1
/// none) and +0x11 its co-host (−1 none).
///
/// Body: with a host, 0xa68d60 (co-hosts) or 0xa68d00 (one host) with the genders, the names, the year and the
/// competition's name, the text, and each host's link (0x7d5, to its national team); without a host no text; then
/// the competition's link (0x7d0). Headline 0xa68ccc with the year and the competition's short name.
/// Original: part of `NewsItem::write_text` (0x77cb75..0x77cd8e).
pub(crate) unsafe fn news_host_chosen(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "host_chosen";
    let f = news_format();
    let comp = news_comp(read_u32(news + 5));
    let year = read_u16(news + 9) as u32;
    if body == 0 {
        let g = news_g(comp + competition::SHORT_GENDER_NAME);
        f(NEWS_TEXT_BUFFER, 0xa68ccc, g, g, year, comp + competition::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 host chosen body");
    let (h, co) = (read_i32(news + 0xd), read_i32(news + 0x11));
    let gc = news_g(comp + competition::GENDER_NAME);
    let team_link = |nat: usize| news_link(out, nat + 4, 0x7d5, exe::database_get_national_team(nat));
    if h > -1 {
        let a = news_nation(h as u32);
        if co > -1 {
            covered!("gra_1 host chosen co-hosts");
            let b = news_nation(co as u32);
            f(NEWS_TEXT_BUFFER, 0xa68d60, news_g(a + 0x37), news_g(a + 0x37), news_g(b + 0x37), news_g(b + 0x37), gc, gc, a + 4, b + 4,
              year, comp + competition::NAME);
            news_set(out, NEWS_TEXT_BUFFER);
            team_link(a);
            team_link(b);
        } else {
            f(NEWS_TEXT_BUFFER, 0xa68d00, news_g(a + 0x37), news_g(a + 0x37), gc, gc, a + 4, year, comp + competition::NAME);
            news_set(out, NEWS_TEXT_BUFFER);
            team_link(a);
        }
    }
    news_link(out, comp + competition::NAME, 0x7d0, comp);
    1
}

/// Type 6, the friendly proposal. Slots: +5 and +9 the two clubs, +0xd the host club, +0x11 and +0x15 the date
/// (words), +0x19 the answer (3 accepted, 4 rejected).
///
/// Body: the date (`Date::Date`, assigned with `exe::date_operator_3d`), the host's friendly match on that date
/// (`exe::friendly_find_friendly_match`) and the date's long text. When the first club is the host it "offered" the
/// match (0xa68c38 at the first club's stadium, whose name is copied with `sprintf` used as format, or 0xa68bc0
/// without a stadium: the first club then the second); otherwise the first club "challenged" the second (0xa68aac or
/// 0xa68a2c, the second club then the first; the stadium is still the first club's). The answer paragraph: accepted
/// 0xa68b84 or rejected 0xa68b48 with the answering club (the second club in the offer, the first in the challenge),
/// joined as "%s\n\n%s" in the buffer; any other answer by `friendly_check_match_invitation` (the first paragraph alone
/// when it returns nonzero, otherwise joined with what it left in the second). Links: the two clubs' short names
/// (0x7d5). Headline 0xa689d4 with the short names and their links.
/// Original: part of `NewsItem::write_text` (0x77cd97..0x77d2e7).
pub(crate) unsafe fn news_friendly_proposal(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "friendly_proposal";
    let f = news_format();
    let sprintf = news_sprintf();
    let (mut d, mut d2, mut tmp) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let dp = d.as_mut_ptr() as usize;
    exe::date_date_54a110(dp);
    let host = read_u32(news + 0xd);
    let (w1, w2) = (read_u16(news + 0x11), read_u16(news + 0x15));
    let (i1, i2) = (read_u32(news + 5), read_u32(news + 9));
    let answer = read_u8(news + 0x19);
    news_date(d2.as_mut_ptr() as usize, w1, w2);
    exe::date_operator_3d(dp, tmp.as_mut_ptr() as usize, d2.as_ptr() as usize);
    let (c1, c2) = (news_club(i1), news_club(i2));
    let sg = |c: usize| news_g(c + club::SHORT_GENDER_NAME);
    if body == 0 {
        f(NEWS_TEXT_BUFFER, 0xa689d4, sg(c1), sg(c1), sg(c2), sg(c2), c1 + club::SHORT_NAME, c2 + club::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, c1 + club::SHORT_NAME, 0x7d5, c1);
        news_link(out, c2 + club::SHORT_NAME, 0x7d5, c2);
        return 1;
    }
    covered!("gra_1 friendly proposal body");
    let (mut first, mut second, mut stadium_name, mut date_text) = ([0u8; 0x400], [0u8; 0x190], [0u8; 0x12c], [0u8; 0x194]);
    let (p1, p2, ps, pd) = (first.as_mut_ptr() as usize, second.as_mut_ptr() as usize, stadium_name.as_mut_ptr() as usize, date_text.as_mut_ptr() as usize);
    let m = exe::friendly_find_friendly_match(read_usize(0xae2c74), dp, news_club(host));
    exe::date_get_long_date_string(dp, pd, 0x64, 0xff);
    let offer = i1 == host;
    let (lead, other) = if offer { (c1, c2) } else { (c2, c1) };
    let stadium = read_usize(c1 + club::STADIUM);
    let (with_stadium, without) = if offer { (0xa68c38, 0xa68bc0) } else { (0xa68aac, 0xa68a2c) };
    if stadium != 0 {
        sprintf(ps, stadium + 4);
        let gs = news_g(stadium + 0x37);
        f(p1, with_stadium, sg(lead), sg(lead), sg(other), sg(other), gs, gs, 0x64, lead + club::SHORT_NAME, other + club::SHORT_NAME, ps, pd);
    } else {
        f(p1, without, sg(lead), sg(lead), sg(other), sg(other), 0x64, lead + club::SHORT_NAME, other + club::SHORT_NAME, pd);
    }
    let answering = Club(if offer { c2 } else { c1 });
    let joined = match answer {
        3 => {
            f(p2, 0xa68b84, sg(answering.addr()), sg(answering.addr()), answering.addr() + club::SHORT_NAME);
            true
        }
        4 => {
            f(p2, 0xa68b48, sg(answering.addr()), sg(answering.addr()), answering.addr() + club::SHORT_NAME);
            true
        }
        _ => {
            covered!("gra_1 friendly proposal answer");
            news_friendly_answer(c1, c2, answering.addr(), w1 as u32, w2 as u32, m, p2) == 0
        }
    };
    if joined {
        sprintf(NEWS_TEXT_BUFFER, 0x991948, p1, p2); // "%s\n\n%s"
    } else {
        news_strcpy(NEWS_TEXT_BUFFER, p1);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c1 + club::SHORT_NAME, 0x7d5, c1);
    news_link(out, c2 + club::SHORT_NAME, 0x7d5, c2);
    1
}

/// Type 7, the answer to a friendly offer. Slots: +5 and +9 the two clubs, +0xd the club that made the offer,
/// +0x11 and +0x15 the date (words), +0x19 the answer (3 accepted, 4 rejected). The answering club is the second
/// when the first made the offer, otherwise the first.
///
/// Body: the date's long text (`exe::date_get_long_date_string`); accepted 0xa68948 or rejected 0xa688bc with the
/// answering club and the offering club (genders, names) and the date; any other answer leaves the buffer as it was.
/// Links: the first and second clubs' names (0x7d5). Headline: accepted 0xa68880 or rejected 0xa68844 with the
/// answering club's short name, and its link.
/// Original: part of `NewsItem::write_text` (0x77d2f0..0x77d4f3).
pub(crate) unsafe fn news_friendly_answered(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "friendly_answered";
    let f = news_format();
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    let (i1, i2, offering) = (read_u32(news + 5), read_u32(news + 9), read_u32(news + 0xd));
    let answer = read_u8(news + 0x19);
    let answering = Club(news_club(if offering == i1 { i2 } else { i1 }));
    news_assigned_date(dp, read_u16(news + 0x11), read_u16(news + 0x15));
    if body == 0 {
        let g = news_g(answering.addr() + club::SHORT_GENDER_NAME);
        match answer {
            3 => {
                f(NEWS_TEXT_BUFFER, 0xa68880, g, g, answering.addr() + club::SHORT_NAME);
            }
            4 => {
                f(NEWS_TEXT_BUFFER, 0xa68844, g, g, answering.addr() + club::SHORT_NAME);
            }
            _ => {}
        }
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, answering.addr() + club::SHORT_NAME, 0x7d5, answering.addr());
        return 1;
    }
    covered!("gra_1 friendly answered body");
    let mut s = [0u8; 0x194];
    let sp = s.as_mut_ptr() as usize;
    exe::date_get_long_date_string(dp, sp, 0x64, 0xff);
    let o = Club(news_club(offering));
    let (ga, go) = (news_g(answering.addr() + club::GENDER_NAME), news_g(o.addr() + club::GENDER_NAME));
    match answer {
        3 => {
            f(NEWS_TEXT_BUFFER, 0xa68948, ga, ga, go, go, 0x64, answering.addr() + club::NAME, o.addr() + club::NAME, sp);
        }
        4 => {
            f(NEWS_TEXT_BUFFER, 0xa688bc, ga, ga, go, go, 0x64, answering.addr() + club::NAME, o.addr() + club::NAME, sp);
        }
        _ => {}
    }
    news_set(out, NEWS_TEXT_BUFFER);
    let (c1, c2) = (news_club(i1), news_club(i2));
    news_link(out, c1 + club::NAME, 0x7d5, c1);
    news_link(out, c2 + club::NAME, 0x7d5, c2);
    1
}

/// Type 8, the friendly cancelled. Slots: +5 and +9 the two clubs, +0xd and +0x11 the date (words).
///
/// Body 0xa6878c with the clubs' short genders and short names and the date's long text; headline 0xa68734 with the
/// short names. Links: the two clubs' short names (0x7d5).
/// Original: part of `NewsItem::write_text` (0x77d4fc..0x77d625).
pub(crate) unsafe fn news_friendly_cancelled(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "friendly_cancelled";
    let f = news_format();
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    let (c1, c2) = (news_club(read_u32(news + 5)), news_club(read_u32(news + 9)));
    news_assigned_date(dp, read_u16(news + 0xd), read_u16(news + 0x11));
    let (g1, g2) = (news_g(c1 + club::SHORT_GENDER_NAME), news_g(c2 + club::SHORT_GENDER_NAME));
    if body != 0 {
        covered!("gra_1 friendly cancelled body");
        let mut s = [0u8; 0x194];
        let sp = s.as_mut_ptr() as usize;
        exe::date_get_long_date_string(dp, sp, 0x64, 0xff);
        f(NEWS_TEXT_BUFFER, 0xa6878c, g1, g1, g2, g2, 0x64, c1 + club::SHORT_NAME, c2 + club::SHORT_NAME, sp);
    } else {
        f(NEWS_TEXT_BUFFER, 0xa68734, g1, g1, g2, g2, c1 + club::SHORT_NAME, c2 + club::SHORT_NAME);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c1 + club::SHORT_NAME, 0x7d5, c1);
    news_link(out, c2 + club::SHORT_NAME, 0x7d5, c2);
    1
}

/// Type 9, the invitation to a friendly tournament. Slots: +5 the inviting club, +9 the invited club, +0xd the
/// tournament's index (byte), +0x11 the answer (2 none yet, 3 accepted, 4 declined), +0x19 and +0x1d the date (words).
///
/// Body: the date (assigned as in the friendlies) and the tournament on it (`friendly_find_tournament`, 0x5c89a0,
/// thiscall on `G_FRIENDLY`: the index byte and the date). With the tournament: 0xa68698 with the clubs' short names,
/// the tournament's name (+0x31) and its start date's long text (+4); accepted 0xa68658 or declined 0xa68618 with the
/// invited club's short name, joined as "%s\n\n%s"; any other answer by `friendly_tournament_answer` (0x5ca290: the
/// invited club, the tournament, the buffer, 0x190; the first paragraph alone when it returns nonzero). Links: both
/// clubs' short names (0x7d5) and the tournament's name (0x7d1). Without it: 0xa6858c with the news date's long
/// text; accepted or declined with the invited club's full name (and its short gender); no answer yet (2) the first
/// paragraph alone; links of both clubs. Headline 0xa6854c with the inviting club's short name, and its link.
///
/// Defect of the original: without the tournament, an answer other than 2, 3 or 4 joins the first paragraph with a
/// buffer never written (stack leftovers); the copy joins it with an empty text.
/// Original: part of `NewsItem::write_text` (0x77d62e..0x77da1a).
pub(crate) unsafe fn news_tournament_invitation(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "tournament_invitation";
    let f = news_format();
    let sprintf = news_sprintf();
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    let (c1, c2) = (news_club(read_u32(news + 5)), news_club(read_u32(news + 9)));
    let index = read_u8(news + 0xd);
    let answer = read_u8(news + 0x11);
    let sg = |c: usize| news_g(c + club::SHORT_GENDER_NAME);
    if body == 0 {
        exe::date_date_54a110(dp);
        f(NEWS_TEXT_BUFFER, 0xa6854c, sg(c1), sg(c1), c1 + club::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, c1 + club::SHORT_NAME, 0x7d5, c1);
        return 1;
    }
    covered!("gra_1 tournament invitation body");
    news_assigned_date(dp, read_u16(news + 0x19), read_u16(news + 0x1d));
    let tour = exe::friendly_get_tournament(read_usize(0xae2c74), index as u32, dp);
    let (mut first, mut second, mut date_text) = ([0u8; 0x400], [0u8; 0x190], [0u8; 0x194]);
    let (p1, p2, pd) = (first.as_mut_ptr() as usize, second.as_mut_ptr() as usize, date_text.as_mut_ptr() as usize);
    let joined;
    if tour != 0 {
        exe::date_get_long_date_string(tour + 4, pd, 0x64, 0xff);
        let tname = tour + 0x31;
        f(p1, 0xa68698, sg(c1), sg(c1), sg(c2), sg(c2), 0x64, c1 + club::SHORT_NAME, c2 + club::SHORT_NAME, tname, pd);
        joined = match answer {
            3 => {
                f(p2, 0xa68658, sg(c2), sg(c2), c2 + club::SHORT_NAME);
                true
            }
            4 => {
                f(p2, 0xa68618, sg(c2), sg(c2), c2 + club::SHORT_NAME);
                true
            }
            _ => {
                covered!("gra_1 tournament invitation answer");
                exe::friendly_check_tournament_invitation(read_usize(0xae2c74), c2, tour, p2, 0x190) == 0
            }
        };
    } else {
        covered!("gra_1 tournament invitation no tournament");
        exe::date_get_long_date_string(dp, pd, 0x64, 0xff);
        f(p1, 0xa6858c, sg(c1), sg(c1), sg(c2), sg(c2), 0x64, c1 + club::SHORT_NAME, c2 + club::SHORT_NAME, pd);
        joined = match answer {
            3 => {
                f(p2, 0xa68658, sg(c2), sg(c2), c2 + club::NAME);
                true
            }
            4 => {
                f(p2, 0xa68618, sg(c2), sg(c2), c2 + club::NAME);
                true
            }
            2 => false,
            _ => true,
        };
    }
    if joined {
        sprintf(NEWS_TEXT_BUFFER, 0x991948, p1, p2); // "%s\n\n%s"
    } else {
        news_strcpy(NEWS_TEXT_BUFFER, p1);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c1 + club::SHORT_NAME, 0x7d5, c1);
    news_link(out, c2 + club::SHORT_NAME, 0x7d5, c2);
    if tour != 0 {
        news_link(out, tour + 0x31, 0x7d1, tour);
    }
    1
}

/// Type 10, the answer to a friendly tournament invitation. Slots: +5 the inviting club, +9 the invited club, +0xd
/// and +0x11 the date (words), +0x15 the answer (3 accepted, 4 declined), +0x19 the tournament's index (byte).
///
/// Body: the tournament on the date (as in type 9) and the date's text (`Date::get_date_string`, 0x54c3b0, 0x64,
/// 0xff). With the tournament: accepted 0xa6848c or declined 0xa683cc (the invited club, then the inviting club, their
/// genders and names, the tournament's name and the date), links of both clubs' names (0x7d5, invited first) and of
/// the tournament (0x7d1); without it 0xa68328 or 0xa68284 and the two clubs' links. Any other answer leaves the buffer
/// as it was. Headline: accepted 0xa6823c or rejected 0xa681f4 with the invited club's short name, and its link.
/// Original: part of `NewsItem::write_text` (0x77da23..0x77dd7d).
pub(crate) unsafe fn news_tournament_answered(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "tournament_answered";
    let f = news_format();
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    let (c1, c2) = (news_club(read_u32(news + 5)), news_club(read_u32(news + 9)));
    let answer = read_u8(news + 0x15);
    let index = read_u8(news + 0x19);
    news_assigned_date(dp, read_u16(news + 0xd), read_u16(news + 0x11));
    if body == 0 {
        let g = news_g(c2 + club::SHORT_GENDER_NAME);
        match answer {
            3 => {
                f(NEWS_TEXT_BUFFER, 0xa6823c, g, g, c2 + club::SHORT_NAME);
            }
            4 => {
                f(NEWS_TEXT_BUFFER, 0xa681f4, g, g, c2 + club::SHORT_NAME);
            }
            _ => {}
        }
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, c2 + club::SHORT_NAME, 0x7d5, c2);
        return 1;
    }
    covered!("gra_1 tournament answered body");
    let tour = exe::friendly_get_tournament(read_usize(0xae2c74), index as u32, dp);
    let mut s = [0u8; 0x194];
    let sp = s.as_mut_ptr() as usize;
    exe::date_get_date_string_no_weekday(dp, sp, 0x64, 0xff);
    let (g1, g2) = (news_g(c1 + club::GENDER_NAME), news_g(c2 + club::GENDER_NAME));
    if tour != 0 {
        let tname = tour + 0x31;
        match answer {
            3 => {
                f(NEWS_TEXT_BUFFER, 0xa6848c, g2, g2, g1, g1, 0x64, c2 + club::NAME, c1 + club::NAME, tname, sp);
            }
            4 => {
                f(NEWS_TEXT_BUFFER, 0xa683cc, g2, g2, g1, g1, 0x64, c2 + club::NAME, c1 + club::NAME, tname, sp);
            }
            _ => {}
        }
    } else {
        covered!("gra_1 tournament answered no tournament");
        match answer {
            3 => {
                f(NEWS_TEXT_BUFFER, 0xa68328, g2, g2, g1, g1, 0x64, c2 + club::NAME, c1 + club::NAME, sp);
            }
            4 => {
                f(NEWS_TEXT_BUFFER, 0xa68284, g2, g2, g1, g1, 0x64, c2 + club::NAME, c1 + club::NAME, sp);
            }
            _ => {}
        }
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c2 + club::NAME, 0x7d5, c2);
    news_link(out, c1 + club::NAME, 0x7d5, c1);
    if tour != 0 {
        news_link(out, tour + 0x31, 0x7d1, tour);
    }
    1
}

/// Type 11, the friendly tournament cancelled. Slots: +5 the organising club, +9 and +0xd the date (words).
///
/// Body 0xa68174 with the club's short name and the date's long text; headline 0xa68130 with the short name. Link:
/// the club's short name (0x7d5).
/// Original: part of `NewsItem::write_text` (0x77dd86..0x77de79).
pub(crate) unsafe fn news_tournament_cancelled(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "tournament_cancelled";
    let f = news_format();
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    let c = Club(news_club(read_u32(news + 5)));
    news_assigned_date(dp, read_u16(news + 9), read_u16(news + 0xd));
    let g = news_g(c.addr() + club::SHORT_GENDER_NAME);
    if body != 0 {
        covered!("gra_1 tournament cancelled body");
        let mut s = [0u8; 0x194];
        let sp = s.as_mut_ptr() as usize;
        exe::date_get_long_date_string(dp, sp, 0x64, 0xff);
        f(NEWS_TEXT_BUFFER, 0xa68174, g, g, 0x64, c.addr() + club::SHORT_NAME, sp);
    } else {
        f(NEWS_TEXT_BUFFER, 0xa68130, g, g, c.addr() + club::SHORT_NAME);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c.addr() + club::SHORT_NAME, 0x7d5, c.addr());
    1
}

/// Type 12, the coach's report. Slots: +5 a club, +9 the coach (index of the coach table 0xadadfc), +0xd and +0x11
/// two bytes, +0x15 a word, +0x19 a dword. Without the coach's record the news.cpp error box (line 0x61f) and 0;
/// otherwise the coach's virtual text writer (slot +0x48 of its table: the club, the byte +0x11, the byte +0xd, the
/// word, 0, 0, the dword, 0, `body`, `out`) and 1.
/// Original: part of `NewsItem::write_text` (0x77de82..0x77df0b).
pub(crate) unsafe fn news_comp_report(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "comp_report";
    let coach = read_usize(read_usize(0xadadfc).wrapping_add((read_u32(news + 9) as usize).wrapping_mul(4)));
    if coach == 0 {
        news_error_box(0x61f);
        return 0;
    }
    covered!("gra_1 comp report");
    let w: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> u32 =
        core::mem::transmute(read_usize(read_usize(coach) + 0x48));
    w(coach, read_u32(news + 5), read_u8(news + 0x11) as u32, read_u8(news + 0xd) as u32, read_u16(news + 0x15) as u32, 0, 0,
      read_u32(news + 0x19), 0, body, out);
    1
}

/// Type 13, the league's fixture list decided. Slots: +5 the competition, +9 the club, +0xd the season's year (word);
/// the club's first three games at +0x11 + 16·k (k = 0..2): the date's two words (+0x11, +0x15), the opponent
/// (+0x19) and home (+0x1d, word, nonzero at home).
///
/// The season's name: "<year>/<(year+1) % 100>" (0xa680e0) when the key nation of the competition
/// (`exe::key_nation_index_of_comp`) has a season over two years (the dword at 0xb63d77 + nation·0x48), otherwise
/// "<year>" (0xa680a4), with the competition's short name. Headline 0xa67f20 with the season's name and the
/// competition's link (0x7d0). Body: each game "<opponent> (home)" (0xa68078) or "(away)" (0xa6804c) with the
/// opponent's short name, and its date's long text (`Date::Date` assigned, `exe::date_get_long_date_string`, 0x32);
/// then 0xa67f68 with the club's short name, the three games and the three dates; links: the competition's short name
/// (0x7d0), the club's and the three opponents' short names (0x7d5).
/// Original: part of `NewsItem::write_text` (0x77df14..0x77e2e2).
pub(crate) unsafe fn news_fixture_list(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "fixture_list";
    let f = news_format();
    let comp = news_comp(read_u32(news + 5));
    let year = read_u16(news + 0xd) as i32;
    let nation = exe::key_nation_index_of_comp(comp) as i8 as i32;
    let mut season = [0u8; 0xc8];
    let sp = season.as_mut_ptr() as usize;
    let gs = news_g(comp + competition::SHORT_GENDER_NAME);
    if read_u32((0xb63d77usize).wrapping_add((nation as usize).wrapping_mul(0x48))) != 0 {
        f(sp, 0xa680e0, gs, gs, year, (year + 1) % 100, comp + competition::SHORT_NAME);
    } else {
        f(sp, 0xa680a4, gs, gs, year, comp + competition::SHORT_NAME);
    }
    if body == 0 {
        f(NEWS_TEXT_BUFFER, 0xa67f20, gs, gs, sp);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 fixture list body");
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    exe::date_date_54a110(dp);
    let c = news_club(read_u32(news + 9));
    let (mut games, mut dates) = ([0u8; 0x3d * 3], [0u8; 0x32 * 3]);
    let (gp, dtp) = (games.as_mut_ptr() as usize, dates.as_mut_ptr() as usize);
    let mut opp = [0usize; 3];
    for k in 0..3 {
        let base = news + 0x11 + 16 * k;
        let (w1, w2) = (read_u16(base), read_u16(base + 4));
        let o = news_club(read_u32(base + 8));
        let home = read_u16(base + 0xc) != 0;
        opp[k] = o;
        let g = news_g(o + club::SHORT_GENDER_NAME);
        f(gp + 0x3d * k, if home { 0xa68078 } else { 0xa6804c }, g, g, o + club::SHORT_NAME);
        let (mut d2, mut tmp) = ([0u8; 8], [0u8; 8]);
        news_date(d2.as_mut_ptr() as usize, w1, w2);
        exe::date_operator_3d(dp, tmp.as_mut_ptr() as usize, d2.as_ptr() as usize);
        exe::date_get_long_date_string(dp, dtp + 0x32 * k, 0x32, 0xff);
    }
    let sg = |x: usize| news_g(x + club::SHORT_GENDER_NAME);
    f(NEWS_TEXT_BUFFER, 0xa67f68, sg(c), sg(c), sg(opp[0]), sg(opp[1]), sg(opp[2]), 0x64, 0x64, 0x64, c + club::SHORT_NAME, gp,
      gp + 0x3d, gp + 0x7a, dtp, dtp + 0x32, dtp + 0x64);
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
    news_link(out, c + club::SHORT_NAME, 0x7d5, c);
    for o in opp {
        news_link(out, o + club::SHORT_NAME, 0x7d5, o);
    }
    1
}

/// Type 16, the coach's match report. Slots: +5 a club, +9 the coach (index of the coach table 0xadadfc), +0xd and
/// +0x11 two bytes, +0x15 a dword, and the fixture from slot 5 on (`comp_util_read_fixture_from_news_slots`, 0x4b0010,
/// cdecl: the news, the fixture, the slot cursor). Without the coach's record 0 (no box); otherwise the coach's virtual
/// text writer (slot +0x48: the club, the byte +0xd, the fixture's dwords +0x42 and +0x32, the fixture, the word of
/// the slot after the fixture, the dword +0x15, the byte +0x11, `body`, `out`) and 1.
/// Original: part of `NewsItem::write_text` (0x77e2eb..0x77e37d).
pub(crate) unsafe fn news_comp_match_report(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "comp_match_report";
    let (b1, b2, d, c) = (read_u8(news + 0xd), read_u8(news + 0x11), read_u32(news + 0x15), read_u32(news + 5));
    let mut fixture = [0u8; 0xc8];
    let fp = fixture.as_mut_ptr() as usize;
    let mut k: u8 = 5;
    exe::comp_util_read_fixture_from_news_slots(news, fp, &mut k);
    let coach = read_usize(read_usize(0xadadfc).wrapping_add((read_u32(news + 9) as usize).wrapping_mul(4)));
    if coach == 0 {
        return 0;
    }
    covered!("gra_1 comp match report");
    let slot = read_u16(news.wrapping_add(5).wrapping_add((k as i8 as isize as usize).wrapping_mul(4))) as u32;
    let w: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize) -> u32 =
        core::mem::transmute(read_usize(read_usize(coach) + 0x48));
    w(coach, c, b1 as u32, read_u32(fp + 0x42), read_u32(fp + 0x32), fp, slot, d, b2 as u32, body, out);
    1
}

/// Types 17 and 18, a play-off place won by the form over a third or a quarter of the season. Slots: +5 the club,
/// +9 the part (1, 2, 3...), +0xd the competition.
///
/// Body: by the part, the third (17: 0xa67e88 first, 0xa67df0 middle, 0xa67d58 last) or the quarter (18: 0xa67c98,
/// 0xa67bfc, 0xa67b64, 0xa67acc), with the club's and the competition's genders and names; links: the club's name
/// (0x7d5) and the competition's (0x7d0). Headline (both) 0xa67d30 with the club's short name, and its link.
/// Original: part of `NewsItem::write_text` (0x77e386..0x77e56f).
pub(crate) unsafe fn news_playoff_place(news: usize, body: u32, out: usize, quarters: bool) -> u32 {
    NEWS_LAST_PIECE = if quarters { "playoff_place_quarter" } else { "playoff_place_third" };
    let f = news_format();
    let c = Club(news_club(read_u32(news + 5)));
    let part = read_u8(news + 9);
    let comp = news_comp(read_u32(news + 0xd));
    if body == 0 {
        let g = news_g(c.addr() + club::SHORT_GENDER_NAME);
        f(NEWS_TEXT_BUFFER, 0xa67d30, g, g, c.addr() + club::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, c.addr() + club::SHORT_NAME, 0x7d5, c.addr());
        return 1;
    }
    covered!("gra_1 playoff place body");
    let fmt = if quarters {
        match part {
            1 => 0xa67c98,
            2 => 0xa67bfc,
            3 => 0xa67b64,
            _ => 0xa67acc,
        }
    } else {
        match part {
            1 => 0xa67e88,
            2 => 0xa67df0,
            _ => 0xa67d58,
        }
    };
    let (gc, gk) = (news_g(c.addr() + club::GENDER_NAME), news_g(comp + competition::GENDER_NAME));
    f(NEWS_TEXT_BUFFER, fmt, gc, gc, gk, gk, c.addr() + club::NAME, comp + competition::NAME);
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c.addr() + club::NAME, 0x7d5, c.addr());
    news_link(out, comp + competition::NAME, 0x7d0, comp);
    1
}

/// Type 19, the nation's leagues reshuffled. Slots: +5 the club, +9 the competition. Without the club's stadium, the
/// stadium's city (+0x38), the competition or its nation (`competition::NATION`), the text "League reshuffled error
/// message" alone.
///
/// Body 0xa67a2c with the club's name, the competition's short name, the city's name and the nation's name (and their
/// genders, `city::GENDER_NAME` for the city); links: the club (0x7d5), the competition's short name (0x7d0) and the
/// nation's name to its national team (0x7d5, `exe::database_get_national_team`). Headline 0xa67a00 with the nation's
/// short name, and its link.
/// Original: part of `NewsItem::write_text` (0x77e578..0x77e6f4).
pub(crate) unsafe fn news_leagues_reshuffled(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "leagues_reshuffled";
    let f = news_format();
    let c = Club(news_club(read_u32(news + 5)));
    let comp = news_comp(read_u32(news + 9));
    let stadium = if !c.is_null() { c.stadium().addr() } else { 0 };
    let city = if stadium != 0 { read_usize(stadium + 0x38) } else { 0 };
    let nation = if city != 0 && comp != 0 { read_usize(comp + competition::NATION) } else { 0 };
    if nation == 0 {
        news_set(out, 0xa679e0); // "League reshuffled error message"
        return 1;
    }
    if body == 0 {
        let g = news_g(nation + 0x52);
        f(NEWS_TEXT_BUFFER, 0xa67a00, g, g, nation + 0x38);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, nation + 0x38, 0x7d5, exe::database_get_national_team(nation));
        return 1;
    }
    covered!("gra_1 leagues reshuffled body");
    let (gc, gk, gt, gn) = (news_g(c.addr() + club::GENDER_NAME), news_g(comp + competition::SHORT_GENDER_NAME), news_g(city + city::GENDER_NAME), news_g(nation + 0x37));
    f(NEWS_TEXT_BUFFER, 0xa67a2c, gc, gc, gk, gk, gt, gt, gn, gn, c.addr() + club::NAME, comp + competition::SHORT_NAME, city + 4, nation + 4);
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c.addr() + club::NAME, 0x7d5, c.addr());
    news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
    news_link(out, nation + 4, 0x7d5, exe::database_get_national_team(nation));
    1
}

/// Type 20, the cup draw. Slots: +5 the competition, +9 and +0xd the stage and round (words), +0x11 the drawn club,
/// +0x15 its opponent, +0x19 nonzero for two legs, +0x1d the stadium (−1 unknown), +0x21 and +0x25 the date (words).
///
/// Body: 0xa67958 with the drawn club and the opponent (short names and genders) and the round's label (form 2), its
/// text and links (the two clubs 0x7d5, the competition on the label 0x7d0); then a second paragraph appended
/// (`exe::text_append_string`): with the stadium, its name — with the city's name when it differs and is not
/// "Unknown": "%s (%s)" (0x9a3d8c), or "%s, %s" (0x98f6ac) in languages 0 and 1; the city's name alone when they are
/// equal or the stadium is "Unknown"; the stadium's when it has no city — and 0xa678cc (first leg) or 0xa67854 (the
/// game) with the date's long text; without the stadium 0xa677f8 or 0xa677a8 with the date. Headline 0xa6776c with the
/// round's label (form 3), and its link.
/// Original: part of `NewsItem::write_text` (0x77e6fd..0x77ed30).
pub(crate) unsafe fn news_cup_draw(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "cup_draw";
    let f = news_format();
    let sprintf = news_sprintf();
    let comp = news_comp(read_u32(news + 5));
    let (stage, round) = (read_u16(news + 9), read_u16(news + 0xd));
    let mut label = [0u8; 0xc8];
    let lb = label.as_ptr() as usize;
    if body == 0 {
        let lg = news_round_label(comp, stage, round, 3, &mut label);
        f(NEWS_TEXT_BUFFER, 0xa6776c, lg, lg, lb);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, lb, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 cup draw body");
    let (drawn, opponent) = (news_club(read_u32(news + 0x11)), news_club(read_u32(news + 0x15)));
    let two_legs = read_u32(news + 0x19) != 0;
    let stadium = read_i32(news + 0x1d);
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    news_date(dp, read_u16(news + 0x21), read_u16(news + 0x25));
    let lg = news_round_label(comp, stage, round, 2, &mut label);
    let (ga, gb) = (news_g(drawn + club::SHORT_GENDER_NAME), news_g(opponent + club::SHORT_GENDER_NAME));
    f(NEWS_TEXT_BUFFER, 0xa67958, ga, ga, gb, gb, lg, lg, drawn + club::SHORT_NAME, opponent + club::SHORT_NAME, lb);
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, drawn + club::SHORT_NAME, 0x7d5, drawn);
    news_link(out, opponent + club::SHORT_NAME, 0x7d5, opponent);
    news_link(out, lb, 0x7d0, comp);
    let mut s = [0u8; 0xc8];
    let sp = s.as_mut_ptr() as usize;
    exe::date_get_long_date_string(dp, sp, 0x64, 0xff);
    if stadium != -1 {
        covered!("gra_1 cup draw stadium");
        let pair = if exe::langlib_get_language() == 0 || exe::langlib_get_language() == 1 { 0x98f6ac } else { 0x9a3d8c };
        let st = read_usize(G_STADIUM_TABLE).wrapping_add((stadium as usize).wrapping_mul(stadium::SIZE));
        let city = read_usize(st + stadium::CITY);
        let mut place = [0u8; 0x138];
        let pp = place.as_mut_ptr() as usize;
        if city != 0 {
            if !news_streq(st + stadium::NAME, city + 4) && !news_streq(st + stadium::NAME, 0xa67950) {
                sprintf(pp, pair, st + stadium::NAME, city + 4); // "%s (%s)" or "%s, %s"
            } else {
                news_strcpy(pp, city + 4); // the same name, or "Unknown"
            }
        } else {
            news_strcpy(pp, st + stadium::NAME);
        }
        let g = news_g(st + stadium::GENDER_NAME);
        f(NEWS_TEXT_BUFFER, if two_legs { 0xa678cc } else { 0xa67854 }, g, g, 0x64, pp, sp);
    } else {
        f(NEWS_TEXT_BUFFER, if two_legs { 0xa677f8 } else { 0xa677a8 }, 0x64, sp);
    }
    exe::text_append_string(out, NEWS_TEXT_BUFFER);
    1
}

/// Type 21, the group draw. Slots: +5 the competition, +9 the stage (word), +0xd the club, +0x11 and +0x15 the date
/// of the first group game (words), +0x19 the number of the other clubs in the group (byte), +0x1d on their ids (one
/// dword each).
///
/// Body: more than 8 clubs gives the news.cpp error box (line 0x942) and 8. The list of the clubs' short names "A and
/// B", "A, B and C"... up to 8 (0xa673cc, 0xa67360, 0xa672d4, 0xa6721c, 0xa67148, 0xa67054, 0xa66f40; the genders in
/// pairs, then the names); the stage's name (`exe::comp_util_get_stage_name`, 0x64, the stage, 0); 0xa66e6c with the
/// club, the stage, the competition and the list, and its text; with the date's second word set, 0xa66e20 with the
/// date's long text appended. Links: the club (0x7d5), the competition (0x7d0), the stage's name (0x7e5, to ((stage &
/// 0x7ff) | competition << 11) << 11) and the listed clubs (0x7d5). Headline: by the stage's parent
/// (`exe::comp_util_get_parent_stage_id`): 0x41c 0xa66de0, 10 0xa66d94, 0x14 0xa66d48, 0x82 0xa66cfc, 0x449 0xa66cb4,
/// 0x44a 0xa66c6c, others no text; with the competition's short name, and its link.
///
/// Defect of the original: with fewer than 2 other clubs the list buffer is never written (stack leftovers go into
/// the text); the copy uses an empty list.
/// Original: part of `NewsItem::write_text` (0x77f3ed..0x77fd91).
pub(crate) unsafe fn news_group_draw(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "group_draw";
    let f = news_format();
    let ci = read_u32(news + 5);
    let comp = news_comp(ci);
    let stage = read_u16(news + 9);
    if body == 0 {
        let fmt = match exe::comp_util_get_parent_stage_id(stage as i16 as i32) {
            0x41c => 0xa66de0,
            0xa => 0xa66d94,
            0x14 => 0xa66d48,
            0x82 => 0xa66cfc,
            0x449 => 0xa66cb4,
            0x44a => 0xa66c6c,
            _ => 0,
        };
        if fmt != 0 {
            let g = news_g(comp + competition::SHORT_GENDER_NAME);
            f(NEWS_TEXT_BUFFER, fmt, g, g, comp + competition::SHORT_NAME);
        }
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 group draw body");
    let c = Club(news_club(read_u32(news + 0xd)));
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    news_date(dp, read_u16(news + 0x11), read_u16(news + 0x15));
    let mut n = read_u8(news + 0x19) as i8;
    if n > 8 {
        news_error_box_keep(0x942);
        n = 8;
        write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
    }
    let mut ids = [0u32; 10];
    let mut k = 0i8;
    while k < n && k < 10 {
        ids[k as usize] = read_u32(news + 0x1d + 4 * k as usize);
        k += 1;
    }
    let mut list = [0u8; 0x190];
    let lp = list.as_mut_ptr() as usize;
    let cl: [usize; 8] = core::array::from_fn(|i| news_club(ids[i]));
    let sg = |i: usize| news_g(cl[i] + club::SHORT_GENDER_NAME);
    let sn = |i: usize| cl[i] + club::SHORT_NAME;
    match n {
        2 => {
            f(lp, 0xa673cc, sg(0), sg(0), sg(1), sg(1), sn(0), sn(1));
        }
        3 => {
            f(lp, 0xa67360, sg(0), sg(0), sg(1), sg(1), sg(2), sg(2), sn(0), sn(1), sn(2));
        }
        4 => {
            f(lp, 0xa672d4, sg(0), sg(0), sg(1), sg(1), sg(2), sg(2), sg(3), sg(3), sn(0), sn(1), sn(2), sn(3));
        }
        5 => {
            f(lp, 0xa6721c, sg(0), sg(0), sg(1), sg(1), sg(2), sg(2), sg(3), sg(3), sg(4), sg(4), sn(0), sn(1), sn(2), sn(3), sn(4));
        }
        6 => {
            f(lp, 0xa67148, sg(0), sg(0), sg(1), sg(1), sg(2), sg(2), sg(3), sg(3), sg(4), sg(4), sg(5), sg(5), sn(0), sn(1), sn(2), sn(3),
              sn(4), sn(5));
        }
        7 => {
            f(lp, 0xa67054, sg(0), sg(0), sg(1), sg(1), sg(2), sg(2), sg(3), sg(3), sg(4), sg(4), sg(5), sg(5), sg(6), sg(6), sn(0), sn(1),
              sn(2), sn(3), sn(4), sn(5), sn(6));
        }
        8 => {
            f(lp, 0xa66f40, sg(0), sg(0), sg(1), sg(1), sg(2), sg(2), sg(3), sg(3), sg(4), sg(4), sg(5), sg(5), sg(6), sg(6), sg(7), sg(7),
              sn(0), sn(1), sn(2), sn(3), sn(4), sn(5), sn(6), sn(7));
        }
        _ => {}
    }
    let mut stage_name = [0u8; 0xc8];
    let sp = stage_name.as_mut_ptr() as usize;
    exe::comp_util_get_stage_name(sp as *mut u8, 0x64, stage as u32, 0);
    let (gc, gk) = (news_g(c.addr() + club::SHORT_GENDER_NAME), news_g(comp + competition::SHORT_GENDER_NAME));
    f(NEWS_TEXT_BUFFER, 0xa66e6c, gc, gc, 0x64, gk, gk, c.addr() + club::SHORT_NAME, sp, comp + competition::SHORT_NAME, lp);
    news_set(out, NEWS_TEXT_BUFFER);
    if read_u16(dp + 2) != 0 {
        covered!("gra_1 group draw start date");
        let mut s = [0u8; 0xc8];
        exe::date_get_long_date_string(dp, s.as_mut_ptr() as usize, 0x64, 0xff);
        f(NEWS_TEXT_BUFFER, 0xa66e20, 0x64, s.as_ptr() as usize);
        exe::text_append_string(out, NEWS_TEXT_BUFFER);
    }
    news_link(out, c.addr() + club::SHORT_NAME, 0x7d5, c.addr());
    news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
    news_link(out, sp, 0x7e5, ((((stage as u32) & 0x7ff) | (ci << 11)) << 11) as usize);
    for i in 0..n.max(0) as usize {
        let x = Club(news_club(ids[i]));
        news_link(out, x.addr() + club::SHORT_NAME, 0x7d5, x.addr());
    }
    1
}

/// Type 22, the invitation to apply for the Intertoto Cup. Slots: +5 nonzero when answered, +9 the club, +0x11
/// nonzero when it applied.
///
/// Body 0xa66bf4 with the club's short name and the Intertoto Cup's name (`KEY_CLUB_COMP_INTER_TOTO_CUP`); when
/// answered, "The application has been made." (0xa66bd0) or "The invitation has been declined." (0xa66bac) appended;
/// links: the club's short name (0x7d5) and the cup's name (0x7d0). Headline 0xa66b80 with the cup's short name, and
/// its link.
/// Original: part of `NewsItem::write_text` (0x77fd9a..0x77feb5).
pub(crate) unsafe fn news_intertoto_invitation(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "intertoto_invitation";
    let f = news_format();
    let comp = news_comp(read_u32(KEY_CLUB_COMP_INTER_TOTO_CUP));
    let c = Club(news_club(read_u32(news + 9)));
    if body == 0 {
        let g = news_g(comp + competition::SHORT_GENDER_NAME);
        f(NEWS_TEXT_BUFFER, 0xa66b80, g, g, comp + competition::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 intertoto invitation body");
    let (gc, gk) = (news_g(c.addr() + club::SHORT_GENDER_NAME), news_g(comp + competition::GENDER_NAME));
    f(NEWS_TEXT_BUFFER, 0xa66bf4, gc, gc, gk, gk, c.addr() + club::SHORT_NAME, comp + competition::NAME);
    news_set(out, NEWS_TEXT_BUFFER);
    if read_u32(news + 5) != 0 {
        f(NEWS_TEXT_BUFFER, if read_u32(news + 0x11) != 0 { 0xa66bd0 } else { 0xa66bac });
        exe::text_append_string(out, NEWS_TEXT_BUFFER);
    }
    news_link(out, c.addr() + club::SHORT_NAME, 0x7d5, c.addr());
    news_link(out, comp + competition::NAME, 0x7d0, comp);
    1
}

/// Type 23, the club chosen for the Intertoto Cup. Slot: +5 the club.
///
/// Body 0xa66af0 with the club's short name and the Intertoto Cup's name; headline 0xa66aa8 with both short names.
/// Links: the club's short name (0x7d5) and the cup's name or short name (0x7d0).
/// Original: part of `NewsItem::write_text` (0x77febe..0x77ffb6).
pub(crate) unsafe fn news_intertoto_chosen(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "intertoto_chosen";
    let f = news_format();
    let comp = news_comp(read_u32(KEY_CLUB_COMP_INTER_TOTO_CUP));
    let c = Club(news_club(read_u32(news + 5)));
    let gc = news_g(c.addr() + club::SHORT_GENDER_NAME);
    let name = if body != 0 {
        let gk = news_g(comp + competition::GENDER_NAME);
        f(NEWS_TEXT_BUFFER, 0xa66af0, gc, gc, gk, gk, c.addr() + club::SHORT_NAME, comp + competition::NAME);
        comp + competition::NAME
    } else {
        let gk = news_g(comp + competition::SHORT_GENDER_NAME);
        f(NEWS_TEXT_BUFFER, 0xa66aa8, gc, gc, gk, gk, c.addr() + club::SHORT_NAME, comp + competition::SHORT_NAME);
        comp + competition::SHORT_NAME
    };
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c.addr() + club::SHORT_NAME, 0x7d5, c.addr());
    news_link(out, name, 0x7d0, comp);
    1
}

/// Type 40, the FA's offer of a bye to the club playing the World Club Championship. Slot: +5 the answer (2
/// accepted, 3 rejected).
///
/// Body 0xa669ac with the club's (0x9d0240), the FA Cup's (`KEY_CLUB_COMP_ENGLISH_FA_CUP`) and the World Club
/// Championship's (`KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP`) short names; accepted 0xa66984 or rejected 0xa66934
/// appended; links: the club (0x7d5) and the two competitions (0x7d0). Headline 0xa668f4 with the club's short name, and
/// its link.
/// Original: part of `NewsItem::write_text` (0x780039..0x7801e9).
pub(crate) unsafe fn news_world_club_bye(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "world_club_bye";
    let f = news_format();
    let c = Club(news_club(read_u32(NEWS_KEY_CLUB_WORLD_CLUB_BYE)));
    let gc = news_g(c.addr() + club::SHORT_GENDER_NAME);
    if body == 0 {
        f(NEWS_TEXT_BUFFER, 0xa668f4, gc, gc, c.addr() + club::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, c.addr() + club::SHORT_NAME, 0x7d5, c.addr());
        return 1;
    }
    covered!("gra_1 world club bye body");
    let fa = news_comp(read_u32(KEY_CLUB_COMP_ENGLISH_FA_CUP));
    let wcc = news_comp(read_u32(KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP));
    let (gf, gw) = (news_g(fa + competition::SHORT_GENDER_NAME), news_g(wcc + competition::SHORT_GENDER_NAME));
    f(NEWS_TEXT_BUFFER, 0xa669ac, gc, gc, gf, gf, gw, gw, c.addr() + club::SHORT_NAME, fa + competition::SHORT_NAME, wcc + competition::SHORT_NAME);
    news_set(out, NEWS_TEXT_BUFFER);
    if read_u32(news + 5) == 2 {
        f(NEWS_TEXT_BUFFER, 0xa66984);
        exe::text_append_string(out, NEWS_TEXT_BUFFER);
    }
    if read_u32(news + 5) == 3 {
        f(NEWS_TEXT_BUFFER, 0xa66934);
        exe::text_append_string(out, NEWS_TEXT_BUFFER);
    }
    news_link(out, c.addr() + club::SHORT_NAME, 0x7d5, c.addr());
    news_link(out, fa + competition::SHORT_NAME, 0x7d0, fa);
    news_link(out, wcc + competition::SHORT_NAME, 0x7d0, wcc);
    1
}

/// Type 38, the cup draw with ties still to be played. Slots: +5 the competition, +9 and +0xd the stage and round
/// (words), +0x11 and +0x15 the club (or the two clubs of the tie) on one side, +0x19 and +0x1d those of the other
/// side (+0x15 and +0x1d −1 when the side is a single club).
///
/// Body: the round's label (form 2) and its gender; "A or B will play either C or D" 0xa676a0, "A or B will play C"
/// 0xa675f8, or (+0x15 = −1) "A will play either C or D" 0xa6754c, with the clubs' short names and genders and the
/// label; links: the clubs +0x11 and +0x19, +0x15 and +0x1d when set (0x7d5), the competition on the label (0x7d0).
/// Headline 0xa6776c with the round's label (form 3), and its link.
/// Original: part of `NewsItem::write_text` (0x77ed39..0x77f20c).
pub(crate) unsafe fn news_cup_draw_open(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "cup_draw_open";
    let f = news_format();
    let comp = news_comp(read_u32(news + 5));
    let (stage, round) = (read_u16(news + 9), read_u16(news + 0xd));
    let mut label = [0u8; 0xc8];
    let lb = label.as_ptr() as usize;
    if body == 0 {
        let lg = news_round_label(comp, stage, round, 3, &mut label);
        f(NEWS_TEXT_BUFFER, 0xa6776c, lg, lg, lb);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, lb, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 cup draw open body");
    let (a, b, c, d) = (read_u32(news + 0x11), read_i32(news + 0x15), read_u32(news + 0x19), read_i32(news + 0x1d));
    let lg = news_round_label(comp, stage, round, 2, &mut label);
    let sg = |x: usize| news_g(x + club::SHORT_GENDER_NAME);
    let (ca, cc) = (news_club(a), news_club(c));
    if b != -1 {
        let cb = news_club(b as u32);
        if d != -1 {
            let cd = news_club(d as u32);
            f(NEWS_TEXT_BUFFER, 0xa676a0, sg(ca), sg(ca), sg(cb), sg(cb), sg(cc), sg(cc), sg(cd), sg(cd), lg, lg, ca + club::SHORT_NAME,
              cb + club::SHORT_NAME, cc + club::SHORT_NAME, cd + club::SHORT_NAME, lb);
        } else {
            f(NEWS_TEXT_BUFFER, 0xa675f8, sg(ca), sg(ca), sg(cb), sg(cb), sg(cc), sg(cc), lg, lg, ca + club::SHORT_NAME, cb + club::SHORT_NAME,
              cc + club::SHORT_NAME, lb);
        }
    } else {
        let cd = news_club(d as u32);
        f(NEWS_TEXT_BUFFER, 0xa6754c, sg(ca), sg(ca), sg(cc), sg(cc), sg(cd), sg(cd), lg, lg, ca + club::SHORT_NAME, cc + club::SHORT_NAME,
          cd + club::SHORT_NAME, lb);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, ca + club::SHORT_NAME, 0x7d5, ca);
    news_link(out, cc + club::SHORT_NAME, 0x7d5, cc);
    if b != -1 {
        let cb = news_club(b as u32);
        news_link(out, cb + club::SHORT_NAME, 0x7d5, cb);
    }
    if d != -1 {
        let cd = news_club(d as u32);
        news_link(out, cd + club::SHORT_NAME, 0x7d5, cd);
    }
    news_link(out, lb, 0x7d0, comp);
    1
}

/// Type 39, the cup draw made today. Slots: +5 the competition, +9 and +0xd the stage and round (words), +0x11 the
/// club, +0x21 nonzero for the general story.
///
/// Body: the round's label (form 1, without the lower-casing of the other types) and its gender; the general story
/// 0xa674f4 with the label, or the club's 0xa67458 with its short name, and the club's link (0x7d5) in that case; the
/// competition's link on the label (0x7d0). Headline 0xa67414 with the label (form 2), and its link.
/// Original: part of `NewsItem::write_text` (0x77f215..0x77f3e4).
pub(crate) unsafe fn news_cup_draw_today(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "cup_draw_today";
    let f = news_format();
    let comp = news_comp(read_u32(news + 5));
    let (stage, round) = (read_u16(news + 9) as u32, read_u16(news + 0xd) as u32);
    let c = Club(news_club(read_u32(news + 0x11)));
    let general = read_u32(news + 0x21) != 0;
    let mut label = [0u8; 0xc8];
    let lb = label.as_ptr() as usize;
    let form = if body != 0 { 1 } else { 2 };
    exe::comp_util_format_comp_stage_round_label(comp, stage, round, form, label.as_mut_ptr(), 0xc8);
    let lg = exe::comp_util_get_label_gender(comp as u32, stage, round, form) as i8 as i32;
    if body == 0 {
        f(NEWS_TEXT_BUFFER, 0xa67414, lg, lg, lb);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, lb, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 cup draw today body");
    if general {
        f(NEWS_TEXT_BUFFER, 0xa674f4, lg, lg, lb);
    } else {
        let g = news_g(c.addr() + club::SHORT_GENDER_NAME);
        f(NEWS_TEXT_BUFFER, 0xa67458, g, g, lg, lg, c.addr() + club::SHORT_NAME, lb);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    if !general {
        news_link(out, c.addr() + club::SHORT_NAME, 0x7d5, c.addr());
    }
    news_link(out, lb, 0x7d0, comp);
    1
}

/// Category 7, type 0x3e9, a competition record. Slot: +5 the competition.
///
/// Body: the competition's object (`G_COMP_OBJECT_TABLE`) and its records object (+8) write it (`comp_records_write
/// _news_text`, 0x4a7f50, thiscall on the records object: the news and `out`); without them the news.cpp error box
/// (line 0xb51) and 0. Headline 0xa668cc "New {competition} record" with the competition's short name, and its link
/// (0x7d0).
/// Original: part of `NewsItem::write_text` (0x78022b..0x7802cb).
pub(crate) unsafe fn news_club_record(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "comp_record";
    let ci = read_u32(news + 5);
    if body != 0 {
        let obj = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((ci as usize).wrapping_mul(4)));
        let records = if obj != 0 { read_usize(obj + 8) } else { 0 };
        if records == 0 {
            news_error_box(0xb51);
            return 0;
        }
        covered!("gra_1 comp record body");
        exe::comp_stats_write_record_news_text(records, news, out);
        return 1;
    }
    let comp = news_comp(ci);
    let g = news_g(comp + competition::SHORT_GENDER_NAME);
    news_format()(NEWS_TEXT_BUFFER, 0xa668cc, g, g, comp + competition::SHORT_NAME);
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
    1
}

/// Category 3, type 0x7d0, the injury or illness news. Slots: +5 the player, +9..+0x11 his name ids, +0x15 the injury
/// type, +0x19/+0x1d the day it happened and +0x25/+0x29 the date compared with the name ids (words), +0x21 the days
/// out, +0x2d the club he was injured against (−1 in training), +0x31 the other club of that match, +0x35 the
/// competition, +0x39 the nation where an illness was caught, +0x3d the player's club (−1 none), +0x41 0x420e9 when
/// the match record that follows (from slot 0x10, `exe::read_match_record_from_news`) is the match of the injury.
///
/// Headline (the short name, style 5): by the type's category (`exe::injury_get_type_category`): treatments (12) by
/// the type (0x4d..0x51: faith healing, physiotherapy, surgery, radiotherapy, leave of absence; others surgery);
/// illnesses (0) "struck down with" the illness's name; injuries over 100 days "injury shocker" / "Injury blow" / "out
/// for" the duration (`format_duration_text`) by (type + player) % 3, the others "injured" / "suffers injury" / "out
/// for"; the player's link (0x7d8) when his name ids still match.
///
/// Body (the full name, style 1; the injury's name and the duration first): an illness 0xa6655c (caught at the club in
/// the nation) or 0xa664e4, with the links of the player (when the ids match), the nation (to its national team) and the
/// club. An injury in a match: in Swedish, for types 0x3b and 0x47, a text of its own (`sprintf` 0xa6648c); with the
/// match record, by the score (a win 0xa661e4, a draw 0xa66084, a defeat 0xa65f24: the round's label in form 1 and
/// "%d-%d" of the higher and lower goals); without it 0xa6633c; the competition's gender is 0x64 for the friendly
/// competition (`KEY_CLUB_COMP_FRIENDLY`). Links: the competition (when not a reserve or minor one,
/// `exe::comp_is_reserve_or_minor_comp`), the player (always), the two clubs, and with the record the match on the score
/// (0x7d3, when `exe::match_stats_fixture_entry_matches`) and the record's away club's name. An injury in training: in
/// Swedish, for types 0x3b and 0x47, 0xa65edc or 0xa65e94 by the weekday name's last letter; for another club's player
/// (not the human manager's club, `G_SCREEN_RECORDS_MANAGER`), treatments with the club's short name (a recurring
/// injury, `Injury::get_recurring_injury_type`, names the illness treated: 0xa65908, 0xa659d4, 0xa65aa4, 0xa65b40;
/// otherwise 0xa65d58, 0xa65ca8, 0xa65c10, 0xa65aa4, 0xa65e04) and injuries 0xa65814; for the human manager's club or no
/// club, treatments 0xa656f4, 0xa65664, 0xa65574, 0xa655ec, 0xa65780 and injuries 0xa654a4; links: the player and his
/// club.
/// Original: part of `NewsItem::write_text` (0x7802e7..0x78164b).
pub(crate) unsafe fn news_match_report(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "injury";
    let f = news_format();
    let sprintf = news_sprintf();
    let mut rec = [0u8; 0x20];
    let rp = rec.as_mut_ptr() as usize;
    exe::date_date_54a110(rp);
    let player = read_u32(news + 5);
    let ids = [read_u32(news + 9), read_u32(news + 0xd), read_u32(news + 0x11)];
    let kind = read_u8(news + 0x15);
    let (mut d1, mut d2) = ([0u8; 8], [0u8; 8]);
    let (d1p, d2p) = (d1.as_mut_ptr() as usize, d2.as_mut_ptr() as usize);
    news_date(d1p, read_u16(news + 0x19), read_u16(news + 0x1d));
    let days = read_i32(news + 0x21);
    news_date(d2p, read_u16(news + 0x25), read_u16(news + 0x29));
    let against = read_i32(news + 0x2d);
    let other = read_u32(news + 0x31);
    let comp_i = read_u32(news + 0x35);
    let nation_i = read_i32(news + 0x39);
    let club_i = read_i32(news + 0x3d);
    let in_match = read_u32(news + 0x41) != 0x420e9;
    let mut cursor: u8 = 0x10;
    exe::read_match_record_from_news(news, &mut cursor, rec.as_mut_ptr());
    let mut label = [0u8; 0xc8];
    label[0] = read_u8(0xad9c64);
    let (ga, gb) = (rec[0x19] as i8, rec[0x1a] as i8);
    let mut score = [0u8; 0xc];
    let scp = score.as_mut_ptr() as usize;
    sprintf(scp, 0x98c114, ga.max(gb) as i32, ga.min(gb) as i32); // "%d-%d"
    let human = read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300)));
    let job = if human != 0 { read_usize(human + 0x39) } else { 0 };
    let mine = if job != 0 { club_i == -1 || club_i == read_i32(job) } else { club_i == -1 };
    let mut name = [0u8; 0x12c];
    let np = name.as_mut_ptr() as usize;
    let staff = news_staff(player);
    let (mut ill, mut dur) = ([0u8; 0xc8], [0u8; 0xc8]);
    let (ip, dp) = (ill.as_mut_ptr() as usize, dur.as_mut_ptr() as usize);
    let duration: unsafe extern "C" fn(i32, usize, u32, u32) -> i32 = core::mem::transmute(FN_FORMAT_DURATION_TEXT);
    let category = || exe::injury_get_type_category(read_usize(G_INJURY), kind as i8);
    if body == 0 {
        exe::database_format_name_from_ids(ids[0], ids[1], ids[2], name.as_mut_ptr(), 0x69, 5, player);
        let cat = category();
        if cat == 0xc {
            let fmt = match (kind as i8 as i32).wrapping_sub(0x4d) {
                0 => 0xa6684c, // faith healing
                1 => 0xa66810, // physiotherapy
                3 => 0xa667c8, // radiotherapy
                4 => 0xa66894, // leave of absence
                _ => 0xa66790, // surgery
            };
            f(NEWS_TEXT_BUFFER, fmt, np);
        } else if cat == 0 {
            news_injury_name(kind, ip);
            f(NEWS_TEXT_BUFFER, 0xa66740, np, ip);
        } else {
            let r = (kind as i8 as i32).wrapping_add(player as i32).wrapping_rem(3);
            let fmt = match (days > 100, r) {
                (true, 0) => 0xa6668c,
                (true, 1) => 0xa666c0,
                (false, 0) => 0xa6662c,
                (false, 1) => 0xa66658,
                _ => 0,
            };
            if fmt != 0 {
                f(NEWS_TEXT_BUFFER, fmt, np);
            } else {
                duration(days, dp, 0x32, 1);
                f(NEWS_TEXT_BUFFER, 0xa666f4, np, dp);
            }
        }
        news_set(out, NEWS_TEXT_BUFFER);
        if exe::database_staff_matches_names(staff, ids[0], ids[1], ids[2], d2p) != 0 {
            news_link(out, np, 0x7d8, staff);
        }
        return 1;
    }
    covered!("gra_1 injury body");
    exe::database_format_name_from_ids(ids[0], ids[1], ids[2], name.as_mut_ptr(), 0x69, 1, player);
    news_injury_name(kind, ip);
    duration(days, dp, 0x32, 1);
    let cat = category();
    if cat == 0 {
        covered!("gra_1 injury illness");
        let a = news_injury_gender(kind);
        let b = news_injury_gender(kind);
        if nation_i != -1 && club_i != -1 {
            let (nat, cl) = (news_nation(nation_i as u32), news_club(club_i as u32));
            let (gc, gn) = (news_g(cl + club::SHORT_GENDER_NAME), news_g(nat + 0x52));
            f(NEWS_TEXT_BUFFER, 0xa6655c, b, a, gc, gc, gn, gn, np, dp, ip, cl + club::SHORT_NAME, nat + 0x38);
        } else {
            f(NEWS_TEXT_BUFFER, 0xa664e4, b, a, np, dp, ip);
        }
        news_set(out, NEWS_TEXT_BUFFER);
        if exe::database_staff_matches_names(staff, ids[0], ids[1], ids[2], d2p) != 0 {
            news_link(out, np, 0x7d8, staff);
        }
        if nation_i != -1 {
            let nat = news_nation(nation_i as u32);
            news_link(out, nat + 0x38, 0x7d5, exe::database_get_national_team(nat));
        }
        if club_i != -1 {
            let cl = news_club(club_i as u32);
            news_link(out, cl + club::SHORT_NAME, 0x7d5, cl);
        }
        return 1;
    }
    if against == -1 {
        return news_injury_in_training(out, kind, np, dp, ip, d1p, staff, club_i, mine);
    }
    covered!("gra_1 injury in a match");
    let (cs, co, comp) = (news_club(against as u32), news_club(other), news_comp(comp_i));
    if exe::langlib_get_language() == 0xa && (kind == 0x3b || kind == 0x47) {
        let wd = news_weekday_name(d1p);
        sprintf(NEWS_TEXT_BUFFER, 0xa6648c, np, dp, ip, comp + competition::NAME, cs + club::SHORT_NAME, co + club::SHORT_NAME, wd);
    } else {
        let gk = if comp_i == read_u32(KEY_CLUB_COMP_FRIENDLY) { 0x64 } else { news_g(comp + competition::GENDER_NAME) as i8 as i32 };
        let (gcs, gco) = (news_g(cs + club::SHORT_GENDER_NAME), news_g(co + club::SHORT_GENDER_NAME));
        if in_match {
            let wd = news_weekday_name(d1p);
            let gwd = news_weekday_gender(d1p);
            let a = news_injury_gender(kind);
            let b = news_injury_gender(kind);
            f(NEWS_TEXT_BUFFER, 0xa6633c, b, a, gk, gk, gcs, gcs, gco, gco, gwd, 0x64, np, dp, ip, comp + competition::NAME,
              cs + club::SHORT_NAME, co + club::SHORT_NAME, wd);
        } else {
            covered!("gra_1 injury with the match record");
            let rc = news_comp(read_u32(rp + 8));
            let lg = news_round_label(rc, read_u16(rp + 0xc), read_u16(rp + 0xe), 1, &mut label);
            let lb = label.as_ptr() as usize;
            let wd = news_weekday_name(d1p);
            let gwd = news_weekday_gender(d1p);
            let a = news_injury_gender(kind);
            let b = news_injury_gender(kind);
            let (csn, con) = (cs + club::SHORT_NAME, co + club::SHORT_NAME);
            if ga > gb {
                f(NEWS_TEXT_BUFFER, 0xa661e4, b, a, gcs, gcs, lg, lg, gco, gco, gwd, 0x64, np, dp, ip, csn, scp, lb, con, wd);
            } else if ga == gb {
                f(NEWS_TEXT_BUFFER, 0xa66084, 0x64, 0x64, b, a, gcs, gcs, lg, lg, gco, gco, gwd, 0x64, np, dp, ip, csn, scp, lb, con, wd);
            } else {
                f(NEWS_TEXT_BUFFER, 0xa65f24, b, a, gcs, gcs, lg, lg, gco, gco, gwd, 0x64, np, dp, ip, csn, scp, lb, con, wd);
            }
        }
    }
    news_set(out, NEWS_TEXT_BUFFER);
    if exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), comp) == 0 {
        news_link(out, comp + competition::NAME, 0x7d0, comp);
    }
    news_link(out, np, 0x7d8, staff);
    news_link(out, cs + club::SHORT_NAME, 0x7d5, cs);
    news_link(out, co + club::SHORT_NAME, 0x7d5, co);
    if rec[0x19] != 0xff && !in_match {
        let fixture = read_u32(rp + 0x1b);
        if exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), fixture as i32, read_i32(rp + 0x11), read_i32(rp + 0x15),
            read_u16(rp) as i32, read_u16(rp + 2) as i32) != 0
        {
            news_link(out, scp, 0x7d3, fixture as usize);
        }
        let away = news_club(read_u32(rp + 0x15));
        news_link(out, away + club::NAME, 0x7d5, away);
    }
    1
}

/// Category 1, type 0x1388, a manager's message. Slots: +5 the sending manager, +9 nonzero for a private message,
/// +0xd the id of the news it answers (0 none), slot 3 its text.
///
/// Headline: "Private message from %s" (0xa65470) or "Public message from %s" (0xa65448) with the manager's name
/// (`exe::database_format_staff_name`, 0xc8, 1). Body: when it answers a news and `flag` is 0, the quoted news is looked
/// up (`News::find_item_by_id` 0x786b70 on the news object 0xb7431c, into a blank news item dated with the game's
/// date 0xae2c90 and day slice 0xae2c88) and, when found, its body is written (this routine, flag 1) into a new text
/// and appended as "> " text "\n>\n"; then the message's text (slot 3, `NewsItem::get_slot_string` 0x785a20, 0x7d0) is
/// appended. The body only appends to `out`.
/// Original: part of `NewsItem::write_text` (0x78284a..0x782a02).
pub(crate) unsafe fn news_follow_up(news: usize, manager: usize, out: usize, body: u32, flag: u32) -> u32 {
    NEWS_LAST_PIECE = "manager_message";
    let sender = read_u32(news + 5);
    let private = read_u32(news + 9) != 0;
    let quoted = read_u32(news + 0xd);
    if body == 0 {
        let mut name = [0u8; 0xc8];
        exe::database_format_staff_name(news_staff(sender), name.as_mut_ptr() as usize, 0xc8, 1);
        news_format()(NEWS_TEXT_BUFFER, if private { 0xa65470 } else { 0xa65448 }, name.as_ptr() as usize);
        news_set(out, NEWS_TEXT_BUFFER);
        return 1;
    }
    covered!("gra_1 manager message body");
    if quoted != 0 && flag == 0 {
        covered!("gra_1 manager message quote");
        let mut item = [0u8; 0xe0];
        let ip = item.as_mut_ptr() as usize;
        exe::date_date_54a110(ip + news_item::DATE);
        let mut tmp = [0u8; 8];
        exe::date_operator_3d(ip + news_item::DATE, tmp.as_mut_ptr() as usize, 0xae2c90usize);
        write_u8(ip + news_item::DAY_SLICE, read_u8(0xae2c88));
        let mut text = [0usize; 2];
        let tp = text.as_mut_ptr() as usize;
        exe::new_text((tp) as usize as *mut usize, (0) as u32 as usize);
        if exe::news_find_item_by_id(read_usize(0xb7431c), 0, quoted, ip) != 0 {
            exe::newsitem_write_text(ip, manager, (tp) as usize as *mut usize, 1, 1);
            exe::text_append_string(out, 0xa654a0usize); // "> "
            exe::text_append_string(out, text[0]);
            exe::text_append_string(out, 0xa6549cusize); // "\n>\n"
        }
        exe::drop_text(text.as_mut_ptr());
    }
    exe::newsitem_get_slot_string(news, 3, NEWS_TEXT_BUFFER, 0x7d0);
    exe::text_append_string(out, NEWS_TEXT_BUFFER);
    1
}

/// Writes the button label (the game's `langlib_sprintf`, so the text goes through the language file) and the screen
/// handler, and returns `r`.
pub(crate) unsafe fn set_action_button(label: usize, handler: *mut u32, text: usize, screen: u32, r: u8) -> u8 {
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    sprintf(label, text);
    *handler = screen;
    r
}

/// The piece of `get_action_button` a news item goes to, by the thousands of its type (`get_category`); for the comparer.
pub(crate) fn action_button_piece(kind: i32) -> &'static str {
    match kind {
        1..=999 => "general",
        2000..=2999 => "discipline",
        3000..=3999 => "player",
        4000..=4999 => "transfer",
        5000..=5999 => "reply",
        6000..=0x1790 => "board",
        _ => "none",
    }
}

/// The board news (6000s). Type 0x1774 with slot 1 (byte) 0: "Respond", handler 0x6b2450, 2. Type 0x1786 with slot 1
/// (byte) other than 5, slot 2 (byte) other than 0 and 6 and slot 5 (byte) 0: the "Ultimatum" label (the
/// `<%s - COMMENT (Issue ultimatum to board)>` template 0xa69454 with the text at 0xad9c64), handler 0x9064a0, 1.
/// Anything else: `None`, and the routine goes on to the other categories (which a board item does not match).
pub(crate) unsafe fn board_button(news: usize, label: usize, handler: *mut u32) -> Option<u8> {
    covered!("nma_8 board button");
    let s = |k: usize| action_slot(news, k);
    match read_i32(news) {
        0x1774 => {
            if read_u8(s(1)) == 0 { // nma8: the board request needs a reply
                return Some(respond_button(label, handler, 0x6b2450));
            }
        }
        0x1786 => {
            let (a, b, c) = (read_u8(s(1)), read_u8(s(2)), read_u8(s(5)));
            if a != 5 && b != 0 && b != 6 && c == 0 {
                let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
                sprintf(label, 0xa69454usize, 0xad9c64usize); // "Ultimatum<%s - COMMENT (Issue ultimatum to board)>"
                *handler = 0x9064a0;
                return Some(1);
            }
        }
        _ => {}
    }
    None
}

/// The general news (1..999): friendly and tournament invitations, cup draws, national squads, awards.
///
/// - 6, the friendly invitation (slots 0 and 1 the clubs, 2 the club asked, 3 and 4 day and year, 5 the state): the
///   state 3 or 4 gives 0. The club of the manager is club 1 when club 2 is club 0, otherwise club 0; it must be his
///   club job or his national job. The friendly of club 2 on the date (`exe::friendly_find_friendly_match`) and
///   `friendly_check_match_invitation(club 0, club 1, his club, day, year, friendly, no text)` other than 0: "Respond",
///   handler 0x4740c0, 2. The original passes day and year as dwords with the stack's upper halves; the routine does
///   not read them without a text.
/// - 9, the tournament invitation (slot 1 the club, 2 the tournament index, 3 the state, 5 and 6 day and year): state 3
///   or 4 gives 0; the tournament (`Friendly::get_tournament`) and `friendly_check_tournament_invitation(club,
///   tournament, no text)` other than 0: "Respond", handler 0x474720, 2.
/// - 0x16: the manager's club with the id in slot 1, slot 2 (word) equal to the word +0x439 of the object at
///   `G_UEFA_SEEDING` and slot 0 = 0: "Respond", 0x49cc60, 2.
/// - 0x18: his club with the id in slot 0 and slot 1 = 0: "Respond", 0x475070, 2. 0x19: the same with slot 3 = 0:
///   "Respond", 0x4752b0, 2. 0x1e: slot 2 = 0: "Respond", 0x7798c0, 2.
/// - 0x22, the national squad (slot 4 the club, 5 the squad slot, 7 the answer): slot 7 other than 0 gives 0. A news
///   item not of today (`news_item::DATE` against `G_CURRENT_DATE`) gets slot 7 = 1 (`set_item_slot` in the human's
///   inbox and the item) and 0. Otherwise the person mark (`read_person_mark` of `G_NATIONAL_TEAMS`) of the club's
///   nation for `exe::national_teams_get_squad_slot(club, slot 5)`, other than 0: "Respond", 0x77a140, 2.
/// - 0x27, the cup draw (slot 0 the competition, 1 the stage label, 4 the stage index or 0xff, 5 the round, 6 the
///   answer): slot 6 other than 0, no competition (`G_COMP_OBJECT_TABLE`), or a stage index above
///   `cup::CURRENT_STAGE` (signed): 0. The cup (the competition with 0xff, otherwise its stage) must be a cup
///   (`cup::COMP_TYPE` 1) with the stage label of slot 1. When the round is above `cup::CURRENT_ROUND`, the last tie of
///   the round before must have its fixture created. A tie of the round without its fixture: "View Draw", 0x49eac0, 1;
///   all created: slot 6 = 1 (`set_item_slot`) and 0.
/// - 0x28: the FA Cup (`KEY_CLUB_COMP_ENGLISH_FA_CUP`) with `eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED` = 1 and slot 0 =
///   1: "Respond", 0x5708f0, 2.
/// - 0x1c, the award nominations: the award (slot 0) and its item (slot 1) in the `G_AWARD_MANAGER` tables with a
///   nominee list (+0x1f) whose +4 word is not 0: "Nominations", 0x416cb0, 1.
/// - anything else: 0.
pub(crate) unsafe fn general_button(news: usize, manager: usize, label: usize, handler: *mut u32) -> u8 {
    covered!("nma_8 general button");
    let s = |k: usize| action_slot(news, k);
    let slot = |k: usize| read_u32(s(k));
    let me = Staff(manager);
    match read_i32(news) {
        6 => {
            let (a, b, c) = (slot(0) as i32, slot(1) as i32, slot(2) as i32);
            let (day, year) = (read_u16(s(3)), read_u16(s(4)));
            let state = read_u8(s(5));
            if state == 3 || state == 4 { // nma8: the friendly invitation is closed
                return 0;
            }
            let mine = if c == a { club_at(b as isize) } else { club_at(a as isize) };
            if me.club_job() != mine && me.national_job() != mine {
                return 0;
            }
            let mut date = [0u8; 8];
            exe::new_date(date.as_mut_ptr() as usize, day as u32, year as u32);
            let friendly = read_usize(G_FRIENDLY);
            let found = exe::friendly_find_friendly_match(friendly, date.as_mut_ptr() as usize, club_at(c as isize));
            if friendly_check_match_invitation(friendly, club_at(a as isize).addr(), club_at(b as isize).addr(), mine.addr(), day as u32, year as u32, found, 0, 0) == 0 {
                return 0;
            }
            respond_button(label, handler, 0x4740c0)
        }
        9 => {
            let state = read_u8(s(3));
            let index = read_u8(s(2));
            let club = club_at(slot(1) as i32 as isize);
            let (day, year) = (read_u16(s(5)), read_u16(s(6)));
            if state == 3 || state == 4 {
                return 0;
            }
            let mut date = [0u8; 8];
            exe::new_date(date.as_mut_ptr() as usize, day as u32, year as u32);
            let friendly = read_usize(G_FRIENDLY);
            let tournament = exe::friendly_get_tournament(friendly, index as u32, date.as_mut_ptr() as usize);
            if friendly_check_tournament_invitation(friendly, club.addr(), tournament, 0, 0) == 0 {
                return 0;
            }
            respond_button(label, handler, 0x474720)
        }
        0x16 => {
            let club = me.club_job();
            if club.is_null() || read_u32(club.addr()) != slot(1) || read_u16(s(2)) != read_u16(read_usize(G_UEFA_SEEDING) + 0x439) || slot(0) != 0 {
                return 0;
            }
            respond_button(label, handler, 0x49cc60)
        }
        0x18 => {
            let club = me.club_job();
            if club.is_null() || read_u32(club.addr()) != slot(0) || slot(1) != 0 {
                return 0;
            }
            respond_button(label, handler, 0x475070)
        }
        0x19 => {
            let club = me.club_job();
            if club.is_null() || read_u32(club.addr()) != slot(0) || slot(3) != 0 {
                return 0;
            }
            respond_button(label, handler, 0x4752b0)
        }
        0x1e => {
            if slot(2) != 0 {
                return 0;
            }
            respond_button(label, handler, 0x7798c0)
        }
        0x22 => {
            if slot(7) != 0 {
                return 0;
            }
            if read_u16(G_CURRENT_DATE_YEAR) != read_u16(news + news_item::DATE + 2) || read_u16(G_CURRENT_DATE) != read_u16(news + news_item::DATE) {
                set_item_slot(read_usize(G_NEWS), manager, read_u32(news + news_item::ID), 7, 1, news);
                return 0;
            }
            let club = club_at(slot(4) as i32 as isize);
            let k = exe::national_teams_get_squad_slot(club, slot(5));
            if read_person_mark(read_usize(G_NATIONAL_TEAMS), club.nation().addr(), k as i8) == 0 {
                return 0;
            }
            respond_button(label, handler, 0x77a140)
        }
        0x27 => {
            let stage_label = read_u16(s(1));
            let stage = read_u8(s(4));
            let round = read_i16(s(5));
            if slot(6) != 0 {
                return 0;
            }
            let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((slot(0) as usize).wrapping_mul(4)));
            if comp == 0 || (stage as i8 as i32) > read_i32(comp + cup::CURRENT_STAGE) {
                return 0;
            }
            let cup = if stage == 0xff { comp } else { read_usize(read_usize(comp + cup::STAGES).wrapping_add((stage as i8 as isize as usize).wrapping_mul(4))) };
            if read_u8(cup + cup::COMP_TYPE) != 1 || read_u16(cup + cup::STAGE_LABEL) != stage_label {
                return 0;
            }
            let r = round as i32 as usize;
            if (read_u8(cup + cup::CURRENT_ROUND) as i8 as i16) < round {
                // the last tie of the round before must have its fixture
                let ties = read_i16(read_usize(cup + cup::ROUND_TABLE).wrapping_add(r.wrapping_sub(1).wrapping_mul(0x68)) + 0x1a) as i32 as usize;
                let tie_table = read_usize(read_usize(cup + cup::TIE_TABLES).wrapping_add(r.wrapping_sub(1).wrapping_mul(4)));
                if read_u8(tie_table.wrapping_add(ties.wrapping_sub(1).wrapping_mul(0x35)) + cup_tie::FIXTURE_STATE) != TIE_FIXTURE_CREATED {
                    return 0;
                }
            }
            let ties = read_i16(read_usize(cup + cup::ROUND_TABLE).wrapping_add(r.wrapping_mul(0x68)) + 0x1a);
            if ties > 0 {
                let tie_table = read_usize(read_usize(cup + cup::TIE_TABLES).wrapping_add(r.wrapping_mul(4)));
                for k in 0..ties {
                    if read_u8(tie_table + k as usize * 0x35 + cup_tie::FIXTURE_STATE) != TIE_FIXTURE_CREATED {
                        return set_action_button(label, handler, 0xa69448, 0x49eac0, 1); // "View Draw"
                    }
                }
            }
            set_item_slot(read_usize(G_NEWS), manager, read_u32(news + news_item::ID), 6, 1, news);
            0
        }
        0x28 => {
            let fa_cup = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_ENGLISH_FA_CUP).wrapping_mul(4)));
            if read_u8(fa_cup + eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED) != 1 || slot(0) != 1 {
                return 0;
            }
            respond_button(label, handler, 0x5708f0)
        }
        0x1c => {
            let award = read_usize(read_usize(read_usize(G_AWARD_MANAGER) + 8).wrapping_add((slot(0) as usize).wrapping_mul(4)));
            let item = read_usize(read_usize(award + 4).wrapping_add((slot(1) as usize).wrapping_mul(4)));
            let nominees = read_usize(item + 0x1f);
            if nominees == 0 || read_u16(nominees + 4) == 0 {
                return 0;
            }
            set_action_button(label, handler, 0x987ff8, 0x416cb0, 1) // "Nominations"
        }
        _ => 0,
    }
}

/// The player news (3000s): contracts, media stories, bids for a player.
///
/// - 0xbbf: the manager's club job with the id in slot 0 and slot 7 (byte) 0: "Respond", 0x4dca40, 2.
/// - 0xbe8: the club of slot 8 managed by him (`club::MANAGER`), slot 18 (byte) 0 and slot 15 = 0: "Respond",
///   0x741f50, 2. 0xbe6: the club of slot 9 managed by him and slot 20 (byte) 0: the same.
/// - 0xbd9: slot 8 (byte) 0, then as 0xbd0. 0xbd0: the club of slot 4 managed by him and slot 10 (byte) 0: "Respond",
///   0x741f50, 2. 0xbdd: the same with slot 13 (byte). 0xbdf: the same with slot 10 (byte), except for a story kind
///   (slot 9, signed byte) of 0x3e, 0x50, 0x51 or 0x5e, which gives 0 (the original's jump table).
/// - 0xbc1: his club job with the id in slot 0 and slot 6 (byte) 0: "Respond", 0x8f66d0, 2. 0xbc2: the same with slot
///   7 (byte): "Offer Contract", 0x4dc9b0, 1.
/// - 0xbb8, the contract talks of a player (slot 0 the person, 4 the club, 8 the transfer offer or −1, 9 its serial,
///   10 the state): his club job with the id in slot 4; with an offer, the offer (`exe::transfer_manager_get_offer_slot`)
///   with the serial of slot 9; without one, slot 10 other than 5; then the contract offer of the person at the club
///   (`exe::contract_manager_find_contract_offer`) with `contract_offer::STATE` 3: "Negotiate", 0x4e5770, 2.
/// - anything else: 0.
pub(crate) unsafe fn player_button(news: usize, manager: usize, label: usize, handler: *mut u32) -> u8 {
    covered!("nma_8 player button");
    let s = |k: usize| action_slot(news, k);
    let slot = |k: usize| read_u32(s(k));
    let me = Staff(manager);
    let managed_by_me = |k: usize| club_at(slot(k) as i32 as isize).manager().addr() == manager;
    let own_club_is = |id: u32| !me.club_job().is_null() && read_u32(me.club_job().addr()) == id;
    let story = |club_slot: usize, flag_slot_addr: usize| {
        if !managed_by_me(club_slot) || read_u8(flag_slot_addr) != 0 {
            return 0;
        }
        respond_button(label, handler, 0x741f50)
    };
    match read_i32(news) {
        0xbbf => {
            if !own_club_is(slot(0)) || read_u8(s(7)) != 0 {
                return 0;
            }
            respond_button(label, handler, 0x4dca40)
        }
        0xbe8 => {
            if !managed_by_me(8) || read_u8(s(18)) != 0 || slot(15) != 0 {
                return 0;
            }
            respond_button(label, handler, 0x741f50)
        }
        0xbe6 => story(9, s(20)),
        0xbd9 => {
            if read_u8(s(8)) != 0 {
                return 0;
            }
            story(4, s(10))
        }
        0xbdf => {
            // the original's jump table on (story kind - 0x3e), 0..0x20: 0 ends the routine
            const KINDS: [u8; 33] = [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0];
            let k = (read_u8(s(9)) as i8 as i32).wrapping_sub(0x3e) as u32;
            if k <= 0x20 && KINDS[k as usize] == 0 {
                return 0;
            }
            story(4, s(10))
        }
        0xbdd => story(4, s(13)),
        0xbd0 => story(4, s(10)),
        0xbc1 => {
            if !own_club_is(slot(0)) || read_u8(s(6)) != 0 {
                return 0;
            }
            respond_button(label, handler, 0x8f66d0)
        }
        0xbc2 => {
            if !own_club_is(slot(0)) || read_u8(s(7)) != 0 {
                return 0;
            }
            set_action_button(label, handler, 0x9aff10, 0x4dc9b0, 1) // "Offer Contract"
        }
        0xbb8 => {
            let (person, club, offer, serial, state) = (slot(0), slot(4), slot(8) as i32, slot(9), read_u8(s(10)));
            if !own_club_is(club) {
                return 0;
            }
            if offer != -1 {
                let o = Offer(read_usize(exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), offer)));
                if o.is_null() || o.serial() as u32 != serial {
                    return 0;
                }
            } else if state == 5 {
                return 0;
            }
            let k = exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), person, club);
            if k == 0 || read_u8(k + contract_offer::STATE) != 3 { // nma8: the player's contract talks need a reply
                return 0;
            }
            set_action_button(label, handler, 0xa6943c, 0x4e5770, 2) // "Negotiate"
        }
        _ => 0,
    }
}

/// The discipline news (2000s): types 0x7d2 and 0x7d3, the ban that can be appealed (slot 0 the appeal made, 4 and 5
/// day and year of the ban, 10 the kind, 12 the appeal pending, 15 and 16 day and year of the hearing, 23 a match).
/// The person (`read_staff_name_ref_from_news` from slot 17) must be a valid id. With a match (slot 23), the match
/// record from slot 24 must have a valid club and competition (`G_CLUB_COMP_TABLE`), the club must not be a national
/// team, the competition must allow appeals (+0x5d) and the person must be at the manager's club. The appeal needs
/// the club's nation not to be Argentina (`club_nation_is_not_argentina`) and a kind other than 0, 1 and 4, no appeal
/// made or pending, and the hearing today or the ban still to start (its year above today's, or the same year and a
/// later day): "Appeal", 0x55e620, 1. Otherwise 0.
pub(crate) unsafe fn discipline_button(news: usize, manager: usize, label: usize, handler: *mut u32) -> u8 {
    covered!("nma_8 discipline button");
    let kind = read_i32(news);
    if kind != 0x7d3 && kind != 0x7d2 {
        return 0;
    }
    let s = |k: usize| action_slot(news, k);
    let appealed = read_u8(s(0));
    let (ban_day, ban_year) = (read_u16(s(4)), read_u16(s(5)));
    let (hearing_day, hearing_year) = (read_u16(s(15)), read_u16(s(16)));
    let ban_kind = read_u32(s(10));
    let pending = read_u32(s(12));
    let mut next = 0x11u8;
    let mut person = [0u8; 0x18];
    exe::read_staff_name_ref_from_news(news, &mut next, person.as_mut_ptr());
    let id = read_i32(person.as_ptr() as usize + 0xc);
    if id < 0 || id > read_i32(G_STAFF_COUNT) - 1 {
        return 0;
    }
    if read_u32(s(23)) != 0 {
        next = 0x18;
        let mut game = [0u8; 0x20];
        (exe::read_match_record_from_news(news, &mut next, game.as_mut_ptr()) as u32);
        let club_index = read_i32(game.as_ptr() as usize + 0x11);
        if club_index < 0 || club_index > read_i32(G_CLUB_COUNT) - 1 {
            return 0;
        }
        let comp_index = read_i32(game.as_ptr() as usize + 8);
        if comp_index < 0 || comp_index > read_i32(G_CLUB_COMP_COUNT) - 1 {
            return 0;
        }
        let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add((comp_index as usize).wrapping_mul(0x6b));
        if exe::database_is_national_team(club_at(club_index as isize)) != 0 || read_u32(comp + 0x5d) == 0
            || Staff(manager).club_job() != staff_at(id as isize).club_job() {
            return 0;
        }
    }
    let mut can = club_nation_is_not_argentina(Staff(manager).club_job().addr()) as u8;
    if ban_kind == 4 || ban_kind == 0 || ban_kind == 1 {
        can = 0;
    }
    if appealed != 0 || can == 0 || pending != 0 {
        return 0;
    }
    let (year, day) = (read_u16(G_CURRENT_DATE_YEAR), read_u16(G_CURRENT_DATE));
    let hearing_today = hearing_year == year && hearing_day == day;
    if !hearing_today && !(ban_year > year || (ban_year == year && (ban_day as i16) > (day as i16))) {
        return 0;
    }
    set_action_button(label, handler, 0xa69434, 0x55e620, 1) // "Appeal"
}

/// The transfer news (4000s), on the offers of `G_TRANSFER_MANAGER` (slot 6, 7 or 11 the offer index; the offer must
/// have the serial (`offer::SERIAL`) the news keeps).
///
/// - 0xfa0, a bid for one of his players: the club of slot 5 managed by him, the serial of slot 7, the kind (slot 9,
///   byte) not 1, 2, 5 or 0x11 and the offer in state 0: "Respond", 0x8f1730, 2.
/// - 0xfbc: the club of slot 4 (above −1) managed by him and slot 7 (byte) 0: "Respond", 0x741f50, 2.
/// - 0xfa2 and 0xfc0, a negotiation (slot 9 the negotiation's serial): the offer of slot 6 with the serial of slot 7,
///   a negotiation (`offer::NEGOTIATION_INDEX`, `get_negotiation_slot`) with the serial of slot 9 and the offer in
///   state 1: "Respond", 0x8f1830 (0xfa2) or 0x8f93b0 (0xfc0), 2.
/// - 0xfac, personal terms (slot 0 the person, 7 the offer, 8 its serial, 9 the kind): kind 1 or 4 gives 0; the
///   offer open (`exe::transfer_offer_is_closed` 0) with the serial and not in state 1. Bought by his club
///   (`offer::BUYER_CLUB_ID`): with no terms yet (`offer::TERMS_STATE` 0) or the offer in state 5 or 6, "Offer
///   Contract", 0x8f1950, 2. Otherwise, when the target's contract (`exe::contract_manager_get_contract`, kept in
///   `G_CONTRACT_LOOKUP_RESULT`) is at his club (or both are none), a player of the exchange list
///   (`offer::EXCHANGE_PLAYERS`) with the id of slot 0 and +4 = 0: the same button.
/// - 0xfa4 (slot 6 the step, 7 the offer or −1, 8 its serial, 9 a flag): step 3, an offer with the serial in state 7
///   or 8 and the contract offer of the person (slot 0) at the club of slot 5 in state 3: "Negotiate", 0x4e5770, 2.
///   Step 4 with the flag 0, an offer with the serial, his club job with the id of slot 5, the offer in state 0xf and
///   `offer::RESPONSE_ROUND` below 3: "Negotiate", 0x4e5770, 1. When no button and an offer index other than −1:
///   slot 7 = −1 (`set_item_slot`) and 0.
/// - 0xfa5 (slot 6 the offer, 7 its serial, 9 the kind; kinds 2, 3, 0xc, 0xd, 0xe, 0x10, 0x11, 0x12 give 0): bought by
///   his club and the offer in state 0xb or 0xc; or the target's contract at his club, no pre-contract and the offer
///   in state 0xb or 0xd: "Confirm", 0x8f2d20, 2.
/// - 0xfab: his club job with the id of slot 0 or 1 and slot 7 (byte) 0: "Respond", 0x8f1a60, 2.
/// - 0xfb4, a contract renewal for a bid (slot 0 the person, 5 his club id, 10 the wish, 11 the offer, 12 its serial,
///   13 a flag): the offer's buyer is his club job, the person has a contract at a club with the id of slot 5, the
///   serial, no pre-contract, state 1, the flag 0 and the wish 1: "Re-Negotiate", 0x8f1b40, 2.
/// - 0xfb5: as 0xfa0 without the kind: "Respond", 0x8f1c30, 2.
/// - 0xfc1: "Details", 0x77b700, 1. 0xfa8, 0xfb6 and the rest: 0.
pub(crate) unsafe fn transfer_button(news: usize, manager: usize, label: usize, handler: *mut u32) -> u8 {
    covered!("nma_8 transfer button");
    let s = |k: usize| action_slot(news, k);
    let slot = |k: usize| read_u32(s(k));
    let tm = read_usize(G_TRANSFER_MANAGER);
    let offer_slot = |k: usize| exe::transfer_manager_get_offer_slot(tm, slot(k) as i32);
    let managed_by_me = |i: i32| club_at(i as isize).manager().addr() == manager;
    let own = Staff(manager).club_job().addr();
    // the club of a person's contract (`exe::contract_manager_get_contract`, kept in G_CONTRACT_LOOKUP_RESULT), or 0
    let contract_club = |person: u32| {
        let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff_at(person as i32 as isize)));
        write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
        if c.is_null() { 0 } else { club_at((c.club() as i32) as isize).addr() }
    };
    let negotiated = |k: usize| {
        let os = offer_slot(6);
        let o = Offer(read_usize(os));
        if o.is_null() || o.serial() as u32 != slot(7) || o.negotiation_index() as i32 == -1 {
            return false;
        }
        let n = Offer(read_usize(get_negotiation_slot(tm, o.negotiation_index() as i32)));
        slot(k) == (n.serial() as u32) && Offer(read_usize(os)).status() == 1
    };
    match read_i32(news) {
        0xfa0 => {
            let o = Offer(read_usize(offer_slot(6)));
            let (club, kind) = (slot(5) as i32, read_u8(s(9)));
            if !o.is_null() && club != -1 && managed_by_me(club) && o.serial() as u32 == slot(7) && kind != 1 && kind != 5 && kind != 2 && kind != 0x11
                && o.status() == 0 { // nma8: a bid for his player needs a reply
                return respond_button(label, handler, 0x8f1730);
            }
            0
        }
        0xfbc => {
            let club = slot(4) as i32;
            if club > -1 && managed_by_me(club) && read_u8(s(7)) == 0 {
                return respond_button(label, handler, 0x741f50);
            }
            0
        }
        0xfa2 => if negotiated(9) { respond_button(label, handler, 0x8f1830) } else { 0 },
        0xfc0 => if negotiated(9) { respond_button(label, handler, 0x8f93b0) } else { 0 },
        0xfac => {
            let person = slot(0);
            let kind = read_u8(s(9));
            let os = offer_slot(7);
            if kind == 1 || kind == 4 {
                return 0;
            }
            let o = Offer(read_usize(os));
            if o.is_null() || o.serial() as u32 != slot(8) || exe::transfer_offer_is_closed(o) != 0 || o.status() == 1 {
                return 0;
            }
            if own != 0 && read_u32(own) as usize == o.buyer_club_id() {
                if o.terms_state() == 0 || o.status() == 5 || o.status() == 6 {
                    return set_action_button(label, handler, 0x9aff10, 0x8f1950, 2); // "Offer Contract"
                }
                return 0;
            }
            if contract_club(o.target_staff_id() as u32) != own {
                return 0;
            }
            let list = Llist(read_usize(read_usize(os) + offer::EXCHANGE_PLAYERS));
            if list.is_null() {
                return 0;
            }
            list.set_cursor(list.head());
            loop {
                let p = exe::llist_get_next(read_usize(read_usize(os) + offer::EXCHANGE_PLAYERS));
                if p == 0 {
                    return 0;
                }
                if read_u32(p) == person && read_u8(p + 4) == 0 {
                    return set_action_button(label, handler, 0x9aff10, 0x8f1950, 2); // "Offer Contract"
                }
            }
        }
        0xfa4 => {
            let step = read_u8(s(6));
            let index = slot(7) as i32;
            let flag = read_u8(s(9));
            if (step == 3 || step == 4) && index == -1 {
                return 0;
            }
            if step == 3 {
                let o = Offer(read_usize(exe::transfer_manager_get_offer_slot(tm, index)));
                if !o.is_null() && o.serial() as u32 == slot(8) && (o.status() == 7 || o.status() == 8) {
                    let k = exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), slot(0), slot(5));
                    if k != 0 && read_u8(k + contract_offer::STATE) == 3 {
                        return set_action_button(label, handler, 0xa6943c, 0x4e5770, 2); // "Negotiate"
                    }
                }
            } else if step == 4 && flag == 0 {
                let o = Offer(read_usize(exe::transfer_manager_get_offer_slot(tm, index)));
                if !o.is_null() && o.serial() as u32 == slot(8) && own != 0 && read_u32(own) == slot(5) && o.status() == 0xf && o.response_round() < 3 {
                    return set_action_button(label, handler, 0xa6943c, 0x4e5770, 1); // "Negotiate"
                }
            }
            if index != -1 {
                set_item_slot(read_usize(G_NEWS), manager, read_u32(news + news_item::ID), 7, -1i32 as u32, news);
            }
            0
        }
        0xfa5 => {
            let kind = read_u8(s(9));
            if [0xe, 0xd, 0xc, 3, 2, 0x10, 0x12, 0x11].contains(&kind) {
                return 0;
            }
            let os = offer_slot(6);
            let o = Offer(read_usize(os));
            if o.is_null() || o.serial() as u32 != slot(7) {
                return 0;
            }
            let confirm = if own != 0 && read_u32(own) as usize == o.buyer_club_id() {
                o.status() == 0xb || o.status() == 0xc
            } else {
                let o2 = Offer(read_usize(os));
                contract_club(o.target_staff_id() as u32) == own && o2.pre_contract() == 0 && (o2.status() == 0xb || o2.status() == 0xd)
            };
            if confirm {
                return set_action_button(label, handler, 0x98a198, 0x8f2d20, 2); // "Confirm"
            }
            0
        }
        0xfab => {
            if own != 0 && (read_u32(own) == slot(0) || read_u32(own) == slot(1)) && read_u8(s(7)) == 0 {
                return respond_button(label, handler, 0x8f1a60);
            }
            0
        }
        0xfb4 => {
            let (wish, serial, flag, person, club_id) = (read_u8(s(10)), slot(12), read_u8(s(13)), slot(0), slot(5));
            let os = exe::transfer_manager_get_offer_slot(tm, slot(11) as i32);
            if os == 0 || read_usize(os) == 0 {
                return 0;
            }
            if club_at(Offer(read_usize(os)).buyer_club_id() as i32 as isize).addr() != own {
                return 0;
            }
            if contract_club(person) == 0 {
                return 0;
            }
            if read_u32(contract_club(person)) != club_id {
                return 0;
            }
            let o = Offer(read_usize(os));
            if o.serial() as u32 == serial && o.pre_contract() == 0 && o.status() == 1 && flag == 0 && wish == 1 {
                return set_action_button(label, handler, 0xa69424, 0x8f1b40, 2); // "Re-Negotiate"
            }
            0
        }
        0xfb5 => {
            let o = Offer(read_usize(offer_slot(6)));
            let club = slot(5) as i32;
            if !o.is_null() && club != -1 && managed_by_me(club) && o.serial() as u32 == slot(7) && o.status() == 0 {
                return respond_button(label, handler, 0x8f1c30);
            }
            0
        }
        0xfc1 => set_action_button(label, handler, 0xa6941c, 0x77b700, 1), // "Details"
        _ => 0,
    }
}

/// The game's text formatter (`langlib_sprintf`: `{}` markers take the gender ints).
pub(crate) unsafe fn news_format() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_LANGLIB_SPRINTF)
}

/// The C runtime `sprintf`.
pub(crate) unsafe fn news_sprintf() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_CRT_SPRINTF)
}

/// The club at index `i` of `G_CLUB_TABLE` (32-bit wrapping, no bounds check, like the original).
pub(crate) unsafe fn news_club(i: u32) -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add((i as usize).wrapping_mul(club::SIZE))
}

/// The competition at index `i` of `G_CLUB_COMP_TABLE`.
pub(crate) unsafe fn news_comp(i: u32) -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add((i as usize).wrapping_mul(NEWS_COMP_SIZE))
}

/// The nation at index `i` of `G_NATION_TABLE`.
pub(crate) unsafe fn news_nation(i: u32) -> usize {
    read_usize(G_NATION_TABLE).wrapping_add((i as usize).wrapping_mul(nation::SIZE))
}

/// The person with id `i` of `G_STAFF_TABLE`.
pub(crate) unsafe fn news_staff(i: u32) -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE))
}

/// A gender byte (sign-extended, as the original's `movsx`).
pub(crate) unsafe fn news_g(a: usize) -> i32 {
    read_i8(a) as i32
}

/// `Text::add_link(out, text, kind, target)`.
pub(crate) unsafe fn news_link(out: usize, text: usize, kind: u32, target: usize) {
    exe::text_add_link(out, text as *const u8, kind, target);
}

/// `Text::set_string(out, text)`.
pub(crate) unsafe fn news_set(out: usize, text: usize) {
    exe::text_set_string(out, text as *const u8);
}

/// The C `strcmp(a, b) == 0`.
pub(crate) unsafe fn news_streq(a: usize, b: usize) -> bool {
    let mut k = 0;
    loop {
        let (x, y) = (read_u8(a + k), read_u8(b + k));
        if x != y {
            return false;
        }
        if x == 0 {
            return true;
        }
        k += 1;
    }
}

/// `Date::Date(this, a, b)` (8 bytes: the two words and the leap-year flag).
pub(crate) unsafe fn news_date(d: usize, a: u16, b: u16) {
    exe::new_date(d, a as u32, b as u32);
}

/// The friendly invitation's answer text writer (`friendly_check_match_invitation`, 0x5ca090, thiscall on
/// `G_FRIENDLY` 0xae2c74: the inviting club, the invited, the club asked, the date's two words, the match, the buffer
/// and its size 0x190; nonzero when it wrote the whole answer).
pub(crate) unsafe fn news_friendly_answer(a: usize, b: usize, c: usize, w1: u32, w2: u32, m: usize, buf: usize) -> u32 {
    exe::friendly_check_match_invitation(read_usize(0xae2c74), a, b, c, w1, w2, m, buf, 0x190)
}

/// `strcpy(dst, src)`.
pub(crate) unsafe fn news_strcpy(dst: usize, src: usize) {
    let mut k = 0;
    loop {
        let c = read_u8(src + k);
        write_u8(dst + k, c);
        if c == 0 {
            break;
        }
        k += 1;
    }
}

/// The date of two words of a news, as the original builds it: a default `Date` (`exe::date_date_54a110`) assigned
/// (`exe::date_operator_3d`) from `Date::Date(a, b)`.
pub(crate) unsafe fn news_assigned_date(d: usize, a: u16, b: u16) {
    let (mut d2, mut tmp) = ([0u8; 8], [0u8; 8]);
    exe::date_date_54a110(d);
    news_date(d2.as_mut_ptr() as usize, a, b);
    exe::date_operator_3d(d, tmp.as_mut_ptr() as usize, d2.as_ptr() as usize);
}

/// The round's label of a competition in `buf` (0xc8 bytes, `exe::comp_util_format_comp_stage_round_label` with the
/// form; "Friendly" and "Reserve" get a lower-case first letter) and its gender (`exe::comp_util_get_label_gender`).
pub(crate) unsafe fn news_round_label(comp: usize, stage: u16, round: u16, form: u32, buf: &mut [u8; 0xc8]) -> i32 {
    let lb = buf.as_mut_ptr() as usize;
    exe::comp_util_format_comp_stage_round_label(comp, stage as u32, round as u32, form, buf.as_mut_ptr(), 0xc8);
    if news_streq(lb, 0x9db958) {
        buf[0] = b'f'; // "Friendly"
    } else if news_streq(lb, 0x9b5b40) {
        buf[0] = b'r'; // "Reserve"
    }
    exe::comp_util_get_label_gender(comp as u32, stage as u32, round as u32, form) as i8 as i32
}

/// The news.cpp error box without releasing the mouse button (the caller does it after its own fix-up, as the
/// original's inline box of type 21).
pub(crate) unsafe fn news_error_box_keep(line: u32) {
    let (mut name, mut ext) = ([0u8; 0x100], [0u8; 0x100]);
    exe::crt_strtok(0xa693ec, 0, 0, name.as_mut_ptr(), ext.as_mut_ptr());
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let v = version(name.as_mut_ptr(), ext.as_mut_ptr(), line);
    news_sprintf()(0xae24d0, 0x9870e8, v, name.as_ptr(), ext.as_ptr(), line); // "v%s %s.%s %d"
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0); // "Error"
}

/// The name of an injury type into `buf` (`injury_get_type_name`, 0x62ef30, stdcall; the original loads `G_INJURY`
/// into ecx).
pub(crate) unsafe fn news_injury_name(kind: u8, buf: usize) {
    let w: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(FN_INJURY_GET_TYPE_NAME);
    w(read_usize(G_INJURY), kind as u32, buf, 0x32, 1);
}

/// The grammatical gender of an injury type's name (`injury_get_type_name_gender`, 0x6306e0).
pub(crate) unsafe fn news_injury_gender(kind: u8) -> i32 {
    (exe::injury_get_type_name_gender(read_usize(G_INJURY), kind as u32) as i8) as i32
}

/// The weekday's name of a date (`Date::get_weekday`, `date_get_weekday_name` 0x54d170).
pub(crate) unsafe fn news_weekday_name(date: usize) -> usize {
    exe::date_get_weekday_name((exe::date_get_weekday(date)) as i32 as u32)
}

/// The weekday's gender of a date in the current language (`exe::date_get_weekday_gender`).
pub(crate) unsafe fn news_weekday_gender(date: usize) -> i32 {
    let wd = exe::date_get_weekday(date);
    exe::date_get_weekday_gender(exe::langlib_get_language() as u32, wd as u32) as i32
}

/// The body of an injury in training (see `news_match_report`): the text, the player's link (0x7d8, always) and his
/// club's (0x7d5, when he has one).
/// Original: part of `NewsItem::write_text` (0x780fe1..0x78164b).
pub(crate) unsafe fn news_injury_in_training(out: usize, kind: u8, np: usize, dp: usize, ip: usize, d1p: usize, staff: usize, club_i: i32, mine: bool) -> u32 {
    covered!("gra_1 injury in training");
    let f = news_format();
    let sprintf = news_sprintf();
    let category = || exe::injury_get_type_category(read_usize(G_INJURY), kind as i8);
    if exe::langlib_get_language() == 0xa && (kind == 0x3b || kind == 0x47) {
        let mut day = [0u8; 0x40];
        news_strcpy(day.as_mut_ptr() as usize, news_weekday_name(d1p));
        if day[0] != 0 {
            let len = day.iter().position(|&c| c == 0).unwrap_or(0);
            let k = (day[len - 1] as i8 as i32).wrapping_sub(0x43);
            // the Swedish genitive: no extra "s" after C, S, X or Z
            let plain = (0..=0x37).contains(&k) && read_u8(0x782b04 + k as usize) == 0;
            sprintf(NEWS_TEXT_BUFFER, if plain { 0xa65edc } else { 0xa65e94 }, np, dp, ip, day.as_ptr() as usize);
        }
    } else if !mine && club_i != -1 {
        let cl = news_club(club_i as u32);
        let gcl = news_g(cl + club::SHORT_GENDER_NAME);
        let csn = cl + club::SHORT_NAME;
        if category() == 0xc {
            let r = exe::injury_get_recurring_injury_type(read_usize(G_INJURY), staff);
            if r == 0xff {
                match (kind as i8 as i32).wrapping_sub(0x4d) {
                    0 => { f(NEWS_TEXT_BUFFER, 0xa65d58, gcl, gcl, csn, np, dp); }
                    1 => { f(NEWS_TEXT_BUFFER, 0xa65ca8, gcl, gcl, csn, np, dp); }
                    3 => { f(NEWS_TEXT_BUFFER, 0xa65aa4, gcl, gcl, csn, np); }
                    4 => { f(NEWS_TEXT_BUFFER, 0xa65e04, np); }
                    _ => { f(NEWS_TEXT_BUFFER, 0xa65c10, gcl, gcl, csn, np, dp); }
                }
            } else {
                covered!("gra_1 injury recurring");
                news_injury_name(r, ip);
                match (kind as i8 as i32).wrapping_sub(0x4d) {
                    0 => {
                        let a = news_injury_gender(r);
                        let b = news_injury_gender(r);
                        f(NEWS_TEXT_BUFFER, 0xa65908, gcl, gcl, b, a, csn, np, dp, ip);
                    }
                    1 => {
                        let a = news_injury_gender(r);
                        let b = news_injury_gender(r);
                        f(NEWS_TEXT_BUFFER, 0xa659d4, gcl, gcl, b, a, csn, np, dp, ip);
                    }
                    3 => { f(NEWS_TEXT_BUFFER, 0xa65aa4, gcl, gcl, csn, np); }
                    _ => {
                        let a = news_injury_gender(r);
                        let b = news_injury_gender(r);
                        f(NEWS_TEXT_BUFFER, 0xa65b40, gcl, gcl, 0x64, b, a, csn, np, dp, ip);
                    }
                }
            }
        } else {
            let wd = news_weekday_name(d1p);
            let gwd = news_weekday_gender(d1p);
            let a = news_injury_gender(kind);
            let b = news_injury_gender(kind);
            f(NEWS_TEXT_BUFFER, 0xa65814, gcl, gcl, 0x64, 0x64, 0x64, b, a, gwd, 0x64, csn, np, dp, ip, wd);
        }
    } else if category() == 0xc {
        match (kind as i8 as i32).wrapping_sub(0x4d) {
            0 => { f(NEWS_TEXT_BUFFER, 0xa656f4, np, dp); }
            1 => { f(NEWS_TEXT_BUFFER, 0xa65664, np, dp); }
            3 => { f(NEWS_TEXT_BUFFER, 0xa655ec, np); }
            4 => { f(NEWS_TEXT_BUFFER, 0xa65780, np); }
            _ => { f(NEWS_TEXT_BUFFER, 0xa65574, np, dp); }
        }
    } else {
        let wd = news_weekday_name(d1p);
        let gwd = news_weekday_gender(d1p);
        let a = news_injury_gender(kind);
        let b = news_injury_gender(kind);
        f(NEWS_TEXT_BUFFER, 0xa654a4, b, a, gwd, 0x64, np, dp, ip, wd);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, np, 0x7d8, staff);
    if club_i > -1 {
        let cl = news_club(club_i as u32);
        news_link(out, cl + club::SHORT_NAME, 0x7d5, cl);
    }
    1
}

/// Address of data slot `k` of a news item (`news_item::DATA` + 4 × k).
pub(crate) fn action_slot(news: usize, k: usize) -> usize {
    news + news_item::DATA + 4 * k
}

/// "Respond" (0xa69488) with the screen handler, needing a reply (2).
pub(crate) unsafe fn respond_button(label: usize, handler: *mut u32, screen: u32) -> u8 {
    set_action_button(label, handler, 0xa69488, screen, 2)
}

/// The size of a competition record of `G_CLUB_COMP_TABLE`.
pub(crate) const NEWS_COMP_SIZE: usize = 0x6b;

/// The piece of the last text written (for the comparer's message).
pub(crate) static mut NEWS_LAST_PIECE: &str = "";

/// The types written by another routine, by category (`NewsItem::get_category`) and type, in the original's order.
pub(crate) const NEWS_WRITERS: &[(u8, u32, NewsWriter)] = {
    use NewsWriter::*;
    &[
        (2, 0x1d, Global(0xb74318, 0x76e3b0)),
        (2, 0x1e, Global(0xb74318, 0x76f370)),
        (2, 0x20, Global(0xb74318, 0x76fa40)),
        (2, 0x21, Global(0xb74318, 0x76ff40)),
        (2, 0x22, Global(0xb74318, 0x7703f0)),
        (2, 0x24, Global(0xb74318, 0x770d20)),
        (2, 0x25, Global(0xb74434, 0x7e7280)),
        (2, 0x2c, Global(0xb74318, 0x777ca0)),
        (2, 0x2f, Global(0xde1f58, 0x906130)),
        (2, 0x2e, Global(0xb6b7c8, 0x75c5a0)),
        (2, 0x2a, Global(0xde2f74, 0x924160)),
        (2, 0x2, Global(0xde2f74, 0x923dd0)),
        (2, 0x29, Global(0xb67a44, 0x6aa910)),
        (2, 0x2b, Global(0xb67a44, 0x6aadb0)),
        (2, 0x18, Global(0xdd7ef0, 0x85af40)),
        (2, 0x19, Global(0xdd7ef0, 0x85b190)),
        (7, 0x3e8, Global(0xad9dac, 0x44c650)),
        (3, 0x7d1, Global(0xb63d28, 0x631580)),
        (3, 0x7d2, Global(0xae28f0, 0x5576e0)),
        (3, 0x7d3, Global(0xae28f0, 0x55b8b0)),
        (6, 0x1779, Global(0xae2a38, 0x59f3e0)),
        (6, 0x177f, Global(0xae2a38, 0x5a0230)),
        (6, 0x1780, Global(0xae2a38, 0x5a04c0)),
        (6, 0x177d, Global(0xae2a38, 0x5a0050)),
        (6, 0x177a, Global(0xae2a38, 0x59f9c0)),
        (6, 0x177b, Global(0xae2a38, 0x59fcf0)),
        (6, 0x1783, Global(0xae2a38, 0x5a0380)),
        (6, 0x1786, Global(0xb67a44, 0x6a9470)),
        (6, 0x1784, Global(0xae2a3c, 0x5a7ed0)),
        (6, 0x177c, Global(0xae2a38, 0x59feb0)),
        (6, 0x1775, Global(0xb67a44, 0x6a0690)),
        (6, 0x1778, Global(0xb67a44, 0x6a0ad0)),
        (6, 0x1774, Global(0xb67a44, 0x6a5120)),
        (6, 0x1777, Global(0xb67a44, 0x6a1190)),
        (6, 0x1776, Global(0xb67a44, 0x69fef0)),
        (6, 0x1770, Global(0xb67a44, 0x6a1d60)),
        (6, 0x1773, Global(0xb67a44, 0x69c6b0)),
        (6, 0x177e, Global(0xb67a44, 0x6a4550)),
        (6, 0x1771, Global(0xb67a44, 0x6a4ae0)),
        (6, 0x1772, Global(0xb67a44, 0x6a4d40)),
        (6, 0x1782, Global(0xb67a44, 0x6a5e30)),
        (6, 0x1781, Global(0xb67a44, 0x6a5b40)),
        (6, 0x1788, Global(0xae2a38, 0x59fb90)),
        (6, 0x1789, Global(0xb67a44, 0x6aab70)),
        (6, 0x178a, Global(0xb67a44, 0x6ac190)),
        (6, 0x178e, Global(0xb6b7c8, 0x758fe0)),
        (6, 0x178d, Global(0xb67a44, 0x6ac880)),
        (6, 0x178f, Global(0xb67a44, 0x6adb70)),
        (6, 0x1790, Global(0xb6b7c8, 0x74af60)),
        (6, 0x178c, Plain(0x58a660)),
        (4, 0xbbb, Global(0xb7434c, 0x7aa4c0)),
        (4, 0xbdb, Global(0xb6b7c8, 0x72bd50)),
        (4, 0xbdd, Global(0xb6b7c8, 0x744f80)),
        (4, 0xbdc, Global(0xb6b7c8, 0x743d30)),
        (4, 0xbd0, Global(0xb6b7c8, 0x72d270)),
        (4, 0xbdf, Global(0xb6b7c8, 0x74bbd0)),
        (4, 0xbda, Global(0xb6b7c8, 0x733000)),
        (4, 0xbde, Plain(0x589480)),
        (4, 0xbcf, Global(0xb6b7c8, 0x7435a0)),
        (4, 0xbd9, Global(0xb6b7c8, 0x733fc0)),
        (4, 0xbce, Global(0xb7434c, 0x7a8eb0)),
        (4, 0xbcd, Plain(0x5886d0)),
        (4, 0xbba, Global(0xb7434c, 0x7a9650)),
        (4, 0xbb8, Global(0xae1050, 0x4d4fb0)),
        (4, 0xbb9, Global(0xae1050, 0x4daa00)),
        (4, 0xbbc, Global(0xae1050, 0x4daec0)),
        (4, 0xbbe, Global(0xae1050, 0x4d6ee0)),
        (4, 0xbbd, Global(0xae1050, 0x4d6a00)),
        (4, 0xbbf, Global(0xae1050, 0x4d8990)),
        (4, 0xbe1, Plain(0x589880)),
        (4, 0xbe4, Global(0xb6b7c8, 0x75d210)),
        (4, 0xbe5, Global(0xb6b7c8, 0x75b9c0)),
        (4, 0xbe2, Plain(0x589d60)),
        (4, 0xbe3, Plain(0x58a210)),
        (4, 0xbe6, Global(0xb6b7c8, 0x759a10)),
        (4, 0xbe9, Global(0xae2a30, 0x56a560)),
        (4, 0xbe8, Global(0xb6b7c8, 0x75a940)),
        (4, 0xbea, Global(0xae1050, 0x4d7770)),
        (4, 0xbc0, Global(0xae1050, 0x4d95e0)),
        (4, 0xbc1, Global(0xae1050, 0x4d9a90)),
        (4, 0xbc2, Global(0xae1050, 0x4da190)),
        (4, 0xbc5, Global(0xad9c6c, 0x419c60)),
        (4, 0xbc6, Global(0xae2a3c, 0x5a76e0)),
        (4, 0xbc7, Plain(0x587e90)),
        (4, 0xbc8, Plain(0x5883f0)),
        (4, 0xbc9, Global(0xad9c6c, 0x41a8e0)),
        (4, 0xbcb, Global(0xb6b7c8, 0x7572a0)),
        (4, 0xbca, Plain(0x5889e0)),
        (4, 0xbc3, Global(0xde1f54, 0x8e3b80)),
        (4, 0xbc4, Global(0xae1050, 0x4d65c0)),
        (4, 0xbcc, Global(0xde1f54, 0x8e4460)),
        (5, 0xfa0, Global(0xde1f54, 0x8d3520)),
        (5, 0xfa1, Global(0xde1f54, 0x8d5420)),
        (5, 0xfbc, Global(0xb6b7c8, 0x755860)),
        (5, 0xfbd, Plain(0x587370)),
        (5, 0xfbe, Plain(0x587800)),
        (5, 0xfa2, Global(0xde1f54, 0x8d6e80)),
        (5, 0xfa3, Global(0xde1f54, 0x8d7870)),
        (5, 0xfac, Global(0xde1f54, 0x8d7fb0)),
        (5, 0xfa4, Global(0xde1f54, 0x8d9480)),
        (5, 0xfa5, Global(0xde1f54, 0x8da8f0)),
        (5, 0xfa7, Global(0xde1f54, 0x8db580)),
        (5, 0xfa6, Global(0xde1f54, 0x8dba80)),
        (5, 0xfa8, Global(0xde1f54, 0x8dc370)),
        (5, 0xfa9, Global(0xde1f54, 0x8dd830)),
        (5, 0xfab, Global(0xde1f54, 0x8ddc70)),
        (5, 0xfad, Global(0xde1f54, 0x8de320)),
        (5, 0xfae, Global(0xde1f54, 0x8d84e0)),
        (5, 0xfaf, Global(0xde1f54, 0x8d8af0)),
        (5, 0xfb0, Global(0xde1f54, 0x8de980)),
        (5, 0xfb1, Global(0xde1f54, 0x8dffe0)),
        (5, 0xfb2, Global(0xde1f54, 0x8e05a0)),
        (5, 0xfb3, Global(0xde1f54, 0x8e08e0)),
        (5, 0xfb4, Global(0xde1f54, 0x8e1370)),
        (5, 0xfb5, Global(0xde1f54, 0x8e19c0)),
        (5, 0xfb6, Global(0xde1f54, 0x8e20a0)),
        (5, 0xfb7, Global(0xde1f54, 0x8e2570)),
        (5, 0xfb8, Global(0xae1050, 0x4db2e0)),
        (5, 0xfb9, Global(0xde1f54, 0x8e2cd0)),
        (5, 0xfba, Global(0xde1f54, 0x8e32c0)),
        (5, 0xfbb, Global(0xde1f54, 0x8e3780)),
        (5, 0xfbf, Global(0xde1f54, 0x8e3f90)),
        (5, 0xfc0, Global(0xde1f54, 0x8e47a0)),
        (5, 0xfc2, Plain(0x58aa60)),
        (5, 0xfc1, Plain(0x58a900)),
        (1, 0x138d, Plain(0x588f00)),
        (1, 0x138f, Global(0xad9c6c, 0x41b030)),
        (1, 0x1390, Global(0xb67a44, 0x69c910)),
        (1, 0x1391, Object(0xadadf4, 0x487f60)),
        (1, 0x1393, Global(0xb7433c, 0x795f50)),
        (1, 0x1394, Global(0xae2a3c, 0x5a8090)),
        (1, 0x1392, Object(0xb74344, 0x7a1cd0)),
        (1, 0x138e, Global(0xb74448, 0x7f9d10)),
        (1, 0x138c, Global(0xb67a44, 0x6a6310)),
        (1, 0x1389, Plain(0x7b7840)),
        (1, 0x138a, Plain(0x7b81f0)),
        (1, 0x138b, Plain(0x7b95c0)),
        (6, 0x1787, Global(0xb67a44, 0x6a3060)),
        (4, 0xbe0, Global(0xb6b7c8, 0x738d50)),
        (5, 0xfc3, Global(0xde1f54, 0x8e51d0)),
    ]
};

/// The club asked to skip the FA Cup for the World Club Championship (the key club index at 0x9d0240).
pub(crate) const NEWS_KEY_CLUB_WORLD_CLUB_BYE: usize = 0x9d0240;

/// The `cup_tie::FIXTURE_STATE` of a tie whose fixture was created.
pub(crate) const TIE_FIXTURE_CREATED: u8 = 3;

/// How a news type is written by another routine: (category, type, writer).
#[derive(Clone, Copy)]
pub(crate) enum NewsWriter {
    /// A `thiscall(object, news, body, out)` method of the object held by a global.
    Global(usize, usize),
    /// A `thiscall(object, news, body, out)` method of a fixed object.
    Object(usize, usize),
    /// A `cdecl(news, body, out)` routine.
    Plain(usize),
}

// ---------- CM_TEXTO_NOTICIAS: every news text written when posted (test mode) ----------
//
// The text writers (`newsitem_write_text` and the `write_*_news_text` it dispatches to) only run when someone opens a
// news item on screen, so the automatic career never calls them. With `CM_TEXTO_NOTICIAS=1` every news item, when it is
// sent (the seven `send_to_*` and `deliver_to_manager`, the doors every posted item goes through, also those of the
// machine's clubs), gets its headline and its body written for the first active human manager into a scratch Text,
// through the exe's address (0x77bc80): in comparison mode the comparers of the writers run. The writing must not change
// the career: the generator's state (`crate::rng::STATE_REGIONS`) is restored and its draws are not counted, the
// game heap's blocks come from a side region (`crate::allocator::aside`) and the private stack below the call is
// copied back (an original that reads a local without a value would find other garbage).

/// The news items being sent (the outer `send_to_*`, then its `deliver_to_manager`): a nested door with the same item
/// does not write it again.
static TEXTS_SENDING: [AtomicUsize; 8] = [const { AtomicUsize::new(0) }; 8];
static TEXTS_DEPTH: AtomicUsize = AtomicUsize::new(0);
static TEXTS_WRITING: AtomicBool = AtomicBool::new(false);
/// The private stack below the writing, copied before and written back after.
static mut TEXTS_STACK: [u32; crate::detours::STACK / 4] = [0; crate::detours::STACK / 4];

fn news_texts_on() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| crate::win::ambiente("CM_TEXTO_NOTICIAS"))
}

/// The guard of a door the news item goes through (dropped at its end).
pub(crate) struct NewsTextsPosting(bool);

impl Drop for NewsTextsPosting {
    fn drop(&mut self) {
        if self.0 {
            TEXTS_DEPTH.fetch_sub(1, Relaxed);
        }
    }
}

/// At a door of the news (`send_to_*`, `deliver_to_manager`): with `CM_TEXTO_NOTICIAS=1`, writes the item's texts
/// unless an outer door is sending the same item. Without the variable, nothing.
pub(crate) unsafe fn news_texts_posting(news: usize) -> NewsTextsPosting {
    if news == 0 || !news_texts_on() {
        return NewsTextsPosting(false);
    }
    let d = TEXTS_DEPTH.load(Relaxed);
    if d >= TEXTS_SENDING.len() {
        return NewsTextsPosting(false);
    }
    if !TEXTS_SENDING[..d].iter().any(|s| s.load(Relaxed) == news) && !TEXTS_WRITING.load(Relaxed) {
        news_texts_write(news);
    }
    TEXTS_SENDING[d].store(news, Relaxed);
    TEXTS_DEPTH.store(d + 1, Relaxed);
    NewsTextsPosting(true)
}

/// Copies whole dwords from `src` up to `src_end` into `dst`, without touching the stack (the copy of the stack).
#[inline(always)]
unsafe fn news_texts_copy(src: usize, dst: usize, src_end: usize) {
    if src >= src_end {
        return;
    }
    core::arch::asm!(
        "2:",
        "mov {t}, dword ptr [{s}]",
        "mov dword ptr [{d}], {t}",
        "add {s}, 4",
        "add {d}, 4",
        "cmp {s}, {e}",
        "jb 2b",
        s = inout(reg) src => _, d = inout(reg) dst => _, e = in(reg) src_end, t = out(reg) _,
        options(nostack),
    );
}

#[inline(always)]
unsafe fn news_texts_esp() -> usize {
    let esp: usize;
    core::arch::asm!("mov {}, esp", out(reg) esp, options(nomem, nostack, preserves_flags));
    esp
}

#[inline(never)]
unsafe fn news_texts_write(news: usize) {
    use crate::detours::{STACK, STACK_BASE};
    let copy = (&raw mut TEXTS_STACK) as usize;
    // the private stack (the simulation) below here; on the game's stack nothing is reproducible anyway
    let top = news_texts_esp();
    let private = (STACK_BASE..STACK_BASE + STACK).contains(&top);
    if private {
        news_texts_copy(STACK_BASE, copy, top);
    }
    TEXTS_WRITING.store(true, Relaxed);
    let rng = [read_u32(0xde1f5c), read_u32(0xde1f60), read_u32(0xad6c30)]; // crate::rng::STATE_REGIONS
    crate::rng::without_counting(|| crate::rng::pause_trace(|| crate::allocator::aside(|| news_texts_write_both(news))));
    write_u32(0xde1f5c, rng[0]);
    write_u32(0xde1f60, rng[1]);
    write_u32(0xad6c30, rng[2]);
    TEXTS_WRITING.store(false, Relaxed);
    if private {
        let now = news_texts_esp().min(top);
        news_texts_copy(copy, STACK_BASE, copy + (now - STACK_BASE));
    }
}

unsafe fn news_texts_write_both(news: usize) {
    let Some(manager) = (0..0x10).find_map(|i| active_human_manager(i)) else { return };
    if !newsitem_writes(exe::newsitem_get_category(news), read_u32(news)) {
        return; // the "Error" box of an unknown type would stop the career
    }
    covered!("news texts written on posting");
    for body in [0, 1] {
        // as the inbox screen: headline (0, 0) and body (1, 0)
        let mut text = [0usize; 2];
        exe::newsitem_write_text(news, manager, text.as_mut_ptr(), body, 0);
        exe::drop_text(text.as_mut_ptr());
    }
}

/// Whether `newsitem_write_text` has a writer for the category and type (otherwise it shows the "Error" box).
pub(crate) fn newsitem_writes(cat: u8, ty: u32) -> bool {
    match (cat, ty) {
        (2, 1 | 3..=13 | 16..=23 | 0x1a | 0x1b | 0x1c | 0x1f | 0x23 | 0x26 | 0x27 | 0x28) | (7, 0x3e9) | (3, 0x7d0) | (1, 0x1388) => true,
        _ => NEWS_WRITERS.iter().any(|e| e.0 == cat && e.1 == ty),
    }
}
