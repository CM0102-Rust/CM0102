use super::*;

/// Rule: the profile's drop-down button lists the squad of the person's club (or national side) for quick jumps,
/// and "History" when the person has a written history.
///
/// `staff_screens_add_squad_and_history_menu(person, club side, colour, text colour, squad handler, history handler)`:
/// the history key "first second d.m.yyyy" (empty strings for missing names; `Date::get_day_of_month`,
/// `Date::get_month` + 1 and the year of `staff::DATE_OF_BIRTH`) is written into the screens' text buffer and
/// `history_his_file_exists` tells whether there is a written history. For a player (`staff::PLAYER`), his club
/// (`staff::CLUB_JOB` when the club side byte is 1, else `staff::NATIONAL_JOB`) gives the squad list
/// (`build_squad_list_for_staff_history`, at most 0x1c names, 0x1a with a history), read from its head: each slot
/// whose list has a current node gives the next person (none: skipped); the first one makes the menu button and its
/// popup (`squad_menu_section`), each one is an entry by name (style 4, 2000 bytes) with the squad handler and the
/// person. With a history: a separator (or the button when there was no squad), then "History" with the history
/// handler. The local list is destroyed. The return is what the list destructor leaves in eax (here 0).
/// Original: `staff_screens_add_squad_and_history_menu` (0x88b910, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_add_squad_and_history_menu(person: usize, club_side: u32, colour: u32, colour_text: u32,
    squad_handler: u32, history_handler: u32) -> u32 {
    let person = Staff(person);
    let mut list = [0u8; 0x10];
    let list = list.as_mut_ptr() as usize;
    exe::new_llist(list);
    let p = person;
    let second = if person.second_name() != 0 { person.second_name() } else { STAFF_NO_NAME };
    let first = if person.first_name() != 0 { person.first_name() } else { STAFF_NO_NAME };
    let year = person.date_of_birth_year() as u32;
    let month = exe::date_get_month(person.addr() + staff::DATE_OF_BIRTH) as i8 as i32 + 1;
    let day = exe::date_get_day_of_month(person.addr() + staff::DATE_OF_BIRTH) as i16 as i32;
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    sprintf(JOB_TEXT, STAFF_HISTORY_KEY, first, second, day, month, year);
    let history = exe::history_his_file_exists(JOB_TEXT) != 0;
    let mut area = u32::MAX;
    if person.player().addr() != 0 {
        let club = if club_side as u8 == 1 { p.club_job().addr() } else { p.national_job().addr() };
        if club != 0 {
            covered!("tel39_1 profile squad menu");
            // tel39_1: two names fewer with the history entry
            let max: i32 = if history { 0x1a } else { 0x1c };
            build_squad_list_for_staff_history(club, list, max);
            list_rewind(list);
            let mut k = 0i8;
            while (k as i32) < max {
                if exe::llist_get_current(list) != 0 {
                    let s = read_usize(exe::llist_get_next(list));
                    if s != 0 {
                        if area as u16 == 0xffff {
                            squad_menu_section(&mut area, colour, colour_text);
                        }
                        exe::database_format_staff_name(s, JOB_TEXT, 0x7d0, 4);
                        squad_menu_entry(area, JOB_TEXT, squad_handler, s);
                    }
                }
                k = k.wrapping_add(1);
            }
        }
    }
    if history {
        covered!("tel39_1 profile history entry");
        squad_menu_section(&mut area, colour, colour_text);
        tel36_langlib()(JOB_TEXT, 0x9914d4, 0xad9c64usize); // "History<%s - COMMENT - written history screen>"
        squad_menu_entry(area, JOB_TEXT, history_handler, 0);
    }
    exe::drop_llist(list);
    0
}

/// Rule: two text rows are ordered by `utils_string_compare(a, b, 1, 0)` (the qsort order of the profile's context
/// menu and lists).
/// Original: `staff_screens_compare_text_rows` (0x88bc50, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_compare_text_rows(a: usize, b: usize) -> i32 {
    // tel39_1: the comparison's mode 1
    exe::utils_string_compare(a as *const u8, b as *const u8, 1, 0)
}

/// Rule: the Player Nickname dialog: the person's name as the title, the nickname entry, and Cancel, Remove (only
/// active with a nickname) and Ok (only active with a nickname).
///
/// `staff_screens_build_nickname_dialog()`: the person (parameter 0, a record) and the nickname buffer (parameter
/// 1). The random background picture; the title: the person's name (style 1, 2000 bytes) in heading colours 0 and 1
/// (`get_heading_colour(1)` asked first); the menu bar (4, 0); the subtitle "Player Nickname" (style 0). The entry
/// (`Display::add_item` flags 0x80018 at (0x96, 0x136)-(0x2e4, 0x172), font 4, limit 0x18, id 0x32). The button area
/// (0x64, 0x22b)-(0x316, 0x24e), three columns (2, 1, 1): "Cancel" (id 0x34), "Remove" (id 0x35, or grey text with an
/// empty nickname) and "Ok" (id 0x33, or grey text with an empty nickname).
/// Original: `staff_screens_build_nickname_dialog` (0x88bc70, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_build_nickname_dialog() {
    covered!("tel39_1 nickname dialog");
    let text = tel36_langlib();
    let columns: [u8; 4] = [2, 1, 1, 0];
    let person = get_screen_param(0);
    let nickname = get_screen_param(1);
    exe::background_set_random_picture();
    exe::database_format_staff_name(person, JOB_TEXT, 0x7d0, 1);
    let second = exe::get_heading_colour(1) as u32;
    let first = exe::get_heading_colour(0) as u32;
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, first, second, JOB_TEXT);
    exe::menubar_build(4, 0);
    text(JOB_TEXT, STAFF_NICKNAME_TITLE);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 0);
    tel26_add_item(current_human_display(), [0x80018, 0x96, 0x136, 0x2e4, 0x172, 0, 0, 0x22, job_colour(JOB_COLOUR_BE04), 0, 0xc, 4,
        job_colour(JOB_COLOUR_3184), nickname as u32, 0x18, NICKNAME_ENTRY as u32, 0, u32::MAX]);
    let area = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 3, columns.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    let (grey, pale) = (job_colour(JOB_COLOUR_317E), job_colour(JOB_COLOUR_3184));
    text(JOB_TEXT, STAFF_CANCEL);
    tel26_area_button([area, 0, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, NICKNAME_CANCEL as u32, 0]);
    let empty = read_u8(nickname) == 0;
    for (column, label, id) in [(1u32, STAFF_REMOVE, NICKNAME_REMOVE), (2, STAFF_OK, NICKNAME_OK)] {
        text(JOB_TEXT, label);
        if !empty {
            tel26_area_button([area, column, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, id as u32, 0]);
        } else {
            // tel39_1: an empty nickname greys the button
            tel26_area_text([area, column, 0, 0x30, grey, 0x2c, 3, pale, JOB_TEXT as u32]);
        }
    }
}

/// Rule: the nickname dialog: typing redraws; Remove takes the nickname away, Ok stores it (changing the old one or
/// adding it, unless it is the person's common name already), Cancel closes; the dialog frees its buffer on each
/// closing button. Other widgets go to the menu bar.
///
/// `staff_screens_handle_nickname_dialog_action(widget)`: the person (parameter 0) and the buffer (1). By the
/// widget's id: 0x32 (the entry): −11. 0x35 (Remove): `HUMAN_MANAGER::remove_nickname(person, 0)`, free the buffer,
/// −9. 0x34 (Cancel): free, −9. 0x33 (Ok): when the person's `staff::COMMON_NAME` is set, non-empty and equal
/// (strcmp) to the buffer, nothing is stored; otherwise `HUMAN_MANAGER::has_nickname(person, 0)` chooses change or
/// add (person, buffer, 0); free, −9. Otherwise `menubar_handle_command`, then `menubar_handle_object_link`.
/// Original: `staff_screens_handle_nickname_dialog_action` (0x88bee0, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_handle_nickname_dialog_action(raw: u32) -> i32 {
    let person = Staff(get_screen_param(0));
    let nickname = get_screen_param(1);
    match tel26_item_id(raw as i16) {
        NICKNAME_ENTRY => -11,
        NICKNAME_REMOVE => {
            covered!("tel39_1 nickname removed");
            let g: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_HUMAN_MANAGER_REMOVE_NICKNAME);
            g(read_usize(G_HUMAN_MANAGER), person.addr(), 0);
            exe::crt_free(nickname);
            -9
        }
        NICKNAME_CANCEL => {
            exe::crt_free(nickname);
            -9
        }
        NICKNAME_OK => {
            let common = person.common_name();
            let same = common != 0 && read_u8(common) != 0
                && core::ffi::CStr::from_ptr(common as *const core::ffi::c_char) == core::ffi::CStr::from_ptr(nickname as *const core::ffi::c_char);
            if !same {
                covered!("tel39_1 nickname stored");
                if exe::human_manager_has_nickname(read_usize(G_HUMAN_MANAGER), person, 0) != 0 {
                    nickname_call(FN_HUMAN_MANAGER_CHANGE_NICKNAME, person.addr(), nickname);
                } else {
                    nickname_call(FN_HUMAN_MANAGER_ADD_NICKNAME, person.addr(), nickname);
                }
            }
            exe::crt_free(nickname);
            -9
        }
        _ => manager_menu_bar(raw),
    }
}

/// Rule: the scout request dialog asks whether the scouts assess the player now (scouts free) or watch him when they
/// become available, under the human's name in his club's colours, with No and Yes.
///
/// `staff_screens_build_scout_request_dialog()`: the player (parameter 0, an id) and whether scouts are free (1);
/// the slot's human and his club (`staff::CLUB_JOB`, else `staff::NATIONAL_JOB`). The player's background picture;
/// with a club its kit 1 colours (else `JOB_COLOUR_BDF0` / `JOB_COLOUR_3184`). The player's name (style 1, 400
/// bytes, local); the title: the human's name (style 1, 2000 bytes); the subtitle: the player's name (style 1). The
/// question (assess when scouts are free, watch otherwise) with the player's name, as a text area (0x6e, 0x8c)-(0x30c,
/// 0x221), rows 0x195 / the height of font 3, linking the name to the player (0x7d7); the menu bar (4, 0); the button
/// area (0x64, 0x22b)-(0x316, 0x24e), columns (3, 1): "No" (id 0x36) and "Yes" (id 0x37).
/// Original: `staff_screens_build_scout_request_dialog` (0x88c100, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_build_scout_request_dialog() {
    covered!("tel39_1 scout request dialog");
    let text = tel36_langlib();
    let columns: [u8; 2] = [3, 1];
    let mut fore = read_u16(JOB_COLOUR_BDF0) as u32;
    let mut back = read_u16(JOB_COLOUR_3184) as u32;
    let player = tel36_staff_param(0);
    let available = get_screen_param(1);
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    let human = Staff(read_usize((G_SCREEN_RECORDS_MANAGER as isize + slot * 0x300) as usize));
    let mut club = human.club_job().addr();
    if club == 0 {
        club = human.national_job().addr();
    }
    exe::background_set_staff_picture(player, 0, 0);
    if club != 0 {
        exe::database_get_club_kit_colours(club, 1, &mut fore, &mut back);
    }
    let mut name = [0u8; 0x190];
    exe::database_format_staff_name(player, name.as_mut_ptr() as usize, 0x190, 1);
    exe::database_format_staff_name(human, JOB_TEXT, 0x7d0, 1);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, JOB_TEXT);
    let n = core::ffi::CStr::from_ptr(name.as_ptr() as *const core::ffi::c_char).to_bytes_with_nul().len();
    core::ptr::copy_nonoverlapping(name.as_ptr(), JOB_TEXT as *mut u8, n);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    // tel39_1: assess now when the scouts are free
    text(JOB_TEXT, if available != 0 { STAFF_SCOUT_ASSESS } else { STAFF_SCOUT_WATCH }, name.as_ptr() as usize);
    let rows = (0x195 / exe::font_get_height(3)) as u8 as i8 as i32 as u32;
    let area_text: unsafe extern "C" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) = core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    area_text(0x6e, 0x8c, 0x30c, 0x221, 2, 0, rows, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, name.as_ptr() as u32, 0x7d7, player as u32, 0);
    exe::menubar_build(4, 0);
    let area = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, columns.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    let (grey, pale) = (job_colour(JOB_COLOUR_317E), job_colour(JOB_COLOUR_3184));
    text(JOB_TEXT, STAFF_NO);
    tel26_area_button([area, 0, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, SCOUT_REQUEST_NO as u32, 0]);
    text(JOB_TEXT, STAFF_YES);
    tel26_area_button([area, 1, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, SCOUT_REQUEST_YES as u32, 0]);
}

/// Rule: No closes the scout request; Yes adds a scout report request for the player (parameter 0) and closes; the
/// rest goes to the menu bar.
///
/// `staff_screens_handle_scout_request_action(widget)`: 0x36: −9; 0x37: `SCOUT_MANAGER::add_report_request(player,
/// 0)`, −9; otherwise `menubar_handle_command`, then `menubar_handle_object_link`.
/// Original: `staff_screens_handle_scout_request_action` (0x88c360, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_handle_scout_request_action(raw: u32) -> i32 {
    match tel26_item_id(raw as i16) {
        SCOUT_REQUEST_NO => -9,
        SCOUT_REQUEST_YES => {
            covered!("tel39_1 scout request added");
            let g: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SCOUT_MANAGER_ADD_REPORT_REQUEST);
            g(read_usize(G_SCOUT_MANAGER), tel36_staff_param(0), 0);
            -9
        }
        _ => manager_menu_bar(raw),
    }
}

/// Comparer for `staff_screens_add_squad_and_history_menu`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_staff_screens_add_squad_and_history_menu(person: usize, club_side: u32, colour: u32, colour_text: u32,
    squad_handler: u32, history_handler: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32) -> u32>(IDX_STAFF_SCREENS_ADD_SQUAD_AND_HISTORY_MENU);
    let mut r = 0;
    compare_match_screen_build("staff profile squad and history menu", &[],
        || r = o(person, club_side, colour, colour_text, squad_handler, history_handler),
        || { staff_screens_add_squad_and_history_menu(person, club_side, colour, colour_text, squad_handler, history_handler); });
    r
}

/// Comparer for `staff_screens_compare_text_rows`: the order side by side.
pub(crate) unsafe extern "C" fn compare_staff_screens_compare_text_rows(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_STAFF_SCREENS_COMPARE_TEXT_ROWS)(a, b);
    let n = staff_screens_compare_text_rows(a, b);
    SCOREBOARD.record(r.signum() == n.signum(), || format!("staff_screens_compare_text_rows original={} rust={}", r, n));
    r
}

/// Comparer for `staff_screens_build_nickname_dialog`: the dialog side by side over the display list.
pub(crate) unsafe extern "C" fn compare_staff_screens_build_nickname_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_STAFF_SCREENS_BUILD_NICKNAME_DIALOG);
    compare_by_display_list_with("nickname dialog (staff_screens_build_nickname_dialog)", &[], || o(), || staff_screens_build_nickname_dialog());
}

/// Comparer for `staff_screens_handle_nickname_dialog_action`: typing (0x32) side by side over the screen params;
/// the closing buttons (they free the buffer and store the nickname) and the menu bar only the original.
pub(crate) unsafe extern "C" fn compare_staff_screens_handle_nickname_dialog_action(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_STAFF_SCREENS_HANDLE_NICKNAME_DIALOG_ACTION);
    if tel26_item_id(item as i16) != NICKNAME_ENTRY {
        // simplification: storing the nickname, freeing the buffer and the menu bar do not run twice; proven in Rust mode
        return o(item);
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&screen_param_regions(), || o(item), || staff_screens_handle_nickname_dialog_action(item));
    SCOREBOARD.record(same && r == n, || format!("nickname dialog typing: original={} rust={} differences {:?}", r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `staff_screens_build_scout_request_dialog`: the dialog side by side over the display list.
pub(crate) unsafe extern "C" fn compare_staff_screens_build_scout_request_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_STAFF_SCREENS_BUILD_SCOUT_REQUEST_DIALOG);
    compare_by_display_list_with("scout request dialog (staff_screens_build_scout_request_dialog)", &[], || o(), || staff_screens_build_scout_request_dialog());
}

/// Comparer for `staff_screens_handle_scout_request_action`: No side by side; Yes (a scout request) and the menu
/// bar only the original.
pub(crate) unsafe extern "C" fn compare_staff_screens_handle_scout_request_action(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_STAFF_SCREENS_HANDLE_SCOUT_REQUEST_ACTION);
    if tel26_item_id(item as i16) != SCOUT_REQUEST_NO {
        // simplification: the scout request and the menu bar do not run twice; proven in Rust mode
        return o(item);
    }
    let (r, n, same, _, _) = crate::comparison::side_by_side(&screen_param_regions(), || o(item), || staff_screens_handle_scout_request_action(item));
    SCOREBOARD.record(same && r == n, || format!("scout request No: original={} rust={}", r, n));
    r
}

/// Opens the staff profile screen of `staff` on tab `tab` and writes its 22 params.
///
/// With no staff: the error box (staff_screens.cpp line 0x149). The screen opens when the person is viewable: someone
/// who is not a manager (`staff::JOB_FOR_CLUB` 2) and is outside the 16 last staff records (id below
/// `G_STAFF_COUNT` − 16 or not below the count); a player sheet (`staff::PLAYER`) with an id below the count; or,
/// with any id below the count, someone who is neither 1 nor 2 in `staff::JOB_FOR_CLUB`. Otherwise nothing.
///
/// With `keep_view`, params 0..59 of the current screen are kept first. `exe::screenmanager_open_screen` with the
/// builder 0x870700, the handler 0x888260 and the close handler 0x88b8a0; then the params: 0 the person, 1 the tab (5
/// for one of the 16 last records), 2 `slot2` (both sign-extended bytes), 3 zero, 4 and 5: 1 and 1 for a non-player
/// sheet (`staff::NON_PLAYER`) below the count that has no player sheet, or whose job is not 0xb, 0xc, 0xd or 0xf, or
/// with `attribute_mode` 1; otherwise `attribute_mode` and 2; 6..0xe zero, 0xf and 0x10 one, 0x11 the blue colour,
/// 0x12 the pale grey colour, 0x13 zero, 0x14 and 0x15 one. With `keep_view`, params 1, 2, 5 and 0x13 come back from
/// the old screen, and 4 becomes 2 when it was 2.
/// Original: `staff_screens_open_staff_profile` (0x8702a0, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_open_staff_profile(staff: usize, tab: u32, slot2: u32, keep_view: u32, attribute_mode: u32) {
    let staff = Staff(staff);
    if staff.is_null() {
        covered!("tel07_1 staff profile without a person");
        return show_error_at(0xa84aac, 0x149);
    }
    let job = staff.job_for_club();
    let n = read_i32(G_STAFF_COUNT);
    let id = read_i32(staff.addr());
    let last_records = id >= n.wrapping_sub(16) && id < n; // tel07_1 the 16 last staff records
    let viewable = (job != 2 && !last_records) || (staff.player().addr() != 0 && id < n) || (job != 1 && job != 2 && id < n);
    if !viewable {
        covered!("tel07_1 staff profile not viewable");
        return;
    }
    let mut kept = [0usize; 0x3c];
    if keep_view != 0 {
        for (k, v) in kept.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
    }
    let tab = if last_records { 5 } else { tab as u8 };
    if exe::screenmanager_open_screen(SCREEN_MANAGER, 0x870700, 0x888260, 0, 0x88b8a0, 0) == 0 {
        return;
    }
    covered!("tel07_1 open the staff profile");
    set_screen_param(0, staff.addr());
    set_screen_param(1, tab as i8 as isize as usize);
    set_screen_param(2, slot2 as u8 as i8 as isize as usize);
    set_screen_param(3, 0);
    let mode = attribute_mode as u8;
    let id = read_i32(staff.addr());
    let job = staff.job_for_club();
    let full_view = id < read_i32(G_STAFF_COUNT)
        && staff.non_player().addr() != 0
        && (staff.player().addr() == 0 || !matches!(job, 0xb | 0xf | 0xd | 0xc) || mode == 1);
    if full_view {
        set_screen_param(4, 1);
        set_screen_param(5, 1);
    } else {
        set_screen_param(4, mode as i8 as isize as usize);
        set_screen_param(5, 2);
    }
    for k in 6..=0xe {
        set_screen_param(k, 0);
    }
    set_screen_param(0xf, 1);
    set_screen_param(0x10, 1); // tel07_1 param 0x10
    set_screen_param(0x11, colour_word(G_GUI_COLOUR_BLUE));
    set_screen_param(0x12, colour_word(G_GUI_COLOUR_PALE_GREY));
    set_screen_param(0x13, 0);
    set_screen_param(0x14, 1);
    set_screen_param(0x15, 1);
    if keep_view != 0 {
        covered!("tel07_1 staff profile keeps the view");
        for k in [1usize, 2, 5, 0x13] {
            set_screen_param(k as u32, kept[k]);
        }
        if kept[4] == 2 {
            set_screen_param(4, 2);
        }
    }
}

/// Opens the compare players screen.
///
/// With no first compare player (`SCOUT_MANAGER::get_compare_player` 1 of `G_SCOUT_MANAGER`) and no `manager`: the
/// error box (staff_screens.cpp line 0x1ac). Otherwise `exe::screenmanager_open_screen` with the builder 0x876520, the
/// handler 0x889c30 and the close handler 0x88b8a0 (no params). The callers push five arguments; only the first is
/// read.
/// Original: `staff_screens_open_compare_players_screen` (0x870650, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_open_compare_players_screen(manager: usize) {
    if exe::scout_manager_get_compare_player(read_usize(G_SCOUT_MANAGER), 1) == 0 && manager == 0 {
        covered!("tel07_1 compare players without players");
        return show_error_at(0xa84aac, 0x1ac);
    }
    covered!("tel07_1 open the compare players");
    exe::screenmanager_open_screen(SCREEN_MANAGER, 0x876520, 0x889c30, 0, 0x88b8a0, 0);
}

/// Comparer for `staff_screens_open_staff_profile`: opening a screen does not run twice (the screen stack, the
/// network), so only the original runs and its 22 params are checked afterwards against the plan
/// (`staff_profile_planned_params`), when the new screen holds the person in param 0.
pub(crate) unsafe extern "C" fn compare_staff_screens_open_staff_profile(staff: usize, tab: u32, slot2: u32, keep_view: u32, attribute_mode: u32) {
    let plan = if staff == 0 { None } else { staff_profile_planned_params(staff, tab, slot2, keep_view, attribute_mode) };
    original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32)>(IDX_STAFF_SCREENS_OPEN_STAFF_PROFILE)(staff, tab, slot2, keep_view, attribute_mode);
    if let Some(plan) = plan {
        if get_screen_param(0) == staff {
            let mut got = [0usize; 0x16];
            for (k, v) in got.iter_mut().enumerate() {
                *v = get_screen_param(k as u32);
            }
            SCOREBOARD.record(got == plan, || format!("staff profile screen (open {:#x}): params {:x?} x plan {:x?}", staff, got, plan));
        }
    }
}

/// Comparer for `staff_screens_open_compare_players_screen`: opening a screen does not run twice; only the original.
pub(crate) unsafe extern "C" fn compare_staff_screens_open_compare_players_screen(manager: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(IDX_STAFF_SCREENS_OPEN_COMPARE_PLAYERS_SCREEN)(manager)
}

/// Builds the staff profile screen of the person in param 0 on the tab of param 1: the setup and the creations
/// (`profile_screen_setup`), the heading (`profile_screen_heading`; no nation: the error box, line 0x308, and nothing
/// more), the Action menu (`profile_screen_action_menu`) and the compare entries, the tab bar (`profile_screen_tabs`),
/// the identity block (`profile_screen_identity`), the statistics table of a player of the database
/// (`profile_screen_statistics`), then the tab: 1 profile (0x87a510), 2 injuries and bans
/// (`profile_screen_injuries_tab`), 3 contract (0x87cd90), 4 transfer (0x87dd80), 5 history (0x885820), another tab
/// the error box (line 0x7de); the squad and history menu (0x88b910) and the Back/Next buttons.
///
/// Convention: cdecl, SEH frame (0x1d10 bytes of locals): a Text and 25 `MenuItem` for the tab bar. Split into named
/// pieces; the whole routine is hooked.
/// Original: `staff_screens_build_profile_screen` (0x870700, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_build_profile_screen() {
    let mut st = ProfileScreen {
        staff: 0,
        tab: 0,
        view: 0,
        list: 0,
        mode: 0,
        flags: 0,
        history: 0,
        history_index: 0,
        history_row: 0,
        compare_columns: 0,
        compare1: 0,
        compare2: 0,
        viewer: 0,
        viewer_club: 0,
        viewer_nation_team: 0,
        contract: 0,
        offer: 0,
        fg: 0,
        bg: 0,
        y: 0,
        bottom: 0,
        text: [0; 2],
        name: [0; 0xc8],
        action: [0; 0x100],
    };
    exe::new_text(st.text.as_mut_ptr(), 0);
    let mut items = [0u8; PROFILE_TAB_COUNT * PROFILE_TAB_ITEM];
    let ip = items.as_mut_ptr() as usize;
    exe::crt_vector_constructor_iterator(ip, PROFILE_TAB_ITEM as u32, PROFILE_TAB_COUNT as u32, 0x4568f0, 0x5a53c0);
    let align: [u8; 12] = [6, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2];
    covered!("tel07_3 build the staff profile");
    profile_screen_setup(&mut st);
    if !profile_screen_heading(&mut st) {
        show_error_at(0xa84aac, 0x308);
        exe::crt_vector_destructor_iterator(ip, PROFILE_TAB_ITEM as u32, PROFILE_TAB_COUNT as u32, 0x5a53c0);
        exe::drop_text(st.text.as_mut_ptr());
        return;
    }
    let menu = profile_screen_action_menu(&mut st);
    profile_screen_compare_entries(&mut st, menu);
    profile_screen_tabs(&mut st, ip);
    profile_screen_identity(&mut st);
    let s = Staff(st.staff);
    if s.player().addr() != 0 && (s.is_null() || read_i32(s.addr()) < read_i32(G_STAFF_COUNT)) {
        profile_screen_statistics(&mut st, align.as_ptr() as usize);
    }
    let (y, bottom) = (st.y, st.bottom);
    match st.tab as i32 {
        1 => drop(exe::staff_screens_draw_profile_tab(y, bottom)),
        2 => profile_screen_injuries_tab(&mut st),
        3 => drop(core::mem::transmute::<usize, unsafe extern "C" fn(i32, i32) -> u32>(0x87cd90)(y, bottom)),
        4 => drop(core::mem::transmute::<usize, unsafe extern "C" fn(i32, i32) -> u32>(0x87dd80)(y, bottom)),
        5 => drop(core::mem::transmute::<usize, unsafe extern "C" fn(i32, i32) -> u32>(0x885820)(y, bottom)),
        _ => {
            covered!("tel07_3 profile on an unknown tab");
            show_error_at(0xa84aac, 0x7de);
        }
    }
    // simplification: the original pushes the view byte in a dword whose high bytes are stack garbage; here zero-extended
    exe::staff_screens_add_squad_and_history_menu(s, st.view as u32, st.bg, st.fg, 1, 0x1e);
    let next = (exe::screenmanager_is_forward_unavailable(SCREEN_MANAGER) == 0) as u32;
    let back = (exe::screenmanager_is_back_unavailable(SCREEN_MANAGER, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(back, next);
    exe::crt_vector_destructor_iterator(ip, PROFILE_TAB_ITEM as u32, PROFILE_TAB_COUNT as u32, 0x5a53c0);
    exe::drop_text(st.text.as_mut_ptr());
}

/// Comparer for `staff_screens_build_profile_screen`: the builder creates game objects (the offer list, a staff
/// contract signed with the contract manager, the career history) and does not run twice, so only the original
/// builds for the game and the result is checked afterwards. Before it runs, the plan (`profile_screen_plan`) says what
/// it will create; afterwards the creations are checked against the plan (param 3 set, the person's contract of type 1
/// with his id, param 6 set, param 5 masked), and the copy builds its display list in a scratch: the screen (the
/// screen manager, the Display, the text buffer, the generator) goes back to before the original, the game keeps what
/// the original created (so the copy finds the list, the contract and the history instead of creating them), and the
/// copy's display list, params and writes are compared with the original's; then the original's screen comes back.
pub(crate) unsafe extern "C" fn compare_staff_screens_build_profile_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_STAFF_SCREENS_BUILD_PROFILE_SCREEN);
    let plan = profile_screen_plan();
    let s = Staff(plan.staff);
    let mut screen = screen_build_regions();
    screen.push((PROFILE_BUF, 0x100));
    screen.push((0xae290c, 4));
    screen.push((PROFILE_SCREEN_RESET, 4));
    screen.push((G_CONTRACT_LOOKUP_RESULT, 4));
    screen.extend_from_slice(&crate::rng::STATE_REGIONS);
    // the background picture's name (parameter block +0x1fc, `ScreenManager::get_background_picture` 0x7fee9d) goes
    // back too: the copy picks a picture only while none is set, and the original's would stop it drawing (22/11/2026)
    {
        let slot = read_i16(SCREEN_MANAGER + screen_manager::CURRENT_SLOT) as isize;
        let block = core::ptr::read_unaligned((SCREEN_MANAGER as isize + (screen_manager::PARAM_BLOCK as isize) + slot * 0x300) as *const usize);
        if block != 0 {
            screen.push((block + 0x1fc, 4));
        }
        // the human's menu bar record: the builder's `menubar_build(4, 0)` updates it, and the copy's must start from
        // the same record (without the refresh time, read from the clock), as `compare_menubar_build` does
        let manager = read_usize(menubar_slot(MENUBAR_SLOT_MANAGER));
        if manager != 0 {
            let k = read_i32(manager).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10);
            if (0..0x10).contains(&k) {
                let rec = MENUBAR_RECORDS + k as usize * MENUBAR_RECORD_SIZE;
                screen.push((rec, menubar_record::REFRESH_TIME));
                screen.push((rec + menubar_record::HEADER_WIDGET, MENUBAR_RECORD_SIZE - menubar_record::HEADER_WIDGET));
            }
        }
    }
    let before = crate::comparison::MemorySnapshot::read(&screen);
    let (p3_before, p6_before) = (get_screen_param(3), get_screen_param(6));
    let mut draws_original = Vec::new();
    let rolled = training_rolled_by(|| draws_original = crate::rng::trace(|| o()).1);
    let mut problems: Vec<String> = Vec::new();
    let p3 = get_screen_param(3);
    if (plan.list && p3 == 0) || (!plan.list && p3 != p3_before) {
        problems.push(format!("offer list: plan {} x param 3 {:#x} -> {:#x}", plan.list, p3_before, p3));
    }
    if plan.contract {
        let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), s));
        if c.is_null() || read_i32(c.addr()) != read_i32(s.addr()) || c.contract_type() & 0x3f != 1 {
            problems.push(format!("staff contract planned, the original signed {:#x}", c.addr()));
        }
    }
    let p6 = get_screen_param(6);
    if (plan.history && p6 == 0) || (!plan.history && p6 != p6_before) {
        problems.push(format!("career history: plan {} x param 6 {:#x} -> {:#x}", plan.history, p6_before, p6));
    }
    if get_screen_param(5) as u32 != plan.flags {
        problems.push(format!("param 5 {:#x} x plan {:#x}", get_screen_param(5), plan.flags));
    }
    let mut all = screen.clone();
    let slot = read_i16(SCREEN_MANAGER + screen_manager::CURRENT_SLOT) as isize;
    let block = core::ptr::read_unaligned((SCREEN_MANAGER as isize + (screen_manager::PARAM_BLOCK as isize) + slot * 0x300) as *const usize);
    if block != 0 {
        all.push((block, 0xb4));
        all.push((block + 0xb8, 0x200 - 0xb8));
    }
    if !s.is_null() {
        all.push((s.addr() + staff::PLAYING_SQUAD, 1));
        if read_i32(s.addr()) >= 0 && read_i32(s.addr()) < read_i32(G_STAFF_COUNT) {
            all.push((profile_state(s.addr()) + staff_state::EXTRA_FLAGS, 1));
        }
    }
    let after_orig = crate::comparison::MemorySnapshot::read(&all);
    before.write_back();
    let draws_rust = crate::rng::trace(|| crate::rng::without_counting(|| staff_screens_build_profile_screen())).1;
    let after_rust = crate::comparison::MemorySnapshot::read(&all);
    after_orig.write_back();
    // the original rolled a week's training: the copy cannot draw it again, only the plan is compared
    let equal = after_orig == after_rust || rolled;
    SCOREBOARD.record(equal && problems.is_empty(), || {
        format!("staff profile screen ({:#x}, table {}): plan {:?}; draws (n, answer) original {:?} rust {:?}; differences in {:?}", s.addr(), plan.tab, problems, draws_original, draws_rust, first_differences(&after_orig, &after_rust))
    });
}

/// Draws the Contract tab of the staff profile (the person of screen parameter 0) between screen rows `top` and
/// `bottom`.
///
/// Without contract information (a person whose `job_for_club` is 1 or 2, a national job 5 without a club job or
/// with a club job other than 11, or an id from the person count − 16 up), only "There is no contract information
/// available." in font 3. Otherwise a two-column table (weights 2 and 7, as many rows as fit in font 2), labels in
/// pale grey and values in yellow: Type (`contract_tab_type_text`); Wages (`STAFF_CONTRACT::format_wage_text`, "-"
/// without a contract); for a normal contract (type 2) of a player whose `job_for_club` is 11, 12, 13 or 15 while the
/// FIFA transfer rules are in force: Started (the start date when it is after the rule change date
/// `G_TRANSFER_RULE_CHANGE_DATE`, year then day; else "Signed before new transfer system came into force" in English
/// or "N/A") and Protected (`check_fifa_protected_period`: "Yes" or "No (At least %d years since start of contract)");
/// Expires (`format_end_date_text`); Squad Status (`shortlist_manager_squad_status_text` of the high nibble of
/// `contract::SQUAD_STATUS`, a button 5 when the current human controls the club) when the human controls the
/// contract's club, the job is a player job and the contract is not a trial (type 5); for a loan, Loan Details ("On
/// loan from <club> until <date>", the club linked 0x7d5, wrapped from x 0x102), "Cannot play in cup matches" when
/// loan byte +0x1c is 0 and "Cannot play in matches against <parent club>" (linked) when loan byte +0x1d is 0;
/// Bonuses (each of the three bonuses ≥ 0, `staff_contract_format_bonus_text`, or "None"); Clauses (each of the five
/// release clause bytes set, `staff_contract_format_release_clause_name` and the release fee as money, in red for a
/// clause byte 2, or "None"); Work Permit (`contract_tab_work_permit_text`) when the person has a permit, needs one
/// for the club, has a normal contract and is a player; Notes (the human's note about the person, at most 499
/// characters, as button 0x2c; "- " without one); then empty rows to the end. Each lookup of the contract also leaves
/// it in `G_CONTRACT_LOOKUP_RESULT`.
///
/// Defects of the original (kept): every clause shows the same amount (`contract::RELEASE_FEE`); the Loan Details and
/// Work Permit lines read the club contract without checking it (a loan without a club contract would crash).
/// Original: `staff_screens_draw_contract_tab` (0x87cd90, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_draw_contract_tab(top: i32, bottom: i32) {
    const LABEL: usize = G_GUI_COLOUR_PALE_GREY;
    const VALUE: usize = G_GUI_COLOUR_YELLOW;
    let lang = staff_tab_lang();
    let format = staff_tab_format();
    let weights: [u8; 2] = [2, 7];
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0);
    let p = Staff(get_screen_param(0));
    let job = p.job_for_club();
    let count = read_i32(G_STAFF_COUNT);
    if job == 1
        || (p.job_for_nation() == 5 && (p.club_job().is_null() || job != 0xb))
        || job == 2
        || read_i32(p.addr()) >= count - 0x10
        || (!p.is_null() && read_i32(p.addr()) >= count)
    {
        covered!("tel08_1 contract tab without information");
        staff_tab_no_information(top, bottom, 0xa8638c); // "There is no contract information available."
        exe::drop_text(t);
        return;
    }
    covered!("tel08_1 contract tab");
    let rows = staff_tab_rows(top, bottom, 2);
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 2, weights.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    let mut row: i32 = 0;
    let mut buf = [0u8; 0x200];
    let b = buf.as_mut_ptr() as usize;
    let contract = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p));
    let label = |row: i32, s: usize| {
        staff_tab_cell(area, 0, row, 2, 0, 1, 2, LABEL, s);
    };
    let value = |row: i32, s: usize| {
        staff_tab_cell(area, 1, row, 2, 0, 1, 2, VALUE, s);
    };

    lang(JOB_TEXT, 0xa86874usize, 0xad9c64usize); // "  Type<%s - COMMENT - contract type>"
    label(row, JOB_TEXT);
    contract_tab_type_text(contract);
    value(row, JOB_TEXT);
    row += 1;

    lang(JOB_TEXT, 0x98f9dcusize); // "  Wages"
    label(row, JOB_TEXT);
    if !contract.is_null() {
        exe::staff_contract_format_wage_text(contract, b, 1, 2);
        format(JOB_TEXT, 0x987ff0usize, b); // "  %s"
    } else {
        lang(JOB_TEXT, 0x98e8c8usize); // "  -"
    }
    value(row, JOB_TEXT);
    row += 1;

    if !contract.is_null()
        && contract.contract_type() & 0x3f == 2
        && !p.player().is_null()
        && matches!(p.job_for_club(), 0xb | 0xd | 0xf | 0xc)
        && read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1
    {
        covered!("tel08_1 contract tab fifa rules");
        lang(JOB_TEXT, 0xa8671cusize, 0xad9c64usize); // "  Started<%s - COMMENT contract start date>"
        label(row, JOB_TEXT);
        let rule_day = read_i16(G_TRANSFER_RULE_CHANGE_DATE);
        let rule_year = read_u16(G_TRANSFER_RULE_CHANGE_DATE + 2);
        let year = contract.date_started_year();
        let after_rule = rule_year < year || (rule_year == year && rule_day < (contract.date_started() as i16));
        if after_rule {
            exe::date_get_short_date_string(contract.addr() + contract::DATE_STARTED, b, 0xc8, 0xff);
            format(JOB_TEXT, 0x987ff0usize, b); // "  %s"
        } else if exe::langlib_get_language() == 0 {
            format(JOB_TEXT, 0xa866e8usize); // "  Signed before new transfer system came into force"
        } else {
            lang(JOB_TEXT, 0xa86748usize); // "  N/A"
        }
        value(row, JOB_TEXT);
        row += 1;
        lang(JOB_TEXT, 0xa866b8usize, 0xad9c64usize); // "  Protected<%s - COMMENT contract protected>"
        label(row, JOB_TEXT);
        let mut years: i8 = 0;
        if exe::staff_contract_check_fifa_protected_period(contract, 0usize, 0usize, &mut years as *mut i8 as usize) != 0 {
            lang(JOB_TEXT, 0xa866b0usize); // "  Yes"
        } else {
            lang(JOB_TEXT, 0xa86668usize, years as i32); // "  No (At least %d years since start of contract)"
        }
        value(row, JOB_TEXT);
        row += 1;
    }

    lang(JOB_TEXT, 0xa8665cusize); // "  Expires"
    label(row, JOB_TEXT);
    if !contract.is_null() {
        exe::staff_contract_format_end_date_text(contract, b);
        format(JOB_TEXT, 0x987ff0usize, b); // "  %s"
    } else {
        staff_tab_strcpy(JOB_TEXT, 0x98e8c8); // "  -"
    }
    value(row, JOB_TEXT);
    row += 1;

    if !contract.is_null() {
        let (_, club) = contract_tab_lookup(p);
        if exe::human_manager_current_controls_club(club, 0, 1) != 0
            && exe::database_is_player_job(p.job_for_club() as i8) != 0
            && {
                let (c, _) = contract_tab_lookup(p);
                c.is_null() || c.contract_type() & 0x3f != 5
            }
        {
            covered!("tel08_1 contract tab squad status");
            lang(JOB_TEXT, 0xa8664cusize); // "  Squad Status"
            label(row, JOB_TEXT);
            // writes after the two spaces the label line left in the buffer
            exe::shortlist_manager_squad_status_text(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), (contract.squad_status() >> 4) as u32, JOB_TEXT + 2, 2000);
            let (_, club) = contract_tab_lookup(p);
            if exe::human_manager_current_controls_club(club, 0, 1) != 0 {
                staff_tab_button(area, 1, row, 2, 0, 1, 2, VALUE, JOB_TEXT, 5, 0);
            } else {
                value(row, JOB_TEXT);
            }
            row += 1;
        }
    }

    let loan = Contract(exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p));
    if !loan.is_null() {
        covered!("tel08_1 contract tab loan");
        // the loan contract's two bytes: may play in cup matches, may play against the parent club
        const LOAN_CUP_MATCHES: usize = 0x1c;
        const LOAN_AGAINST_PARENT: usize = 0x1d;
        lang(JOB_TEXT, 0xa8663cusize); // "  Loan Details"
        label(row, JOB_TEXT);
        exe::date_get_long_date_string(loan.addr() + contract::CONTRACT_EXPIRES, b, 0xc8, 0xff);
        let club = club_at(contract.club() as isize);
        let gender = read_i8(club.addr() + club::SHORT_GENDER_NAME) as i32;
        // "  On loan from {}<%s - Club Name>{} until {}<%s - Date>"
        lang(JOB_TEXT, 0xa865dcusize, gender, gender, 100u32, club.addr() + club::SHORT_NAME, b);
        exe::text_set_string(t as usize, JOB_TEXT as *const u8);
        exe::text_add_link(t as usize, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
        value(row, 0);
        staff_tab_draw_wrapped(t, area, 0x102, 0x30c, &mut row, 2, VALUE, 2, 0);
        row += 1;
        if read_u8(loan.addr() + LOAN_CUP_MATCHES) == 0 {
            label(row, 0);
            lang(JOB_TEXT, 0xa865bcusize); // "  Cannot play in cup matches"
            value(row, JOB_TEXT);
            row += 1;
        }
        if read_u8(loan.addr() + LOAN_AGAINST_PARENT) == 0 {
            let (_, club) = contract_tab_lookup(p);
            label(row, 0);
            let gender = read_i8(club + club::SHORT_GENDER_NAME) as i32;
            lang(JOB_TEXT, 0xa86574usize, gender, gender, club + club::SHORT_NAME); // "  Cannot play in matches against {}<%s>{}"
            exe::text_set_string(t as usize, JOB_TEXT as *const u8);
            exe::text_add_link(t as usize, (club + club::SHORT_NAME) as *const u8, 0x7d5, club);
            value(row, 0);
            staff_tab_draw_wrapped(t, area, 0x102, 0x30c, &mut row, 2, VALUE, 2, 0);
            row += 1;
        }
    }

    lang(JOB_TEXT, 0x98f9d0usize); // "  Bonuses"
    label(row, JOB_TEXT);
    let first = row;
    if !contract.is_null() {
        for i in 0..3u32 {
            if read_i32(contract.addr() + contract::GOAL_BONUS + 4 * i as usize) > -1 {
                if row > first {
                    label(row, 0);
                }
                exe::staff_contract_format_bonus_text(contract, i, b, 100, 2);
                format(JOB_TEXT, 0x987ff0usize, b); // "  %s"
                value(row, JOB_TEXT);
                row += 1;
            }
        }
    }
    if row == first {
        lang(JOB_TEXT, 0xa86550usize, 0xad9c64usize); // "  None<%s - COMMENT - no bonuses>"
        value(row, JOB_TEXT);
        row += 1;
    }

    lang(JOB_TEXT, 0xa86544usize); // "  Clauses"
    label(row, JOB_TEXT);
    let first = row;
    if !contract.is_null() {
        let mut money = [0u8; 0x800];
        for i in 0..5u32 {
            let kind = read_u8(contract.addr() + contract::NON_PROMOTION_RC + i as usize);
            if kind != 0 {
                let fee = contract.release_fee() as u32;
                let amount = exe::cash_get_display_amount(&fee, 6);
                exe::cash_format_money(money.as_mut_ptr() as usize, 2000, amount, 6);
                if row > first {
                    label(row, 0);
                }
                exe::staff_contract_format_release_clause_name(contract, i, b, 100, 2);
                format(JOB_TEXT, 0xa86538usize, b, money.as_ptr() as usize); // "  %s (%s)"
                staff_tab_cell(area, 1, row, 2, 0, 1, 2, if kind == 2 { G_GUI_COLOUR_RED } else { VALUE }, JOB_TEXT);
                row += 1;
            }
        }
    }
    if row == first {
        lang(JOB_TEXT, 0xa86514usize, 0xad9c64usize); // "  None<%s - COMMENT - no clauses>"
        value(row, JOB_TEXT);
        row += 1;
    }

    let permit = exe::transfer_manager_find_work_permit(read_usize(G_TRANSFER_MANAGER), p);
    if permit != 0 {
        let (_, club) = contract_tab_lookup(p);
        if exe::transfer_manager_player_needs_work_permit(read_usize(G_TRANSFER_MANAGER), p, club, 1) != 0
            && contract.contract_type() & 0x3f == 2
            && !p.player().is_null()
        {
            covered!("tel08_1 contract tab work permit");
            lang(JOB_TEXT, 0xa86504usize); // "  Work Permit"
            label(row, JOB_TEXT);
            contract_tab_work_permit_text(permit);
            value(row, JOB_TEXT);
            row += 1;
        }
    }

    lang(JOB_TEXT, 0xa863c0usize); // "  Notes"
    label(row, JOB_TEXT);
    let manager = tel26_slot_manager().addr();
    if manager != 0 && (exe::notes_find_note(read_usize(G_NOTES), manager, p) as usize) != 0 {
        let mut note = [0u8; 0x200];
        exe::notes_format_note_text(read_usize(G_NOTES), manager, p, note.as_mut_ptr() as usize);
        // strncpy(500) then the 500th byte cleared: at most 499 characters
        let mut copy = [0u8; 500];
        if let Some(n) = note.iter().take(500).position(|&c| c == 0) {
            copy[..n].copy_from_slice(&note[..n]);
        } else {
            copy.copy_from_slice(&note[..500]);
        }
        copy[499] = 0;
        format(JOB_TEXT, 0x987ff0usize, copy.as_ptr() as usize); // "  %s"
    } else {
        lang(JOB_TEXT, 0xa863b8usize); // "  - "
    }
    staff_tab_button(area, 1, row, 2, 0, 1, 2, VALUE, JOB_TEXT, 0x2c, 0); // tel08_1 the notes button
    row += 1;
    while row < rows as i32 {
        staff_tab_cell(area, 0, row, 2, 0, 0xc, 2, LABEL, 0);
        staff_tab_cell(area, 1, row, 2, 0, 0xc, 2, LABEL, 0);
        row += 1;
    }
    exe::drop_text(t);
}

/// Comparer for `staff_screens_draw_contract_tab`: both draw side by side and are compared by the display list
/// (`compare_match_screen_build`) plus the contract lookup result the tab leaves (`G_CONTRACT_LOOKUP_RESULT`).
pub(crate) unsafe extern "C" fn compare_staff_screens_draw_contract_tab(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(IDX_STAFF_SCREENS_DRAW_CONTRACT_TAB);
    let extra = [(G_CONTRACT_LOOKUP_RESULT, 4)];
    compare_match_screen_build("staff profile contract tab", &extra, || o(top, bottom), || staff_screens_draw_contract_tab(top, bottom));
}

/// Draws the Profile tab of the staff profile (the person of screen parameter 0, view mode in parameter 4) between
/// screen rows `top` and `bottom`.
///
/// No profile ("There is no profile available.", font 3) for a person whose `job_for_club` is 2 or whose id is from
/// the person count − 16 up (past the count, or a null person). View 2 (the scouting view): the match history when
/// the scouting manager has a search entry for him (`find_club_search_entry`, `profile_tab_match_history`), else the
/// player attributes for a player or the non-player profile; view 0: the player attributes of a player (nothing for
/// a non-player); view 1: the non-player profile of a person with a non-player sheet.
///
/// Convention: cdecl(top, bottom), SEH frame (a Text, destroyed at the end). Split into named pieces; the whole
/// routine is hooked.
/// Original: `staff_screens_draw_profile_tab` (0x87a510, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_draw_profile_tab(top: i32, bottom: i32) {
    let w6: [u8; 6] = [3, 2, 3, 2, 3, 2];
    let w4: [u8; 4] = [3, 2, 3, 2];
    let w2: [u8; 2] = [3, 7];
    let w3: [u8; 3] = [3, 1, 7];
    let w_matches: [u8; 14] = [7, 0xe, 4, 6, 0, 0, 0, 4, 4, 4, 4, 4, 4, 4];
    let w_header: [u8; 14] = [7, 0xe, 4, 1, 2, 2, 1, 4, 4, 4, 4, 4, 4, 4];
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0);
    let manager = tel26_slot_manager().addr();
    let p = Staff(get_screen_param(0));
    let mode = get_screen_param(4);
    let count = read_i32(G_STAFF_COUNT);
    let id = read_i32(p.addr());
    if p.job_for_club() == 2 || (id >= count - 0x10 && (p.is_null() || id >= count)) {
        covered!("tel08_2 profile tab without profile");
        staff_tab_no_information(top, bottom, 0xa859d8); // "There is no profile available."
        exe::drop_text(t);
        return;
    }
    let has_player = !p.player().is_null();
    let has_non_player = !p.non_player().is_null();
    if mode == 2 {
        let entry = exe::scout_manager_find_club_search_entry(read_usize(G_SCOUT_MANAGER), p);
        if entry != 0 {
            profile_tab_match_history(top, bottom, p, entry, t, &w_header, &w_matches);
        } else if has_player {
            profile_tab_player_attributes(top, bottom, p, manager, t, &w6);
        } else if has_non_player {
            profile_tab_non_player(top, bottom, p, &w4, &w3, &w2);
        }
    } else if mode == 0 {
        if has_player {
            covered!("tel08_2 profile tab player");
            profile_tab_player_attributes(top, bottom, p, manager, t, &w6);
        }
    } else if mode == 1 && has_non_player {
        profile_tab_non_player(top, bottom, p, &w4, &w3, &w2);
    }
    exe::drop_text(t);
}

/// Comparer for `staff_screens_draw_profile_tab`: both draw side by side and are compared by the display list
/// (`compare_match_screen_build`) plus the attribute table the tab fills (0xddf570, 36 rows) and the contract lookup
/// result. Both sides bring the player's training up to date (`update_player_training`, as the compare players
/// screen does); when it is due (`training_update_pending`, tel07_2) only the original runs, so the week's training
/// rolls once.
pub(crate) unsafe extern "C" fn compare_staff_screens_draw_profile_tab(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(IDX_STAFF_SCREENS_DRAW_PROFILE_TAB);
    if training_update_pending(get_screen_param(0)) {
        // simplification: the training roll runs once, in the original
        return o(top, bottom);
    }
    let extra = [(PROFILE_TABLE, PROFILE_PLAYER_ROWS * PROFILE_ROW), (G_CONTRACT_LOOKUP_RESULT, 4)];
    compare_match_screen_build("staff profile profile tab", &extra, || o(top, bottom), || staff_screens_draw_profile_tab(top, bottom));
}

/// Draws the Transfer tab of the staff profile (the person of screen parameter 0) between screen rows `top` and
/// `bottom`.
///
/// Without transfer information (a person whose `job_for_club` is 1 or 2, or an id from the person count − 16 up),
/// only "There is no transfer information available." in font 3. Otherwise a two-column table (weights 2 and 7, as
/// many rows as fit in font 2), labels in pale grey and values in yellow, in sections: the details
/// (`transfer_tab_details`: full name, availability, value, second nationality, fluent and basic languages), Offers
/// (`transfer_tab_offers`, with the offers listed in screen parameter 3 and the viewer's club taken from the human of
/// the current screen slot), Interested (`transfer_tab_interested`), Future (`transfer_tab_future`: the contract offer
/// being considered, retirement, and the long list of the person's mood comments by his contract's grievances and
/// flags, his character, his club and its league form, his manager and the board, each a wrapped line) and Potential
/// Fees (`transfer_tab_potential_fees`), then empty rows to the end, the area's scroll row from screen parameter 0xb and
/// the area's index in screen parameter 0xc. Each contract lookup but the first also leaves the contract in
/// `G_CONTRACT_LOOKUP_RESULT`.
///
/// Kept from the original: a few texts are written into the buffer and then overwritten without being shown (the
/// supporters he cares about, the next manager, the enthusiasm after the development line); the "club" checks of the
/// mood comments (a contract always has one); the future transfer's missing offer slot shows the "Error" box (line
/// 0x1902).
/// Original: `staff_screens_draw_transfer_tab` (0x87dd80, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_draw_transfer_tab(top: i32, bottom: i32) {
    let mut list = [0u8; 0x10];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    let weights: [u8; 2] = [2, 7];
    let p = Staff(get_screen_param(0));
    let offers = get_screen_param(3);
    let manager = tel26_slot_manager();
    let viewer = if manager.is_null() { Club(0) } else { manager.club_job() };
    let job = p.job_for_club();
    let count = read_i32(G_STAFF_COUNT);
    if job == 1 || job == 2 || read_i32(p.addr()) >= count - 0x10 || (!p.is_null() && read_i32(p.addr()) >= count) {
        covered!("tel08_3 transfer tab without information");
        staff_tab_no_information(top, bottom, 0xa8689c); // "There is no transfer information available."
        exe::drop_llist(l);
        return;
    }
    covered!("tel08_3 transfer tab");
    let rows = staff_tab_rows(top, bottom, 2);
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 2, weights.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    let mut st = TransferTab {
        area,
        row: 0,
        first: 0,
        p,
        contract: Contract(0),
        club: Club(0),
        viewer,
        future: 0,
        list: l,
        matches_played: 0,
        position: 0,
        final_position: 0,
        losing_run: 0,
        winning_run: 0,
    };
    transfer_tab_details(&mut st);
    transfer_tab_offers(&mut st, offers);
    transfer_tab_interested(&mut st);
    transfer_tab_future(&mut st);
    transfer_tab_potential_fees(&mut st);
    while st.row < rows as i32 {
        staff_tab_cell(area, 0, st.row, 2, 0, 0xc, 2, G_GUI_COLOUR_PALE_GREY, 0);
        staff_tab_cell(area, 1, st.row, 2, 0, 0xc, 2, G_GUI_COLOUR_PALE_GREY, 0);
        st.row += 1;
    }
    exe::screenmanager_set_area_scroll_row(G_SCREEN_MANAGER, area, get_screen_param(0xb) as u32);
    set_screen_param(0xc, area as u16 as i16 as isize as usize);
    exe::drop_llist(l);
}

/// Comparer for `staff_screens_draw_transfer_tab`: both draw side by side and are compared by the display list
/// (`compare_match_screen_build`, which takes the screen's parameter block, so the area index of parameter 0xc) plus
/// the contract lookup result the tab leaves (`G_CONTRACT_LOOKUP_RESULT`).
pub(crate) unsafe extern "C" fn compare_staff_screens_draw_transfer_tab(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(IDX_STAFF_SCREENS_DRAW_TRANSFER_TAB);
    let extra = [(G_CONTRACT_LOOKUP_RESULT, 4)];
    compare_match_screen_build("staff profile transfer tab", &extra, || o(top, bottom), || staff_screens_draw_transfer_tab(top, bottom));
}

/// The contract's morale (`contract::MORALE`, signed byte).
///
/// An out-of-line accessor of staff_screens.cpp, called by the profile's Transfer tab (0x87dd80) only.
/// Original: `staff_screens_contract_morale` (0x8857b0, staff_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn staff_screens_contract_morale(this: usize) -> u8 {
    Contract(this).morale() as u8
}

/// The matches the contract's person was left out (`contract::MATCHES_LEFT_OUT`).
///
/// An out-of-line accessor of staff_screens.cpp, called by the profile's Transfer tab (0x87dd80) only.
/// Original: `staff_screens_contract_matches_left_out` (0x8857c0, staff_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn staff_screens_contract_matches_left_out(this: usize) -> u8 {
    Contract(this).matches_left_out()
}

/// The contract's squad status: the high nibble of `contract::SQUAD_STATUS`.
///
/// An out-of-line accessor of staff_screens.cpp, called by the profile's Transfer tab (0x87dd80) only.
/// Original: `staff_screens_contract_squad_status` (0x8857d0, staff_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn staff_screens_contract_squad_status(this: usize) -> u8 {
    Contract(this).squad_status() >> 4
}

/// Whether the contract's person leaves on a Bosman (`contract::LEAVING_ON_BOSMAN`).
///
/// An out-of-line accessor of staff_screens.cpp, called by the profile's Transfer tab (0x87dd80) only.
/// Original: `staff_screens_contract_leaving_on_bosman` (0x8857e0, staff_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn staff_screens_contract_leaving_on_bosman(this: usize) -> u8 {
    Contract(this).leaving_on_bosman()
}

/// The transfer request bit of `contract::TRANSFER_STATUS` (`transfer_status::TRANSFER_REQUESTED`, 8 or 0, in the
/// whole eax).
///
/// An out-of-line accessor of staff_screens.cpp, called by the profile's Transfer tab (0x87dd80) only.
/// Original: `staff_screens_contract_transfer_requested` (0x8857f0, staff_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn staff_screens_contract_transfer_requested(this: usize) -> u32 {
    (Contract(this).transfer_status() & transfer_status::TRANSFER_REQUESTED) as u32
}

/// The board's confidence in the manager of `club` (`board_record::CONFIDENCE` of the club's 0x49-byte record in the
/// MANAGER_MANAGER board table); 0 without a club.
///
/// An out-of-line accessor of staff_screens.cpp, called by the profile's Transfer tab (0x87dd80) only.
/// Original: `staff_screens_board_confidence` (0x885800, staff_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn staff_screens_board_confidence(this: usize, club: usize) -> u16 {
    if club == 0 {
        covered!("tel09_1 board confidence without club");
        return 0;
    }
    read_u16(read_usize(this).wrapping_add((read_i32(club) as usize).wrapping_mul(0x49)) + board_record::CONFIDENCE)
}

/// Draws the History tab of the staff profile (the person of screen parameter 0) between screen rows `top` and
/// `bottom`.
///
/// Without history (`job_for_club` 1 or 2, or an id from the person count up), only "There is no history information
/// available." in font 3. Otherwise a View button (command 0) with its popup: Achievements (bit 1 of screen parameter
/// 5), and for a player Playing Career (2), Injuries (4) and Bans (8), and Manager Stats (0x10) when the person has a
/// manager record; each line is command 0x14 with its bit, flags 0x2002 for the one in use, which is also the tab's
/// title. With Achievements (the history records) a Filter button with its popup (command 0x20): All Records (0),
/// Competitions (1), Awards (2), and for a club or national job 5 or 0xc Transfers (4) and Jobs (8), the kind in use
/// being screen parameter 0xe; the records of that kind (`Staff_Records` 0x86de70) and, when there are more than 100,
/// the page count in screen parameter 0x10 and the page buttons (`gui_add_page_buttons` with page parameter 0xf,
/// commands 0x21 and 0x22). The title (font 3, yellow) goes from row top + 0x19, and the table from top + 0x3c: the
/// records (`history_tab_records`), the playing career (`history_tab_playing_career`), the injuries, the bans or the
/// manager stats; the table area's scroll row comes from screen parameter 9 and its index goes to parameter 10 (no
/// table, nothing).
///
/// Defects of the original (kept): the seasons' club ids are only checked against -1 and -2, not against a null club.
/// Original: `staff_screens_draw_history_tab` (0x885820, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_draw_history_tab(top: i32, bottom: i32) {
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0);
    let p = Staff(get_screen_param(0));
    let view = get_screen_param(5) as u32;
    // parameters 6, 7, 8 (the playing career) are read again by its view; 0x14 is read and unused
    for k in [6u32, 7, 8] {
        get_screen_param(k);
    }
    let mut kind = get_screen_param(0xe) as u32;
    let page = get_screen_param(0xf);
    get_screen_param(0x14);
    let second_view = get_screen_param(0x15);
    let job = p.job_for_club();
    if job == 1 || job == 2 || (!p.is_null() && read_i32(p.addr()) >= read_i32(G_STAFF_COUNT)) {
        covered!("tel09_2 history tab without information");
        staff_tab_no_information(top, bottom, 0xa8aed8); // "There is no history information available."
        exe::drop_text(t);
        return;
    }
    covered!("tel09_2 history tab");
    let mut title = [0u8; 200];
    let view_area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x168, (top + 0x14) as u32, 2, 0, 1, 0, 1, 0, u32::MAX]);
    staff_tab_lang()(JOB_TEXT, 0x988004usize); // "View"
    let button = staff_tab_button(view_area, 0, 0, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT, 0, 0);
    let menu = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), button]);
    staff_tab_lang()(JOB_TEXT, 0xa8b124usize); // " Achievements"
    let filter = view & 1;
    history_tab_view_line(view, 1, menu, &mut title);
    if !p.player().is_null() {
        staff_tab_lang()(JOB_TEXT, 0xa8b114usize); // " Playing Career"
        history_tab_view_line(view, 2, menu, &mut title);
        staff_tab_lang()(JOB_TEXT, 0xa7fde4usize); // " Injuries"
        history_tab_view_line(view, 4, menu, &mut title);
        staff_tab_lang()(JOB_TEXT, 0xa8b10cusize); // " Bans"
        history_tab_view_line(view, 8, menu, &mut title);
    }
    if exe::staff_records_get_manager_record(read_usize(G_STAFF_RECORDS), p, 0) != 0 {
        staff_tab_lang()(JOB_TEXT, 0xa8b0fcusize); // " Manager Stats"
        history_tab_view_line(view, 0x10, menu, &mut title);
    }
    let mut list = 0usize;
    if filter != 0 {
        covered!("tel09_2 history tab filter");
        staff_tab_lang()(JOB_TEXT, 0x98ebf8usize); // "Filter"
        let button = staff_tab_button(view_area, 1, 0, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT, 0, 0);
        let menu = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), button]);
        let line = |s: usize, value: u32| {
            staff_tab_lang()(JOB_TEXT, s);
            history_tab_menu_line(kind == value, 0x20, value, menu);
        };
        line(0xa8b0ec, 0); // " All Records"
        line(0xa8b0dc, 1); // " Competitions"
        line(0xa8b0d4, 2); // " Awards"
        if matches!(p.job_for_club(), 5 | 0xc) || matches!(p.job_for_nation(), 5 | 0xc) {
            line(0xa8b0c8, 4); // " Transfers"
            line(0xa8b0c0, 8); // " Jobs"
        }
        let records: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_STAFF_HISTORY);
        list = records(read_usize(G_STAFF_RECORDS), p.addr(), kind);
        let rows = ((bottom - top - 0x3c) / exe::font_get_height(2)) as u8;
        kind = (kind & !0xff) | rows as u32;
        if list != 0 && read_i16(list) > 100 {
            covered!("tel09_2 history tab pages");
            let n = read_i16(list) as i32;
            let pages = (n % rows as i32 != 0) as i32 + n / rows as i32;
            set_screen_param(0x10, pages as i16 as i32 as usize);
            exe::gui_add_page_buttons(0x30c, (top) as i32 as u32, (page) as usize as u32, (pages) as i32 as u32, 0, 0x21, 0x22, 0);
        }
    }
    let add_text: unsafe extern "thiscall" fn(usize, u32, i32, u32, i32, u32, u32, u32, u32, u32, usize, u32) -> u32 =
        core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    add_text(G_SCREEN_MANAGER, 0x6e, top + 0x19, 0x30c, top + 0x37, 2, 0, 0xc, 3, job_colour(G_GUI_COLOUR_YELLOW), title.as_ptr() as usize, 1);
    let top = top + 0x3c;
    if filter != 0 {
        history_tab_records(top, bottom, kind, list, page, t);
    } else if view & 2 != 0 {
        covered!("tel09_2 history tab playing career");
        history_tab_playing_career(p, top, bottom, kind, second_view);
    } else if view & 4 != 0 {
        covered!("tel09_2 history tab injuries");
        history_tab_injuries(p, top, bottom, kind);
    } else if view & 8 != 0 {
        covered!("tel09_2 history tab bans");
        history_tab_bans(p, top, bottom, kind);
    } else if view & 0x10 != 0 {
        history_tab_manager_stats(p, top, bottom, kind);
    }
    exe::drop_text(t);
}

/// Comparer for `staff_screens_draw_history_tab`: both draw side by side and are compared by the display list
/// (`compare_match_screen_build`, which also holds the screen parameters the tab writes: the page count 0x10 and the
/// table area 10).
pub(crate) unsafe extern "C" fn compare_staff_screens_draw_history_tab(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(IDX_STAFF_SCREENS_DRAW_HISTORY_TAB);
    compare_match_screen_build("staff profile history tab", &[], || o(top, bottom), || staff_screens_draw_history_tab(top, bottom));
}

/// Answers a click on the staff profile screen (item `item` of the current human's display; its command id, 0 and -1
/// being the default command `G_CURRENT_COMMAND_ID`, and its value).
///
/// On a network game (`[0xb77c84]`), first the pending box answer: 0x18 (the award vote) clears the pending code and
/// argument, and with answer 2 votes (0x417fe0 on the shortlist of screen parameter 0xd, for the slot's manager) and
/// returns -10, clearing parameter 0xd either way; 0x27, 0x42 and 0x3c are only cleared. Then by command: 0x3c / 0x3d
/// parameter 0x14 = 1 / 0 (-11); 1 opens the profile of the item's value on the same tab (`staff_screens_open_staff_profile`);
/// 0x1f 0x4e53e0; 0x1e "first second day.month.year" to 0x5f11c0; 2 the award vote box (the shortlist entry in
/// parameter 0xd, the kit colours of the entry's club, of the award's nation or of the default nation 0x9cf79c,
/// "Vote for ... award ?", "Please Confirm", "Vote For Award", command 0x18); 3 moving the player to the squad
/// (0x4cbd20; when it fails, -11; else "There is no space in the ... squad", command 0x27); 0x1d the shortlist
/// (0x842c00 when there is a transfer manager with a shortlist; -11); 0x2c notes; 0x2e / 0x2d 0x7e5b10 with 1 / 0;
/// 0x2a the club's affiliate or another person of the club's five staff slots (+0x19f) to 0x48bc00 (-11); 0x2b the
/// first of the club's three slots at +0x1cf to 0x7a2a50 (-11); 0x23 0x538bc0 (-11); 5 0x8f6770; 0x24 0x5a8380; 6 an
/// offer of contract (0x8c5450: "Cannot offer contract", command 0x3c; else 0x4dd0f0); 7 and 8 / 0x19 and 0x1a
/// 0x8f8120 with 0 / 1; 9 0x8f8630; 0xa / 0xb 0x8f4160 with 0 / 1; 0x2f / 0x30 0x8f41b0 with 1, 1 / 1, 2; 0x31 the
/// compare players screen (two different players picked: open 0x876520 for the first one or the person, the
/// staff_screens.cpp 428 box without one; else the picking screen 0x88c460); 0xc 0x4dd0f0 with the value; 0xd the
/// value's offer (0x8f2470 for a kind 2 offer, else 0x8f4c70); 0xe 0x8e9350 with the value; 0xf, 0x10, 0x17 and 0x16
/// the approaches of the slot manager's club (0x8ce990 and 0x8f0e40 with kinds 3, 0, 2 and 1; 0x16 first checks the
/// work permit, reason 0x29, and compares the two dates of 0x8d0590 to choose between the approach and 0x8f9690);
/// 0x18 0x8fbcc0; 0x11 the item's value in parameter 1 when it changes (-11); 0x13 0x8e9550 with the value; 0x1b /
/// 0x1c 0x778f60 with the value and 1 / 0; 0x25 and 0x26 the person's name screen (0x88bc70; parameter 0 the person,
/// parameter 1 a 26-byte copy of his name); 0x29 the scouting of the person (0x7f8ff0: assess screen 0x88c100 with
/// parameter 0 his id and 1 = 1 or 0, or the "no scouts", "already scouting" and "no resources" boxes, command 0x42;
/// then 0x7f75f0). Every action returns -4 unless said otherwise. Any other command goes by the view of parameter
/// 1: 1 the squad menu 0x88b2f0, 5 the history menu 0x88b4b0 (their answer when not 0), 4 the scroll row of parameter
/// 0xc into parameter 0xb for a player, another view the staff_screens.cpp 13210 box; then the menu bar's command and
/// object link answers.
///
/// Defects of the original (kept): 0x25 and 0x26 leak the name copy when the screen does not open, and copy the name
/// into 26 bytes without a limit; the error box view path tests an uninitialised local (here 0, the menu bar goes on).
/// Original: `staff_screens_profile_input` (0x888260, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_profile_input(item: u32) -> i32 {
    let p = Staff(get_screen_param(0));
    let view = get_screen_param(1) as u8;
    let mut c1 = get_screen_param(0x11) as u32;
    let mut c2 = get_screen_param(0x12) as u32;
    let manager = tel26_slot_manager();
    if read_u32(0xb77c84) != 0 {
        if read_u32(G_PENDING_COMMAND_CODE) == 0x18 {
            let answer = read_u32(G_PENDING_COMMAND_ARG);
            write_u32(G_PENDING_COMMAND_CODE, 0);
            write_u32(G_PENDING_COMMAND_ARG, 0);
            if answer == 2 {
                covered!("tel09_3 profile input award vote");
                let shortlist = get_screen_param(0xd);
                exe::award_shortlist_cast_vote(read_usize(shortlist + 0x1f), manager, p);
                set_screen_param(0xd, 0);
                return -10;
            }
            set_screen_param(0xd, 0);
        }
        for code in [0x27u32, 0x42, 0x3c] {
            if read_u32(G_PENDING_COMMAND_CODE) == code {
                write_u32(G_PENDING_COMMAND_ARG, 0);
                write_u32(G_PENDING_COMMAND_CODE, 0);
            }
        }
    }
    let id = tel26_item_id(item as i16);
    let value = || tel26_item_value(item as i16);
    let lang = staff_tab_lang();
    let mut name = [0u8; 200];
    let mut subtitle = [0u8; 0x200];
    let mut title = [0u8; 0x200];
    let nm = name.as_mut_ptr() as usize;
    let sub = subtitle.as_mut_ptr() as usize;
    match id {
        0x3c | 0x3d => {
            set_screen_param(0x14, (id == 0x3c) as usize);
            return -11;
        }
        1 => {
            exe::staff_screens_open_staff_profile(value(), view as u32, 1, 1, 0);
            return -4;
        }
        0x1f => {
            let f: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CONTRACT_SCREENS_OPEN_TERMINATE_CONTRACT_SCREEN);
            f(p.addr());
            return -4;
        }
        0x1e => {
            let second = if read_usize(p.addr() + 8) != 0 { read_usize(p.addr() + 8) } else { 0x989ad8 };
            let first = if read_usize(p.addr() + 4) != 0 { read_usize(p.addr() + 4) } else { 0x989ad8 };
            let dob = p.addr() + staff::DATE_OF_BIRTH;
            let year = read_u16(dob + 2) as u32;
            let month = exe::date_get_month(dob) as i8 as i32 + 1;
            let day = exe::date_get_day_of_month(dob) as i16 as i32;
            // the month and the year are the two arguments the original left on the stack for the date calls
            staff_tab_format()(JOB_TEXT, 0x9ced08usize, first, second, day, month, year); // "%s %s %d.%d.%d"
            let f: unsafe extern "C" fn(usize) = core::mem::transmute(FN_HISTORY_SCREEN_OPEN);
            f(JOB_TEXT);
            return -4;
        }
        2 => {
            covered!("tel09_3 profile input vote box");
            let entry = value() as usize;
            let award = read_usize(read_usize(read_usize(G_AWARD_MANAGER) + 8) + (read_i32(entry + 0xd) as usize).wrapping_mul(4));
            set_screen_param(0xd, entry);
            exe::database_format_staff_name(p, nm, 200, 1);
            let club = read_u32(entry + 9);
            if club != 0 {
                exe::database_get_comp_colours((club) as u32 as usize, &mut c1, &mut c2);
            } else if read_u8(award + 0x12) == 1 {
                exe::database_get_club_kit_colours((exe::database_get_national_team((read_u32(award + 0xe)) as u32 as usize) as u32) as usize, 1, &mut c1, &mut c2);
            } else {
                exe::database_get_comp_colours(((read_usize(G_CLUB_COMP_TABLE) as u32).wrapping_add((read_i32(0x9cf79c) as u32).wrapping_mul(0x6b))) as u32 as usize, &mut c1, &mut c2);
            }
            // "Vote for <%s - player> for the <%s - award> award ?"
            let text = lang(JOB_TEXT, 0xa8b320usize, nm, read_u32(entry + 5).wrapping_add(4), 1u32, c1, c2, 0x18u32) as u32 as usize;
            let s = lang(sub, 0x9909b8usize, text) as u32 as usize; // "Please Confirm"
            let t = lang(title.as_mut_ptr() as usize, 0xa8562cusize, s) as u32 as usize; // "Vote For Award"
            profile_input_box(t, s, text, 1, c1, c2, 0x18);
            return -4;
        }
        3 => {
            let mut club_out: usize = 0;
            let club = manager.club_job().addr() as u32;
            if exe::contract_manager_toggle_squad_placement(read_usize(G_CONTRACT_MANAGER), p, (club) as u32 as usize, (&mut club_out) as *mut usize as usize) != 0 {
                return -11;
            }
            covered!("tel09_3 profile input no squad space");
            exe::database_format_staff_name(p, nm, 200, 1);
            exe::database_get_club_kit_colours(club as usize, 1, &mut c1, &mut c2);
            let g = read_i8(club_out + 0x52) as i32;
            // "There is no space in the {}<%s - Club Name(e.g.Barcelona)>{} squad ..."
            let text = lang(JOB_TEXT, 0xa8b2ccusize, g, g, club_out + 0x38, 0u32, c1, c2, 0x27u32) as u32 as usize;
            profile_input_notice(nm, sub, 0xa8b2b0, text, c1, c2, 0x27); // "Cannot move player to squad"
            return -4;
        }
        0x1d => {
            let tm = read_usize(G_TRANSFER_MANAGER);
            if tm != 0 && read_usize(tm + 0x856) != 0 {
                let shortlist: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_ADD_PLAYER_TO_CLUB_SHORTLISTS);
                shortlist(read_usize(tm + 0x856), p.addr(), 1);
            }
            return -11;
        }
        0x2c => {
            let f: unsafe extern "C" fn(usize) = core::mem::transmute(FN_NOTES_OPEN_STAFF_NOTE_SCREEN);
            f(p.addr());
            return -4;
        }
        0x2e | 0x2d => {
            let f: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_OPEN_TREATMENT_CONFIRM_SCREEN);
            f(p.addr(), (id == 0x2e) as u32);
            return -4;
        }
        0x2a => {
            let mut club = p.club_job().addr();
            if club == 0 {
                return -11;
            }
            let mut out = view as u32;
            let a = exe::club_get_linked_b_team(club, &mut out, 1);
            if a != 0 && out == 0 {
                club = a;
            }
            let person: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_ASSISTANT);
            let mut other = person(club);
            if other == p.addr() {
                other = 0;
            }
            if other == 0 {
                for k in 0..5 {
                    other = read_usize(club + 0x19f + 4 * k);
                    if other == p.addr() {
                        other = 0;
                        continue;
                    }
                    if other != 0 {
                        break;
                    }
                }
                if other == 0 {
                    return -11;
                }
            }
            let f: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_COACH_OPEN_REPORT_REQUEST_SCREEN);
            f(p.addr(), other);
            return -11;
        }
        0x2b => {
            let club = p.club_job().addr();
            if club == 0 {
                return -11;
            }
            let mut other = 0;
            for k in 0..3 {
                other = read_usize(club + 0x1cf + 4 * k);
                if other != 0 {
                    break;
                }
            }
            if other != 0 {
                let f: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_PHYSIO_REPORT_SCREEN_OPEN);
                f(p.addr(), other);
            }
            return -11;
        }
        0x23 => {
            let f: unsafe extern "C" fn(usize) = core::mem::transmute(FN_STAFF_NORMALISE_PLAYER_ATTRIBUTES);
            f(p.addr());
            return -11;
        }
        5 => {
            profile_input_action(0x8f6770, p, 0, u32::MAX, 0);
            return -4;
        }
        0x24 => {
            let f: unsafe extern "C" fn(usize) = core::mem::transmute(FN_OPEN_DISCIPLINE_PLAYER_SCREEN);
            f(p.addr());
            return -4;
        }
        6 => {
            let mut club_out: usize = 0;
            let offer: unsafe extern "thiscall" fn(usize, usize, *mut usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_AGREED_CONTRACT_CLUB);
            if offer(read_usize(G_TRANSFER_MANAGER), p.addr(), &mut club_out) != 0 {
                covered!("tel09_3 profile input cannot offer contract");
                exe::database_format_staff_name(p, nm, 200, 1);
                let g = read_i8(club_out + 0x52) as i32;
                // "<%s - Staff Name(e.g. Kevin James)> ..."
                let text = lang(JOB_TEXT, 0xa8b23cusize, g, g, nm, club_out + 0x38, 0u32, c1, c2, 0x3cu32) as u32 as usize;
                profile_input_notice(nm, sub, 0xa8b224, text, c1, c2, 0x3c); // "Cannot offer contract"
            } else {
                profile_input_action(0x4dd0f0, p, 0, 0, 0);
            }
            return -4;
        }
        7 | 8 | 0x19 | 0x1a => {
            let f: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_TRANSFER_SCREENS_OPEN_TRANSFER_LIST_DIALOG);
            f(p.addr(), (id >= 0x19) as u32);
            return -4;
        }
        9 => {
            let f: unsafe extern "C" fn(usize) = core::mem::transmute(FN_TRANSFER_SCREENS_OPEN_RELEASE_DIALOG);
            f(p.addr());
            return -4;
        }
        0xa | 0xb => {
            let f: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_TRANSFER_SCREENS_OPEN_SHORTLIST_DIALOG);
            f(p.addr(), (id == 0xb) as u32);
            return -4;
        }
        0x2f | 0x30 => {
            let f: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(FN_TRANSFER_SCREENS_OPEN_COMPARISON_PROMPT);
            f(p.addr(), 1, if id == 0x2f { 1 } else { 2 });
            return -4;
        }
        0x31 => {
            let a = exe::scout_manager_get_compare_player(read_usize(G_SCOUT_MANAGER), 1);
            let b = exe::scout_manager_get_compare_player(read_usize(G_SCOUT_MANAGER), 2);
            if a == 0 || b == 0 || a == b {
                exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x88c460usize, 0x88c660usize, 1, 0usize, 0);
                return -4;
            }
            let first = exe::scout_manager_get_compare_player(read_usize(G_SCOUT_MANAGER), 1);
            let who = if first != 0 { first } else { p.addr() };
            if who == 0 {
                tel26_error_box(0xa84aac, 0x1ac); // "E:\dev\CM3\cm3 00-01\cm3\code\staff_screens.cpp"
                return -4;
            }
            exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x876520usize, 0x889c30usize, 0, 0x88b8a0usize, 0);
            return -4;
        }
        0xc => {
            profile_input_action(0x4dd0f0, p, value(), 1, 0);
            return -4;
        }
        0xd => {
            let v = value() as usize;
            let offer = read_usize(v);
            if read_u8(offer + 0x2c) == 2 {
                let f: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_TRANSFER_SCREENS_OPEN_OFFER_CHANGED_SCREEN);
                f(offer, read_u32(offer + 0x24));
            } else {
                let f: unsafe extern "C" fn(usize) = core::mem::transmute(FN_TRANSFER_SCREENS_OPEN_WITHDRAW_BID_DIALOG);
                f(v);
            }
            return -4;
        }
        0xe => {
            let f: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_TRANSFER_SCREENS_OPEN_BID_SCREEN_FOR_PLAYER);
            f(p.addr(), value());
            return -4;
        }
        0xf => {
            let mut reason = 0u32;
            if profile_input_approach(p, manager.club_job().addr() as u32, &mut reason) == 0 {
                profile_input_action(0x8f0e40, p, manager.club_job().addr() as u32, reason, 3);
            } else {
                profile_input_action(0x4dd0f0, p, 0, 1, 0);
            }
            return -4;
        }
        0x10 => {
            let mut reason = 0u32;
            if profile_input_approach(p, manager.club_job().addr() as u32, &mut reason) == 0 {
                profile_input_action(0x8f0e40, p, manager.club_job().addr() as u32, reason, 0);
                return -4;
            }
            if !p.club_job().is_null() {
                let f: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_TRANSFER_SCREENS_OPEN_BID_SCREEN_FOR_PLAYER);
                f(p.addr(), 0);
                return -4;
            }
            covered!("tel09_3 profile input approach without club");
        }
        0x17 => {
            let mut reason = 0u32;
            if profile_input_approach(p, manager.club_job().addr() as u32, &mut reason) == 0 {
                profile_input_action(0x8f0e40, p, manager.club_job().addr() as u32, reason, 2);
            } else {
                let f: unsafe extern "C" fn(usize) = core::mem::transmute(FN_TRANSFER_SCREENS_OPEN_OFFER_TRIAL_DIALOG);
                f(p.addr());
            }
            return -4;
        }
        0x16 => {
            let club = manager.club_job().addr() as u32;
            if exe::transfer_manager_player_needs_work_permit(read_usize(G_TRANSFER_MANAGER), p, club as usize, 1) != 0 {
                profile_input_action(0x8f0e40, p, club, 0x29, 1);
                return -4;
            }
            let mut reason = 0u32;
            if profile_input_approach(p, club, &mut reason) == 0 {
                profile_input_action(0x8f0e40, p, club, reason, 1);
                return -4;
            }
            covered!("tel09_3 profile input approach dates");
            let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
            write_usize(G_CONTRACT_LOOKUP_RESULT, contract);
            let current = if contract != 0 { club_at(read_i32(contract + 4) as isize).addr() } else { 0 };
            let (mut a, mut b, mut c, mut d) = (0u32, 0u32, 0u32, 0u32);
            let dates: unsafe extern "thiscall" fn(usize, u32, usize, usize, *mut u32, *mut u32, *mut u32, *mut u32) =
                core::mem::transmute(FN_TRANSFER_MANAGER_GET_LOAN_PERIOD);
            dates(read_usize(G_TRANSFER_MANAGER), club, current, p.addr(), &mut c, &mut d, &mut a, &mut b);
            let (mut t1, mut t2) = ([0u16; 4], [0u16; 4]);
            let d1 = profile_input_make_date(&mut t1, a, b);
            let (day1, year1) = (read_i16(d1 as usize), read_u16(d1 as usize + 2));
            let d2 = profile_input_make_date(&mut t2, c, d);
            let year2 = read_u16(d2 as usize + 2);
            let earlier = if year1 < year2 { true } else if year1 > year2 { false } else { day1 < read_i16(d2 as usize) };
            if earlier {
                profile_input_action(0x8f0e40, p, club, reason, 1);
            } else {
                let f: unsafe extern "C" fn(usize) = core::mem::transmute(FN_TRANSFER_SCREENS_OPEN_LOAN_BID_SCREEN);
                f(p.addr());
            }
            return -4;
        }
        0x18 => {
            let f: unsafe extern "C" fn(usize) = core::mem::transmute(FN_TRANSFER_SCREENS_OPEN_END_LOAN_DIALOG);
            f(p.addr());
            return -4;
        }
        0x11 => {
            if value() as usize != get_screen_param(1) {
                set_screen_param(1, value() as usize);
                return -11;
            }
        }
        0x13 => {
            let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(FN_TRANSFER_SCREENS_OPEN_BID_SCREEN_FOR_OFFER);
            f(value(), 1);
            return -4;
        }
        0x1b | 0x1c => {
            let f: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(FN_NATIONAL_TEAMS_SCREENS_OPEN_SQUAD_MOVE_SCREEN);
            f(p.addr(), value(), (id == 0x1b) as u32);
            return -4;
        }
        0x25 | 0x26 => {
            let copy = exe::crt_malloc(0x1a);
            if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x88bc70usize, 0x88bee0usize, 1, 0usize, 0) == 0 {
                return -4;
            }
            set_screen_param(0, p.addr());
            let hm = read_usize(G_HUMAN_MANAGER);
            if exe::human_manager_has_nickname(hm, p, 0) != 0 {
                staff_tab_strcpy(copy, exe::human_manager_get_nickname(hm, p, 0));
            } else {
                staff_tab_strcpy(copy, p.common_name());
            }
            set_screen_param(1, copy);
            return -4;
        }
        0x29 => {
            covered!("tel09_3 profile input scouting");
            let club = manager.club_job().addr();
            exe::database_format_staff_name(p, nm, 200, 1);
            exe::database_get_club_kit_colours(club, 1, &mut c1, &mut c2);
            let answer = exe::scout_manager_get_scout_request_status(read_usize(G_SCOUT_MANAGER), p, 0) as i8;
            let notice = |text: usize, subtitle: usize| {
                let text = lang(JOB_TEXT, text, 0u32, c1, c2, 0x42u32) as u32 as usize;
                profile_input_notice(nm, sub, subtitle, text, c1, c2, 0x42);
            };
            match answer {
                0 | 1 => {
                    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x88c100usize, 0x88c360usize, 1, 0usize, 0) != 0 {
                        set_screen_param(0, read_u32(p.addr()) as usize);
                        set_screen_param(1, (answer == 0) as usize);
                    }
                }
                2 => notice(0xa8b1e4, 0xa8b1c8), // "There are currently no scouts available ..." / "Cannot have player scouted"
                3 => notice(0xa8b144, 0xa8b134), // "You are already scouting this player." / "Already scouted"
                4 => notice(0xa8b16c, 0xa8b1c8), // "Your club does not have the resources ..." / "Cannot have player scouted"
                _ => {}
            }
            exe::scout_manager_add_to_shortlist(read_usize(G_SCOUT_MANAGER), p, 0);
            return -4;
        }
        _ => {}
    }
    let r = match (view as i8 as i32) - 1 {
        0 => {
            let f: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_STAFF_SCREENS_HANDLE_PROFILE_TAB_ACTION);
            f(item)
        }
        1 | 2 => 0,
        3 => {
            let q = Staff(get_screen_param(0));
            if !q.player().is_null() && (q.is_null() || read_i32(q.addr()) < read_i32(G_STAFF_COUNT)) && item as i16 != -1 {
                let row = exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(0xc));
                set_screen_param(0xb, row);
            }
            0
        }
        4 => {
            let f: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_STAFF_SCREENS_HANDLE_HISTORY_TAB_ACTION);
            f(item)
        }
        _ => {
            tel26_error_box(0xa84aac, 0x339a); // "E:\dev\CM3\cm3 00-01\cm3\code\staff_screens.cpp"
            0
        }
    };
    if r != 0 {
        return r;
    }
    let r = exe::menubar_handle_command(item);
    if r != 0 {
        return r;
    }
    exe::menubar_handle_object_link(item)
}

/// Comparer for `staff_screens_profile_input`: side by side over the screen parameters and the answer only for the
/// clicks that just write a parameter (0x3c, 0x3d, and 0x11 with a new value) with no pending network box answer;
/// every other click opens a box, a screen or acts on the game, and only the original runs.
pub(crate) unsafe extern "C" fn compare_staff_screens_profile_input(item: u32) -> i32 {
    let pending = read_u32(0xb77c84) != 0 && matches!(read_u32(G_PENDING_COMMAND_CODE), 0x18 | 0x27 | 0x42 | 0x3c);
    let id = if item as i16 != -1 { tel26_item_id(item as i16) } else { 0 };
    let stays = !pending
        && match id {
            0x3c | 0x3d => true,
            0x11 => tel26_item_value(item as i16) as usize != get_screen_param(1),
            _ => false,
        };
    compare_resposta("staff profile input", item, original_fn(IDX_STAFF_SCREENS_PROFILE_INPUT), staff_screens_profile_input, !stays)
}

/// Rule: the staff profile and compare screens answer their action menu: view, history, votes, squad moves,
/// shortlists, notes, treatments, reports, contracts, transfer and loan offers, scouting, nicknames, the tab choices;
/// what is not an action goes to the profile tab answer and then the menu bar.
///
/// `staff_screens_handle_compare_action(item)`: the person (parameter 0), the two colours (0x11, 0x12) and the slot's
/// human. In a network game the pending command (`G_PENDING_COMMAND` 0xdd6be0, argument `G_PENDING_COMMAND_ARG`): 0x18
/// clears both and, with the argument 2, casts the human's vote for the person in the shortlist of parameter 0xd
/// (`AWARD_SHORTLIST::cast_vote`, at +0x1f) and answers -10, otherwise clears parameter 0xd; 0x27, 0x42 and 0x3c clear
/// both. Then by the widget's id (read again at each test): 0x3c / 0x3d: parameter 0x14 = 1 / 0, -11. 1: the profile of
/// the widget's person (`staff_screens_open_staff_profile(person, 1, 1, 1, 0)`). 0x1f: the terminate contract screen.
/// 0x1e: the history screen with "<first> <second> <day>.<month>.<year>" of the birth date. 2: the vote confirm box
/// for the award entry of the widget (parameter 0xd = the entry; colours of its competition, of its nation's national
/// team, or of the World Cup; "Vote for <name> for the <award> award ?", heading "Please Confirm", title "Vote For
/// Award", yes/no, code 0x18). 3: `CONTRACT_MANAGER::toggle_squad_placement(person, human club, &club)`; failing, the box
/// "There is no space in the <club> squad..." / "Cannot move player to squad" (code 0x27); else -11. 0x1d: the club
/// shortlists of `G_TRANSFER_MANAGER`'s shortlist manager (+0x856) take the person (when there is one), -11. 0x2c: the
/// staff note screen. 0x2e / 0x2d: the treatment confirm screen (leave 1 / 0). 0x2a: the coach report request with the
/// club's exe::database_get_club_assistant(the linked B team's when it is not a reserve) or the first of the five coaches other than the
/// person; -11. 0x2b: the physio report with the club's first physio; -11. 0x23: `staff_normalise_player_attributes`,
/// -11. 5: the transfer status screen. 0x24: the discipline screen. 6: with an agreed contract
/// (`TRANSFER_MANAGER::get_agreed_contract_club`), the box "<name> has already accepted a contract offer from <club>."
/// / "Cannot offer contract" (code 0x3c), else the contract offer (0, 0, 0). 7 / 8 and 0x19 / 0x1a: the transfer list
/// dialog (0 / 1). 9: the release dialog. 0xa / 0xb: the shortlist dialog (0 / 1). 0xc: the contract offer of the
/// widget's kind. 0xd: the widget's offer: status 2 the offer changed screen, else the withdraw dialog. 0xe: the bid
/// screen with the widget's offer slot. 0xf, 0x10, 0x17, 0x16: `can_club_bid_for_player` for the human's club; refused,
/// the cannot sign dialog (kind 3, 0, 2, 1); allowed: 0xf the contract offer (0, 1, 0), 0x10 the bid screen (only for a
/// person with a club, else on to the next tests), 0x17 the trial dialog, 0x16 the loan period
/// (`TRANSFER_MANAGER::get_loan_period`) and, when its start is before its end, the cannot sign dialog (kind 1), else
/// the loan bid screen. 0x18: the end loan dialog. 0x11: a widget data other than parameter 1 becomes parameter 1,
/// -11 (else on). 0x13: the bid screen of the widget's offer. 0x1b / 0x1c: the squad move screen (1 / 0). 0x25, 0x26:
/// the nickname dialog with a 26-byte buffer of the human's nickname for the person, or his common name. 0x29: the
/// scouting: by `SCOUT_MANAGER::get_scout_request_status` 0 / 1 the scout request dialog (parameter 1 = 1 / 0), 2 / 4 /
/// 3 the boxes "no scouts" / "no resources" / "already scouting" (code 0x42); then `SCOUT_MANAGER::add_to_shortlist`.
/// Otherwise `staff_screens_handle_profile_tab_action`, then the menu bar. Actions answer -4 unless said.
/// Original: `staff_screens_handle_compare_action` (0x889c30, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_handle_compare_action(raw: u32) -> i32 {
    let text = tel36_langlib();
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let person = Staff(get_screen_param(0));
    let mut colours = (get_screen_param(0x11) as u32, get_screen_param(0x12) as u32);
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    let human = read_usize((G_SCREEN_RECORDS_MANAGER as isize + slot * 0x300) as usize);
    let shortlist_vote = |human: usize| {
        exe::award_shortlist_cast_vote(read_usize(get_screen_param(0xd) + 0x1f), human, person);
    };
    if read_u32(TEL35_NETWORK_FLAG) != 0 {
        let mut command = read_u32(STAFF_PENDING_COMMAND);
        if command == 0x18 {
            let argument = read_u32(G_PENDING_COMMAND_ARG);
            write_u32(STAFF_PENDING_COMMAND, 0);
            write_u32(G_PENDING_COMMAND_ARG, 0);
            if argument == 2 {
                covered!("tel36_9 network vote");
                shortlist_vote(human);
                set_screen_param(0xd, 0);
                return -10;
            }
            set_screen_param(0xd, 0);
            command = read_u32(STAFF_PENDING_COMMAND);
        }
        if matches!(command, 0x27 | 0x42 | 0x3c) {
            write_u32(G_PENDING_COMMAND_ARG, 0);
            write_u32(STAFF_PENDING_COMMAND, 0);
        }
    }
    let item = raw as i16;
    let id = || tel26_item_id(item);
    let value = || tel26_item_value(item) as usize;
    let mut name = [0u8; 0xc8];
    let n = name.as_mut_ptr() as usize;
    match id() {
        0x3c | 0x3d => {
            set_screen_param(0x14, (id() == 0x3c) as usize);
            return -11;
        }
        1 => {
            exe::staff_screens_open_staff_profile((value()) as usize as u32, 1, 1, 1, 0);
        }
        0x1f => staff_action1(0x4e53e0, person.addr()),
        0x1e => {
            covered!("tel36_9 person history");
            let second = if read_usize(person.addr() + 8) != 0 { read_usize(person.addr() + 8) } else { 0x989ad8 };
            let first = if read_usize(person.addr() + 4) != 0 { read_usize(person.addr() + 4) } else { 0x989ad8 };
            let born = person.addr() + staff::DATE_OF_BIRTH;
            let year = person.date_of_birth_year() as u32;
            let month = exe::date_get_month(born) as i8 as i32 + 1;
            let day = exe::date_get_day_of_month(born) as i16 as i32;
            format(JOB_TEXT, 0x9ced08, first, second, day, month, year);
            staff_action1(0x5f11c0, JOB_TEXT);
        }
        2 => {
            covered!("tel36_9 award vote box");
            let entry = value();
            let award = read_usize(read_usize(read_usize(G_AWARD_MANAGER) + 8) + read_u32(entry + 0xd) as usize * 4);
            set_screen_param(0xd, entry);
            exe::database_format_staff_name(person, n, 0xc8, 1);
            let comp = read_usize(entry + 9);
            if comp != 0 {
                exe::database_get_comp_colours(comp, &mut colours.0, &mut colours.1);
            } else if read_u8(award + 0x12) == 1 {
                exe::database_get_club_kit_colours(exe::database_get_national_team(read_usize(award + 0xe)), 1, &mut colours.0, &mut colours.1);
            } else {
                let world_cup = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_u32(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(TEL36_CLUB_COMP_SIZE) as usize);
                exe::database_get_comp_colours(world_cup, &mut colours.0, &mut colours.1);
            }
            let message = text(JOB_TEXT, 0xa8b320, n, read_usize(entry + 5) + 4) as u32 as usize;
            let (mut heading, mut title) = ([0u8; 0xc8], [0u8; 0xc8]);
            let h = text(heading.as_mut_ptr() as usize, 0x9909b8) as u32 as usize;
            let t = text(title.as_mut_ptr() as usize, 0xa8562c) as u32 as usize;
            staff_confirm_push(t, h, message, 1, colours.0, colours.1, 0x18);
        }
        3 => {
            let club = read_usize(human + staff::CLUB_JOB);
            let mut full = 0usize;
            if exe::contract_manager_toggle_squad_placement(read_usize(G_CONTRACT_MANAGER), person, club, (&mut full) as *mut usize as usize) != 0 {
                return -11;
            }
            covered!("tel36_9 no space in the squad");
            exe::database_format_staff_name(person, n, 0xc8, 1);
            exe::database_get_club_kit_colours(club, 1, &mut colours.0, &mut colours.1);
            let gender = read_u8(full + club::SHORT_GENDER_NAME) as i8 as i32;
            let message = text(JOB_TEXT, 0xa8b2cc, gender, gender, full + club::SHORT_NAME) as u32 as usize;
            staff_cannot_box(n, 0xa8b2b0, message, colours, 0x27);
        }
        0x1d => {
            let tm = read_usize(G_TRANSFER_MANAGER);
            if tm != 0 && read_usize(tm + 0x856) != 0 {
                let add: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_ADD_PLAYER_TO_CLUB_SHORTLISTS);
                add(read_usize(tm + 0x856), person.addr(), 1);
            }
            return -11;
        }
        0x2c => staff_action1(0x7961b0, person.addr()),
        0x2e => staff_action2(0x7e5b10, person.addr(), 1),
        0x2d => staff_action2(0x7e5b10, person.addr(), 0),
        0x2a => {
            let mut club = person.club_job().addr();
            if club == 0 {
                return -11;
            }
            // the original's flag is the frame slot that holds the human until the call writes it
            let mut reserve = human as u32;
            let b_team = exe::club_get_linked_b_team(club, &mut reserve, 1);
            if b_team != 0 && reserve == 0 {
                club = b_team;
            }
            let mut coach = exe::database_get_club_assistant(club);
            if coach == person.addr() {
                coach = 0;
            }
            if coach == 0 {
                coach = (0..5).map(|k| read_usize(club + club::COACHES + 4 * k)).find(|&c| c != 0 && c != person.addr()).unwrap_or(0);
            }
            if coach != 0 {
                covered!("tel36_9 coach report");
                staff_action2(0x48bc00, person.addr(), coach);
            }
            return -11;
        }
        0x2b => {
            let club = person.club_job().addr();
            if club != 0 {
                if let Some(physio) = (0..3).map(|k| read_usize(club + club::PHYSIOS + 4 * k)).find(|&p| p != 0) {
                    staff_action2(0x7a2a50, person.addr(), physio);
                }
            }
            return -11;
        }
        0x23 => {
            staff_action1(FN_STAFF_NORMALISE_PLAYER_ATTRIBUTES, person.addr());
            return -11;
        }
        5 => staff_action4(0x8f6770, person.addr(), 0, usize::MAX, 0),
        0x24 => staff_action1(0x5a8380, person.addr()),
        6 => {
            let mut club = 0usize;
            if exe::transfer_manager_get_agreed_contract_club(read_usize(G_TRANSFER_MANAGER), person, (&mut club) as *mut usize as usize) != 0 {
                covered!("tel36_9 contract already agreed");
                exe::database_format_staff_name(person, n, 0xc8, 1);
                let gender = read_u8(club + club::SHORT_GENDER_NAME) as i8 as i32;
                let message = text(JOB_TEXT, 0xa8b23c, gender, gender, n, club + club::SHORT_NAME) as u32 as usize;
                staff_cannot_box(n, 0xa8b224, message, colours, 0x3c);
            } else {
                staff_action4(0x4dd0f0, person.addr(), 0, 0, 0);
            }
        }
        7 | 8 => staff_action2(0x8f8120, person.addr(), 0),
        0x19 | 0x1a => staff_action2(0x8f8120, person.addr(), 1),
        9 => staff_action1(0x8f8630, person.addr()),
        0xa => staff_action2(0x8f4160, person.addr(), 0),
        0xb => staff_action2(0x8f4160, person.addr(), 1),
        0xc => staff_action4(0x4dd0f0, person.addr(), value(), 1, 0),
        0xd => {
            let slot = value();
            let offer = Offer(read_usize(slot));
            if offer.status() == 2 {
                staff_action2(0x8f2470, offer.addr(), (offer.serial() as u32) as usize);
            } else {
                staff_action1(0x8f4c70, slot);
            }
        }
        0xe => staff_action2(0x8e9350, person.addr(), value()),
        0xf | 0x10 | 0x17 | 0x16 => {
            let action = id();
            let club = read_usize(human + staff::CLUB_JOB);
            let mut reason = 0u32;
            if staff_can_bid(person.addr(), club, &mut reason) == 0 {
                covered!("tel36_9 cannot sign");
                let kind = match action { 0xf => 3, 0x10 => 0, 0x17 => 2, _ => 1 };
                staff_cannot_sign(person.addr(), read_usize(human + staff::CLUB_JOB), reason, kind);
            } else {
                match action {
                    0xf => staff_action4(0x4dd0f0, person.addr(), 0, 1, 0),
                    0x10 => {
                        if person.club_job().addr() == 0 {
                            return staff_compare_default(raw);
                        }
                        staff_action2(0x8e9350, person.addr(), 0);
                    }
                    0x17 => staff_action1(0x8f8be0, person.addr()),
                    _ => {
                        covered!("tel36_9 loan bid");
                        let contract = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
                        write_usize(G_CONTRACT_LOOKUP_RESULT, contract.addr());
                        let seller = if !contract.is_null() {
                            read_usize(G_CLUB_TABLE).wrapping_add((contract.club() as u32).wrapping_mul(club::SIZE as u32) as usize)
                        } else {
                            0
                        };
                        let (mut a, mut b, mut c, mut d) = (0u32, 0u32, 0u32, 0u32);
                        let period: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u32, *mut u32, *mut u32, *mut u32) =
                            core::mem::transmute(FN_TRANSFER_MANAGER_GET_LOAN_PERIOD);
                        period(read_usize(G_TRANSFER_MANAGER), read_usize(human + staff::CLUB_JOB), seller, person.addr(), &mut a, &mut b, &mut c, &mut d);
                        let (mut start, mut end) = ([0u16; 4], [0u16; 4]);
                        exe::date_date_549f70(&mut start, 1, c, d, u32::MAX);
                        exe::date_date_549f70(&mut end, 1, a, b, u32::MAX);
                        let earlier = if start[1] != end[1] { start[1] < end[1] } else { (start[0] as i16) < (end[0] as i16) };
                        if earlier {
                            staff_cannot_sign(person.addr(), read_usize(human + staff::CLUB_JOB), reason, 1);
                        } else {
                            staff_action1(0x8f9690, person.addr());
                        }
                    }
                }
            }
        }
        0x18 => staff_action1(0x8fbcc0, person.addr()),
        0x11 if value() != get_screen_param(1) => {
            set_screen_param(1, value());
            return -11;
        }
        0x13 => staff_action2(0x8e9550, value(), 1),
        0x1b => staff_action3(0x778f60, person.addr(), value(), 1),
        0x1c => staff_action3(0x778f60, person.addr(), value(), 0),
        0x25 | 0x26 => {
            covered!("tel36_9 nickname dialog");
            let buffer = exe::crt_malloc(0x1a);
            if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x88bc70usize, 0x88bee0usize, 1, 0usize, 0) != 0 {
                set_screen_param(0, person.addr());
                let manager = read_usize(G_HUMAN_MANAGER);
                let source = if exe::human_manager_has_nickname(manager, person, 0) != 0 {
                    exe::human_manager_get_nickname(manager, person, 0)
                } else {
                    person.common_name()
                };
                tel26_copy_text(buffer as *mut u8, source as *const u8);
                set_screen_param(1, buffer);
            }
        }
        0x29 => {
            covered!("tel36_9 scouting");
            let club = read_usize(human + staff::CLUB_JOB);
            exe::database_format_staff_name(person, n, 0xc8, 1);
            exe::database_get_club_kit_colours(club, 1, &mut colours.0, &mut colours.1);
            match exe::scout_manager_get_scout_request_status(read_usize(G_SCOUT_MANAGER), person, 0) as i8 {
                s @ (0 | 1) => {
                    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x88c100usize, 0x88c360usize, 1, 0usize, 0) != 0 {
                        set_screen_param(0, read_u32(person.addr()) as usize);
                        set_screen_param(1, (s == 0) as usize);
                    }
                }
                s @ (2 | 3 | 4) => {
                    let (question, heading) = match s { 2 => (0xa8b1e4, 0xa8b1c8), 4 => (0xa8b16c, 0xa8b1c8), _ => (0xa8b144, 0xa8b134) };
                    let message = text(JOB_TEXT, question) as u32 as usize;
                    staff_cannot_box(n, heading, message, colours, 0x42);
                }
                _ => {}
            }
            let add: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SCOUT_MANAGER_ADD_TO_SHORTLIST);
            add(read_usize(G_SCOUT_MANAGER), person.addr(), 0);
        }
        _ => return staff_compare_default(raw),
    }
    -4
}

/// Rule: on the profile tab the attribute view (id 0x12) switches to the widget's view, and "-" / "+" (0x27 / 0x28)
/// move the zoom (parameter 0x13) between 0 and 3.
///
/// `staff_screens_handle_profile_tab_action(item)`: 0x12 with a widget data other than parameter 4: parameter 4 = the
/// data, -11. 0x27 with parameter 0x13 (a char) above 0: one less, -11. 0x28 with parameter 0x13 + 1 below 4: that,
/// -11. Otherwise 0.
/// Original: `staff_screens_handle_profile_tab_action` (0x88b2f0, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_handle_profile_tab_action(raw: u32) -> i32 {
    let item = raw as i16;
    if tel26_item_id(item) == 0x12 && tel26_item_value(item) as usize != get_screen_param(4) {
        covered!("tel36_9 profile view");
        set_screen_param(4, tel26_item_value(item) as usize);
        return -11;
    }
    if tel26_item_id(item) == 0x27 {
        let zoom = get_screen_param(0x13) as u8 as i8;
        if zoom > 0 {
            set_screen_param(0x13, (zoom as i32 - 1) as usize);
            return -11;
        }
    }
    if tel26_item_id(item) == 0x28 {
        let zoom = get_screen_param(0x13) as u8 as i8 as i32 + 1;
        if zoom < 4 {
            set_screen_param(0x13, zoom as usize);
            return -11;
        }
    }
    0
}

/// Rule: on the history tab the list keeps its scroll, the filters (0x14) toggle the low five bits of the history
/// mode, the season chooser (0x20, 0x21, 0x22) moves the page, the 0x3e / 0x3f switch the detail and 0x15 picks the row.
///
/// `staff_screens_handle_history_tab_action(item)`: the person (parameter 0), the mode (5), the season (0xe), the page
/// (0xf) and the last page (0x10). For no person or one of the regular staff (id below `G_STAFF_COUNT`), with a widget:
/// parameter 9 = the scroll row of the area in parameter 0xa. By the widget's id: 0x14 with a data sharing no bit with
/// the mode: data 1, 2, 4, 8 or 16 replaces the low five bits of the mode (parameter 5); any other value only answers;
/// -11. 0x20: a season other than parameter 0xe goes there; the page goes back to 1; -11. 0x3e / 0x3f: parameter 0x15 =
/// 1 / 0, -11. 0x21: the page − 1 (at least 1), -11. 0x22: the page + 1 (at most the last page), -11. 0x15: parameter
/// 8 = the data, -11. Otherwise 0.
/// Original: `staff_screens_handle_history_tab_action` (0x88b4b0, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_handle_history_tab_action(raw: u32) -> i32 {
    let person = get_screen_param(0);
    let mode = get_screen_param(5) as u32;
    let season = get_screen_param(0xe);
    let page = get_screen_param(0xf) as u16 as i16 as i32;
    let last = get_screen_param(0x10) as u16 as i16 as i32;
    let item = raw as i16;
    if (person == 0 || read_i32(person) < read_i32(G_STAFF_COUNT)) && item != -1 {
        let row = exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(0xa));
        set_screen_param(9, row);
    }
    let id = || tel26_item_id(item);
    let value = || tel26_item_value(item);
    if id() == 0x14 && mode & value() == 0 {
        covered!("tel36_9 history filter");
        let v = value();
        if matches!(v, 1 | 2 | 4 | 8 | 0x10) {
            set_screen_param(5, ((mode & !0x1f) | v) as usize);
        }
        return -11;
    }
    match id() {
        0x20 => {
            if season != value() as usize {
                set_screen_param(0xe, value() as usize);
            }
            set_screen_param(0xf, 1);
            -11
        }
        0x3e | 0x3f => {
            set_screen_param(0x15, (id() == 0x3e) as usize);
            -11
        }
        0x21 => {
            set_screen_param(0xf, (page - 1).max(1) as usize);
            -11
        }
        0x22 => {
            set_screen_param(0xf, (page + 1).min(last) as isize as usize);
            -11
        }
        0x15 => {
            set_screen_param(8, value() as usize);
            -11
        }
        _ => 0,
    }
}

/// Rule: leaving the staff profile frees its lists.
///
/// `staff_screens_leave_profile_screen()`: the list of parameter 3 (an LLIST, destroyed and deleted) and the buffer of
/// parameter 6 (`free`), each cleared when present.
/// Original: `staff_screens_leave_profile_screen` (0x88b8a0, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_leave_profile_screen() {
    let list = get_screen_param(3);
    let buffer = get_screen_param(6);
    if list != 0 {
        covered!("tel36_9 leave the profile frees the list");
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
        set_screen_param(3, 0);
    }
    if buffer != 0 {
        exe::crt_free(buffer);
        set_screen_param(6, 0);
    }
}

/// Rule: the compare players dialog opens from the menu.
///
/// `staff_screens_open_compare_players_dialog()`: `open_screen` with the builder 0x88c460, the handler 0x88c660,
/// single instance, no leave routine; returns what `open_screen` answers.
/// Original: `staff_screens_open_compare_players_dialog` (0x88c440, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_open_compare_players_dialog() -> u32 {
    covered!("tel36_9 open the compare dialog");
    exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x88c460usize, 0x88c660usize, 1, 0usize, 0)
}

/// Rule: the compare players dialog says two players must be chosen first (when they are not), under the human's name
/// in his club's colours (or "Can not compare players"), with an Okay button.
///
/// `staff_screens_build_compare_players_dialog()`: the slot's human; his club (`staff::CLUB_JOB`, else
/// `staff::NATIONAL_JOB`) gives the background picture and, when there is one, the kit 1 colours (else the defaults
/// `JOB_COLOUR_BDF0` / `JOB_COLOUR_3184`). The title: the human's name (2000 bytes into the screens' text buffer) when
/// there is a human and the language is not 0, else "Can not compare players". The two compare players
/// (`SCOUT_MANAGER::get_compare_player` 1 and 2) have their names formatted (200 bytes, unused); without both, the
/// text "Two players must be selected to compare against each other" (otherwise the buffer keeps what was there). The
/// text area (0x6e, 0x8c)-(0x30c, 0x221) (font 3, pale grey), the menu bar (4, 0) and a one-column button area with
/// "Okay" (id 0x38).
/// Original: `staff_screens_build_compare_players_dialog` (0x88c460, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_build_compare_players_dialog() {
    let text = tel36_langlib();
    let weights: [u8; 2] = [1, 0];
    let mut fore = read_u16(JOB_COLOUR_BDF0) as u32;
    let mut back = read_u16(JOB_COLOUR_3184) as u32;
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    let human = Staff(read_usize((G_SCREEN_RECORDS_MANAGER as isize + slot * 0x300) as usize));
    let mut club = human.club_job().addr();
    if club == 0 {
        club = human.national_job().addr();
    }
    covered!("tel36_9 build the compare dialog");
    exe::background_set_club_picture(club, 0, 0);
    if club != 0 {
        exe::database_get_club_kit_colours(club, 1, &mut fore, &mut back);
    }
    if !human.is_null() && exe::langlib_get_language() != 0 {
        exe::database_format_staff_name(human, JOB_TEXT, 0x7d0, 1);
        exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, JOB_TEXT);
    } else {
        exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, 0xa8b44c);
    }
    let first = exe::scout_manager_get_compare_player(read_usize(G_SCOUT_MANAGER), 1);
    let second = exe::scout_manager_get_compare_player(read_usize(G_SCOUT_MANAGER), 2);
    let (mut a, mut b) = ([0u8; 0xc8], [0u8; 0xc8]);
    if first != 0 {
        exe::database_format_staff_name(first, a.as_mut_ptr() as usize, 0xc8, 1);
    }
    if second != 0 {
        exe::database_format_staff_name(second, b.as_mut_ptr() as usize, 0xc8, 1);
    }
    if first == 0 || second == 0 {
        text(JOB_TEXT, 0xa8b410);
    }
    let rows = (0x195 / exe::font_get_height(3)) as u8 as i8 as i32 as u32;
    tel26_text_area([0x6e, 0x8c, 0x30c, 0x221, 2, 0, rows, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0]);
    exe::menubar_build(4, 0);
    let area = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 1, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    text(JOB_TEXT, 0xa8b408);
    tel26_area_button([area, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0x38, 0]);
}

/// Rule: Okay (id 0x38) closes the compare dialog; the rest goes to the menu bar.
/// Original: `staff_screens_handle_compare_players_action` (0x88c660, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_handle_compare_players_action(raw: u32) -> i32 {
    if tel26_item_id(raw as i16) == 0x38 {
        return -9;
    }
    manager_menu_bar(raw)
}

/// Comparer for `staff_screens_handle_compare_action`: the tab choices (0x3c, 0x3d, 0x11) side by side over the screen
/// params; the actions (screens, boxes, transfers, notes) only the original.
pub(crate) unsafe extern "C" fn compare_staff_screens_handle_compare_action(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_STAFF_SCREENS_HANDLE_COMPARE_ACTION);
    if read_u32(TEL35_NETWORK_FLAG) != 0 || !matches!(tel26_item_id(item as i16), 0x3c | 0x3d) {
        // simplification: opening screens, the boxes and the game actions do not run twice; proven in Rust mode
        return o(item);
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&screen_param_regions(), || o(item), || staff_screens_handle_compare_action(item));
    SCOREBOARD.record(same && r == n, || format!("staff profile action: original={} rust={} differences {:?}", r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `staff_screens_handle_profile_tab_action`: side by side over the screen params.
pub(crate) unsafe extern "C" fn compare_staff_screens_handle_profile_tab_action(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_STAFF_SCREENS_HANDLE_PROFILE_TAB_ACTION);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&screen_param_regions(), || o(item), || staff_screens_handle_profile_tab_action(item));
    SCOREBOARD.record(same && r == n, || format!("staff profile table action: original={} rust={} differences {:?}", r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `staff_screens_handle_history_tab_action`: side by side over the screen params.
pub(crate) unsafe extern "C" fn compare_staff_screens_handle_history_tab_action(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_STAFF_SCREENS_HANDLE_HISTORY_TAB_ACTION);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&screen_param_regions(), || o(item), || staff_screens_handle_history_tab_action(item));
    SCOREBOARD.record(same && r == n, || format!("staff history table action: original={} rust={} differences {:?}", r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `staff_screens_leave_profile_screen`.
pub(crate) unsafe extern "C" fn compare_staff_screens_leave_profile_screen() {
    // simplification: freeing the lists does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(IDX_STAFF_SCREENS_LEAVE_PROFILE_SCREEN)()
}

/// Comparer for `staff_screens_open_compare_players_dialog`.
pub(crate) unsafe extern "C" fn compare_staff_screens_open_compare_players_dialog() -> u32 {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn() -> u32>(IDX_STAFF_SCREENS_OPEN_COMPARE_PLAYERS_DIALOG)()
}

/// Comparer for `staff_screens_build_compare_players_dialog`: the dialog side by side over the display list.
pub(crate) unsafe extern "C" fn compare_staff_screens_build_compare_players_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_STAFF_SCREENS_BUILD_COMPARE_PLAYERS_DIALOG);
    compare_by_display_list_with("compare players dialog (staff_screens_build_compare_players_dialog)", &[], || o(), || staff_screens_build_compare_players_dialog());
}

/// Comparer for `staff_screens_handle_compare_players_action`.
pub(crate) unsafe extern "C" fn compare_staff_screens_handle_compare_players_action(item: u32) -> i32 {
    // simplification: the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_STAFF_SCREENS_HANDLE_COMPARE_PLAYERS_ACTION)(item)
}

/// `HUMAN_MANAGER` nickname call (person, nickname, 0) on [G_HUMAN_MANAGER].
pub(crate) unsafe fn nickname_call(f: usize, person: usize, nickname: usize) {
    let g: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(f);
    g(read_usize(G_HUMAN_MANAGER), person, nickname, 0);
}

/// The params the staff profile screen of `staff` should get (read before it opens: with `keep_view` the current
/// screen's params come back), or `None` when the person is not viewable. Written apart from the routine, from the
/// same disassembly, to check the original afterwards.
pub(crate) unsafe fn staff_profile_planned_params(staff: usize, tab: u32, slot2: u32, keep_view: u32, attribute_mode: u32) -> Option<[usize; 0x16]> {
    let staff = Staff(staff);
    let job = staff.job_for_club();
    let n = read_i32(G_STAFF_COUNT);
    let id = read_i32(staff.addr());
    let last = id >= n.wrapping_sub(16) && id < n;
    let player = staff.player().addr() != 0;
    if !((job != 2 && !last) || (player && id < n) || (job != 1 && job != 2 && id < n)) {
        return None;
    }
    let mut p = [0usize; 0x16];
    p[0] = staff.addr();
    p[1] = if last { 5 } else { tab as u8 as i8 as isize as usize };
    p[2] = slot2 as u8 as i8 as isize as usize;
    let mode = attribute_mode as u8;
    if id < n && staff.non_player().addr() != 0 && (!player || !matches!(job, 0xb | 0xc | 0xd | 0xf) || mode == 1) {
        p[4] = 1;
        p[5] = 1;
    } else {
        p[4] = mode as i8 as isize as usize;
        p[5] = 2;
    }
    p[0xf] = 1;
    p[0x10] = 1;
    p[0x11] = colour_word(G_GUI_COLOUR_BLUE);
    p[0x12] = colour_word(G_GUI_COLOUR_PALE_GREY);
    p[0x14] = 1;
    p[0x15] = 1;
    if keep_view != 0 {
        for k in [1u32, 2, 5, 0x13] {
            p[k as usize] = get_screen_param(k);
        }
        if get_screen_param(4) == 2 {
            p[4] = 2;
        }
    }
    Some(p)
}

/// Whether `TRAINING_MANAGER::update_player_training(person, 0)` (0x8b1750) would train `person` now. It returns
/// without writing anything but the contract lookup global when the person's id is past `G_STAFF_COUNT`, or when his
/// training entry (`PLAYER_ENTRIES` + sheet id * 6) already holds the current `WEEK_COUNTER` at +1 (0x8b1bce..0x8b1bea)
/// and its bit 0x20 at +5 ("at a human club") needs no setting: set, or clear with no contract club whose manager is
/// one of the 16 last staff records (0x8b188d..0x8b1948). Otherwise it rolls the week's training (`utils_random`
/// 0x90cfc0, injuries, the position training items) and stamps the week, so a second call is a no-op but a call
/// after the comparers restore the entry trains again. A null person or one without a player sheet: false (the
/// screens do not call it for them).
pub(crate) unsafe fn training_update_pending(person: usize) -> bool {
    let person = Staff(person);
    let tm = read_usize(G_TRAINING_MANAGER);
    if person.is_null() || tm == 0 || read_i32(person.addr()) >= read_i32(G_STAFF_COUNT) {
        return false;
    }
    let sheet = person.player().addr();
    if sheet == 0 {
        return false;
    }
    let entry = read_usize(tm + training_manager_fields::PLAYER_ENTRIES) + read_i32(sheet) as usize * 6;
    if read_u8(entry + 5) & 0x20 == 0 {
        let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
        if c != 0 {
            let manager = read_usize(club_at(read_i32(c + 4) as isize).addr() + club::MANAGER);
            if manager != 0 && read_i32(manager) >= read_i32(G_STAFF_COUNT) - 0x10 {
                return true;
            }
        }
    }
    read_u16(tm + training_manager_fields::WEEK_COUNTER) != read_u16(entry + 1)
}

/// Byte `o` of the 79-byte staff state record of `s` (`G_STAFF_STATE_TABLE`).
pub(crate) unsafe fn profile_state(s: usize) -> usize {
    read_usize(G_STAFF_STATE_TABLE).wrapping_add((read_i32(s) as usize).wrapping_mul(staff_state::SIZE))
}

/// The params, the creations and the person's name: reads params 0..8 and 0x14 and the two compare players
/// (`SCOUT_MANAGER::get_compare_player` 1 and 2); with no offer list (param 3) creates one (`LLIST`, `operator new`
/// 0xe), stores it in param 3 and fills it (`TRANSFER_MANAGER::get_offers_for_player`); with no player sheet, param 5
/// loses bits 1..3 (1 when nothing is left) and a reserve flag (`staff::PLAYING_SQUAD` bit 2) becomes 1
/// (`staff_set_squad_flags`); with no manager record (`Staff_Records::get_manager_record`), param 5 loses bit 4 (1 when
/// nothing is left); on tab 5 with bit 2 of param 5, no career history (param 6) and a player sheet, builds it
/// (`PlayerStats::build_career_history`) into params 6 and 7. The background photo (`background_set_staff_picture`),
/// the viewer, his club and national team; the contract (`CONTRACT_MANAGER::get_contract`), and for someone without
/// one who has a non-player sheet, is in the database, has a club and a contract job (`is_contract_staff_job`), a new
/// `STAFF_CONTRACT` (`operator new` 0x50, the person's id, `generate_initial_contract`, type 1 keeping bits 0, 6 and 7)
/// signed with the contract manager (`sign_contract`). The name (`database_format_staff_name`); view 2 without a
/// national job becomes view 1.
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
pub(crate) unsafe fn profile_screen_setup(st: &mut ProfileScreen) {
    write_u32(PROFILE_SCREEN_RESET, 0);
    st.staff = get_screen_param(0);
    st.tab = get_screen_param(1) as u8 as i8;
    st.view = get_screen_param(2) as u8;
    st.list = get_screen_param(3);
    st.mode = get_screen_param(4) as u8 as i8 as i32;
    st.flags = get_screen_param(5) as u32;
    st.history = get_screen_param(6);
    st.history_index = get_screen_param(7) as i32;
    st.history_row = get_screen_param(8);
    st.compare_columns = get_screen_param(0x14);
    st.compare1 = exe::scout_manager_get_compare_player(read_usize(G_SCOUT_MANAGER), 1);
    st.compare2 = exe::scout_manager_get_compare_player(read_usize(G_SCOUT_MANAGER), 2);
    if st.list == 0 {
        covered!("tel07_3 profile creates the offer list");
        let p = exe::crt_operator_new(0xe);
        st.list = if p != 0 { exe::new_llist(p) } else { 0 };
        set_screen_param(3, st.list);
        exe::transfer_manager_get_offers_for_player(read_usize(G_TRANSFER_MANAGER), st.staff, st.list, 0);
    }
    let s = Staff(st.staff);
    if s.player().addr() == 0 {
        st.flags &= 0xfffffff1;
        if st.flags == 0 {
            st.flags = 1;
        }
        set_screen_param(5, st.flags as usize);
        if s.playing_squad() & 2 != 0 {
            exe::staff_set_squad_flags(s, 1);
        }
    }
    if exe::staff_records_get_manager_record(read_usize(G_STAFF_RECORDS), s, 0) == 0 {
        st.flags &= 0xffffffef;
        if st.flags == 0 {
            st.flags = 1;
        }
        set_screen_param(5, st.flags as usize);
    }
    if st.tab == 5 && st.flags & 2 != 0 && st.history == 0 && s.player().addr() != 0 {
        covered!("tel07_3 profile builds the career history");
        st.history = exe::playerstats_build_career_history(read_usize(G_PLAYER_STATS), s, &mut st.history_index);
        set_screen_param(6, st.history);
        set_screen_param(7, st.history_index as usize);
    }
    exe::background_set_staff_picture(s, 0, 0);
    st.viewer = read_usize(G_SCREEN_RECORDS_MANAGER + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300);
    if st.viewer != 0 {
        st.viewer_club = read_usize(st.viewer + staff::CLUB_JOB);
        st.viewer_nation_team = read_usize(st.viewer + staff::NATIONAL_JOB);
    }
    st.contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), s);
    if st.contract == 0
        && s.non_player().addr() != 0
        && (s.is_null() || read_i32(s.addr()) < read_i32(G_STAFF_COUNT))
        && s.club_job().addr() != 0
        && exe::is_contract_staff_job(s.job_for_club() as u32) != 0
    {
        covered!("tel07_3 profile creates a staff contract");
        let p = exe::crt_operator_new(0x50);
        st.contract = if p != 0 { exe::new_staff_contract(p) } else { 0 };
        let c = Contract(st.contract);
        if !c.is_null() {
            write_u32(c.addr(), read_u32(s.addr()));
            exe::staff_contract_generate_initial_contract(c, s, 0);
            c.set_contract_type((c.contract_type() & 0xc1) | 1);
            // simplification: the original pushes the job byte in a dword whose high bytes are register garbage
            // (the callee reads the byte); here zero-extended
            exe::contract_manager_sign_contract(read_usize(G_CONTRACT_MANAGER), c, s.job_for_club() as u32);
        }
    }
    exe::database_format_staff_name(s, st.name.as_mut_ptr() as usize, 0xc8, 1);
    if st.view == 2 && s.national_job().addr() == 0 {
        st.view = 1;
    }
    st.y = 0x50;
    st.bottom = 0x221;
}

/// The heading of the profile, by the view (param 2): view 1 with a club, the club's kit and "<n>. <name> (<club>)"
/// with the squad number when the club uses squad numbers and the person has one, " Reserves" for a reserve
/// (`staff::PLAYING_SQUAD` bit 2) of a human club, the club linked (0x7d5); a national manager (national job 5, club
/// job not 5), "<name> (<nation>)" in the national team's kit; view 2 with a national job, the national team's kit and
/// "<name> (<club>)" (no club: the national team's name, with its address as the grammar args, as in the original);
/// otherwise the screen title (`ScreenManager::add_screen_title`) in his nation's kit with "<name> (Retired)" for the
/// retired; no nation: false (the caller shows the error box, line 0x308, and builds nothing more). The colours go to
/// params 0x11 and 0x12.
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
pub(crate) unsafe fn profile_screen_heading(st: &mut ProfileScreen) -> bool {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let s = Staff(st.staff);
    let name = st.name.as_ptr() as usize;
    let text = st.text.as_mut_ptr() as usize;
    let club = s.club_job();
    let nation_team = s.national_job();
    if st.view == 1 && !club.is_null() {
        covered!("tel07_3 profile heading of a club");
        exe::database_get_club_kit_colours(club, 1, &mut st.fg, &mut st.bg);
        let squads = read_usize(G_SQUAD_MANAGER);
        let g = (club.short_gender_name() as i8) as i32;
        let reserves = || s.playing_squad() & 2 != 0 && exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0;
        if exe::squad_manager_club_uses_squad_numbers(squads, club, 0) != 0 && exe::squad_manager_get_squad_number(squads, s, club) != 0 {
            let fmt = if reserves() { 0xa856d8 } else { 0xa856a0 }; // "<n>. <player> ({}<club>{}[ Reserves])"
            let n = (exe::squad_manager_get_squad_number(squads, s, club) as i32).abs();
            lang(PROFILE_BUF, fmt, g, g, n, name, club.addr() + club::SHORT_NAME);
        } else {
            let fmt = if reserves() { 0xa85674 } else { 0xa85654 }; // "<player> ({}<club>{}[ Reserves])"
            lang(PROFILE_BUF, fmt, g, g, name, club.addr() + club::SHORT_NAME);
        }
        exe::text_set_string(text, PROFILE_BUF as *const u8);
        exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
        profile_screen_title_box(st, read_i32(0x870cce));
    } else if !nation_team.is_null() && s.job_for_nation() == 5 && s.job_for_club() != 5 {
        covered!("tel07_3 profile heading of a national manager");
        exe::database_get_club_kit_colours(nation_team, 1, &mut st.fg, &mut st.bg);
        let g = (nation_team.short_gender_name() as i8) as i32;
        lang(PROFILE_BUF, 0xa85654, g, g, name, nation_team.addr() + club::SHORT_NAME);
        exe::text_set_string(text, PROFILE_BUF as *const u8);
        exe::text_add_link(text, (nation_team.addr() + club::SHORT_NAME) as *const u8, 0x7d5, nation_team);
        profile_screen_title_box(st, read_i32(0x870cce));
    } else if st.view == 2 && !nation_team.is_null() {
        covered!("tel07_3 profile heading of the national view");
        exe::database_get_club_kit_colours(nation_team, 1, &mut st.fg, &mut st.bg);
        if !club.is_null() {
            let g = (club.short_gender_name() as i8) as i32;
            lang(PROFILE_BUF, 0xa85654, g, g, name, club.addr() + club::SHORT_NAME);
            exe::text_set_string(text, PROFILE_BUF as *const u8);
            exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
        } else {
            // DEFECT (original): with no club the grammar args are the national team name's address
            let n = nation_team.addr() + club::SHORT_NAME;
            lang(PROFILE_BUF, 0xa85654, n, n, name, n);
            exe::text_set_string(text, PROFILE_BUF as *const u8);
            exe::text_add_link(text, n as *const u8, 0x7d5, nation_team);
        }
        profile_screen_title_box(st, read_i32(0x870f57));
    } else {
        let nation = s.nation().addr();
        if nation == 0 {
            covered!("tel07_3 profile without a nation");
            return false;
        }
        covered!("tel07_3 profile screen title");
        exe::database_get_club_kit_colours(exe::database_get_national_team(nation), 1, &mut st.fg, &mut st.bg);
        if exe::player_regen_get_retirement_status(read_usize(G_PLAYER_REGEN), s) == 2 {
            lang(PROFILE_BUF, 0xa8563c, name); // "<player> (Retired)"
        } else {
            profile_strcpy(PROFILE_BUF, name);
        }
        exe::screenmanager_add_screen_title(SCREEN_MANAGER, st.bg, st.fg, PROFILE_BUF);
    }
    set_screen_param(0x11, (st.fg & 0xffff) as usize);
    set_screen_param(0x12, (st.bg & 0xffff) as usize);
    true
}

/// The Action menu of the profile: with a pending award vote of the viewer for this person whose shortlist lets him
/// vote (`AWARD_MANAGER::get_pending_vote`, `AWARD_SHORTLIST::get_vote_status` −1), only the "Vote For Award" button
/// (code 2, the shortlist as data); for a person outside the database, nothing; otherwise the menu entries
/// (`profile_screen_contract_actions`, `profile_screen_approach_to_sign`, `profile_screen_national_actions`,
/// `profile_screen_staff_actions`; with no viewer straight to "Set Nickname"). Returns the menu area (−1 when none).
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
pub(crate) unsafe fn profile_screen_action_menu(st: &mut ProfileScreen) -> usize {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let s = st.staff;
    let mut menu = usize::MAX;
    if st.viewer != 0 {
        let e = exe::award_manager_get_pending_vote(read_usize(G_AWARD_MANAGER), st.viewer, s);
        if e != 0 {
            let am = read_usize(G_AWARD_MANAGER);
            let t = read_usize(read_usize(am + 8).wrapping_add((read_i32(e + 4) as usize).wrapping_mul(4)));
            let shortlist = read_usize(read_usize(t + 4).wrapping_add((read_i32(e + 8) as usize).wrapping_mul(4)));
            if exe::award_shortlist_get_vote_status(read_usize(shortlist + 0x1f), st.viewer) == 0xffff {
                covered!("tel07_3 profile vote for award");
                lang(PROFILE_BUF, 0xa8562c); // "Vote For Award"
                profile_button(0x294, 4, 0x311, 0x18, 0x30, st.fg as usize, 0xc, 1, st.bg as usize, PROFILE_BUF, 2, shortlist);
                return menu;
            }
        }
    }
    if s != 0 && read_i32(s) >= read_i32(G_STAFF_COUNT) {
        return menu;
    }
    covered!("tel07_3 profile action menu");
    let can_sign = profile_screen_contract_actions(st, &mut menu);
    if st.viewer == 0 {
        profile_screen_staff_actions(st, &mut menu, false);
        return menu;
    }
    profile_screen_approach_to_sign(st, &mut menu, can_sign);
    profile_screen_national_actions(st, &mut menu);
    profile_screen_staff_actions(st, &mut menu, true);
    menu
}

/// The "Compare Players" entry of the menu (code 0, under the Action menu, −1 when there is none) for a player of
/// the database who is not retired, with its sub-menu: "Set as 1st player in comparison" (0x2f, with the current one's
/// name), "Set as 2nd player in comparison" (0x30) and "Compare two chosen players" (0x31); when the sub-menu area is
/// −1 the entries open their own Action button.
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
pub(crate) unsafe fn profile_screen_compare_entries(st: &mut ProfileScreen, menu: usize) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let s = Staff(st.staff);
    if s.player().addr() == 0 || read_i32(s.addr()) >= read_i32(G_STAFF_COUNT) || read_u8(profile_state(s.addr()) + staff_state::STATUS_FLAGS) & 0x30 == 0x20 {
        return;
    }
    covered!("tel07_3 profile compare entries");
    lang(PROFILE_BUF, 0xa85160); // "Compare Players"
    let item = profile_item(2, 0x10, colour_word(0xae31aa), colour_word(G_GUI_COLOUR_YELLOW), colour_word(G_GUI_COLOUR_BLACK), PROFILE_BUF, 0, 0, menu);
    let mut sub = profile_area(0, 0, 0, 0, 0, 0, 0, 0, 0x30, colour_word(0xae31aa), item);
    let (fg, bg, act) = (st.fg, st.bg, st.action.as_mut_ptr() as usize);
    let name = st.name.as_mut_ptr() as usize;
    for (k, (p, with, without)) in [(st.compare1, 0xa85114, 0xa850f4), (st.compare2, 0xa850a8, 0xa85088)].into_iter().enumerate() {
        if p != 0 {
            exe::database_format_staff_name(p, name, 0xc8, 1);
            lang(PROFILE_BUF, with, name);
        } else {
            lang(PROFILE_BUF, without);
        }
        profile_add_action(&mut sub, fg, bg, act, 0x2f + k, 0);
    }
    lang(PROFILE_BUF, 0xa5a5e8); // "Compare two chosen players"
    profile_add_action(&mut sub, fg, bg, act, 0x31, 0);
}

/// The menu bar and the tab bar of the profile (`gui_add_tab_bar` with the 25 MenuItems, moving the top and the
/// bottom of the screen's body): Profile (1; greyed and dropped for a manager or one of the 16 last records),
/// Injuries & Bans (2; greyed without a player sheet or outside the database), Contract (3; greyed for jobs 1 and 2,
/// for a national manager unless a club player, and for the 16 last records), Transfer (4; greyed for jobs 1 and 2
/// and the 16 last records) and History (5; greyed for jobs 1 and 2 and outside the database).
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
pub(crate) unsafe fn profile_screen_tabs(st: &mut ProfileScreen, items: usize) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let s = Staff(st.staff);
    exe::menubar_build(4, 0);
    let grey = read_u16(G_GUI_COLOUR_GREY);
    let item = |k: usize| items + k * PROFILE_TAB_ITEM;
    let n = read_i32(G_STAFF_COUNT);
    let id = read_i32(s.addr());
    lang(item(0) + 4, 0xa85080); // "Profile"
    write_u32(item(0) + 0xcc, 0x11);
    let job = s.job_for_club();
    if job == 2 || (id >= n.wrapping_sub(0x10) && (s.is_null() || id < n)) {
        write_u16(item(0) + 2, grey);
        write_u32(item(0) + 0xcc, 0);
    }
    let mut k = 0;
    if job != 2 && (id < n.wrapping_sub(0x10) || (!s.is_null() && id >= n)) {
        k = 1;
        write_u32(item(0) + 0xd0, 1);
    }
    let grey_out = |k: usize| {
        write_u16(item(k) + 2, grey);
        write_u32(item(k) + 0xcc, 0);
    };
    lang(item(k) + 4, 0xa85070); // "Injuries & Bans"
    write_u32(item(k) + 0xcc, 0x11);
    if s.player().addr() == 0 || (!s.is_null() && read_i32(s.addr()) >= read_i32(G_STAFF_COUNT)) {
        grey_out(k);
    }
    write_u32(item(k) + 0xd0, 2); // tel07_3 the Injuries & Bans tab
    k += 1;
    lang(item(k) + 4, 0x98e6d0); // "Contract"
    write_u32(item(k) + 0xcc, 0x11);
    let job = s.job_for_club();
    let national_only = s.job_for_nation() == 5 && (s.club_job().addr() == 0 || job != 0xb);
    if job == 1 || national_only || job == 2 || read_i32(s.addr()) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10) {
        grey_out(k);
    }
    write_u32(item(k) + 0xd0, 3);
    k += 1;
    lang(item(k) + 4, 0xa85064); // "Transfer"
    write_u32(item(k) + 0xcc, 0x11);
    let job = s.job_for_club();
    if job == 1 || job == 2 || read_i32(s.addr()) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10) {
        grey_out(k);
    }
    write_u32(item(k) + 0xd0, 4);
    k += 1;
    lang(item(k) + 4, 0xa85034, 0xad9c64usize); // "History"
    write_u32(item(k) + 0xcc, 0x11);
    let job = s.job_for_club();
    if job == 1 || job == 2 || (!s.is_null() && read_i32(s.addr()) >= read_i32(G_STAFF_COUNT)) {
        grey_out(k);
    }
    write_u32(item(k) + 0xd0, 5);
    k += 1;
    exe::gui_add_tab_bar(items, k as u32, st.tab as i32, &mut st.y, &mut st.bottom, 1);
}

/// The identity block of the profile (not on tab 5): "Born <date> (Age <n>). <Nationality> (<caps>)." with the
/// nations linked to their national teams (0x7d5; the second only without caps), the caps line by caps and goals, or
/// "(capped at B Team level)" / "(capped at U21 level)" from the staff state; on tab 1, for a person shown in full
/// with both sheets or a scout report, the view button (" Non-Playing" 1, " Playing" 0, " Scout Report" 2, the
/// current one marked 0x2002, item 0x12); then the positions (`database_format_player_positions` style 2; not on tab 1
/// in mode 1) or the job ("Unemployed") above the bottom.
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
pub(crate) unsafe fn profile_screen_identity(st: &mut ProfileScreen) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let draw: ProfileDrawFn = core::mem::transmute(FN_TEXT_DRAW_WRAPPED);
    let positions: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_FORMAT_PLAYER_POSITIONS);
    let text_item: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let s = Staff(st.staff);
    let text = st.text.as_mut_ptr() as usize;
    if st.tab != 5 {
        covered!("tel07_3 profile identity");
        let (mut date, mut born, mut nat, mut extra, mut caps) = ([0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8]);
        (exe::date_get_short_date_string(s.addr() + staff::DATE_OF_BIRTH, date.as_mut_ptr() as usize, 0xc8, 0xff) as usize);
        lang(born.as_mut_ptr() as usize, 0xa85010, 0x64usize, date.as_ptr() as usize, (s.age() as i8) as i32); // "Born {}<date> (Age <n>)"
        profile_screen_nationality(s.addr(), &mut nat, &mut extra);
        let apps = s.int_apps() as u32;
        let goals = s.int_goals() as u32;
        let cp = caps.as_mut_ptr() as usize;
        if apps > 1 {
            match goals {
                0 => drop(lang(cp, 0xa84fb8, apps)),        // "(<n> caps)"
                1 => drop(lang(cp, 0xa84fcc, apps)),        // "(<n> caps/1 goal)"
                _ => drop(lang(cp, 0xa84fe8, apps, goals)), // "(<n> caps/<g> goals)"
            }
        } else if apps == 1 {
            match goals {
                0 => drop(lang(cp, 0xa84f84)),         // "(1 cap)"
                1 => drop(lang(cp, 0xa84f8c)),         // "(1 cap/1 goal)"
                _ => drop(lang(cp, 0xa84f9c, goals)), // "(1 cap/<g> goals)"
            }
        } else if (s.is_null() || read_i32(s.addr()) < read_i32(G_STAFF_COUNT)) && s.nation().addr() != 0 {
            let state = profile_state(s.addr());
            if read_u8(state + staff_state::B_INTERNATIONAL_PLAYED) != 0 {
                lang(cp, 0xa84f5c); // "(capped at B Team level)"
            } else if read_u8(state + staff_state::EXTRA_FLAGS) & 2 != 0 {
                lang(cp, 0xa84f44); // "(capped at U21 level)"
            } else {
                caps[0] = 0;
            }
        } else {
            caps[0] = 0;
        }
        if caps[0] != 0 {
            format(PROFILE_BUF, 0xa84f78, born.as_ptr() as usize, nat.as_ptr() as usize, caps.as_ptr() as usize); // "%s. %s %s."
        } else {
            format(PROFILE_BUF, 0xa84f3c, born.as_ptr() as usize, nat.as_ptr() as usize); // "%s. %s."
        }
        exe::text_clear((text) as usize as *mut usize);
        exe::text_set_string(text, PROFILE_BUF as *const u8);
        // the links carry the lower-case adjectives (the line has them capitalised), as in the original
        let nation = s.nation().addr();
        if nation != 0 {
            exe::nation_get_nationality_adjective(nation, nat.as_mut_ptr() as usize, 0xc8, 0xc);
            exe::text_add_link(text, nat.as_ptr(), 0x7d5, exe::database_get_national_team(nation));
        }
        let second = s.second_nation().addr();
        if second != 0 && second != s.nation().addr() && s.int_apps() == 0 {
            exe::nation_get_nationality_adjective(second, extra.as_mut_ptr() as usize, 0xc8, 0xc);
            exe::text_add_link(text, extra.as_ptr(), 0x7d5, exe::database_get_national_team(second));
        }
        let y = st.y as usize;
        let a = profile_area(0x6e, y, 0x30c, y.wrapping_add(0x1e), 1, 0, 1, 0, 2, 0, usize::MAX);
        let mut row: i32 = 0;
        // the right edge is the push at 0x8750e1 in the running exe (0x30c; 0x421 in the 1920x1080 exe)
        draw(text, a, 0x6e, read_i32(0x8750e2) as usize, &mut row, 3, colour_word(G_GUI_COLOUR_YELLOW), 3, 0);
        let report = exe::scout_manager_find_club_search_entry(read_usize(G_SCOUT_MANAGER), s);
        let n = read_i32(G_STAFF_COUNT);
        let id = read_i32(s.addr());
        let job = s.job_for_club();
        let shown = job != 2 && (id < n.wrapping_sub(0x10) || (!s.is_null() && id >= n));
        let sheets = s.player().addr() != 0 && s.non_player().addr() != 0;
        if shown && st.tab == 1 && (sheets || report != 0) {
            covered!("tel07_3 profile view button");
            let y = st.y as usize;
            let b = profile_button(0x73, y.wrapping_add(5), 0x82, y.wrapping_add(0x19), 0x30, colour_word(G_GUI_COLOUR_BLACK), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), 0, 0, 0);
            let sub = profile_area(0, 0, 0, 0, 0, 0, 0, 0, 0x30, colour_word(0xae31aa), b);
            let entry = |text: usize, view: i32| {
                let kind = if st.mode == view { 0x2002 } else { 2 };
                profile_item(kind, 0x10, colour_word(0xae31aa), colour_word(G_GUI_COLOUR_YELLOW), colour_word(G_GUI_COLOUR_BLACK), text, 0x12, view as usize, sub);
            };
            if s.non_player().addr() != 0 {
                lang(PROFILE_BUF, 0xa84f2c); // " Non-Playing"
                entry(PROFILE_BUF, 1);
            }
            if s.player().addr() != 0 {
                lang(PROFILE_BUF, 0xa84f00, 0xad9c64usize); // " Playing"
                entry(PROFILE_BUF, 0);
            }
            if report != 0 {
                profile_separator(sub);
                lang(PROFILE_BUF, 0xa84ef0); // " Scout Report"
                entry(PROFILE_BUF, 2);
            }
        }
        st.y += 0x23;
    }
    let player = s.player().addr();
    if player != 0 && !(st.tab == 1 && st.mode == 1) {
        positions(player, PROFILE_BUF, 0x7d0, 2);
    } else if s.club_job().addr() != 0 {
        profile_strcpy(PROFILE_BUF, exe::database_get_job_name(s.job_for_club() as u32, 0, 1, core::ptr::null_mut()));
    } else if s.national_job().addr() != 0 {
        profile_strcpy(PROFILE_BUF, exe::database_get_job_name(s.job_for_nation() as u32, 0, 1, core::ptr::null_mut()));
    } else {
        lang(PROFILE_BUF, 0x9b6b04); // "Unemployed"
    }
    let b = st.bottom as usize;
    text_item(SCREEN_MANAGER, 0x6e, b.wrapping_sub(0x1e), 0x30c, b, 2, 0, 0xc, 3, colour_word(0xaebde8), PROFILE_BUF, 1);
    st.bottom -= 0x23;
}

/// The statistics table of a player of the database (7 rows of font 2 above the bottom, 12 columns aligned by
/// `align`): the first row the season of the career history (tab 5 with bit 2 of param 5 and a history: the club
/// linked, 0x7d5) or empty, the page buttons "<<" (0x3c) and ">>" (0x3d) (font symbols 0x2a / 0x2b when the font mode
/// is set) and the column titles (the list by param 0x14 and the goalkeeper rating 20 of the player sheet); then six
/// rows, each with its title and the values (`PlayerStats` 0x7c2750 from the history entry, or 0x7c25b0 from the
/// player sheet), the yellow / grey stripes, column 0x11 in its own colour.
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
pub(crate) unsafe fn profile_screen_statistics(st: &mut ProfileScreen, align: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let s = st.staff;
    let h = exe::font_get_height(2);
    let height = h.wrapping_mul(7);
    let (b, top) = (st.bottom as usize, st.bottom.wrapping_sub(height) as usize);
    profile_area(0x6e, top, 0x30c, b, 1, 0, 1, 0, 2, 0, usize::MAX);
    let table = profile_area(0x6e, top, 0x30c, b, 0xc, align, 7, 0, 1, 0, usize::MAX);
    let mut row: usize = 0;
    let from_history = st.tab == 5 && st.flags & 2 != 0 && st.history != 0 && read_i16(st.history + 0xeb) != -1;
    let mut entry = st.history.wrapping_add(st.history_row.wrapping_mul(0x2b));
    let mut name = [0u8; 0xc8];
    let np = name.as_mut_ptr() as usize;
    if from_history {
        covered!("tel07_3 profile statistics of a history season");
        let index = read_i32(entry + 2);
        let club = Club(if index > -1 { club_at(index as isize).addr() } else { 0 });
        if index > -1 && !club.is_null() {
            exe::key_nation_format_club_season(read_u16(entry) as u32, club, np, 0x7d0);
            format(PROFILE_BUF, 0xa6a1d0, np, club.addr() + club::SHORT_NAME); // "  %s %s"
            profile_cell_button(table, 0, row, 1, 0, 1, 2, colour_word(0xaebdde), PROFILE_BUF, 0x7d5, club.addr());
        } else {
            exe::key_nation_format_club_season(read_u16(entry) as u32, 0, np, 0x7d0);
            format(PROFILE_BUF, 0x987ff0, np); // "  %s"
            profile_cell(table, 0, row, 1, 0, 1, 2, colour_word(0xaebdde), PROFILE_BUF);
        }
    }
    let symbols = read_i32(G_FONT_MODE) != -1;
    for (col, symbol, arrow, code) in [(1usize, 0x2a, 0x98e7c4usize, 0x3c), (2, 0x2b, 0x98e7a8, 0x3d)] {
        let text = if symbols {
            format(PROFILE_BUF, 0x99ec1c, symbol); // "%c"
            PROFILE_BUF
        } else {
            arrow // "<<" / ">>"
        };
        profile_cell_button(table, col, row, 0x30, colour_word(G_GUI_COLOUR_DARK_BLUE), 0xc, (!symbols) as usize, colour_word(G_GUI_COLOUR_PALE_GREY), text, code, 0);
    }
    let columns = |s: usize| -> usize {
        let p = PlayerSheet(read_usize(s + staff::PLAYER));
        let keeper = !p.is_null() && p.goalkeeper() == 0x14;
        match (st.compare_columns != 0, keeper) {
            (true, true) => 0xa84a90,
            (true, false) => 0xa84a78,
            (false, true) => 0xa84a9c,
            (false, false) => 0xa84a84,
        }
    };
    let list = columns(s);
    let mut i = 0;
    while read_u8(list + i) != 0xff {
        exe::player_stats_get_field_label(read_usize(G_PLAYER_STATS), read_u8(list + i) as u32, PROFILE_BUF, 0x7d0);
        profile_cell(table, 3 + i, row, 0x30, colour_word(G_GUI_COLOUR_GREY), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), PROFILE_BUF);
        i += 1;
    }
    row += 1;
    entry = entry.wrapping_add(6);
    for k in 1..=6u32 {
        (exe::player_stats_get_category_name(read_usize(G_PLAYER_STATS), k, (np) as usize as *mut u8, 0xc8) as usize);
        format(PROFILE_BUF, 0x987ff0, np); // "  %s"
        profile_cell(table, 0, row, 1, 0, 1, 2, colour_word(G_GUI_COLOUR_PALE_GREY), PROFILE_BUF);
        let list = columns(st.staff);
        let mut i = 0;
        while read_u8(list + i) != 0xff {
            let column = read_u8(list + i);
            if st.tab == 5 && st.flags & 2 != 0 && st.history != 0 && read_i16(st.history + 0xeb) != -1 {
                exe::playerstats_format_record_field(read_usize(G_PLAYER_STATS), entry, column as u32, PROFILE_BUF, 0x7d0, 1, 0, 0);
            } else {
                (exe::playerstats_format_player_stat(read_usize(G_PLAYER_STATS), read_usize(st.staff + staff::PLAYER), k, column as u32, PROFILE_BUF, 0x7d0, 1, 0, 1) as usize);
            }
            if column == 0x11 {
                profile_cell(table, 3 + i, row, 0x30, colour_word(0xae3192), 0xc, 1, colour_word(0xaebde8), PROFILE_BUF);
            } else {
                let colour = if (k as i32 - 1) % 2 != 0 { colour_word(0xaebdd4) } else { colour_word(G_GUI_COLOUR_YELLOW) }; // tel07_3 the stripes
                profile_cell(table, 3 + i, row, 1, 0, 0xc, 1, colour, PROFILE_BUF);
            }
            i += 1;
        }
        row += 1;
        entry = entry.wrapping_add(0x2b);
    }
    st.bottom = st.bottom.wrapping_sub(5).wrapping_sub(height);
}

/// The Injuries & Bans tab (between `y` and `bottom`, two columns aligned 2 and 7): for a player of the database, the
/// injury (`Injury::get_player_injury_status`; the injury text by its type 0x4d..0x51: attending, surgery,
/// radiotherapy or the foreseeable future, leave, or "Out for <period> with <injury>"; without one the potential
/// injury of the category or "None"), its type (the cause in a wrapped Text, "Injured during training", or "  -"),
/// the condition (by condition, fatigue and the jadedness statistic: lacking match practice, in need of a rest,
/// exhausted, or the percentage; "  -" for status 2), the training (full, light or none, "(absent from training
/// recently)" for one who missed training), the bans (`Discipline::get_slot_text` of the four slots, or "None") and
/// empty rows down to the bottom; otherwise "There is no injury and ban information available.". A person without an
/// injury record despite the status: the tab stops after the injury row (the original jumps to the footer).
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
pub(crate) unsafe fn profile_screen_injuries_tab(st: &mut ProfileScreen) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let draw: ProfileDrawFn = core::mem::transmute(FN_TEXT_DRAW_WRAPPED);
    let (y, bottom) = (st.y, st.bottom);
    let mut cause = [0usize; 2];
    let cp = cause.as_mut_ptr();
    exe::new_text(cp, 0);
    let align: [u8; 2] = [2, 7];
    let s = Staff(get_screen_param(0));
    let inj = read_usize(G_INJURY);
    let pale = colour_word(G_GUI_COLOUR_PALE_GREY);
    let yellow = colour_word(G_GUI_COLOUR_YELLOW);
    if s.player().addr() == 0 || (!s.is_null() && read_i32(s.addr()) >= read_i32(G_STAFF_COUNT)) {
        covered!("tel07_3 profile without injury information");
        let rows = (bottom.wrapping_sub(y) / exe::font_get_height(3)) as u8;
        lang(PROFILE_BUF, 0xa84adc); // "There is no injury and ban information available."
        let text_area: unsafe extern "C" fn(i32, i32, i32, i32, u32, u32, u32, u32, usize, usize, u32) -> u32 = core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
        text_area(0x6e, y, 0x30c, bottom, 2, 0, rows as u32, 3, pale, PROFILE_BUF, 0);
        exe::drop_text(cp);
        return;
    }
    covered!("tel07_3 profile injuries tab");
    let rows = (bottom.wrapping_sub(y) / exe::font_get_height(2)) as u8;
    // simplification: the original pushes the row count as a dword whose high bytes are a stale local; here zero-extended
    let area = exe::screenmanager_add_area(SCREEN_MANAGER, 0x6e, y as usize, 0x30c, bottom as usize, 2, align.as_ptr() as usize, rows as usize, 0, 1, 0, usize::MAX);
    let mut row: i32 = 0;
    let status = exe::injury_get_player_injury_status(inj, s);
    lang(PROFILE_BUF, 0xa84ee4); // "  Injury"
    profile_cell(area, 0, row as usize, 2, 0, 1, 2, pale, PROFILE_BUF);
    if status != 0 {
        let (mut name, mut period) = ([0u8; 0x100], [0u8; 0x100]);
        let (np, pp) = (name.as_mut_ptr() as usize, period.as_mut_ptr() as usize);
        exe::injury_describe_player_injury(inj, s, np, 0xc8, 1);
        exe::injury_format_days_out(inj, s, pp, 0xc8, 1, 1);
        let injury = exe::injury_get_player_injury(inj, s);
        if injury == 0 {
            covered!("tel07_3 profile injury status without an injury");
            exe::drop_text(cp);
            return;
        }
        match read_i8(injury) as i32 - 0x4d {
            0 => drop(lang(PROFILE_BUF, 0xa84e94, pp)),
            1 => drop(lang(PROFILE_BUF, 0xa84e4c, pp)),
            2 => drop(lang(PROFILE_BUF, 0xa84d84, pp)), // "  Out for <period> due to surgery"
            3 => drop(if status == 1 { lang(PROFILE_BUF, 0xa84dfc) } else { lang(PROFILE_BUF, 0xa84dac) }),
            4 => drop(lang(PROFILE_BUF, 0xa84d64, pp)), // "  On leave for <period>"
            _ => drop(lang(PROFILE_BUF, 0xa84d38, pp, np)), // "  Out for <period> with <injury>"
        }
    } else {
        let p = exe::injury_find_match_injury(inj, s);
        if p != 0 {
            let mut name = [0u8; 0x100];
            let np = name.as_mut_ptr() as usize;
            let g = exe::injury_get_category_name(inj, read_u8(p + 4) as u32, np, 0xc8) as i32;
            lang(PROFILE_BUF, 0xa84d00, g, g, np); // "  Potential {}<category>{}"
        } else {
            lang(PROFILE_BUF, 0xa84cdc, 0xad9c64usize); // "  None"
        }
    }
    profile_cell(area, 1, row as usize, 2, 0, 1, 2, yellow, PROFILE_BUF);
    row += 1;
    lang(PROFILE_BUF, 0xa84cb8, 0xad9c64usize); // "  Type"
    profile_cell(area, 0, row as usize, 2, 0, 1, 2, pale, PROFILE_BUF);
    let mut type_text = 0x98e8c8usize; // "  -"
    let mut wrapped = false;
    if status != 0 {
        let injury = exe::injury_get_player_injury(inj, s);
        if read_u8(injury + 0x1a) == 0xff {
            if exe::injury_get_type_category(inj, read_i8(injury)) != 0 && exe::injury_get_type_category(inj, read_i8(injury)) != 0xc {
                lang(PROFILE_BUF, 0xa84c9c); // "  Injured during training"
                type_text = PROFILE_BUF;
            }
        } else {
            exe::injury_write_match_injury_text(inj, injury, cp as usize);
            profile_cell(area, 1, row as usize, 2, 0, 1, 2, yellow, 0);
            draw(cp as usize, area, 0x102, 0x30c, &mut row, 2, yellow, 2, 0);
            wrapped = true;
        }
    }
    if !wrapped {
        profile_cell(area, 1, row as usize, 2, 0, 1, 2, yellow, type_text);
    }
    row += 1;
    lang(PROFILE_BUF, 0xa84c90); // "  Condition"
    profile_cell(area, 0, row as usize, 2, 0, 1, 2, pale, PROFILE_BUF);
    if status == 2 {
        profile_strcpy(PROFILE_BUF, 0x98e8c8); // "  -"
    } else {
        let id = read_u32(s.addr());
        if exe::injury_get_condition(inj, s, 1) > 0x5f && exe::injury_get_fatigue(inj, id) >= -750 {
            format(PROFILE_BUF, 0xa84c08, exe::injury_get_condition(inj, s, 1) as i32); // "  %d%%"
        } else {
            let jaded = exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), s.player().addr(), 6, 1, 6.5) as i32 as i16;
            if exe::injury_get_fatigue(inj, id) < -750 && jaded < 0xa {
                lang(PROFILE_BUF, 0xa84c60, exe::injury_get_condition(inj, s, 1) as i32); // lacking match practice
            } else if exe::injury_get_fatigue(inj, id) > 0x44c && jaded > 0x1e {
                lang(PROFILE_BUF, 0xa84c34, exe::injury_get_condition(inj, s, 1) as i32); // in need of a rest
            } else if exe::injury_get_fatigue(inj, id) > 0x4c9 {
                lang(PROFILE_BUF, 0xa84c10, exe::injury_get_condition(inj, s, 1) as i32); // exhausted
            } else {
                format(PROFILE_BUF, 0xa84c08, exe::injury_get_condition(inj, s, 1) as i32); // "  %d%%"
            }
        }
    }
    profile_cell(area, 1, row as usize, 2, 0, 1, 2, yellow, PROFILE_BUF);
    row += 1;
    lang(PROFILE_BUF, 0xa84bfc); // "  Training"
    profile_cell(area, 0, row as usize, 2, 0, 1, 2, pale, PROFILE_BUF);
    let awol = read_usize(G_AWOL);
    let missed = awol != 0 && exe::awol_has_missed_training(awol, s) != 0;
    match (status, missed) {
        (0, true) => drop(lang(PROFILE_BUF, 0xa84bb4, 0xad9c64usize)), // "  Full (absent from training recently)"
        (1, true) => drop(lang(PROFILE_BUF, 0xa84b8c)),                // "  Light (absent from training recently)"
        (0, false) => drop(lang(PROFILE_BUF, 0xa84b64, 0xad9c64usize)), // "  Full"
        (1, false) => drop(lang(PROFILE_BUF, 0xa84b5c)),               // "  Light"
        _ => drop(lang(PROFILE_BUF, 0xa84b38, 0xad9c64usize)),          // "  None"
    }
    profile_cell(area, 1, row as usize, 2, 0, 1, 2, yellow, PROFILE_BUF);
    row += 1;
    lang(PROFILE_BUF, 0xa84b30); // "  Bans"
    profile_cell(area, 0, row as usize, 2, 0, 1, 2, pale, PROFILE_BUF);
    let first = row;
    let mut ban = [0u8; 0x100];
    let bp = ban.as_mut_ptr() as usize;
    for k in 0..4u32 {
        if exe::discipline_get_slot_text(read_usize(G_DISCIPLINE), s, k, bp, 0xc8) != 0 {
            if row > first {
                profile_cell(area, 0, row as usize, 2, 0, 1, 2, pale, 0);
            }
            format(PROFILE_BUF, 0x987ff0, bp); // "  %s"
            profile_cell(area, 1, row as usize, 2, 0, 1, 2, yellow, PROFILE_BUF);
            row += 1;
        }
    }
    if row == first {
        lang(PROFILE_BUF, 0xa84b10, 0xad9c64usize); // "  None"
        profile_cell(area, 1, row as usize, 2, 0, 1, 2, yellow, PROFILE_BUF);
        row += 1;
    }
    while row < rows as i32 {
        profile_cell(area, 0, row as usize, 2, 0, 0xc, 2, pale, 0);
        profile_cell(area, 1, row as usize, 2, 0, 0xc, 2, pale, 0);
        row += 1;
    }
    exe::drop_text(cp);
}

pub(crate) unsafe fn profile_screen_plan() -> ProfilePlan {
    let s = Staff(get_screen_param(0));
    let mut flags = get_screen_param(5) as u32;
    if s.player().addr() == 0 {
        flags &= 0xfffffff1;
        if flags == 0 {
            flags = 1;
        }
    }
    if exe::staff_records_get_manager_record(read_usize(G_STAFF_RECORDS), s, 0) == 0 {
        flags &= 0xffffffef;
        if flags == 0 {
            flags = 1;
        }
    }
    let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), s) == 0
        && s.non_player().addr() != 0
        && (s.is_null() || read_i32(s.addr()) < read_i32(G_STAFF_COUNT))
        && s.club_job().addr() != 0
        && exe::is_contract_staff_job(s.job_for_club() as u32) != 0;
    let tab = get_screen_param(1) as u8;
    let history = tab == 5 && flags & 2 != 0 && get_screen_param(6) == 0 && s.player().addr() != 0;
    ProfilePlan { staff: s.addr(), tab, list: get_screen_param(3) == 0, contract, history, flags }
}

/// `langlib_sprintf` (0x66f4e0, cdecl, variadic): the translated format into `out`.
pub(crate) unsafe fn staff_tab_lang() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_LANGLIB_SPRINTF)
}

/// The CRT `sprintf` (0x944c9f, cdecl, variadic).
pub(crate) unsafe fn staff_tab_format() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_CRT_SPRINTF)
}

/// `ScreenManager::add_area_text` (0x804100) of the staff profile tabs: cell (`col`, `row`) of `area`, the four
/// style fields `a`..`d`, the colour word at `colour` (zero-extended: the original pushes the register with only its
/// low half loaded) and the text (0 = an empty cell).
pub(crate) unsafe fn staff_tab_cell(area: u32, col: u32, row: i32, a: u32, b: u32, c: u32, d: u32, colour: usize, text: usize) -> u32 {
    tel26_area_text([area, col, row as u32, a, b, c, d, job_colour(colour), text as u32])
}

/// `ScreenManager::add_area_button` (0x8040a0) of the staff profile tabs: like `staff_tab_cell` plus the command id
/// and its value.
pub(crate) unsafe fn staff_tab_button(area: u32, col: u32, row: i32, a: u32, b: u32, c: u32, d: u32, colour: usize, text: usize, id: u32, value: u32) -> u32 {
    tel26_area_button([area, col, row as u32, a, b, c, d, job_colour(colour), text as u32, id, value])
}

/// `Text::draw_wrapped` (0x4aeaf0, thiscall, ret 0x20): draws the Text into `area` from x1 to x2 starting at row
/// `*row` (which it moves on when the text wraps), with the font, the colour word at `colour`, the style and the flags.
pub(crate) unsafe fn staff_tab_draw_wrapped(text: *mut usize, area: u32, x1: u32, x2: u32, row: &mut i32, font: u32, colour: usize, style: u32, flags: u32) {
    exe::text_draw_wrapped((text) as *mut usize as usize, area, (x1) as u32 as i32, (x2) as u32 as i32, row, font, job_colour(colour), style, flags);
}

/// How many rows of font `font` fit between `top` and `bottom` (signed division, kept as a byte like the original).
pub(crate) unsafe fn staff_tab_rows(top: i32, bottom: i32, font: i32) -> u8 {
    ((bottom - top) / exe::font_get_height(font)) as u8
}

/// The "no information" text of a tab: the message `msg` (translated into the screens' buffer) as a text area from
/// x 0x6e to 0x30c between `top` and `bottom`, in font 3, as many lines as fit in that font.
pub(crate) unsafe fn staff_tab_no_information(top: i32, bottom: i32, msg: usize) {
    let lines = staff_tab_rows(top, bottom, 3);
    staff_tab_lang()(JOB_TEXT, msg);
    tel26_text_area([0x6e, top as u32, 0x30c, bottom as u32, 2, 0, lines as u32, 3, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 0]);
}

/// The CRT's inline strcpy of the original (the same bytes).
pub(crate) unsafe fn staff_tab_strcpy(dst: usize, src: usize) {
    let mut i = 0;
    loop {
        let c = read_u8(src + i);
        write_u8(dst + i, c);
        if c == 0 {
            break;
        }
        i += 1;
    }
}

/// The contract of person `p` at his club (`CONTRACT_MANAGER::get_contract`), also left in
/// `G_CONTRACT_LOOKUP_RESULT` as the original does each time it asks, and its club (null without a contract).
pub(crate) unsafe fn contract_tab_lookup(p: Staff) -> (Contract, usize) {
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p));
    write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
    let club = if c.is_null() { 0 } else { club_at(c.club() as isize).addr() };
    (c, club)
}

/// The contract type text of the Contract tab into the screens' buffer: the low 6 bits of `contract::CONTRACT_TYPE`
/// as a signed number (0 invalid, 1 monthly, 2 normal, 3 and 4 "N/A", 5 trial, 6 and 7 loan), "Part Time" when bit
/// 0x80 is set, "Full Time" otherwise; any other value: "Part Time Contract", or "Full Time" with the comment for a
/// full-time one. No contract: "N/A".
pub(crate) unsafe fn contract_tab_type_text(c: Contract) {
    const NOT_AVAILABLE: usize = 0xa86748; // "  N/A"
    let lang = staff_tab_lang();
    if c.is_null() {
        lang(JOB_TEXT, NOT_AVAILABLE);
        return;
    }
    let b = c.contract_type();
    let kind = ((b << 2) as i8 >> 2) as i32;
    let s = if b & 0x80 != 0 {
        match kind {
            0 => 0xa867ec, // "  Invalid Contract Type"
            1 => 0xa86854, // "  Part Time Monthly Contract"
            3 | 4 => NOT_AVAILABLE,
            5 => 0xa86838, // "  Part Time Trial Contract"
            6 | 7 => 0xa8681c, // "  Part Time Loan Contract"
            _ => 0xa86804, // "  Part Time Contract"
        }
    } else {
        match kind {
            0 => 0xa867ec, // "  Invalid Contract Type"
            1 => 0xa867cc, // "  Full Time Monthly Contract"
            2 => 0xa867b4, // "  Full Time Contract"
            3 | 4 => NOT_AVAILABLE,
            5 => 0xa86798, // "  Full Time Trial Contract"
            6 | 7 => 0xa8677c, // "  Full Time Loan Contract"
            _ => {
                lang(JOB_TEXT, 0xa86750usize, 0xad9c64usize); // "  Full Time<%s - COMMENT - contract type>"
                return;
            }
        }
    };
    lang(JOB_TEXT, s);
}

/// The work permit line of the Contract tab into the screens' buffer: the games played and missed under the permit
/// and the share played (played × 100 / (played + missed), integer, 0 with no games), with the singular texts for 1.
pub(crate) unsafe fn contract_tab_work_permit_text(permit: usize) {
    let lang = staff_tab_lang();
    let played = read_u16(permit + work_permit::MATCHES_PLAYED);
    let missed = read_u16(permit + work_permit::MATCHES_MISSED);
    let total = played as i32 + missed as i32;
    let share = if total != 0 { ((played as i32 * 100) / total) as u8 as i8 } else { 0 };
    if played != 1 && missed != 1 {
        lang(JOB_TEXT, 0xa86498usize, played as u32, missed as u32, share as i32); // "  %d games played, %d games missed (%d%%)"
    } else if played != 1 {
        lang(JOB_TEXT, 0xa863f0usize, played as u32, share as i32); // "  %d games played, 1 game missed (%d%%)"
    } else if missed != 1 {
        lang(JOB_TEXT, 0xa86444usize, missed as u32, share as i32); // "  1 game played, %d games missed (%d%%)"
    } else {
        lang(JOB_TEXT, 0xa863c8usize); // "  1 game played, 1 game missed (50%%)"
    }
}

/// The player attribute table of the Profile tab: six columns (weights 3 2 3 2 3 2) between `top` and `bottom`, as
/// many rows as fit in font 2. Brings the player's training up to date (`update_player_training`), fills the 31
/// attribute rows (the value the human sees with attribute masking on, `staff_get_player_attribute_for_viewer`, else
/// `staff_compute_player_attribute`), sorts them by name and adds the five extra rows; then three name/value column
/// pairs of 12 rows: the value in yellow, the extra rows' texts in light blue (`profile_extra_row_text`; the form
/// with one more space and another style), and for the On Loan row an empty item across the value columns and "On
/// loan from <club>" (linked 0x7d5) or "Given free transfer" wrapped from x 0x238. Each column is padded with empty
/// rows to the tallest.
/// Original: part of `staff_screens_draw_profile_tab` (0x87a510, staff_screens.cpp).
pub(crate) unsafe fn profile_tab_player_attributes(top: i32, bottom: i32, p: Staff, manager: usize, t: *mut usize, weights: &[u8; 6]) {
    let lang = staff_tab_lang();
    let format = staff_tab_format();
    let rows = staff_tab_rows(top, bottom, 2);
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 6, weights.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    exe::training_manager_update_player_training(read_usize(G_TRAINING_MANAGER), p, 0);
    for i in 1..=31u32 {
        let r = profile_row(i as usize - 1);
        exe::database_get_player_attribute_name(i, r, 100, 0);
        let v = if read_u8(G_ATTRIBUTE_MASKING_ENABLED) != 0 {
            exe::staff_get_player_attribute_for_viewer(manager, p, i)
        } else {
            exe::staff_compute_player_attribute(p, p.player(), i)
        };
        write_u8(r + PROFILE_ROW_VALUE, v);
    }
    exe::crt_qsort(PROFILE_TABLE, 0x1f, PROFILE_ROW as u32, PROFILE_ROW_COMPARER);
    let extra = [
        (0xa85874usize, 0xffu8), // "Preferred Foot"
        (0x98e9f8, 0xfe),        // "Form"
        (0x98e9f0, 0xfd),        // "Morale"
        (0xa86380, 0xfc),        // "Condition"
        (0xa85860, 0xfb),        // "On Loan"
    ];
    for (k, (s, code)) in extra.iter().enumerate() {
        let r = profile_row(31 + k);
        lang(r, *s);
        write_u8(r + PROFILE_ROW_VALUE, *code);
    }
    let (counts, tallest) = profile_columns(PROFILE_PLAYER_ROWS, 3, rows);
    let mut col: u32 = 0;
    let mut row: i32 = 0;
    let mut out = [0u8; 0x100];
    let o = out.as_mut_ptr() as usize;
    for k in 0..PROFILE_PLAYER_ROWS {
        let r = profile_row(k);
        let code = read_u8(r + PROFILE_ROW_VALUE);
        if code == 0xfb {
            tel26_add_item(current_human_display(), [1, 0x22e, 0, 0x30c, 0, 0, row as u32, 2, 0, 0, 0xc, 2, job_colour(G_GUI_COLOUR_PALE_GREY), 0, 0, 0, 0, area]);
            let mut done = false;
            if exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) != 0 {
                let (c, club) = contract_tab_lookup(p);
                if !c.is_null() && club != 0 {
                    covered!("tel08_2 profile tab on loan");
                    let gender = read_i8(club + club::SHORT_GENDER_NAME) as i32;
                    lang(JOB_TEXT, 0xa85834usize, gender, gender, club + club::SHORT_NAME); // "On loan from {}<%s - Club>{}"
                    exe::text_set_string(t as usize, JOB_TEXT as *const u8);
                    exe::text_add_link(t as usize, (club + club::SHORT_NAME) as *const u8, 0x7d5, club);
                    staff_tab_draw_wrapped(t, area, 0x238, 0x30c, &mut row, 2, PROFILE_COLOUR_3152, 3, 0);
                    done = true;
                }
            }
            if !done && exe::transfer_manager_is_queued_for_free_transfer(read_usize(G_TRANSFER_MANAGER), p) != 0 {
                lang(JOB_TEXT, 0xa85820usize); // "Given free transfer"
                exe::text_set_string(t as usize, JOB_TEXT as *const u8);
                staff_tab_draw_wrapped(t, area, 0x238, 0x30c, &mut row, 2, PROFILE_COLOUR_3152, 3, 0);
            }
        } else {
            format(JOB_TEXT, 0x987ff0usize, r); // "  %s"
            staff_tab_cell(area, col, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
            if (code as i8) >= 0 {
                format(JOB_TEXT, 0x98806cusize, code as i8 as i32); // "%d" tel08_2 the attribute value
                staff_tab_cell(area, col + 1, row, 2, 0, 0xc, 2, G_GUI_COLOUR_YELLOW, JOB_TEXT);
            } else {
                profile_extra_row_text(code, p, o);
                if code == 0xfe {
                    format(JOB_TEXT, 0xa694f4usize, o); // "   %s"
                    staff_tab_cell(area, col + 1, row, 2, 0, 1, 1, JOB_COLOUR_BDD4, JOB_TEXT);
                } else if code == 0xf9 {
                    format(JOB_TEXT, 0x98e8c8usize); // "  -"
                    staff_tab_cell(area, col + 1, row, 2, 0, 0xc, 2, JOB_COLOUR_BDD4, JOB_TEXT);
                } else {
                    format(JOB_TEXT, 0x987ff0usize, o); // "  %s"
                    staff_tab_cell(area, col + 1, row, 2, 0, 1, 2, JOB_COLOUR_BDD4, JOB_TEXT);
                }
            }
        }
        row += 1;
        if row == counts[(col / 2) as usize] as i32 {
            profile_blank_rows(area, col, &mut row, tallest);
            col += 2;
            row = 0;
            if col + 1 > 5 {
                return;
            }
        }
    }
    while col < 6 {
        profile_blank_rows(area, col, &mut row, tallest);
        col += 2;
        row = 0;
    }
}

/// The non-player profile of the Profile tab. For a person who is not a chairman (`job_for_club` ≠ 1), the attribute
/// table: the 12 non-player attributes (the 10th only when the person's club or national job is 10 or the attribute
/// is at least 10), sorted by name, plus the coaching style row (coaching technique ≤ 5 fitness-based, ≥ 15
/// technique-based, else general), in two name/value column pairs (weights 3 2 3 2) as tall as half the rows. For a
/// chairman whose state record has bit 0x40, the chairman table (`profile_tab_chairman`). Then, unless he is a
/// chairman without that bit, the formation table (`profile_tab_formation`) 5 pixels under the table above.
/// Original: part of `staff_screens_draw_profile_tab` (0x87a510, staff_screens.cpp).
pub(crate) unsafe fn profile_tab_non_player(top: i32, bottom: i32, p: Staff, w4: &[u8; 4], w3: &[u8; 3], w2: &[u8; 2]) {
    let lang = staff_tab_lang();
    let format = staff_tab_format();
    let np = p.non_player();
    let chairman = p.job_for_club() == 1;
    let mut height = 0;
    if !chairman {
        covered!("tel08_2 profile tab non-player attributes");
        let mut n = 0usize;
        for i in 1..=12u32 {
            if i == 10
                && p.job_for_club() != 10
                && p.job_for_nation() != 10
                && (exe::staff_compute_non_player_attribute(p, np, 10) as i8) < 10
            {
                continue;
            }
            let r = profile_row(n);
            exe::database_get_non_player_attribute_name(i, r, 100, 0);
            write_u8(r + PROFILE_ROW_VALUE, exe::staff_compute_non_player_attribute(p, np, i));
            n += 1;
        }
        exe::crt_qsort(PROFILE_TABLE, n as u32, PROFILE_ROW as u32, PROFILE_ROW_COMPARER);
        lang(profile_row(n), 0xa862f4usize); // "Coaching Style"
        write_u8(profile_row(n) + PROFILE_ROW_VALUE, 0xfa);
        n += 1;
        let half = ((n as i32 + 1) / 2) as u8;
        height = exe::font_get_height(2) * half as i32;
        let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, (top + height) as u32, 4, w4.as_ptr() as u32, half as u32, 0, 1, 0, u32::MAX]);
        let (counts, tallest) = profile_columns(n, 2, 0);
        let mut col: u32 = 0;
        let mut row: i32 = 0;
        for k in 0..n {
            let r = profile_row(k);
            format(JOB_TEXT, 0x987ff0usize, r); // "  %s"
            staff_tab_cell(area, col, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
            let code = read_u8(r + PROFILE_ROW_VALUE);
            let colour = if code == 0xfa {
                let c = read_i8(np.addr() + non_player::COACHING_TECHNIQUE);
                if c <= 5 {
                    lang(JOB_TEXT, 0xa862e4usize); // "Fitness-based"
                } else if c >= 15 {
                    lang(JOB_TEXT, 0xa862d4usize); // "Technique-based"
                } else {
                    lang(JOB_TEXT, 0xa862a4usize, 0xad9c64usize); // "General<%s - COMMENT - general coaching style>"
                }
                JOB_COLOUR_BDD4
            } else {
                format(JOB_TEXT, 0x98806cusize, code as i8 as i32); // "%d"
                G_GUI_COLOUR_YELLOW
            };
            staff_tab_cell(area, col + 1, row, 2, 0, 0xc, 2, colour, JOB_TEXT);
            row += 1;
            if row == counts[(col / 2) as usize] as i32 {
                profile_blank_rows(area, col, &mut row, tallest);
                col += 2;
                row = 0;
                if col > 2 {
                    break;
                }
            }
        }
        while col < 4 {
            profile_blank_rows(area, col, &mut row, tallest);
            col += 2;
            row = 0;
        }
    }
    let state = read_u8(read_usize(G_STAFF_STATE_TABLE) + read_i32(p.addr()) as usize * staff_state::SIZE + staff_state::STATUS_FLAGS);
    let shown = state & PROFILE_STATE_SHOWN != 0;
    if shown && chairman {
        covered!("tel08_2 profile tab chairman");
        height = profile_tab_chairman(top, bottom, p, w3);
    }
    if !shown && chairman {
        return;
    }
    profile_tab_formation(top + height + 5, bottom, p, w2);
}

/// The match history of the Profile tab (the person has a scouting search entry): a frame (14 columns), the header
/// row (the arrows 4 and 5, buttons 0x27 and 0x28, as font characters 0x2a/0x2b or "<<"/">>" by `G_FONT_MODE`; the
/// statistics headers of view `kind` (screen parameter 0x13, `profile_history_headers`) and "Min") and up to 10
/// matches (`player_search_slot_get_match_entry`): the date, "<score> v <opposition> (<venue>)" wrapped from x 0xbd
/// to 0x15c (the score linked 0x7d3 to the match when the person's club played it, `fixture_entry_matches`; the club
/// linked 0x7d5; grey for an opposition id below 0), the six statistics of the view and the rating.
/// Original: part of `staff_screens_draw_profile_tab` (0x87a510, staff_screens.cpp).
pub(crate) unsafe fn profile_tab_match_history(top: i32, bottom: i32, p: Staff, entry: usize, t: *mut usize, w_header: &[u8; 14], w_matches: &[u8; 14]) {
    let lang = staff_tab_lang();
    let format = staff_tab_format();
    let kind = get_screen_param(0x13) as u8;
    let d = current_human_display();
    tel26_add_area(d, [0x6e, top as u32, 0x30c, bottom as u32, 0xe, w_header.as_ptr() as u32, 1, 0, 2, 0, u32::MAX]);
    let (y1, y2) = (top + 4, bottom - 4);
    let rows = staff_tab_rows(y1, y2, 2);
    let mut row: i32 = 0;
    let area = tel26_add_area(d, [0x6e, y1 as u32, 0x30c, y2 as u32, 0xe, w_header.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    for col in 0..3 {
        staff_tab_cell(area, col, row, 1, 0, 0xc, 1, G_GUI_COLOUR_PALE_GREY, 0);
    }
    let blue = job_colour(G_GUI_COLOUR_DARK_BLUE);
    for (col, ch, arrows, id) in [(4u32, 0x2au32, 0x98e7c4usize, 0x27u32), (5, 0x2b, 0x98e7a8, 0x28)] {
        if read_i32(G_FONT_MODE) != -1 {
            format(JOB_TEXT, 0x99ec1cusize, ch); // "%c"
            staff_tab_button(area, col, row, 0x30, blue, 0xc, 0, G_GUI_COLOUR_PALE_GREY, JOB_TEXT, id, 0);
        } else {
            staff_tab_button(area, col, row, 0x30, blue, 0xc, 1, G_GUI_COLOUR_PALE_GREY, arrows, id, 0); // "<<" / ">>"
        }
    }
    let grey = job_colour(G_GUI_COLOUR_GREY);
    let headers = profile_history_headers(kind);
    for (k, (s, comment)) in headers.iter().chain([(0xa85a3c, true)].iter()).enumerate() {
        if *comment {
            lang(JOB_TEXT, *s, 0xad9c64usize);
        } else {
            lang(JOB_TEXT, *s);
        }
        staff_tab_cell(area, 7 + k as u32, row, 0x30, grey, 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
    }

    let table = tel26_add_area(d, [0x6e, y1 as u32, 0x30c, y2 as u32, 0xe, w_matches.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    row = 1;
    let (mut score, mut venue) = ([0u8; 0x100], [0u8; 0x100]);
    let (sb, vb) = (score.as_mut_ptr() as usize, venue.as_mut_ptr() as usize);
    for k in 0..10u32 {
        let e = exe::player_search_slot_get_match_entry(entry, k);
        if e == 0 {
            break;
        }
        covered!("tel08_2 profile tab match history");
        let mut date = [0u16; 4];
        exe::new_date(date.as_mut_ptr() as usize, read_u16(e) as u32, read_u16(e + 2) as u32);
        exe::date_get_short_date_string(date.as_ptr() as usize, JOB_TEXT, 2000, 0xff);
        staff_tab_cell(table, 0, row, 0x30, blue, 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
        lang(sb, 0x990bb4usize, read_u8(e + 8) as u32, read_u8(e + 9) as u32); // "<%d - home score>:<%d - away score>"
        exe::format_venue_marker(read_u8(e + 0xe) as u32, vb, 0xc8);
        let opposition = read_i32(e + 4);
        let club = club_at(opposition.wrapping_abs() as isize).addr();
        let gender = read_i8(club + club::SHORT_GENDER_NAME) as i32;
        // "  <%s - score> v {}<%s - opposition team>{} (<%s - venue e.g. H>)"
        lang(JOB_TEXT, 0xa859f8usize, gender, gender, sb, club + club::SHORT_NAME, vb);
        staff_tab_text_clear(t);
        exe::text_set_string(t as usize, JOB_TEXT as *const u8);
        let own = p.club_job();
        if !own.is_null()
            && exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), read_i32(e + 0xa), read_i32(own.addr()), opposition, read_u16(e) as i32, read_u16(e + 2) as i32) != 0
        {
            exe::text_add_link(t as usize, sb as *const u8, 0x7d3, read_usize(e + 0xa));
        }
        let club = club_at(read_i32(e + 4).wrapping_abs() as isize).addr();
        exe::text_add_link(t as usize, (club + club::SHORT_NAME) as *const u8, 0x7d5, club);
        let colour = if read_i32(e + 4) > -1 { G_GUI_COLOUR_PALE_GREY } else { G_GUI_COLOUR_GREY };
        staff_tab_draw_wrapped(t, table, 0xbd, 0x15c, &mut row, 2, colour, 2, 0);
        for c in 0..6 {
            let (at, signed) = profile_history_stat(kind, c);
            if kind > 2 && c == 0 {
                let scored = read_u8(e + 0x26);
                if scored == 0 {
                    format(JOB_TEXT, 0x99cc54usize); // "0"
                } else {
                    format(JOB_TEXT, 0x99cc58usize, scored as u32, read_u8(e + 0x27) as u32); // "%d (%d)"
                }
            } else {
                let v = if signed { read_i8(e + at) as i32 } else { read_u8(e + at) as i32 };
                format(JOB_TEXT, 0x98806cusize, v); // "%d"
            }
            staff_tab_cell(table, 7 + c as u32, row, 1, 0, 0xc, 2, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
        }
        format(JOB_TEXT, 0x98806cusize, read_u8(e + 0xf) as u32); // "%d" the rating
        staff_tab_cell(table, 0xd, row, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 2, G_GUI_COLOUR_YELLOW, JOB_TEXT);
        row += 1;
    }
}

/// The top of the Transfer tab: Full Name (when the person has a common name or the human gave him a nickname: the
/// full name, kind 7), Availability (transfer listed by the club, by request, listed for loan or unknown), Value (for
/// a player job without a trial contract: `staff::VALUE` as money, a button 5 when the human controls the person's
/// club or his contract's club), Second Nationality (when set and different: "2nd Nationality" with English names,
/// the nationality as a button 0x7d5 to the national team) and the fluent (fluency ≥ 8) and basic (4..7) languages,
/// joined with ", " and " and ", the basic row only when there is one. Leaves the contract (first lookup, not stored)
/// in the state.
pub(crate) unsafe fn transfer_tab_details(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let format = staff_tab_format();
    let p = st.p;
    let mut buf = [0u8; 0x578];
    let b = buf.as_mut_ptr() as usize;

    let common = p.common_name();
    if (common != 0 && read_u8(common) != 0) || exe::human_manager_has_nickname(read_usize(G_HUMAN_MANAGER), p, 0) != 0 {
        covered!("tel08_3 transfer tab full name");
        lang(JOB_TEXT, 0xa8aeccusize); // "  Full Name"
        st.label(JOB_TEXT);
        exe::database_format_staff_name(p, b, 0xc8, 7);
        format(JOB_TEXT, 0x987ff0usize, b); // "  %s"
        st.value(JOB_TEXT);
        st.row += 1;
    }

    lang(JOB_TEXT, 0xa8aebcusize); // "  Availability"
    st.label(JOB_TEXT);
    if exe::staff_is_transfer_listed(p, 0usize) != 0 {
        lang(JOB_TEXT, 0xa8aea0usize); // "  Transfer listed by club"
    } else if exe::staff_is_transfer_listed_or_requested(p, 0usize) != 0 {
        lang(JOB_TEXT, 0xa8ae80usize); // "  Transfer listed by request"
    } else if exe::staff_is_loan_listed(p, 0usize) != 0 {
        lang(JOB_TEXT, 0xa8ae6cusize); // "  Listed for loan"
    } else {
        lang(JOB_TEXT, 0xa8ae3cusize, TRANSFER_TAB_COMMENT); // "  Unknown<%s - COMMENT - unknown availabilty>" tel08_3 unknown availability
    }
    st.value(JOB_TEXT);
    st.row += 1;

    let contract = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p));
    st.contract = contract;
    if exe::database_is_player_job(p.job_for_club() as i8) != 0 && {
        let (c, _) = contract_tab_lookup(p);
        c.is_null() || c.contract_type() & 0x3f != 5
    } {
        covered!("tel08_3 transfer tab value");
        lang(JOB_TEXT, 0xa8ae34usize); // "  Value"
        st.label(JOB_TEXT);
        let value = p.value() as u32;
        let amount = exe::cash_get_display_amount(&value, 6);
        exe::cash_format_money(b, 0xc8, amount, 6);
        format(JOB_TEXT, 0x987ff0usize, b); // "  %s"
        let club = p.club_job();
        if (!club.is_null() && exe::human_manager_current_controls_club(club, 0, 1) != 0)
            || (!contract.is_null() && exe::human_manager_current_controls_club(club_at(contract.club() as isize), 0, 1) != 0)
        {
            st.button(5, 0);
        } else {
            st.value(JOB_TEXT);
        }
        st.row += 1;
    }

    let second = p.second_nation();
    if !second.is_null() && second != p.nation() {
        covered!("tel08_3 transfer tab second nationality");
        if read_u32(G_DATABASE_NAMES_LANGUAGE) == 0 {
            staff_tab_strcpy(JOB_TEXT, 0xa8ae20); // "  2nd Nationality"
        } else {
            lang(JOB_TEXT, 0xa8ae08usize); // "  Second Nationality"
        }
        st.label(JOB_TEXT);
        format(JOB_TEXT, 0x987ff0usize, second.addr() + nation::NATIONALITY); // "  %s"
        st.button(0x7d5, exe::database_get_national_team(second));
        st.row += 1;
    }

    lang(JOB_TEXT, 0xa8adf4usize); // "  Fluent Languages"
    st.label(JOB_TEXT);
    let mut list = [0u8; 0x3e8];
    let l = list.as_mut_ptr() as usize;
    lang(l, 0xa8add0usize, TRANSFER_TAB_COMMENT); // "  None<%s - COMMENT - no languages>"
    transfer_tab_languages(p, b, l, 8, 0x7f);
    st.value(l);
    st.row += 1;
    list[0] = 0;
    transfer_tab_languages(p, b, l, 4, 7);
    if list[0] != 0 {
        covered!("tel08_3 transfer tab basic languages");
        lang(JOB_TEXT, 0xa8ad70usize); // "  Basic Languages"
        st.label(JOB_TEXT);
        st.value(l);
        st.row += 1;
    }
}

/// The Offers section: "None" (no bids) when the person is not under offer; otherwise the future transfer ("<club> -
/// transfer arranged for <date>", the club linked) or each offer of the list in screen parameter 3 that the viewer may
/// see, then a trial offer of the viewer's club, and "None" (no offers) when nothing was written. Asks for the future
/// transfer into the state.
pub(crate) unsafe fn transfer_tab_offers(st: &mut TransferTab, offers: usize) {
    let lang = staff_tab_lang();
    let p = st.p;
    let tm = read_usize(G_TRANSFER_MANAGER);
    let mut buf = [0u8; 0x200];
    let b = buf.as_mut_ptr() as usize;
    lang(JOB_TEXT, 0xa8ad64usize); // "  Offers"
    st.label(JOB_TEXT);
    let viewer = st.viewer;
    st.first = st.row;
    let shortlisted = !viewer.is_null() && exe::scout_manager_is_player_shortlisted(read_usize(G_SCOUT_MANAGER), p, viewer.manager().addr()) != 0;
    if exe::transfer_manager_is_player_under_offer(tm, p, 1) == 0 {
        lang(JOB_TEXT, 0xa8ad44usize, TRANSFER_TAB_COMMENT); // "  None<%s - COMMENT - no bids>"
        st.value(JOB_TEXT);
        st.row += 1;
        return;
    }
    if exe::transfer_manager_get_future_transfer(tm, p, &mut st.future as *mut usize as u32) != 0 {
        covered!("tel08_3 transfer tab future transfer");
        let slot = exe::transfer_manager_get_offer_slot(tm, read_i32(st.future + future_transfer::OFFER_SLOT));
        if slot == 0 {
            tel26_error_box(TRANSFER_TAB_SOURCE, 0x1902);
        } else {
            exe::date_get_long_date_string(st.future, b, 0xc8, 0xff);
            let offer = Offer(read_usize(slot));
            let club = if read_i32(p.addr()) == (offer.target_staff_id() as i32) {
                club_at((offer.buyer_club_id() as i32) as isize)
            } else {
                let (_, club) = contract_tab_lookup(staff_at((offer.target_staff_id() as i32) as isize));
                Club(club)
            };
            let g = (club.short_gender_name() as i8) as i32;
            // "  {}<%s - Club Name(e.g.Chelsea)>{} - transfer arranged for {}<%s - Date(e.g.10th November 1998)>."
            lang(JOB_TEXT, 0xa8ace0usize, g, g, 100u32, club.addr() + club::SHORT_NAME, b);
            st.button(0x7d5, club.addr());
            st.row += 1;
        }
    } else if offers != 0 && read_u16(offers) != 0 {
        transfer_tab_offer_list(st, offers, shortlisted, b);
    }
    if !viewer.is_null() {
        let o = exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), read_u32(p.addr()), read_u32(viewer.addr()));
        if o != 0 && read_u8(o + contract_offer::CONTRACT_TYPE) & 0x3f == 5 {
            covered!("tel08_3 transfer tab trial offer");
            let g = (viewer.short_gender_name() as i8) as i32;
            lang(JOB_TEXT, 0xa8ab84usize, g, g, viewer.addr() + club::SHORT_NAME); // "  {}<%s - Club(e.g.Arsenal)>{} - trial offer"
            st.button(0x7d5, viewer.addr());
            st.row += 1;
        }
    }
    if st.row == st.first {
        lang(JOB_TEXT, 0xa8ab60usize, TRANSFER_TAB_COMMENT); // "  None<%s - COMMENT - no offers>"
        st.value(JOB_TEXT);
        st.row += 1;
    }
}

/// The Interested section: the clubs the shortlist manager finds interested (`collect_interested_clubs` into the
/// state's list), each a button 0x7d5 with its short name, or "None" (no clubs interested).
pub(crate) unsafe fn transfer_tab_interested(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let format = staff_tab_format();
    lang(JOB_TEXT, 0xa8ab30usize, TRANSFER_TAB_COMMENT); // "  Interested<%s - COMMENT - clubs interested>"
    st.label(JOB_TEXT);
    let shortlists = read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856);
    exe::shortlist_manager_collect_interested_clubs(shortlists, st.p, st.list, 1, 1, 0);
    if read_u16(st.list) == 0 {
        lang(JOB_TEXT, 0xa8ab04usize, TRANSFER_TAB_COMMENT); // "  None<%s - COMMENT - no clubs interested>"
        st.value(JOB_TEXT);
        st.row += 1;
        return;
    }
    covered!("tel08_3 transfer tab interested");
    write_u32(st.list + 0xa, read_u32(st.list + 2));
    st.first = st.row;
    let mut node = exe::llist_get_next(st.list);
    while node != 0 {
        st.pad();
        let club = club_at(read_i32(node + 4) as isize);
        format(JOB_TEXT, 0x987ff0usize, club.addr() + club::SHORT_NAME); // "  %s"
        st.button(0x7d5, club.addr());
        st.row += 1;
        node = exe::llist_get_next(st.list);
    }
}

/// The Future section: the contract offer of the viewer's club being considered (a grey line), "Retired from
/// international football", then either the retirement (retired, no future at under 30, retiring from the playing
/// staff or at the end of the season) or, without a contract, "-", or the person's mood comments
/// (`transfer_tab_mood`) and "Unknown" when nothing was written.
pub(crate) unsafe fn transfer_tab_future(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    lang(JOB_TEXT, 0xa8aaf8usize); // "  Future"
    st.label(JOB_TEXT);
    st.first = st.row;
    let p = st.p;
    let viewer = st.viewer;
    if !viewer.is_null() {
        let (_, club) = contract_tab_lookup(p);
        if viewer.addr() == club {
            let o = exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), read_u32(p.addr()), read_u32(viewer.addr()));
            if o != 0 && read_u8(o + contract_offer::CONTRACT_TYPE) & 0x3f == 2 && read_u8(o + contract_offer::STATE) == 0 {
                covered!("tel08_3 transfer tab considering offer");
                let g = (viewer.short_gender_name() as i8) as i32;
                // "  Currently considering new contract offer from {}<%s - Club Name(e.g.Falkirk)>{}"
                lang(JOB_TEXT, 0xa8aaa4usize, g, g, viewer.addr() + club::NAME);
                st.line_in(G_GUI_COLOUR_PALE_GREY);
            }
        }
    }
    if exe::nationalteams_player_dislikes_national_team(read_usize(G_NATIONAL_TEAMS), p) != 0 && !p.player().is_null() {
        st.say(0xa8aa7c); // "  Retired from international football"
    }
    let regen = read_usize(G_PLAYER_REGEN);
    if exe::player_regen_get_retirement_status(regen, p) == 2 {
        st.say(0xa8aa54); // "  Retired from professional football"
        return;
    }
    if exe::player_regen_get_retirement_status(regen, p) == 1 {
        covered!("tel08_3 transfer tab retiring");
        let s = if (p.age() as i8) < 0x1e {
            0xa8aa1c // "  Believes he has no future in professional football"
        } else if !p.player().is_null() && !p.non_player().is_null() {
            0xa8a9e0 // "  Retiring from the playing staff at the end of the season"
        } else {
            0xa8a9bc // "  Retiring at the end of the season"
        };
        st.say(s);
        return;
    }
    if st.contract.is_null() {
        st.say(0x98e8c8); // "  -"
        return;
    }
    covered!("tel08_3 transfer tab mood");
    transfer_tab_mood(st);
    let club = st.club;
    let pl = p.player();
    if !club.is_null()
        && !pl.is_null()
        && st.future == 0
        && p.ambition() > 0xf
        && p.loyalty() < 6
        && p.professionalism() < 6
        && club.reputation() < 0x1676
        && pl.current_reputation() > 0xcb2
        && pl.current_reputation() as i32 > club.reputation() as i32 + 0x2ee
    {
        // "  Sure he will soon move to a bigger club<%s - COMMENT - Boasting about moving to a bigger club>"
        lang(JOB_TEXT, 0xa86a8cusize, TRANSFER_TAB_COMMENT);
        st.line();
    }
    if st.row == st.first {
        lang(JOB_TEXT, 0xa86a68usize, TRANSFER_TAB_COMMENT); // "  Unknown<%s - COMMENT - Unknown>"
        st.line();
    }
}

/// The Potential Fees section: the percentage of the next transfer fee, the first team appearance clause and the
/// international appearance clause (after the first, or after N appearances with the count so far), when any exists.
pub(crate) unsafe fn transfer_tab_potential_fees(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let tm = read_usize(G_TRANSFER_MANAGER);
    let percentage = find_next_sale_percentage(tm, p.addr());
    let league = find_first_team_appearance_money(tm, p.addr());
    let international = find_international_appearance_money(tm, p.addr());
    if percentage == 0 && league == 0 && international == 0 {
        return;
    }
    covered!("tel08_3 transfer tab potential fees");
    lang(JOB_TEXT, 0xa86a54usize); // "  Potential Fees"
    st.label(JOB_TEXT);
    let first = st.row;
    if percentage != 0 {
        lang(JOB_TEXT, 0xa86a20usize, read_i8(percentage) as i32); // "  <%d - Percentage(e.g.25)>%% of next transfer fee"
        st.wrapped(G_GUI_COLOUR_YELLOW);
        st.row += 1;
    }
    let mut buf = [0u8; 0xc8];
    for (clause, kind, once, many) in [(league, 2, 0xa869e4usize, 0xa86974usize), (international, 4, 0xa86938, 0xa868c8)] {
        if clause == 0 {
            continue;
        }
        let b = transfer_tab_cash(read_i32(clause + appearance_clause::AMOUNT), &mut buf);
        if st.row != first {
            st.label(0);
        }
        let games = read_i8(clause + appearance_clause::GAMES);
        if games == 1 {
            lang(JOB_TEXT, once, b); // "  <%s - Cash> after first league appearance" / "... first international cap"
        } else {
            let so_far = get_clause_count(tm, p.addr(), kind) as i16 as i32;
            lang(JOB_TEXT, many, b, games as i32, so_far); // "  <%s> after <%d> league appearances (<%d> so far)" / "... international caps"
        }
        st.wrapped(G_GUI_COLOUR_YELLOW);
        st.row += 1;
    }
}

/// `Display::add_item` (0x55f990) of the History tab's View and Filter popups: a menu line with the text in the
/// screens' buffer, flags 0x2002 when it is the line in use (2 otherwise), command `id` with `value`, in popup area
/// `parent` (the whole eax the area call returned, as the original pushes it).
pub(crate) unsafe fn history_tab_menu_line(in_use: bool, id: u32, value: u32, parent: u32) {
    tel26_add_item(current_human_display(), [if in_use { 0x2002 } else { 2 }, 0, 0, 0, 0, 0, 0, 0x10, job_colour(JOB_COLOUR_31AA),
        job_colour(G_GUI_COLOUR_YELLOW), 1, 1, job_colour(G_GUI_COLOUR_BLACK), JOB_TEXT as u32, 0, id, value, parent]);
}

/// A View popup line (command 0x14, `bit` of screen parameter 5): the line in use also becomes the tab's title.
pub(crate) unsafe fn history_tab_view_line(view: u32, bit: u32, parent: u32, title: &mut [u8; 200]) {
    let in_use = view & bit != 0;
    history_tab_menu_line(in_use, 0x14, bit, parent);
    if in_use {
        staff_tab_strcpy(title.as_mut_ptr() as usize, JOB_TEXT);
    }
}

/// The Playing Career view: the seasons of screen parameter 6 (0x102-byte rows from +0xd9; screen parameter 7 is
/// six times their number, 8 the season in use), the career totals and empty rows to the end.
pub(crate) unsafe fn history_tab_playing_career(p: Staff, top: i32, bottom: i32, kind: u32, second_view: usize) {
    let weights_head: [u8; 12] = [0x33, 5, 5, 8, 8, 8, 8, 8, 8, 8, 8, 8];
    let weights_body: [u8; 14] = [0xd, 0x1a, 0xc, 5, 5, 8, 8, 8, 8, 8, 8, 8, 8, 8];
    let career = get_screen_param(6);
    let size = get_screen_param(7) as i32;
    let in_use = get_screen_param(8) as i32;
    let mut totals = [0i32; 19];
    let t = totals.as_mut_ptr() as usize;
    let add = |i: usize, v: i32| write_i32(t + 4 * i, read_i32(t + 4 * i).wrapping_add(v));
    let mut loaned_apps: i32 = 0;
    let span = bottom - top;
    let full = (span / exe::font_get_height(2)) as u8;
    let row_height = span / full as i32;
    let rows = full.wrapping_sub(1);
    let rows_dword = (kind & !0xff) | rows as u32;
    let d = current_human_display();
    let x2 = if size / 6 > rows as i32 { 0x2f8 } else { 0x30c };
    let head = tel26_add_area(d, [0x6e, top as u32, x2, (top + row_height) as u32, 0xc, weights_head.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    let body = tel26_add_area(d, [0x6e, (top + row_height) as u32, 0x30c, bottom as u32, 0xe, weights_body.as_ptr() as u32, rows_dword, 0, 1, 0, u32::MAX]);
    let mut row: i32 = 0;
    staff_tab_cell(head, 0, 0, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, 0);
    // the previous and next statistics views: the symbol font's arrows or "<<" and ">>"
    for (col, ch, plain, id) in [(1u32, 0x2au32, 0x98e7c4usize, 0x3eu32), (2, 0x2b, 0x98e7a8, 0x3f)] {
        if read_i32(G_FONT_MODE) != -1 {
            staff_tab_format()(JOB_TEXT, 0x99ec1cusize, ch); // "%c"
            staff_tab_button(head, col, row, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 0, G_GUI_COLOUR_PALE_GREY, JOB_TEXT, id, 0);
        } else {
            staff_tab_button(head, col, row, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, G_GUI_COLOUR_PALE_GREY, plain, id, 0);
        }
    }
    let codes = history_tab_columns(p, second_view);
    let mut k = 0;
    while read_u8(codes + k) != 0xff {
        history_tab_stat_title(read_u8(codes + k));
        staff_tab_cell(head, k as u32 + 3, row, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
        k += 1;
    }
    let mut index: i32 = 0;
    if size > 0 {
        let mut e = career + 0xd9;
        loop {
            staff_tab_cell(body, 3, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, 0);
            staff_tab_cell(body, 4, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, 0);
            let id = read_i32(e);
            let club = if id > -1 {
                let club = club_at(id as isize).addr();
                staff_tab_format()(JOB_TEXT, 0x987ff0usize, club + 0x38); // "  %s"
                staff_tab_button(body, 1, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, JOB_TEXT, 0x7d5, club as u32);
                club
            } else if id == -2 {
                staff_tab_lang()(JOB_TEXT, 0xa8b0b0usize); // "  Unknown Club"
                staff_tab_cell(body, 1, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
                0
            } else {
                staff_tab_cell(body, 1, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, 0);
                0
            };
            add(0, read_u8(e + 4) as i32);
            add(2, read_u8(e + 6) as i32);
            // the season text (key_nation 0x669630): the year word pushed in a register whose high half still holds
            // the second total
            exe::key_nation_format_club_season((read_u32(t + 8) & 0xffff_0000) | read_u16(e - 2) as u32, club, JOB_TEXT, 2000);
            if read_i16(e + 0x12) >= 0 {
                if index != in_use {
                    staff_tab_button(body, 0, row, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT, 0x15, index as u32);
                } else {
                    staff_tab_cell(body, 0, row, 0x30, job_colour(G_GUI_COLOUR_PALE_GREY), 0xc, 1, G_GUI_COLOUR_DARK_BLUE, JOB_TEXT);
                }
                // the season's byte statistics +5..+0x11 into the totals 1..13, the signed word +0x12 into 14, the
                // words +0x14..+0x1e into the words of 15..17 and the dword +0x20 into 18
                for b in 1..14usize {
                    if b != 2 {
                        add(b, read_u8(e + 4 + b) as i32);
                    }
                }
                add(14, read_i16(e + 0x12) as i32);
                for w in 0..6usize {
                    let a = t + 0x3c + 2 * w;
                    write_i16(a, read_i16(a).wrapping_add(read_i16(e + 0x14 + 2 * w)));
                }
                add(18, read_i32(e + 0x20));
            } else {
                covered!("tel09_2 history tab loan season");
                staff_tab_cell(body, 0, row, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
                loaned_apps += read_u8(e + 4) as i32;
            }
            let mut fee = [0u8; 200];
            history_tab_fee_text(e, &mut fee);
            staff_tab_cell(body, 2, row, 2, 0, 0xc, 1, HISTORY_TAB_COLOUR_3152, fee.as_ptr() as usize);
            let codes = history_tab_columns(p, second_view);
            let mut k = 0;
            while read_u8(codes + k) != 0xff {
                let code = read_u8(codes + k);
                if read_i16(e + 0x12) < 0 {
                    if k >= 2 {
                        write_u8(JOB_TEXT, 0);
                    } else {
                        history_tab_stat_season(e + 4, code, 0);
                    }
                } else {
                    history_tab_stat_season(e + 4, code, 1);
                }
                history_tab_stat_cell(body, k as u32 + 5, row, code, G_GUI_COLOUR_YELLOW);
                k += 1;
            }
            index += 6;
            row += 1;
            e += 0x102;
            if index >= size {
                break;
            }
        }
    }
    staff_tab_cell(body, 0, row, 2, 0, 0xc, 2, G_GUI_COLOUR_PALE_GREY, 0);
    staff_tab_lang()(JOB_TEXT, 0xa8b098usize); // "  Total"
    staff_tab_cell(body, 1, row, 2, 0, 1, 2, HISTORY_TAB_COLOUR_3152, JOB_TEXT);
    for col in 2..5 {
        staff_tab_cell(body, col, row, 2, 0, 1, 2, HISTORY_TAB_COLOUR_3152, 0);
    }
    let codes = history_tab_columns(p, second_view);
    let mut k = 0;
    while read_u8(codes + k) != 0xff {
        let code = read_u8(codes + k);
        if code == 0x11 {
            add(0, loaned_apps.wrapping_neg());
        }
        if read_i32(t + 0x38) < 0 && k >= 2 {
            write_u8(JOB_TEXT, 0);
        } else {
            history_tab_stat_total(t, code);
        }
        history_tab_stat_cell(body, k as u32 + 5, row, code, HISTORY_TAB_COLOUR_3152);
        k += 1;
    }
    row += 1;
    while row < rows as i32 {
        history_tab_blank_row(body, row, 14);
        row += 1;
    }
    history_tab_keep_scroll(body);
}

/// The Injuries view: the person's injury history (`Injury` 0x62ec20 on `G_INJURY`), one row per injury.
pub(crate) unsafe fn history_tab_injuries(p: Staff, top: i32, bottom: i32, kind: u32) {
    let weights: [u8; 4] = [3, 0xb, 6, 6];
    let rows = ((bottom - top) / exe::font_get_height(2)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 4, weights.as_ptr() as u32, (kind & !0xff) | rows as u32, 0, 1, 0, u32::MAX]);
    let mut row: i32 = 0;
    let history: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_HISTORY);
    let list = history(read_usize(G_INJURY), p.addr());
    if list != 0 {
        write_u32(list + 0xa, read_u32(list + 6));
        let mut e = exe::llist_get_prev(list);
        while e != 0 {
            if row == 0 {
                staff_tab_cell(area, 0, 0, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, 0);
                staff_tab_cell(area, 1, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, 0);
                staff_tab_lang()(JOB_TEXT, 0xa8b074usize, 0xad9c64usize); // "Type<%s - COMMENT - injury type>"
                staff_tab_cell(area, 2, row, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
                staff_tab_lang()(JOB_TEXT, 0xa8b068usize); // "Period Out"
                staff_tab_cell(area, 3, row, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
                row += 1;
            }
            history_tab_date(e + 0x20, JOB_TEXT);
            staff_tab_cell(area, 0, row, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
            let mut name = [0u8; 200];
            exe::injury_get_type_name(read_usize(G_INJURY), read_u8(e) as u32, name.as_mut_ptr() as usize, 200, 0);
            history_tab_capitalised(&name);
            staff_tab_cell(area, 1, row, 2, 0, 1, 2, G_GUI_COLOUR_YELLOW, JOB_TEXT);
            if read_u8(e + 0x1a) != 0xff {
                staff_tab_lang()(JOB_TEXT, 0xa8b03cusize, 0xad9c64usize); // "Match<%s - COMMENT - match injury>"
            } else {
                staff_tab_lang()(JOB_TEXT, 0x98e834usize); // "Training"
            }
            staff_tab_cell(area, 2, row, 2, 0, 0xc, 2, JOB_COLOUR_BDD4, JOB_TEXT);
            exe::format_duration_text(read_i16(e + 0x28) as i32, JOB_TEXT, 2000, 0);
            staff_tab_cell(area, 3, row, 2, 0, 0xc, 2, JOB_COLOUR_BDD4, JOB_TEXT);
            row += 1;
            e = exe::llist_get_prev(list);
        }
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
    }
    while row < rows as i32 {
        history_tab_blank_row(area, row, 4);
        row += 1;
    }
    history_tab_keep_scroll(area);
}

/// The Bans view: the person's ban exe::discipline_get_ban_history(`Discipline` 0x55cbc0 on `G_DISCIPLINE`, by the person's id), one row
/// per ban.
pub(crate) unsafe fn history_tab_bans(p: Staff, top: i32, bottom: i32, kind: u32) {
    let weights: [u8; 3] = [3, 0x11, 6];
    let rows = ((bottom - top) / exe::font_get_height(2)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 3, weights.as_ptr() as u32, (kind & !0xff) | rows as u32, 0, 1, 0, u32::MAX]);
    let mut row: i32 = 0;
    let list = exe::discipline_get_ban_history(read_usize(G_DISCIPLINE), read_u32(p.addr()));
    if list != 0 {
        write_u32(list + 0xa, read_u32(list + 6));
        let mut e = exe::llist_get_prev(list);
        while e != 0 {
            if row == 0 {
                staff_tab_cell(area, 0, 0, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, 0);
                staff_tab_cell(area, 1, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, 0);
                staff_tab_lang()(JOB_TEXT, 0xa7e1b4usize); // "Reason"
                staff_tab_cell(area, 2, row, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
                row += 1;
            }
            history_tab_date(e + 4, JOB_TEXT);
            staff_tab_cell(area, 0, row, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
            let mut name = [0u8; 200];
            exe::discipline_describe_ban(read_usize(G_DISCIPLINE), e, name.as_mut_ptr() as usize, 200);
            history_tab_capitalised(&name);
            staff_tab_cell(area, 1, row, 2, 0, 1, 2, G_GUI_COLOUR_YELLOW, JOB_TEXT);
            exe::discipline_get_reason_text(read_usize(G_DISCIPLINE), e, JOB_TEXT, 2000);
            staff_tab_cell(area, 2, row, 2, 0, 0xc, 2, JOB_COLOUR_BDD4, JOB_TEXT);
            row += 1;
            e = exe::llist_get_prev(list);
        }
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
    }
    while row < rows as i32 {
        history_tab_blank_row(area, row, 3);
        row += 1;
    }
    history_tab_keep_scroll(area);
}

/// The Manager Stats view: the manager record (`Staff_Records::get_manager_record` with 1) as label and value rows.
pub(crate) unsafe fn history_tab_manager_stats(p: Staff, top: i32, bottom: i32, kind: u32) {
    let weights: [u8; 2] = [1, 3];
    let rows = ((bottom - top) / exe::font_get_height(2)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 2, weights.as_ptr() as u32, (kind & !0xff) | rows as u32, 0, 1, 0, u32::MAX]);
    let mut row: i32 = 0;
    let r = exe::staff_records_get_manager_record(read_usize(G_STAFF_RECORDS), p, 1);
    if r != 0 {
        covered!("tel09_2 history tab manager stats");
        history_tab_stat_row(area, &mut row, 0xa8b02c, || history_tab_number(read_i16(r + 0x8c) as i32)); // "  Leagues won"
        history_tab_stat_row(area, &mut row, 0xa8b020, || history_tab_number(read_i16(r + 0x8a) as i32)); // "  Cups won"
        history_tab_stat_row(area, &mut row, 0xa8b010, || history_tab_number(read_i16(r + 0x9e) as i32)); // "  Awards won"
        history_tab_stat_row(area, &mut row, 0xa8affc, || history_tab_number(read_i32(r + 0xba))); // "  Manager points"
        history_tab_stat_row(area, &mut row, 0xa8afec, || history_tab_number(read_i16(r + 0x8e) as i32)); // "  Games played"
        history_tab_stat_row(area, &mut row, 0xa8afe0, || history_tab_number(read_i16(r + 0x90) as i32)); // "  Games won"
        history_tab_stat_row(area, &mut row, 0xa8afd0, || history_tab_number(read_i16(r + 0x92) as i32)); // "  Games drawn"
        history_tab_stat_row(area, &mut row, 0xa8afc0, || history_tab_number(read_i16(r + 0x94) as i32)); // "  Games lost"
        history_tab_stat_row(area, &mut row, 0xa8afb0, || history_tab_number(read_i32(r + 0x96))); // "  Goals scored"
        history_tab_stat_row(area, &mut row, 0xa8af9c, || history_tab_number(read_i32(r + 0x9a))); // "  Goals conceded"
        // "  Players bought" / "  <%d - players bought> (total value <%s - total value>)"
        history_tab_stat_row(area, &mut row, 0xa8af88, || history_tab_deals(read_i16(r + 0xa0), read_u32(r + 0xa4), 0xa8af4c));
        // "  Players sold" / "  <%d - players sold> (total value <%s - total value>)"
        history_tab_stat_row(area, &mut row, 0xa8af3c, || history_tab_deals(read_i16(r + 0xa2), read_u32(r + 0xac), 0xa8af04));
    }
    while row < rows as i32 {
        history_tab_blank_row(area, row, 2);
        row += 1;
    }
    history_tab_keep_scroll(area);
}

/// The Filter view (the history records): the records of the kind in use (`Staff_Records` 0x86de70), a page of them
/// as date, club and the wrapped record text, and empty rows to the end.
pub(crate) unsafe fn history_tab_records(top: i32, bottom: i32, kind: u32, list: usize, page: usize, t: *mut usize) {
    let weights: [u8; 3] = [3, 5, 0xf];
    let rows = ((bottom - top) / exe::font_get_height(2)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 3, weights.as_ptr() as u32, (kind & !0xff) | rows as u32, 0, 1, 0, u32::MAX]);
    let mut row: i32 = 0;
    if list != 0 {
        covered!("tel09_2 history tab records");
        let n = read_i16(list);
        let width: i32 = if n > rows as i16 && n <= 100 { 0x2f6 } else { 0x30c };
        let x1 = (width * 8 - 0x370) / 0x17 + 0x6e;
        let mut index = (page as i16 as i32 - 1) * rows as i32;
        write_u32(list + 0xa, read_u32(list + 6));
        for _ in 0..index.max(0) {
            exe::llist_get_prev(list);
        }
        while (row < rows as i32 || read_i16(list) < 100) && index < read_i16(list) as i32 {
            let node = exe::llist_get_prev(list);
            history_tab_date(node, JOB_TEXT);
            staff_tab_cell(area, 0, row, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
            let id = read_i32(node + 8);
            let club = if id > -1 { club_at(id as isize).addr() } else { 0 };
            if club != 0 {
                staff_tab_format()(JOB_TEXT, 0x987ff0usize, club + 0x38); // "  %s"
                staff_tab_button(area, 1, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, JOB_TEXT, 0x7d5, club as u32);
            } else {
                staff_tab_cell(area, 1, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, 0);
            }
            exe::staff_records_format_staff_history_entry(read_usize(G_STAFF_RECORDS), node, t);
            staff_tab_cell(area, 2, row, 2, 0, 1, 2, G_GUI_COLOUR_YELLOW, 0);
            staff_tab_draw_wrapped(t, area, x1 as u32, width as u32, &mut row, 2, G_GUI_COLOUR_YELLOW, 2, 0);
            row += 1;
            index += 1;
        }
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
    }
    while row < rows as i32 {
        history_tab_blank_row(area, row, 3);
        row += 1;
    }
    history_tab_keep_scroll(area);
}

/// The notice or confirm box (0x591840, cdecl): title, subtitle, text, kind (1 asks to confirm), the two kit colour
/// words and the command id the answer comes back with (`G_PENDING_COMMAND_CODE`).
pub(crate) unsafe fn profile_input_box(title: usize, subtitle: usize, text: usize, kind: u32, c1: u32, c2: u32, id: u32) {
    exe::confirm_push(title, subtitle, text, kind, c1, c2, id);
}

/// A notice box about the person (titled with his name `name`, kind 0): the text `fmt` with `args` into the screens'
/// buffer, the subtitle `subtitle` into `buf`, the colours and the command id.
pub(crate) unsafe fn profile_input_notice(name: usize, buf: usize, subtitle: usize, text: usize, c1: u32, c2: u32, id: u32) {
    let r = staff_tab_lang()(buf, subtitle, text);
    profile_input_box(name, r as u32 as usize, text, 0, c1, c2, id);
}

/// `TRANSFER_MANAGER` 0x8ce990 (thiscall, ret 0x10): whether the person can be approached by `club`; the reason code
/// goes to `out` (0 = he can, the reason is then the offer kind of 0x8f0e40).
pub(crate) unsafe fn profile_input_approach(p: Staff, club: u32, out: &mut u32) -> u32 {
    exe::transfer_manager_can_club_bid_for_player(read_usize(G_TRANSFER_MANAGER), p, (club) as u32 as usize, (out) as *mut u32 as *mut u16, 0)
}

/// A cdecl action of the profile on the person with up to three more dword arguments.
pub(crate) unsafe fn profile_input_action(f: usize, p: Staff, a: u32, b: u32, c: u32) {
    let g: unsafe extern "C" fn(usize, u32, u32, u32) = core::mem::transmute(f);
    g(p.addr(), a, b, c);
}

/// `Date` 0x549f70 (thiscall, ret 0x10): builds a date into `out` from (1, `a`, `b`, -1) and returns `out`.
pub(crate) unsafe fn profile_input_make_date(out: &mut [u16; 4], a: u32, b: u32) -> *const u16 {
    exe::date_date_549f70((out.as_mut_ptr()) as *mut u16 as *mut [u16; 4], 1, a, b, (-1) as i32 as u32) as *const u16
}

/// `langlib_sprintf(out, format, ...)` (0x66f4e0, cdecl): the language text of `format` into `out`.
pub(crate) unsafe fn tel36_langlib() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_LANGLIB_SPRINTF)
}

/// A cdecl screen opener by address with one argument.
pub(crate) unsafe fn staff_action1(f: usize, a: usize) {
    let g: unsafe extern "C" fn(usize) = core::mem::transmute(f);
    g(a)
}

/// A cdecl screen opener by address with two arguments.
pub(crate) unsafe fn staff_action2(f: usize, a: usize, b: usize) {
    let g: unsafe extern "C" fn(usize, usize) = core::mem::transmute(f);
    g(a, b)
}

/// A cdecl screen opener by address with three arguments.
pub(crate) unsafe fn staff_action3(f: usize, a: usize, b: usize, c: usize) {
    let g: unsafe extern "C" fn(usize, usize, usize) = core::mem::transmute(f);
    g(a, b, c)
}

/// A cdecl screen opener by address with four arguments.
pub(crate) unsafe fn staff_action4(f: usize, a: usize, b: usize, c: usize, d: usize) {
    let g: unsafe extern "C" fn(usize, usize, usize, usize) = core::mem::transmute(f);
    g(a, b, c, d)
}

/// `confirm_push(title, heading, message, yes/no, colour, colour, dialog code)` (0x591840, cdecl): a message box.
pub(crate) unsafe fn staff_confirm_push(title: usize, heading: usize, message: usize, yes_no: u32, a: u32, b: u32, code: u32) {
    exe::confirm_push(title, heading, message, yes_no, a, b, code)
}

/// The message box of a profile action that cannot be done: the message (already in the screens' text buffer, `message`
/// as the formatter answered), the heading `heading` (a language text into a local buffer), the person's name as the
/// title, OK only, the two colours and the dialog code.
pub(crate) unsafe fn staff_cannot_box(name: usize, heading: usize, message: usize, colours: (u32, u32), code: u32) {
    let mut buffer = [0u8; 0xc8];
    let h = tel36_langlib()(buffer.as_mut_ptr() as usize, heading) as u32 as usize;
    staff_confirm_push(name, h, message, 0, colours.0, colours.1, code);
}

/// `TRANSFER_MANAGER::can_club_bid_for_player(person, club, &reason, 0)` (0x8ce990, thiscall, ret 0x10).
pub(crate) unsafe fn staff_can_bid(person: usize, club: usize, reason: &mut u32) -> u32 {
    exe::transfer_manager_can_club_bid_for_player(read_usize(G_TRANSFER_MANAGER), person, club, (reason) as *mut u32 as *mut u16, 0)
}

/// `transfer_screens_open_cannot_sign_dialog(person, club, reason, kind)` (0x8f0e40, cdecl).
pub(crate) unsafe fn staff_cannot_sign(person: usize, club: usize, reason: u32, kind: usize) {
    staff_action4(0x8f0e40, person, club, reason as usize, kind)
}

/// The answer of the compare actions to a widget that is not an action: the profile tab answer, then the menu bar.
pub(crate) unsafe fn staff_compare_default(raw: u32) -> i32 {
    let r = staff_screens_handle_profile_tab_action(raw);
    if r != 0 {
        return r;
    }
    manager_menu_bar(raw)
}

/// `Display::add_area` on the current human's Display.
pub(crate) unsafe fn profile_area(x1: usize, y1: usize, x2: usize, y2: usize, cols: usize, align: usize, rows: usize, a8: usize, a9: usize, colour: usize, parent: usize) -> usize {
    let f: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    f(current_human_display(), x1, y1, x2, y2, cols, align, rows, a8, a9, colour, parent)
}

/// `Display::add_item` on the current human's Display, in the shape the profile uses: kind (2 text item, 0x2002 the
/// chosen one, 1 the separator), the flags, the three colours, the text, the item code and its data, the parent.
pub(crate) unsafe fn profile_item(kind: usize, flags: usize, c1: usize, c2: usize, c3: usize, text: usize, code: usize, data: usize, parent: usize) -> usize {
    let f: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    f(current_human_display(), kind, 0, 0, 0, 0, 0, 0, flags, c1, c2, 1, 1, c3, text, 0, code, data, parent)
}

/// `ScreenManager::add_button_item` (0x804030, thiscall, ret 0x30).
pub(crate) unsafe fn profile_button(x1: usize, y1: usize, x2: usize, y2: usize, a5: usize, c1: usize, a7: usize, a8: usize, c2: usize, text: usize, code: usize, data: usize) -> usize {
    let f: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    f(SCREEN_MANAGER, x1, y1, x2, y2, a5, c1, a7, a8, c2, text, code, data)
}

/// `ScreenManager::add_area_text` (0x804100, thiscall, ret 0x24): cell (`col`, `row`) of table `area`.
pub(crate) unsafe fn profile_cell(area: usize, col: usize, row: usize, a4: usize, a5: usize, a6: usize, a7: usize, colour: usize, text: usize) -> usize {
    let f: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    f(SCREEN_MANAGER, area, col, row, a4, a5, a6, a7, colour, text)
}

/// `ScreenManager::add_area_button` (0x8040a0, thiscall, ret 0x2c): a clickable cell with an item code and its data.
pub(crate) unsafe fn profile_cell_button(area: usize, col: usize, row: usize, a4: usize, a5: usize, a6: usize, a7: usize, colour: usize, text: usize, code: usize, data: usize) -> usize {
    let f: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    f(SCREEN_MANAGER, area, col, row, a4, a5, a6, a7, colour, text, code, data)
}

/// The separator line of a menu (`profile_item` kind 1, flags 0x1000010, no text).
pub(crate) unsafe fn profile_separator(parent: usize) {
    profile_item(1, 0x1000010, colour_word(0xae31aa), colour_word(G_GUI_COLOUR_YELLOW), colour_word(G_GUI_COLOUR_BLACK), 0, 0, 0, parent);
}

/// One entry of a profile menu (text in the shared buffer): the first one creates the "Action" button (kit colours,
/// top right) and its drop-down area (0x130); the entry goes into it with `code` and `data`. `menu` is −1 (low word)
/// until then.
pub(crate) unsafe fn profile_add_action(menu: &mut usize, fg: u32, bg: u32, action: usize, code: usize, data: usize) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if *menu as u16 == 0xffff {
        lang(action, 0xa8560c); // "Action"
        let b = profile_button(0x294, 4, 0x311, 0x18, 0x30, fg as usize, 0xc, 1, bg as usize, action, 0, 0);
        *menu = profile_area(0, 0, 0, 0, 0, 0, 0, 0, 0x130, colour_word(0xae31aa), b);
    }
    profile_item(2, 0x10, colour_word(0xae31aa), colour_word(G_GUI_COLOUR_YELLOW), colour_word(G_GUI_COLOUR_BLACK), PROFILE_BUF, code, data, *menu);
}

/// The CRT's inline strcpy of the original (the same bytes).
pub(crate) unsafe fn profile_strcpy(dst: usize, src: usize) {
    let mut i = 0;
    loop {
        let c = read_u8(src + i);
        write_u8(dst + i, c);
        if c == 0 {
            break;
        }
        i += 1;
    }
}

/// The title box of the profile: a red-bordered box (0x64, 0xa)-(0x316, 0x46) in the background colour, the Text
/// drawn in font 7, or 6 when the title is at least 0x2b2 wide in font 7, in the kit's foreground colour. The font 7
/// text's right edge `font7_x2` is the immediate of that push in the running exe (0x316, 0x440 in the 1920x1080 exe,
/// which patches only the font 7 draw: 0x870cce in the club heading, 0x870f57 in the national view); font 6 keeps 0x316.
pub(crate) unsafe fn profile_screen_title_box(st: &mut ProfileScreen, font7_x2: i32) {
    let draw: ProfileDrawFn = core::mem::transmute(FN_TEXT_DRAW_WRAPPED);
    let bg = st.bg as usize;
    profile_area(0x64, 0xa, 0x316, 0x46, 1, 0, 1, 0, 0x30, bg, usize::MAX);
    let font = if exe::font_text_width(7, PROFILE_BUF as *const u8) < 0x2b2 { 7 } else { 6 };
    let mut row: i32 = 0;
    let a = profile_area(0x64, 0x19, 0x316, 0x37, 1, 0, 1, 0, 1, bg, usize::MAX);
    let x2 = if font == 7 { font7_x2 as usize } else { 0x316 };
    draw(st.text.as_mut_ptr() as usize, a, 0x64, x2, &mut row, font, st.fg as usize, 3, 0);
}

/// The contract and transfer entries of the profile's Action menu (the viewer's own players and the others):
/// "Remove From / Add To Shortlist" (0xb) for a player the viewer does not control who is not on the board, not
/// retired and not seen by a viewer of classification 5; the squad moves (3) by `CONTRACT_MANAGER::get_squad_placement`
/// (1 "Promote To Seniors", 2 "Relegate To Reserves", 3 "Promote To 'A' Team", 4 "Relegate To 'B' Team"); for a
/// player of a club the viewer controls "Set Transfer Status" (5, jobs 0xb, 0xd, 0xf, not type 5); "Terminate Loan
/// Deal" (0x18) for one on loan at it; for a contract with a club the viewer controls (not type 5) "Offer New Contract"
/// (6, no future transfer, not queued for a free transfer, not leaving on a Bosman), "Terminate Contract" (0x1f, not a
/// player job), "Recall From Loan" (0x18, a loan not of type 7), "Discipline Player" (0x24, player jobs), "Give player
/// leave of absence" (0x2e, with a contract, not injured or AWOL, a grievance in 0x20060) and "Release On A Free" (9,
/// no loan, offers, exchange offers, future transfer or free transfer queue, jobs 0xb, 0xf, 0xd); with an offer of the
/// viewer's club, "Offer Contract" (0xc, accepted offer, no future transfer), "Withdraw Transfer Bid" (0xd) and
/// "Change Transfer Bid" (0xe, status 0 or 1, a contracted player, not a loan offer); "Offer Trial" (0x17) for a
/// clubless player; "Approach To Buy" (0x10) and "Approach To Loan" (0x16) for another club's player.
/// Returns whether the viewer may approach to sign (the contract's expiry band, or type 1, without the viewer's
/// control; or the FIFA rules with the protected period over).
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
pub(crate) unsafe fn profile_screen_contract_actions(st: &mut ProfileScreen, menu: &mut usize) -> bool {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let s = Staff(st.staff);
    let (fg, bg, act) = (st.fg, st.bg, st.action.as_mut_ptr() as usize);
    let cm = read_usize(G_CONTRACT_MANAGER);
    let tm = read_usize(G_TRANSFER_MANAGER);
    let add = |menu: &mut usize, text: usize, code: usize, data: usize| {
        lang(PROFILE_BUF, text);
        profile_add_action(menu, fg, bg, act, code, data);
    };
    // the offer of the viewer's club among the offers for the person
    let list = st.list;
    write_u32(list + 0xa, read_u32(list + 2));
    let mut node = exe::llist_get_next(list);
    while node != 0 {
        let slot = exe::transfer_manager_get_offer_slot(tm, read_i32(node));
        if st.viewer_club != 0 && read_i32(read_usize(slot) + offer::BUYER_CLUB_ID) == read_i32(st.viewer_club) {
            st.offer = slot;
        }
        node = exe::llist_get_next(list);
    }
    let own = profile_contract_club(s.addr()) != 0 && profile_controls(profile_contract_club(s.addr()));
    if !own
        && profile_not_board_member(s.addr())
        && st.viewer != 0
        && read_u8(st.viewer + staff::CLASSIFICATION) != 5
        && s.player().addr() != 0
        && exe::player_regen_get_retirement_status(read_usize(G_PLAYER_REGEN), s) != 2
    {
        // "Remove From Shortlist" is code 0xa (0x87129f), "Add To Shortlist" 0xb (0x871381)
        let (text, code) = if exe::scout_manager_is_player_shortlisted(read_usize(G_SCOUT_MANAGER), s, (0) as u32 as usize) != 0 { (0xa85614, 0xa) } else { (0xa855f8, 0xb) };
        add(menu, text, code, 0);
    }
    let placement = exe::contract_manager_get_squad_placement(cm, s, st.viewer_club);
    if (1..=4).contains(&placement) {
        let text = [0xa855e4, 0xa855cc, 0xa855b8, 0xa855a0][placement as usize - 1];
        add(menu, text, 3, 0);
    }
    let club = s.club_job().addr();
    let job = s.job_for_club();
    if ((club != 0 && profile_controls(club)) || (profile_contract_club(s.addr()) != 0 && profile_controls(profile_contract_club(s.addr()))))
        && !profile_contract_is(s.addr(), 5)
        && matches!(job, 0xb | 0xd | 0xf)
    {
        add(menu, 0xa8558c, 5, 0); // "Set Transfer Status"
    }
    if s.club_job().addr() != 0 && exe::contract_manager_get_loan_contract(cm, s) != 0 && profile_controls(s.club_job().addr()) {
        add(menu, 0xa85578, 0x18, 0); // "Terminate Loan Deal"
    }
    if profile_contract_club(s.addr()) != 0 && profile_controls(profile_contract_club(s.addr())) && !profile_contract_is(s.addr(), 5) {
        covered!("tel07_3 profile actions of an own contract");
        if exe::transfer_manager_get_future_transfer(tm, s, 0) == 0
            && exe::transfer_manager_is_queued_for_free_transfer(tm, s) == 0
            && (exe::contract_manager_get_contract(cm, s) == 0 || read_u8(exe::contract_manager_get_contract(cm, s) + contract::LEAVING_ON_BOSMAN) == 0)
        {
            add(menu, 0xa85564, 6, 0); // "Offer New Contract"
        }
        let job = s.job_for_club();
        if !matches!(job, 0xf | 0xd | 0xc | 0xb) {
            add(menu, 0x9b10f4, 0x1f, 0); // "Terminate Contract"
        }
        if exe::contract_manager_get_loan_contract(cm, s) != 0 && read_u8(exe::contract_manager_get_loan_contract(cm, s) + contract::CONTRACT_TYPE) & 0x3f != 7 {
            add(menu, 0xa85550, 0x18, 0); // "Recall From Loan"
        }
        let job = s.job_for_club();
        if matches!(job, 0xf | 0xd | 0xc | 0xb) {
            add(menu, 0x9cc0e8, 0x24, 0); // "Discipline Player"
        }
        let injury = exe::injury_get_player_injury(read_usize(G_INJURY), s);
        let c = st.contract;
        if c != 0
            && exe::injury_get_player_injury_status(read_usize(G_INJURY), s) != 2
            && exe::awol_is_player_awol(read_usize(G_AWOL), s, 0) == 0
            && injury == 0
            && read_u32(c + contract::GRIEVANCES) & 0x20060 != 0
        {
            add(menu, 0xa85530, 0x2e, 0); // "Give player leave of absence"
        }
        if exe::contract_manager_get_loan_contract(cm, s) == 0
            && exe::transfer_manager_get_offers_for_player(tm, s, 0, 0) == 0
            && exe::transfer_manager_get_exchange_offers_for_player(tm, s, 0) == 0
            && exe::transfer_manager_get_future_transfer(tm, s, 0) == 0
            && exe::transfer_manager_is_queued_for_free_transfer(tm, s) == 0
            && matches!(s.job_for_club(), 0xb | 0xf | 0xd)
        {
            add(menu, 0xa8551c, 9, 0); // "Release On A Free"
        }
    }
    let offer = st.offer;
    if offer != 0 {
        covered!("tel07_3 profile actions of an offer");
        if read_u8(read_usize(offer) + offer::STATUS) == 5 && exe::transfer_manager_get_future_transfer(tm, s, 0) == 0 {
            add(menu, 0x9aff10, 0xc, offer); // "Offer Contract"
        }
        add(menu, 0xa85504, 0xd, offer); // "Withdraw Transfer Bid"
        let status = read_u8(read_usize(offer) + offer::STATUS);
        if (status == 0 || status == 1) && profile_contract_club(s.addr()) != 0 && read_u8(read_usize(offer) + offer::OFFER_TYPE) != 1 {
            add(menu, 0xa854f0, 0xe, offer); // "Change Transfer Bid"
        }
    }
    if s.club_job().addr() == 0
        && st.viewer_club != 0
        && offer == 0
        && exe::transfer_manager_get_future_transfer(tm, s, 0) == 0
        && exe::contract_manager_find_contract_offer(cm, read_u32(s.addr()), read_u32(st.viewer_club)) == 0
        && profile_not_board_member(s.addr())
        && s.player().addr() != 0
        && exe::player_regen_get_retirement_status(read_usize(G_PLAYER_REGEN), s) != 2
        && st.viewer != 0
        && read_usize(st.viewer + staff::CLUB_JOB) != 0
    {
        add(menu, 0xa854e4, 0x17, 0); // "Offer Trial"
    }
    // approach to buy and to loan: another club's player (0x872620..0x872918)
    let c = st.contract;
    'buy: {
        if profile_contract_club(s.addr()) == 0 || profile_controls(profile_contract_club(s.addr())) {
            break 'buy;
        }
        if exe::transfer_manager_get_future_transfer(tm, s, 0) != 0 || offer != 0 || st.viewer_club == 0 {
            break 'buy;
        }
        if c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 1 && exe::nation_allows_free_transfers(read_usize(club_at(read_i32(c + contract::CLUB) as isize).addr() + club::NATION)) != 0 {
            break 'buy;
        }
        if profile_contract_is(s.addr(), 5) || exe::club_is_b_team(st.viewer_club) != 0 || s.player().addr() == 0 || !profile_not_board_member(s.addr()) {
            break 'buy;
        }
        if read_u8(profile_state(s.addr()) + staff_state::STATUS_FLAGS) & 0x30 == 0x20 || st.viewer == 0 || read_usize(st.viewer + staff::CLUB_JOB) == 0 {
            break 'buy;
        }
        covered!("tel07_3 profile approach to buy");
        // DEFECT (original): this entry does not open the Action button when it is the first one (parent −1)
        lang(PROFILE_BUF, 0xa854d4); // "Approach To Buy"
        profile_item(2, 0x10, colour_word(0xae31aa), colour_word(G_GUI_COLOUR_YELLOW), colour_word(G_GUI_COLOUR_BLACK), PROFILE_BUF, 0x10, 0, *menu);
        if s.job_for_club() != 0xc && exe::contract_manager_get_loan_contract(cm, s) == 0 && !profile_controls(s.club_job().addr()) {
            add(menu, 0xa854c0, 0x16, 0); // "Approach To Loan"
        }
    }
    let mut can_sign = false;
    if c != 0 && (exe::staff_contract_get_expiry_band(c) != 0 || read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 1) {
        can_sign = !profile_controls(profile_contract_club(s.addr()));
    }
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1
        && c != 0
        && exe::staff_contract_check_fifa_protected_period(c, st.viewer_club, 0, 0) == 0
        && !profile_controls(profile_contract_club(s.addr()))
    {
        can_sign = true;
    }
    can_sign
}

/// "Approach To Sign" (0xf): a viewer with a club; for a player of a club without a type-5 contract, only when he may
/// approach (`profile_screen_contract_actions`) and the club uses the Bosman rule; not for a contract with a club the
/// viewer controls (not type 5); only a person outside the 16 last records, not on the board, without an offer of the
/// viewer's club, not retired.
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
pub(crate) unsafe fn profile_screen_approach_to_sign(st: &mut ProfileScreen, menu: &mut usize, can_sign: bool) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let s = Staff(st.staff);
    if read_usize(st.viewer + staff::CLUB_JOB) == 0 {
        return;
    }
    if s.club_job().addr() != 0 && !profile_contract_is(s.addr(), 5) && s.player().addr() != 0 {
        if !can_sign || exe::transfer_manager_player_club_uses_bosman(read_usize(G_TRANSFER_MANAGER), s) == 0 {
            return;
        }
    }
    if profile_contract_club(s.addr()) != 0 && !profile_contract_is(s.addr(), 5) && profile_controls(profile_contract_club(s.addr())) {
        return;
    }
    if read_i32(s.addr()) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10) || !profile_not_board_member(s.addr()) || st.offer != 0 {
        return;
    }
    if exe::player_regen_get_retirement_status(read_usize(G_PLAYER_REGEN), s) == 2 {
        return;
    }
    lang(PROFILE_BUF, 0xa854ac); // "Approach To Sign"
    profile_add_action(menu, st.fg, st.bg, st.action.as_mut_ptr() as usize, 0xf, 0);
}

/// The national team entries of the Action menu, when the viewer may pick the person for his national team
/// (0x76d9b0 of `G_NATIONAL_TEAMS`): a separator (only when the menu exists), then by the squad the person is in
/// (0x76da80: 0 the squad, 1 the 'B' squad, 2 the under-21/23 squad, of the viewer's team's nation): "Remove From"
/// (0x1c) or "Move To / Call Up To" (0x1b), with the data 0 squad, 1 'B' squad, 2 youth squad; the 'B' squad only for a
/// team with one (`database_club_has_reserve_squad`) and a viewer controlling all national teams; the youth squad
/// only when its team has no manager and the viewer controls all national teams, under 23 (the nation's byte at
/// +0x36 of the national teams) for those under 24, else under 21 for those under 22. When the viewer's team is the
/// nation itself: only the youth squad (remove, or call up by age).
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
pub(crate) unsafe fn profile_screen_national_actions(st: &mut ProfileScreen, menu: &mut usize) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let controls_all_fn: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_ALL_NATIONAL_TEAMS);
    let s = Staff(st.staff);
    let viewer = st.viewer;
    let nts = read_usize(G_NATIONAL_TEAMS);
    if exe::national_teams_player_is_eligible(nts, s, viewer) == 0 {
        return;
    }
    covered!("tel07_3 profile national team actions");
    if *menu as u16 != 0xffff {
        profile_separator(*menu);
    }
    let (fg, bg, act) = (st.fg, st.bg, st.action.as_mut_ptr() as usize);
    let team = Club(st.viewer_nation_team);
    let nation = team.nation().addr();
    let g = (team.short_gender_name() as i8) as i32;
    let nm = team.addr() + club::SHORT_NAME;
    let mut add = |menu: &mut usize, text: usize, code: usize, data: usize| {
        lang(PROFILE_BUF, text, g, g, nm);
        profile_add_action(menu, fg, bg, act, code, data);
    };
    let controls_all = || controls_all_fn(read_usize(G_HUMAN_MANAGER), viewer, 0) != 0;
    let under23 = || read_u8(read_usize(read_usize(G_NATIONAL_TEAMS) + 0x36).wrapping_add(read_i32(nation) as usize)) != 0;
    let youth_free = || read_usize(exe::database_get_national_u21_team(nation) + club::MANAGER) == 0;
    let age = || s.age() as i8;
    // the youth squad entry of the first paths (0x872ec3, 0x8731f2, 0x873877): `texts` = (under 23, under 21)
    let youth = |menu: &mut usize, add: &mut dyn FnMut(&mut usize, usize, usize, usize), texts: (usize, usize)| {
        if !youth_free() || !controls_all() {
            return;
        }
        if under23() {
            if age() < 0x18 {
                add(menu, texts.0, 0x1b, 2);
            }
        } else if age() < 0x16 {
            add(menu, texts.1, 0x1b, 2);
        }
    };
    if exe::club_is_national_team(team) == 0 {
        if exe::national_teams_player_in_squad(nts, s, nation, 0) != 0 {
            add(menu, 0xa85474, 0x1c, 0); // Remove From Squad
            if exe::database_club_has_reserve_squad(team) != 0 && controls_all() {
                add(menu, 0xa8543c, 0x1b, 1); // Move To 'B' Squad
            }
            youth(menu, &mut add, (0xa85400, 0xa853c4)); // Move To Under 23 / 21
        } else if exe::national_teams_player_in_squad(nts, s, nation, 1) != 0 {
            add(menu, 0xa85390, 0x1b, 0); // Move To Squad
            if controls_all() {
                add(menu, 0xa85354, 0x1c, 1); // Remove From 'B' Squad
            }
            youth(menu, &mut add, (0xa85400, 0xa853c4));
        } else if exe::national_teams_player_in_squad(nts, s, nation, 2) != 0 {
            add(menu, 0xa85390, 0x1b, 0); // Move To Squad
            if !controls_all() {
                return;
            }
            if exe::database_club_has_reserve_squad(exe::database_get_national_team(nation)) != 0 {
                add(menu, 0xa8543c, 0x1b, 1); // Move To 'B' Squad
            }
            if youth_free() {
                add(menu, if under23() { 0xa85314 } else { 0xa852d4 }, 0x1c, 2); // Remove From Under 23 / 21
            }
        } else {
            add(menu, 0xa8529c, 0x1b, 0); // Call Up To Squad
            if exe::database_club_has_reserve_squad(team) != 0 && controls_all() {
                add(menu, 0xa85260, 0x1b, 1); // Call Up To 'B' Squad
            }
            youth(menu, &mut add, (0xa85220, 0xa851e0)); // Call Up To Under 23 / 21
        }
    } else {
        let squad = exe::national_teams_player_in_squad(nts, s, nation, 2) != 0;
        let u23 = under23();
        if squad {
            add(menu, if u23 { 0xa85314 } else { 0xa852d4 }, 0x1c, 2);
        } else if u23 {
            if age() < 0x18 {
                add(menu, 0xa85220, 0x1b, 2);
            }
        } else if age() < 0x16 {
            add(menu, 0xa851e0, 0x1b, 2);
        }
    }
}

/// The staff entries of the Action menu: "Scout Player" (0x29) for another club's player without a scout request;
/// "Edit Nickname" (0x25) when the viewer gave him one or he has a common name, else "Set Nickname" (0x26; always with
/// no viewer); for the viewer's own club (or its linked senior club for a B team): "Request Physio Report" (0x2b) with
/// physios, "Request Coach Report" (0x2a) with a coach other than him (the assistant or the first of the five
/// coaches), after a separator; "Send for Rehabilitation" (0x2d, after a separator) for one flagged for it
/// (`staff_state::EXTRA_FLAGS` bit 1), unless his injury is of category 0xc or 0x62ed30 answers 0xff, which clears the
/// flag instead; "Create / Edit Manager Note" (0x2c, after a separator) for a person of the database when the notes
/// exist. The separators here do not check that the menu exists.
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
pub(crate) unsafe fn profile_screen_staff_actions(st: &mut ProfileScreen, menu: &mut usize, viewer_known: bool) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let s = Staff(st.staff);
    let viewer = Staff(st.viewer);
    let (fg, bg, act) = (st.fg, st.bg, st.action.as_mut_ptr() as usize);
    let add = |menu: &mut usize, text: usize, code: usize| {
        lang(PROFILE_BUF, text);
        profile_add_action(menu, fg, bg, act, code, 0);
    };
    if viewer_known {
        let vclub = viewer.club_job().addr();
        if vclub != 0
            && (s.club_job().addr() == 0 || s.club_job().addr() != vclub)
            && profile_contract_club(s.addr()) != vclub
            && s.player().addr() != 0
            && exe::scout_manager_has_scout_request(read_usize(G_SCOUT_MANAGER), s, 0, 0) == 0
        {
            add(menu, 0xa851d0, 0x29); // "Scout Player"
        }
        let common = s.common_name();
        if exe::human_manager_has_nickname(read_usize(G_HUMAN_MANAGER), s, viewer.addr() as u32) != 0 || (common != 0 && read_u8(common) != 0) {
            add(menu, 0xa851c0, 0x25); // "Edit Nickname"
        } else {
            add(menu, 0xa851b0, 0x26); // "Set Nickname"
        }
    } else {
        add(menu, 0xa851b0, 0x26); // "Set Nickname"
    }
    let mut own = s.club_job().addr();
    if own != 0 {
        let mut senior_unlinked: u32 = st.list as u32;
        let b = exe::club_get_linked_b_team(own, &mut senior_unlinked, 1);
        if b != 0 && senior_unlinked == 0 {
            own = b;
        }
    }
    if s.player().addr() != 0 && own != 0 && !viewer.is_null() && viewer.club_job().addr() != 0 && own == viewer.club_job().addr() {
        covered!("tel07_3 profile reports of the own club");
        let mut coach = exe::database_get_club_assistant(own);
        if coach == s.addr() {
            coach = 0;
        }
        if coach == 0 {
            for k in 0..5 {
                let c = read_usize(own + club::COACHES + k * 4);
                if c != s.addr() && c != 0 {
                    coach = c;
                    break;
                }
            }
        }
        if coach != 0 || exe::database_count_club_physios(viewer.club_job().addr()) != 0 {
            profile_separator(*menu);
        }
        if exe::database_count_club_physios(viewer.club_job().addr()) != 0 {
            add(menu, 0xa6b2cc, 0x2b); // "Request Physio Report"
        }
        if coach != 0 {
            add(menu, 0x99b4d4, 0x2a); // "Request Coach Report"
        }
    }
    // the viewer is not checked here, as in the original
    let vclub = viewer.club_job().addr();
    if vclub != 0 && own == vclub && read_u8(profile_state(s.addr()) + staff_state::EXTRA_FLAGS) & 1 != 0 {
        let injury = exe::injury_get_player_injury(read_usize(G_INJURY), s);
        if (injury != 0 && exe::injury_get_type_category(read_usize(G_INJURY), read_i8(injury)) == 0xc) || exe::injury_get_recurring_injury_type(read_usize(G_INJURY), s) == 0xff {
            covered!("tel07_3 profile clears the rehabilitation flag");
            let e = profile_state(s.addr()) + staff_state::EXTRA_FLAGS;
            write_u8(e, read_u8(e) & 0xfe);
        } else {
            profile_separator(*menu);
            add(menu, 0xa85198, 0x2d); // "Send for Rehabilitation"
        }
    }
    if (s.is_null() || read_i32(s.addr()) < read_i32(G_STAFF_COUNT)) && read_usize(G_NOTES) != 0 {
        profile_separator(*menu);
        let text = if exe::notes_find_note(read_usize(G_NOTES), viewer, s) != 0 { 0xa85170 } else { 0xa85184 }; // Edit / Create Manager Note
        add(menu, text, 0x2c);
    }
}

/// The nationality of the profile into `out` (200 bytes): the adjective of his nation (`nation_get_nationality_adjective`
/// with 0xc, first letter upper-cased by the CRT `toupper` 0x9471cd) and, for a second nation that is not the first,
/// without international caps, without the "blocked by the second nation" flag (`staff_state::EXTRA_FLAGS` bit 2, for
/// a person of the database) and not Portugal (`KEY_NATION_PORTUGAL`), "/" and the second adjective (`extra` keeps it).
/// No nation: empty.
pub(crate) unsafe fn profile_screen_nationality(s: usize, out: &mut [u8; 0xc8], extra: &mut [u8; 0xc8]) {
    let s = Staff(s);
    let nation = s.nation().addr();
    if nation == 0 {
        out[0] = 0;
        return;
    }
    let second = s.second_nation().addr();
    let blocked = read_i32(s.addr()) < read_i32(G_STAFF_COUNT) && read_u8(profile_state(s.addr()) + staff_state::EXTRA_FLAGS) & 2 != 0;
    if second != nation && second != 0 && s.int_apps() == 0 && !blocked && read_i32(second) != read_i32(KEY_NATION_PORTUGAL) {
        exe::nation_get_nationality_adjective(nation, out.as_mut_ptr() as usize, 0xc8, 0xc);
        out[0] = exe::at_9471cd(out[0] as i8 as i32) as u8;
        exe::nation_get_nationality_adjective(second, extra.as_mut_ptr() as usize, 0xc8, 0xc);
        extra[0] = exe::at_9471cd(extra[0] as i8 as i32) as u8;
        profile_strcat(out.as_mut_ptr() as usize, 0x9b7630); // "/"
        profile_strcat(out.as_mut_ptr() as usize, extra.as_ptr() as usize);
    } else {
        exe::nation_get_nationality_adjective(nation, out.as_mut_ptr() as usize, 0xc8, 0xc);
        out[0] = exe::at_9471cd(out[0] as i8 as i32) as u8;
    }
}

/// `Text::clear` (0x4afa40, thiscall).
pub(crate) unsafe fn staff_tab_text_clear(text: *mut usize) {
    exe::text_clear(text);
}

/// The row `k` of the attribute table.
pub(crate) fn profile_row(k: usize) -> usize {
    PROFILE_TABLE + k * PROFILE_ROW
}

/// Balances `n` rows over `columns` columns one by one: the rows of each column and the tallest column (at least
/// `rows`, unsigned bytes like the original).
pub(crate) fn profile_columns(n: usize, columns: usize, rows: u8) -> ([u8; 3], u8) {
    let mut counts = [0u8; 3];
    let mut tallest = rows;
    let mut k = 0;
    for _ in 0..n {
        counts[k] = counts[k].wrapping_add(1);
        if counts[k] > tallest {
            tallest = counts[k];
        }
        k += 1;
        if k >= columns {
            k = 0;
        }
    }
    (counts, tallest)
}

/// Empty cells of the column pair (`col`, `col + 1`) of `area` from `*row` up to `tallest`.
pub(crate) unsafe fn profile_blank_rows(area: u32, col: u32, row: &mut i32, tallest: u8) {
    while *row < tallest as i32 {
        staff_tab_cell(area, col, *row, 2, 0, 0xc, 2, G_GUI_COLOUR_PALE_GREY, 0);
        staff_tab_cell(area, col + 1, *row, 2, 0, 0xc, 2, G_GUI_COLOUR_PALE_GREY, 0);
        *row += 1;
    }
}

/// The text of an extra row of the player attribute table into `out` (200 bytes): the preferred foot (left foot ≥
/// 15: "Left Only" when the right foot is under 8, "Left" under 14, else "Either"; otherwise "Right Only" when the
/// left foot is under 8, else "Right"), the form (`PlayerStats::format_recent_ratings`; "-" for an id past the person
/// count), the morale (`contract_manager_write_morale_text`) and the condition ("Injured" for an injury status 2,
/// else the condition `%d%%`). Other codes leave `out` as it is.
pub(crate) unsafe fn profile_extra_row_text(code: u8, p: Staff, out: usize) {
    let lang = staff_tab_lang();
    let format = staff_tab_format();
    match code {
        0xff => {
            let sheet = p.player();
            let left = sheet.left_foot();
            if left >= 15 {
                let right = sheet.right_foot();
                if right < 8 {
                    lang(out, 0xa86374usize); // "Left Only"
                } else if right < 14 {
                    lang(out, 0xa8634cusize, 0xad9c64usize); // "Left<%s - COMMENT - prefers left foot>"
                } else {
                    lang(out, 0xa86344usize); // "Either"
                }
            } else if left < 8 {
                lang(out, 0xa86338usize); // "Right Only"
            } else {
                lang(out, 0xa8630cusize, 0xad9c64usize); // "Right<%s - COMMENT - prefers right foot>"
            }
        }
        0xfe => {
            if !p.is_null() && read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) {
                format(out, 0x98ba0cusize); // "-"
            } else {
                exe::playerstats_format_recent_ratings(read_usize(G_PLAYER_STATS), p.player().addr(), out, 0xc8);
            }
        }
        0xfd => {
            let morale: unsafe extern "stdcall" fn(usize, usize, u32) = core::mem::transmute(FN_CONTRACT_MANAGER_WRITE_MORALE_TEXT);
            morale(p.addr(), out, 0xc8);
        }
        0xfc => {
            if exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2 {
                lang(out, 0xa86304usize); // "Injured"
            } else {
                let condition = exe::injury_get_condition(read_usize(G_INJURY), p, 1);
                format(out, 0x98e8d4usize, condition as i32); // "%d%%"
            }
        }
        _ => {}
    }
}

/// The chairman table of the Profile tab (three columns, weights 3 1 7, the whole tab): business acumen (> 15
/// astute, > 10 competent, > 5 average, else poor), discipline (> 15 harsh, > 10 strict, else average), interference
/// (> 15 very hands on, else quite reserved), man handling (`non_player_attribute_rating` of the man handling, asked
/// again for each threshold: > 15 good, > 3 average, else poor), patience (> 15 very patient, > 5 patient, else
/// impatient), resources (> 15 vastly wealthy, > 5 rich, else wealthy) and ambition (the person's: 20 with resources
/// > 15 incredibly ambitious, > 14 very, < 5 content, else ambitious); then empty rows. Returns the table's height
/// (rows × the font 2 height).
/// Original: part of `staff_screens_draw_profile_tab` (0x87a510, staff_screens.cpp).
pub(crate) unsafe fn profile_tab_chairman(top: i32, bottom: i32, p: Staff, weights: &[u8; 3]) -> i32 {
    let rows = staff_tab_rows(top, bottom, 2);
    let height = exe::font_get_height(2) * rows as i32;
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 3, weights.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    let np = p.non_player().addr();
    let a = |k: usize| read_i8(np + k);
    let mut row: i32 = 0;

    let v = a(non_player::BUSINESS);
    let s = if v > 15 { 0xa86260 } else if v > 10 { 0xa86224 } else if v > 5 { 0xa861e8 } else { 0xa861ac };
    profile_chairman_line(area, row, 0xa86290, v, s); // "  Business Acumen"
    row += 1;
    let v = a(non_player::DISCIPLINE);
    let s = if v > 15 { 0xa86168 } else if v > 10 { 0xa8612c } else { 0xa860ec };
    profile_chairman_line(area, row, 0xa8619c, v, s); // "  Discipline"
    row += 1;
    let v = a(non_player::INTERFERENCE);
    let s = if v > 15 { 0xa860a0 } else { 0xa8605c };
    profile_chairman_line(area, row, 0xa860dc, v, s); // "  Interference"
    row += 1;
    let rating = || exe::non_player_attribute_rating(np, a(non_player::MAN_HANDLING));
    let v = rating();
    let s = if rating() > 15 { 0xa86014 } else if rating() > 3 { 0xa85fe0 } else { 0xa85fa8 };
    profile_chairman_line(area, row, 0xa8604c, v, s); // "  Man Handling"
    row += 1;
    let v = a(non_player::PATIENCE);
    let s = if v > 15 { 0xa85f64 } else if v > 5 { 0xa85f38 } else { 0xa85f04 };
    profile_chairman_line(area, row, 0xa85f9c, v, s); // "  Patience"
    row += 1;
    let v = a(non_player::RESOURCES);
    let s = if v > 15 { 0xa85ec0 } else if v > 5 { 0xa85e98 } else { 0xa85e6c };
    profile_chairman_line(area, row, 0xa85ef8, v, s); // "  Resources"
    row += 1;
    let v = p.ambition();
    let s = if v == 20 && a(non_player::RESOURCES) > 15 {
        0xa85e1c
    } else if v > 14 {
        0xa85ddc
    } else if v < 5 {
        0xa85da0
    } else {
        0xa85d5c
    };
    profile_chairman_line(area, row, 0xa85e60, v, s); // "  Ambition"
    row += 1;
    while row < rows as i32 {
        staff_tab_cell(area, 0, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, 0xad9c64);
        staff_tab_cell(area, 1, row, 2, 0, 0xc, 2, JOB_COLOUR_BDD4, 0xad9c64);
        staff_tab_cell(area, 2, row, 2, 0, 0xc, 2, JOB_COLOUR_BDD4, 0xad9c64);
        row += 1;
    }
    height
}

/// The formation and style table of a non-player (two columns, weights 3 7) from `y` to `bottom`: the preferred
/// formation (1..9: 5-3-2 sweeper, 5-3-2, 4-4-2, 4-3-3, 4-2-4, 3-5-2, 4-1-2-1-2, 4-5-1, 3-4-3; else "-"), the
/// preferred style (`profile_style_text`) and "Other": pressing, offside trap and man-marking ≥ 15 each on its own
/// row ("-" for none); then empty rows.
/// Original: part of `staff_screens_draw_profile_tab` (0x87a510, staff_screens.cpp).
pub(crate) unsafe fn profile_tab_formation(y: i32, bottom: i32, p: Staff, weights: &[u8; 2]) {
    let lang = staff_tab_lang();
    let rows = staff_tab_rows(y, bottom, 2);
    let area = tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, bottom as u32, 2, weights.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    let np = p.non_player().addr();
    let a = |k: usize| read_i8(np + k);
    let label = |row: i32, s: usize| {
        staff_tab_cell(area, 0, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, s);
    };
    let value = |row: i32, s: usize| {
        staff_tab_cell(area, 1, row, 2, 0, 1, 2, G_GUI_COLOUR_YELLOW, s);
    };
    let mut row: i32 = 0;
    lang(JOB_TEXT, 0xa85d44usize); // "  Preferred Formation"
    label(row, JOB_TEXT);
    let formations = [0xa85d34usize, 0xa85d2c, 0xa85d24, 0xa85d1c, 0xa85d14, 0xa85d0c, 0xa85d00, 0xa85cf8, 0xa85cf0];
    match a(non_player::FORMATION) {
        f @ 1..=9 => {
            lang(JOB_TEXT, formations[f as usize - 1]);
        }
        _ => staff_tab_strcpy(JOB_TEXT, 0x98e8c8), // "  -"
    }
    value(row, JOB_TEXT);
    row += 1;
    lang(JOB_TEXT, 0xa85cdcusize); // "  Preferred Style"
    label(row, JOB_TEXT);
    profile_style_text(a(non_player::ATTACKING), a(non_player::DIRECTNESS));
    value(row, JOB_TEXT);
    row += 1;
    lang(JOB_TEXT, 0xa85b38usize, 0xad9c64usize); // "  Other<%s - COMMENTS - other playing styles>"
    label(row, JOB_TEXT);
    let start = row;
    if a(non_player::PRESSING) >= 15 {
        lang(JOB_TEXT, 0xa85b04usize); // "  Likes his players to close down the opposition"
        value(row, JOB_TEXT);
        row += 1;
    }
    for (field, s) in [(non_player::OFFSIDE, 0xa85ae0usize), (non_player::MARKING, 0xa85ab8)] {
        if a(field) >= 15 {
            if row > start {
                label(row, 0);
            }
            lang(JOB_TEXT, s); // the offside trap, the man-marking system
            value(row, JOB_TEXT);
            row += 1;
        }
    }
    if row == start {
        value(row, 0x98e8c8); // "  -"
        row += 1;
    }
    profile_blank_rows(area, 0, &mut row, rows);
}

/// The headers of the match history columns 7 to 12 for statistics view `kind` (0 general, 1 passing and shooting,
/// 2 tackles and headers, other discipline): each string and whether its format takes the comment argument.
pub(crate) fn profile_history_headers(kind: u8) -> [(usize, bool); 6] {
    match kind {
        0 => [(0xa78490, false), (0xa7845c, false), (0xa78464, true), (0x98e978, false), (0xa78440, false), (0xa78434, false)],
        1 => [(0x9b7964, false), (0xa161d0, false), (0xa85a98, true), (0xa169cc, true), (0xa169a8, true), (0xa78490, false)],
        2 => [(0xa161c8, false), (0xa16968, true), (0xa85a78, true), (0x9b7a28, false), (0xa169e8, true), (0xa85a58, true)],
        _ => [(0x990e40, false), (0xa161b0, false), (0xa161a8, false), (0xa161a4, false), (0xa16924, true), (0xa16944, true)],
    }
}

/// One statistic of a match history entry for column 7..12 and view `kind`: the byte offset and whether it is signed
/// (column 7 of the discipline view is written apart: penalties scored and taken).
pub(crate) fn profile_history_stat(kind: u8, k: usize) -> (usize, bool) {
    match kind {
        0 => [(0x10, true), (0x11, true), (0x12, true), (0x13, true), (0x14, true), (0x15, false)][k],
        1 => [(0x16, false), (0x17, false), (0x18, false), (0x21, false), (0x22, false), (0x10, true)][k],
        2 => [(0x19, false), (0x1a, false), (0x1b, false), (0x1c, false), (0x1d, false), (0x1e, false)][k],
        _ => [(0x26, false), (0x20, false), (0x23, false), (0x24, false), (0x25, false), (0x1f, false)][k],
    }
}

/// A money amount as the game shows it (`Cash::format_display_amount`, 0x450aa0, on a local `Cash`) into `out`.
pub(crate) unsafe fn transfer_tab_cash(amount: i32, out: &mut [u8]) -> usize {
    let b = out.as_mut_ptr() as usize;
    exe::cash_format_display_amount(&amount, b, 0xc8, 6);
    b
}

/// The languages (1..64) the person speaks with a fluency from `low` to `high` (`PlayerStats::get_language_fluency`),
/// joined into `list` ("  a", "a, b", "a, b and c") through `tmp`: a first pass counts, the second writes.
pub(crate) unsafe fn transfer_tab_languages(p: Staff, tmp: usize, list: usize, low: i8, high: i8) {
    let lang = staff_tab_lang();
    let format = staff_tab_format();
    let mut total = 0;
    for pass in 1..=2 {
        let mut n = 0;
        for language in 1..=0x40u32 {
            let f = exe::playerstats_get_language_fluency(read_usize(G_PLAYER_STATS), p, 0usize, language, 1) as i8;
            if f < low || f > high {
                continue;
            }
            n += 1;
            if pass != 2 {
                total = n;
                continue;
            }
            let name = database_get_language_name(language);
            if n == 1 {
                format(tmp, 0x987ff0usize, name); // "  %s"
            } else if n < total {
                lang(tmp, 0xa8adacusize, list, name); // "<%s - languages>, <%s - language>"
            } else {
                lang(tmp, 0xa8ad84usize, list, name); // "<%s - languages> and <%s - language>"
            }
            staff_tab_strcpy(list, tmp);
        }
    }
}

/// The offers of the list `list` (each node holds an offer slot index): an offer is shown when the human controls the
/// person's contract club or the bidding club, the person knows of it (`offer::PLAYER_AWARE`) or the viewer has him
/// shortlisted. The detailed text (the bid's total value as money, with "part-exchange", or the part-exchange, loan
/// or free transfer kind) goes to whoever controls one of the two clubs or when the person knows; otherwise only the
/// bidding club. The row is a button 0x13 (the human's club, slot), 0xe (the bidding human, slot) or 0x7d5 (the
/// bidding club); loan and free transfer offers always link the club.
pub(crate) unsafe fn transfer_tab_offer_list(st: &mut TransferTab, list: usize, shortlisted: bool, b: usize) {
    let lang = staff_tab_lang();
    let p = st.p;
    let tm = read_usize(G_TRANSFER_MANAGER);
    write_u32(list + 0xa, read_u32(list + 2)); // the list's cursor back to its head
    let mut node = exe::llist_get_next(list);
    while node != 0 {
        let slot = exe::transfer_manager_get_offer_slot(tm, read_i32(node));
        let offer = Offer(read_usize(slot));
        let bidder = club_at((offer.buyer_club_id() as i32) as isize);
        let controls = |c: Club| exe::human_manager_current_controls_club(c, 0, 1) != 0;
        if transfer_tab_controls_contract_club(p) || controls(bidder) || offer.player_aware() != 0 || shortlisted {
            covered!("tel08_3 transfer tab offer");
            if st.row > st.first {
                st.label(0);
            }
            let value = exe::transfer_offer_get_total_value(offer, 1, 1, 0, 1);
            let v = value as u32;
            let amount = exe::cash_get_display_amount(&v, 5);
            exe::cash_format_money(b, 0xc8, amount, 5);
            let mut kind_only = false;
            let g = (bidder.short_gender_name() as i8) as i32;
            let name = bidder.addr() + club::SHORT_NAME;
            if transfer_tab_controls_contract_club(p) || controls(bidder) || offer.player_aware() != 0 {
                let exchange = offer.exchange_players().addr();
                if value != 0 && exchange != 0 {
                    lang(JOB_TEXT, 0xa8ac78usize, g, g, name, b); // "  {}<%s - Club>{} - <%s - Cash> part-exchange"
                } else if offer.pre_contract() == 0 && value != 0 {
                    lang(JOB_TEXT, 0xa8ac40usize, g, g, name, b); // "  {}<%s - Club>{} - <%s - Cash>"
                } else if exchange != 0 {
                    lang(JOB_TEXT, 0xa8ac10usize, g, g, name); // "  {}<%s - Club(e.g.Arsenal)>{} - part-exchange"
                } else if offer.offer_type() == 1 {
                    lang(JOB_TEXT, 0xa8abe4usize, g, g, name); // "  {}<%s - Club(e.g.Arsenal)>{} - loan offer"
                    kind_only = true;
                } else {
                    lang(JOB_TEXT, 0xa8abb4usize, g, g, name); // "  {}<%s - Club(e.g.Arsenal)>{} - free transfer"
                    kind_only = true;
                }
            } else {
                lang(JOB_TEXT, 0xa8acc0usize, g, g, name); // "  {}<%s - Club(e.g.Arsenal)>{}"
            }
            if transfer_tab_controls_contract_club(p) && !kind_only {
                st.button(0x13, slot);
            } else if controls(bidder) && !kind_only {
                st.button(0xe, slot);
            } else {
                st.button(0x7d5, bidder.addr());
            }
            st.row += 1;
        }
        node = exe::llist_get_next(list);
    }
}

/// The person's mood comments of the Future section, in the original's order (each piece below). Leaves the
/// contract's club (looked up again) and the club's league statistics entry (matches played; current and final
/// position, losing and winning run when more than 8 matches were played) in the state.
pub(crate) unsafe fn transfer_tab_mood(st: &mut TransferTab) {
    let (_, club) = contract_tab_lookup(st.p);
    let club = Club(club);
    st.club = club;
    let division = club.division().addr();
    if division != 0 {
        let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_i32(division) as usize * 4));
        if comp != 0 && read_usize(comp + 8) != 0 {
            let e = exe::comp_stats_find_club_entry(read_usize(comp + 8), club);
            if e != 0 {
                st.matches_played = read_i8(e + comp_stats_club::MATCHES_PLAYED);
                if read_u8(e + comp_stats_club::MATCHES_PLAYED) > 8 {
                    st.position = read_i8(e + comp_stats_club::CURRENT_POSITION);
                    st.final_position = read_i8(e + comp_stats_club::FINAL_POSITION);
                    st.losing_run = read_i8(e + comp_stats_club::LOSING_RUN);
                    st.winning_run = read_i8(e + comp_stats_club::WINNING_RUN);
                }
            }
        }
    }
    if transfer_tab_mood_interest(st) {
        transfer_tab_mood_sales(st);
    }
    transfer_tab_mood_squad(st);
    transfer_tab_mood_league(st);
    transfer_tab_mood_listing(st);
    transfer_tab_mood_pay(st);
    transfer_tab_mood_dislikes(st);
    transfer_tab_mood_manager(st);
    transfer_tab_mood_contract(st);
    transfer_tab_mood_grievances(st);
    transfer_tab_mood_selection(st);
    transfer_tab_mood_personal(st);
    transfer_tab_mood_outlook(st);
}

/// `Date::get_short_date_string` (0x54a650, thiscall, ret 0xc): the date at `date` into `out`.
pub(crate) unsafe fn history_tab_date(date: usize, out: usize) {
    exe::date_get_short_date_string(date, out, 2000, 0xff);
}

/// The 0xff-ended list of statistic codes that are the columns of the Playing Career view: with screen parameter
/// 0x15 (the second statistics view) the 0xa84a90 list for a goalkeeper (`goalkeeper` 20) and 0xa84a78 otherwise;
/// without it 0xa84a9c for a goalkeeper and 0xa84a84 otherwise.
pub(crate) unsafe fn history_tab_columns(p: Staff, second_view: usize) -> usize {
    let keeper = !p.player().is_null() && p.player().goalkeeper() == 0x14;
    match (second_view != 0, keeper) {
        (true, true) => 0xa84a90,
        (true, false) => 0xa84a78,
        (false, true) => 0xa84a9c,
        (false, false) => 0xa84a84,
    }
}

/// The empty cells of a table row from column 0 to `columns` - 1 (style 2, 0, 0xc, 2, pale grey, no text).
pub(crate) unsafe fn history_tab_blank_row(area: u32, row: i32, columns: u32) {
    for col in 0..columns {
        staff_tab_cell(area, col, row, 2, 0, 0xc, 2, G_GUI_COLOUR_PALE_GREY, 0);
    }
}

/// The scroll row of the table area: `Area::set_scroll_row` with screen parameter 9 when there is an area, and the
/// area index (as a signed word) in screen parameter 10.
pub(crate) unsafe fn history_tab_keep_scroll(area: u32) {
    let row = get_screen_param(9);
    if area as i16 != -1 {
        exe::area_set_scroll_row(display_area_at(current_human_display(), area as i16), row);
    }
    set_screen_param(10, area as i16 as i32 as usize);
}

/// The fee text of a career season into `out` (200 bytes): a positive fee as money (`Cash::get_display_amount` and
/// `cash_format_money` in mode 5), with "<%s - cash> p/x" around it for a type 2 (exchange) move; no fee: "Exch" for
/// type 2, "Bos" for type 3 and "Free" otherwise; a negative fee: "Loan" for type 1, nothing otherwise. The type is
/// the low 3 bits of the season's byte +0x28.
pub(crate) unsafe fn history_tab_fee_text(e: usize, out: &mut [u8; 200]) {
    let lang = staff_tab_lang();
    out[0] = 0;
    let fee = read_i32(e + 0x24);
    let kind = read_u8(e + 0x28) & 7;
    if fee > 0 {
        let mut money = [0u8; 200];
        let amount = exe::cash_get_display_amount(&(fee as u32), 5);
        exe::cash_format_money(money.as_mut_ptr() as usize, 200, amount, 5);
        if kind == 2 {
            lang(out.as_mut_ptr() as usize, 0xa8b0a0usize, money.as_ptr() as usize); // "<%s - cash> p/x"
        } else {
            staff_tab_strcpy(out.as_mut_ptr() as usize, money.as_ptr() as usize);
        }
    } else if fee == 0 {
        let s = match kind {
            2 => 0x98e620, // "Exch"
            3 => 0x98e61c, // "Bos"
            _ => 0x98e614, // "Free"
        };
        lang(out.as_mut_ptr() as usize, s);
    } else if kind == 1 {
        lang(out.as_mut_ptr() as usize, 0x98e5fcusize); // "Loan"
    }
}

/// `PlayerStats` text of statistic `code` (0x7c1b20 the column title; 0x7c2750 a season's value; 0x7c3060 the career
/// total), thiscall on `G_PLAYER_STATS`, into the screens' buffer.
pub(crate) unsafe fn history_tab_stat_title(code: u8) {
    exe::player_stats_get_field_label(read_usize(G_PLAYER_STATS), code as u32, JOB_TEXT, 2000);
}

pub(crate) unsafe fn history_tab_stat_season(entry: usize, code: u8, full: u32) {
    exe::playerstats_format_record_field(read_usize(G_PLAYER_STATS), entry, code as u32, JOB_TEXT, 2000, 1, 0, full);
}

pub(crate) unsafe fn history_tab_stat_total(totals: usize, code: u8) {
    exe::playerstats_format_totals_field(read_usize(G_PLAYER_STATS), totals, code as u32, JOB_TEXT, 2000, 0, 0, 0);
}

/// A statistic cell of the Playing Career table: code 0x11 in style 0x30 with the 0xae3192 colour word, the others in
/// style 2, 0, 0xc, 2; text colour `colour`.
pub(crate) unsafe fn history_tab_stat_cell(area: u32, col: u32, row: i32, code: u8, colour: usize) {
    if code == 0x11 {
        staff_tab_cell(area, col, row, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, colour, JOB_TEXT);
    } else {
        staff_tab_cell(area, col, row, 2, 0, 0xc, 2, colour, JOB_TEXT);
    }
}

/// "  %c%s" of a name with its first letter in capitals (`utils_char_to_upper`) into the screens' buffer.
pub(crate) unsafe fn history_tab_capitalised(name: &[u8; 200]) {
    let first = exe::utils_char_to_upper(u32::from_le_bytes([name[0], name[1], name[2], name[3]]) as i32) as u32 & 0xff;
    staff_tab_format()(JOB_TEXT, 0xa8b060usize, first, name.as_ptr().add(1) as usize); // "  %c%s"
}

/// A label and value row of the Manager Stats view: the label in pale grey, the value (in the screens' buffer, written
/// by `value`) in yellow.
pub(crate) unsafe fn history_tab_stat_row(area: u32, row: &mut i32, label: usize, value: impl FnOnce()) {
    staff_tab_lang()(JOB_TEXT, label);
    staff_tab_cell(area, 0, *row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
    value();
    staff_tab_cell(area, 1, *row, 2, 0, 1, 2, G_GUI_COLOUR_YELLOW, JOB_TEXT);
    *row += 1;
}

/// "  %d" of a number into the screens' buffer.
pub(crate) unsafe fn history_tab_number(v: i32) {
    staff_tab_format()(JOB_TEXT, 0x990b48usize, v); // "  %d"
}

/// The players bought or sold line: "  %d" with none, else the count and the total value as money (mode 6).
pub(crate) unsafe fn history_tab_deals(count: i16, value: u32, fmt: usize) {
    if count == 0 {
        history_tab_number(0);
    } else {
        let mut money = [0u8; 200];
        let amount = exe::cash_get_display_amount(&value, 6);
        exe::cash_format_money(money.as_mut_ptr() as usize, 200, amount, 6);
        staff_tab_lang()(JOB_TEXT, fmt, count as i32, money.as_ptr() as usize);
    }
}

/// The club of the contract of `s` (`profile_contract_lookup`), or 0 without a contract.
pub(crate) unsafe fn profile_contract_club(s: usize) -> usize {
    let c = Contract(profile_contract_lookup(s));
    if c.is_null() { 0 } else { club_at((c.club() as i32) as isize).addr() }
}

/// Whether the contract of `s` (`profile_contract_lookup`) exists and is of type `t` (`contract::CONTRACT_TYPE` & 0x3f).
pub(crate) unsafe fn profile_contract_is(s: usize, t: u8) -> bool {
    let c = Contract(profile_contract_lookup(s));
    !c.is_null() && c.contract_type() & 0x3f == t
}

/// `human_manager_current_controls_club(club, 0, 1)`.
pub(crate) unsafe fn profile_controls(club: usize) -> bool {
    exe::human_manager_current_controls_club(club, 0, 1) != 0
}

/// `staff_is_not_board_member` (0x4cb960, stdcall, ret 4; the callers load ecx with the contract manager).
pub(crate) unsafe fn profile_not_board_member(s: usize) -> bool {
    exe::staff_is_not_board_member(s) != 0
}

/// The CRT's inline strcat of the original.
pub(crate) unsafe fn profile_strcat(dst: usize, src: usize) {
    let mut n = 0;
    while read_u8(dst + n) != 0 {
        n += 1;
    }
    profile_strcpy(dst + n, src);
}

/// One line of the chairman table: the label, the value (`%d`, light blue) and the description, row `row`.
pub(crate) unsafe fn profile_chairman_line(area: u32, row: i32, label: usize, value: i8, description: usize) {
    let lang = staff_tab_lang();
    lang(JOB_TEXT, label);
    staff_tab_cell(area, 0, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
    staff_tab_format()(JOB_TEXT, 0x98806cusize, value as i32); // "%d"
    staff_tab_cell(area, 1, row, 2, 0, 0xc, 2, JOB_COLOUR_BDD4, JOB_TEXT);
    lang(JOB_TEXT, description);
    staff_tab_cell(area, 2, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
}

/// The preferred style text of a coach into the screens' buffer, by attacking (`a`) and directness (`d`): both ≥ 15
/// direct attacking, attacking ≥ 15 and directness ≤ 5 attractive attacking, attacking ≤ 5 and directness ≥ 15
/// cautious but direct, both ≤ 5 patient and defensive; otherwise attacking (≥ 15), defensive (≤ 5), direct
/// (directness ≥ 15), patient (≤ 5) or none.
pub(crate) unsafe fn profile_style_text(a: i8, d: i8) {
    let s = if a >= 15 && d >= 15 {
        0xa85cb0
    } else if a >= 15 && d <= 5 {
        0xa85c80
    } else if a <= 5 && d >= 15 {
        0xa85c50
    } else if a <= 5 && d <= 5 {
        0xa85c20
    } else if a >= 15 {
        0xa85bf8
    } else if a <= 5 {
        0xa85bd4
    } else if d >= 15 {
        0xa85bb0
    } else if d <= 5 {
        0xa85b8c
    } else {
        0xa85b68
    };
    staff_tab_lang()(JOB_TEXT, s);
}

/// Whether the current human controls the club of the person's contract: the contract asked twice (each lookup left in
/// `G_CONTRACT_LOOKUP_RESULT`), false without a contract.
pub(crate) unsafe fn transfer_tab_controls_contract_club(p: Staff) -> bool {
    let (c, club) = contract_tab_lookup(p);
    if c.is_null() || club == 0 {
        return false;
    }
    let (_, club) = contract_tab_lookup(p);
    exe::human_manager_current_controls_club(club, 0, 1) != 0
}

/// Mood, part 1: the university course (the easter egg of "Andrew Mainwaring", potential over 100, 18 to 21), the
/// 'Sporting Just Cause' dispute with the club, the interest shown in him (a player not listed, without a future
/// transfer, with interested clubs and more than 150 days at the club: the visiting scouts, pressure or flattery by
/// the contract grievances, the club's reputation, ambition, loyalty and pressure) and the favoured team-mate (no
/// loan, not listed, no future transfer, none of the grievances 0x10181). Returns false (the original's jump past the
/// sales and the first new contract check) when the contract has no club.
pub(crate) unsafe fn transfer_tab_mood_interest(st: &mut TransferTab) -> bool {
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let pl = p.player();
    let age = p.age() as i8;
    if exe::staff_name_equals(p, 0xa8a9a8usize) != 0 // "Andrew Mainwaring"
        && !club.is_null()
        && !pl.is_null()
        && pl.potential_ability() > 100
        && age > 0x11
        && age < 0x16
    {
        st.say(0xa8a96c); // "  Wants to finish his university course before moving clubs"
    }
    let dispute = read_usize(G_DISPUTE);
    if dispute != 0 {
        if club.is_null() {
            return false;
        }
        if exe::dispute_is_in_dispute(dispute, p, 0) != 0 && exe::dispute_get_dispute_club_id(read_usize(G_DISPUTE), p) == read_i32(club.addr()) {
            st.say(0xa8a920); // "  Is attempting to leave the club under the 'Sporting Just Cause' clause"
        }
    }
    if club.is_null() {
        return false;
    }
    if !pl.is_null() && exe::staff_is_transfer_listed_or_requested(p, 0usize) == 0 && !st.future_now() && read_u16(st.list) != 0 && st.days_joined() > 0x96 {
        let g = c.grievances();
        let small = g & 0x14 != 0 || club.reputation() < 0x109a;
        let s = if g & 0x101 != 0 {
            Some(0xa8a8f4) // "  Is hoping to impress the visiting scouts"
        } else if small && p.ambition() > 0xf && p.loyalty() < 6 {
            Some(0xa8a8f4)
        } else if small {
            if p.pressure() < 6 {
                Some(0xa8a8bc) // "  Feels pressured by the interest being shown in him"
            } else if club.reputation() < 0x109a && p.ambition() > 0xf && p.pressure() > 0xf {
                Some(0xa8a88c) // "  Flattered by the interest being shown in him"
            } else {
                None
            }
        } else {
            None
        };
        if let Some(s) = s {
            st.say(s);
        }
    }
    if exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) == 0
        && exe::staff_is_transfer_listed_or_requested(p, 0usize) == 0
        && !st.future_now()
        && c.grievances() & 0x10181 == 0
    {
        let fav = Staff(staff_pick_favoured_team_mate(p.addr(), club.addr()));
        if !fav.is_null() && !fav.player().is_null() {
            transfer_tab_favoured_team_mate(st, fav);
        }
    }
    true
}

/// Mood, part 2: the sale of a promising youngster (grievance 0x40000000, named when the opinion link holds him) and
/// of a star player (grievance 0x20000000; by the club's form, ambition and loyalty).
pub(crate) unsafe fn transfer_tab_mood_sales(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let c = st.contract;
    if c.grievances() & 0x40000000 != 0 {
        let link = staff_get_opinion_link_staff(p.addr(), 0x40000000);
        if link != 0 {
            let mut buf = [0u8; 0x1f4];
            let n = transfer_tab_name(link, &mut buf, 0x1f4, 5);
            if p.pressure() < 6 {
                lang(JOB_TEXT, 0xa8a240usize, n); // "  Thinks selling youngster <%s> was a very short-sighted move"
            } else {
                lang(JOB_TEXT, 0xa8a1e4usize, n); // "  Is disappointed that the club sold promising youngster <%s>"
            }
        } else if p.pressure() < 6 {
            lang(JOB_TEXT, 0xa8a19cusize); // "  Is upset by the sale of one of the club's most promising youngsters"
        } else {
            lang(JOB_TEXT, 0xa8a150usize); // "  Is disappointed that the club sold one of its most promising youngsters"
        }
        st.line();
    }
    if c.grievances() & 0x20000000 != 0 {
        let link = staff_get_opinion_link_staff(p.addr(), 0x20000000);
        let mut buf = [0u8; 0x1f4];
        let n = if link != 0 { transfer_tab_name(link, &mut buf, 0x1f4, 5) } else { 0 };
        let ambitious = p.ambition() > 0xf && p.loyalty() < 6;
        let s = if st.club_state(TRANSFER_TAB_CLUB_FORM) > -2 {
            match (link != 0, ambitious) {
                (true, true) => 0xa8a0ec,   // "  Thinks the club's lack of ambition is shown by the sale of <%s>"
                (true, false) => 0xa8a0a8,  // "  Is unhappy at the sale of <%s>"
                (false, true) => 0xa89f9c,  // "  Thinks the club's lack of ambition is shown by the sale of its star players"
                (false, false) => 0xa89f60, // "  Is unhappy at the sale of one of the club's star players"
            }
        } else {
            match (link != 0, p.loyalty() > 0xf) {
                (true, true) => 0xa8a064,   // "  Disappointed by the sale of <%s>"
                (true, false) => 0xa89fec,  // "  Believes the club will never turn things around while they sell players like <%s>"
                (false, true) => 0xa89f20,  // "  Hopes the club can hold onto its star players in the future"
                (false, false) => 0xa89ec8, // "  Believes the club will never turn things around while they sell their best players"
            }
        };
        if link != 0 {
            lang(JOB_TEXT, s, n);
        } else {
            lang(JOB_TEXT, s);
        }
        st.line();
    }
}

/// Mood, part 3: the new contract (pleased after 300 days at the club, flags and grievances clear and loyal; or very
/// happy after 180 days, no grievances, loyal and professional; both within 30 days of the start), and for a player
/// the competition for a place (flag 0x200), a rival signed (flag 0x80000000: his position, named when linked), the
/// rumoured replacement (flag 0x800), the lack of harmony (flag 4 after 30 days), else the good dressing room
/// atmosphere (no grievances, not listed, a settled club, professional), and the squad depth (flag 0x400: by the
/// club's form and transfer window, or its reputation).
pub(crate) unsafe fn transfer_tab_mood_squad(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let pl = p.player();
    if !club.is_null() && st.days_joined() > 0x12c && st.days_started() < 0x1e && st.flags() == 0 && c.grievances() == 0 && p.loyalty() > 0xf {
        st.say(0xa89e9c); // "  Is pleased to have signed a new contract"
    } else if c.grievances() == 0 && st.days_joined() > 0xb4 && st.days_started() < 0x1e && p.loyalty() > 10 && p.professionalism() > 10 {
        st.say(0xa89e74); // "  Is very happy with his new contract"
    }
    let mut harmony = false;
    if !pl.is_null() {
        if st.flags() & 0x200 != 0 {
            st.say(0xa89e34); // "  Feels there is too much competition for a first team place"
        }
        if st.flags() & 0x80000000 != 0 {
            let mask = exe::staff_get_natural_position_mask(p);
            let mut pos = [0u8; 0x200];
            let pb = pos.as_mut_ptr() as usize;
            let g = position_mask_get_text(mask, pb, 0) as i8 as i32;
            let link = staff_get_opinion_link_staff(p.addr(), 0x80000000);
            if link != 0 {
                let mut buf = [0u8; 0x1f4];
                let n = transfer_tab_name(link, &mut buf, 0x1f4, 5);
                // "  Is upset that his manager has signed rival {}<%s - Position>{} <%s - Rival Name>{}" / "  Is unhappy that ..."
                lang(JOB_TEXT, if p.pressure() < 6 { 0xa89ddcusize } else { 0xa89d70 }, g, g, 100u32, pb, n);
            } else {
                // "  Is upset that his manager has signed a rival {}<%s>{}" / "  Is unhappy that the manager has signed another {}<%s>{}"
                lang(JOB_TEXT, if p.pressure() < 6 { 0xa89d2cusize } else { 0xa89cd0 }, g, g, pb);
            }
            st.line();
        }
        if st.flags() & 0x800 != 0 {
            st.say(0xa89c8c); // "  Worried by rumours that the club is signing a replacement for him"
        }
        if st.days_joined() > 0x1e && st.flags() & 4 != 0 {
            st.say(if p.pressure() < 6 {
                0xa89c5c // "  Worried by the lack of harmony at the club"
            } else if p.pressure() > 0xf {
                0xa89c28 // "  Disappointed by the lack of harmony at the club"
            } else {
                0xa89bf8 // "  Dismayed by the lack of harmony at the club"
            });
            harmony = true;
        }
    }
    if club.is_null() {
        return;
    }
    if !harmony
        && !pl.is_null()
        && c.grievances() == 0
        && exe::staff_is_transfer_listed_or_requested(p, 0usize) == 0
        && st.club_state(TRANSFER_TAB_CLUB_UNREST) == 0
        && p.professionalism() > 0xe
    {
        st.say(0xa89bbc); // "  Thinks that the club has a good dressing room atmosphere"
    }
    if st.flags() & 0x400 != 0 {
        covered!("tel08_3 transfer tab squad depth");
        let nation = club.nation();
        let mut k: i8 = 0;
        let mut in_window = false;
        if !nation.is_null() {
            k = exe::key_nation_index_of_nation(nation);
        }
        if nation.is_null() || (0..0x23).contains(&k) {
            let t = G_LEAGUE_WORLD_CUP_START_DATE.wrapping_add((k as isize * 0x48) as usize);
            let day = read_i16(G_CURRENT_DATE);
            let year = read_u16(G_CURRENT_DATE + 2);
            let after = (day > read_i16(t) && year == read_u16(t + 2)) || year > read_u16(t + 2);
            let before = (day < read_i16(t + 0xf) && year == read_u16(t + 0x11)) || year < read_u16(t + 0x11);
            in_window = after && before;
        }
        let s = if st.club_state(TRANSFER_TAB_CLUB_FORM) > 2 {
            if in_window {
                0xa89b50 // "  Feels the club needs more squad depth to sustain its league form"
            } else {
                0xa89b94 // "  Feels the club needs more squad depth"
            }
        } else if club.reputation() > 0x1676 {
            0xa89b24 // "  Believes the club needs more squad depth"
        } else {
            0xa89ae4 // "  Is worried by the lack of competition for places in the team"
        };
        lang(JOB_TEXT, s);
        st.line();
    }
}

/// Mood, part 4: the league: underachieving (flag 0x10 of a player at a club below the expectations:
/// `transfer_tab_league_position`), more potential (no flag 0x10, more than 5 matches, the club well below), the new
/// manager (grievance 0x10000000: his reputation and days at the club), the successful club (no grievances, happy, a
/// player at a club above the expectations: joined, being there, the style of play by his ratings against the
/// competition's average, the league position, performing well) and the style of play he dislikes.
pub(crate) unsafe fn transfer_tab_mood_league(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let pl = p.player();
    if !pl.is_null() && !club.is_null() && st.club_state(TRANSFER_TAB_CLUB_FORM) < -1 && st.flags() & 0x10 != 0 {
        let s = transfer_tab_league_position(st);
        lang(JOB_TEXT, s);
        st.line();
    }
    if st.flags() & 0x10 == 0
        && st.matches_played > 5
        && !pl.is_null()
        && !club.is_null()
        && st.club_state(TRANSFER_TAB_CLUB_FORM) < -2
        && p.loyalty() > 10
        && p.professionalism() > 10
    {
        st.say(if p.pressure() > 0xf {
            0xa89938 // "  Feels the team has more potential than it is showing"
        } else {
            0xa898fc // "  Will strive to help the club improve it's league position"
        });
    }
    if c.grievances() & 0x10000000 != 0 && !club.is_null() && !club.manager().is_null() {
        let m = club.manager();
        let s = if st.days_since(m.addr() + staff::DATE_JOINED_CLUB) < 0x5a {
            let np = m.non_player();
            if !np.is_null()
                && st.club_state(TRANSFER_TAB_CLUB_FORM) < 0
                && np.current_reputation() < club.reputation().wrapping_sub(0x2ee)
                && p.loyalty() < 10
            {
                0xa898bc // "  Not convinced the club's new manager can turn the club around"
            } else if np.current_reputation() < club.reputation().wrapping_sub(0x4e2) {
                0xa89880 // "  Worried by his manager's lack of experience at this level"
            } else {
                0xa89844 // "  Waiting to see how the club's new manager changes things"
            }
        } else {
            0xa89814 // "  Has lost confidence in his manager's ability"
        };
        st.say(s);
    }
    let stats = read_usize(G_PLAYER_STATS);
    let average = || {
        let d = read_f64(stats + 0x18);
        if !(d < 0.0 || d > 0.0) { read_f64(rdata::F64_6_8) } else { read_f64(stats + 0x10) / d }
    };
    if c.grievances() == 0 && c.happiness() > 0x19 && !pl.is_null() {
        if club.is_null() {
            return;
        }
        if st.club_state(TRANSFER_TAB_CLUB_FORM) > 2 && p.loyalty() > 10 {
            covered!("tel08_3 transfer tab successful club");
            let rating = exe::playerstats_get_stat(stats, pl, 6, 1, 6.5) as f32;
            let other = exe::playerstats_get_stat(stats, pl, 6, 0x11, 6.5) as f32;
            let s = if st.days_joined() < 0x1e {
                0xa897e4 // "  Is pleased to have joined a successful club"
            } else if st.days_joined() < 0x5a {
                0xa897bc // "  Is pleased to be at a successful club"
            } else if rating > read_f32(rdata::F32_5) && (c.matches_left_out() as i8) <= 1 {
                0xa89734 // "  Is happy to see the club performing well in the league"
            } else if rating > read_f32(rdata::F32_5) && !(average() + read_f64(rdata::F64_0_25) >= other as f64) {
                0xa89798 // "  Enjoys the team's style of play"
            } else if (c.matches_left_out() as i8) > 1 && p.loyalty() > 0xf {
                0xa89770 // "  Is proud of the clubs league position"
            } else {
                0xa89734
            };
            st.say(s);
            return;
        }
    }
    if !club.is_null()
        && c.happiness() > 0x19
        && p.loyalty() < 10
        && p.pressure() < 10
        && p.ambition() > 0xf
        && p.professionalism() < 6
        && !pl.is_null()
        && pl.attacker() == 0x14
        && pl.current_reputation() > 0x128e
        && pl.current_reputation() as i32 > club.reputation() as i32 - 0x2ee
    {
        let rating = exe::playerstats_get_stat(stats, pl, 6, 1, 6.5) as f32;
        let other = exe::playerstats_get_stat(stats, pl, 6, 0x11, 6.5) as f32;
        if rating > read_f32(rdata::F32_5) && (c.matches_left_out() as i8) > 1 && average() - read_f64(rdata::F64_0_35) > other as f64 {
            st.say(0xa8970c); // "  Doesn't like the team's style of play"
        }
    }
}

/// Mood, part 5: the club's valuation of him (flag 8, no future transfer: outraged, disappointed, upset, unrealistic
/// or pressured by the grievances, temperament, professionalism, pressure and age), the club's finances (grievance
/// 0x400000), the style of play (flag 0x1000), the transfer listing (listed by the club: the squad status he deserves,
/// days at the club and character; listed by request: the grievance 1 and character; or the forthcoming transfer),
/// the agent (within 210 days of the expiry), the stepping stone (a young ambitious player at a small club), absent
/// without permission and the lost place in the team (more matches left out than loyalty / 5 + 1).
pub(crate) unsafe fn transfer_tab_mood_listing(st: &mut TransferTab) {
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let age = p.age() as i8;
    if st.flags() & 8 != 0 && !st.future_now() {
        let s = if c.grievances() & 0x10181 != 0 {
            if p.temperament() < 5 && p.professionalism() < 0xf {
                Some(0xa896d4) // "  Is outraged by the clubs unrealistic valuation of him"
            } else if p.professionalism() > 0xf {
                Some(0xa89698) // "  Is disappointed by the clubs unrealistic valuation of him"
            } else if p.pressure() < 5 {
                Some(0xa89660) // "  Is upset by the clubs unrealistic valuation of him"
            } else {
                Some(0xa895f4) // "  Thinks the clubs valuation of him is unrealistic"
            }
        } else if age > 0x12 {
            Some(if p.pressure() < 6 { 0xa89628 } else { 0xa895f4 }) // "  Feels pressured by the clubs high valuation of him"
        } else if p.pressure() < 6 {
            Some(0xa89628)
        } else {
            None
        };
        if let Some(s) = s {
            st.say(s);
        }
    }
    if c.grievances() & 0x400000 != 0 {
        let pl = p.player();
        let s = if p.ambition() > 0xf
            && age > 0x15
            && matches!(st.squad_bits(), 0x10 | 0x20)
            && pl.current_ability() as i32 > p.loyalty() as i32 + 100
            && pl.current_reputation() > 0xcb2
        {
            0xa895ac // "  Doesn't think a club in this financial state can fulfil his ambitions"
        } else if c.morale() < -75 && p.pressure() < 10 {
            0xa89584 // "  Unsettled by the financial upheaval"
        } else if p.pressure() < 6 {
            0xa89558 // "  Worried by the club's financial plight"
        } else if exe::staff_contract_wants_to_leave(c) != 0 {
            0xa89514 // "  Wants to join a club that has a more secure financial position"
        } else {
            0xa894dc // "  Concerned about the financial stability of the club"
        };
        st.say(s);
    }
    if st.flags() & 0x1000 != 0 {
        st.say(if st.future_now() {
            0xa8949c // "  Hopes he is joining a club which respects his style of play"
        } else if c.leaving_on_bosman() == 0 && exe::staff_is_listed_by_request(p, 0usize) == 0 {
            0xa89464 // "  Thinks the manager should accept his style of play"
        } else {
            0xa8942c // "  Wants to join a club which respects his style of play"
        });
    }
    transfer_tab_listed(st);
    if !club.is_null()
        && !p.player().is_null()
        && c.leaving_on_bosman() == 0
        && st.days_to_expiry() < 0xd2
        && p.loyalty() < 6
        && p.professionalism() < 10
        && age < 0x20
        && p.player().current_reputation() > club.reputation()
        && p.player().current_reputation() > 0x109a
    {
        st.say(0xa891a8); // "  Has been told by his agent that it is time to move on"
    }
    let pl = p.player();
    if club.reputation() < 0x128e
        && !pl.is_null()
        && age < 0x1a
        && pl.potential_ability() as i32 + p.ambition() as i32 * 2 > 0x78
        && p.loyalty() < 6
        && p.professionalism() < 10
        && p.ambition() > 10
        && age < 0x1c
    {
        st.say(if st.squad_bits() == 0x70 {
            0xa8913c // "  Thinks his manager is misjudging his ability"
        } else {
            0xa8916c // "  Thinks this club is a stepping stone towards his future"
        });
    }
    if exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) != 0 {
        st.say(0xa89120); // "  Absent without permission"
    }
    if !pl.is_null()
        && exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) == 0
        && c.unhappy_flag() as i32 > p.loyalty() as i32 / 5 + 1
        && matches!(st.squad_bits(), 0x10 | 0x20)
        && age > 0x14
        && age < 0x22
        && exe::injury_get_player_injury_status(read_usize(G_INJURY), p) != 2
    {
        let s = if p.professionalism() > 10 && pl.work_rate() > 10 {
            if p.loyalty() > 10 && age > 0x1c && !pl.is_null() && pl.current_reputation() > 0x1482 && club.reputation() > 0x1a5e {
                Some(0xa890f0) // "  Wants to be more involved in the first team"
            } else if exe::staff_contract_wants_to_leave(c) != 0 {
                Some(0xa890b0) // "  Eager to move in order to find regular first team football"
            } else {
                Some(0xa89080) // "  Is willing to fight for his place in the team"
            }
        } else if p.professionalism() < 6 && p.temperament() < 10 {
            Some(0xa89058) // "  Upset at losing his place in the team"
        } else {
            None
        };
        if let Some(s) = s {
            st.say(s);
        }
    }
}

/// Mood, part 6: the pay (grievance 0x1000000 at a club that is not a national team: his wage against 0.9 × the
/// club's highest wage, his world reputation and character), homesick or ending his career at home (grievance
/// 0x20000, over 32), wanting to leave or to be listed (grievance 0x10000, no future transfer), no support from the
/// manager (0x80000) and the role in the team (0x4000: ability and ambition against professionalism, reputation
/// against the club).
pub(crate) unsafe fn transfer_tab_mood_pay(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    if !club.is_null() && exe::database_is_national_team(club) == 0 && c.grievances() & 0x1000000 != 0 {
        let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
        let s = if finance != 0 && (read_i32(finance + finance::HIGHEST_WAGE) as f64 * read_f64(rdata::F64_0_9)) < c.wage() as f64 {
            0xa89034 // "  Feels he deserves to be paid more"
        } else if p.loyalty() < 10 && p.professionalism() < 5 && !p.player().is_null() && p.player().world_reputation() > 0x128e {
            0xa88fe0 // "  Feels that his pay must be on a par with the highest at the club if he is to stay"
        } else if p.ambition() > 0xf && p.loyalty() < 6 {
            0xa88fa4 // "  Demands that his pay matches the top players at the club"
        } else {
            0xa88f68 // "  Feels his pay should match the top players at the club"
        };
        st.say(s);
    }
    if c.grievances() & 0x20000 != 0 {
        if (p.age() as i8) > 0x20 {
            let n = p.nation();
            let g = (n.short_gender_name() as i8) as i32;
            lang(JOB_TEXT, 0xa88f28usize, g, g, n.addr() + nation::SHORT_NAME); // "  Wants to end career in {}<%s - Home Nation>{}"
        } else {
            lang(JOB_TEXT, 0xa88f18usize); // "  Is homesick "
        }
        st.line();
    }
    if c.grievances() & 0x10000 != 0 && !st.future_now() {
        st.say(if p.professionalism() < 10 && p.temperament() < 5 {
            0xa88ef0 // "  Wants to leave the club immediately"
        } else {
            0xa88ed0 // "  Wants to be transfer listed "
        });
    }
    if c.grievances() & 0x80000 != 0 {
        st.say(0xa88ea4); // "  Feels he has no support from his manager"
    }
    if c.grievances() & 0x4000 != 0 {
        let pl = p.player();
        st.say(
            if p.temperament() < 6
                && p.ambition() > 10
                && !pl.is_null()
                && pl.current_ability() as i32 + p.ambition() as i32 > p.professionalism() as i32 + 0x5a
                && pl.current_reputation() > club.reputation()
            {
                0xa88e78 // "  Expects a more important role in the team"
            } else {
                0xa88e40 // "  Feels he deserves a more important role in the team"
            },
        );
    }
}

/// Mood, part 7: the disliked staff (grievance 0x800, then flag 1 without 0x800, each after more than id % 7 + 4 days
/// at the club: `transfer_tab_disliked`), the assistant manager (grievance 0x1000), the referees of the club's nation
/// (flag 0x20, another region than his nation's) and the pride in the club he favours (no grievances or flags, loyal).
pub(crate) unsafe fn transfer_tab_mood_dislikes(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    if c.grievances() & 0x800 != 0 && st.days_joined() > read_i32(p.addr()) % 7 + 4 {
        transfer_tab_disliked(st, false);
    }
    if c.grievances() & 0x1000 != 0 && !club.is_null() && club.assistant_manager() != 0 {
        st.say(0xa88b68); // "  Dislikes the club's Assistant Manager"
    }
    if st.flags() & 1 != 0 && c.grievances() & 0x800 == 0 && st.days_joined() > read_i32(p.addr()) % 7 + 4 {
        transfer_tab_disliked(st, true);
    }
    if !club.is_null() {
        let n = club.nation();
        let home = p.nation();
        if !n.is_null() && n != home && home.actual_region() != n.actual_region() && st.flags() & 0x20 != 0 {
            let g = (n.gender_name() as i8) as i32;
            // "  Feels that referees in {}<%s - Nation Name (eg. England)>{} are victimising him"
            lang(JOB_TEXT, 0xa88a3cusize, g, g, n.addr() + 4);
            st.line();
        }
    }
    if c.grievances() == 0 && st.flags() == 0 && p.loyalty() > 0xf && exe::staff_favours_club(p, club) != 0 {
        let g = (club.short_gender_name() as i8) as i32;
        if c.matches_left_out() != 0 {
            lang(JOB_TEXT, 0xa889f8usize, g, g, club.addr() + club::SHORT_NAME); // "  Proud to be playing at {}<%s>{}"
        } else {
            lang(JOB_TEXT, 0xa889bcusize, g, g, club.addr() + club::SHORT_NAME); // "  Proud to be at {}<%s>{}"
        }
        st.button(0x7d5, club.addr());
        st.pad();
        st.row += 1;
    }
}

/// Mood, part 8: the manager: his praise (a loyal, professional player under no grievances or flags, not a youth or
/// part-time job, at a club whose board trusts the manager, in good form and happy: a tactical genius, proud to play
/// for him, hopes he stays, extremely competent), his dislike (grievance 0x2000: tactics or team selection by the
/// manager's ratings when the board's confidence is low), the squad status (above what he deserves: indispensable,
/// the rotation, or the value and contribution he wants acknowledged under a 10000 wage), the rumours the manager
/// may leave (the manager favours him, board confidence under 4000) and the board dismissing him (board confidence
/// under 4250: sure he will turn it around, foolish or wrong to dismiss).
pub(crate) unsafe fn transfer_tab_mood_manager(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let mm = read_usize(G_MANAGER_MANAGER);
    let m = if club.is_null() { Staff(0) } else { club.manager() };
    if c.grievances() == 0
        && st.flags() == 0
        && !matches!(p.job_for_club(), 0xd | 0xf | 5)
        && p.loyalty() > 0xf
        && p.professionalism() > 0xf
        && !m.is_null()
        && !m.non_player().is_null()
        && mm != 0
        && !club.is_null()
        && transfer_tab_board_confidence(club) > 0x186a
        && st.club_state(TRANSFER_TAB_CLUB_FORM) > 0
        && c.happiness() > 0
    {
        covered!("tel08_3 transfer tab manager praise");
        let mut buf = [0u8; 0x190];
        let n = transfer_tab_name(m, &mut buf, 0x190, 5);
        let s = if (p.age() as i8) < 0x14 && st.winning_run != 0 && m.non_player().current_reputation() > 0x1c52 {
            0xa8897c // "  Thinks <%s> is a tactical genius"
        } else if exe::staff_is_favoured_by(m, p) != 0 {
            if c.matches_left_out() != 0 && p.ambition() > 0xf {
                0xa88940 // "  Proud to be playing for <%s>"
            } else {
                0xa888e8 // "  Hopes that <%s> will remain manager for a very long time"
            }
        } else {
            0xa888a4 // "  Thinks <%s> is extremely competent"
        };
        lang(JOB_TEXT, s, n);
        st.line();
    }
    if c.grievances() & 0x2000 != 0 {
        let s = if !club.is_null() && !m.is_null() && !m.non_player().is_null() && p.temperament() < 5 && mm != 0 && transfer_tab_board_confidence(club) < 0x109a {
            let np = m.non_player();
            if exe::non_player_attribute_rating(np, np.tactics()) < 10 {
                0xa88874 // "  Thinks the manager is tactically incompetant"
            } else if exe::non_player_attribute_rating(np, np.judgement()) < 10 {
                0xa88844 // "  Thinks the manager's team selection is poor"
            } else {
                0xa88824 // "  Dislikes the club's Manager"
            }
        } else {
            0xa88824
        };
        st.say(s);
    }
    if c.grievances() & 1 == 0 && exe::staff_is_transfer_listed(p, 0usize) == 0 {
        let deserved = st.deserved() as i8;
        let sq = st.squad() as i8;
        if sq > deserved
            && sq != 0
            && sq != 8
            && (sq != 3 || (p.ambition() > 0xc && p.professionalism() < 0xf && p.temperament() < 0xf))
            && st.deserved() != 6
            && st.deserved() != 5
        {
            let pl = p.player();
            if st.deserved() == 1 && !pl.is_null() && pl.current_reputation() > 0xcb2 && p.professionalism() < 10 && p.temperament() < 6 {
                st.say(0xa887f8); // "  Believes he is indispensable to the club"
            } else if st.squad() == 3 && c.unhappy_flag() > 1 {
                st.say(0xa887c8); // "  Is not happy with the squad rotation system"
            } else {
                transfer_tab_undervalued(st);
            }
        }
    }
    if !m.is_null()
        && exe::staff_is_favoured_by(m, p) != 0
        && c.grievances() == 0
        && p.loyalty() > 10
        && mm != 0
        && transfer_tab_board_confidence(club) < 0xfa0
    {
        let mut buf = [0u8; 0x190];
        let n = transfer_tab_name(m, &mut buf, 0x190, 5);
        lang(JOB_TEXT, 0xa88774usize, n); // "  Is unhappy at rumours that <%s> may leave the club"
        st.line();
    }
    let pl = p.player();
    if st.flags() == 0
        && c.grievances() == 0
        && !m.is_null()
        && mm != 0
        && !pl.is_null()
        && transfer_tab_board_confidence(club) < 0x109a
        && p.loyalty() > 0xf
    {
        let mut buf = [0u8; 0x190];
        let n = transfer_tab_name(m, &mut buf, 0x190, 5);
        let s = if st.club_state(TRANSFER_TAB_CLUB_FORM) < 0 {
            0xa885d4 // "  Is sure his manager will turn the club around"
        } else if pl.current_reputation() as i32 > club.reputation() as i32 + 0x4e2 && p.ambition() > 10 && p.professionalism() < 10 {
            0xa88720 // "  Thinks the board would be foolish to dismiss <%s>"
        } else {
            0xa88604 // "  Thinks the board would be wrong to dismiss <%s>"
        };
        if s == 0xa885d4 {
            lang(JOB_TEXT, s);
        } else {
            lang(JOB_TEXT, s, n);
        }
        st.line();
    }
}

/// Mood, part 9: the contract's status bits. Bit 2 alone set with others (no future transfer): turning professional (a
/// part-time ambitious youngster with potential), then always one of loyalty rewarded, a manager who appreciates him,
/// the agent looking for a club or thinking of moving; bit 1: a bigger club (the future transfer's club against his
/// own) or furthering his career; bit 2: the agent, being paid what he deserves, the forthcoming move, or the contract
/// (disliking the club, extending, negotiating or a new contract).
pub(crate) unsafe fn transfer_tab_mood_contract(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let g = c.grievances();
    if g & 2 != 0 && g != 2 && !st.future_now() {
        covered!("tel08_3 transfer tab contract status");
        let pl = p.player();
        if c.contract_type() & 0x80 != 0
            && c.grievances() & 1 == 0
            && p.ambition() > 0xf
            && p.loyalty() < 0xf
            && !pl.is_null()
            && pl.potential_ability() as i32 > 0x50 - p.ambition() as i32
            && (p.age() as i8) < 0x19
        {
            st.say(if p.loyalty() >= 10 {
                0xa88554 // "  Would like the opportunity to turn professional"
            } else if p.ambition() > 0xe && p.professionalism() < 10 {
                0xa885a8 // "  Feels he should be a professional player"
            } else {
                0xa88588 // "  Wants to turn professional"
            });
        }
        let agent = || {
            if p.temperament() < 5 && !p.player().is_null() && p.player().current_reputation() > 0x1194 {
                0xa88488 // "  Has instructed his agent to look for a more suitable club to join"
            } else {
                0xa8846c // "  Thinking of moving clubs"
            }
        };
        let s = if !club.is_null()
            && !pl.is_null()
            && p.loyalty() > 10
            && pl.current_reputation() > 0x109a
            && pl.current_reputation() > club.reputation().wrapping_sub(0x113)
        {
            if p.professionalism() > 5 && st.days_joined() > 0x190 {
                0xa88520 // "  Thinks his loyalty to the club should be rewarded"
            } else if !club.manager().is_null() {
                if p.temperament() >= 5 {
                    0xa8846c
                } else if read_usize(G_MANAGER_MANAGER) != 0 && transfer_tab_board_confidence(club) < 0xfa0 {
                    0xa884cc // "  Hopes that a manager who appreciates his contribution takes over the club soon"
                } else {
                    agent()
                }
            } else {
                agent()
            }
        } else if p.temperament() >= 6 || p.professionalism() >= 10 || p.loyalty() >= 10 {
            0xa8846c
        } else if !p.player().is_null() && p.player().current_reputation() > 0x1194 {
            0xa88488
        } else {
            0xa8846c
        };
        st.say(s);
        return;
    }
    if c.grievances() & 1 != 0 {
        let pl = p.player();
        let s = if !pl.is_null() && pl.current_reputation() > 0x1194 && p.professionalism() < 10 && p.temperament() < 5 {
            if st.future != 0 {
                0xa8843c // "  Relieved to be moving to a bigger club soon"
            } else {
                0xa88404 // "  Wants to move to a bigger club as soon as possible"
            }
        } else if st.future != 0 {
            let slot = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), read_i32(st.future + future_transfer::OFFER_SLOT));
            if slot != 0
                && read_usize(slot) != 0
                && club.reputation() as i32 + 0x2ee < club_at(read_i32(read_usize(slot) + offer::BUYER_CLUB_ID) as isize).reputation() as i32
            {
                0xa883e0 // "  Looking forward to his big move"
            } else {
                0xa892ac // "  Looking forward to moving clubs"
            }
        } else if (p.age() as i8) < 0x1c && p.professionalism() > 0xf {
            0xa883a8 // "  Feels that he needs to leave to further his career"
        } else {
            0xa88384 // "  Wants to move to a bigger club"
        };
        st.say(s);
    }
    if c.grievances() & 2 != 0 {
        let pl = p.player();
        let s = if p.loyalty() < 10 && p.professionalism() < 5 && !pl.is_null() && pl.world_reputation() > 0x128e {
            0xa88348 // "  His agent believes he can get a better contract elsewhere"
        } else if p.temperament() < 5 && pl.current_reputation() > 0x128e {
            0xa88324 // "  Wants to be paid what he deserves"
        } else if st.future_now() {
            if p.loyalty() < 6 && p.professionalism() < 10 {
                0xa88308 // "  Glad to be moving clubs"
            } else if p.loyalty() > 0xf {
                0xa882e8 // "  Sad to be leaving the club"
            } else {
                0xa882d0 // "  Looking to the future"
            }
        } else {
            let dislikes = exe::staff_dislikes_club(p, club) != 0;
            let age = p.age() as i8;
            if dislikes && age < 0x20 {
                if p.loyalty() < 10 && p.professionalism() < 10 {
                    0xa88348
                } else if c.grievances() == 2 {
                    0xa88298 // "  Considering moving clubs at the end of his contract"
                } else {
                    0xa8827c // "  Considering his options"
                }
            } else if age > 0x20 && st.days_to_expiry() < 0x5a {
                0xa88258 // "  Would like to extend his contract"
            } else if c.morale() > 0x19 && c.grievances() == 2 && st.days_to_expiry() < p.loyalty() as i32 * 10 + 0x7d {
                0xa88258
            } else if p.loyalty() > 10 && p.professionalism() > 10 {
                0xa8822c // "  Would like to negotiate a new contract"
            } else {
                0xa88214 // "  Wants a new contract"
            }
        };
        lang(JOB_TEXT, s);
        st.line();
    }
}

/// Mood, part 10: the grievances about his position (0x2000000, not listed: by days at the club and temperament, then
/// the manager tactically inept), his preferred position (flag 0x40 without 0x2000000), trust (0x8000000), the
/// strongest positions (flag 0x100), not listening (flag 0x80), the fringe or being substituted (0x4000000), unfair
/// treatment (0x200), a teammate's treatment (0x400), being obstructed from leaving (0x800000), the first team regular
/// (4), the media (0x40000: the story kind, the supporters' and the media's view by days at the club), the new
/// challenge (0x100) and the manager (8).
pub(crate) unsafe fn transfer_tab_mood_grievances(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let age = p.age() as i8;
    if c.grievances() & 0x2000000 != 0 && exe::staff_is_transfer_listed_or_requested(p, 0usize) == 0 {
        let mut pos = [0u8; 0x200];
        let pb = pos.as_mut_ptr() as usize;
        position_mask_get_text(exe::staff_get_natural_position_mask(p), pb, 0);
        let s = if st.days_joined() < 0x3c {
            if p.temperament() < 6 {
                0xa881a8 // "  Wouldn't have joined the club if he'd known he wouldn't play as a <%s>"
            } else {
                0xa88110 // "  Wants to play as a <%s>"
            }
        } else if p.temperament() < 6 && p.professionalism() < 6 {
            0xa88150 // "  Wants to move to a club where he can play as a <%s>"
        } else {
            0xa88110
        };
        lang(JOB_TEXT, s, pb);
        st.line();
        if p.professionalism() < 6
            && p.pressure() > 10
            && !p.player().is_null()
            && age > 0x14
            && p.player().current_reputation() > 0x1676
            && c.grievances() & 0x2000 == 0
        {
            st.say(0xa880e4); // "  Thinks the manager is tactically inept"
        }
    }
    if exe::staff_is_transfer_listed_or_requested(p, 0usize) == 0 && st.flags() & 0x40 != 0 && c.grievances() & 0x2000000 == 0 {
        let mut pos = [0u8; 0x200];
        let pb = pos.as_mut_ptr() as usize;
        position_mask_get_text(exe::staff_get_natural_position_mask(p), pb, 0);
        // "  Feels his best position is playing as a <%s>" / "  Would like to play as a <%s>"
        lang(JOB_TEXT, if p.professionalism() > 5 { 0xa88094usize } else { 0xa88050 }, pb);
        st.line();
    }
    if c.grievances() & 0x8000000 != 0 {
        st.say(0xa88024); // "  Feels he can no longer trust his manager"
    }
    if st.flags() & 0x100 != 0 {
        st.say(if p.temperament() < 6 {
            0xa87fd8 // "  Angry that the manager is not putting players in their strongest position"
        } else if p.professionalism() > 10 {
            0xa87f84 // "  Thinks the team would perform better if players were in their strongest positions"
        } else {
            0xa87f38 // "  Upset that the manager doesn't field players in their strongest position"
        });
    }
    if st.flags() & 0x80 != 0 {
        st.say(0xa87f04); // "  Feels the manager doesn't listen to his players"
    }
    if c.grievances() & 0x4000000 != 0 {
        st.say(
            if p.ambition() > 0xf && age > 0x17 && c.happiness() < 0 && !matches!(st.squad(), 0 | 8 | 7 | 1 | 2) {
                0xa87ed4 // "  Wants to be more than just a fringe player"
            } else {
                0xa87eb4 // "  Unhappy at being substituted"
            },
        );
    }
    if c.grievances() & 0x200 != 0 {
        let pl = p.player();
        st.say(if st.days_joined() < 0xe {
            0xa87e78 // "  Feels he was brought to the club under false pretences"
        } else if st.days_joined() < 0x3c {
            0xa87e40 // "  Feels he made the wrong decision in joining the club"
        } else if p.pressure() < 6 && p.professionalism() < 0xf {
            0xa87e10 // "  Feels he has been victimised by his manager"
        } else if p.loyalty() > 5
            && p.professionalism() < 0xb
            && p.temperament() < 0xb
            && age > 0x18
            && !pl.is_null()
            && pl.leadership() > 8
            && pl.current_reputation() > 0x1194
        {
            0xa87de0 // "  Feels he should be treated with more respect"
        } else {
            0xa87db8 // "  Feels he has been treated unfairly"
        });
    }
    if c.grievances() & 0x400 != 0 {
        st.say(if p.temperament() > 10 && p.pressure() > 10 {
            0xa87d78 // "  Believes the manager should be more patient with his staff"
        } else if p.pressure() < 5 && p.professionalism() < 10 {
            0xa87d2c // "  Believes the manager's treatment of players is affecting the team morale"
        } else {
            0xa87d08 // "  Unhappy at treatment of teammate"
        });
    }
    if c.grievances() & 0x800000 != 0 && !st.future_now() {
        st.say(if exe::staff_is_transfer_listed_or_requested(p, 0usize) != 0 {
            0xa87cb8 // "  Feels he cannot trust the manager as he is obstructing his leaving the club"
        } else {
            0xa87c70 // "  Feels the manager is obstructing any chance of his leaving the club"
        });
    }
    if c.grievances() & 4 != 0 {
        let pl = p.player();
        let s = if st.future_now()
            && !pl.is_null()
            && p.pressure() < 10
            && pl.current_reputation() > 0xea6
            && pl.current_reputation() > club.reputation().wrapping_sub(0x2ee)
        {
            0xa87c1c // "  Thinks the manager is leaving him out of the team because he's leaving the club"
        } else if !club.is_null()
            && !pl.is_null()
            && pl.current_reputation() > 0xea6
            && pl.current_reputation() > club.reputation()
            && p.professionalism() < 0xf
        {
            if p.temperament() < 6 {
                0xa87bd0 // "  Is angry that his manager doesn't consider him to be a first team regular"
            } else if p.ambition() > 10 {
                if read_i32(p.addr()) % 3 == 0 {
                    0xa87b80 // "  Is shocked that his manager doesn't consider him to be a first team regular"
                } else {
                    0xa87b38 // "  Is shocked that his manager doesn't treat him as a first team regular"
                }
            } else {
                0xa87b0c // "  Feels he should be a first team regular"
            }
        } else {
            0xa87b0c
        };
        st.say(s);
    }
    if !p.player().is_null() && c.grievances() & 0x40000 != 0 {
        transfer_tab_media(st);
    }
    if c.grievances() & 0x100 != 0 {
        st.say(if st.future_now() {
            0xa87794 // "  Is looking forward to a new challenge"
        } else {
            0xa877bc // "  Wants a new challenge"
        });
    }
    if !club.is_null() && !club.manager().is_null() && c.grievances() & 8 != 0 {
        st.say(0xa87778); // "  Unhappy with his manager"
    }
}

/// Mood, part 11: the selection (grievance 0x10): a player out of the board's favour wanting a new manager (the text
/// is written and then overwritten, as in the original), then by the matches left out: back in the team (glad,
/// worried, hoping to stay or not convinced, with "Unsettled" or "Wants a long run" first when 0x10 is the only
/// grievance) or out of it (reputation, waiting, ready, happy to wait, fed up, forcing his way, first team football).
pub(crate) unsafe fn transfer_tab_mood_selection(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    if c.grievances() & 0x10 == 0 {
        return;
    }
    covered!("tel08_3 transfer tab selection");
    let pl = p.player();
    if !pl.is_null()
        && p.loyalty() < 6
        && p.professionalism() < 6
        && pl.current_ability() > 0x50
        && pl.current_reputation() > 0x109a
        && read_usize(G_MANAGER_MANAGER) != 0
        && transfer_tab_board_confidence(club) < 0x109a
    {
        // "  Hopes the club's next manager will appreciate his talent more" (overwritten below, never shown)
        lang(JOB_TEXT, 0xa87738usize);
    }
    let only = |c: Contract| c.grievances() == 0x10;
    let unsettled = |st: &mut TransferTab| {
        if only(st.contract) {
            st.say(0xa88b90); // "  Unsettled"
        }
    };
    let s = if c.matches_left_out() as i8 > 1 {
        if c.grievances() & 0x6046008 != 0 || exe::staff_is_transfer_listed_or_requested(p, club) != 0 {
            unsettled(st);
            0xa87650 // "  Isn't convinced that he will stay in the managers plans"
        } else if p.professionalism() > 10 && p.pressure() > 10 {
            if only(c) {
                st.say(0xa87710); // "  Wants a long run in the first team"
            }
            0xa876ec // "  Is glad to be back in the team"
        } else if p.pressure() < 6 {
            unsettled(st);
            0xa876b4 // "  Is worried that he may be dropped from the team again"
        } else {
            unsettled(st);
            0xa8768c // "  Hopes to stay in the managers plans"
        }
    } else {
        let rep = pl.current_reputation();
        if rep > 0x186a && rep as i32 > club.reputation() as i32 - 0x4e2 && p.ambition() > 10 && p.professionalism() < 6 {
            0xa8760c // "  Feels his reputation should guarantee his place in the first team"
        } else if !club.is_null() && !pl.is_null() && p.ambition() > 0xe && rep < club.reputation() {
            let r = (read_i32(p.addr()) % 8) as i16;
            if p.professionalism() < 6
                && p.loyalty() < 6
                && pl.potential_ability() > (p.ambition() as i16).wrapping_add(r).wrapping_add(100)
            {
                unsettled(st);
                0xa875cc // "  May look elsewhere if he doesn't get first team football soon"
            } else if p.professionalism() < 0xf && (p.age() as i8) < 0x17 {
                if p.determination() < 10 && p.pressure() < 6 {
                    0xa875a4 // "  Worried about his future at the club"
                } else if p.pressure() <= 10 && p.loyalty() <= 10 {
                    0xa87578 // "  Feels he is ready for first team football"
                } else if c.morale() > 0 {
                    0xa87548 // "  Is happy to wait for his chance in the team"
                } else {
                    0xa88b90 // "  Unsettled"
                }
            } else {
                unsettled(st);
                if c.grievances() & 0x101 != 0 {
                    0xa87518 // "  Fed up waiting for a chance in the first team"
                } else {
                    0xa874e8 // "  Trying to force his way into the first team"
                }
            }
        } else {
            0xa874cc // "  Wants first team football"
        }
    };
    st.say(s);
}

/// Mood, part 12: life abroad (grievance 0x20 with a nation for the contract's club: time to move on after 720 days,
/// a mistake, or the lifestyle by the nationality adjective), personal problems (0x40), a team-mate's remarks
/// (0x200000), the manager's handling of the media (the second media response, a first team player), discipline
/// (flag 2), motivation (morale under −25), pressure (0x100000: handling it, rested, past his peak, worried, or in
/// English the club relying on him or jaded) and training (0x8000).
pub(crate) unsafe fn transfer_tab_mood_personal(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let age = p.age() as i8;
    let n = Nation(read_usize(club_at(c.club() as isize).addr() + club::NATION));
    if c.grievances() & 0x20 != 0 && !n.is_null() {
        if st.days_joined() > 0x2d0 {
            lang(JOB_TEXT, 0xa874acusize); // "  Feels it is time to move on"
        } else if p.temperament() < 6 && p.professionalism() < 0xf {
            let g = (n.gender_name() as i8) as i32;
            lang(JOB_TEXT, 0xa8745cusize, g, g, n.addr() + 4); // "  Feels he may have made a mistake in moving to {}<%s>{}"
        } else {
            let mut adj = [0u8; 0xc8];
            let a = adj.as_mut_ptr() as usize;
            exe::nation_get_nationality_adjective(n, a, 0xc8, 0x20);
            let g = (n.gender_name() as i8) as i32;
            lang(JOB_TEXT, 0xa87408usize, g, g, a); // "  Finding it hard to adjust to the {}<%s>{} lifestyle"
        }
        st.line();
    }
    if c.grievances() & 0x40 != 0 {
        st.say(
            if p.pressure() < 5 && p.professionalism() < 6 && p.temperament() < 10 && p.player().world_reputation() > 0x109a {
                0xa873e4 // "  Is being treated for depression"
            } else {
                0xa873cc // "  Has personal problems"
            },
        );
    }
    if c.grievances() & 0x200000 != 0 {
        st.say(if p.pressure() < 6 {
            0xa873a4 // "  Feels hurt by his team-mate's remarks"
        } else {
            0xa87378 // "  Is unhappy about his team-mate's remarks"
        });
    }
    if exe::media_has_response_two_for_player(read_usize(G_MEDIA), p) != 0
        && p.pressure() > 0xf
        && p.temperament() < 6
        && p.player().current_reputation() > 0xea6
        && age > 0x12
        && matches!(st.squad(), 1 | 2)
    {
        st.say(if p.temperament() < 6 && p.professionalism() < 10 && p.pressure() > 5 {
            0xa87344 // "  Outraged by his manager's handling of the media"
        } else if p.pressure() < 6 {
            0xa87314 // "  Hurt by his manager's comments to the media"
        } else {
            0xa872e0 // "  Upset with his manager's handling of the media"
        });
    }
    if st.flags() & 2 != 0 {
        st.say(0xa872b0); // "  Worried by the lack of managerial discipline"
    }
    let pl = p.player();
    if !club.is_null()
        && !pl.is_null()
        && c.morale() < -25
        && pl.work_rate() < 10
        && p.professionalism() < 10
        && pl.current_reputation() as i32 > club.reputation() as i32 + 0xfa
    {
        st.say(0xa87288); // "  Is having trouble motivating himself"
    }
    if c.grievances() & 0x100000 != 0 {
        let s = if age < 0x17 && p.pressure() < 10 && p.professionalism() < 6 {
            Some(0xa87244) // "  Is having trouble handling the pressure of first team football"
        } else if !pl.is_null() && age < 0x17 && p.professionalism() > 0xf && pl.stamina() < 0xf {
            Some(0xa87220) // "  Feels that he needs to be rested"
        } else if age > 0x1e && p.pressure() < 6 {
            Some(0xa871fc) // "  Believes that he is past his peak"
        } else if age <= 0x1e && p.pressure() < 6 && c.grievances() & 0x20181 == 0 {
            Some(0xa875a4) // "  Worried about his future at the club"
        } else if exe::langlib_get_language() == 0 {
            Some(if ((c.matches_left_out() as i8) > 4 || c.happiness() > 0x4b) && age < 0x1b {
                0xa871d0 // "  Feels the club relies upon him too much"
            } else {
                0xa871c8 // "  Jaded"
            })
        } else {
            None
        };
        if let Some(s) = s {
            st.say(s);
        }
    }
    if c.grievances() & 0x8000 != 0 {
        st.say(if p.pressure() < 10 && p.professionalism() < 10 && age < 0x18 {
            0xa87190 // "  Feels his training regime is harming his development"
        } else {
            0xa87178 // "  Unhappy with training"
        });
    }
}

/// Mood, part 13: the outlook. With a future transfer: `transfer_tab_outlook_moving`. Otherwise not being allowed to
/// move (grievance 0x80), then the Bosman exit (with its date), the clarification wanted by a player without a squad
/// status (a button 5 when the human controls him), or, with no grievances, morale not below −loyalty and no
/// transfer request: anxious for a new contract, the manager's favour (helping the club, proving him wrong, a lost
/// career, the mentor, enthusiasm, developing), the loan, and the stay (`transfer_tab_outlook_stay`) or "Unknown".
pub(crate) unsafe fn transfer_tab_mood_outlook(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    if st.future_now() {
        transfer_tab_outlook_moving(st);
        return;
    }
    if c.grievances() & 0x80 != 0 {
        st.say(if p.temperament() < 6 && p.professionalism() < 0xf {
            0xa87144 // "  Is angry about not being allowed to move clubs"
        } else if p.professionalism() > 0xe {
            0xa87110 // "  Is frustrated at not being allowed to move clubs"
        } else {
            0xa870e0 // "  Is unhappy at not being allowed to move clubs"
        });
    }
    if c.leaving_on_bosman() != 0 {
        covered!("tel08_3 transfer tab bosman");
        let mut buf = [0u8; 0xc8];
        let b = buf.as_mut_ptr() as usize;
        exe::date_get_short_date_string(c.addr() + contract::CONTRACT_EXPIRES, b, 0xc8, 0xff);
        lang(JOB_TEXT, 0xa870a4usize, 100u32, b); // "  Leaving the club under the Bosman ruling on {}<%s - date>"
        st.line();
        return;
    }
    if st.squad() == 0 && p.job_for_club() == 0xb {
        lang(JOB_TEXT, 0xa87080usize); // "  Wants clarification on his future"
        let own = p.club_job();
        if (!own.is_null()
            && exe::human_manager_current_controls_club(own, 0, 1) != 0
            && exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) == 0)
            || exe::human_manager_current_controls_club(club_at(c.club() as isize), 0, 1) != 0
        {
            st.button(5, 0);
        } else {
            st.value(JOB_TEXT);
        }
        st.pad();
        st.row += 1;
        return;
    }
    if c.grievances() != 0 || (c.morale() as i32) < -(p.loyalty() as i32) || c.transfer_status() & 8 != 0 {
        return;
    }
    let pl = p.player();
    if p.pressure() < 6
        && (p.age() as i8) < 0x18
        && !st.future_now()
        && exe::staff_is_transfer_listed_or_requested(p, club) == 0
        && pl.current_reputation() < 0xcb2
        && pl.current_reputation() < club.reputation().wrapping_add(0xfa)
        && st.days_to_expiry() < 0xc8 - p.pressure() as i32
    {
        st.say(0xa8705c); // "  Anxious to sign a new contract"
    }
    if exe::staff_is_favoured_by(club.manager(), p) != 0 {
        transfer_tab_outlook_favoured(st);
    }
    if exe::staff_is_loan_listed(p, 0usize) != 0
        && (p.age() as i8) < 0x18
        && !pl.is_null()
        && pl.current_reputation() < 0x109a
        && p.professionalism() > 0xf
        && p.pressure() > 10
        && !matches!(st.squad(), 4 | 1 | 2 | 3)
    {
        st.say(0xa86e78); // "  Hoping to gain first team experience from going on loan"
    }
    if exe::staff_is_transfer_listed_or_requested(p, club) == 0 && exe::dispute_is_in_dispute(read_usize(G_DISPUTE), p, 0) == 0 {
        let s = transfer_tab_outlook_stay(st);
        st.say(s);
    } else if st.row == st.first {
        staff_tab_lang()(JOB_TEXT, 0xa86cb8usize); // "  Unknown"
        st.wrapped(G_GUI_COLOUR_YELLOW);
        st.row += 1;
    }
}

/// `CONTRACT_MANAGER::get_contract` of `s` kept in `G_CONTRACT_LOOKUP_RESULT` (the original's inline lookup).
pub(crate) unsafe fn profile_contract_lookup(s: usize) -> usize {
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), s);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    c
}

/// The person's name into `out` (`database_format_staff_name`, kind `kind`).
pub(crate) unsafe fn transfer_tab_name(s: impl Addr, out: &mut [u8], size: u32, kind: u32) -> usize {
    let b = out.as_mut_ptr() as usize;
    exe::database_format_staff_name(s, b, size, kind);
    b
}

/// The board confidence in the club's manager (`manager_manager_get_board_confidence`, 0x885800: the word +6 of the
/// club's 0x49-byte board record; 0 for a null club).
pub(crate) unsafe fn transfer_tab_board_confidence(club: Club) -> i16 {
    if club.is_null() {
        return 0;
    }
    read_i16(read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add(read_i32(club.addr()) as usize * 0x49) + board_record::CONFIDENCE)
}

/// The favoured team-mate line (a button 0x7d7 to him, then the same text wrapped): a fellow national (with the
/// grievances 0x20020), an old hand (over 34 and less than 20 happiness at the club: a good influence when
/// professional and a leader, else experience), a youngster (under 21: listed, capped, rated against the club, or a
/// hot prospect), or by the listing, his contract, the positions, his role, age and reputation, leadership and
/// happiness (retain, renew, learn from, a more important role, honoured, hang onto, inspired leader, essential, a lot
/// to offer).
pub(crate) unsafe fn transfer_tab_favoured_team_mate(st: &mut TransferTab, fav: Staff) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let fc = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), fav, club));
    let mut buf = [0u8; 0x1f4];
    let n = transfer_tab_name(fav, &mut buf, 0x1f4, 1);
    let nation = p.nation();
    let fage = fav.age() as i8;
    let fp = fav.player();
    if !nation.is_null() && nation == fav.nation() && c.grievances() & 0x20020 != 0 {
        // "  Is finding the presence of fellow {}<%s - Nationality>{} player <%s - Admired Staff Name>{} helpful"
        lang(JOB_TEXT, 0xa8a804usize, 100u32, 100u32, 100u32, nation.addr() + nation::NATIONALITY, n);
    } else if fage > 0x22 && fc.happiness() < 0x14 {
        if fav.professionalism() > 0xf && fp.leadership() > 0xf {
            lang(JOB_TEXT, 0xa8a7a0usize, n); // "  Thinks <%s> is a good influence on youngsters at the club"
        } else {
            lang(JOB_TEXT, 0xa8a744usize, 100u32, n); // "  Thinks <%s>{s} experience is useful to the team"
        }
    } else if fage < 0x15 {
        if exe::staff_is_transfer_listed_or_requested(fav, 0usize) != 0 {
            lang(JOB_TEXT, 0xa8a6dcusize, n); // "  Thinks <%s> should be given the chance to develop as a player"
        } else if fav.int_apps() != 0
            || (!fp.is_null() && fp.current_reputation() > 0xea6 && fp.current_reputation() as i32 > club.reputation() as i32 - 0x2ee)
        {
            lang(JOB_TEXT, 0xa8a64cusize, n); // "  Thinks <%s> is a talented player"
        } else {
            lang(JOB_TEXT, 0xa8a694usize, n); // "  Thinks <%s> is a hot prospect"
        }
    } else if exe::staff_is_transfer_listed_or_requested(fav, 0usize) != 0 {
        lang(JOB_TEXT, 0xa8a5ecusize, 100u32, n); // "  Thinks the club should retain the services of <%s>{}"
    } else if fc.is_null() {
        lang(JOB_TEXT, 0xa8a578usize, 100u32, n); // "  Thinks the club should try everything possible to renew <%s>{s} contract"
    } else {
        let s = transfer_tab_favoured_opinion(st, fav, fc);
        lang(JOB_TEXT, s, n);
    }
    st.button(0x7d7, fav.addr());
    st.line();
}

/// The league position comment of `transfer_tab_mood_league` (flag 0x10 of a player at an underachieving club).
pub(crate) unsafe fn transfer_tab_league_position(st: &TransferTab) -> usize {
    let p = st.p;
    let c = st.contract;
    let pl = p.player();
    let top_or_first = matches!(st.squad_bits(), 0x10 | 0x20 | 0x30);
    if p.pressure() > 10 && p.loyalty() < 6 && pl.current_reputation() > 0x1676 && st.club.reputation() < 0x186a && top_or_first {
        return 0xa89ac0; // "  Feels he is too good for the club"
    }
    if p.pressure() < 6 && p.loyalty() < 6 && pl.current_reputation() > 0x1676 {
        return 0xa89a74; // "  Worried that the club's poor league position will affect his reputation"
    }
    let near_top = st.position < 8 && st.final_position < 4;
    if near_top && p.ambition() > 10 {
        return 0xa89a44; // "  Thinks the club should be topping the table"
    }
    if st.losing_run > 2 && p.loyalty() > 0xf {
        return if near_top {
            0xa89a44
        } else {
            0xa89a0c // "  Is ashamed of the team's recent dismal league form"
        };
    }
    if p.loyalty() < 6 && (st.position > 8 || st.final_position > 3) && top_or_first {
        return 0xa899d8; // "  Disappointed by the club's poor league position"
    }
    if exe::injury_get_player_injury_status(read_usize(G_INJURY), p) != 2
        && st.winning_run == 0
        && p.loyalty() > 0xf
        && p.pressure() > 10
        && c.happiness() > 10
        && exe::playerstats_get_recent_form(read_usize(G_PLAYER_STATS), pl) < 0x1e
        && matches!(st.squad_bits(), 0x10 | 0x20)
    {
        return 0xa89994; // "  Admits he is partially responsible for the team's slump in form"
    }
    0xa89970 // "  Thinks the club is underachieving"
}

/// The transfer listing comments of `transfer_tab_mood_listing`.
pub(crate) unsafe fn transfer_tab_listed(st: &mut TransferTab) {
    let p = st.p;
    let c = st.contract;
    if exe::staff_is_transfer_listed(p, 0usize) != 0
        && c.leaving_on_bosman() == 0
        && !st.future_now()
        && c.grievances() == 0
        && exe::staff_contract_wants_to_leave(c) == 0
    {
        covered!("tel08_3 transfer tab listed by club");
        let bits = st.squad_bits();
        let out_of_plans = bits == 0 || bits == 0x80 || st.deserved() == 6 || st.deserved() == 5;
        let s = if out_of_plans {
            if p.ambition() > 0xf {
                0xa89328 // "  Believes he can force his way back into the manager's plans"
            } else if p.pressure() < 10 && p.loyalty() > 0xf {
                0xa89408 // "  Doesn't want to leave the club"
            } else {
                0xa89300 // "  Disappointed at being transfer listed"
            }
        } else {
            if st.days_joined() > 0x28a && p.loyalty() > 0xf && p.professionalism() > 10 {
                st.say(0xa89408);
            }
            if p.loyalty() > 10 && p.temperament() > 10 && p.professionalism() > 10 {
                0xa893e4 // "  Shocked at being transfer listed"
            } else if p.temperament() < 6 {
                0xa893c0 // "  Angry at being transfer listed"
            } else if p.ambition() > 10 && st.deserved() == 1 {
                0xa89390 // "  Thinks he is too good to be transfer listed"
            } else if p.pressure() < 6 && st.deserved() == 1 {
                0xa89368 // "  Surprised at being transfer listed"
            } else {
                0xa89300
            }
        };
        st.say(s);
        return;
    }
    if (exe::staff_is_transfer_listed(p, 0usize) != 0 || exe::staff_is_listed_by_request(p, 0usize) != 0)
        && p.loyalty() < 6
        && p.professionalism() < 10
        && c.grievances() & 1 != 0
    {
        let s = if p.loyalty() == 1 && p.ambition() > 0xf && p.temperament() < 10 {
            0xa892d0 // "  Looking forward to moving to a better club"
        } else if p.ambition() > 0xf && p.temperament() < 6 {
            0xa892ac // "  Looking forward to moving clubs"
        } else {
            0xa89288 // "  Pleased at being transfer listed"
        };
        st.say(s);
        return;
    }
    if exe::staff_is_listed_by_request(p, 0usize) != 0 && c.grievances() == 0 && exe::staff_contract_wants_to_leave(c) == 0 {
        st.say(if st.future_now() {
            if p.pressure() < 10 {
                0xa8925c // "  Nervous about his forthcoming transfer"
            } else if p.loyalty() > 10 {
                0xa89238 // "  Feels he is leaving a great club"
            } else {
                0xa89208 // "  Feels it is a shame he has to leave the club"
            }
        } else {
            0xa891e0 // "  Would now like to stay at the club"
        });
    }
}

/// The disliked staff of the person (`staff_get_disliked_staff`, at most 3) at his club, each a line with a button
/// 0x7db to him, then, when the disliked one is transfer listed, the glad / taking action / harmony line; "Unsettled"
/// when he dislikes nobody. The first list (grievance 0x800) takes only players (job 11) and says disruptive,
/// selfish, arrogant or dislikes; the second (flag 1 without 0x800) says immature, hopes he will learn, or irritated.
pub(crate) unsafe fn transfer_tab_disliked(st: &mut TransferTab, second: bool) {
    let lang = staff_tab_lang();
    let p = st.p;
    let mut list = [0usize; 50];
    let mut count: i8 = 0;
    exe::staff_get_disliked_staff(p, list.as_mut_ptr() as usize, &mut count as *mut i8 as usize);
    let mut k: i8 = 0;
    while k < count {
        let d = Staff(list[k as usize]);
        if (second || d.job_for_club() == 0xb) && d.club_job() == p.club_job() {
            covered!("tel08_3 transfer tab disliked");
            let mut buf = [0u8; 0x1f4];
            let n = transfer_tab_name(d, &mut buf, 0x1f4, 1);
            let dp = d.player();
            let s = if dp.is_null() {
                if second { 0xa88a90 } else { 0xa88cc4 } // "  Irritated by <%s>" / "  Dislikes <%s>"
            } else if second {
                if p.professionalism() > 0xf && d.professionalism() < 6 && (d.age() as i8) < 0x18 {
                    0xa88b24 // "  Thinks <%s> is immature"
                } else {
                    0xa88ad0 // "  Hopes <%s> will learn from his mistakes"
                }
            } else if p.professionalism() > 0xf && d.professionalism() < 6 {
                0xa88de0 // "  Thinks <%s> is a disruptive influence on the team"
            } else {
                let arrogant = dp.current_ability() as i32 > 100 - p.ambition() as i32 && d.ambition() > 0xf && d.professionalism() < 6;
                if dp.goalkeeper() != 0x14 && dp.teamwork() < 6 {
                    if arrogant { 0xa88d90 } else { 0xa88d44 } // "  Feels <%s> is selfish and arrogant" / "... a selfish player"
                } else if arrogant {
                    0xa88d00 // "  Feels <%s> is arrogant"
                } else {
                    0xa88cc4
                }
            };
            lang(JOB_TEXT, s, n);
            st.button(0x7db, d.addr());
            st.pad();
            st.row += 1;
            if exe::staff_is_transfer_listed(d, 0usize) != 0 {
                if p.temperament() < 6 && p.professionalism() < 10 {
                    lang(JOB_TEXT, 0xa88c74usize, n); // "  Will be glad to see the back of <%s>"
                } else if read_i32(p.addr()) % 3 != 0 {
                    lang(JOB_TEXT, 0xa88c10usize, n); // "  Is glad to see the manager taking action against <%s>"
                } else {
                    lang(JOB_TEXT, 0xa88b9cusize, 100u32, n); // "  Thinks that <%s>{} leaving the club will restore dressing-room harmony"
                }
                st.button(0x7db, d.addr());
                st.pad();
                st.row += 1;
            }
        }
        k += 1;
    }
    if count == 0 {
        lang(JOB_TEXT, 0xa88b90usize); // "  Unsettled"
        st.value(JOB_TEXT);
        st.pad();
        st.row += 1;
    }
}

/// The undervalued player of `transfer_tab_mood_manager` (a squad status above the one he deserves, earning under
/// 10000): re-assess his value, acknowledge his contribution, or more important than the manager indicates.
pub(crate) unsafe fn transfer_tab_undervalued(st: &mut TransferTab) {
    let p = st.p;
    let c = st.contract;
    let pl = p.player();
    let not_first = |st: &TransferTab| st.squad() != 3 && st.squad() != 2;
    if p.professionalism() > 0xe && !pl.is_null() && pl.current_reputation() > 0x128e && not_first(st) && c.wage() < 10000 {
        st.say(0xa886e4); // "  Hopes the manager will re-assess his value to the team"
    } else if !pl.is_null() && p.professionalism() < 6 && pl.current_reputation() > 0xea6 && c.wage() < 10000 {
        st.say(0xa8869c); // "  Believes the manager should acknowledge his contribution to the team"
    } else if p.pressure() < 10 && !pl.is_null() && pl.current_reputation() > 0x109a && not_first(st) && c.wage() < 10000 {
        st.say(0xa88654); // "  Believes he is more important to the team than the manager indicates"
    }
}

/// The media comment of `transfer_tab_mood_grievances` (grievance 0x40000 of a player): by the finable story kind
/// (`get_finable_story_kind`), else by the supporters' view (`club_dislikes_staff`, `club_favours_staff`), loyalty,
/// pressure and days at the club, else the (local) media being overly critical.
pub(crate) unsafe fn transfer_tab_media(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let kind = get_finable_story_kind(read_usize(G_MEDIA), p.addr());
    let critical = || if club.reputation() > 0x1388 { 0xa87810 } else { 0xa877d4 }; // "  Feels the (local) media have been overly critical of him"
    let s = if kind != 0 {
        if kind == 8 {
            0xa87810
        } else if kind == 0x13 || (p.loyalty() <= 0xf && p.professionalism() <= 10) {
            0xa87ae8 // "  Has vented his feelings publicly"
        } else {
            0xa87abc // "  Feels he has been misquoted by the media"
        }
    } else if st.days_joined() < 0x5a {
        if exe::club_dislikes_staff(p, club) != 0 {
            if p.loyalty() > 0xf {
                0xa87a98 // "  Wants to win over the supporters"
            } else if p.loyalty() < 5 {
                0xa87a60 // "  Is fed up with the sniping from a minority of fans"
            } else {
                0xa87a30 // "  Hopes the supporters will give him a chance"
            }
        } else if exe::club_favours_staff(p, club) != 0 {
            if p.loyalty() > 0xf {
                0xa879fc // "  Cares only about what the supporters think of him"
            } else if p.loyalty() < 5 {
                0xa879cc // "  Dislikes the media criticism he has recieved"
            } else {
                0xa8799c // "  Wants to prove the media wrong for the fans"
            }
        } else {
            critical()
        }
    } else if exe::club_dislikes_staff(p, club) != 0 {
        if p.loyalty() > 0xf {
            0xa87974 // "  Wants to prove the supporters wrong"
        } else if p.loyalty() < 5 {
            0xa87944 // "  Has had enough of the fans' treatment of him"
        } else if c.grievances() == 0x40000 {
            0xa87910 // "  Wants the supporters to give him another chance"
        } else {
            0xa878d4 // "  Wants to move to a club with more appreciative supporters"
        }
    } else if exe::club_favours_staff(p, club) != 0 {
        if p.loyalty() > 0xf && p.pressure() > 5 {
            // the original writes this text and then overwrites it
            lang(JOB_TEXT, 0xa879fcusize);
        }
        if p.pressure() < 6 {
            0xa878a8 // "  Just wants the media to leave him alone"
        } else if p.loyalty() < 5 {
            0xa87874 // "  Unhappy with the media criticism he has recieved"
        } else {
            0xa87844 // "  Aims to prove the media wrong for the fans"
        }
    } else {
        critical()
    };
    st.say(s);
}

/// The manager's favour of `transfer_tab_mood_outlook` (the club's manager favours a person with no grievances).
pub(crate) unsafe fn transfer_tab_outlook_favoured(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let pl = p.player();
    let enthusiastic = 0xa86eec; // "  Enthusiastic about his future at the club"
    let convinced = 0xa86f18; // "  Convinced he will still make it as a professional"
    if !pl.is_null() {
        if p.loyalty() > 0xf
            && !st.future_now()
            && exe::staff_is_transfer_listed_or_requested(p, club) == 0
            && pl.current_reputation() as i32 > club.reputation() as i32 - 0x2ee
            && pl.current_ability() > 100i16.wrapping_sub(p.ambition() as i16)
            && club.reputation() < 0x128e
        {
            st.say(0xa8702c); // "  Wants to help the club fulfill its potential"
        }
        let rep = pl.current_reputation();
        if exe::media_has_response_two_for_player(read_usize(G_MEDIA), p) == 0 && (rep < 0xcb2 || rep < club.reputation()) && (p.age() as i8) < 0x18 {
            if exe::staff_is_transfer_listed_or_requested(p, club) == 0 {
                st.say(0xa86eb4); // "  Thinks his manager will help him develop as a player"
                lang(JOB_TEXT, enthusiastic); // written and not shown
                return;
            }
            if exe::staff_is_transfer_listed(p, 0usize) == 0 {
                st.say(enthusiastic);
                return;
            }
            if p.ambition() > 0xf {
                st.say(0xa87004); // "  Determined to prove his manager wrong"
                return;
            }
            let age = p.age() as i8;
            if age < 0x13 && pl.current_reputation() < 0x9c4 && p.ambition() < 6 {
                st.say(0xa86fc8); // "  Believes his ambition for a professional career is over"
                return;
            }
            let m = club.manager();
            if !m.is_null()
                && st.days_since(m.addr() + staff::DATE_JOINED_CLUB) > 0x258
                && age < 0x1c
                && st.days_joined() > 0x258
                && p.temperament() < 6
                && pl.aggression() > 0xf
            {
                st.say(0xa86f98); // "  Is angry that his mentor has abandoned him"
                return;
            }
            if p.professionalism() > 0xf {
                let g = (club.short_gender_name() as i8) as i32;
                lang(JOB_TEXT, 0xa86f4cusize, g, g, club.addr() + club::SHORT_NAME); // "  Realises that his career at {}<%s>{} is over"
                st.line();
                return;
            }
            if age < 0x13 && p.ambition() > 0xf && !pl.is_null() && pl.current_reputation() < 0x9c4 {
                st.say(convinced);
            }
            return;
        }
    }
    if exe::staff_is_transfer_listed(p, 0usize) != 0 && (p.age() as i8) < 0x13 && p.ambition() > 0xf && !pl.is_null() && pl.current_reputation() < 0x9c4 {
        st.say(convinced);
        return;
    }
    if exe::staff_is_transfer_listed(p, 0usize) == 0 {
        lang(JOB_TEXT, enthusiastic); // written and not shown
    }
}

/// The stay comment of `transfer_tab_mood_outlook` (not listed, not in dispute): favouring the club (player-manager,
/// staying on after playing, enthusiastic, enjoying life, impressing the new manager, enjoying playing, content),
/// with contract flags (considering options, willing to stay), committing his future, happy with the conditions, or
/// happy to stay.
pub(crate) unsafe fn transfer_tab_outlook_stay(st: &TransferTab) -> usize {
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let age = p.age() as i8;
    let happy = 0xa86cc4; // "  Happy to stay at the club"
    if exe::staff_favours_club(p, club) != 0 && p.loyalty() > 5 {
        let job = p.job_for_club();
        if job == 0xc {
            return 0xa86e50; // "  Happy in his role as player-manager"
        }
        if !p.non_player().is_null() && !p.player().is_null() && age > 0x20 && job == 0xb {
            return 0xa86e0c; // "  Would like to remain at the club after his playing career is over"
        }
        if age < 0x19 {
            return 0xa86eec; // "  Enthusiastic about his future at the club"
        }
        if exe::database_is_player_job(job as i8) == 0 {
            return 0xa86dec; // "  Is enjoying life at the club"
        }
        if st.days_joined() < 0x1e {
            return 0xa86dc8; // "  Wants to impress his new manager"
        }
        let enjoying = 0xa86da4; // "  Is enjoying playing for the club"
        if st.squad() == 1 || (st.squad() == 2 && c.happiness() > 0x19 && st.days_joined() > 0x2d) {
            return enjoying;
        }
        if st.squad() == 3 && c.unhappy_flag() > 1 {
            return happy;
        }
        if st.squad() == 3 {
            return enjoying;
        }
        return if age >= 0x1f { happy } else { 0xa86d84 }; // "  Content to stay at the club"
    }
    if st.flags() != 0 {
        return if p.loyalty() < 6 && p.professionalism() < 10 {
            0xa8827c // "  Considering his options"
        } else if p.loyalty() < 6 {
            0xa86d5c // "  Willing to stay at the club currently"
        } else {
            0xa86d3c // "  Willing to stay at the club"
        };
    }
    if club.is_null() {
        return happy;
    }
    let pl = p.player();
    if p.loyalty() > 0xf && c.morale() > 0x19 && !pl.is_null() && pl.current_reputation() < club.reputation() && age < 0x19 {
        return 0xa86d10; // "  Happy to commit his future to the club"
    }
    if p.loyalty() < 5
        && p.professionalism() < 5
        && !pl.is_null()
        && pl.current_reputation() > 0x109a
        && pl.current_reputation() as i32 > club.reputation() as i32 - 0x5dc
        && age < 0x1f
    {
        return 0xa86ce0; // "  Happy with conditions at the club currently"
    }
    happy
}

/// The outlook of a person with a future transfer (`transfer_tab_mood_outlook`): not in dispute and with no grievances,
/// with morale not below −50 − loyalty: the loan listing (gaining experience, unsure of the gain, too good to be
/// listed, disappointed) or insecure, jaded, wanting to extend, unsettled or unknown; with low morale: considering his
/// options, the loan listing, or unsure.
pub(crate) unsafe fn transfer_tab_outlook_moving(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let age = p.age() as i8;
    let unknown = 0xa86b14; // "  Unknown<%s - COMMENT - unknown future>"
    let unsure = 0xa86c7c; // "  Unsure"
    let experience = 0xa86e78; // "  Hoping to gain first team experience from going on loan"
    let not_sure = 0xa86c88; // "  Not sure what he will gain by going on loan"
    let too_good = 0xa86c3c; // "  Feels a player of his ability shouldn't be listed for loan"
    if exe::dispute_is_in_dispute(read_usize(G_DISPUTE), p, 0) == 0 && c.grievances() == 0 && c.morale() as i32 >= -0x32 - p.loyalty() as i32 {
        covered!("tel08_3 transfer tab outlook moving");
        let pl = p.player();
        let (s, comment) = if exe::staff_is_loan_listed(p, 0usize) != 0 {
            if age < 0x18 {
                if p.professionalism() > 0xf && p.pressure() > 10 && !pl.is_null() && pl.current_reputation() < 0xcb2 && !matches!(st.squad(), 4 | 1 | 2 | 3) {
                    (experience, false)
                } else if p.loyalty() < 6
                    && p.professionalism() < 10
                    && !pl.is_null()
                    && pl.current_ability() as i32 > p.professionalism() as i32 - p.ambition() as i32 + 100
                {
                    (not_sure, false)
                } else {
                    (unsure, false)
                }
            } else if !pl.is_null()
                && p.professionalism() < 10
                && p.ambition() > 0xf
                && pl.current_ability() as i32 > p.professionalism() as i32 - p.ambition() as i32 + 100
                && pl.current_reputation() > 0xcb2
            {
                (too_good, false)
            } else if p.professionalism() < 10 && p.temperament() < 10 {
                (0xa86c14, false) // "  Disappointed at being listed for loan"
            } else {
                (unknown, true)
            }
        } else if p.pressure() < 5 && age < 0x17 {
            (0xa86be0, true) // "  Feels insecure<%s - COMMENT - Insecure future>"
        } else if p.loyalty() < 5 && age > 0x1f {
            (0xa86bac, true) // "  Jaded<%s - COMMENT - Jaded opinion on situation>"
        } else if p.pressure() < 10
            && !pl.is_null()
            && pl.current_reputation() < 0xcb2
            && pl.current_reputation() < club.reputation().wrapping_add(0xfa)
            && st.days_to_expiry() < p.pressure() as i32 + 0x1e
        {
            (if age < 0x15 { 0xa86be0 } else { 0xa86b68 }, true) // "  Would like to extend his contract<%s - COMMENT - Insecure future>"
        } else if st.row != st.first {
            (0xa86b40, true) // "  Unsettled<%s - COMMENT - Unsettled>"
        } else {
            (unknown, true)
        };
        if comment {
            lang(JOB_TEXT, s, TRANSFER_TAB_COMMENT);
        } else {
            lang(JOB_TEXT, s);
        }
        st.line();
        return;
    }
    if exe::dispute_is_in_dispute(read_usize(G_DISPUTE), p, 0) != 0 || c.grievances() != 0 {
        return;
    }
    let s = if c.morale() < -75 && p.temperament() < 0xf && p.professionalism() < 10 && p.ambition() > 5 && !st.future_now() {
        0xa8827c // "  Considering his options"
    } else if exe::staff_is_loan_listed(p, 0usize) != 0 {
        if age < 0x18 {
            if p.professionalism() > 0xf && p.pressure() > 10 {
                experience
            } else if p.loyalty() < 6 && p.professionalism() < 10 {
                not_sure
            } else {
                unsure
            }
        } else if p.professionalism() < 10 && p.ambition() > 0xf && p.player().current_reputation() > 0xcb2 {
            too_good
        } else if p.professionalism() < 10 && p.temperament() < 10 {
            0xa86af0 // "  Upset at being listed for loan"
        } else {
            lang(JOB_TEXT, unknown, TRANSFER_TAB_COMMENT);
            st.line();
            return;
        }
    } else {
        unsure
    };
    st.say(s);
}

/// The favoured team-mate text of a team-mate under contract at the club (see `transfer_tab_favoured_team_mate`).
pub(crate) unsafe fn transfer_tab_favoured_opinion(st: &TransferTab, fav: Staff, fc: Contract) -> usize {
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let fage = fav.age() as i8;
    let fp = fav.player();
    if (p.age() as i8) < 0x18 && p.pressure() > 10 && exe::staff_get_natural_position_mask(p) == exe::staff_get_natural_position_mask(fav) {
        return 0xa8a528; // "  Thinks he can learn a lot from <%s>"
    }
    if fc.unhappy_flag() > 2 && fc.happiness() < 0x32 {
        return 0xa8a4c0; // "  Thinks that <%s> should play a more important role at the club"
    }
    if (fage > 0x1c || (fage > 0x18 && fav.int_apps() > 10))
        && fp.current_reputation() > 0x1c52
        && (p.age() as i8) < 0x19
        && fc.happiness() > 0
        && fc.matches_left_out() as i8 > 0
        && c.happiness() < 0
        && p.player().current_reputation() < 0x128e
    {
        return if c.matches_left_out() != 0 {
            0xa8a470 // "  Honoured to be playing alongside <%s>"
        } else if p.playing_squad() == fav.playing_squad() {
            0xa8a420 // "  Honoured to be training alongside <%s>"
        } else {
            0xa8a3cc // "  Honoured to be at the same club as <%s>"
        };
    }
    if fage < 0x1d && fp.current_reputation() > 0x1c52 && club.reputation() < 0x1a5e {
        return 0xa8a384; // "  Hopes the club can hang onto <%s>"
    }
    let pp = p.player();
    if !pp.is_null()
        && fage > 0x1a
        && fc.happiness() > 0
        && fp.leadership() > 0xf
        && fp.current_reputation() > 0xea6
        && (fp.current_reputation() > club.reputation().wrapping_sub(0x4e2) || fp.world_reputation() > 0x1676)
        && pp.leadership() < 0xe
    {
        return 0xa8a33c; // "  Thinks <%s> is an inspired leader"
    }
    if fc.happiness() > 0x14 {
        0xa8a2ec // "  Thinks <%s> is an essential squad member"
    } else {
        0xa8a29c // "  Thinks <%s> has a lot to offer the club"
    }
}

/// "%s %s %d.%d.%d" (first name, second name, day, month, year: the written history's key of a person), the empty
/// string, "Player Nickname" and the two scout request questions ("Have <%s> assessed by your scouts ?", "Have <%s>
/// watched when your scouts become available ?"), "Remove", "Cancel", "Ok", "No" and "Yes".
pub(crate) const STAFF_HISTORY_KEY: usize = 0x9ced08;

pub(crate) const STAFF_NO_NAME: usize = 0x989ad8;

pub(crate) const STAFF_NICKNAME_TITLE: usize = 0xa8b35c;

pub(crate) const STAFF_SCOUT_ASSESS: usize = 0xa8b3c4;

pub(crate) const STAFF_SCOUT_WATCH: usize = 0xa8b36c;

pub(crate) const STAFF_REMOVE: usize = 0xa8b354;

pub(crate) const STAFF_CANCEL: usize = 0x98a1a0;

pub(crate) const STAFF_OK: usize = 0x990f7c;

pub(crate) const STAFF_NO: usize = 0x98e8cc;

pub(crate) const STAFF_YES: usize = 0x98e8d0;

/// The nickname dialog's ids: the text entry, Ok, Cancel and Remove; the scout request's No and Yes.
pub(crate) const NICKNAME_ENTRY: u16 = 0x32;

pub(crate) const NICKNAME_OK: u16 = 0x33;

pub(crate) const NICKNAME_CANCEL: u16 = 0x34;

pub(crate) const NICKNAME_REMOVE: u16 = 0x35;

pub(crate) const SCOUT_REQUEST_NO: u16 = 0x36;

pub(crate) const SCOUT_REQUEST_YES: u16 = 0x37;

/// `HUMAN_MANAGER` methods of the nicknames (thiscall on [G_HUMAN_MANAGER]): remove (person, 0) ret 8, change
/// (person, nickname, 0) ret 0xc, add (person, nickname, 0) ret 0xc [inference: the names, from the dialog].
pub(crate) const FN_HUMAN_MANAGER_REMOVE_NICKNAME: usize = 0x6003d0;

pub(crate) const FN_HUMAN_MANAGER_CHANGE_NICKNAME: usize = 0x600870;

pub(crate) const FN_HUMAN_MANAGER_ADD_NICKNAME: usize = 0x600090;

/// `SCOUT_MANAGER::add_report_request` (0x7f9290, thiscall on [G_SCOUT_MANAGER], (person, 0), ret 8) [inference].
pub(crate) const FN_SCOUT_MANAGER_ADD_REPORT_REQUEST: usize = 0x7f9290;

/// The screen's shared text buffer (the one every builder formats into before handing it to the display list).
pub(crate) const PROFILE_BUF: usize = 0xde1f64;

/// A dword the profile builder clears on entry (staff_screens.cpp; read by the profile's handler) [inference].
pub(crate) const PROFILE_SCREEN_RESET: usize = 0xa84aa8;

/// One `MenuItem` of the profile's tab bar: 0xd4 bytes (25 of them, built and destroyed by the CRT vector iterators
/// with 0x4568f0 / 0x5a53c0): +0 the word −1, +2 the colour word, +4 the text, +0xcc the style (0x11 shown, 0 greyed),
/// +0xd0 the tab number.
pub(crate) const PROFILE_TAB_ITEM: usize = 0xd4;

pub(crate) const PROFILE_TAB_COUNT: usize = 25;

/// `Text::draw_wrapped` (0x4aeaf0, thiscall, ret 0x20): a Text into area `slot` from `left` to `right`, starting at row
/// `*line` (moved on), with the font, the colour, the style and one more flag.
pub(crate) type ProfileDrawFn = unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i32, usize, usize, usize, usize) -> usize;

/// Everything the profile builder keeps in its frame: the params of the screen (0..8 and 0x14), the viewer (the
/// screen record's human, a person among the 16 last staff records), the kit colours, the contract and the offer of
/// the viewer's club, the coordinates of the next block and the buffers.
pub(crate) struct ProfileScreen {
    pub(crate) staff: usize,
    pub(crate) tab: i8,
    pub(crate) view: u8,
    pub(crate) list: usize,
    pub(crate) mode: i32,
    pub(crate) flags: u32,
    pub(crate) history: usize,
    pub(crate) history_index: i32,
    pub(crate) history_row: usize,
    pub(crate) compare_columns: usize,
    pub(crate) compare1: usize,
    pub(crate) compare2: usize,
    pub(crate) viewer: usize,
    pub(crate) viewer_club: usize,
    pub(crate) viewer_nation_team: usize,
    pub(crate) contract: usize,
    pub(crate) offer: usize,
    pub(crate) fg: u32,
    pub(crate) bg: u32,
    pub(crate) y: i32,
    pub(crate) bottom: i32,
    pub(crate) text: [usize; 2],
    pub(crate) name: [u8; 0xc8],
    pub(crate) action: [u8; 0x100],
}

/// What the profile builder will create or change in the game, read before it runs (the same rules as
/// `profile_screen_setup`, written apart to check the original afterwards): the offer list (param 3 null), a staff
/// contract (none, a non-player sheet, in the database, a club and a contract job), the career history (tab 5, bit 2
/// of param 5 after its masks, no param 6, a player sheet) and param 5 after its masks.
pub(crate) struct ProfilePlan {
    pub(crate) staff: usize,
    pub(crate) tab: u8,
    pub(crate) list: bool,
    pub(crate) contract: bool,
    pub(crate) history: bool,
    pub(crate) flags: u32,
}

/// The attribute rows of the Profile tab (shared with the compare players screen, the same table at 0xddf570): the
/// name (`database_get_player_attribute_name` / `database_get_non_player_attribute_name`) and, at +0x64, the value
/// or the code of an extra row (0xff preferred foot, 0xfe form, 0xfd morale, 0xfc condition, 0xfb on loan, 0xfa
/// coaching style). Rows of 0x65 bytes; 31 player attributes plus 5 extra rows.
pub(crate) const PROFILE_TABLE: usize = 0xddf570;

pub(crate) const PROFILE_ROW: usize = 0x65;

pub(crate) const PROFILE_ROW_VALUE: usize = 0x64;

pub(crate) const PROFILE_PLAYER_ROWS: usize = 0x24;

/// The qsort comparer of the attribute rows (by name).
pub(crate) const PROFILE_ROW_COMPARER: usize = 0x88bc50;

/// Colour of the loan and free transfer lines (a word).
pub(crate) const PROFILE_COLOUR_3152: usize = 0xae3152;

/// `staff_state::STATUS_FLAGS` bit 0x40 (the regen mark of docs/MARCA_DO_REGEN.md): here it also gates the
/// chairman table and lets a chairman see the formation and style table [inference: what the bit means for a
/// non-player].
pub(crate) const PROFILE_STATE_SHOWN: u8 = 0x40;

/// The game's empty text, the argument of the "<%s - COMMENT ...>" formats.
pub(crate) const TRANSFER_TAB_COMMENT: usize = 0xad9c64;

/// The staff_screens.cpp path of the error box.
pub(crate) const TRANSFER_TAB_SOURCE: usize = 0xa84aac;

/// Byte +4 of a club's row in `G_CLUB_STATE_TABLE` (6 bytes per club id): the league form against the expectations,
/// signed (> 2 above, < 0 below) [inference].
pub(crate) const TRANSFER_TAB_CLUB_FORM: usize = 4;

/// Byte +5 of a club's row in `G_CLUB_STATE_TABLE`: nonzero when the dressing room is unsettled [inference].
pub(crate) const TRANSFER_TAB_CLUB_UNREST: usize = 5;

/// What the Transfer tab carries from one piece to the next: the area and the row being written, the first row of the
/// current section (the empty label cells go below it), the person, his contract and its club, the viewer's club, the
/// future transfer the Offers section asked for, the list of interested clubs and the five bytes of the club's league
/// statistics entry (matches played, current and final position, losing and winning run).
pub(crate) struct TransferTab {
    pub(crate) area: u32,
    pub(crate) row: i32,
    pub(crate) first: i32,
    pub(crate) p: Staff,
    pub(crate) contract: Contract,
    pub(crate) club: Club,
    pub(crate) viewer: Club,
    pub(crate) future: usize,
    pub(crate) list: usize,
    pub(crate) matches_played: i8,
    pub(crate) position: i8,
    pub(crate) final_position: i8,
    pub(crate) losing_run: i8,
    pub(crate) winning_run: i8,
}

impl TransferTab {
    /// A cell of the current row in column `col`.
    pub(crate) unsafe fn cell(&self, col: u32, colour: usize, text: usize) {
        staff_tab_cell(self.area, col, self.row, 2, 0, 1, 2, colour, text);
    }
    /// The label (column 0, pale grey) of the current row.
    pub(crate) unsafe fn label(&self, text: usize) {
        self.cell(0, G_GUI_COLOUR_PALE_GREY, text);
    }
    /// The value (column 1, yellow) of the current row.
    pub(crate) unsafe fn value(&self, text: usize) {
        self.cell(1, G_GUI_COLOUR_YELLOW, text);
    }
    /// The screens' buffer as a yellow button of the current row in column 1.
    pub(crate) unsafe fn button(&self, id: u32, value: usize) {
        staff_tab_button(self.area, 1, self.row, 2, 0, 1, 2, G_GUI_COLOUR_YELLOW, JOB_TEXT, id, value as u32);
    }
    /// The empty label of a row below the first of the section.
    pub(crate) unsafe fn pad(&self) {
        if self.row > self.first {
            self.label(0);
        }
    }
    /// The screens' buffer as a wrapped text of the current row (`add_area_text_by_ref`, 0x804160: area, column 1,
    /// the row by address, the style, the colour word, the text, width 0x209 and the "  " indent).
    pub(crate) unsafe fn wrapped(&mut self, colour: usize) {
        let f: unsafe extern "thiscall" fn(usize, u32, u32, *mut i32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT_BY_REF);
        f(G_SCREEN_MANAGER, self.area, 1, &mut self.row, 2, 0, 1, 2, job_colour(colour), JOB_TEXT as u32, 0x209, TRANSFER_TAB_INDENT as u32);
    }
    /// A line of the Future section: the wrapped text in `colour`, the empty label and the next row.
    pub(crate) unsafe fn line_in(&mut self, colour: usize) {
        self.wrapped(colour);
        self.pad();
        self.row += 1;
    }
    /// A yellow line of the Future section.
    pub(crate) unsafe fn line(&mut self) {
        self.line_in(G_GUI_COLOUR_YELLOW);
    }
    /// The translated text `s` as a yellow line of the Future section.
    pub(crate) unsafe fn say(&mut self, s: usize) {
        staff_tab_lang()(JOB_TEXT, s);
        self.line();
    }
    /// Days from the date at `date` (a TCMDate) to today (`Date::days_since` on today).
    pub(crate) unsafe fn days_since(&self, date: usize) -> i32 {
        exe::date_days_since(G_CURRENT_DATE, read_u32(date), read_u32(date + 4))
    }
    /// Days since the person joined his club.
    pub(crate) unsafe fn days_joined(&self) -> i32 {
        self.days_since(self.p.addr() + staff::DATE_JOINED_CLUB)
    }
    /// Days since the contract started.
    pub(crate) unsafe fn days_started(&self) -> i32 {
        self.days_since(self.contract.addr() + contract::DATE_STARTED)
    }
    /// Days from today to the contract's expiry (`Date::days_since` on the expiry date).
    pub(crate) unsafe fn days_to_expiry(&self) -> i32 {
        exe::date_days_since(self.contract.addr() + contract::CONTRACT_EXPIRES, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP))
    }
    /// Whether the person has a future transfer (asked again, without keeping it).
    pub(crate) unsafe fn future_now(&self) -> bool {
        exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), self.p, 0) != 0
    }
    /// Byte `k` of the club's row in `G_CLUB_STATE_TABLE`.
    pub(crate) unsafe fn club_state(&self, k: usize) -> i8 {
        read_i8(read_usize(G_CLUB_STATE_TABLE).wrapping_add((read_i32(self.club.addr()) as isize * 6) as usize) + k)
    }
    /// The squad status bits of the contract (the high nibble of `contract::SQUAD_STATUS`, in place).
    pub(crate) unsafe fn squad_bits(&self) -> u8 {
        self.contract.squad_status() & 0xf0
    }
    /// The squad status of the contract (`staff_contract_get_squad_status`, 0x8857d0: the high nibble).
    pub(crate) unsafe fn squad(&self) -> u8 {
        self.contract.squad_status() >> 4
    }
    /// `STAFF_CONTRACT::get_flags`.
    pub(crate) unsafe fn flags(&self) -> u32 {
        exe::staff_contract_get_flags(self.contract)
    }
    /// The squad status the person deserves (`contract_manager_get_deserved_squad_status`).
    pub(crate) unsafe fn deserved(&self) -> u8 {
        exe::contract_manager_get_deserved_squad_status(read_usize(G_CONTRACT_MANAGER), self.p, self.contract)
    }
}

/// Colour word 0xae3152 of the screen items (the season fee and the career totals).
pub(crate) const HISTORY_TAB_COLOUR_3152: usize = 0xae3152;

/// The pending network command (0xdd6be0; its argument is `G_PENDING_COMMAND_ARG`).
pub(crate) const STAFF_PENDING_COMMAND: usize = 0xdd6be0;

/// The default title colour of the compare dialog (word).
pub(crate) const JOB_COLOUR_BDF0: usize = 0xaebdf0;

/// The "  " prefix the wrapped lines of the Transfer tab pass to `add_area_text_by_ref`.
pub(crate) const TRANSFER_TAB_INDENT: usize = 0xa78880;

/// Builds the compare players screen: the two compare players of the scouting manager
/// (`SCOUT_MANAGER::get_compare_player` 1 and 2; with no first one the screen's player, param 0; with neither,
/// nothing), the heading (`compare_screen_heading`), for a first player who is a player of the database the position
/// table (`compare_screen_positions`, `compare_screen_position_table`), the statistics table
/// (`compare_screen_statistics`) and the Back/Next buttons (`gui_add_back_next_buttons` with "back possible" and
/// "next possible"). Params read: 0, 4 (the view mode); 1, 2, 3, 5..8 and 0x14 read and ignored.
///
/// Convention: cdecl, SEH frame (0x2644 bytes of locals): a Text and an array of 25 `MenuItem` (built and destroyed,
/// never used). Split into named pieces; the whole routine is hooked.
/// Original: `staff_screens_build_compare_players_screen` (0x876520, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_build_compare_players_screen() {
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0);
    let mut menu = [0u8; 25 * 0xd4];
    exe::crt_vector_constructor_iterator(menu.as_mut_ptr() as usize, 0xd4, 0x19, 0x4568f0, 0x5a53c0);
    let align: [u8; 8] = [3, 3, 3, 0, 3, 3, 3, 3];
    let legend_align: u8 = 1;

    let mut p1 = compare_screen_player(1);
    let p2 = compare_screen_player(2);
    let param0 = get_screen_param(0);
    for k in [1, 2, 3] {
        get_screen_param(k);
    }
    let mode = get_screen_param(4) as u8 as i8 as i32;
    for k in [5, 6, 7, 8, 0x14] {
        get_screen_param(k);
    }
    let (better2, same) = compare_screen_colours();
    let better1 = colour_word(0xae31aa);
    if p1 == 0 && p2 == 0 {
        covered!("tel07_2 compare screen with nobody");
    } else {
        covered!("tel07_2 compare screen");
        if p1 == 0 {
            p1 = param0;
        }
        compare_screen_heading(t, p1, p2, mode);
        let mut y = 0x1fe;
        if read_usize(p1 + staff::PLAYER) != 0 && (p1 == 0 || read_i32(p1) < read_i32(G_STAFF_COUNT)) {
            compare_screen_positions(p1, p2);
            y = compare_screen_position_table(p1, p2, &align, &legend_align, better1, same, better2);
        }
        compare_screen_statistics(y);
        let next = (exe::screenmanager_is_forward_unavailable(SCREEN_MANAGER) == 0) as u32;
        let back = (exe::screenmanager_is_back_unavailable(SCREEN_MANAGER, 0) == 0) as u32;
        exe::gui_add_back_next_buttons(back, next);
    }
    exe::crt_vector_destructor_iterator(menu.as_mut_ptr() as usize, 0xd4, 0x19, 0x5a53c0);
    exe::drop_text(t);
}

/// Comparer for `staff_screens_build_compare_players_screen`: both build side by side and are compared by the screen
/// (the display list, `compare_match_screen_build`) plus what the screen writes outside it: the compare tables
/// (names, values, differences and masked marks), the position words and marks and the contract pointer 0xae1080.
/// When the first player's training is due (`training_update_pending`) only the original runs: the week's training
/// rolls once and the copy would start from another state.
pub(crate) unsafe extern "C" fn compare_staff_screens_build_compare_players_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_STAFF_SCREENS_BUILD_COMPARE_PLAYERS_SCREEN);
    let c1 = compare_screen_player(1);
    if training_update_pending(if c1 != 0 { c1 } else { get_screen_param(0) }) {
        // simplification: the training roll runs once, in the original
        return o();
    }
    let tables = 0x27 * COMPARE_ROW;
    let extra = [
        (COMPARE_MASKED, tables),
        (COMPARE_POS_AM, 4),
        (COMPARE_TABLE_SECOND, tables),
        (COMPARE_TABLE_DIFF, tables),
        (COMPARE_TABLE_FIRST, tables),
        (COMPARE_POS_GK, 0x24),
        (0xae1080, 4),
    ];
    compare_match_screen_build("compare players screen", &extra, || o(), || staff_screens_build_compare_players_screen());
}

/// The heading and the identity lines of the compare players screen: the background photo of the first player, the
/// two names ("%s ~ %s", each linked 0x7d7), the menu bar, the ages and nationalities ("%s. %s ~ %s. %s", the
/// nations linked 0x7d5 to their national teams) and the positions ("%s ~ %s" of
/// `database_format_player_positions`) or, for a non-player or in mode 1, the first one's job ("Unemployed").
/// Original: part of the compare players screen builder (0x876520, staff_screens.cpp).
pub(crate) unsafe fn compare_screen_heading(text: *mut usize, p1: usize, p2: usize, mode: i32) {
    let p1 = Staff(p1);
    let p2 = Staff(p2);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let draw: TextDrawWrappedFn = core::mem::transmute(FN_TEXT_DRAW_WRAPPED);
    let positions: unsafe extern "C" fn(usize, usize, u32, u32) -> usize = core::mem::transmute(FN_DATABASE_FORMAT_PLAYER_POSITIONS);
    let text_item: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    const BUF: usize = 0xde1f64;
    let (mut name1, mut name2) = ([0u8; 0xc8], [0u8; 0xc8]);
    let (n1, n2) = (name1.as_mut_ptr() as usize, name2.as_mut_ptr() as usize);

    exe::background_set_staff_picture(p1, 0, 0);
    exe::database_format_staff_name(p1, n1, 0xc8, 1);
    if !p2.is_null() {
        exe::database_format_staff_name(p2, n2, 0xc8, 1);
    }
    // simplification: with no second player the original prints its stack garbage after the "~"; here empty
    format(BUF, 0xa859d0, n1, n2); // "%s ~ %s"
    exe::text_set_string(text as usize, BUF as *const u8);
    if !p1.is_null() {
        exe::text_add_link(text as usize, n1 as *const u8, 0x7d7, p1);
    }
    if !p2.is_null() {
        exe::text_add_link(text as usize, n2 as *const u8, 0x7d7, p2);
    }
    // the colours are words in dword slots: the high half is stack garbage in the original, here 0
    let red = colour_word(G_GUI_COLOUR_RED);
    let grey = colour_word(G_GUI_COLOUR_PALE_GREY);
    compare_screen_area(0x64, 0xa, 0x316, 0x46, 1, 0, 1, 0, 0x30, red);
    let wide = exe::font_text_width(4, BUF as *const u8) >= 0x2b2;
    let mut row: i32 = 0;
    let a = compare_screen_area(0x64, 0x19, 0x316, 0x37, 1, 0, 1, 0, 1, red);
    draw(text, a, 0x64, 0x316, &mut row, if wide { 6 } else { 7 }, grey, 3, 0);
    exe::menubar_build(4, 0);
    // the date of birth goes into the first name buffer and is never shown (the positions overwrite it)
    (exe::date_get_short_date_string(p1.addr() + staff::DATE_OF_BIRTH, n1, 0xc8, 0xff) as usize);

    let (mut age1, mut age2) = ([0u8; 0xc8], [0u8; 0xc8]);
    lang(age1.as_mut_ptr() as usize, 0xa859c0, (p1.age() as i8) as i32); // "Age <%d - age>"
    lang(age2.as_mut_ptr() as usize, 0xa859c0, (p2.age() as i8) as i32);
    let (mut nat1, mut nat2, mut extra1, mut extra2) = ([0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8]);
    compare_screen_nationality(p1.addr(), &mut nat1, &mut extra1);
    compare_screen_nationality(p2.addr(), &mut nat2, &mut extra2);
    format(BUF, 0xa859b0, age1.as_ptr() as usize, nat1.as_ptr() as usize, age2.as_ptr() as usize, nat2.as_ptr() as usize); // "%s. %s ~ %s. %s"
    exe::text_clear(text);
    exe::text_set_string(text as usize, BUF as *const u8);
    // the links carry the lower-case adjective (the line has it capitalised), as in the original
    for (p, buf) in [(p1.addr(), &mut nat1), (p2.addr(), &mut nat2)] {
        let nation = read_usize(p + staff::NATION);
        if nation != 0 {
            exe::nation_get_nationality_adjective(nation, buf.as_mut_ptr() as usize, 0xc8, 0xc);
            let team = exe::database_get_national_team(nation);
            exe::text_add_link(text as usize, buf.as_ptr(), 0x7d5, team);
        }
    }
    let a = compare_screen_area(0x6e, 0x50, 0x30c, 0x6e, 1, 0, 1, 0, 2, 0);
    row = 0;
    draw(text, a, 0x6e, 0x30c, &mut row, 3, colour_word(G_GUI_COLOUR_YELLOW), 3, 0);

    if p1.player().addr() != 0 && mode != 1 {
        positions(p1.player().addr(), n1, 0x7d0, 1);
        positions(p2.player().addr(), n2, 0x7d0, 1);
        format(BUF, 0xa859d0, n1, n2); // "%s ~ %s"
    } else if p1.club_job().addr() != 0 {
        compare_screen_strcpy(BUF, exe::database_get_job_name(p1.job_for_club() as u32, 0, 1, core::ptr::null_mut()));
    } else if p1.national_job().addr() != 0 {
        compare_screen_strcpy(BUF, exe::database_get_job_name(p1.job_for_nation() as u32, 0, 1, core::ptr::null_mut()));
    } else {
        lang(BUF, 0x9b6b04); // "Unemployed"
    }
    text_item(SCREEN_MANAGER, 0x6e, 0x203, 0x30c, 0x221, 2, 0, 0xc, 3, colour_word(0xaebde8), BUF, 1);
}

/// The position table of the compare screen (the first player is a player of the database): the position words and
/// marks from the two player sheets (`compare_screen_position`; the full back and the winger by their own rules), then
/// the 31 attributes into the three tables, each adding its difference to the positions it matters for, and the two
/// partial sums (defence, attack) spread over the outfield positions.
///
/// Full back (the defender rating with a side, left or right): "n/a" when neither is a defender or nobody has a side;
/// 35 when the first is a full back and the second has no side and is no defender; −35 when the second is a full back,
/// the first has no side and is no defender; otherwise twice the defender difference, except that a first player with
/// a side but no defender rating against a second with both gives 0 and "n/a". Winger (midfielder or attacking
/// midfielder with a side): "n/a" when neither, ±35 when only one, twice the attacking midfielder difference when both.
/// Attribute values: with masking on (`G_ATTRIBUTE_MASKING_ENABLED`), what the viewer (the screen record's human,
/// `G_SCREEN_RECORDS_MANAGER` of the slot) knows (`staff_get_player_attribute_for_viewer`, negative = masked: the row
/// is marked and adds nothing); otherwise `exe::staff_compute_player_attribute`.
/// Original: part of the compare players screen builder (0x876520, staff_screens.cpp).
pub(crate) unsafe fn compare_screen_positions(p1: usize, p2: usize) {
    let p1 = Staff(p1);
    let p2 = Staff(p2);
    let (a, b) = (p1.player().addr(), p2.player().addr());
    let ra = |o: usize| read_i8(a + o);
    let rb = |o: usize| read_i8(b + o);
    for w in [COMPARE_POS_GK, COMPARE_POS_CB, COMPARE_POS_FB, COMPARE_POS_SW, COMPARE_POS_WB, COMPARE_POS_DM, COMPARE_POS_MC, COMPARE_POS_AM, COMPARE_POS_WG, COMPARE_POS_DEFENCE, COMPARE_POS_ATTACK, COMPARE_POS_ST] {
        write_u16(w, 0);
    }
    for m in [COMPARE_MARK_GK, COMPARE_MARK_CB, COMPARE_MARK_FB, COMPARE_MARK_SW, COMPARE_MARK_WB, COMPARE_MARK_DM, COMPARE_MARK_MC, COMPARE_MARK_AM, COMPARE_MARK_WG, COMPARE_MARK_ST] {
        write_u8(m, 1);
    }
    write_u32(COMPARE_BOTH_GOALKEEPERS, 0);
    let viewer = read_usize(G_SCREEN_RECORDS_MANAGER + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300);
    const T: i8 = 15;
    use player_sheet as ps;
    compare_screen_position(ra(ps::GOALKEEPER), rb(ps::GOALKEEPER), COMPARE_POS_GK, COMPARE_MARK_GK, COMPARE_BOTH_GOALKEEPERS);
    compare_screen_position(ra(ps::DEFENDER), rb(ps::DEFENDER), COMPARE_POS_CB, COMPARE_MARK_CB, COMPARE_MARK_CB);
    compare_screen_position(ra(ps::SWEEPER), rb(ps::SWEEPER), COMPARE_POS_SW, COMPARE_MARK_SW, COMPARE_MARK_SW);

    // the full back (0x876fc1..0x8770b6 of the original, branch for branch)
    let (ad, al, ar) = (ra(ps::DEFENDER), ra(ps::LEFT_SIDE), ra(ps::RIGHT_SIDE));
    let (bd, bl, br) = (rb(ps::DEFENDER), rb(ps::LEFT_SIDE), rb(ps::RIGHT_SIDE));
    let a_side = al >= T || ar >= T;
    let b_side = bl >= T || br >= T;
    'full_back: {
        if (ad < T && bd < T) || (!a_side && !b_side) {
            write_u8(COMPARE_MARK_FB, 0);
            break 'full_back;
        }
        if ad >= T && a_side && bl < T && br < T && bd < T {
            write_u16(COMPARE_POS_FB, 0x23);
            break 'full_back;
        }
        let mut difference = false;
        if bd >= T && b_side && !a_side {
            if ad >= T {
                difference = true;
            } else {
                write_u16(COMPARE_POS_FB, 0xffdd);
                break 'full_back;
            }
        }
        if !difference {
            difference = ad >= T || !a_side || bd >= T || !b_side;
        }
        if difference {
            write_u8(COMPARE_MARK_FB, 1);
            write_u16(COMPARE_POS_FB, ((ad as i16 - bd as i16) * 2) as u16);
        } else {
            write_u16(COMPARE_POS_FB, 0);
            write_u8(COMPARE_MARK_FB, 0);
        }
    }
    compare_screen_position(ra(ps::WING_BACK), rb(ps::WING_BACK), COMPARE_POS_WB, COMPARE_MARK_WB, COMPARE_MARK_WB);
    compare_screen_position(ra(ps::MIDFIELDER), rb(ps::MIDFIELDER), COMPARE_POS_MC, COMPARE_MARK_MC, COMPARE_MARK_MC);
    compare_screen_position(ra(ps::DEFENSIVE_MIDFIELDER), rb(ps::DEFENSIVE_MIDFIELDER), COMPARE_POS_DM, COMPARE_MARK_DM, COMPARE_MARK_DM);
    compare_screen_position(ra(ps::ATTACKING_MIDFIELDER), rb(ps::ATTACKING_MIDFIELDER), COMPARE_POS_AM, COMPARE_MARK_AM, COMPARE_MARK_AM);
    // the winger (0x87728f..0x8773a4)
    let winger = |m: i8, am: i8, l: i8, r: i8| (m >= T || am >= T) && (l >= T || r >= T);
    let wa = winger(ra(ps::MIDFIELDER), ra(ps::ATTACKING_MIDFIELDER), ra(ps::LEFT_SIDE), ra(ps::RIGHT_SIDE));
    let wb = winger(rb(ps::MIDFIELDER), rb(ps::ATTACKING_MIDFIELDER), rb(ps::LEFT_SIDE), rb(ps::RIGHT_SIDE));
    match (wa, wb) {
        (false, false) => write_u8(COMPARE_MARK_WG, 0),
        (true, false) => write_u16(COMPARE_POS_WG, 0x23),
        (false, true) => write_u16(COMPARE_POS_WG, 0xffdd),
        (true, true) => {
            write_u8(COMPARE_MARK_WG, 1);
            write_u16(COMPARE_POS_WG, ((ra(ps::ATTACKING_MIDFIELDER) as i16 - rb(ps::ATTACKING_MIDFIELDER) as i16) * 2) as u16);
        }
    }
    compare_screen_position(ra(ps::ATTACKER), rb(ps::ATTACKER), COMPARE_POS_ST, COMPARE_MARK_ST, COMPARE_MARK_ST);

    // the 31 attributes (rows 0..30)
    let add = |w: usize, v: i16| write_u16(w, read_u16(w).wrapping_add(v as u16));
    for row in 0..31u8 {
        let attr = row as u32 + 1;
        let k = row as usize * COMPARE_ROW;
        // simplification: the original pushes the attribute as a dword whose high bytes are stack garbage (the
        // variable is a char, the callees read the byte); here zero-extended
        (exe::database_get_player_attribute_name(attr, COMPARE_TABLE_FIRST + k, 0x64, 0) as usize);
        write_u8(COMPARE_MASKED + k, 0);
        if read_u8(G_ATTRIBUTE_MASKING_ENABLED) != 0 {
            if !p1.is_null() && !p2.is_null() {
                let v2 = exe::staff_get_player_attribute_for_viewer(viewer, p2, attr);
                let v1 = exe::staff_get_player_attribute_for_viewer(viewer, p1, attr);
                write_u8(COMPARE_TABLE_DIFF + 0x64 + k, v1.wrapping_sub(v2));
                write_u8(COMPARE_TABLE_SECOND + 0x64 + k, exe::staff_get_player_attribute_for_viewer(viewer, p2, attr));
                let v1 = exe::staff_get_player_attribute_for_viewer(viewer, p1, attr);
                write_u8(COMPARE_TABLE_FIRST + 0x64 + k, v1);
                write_u8(COMPARE_MASKED + k, ((v1 as i8) < 0) as u8);
                if read_i8(COMPARE_TABLE_SECOND + 0x64 + k) < 0 {
                    write_u8(COMPARE_MASKED + k, 1);
                }
            }
        } else if !p1.is_null() && !p2.is_null() {
            let v2 = exe::staff_compute_player_attribute(p2, p2.player().addr(), attr);
            let v1 = exe::staff_compute_player_attribute(p1, p1.player().addr(), attr);
            write_u8(COMPARE_TABLE_DIFF + 0x64 + k, v1.wrapping_sub(v2));
            write_u8(COMPARE_TABLE_SECOND + 0x64 + k, exe::staff_compute_player_attribute(p2, p2.player().addr(), attr));
            write_u8(COMPARE_TABLE_FIRST + 0x64 + k, exe::staff_compute_player_attribute(p1, p1.player().addr(), attr));
        }
        if read_u8(COMPARE_MASKED + k) != 0 {
            continue;
        }
        let d = read_i8(COMPARE_TABLE_FIRST + 0x64 + k) as i16 - read_i8(COMPARE_TABLE_SECOND + 0x64 + k) as i16;
        if matches!(row, 0 | 2 | 0x10 | 0x14 | 0x15 | 0x16 | 0x17 | 0x1a) {
            add(COMPARE_POS_GK, d);
        }
        if row == 0xd {
            add(COMPARE_POS_GK, d * 2); // tel07_2 the goalkeeper's doubled attribute
            continue;
        }
        if matches!(row, 0xe | 0x10 | 0x14 | 0x19 | 0x16 | 0x1a) {
            add(COMPARE_POS_DEFENCE, d);
        }
        if matches!(row, 0x12 | 0x1b) {
            add(COMPARE_POS_CB, d * 2);
        }
        if matches!(row, 0x12 | 0x15 | 0) {
            add(COMPARE_POS_FB, d * 2);
        }
        if matches!(row, 0x12 | 0x15 | 0x1b) {
            add(COMPARE_POS_SW, d * 2);
        }
        if matches!(row, 0 | 0x15 | 7) {
            add(COMPARE_POS_WB, d * 2);
        }
        if matches!(row, 0 | 0xe | 0x10 | 0x14 | 0x19 | 0x1a | 9 | 0x1c) {
            add(COMPARE_POS_ATTACK, d);
        }
        if matches!(row, 0x1b | 0x12 | 0x15) {
            add(COMPARE_POS_DM, d * 2);
        }
        if matches!(row, 0xa | 0x15 | 0x19 | 0x1d) {
            add(COMPARE_POS_MC, d * 2);
        }
        if matches!(row, 6 | 0xa | 0x13 | 0x11 | 0x15) {
            add(COMPARE_POS_AM, d * 2);
        }
        if matches!(row, 6 | 7 | 0x14 | 0xa) {
            add(COMPARE_POS_WG, d * 2);
        }
        if matches!(row, 6 | 7 | 0xa | 0xc | 0x13 | 0x15 | 0xe | 0x10 | 0x18) {
            add(COMPARE_POS_ST, d);
        }
        if matches!(row, 0xb | 0x11 | 0x14) {
            add(COMPARE_POS_ST, d * 2);
        }
    }
    let defence = read_u16(COMPARE_POS_DEFENCE) as i16;
    for w in [COMPARE_POS_CB, COMPARE_POS_FB, COMPARE_POS_WB, COMPARE_POS_SW] {
        add(w, defence);
    }
    let attack = read_u16(COMPARE_POS_ATTACK) as i16;
    for w in [COMPARE_POS_DM, COMPARE_POS_MC, COMPARE_POS_AM, COMPARE_POS_WG] {
        add(w, attack);
    }
}

/// The key and the position table under the attributes: the "better" header ("<%s - player>{s} better", "Both the
/// same"), the legend ("Legend:", the masked attribute note) and the five rows of two positions each with their stars.
/// Returns the top of the last table less 10 (where the statistics end).
/// Original: part of the compare players screen builder (0x876520, staff_screens.cpp).
pub(crate) unsafe fn compare_screen_position_table(p1: usize, p2: usize, align: &[u8; 8], legend_align: &u8, better1: usize, same: usize, better2: usize) -> i32 {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    const BUF: usize = 0xde1f64;
    let (mut name1, mut name2) = ([0u8; 0xc8], [0u8; 0xc8]);
    exe::database_format_staff_name(p1, name1.as_mut_ptr() as usize, 0xc8, 1);
    exe::database_format_staff_name(p2, name2.as_mut_ptr() as usize, 0xc8, 1);
    let grey = colour_word(G_GUI_COLOUR_PALE_GREY);
    let bottom = 0x1f9 - exe::font_get_height(2);
    compare_screen_area(0x6e, bottom, 0x30c, 0x1f9, 1, 0, 1, 0, 2, 0);
    let t = compare_screen_area(0x6e, bottom, 0x30c, 0x1f9, 3, align.as_ptr() as usize, 1, 0, 1, 0);
    lang(BUF, 0xa85998, 0x64u32, name1.as_ptr() as usize); // "<%s - player>{s} better"
    compare_screen_cell(t, 0, 0, 0xc, 2, better1, BUF);
    lang(BUF, 0xa85988); // "Both the same"
    compare_screen_cell(t, 1, 0, 0xc, 2, same, BUF);
    lang(BUF, 0xa85998, 0x64u32, name2.as_ptr() as usize);
    compare_screen_cell(t, 2, 0, 0xc, 2, better2, BUF);

    let top = bottom - exe::font_get_height(2) * 2;
    compare_screen_area(0x6e, top, 0x30c, bottom, 1, 0, 1, 0, 2, 0);
    let t = compare_screen_area(0x6e, top, 0x30c, bottom, 1, legend_align as *const u8 as usize, 2, 0, 1, 0);
    lang(BUF, 0xa85980); // "Legend:"
    compare_screen_cell(t, 0, 0, 0xc, 2, grey, BUF);
    lang(BUF, 0xa85934); // "( - ) indicates that this attribute is masked and requires further scouting"
    compare_screen_cell(t, 0, 1, 0xc, 2, colour_word(0xaebdd4), BUF);

    let table_top = top - exe::font_get_height(2) * 5;
    compare_screen_area(0x6e, table_top, 0x30c, top, 1, 0, 1, 0, 2, 0);
    let t = compare_screen_area(0x6e, table_top, 0x30c, top, 4, align[4..].as_ptr() as usize, 5, 0, 1, 0);
    let rows: [(usize, usize, usize, usize, usize, usize); 5] = [
        (0xa85924, COMPARE_MARK_GK, COMPARE_POS_GK, 0xa858ec, COMPARE_MARK_DM, COMPARE_POS_DM), // Goalkeeper, Def. Mid.
        (0xa858e0, COMPARE_MARK_FB, COMPARE_POS_FB, 0xa858d0, COMPARE_MARK_MC, COMPARE_POS_MC), // Full Back, Midfielder
        (0xa858c0, COMPARE_MARK_CB, COMPARE_POS_CB, 0xa858b4, COMPARE_MARK_AM, COMPARE_POS_AM), // Centre Back, Att. Mid.
        (0xa858a8, COMPARE_MARK_SW, COMPARE_POS_SW, 0xa8589c, COMPARE_MARK_WG, COMPARE_POS_WG), // Sweeper, Winger
        (0xa85890, COMPARE_MARK_WB, COMPARE_POS_WB, 0xa85884, COMPARE_MARK_ST, COMPARE_POS_ST), // Wing Back, Striker
    ];
    for (row, &(left, lm, lw, right, rm, rw)) in rows.iter().enumerate() {
        let row = row as i32;
        lang(BUF, left);
        compare_screen_cell(t, 0, row, 0xc, 2, grey, BUF);
        compare_screen_stars(t, 1, row, lm, lw, better1, same, better2);
        lang(BUF, right);
        compare_screen_cell(t, 2, row, 0xc, 2, grey, BUF);
        compare_screen_stars(t, 3, row, rm, rw, better1, same, better2);
    }
    table_top - 10
}

/// The statistics table of the compare screen (from the y the position table left): with two different players, the
/// first one not a manager (`staff::JOB_FOR_CLUB` 2) nor one of the 16 last staff records, the statistics of both
/// (`compare_screen_player_stats`); then, with the view mode (param 4) 0, or 2 for a player outside the scouting
/// search, and a first player who is a player, the 31 attributes again into the three tables (sorted by name with
/// 0x88bc50), the eight extra rows (Preferred Foot 0xff, Avg. Rating 0xfe, Morale 0xfd, On Loan 0xfb, Goals 0xf8 or,
/// with two goalkeepers, Conceded 0xf4, Appearances 0xf7, Assists 0xf6, Man of Match 0xf5) and the table in three
/// column pairs (name, value) of 13 rows (more when they fit): the attribute difference (" %d", in the colour of the
/// better one; " -" masked), the two feet, the ratings, the morale comparison, the statistics "%s - %s", and the
/// loan ("On loan from", linked) or the free transfer over the whole width. Without two players: "Need two players to
/// compare."
/// Original: part of the compare players screen builder (0x876520, staff_screens.cpp).
pub(crate) unsafe fn compare_screen_statistics(y: i32) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let draw: TextDrawWrappedFn = core::mem::transmute(FN_TEXT_DRAW_WRAPPED);
    let row_item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let gui_text_area: unsafe extern "C" fn(i32, i32, i32, i32, u32, u32, u32, u32, usize, usize, u32) -> usize = core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    const BUF: usize = 0xde1f64;

    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0);
    let align: [u8; 6] = [3, 2, 3, 2, 3, 2];
    let mut morale: i8 = 0;
    let viewer = read_usize(G_SCREEN_RECORDS_MANAGER + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300);
    let (better2, same) = compare_screen_colours();
    let better1 = colour_word(0xae31aa);
    for k in [1, 2, 5, 6] {
        get_screen_param(k);
    }
    let c1 = Staff(compare_screen_player(1));
    let c2 = Staff(compare_screen_player(2));
    let first = Staff(if !c1.is_null() { c1.addr() } else { get_screen_param(0) });
    let mode = get_screen_param(4) as i32;
    let n = read_i32(G_STAFF_COUNT);
    let skip = c1 == c2 || c1.is_null() || c2.is_null() || first.job_for_club() == 2 || {
        let id = read_i32(first.addr());
        id >= n.wrapping_sub(16) && (first.is_null() || id < n)
    };
    'table: {
        if skip {
            covered!("tel07_2 compare screen without two players");
            let rows = ((y - 0x73) / exe::font_get_height(3)) as u8;
            lang(BUF, 0xa85718); // "Need two players to compare."
            gui_text_area(0x6e, 0x73, 0x30c, y, 2, 0, rows as u32, 3, colour_word(G_GUI_COLOUR_PALE_GREY), BUF, 0);
            break 'table;
        }
        let mut s1 = [[0u8; 0xc8]; 6];
        let mut s2 = [[0u8; 0xc8]; 6];
        // the four "0" the original copies first (into buffers the statistics then overwrite)
        compare_screen_copy(&mut s1[2], 0x99cc54);
        compare_screen_copy(&mut s2[2], 0x99cc54);
        compare_screen_copy(&mut s1[1], 0x99cc54);
        compare_screen_copy(&mut s2[1], 0x99cc54);
        compare_screen_player_stats(c1.addr(), &mut s1);
        compare_screen_player_stats(c2.addr(), &mut s2);
        if mode == 2 {
            if exe::scout_manager_find_club_search_entry(read_usize(G_SCOUT_MANAGER), first) != 0 || first.player().addr() == 0 {
                break 'table;
            }
        } else if mode != 0 {
            break 'table;
        }
        if first.player().addr() == 0 {
            break 'table;
        }
        covered!("tel07_2 compare screen statistics");
        let fit = ((y - 0x73) / exe::font_get_height(2)) as u8;
        compare_screen_area(0x6e, 0x73, 0x30c, y, 1, 0, 1, 0, 2, 0);
        let area = compare_screen_area(0x6e, 0x73, 0x30c, y, 6, align.as_ptr() as usize, fit as u32, 0, 1, 0);
        let mut col: u32 = 0;
        let mut row: i32 = 0;
        exe::training_manager_update_player_training(read_usize(G_TRAINING_MANAGER), first, 0);
        for attr in 1..=0x1fu32 {
            let k = (attr as usize - 1) * COMPARE_ROW;
            (exe::database_get_player_attribute_name(attr, COMPARE_TABLE_FIRST + k, 0x64, 0) as usize);
            (exe::database_get_player_attribute_name(attr, COMPARE_TABLE_SECOND + k, 0x64, 0) as usize);
            (exe::database_get_player_attribute_name(attr, COMPARE_TABLE_DIFF + k, 0x64, 0) as usize);
            if read_u8(G_ATTRIBUTE_MASKING_ENABLED) != 0 {
                let v2 = exe::staff_get_player_attribute_for_viewer(viewer, c2, attr);
                let v1 = exe::staff_get_player_attribute_for_viewer(viewer, c1, attr);
                write_u8(COMPARE_TABLE_DIFF + 0x64 + k, v1.wrapping_sub(v2));
                write_u8(COMPARE_TABLE_SECOND + 0x64 + k, exe::staff_get_player_attribute_for_viewer(viewer, c2, attr));
                let v1 = exe::staff_get_player_attribute_for_viewer(viewer, c1, attr);
                write_u8(COMPARE_TABLE_FIRST + 0x64 + k, v1);
                write_u8(COMPARE_MASKED + k, ((v1 as i8) < 0) as u8);
                if read_i8(COMPARE_TABLE_SECOND + 0x64 + k) < 0 {
                    write_u8(COMPARE_MASKED + k, 1);
                }
            } else {
                // without masking the marks stay as the position table left them
                let v2 = exe::staff_compute_player_attribute(c2, c2.player().addr(), attr);
                let v1 = exe::staff_compute_player_attribute(c1, c1.player().addr(), attr);
                write_u8(COMPARE_TABLE_DIFF + 0x64 + k, v1.wrapping_sub(v2));
                write_u8(COMPARE_TABLE_SECOND + 0x64 + k, exe::staff_compute_player_attribute(c2, c2.player().addr(), attr));
                write_u8(COMPARE_TABLE_FIRST + 0x64 + k, exe::staff_compute_player_attribute(c1, c1.player().addr(), attr));
            }
        }
        // DEFECT of the original kept: the masked marks are not sorted with the three tables; with the attribute
        // names in alphabetical order (English) the sort keeps the order, in another language the " -" lands on
        // another row
        for table in [COMPARE_TABLE_FIRST, COMPARE_TABLE_SECOND, COMPARE_TABLE_DIFF] {
            exe::crt_qsort(table, 0x1f, 0x65, 0x88bc50);
        }
        let extras: [(usize, usize, u8); 4] = [(31, 0xa85874, 0xff), (32, 0xa85868, 0xfe), (33, 0x98e9f0, 0xfd), (34, 0xa85860, 0xfb)];
        for (r, s, code) in extras {
            lang(COMPARE_TABLE_FIRST + r * COMPARE_ROW, s); // Preferred Foot, Avg. Rating, Morale, On Loan
            write_u8(COMPARE_TABLE_FIRST + r * COMPARE_ROW + 0x64, code);
        }
        if read_u32(COMPARE_BOTH_GOALKEEPERS) == 0 {
            lang(COMPARE_TABLE_FIRST + 35 * COMPARE_ROW, 0x98e9cc); // "Goals"
            write_u8(COMPARE_TABLE_FIRST + 35 * COMPARE_ROW + 0x64, 0xf8);
        } else {
            lang(COMPARE_TABLE_FIRST + 35 * COMPARE_ROW, 0x99cb44); // "Conceded"
            write_u8(COMPARE_TABLE_FIRST + 35 * COMPARE_ROW + 0x64, 0xf4);
        }
        for (r, s, code) in [(36usize, 0x99cc40usize, 0xf7u8), (37, 0x99c06c, 0xf6), (38, 0x99c04c, 0xf5)] {
            lang(COMPARE_TABLE_FIRST + r * COMPARE_ROW, s); // Appearances, Assists, Man of Match
            write_u8(COMPARE_TABLE_FIRST + r * COMPARE_ROW + 0x64, code);
        }
        // the 39 rows dealt to the three columns; the table height is the most rows of a column or what fits
        let mut per_column = [0u8; 3];
        let mut rows = fit;
        let mut c = 0usize;
        for _ in 0..0x27 {
            per_column[c] += 1;
            if per_column[c] > rows {
                rows = per_column[c];
            }
            c = if c + 1 > 2 { 0 } else { c + 1 };
        }
        let grey = colour_word(G_GUI_COLOUR_PALE_GREY);
        let blank = |col: u32, row: i32| {
            compare_screen_cell(area, col, row, 0xc, 2, grey, 0);
            compare_screen_cell(area, col + 1, row, 0xc, 2, grey, 0);
        };
        let (mut feet1, mut feet2) = ([0u8; 0xc8], [0u8; 0xc8]);
        let mut item = 0usize;
        while item < 0x27 {
            let k = item * COMPARE_ROW;
            let code = read_u8(COMPARE_TABLE_FIRST + 0x64 + k);
            if code == 0xfb {
                row_item(current_human_display(), 1, 0x22e, 0, 0x30c, 0, 0, row as usize, 2, 0, 0, 0xc, 2, grey, 0, 0, 0, 0, area);
                let mut drawn = false;
                if exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), first) != 0 {
                    let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), first);
                    write_usize(0xae1080, contract);
                    if contract != 0 {
                        let club = Club(read_usize(G_CLUB_TABLE) + read_i32(contract + 4) as isize as usize * club::SIZE);
                        if !club.is_null() {
                            let gender = (club.short_gender_name() as i8) as i32;
                            lang(BUF, 0xa85834, gender, gender, club.addr() + club::SHORT_NAME); // "On loan from {}<%s - Club(e.g.Falkirk)>{}"
                            exe::text_set_string(t as usize, BUF as *const u8);
                            exe::text_add_link(t as usize, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
                            draw(t, area, 0x238, 0x30c, &mut row, 2, colour_word(0xae3152), 3, 0);
                            drawn = true;
                        }
                    }
                }
                if !drawn && exe::transfer_manager_is_queued_for_free_transfer(read_usize(G_TRANSFER_MANAGER), first) != 0 {
                    lang(BUF, 0xa85820); // "Given free transfer"
                    exe::text_set_string(t as usize, BUF as *const u8);
                    draw(t, area, 0x238, 0x30c, &mut row, 2, colour_word(0xae3152), 3, 0);
                }
            } else {
                format(BUF, 0x987ff0, COMPARE_TABLE_FIRST + k); // "  %s"
                compare_screen_cell(area, col, row, 1, 2, grey, BUF);
                let value = col + 1;
                let code = code as i8;
                if code >= 0 {
                    if read_u8(COMPARE_MASKED + k) == 1 {
                        format(BUF, 0xa8573c); // " -"
                        compare_screen_cell(area, value, row, 0xc, 2, colour_word(0xaebdd4), BUF);
                    } else {
                        let d = read_i8(COMPARE_TABLE_DIFF + 0x64 + k);
                        let colour = if d > 0 { better1 } else if d == 0 { same } else { better2 };
                        format(BUF, 0xa85738, (d as i32).abs()); // " %d"
                        compare_screen_cell(area, value, row, 0xc, 2, colour, BUF);
                    }
                } else {
                    match code as u8 {
                        0xff => {
                            compare_screen_foot(c1.addr(), &mut feet1);
                            compare_screen_foot(c2.addr(), &mut feet2);
                        }
                        0xfe => {
                            if !first.is_null() && read_i32(first.addr()) >= n {
                                format(feet1.as_mut_ptr() as usize, 0x98ba0c); // "-"
                            } else {
                                // computed but not shown: the row prints the average ratings of the statistics
                                (exe::playerstats_format_recent_ratings(read_usize(G_PLAYER_STATS), c1.player().addr(), feet1.as_mut_ptr() as usize, 0xc8) as usize);
                                (exe::playerstats_format_recent_ratings(read_usize(G_PLAYER_STATS), c2.player().addr(), feet2.as_mut_ptr() as usize, 0xc8) as usize);
                            }
                        }
                        0xfd => {
                            morale = exe::contract_manager_write_morale_comparison_text(read_usize(G_CONTRACT_MANAGER), c1, c2, feet1.as_mut_ptr() as usize, 0xc8);
                        }
                        _ => {}
                    }
                    let hint = colour_word(0xaebdd4);
                    match code as u8 {
                        0xfe => {
                            format(BUF, 0xa8574c, s1[0].as_ptr() as usize, s2[0].as_ptr() as usize); // " %s - %s "
                            compare_screen_cell(area, value, row, 0xc, 1, hint, BUF);
                        }
                        0xf9 => {
                            format(BUF, 0xa8573c); // " -"
                            compare_screen_cell(area, value, row, 0xc, 2, hint, BUF);
                        }
                        0xf8 | 0xf4 | 0xf7 | 0xf6 | 0xf5 => {
                            let i = match code as u8 { 0xf8 => 1, 0xf4 => 2, 0xf7 => 3, 0xf6 => 4, _ => 5 };
                            format(BUF, 0x988080, s1[i].as_ptr() as usize, s2[i].as_ptr() as usize); // "%s - %s"
                            compare_screen_cell(area, value, row, 0xc, 2, hint, BUF);
                        }
                        0xfd => {
                            format(BUF, 0x987ff0, feet1.as_ptr() as usize); // "  %s"
                            let colour = if morale < -3 { better2 } else if morale < 3 { same } else { better1 };
                            compare_screen_cell(area, value, row, 0xc, 2, colour, BUF);
                        }
                        _ => {
                            format(BUF, 0xa85740, feet1.as_ptr() as usize, feet2.as_ptr() as usize); // "  %s ~ %s"
                            compare_screen_cell(area, value, row, 0xc, 2, hint, BUF);
                        }
                    }
                }
            }
            row += 1;
            if row == per_column[(col / 2) as usize] as i32 {
                while row < rows as i32 {
                    blank(col, row);
                    row += 1;
                }
                col += 2;
                row = 0;
                if col + 1 > 5 {
                    break;
                }
            }
            item += 1;
        }
        while col < 6 {
            while row < rows as i32 {
                blank(col, row);
                row += 1;
            }
            col += 2;
            row = 0;
        }
    }
    exe::drop_text(t);
}

/// One position of the compare screen's table: first player rating `a`, second `b` (the ratings 1..20 of the player
/// sheet; 15 = can play there). Both can (≥ 15 for the first, ≤ 15 for the second: the first is better, 35 (0x23) in
/// `word`; the other way round (the first ≤ 15 and the second ≥ 15): −35; both above 15: twice the difference, and 1
/// in `extra` (the goalkeepers' 0xde1d04; the others pass their own mark, already 1); both below 15: the mark `mark`
/// goes to 0 ("n/a").
/// Original: part of the compare players screen builder (0x876520, staff_screens.cpp).
pub(crate) unsafe fn compare_screen_position(a: i8, b: i8, word: usize, mark: usize, extra: usize) {
    const CAN_PLAY: i8 = 15; // tel07_2 the "can play there" rating
    if a >= CAN_PLAY && b <= CAN_PLAY {
        write_u16(word, 0x23);
    } else if a <= CAN_PLAY && b >= CAN_PLAY {
        write_u16(word, 0xffdd);
    } else if a > CAN_PLAY && b > CAN_PLAY {
        write_u16(word, ((a as i16 - b as i16) * 2) as u16);
        write_u8(extra, 1);
    } else {
        write_u8(mark, 0);
    }
}

/// The statistics of one player of the compare screen: for the six statistics periods 1..6, each code of his list
/// (the goalkeeper list 0xa84a90 when the player sheet's goalkeeper rating is 20, otherwise 0xa84a78; 0xff ends it)
/// formatted by `PlayerStats::format_player_stat` (`G_PLAYER_STATS`, 2000 bytes into the text buffer, 1, 0, 1), and
/// kept by code: 0x11 the average rating, 2 the goals, 3 the goals conceded, 1 the appearances, 5 the assists, 0xc the
/// man of the match awards (only the last period stays). A "-" becomes "0".
/// Original: part of the compare players screen builder (0x876520, staff_screens.cpp).
pub(crate) unsafe fn compare_screen_player_stats(p: usize, out: &mut [[u8; 0xc8]; 6]) {
    let p = Staff(p);
    const BUF: usize = 0xde1f64;
    const CODES: [u8; 6] = [0x11, 2, 3, 1, 5, 0xc];
    for period in 1..=6u32 {
        let player = p.player();
        let list = if !player.is_null() && (player.goalkeeper() as u8) == 0x14 { 0xa84a90usize } else { 0xa84a78 };
        let mut k = list;
        while read_u8(k) != 0xff {
            let code = read_u8(k);
            (exe::playerstats_format_player_stat(read_usize(G_PLAYER_STATS), p.player().addr(), period, code as u32, BUF, 0x7d0, 1, 0, 1) as usize);
            if let Some(i) = CODES.iter().position(|&c| c == code) {
                compare_screen_copy(&mut out[i], BUF);
            }
            k += 1;
        }
    }
    for s in out.iter_mut() {
        if compare_screen_streq(s.as_ptr() as usize, 0x98ba0c) {
            // "-"
            compare_screen_copy(s, 0x99cc54); // "0"
        }
    }
}
