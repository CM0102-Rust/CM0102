// ---------- batch %%LEVA%%: the news screen (news_screens.cpp): opening the inbox and building it ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\news_screens.cpp", the path the news_screens.cpp error boxes print.
pub(crate) const NEWS_SCREENS_CPP: usize = 0xa694ac;
/// The placeholder the inbox's filter buffer starts with ("#;.,234-][+-=-"): the build clears a filter holding it.
const NEWS_FILTER_PLACEHOLDER: usize = 0xa6949c;
/// The colours of the inbox list: a read news item (0xae3154) and the open item's date (0xaebdcc) [inference].
const NEWS_COLOUR_READ: usize = 0xae3154;
const NEWS_COLOUR_OPEN: usize = 0xaebdcc;
/// A word per screen slot (0x300 bytes each) at 0xb74c36: when set, the inbox's reply button is drawn as plain text
/// and parameter 0x3b raised [inference: a reply already pending for the slot].
const NEWS_SLOT_REPLY_LOCK: usize = 0xb74c36;
/// The game options' flag (+4 of [`G_CONFIG`]) that gives the reply button the 0x800 style [inference].
const NEWS_CONFIG_REPLY_STYLE: u32 = 0x20000;
/// The immediates the 1080 exe changes in the news build: the font of the list's date cell (1 → 2, the push at
/// 0x788a32 and 0x788a85) and the width of the open item's body (0x302 → 0x44c, the push at 0x78900e).
const NEWS_DATE_FONT_TODAY: *const u8 = 0x788a33 as *const u8;
const NEWS_DATE_FONT_OTHER: *const u8 = 0x788a86 as *const u8;
const NEWS_BODY_X2: *const u32 = 0x78900f as *const u32;
/// A news item's size in the inbox vector (0xdf bytes, `news_item`).
const NEWS_ITEM_SIZE: usize = 0xdf;

/// The length of the C string at `s` (the original's `repne scasb`).
unsafe fn news_strlen(s: usize) -> usize {
    let mut n = 0;
    while read_u8(s + n) != 0 {
        n += 1;
    }
    n
}

/// The 0x3c parameters of the news screen as `news_screens_open_news_screen` leaves them for a record: 0 the
/// category (the second argument), 8 the filter buffer, every other one 0.
fn news_screen_planned_params(category: u32, filter: usize) -> [usize; 0x3c] {
    let mut p = [0usize; 0x3c];
    p[0] = category as usize;
    p[8] = filter;
    p
}

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
    original_fn::<unsafe extern "C" fn(usize, u32)>(%%I0%%)(record, category);
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

/// `NewsItem::write_text` (0x77bc80, thiscall, ret 0x10): the headline (`body` 0) or the body of a news item for the
/// manager into the Text `text`.
unsafe fn news_write_text(item: usize, manager: usize, text: &mut [usize; 2], body: u32, flag: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, *mut usize, u32, u32) -> u32 = core::mem::transmute(0x77bc80usize);
    f(item, manager, text.as_mut_ptr(), body, flag);
}

/// `Text::clear` (0x4afa40, thiscall).
unsafe fn news_text_clear(text: &mut [usize; 2]) {
    let f: unsafe extern "thiscall" fn(*mut usize) -> u32 = core::mem::transmute(0x4afa40usize);
    f(text.as_mut_ptr());
}

/// The filter test of the inbox (search_eng 0x806260, cdecl(text, filter, 0)): whether the Text's string matches.
unsafe fn news_text_matches(text: &[usize; 2], filter: usize) -> bool {
    let f: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x806260usize);
    f(text[0], filter, 0) != 0
}

/// `Text::draw_wrapped` (0x4aeaf0, thiscall, ret 0x20): the Text into `area` from `x1` to `x2` from row `*row`
/// (moved on), with the font, the colour, the style and the last flag.
unsafe fn news_text_draw(text: &mut [usize; 2], area: u32, x1: u32, x2: u32, row: &mut i32, font: u32, colour: u32, style: u32) {
    let f: unsafe extern "thiscall" fn(*mut usize, u32, u32, u32, *mut i32, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x4aeaf0usize);
    f(text.as_mut_ptr(), area, x1, x2, row, font, colour, style, 0);
}

/// The news item's date (`NewsItem::get_date` 0x789280, thiscall, ret 4) into `out`; returns the date.
unsafe fn news_item_date(item: usize, out: &mut [u8; 8]) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x789280usize);
    f(item, out.as_mut_ptr() as usize)
}

/// The action button of a news item (`NewsItem::get_action_button`, its label in [`JOB_TEXT`], its data in `data`):
/// 0 none, 2 a reply.
unsafe fn news_item_action(item: usize, manager: usize, data: &mut u32) -> u8 {
    exe::newsitem_get_action_button(item, manager, JOB_TEXT, 0x7d0, data)
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
unsafe fn news_screen_pick_default(list: usize, count: i32, manager: usize, filter: usize, k: u32, text_a: &mut [usize; 2], data: &mut u32) -> (u32, i32, u32) {
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
unsafe fn news_screen_list_rows(list: usize, count: i32, manager: usize, filter: usize, k: u32, sel: u32, area: u32, visible: i32, first: u32,
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
                let weekday: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x54b550usize);
                weekday(d, JOB_TEXT, 0x7d0, slice);
                NEWS_DATE_FONT_TODAY.read()
            } else {
                let d = news_item_date(item, &mut shown_date);
                let short: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x54a650usize);
                short(d, JOB_TEXT, 0x7d0, 0xff);
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
    let manager = tel26_slot_manager().addr();
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
        let urgent: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x787350usize);
        if urgent(read_usize(G_NEWS), manager, 1) == 0 && exe::screenmanager_is_manager_done(G_SCREEN_MANAGER, manager as u32) == 0 {
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
        let inbox: unsafe extern "thiscall" fn(usize, usize, *mut i32, u32) -> usize = core::mem::transmute(0x786960usize);
        list = inbox(read_usize(G_NEWS), manager, &mut count, tab as u32);
        set_screen_param(2, list);
        set_screen_param(3, count as isize as usize);
    }
    exe::background_set_random_picture();
    let (mut front, mut back) = (0u32, 0u32);
    let team = if read_usize(manager + staff::CLUB_JOB) != 0 { read_usize(manager + staff::CLUB_JOB) } else { read_usize(manager + staff::NATIONAL_JOB) };
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
            let (s, r, f) = news_screen_pick_default(list, count, manager, filter, k, &mut text_a, &mut data);
            sel = s;
            sel_row = r;
            first = f;
        }
        shown = news_screen_list_rows(list, count, manager, filter, k, sel, area, visible as i32, first, &mut row, &mut text_a, &mut text_b);
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
    news_screen_lower(top, bottom, manager, filter, shown, weights.as_ptr() as u32, &mut text_a, &mut data, reset);
    exe::drop_text(text_b.as_mut_ptr());
    exe::drop_text(text_a.as_mut_ptr());
}

/// The lower half of the inbox: the filter, Next Unread, the open item's headline and text, its action button, the
/// menu bar, Back/Next and the unread count (parameter 6).
/// Original: part of `news_screens_build_news_screen` (0x7881e0, news_screens.cpp).
#[allow(clippy::too_many_arguments)]
unsafe fn news_screen_lower(top: i32, bottom: i32, manager: usize, filter: usize, shown: usize, weights: u32, text_a: &mut [usize; 2], data: &mut u32, reset: u32) {
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
unsafe fn news_screen_build_changes_game() -> bool {
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

/// Comparer for `news_screens_build_news_screen`: side by side over the display list when the build only reads (an
/// open list with a read item selected); otherwise only the original (it marks news read, allocates or frees the list
/// or stops on a refresh).
pub(crate) unsafe extern "C" fn compare_news_screens_build_news_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    if news_screen_build_changes_game() {
        // simplification: marking news read, the list allocation and the refresh do not run twice; proven in Rust
        // mode and on the next build of the same screen
        return o();
    }
    compare_match_screen_build("news screen", &[], || o(), || news_screens_build_news_screen());
}
