// ---------- batch %%LEVA%%: the Football Association meeting screen (fifa_confed.cpp) ----------

/// The screen manager's flag at +0x3834 (0xb77c84) that the dialog handlers check first: set, the handler clears the
/// pending command argument [`G_PENDING_COMMAND_ARG`] and closes (a network game's answer) [inference].
const TEL34_SCREEN_DIALOG_FLAG: usize = 0xb77c84;

/// Rule: opening the Football Association meeting screen of a club: parameter 0 = the club's index, 1 and 2 cleared.
///
/// `open_screen` (builder 0x58ad30, handler 0x58b0f0, single instance, leave 0x5a8f60 (the shared empty routine), no
/// exit); when it opens, parameter 0 = the club record's id (+0), parameters 1 and 2 = 0.
/// Original: `fa_meeting_open_screen` (0x58acd0, fifa_confed.cpp, cdecl).
pub(crate) unsafe extern "C" fn fa_meeting_open_screen(club: usize) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x58ad30usize, 0x58b0f0usize, 1, 0x5a8f60usize, 0) != 0 {
        covered!("tel34_5 open fa meeting");
        set_screen_param(0, read_usize(club));
        set_screen_param(1, 0);
        set_screen_param(2, 0);
    }
}

/// Rule: the Football Association meeting screen: the club's manager in the title, the explanation, the two requests
/// to choose from (the chosen one highlighted) and Cancel and Confirm.
///
/// A local `Text` (`Text::Text`, 0); the club = parameter 0 in the club table; no manager (`club::MANAGER`): the Text
/// destroyed and nothing else. The chosen request = parameter 1 (byte); parameter 2 read and not used. The club's
/// background (`background_set_club_picture`, 0, 0); the title: the manager's name (500 bytes) in the club's kit
/// colours; the subtitle "Meet with the Football Association"; the rows = 0x195 / font 3's height (signed, a byte);
/// a 7-column area (weights 2,20,2,10,2,4,3) from (0x6e,0x8c) to (0x30c,0x221) where the explanation ("Specify the
/// details of requests from the Football Association:", translated) is wrapped (`Text::set_string`,
/// `Text::draw_wrapped`, font 3, from line 0); then a 2-column area (weights 2,17) over the same rectangle and, from the
/// line after the explanation + 2, the requests 1 and 2 (`get_fa_request_option_text` 0x6a9e20, 2000 bytes) as buttons
/// in column 1 (id 100, value the request), in the highlight colour [0xaebdec] when chosen, otherwise [0xae317e];
/// the first button's item goes to parameter 2. The menu bar (4, 0); a 2-column area (weights 3/1) in the kit's second
/// colour with "Cancel" (id 4) and "Confirm" (id 5); the Text destroyed.
/// Original: `fa_meeting_screen_draw` (0x58ad30, fifa_confed.cpp, cdecl).
pub(crate) unsafe extern "C" fn fa_meeting_screen_draw() {
    type DrawWrapped = unsafe extern "thiscall" fn(*mut usize, u32, u32, u32, *mut i32, u32, u32, u32, u32) -> u32;
    type RequestText = unsafe extern "thiscall" fn(usize, u32, usize, u32);
    let weights_explanation: [u8; 7] = [2, 0x14, 2, 0xa, 2, 4, 3];
    let weights_requests: [u8; 2] = [2, 0x11];
    let weights_buttons: [u8; 2] = [3, 1];
    let mut text = [0usize; 8];
    let mut name = [0u8; 500];
    // the wrapped text's line counter (zeroed again by the original just before the wrap)
    let mut line: i32 = 0;
    exe::new_text(text.as_mut_ptr(), 0usize);
    let club = club_at(get_screen_param(0) as i32 as isize);
    let manager = read_usize(club.addr() + club::MANAGER);
    if manager == 0 {
        exe::drop_text(text.as_mut_ptr());
        return;
    }
    covered!("tel34_5 fa meeting");
    let chosen = get_screen_param(1) as u8;
    get_screen_param(2);
    exe::background_set_club_picture(club.addr(), 0, 0);
    let (mut fore, mut back) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club.addr(), 1, &mut fore, &mut back);
    exe::database_format_staff_name(manager, name.as_mut_ptr() as usize, 0x1f4, 1);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, name.as_ptr() as usize);
    tel34_lang_text(0x9c7450); // "Meet with the Football Association"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let rows = 0x195i32.wrapping_div(exe::font_get_height(3)) as u8 as u32;
    let explanation = tel26_add_area(current_human_display(), [0x6e, 0x8c, 0x30c, 0x221, 7, weights_explanation.as_ptr() as u32, rows, 0, 2, 0, u32::MAX]);
    tel34_lang_text(0x9c740c); // "  Specify the details of requests from the Football Association:"
    exe::text_set_string(text.as_mut_ptr() as usize, JOB_TEXT as *const u8);
    let wrap: DrawWrapped = core::mem::transmute(0x4aeaf0usize);
    wrap(text.as_mut_ptr(), explanation, 0x6e, 0x30c, &mut line, 3, job_colour(JOB_COLOUR_3184), 0, 0);
    let requests = tel26_add_area(current_human_display(), [0x6e, 0x8c, 0x30c, 0x221, 2, weights_requests.as_ptr() as u32, rows, 0, 1, 0, u32::MAX]);
    line = line.wrapping_add(2);
    let request_text: RequestText = core::mem::transmute(0x6a9e20usize);
    for request in 1u8..=2 {
        request_text(read_usize(0xb67a44), request as u32, JOB_TEXT, 0x7d0);
        let colour = if chosen == request { job_colour(JOB_COLOUR_BDEC) } else { job_colour(JOB_COLOUR_317E) };
        let item = tel26_area_button([requests, 1, line as u32, 1, 0, 1, 3, colour, JOB_TEXT as u32, 0x64, request as i8 as i32 as u32]);
        if request == 1 {
            set_screen_param(2, item as u16 as i16 as isize as usize);
        }
        line = line.wrapping_add(1);
    }
    exe::menubar_build(4, 0);
    let buttons = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, weights_buttons.as_ptr() as u32, 1, 0, 1, back, u32::MAX]);
    tel34_lang_text(0x98a1a0); // "Cancel"
    tel26_area_button([buttons, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 4, 0]);
    tel34_lang_text(0x98a198); // "Confirm"
    tel26_area_button([buttons, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 5, 0]);
    exe::drop_text(text.as_mut_ptr());
}

/// Rule: on the FA meeting screen a request button picks that request (the two buttons recoloured, the screen
/// redrawn); Confirm sends the chosen request to the FA and closes; Cancel closes; other widgets go to the menu bar.
///
/// The club = parameter 0 in the club table, the chosen request = parameter 1 (parameter 2 read and not used). With
/// the dialog flag (0xb77c84) set: the pending command argument cleared and -9. Id 100: the button's value (0 for -1),
/// parameter 2 = the first button's item; value 1: the first button in the highlight colour [0xaebdec] and the second in
/// [0xae317e]; value 2: the other way round (`ScreenManager::set_widget_text_colour`); any value: parameter 1 = it,
/// `ScreenManager::redraw_screen` (0, 0), 0. Id 5: with a chosen request, `MANAGER_MANAGER::handle_fa_request`
/// (0x6a9e60, club, request); -9. Id 4: -9.
/// Original: `fa_meeting_screen_handle_event` (0x58b0f0, fifa_confed.cpp, cdecl).
pub(crate) unsafe extern "C" fn fa_meeting_screen_handle_event(raw: u32) -> i32 {
    let club = club_at(get_screen_param(0) as i32 as isize);
    let chosen = get_screen_param(1);
    get_screen_param(2);
    if read_u32(TEL34_SCREEN_DIALOG_FLAG) != 0 {
        write_u32(G_PENDING_COMMAND_ARG, 0);
        return -9;
    }
    let item = raw as i16;
    match tel26_item_id(item) {
        100 => {
            covered!("tel34_5 fa meeting pick");
            let value = tel26_item_value(item);
            let first = get_screen_param(2) as u32;
            let colour = |widget: u32, c: usize| exe::screenmanager_set_widget_text_colour(G_SCREEN_MANAGER, widget, job_colour(c));
            match value {
                1 => {
                    colour(first, JOB_COLOUR_BDEC);
                    colour(first.wrapping_add(1), JOB_COLOUR_317E);
                }
                2 => {
                    colour(first, JOB_COLOUR_317E);
                    colour(first.wrapping_add(1), JOB_COLOUR_BDEC);
                }
                _ => {}
            }
            set_screen_param(1, value as usize);
            exe::screenmanager_redraw_screen(G_SCREEN_MANAGER, 0, 0);
            0
        }
        5 => {
            if chosen != 0 {
                covered!("tel34_5 fa meeting confirm");
                let handle: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a9e60usize);
                handle(read_usize(0xb67a44), club.addr(), chosen as u32);
            }
            -9
        }
        4 => -9,
        _ => {
            let r = exe::menubar_handle_command(raw);
            if r != 0 { r } else { exe::menubar_handle_object_link(raw) }
        }
    }
}

/// Comparer for `fa_meeting_open_screen`.
pub(crate) unsafe extern "C" fn compare_fa_meeting_open_screen(club: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize)>(%%I0%%)(club)
}

/// Comparer for `fa_meeting_screen_draw`: the FA meeting screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_fa_meeting_screen_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    compare_by_display_list_with("FA meeting screen (fa_meeting_screen_draw)", &[], || o(), || fa_meeting_screen_draw());
}

/// Comparer for `fa_meeting_screen_handle_event`.
pub(crate) unsafe extern "C" fn compare_fa_meeting_screen_handle_event(raw: u32) -> i32 {
    // simplification: the redraw (network), the FA request and the menu bar commands do not run twice; proven in Rust
    // mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%)(raw)
}
