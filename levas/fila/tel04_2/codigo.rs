// ---------- batch %%LEVA%%: the friendly tour and the tournament settings (club_screens.cpp, part 4) ----------

/// Opens the arrange friendly tour screen of `club`, the nation choice (builder `friendly_tour_nation_screen_build`,
/// handler `friendly_tour_nation_screen_handle_event`); no club shows the club_screens.cpp 12948 box. Params: 0 the
/// club, 1 the chosen nation (none), 2 and 3 the scroll row and the area (0), 4 "available only" on.
/// Original: `open_friendly_tour_nation_screen` (0x4719a0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_friendly_tour_nation_screen(club: usize) {
    if club == 0 {
        club_pages_error_box(0x3294);
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x471a90usize, 0x472180usize, 1, 0usize, 0) != 0 {
        covered!("tel04_2 open the tour nations");
        for (k, v) in [(0u32, club), (1, 0), (2, 0), (3, 0), (4, 1)] {
            set_screen_param(k, v);
        }
    }
}

/// Whether a nation is open to a tour of `club`: a club of reputation tier (`friendly_club_reputation_tier`) below 4
/// only tours its own region, below 5 its own continent.
unsafe fn friendly_tour_nation_open(club: usize, n: usize) -> bool {
    let tier: unsafe extern "thiscall" fn(usize, usize) -> i8 = core::mem::transmute(0x5c52d0usize);
    let home = read_usize(club + club::NATION);
    if tier(read_usize(G_FRIENDLY), club) < 4 && read_i8(home + nation::ACTUAL_REGION) != read_i8(n + nation::ACTUAL_REGION) {
        return false;
    }
    !(tier(read_usize(G_FRIENDLY), club) < 5 && read_usize(home + nation::CONTINENT) != read_usize(n + nation::CONTINENT)) // tel04_2 the continent rule
}

/// Builds the arrange friendly tour screen, the nation choice of the club (param 0).
///
/// The club's background picture, "Arrange Friendly Tour" in its kit colours, "Select Nation" and the menu bar; the
/// Filter popup (Available Only, ticked with param 4, and All Nations). The nations that have a continent, a game
/// importance up to 2 and clubs, other than the club's own, sorted by short name (`database_compare_clubs_by_short_name`
/// over a malloc'd list; a failed malloc shows the club_screens.cpp 13048 box and ends the build), two per row: the
/// name (a button, id 2, highlighted 0x801 when it is the chosen one, param 1, or plain grey text when the tour rules
/// close it, `friendly_tour_nation_open`; with Available Only the closed ones are left out) and its continent. The
/// scroll row of param 2, the area to param 3; Cancel (id 3) and Next (id 4, only with a chosen nation).
/// Original: `friendly_tour_nation_screen_build` (0x471a90, club_screens.cpp, cdecl; the map's 0x472070 fragment is its
/// tail).
pub(crate) unsafe extern "C" fn friendly_tour_nation_screen_build() {
    let lang = club_pages_lang();
    let format = club_pages_format();
    let row_weights: [u8; 4] = [6, 1, 6, 1];
    let button_weights: [u8; 2] = [3, 1];
    let club = get_screen_param(0);
    let chosen = get_screen_param(1);
    let available_only = get_screen_param(4);
    covered!("tel04_2 tour nations build");
    exe::background_set_club_picture(club, 0, 0);
    lang(JOB_TEXT, 0x990dc4); // "Arrange Friendly Tour"
    club_pages_title(club, JOB_TEXT);
    lang(JOB_TEXT, 0x990db4); // "Select Nation"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    exe::menubar_build(4, 0);
    let (grey, pale, yellow) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY), job_colour(G_GUI_COLOUR_YELLOW));
    let (blue, black) = (job_colour(0xae31aa), job_colour(G_GUI_COLOUR_BLACK));
    lang(JOB_TEXT, 0x98ebf8); // "Filter"
    let button = tel26_add_button([0x28f, 0x91, 0x30c, 0xa5, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0, 0]);
    let filter = club_pages_area([0, 0, 0, 0, 0, 0, 0, 0, 0x130, blue, button]);
    lang(JOB_TEXT, 0x98eb6c); // " Available Only"
    let on = available_only != 0;
    club_pages_item([if on { 0x2002 } else { 2 }, 0, 0, 0, 0, 0, 0, 0x10, blue, yellow, 1, 1, black, JOB_TEXT as u32, 0, !on as u32, !on as u32, filter]);
    lang(JOB_TEXT, 0x990da4); // " All Nations"
    let on = available_only == 0;
    club_pages_item([if on { 0x2002 } else { 2 }, 0, 0, 0, 0, 0, 0, 0x10, blue, yellow, 1, 1, black, JOB_TEXT as u32, 0, !on as u32, 0, filter]);
    let rows = (0x15d / exe::font_get_height(3)) as u8 as u32;
    let area = club_pages_area([0x6e, 0xaa, 0x30c, 0x217, 4, row_weights.as_ptr() as u32, rows, 0, 2, 0, u32::MAX]);
    let (mut col, mut row) = (0u32, 0u32);
    let list = exe::crt_malloc((read_i32(G_NATION_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        club_pages_error_box(0x32f8);
        return;
    }
    let mut count = 0usize;
    for k in 0..read_i32(G_NATION_COUNT).max(0) as usize {
        let n = read_usize(G_NATION_TABLE) + k * nation::SIZE;
        if read_usize(n + nation::CONTINENT) != 0 && read_i8(n + nation::GAME_IMPORTANCE) <= 2 && n != read_usize(club + club::NATION) && read_i16(n + nation::NUMBER_CLUBS) != 0 {
            write_usize(list + count * 4, n);
            count += 1;
        }
    }
    exe::crt_qsort(list, count as u32, 4, 0x540080usize);
    for k in 0..count {
        let n = read_usize(list + k * 4);
        let open = friendly_tour_nation_open(club, n);
        if available_only != 0 && !open {
            continue;
        }
        format(JOB_TEXT, 0x987ff0, n + 4); // "  %s"
        if open {
            let style = if n == chosen { 0x801 } else { 1 };
            tel26_area_button([area, col, row, style, 0, 1, 3, pale, JOB_TEXT as u32, 2, n as u32]);
        } else {
            tel26_area_text([area, col, row, 1, 0, 1, 3, grey, JOB_TEXT as u32]);
        }
        tel26_area_text([area, col + 1, row, 1, 0, 0xc, 2, yellow, (read_usize(n + nation::CONTINENT) + 0x1f) as u32]);
        col += 2;
        if col > 3 {
            col = 0;
            row += 1;
        }
    }
    exe::crt_free(list);
    club_pages_restore_scroll(area, 2, 3);
    let buttons = club_pages_area([100, 0x22b, 0x316, 0x24e, 2, button_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    lang(JOB_TEXT, 0x98a1a0); // "Cancel"
    tel26_area_button([buttons, 0, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 3, 0]);
    lang(JOB_TEXT, 0x9880a0); // "Next"
    if chosen == 0 {
        tel26_area_text([buttons, 1, 0, 0x30, grey, 0x2c, 3, pale, JOB_TEXT as u32]);
    } else {
        tel26_area_button([buttons, 1, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 4, 0]);
    }
}

/// Answers a click on the tour nation choice. A real widget saves the scroll row (param 2 from the area of param 3).
/// Id 1 toggles Available Only (param 4, −11); id 2 chooses the nation (param 1, −11); id 3 cancels (−9); id 4 opens
/// the tour dates (`open_arrange_friendly_screen`, kind 0xb, with the nation; −4). Anything else goes to the menu bar.
/// Original: `friendly_tour_nation_screen_handle_event` (0x472180, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn friendly_tour_nation_screen_handle_event(widget: u32) -> i32 {
    club_pages_save_scroll(widget, 2, 3);
    let club = get_screen_param(0);
    let chosen = get_screen_param(1);
    let available_only = get_screen_param(4);
    match club_pages_id(widget) {
        1 => {
            set_screen_param(4, (available_only == 0) as usize);
            -11
        }
        2 => {
            set_screen_param(1, tel26_item_value(widget as i16) as usize);
            -11
        }
        3 => -9,
        4 => {
            let open: unsafe extern "C" fn(usize, u32, usize) = core::mem::transmute(0x4701f0usize);
            open(club, 0xb, chosen);
            -4
        }
        _ => club_pages_menu_bar(widget),
    }
}

/// Builds the tour club choice of a tour (param 0: the club, then the nation).
///
/// The club's background picture, "Arrange Tour Of <nation>" in its kit colours, "Select Club" and the menu bar; Cancel
/// (id 1) and Next (−3, plain text when there is no next screen). The nation's clubs (not national teams), sorted by
/// short name over a malloc'd list (a failed malloc: the club_screens.cpp 13318 box), two per row of three columns: the
/// name (yellow for a human's), the national team (linked) and the division (linked when it has a competition object).
/// The scroll row of param 1, the area to param 2.
/// Original: `friendly_tour_club_screen_build` (0x472380, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn friendly_tour_club_screen_build() {
    let lang = club_pages_lang();
    let format = club_pages_format();
    let row_weights: [u8; 6] = [5, 1, 1, 5, 1, 1];
    let button_weights: [u8; 2] = [3, 1];
    let tour = get_screen_param(0);
    let club = read_usize(tour);
    let tour_nation = read_usize(tour + 4);
    covered!("tel04_2 tour clubs build");
    exe::background_set_club_picture(club, 0, 0);
    let c = read_i8(tour_nation + nation::SHORT_GENDER_NAME) as i32;
    lang(JOB_TEXT, 0x990d4c, c, c, tour_nation + nation::SHORT_NAME); // "Arrange Tour Of {}<%s - nation>{}"
    club_pages_title(club, JOB_TEXT);
    lang(JOB_TEXT, 0x990ddc); // "Select Club"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    exe::menubar_build(4, 0);
    let (grey, pale, yellow) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY), job_colour(G_GUI_COLOUR_YELLOW));
    let buttons = club_pages_area([100, 0x22b, 0x316, 0x24e, 2, button_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    lang(JOB_TEXT, 0x98a1a0); // "Cancel"
    tel26_area_button([buttons, 0, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 1, 0]);
    lang(JOB_TEXT, 0x9880a0); // "Next"
    if exe::screenmanager_is_forward_unavailable(G_SCREEN_MANAGER) == 0 {
        tel26_area_button([buttons, 1, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 0xffff_fffd, 0]);
    } else {
        tel26_area_text([buttons, 1, 0, 0x30, grey, 0x2c, 3, pale, JOB_TEXT as u32]);
    }
    let rows = (0x176 / exe::font_get_height(3)) as u8 as u32;
    let area = club_pages_area([0x6e, 0x91, 0x30c, 0x217, 6, row_weights.as_ptr() as u32, rows, 0, 2, 0, u32::MAX]);
    let (mut col, mut row) = (0u32, 0u32);
    let list = exe::crt_malloc((read_i32(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        club_pages_error_box(0x3406);
        return;
    }
    let mut count = 0usize;
    for k in 0..read_i32(G_CLUB_COUNT).max(0) as usize {
        let c = read_usize(G_CLUB_TABLE) + k * club::SIZE;
        if exe::database_is_national_team(c) == 0 && read_usize(c + club::NATION) == tour_nation {
            write_usize(list + count * 4, c);
            count += 1;
        }
    }
    exe::crt_qsort(list, count as u32, 4, 0x540080usize);
    for k in 0..count {
        let c = read_usize(list + k * 4);
        format(JOB_TEXT, 0x987ff0, c + club::SHORT_NAME); // "  %s"
        let colour = if exe::human_manager_is_human_club(c, 0, 1, 0, 0) != 0 { yellow } else { pale };
        tel26_area_button([area, col, row, 1, 0, 1, 3, colour, JOB_TEXT as u32, 0x7d5, c as u32]);
        let n = read_usize(c + club::NATION);
        let team = exe::database_get_national_team(n);
        tel26_area_button([area, col + 1, row, 1, 0, 0xc, 2, yellow, (n + 0x53) as u32, 0x7d5, team as u32]);
        let division = read_usize(c + club::DIVISION);
        if division != 0 {
            if read_u32(read_usize(G_COMP_OBJECT_TABLE) + read_u32(division) as usize * 4) == 0 {
                tel26_area_text([area, col + 2, row, 1, 0, 0xc, 2, job_colour(0xaebdd4), (division + 0x53) as u32]);
            } else {
                tel26_area_button([area, col + 2, row, 1, 0, 0xc, 2, job_colour(0xaebdd4), (division + 0x53) as u32, 0x7d0, division as u32]);
            }
        }
        col += 3;
        if col > 3 {
            col = 0;
            row += 1;
        }
    }
    exe::crt_free(list);
    club_pages_restore_scroll(area, 1, 2);
}

/// Answers a click on the tour club choice: a real widget saves the scroll row (param 1 from the area of param 2); id 1
/// (Cancel) clears the tour record's +0x103 and leaves (−9); anything else goes to the menu bar.
/// Original: `friendly_tour_club_screen_handle_event` (0x472850, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn friendly_tour_club_screen_handle_event(widget: u32) -> i32 {
    let tour = get_screen_param(0);
    club_pages_save_scroll(widget, 1, 2);
    if club_pages_id(widget) == 1 {
        write_u32(tour + 0x103, 0);
        return -9;
    }
    club_pages_menu_bar(widget)
}

/// Opens the Configure Tournament screen over the tournament settings `settings` (builder
/// `configure_tournament_screen_build`, handler `configure_tournament_screen_handle_event`); no settings shows the
/// club_screens.cpp 13437 box. Params: 0 the settings, 1 `arg` (set: the screen was opened by a new tournament and its
/// settings are freed on Cancel), 2 a new working copy of the 0xd1 bytes.
/// Original: `open_configure_tournament_screen` (0x472900, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_configure_tournament_screen(settings: usize, arg: u32) {
    if settings == 0 {
        club_pages_error_box(0x347d);
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x472a00usize, 0x4735f0usize, 1, 0usize, 0) != 0 {
        covered!("tel04_2 open the tournament settings");
        let copy = exe::crt_operator_new(0xd1);
        core::ptr::copy_nonoverlapping(settings as *const u8, copy as *mut u8, 0xd1);
        set_screen_param(0, settings);
        set_screen_param(1, arg as usize);
        set_screen_param(2, copy);
    }
}

/// One choice of the Configure Tournament table: plain grey text when it does not apply (a cup's points, tie-break and
/// decider), the yellow text 0x221 when it is the setting, else a button with its id and value.
unsafe fn tournament_setting(area: u32, col: u32, row: u32, disabled: bool, current: bool, id: u32, value: u32) {
    let green = job_colour(0xad9db6);
    if disabled {
        tel26_area_text([area, col, row, 1, 0, 0x2c, 2, green, JOB_TEXT as u32]);
    } else if current {
        let yellow = job_colour(G_GUI_COLOUR_YELLOW);
        tel26_area_text([area, col, row, 0x221, yellow, 0xc, 2, yellow, JOB_TEXT as u32]);
    } else {
        tel26_area_button([area, col, row, 1, 0, 0xc, 2, green, JOB_TEXT as u32, id, value]);
    }
}

/// Builds the Configure Tournament screen over the working copy of the settings (param 2).
///
/// The green of the choices ([0xad9db6]) is made once (`graphics_rgb_to_pixel(0, 0xc0, 0, 0)`). A cup (kind 1) gets a
/// decider (+0xcd 0 → 1) and 4 teams (+0xcf 3 → 4). No background picture; without param 1 a screen-wide area blocks
/// the screen below; a dark green box, the title "Configure Tournament", Cancel (id 1) and Ok (id 2), the pitch drawing
/// and the table: the name (an edit field over the copy's name, 15 characters), the type (League 2 / Cup 1, id 3),
/// points for a win (3 / 2, id 4), the tie-break (goal difference 1, goals for 2, games won 3, id 5), extra time (none
/// 0, 30 minutes 2, golden goal 0x80, id 6), the decider (none 0 / penalties 1, id 7) and the teams (4 / 3, id 8); a
/// cup has no points, tie-break, "none" decider nor 3 teams.
/// Original: `configure_tournament_screen_build` (0x472a00, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn configure_tournament_screen_build() {
    let lang = club_pages_lang();
    let button_weights: [u8; 2] = [3, 1];
    let name_weights: [u8; 4] = [3, 6, 0, 0];
    let table_weights: [u8; 4] = [3, 2, 2, 2];
    if read_u8(0xadab40) & 1 == 0 {
        write_u8(0xadab40, read_u8(0xadab40) | 1);
        write_u16(0xad9db6, exe::graphics_rgb_to_pixel(0, 0xc0, 0, 0) as u16);
    }
    let arg = get_screen_param(1);
    let s = get_screen_param(2);
    covered!("tel04_2 tournament settings build");
    let kind = read_u8(s + 0xc8);
    if kind == 1 {
        if read_u16(s + 0xcd) == 0 {
            write_u16(s + 0xcd, 1);
        }
        if read_u16(s + 0xcf) == 3 {
            write_u16(s + 0xcf, 4); // tel04_2 a cup's teams
        }
    }
    exe::background_set_picture(0usize);
    if arg == 0 {
        let (w, h) = (exe_immediate(0x472adc as *const i32), exe_immediate(0x472ad7 as *const i32));
        club_pages_area([0, 0, w as u32, h as u32, 1, 0, 1, 0, 2, 0, u32::MAX]);
    }
    let dark_green = exe::graphics_rgb_to_pixel(0, 0x38, 0, 0);
    club_pages_area([0xaa, 0x89, 0x2cf, 0x1cd, 1, 0, 1, 0, 0x30, dark_green, u32::MAX]);
    let (grey, pale, label) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY), job_colour(SCREEN_COLOUR_3154));
    lang(JOB_TEXT, 0x990f80); // "Configure Tournament"
    let text_item: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    text_item(G_SCREEN_MANAGER, 0xb4, 0x93, 0x2c5, 0xc0, 0x30, job_colour(G_GUI_COLOUR_BLACK), 0xc, 4, pale, JOB_TEXT as u32, 1);
    let buttons = club_pages_area([0xb4, 0x1a5, 0x2c5, 0x1c3, 2, button_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    lang(JOB_TEXT, 0x98a1a0); // "Cancel"
    tel26_area_button([buttons, 0, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 1, 0]);
    lang(JOB_TEXT, 0x990f7c); // "Ok"
    tel26_area_button([buttons, 1, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 2, 0]);
    let mut pitch = [0i32; 4];
    let p = pitch.as_mut_ptr();
    let draw_pitch: unsafe extern "C" fn(u32, u32, u32, u32, *mut i32, *mut i32, *mut i32, *mut i32, u32, u32, u32) -> u32 = core::mem::transmute(FN_DRAW_PITCH);
    draw_pitch(0xbe, 0xd4, 0x2bb, 0x191, p.add(3), p.add(2), p.add(1), p, 1, buttons, 0x4080_0000);
    let names = club_pages_area([0xc3, 0xd9, 0x2b6, 0x18c, 4, name_weights.as_ptr() as u32, 7, 0, 1, 0, u32::MAX]);
    lang(JOB_TEXT, 0x98ee04); // " Name"
    tel26_area_text([names, 0, 0, 1, 0, 1, 3, label, JOB_TEXT as u32]);
    club_pages_item([0x80018, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 2, job_colour(0xad9db6), s as u32, 0xf, 0, 0, names]);
    let t = club_pages_area([0xc3, 0xd9, 0x2b6, 0x18c, 4, table_weights.as_ptr() as u32, 7, 0, 1, 0, u32::MAX]);
    let (c9, ca, cb, cd, cf) = (read_u8(s + 0xc9), read_u8(s + 0xca), read_u16(s + 0xcb), read_u16(s + 0xcd), read_u16(s + 0xcf));
    let cup = kind == 1;
    let title = |row: u32, text: usize, arg: usize| {
        if arg != 0 {
            lang(JOB_TEXT, text, arg);
        } else {
            lang(JOB_TEXT, text);
        }
        tel26_area_text([t, 0, row, 1, 0, 1, 3, label, JOB_TEXT as u32]);
    };
    let choice = |col: u32, row: u32, text: usize, arg: usize, disabled: bool, current: bool, id: u32, value: u32| {
        if arg != 0 {
            lang(JOB_TEXT, text, arg);
        } else {
            lang(JOB_TEXT, text);
        }
        tournament_setting(t, col, row, disabled, current, id, value);
    };
    title(1, 0x990f54, 0xad9c64); // " Type"
    choice(1, 1, 0x990f4c, 0, false, kind == 2, 3, 2); // "League"
    choice(2, 1, 0x990f48, 0, false, kind == 1, 3, 1); // "Cup"
    title(2, 0x990f38, 0); // " Points For Win"
    choice(1, 2, 0x990f10, 0xad9c64, cup, c9 == 3, 4, 3); // "Three"
    choice(2, 2, 0x990ee8, 0xad9c64, cup, c9 == 2, 4, 2); // "Two"
    title(3, 0x990edc, 0); // " Separator"
    choice(1, 3, 0x990ed0, 0, cup, ca == 1, 5, 1); // "Goal Diff."
    choice(2, 3, 0x990ec4, 0, cup, ca == 2, 5, 2); // "Goals For"
    choice(3, 3, 0x990eb8, 0, cup, ca == 3, 5, 3); // "Games Won"
    title(4, 0x990eac, 0); // " Extra Time"
    choice(1, 4, 0x990e88, 0xad9c64, false, cb == 0, 6, 0); // "None"
    choice(2, 4, 0x990e80, 0, false, cb == 2, 6, 2); // "30 Mins"
    choice(3, 4, 0x990e74, 0, false, cb == 0x80, 6, 0x80); // "Golden Goal"
    title(5, 0x990e68, 0); // " Decider"
    choice(1, 5, 0x990e48, 0xad9c64, cup, cd == 0, 7, 0); // "None"
    choice(2, 5, 0x990e40, 0, false, cd == 1, 7, 1); // "Penalties"
    title(6, 0x990e38, 0); // " Teams"
    choice(1, 6, 0x990e10, 0xad9c64, false, cf == 4, 8, 4); // "Four"
    choice(2, 6, 0x990de8, 0xad9c64, cup, cf == 3, 8, 3); // "Three"
}

/// Answers a click on Configure Tournament (params 0 the settings, 1 the new-tournament flag, 2 the working copy).
/// Id 1 (Cancel) frees the copy and, for a new tournament, the settings too (−10; else −9). Id 2 (Ok) names an unnamed
/// copy "CM Cup", copies it over the settings and frees it (−9). Ids 3, 4 and 5 store the value as the kind, the points
/// (+0xc9) and the tie-break (+0xca), 6, 7 and 8 as the extra time (+0xcb), the decider (+0xcd) and the teams (+0xcf)
/// (−11). Anything else: 0.
/// Original: `configure_tournament_screen_handle_event` (0x4735f0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn configure_tournament_screen_handle_event(widget: u32) -> i32 {
    let settings = get_screen_param(0);
    let arg = get_screen_param(1);
    let copy = get_screen_param(2);
    let value = tel26_item_value(widget as i16);
    match club_pages_id(widget) {
        1 => {
            exe::crt_operator_delete(copy);
            if arg != 0 {
                exe::crt_operator_delete(settings);
                return -10;
            }
            -9
        }
        2 => {
            if read_u8(copy) == 0 {
                club_pages_lang()(copy, 0x990cb4); // "CM Cup"
            }
            core::ptr::copy_nonoverlapping(copy as *const u8, settings as *mut u8, 0xd1);
            exe::crt_operator_delete(copy);
            -9
        }
        3 => {
            write_u8(copy + 0xc8, value as u8);
            -11
        }
        4 => {
            write_u8(copy + 0xc9, value as u8);
            -11
        }
        5 => {
            write_u8(copy + 0xca, value as u8);
            -11
        }
        6 => {
            write_u16(copy + 0xcb, value as u16);
            -11
        }
        7 => {
            write_u16(copy + 0xcd, value as u16); // tel04_2 the decider
            -11
        }
        8 => {
            write_u16(copy + 0xcf, value as u16);
            -11
        }
        _ => 0,
    }
}

/// Returns the value of widget `widget` of the Display `this`, 0 for −1.
/// Original: `Display::get_widget_data` (0x4739a0, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn display_get_widget_data(this: usize, widget: u32) -> u32 {
    let w = widget as i16;
    if w == -1 {
        return 0;
    }
    read_u32((this as isize + w as isize * display_list::ITEM_SIZE as isize + display::ITEM_VALUE as isize) as usize)
}

/// Comparer for `open_friendly_tour_nation_screen`: only the original runs; the params checked against the plan.
pub(crate) unsafe extern "C" fn compare_open_friendly_tour_nation_screen(club: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I0%%)(club);
    if club != 0 && get_screen_param(0) == club {
        let got = club_pages_params(5);
        SCOREBOARD.record(got == [club, 0, 0, 0, 1], || format!("friendly tour nation screen (open): params {:x?}", got));
    }
}

/// Comparer for `friendly_tour_nation_screen_build`: side by side by the display list.
pub(crate) unsafe extern "C" fn compare_friendly_tour_nation_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    compare_match_screen_build("friendly tour nation screen", &[], || o(), || friendly_tour_nation_screen_build());
}

/// Comparer for `friendly_tour_nation_screen_handle_event`: ids 1..3 side by side over the params; opening the tour
/// dates and the menu bar only the original.
pub(crate) unsafe extern "C" fn compare_friendly_tour_nation_screen_handle_event(widget: u32) -> i32 {
    let both = matches!(club_pages_id(widget), 1..=3);
    club_pages_compare_input("friendly tour nation screen input", widget, original_fn(%%I2%%), friendly_tour_nation_screen_handle_event, both, &[])
}

/// Comparer for `friendly_tour_club_screen_build`: side by side by the display list.
pub(crate) unsafe extern "C" fn compare_friendly_tour_club_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I3%%);
    compare_match_screen_build("friendly tour club screen", &[], || o(), || friendly_tour_club_screen_build());
}

/// Comparer for `friendly_tour_club_screen_handle_event`: Cancel side by side over the params and the tour record's
/// +0x103; the menu bar only the original.
pub(crate) unsafe extern "C" fn compare_friendly_tour_club_screen_handle_event(widget: u32) -> i32 {
    let tour = get_screen_param(0);
    let both = tour != 0 && club_pages_id(widget) == 1;
    club_pages_compare_input("friendly tour club screen input", widget, original_fn(%%I4%%), friendly_tour_club_screen_handle_event, both, &[(tour + 0x103, 4)])
}

/// Comparer for `open_configure_tournament_screen`: only the original runs (it allocates the copy); the params and
/// the copy checked against the settings.
pub(crate) unsafe extern "C" fn compare_open_configure_tournament_screen(settings: usize, arg: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32)>(%%I5%%)(settings, arg);
    if settings != 0 && get_screen_param(0) == settings {
        let copy = get_screen_param(2);
        let same = copy != 0 && core::slice::from_raw_parts(copy as *const u8, 0xd1) == core::slice::from_raw_parts(settings as *const u8, 0xd1);
        SCOREBOARD.record(same && get_screen_param(1) == arg as usize, || format!("configure tournament screen (open): copy {:#x} same {} arg {}", copy, same, get_screen_param(1)));
    }
}

/// Comparer for `configure_tournament_screen_build`: side by side by the display list plus the working copy and the
/// green colour made once.
pub(crate) unsafe extern "C" fn compare_configure_tournament_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I6%%);
    let copy = get_screen_param(2);
    if copy == 0 {
        return o();
    }
    compare_match_screen_build("configure tournament screen", &[(copy, 0xd1), (0xadab40, 1), (0xad9db6, 2)], || o(), || configure_tournament_screen_build());
}

/// Comparer for `configure_tournament_screen_handle_event`: the settings choices side by side over the params and the
/// working copy; Cancel and Ok (they free) only the original.
pub(crate) unsafe extern "C" fn compare_configure_tournament_screen_handle_event(widget: u32) -> i32 {
    let copy = get_screen_param(2);
    let both = copy != 0 && !matches!(club_pages_id(widget), 1 | 2);
    club_pages_compare_input("configure tournament screen input", widget, original_fn(%%I7%%), configure_tournament_screen_handle_event, both, &[(copy, 0xd1)])
}

/// Comparer for `display_get_widget_data`: both read, the values side by side.
pub(crate) unsafe extern "thiscall" fn compare_display_get_widget_data(this: usize, widget: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I8%%)(this, widget);
    let r = display_get_widget_data(this, widget);
    SCOREBOARD.record(o == r, || format!("Display::get_widget_data({:#x}, {}) original={:#x} rust={:#x}", this, widget as i16, o, r));
    o
}
