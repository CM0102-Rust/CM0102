use super::*;

/// Rule: opening the inbox of a manager record: the news screen with an empty filter.
///
/// `news_screens_open_news_screen(record, category)`: `open_screen_for_each_record` (builder
/// `news_screens_build_news_screen`, input `news_screens_news_input` 0x7892b0, flag 0, exit `news_screens_news_exit`
/// 0x7897a0); if it does not open nothing else happens. A 30-byte filter buffer (operator new; none: the
/// news_screens.cpp box of line 0x66 and nothing else): spaces with a terminator at 0x1d, then the placeholder
/// "#;.,234-][+-=-" printed at its start. For the record (`set_screen_param_for_each_record`): parameter 0 = the
/// second argument, 1 to 7 = 0, 8 = the buffer (with the flag 1: freed with the screen), 9 to 0x20 = 0 and 0x3b = 0.
/// Original: `news_screens_open_news_screen` (0x788010, news_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_screens_open_news_screen(record: usize, category: u32) {
    if exe::screenmanager_open_screen_for_each_record(G_SCREEN_MANAGER, record as u32, 0x7881e0usize, 0x7892b0usize, 0, 0x7897a0usize, 0) == 0 {
        return;
    }
    let filter = exe::crt_operator_new(0x1e);
    if filter == 0 {
        tel26_error_box(NEWS_SCREENS_CPP, 0x66);
        return;
    }
    covered!("tel28_3 open the inbox");
    for k in 0..0x1e {
        write_u8(filter + k, if k == 0x1d { 0 } else { 0x20 });
    }
    tel28_crt()(filter, NEWS_FILTER_PLACEHOLDER);
    let set = |k: u32, v: usize, flag: u32| exe::screenmanager_set_screen_param_for_each_record(G_SCREEN_MANAGER, record as u32, k, v, flag);
    set(0, category as usize, 0);
    for k in 1..8 {
        set(k, 0, 0);
    }
    set(8, filter, 1);
    for k in 9..=0x20 {
        set(k, 0, 0);
    }
    set(0x3b, 0, 0);
}

/// Comparer for `news_screens_open_news_screen`: only the original runs (it opens a screen and allocates the filter);
/// when the record's screen is the current one, its parameters are checked against the plan (the filter buffer as
/// the original left it, holding the placeholder).
pub(crate) unsafe extern "C" fn compare_news_screens_open_news_screen(record: usize, category: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_NEWS_SCREENS_OPEN_NEWS_SCREEN)(record, category);
    let filter = get_screen_param(8);
    if filter != 0 && get_screen_param(0) == category as usize && exe::crt_strstr(filter, NEWS_FILTER_PLACEHOLDER) == filter {
        let plan = news_screen_planned_params(category, filter);
        let mut got = [0usize; 0x3c];
        for (k, v) in got.iter_mut().enumerate() {
            *v = if (0x21..0x3b).contains(&k) { 0 } else { get_screen_param(k as u32) };
        }
        SCOREBOARD.record(got == plan, || format!("news screen (open {}): params {:x?} x plan {:x?}", category, got, plan));
    }
}

/// Rule: building the inbox: the manager's news of the category, filtered, with the open item's headline and text
/// and its action button.
///
/// Parameters: 0 a refresh, 1 the category (the tab), 2 the item list and 3 its count, 4 a reset, 8 the filter
/// (holding the placeholder "#;.,234-][+-=-": cleared), the category's selected id at 9 + 3 × category (its scroll
/// row next, its list area after). The manager is the current slot's. A refresh with nothing urgent
/// (`News::has_urgent_items`, 1) for a manager whose day is not done (`is_manager_done`) only sets the plain picture
/// and stops; otherwise parameter 0 = 0. A reset frees the list (parameters 2 and 3 = 0) and clears parameter 4. No
/// list: `News::get_inbox_items` of the category (parameters 2 and 3). The random picture; the colours of the
/// manager's club or national team (`database_get_club_kit_colours`, kit 1) or the heading colours. The heading "<the
/// manager's name> News" (in French "Infos d'..." before a vowel or Y, "Infos de ..." otherwise) as a Text linked to
/// the manager (0x7d7), in two banner areas; the title. The tab bar: All, Messages, Competitions, Injuries and Bans,
/// Contracts and Media, Transfers, Jobs, Records (values 0 to 7). The list area (2 columns, the rows of font 2 in 0x5a
/// pixels, over the reset parameter's upper bytes); the default selection when none (`news_screen_pick_default`); the
/// rows (`news_screen_list_rows`), empty rows to the bottom, the selected row kept in view, the scroll row and the
/// list area to the category's parameters. Then "Filter :" with the edit box (0x80008, the filter buffer, 0x19
/// characters), "Next Unread" (button id 5 with unread news, else grey text), the open item's headline (font 3, the
/// highlight colour) and its text (font 3, pale grey; the width read from the running exe, which the 1080 exe
/// changes), and its action button: a reply (2) as a widget (style 0x802 with the reply option, else 2; data the
/// button's), or plain text with parameter 0x3b = 1 when the slot already has one; any other as a button (id 3).
/// The menu bar (4, with the reply flag), Back/Next and parameter 6 = the unread count.
/// Original: `news_screens_build_news_screen` (0x7881e0, news_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_screens_build_news_screen() {
    let mut items = [[0u8; 0xd4]; 0x19];
    for it in items.iter_mut() {
        it[0] = 0xff;
        it[1] = 0xff;
    }
    let weights: [u8; 4] = [1, 3, 1, 4];
    let (mut text_a, mut text_b) = ([0usize; 2], [0usize; 2]);
    exe::new_text(text_a.as_mut_ptr(), 0usize);
    exe::new_text(text_b.as_mut_ptr(), 0usize);
    let manager = tel26_slot_manager();
    let refresh = get_screen_param(0);
    let tab = get_screen_param(1) as u8;
    let mut list = get_screen_param(2);
    let mut count = get_screen_param(3) as i32;
    let reset = get_screen_param(4) as u32;
    let filter = get_screen_param(8);
    if exe::crt_strstr(filter, NEWS_FILTER_PLACEHOLDER) != 0 {
        tel28_crt()(filter, 0xad9c64usize);
    }
    let k = (tab as i8 as i32 * 3 + 9) as u32;
    let mut sel = get_screen_param(k) as u32;
    if refresh != 0 {
        if exe::news_has_urgent_items(read_usize(G_NEWS), manager, 1) == 0 && exe::screenmanager_is_manager_done(G_SCREEN_MANAGER, manager.addr() as u32) == 0 {
            covered!("tel28_3 inbox refresh with nothing urgent");
            exe::background_set_picture(0usize);
            exe::drop_text(text_b.as_mut_ptr());
            exe::drop_text(text_a.as_mut_ptr());
            return;
        }
        set_screen_param(0, 0);
    }
    if reset != 0 {
        if list != 0 {
            exe::crt_free(list);
            list = 0;
            set_screen_param(2, 0);
            count = 0;
            set_screen_param(3, 0);
        }
        set_screen_param(4, 0);
    }
    if list == 0 {
        list = exe::news_get_inbox_items(read_usize(G_NEWS), manager, &mut count, tab as u32);
        set_screen_param(2, list);
        set_screen_param(3, count as isize as usize);
    }
    exe::background_set_random_picture();
    let (mut front, mut back) = (0u32, 0u32);
    let team = if manager.club_job().addr() != 0 { manager.club_job().addr() } else { manager.national_job().addr() };
    if team != 0 {
        exe::database_get_club_kit_colours(team, 1, &mut front, &mut back);
    } else {
        front = exe::get_heading_colour(1) as u32;
        back = exe::get_heading_colour(0) as u32;
    }
    let mut name = [0u8; 0x190];
    exe::database_format_staff_name(manager, name.as_mut_ptr() as usize, 0x190, 1);
    if exe::langlib_get_language() == 2 {
        let vowel = b"AEIOUYaeiouy".contains(&name[0]);
        tel28_crt()(JOB_TEXT, if vowel { 0xa69598 } else { 0xa6958c }, name.as_ptr()); // "Infos d'%s" / "Infos de %s"
    } else {
        tel28_lang()(JOB_TEXT, 0xa69560, name.as_ptr()); // "<%s - Manager Name(e.g.Walter Smith)> News"
    }
    exe::text_set_string(text_a.as_mut_ptr() as usize, JOB_TEXT as *const u8);
    exe::text_add_link(text_a.as_mut_ptr() as usize, name.as_ptr(), 0x7d7, manager);
    tel26_add_area(current_human_display(), [0x64, 0xa, 0x316, 0x46, 1, 0, 1, 0, 0x30, back, u32::MAX]);
    let mut row: i32 = 0;
    let banner = tel26_add_area(current_human_display(), [0x64, 0x19, 0x316, 0x37, 1, 0, 1, 0, 1, back, u32::MAX]);
    news_text_draw(&mut text_a, banner, 0x64, 0x316, &mut row, 7, front, 3);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, front, JOB_TEXT);
    let tabs: [(usize, u32); 8] = [(0xa6952c, 1), (0x98ed18, 2), (0xa69518, 3), (0xa69504, 4), (0x98e880, 5), (0xa694fc, 6), (0x990ab4, 7), (0, 0)];
    let set_kind = |item: &mut [u8; 0xd4], value: u32| {
        item[0xcc..0xd0].copy_from_slice(&1u32.to_le_bytes());
        item[0xd0..0xd4].copy_from_slice(&value.to_le_bytes());
    };
    tel28_lang()(items[0][4..].as_mut_ptr() as usize, 0xa69538, 0xad9c64usize); // "All<%s - COMMENT - all news categories>"
    set_kind(&mut items[0], 0);
    for (j, (fmt, value)) in tabs.iter().take(7).enumerate() {
        // "Messages", "Competitions", "Injuries and Bans", "Contracts and Media", "Transfers", "Jobs", "Records"
        tel28_lang()(items[j + 1][4..].as_mut_ptr() as usize, *fmt);
        set_kind(&mut items[j + 1], *value);
    }
    let (mut top, mut bottom) = (0x50i32, 0x221i32);
    exe::gui_add_tab_bar(items.as_mut_ptr() as usize, 8, tab as i8 as i32, &mut top, &mut bottom, 1);
    let visible = (0x5a / exe::font_get_height(2)) as u8;
    // the original stores the row count over the reset parameter's slot: the dword carries its upper bytes
    let rows = (reset & 0xffff_ff00) | visible as u32;
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, (top + 0x5a) as u32, 2, weights[2..].as_ptr() as u32, rows, 0, 1, 0, u32::MAX]);
    row = 0;
    let mut sel_row: i32 = -1;
    let mut shown = 0usize;
    let mut data = 0u32;
    set_screen_param(5, 0);
    if list != 0 {
        let mut first = u32::MAX;
        if sel == 0 {
            let (s, r, f) = news_screen_pick_default(list, count, manager.addr(), filter, k, &mut text_a, &mut data);
            sel = s;
            sel_row = r;
            first = f;
        }
        shown = news_screen_list_rows(list, count, manager.addr(), filter, k, sel, area, visible as i32, first, &mut row, &mut text_a, &mut text_b);
    }
    while row < visible as i32 {
        tel26_area_text([area, 0, row as u32, 2, 0, 1, 2, job_colour(JOB_COLOUR_3184), 0]);
        tel26_area_text([area, 1, row as u32, 2, 0, 1, 2, job_colour(JOB_COLOUR_3184), 0]);
        row += 1;
    }
    if sel_row > -1 {
        let p = k + 1;
        let low = sel_row - visible as i32 + 1;
        let kept = |low: i32| if get_screen_param(p) as i32 > low { get_screen_param(p) as i32 } else { low };
        let x = if kept(low) < sel_row { kept(low) } else { sel_row };
        set_screen_param(p, x as isize as usize);
    }
    let scroll = get_screen_param(k + 1);
    if area as i16 != -1 {
        exe::area_set_scroll_row(current_human_display() + 4 + (area as i16 as isize as usize).wrapping_mul(0xbf1), scroll);
    }
    set_screen_param(k + 2, area as i16 as isize as usize);
    news_screen_lower(top, bottom, manager.addr(), filter, shown, weights.as_ptr() as u32, &mut text_a, &mut data, reset);
    exe::drop_text(text_b.as_mut_ptr());
    exe::drop_text(text_a.as_mut_ptr());
}

/// Comparer for `news_screens_build_news_screen`: side by side over the display list when the build only reads (an
/// open list with a read item selected); otherwise only the original (it marks news read, allocates or frees the list
/// or stops on a refresh).
pub(crate) unsafe extern "C" fn compare_news_screens_build_news_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_NEWS_SCREENS_BUILD_NEWS_SCREEN);
    if news_screen_build_changes_game() {
        // simplification: marking news read, the list allocation and the refresh do not run twice; proven in Rust
        // mode and on the next build of the same screen
        return o();
    }
    compare_match_screen_build("news screen", &[], || o(), || news_screens_build_news_screen());
}

/// Rule: a news item's date is the Date it keeps at +0xcd.
///
/// `NewsItem::get_date(out)`: copies the 8 bytes at `NEWS_ITEM_DATE` (the day/year dword and the leap-year dword) to
/// `out` and returns `out`.
/// Original: `NewsItem::get_date` (0x789280, news_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn news_item_get_date(item: usize, out: usize) -> usize {
    covered!("tel36_6 news item date");
    write_u32(out, read_u32(item + NEWS_ITEM_DATE));
    write_u32(out + 4, read_u32(item + NEWS_ITEM_DATE + 4));
    out
}

/// Rule: the news screen answers its widgets: a filter tab (id 1) or a list scroll (2, 9) changes the view; an item's
/// action button (3) runs the action; the urgent-news buttons (5, 0x3e8, 0x3e9) move to the next item or leave; when
/// the unread count changed the screen is rebuilt; the rest goes to the menu bar.
///
/// `news_screens_news_input(item)`: the slot's human (`G_SCREEN_RECORDS_MANAGER`), parameter 0 (a mode), 1 (the filter,
/// a char), 5 (the item shown) and 6 (the unread count shown); the filter's parameter block starts at k = (filter + 3)
/// × 3. With parameter 0 set and `ScreenManager::is_manager_done(human)` = 0: -5. With a widget, parameter k + 1 =
/// `ScreenManager::get_area_scroll_row` of the area in parameter k + 2. When `News::count_unread(human, 0)` differs
/// from parameter 6: parameter 4 = 1, -11. By the widget's id (`tel26_item_id`, read again at each test like the
/// original): 1 with a data byte other than the filter: parameter 1 = the data, parameter 4 = 1, -11; 2: parameter k =
/// the data, -11; 9: parameter k = 0, -11; 3: the data is a routine called with (human, +0xd6 of parameter 5), -4.
/// 5, 0x3e9, and 0x3e8 when `News::has_urgent_items(human, 0)`: for 0x3e8 with parameter 0x3b set, the menu bar; for 5
/// with `News::count_unread(human, the filter byte)` ≠ 0 (its flag argument; the original pushes the dword whose low
/// byte is the filter), parameter k = 0 and parameter 4 = 1; otherwise parameters 1 and
/// 9 = 0, 7 = 1 and 4 = 1; -11. Otherwise the menu bar's command, then its object link.
/// Original: `news_screens_news_input` (0x7892b0, news_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_screens_news_input(raw: u32) -> i32 {
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    let human = read_usize((G_SCREEN_RECORDS_MANAGER as isize + slot * 0x300) as usize);
    let mode = get_screen_param(0);
    let filter = get_screen_param(1) as u8;
    let shown = get_screen_param(5);
    let unread = get_screen_param(6) as i32;
    let k = ((filter as i8 as i32 + 3) * 3) as u32;
    if mode != 0 && exe::screenmanager_is_manager_done(G_SCREEN_MANAGER, human as u32) == 0 {
        return -5;
    }
    let item = raw as i16;
    if item != -1 {
        let row = exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(k + 2));
        set_screen_param(k + 1, row);
    }
    if exe::news_count_unread(read_usize(G_NEWS), human, 0) != unread {
        covered!("tel36_6 news unread count changed");
        set_screen_param(4, 1);
        return -11;
    }
    let id = || tel26_item_id(item);
    if id() == 1 && tel26_item_value(item) as u8 != filter {
        covered!("tel36_6 news filter");
        set_screen_param(1, tel26_item_value(item) as usize);
        set_screen_param(4, 1);
        return -11;
    }
    if id() == 2 {
        set_screen_param(k, tel26_item_value(item) as usize);
        return -11;
    }
    if id() == 9 {
        set_screen_param(k, 0);
        return -11;
    }
    let urgent = id() == 5 || id() == 0x3e9 || (id() == 0x3e8 && {
        exe::news_has_urgent_items(read_usize(G_NEWS), human, 0) != 0
    });
    if !urgent {
        if id() == 3 {
            covered!("tel36_6 news action");
            let action: unsafe extern "C" fn(usize, u32) = core::mem::transmute(tel26_item_value(item) as usize);
            action(human, read_u32(shown + 0xd6));
            return -4;
        }
        return manager_menu_bar(raw);
    }
    if id() == 0x3e8 && get_screen_param(0x3b) != 0 {
        return manager_menu_bar(raw);
    }
    covered!("tel36_6 news urgent button");
    if id() == 5 && exe::news_count_unread(read_usize(G_NEWS), human, filter as u32) != 0 {
        set_screen_param(k, 0);
    } else {
        set_screen_param(1, 0);
        set_screen_param(9, 0);
        set_screen_param(7, 1);
    }
    set_screen_param(4, 1);
    -11
}

/// Rule: leaving the news screen frees its list.
///
/// `news_screens_news_exit()`: with a list in parameter 2, `free` and parameters 2 and 3 = 0.
/// Original: `news_screens_news_exit` (0x7897a0, news_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_screens_news_exit() {
    if get_screen_param(2) != 0 {
        covered!("tel36_6 news exit frees the list");
        exe::crt_free(get_screen_param(2));
        set_screen_param(2, 0);
        set_screen_param(3, 0);
    }
}

/// Rule: the send message screen opens for an addressee (or everybody) with an empty text.
///
/// `news_screens_open_send_message(addressee, news_id)`: `open_screen` with the builder 0x789990, the handler
/// 0x789bb0, single instance, no leave routine and 0; when it opens, `send_message_params`.
/// Original: `news_screens_open_send_message` (0x7897f0, news_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_screens_open_send_message(addressee: usize, news_id: u32) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x789990usize, 0x789bb0usize, 1, 0usize, 0) == 0 {
        return;
    }
    covered!("tel36_6 open send message");
    send_message_params(addressee, news_id);
}

/// Rule: replying to a news item opens the send message screen to the item's sender, or to everybody when the item has
/// no sender flag.
///
/// `news_screens_reply_to_news_item(manager, news_id)`: a local NewsItem (`NewsItem::NewsItem(0, 0)`),
/// `News::find_item_by_id(manager, news_id, &item)` on `G_NEWS` (its result is not looked at); slot 0 is the sender's
/// staff id; with slot 1 set the screen opens with the sender's record, otherwise with 0; when it opens,
/// `send_message_params`.
/// Original: `news_screens_reply_to_news_item` (0x789860, news_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_screens_reply_to_news_item(manager: usize, news_id: u32) {
    let mut item: TelNewsItem = [0; 0x40];
    let at = item.as_mut_ptr() as usize;
    exe::new_newsitem(at, 0, 0);
    exe::news_find_item_by_id(read_usize(G_NEWS), manager, news_id, at);
    let sender = exe::newsitem_get_slot(at, 0);
    let addressee = if exe::newsitem_get_slot(at, 1) != 0 {
        read_usize(G_STAFF_TABLE).wrapping_add(sender.wrapping_mul(staff::SIZE as u32) as usize)
    } else {
        0
    };
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x789990usize, 0x789bb0usize, 1, 0usize, 0) == 0 {
        return;
    }
    covered!("tel36_6 reply to a news item");
    send_message_params(addressee, news_id);
}

/// Rule: the send message screen shows the game's heading, "Send Message To <name>" (or "To All"), a text entry for
/// the message (100 characters) and Cancel / Send (Send only when there is text).
///
/// `news_screens_build_send_message()`: the addressee (parameter 0) and the text (parameter 1). The addressee's
/// background picture (`background_set_staff_picture`, 0, 0); the title `add_screen_title(get_heading_colour(0),
/// get_heading_colour(1), get_game_title())` (called in the order title, colour 1, colour 0); the menu bar (4, 0); the
/// subtitle "Send Message To <name>" (the name in 200 bytes, style 1) or "Send Message To All". A text entry item
/// (`Display::add_item`, flags 0x10018, (0x96, 0x136)-(0x2e4, 0x172), font 0x22, colours `JOB_COLOUR_BE04` /
/// `JOB_COLOUR_3184`, the text buffer, limit 100, id 6). The button area (0x64, 0x22b)-(0x316, 0x24e) (columns 3, 1):
/// "Cancel" (id 7) and "Send" (id 8) when the text is not empty, otherwise a grey "Send" text.
/// Original: `news_screens_build_send_message` (0x789990, news_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_screens_build_send_message() {
    let text = tel36_langlib();
    let weights: [u8; 2] = [3, 1];
    let addressee = get_screen_param(0);
    let message = get_screen_param(1);
    covered!("tel36_6 build send message");
    exe::background_set_staff_picture(addressee, 0, 0);
    let title = exe::get_game_title();
    let fore = exe::get_heading_colour(1) as u32;
    let back = exe::get_heading_colour(0) as u32;
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, title);
    exe::menubar_build(4, 0);
    if addressee != 0 {
        let mut name = [0u8; 0xc8];
        exe::database_format_staff_name(addressee, name.as_mut_ptr() as usize, 0xc8, 1);
        text(JOB_TEXT, 0xa695c0, name.as_ptr());
    } else {
        text(JOB_TEXT, 0xa695ac);
    }
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let d = current_human_display();
    tel26_add_item(d, [0x10018, 0x96, 0x136, 0x2e4, 0x172, 0, 0, 0x22, job_colour(JOB_COLOUR_BE04), 0, 0xc, 4,
        job_colour(JOB_COLOUR_3184), message as u32, 0x64, 6, 0, u32::MAX]);
    let d = current_human_display();
    let area = tel26_add_area(d, [0x64, 0x22b, 0x316, 0x24e, 2, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    text(JOB_TEXT, 0x98a1a0);
    tel26_area_button([area, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 7, 0]);
    text(JOB_TEXT, 0xa695a4);
    if read_u8(message) != 0 {
        tel26_area_button([area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 8, 0]);
    } else {
        tel26_area_text([area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0x2c, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);
    }
}

/// Rule: on the send message screen typing rebuilds it, Cancel drops the text, Send posts the message as news item
/// 5000 to the addressee, or to every other human manager when it is for all.
///
/// `news_screens_send_message_input(item)`: the slot's human, the addressee (parameter 0), the text (1) and the news id
/// (2). Id 6 (the text entry): -11. 7: `operator delete` of the text, -9. 8: a local NewsItem 0x1388
/// (`NewsItem::NewsItem(0x1388, 0)`) with slot 0 = the human's id, 1 = 1 with an addressee (0 for all), 2 = the news
/// id, 3 the text (`NewsItem::set_slot_string(3, text, 0)`); `News::deliver_to_manager(&item, addressee, 0)` on
/// `G_NEWS`, or for all to each of the 16 last staff records (the human managers) with `staff::CLASSIFICATION` 1 other
/// than the sender; then the text is deleted, -9. Otherwise the menu bar's command, then its object link. The text
/// pointer stays in parameter 1 after the delete (the screen closes).
/// Original: `news_screens_send_message_input` (0x789bb0, news_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_screens_send_message_input(raw: u32) -> i32 {
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    let human = read_usize((G_SCREEN_RECORDS_MANAGER as isize + slot * 0x300) as usize);
    let addressee = get_screen_param(0);
    let message = get_screen_param(1);
    let news_id = get_screen_param(2) as u32;
    let item = raw as i16;
    let drop_text = || {
        if message != 0 {
            exe::crt_operator_delete(message);
        }
        -9
    };
    match tel26_item_id(item) {
        6 => -11,
        7 => drop_text(),
        8 => {
            covered!("tel36_6 send the message");
            let mut news: TelNewsItem = [0; 0x40];
            let at = news.as_mut_ptr() as usize;
            exe::new_newsitem(at, 0x1388, 0);
            exe::newsitem_set_slot(at, 0, read_u32(human));
            exe::newsitem_set_slot(at, 1, (addressee != 0) as u32);
            exe::newsitem_set_slot(at, 2, news_id);
            exe::newsitem_set_slot_string(at, 3, message, core::ptr::null_mut());
            if addressee != 0 {
                exe::news_deliver_to_manager(read_usize(G_NEWS), at, addressee, 0);
            } else {
                for k in 0..0x10u32 {
                    let id = (read_u32(G_STAFF_COUNT)).wrapping_sub(k).wrapping_sub(1);
                    let manager = Staff(read_usize(G_STAFF_TABLE).wrapping_add(id.wrapping_mul(staff::SIZE as u32) as usize));
                    if manager.classification() == 1 && manager.addr() != human {
                        exe::news_deliver_to_manager(read_usize(G_NEWS), at, manager, 0);
                    }
                }
            }
            drop_text()
        }
        _ => manager_menu_bar(raw),
    }
}

/// Comparer for `news_item_get_date`: the date written side by side.
pub(crate) unsafe extern "thiscall" fn compare_news_item_get_date(item: usize, out: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_NEWS_ITEM_GET_DATE)(item, out);
    let mut mine = [0u32; 2];
    let n = news_item_get_date(item, mine.as_mut_ptr() as usize);
    let theirs = [read_u32(out), read_u32(out + 4)];
    SCOREBOARD.record(theirs == mine && r == out && n == mine.as_ptr() as usize, || format!("NewsItem::get_date({:#x}): original={:x?} rust={:x?}", item, theirs, mine));
    r
}

/// Comparer for `news_screens_news_input`.
pub(crate) unsafe extern "C" fn compare_news_screens_news_input(item: u32) -> i32 {
    // simplification: the actions, the redraws and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_NEWS_SCREENS_NEWS_INPUT)(item)
}

/// Comparer for `news_screens_news_exit`.
pub(crate) unsafe extern "C" fn compare_news_screens_news_exit() {
    // simplification: freeing the list does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(IDX_NEWS_SCREENS_NEWS_EXIT)()
}

/// Comparer for `news_screens_open_send_message`.
pub(crate) unsafe extern "C" fn compare_news_screens_open_send_message(addressee: usize, news_id: u32) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_NEWS_SCREENS_OPEN_SEND_MESSAGE)(addressee, news_id)
}

/// Comparer for `news_screens_reply_to_news_item`.
pub(crate) unsafe extern "C" fn compare_news_screens_reply_to_news_item(manager: usize, news_id: u32) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_NEWS_SCREENS_REPLY_TO_NEWS_ITEM)(manager, news_id)
}

/// Comparer for `news_screens_build_send_message`: the screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_news_screens_build_send_message() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_NEWS_SCREENS_BUILD_SEND_MESSAGE);
    compare_by_display_list_with("send message screen (news_screens_build_send_message)", &[(0xdd70a8, 0x100)], || o(), || news_screens_build_send_message());
}

/// Comparer for `news_screens_send_message_input`.
pub(crate) unsafe extern "C" fn compare_news_screens_send_message_input(item: u32) -> i32 {
    // simplification: posting the news and deleting the text do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_NEWS_SCREENS_SEND_MESSAGE_INPUT)(item)
}

/// `crt_sprintf` (0x944c9f, cdecl).
pub(crate) unsafe fn tel28_crt() -> Tel28FormatFn {
    core::mem::transmute(FN_CRT_SPRINTF)
}

/// The 0x3c parameters of the news screen as `news_screens_open_news_screen` leaves them for a record: 0 the
/// category (the second argument), 8 the filter buffer, every other one 0.
pub(crate) fn news_screen_planned_params(category: u32, filter: usize) -> [usize; 0x3c] {
    let mut p = [0usize; 0x3c];
    p[0] = category as usize;
    p[8] = filter;
    p
}

/// `Text::draw_wrapped` (0x4aeaf0, thiscall, ret 0x20): the Text into `area` from `x1` to `x2` from row `*row`
/// (moved on), with the font, the colour, the style and the last flag.
pub(crate) unsafe fn news_text_draw(text: &mut [usize; 2], area: u32, x1: u32, x2: u32, row: &mut i32, font: u32, colour: u32, style: u32) {
    (exe::text_draw_wrapped((text.as_mut_ptr()) as *mut usize as usize, area, (x1) as u32 as i32, (x2) as u32 as i32, row, font, colour, style, 0) as u32);
}

/// The inbox's default selection (no news item selected for the category): the last of the highest priority among
/// the items the filter keeps (−1 by default; an unread one 1 when the filter buffer exists, else its importance or 1
/// when it asks for a reply; any one that asks for a reply 1); the first kept item's id is the fallback. Returns the
/// selected id, its row and the first id.
///
/// With a filter (not empty), an item is kept when its headline or (the Text cleared) its body (`NewsItem::write_text`
/// 1, 1) matches; otherwise the Text is cleared and the item skipped. The first kept item's id goes to `first`; an
/// item at least as urgent as the best so far becomes the selection (parameter `k`, the row). With no selection at the
/// end, parameter `k` = the first id.
/// Original: part of `news_screens_build_news_screen` (0x7881e0, news_screens.cpp).
pub(crate) unsafe fn news_screen_pick_default(list: usize, count: i32, manager: usize, filter: usize, k: u32, text_a: &mut [usize; 2], data: &mut u32) -> (u32, i32, u32) {
    let (mut sel, mut sel_row, mut first): (u32, i32, u32) = (0, 0, u32::MAX);
    let mut best: i8 = 0;
    let mut item = list;
    for i in 0..count.max(0) {
        let mut priority: i8 = -1;
        let kept = if news_strlen(filter) != 0 {
            news_write_text(item, manager, text_a, 0, 0);
            if news_text_matches(text_a, filter) {
                true
            } else {
                news_text_clear(text_a);
                news_write_text(item, manager, text_a, 1, 1);
                if news_text_matches(text_a, filter) {
                    true
                } else {
                    news_text_clear(text_a);
                    false
                }
            }
        } else {
            true
        };
        if kept {
            if first == u32::MAX {
                first = read_u32(item + news_item::ID);
            }
            if read_u32(item + news_item::READ) == 0 {
                if get_screen_param(8) != 0 {
                    priority = 1;
                } else {
                    priority = read_u8(item + news_item::IMPORTANT) as i8;
                    if priority == 0 && news_item_action(item, manager, data) == 2 {
                        priority = 1;
                    }
                }
            }
            if news_item_action(item, manager, data) == 2 {
                priority = 1;
            }
            if priority >= best {
                sel = read_u32(item + news_item::ID);
                set_screen_param(k, sel as usize);
                sel_row = i;
                best = priority;
            }
        }
        item += NEWS_ITEM_SIZE;
    }
    if sel == 0 {
        set_screen_param(k, first as usize);
        sel = first;
    }
    (sel, sel_row, first)
}

/// The inbox list's rows: per news item the filter keeps, its date (today's year: the day and time of
/// `Date::get_weekday_string` 0x54b550 with the item's day slice; else `Date::get_short_date_string` 0x54a650), dark
/// blue, and its headline: the selected one as text in the open colour (marked read, `News::mark_item_read`, with
/// parameter 4 = 1 when it was unread; parameter 5 and the open item = it; the scroll row (parameter `k` + 1) moved to
/// show it), the others as buttons (id 2, the item's id), pale grey or the read colour. Returns the open item.
/// Original: part of `news_screens_build_news_screen` (0x7881e0, news_screens.cpp).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn news_screen_list_rows(list: usize, count: i32, manager: usize, filter: usize, k: u32, sel: u32, area: u32, visible: i32, first: u32,
    row: &mut i32, text_a: &mut [usize; 2], text_b: &mut [usize; 2]) -> usize {
    let mut shown = 0usize;
    let mut item = list;
    for _ in 0..count.max(0) {
        news_write_text(item, manager, text_a, 0, 0);
        let mut kept = true;
        if news_strlen(filter) != 0 && !news_text_matches(text_a, filter) {
            news_write_text(item, manager, text_b, 1, 1);
            if !news_text_matches(text_b, filter) {
                news_text_clear(text_b);
                news_text_clear(text_a);
                kept = false;
            }
        }
        if kept {
            let (mut date, mut shown_date) = ([0u8; 8], [0u8; 8]);
            let d = news_item_date(item, &mut date);
            let font = if read_u16(d + 2) == read_u16(G_CURRENT_DATE_YEAR) {
                // the original passes the day slice over the rest of the stack slot it stores it into (the list
                // pointer's upper bytes)
                let slice = (list as u32 & 0xffff_ff00) | read_u8(item + news_item::DAY_SLICE) as u32;
                let d = news_item_date(item, &mut shown_date);
                (exe::date_get_short_weekday_date_string_no_year((d) as usize as *mut u32, JOB_TEXT, 0x7d0, slice) as u32);
                NEWS_DATE_FONT_TODAY.read()
            } else {
                let d = news_item_date(item, &mut shown_date);
                (exe::date_get_short_date_string(d, JOB_TEXT, 0x7d0, 0xff) as u32);
                NEWS_DATE_FONT_OTHER.read()
            };
            tel26_area_text([area, 0, *row as u32, 0x30, job_colour(JOB_COLOUR_BE04), 0xc, font as u32, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);
            // the original keeps the colour word in the first id's slot: the dword carries its upper half
            let colour = (first & 0xffff_0000) | if read_u32(item + news_item::READ) != 0 { job_colour(NEWS_COLOUR_READ) } else { job_colour(JOB_COLOUR_3184) };
            let id = read_u32(item + news_item::ID);
            if id == sel {
                covered!("tel28_3 inbox open item");
                news_write_text(item, manager, text_a, 0, 0);
                tel28_crt()(JOB_TEXT, 0xa694f4, text_a[0]); // "   %s"
                tel26_area_text([area, 1, *row as u32, 0x210, job_colour(NEWS_COLOUR_OPEN), 1, 2, colour, JOB_TEXT as u32]);
                if read_u32(item + news_item::READ) == 0 {
                    exe::news_mark_item_read(read_usize(G_NEWS), manager, read_u32(item + news_item::ID), 1);
                    set_screen_param(4, 1);
                }
                set_screen_param(5, item);
                shown = item;
                let p = k + 1;
                while (get_screen_param(p) as i32) > *row {
                    set_screen_param(p, (get_screen_param(p) as i32 - 1) as isize as usize);
                }
                while (get_screen_param(p) as i32) + visible - 1 < *row {
                    set_screen_param(p, (get_screen_param(p) as i32 + 1) as isize as usize);
                }
            } else {
                tel28_crt()(JOB_TEXT, 0xa694f4, text_a[0]); // "   %s"
                tel26_area_button([area, 1, *row as u32, 2, 0, 1, 2, colour, JOB_TEXT as u32, 2, id]);
            }
            *row += 1;
        }
        item += NEWS_ITEM_SIZE;
    }
    shown
}

/// The lower half of the inbox: the filter, Next Unread, the open item's headline and text, its action button, the
/// menu bar, Back/Next and the unread count (parameter 6).
/// Original: part of `news_screens_build_news_screen` (0x7881e0, news_screens.cpp).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn news_screen_lower(top: i32, bottom: i32, manager: usize, filter: usize, shown: usize, weights: u32, text_a: &mut [usize; 2], data: &mut u32, reset: u32) {
    let y2 = top + 0x5f;
    let filter_area = tel26_add_area(current_human_display(), [0x195, y2 as u32, 0x28f, (y2 + 0x14) as u32, 2, weights, 1, 0, 2, 0, u32::MAX]);
    tel28_lang()(JOB_TEXT, 0xa694e8); // "Filter :"
    tel26_area_text([filter_area, 0, 0, 1, 0, 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);
    tel26_add_item(current_human_display(), [0x80008, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, job_colour(JOB_COLOUR_BDE8), filter as u32, 0x19, 9, 0, filter_area]);
    tel28_lang()(JOB_TEXT, 0xa694dc); // "Next Unread"
    if exe::news_count_unread(read_usize(G_NEWS), manager, 0) != 0 {
        tel26_add_button([0x28f, y2 as u32, 0x30c, (y2 + 0x14) as u32, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 5, 0]);
    } else {
        tel28_text_item([0x28f, y2 as u32, 0x30c, (y2 + 0x14) as u32, 0x30, job_colour(JOB_COLOUR_317E), 0x2c, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 1]);
    }
    let y3 = y2 + 0x19;
    let subject = tel26_add_area(current_human_display(), [0x6e, y3 as u32, 0x30c, (y3 + 0x23) as u32, 1, 0, 1, 0, 2, 0, u32::MAX]);
    let mut row: i32 = 0;
    if shown != 0 {
        news_write_text(shown, manager, text_a, 0, 0);
        news_text_draw(text_a, subject, 0x78, 0x302, &mut row, 3, job_colour(JOB_COLOUR_BDEC), 3);
    }
    let y4 = y3 + 0x28;
    let lines = ((bottom - y4 - 0x10) / exe::font_get_height(3)) as u8;
    let body = tel26_add_area(current_human_display(), [0x6e, y4 as u32, 0x30c, bottom as u32, 1, 0, (reset & 0xffff_ff00) | lines as u32, 0, 2, 0, u32::MAX]);
    row = 0;
    if shown != 0 {
        news_write_text(shown, manager, text_a, 1, 0);
        news_text_draw(text_a, body, 0x78, NEWS_BODY_X2.read_unaligned(), &mut row, 3, job_colour(JOB_COLOUR_3184), 0);
    }
    let mut reply = 0u32;
    if shown != 0 {
        match news_item_action(shown, manager, data) {
            0 => {}
            2 => {
                let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
                if read_u16((NEWS_SLOT_REPLY_LOCK as isize + slot * 0x300) as usize) == 0 {
                    covered!("tel28_3 inbox reply button");
                    let style = if read_u32(read_usize(G_CONFIG) + 4) & NEWS_CONFIG_REPLY_STYLE != 0 { 0x802 } else { 2 };
                    tel26_add_item(current_human_display(), [style, 0x6e, y2 as u32, 0xeb, (y2 + 0x14) as u32, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), job_colour(JOB_COLOUR_317E),
                        0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0, 3, *data, u32::MAX]);
                    reply = 1;
                } else {
                    set_screen_param(0x3b, 1);
                    tel28_text_item([0x6e, y2 as u32, 0xeb, (y2 + 0x14) as u32, 0x30, job_colour(JOB_COLOUR_317E), 0x2c, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 1]);
                }
            }
            _ => {
                tel26_add_button([0x6e, y2 as u32, 0xeb, (y2 + 0x14) as u32, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 3, *data]);
            }
        }
    }
    exe::menubar_build(4, reply);
    tel26_back_next();
    set_screen_param(6, exe::news_count_unread(read_usize(G_NEWS), manager, 0) as isize as usize);
}

/// Whether the inbox build would change the game or run only once: a refresh, a reset, no list yet (it allocates
/// one), no selection (the default may pick an unread item) or a selected item still unread (it gets marked read).
pub(crate) unsafe fn news_screen_build_changes_game() -> bool {
    if get_screen_param(0) != 0 || get_screen_param(4) != 0 || get_screen_param(2) == 0 {
        return true;
    }
    let k = (get_screen_param(1) as u8 as i8 as i32 * 3 + 9) as u32;
    let sel = get_screen_param(k) as u32;
    if sel == 0 {
        return true;
    }
    let (list, count) = (get_screen_param(2), get_screen_param(3) as i32);
    (0..count.max(0) as usize).any(|i| {
        let item = list + i * NEWS_ITEM_SIZE;
        read_u32(item + news_item::ID) == sel && read_u32(item + news_item::READ) == 0
    })
}

/// The parameters of the send message screen once it is open: 0 = the addressee (a staff record, 0 for all), 1 = a new
/// 101-byte text buffer, empty, 2 = the news item answered.
pub(crate) unsafe fn send_message_params(addressee: usize, news_id: u32) {
    set_screen_param(0, addressee);
    set_screen_param(1, exe::crt_operator_new(0x65));
    set_screen_param(2, news_id as usize);
    write_u8(get_screen_param(1), 0);
}

/// `ScreenManager::add_text_item` (0x803fd0, thiscall, ret 0x2c): x1, y1, x2, y2, the font, the flags, the style, the
/// colour, the text, the id and the value; returns eax.
pub(crate) unsafe fn tel28_text_item(f: [u32; 11]) -> u32 {
    type TextItem = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: TextItem = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    g(G_SCREEN_MANAGER, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10])
}

/// The length of the C string at `s` (the original's `repne scasb`).
pub(crate) unsafe fn news_strlen(s: usize) -> usize {
    let mut n = 0;
    while read_u8(s + n) != 0 {
        n += 1;
    }
    n
}

/// `NewsItem::write_text` (0x77bc80, thiscall, ret 0x10): the headline (`body` 0) or the body of a news item for the
/// manager into the Text `text`.
pub(crate) unsafe fn news_write_text(item: usize, manager: usize, text: &mut [usize; 2], body: u32, flag: u32) {
    exe::newsitem_write_text(item, manager, text.as_mut_ptr(), body, flag);
}

/// `Text::clear` (0x4afa40, thiscall).
pub(crate) unsafe fn news_text_clear(text: &mut [usize; 2]) {
    exe::text_clear(text.as_mut_ptr());
}

/// The filter test of the inbox (search_eng 0x806260, cdecl(text, filter, 0)): whether the Text's string matches.
pub(crate) unsafe fn news_text_matches(text: &[usize; 2], filter: usize) -> bool {
    exe::search_text_contains(text[0], filter, 0) != 0
}

/// The news item's date (`NewsItem::get_date` 0x789280, thiscall, ret 4) into `out`; returns the date.
pub(crate) unsafe fn news_item_date(item: usize, out: &mut [u8; 8]) -> usize {
    exe::newsitem_get_date(item, out.as_mut_ptr() as usize)
}

/// The action button of a news item (`NewsItem::get_action_button`, its label in [`JOB_TEXT`], its data in `data`):
/// 0 none, 2 a reply.
pub(crate) unsafe fn news_item_action(item: usize, manager: usize, data: &mut u32) -> u8 {
    exe::newsitem_get_action_button(item, manager, JOB_TEXT, 0x7d0, data)
}

/// "E:\dev\CM3\cm3 00-01\cm3\code\news_screens.cpp", the path the news_screens.cpp error boxes print.
pub(crate) const NEWS_SCREENS_CPP: usize = 0xa694ac;

/// The placeholder the inbox's filter buffer starts with ("#;.,234-][+-=-"): the build clears a filter holding it.
pub(crate) const NEWS_FILTER_PLACEHOLDER: usize = 0xa6949c;

/// The colours of the inbox list: a read news item (0xae3154) and the open item's date (0xaebdcc) [inference].
pub(crate) const NEWS_COLOUR_READ: usize = 0xae3154;

pub(crate) const NEWS_COLOUR_OPEN: usize = 0xaebdcc;

/// A word per screen slot (0x300 bytes each) at 0xb74c36: when set, the inbox's reply button is drawn as plain text
/// and parameter 0x3b raised [inference: a reply already pending for the slot].
pub(crate) const NEWS_SLOT_REPLY_LOCK: usize = 0xb74c36;

/// The game options' flag (+4 of [`G_CONFIG`]) that gives the reply button the 0x800 style [inference].
pub(crate) const NEWS_CONFIG_REPLY_STYLE: u32 = 0x20000;

/// The immediates the 1080 exe changes in the news build: the font of the list's date cell (1 → 2, the push at
/// 0x788a32 and 0x788a85) and the width of the open item's body (0x302 → 0x44c, the push at 0x78900e).
pub(crate) const NEWS_DATE_FONT_TODAY: *const u8 = 0x788a33 as *const u8;

pub(crate) const NEWS_DATE_FONT_OTHER: *const u8 = 0x788a86 as *const u8;

pub(crate) const NEWS_BODY_X2: *const u32 = 0x78900f as *const u32;

/// A news item's size in the inbox vector (0xdf bytes, `news_item`).
pub(crate) const NEWS_ITEM_SIZE: usize = 0xdf;

/// The NewsItem (news.cpp) date: the Date (8 bytes) at +0xcd.
pub(crate) const NEWS_ITEM_DATE: usize = 0xcd;

/// A local NewsItem buffer (the item is 0xda bytes and more; the originals keep it in frames of 0xe0 bytes and up).
pub(crate) type TelNewsItem = [u32; 0x40];
