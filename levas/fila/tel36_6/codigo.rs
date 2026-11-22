// ---------- batch %%LEVA%%: the news screen input and the send message screen (news_screens.cpp) ----------

/// The NewsItem (news.cpp) date: the Date (8 bytes) at +0xcd.
const NEWS_ITEM_DATE: usize = 0xcd;
/// A local NewsItem buffer (the item is 0xda bytes and more; the originals keep it in frames of 0xe0 bytes and up).
type TelNewsItem = [u32; 0x40];

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
        let has: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x787350usize);
        has(read_usize(G_NEWS), human, 0) != 0
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

/// The parameters of the send message screen once it is open: 0 = the addressee (a staff record, 0 for all), 1 = a new
/// 101-byte text buffer, empty, 2 = the news item answered.
unsafe fn send_message_params(addressee: usize, news_id: u32) {
    set_screen_param(0, addressee);
    set_screen_param(1, exe::crt_operator_new(0x65));
    set_screen_param(2, news_id as usize);
    write_u8(get_screen_param(1), 0);
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
    let find: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(0x786b70usize);
    find(read_usize(G_NEWS), manager, news_id, at);
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
    let background: unsafe extern "C" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x41c790usize);
    background(addressee, 0, 0);
    let game_title: unsafe extern "C" fn() -> usize = core::mem::transmute(0x839570usize);
    let title = game_title();
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
            let deliver: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_NEWS_DELIVER_TO_MANAGER);
            if addressee != 0 {
                deliver(read_usize(G_NEWS), at, addressee, 0);
            } else {
                for k in 0..0x10u32 {
                    let id = (read_u32(G_STAFF_COUNT)).wrapping_sub(k).wrapping_sub(1);
                    let manager = read_usize(G_STAFF_TABLE).wrapping_add(id.wrapping_mul(staff::SIZE as u32) as usize);
                    if read_u8(manager + staff::CLASSIFICATION) == 1 && manager != human {
                        deliver(read_usize(G_NEWS), at, manager, 0);
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
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(item, out);
    let mut mine = [0u32; 2];
    let n = news_item_get_date(item, mine.as_mut_ptr() as usize);
    let theirs = [read_u32(out), read_u32(out + 4)];
    SCOREBOARD.record(theirs == mine && r == out && n == mine.as_ptr() as usize, || format!("NewsItem::get_date({:#x}): original={:x?} rust={:x?}", item, theirs, mine));
    r
}

/// Comparer for `news_screens_news_input`.
pub(crate) unsafe extern "C" fn compare_news_screens_news_input(item: u32) -> i32 {
    // simplification: the actions, the redraws and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I1%%)(item)
}

/// Comparer for `news_screens_news_exit`.
pub(crate) unsafe extern "C" fn compare_news_screens_news_exit() {
    // simplification: freeing the list does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(%%I2%%)()
}

/// Comparer for `news_screens_open_send_message`.
pub(crate) unsafe extern "C" fn compare_news_screens_open_send_message(addressee: usize, news_id: u32) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize, u32)>(%%I3%%)(addressee, news_id)
}

/// Comparer for `news_screens_reply_to_news_item`.
pub(crate) unsafe extern "C" fn compare_news_screens_reply_to_news_item(manager: usize, news_id: u32) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize, u32)>(%%I4%%)(manager, news_id)
}

/// Comparer for `news_screens_build_send_message`: the screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_news_screens_build_send_message() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I5%%);
    compare_by_display_list_with("send message screen (news_screens_build_send_message)", &[(0xdd70a8, 0x100)], || o(), || news_screens_build_send_message());
}

/// Comparer for `news_screens_send_message_input`.
pub(crate) unsafe extern "C" fn compare_news_screens_send_message_input(item: u32) -> i32 {
    // simplification: posting the news and deleting the text do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I6%%)(item)
}
