// ---------- batch %%LEVA%%: the Choose Friendly Teams (Find) screen (find_screens.cpp): opening it, building it with its six tabs and its input ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\find_screens.cpp", the path the find_screens.cpp error boxes print.
pub(crate) const FIND_SCREENS_CPP: usize = 0x9c9eb4;

/// A variadic format routine of the exe (`langlib_sprintf` 0x66f4e0 translates the format first; `crt_sprintf`
/// 0x944c9f does not): destination, format, the arguments.
pub(crate) type Tel28FormatFn = unsafe extern "C" fn(usize, usize, ...) -> i32;

/// `langlib_sprintf` (0x66f4e0, cdecl).
pub(crate) unsafe fn tel28_lang() -> Tel28FormatFn {
    core::mem::transmute(FN_LANGLIB_SPRINTF)
}

/// `crt_sprintf` (0x944c9f, cdecl).
pub(crate) unsafe fn tel28_crt() -> Tel28FormatFn {
    core::mem::transmute(FN_CRT_SPRINTF)
}

/// `ScreenManager::add_text_item` (0x803fd0, thiscall, ret 0x2c): x1, y1, x2, y2, the font, the flags, the style, the
/// colour, the text, the id and the value; returns eax.
pub(crate) unsafe fn tel28_text_item(f: [u32; 11]) -> u32 {
    type TextItem = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: TextItem = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    g(G_SCREEN_MANAGER, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10])
}

/// A screen's click comparer: side by side over the screen manager and the current screen's parameter block
/// (`screen_param_regions`) and the answer, when `pure` (the click only reads and writes params); otherwise only the
/// original runs.
pub(crate) unsafe fn tel28_click_compare(name: &str, widget: u32, pure: bool, original: impl FnOnce() -> i32, rust: impl FnOnce() -> i32) -> i32 {
    if !pure {
        // simplification: opening screens, the menu bar and the game changes do not run twice; those clicks are
        // proven in Rust mode
        return original();
    }
    let mut regions = screen_param_regions();
    regions.push((G_SCREEN_MANAGER, 0x3840));
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, original, rust);
    SCOREBOARD.record(equal && ro == rr, || format!("{}(widget {}): return {} x {}, differences in {:?}", name, widget as i16, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// A MenuItem of the tab bar (0xd4 bytes): word +0 = -1 and word +2 = 0 (MenuItem::MenuItem 0x4568f0), the text at
/// +4, the kind at +0xcc and the value at +0xd0.
const FIND_SCREEN_MENU_ITEM: usize = 0xd4;
/// A continent: its gender byte (+0x1e) and its three-letter name (+0x1f) (docs/FORMATOS TContinent).
const FIND_CONTINENT_GENDER: usize = 0x1e;
const FIND_CONTINENT_THREE_LETTER: usize = 0x1f;
/// A competition's three-letter name (+0x53) [inference: between the short gender byte +0x52 and the continent +0x59].
const FIND_COMP_THREE_LETTER: usize = 0x53;
/// The comparators of the friendly lists: by reputation (0x53ffe0) and by name (0x540080), qsort callbacks of 4-byte
/// club pointers [inference: from how the Major Clubs tab keeps the best quarter].
const FIND_SORT_BY_REPUTATION: usize = 0x53ffe0;
const FIND_SORT_BY_NAME: usize = 0x540080;
/// The two-column weights of the club lists (club, place) and the six-column ones (club, nation, division).
const FIND_WEIGHTS_4: [u8; 4] = [5, 1, 5, 1];
const FIND_WEIGHTS_6: [u8; 6] = [5, 1, 1, 5, 1, 1];

/// The 20 parameters of the Choose Friendly Teams screen as `open_choose_friendly_teams_screen` leaves them.
fn find_screen_planned_params(tab: u32, continent: usize, nation: usize, mode: u32) -> [usize; 0x14] {
    let mut p = [0usize; 0x14];
    p[0] = tab as u8 as i8 as isize as usize;
    p[1] = continent;
    p[4] = continent;
    p[7] = continent;
    p[0xa] = nation;
    p[0xd] = nation;
    p[0x10] = continent;
    p[0x13] = mode as u8 as i8 as isize as usize;
    p
}

/// Rule: opening the Choose Friendly Teams screen (also the Find screen) on a tab, with the continent of the continent
/// tabs, the nation of the league tabs and the mode.
///
/// `open_choose_friendly_teams_screen(tab, continent, nation, mode)`: `open_screen` (builder
/// `draw_choose_friendly_teams_screen`, input `choose_friendly_teams_screen_input`, flag 0, no leave, no exit); if it
/// does not open nothing else happens. Without a continent the find_screens.cpp box of line 0x7a (and the screen
/// still opens). The 20 parameters, in order: 0 the tab (the byte, sign-extended); 1, 4, 7 and 0x10 the continent
/// (the Nations, Under-21s, Major Clubs and Other tabs); 0xa and 0xd the nation (League and Non-League); 0x13 the mode
/// (the byte, sign-extended: 0 the Find screen, 1 the friendly clubs, 2 the friendly nations only); the rest (the
/// scroll rows and the areas of each tab) 0.
/// Original: `open_choose_friendly_teams_screen` (0x5a0650, find_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_choose_friendly_teams_screen(tab: u32, continent: usize, nation: usize, mode: u32) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x5a0840usize, 0x5a2780usize, 0, 0usize, 0) == 0 {
        return;
    }
    covered!("tel28_1 open choose friendly teams");
    if continent == 0 {
        tel26_error_box(FIND_SCREENS_CPP, 0x7a);
    }
    for (k, v) in find_screen_planned_params(tab, continent, nation, mode).iter().enumerate() {
        set_screen_param(k as u32, *v);
    }
}

/// Comparer for `open_choose_friendly_teams_screen`: only the original runs (it opens a screen), and the parameters
/// are checked against the plan afterwards.
pub(crate) unsafe extern "C" fn compare_open_choose_friendly_teams_screen(tab: u32, continent: usize, nation: usize, mode: u32) {
    let plan = find_screen_planned_params(tab, continent, nation, mode);
    original_fn::<unsafe extern "C" fn(u32, usize, usize, u32)>(%%I0%%)(tab, continent, nation, mode);
    if continent != 0 && get_screen_param(1) == continent && get_screen_param(0x13) == plan[0x13] {
        let mut got = [0usize; 0x14];
        for (k, v) in got.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
        SCOREBOARD.record(got == plan, || format!("choose friendly teams screen (open, tab {}): params {:x?} x plan {:x?}", tab as i8, got, plan));
    }
}

/// The selector button of a friendly list tab ("Continent" or "Nation" at `label`) at the top, its popup area (flags
/// 0x130, the popup colour, owned by the button) and the popup's entries (`filler`: 0x765010 the continents, 0x7651e0
/// the nations; cdecl(area, code, the chosen record)).
unsafe fn find_tab_selector(top: i32, label: usize, filler: usize, code: u32, record: usize) {
    tel28_lang()(JOB_TEXT, label);
    let button = tel26_add_button([0x6e, top as u32, 0xeb, (top + 0x14) as u32, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0, 0]);
    let popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), button]);
    let fill: unsafe extern "C" fn(u32, u32, usize) = core::mem::transmute(filler);
    fill(popup, code, record);
}

/// The heading of a friendly list tab: `fmt` with the gender byte twice and the adjective, a text item in the
/// highlight colour font 3 from `y` to `y` + 0x23.
unsafe fn find_tab_heading(y: i32, fmt: usize, gender: u8, adjective: &[u8; 0x64]) {
    let g = gender as i8 as i32;
    tel28_lang()(JOB_TEXT, fmt, g, g, adjective.as_ptr());
    tel28_text_item([0x6e, y as u32, 0x30c, (y + 0x23) as u32, 2, 0, 0xc, 3, job_colour(JOB_COLOUR_BDEC), JOB_TEXT as u32, 1]);
}

/// The club cell of a friendly list: "  short name" as a button (id 0x7d5, the club), in the highlight colour for a
/// human's club, else pale grey.
unsafe fn find_club_cell(area: u32, column: i32, row: i32, club: usize) {
    let colour = if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 { job_colour(JOB_COLOUR_BDEC) } else { job_colour(JOB_COLOUR_3184) };
    tel28_crt()(JOB_TEXT, 0x987ff0, club + club::SHORT_NAME); // "  %s"
    tel26_area_button([area, column as u32, row as u32, 1, 0, 1, 3, colour, JOB_TEXT as u32, 0x7d5, club as u32]);
}

/// The nation cell of a friendly list: the club's nation's three-letter name as a button (id 0x7d5, the national team
/// from `database_get_national_team`).
unsafe fn find_nation_cell(area: u32, column: i32, row: i32, club: usize) {
    let national = exe::database_get_national_team(read_usize(club + club::NATION));
    let nation = read_usize(club + club::NATION);
    tel26_area_button([area, (column + 1) as u32, row as u32, 1, 0, 0xc, 2, job_colour(JOB_COLOUR_BDEC), (nation + nation::THREE_LETTER_NAME) as u32, 0x7d5, national as u32]);
}

/// The division cell of a friendly list: with a division, its three-letter name as a button (id 0x7d0, the
/// competition) when the competition table has an object for it, else as text (grey-blue).
unsafe fn find_division_cell(area: u32, column: i32, row: i32, club: usize) {
    let division = read_usize(club + club::DIVISION);
    if division == 0 {
        return;
    }
    let text = (division + FIND_COMP_THREE_LETTER) as u32;
    if read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(division) as usize * 4) != 0 {
        tel26_area_button([area, (column + 2) as u32, row as u32, 1, 0, 0xc, 2, job_colour(JOB_COLOUR_BDD4), text, 0x7d0, division as u32]);
    } else {
        tel26_area_text([area, (column + 2) as u32, row as u32, 1, 0, 0xc, 2, job_colour(JOB_COLOUR_BDD4), text]);
    }
}

/// The rows of a two-column-pair list (Nations, Under-21s): per club its cell and its continent's three-letter name
/// (highlight colour), two clubs per row.
unsafe fn find_rows_with_continent(area: u32, list: usize, n: i32) {
    let (mut column, mut row) = (0i32, 0i32);
    for k in 0..n.max(0) as usize {
        let club = read_usize(list + k * 4);
        find_club_cell(area, column, row, club);
        let continent = read_usize(read_usize(club + club::NATION) + nation::CONTINENT);
        tel26_area_text([area, (column + 1) as u32, row as u32, 1, 0, 0xc, 2, job_colour(JOB_COLOUR_BDEC), (continent + FIND_CONTINENT_THREE_LETTER) as u32]);
        column += 2;
        if column > 2 {
            column = 0;
            row += 1;
        }
    }
}

/// The rows of a three-column-pair list (Major Clubs, League, Other; Non-League without the division): per club its
/// cell, its nation and its division, two clubs per row.
unsafe fn find_rows_with_nation(area: u32, list: usize, n: i32, division: bool) {
    let (mut column, mut row) = (0i32, 0i32);
    for k in 0..n.max(0) as usize {
        let club = read_usize(list + k * 4);
        find_club_cell(area, column, row, club);
        find_nation_cell(area, column, row, club);
        if division {
            find_division_cell(area, column, row, club);
        }
        column += 3;
        if column > 3 {
            column = 0;
            row += 1;
        }
    }
}

/// The list area's row count as `Display::add_area` gets it: the rows that fit in font 3 below `y` (the byte) over
/// the rest of the stack dword the original stores it into (`over`).
unsafe fn find_rows_dword(bottom: i32, y: i32, over: u32) -> u32 {
    let rows = ((bottom - y - 0x10) / exe::font_get_height(3)) as u8 as u32;
    (over & 0xffff_ff00) | rows
}

/// Restores a list's scroll row (parameter `row_param`) on the current display's area (when not -1) and keeps the
/// area in parameter `area_param`, as the tabs do inline.
unsafe fn find_restore_scroll(area: u32, row_param: u32, area_param: u32) {
    let row = get_screen_param(row_param);
    if area as i16 != -1 {
        exe::area_set_scroll_row(current_human_display() + 4 + (area as i16 as isize as usize).wrapping_mul(0xbf1), row);
    }
    set_screen_param(area_param, area as i16 as isize as usize);
}

/// The friendly list buffer (one pointer per club, `crt_malloc`); none: the find_screens.cpp box at `line`.
unsafe fn find_list_alloc(line: u32) -> usize {
    let list = exe::crt_malloc((read_i32(G_CLUB_COUNT) * 4) as u32);
    if list == 0 {
        tel26_error_box(FIND_SCREENS_CPP, line);
    }
    list
}

/// The Nations tab of the Choose Friendly Teams screen (tab 1): the national teams of a continent.
///
/// The nation picture (0x41c100). The continent is parameter 1 (none: the find_screens.cpp box of line 0x163, then
/// on). The "Continent" selector (popup code 2) and the heading "<adjective> Nations" (`get_continent_adjective`, form
/// 0x12). The clubs from the first (`G_CLUB_COUNT` − `G_NATION_COUNT`) that are national teams
/// (`database_is_national_team`) of a nation of the continent (+0x71), sorted by name, in a 4-column area of
/// `ScreenManager::add_area` (the rows over the bottom's upper bytes, as the original's stack slot), two per row with
/// their continent; the scroll row of parameter 2 and the area to parameter 3. No list buffer: the box of line 0x184.
/// Original: part of `draw_choose_friendly_teams_screen` (0x5a0840, find_screens.cpp).
unsafe fn find_nations_tab(top: i32, bottom: i32) {
    let picture: unsafe extern "C" fn(u32, u32, u32) = core::mem::transmute(0x41c100usize);
    picture(0, 0, 0);
    let weights = FIND_WEIGHTS_4;
    let continent = get_screen_param(1);
    if continent == 0 {
        tel26_error_box(FIND_SCREENS_CPP, 0x163);
    }
    find_tab_selector(top, 0x9c9f10, 0x765010, 2, continent); // "Continent"
    let mut adjective = [0u8; 0x64];
    exe::get_continent_adjective(continent, adjective.as_mut_ptr(), 0x64, 0x12);
    let y = top + 0x19;
    find_tab_heading(y, 0x9c9ee4, read_u8(continent + FIND_CONTINENT_GENDER), &adjective); // "{}<%s>{} Nations"
    let y = y + 0x28;
    let list = find_list_alloc(0x184);
    if list == 0 {
        return;
    }
    covered!("tel28_1 nations tab");
    let mut n = 0i32;
    for i in 0..(read_i32(G_CLUB_COUNT) - read_i32(G_NATION_COUNT)).max(0) as usize {
        let club = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        if exe::database_is_national_team(club) != 0 {
            let nation = read_usize(club + club::NATION);
            if nation != 0 && read_usize(nation + nation::CONTINENT) == continent {
                write_usize(list + n as usize * 4, club);
                n += 1;
            }
        }
    }
    exe::crt_qsort(list, n as u32, 4, FIND_SORT_BY_NAME);
    let rows = find_rows_dword(bottom, y, bottom as u32);
    let add_area: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x4567f0usize);
    let area = add_area(G_SCREEN_MANAGER, 0x6e, y as u32, 0x30c, bottom as u32, 4, weights.as_ptr() as u32, rows, 0, 2, 0, u32::MAX);
    find_rows_with_continent(area, list, n);
    exe::crt_free(list);
    let scroll: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x4567a0usize);
    scroll(G_SCREEN_MANAGER, area, get_screen_param(2) as u32);
    set_screen_param(3, area as i16 as isize as usize);
}

/// Rule: building the Choose Friendly Teams screen (also the Find screen): the title, the tab bar and the open tab.
///
/// The tab is parameter 0's byte, the mode parameter 0x13's. The title: "Choose Friendly Teams" with a mode, else the
/// game title (`get_game_title` 0x839570, copied); in the heading colours (`get_heading_colour` 1 then 0); the menu
/// bar (4). Mode 2 (nations only): no tab bar, parameter 0 = 1 and the Nations tab. Otherwise the tabs "Nations" (1)
/// and "Under-21s" (2) without mode 1, then "Major Clubs" (3), "League" (4), "Non-League" (5) and "Other" (6), all of
/// kind 1, in the narrow tab bar (`gui_add_tab_bar` from 0x50 to 0x221). Then the tab: 1 `find_nations_tab`; 2 the
/// nation picture and `draw_choose_friendly_under21s_tab`; 3 to 6 the club picture (none) and the Major Clubs, League,
/// Non-League and Other tabs; any other tab the random picture and an empty one-column area. Then the Back/Next
/// buttons.
/// Original: `draw_choose_friendly_teams_screen` (0x5a0840, find_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn draw_choose_friendly_teams_screen() {
    let mut items = [[0u8; FIND_SCREEN_MENU_ITEM]; 0x19];
    for it in items.iter_mut() {
        it[0] = 0xff;
        it[1] = 0xff;
    }
    let (mut top, mut bottom) = (0x50i32, 0x221i32);
    let mut tab = get_screen_param(0) as u8 as i8;
    let mode = get_screen_param(0x13) as u8;
    if mode != 0 {
        tel28_lang()(JOB_TEXT, 0x9c9f6c); // "Choose Friendly Teams"
    } else {
        let title: unsafe extern "C" fn() -> *const u8 = core::mem::transmute(0x839570usize);
        tel26_copy_text(JOB_TEXT as *mut u8, title());
    }
    let colour_1 = exe::get_heading_colour(1) as u32;
    let colour_0 = exe::get_heading_colour(0) as u32;
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, colour_0, colour_1, JOB_TEXT);
    exe::menubar_build(4, 0);
    if mode == 2 {
        set_screen_param(0, 1);
        tab = 1;
    } else {
        fn set_kind(items: &mut [[u8; FIND_SCREEN_MENU_ITEM]; 0x19], k: usize, value: u32) {
            items[k][0xcc..0xd0].copy_from_slice(&1u32.to_le_bytes());
            items[k][0xd0..0xd4].copy_from_slice(&value.to_le_bytes());
        }
        let text = |items: &mut [[u8; FIND_SCREEN_MENU_ITEM]; 0x19], k: usize| items[k][4..].as_mut_ptr() as usize;
        let mut k = 0usize;
        if mode != 1 {
            tel28_lang()(text(&mut items, 0), 0x9c9f64); // "Nations"
            set_kind(&mut items, 0, 1);
            tel28_lang()(text(&mut items, 1), 0x9c9f58); // "Under-21s"
            set_kind(&mut items, 1, 2);
            k = 2;
        }
        for (fmt, value) in [(0x9c9f4cusize, 3u32), (0x990f4c, 4), (0x9c9f40, 5)] {
            tel28_lang()(text(&mut items, k), fmt); // "Major Clubs", "League", "Non-League"
            set_kind(&mut items, k, value);
            k += 1;
        }
        tel28_lang()(text(&mut items, k), 0x9c9f1c, 0xad9c64usize); // "Other<%s - COMMENT - other clubs>"
        set_kind(&mut items, k, 6);
        k += 1;
        exe::gui_add_tab_bar(items.as_mut_ptr() as usize, k as u32, tab as i32, &mut top, &mut bottom, 1);
    }
    let tab_routine = |address: usize| {
        let f: unsafe extern "C" fn(i32, i32) = core::mem::transmute(address);
        f(top, bottom);
    };
    match tab {
        1 => find_nations_tab(top, bottom),
        2 => {
            let picture: unsafe extern "C" fn(u32, u32, u32) = core::mem::transmute(0x41c100usize);
            picture(0, 0, 0);
            tab_routine(0x5a1030);
        }
        3..=6 => {
            exe::background_set_club_picture(0usize, 0, 0);
            tab_routine([0x5a1480usize, 0x5a1990, 0x5a1e10, 0x5a2250][(tab - 3) as usize]);
        }
        _ => {
            exe::background_set_random_picture();
            tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 1, 0, 1, 0, 2, 0, u32::MAX]);
        }
    }
    tel26_back_next();
}

/// Whether building the Choose Friendly Teams screen would show an error box (the open tab's continent or nation
/// missing): those builds run the original only.
unsafe fn find_screen_build_has_box() -> bool {
    let tab = if get_screen_param(0x13) as u8 == 2 { 1 } else { get_screen_param(0) as u8 };
    match tab {
        1 => get_screen_param(1) == 0,
        2 => get_screen_param(4) == 0,
        3 => get_screen_param(7) == 0,
        6 => get_screen_param(0x10) == 0,
        _ => false,
    }
}

/// Comparer for `draw_choose_friendly_teams_screen`: side by side over the display list (`compare_match_screen_build`);
/// a build with an error box (no continent) only the original.
pub(crate) unsafe extern "C" fn compare_draw_choose_friendly_teams_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    if find_screen_build_has_box() {
        // simplification: the error box does not run twice
        return o();
    }
    compare_match_screen_build("choose friendly teams screen", &[], || o(), || draw_choose_friendly_teams_screen());
}

/// Rule: the Under-21s tab of the Choose Friendly Teams screen lists the national teams of a continent among the last
/// clubs of the table (the under-21 sides).
///
/// `draw_choose_friendly_under21s_tab(top, bottom)`: the continent is parameter 4 (none: the find_screens.cpp box of
/// line 0x1ea, then on). The "Continent" selector (popup code 3) and the heading "<adjective> Under-21s"
/// (`get_continent_adjective`, form 0x11). The clubs from index `G_CLUB_COUNT` − `G_NATION_COUNT` to the end that are
/// national teams (`database_is_national_team`) of a nation of the continent, sorted by name, in a 4-column area of
/// the current Display (the rows over the last scanned club pointer, the original's stack slot), two per row with
/// their continent; the scroll row of parameter 5 and the area to parameter 6. No list buffer: the box of line 0x20a
/// and nothing else.
/// Original: `draw_choose_friendly_under21s_tab` (0x5a1030, find_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn draw_choose_friendly_under21s_tab(top: i32, bottom: i32) {
    let weights = FIND_WEIGHTS_4;
    let continent = get_screen_param(4);
    if continent == 0 {
        tel26_error_box(FIND_SCREENS_CPP, 0x1ea);
    }
    find_tab_selector(top, 0x9c9f10, 0x765010, 3, continent); // "Continent"
    let mut adjective = [0u8; 0x64];
    exe::get_continent_adjective(continent, adjective.as_mut_ptr(), 0x64, 0x11);
    let y = top + 0x19;
    find_tab_heading(y, 0x9c9f84, read_u8(continent + FIND_CONTINENT_GENDER), &adjective); // "{}<%s>{} Under-21s"
    let y = y + 0x28;
    let list = find_list_alloc(0x20a);
    if list == 0 {
        return;
    }
    covered!("tel28_1 under-21s tab");
    let (mut n, mut last) = (0i32, 0usize);
    let count = read_i32(G_CLUB_COUNT);
    for i in (count - read_i32(G_NATION_COUNT)).max(0)..count.max(0) {
        let club = read_usize(G_CLUB_TABLE) + i as usize * club::SIZE;
        last = club;
        if exe::database_is_national_team(club) != 0 {
            let nation = read_usize(club + club::NATION);
            if nation != 0 && read_usize(nation + nation::CONTINENT) == continent {
                write_usize(list + n as usize * 4, club);
                n += 1;
            }
        }
    }
    exe::crt_qsort(list, n as u32, 4, FIND_SORT_BY_NAME);
    // simplification: with no club scanned the original's slot is stack garbage (0 here)
    let rows = find_rows_dword(bottom, y, last as u32);
    let area = tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, bottom as u32, 4, weights.as_ptr() as u32, rows, 0, 2, 0, u32::MAX]);
    find_rows_with_continent(area, list, n);
    exe::crt_free(list);
    find_restore_scroll(area, 5, 6);
}

/// Comparer for `draw_choose_friendly_under21s_tab`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_draw_choose_friendly_under21s_tab(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(%%I2%%);
    if get_screen_param(4) == 0 {
        return o(top, bottom);
    }
    compare_match_screen_build("choose friendly teams screen (Under-21s)", &[], || o(top, bottom), || draw_choose_friendly_under21s_tab(top, bottom));
}

/// The best quarter of a sorted list, at most 100: `_ftol(min(n × 0.25, 100.0))`.
unsafe fn find_best_quarter(n: i32) -> i32 {
    let x = n as f64 * read_f64(rdata::F64_0_25);
    let cap = read_f64(rdata::F64_100);
    (if x < cap { x } else { cap }) as i32
}

/// Rule: the Major Clubs tab lists the best quarter (at most 100) of a continent's clubs, by reputation, then by name.
///
/// `draw_choose_friendly_major_clubs_tab(top, bottom)`: the continent is parameter 7 (none: the box of line 0x271,
/// then on). The "Continent" selector (popup code 4) and the heading "Major <adjective> Clubs"
/// (`get_continent_adjective`, form 2). The clubs of the first (`G_CLUB_COUNT` − 2 × `G_NATION_COUNT`) that are not
/// national teams, of a nation of the continent; sorted by reputation (0x53ffe0), the first `find_best_quarter` of
/// them sorted by name; a 6-column area (the rows over the scan's last index), two per row with their nation and
/// division; the scroll row of parameter 8 and the area to parameter 9. No list buffer: the box of line 0x291 and
/// nothing else.
/// Original: `draw_choose_friendly_major_clubs_tab` (0x5a1480, find_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn draw_choose_friendly_major_clubs_tab(top: i32, bottom: i32) {
    let weights = FIND_WEIGHTS_6;
    let continent = get_screen_param(7);
    if continent == 0 {
        tel26_error_box(FIND_SCREENS_CPP, 0x271);
    }
    find_tab_selector(top, 0x9c9f10, 0x765010, 4, continent); // "Continent"
    let mut adjective = [0u8; 0x64];
    exe::get_continent_adjective(continent, adjective.as_mut_ptr(), 0x64, 2);
    let y = top + 0x19;
    find_tab_heading(y, 0x9c9fb4, read_u8(continent + FIND_CONTINENT_GENDER), &adjective); // "Major {}<%s>{} Clubs"
    let y = y + 0x28;
    let list = find_list_alloc(0x291);
    if list == 0 {
        return;
    }
    covered!("tel28_1 major clubs tab");
    let (mut n, mut i) = (0i32, 0i32);
    while i < read_i32(G_CLUB_COUNT) - 2 * read_i32(G_NATION_COUNT) {
        let club = read_usize(G_CLUB_TABLE) + i as usize * club::SIZE;
        if exe::database_is_national_team(club) == 0 {
            let nation = read_usize(club + club::NATION);
            if nation != 0 && read_usize(nation + nation::CONTINENT) == continent {
                write_usize(list + n as usize * 4, club);
                n += 1;
            }
        }
        i += 1;
    }
    exe::crt_qsort(list, n as u32, 4, FIND_SORT_BY_REPUTATION);
    let n = find_best_quarter(n);
    exe::crt_qsort(list, n as u32, 4, FIND_SORT_BY_NAME);
    let rows = find_rows_dword(bottom, y, i as u32);
    let area = tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, bottom as u32, 6, weights.as_ptr() as u32, rows, 0, 2, 0, u32::MAX]);
    find_rows_with_nation(area, list, n, true);
    exe::crt_free(list);
    find_restore_scroll(area, 8, 9);
}

/// Comparer for `draw_choose_friendly_major_clubs_tab`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_draw_choose_friendly_major_clubs_tab(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(%%I3%%);
    if get_screen_param(7) == 0 {
        return o(top, bottom);
    }
    compare_match_screen_build("choose friendly teams screen (Major Clubs)", &[], || o(top, bottom), || draw_choose_friendly_major_clubs_tab(top, bottom));
}

/// Rule: the League tab lists a nation's clubs that play in a league the game runs, by name.
///
/// `draw_choose_friendly_league_clubs_tab(top, bottom)`: the nation is parameter 10. The "Nation" selector (popup
/// 0x7651e0, code 5) and the heading "<adjective> League Clubs" (`nation_get_nationality_adjective`, gender 3 in
/// language 4, else 2; the nation's gender byte). The clubs of the first (`G_CLUB_COUNT` − `G_NATION_COUNT`) that are
/// not national teams, of the nation, with a division that has an object in the competition table; sorted by name; a
/// 6-column area (the rows over the scan's last index), two per row with their nation and division; the scroll row
/// of parameter 11 and the area to parameter 12. No list buffer: the box of line 0x332 and nothing else.
/// Original: `draw_choose_friendly_league_clubs_tab` (0x5a1990, find_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn draw_choose_friendly_league_clubs_tab(top: i32, bottom: i32) {
    let weights = FIND_WEIGHTS_6;
    let nation = get_screen_param(0xa);
    find_tab_selector(top, 0x9ca010, 0x7651e0, 5, nation); // "Nation"
    let mut adjective = [0u8; 0x64];
    // simplification: the original adds 2 to a register whose upper bytes are a callee's leftovers; the gender is
    // read as its byte
    let gender = (exe::langlib_get_language() == 4) as u32 + 2;
    exe::nation_get_nationality_adjective(nation, adjective.as_mut_ptr() as usize, 0x64, gender);
    let y = top + 0x19;
    find_tab_heading(y, 0x9c9fe4, read_u8(nation + nation::GENDER_NAME), &adjective); // "{}<%s>{} League Clubs"
    let y = y + 0x28;
    let list = find_list_alloc(0x332);
    if list == 0 {
        return;
    }
    covered!("tel28_1 league clubs tab");
    let (mut n, mut i) = (0i32, 0i32);
    while i < read_i32(G_CLUB_COUNT) - read_i32(G_NATION_COUNT) {
        let club = read_usize(G_CLUB_TABLE) + i as usize * club::SIZE;
        if exe::database_is_national_team(club) == 0 && read_usize(club + club::NATION) == nation {
            let division = read_usize(club + club::DIVISION);
            if division != 0 && read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(division) as usize * 4) != 0 {
                write_usize(list + n as usize * 4, club);
                n += 1;
            }
        }
        i += 1;
    }
    exe::crt_qsort(list, n as u32, 4, FIND_SORT_BY_NAME);
    let rows = find_rows_dword(bottom, y, i as u32);
    let area = tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, bottom as u32, 6, weights.as_ptr() as u32, rows, 0, 2, 0, u32::MAX]);
    find_rows_with_nation(area, list, n, true);
    exe::crt_free(list);
    find_restore_scroll(area, 0xb, 0xc);
}

/// Comparer for `draw_choose_friendly_league_clubs_tab`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_draw_choose_friendly_league_clubs_tab(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(%%I4%%);
    compare_match_screen_build("choose friendly teams screen (League)", &[], || o(top, bottom), || draw_choose_friendly_league_clubs_tab(top, bottom));
}

/// Rule: the Non-League tab lists a nation's clubs outside the leagues the game runs, the 250 best by reputation, by
/// name.
///
/// `draw_choose_friendly_non_league_tab(top, bottom)`: the nation is parameter 13. The "Nation" selector (popup code
/// 6) and the heading "<adjective> Non-League Clubs" (`nation_get_nationality_adjective`, gender 2). The clubs of the
/// first (`G_CLUB_COUNT` − 2 × `G_NATION_COUNT`) that are not national teams, of the nation, with no division or a
/// division with no object in the competition table; sorted by reputation, the first 250 at most sorted by name; a
/// 6-column area (the rows over the scan's last index), two per row with their nation (no division); the scroll row
/// of parameter 14 and the area to parameter 15. No list buffer: the box of line 0x3cd and nothing else.
/// Original: `draw_choose_friendly_non_league_tab` (0x5a1e10, find_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn draw_choose_friendly_non_league_tab(top: i32, bottom: i32) {
    let weights = FIND_WEIGHTS_6;
    let nation = get_screen_param(0xd);
    find_tab_selector(top, 0x9ca010, 0x7651e0, 6, nation); // "Nation"
    let mut adjective = [0u8; 0x64];
    exe::nation_get_nationality_adjective(nation, adjective.as_mut_ptr() as usize, 0x64, 2);
    let y = top + 0x19;
    find_tab_heading(y, 0x9ca018, read_u8(nation + nation::GENDER_NAME), &adjective); // "{}<%s>{} Non-League Clubs"
    let y = y + 0x28;
    let list = find_list_alloc(0x3cd);
    if list == 0 {
        return;
    }
    covered!("tel28_1 non-league tab");
    let (mut n, mut i) = (0i32, 0i32);
    while i < read_i32(G_CLUB_COUNT) - 2 * read_i32(G_NATION_COUNT) {
        let club = read_usize(G_CLUB_TABLE) + i as usize * club::SIZE;
        if exe::database_is_national_team(club) == 0 && read_usize(club + club::NATION) == nation {
            let division = read_usize(club + club::DIVISION);
            if division == 0 || read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(division) as usize * 4) == 0 {
                write_usize(list + n as usize * 4, club);
                n += 1;
            }
        }
        i += 1;
    }
    exe::crt_qsort(list, n as u32, 4, FIND_SORT_BY_REPUTATION);
    let n = if n >= 0xfa { 0xfa } else { n }; // tel28_1 the Non-League cap
    exe::crt_qsort(list, n as u32, 4, FIND_SORT_BY_NAME);
    let rows = find_rows_dword(bottom, y, i as u32);
    let area = tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, bottom as u32, 6, weights.as_ptr() as u32, rows, 0, 2, 0, u32::MAX]);
    find_rows_with_nation(area, list, n, false);
    exe::crt_free(list);
    find_restore_scroll(area, 0xe, 0xf);
}

/// Comparer for `draw_choose_friendly_non_league_tab`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_draw_choose_friendly_non_league_tab(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(%%I5%%);
    compare_match_screen_build("choose friendly teams screen (Non-League)", &[], || o(top, bottom), || draw_choose_friendly_non_league_tab(top, bottom));
}

/// Rule: the Other tab lists a continent's clubs from nations whose leagues are not selected, past the best quarter,
/// the next 100 at most, by name.
///
/// `draw_choose_friendly_other_clubs_tab(top, bottom)`: the continent is parameter 16 (none: the box of line 0x43d,
/// then on). The "Continent" selector (popup code 7) and the heading "Other <adjective> Clubs"
/// (`get_continent_adjective`, form 2). The clubs of the first (`G_CLUB_COUNT` − `G_NATION_COUNT`) that are not
/// national teams, of a nation of the continent whose leagues are not selected (`nation::LEAGUE_SELECTED` 0); sorted
/// by reputation; the `find_best_quarter` first are skipped and the next ones (at most 100) moved to the front and
/// sorted by name; a 6-column area (the rows over the scan's last index), two per row with their nation and division;
/// the scroll row of parameter 17 and the area to parameter 18. No list buffer: the box of line 0x45e and nothing
/// else.
/// Original: `draw_choose_friendly_other_clubs_tab` (0x5a2250, find_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn draw_choose_friendly_other_clubs_tab(top: i32, bottom: i32) {
    let weights = FIND_WEIGHTS_6;
    let continent = get_screen_param(0x10);
    if continent == 0 {
        tel26_error_box(FIND_SCREENS_CPP, 0x43d);
    }
    find_tab_selector(top, 0x9c9f10, 0x765010, 7, continent); // "Continent"
    let mut adjective = [0u8; 0x64];
    exe::get_continent_adjective(continent, adjective.as_mut_ptr(), 0x64, 2);
    let y = top + 0x19;
    find_tab_heading(y, 0x9ca048, read_u8(continent + FIND_CONTINENT_GENDER), &adjective); // "Other {}<%s>{} Clubs"
    let y = y + 0x28;
    let list = find_list_alloc(0x45e);
    if list == 0 {
        return;
    }
    covered!("tel28_1 other clubs tab");
    let (mut n, mut i) = (0i32, 0i32);
    while i < read_i32(G_CLUB_COUNT) - read_i32(G_NATION_COUNT) {
        let club = read_usize(G_CLUB_TABLE) + i as usize * club::SIZE;
        if exe::database_is_national_team(club) == 0 {
            let nation = read_usize(club + club::NATION);
            if nation != 0 && read_u8(nation + nation::LEAGUE_SELECTED) == 0 && read_usize(nation + nation::CONTINENT) == continent {
                write_usize(list + n as usize * 4, club);
                n += 1;
            }
        }
        i += 1;
    }
    exe::crt_qsort(list, n as u32, 4, FIND_SORT_BY_REPUTATION);
    let skip = find_best_quarter(n);
    let rest = (n - skip).min(0x64);
    for k in 0..rest.max(0) as usize {
        write_usize(list + k * 4, read_usize(list + (skip as usize + k) * 4));
    }
    exe::crt_qsort(list, rest as u32, 4, FIND_SORT_BY_NAME);
    let rows = find_rows_dword(bottom, y, i as u32);
    let area = tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, bottom as u32, 6, weights.as_ptr() as u32, rows, 0, 2, 0, u32::MAX]);
    find_rows_with_nation(area, list, rest, true);
    exe::crt_free(list);
    find_restore_scroll(area, 0x11, 0x12);
}

/// Comparer for `draw_choose_friendly_other_clubs_tab`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_draw_choose_friendly_other_clubs_tab(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(%%I6%%);
    if get_screen_param(0x10) == 0 {
        return o(top, bottom);
    }
    compare_match_screen_build("choose friendly teams screen (Other)", &[], || o(top, bottom), || draw_choose_friendly_other_clubs_tab(top, bottom));
}

/// The pick of the Choose Friendly Teams tabs 1 to 5: the scroll row and area parameters, the parameter the pick
/// goes to, the widget id and whether the id is the action id (0x468b40) rather than the widget id.
fn find_tab_pick(tab: i8) -> Option<(u32, u32, u32, i32, bool)> {
    match tab {
        1 => Some((2, 3, 1, 2, false)),
        2 => Some((5, 6, 4, 3, false)),
        3 => Some((8, 9, 7, 4, false)),
        4 => Some((0xb, 0xc, 0xa, 5, true)),
        5 => Some((0xe, 0xf, 0xd, 6, true)),
        _ => None,
    }
}

/// `ScreenManager::get_widget_action_id` (0x468b40, thiscall, ret 4): the widget's id, or the default command id.
unsafe fn find_action_id(widget: u32) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x468b40usize);
    f(G_SCREEN_MANAGER, widget)
}

/// Rule: a click on the Choose Friendly Teams screen picks a continent or nation for the open tab, or a tab.
///
/// `choose_friendly_teams_screen_input(widget)`, by the tab (parameter 0's byte): tabs 1 to 5 first keep the list's
/// scroll row (when there is a widget: `get_area_scroll_row` of the area parameter into the row parameter), then a
/// pick of the tab's popup (Nations: widget id 2 into parameter 1; Under-21s: 3 into 4; Major Clubs: 4 into 7; League:
/// action id 5 into 10; Non-League: 6 into 13) whose data (`get_widget_action_data`) differs from the parameter is
/// stored and returns −11 (rebuild); tab 6 asks `choose_friendly_teams_other_clubs_input` (0x5a2b30) first and returns
/// its answer when not 0. Then a tab click (widget id 1) with a new tab stores it in parameter 0 and returns −11;
/// otherwise the menu bar (`menubar_handle_command`) and, when it does nothing, the object link
/// (`menubar_handle_object_link`) answer.
/// Original: `choose_friendly_teams_screen_input` (0x5a2780, find_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn choose_friendly_teams_screen_input(widget: u32) -> i32 {
    let w = widget as i16;
    let tab = get_screen_param(0) as u8 as i8;
    if tab == 6 {
        let other: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x5a2b30usize);
        let r = other(widget);
        if r != 0 {
            return r;
        }
    } else if let Some((row_param, area_param, pick_param, id, by_action)) = find_tab_pick(tab) {
        if w != -1 {
            set_screen_param(row_param, exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(area_param)));
        }
        let hit = if by_action { find_action_id(widget) == id } else { tel26_item_id(w) as i32 == id };
        if hit && exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget) != get_screen_param(pick_param) {
            covered!("tel28_1 pick a continent or nation");
            set_screen_param(pick_param, exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget));
            return -11;
        }
    }
    if tel26_item_id(w) == 1 && exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget) != get_screen_param(0) {
        covered!("tel28_1 pick a tab");
        set_screen_param(0, exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget));
        return -11;
    }
    let r = exe::menubar_handle_command(widget);
    if r != 0 {
        return r;
    }
    exe::menubar_handle_object_link(widget)
}

/// Comparer for `choose_friendly_teams_screen_input`: side by side for the picks (a continent, a nation or a tab);
/// the menu bar, the object links and tab 6 (its own input routine) only the original.
pub(crate) unsafe extern "C" fn compare_choose_friendly_teams_screen_input(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I7%%);
    let w = widget as i16;
    let tab = get_screen_param(0) as u8 as i8;
    let data = exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget);
    let tab_pick = find_tab_pick(tab).is_some_and(|(_, _, pick_param, id, by_action)| {
        let hit = if by_action { find_action_id(widget) == id } else { tel26_item_id(w) as i32 == id };
        hit && data != get_screen_param(pick_param)
    });
    let pure = tab != 6 && (tab_pick || (tel26_item_id(w) == 1 && data != get_screen_param(0)));
    tel28_click_compare("choose friendly teams screen input", widget, pure, || o(widget), || choose_friendly_teams_screen_input(widget))
}
