use super::*;

/// Writes value v into news item slot k (NEWSITEM set_slot).
pub(crate) unsafe fn set_news_slot(n: usize, k: i8, v: u32) {
    exe::newsitem_set_slot(n, k, v);
}

/// Writes the person's id and the +0x33 keys of +4, +8, +0xc into news slots k..k+3.
pub(crate) unsafe fn set_news_person_slots(n: usize, k: i8, p: Staff) {
    set_news_slot(n, k, read_u32(p.addr()));
    set_news_slot(n, k + 1, read_u32(p.first_name() + names::ID));
    set_news_slot(n, k + 2, read_u32(p.second_name() + names::ID));
    set_news_slot(n, k + 3, read_u32(p.common_name() + names::ID));
}

/// The rehearsal of the news items: the copy stops at the point where it would build the news item (which draws at
/// random and goes to the managers' inboxes).
pub(crate) static FINE_NEWS_DRY_RUN: AtomicBool = AtomicBool::new(false);
pub(crate) static FINE_NEWS_WOULD_BUILD: AtomicBool = AtomicBool::new(false);

/// Posts the news of a fined player's reaction: for a player under contract at a human club, news 0xbc6 with reason,
/// reaction, random(2), personality, period, contract type, future transfer and effects. SEH frame for the local news
/// item, no destructor. `post_fine_reaction_news(player, reason, reaction, deadline, efeitos)`, called from the fine
/// (0x5a5980, five calls). No player, no contract (`exe::contract_manager_get_contract`, on
/// `G_CONTRACT_LOOKUP_RESULT`), no club of his or no club contract (`exe::contract_manager_get_contract_for_club`):
/// nothing; player of a club (`club_job()`) without a human (`club_has_human_manager`): nothing. News 0xbc6 to the
/// contract's club: the player (id and keys), reason, reaction, random(2), the personality (`temperament()` +
/// `professionalism()` below 12, `loyalty()` below 6 and reputation `current_reputation()` above 5250), period 1, the
/// type 7 contract (`squad_status()`), the future transfer (`exe::transfer_manager_get_future_transfer`), the effects
/// and the game by the type (`squad_status()` 1 with more than 23 years, or 2).
/// Original: `post_fine_reaction_news` (0x5a74b0, Fine.cpp, stdcall, ret 0x14).
pub(crate) unsafe extern "stdcall" fn post_fine_reaction_news(player: usize, reason: u32, reaction: u32, deadline: u32, efeitos: u32) {
    let player = Staff(player);
    if player.is_null() {
        return;
    }
    let k = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player));
    write_usize(G_CONTRACT_LOOKUP_RESULT, k.addr());
    if k.is_null() {
        return;
    }
    let club = read_usize(G_CLUB_TABLE).wrapping_add(k.club().wrapping_mul(club::SIZE));
    if club == 0 {
        return;
    }
    let c = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, club));
    if c.is_null() {
        return;
    }
    let current = player.club_job();
    if !current.is_null() && !club_has_human_manager(current.addr()) {
        return;
    }
    covered!("cpz_9 news of the reaction to the fine");
    if FINE_NEWS_DRY_RUN.load(Relaxed) {
        FINE_NEWS_WOULD_BUILD.store(true, Relaxed);
        return;
    }
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    exe::new_newsitem(nt, 0xbc6, 0);
    set_news_person_slots(nt, 0, player);
    set_news_slot(nt, 4, reason as i8 as u32);
    set_news_slot(nt, 5, reaction as i8 as u32);
    let r = friendly_random(2);
    set_news_slot(nt, 6, r as u32);
    let p = (player.temperament() as i32) + (player.professionalism() as i32) < 0xc
        && (player.loyalty() as i32) < 6
        && player.player().current_reputation() > 0x1482;
    set_news_slot(nt, 7, p as u32);
    set_news_slot(nt, 8, (deadline as u8 == 1) as u32);
    let kind = c.squad_status() & 0xf0;
    set_news_slot(nt, 9, (kind == 0x70) as u32);
    set_news_slot(nt, 0xa, (exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), player, (0) as u32) != 0) as u32);
    set_news_slot(nt, 0xb, efeitos as i8 as u32);
    let game = (((player.age() as i8) as i32) > 0x17 && kind == 0x10) || kind == 0x20;
    set_news_slot(nt, 0xc, game as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), nt, club);
}

/// Posts the coach's feedback on a fine: with player and coach both having a record and a nation, and the player having
/// a club, news 0x1394 with the outcome and random(5). SEH frame for the local news item, no destructor.
/// `post_coach_fine_feedback_news(player, coach, outcome)`, called from the fines' daily pass (0x5a5820: the feedback
/// marked as due, more than 30 days at the club and no serious injury; the original leaves the Fine in ecx). Both with
/// a record (`player()`) and a nation (`nation()`), the player with a club (`club_job()`): news 0x1394 (the player and
/// the coach, id and keys; the outcome; random(5)) to the player's club.
/// Original: `post_coach_fine_feedback_news` (0x5a7f20, Fine.cpp, stdcall, ret 0xc).
pub(crate) unsafe extern "stdcall" fn post_coach_fine_feedback_news(player: usize, coach: usize, outcome: u32) {
    let coach = Staff(coach);
    let player = Staff(player);
    if player.is_null() || player.player().is_null() || player.nation().is_null() || player.club_job().is_null() {
        return;
    }
    if coach.is_null() || coach.player().is_null() || coach.nation().is_null() {
        return;
    }
    covered!("cpz_9 news of the coach return");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    exe::new_newsitem(nt, 0x1394, 0);
    set_news_person_slots(nt, 0, player);
    set_news_person_slots(nt, 4, coach);
    set_news_slot(nt, 8, outcome as i8 as u32);
    let r = friendly_random(5);
    set_news_slot(nt, 9, r as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), nt, player.club_job());
}

// simplification: the news rolls and goes to the managers’ inboxes (allocates): original only, with the copy taking the branch; without
// news item, both do nothing.

/// Rule: the Discipline Player screen opens for a person with the fine period at 1 (an official warning) and the reason
/// at 0.
///
/// `open_discipline_player_screen(&person)`: `open_screen` with the builder 0x5a83e0, the handler 0x5a8990, single
/// instance, the no-op leave routine 0x5a8f60 and 0; when it opens, parameter 0 = the person's id (the first dword of
/// the record the argument points to), 1 = 1 (the period) and 3 = 0 (the reason).
/// Original: `open_discipline_player_screen` (0x5a8380, Fine.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_discipline_player_screen(person: usize) {
    covered!("tel35_1 open discipline");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x5a83e0usize, 0x5a8990usize, 1, 0x5a8f60usize, 0) == 0 {
        return;
    }
    set_screen_param(0, read_u32(person) as usize);
    set_screen_param(1, 1);
    set_screen_param(3, 0);
}

/// Rule: the Discipline Player screen shows the person's name in his club's colours, the sentence with a link to the
/// person, the period chooser ("-", the period, "+"; "-" greyed at a warning, "+" greyed at two weeks), the five
/// reasons (the chosen one in yellow) and Cancel / Confirm.
///
/// `draw_discipline_player_screen()`: the person of parameter 0 (`staff::SIZE` × id into the staff table); with no club
/// (`staff::CLUB_JOB`) nothing is built. Otherwise the period (parameter 1) and, as the reason, the low byte of
/// parameter 2 (the original reads 2, where this routine keeps the "-" widget, not 3 where the handler keeps the
/// reason: a defect kept). The person's background picture (`background_set_staff_picture`, 0, 0); the title: his name
/// (`database_format_staff_name`, 500 bytes, style 1) in the club's kit colours (kit 1); the subtitle "Discipline
/// Player". Area 1 (0x6e, 0x8c)-(0x30c, 0x221), seven columns (2, 20, 2, 10, 2, 4, 3), rows = 0x195 / the height of
/// font 3, kind 2: the sentence with the person's name (a Text with the link 0x7d7 to the person) drawn wrapped from
/// row 0 (font 3, pale grey); two rows below, "Disciplinary action:" (column 1), "-" in column 2 (a grey text at period
/// ≤ 1, otherwise a button id 2), its widget kept in parameter 2, the period's text in column 3 ("Official Warning", or
/// "<n> Week Fine"; in language 10 a fixed text for period 2 and a plain "%d" one otherwise), "+" in column 4 (a grey
/// text at period ≥ 3, otherwise a button id 1); two rows below "Reason For Disciplinary Action:". Area 2, the same
/// rectangle, two columns (2, 17), kind 1: from two rows below, the five reasons (`Fine::reason_text` k, 2000 bytes)
/// as buttons id 3 with data k, the chosen one yellow and the others grey. The menu bar (4, 0) and the button area
/// (0x64, 0x22b)-(0x316, 0x24e) (columns 3, 1; the club's background colour) with "Cancel" (id 4) and "Confirm" (id 5).
/// Original: `draw_discipline_player_screen` (0x5a83e0, Fine.cpp, cdecl).
pub(crate) unsafe extern "C" fn draw_discipline_player_screen() {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let columns: [u8; 7] = [2, 0x14, 2, 0xa, 2, 4, 3];
    let buttons: [u8; 4] = [3, 1, 2, 0x11];
    let mut tx = [0u32; 8];
    let t = tx.as_mut_ptr() as usize;
    exe::new_text(tx.as_mut_ptr() as *mut usize, 0usize);
    let person = Staff(discipline_person());
    let club = person.club_job().addr();
    if club == 0 {
        exe::drop_text(tx.as_mut_ptr() as *mut usize);
        return;
    }
    covered!("tel35_1 draw discipline");
    let period = get_screen_param(1) as u8 as i8 as i32;
    let reason = get_screen_param(2) as u8;
    exe::background_set_staff_picture(person, 0, 0);
    let (mut fore, mut back) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut fore, &mut back);
    let mut name = [0u8; 500];
    exe::database_format_staff_name(person, name.as_mut_ptr() as usize, 0x1f4, 1);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back & 0xffff, fore & 0xffff, name.as_ptr() as usize);
    text(JOB_TEXT, DISCIPLINE_TITLE);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let rows = (0x195 / exe::font_get_height(3)) as u8 as u32;
    let d = current_human_display();
    let area = tel26_add_area(d, [0x6e, 0x8c, 0x30c, 0x221, 7, columns.as_ptr() as u32, rows, 0, 2, 0, u32::MAX]);
    text(JOB_TEXT, 0x9cc08c, 0x64, name.as_ptr());
    exe::text_set_string(t, JOB_TEXT as *const u8);
    exe::text_add_link(t, name.as_ptr(), 0x7d7, person);
    let mut line = 0i32;
    tel35_text_draw_wrapped(t, area, 0x6e, 0x30c, &mut line, 3, tel35_colour(G_GUI_COLOUR_PALE_GREY), 0, 0);
    let mut row = line + 2;
    let pale = tel35_colour(G_GUI_COLOUR_PALE_GREY);
    let grey = tel35_colour(G_GUI_COLOUR_GREY);
    let blue = tel35_colour(G_GUI_COLOUR_DARK_BLUE);
    text(JOB_TEXT, 0x9cc074);
    tel26_area_button([area, 1, row as u32, 1, 0, 1, 3, pale, JOB_TEXT as u32, 0, 0]);
    let minus = if period <= 1 {
        tel26_area_text([area, 2, row as u32, 0x30, blue, 0xc, 3, grey, 0x98ba0c])
    } else {
        tel26_area_button([area, 2, row as u32, 0x30, blue, 0xc, 3, pale, 0x98ba0c, 2, 0])
    };
    set_screen_param(2, minus as i16 as isize as usize);
    if period == 1 {
        text(JOB_TEXT, DISCIPLINE_WARNING);
    } else if exe::langlib_get_language() == 0xa {
        if period == 2 {
            tel26_copy_text(JOB_TEXT as *mut u8, DISCIPLINE_ONE_WEEK_L10 as *const u8);
        } else {
            format(JOB_TEXT, DISCIPLINE_WEEK_FINE_L10, period - 1);
        }
    } else {
        text(JOB_TEXT, DISCIPLINE_WEEK_FINE, period - 1);
    }
    tel26_area_text([area, 3, row as u32, 1, tel35_colour(G_GUI_COLOUR_BLACK), 0xc, 3, pale, JOB_TEXT as u32]);
    if period >= 3 {
        tel26_area_text([area, 4, row as u32, 0x30, blue, 0xc, 3, grey, 0x9afe7c]);
    } else {
        tel26_area_button([area, 4, row as u32, 0x30, blue, 0xc, 3, pale, 0x9afe7c, 1, 0]);
    }
    row += 2;
    text(JOB_TEXT, 0x9cbffc);
    tel26_area_button([area, 1, row as u32, 1, 0, 1, 3, pale, JOB_TEXT as u32, 0, 0]);
    row += 2;
    let reasons = tel26_add_area(d, [0x6e, 0x8c, 0x30c, 0x221, 2, buttons.as_ptr().wrapping_add(2) as u32, rows, 0, 1, 0, u32::MAX]);
    for k in 0..5u8 {
        exe::fine_reason_text(read_usize(G_FINE), k as u32, JOB_TEXT, 0x7d0, 0);
        let colour = if reason != k { grey } else { tel35_colour(G_GUI_COLOUR_YELLOW) };
        tel26_area_button([reasons, 1, row as u32, 1, 0, 1, 3, colour, JOB_TEXT as u32, 3, k as u32]);
        row += 1;
    }
    exe::menubar_build(4, 0);
    let d = current_human_display();
    let bar = tel26_add_area(d, [0x64, 0x22b, 0x316, 0x24e, 2, buttons.as_ptr() as u32, 1, 0, 1, back & 0xffff, u32::MAX]);
    text(JOB_TEXT, 0x98a1a0);
    tel26_area_button([bar, 0, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 4, 0]);
    text(JOB_TEXT, 0x98a198);
    tel26_area_button([bar, 1, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 5, 0]);
    exe::drop_text(tx.as_mut_ptr() as *mut usize);
}

/// Rule: on the Discipline Player screen "-" and "+" move the period between a warning and a two-week fine (greying
/// the button at the end), a reason click colours it yellow, Confirm fines the person and closes, Cancel closes.
///
/// `discipline_player_screen_input(item)`: in a network game (`[0xb77c84]`) the pending command argument is cleared and
/// -9 comes back. The widget's id (`tel26_item_id`): 2 ("-"): the period (parameter 1) goes down while above 1 and is
/// written back; the "-" widget (parameter 2) gets flags 6, grey and data 0 when the period reaches 1; the period text
/// (the next widget) is rewritten; "+" (the one after) gets flags 2, pale grey and id 1; redraw, 0. 1 ("+"): the period
/// goes up while below 3; "-" gets flags 2, pale grey and id 2; the text; "+" gets flags 6, grey and data 0 when it
/// reaches 3; redraw, 0. 3 (a reason): the widget's data k; the five reason widgets (parameter 2 + 4 ...) recoloured,
/// yellow the chosen one (k above 3 selects the fifth) and grey the others; parameter 3 = k; redraw, 0. 5 (Confirm):
/// `Fine::fine_player(person, period, reason)`, -9. 4 (Cancel): -9. Otherwise the menu bar's command, then its object
/// link.
/// Original: `discipline_player_screen_input` (0x5a8990, Fine.cpp, cdecl).
pub(crate) unsafe extern "C" fn discipline_player_screen_input(raw: u32) -> i32 {
    let person = discipline_person();
    let mut period = get_screen_param(1) as i32;
    let reason = get_screen_param(3);
    if read_u32(TEL35_NETWORK_FLAG) != 0 {
        write_u32(G_PENDING_COMMAND_ARG, 0);
        return -9;
    }
    let item = raw as i16;
    let sm = G_SCREEN_MANAGER;
    match tel26_item_id(item) {
        2 => {
            covered!("tel35_1 discipline minus");
            if period > 1 {
                period -= 1;
            }
            set_screen_param(1, period as usize);
            let mut w = get_screen_param(2) as u32;
            if period <= 1 {
                exe::screenmanager_set_widget_flags(sm, w as usize, 6);
                exe::screenmanager_set_widget_text_colour(sm, w, tel35_colour(G_GUI_COLOUR_GREY));
                exe::screenmanager_set_widget_data(sm, w, 0usize);
            }
            w = w.wrapping_add(1);
            discipline_period_text(period);
            exe::screenmanager_set_widget_text(sm, w, JOB_TEXT);
            w = w.wrapping_add(1);
            exe::screenmanager_set_widget_flags(sm, w as usize, 2);
            exe::screenmanager_set_widget_text_colour(sm, w, tel35_colour(G_GUI_COLOUR_PALE_GREY));
            exe::screenmanager_set_widget_id(sm, w, 1);
        }
        1 => {
            covered!("tel35_1 discipline plus");
            if period < 3 {
                period += 1;
            }
            set_screen_param(1, period as usize);
            let mut w = get_screen_param(2) as u32;
            exe::screenmanager_set_widget_flags(sm, w as usize, 2);
            exe::screenmanager_set_widget_text_colour(sm, w, tel35_colour(G_GUI_COLOUR_PALE_GREY));
            exe::screenmanager_set_widget_id(sm, w, 2);
            w = w.wrapping_add(1);
            discipline_period_text(period);
            exe::screenmanager_set_widget_text(sm, w, JOB_TEXT);
            w = w.wrapping_add(1);
            if period >= 3 {
                exe::screenmanager_set_widget_flags(sm, w as usize, 6);
                exe::screenmanager_set_widget_text_colour(sm, w, tel35_colour(G_GUI_COLOUR_GREY));
                exe::screenmanager_set_widget_data(sm, w, 0usize);
            }
        }
        3 => {
            covered!("tel35_1 discipline reason");
            let k = tel26_item_value(item);
            let chosen = k.min(4);
            let first = (get_screen_param(2) as u32).wrapping_add(4);
            for j in 0..5u32 {
                let colour = if j == chosen { G_GUI_COLOUR_YELLOW } else { G_GUI_COLOUR_GREY };
                exe::screenmanager_set_widget_text_colour(sm, first.wrapping_add(j), tel35_colour(colour));
            }
            set_screen_param(3, k as usize);
        }
        5 => {
            covered!("tel35_1 discipline confirm");
            exe::fine_fine_player(read_usize(G_FINE), person, period as u32, reason as u32);
            return -9;
        }
        4 => return -9,
        _ => return manager_menu_bar(raw),
    }
    exe::screenmanager_redraw_screen(sm, 0, 0);
    0
}

/// Comparer for `open_discipline_player_screen`.
pub(crate) unsafe extern "C" fn compare_open_discipline_player_screen(person: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize)>(IDX_OPEN_DISCIPLINE_PLAYER_SCREEN)(person)
}

/// Comparer for `draw_discipline_player_screen`: the screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_draw_discipline_player_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_DRAW_DISCIPLINE_PLAYER_SCREEN);
    compare_by_display_list_with("Discipline Player screen (draw_discipline_player_screen)", &[], || o(), || draw_discipline_player_screen());
}

/// Comparer for `discipline_player_screen_input`.
pub(crate) unsafe extern "C" fn compare_discipline_player_screen_input(item: u32) -> i32 {
    // simplification: the fine, the redraw and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_DISCIPLINE_PLAYER_SCREEN_INPUT)(item)
}

/// `Text::draw_wrapped(slot, left, right, &line, font, colour, style, flags)` (0x4aeaf0, thiscall, ret 0x20): lays a
/// Text's words out as area rows from `line` on, advancing it.
pub(crate) unsafe fn tel35_text_draw_wrapped(text: usize, area: u32, left: i32, right: i32, line: *mut i32, font: u32, colour: u32, style: u32, flags: u32) -> i32 {
    exe::text_draw_wrapped(text, area, left, right, line, font, colour, style, flags)
}

/// The person of screen parameter 0 (a staff id): the staff table entry.
pub(crate) unsafe fn discipline_person() -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add((get_screen_param(0) as u32).wrapping_mul(staff::SIZE as u32) as usize)
}

/// The fine period's text into the screens' text buffer: 1 "Official Warning", otherwise "<n> Week Fine" with
/// n = period − 1 (langlib).
pub(crate) unsafe fn discipline_period_text(period: i32) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if period == 1 {
        text(JOB_TEXT, DISCIPLINE_WARNING);
    } else {
        text(JOB_TEXT, DISCIPLINE_WEEK_FINE, period - 1);
    }
}

/// Fine.cpp's discipline texts: "Discipline Player", "Official Warning", "<%d - Fine period> Week Fine" and the
/// language-10 variants (a plain "%d" week text and a fixed one-week text).
pub(crate) const DISCIPLINE_TITLE: usize = 0x9cc0e8;

pub(crate) const DISCIPLINE_WARNING: usize = 0x9cc060;

pub(crate) const DISCIPLINE_WEEK_FINE: usize = 0x9cc01c;

pub(crate) const DISCIPLINE_WEEK_FINE_L10: usize = 0x9cc03c;

pub(crate) const DISCIPLINE_ONE_WEEK_L10: usize = 0x9cc050;

/// Reads the Fine manager from the save's "fine.dat": without a save or the item, 0. The fine count (dword), a new list
/// (`operator new(0xe)` and `LLIST::LLIST`, null when the allocation fails) in +0 with that many 14-byte fines; the
/// length of the per-club fine count table (dword), the table (`malloc`, not checked) in +8 and its bytes; a second new
/// list in +4 with the coach feedback count (dword) of 17-byte records. The item closed, 1. A short read closes the
/// item and answers 0 (what was built stays: the constructor destroys the lists and builds new ones).
/// Original: `Fine::load` (0x5a5180, Fine.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fine_load(this: usize, save: usize) -> u8 {
    if save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, FINE_DAT, 0, 0x16, 4);
    if z == 0 {
        return 0;
    }
    covered!("sx09c_2 load fine.dat");
    let mut count = 0i32;
    if exe::zread(&mut count as *mut i32 as usize, 4, 1, z) != 1 {
        exe::zclose(&mut z);
        return 0;
    }
    write_usize(this, club_history_new_list());
    if !fine_read_records(z, read_usize(this), count, 0xe) {
        exe::zclose(&mut z);
        return 0;
    }
    let mut length = 0u32;
    if exe::zread(&mut length as *mut u32 as usize, 4, 1, z) != 1 {
        exe::zclose(&mut z);
        return 0;
    }
    let counts = exe::crt_malloc(length);
    write_usize(this + 8, counts); // sx09c_2: the per-club fine counts
    if exe::zread(counts, 1, length, z) != length {
        exe::zclose(&mut z);
        return 0;
    }
    write_usize(this + 4, club_history_new_list());
    let mut reports = 0i32;
    if exe::zread(&mut reports as *mut i32 as usize, 4, 1, z) != 1 || !fine_read_records(z, read_usize(this + 4), reports, 0x11) {
        exe::zclose(&mut z);
        return 0;
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `fine_load`: it reads the save item and allocates, so it cannot run twice; only the original runs.
/// The copy is proven by loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_fine_load(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_FINE_LOAD)(this, save)
}

/// Writes the Fine manager to the save's "fine.dat": the fine count (the list's i16 as a dword), the 14-byte fines,
/// the club count (`G_CLUB_COUNT`, the global itself), the per-club fine count table (that many bytes), the coach
/// feedback count and its 17-byte records; the item closed, 1. Without a save (line 0x109) or the item (0x111): the
/// Fine.cpp "Error" box and 0. A short write or a missing node: the box (lines 0x119, 0x123, 0x129, 0x132, 0x13a,
/// 0x143, 0x14d, 0x153), the item closed and 0.
/// Original: `Fine::save` (0x5a53d0, Fine.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fine_save(this: usize, save: usize) -> u8 {
    if save == 0 {
        show_error_box(FINE_CPP_PATH, 0x109);
        return 0;
    }
    let mut z = exe::zopen(save, FINE_DAT, 1, 0x16, 4);
    if z == 0 {
        show_error_box(FINE_CPP_PATH, 0x111);
        return 0;
    }
    covered!("sx09c_2 save fine.dat");
    let write = |z: usize| -> Result<(), u32> {
        let fines = Llist(read_usize(this));
        let n = fines.count() as i32;
        if exe::zwrite(&n as *const i32 as usize, 4, 1, z) != 1 {
            return Err(0x119);
        }
        fine_write_records(z, fines, 0xe, 0x123, 0x129)?;
        if exe::zwrite(G_CLUB_COUNT, 4, 1, z) != 1 {
            return Err(0x132);
        }
        if exe::zwrite(read_usize(this + 8), 1, read_u32(G_CLUB_COUNT), z) != read_u32(G_CLUB_COUNT) { // sx09c_2: the fine counts
            return Err(0x13a);
        }
        let feedback = Llist(read_usize(this + 4));
        let n = feedback.count() as i32;
        if exe::zwrite(&n as *const i32 as usize, 4, 1, z) != 1 {
            return Err(0x143);
        }
        fine_write_records(z, feedback, 0x11, 0x14d, 0x153)
    };
    let r = write(z);
    if let Err(row) = r {
        show_error_box(FINE_CPP_PATH, row);
    }
    exe::zclose(&mut z);
    r.is_ok() as u8
}

/// Comparer for `fine_save`: it writes into the save container, so it cannot run twice; only the original runs. The
/// copy is proven by the save proof (the .qst written with and without the batch, item by item).
pub(crate) unsafe extern "thiscall" fn compare_fine_save(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_FINE_SAVE)(this, save)
}

/// Reads `count` records of `size` bytes from the open save item into the list `list` (`LLIST::add_tail` each); false at
/// the first short read (the records read so far stay in the list).
pub(crate) unsafe fn fine_read_records(z: usize, list: usize, count: i32, size: u32) -> bool {
    let mut record = [0u8; 0x11];
    for _ in 0..count.max(0) {
        if exe::zread(record.as_mut_ptr() as usize, size, 1, z) != 1 {
            return false;
        }
        exe::llist_add_tail(list, record.as_ptr() as usize, size);
    }
    true
}

/// Writes the records of `list` (from its head, `count` re-read each step) to the open save item, `size` bytes each;
/// the line of the "Error" box when a node is missing (`missing`) or a write is short (`short`).
pub(crate) unsafe fn fine_write_records(z: usize, list: Llist, size: u32, missing: u32, short: u32) -> Result<(), u32> {
    list.set_cursor(list.head());
    let mut k = 0i32;
    while k < list.count() as i32 {
        let record = exe::llist_get_next(list);
        if record == 0 {
            return Err(missing);
        }
        if exe::zwrite(record, size, 1, z) != 1 {
            return Err(short);
        }
        k += 1;
    }
    Ok(())
}

/// The save item "fine.dat".
pub(crate) const FINE_DAT: usize = 0x9ca1d4;

/// The source path of Fine.cpp in the "Error" box ("E:\dev\CM3\cm3 00-01\cm3\code\Fine.cpp").
pub(crate) const FINE_CPP_PATH: usize = 0x9ca1e0;

/// Halves the club's count of player fines (at the new season): the signed byte × 0.5 (`rdata::F64_0_5`), truncated
/// (`ftol`). A null club: nothing.
/// Original: `Fine::halve_club_fine_count` (0x5a5930, Fine.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn halve_club_fine_count(this: usize, club: usize) {
    if club == 0 {
        return;
    }
    let e = club_fine_count(this, club);
    write_u8(e, ftol(read_i8(e) as f64 * read_f64(rdata::F64_0_5)) as u8);
}

/// Clears the club's count of player fines (when its manager leaves). A null club: nothing.
/// Original: `Fine::clear_club_fine_count` (0x5a5960, Fine.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn clear_club_fine_count(this: usize, club: usize) {
    if club == 0 {
        return;
    }
    write_u8(club_fine_count(this, club), 0);
}

/// Comparer for `halve_club_fine_count`: side by side over the club's byte.
pub(crate) unsafe extern "thiscall" fn compare_halve_club_fine_count(this: usize, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_HALVE_CLUB_FINE_COUNT);
    if club == 0 {
        return o(this, club);
    }
    let e = club_fine_count(this, club);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(e, 1)], || o(this, club), || halve_club_fine_count(this, club));
    SCOREBOARD.record(same, || format!("halve_club_fine_count(club {:#x}) original={:x?} rust={:x?}", club, mo, mr));
}

/// Comparer for `clear_club_fine_count`: side by side over the club's byte.
pub(crate) unsafe extern "thiscall" fn compare_clear_club_fine_count(this: usize, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_CLEAR_CLUB_FINE_COUNT);
    if club == 0 {
        return o(this, club);
    }
    let e = club_fine_count(this, club);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(e, 1)], || o(this, club), || clear_club_fine_count(this, club));
    SCOREBOARD.record(same, || format!("clear_club_fine_count(club {:#x}) original={:x?} rust={:x?}", club, mo, mr));
}

/// Writes the headline or the body of news item 6020 (0x1784): the board's warning about a manager who fines his
/// players too often.
///
/// `write_board_fine_warning_news_text(news, full, out)`: the headline "The board make concerns known" (0x9cba54); the
/// body by the news item's slot 0 (`exe::newsitem_get_slot`): set, the first warning ("The board are concerned that
/// your frequent fining of players will damage the squads morale...", 0x9cbb3c), clear, the last one ("...we may be
/// forced to look for a new manager", 0x9cba74). Formatted into `NEWS_TEXT_BUFFER` and set as the text of `out`
/// (`exe::text_set_string`).
/// Original: `write_board_fine_warning_news_text` (0x5a7ed0, Fine.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_board_fine_warning_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let template = if full == 0 {
        0x9cba54 // "The board make concerns known"
    } else if exe::newsitem_get_slot(news, 0) != 0 { // sx09b_8: the first warning
        covered!("sx09b_8 board fine warning, the first");
        0x9cbb3c
    } else {
        covered!("sx09b_8 board fine warning, the last");
        0x9cba74
    };
    story_lang!(NEWS_TEXT_BUFFER, template);
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
}

/// Comparer for `write_board_fine_warning_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_board_fine_warning_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_BOARD_FINE_WARNING_NEWS_TEXT);
    let what = || format!("write_board_fine_warning_news_text(news {:#x}, full {})", news, full);
    compare_text_writer(out, || o(this, news, full, out), |t| write_board_fine_warning_news_text(this, news, full, t), what);
}

/// The club's fine count: the byte of the club (by its id) in the Fine object's per-club table (the pointer at +8).
pub(crate) unsafe fn club_fine_count(this: usize, club: usize) -> usize {
    read_usize(this + 8).wrapping_add(read_usize(club)) // +8: the per-club fine count table
}

/// Drops the first expired fine and, when a coach feedback is due, posts it.
///
/// `Fine::daily_update()`: the fine list (the 14-byte records at `[this]`) from its head: the first of the first
/// `count` records whose date (+6) today is past (`exe::date_operator_3e` on `G_CURRENT_DATE`) leaves the list
/// (`exe::llist_remove`); a record walk that runs out of nodes ends the whole pass. Then the coach feedback list (the
/// 17-byte records at `[this + 4]`) from its head: the first of the first `count` records whose date (+9) today is
/// past: when the person of +0 (the club's assistant manager when the fine was recorded) joined his club more than 30
/// days ago (`exe::date_days_since` on `date_joined_club()`) and his injury status (`exe::injury_get_player_injury_status`
/// on `G_INJURY`) is not 2, `post_coach_fine_feedback_news(+0, the player of +4, the outcome byte +8)`; then the
/// feedback list removes the node whose data equals, byte for byte, the original's stale stack slot (see below), and
/// the pass ends.
///
/// Defect of the original: the removal of the feedback record passes a stack slot that still holds the last fine the
/// first walk looked at (freed already when it was the expired one), or the Fine object itself when the fine list was
/// empty, instead of the feedback record. `LLIST::remove` compares node data byte for byte, so the feedback record
/// stays and its news is posted again every day while the conditions hold. The copy passes the same slot.
/// Original: `Fine::daily_update` (0x5a5820, Fine.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn fine_daily_update(this: usize) {
    let fines = Llist(read_usize(this));
    fines.set_cursor(fines.head());
    let mut last_fine = this; // the original's stack slot: `push ecx` (this), then each fine the walk reads
    if fines.count() > 0 {
        let mut k = 0i32;
        loop {
            let fine = exe::llist_get_next(fines);
            last_fine = fine;
            if fine == 0 {
                return;
            }
            if exe::date_operator_3e(G_CURRENT_DATE, read_u32(fine + 6), read_u32(fine + 0xa)) != 0 { // +6: the fine's end date
                covered!("sx09c_1 expired fine dropped");
                exe::llist_remove(fines, fine);
                break;
            }
            k += 1;
            if k >= fines.count() as i32 {
                break;
            }
        }
    }
    let feedback = Llist(read_usize(this + 4));
    feedback.set_cursor(feedback.head());
    if feedback.count() <= 0 {
        return;
    }
    let mut k = 0i32;
    loop {
        let entry = exe::llist_get_next(feedback);
        if entry == 0 {
            return;
        }
        if exe::date_operator_3e(G_CURRENT_DATE, read_u32(entry + 9), read_u32(entry + 0xd)) != 0 { // +9: the report date
            let first = staff_at(read_i32(entry) as isize); // +0: the club's assistant manager at the fine
            let player = staff_at(read_i32(entry + 4) as isize); // +4: the fined player
            if exe::date_days_since(G_CURRENT_DATE, first.date_joined_club() as u32, first.date_joined_club_leap_year() as u32) > 0x1e
                && exe::injury_get_player_injury_status(read_usize(G_INJURY), first) != 2 { // sx09c_1: the report is posted
                covered!("sx09c_1 coach feedback posted");
                post_coach_fine_feedback_news(first.addr(), player.addr(), read_u8(entry + 8) as u32);
            }
            exe::llist_remove(feedback, last_fine); // the original's defect: the stale slot, not `entry`
            return;
        }
        k += 1;
        if k >= feedback.count() as i32 {
            return;
        }
    }
}

/// Comparer for `fine_daily_update`: it frees list nodes and posts news, so only the original runs; the plan is
/// checked afterwards: the fine list loses one node exactly when the walk finds an expired fine (counted before), and
/// the feedback list keeps its nodes when none is due.
pub(crate) unsafe extern "thiscall" fn compare_fine_daily_update(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_FINE_DAILY_UPDATE);
    let due = |list: Llist, date: usize| {
        let mut node = list.head();
        let mut k = 0i32;
        while node != 0 && k < list.count() as i32 {
            let d = read_usize(node + llist_node::DATA);
            if exe::date_operator_3e(G_CURRENT_DATE, read_u32(d + date), read_u32(d + date + 4)) != 0 {
                return true;
            }
            node = read_usize(node + llist_node::NEXT);
            k += 1;
        }
        false
    };
    let (fines, feedback) = (Llist(read_usize(this)), Llist(read_usize(this + 4)));
    let (n1, n2) = (fines.count(), feedback.count());
    let (expired, report) = (due(fines, 6), due(feedback, 9));
    o(this);
    let ok = fines.count() == n1 - expired as i16 && (report || feedback.count() == n2);
    SCOREBOARD.record(ok, || format!("fine_daily_update: fines {} -> {} (expired {}), feedback {} -> {} (due {})",
        n1, fines.count(), expired, n2, feedback.count(), report));
}

/// Builds the Fine manager: the fine list and the coach feedback list, from the save's fine.dat (`Fine::load`) or new
/// and empty, and the per-club fine count table.
///
/// `Fine::Fine(save)`: both list pointers null; with a save, `Fine::load` (`FN_FINE_LOAD`); when it answers, nothing else (it built the
/// lists and the table). Otherwise (no save, or the load failed): with a save, the lists the load may have left are
/// destroyed (`LLIST::~LLIST` and `operator delete`, the pointers kept), and in both cases two new empty lists
/// (`operator new(0xe)` and `LLIST::LLIST`, null when the allocation fails) and the fine count table: one zeroed byte
/// per club (`calloc(G_CLUB_COUNT, 1)`) in +8. Returns the object.
/// Original: `Fine::Fine` (0x5a4fc0, Fine.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_fine(this: usize, save: usize) -> usize {
    write_usize(this, 0);
    write_usize(this + 4, 0);
    if save != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_FINE_LOAD);
        if load(this, save) != 0 {
            return this;
        }
        covered!("sx09c_1 fine.dat not loaded");
        for field in [0usize, 4] {
            let list = read_usize(this + field);
            if list != 0 {
                exe::drop_llist(list);
                exe::crt_operator_delete(list);
            }
        }
    }
    write_usize(this, club_history_new_list()); // the fine list
    write_usize(this + 4, club_history_new_list()); // the coach feedback list
    write_usize(this + 8, exe::crt_calloc(read_u32(G_CLUB_COUNT), 1)); // sx09c_1: the per-club fine counts
    this
}

/// Comparer for `new_fine`: it allocates (and, with a save, reads the save item), so it cannot run twice; only the
/// original runs. The copy is proven by the new career (Rust mode).
pub(crate) unsafe extern "thiscall" fn compare_new_fine(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_NEW_FINE)(this, save)
}

/// Frees the Fine manager's data: each list present (`LLIST::~LLIST` and `operator delete`) and the fine count table
/// (`free`), each pointer set to null.
/// Original: `Fine::free_data` (0x5a5120, Fine.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn fine_free_data(this: usize) {
    for field in [0usize, 4] {
        let list = read_usize(this + field);
        if list != 0 {
            exe::drop_llist(list);
            exe::crt_operator_delete(list);
            write_usize(this + field, 0);
        }
    }
    let counts = read_usize(this + 8);
    if counts != 0 {
        exe::crt_free(counts);
        write_usize(this + 8, 0);
    }
}

/// Comparer for `fine_free_data`: it frees, so it cannot run twice; only the original runs, and the three pointers must
/// end null.
pub(crate) unsafe extern "thiscall" fn compare_fine_free_data(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_FINE_FREE_DATA)(this);
    SCOREBOARD.record(read_usize(this) == 0 && read_usize(this + 4) == 0 && read_usize(this + 8) == 0,
        || format!("fine_free_data({:#x}): pointers left {:#x} {:#x} {:#x}", this, read_usize(this), read_usize(this + 4), read_usize(this + 8)));
}

/// `Fine::load` (0x5a5180, Fine.cpp; thiscall(fine, save), ret 4, the answer in al): reads fine.dat (sx09c_2).
pub(crate) const FN_FINE_LOAD: usize = 0x5a5180;
