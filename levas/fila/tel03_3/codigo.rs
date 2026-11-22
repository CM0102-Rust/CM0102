// ---------- batch %%LEVA%%: the club history screen's tabs (club_screens.cpp, part 3): competitions and the season tabs ----------

/// The shared text buffer the screens format into (0xde1f64).
const HISTORY_TAB_TEXT: usize = 0xde1f64;

/// `ScreenManager::add_area_text` (0x804100, thiscall, ret 0x24) of the club history tabs: cell (`col`, `row`) of area
/// `area` with the two layout words, the font, the style, the colour and the text (0 = an empty cell).
unsafe fn history_tab_cell(area: usize, col: u32, row: i32, a4: u32, a5: usize, font: u32, style: u32, colour: usize, text: usize) -> usize {
    let f: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    f(G_SCREEN_MANAGER, area, col as usize, row as usize, a4 as usize, a5, font as usize, style as usize, colour, text)
}

/// `ScreenManager::add_area_button` (0x8040a0, thiscall, ret 0x2c): a cell that links (`link`, `data`).
unsafe fn history_tab_link_cell(area: usize, col: u32, row: i32, colour: usize, text: usize, link: u32, data: usize) -> usize {
    let f: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    f(G_SCREEN_MANAGER, area, col as usize, row as usize, 2, 0, 1, 2, colour, text, link as usize, data)
}

/// `Display::add_item` (0x55f990, thiscall, ret 0x48) of one entry of a View popup (`popup`): the text in `HISTORY_TAB_TEXT`, the
/// flags `flags` (1 = can be picked, 0x21 = greyed out), the link `link` with `data`.
unsafe fn history_tab_menu_entry(popup: usize, flags: u32, link: u32, data: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize = core::mem::transmute(0x55f990usize);
    let menu = colour_word(0xae31aa);
    f(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, menu, colour_word(G_GUI_COLOUR_YELLOW), flags as usize, 1, colour_word(G_GUI_COLOUR_BLACK), HISTORY_TAB_TEXT, 0, link as usize, data, popup)
}

/// The View button of a club history tab (from `top`) and its popup area (`Display::add_area` 0x130 with the menu colour
/// 0xae31aa, under the button); returns the popup.
unsafe fn history_tab_view_button(top: i32) -> usize {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let button_item: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    lang(HISTORY_TAB_TEXT, 0x988004); // "View"
    let b = button_item(G_SCREEN_MANAGER, 0x6e, top as usize, 0xeb, (top + 0x14) as usize, 0x30, colour_word(G_GUI_COLOUR_GREY), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), HISTORY_TAB_TEXT, 0, 0);
    add_area(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x130, colour_word(0xae31aa), b)
}

/// The CRT's inline strcpy of the original into a local buffer.
unsafe fn history_tab_copy(dst: &mut [u8], src: usize) {
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

/// `ScreenManager::add_text_item` of a tab's heading (the chosen view) from `y`.
unsafe fn history_tab_heading(y: i32, heading: usize) {
    let text_item: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    text_item(G_SCREEN_MANAGER, 0x6e, y as usize, 0x30c, (y + 0x23) as usize, 2, 0, 0xc, 3, colour_word(G_GUI_COLOUR_YELLOW), heading, 1);
}

/// The inline `Display::set_area_scroll_row` of the tabs: the scroll row of param `scroll_param` (read first) goes to
/// the area (nothing for −1), and the area goes to param `area_param`.
unsafe fn history_tab_scroll(area: usize, scroll_param: u32, area_param: u32) {
    let row = get_screen_param(scroll_param);
    if area as i16 != -1 {
        exe::area_set_scroll_row((current_human_display() as isize + (area as i16) as isize * 0xbf1 + 4) as usize, row);
    }
    set_screen_param(area_param, area as i16 as isize as usize);
}

/// The record of competition `id` (0x6b bytes in `G_CLUB_COMP_TABLE`).
unsafe fn history_tab_comp(id: usize) -> usize {
    read_usize(G_CLUB_COMP_TABLE) + id.wrapping_mul(0x6b)
}

/// Whether competition `id` has its object loaded (`G_COMP_OBJECT_TABLE`).
unsafe fn history_tab_comp_loaded(id: usize) -> bool {
    read_usize(read_usize(G_COMP_OBJECT_TABLE) + id.wrapping_mul(4)) != 0
}

/// The competitions of the View popup of the Competitions tab from a list of competition ids (`list`, an LLIST of ids,
/// walked from its head): one entry per competition (its short name, link 3 with its id); the one equal to `view` gives
/// the heading.
unsafe fn history_competitions_menu_list(list: usize, popup: usize, view: i32, heading: &mut [u8]) {
    write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
    let mut node = exe::llist_get_next(list);
    while node != 0 {
        let comp = history_tab_comp(read_usize(node));
        history_tab_copy(core::slice::from_raw_parts_mut(HISTORY_TAB_TEXT as *mut u8, 0x100), comp + competition::SHORT_NAME);
        history_tab_menu_entry(popup, 1, 3, read_usize(comp));
        if view == read_i32(comp) {
            history_tab_copy(heading, HISTORY_TAB_TEXT);
        }
        node = exe::llist_get_next(list);
    }
}

/// The cup runs view of the Competitions tab (one cup, `view`): one row per run of the club in that cup (the cup history
/// list, param 0x1b): the season (`key_nation_format_club_season`, two years on for the World Cup qualifying sections
/// and the European Championship qualifying) and the round reached, as the competition object writes it (its virtual
/// method +0x38 with the round, the stage and the flag of the run), "-" when it writes nothing. Empty rows fill the
/// rest. Returns the area.
/// Original: part of `club_history_draw_competitions_tab` (0x46adc0, club_screens.cpp).
unsafe fn history_competitions_cup_runs(club: usize, view: i32, top: i32, bottom: i32, rows: u32, weights: &[u8; 5], cups: usize) -> usize {
    let season: unsafe extern "C" fn(u32, usize, usize, u32) -> i32 = core::mem::transmute(0x669630usize);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let area = add_area(current_human_display(), 0x6e, top as usize, 0x30c, bottom as usize, 2, weights.as_ptr() as usize, rows as usize, 0, 1, 0, usize::MAX);
    let pale = colour_word(G_GUI_COLOUR_PALE_GREY);
    let mut r: i32 = 0;
    let mut round = [0u8; 0x7d0];
    if cups != 0 {
        write_u32(cups + llist::CURSOR, read_u32(cups + llist::HEAD));
        let mut node = exe::llist_get_next(cups);
        while node != 0 {
            if read_i32(node) == view {
                let mut year = read_u16(node + 4);
                let qualifying = [
                    KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION,
                    KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION,
                    KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION,
                    KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT,
                    KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION,
                    KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION,
                    KEY_CLUB_COMP_EUROPEAN_CHAMPIONSHIP_QUALIFYING,
                ];
                if qualifying.iter().any(|&k| view == read_i32(k)) {
                    year = year.wrapping_add(2);
                }
                season(year as u32, club, HISTORY_TAB_TEXT, 0x7d0);
                history_tab_cell(area, 0, r, 0x30, colour_word(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, pale, HISTORY_TAB_TEXT);
                let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_usize(node) * 4);
                let method: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, usize) = core::mem::transmute(read_usize(read_usize(object) + 0x38));
                // the original pushes the byte and the words in the club pointer's register: its high bytes go along
                let high = club as u32 & 0xffff_0000;
                let flag = (club as u32 & 0xffff_ff00) | read_u8(node + 6) as u32;
                method(object, round.as_mut_ptr() as usize, 0x7d0, high | read_u16(node + 7) as u32, high | read_u16(node + 9) as u32, flag, club);
                let shown = if round[0] != 0 { round.as_ptr() as usize } else { 0x98ba0c }; // "-"
                format(HISTORY_TAB_TEXT, 0x987ff0, shown); // "  %s"
                history_tab_cell(area, 1, r, 2, 0, 1, 2, colour_word(G_GUI_COLOUR_YELLOW), HISTORY_TAB_TEXT);
                r += 1;
            }
            node = exe::llist_get_next(cups);
        }
    }
    while r < rows as i32 {
        history_tab_cell(area, 0, r, 2, 0, 1, 2, pale, 0);
        history_tab_cell(area, 1, r, 2, 0, 1, 2, pale, 0);
        r += 1;
    }
    area
}

/// Where a club finished in an honours row (0x1a-byte competition history record: +0xa the winner, +0xe the runner-up):
/// 0 winner, 1 runner-up, 2 third, counting the partner club too (West Germany's national team for Germany's).
unsafe fn history_honours_place(row: usize, club: usize, partner: usize) -> u8 {
    let is = |c: usize| c == club || (partner != 0 && c == partner);
    if is(read_usize(row + 0xa)) {
        0
    } else if is(read_usize(row + 0xe)) {
        1
    } else {
        2
    }
}

/// The honours view of the Competitions tab: every first, second or third place of the club (and, for Germany's
/// national team, of West Germany's: `comp_util_collect_club_honours`), in three columns: the competition (linked 0x7d0
/// when its object is loaded; only on its first row), the place ("Winners", "Runners Up", "Third Placed") and the
/// years, up to 8 a row (", %d"), the same competition and place running on in new rows. Empty rows fill the rest.
/// Returns the area.
/// Original: part of `club_history_draw_competitions_tab` (0x46adc0, club_screens.cpp).
unsafe fn history_competitions_honours(club: usize, top: i32, bottom: i32, rows: u32, weights: &[u8; 5]) -> usize {
    let collect: unsafe extern "C" fn(usize, usize, *mut usize) -> i32 = core::mem::transmute(0x4c0140usize);
    let clear: unsafe extern "thiscall" fn(*mut usize) -> u32 = core::mem::transmute(0x4afa40usize);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0);
    let mut previous: usize = 0;
    let mut partner: usize = 0;
    if exe::database_is_national_team(club) != 0 {
        let nation = read_usize(club + club::NATION);
        if nation != 0 && read_i32(nation) == read_i32(KEY_NATION_GERMANY) && exe::club_is_national_team(club) == 0 {
            covered!("tel03_3 honours with west germany");
            partner = exe::database_get_national_team(read_usize(G_NATION_TABLE) + read_usize(KEY_NATION_WEST_GERMANY) * 0x122);
        }
    }
    let mut rows_of: usize = 0;
    let count = collect(club, partner, &mut rows_of);
    let area = add_area(current_human_display(), 0x6e, top as usize, 0x30c, bottom as usize, 3, weights.as_ptr() as usize + 2, rows as usize, 0, 1, 0, usize::MAX);
    let pale = colour_word(G_GUI_COLOUR_PALE_GREY);
    let yellow = colour_word(G_GUI_COLOUR_YELLOW);
    let mut r: i32 = 0;
    let mut i: i32 = 0;
    let mut row = rows_of;
    let mut more = [0u8; 0x7d0];
    while i < count {
        clear(t);
        let comp = read_usize(row + 4);
        let first = row;
        if previous != comp {
            format(HISTORY_TAB_TEXT, 0x987ff0, comp + competition::SHORT_NAME); // "  %s"
            if history_tab_comp_loaded(read_usize(comp)) {
                history_tab_link_cell(area, 0, r, pale, HISTORY_TAB_TEXT, 0x7d0, comp);
            } else {
                history_tab_cell(area, 0, r, 2, 0, 1, 2, pale, HISTORY_TAB_TEXT);
            }
        } else {
            history_tab_cell(area, 0, r, 2, 0, 1, 2, pale, 0);
        }
        previous = comp;
        let place = history_honours_place(row, club, partner);
        lang(HISTORY_TAB_TEXT, [0x990b50usize, 0x990b5c, 0x990b6c][place as usize]); // "  Winners", "  Runners Up", "  Third Placed"
        history_tab_cell(area, 1, r, 2, 0, 1, 2, pale, HISTORY_TAB_TEXT);
        format(HISTORY_TAB_TEXT, 0x990b48, read_i16(row + 8) as i32); // "  %d"
        exe::text_set_string(t as usize, HISTORY_TAB_TEXT as *const u8);
        i += 1;
        row += 0x1a;
        let mut years: i16 = 1;
        while i < count {
            if read_usize(row + 4) != read_usize(first + 4) || history_honours_place(row, club, partner) != place {
                break;
            }
            if years < 8 { // tel03_3 up to 8 years a row
                format(more.as_mut_ptr() as usize, 0x990b40, read_i16(row + 8) as i32); // ", %d"
                let mut n = 0;
                while read_u8(HISTORY_TAB_TEXT + n) != 0 {
                    n += 1;
                }
                let mut k = 0;
                loop {
                    write_u8(HISTORY_TAB_TEXT + n + k, more[k]);
                    if more[k] == 0 {
                        break;
                    }
                    k += 1;
                }
                years += 1;
            } else {
                covered!("tel03_3 honours years on a new row");
                history_tab_cell(area, 2, r, 2, 0, 1, 2, yellow, HISTORY_TAB_TEXT);
                format(HISTORY_TAB_TEXT, 0x990b48, read_i16(row + 8) as i32); // "  %d"
                r += 1;
                years = 1;
                history_tab_cell(area, 0, r, 2, 0, 1, 2, yellow, 0);
                history_tab_cell(area, 1, r, 2, 0, 1, 2, yellow, 0);
            }
            i += 1;
            row += 0x1a;
        }
        history_tab_cell(area, 2, r, 2, 0, 1, 2, yellow, HISTORY_TAB_TEXT);
        r += 1;
    }
    if rows_of != 0 {
        exe::crt_free(rows_of);
    }
    while r < rows as i32 {
        for col in 0..3 {
            history_tab_cell(area, col, r, 2, 0, 1, 2, pale, 0);
        }
        r += 1;
    }
    exe::drop_text(t);
    area
}

/// The league table view of the Competitions tab (all the domestic leagues, view −2, or one league, a division of
/// `comp_util_is_league_division`): a heading row (Pld, Won, Drn or SOW when the first league's object has flag 0x20 at
/// +0xab, Lst, For, Ag, Pts) and one row per league season of the club (the league history list, param 0x19) in that
/// view: the season, the position (`utils_format_ordinal`), the league (linked 0x7d0 when its object is loaded) and the
/// seven numbers of the record. Empty rows fill the rest. Returns the area.
/// Original: part of `club_history_draw_competitions_tab` (0x46adc0, club_screens.cpp).
unsafe fn history_competitions_league_table(club: usize, view: i32, top: i32, bottom: i32, rows: u32, weights: &[u8; 10], leagues: usize) -> usize {
    let season: unsafe extern "C" fn(u32, usize, usize, u32) -> i32 = core::mem::transmute(0x669630usize);
    let ordinal: unsafe extern "C" fn(i32, i32) -> usize = core::mem::transmute(0x90c310usize);
    let current: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d2e0usize);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let area = add_area(current_human_display(), 0x6e, top as usize, 0x30c, bottom as usize, 0xa, weights.as_ptr() as usize, rows as usize, 0, 1, 0, usize::MAX);
    let pale = colour_word(G_GUI_COLOUR_PALE_GREY);
    let grey = colour_word(G_GUI_COLOUR_GREY);
    let mut r: i32 = 0;
    if leagues != 0 {
        for col in 0..3 {
            history_tab_cell(area, col, 0, 2, 0, 1, 2, pale, 0);
        }
        let heading = |col: u32| {
            history_tab_cell(area, col, 0, 0x30, grey, 0xc, 1, pale, HISTORY_TAB_TEXT);
        };
        lang(HISTORY_TAB_TEXT, 0x990b3c); // "Pld"
        heading(3);
        lang(HISTORY_TAB_TEXT, 0x990b1c, 0xad9c64); // "Won<%s - COMMENT - games won>"
        heading(4);
        write_u32(leagues + llist::CURSOR, read_u32(leagues + llist::HEAD));
        let first = read_usize(current(leagues));
        if read_u8(read_usize(read_usize(G_COMP_OBJECT_TABLE) + first * 4) + 0xab) & 0x20 != 0 {
            lang(HISTORY_TAB_TEXT, 0x990b18); // "SOW"
        } else {
            lang(HISTORY_TAB_TEXT, 0x990b14); // "Drn"
        }
        heading(5);
        lang(HISTORY_TAB_TEXT, 0x990af4, 0xad9c64); // "Lst<%s - COMMENT - games lost>"
        heading(6);
        lang(HISTORY_TAB_TEXT, 0x990af0); // "For"
        heading(7);
        lang(HISTORY_TAB_TEXT, 0x990aec); // "Ag"
        heading(8);
        lang(HISTORY_TAB_TEXT, 0x990ae8); // "Pts"
        heading(9);
        write_u32(leagues + llist::CURSOR, read_u32(leagues + llist::HEAD));
        r = 1;
        let mut node = exe::llist_get_next(leagues);
        while node != 0 {
            if view == -2 || read_i32(node) == view {
                let comp = history_tab_comp(read_usize(node));
                season(read_u16(node + 4) as u32, club, HISTORY_TAB_TEXT, 0x7d0);
                history_tab_cell(area, 0, r, 0x30, colour_word(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, pale, HISTORY_TAB_TEXT);
                history_tab_copy(core::slice::from_raw_parts_mut(HISTORY_TAB_TEXT as *mut u8, 0x100), ordinal(read_i8(node + 6) as i32 + 1, -1));
                history_tab_cell(area, 1, r, 2, 0, 0xc, 2, pale, HISTORY_TAB_TEXT);
                format(HISTORY_TAB_TEXT, 0x987ff0, comp + competition::SHORT_NAME); // "  %s"
                if history_tab_comp_loaded(read_usize(comp)) {
                    history_tab_link_cell(area, 2, r, colour_word(G_GUI_COLOUR_YELLOW), HISTORY_TAB_TEXT, 0x7d0, comp);
                } else {
                    history_tab_cell(area, 2, r, 2, 0, 1, 2, colour_word(G_GUI_COLOUR_YELLOW), HISTORY_TAB_TEXT);
                }
                let numbers = [
                    read_i8(node + 0xb) as i32,
                    read_i8(node + 0xc) as i32,
                    read_i8(node + 0xd) as i32,
                    read_i8(node + 0xe) as i32,
                    read_i16(node + 0xf) as i32,
                    read_i16(node + 0x11) as i32,
                ];
                for (k, &n) in numbers.iter().enumerate() {
                    format(HISTORY_TAB_TEXT, 0x98806c, n); // "%d"
                    history_tab_cell(area, 3 + k as u32, r, 2, 0, 0xc, 2, pale, HISTORY_TAB_TEXT);
                }
                format(HISTORY_TAB_TEXT, 0x98806c, read_i16(node + 0x13) as i32); // "%d"
                history_tab_cell(area, 9, r, 0x30, colour_word(0xae3192), 0xc, 1, pale, HISTORY_TAB_TEXT);
                r += 1;
            }
            node = exe::llist_get_next(leagues);
        }
    }
    while r < rows as i32 {
        for col in 0..10 {
            history_tab_cell(area, col, r, 2, 0, 1, 2, pale, 0);
        }
        r += 1;
    }
    area
}

/// Draws the Competitions tab of the club history screen from `top` to `bottom`. The View popup: Honours (link 3, −1),
/// Domestic Leagues (−2) and one entry per league and per cup the club played (the league and cup history lists,
/// params 0x19 and 0x1b, and their competition id lists, 0x1a and 0x1c, built by `Club_History` and kept in the params
/// on the first draw); the chosen view (param 0x1d) is the heading, a link to the competition (0x7d0) when it is a loaded
/// one. Then, in the rows that fit in font 2: Domestic Leagues or a league division, the league table
/// (`history_competitions_league_table`); a cup, the cup runs (`history_competitions_cup_runs`); Honours, the honours
/// (`history_competitions_honours`). The scroll row comes back from param 0x1e and the area goes to param 0x1f.
///
/// Convention: cdecl(top, bottom), SEH frame (0x818 bytes of locals: the view label and a Text). Split into named
/// pieces; the whole routine is hooked.
/// Original: `club_history_draw_competitions_tab` (0x46adc0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_draw_competitions_tab(top: i32, bottom: i32) {
    let button_item: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let league_history: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x443380usize);
    let league_ids: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x443770usize);
    let cup_history: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x443520usize);
    let cup_ids: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x443a20usize);
    covered!("tel03_3 competitions tab");
    let table_weights: [u8; 10] = [4, 2, 9, 3, 3, 3, 3, 3, 3, 3];
    let weights: [u8; 5] = [1, 8, 4, 3, 8];
    let club = get_screen_param(0);
    let mut leagues = get_screen_param(0x19);
    let mut league_list = get_screen_param(0x1a);
    let mut cups = get_screen_param(0x1b);
    let mut cup_list = get_screen_param(0x1c);
    let view = get_screen_param(0x1d) as i32;
    let popup = history_tab_view_button(top);
    // the heading stays as it was when no entry matches the view (stack garbage in the original, empty here)
    let mut heading = [0u8; 0x7d0];
    lang(HISTORY_TAB_TEXT, 0x990b90); // "Honours"
    history_tab_menu_entry(popup, 1, 3, usize::MAX);
    if view == -1 {
        history_tab_copy(&mut heading, HISTORY_TAB_TEXT);
    }
    let history = read_usize(G_CLUB_HISTORY);
    if leagues == 0 {
        leagues = league_history(history, club);
        set_screen_param(0x19, leagues);
    }
    if league_list == 0 {
        league_list = league_ids(history, leagues);
        set_screen_param(0x1a, league_list);
    }
    if league_list != 0 {
        lang(HISTORY_TAB_TEXT, 0x990b7c); // "Domestic Leagues"
        history_tab_menu_entry(popup, 1, 3, (-2isize) as usize);
        if view == -2 {
            history_tab_copy(&mut heading, HISTORY_TAB_TEXT);
        }
        history_competitions_menu_list(league_list, popup, view, &mut heading);
    }
    if cups == 0 {
        cups = cup_history(history, club);
        set_screen_param(0x1b, cups);
    }
    if cup_list == 0 {
        cup_list = cup_ids(history, cups);
        set_screen_param(0x1c, cup_list);
    }
    if cup_list != 0 {
        history_competitions_menu_list(cup_list, popup, view, &mut heading);
    }
    let mut y = top + 0x19;
    if view > -1 && history_tab_comp_loaded(view as usize) {
        button_item(G_SCREEN_MANAGER, 0x6e, y as usize, 0x30c, (y + 0x23) as usize, 2, 0, 0xc, 3, colour_word(G_GUI_COLOUR_YELLOW), heading.as_ptr() as usize, 0x7d0, history_tab_comp(view as usize));
    } else {
        history_tab_heading(y, heading.as_ptr() as usize);
    }
    y += 0x28;
    let rows = (((bottom - y) / exe::font_get_height(2)) as u32) & 0xff;
    let area = if view == -2 || (view > -1 && exe::comp_util_is_league_division(view) != 0) {
        history_competitions_league_table(club, view, y, bottom, rows, &table_weights, leagues)
    } else if view > -1 && exe::comp_util_is_league_division(view) == 0 {
        history_competitions_cup_runs(club, view, y, bottom, rows, &weights, cups)
    } else {
        history_competitions_honours(club, y, bottom, rows, &weights)
    };
    history_tab_scroll(area, 0x1e, 0x1f);
}

/// How a season tab picks the period of the records it lists.
#[derive(Clone, Copy)]
enum HistoryPeriod {
    /// the records tab's period (param 2)
    Param,
    /// a fixed period
    Fixed(u8),
}

/// What a season tab does when the stored record type does not apply to the club in the period.
#[derive(Clone, Copy)]
enum HistoryDefault {
    /// nothing
    Keep,
    /// this type
    Type(u8),
    /// the first type that applies (0xff until then)
    FirstThatApplies,
}

/// One season tab of the club history screen (positions, attendances, results, sequences, players, transfers): the
/// record types of `category` (`club_record_type_to_category`) in the View popup (link `category` + 2 with the type,
/// greyed out when the type does not apply to the club in the period: `club_records_record_type_applies`), the chosen
/// type (param `view_param`, or the default) as the heading, and one row per season of the club's records (the array
/// of `Club_Records::build_history_list`, built once and kept in params 5 and 6; 0x38d bytes a season, year in the first
/// word, 0 = none): the season (`key_nation_format_club_season`) and the record of that type
/// (`club_records_format_record_text`, wrapped). Empty rows fill the rest; the scroll row comes back from param
/// `scroll_param` and the area goes to param `area_param`.
unsafe fn history_season_tab(top: i32, bottom: i32, category: u8, view_param: u32, scroll_param: u32, area_param: u32, period: HistoryPeriod, default: HistoryDefault) {
    let to_category: unsafe extern "C" fn(u32, usize) -> u8 = core::mem::transmute(0x483920usize);
    let applies: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32 = core::mem::transmute(0x449590usize);
    let type_label: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32 = core::mem::transmute(0x4490b0usize);
    let build: unsafe extern "thiscall" fn(usize, usize, *mut i32) -> usize = core::mem::transmute(0x450d50usize);
    let season: unsafe extern "C" fn(u32, usize, usize, u32) -> i32 = core::mem::transmute(0x669630usize);
    let record_text: unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut usize) -> u32 = core::mem::transmute(0x449810usize);
    let draw: unsafe extern "thiscall" fn(*mut usize, usize, usize, usize, *mut i32, usize, usize, usize, usize) -> usize = core::mem::transmute(0x4aeaf0usize);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let weights: [u8; 2] = [1, 8];
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0);
    let club = get_screen_param(0);
    let mut seasons = get_screen_param(5);
    let mut count = get_screen_param(6) as i32;
    let mut view = get_screen_param(view_param) as u8;
    let period = match period {
        HistoryPeriod::Param => get_screen_param(2) as u8 as u32,
        HistoryPeriod::Fixed(p) => p as u32,
    };
    let popup = history_tab_view_button(top);
    let records = read_usize(G_CLUB_RECORDS);
    match default {
        HistoryDefault::Keep => {}
        HistoryDefault::Type(k) => {
            if applies(records, view as u32, club, period) == 0 {
                view = k;
            }
        }
        HistoryDefault::FirstThatApplies => {
            if applies(records, view as u32, club, period) == 0 {
                view = 0xff;
            }
        }
    }
    // the heading stays as it was when no type matches the view (stack garbage in the original, empty here)
    let mut heading = [0u8; 0x7d0];
    for kind in 1..=0x20u32 {
        if to_category(kind, 0) != category {
            continue;
        }
        if applies(records, kind, club, period) != 0 {
            if let HistoryDefault::FirstThatApplies = default {
                if view == 0xff {
                    view = kind as u8;
                }
            }
            type_label(records, kind, HISTORY_TAB_TEXT, 0x7d0);
            history_tab_menu_entry(popup, 1, category as u32 + 2, kind as usize);
            if kind as u8 == view {
                history_tab_copy(&mut heading, HISTORY_TAB_TEXT);
            }
        } else {
            type_label(records, kind, HISTORY_TAB_TEXT, 0x7d0);
            history_tab_menu_entry(popup, 0x21, 0, 0);
        }
    }
    history_tab_heading(top + 0x19, heading.as_ptr() as usize);
    let y = top + 0x41;
    let rows = ((bottom - y) / exe::font_get_height(2)) as u8;
    let area = add_area(current_human_display(), 0x6e, y as usize, 0x30c, bottom as usize, 2, weights.as_ptr() as usize, rows as usize, 0, 1, 0, usize::MAX);
    if seasons == 0 {
        covered!("tel03_3 season tab builds the records");
        seasons = build(records, club, &mut count);
        set_screen_param(5, seasons);
        set_screen_param(6, count as usize);
    }
    let pale = colour_word(G_GUI_COLOUR_PALE_GREY);
    let yellow = colour_word(G_GUI_COLOUR_YELLOW);
    let mut r: i32 = 0;
    let mut entry = seasons;
    let mut i = 0;
    while i < count {
        let year = read_u16(entry);
        if year != 0 {
            season(year as u32, club, HISTORY_TAB_TEXT, 0x7d0);
            history_tab_cell(area, 0, r, 0x30, colour_word(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, pale, HISTORY_TAB_TEXT);
            record_text(records, club, entry + 2, view as u32, t);
            history_tab_cell(area, 1, r, 2, 0, 1, 2, 0, 0);
            draw(t, area, 0xbe, 0x2ee, &mut r, 2, yellow, 2, 0);
            r += 1;
        }
        i += 1;
        entry += 0x38d;
    }
    while r < rows as i32 {
        history_tab_cell(area, 0, r, 2, 0, 1, 2, yellow, 0);
        history_tab_cell(area, 1, r, 2, 0, 1, 2, yellow, 0);
        r += 1;
    }
    history_tab_scroll(area, scroll_param, area_param);
    exe::drop_text(t);
}

/// Draws the Positions tab of the club history screen: the season tab of category 4 (view param 7, scroll 8, area 9),
/// in the records tab's period (param 2).
/// Original: `club_history_draw_positions_tab` (0x46c030, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_draw_positions_tab(top: i32, bottom: i32) {
    covered!("tel03_3 positions tab");
    history_season_tab(top, bottom, 4, 7, 8, 9, HistoryPeriod::Param, HistoryDefault::Keep);
}

/// Draws the Attendances tab of the club history screen: the season tab of category 5 (view param 0xa, scroll 0xb,
/// area 0xc), period 1, type 3 when the stored one does not apply.
/// Original: `club_history_draw_attendances_tab` (0x46c510, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_draw_attendances_tab(top: i32, bottom: i32) {
    history_season_tab(top, bottom, 5, 0xa, 0xb, 0xc, HistoryPeriod::Fixed(1), HistoryDefault::Type(3)); // tel03_3 the attendances default type
}

/// Draws the Results tab of the club history screen: the season tab of category 6 (view param 0xd, scroll 0xe, area
/// 0xf), period 1, type 7 when the stored one does not apply.
/// Original: `club_history_draw_results_tab` (0x46c9e0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_draw_results_tab(top: i32, bottom: i32) {
    history_season_tab(top, bottom, 6, 0xd, 0xe, 0xf, HistoryPeriod::Fixed(1), HistoryDefault::Type(7));
}

/// Draws the Sequences tab of the club history screen: the season tab of category 7 (view param 0x10, scroll 0x11, area
/// 0x12), period 1.
/// Original: `club_history_draw_sequences_tab` (0x46ceb0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_draw_sequences_tab(top: i32, bottom: i32) {
    history_season_tab(top, bottom, 7, 0x10, 0x11, 0x12, HistoryPeriod::Fixed(1), HistoryDefault::Keep);
}

/// Draws the Players tab of the club history screen: the season tab of category 8 (view param 0x13, scroll 0x14, area
/// 0x15), period 2, the first type that applies when the stored one does not.
/// Original: `club_history_draw_players_tab` (0x46d370, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_draw_players_tab(top: i32, bottom: i32) {
    history_season_tab(top, bottom, 8, 0x13, 0x14, 0x15, HistoryPeriod::Fixed(2), HistoryDefault::FirstThatApplies);
}

/// Draws the Transfers tab of the club history screen: the season tab of category 9 (view param 0x16, scroll 0x17, area
/// 0x18), period 1.
/// Original: `club_history_draw_transfers_tab` (0x46d860, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_draw_transfers_tab(top: i32, bottom: i32) {
    history_season_tab(top, bottom, 9, 0x16, 0x17, 0x18, HistoryPeriod::Fixed(1), HistoryDefault::Keep);
}

/// The side-by-side comparer of a club history tab: both draw and are compared by the display list
/// (`compare_match_screen_build`); when a list or the record array the tab keeps in the params is still to be built
/// (`cache` holds one of those params at 0), only the original runs (building twice would leave two lists and the
/// params would differ by the pointer).
unsafe fn compare_history_tab(name: &str, cache: &[u32], o: unsafe extern "C" fn(i32, i32), r: unsafe extern "C" fn(i32, i32), top: i32, bottom: i32) {
    if cache.iter().any(|&k| get_screen_param(k) == 0) {
        return o(top, bottom);
    }
    compare_match_screen_build(name, &[], || o(top, bottom), || r(top, bottom));
}

/// Comparer for `club_history_draw_competitions_tab` (lists in params 0x19..0x1c).
pub(crate) unsafe extern "C" fn compare_club_history_draw_competitions_tab(top: i32, bottom: i32) {
    compare_history_tab("club history competitions tab", &[0x19, 0x1a, 0x1b, 0x1c], original_fn(%%I0%%), club_history_draw_competitions_tab, top, bottom)
}

/// Comparer for `club_history_draw_positions_tab` (record array in param 5).
pub(crate) unsafe extern "C" fn compare_club_history_draw_positions_tab(top: i32, bottom: i32) {
    compare_history_tab("club history positions tab", &[5], original_fn(%%I1%%), club_history_draw_positions_tab, top, bottom)
}

/// Comparer for `club_history_draw_attendances_tab` (record array in param 5).
pub(crate) unsafe extern "C" fn compare_club_history_draw_attendances_tab(top: i32, bottom: i32) {
    compare_history_tab("club history attendances tab", &[5], original_fn(%%I2%%), club_history_draw_attendances_tab, top, bottom)
}

/// Comparer for `club_history_draw_results_tab` (record array in param 5).
pub(crate) unsafe extern "C" fn compare_club_history_draw_results_tab(top: i32, bottom: i32) {
    compare_history_tab("club history results tab", &[5], original_fn(%%I3%%), club_history_draw_results_tab, top, bottom)
}

/// Comparer for `club_history_draw_sequences_tab` (record array in param 5).
pub(crate) unsafe extern "C" fn compare_club_history_draw_sequences_tab(top: i32, bottom: i32) {
    compare_history_tab("club history sequences tab", &[5], original_fn(%%I4%%), club_history_draw_sequences_tab, top, bottom)
}

/// Comparer for `club_history_draw_players_tab` (record array in param 5).
pub(crate) unsafe extern "C" fn compare_club_history_draw_players_tab(top: i32, bottom: i32) {
    compare_history_tab("club history players tab", &[5], original_fn(%%I5%%), club_history_draw_players_tab, top, bottom)
}

/// Comparer for `club_history_draw_transfers_tab` (record array in param 5).
pub(crate) unsafe extern "C" fn compare_club_history_draw_transfers_tab(top: i32, bottom: i32) {
    compare_history_tab("club history transfers tab", &[5], original_fn(%%I6%%), club_history_draw_transfers_tab, top, bottom)
}
