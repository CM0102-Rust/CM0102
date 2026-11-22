// ---------- batch %%LEVA%%: the Setup screens, part 4: take control, the password and resume control boxes (Setup.cpp) ----------

// (uses the Setup helpers of tel11_1, two FN_ names of tel11_2 and setup_texts_differ/setup_confirm_box of tel11_3)
const FN_HUMAN_MANAGER_SET_COMPUTER_CONTROLLED: usize = 0x5fe140; // thiscall(HM, value, staff) ret 8
const FN_SCREENMANAGER_ADD_MANAGER_RECORD: usize = 0x7ff8f0; // thiscall(sm, on_join, on_leave, key) ret 0xc
const FN_MATCH_DAY_IS_CLUB_PLAYING: usize = 0x6b59a0; // thiscall(MATCH_DAY, club, watched_only) ret 8
const FN_FOG_OF_WAR_ADD_MANAGER_MAP: usize = 0x5afd70; // thiscall(FOG_OF_WAR, staff) ret 4
const G_SCREEN_MANAGER_PLAYER_JOINED_CALLBACK: usize = 0xdd6bf0; // g_screen_manager_player_joined_callback
const G_SCREEN_MANAGER_PLAYER_LEFT_CALLBACK: usize = 0xdd6bf4; // g_screen_manager_player_left_callback
const SETUP_MASTER_PASSWORD: usize = 0xa8099c; // "sportsinteractive": accepted for any manager's password
const CLUB_HUMAN_MANAGER: usize = 0xcf; // the club's human manager (a staff record) [inference: named in the box]

/// The bottom area of the Setup confirmation boxes (0x64, 0x22b to 0x316, 0x24e; columns 3, 1) with two buttons:
/// `left` (id `left_id`) and `right` (id `right_id`), translated.
unsafe fn setup_box_buttons(left: usize, left_id: usize, right: usize, right_id: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let button: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let mut columns: [u8; 2] = [3, 1];
    let (grey, pale) = (setup_colour(G_GUI_COLOUR_GREY), setup_colour(G_GUI_COLOUR_PALE_GREY));
    let a = area(current_human_display(), 0x64, 0x22b, 0x316, 0x24e, 2, columns.as_mut_ptr() as usize, 1, 0, 1, 0, usize::MAX);
    text(SETUP_TEXT, left);
    button(SCREEN_MANAGER, a, 0, 0, 0x30, grey, 0xc, 3, pale, SETUP_TEXT, left_id, 0);
    text(SETUP_TEXT, right);
    button(SCREEN_MANAGER, a, 1, 0, 0x30, grey, 0xc, 3, pale, SETUP_TEXT, right_id, 0);
}

/// The title bar of a Setup box: the manager's name (`exe::database_format_staff_name` into the shared text) in red
/// with pale grey text, the menu bar (mode 4) and the subtitle `subtitle`.
unsafe fn setup_box_header(manager: usize, subtitle: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    exe::database_format_staff_name(manager, SETUP_TEXT, 0x7d0, 1);
    exe::screenmanager_add_screen_title(SCREEN_MANAGER, setup_colour(G_GUI_COLOUR_RED) as u32, setup_colour(G_GUI_COLOUR_PALE_GREY) as u32, SETUP_TEXT);
    exe::menubar_build(4, 0);
    text(SETUP_TEXT, subtitle);
    exe::screenmanager_add_screen_subtitle(SCREEN_MANAGER, SETUP_TEXT, 0);
}

/// The question of a Setup box (already in the shared text) as a text area (0x6e, 0x91 to 0x30c, 0x217, font 4, pale
/// grey).
unsafe fn setup_box_question() {
    let text_area: unsafe extern "C" fn(i32, i32, i32, i32, u32, u32, u32, u32, usize, usize, ...) -> u32 = core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    text_area(0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, setup_colour(G_GUI_COLOUR_PALE_GREY), SETUP_TEXT, 0);
}

/// Opens the take control box for `club` (`take_control_confirm_build`, `take_control_confirm_handle_input`) with the
/// club in param 0.
/// Original: `open_take_control_confirm` (0x822a60, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_take_control_confirm(club: usize) {
    covered!("tel11_4 open take control");
    if exe::screenmanager_open_screen(SCREEN_MANAGER, 0x822aa0usize, 0x822c50usize, 0, 0usize, 0) != 0 {
        exe::screenmanager_set_screen_param(SCREEN_MANAGER, 0, club, 0);
    }
}

/// Builds the take control box: the club's picture (`exe::background_set_club_picture`), the current manager's name as
/// title, "Please Confirm", the question "Take control of {}<club>{} ?" (`club::NAME` with its `club::GENDER_NAME`) and
/// No (0x20) / Yes (0x21).
/// Original: `take_control_confirm_build` (0x822aa0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn take_control_confirm_build() {
    covered!("tel11_4 build take control");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let club = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0);
    let manager = read_usize(G_SCREEN_RECORDS_MANAGER + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300);
    exe::background_set_club_picture(club, 0, 0);
    setup_box_header(manager, 0x9909b8); // "Please Confirm"
    let g = read_i8(club + club::GENDER_NAME) as i32;
    text(SETUP_TEXT, 0xa8086c, g, g, club + club::NAME); // "Take control of {}<%s - club name>{} ?"
    setup_box_question();
    setup_box_buttons(0x98e8cc, 0x20, 0x98e8d0, 0x21); // "No", "Yes"
}

/// The take control box's answer to item `item`.
///
/// After its own "Already Controlled" box (`G_PENDING_COMMAND_CODE` 0x3b): cleared, −2. No (0x20): −2. Yes (0x21): a
/// club a human already runs (`exe::human_manager_is_human_club`) gives the "Already Controlled" box with his name
/// (code 0x3b); a club playing now (`MATCH_DAY::is_club_playing`) the "Playing In Match" box; both −4, red with pale grey
/// text. Otherwise the current manager becomes human (`staff::CLASSIFICATION` 1), is appointed
/// (`exe::appoint_human_manager_to_club`), gets his attribute map when Attribute Masking is on
/// (`FOG_OF_WAR::add_manager_map`), `G_PENDING_COMMAND_ARG` = 0, −10. Any other goes to the menu bar.
/// Original: `take_control_confirm_handle_input` (0x822c50, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn take_control_confirm_handle_input(item: u32) -> i32 {
    covered!("tel11_4 take control input");
    let text: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let playing: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_MATCH_DAY_IS_CLUB_PLAYING);
    let fog: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FOG_OF_WAR_ADD_MANAGER_MAP);
    if read_u32(G_PENDING_COMMAND_DISPATCH_FLAG) != 0 && read_u32(G_PENDING_COMMAND_CODE) == 0x3b {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        return -2;
    }
    match setup_clicked_id(item as i16) {
        0x20 => -2,
        0x21 => {
            let manager = read_usize(G_SCREEN_RECORDS_MANAGER + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300);
            let club = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0);
            let g = read_i8(club + club::SHORT_GENDER_NAME) as i32;
            if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
                let mut name = [0u8; 0xc8];
                exe::database_format_staff_name(read_usize(club + CLUB_HUMAN_MANAGER), name.as_mut_ptr() as usize, 0xc8, 1);
                let (red, pale) = (setup_colour(G_GUI_COLOUR_RED), setup_colour(G_GUI_COLOUR_PALE_GREY));
                setup_format_title();
                text(SETUP_TEXT, 0xa80914, g, g, name.as_ptr(), club + club::SHORT_NAME); // "<%s - manager> has already taken control of {}<%s - club>{}."
                setup_confirm_box(0xa80900, pale, red, 0x3b); // "Already Controlled"
                return -4;
            }
            let match_day = read_usize(G_MATCH_DAY);
            if match_day != 0 && playing(match_day, club, 0) != 0 {
                let (red, pale) = (setup_colour(G_GUI_COLOUR_RED), setup_colour(G_GUI_COLOUR_PALE_GREY));
                setup_format_title();
                text(SETUP_TEXT, 0xa808a8, g, g, club + club::SHORT_NAME); // "{}<%s - club>{} are currently playing in a match. ..."
                setup_confirm_box(0xa80894, pale, red, 0); // "Playing In Match"
                return -4;
            }
            write_u8(manager + staff::CLASSIFICATION, 1); // tel11_4 the manager becomes human
            exe::appoint_human_manager_to_club(manager, club);
            if read_u8(G_ATTRIBUTE_MASKING_ENABLED) != 0 {
                fog(read_usize(G_FOG_OF_WAR), manager);
            }
            write_u32(G_PENDING_COMMAND_ARG, 0);
            -10
        }
        _ => {
            let r = exe::menubar_handle_command(item);
            if r != 0 { r } else { exe::menubar_handle_object_link(item) }
        }
    }
}

/// Builds the Enter Password box of the manager in param 0: a random background, his name as title, "Enter Password",
/// the password entry (`Display::add_item` type 0x18 at 0x96, 0x136 to 0x2e4, 0x172, style 0x22, dark blue, flags 0x8c,
/// font 4, his `get_password_entry`, 0x32 characters, id 0x22), Cancel (0x23) and Ok (0x24, a button only when something
/// is typed, otherwise text).
/// Original: `enter_password_box_build` (0x822ec0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn enter_password_box_build() {
    covered!("tel11_4 build enter password");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let typed: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_PASSWORD_ENTRY);
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let button: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let label: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let sm = SCREEN_MANAGER;
    let mut columns: [u8; 2] = [3, 1];
    let manager = exe::screenmanager_get_screen_param(sm, 0);
    exe::background_set_random_picture();
    setup_box_header(manager, 0xa80954); // "Enter Password"
    let (grey, pale) = (setup_colour(G_GUI_COLOUR_GREY), setup_colour(G_GUI_COLOUR_PALE_GREY));
    let entry = typed(read_usize(G_HUMAN_MANAGER), manager);
    item(current_human_display(), 0x18, 0x96, 0x136, 0x2e4, 0x172, 0, 0, 0x22, setup_colour(G_GUI_COLOUR_DARK_BLUE), 0, 0x8c, 4, pale, entry, 0x32, 0x22, 0, usize::MAX);
    let a = area(current_human_display(), 0x64, 0x22b, 0x316, 0x24e, 2, columns.as_mut_ptr() as usize, 1, 0, 1, 0, usize::MAX);
    text(SETUP_TEXT, 0x98a1a0); // "Cancel"
    button(sm, a, 0, 0, 0x30, grey, 0xc, 3, pale, SETUP_TEXT, 0x23, 0);
    text(SETUP_TEXT, 0x990f7c); // "Ok"
    if read_u8(typed(read_usize(G_HUMAN_MANAGER), manager)) != 0 {
        button(sm, a, 1, 0, 0x30, grey, 0xc, 3, pale, SETUP_TEXT, 0x24, 0);
    } else {
        label(sm, a, 1, 0, 0x30, grey, 0x2c, 3, pale, SETUP_TEXT);
    }
}

/// Resumes control of `manager`: no longer computer controlled nor on holiday; when the current slot has no manager
/// he becomes its manager (and the player join/leave hooks are set to 0x838a80/0x838c00), otherwise a record is added
/// for him (`ScreenManager::add_manager_record`). −9.
unsafe fn setup_resume_manager(manager: usize) -> i32 {
    let computer: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_HUMAN_MANAGER_SET_COMPUTER_CONTROLLED);
    let add_record: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_SCREENMANAGER_ADD_MANAGER_RECORD);
    computer(read_usize(G_HUMAN_MANAGER), 0, manager);
    exe::human_manager_set_on_holiday(read_usize(G_HUMAN_MANAGER), 0, manager);
    let slot = G_SCREEN_RECORDS_MANAGER + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300;
    if read_usize(slot) == 0 {
        write_usize(slot, manager);
        write_usize(G_SCREEN_MANAGER_PLAYER_JOINED_CALLBACK, NETWORK_JOIN_CALLBACK);
        write_usize(G_SCREEN_MANAGER_PLAYER_LEFT_CALLBACK, NETWORK_LEAVE_CALLBACK);
    } else {
        add_record(SCREEN_MANAGER, NETWORK_JOIN_CALLBACK, NETWORK_LEAVE_CALLBACK, manager);
    }
    -9
}

/// The "Already Controlled" box for a manager who already has a channel (`exe::screenmanager_find_manager_channel` ≥ 0):
/// "<manager> has already resumed control.", red with pale grey text, code 0x3b. Returns whether it opened.
unsafe fn setup_already_resumed(manager: usize) -> bool {
    let text: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if (exe::screenmanager_find_manager_channel(SCREEN_MANAGER, manager) as i16) <= -1 {
        return false;
    }
    let mut name = [0u8; 0xc8];
    exe::database_format_staff_name(manager, name.as_mut_ptr() as usize, 0xc8, 1);
    let (red, pale) = (setup_colour(G_GUI_COLOUR_RED), setup_colour(G_GUI_COLOUR_PALE_GREY));
    setup_format_title();
    text(SETUP_TEXT, 0xa809b0, name.as_ptr()); // "<%s - manager> has already resumed control."
    setup_confirm_box(0xa80900, pale, red, 0x3b); // "Already Controlled"
    true
}

/// The Enter Password box's answer to item `item`.
///
/// After its own box (code 0x3b): cleared, −9. The entry (0x22) −11; Cancel (0x23) −9. Ok (0x24): a manager who
/// already has a channel gives the "Already Controlled" box (−4); a typed password different from his and from the
/// master password "sportsinteractive" is emptied and gives the "Incorrect Password" box (pale grey on dark red, −4);
/// otherwise he resumes control (`setup_resume_manager`, −9). Any other goes to the menu bar.
/// Original: `enter_password_box_handle_input` (0x8230b0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn enter_password_box_handle_input(item: u32) -> i32 {
    covered!("tel11_4 enter password input");
    let text: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let password: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_PASSWORD);
    let typed: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_PASSWORD_ENTRY);
    let hm = || read_usize(G_HUMAN_MANAGER);
    if read_u32(G_PENDING_COMMAND_DISPATCH_FLAG) != 0 && read_u32(G_PENDING_COMMAND_CODE) == 0x3b {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        return -9;
    }
    let manager = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0);
    match setup_clicked_id(item as i16) {
        0x22 => -11,
        0x23 => -9,
        0x24 => {
            if setup_already_resumed(manager) {
                return -4;
            }
            let entry = typed(hm(), manager);
            if setup_texts_differ(password(hm(), manager), entry) && setup_texts_differ(typed(hm(), manager), SETUP_MASTER_PASSWORD) {
                write_u8(typed(hm(), manager), 0);
                setup_format_title();
                text(SETUP_TEXT, 0xa80978); // "The password entered is incorrect."
                setup_confirm_box(0xa80964, setup_colour(G_GUI_COLOUR_PALE_GREY), setup_colour(G_GUI_COLOUR_DARK_RED), 0); // tel11_4 "Incorrect Password"
                return -4;
            }
            setup_resume_manager(manager)
        }
        _ => {
            let r = exe::menubar_handle_command(item);
            if r != 0 { r } else { exe::menubar_handle_object_link(item) }
        }
    }
}

/// Builds the resume control box of the manager in param 0: his name as title, "Please Confirm" and the question with
/// what he runs: the national side and the club ("Resume control of {}<nation>{} and {}<club>{} ?", the national side's
/// picture; the gender bytes go club, club, nation, nation and the short names nation, club, as in the original), the
/// national side only, the club only (their pictures, `club::NAME`) or nothing ("Resume control ?", a random
/// background); No (0x25) / Yes (0x26).
/// Original: `resume_control_confirm_build` (0x823420, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn resume_control_confirm_build() {
    covered!("tel11_4 build resume control");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let manager = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0);
    setup_box_header(manager, 0x9909b8); // "Please Confirm"
    let (national, club) = (read_usize(manager + staff::NATIONAL_JOB), read_usize(manager + staff::CLUB_JOB));
    if national != 0 && club != 0 {
        exe::background_set_club_picture(national, 0, 0);
        let (gc, gn) = (read_i8(club + club::SHORT_GENDER_NAME) as i32, read_i8(national + club::SHORT_GENDER_NAME) as i32);
        text(SETUP_TEXT, 0xa80a3c, gc, gc, gn, gn, national + club::SHORT_NAME, club + club::SHORT_NAME);
    } else if national != 0 || club != 0 {
        let side = if national != 0 { national } else { club };
        exe::background_set_club_picture(side, 0, 0);
        let g = read_i8(side + club::GENDER_NAME) as i32;
        text(SETUP_TEXT, if national != 0 { 0xa80a14 } else { 0xa809f0 }, g, g, side + club::NAME);
    } else {
        exe::background_set_random_picture();
        text(SETUP_TEXT, 0xa809dc); // "Resume control ?"
    }
    setup_box_question();
    setup_box_buttons(0x98e8cc, 0x25, 0x98e8d0, 0x26); // "No", "Yes"
}

/// The resume control box's answer to item `item`: after its own box (code 0x3b) cleared and −9; No (0x25) −9; Yes
/// (0x26) the "Already Controlled" box when the manager already has a channel (−4), otherwise he resumes control
/// (`setup_resume_manager`, −9); any other goes to the menu bar.
/// Original: `resume_control_confirm_handle_input` (0x823660, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn resume_control_confirm_handle_input(item: u32) -> i32 {
    covered!("tel11_4 resume control input");
    if read_u32(G_PENDING_COMMAND_DISPATCH_FLAG) != 0 && read_u32(G_PENDING_COMMAND_CODE) == 0x3b {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        return -9;
    }
    let manager = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0);
    match setup_clicked_id(item as i16) {
        0x25 => -9,
        0x26 => {
            if setup_already_resumed(manager) {
                return -4;
            }
            setup_resume_manager(manager)
        }
        _ => {
            let r = exe::menubar_handle_command(item);
            if r != 0 { r } else { exe::menubar_handle_object_link(item) }
        }
    }
}

/// Comparer of `open_take_control_confirm`: only the original runs (it opens the box).
pub(crate) unsafe extern "C" fn compare_open_take_control_confirm(club: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I0%%)(club)
}

/// Comparer of `take_control_confirm_build`: side by side over the display list (with the club's picture: the
/// generator is restored by `compare_by_display_list_with`).
pub(crate) unsafe extern "C" fn compare_take_control_confirm_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    compare_by_display_list_with("Take Control box", &[], || o(), || take_control_confirm_build());
}

/// Comparer of `take_control_confirm_handle_input`: only the original runs (it appoints the manager or opens boxes).
pub(crate) unsafe extern "C" fn compare_take_control_confirm_handle_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%)(item)
}

/// Comparer of `enter_password_box_build`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_enter_password_box_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I3%%);
    compare_by_display_list_with("Enter Password box", &[], || o(), || enter_password_box_build());
}

/// Comparer of `enter_password_box_handle_input`: only the original runs (it resumes the manager or opens boxes).
pub(crate) unsafe extern "C" fn compare_enter_password_box_handle_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I4%%)(item)
}

/// Comparer of `resume_control_confirm_build`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_resume_control_confirm_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I5%%);
    compare_by_display_list_with("Resume Control box", &[], || o(), || resume_control_confirm_build());
}

/// Comparer of `resume_control_confirm_handle_input`: only the original runs (it resumes the manager or opens a box).
pub(crate) unsafe extern "C" fn compare_resume_control_confirm_handle_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I6%%)(item)
}
