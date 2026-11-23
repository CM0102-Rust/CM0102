use super::*;

/// Opens the transfer regulations information screen at its first page and, from a news item, passes the item's first
/// slot as the screen's second parameter.
///
/// A local news item (NewsItem::NewsItem `exe::new_newsitem`(item, 0, 0)). The screen opens
/// (`exe::screenmanager_open_screen` on [`G_SCREEN_MANAGER`] with `FN_TRANSFER_RULES_INFO_SCREEN_DISPLAY` and
/// `FN_TRANSFER_RULES_INFO_SCREEN_HANDLE_CLICK`, no flag, leave or exit function); if it does not, nothing more. With a
/// manager, his news item `news_id` is looked up (News::find_item_by_id 0x786b70 on [`G_NEWS`], thiscall, into the local
/// item); not found: the error box (`new_transfer_rule_screens.cpp` 74) and nothing more. Parameter 0 = 1 (the page);
/// parameter 1 = the item's slot 0 (`exe::newsitem_get_slot`) with a manager, 0 without. The original keeps the item
/// under an SEH frame; nothing here throws.
/// Original: `transfer_rules_info_screen_open` (0x77b700, new_transfer_rule_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_rules_info_screen_open(manager: usize, news_id: u32) {
    let mut news = [0u32; 0x38];
    let item = news.as_mut_ptr() as usize;
    exe::new_newsitem(item, 0, 0);
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, FN_TRANSFER_RULES_INFO_SCREEN_DISPLAY, FN_TRANSFER_RULES_INFO_SCREEN_HANDLE_CLICK, 0, 0usize, 0) == 0 {
        return;
    }
    if manager != 0 {
        if exe::news_find_item_by_id(read_usize(G_NEWS), manager, news_id, item) == 0 {
            show_error_at(NEW_TRANSFER_RULE_SCREENS_CPP, 0x4a);
            return;
        }
    }
    covered!("tel29_4 transfer rules screen opened");
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0, 1usize, 0); // tel29_4: the first page
    let slot = if manager != 0 { exe::newsitem_get_slot(item, 0) } else { 0 };
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 1, slot as usize, 0);
}

/// Comparer for `transfer_rules_info_screen_open`.
pub(crate) unsafe extern "C" fn compare_transfer_rules_info_screen_open(manager: usize, news_id: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_TRANSFER_RULES_INFO_SCREEN_OPEN)(manager, news_id);
    let node = read_usize(G_SCREEN_MANAGER + read_i16(G_SCREEN_MANAGER + screen_manager::CURRENT_SLOT) as isize as usize * 0x300 + screen_manager::PARAM_BLOCK);
    if node == 0 || read_usize(node) != FN_TRANSFER_RULES_INFO_SCREEN_DISPLAY {
        return; // not opened (or the news item was not found before the parameters)
    }
    let page = exe::screenmanager_get_screen_param(G_SCREEN_MANAGER, 0);
    SCOREBOARD.record(page == 1 || (manager != 0 && page == 0), || format!("transfer_rules_info_screen_open({:#x}, {}): page {}", manager, news_id, page));
}

/// Rule: the transfer regulations screen (current or proposed) has four tabs, Overview, Transfer Windows, Young
/// Players and Contract Protection, each with its explanation text, and the Back / Next buttons.
///
/// `transfer_rules_info_screen_display()`: four MenuItems built by the CRT vector iterator (0x4568f0, destroyed by
/// 0x5a53c0 at the end). The tab of parameter 0 (a signed char), the "current" flag of parameter 1. The club shown: the
/// slot's human's `staff::CLUB_JOB`, else the club of `TRANSFER_RULES_DEFAULT_CLUB_KEY`. The menu bar (4, 0); the title
/// in that club's kit 1 colours: its name, or "Information" when the human has no club; the subtitle "Current Transfer
/// Regulations" (parameter 1 set) or "Proposed Transfer Regulations". The tabs: "Overview" (with the empty text
/// 0xad9c64), "Transfer Windows", "Young Players", "Contract Protection", each link 1 and value 1..4; `gui_add_tab_bar`
/// (4 tabs, the chosen one, the top 0x8c and the bottom 0x221 it moves, 1). The explanation by the tab: 1 the current
/// (0xa65188) or proposed (0xa65020) overview, 3 0xa64e30, 4 0xa64c24, 2 0xa64a24 (language 0: the plain C text
/// 0xa64b24); another tab no text. The text area (0x6e, top)-(0x30c, bottom) (font 4, pale grey), then the Back / Next
/// buttons (`tel26_back_next`).
/// Original: `transfer_rules_info_screen_display` (0x77b850, new_transfer_rule_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_rules_info_screen_display() {
    let text = tel36_langlib();
    let mut tabs = [0u8; TRANSFER_RULES_TAB_SIZE * 4];
    let t = tabs.as_mut_ptr() as usize;
    exe::crt_vector_constructor_iterator(t, TRANSFER_RULES_TAB_SIZE as u32, 4, 0x4568f0usize, 0x5a53c0usize);
    let table = get_screen_param(0) as u8 as i8;
    let current = get_screen_param(1);
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    let human = Staff(read_usize((G_SCREEN_RECORDS_MANAGER as isize + slot * 0x300) as usize));
    let own = human.club_job().addr();
    let shown = Club(if own != 0 {
        own
    } else {
        read_usize(G_CLUB_TABLE).wrapping_add(read_u32(TRANSFER_RULES_DEFAULT_CLUB_KEY).wrapping_mul(club::SIZE as u32) as usize)
    });
    covered!("tel36_5 transfer regulations screen");
    exe::menubar_build(4, 0);
    let (mut fore, mut back) = (0u32, 0u32);
    exe::database_get_club_kit_colours(shown, 1, &mut fore, &mut back);
    let title = if human.club_job().addr() != 0 { shown.addr() + club::NAME } else { 0xa6543c };
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, title);
    text(JOB_TEXT, if current == 0 { 0xa6541c } else { 0xa653fc });
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let (mut top, mut bottom) = (0x8ci32, 0x221i32);
    for (k, name) in [0xa653bcusize, 0xa653a8, 0xa65398, 0xa65384].into_iter().enumerate() {
        let item = t + k * TRANSFER_RULES_TAB_SIZE;
        text(item + TRANSFER_RULES_TAB_TEXT, name, 0xad9c64usize);
        write_u32(item + TRANSFER_RULES_TAB_LINK, 1);
        write_u32(item + TRANSFER_RULES_TAB_VALUE, k as u32 + 1);
    }
    exe::gui_add_tab_bar(t, 4, table as i32, &mut top, &mut bottom, 1);
    let explanation = match table {
        1 => Some(if current != 0 { 0xa65188 } else { 0xa65020 }),
        3 => Some(0xa64e30),
        4 => Some(0xa64c24),
        2 => {
            if exe::langlib_get_language() == 0 {
                let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
                format(JOB_TEXT, 0xa64b24usize);
                Some(0)
            } else {
                Some(0xa64a24)
            }
        }
        _ => None,
    };
    if let Some(e) = explanation {
        if e != 0 {
            text(JOB_TEXT, e);
        }
        tel26_text_area([0x6e, top as u32, 0x30c, bottom as u32, 2, 0, 0x10, 4, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0]);
    }
    tel26_back_next();
    exe::crt_vector_destructor_iterator(t, TRANSFER_RULES_TAB_SIZE as u32, 4, 0x5a53c0usize);
}

/// Rule: a tab click on the transfer regulations screen shows that tab.
///
/// `transfer_rules_info_screen_handle_click(item)`: id 1 (`tel26_item_id`): parameter 0 = the widget's data
/// (`tel26_item_value`), -11; otherwise the menu bar's command, then its object link.
/// Original: `transfer_rules_info_screen_handle_click` (0x77bb30, new_transfer_rule_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_rules_info_screen_handle_click(raw: u32) -> i32 {
    let item = raw as i16;
    if tel26_item_id(item) == 1 {
        covered!("tel36_5 transfer regulations tab");
        set_screen_param(0, tel26_item_value(item) as usize);
        return -11;
    }
    manager_menu_bar(raw)
}

/// Comparer for `transfer_rules_info_screen_display`: the screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_transfer_rules_info_screen_display() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_RULES_INFO_SCREEN_DISPLAY);
    compare_by_display_list_with("transfer regulations screen (transfer_rules_info_screen_display)", &[], || o(), || transfer_rules_info_screen_display());
}

/// Comparer for `transfer_rules_info_screen_handle_click`: the tab choice side by side over the screen params; the
/// menu bar only the original.
pub(crate) unsafe extern "C" fn compare_transfer_rules_info_screen_handle_click(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRANSFER_RULES_INFO_SCREEN_HANDLE_CLICK);
    if tel26_item_id(item as i16) != 1 {
        // simplification: the menu bar commands do not run twice; proven in Rust mode
        return o(item);
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&screen_param_regions(), || o(item), || transfer_rules_info_screen_handle_click(item));
    SCOREBOARD.record(same && r == n, || format!("transfer regulations table: original={} rust={} differences {:?}", r, n, first_differences(&mo, &mr)));
    r
}

/// "E:\dev\CM3\cm3 00-01\cm3\code\new_transfer_rule_screens.cpp" (the error box) and the screen's build and input
/// functions (transfer_rules_info_screen_display, transfer_rules_info_screen_handle_click).
pub(crate) const NEW_TRANSFER_RULE_SCREENS_CPP: usize = 0xa649e8;

pub(crate) const FN_TRANSFER_RULES_INFO_SCREEN_DISPLAY: usize = 0x77b850;

pub(crate) const FN_TRANSFER_RULES_INFO_SCREEN_HANDLE_CLICK: usize = 0x77bb30;

/// The MenuItem (club_screens.cpp, 0xd4 bytes) of a tab bar: its constructor and the no-op destructor, run by the CRT
/// vector iterators; the text at +4, the link kind at +0xcc and the tab's value at +0xd0.
pub(crate) const TRANSFER_RULES_TAB_SIZE: usize = 0xd4;

pub(crate) const TRANSFER_RULES_TAB_TEXT: usize = 0x4;

pub(crate) const TRANSFER_RULES_TAB_LINK: usize = 0xcc;

pub(crate) const TRANSFER_RULES_TAB_VALUE: usize = 0xd0;
