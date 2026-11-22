// ---------- batch %%LEVA%%: the Search For screen and the friendly teams' other clubs input (find_screens.cpp) ----------

/// The search engine of a Search For tab (`SearchEngine`, 0x3c bytes): the found records (+0x34, an array of record
/// pointers) and their count (+0x38).
const SEARCH_ENGINE_SIZE: u32 = 0x3c;
const SEARCH_ENGINE_RESULTS: usize = 0x34;
const SEARCH_ENGINE_COUNT: usize = 0x38;
/// A continent record: its name (+4) and its federation's name (+0xa3), as the nations tab shows them [inference].
const CONTINENT_NAME: usize = 0x4;
const CONTINENT_FEDERATION_NAME: usize = 0xa3;
/// A tab bar entry (`MenuItem`, 0xd4 bytes): the text at +4, the kind at +0xcc and the value at +0xd0.
const FIND_TAB_ENTRY_SIZE: usize = 0xd4;
/// The size of a competition record of `G_CLUB_COMP_TABLE` (107 bytes).
const FIND_COMP_RECORD_SIZE: usize = 0x6b;

/// One of the four tabs of the Search For screen: its first parameter (`base`: the search text is `base` + 1, the
/// search engine + 2, the "search again" flag + 3, the first row shown + 4, the scroll row + 5, the list area + 6), the
/// edit box's kind and id, the search (`SearchEngine::find_clubs` 0x804fe0 or `find_staff` 0x805310) and its mode, and
/// the headings "<things> Matching ..." and "All <things>".
struct Tel34FindTab {
    base: u32,
    edit_kind: u32,
    edit_id: u32,
    search: usize,
    mode: u32,
    matching: usize,
    all: usize,
}

const FIND_NATIONS: Tel34FindTab = Tel34FindTab { base: 0, edit_kind: 0x880018, edit_id: 6, search: 0x804fe0, mode: 1, matching: 0x9ca0c8, all: 0x9ca0bc };
const FIND_CLUBS: Tel34FindTab = Tel34FindTab { base: 6, edit_kind: 0x480018, edit_id: 7, search: 0x804fe0, mode: 0, matching: 0x9ca12c, all: 0x9ca120 };
const FIND_NON_PLAYERS: Tel34FindTab = Tel34FindTab { base: 0xc, edit_kind: 0x2380018, edit_id: 8, search: 0x805310, mode: 1, matching: 0x9ca174, all: 0x9ca164 };
const FIND_PLAYERS: Tel34FindTab = Tel34FindTab { base: 0x12, edit_kind: 0x1380018, edit_id: 9, search: 0x805310, mode: 0, matching: 0x9ca1ac, all: 0x9ca1a0 };

/// `ScreenManager::add_text_item` (0x803fd0, thiscall, ret 0x2c): x1, y1, x2, y2, the style, the colour, the text
/// flags, the font, the text colour, the text and an unused dword.
unsafe fn tel34_text_item(f: [u32; 11]) -> u32 {
    type TextItem = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: TextItem = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    g(G_SCREEN_MANAGER, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10])
}

/// A cell of a result row in the list area: `add_area_text` in column `column` of row `row`, the result style
/// (1, 0, 1, 2) in the colour word at `colour`.
unsafe fn tel34_find_cell_text(area: u32, column: u32, row: u32, colour: usize, text: usize) {
    tel26_area_text([area, column, row, 1, 0, 1, 2, job_colour(colour), text as u32]);
}

/// A linked cell of a result row: `add_area_button` in column `column` of row `row` (style 1, 0, 1, `font`) with the
/// link id and object.
unsafe fn tel34_find_cell_link(area: u32, column: u32, row: u32, font: u32, colour: u32, text: usize, id: u32, object: usize) {
    tel26_area_button([area, column, row, 1, 0, 1, font, colour, text as u32, id, object as u32]);
}

/// The common body of the four result tabs (the routine's whole frame but the rows): the parameters, the search
/// engine, the "Search For" edit box, the search when asked, the page buttons, the heading, the list area and, for
/// each record shown, `row(area, row, record)`; then the scroll row and the area kept.
///
/// The search text = parameter `base` + 1 and the engine = + 2; parameter 0x19 is read and cleared. No engine yet:
/// one is made (`operator new` 0x3c bytes and `SearchEngine::SearchEngine`, or 0 when the allocation fails), kept in
/// + 2, and the "search again" flag + 3 set. The first row shown = + 4. An area from (0x6e, `top`) to (0x168, `top` +
/// 0x14) with "Search For" and the edit box (the tab's kind, the text at most 0x19 chars, the tab's id). With the flag:
/// the search (the text, the tab's mode) and the flag cleared. More than 100 found: the page buttons
/// (`gui_add_page_buttons`: right edge 0x30c at `top`, the page = first / 16 + 1, the pages = found / 16 rounded up,
/// ids 2..5) and at most 16 rows. The heading at `top` + 0x19: "<things> Matching <text>" or, with an empty text, "All
/// <things>". The list area from `top` + 0x41 to `bottom` (3 columns, weights 3/2/2, as many rows as fit in font 3 with
/// 16 pixels to spare), the rows from the first shown; the area scrolled to + 5, + 6 = the area.
unsafe fn tel34_find_results(tab: &Tel34FindTab, top: i32, bottom: i32, mut row: impl FnMut(u32, u32, usize)) {
    let weights: [u8; 3] = [3, 2, 2];
    let text = get_screen_param(tab.base + 1);
    let mut engine = get_screen_param(tab.base + 2);
    get_screen_param(0x19);
    set_screen_param(0x19, 0);
    if engine == 0 {
        let memory = exe::crt_operator_new(SEARCH_ENGINE_SIZE);
        engine = if memory != 0 {
            let construct: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x804f00usize);
            construct(memory)
        } else {
            0
        };
        set_screen_param(tab.base + 2, engine);
        set_screen_param(tab.base + 3, 1);
    }
    let start = get_screen_param(tab.base + 4) as i32;
    let edit_area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x168, top.wrapping_add(0x14) as u32, 2, 0, 1, 0, 1, 0, u32::MAX]);
    tel34_lang_text(0x9ca0f0); // "Search For"
    tel26_area_text([edit_area, 0, 0, 2, 0, 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);
    tel26_add_item(current_human_display(), [tab.edit_kind, 0, 0, 0, 0, 1, 0, 2, 0, 0, 0xc, 1, job_colour(JOB_COLOUR_BDE8), text as u32, 0x19,
        tab.edit_id, 0, edit_area]);
    if get_screen_param(tab.base + 3) != 0 {
        covered!("tel34_6 search");
        let search: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(tab.search);
        search(engine, text, tab.mode);
        set_screen_param(tab.base + 3, 0);
    }
    let count = read_i32(engine + SEARCH_ENGINE_COUNT);
    let paged = count > 100;
    if paged {
        covered!("tel34_6 page buttons");
        type PageButtons = unsafe extern "C" fn(i32, i32, i32, i32, u32, u32, u32, u32) -> i32;
        let pages: PageButtons = core::mem::transmute(0x5ede20usize);
        pages(0x30c, top, start / 16 + 1, count / 16 + (count % 16 != 0) as i32, 2, 3, 4, 5);
    }
    let lang: Tel34FormatFn = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if read_u8(text) != 0 {
        lang(JOB_TEXT, tab.matching, text);
    } else {
        lang(JOB_TEXT, tab.all);
    }
    let heading = top.wrapping_add(0x19);
    tel34_text_item([0x6e, heading as u32, 0x30c, heading.wrapping_add(0x23) as u32, 2, 0, 0xc, 3, job_colour(JOB_COLOUR_BDEC), JOB_TEXT as u32, 0]);
    let list_top = top.wrapping_add(0x41);
    let rows = (bottom.wrapping_sub(list_top).wrapping_sub(0x10) / exe::font_get_height(3)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, list_top as u32, 0x30c, bottom as u32, 3, weights.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    let results = read_usize(engine + SEARCH_ENGINE_RESULTS);
    let mut k: i32 = 0;
    while start.wrapping_add(k) < read_i32(engine + SEARCH_ENGINE_COUNT) {
        if paged && k >= 16 {
            break;
        }
        let record = read_usize(results.wrapping_add((start.wrapping_add(k) as usize).wrapping_mul(4)));
        row(area, k as u32, record);
        k += 1;
    }
    let scroll = get_screen_param(tab.base + 5);
    if area as i16 != -1 {
        exe::area_set_scroll_row(display_area_at(current_human_display(), area as i16), scroll);
    }
    set_screen_param(tab.base + 6, area as i16 as isize as usize);
}

/// The first cell of a club result row: the club's short name ("  %s") linked to the club (id 0x7d5) in the
/// highlight colour when a human manages it (`human_manager_is_human_club`, 0, 1, 0, 0), otherwise in the text colour.
unsafe fn tel34_find_club_name(area: u32, row: u32, club: usize) {
    let crt: Tel34FormatFn = core::mem::transmute(FN_CRT_SPRINTF);
    let human = exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0;
    crt(JOB_TEXT, 0x987ff0, club + club::SHORT_NAME); // "  %s"
    let colour = if human { job_colour(JOB_COLOUR_BDEC) } else { job_colour(JOB_COLOUR_3184) };
    tel34_find_cell_link(area, 0, row, 3, colour, JOB_TEXT, 0x7d5, club);
}

/// The division cell (column 2) of a club: the division's short name linked to it (id 2000) when the competition is
/// loaded (an object in [`G_COMP_OBJECT_TABLE`]), as plain text otherwise; "Unknown Division" without one. With
/// `lower_division`, a division whose nation's league is selected (`nation::LEAGUE_SELECTED`) but not loaded shows "A
/// lower division" (`KEY_CLUB_COMP_A_LOWER_DIVISION`'s short name).
unsafe fn tel34_find_division_cell(area: u32, row: u32, club: usize, lower_division: bool) {
    let division = read_usize(club + club::DIVISION);
    if division == 0 {
        tel34_lang_text(0x9ca0fc); // "Unknown Division"
        tel34_find_cell_text(area, 2, row, JOB_COLOUR_BDD4, JOB_TEXT);
        return;
    }
    let loaded = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_u32(division) as usize).wrapping_mul(4))) != 0;
    if lower_division {
        let nation = read_usize(division + competition::NATION);
        if nation != 0 && read_u8(nation + nation::LEAGUE_SELECTED) != 0 && !loaded {
            let generic = read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_i32(KEY_CLUB_COMP_A_LOWER_DIVISION) as usize).wrapping_mul(FIND_COMP_RECORD_SIZE));
            tel34_find_cell_text(area, 2, row, JOB_COLOUR_BDD4, generic + competition::SHORT_NAME);
            return;
        }
    }
    if loaded {
        tel34_find_cell_link(area, 2, row, 2, job_colour(JOB_COLOUR_BDD4), division + competition::SHORT_NAME, 2000, division);
    } else {
        tel34_find_cell_text(area, 2, row, JOB_COLOUR_BDD4, division + competition::SHORT_NAME);
    }
}

/// A staff result row: the person's name ("  %s", `database_format_staff_name` 2000 bytes, style 3) linked to him
/// (id 0x7d7), highlighted when his club is a human's; then his club's short name linked to it (id 0x7d5) and its
/// division (`tel34_find_division_cell`), or "No Club" and "No Division".
unsafe fn tel34_find_staff_row(area: u32, row: u32, person: usize, club: usize) {
    let crt: Tel34FormatFn = core::mem::transmute(FN_CRT_SPRINTF);
    let mut name = [0u8; 0x7d0];
    let human = club != 0 && exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0;
    exe::database_format_staff_name(person, name.as_mut_ptr() as usize, 0x7d0, 3);
    crt(JOB_TEXT, 0x987ff0, name.as_ptr()); // "  %s"
    let colour = if human { job_colour(JOB_COLOUR_BDEC) } else { job_colour(JOB_COLOUR_3184) };
    tel34_find_cell_link(area, 0, row, 3, colour, JOB_TEXT, 0x7d7, person);
    if club == 0 {
        tel34_lang_text(0x9ca15c); // "No Club"
        tel34_find_cell_text(area, 1, row, JOB_COLOUR_BDEC, JOB_TEXT);
        tel34_lang_text(0x9ca150); // "No Division"
        tel34_find_cell_text(area, 2, row, JOB_COLOUR_BDD4, JOB_TEXT);
        return;
    }
    tel34_find_cell_link(area, 1, row, 2, job_colour(JOB_COLOUR_BDEC), club + club::SHORT_NAME, 0x7d5, club);
    tel34_find_division_cell(area, row, club, false);
}

/// Rule: a click on the Other clubs tab of Choose Friendly Teams keeps the list's scroll row and, on a club (id 7)
/// other than the one kept, keeps it (-11, rebuild); otherwise 0.
///
/// For an item (not -1) parameter 0x11 = the scroll row of the area in parameter 0x12. Id 7 (or the command id): the
/// item's value (0 for -1) against parameter 0x10; different: parameter 0x10 = it, -11.
/// Original: `choose_friendly_teams_other_clubs_input` (0x5a2b30, find_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn choose_friendly_teams_other_clubs_input(raw: u32) -> i32 {
    let item = raw as i16;
    if item != -1 {
        let row = exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(0x12));
        set_screen_param(0x11, row);
    }
    if tel26_item_id(item) == 7 && tel26_item_value(item) as usize != get_screen_param(0x10) {
        covered!("tel34_6 friendly other club");
        set_screen_param(0x10, tel26_item_value(item) as usize);
        return -11;
    }
    0
}

/// Rule: opening the Search For screen on a tab (1 nations, 2 clubs, 3 non-players, 4 players) with every tab reset.
///
/// `open_screen` (builder 0x5a2e60, input 0x5a45c0, flag 0, leave `release_find_screen` 0x5a4eb0, no exit); when it
/// opens, parameter 0 = the tab (signed byte) and, for each tab's parameters (1, 7, 0xd, 0x13 onward): the search text
/// = a copy of 25 spaces (`set_screen_param` with copy) emptied at once (its first byte 0), no engine, "search again"
/// 1, first row, scroll and area 0; then parameter 0x19 = 1.
/// Original: `open_find_screen` (0x5a2c30, find_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_find_screen(tab: u32) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x5a2e60usize, 0x5a45c0usize, 0, 0x5a4eb0usize, 0) == 0 {
        return;
    }
    covered!("tel34_6 open find screen");
    set_screen_param(0, tab as u8 as i8 as isize as usize);
    let mut spaces = [0x20u8; 26];
    spaces[25] = 0;
    for base in [0u32, 6, 0xc, 0x12] {
        exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, base + 1, spaces.as_ptr() as usize, 1);
        write_u8(get_screen_param(base + 1), 0);
        set_screen_param(base + 2, 0);
        set_screen_param(base + 3, 1);
        set_screen_param(base + 4, 0);
        set_screen_param(base + 5, 0);
        set_screen_param(base + 6, 0);
    }
    set_screen_param(0x19, 1);
}

/// Rule: building the Search For screen: the title, the menu bar, the four tabs and the open tab's results.
///
/// 25 tab entries (`MenuItem`: word +0 = -1, word +2 = 0; the other bytes are stack leftovers in the original, zero
/// here); the tab = parameter 0 (byte); the game title (heading colours 0 and 1); the menu bar (4, 0); the entries
/// "Nation", "Club", "Non-Player" and "Player" (translated, kind 1, value 1..4); the tab bar (`gui_add_tab_bar`, 4
/// entries, from 0x50 to 0x221, split). Tab 1: the nation background (null) and the nations; 2: the club background
/// (null) and the clubs; 3 and 4: the staff background (null) and the non-players or players (the four result routines
/// called by their addresses, as the original); any other: a random background and an empty area (0x6e, top .. 0x30c,
/// bottom). Then the Back/Next buttons.
/// Original: `draw_find_screen` (0x5a2e60, find_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn draw_find_screen() {
    let mut entries = [[0u8; FIND_TAB_ENTRY_SIZE]; 0x19];
    for e in entries.iter_mut() {
        e[0] = 0xff;
        e[1] = 0xff;
    }
    covered!("tel34_6 find screen");
    let tab = get_screen_param(0) as u8 as i8;
    tel34_game_title();
    exe::menubar_build(4, 0);
    let lang: Tel34FormatFn = core::mem::transmute(FN_LANGLIB_SPRINTF);
    for (k, text) in [0x9ca010usize, 0x9ca08c, 0x9ca080, 0x9ca078].into_iter().enumerate() {
        // "Nation", "Club", "Non-Player", "Player"
        lang(entries[k][4..].as_mut_ptr() as usize, text);
        entries[k][0xcc..0xd0].copy_from_slice(&1u32.to_le_bytes());
        entries[k][0xd0..0xd4].copy_from_slice(&(k as u32 + 1).to_le_bytes());
    }
    let (mut top, mut bottom) = (0x50i32, 0x221i32);
    exe::gui_add_tab_bar(entries.as_mut_ptr() as usize, 4, tab as i32, &mut top, &mut bottom, 1);
    let picture = |address: usize| {
        let f: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(address);
        f(0, 0, 0);
    };
    // the result tabs by their exe addresses: the 1080 exe has its own (27 rows per page), left unhooked there
    let results = |address: usize| {
        let f: unsafe extern "C" fn(i32, i32) = core::mem::transmute(address);
        f(top, bottom);
    };
    match tab {
        1 => {
            picture(0x41c100);
            results(0x5a30f0);
        }
        2 => {
            picture(0x41c250);
            results(0x5a35b0);
        }
        3 => {
            picture(0x41c790);
            results(0x5a3af0);
        }
        4 => {
            picture(0x41c790);
            results(0x5a4060);
        }
        _ => {
            exe::background_set_random_picture();
            tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 1, 0, 1, 0, 2, 0, u32::MAX]);
        }
    }
    tel26_back_next();
}

/// Rule: the nations tab of the Search For screen: the national teams found (`SearchEngine::find_clubs` with national
/// teams), each with its continent and federation.
///
/// `tel34_find_results` with the nations tab; each row: the team (`tel34_find_club_name`), then, when its nation has a
/// continent (`nation::CONTINENT`), the continent's name (column 1, highlight colour) and its federation's (column 2),
/// otherwise "Unknown Continent" and "Unknown Federation".
/// Original: `draw_find_nations_results` (0x5a30f0, find_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn draw_find_nations_results(top: i32, bottom: i32) {
    covered!("tel34_6 nations");
    tel34_find_results(&FIND_NATIONS, top, bottom, |area, row, team| {
        tel34_find_club_name(area, row, team);
        let nation = read_usize(team + club::NATION);
        if nation != 0 && read_usize(nation + nation::CONTINENT) != 0 {
            let continent = read_usize(nation + nation::CONTINENT);
            tel34_find_cell_text(area, 1, row, JOB_COLOUR_BDEC, continent + CONTINENT_NAME);
            tel34_find_cell_text(area, 2, row, JOB_COLOUR_BDD4, continent + CONTINENT_FEDERATION_NAME);
        } else {
            tel34_lang_text(0x9ca0a8); // "Unknown Continent"
            tel34_find_cell_text(area, 1, row, JOB_COLOUR_BDEC, JOB_TEXT);
            tel34_lang_text(0x9ca094); // "Unknown Federation"
            tel34_find_cell_text(area, 2, row, JOB_COLOUR_BDD4, JOB_TEXT);
        }
    });
}

/// Rule: the clubs tab of the Search For screen: the clubs found, each with its nation and its division.
///
/// `tel34_find_results` with the clubs tab; each row: the club (`tel34_find_club_name`), its nation's name linked to
/// the nation's national team (id 0x7d5, `database_get_national_team`) or "Unknown Nation", and its division
/// (`tel34_find_division_cell` with "A lower division").
/// Original: `draw_find_clubs_results` (0x5a35b0, find_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn draw_find_clubs_results(top: i32, bottom: i32) {
    covered!("tel34_6 clubs");
    tel34_find_results(&FIND_CLUBS, top, bottom, |area, row, club| {
        tel34_find_club_name(area, row, club);
        let nation = read_usize(club + club::NATION);
        if nation != 0 {
            let team = exe::database_get_national_team(nation);
            tel34_find_cell_link(area, 1, row, 2, job_colour(JOB_COLOUR_BDEC), nation + 4, 0x7d5, team);
        } else {
            tel34_lang_text(0x9ca110); // "Unknown Nation"
            tel34_find_cell_text(area, 1, row, JOB_COLOUR_BDEC, JOB_TEXT);
        }
        tel34_find_division_cell(area, row, club, true);
    });
}

/// Rule: the non-players tab of the Search For screen: the staff found (`SearchEngine::find_staff`, non-players), each
/// with his club and its division.
///
/// `tel34_find_results` with the non-players tab; the club shown = his national job (`staff::NATIONAL_JOB`) when he
/// has one with `staff::JOB_FOR_NATION` 5 and `staff::JOB_FOR_CLUB` not 5, otherwise his club (`staff::CLUB_JOB`);
/// the row is `tel34_find_staff_row`.
/// Original: `draw_find_non_players_results` (0x5a3af0, find_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn draw_find_non_players_results(top: i32, bottom: i32) {
    covered!("tel34_6 non-players");
    tel34_find_results(&FIND_NON_PLAYERS, top, bottom, |area, row, person| {
        let national = read_usize(person + staff::NATIONAL_JOB);
        let club = if national != 0 && read_u8(person + staff::JOB_FOR_NATION) == 5 && read_u8(person + staff::JOB_FOR_CLUB) != 5 {
            national
        } else {
            read_usize(person + staff::CLUB_JOB)
        };
        tel34_find_staff_row(area, row, person, club);
    });
}

/// Rule: the players tab of the Search For screen: the players found (`SearchEngine::find_staff`, players), each with
/// his club (`staff::CLUB_JOB`) and its division (`tel34_find_staff_row`).
/// Original: `draw_find_players_results` (0x5a4060, find_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn draw_find_players_results(top: i32, bottom: i32) {
    covered!("tel34_6 players");
    tel34_find_results(&FIND_PLAYERS, top, bottom, |area, row, person| {
        tel34_find_staff_row(area, row, person, read_usize(person + staff::CLUB_JOB));
    });
}

/// The input of a result tab: for an item (not -1) the scroll row (+ 5) = the area's (+ 6); the tab's edit box id
/// (search) sets "search again" (+ 3) and clears the first row (+ 4) and the scroll (+ 5); the page ids move the first
/// row (+ 4): 2 back 160, 3 back 16 (below 0: up to the first page boundary at or above 0), 4 on 16, 5 on 160 (past
/// the last found: back by whole pages of 16 until it is at or before it). Each answers -11; anything else 0.
unsafe fn tel34_find_tab_input(raw: u32, base: u32, edit_id: u16) -> i32 {
    let engine = get_screen_param(base + 2);
    let start = get_screen_param(base + 4) as i32;
    let item = raw as i16;
    if item != -1 {
        let row = exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(base + 6));
        set_screen_param(base + 5, row);
    }
    let id = tel26_item_id(item);
    if id == edit_id {
        covered!("tel34_6 search again");
        set_screen_param(base + 3, 1);
        set_screen_param(base + 4, 0);
        set_screen_param(base + 5, 0);
        return -11;
    }
    let back = |s: i32| if s < 0 { s.wrapping_add((0xfu32.wrapping_sub(s as u32) >> 4 << 4) as i32) } else { s };
    let on = |s: i32| {
        let last = read_i32(engine + SEARCH_ENGINE_COUNT).wrapping_sub(1);
        if s > last { s.wrapping_sub((s.wrapping_sub(last).wrapping_add(0xf) as u32 >> 4 << 4) as i32) } else { s }
    };
    let next = match id {
        2 => back(start.wrapping_sub(0xa0)),
        3 => back(start.wrapping_sub(0x10)),
        4 => on(start.wrapping_add(0x10)),
        5 => on(start.wrapping_add(0xa0)),
        _ => return 0,
    };
    covered!("tel34_6 page");
    set_screen_param(base + 4, next as usize);
    -11
}

/// Rule: a click on the Search For screen: the open tab's input first (search, pages), then a tab button (id 1) with
/// another tab opens it (-11); otherwise the menu bar.
///
/// The tab = parameter 0 (byte): 1 the nations' input (edit id 6), 2 the clubs' (7), 3 `find_screen_non_players_input`,
/// 4 `find_screen_players_input` (`tel34_find_tab_input`); an answer other than 0 is returned. Then id 1: the item's
/// value against parameter 0; different: parameter 0 = it, -11. Otherwise `menubar_handle_command`, then
/// `menubar_handle_object_link` when it gives 0.
/// Original: `find_screen_input` (0x5a45c0, find_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn find_screen_input(raw: u32) -> i32 {
    let r = match get_screen_param(0) as u8 as i8 {
        1 => tel34_find_tab_input(raw, FIND_NATIONS.base, 6),
        2 => tel34_find_tab_input(raw, FIND_CLUBS.base, 7),
        3 => find_screen_non_players_input(raw) as i32,
        4 => find_screen_players_input(raw) as i32,
        _ => 0,
    };
    if r != 0 {
        return r;
    }
    let item = raw as i16;
    if tel26_item_id(item) as i16 == 1 && tel26_item_value(item) as usize != get_screen_param(0) {
        covered!("tel34_6 change tab");
        set_screen_param(0, tel26_item_value(item) as usize);
        return -11;
    }
    let r = exe::menubar_handle_command(raw);
    if r != 0 { r } else { exe::menubar_handle_object_link(raw) }
}

/// Rule: the input of the non-players tab (parameters 0xd..0x12, edit id 8): `tel34_find_tab_input`.
/// Original: `find_screen_non_players_input` (0x5a4950, find_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn find_screen_non_players_input(raw: u32) -> u32 {
    tel34_find_tab_input(raw, FIND_NON_PLAYERS.base, 8) as u32
}

/// Rule: the input of the players tab (parameters 0x13..0x18, edit id 9): `tel34_find_tab_input`.
/// Original: `find_screen_players_input` (0x5a4c00, find_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn find_screen_players_input(raw: u32) -> u32 {
    tel34_find_tab_input(raw, FIND_PLAYERS.base, 9) as u32
}

/// Rule: leaving the Search For screen frees the four tabs' search engines (parameters 2, 8, 0xe and 0x14).
///
/// Each one set: `SearchEngine::~SearchEngine` (0x804fd0) and `operator delete`, and the parameter cleared.
/// Original: `release_find_screen` (0x5a4eb0, find_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn release_find_screen() {
    for k in [2u32, 8, 0xe, 0x14] {
        if get_screen_param(k) == 0 {
            continue;
        }
        let engine = get_screen_param(k);
        if engine != 0 {
            let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x804fd0usize);
            destroy(engine);
            exe::crt_operator_delete(engine);
        }
        set_screen_param(k, 0);
    }
}

/// The parameters a find screen routine may write: the whole parameter block of the current screen (0x1a dwords).
unsafe fn tel34_find_params() -> [usize; 0x1a] {
    let mut p = [0usize; 0x1a];
    for (k, v) in p.iter_mut().enumerate() {
        *v = get_screen_param(k as u32);
    }
    p
}

/// Comparer for `choose_friendly_teams_other_clubs_input`: side by side over the screen parameters and the answer.
pub(crate) unsafe extern "C" fn compare_choose_friendly_teams_other_clubs_input(raw: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I0%%);
    let regions = screen_param_regions();
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(raw), || choose_friendly_teams_other_clubs_input(raw));
    SCOREBOARD.record(equal && ro == rr, || format!("Choose Friendly Teams (choose_friendly_teams_other_clubs_input): answer {} x {}, differences in {:?}",
        ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `open_find_screen`.
pub(crate) unsafe extern "C" fn compare_open_find_screen(tab: u32) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32)>(%%I1%%)(tab)
}

/// Comparer for `draw_find_screen`: the Search For screen side by side over the display list, when every tab already
/// has its search engine and none has a search pending (a first build allocates the engine and searches: only the
/// original runs).
pub(crate) unsafe extern "C" fn compare_draw_find_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I2%%);
    let tab = get_screen_param(0) as u8 as i8;
    let base = match tab {
        1 => FIND_NATIONS.base,
        2 => FIND_CLUBS.base,
        3 => FIND_NON_PLAYERS.base,
        4 => FIND_PLAYERS.base,
        _ => return compare_by_display_list_with("Search For screen (draw_find_screen)", &[], || o(), || draw_find_screen()),
    };
    if get_screen_param(base + 2) == 0 || get_screen_param(base + 3) != 0 {
        // simplification: making the engine and searching do not run twice; proven in Rust mode
        return o();
    }
    compare_by_display_list_with("Search For screen (draw_find_screen)", &[], || o(), || draw_find_screen());
}

/// A result tab's comparer: side by side over the display list when the tab has its engine and no search pending;
/// otherwise only the original runs (allocation and search).
unsafe fn tel34_compare_find_results(tab: &Tel34FindTab, name: &str, original: impl FnOnce(), rust: impl FnOnce()) {
    if get_screen_param(tab.base + 2) == 0 || get_screen_param(tab.base + 3) != 0 {
        // simplification: making the engine and searching do not run twice; proven in Rust mode
        return original();
    }
    compare_by_display_list_with(name, &[], original, rust);
}

/// Comparer for `draw_find_nations_results`.
pub(crate) unsafe extern "C" fn compare_draw_find_nations_results(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(%%I3%%);
    tel34_compare_find_results(&FIND_NATIONS, "Search For screen (draw_find_nations_results)", || o(top, bottom), || draw_find_nations_results(top, bottom));
}

/// Comparer for `draw_find_clubs_results`.
pub(crate) unsafe extern "C" fn compare_draw_find_clubs_results(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(%%I4%%);
    tel34_compare_find_results(&FIND_CLUBS, "Search For screen (draw_find_clubs_results)", || o(top, bottom), || draw_find_clubs_results(top, bottom));
}

/// Comparer for `draw_find_non_players_results`.
pub(crate) unsafe extern "C" fn compare_draw_find_non_players_results(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(%%I5%%);
    tel34_compare_find_results(&FIND_NON_PLAYERS, "Search For screen (draw_find_non_players_results)", || o(top, bottom),
        || draw_find_non_players_results(top, bottom));
}

/// Comparer for `draw_find_players_results`.
pub(crate) unsafe extern "C" fn compare_draw_find_players_results(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(%%I6%%);
    tel34_compare_find_results(&FIND_PLAYERS, "Search For screen (draw_find_players_results)", || o(top, bottom), || draw_find_players_results(top, bottom));
}

/// Comparer for `find_screen_input`.
pub(crate) unsafe extern "C" fn compare_find_screen_input(raw: u32) -> i32 {
    // simplification: the menu bar commands (they open screens) do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I7%%)(raw)
}

/// A tab input's comparer: side by side over the screen parameters and the answer.
unsafe fn tel34_compare_tab_input(name: &str, original: impl FnOnce() -> u32, rust: impl FnOnce() -> u32) -> u32 {
    let regions = screen_param_regions();
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, original, rust);
    SCOREBOARD.record(equal && ro == rr, || format!("Search For screen ({}): answer {} x {}, differences in {:?}", name, ro as i32, rr as i32,
        first_differences(&mo, &mr)));
    ro
}

/// Comparer for `find_screen_non_players_input`.
pub(crate) unsafe extern "C" fn compare_find_screen_non_players_input(raw: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> u32>(%%I8%%);
    tel34_compare_tab_input("find_screen_non_players_input", || o(raw), || find_screen_non_players_input(raw))
}

/// Comparer for `find_screen_players_input`.
pub(crate) unsafe extern "C" fn compare_find_screen_players_input(raw: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> u32>(%%I9%%);
    tel34_compare_tab_input("find_screen_players_input", || o(raw), || find_screen_players_input(raw))
}

/// Comparer for `release_find_screen`: only the original runs (it frees the engines), the parameters checked against
/// the plan afterwards (2, 8, 0xe and 0x14 cleared, the others as they were).
pub(crate) unsafe extern "C" fn compare_release_find_screen() {
    let mut plan = tel34_find_params();
    for k in [2usize, 8, 0xe, 0x14] {
        plan[k] = 0;
    }
    original_fn::<unsafe extern "C" fn()>(%%I10%%)();
    let got = tel34_find_params();
    SCOREBOARD.record(got == plan, || format!("Search For screen (release_find_screen): params {:x?} x plan {:x?}", got, plan));
}
