// ---------- batch %%LEVA%%: the transfer regulations screen: opening it from a news item (new_transfer_rule_screens.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\new_transfer_rule_screens.cpp" (the error box) and the screen's build and input
/// functions (transfer_rules_info_screen_display, transfer_rules_info_screen_handle_click).
pub(crate) const NEW_TRANSFER_RULE_SCREENS_CPP: usize = 0xa649e8;
pub(crate) const FN_TRANSFER_RULES_INFO_SCREEN_DISPLAY: usize = 0x77b850;
pub(crate) const FN_TRANSFER_RULES_INFO_SCREEN_HANDLE_CLICK: usize = 0x77bb30;

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
        let find: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(0x786b70usize);
        if find(read_usize(G_NEWS), manager, news_id, item) == 0 {
            show_error_at(NEW_TRANSFER_RULE_SCREENS_CPP, 0x4a);
            return;
        }
    }
    covered!("tel29_4 transfer rules screen opened");
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0, 1usize, 0); // tel29_4: the first page
    let slot = if manager != 0 { exe::newsitem_get_slot(item, 0) } else { 0 };
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 1, slot as usize, 0);
}

// simplification: opens a screen (the history allocates a node): it does not run twice. Original only; the plan: the
// new current screen is this one, at page 1.
/// Comparer for `transfer_rules_info_screen_open`.
pub(crate) unsafe extern "C" fn compare_transfer_rules_info_screen_open(manager: usize, news_id: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32)>(%%I0%%)(manager, news_id);
    let node = read_usize(G_SCREEN_MANAGER + read_i16(G_SCREEN_MANAGER + screen_manager::CURRENT_SLOT) as isize as usize * 0x300 + screen_manager::PARAM_BLOCK);
    if node == 0 || read_usize(node) != FN_TRANSFER_RULES_INFO_SCREEN_DISPLAY {
        return; // not opened (or the news item was not found before the parameters)
    }
    let page = exe::screenmanager_get_screen_param(G_SCREEN_MANAGER, 0);
    SCOREBOARD.record(page == 1 || (manager != 0 && page == 0), || format!("transfer_rules_info_screen_open({:#x}, {}): page {}", manager, news_id, page));
}
