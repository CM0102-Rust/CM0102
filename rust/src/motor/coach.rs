use super::*;

/// Opens the coach report request screen for `player` from `coach`'s profile (the coach's club, `staff::CLUB_JOB`).
///
/// Nothing without a player, a coach or his club. The screen opens (`ScreenManager::open_screen` with
/// `FN_COACH_REPORT_REQUEST_DRAW` and `FN_COACH_REPORT_REQUEST_HANDLE_ACTION`, single instance, the empty 0x5a8f60
/// as the leave function); with no candidate (`coach_report_candidates`) nothing more. Parameter 0 = the player's
/// id, 1 = the bit of the coach among the candidates (the last match; 0 when he is not one), 2 = the id of the club
/// or, for a B team (`club_get_linked_b_team` without the senior flag), of its senior side.
/// Original: `coach_open_report_request_screen` (0x48bc00, coach.cpp, cdecl).
pub(crate) unsafe extern "C" fn coach_open_report_request_screen(player: usize, coach: usize) {
    let coach = Staff(coach);
    if player == 0 || coach.is_null() {
        return;
    }
    let mut club = coach.club_job().addr();
    if club == 0 {
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, FN_COACH_REPORT_REQUEST_DRAW, FN_COACH_REPORT_REQUEST_HANDLE_ACTION, 1, FN_SHARED_NOOP_RET0_5A8F60, 0) == 0 {
        return;
    }
    let (n, list) = coach_report_candidates(club, player);
    if n == 0 {
        return;
    }
    covered!("tel30_9 coach report screen opened");
    let mut mask: u32 = 0;
    for (k, &c) in list.iter().take(n as usize).enumerate() {
        if c == coach.addr() {
            mask = 1 << k;
        }
    }
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0, read_usize(player), 0);
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 1, mask as usize, 0);
    let mut senior = 0u32;
    let linked = exe::club_get_linked_b_team(club, &mut senior, 1);
    if linked != 0 && senior == 0 {
        club = linked;
    }
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 2, read_usize(club), 0);
}

/// Builds the coach report request screen.
///
/// Parameter 1 holds the chosen coaches' bits. The training picture (`background_set_section_picture(6, 0)`); the
/// title is the player's name (`database_format_staff_name`, style 1) in the club's kit colours (kit 1, background
/// then foreground); the subtitle "Request Coach Report"; a "Select All" button (0x2a3, 0x82)-(0x30c, 0x96), id 4.
/// With a single candidate (`coach_report_candidates`) a text area (0x6e, 0x8c)-(0x30c, 0x221) asks "Ask <coach> to
/// file a report on <player> ?" (0x186 / font_get_height(3) rows, pale grey); otherwise an area
/// (0x6e, 0x9b)-(0x30c, 0x221) (columns [2, 0x14, 2, 0xa, 2, 4, 3]) takes the wrapped question with a link on the
/// player's name (0x7d7), and a second one (columns [2, 0x11, 0xa]) from two rows below it one row per candidate: his
/// name (column 1) and his club job (`database_get_job_name(job, 0, 1, 0)`, column 2), buttons with id 1 and his bit
/// as value, yellow when chosen, grey otherwise. Then the menu bar (`menubar_build(4, 0)`) and an area
/// (0x64, 0x22b)-(0x316, 0x24e) (columns [3, 1], one row, the kit background): "Cancel" (id 2) and "Confirm" (id 3, a
/// plain text with no coach chosen). The colour words are passed zero-extended.
/// Original: `coach_report_request_draw` (0x48bd20, coach.cpp, cdecl).
pub(crate) unsafe extern "C" fn coach_report_request_draw() {
    let mut text = [0usize; 2];
    exe::new_text(text.as_mut_ptr(), 0usize);
    let mut row: i32 = 0;
    let player = coach_report_player();
    let chosen = get_screen_param(1) as u32;
    let club = coach_report_club();
    let (n, list) = coach_report_candidates(club, player);
    background_set_section_picture(6, 0);
    let (mut foreground, mut background) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut foreground, &mut background);
    let mut name = [0u8; 500];
    exe::database_format_staff_name(player, name.as_mut_ptr() as usize, 0x1f4, 1);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, background & 0xffff, foreground & 0xffff, name.as_ptr() as usize);
    let langlib: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    langlib(JOB_TEXT, COACH_REPORT_TITLE);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    langlib(JOB_TEXT, COACH_REPORT_SELECT_ALL);
    let (grey, pale, yellow) = (read_u16(G_GUI_COLOUR_GREY) as u32, read_u16(G_GUI_COLOUR_PALE_GREY) as u32, read_u16(G_GUI_COLOUR_YELLOW) as u32);
    tel26_add_button([0x2a3, 0x82, 0x30c, 0x96, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, COACH_REPORT_ITEM_SELECT_ALL as u32, 0]);
    let rows = (0x186 / exe::font_get_height(3)) as u8;
    if n == 1 {
        covered!("tel30_9 coach report: a single coach");
        let mut coach_name = [0u8; 0x96];
        exe::database_format_staff_name(list[0], coach_name.as_mut_ptr() as usize, 0x96, 1);
        exe::database_format_staff_name(player, name.as_mut_ptr() as usize, 0x96, 1);
        langlib(JOB_TEXT, COACH_REPORT_ASK_ONE, coach_name.as_ptr(), name.as_ptr());
        tel26_text_area([0x6e, 0x8c, 0x30c, 0x221, 2, 0, rows as i8 as i32 as u32, 3, pale, JOB_TEXT as u32, 0]);
    } else {
        let question: [u8; 7] = [2, 0x14, 2, 0xa, 2, 4, 3];
        let coaches: [u8; 3] = [2, 0x11, 0xa];
        let intro = tel26_add_area(current_human_display(), [0x6e, 0x9b, 0x30c, 0x221, 7, question.as_ptr() as u32, rows as u32, 0, 2, 0, 0xffff_ffff]);
        langlib(JOB_TEXT, COACH_REPORT_ASK_WHICH, name.as_ptr());
        exe::text_set_string(text.as_mut_ptr() as usize, JOB_TEXT as *const u8);
        exe::text_add_link(text.as_mut_ptr() as usize, name.as_ptr(), 0x7d7, player);
        exe::text_draw_wrapped(text.as_mut_ptr() as usize, intro, 0x6e, 0x30c, &mut row, 3, pale, 0, 0);
        let area = tel26_add_area(current_human_display(), [0x6e, 0x9b, 0x30c, 0x221, 3, coaches.as_ptr() as u32, rows as u32, 0, 1, 0, 0xffff_ffff]);
        row += 2;
        let mut bit: u32 = 1;
        for &c in list.iter().take(n.max(0) as usize) {
            // the original writes each coach's name over the player's, in the same buffer
            exe::database_format_staff_name(c, name.as_mut_ptr() as usize, 0x96, 1);
            let colour = if bit & chosen != 0 { yellow } else { grey };
            tel26_area_button([area, 1, row as u32, 1, 0, 1, 3, colour, name.as_ptr() as u32, COACH_REPORT_ITEM_COACH as u32, bit]);
            let job = exe::database_get_job_name(read_u8(c + staff::JOB_FOR_CLUB) as u32, 0, 1, core::ptr::null_mut());
            tel26_area_button([area, 2, row as u32, 1, 0, 1, 3, colour, job as u32, COACH_REPORT_ITEM_COACH as u32, bit]);
            row += 1;
            bit <<= 1;
        }
    }
    exe::menubar_build(4, 0);
    let footer: [u8; 2] = [3, 1];
    let bar = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, footer.as_ptr() as u32, 1, 0, 1, background, 0xffff_ffff]);
    langlib(JOB_TEXT, COACH_REPORT_CANCEL);
    tel26_area_button([bar, 0, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, COACH_REPORT_ITEM_CANCEL as u32, 0]);
    langlib(JOB_TEXT, COACH_REPORT_CONFIRM);
    if chosen != 0 {
        tel26_area_button([bar, 1, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, COACH_REPORT_ITEM_CONFIRM as u32, 0]);
    } else {
        tel26_area_text([bar, 1, 0, 0x30, grey, 0x2c, 3, pale, JOB_TEXT as u32]);
    }
    exe::drop_text(text.as_mut_ptr());
}

/// Answers a click on the coach report request screen.
///
/// Parameters 0 (the player), 1 (the chosen coaches' bits) and 2 (the club) are read. In a network game the pending
/// command argument is cleared and -9 returned. A coach (id 1) flips its bit (the item's value) in parameter 1 and
/// "Select All" (id 4) sets the bits of all the candidates (`coach_report_candidates`), both returning -11 (redraw);
/// Confirm (id 3) asks each chosen candidate for the report (`coach_post_report_news(coach, player)` on 0xadadf4) and
/// returns -9, Cancel (id 2) -9; any other item goes to the menu bar (`menubar_handle_command`, then
/// `menubar_handle_object_link` when it returns 0).
/// Original: `coach_report_request_handle_action` (0x48c290, coach.cpp, cdecl).
pub(crate) unsafe extern "C" fn coach_report_request_handle_action(item: u32) -> i32 {
    let player = coach_report_player();
    let chosen = get_screen_param(1) as u32;
    let club = coach_report_club();
    if read_u32(COACH_REPORT_NETWORK_GAME) != 0 {
        write_u32(G_PENDING_COMMAND_ARG, 0);
        return -9;
    }
    let id = tel26_item_id(item as i16);
    if id == COACH_REPORT_ITEM_COACH {
        set_screen_param(1, (tel26_item_value(item as i16) ^ chosen) as usize);
        return -11;
    }
    if id == COACH_REPORT_ITEM_SELECT_ALL {
        let (n, _) = coach_report_candidates(club, player);
        let mut all: u32 = 0;
        for k in 0..n.max(0) {
            all |= 1 << k;
        }
        set_screen_param(1, all as usize);
        return -11;
    }
    if id == COACH_REPORT_ITEM_CONFIRM {
        covered!("tel30_9 coach reports requested");
        let (n, list) = coach_report_candidates(club, player);
        let mut bit: u32 = 1;
        for &c in list.iter().take(n.max(0) as usize) {
            if chosen & bit != 0 {
                exe::coach_post_report_news(COACH_REPORTS, c, player);
            }
            bit <<= 1;
        }
        return -9;
    }
    if id == COACH_REPORT_ITEM_CANCEL {
        return -9;
    }
    let r = exe::menubar_handle_command(item);
    if r != 0 {
        return r;
    }
    exe::menubar_handle_object_link(item)
}

/// Comparer for `coach_open_report_request_screen`.
pub(crate) unsafe extern "C" fn compare_coach_open_report_request_screen(player: usize, coach: usize) {
    let coach = Staff(coach);
    original_fn::<unsafe extern "C" fn(usize, usize)>(IDX_COACH_OPEN_REPORT_REQUEST_SCREEN)(player, coach.addr());
    let node = current_screen_node(G_SCREEN_MANAGER);
    if player == 0 || coach.is_null() || node == 0 || read_usize(node) != FN_COACH_REPORT_REQUEST_DRAW || get_screen_param(0) != read_usize(player) {
        return;
    }
    let club = coach.club_job().addr();
    let (n, list) = coach_report_candidates(club, player);
    let mut mask: usize = 0;
    for (k, &c) in list.iter().take(n.max(0) as usize).enumerate() {
        if c == coach.addr() {
            mask = 1 << k;
        }
    }
    let got = get_screen_param(1);
    SCOREBOARD.record(got == mask, || format!("coach_open_report_request_screen (Request Coach Report): coach bits original={:#x} rust={:#x}", got, mask));
}

/// Comparer for `coach_report_request_draw`: through the display list (`compare_by_display_list_with`), plus the
/// screen's picture path.
pub(crate) unsafe extern "C" fn compare_coach_report_request_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_COACH_REPORT_REQUEST_DRAW);
    let mut extra = Vec::new();
    let node = current_screen_node(G_SCREEN_MANAGER);
    if node != 0 {
        extra.push((node + screen_node_fields::PICTURE, screen_node_fields::PICTURE_SIZE));
    }
    compare_by_display_list_with("coach_report_request_draw (Request Coach Report)", &extra, || o(), || coach_report_request_draw());
}

/// Comparer for `coach_report_request_handle_action`: a coach, Select All and Cancel side by side over the parameters
/// and the answer (`compare_resposta`); Confirm (the reports go out as news), the menu bar and a network game the
/// original only.
pub(crate) unsafe extern "C" fn compare_coach_report_request_handle_action(item: u32) -> i32 {
    let id = tel26_item_id(item as i16);
    let stays = read_u32(COACH_REPORT_NETWORK_GAME) == 0
        && (id == COACH_REPORT_ITEM_COACH || id == COACH_REPORT_ITEM_SELECT_ALL || id == COACH_REPORT_ITEM_CANCEL);
    compare_resposta("coach_report_request_handle_action (Request Coach Report)", item, original_fn(IDX_COACH_REPORT_REQUEST_HANDLE_ACTION), coach_report_request_handle_action, !stays)
}

/// The coaches a club can ask for a report on `player`: its manager (+0xd3) and its five coaches (+0x19f), each when
/// present, with a non-player record (`staff::NON_PLAYER`) and not the player himself; returns the count and the list.
pub(crate) unsafe fn coach_report_candidates(club: usize, player: usize) -> (i8, [usize; 6]) {
    let (mut n, mut list) = (0i8, [0usize; 6]);
    for at in core::iter::once(club + 0xd3).chain((0..5).map(|k| club + 0x19f + k * 4)) {
        let c = Staff(read_usize(at));
        if !c.is_null() && c.non_player().addr() != 0 && c.addr() != player {
            list[n as usize] = c.addr();
            n += 1;
        }
    }
    (n, list)
}

/// The player (parameter 0, a staff id) and the club (parameter 2, a club id) of the screen.
pub(crate) unsafe fn coach_report_player() -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add(get_screen_param(0).wrapping_mul(staff::SIZE))
}

pub(crate) unsafe fn coach_report_club() -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add(get_screen_param(2).wrapping_mul(club::SIZE))
}

/// The screen's build and input functions (passed to `ScreenManager::open_screen`) and the coach report owner
/// (0xadadf4, ecx of `coach_post_report_news`).
pub(crate) const FN_COACH_REPORT_REQUEST_DRAW: usize = 0x48bd20;

pub(crate) const FN_COACH_REPORT_REQUEST_HANDLE_ACTION: usize = 0x48c290;

pub(crate) const COACH_REPORTS: usize = 0xadadf4;

/// The texts "Request Coach Report", "Select All", "  Ask <%s - Coach Name...> to file a report on <%s - Player
/// name...> ?" and "  Specify the coach you want to file a report on <%s - Player name...>:".
pub(crate) const COACH_REPORT_TITLE: usize = 0x99b4d4;

pub(crate) const COACH_REPORT_SELECT_ALL: usize = 0x99b4c8;

pub(crate) const COACH_REPORT_ASK_ONE: usize = 0x99b460;

pub(crate) const COACH_REPORT_ASK_WHICH: usize = 0x99b408;

/// The item ids: a coach (its value is its bit), Cancel, Confirm, Select All.
pub(crate) const COACH_REPORT_ITEM_COACH: u16 = 1;

pub(crate) const COACH_REPORT_ITEM_CANCEL: u16 = 2;

pub(crate) const COACH_REPORT_ITEM_CONFIRM: u16 = 3;

pub(crate) const COACH_REPORT_ITEM_SELECT_ALL: u16 = 4;

/// "Cancel" and "Confirm" (0x98a1a0, 0x98a198).
pub(crate) const COACH_REPORT_CANCEL: usize = 0x98a1a0;

pub(crate) const COACH_REPORT_CONFIRM: usize = 0x98a198;

/// u32: the game is a network game.
pub(crate) const COACH_REPORT_NETWORK_GAME: usize = 0xb77c84;
