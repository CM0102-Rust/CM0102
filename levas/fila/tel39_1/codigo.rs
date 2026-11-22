// ---------- batch %%LEVA%%: the staff profile's last pieces: the squad and history menu, the text rows order, the nickname dialog and the scout request dialog (staff_screens.cpp) ----------

/// "%s %s %d.%d.%d" (first name, second name, day, month, year: the written history's key of a person), the empty
/// string, "Player Nickname" and the two scout request questions ("Have <%s> assessed by your scouts ?", "Have <%s>
/// watched when your scouts become available ?"), "Remove", "Cancel", "Ok", "No" and "Yes".
const STAFF_HISTORY_KEY: usize = 0x9ced08;
const STAFF_NO_NAME: usize = 0x989ad8;
const STAFF_NICKNAME_TITLE: usize = 0xa8b35c;
const STAFF_SCOUT_ASSESS: usize = 0xa8b3c4;
const STAFF_SCOUT_WATCH: usize = 0xa8b36c;
const STAFF_REMOVE: usize = 0xa8b354;
const STAFF_CANCEL: usize = 0x98a1a0;
const STAFF_OK: usize = 0x990f7c;
const STAFF_NO: usize = 0x98e8cc;
const STAFF_YES: usize = 0x98e8d0;
/// The nickname dialog's ids: the text entry, Ok, Cancel and Remove; the scout request's No and Yes.
const NICKNAME_ENTRY: u16 = 0x32;
const NICKNAME_OK: u16 = 0x33;
const NICKNAME_CANCEL: u16 = 0x34;
const NICKNAME_REMOVE: u16 = 0x35;
const SCOUT_REQUEST_NO: u16 = 0x36;
const SCOUT_REQUEST_YES: u16 = 0x37;
/// `HUMAN_MANAGER` methods of the nicknames (thiscall on [G_HUMAN_MANAGER]): remove (person, 0) ret 8, change
/// (person, nickname, 0) ret 0xc, add (person, nickname, 0) ret 0xc [inference: the names, from the dialog].
const FN_HUMAN_MANAGER_REMOVE_NICKNAME: usize = 0x6003d0;
const FN_HUMAN_MANAGER_CHANGE_NICKNAME: usize = 0x600870;
const FN_HUMAN_MANAGER_ADD_NICKNAME: usize = 0x600090;
/// `SCOUT_MANAGER::add_report_request` (0x7f9290, thiscall on [G_SCOUT_MANAGER], (person, 0), ret 8) [inference].
const FN_SCOUT_MANAGER_ADD_REPORT_REQUEST: usize = 0x7f9290;

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
    let mut list = [0u8; 0x10];
    let list = list.as_mut_ptr() as usize;
    exe::new_llist(list);
    let p = Staff(person);
    let second = if read_usize(person + staff::SECOND_NAME) != 0 { read_usize(person + staff::SECOND_NAME) } else { STAFF_NO_NAME };
    let first = if read_usize(person + staff::FIRST_NAME) != 0 { read_usize(person + staff::FIRST_NAME) } else { STAFF_NO_NAME };
    let year = read_u16(person + staff::DATE_OF_BIRTH_YEAR) as u32;
    let month = exe::date_get_month(person + staff::DATE_OF_BIRTH) as i8 as i32 + 1;
    let day = exe::date_get_day_of_month(person + staff::DATE_OF_BIRTH) as i16 as i32;
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    sprintf(JOB_TEXT, STAFF_HISTORY_KEY, first, second, day, month, year);
    let history = exe::history_his_file_exists(JOB_TEXT) != 0;
    let mut area = u32::MAX;
    if read_usize(person + staff::PLAYER) != 0 {
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

/// `HUMAN_MANAGER` nickname call (person, nickname, 0) on [G_HUMAN_MANAGER].
unsafe fn nickname_call(f: usize, person: usize, nickname: usize) {
    let g: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(f);
    g(read_usize(G_HUMAN_MANAGER), person, nickname, 0);
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
    let person = get_screen_param(0);
    let nickname = get_screen_param(1);
    match tel26_item_id(raw as i16) {
        NICKNAME_ENTRY => -11,
        NICKNAME_REMOVE => {
            covered!("tel39_1 nickname removed");
            let g: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_HUMAN_MANAGER_REMOVE_NICKNAME);
            g(read_usize(G_HUMAN_MANAGER), person, 0);
            exe::crt_free(nickname);
            -9
        }
        NICKNAME_CANCEL => {
            exe::crt_free(nickname);
            -9
        }
        NICKNAME_OK => {
            let common = read_usize(person + staff::COMMON_NAME);
            let same = common != 0 && read_u8(common) != 0
                && core::ffi::CStr::from_ptr(common as *const core::ffi::c_char) == core::ffi::CStr::from_ptr(nickname as *const core::ffi::c_char);
            if !same {
                covered!("tel39_1 nickname stored");
                if exe::human_manager_has_nickname(read_usize(G_HUMAN_MANAGER), person, 0) != 0 {
                    nickname_call(FN_HUMAN_MANAGER_CHANGE_NICKNAME, person, nickname);
                } else {
                    nickname_call(FN_HUMAN_MANAGER_ADD_NICKNAME, person, nickname);
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
