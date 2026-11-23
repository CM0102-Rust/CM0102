// ---------- batch 322: the staff profile's last pieces: the squad and history menu, the text rows order, the nickname dialog and the scout request dialog (staff_screens.cpp) ----------

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

// ---------- batch 322: the small routines of the screens: the background picture settings, the Text copy, a list's deleting destructor, the contract news buttons, the waiting screen, a static date's destructor and the search filter record ----------

/// u32: the fixed background flag of the game settings (Display::draw_screen uses the fixed picture when it is set).
const BACKGROUND_FIXED_FLAG: usize = 0xdd6bda;

/// Frees the pics.cfg tables: for each of the 11 sections with subsections, each subsection's two picture lists
/// (every non-null name, then the list; the list pointer and its count cleared) and its name, then the subsection
/// array (pointer and count cleared). The section names stay.
///
/// `background_free_config_tables()`: section k (`background_section`, 10 bytes): with a non-null subsection array,
/// subsection j < the count (i16): `background_sub::LIST` (when non-null: its `background_sub::COUNT` names, each
/// freed when non-null, then the list freed, the pointer and the count zeroed), the same with
/// `background_sub::SECOND_LIST` and `background_sub::SECOND_COUNT`, then the name freed when non-null (the pointer
/// is not cleared); then the array is freed and the pointer and the count zeroed. `free` of the CRT throughout.
/// Original: `background_free_config_tables` (0x41bad0, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn background_free_config_tables() {
    for k in 0..BACKGROUND_SECTION_COUNT as i32 {
        let section = background_section(k);
        if read_usize(section + 6) == 0 {
            continue;
        }
        covered!("tel39_2 free the pictures tables");
        let mut j = 0i16;
        while j < read_i16(section + 4) {
            let sub = read_usize(section + 6) + j as usize * background_sub::SIZE;
            for (list, count) in [(background_sub::LIST, background_sub::COUNT), (background_sub::SECOND_LIST, background_sub::SECOND_COUNT)] {
                if read_usize(sub + list) == 0 {
                    continue;
                }
                let mut i = 0i16;
                while i < read_i16(sub + count) {
                    let name = read_usize(read_usize(sub + list) + i as usize * 4);
                    if name != 0 {
                        exe::crt_free(name);
                    }
                    i += 1;
                }
                exe::crt_free(read_usize(sub + list));
                write_usize(sub + list, 0);
                // tel39_2: the list's count cleared
                write_i16(sub + count, 0);
            }
            if read_usize(sub + background_sub::NAME) != 0 {
                exe::crt_free(read_usize(sub + background_sub::NAME));
            }
            j += 1;
        }
        exe::crt_free(read_usize(section + 6));
        write_usize(section + 6, 0);
        write_i16(section + 4, 0);
    }
}

/// Sets the fixed background flag of the game settings to the inverse of `on`.
///
/// `background_set_pictures_disabled(on)`: [0xdd6bda] = 1 when `on` is 0, else 0.
/// Original: `background_set_pictures_disabled` (0x41bf80, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn background_set_pictures_disabled(on: u32) {
    covered!("tel39_2 picture setting");
    // tel39_2: the inverse of the argument
    write_u32(BACKGROUND_FIXED_FLAG, (on == 0) as u32);
}

/// The inverse of the fixed background flag: 1 when [0xdd6bda] is 0 (what `background_set_pictures_disabled` got).
/// Original: `background_pictures_disabled` (0x41bfa0, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn background_pictures_disabled() -> u32 {
    (read_u32(BACKGROUND_FIXED_FLAG) == 0) as u32
}

/// The scalar deleting destructor of a linked list: `LLIST::~LLIST`, then operator delete when bit 0 of the flags is
/// set; returns the list.
/// Original: `LLIST::scalar_deleting_destructor` (0x497910, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn llist_scalar_deleting_destructor(this: usize, flags: u32) -> usize {
    covered!("tel39_2 list deleting destructor");
    exe::drop_llist(this);
    if flags & 1 != 0 {
        exe::crt_operator_delete(this);
    }
    this
}

/// Constructs a Text as a copy of another: its string and each of its links.
///
/// `Text::Text(this, other)` (the other by pointer, may be null): string and link list null; with another Text, its
/// string (when there is one) through `Text::set_string`, then, with a link list, the list read from its head (the
/// other's cursor moves) and each link added through `Text::add_link(phrase, kind, data)`. Returns `this`.
/// Original: `Text::Text` (0x4ae5c0, comp_text.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn text_construct_copy(this: usize, other: usize) -> usize {
    write_usize(this + TEXT_STRING, 0);
    write_usize(this + TEXT_LINKS, 0);
    if other == 0 {
        return this;
    }
    covered!("tel39_2 copy a text");
    let s = read_usize(other + TEXT_STRING);
    if s != 0 {
        text_set_string(this, s);
    }
    if read_usize(other + TEXT_LINKS) == 0 {
        return this;
    }
    list_rewind(read_usize(other + TEXT_LINKS));
    loop {
        let e = exe::llist_get_next(read_usize(other + TEXT_LINKS));
        if e == 0 {
            break;
        }
        // tel39_2: each link with its kind and data
        text_add_link(this, e, read_u32(e + TEXT_LINK_KIND), read_u32(e + TEXT_LINK_DATA));
    }
    this
}


/// Comparer for `text_construct_copy`: the copy builds a local Text, the original the real one; string and links
/// equal.
pub(crate) unsafe extern "thiscall" fn compare_text_construct_copy(this: usize, other: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_TEXT_CONSTRUCT_COPY);
    let mut local = [0usize; 2];
    text_construct_copy(local.as_mut_ptr() as usize, other);
    let r = o(this, other);
    let (mo, mr) = (text_contents(this), text_contents(local.as_ptr() as usize));
    comp_text_free(&mut local);
    SCOREBOARD.record(r == this && mo == mr, || format!("Text::Text (copy): original {:?} {:x?}, rust {:?} {:x?}", String::from_utf8_lossy(&mo.0), mo.1,
        String::from_utf8_lossy(&mr.0), mr.1));
    r
}

/// The staff record of a staff id (`staff::SIZE` × id into the staff table, in 32 bits like the original).
unsafe fn tel39_staff(id: u32) -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add(id.wrapping_mul(staff::SIZE as u32) as usize)
}

/// The manager's news item `id` into a local NewsItem (`NewsItem::NewsItem(0, 0)`, `News::find_item_by_id`).
unsafe fn tel39_news_item(manager: usize, id: u32, item: usize) {
    exe::new_newsitem(item, 0, 0);
    exe::news_find_item_by_id(read_usize(G_NEWS), manager, id, item);
}

/// The news type of a contract news item that comes with a transfer offer.
const NEWS_TYPE_OFFER_ACCEPTED: u32 = 0xfa4;

/// Rule: the "Offer Contract" button of a news item opens the contract offer for the player it names.
///
/// `contract_screens_news_offer_contract_action(manager, id)`: the news item (`tel39_news_item`); its slot 2 is the
/// person's id; `contract_screens_offer_contract_to_staff(person, 0, 0, 1)`.
/// Original: `contract_screens_news_offer_contract_action` (0x4dc9b0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_news_offer_contract_action(manager: usize, id: u32) {
    covered!("tel39_2 news offer contract");
    let mut item = [0usize; 0x38];
    let item = item.as_mut_ptr() as usize;
    tel39_news_item(manager, id, item);
    // tel39_2: the player of slot 2
    contract_screens_offer_contract_to_staff(tel39_staff(exe::newsitem_get_slot(item, 2)), 0, 0, 1);
}

/// Rule: the "Negotiate" button of a contract news item opens the negotiation with the person it names; after an
/// accepted transfer offer (news type 0xfa4) it negotiates on that offer.
///
/// `contract_screens_news_negotiate_action(manager, id)`: the news item; its slot 0 is the person's id. With the
/// item's type (+0) 0xfa4: `contract_screens_offer_contract_to_staff(person, TRANSFER_MANAGER::get_offer_slot(slot
/// 7), 1, 0)`; otherwise `(person, 0, 0, 0)`.
/// Original: `contract_screens_news_negotiate_action` (0x4e5770, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_news_negotiate_action(manager: usize, id: u32) {
    covered!("tel39_2 news negotiate");
    let mut item = [0usize; 0x38];
    let item = item.as_mut_ptr() as usize;
    tel39_news_item(manager, id, item);
    let person = tel39_staff(exe::newsitem_get_slot(item, 0));
    if read_u32(item) == NEWS_TYPE_OFFER_ACCEPTED {
        let offer = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), exe::newsitem_get_slot(item, 7) as i32);
        contract_screens_offer_contract_to_staff(person, offer, 1, 0);
    } else {
        contract_screens_offer_contract_to_staff(person, 0, 0, 0);
    }
}

/// The waiting for players screen (a network game): only the background picture is cleared
/// (`background_set_picture(null)`).
/// Original: `match_screens_draw_waiting_screen` (0x727fa0, match_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn match_screens_draw_waiting_screen() {
    covered!("tel39_2 waiting screen");
    background_set_picture(0);
}

/// The empty atexit destructor of the static Date that 0x729ff0 builds on its first call: nothing.
/// Original: `match_stats_static_date_atexit` (0x72aa70, match_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn match_stats_static_date_atexit() {}

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

/// Comparer for `background_free_config_tables`: only the original runs (freeing does not run twice); then every
/// section must have no subsections left (array and count zero).
pub(crate) unsafe extern "C" fn compare_background_free_config_tables() {
    original_fn::<unsafe extern "C" fn()>(IDX_BACKGROUND_FREE_CONFIG_TABLES)();
    let left: Vec<i32> = (0..BACKGROUND_SECTION_COUNT as i32)
        .filter(|&k| read_usize(background_section(k) + 6) != 0 || read_i16(background_section(k) + 4) != 0)
        .collect();
    SCOREBOARD.record(left.is_empty(), || format!("background_free_config_tables: sections left {:?}", left));
}

/// Comparer for `background_set_pictures_disabled`: side by side over the flag.
pub(crate) unsafe extern "C" fn compare_background_set_pictures_disabled(on: u32) {
    let o = original_fn::<unsafe extern "C" fn(u32)>(IDX_BACKGROUND_SET_PICTURES_DISABLED);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(BACKGROUND_FIXED_FLAG, 4)], || o(on), || background_set_pictures_disabled(on));
    SCOREBOARD.record(same, || format!("background_set_pictures_disabled({}) original {:x?} rust {:x?}", on, mo, mr));
}

/// Comparer for `background_pictures_disabled`: the answer side by side.
pub(crate) unsafe extern "C" fn compare_background_pictures_disabled() -> u32 {
    let r = original_fn::<unsafe extern "C" fn() -> u32>(IDX_BACKGROUND_PICTURES_DISABLED)();
    let n = background_pictures_disabled();
    SCOREBOARD.record(r == n, || format!("background_pictures_disabled() original={} rust={}", r, n));
    r
}

/// Comparer for `llist_scalar_deleting_destructor`: only the original runs (freeing does not run twice).
pub(crate) unsafe extern "thiscall" fn compare_llist_scalar_deleting_destructor(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(IDX_LLIST_SCALAR_DELETING_DESTRUCTOR)(this, flags)
}

/// Comparer for `contract_screens_news_offer_contract_action`: only the original runs (it opens a screen; the
/// contract offer's own comparer checks it).
pub(crate) unsafe extern "C" fn compare_contract_screens_news_offer_contract_action(manager: usize, id: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_CONTRACT_SCREENS_NEWS_OFFER_CONTRACT_ACTION)(manager, id)
}

/// Comparer for `contract_screens_news_negotiate_action`: only the original runs (it opens a screen; the contract
/// offer's own comparer checks it).
pub(crate) unsafe extern "C" fn compare_contract_screens_news_negotiate_action(manager: usize, id: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_CONTRACT_SCREENS_NEWS_NEGOTIATE_ACTION)(manager, id)
}

/// Comparer for `match_screens_draw_waiting_screen`: side by side over the screen's parameter block (where the
/// picture name goes).
pub(crate) unsafe extern "C" fn compare_match_screens_draw_waiting_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_MATCH_SCREENS_DRAW_WAITING_SCREEN);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&screen_param_regions(), || o(), || match_screens_draw_waiting_screen());
    SCOREBOARD.record(same, || format!("waiting for players screen: differences {:?}", first_differences(&mo, &mr)));
}

/// Comparer for `match_stats_static_date_atexit`: the original (nothing to compare).
pub(crate) unsafe extern "C" fn compare_match_stats_static_date_atexit() {
    original_fn::<unsafe extern "C" fn()>(IDX_MATCH_STATS_STATIC_DATE_ATEXIT)()
}

/// Comparer for `search_filter_record_init`: side by side over the 0x11 bytes and the return.
pub(crate) unsafe extern "thiscall" fn compare_search_filter_record_init(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_SEARCH_FILTER_RECORD_INIT);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, 0x11)], || o(this), || search_filter_record_init(this));
    SCOREBOARD.record(same && r == n, || format!("search_filter_record_init({:#x}) original {:x?} rust {:x?}", this, mo, mr));
    r
}

// ---------- batch 322: the screen manager's players and network packets: the player records, the network session, the click and scroll packets and the status message (scrman.cpp) ----------

/// The screen manager's join and leave hooks (cdecl(player), given to `start_network_session` and to
/// `add_manager_record`; the join hook is the global 0xdd6bf0 the player record callback calls), the channel a join
/// hook asks `monitor` to switch to (−1 none) and the dword of a record `monitor` stamps with the time [inference].
const SCRMAN_JOIN_HOOK: usize = 0x2627a0;
const SCRMAN_LEAVE_HOOK: usize = 0x2627a4;
const SCRMAN_NEXT_CHANNEL: usize = 0x382c;
const SCRMAN_RECORD_STAMP: usize = 0x2c;
/// The two player record callbacks by their exe address (through their detours, as the original passes them).
const SCRMAN_ADD_PLAYER_RECORD: usize = 0x7ff630;
const SCRMAN_REMOVE_PLAYER_RECORDS: usize = 0x7ff7b0;

/// The address of record `k` of channel `c` of the screen manager `sm` (16 channels of 16 records of 0x30 bytes from
/// +0x7d4; `scrman_channel_record` gives the base to add a `screen_history` offset to).
fn channel_record_at(sm: usize, c: usize, k: usize) -> usize {
    scrman_channel_record(sm, c as i16, k as i16) + screen_history::NETWORK_PLAYER
}

/// The address of the record count of channel `c` (a signed word).
fn channel_count_at(sm: usize, c: usize) -> usize {
    sm + screen_history::CHANNEL_RECORD_COUNTS + 2 * c
}

/// Rule: the machine's own network addresses, for the network game screen.
///
/// Forwards to `Network::get_local_addresses(out)` (0x77a980) on the session (`screen_history::NETWORK`, not checked
/// for null) and returns what it returns (the count).
/// Original: `ScreenManager::get_local_addresses` (0x7fc7c0, scrman.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screen_manager_get_local_addresses(sm: usize, out: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = tcpip_fn(0x77a980);
    f(read_usize(sm + screen_history::NETWORK), out)
}

/// Rule: starting a network session keeps the join and leave hooks, closes the old session and opens a new one whose
/// connections call the player record callbacks; returns whether it opened.
///
/// The join and leave hooks are kept (`SCRMAN_JOIN_HOOK`, `SCRMAN_LEAVE_HOOK`). With a session
/// (`screen_history::NETWORK`): `ScreenManager::disconnect_network` (0x7fc690), then the session destroyed
/// (`Network::~Network` 0x77a890 and `operator delete`) and cleared. A new 0xc3e54-byte object (`operator new`) is built
/// by `Network::Network(ok, scrman_add_player_record, scrman_remove_player_records)` (0x77a7d0) and kept as the session;
/// when it does not open (`ok` 0) it is destroyed and the session cleared. Returns `ok`. Defect of the original: `ok`
/// lives in the join hook's argument slot, so when `operator new` fails it returns the join hook (nonzero, "opened")
/// with no session.
/// Original: `ScreenManager::start_network_session` (0x7ff500, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_start_network_session(sm: usize, on_join: usize, on_leave: usize) -> u32 {
    use screen_history as h;
    covered!("tel40_1 start a network session");
    write_usize(sm + SCRMAN_JOIN_HOOK, on_join);
    write_usize(sm + SCRMAN_LEAVE_HOOK, on_leave);
    let destroy: unsafe extern "thiscall" fn(usize) = tcpip_fn(FN_NETWORK_DESTRUCTOR_NETWORK);
    if read_usize(sm + h::NETWORK) != 0 {
        exe::screenmanager_disconnect_network(sm);
        let old = read_usize(sm + h::NETWORK);
        if old != 0 {
            destroy(old);
            exe::crt_operator_delete(old);
        }
        write_usize(sm + h::NETWORK, 0);
    }
    let mut ok = on_join as u32; // the original's argument slot (see the defect above)
    let p = exe::crt_operator_new(0xc3e54);
    let net = if p != 0 {
        let build: unsafe extern "thiscall" fn(usize, *mut u32, usize, usize) -> usize = tcpip_fn(0x77a7d0);
        build(p, &mut ok, SCRMAN_ADD_PLAYER_RECORD, SCRMAN_REMOVE_PLAYER_RECORDS)
    } else {
        0
    };
    write_usize(sm + h::NETWORK, net);
    if ok == 0 {
        if net != 0 {
            destroy(net);
            exe::crt_operator_delete(net);
        }
        write_usize(sm + h::NETWORK, 0);
    }
    ok
}

/// Rule: a player who joins gets a fresh record in his channel (the channel whose first record is his, or the first
/// empty channel); on a network client nothing happens.
///
/// On a network client (`G_SCREEN_MANAGER_NETWORK` set, `G_SCREEN_MANAGER_NETWORK_HOST_FLAG` 0) it returns. The channels
/// are walked from 0: the first with no records, or whose record 0 is the player (`screen_history::NETWORK_PLAYER`, the
/// low word of `player`), is taken; none, it returns. A full channel (16 records) returns; otherwise its records move one
/// place up (`utils_swap_bytes` 0x90ba90 of records k and k − 1, from the count down to 1, so the free slot comes to
/// the front with its old bytes), the count goes up and record 0 is set: the player, the key, the three history
/// pointers, the flagged and node counts 0, the active flag 1, the Display (the local one, `LOCAL_DISPLAY`, for player 0,
/// the remote one, `SCRMAN_ECHO_DISPLAY`, otherwise), the away flag 0, the screen key −1 and its dword 0, the stamp −1
/// (the done flag keeps the slot's old value). The current channel is set to it: with a join hook (`SCRMAN_JOIN_HOOK`)
/// the hook runs with `player` and `SCRMAN_NEXT_CHANNEL` = the channel; without one the new record gets the active flag 0
/// and the away flag 1 and is swapped with record 1 (it goes behind the current one). The current channel is restored.
/// Original: `scrman_add_player_record` (0x7ff630, scrman.cpp, cdecl).
pub(crate) unsafe extern "C" fn scrman_add_player_record(player: u32) {
    use screen_history as h;
    let sm = G_SCREEN_MANAGER;
    if read_usize(G_SCREEN_MANAGER_NETWORK) != 0 && read_u32(G_SCREEN_MANAGER_NETWORK_HOST_FLAG) == 0 {
        covered!("tel40_1 a client adds no record");
        return;
    }
    let id = player as u16;
    let mut c = 0usize;
    loop {
        if read_i16(channel_count_at(sm, c)) == 0 {
            break; // tel40_1: an empty channel
        }
        if read_u16(channel_record_at(sm, c, 0)) == id {
            let n = read_i16(channel_count_at(sm, c));
            if n >= 0x10 {
                return;
            }
            let mut k = n;
            while k > 0 {
                exe::utils_swap_bytes(channel_record_at(sm, c, k as usize) as *mut i32, channel_record_at(sm, c, k as usize - 1) as *mut i32, 0x30);
                k -= 1;
            }
            break;
        }
        c += 1;
        if c == 0x10 {
            return;
        }
    }
    let count = channel_count_at(sm, c);
    write_i16(count, read_i16(count).wrapping_add(1));
    let r = channel_record_at(sm, c, 0);
    write_u16(r, id);
    for f in [h::KEY, h::FIRST_NODE, h::LAST_NODE, h::CURRENT_NODE] {
        write_u32(r + f - h::NETWORK_PLAYER, 0);
    }
    write_u16(r + h::FLAGGED_COUNT - h::NETWORK_PLAYER, 0);
    write_u16(r + h::NODE_COUNT - h::NETWORK_PLAYER, 0);
    write_u32(r + h::ACTIVE_FLAG - h::NETWORK_PLAYER, 1);
    let display = if id == 0 { sm + h::LOCAL_DISPLAY } else { sm + SCRMAN_ECHO_DISPLAY };
    write_usize(r + screen_manager::DISPLAY - h::NETWORK_PLAYER, display);
    write_u32(r + h::AWAY - h::NETWORK_PLAYER, 0);
    write_u16(r + h::SCREEN_KEY - h::NETWORK_PLAYER, 0xffff);
    write_u32(r + h::SCREEN_KEY_DATA - h::NETWORK_PLAYER, 0);
    write_u32(r + SCRMAN_RECORD_STAMP, 0xffff_ffff);
    let saved = read_u16(sm + screen_manager::CURRENT_SLOT);
    write_u16(sm + screen_manager::CURRENT_SLOT, c as u16);
    let hook = read_usize(sm + SCRMAN_JOIN_HOOK);
    if hook != 0 {
        covered!("tel40_1 join hook");
        let f: unsafe extern "C" fn(u32) = core::mem::transmute(hook);
        f(player);
        write_u16(sm + SCRMAN_NEXT_CHANNEL, c as u16);
        write_u16(sm + screen_manager::CURRENT_SLOT, saved);
        return;
    }
    write_u32(r + h::ACTIVE_FLAG - h::NETWORK_PLAYER, 0);
    write_u32(r + h::AWAY - h::NETWORK_PLAYER, 1);
    exe::utils_swap_bytes(r as *mut i32, channel_record_at(sm, c, 1) as *mut i32, 0x30); // tel40_1: behind the current record
    write_u16(sm + screen_manager::CURRENT_SLOT, saved);
}

/// Rule: a player who leaves loses every record of the channels that are his; only on the host.
///
/// With `G_SCREEN_MANAGER_NETWORK_HOST_FLAG` set, each of the 16 channels whose record 0 is the player (the low word of
/// `player`) has its records removed (`ScreenManager::remove_manager_record` 0x7ff990 with record 0's key and the
/// channel) while its count is above 0, then the count 0.
/// Original: `scrman_remove_player_records` (0x7ff7b0, scrman.cpp, cdecl).
pub(crate) unsafe extern "C" fn scrman_remove_player_records(player: u32) {
    let sm = G_SCREEN_MANAGER;
    if read_u32(G_SCREEN_MANAGER_NETWORK_HOST_FLAG) == 0 {
        return;
    }
    covered!("tel40_1 a player leaves");
    for c in 0..0x10usize {
        if read_u16(channel_record_at(sm, c, 0)) != player as u16 {
            continue;
        }
        while read_i16(channel_count_at(sm, c)) > 0 {
            exe::screenmanager_remove_manager_record(sm, read_usize(channel_record_at(sm, c, 0) + 2), c as i32);
        }
        write_i16(channel_count_at(sm, c), 0);
    }
}

/// Rule: a manager gets a new record in the current channel, for the current channel's player, keeping the session's
/// hooks; with a manager key the record goes behind the current one and gets that key.
///
/// With a key: the join hook 0 and the leave hook `on_leave`, `scrman_add_player_record` with the low word of record 0's
/// player (with no join hook it puts the new record at place 1), record 1's key = `key`, then the join hook `on_join`.
/// Without a key: the hooks `on_leave` and `on_join`, then `scrman_add_player_record` (which runs the join hook).
/// Original: `ScreenManager::add_manager_record` (0x7ff8f0, scrman.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn screen_manager_add_manager_record(sm: usize, on_join: usize, on_leave: usize, key: u32) {
    use screen_history as h;
    let player = read_u16(screen_record(sm) + h::NETWORK_PLAYER) as u32;
    if key != 0 {
        covered!("tel40_1 a manager record with a key");
        write_usize(sm + SCRMAN_JOIN_HOOK, 0);
        write_usize(sm + SCRMAN_LEAVE_HOOK, on_leave);
        scrman_add_player_record(player);
        write_u32(screen_record(sm) + h::RECORD_SIZE + h::KEY, key); // tel40_1: the key on record 1
        write_usize(sm + SCRMAN_JOIN_HOOK, on_join);
        return;
    }
    write_usize(sm + SCRMAN_LEAVE_HOOK, on_leave);
    write_usize(sm + SCRMAN_JOIN_HOOK, on_join);
    scrman_add_player_record(player);
}

/// Delivers the screen manager's packet like the click senders: a local current record (player 0) has it applied to
/// the local Display (`Display::process_command_packet` 0x564080 with the session and 0), otherwise it is sent to its
/// player (`scrman_send_packet`, failing: the box `scrman.cpp <line>`). No echo Display here.
unsafe fn scrman_apply_or_send(sm: usize, line: u32) {
    use screen_history as h;
    if read_u16(screen_record(sm) + h::NETWORK_PLAYER) == 0 {
        exe::display_process_command_packet(sm + h::LOCAL_DISPLAY, read_usize(sm + h::NETWORK), 0, sm + h::NET_BUFFER);
        return;
    }
    covered!("tel40_1 click packet to a remote manager");
    scrman_send_packet(sm, line);
}

/// Rule: the current screen's queued click is cleared (command 0x11), on the local Display or the remote player's.
///
/// `scrman_command_packet` with 0x11 (box line `network.h 133`), delivered by `scrman_apply_or_send` (send failure
/// `scrman.cpp 2480`).
/// Original: `ScreenManager::send_clear_click` (0x800000, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_send_clear_click(sm: usize) {
    scrman_command_packet(sm, 0x11); // tel40_1: the clear click command
    scrman_apply_or_send(sm, 0x9b0);
}

/// Rule: the next left click on the current screen returns the given widget (command 0x13 with the widget).
///
/// `scrman_command_packet` with 0x13, the widget as a word (box line `network.h 134`), delivered by
/// `scrman_apply_or_send` (send failure `scrman.cpp 2527`).
/// Original: `ScreenManager::set_pending_click_widget` (0x800160, scrman.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_pending_click_widget(sm: usize, widget: u32) {
    scrman_command_packet(sm, 0x13);
    scrman_net_put(sm, widget, 2, 0x86); // tel40_1: the widget of the pending click
    scrman_apply_or_send(sm, 0x9df);
}

/// The scroll packet of an area into the network send buffer: the position to 0, command 0xf (box line
/// `network.h 133`), the area as a word (134) and the first visible row as a dword (136).
unsafe fn area_scroll_row_packet(area: u32, row: u32) {
    write_u32(NET_SEND_BUFFER + network_buffer::POS, 0);
    if network_buffer_room(NET_SEND_BUFFER, 1, 0x85) {
        write_u8(network_buffer_cursor(NET_SEND_BUFFER), 0xf);
        network_buffer_advance(NET_SEND_BUFFER, 1);
    }
    if network_buffer_room(NET_SEND_BUFFER, 2, 0x86) {
        write_u16(network_buffer_cursor(NET_SEND_BUFFER), area as u16);
        network_buffer_advance(NET_SEND_BUFFER, 2);
    }
    if network_buffer_room(NET_SEND_BUFFER, 4, 0x88) {
        write_u32(network_buffer_cursor(NET_SEND_BUFFER), row); // tel40_1: the first visible row
        network_buffer_advance(NET_SEND_BUFFER, 4);
    }
}

/// Rule: when an area scrolls, its first visible row is written as a packet (command 0xf) and sent to the remote
/// screen when channel 0 shows the remote Display; a local game only fills the buffer.
///
/// `area_scroll_row_packet` into `NET_SEND_BUFFER`; when channel 0's record shows the remote Display
/// (`screen_manager::DISPLAY` = `SCRMAN_ECHO_DISPLAY` of the global screen manager), `Network::send_packet` on the
/// session with the buffer and the channel of the screen being applied (`SCREEN_CHANNEL`, a word); failing, the box
/// `scrman.cpp 2571`. The record looked at is channel 0's, whatever the current channel.
/// Original: `ScreenManager::send_area_scroll_row` (0x800330, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_send_area_scroll_row(sm: usize, area: u32, row: u32) {
    area_scroll_row_packet(area, row);
    if read_usize(sm + screen_manager::DISPLAY) != G_SCREEN_MANAGER + SCRMAN_ECHO_DISPLAY {
        return;
    }
    covered!("tel40_1 scroll row to the remote screen");
    if exe::network_send_packet(read_usize(sm + screen_history::NETWORK), read_usize(NET_SEND_BUFFER), read_u32(NET_SEND_BUFFER + network_buffer::POS),
        read_u16(SCREEN_CHANNEL) as u32) == 0 {
        tel26_error_box(SCRMAN_CPP, 0xa0b);
    }
}

/// The status packet into the screen manager's buffer: command 7, the two numbers as dwords (box line `network.h 136`
/// each) and the text (`scrman_net_put_text`: its length with the terminator, then the text with no room check).
unsafe fn status_message_packet(sm: usize, text: usize, value: u32, total: u32) {
    scrman_command_packet(sm, 7); // tel40_1: the status command
    scrman_net_put(sm, value, 4, 0x88);
    scrman_net_put(sm, total, 4, 0x88);
    scrman_net_put_text(sm, text);
}

/// Rule: a progress or status message (a text and two numbers, the progress bar of the long jobs) is shown on the given
/// channel's screen, or on every channel's for channel −1: applied to the local Display for a local record, sent to the
/// player for a remote one.
///
/// `status_message_packet`. Channel −1 (the word 0xffff): for each of the 16 channels with records, record 0's player 0
/// has the packet applied to the local Display (`Display::process_command_packet` 0x564080 with the session and 0),
/// another player has it sent (`Network::send_packet`, failing: the box `scrman.cpp 3212`). One channel: record 0's
/// player 0 applies it locally (even with no records), another sends it when the channel has records (failing:
/// `scrman.cpp 3188`).
/// Original: `ScreenManager::send_network_status_message` (0x802f80, scrman.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn screen_manager_send_network_status_message(sm: usize, text: usize, value: u32, total: u32, channel: u32) {
    use screen_history as h;
    status_message_packet(sm, text, value, total);
    let apply = || exe::display_process_command_packet(sm + h::LOCAL_DISPLAY, read_usize(sm + h::NETWORK), 0, sm + h::NET_BUFFER);
    let send = |player: u16, line: u32| {
        if exe::network_send_packet(read_usize(sm + h::NETWORK), read_usize(sm + h::NET_BUFFER), read_u32(sm + h::NET_BUFFER_POS), player as u32) == 0 {
            tel26_error_box(SCRMAN_CPP, line);
        }
    };
    if channel as u16 != 0xffff {
        let c = channel as i16 as isize;
        let player = read_u16((sm as isize + h::NETWORK_PLAYER as isize + c * h::CHANNEL_SIZE as isize) as usize);
        if player == 0 {
            apply();
            return;
        }
        covered!("tel40_1 status message to a remote channel");
        if read_i16((sm as isize + h::CHANNEL_RECORD_COUNTS as isize + c * 2) as usize) > 0 {
            send(player, 0xc74);
        }
        return;
    }
    covered!("tel40_1 status message to every channel");
    for c in 0..0x10usize {
        if read_i16(channel_count_at(sm, c)) <= 0 {
            continue;
        }
        let player = read_u16(channel_record_at(sm, c, 0));
        if player == 0 {
            apply();
        } else {
            send(player, 0xc8c);
        }
    }
}

/// The regions the player records touch: the 16 channels of records, their counts, the current and the next channel and
/// the two hooks.
fn scrman_records_regions(sm: usize) -> [(usize, usize); 4] {
    [(sm + screen_history::NETWORK_PLAYER, 0x10 * screen_history::CHANNEL_SIZE), (sm + screen_history::CHANNEL_RECORD_COUNTS, 0x20),
        (sm + screen_manager::CURRENT_SLOT, 4), (sm + SCRMAN_JOIN_HOOK, 8)]
}

/// Side by side over the player records (`scrman_records_regions`).
unsafe fn compare_scrman_records(name: &str, sm: usize, original: impl FnOnce(), rust: impl FnOnce()) {
    let reg = scrman_records_regions(sm);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&reg, original, rust);
    SCOREBOARD.record(same, || format!("{}: player records differ {:?}", name, first_differences(&mo, &mr)));
}

/// Side by side over the packet a sender builds before delivering it: the copy's builder runs first into the
/// NetworkBuffer `buf`, its bytes are kept and the buffer (position and the `need` bytes) restored; then the original
/// runs once, building and delivering, and the bytes it left are compared with the copy's. No room for the `need`
/// bytes: only the original (the error box does not run twice).
unsafe fn compare_packet_build(name: &str, buf: usize, need: usize, build: impl FnOnce(), original: impl FnOnce()) {
    let data = read_usize(buf + network_buffer::DATA);
    if data == 0 || need > read_u32(buf + network_buffer::CAPACITY) as usize {
        return original();
    }
    let pos = read_u32(buf + network_buffer::POS);
    let saved = core::slice::from_raw_parts(data as *const u8, need).to_vec();
    build();
    let n = read_u32(buf + network_buffer::POS) as usize;
    let copy = core::slice::from_raw_parts(data as *const u8, n.min(need)).to_vec();
    core::ptr::copy_nonoverlapping(saved.as_ptr(), data as *mut u8, need);
    write_u32(buf + network_buffer::POS, pos);
    original();
    let got = core::slice::from_raw_parts(data as *const u8, copy.len());
    SCOREBOARD.record(n == need && got == &copy[..], || format!("{}: packet {:x?} (length {}), copy {:x?} (length {})", name, got, need, copy, n));
}

// simplification: the session is Winsock and sockets: it does not run twice. Original only.
/// Comparer for `screen_manager_get_local_addresses`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_get_local_addresses(sm: usize, out: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SCREEN_MANAGER_GET_LOCAL_ADDRESSES)(sm, out)
}

// simplification: it opens Winsock and the broadcast sockets: it does not run twice. Original only; the plan: the hooks
// kept, and the session set exactly when it returns nonzero (allocation failure aside).
/// Comparer for `screen_manager_start_network_session`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_start_network_session(sm: usize, on_join: usize, on_leave: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SCREEN_MANAGER_START_NETWORK_SESSION)(sm, on_join, on_leave);
    let net = read_usize(sm + screen_history::NETWORK);
    SCOREBOARD.record(read_usize(sm + SCRMAN_JOIN_HOOK) == on_join && read_usize(sm + SCRMAN_LEAVE_HOOK) == on_leave && (r == 0) == (net == 0),
        || format!("start_network_session: returned {:#x}, session {:#x}, hooks {:#x} {:#x}", r, net, read_usize(sm + SCRMAN_JOIN_HOOK), read_usize(sm + SCRMAN_LEAVE_HOOK)));
    r
}

/// Comparer for `scrman_add_player_record`: side by side over the records; with a join hook (it opens screens) only
/// the original.
pub(crate) unsafe extern "C" fn compare_scrman_add_player_record(player: u32) {
    let o = original_fn::<unsafe extern "C" fn(u32)>(IDX_SCRMAN_ADD_PLAYER_RECORD);
    let sm = G_SCREEN_MANAGER;
    let client = read_usize(G_SCREEN_MANAGER_NETWORK) != 0 && read_u32(G_SCREEN_MANAGER_NETWORK_HOST_FLAG) == 0;
    if !client && read_usize(sm + SCRMAN_JOIN_HOOK) != 0 {
        // simplification: the join hook opens screens: it does not run twice
        return o(player);
    }
    compare_scrman_records("scrman_add_player_record", sm, || o(player), || scrman_add_player_record(player));
}

// simplification: removing a record frees its screen history: it does not run twice. Original only; the plan: on the
// host no channel left whose record 0 is the player and has records.
/// Comparer for `scrman_remove_player_records`.
pub(crate) unsafe extern "C" fn compare_scrman_remove_player_records(player: u32) {
    original_fn::<unsafe extern "C" fn(u32)>(IDX_SCRMAN_REMOVE_PLAYER_RECORDS)(player);
    let sm = G_SCREEN_MANAGER;
    if read_u32(G_SCREEN_MANAGER_NETWORK_HOST_FLAG) == 0 {
        return;
    }
    let left = (0..0x10usize).find(|&c| read_u16(channel_record_at(sm, c, 0)) == player as u16 && read_i16(channel_count_at(sm, c)) != 0);
    SCOREBOARD.record(left.is_none(), || format!("scrman_remove_player_records(player {}): channel {:?} still has records", player as u16, left));
}

/// Comparer for `screen_manager_add_manager_record`: side by side over the records; without a key and with a join hook
/// (it opens screens) only the original.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_add_manager_record(sm: usize, on_join: usize, on_leave: usize, key: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_SCREEN_MANAGER_ADD_MANAGER_RECORD);
    if key == 0 && on_join != 0 {
        // simplification: the join hook opens screens: it does not run twice
        return o(sm, on_join, on_leave, key);
    }
    compare_scrman_records("screen_manager_add_manager_record", sm, || o(sm, on_join, on_leave, key), || screen_manager_add_manager_record(sm, on_join, on_leave, key));
}

/// Comparer for `screen_manager_send_clear_click`: the packet side by side, delivered once by the original.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_send_clear_click(sm: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SCREEN_MANAGER_SEND_CLEAR_CLICK);
    compare_packet_build("send_clear_click", sm + screen_history::NET_BUFFER, 1, || scrman_command_packet(sm, 0x11), || o(sm));
}

/// Comparer for `screen_manager_set_pending_click_widget`: the packet side by side, delivered once by the original.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_pending_click_widget(sm: usize, widget: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SCREEN_MANAGER_SET_PENDING_CLICK_WIDGET);
    compare_packet_build("set_pending_click_widget", sm + screen_history::NET_BUFFER, 3, || {
        scrman_command_packet(sm, 0x13);
        scrman_net_put(sm, widget, 2, 0x86);
    }, || o(sm, widget));
}

/// Comparer for `screen_manager_send_area_scroll_row`: in a local game (channel 0 not on the remote Display) side by
/// side over the network send buffer (its header and the 7 bytes); otherwise the packet side by side and the send once
/// by the original.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_send_area_scroll_row(sm: usize, area: u32, row: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_SCREEN_MANAGER_SEND_AREA_SCROLL_ROW);
    let data = read_usize(NET_SEND_BUFFER + network_buffer::DATA);
    if data == 0 || read_u32(NET_SEND_BUFFER + network_buffer::CAPACITY) < 7 {
        // simplification: the error boxes do not run twice
        return o(sm, area, row);
    }
    if read_usize(sm + screen_manager::DISPLAY) == G_SCREEN_MANAGER + SCRMAN_ECHO_DISPLAY {
        return compare_packet_build("send_area_scroll_row", NET_SEND_BUFFER, 7, || area_scroll_row_packet(area, row), || o(sm, area, row));
    }
    let reg = [(NET_SEND_BUFFER, 12), (data, 7)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(sm, area, row), || screen_manager_send_area_scroll_row(sm, area, row));
    SCOREBOARD.record(same, || format!("send_area_scroll_row(area {}, row {}): differences {:?}", area as u16, row as i32, first_differences(&mo, &mr)));
}

/// Comparer for `screen_manager_send_network_status_message`: the packet side by side, applied or sent once by the
/// original (the local Display draws the progress).
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_send_network_status_message(sm: usize, text: usize, value: u32, total: u32, channel: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(IDX_SCREEN_MANAGER_SEND_NETWORK_STATUS_MESSAGE);
    let need = 1 + 4 + 4 + 2 + c_string_len(text) + 1;
    compare_packet_build("send_network_status_message", sm + screen_history::NET_BUFFER, need, || status_message_packet(sm, text, value, total),
        || o(sm, text, value, total, channel));
}

// ---------- batch 322: the network object's life and its packets: destroying a TcpIp and a Network, closing a player's connection, sending a packet and building and destroying a network buffer (national_teams_screens.cpp) ----------

/// The TcpIp methods the Network calls (tcpip.cpp, thiscall): close every socket and shut Winsock down, close one
/// connection slot (short), send on one connection slot (slot short, data, length) -> bytes sent or −2 when closed.
const FN_TCPIP_CLOSE_ALL_SOCKETS: usize = 0x8ae2a0;
const FN_TCPIP_CLOSE_CONNECTION: usize = 0x8ae340;
const FN_TCPIP_SEND_TO_CONNECTION: usize = 0x8ae8d0;
/// "E:\dev\CM3\cm3 00-01\si\code\network.CPP" (the error boxes of this file).
const NETWORK_CPP: usize = 0xa649a8;
/// The TcpIp's connection table pointer (+0, freed by the destructors) and the Network's connection slots as seen from
/// a 1-based player number (slot k is at `network_game::SOCKETS` + 2 × (k − 1)).
const TCPIP_CONNECTIONS: usize = 0x0;
const NETWORK_PLAYER_SLOTS: usize = 0x4b8;

/// The TcpIp base's destructor body: every socket closed (`TcpIp::close_all_sockets`) and the connection table freed
/// (`operator delete`, when set; the pointer is left as it was).
unsafe fn tcpip_close_and_free(this: usize) {
    let close: unsafe extern "thiscall" fn(usize) = tcpip_fn(FN_TCPIP_CLOSE_ALL_SOCKETS);
    close(this);
    let table = read_usize(this + TCPIP_CONNECTIONS);
    if table != 0 {
        exe::crt_operator_delete(table);
    }
}

/// Rule: a TcpIp object is destroyed by closing all its sockets and freeing its connection table.
///
/// `tcpip_close_and_free`. Only reached from the unwind handlers of `Network::Network` (an exception while the TcpIp
/// base is built).
/// Original: `TcpIp::~TcpIp` (0x77a870, national_teams_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn tcpip_destructor(this: usize) {
    covered!("tel40_2 TcpIp destroyed");
    tcpip_close_and_free(this);
}

/// Rule: a Network object is destroyed; when it hosts a game it first says goodbye to the list servers ("CM3 LIST SERVER
/// BYE" with its address), then all its sockets are closed and its connection table freed.
///
/// Hosting (`network_game::HOSTING`): the datagram "CM3 LIST SERVER BYE" (`NETWORK_LIST_BYE`, without its terminator)
/// followed by the 50 bytes of server address 0 (`network_game::SERVER_ADDRESSES`), sent by `TcpIp::send_datagram`
/// with the length of the text + 100, the game's port and no address (broadcast). The original's buffer is a stack
/// local, so the last 50 bytes sent are whatever its stack held; the copy sends zeros there. Then
/// `TcpIp::close_all_sockets` and the TcpIp base's destructor (`tcpip_close_and_free`: closing them again and freeing
/// the table).
/// Original: `Network::~Network` (0x77a890, national_teams_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn network_destructor(this: usize) {
    covered!("tel40_2 Network destroyed");
    if read_u32(this + network_game::HOSTING) != 0 {
        let mut buffer = [0u8; 0x7d0];
        let p = buffer.as_mut_ptr() as usize;
        let n = network_strlen(NETWORK_LIST_BYE);
        core::ptr::copy(NETWORK_LIST_BYE as *const u8, p as *mut u8, n);
        core::ptr::copy((this + network_game::SERVER_ADDRESSES) as *const u8, (p + n) as *mut u8, network_game::SERVER_TEXT);
        let send: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32) -> i32 = tcpip_fn(FN_TCPIP_SEND_DATAGRAM);
        send(this, p, (n + 0x64) as u32, NETWORK_PORT, 0, 0, 0); // tel40_2: the goodbye datagram
    }
    let close: unsafe extern "thiscall" fn(usize) = tcpip_fn(FN_TCPIP_CLOSE_ALL_SOCKETS);
    close(this);
    tcpip_close_and_free(this);
}

/// Rule: the connection of one of the sixteen players (numbered 1 to 16) is closed; any other number does nothing.
///
/// For `player` 1..16 (a short): `TcpIp::close_connection` with the connection slot of that player
/// (`NETWORK_PLAYER_SLOTS` + 2 × player, a word); the slot is not marked free here.
/// Original: `Network::release_slot_socket` (0x77ac00, national_teams_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_release_slot_socket(this: usize, player: u32) {
    let k = player as i16;
    if k <= 0 || k > 0x10 {
        return;
    }
    covered!("tel40_2 close a player's connection");
    let close: unsafe extern "thiscall" fn(usize, u32) = tcpip_fn(FN_TCPIP_CLOSE_CONNECTION);
    close(this, read_u16(this + NETWORK_PLAYER_SLOTS + 2 * k as usize) as u32);
}

/// Sends one packet on a connection slot: its length as 4 single-byte sends (little-endian), then the data; true
/// unless the data send returned −2 (the connection closed).
unsafe fn network_send_on(this: usize, slot: u16, data: usize, length: u32) -> bool {
    let send: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> i32 = tcpip_fn(FN_TCPIP_SEND_TO_CONNECTION);
    let header = length.to_le_bytes();
    for k in 0..4 {
        send(this, slot as u32, header.as_ptr() as usize + k, 1);
    }
    send(this, slot as u32, data, length) != -2 // tel40_2: only a closed connection fails
}

/// Rule: a packet goes to one player's connection, or to every connection for player 0 when hosting, preceded by its
/// 4-byte length; a connection found closed is freed (and the leave callback told, when hosting). Returns whether the
/// (last) send worked.
///
/// A length of 0 or less: the box `network.CPP 211` and 0. A length of 50000 or more: 0. Hosting
/// (`network_game::HOSTING`): player 0 (the low word) walks the 16 connection slots (`network_game::SOCKETS`); each one
/// in use (not 0xffff) gets `network_send_on`; a closed one is marked free (0xffff) and the leave callback
/// (`network_game::ON_DISCONNECT`, when set) gets the slot + 1. Returns the result of the last slot in use (0 with
/// none). Another player: his slot (`NETWORK_PLAYER_SLOTS` + 2 × player); free, 0; otherwise `network_send_on`, closed
/// -> freed and the callback gets `player`. A client: slot 0 the same way, freed without the callback.
/// Original: `Network::send_packet` (0x77ac30, national_teams_screens.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn network_send_packet(this: usize, data: usize, length: i32, player: u32) -> u32 {
    use network_game as g;
    if length <= 0 {
        tel26_error_box(NETWORK_CPP, 0xd3);
        return 0;
    }
    if length >= 0xc350 {
        return 0;
    }
    covered!("tel40_2 send a packet");
    let callback = |arg: u32| {
        let f = read_usize(this + g::ON_DISCONNECT);
        if f != 0 {
            let f: unsafe extern "C" fn(u32) = core::mem::transmute(f);
            f(arg);
        }
    };
    if read_u32(this + g::HOSTING) == 0 {
        let slot = read_u16(this + g::SOCKETS);
        if slot == 0xffff {
            return 0;
        }
        let ok = network_send_on(this, slot, data, length as u32);
        if !ok {
            write_u16(this + g::SOCKETS, 0xffff);
        }
        return ok as u32;
    }
    if player as u16 != 0 {
        let at = (this as isize + NETWORK_PLAYER_SLOTS as isize + player as i16 as isize * 2) as usize;
        let slot = read_u16(at);
        if slot == 0xffff {
            return 0;
        }
        let ok = network_send_on(this, slot, data, length as u32);
        if !ok {
            write_u16(at, 0xffff);
            callback(player);
        }
        return ok as u32;
    }
    covered!("tel40_2 send a packet to everyone");
    let mut ok = false;
    for k in 0..0x10usize {
        let slot = read_u16(this + g::SOCKETS + 2 * k);
        if slot == 0xffff {
            continue;
        }
        ok = network_send_on(this, slot, data, length as u32);
        if !ok {
            write_u16(this + g::SOCKETS + 2 * k, 0xffff); // tel40_2: the closed connection freed
            callback(k as u32 + 1);
        }
    }
    ok as u32
}

/// Rule: a network buffer starts empty with the given capacity and its own storage; with no memory the game stops.
///
/// Position 0, capacity = `capacity`, the storage `operator new(capacity)`; null: the box `network.CPP 562`,
/// `game_fatal_cleanup` (0x5cc7d0), the empty routine 0x5a8f60 and `exit(-1)`. Returns the buffer.
/// Original: `NetworkBuffer::NetworkBuffer` (0x77b630, national_teams_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_buffer_construct(this: usize, capacity: u32) -> usize {
    write_u32(this + network_buffer::POS, 0);
    write_u32(this + network_buffer::CAPACITY, capacity);
    let data = exe::crt_operator_new(capacity);
    write_usize(this + network_buffer::DATA, data);
    if data == 0 {
        tel26_error_box(NETWORK_CPP, 0x232);
        let cleanup: unsafe extern "C" fn() = core::mem::transmute(FN_GAME_FATAL_CLEANUP);
        cleanup();
        let noop: unsafe extern "C" fn() = core::mem::transmute(FN_SHARED_NOOP_RET0_5A8F60);
        noop();
        let exit: unsafe extern "C" fn(i32) = core::mem::transmute(FN_CRT_EXIT);
        exit(-1);
    }
    this
}

/// Rule: a network buffer is destroyed by freeing its storage, when it has one, and clearing the pointer.
/// Original: `NetworkBuffer::~NetworkBuffer` (0x77b6e0, national_teams_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn network_buffer_destruct(this: usize) {
    let data = read_usize(this + network_buffer::DATA);
    if data != 0 {
        exe::crt_operator_delete(data);
    }
    write_usize(this + network_buffer::DATA, 0); // tel40_2: the pointer cleared
}

// simplification: closing sockets and freeing memory do not run twice. Original only.
/// Comparer for `tcpip_destructor`.
pub(crate) unsafe extern "thiscall" fn compare_tcpip_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TCPIP_DESTRUCTOR)(this)
}

// simplification: the datagram, the sockets and the table do not run twice. Original only.
/// Comparer for `network_destructor`.
pub(crate) unsafe extern "thiscall" fn compare_network_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_NETWORK_DESTRUCTOR)(this)
}

// simplification: closing a socket does not run twice. Original only.
/// Comparer for `network_release_slot_socket`.
pub(crate) unsafe extern "thiscall" fn compare_network_release_slot_socket(this: usize, player: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_NETWORK_RELEASE_SLOT_SOCKET)(this, player)
}

// simplification: a packet is not sent twice. Original only; the plan: hosting and sending to everyone, a failed send
// leaves its slot free.
/// Comparer for `network_send_packet`.
pub(crate) unsafe extern "thiscall" fn compare_network_send_packet(this: usize, data: usize, length: i32, player: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(IDX_NETWORK_SEND_PACKET)(this, data, length, player)
}

// simplification: the storage is allocated once (and a failure exits). Original only; the plan: position 0, the
// capacity kept, the storage set, the buffer returned.
/// Comparer for `network_buffer_construct`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_construct(this: usize, capacity: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(IDX_NETWORK_BUFFER_CONSTRUCT)(this, capacity);
    SCOREBOARD.record(r == this && read_u32(this + network_buffer::POS) == 0 && read_u32(this + network_buffer::CAPACITY) == capacity
        && read_usize(this + network_buffer::DATA) != 0, || format!("NetworkBuffer::NetworkBuffer({}): returned {:#x}, buffer {:x?}", capacity, r,
        core::slice::from_raw_parts(this as *const u8, 12)));
    r
}

// simplification: the storage is freed once. Original only; the plan: the pointer cleared.
/// Comparer for `network_buffer_destruct`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_destruct(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_NETWORK_BUFFER_DESTRUCT)(this);
    SCOREBOARD.record(read_usize(this + network_buffer::DATA) == 0, || format!("NetworkBuffer::~NetworkBuffer: storage {:#x} left", read_usize(this + network_buffer::DATA)));
}

// ---------- batch 322: the Display's network side: a text entry's text back to its caller and a remote player's click sent to the server (display.cpp) ----------

/// The NetworkBuffer writers this file calls by address (thiscall, ret 4): put_long (0x5694a0, box line `network.h 136`;
/// not ported).
const FN_NETWORKBUFFER_PUT_LONG: usize = 0x5694a0;
/// A widget's dword the click packets send as its last field (`Guio::get_fields` gives +0x54, the entry target, but
/// the packets send +0x58) [inference].
const GUIO_CLICK_TAIL: usize = 0x58;

/// Rule: a text entry widget with a caller's buffer gets its text copied back there.
///
/// With flag 8 (`display_item::FLAGS`, a text entry) and a target (`guio_fields::ENTRY_TARGET`), the text
/// (`display_item::TEXT`) is copied there with its terminator.
/// Original: `guio_copy_text_to_caller` (0x567b00, display.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn guio_copy_text_to_caller(item: usize) {
    let target = read_usize(item + guio_fields::ENTRY_TARGET);
    if read_u8(item + display_item::FLAGS) & 8 != 0 && target != 0 {
        tel26_copy_text(target as *mut u8, (item + display_item::TEXT) as *const u8); // tel40_3: the entry's text back
    }
}

/// The fields `Guio::get_fields` (0x5ee4a0, thiscall, 19 out pointers, ret 0x4c) gives of a widget, in its order: the
/// flags, the rectangle, the grid column and row, the style, the two colours (words), the third colour, the font and the
/// text colour (words), the text pointer (the text, or 5 bytes in for an item of a popup area), the length limit, the id,
/// the data, the area (word) and the entry target.
#[derive(Default)]
struct GuioFields {
    flags: u32,
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
    column: u32,
    row: u32,
    style: u32,
    colour: u16,
    highlight: u16,
    colour_c: u32,
    font: u16,
    text_colour: u16,
    text: usize,
    max_length: u32,
    id: u32,
    data: u32,
    area: u16,
    target: u32,
}

/// `Guio::get_fields` (0x5ee4a0, by its address) on `item`.
unsafe fn click_guio_fields(item: usize) -> GuioFields {
    type Get = unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u16, *mut u16,
        *mut u32, *mut u16, *mut u16, *mut usize, *mut u32, *mut u32, *mut u32, *mut u16, *mut u32);
    let get: Get = core::mem::transmute(0x5ee4a0usize);
    let mut f = GuioFields::default();
    get(item, &mut f.flags, &mut f.x1, &mut f.y1, &mut f.x2, &mut f.y2, &mut f.column, &mut f.row, &mut f.style, &mut f.colour, &mut f.highlight,
        &mut f.colour_c, &mut f.font, &mut f.text_colour, &mut f.text, &mut f.max_length, &mut f.id, &mut f.data, &mut f.area, &mut f.target);
    f
}

/// The widget's text as the click packet copies it (the text with its terminator; empty with no text).
unsafe fn guio_text_copy(text: usize) -> Vec<u8> {
    if text == 0 {
        return vec![0];
    }
    let n = c_string_len(text) + 1;
    core::slice::from_raw_parts(text as *const u8, n).to_vec()
}

/// One inline write into the network send buffer (`NET_SEND_BUFFER`): with room for `size` bytes it is written and
/// the position advances; without, the box `network.h <line>`.
unsafe fn send_buffer_put(value: u32, size: u32, line: u32) {
    if !network_buffer_room(NET_SEND_BUFFER, size, line) {
        return;
    }
    let at = network_buffer_cursor(NET_SEND_BUFFER);
    match size {
        1 => write_u8(at, value as u8),
        2 => write_u16(at, value as u16),
        _ => write_u32(at, value),
    }
    network_buffer_advance(NET_SEND_BUFFER, size);
}

/// The "Network Error" box of display.cpp at `line` (as `display_send_keepalive`'s): the version string, then the box
/// with the game's title and message texts (no mouse reset).
unsafe fn display_network_error(line: u32) {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(DISPLAY_CPP, 0usize, 0usize, name.as_mut_ptr(), ext.as_mut_ptr());
    let v = version(name.as_mut_ptr(), ext.as_mut_ptr(), line);
    format(0xae24d0, 0x9870e8, v, name.as_ptr(), ext.as_ptr(), line);
    text(0xae28bc, 0x9c3814);
    text(0xae26c4, 0x9c37b0, 0xae24d0usize);
    exe::gui_show_message_box(0xae28bcusize, 0xae26c4usize, 0);
}

/// Rule: a remote player's click is sent to the server: command 6 with the widget and all its current fields, and,
/// when the widget was dropped on another widget, the drop rectangle and that widget's fields too. Returns whether it
/// was sent.
///
/// Into `NET_SEND_BUFFER` (position 0), inline writes (box `network.h <line>` without room): command 6 (133), the widget
/// (134); `Guio::get_fields` of the widget (`display_item_at`); the flags (141), x1, y1, x2, y2 (134 each), the column's
/// low byte (133), the row (134), the style (141), the colour and highlight colour (139 each), the third colour (141), the
/// font's low byte (133), the text colour (139), the text (its length with the terminator by `NetworkBuffer::put_short`,
/// then the bytes with no room check), the length limit (134), the id (136), the data (137); then through the
/// NetworkBuffer methods: the area (`put_short`), the widget's `GUIO_CLICK_TAIL` dword (`put_int`) and the drop target
/// (`display_item::DROP_TARGET`, `put_short`). With a drop target: 16 (`put_short`) and the four dwords of
/// `display_item::DROP_RECT` (`put_long` 0x5694a0); a target other than the widget adds its fields: flags (`put_ulong`),
/// x1, y1, x2, y2 (`put_short`), column (`put_char`), row (`put_short`), style (`put_ulong`), colours (`put_ushort`),
/// third colour (`put_ulong`), font (`put_char`), text colour (`put_ushort`), the text block (`put_sized_block` with the
/// length with the terminator), the length limit (`put_short`), the id (`put_long`), the data (`put_int`), the area
/// (`put_short`) and its tail dword (`put_int`). Then `Network::send_packet(session, buffer, position, SCREEN_CHANNEL)`:
/// failing, the "Network Error" box (display.cpp 3322) and 0; otherwise the Display's `display_life::DWORD_551` = 0 and 1.
/// Original: `Display::send_widget_click` (0x568560, display.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn display_send_widget_click(d: usize, session: usize, widget: u32) -> u32 {
    covered!("tel40_3 a remote click sent");
    let b = NET_SEND_BUFFER;
    write_u32(b + network_buffer::POS, 0);
    send_buffer_put(6, 1, 0x85); // tel40_3: the click command
    send_buffer_put(widget, 2, 0x86);
    let item = display_item_at(d, widget as i16);
    let f = click_guio_fields(item);
    let tail = read_u32(item + GUIO_CLICK_TAIL);
    let text = guio_text_copy(f.text);
    send_buffer_put(f.flags, 4, 0x8d);
    for v in [f.x1, f.y1, f.x2, f.y2] {
        send_buffer_put(v, 2, 0x86);
    }
    send_buffer_put(f.column & 0xff, 1, 0x85);
    send_buffer_put(f.row, 2, 0x86);
    send_buffer_put(f.style, 4, 0x8d);
    send_buffer_put(f.colour as u32, 2, 0x8b);
    send_buffer_put(f.highlight as u32, 2, 0x8b);
    send_buffer_put(f.colour_c, 4, 0x8d);
    send_buffer_put(f.font as u32 & 0xff, 1, 0x85);
    send_buffer_put(f.text_colour as u32, 2, 0x8b);
    let len = text.len() as u32;
    network_buffer_put_short(b, len);
    let n = len as i16 as i32 as usize;
    core::ptr::copy(text.as_ptr(), network_buffer_cursor(b) as *mut u8, n);
    network_buffer_advance(b, len as i16 as u32);
    send_buffer_put(f.max_length, 2, 0x86);
    send_buffer_put(f.id, 4, 0x88);
    send_buffer_put(f.data, 4, 0x89);
    network_buffer_put_short(b, f.area as u32);
    network_buffer_put_int(b, tail);
    let target = read_u16(item + display_item::DROP_TARGET);
    network_buffer_put_short(b, target as u32);
    if target != 0xffff {
        covered!("tel40_3 a remote drop sent");
        let put_long: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_NETWORKBUFFER_PUT_LONG);
        let rect = [0, 4, 8, 12].map(|k| read_u32(item + display_item::DROP_RECT + k));
        network_buffer_put_short(b, 0x10);
        for v in rect {
            put_long(b, v);
        }
        if target != widget as u16 {
            let other = display_item_at(d, target as i16);
            let g = click_guio_fields(other);
            let tail = read_u32(other + GUIO_CLICK_TAIL);
            let text = guio_text_copy(g.text);
            network_buffer_put_ulong(b, g.flags);
            for v in [g.x1, g.y1, g.x2, g.y2] {
                network_buffer_put_short(b, v);
            }
            network_buffer_put_char(b, g.column);
            network_buffer_put_short(b, g.row);
            network_buffer_put_ulong(b, g.style);
            network_buffer_put_ushort(b, g.colour as u32);
            network_buffer_put_ushort(b, g.highlight as u32);
            network_buffer_put_ulong(b, g.colour_c);
            network_buffer_put_char(b, g.font as u32);
            network_buffer_put_ushort(b, g.text_colour as u32);
            exe::networkbuffer_put_sized_block(b, text.as_ptr() as usize, text.len() as u32);
            network_buffer_put_short(b, g.max_length);
            put_long(b, g.id);
            network_buffer_put_int(b, g.data);
            network_buffer_put_short(b, g.area as u32);
            network_buffer_put_int(b, tail);
        }
    }
    if exe::network_send_packet(session, read_usize(b), read_u32(b + network_buffer::POS), read_u16(SCREEN_CHANNEL) as u32) == 0 {
        display_network_error(0xcfa);
        return 0;
    }
    write_u32(d + display_life::DWORD_551, 0);
    1
}

/// Comparer for `guio_copy_text_to_caller`: side by side over the caller's buffer (the text's length with its
/// terminator).
pub(crate) unsafe extern "thiscall" fn compare_guio_copy_text_to_caller(item: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_GUIO_COPY_TEXT_TO_CALLER);
    let target = read_usize(item + guio_fields::ENTRY_TARGET);
    if target == 0 || read_u8(item + display_item::FLAGS) & 8 == 0 {
        return o(item);
    }
    let reg = [(target, c_string_len(item + display_item::TEXT) + 1)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(item), || guio_copy_text_to_caller(item));
    SCOREBOARD.record(same, || format!("guio_copy_text_to_caller(item {:#x}): differences {:?}", item, first_differences(&mo, &mr)));
}

// simplification: the packet is sent to the server: it does not run twice. Original only; the plan: the packet starts
// with command 6 and the widget, and a sent click clears the Display's DWORD_551.
/// Comparer for `display_send_widget_click`.
pub(crate) unsafe extern "thiscall" fn compare_display_send_widget_click(d: usize, session: usize, widget: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_DISPLAY_SEND_WIDGET_CLICK)(d, session, widget);
    let data = read_usize(NET_SEND_BUFFER);
    let head_ok = data == 0 || read_u32(NET_SEND_BUFFER + network_buffer::POS) < 3 || (read_u8(data) == 6 && read_u16(data + 1) == widget as u16);
    SCOREBOARD.record(head_ok && (r == 0 || read_u32(d + display_life::DWORD_551) == 0), || format!("send_widget_click(widget {}): returned {}, packet head {:x?}",
        widget as i16, r, if data == 0 { &[][..] } else { core::slice::from_raw_parts(data as *const u8, 3) }));
    r
}

// ---------- batch 322: the human managers of a restored game: who takes the screens and who goes to the computer (Setup.cpp) ----------

/// The size of a human manager record (`human_manager_record`, 42 735 bytes) and the match day object
/// (`set_computer_controlled` tells it when it is set).
const HUMAN_MANAGER_RECORD_SIZE: usize = 0xa6ef;
const G_MATCH_DAY_OBJECT: usize = 0xb67a50;

/// Rule: when a saved game is restored (`-load` and the network game), each human manager with no password (in a game
/// with no network name) takes the screen manager: the first one the current record, the others a record each; a
/// manager with a password who is not on holiday goes to computer control until he resumes it. Returns whether
/// Manager Status must open (a manager locked, or one with a password on holiday).
///
/// For k from 0 to 15, the person `G_STAFF_COUNT` − 1 − k of the person table (`G_STAFF_TABLE`, `staff::SIZE` bytes
/// each; the count read anew each time) with `staff::CLASSIFICATION` 1 (a human manager): with an empty password
/// (`HUMAN_MANAGER::get_password` 0x5fde40) and `G_NETWORK_GAME_NAMED_FLAG` 0, a current record (current channel) with no
/// key gets him as its key and the join and leave hooks (`NETWORK_JOIN_CALLBACK`, `NETWORK_LEAVE_CALLBACK`) are set;
/// with a key he gets a record of his own (`ScreenManager::add_manager_record` with the two hooks and him). Otherwise,
/// not on holiday (`HUMAN_MANAGER::is_on_holiday` 0x5fd210): `HUMAN_MANAGER::set_computer_controlled(1, him)`
/// (0x5fe140); on holiday: the result 1. Returns 1 when the HUMAN_MANAGER's locked count (`HUMAN_MANAGER_LOCKED_COUNT`)
/// is nonzero, otherwise the result.
/// With `CM_FERIAS` (the save proof) `simulation.rs` hooks this entry first: the engine's detour is refused (the bytes
/// are its jump) and the original runs inside that wrapper; without it the copy runs.
/// Original: `setup_assign_restored_network_managers` (0x8387e0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_assign_restored_network_managers() -> u32 {
    covered!("tel40_5 restored managers");
    let sm = G_SCREEN_MANAGER;
    let mut result = 0u32;
    for k in 0..0x10isize {
        let index = read_i32(G_STAFF_COUNT) as isize - k - 1;
        let person = Staff((read_usize(G_STAFF_TABLE) as isize + index * staff::SIZE as isize) as usize);
        if person.classification() != 1 {
            continue;
        }
        let password = exe::human_manager_get_password(read_usize(G_HUMAN_MANAGER), person.addr());
        if read_u8(password) == 0 && read_u32(G_NETWORK_GAME_NAMED_FLAG) == 0 {
            let key = screen_record(sm) + screen_history::KEY;
            if read_u32(key) == 0 {
                covered!("tel40_5 the first manager takes the current record");
                write_usize(key, person.addr());
                write_usize(sm + SCRMAN_JOIN_HOOK, NETWORK_JOIN_CALLBACK);
                write_usize(sm + SCRMAN_LEAVE_HOOK, NETWORK_LEAVE_CALLBACK);
            } else {
                let add: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x7ff8f0usize);
                add(sm, NETWORK_JOIN_CALLBACK, NETWORK_LEAVE_CALLBACK, person.addr()); // tel40_5: a record of his own
            }
        } else if exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER), person.addr()) == 0 {
            covered!("tel40_5 a manager with a password goes to the computer");
            let computer: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_HUMAN_MANAGER_SET_COMPUTER_CONTROLLED);
            computer(read_usize(G_HUMAN_MANAGER), 1, person.addr());
        } else {
            result = 1;
        }
    }
    if read_u32(read_usize(G_HUMAN_MANAGER) + HUMAN_MANAGER_LOCKED_COUNT) != 0 {
        return 1;
    }
    result
}

/// Comparer for `setup_assign_restored_network_managers`: side by side over the screen manager's records
/// (`scrman_records_regions`), the HUMAN_MANAGER's header (records, count, locked and watched counts) and each human
/// record's computer control flag; during a match (the match day object set, which `set_computer_controlled` tells)
/// only the original.
pub(crate) unsafe extern "C" fn compare_setup_assign_restored_network_managers() -> u32 {
    let o = original_fn::<unsafe extern "C" fn() -> u32>(IDX_SETUP_ASSIGN_RESTORED_NETWORK_MANAGERS);
    let hm = read_usize(G_HUMAN_MANAGER);
    if hm == 0 || read_usize(G_MATCH_DAY_OBJECT) != 0 {
        // simplification: the match day object is not told twice
        return o();
    }
    let mut reg = scrman_records_regions(G_SCREEN_MANAGER).to_vec();
    reg.push((hm, 0x10));
    let base = read_usize(hm);
    for i in 0..read_i32(hm + 4).clamp(0, 0x10) as usize {
        reg.push((base + i * HUMAN_MANAGER_RECORD_SIZE + human_manager_record::COMPUTER_CONTROLLED, 4));
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(), || setup_assign_restored_network_managers());
    SCOREBOARD.record(same && r == n, || format!("setup_assign_restored_network_managers: returned {} / {}, differences {:?}", r, n, first_differences(&mo, &mr)));
    r
}

// ---------- batch 322: the club history lists of the club screens: league seasons, cup runs, their competitions and the current league record (club_history.cpp) ----------

/// The club history store as the club screens read it (`G_CLUB_HISTORY`): +0 the league history file list, +4 the cup
/// history file list, +8 the per-club records (16 bytes, the club id the index: +0/+4 the first and last file offsets
/// of the club's league seasons, +8/+0xc those of its cup runs).
pub(crate) mod club_history_lists {
    pub(crate) const LEAGUE_LIST: usize = 0x0;
    pub(crate) const CUP_LIST: usize = 0x4;
    pub(crate) const RECORDS: usize = 0x8;
    pub(crate) const RECORD_SIZE: usize = 0x10;
    pub(crate) const LEAGUE_FIRST: usize = 0x0;
    pub(crate) const LEAGUE_LAST: usize = 0x4;
    pub(crate) const CUP_FIRST: usize = 0x8;
    pub(crate) const CUP_LAST: usize = 0xc;
    /// A league season record (+0 comp id, +4 year, +6 position, +7 the 14 bytes of the table totals) and a cup run
    /// record (+0 comp id, +4 year, +6 status, +7 stage label, +9 round code).
    pub(crate) const LEAGUE_ENTRY_SIZE: u32 = 0x15;
    pub(crate) const CUP_ENTRY_SIZE: u32 = 0xb;
    /// "E:\dev\CM3\cm3 00-01\cm3\code\club_history.cpp" (the error boxes).
    pub(crate) const CLUB_HISTORY_CPP: usize = 0x98b644;
    /// The qsort comparison of the competition ids (`compare_int_ids`).
    pub(crate) const COMPARE_INT_IDS: usize = 0x444500;
}

/// A new empty list (`operator new(0xe)` and `LLIST::LLIST`), or 0 when the allocation fails.
unsafe fn club_history_new_list() -> usize {
    let p = exe::crt_operator_new(0xe);
    if p != 0 { exe::new_llist(p) } else { 0 }
}

/// The stored records of a club's file list from the newest to the oldest, appended to `list` (created on the first
/// one); returns the list.
unsafe fn club_history_append_stored(mut list: usize, file_list: usize, first: usize, last: usize, size: u32) -> usize {
    let nodes = exe::file_llist_get_nodes(file_list, read_u32(first), read_u32(last));
    if nodes == 0 {
        return list;
    }
    write_usize(nodes + llist::CURSOR, read_usize(nodes + llist::TAIL));
    loop {
        let data = exe::llist_get_prev(nodes);
        if data == 0 {
            break;
        }
        if list == 0 {
            list = club_history_new_list();
        }
        exe::llist_add_tail(list, data, size);
    }
    exe::drop_llist(nodes);
    exe::crt_operator_delete(nodes);
    list
}

/// Builds the list of a club's league seasons, newest first, with the current season first when it is not over.
///
/// `Club_History::get_league_history(club)`: no club: the club_history.cpp 0x184 box and 0. The current record
/// (`club_history_get_current_league_record` with no stage and no reserve league): found and its league not complete,
/// a new list gets it. Then the club's stored seasons (`FILE_LLIST::get_nodes` of the league list with the club's
/// offsets), from the newest (the tail) back, each appended (21 bytes; the list created on the first one); the node
/// list destroyed. Returns the list or 0.
/// Original: `Club_History::get_league_history` (0x443380, club_history.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn club_history_get_league_history(this: usize, club: usize) -> usize {
    use club_history_lists as ch;
    if club == 0 {
        show_error_at(ch::CLUB_HISTORY_CPP, 0x184);
        return 0;
    }
    let mut list = 0usize;
    let mut rec = [0u8; 0x15];
    // simplification: the original's flag is a stack slot it never sets; a record found in a stage the club has not
    // played yet leaves it unwritten (there the copy takes 0: the record is shown)
    let mut complete = 0i32;
    if club_history_get_current_league_record(club, rec.as_mut_ptr(), &mut complete, core::ptr::null_mut(), 0) != 0
        && complete == 0 // tel42_3: the season not over
    {
        covered!("tel42_3 current league season");
        list = club_history_new_list();
        exe::llist_add_tail(list, rec.as_ptr() as usize, ch::LEAGUE_ENTRY_SIZE);
    }
    let r = read_usize(this + ch::RECORDS) + read_u32(club) as usize * ch::RECORD_SIZE;
    club_history_append_stored(list, read_usize(this + ch::LEAGUE_LIST), r + ch::LEAGUE_FIRST, r + ch::LEAGUE_LAST, ch::LEAGUE_ENTRY_SIZE)
}

/// Builds the list of a club's cup runs: its progress in each running cup first, then the stored runs, newest first.
///
/// `Club_History::get_cup_history(club)`: no club: the club_history.cpp 0x1d0 box and 0. Each competition object
/// (`G_COMP_OBJECT_TABLE`, `G_COMP_OBJECT_COUNT`) that is not a league (virtual +0xc), not a league division
/// (`comp_util_is_league_division`) and not a friendly competition (`Friendly::is_friendly_competition`): a cup record
/// (competition id, year, status 0, stage -1, round -1) filled by `Comp::find_club_progress`; with a stage reached
/// (≥ 0) it is appended (11 bytes; the list created on the first one). Then the club's stored runs (the cup list with
/// the club's +8/+0xc offsets) from the newest back. Returns the list or 0.
/// Original: `Club_History::get_cup_history` (0x443520, club_history.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn club_history_get_cup_history(this: usize, club: usize) -> usize {
    use club_history_lists as ch;
    if club == 0 {
        show_error_at(ch::CLUB_HISTORY_CPP, 0x1d0);
        return 0;
    }
    let mut list = 0usize;
    for i in 0..read_i32(G_COMP_OBJECT_COUNT).max(0) as usize {
        let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE) + i * 4);
        if comp == 0 {
            continue;
        }
        let is_league: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(read_usize(read_usize(comp) + 0xc));
        if is_league(comp) != 0 {
            continue;
        }
        let competition = read_usize(comp + league::COMPETITION);
        if exe::comp_util_is_league_division(read_u32(competition) as i32) != 0
            || exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), competition) != 0
        {
            continue;
        }
        let mut rec = [0u8; 0xb];
        rec[0..4].copy_from_slice(&read_u32(competition).to_le_bytes());
        rec[4..6].copy_from_slice(&read_u16(comp + league::YEAR).to_le_bytes());
        rec[7..0xb].fill(0xff);
        let p = rec.as_mut_ptr();
        find_club_progress(comp, club, p.add(7) as *mut i16, p.add(9) as *mut i16, p.add(6));
        if i16::from_le_bytes([rec[7], rec[8]]) > -1 { // tel42_3: a stage reached
            covered!("tel42_3 running cup progress");
            if list == 0 {
                list = club_history_new_list();
            }
            exe::llist_add_tail(list, rec.as_ptr() as usize, ch::CUP_ENTRY_SIZE);
        }
    }
    let r = read_usize(this + ch::RECORDS) + read_u32(club) as usize * ch::RECORD_SIZE;
    club_history_append_stored(list, read_usize(this + ch::CUP_LIST), r + ch::CUP_FIRST, r + ch::CUP_LAST, ch::CUP_ENTRY_SIZE)
}

/// The distinct competition ids of a history list (the first dword of each record), sorted with `compare_int_ids`, as
/// a new list of ints; the two `operator new` failures give the boxes at the lines given.
unsafe fn club_history_comp_ids(history: usize, lines: [u32; 3]) -> usize {
    use club_history_lists as ch;
    if history == 0 {
        return 0;
    }
    let count = read_u32(G_CLUB_COMP_COUNT);
    let ids = exe::crt_operator_new(count.wrapping_mul(4));
    if ids == 0 {
        show_error_at(ch::CLUB_HISTORY_CPP, lines[0]);
        return 0;
    }
    let seen = exe::crt_operator_new(count.wrapping_mul(4));
    if seen == 0 {
        show_error_at(ch::CLUB_HISTORY_CPP, lines[1]);
        exe::crt_operator_delete(ids);
        return 0;
    }
    for k in 0..read_i32(G_CLUB_COMP_COUNT).max(0) as usize {
        write_u32(seen + k * 4, 0);
    }
    let mut n = 0usize;
    write_usize(history + llist::CURSOR, read_usize(history + llist::HEAD));
    loop {
        let data = exe::llist_get_next(history);
        if data == 0 {
            break;
        }
        let id = read_u32(data) as usize;
        if read_u32(seen + id * 4) == 0 { // tel42_3: each competition once
            write_u32(ids + n * 4, id as u32);
            n += 1;
            write_u32(seen + id * 4, 1);
        }
    }
    exe::crt_operator_delete(seen);
    // the original's comparison never orders (1 for any two different ids): the order is whatever qsort leaves
    exe::crt_qsort(ids, n as u32, 4, ch::COMPARE_INT_IDS);
    let mut list = 0usize;
    for k in 0..n {
        if list == 0 {
            list = club_history_new_list();
            if list == 0 {
                show_error_at(ch::CLUB_HISTORY_CPP, lines[2]);
                exe::crt_operator_delete(ids);
                return 0;
            }
        }
        exe::llist_add_tail(list, ids + k * 4, 4);
    }
    exe::crt_operator_delete(ids);
    list
}

/// Collects the distinct competition ids of a club's league history list, "sorted", as a new list of ints.
///
/// `Club_History::get_league_comp_ids(history)`: no list: 0. Two tables of `G_CLUB_COMP_COUNT` ints (`operator new`;
/// none: the club_history.cpp 0x238 / 0x243 box and 0), the seen marks zeroed; the list walked from its head
/// (`LLIST::get_next`), each competition id (the record's first dword) not seen yet kept and marked; the marks freed;
/// the ids sorted (`qsort` with `compare_int_ids`, which returns 1 for any two different ids: no real order); each one
/// appended (4 bytes) to a new list (none: the 0x26e box and 0). The table freed. Returns the list or 0.
/// Original: `Club_History::get_league_comp_ids` (0x443770, club_history.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn club_history_get_league_comp_ids(_this: usize, history: usize) -> usize {
    club_history_comp_ids(history, [0x238, 0x243, 0x26e])
}

/// Collects the distinct competition ids of a club's cup history list, "sorted", as a new list of ints.
///
/// `Club_History::get_cup_comp_ids(history)`: as `club_history_get_league_comp_ids`, with the boxes at lines 0x298,
/// 0x2a3 and 0x2ce.
/// Original: `Club_History::get_cup_comp_ids` (0x443a20, club_history.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn club_history_get_cup_comp_ids(_this: usize, history: usize) -> usize {
    club_history_comp_ids(history, [0x298, 0x2a3, 0x2ce])
}

/// Finds a club in the current table of its division and fills a league history record with its position and totals.
///
/// `club_history_get_current_league_record(club, rec, complete, stage, reserve)`: `*stage` = 0 when given; no club: the
/// club_history.cpp 0x36a box and 0. The division is the club's (`club::DIVISION`), or its reserve division
/// (`club::RESERVE_DIVISION`) for a Brazilian club (`club::NATION` = Brazil) with `reserve` set while today
/// (`G_CURRENT_DATE`, the day only) is after 1 February and before 20 June (the state championships). No division or
/// no competition object for it (`G_COMP_OBJECT_TABLE`): 0. From the league's current stage (the low byte of
/// `league::CURRENT_STAGE`; -1 the league itself, always so in the Argentine premier division) back to -1: a league
/// stage (`league::COMP_TYPE` = 2) with more rounds (`league::ROUND_COUNT`) than 7 (0 in the Polish first division)
/// gets `league::TABLE_COLUMN` = 0 and is sorted (`League::sort_table`, 0); the club's row there, in the US Major
/// League, on the league itself or after a game played (`league_table_row::PLAYED`), fills the record at once (the
/// division's id, the stage's year, the row's position and the 14 bytes of the totals from +0xd), `*complete` = the
/// league's virtual +0x10 (1, 1) (the season over), `*stage` = the stage, and returns 1; a row with no game yet keeps
/// the stage and the row and the search goes on. At the end, with a kept stage: the record from it (with the club's
/// `club::DIVISION` id), `*stage`, 1 (`*complete` not written); otherwise 0.
/// Original: `club_history_get_current_league_record` (0x444190, club_history.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_get_current_league_record(club: usize, rec: *mut u8, complete: *mut i32,
                                                                       stage_out: *mut usize, reserve: u32) -> u32 {
    use club_history_lists as ch;
    use league as l;
    if !stage_out.is_null() {
        *stage_out = 0;
    }
    if club == 0 {
        show_error_at(ch::CLUB_HISTORY_CPP, 0x36a);
        return 0;
    }
    let nation = read_usize(club + club::NATION);
    let mut division = read_usize(club + club::DIVISION);
    if nation != 0 && read_u32(nation) == read_u32(KEY_NATION_BRAZIL) && reserve != 0 {
        let (mut from, mut to) = ([0u16; 4], [0u16; 4]);
        let from_day = read_u16(exe::date_date_549f70(&mut from, 1, 1, 0x7d1, u32::MAX)) as i16;
        let today = read_u16(G_CURRENT_DATE) as i16;
        if today > from_day && today < read_u16(exe::date_date_549f70(&mut to, 0x14, 5, 0x7d1, u32::MAX)) as i16 {
            covered!("tel42_3 Brazilian state league");
            division = read_usize(club + club::RESERVE_DIVISION);
        }
    }
    if division == 0 {
        return 0;
    }
    let league = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(division) as usize * 4);
    if league == 0 {
        return 0;
    }
    let mut current = read_u8(league + l::CURRENT_STAGE) as i8;
    let mut min_rounds = 7i16;
    let competition = read_usize(league + l::COMPETITION);
    if competition != 0 {
        if read_u32(competition) == read_u32(KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION) {
            current = -1;
        }
        if read_u32(competition) == read_u32(KEY_CLUB_COMP_POLISH_FIRST_DIVISION) {
            min_rounds = 0;
        }
    }
    let fill = |stage: usize, k: usize, id: u32| {
        let row = read_usize(stage + l::TABLE) + k * league_table_row::SIZE;
        write_u32(rec as usize, id);
        write_u16(rec as usize + 4, read_u16(stage + l::YEAR));
        write_u8(rec as usize + 6, read_u8(row + league_table_row::POSITION)); // tel42_3: the position
        for i in 0..14 {
            write_u8(rec as usize + 7 + i, read_u8(row + 0xd + i));
        }
    };
    let (mut kept, mut kept_row) = (0usize, 0usize);
    while current >= -1 {
        let stage = if current == -1 { league } else { read_usize(read_usize(league + l::STAGES) + current as isize as usize * 4) };
        if read_u8(stage + l::COMP_TYPE) == 2 && read_i16(stage + l::ROUND_COUNT) > min_rounds {
            write_u32(stage + l::TABLE_COLUMN, 0);
            exe::league_sort_table(stage, 0usize);
            for k in 0..read_i16(stage + l::CLUB_COUNT).max(0) as usize {
                let row = read_usize(stage + l::TABLE) + k * league_table_row::SIZE;
                if read_usize(row + league_table_row::CLUB) != club {
                    continue;
                }
                if read_u32(competition) == read_u32(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) || current == -1
                    || read_u8(row + league_table_row::PLAYED) != 0
                {
                    covered!("tel42_3 current league row");
                    fill(stage, k, read_u32(division));
                    let over: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(league) + 0x10));
                    *complete = over(league, 1, 1) as i8 as i32;
                    if !stage_out.is_null() {
                        *stage_out = stage;
                    }
                    return 1;
                }
                kept = stage;
                kept_row = k;
                break;
            }
        }
        current -= 1;
    }
    if kept == 0 {
        return 0;
    }
    covered!("tel42_3 league row before a game");
    fill(kept, kept_row, read_u32(read_usize(club + club::DIVISION)));
    if !stage_out.is_null() {
        *stage_out = kept;
    }
    1
}

/// The qsort comparison of two competition ids that never orders them: 0 when equal, 1 otherwise (either way round).
///
/// `compare_int_ids(a, b)`: 1 when *a > *b, otherwise 1 when *a < *b and 0 when equal (signed).
/// Original: `compare_int_ids` (0x444500, club_history.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_int_ids(a: *const usize, b: *const usize) -> i32 {
    let (x, y) = (read_i32(a as usize), read_i32(b as usize));
    (x != y) as i32 // tel42_3: never an order
}

/// The contents of a list of records of `size` bytes, from the head (the cursor moved as `LLIST::get_next` does).
unsafe fn club_history_list_contents(list: usize, size: usize) -> Vec<Vec<u8>> {
    let mut v = Vec::new();
    if list == 0 {
        return v;
    }
    let cursor = read_usize(list + llist::CURSOR);
    write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
    loop {
        let d = exe::llist_get_next(list);
        if d == 0 {
            break;
        }
        v.push((0..size).map(|i| read_u8(d + i)).collect());
    }
    write_usize(list + llist::CURSOR, cursor);
    v
}

/// The side-by-side comparison of the routines that return a new list: the original's list is the game's; the copy's
/// is compared by content (`club_history_list_contents`) and destroyed. `regions` are the other memory compared.
unsafe fn club_history_compare_list(name: &str, size: usize, regions: &[(usize, usize)], original: impl FnOnce() -> usize,
                                    rust: impl FnOnce() -> usize) -> usize {
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(regions, original, rust);
    let (lo, lr) = (club_history_list_contents(ro, size), club_history_list_contents(rr, size));
    if rr != 0 {
        exe::llist_scalar_deleting_destructor(rr, 1);
    }
    SCOREBOARD.record(equal && lo == lr && (ro == 0) == (rr == 0), || format!("{} (club history screen): original {:02x?} rust {:02x?}, differences in {:?}",
        name, lo, lr, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `club_history_get_league_history`: side by side by the list contents (the sorts it asks for give the
/// same tables twice).
pub(crate) unsafe extern "thiscall" fn compare_club_history_get_league_history(this: usize, club: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CLUB_HISTORY_GET_LEAGUE_HISTORY);
    if club == 0 {
        return o(this, club);
    }
    club_history_compare_list("club_history_get_league_history", 0x15, &[], || o(this, club), || club_history_get_league_history(this, club))
}

/// Comparer for `club_history_get_cup_history`: side by side by the list contents.
pub(crate) unsafe extern "thiscall" fn compare_club_history_get_cup_history(this: usize, club: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CLUB_HISTORY_GET_CUP_HISTORY);
    if club == 0 {
        return o(this, club);
    }
    club_history_compare_list("club_history_get_cup_history", 0xb, &[], || o(this, club), || club_history_get_cup_history(this, club))
}

/// Comparer for `club_history_get_league_comp_ids`: side by side by the list contents and the history list's cursor.
pub(crate) unsafe extern "thiscall" fn compare_club_history_get_league_comp_ids(this: usize, history: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CLUB_HISTORY_GET_LEAGUE_COMP_IDS);
    if history == 0 {
        return o(this, history);
    }
    club_history_compare_list("club_history_get_league_comp_ids", 4, &[(history, 0xe)], || o(this, history), || club_history_get_league_comp_ids(this, history))
}

/// Comparer for `club_history_get_cup_comp_ids`: side by side by the list contents and the history list's cursor.
pub(crate) unsafe extern "thiscall" fn compare_club_history_get_cup_comp_ids(this: usize, history: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CLUB_HISTORY_GET_CUP_COMP_IDS);
    if history == 0 {
        return o(this, history);
    }
    club_history_compare_list("club_history_get_cup_comp_ids", 4, &[(history, 0xe)], || o(this, history), || club_history_get_cup_comp_ids(this, history))
}

/// Comparer for `club_history_get_current_league_record`: side by side over the record and the two answers given (the
/// sorts give the same tables twice). Without a club: only the original (the box).
pub(crate) unsafe extern "C" fn compare_club_history_get_current_league_record(club: usize, rec: *mut u8, complete: *mut i32,
                                                                                stage_out: *mut usize, reserve: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut u8, *mut i32, *mut usize, u32) -> u32>(IDX_CLUB_HISTORY_GET_CURRENT_LEAGUE_RECORD);
    if club == 0 {
        return o(club, rec, complete, stage_out, reserve);
    }
    let mut regions = vec![(rec as usize, 0x15)];
    if !complete.is_null() {
        regions.push((complete as usize, 4));
    }
    if !stage_out.is_null() {
        regions.push((stage_out as usize, 4));
    }
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(club, rec, complete, stage_out, reserve),
        || club_history_get_current_league_record(club, rec, complete, stage_out, reserve));
    SCOREBOARD.record(equal && ro == rr, || format!("club_history_get_current_league_record(club {:#x}, reserve {}) original={} rust={}, differences in {:?}",
        club, reserve, ro, rr, first_differences(&mo, &mr)));
    ro
}

compare_order!(compare_compare_int_ids, compare_int_ids, IDX_COMPARE_INT_IDS);

// ---------- batch 322: the written history files (.his): reading one into the history screen, existence, category, title, the -verifyhist check and the order by title (History.cpp) ----------

/// Constants of the .his readers (History.cpp).
pub(crate) mod his_file {
    /// The character pushed back by the readers (-1 = none), read again before the file.
    pub(crate) const PUSHBACK: usize = 0x9cebe0;
    /// The static title buffer `history_get_his_file_title` returns.
    pub(crate) const TITLE_TEXT: usize = 0xb5e008;
    /// The compared size of the two static buffers (the name / title buffer and the title buffer).
    pub(crate) const TEXT_SIZE: usize = 0x12c;
    /// The five category names ("CLUB", "NATION", "COMP", "LEAGUE", "PLAYER"; pointers from 0x9cebc8).
    pub(crate) const CATEGORY_NAMES: usize = 0x9cebc8;
    pub(crate) const CATEGORY_COUNT: usize = 5;
    /// "cm3_history", "CM3_HISTORY", "%s.his", "%s", "r", "*" and ".his".
    pub(crate) const FOLDER: usize = 0x9cec18;
    pub(crate) const FOLDER_KEY: usize = 0x9cedb8;
    pub(crate) const NAME_FORMAT: usize = 0x9cec50;
    pub(crate) const TOKEN_FORMAT: usize = 0x991944;
    pub(crate) const READ_MODE: usize = 0x9899fc;
    pub(crate) const ANY_NAME: usize = 0x989ad8;
    pub(crate) const EXTENSION: usize = 0x9cedb0;
    /// The tags: "title:", "section:", "pic:", "cat:", "category:".
    pub(crate) const TITLE_TAG: usize = 0x9cec6c;
    pub(crate) const SECTION_TAG: usize = 0x9cec60;
    pub(crate) const PIC_TAG: usize = 0x9cec58;
    pub(crate) const CAT_TAG: usize = 0x9cec8c;
    pub(crate) const CATEGORY_TAG: usize = 0x9cec80;
    /// The history screen's default background ("wcup.hsr", parameter 7).
    pub(crate) const DEFAULT_PICTURE: usize = 0x9cec74;
    /// A section's paragraph: +0 the text (allocated), +4 the picture index (-1 none).
    pub(crate) const PARAGRAPH_SIZE: usize = 8;
    /// `fscanf` (C runtime, cdecl).
    pub(crate) const CRT_FSCANF: usize = 0x9470b8;
}

/// The next character of a .his file: the pushed-back one (`his_file::PUSHBACK`, then -1) or `fgetc`.
unsafe fn his_file_getc(file: usize) -> i32 {
    let c = read_i32(his_file::PUSHBACK);
    if c != -1 {
        write_i32(his_file::PUSHBACK, -1);
        return c;
    }
    exe::at_9469e0(file)
}

/// The end-of-file flag of a C runtime FILE (`_flag` +0xc, `_IOEOF` 0x10).
unsafe fn his_file_at_end(file: usize) -> bool {
    read_u8(file + 0xc) & 0x10 != 0
}

/// Opens "cm3_history\<name>.his" for reading ("%s.his" written into `HISTORY_FILE_TEXT`); 0 when missing.
unsafe fn his_file_open(name: usize) -> usize {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    format(HISTORY_FILE_TEXT, his_file::NAME_FORMAT, name);
    exe::crt_fopen(exe::utils_build_file_path(his_file::FOLDER, HISTORY_FILE_TEXT), his_file::READ_MODE)
}

/// The value of a "{tag value}" after the tag: the spaces skipped, then up to '}' or the end, into `out` (closed).
unsafe fn his_file_tag_value(file: usize, out: usize) {
    let mut c = his_file_getc(file);
    while c == b' ' as i32 {
        c = his_file_getc(file);
    }
    let mut i = 0usize;
    write_u8(out, 0);
    while c != b'}' as i32 && c != -1 {
        write_u8(out + i, c as u8);
        i += 1;
        c = his_file_getc(file);
    }
    write_u8(out + i, 0);
}

/// Parses a .his history file into the history screen's title, sections and pictures.
///
/// `history_load_his_file(name, last)`: parameter 7 empty gets "wcup.hsr". The file (parameter 0x13's folder and the
/// name, `utils_build_file_path`, "r") missing: the History.cpp 0x1fe box and 0. Until the end of the file, each
/// character read (the pushed-back one first, `his_file::PUSHBACK`):
/// - '{' and a word (`fscanf` "%s"; tags compared without case, `stricmp`): "title:" the value (spaces skipped, up to
///   '}') into `HISTORY_FILE_TEXT`, parameter 1 a copy of it; "section:" a new section (the table grows by
///   `HISTORY_RECORD_SIZE`, `realloc`; none: the 0x22f box and 0) with no paragraph, its name at +8 up to '}', the
///   current picture back to -1; "pic:" the value without spaces into a new picture name (`HISTORY_PICTURE_NAME_SIZE`
///   each, `realloc`; none: 0x269), which becomes the current picture; any other word: skipped up to '}';
/// - any other character inside a section (before the first one it is lost): that character is lost, then the
///   paragraphs: blanks (space, tab, CR, LF) skipped, '{' or the end pushed back to end the text; otherwise a new
///   paragraph (+0 grows by 8, `realloc`; none: 0x29b) with the current picture, whose text grows by one byte per
///   character (`realloc`; none: 0x2af): a single line break becomes one space (the next line's leading blanks
///   skipped, its first character pushed back), two line breaks or the end close the paragraph (the 0 stored).
/// The file closed; parameters 2 = the sections, 3 = their count, 4 = the pictures, 5 = their count, 6 = the last
/// section when `last` is set, else 0. Returns whether parameter 1 (the title) is set. The error returns leave the file
/// open.
/// Original: `history_load_his_file` (0x5f1e50, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_load_his_file(name: usize, last: u32) -> i32 {
    let fscanf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(his_file::CRT_FSCANF);
    let (mut section, mut sections, mut pictures, mut picture_count, mut picture) = (-1i32, 0usize, 0usize, 0u32, -1i32);
    if get_screen_param(7) == 0 {
        set_screen_param(7, his_file::DEFAULT_PICTURE);
    }
    let file = exe::crt_fopen(exe::utils_build_file_path(get_screen_param(0x13), name), his_file::READ_MODE);
    if file == 0 {
        show_error_at(HISTORY_CPP, 0x1fe);
        return 0;
    }
    covered!("tel42_4 read a .his file");
    // the original's word buffer is 32 bytes in a 0x238-byte frame: a longer word runs into the error box buffers
    let mut token = [0u8; 0x220];
    let tok = token.as_mut_ptr() as usize;
    while !his_file_at_end(file) {
        let mut c = his_file_getc(file);
        if c == -1 {
            continue;
        }
        if c == b'{' as i32 {
            fscanf(file, his_file::TOKEN_FORMAT, tok);
            if exe::crt_stricmp(tok, his_file::TITLE_TAG) == 0 {
                his_file_tag_value(file, HISTORY_FILE_TEXT);
                exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 1, HISTORY_FILE_TEXT, 1);
            } else if exe::crt_stricmp(tok, his_file::SECTION_TAG) == 0 {
                section += 1;
                picture = -1;
                sections = exe::crt_realloc(sections, (section as u32 + 1).wrapping_mul(HISTORY_RECORD_SIZE as u32));
                if sections == 0 {
                    show_error_at(HISTORY_CPP, 0x22f);
                    return 0;
                }
                let s = sections + section as usize * HISTORY_RECORD_SIZE;
                write_usize(s, 0);
                write_u32(s + 4, 0);
                // the name has no bound (0x64 bytes before the next section)
                let mut i = 0usize;
                while c != -1 {
                    c = his_file_getc(file);
                    if c != b'}' as i32 && c != -1 {
                        write_u8(s + 8 + i, c as u8);
                        i += 1;
                    } else {
                        write_u8(s + 8 + i, 0);
                    }
                    if c == b'}' as i32 {
                        break;
                    }
                }
            } else if exe::crt_stricmp(tok, his_file::PIC_TAG) == 0 {
                let mut i = 0usize;
                while c != -1 {
                    c = his_file_getc(file);
                    if c == b'}' as i32 || c == -1 {
                        token[i] = 0;
                    } else if c != b' ' as i32 {
                        token[i] = c as u8;
                        i += 1;
                    }
                    if c == b'}' as i32 {
                        break;
                    }
                }
                pictures = exe::crt_realloc(pictures, (picture_count + 1).wrapping_mul(HISTORY_PICTURE_NAME_SIZE as u32));
                if pictures == 0 {
                    show_error_at(HISTORY_CPP, 0x269);
                    return 0;
                }
                tel26_copy_text((pictures + picture_count as usize * HISTORY_PICTURE_NAME_SIZE) as *mut u8, token.as_ptr());
                picture = picture_count as i32; // tel42_4: the current picture
                picture_count += 1;
            } else {
                while c != -1 {
                    c = his_file_getc(file);
                    if c == b'}' as i32 {
                        break;
                    }
                }
            }
        } else if section != -1 {
            // the character that brought us here is lost; then the paragraphs up to '{' or the end
            loop {
                c = his_file_getc(file);
                while matches!(c, 0x20 | 0x9 | 0xa | 0xd) {
                    c = his_file_getc(file);
                }
                if c == b'{' as i32 || c == -1 {
                    write_i32(his_file::PUSHBACK, c);
                    break;
                }
                covered!("tel42_4 .his paragraph");
                let s = sections + section as usize * HISTORY_RECORD_SIZE;
                let n = read_u32(s + 4) as usize;
                let paragraphs = exe::crt_realloc(read_usize(s), (n as u32).wrapping_mul(8).wrapping_add(8));
                if paragraphs == 0 {
                    show_error_at(HISTORY_CPP, 0x29b);
                    return 0;
                }
                write_usize(s, paragraphs);
                let p = paragraphs + n * his_file::PARAGRAPH_SIZE;
                write_usize(p, 0);
                write_i32(p + 4, picture);
                let mut len = 0usize;
                loop {
                    let text = exe::crt_realloc(read_usize(read_usize(s) + n * his_file::PARAGRAPH_SIZE), len as u32 + 1);
                    if text == 0 {
                        show_error_at(HISTORY_CPP, 0x2af);
                        return 0;
                    }
                    write_usize(read_usize(s) + n * his_file::PARAGRAPH_SIZE, text);
                    write_u8(text + len, c as u8);
                    len += 1;
                    if c == 0 {
                        break;
                    }
                    let mut line_break = false;
                    loop {
                        let d = his_file_getc(file);
                        c = if d == -1 {
                            0
                        } else if d == b'\n' as i32 {
                            if !line_break {
                                line_break = true;
                                continue;
                            }
                            0 // tel42_4: two line breaks end the paragraph
                        } else {
                            d
                        };
                        if line_break && matches!(c, 0x20 | 0x9 | 0xd) {
                            continue;
                        }
                        break;
                    }
                    if line_break && c != 0 {
                        write_i32(his_file::PUSHBACK, c);
                        c = b' ' as i32;
                    }
                }
                write_u32(s + 4, n as u32 + 1);
            }
        }
    }
    exe::crt_fclose(file);
    let count = section + 1;
    set_screen_param(2, sections);
    set_screen_param(3, count as usize);
    set_screen_param(4, pictures);
    set_screen_param(5, picture_count as usize);
    set_screen_param(6, if last != 0 { (count - 1) as usize } else { 0 });
    (get_screen_param(1) != 0) as i32
}

/// Whether a named .his file exists in cm3_history, for a single-player game on a local screen only.
///
/// `history_his_file_exists(name)`: the game mode (`G_CONFIG` +8) not 0 or 1, or the current screen slot going over
/// the network (`screen_manager::NETWORK_SLOT`): 0. Otherwise "cm3_history\<name>.his" opened for reading (the name
/// written into `HISTORY_FILE_TEXT`): open, it is closed and 1; missing, 0.
/// Original: `history_his_file_exists` (0x5f2500, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_his_file_exists(name: usize) -> u32 {
    if !matches!(read_u32(read_usize(G_CONFIG) + 8), 0 | 1) {
        return 0;
    }
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize;
    if read_u16(G_SCREEN_MANAGER + screen_manager::NETWORK_SLOT + slot.wrapping_mul(0x300)) != 0 {
        return 0;
    }
    let file = his_file_open(name);
    if file == 0 {
        return 0;
    }
    covered!("tel42_4 .his file exists");
    exe::crt_fclose(file);
    1 // tel42_4: the file is there
}

/// Reads the category of a .his file and returns its index in the category names, or -1.
///
/// `history_get_his_file_category(name)`: "cm3_history\<name>.his" (`his_file_open`) missing: -1. `HISTORY_FILE_TEXT`
/// emptied; until the end of the file, a '{' followed by the word "cat:" or "category:" (without case) gives the value
/// (spaces skipped, up to '}') into `HISTORY_FILE_TEXT` and stops; any other character goes on. The file closed. The
/// text compared without case to "CLUB", "NATION", "COMP", "LEAGUE", "PLAYER": the index of the first equal, else -1
/// (a file with no category compares the empty text).
/// Original: `history_get_his_file_category` (0x5f2580, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_get_his_file_category(name: usize) -> i32 {
    let fscanf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(his_file::CRT_FSCANF);
    let file = his_file_open(name);
    if file == 0 {
        return -1;
    }
    write_u8(HISTORY_FILE_TEXT, 0);
    // the original's word buffer is 32 bytes at the bottom of its frame: a longer word would run over its stack
    let mut word = [0u8; 0x200];
    while !his_file_at_end(file) {
        let c = his_file_getc(file);
        if c == b'{' as i32 {
            fscanf(file, his_file::TOKEN_FORMAT, word.as_mut_ptr());
            if exe::crt_stricmp(word.as_ptr() as usize, his_file::CAT_TAG) == 0
                || exe::crt_stricmp(word.as_ptr() as usize, his_file::CATEGORY_TAG) == 0
            {
                covered!("tel42_4 .his category");
                his_file_tag_value(file, HISTORY_FILE_TEXT);
                break;
            }
        }
    }
    exe::crt_fclose(file);
    (0..his_file::CATEGORY_COUNT)
        .find(|&k| exe::crt_stricmp(read_usize(his_file::CATEGORY_NAMES + k * 4), HISTORY_FILE_TEXT) == 0) // tel42_4: the category index
        .map_or(-1, |k| k as i32)
}

/// Reads the title of a named .his file into a static buffer and returns it, or 0 when the file has none.
///
/// `history_get_his_file_title(name)`: `his_file::TITLE_TEXT` emptied; "cm3_history\<name>.his" missing: 0. Until the
/// end of the file, a '{' followed by the word "title:" (without case) gives the value (spaces skipped, up to '}') into
/// `his_file::TITLE_TEXT`; the file closed and the buffer returned. No title: the file closed and 0.
/// Original: `history_get_his_file_title` (0x5f2700, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_get_his_file_title(name: usize) -> usize {
    let fscanf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(his_file::CRT_FSCANF);
    write_u8(his_file::TITLE_TEXT, 0);
    let file = his_file_open(name);
    if file == 0 {
        return 0;
    }
    // the original's word buffer is 32 bytes at the bottom of its frame: a longer word would run over its stack
    let mut word = [0u8; 0x200];
    while !his_file_at_end(file) {
        let c = his_file_getc(file);
        if c == b'{' as i32 {
            fscanf(file, his_file::TOKEN_FORMAT, word.as_mut_ptr());
            if exe::crt_stricmp(word.as_ptr() as usize, his_file::TITLE_TAG) == 0 {
                covered!("tel42_4 .his title");
                his_file_tag_value(file, his_file::TITLE_TEXT);
                exe::crt_fclose(file);
                return his_file::TITLE_TEXT; // tel42_4: the title
            }
        }
    }
    exe::crt_fclose(file);
    0
}

/// The verification of one .his file name (without its extension) against the database, by category: the message
/// format of the failure, or 0 when it matches.
unsafe fn verify_history_file(name: *const u8) -> usize {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    match history_get_his_file_category(name as usize) {
        0 | 1 => {
            // a NATION file is checked against the club names too
            for k in 0..read_i32(G_CLUB_COUNT).max(0) as usize {
                let club = read_usize(G_CLUB_TABLE) + k * 0x245;
                if exe::utils_string_compare(name, exe::club_get_english_name(club) as *const u8, 1, 0) == 0 {
                    return 0;
                }
            }
            0x9ced58 // "verify_history_files() : file %s has no matching nation or club"
        }
        2 | 3 => {
            for k in 0..read_i32(G_CLUB_COMP_COUNT).max(0) as usize {
                let comp = read_usize(G_CLUB_COMP_TABLE) + k * 0x6b;
                if exe::utils_string_compare(name, comp_get_english_name(comp) as *const u8, 1, 0) == 0 {
                    return 0;
                }
            }
            0x9ced18 // "verify_history_files() : file %s has no matching competition"
        }
        4 => {
            for k in 0..read_i32(G_STAFF_COUNT).max(0) as usize {
                let person = read_usize(G_STAFF_TABLE) + k * 0x6e;
                let second = match read_usize(person + 8) { 0 => his_file::ANY_NAME, p => p };
                let first = match read_usize(person + 4) { 0 => his_file::ANY_NAME, p => p };
                let birth = person + 0x10;
                let year = read_u16(birth + 2) as u32;
                let month = exe::date_get_month(birth) as i8 as i32 + 1;
                let day = exe::date_get_day_of_month(birth) as i16 as i32;
                // "%s %s %d.%d.%d": the two names and the date of birth
                format(JOB_TEXT, 0x9ced08, first, second, day, month, year);
                if exe::utils_string_compare(name, JOB_TEXT as *const u8, 1, 0) == 0 {
                    return 0;
                }
            }
            0x9cecd0 // "verify_history_files() : file %s has no matching staff"
        }
        _ => 0x9cec94, // "verify_history_files() : file %s has an invalid category"
    }
}

/// Checks every .his file in CM3_HISTORY against the database and reports each one with no match (the -verifyhist
/// option of the command line).
///
/// `verify_history_files()`: the folder (`utils_build_file_path` "CM3_HISTORY"), the pattern "*.his" (`makepath`). A
/// first pass only counts the files (`file_find_first`/`file_find_next`); the second, for each file: the network
/// status message "Verifying Histories..." with the file's number and the count (`send_network_status_message`, -1);
/// the name without its last extension; by its category (`history_get_his_file_category`): CLUB and NATION against
/// the clubs' English names (`club_get_english_name`), COMP and LEAGUE against the competitions'
/// (`comp_get_english_name`), PLAYER against "<first> <second> <day>.<month>.<year>" of each person (a missing name is
/// "*"; the date of birth +0x10), compared by `utils_string_compare` (1, 0); no match, or another category: the
/// message (with the full file name) in an "Error" box. Each search closed (`file_find_close`).
/// Original: `verify_history_files` (0x5f2840, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn verify_history_files() {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut folder = [0u8; 0x104];
    tel26_copy_text(folder.as_mut_ptr(), exe::utils_build_file_path(his_file::FOLDER_KEY, HISTORY_EMPTY) as *const u8);
    let mut total = 0u32;
    for pass in 1..=2 {
        let mut pattern = [0u8; 0x104];
        let mut found = [0u8; 0x114];
        exe::crt_makepath(pattern.as_mut_ptr(), 0usize, folder.as_ptr() as usize, his_file::ANY_NAME, his_file::EXTENSION);
        let mut count = 0u32;
        if exe::file_find_first(pattern.as_ptr() as usize, found.as_mut_ptr() as usize, 0) == 0 {
            let mut number = 1i32;
            loop {
                if pass == 2 {
                    covered!("tel42_4 verify a .his file");
                    text(JOB_TEXT, 0x9ced98); // "Verifying Histories..."
                    exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, JOB_TEXT, number, total, -1);
                    let mut name = [0u8; 0x104];
                    tel26_copy_text(name.as_mut_ptr(), found.as_ptr().add(0x10));
                    if let Some(dot) = name[..tel35_c_string_len(name.as_ptr())].iter().rposition(|&b| b == b'.') {
                        name[dot] = 0;
                    }
                    let message = verify_history_file(name.as_ptr());
                    if message != 0 { // tel42_4: a file with no match
                        format(JOB_TEXT, message, found.as_ptr().add(0x10));
                        exe::gui_show_message_box(0x9870e0, JOB_TEXT, 0);
                    }
                }
                count += 1;
                number += 1;
                if exe::file_find_next(found.as_mut_ptr()) != 0 {
                    break;
                }
            }
        }
        exe::file_find_close(found.as_ptr() as usize);
        total = count;
    }
}

/// Compares two history file entries by their titles for qsort, a missing entry last.
///
/// `history_compare_entries_by_title(a, b)`: no a: 1; no b: -1; otherwise `utils_string_compare` of the titles (+0x104,
/// 1, 0).
/// Original: `history_compare_entries_by_title` (0x5f3410, History.cpp, cdecl).
pub(crate) unsafe extern "C" fn history_compare_entries_by_title(a: *const usize, b: *const usize) -> i32 {
    if a.is_null() {
        return 1;
    }
    if b.is_null() {
        return -1;
    }
    exe::utils_string_compare((a as usize + 0x104) as *const u8, (b as usize + 0x104) as *const u8, 1, 0) // tel42_4: by title
}

/// A deep copy of what `history_load_his_file` leaves in the screen parameters: the return, the title text, each
/// section's name and paragraphs (text and picture) and the picture names.
unsafe fn his_file_digest(ret: i32) -> Vec<u8> {
    let mut v = ret.to_le_bytes().to_vec();
    let text = |v: &mut Vec<u8>, p: usize| {
        if p != 0 {
            let n = tel35_c_string_len(p as *const u8);
            v.extend((0..n).map(|i| read_u8(p + i)));
        }
        v.push(0);
    };
    text(&mut v, get_screen_param(1));
    let (sections, count) = (get_screen_param(2), get_screen_param(3) as i32);
    v.extend(count.to_le_bytes());
    for k in 0..count.max(0) as usize {
        let s = sections + k * HISTORY_RECORD_SIZE;
        text(&mut v, s + 8);
        let n = read_u32(s + 4) as usize;
        v.extend((n as u32).to_le_bytes());
        for j in 0..n {
            let p = read_usize(s) + j * his_file::PARAGRAPH_SIZE;
            text(&mut v, read_usize(p));
            v.extend(read_u32(p + 4).to_le_bytes());
        }
    }
    let (pictures, pc) = (get_screen_param(4), get_screen_param(5));
    v.extend((pc as u32).to_le_bytes());
    for k in 0..pc {
        text(&mut v, pictures + k * HISTORY_PICTURE_NAME_SIZE);
    }
    v.extend((get_screen_param(6) as u32).to_le_bytes());
    v
}

/// Comparer for `history_load_his_file`: the copy reads first, its result is kept as a deep copy and freed, then the
/// original reads from the same state and both are compared by content.
///
/// The routine allocates the sections, paragraphs and pictures it leaves in the screen parameters (and parameter 1
/// takes a copy of the title): the copy's blocks are freed after the digest (`his_file_digest`) and the parameter
/// block, the two static buffers and the pushed-back character are put back before the original runs. A file that
/// does not open: only the original (the box).
pub(crate) unsafe extern "C" fn compare_history_load_his_file(name: usize, last: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> i32>(IDX_HISTORY_LOAD_HIS_FILE);
    // a value owned by one of the parameters it sets (1..7: the slot's flag at +0x18 + 8n) would be freed by the copy's
    // set and again by the original's; so would a title already in parameter 1: only the original then (the history
    // screen calls it with parameter 1 empty)
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize;
    let block = read_usize(G_SCREEN_MANAGER + screen_manager::PARAM_BLOCK + slot.wrapping_mul(0x300));
    if block == 0 || get_screen_param(1) != 0 || (1..=7).any(|n| read_u32(block + 0x18 + 8 * n) != 0) {
        return o(name, last);
    }
    let probe = exe::crt_fopen(exe::utils_build_file_path(get_screen_param(0x13), name), his_file::READ_MODE);
    if probe == 0 {
        return o(name, last);
    }
    exe::crt_fclose(probe);
    let mut regions = screen_param_regions();
    regions.push((HISTORY_FILE_TEXT, his_file::TEXT_SIZE));
    regions.push((his_file::PUSHBACK, 4));
    let before = crate::comparison::MemorySnapshot::read(&regions);
    let rr = history_load_his_file(name, last);
    let mine = his_file_digest(rr);
    // free what the copy allocated: the title copy (when it is a new one), the texts, the paragraphs, the tables
    let title = get_screen_param(1);
    if title != 0 {
        exe::crt_operator_delete(title);
    }
    let (sections, count) = (get_screen_param(2), get_screen_param(3) as i32);
    for k in 0..count.max(0) as usize {
        let s = sections + k * HISTORY_RECORD_SIZE;
        for j in 0..read_u32(s + 4) as usize {
            exe::crt_free(read_usize(read_usize(s) + j * his_file::PARAGRAPH_SIZE));
        }
        exe::crt_free(read_usize(s));
    }
    exe::crt_free(sections);
    exe::crt_free(get_screen_param(4));
    before.write_back();
    let ro = o(name, last);
    let theirs = his_file_digest(ro);
    SCOREBOARD.record(mine == theirs, || format!("history_load_his_file (history screen): the original read {} bytes of content, the copy {}; first difference at {:?}",
        theirs.len(), mine.len(), theirs.iter().zip(mine.iter()).position(|(a, b)| a != b)));
    ro
}

/// Comparer for `history_his_file_exists`: side by side over the name buffer and the pushed-back character.
pub(crate) unsafe extern "C" fn compare_history_his_file_exists(name: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_HISTORY_HIS_FILE_EXISTS);
    let regions = [(HISTORY_FILE_TEXT, his_file::TEXT_SIZE), (his_file::PUSHBACK, 4)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(name), || history_his_file_exists(name));
    SCOREBOARD.record(equal && ro == rr, || format!("history_his_file_exists (history menus) original={} rust={}, differences in {:?}", ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `history_get_his_file_category`: side by side over the name buffer and the pushed-back character.
pub(crate) unsafe extern "C" fn compare_history_get_his_file_category(name: usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> i32>(IDX_HISTORY_GET_HIS_FILE_CATEGORY);
    let regions = [(HISTORY_FILE_TEXT, his_file::TEXT_SIZE), (his_file::PUSHBACK, 4)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(name), || history_get_his_file_category(name));
    SCOREBOARD.record(equal && ro == rr, || format!("history_get_his_file_category (written history screen) original={} rust={}, differences in {:?}", ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `history_get_his_file_title`: side by side over the two static buffers and the pushed-back character.
pub(crate) unsafe extern "C" fn compare_history_get_his_file_title(name: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_HISTORY_GET_HIS_FILE_TITLE);
    let regions = [(HISTORY_FILE_TEXT, his_file::TEXT_SIZE), (his_file::TITLE_TEXT, his_file::TEXT_SIZE), (his_file::PUSHBACK, 4)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(name), || history_get_his_file_title(name));
    SCOREBOARD.record(equal && ro == rr, || format!("history_get_his_file_title (written history screen) original={:#x} rust={:#x}, differences in {:?}", ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `verify_history_files`.
///
/// It sends network status messages and shows a box per file with no match: nothing that shows runs twice. Only the
/// original runs; the copy is proven in Rust mode by starting a new game with -verifyhist (the same boxes as the
/// original's).
pub(crate) unsafe extern "C" fn compare_verify_history_files() {
    original_fn::<unsafe extern "C" fn()>(IDX_VERIFY_HISTORY_FILES)()
}

compare_order!(compare_history_compare_entries_by_title, history_compare_entries_by_title, IDX_HISTORY_COMPARE_ENTRIES_BY_TITLE);
