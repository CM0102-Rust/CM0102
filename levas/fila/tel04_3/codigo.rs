// ---------- batch %%LEVA%%: the tournament and friendly invitations, the squad numbers openers (club_screens.cpp, part 4) ----------

/// A NewsItem (0xe0 bytes; room to spare) made by its constructor (0x77bbe0, thiscall(type, important flag)).
unsafe fn club_pages_news_item(item: &mut [u32; 0x40], kind: u32, important: u32) -> usize {
    let make: unsafe extern "thiscall" fn(*mut u32, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    make(item.as_mut_ptr(), kind, important);
    item.as_mut_ptr() as usize
}

/// `News::find_item_by_id(manager, id, out)` (0x786b70, thiscall on [0xb7431c]): nonzero when found.
unsafe fn club_pages_find_news(manager: usize, id: usize, item: usize) -> u32 {
    let find: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x786b70usize);
    find(read_usize(G_NEWS), manager, id, item)
}

/// `News::set_item_slot(manager, id, slot, value, copy)` (0x786d30, thiscall on [0xb7431c]).
unsafe fn club_pages_set_news_slot(manager: usize, id: usize, slot: u32, value: u32) {
    let set: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_NEWS_SET_ITEM_SLOT);
    set(read_usize(G_NEWS), manager, id, slot, value, 0);
}

/// Club `k` of the club table (0 for a negative index).
unsafe fn club_pages_club_at(k: i32) -> usize {
    if k < 0 { 0 } else { read_usize(G_CLUB_TABLE) + k as usize * club::SIZE }
}

/// The Back (id 1), No (id 2) and Yes (id 3) buttons of an invitation.
unsafe fn invitation_answer_buttons() {
    let lang = club_pages_lang();
    let weights: [u8; 3] = [2, 1, 1];
    let (grey, pale) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY));
    let area = club_pages_area([100, 0x22b, 0x316, 0x24e, 3, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    for (col, text) in [(0u32, 0x9880a8usize), (1, 0x98e8cc), (2, 0x98e8d0)] {
        lang(JOB_TEXT, text); // "Back", "No", "Yes"
        tel26_area_button([area, col, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, col + 1, 0]);
    }
}

/// Whether the current slot's human manages `club`.
unsafe fn tournament_hosted_here(t: usize) -> bool {
    read_u32(read_usize(t) + club::MANAGER) == read_u32((G_SCREEN_RECORDS_MANAGER as isize + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize * 0x300) as usize)
}

/// Opens the invited clubs screen of tournament `t` (builder `tournament_invitations_screen_build`, handler
/// `tournament_invitations_screen_handle_event`; a new history entry only for the host); no tournament shows the
/// club_screens.cpp 13860 box. The host is the current human when he manages the tournament's club. Params: 0 the
/// tournament, 1 the host flag; for the host the tournament marks the invite session (+0x29 = 1, +0x2d = `arg`).
/// Original: `open_tournament_invitations_screen` (0x4739d0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_tournament_invitations_screen(t: usize, arg: u32) {
    if t == 0 {
        club_pages_error_box(0x3624);
        return;
    }
    let host = tournament_hosted_here(t);
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x473ad0usize, 0x473fb0usize, host as u32, 0usize, 0) == 0 {
        return;
    }
    covered!("tel04_3 open the invitations");
    set_screen_param(0, t);
    set_screen_param(1, host as usize);
    if host {
        write_u32(t + 0x29, 1);
        write_u32(t + 0x2d, arg);
    }
}

/// Builds the invited clubs screen of a tournament (param 0; param 1 the host flag).
///
/// The host club's background picture, the tournament's name in its kit colours, "Invited Clubs" (or "Invited
/// Nations" for a national team) and the menu bar; the host gets Invite Another (id 1, plain text when every slot is
/// taken). One row per taken slot: the club (yellow for a human's) with, for a club, its national team and its division
/// (linked when it has a competition object), for a national team its continent; then "Hosts" for the first slot,
/// "Accepted Invitation" (status 3) or "Waiting For Reply". The host has Back (id 2) and Next (−3, text without a next
/// screen); the others the Back and Next buttons.
/// Original: `tournament_invitations_screen_build` (0x473ad0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn tournament_invitations_screen_build() {
    let lang = club_pages_lang();
    let format = club_pages_format();
    let row_weights: [u8; 4] = [5, 3, 3, 3];
    let button_weights: [u8; 2] = [3, 1];
    let t = get_screen_param(0);
    let host = get_screen_param(1);
    let club = read_usize(t);
    covered!("tel04_3 invitations build");
    exe::background_set_club_picture(club, 0, 0);
    club_pages_title(club, t + 0x31);
    lang(JOB_TEXT, if exe::database_is_national_team(club) != 0 { 0x990fe8usize } else { 0x990fd8 }); // Invited Nations / Clubs
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    exe::menubar_build(4, 0);
    let (grey, pale, yellow) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY), job_colour(G_GUI_COLOUR_YELLOW));
    let slots = |t: usize| read_i16(t + 0x13);
    let slot = |t: usize, k: i8| read_usize(t + 0x15 + k as isize as usize * 4);
    let mut taken: i8 = 0;
    let mut k: i8 = 0;
    while (k as i16) < slots(t) {
        if slot(t, k) != 0 {
            taken = taken.wrapping_add(1);
        }
        k = k.wrapping_add(1);
    }
    let mut top = 0x91;
    if host != 0 {
        lang(JOB_TEXT, 0x990fc8); // "Invite Another"
        if (taken as i16) < slots(t) {
            tel26_add_button([0x28f, 0x91, 0x30c, 0xa5, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 1, 0]);
        } else {
            let text_item: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
            text_item(G_SCREEN_MANAGER, 0x28f, 0x91, 0x30c, 0xa5, 0x30, grey, 0x2c, 1, pale, JOB_TEXT as u32, 1);
        }
        top = 0xaa;
    }
    let rows = ((0x207 - top) / exe::font_get_height(3)) as u8 as u32;
    let area = club_pages_area([0x6e, top as u32, 0x30c, 0x217, 4, row_weights.as_ptr() as u32, rows, 0, 2, 0, u32::MAX]);
    let mut row = 0u32;
    let mut k: i8 = 0;
    while (k as i16) < slots(t) {
        let c = slot(t, k);
        if c != 0 {
            format(JOB_TEXT, 0x987ff0, c + club::SHORT_NAME); // "  %s"
            let colour = if exe::human_manager_is_human_club(c, 0, 1, 0, 0) != 0 { yellow } else { pale };
            tel26_area_button([area, 0, row, 1, 0, 1, 3, colour, JOB_TEXT as u32, 0x7d5, c as u32]);
            let n = read_usize(c + club::NATION);
            if exe::database_is_national_team(c) != 0 {
                tel26_area_text([area, 1, row, 1, 0, 1, 2, yellow, (read_usize(n + nation::CONTINENT) + 4) as u32]);
            } else {
                let team = exe::database_get_national_team(n);
                tel26_area_button([area, 1, row, 1, 0, 1, 2, yellow, (n + nation::SHORT_NAME) as u32, 0x7d5, team as u32]);
                let division = read_usize(c + club::DIVISION);
                if division != 0 {
                    if read_u32(read_usize(G_COMP_OBJECT_TABLE) + read_u32(division) as usize * 4) != 0 {
                        tel26_area_button([area, 2, row, 1, 0, 1, 2, job_colour(0xaebdd4), (division + 0x38) as u32, 0x7d0, division as u32]);
                    } else {
                        tel26_area_text([area, 2, row, 1, 0, 1, 2, job_colour(0xaebdd4), (division + 0x38) as u32]);
                    }
                }
            }
            let status = if k == 0 {
                0x990fc0usize // "Hosts"
            } else if read_u8(t + 0x25 + k as isize as usize) == 3 {
                0x990fac // "Accepted Invitation"
            } else {
                0x990f98 // "Waiting For Reply"
            };
            lang(JOB_TEXT, status);
            tel26_area_text([area, 3, row, 1, 0, 1, 2, grey, JOB_TEXT as u32]); // tel04_3 the reply column
            row += 1;
        }
        k = k.wrapping_add(1);
    }
    if host == 0 {
        tel26_back_next();
        return;
    }
    let buttons = club_pages_area([100, 0x22b, 0x316, 0x24e, 2, button_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    lang(JOB_TEXT, 0x9880a8); // "Back"
    tel26_area_button([buttons, 0, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 2, 0]);
    lang(JOB_TEXT, 0x9880a0); // "Next"
    if exe::screenmanager_is_forward_unavailable(G_SCREEN_MANAGER) != 0 {
        tel26_area_text([buttons, 1, 0, 0x30, grey, 0x2c, 3, pale, JOB_TEXT as u32]);
    } else {
        tel26_area_button([buttons, 1, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 0xffff_fffd, 0]);
    }
}

/// Answers a click on the invited clubs screen: id 2 (Back) ends the invite session (+0x29 and +0x2d cleared, −9);
/// id 1 (Invite Another) opens the opponent choice (by continent for a national team, by nation for a club; −4);
/// anything else goes to the menu bar.
/// Original: `tournament_invitations_screen_handle_event` (0x473fb0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn tournament_invitations_screen_handle_event(widget: u32) -> i32 {
    let t = get_screen_param(0);
    let club = read_usize(t);
    match club_pages_id(widget) {
        2 => {
            write_u32(t + 0x29, 0); // tel04_3 the invite session
            write_u32(t + 0x2d, 0);
            -9
        }
        1 => {
            let national = exe::database_is_national_team(club);
            let n = read_usize(club + club::NATION);
            let opponents: unsafe extern "C" fn(u32, usize, usize, u32) = core::mem::transmute(0x5a0650usize);
            if national != 0 {
                opponents(1, read_usize(n + nation::CONTINENT), n, 2);
            } else {
                opponents(3, read_usize(n + nation::CONTINENT), n, 1);
            }
            -4
        }
        _ => club_pages_menu_bar(widget),
    }
}

/// The friendly match a friendly invitation news item points at: slot 2 the club index, slots 3 and 4 the day and year
/// (`Friendly::find_friendly_match`); `None` when the item is not found.
unsafe fn friendly_invitation_match(manager: usize, id: usize) -> Option<usize> {
    let mut item = [0u32; 0x40];
    let it = club_pages_news_item(&mut item, 0, 0);
    if club_pages_find_news(manager, id, it) == 0 {
        return None;
    }
    let c = exe::newsitem_get_slot(it, 2) as i32;
    let day = exe::newsitem_get_slot(it, 3);
    let year = exe::newsitem_get_slot(it, 4);
    let club = club_pages_club_at(c);
    let mut date = [0u32; 2];
    let d = club_pages_date(date.as_mut_ptr(), day as u16, year as u16);
    Some(exe::friendly_find_friendly_match(read_usize(G_FRIENDLY), d, club))
}

/// Opens the friendly invitation of a news item (`manager`, `id`; not found: the club_screens.cpp 14121 box): builder
/// `friendly_invitation_screen_build`, handler `friendly_invitation_screen_handle_event`. Params: 0 and 1 the news item,
/// 2 the friendly match (`friendly_invitation_match`).
/// Original: `open_friendly_invitation_screen` (0x4740c0, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn open_friendly_invitation_screen(manager: usize, id: usize) {
    let Some(fixture) = friendly_invitation_match(manager, id) else {
        club_pages_error_box(0x3729);
        return;
    };
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x474280usize, 0x4744b0usize, 1, 0usize, 0) != 0 {
        covered!("tel04_3 open the friendly invitation");
        set_screen_param(0, manager);
        set_screen_param(1, id);
        set_screen_param(2, fixture);
    }
}

/// Builds the friendly invitation (param 2, the friendly match): the background picture and kit colours of the invited
/// club (the one that is not the inviter, +0xc), "Friendly Match" and "Invitation", the menu bar, the question "Accept
/// the invitation to a friendly match with <inviter> on <date> ?" (the inviter linked) and Back, No and Yes.
/// Original: `friendly_invitation_screen_build` (0x474280, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn friendly_invitation_screen_build() {
    let lang = club_pages_lang();
    let fixture = get_screen_param(2);
    let inviter = read_usize(fixture + 0xc);
    let mut club = read_usize(fixture + 0x10);
    if inviter == club {
        club = read_usize(fixture + 0x14);
    }
    covered!("tel04_3 friendly invitation build");
    exe::background_set_club_picture(club, 0, 0);
    lang(JOB_TEXT, 0x991058); // "Friendly Match"
    club_pages_title(club, JOB_TEXT);
    lang(JOB_TEXT, 0x99104c); // "Invitation"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    exe::menubar_build(4, 0);
    let mut date = [0u8; 0xc8];
    exe::date_get_long_date_string(fixture, date.as_mut_ptr() as usize, 0xc8, 0xff);
    let c = read_i8(inviter + club::SHORT_GENDER_NAME) as i32;
    lang(JOB_TEXT, 0x990ff8, c, c, 100, inviter + club::SHORT_NAME, date.as_ptr()); // "Accept the invitation to a friendly match with ..."
    let area: unsafe extern "C" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) = core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    area(0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, (inviter + club::SHORT_NAME) as u32, 0x7d5, inviter as u32, 0);
    invitation_answer_buttons();
}

/// Answers the friendly invitation (params 0 and 1 the news item, 2 the friendly match). Back: −9. No or Yes: the
/// match's reply (+0x18) is 4 or 3; the news item gets slot 5 the reply and slot 6 cleared; a reply news item (type 7:
/// the home and away clubs' ids, the inviter's, the day, the year and the reply) goes to the inviter's manager and the
/// friendlies process the reply (−9). Anything else goes to the menu bar.
/// Original: `friendly_invitation_screen_handle_event` (0x4744b0, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn friendly_invitation_screen_handle_event(widget: u32) -> i32 {
    let manager = get_screen_param(0);
    let id = get_screen_param(1);
    let fixture = get_screen_param(2);
    let answer = club_pages_id(widget);
    match answer {
        1 => -9,
        2 | 3 => {
            write_u8(fixture + 0x18, if answer == 3 { 3 } else { 4 }); // tel04_3 the friendly reply
            let reply = read_i8(fixture + 0x18) as i32 as u32;
            club_pages_set_news_slot(manager, id, 5, reply);
            club_pages_set_news_slot(manager, id, 6, 0);
            let mut item = [0u32; 0x40];
            let it = club_pages_news_item(&mut item, 7, 0);
            exe::newsitem_set_slot(it, 0, read_u32(read_usize(fixture + 0x10)));
            exe::newsitem_set_slot(it, 1, read_u32(read_usize(fixture + 0x14)));
            exe::newsitem_set_slot(it, 2, read_u32(read_usize(fixture + 0xc)));
            exe::newsitem_set_slot(it, 3, read_i16(fixture) as i32 as u32);
            exe::newsitem_set_slot(it, 4, read_u16(fixture + 2) as u32);
            exe::newsitem_set_slot(it, 5, read_i8(fixture + 0x18) as i32 as u32);
            exe::news_send_to_club_manager(read_usize(G_NEWS), it, read_usize(fixture + 0xc));
            exe::friendly_process_friendly_reply(read_usize(G_FRIENDLY), fixture, 0);
            -9
        }
        _ => club_pages_menu_bar(widget),
    }
}

/// The tournament and the invited club of a tournament invitation news item (slot 2 the tournament index,
/// `Friendly::get_tournament` without a date; slot 1 the club index); `None` when the item is not found.
unsafe fn tournament_invitation_target(manager: usize, id: usize) -> Option<(usize, usize)> {
    let mut item = [0u32; 0x40];
    let it = club_pages_news_item(&mut item, 0, 0);
    if club_pages_find_news(manager, id, it) == 0 {
        return None;
    }
    let c = exe::newsitem_get_slot(it, 1) as i32;
    let index = exe::newsitem_get_slot(it, 2) as u8;
    let get: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(0x5c89a0usize);
    let t = get(read_usize(G_FRIENDLY), index as u32, 0);
    Some((t, club_pages_club_at(c)))
}

/// Opens the tournament invitation of a news item (`manager`, `id`; not found: the club_screens.cpp 14287 box): builder
/// `tournament_invitation_screen_build`, handler `tournament_invitation_screen_handle_event`. Params: 0 and 1 the news
/// item, 2 the tournament, 3 the invited club (`tournament_invitation_target`).
/// Original: `open_tournament_invitation_screen` (0x474720, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn open_tournament_invitation_screen(manager: usize, id: usize) {
    let Some((t, club)) = tournament_invitation_target(manager, id) else {
        club_pages_error_box(0x37cf);
        return;
    };
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x4748a0usize, 0x474ac0usize, 1, 0usize, 0) != 0 {
        covered!("tel04_3 open the tournament invitation");
        for (k, v) in [(0u32, manager), (1, id), (2, t), (3, club)] {
            set_screen_param(k, v);
        }
    }
}

/// Builds the tournament invitation (param 2 the tournament, 3 the invited club). A tournament without a host only
/// clears the background picture. Otherwise the club's background picture, the tournament's name in its kit colours,
/// "Invitation", the menu bar, the question "Accept the invitation to the <tournament> tournament starting on <date> ?"
/// and Back, No and Yes.
/// Original: `tournament_invitation_screen_build` (0x4748a0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn tournament_invitation_screen_build() {
    let lang = club_pages_lang();
    let t = get_screen_param(2);
    let club = get_screen_param(3);
    if read_usize(t) == 0 {
        exe::background_set_picture(0usize);
        return;
    }
    covered!("tel04_3 tournament invitation build");
    exe::background_set_club_picture(club, 0, 0);
    club_pages_title(club, t + 0x31);
    lang(JOB_TEXT, 0x99104c); // "Invitation"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    exe::menubar_build(4, 0);
    let mut date = [0u8; 0xc8];
    exe::date_get_long_date_string(t + 4, date.as_mut_ptr() as usize, 0xc8, 0xff);
    lang(JOB_TEXT, 0x991068, 100, t + 0x31, date.as_ptr()); // "Accept the invitation to the <tournament> tournament ..."
    tel26_text_area([0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 0]);
    invitation_answer_buttons();
}

/// Answers the tournament invitation (params 0 and 1 the news item, 2 the tournament, 3 the invited club). A tournament
/// without a host, or Back: −9. No or Yes: the club's invite slot gets status 3 (Yes) or is freed (No: status 0, slot
/// cleared); the news item gets slot 3 the reply (3 or 4); a reply news item (type 10: the host's id, the club's, the
/// start day and year, the reply and the tournament's +0x64) goes to the host's manager and the friendlies process the
/// replies (−9). Anything else goes to the menu bar.
/// Original: `tournament_invitation_screen_handle_event` (0x474ac0, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn tournament_invitation_screen_handle_event(widget: u32) -> i32 {
    let manager = get_screen_param(0);
    let id = get_screen_param(1);
    let t = get_screen_param(2);
    let club = get_screen_param(3);
    if read_usize(t) == 0 {
        return -9;
    }
    let answer = club_pages_id(widget);
    match answer {
        1 => -9,
        2 | 3 => {
            let mut k: i8 = 0;
            while (k as i16) < read_i16(t + 0x13) {
                let slot = t + 0x15 + k as isize as usize * 4;
                if read_usize(slot) == club {
                    if answer == 3 {
                        write_u8(t + 0x25 + k as isize as usize, 3);
                    } else {
                        write_u8(t + 0x25 + k as isize as usize, 0);
                        write_u32(slot, 0);
                    }
                    break;
                }
                k = k.wrapping_add(1);
            }
            let reply = if answer == 3 { 3 } else { 4 };
            club_pages_set_news_slot(manager, id, 3, reply);
            let mut item = [0u32; 0x40];
            let it = club_pages_news_item(&mut item, 10, 0);
            exe::newsitem_set_slot(it, 0, read_u32(read_usize(t)));
            exe::newsitem_set_slot(it, 1, read_u32(club));
            exe::newsitem_set_slot(it, 2, read_i16(t + 4) as i32 as u32);
            exe::newsitem_set_slot(it, 3, read_u16(t + 6) as u32);
            exe::newsitem_set_slot(it, 4, reply); // tel04_3 the tournament reply
            exe::newsitem_set_slot(it, 5, read_i8(t + 0x64) as i32 as u32);
            exe::news_send_to_club_manager(read_usize(G_NEWS), it, read_usize(t));
            exe::friendly_process_tournament_replies(read_usize(G_FRIENDLY), t, 0, 0);
            -9
        }
        _ => club_pages_menu_bar(widget),
    }
}

/// The 19 params of the squad numbers screen as its openers write them: 0 and 1 the given pair, 2 the screen kind, 3
/// its flag, 4 the view 0x41, 5 the sort 8, 6 the second sort 6, 7 and 8 0, 9 and 10 −1, 0xb..0xd the three bytes,
/// 0xe..0x11 0, 0x12 the match fixture.
fn squad_numbers_plan(p0: usize, p1: usize, kind: usize, flag: usize, bytes: [usize; 3], fixture: usize) -> [usize; 0x13] {
    [p0, p1, kind, flag, 0x41, 8, 6, 0, 0, usize::MAX, usize::MAX, bytes[0], bytes[1], bytes[2], 0, 0, 0, 0, fixture]
}

/// Opens the squad numbers screen (builder 0x475520, handler 0x47a9e0, leave 0x47c070) with its 19 params (made
/// once the screen is open).
unsafe fn open_squad_numbers_with(plan: impl FnOnce() -> [usize; 0x13]) -> bool {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x475520usize, 0x47a9e0usize, 1, 0x47c070usize, 0) == 0 {
        return false;
    }
    for (k, v) in plan().iter().enumerate() {
        set_screen_param(k as u32, *v);
    }
    true
}

/// Opens the squad numbers screen before a match, when it is needed: the club of the match screen context (+0xe6) in
/// its fixture (*+0xe2) has the squad flag set (+0x40 at home, +0x41 away) and uses squad numbers
/// (`SQUAD_MANAGER::club_uses_squad_numbers`). Each selected player (slots 0 to 10 + the fixture's substitutes, +0x4d &
/// 0xf) who is in the database and still belongs to the club (for a national team his national team, +0x24; for a club
/// his club, +0x39, unless there are no national teams or the club is a national or all-star team) and has a squad
/// number (not negative) opens the screen (params: the slot's manager, −1, 1, 1, ..., the fixture) and returns 1; a
/// player who no longer belongs is taken out of the selection. Otherwise 0.
/// Original: `open_squad_numbers_screen_for_match` (0x474e00, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_squad_numbers_screen_for_match(context: usize) -> u32 {
    let club = read_usize(context + 0xe6);
    let fixture = read_usize(read_usize(context + 0xe2));
    let flag = if club == read_usize(fixture + screen_fixture::HOME) { read_u8(fixture + 0x40) } else { read_u8(fixture + 0x41) };
    if flag == 0 || exe::squad_manager_club_uses_squad_numbers(read_usize(G_SQUAD_MANAGER), club, 0) == 0 {
        return 0;
    }
    let mut k: i8 = 0;
    while (k as i32) < ((read_u8(fixture + 0x4d) & 0xf) as i32 + 0xb) {
        // the slot goes in the high bytes of the original's argument slot (the context pointer): the same dword here
        let slot = (context as u32 & 0xffff_ff00) | k as u8 as u32;
        let p = exe::tactics_get_selected_player(read_usize(G_TACTICS), club, slot, 0);
        if p != 0 && read_i32(p) < read_i32(G_STAFF_COUNT) {
            let belongs_to = if exe::database_is_national_team(club) != 0 {
                Some(read_usize(p + 0x24))
            } else if read_usize(G_NATIONAL_TEAMS) != 0 && exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), club) == 0 {
                Some(read_usize(p + 0x39))
            } else {
                None
            };
            if belongs_to.is_some_and(|c| c != club) {
                exe::tactics_clear_selection_slot(read_usize(G_TACTICS), club, slot, 0);
            } else if exe::squad_manager_get_squad_number(read_usize(G_SQUAD_MANAGER), p, club) >= 0 {
                covered!("tel04_3 squad numbers before the match");
                return open_squad_numbers_with(|| {
                    let manager = read_usize((G_SCREEN_RECORDS_MANAGER as isize + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize * 0x300) as usize);
                    squad_numbers_plan(manager, usize::MAX, 1, 1, [0, 0, 0], fixture)
                }) as u32;
            }
        }
        k = k.wrapping_add(1);
    }
    0
}

/// Opens the squad numbers screen of a news item (`manager`, `id`; not found: the club_screens.cpp 14580 box): kind 1,
/// flag set when the item's type is 0x18.
/// Original: `open_squad_numbers_screen` (0x475070, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn open_squad_numbers_screen(manager: usize, id: usize) {
    let mut item = [0u32; 0x40];
    let it = club_pages_news_item(&mut item, 0, 0);
    if club_pages_find_news(manager, id, it) == 0 {
        club_pages_error_box(0x38f4);
        return;
    }
    covered!("tel04_3 squad numbers from the news");
    let flag = (item[0] == 0x18) as usize;
    open_squad_numbers_with(|| squad_numbers_plan(manager, id, 1, flag, [0, 0, 0], 0));
}

/// Opens the European squad screen of a news item (`manager`, `id`; not found: the club_screens.cpp 14633 box): kind 2,
/// the item's slots 2, 4 and 5 (bytes) in params 0xb..0xd.
/// Original: `open_european_squad_screen` (0x4752b0, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn open_european_squad_screen(manager: usize, id: usize) {
    let mut item = [0u32; 0x40];
    let it = club_pages_news_item(&mut item, 0, 0);
    if club_pages_find_news(manager, id, it) == 0 {
        club_pages_error_box(0x3929);
        return;
    }
    covered!("tel04_3 european squad");
    let bytes = [2u32, 4, 5].map(|s| exe::newsitem_get_slot(it, s) as u8 as i8 as isize as usize);
    open_squad_numbers_with(|| squad_numbers_plan(manager, id, 2, 0, bytes, 0));
}

/// Comparer for `open_tournament_invitations_screen`: only the original runs; the params and the invite session marks
/// checked against the plan.
pub(crate) unsafe extern "C" fn compare_open_tournament_invitations_screen(t: usize, arg: u32) {
    let host = t != 0 && tournament_hosted_here(t);
    original_fn::<unsafe extern "C" fn(usize, u32)>(%%I0%%)(t, arg);
    if t != 0 && get_screen_param(0) == t {
        let ok = get_screen_param(1) == host as usize && (!host || (read_u32(t + 0x29) == 1 && read_u32(t + 0x2d) == arg));
        SCOREBOARD.record(ok, || format!("tournament invitations screen (open {:#x}): host {} params {:x?}", t, host, club_pages_params(2)));
    }
}

/// Comparer for `tournament_invitations_screen_build`: side by side by the display list.
pub(crate) unsafe extern "C" fn compare_tournament_invitations_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    compare_match_screen_build("tournament invitations screen", &[], || o(), || tournament_invitations_screen_build());
}

/// Comparer for `tournament_invitations_screen_handle_event`: Back side by side over the params and the invite session
/// marks; the opponent choice and the menu bar only the original.
pub(crate) unsafe extern "C" fn compare_tournament_invitations_screen_handle_event(widget: u32) -> i32 {
    let t = get_screen_param(0);
    let both = t != 0 && club_pages_id(widget) == 2;
    club_pages_compare_input("tournament invitations screen input", widget, original_fn(%%I2%%), tournament_invitations_screen_handle_event, both, &[(t + 0x29, 8)])
}

/// Comparer for `open_friendly_invitation_screen`: only the original runs; the params checked against the plan.
pub(crate) unsafe extern "C" fn compare_open_friendly_invitation_screen(manager: usize, id: usize) {
    let plan = friendly_invitation_match(manager, id);
    original_fn::<unsafe extern "C" fn(usize, usize)>(%%I3%%)(manager, id);
    if let Some(fixture) = plan {
        if get_screen_param(0) == manager && get_screen_param(1) == id {
            let got = club_pages_params(3);
            SCOREBOARD.record(got == [manager, id, fixture], || format!("friendly invitation screen (open): params {:x?} x match {:#x}", got, fixture));
        }
    }
}

/// Comparer for `friendly_invitation_screen_build`: side by side by the display list.
pub(crate) unsafe extern "C" fn compare_friendly_invitation_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I4%%);
    compare_match_screen_build("friendly invitation screen", &[], || o(), || friendly_invitation_screen_build());
}

/// Comparer for `friendly_invitation_screen_handle_event`: Back side by side; the reply (news and the friendlies'
/// records) and the menu bar only the original.
pub(crate) unsafe extern "C" fn compare_friendly_invitation_screen_handle_event(widget: u32) -> i32 {
    let both = club_pages_id(widget) == 1;
    club_pages_compare_input("friendly invitation screen input", widget, original_fn(%%I5%%), friendly_invitation_screen_handle_event, both, &[])
}

/// Comparer for `open_tournament_invitation_screen`: only the original runs; the params checked against the plan.
pub(crate) unsafe extern "C" fn compare_open_tournament_invitation_screen(manager: usize, id: usize) {
    let plan = tournament_invitation_target(manager, id);
    original_fn::<unsafe extern "C" fn(usize, usize)>(%%I6%%)(manager, id);
    if let Some((t, club)) = plan {
        if get_screen_param(0) == manager && get_screen_param(1) == id {
            let got = club_pages_params(4);
            SCOREBOARD.record(got == [manager, id, t, club], || format!("tournament invitation screen (open): params {:x?} x {:#x} {:#x}", got, t, club));
        }
    }
}

/// Comparer for `tournament_invitation_screen_build`: side by side by the display list.
pub(crate) unsafe extern "C" fn compare_tournament_invitation_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I7%%);
    compare_match_screen_build("tournament invitation screen", &[], || o(), || tournament_invitation_screen_build());
}

/// Comparer for `tournament_invitation_screen_handle_event`: Back (and a tournament without a host) side by side; the
/// reply and the menu bar only the original.
pub(crate) unsafe extern "C" fn compare_tournament_invitation_screen_handle_event(widget: u32) -> i32 {
    let t = get_screen_param(2);
    let both = t != 0 && (read_usize(t) == 0 || club_pages_id(widget) == 1);
    club_pages_compare_input("tournament invitation screen input", widget, original_fn(%%I8%%), tournament_invitation_screen_handle_event, both, &[])
}

/// Comparer for `open_squad_numbers_screen_for_match`: only the original runs (it opens a screen and edits the
/// selection); when it opened the screen, its params are checked against the plan.
pub(crate) unsafe extern "C" fn compare_open_squad_numbers_screen_for_match(context: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I9%%)(context);
    if r == 1 {
        let manager = read_usize((G_SCREEN_RECORDS_MANAGER as isize + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize * 0x300) as usize);
        let plan = squad_numbers_plan(manager, usize::MAX, 1, 1, [0, 0, 0], read_usize(read_usize(context + 0xe2)));
        let got = club_pages_params(0x13);
        SCOREBOARD.record(got == plan, || format!("squad numbers screen (open for the match): params {:x?} x plan {:x?}", got, plan));
    }
    r
}

/// Comparer for `open_squad_numbers_screen`: only the original runs; the params checked against the plan.
pub(crate) unsafe extern "C" fn compare_open_squad_numbers_screen(manager: usize, id: usize) {
    let mut item = [0u32; 0x40];
    let it = club_pages_news_item(&mut item, 0, 0);
    let found = club_pages_find_news(manager, id, it) != 0;
    original_fn::<unsafe extern "C" fn(usize, usize)>(%%I10%%)(manager, id);
    if found && get_screen_param(0) == manager && get_screen_param(1) == id {
        let plan = squad_numbers_plan(manager, id, 1, (item[0] == 0x18) as usize, [0, 0, 0], 0);
        let got = club_pages_params(0x13);
        SCOREBOARD.record(got == plan, || format!("squad numbers screen (open): params {:x?} x plan {:x?}", got, plan));
    }
}

/// Comparer for `open_european_squad_screen`: only the original runs; the params checked against the plan.
pub(crate) unsafe extern "C" fn compare_open_european_squad_screen(manager: usize, id: usize) {
    let mut item = [0u32; 0x40];
    let it = club_pages_news_item(&mut item, 0, 0);
    let found = club_pages_find_news(manager, id, it) != 0;
    let bytes = [2u32, 4, 5].map(|s| if found { exe::newsitem_get_slot(it, s) as u8 as i8 as isize as usize } else { 0 });
    original_fn::<unsafe extern "C" fn(usize, usize)>(%%I11%%)(manager, id);
    if found && get_screen_param(0) == manager && get_screen_param(1) == id {
        let plan = squad_numbers_plan(manager, id, 2, 0, bytes, 0);
        let got = club_pages_params(0x13);
        SCOREBOARD.record(got == plan, || format!("european squad screen (open): params {:x?} x plan {:x?}", got, plan));
    }
}
