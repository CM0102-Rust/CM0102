// ---------- batch %%LEVA%%: the club screen's next match and fixtures tabs input and its leave (club_screens.cpp, part 3) ----------

/// The shared text buffer the screens format into (0xde1f64).
const CLUB_TABS_TEXT: usize = 0xde1f64;
/// The screen manager's "a dialog has answered" flag (0xb77c84, screen manager +0x3834) [inference]: with it set,
/// `G_PENDING_COMMAND_CODE` names the dialog and `G_PENDING_COMMAND_ARG` holds the answer (2 = yes).
const CLUB_TABS_DIALOG_ANSWERED: usize = 0xb77c84;

/// The action id of widget `w` on the current human's Display (the item id read inline), or the injected id
/// (`G_CURRENT_COMMAND_ID`) when it has none or for widget −1.
unsafe fn club_tabs_action(w: i16) -> u16 {
    let id = if w == -1 { 0 } else { read_u16((current_human_display() as isize + w as isize * 0x18c + display::ITEM_ID as isize) as usize) };
    if id == 0 {
        read_u16(G_CURRENT_COMMAND_ID)
    } else {
        id
    }
}

/// The value attached to widget `w` on the current human's Display (read inline), or 0 for widget −1.
unsafe fn club_tabs_value(w: i16) -> usize {
    if w == -1 {
        0
    } else {
        read_usize((current_human_display() as isize + w as isize * 0x18c + display::ITEM_VALUE as isize) as usize)
    }
}

/// The fixture the Previous (`forward` false) or Next (`forward` true) button of the next match tab moves to, from the
/// shown fixture `fixture`: `FIXTURE_MANAGER::find_fixtures` of the club (any competition, type −2, the side "not
/// reserves") from the day after the fixture to 24 weeks on (8 for a club that is not a national team), or from 24 (8)
/// weeks before to the day before; Next takes the first found, Previous the last (the list walked from its tail).
/// Returns a copy of its 0x4f bytes, or None.
unsafe fn club_next_match_neighbour(club: usize, reserves: usize, fixture: usize, forward: bool) -> Option<[u8; 0x4f]> {
    type DateStep = unsafe extern "thiscall" fn(*const [u8; 8], *mut [u8; 8], u32) -> *const [u8; 8];
    let date: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let sub_days: DateStep = core::mem::transmute(0x54c8e0usize);
    let sub_weeks: DateStep = core::mem::transmute(0x54cab0usize);
    let add_weeks: DateStep = core::mem::transmute(0x54c770usize);
    let add_days: DateStep = core::mem::transmute(0x54c5b0usize);
    let mut list = [0u8; 0xe];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    let mut d = [0u8; 8];
    date(&mut d, read_u16(fixture + fixture::DAY) as u32, read_u16(fixture + fixture::YEAR) as u32);
    let weeks = if exe::database_is_national_team(club) != 0 { 0x18 } else { 8 }; // tel03_4 the search window
    // each date is worked out twice, once for its year and once for its day, as in the original
    let step = |f: DateStep, n: u32| -> (u32, u32) {
        let (mut a, mut b) = ([0u8; 8], [0u8; 8]);
        let year = read_u16(f(&d, &mut a, n) as usize + 2) as u32;
        let day = read_u16(f(&d, &mut b, n) as usize) as u32;
        (day, year)
    };
    let ((start_day, start_year), (end_day, end_year)) = if forward {
        (step(add_days, 1), step(add_weeks, weeks))
    } else {
        (step(sub_weeks, weeks), step(sub_days, 1))
    };
    exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), start_day, start_year, end_day, end_year, read_u32(club), l, u32::MAX, (-2i32) as u32, (reserves == 0) as u32);
    let found = if forward {
        write_u32(l + llist::CURSOR, read_u32(l + llist::HEAD));
        exe::llist_get_next(l)
    } else {
        write_u32(l + llist::CURSOR, read_u32(l + llist::TAIL));
        exe::llist_get_prev(l)
    };
    let copy = if found == 0 {
        None
    } else {
        let mut c = [0u8; 0x4f];
        for (k, b) in c.iter_mut().enumerate() {
            *b = read_u8(found + k);
        }
        Some(c)
    };
    exe::drop_llist(l);
    copy
}

/// Answers a click on the club screen's next match tab (param 0 the club, 1 the reserves flag, 3 the shown fixture, a
/// copy the screen owns): action 0x1d opens the league progress of the fixture (−4); 0x1e the past meetings with the
/// widget's club, or with the fixture's other club (−4); 0x26 and 0x27 move to the previous and the next fixture of the
/// club (`club_next_match_neighbour`): a new 0x4f-byte copy goes to param 3 (the old one is not freed, as in the
/// original) and the answer is −11, or 0 when there is none. Anything else: 0.
/// Original: `club_screen_next_match_tab_input` (0x468c10, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screen_next_match_tab_input(widget: u32) -> i32 {
    let w = widget as i16;
    let club = get_screen_param(0);
    let reserves = get_screen_param(1);
    let fixture = get_screen_param(3);
    let action = club_tabs_action(w);
    if action == 0x1d {
        exe::comp_open_league_progress_for_fixture(fixture);
        return -4;
    }
    if action == 0x1e {
        let open: unsafe extern "C" fn(usize, usize, usize) = core::mem::transmute(0x46e6b0usize);
        let other = club_tabs_value(w);
        let other = if other != 0 {
            other
        } else if club != read_usize(fixture + fixture::HOME_CLUB) {
            read_usize(fixture + fixture::HOME_CLUB)
        } else {
            read_usize(fixture + fixture::AWAY_CLUB)
        };
        open(club, other, reserves);
        return -4;
    }
    if action == 0x26 || action == 0x27 {
        covered!("tel03_4 next match neighbour");
        return match club_next_match_neighbour(club, reserves, fixture, action == 0x27) {
            None => 0,
            Some(c) => {
                let copy = exe::crt_operator_new(fixture::SIZE as u32);
                for (k, b) in c.iter().enumerate() {
                    write_u8(copy + k, *b);
                }
                set_screen_param(3, copy);
                -11
            }
        };
    }
    0
}

/// The Please Confirm box of a cancellation on the fixtures tab: the title "Cancel ..." (`caption`), the heading
/// "Please Confirm", the message (already in the text buffer: `message` is what the original passes, the buffer or the
/// formatter's answer), the yes/no buttons, the club's kit colours and the dialog code `code`.
unsafe fn club_fixtures_confirm(caption: usize, message: usize, colours: (u32, u32), code: u32) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let confirm: unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x591840usize);
    let (mut heading, mut title) = ([0u8; 0x100], [0u8; 0x100]);
    let h = lang(heading.as_mut_ptr() as usize, 0x9909b8); // "Please Confirm"
    let t = lang(title.as_mut_ptr() as usize, caption);
    confirm(t, h, message, 1, colours.0, colours.1, code);
}

/// The long date of the cancellation boxes (`Date::get_long_date_string`, 200 bytes, no time of day).
unsafe fn club_fixtures_long_date(date: usize, out: &mut [u8; 0xc8]) {
    exe::date_get_long_date_string(date, out.as_mut_ptr() as usize, 0xc8, 0xff);
}

/// Answers a click on the club screen's fixtures tab (param 0 the club, 0xc the season page, 0xd the fixtures list, 0xe
/// the cancel mode, 0x1e what a cancellation box is about). First the answer of a cancellation box: 0x10/0x11 open the
/// tournament invitations of the answer (−4); 0x12 (a friendly), 0x13 (a proposed friendly), 0x14 (a tour) and 0x15 (a
/// tournament) cancel it on yes (`Friendly::on_friendly_fixture_cancelled`, `remove_friendly_match`,
/// `cancel_friendly_dates_request`, `cancel_tournament`), the friendly's copy is deleted, and param 0x1e goes back to 0.
/// Then the click (a real widget saves the scroll row of param 0x11 into param 0x10): 0x10/0x11 the season page down or
/// up (param 0xc; the list of param 0xd freed, −11). In cancel mode: 0x15 a friendly, 0x16 a proposed friendly, 0x17 a
/// tour and 0x18 a tournament the club controls and hosts open the Please Confirm box (codes 0x12..0x15) after leaving
/// the mode (−4); any other widget only leaves the mode (−11). Otherwise: 0x12 switches the cancel mode (−11); 0x13
/// arranges a friendly (a tour, kind 0xb, opens the tour nation screen) (−4); 0x14 opens the competition at the
/// fixture's date (−4); 0x17 the tour's clubs screen (the assert box without a tour) (−4); 0x18 the tournament
/// invitations (−4). Anything else: 0.
/// Original: `club_screen_fixtures_tab_input` (0x4691d0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screen_fixtures_tab_input(widget: u32) -> i32 {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let invitations: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x4739d0usize);
    let w = widget as i16;
    let club = get_screen_param(0);
    let page = get_screen_param(0xc);
    let list = get_screen_param(0xd);
    let cancel_mode = get_screen_param(0xe);
    if read_u32(CLUB_TABS_DIALOG_ANSWERED) != 0 {
        let code = read_u32(G_PENDING_COMMAND_CODE);
        if code == 0x11 || code == 0x10 {
            covered!("tel03_4 fixtures tournament invitations answer");
            let answer = read_usize(G_PENDING_COMMAND_ARG);
            write_u32(G_PENDING_COMMAND_CODE, 0);
            write_u32(G_PENDING_COMMAND_ARG, 0);
            invitations(answer, 1);
            return -4;
        }
        let friendly = read_usize(G_FRIENDLY);
        let yes = || read_u32(G_PENDING_COMMAND_ARG) == 2;
        match code {
            0x12 => {
                write_u32(G_PENDING_COMMAND_CODE, 0);
                let copy = get_screen_param(0x1e);
                if yes() {
                    let cancelled: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x5c6570usize);
                    cancelled(friendly, copy, club);
                }
                exe::crt_operator_delete(copy);
                set_screen_param(0x1e, 0);
                write_u32(G_PENDING_COMMAND_ARG, 0);
            }
            0x13 => {
                write_u32(G_PENDING_COMMAND_CODE, 0);
                let p = get_screen_param(0x1e);
                if yes() {
                    let remove: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x5c87b0usize);
                    remove(friendly, p, 1);
                }
                set_screen_param(0x1e, 0);
                write_u32(G_PENDING_COMMAND_ARG, 0);
            }
            0x14 => {
                write_u32(G_PENDING_COMMAND_CODE, 0);
                let p = get_screen_param(0x1e);
                if yes() {
                    let cancel_dates: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5c85d0usize);
                    cancel_dates(friendly, p);
                }
                set_screen_param(0x1e, 0);
                write_u32(G_PENDING_COMMAND_ARG, 0);
            }
            0x15 => {
                write_u32(G_PENDING_COMMAND_CODE, 0);
                let p = get_screen_param(0x1e);
                if yes() {
                    let cancel_tournament: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x5c8260usize);
                    cancel_tournament(friendly, p, 1);
                }
                set_screen_param(0x1e, 0);
                write_u32(G_PENDING_COMMAND_ARG, 0);
            }
            _ => {}
        }
    }
    if w != -1 {
        set_screen_param(0x10, exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(0x11)));
    }
    let action = club_tabs_action(w);
    if action == 0x10 || action == 0x11 {
        let page = (page & 0xffff) as i32 + if action == 0x10 { -1 } else { 1 }; // tel03_4 the season page step
        set_screen_param(0xc, page as usize);
        if list != 0 {
            exe::drop_llist(list);
            exe::crt_operator_delete(list);
            set_screen_param(0xd, 0);
        }
        return -11;
    }
    let mut colours = (0u32, 0u32);
    let mut date = [0u8; 0xc8];
    if cancel_mode != 0 {
        match action {
            0x15 => {
                covered!("tel03_4 fixtures cancel friendly");
                set_screen_param(0xe, 0);
                let copy = exe::crt_operator_new(fixture::SIZE as u32);
                let source = club_tabs_value(w);
                for k in 0..fixture::SIZE {
                    write_u8(copy + k, read_u8(source + k));
                }
                set_screen_param(0x1e, copy);
                let home = read_usize(copy + fixture::HOME_CLUB);
                let other = if club == home { read_usize(copy + fixture::AWAY_CLUB) } else { home };
                let date_of: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
                let mut d = [0u8; 8];
                let at = date_of(&mut d, read_u16(copy + fixture::DAY) as u32, read_u16(copy + fixture::YEAR) as u32);
                club_fixtures_long_date(at, &mut date);
                exe::database_get_club_kit_colours(club, 1, &mut colours.0, &mut colours.1);
                let gender = read_i8(other + club::SHORT_GENDER_NAME) as i32;
                // "Cancel the friendly match against {}<%s - club>{} on {}<%s - date> ?" (and the other text for a
                // friendly with a stage byte at +0x42)
                let text = if read_u8(copy + 0x42) == 0xff { 0x990a38 } else { 0x9909c8 };
                lang(CLUB_TABS_TEXT, text, gender, gender, 0x64, other + club::SHORT_NAME, date.as_ptr() as usize);
                club_fixtures_confirm(0x9909a8, CLUB_TABS_TEXT, colours, 0x12); // "Cancel Friendly"
                return -4;
            }
            0x16 => {
                set_screen_param(0xe, 0);
                let p = club_tabs_value(w);
                set_screen_param(0x1e, p);
                let mut other = read_usize(p + 0x10);
                if club == other {
                    other = read_usize(p + 0x14);
                }
                club_fixtures_long_date(p, &mut date);
                exe::database_get_club_kit_colours(club, 1, &mut colours.0, &mut colours.1);
                let gender = read_i8(other + club::SHORT_GENDER_NAME) as i32;
                // "Cancel the proposed friendly match against {}<%s - club>{} on {}<%s - date> ?"
                let message = lang(CLUB_TABS_TEXT, 0x990958, gender, gender, 0x64, other + club::SHORT_NAME, date.as_ptr() as usize);
                club_fixtures_confirm(0x9909a8, message, colours, 0x13); // "Cancel Friendly"
                return -4;
            }
            0x17 => {
                set_screen_param(0xe, 0);
                let p = club_tabs_value(w);
                set_screen_param(0x1e, p);
                let start = p + 9;
                club_fixtures_long_date(start, &mut date);
                exe::database_get_club_kit_colours(club, 1, &mut colours.0, &mut colours.1);
                let today = read_u32(G_CURRENT_DATE);
                let (today_day, today_year) = (today as u16 as i16, (today >> 16) as u16);
                let start_year = read_u16(start + 2);
                let started = today_year > start_year || (today_year == start_year && today_day > read_i16(start));
                let nation = read_usize(p + 4);
                let gender = read_i8(nation + nation::SHORT_GENDER_NAME) as i32;
                // "Cancel the tour of {}<%s - nation>{} which started on / starts on {}<%s - date> ?"
                lang(CLUB_TABS_TEXT, if started { 0x990910 } else { 0x9908c8 }, gender, gender, 0x64, nation + nation::SHORT_NAME, date.as_ptr() as usize);
                club_fixtures_confirm(0x9908bc, CLUB_TABS_TEXT, colours, 0x14); // "Cancel Tour"
                return -4;
            }
            0x18 => {
                set_screen_param(0xe, 0);
                let p = club_tabs_value(w);
                if exe::human_manager_current_controls_club(club, 0, 1) != 0 && club == read_usize(p) {
                    set_screen_param(0x1e, p);
                    club_fixtures_long_date(p + 4, &mut date);
                    exe::database_get_club_kit_colours(club, 1, &mut colours.0, &mut colours.1);
                    // "Cancel the <%s - competition> tournament which starts on {}<%s - date> ?"
                    lang(CLUB_TABS_TEXT, 0x990870, 0x64, p + 0x31, date.as_ptr() as usize);
                    club_fixtures_confirm(0x99085c, CLUB_TABS_TEXT, colours, 0x15); // "Cancel Tournament"
                    return -4;
                }
                // not a tournament the club hosts: on to the ordinary answers
            }
            _ => {
                if w != -1 {
                    set_screen_param(0xe, 0);
                    return -11;
                }
            }
        }
    }
    let action = club_tabs_action(w);
    match action {
        0x12 => {
            set_screen_param(0xe, (cancel_mode == 0) as usize); // tel03_4 the cancel mode switch
            -11
        }
        0x13 => {
            let value = club_tabs_value(w);
            if w != -1 && value as u8 == 0xb {
                let tour: unsafe extern "C" fn(usize) = core::mem::transmute(0x4719a0usize);
                tour(club);
            } else {
                let arrange: unsafe extern "C" fn(usize, usize, usize) = core::mem::transmute(0x4701f0usize);
                arrange(club, value, 0);
            }
            -4
        }
        0x14 => {
            let f = club_tabs_value(w);
            let open: unsafe extern "C" fn(usize, u32, u32, u32) = core::mem::transmute(0x48f8e0usize);
            let by_stage = (read_u8(f + 0x43) != 0xff) as u32;
            open(read_usize(f + fixture::COMPETITION), read_u16(f + fixture::DAY) as u32, read_u16(f + fixture::YEAR) as u32, by_stage);
            -4
        }
        0x17 => {
            let tour = club_tabs_value(w);
            if tour == 0 {
                covered!("tel03_4 fixtures tour assert");
                club_tabs_error_box(0x33a3);
                return -4;
            }
            if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x472380, 0x472850, 1, 0, 0) != 0 {
                set_screen_param(0, tour);
                set_screen_param(1, 0);
                set_screen_param(2, 0);
                write_u32(tour + 0x103, 1);
            }
            -4
        }
        0x18 => {
            invitations(club_tabs_value(w), 1);
            -4
        }
        _ => 0,
    }
}

/// Shows the club_screens.cpp "Error" box at the given line (the game continues) and clears the left mouse flag.
unsafe fn club_tabs_error_box(line: u32) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let join: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(0x98e598, 0, 0, name.as_mut_ptr(), ext.as_mut_ptr());
    let s = join(name.as_mut_ptr(), ext.as_mut_ptr(), line);
    format(0xae24d0, 0x9870e8, s); // "v%s %s.%s %d"
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0); // "Error"
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
}

/// Frees what the club screen holds when it closes: the buffers of params 2 and 3 (delete; 3 is the next match tab's
/// fixture copy), the fixtures list of param 0xd (`LLIST::~LLIST` and delete) and the array of param 0x15 (`free`, with
/// param 0x16), each param back to 0. The four params are read first.
/// Original: `club_screen_leave` (0x469da0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screen_leave() {
    let a = get_screen_param(2);
    let b = get_screen_param(3);
    let list = get_screen_param(0xd);
    let array = get_screen_param(0x15);
    if a != 0 {
        exe::crt_operator_delete(a);
        set_screen_param(2, 0);
    }
    if b != 0 {
        exe::crt_operator_delete(b);
        set_screen_param(3, 0);
    }
    if list != 0 {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
        set_screen_param(0xd, 0);
    }
    if array != 0 {
        exe::crt_free(array);
        set_screen_param(0x15, 0);
        set_screen_param(0x16, 0);
    }
}

/// Comparer for `club_screen_next_match_tab_input`: answers that open a screen run the original only; Previous and
/// Next (0x26/0x27) also run the original only (they allocate the new copy), and the fixture it leaves in param 3 is
/// checked against the one the copy picks (`club_next_match_neighbour`), or "none" against an answer of 0.
pub(crate) unsafe extern "C" fn compare_club_screen_next_match_tab_input(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I0%%);
    let action = club_tabs_action(widget as i16);
    if action != 0x26 && action != 0x27 {
        return o(widget);
    }
    let old = get_screen_param(3);
    let plan = club_next_match_neighbour(get_screen_param(0), get_screen_param(1), old, action == 0x27);
    let r = o(widget);
    let now = get_screen_param(3);
    let ok = match plan {
        None => r == 0 && now == old,
        Some(c) => r == -11 && now != 0 && c.iter().enumerate().all(|(k, b)| read_u8(now + k) == *b),
    };
    SCOREBOARD.record(ok, || format!("club screen next match (action {:#x}): answer {} param 3 {:#x} (was {:#x}), plan {}", action, r, now, old, if plan.is_some() { "a fixture" } else { "none" }));
    r
}

/// Comparer for `club_screen_fixtures_tab_input`: the clicks that only write params (switching or leaving the cancel
/// mode) run side by side (`compare_resposta`); the season page runs the original only (it frees the list) and the
/// page and the list param are checked afterwards; a pending box answer and everything that opens a screen or a box
/// run the original only.
pub(crate) unsafe extern "C" fn compare_club_screen_fixtures_tab_input(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I1%%);
    let w = widget as i16;
    let pending = read_u32(CLUB_TABS_DIALOG_ANSWERED) != 0 && (0x10..=0x15).contains(&read_u32(G_PENDING_COMMAND_CODE));
    if pending || w == -1 {
        return o(widget);
    }
    let action = club_tabs_action(w);
    if action == 0x10 || action == 0x11 {
        let page = get_screen_param(0xc);
        let r = o(widget);
        let want = ((page & 0xffff) as i32 + if action == 0x10 { -1 } else { 1 }) as usize;
        let ok = r == -11 && get_screen_param(0xc) == want && get_screen_param(0xd) == 0;
        SCOREBOARD.record(ok, || format!("club screen fixtures page (action {:#x}): answer {} page {:#x} (want {:#x}) list {:#x}", action, r, get_screen_param(0xc), want, get_screen_param(0xd)));
        return r;
    }
    let cancel_mode = get_screen_param(0xe) != 0;
    let stays = if cancel_mode { !(0x15..=0x18).contains(&action) } else { action == 0x12 };
    compare_resposta("club screen fixtures tab input", widget, o, club_screen_fixtures_tab_input, !stays)
}

/// Comparer for `club_screen_leave`: freeing does not run twice; only the original runs and the params it clears are
/// checked afterwards.
pub(crate) unsafe extern "C" fn compare_club_screen_leave() {
    let had = [get_screen_param(2), get_screen_param(3), get_screen_param(0xd), get_screen_param(0x15)];
    original_fn::<unsafe extern "C" fn()>(%%I2%%)();
    let left = [get_screen_param(2), get_screen_param(3), get_screen_param(0xd), get_screen_param(0x15), get_screen_param(0x16)];
    let ok = (0..4).all(|k| had[k] == 0 || left[k] == 0) && (had[3] == 0 || left[4] == 0);
    SCOREBOARD.record(ok, || format!("club screen leave: params 2, 3, 0xd, 0x15, 0x16 left {:x?}", left));
}
