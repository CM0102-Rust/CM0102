use super::*;

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
pub(crate) unsafe extern "C" fn open_choose_friendly_teams_screen(table: u32, continent: usize, nation: usize, mode: u32) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x5a0840usize, 0x5a2780usize, 0, 0usize, 0) == 0 {
        return;
    }
    covered!("tel28_1 open choose friendly teams");
    if continent == 0 {
        tel26_error_box(FIND_SCREENS_CPP, 0x7a);
    }
    for (k, v) in find_screen_planned_params(table, continent, nation, mode).iter().enumerate() {
        set_screen_param(k as u32, *v);
    }
}

/// Comparer for `open_choose_friendly_teams_screen`: only the original runs (it opens a screen), and the parameters
/// are checked against the plan afterwards.
pub(crate) unsafe extern "C" fn compare_open_choose_friendly_teams_screen(table: u32, continent: usize, nation: usize, mode: u32) {
    let plan = find_screen_planned_params(table, continent, nation, mode);
    original_fn::<unsafe extern "C" fn(u32, usize, usize, u32)>(IDX_OPEN_CHOOSE_FRIENDLY_TEAMS_SCREEN)(table, continent, nation, mode);
    if continent != 0 && get_screen_param(1) == continent && get_screen_param(0x13) == plan[0x13] {
        let mut got = [0usize; 0x14];
        for (k, v) in got.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
        SCOREBOARD.record(got == plan, || format!("choose friendly teams screen (open, table {}): params {:x?} x plan {:x?}", table as i8, got, plan));
    }
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
    let mut table = get_screen_param(0) as u8 as i8;
    let mode = get_screen_param(0x13) as u8;
    if mode != 0 {
        tel28_lang()(JOB_TEXT, 0x9c9f6c); // "Choose Friendly Teams"
    } else {
        let title: unsafe extern "C" fn() -> *const u8 = core::mem::transmute(FN_GET_GAME_TITLE);
        tel26_copy_text(JOB_TEXT as *mut u8, title());
    }
    let colour_1 = exe::get_heading_colour(1) as u32;
    let colour_0 = exe::get_heading_colour(0) as u32;
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, colour_0, colour_1, JOB_TEXT);
    exe::menubar_build(4, 0);
    if mode == 2 {
        set_screen_param(0, 1);
        table = 1;
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
        exe::gui_add_tab_bar(items.as_mut_ptr() as usize, k as u32, table as i32, &mut top, &mut bottom, 1);
    }
    let table_routine = |address: usize| {
        let f: unsafe extern "C" fn(i32, i32) = core::mem::transmute(address);
        f(top, bottom);
    };
    match table {
        1 => find_nations_tab(top, bottom),
        2 => {
            let picture: unsafe extern "C" fn(u32, u32, u32) = core::mem::transmute(FN_BACKGROUND_SET_NATION_PICTURE);
            picture(0, 0, 0);
            table_routine(0x5a1030);
        }
        3..=6 => {
            exe::background_set_club_picture(0usize, 0, 0);
            table_routine([0x5a1480usize, 0x5a1990, 0x5a1e10, 0x5a2250][(table - 3) as usize]);
        }
        _ => {
            exe::background_set_random_picture();
            tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 1, 0, 1, 0, 2, 0, u32::MAX]);
        }
    }
    tel26_back_next();
}

/// Comparer for `draw_choose_friendly_teams_screen`: side by side over the display list (`compare_match_screen_build`);
/// a build with an error box (no continent) only the original.
pub(crate) unsafe extern "C" fn compare_draw_choose_friendly_teams_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_DRAW_CHOOSE_FRIENDLY_TEAMS_SCREEN);
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
        let club = Club(read_usize(G_CLUB_TABLE) + i as usize * club::SIZE);
        last = club.addr();
        if exe::database_is_national_team(club) != 0 {
            let nation = club.nation();
            if !nation.is_null() && nation.continent() == continent {
                write_usize(list + n as usize * 4, club.addr());
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
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(IDX_DRAW_CHOOSE_FRIENDLY_UNDER21S_TAB);
    if get_screen_param(4) == 0 {
        return o(top, bottom);
    }
    compare_match_screen_build("choose friendly teams screen (Under-21s)", &[], || o(top, bottom), || draw_choose_friendly_under21s_tab(top, bottom));
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
        let club = Club(read_usize(G_CLUB_TABLE) + i as usize * club::SIZE);
        if exe::database_is_national_team(club) == 0 {
            let nation = club.nation();
            if !nation.is_null() && nation.continent() == continent {
                write_usize(list + n as usize * 4, club.addr());
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
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(IDX_DRAW_CHOOSE_FRIENDLY_MAJOR_CLUBS_TAB);
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
    let nation = Nation(get_screen_param(0xa));
    find_tab_selector(top, 0x9ca010, 0x7651e0, 5, nation.addr()); // "Nation"
    let mut adjective = [0u8; 0x64];
    // simplification: the original adds 2 to a register whose upper bytes are a callee's leftovers; the gender is
    // read as its byte
    let gender = (exe::langlib_get_language() == 4) as u32 + 2;
    exe::nation_get_nationality_adjective(nation, adjective.as_mut_ptr() as usize, 0x64, gender);
    let y = top + 0x19;
    find_tab_heading(y, 0x9c9fe4, nation.gender_name(), &adjective); // "{}<%s>{} League Clubs"
    let y = y + 0x28;
    let list = find_list_alloc(0x332);
    if list == 0 {
        return;
    }
    covered!("tel28_1 league clubs tab");
    let (mut n, mut i) = (0i32, 0i32);
    while i < read_i32(G_CLUB_COUNT) - read_i32(G_NATION_COUNT) {
        let club = Club(read_usize(G_CLUB_TABLE) + i as usize * club::SIZE);
        if exe::database_is_national_team(club) == 0 && club.nation() == nation {
            let division = club.division().addr();
            if division != 0 && read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(division) as usize * 4) != 0 {
                write_usize(list + n as usize * 4, club.addr());
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
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(IDX_DRAW_CHOOSE_FRIENDLY_LEAGUE_CLUBS_TAB);
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
    let nation = Nation(get_screen_param(0xd));
    find_tab_selector(top, 0x9ca010, 0x7651e0, 6, nation.addr()); // "Nation"
    let mut adjective = [0u8; 0x64];
    exe::nation_get_nationality_adjective(nation, adjective.as_mut_ptr() as usize, 0x64, 2);
    let y = top + 0x19;
    find_tab_heading(y, 0x9ca018, nation.gender_name(), &adjective); // "{}<%s>{} Non-League Clubs"
    let y = y + 0x28;
    let list = find_list_alloc(0x3cd);
    if list == 0 {
        return;
    }
    covered!("tel28_1 non-league tab");
    let (mut n, mut i) = (0i32, 0i32);
    while i < read_i32(G_CLUB_COUNT) - 2 * read_i32(G_NATION_COUNT) {
        let club = Club(read_usize(G_CLUB_TABLE) + i as usize * club::SIZE);
        if exe::database_is_national_team(club) == 0 && club.nation() == nation {
            let division = club.division().addr();
            if division == 0 || read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(division) as usize * 4) == 0 {
                write_usize(list + n as usize * 4, club.addr());
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
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(IDX_DRAW_CHOOSE_FRIENDLY_NON_LEAGUE_TAB);
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
        let club = Club(read_usize(G_CLUB_TABLE) + i as usize * club::SIZE);
        if exe::database_is_national_team(club) == 0 {
            let nation = club.nation();
            if !nation.is_null() && nation.league_selected() == 0 && nation.continent() == continent {
                write_usize(list + n as usize * 4, club.addr());
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
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(IDX_DRAW_CHOOSE_FRIENDLY_OTHER_CLUBS_TAB);
    if get_screen_param(0x10) == 0 {
        return o(top, bottom);
    }
    compare_match_screen_build("choose friendly teams screen (Other)", &[], || o(top, bottom), || draw_choose_friendly_other_clubs_tab(top, bottom));
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
    let table = get_screen_param(0) as u8 as i8;
    if table == 6 {
        let r = exe::choose_friendly_teams_other_clubs_input(widget);
        if r != 0 {
            return r;
        }
    } else if let Some((row_param, area_param, pick_param, id, by_action)) = find_tab_pick(table) {
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
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_CHOOSE_FRIENDLY_TEAMS_SCREEN_INPUT);
    let w = widget as i16;
    let table = get_screen_param(0) as u8 as i8;
    let data = exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget);
    let table_pick = find_tab_pick(table).is_some_and(|(_, _, pick_param, id, by_action)| {
        let hit = if by_action { find_action_id(widget) == id } else { tel26_item_id(w) as i32 == id };
        hit && data != get_screen_param(pick_param)
    });
    let pure = table != 6 && (table_pick || (tel26_item_id(w) == 1 && data != get_screen_param(0)));
    tel28_click_compare("choose friendly teams screen input", widget, pure, || o(widget), || choose_friendly_teams_screen_input(widget))
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
pub(crate) unsafe extern "C" fn open_find_screen(table: u32) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x5a2e60usize, 0x5a45c0usize, 0, 0x5a4eb0usize, 0) == 0 {
        return;
    }
    covered!("tel34_6 open find screen");
    set_screen_param(0, table as u8 as i8 as isize as usize);
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
    let table = get_screen_param(0) as u8 as i8;
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
    exe::gui_add_tab_bar(entries.as_mut_ptr() as usize, 4, table as i32, &mut top, &mut bottom, 1);
    let picture = |address: usize| {
        let f: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(address);
        f(0, 0, 0);
    };
    // the result tabs by their exe addresses: the 1080 exe has its own (27 rows per page), left unhooked there
    let results = |address: usize| {
        let f: unsafe extern "C" fn(i32, i32) = core::mem::transmute(address);
        f(top, bottom);
    };
    match table {
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
        let nation = Nation(read_usize(team + club::NATION));
        if !nation.is_null() && nation.continent() != 0 {
            let continent = nation.continent();
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
            exe::drop_searchengine(engine);
            exe::crt_operator_delete(engine);
        }
        set_screen_param(k, 0);
    }
}

/// Comparer for `choose_friendly_teams_other_clubs_input`: side by side over the screen parameters and the answer.
pub(crate) unsafe extern "C" fn compare_choose_friendly_teams_other_clubs_input(raw: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_CHOOSE_FRIENDLY_TEAMS_OTHER_CLUBS_INPUT);
    let regions = screen_param_regions();
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(raw), || choose_friendly_teams_other_clubs_input(raw));
    SCOREBOARD.record(equal && ro == rr, || format!("Choose Friendly Teams (choose_friendly_teams_other_clubs_input): answer {} x {}, differences in {:?}",
        ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `open_find_screen`.
pub(crate) unsafe extern "C" fn compare_open_find_screen(table: u32) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32)>(IDX_OPEN_FIND_SCREEN)(table)
}

/// Comparer for `draw_find_screen`: the Search For screen side by side over the display list, when every tab already
/// has its search engine and none has a search pending (a first build allocates the engine and searches: only the
/// original runs).
pub(crate) unsafe extern "C" fn compare_draw_find_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_DRAW_FIND_SCREEN);
    let table = get_screen_param(0) as u8 as i8;
    let base = match table {
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

/// Comparer for `draw_find_nations_results`.
pub(crate) unsafe extern "C" fn compare_draw_find_nations_results(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(IDX_DRAW_FIND_NATIONS_RESULTS);
    tel34_compare_find_results(&FIND_NATIONS, "Search For screen (draw_find_nations_results)", || o(top, bottom), || draw_find_nations_results(top, bottom));
}

/// Comparer for `draw_find_clubs_results`.
pub(crate) unsafe extern "C" fn compare_draw_find_clubs_results(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(IDX_DRAW_FIND_CLUBS_RESULTS);
    tel34_compare_find_results(&FIND_CLUBS, "Search For screen (draw_find_clubs_results)", || o(top, bottom), || draw_find_clubs_results(top, bottom));
}

/// Comparer for `draw_find_non_players_results`.
pub(crate) unsafe extern "C" fn compare_draw_find_non_players_results(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(IDX_DRAW_FIND_NON_PLAYERS_RESULTS);
    tel34_compare_find_results(&FIND_NON_PLAYERS, "Search For screen (draw_find_non_players_results)", || o(top, bottom),
        || draw_find_non_players_results(top, bottom));
}

/// Comparer for `draw_find_players_results`.
pub(crate) unsafe extern "C" fn compare_draw_find_players_results(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(IDX_DRAW_FIND_PLAYERS_RESULTS);
    tel34_compare_find_results(&FIND_PLAYERS, "Search For screen (draw_find_players_results)", || o(top, bottom), || draw_find_players_results(top, bottom));
}

/// Comparer for `find_screen_input`.
pub(crate) unsafe extern "C" fn compare_find_screen_input(raw: u32) -> i32 {
    // simplification: the menu bar commands (they open screens) do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_FIND_SCREEN_INPUT)(raw)
}

/// Comparer for `find_screen_non_players_input`.
pub(crate) unsafe extern "C" fn compare_find_screen_non_players_input(raw: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> u32>(IDX_FIND_SCREEN_NON_PLAYERS_INPUT);
    tel34_compare_tab_input("find_screen_non_players_input", || o(raw), || find_screen_non_players_input(raw))
}

/// Comparer for `find_screen_players_input`.
pub(crate) unsafe extern "C" fn compare_find_screen_players_input(raw: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> u32>(IDX_FIND_SCREEN_PLAYERS_INPUT);
    tel34_compare_tab_input("find_screen_players_input", || o(raw), || find_screen_players_input(raw))
}

/// Comparer for `release_find_screen`: only the original runs (it frees the engines), the parameters checked against
/// the plan afterwards (2, 8, 0xe and 0x14 cleared, the others as they were).
pub(crate) unsafe extern "C" fn compare_release_find_screen() {
    let mut plan = tel34_find_params();
    for k in [2usize, 8, 0xe, 0x14] {
        plan[k] = 0;
    }
    original_fn::<unsafe extern "C" fn()>(IDX_RELEASE_FIND_SCREEN)();
    let got = tel34_find_params();
    SCOREBOARD.record(got == plan, || format!("Search For screen (release_find_screen): params {:x?} x plan {:x?}", got, plan));
}

/// The 20 parameters of the Choose Friendly Teams screen as `open_choose_friendly_teams_screen` leaves them.
pub(crate) fn find_screen_planned_params(table: u32, continent: usize, nation: usize, mode: u32) -> [usize; 0x14] {
    let mut p = [0usize; 0x14];
    p[0] = table as u8 as i8 as isize as usize;
    p[1] = continent;
    p[4] = continent;
    p[7] = continent;
    p[0xa] = nation;
    p[0xd] = nation;
    p[0x10] = continent;
    p[0x13] = mode as u8 as i8 as isize as usize;
    p
}

/// The selector button of a friendly list tab ("Continent" or "Nation" at `label`) at the top, its popup area (flags
/// 0x130, the popup colour, owned by the button) and the popup's entries (`filler`: 0x765010 the continents, 0x7651e0
/// the nations; cdecl(area, code, the chosen record)).
pub(crate) unsafe fn find_tab_selector(top: i32, label: usize, filler: usize, code: u32, record: usize) {
    tel28_lang()(JOB_TEXT, label);
    let button = tel26_add_button([0x6e, top as u32, 0xeb, (top + 0x14) as u32, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0, 0]);
    let popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), button]);
    let fill: unsafe extern "C" fn(u32, u32, usize) = core::mem::transmute(filler);
    fill(popup, code, record);
}

/// The heading of a friendly list tab: `fmt` with the gender byte twice and the adjective, a text item in the
/// highlight colour font 3 from `y` to `y` + 0x23.
pub(crate) unsafe fn find_tab_heading(y: i32, fmt: usize, gender: u8, adjective: &[u8; 0x64]) {
    let g = gender as i8 as i32;
    tel28_lang()(JOB_TEXT, fmt, g, g, adjective.as_ptr());
    tel28_text_item([0x6e, y as u32, 0x30c, (y + 0x23) as u32, 2, 0, 0xc, 3, job_colour(JOB_COLOUR_BDEC), JOB_TEXT as u32, 1]);
}

/// The rows of a two-column-pair list (Nations, Under-21s): per club its cell and its continent's three-letter name
/// (highlight colour), two clubs per row.
pub(crate) unsafe fn find_rows_with_continent(area: u32, list: usize, n: i32) {
    let (mut column, mut row) = (0i32, 0i32);
    for k in 0..n.max(0) as usize {
        let club = Club(read_usize(list + k * 4));
        find_club_cell(area, column, row, club.addr());
        let continent = read_usize(club.nation().addr() + nation::CONTINENT);
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
pub(crate) unsafe fn find_rows_with_nation(area: u32, list: usize, n: i32, division: bool) {
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
pub(crate) unsafe fn find_rows_dword(bottom: i32, y: i32, over: u32) -> u32 {
    let rows = ((bottom - y - 0x10) / exe::font_get_height(3)) as u8 as u32;
    (over & 0xffff_ff00) | rows
}

/// Restores a list's scroll row (parameter `row_param`) on the current display's area (when not -1) and keeps the
/// area in parameter `area_param`, as the tabs do inline.
pub(crate) unsafe fn find_restore_scroll(area: u32, row_param: u32, area_param: u32) {
    let row = get_screen_param(row_param);
    if area as i16 != -1 {
        exe::area_set_scroll_row(current_human_display() + 4 + (area as i16 as isize as usize).wrapping_mul(0xbf1), row);
    }
    set_screen_param(area_param, area as i16 as isize as usize);
}

/// The friendly list buffer (one pointer per club, `crt_malloc`); none: the find_screens.cpp box at `line`.
pub(crate) unsafe fn find_list_alloc(line: u32) -> usize {
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
pub(crate) unsafe fn find_nations_tab(top: i32, bottom: i32) {
    let picture: unsafe extern "C" fn(u32, u32, u32) = core::mem::transmute(FN_BACKGROUND_SET_NATION_PICTURE);
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
        let club = Club(read_usize(G_CLUB_TABLE) + i * club::SIZE);
        if exe::database_is_national_team(club) != 0 {
            let nation = club.nation();
            if !nation.is_null() && nation.continent() == continent {
                write_usize(list + n as usize * 4, club.addr());
                n += 1;
            }
        }
    }
    exe::crt_qsort(list, n as u32, 4, FIND_SORT_BY_NAME);
    let rows = find_rows_dword(bottom, y, bottom as u32);
    let area = exe::screenmanager_add_area(G_SCREEN_MANAGER, (0x6e) as u32 as usize, (y as u32) as u32 as usize, (0x30c) as u32 as usize, (bottom as u32) as u32 as usize, (4) as u32 as usize, (weights.as_ptr() as u32) as u32 as usize, (rows) as u32 as usize, (0) as u32 as usize, (2) as u32 as usize, (0) as u32 as usize, (u32::MAX) as u32 as usize) as u32;
    find_rows_with_continent(area, list, n);
    exe::crt_free(list);
    exe::screenmanager_set_area_scroll_row(G_SCREEN_MANAGER, area, get_screen_param(2) as u32);
    set_screen_param(3, area as i16 as isize as usize);
}

/// Whether building the Choose Friendly Teams screen would show an error box (the open tab's continent or nation
/// missing): those builds run the original only.
pub(crate) unsafe fn find_screen_build_has_box() -> bool {
    let table = if get_screen_param(0x13) as u8 == 2 { 1 } else { get_screen_param(0) as u8 };
    match table {
        1 => get_screen_param(1) == 0,
        2 => get_screen_param(4) == 0,
        3 => get_screen_param(7) == 0,
        6 => get_screen_param(0x10) == 0,
        _ => false,
    }
}

/// The best quarter of a sorted list, at most 100: `_ftol(min(n × 0.25, 100.0))`.
pub(crate) unsafe fn find_best_quarter(n: i32) -> i32 {
    let x = n as f64 * read_f64(rdata::F64_0_25);
    let cap = read_f64(rdata::F64_100);
    (if x < cap { x } else { cap }) as i32
}

/// The pick of the Choose Friendly Teams tabs 1 to 5: the scroll row and area parameters, the parameter the pick
/// goes to, the widget id and whether the id is the action id (0x468b40) rather than the widget id.
pub(crate) fn find_tab_pick(table: i8) -> Option<(u32, u32, u32, i32, bool)> {
    match table {
        1 => Some((2, 3, 1, 2, false)),
        2 => Some((5, 6, 4, 3, false)),
        3 => Some((8, 9, 7, 4, false)),
        4 => Some((0xb, 0xc, 0xa, 5, true)),
        5 => Some((0xe, 0xf, 0xd, 6, true)),
        _ => None,
    }
}

/// `ScreenManager::get_widget_action_id` (0x468b40, thiscall, ret 4): the widget's id, or the default command id.
pub(crate) unsafe fn find_action_id(widget: u32) -> i32 {
    exe::screenmanager_get_widget_action_id(G_SCREEN_MANAGER, widget)
}

/// A cell of a result row in the list area: `add_area_text` in column `column` of row `row`, the result style
/// (1, 0, 1, 2) in the colour word at `colour`.
pub(crate) unsafe fn tel34_find_cell_text(area: u32, column: u32, row: u32, colour: usize, text: usize) {
    tel26_area_text([area, column, row, 1, 0, 1, 2, job_colour(colour), text as u32]);
}

/// A linked cell of a result row: `add_area_button` in column `column` of row `row` (style 1, 0, 1, `font`) with the
/// link id and object.
pub(crate) unsafe fn tel34_find_cell_link(area: u32, column: u32, row: u32, font: u32, colour: u32, text: usize, id: u32, object: usize) {
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
pub(crate) unsafe fn tel34_find_results(table: &Tel34FindTab, top: i32, bottom: i32, mut row: impl FnMut(u32, u32, usize)) {
    let weights: [u8; 3] = [3, 2, 2];
    let text = get_screen_param(table.base + 1);
    let mut engine = get_screen_param(table.base + 2);
    get_screen_param(0x19);
    set_screen_param(0x19, 0);
    if engine == 0 {
        let memory = exe::crt_operator_new(SEARCH_ENGINE_SIZE);
        engine = if memory != 0 {
            exe::new_searchengine(memory)
        } else {
            0
        };
        set_screen_param(table.base + 2, engine);
        set_screen_param(table.base + 3, 1);
    }
    let start = get_screen_param(table.base + 4) as i32;
    let edit_area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x168, top.wrapping_add(0x14) as u32, 2, 0, 1, 0, 1, 0, u32::MAX]);
    tel34_lang_text(0x9ca0f0); // "Search For"
    tel26_area_text([edit_area, 0, 0, 2, 0, 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);
    tel26_add_item(current_human_display(), [table.edit_kind, 0, 0, 0, 0, 1, 0, 2, 0, 0, 0xc, 1, job_colour(JOB_COLOUR_BDE8), text as u32, 0x19,
        table.edit_id, 0, edit_area]);
    if get_screen_param(table.base + 3) != 0 {
        covered!("tel34_6 search");
        let search: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(table.search);
        search(engine, text, table.mode);
        set_screen_param(table.base + 3, 0);
    }
    let count = read_i32(engine + SEARCH_ENGINE_COUNT);
    let paged = count > 100;
    if paged {
        covered!("tel34_6 page buttons");
        type PageButtons = unsafe extern "C" fn(i32, i32, i32, i32, u32, u32, u32, u32) -> i32;
        let pages: PageButtons = core::mem::transmute(FN_GUI_ADD_PAGE_BUTTONS);
        pages(0x30c, top, start / 16 + 1, count / 16 + (count % 16 != 0) as i32, 2, 3, 4, 5);
    }
    let lang: Tel34FormatFn = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if read_u8(text) != 0 {
        lang(JOB_TEXT, table.matching, text);
    } else {
        lang(JOB_TEXT, table.all);
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
    let scroll = get_screen_param(table.base + 5);
    if area as i16 != -1 {
        exe::area_set_scroll_row(display_area_at(current_human_display(), area as i16), scroll);
    }
    set_screen_param(table.base + 6, area as i16 as isize as usize);
}

/// The first cell of a club result row: the club's short name ("  %s") linked to the club (id 0x7d5) in the
/// highlight colour when a human manages it (`human_manager_is_human_club`, 0, 1, 0, 0), otherwise in the text colour.
pub(crate) unsafe fn tel34_find_club_name(area: u32, row: u32, club: usize) {
    let club = Club(club);
    let crt: Tel34FormatFn = core::mem::transmute(FN_CRT_SPRINTF);
    let human = exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0;
    crt(JOB_TEXT, 0x987ff0, club.addr() + club::SHORT_NAME); // "  %s"
    let colour = if human { job_colour(JOB_COLOUR_BDEC) } else { job_colour(JOB_COLOUR_3184) };
    tel34_find_cell_link(area, 0, row, 3, colour, JOB_TEXT, 0x7d5, club.addr());
}

/// The division cell (column 2) of a club: the division's short name linked to it (id 2000) when the competition is
/// loaded (an object in [`G_COMP_OBJECT_TABLE`]), as plain text otherwise; "Unknown Division" without one. With
/// `lower_division`, a division whose nation's league is selected (`nation::LEAGUE_SELECTED`) but not loaded shows "A
/// lower division" (`KEY_CLUB_COMP_A_LOWER_DIVISION`'s short name).
pub(crate) unsafe fn tel34_find_division_cell(area: u32, row: u32, club: usize, lower_division: bool) {
    let club = Club(club);
    let division = club.division().addr();
    if division == 0 {
        tel34_lang_text(0x9ca0fc); // "Unknown Division"
        tel34_find_cell_text(area, 2, row, JOB_COLOUR_BDD4, JOB_TEXT);
        return;
    }
    let loaded = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_u32(division) as usize).wrapping_mul(4))) != 0;
    if lower_division {
        let nation = Nation(read_usize(division + competition::NATION));
        if !nation.is_null() && nation.league_selected() != 0 && !loaded {
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
pub(crate) unsafe fn tel34_find_staff_row(area: u32, row: u32, person: usize, club: usize) {
    let club = Club(club);
    let crt: Tel34FormatFn = core::mem::transmute(FN_CRT_SPRINTF);
    let mut name = [0u8; 0x7d0];
    let human = !club.is_null() && exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0;
    exe::database_format_staff_name(person, name.as_mut_ptr() as usize, 0x7d0, 3);
    crt(JOB_TEXT, 0x987ff0, name.as_ptr()); // "  %s"
    let colour = if human { job_colour(JOB_COLOUR_BDEC) } else { job_colour(JOB_COLOUR_3184) };
    tel34_find_cell_link(area, 0, row, 3, colour, JOB_TEXT, 0x7d7, person);
    if club.is_null() {
        tel34_lang_text(0x9ca15c); // "No Club"
        tel34_find_cell_text(area, 1, row, JOB_COLOUR_BDEC, JOB_TEXT);
        tel34_lang_text(0x9ca150); // "No Division"
        tel34_find_cell_text(area, 2, row, JOB_COLOUR_BDD4, JOB_TEXT);
        return;
    }
    tel34_find_cell_link(area, 1, row, 2, job_colour(JOB_COLOUR_BDEC), club.addr() + club::SHORT_NAME, 0x7d5, club.addr());
    tel34_find_division_cell(area, row, club.addr(), false);
}

/// The input of a result tab: for an item (not -1) the scroll row (+ 5) = the area's (+ 6); the tab's edit box id
/// (search) sets "search again" (+ 3) and clears the first row (+ 4) and the scroll (+ 5); the page ids move the first
/// row (+ 4): 2 back 160, 3 back 16 (below 0: up to the first page boundary at or above 0), 4 on 16, 5 on 160 (past
/// the last found: back by whole pages of 16 until it is at or before it). Each answers -11; anything else 0.
pub(crate) unsafe fn tel34_find_tab_input(raw: u32, base: u32, edit_id: u16) -> i32 {
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

/// The parameters a find screen routine may write: the whole parameter block of the current screen (0x1a dwords).
pub(crate) unsafe fn tel34_find_params() -> [usize; 0x1a] {
    let mut p = [0usize; 0x1a];
    for (k, v) in p.iter_mut().enumerate() {
        *v = get_screen_param(k as u32);
    }
    p
}

/// A result tab's comparer: side by side over the display list when the tab has its engine and no search pending;
/// otherwise only the original runs (allocation and search).
pub(crate) unsafe fn tel34_compare_find_results(table: &Tel34FindTab, name: &str, original: impl FnOnce(), rust: impl FnOnce()) {
    if get_screen_param(table.base + 2) == 0 || get_screen_param(table.base + 3) != 0 {
        // simplification: making the engine and searching do not run twice; proven in Rust mode
        return original();
    }
    compare_by_display_list_with(name, &[], original, rust);
}

/// A tab input's comparer: side by side over the screen parameters and the answer.
pub(crate) unsafe fn tel34_compare_tab_input(name: &str, original: impl FnOnce() -> u32, rust: impl FnOnce() -> u32) -> u32 {
    let regions = screen_param_regions();
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, original, rust);
    SCOREBOARD.record(equal && ro == rr, || format!("Search For screen ({}): answer {} x {}, differences in {:?}", name, ro as i32, rr as i32,
        first_differences(&mo, &mr)));
    ro
}

/// The club cell of a friendly list: "  short name" as a button (id 0x7d5, the club), in the highlight colour for a
/// human's club, else pale grey.
pub(crate) unsafe fn find_club_cell(area: u32, column: i32, row: i32, club: usize) {
    let club = Club(club);
    let colour = if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 { job_colour(JOB_COLOUR_BDEC) } else { job_colour(JOB_COLOUR_3184) };
    tel28_crt()(JOB_TEXT, 0x987ff0, club.addr() + club::SHORT_NAME); // "  %s"
    tel26_area_button([area, column as u32, row as u32, 1, 0, 1, 3, colour, JOB_TEXT as u32, 0x7d5, club.addr() as u32]);
}

/// The nation cell of a friendly list: the club's nation's three-letter name as a button (id 0x7d5, the national team
/// from `database_get_national_team`).
pub(crate) unsafe fn find_nation_cell(area: u32, column: i32, row: i32, club: usize) {
    let club = Club(club);
    let national = exe::database_get_national_team(club.nation().addr());
    let nation = club.nation();
    tel26_area_button([area, (column + 1) as u32, row as u32, 1, 0, 0xc, 2, job_colour(JOB_COLOUR_BDEC), (nation.addr() + nation::THREE_LETTER_NAME) as u32, 0x7d5, national as u32]);
}

/// The division cell of a friendly list: with a division, its three-letter name as a button (id 0x7d0, the
/// competition) when the competition table has an object for it, else as text (grey-blue).
pub(crate) unsafe fn find_division_cell(area: u32, column: i32, row: i32, club: usize) {
    let club = Club(club);
    let division = club.division().addr();
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

/// `ScreenManager::add_text_item` (0x803fd0, thiscall, ret 0x2c): x1, y1, x2, y2, the style, the colour, the text
/// flags, the font, the text colour, the text and an unused dword.
pub(crate) unsafe fn tel34_text_item(f: [u32; 11]) -> u32 {
    type TextItem = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: TextItem = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    g(G_SCREEN_MANAGER, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10])
}

/// "E:\dev\CM3\cm3 00-01\cm3\code\find_screens.cpp", the path the find_screens.cpp error boxes print.
pub(crate) const FIND_SCREENS_CPP: usize = 0x9c9eb4;

/// A MenuItem of the tab bar (0xd4 bytes): word +0 = -1 and word +2 = 0 (MenuItem::MenuItem 0x4568f0), the text at
/// +4, the kind at +0xcc and the value at +0xd0.
pub(crate) const FIND_SCREEN_MENU_ITEM: usize = 0xd4;

/// A continent: its gender byte (+0x1e) and its three-letter name (+0x1f) (docs/FORMATOS TContinent).
pub(crate) const FIND_CONTINENT_GENDER: usize = 0x1e;

pub(crate) const FIND_CONTINENT_THREE_LETTER: usize = 0x1f;

/// A competition's three-letter name (+0x53) [inference: between the short gender byte +0x52 and the continent +0x59].
pub(crate) const FIND_COMP_THREE_LETTER: usize = 0x53;

/// The comparators of the friendly lists: by reputation (0x53ffe0) and by name (0x540080), qsort callbacks of 4-byte
/// club pointers [inference: from how the Major Clubs tab keeps the best quarter].
pub(crate) const FIND_SORT_BY_REPUTATION: usize = 0x53ffe0;

pub(crate) const FIND_SORT_BY_NAME: usize = 0x540080;

/// The two-column weights of the club lists (club, place) and the six-column ones (club, nation, division).
pub(crate) const FIND_WEIGHTS_4: [u8; 4] = [5, 1, 5, 1];

pub(crate) const FIND_WEIGHTS_6: [u8; 6] = [5, 1, 1, 5, 1, 1];

/// The search engine of a Search For tab (`SearchEngine`, 0x3c bytes): the found records (+0x34, an array of record
/// pointers) and their count (+0x38).
pub(crate) const SEARCH_ENGINE_SIZE: u32 = 0x3c;

pub(crate) const SEARCH_ENGINE_RESULTS: usize = 0x34;

pub(crate) const SEARCH_ENGINE_COUNT: usize = 0x38;

/// A continent record: its name (+4) and its federation's name (+0xa3), as the nations tab shows them [inference].
pub(crate) const CONTINENT_NAME: usize = 0x4;

pub(crate) const CONTINENT_FEDERATION_NAME: usize = 0xa3;

/// A tab bar entry (`MenuItem`, 0xd4 bytes): the text at +4, the kind at +0xcc and the value at +0xd0.
pub(crate) const FIND_TAB_ENTRY_SIZE: usize = 0xd4;

/// The size of a competition record of `G_CLUB_COMP_TABLE` (107 bytes).
pub(crate) const FIND_COMP_RECORD_SIZE: usize = 0x6b;

pub(crate) const FIND_NATIONS: Tel34FindTab = Tel34FindTab { base: 0, edit_kind: 0x880018, edit_id: 6, search: 0x804fe0, mode: 1, matching: 0x9ca0c8, all: 0x9ca0bc };

pub(crate) const FIND_CLUBS: Tel34FindTab = Tel34FindTab { base: 6, edit_kind: 0x480018, edit_id: 7, search: 0x804fe0, mode: 0, matching: 0x9ca12c, all: 0x9ca120 };

pub(crate) const FIND_NON_PLAYERS: Tel34FindTab = Tel34FindTab { base: 0xc, edit_kind: 0x2380018, edit_id: 8, search: 0x805310, mode: 1, matching: 0x9ca174, all: 0x9ca164 };

pub(crate) const FIND_PLAYERS: Tel34FindTab = Tel34FindTab { base: 0x12, edit_kind: 0x1380018, edit_id: 9, search: 0x805310, mode: 0, matching: 0x9ca1ac, all: 0x9ca1a0 };

/// One of the four tabs of the Search For screen: its first parameter (`base`: the search text is `base` + 1, the
/// search engine + 2, the "search again" flag + 3, the first row shown + 4, the scroll row + 5, the list area + 6), the
/// edit box's kind and id, the search (`SearchEngine::find_clubs` 0x804fe0 or `find_staff` 0x805310) and its mode, and
/// the headings "<things> Matching ..." and "All <things>".
pub(crate) struct Tel34FindTab {
    pub(crate) base: u32,
    pub(crate) edit_kind: u32,
    pub(crate) edit_id: u32,
    pub(crate) search: usize,
    pub(crate) mode: u32,
    pub(crate) matching: usize,
    pub(crate) all: usize,
}
