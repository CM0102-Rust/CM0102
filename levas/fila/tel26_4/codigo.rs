// ---------- batch %%LEVA%%: the manager confirm screens and the board confidence (manager_screens.cpp) ----------

/// Opens a "Please Confirm" screen for `club` (`open_screen` with `builder`, `handler`, 1, 0, 0; parameter 0 =
/// the club); a null club shows the manager_screens.cpp box at `line` instead.
unsafe fn manager_confirm_open(club: usize, builder: usize, handler: usize, single: u32, line: u32) {
    if club == 0 {
        tel26_error_box(MANAGER_SCREENS_CPP, line);
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, builder, handler, single, 0usize, 0) != 0 {
        set_screen_param(0, club);
    }
}

/// The "Please Confirm" screen of a club (parameter `club_param`): the club's background picture, the menu bar
/// (4, 0), the title (the human manager's name in the club's kit colours, `database_get_club_kit_colours`), the
/// subtitle "Please Confirm", the question (`question`, with the club's name and gender char +0x37) as a text area,
/// and a two-column area (weights 3/1) with "No" (id 1) and "Yes" (id 2).
unsafe fn manager_confirm_build(club_param: u32, question: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let weights: [u8; 2] = [3, 1];
    let club = get_screen_param(club_param);
    exe::background_set_club_picture(club, 0, 0);
    exe::menubar_build(4, 0);
    let (mut fore, mut back) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut fore, &mut back);
    exe::database_format_staff_name(tel26_slot_manager(), JOB_TEXT, 0x7d0, 1);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, JOB_TEXT);
    text(JOB_TEXT, 0x9909b8);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let gender = read_u8(club + 0x37) as i8 as i32;
    text(JOB_TEXT, question, gender, gender, club + 4);
    tel26_text_area([0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0]);
    let area = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    text(JOB_TEXT, 0x98e8cc);
    tel26_area_button([area, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 1, 0]);
    text(JOB_TEXT, 0x98e8d0);
    tel26_area_button([area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 2, 0]);
}

/// The menu bar's answer to a widget the screen does not handle: `menubar_handle_command`, then
/// `menubar_handle_object_link` when it gives 0.
unsafe fn manager_menu_bar(raw: u32) -> i32 {
    let r = exe::menubar_handle_command(raw);
    if r != 0 { r } else { exe::menubar_handle_object_link(raw) }
}

/// Opens the confirm screen asking to apply for `club`'s job (builder 0x6b1a80, handler 0x6b1c30); a null club
/// shows the manager_screens.cpp 1319 box.
/// Original: `open_apply_for_job_screen` (0x6b19d0, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_apply_for_job_screen(club: usize) {
    covered!("tel26_4 open apply for job");
    manager_confirm_open(club, 0x6b1a80, 0x6b1c30, 1, 0x527);
}

/// Builds the "Apply for the <club> job ?" confirm screen (`manager_confirm_build` on parameter 0).
/// Original: `apply_for_job_screen_build` (0x6b1a80, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn apply_for_job_screen_build() {
    covered!("tel26_4 apply for job");
    manager_confirm_build(0, 0xa15434);
}

/// "No" (id 1) closes with -9; "Yes" (id 2) applies the slot's human manager for the club (parameter 0,
/// `MANAGER_MANAGER::apply_for_job` on [0xb67a44]) and closes with -9; other ids go to the menu bar.
/// Original: `apply_for_job_screen_handle_action` (0x6b1c30, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn apply_for_job_screen_handle_action(raw: u32) -> i32 {
    match tel26_item_id(raw as i16) {
        1 => -9,
        2 => {
            let manager = tel26_slot_manager();
            exe::manager_manager_apply_for_job(read_usize(0xb67a44), get_screen_param(0), manager);
            -9
        }
        _ => manager_menu_bar(raw),
    }
}

/// Opens the confirm screen asking to resign from `club` (builder 0x6b1dc0, handler 0x6b1f70); a null club shows
/// the manager_screens.cpp 1428 box.
/// Original: `open_resign_job_screen` (0x6b1d10, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_resign_job_screen(club: usize) {
    covered!("tel26_4 open resign");
    manager_confirm_open(club, 0x6b1dc0, 0x6b1f70, 1, 0x594);
}

/// Builds the "Resign as manager of <club> ?" confirm screen (`manager_confirm_build` on parameter 0).
/// Original: `resign_job_screen_build` (0x6b1dc0, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn resign_job_screen_build() {
    covered!("tel26_4 resign");
    manager_confirm_build(0, 0xa15458);
}

/// "No" (id 1) closes with -9; "Yes" (id 2) removes the manager of the club (parameter 0,
/// `MANAGER_MANAGER::remove_club_manager` with departure 2), refreshes the match day's human interest flags
/// ([0xb67a50], when there is one), opens the vacancy with the slot's manager as the previous one (`add_job_vacancy`)
/// and closes with -9; other ids go to the menu bar.
/// Original: `resign_job_screen_handle_action` (0x6b1f70, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn resign_job_screen_handle_action(raw: u32) -> i32 {
    let club = get_screen_param(0);
    match tel26_item_id(raw as i16) {
        1 => -9,
        2 => {
            exe::manager_manager_remove_club_manager(read_usize(0xb67a44), club, 2, 0, 0);
            let day = read_usize(0xb67a50);
            if day != 0 {
                exe::match_day_refresh_human_interest_flags(day);
            }
            exe::manager_manager_add_job_vacancy(read_usize(0xb67a44), club, tel26_slot_manager());
            -9
        }
        _ => manager_menu_bar(raw),
    }
}

/// Opens the Board Confidence screen of `club` (builder 0x6b2130, handler 0x6b2370); a null club shows the
/// manager_screens.cpp 1548 box.
/// Original: `open_board_confidence_screen` (0x6b2080, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_board_confidence_screen(club: usize) {
    covered!("tel26_4 open board confidence");
    manager_confirm_open(club, 0x6b2130, 0x6b2370, 0, 0x60c);
}

/// Builds the Board (or F.A., for a national team) Confidence screen of the club in parameter 0.
///
/// The club's background picture, the menu bar (4, 0), the title (the club's name in its kit colours), the
/// subtitle "F.A. Confidence" or "Board Confidence"; for a club, the "Board Request" (id 1) and "FA Request" (id
/// 0x145) buttons. The days in the job = 365 × (the year [0xae2c92] − the manager's start year +0x40, both
/// unsigned) − his start day (+0x3e) + the day [0xae2c90]; `MANAGER_MANAGER::build_board_confidence_text` (club,
/// days, 2000 bytes) and `build_board_squad_opinion_text` (into the same frame at +0x3f0) give the text: both
/// ("%s" twice, 0x991948) or only the first ("%s") in a text area; then the Back/Next buttons.
/// Original: `board_confidence_screen_build` (0x6b2130, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn board_confidence_screen_build() {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let confidence: unsafe extern "thiscall" fn(usize, usize, i32, *mut u8, u32) = core::mem::transmute(0x690d80usize);
    let opinion: unsafe extern "thiscall" fn(usize, *mut u8, usize, u32) -> u8 = core::mem::transmute(0x693850usize);
    covered!("tel26_4 board confidence");
    // the original's frame: the confidence text at +8 (2000 bytes) and the squad opinion at +0x3f0, overlapping
    let mut frame = [0u8; 0x7d8];
    let club = get_screen_param(0);
    let manager = tel26_slot_manager().addr();
    exe::background_set_club_picture(club, 0, 0);
    exe::menubar_build(4, 0);
    let (mut fore, mut back) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut fore, &mut back);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, club + 4);
    text(JOB_TEXT, if exe::database_is_national_team(club) != 0 { 0xa154b0 } else { 0xa1549c });
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    if exe::database_is_national_team(club) == 0 {
        text(JOB_TEXT, 0xa1548c);
        tel26_add_button([0x6e, 0x82, 0xeb, 0x96, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 1, 0]);
        text(JOB_TEXT, 0xa15480);
        tel26_add_button([0xf0, 0x82, 0x16d, 0x96, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0x145, 0]);
    }
    let years = (read_u16(0xae2c92) as i32).wrapping_sub(read_u16(manager + 0x40) as i32);
    let days = years.wrapping_mul(365).wrapping_sub(read_i16(manager + 0x3e) as i32).wrapping_add(read_i16(0xae2c90) as i32);
    let first = frame.as_mut_ptr().wrapping_add(8);
    let second = frame.as_mut_ptr().wrapping_add(0x3f0);
    confidence(read_usize(0xb67a44), club, days, first, 0x7d0);
    if opinion(read_usize(0xb67a44), second, club, 0) != 0 {
        format(JOB_TEXT, 0x991948, first, second);
    } else {
        format(JOB_TEXT, 0x991944, first);
    }
    tel26_text_area([0x6e, 0x9b, 0x30c, 0x21c, 2, 0, 0x10, 4, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0]);
    tel26_back_next();
}

/// "Board Request" (id 1) opens the board request screen of the club (parameter 0, `board_request_screen_open`
/// 0x425390) and "FA Request" (0x145) the FA meeting (`fa_meeting_open_screen` 0x58acd0), both returning -4;
/// other ids go to the menu bar.
/// Original: `board_confidence_screen_handle_action` (0x6b2370, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn board_confidence_screen_handle_action(raw: u32) -> i32 {
    let target = match tel26_item_id(raw as i16) {
        1 => 0x425390usize,
        0x145 => 0x58acd0,
        _ => return manager_menu_bar(raw),
    };
    let open: unsafe extern "C" fn(usize) = core::mem::transmute(target);
    open(get_screen_param(0));
    -4
}

/// Opens the confirm screen of a job offer news item: the item `news_id` of `manager`'s inbox is fetched
/// (`News::find_item_by_id` on [0xb7431c] into a local `NewsItem`); a null manager or a missing item shows the
/// manager_screens.cpp 1678 / 1685 box. When the screen opens (builder 0x6b25e0, handler 0x6b2790, 1): parameter
/// 0 = the news id and 1 = the offering club (the club table entry of the item's slot 0).
/// Original: `open_job_offer_screen` (0x6b2450, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_job_offer_screen(manager: usize, news_id: u32) {
    let find: unsafe extern "thiscall" fn(usize, usize, u32, *mut u8) -> u32 = core::mem::transmute(0x786b70usize);
    covered!("tel26_4 open job offer");
    let mut item = [0u8; 0x100];
    exe::new_newsitem(item.as_mut_ptr() as usize, 0, 0);
    if manager == 0 {
        tel26_error_box(MANAGER_SCREENS_CPP, 0x68e);
        return;
    }
    if find(read_usize(0xb7431c), manager, news_id, item.as_mut_ptr()) == 0 {
        tel26_error_box(MANAGER_SCREENS_CPP, 0x695);
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x6b25e0usize, 0x6b2790usize, 1, 0usize, 0) == 0 {
        return;
    }
    let slot = exe::newsitem_get_slot(item.as_mut_ptr() as usize, 0);
    let club = read_usize(G_CLUB_TABLE).wrapping_add(slot.wrapping_mul(0x245) as usize);
    set_screen_param(0, news_id as usize);
    set_screen_param(1, club);
}

/// Builds the "Accept the position as manager of <club> ?" confirm screen (`manager_confirm_build` on parameter 1).
/// Original: `job_offer_screen_build` (0x6b25e0, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn job_offer_screen_build() {
    covered!("tel26_4 job offer");
    manager_confirm_build(1, 0xa154c0);
}

/// The job offer's answer: "No" (id 1) marks the news item (parameter 0) answered with 2
/// (`News::set_item_slot(manager, id, 1, 2, 0)`) and puts the vacancy back (`add_job_vacancy` of the club in
/// parameter 1); "Yes" (id 2) marks it with 1 and appoints the slot's manager (`appoint_manager_to_club`); both
/// close with -9; other ids go to the menu bar.
/// Original: `job_offer_screen_handle_action` (0x6b2790, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn job_offer_screen_handle_action(raw: u32) -> i32 {
    let mark: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x786d30usize);
    let appoint: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x69ebc0usize);
    let news = get_screen_param(0);
    let club = get_screen_param(1);
    let manager = tel26_slot_manager().addr();
    match tel26_item_id(raw as i16) {
        1 => {
            mark(read_usize(0xb7431c), manager, news, 1, 2, 0);
            exe::manager_manager_add_job_vacancy(read_usize(0xb67a44), club, manager);
            -9
        }
        2 => {
            mark(read_usize(0xb7431c), manager, news, 1, 1, 0);
            appoint(read_usize(0xb67a44), club, manager);
            -9
        }
        _ => manager_menu_bar(raw),
    }
}

/// Comparer for `open_apply_for_job_screen`.
pub(crate) unsafe extern "C" fn compare_open_apply_for_job_screen(club: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize)>(%%I0%%)(club)
}

/// Comparer for `apply_for_job_screen_build`: the confirm screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_apply_for_job_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    compare_by_display_list_with("Apply for job screen (apply_for_job_screen_build)", &[], || o(), || apply_for_job_screen_build());
}

/// Comparer for `apply_for_job_screen_handle_action`.
pub(crate) unsafe extern "C" fn compare_apply_for_job_screen_handle_action(item: u32) -> i32 {
    // simplification: the application and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%)(item)
}

/// Comparer for `open_resign_job_screen`.
pub(crate) unsafe extern "C" fn compare_open_resign_job_screen(club: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize)>(%%I3%%)(club)
}

/// Comparer for `resign_job_screen_build`: the confirm screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_resign_job_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I4%%);
    compare_by_display_list_with("Resign screen (resign_job_screen_build)", &[], || o(), || resign_job_screen_build());
}

/// Comparer for `resign_job_screen_handle_action`.
pub(crate) unsafe extern "C" fn compare_resign_job_screen_handle_action(item: u32) -> i32 {
    // simplification: the resignation and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I5%%)(item)
}

/// Comparer for `open_board_confidence_screen`.
pub(crate) unsafe extern "C" fn compare_open_board_confidence_screen(club: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize)>(%%I6%%)(club)
}

/// Comparer for `board_confidence_screen_build`: the Board Confidence screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_board_confidence_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I7%%);
    compare_by_display_list_with("Board Confidence screen (board_confidence_screen_build)", &[], || o(), || board_confidence_screen_build());
}

/// Comparer for `board_confidence_screen_handle_action`.
pub(crate) unsafe extern "C" fn compare_board_confidence_screen_handle_action(item: u32) -> i32 {
    // simplification: opening the next screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I8%%)(item)
}

/// Comparer for `open_job_offer_screen`.
pub(crate) unsafe extern "C" fn compare_open_job_offer_screen(manager: usize, news_id: u32) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize, u32)>(%%I9%%)(manager, news_id)
}

/// Comparer for `job_offer_screen_build`: the confirm screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_job_offer_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I10%%);
    compare_by_display_list_with("Job offer screen (job_offer_screen_build)", &[], || o(), || job_offer_screen_build());
}

/// Comparer for `job_offer_screen_handle_action`.
pub(crate) unsafe extern "C" fn compare_job_offer_screen_handle_action(item: u32) -> i32 {
    // simplification: answering the offer and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I11%%)(item)
}
