// ---------- batch %%LEVA%%: the search screens (4): the search edit session, opening and closing the screens and the Filters input (search_screens.cpp) ----------

/// The search edit session (`SearchEditSession`, opened by the Player and Staff Search screen): the current search,
/// the ten searches being edited and the ten saved copies (`PLAYER_SEARCH`, 0x2b0 bytes each) and a list.
#[allow(dead_code)]
pub(crate) mod search_edit_session {
    /// i8: index of the current search (0..9).
    pub(crate) const CURRENT: usize = 0x0;
    /// The ten searches being edited.
    pub(crate) const SEARCHES: usize = 0x1;
    /// The ten saved searches.
    pub(crate) const SAVED_SEARCHES: usize = 0x1b0e;
    pub(crate) const SEARCH_SIZE: usize = 0x2b0;
    pub(crate) const SEARCH_COUNT: u32 = 10;
    /// An LLIST [inference: the session's list of edited results].
    pub(crate) const LIST: usize = 0x361e;
}

/// The fields of a search (`PLAYER_SEARCH`) the Filters screen uses.
#[allow(dead_code)]
pub(crate) mod search_filters_fields {
    /// The search's name (text), shown in the Filters title.
    pub(crate) const NAME: usize = 0xb;
    /// The search's filter block (`SearchFilters`, `SIZE` bytes).
    pub(crate) const FILTERS: usize = 0x1fa;
    pub(crate) const SIZE: u32 = 0x99;
    /// u16: set to 1 when the Filters screen applies new filters [inference: the search must run again].
    pub(crate) const FILTERS_CHANGED: usize = 0x2a1;
    /// i8: the Filters page shown last (1 General, 2 Attributes).
    pub(crate) const FILTERS_PAGE: usize = 0x2a3;
}

/// `PLAYER_SEARCH::~PLAYER_SEARCH` (0x7ae4f0), the element destructor of the session's search arrays.
const SEARCH_FN_PLAYER_SEARCH_DESTRUCTOR: usize = 0x7ae4f0;
/// `SearchEditSession::save_all_slots` (0x804450, thiscall) and `SearchEditSession::set_current_filters` (0x804c10,
/// thiscall(session, filters) ret 4).
const SEARCH_FN_SAVE_ALL_SLOTS: usize = 0x804450;
const SEARCH_FN_SET_CURRENT_FILTERS: usize = 0x804c10;
/// `SearchFilters` methods: set_defaults 0x806450 (thiscall), reset_general_filters 0x8064d0, reset_attribute_filters
/// 0x8065d0, set_option_flag 0x806630 (thiscall(filters, lo, hi) ret 8 -> changed), the one-value setters
/// (thiscall(filters, value) ret 4), get_attribute_minimum 0x808190 (thiscall(filters, attribute, staff table) ret 8
/// -> al), set_attribute_minimum 0x808280 (thiscall(filters, attribute, minimum, staff table) ret 0xc),
/// increase/decrease_attributes_to_match 0x8084c0 / 0x808500 (thiscall(filters, staff table) ret 4).
const SEARCH_FN_FILTERS_SET_DEFAULTS: usize = 0x806450;
const SEARCH_FN_FILTERS_RESET_GENERAL: usize = 0x8064d0;
const SEARCH_FN_FILTERS_RESET_ATTRIBUTES: usize = 0x8065d0;
const SEARCH_FN_FILTERS_SET_OPTION_FLAG: usize = 0x806630;
const SEARCH_FN_FILTERS_SET_NATIONALITY_NATION: usize = 0x806ff0;
const SEARCH_FN_FILTERS_SET_NATIONALITY_REGION: usize = 0x807090;
const SEARCH_FN_FILTERS_SET_LANGUAGE_OPTION: usize = 0x807130;
const SEARCH_FN_FILTERS_SET_BASED_NATION: usize = 0x8071d0;
const SEARCH_FN_FILTERS_SET_BASED_REGION: usize = 0x807270;
const SEARCH_FN_FILTERS_SET_VALUE_BAND: usize = 0x807310;
const SEARCH_FN_FILTERS_SET_JOB_FILTER: usize = 0x8073b0;
const SEARCH_FN_FILTERS_GET_ATTRIBUTE_MINIMUM: usize = 0x808190;
const SEARCH_FN_FILTERS_SET_ATTRIBUTE_MINIMUM: usize = 0x808280;
const SEARCH_FN_FILTERS_INCREASE_TO_MATCH: usize = 0x8084c0;
const SEARCH_FN_FILTERS_DECREASE_TO_MATCH: usize = 0x808500;
/// `search_screens_filter_index_to_bit` (0x81b4e0, cdecl(char index) -> u64 mask in edx:eax).
const SEARCH_FN_FILTER_INDEX_TO_BIT: usize = 0x81b4e0;
/// `ScreenManager::get_widget_action_id` (0x468b40, thiscall(manager, item) ret 4).
const SEARCH_FN_GET_WIDGET_ACTION_ID: usize = 0x468b40;
/// The Filters screen's builder and input handler (the routines of tel21_5 and of this item).
const SEARCH_FN_BUILD_FILTERS_SCREEN: usize = 0x8133a0;
const SEARCH_FN_HANDLE_FILTERS_INPUT: usize = 0x818100;
/// search_screens.cpp (the error box path).
const SEARCH_SCREENS_CPP: usize = 0xa7e840;
/// The list type of a staff search (its attribute table is the non-player one).
const SEARCH_LIST_TYPE_STAFF: u8 = 3;
/// What the Filters input returns: stay and redraw (-11), leave the screen (-9).
const SEARCH_INPUT_REDRAW: i32 = -11;
const SEARCH_INPUT_LEAVE: i32 = -9;

/// The current search of an edit session.
pub(crate) unsafe fn search_session_current(session: usize) -> usize {
    (session as isize + search_edit_session::SEARCHES as isize + read_i8(session + search_edit_session::CURRENT) as isize * search_edit_session::SEARCH_SIZE as isize) as usize
}

/// A thiscall on the filters with no argument.
unsafe fn search_filters_call0(f: usize, filters: usize) {
    let g: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
    g(filters)
}

/// A thiscall on the filters with one argument, returning eax.
unsafe fn search_filters_call1(f: usize, filters: usize, a: u32) -> u32 {
    let g: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(f);
    g(filters, a)
}

/// `SearchFilters::get_attribute_minimum(attribute, staff table)`.
unsafe fn search_filters_get_minimum(filters: usize, attribute: u32, staff_table: u32) -> u8 {
    let g: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(SEARCH_FN_FILTERS_GET_ATTRIBUTE_MINIMUM);
    g(filters, attribute, staff_table)
}

/// `SearchFilters::set_attribute_minimum(attribute, minimum, staff table)`.
unsafe fn search_filters_set_minimum(filters: usize, attribute: u32, minimum: u8, staff_table: u32) {
    let g: unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32 = core::mem::transmute(SEARCH_FN_FILTERS_SET_ATTRIBUTE_MINIMUM);
    g(filters, attribute, minimum as u32, staff_table);
}

/// `SearchFilters::set_option_flag(lo, hi)`: 1 when the mask changed.
unsafe fn search_filters_set_option_flag(filters: usize, mask: u64) -> u32 {
    let g: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(SEARCH_FN_FILTERS_SET_OPTION_FLAG);
    g(filters, mask as u32, (mask >> 32) as u32)
}

/// The widget data and action data of an item of the current human's display (`Display::get_widget_data`,
/// `ScreenManager::get_widget_action_data`) and its action id.
unsafe fn search_widget_data(item: i16) -> u32 {
    exe::display_get_widget_data(current_human_display(), item as i32 as u32) as u32
}
unsafe fn search_widget_action_data(item: i16) -> u32 {
    exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, item as i32 as u32) as u32
}
unsafe fn search_widget_action_id(item: i16) -> i32 {
    let g: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(SEARCH_FN_GET_WIDGET_ACTION_ID);
    g(G_SCREEN_MANAGER, item as i32 as u32)
}

/// Destroys a search edit session: its list (`LLIST::~LLIST` at +0x361e), then the ten saved searches and the ten
/// searches (`PLAYER_SEARCH::~PLAYER_SEARCH` over each 0x2b0-byte element, by the CRT vector destructor iterator).
/// Original: SearchEditSession::~SearchEditSession (0x813140, search_screens.cpp, thiscall)
pub(crate) unsafe extern "thiscall" fn search_edit_session_destructor(session: usize) {
    covered!("tel21_4 search edit session destructor");
    exe::drop_llist(session + search_edit_session::LIST);
    exe::crt_vector_destructor_iterator(session + search_edit_session::SAVED_SEARCHES, search_edit_session::SEARCH_SIZE as u32, search_edit_session::SEARCH_COUNT, SEARCH_FN_PLAYER_SEARCH_DESTRUCTOR);
    exe::crt_vector_destructor_iterator(session + search_edit_session::SEARCHES, search_edit_session::SEARCH_SIZE as u32, search_edit_session::SEARCH_COUNT, SEARCH_FN_PLAYER_SEARCH_DESTRUCTOR);
}

/// Closes the Player and Staff Search screen: the edit session in screen param 0, when there is one, saves every
/// search back (`SearchEditSession::save_all_slots`), is destroyed and freed; then param 0 is cleared and the scout
/// manager's edit session of the current human is set to none (`set_edit_session(0, 0)`).
/// Original: search_screens_close_player_search_screen (0x8131c0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_close_player_search_screen() {
    covered!("tel21_4 close the player search screen");
    let session = get_screen_param(0);
    if session == 0 {
        return;
    }
    search_filters_call0(SEARCH_FN_SAVE_ALL_SLOTS, session);
    search_edit_session_destructor(session);
    exe::crt_operator_delete(session);
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0, 0usize, 0);
    set_edit_session(read_usize(G_SCOUT_MANAGER), 0, 0);
}

/// Opens the Filters screen of the session's current search.
///
/// Without a session: the "Error" box of search_screens.cpp line 0x100c and nothing else. Otherwise the screen opens
/// (`ScreenManager::open_screen(build_filters_screen, handle_filters_input, 1, 0, 0)`); when it does, a new
/// `SearchFilters` (0x99 bytes, `set_defaults`) is overwritten with a copy of the current search's filter block, and
/// the params are 0 the session, 1 the copy and 2 the search's last Filters page (sign-extended). DEFECT of the
/// original kept: a failed allocation copies into address 0.
/// Original: search_screens_open_filters_screen (0x813210, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_open_filters_screen(session: usize) {
    covered!("tel21_4 open the filters screen");
    if session == 0 {
        tel26_error_box(SEARCH_SCREENS_CPP, 0x100c);
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, SEARCH_FN_BUILD_FILTERS_SCREEN, SEARCH_FN_HANDLE_FILTERS_INPUT, 1, 0usize, 0) == 0 {
        return;
    }
    let filters = exe::crt_operator_new(search_filters_fields::SIZE);
    if filters != 0 {
        search_filters_call0(SEARCH_FN_FILTERS_SET_DEFAULTS, filters);
    }
    let search = search_session_current(session);
    core::ptr::copy_nonoverlapping((search + search_filters_fields::FILTERS) as *const u8, filters as *mut u8, search_filters_fields::SIZE as usize);
    let page = read_i8(search_session_current(session) + search_filters_fields::FILTERS_PAGE);
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0, session, 0);
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 1, filters, 0);
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 2, page as i32 as usize, 0);
}

/// Handles the Filters screen's input (params: 0 the session, 1 the filters being edited, 2 the page).
///
/// By the item's id (`tel26_item_id`; the later ones through `Display::get_widget_id`, the same value) and, with
/// `staff` = the current search is a staff search (list type 3) for the attribute table:
/// - 1: param 2 = the item's value (the General / Attributes toggle); redraw (-11);
/// - 2: Clear: `reset_general_filters` on page 1, else `reset_attribute_filters`; redraw;
/// - 3: an attribute's on/off: its minimum (`get_attribute_minimum(widget data, staff)`) becomes 10 if it was 1, else
///   1, written with `set_attribute_minimum(widget action data, new, staff)`; redraw;
/// - 0x12: the attribute's level button: minimum below 10 goes to 10, below 15 to 15, from 20 to 1, otherwise 20;
/// - 4 / 5: the minimum one down / up (`get_attribute_minimum` of the item's value on a staff search, of the widget data
///   on a player search; written back for the widget data, the table by the list type);
/// - 6 / 7: `decrease_attributes_to_match` / `increase_attributes_to_match(staff)`;
/// - 8: Cancel: the filters are freed; leave (-9);
/// - 9: Ok: `SearchEditSession::set_current_filters(filters)`, the search keeps the page, the filters are freed and the
///   search is marked changed (+0x2a1 = 1); leave;
/// - 0xa: an option button: `set_option_flag(filter_index_to_bit(widget data))`; redraw when it changed, otherwise the
///   item goes on to the checks below (and in the end to the menu bar);
/// - 0xb..0xf: the option flag 0x10 / 0x20 / 0x80000 / 0x800000 / 0x1000000 and then the nationality nation, the
///   nationality region, the language option, the based nation, the based region from the widget data; redraw;
/// - action id 0x10 / 0x11: `set_job_filter` / `set_value_band` with the widget action data; redraw;
/// - anything else: `menubar_handle_object_link(item)` and its answer.
/// Original: search_screens_handle_filters_input (0x818100, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_handle_filters_input(item: i16) -> i32 {
    covered!("tel21_4 filters input");
    let session = get_screen_param(0);
    let filters = get_screen_param(1);
    let page = get_screen_param(2) as u8;
    let staff = || (read_u8(search_session_current(session) + player_search_fields::LIST_TYPE) == SEARCH_LIST_TYPE_STAFF) as u32;
    let id = tel26_item_id(item);
    match id {
        1 => {
            exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 2, tel26_item_value(item) as usize, 0);
            return SEARCH_INPUT_REDRAW;
        }
        2 => {
            if page == 1 {
                search_filters_call0(SEARCH_FN_FILTERS_RESET_GENERAL, filters);
            } else {
                search_filters_call0(SEARCH_FN_FILTERS_RESET_ATTRIBUTES, filters);
            }
            return SEARCH_INPUT_REDRAW;
        }
        3 => {
            covered!("tel21_4 filters input attribute toggle");
            let table = staff();
            let minimum = search_filters_get_minimum(filters, search_widget_data(item), table);
            let new = if minimum == 1 { 10 } else { 1 };
            search_filters_set_minimum(filters, search_widget_action_data(item), new, table);
            return SEARCH_INPUT_REDRAW;
        }
        0x12 => {
            covered!("tel21_4 filters input attribute level");
            let table = staff();
            let minimum = search_filters_get_minimum(filters, search_widget_action_data(item), table) as i8;
            let new = if minimum < 10 {
                10
            } else if minimum < 15 {
                15
            } else if minimum >= 20 {
                1
            } else {
                20
            };
            search_filters_set_minimum(filters, search_widget_action_data(item), new, table);
            return SEARCH_INPUT_REDRAW;
        }
        4 | 5 => {
            covered!("tel21_4 filters input minimum step");
            let step = |m: u8| if id == 4 { m.wrapping_sub(1) } else { m.wrapping_add(1) };
            if staff() != 0 {
                // the minus button reads the item's value, the plus button the widget data
                let attribute = if id == 4 { tel26_item_value(item) } else { search_widget_data(item) };
                let target = search_widget_data(item);
                let m = search_filters_get_minimum(filters, attribute, 1);
                search_filters_set_minimum(filters, target, step(m), 1);
            } else {
                let attribute = search_widget_data(item);
                let target = search_widget_data(item);
                let m = search_filters_get_minimum(filters, attribute, 0);
                search_filters_set_minimum(filters, target, step(m), 0);
            }
            return SEARCH_INPUT_REDRAW;
        }
        6 => {
            search_filters_call1(SEARCH_FN_FILTERS_DECREASE_TO_MATCH, filters, staff());
            return SEARCH_INPUT_REDRAW;
        }
        7 => {
            search_filters_call1(SEARCH_FN_FILTERS_INCREASE_TO_MATCH, filters, staff());
            return SEARCH_INPUT_REDRAW;
        }
        8 => {
            covered!("tel21_4 filters input cancel");
            if filters != 0 {
                exe::crt_operator_delete(filters);
            }
            return SEARCH_INPUT_LEAVE;
        }
        9 => {
            covered!("tel21_4 filters input ok");
            search_filters_call1(SEARCH_FN_SET_CURRENT_FILTERS, session, filters as u32);
            write_u8(search_session_current(session) + search_filters_fields::FILTERS_PAGE, page);
            if filters != 0 {
                exe::crt_operator_delete(filters);
            }
            write_u16(search_session_current(session) + search_filters_fields::FILTERS_CHANGED, 1);
            return SEARCH_INPUT_LEAVE;
        }
        _ => {}
    }
    if id == 0xa {
        covered!("tel21_4 filters input option");
        let to_bit: unsafe extern "C" fn(u32) -> u64 = core::mem::transmute(SEARCH_FN_FILTER_INDEX_TO_BIT);
        if search_filters_set_option_flag(filters, to_bit(search_widget_data(item))) != 0 {
            return SEARCH_INPUT_REDRAW;
        }
    }
    let option = match id {
        0xb => Some((0x10u64, SEARCH_FN_FILTERS_SET_NATIONALITY_NATION)),
        0xc => Some((0x20, SEARCH_FN_FILTERS_SET_NATIONALITY_REGION)),
        0xd => Some((0x80000, SEARCH_FN_FILTERS_SET_LANGUAGE_OPTION)),
        0xe => Some((0x800000, SEARCH_FN_FILTERS_SET_BASED_NATION)),
        0xf => Some((0x1000000, SEARCH_FN_FILTERS_SET_BASED_REGION)),
        _ => None,
    };
    if let Some((mask, setter)) = option {
        covered!("tel21_4 filters input nation or region");
        search_filters_set_option_flag(filters, mask);
        search_filters_call1(setter, filters, search_widget_data(item));
        return SEARCH_INPUT_REDRAW;
    }
    if search_widget_action_id(item) == 0x10 {
        search_filters_call1(SEARCH_FN_FILTERS_SET_JOB_FILTER, filters, search_widget_action_data(item));
        return SEARCH_INPUT_REDRAW;
    }
    if search_widget_action_id(item) == 0x11 {
        search_filters_call1(SEARCH_FN_FILTERS_SET_VALUE_BAND, filters, search_widget_action_data(item));
        return SEARCH_INPUT_REDRAW;
    }
    exe::menubar_handle_object_link(item as i32 as u32)
}

/// Comparer for `search_edit_session_destructor`.
pub(crate) unsafe extern "thiscall" fn compare_search_edit_session_destructor(session: usize) {
    // simplification: freeing does not run twice; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(session)
}

/// Comparer for `search_screens_close_player_search_screen`.
pub(crate) unsafe extern "C" fn compare_search_screens_close_player_search_screen() {
    // simplification: saving and freeing the session do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(%%I1%%)()
}

/// Comparer for `search_screens_open_filters_screen`: only the original runs (it opens a screen and allocates); when
/// the screen opened with the session, the plan is checked: param 1 is a copy of the current search's filter block and
/// param 2 its last page.
pub(crate) unsafe extern "C" fn compare_search_screens_open_filters_screen(session: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I2%%)(session);
    if session == 0 || get_screen_param(0) != session {
        return;
    }
    let search = search_session_current(session);
    let copy = get_screen_param(1);
    let same = copy != 0
        && core::slice::from_raw_parts(copy as *const u8, search_filters_fields::SIZE as usize)
            == core::slice::from_raw_parts((search + search_filters_fields::FILTERS) as *const u8, search_filters_fields::SIZE as usize);
    let page = read_i8(search + search_filters_fields::FILTERS_PAGE) as i32 as usize;
    SCOREBOARD.record(same && get_screen_param(2) == page, || {
        format!("search_screens_open_filters_screen: filters copied {} page {:#x} (plan {:#x})", same, get_screen_param(2), page)
    });
}

/// Comparer for `search_screens_handle_filters_input`: side by side over the filters (0x99 bytes) and the screen
/// params for the answers that only change them; Cancel and Ok (they free the filters), the option buttons (id 0xa,
/// which may go on to the menu bar) and the rest (the menu bar opens other screens) only run the original.
pub(crate) unsafe extern "C" fn compare_search_screens_handle_filters_input(item: i16) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(i16) -> i32>(%%I3%%);
    let filters = get_screen_param(1);
    let id = tel26_item_id(item);
    let action = search_widget_action_id(item);
    let only_filters = matches!(id, 1..=7 | 0xb..=0xf | 0x12) || (!matches!(id, 8..=0xa) && (action == 0x10 || action == 0x11));
    if filters == 0 || !only_filters {
        // simplification: freeing the filters and the menu bar do not run twice; proven in Rust mode
        return o(item);
    }
    let mut regions = screen_param_regions();
    regions.push((filters, search_filters_fields::SIZE as usize));
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(item), || search_screens_handle_filters_input(item));
    SCOREBOARD.record(equal && r == n, || format!("Search filters input (id {:#x}): original={} rust={} differences {:?}", id, r, n, first_differences(&mo, &mr)));
    r
}
