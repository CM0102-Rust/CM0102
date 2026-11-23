use super::*;

/// The constructor of the player search's filter record (0x11 bytes at PLAYER_SEARCH +0x293): dword +0 = 0x81,
/// dword +4 = 4, bytes +8 and +9 = 0xff, dword +0xa = 0, word +0xe = 1, byte +0x10 = 1 [inference: the default
/// filters]; returns `this`.
/// Original: `search_filter_record_init` (0x81b800, search_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn search_filter_record_init(this: usize) -> usize {
    covered!("tel39_2 search filter record");
    write_u8(this + 8, 0xff);
    write_u8(this + 9, 0xff);
    write_u32(this + 4, 4);
    // tel39_2: the default filter word
    write_u32(this, 0x81);
    write_u32(this + 0xa, 0);
    write_u16(this + 0xe, 1);
    write_u8(this + 0x10, 1);
    this
}

/// Comparer for `search_filter_record_init`: side by side over the 0x11 bytes and the return.
pub(crate) unsafe extern "thiscall" fn compare_search_filter_record_init(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_SEARCH_FILTER_RECORD_INIT);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, 0x11)], || o(this), || search_filter_record_init(this));
    SCOREBOARD.record(same && r == n, || format!("search_filter_record_init({:#x}) original {:x?} rust {:x?}", this, mo, mr));
    r
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
    original_fn::<unsafe extern "C" fn(u32)>(IDX_SEARCH_SCREENS_OPEN_PLAYER_SEARCH_SCREEN)(list);
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

/// Rule: the type of a list of the edit session: list `index` (its low byte, signed), or the current one for -1.
///
/// Original: `SearchEditSession::get_list_type` (0x80eaa0, finder_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn search_edit_session_get_list_type(this: usize, index: u32) -> u8 {
    let k = if index as u8 == 0xff { read_i8(this) } else { index as u8 as i8 };
    read_u8(finder_list_at(this, k) + finder_list::TYPE)
}

/// Rule: the club a list of the edit session follows (-1 none): list `index` (low byte, signed), or the current one
/// for -1.
///
/// Original: `SearchEditSession::get_assigned_club` (0x80ead0, finder_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn search_edit_session_get_assigned_club(this: usize, index: u32) -> i32 {
    let k = if index as u8 == 0xff { read_i8(this) } else { index as u8 as i8 };
    read_i32(finder_list_at(this, k) + finder_list::ASSIGNED_CLUB)
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

/// Comparer for `search_screens_build_player_search_screen`.
///
/// Side by side over the built display list (`compare_match_screen_build`: the screen's parameter block, so the title
/// colours and the page count) plus what the screen writes to the game: the two view globals, the current list's view
/// flags, sort and page, the attribute and statistics column lists (the "Filtered" one is rebuilt and `qsort` sorts
/// them in place) and the contract lookup. When the results are not cached yet
/// (the session's search would run in the original's round and not in the copy's), only the original runs.
pub(crate) unsafe extern "C" fn compare_search_screens_build_player_search_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_SEARCH_SCREENS_BUILD_PLAYER_SEARCH_SCREEN);
    let s = get_screen_param(0);
    if s == 0 || read_u32(s + finder_session::CACHED) == 0 {
        // simplification: the first build runs the search (and may draw); proven in Rust mode
        return o();
    }
    let view = finder_current(s) + finder_list::VIEW;
    let extra = [(FINDER_VIEW_FLAGS, 8), (view, 0x10), (FINDER_COLUMNS_STATS, 0x6a), (G_CONTRACT_LOOKUP_RESULT, 4)];
    compare_match_screen_build("player search screen", &extra, || o(), || search_screens_build_player_search_screen());
}

/// Rule: answers the player and staff search screen.
///
/// First, when a box or a file dialog has answered ([`FINDER_PENDING`]), its pending command (`finder_pending`); the
/// list's name is copied for the boxes (also for any click). Then the clicked item's id (its id, or the current
/// command id): in pick mode (param 5) the pick button (0x1b) asks to delete the person from the list ("cannot be
/// deleted" when he is not in it; param 6 = the person, pending 0x26) and any other click leaves pick mode (-11);
/// otherwise 0x21 the person's note, 2 the human's squad, 3 another tab (-11), the scout's assignments
/// (`finder_assign`), 7 the recall (`finder_recall`), the list commands (`finder_list_command`), the view and sort
/// commands (`finder_view_command`), 0x13 the filters screen, 0x16 pick mode on/off (-11), 0x1b the person's profile,
/// 0x1c cancel (the box when the session has changes, pending 0x25; else the screen closes, -9), 0x1d Ok (every list
/// saved, the screen closes, -9); anything else goes to the menu bar (`menubar_handle_command`, then
/// `menubar_handle_object_link`). Returns -4 when a screen or a box opened, -9 when the screen closes, -11 to redraw.
/// Original: `search_screens_handle_player_search_input` (0x80eb00, search_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn search_screens_handle_player_search_input(item: u32) -> i32 {
    let mut st = FinderInput {
        s: 0,
        mgr: read_usize((G_SCREEN_RECORDS_MANAGER as isize + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize * 0x300) as usize),
        c1: 0,
        c2: 0,
        lock_owner: 0,
        pick: 0,
        pending_person: 0,
        pages: 0,
        item: item as i16,
        name: [0; 0xc8],
        header: [0; 0x170],
        file: [0; 0x104],
        title: [0; 0xc8],
        heading: [0; 0xc8],
        scout: [0; 0xc8],
        place: [0; 0xc8],
        path: [0; 0x104],
    };
    st.s = get_screen_param(0);
    st.c1 = get_screen_param(1) as u32;
    st.c2 = get_screen_param(2) as u32;
    st.lock_owner = get_screen_param(3);
    st.pick = get_screen_param(5);
    st.pending_person = get_screen_param(6);
    st.pages = get_screen_param(7) as i16;
    let s = st.s;
    write_usize(FINDER_VIEW_FLAGS, finder_current(s) + finder_list::VIEW);
    if read_u32(FINDER_PENDING) != 0 || st.item != -1 {
        exe::at_9466d0(st.name.as_mut_ptr() as usize, (finder_current(s) + finder_list::NAME) as *const u8, 0xc8);
        st.name[0xc7] = 0;
    }
    if read_u32(FINDER_PENDING) != 0 {
        if let Some(r) = finder_pending(&mut st) {
            return r;
        }
    }
    let lang = finder_lang();
    if st.pick != 0 {
        if st.id() == 0x1b {
            covered!("tel19_2 search pick delete");
            exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 5, 0usize, 0);
            let p = st.value();
            exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 6, p, 0);
            let k = st.kind();
            if exe::player_search_contains_player(st.search(), p) != 0 {
                st.scout_name(p, 1);
                lang(JOB_TEXT, finder_text::DELETE_PLAYER[k], st.sp());
                return st.confirm(finder_text::PLAYER_SEARCH, finder_text::PLEASE_CONFIRM, 1, 0x26);
            }
            st.scout_name(p, 1);
            lang(JOB_TEXT, finder_text::CANNOT_DELETE[k], st.sp());
            return st.confirm(finder_text::PLAYER_SEARCH, finder_text::UNABLE_DELETE, 0, 0);
        }
        if st.item != -1 {
            exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 5, 0usize, 0);
            return FINDER_REDRAW;
        }
    }
    let id = st.id();
    match id {
        0x21 => {
            let f: unsafe extern "C" fn(usize) = core::mem::transmute(FINDER_OPEN_NOTE);
            f(st.value());
            return FINDER_OPENED;
        }
        2 => {
            let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) = core::mem::transmute(FINDER_OPEN_SQUAD);
            f(read_usize(st.mgr + staff::CLUB_JOB), 0, 0, 1, 0);
            return FINDER_OPENED;
        }
        3 => {
            let v = st.value();
            if v as u8 != read_u8(s) {
                finder_call1(FINDER_SET_CURRENT, s, v as u32);
                return FINDER_REDRAW;
            }
        }
        _ => {}
    }
    if let Some(r) = finder_assign(&mut st, id) {
        return r;
    }
    if id == 7 {
        if let Some(r) = finder_recall(&mut st) {
            return r;
        }
    }
    let id = st.id();
    if let Some(r) = finder_list_command(&mut st, id) {
        return r;
    }
    if let Some(r) = finder_view_command(&mut st, id) {
        return r;
    }
    match id {
        0x13 => {
            let f: unsafe extern "C" fn(usize) = core::mem::transmute(FINDER_OPEN_FILTERS);
            f(s);
            FINDER_OPENED
        }
        0x16 => {
            exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 5, (st.pick == 0) as usize, 0);
            FINDER_REDRAW
        }
        0x1b => {
            let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) = core::mem::transmute(FINDER_OPEN_PROFILE);
            f(st.value(), 1, 1, 0, 0);
            FINDER_OPENED
        }
        0x1c => {
            if read_u32(s + 0x1b09) != 0 {
                lang(JOB_TEXT, 0xa7e874usize); // "Changes have been made to the player search. Cancel all changes ?"
                return st.confirm(finder_text::PLAYER_SEARCH, finder_text::CHANGES_MADE, 1, 0x25);
            }
            finder_close(&st)
        }
        0x1d => {
            covered!("tel19_2 search ok");
            finder_call0(FINDER_SAVE_ALL, s);
            finder_close(&st)
        }
        _ => {
            let r = exe::menubar_handle_command(item);
            if r != 0 {
                return r;
            }
            exe::menubar_handle_object_link(item)
        }
    }
}

/// Comparer for `search_screens_handle_player_search_input`.
///
/// The answers open screens, boxes and file dialogs, load and save files and free the session: only the original runs,
/// except for the view, sort, page and pick-mode commands without a pending answer, which only change the current
/// list's view flags, the "results cached" flag and parameter 5; those run side by side (the list's view fields, the
/// cache flag, the view global and the screen's parameters) and the return codes must match.
pub(crate) unsafe extern "C" fn compare_search_screens_handle_player_search_input(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_SEARCH_SCREENS_HANDLE_PLAYER_SEARCH_INPUT);
    let s = get_screen_param(0);
    let node = current_screen_node(G_SCREEN_MANAGER);
    let k = item as i16;
    let id = {
        let d = if k != -1 { read_u16(display_item_at(finder_display(), k) + display_item::ID) } else { 0 };
        if d != 0 { d } else { read_u16(G_CURRENT_COMMAND_ID) }
    };
    let simple = matches!(id, 0x12 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1e);
    if s == 0 || node == 0 || read_u32(FINDER_PENDING) != 0 || get_screen_param(5) != 0 || !simple {
        // simplification: the other answers cannot run twice (screens, boxes, files); proven in Rust mode
        return o(item);
    }
    let view = finder_current(s) + finder_list::VIEW;
    let regions = [(view, 0x10), (s + finder_session::CACHED, 4), (FINDER_VIEW_FLAGS, 4),
        (node + screen_node_fields::PARAMS, screen_node_fields::PARAM_COUNT * 8)];
    let (ro, rr, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(item), || search_screens_handle_player_search_input(item));
    SCOREBOARD.record(same && ro == rr, || format!("player search screen answer (id {:#x}): returns {} / {}, differences in {:?}", id, ro, rr,
        first_differences(&mo, &mr)));
    ro
}

/// Orders two search results by the manager's note rating, highest first, unrated (-1) last, then by name.
///
/// A result rated -1 goes after one that is rated (1 / -1); otherwise the byte difference `rating(b) - rating(a)`
/// (8 bits, sign-extended) and, when equal, `search_results_compare_names`.
/// Original: search_screens_compare_results_by_note_rating (0x8189f0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_note_rating(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by note rating");
    let (a, b) = (a as usize, b as usize);
    let ra = read_u8(a + search_result_keys::NOTE_RATING);
    let rb = read_u8(b + search_result_keys::NOTE_RATING);
    if ra == 0xff && rb != 0xff {
        return 1;
    }
    if ra != 0xff && rb == 0xff {
        return -1;
    }
    let d = rb.wrapping_sub(ra) as i8;
    if d != 0 {
        return d as i32;
    }
    search_results_compare_names(a, b)
}

/// Orders two search results by the higher of their ability and potential stars, best first, then by name.
///
/// Each side's key is max(ability stars, potential stars) (signed bytes); the byte difference `key(b) - key(a)`
/// (8 bits, sign-extended), then `search_results_compare_names`.
/// Original: search_screens_compare_results_by_best_stars (0x818ab0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_best_stars(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by best stars");
    let (a, b) = (a as usize, b as usize);
    let best = |e: usize| read_i8(e + search_result_keys::ABILITY_STARS).max(read_i8(e + search_result_keys::POTENTIAL_STARS));
    let d = (best(b) as u8).wrapping_sub(best(a) as u8) as i8;
    if d != 0 {
        return d as i32;
    }
    search_results_compare_names(a, b)
}

/// Orders two search results by the matches the scout watched (the slot's history count, +0x1a1), most first.
///
/// A person no scout follows (`find_club_search_entry` 0) goes last; both unfollowed: by name. Otherwise the byte
/// difference of the two history counts `count(b) - count(a)` (8 bits, sign-extended), then the name.
/// Original: search_screens_compare_results_by_matches_watched (0x818b60, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_matches_watched(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by matches watched");
    search_results_compare_scouted(a as usize, b as usize, |sa, sb| {
        read_u8(sb + scouted_player_slot::HISTORY_COUNT).wrapping_sub(read_u8(sa + scouted_player_slot::HISTORY_COUNT)) as i8 as i32
    })
}

/// Orders two search results by the goals in the matches the scout watched (`player_search_slot_sum_goals`), most
/// first: an unfollowed person last, the 16-bit difference `goals(b) - goals(a)`, then the name.
/// Original: search_screens_compare_results_by_scouted_goals (0x818ce0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_scouted_goals(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by scouted goals");
    search_results_compare_scouted_sum(a as usize, b as usize, SEARCH_FN_SLOT_SUM_GOALS)
}

/// Orders two search results by the assists in the matches the scout watched (`player_search_slot_sum_assists`), as
/// the goals.
/// Original: search_screens_compare_results_by_scouted_assists (0x818e70, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_scouted_assists(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by scouted assists");
    search_results_compare_scouted_sum(a as usize, b as usize, SEARCH_FN_SLOT_SUM_ASSISTS)
}

/// Orders two search results by the goals conceded in the matches the scout watched
/// (`player_search_slot_sum_goals_conceded`), most first like the goals.
/// Original: search_screens_compare_results_by_scouted_goals_conceded (0x819000, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_scouted_goals_conceded(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by scouted goals conceded");
    search_results_compare_scouted_sum(a as usize, b as usize, SEARCH_FN_SLOT_SUM_GOALS_CONCEDED)
}

/// Orders two search results by the average rating in the matches the scout watched, highest first.
///
/// An unfollowed person last (both: by name); otherwise `average(b) - average(a)` (`player_search_slot_average_rating`,
/// doubles): above 0 gives 1, equal goes to the name, below 0 or not a number gives -1.
/// Original: search_screens_compare_results_by_scouted_average_rating (0x819190, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_scouted_average_rating(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by scouted average rating");
    let average: unsafe extern "thiscall" fn(usize) -> f64 = core::mem::transmute(SEARCH_FN_SLOT_AVERAGE_RATING);
    search_results_compare_scouted(a as usize, b as usize, |sa, sb| {
        let d = average(sb) - average(sa);
        if d > 0.0 {
            1
        } else if d == 0.0 {
            0 // equal: the name
        } else {
            -1 // below 0 or not a number (the x87 compare sets C0 for both)
        }
    })
}

/// Orders two search results by the date of the last match the scout watched, latest first.
///
/// An unfollowed person last (both: by name). Each slot's last match is entry `count - 1` of its history
/// (`player_search_slot_get_match_entry`), none with an empty history. `a` without one: `b` with one gives 1, neither
/// goes to the name. `a` with one: both dates (`Date::Date(day, year)` from the entry's +0 and +2) compared, the year
/// first (unsigned) then the day (signed): `b` later 1, earlier -1, equal by name. DEFECT of the original kept: `a`
/// with a last match and `b` with an empty history reads the date at address 0 (the original crashes there too).
/// Original: search_screens_compare_results_by_last_scouted_match (0x819340, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_last_scouted_match(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by last scouted match");
    let entry: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(SEARCH_FN_SLOT_GET_MATCH_ENTRY);
    let last = |s: usize| {
        let n = read_i8(s + scouted_player_slot::HISTORY_COUNT);
        if n > 0 { entry(s, (n - 1) as u8 as u32) } else { 0 }
    };
    search_results_compare_scouted(a as usize, b as usize, |sa, sb| {
        let ea = last(sa);
        let eb = last(sb);
        if ea == 0 {
            return (eb != 0) as i32; // neither: the name
        }
        let (mut da, mut db) = ([0u8; 8], [0u8; 8]);
        exe::new_date(da.as_mut_ptr() as usize, read_u16(ea) as u32, read_u16(ea + 2) as u32);
        exe::new_date(db.as_mut_ptr() as usize, read_u16(eb) as u32, read_u16(eb + 2) as u32);
        let year = |d: &[u8; 8]| u16::from_le_bytes([d[2], d[3]]);
        let day = |d: &[u8; 8]| i16::from_le_bytes([d[0], d[1]]);
        if year(&db) > year(&da) {
            1
        } else if year(&db) < year(&da) {
            -1
        } else if day(&db) > day(&da) {
            1
        } else if day(&db) < day(&da) {
            -1
        } else {
            0
        }
    })
}

/// Orders two search results by their ability stars, best first: a null result goes last (`a` null 1, `b` null -1),
/// then the byte difference `stars(b) - stars(a)` and the name.
/// Original: search_screens_compare_results_by_ability_stars (0x819620, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_ability_stars(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by ability stars");
    let (a, b) = (a as usize, b as usize);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let d = read_u8(b + search_result_keys::ABILITY_STARS).wrapping_sub(read_u8(a + search_result_keys::ABILITY_STARS)) as i8;
    if d != 0 {
        return d as i32;
    }
    search_results_compare_names(a, b)
}

/// Orders two search results by their potential stars, best first: the byte difference `stars(b) - stars(a)`, then
/// the name.
/// Original: search_screens_compare_results_by_potential_stars (0x8196e0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_potential_stars(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by potential stars");
    let (a, b) = (a as usize, b as usize);
    let d = read_u8(b + search_result_keys::POTENTIAL_STARS).wrapping_sub(read_u8(a + search_result_keys::POTENTIAL_STARS)) as i8;
    if d != 0 {
        return d as i32;
    }
    search_results_compare_names(a, b)
}

/// Orders two search results by the slot in the national squad, lowest first, players outside it (-1, counted as
/// 0x7f) last: the byte difference `slot(a) - slot(b)` (`national_teams_get_player_squad_slot`), then the name.
/// Original: search_screens_compare_results_by_national_squad_slot (0x819780, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_national_squad_slot(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by national squad slot");
    let (a, b) = (a as usize, b as usize);
    let slot_of: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(SEARCH_FN_NATIONAL_SQUAD_SLOT);
    let slot = |e: usize| {
        let s = slot_of(read_usize(G_NATIONAL_TEAMS), search_result_person(e).addr());
        if s == 0xff { 0x7f } else { s }
    };
    let sa = slot(a);
    let d = sa.wrapping_sub(slot(b)) as i8;
    if d != 0 {
        return d as i32;
    }
    search_results_compare_names(a, b)
}

/// Orders two search results by the player status code, ascending: the byte difference `code(a) - code(b)`, then the
/// name.
/// Original: search_screens_compare_results_by_status_code (0x819870, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_status_code(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by status code");
    let (a, b) = (a as usize, b as usize);
    let d = read_u8(a + search_result_keys::STATUS_CODE).wrapping_sub(read_u8(b + search_result_keys::STATUS_CODE)) as i8;
    if d != 0 {
        return d as i32;
    }
    search_results_compare_names(a, b)
}

/// Orders two search results by name only (`search_results_compare_names`).
/// Original: search_screens_compare_results_by_name (0x819910, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_name(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by name");
    search_results_compare_names(a as usize, b as usize)
}

/// Sorts two search entries by the short name of the person's club, a person without a club last (`a` 1, `b` -1),
/// through `exe::utils_string_compare(name a, name b, 1, 0)`, then by name.
/// Original: search_screens_compare_staff_by_club (0x819990, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_club(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by club");
    let (a, b) = (a as usize, b as usize);
    let ca = search_result_person(a).club_job();
    let cb = search_result_person(b).club_job();
    if ca.is_null() {
        return 1;
    }
    if cb.is_null() {
        return -1;
    }
    let r = exe::utils_string_compare((ca.addr() + club::SHORT_NAME) as *const u8, (cb.addr() + club::SHORT_NAME) as *const u8, 1, 0);
    if r != 0 {
        return r;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by the short name of the employer: the club, or without one the national side when the
/// national job is not 0xb; no employer last (`a` 1, `b` -1); then by name.
/// Original: search_screens_compare_staff_by_employer (0x819a80, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_employer(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by employer");
    let (a, b) = (a as usize, b as usize);
    let employer = |p: Staff| {
        let c = p.club_job();
        if c.is_null() && !p.national_job().is_null() && p.job_for_nation() != SEARCH_NATIONAL_JOB_NONE {
            p.national_job()
        } else {
            c
        }
    };
    let ea = employer(search_result_person(a));
    let eb = employer(search_result_person(b));
    if ea.is_null() {
        return 1;
    }
    if eb.is_null() {
        return -1;
    }
    let r = exe::utils_string_compare((ea.addr() + club::SHORT_NAME) as *const u8, (eb.addr() + club::SHORT_NAME) as *const u8, 1, 0);
    if r != 0 {
        return r;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by league: the division of a club in the nation of the current human's club by its
/// reputation, everything else by nation name.
///
/// For each side (`a` first): a person with a club whose division's nation is the nation of the club of the current
/// slot's human manager (`tel26_slot_manager`) keys on that division; otherwise on the club's nation name, or without
/// a club nation on the person's own nation name; a person without any of them stops the sort (`a` 1, `b` -1). Two
/// names: `exe::utils_string_compare(name a, name b, 1, 0)`; a name against a division: the name side goes last (`a` 1,
/// `b` -1); two divisions: `reputation(b) - reputation(a)` (16-bit values, 32-bit difference). Equal: by name.
/// Original: search_screens_compare_staff_by_league (0x819ba0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_league(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by league");
    let (a, b) = (a as usize, b as usize);
    let manager = tel26_slot_manager();
    // Some((division, 0)) or Some((0, nation name)); None: no nation at all
    let key = |p: Staff| -> Option<(usize, usize)> {
        let club = p.club_job();
        if !club.is_null() {
            let division = club.division();
            if !division.is_null() && !manager.club_job().is_null() && division.nation() == manager.club_job().nation() {
                return Some((division.addr(), 0));
            }
            let n = club.nation();
            if !n.is_null() {
                return Some((0, n.addr() + nation::SHORT_NAME));
            }
        }
        let n = p.nation();
        if n.is_null() { None } else { Some((0, n.addr() + nation::SHORT_NAME)) }
    };
    let Some((da, na)) = key(search_result_person(a)) else { return 1 };
    let Some((db, nb)) = key(search_result_person(b)) else { return -1 };
    let r = if da == 0 {
        if db != 0 {
            return 1;
        }
        exe::utils_string_compare(na as *const u8, nb as *const u8, 1, 0)
    } else {
        if db == 0 {
            return -1;
        }
        (Competition(db).reputation() as i32).wrapping_sub(Competition(da).reputation() as i32)
    };
    if r != 0 {
        return r;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by the short positions text of the player record (`database_format_player_positions`
/// style 4 into 0xc8 bytes) through `exe::utils_string_compare(text a, text b, 1, 0)`, then by name.
/// Original: search_screens_compare_staff_by_position (0x819da0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_position(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by position");
    let (a, b) = (a as usize, b as usize);
    let positions: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(SEARCH_FN_FORMAT_PLAYER_POSITIONS);
    let (mut ta, mut tb) = ([0u8; 0xc8], [0u8; 0xc8]);
    positions(search_result_person(a).player().addr(), ta.as_mut_ptr() as usize, 0xc8, 4);
    positions(search_result_person(b).player().addr(), tb.as_mut_ptr() as usize, 0xc8, 4);
    let r = exe::utils_string_compare(ta.as_ptr(), tb.as_ptr(), 1, 0);
    if r != 0 {
        return r;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by job code, ascending: the club job (`job_for_club`) with a club, otherwise the national
/// job when there is a national side and it is not 0xb; none, or a code 0, counts as 0x64 (last). The byte difference
/// `job(a) - job(b)`, then the name.
/// Original: search_screens_compare_staff_by_job (0x819ea0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_job(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by job");
    let (a, b) = (a as usize, b as usize);
    let job = |p: Staff| {
        let j = if !p.club_job().is_null() {
            p.job_for_club()
        } else if !p.national_job().is_null() && p.job_for_nation() != SEARCH_NATIONAL_JOB_NONE {
            p.job_for_nation()
        } else {
            SEARCH_NO_JOB
        };
        if j == 0 { SEARCH_NO_JOB } else { j }
    };
    let ja = job(search_result_person(a));
    let d = ja.wrapping_sub(job(search_result_person(b))) as i8;
    if d != 0 {
        return d as i32;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by estimated value, highest first: `value(b) - value(a)` (32 bits), then the name.
/// Original: search_screens_compare_staff_by_value (0x819f70, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_value(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by value");
    let (a, b) = (a as usize, b as usize);
    let d = (search_result_person(b).value() as i32).wrapping_sub(search_result_person(a).value() as i32);
    if d != 0 {
        return d;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by the stored reputation (`search_result_keys::REPUTATION`), highest first.
///
/// Below 3750 a person under 35 (the age byte, signed) is ranked at 3750; exactly 3750 counts as 3751. The 16-bit
/// difference `key(b) - key(a)` (sign-extended), then the name.
/// Original: search_screens_compare_staff_by_reputation (0x81a000, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_reputation(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by reputation");
    let (a, b) = (a as usize, b as usize);
    let key = |e: usize| {
        let r = read_i16(e + search_result_keys::REPUTATION);
        if r < SEARCH_REPUTATION_ESTABLISHED {
            if read_i8(search_result_person(e).addr() + staff::AGE) < 0x23 { SEARCH_REPUTATION_ESTABLISHED } else { r }
        } else if r == SEARCH_REPUTATION_ESTABLISHED {
            r + 1
        } else {
            r
        }
    };
    let ka = key(a);
    let d = key(b).wrapping_sub(ka);
    if d != 0 {
        return d as i32;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by the selected player statistic (`exe::playerstats_get_stat(player, 6, settings + 8,
/// 6.5)`), highest first: `a`'s value kept as a float, `b`'s above it 1, equal by name, below (or not a number) -1.
/// Original: search_screens_compare_staff_by_stat (0x81a0f0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_stat(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by stat");
    let (a, b) = (a as usize, b as usize);
    let stat = |e: usize| {
        let column = read_u8(read_usize(SEARCH_SETTINGS) + SEARCH_SETTINGS_STAT) as u32;
        exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), search_result_person(e).player().addr(), 6, column, 6.5)
    };
    let sa = stat(a) as f32;
    let sb = stat(b);
    if sb > sa as f64 {
        return 1;
    }
    if sb != sa as f64 {
        return -1; // below, or not a number (the x87 compare sets C0 for both)
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by the selected player attribute (settings + 9) where the viewer can see it
/// (`exe::staff_is_player_attribute_visible`; hidden counts as -1), highest first: the byte difference
/// `value(b) - value(a)`, then the name.
/// Original: search_screens_compare_staff_by_player_attribute (0x81a220, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_player_attribute(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by player attribute");
    let (a, b) = (a as usize, b as usize);
    let value = |e: usize| {
        let p = search_result_person(e);
        let attribute = read_u8(read_usize(SEARCH_SETTINGS) + SEARCH_SETTINGS_ATTRIBUTE) as u32;
        if exe::staff_is_player_attribute_visible(read_usize(SEARCH_VIEWER), p, attribute) != 0 {
            exe::staff_get_player_attribute(p, attribute)
        } else {
            -1
        }
    };
    let va = value(a);
    let d = (value(b) as u8).wrapping_sub(va as u8) as i8;
    if d != 0 {
        return d as i32;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by the selected non-player attribute (settings + 9, `exe::staff_get_non_player_attribute`,
/// `b` read first), highest first: the byte difference `value(b) - value(a)`, then the name.
/// Original: search_screens_compare_staff_by_non_player_attribute (0x81a360, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_non_player_attribute(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by non-player attribute");
    let (a, b) = (a as usize, b as usize);
    let value = |e: usize| {
        let attribute = read_u8(read_usize(SEARCH_SETTINGS) + SEARCH_SETTINGS_ATTRIBUTE) as u32;
        exe::staff_get_non_player_attribute(search_result_person(e).addr(), attribute)
    };
    let vb = value(b);
    let d = (vb as u8).wrapping_sub(value(a) as u8) as i8;
    if d != 0 {
        return d as i32;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by the three-letter code of the person's nation, no nation last (`a` 1, `b` -1), through
/// `exe::utils_string_compare(code a, code b, 1, 0)` (the original compares copies), then by name.
/// Original: search_screens_compare_staff_by_nationality (0x81a450, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_nationality(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by nationality");
    let (a, b) = (a as usize, b as usize);
    let na = search_result_person(a).nation();
    let nb = search_result_person(b).nation();
    if na.is_null() {
        return 1;
    }
    if nb.is_null() {
        return -1;
    }
    let r = exe::utils_string_compare((na.addr() + nation::THREE_LETTER_NAME) as *const u8, (nb.addr() + nation::THREE_LETTER_NAME) as *const u8, 1, 0);
    if r != 0 {
        return r;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by age (the signed age byte), oldest first: `age(b) - age(a)`, then the name.
/// Original: search_screens_compare_staff_by_age (0x81a5a0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_age(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by age");
    let (a, b) = (a as usize, b as usize);
    let age = |e: usize| read_i8(search_result_person(e).addr() + staff::AGE) as i32;
    let d = age(b) - age(a);
    if d != 0 {
        return d;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by international caps, most first: `caps(b) - caps(a)` (unsigned bytes), then the name.
/// Original: search_screens_compare_staff_by_caps (0x81a630, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_caps(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort by caps");
    let (a, b) = (a as usize, b as usize);
    let d = search_result_person(b).int_apps() as i32 - search_result_person(a).int_apps() as i32;
    if d != 0 {
        return d;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by international goals, most first: `goals(b) - goals(a)` (unsigned bytes), then the name.
/// Original: search_screens_compare_staff_by_international_goals (0x81a6c0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_international_goals(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort by international goals");
    let (a, b) = (a as usize, b as usize);
    let d = search_result_person(b).int_goals() as i32 - search_result_person(a).int_goals() as i32;
    if d != 0 {
        return d;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by the recent form of the player record (`exe::playerstats_get_recent_form`, `b` read
/// first), best first; a person without a player record last (`a` 1, `b` -1): `form(b) - form(a)`, then the name.
/// Original: search_screens_compare_staff_by_form (0x81a750, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_form(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort by form");
    let (a, b) = (a as usize, b as usize);
    let pa = search_result_person(a).player();
    let pb = search_result_person(b).player();
    if pa.is_null() {
        return 1;
    }
    if pb.is_null() {
        return -1;
    }
    let fb = exe::playerstats_get_recent_form(read_usize(G_PLAYER_STATS), pb) as i32;
    let d = fb - exe::playerstats_get_recent_form(read_usize(G_PLAYER_STATS), pa) as i32;
    if d != 0 {
        return d;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by player morale (`contract_manager_get_player_morale`, `b` read first), highest first:
/// `morale(b) - morale(a)`, then the name.
/// Original: search_screens_compare_staff_by_morale (0x81a860, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_morale(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort by morale");
    let (a, b) = (a as usize, b as usize);
    let morale: unsafe extern "thiscall" fn(usize, usize) -> i8 = core::mem::transmute(SEARCH_FN_PLAYER_MORALE);
    let mb = morale(read_usize(G_CONTRACT_MANAGER), search_result_person(b).addr()) as i32;
    let d = mb - morale(read_usize(G_CONTRACT_MANAGER), search_result_person(a).addr()) as i32;
    if d != 0 {
        return d;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by injury status, ascending (`exe::injury_get_player_injury_status`, `b` read first:
/// `status(a) - status(b)`), then by condition, highest first (`exe::injury_get_condition(person, 1)`, `b` first:
/// `condition(b) - condition(a)`), then the name.
/// Original: search_screens_compare_staff_by_fitness (0x81a940, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_fitness(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort by fitness");
    let (a, b) = (a as usize, b as usize);
    let (sa, sb) = (search_result_person(a), search_result_person(b));
    let injury = read_usize(G_INJURY);
    let ib = exe::injury_get_player_injury_status(injury, sb) as i8 as i32;
    let d = exe::injury_get_player_injury_status(injury, sa) as i8 as i32 - ib;
    if d != 0 {
        return d;
    }
    let cb = exe::injury_get_condition(injury, sb, 1) as i32;
    let d = cb - exe::injury_get_condition(injury, sa, 1) as i32;
    if d != 0 {
        return d;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by the weekly wage of the main contract, lowest first: `a` without a contract gives -1
/// when `b` has one and 0 (no name tie-break) when neither has; `b` without one gives 1; then `wage(a) - wage(b)` and
/// the name.
/// Original: search_screens_compare_staff_by_wage (0x81aa50, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_wage(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort by wage");
    let (a, b) = (a as usize, b as usize);
    let (ca, cb) = search_entry_contracts(a, b);
    if ca == 0 {
        return -((cb != 0) as i32);
    }
    if cb == 0 {
        return 1;
    }
    let d = read_i32(ca + contract::WAGE).wrapping_sub(read_i32(cb + contract::WAGE));
    if d != 0 {
        return d;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by the end date of the main contract, earliest first, with the contracts as in the wage
/// sort (`a` without one -1 or 0, `b` without one 1); `Date::operator<` -1, `Date::operator>` 1, then the name.
/// Original: search_screens_compare_staff_by_contract_end (0x81ab60, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_contract_end(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort by contract end");
    let (a, b) = (a as usize, b as usize);
    let (ca, cb) = search_entry_contracts(a, b);
    if ca == 0 {
        return -((cb != 0) as i32);
    }
    if cb == 0 {
        return 1;
    }
    let end_a = ca + contract::CONTRACT_EXPIRES;
    let (lo, hi) = (read_u32(cb + contract::CONTRACT_EXPIRES), read_u32(cb + contract::CONTRACT_EXPIRES + 4));
    if exe::date_operator_3c(end_a, lo, hi) != 0 {
        return -1;
    }
    let (lo, hi) = (read_u32(cb + contract::CONTRACT_EXPIRES), read_u32(cb + contract::CONTRACT_EXPIRES + 4));
    if exe::date_operator_3e(end_a, lo, hi) != 0 {
        return 1;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by the release clause of the main contract.
///
/// A null person goes last (`a` 1, `b` -1). Each side's clause code is -1 and its text "  -" (30 bytes) unless the
/// person has a contract, whose `get_release_clause_abbreviation(text, 0x1e)` gives both. The byte difference
/// `code(b) - code(a)` (8 bits), then the texts by plain `strcmp` (the result's low byte, -1 or 1), then the name.
/// Original: search_screens_compare_staff_by_release_clause (0x81aca0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_release_clause(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort by release clause");
    let (a, b) = (a as usize, b as usize);
    let none = read_u32(SEARCH_RELEASE_CLAUSE_NONE).to_le_bytes();
    let (mut ta, mut tb) = ([0u8; 0x1e], [0u8; 0x1e]);
    ta[..4].copy_from_slice(&none);
    tb[..4].copy_from_slice(&none);
    if search_result_person(a).addr() == 0 {
        return 1;
    }
    if search_result_person(b).addr() == 0 {
        return -1;
    }
    let (ca, cb) = search_entry_contracts(a, b);
    let abbreviation: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> u8 = core::mem::transmute(SEARCH_FN_RELEASE_CLAUSE_ABBREVIATION);
    let mut code_a = 0xffu8;
    let mut code_b = 0xffu8;
    if ca != 0 {
        code_a = abbreviation(ca, ta.as_mut_ptr(), 0x1e);
    }
    if cb != 0 {
        code_b = abbreviation(cb, tb.as_mut_ptr(), 0x1e);
    }
    let d = code_b.wrapping_sub(code_a) as i8;
    if d != 0 {
        return d as i32;
    }
    // the inline strcmp of the original: -1 / 0 / 1 by the first differing byte (unsigned)
    let mut k = 0;
    let r: i8 = loop {
        let (x, y) = (ta[k], tb[k]);
        if x != y {
            break if x < y { -1 } else { 1 };
        }
        if x == 0 {
            break 0;
        }
        k += 1;
    };
    if r != 0 {
        return r as i32;
    }
    search_results_compare_names(a, b)
}

/// Sorts two player attribute ids (1 to 31, by pointer) by their three-letter names; invalid ids last.
/// Original: search_screens_compare_player_attribute_short_names (0x81ae60, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_player_attribute_short_names(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort player attribute short names");
    search_compare_attribute_names(a as *const u8, b as *const u8, 0x1f, SEARCH_FN_PLAYER_ATTRIBUTE_NAME, 1)
}

/// Sorts two non-player attribute ids (1 to 12, by pointer) by their three-letter names; invalid ids last.
/// Original: search_screens_compare_non_player_attribute_short_names (0x81af00, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_non_player_attribute_short_names(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort non-player attribute short names");
    search_compare_attribute_names(a as *const u8, b as *const u8, 0xc, SEARCH_FN_NON_PLAYER_ATTRIBUTE_NAME, 1)
}

/// Sorts two player attribute ids (1 to 31, by pointer) by their long names; invalid ids last.
/// Original: search_screens_compare_player_attribute_names (0x81afa0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_player_attribute_names(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort player attribute names");
    search_compare_attribute_names(a as *const u8, b as *const u8, 0x1f, SEARCH_FN_PLAYER_ATTRIBUTE_NAME, 0)
}

/// Sorts two non-player attribute ids (1 to 12, by pointer) by their long names; invalid ids last.
/// Original: search_screens_compare_non_player_attribute_names (0x81b040, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_non_player_attribute_names(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort non-player attribute names");
    search_compare_attribute_names(a as *const u8, b as *const u8, 0xc, SEARCH_FN_NON_PLAYER_ATTRIBUTE_NAME, 0)
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
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SEARCH_EDIT_SESSION_DESTRUCTOR)(session)
}

/// Comparer for `search_screens_close_player_search_screen`.
pub(crate) unsafe extern "C" fn compare_search_screens_close_player_search_screen() {
    // simplification: saving and freeing the session do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(IDX_SEARCH_SCREENS_CLOSE_PLAYER_SEARCH_SCREEN)()
}

/// Comparer for `search_screens_open_filters_screen`: only the original runs (it opens a screen and allocates); when
/// the screen opened with the session, the plan is checked: param 1 is a copy of the current search's filter block and
/// param 2 its last page.
pub(crate) unsafe extern "C" fn compare_search_screens_open_filters_screen(session: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(IDX_SEARCH_SCREENS_OPEN_FILTERS_SCREEN)(session);
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
    let o = original_fn::<unsafe extern "C" fn(i16) -> i32>(IDX_SEARCH_SCREENS_HANDLE_FILTERS_INPUT);
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

/// Builds the search Filters screen of the session's current search (params: 0 the session, 1 the filters being
/// edited, 2 the page: 1 General, 2 Attributes).
///
/// The function-static green (`FILTERS_STATIC_GREEN`) is made on the first call. The current slot's human manager is
/// needed (none: the "Error" box of search_screens.cpp line 0x104e) and so is his nation: his club's nation, else his
/// national side's nation, else his own (none: line 0x105d). The screen: no background picture, the whole-screen area,
/// a frame 0x6b..0x30d and centred vertically (0x15e high for a staff search, 500 otherwise) in `rgb(0, 0x38, 0)`, the
/// title "Filters for <search name>", the General / Attributes toggle (the page shown is plain text, the other a button
/// with id 1 and the page as value), Clear (plain text while the general mask is the default, else a button with id 2),
/// Cancel (id 8) and Ok (id 9), the pitch drawn behind the body (`draw_pitch`) and the seven column layouts of the body
/// (7 rows for a staff search and 14 otherwise; the 9-column ones 7 or 16).
///
/// The Attributes page (`filters_screen_attributes_page`) and the General page (`filters_screen_general_page`) follow.
/// Original: search_screens_build_filters_screen (0x8133a0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_build_filters_screen() {
    covered!("tel21_5 build the filters screen");
    let c = job_colour;
    let guard = read_u8(FILTERS_STATIC_GUARD);
    if guard & 1 == 0 {
        write_u8(FILTERS_STATIC_GUARD, guard | 1);
        write_u16(FILTERS_STATIC_GREEN, exe::graphics_rgb_to_pixel(0, 0xc0, 0, 0) as u16);
    }
    let manager = tel26_slot_manager();
    if manager.addr() == 0 {
        tel26_error_box(SEARCH_SCREENS_CPP, 0x104e);
        return;
    }
    let club = manager.club_job();
    let nation = if !club.is_null() && club.nation().addr() != 0 {
        club.nation().addr()
    } else if !manager.national_job().is_null() {
        manager.national_job().nation().addr()
    } else {
        manager.nation().addr()
    };
    if nation == 0 {
        tel26_error_box(SEARCH_SCREENS_CPP, 0x105d);
        return;
    }
    let session = get_screen_param(0);
    let filters = get_screen_param(1);
    let page = get_screen_param(2) as u8;
    exe::background_set_picture(0usize);
    tel26_add_area(current_human_display(), [0, 0, 0x31f, 0x257, 1, 0, 1, 0, 2, 0, u32::MAX]);
    let search = search_session_current(session);
    let list_type = read_u8(search + player_search_fields::LIST_TYPE);
    let staff = list_type == FILTERS_LIST_STAFF;
    let (height, rows) = if staff { (0x15e, 7) } else { (500, 0xe) };
    let mut y1: i32 = (600 - height) / 2;
    let mut y2: i32 = y1 + height - 1;
    let frame = exe::graphics_rgb_to_pixel(0, 0x38, 0, 0);
    tel26_add_area(current_human_display(), [0x6b, y1 as u32, 0x30d, y2 as u32, 1, 0, 1, 0, 0x30, frame, u32::MAX]);
    y1 += 10;
    y2 -= 10;
    filters_text1(0xa8019c, search_session_current(session) + search_filters_fields::NAME); // "Filters for <%s - search name>"
    exe::screenmanager_add_text_item(G_SCREEN_MANAGER, 0x75, y1 as u32, 0x303, (y1 + 0x2d) as u32, 0x30, c(JOB_COLOUR_BDEA), 0xc, 4, c(JOB_COLOUR_3184), FILTERS_TEXT as u32, 0);
    y1 += 0x37;
    let toggle = filters_area([0x75, y1 as u32, 0x16f, (y1 + 0x14) as u32, 2, 0, 1, 0, 1, 0, u32::MAX]);
    filters_text1(0xa8016c, FILTERS_NO_COMMENT); // "General<..>"
    if page == 1 {
        tel26_area_text([toggle, 0, 0, 0x70, c(JOB_COLOUR_317E), 0xc, 1, c(JOB_COLOUR_3184), FILTERS_TEXT as u32]);
    } else {
        tel26_area_button([toggle, 0, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 1, c(JOB_COLOUR_3184), FILTERS_TEXT as u32, 1, 1]);
    }
    filters_text1(0xa80138, FILTERS_NO_COMMENT); // "Attributes<..>"
    if page == 2 {
        tel26_area_text([toggle, 1, 0, 0x70, c(JOB_COLOUR_317E), 0xc, 1, c(JOB_COLOUR_3184), FILTERS_TEXT as u32]);
    } else {
        tel26_area_button([toggle, 1, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 1, c(JOB_COLOUR_3184), FILTERS_TEXT as u32, 1, 2]);
    }
    filters_text1(0xa80110, FILTERS_NO_COMMENT); // "Clear<..>"
    if read_u32(filters) == FILTERS_DEFAULT_MASK[0] && read_u32(filters + 4) == FILTERS_DEFAULT_MASK[1] {
        exe::screenmanager_add_text_item(G_SCREEN_MANAGER, 0x286, y1 as u32, 0x303, (y1 + 0x14) as u32, 0x30, c(JOB_COLOUR_317E), 0x2c, 1, c(JOB_COLOUR_3184), FILTERS_TEXT as u32, 0);
    } else {
        tel26_add_button([0x286, y1 as u32, 0x303, (y1 + 0x14) as u32, 0x30, c(JOB_COLOUR_317E), 0xc, 1, c(JOB_COLOUR_3184), FILTERS_TEXT as u32, 2, 0]);
    }
    let weights_buttons: [u8; 2] = [3, 1];
    let buttons = filters_area([0x75, (y2 - 0x1e) as u32, 0x303, y2 as u32, 2, weights_buttons.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    filters_text(0x98a1a0); // "Cancel"
    tel26_area_button([buttons, 0, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 3, c(JOB_COLOUR_3184), FILTERS_TEXT as u32, 8, 0]);
    filters_text(0x990f7c); // "Ok"
    tel26_area_button([buttons, 1, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 3, c(JOB_COLOUR_3184), FILTERS_TEXT as u32, 9, 0]);
    y1 += 0x23;
    y2 -= 0x32;
    let mut pitch = [0i32; 4];
    let draw_pitch: unsafe extern "C" fn(u32, u32, u32, u32, *mut i32, *mut i32, *mut i32, *mut i32, u32, u32, f32) = core::mem::transmute(FN_DRAW_PITCH);
    let p = pitch.as_mut_ptr();
    draw_pitch(0x7f, y1 as u32, 0x2f9, y2 as u32, p, p.add(1), p.add(2), p.add(3), 1, buttons, 4.0);
    y1 += 5;
    y2 -= 5;
    let w_age: [u8; 7] = [2, 1, 1, 1, 1, 1, 1];
    let w_nationality: [u8; 7] = [8, 4, 4, 4, 7, 1, 4];
    let w_based: [u8; 7] = [8, 4, 4, 4, 4, 7, 1];
    let w_job: [u8; 7] = [8, 4, 4, 7, 1, 4, 4];
    let w_attributes: [u8; 9] = [6, 1, 1, 1, 1, 6, 1, 1, 1];
    let w_match: [u8; 9] = [6, 1, 1, 1, 1, 5, 1, 2, 1];
    let w_text: [u8; 7] = [8, 4, 4, 1, 0xf, 0, 0];
    let body = |cols: u32, weights: *const u8, rows: u32| filters_area([0x84, y1 as u32, 0x2f4, y2 as u32, cols, weights as u32, rows, 0, 1, 0, u32::MAX]);
    let rows9 = if staff { 7 } else { 0x10 };
    let mut b = FiltersBuild {
        filters,
        staff,
        list_type,
        manager,
        nation,
        age: body(7, w_age.as_ptr(), rows),
        nationality: 0,
        based: 0,
        job: 0,
        attributes: 0,
        attributes_match: 0,
        text: 0,
    };
    b.nationality = body(7, w_nationality.as_ptr(), rows);
    b.based = body(7, w_based.as_ptr(), rows);
    b.job = body(7, w_job.as_ptr(), rows);
    b.attributes = body(9, w_attributes.as_ptr(), rows9);
    b.attributes_match = body(9, w_match.as_ptr(), rows9);
    b.text = body(7, w_text.as_ptr(), rows);
    if page == 2 {
        filters_screen_attributes_page(&b);
    } else {
        filters_screen_general_page(&b);
    }
}

/// Comparer of `search_screens_build_filters_screen`: side by side over the display list (with the screen params) and
/// the function-static green and its guard.
pub(crate) unsafe extern "C" fn compare_search_screens_build_filters_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_SEARCH_SCREENS_BUILD_FILTERS_SCREEN);
    let extra = [(FILTERS_STATIC_GUARD, 1), (FILTERS_STATIC_GREEN, 2)];
    compare_by_display_list_with("Search filters screen", &extra, || o(), || search_screens_build_filters_screen());
}

/// Rule: one more attribute must match, up to the maximum.
///
/// `SearchFilters::increase_attributes_to_match(which)`: the counter of the set (`which` ≠ 0 the goalkeeper's) goes up
/// by one while below its maximum (signed bytes) and 1 comes back; at the maximum 0.
/// Original: `SearchFilters::increase_attributes_to_match` (0x8084c0, search_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn search_filters_increase_attributes_to_match(filters: usize, which: u32) -> u32 {
    let (count, max) = if which != 0 { (SEARCH_FILTERS_GK_COUNT, SEARCH_FILTERS_GK_MAX) } else { (SEARCH_FILTERS_COUNT, SEARCH_FILTERS_MAX) };
    let n = read_u8(filters + count) as i8;
    if n >= read_u8(filters + max) as i8 {
        return 0;
    }
    covered!("tel36_10 one more attribute to match");
    write_u8(filters + count, n.wrapping_add(1) as u8);
    1
}

/// Rule: one attribute fewer must match, down to one.
///
/// `SearchFilters::decrease_attributes_to_match(which)`: the counter of the set goes down by one while above 1 (signed)
/// and 1 comes back; at 1 or below 0.
/// Original: `SearchFilters::decrease_attributes_to_match` (0x808500, search_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn search_filters_decrease_attributes_to_match(filters: usize, which: u32) -> u32 {
    let count = if which != 0 { SEARCH_FILTERS_GK_COUNT } else { SEARCH_FILTERS_COUNT };
    let n = read_u8(filters + count) as i8;
    if n <= 1 {
        return 0;
    }
    covered!("tel36_10 one attribute fewer to match");
    write_u8(filters + count, n.wrapping_sub(1) as u8);
    1
}

/// Rule: a search filter mask with a single bit gives its index (0..63); the double mask 0x3_8000_0000 gives 0x40;
/// anything else -1.
///
/// `search_screens_filter_bit_to_index(low, high)`: the 64-bit mask high:low compared against 1 << k for k = 0..63 in
/// order, then against `SEARCH_FILTER_INDEX_0X40`.
/// Original: `search_screens_filter_bit_to_index` (0x81b0e0, search_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn search_screens_filter_bit_to_index(low: u32, high: u32) -> i8 {
    let mask = (high as u64) << 32 | low as u64;
    if mask.count_ones() == 1 {
        return mask.trailing_zeros() as i8;
    }
    if mask == SEARCH_FILTER_INDEX_0X40 {
        covered!("tel36_10 filter bit index 0x40");
        return 0x40;
    }
    -1
}

/// Rule: a search filter index gives its mask: 1 << k for 0..63, 0x3_8000_0000 for 0x40, 0 otherwise.
///
/// `search_screens_filter_index_to_bit(k)`: the 64-bit mask in edx:eax for the char `k`.
/// Original: `search_screens_filter_index_to_bit` (0x81b4e0, search_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn search_screens_filter_index_to_bit(k: u32) -> u64 {
    match k as u8 {
        b @ 0..=0x3f => 1u64 << b,
        0x40 => SEARCH_FILTER_INDEX_0X40,
        _ => 0,
    }
}

/// Comparer for `search_filters_increase_attributes_to_match`: the two counters and the answer side by side.
pub(crate) unsafe extern "thiscall" fn compare_search_filters_increase_attributes_to_match(filters: usize, which: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_SEARCH_FILTERS_INCREASE_ATTRIBUTES_TO_MATCH);
    let reg = [(filters + SEARCH_FILTERS_MAX, 2), (filters + SEARCH_FILTERS_GK_MAX, 2)];
    let (r, n, same, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(filters, which), || search_filters_increase_attributes_to_match(filters, which));
    SCOREBOARD.record(same && r == n, || format!("SearchFilters::increase_attributes_to_match({}): original={} rust={} differences {:?}", which, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `search_filters_decrease_attributes_to_match`.
pub(crate) unsafe extern "thiscall" fn compare_search_filters_decrease_attributes_to_match(filters: usize, which: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_SEARCH_FILTERS_DECREASE_ATTRIBUTES_TO_MATCH);
    let reg = [(filters + SEARCH_FILTERS_MAX, 2), (filters + SEARCH_FILTERS_GK_MAX, 2)];
    let (r, n, same, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(filters, which), || search_filters_decrease_attributes_to_match(filters, which));
    SCOREBOARD.record(same && r == n, || format!("SearchFilters::decrease_attributes_to_match({}): original={} rust={} differences {:?}", which, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `search_screens_filter_bit_to_index`.
pub(crate) unsafe extern "C" fn compare_search_screens_filter_bit_to_index(low: u32, high: u32) -> i8 {
    let r = original_fn::<unsafe extern "C" fn(u32, u32) -> i8>(IDX_SEARCH_SCREENS_FILTER_BIT_TO_INDEX)(low, high);
    let n = search_screens_filter_bit_to_index(low, high);
    SCOREBOARD.record(r == n, || format!("search_screens_filter_bit_to_index({:#x}, {:#x}): original={} rust={}", low, high, r, n));
    r
}

/// Comparer for `search_screens_filter_index_to_bit`.
pub(crate) unsafe extern "C" fn compare_search_screens_filter_index_to_bit(k: u32) -> u64 {
    let r = original_fn::<unsafe extern "C" fn(u32) -> u64>(IDX_SEARCH_SCREENS_FILTER_INDEX_TO_BIT)(k);
    let n = search_screens_filter_index_to_bit(k);
    SCOREBOARD.record(r == n, || format!("search_screens_filter_index_to_bit({}): original={:#x} rust={:#x}", k as u8, r, n));
    r
}

/// List `k` of the session `s` (the index sign-extended).
pub(crate) fn finder_list_at(s: usize, k: i8) -> usize {
    (s as isize + k as isize * FINDER_LIST_SIZE as isize) as usize
}

/// The current list of the session `s`.
pub(crate) unsafe fn finder_current(s: usize) -> usize {
    finder_list_at(s, read_i8(s))
}

/// The game's text function (`langlib_sprintf`, cdecl, variadic).
pub(crate) unsafe fn finder_lang() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_LANGLIB_SPRINTF)
}

/// A thiscall with one dword argument, returning eax.
pub(crate) unsafe fn finder_call1(f: usize, this: usize, a: u32) -> u32 {
    let g: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(f);
    g(this, a)
}

/// The Display of the current screen slot.
pub(crate) unsafe fn finder_display() -> usize {
    screens_current_display(G_SCREEN_MANAGER)
}

/// The search screen's tab bar (one tab per list: the list's name, greyed for a scout list without a scout), the
/// Search button and its menu: for a scout list with a scout, Assign Scout (when he is free: the duty competitions,
/// Club, Next Opposition, Youth Players, the nations and the regions) or Recall Scout; Load, Save, Save As, Save For (the
/// other humans with a password; only with more than one human), Delete, Unlock or Lock, Reset, Clear and the four
/// recent files of the list's kind.
pub(crate) unsafe fn finder_build_menu(b: &mut FinderBuild) {
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
                let mgr = Staff(b.mgr);
                let shown = (n != 0 && (read_i8(n + 0x85) > 8 || read_u8(n + 0x11c) != 0))
                    || {
                        let c = mgr.club_job().addr();
                        c != 0 && n == read_usize(c + club::NATION)
                    }
                    || {
                        let c = mgr.national_job().addr();
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
            let p = Staff(staff_at(((read_i32(G_STAFF_COUNT) - k - 1)) as isize).addr());
            if p.classification() == 1 && p.addr() != b.mgr {
                exe::database_format_staff_name(p, name.as_mut_ptr() as usize, 0xc8, 1);
                if read_u8(finder_password(p.addr())) != 0 {
                    finder_entry(2, true, name.as_ptr() as usize, 0xb, p.addr() as u32, humans);
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

/// The View button and its menus (only with results; otherwise a greyed "View"): General, Contract, Stats, More Stats
/// (not for staff), Attributes, Scout Report (a list following a club), Player Notes and Other (not for staff), each
/// ticked by its view flag; with the Attributes view, the Attributes button and its groups (Non Playing for staff;
/// Physical, Mental, Goalkeeping, Defensive and Attacking for players) and Filtered.
pub(crate) unsafe fn finder_build_view_menu(b: &mut FinderBuild) {
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
pub(crate) unsafe fn finder_build_filters_bar(b: &mut FinderBuild) -> u32 {
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
pub(crate) unsafe fn finder_build_heading(b: &mut FinderBuild) {
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

/// The list's name (the scout manager's name for a list not from a file; at most 0x7cf characters, in yellow), "LOCKED"
/// for a locked list, and the found line: "N players/staff found", with "(M filtered out)" when the total is larger.
pub(crate) unsafe fn finder_build_title(b: &mut FinderBuild) {
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

/// The column comparer of the result list for the sort flags (`SearchEditSession` + 0x298 of the current list): the
/// first set flag in the original's order (bit 0 with a filter flag picks the stars or rating comparers); 0 for none.
pub(crate) unsafe fn finder_sort_comparer(sort: u32, filter_flags: u32, list_type: u8) -> usize {
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

/// The header of the view, the list area, the pick column and the rows of the current page (see
/// `search_screens_build_player_search_screen`).
pub(crate) unsafe fn finder_build_rows(b: &mut FinderBuild, bar: u32) {
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

/// A thiscall without arguments returning eax.
pub(crate) unsafe fn finder_call0(f: usize, this: usize) -> u32 {
    let g: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(f);
    g(this)
}

/// The answers to the boxes and file dialogs the screen opened (the pending command, cleared; with the argument 2 the
/// box was accepted): 0x19, 0x1a, 0x1b, 0x3f, 0x45 the scout's nation, region, competition, youth players and next
/// opposition; 0x40 the club (the club pick opens); 0x1c the recall; 0x1d load another (the load dialog opens); 0x1e,
/// 0x1f, 0x20 the file chosen to load, save (with the lock toggle of param 11 and the lock owner of param 3) and delete
/// (a box when the file is locked by another human or fails); 0x21 save before the lock changes (the save dialog
/// opens); 0x22 reset; 0x23 clear; 0x24 load the recent file of param 4; 0x25 cancel the changes (the screen
/// closes); 0x26 delete the person of param 6. `Some(code)` when the answer returns.
pub(crate) unsafe fn finder_pending(st: &mut FinderInput) -> Option<i32> {
    let code = || read_u32(G_PENDING_COMMAND_CODE);
    let arg = || read_u32(G_PENDING_COMMAND_ARG);
    let clear_code = || write_u32(G_PENDING_COMMAND_CODE, 0);
    let clear_arg = || write_u32(G_PENDING_COMMAND_ARG, 0);
    if code() == 0x19 {
        clear_code();
        if arg() == 2 {
            st.scope(get_screen_param(8) as u32, u32::MAX, u32::MAX, 0, 0);
        }
        clear_arg();
    }
    if code() == 0x40 {
        clear_code();
        if arg() == 2 {
            covered!("tel19_2 search answer club scope");
            get_scout(st.search());
            finder_call1(FINDER_SET_ASSIGNED_CLUB, st.search(), (-2i32) as u32);
            clear_arg();
            return Some(finder_open_club_pick(st));
        }
        clear_arg();
    }
    if code() == 0x3f {
        clear_code();
        if arg() == 2 {
            st.scope(u32::MAX, u32::MAX, u32::MAX, 1, 0);
        }
        clear_arg();
    }
    if code() == 0x1a {
        clear_code();
        if arg() == 2 {
            let r = get_screen_param(9) as u8;
            st.scope(u32::MAX, r as u32, u32::MAX, 0, 0);
        }
        clear_arg();
    }
    if code() == 0x45 {
        clear_code();
        if arg() == 2 {
            st.scope(u32::MAX, u32::MAX, u32::MAX, 0, 1);
        }
        clear_arg();
    }
    if code() == 0x1b {
        clear_code();
        if arg() == 2 {
            st.scope(u32::MAX, u32::MAX, get_screen_param(0xa) as u32, 0, 0);
        }
        clear_arg();
    }
    if code() == 0x1c {
        clear_code();
        if arg() == 2 {
            finder_call0(FINDER_RESET_SCOUT, st.s);
        }
        clear_arg();
    }
    if code() == 0x1d {
        clear_code();
        if arg() == 2 {
            clear_arg();
            let k = st.kind();
            return Some(st.file_dialog(finder_text::LOAD[k], finder_text::SELECT_FILE, 0, 0, 0x1e));
        }
        clear_arg();
    }
    if code() == 0x1e {
        let path = arg() as usize;
        clear_code();
        if path != 0 {
            covered!("tel19_2 search answer load file");
            if st.locked(path) {
                st.file_box(path, finder_text::LOCKED_LOAD, true, finder_text::UNABLE_LOAD, finder_text::LOAD);
                return Some(st.drop_path(path));
            }
            if finder_call1(FINDER_LOAD_FILE, st.s, path as u32) == 0 {
                st.file_box(path, finder_text::NOT_LOADED, false, finder_text::UNABLE_LOAD, finder_text::LOAD);
                return Some(st.drop_path(path));
            }
            exe::crt_operator_delete(path);
            clear_arg();
        }
    }
    if code() == 0x1f {
        clear_code();
        let toggle = get_screen_param(0xb);
        exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0xb, 0usize, 0);
        let path = arg() as usize;
        if path != 0 {
            covered!("tel19_2 search answer save file");
            if st.locked(path) {
                st.file_box(path, finder_text::LOCKED_SAVE, true, finder_text::UNABLE_SAVE, finder_text::SAVE);
                return Some(st.drop_path(path));
            }
            if toggle != 0 {
                st.toggle_lock();
            }
            let save: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FINDER_SAVE_FILE);
            if save(st.s, path, st.lock_owner) == 0 {
                if toggle != 0 {
                    st.toggle_lock();
                }
                st.file_box(path, finder_text::NOT_SAVED_FILE, false, finder_text::UNABLE_SAVE, finder_text::SAVE);
                return Some(st.drop_path(path));
            }
            exe::crt_operator_delete(path);
            clear_arg();
        }
    }
    if code() == 0x20 {
        let path = arg() as usize;
        clear_code();
        if path != 0 {
            covered!("tel19_2 search answer delete file");
            if st.locked(path) {
                st.file_box(path, finder_text::LOCKED_DELETE, true, finder_text::UNABLE_DELETE, finder_text::DELETE);
                return Some(st.drop_path(path));
            }
            if exe::crt_remove(path as *const u8) != 0 {
                st.file_box(path, finder_text::NOT_DELETED, false, finder_text::UNABLE_DELETE, finder_text::DELETE);
                return Some(st.drop_path(path));
            }
            exe::crt_operator_delete(path);
            clear_arg();
        }
    }
    if code() == 0x21 {
        clear_code();
        if arg() == 2 {
            clear_arg();
            return Some(finder_save_dialog(st));
        }
        exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0xb, 0usize, 0);
        clear_arg();
    }
    if code() == 0x22 {
        clear_code();
        if arg() == 2 {
            clear_arg();
            finder_call0(FINDER_REVERT, st.s);
        }
        clear_arg();
    }
    if code() == 0x23 {
        clear_code();
        if arg() == 2 {
            clear_arg();
            finder_call1(FINDER_CLEAR_RESULTS, st.s, 1);
        }
        clear_arg();
    }
    if code() == 0x24 {
        clear_code();
        if arg() == 2 {
            clear_arg();
            if let Some(r) = finder_load_recent(st) {
                return Some(r);
            }
        }
        clear_arg();
    }
    if code() == 0x25 {
        clear_code();
        if arg() == 2 {
            clear_arg();
            return Some(finder_close(st));
        }
        clear_arg();
    }
    if code() == 0x26 {
        clear_code();
        if arg() == 2 {
            clear_arg();
            finder_call1(FINDER_REMOVE_PLAYER, st.s, st.pending_person as u32);
        }
        clear_arg();
    }
    None
}

/// The screen closes: the session destroyed and freed, the scout manager without a session; -9.
pub(crate) unsafe fn finder_close(st: &FinderInput) -> i32 {
    if st.s != 0 {
        finder_call0(FINDER_SESSION_DROP, st.s);
        exe::crt_operator_delete(st.s);
    }
    set_edit_session(read_usize(G_SCOUT_MANAGER), 0, 0);
    FINDER_CLOSED
}

/// The scout's assignment confirmations (ids 4, 0x1f, 0x20, 0x22, 5, 6): the scout's name and the box asking to assign
/// him ("not saved" first when the scout search was never saved); the club assignment of a saved search opens the
/// club pick directly. `None` when the id is not one of them.
pub(crate) unsafe fn finder_assign(st: &mut FinderInput, id: u16) -> Option<i32> {
    let lang = finder_lang();
    let saved = |st: &FinderInput| unsafe { st.saved() };
    match id {
        4 => {
            covered!("tel19_2 search assign nation");
            let nation = st.value();
            let scout = get_scout(st.search());
            exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 8, nation, 0);
            let n = finder_nation(nation as i32);
            finder_copy(&mut st.place, n + 4);
            st.scout_name(scout, 1);
            let g = read_i8(finder_nation(nation as i32) + nation::GENDER_NAME) as i32;
            if !saved(st) {
                lang(JOB_TEXT, 0xa7f3e4usize, g, g, st.np(), st.sp(), st.pp()); // "The scout search .. not saved. Assign scout .. to {}<nation>{} ?"
                Some(st.confirm(finder_text::ASSIGN_SCOUT, finder_text::SCOUT_NOT_SAVED, 1, 0x19))
            } else {
                lang(JOB_TEXT, 0xa7f3b0usize, g, g, st.sp(), st.pp()); // "Assign scout <%s> to {}<%s - nation>{} ?"
                Some(st.confirm(finder_text::ASSIGN_SCOUT, finder_text::PLEASE_CONFIRM, 1, 0x19))
            }
        }
        0x1f => {
            let scout = get_scout(st.search());
            st.scout_name(scout, 1);
            if !saved(st) {
                lang(JOB_TEXT, 0xa7f348usize, st.np(), st.sp()); // ".. not saved. Assign scout .. to find youth players ?"
                Some(st.confirm(finder_text::ASSIGN_SCOUT, finder_text::SCOUT_NOT_SAVED, 1, 0x3f))
            } else {
                lang(JOB_TEXT, 0xa7f314usize, st.sp()); // "Assign scout <%s> to find youth players ?"
                Some(st.confirm(finder_text::ASSIGN_SCOUT, finder_text::PLEASE_CONFIRM, 1, 0x3f))
            }
        }
        0x20 => {
            let scout = get_scout(st.search());
            if !saved(st) {
                st.scout_name(scout, 1);
                lang(JOB_TEXT, 0xa7f2b4usize, st.np(), st.sp()); // ".. not saved. Assign scout .. to watch a club ?"
                return Some(st.confirm(finder_text::ASSIGN_SCOUT, finder_text::SCOUT_NOT_SAVED, 1, 0x40));
            }
            covered!("tel19_2 search assign club");
            finder_call1(FINDER_SET_ASSIGNED_CLUB, st.search(), (-2i32) as u32);
            Some(finder_open_club_pick(st))
        }
        0x22 => {
            let scout = get_scout(st.search());
            st.scout_name(scout, 1);
            if !saved(st) {
                lang(JOB_TEXT, 0xa7f238usize, st.np(), st.sp()); // ".. not saved. Assign scout .. to next opposition ?"
                Some(st.confirm(finder_text::ASSIGN_SCOUT, finder_text::SCOUT_NOT_SAVED, 1, 0x45))
            } else {
                lang(JOB_TEXT, 0xa7f208usize, st.sp()); // "Assign scout <%s> to next opposition ?"
                Some(st.confirm(finder_text::ASSIGN_SCOUT, finder_text::PLEASE_CONFIRM, 1, 0x45))
            }
        }
        5 => {
            let region = st.value() as u8;
            let scout = get_scout(st.search());
            exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 9, region as usize, 0);
            get_region_name(region as u32, st.pp(), 0xc8, u32::MAX);
            let form: unsafe extern "C" fn(u32) -> u8 = core::mem::transmute(FINDER_REGION_ARTICLE_FORM);
            let f = form(region as u32) as i8 as i32;
            st.scout_name(scout, 1);
            if !saved(st) {
                lang(JOB_TEXT, 0xa7f1a4usize, f, f, st.np(), st.sp(), st.pp()); // ".. not saved. Assign scout .. to {}<region>{} ?"
                Some(st.confirm(finder_text::ASSIGN_SCOUT, finder_text::SCOUT_NOT_SAVED, 1, 0x1a))
            } else {
                lang(JOB_TEXT, 0xa7f170usize, f, f, st.sp(), st.pp()); // "Assign scout <%s> to {}<%s - region>{} ?"
                Some(st.confirm(finder_text::ASSIGN_SCOUT, finder_text::PLEASE_CONFIRM, 1, 0x1a))
            }
        }
        6 => {
            let comp = st.value();
            let scout = get_scout(st.search());
            exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0xa, comp, 0);
            let c = finder_comp(comp as i32);
            finder_copy(&mut st.place, c + 4);
            st.scout_name(scout, 1);
            let g = read_i8(finder_comp(comp as i32) + 0x37) as i32;
            if !saved(st) {
                lang(JOB_TEXT, 0xa7f104usize, g, g, st.np(), st.sp(), st.pp()); // ".. not saved. Assign scout .. to {}<competition>{} ?"
                Some(st.confirm(finder_text::ASSIGN_SCOUT, finder_text::SCOUT_NOT_SAVED, 1, 0x1b))
            } else {
                lang(JOB_TEXT, 0xa7f0ccusize, g, g, st.sp(), st.pp()); // "Assign scout <%s> to {}<%s - competition>{} ?"
                Some(st.confirm(finder_text::ASSIGN_SCOUT, finder_text::PLEASE_CONFIRM, 1, 0x1b))
            }
        }
        _ => None,
    }
}

/// The recall box (id 7) for the scout's current assignment (club, youth players, next opposition, nation, region or
/// competition), pending 0x1c; `None` without an assignment.
pub(crate) unsafe fn finder_recall(st: &mut FinderInput) -> Option<i32> {
    let lang = finder_lang();
    let (mut club, mut nation, mut comp, mut youth, mut next) = (0u32, 0u32, 0u32, 0u32, 0u32);
    let mut region = [0u8; 4];
    if get_scout_assignment(st.search(), &mut club, &mut nation, region.as_mut_ptr(), &mut comp, &mut youth, &mut next) == 0 {
        return None;
    }
    covered!("tel19_2 search recall scout");
    let scout = get_scout(st.search());
    if club as i32 > -1 {
        let c = finder_club(club as i32);
        finder_copy(&mut st.place, c + club::SHORT_NAME);
        st.scout_name(scout, 1);
        let g = read_i8(finder_club(club as i32) + club::SHORT_GENDER_NAME) as i32;
        lang(JOB_TEXT, 0xa7f098usize, g, g, st.sp(), st.pp()); // "Recall scout <%s> from {}<%s - club>{} ?"
    } else if youth != 0 {
        // the nation's name is copied too (unused; the original reads it with the nation the assignment has, -1)
        let n = finder_nation(nation as i32);
        finder_copy(&mut st.place, n + 4);
        st.scout_name(scout, 1);
        lang(JOB_TEXT, 0xa7f060usize, st.sp()); // "Recall scout <%s> from finding youth players ?"
    } else if next != 0 {
        st.scout_name(scout, 1);
        lang(JOB_TEXT, 0xa7f020usize, st.sp()); // "Recall scout <%s> from researching next opposition ?"
    } else if nation as i32 > -1 {
        let n = finder_nation(nation as i32);
        finder_copy(&mut st.place, n + 4);
        st.scout_name(scout, 1);
        let g = read_i8(finder_nation(nation as i32) + nation::GENDER_NAME) as i32;
        lang(JOB_TEXT, 0xa7efecusize, g, g, st.sp(), st.pp()); // "Recall scout <%s> from {}<%s - nation>{} ?"
    } else if region[0] as i8 > -1 {
        get_region_name(region[0] as u32, st.pp(), 0xc8, u32::MAX);
        let form: unsafe extern "C" fn(u32) -> u8 = core::mem::transmute(FINDER_REGION_ARTICLE_FORM);
        let f = form(region[0] as u32) as i8 as i32;
        st.scout_name(scout, 1);
        lang(JOB_TEXT, 0xa7efb8usize, f, f, st.sp(), st.pp()); // "Recall scout <%s> from {}<%s - region>{} ?"
    } else {
        let c = finder_comp(comp as i32);
        finder_copy(&mut st.place, c + 4);
        st.scout_name(scout, 1);
        let g = read_i8(finder_comp(comp as i32) + 0x37) as i32;
        lang(JOB_TEXT, 0xa7ef80usize, g, g, st.sp(), st.pp()); // "Recall scout <%s> from {}<%s - competition>{} ?"
    }
    Some(st.confirm(finder_text::RECALL_SCOUT, finder_text::PLEASE_CONFIRM, 1, 0x1c))
}

/// The list commands: 8 load (the "not saved" box first, pending 0x1d, else the load dialog, 0x1e); 9 save (to the
/// human's folder under the list's name; a box when the file is locked by another human or fails); 0xa save as and 0xb
/// save for (param 3 = the other human for 0xb; the save dialog); 0xc delete (the delete dialog, 0x20); 0xd unlock and
/// 0xe lock (param 11 = 1 and the box to save first, 0x21); 0xf reset (the box when there are changes, 0x22; else the
/// list reverted at once); 0x10 clear (0x23); 0x11 a recent file (param 4; the "not saved" box first, 0x24).
pub(crate) unsafe fn finder_list_command(st: &mut FinderInput, id: u16) -> Option<i32> {
    let lang = finder_lang();
    let k = st.kind();
    match id {
        8 => {
            if !st.saved() {
                lang(JOB_TEXT, finder_text::LOAD_ANOTHER[k], st.np());
                return Some(st.confirm(finder_text::LOAD[k], finder_text::NOT_SAVED[k], 1, 0x1d));
            }
            Some(st.file_dialog(finder_text::LOAD[k], finder_text::SELECT_FILE, 0, 0, 0x1e))
        }
        9 => {
            covered!("tel19_2 search save");
            let folder: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FINDER_FOLDER_PATH);
            let dir = folder(read_usize(G_HUMAN_MANAGER), 0, 2);
            let path = st.path.as_mut_ptr() as usize;
            exe::crt_makepath(st.path.as_mut_ptr(), 0xad9c64usize, dir, st.np(), finder_text::EXTENSION);
            if st.locked(path) {
                return Some(st.file_box(path, finder_text::LOCKED_SAVE, true, finder_text::UNABLE_SAVE, finder_text::SAVE));
            }
            let save: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FINDER_SAVE_FILE);
            if save(st.s, path, 0) == 0 {
                return Some(st.file_box(path, finder_text::NOT_SAVED_FILE, false, finder_text::UNABLE_SAVE, finder_text::SAVE));
            }
            Some(FINDER_REDRAW)
        }
        0xa | 0xb => {
            let other = if id == 0xb { st.value() } else { 0 };
            exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 3, other, 0);
            Some(finder_save_dialog(st))
        }
        0xc => Some(st.file_dialog(finder_text::DELETE[k], finder_text::SELECT_FILE, 2, 0, 0x20)),
        0xd | 0xe => {
            exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0xb, 1usize, 0);
            let (text, title) = if id == 0xd { (finder_text::SAVE_TO_UNLOCK, finder_text::UNLOCK) } else { (finder_text::SAVE_TO_LOCK, finder_text::LOCK) };
            lang(JOB_TEXT, text[k]);
            Some(st.confirm(title[k], finder_text::NOT_SAVED[k], 1, 0x21))
        }
        0xf => {
            if read_u32((st.s as isize + read_i8(st.s) as isize * 4 + finder_session::RESET_AVAILABLE as isize) as usize) != 0 {
                lang(JOB_TEXT, finder_text::RESET[k]);
                return Some(st.confirm(finder_text::KIND[k], finder_text::CHANGES_MADE, 1, 0x22));
            }
            finder_call0(FINDER_REVERT, st.s);
            Some(FINDER_REDRAW)
        }
        0x10 => {
            lang(JOB_TEXT, finder_text::CLEAR[k]);
            Some(st.confirm(finder_text::KIND[k], finder_text::PLEASE_CONFIRM, 1, 0x23))
        }
        0x11 => {
            exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 4, st.value(), 0);
            if !st.saved() {
                lang(JOB_TEXT, finder_text::LOAD_ANOTHER[k], st.np());
                return Some(st.confirm(finder_text::LOAD[k], finder_text::NOT_SAVED[k], 1, 0x24));
            }
            covered!("tel19_2 search recent file");
            Some(finder_load_recent(st).unwrap_or(FINDER_REDRAW))
        }
        _ => None,
    }
}

/// The view and sort commands: 0x12 a view (the main views 1..0x40 and Scout Report replace the main view, keeping
/// the attribute groups; the groups 0x80..0x1000 replace the group); 0x1e a column (the same column again flips the
/// direction; the results recomputed), 0x14 a statistic column and 0x15 an attribute column (the same one again flips
/// the direction); 0x17..0x1a the pages (-10, -1, +1, +10, within 1 and the page count). Returns -11.
pub(crate) unsafe fn finder_view_command(st: &mut FinderInput, id: u16) -> Option<i32> {
    let view = read_usize(FINDER_VIEW_FLAGS);
    match id {
        0x12 => {
            let v = st.value() as u32;
            match v {
                1 | 2 | 4 | 8 | 0x10 | 0x20 | 0x40 | 0x2000 => {
                    write_u32(view, read_u32(view) & 0xffffdf80);
                    write_u32(read_usize(FINDER_VIEW_FLAGS), read_u32(read_usize(FINDER_VIEW_FLAGS)) | v);
                }
                0x80 | 0x100 | 0x200 | 0x400 | 0x800 | 0x1000 => {
                    write_u32(view, read_u32(view) & 0xffffe07f);
                    write_u32(read_usize(FINDER_VIEW_FLAGS), read_u32(read_usize(FINDER_VIEW_FLAGS)) | v);
                }
                _ => {}
            }
            Some(FINDER_REDRAW)
        }
        0x1e => {
            let v = st.value() as u32;
            if v != 0 && v.is_power_of_two() && v <= 0x20000000 {
                write_u32(st.s + finder_session::CACHED, 0);
                let f = read_usize(FINDER_VIEW_FLAGS);
                if read_u32(f + 4) & v != 0 && read_u32(f + 0xa) == 0 {
                    write_u32(f + 0xa, 1);
                } else {
                    write_u32(f + 0xa, 0);
                }
                write_u32(read_usize(FINDER_VIEW_FLAGS) + 4, read_u32(read_usize(FINDER_VIEW_FLAGS) + 4) & 0xc0000000);
                let f = read_usize(FINDER_VIEW_FLAGS);
                write_u32(f + 4, read_u32(f + 4) | v);
            }
            Some(FINDER_REDRAW)
        }
        0x14 | 0x15 => {
            write_u32(st.s + finder_session::CACHED, 0);
            let (bit, at) = if id == 0x14 { (0x400u32, 8usize) } else { (0x800, 9) };
            let f = read_usize(FINDER_VIEW_FLAGS);
            if read_u32(f + 4) & bit != 0 && st.value() as u8 == read_u8(f + at) && read_u32(f + 0xa) == 0 {
                write_u32(f + 0xa, 1);
            } else {
                write_u32(f + 0xa, 0);
            }
            let f = read_usize(FINDER_VIEW_FLAGS);
            write_u32(f + 4, read_u32(f + 4) & 0xe0000000);
            let f = read_usize(FINDER_VIEW_FLAGS);
            write_u32(f + 4, read_u32(f + 4) | bit);
            write_u8(read_usize(FINDER_VIEW_FLAGS) + at, st.value() as u8);
            Some(FINDER_REDRAW)
        }
        0x17 | 0x18 => {
            let page = read_i16(view + 0xe) as i32 + if id == 0x17 { -0xa } else { -1 };
            write_i16(view + 0xe, if page > 1 { page as i16 } else { 1 });
            Some(FINDER_REDRAW)
        }
        0x19 | 0x1a => {
            let page = read_i16(view + 0xe) as i32 + if id == 0x19 { 1 } else { 0xa };
            let last = st.pages as i32;
            write_i16(view + 0xe, if page < last { page as i16 } else { last as i16 });
            Some(FINDER_REDRAW)
        }
        _ => None,
    }
}

/// The person of a search result: the row of `G_STAFF_TABLE` of its staff id.
pub(crate) unsafe fn search_result_person(entry: usize) -> Staff {
    staff_at(read_i32(entry + search_result_keys::STAFF_ID) as isize)
}

/// The tie-break of every search sort: the two people's names in style 3 (`exe::database_format_staff_name` into 0xc8
/// bytes, `a` first) through `exe::utils_string_compare(name a, name b, 1, 0)`.
pub(crate) unsafe fn search_results_compare_names(a: usize, b: usize) -> i32 {
    let (mut na, mut nb) = ([0u8; 0xc8], [0u8; 0xc8]);
    exe::database_format_staff_name(search_result_person(a), na.as_mut_ptr() as usize, 0xc8, 3);
    exe::database_format_staff_name(search_result_person(b), nb.as_mut_ptr() as usize, 0xc8, 3);
    exe::utils_string_compare(na.as_ptr(), nb.as_ptr(), 1, 0)
}

/// The shared order of the scouted-match sorts: a person no scout follows goes last (`a` unfollowed 1, `b` unfollowed
/// -1, both: by name); otherwise `diff(slot a, slot b)` and, when it is 0, the name.
pub(crate) unsafe fn search_results_compare_scouted(a: usize, b: usize, diff: impl Fn(usize, usize) -> i32) -> i32 {
    let sa = search_result_scouted_slot(a);
    let sb = search_result_scouted_slot(b);
    if sa == 0 {
        if sb == 0 {
            return search_results_compare_names(a, b);
        }
        return 1;
    }
    if sb == 0 {
        return -1;
    }
    let d = diff(sa, sb);
    if d != 0 {
        return d;
    }
    search_results_compare_names(a, b)
}

/// The three scouted sums share their order: an unfollowed person last, the 16-bit difference of the sums
/// `sum(b) - sum(a)` (sign-extended), then the name.
pub(crate) unsafe fn search_results_compare_scouted_sum(a: usize, b: usize, f: usize) -> i32 {
    search_results_compare_scouted(a, b, |sa, sb| search_slot_sum(f, sb).wrapping_sub(search_slot_sum(f, sa)) as i32)
}

/// The main contracts of two search entries' people (`exe::contract_manager_get_contract`, `a` first).
pub(crate) unsafe fn search_entry_contracts(a: usize, b: usize) -> (usize, usize) {
    let ca = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), search_result_person(a).addr());
    let cb = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), search_result_person(b).addr());
    (ca, cb)
}

/// The shared order of the four attribute name sorts: a null pointer or an id outside `1..=last` goes last (`a` 1,
/// `b` -1); otherwise the two names (`name(id, text, 0xc8, short)`, `a` first) through
/// `exe::utils_string_compare(name a, name b, 1, 0)`.
pub(crate) unsafe fn search_compare_attribute_names(a: *const u8, b: *const u8, last: i8, f: usize, short: u32) -> i32 {
    if a.is_null() {
        return 1;
    }
    if b.is_null() {
        return -1;
    }
    let (ia, ib) = (*a as i8, *b as i8);
    if ia < 1 || ia > last {
        return 1;
    }
    if ib < 1 || ib > last {
        return -1;
    }
    let name: unsafe extern "C" fn(u32, *mut u8, u32, u32) -> u32 = core::mem::transmute(f);
    let (mut na, mut nb) = ([0u8; 0xc8], [0u8; 0xc8]);
    name(ia as u8 as u32, na.as_mut_ptr(), 0xc8, short);
    name(ib as u8 as u32, nb.as_mut_ptr(), 0xc8, short);
    exe::utils_string_compare(na.as_ptr(), nb.as_ptr(), 1, 0)
}

/// The current search of an edit session.
pub(crate) unsafe fn search_session_current(session: usize) -> usize {
    (session as isize + search_edit_session::SEARCHES as isize + read_i8(session + search_edit_session::CURRENT) as isize * search_edit_session::SEARCH_SIZE as isize) as usize
}

/// A thiscall on the filters with no argument.
pub(crate) unsafe fn search_filters_call0(f: usize, filters: usize) {
    let g: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
    g(filters)
}

/// A thiscall on the filters with one argument, returning eax.
pub(crate) unsafe fn search_filters_call1(f: usize, filters: usize, a: u32) -> u32 {
    let g: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(f);
    g(filters, a)
}

/// `SearchFilters::get_attribute_minimum(attribute, staff table)`.
pub(crate) unsafe fn search_filters_get_minimum(filters: usize, attribute: u32, staff_table: u32) -> u8 {
    let g: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(SEARCH_FN_FILTERS_GET_ATTRIBUTE_MINIMUM);
    g(filters, attribute, staff_table)
}

/// `SearchFilters::set_attribute_minimum(attribute, minimum, staff table)`.
pub(crate) unsafe fn search_filters_set_minimum(filters: usize, attribute: u32, minimum: u8, staff_table: u32) {
    let g: unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32 = core::mem::transmute(SEARCH_FN_FILTERS_SET_ATTRIBUTE_MINIMUM);
    g(filters, attribute, minimum as u32, staff_table);
}

/// `SearchFilters::set_option_flag(lo, hi)`: 1 when the mask changed.
pub(crate) unsafe fn search_filters_set_option_flag(filters: usize, mask: u64) -> u32 {
    let g: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(SEARCH_FN_FILTERS_SET_OPTION_FLAG);
    g(filters, mask as u32, (mask >> 32) as u32)
}

/// The widget data and action data of an item of the current human's display (`Display::get_widget_data`,
/// `ScreenManager::get_widget_action_data`) and its action id.
pub(crate) unsafe fn search_widget_data(item: i16) -> u32 {
    exe::display_get_widget_data(current_human_display(), item as i32 as u32) as u32
}

pub(crate) unsafe fn search_widget_action_data(item: i16) -> u32 {
    exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, item as i32 as u32) as u32
}

pub(crate) unsafe fn search_widget_action_id(item: i16) -> i32 {
    let g: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(SEARCH_FN_GET_WIDGET_ACTION_ID);
    g(G_SCREEN_MANAGER, item as i32 as u32)
}

/// `langlib_sprintf` into the shared text without and with one argument.
pub(crate) unsafe fn filters_text(fmt: usize) {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    f(FILTERS_TEXT, fmt);
}

pub(crate) unsafe fn filters_text1(fmt: usize, a: usize) {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    f(FILTERS_TEXT, fmt, a);
}

/// `Display::add_area` on the current human's display; the area index is the low 16 bits.
pub(crate) unsafe fn filters_area(f: [u32; 11]) -> u32 {
    tel26_add_area(current_human_display(), f) & 0xffff
}

/// The Attributes page of the Filters screen: every attribute (12 non-player ones on a staff search, 31 player ones
/// otherwise) sorted by name, two per row (columns 0 and 5 of the 9-column layout).
///
/// Each attribute: its name (" %s") as a button with id 3 (yellow when its minimum is above 1, else colour 0xae3154);
/// "-" (id 4, plain text while the minimum is below 2); the minimum ("%d", id 0x12, yellow above 1, else the static
/// green); "+" (id 5, plain text from 20). When at least two ranges are set, "Match" with "-" (id 6, plain text below 2
/// to match), "<to match>/<set>" and "+" (id 7, plain text when all of them must match).
pub(crate) unsafe fn filters_screen_attributes_page(b: &FiltersBuild) {
    covered!("tel21_5 filters attributes page");
    let c = job_colour;
    let (count, sort, name_of) = if b.staff {
        (12usize, FILTERS_FN_SORT_STAFF_ATTRIBUTES, FILTERS_FN_NON_PLAYER_ATTRIBUTE_NAME)
    } else {
        (31, FILTERS_FN_SORT_PLAYER_ATTRIBUTES, FILTERS_FN_PLAYER_ATTRIBUTE_NAME)
    };
    let table = b.staff as u32;
    let mut list = [0u8; 31];
    for (k, a) in list.iter_mut().enumerate().take(count) {
        *a = (k + 1) as u8;
    }
    exe::crt_qsort(list.as_mut_ptr() as usize, count as u32, 1, sort);
    let name: unsafe extern "C" fn(u32, *mut u8, u32, u32) -> u32 = core::mem::transmute(name_of);
    let minimum = |a: u8| search_filters_get_minimum(b.filters, a as u32, table) as i8;
    let area = b.attributes;
    let (mut col, mut row) = (0u32, 0u32);
    let mut buf = [0u8; 0xc8];
    for &a in list.iter().take(count) {
        let value = a as i8 as i32 as u32;
        name(a as u32, buf.as_mut_ptr(), 200, 0);
        filters_format(FILTERS_TEXT, 0x98ed14, buf.as_ptr() as usize); // " %s"
        let colour = if minimum(a) > 1 { c(JOB_COLOUR_BDEC) } else { c(FILTERS_COLOUR_3154) };
        tel26_area_button([area, col, row, 1, 0, 1, 2, colour, FILTERS_TEXT as u32, 3, value]);
        if minimum(a) < 2 {
            tel26_area_text([area, col + 1, row, 0x30, c(FILTERS_COLOUR_3150), 0xc, 1, c(JOB_COLOUR_3184), 0x98ba0c]);
        } else {
            tel26_area_button([area, col + 1, row, 0x30, c(FILTERS_COLOUR_3150), 0xc, 1, c(JOB_COLOUR_3184), 0x98ba0c, 4, value]);
        }
        filters_format(FILTERS_TEXT, 0x98806c, minimum(a) as i32 as usize); // "%d"
        let colour = if minimum(a) > 1 { c(JOB_COLOUR_BDEC) } else { c(FILTERS_STATIC_GREEN) };
        tel26_area_button([area, col + 2, row, 1, 0, 0xc, 2, colour, FILTERS_TEXT as u32, 0x12, value]);
        if minimum(a) < 0x14 {
            tel26_area_button([area, col + 3, row, 0x30, c(FILTERS_COLOUR_3150), 0xc, 1, c(JOB_COLOUR_3184), 0x9afe7c, 5, value]);
        } else {
            tel26_area_text([area, col + 3, row, 0x30, c(FILTERS_COLOUR_3150), 0xc, 1, c(JOB_COLOUR_3184), 0x9afe7c]);
        }
        if col == 0 {
            col = 5;
        } else {
            col = 0;
            row += 1;
        }
    }
    let (set, to_match) = if b.staff {
        (read_i8(b.filters + search_filters_block::STAFF_RANGES_SET), read_i8(b.filters + search_filters_block::STAFF_RANGES_TO_MATCH))
    } else {
        (read_i8(b.filters + search_filters_block::PLAYER_RANGES_SET), read_i8(b.filters + search_filters_block::PLAYER_RANGES_TO_MATCH))
    };
    if set < 2 {
        return;
    }
    filters_text1(0xa800ec, FILTERS_NO_COMMENT); // " Match<..>"
    let area = b.attributes_match;
    tel26_area_text([area, 5, row, 1, 0, 1, 2, c(FILTERS_COLOUR_3154), FILTERS_TEXT as u32]);
    if to_match < 2 {
        tel26_area_text([area, 6, row, 0x30, c(FILTERS_COLOUR_3150), 0xc, 1, c(JOB_COLOUR_3184), 0x98ba0c]);
    } else {
        tel26_area_button([area, 6, row, 0x30, c(FILTERS_COLOUR_3150), 0xc, 1, c(JOB_COLOUR_3184), 0x98ba0c, 6, 0]);
    }
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    f(FILTERS_TEXT, 0xa800cc, to_match as i32, set as i32); // "<%d - number1>/<%d - number2>"
    tel26_area_text([area, 7, row, 1, 0, 0xc, 2, c(JOB_COLOUR_BDE8), FILTERS_TEXT as u32]);
    if set <= to_match {
        tel26_area_text([area, 8, row, 0x30, c(FILTERS_COLOUR_3150), 0xc, 1, c(JOB_COLOUR_3184), 0x9afe7c]);
    } else {
        tel26_area_button([area, 8, row, 0x30, c(FILTERS_COLOUR_3150), 0xc, 1, c(JOB_COLOUR_3184), 0x9afe7c, 7, 0]);
    }
}

/// The General page of the Filters screen, row by row (headings in column 0, the options from column 1; a chosen
/// option is the yellow item, the others green buttons with id 10 and their flag's index; see the options' order in
/// the header of each piece):
/// - Text (not on a scout's match list): Any / Specified (high 0x200000 / 0x400000) and, when specified, the text field
///   (0x19 characters of the filters' +0x7f);
/// - Nationality (`filters_screen_nationality_row`), Age (`filters_screen_age_row`), Language
///   (`filters_screen_language_row`), Based (`filters_screen_based_row`);
/// - a staff search: Job (`filters_screen_job_row`); the others: Value (`filters_screen_value_row`), Status (Any, Listed,
///   For Loan: 0x10000000, 0x20000000, 0x40000000), Contract (Any is chosen when no contract flag is set; Expired,
///   Expiring: high 1, 2; Unprotected 0x80000000 only while the FIFA rules are in force), Position (Any, GK, Def, Mid,
///   Att: high 4..0x40) and Side (Any, Left, Centre, Right: high 0x80..0x400);
/// - a scout's match list: Ability (Any, Current, Future: high 0x800000..0x2000000);
/// - not a staff search: Injuries (Any, None: high 0x2000, 0x4000);
/// - Interested (Any, Yes: high 0x8000, 0x10000);
/// - not a staff search: Scouted Only (No, Yes: high 0x40000000, 0x80000000) and, when the manager has a club, Include
///   Squad (No, Yes: high 0x4000000, 0x8000000) with Senior / Reserve when the club has a reserve squad, else A / B when
///   it has a linked B team (high 0x10000000, 0x20000000; the last is the chosen item or, else, the button).
pub(crate) unsafe fn filters_screen_general_page(b: &FiltersBuild) {
    covered!("tel21_5 filters general page");
    let f = b.filters;
    let hi = read_u32(f + search_filters_block::MASK_HIGH);
    let mut row = 0u32;
    if b.list_type != FILTERS_LIST_SCOUT {
        let area = b.text;
        filters_text(0xa800c4); // " Text"
        filters_heading(area, 0);
        filters_text1(0xa8009c, FILTERS_NO_COMMENT); // "Any<..>"
        filters_option(area, 1, 0, hi & 0x200000 != 0, 0, 0x200000);
        filters_text(0xa80090); // "Specified"
        filters_option(area, 2, 0, hi & 0x400000 != 0, 0, 0x400000);
        if read_u32(f + search_filters_block::MASK_HIGH) & 0x400000 != 0 {
            let g = job_colour(FILTERS_STATIC_GREEN);
            filters_item([0x80018, 0, 0, 0, 0, 4, 0, 1, 0, 0, 1, 2, g, (f + search_filters_block::TEXT) as u32, 0x19, 0, 0, area]);
        }
        row = 1;
    }
    filters_screen_nationality_row(b, row);
    filters_screen_age_row(b, row + 1);
    filters_screen_language_row(b, row + 2);
    filters_screen_based_row(b, row + 3);
    let area = b.age;
    let option = |col: u32, row: u32, fmt: usize, comment: bool, lo_bit: u32, hi_bit: u32| {
        if comment { filters_text1(fmt, FILTERS_NO_COMMENT) } else { filters_text(fmt) }
        let set = (read_u32(f) & lo_bit) != 0 || (read_u32(f + 4) & hi_bit) != 0;
        filters_option(area, col, row, set, lo_bit, hi_bit);
    };
    if b.staff {
        row += 4;
        filters_screen_job_row(b, row);
    } else {
        filters_screen_value_row(b, row + 4);
        let r = row + 5;
        filters_text(0xa7fef0); // " Status"
        filters_heading(area, r);
        option(1, r, 0xa7fed4, true, 0x10000000, 0);
        option(2, r, 0xa7fecc, false, 0x20000000, 0); // "Listed"
        option(3, r, 0xa7fec0, false, 0x40000000, 0); // "For Loan"
        let r = row + 6;
        filters_text(0x98eec4); // " Contract"
        filters_heading(area, r);
        filters_text1(0xa7fea0, FILTERS_NO_COMMENT); // "Any<..contract>"
        // Any is the chosen item when no contract flag is set (the inverse of the other options)
        let index = filters_bit_index(0x80000000, 3);
        if read_u32(f) & 0x80000000 == 0 && read_u32(f + 4) & 3 == 0 {
            filters_chosen(area, 1, r, 10, index);
        } else {
            filters_option_button(area, 1, r, 10, index);
        }
        option(2, r, 0xa7fe98, false, 0, 1); // "Expired"
        option(3, r, 0xa7fe8c, false, 0, 2); // "Expiring"
        if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 {
            option(4, r, 0xa7fe58, true, 0x80000000, 0); // "Unprotected<..>"
        }
        let r = row + 7;
        filters_text(0xa7fe4c); // " Position"
        filters_heading(area, r);
        option(1, r, 0xa7fe2c, true, 0, 4);
        option(2, r, 0xa7fe28, false, 0, 8); // "GK"
        option(3, r, 0xa7fe24, false, 0, 0x10); // "Def"
        option(4, r, 0xa7fe20, false, 0, 0x20); // "Mid"
        option(5, r, 0xa7fe1c, false, 0, 0x40); // "Att"
        row += 8;
        filters_text(0xa7fe14); // " Side"
        filters_heading(area, row);
        option(1, row, 0xa7fdf8, true, 0, 0x80);
        option(2, row, 0x9b7444, false, 0, 0x100); // "Left"
        option(3, row, 0x9b7420, false, 0, 0x200); // "Centre"
        option(4, row, 0x9b7464, false, 0, 0x400); // "Right"
    }
    if b.list_type == FILTERS_LIST_SCOUT {
        row += 1;
        filters_text(0xa7ff48); // " Ability"
        filters_heading(area, row);
        option(1, row, 0xa7ff2c, true, 0, 0x800000);
        option(2, row, 0x99ccdc, false, 0, 0x1000000); // "Current"
        option(3, row, 0xa7fdf0, false, 0, 0x2000000); // "Future"
    }
    if !b.staff {
        row += 1;
        filters_text(0xa7fde4); // " Injuries"
        filters_heading(area, row);
        option(1, row, 0xa7fdc4, true, 0, 0x2000);
        option(2, row, 0xa7fda4, true, 0, 0x4000); // "None<..>"
    }
    let mut r = row + 1;
    filters_text1(0xa7fd70, FILTERS_NO_COMMENT); // " Interested<..>"
    filters_heading(area, r);
    option(1, r, 0xa7fd50, true, 0, 0x8000);
    option(2, r, 0x98e8d0, false, 0, 0x10000); // "Yes"
    if !b.staff {
        r = row + 2;
        filters_text(0xa7fd40); // " Scouted Only"
        filters_heading(area, r);
        option(1, r, 0xa7fd1c, true, 0, 0x40000000); // "No<..>"
        option(2, r, 0xa7fcf8, true, 0, 0x80000000); // "Yes<..>"
    }
    if b.staff {
        return;
    }
    let club = b.manager.club_job();
    if club.is_null() {
        return;
    }
    r += 1;
    filters_text(0xa7fce8); // " Include Squad"
    filters_heading(area, r);
    option(1, r, 0x98e8cc, false, 0, 0x4000000); // "No"
    option(2, r, 0x98e8d0, false, 0, 0x8000000); // "Yes"
    if exe::database_club_has_reserve_squad(club) == 0 {
        if exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1) == 0 {
            return;
        }
        option(3, r, 0xa7fcbc, true, 0, 0x10000000); // "A<..>"
        option(4, r, 0xa7fc98, true, 0, 0x20000000); // "B<..>"
    } else {
        option(3, r, 0xa7fce0, false, 0, 0x10000000); // "Senior"
        option(4, r, 0x9b5b40, false, 0, 0x20000000); // "Reserve"
    }
}

/// The type of the current list.
pub(crate) unsafe fn finder_type(s: usize) -> u8 {
    read_u8(finder_current(s) + finder_list::TYPE)
}

/// `get_scout` of the current list's PLAYER_SEARCH.
pub(crate) unsafe fn finder_scout(s: usize) -> usize {
    get_scout(finder_current(s) + finder_list::SEARCH)
}

/// The CRT's sprintf.
pub(crate) unsafe fn finder_format() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_CRT_SPRINTF)
}

/// `Display::add_area` on the current slot's Display.
pub(crate) unsafe fn finder_area(f: [u32; 11]) -> u32 {
    tel26_add_area(finder_display(), f)
}

/// A popup area of the screen's menus (flags `flags`, the popup colour), owned by `owner`.
pub(crate) unsafe fn finder_popup(flags: u32, owner: u32) -> u32 {
    finder_area([0, 0, 0, 0, 0, 0, 0, 0, flags, job_colour(JOB_COLOUR_31AA), owner])
}

/// An entry of a popup menu (kind 2, or 0x2002 ticked): the text, enabled (1) or greyed (0x21), the id and the value.
pub(crate) unsafe fn finder_entry(kind: u32, enabled: bool, text: usize, id: u32, value: u32, area: u32) -> u32 {
    finder_item([kind, 0, 0, 0, 0, 0, 0, 0x10, job_colour(JOB_COLOUR_31AA), job_colour(JOB_COLOUR_BDEC), if enabled { 1 } else { 0x21 }, 1,
        job_colour(JOB_COLOUR_BDEA), text as u32, 0, id, value, area])
}

/// The screens' buffer as an enabled entry with `id`/`value`, or greyed without them.
pub(crate) unsafe fn finder_entry_if(enabled: bool, id: u32, value: u32, area: u32) -> u32 {
    if enabled { finder_entry(2, true, JOB_TEXT, id, value, area) } else { finder_entry(2, false, JOB_TEXT, 0, 0, area) }
}

/// The separator line of a popup menu.
pub(crate) unsafe fn finder_separator(area: u32) -> u32 {
    finder_item([1, 0, 0, 0, 0, 0, 0, 0x1000010, job_colour(JOB_COLOUR_31AA), job_colour(JOB_COLOUR_BDEC), 1, 1, job_colour(JOB_COLOUR_BDEA),
        0, 0, 0, 0, area])
}

/// A ticked option of the View menus: the translated `text`, ticked when `bit` is set in the view flags, id 0x12.
pub(crate) unsafe fn finder_view_option(text: usize, bit: u32, area: u32) {
    finder_lang()(JOB_TEXT, text);
    let kind = if read_u32(read_usize(FINDER_VIEW_FLAGS)) & bit != 0 { 0x2002 } else { 2 };
    finder_entry(kind, true, JOB_TEXT, 0x12, bit, area);
}

/// `HUMAN_MANAGER::get_password(manager)`: the manager's password (0 = the current human).
pub(crate) unsafe fn finder_password(manager: usize) -> usize {
    finder_call1(FINDER_GET_PASSWORD, read_usize(G_HUMAN_MANAGER), manager as u32) as usize
}

/// The 11 fields of `ScreenManager::add_text_item` (0x803fd0).
pub(crate) unsafe fn finder_text_item(f: [u32; 11]) -> u32 {
    let g: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
        core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    g(G_SCREEN_MANAGER, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10])
}

/// Club, nation and competition `id` of the game's tables.
pub(crate) unsafe fn finder_club(id: i32) -> usize {
    (read_usize(G_CLUB_TABLE) as isize + id as isize * club::SIZE as isize) as usize
}

pub(crate) unsafe fn finder_nation(id: i32) -> usize {
    (read_usize(G_NATION_TABLE) as isize + id as isize * nation::SIZE as isize) as usize
}

pub(crate) unsafe fn finder_comp(id: i32) -> usize {
    (read_usize(G_CLUB_COMP_TABLE) as isize + id as isize * FINDER_COMP_SIZE as isize) as usize
}

/// Copies the C string at `src` into `dst` (strcpy).
pub(crate) unsafe fn finder_copy(dst: &mut [u8], src: usize) {
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

/// The scout line of a scout list (see `finder_build_heading`).
pub(crate) unsafe fn finder_build_scout_heading(b: &mut FinderBuild, cur: usize) {
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
        let n = Nation(finder_nation(nation as i32));
        finder_copy(&mut place, n.addr() + nation::SHORT_NAME);
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

/// The header row of the view and the list area under it; returns the list area (the Filters bar's when no view flag is
/// set). The header area is one row high (the row height: the band divided by the rows, rounded) from `top` + 8.
pub(crate) unsafe fn finder_build_header(b: &mut FinderBuild, list_area: u32) -> u32 {
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

/// The first cell of a row: for a short list or a quick search, the person's national squad (A, B, U23 or U21, a link
/// to the national team) when both he and the human have a national team; for a scout list following a club, the
/// matches watched (a link 0x7e7) or nothing; for another scout list, the recommendation stars (by the filters' flag
/// mask: the first or the second rating, or the better one; five stars in yellow); empty for staff.
pub(crate) unsafe fn finder_row_first(b: &FinderBuild, r: &mut FinderRow) {
    let lang = finder_lang();
    let (p, s) = (r.person, b.s);
    let cur = finder_current(s);
    let dark = job_colour(JOB_COLOUR_BE04);
    let grey = job_colour(JOB_COLOUR_3184);
    match read_u8(cur + finder_list::TYPE) {
        1 | 2 => {
            let nt = Club(read_usize(p + staff::NATIONAL_JOB));
            if read_usize(b.mgr + staff::NATIONAL_JOB) == 0 || nt.is_null() {
                finder_cell(r, 0, [0x30, dark, 0xc, 1], grey, 0xad9c64);
                return;
            }
            let side: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FINDER_NATIONAL_SIDE);
            let (id, text) = match side(read_usize(FINDER_NATIONAL_TEAMS), p) {
                0 => (0x7d5, 0xa7e144usize), // "A<%s - COMMENT A = Nation main squad>"
                1 => (0x7d6, 0xa7e11c), // "B<%s - COMMENT B = Nation 'B' squad>"
                _ => {
                    let n = nt.nation().addr();
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
pub(crate) unsafe fn finder_row_status(r: &FinderRow) {
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
pub(crate) unsafe fn finder_row_name(b: &FinderBuild, r: &mut FinderRow) {
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
pub(crate) unsafe fn finder_row_view(b: &FinderBuild, r: &mut FinderRow) {
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
            let mine = Club(read_usize(b.mgr + staff::CLUB_JOB));
            if div != 0 && !mine.is_null() && read_usize(div + 0x5d) == mine.nation().addr() {
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

/// The last cell of a row (column 0xc): staff the reputation band of the result entry (Superb ≥ 0x251c, Very Good ≥
/// 0x1f40, Good ≥ 0x1770, Ok ≥ 0xea6, Unproven under 35, Poor ≥ 0x1f4, Very Poor), players the value (`Cash`) when he
/// has a club and a value, "Free" otherwise; in pick mode a pick button (0x1b) in the pick column.
pub(crate) unsafe fn finder_row_value(b: &FinderBuild, r: &FinderRow, pick_area: u32) {
    let lang = finder_lang();
    let p = Staff(r.person);
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
        } else if (p.age() as i8) < 0x23 {
            0xa7dfd8 // "Unproven<..>"
        } else if rep >= 0x1f4 {
            0xa14fe4 // "Poor<..>"
        } else {
            0xa14fb8 // "Very Poor<..>"
        };
        lang(JOB_TEXT, text, 0xad9c64usize);
    } else if r.club != 0 && (p.value() as i32) != 0 {
        let mut cash = [0u8; 8];
        let new_cash: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FINDER_CASH_NEW);
        let c = new_cash(cash.as_mut_ptr() as usize, p.value() as i32);
        let fmt: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FINDER_CASH_FORMAT);
        fmt(c, JOB_TEXT, 0x7d0, 5);
    } else {
        lang(JOB_TEXT, 0x98e614usize); // "Free"
    }
    finder_cell(r, 0xc, [0x30, job_colour(JOB_COLOUR_3192), 0xc, 1], job_colour(JOB_COLOUR_3184), JOB_TEXT);
    if b.pick != 0 {
        tel26_area_button([pick_area, 0, r.row as u32, 1, 0, 0xc, 1, job_colour(JOB_COLOUR_3184), 0, 0x1b, p.addr() as u32]);
    }
}

/// The club pick of the scout's "Club" assignment: the human's club's nation, else his nation when it is a playable
/// key nation (+0x11c), else the first playable key nation, else the default nation; the club pick screen (tab 4)
/// of that nation's continent opens.
pub(crate) unsafe fn finder_open_club_pick(st: &FinderInput) -> i32 {
    let mut n = finder_nation(read_i32(FINDER_DEFAULT_NATION));
    let club = Club(read_usize(st.mgr + staff::CLUB_JOB));
    let own = read_usize(st.mgr + staff::NATION);
    if !club.is_null() {
        n = club.nation().addr();
    } else if own != 0 && read_u8(own + 0x11c) != 0 {
        n = own;
    } else {
        for k in 0..0x23i16 {
            let e = read_usize((G_LEAGUE_TABLE as isize + k as isize * 0x48) as usize);
            if e != 0 && read_u8(e + 0x11c) != 0 {
                n = e;
                break;
            }
        }
    }
    let f: unsafe extern "C" fn(u32, usize, usize, u32) = core::mem::transmute(FINDER_OPEN_CLUB_PICK);
    f(4, read_usize(n + nation::CONTINENT), n, 0);
    FINDER_OPENED
}

/// The save dialog of the list's kind ("Enter File Name", mode 1, pending 0x1f): the default name is the list's name,
/// except for a short list not from a file (empty).
pub(crate) unsafe fn finder_save_dialog(st: &mut FinderInput) -> i32 {
    let k = st.kind();
    let default = if k == 0 && read_u32(finder_current(st.s) + finder_list::FROM_FILE) == 0 { 0xad9c64 } else { st.np() };
    st.file_dialog(finder_text::SAVE[k], finder_text::ENTER_FILE_NAME, 1, default, 0x1f)
}

/// Loads the recent file of param 4 (kind 2) into the current list: a box when it is locked by another human or does
/// not load; `None` when it loaded.
pub(crate) unsafe fn finder_load_recent(st: &mut FinderInput) -> Option<i32> {
    let recent: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FINDER_GET_RECENT_FILE);
    let path = st.path.as_mut_ptr() as usize;
    recent(read_usize(G_HUMAN_MANAGER), get_screen_param(4) as u32, path, 0x104, 0, 0, 2);
    if st.locked(path) {
        return Some(st.file_box(path, finder_text::LOCKED_LOAD, true, finder_text::UNABLE_LOAD, finder_text::LOAD));
    }
    if finder_call1(FINDER_LOAD_FILE, st.s, path as u32) == 0 {
        return Some(st.file_box(path, finder_text::NOT_LOADED, false, finder_text::UNABLE_LOAD, finder_text::LOAD));
    }
    None
}

/// The scouted player slot of a result's person in the scouted club records (`find_club_search_entry`), 0 when no scout
/// follows his club.
pub(crate) unsafe fn search_result_scouted_slot(entry: usize) -> usize {
    find_club_search_entry(read_usize(G_SCOUT_MANAGER), search_result_person(entry).addr())
}

/// A thiscall on a scouted player slot returning a short (the match sums).
pub(crate) unsafe fn search_slot_sum(f: usize, slot: usize) -> i16 {
    let g: unsafe extern "thiscall" fn(usize) -> i16 = core::mem::transmute(f);
    g(slot)
}

/// `crt_sprintf(out, fmt, a)`.
pub(crate) unsafe fn filters_format(out: usize, fmt: usize, a: usize) {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    f(out, fmt, a);
}

/// The index of a one-bit filter mask (`search_screens_filter_bit_to_index`), sign-extended.
pub(crate) unsafe fn filters_bit_index(lo: u32, hi: u32) -> u32 {
    let f: unsafe extern "C" fn(u32, u32) -> i8 = core::mem::transmute(FILTERS_FN_BIT_TO_INDEX);
    f(lo, hi) as i32 as u32
}

/// `Display::add_item` on the current human's display.
pub(crate) unsafe fn filters_item(f: DisplayItemFields) -> u32 {
    tel26_add_item(current_human_display(), f)
}

/// A row heading: the shared text in column 0 (`add_area_text(area, 0, row, 1, 0, 1, 3, colour 0xae3154, text)`).
pub(crate) unsafe fn filters_heading(area: u32, row: u32) {
    tel26_area_text([area, 0, row, 1, 0, 1, 3, job_colour(FILTERS_COLOUR_3154), FILTERS_TEXT as u32]);
}

/// The text of a chosen option at `col`: a yellow item (flags 6, style 0x221) with `id` and `value`.
pub(crate) unsafe fn filters_chosen(area: u32, col: u32, row: u32, id: u32, value: u32) {
    let y = job_colour(JOB_COLOUR_BDEC);
    filters_item([6, 0, 0, 0, 0, col, row, 0x221, y, y, 0xc, 2, y, FILTERS_TEXT as u32, 0, id, value, area]);
}

/// An option that can be clicked at `col`: a green button with `id` and `value`.
pub(crate) unsafe fn filters_option_button(area: u32, col: u32, row: u32, id: u32, value: u32) {
    tel26_area_button([area, col, row, 1, 0, 0xc, 2, job_colour(FILTERS_STATIC_GREEN), FILTERS_TEXT as u32, id, value]);
}

/// The usual option: id 10 with the index of its flag bit; chosen (`set`) it is the yellow item, otherwise the green
/// button.
pub(crate) unsafe fn filters_option(area: u32, col: u32, row: u32, set: bool, lo: u32, hi: u32) {
    let index = filters_bit_index(lo, hi);
    if set {
        filters_chosen(area, col, row, 10, index);
    } else {
        filters_option_button(area, col, row, 10, index);
    }
}

/// The Nationality row: Any (low 1), the manager's nation's three-letter code (low 2) and Other: plain yellow text when
/// a nation, a region, EU National or Basque is chosen (low 4..0x20), else a button for low 0x40. With one of those
/// chosen, a menu button at column 5 opens: EU National (the key nation is an EU member) and Basque (the manager's club
/// is Basque) with a separator, the key nations whose level byte is above 8 (by nationality adjective, id 0xb), a
/// separator and "Region" opening the regions sorted by name (id 0xc). The chosen nation (link 0x7d5 to its national
/// side), region, "  EU National" or "  Basque" is shown at column 4.
pub(crate) unsafe fn filters_screen_nationality_row(b: &FiltersBuild, row: u32) {
    covered!("tel21_5 filters nationality row");
    let c = job_colour;
    let f = b.filters;
    let area = b.nationality;
    filters_text(0x98edb8); // " Nationality"
    filters_heading(area, row);
    filters_text1(0xa80070, FILTERS_NO_COMMENT); // "Any<..nationality>"
    filters_option(area, 1, row, read_u32(f) & 1 != 0, 1, 0);
    filters_copy_text(b.nation + nation::THREE_LETTER_NAME);
    filters_option(area, 2, row, read_u32(f) & 2 != 0, 2, 0);
    filters_text(0xa80068); // "Other"
    if read_u32(f) & 0x3c == 0 {
        filters_option_button(area, 3, row, 10, filters_bit_index(0x40, 0));
    } else {
        tel26_area_text([area, 3, row, 0x221, c(JOB_COLOUR_BDEC), 0xc, 2, c(JOB_COLOUR_BDEC), FILTERS_TEXT as u32]);
    }
    if read_u32(f) & 0x3c == 0 {
        return;
    }
    let menu = filters_menu_button(area, 5, row);
    let eu = read_u8(b.nation + FILTERS_NATION_EU) == 2;
    if eu {
        filters_text(0xa8005c); // "EU National"
        filters_menu_entry(FILTERS_TEXT, 10, filters_bit_index(4, 0), menu);
    }
    let club = b.manager.club_job();
    if !club.is_null() && exe::database_is_basque_club(club) != 0 {
        filters_text(0xa80054); // "Basque"
        filters_menu_entry(FILTERS_TEXT, 10, filters_bit_index(8, 0), menu);
        filters_menu_separator(menu);
    } else if eu {
        filters_menu_separator(menu);
    }
    let order: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FILTERS_FN_KEY_NATION_SORTED_ORDER);
    let list = order(0);
    let mut buf = [0u8; 0xc8];
    for k in 0..FILTERS_KEY_NATION_COUNT {
        let n = read_usize((G_LEAGUE_TABLE as isize + read_i8(list + k) as isize * FILTERS_KEY_NATION_SIZE as isize) as usize);
        if n == 0 || read_i8(n + FILTERS_NATION_LEVEL) <= 8 {
            continue;
        }
        if read_u32(f) & 0x10 != 0 && read_u32(f + search_filters_block::NATIONALITY_NATION) == read_u32(n) {
            filters_format(FILTERS_TEXT, 0xa78880, 0); // "  "
            exe::nation_get_nationality_adjective(n, FILTERS_TEXT + 2, 0x7d0, 0xc);
            let team = exe::database_get_national_team(n);
            tel26_area_button([area, 4, row, 1, 0, 1, 2, c(JOB_COLOUR_BDEC), FILTERS_TEXT as u32, 0x7d5, team as u32]);
        }
        exe::nation_get_nationality_adjective(n, buf.as_mut_ptr() as usize, 0xc8, 0xc);
        filters_menu_entry(buf.as_ptr() as usize, 0xb, read_u32(n), menu);
    }
    filters_menu_separator(menu);
    filters_text(0xa7e80c); // "Region"
    let entry = filters_menu_entry(FILTERS_TEXT, 0, 0, menu);
    let regions_menu = filters_menu(0x30, entry);
    let sorted: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FILTERS_FN_REGIONS_SORTED_BY_NAME);
    let regions = sorted(0xc);
    for k in 0..0xe {
        let region = read_i8(regions + k);
        exe::get_region_name(region as i32 as u32, buf.as_mut_ptr() as usize, 200, 0xc);
        if read_u32(f) & 0x20 != 0 && read_i8(f + search_filters_block::NATIONALITY_REGION) == region {
            filters_format(FILTERS_TEXT, 0x987ff0, buf.as_ptr() as usize); // "  %s"
            filters_choice(area, 4, row);
        }
        filters_menu_entry(buf.as_ptr() as usize, 0xc, region as i32 as u32, regions_menu);
    }
    if read_u32(f) & 4 != 0 {
        filters_text(0xa80044); // "  EU National"
        filters_choice(area, 4, row);
    } else if read_u32(f) & 8 != 0 {
        filters_text(0xa80038); // "  Basque"
        filters_choice(area, 4, row);
    }
}

/// The Age row: Any (low 0x80) and the bands: a staff search 0-30, 30-40, 40-50, 50-60, 60+ (low 0x2000..0x20000), the
/// others 0-21, 19-25, 20-30, 25-32, 30+ (low 0x100..0x1000).
pub(crate) unsafe fn filters_screen_age_row(b: &FiltersBuild, row: u32) {
    let f = b.filters;
    let area = b.age;
    filters_text(0x98eda8); // " Age"
    filters_heading(area, row);
    filters_text1(0xa80020, FILTERS_NO_COMMENT); // "Any<..age>"
    filters_option(area, 1, row, read_u32(f) & 0x80 != 0, 0x80, 0);
    let bands: [(usize, u32); 5] = if b.staff {
        [(0xa80018, 0x2000), (0xa80010, 0x4000), (0xa80008, 0x8000), (0xa80000, 0x10000), (0xa7fffc, 0x20000)]
    } else {
        [(0xa7fff4, 0x100), (0xa7ffec, 0x200), (0xa7ffe4, 0x400), (0xa7ffdc, 0x800), (0xa7ffd8, 0x1000)]
    };
    for (k, &(fmt, bit)) in bands.iter().enumerate() {
        filters_text(fmt);
        filters_option(area, 2 + k as u32, row, read_u32(f) & bit != 0, bit, 0);
    }
}

/// The Language row: Any (low 0x40000; chosen it is the yellow item without id) and the key nation's languages (the
/// first always, the second and third when between 1 and 0x40), each chosen when low 0x80000 is set and the filters'
/// language is its number (1..3), else a button with id 0xd and the number.
pub(crate) unsafe fn filters_screen_language_row(b: &FiltersBuild, row: u32) {
    let f = b.filters;
    let area = b.age;
    filters_text(0xa7ffcc); // " Language"
    filters_heading(area, row);
    filters_text1(0xa7ffac, FILTERS_NO_COMMENT); // "Any<..language>"
    if read_u32(f) & 0x40000 == 0 {
        filters_option_button(area, 1, row, 10, filters_bit_index(0x40000, 0));
    } else {
        filters_chosen(area, 1, row, 0, 0);
    }
    let name: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FILTERS_FN_LANGUAGE_NAME);
    for (k, &field) in FILTERS_NATION_LANGUAGES.iter().enumerate() {
        let language = read_i8(b.nation + field);
        if k > 0 && !(language > 0 && language < 0x41) {
            continue;
        }
        filters_copy_text(name(language as u8 as u32));
        let n = k as u32 + 1;
        if read_u32(f) & 0x80000 == 0 || read_i8(f + search_filters_block::LANGUAGE) as i32 != n as i32 {
            filters_option_button(area, 2 + k as u32, row, 0xd, n);
        } else {
            filters_chosen(area, 2 + k as u32, row, 0, 0);
        }
    }
}

/// The Based row: Any (low 0x100000), the manager's nation's three-letter code (0x200000), Abroad (0x400000) and Other
/// (plain yellow text when a based nation or region is chosen, low 0x800000 / 0x1000000, else a button for 0x2000000).
/// With one of those chosen, a menu button at column 6 opens the key nations above level 8 (by short name, id 0xe), a
/// separator and "Region" with the regions (id 0xf); the chosen nation (link 0x7d5) or region is shown at column 5.
pub(crate) unsafe fn filters_screen_based_row(b: &FiltersBuild, row: u32) {
    covered!("tel21_5 filters based row");
    let c = job_colour;
    let f = b.filters;
    let area = b.based;
    filters_text(0xa7ffa4); // " Based"
    filters_heading(area, row);
    filters_text1(0xa7ff88, FILTERS_NO_COMMENT); // "Any<..based>"
    filters_option(area, 1, row, read_u32(f) & 0x100000 != 0, 0x100000, 0);
    filters_copy_text(b.nation + nation::THREE_LETTER_NAME);
    filters_option(area, 2, row, read_u32(f) & 0x200000 != 0, 0x200000, 0);
    filters_text(0xa7ff80); // "Abroad"
    filters_option(area, 3, row, read_u32(f) & 0x400000 != 0, 0x400000, 0);
    filters_text(0xa80068); // "Other"
    if read_u32(f) & 0x800000 == 0 && read_u32(f) & 0x1000000 == 0 {
        filters_option_button(area, 4, row, 10, filters_bit_index(0x2000000, 0));
    } else {
        tel26_area_text([area, 4, row, 0x221, c(JOB_COLOUR_BDEC), 0xc, 2, c(JOB_COLOUR_BDEC), FILTERS_TEXT as u32]);
    }
    if read_u32(f) & 0x800000 == 0 && read_u32(f) & 0x1000000 == 0 {
        return;
    }
    let menu = filters_menu_button(area, 6, row);
    let order: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FILTERS_FN_KEY_NATION_SORTED_ORDER);
    let list = order(0);
    for k in 0..FILTERS_KEY_NATION_COUNT {
        let n = Nation(read_usize((G_LEAGUE_TABLE as isize + read_i8(list + k) as isize * FILTERS_KEY_NATION_SIZE as isize) as usize));
        if n.is_null() || read_i8(n.addr() + FILTERS_NATION_LEVEL) <= 8 {
            continue;
        }
        if read_u32(f) & 0x800000 != 0 && read_u32(f + search_filters_block::BASED_NATION) == read_u32(n.addr()) {
            filters_format(FILTERS_TEXT, 0x987ff0, n.addr() + nation::SHORT_NAME); // "  %s"
            let team = exe::database_get_national_team(n);
            tel26_area_button([area, 5, row, 1, 0, 1, 2, c(JOB_COLOUR_BDEC), FILTERS_TEXT as u32, 0x7d5, team as u32]);
        }
        filters_menu_entry(n.addr() + nation::SHORT_NAME, 0xe, read_u32(n.addr()), menu);
    }
    filters_menu_separator(menu);
    filters_text(0xa7e80c); // "Region"
    let entry = filters_menu_entry(FILTERS_TEXT, 0, 0, menu);
    let regions_menu = filters_menu(0x30, entry);
    let sorted: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FILTERS_FN_REGIONS_SORTED_BY_NAME);
    let regions = sorted(u32::MAX);
    let mut buf = [0u8; 0xc8];
    for k in 0..0xe {
        let region = read_i8(regions + k);
        exe::get_region_name(region as i32 as u32, buf.as_mut_ptr() as usize, 200, u32::MAX);
        if read_u32(f) & 0x1000000 != 0 && read_i8(f + search_filters_block::BASED_REGION) == region {
            filters_format(FILTERS_TEXT, 0x987ff0, buf.as_ptr() as usize); // "  %s"
            filters_choice(area, 5, row);
        }
        filters_menu_entry(buf.as_ptr() as usize, 0xf, region as i32 as u32, regions_menu);
    }
}

/// The Job row of a staff search: Any (high 0x800), Specified (high 0x1000) and, when specified, a menu button at
/// column 4 with Manager, Assistant, Coach, Scout, Physio, Player and Unemployed (id 0x10, values 3..9); the chosen job
/// ("  %s") at column 3. DEFECT of the original kept apart: with a job value outside 3..9 the shown text is an
/// uninitialised stack buffer (here empty).
pub(crate) unsafe fn filters_screen_job_row(b: &FiltersBuild, row: u32) {
    covered!("tel21_5 filters job row");
    let f = b.filters;
    let area = b.job;
    filters_text(0xa7ff78); // " Job"
    filters_heading(area, row);
    filters_text1(0xa7ff60, FILTERS_NO_COMMENT); // "Any<..job>"
    filters_option(area, 1, row, read_u32(f + 4) & 0x800 != 0, 0, 0x800);
    filters_text(0xa80090); // "Specified"
    filters_option(area, 2, row, read_u32(f + 4) & 0x1000 != 0, 0, 0x1000);
    if read_u32(f + 4) & 0x1000 == 0 {
        return;
    }
    let menu = filters_menu_button(area, 4, row);
    let mut chosen = [0u8; 0xc8];
    let jobs: [(usize, u32); 7] = [(0x9b6440, 3), (0xa7ff54, 4), (0x9b6448, 5), (0x9b67d8, 6), (0x9b6710, 7), (0x9ca078, 8), (0x9b6b04, 9)];
    for &(fmt, value) in jobs.iter() {
        filters_text(fmt);
        filters_menu_entry(FILTERS_TEXT, 0x10, value, menu);
        if read_i8(f + search_filters_block::JOB) as i32 == value as i32 {
            tel26_copy_text(chosen.as_mut_ptr(), FILTERS_TEXT as *const u8);
        }
    }
    filters_format(FILTERS_TEXT, 0x987ff0, chosen.as_ptr() as usize); // "  %s"
    filters_choice(area, 3, row);
}

/// The Value row of a player search: Any (low 0x4000000), Range (low 0x8000000) and, with a range, a menu button at
/// column 4 with the value bands from 7 down to -1 (id 0x11): -1 "Free", below 7 "0 - <amount of preset band>", 7
/// "<amount of preset band 6> +" (`Cash::set_from_preset_index`, `cash_get_display_amount(5)`, `cash_format_money`);
/// the chosen band ("  %s") at column 3.
pub(crate) unsafe fn filters_screen_value_row(b: &FiltersBuild, row: u32) {
    covered!("tel21_5 filters value row");
    let f = b.filters;
    let area = b.job;
    filters_text(0x98ed54); // " Value"
    filters_heading(area, row);
    filters_text1(0xa7ff10, FILTERS_NO_COMMENT); // "Any<..value>"
    filters_option(area, 1, row, read_u32(f) & 0x4000000 != 0, 0x4000000, 0);
    filters_text(0xa7ff08); // "Range"
    filters_option(area, 2, row, read_u32(f) & 0x8000000 != 0, 0x8000000, 0);
    if read_u32(f) & 0x8000000 == 0 {
        return;
    }
    let menu = filters_menu_button(area, 4, row);
    let preset: unsafe extern "thiscall" fn(*mut u32, u32) = core::mem::transmute(FILTERS_FN_CASH_SET_FROM_PRESET);
    let mut cash = [0u32; 2];
    let mut amount = [0u8; 0xc8];
    let mut band_text = [0u8; 0xc8];
    let mut band: i8 = 7;
    loop {
        if band == -1 {
            let t: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
            t(band_text.as_mut_ptr() as usize, 0x98e614); // "Free"
        } else {
            let (index, fmt) = if band < 7 { (band, 0xa7ff00) } else { (band - 1, 0xa7fef8) }; // "0 - %s" / "%s +"
            preset(cash.as_mut_ptr(), index as u8 as u32);
            let value = exe::cash_get_display_amount(cash.as_ptr(), 5);
            exe::cash_format_money(amount.as_mut_ptr() as usize, 0xc8, value, 5);
            filters_format(band_text.as_mut_ptr() as usize, fmt, amount.as_ptr() as usize);
        }
        if band == read_i8(f + search_filters_block::VALUE_BAND) {
            filters_format(FILTERS_TEXT, 0x987ff0, band_text.as_ptr() as usize); // "  %s"
            filters_choice(area, 3, row);
        }
        filters_menu_entry(band_text.as_ptr() as usize, 0x11, band as i32 as u32, menu);
        band -= 1;
        if band < -1 {
            break;
        }
    }
}

/// `Display::add_item` on the current slot's Display.
pub(crate) unsafe fn finder_item(f: [u32; 18]) -> u32 {
    tel26_add_item(finder_display(), f)
}

/// A header button of the result columns (grey, pale grey text, style 0xc or 1 for the left-aligned ones), id 0x1e and
/// the sort bit.
pub(crate) unsafe fn finder_header(area: u32, column: u32, align: u32, bit: u32) {
    tel26_area_button([area, column, 0, 0x30, job_colour(JOB_COLOUR_317E), align, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0x1e, bit]);
}

/// The first header column (sorting by it): for a short list or a quick search of a human with a national team "Sqd"
/// (0x80000); for a scout list "Mat" (matches watched, 0x800000; `commented` adds the translator's comment) when it
/// follows a club, otherwise "Rec" (1); nothing for staff.
pub(crate) unsafe fn finder_first_header(b: &FinderBuild, area: u32, commented: bool) {
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
pub(crate) unsafe fn finder_inf_name_headers(area: u32) {
    let lang = finder_lang();
    lang(JOB_TEXT, 0x98ea64usize); // "Inf"
    finder_header(area, 1, 0xc, 2);
    lang(JOB_TEXT, 0x98ea5cusize); // "   Name"
    finder_header(area, 2, 1, 4);
}

/// The last column: "Rep." for staff (0x200), "Value" for players (0x100).
pub(crate) unsafe fn finder_value_header(b: &FinderBuild, area: u32) {
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
pub(crate) unsafe fn finder_attribute_headers(b: &mut FinderBuild, area: u32) {
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

/// A text cell of the list (`ScreenManager::add_area_text`): column, the four style fields, the colour and the text.
pub(crate) unsafe fn finder_cell(r: &FinderRow, col: u32, a: [u32; 4], colour: u32, text: usize) -> u32 {
    tel26_area_text([r.area, col, r.row as u32, a[0], a[1], a[2], a[3], colour, text as u32])
}

/// A button cell of the list (`ScreenManager::add_area_button`).
pub(crate) unsafe fn finder_cell_button(r: &FinderRow, col: u32, a: [u32; 4], colour: u32, text: usize, id: u32, value: usize) -> u32 {
    tel26_area_button([r.area, col, r.row as u32, a[0], a[1], a[2], a[3], colour, text as u32, id, value as u32])
}

/// The contract of a person (`CONTRACT_MANAGER::get_contract`), stored in `G_CONTRACT_LOOKUP_RESULT` when `keep`.
pub(crate) unsafe fn finder_contract(p: usize, keep: bool) -> usize {
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
    if keep {
        write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    }
    c
}

/// The club of a person's contract (0 without one), the lookup stored.
pub(crate) unsafe fn finder_contract_club(p: usize) -> usize {
    let c = finder_contract(p, true);
    if c == 0 { 0 } else { finder_club(read_i32(c + 4)) }
}

/// Length of the C string at `s`.
pub(crate) unsafe fn finder_strlen(s: usize) -> usize {
    let mut n = 0;
    while read_u8(s + n) != 0 {
        n += 1;
    }
    n
}

/// `strcpy` into the shared text.
pub(crate) unsafe fn filters_copy_text(src: usize) {
    tel26_copy_text(FILTERS_TEXT as *mut u8, src as *const u8);
}

/// A popup menu attached to item `owner` (`Display::add_area` with only the flags and the menu colour).
pub(crate) unsafe fn filters_menu(flags: u32, owner: u32) -> u32 {
    tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, flags, job_colour(JOB_COLOUR_31AA), owner])
}

/// One entry of a popup menu (`Display::add_item` flags 2, style 0x10): the text, the id and its value.
pub(crate) unsafe fn filters_menu_entry(text: usize, id: u32, value: u32, menu: u32) -> u32 {
    let c = job_colour;
    filters_item([2, 0, 0, 0, 0, 0, 0, 0x10, c(JOB_COLOUR_31AA), c(JOB_COLOUR_BDEC), 1, 1, c(JOB_COLOUR_BDEA), text as u32, 0, id, value, menu])
}

/// A separator line of a popup menu.
pub(crate) unsafe fn filters_menu_separator(menu: u32) {
    let c = job_colour;
    filters_item([1, 0, 0, 0, 0, 0, 0, 0x1000010, c(JOB_COLOUR_31AA), c(JOB_COLOUR_BDEC), 1, 1, c(JOB_COLOUR_BDEA), 0, 0, 0, 0, menu]);
}

/// A shown choice: the shared text in yellow at `col` (`add_area_text(.., 1, 0, 1, 2, yellow, text)`).
pub(crate) unsafe fn filters_choice(area: u32, col: u32, row: u32) {
    tel26_area_text([area, col, row, 1, 0, 1, 2, job_colour(JOB_COLOUR_BDEC), FILTERS_TEXT as u32]);
}

/// The button that opens a popup menu at `col` (`add_area_button(.., 0x30, black, 0xc, 1, pale grey, no text, 0, 0)`)
/// and the menu (flags 0x500030) attached to it.
pub(crate) unsafe fn filters_menu_button(area: u32, col: u32, row: u32) -> u32 {
    let c = job_colour;
    let b = tel26_area_button([area, col, row, 0x30, c(JOB_COLOUR_BDEA), 0xc, 1, c(JOB_COLOUR_3184), 0, 0, 0]);
    filters_menu(0x500030, b)
}

/// "E:\dev\CM3\cm3 00-01\cm3\code\finder_screens.cpp" (the error box).
pub(crate) const FINDER_SCREENS_CPP: usize = 0xa7e840;

/// The search screen's global pointers: the view flags of the current list (`SearchEditSession` + 0x294 of the current
/// list, read by the column comparers) and the manager the screen was built for.
pub(crate) const FINDER_VIEW_FLAGS: usize = 0xdd6cc8;

pub(crate) const FINDER_VIEW_MANAGER: usize = 0xdd6ccc;

/// The attribute columns of the Attributes view (bytes, 0xff ends): the fixed lists of each attribute group (by the view
/// flag: 0x80, 0x100, 0x200, 0x400, 0x800; the group names are the menu's) and the
/// "Filtered" list the screen builds from the filter minimums (9 bytes and the 0xff end, 0xa7dfcc..0xa7dfd5).
pub(crate) const FINDER_COLUMNS_STAFF: usize = 0xa7dfc0;

pub(crate) const FINDER_COLUMNS_PHYSICAL: usize = 0xa7df84;

pub(crate) const FINDER_COLUMNS_MENTAL: usize = 0xa7dfb4;

pub(crate) const FINDER_COLUMNS_GOALKEEPING: usize = 0xa7df90;

pub(crate) const FINDER_COLUMNS_DEFENSIVE: usize = 0xa7df9c;

pub(crate) const FINDER_COLUMNS_ATTACKING: usize = 0xa7dfa8;

pub(crate) const FINDER_COLUMNS_FILTERED: usize = 0xa7dfcc;

/// The statistics columns of the Stats view (bytes, 0xff ends).
pub(crate) const FINDER_COLUMNS_STATS: usize = 0xa7df6c;

pub(crate) const FINDER_COLUMNS_MORE_STATS: usize = 0xa7df78;

/// The byte that cuts the list name copied to the screens' buffer at 0x7cf characters.
pub(crate) const FINDER_TEXT_END: usize = 0xde2733;

/// The column comparers of the result list (finder_screens.cpp, ported elsewhere or still the original's), passed to
/// `SearchEditSession::get_current_results`.
pub(crate) const FINDER_SORT_RECORD: usize = 0x818ab0;

/// The SearchEditSession methods still called by address (finder_edit_session.cpp).
pub(crate) const FINDER_SESSION_GET_CURRENT_RESULTS: usize = 0x804a40;

pub(crate) const FINDER_SESSION_NEW: usize = 0x8041c0;

/// `SCOUT_MANAGER::collect_current_duty_comps(list)` (0x7f7010, thiscall ret 4).
pub(crate) const FINDER_COLLECT_DUTY_COMPS: usize = 0x7f7010;

/// `HUMAN_MANAGER::get_password(manager)` (0x5fde40, thiscall ret 4, char*) and `get_recent_file(index, out, size,
/// strip folder, manager, kind)` (0x5fc760, thiscall ret 0x18).
pub(crate) const FINDER_GET_PASSWORD: usize = 0x5fde40;

pub(crate) const FINDER_GET_RECENT_FILE: usize = 0x5fc760;

/// `key_nation_sorted_order(by_second_name)` (0x66a0c0, cdecl, the 35 sorted key nation indices).
pub(crate) const FINDER_KEY_NATION_ORDER: usize = 0x66a0c0;

/// `get_region_article_form(region)` (0x617780, cdecl, al).
pub(crate) const FINDER_REGION_ARTICLE_FORM: usize = 0x617780;

/// `gui_add_page_buttons(right, y, page, pages, first, previous, next, last)` (0x5ede20, cdecl).
pub(crate) const FINDER_PAGE_BUTTONS: usize = 0x5ede20;

/// `Text::draw(slot, left, right, &line, font, colour, 3, 0)` (0x4aeaf0, thiscall ret 0x20).
pub(crate) const FINDER_TEXT_DRAW: usize = 0x4aeaf0;

/// `ScreenManager::add_area(..)` (0x4567f0, 11 dwords, thiscall ret 0x2c) and `add_item(..)` (0x45cb60, 18 dwords).
pub(crate) const FINDER_SM_ADD_AREA: usize = 0x4567f0;

pub(crate) const FINDER_SM_ADD_ITEM: usize = 0x45cb60;

/// What the pieces of the search screen's builder share: the edit session (param 0), the human it is built for, the
/// title colours (params 1 and 2, as dwords), the pick mode (param 5: the screen picks a person and shows Cancel and
/// OK), the copy of the current list's filters, the result list and its counts, the free band left by the tab bar
/// (`top`, `bottom`), the tab area and the page buttons' left edge.
pub(crate) struct FinderBuild {
    pub(crate) s: usize,
    pub(crate) mgr: usize,
    pub(crate) fg: u32,
    pub(crate) bg: u32,
    pub(crate) pick: usize,
    pub(crate) filters: [u8; 0xa0],
    pub(crate) results: usize,
    pub(crate) shown: u32,
    /// The total found, later the attribute column list of the Attributes view (one variable in the original).
    pub(crate) l14: usize,
    pub(crate) top: i32,
    pub(crate) bottom: i32,
    pub(crate) tab_area: u32,
    pub(crate) page_left: i32,
    /// The rows of a page plus one (low byte) and the row height.
    pub(crate) rows: u8,
    pub(crate) row_height: i32,
    /// The screen's Text object (the last match lines are drawn through it).
    pub(crate) text: usize,
}

/// A tab of `gui_add_tab_bar` (0xd4 bytes: +2 the colour, +4 the text, +0xcc the kind, +0xd0 the link).
pub(crate) const FINDER_TAB_SIZE: usize = 0xd4;

impl FinderBuild {
    /// The right edge of the texts above the list: 5 left of the page buttons, or 0x30c without them.
    pub(crate) fn right(&self) -> u32 {
        if self.page_left != 0 { (self.page_left - 5) as u32 } else { 0x30c }
    }
    /// The 0x7d7 (person) button of the scout line, in the heading colour.
    pub(crate) unsafe fn scout_line(&self, scout: usize) {
        tel26_add_button([0x6e, self.top as u32, self.right(), (self.top + 0x14) as u32, 2, 0, 0xc, 1, job_colour(JOB_COLOUR_BDE8), JOB_TEXT as u32, 0x7d7, scout as u32]);
    }
    /// The screens' buffer as the text line above the list, in the heading colour.
    pub(crate) unsafe fn heading_line(&self) {
        finder_text_item([0x6e, self.top as u32, self.right(), (self.top + 0x14) as u32, 2, 0, 0xc, 1, job_colour(JOB_COLOUR_BDE8), JOB_TEXT as u32, 0]);
    }
}

/// The size of a club competition record (`G_CLUB_COMP_TABLE`).
pub(crate) const FINDER_COMP_SIZE: usize = 0x6b;

/// Colours of the status cells (words): suspended and injured (0xaebdcc), long injured (0xae3190), the transfer status
/// box (0xae317c), and the name of a person at another club than his contract's (0xae3152).
pub(crate) const FINDER_COLOUR_SUSPENDED: usize = 0xaebdcc;

pub(crate) const FINDER_COLOUR_LONG_INJURY: usize = 0xae3190;

pub(crate) const FINDER_COLOUR_STATUS_BOX: usize = 0xae317c;

pub(crate) const FINDER_COLOUR_OTHER_CLUB: usize = 0xae3152;

/// The competition objects (`G_COMP_OBJECT_TABLE`-like table at 0xadadfc: one pointer per competition id; a division
/// with an object is a link).
pub(crate) const FINDER_COMP_OBJECTS: usize = 0xadadfc;

/// `get_player_status_code(person, 0, 0)` (0x4849f0, cdecl, al).
pub(crate) const FINDER_PLAYER_STATUS_CODE: usize = 0x4849f0;

/// STAFF_CONTRACT: `format_wage_text(out, style, cash flags)` (0x85e750, ret 0xc), `format_end_date_text(out)`
/// (0x85e8f0, ret 4), `get_release_clause_abbreviation(out, size)` (0x868020, ret 8, al).
pub(crate) const FINDER_CONTRACT_WAGE: usize = 0x85e750;

pub(crate) const FINDER_CONTRACT_END: usize = 0x85e8f0;

pub(crate) const FINDER_CONTRACT_RELEASES: usize = 0x868020;

/// PlayerStats: `get_field_label(field, out, size)` (0x7c1b20), `format_player_stat(player, 6, field, out, size, 1, 0,
/// 1)` (0x7c25b0) and `format_recent_ratings(player, out, size)` (0x7c1660), on [`G_PLAYER_STATS`].
pub(crate) const FINDER_STAT_LABEL: usize = 0x7c1b20;

pub(crate) const FINDER_STAT_FORMAT: usize = 0x7c25b0;

pub(crate) const FINDER_RECENT_RATINGS: usize = 0x7c1660;

/// The scout report slot: goals, assists, goals conceded (0x7bd870, 0x7bd8c0, 0x7bd910, ax) and the average rating
/// (0x7bd960, st(0)); `format_venue_marker(venue, out, size)` (0x7e4ca0, cdecl); `Date::get_short_date_string(out,
/// size, time)` (0x54a650, ret 0xc).
pub(crate) const FINDER_SLOT_GOALS: usize = 0x7bd870;

pub(crate) const FINDER_SLOT_ASSISTS: usize = 0x7bd8c0;

pub(crate) const FINDER_SLOT_CONCEDED: usize = 0x7bd910;

pub(crate) const FINDER_SLOT_AVERAGE: usize = 0x7bd960;

pub(crate) const FINDER_VENUE_MARKER: usize = 0x7e4ca0;

pub(crate) const FINDER_SHORT_DATE: usize = 0x54a650;

/// `Notes::find_note(manager, person)` (0x795750, ret 8) and `notes_get_reason_label(out, size, reason, style)`
/// (0x795b90, stdcall), on the notes at [0xb7433c]; `contract_manager_write_morale_text(person, out, size)` (0x4cc2e0)
/// on [`G_CONTRACT_MANAGER`]; `NATIONAL_TEAMS::class_by_national_side(person)` (0x76e070) on [0xb74318].
pub(crate) const FINDER_NOTES: usize = 0xb7433c;

pub(crate) const FINDER_FIND_NOTE: usize = 0x795750;

pub(crate) const FINDER_NOTE_REASON: usize = 0x795b90;

pub(crate) const FINDER_MORALE_TEXT: usize = 0x4cc2e0;

pub(crate) const FINDER_NATIONAL_TEAMS: usize = 0xb74318;

pub(crate) const FINDER_NATIONAL_SIDE: usize = 0x76e070;

/// `SearchFilters::get_attribute_minimum(attribute, is_non_player)` (0x808190, thiscall ret 8, al).
pub(crate) const FINDER_ATTRIBUTE_MINIMUM: usize = 0x808190;

/// The comparers `qsort` uses for the attribute columns (staff, players).
pub(crate) const FINDER_SORT_STAFF_COLUMNS: usize = 0x81af00;

pub(crate) const FINDER_SORT_PLAYER_COLUMNS: usize = 0x81ae60;

/// `Cash::Cash(value)` (0x450a90) and `Cash::format_display_amount(out, size, flags)` (0x450aa0, ret 0xc).
pub(crate) const FINDER_CASH_NEW: usize = 0x450a90;

pub(crate) const FINDER_CASH_FORMAT: usize = 0x450aa0;

pub(crate) const FINDER_COLUMNS: FinderColumns = FinderColumns {
    general: [6, 5, 0x16, 0x14, 0x14, 0xe, 0, 0, 0, 0, 0, 0, 0xa],
    contract: [6, 5, 0x16, 0xe, 0xe, 0xe, 0xc, 0, 0, 0, 0, 0, 0xa],
    stats: [6, 5, 0x16, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0xa],
    attributes: [6, 5, 0x16, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0xa],
    scout: [6, 5, 0x16, 6, 6, 6, 6, 0x1e, 0, 0, 0, 0, 0xa],
    notes: [2, 2, 0xa, 0, 6, 0, 0x10],
    other: [6, 5, 0x16, 8, 6, 6, 6, 0xa, 0xc, 6, 0, 0, 0xa],
};

/// One result row being written: the list area, the row, the person, the result entry, the club the person is shown
/// with (a dword that survives from the previous row: the international-duty status reads it before it is updated),
/// the scout entry (lists following a club) and the B team of the contract's club.
pub(crate) struct FinderRow {
    pub(crate) area: u32,
    pub(crate) row: i32,
    pub(crate) person: usize,
    pub(crate) entry: usize,
    pub(crate) club: usize,
    pub(crate) slot: usize,
    pub(crate) b_team: usize,
}

/// `SearchFilters::set_defaults` (0x806450, thiscall).
pub(crate) const FINDER_FILTERS_SET_DEFAULTS: usize = 0x806450;

pub(crate) const FINDER_CLOSED: i32 = -9;

pub(crate) const FINDER_REDRAW: i32 = -11;

/// [0xb77c84]: a box or a file dialog has answered (the pending command and its argument are set).
pub(crate) const FINDER_PENDING: usize = 0xb77c84;

/// The default nation (id) of the club pick when the human has none.
pub(crate) const FINDER_DEFAULT_NATION: usize = 0x9cf2e4;

/// SearchEditSession methods (search_edit_session.cpp), thiscall: revert the current list (0x8042e0), save all
/// (0x804450), set the current list (0x8044a0, ret 4), unlock / lock (0x804750 / 0x804810), set the scout's scope
/// (0x8048e0, ret 0x14), reset the scout fields (0x8049a0), remove a player (0x804ab0, ret 4), clear the results
/// (0x804b60, ret 4), load a file (0x804cd0, ret 4), save the current list (0x804e10, ret 8), the destructor (0x813140).
pub(crate) const FINDER_REVERT: usize = 0x8042e0;

pub(crate) const FINDER_SAVE_ALL: usize = 0x804450;

pub(crate) const FINDER_SET_CURRENT: usize = 0x8044a0;

pub(crate) const FINDER_RESET_SCOUT: usize = 0x8049a0;

pub(crate) const FINDER_REMOVE_PLAYER: usize = 0x804ab0;

pub(crate) const FINDER_CLEAR_RESULTS: usize = 0x804b60;

pub(crate) const FINDER_LOAD_FILE: usize = 0x804cd0;

pub(crate) const FINDER_SAVE_FILE: usize = 0x804e10;

pub(crate) const FINDER_SESSION_DROP: usize = 0x813140;

/// `PLAYER_SEARCH::set_assigned_club(club)` (0x7b3d50, ret 4); `search_screens_open_filters_screen(session)`
/// (0x813210, cdecl); `open_choose_friendly_teams_screen(tab, continent, nation, kind)` (0x5a0650, cdecl);
/// `notes_open_staff_note_screen(person)` (0x7961b0, cdecl); `open_club_squad_screen(club, reserves, restore, alt,
/// locked)` (0x453d60, cdecl); `staff_screens_open_staff_profile(person, tab, slot2, keep, mode)` (0x8702a0, cdecl).
pub(crate) const FINDER_SET_ASSIGNED_CLUB: usize = 0x7b3d50;

pub(crate) const FINDER_OPEN_FILTERS: usize = 0x813210;

pub(crate) const FINDER_OPEN_CLUB_PICK: usize = 0x5a0650;

pub(crate) const FINDER_OPEN_NOTE: usize = 0x7961b0;

pub(crate) const FINDER_OPEN_SQUAD: usize = 0x453d60;

pub(crate) const FINDER_OPEN_PROFILE: usize = 0x8702a0;

/// What the answer's pieces share: the session, the human, the box colours (params 1 and 2), params 3 (lock on save),
/// 5 (pick mode), 6 (the person to delete) and 7 (pages), the clicked item, and the buffers of the boxes (the list's
/// name, the file header, the file name, the box title and heading, the scout's name, the place and the path).
pub(crate) struct FinderInput {
    pub(crate) s: usize,
    pub(crate) mgr: usize,
    pub(crate) c1: u32,
    pub(crate) c2: u32,
    pub(crate) lock_owner: usize,
    pub(crate) pick: usize,
    pub(crate) pending_person: usize,
    pub(crate) pages: i16,
    pub(crate) item: i16,
    pub(crate) name: [u8; 0xc8],
    pub(crate) header: [u8; 0x170],
    pub(crate) file: [u8; 0x104],
    pub(crate) title: [u8; 0xc8],
    pub(crate) heading: [u8; 0xc8],
    pub(crate) scout: [u8; 0xc8],
    pub(crate) place: [u8; 0xc8],
    pub(crate) path: [u8; 0x104],
}

impl FinderInput {
    pub(crate) fn np(&self) -> usize {
        self.name.as_ptr() as usize
    }
    pub(crate) fn sp(&self) -> usize {
        self.scout.as_ptr() as usize
    }
    pub(crate) fn pp(&self) -> usize {
        self.place.as_ptr() as usize
    }
    pub(crate) fn fp(&self) -> usize {
        self.file.as_ptr() as usize
    }
    pub(crate) fn creator(&self) -> usize {
        self.header.as_ptr() as usize + FINDER_HEADER_CREATOR
    }
    /// The kind of the current list as an index of the texts (1, 2, 3; anything else is a scout search).
    pub(crate) unsafe fn kind(&self) -> usize {
        match finder_type(self.s) {
            1 => 0,
            2 => 1,
            3 => 2,
            _ => 3,
        }
    }
    /// The current list's PLAYER_SEARCH.
    pub(crate) unsafe fn search(&self) -> usize {
        finder_current(self.s) + finder_list::SEARCH
    }
    /// The current list was saved to a file.
    pub(crate) unsafe fn saved(&self) -> bool {
        read_u32(finder_current(self.s) + finder_list::SAVED) != 0
    }
    /// The clicked item's id: its id when it has one, otherwise the current command id.
    pub(crate) unsafe fn id(&self) -> u16 {
        let id = if self.item != -1 { read_u16(finder_item_at(self.item) + display_item::ID) } else { 0 };
        if id != 0 { id } else { read_u16(G_CURRENT_COMMAND_ID) }
    }
    /// The clicked item's value (0 for none).
    pub(crate) unsafe fn value(&self) -> usize {
        if self.item != -1 { read_usize((finder_display() as isize + display::ITEM_VALUE as isize + self.item as isize * display_list::ITEM_SIZE as isize) as usize) } else { 0 }
    }
    /// A box (`confirm_push`): the screens' buffer as the text, the heading and the title translated into their
    /// buffers (in that order), `buttons` (1 = yes/no) and the pending `code` its answer sets; the answer opens it.
    pub(crate) unsafe fn confirm(&mut self, title: usize, heading: usize, buttons: u32, code: u32) -> i32 {
        let lang = finder_lang();
        let h = lang(self.heading.as_mut_ptr() as usize, heading) as usize;
        let t = lang(self.title.as_mut_ptr() as usize, title) as usize;
        let f: unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(FINDER_CONFIRM);
        f(t, h, JOB_TEXT, buttons, self.c1, self.c2, code);
        FINDER_OPENED
    }
    /// The file dialog of the list's kind (`open_file_dialog`: the human's folder of kind 2, ".pls", the kind's check),
    /// in `mode` (0 load, 1 save, 2 delete) with the default name and the pending `code`.
    pub(crate) unsafe fn file_dialog(&mut self, title: usize, heading: usize, mode: u32, default: usize, code: u32) -> i32 {
        let lang = finder_lang();
        let folder: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FINDER_FOLDER_PATH);
        let dir = folder(read_usize(G_HUMAN_MANAGER), 0, 2);
        let h = lang(self.heading.as_mut_ptr() as usize, heading) as usize;
        let t = lang(self.title.as_mut_ptr() as usize, title) as usize;
        let f: unsafe extern "C" fn(usize, usize, u32, usize, usize, usize, usize, u32, u32, u32, u32, u32, u32, u32) =
            core::mem::transmute(FINDER_FILE_DIALOG);
        f(t, h, mode, dir, finder_text::EXTENSION, default, FINDER_FILE_CHECKS[self.kind()], self.c1, self.c2, code, 0, 0, 0, 0);
        FINDER_OPENED
    }
    /// The file at `path` is locked by another human: its header (kind 4) has a password that is not the current
    /// human's.
    pub(crate) unsafe fn locked(&mut self, path: usize) -> bool {
        let read: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FINDER_READ_FILE_HEADER);
        read(path, self.header.as_mut_ptr() as usize, 4);
        let pw = self.header.as_ptr() as usize + FINDER_HEADER_PASSWORD;
        read_u8(pw) != 0 && !finder_same_text(finder_password(0), pw)
    }
    /// The file box of the list's kind: the file name copied, the text of the kind with the file (and the creator),
    /// "Unable To .." and the kind's title, no buttons.
    pub(crate) unsafe fn file_box(&mut self, path: usize, texts: [usize; 4], creator: bool, heading: usize, titles: [usize; 4]) -> i32 {
        finder_copy(&mut self.file, path);
        let k = self.kind();
        if creator {
            finder_lang()(JOB_TEXT, texts[k], self.fp(), self.creator());
        } else {
            finder_lang()(JOB_TEXT, texts[k], self.fp());
        }
        self.confirm(titles[k], heading, 0, 0)
    }
    /// The pending file name is freed and the argument cleared (after a file box opened by a dialog's answer).
    pub(crate) unsafe fn drop_path(&self, path: usize) -> i32 {
        exe::crt_operator_delete(path);
        write_u32(G_PENDING_COMMAND_ARG, 0);
        FINDER_OPENED
    }
    /// The current list's lock toggled (unlocked when locked, locked otherwise).
    pub(crate) unsafe fn toggle_lock(&self) {
        let f = if read_u8(finder_current(self.s) + finder_list::LOCKED) != 0 { FINDER_UNLOCK } else { FINDER_LOCK };
        finder_call0(f, self.s);
    }
    /// `SearchEditSession::set_search_scope(nation, region, competition, youth, next opposition)`.
    pub(crate) unsafe fn scope(&self, nation: u32, region: u32, comp: u32, youth: u32, next: u32) {
        let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(FINDER_SET_SCOPE);
        f(self.s, nation, region, comp, youth, next);
    }
    /// The scout's name (style 1) into the scout buffer.
    pub(crate) unsafe fn scout_name(&mut self, scout: usize, style: u32) {
        exe::database_format_staff_name(scout, self.sp(), 0xc8, style);
    }
}

/// `player_search_slot_get_match_entry` (0x7bd830, thiscall(slot, char index) -> the match entry).
pub(crate) const SEARCH_FN_SLOT_GET_MATCH_ENTRY: usize = 0x7bd830;

/// The scouted player slot sums (thiscall(slot) -> short): goals, assists, goals conceded.
pub(crate) const SEARCH_FN_SLOT_SUM_GOALS: usize = 0x7bd870;

pub(crate) const SEARCH_FN_SLOT_SUM_ASSISTS: usize = 0x7bd8c0;

pub(crate) const SEARCH_FN_SLOT_SUM_GOALS_CONCEDED: usize = 0x7bd910;

/// `player_search_slot_average_rating` (0x7bd960, thiscall(slot) -> double in st0).
pub(crate) const SEARCH_FN_SLOT_AVERAGE_RATING: usize = 0x7bd960;

/// `national_teams_get_player_squad_slot` (0x76e070, thiscall(national teams, person) -> char, -1 outside the squad).
pub(crate) const SEARCH_FN_NATIONAL_SQUAD_SLOT: usize = 0x76e070;

/// The search settings of the screen being sorted (`g_search_session_settings_ptr`): +8 the statistic column, +9 the
/// attribute column.
pub(crate) const SEARCH_SETTINGS: usize = 0xdd6cc8;

pub(crate) const SEARCH_SETTINGS_STAT: usize = 0x8;

pub(crate) const SEARCH_SETTINGS_ATTRIBUTE: usize = 0x9;

/// The person who looks at the search (`g_search_screen_manager_staff`): what he can see of the attributes.
pub(crate) const SEARCH_VIEWER: usize = 0xdd6ccc;

/// `database_format_player_positions` (0x53ee50, cdecl(player, buffer, size, style)).
pub(crate) const SEARCH_FN_FORMAT_PLAYER_POSITIONS: usize = 0x53ee50;

/// The national job code that does not count as a job (the employer and job columns skip it).
pub(crate) const SEARCH_NATIONAL_JOB_NONE: u8 = 0xb;

/// The job key of a person without one (sorted last).
pub(crate) const SEARCH_NO_JOB: u8 = 0x64;

/// The reputation of an established person (3750): a person under 35 below it is ranked at it, one exactly at it just
/// above.
pub(crate) const SEARCH_REPUTATION_ESTABLISHED: i16 = 0xea6;

/// `contract_manager_get_player_morale` (0x4cc2d0, thiscall(contract manager, person) -> char).
pub(crate) const SEARCH_FN_PLAYER_MORALE: usize = 0x4cc2d0;

/// `STAFF_CONTRACT::get_release_clause_abbreviation` (0x868020, thiscall(contract, text, size) -> char code).
pub(crate) const SEARCH_FN_RELEASE_CLAUSE_ABBREVIATION: usize = 0x868020;

/// `database_get_player_attribute_name` (0x5428a0) and `database_get_non_player_attribute_name` (0x543b10):
/// cdecl(attribute, text, size, short).
pub(crate) const SEARCH_FN_PLAYER_ATTRIBUTE_NAME: usize = 0x5428a0;

pub(crate) const SEARCH_FN_NON_PLAYER_ATTRIBUTE_NAME: usize = 0x543b10;

/// "  -", the clause text both sides start with (the abbreviation overwrites it).
pub(crate) const SEARCH_RELEASE_CLAUSE_NONE: usize = 0x98e8c8;

/// `PLAYER_SEARCH::~PLAYER_SEARCH` (0x7ae4f0), the element destructor of the session's search arrays.
pub(crate) const SEARCH_FN_PLAYER_SEARCH_DESTRUCTOR: usize = 0x7ae4f0;

/// `SearchEditSession::save_all_slots` (0x804450, thiscall) and `SearchEditSession::set_current_filters` (0x804c10,
/// thiscall(session, filters) ret 4).
pub(crate) const SEARCH_FN_SAVE_ALL_SLOTS: usize = 0x804450;

pub(crate) const SEARCH_FN_SET_CURRENT_FILTERS: usize = 0x804c10;

/// `SearchFilters` methods: set_defaults 0x806450 (thiscall), reset_general_filters 0x8064d0, reset_attribute_filters
/// 0x8065d0, set_option_flag 0x806630 (thiscall(filters, lo, hi) ret 8 -> changed), the one-value setters
/// (thiscall(filters, value) ret 4), get_attribute_minimum 0x808190 (thiscall(filters, attribute, staff table) ret 8
/// -> al), set_attribute_minimum 0x808280 (thiscall(filters, attribute, minimum, staff table) ret 0xc),
/// increase/decrease_attributes_to_match 0x8084c0 / 0x808500 (thiscall(filters, staff table) ret 4).
pub(crate) const SEARCH_FN_FILTERS_SET_DEFAULTS: usize = 0x806450;

pub(crate) const SEARCH_FN_FILTERS_RESET_GENERAL: usize = 0x8064d0;

pub(crate) const SEARCH_FN_FILTERS_RESET_ATTRIBUTES: usize = 0x8065d0;

pub(crate) const SEARCH_FN_FILTERS_SET_OPTION_FLAG: usize = 0x806630;

pub(crate) const SEARCH_FN_FILTERS_SET_NATIONALITY_NATION: usize = 0x806ff0;

pub(crate) const SEARCH_FN_FILTERS_SET_NATIONALITY_REGION: usize = 0x807090;

pub(crate) const SEARCH_FN_FILTERS_SET_LANGUAGE_OPTION: usize = 0x807130;

pub(crate) const SEARCH_FN_FILTERS_SET_BASED_NATION: usize = 0x8071d0;

pub(crate) const SEARCH_FN_FILTERS_SET_BASED_REGION: usize = 0x807270;

pub(crate) const SEARCH_FN_FILTERS_SET_VALUE_BAND: usize = 0x807310;

pub(crate) const SEARCH_FN_FILTERS_SET_JOB_FILTER: usize = 0x8073b0;

pub(crate) const SEARCH_FN_FILTERS_GET_ATTRIBUTE_MINIMUM: usize = 0x808190;

pub(crate) const SEARCH_FN_FILTERS_SET_ATTRIBUTE_MINIMUM: usize = 0x808280;

pub(crate) const SEARCH_FN_FILTERS_INCREASE_TO_MATCH: usize = 0x8084c0;

pub(crate) const SEARCH_FN_FILTERS_DECREASE_TO_MATCH: usize = 0x808500;

/// `search_screens_filter_index_to_bit` (0x81b4e0, cdecl(char index) -> u64 mask in edx:eax).
pub(crate) const SEARCH_FN_FILTER_INDEX_TO_BIT: usize = 0x81b4e0;

/// `ScreenManager::get_widget_action_id` (0x468b40, thiscall(manager, item) ret 4).
pub(crate) const SEARCH_FN_GET_WIDGET_ACTION_ID: usize = 0x468b40;

/// The Filters screen's builder and input handler (the routines of tel21_5 and of this item).
pub(crate) const SEARCH_FN_BUILD_FILTERS_SCREEN: usize = 0x8133a0;

pub(crate) const SEARCH_FN_HANDLE_FILTERS_INPUT: usize = 0x818100;

/// search_screens.cpp (the error box path).
pub(crate) const SEARCH_SCREENS_CPP: usize = 0xa7e840;

/// The list type of a staff search (its attribute table is the non-player one).
pub(crate) const SEARCH_LIST_TYPE_STAFF: u8 = 3;

/// What the Filters input returns: stay and redraw (-11), leave the screen (-9).
pub(crate) const SEARCH_INPUT_REDRAW: i32 = -11;

pub(crate) const SEARCH_INPUT_LEAVE: i32 = -9;

/// The function-static colour of the Filters screen: bit 0 of the guard byte, then the pixel word
/// (`graphics_rgb_to_pixel(0, 0xc0, 0, 0)`), the colour of an option that is not chosen.
pub(crate) const FILTERS_STATIC_GUARD: usize = 0xdd6cc4;

pub(crate) const FILTERS_STATIC_GREEN: usize = 0xdd6cc6;

/// Colour words of the screen items not named in globals.rs (the headings, the minus/plus buttons).
pub(crate) const FILTERS_COLOUR_3150: usize = 0xae3150;

pub(crate) const FILTERS_COLOUR_3154: usize = 0xae3154;

/// The shared text buffer and the empty argument of the "<%s - COMMENT ...>" texts.
pub(crate) const FILTERS_TEXT: usize = 0xde1f64;

pub(crate) const FILTERS_NO_COMMENT: usize = 0xad9c64;

/// The default general flag mask of a search (`SearchFilters` +0 / +4): with it the Clear button is plain text.
pub(crate) const FILTERS_DEFAULT_MASK: [u32; 2] = [0x14140081, 0x44aaa884];

/// The search list types: a staff search, a match-watching scout's list.
pub(crate) const FILTERS_LIST_STAFF: u8 = 3;

pub(crate) const FILTERS_LIST_SCOUT: u8 = 4;

/// Functions called by address: `search_screens_filter_bit_to_index` (cdecl(lo, hi) -> char), `draw_pitch`,
/// `key_nation_sorted_order`, `get_regions_sorted_by_name`, `database_get_language_name`, `Cash::set_from_preset_index`.
pub(crate) const FILTERS_FN_BIT_TO_INDEX: usize = 0x81b0e0;

pub(crate) const FILTERS_FN_KEY_NATION_SORTED_ORDER: usize = 0x66a0c0;

pub(crate) const FILTERS_FN_REGIONS_SORTED_BY_NAME: usize = 0x545dc0;

pub(crate) const FILTERS_FN_LANGUAGE_NAME: usize = 0x53cee0;

pub(crate) const FILTERS_FN_CASH_SET_FROM_PRESET: usize = 0x43f620;

/// The key nations table (`G_LEAGUE_TABLE`, 0x48 bytes per key nation; +0 the nation): 0x23 entries.
pub(crate) const FILTERS_KEY_NATION_SIZE: usize = 0x48;

pub(crate) const FILTERS_KEY_NATION_COUNT: usize = 0x23;

/// A nation's byte the menus need above 8 to list it [inference: its database coverage level].
pub(crate) const FILTERS_NATION_LEVEL: usize = 0x85;

/// A nation's EU membership byte (2 = member; the "EU National" option) and its languages.
pub(crate) const FILTERS_NATION_EU: usize = 0x7f;

pub(crate) const FILTERS_NATION_LANGUAGES: [usize; 3] = [0x77, 0x78, 0x79];

/// The sort comparers of the attribute lists (tel21_3), by address as the original passes them.
pub(crate) const FILTERS_FN_SORT_STAFF_ATTRIBUTES: usize = 0x81b040;

pub(crate) const FILTERS_FN_SORT_PLAYER_ATTRIBUTES: usize = 0x81afa0;

/// `database_get_player_attribute_name` / `database_get_non_player_attribute_name` (cdecl(id, text, size, short)).
pub(crate) const FILTERS_FN_PLAYER_ATTRIBUTE_NAME: usize = 0x5428a0;

pub(crate) const FILTERS_FN_NON_PLAYER_ATTRIBUTE_NAME: usize = 0x543b10;

/// The screen being built: the filters, the session's current search, the key nation and the seven column layouts
/// that share the body rectangle.
pub(crate) struct FiltersBuild {
    pub(crate) filters: usize,
    pub(crate) staff: bool,
    pub(crate) list_type: u8,
    pub(crate) manager: Staff,
    pub(crate) nation: usize,
    /// The body areas (Display::add_area, low 16 bits): 2,1,1,1,1,1,1 / 8,4,4,4,7,1,4 / 8,4,4,4,4,7,1 / 8,4,4,7,1,4,4
    /// (7 columns), 6,1,1,1,1,6,1,1,1 / 6,1,1,1,1,5,1,2,1 (9 columns), 8,4,4,1,15,0,0.
    pub(crate) age: u32,
    pub(crate) nationality: u32,
    pub(crate) based: u32,
    pub(crate) job: u32,
    pub(crate) attributes: u32,
    pub(crate) attributes_match: u32,
    pub(crate) text: u32,
}

/// The SearchFilters fields of the "attributes to match" counters: for outfield players the maximum (+0x63) and the
/// count (+0x64); with the goalkeeper set (`which` ≠ 0) the maximum (+0x7d) and the count (+0x7e). Signed bytes.
pub(crate) const SEARCH_FILTERS_MAX: usize = 0x63;

pub(crate) const SEARCH_FILTERS_COUNT: usize = 0x64;

pub(crate) const SEARCH_FILTERS_GK_MAX: usize = 0x7d;

pub(crate) const SEARCH_FILTERS_GK_COUNT: usize = 0x7e;

/// The 65th search filter: two bits (bits 31 to 33 of the 64-bit mask: 0x3_8000_0000) for index 0x40.
pub(crate) const SEARCH_FILTER_INDEX_0X40: u64 = 0x3_8000_0000;

/// `player_search_read_file_header(path, out, kind)` (0x7bcee0, cdecl, bool).
pub(crate) const FINDER_READ_FILE_HEADER: usize = 0x7bcee0;

/// The list's columns: the column weights of each view and the header buttons.
pub(crate) struct FinderColumns {
    pub(crate) general: [u8; 13],
    pub(crate) contract: [u8; 13],
    pub(crate) stats: [u8; 13],
    pub(crate) attributes: [u8; 13],
    pub(crate) scout: [u8; 13],
    pub(crate) notes: [u8; 7],
    pub(crate) other: [u8; 13],
}

/// The answer's return codes: -4 a new screen or a box opened, -9 the screen closes, -11 redraw.
pub(crate) const FINDER_OPENED: i32 = -4;

/// `confirm_push(title, heading, text, buttons, text colour, colour, code)` (0x591840) and `open_file_dialog(title,
/// heading, mode, directory, extension, default name, on ok, colour a, colour b, code, 0, 0, 0, 0)` (0x58f500), cdecl.
pub(crate) const FINDER_CONFIRM: usize = 0x591840;

pub(crate) const FINDER_FILE_DIALOG: usize = 0x58f500;

/// The file dialog's checks of each list kind (short list, quick search, staff search, scout search).
pub(crate) const FINDER_FILE_CHECKS: [usize; 4] = [0x7bd010, 0x7bd0d0, 0x7bd190, 0x7bd250];

/// `HUMAN_MANAGER::get_folder_path(manager, kind)` (0x5fc5b0, thiscall ret 8).
pub(crate) const FINDER_FOLDER_PATH: usize = 0x5fc5b0;

pub(crate) const FINDER_UNLOCK: usize = 0x804750;

pub(crate) const FINDER_LOCK: usize = 0x804810;

pub(crate) const FINDER_SET_SCOPE: usize = 0x8048e0;

/// The header of a saved search file (`player_search_read_file_header`): the creator at +0x6b, the password at +0x134.
pub(crate) const FINDER_HEADER_CREATOR: usize = 0x6b;

pub(crate) const FINDER_HEADER_PASSWORD: usize = 0x134;
