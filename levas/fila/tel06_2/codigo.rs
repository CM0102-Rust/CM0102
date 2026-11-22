// ---------- batch %%LEVA%%: the club screens (6b): the squad navigation menu, the club screen's action answers and the part exchange screen's input (club_screens.cpp) ----------

/// One entry of the club squad navigation menu (`Display::add_item`, kind 2): the text, the handler and the data, in
/// the popup list colours.
unsafe fn squad_menu_entry(area: u32, text: usize, handler: u32, data: usize) {
    tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, job_colour(JOB_COLOUR_31AA), job_colour(JOB_COLOUR_BDEC), 1, 1,
        job_colour(JOB_COLOUR_BDEA), text as u32, 0, handler, data as u32, area]);
}

/// The menu's popup area, made on the first entry: the menu button (`ScreenManager::add_button_item` at 0x69, 0xf,
/// 0x78, 0x23 in the screen's colours) and the popup area it opens (`Display::add_area`, flags 0x30, popup colour);
/// later sections get a separator line instead (kind 1, flags 0x1000010). Returns true when a separator was added.
unsafe fn squad_menu_section(area: &mut u32, colour: u32, colour_text: u32) -> bool {
    if *area as u16 == 0xffff {
        let button = tel26_add_button([0x69, 0xf, 0x78, 0x23, 0x30, colour, 0xc, 1, colour_text, 0, 0, 0]);
        *area = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x30, job_colour(JOB_COLOUR_31AA), button]);
        false
    } else {
        tel26_add_item(current_human_display(), [1, 0, 0, 0, 0, 0, 0, 0x1000010, job_colour(JOB_COLOUR_31AA), job_colour(JOB_COLOUR_BDEC), 1, 1,
            job_colour(JOB_COLOUR_BDEA), 0, 0, 0, 0, *area]);
        true
    }
}

/// The clubs the navigation menu lists as related (`build_club_squad_navigation_menu`), in table order.
unsafe fn squad_menu_related_clubs(club: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let nation = read_usize(club + club::NATION);
    let table = read_usize(G_CLUB_TABLE);
    let at = |k: i32| table + k as usize * club::SIZE;
    let same_continent = |c: usize| read_usize(c + club::NATION) != 0 && read_u32(read_usize(c + club::NATION) + nation::CONTINENT) == read_u32(nation + nation::CONTINENT);
    if exe::club_is_national_team(club) != 0 && nation != 0 && read_u32(nation + nation::CONTINENT) != 0 {
        // the national teams of the continent (the last G_NATION_COUNT entries of the club table)
        let mut k = read_i32(G_CLUB_COUNT) - read_i32(G_NATION_COUNT);
        while k < read_i32(G_CLUB_COUNT) {
            if same_continent(at(k)) {
                v.push(at(k));
            }
            k += 1;
        }
    } else if exe::database_is_national_team(club) != 0 && nation != 0 && read_u32(nation + nation::CONTINENT) != 0 {
        // the second sides of the continent's national teams (the G_NATION_COUNT entries before them)
        let m = read_i32(G_NATION_COUNT);
        let mut k = read_i32(G_CLUB_COUNT) - 2 * m;
        while k < read_i32(G_CLUB_COUNT) - read_i32(G_NATION_COUNT) {
            if same_continent(at(k)) {
                v.push(at(k));
            }
            k += 1;
        }
    } else if read_usize(club + club::DIVISION) != 0 && nation != 0 {
        // the clubs of the same nation and division (the club itself included)
        let mut k = 0;
        while k < read_i32(G_CLUB_COUNT) - read_i32(G_NATION_COUNT) {
            let c = at(k);
            if read_usize(c + club::NATION) == nation && read_usize(c + club::DIVISION) == read_usize(club + club::DIVISION) {
                v.push(c);
            }
            k += 1;
        }
    }
    v
}

/// Rule: the club screen's squad menu lists the club's own sides (first team, 'B' or reserve side, the national
/// Under-21 or Under-23 side), the related clubs (the continent's national teams, or the clubs of the same division),
/// the club's national team and the written history.
///
/// `build_club_squad_navigation_menu(club, colour, colour_text, squad_handler, history_handler, alt_handler,
/// with_squads, senior_mode)`; the popup and its sections are made by `squad_menu_section`, each entry by
/// `squad_menu_entry`. With `with_squads`:
/// - a national team (`exe::database_is_national_team`): a second side (`exe::club_is_national_team` false) lists
///   "Main Squad" (with `senior_mode`) or "'B' Squad" (when it has one, `exe::database_club_has_reserve_squad`) with
///   `alt_handler`, then "Under 23 Squad" (the nation's flag in [G_NATIONAL_TEAMS]+0x36 by nation id) or "Under 21
///   Squad" with the nation's Under-21 side (`exe::database_get_national_u21_team`) and `squad_handler`; a first side
///   lists "Main Squad" with the national team (`exe::database_get_national_team`) and `squad_handler`, and "'B'
///   Squad" with `alt_handler` when the national team has one;
/// - a club: its linked side (`exe::club_get_linked_b_team`) by name with `squad_handler`, or, when the human runs it
///   and it has a reserve side, "Senior Squad" (`senior_mode`) or "Reserve Squad" with `alt_handler`.
/// Then the related clubs (`squad_menu_related_clubs`; a separator counts as an entry), sorted by 0x53ffe0 (reputation)
/// and cut to 28 entries minus those already listed, minus 2 for a club's national team and minus 2 for the history,
/// sorted again by 0x540080 (name), each by name with `alt_handler` (`senior_mode`) or `squad_handler`; the club's
/// national team for a club (by name, `squad_handler`); "History" with `history_handler` when the club's history file
/// exists (`history_his_file_exists`, 0x5f2500, of `exe::club_get_english_name`). Returns 1.
/// Original: `build_club_squad_navigation_menu` (0x481ef0, club_screens.cpp, cdecl).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn build_club_squad_navigation_menu(club: usize, colour: u32, colour_text: u32, squad_handler: u32, history_handler: u32,
    alt_handler: u32, with_squads: u32, senior_mode: u32) -> u32 {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    const MAIN_SQUAD: usize = 0x991550;
    const B_SQUAD: usize = 0x991544;
    let mut area = u32::MAX;
    let mut count = 0i32;
    let nation = read_usize(club + club::NATION);
    if with_squads != 0 {
        if exe::database_is_national_team(club) != 0 {
            squad_menu_section(&mut area, colour, colour_text);
            if exe::club_is_national_team(club) == 0 {
                covered!("tel06_2 squad menu of a second national side");
                if senior_mode != 0 {
                    lang(JOB_TEXT, MAIN_SQUAD);
                    squad_menu_entry(area, JOB_TEXT, alt_handler, 0);
                    count = 1;
                } else if exe::database_club_has_reserve_squad(club) != 0 {
                    lang(JOB_TEXT, B_SQUAD);
                    squad_menu_entry(area, JOB_TEXT, alt_handler, 0);
                    count = 1;
                }
                let under_23 = read_u8(read_usize(read_usize(G_NATIONAL_TEAMS) + 0x36) + read_u32(nation) as usize) != 0;
                lang(JOB_TEXT, if under_23 { 0x991534usize } else { 0x991524 });
                let side = exe::database_get_national_u21_team(nation);
                squad_menu_entry(area, JOB_TEXT, squad_handler, side);
                count += 1;
            } else {
                lang(JOB_TEXT, MAIN_SQUAD);
                squad_menu_entry(area, JOB_TEXT, squad_handler, exe::database_get_national_team(nation));
                count = 1;
                if exe::database_club_has_reserve_squad(exe::database_get_national_team(nation)) != 0 {
                    lang(JOB_TEXT, B_SQUAD);
                    squad_menu_entry(area, JOB_TEXT, alt_handler, exe::database_get_national_team(nation));
                    count = 2;
                }
            }
        } else {
            let mut no_link = 0u32;
            let linked = exe::club_get_linked_b_team(club, &mut no_link, 1);
            if linked != 0 {
                squad_menu_section(&mut area, colour, colour_text);
                squad_menu_entry(area, linked + club::SHORT_NAME, squad_handler, linked);
                count = 1;
            } else if exe::human_manager_current_controls_club(club, 0, 1) != 0 && exe::database_club_has_reserve_squad(club) != 0 {
                squad_menu_section(&mut area, colour, colour_text);
                lang(JOB_TEXT, if senior_mode != 0 { 0x991514usize } else { 0x991504 }); // "Senior Squad" / "Reserve Squad"
                squad_menu_entry(area, JOB_TEXT, alt_handler, 0);
                count = 1;
            }
        }
    }
    let mut related = squad_menu_related_clubs(club);
    let history_exists: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5f2500usize);
    let history = history_exists(exe::club_get_english_name(club)) != 0;
    if !related.is_empty() {
        covered!("tel06_2 squad menu related clubs");
        if squad_menu_section(&mut area, colour, colour_text) {
            count += 1;
        }
        let arr = related.as_mut_ptr() as usize;
        exe::crt_qsort(arr, related.len() as u32, 4, 0x53ffe0usize);
        let mut limit = 0x1c - count;
        if exe::database_is_national_team(club) == 0 && nation != 0 && exe::database_get_national_team(nation) != 0 {
            limit -= 2;
        }
        if history {
            limit -= 2;
        }
        let n = (related.len() as i32).min(limit);
        exe::crt_qsort(arr, n as u32, 4, 0x540080usize);
        for &c in related.iter().take(n.max(0) as usize) {
            squad_menu_entry(area, c + club::SHORT_NAME, if senior_mode != 0 { alt_handler } else { squad_handler }, c);
        }
    }
    if exe::database_is_national_team(club) == 0 && nation != 0 {
        let team = exe::database_get_national_team(nation);
        if team != 0 {
            squad_menu_section(&mut area, colour, colour_text);
            squad_menu_entry(area, team + club::SHORT_NAME, squad_handler, team);
        }
    }
    if history {
        squad_menu_section(&mut area, colour, colour_text);
        lang(JOB_TEXT, 0x9914d4usize, 0xad9c64usize); // "History<%s - COMMENT - written history screen>"
        squad_menu_entry(area, JOB_TEXT, history_handler, 0);
    }
    1
}

/// Comparer for `build_club_squad_navigation_menu`: side by side over the display list (`compare_match_screen_build`)
/// and the return.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn compare_build_club_squad_navigation_menu(club: usize, colour: u32, colour_text: u32, squad_handler: u32,
    history_handler: u32, alt_handler: u32, with_squads: u32, senior_mode: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32) -> u32>(%%I0%%);
    let (mut a, mut b) = (0u32, 0u32);
    compare_match_screen_build("club squad menu", &[],
        || a = o(club, colour, colour_text, squad_handler, history_handler, alt_handler, with_squads, senior_mode),
        || b = build_club_squad_navigation_menu(club, colour, colour_text, squad_handler, history_handler, alt_handler, with_squads, senior_mode));
    SCOREBOARD.record(a == b, || format!("build_club_squad_navigation_menu (club screen) returned {} and Rust {}", a, b));
    a
}

/// A method of the Friendly object ([G_FRIENDLY]) without arguments that returns a pointer (0x5c68b0 the human club's
/// open invitation, 0x5c6920 its open dates request, 0x5c6830 its tournament).
unsafe fn club_action_friendly_find(f: usize) -> usize {
    let g: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(f);
    g(read_usize(G_FRIENDLY))
}

/// `Friendly::get_club_date_status` (0x5c75a0, thiscall, ret 0x24): the date status of `club` on `date` (1 = free
/// for the match), with no chosen dates.
unsafe fn club_action_date_status(club: usize, date: usize, kind: u32, check_stadium: u32) -> u8 {
    type F = unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut usize, *mut usize, usize, u32, u32, u32) -> u8;
    let f: F = core::mem::transmute(0x5c75a0usize);
    let (mut tournament, mut fixture, mut request) = (0usize, 0usize, 0usize);
    f(read_usize(G_FRIENDLY), club, date, &mut tournament, &mut fixture, &mut request, 0, 0, kind, check_stadium)
}

/// Whether `club` is free on `date` (`exe::friendly_is_club_free_on_date`) and its date status is 1.
unsafe fn club_action_free(club: usize, date: usize, kind: u32, check_stadium: u32) -> bool {
    exe::friendly_is_club_free_on_date(read_usize(G_FRIENDLY), date as *mut [u8; 8], club, kind) != 0 && club_action_date_status(club, date, kind, check_stadium) == 1
}

/// The club's kit colours (`exe::database_get_club_kit_colours`, kit 1): the foreground and background words in
/// dwords.
unsafe fn club_action_kit(club: usize) -> (u32, u32) {
    let (mut fg, mut bg) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut fg, &mut bg);
    (fg, bg)
}

/// `confirm_push` (0x591840, cdecl): the box with its title, heading, text, buttons (1 = yes/no), the two colours and
/// the dialog code its answer comes back with.
unsafe fn club_action_confirm(title: usize, heading: usize, text: usize, buttons: u32, text_colour: u32, colour: u32, code: u32) {
    let f: unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x591840usize);
    f(title, heading, text, buttons, text_colour, colour, code);
}

/// The answer of a club screen dialog (see `handle_club_screen_action`); None when it falls through to the buttons.
unsafe fn club_action_dialog_answer() -> Option<i32> {
    let code = read_u32(G_PENDING_COMMAND_CODE);
    let yes = read_u32(G_PENDING_COMMAND_ARG) == 2;
    let f = read_usize(G_FRIENDLY);
    let tournament_answer = |add: u32| {
        let t = club_action_friendly_find(0x5c6830);
        let invite: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x5c6990usize);
        invite(f, get_screen_param(0), add);
        write_u32(t + 0x29, 0);
        if read_u32(t + 0x2d) != 0 {
            write_u32(G_PENDING_COMMAND_CODE, 0x10);
            write_usize(G_PENDING_COMMAND_ARG, t);
            write_u32(t + 0x2d, 0);
        }
        -10
    };
    match code {
        0x41 => {
            write_u32(G_PENDING_COMMAND_CODE, 0);
            if yes {
                covered!("tel06_2 scout assigned");
                write_u32(G_PENDING_COMMAND_ARG, 0);
                let scope: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7f82f0usize);
                scope(read_usize(G_SCOUT_MANAGER), get_screen_param(0));
                return Some(-13);
            }
        }
        0xc => {
            write_u32(G_PENDING_COMMAND_CODE, 0);
            if yes {
                covered!("tel06_2 friendly invitation sent");
                write_u32(G_PENDING_COMMAND_ARG, 0);
                let m = club_action_friendly_find(0x5c68b0);
                let club = get_screen_param(0);
                if read_u32(m + 0x10) == read_u32(m + 0xc) {
                    write_usize(m + 0x14, club);
                } else {
                    write_usize(m + 0x10, club);
                }
                exe::friendly_send_friendly_request(f, m);
                write_u32(m + 8, 0);
                return Some(-10);
            }
            write_u32(G_PENDING_COMMAND_ARG, 0);
        }
        0xd => {
            write_u32(G_PENDING_COMMAND_CODE, 0);
            if yes {
                covered!("tel06_2 friendly dates request answered");
                write_u32(G_PENDING_COMMAND_ARG, 0);
                let r = club_action_friendly_find(0x5c6920);
                let assign: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x5c57d0usize);
                assign(f, r, get_screen_param(0));
                let mut k = 0i8;
                while read_i8(r + 8) > 0 && k < read_i8(r + 8) {
                    let e = (r as isize + 25 * k as isize) as usize;
                    if read_u32(e + 0x19) != 0 && read_u8(e + 0x21) == 1 {
                        write_u8(e + 0x21, 2);
                        let m = exe::friendly_add_friendly_match(f, read_usize(e + 0x1d), e + 9, read_usize(e + 0x19), 0);
                        if m != 0 {
                            exe::friendly_send_friendly_request(f, m);
                        }
                    }
                    k = k.wrapping_add(1);
                }
                write_u32(r + 0x103, 0);
                return Some(-10);
            }
            write_u32(G_PENDING_COMMAND_ARG, 0);
        }
        0xe => {
            write_u32(G_PENDING_COMMAND_CODE, 0);
            if yes {
                write_u32(G_PENDING_COMMAND_ARG, 0);
                return Some(tournament_answer(0));
            }
            write_u32(G_PENDING_COMMAND_ARG, 0);
        }
        0xf => {
            write_u32(G_PENDING_COMMAND_CODE, 0);
            write_u32(G_PENDING_COMMAND_ARG, 0);
            if yes {
                return Some(tournament_answer(1));
            }
        }
        _ => {}
    }
    None
}

/// The friendly dialogs (buttons 0x66 and 0x67): "Invite For Friendly"/"Offer Friendly" when the club is free on the
/// date, otherwise the "unavailable" box (see `handle_club_screen_action`).
unsafe fn club_action_friendly_box(club: usize, date: usize, kit_club: usize, invite: bool, code: u32, swapped: bool) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (mut title, mut heading, mut when) = ([0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8]);
    let (t, h, d) = (title.as_mut_ptr() as usize, heading.as_mut_ptr() as usize, when.as_mut_ptr() as usize);
    let gender = read_i8(club + club::SHORT_GENDER_NAME) as i32;
    exe::date_get_long_date_string(date, d, 0xc8, 0xff);
    let (fg, bg) = club_action_kit(kit_club);
    let (text_colour, colour) = if swapped { (bg, fg) } else { (fg, bg) };
    if code != 0 {
        if swapped {
            // button 0x66: the title first, the text, the heading
            lang(t, if invite { 0x99180cusize } else { 0x9917bc });
            lang(JOB_TEXT, if invite { 0x9917ccusize } else { 0x991778 }, gender, gender, 0x64, club + club::SHORT_NAME, d);
            let hd = lang(h, 0x9909b8usize); // "Please Confirm"
            club_action_confirm(t, hd, JOB_TEXT, 1, text_colour, colour, code);
        } else {
            let text = lang(JOB_TEXT, 0x9917ccusize, gender, gender, 0x64, club + club::SHORT_NAME, d);
            let hd = lang(h, 0x9909b8usize);
            let tt = lang(t, 0x99180cusize); // "Invite For Friendly"
            club_action_confirm(tt, hd, text, 1, text_colour, colour, code);
        }
    } else {
        // "{}<%s - club>{} are unavailable for a friendly match on {}<%s - date>."
        let text = lang(JOB_TEXT, 0x991730usize, gender, gender, 0x64, club + club::SHORT_NAME, d);
        let hd = lang(h, 0x9916f8usize, 0xad9c64usize);
        let tt = lang(t, 0x99180cusize);
        club_action_confirm(tt, hd, text, 0, text_colour, colour, 0);
    }
}

/// The tournament dialogs (buttons 0x68 and 0x69; see `handle_club_screen_action`).
unsafe fn club_action_tournament_box(club: usize, t: usize, format: usize, heading_format: usize, title_format: usize, buttons: u32, code: u32) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (mut title, mut heading, mut when) = ([0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8]);
    let (tb, h, d) = (title.as_mut_ptr() as usize, heading.as_mut_ptr() as usize, when.as_mut_ptr() as usize);
    let gender = read_i8(club + club::SHORT_GENDER_NAME) as i32;
    exe::date_get_long_date_string(t + 4, d, 0xc8, 0xff);
    let (fg, bg) = club_action_kit(read_usize(t));
    let text = lang(JOB_TEXT, format, gender, gender, 0x64, club + club::SHORT_NAME, t + 0x31, d);
    let hd = if heading_format == 0x9916f8 { lang(h, heading_format, 0xad9c64usize) } else { lang(h, heading_format) };
    let tt = lang(tb, title_format);
    club_action_confirm(tt, hd, text, buttons, fg, bg, code);
}

/// Rule: the club screen's action buttons and the answers to their dialogs: take control, assign a scout, apply for
/// the job, invite for a friendly (or offer one), answer a dates request, invite to or uninvite from the human's
/// tournament.
///
/// `handle_club_screen_action(widget)`. First, when a dialog has answered (`CLUB_TABS_DIALOG_ANSWERED`), by its code
/// (`G_PENDING_COMMAND_CODE`, cleared) and a yes (`G_PENDING_COMMAND_ARG` 2, cleared; codes 0xc..0xf clear the answer
/// also on a no):
/// - 0x41: the scouts' current search goes to the screen's club (`SCOUT_MANAGER::scope_current_search_to_club`,
///   0x7f82f0, param 0): -13;
/// - 0xc: the open invitation (0x5c68b0) gets the club as opponent (+0x10, or +0x14 when +0x10 equals the human club
///   +0xc), is sent (`exe::friendly_send_friendly_request`) and +8 cleared: -10;
/// - 0xd: the open dates request (0x5c6920) gets the club (`Friendly::schedule_assign_opponent`, 0x5c57d0); each of
///   its dates (count +8, entries of 25 bytes from +9: the date, +0x10 the opponent, +0x14 the club, +0x18 the state)
///   with an opponent and state 1 goes to state 2 and becomes a friendly (`exe::friendly_add_friendly_match`, sent when
///   made); +0x103 cleared: -10;
/// - 0xe / 0xf: the tournament (0x5c6830) drops / adds the club (`Friendly::set_tournament_invite`, 0x5c6990), +0x29
///   cleared; a pending +0x2d becomes dialog 0x10 with the tournament (and is cleared): -10.
/// Then the button (`club_tabs_action`), its value (`club_tabs_value`) and the screen's club (param 0, `c` below);
/// the boxes use `confirm_push` with the kit colours of the other club (`exe::database_get_club_kit_colours`):
/// - 0x64 Take Control: `open_take_control_confirm` (0x822a60) of the club: -4;
/// - 0x6a Scout Club: "Club Already Being Watched" when the club is in the session's searches
///   (`SCOUT_MANAGER::is_club_in_session_searches`, 0x7f8500), otherwise "Send scout to watch all ... matches ?"
///   (yes/no, code 0x41): -4;
/// - 0x65 Apply for Job: with a human on the screen slot (`G_SCREEN_RECORDS_MANAGER`), `open_apply_for_job_screen`
///   (0x6b19d0) of the club and -4; otherwise 0;
/// - 0x66 the open invitation (the value): when `c` is free on its date (`exe::friendly_is_club_free_on_date` and
///   `Friendly::get_club_date_status` 1, kind 0, stadium checked) "Invite For Friendly" (with an opponent at +0x10) or
///   "Offer Friendly" (yes/no, code 0xc); otherwise its date is moved to a week from today when its year is before
///   2001 and the "unavailable" box; -4;
/// - 0x67 the dates request (the value): the date of its first entry without an opponent; free: "Invite For Friendly"
///   (code 0xd), otherwise the "unavailable" box; -4;
/// - 0x68 the tournament (the value): "Uninvite To Tournament" (code 0xe); -4;
/// - 0x69 the tournament: free on its date (+4, kind +0xc, stadium not checked): "Invite To Tournament" (code 0xf),
///   otherwise the "unavailable for the tournament" box; -4.
/// Any other button: 0.
/// Original: `handle_club_screen_action` (0x482d10, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn handle_club_screen_action(widget: u32) -> i32 {
    if read_u32(CLUB_TABS_DIALOG_ANSWERED) != 0 {
        if let Some(r) = club_action_dialog_answer() {
            return r;
        }
    }
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let w = widget as i16;
    let value = club_tabs_value(w);
    match club_tabs_action(w) {
        0x64 => {
            let take_control: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x822a60usize);
            take_control(get_screen_param(0), 0);
            -4
        }
        0x6a => {
            covered!("tel06_2 scout club");
            let c = get_screen_param(0);
            let (fg, bg) = club_action_kit(c);
            let (mut title, mut heading) = ([0u8; 0xc8], [0u8; 0xc8]);
            let gender = read_i8(c + club::SHORT_GENDER_NAME) as i32;
            let searched: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7f8500usize);
            if searched(read_usize(G_SCOUT_MANAGER), c) != 0 {
                lang(title.as_mut_ptr() as usize, 0x9918c4usize); // "Assign Scout"
                let text = lang(JOB_TEXT, 0x991880usize, gender, gender, c + club::SHORT_NAME);
                let h = lang(heading.as_mut_ptr() as usize, 0x991864usize); // "Club Already Being Watched"
                club_action_confirm(title.as_ptr() as usize, h, text, 0, fg, bg, 0);
            } else {
                lang(title.as_mut_ptr() as usize, 0x991854usize); // "Assign Scout ?"
                lang(JOB_TEXT, 0x991820usize, gender, gender, c + club::SHORT_NAME);
                let h = lang(heading.as_mut_ptr() as usize, 0x9909b8usize);
                club_action_confirm(title.as_ptr() as usize, h, JOB_TEXT, 1, fg, bg, 0x41);
            }
            -4
        }
        0x65 => {
            let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
            if read_u32((G_SCREEN_RECORDS_MANAGER as isize + slot * 0x300) as usize) == 0 {
                return 0;
            }
            let apply: unsafe extern "C" fn(usize) = core::mem::transmute(0x6b19d0usize);
            apply(get_screen_param(0));
            -4
        }
        0x66 => {
            covered!("tel06_2 friendly invitation");
            let c = get_screen_param(0);
            if club_action_free(c, value, 0, 1) {
                club_action_friendly_box(c, value, read_usize(value + 0xc), read_u32(value + 0x10) != 0, 0xc, true);
            } else {
                if read_u16(value + 2) < 0x7d1 {
                    let (mut later, mut result) = ([0u8; 8], [0u8; 8]);
                    let p = exe::date_add_days(G_CURRENT_DATE, later.as_mut_ptr() as usize, 7);
                    exe::date_operator_3d(value, result.as_mut_ptr() as usize, p);
                }
                club_action_friendly_box(c, value, read_usize(value + 0xc), true, 0, true);
            }
            -4
        }
        0x67 => {
            let mut date = [0u8; 8];
            exe::date_date_54a110(date.as_mut_ptr() as usize);
            let last = read_i8(value + 8).wrapping_sub(1);
            if last >= 0 {
                for k in (0..=last as isize).rev() {
                    let e = (value as isize + 25 * k + 9) as usize;
                    if read_u32(e + 0x10) == 0 {
                        let mut result = [0u8; 8];
                        exe::date_operator_3d(date.as_mut_ptr() as usize, result.as_mut_ptr() as usize, e);
                    }
                }
            }
            let c = get_screen_param(0);
            let d = date.as_mut_ptr() as usize;
            let code = if club_action_free(c, d, 0, 1) { 0xd } else { 0 };
            club_action_friendly_box(c, d, read_usize(value), true, code, false);
            -4
        }
        0x68 => {
            let c = get_screen_param(0);
            club_action_tournament_box(c, value, 0x991690, 0x9909b8, 0x991678, 1, 0xe); // "Uninvite To Tournament"
            -4
        }
        0x69 => {
            covered!("tel06_2 tournament invitation");
            let c = get_screen_param(0);
            let kind = read_u8(value + 0xc) as u32;
            if club_action_free(c, value + 4, kind, 0) {
                club_action_tournament_box(c, value, 0x991620, 0x9909b8, 0x991608, 1, 0xf); // "Invite To Tournament"
            } else {
                club_action_tournament_box(c, value, 0x9915a4, 0x9916f8, 0x991608, 0, 0);
            }
            -4
        }
        _ => 0,
    }
}

/// What `handle_club_screen_action` answers, worked out without running it.
unsafe fn club_action_planned_answer(widget: u32) -> i32 {
    if read_u32(CLUB_TABS_DIALOG_ANSWERED) != 0 && read_u32(G_PENDING_COMMAND_ARG) == 2 {
        match read_u32(G_PENDING_COMMAND_CODE) {
            0x41 => return -13,
            0xc..=0xf => return -10,
            _ => {}
        }
    }
    match club_tabs_action(widget as i16) {
        0x64 | 0x6a | 0x66..=0x69 => -4,
        0x65 => {
            let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
            if read_u32((G_SCREEN_RECORDS_MANAGER as isize + slot * 0x300) as usize) == 0 { 0 } else { -4 }
        }
        _ => 0,
    }
}

/// Comparer for `handle_club_screen_action`: the answers write to the friendlies and the scouts and open dialogs and
/// screens, so only the original runs; its answer is checked against the plan (`club_action_planned_answer`).
pub(crate) unsafe extern "C" fn compare_handle_club_screen_action(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I1%%);
    let plan = club_action_planned_answer(widget);
    let r = o(widget);
    SCOREBOARD.record(r == plan, || format!("handle_club_screen_action({}) (club screen actions) returned {} and the plan {}", widget as i16, r, plan));
    r
}

/// The new value of a part exchange view parameter for the picked bit `v`: the bit replaces the others of its group
/// (`groups`: the masks that clear a group, with its members); None for a bit of no group.
fn part_exchange_pick(param: u32, v: u32, groups: &[(u32, &[u32])]) -> Option<u32> {
    groups.iter().find(|(_, members)| members.contains(&v)).map(|&(mask, _)| (param & mask) | v)
}

/// The groups of the part exchange screen's parameter 4 (the view) and parameter 9 (the columns).
const PART_EXCHANGE_VIEW_GROUPS: [(u32, &[u32]); 2] = [(0xffffefc0, &[1, 2, 4, 8, 0x10, 0x20, 0x1000]), (0xfffff83f, &[0x40, 0x80, 0x100, 0x200, 0x400])];
const PART_EXCHANGE_COLUMN_GROUPS: [(u32, &[u32]); 8] = [
    (0xffffffe0, &[1, 2, 4, 8, 0x10]), (0xfffffe1f, &[0x20, 0x40, 0x80, 0x100]), (0xfffff9ff, &[0x200, 0x400]),
    (0xffffc7ff, &[0x800, 0x1000, 0x2000]), (0xffff3fff, &[0x4000, 0x8000]), (0xfffcffff, &[0x10000, 0x20000]),
    (0xfff3ffff, &[0x40000, 0x80000]), (0xffcfffff, &[0x100000, 0x200000]),
];

/// The sort parameter 5 after picking sort column `bit` with value `v` against `current` (parameter 7 or 8; for bit
/// 0 the column is `v` itself): picking the column already sorted by flips the reverse flag 0x1000000, a new column
/// clears it; the bits other than 0xc1000000 and the column's are cleared.
fn part_exchange_sort(p5: u32, bit: u32, same: bool) -> u32 {
    let reverse = if p5 & bit != 0 && same && p5 & 0x1000000 == 0 { p5 | 0x1000000 } else { p5 & !0x1000000 };
    reverse & (0xc1000000 | bit)
}

/// Rule: the part exchange screen's input: the view and the columns (exclusive groups of bits), the sort column (a
/// second click reverses it), the scroll, adding the picked player to the offer (his value comes off the cash part)
/// and the transfer status screen.
///
/// `part_exchange_screen_handle_event(widget)`, from the screen's parameters 4 (view), 5 (sort), 6 (the sort column),
/// 9 (columns), 7 and 8 (the two other sort columns). A widget other than -1 first keeps the scroll of the list (param
/// 2's area, `exe::screenmanager_get_area_scroll_row`) in param 1. Then by the widget's action (`club_tabs_action`)
/// and value (`club_tabs_value`), each answering -11 unless said:
/// - 4: a view bit not already set replaces its group in param 4 (`PART_EXCHANGE_VIEW_GROUPS`; another bit: nothing);
/// - 9: a column bit not already set replaces its group in param 9 (`PART_EXCHANGE_COLUMN_GROUPS`);
/// - 5: a sort column (low byte) different from param 6 goes to param 6;
/// - 6: param 5 becomes the value plus the reverse flag (`part_exchange_sort` with the value's bits);
/// - 7 / 8: param 5 sorts by column 0x400 / 0x800 (reversed when it already did with the same column in param 7 / 8)
///   and the value goes to param 7 / 8;
/// - 3: -9; 1: the value to param 3;
/// - 2: the player (the value) joins the offer (param 0): its list (+4, made on the first one, `LLIST` of 0xe bytes)
///   gets a 7-byte record (his id, 0, 2, and a byte the original leaves uninitialised: 0 here), and his
///   `staff::VALUE` comes off the cash +0x1b (not below 0): -9;
/// - 0xa: the transfer status screen of the player (0x8f6770, (player, 0, -1, 0)): -4;
/// - otherwise (also 4, 9 and 5 when nothing changes) the menu bar (`exe::menubar_handle_command`, then
///   `exe::menubar_handle_object_link` when that answers 0) answers.
/// Original: `part_exchange_screen_handle_event` (0x4814e0, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn part_exchange_screen_handle_event(widget: u32) -> i32 {
    let p4 = get_screen_param(4) as u32;
    let p5 = get_screen_param(5) as u32;
    let p6 = get_screen_param(6) as u8;
    let p9 = get_screen_param(9) as u32;
    let p7 = get_screen_param(7) as u8;
    let p8 = get_screen_param(8) as u8;
    let w = widget as i16;
    if w != -1 {
        set_screen_param(1, exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(2)));
    }
    let id = club_tabs_action(w);
    let v = club_tabs_value(w) as u32;
    match id {
        4 if p4 & v == 0 => {
            if let Some(n) = part_exchange_pick(p4, v, &PART_EXCHANGE_VIEW_GROUPS) {
                set_screen_param(4, n as usize);
            }
            return -11;
        }
        9 if p9 & v == 0 => {
            if let Some(n) = part_exchange_pick(p9, v, &PART_EXCHANGE_COLUMN_GROUPS) {
                set_screen_param(9, n as usize);
            }
            return -11;
        }
        5 if v as u8 != p6 => {
            set_screen_param(6, v as usize);
            return -11;
        }
        6 => {
            let mut s = if p5 & v != 0 && p5 & 0x1000000 == 0 { p5 | 0x1000000 } else { p5 & !0x1000000 };
            s &= 0xc1000000;
            set_screen_param(5, (v | s) as usize);
            return -11;
        }
        7 => {
            covered!("tel06_2 part exchange sort 7");
            set_screen_param(5, (part_exchange_sort(p5, 0x400, v as u8 == p7) | 0x400) as usize);
            set_screen_param(7, v as usize);
            return -11;
        }
        8 => {
            set_screen_param(5, (part_exchange_sort(p5, 0x800, v as u8 == p8) | 0x800) as usize);
            set_screen_param(8, v as usize);
            return -11;
        }
        3 => return -9,
        1 => {
            set_screen_param(3, v as usize);
            return -11;
        }
        2 => {
            covered!("tel06_2 part exchange player added");
            let offer = get_screen_param(0);
            let player = v as usize;
            let mut record = [0u8; 7];
            record[0..4].copy_from_slice(&read_u32(player).to_le_bytes());
            record[5] = 2;
            if read_usize(offer + 4) == 0 {
                let l = exe::crt_operator_new(0xe);
                write_usize(offer + 4, if l != 0 { exe::new_llist(l) } else { 0 });
            }
            exe::llist_add_tail(read_usize(offer + 4), record.as_ptr() as usize, 7);
            let cash = read_i32(offer + 0x1b).wrapping_sub(read_i32(player + staff::VALUE));
            write_i32(offer + 0x1b, if cash < 0 { 0 } else { cash });
            return -9;
        }
        0xa => {
            let open: unsafe extern "C" fn(usize, u32, i32, u32) = core::mem::transmute(0x8f6770usize);
            open(v as usize, 0, -1, 0);
            return -4;
        }
        _ => {}
    }
    let r = exe::menubar_handle_command(widget);
    if r != 0 {
        r
    } else {
        exe::menubar_handle_object_link(widget)
    }
}

/// Whether `part_exchange_screen_handle_event` only writes screen parameters for this widget (and can run twice).
unsafe fn part_exchange_input_params_only(widget: u32) -> bool {
    let w = widget as i16;
    let v = club_tabs_value(w) as u32;
    match club_tabs_action(w) {
        4 => get_screen_param(4) as u32 & v == 0,
        9 => get_screen_param(9) as u32 & v == 0,
        5 => v as u8 != get_screen_param(6) as u8,
        1 | 3 | 6 | 7 | 8 => true,
        _ => false,
    }
}

/// Comparer for `part_exchange_screen_handle_event`: the clicks that only write screen parameters side by side over
/// the display list and the parameters (`compare_by_display_list_with`) and the answer; adding a player (2) and the
/// transfer status screen (0xa) run only in the original, the answer checked (-9, -4); the menu bar, only the original.
pub(crate) unsafe extern "C" fn compare_part_exchange_screen_handle_event(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%);
    if part_exchange_input_params_only(widget) {
        let (mut a, mut b) = (0i32, 0i32);
        compare_by_display_list_with("part exchange screen input", &[], || a = o(widget), || b = part_exchange_screen_handle_event(widget));
        SCOREBOARD.record(a == b, || format!("part_exchange_screen_handle_event({}) (part exchange screen) returned {} and Rust {}", widget as i16, a, b));
        return a;
    }
    let plan = match club_tabs_action(widget as i16) {
        2 => Some(-9),
        0xa => Some(-4),
        _ => None,
    };
    let r = o(widget);
    if let Some(p) = plan {
        SCOREBOARD.record(r == p, || format!("part_exchange_screen_handle_event({}) (part exchange screen) returned {} and the plan {}", widget as i16, r, p));
    }
    r
}
