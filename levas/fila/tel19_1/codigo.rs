// ---------- batch %%LEVA%%: the player and staff search screen (finder_screens.cpp, part 1): opening it, building it ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\finder_screens.cpp" (the error box).
const FINDER_SCREENS_CPP: usize = 0xa7e840;
/// The search screen's global pointers: the view flags of the current list (`SearchEditSession` + 0x294 of the current
/// list, read by the column comparers) and the manager the screen was built for.
const FINDER_VIEW_FLAGS: usize = 0xdd6cc8;
const FINDER_VIEW_MANAGER: usize = 0xdd6ccc;
/// The attribute columns of the Attributes view (bytes, 0xff ends): the fixed lists of each attribute group (by the view
/// flag: 0x80, 0x100, 0x200, 0x400, 0x800; the group names are the menu's) and the
/// "Filtered" list the screen builds from the filter minimums (9 bytes and the 0xff end, 0xa7dfcc..0xa7dfd5).
const FINDER_COLUMNS_STAFF: usize = 0xa7dfc0;
const FINDER_COLUMNS_PHYSICAL: usize = 0xa7df84;
const FINDER_COLUMNS_MENTAL: usize = 0xa7dfb4;
const FINDER_COLUMNS_GOALKEEPING: usize = 0xa7df90;
const FINDER_COLUMNS_DEFENSIVE: usize = 0xa7df9c;
const FINDER_COLUMNS_ATTACKING: usize = 0xa7dfa8;
const FINDER_COLUMNS_FILTERED: usize = 0xa7dfcc;
/// The statistics columns of the Stats view (bytes, 0xff ends).
const FINDER_COLUMNS_STATS: usize = 0xa7df6c;
const FINDER_COLUMNS_MORE_STATS: usize = 0xa7df78;
/// The byte that cuts the list name copied to the screens' buffer at 0x7cf characters.
const FINDER_TEXT_END: usize = 0xde2733;
/// The column comparers of the result list (finder_screens.cpp, ported elsewhere or still the original's), passed to
/// `SearchEditSession::get_current_results`.
const FINDER_SORT_RECORD: usize = 0x818ab0;
/// The SearchEditSession methods still called by address (finder_edit_session.cpp).
const FINDER_SESSION_GET_CURRENT_RESULTS: usize = 0x804a40;
const FINDER_SESSION_NEW: usize = 0x8041c0;
/// `SCOUT_MANAGER::collect_current_duty_comps(list)` (0x7f7010, thiscall ret 4).
const FINDER_COLLECT_DUTY_COMPS: usize = 0x7f7010;
/// `HUMAN_MANAGER::get_password(manager)` (0x5fde40, thiscall ret 4, char*) and `get_recent_file(index, out, size,
/// strip folder, manager, kind)` (0x5fc760, thiscall ret 0x18).
const FINDER_GET_PASSWORD: usize = 0x5fde40;
const FINDER_GET_RECENT_FILE: usize = 0x5fc760;
/// `player_search_read_file_header(path, out, kind)` (0x7bcee0, cdecl, bool).
const FINDER_READ_FILE_HEADER: usize = 0x7bcee0;
/// `key_nation_sorted_order(by_second_name)` (0x66a0c0, cdecl, the 35 sorted key nation indices).
const FINDER_KEY_NATION_ORDER: usize = 0x66a0c0;
/// `get_region_article_form(region)` (0x617780, cdecl, al).
const FINDER_REGION_ARTICLE_FORM: usize = 0x617780;
/// `gui_add_page_buttons(right, y, page, pages, first, previous, next, last)` (0x5ede20, cdecl).
const FINDER_PAGE_BUTTONS: usize = 0x5ede20;
/// `Text::draw(slot, left, right, &line, font, colour, 3, 0)` (0x4aeaf0, thiscall ret 0x20).
const FINDER_TEXT_DRAW: usize = 0x4aeaf0;
/// `ScreenManager::add_area(..)` (0x4567f0, 11 dwords, thiscall ret 0x2c) and `add_item(..)` (0x45cb60, 18 dwords).
const FINDER_SM_ADD_AREA: usize = 0x4567f0;
const FINDER_SM_ADD_ITEM: usize = 0x45cb60;
/// The size of one list of the edit session (10 lists from +0; +0 holds the current list index).
const FINDER_LIST_SIZE: usize = 0x2b0;

/// The SearchEditSession (finder_edit_session.cpp, 0x3638 bytes): +0 the current list (char, 0..9) and ten lists of
/// `FINDER_LIST_SIZE` bytes; the offsets of a list are from the session plus 0x2b0 × the list index.
#[allow(dead_code)]
mod finder_list {
    /// The PLAYER_SEARCH of the list (`get_scout`, `get_scout_assignment` take it as `this`).
    pub(crate) const SEARCH: usize = 0x1;
    /// u8: the list type (1 short list, 2 quick search, 3 staff search, 4 scout search).
    pub(crate) const TYPE: usize = 0xb;
    /// char[]: the list name.
    pub(crate) const NAME: usize = 0xc;
    /// u8: the list is locked.
    pub(crate) const LOCKED: usize = 0x13a;
    /// i32: the club the list follows (-1 none).
    pub(crate) const ASSIGNED_CLUB: usize = 0x191;
    /// The list's SearchFilters (0x99 bytes).
    pub(crate) const FILTERS: usize = 0x1fb;
    pub(crate) const FILTERS_SIZE: usize = 0x99;
    /// u32: the view flags (1 General, 2 Stats, 4 More Stats, 8 Attributes, 0x10 Player Notes, 0x20 Other, 0x40 Contract,
    /// 0x80.. the attribute groups, 0x1000 Filtered, 0x2000 Scout Report); +4 the sort column, +0xa the sort direction,
    /// i16 +0xe the page.
    pub(crate) const VIEW: usize = 0x294;
    pub(crate) const SORT: usize = 0x298;
    pub(crate) const SORT_DESCENDING: usize = 0x29e;
    pub(crate) const PAGE: usize = 0x2a2;
    /// u32: the list was loaded from a file (Save is only offered for a file list not yet saved).
    pub(crate) const SAVED: usize = 0x2a5;
    pub(crate) const FROM_FILE: usize = 0x2ad;
}
/// Session fields after the ten lists: the "reset available" flag of each list (u32 × 10), the result list (LLIST),
/// the total found, the shown count and the "results cached" flag.
#[allow(dead_code)]
mod finder_session {
    pub(crate) const RESET_AVAILABLE: usize = 0x1ae1;
    pub(crate) const RESULTS: usize = 0x361e;
    pub(crate) const TOTAL: usize = 0x362c;
    pub(crate) const SHOWN: usize = 0x3630;
    pub(crate) const CACHED: usize = 0x3634;
    pub(crate) const SIZE: usize = 0x3638;
}

/// List `k` of the session `s` (the index sign-extended).
fn finder_list_at(s: usize, k: i8) -> usize {
    (s as isize + k as isize * FINDER_LIST_SIZE as isize) as usize
}
/// The current list of the session `s`.
unsafe fn finder_current(s: usize) -> usize {
    finder_list_at(s, read_i8(s))
}
/// The type of the current list.
unsafe fn finder_type(s: usize) -> u8 {
    read_u8(finder_current(s) + finder_list::TYPE)
}
/// `get_scout` of the current list's PLAYER_SEARCH.
unsafe fn finder_scout(s: usize) -> usize {
    get_scout(finder_current(s) + finder_list::SEARCH)
}
/// The game's text function (`langlib_sprintf`, cdecl, variadic).
unsafe fn finder_lang() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_LANGLIB_SPRINTF)
}
/// The CRT's sprintf.
unsafe fn finder_format() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_CRT_SPRINTF)
}
/// A thiscall with one dword argument, returning eax.
unsafe fn finder_call1(f: usize, this: usize, a: u32) -> u32 {
    let g: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(f);
    g(this, a)
}
/// The Display of the current screen slot.
unsafe fn finder_display() -> usize {
    screens_current_display(G_SCREEN_MANAGER)
}
/// `Display::add_item` on the current slot's Display.
unsafe fn finder_item(f: [u32; 18]) -> u32 {
    tel26_add_item(finder_display(), f)
}
/// `Display::add_area` on the current slot's Display.
unsafe fn finder_area(f: [u32; 11]) -> u32 {
    tel26_add_area(finder_display(), f)
}
/// A popup area of the screen's menus (flags `flags`, the popup colour), owned by `owner`.
unsafe fn finder_popup(flags: u32, owner: u32) -> u32 {
    finder_area([0, 0, 0, 0, 0, 0, 0, 0, flags, job_colour(JOB_COLOUR_31AA), owner])
}
/// An entry of a popup menu (kind 2, or 0x2002 ticked): the text, enabled (1) or greyed (0x21), the id and the value.
unsafe fn finder_entry(kind: u32, enabled: bool, text: usize, id: u32, value: u32, area: u32) -> u32 {
    finder_item([kind, 0, 0, 0, 0, 0, 0, 0x10, job_colour(JOB_COLOUR_31AA), job_colour(JOB_COLOUR_BDEC), if enabled { 1 } else { 0x21 }, 1,
        job_colour(JOB_COLOUR_BDEA), text as u32, 0, id, value, area])
}
/// The screens' buffer as an enabled entry with `id`/`value`, or greyed without them.
unsafe fn finder_entry_if(enabled: bool, id: u32, value: u32, area: u32) -> u32 {
    if enabled { finder_entry(2, true, JOB_TEXT, id, value, area) } else { finder_entry(2, false, JOB_TEXT, 0, 0, area) }
}
/// The separator line of a popup menu.
unsafe fn finder_separator(area: u32) -> u32 {
    finder_item([1, 0, 0, 0, 0, 0, 0, 0x1000010, job_colour(JOB_COLOUR_31AA), job_colour(JOB_COLOUR_BDEC), 1, 1, job_colour(JOB_COLOUR_BDEA),
        0, 0, 0, 0, area])
}
/// A ticked option of the View menus: the translated `text`, ticked when `bit` is set in the view flags, id 0x12.
unsafe fn finder_view_option(text: usize, bit: u32, area: u32) {
    finder_lang()(JOB_TEXT, text);
    let kind = if read_u32(read_usize(FINDER_VIEW_FLAGS)) & bit != 0 { 0x2002 } else { 2 };
    finder_entry(kind, true, JOB_TEXT, 0x12, bit, area);
}
/// A header button of the result columns (grey, pale grey text, style 0xc or 1 for the left-aligned ones), id 0x1e and
/// the sort bit.
unsafe fn finder_header(area: u32, column: u32, align: u32, bit: u32) {
    tel26_area_button([area, column, 0, 0x30, job_colour(JOB_COLOUR_317E), align, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0x1e, bit]);
}
/// `HUMAN_MANAGER::get_password(manager)`: the manager's password (0 = the current human).
unsafe fn finder_password(manager: usize) -> usize {
    finder_call1(FINDER_GET_PASSWORD, read_usize(G_HUMAN_MANAGER), manager as u32) as usize
}

/// Rule: opens the player and staff search screen for the scout list `list`.
///
/// `ScreenManager::open_screen` (the builder and the answer of this screen, single instance, the leave function
/// 0x8131c0); when it opens, a new edit session (operator new 0x3638, `SearchEditSession::SearchEditSession(list)`; 0 if
/// the allocation fails) is handed to the scout manager (`SCOUT_MANAGER::set_edit_session(session, 0)`) and the
/// screen's parameters are set: 0 the session, 1..6 zero, 7 = 1 (the page count), 8, 9 and 10 = -1, 11 zero.
/// Original: `search_screens_open_player_search_screen` (0x808540, finder_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn search_screens_open_player_search_screen(list: u32) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x808690usize, 0x80eb00usize, 1, 0usize, 0x8131c0) == 0 {
        return;
    }
    covered!("tel19_1 open search screen");
    let p = exe::crt_operator_new(finder_session::SIZE as u32);
    let session = if p != 0 { finder_call1(FINDER_SESSION_NEW, p, list) as usize } else { 0 };
    set_edit_session(read_usize(G_SCOUT_MANAGER), session, 0);
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0, session, 0);
    for k in 1..=6 {
        exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, k, 0usize, 0);
    }
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 7, 1usize, 0);
    for k in 8..=10 {
        exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, k, usize::MAX, 0);
    }
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0xb, 0usize, 0);
}

/// Comparer for `search_screens_open_player_search_screen`.
///
/// Opening a screen and allocating the session cannot run twice: only the original runs, and when it opened the
/// screen (the current screen's builder is the search screen's) the parameters it set are checked against the plan.
pub(crate) unsafe extern "C" fn compare_search_screens_open_player_search_screen(list: u32) {
    original_fn::<unsafe extern "C" fn(u32)>(%%I0%%)(list);
    let node = current_screen_node(G_SCREEN_MANAGER);
    if node == 0 || read_usize(node + screen_node_fields::BUILD) != 0x808690 {
        return;
    }
    let plan: [usize; 11] = [0, 0, 0, 0, 0, 0, 1, usize::MAX, usize::MAX, usize::MAX, 0];
    let got: Vec<usize> = (1..=11).map(|k| get_screen_param(k)).collect();
    SCOREBOARD.record(got[..] == plan[..], || format!("search screen open({}): parameters 1..11 {:x?}", list, got));
}

/// Rule: the two dwords of a SearchFilters' flag mask.
///
/// Returns +0 in eax and +4 in edx (the 64-bit mask).
/// Original: `SearchFilters::get_flag_mask` (0x80ea90, finder_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn search_filters_get_flag_mask(this: usize) -> u64 {
    read_u32(this) as u64 | (read_u32(this + 4) as u64) << 32
}
compare_value!(compare_search_filters_get_flag_mask, search_filters_get_flag_mask, %%I1%%, () -> u64);

/// Rule: the type of a list of the edit session: list `index` (its low byte, signed), or the current one for -1.
///
/// Original: `SearchEditSession::get_list_type` (0x80eaa0, finder_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn search_edit_session_get_list_type(this: usize, index: u32) -> u8 {
    let k = if index as u8 == 0xff { read_i8(this) } else { index as u8 as i8 };
    read_u8(finder_list_at(this, k) + finder_list::TYPE)
}
compare_value!(compare_search_edit_session_get_list_type, search_edit_session_get_list_type, %%I2%%, (index: u32) -> u8);

/// Rule: the club a list of the edit session follows (-1 none): list `index` (low byte, signed), or the current one
/// for -1.
///
/// Original: `SearchEditSession::get_assigned_club` (0x80ead0, finder_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn search_edit_session_get_assigned_club(this: usize, index: u32) -> i32 {
    let k = if index as u8 == 0xff { read_i8(this) } else { index as u8 as i8 };
    read_i32(finder_list_at(this, k) + finder_list::ASSIGNED_CLUB)
}
compare_value!(compare_search_edit_session_get_assigned_club, search_edit_session_get_assigned_club, %%I3%%, (index: u32) -> i32);

/// What the pieces of the search screen's builder share: the edit session (param 0), the human it is built for, the
/// title colours (params 1 and 2, as dwords), the pick mode (param 5: the screen picks a person and shows Cancel and
/// OK), the copy of the current list's filters, the result list and its counts, the free band left by the tab bar
/// (`top`, `bottom`), the tab area and the page buttons' left edge.
struct FinderBuild {
    s: usize,
    mgr: usize,
    fg: u32,
    bg: u32,
    pick: usize,
    filters: [u8; 0xa0],
    results: usize,
    shown: u32,
    /// The total found, later the attribute column list of the Attributes view (one variable in the original).
    l14: usize,
    top: i32,
    bottom: i32,
    tab_area: u32,
    page_left: i32,
    /// The rows of a page plus one (low byte) and the row height.
    rows: u8,
    row_height: i32,
    /// The screen's Text object (the last match lines are drawn through it).
    text: usize,
}

/// A tab of `gui_add_tab_bar` (0xd4 bytes: +2 the colour, +4 the text, +0xcc the kind, +0xd0 the link).
const FINDER_TAB_SIZE: usize = 0xd4;

/// The 11 fields of `ScreenManager::add_text_item` (0x803fd0).
unsafe fn finder_text_item(f: [u32; 11]) -> u32 {
    let g: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
        core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    g(G_SCREEN_MANAGER, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10])
}

impl FinderBuild {
    /// The right edge of the texts above the list: 5 left of the page buttons, or 0x30c without them.
    fn right(&self) -> u32 {
        if self.page_left != 0 { (self.page_left - 5) as u32 } else { 0x30c }
    }
    /// The 0x7d7 (person) button of the scout line, in the heading colour.
    unsafe fn scout_line(&self, scout: usize) {
        tel26_add_button([0x6e, self.top as u32, self.right(), (self.top + 0x14) as u32, 2, 0, 0xc, 1, job_colour(JOB_COLOUR_BDE8), JOB_TEXT as u32, 0x7d7, scout as u32]);
    }
    /// The screens' buffer as the text line above the list, in the heading colour.
    unsafe fn heading_line(&self) {
        finder_text_item([0x6e, self.top as u32, self.right(), (self.top + 0x14) as u32, 2, 0, 0xc, 1, job_colour(JOB_COLOUR_BDE8), JOB_TEXT as u32, 0]);
    }
}

/// The search screen's tab bar (one tab per list: the list's name, greyed for a scout list without a scout), the
/// Search button and its menu: for a scout list with a scout, Assign Scout (when he is free: the duty competitions,
/// Club, Next Opposition, Youth Players, the nations and the regions) or Recall Scout; Load, Save, Save As, Save For (the
/// other humans with a password; only with more than one human), Delete, Unlock or Lock, Reset, Clear and the four
/// recent files of the list's kind.
unsafe fn finder_build_menu(b: &mut FinderBuild) {
    let lang = finder_lang();
    let s = b.s;
    let mut tabs = [0u8; FINDER_TAB_SIZE * 0x19];
    for k in 0..0x19 {
        tabs[k * FINDER_TAB_SIZE..k * FINDER_TAB_SIZE + 2].copy_from_slice(&0xffffu16.to_le_bytes());
        tabs[k * FINDER_TAB_SIZE + 2..k * FINDER_TAB_SIZE + 4].copy_from_slice(&0u16.to_le_bytes());
    }
    let tab = |k: usize| tabs.as_ptr() as usize + k * FINDER_TAB_SIZE;
    for k in 0..10i8 {
        let t = tab(k as usize);
        scout_manager_write_list_name(k as u32, t + 4, 0xc8);
        let list = finder_list_at(s, k);
        if read_u8(list + finder_list::TYPE) == 4 && get_scout(list + finder_list::SEARCH) == 0 {
            covered!("tel19_1 search screen scout tab without scout");
            write_u16(t + 2, read_u16(JOB_COLOUR_317E));
        }
        write_u32(t + 0xcc, 3);
        write_i32(t + 0xd0, k as i32);
    }
    b.top = 0x50;
    b.bottom = 0x221;
    exe::gui_add_tab_bar(tabs.as_ptr() as usize, 10, read_i8(s) as i32, &mut b.top, &mut b.bottom, 1);
    let area = finder_area([0x6e, b.top as u32, 0x1e5, (b.top + 0x14) as u32, 3, 0, 1, 0, 1, 0, u32::MAX]);
    b.tab_area = area;
    lang(JOB_TEXT, 0xa7e824usize); // "Search"
    let button = tel26_area_button([area, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0, 0]);
    let menu = finder_popup(0x130, button);

    if finder_type(s) == 4 && finder_scout(s) != 0 {
        let (mut club, mut nation, mut comp, mut youth, mut next) = (0u32, 0u32, 0u32, 0u32, 0u32);
        let mut region = [0u8; 4];
        if get_scout_assignment(finder_current(s) + finder_list::SEARCH, &mut club, &mut nation, region.as_mut_ptr(), &mut comp, &mut youth, &mut next) == 0 {
            covered!("tel19_1 search screen assign scout menu");
            lang(JOB_TEXT, 0x9918c4usize); // "Assign Scout"
            let assign = finder_entry(2, true, JOB_TEXT, 0, 0, menu);
            let sub = finder_popup(0x30, assign);
            let mut comps = [0u8; 0x20];
            let list = comps.as_mut_ptr() as usize;
            exe::new_llist(list);
            finder_call1(FINDER_COLLECT_DUTY_COMPS, read_usize(G_SCOUT_MANAGER), list as u32);
            if read_u16(list) != 0 {
                write_u32(list + 0xa, read_u32(list + 2));
                while get_current(list) != 0 {
                    let id = read_i32(next_list_item(list) as usize);
                    if id > -1 {
                        let c = (read_usize(G_CLUB_COMP_TABLE) as isize + id as isize * 0x6b) as usize;
                        finder_entry(2, true, c + 0x38, 6, id as u32, sub);
                    }
                }
                finder_separator(sub);
            }
            lang(JOB_TEXT, 0x9ca08cusize); // "Club"
            finder_entry(2, true, JOB_TEXT, 0x20, 0, sub);
            lang(JOB_TEXT, 0xa701e0usize); // "Next Opposition"
            finder_entry(2, true, JOB_TEXT, 0x22, 0, sub);
            lang(JOB_TEXT, 0xa7e814usize); // "Youth Players"
            finder_entry(2, true, JOB_TEXT, 0x1f, 0, sub);
            finder_separator(sub);
            let mut order = {
                let f: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FINDER_KEY_NATION_ORDER);
                f(0)
            };
            lang(JOB_TEXT, 0x9ca010usize); // "Nation"
            let n_item = finder_entry(2, true, JOB_TEXT, 0, 0, sub);
            let nations = finder_popup(0x30, n_item);
            for _ in 0..0x23 {
                let n = read_usize((G_LEAGUE_TABLE as isize + read_i8(order) as isize * 0x48) as usize);
                let mgr = b.mgr;
                let shown = (n != 0 && (read_i8(n + 0x85) > 8 || read_u8(n + 0x11c) != 0))
                    || {
                        let c = read_usize(mgr + staff::CLUB_JOB);
                        c != 0 && n == read_usize(c + club::NATION)
                    }
                    || {
                        let c = read_usize(mgr + staff::NATIONAL_JOB);
                        c != 0 && n == read_usize(c + club::NATION)
                    };
                if shown {
                    finder_entry(2, true, n + 0x38, 4, read_u32(n), nations);
                }
                order += 1;
            }
            finder_separator(sub);
            lang(JOB_TEXT, 0xa7e80cusize); // "Region"
            let r_item = finder_entry(2, true, JOB_TEXT, 0, 0, sub);
            let regions = finder_popup(0x30, r_item);
            let mut r = get_regions_sorted_by_name(u32::MAX);
            let mut name = [0u8; 0xc8];
            for _ in 0..0xe {
                get_region_name(read_u8(r) as u32, name.as_mut_ptr() as usize, 0xc8, u32::MAX);
                finder_entry(2, true, name.as_ptr() as usize, 5, read_i8(r) as i32 as u32, regions);
                r += 1;
            }
            exe::drop_llist(list);
        } else {
            covered!("tel19_1 search screen recall scout");
            lang(JOB_TEXT, 0xa7e7fcusize); // "Recall Scout"
            finder_entry(2, true, JOB_TEXT, 7, 0, menu);
        }
    }

    let settings = matches!(finder_type(s), 2 | 3);
    lang(JOB_TEXT, if settings { 0xa7e7e4usize } else { 0xa7e7f4 }); // "Load Settings" / "Load"
    finder_entry(2, true, JOB_TEXT, 8, 0, menu);
    lang(JOB_TEXT, if matches!(finder_type(s), 2 | 3) { 0xa7e7ccusize } else { 0xa7e7dc }); // "Save Settings" / "Save"
    let cur = finder_current(s);
    finder_entry_if(read_u32(cur + finder_list::SAVED) == 0 && read_u32(cur + finder_list::FROM_FILE) != 0, 9, 0, menu);
    lang(JOB_TEXT, if matches!(finder_type(s), 2 | 3) { 0xa7e7b0usize } else { 0xa7e7c4 }); // "Save Settings As" / "Save As"
    finder_entry(2, true, JOB_TEXT, 0xa, 0, menu);
    if count_human_managers(0) > 1 {
        covered!("tel19_1 search screen save for");
        lang(JOB_TEXT, if matches!(finder_type(s), 2 | 3) { 0xa7e790usize } else { 0xa7e7a4 }); // "Save Settings For" / "Save For"
        let item = finder_entry(2, true, JOB_TEXT, 0, 0, menu);
        let humans = finder_popup(0x30, item);
        let mut name = [0u8; 0xc8];
        for k in 0..0x10i32 {
            let p = (read_usize(G_STAFF_TABLE) as isize + (read_i32(G_STAFF_COUNT) - k - 1) as isize * 0x6e) as usize;
            if read_u8(p + staff::CLASSIFICATION) == 1 && p != b.mgr {
                exe::database_format_staff_name(p, name.as_mut_ptr() as usize, 0xc8, 1);
                if read_u8(finder_password(p)) != 0 {
                    finder_entry(2, true, name.as_ptr() as usize, 0xb, p as u32, humans);
                } else {
                    finder_entry(2, false, name.as_ptr() as usize, 0, 0, humans);
                }
            }
        }
    }
    lang(JOB_TEXT, if matches!(finder_type(s), 2 | 3) { 0xa7e780usize } else { 0xa69fb4 }); // "Delete Settings" / "Delete"
    finder_entry(2, true, JOB_TEXT, 0xc, 0, menu);
    if read_u8(finder_current(s) + finder_list::LOCKED) != 0 {
        lang(JOB_TEXT, 0xa7e778usize); // "Unlock"
        finder_entry(2, true, JOB_TEXT, 0xd, 0, menu);
    } else if read_u8(finder_password(0)) != 0 {
        lang(JOB_TEXT, 0xa7e770usize); // "Lock"
        finder_entry(2, true, JOB_TEXT, 0xe, 0, menu);
    }
    lang(JOB_TEXT, 0xa7e768usize); // "Reset"
    finder_entry_if(read_u32((s as isize + read_i8(s) as isize * 4 + finder_session::RESET_AVAILABLE as isize) as usize) != 0, 0xf, 0, menu);
    lang(JOB_TEXT, 0xa7e730usize, 0xad9c64usize); // "Clear<%s - COMMENT - clear short list or scout list>"
    let cur = finder_current(s);
    let clear = !matches!(read_u8(cur + finder_list::TYPE), 2 | 3) && b.shown != 0
        && (read_i32(cur + finder_list::ASSIGNED_CLUB) < 0 || get_scout(cur + finder_list::SEARCH) == 0);
    finder_entry_if(clear, 0x10, 0, menu);

    // the recent files of the list's kind (a separator before the first)
    let mut separator = -1i16;
    let mut path = [0u8; 0x104];
    let mut header = [0u8; 0x170];
    let recent: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FINDER_GET_RECENT_FILE);
    let read_header: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FINDER_READ_FILE_HEADER);
    for k in 0..4u32 {
        recent(read_usize(G_HUMAN_MANAGER), k, path.as_mut_ptr() as usize, 0x104, 0, 0, 2);
        if path[0] == 0 {
            continue;
        }
        covered!("tel19_1 search screen recent file");
        let ok = read_header(path.as_ptr() as usize, header.as_mut_ptr() as usize, finder_type(s) as u32);
        recent(read_usize(G_HUMAN_MANAGER), k, path.as_mut_ptr() as usize, 0x104, 1, 0, 2);
        if separator == -1 {
            separator = finder_separator(menu) as i16;
        }
        finder_format()(JOB_TEXT, 0xa7e728usize, k + 1, path.as_ptr()); // "%d. %s"
        finder_entry_if(ok != 0, 0x11, k, menu);
    }
}

/// The size of a club competition record (`G_CLUB_COMP_TABLE`).
const FINDER_COMP_SIZE: usize = 0x6b;
/// Club, nation and competition `id` of the game's tables.
unsafe fn finder_club(id: i32) -> usize {
    (read_usize(G_CLUB_TABLE) as isize + id as isize * club::SIZE as isize) as usize
}
unsafe fn finder_nation(id: i32) -> usize {
    (read_usize(G_NATION_TABLE) as isize + id as isize * nation::SIZE as isize) as usize
}
unsafe fn finder_comp(id: i32) -> usize {
    (read_usize(G_CLUB_COMP_TABLE) as isize + id as isize * FINDER_COMP_SIZE as isize) as usize
}
/// Copies the C string at `src` into `dst` (strcpy).
unsafe fn finder_copy(dst: &mut [u8], src: usize) {
    let mut i = 0;
    loop {
        let c = read_u8(src + i);
        dst[i] = c;
        if c == 0 {
            break;
        }
        i += 1;
    }
}

/// The View button and its menus (only with results; otherwise a greyed "View"): General, Contract, Stats, More Stats
/// (not for staff), Attributes, Scout Report (a list following a club), Player Notes and Other (not for staff), each
/// ticked by its view flag; with the Attributes view, the Attributes button and its groups (Non Playing for staff;
/// Physical, Mental, Goalkeeping, Defensive and Attacking for players) and Filtered.
unsafe fn finder_build_view_menu(b: &mut FinderBuild) {
    let lang = finder_lang();
    let s = b.s;
    lang(JOB_TEXT, 0x988004usize); // "View"
    if read_u16(b.results) == 0 {
        tel26_area_text([b.tab_area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0x2c, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);
        return;
    }
    let area = b.tab_area;
    let button = tel26_area_button([area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0, 0]);
    let menu = finder_popup(0x130, button);
    finder_view_option(0xa7e71c, 1, menu); // "  General"
    if finder_type(s) != 3 {
        finder_view_option(0xa7e710, 0x40, menu); // "  Contract"
        finder_view_option(0xa7e708, 2, menu); // "  Stats"
        finder_view_option(0xa7e6f8, 4, menu); // "  More Stats"
    }
    finder_view_option(0xa7e6e8, 8, menu); // "  Attributes"
    if read_i32(finder_current(s) + finder_list::ASSIGNED_CLUB) > -1 {
        finder_view_option(0xa7e6d8, 0x2000, menu); // "  Scout Report"
    }
    finder_view_option(0xa7e6c8, 0x10, menu); // "  Player Notes"
    if finder_type(s) != 3 {
        finder_view_option(0xa7e6c0, 0x20, menu); // "  Other"
    }
    if read_u32(read_usize(FINDER_VIEW_FLAGS)) & 8 == 0 {
        return;
    }
    covered!("tel19_1 search screen attributes menu");
    lang(JOB_TEXT, 0x98ece8usize); // "Attributes"
    let button = tel26_area_button([area, 2, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0, 0]);
    let groups = finder_popup(0x130, button);
    if finder_type(s) == 3 {
        finder_view_option(0xa7e6b0, 0x80, groups); // "  Non Playing"
    } else {
        finder_view_option(0xa7e6a4, 0x80, groups); // "  Physical"
        finder_view_option(0xa7e698, 0x100, groups); // "  Mental"
        finder_view_option(0xa7e688, 0x200, groups); // "  Goalkeeping"
        finder_view_option(0xa7e67c, 0x400, groups); // "  Defensive"
        finder_view_option(0xa7e670, 0x800, groups); // "  Attacking"
    }
    finder_view_option(0xa7e664, 0x1000, groups); // "  Filtered"
}

/// The Filters bar: Filters (id 0x13), Delete (id 0x16, enabled for a short list or a scout list not following a club,
/// with results; 0x70 wide in pick mode), then 0x19 down: with results, the rows that fit (the band below the bar less
/// 0x79, in lines of font 2), the page count (parameter 7; the page clamped to it) and, with more than one page, the
/// page buttons (0x17..0x1a), whose left edge the texts above the list stop at. Returns the bar's area.
unsafe fn finder_build_filters_bar(b: &mut FinderBuild) -> u32 {
    let lang = finder_lang();
    let s = b.s;
    let bar = finder_area([0x212, b.top as u32, 0x30c, (b.top + 0x14) as u32, 2, 0, 1, 0, 1, 0, u32::MAX]);
    lang(JOB_TEXT, 0xa7e65cusize); // "Filters"
    tel26_area_button([bar, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0x13, 0]);
    lang(JOB_TEXT, 0xa69fb4usize); // "Delete"
    let cur = finder_current(s);
    if !matches!(read_u8(cur + finder_list::TYPE), 2 | 3) && b.shown != 0 && read_i32(cur + finder_list::ASSIGNED_CLUB) < 0 {
        let width = if b.pick != 0 { 0x70 } else { 0x30 };
        tel26_area_button([bar, 1, 0, width, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0x16, 0]);
    } else {
        tel26_area_text([bar, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0x2c, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);
    }
    b.top += 0x19;
    b.page_left = 0;
    if read_u16(b.results) == 0 {
        return bar;
    }
    let h = exe::font_get_height(2);
    b.rows = ((b.bottom - b.top - 0x79) / h) as u8;
    let per = b.rows as i32 - 1;
    let count = read_i16(b.results) as i32;
    let pages = count / per + (count % per != 0) as i32;
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 7, pages as i16 as i32 as usize, 0);
    let view = read_usize(FINDER_VIEW_FLAGS);
    if read_i16(view + 0xe) > pages as i16 {
        write_i16(view + 0xe, pages as i16);
    }
    if pages as i16 > 1 {
        covered!("tel19_1 search screen page buttons");
        let f: unsafe extern "C" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(FINDER_PAGE_BUTTONS);
        b.page_left = f(0x30c, b.top as u32, read_u16(read_usize(FINDER_VIEW_FLAGS) + 0xe) as u32, pages as u32, 0x17, 0x18, 0x19, 0x1a) as i32;
    }
    bar
}

/// The line above the list: for a short list "Players currently on your shortlist"; for a quick search the famous
/// players and those of the human's nation (with a national team), of his region (database names in English: the text
/// copied untranslated) or the listed and free ones too; for a staff search the famous and regional staff; for a scout
/// list the scout's assignment (a club: a wrapped text with links to the scout and the club; a nation, a region, the
/// youth players, the next opposition or a competition: a button to the scout), "not assigned" or "No Scout Available".
unsafe fn finder_build_heading(b: &mut FinderBuild) {
    let lang = finder_lang();
    let s = b.s;
    let cur = finder_current(s);
    match read_u8(cur + finder_list::TYPE) {
        1 => {
            lang(JOB_TEXT, 0xa7e638usize); // "Players currently on your shortlist"
            b.heading_line();
        }
        2 => {
            if read_usize(b.mgr + staff::NATIONAL_JOB) != 0 {
                lang(JOB_TEXT, 0xa7e60cusize); // "Famous players and players in your nation"
            } else if read_u32(G_DATABASE_NAMES_LANGUAGE) == 0 {
                finder_copy(core::slice::from_raw_parts_mut(JOB_TEXT as *mut u8, 0x7d0), 0xa7e5ec); // "Famous players in your region"
            } else {
                lang(JOB_TEXT, 0xa7e5a0usize); // "Famous players and transfer listed/out of contract players in your region"
            }
            b.heading_line();
        }
        3 => {
            lang(JOB_TEXT, 0xa7e574usize); // "Famous staff and all staff in your region"
            b.heading_line();
        }
        4 => finder_build_scout_heading(b, cur),
        _ => {}
    }
}

/// The scout line of a scout list (see `finder_build_heading`).
unsafe fn finder_build_scout_heading(b: &mut FinderBuild, cur: usize) {
    let lang = finder_lang();
    let scout = get_scout(cur + finder_list::SEARCH);
    if scout == 0 {
        lang(JOB_TEXT, 0xa7e3e8usize); // "No Scout Available"
        b.heading_line();
        return;
    }
    let (mut club, mut nation, mut comp, mut youth, mut next) = (0u32, 0u32, 0u32, 0u32, 0u32);
    let mut region = [0u8; 4];
    let mut name = [0u8; 0xc8];
    let mut place = [0u8; 0xc8];
    let np = name.as_mut_ptr() as usize;
    let pp = place.as_mut_ptr() as usize;
    if get_scout_assignment(finder_current(b.s) + finder_list::SEARCH, &mut club, &mut nation, region.as_mut_ptr(), &mut comp, &mut youth, &mut next) == 0 {
        exe::database_format_staff_name(scout, np, 0xc8, 1);
        lang(JOB_TEXT, 0xa7e3fcusize, np); // "Scout <%s - scout name> not assigned"
        b.scout_line(scout);
        return;
    }
    if club as i32 > -1 {
        covered!("tel19_1 search screen scout at a club");
        let mut text = [0usize; 2];
        exe::new_text(text.as_mut_ptr(), 0usize);
        let t = text.as_mut_ptr() as usize;
        let mut line = 0i32;
        let c = finder_club(club as i32);
        finder_copy(&mut place, c + club::SHORT_NAME);
        exe::database_format_staff_name(scout, np, 0xc8, 2);
        let g = read_i8(finder_club(club as i32) + club::SHORT_GENDER_NAME) as i32;
        lang(JOB_TEXT, 0xa7e540usize, g, g, np, pp); // "Scout <%s - scout name> assigned to {}<%s - club>{}"
        exe::text_set_string(t, JOB_TEXT as *const u8);
        exe::text_add_link(t, np as *const u8, 0x7d7, scout);
        exe::text_add_link(t, pp as *const u8, 0x7d5, finder_club(club as i32));
        let right = b.right();
        let area = finder_area([0x6e, b.top as u32, right, (b.top + 0x14) as u32, 1, 0, 1, 0, 2, 0, u32::MAX]);
        let draw: unsafe extern "thiscall" fn(usize, u32, u32, u32, *mut i32, u32, u32, u32, u32) = core::mem::transmute(FINDER_TEXT_DRAW);
        draw(t, area, 0x6e, b.right(), &mut line, 1, job_colour(JOB_COLOUR_BDE8), 3, 0);
        exe::drop_text(text.as_mut_ptr());
        return;
    }
    if nation as i32 > -1 {
        let n = finder_nation(nation as i32);
        finder_copy(&mut place, n + nation::SHORT_NAME);
        exe::database_format_staff_name(scout, np, 0xc8, 2);
        let g = read_i8(finder_nation(nation as i32) + nation::SHORT_GENDER_NAME) as i32;
        lang(JOB_TEXT, 0xa7e508usize, g, g, np, pp); // "Scout <%s - scout name> assigned to {}<%s - nation>{}"
        b.scout_line(scout);
        return;
    }
    if region[0] as i8 > -1 {
        get_region_name(region[0] as u32, pp, 0xc8, u32::MAX);
        exe::database_format_staff_name(scout, np, 0xc8, 2);
        let form: unsafe extern "C" fn(u32) -> u8 = core::mem::transmute(FINDER_REGION_ARTICLE_FORM);
        let f1 = form(region[0] as u32) as i8 as i32;
        let f2 = form(region[0] as u32) as i8 as i32;
        lang(JOB_TEXT, 0xa7e4d0usize, f2, f1, np, pp); // "Scout <%s - scout name> assigned to {}<%s - region>{}"
        b.scout_line(scout);
        return;
    }
    if youth != 0 {
        exe::database_format_staff_name(scout, np, 0xc8, 2);
        lang(JOB_TEXT, 0xa7e49cusize, np); // "Scout <%s - scout name> assigned to youth players"
        b.scout_line(scout);
        return;
    }
    if next != 0 {
        exe::database_format_staff_name(scout, np, 0xc8, 2);
        lang(JOB_TEXT, 0xa7e460usize, np); // "Scout <%s - scout name> assigned to scout next opposition"
        b.scout_line(scout);
        return;
    }
    if comp as i32 > -1 {
        let c = finder_comp(comp as i32);
        finder_copy(&mut place, c + 0x38);
        exe::database_format_staff_name(scout, np, 0xc8, 2);
        let g = read_i8(finder_comp(comp as i32) + 0x52) as i32;
        lang(JOB_TEXT, 0xa7e424usize, g, g, np, pp); // "Scout <%s - scout name> assigned to {}<%s - competition>{}"
        b.scout_line(scout);
    }
}

/// The list's name (the scout manager's name for a list not from a file; at most 0x7cf characters, in yellow), "LOCKED"
/// for a locked list, and the found line: "N players/staff found", with "(M filtered out)" when the total is larger.
unsafe fn finder_build_title(b: &mut FinderBuild) {
    let lang = finder_lang();
    let s = b.s;
    b.top += 0x19;
    let cur = finder_current(s);
    if read_u32(cur + finder_list::FROM_FILE) == 0 {
        scout_manager_write_list_name(read_u8(s) as u32, cur + finder_list::NAME, 0x64);
    }
    exe::at_9466d0(JOB_TEXT, (finder_current(s) + finder_list::NAME) as *const u8, 0x7d0);
    write_u8(FINDER_TEXT_END, 0);
    finder_text_item([0x6e, b.top as u32, 0x30c, (b.top + 0x23) as u32, 2, 0, 0xc, 3, job_colour(JOB_COLOUR_BDEC), JOB_TEXT as u32, 0]);
    if read_u8(finder_current(s) + finder_list::LOCKED) != 0 {
        lang(JOB_TEXT, 0xa7e3dcusize); // "  LOCKED"
        finder_text_item([0x6e, b.top as u32, 0xd2, (b.top + 0x14) as u32, 1, 0, 0xd, 1, job_colour(JOB_COLOUR_BDEC), JOB_TEXT as u32, 0]);
    }
    b.top += 0x28;
    let found = read_i16(b.results);
    let total = b.l14 as i32;
    let staff = finder_type(s) == 3;
    if total > found as i32 {
        if found == 0 {
            lang(JOB_TEXT, if staff { 0xa7e3b0usize } else { 0xa7e2dc }, total); // "No staff/players found (<%d> filtered out)"
        } else if found > 1 {
            lang(JOB_TEXT, if staff { 0xa7e374usize } else { 0xa7e2a0 }, found as i32, total - found as i32); // "<%d> staff/players found (<%d> filtered out)"
        } else {
            lang(JOB_TEXT, if staff { 0xa7e348usize } else { 0xa7e274 }, total - 1); // "1 staff/player found (<%d> filtered out)"
        }
    } else if found == 0 {
        lang(JOB_TEXT, if staff { 0xa7e338usize } else { 0xa7e260 }); // "No staff/players found"
    } else if found > 1 {
        lang(JOB_TEXT, if staff { 0xa7e31cusize } else { 0xa7e244 }, found as i32); // "<%d> staff/players found"
    } else {
        lang(JOB_TEXT, if staff { 0xa7e30cusize } else { 0xa7e234 }); // "1 staff/player found"
    }
    finder_text_item([0x6e, (b.bottom - 0x23) as u32, 0x30c, b.bottom as u32, 2, 0, 0xc, 3, job_colour(JOB_COLOUR_BDE8), JOB_TEXT as u32, 0]);
    b.bottom -= 0x28;
    finder_area([0x6e, b.top as u32, 0x30c, b.bottom as u32, 1, 0, 1, 0, 2, 0, u32::MAX]);
}

/// Colours of the status cells (words): suspended and injured (0xaebdcc), long injured (0xae3190), the transfer status
/// box (0xae317c), and the name of a person at another club than his contract's (0xae3152).
const FINDER_COLOUR_SUSPENDED: usize = 0xaebdcc;
const FINDER_COLOUR_LONG_INJURY: usize = 0xae3190;
const FINDER_COLOUR_STATUS_BOX: usize = 0xae317c;
const FINDER_COLOUR_OTHER_CLUB: usize = 0xae3152;
/// The competition objects (`G_COMP_OBJECT_TABLE`-like table at 0xadadfc: one pointer per competition id; a division
/// with an object is a link).
const FINDER_COMP_OBJECTS: usize = 0xadadfc;
/// `get_player_status_code(person, 0, 0)` (0x4849f0, cdecl, al).
const FINDER_PLAYER_STATUS_CODE: usize = 0x4849f0;
/// STAFF_CONTRACT: `format_wage_text(out, style, cash flags)` (0x85e750, ret 0xc), `format_end_date_text(out)`
/// (0x85e8f0, ret 4), `get_release_clause_abbreviation(out, size)` (0x868020, ret 8, al).
const FINDER_CONTRACT_WAGE: usize = 0x85e750;
const FINDER_CONTRACT_END: usize = 0x85e8f0;
const FINDER_CONTRACT_RELEASES: usize = 0x868020;
/// PlayerStats: `get_field_label(field, out, size)` (0x7c1b20), `format_player_stat(player, 6, field, out, size, 1, 0,
/// 1)` (0x7c25b0) and `format_recent_ratings(player, out, size)` (0x7c1660), on [`G_PLAYER_STATS`].
const FINDER_STAT_LABEL: usize = 0x7c1b20;
const FINDER_STAT_FORMAT: usize = 0x7c25b0;
const FINDER_RECENT_RATINGS: usize = 0x7c1660;
/// The scout report slot: goals, assists, goals conceded (0x7bd870, 0x7bd8c0, 0x7bd910, ax) and the average rating
/// (0x7bd960, st(0)); `format_venue_marker(venue, out, size)` (0x7e4ca0, cdecl); `Date::get_short_date_string(out,
/// size, time)` (0x54a650, ret 0xc).
const FINDER_SLOT_GOALS: usize = 0x7bd870;
const FINDER_SLOT_ASSISTS: usize = 0x7bd8c0;
const FINDER_SLOT_CONCEDED: usize = 0x7bd910;
const FINDER_SLOT_AVERAGE: usize = 0x7bd960;
const FINDER_VENUE_MARKER: usize = 0x7e4ca0;
const FINDER_SHORT_DATE: usize = 0x54a650;
/// `Notes::find_note(manager, person)` (0x795750, ret 8) and `notes_get_reason_label(out, size, reason, style)`
/// (0x795b90, stdcall), on the notes at [0xb7433c]; `contract_manager_write_morale_text(person, out, size)` (0x4cc2e0)
/// on [`G_CONTRACT_MANAGER`]; `NATIONAL_TEAMS::class_by_national_side(person)` (0x76e070) on [0xb74318].
const FINDER_NOTES: usize = 0xb7433c;
const FINDER_FIND_NOTE: usize = 0x795750;
const FINDER_NOTE_REASON: usize = 0x795b90;
const FINDER_MORALE_TEXT: usize = 0x4cc2e0;
const FINDER_NATIONAL_TEAMS: usize = 0xb74318;
const FINDER_NATIONAL_SIDE: usize = 0x76e070;
/// `SearchFilters::get_attribute_minimum(attribute, is_non_player)` (0x808190, thiscall ret 8, al).
const FINDER_ATTRIBUTE_MINIMUM: usize = 0x808190;
/// The comparers `qsort` uses for the attribute columns (staff, players).
const FINDER_SORT_STAFF_COLUMNS: usize = 0x81af00;
const FINDER_SORT_PLAYER_COLUMNS: usize = 0x81ae60;
/// `Cash::Cash(value)` (0x450a90) and `Cash::format_display_amount(out, size, flags)` (0x450aa0, ret 0xc).
const FINDER_CASH_NEW: usize = 0x450a90;
const FINDER_CASH_FORMAT: usize = 0x450aa0;

/// The list's columns: the column weights of each view and the header buttons.
struct FinderColumns {
    general: [u8; 13],
    contract: [u8; 13],
    stats: [u8; 13],
    attributes: [u8; 13],
    scout: [u8; 13],
    notes: [u8; 7],
    other: [u8; 13],
}
const FINDER_COLUMNS: FinderColumns = FinderColumns {
    general: [6, 5, 0x16, 0x14, 0x14, 0xe, 0, 0, 0, 0, 0, 0, 0xa],
    contract: [6, 5, 0x16, 0xe, 0xe, 0xe, 0xc, 0, 0, 0, 0, 0, 0xa],
    stats: [6, 5, 0x16, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0xa],
    attributes: [6, 5, 0x16, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0xa],
    scout: [6, 5, 0x16, 6, 6, 6, 6, 0x1e, 0, 0, 0, 0, 0xa],
    notes: [2, 2, 0xa, 0, 6, 0, 0x10],
    other: [6, 5, 0x16, 8, 6, 6, 6, 0xa, 0xc, 6, 0, 0, 0xa],
};

/// The first header column (sorting by it): for a short list or a quick search of a human with a national team "Sqd"
/// (0x80000); for a scout list "Mat" (matches watched, 0x800000; `commented` adds the translator's comment) when it
/// follows a club, otherwise "Rec" (1); nothing for staff.
unsafe fn finder_first_header(b: &FinderBuild, area: u32, commented: bool) {
    let lang = finder_lang();
    let cur = finder_current(b.s);
    let bit = match read_u8(cur + finder_list::TYPE) {
        1 | 2 => {
            if read_usize(b.mgr + staff::NATIONAL_JOB) == 0 {
                return;
            }
            lang(JOB_TEXT, 0xa7e204usize); // "Sqd"
            0x80000
        }
        4 => {
            if read_i32(cur + finder_list::ASSIGNED_CLUB) > -1 {
                if commented {
                    lang(JOB_TEXT, 0xa7e20cusize, 0xad9c64usize); // "Mat<%s - COMMENT - Number of matches>"
                } else {
                    lang(JOB_TEXT, 0xa7e1d4usize); // "Mat"
                }
                0x800000
            } else {
                lang(JOB_TEXT, 0xa7e208usize); // "Rec"
                1
            }
        }
        _ => return,
    };
    finder_header(area, 0, 0xc, bit);
}

/// "Inf" and "   Name", the first three columns of most views.
unsafe fn finder_inf_name_headers(area: u32) {
    let lang = finder_lang();
    lang(JOB_TEXT, 0x98ea64usize); // "Inf"
    finder_header(area, 1, 0xc, 2);
    lang(JOB_TEXT, 0x98ea5cusize); // "   Name"
    finder_header(area, 2, 1, 4);
}

/// The last column: "Rep." for staff (0x200), "Value" for players (0x100).
unsafe fn finder_value_header(b: &FinderBuild, area: u32) {
    let lang = finder_lang();
    if finder_type(b.s) == 3 {
        lang(JOB_TEXT, 0xa7e1d8usize); // "Rep."
        finder_header(area, 0xc, 0xc, 0x200);
    } else {
        lang(JOB_TEXT, 0x98ea0cusize); // "Value"
        finder_header(area, 0xc, 0xc, 0x100);
    }
}

/// The attribute headers of the Attributes view: the columns are the group list of the view flags (staff: Non Playing;
/// players: Physical, Mental, Goalkeeping, Defensive, Attacking; the last set flag wins), or, with Filtered, the
/// attributes whose filter minimum is above 1 (staff 1..12, players 1..31, at most 9; the list rebuilt with 0xff ends),
/// sorted (`qsort` of 9 bytes with the column comparer, even a fixed group list); one header button (id 0x15) per
/// column from column 3, and "-" in the columns left of the 9.
unsafe fn finder_attribute_headers(b: &mut FinderBuild, area: u32) {
    let staff_list = finder_type(b.s) == 3;
    let flags = read_u32(read_usize(FINDER_VIEW_FLAGS));
    if staff_list {
        if flags & 0x80 != 0 {
            b.l14 = FINDER_COLUMNS_STAFF;
        }
    } else {
        if flags & 0x80 != 0 {
            b.l14 = FINDER_COLUMNS_PHYSICAL;
        }
        if flags & 0x100 != 0 {
            b.l14 = FINDER_COLUMNS_MENTAL;
        }
        if flags & 0x200 != 0 {
            b.l14 = FINDER_COLUMNS_GOALKEEPING;
        }
        if flags & 0x400 != 0 {
            b.l14 = FINDER_COLUMNS_DEFENSIVE;
        }
        if flags & 0x800 != 0 {
            b.l14 = FINDER_COLUMNS_ATTACKING;
        }
    }
    if flags & 0x1000 != 0 {
        covered!("tel19_1 search screen filtered attributes");
        b.l14 = FINDER_COLUMNS_FILTERED;
        write_u32(FINDER_COLUMNS_FILTERED, u32::MAX);
        write_u32(FINDER_COLUMNS_FILTERED + 4, u32::MAX);
        write_u16(FINDER_COLUMNS_FILTERED + 8, 0xffff);
        let (last, kind) = if staff_list { (0xc, 1) } else { (0x1f, 0) };
        let mut n = 0i8;
        let minimum: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(FINDER_ATTRIBUTE_MINIMUM);
        for a in 1..=last {
            if minimum(b.filters.as_ptr() as usize, a, kind) as i8 > 1 && n < 9 {
                write_u8(FINDER_COLUMNS_FILTERED + n as usize, a as u8);
                n += 1;
            }
        }
    }
    let list = b.l14;
    exe::crt_qsort(list, 9, 1, if staff_list { FINDER_SORT_STAFF_COLUMNS } else { FINDER_SORT_PLAYER_COLUMNS });
    let mut k = 0i8;
    while read_u8(b.l14 + k as usize) != 0xff {
        let a = read_u8(b.l14 + k as usize);
        if staff_list {
            database_get_non_player_attribute_name(a as u32, JOB_TEXT, 0x7d0, 1);
        } else {
            database_get_player_attribute_name(a as u32, JOB_TEXT, 0x7d0, 1);
        }
        tel26_area_button([area, k as u32 + 3, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0x15,
            read_i8(b.l14 + k as usize) as i32 as u32]);
        k += 1;
    }
    while k < 9 {
        tel26_area_text([area, k as u32 + 3, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), 0x98ba0c]); // "-"
        k += 1;
    }
}

/// The header row of the view and the list area under it; returns the list area (the Filters bar's when no view flag is
/// set). The header area is one row high (the row height: the band divided by the rows, rounded) from `top` + 8.
unsafe fn finder_build_header(b: &mut FinderBuild, list_area: u32) -> u32 {
    let lang = finder_lang();
    let per = b.rows as i32;
    let band = b.bottom - b.top - 0x10;
    b.row_height = ftol(band as f64 / per as f64 + read_f64(rdata::F64_0_5));
    let rh = b.row_height;
    let top = b.top;
    let header = |w: &[u8]| finder_area([0x6e, (top + 8) as u32, 0x30c, (rh + top + 8 - 1) as u32, w.len() as u32, w.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    let flags = read_u32(read_usize(FINDER_VIEW_FLAGS));
    let columns = FINDER_COLUMNS;
    let weights: &[u8];
    if flags & 1 != 0 {
        weights = &columns.general;
        let area = header(weights);
        finder_first_header(b, area, true);
        finder_inf_name_headers(area);
        if finder_type(b.s) == 3 {
            lang(JOB_TEXT, 0xa7e1f4usize); // "   Club/Nation"
            finder_header(area, 3, 1, 0x10);
        } else {
            lang(JOB_TEXT, 0x98ea44usize); // "   Club"
            finder_header(area, 3, 1, 8);
        }
        lang(JOB_TEXT, 0xa7e1e8usize); // "   Based"
        finder_header(area, 4, 1, 0x20);
        if finder_type(b.s) == 3 {
            lang(JOB_TEXT, 0xa7e1e0usize); // "   Job"
            finder_header(area, 5, 1, 0x80);
            lang(JOB_TEXT, 0xa7e1d8usize); // "Rep."
            finder_header(area, 0xc, 0xc, 0x200);
        } else {
            lang(JOB_TEXT, 0x98ea00usize); // "   Position"
            finder_header(area, 5, 1, 0x40);
            lang(JOB_TEXT, 0x98ea0cusize); // "Value"
            finder_header(area, 0xc, 0xc, 0x100);
        }
    } else if flags & 0x40 != 0 {
        weights = &columns.contract;
        let area = header(weights);
        finder_first_header(b, area, false);
        finder_inf_name_headers(area);
        lang(JOB_TEXT, 0x98ea44usize); // "   Club"
        finder_header(area, 3, 1, 8);
        lang(JOB_TEXT, 0x98ea34usize); // "   Basic Wage"
        finder_header(area, 4, 1, 0x100000);
        lang(JOB_TEXT, 0x98ea20usize); // "   Contract Expiry"
        finder_header(area, 5, 1, 0x200000);
        lang(JOB_TEXT, 0x98ea14usize); // "   Releases"
        finder_header(area, 6, 1, 0x400000);
        lang(JOB_TEXT, 0x98ea0cusize); // "Value"
        finder_header(area, 0xc, 0xc, 0x100);
    } else if flags & 6 != 0 {
        weights = &columns.stats;
        let area = header(weights);
        finder_first_header(b, area, false);
        finder_inf_name_headers(area);
        let list = if read_u32(read_usize(FINDER_VIEW_FLAGS)) & 2 != 0 { FINDER_COLUMNS_STATS } else { FINDER_COLUMNS_MORE_STATS };
        let label: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(FINDER_STAT_LABEL);
        let mut k = 0usize;
        while read_u8(list + k) != 0xff {
            label(read_usize(G_PLAYER_STATS), read_u8(list + k) as u32, JOB_TEXT, 0x7d0);
            tel26_area_button([area, k as u32 + 3, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0x14,
                read_i8(list + k) as i32 as u32]);
            k += 1;
        }
        finder_value_header(b, area);
    } else if flags & 8 != 0 {
        weights = &columns.attributes;
        let area = header(weights);
        finder_first_header(b, area, false);
        finder_inf_name_headers(area);
        finder_attribute_headers(b, area);
        finder_value_header(b, area);
    } else if flags & 0x2000 != 0 {
        weights = &columns.scout;
        let area = header(weights);
        lang(JOB_TEXT, 0xa7e1d4usize); // "Mat"
        finder_header(area, 0, 0xc, 0x800000);
        finder_inf_name_headers(area);
        lang(JOB_TEXT, 0xa78490usize); // "Gls"
        finder_header(area, 3, 0xc, 0x1000000);
        lang(JOB_TEXT, 0xa7845cusize); // "Asts"
        finder_header(area, 4, 0xc, 0x2000000);
        lang(JOB_TEXT, 0xa78464usize, 0xad9c64usize); // "Con<%s - COMMENT - player goals conceded>"
        finder_header(area, 5, 0xc, 0x4000000);
        lang(JOB_TEXT, 0xa78434usize); // "Av R"
        finder_header(area, 6, 0xc, 0x8000000);
        lang(JOB_TEXT, 0xa7e1c4usize); // "  Last Match"
        finder_header(area, 7, 1, 0x10000000);
        lang(JOB_TEXT, 0x98ea0cusize); // "Value"
        finder_header(area, 0xc, 0xc, 0x100);
    } else if flags & 0x10 != 0 {
        weights = &columns.notes;
        let area = header(weights);
        lang(JOB_TEXT, 0xa7e1bcusize); // "   Inf"
        finder_header(area, 1, 1, 4);
        lang(JOB_TEXT, 0x98ea5cusize); // "   Name"
        finder_header(area, 2, 1, 4);
        lang(JOB_TEXT, 0xa7e1b4usize); // "Reason"
        finder_header(area, 4, 0xc, 0x20000000);
        lang(JOB_TEXT, 0xa7e1acusize); // "Note"
        finder_header(area, 6, 0xc, 0x20000000);
    } else if flags & 0x20 != 0 {
        weights = &columns.other;
        let area = header(weights);
        finder_first_header(b, area, false);
        finder_inf_name_headers(area);
        lang(JOB_TEXT, 0x98e9e0usize); // "Nat."
        finder_header(area, 3, 0xc, 0x1000);
        lang(JOB_TEXT, 0x98e9dcusize); // "Age"
        finder_header(area, 4, 0xc, 0x2000);
        lang(JOB_TEXT, 0x98e9d4usize); // "Caps"
        finder_header(area, 5, 0xc, 0x4000);
        lang(JOB_TEXT, 0xa7e184usize, 0xad9c64usize); // "I.Gls<%s - COMMENT International Goals>"
        finder_header(area, 6, 0xc, 0x8000);
        lang(JOB_TEXT, 0x98e9f8usize); // "Form"
        finder_header(area, 7, 0xc, 0x10000);
        lang(JOB_TEXT, 0x98e9f0usize); // "Morale"
        finder_header(area, 8, 0xc, 0x20000);
        lang(JOB_TEXT, 0x98e9e8usize); // "Cond."
        finder_header(area, 9, 0xc, 0x40000);
        finder_value_header(b, area);
    } else {
        return list_area;
    }
    finder_area([0x6e, (rh + top + 8) as u32, 0x30c, (b.bottom - 8) as u32, weights.len() as u32, weights.as_ptr() as u32,
        (b.rows as u32).wrapping_sub(1) & 0xff, 0, 1, 0, u32::MAX])
}

/// One result row being written: the list area, the row, the person, the result entry, the club the person is shown
/// with (a dword that survives from the previous row: the international-duty status reads it before it is updated),
/// the scout entry (lists following a club) and the B team of the contract's club.
struct FinderRow {
    area: u32,
    row: i32,
    person: usize,
    entry: usize,
    club: usize,
    slot: usize,
    b_team: usize,
}

/// A text cell of the list (`ScreenManager::add_area_text`): column, the four style fields, the colour and the text.
unsafe fn finder_cell(r: &FinderRow, col: u32, a: [u32; 4], colour: u32, text: usize) -> u32 {
    tel26_area_text([r.area, col, r.row as u32, a[0], a[1], a[2], a[3], colour, text as u32])
}
/// A button cell of the list (`ScreenManager::add_area_button`).
unsafe fn finder_cell_button(r: &FinderRow, col: u32, a: [u32; 4], colour: u32, text: usize, id: u32, value: usize) -> u32 {
    tel26_area_button([r.area, col, r.row as u32, a[0], a[1], a[2], a[3], colour, text as u32, id, value as u32])
}
/// The contract of a person (`CONTRACT_MANAGER::get_contract`), stored in `G_CONTRACT_LOOKUP_RESULT` when `keep`.
unsafe fn finder_contract(p: usize, keep: bool) -> usize {
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
    if keep {
        write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    }
    c
}
/// The club of a person's contract (0 without one), the lookup stored.
unsafe fn finder_contract_club(p: usize) -> usize {
    let c = finder_contract(p, true);
    if c == 0 { 0 } else { finder_club(read_i32(c + 4)) }
}

/// The first cell of a row: for a short list or a quick search, the person's national squad (A, B, U23 or U21, a link
/// to the national team) when both he and the human have a national team; for a scout list following a club, the
/// matches watched (a link 0x7e7) or nothing; for another scout list, the recommendation stars (by the filters' flag
/// mask: the first or the second rating, or the better one; five stars in yellow); empty for staff.
unsafe fn finder_row_first(b: &FinderBuild, r: &mut FinderRow) {
    let lang = finder_lang();
    let (p, s) = (r.person, b.s);
    let cur = finder_current(s);
    let dark = job_colour(JOB_COLOUR_BE04);
    let grey = job_colour(JOB_COLOUR_3184);
    match read_u8(cur + finder_list::TYPE) {
        1 | 2 => {
            let nt = read_usize(p + staff::NATIONAL_JOB);
            if read_usize(b.mgr + staff::NATIONAL_JOB) == 0 || nt == 0 {
                finder_cell(r, 0, [0x30, dark, 0xc, 1], grey, 0xad9c64);
                return;
            }
            let side: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FINDER_NATIONAL_SIDE);
            let (id, text) = match side(read_usize(FINDER_NATIONAL_TEAMS), p) {
                0 => (0x7d5, 0xa7e144usize), // "A<%s - COMMENT A = Nation main squad>"
                1 => (0x7d6, 0xa7e11c), // "B<%s - COMMENT B = Nation 'B' squad>"
                _ => {
                    let n = read_usize(nt + club::NATION);
                    let u23 = read_u8(read_usize(read_usize(FINDER_NATIONAL_TEAMS) + 0x36).wrapping_add(read_usize(n))) != 0;
                    (0x7d5, if u23 { 0xa7e0ec } else { 0xa7e0bc }) // "U23<..>" / "U21<..>"
                }
            };
            lang(JOB_TEXT, text, 0xad9c64usize);
            finder_cell_button(r, 0, [0x30, dark, 0xc, 1], grey, JOB_TEXT, id, read_usize(p + staff::NATIONAL_JOB));
        }
        4 => {
            if read_i32(cur + finder_list::ASSIGNED_CLUB) > -1 {
                r.slot = find_club_search_entry(read_usize(G_SCOUT_MANAGER), p);
                if r.slot != 0 {
                    finder_format()(JOB_TEXT, 0x98806cusize, read_i8(r.slot + 0x1a1) as i32); // "%d"
                    finder_cell_button(r, 0, [0x30, dark, 0xc, 1], grey, JOB_TEXT, 0x7e7, p);
                } else {
                    finder_cell(r, 0, [0x30, dark, 0xc, 1], grey, 0);
                }
                return;
            }
            let e = r.entry;
            let rating = if read_u32(b.filters.as_ptr() as usize + 4) & 0x1000000 != 0 {
                read_i8(e + 0x29)
            } else if (search_filters_get_flag_mask(b.filters.as_ptr() as usize) >> 32) as u32 & 0x2000000 != 0 {
                read_i8(e + 0x2a)
            } else {
                read_i8(e + 0x29).max(read_i8(e + 0x2a))
            };
            let (text, colour) = match rating {
                5 => (0xa7e17c, job_colour(JOB_COLOUR_BDEC)), // "*****"
                4 => (0xa7e174, grey),
                3 => (0xa7e170, grey),
                2 => (0xa7e16c, grey),
                1 => (0x989ad8, grey),
                _ => (0, grey),
            };
            finder_cell(r, 0, [0x30, dark, 0xc, 1], colour, text);
        }
        _ => {
            finder_cell(r, 0, [0x30, dark, 0xc, 1], grey, 0xad9c64);
        }
    }
}

/// The status cell (column 1): the person's status code (`get_player_status_code`) as a small box: work permit, cup
/// ineligible, ineligible, on international duty (a link to the national team, the text left as it was; without a team
/// a plain item), suspended, injured, foreign, long injury, yellow cards, and the transfer statuses (training, bid,
/// wanted, scouted, requested, listed, loan, contract, retiring, Bosman, unhappy, future), each a link (0x7d8 injury,
/// 0x7d9 discipline, 0x7da contract, 0x7db transfer).
unsafe fn finder_row_status(r: &FinderRow) {
    let lang = finder_lang();
    let p = r.person;
    let code: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(FINDER_PLAYER_STATUS_CODE);
    let add = |kind: u32, c: u32, text_colour: u32, id: u32, value: usize| {
        let sm: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(FINDER_SM_ADD_ITEM);
        sm(G_SCREEN_MANAGER, kind, 0, 0, 0, 0, 1, r.row as u32, 0x30, c, c, 0xc, 1, text_colour, JOB_TEXT as u32, 0, id, value as u32, r.area);
    };
    let dark = job_colour(JOB_COLOUR_BE04);
    let grey = job_colour(JOB_COLOUR_3184);
    let boxed = job_colour(FINDER_COLOUR_STATUS_BOX);
    let yellow = job_colour(JOB_COLOUR_BDEC);
    match code(p, 0, 0) {
        1 => {
            lang(JOB_TEXT, 0x98e9c8usize); // "Wpm"
            add(2, dark, grey, 0x7db, p);
        }
        2 => {
            lang(JOB_TEXT, 0x98e998usize, 0xad9c64usize); // "Cup<%s - COMMENT - ineligible for cup match>"
            add(2, dark, grey, 0x7db, p);
        }
        3 => {
            lang(JOB_TEXT, 0x98e994usize); // "Ine"
            add(2, dark, grey, 0x7db, p);
        }
        4 => {
            let club = exe::nationalteams_get_international_duty_club(read_usize(FINDER_NATIONAL_TEAMS), p, G_CURRENT_DATE, r.club, 0, 0);
            if club != 0 {
                covered!("tel19_1 search screen international duty");
                add(2, dark, grey, 0x7d5, club as usize);
            } else {
                add(1, dark, grey, 0, 0);
            }
        }
        5 => {
            lang(JOB_TEXT, 0x98e990usize); // "Sus"
            add(2, job_colour(FINDER_COLOUR_SUSPENDED), grey, 0x7d9, p);
        }
        6 => {
            lang(JOB_TEXT, 0x98e98cusize); // "Inj"
            add(2, job_colour(FINDER_COLOUR_SUSPENDED), grey, 0x7d8, p);
        }
        7 | 9 => {
            lang(JOB_TEXT, 0x98e984usize); // "Fgn"
            add(2, dark, grey, 0x7db, p);
        }
        8 => {
            lang(JOB_TEXT, 0x98e98cusize); // "Inj"
            add(2, job_colour(FINDER_COLOUR_LONG_INJURY), grey, 0x7d8, p);
        }
        0xa => {
            lang(JOB_TEXT, 0x98e978usize); // "Yel"
            let c9 = exe::graphics_rgb_to_pixel(0xb4, 0xb4, 0, 0);
            let c8 = exe::graphics_rgb_to_pixel(0xb4, 0xb4, 0, 0);
            let sm: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute(FINDER_SM_ADD_ITEM);
            sm(G_SCREEN_MANAGER, 2, 0, 0, 0, 0, 1, r.row as u32, 0x30, c8, c9, 0xc, 1, job_colour(JOB_COLOUR_BDEA), JOB_TEXT as u32, 0, 0x7d9, p as u32, r.area);
        }
        0x10 => {
            lang(JOB_TEXT, 0x98e900usize); // "Ctr"
            add(2, boxed, yellow, 0x7da, p);
        }
        c @ (0xb..=0xf | 0x11..=0x14 | 0x17 | 0x18) => {
            match c {
                0x17 => lang(JOB_TEXT, 0xa7e08cusize, 0xad9c64usize), // "Sct<%s - COMMENT - scouted by human manager>"
                0xe => lang(JOB_TEXT, 0x98e908usize, 0xad9c64usize), // "Lst<%s - COMMENT - transfer listed by club>"
                _ => lang(JOB_TEXT, match c {
                    0x18 => 0x98e970usize, // "Trn"
                    0xb => 0x98e974, // "Bid"
                    0xc => 0x98e96c, // "Wnt"
                    0xd => 0x98e934, // "Req"
                    0xf => 0x98e904, // "Loa"
                    0x11 => 0x98e8f8, // "Ret "
                    0x12 => 0x98e8f0, // "Bos "
                    0x13 => 0x98e8e8, // "Unh "
                    _ => 0x98e8e0, // "Fut "
                }),
            };
            add(2, boxed, yellow, 0x7db, p);
        }
        _ => {}
    }
}

/// The name cell (column 2) and what the other cells need: the club the person is shown with (a staff member without
/// a club: his national team unless he is its 0xb job) and, for players, the B team of his contract's club. A person on
/// the short list (in another list) is yellow; one of the human's club (by club or contract) is grey, except a player of
/// the club the list follows; one whose contract is with another club than his own (and not its B team) is in the
/// other-club colour; the rest pale grey. A link 0x7d7 to the person.
unsafe fn finder_row_name(b: &FinderBuild, r: &mut FinderRow) {
    let (p, s) = (r.person, b.s);
    let mut club = read_usize(p + staff::CLUB_JOB);
    r.club = club;
    r.b_team = 0;
    if search_edit_session_get_list_type(s, u32::MAX) == 3 {
        let nt = read_usize(p + staff::NATIONAL_JOB);
        if club == 0 && nt != 0 && read_u8(p + staff::JOB_FOR_NATION) != 0xb {
            club = nt;
            r.club = club;
        }
    } else {
        let c = finder_contract(p, true);
        if c != 0 && finder_club(read_i32(c + 4)) != 0 {
            let cc = finder_contract_club(p);
            r.b_team = exe::club_get_linked_b_team(cc, core::ptr::null_mut(), 1);
        }
    }
    exe::database_format_staff_name(p, JOB_TEXT, 0x7d0, 3);
    let style = [1, 0, 1, 2];
    if search_edit_session_get_list_type(s, u32::MAX) != 1 && exe::searcheditsession_is_player_on_short_list(s, p) != 0 {
        finder_cell_button(r, 2, style, job_colour(JOB_COLOUR_BDEC), JOB_TEXT, 0x7d7, p);
        return;
    }
    let mine = read_usize(b.mgr + staff::CLUB_JOB);
    let human_club = mine != 0 && (read_usize(p + staff::CLUB_JOB) == mine || finder_contract_club(p) == read_usize(b.mgr + staff::CLUB_JOB));
    if human_club {
        let followed = r.slot != 0 && read_i32(read_usize(p + staff::CLUB_JOB)) == search_edit_session_get_assigned_club(s, u32::MAX);
        if !followed {
            finder_cell_button(r, 2, style, job_colour(JOB_COLOUR_317E), JOB_TEXT, 0x7d7, p);
            return;
        }
    }
    let colour = if finder_contract(p, false) != 0 && read_usize(p + staff::CLUB_JOB) != 0 && {
        let cc = finder_contract_club(p);
        let own = read_usize(p + staff::CLUB_JOB);
        own != cc && own != r.b_team
    } {
        job_colour(FINDER_COLOUR_OTHER_CLUB)
    } else {
        job_colour(JOB_COLOUR_3184)
    };
    finder_cell_button(r, 2, style, colour, JOB_TEXT, 0x7d7, p);
}

/// The view's cells of a row (the first view flag set among General, Contract, Stats, More Stats, Attributes, Scout
/// Report, Player Notes and Other).
unsafe fn finder_row_view(b: &FinderBuild, r: &mut FinderRow) {
    let lang = finder_lang();
    let format = finder_format();
    let (p, s) = (r.person, b.s);
    let flags = read_u32(read_usize(FINDER_VIEW_FLAGS));
    let left = [1, 0, 1, 2];
    let centre = [1, 0, 0xc, 2];
    let mut buf = [0u8; 0xc8];
    let bp = buf.as_mut_ptr() as usize;
    if flags & 1 != 0 {
        // General: the club (or "No Club"), where he is based (the division of a club in the human's nation when it
        // shows; otherwise the club's or the person's nation), and the job (staff) or the positions (players)
        let club = r.club;
        let mut nation_from_club = true;
        let mut based_done = false;
        if club != 0 {
            format(JOB_TEXT, 0x987ff0usize, club + club::SHORT_NAME); // "  %s"
            finder_cell_button(r, 3, left, job_colour(JOB_COLOUR_BDE8), JOB_TEXT, 0x7d5, club);
            let div = read_usize(club + club::DIVISION);
            let mine = read_usize(b.mgr + staff::CLUB_JOB);
            if div != 0 && mine != 0 && read_usize(div + 0x5d) == read_usize(mine + club::NATION) {
                format(JOB_TEXT, 0x987ff0usize, div + 0x38); // "  %s"
                let shown = if read_usize((read_usize(FINDER_COMP_OBJECTS) as isize + read_i32(div) as isize * 4) as usize) != 0 {
                    finder_cell_button(r, 4, left, job_colour(JOB_COLOUR_BDEC), JOB_TEXT, 0x7d0, div)
                } else {
                    finder_cell(r, 4, left, job_colour(JOB_COLOUR_BDEC), JOB_TEXT)
                };
                based_done = shown as u16 != 0xffff;
            }
        } else {
            lang(JOB_TEXT, if search_edit_session_get_list_type(s, u32::MAX) == 3 { 0xa7e078usize } else { 0xa7e06c }); // "  No Club/Nation" / "  No Club"
            finder_cell(r, 3, left, job_colour(JOB_COLOUR_BDE8), JOB_TEXT);
            nation_from_club = false;
        }
        if !based_done {
            let mut n = if nation_from_club { read_usize(club + club::NATION) } else { 0 };
            if n == 0 {
                n = read_usize(p + staff::NATION);
            }
            if n != 0 {
                format(JOB_TEXT, 0x987ff0usize, n + 0x38); // "  %s"
                let team = exe::database_get_national_team(n);
                finder_cell_button(r, 4, left, job_colour(JOB_COLOUR_BDEC), JOB_TEXT, 0x7d5, team);
            }
        }
        if search_edit_session_get_list_type(s, u32::MAX) == 3 {
            let job = if read_usize(p + staff::CLUB_JOB) != 0 {
                read_u8(p + staff::JOB_FOR_CLUB)
            } else if read_usize(p + staff::NATIONAL_JOB) != 0 && read_u8(p + staff::JOB_FOR_NATION) != 0xb {
                read_u8(p + staff::JOB_FOR_NATION)
            } else {
                0
            };
            let name = exe::database_get_job_name(job as u32, 1, 1, core::ptr::null_mut());
            format(JOB_TEXT, 0x987ff0usize, name); // "  %s"
        } else {
            database_format_player_positions(read_usize(p + staff::PLAYER), bp, 0xc8, 1);
            format(JOB_TEXT, 0x987ff0usize, bp); // "  %s"
        }
        finder_cell(r, 5, left, job_colour(JOB_COLOUR_BDD4), JOB_TEXT);
    } else if flags & 0x40 != 0 {
        // Contract: the club, the wage (the short form when the long one has more than 8 characters), the expiry and the
        // release clauses (red when they bind, a link when there are any)
        let club = r.club;
        if club != 0 {
            format(JOB_TEXT, 0x987ff0usize, club + club::SHORT_NAME); // "  %s"
            finder_cell_button(r, 3, left, job_colour(JOB_COLOUR_BDE8), JOB_TEXT, 0x7d5, club);
        } else {
            lang(JOB_TEXT, 0xa7e06cusize); // "  No Club"
            finder_cell(r, 3, left, job_colour(JOB_COLOUR_BDE8), JOB_TEXT);
        }
        let c = finder_contract(p, false);
        let wage: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FINDER_CONTRACT_WAGE);
        if c != 0 {
            wage(c, bp, 0, 2);
            if finder_strlen(bp) > 8 {
                wage(c, bp, 0, 1);
            }
            format(JOB_TEXT, 0x987ff0usize, bp); // "  %s"
        } else {
            format(JOB_TEXT, 0x98e8c8usize); // "  -"
        }
        finder_cell(r, 4, left, job_colour(JOB_COLOUR_BDEC), JOB_TEXT);
        if c != 0 {
            let end: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FINDER_CONTRACT_END);
            end(c, bp);
            format(JOB_TEXT, 0x987ff0usize, bp); // "  %s"
        } else {
            format(JOB_TEXT, 0x98e8c8usize); // "  -"
        }
        finder_cell(r, 5, left, job_colour(JOB_COLOUR_BDEC), JOB_TEXT);
        if c != 0 {
            let releases: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FINDER_CONTRACT_RELEASES);
            let k = releases(c, bp, 0x7d0);
            format(JOB_TEXT, 0x987ff0usize, bp); // "  %s"
            match k {
                2 => {
                    finder_cell_button(r, 6, left, job_colour(G_GUI_COLOUR_RED), JOB_TEXT, 0x7da, p);
                }
                1 => {
                    finder_cell_button(r, 6, left, job_colour(JOB_COLOUR_BDD4), JOB_TEXT, 0x7da, p);
                }
                _ => {
                    finder_cell(r, 6, left, job_colour(JOB_COLOUR_BDD4), JOB_TEXT);
                }
            }
        } else {
            finder_cell(r, 6, left, job_colour(JOB_COLOUR_BDD4), 0x98e8c8); // "  -"
        }
    } else if flags & 6 != 0 {
        // Stats / More Stats: one formatted statistic per column (the rating, 0x11, in yellow)
        let list = if flags & 2 != 0 { FINDER_COLUMNS_STATS } else { FINDER_COLUMNS_MORE_STATS };
        let stat: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32, u32, u32) = core::mem::transmute(FINDER_STAT_FORMAT);
        let mut k = 0usize;
        while read_u8(list + k) != 0xff {
            let field = read_u8(list + k);
            stat(read_usize(G_PLAYER_STATS), read_usize(p + staff::PLAYER), 6, field as u32, JOB_TEXT, 0x7d0, 1, 0, 1);
            let colour = if read_u8(list + k) == 0x11 { JOB_COLOUR_BDEC } else { JOB_COLOUR_BDE8 };
            finder_cell(r, k as u32 + 3, centre, job_colour(colour), JOB_TEXT);
            k += 1;
        }
    } else if flags & 8 != 0 {
        // Attributes: the column list chosen by the header; staff show every value, players only the ones the human
        // can see ("-" otherwise)
        let cols = b.l14;
        if search_edit_session_get_list_type(s, u32::MAX) == 3 {
            let mut k = 0usize;
            while read_u8(cols + k) != 0xff {
                let v = exe::staff_get_non_player_attribute(p, read_u8(cols + k) as u32);
                format(JOB_TEXT, 0x98806cusize, v as i32); // "%d"
                finder_cell(r, k as u32 + 3, centre, job_colour(JOB_COLOUR_BDE8), JOB_TEXT);
                k += 1;
            }
        } else {
            let mut k = 0usize;
            while read_u8(cols + k) != 0xff {
                let a = read_u8(cols + k) as u32;
                if exe::staff_is_player_attribute_visible(b.mgr, p, a) != 0 {
                    let v = exe::staff_get_player_attribute(p, read_u8(cols + k) as u32);
                    format(JOB_TEXT, 0x98806cusize, v as i32); // "%d"
                    finder_cell(r, k as u32 + 3, centre, job_colour(JOB_COLOUR_BDE8), JOB_TEXT);
                } else {
                    finder_cell(r, k as u32 + 3, centre, job_colour(JOB_COLOUR_BDE8), 0x98ba0c); // "-"
                }
                k += 1;
            }
        }
    } else if flags & 0x2000 != 0 {
        // Scout Report: goals, assists, goals conceded and the average rating of the watched matches, and the last one
        // (date, opponent and venue as a wrapped text with a link to the opponent, yellow when he is known)
        let slot = r.slot;
        for (col, f) in [(3u32, FINDER_SLOT_GOALS), (4, FINDER_SLOT_ASSISTS), (5, FINDER_SLOT_CONCEDED)] {
            if slot != 0 {
                let g: unsafe extern "thiscall" fn(usize) -> i16 = core::mem::transmute(f);
                format(JOB_TEXT, 0x98806cusize, g(slot) as i32); // "%d"
            } else {
                format(JOB_TEXT, 0x98ba0cusize); // "-"
            }
            finder_cell(r, col, centre, job_colour(JOB_COLOUR_BDE8), JOB_TEXT);
        }
        if slot != 0 {
            let avg: unsafe extern "thiscall" fn(usize) -> f64 = core::mem::transmute(FINDER_SLOT_AVERAGE);
            format(JOB_TEXT, 0x98fe94usize, avg(slot)); // "%4.2f"
        } else {
            format(JOB_TEXT, 0x98ba0cusize); // "-"
        }
        finder_cell(r, 6, centre, job_colour(JOB_COLOUR_BDE8), JOB_TEXT);
        let n = if slot != 0 { read_i8(slot + 0x1a1) } else { 0 };
        let e = if n > 0 { player_search_slot_get_match_entry(slot, (n - 1) as u8 as u32) } else { 0 };
        if e != 0 {
            covered!("tel19_1 search screen last match");
            let venue: unsafe extern "C" fn(u32, usize, u32) -> i32 = core::mem::transmute(FINDER_VENUE_MARKER);
            venue(read_u8(e + 0xe) as u32, bp, 0xc8);
            let mut date = [0u8; 8];
            let d = exe::new_date(date.as_mut_ptr() as usize, read_u16(e) as u32, read_u16(e + 2) as u32);
            let mut when = [0u8; 0xc8];
            let short: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FINDER_SHORT_DATE);
            short(d, when.as_mut_ptr() as usize, 0xc8, 0xff);
            let opp = finder_club(read_i32(e + 4).wrapping_abs());
            let g = read_i8(opp + club::SHORT_GENDER_NAME) as i32;
            lang(JOB_TEXT, 0xa7e028usize, 0x64, g, g, when.as_ptr(), opp + club::SHORT_NAME, bp); // "  {}<%s - date> - {}<%s - opposition team>{} (<%s - venue e.g. H>)"
            let text = b.text;
            exe::text_set_string(text, JOB_TEXT as *const u8);
            let opp = finder_club(read_i32(e + 4).wrapping_abs());
            exe::text_add_link(text, (opp + club::SHORT_NAME) as *const u8, 0x7d5, opp);
            let colour = if read_i32(e + 4) > -1 { JOB_COLOUR_BDEC } else { JOB_COLOUR_317E };
            let draw: unsafe extern "thiscall" fn(usize, u32, u32, u32, *mut i32, u32, u32, u32, u32) = core::mem::transmute(FINDER_TEXT_DRAW);
            draw(text, r.area, 0x1f4, 0x2c6, &mut r.row, 2, job_colour(colour), 2, 0);
        } else {
            format(JOB_TEXT, 0x98ba0cusize); // "-" (written, but the cell gets no text)
            finder_cell(r, 7, centre, job_colour(JOB_COLOUR_BDEC), 0);
        }
    } else if flags & 0x10 != 0 {
        // Player Notes: the reason and the note of the human's note on the person (links 0x21)
        let find: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FINDER_FIND_NOTE);
        let note = find(read_usize(FINDER_NOTES), b.mgr, p);
        if note != 0 {
            let reason: unsafe extern "stdcall" fn(usize, u32, u32, u32) -> usize = core::mem::transmute(FINDER_NOTE_REASON);
            reason(JOB_TEXT, 0x3e8, read_u8(note + 0x113) as u32, 1);
        } else {
            format(JOB_TEXT, 0x98ba0cusize); // "-"
        }
        finder_cell_button(r, 4, centre, job_colour(JOB_COLOUR_BDE8), JOB_TEXT, 0x21, p);
        if note != 0 && read_u8(note + 4) != 0 {
            format(JOB_TEXT, 0x991944usize, note + 4); // "%s"
            finder_cell_button(r, 6, left, job_colour(JOB_COLOUR_BDE8), JOB_TEXT, 0x21, p);
        } else {
            format(JOB_TEXT, 0x98ba0cusize); // "-"
            finder_cell_button(r, 6, centre, job_colour(JOB_COLOUR_BDE8), JOB_TEXT, 0x21, p);
        }
    } else if flags & 0x20 != 0 {
        // Other: nationality (a link to the national team), age, caps, international goals, form, morale and condition
        let n = read_usize(p + staff::NATION);
        if n != 0 {
            let team = exe::database_get_national_team(n);
            finder_cell_button(r, 3, centre, job_colour(JOB_COLOUR_BDE8), read_usize(p + staff::NATION) + nation::THREE_LETTER_NAME, 0x7d5, team);
        }
        format(JOB_TEXT, 0x98806cusize, read_i8(p + staff::AGE) as i32); // "%d"
        finder_cell(r, 4, centre, job_colour(JOB_COLOUR_BDE8), JOB_TEXT);
        format(JOB_TEXT, 0x98806cusize, read_u8(p + staff::INT_APPS) as u32); // "%d"
        finder_cell(r, 5, centre, job_colour(JOB_COLOUR_BDE8), JOB_TEXT);
        format(JOB_TEXT, 0x98806cusize, read_u8(p + staff::INT_GOALS) as u32); // "%d"
        finder_cell(r, 6, centre, job_colour(JOB_COLOUR_BDE8), JOB_TEXT);
        let ratings: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FINDER_RECENT_RATINGS);
        ratings(read_usize(G_PLAYER_STATS), read_usize(p + staff::PLAYER), JOB_TEXT, 0x7d0);
        finder_cell(r, 7, [1, 0, 0xc, 1], job_colour(JOB_COLOUR_BDEC), JOB_TEXT);
        let morale: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FINDER_MORALE_TEXT);
        morale(read_usize(G_CONTRACT_MANAGER), p, bp, 0xc8);
        format(JOB_TEXT, 0x991944usize, bp); // "%s"
        finder_cell(r, 8, centre, job_colour(JOB_COLOUR_BDEC), JOB_TEXT);
        if exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2 {
            lang(JOB_TEXT, 0x98e98cusize); // "Inj"
        } else {
            let c = exe::injury_get_condition(read_usize(G_INJURY), p, 1);
            format(JOB_TEXT, 0x98e8d4usize, c as i32); // "%d%%"
        }
        finder_cell(r, 9, centre, job_colour(JOB_COLOUR_BDD4), JOB_TEXT);
    }
}

/// Length of the C string at `s`.
unsafe fn finder_strlen(s: usize) -> usize {
    let mut n = 0;
    while read_u8(s + n) != 0 {
        n += 1;
    }
    n
}

/// The last cell of a row (column 0xc): staff the reputation band of the result entry (Superb ≥ 0x251c, Very Good ≥
/// 0x1f40, Good ≥ 0x1770, Ok ≥ 0xea6, Unproven under 35, Poor ≥ 0x1f4, Very Poor), players the value (`Cash`) when he
/// has a club and a value, "Free" otherwise; in pick mode a pick button (0x1b) in the pick column.
unsafe fn finder_row_value(b: &FinderBuild, r: &FinderRow, pick_area: u32) {
    let lang = finder_lang();
    let p = r.person;
    if search_edit_session_get_list_type(b.s, u32::MAX) == 3 {
        let rep = read_i16(r.entry + 0x21);
        let text = if rep >= 0x251c {
            0xa150b4usize // "Superb<%s - COMMENT - staff reputation>"
        } else if rep >= 0x1f40 {
            0xa15088 // "Very Good<..>"
        } else if rep >= 0x1770 {
            0xa15060 // "Good<..>"
        } else if rep >= 0xea6 {
            0xa7e004 // "Ok<..>"
        } else if read_i8(p + staff::AGE) < 0x23 {
            0xa7dfd8 // "Unproven<..>"
        } else if rep >= 0x1f4 {
            0xa14fe4 // "Poor<..>"
        } else {
            0xa14fb8 // "Very Poor<..>"
        };
        lang(JOB_TEXT, text, 0xad9c64usize);
    } else if r.club != 0 && read_i32(p + staff::VALUE) != 0 {
        let mut cash = [0u8; 8];
        let new_cash: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FINDER_CASH_NEW);
        let c = new_cash(cash.as_mut_ptr() as usize, read_i32(p + staff::VALUE));
        let fmt: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FINDER_CASH_FORMAT);
        fmt(c, JOB_TEXT, 0x7d0, 5);
    } else {
        lang(JOB_TEXT, 0x98e614usize); // "Free"
    }
    finder_cell(r, 0xc, [0x30, job_colour(JOB_COLOUR_3192), 0xc, 1], job_colour(JOB_COLOUR_3184), JOB_TEXT);
    if b.pick != 0 {
        tel26_area_button([pick_area, 0, r.row as u32, 1, 0, 0xc, 1, job_colour(JOB_COLOUR_3184), 0, 0x1b, p as u32]);
    }
}

/// `SearchFilters::set_defaults` (0x806450, thiscall).
const FINDER_FILTERS_SET_DEFAULTS: usize = 0x806450;

/// The column comparer of the result list for the sort flags (`SearchEditSession` + 0x298 of the current list): the
/// first set flag in the original's order (bit 0 with a filter flag picks the stars or rating comparers); 0 for none.
unsafe fn finder_sort_comparer(sort: u32, filter_flags: u32, list_type: u8) -> usize {
    let asc = sort & 1 != 0;
    if asc && filter_flags & 0x800000 != 0 {
        return FINDER_SORT_RECORD;
    }
    for (bit, f) in [(0x800000u32, 0x818b60usize), (0x1000000, 0x818ce0), (0x2000000, 0x818e70), (0x4000000, 0x819000),
        (0x8000000, 0x819190), (0x10000000, 0x819340)] {
        if sort & bit != 0 {
            return f;
        }
    }
    if asc && filter_flags & 0x1000000 != 0 {
        return 0x819620;
    }
    if asc && filter_flags & 0x2000000 != 0 {
        return 0x8196e0;
    }
    for (bit, f) in [(0x80000u32, 0x819780usize), (2, 0x819870), (4, 0x819910), (8, 0x819990), (0x10, 0x819a80), (0x20, 0x819ba0),
        (0x40, 0x819da0), (0x80, 0x819ea0), (0x100, 0x819f70), (0x200, 0x81a000), (0x400, 0x81a0f0)] {
        if sort & bit != 0 {
            return f;
        }
    }
    if sort & 0x800 != 0 {
        return if list_type == 3 { 0x81a360 } else { 0x81a220 };
    }
    for (bit, f) in [(0x1000u32, 0x81a450usize), (0x2000, 0x81a5a0), (0x4000, 0x81a630), (0x8000, 0x81a6c0), (0x10000, 0x81a750),
        (0x20000, 0x81a860), (0x40000, 0x81a940), (0x100000, 0x81aa50), (0x200000, 0x81ab60), (0x20000000, 0x8189f0),
        (0x400000, 0x81aca0)] {
        if sort & bit != 0 {
            return f;
        }
    }
    0
}

/// Rule: builds the player and staff search screen of the edit session (parameter 0) for the human of the slot.
///
/// Without a human: the error box (finder_screens.cpp line 0x190) and nothing else. The current list's view flags go
/// to [`FINDER_VIEW_FLAGS`] (a list no longer following a club loses the Scout Report view for General); with both
/// title colours (params 1 and 2) empty, the human's club kit (his club, else his national team, else his nation's
/// national team; else the heading colours) is written there. The list's filters are copied, the human goes to
/// [`FINDER_VIEW_MANAGER`] and the results are asked for with the column comparer of the sort flags
/// (`SearchEditSession::get_current_results`: the list, the shown count, the total). A random background; in pick
/// mode (param 5) a full-screen button; the title "Player & Staff Search"; for a human with a club, his transfer funds
/// (the budget when positive) at the top right; the menu bar; then the tab bar and the Search menu
/// (`finder_build_menu`), the View menus, the Filters bar and the page count, the line above the list, the list name
/// and the found line, and with results the header of the view, the list area, the pick column and the rows of the
/// current page (the list walked from its head, or its tail when the sort is descending): the first cell, the status,
/// the name, the view's cells and the value. Last the Cancel (0x1c) and Ok (0x1d) buttons.
/// Original: `search_screens_build_player_search_screen` (0x808690, finder_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn search_screens_build_player_search_screen() {
    let mut text = [0usize; 2];
    exe::new_text(text.as_mut_ptr(), 0usize);
    let mgr = read_usize((G_SCREEN_RECORDS_MANAGER as isize + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize * 0x300) as usize);
    if mgr == 0 {
        tel26_error_box(FINDER_SCREENS_CPP, 0x190);
        exe::drop_text(text.as_mut_ptr());
        return;
    }
    covered!("tel19_1 search screen build");
    let mut b = FinderBuild {
        s: get_screen_param(0),
        mgr,
        fg: get_screen_param(1) as u32,
        bg: get_screen_param(2) as u32,
        pick: get_screen_param(5),
        filters: [0; 0xa0],
        results: 0,
        shown: 0,
        l14: 0,
        top: 0,
        bottom: 0,
        tab_area: 0,
        page_left: 0,
        rows: 0,
        row_height: 0,
        text: text.as_mut_ptr() as usize,
    };
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FINDER_FILTERS_SET_DEFAULTS);
    f(b.filters.as_mut_ptr() as usize);
    let s = b.s;
    let view = finder_current(s) + finder_list::VIEW;
    write_usize(FINDER_VIEW_FLAGS, view);
    if read_u32(view) & 0x2000 != 0 && read_i32(finder_current(s) + finder_list::ASSIGNED_CLUB) < 0 {
        write_u32(view, read_u32(view).wrapping_sub(0x2000));
        write_u32(read_usize(FINDER_VIEW_FLAGS), read_u32(read_usize(FINDER_VIEW_FLAGS)) | 1);
    }
    if b.fg as u16 == 0 && b.bg as u16 == 0 {
        let club = read_usize(mgr + staff::CLUB_JOB);
        let team = if club != 0 {
            club
        } else if read_usize(mgr + staff::NATIONAL_JOB) != 0 {
            read_usize(mgr + staff::NATIONAL_JOB)
        } else if read_usize(mgr + staff::NATION) != 0 {
            exe::database_get_national_team(read_usize(mgr + staff::NATION))
        } else {
            0
        };
        if club != 0 || read_usize(mgr + staff::NATIONAL_JOB) != 0 || read_usize(mgr + staff::NATION) != 0 {
            exe::database_get_club_kit_colours(team, 1, &mut b.fg, &mut b.bg);
        } else {
            b.fg = exe::get_heading_colour(1) as u32;
            b.bg = exe::get_heading_colour(0) as u32;
        }
        exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 1, (b.fg & 0xffff) as usize, 0);
        exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 2, (b.bg & 0xffff) as usize, 0);
    }
    let cur = finder_current(s);
    core::ptr::copy_nonoverlapping((cur + finder_list::FILTERS) as *const u8, b.filters.as_mut_ptr(), finder_list::FILTERS_SIZE);
    write_usize(FINDER_VIEW_MANAGER, mgr);
    let sort = read_u32(read_usize(FINDER_VIEW_FLAGS) + 4);
    let comparer = finder_sort_comparer(sort, read_u32(b.filters.as_ptr() as usize + 4), read_u8(cur + finder_list::TYPE));
    let results: unsafe extern "thiscall" fn(usize, *mut usize, *mut u32, usize) -> u32 = core::mem::transmute(FINDER_SESSION_GET_CURRENT_RESULTS);
    b.l14 = results(s, &mut b.results, &mut b.shown, comparer) as usize;
    exe::background_set_random_picture();
    if b.pick != 0 {
        tel26_add_button([0, 0, exe_immediate(0x808e7c as *const i32) as u32, exe_immediate(0x808e77 as *const i32) as u32, 1, 0, 0xc, 1,
            job_colour(JOB_COLOUR_3184), 0, 1, 0]);
    }
    let lang = finder_lang();
    lang(JOB_TEXT, 0xa5a604usize); // "Player & Staff Search"
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, b.bg, b.fg, JOB_TEXT);
    let club = read_usize(mgr + staff::CLUB_JOB);
    if club != 0 {
        let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
        if fin != 0 {
            covered!("tel19_1 search screen funds");
            let budget = exe::finance_get_transfer_budget(fin, 0, 0, 0usize, 1, 0);
            let cash: u32 = if budget > 0 { budget as u32 } else { 0 };
            let mut money = [0u8; 0xc8];
            let amount = exe::cash_get_display_amount(&cash, 5);
            exe::cash_format_money(money.as_mut_ptr() as usize, 0xc8, amount, 5);
            lang(JOB_TEXT, 0xa7e82cusize, money.as_ptr()); // "Funds - <%s - cash>"
            let w = exe::font_text_width(1usize, JOB_TEXT as *const u8);
            tel26_add_button([(0x307 - w) as u32, 0xf, 0x311, 0x23, 0x30, b.bg, 0xc, 1, b.fg, JOB_TEXT as u32, 2, 0]);
        }
    }
    exe::menubar_build(4, 0);
    finder_build_menu(&mut b);
    finder_build_view_menu(&mut b);
    let bar = finder_build_filters_bar(&mut b);
    finder_build_heading(&mut b);
    finder_build_title(&mut b);
    if read_u16(b.results) != 0 {
        finder_build_rows(&mut b, bar);
    }
    let area = {
        let g: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(FINDER_SM_ADD_AREA);
        let w: [u8; 2] = [3, 1];
        g(G_SCREEN_MANAGER, 0x64, 0x22b, 0x316, 0x24e, 2, w.as_ptr() as usize, 1, 0, 1, 0, u32::MAX)
    };
    lang(JOB_TEXT, 0x98a1a0usize); // "Cancel"
    tel26_area_button([area, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0x1c, 0]);
    lang(JOB_TEXT, 0x990f7cusize); // "Ok"
    tel26_area_button([area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0x1d, 0]);
    exe::drop_text(text.as_mut_ptr());
}

/// The header of the view, the list area, the pick column and the rows of the current page (see
/// `search_screens_build_player_search_screen`).
unsafe fn finder_build_rows(b: &mut FinderBuild, bar: u32) {
    let list_area = finder_build_header(b, bar);
    let pick_area = finder_area([0x6e, (b.row_height + b.top + 8) as u32, 0x30c, (b.bottom - 8) as u32, 1, 0, (b.rows as u32).wrapping_sub(1) & 0xff, 0, 1, 0, u32::MAX]);
    let list = b.results;
    let descending = read_u32(read_usize(FINDER_VIEW_FLAGS) + 0xa) != 0;
    write_u32(list + 0xa, read_u32(list + if descending { 6 } else { 2 }));
    let per = b.rows as i32 - 1;
    let mut r = FinderRow { area: list_area, row: 0, person: 0, entry: 0, club: b.page_left as u32 as usize, slot: 0, b_team: 0 };
    let mut index = 0i32;
    loop {
        let entry = get_current(list);
        if entry == 0 {
            break;
        }
        r.slot = 0;
        if read_u32(read_usize(FINDER_VIEW_FLAGS) + 0xa) == 0 {
            next_list_item(list);
        } else {
            exe::llist_get_prev(list);
        }
        let page = index / per + 1;
        index += 1;
        if page != read_i16(read_usize(FINDER_VIEW_FLAGS) + 0xe) as i32 {
            continue;
        }
        r.entry = entry;
        r.person = staff_at(read_i32(entry + 0xc) as isize).addr();
        finder_row_first(b, &mut r);
        finder_row_status(&r);
        finder_row_name(b, &mut r);
        finder_row_view(b, &mut r);
        finder_row_value(b, &r, pick_area);
        r.row += 1;
    }
}

/// Comparer for `search_screens_build_player_search_screen`.
///
/// Side by side over the built display list (`compare_match_screen_build`: the screen's parameter block, so the title
/// colours and the page count) plus what the screen writes to the game: the two view globals, the current list's view
/// flags, sort and page, the attribute and statistics column lists (the "Filtered" one is rebuilt and `qsort` sorts
/// them in place) and the contract lookup. When the results are not cached yet
/// (the session's search would run in the original's round and not in the copy's), only the original runs.
pub(crate) unsafe extern "C" fn compare_search_screens_build_player_search_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I4%%);
    let s = get_screen_param(0);
    if s == 0 || read_u32(s + finder_session::CACHED) == 0 {
        // simplification: the first build runs the search (and may draw); proven in Rust mode
        return o();
    }
    let view = finder_current(s) + finder_list::VIEW;
    let extra = [(FINDER_VIEW_FLAGS, 8), (view, 0x10), (FINDER_COLUMNS_STATS, 0x6a), (G_CONTRACT_LOOKUP_RESULT, 4)];
    compare_match_screen_build("player search screen", &extra, || o(), || search_screens_build_player_search_screen());
}
