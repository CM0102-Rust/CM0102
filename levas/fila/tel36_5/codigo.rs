// ---------- batch %%LEVA%%: the transfer regulations information screen (new_transfer_rule_screens.cpp) ----------

/// The fixed key whose club id titles the screen when the human has no club (Brighton & Hove Albion's key, see
/// `index.rs`).
const TRANSFER_RULES_DEFAULT_CLUB_KEY: usize = 0x9d025c;
/// The MenuItem (club_screens.cpp, 0xd4 bytes) of a tab bar: its constructor and the no-op destructor, run by the CRT
/// vector iterators; the text at +4, the link kind at +0xcc and the tab's value at +0xd0.
const TRANSFER_RULES_TAB_SIZE: usize = 0xd4;
const TRANSFER_RULES_TAB_TEXT: usize = 0x4;
const TRANSFER_RULES_TAB_LINK: usize = 0xcc;
const TRANSFER_RULES_TAB_VALUE: usize = 0xd0;

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
    let tab = get_screen_param(0) as u8 as i8;
    let current = get_screen_param(1);
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    let human = read_usize((G_SCREEN_RECORDS_MANAGER as isize + slot * 0x300) as usize);
    let own = read_usize(human + staff::CLUB_JOB);
    let shown = if own != 0 {
        own
    } else {
        read_usize(G_CLUB_TABLE).wrapping_add(read_u32(TRANSFER_RULES_DEFAULT_CLUB_KEY).wrapping_mul(club::SIZE as u32) as usize)
    };
    covered!("tel36_5 transfer regulations screen");
    exe::menubar_build(4, 0);
    let (mut fore, mut back) = (0u32, 0u32);
    exe::database_get_club_kit_colours(shown, 1, &mut fore, &mut back);
    let title = if read_usize(human + staff::CLUB_JOB) != 0 { shown + club::NAME } else { 0xa6543c };
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
    exe::gui_add_tab_bar(t, 4, tab as i32, &mut top, &mut bottom, 1);
    let explanation = match tab {
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
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    compare_by_display_list_with("transfer regulations screen (transfer_rules_info_screen_display)", &[], || o(), || transfer_rules_info_screen_display());
}

/// Comparer for `transfer_rules_info_screen_handle_click`: the tab choice side by side over the screen params; the
/// menu bar only the original.
pub(crate) unsafe extern "C" fn compare_transfer_rules_info_screen_handle_click(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I1%%);
    if tel26_item_id(item as i16) != 1 {
        // simplification: the menu bar commands do not run twice; proven in Rust mode
        return o(item);
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&screen_param_regions(), || o(item), || transfer_rules_info_screen_handle_click(item));
    SCOREBOARD.record(same && r == n, || format!("transfer regulations tab: original={} rust={} differences {:?}", r, n, first_differences(&mo, &mr)));
    r
}
