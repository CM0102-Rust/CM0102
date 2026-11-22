// ---------- batch %%LEVA%%: the menu bar's records, lists and the manager chat screen (Menubar.cpp) ----------

/// The manager chat log: one 0x7b8-byte page per human (16), 19 lines of 0x68 bytes (the text, then at +0x66 the
/// sender's human index, -1 for none); the line counts (words), the "page changed" flags (dwords) and the "set up" flag.
const CHAT_PAGES: usize = 0xb6b810;
const CHAT_PAGE_SIZE: usize = 0x7b8;
const CHAT_LINE_SIZE: usize = 0x68;
const CHAT_LINE_SENDER: usize = 0x66;
const CHAT_LINES: usize = 0x13;
const CHAT_LINE_COUNTS: usize = 0xb742f0;
const CHAT_CHANGED: usize = 0xb6b7d0;
const CHAT_SET_UP: usize = 0xb74314;
/// The colour word at 0xaebde8 (no name yet): the empty chat lines and the text entry.
const CHAT_COLOUR_BDE8: usize = 0xaebde8;
/// The colour word at 0xae31aa (no name yet): the menu entries' first colour.
const MENU_ENTRY_COLOUR: usize = 0xae31aa;

/// A chat line (`line` of the human `human`'s page).
fn chat_line(human: i32, line: i32) -> usize {
    (CHAT_PAGES as isize + human as isize * CHAT_PAGE_SIZE as isize + line as isize * CHAT_LINE_SIZE as isize) as usize
}

/// The human index of the slot manager (his staff id − the staff count + 16, as a word).
unsafe fn chat_human() -> i16 {
    (read_u16(tel26_slot_manager().addr()).wrapping_sub(read_u16(G_STAFF_COUNT))).wrapping_add(0x10) as i16
}

/// A menu entry item (`Display::add_item` kind `kind`, flags 0x10) in the menu entry colours, with its text, id, data
/// and area.
unsafe fn menu_list_entry(kind: u32, text: u32, id: u32, data: u32, area: u32) -> u32 {
    tel26_add_item(current_human_display(), [kind, 0, 0, 0, 0, 0, 0, 0x10, tel35_colour(MENU_ENTRY_COLOUR), tel35_colour(G_GUI_COLOUR_YELLOW), 1, 1,
        tel35_colour(G_GUI_COLOUR_BLACK), text, 0, id, data, area])
}

/// Rule: at start-up the 16 human manager menu bar records are built and their destruction is registered.
///
/// `menubar_init_human_manager_records()`: the runtime's vector constructor over `MENUBAR_RECORDS` (16 records of
/// `MENUBAR_RECORD_SIZE`, `HUMAN_MANAGER_RECORD::HUMAN_MANAGER_RECORD`, the no-op destructor 0x5a53c0), then
/// `atexit(menubar_destroy_human_manager_records)`.
/// Original: `menubar_init_human_manager_records` (0x75da50, Menubar.cpp, cdecl).
pub(crate) unsafe extern "C" fn menubar_init_human_manager_records() {
    exe::crt_vector_constructor_iterator(MENUBAR_RECORDS, MENUBAR_RECORD_SIZE as u32, 0x10, 0x75da80usize, 0x5a53c0usize);
    exe::crt_atexit(0x75dae0usize);
}

/// Rule: a fresh human manager menu bar record has no manager, no mode and no stamps.
///
/// `HUMAN_MANAGER_RECORD::HUMAN_MANAGER_RECORD()`: +0 = 0 (byte), +1 = 0, +5 = 0 (byte), +6 = 0, +0xa = -1 (word),
/// +0xc = 0 (byte), +0xd4 = -1 (word), +0xd6 = 1 (byte), +0xd7 = -1 (word), +0xd9 = 0 (byte), +0xda..+0xf2 seven
/// dwords 0; returns the record.
/// Original: `HUMAN_MANAGER_RECORD::HUMAN_MANAGER_RECORD` (0x75da80, Menubar.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn human_manager_record_human_manager_record(this: usize) -> usize {
    write_u8(this, 0);
    write_u32(this + 1, 0);
    write_u8(this + 5, 0);
    write_u32(this + 6, 0);
    write_u16(this + 0xa, 0xffff);
    write_u8(this + 0xc, 0);
    write_u16(this + 0xd4, 0xffff);
    write_u8(this + 0xd6, 1);
    write_u16(this + 0xd7, 0xffff);
    write_u8(this + 0xd9, 0);
    for k in 0..7 {
        write_u32(this + 0xda + 4 * k, 0);
    }
    this
}

/// Rule: at exit the 16 human manager menu bar records are destroyed.
///
/// `menubar_destroy_human_manager_records()`: the runtime's vector destructor over `MENUBAR_RECORDS` (16 records,
/// the no-op destructor 0x5a53c0).
/// Original: `menubar_destroy_human_manager_records` (0x75dae0, Menubar.cpp, cdecl).
pub(crate) unsafe extern "C" fn menubar_destroy_human_manager_records() {
    exe::crt_vector_destructor_iterator(MENUBAR_RECORDS, MENUBAR_RECORD_SIZE as u32, 0x10, 0x5a53c0usize);
}

/// Rule: a menu separator is an empty line item (flags 0x1000010) of the menu area.
///
/// `menubar_add_separator(area)` (on the screen manager): `Display::add_item` of the manager's current slot display
/// with kind 1, flags 0x1000010, the menu entry colours (0xae31aa, yellow, 1, 1, black), no text and the area; the item
/// index comes back.
/// Original: `menubar_add_separator` (0x761260, Menubar.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn menubar_add_separator(sm: usize, area: u32) -> u32 {
    let d = read_usize((sm as isize + 0x7fc + read_i16(sm + 0x382a) as isize * 0x300) as usize);
    tel26_add_item(d, [1, 0, 0, 0, 0, 0, 0, 0x1000010, tel35_colour(MENU_ENTRY_COLOUR), tel35_colour(G_GUI_COLOUR_YELLOW), 1, 1,
        tel35_colour(G_GUI_COLOUR_BLACK), 0, 0, 0, 0, area])
}

/// Rule: with more than one human, the menu offers "Change Player": one entry per human manager (his name, with his
/// club's or national team's name), the current one greyed, the ones done (when the menu is not the waiting one)
/// greyed out without link.
///
/// `menubar_add_change_player_menu(area, &row, waiting)`: the channel's manager keys (`get_channel_manager_keys`);
/// with one or none, or while a manager is being created, nothing; otherwise the row is advanced and "Change Player"
/// (kind 0x82, flags 0x1021, colour 0xaebde8) added at it in the area, with its popup area (0x30, 0xae31aa); the keys
/// sorted by full name (qsort with 0x540270). Each manager (a null key skipped): "<name> (<club>)" with the club or
/// national team's name (`club_get_national_team_name`, 100 bytes) or the plain name; the slot's own manager: id
/// 0x3fd, pale grey, data the manager; a manager done (`is_manager_done`) when `waiting` is 0: kind 2 flags 0x21,
/// no id; any other: id 0x3fd with the manager as data. Returns 1.
/// Original: `menubar_add_change_player_menu` (0x7612c0, Menubar.cpp, cdecl).
pub(crate) unsafe extern "C" fn menubar_add_change_player_menu(area: u32, row: *mut i32, waiting: u32) -> u32 {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let being_created: unsafe extern "C" fn() -> u32 = core::mem::transmute(0x838de0usize);
    let mut keys = [0usize; 0x10];
    let mut count = 0i16;
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT);
    exe::screenmanager_get_channel_manager_keys(G_SCREEN_MANAGER, keys.as_mut_ptr(), &mut count, slot as i32);
    if count <= 1 || being_created() != 0 {
        return 1;
    }
    covered!("tel35_5 change player menu");
    *row += 1;
    text(JOB_TEXT, 0xa5a800);
    let heading = tel26_add_item(current_human_display(), [0x82, 0, 0, 0, 0, 0, *row as u32, 0x1021, 0, 0, 0xc, 1, tel35_colour(CHAT_COLOUR_BDE8),
        JOB_TEXT as u32, 0, 0, 0, area]);
    let popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x30, tel35_colour(MENU_ENTRY_COLOUR), heading]);
    exe::crt_qsort(keys.as_mut_ptr() as usize, count as u32, 4, 0x540270usize);
    let mut name = [0u8; 200];
    let mut team_name = [0u8; 100];
    for k in 0..count as usize {
        let person = keys[k];
        if person == 0 {
            continue;
        }
        exe::database_format_staff_name(person, name.as_mut_ptr() as usize, 0xc8, 1);
        let club = read_usize(person + staff::CLUB_JOB);
        let team = if club != 0 { club } else { read_usize(person + staff::NATIONAL_JOB) };
        if team != 0 {
            exe::club_get_national_team_name(team, 0usize, team_name.as_mut_ptr() as usize, 0x64, 1, 1);
            text(JOB_TEXT, 0xa5a7d8, name.as_ptr(), team_name.as_ptr());
        } else {
            tel26_copy_text(JOB_TEXT as *mut u8, name.as_ptr());
        }
        if person == tel26_slot_manager().addr() {
            tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, tel35_colour(MENU_ENTRY_COLOUR), tel35_colour(G_GUI_COLOUR_BLACK), 1, 1,
                tel35_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 0, 0x3fd, person as u32, popup]);
        } else if exe::screenmanager_is_manager_done(G_SCREEN_MANAGER, person as u32) != 0 && waiting == 0 {
            tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, tel35_colour(MENU_ENTRY_COLOUR), tel35_colour(G_GUI_COLOUR_YELLOW), 0x21, 1,
                tel35_colour(G_GUI_COLOUR_BLACK), JOB_TEXT as u32, 0, 0, 0, popup]);
        } else {
            menu_list_entry(2, JOB_TEXT as u32, 0x3fd, person as u32, popup);
        }
    }
    1
}

/// Rule: the continents menu lists the continents by name, the chosen one ticked.
///
/// `menubar_list_continents(area, id, chosen)`: the continent indexes 0..count−1 sorted by name (qsort with
/// `compare_continents_by_name`); for each continent (`G_CONTINENT_TABLE`, 0xc6 bytes): with a chosen continent, " <name>"
/// ticked (kind 0x2002) when it is the chosen one, plain (2) otherwise; with none, "<name>" (2); id `id`, data the
/// continent, in the area.
/// Original: `menubar_list_continents` (0x765010, Menubar.cpp, cdecl).
pub(crate) unsafe extern "C" fn menubar_list_continents(area: u32, id: u32, chosen: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let count = read_i32(G_CONTINENT_COUNT);
    let mut order = [0u8; 0x100];
    let mut k = 0i8;
    while (k as i32) < count {
        order[k as u8 as usize] = k as u8;
        k = k.wrapping_add(1);
    }
    exe::crt_qsort(order.as_mut_ptr() as usize, count as u32, 1, 0x765b90usize);
    let (mut name, mut spaced) = ([0u8; 0x100], [0u8; 0x100]);
    let mut k = 0i8;
    while (k as i32) < read_i32(G_CONTINENT_COUNT) {
        let index = order[k as u8 as usize] as i8 as isize;
        let continent = (read_usize(G_CONTINENT_TABLE) as isize + index * 0xc6) as usize;
        if continent != 0 {
            format(name.as_mut_ptr() as usize, 0x991944, continent + 4);
            if chosen != 0 {
                format(spaced.as_mut_ptr() as usize, 0x98ed14, name.as_ptr());
                let kind = if continent == chosen { 0x2002 } else { 2 };
                menu_list_entry(kind, spaced.as_ptr() as u32, id, continent as u32, area);
            } else {
                menu_list_entry(2, name.as_ptr() as u32, id, continent as u32, area);
            }
        }
        k = k.wrapping_add(1);
    }
}

/// Rule: the nations menu lists the 35 league nations in the key order, split in parts with a header each when the
/// list is long, the chosen one ticked.
///
/// `menubar_list_league_nations(area, id, chosen)`: the key nations' order (`key_nation_sorted_order(0)`) and its
/// split (`NATION_MENU_SPLIT`, a local); for each of the 35 positions, the nation's league (`G_LEAGUE_TABLE`, 72 bytes
/// per nation) when it exists and is loaded (+0x11c): with more than one part, a nation in another part than the last
/// adds that part's header (`format_header`, kind 2 in the area) and its popup area, where the following nations go;
/// with one part they go to the area itself (with no part, to the order's address, as the original); then " <name>"
/// ticked (0x2002) or plain (2) with a chosen nation, "<name>" (2) without; id `id`, data the league.
/// Original: `menubar_list_league_nations` (0x7651e0, Menubar.cpp, cdecl).
pub(crate) unsafe extern "C" fn menubar_list_league_nations(area: u32, id: u32, chosen: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let order = tel35_cdecl1(0x66a0c0, 0);
    let mut split = [0u8; 0x10];
    let s = split.as_mut_ptr() as usize;
    let construct: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> usize = core::mem::transmute(0x66a280usize);
    let part_count: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x66a350usize);
    let in_second_part: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x66a430usize);
    let header: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> usize = core::mem::transmute(0x66a360usize);
    construct(s, 0, order, 0);
    let mut part = 0xffu8;
    let mut target = order as u32;
    let (mut name, mut spaced) = ([0u8; 0x100], [0u8; 0x100]);
    for k in 0..0x23usize {
        let nation = read_u8(order + k) as i8 as isize;
        let league = read_usize((G_LEAGUE_TABLE as isize + nation * 0x48) as usize);
        if league == 0 {
            continue;
        }
        format(name.as_mut_ptr() as usize, 0x991944, league + 4);
        if read_u8(league + 0x11c) == 0 {
            continue;
        }
        if part_count(s) > 1 {
            if in_second_part(s, league) != part {
                covered!("tel35_5 nations menu part header");
                part = in_second_part(s, league);
                header(s, JOB_TEXT, 0x7d0, part as u32);
                let item = menu_list_entry(2, JOB_TEXT as u32, 0, 0, area);
                target = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x30, tel35_colour(MENU_ENTRY_COLOUR), item]);
            }
        } else if part_count(s) == 1 {
            target = area;
        }
        if chosen != 0 {
            format(spaced.as_mut_ptr() as usize, 0x98ed14, name.as_ptr());
            let kind = if league == chosen { 0x2002 } else { 2 };
            menu_list_entry(kind, spaced.as_ptr() as u32, id, league as u32, target);
        } else {
            menu_list_entry(2, name.as_ptr() as u32, id, league as u32, target);
        }
    }
}

/// Rule: the manager chat screen shows the slot human's page of the chat (his own lines yellow, the others' pale grey,
/// empty lines in the entry colour) and the "Say:" entry.
///
/// `manager_chat_screen_build()`: the first time, every page is set up ("Welcome to manager chat" from nobody, 18
/// empty lines, one line, unchanged). The human's page is marked unchanged. The random background, the menu bar
/// (4, 0), the title (the game title) in the heading colours 0 and 1, the subtitle "Manager Chat". The page area
/// (0x6e, 0x91)-(0x30c, 0x1f9), 19 rows, kind 2, black; "Say: " (font 2); the text entry item (kind 0x10018, at
/// (0x91, 0x203)-(0x30c, 0x217), up to 0x60 − the lengths of the manager's two names, -1 area); the 19 lines (font 2),
/// the first one's widget written to parameter 0; the Back and Next buttons.
/// Original: `manager_chat_screen_build` (0x7654d0, Menubar.cpp, cdecl).
pub(crate) unsafe extern "C" fn manager_chat_screen_build() {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if read_u32(CHAT_SET_UP) == 0 {
        covered!("tel35_5 chat set up");
        for human in 0..0x10i32 {
            text(chat_line(human, 0), 0xa5aad8);
            write_u16(chat_line(human, 0) + CHAT_LINE_SENDER, 0xffff);
            write_u16(CHAT_LINE_COUNTS + 2 * human as usize, 1);
            write_u32(CHAT_CHANGED + 4 * human as usize, 0);
            for line in 1..CHAT_LINES as i32 {
                write_u8(chat_line(human, line), 0);
                write_u16(chat_line(human, line) + CHAT_LINE_SENDER, 0xffff);
            }
        }
        write_u32(CHAT_SET_UP, 1);
    }
    let me = chat_human();
    write_u32((CHAT_CHANGED as isize + me as isize * 4) as usize, 0);
    exe::background_set_random_picture();
    exe::menubar_build(4, 0);
    let title = tel35_cdecl1(0x839570, 0);
    let fore = exe::get_heading_colour(1) as u32;
    let back = exe::get_heading_colour(0) as u32;
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, title);
    text(JOB_TEXT, 0xa5aac8);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let black = tel35_colour(G_GUI_COLOUR_BLACK);
    write_u8(JOB_TEXT, 0);
    let area = tel26_add_area(current_human_display(), [0x6e, 0x91, 0x30c, 0x1f9, 1, 0, 0x13, 0, 2, black, u32::MAX]);
    text(JOB_TEXT, 0xa5aac0);
    tel35_text_item([0x6e, 0x203, 0x91, 0x217, 2, black, 1, 2, tel35_colour(CHAT_COLOUR_BDE8), JOB_TEXT as u32, 1]);
    write_u8(JOB_TEXT, 0);
    let manager = tel26_slot_manager().addr();
    let room = 0x60 - tel35_c_string_len(read_usize(manager + staff::FIRST_NAME) as *const u8) as i32
        - tel35_c_string_len(read_usize(manager + staff::SECOND_NAME) as *const u8) as i32;
    tel26_add_item(current_human_display(), [0x10018, 0x91, 0x203, 0x30c, 0x217, 0, 0, 2, black, black, 1, 2, tel35_colour(CHAT_COLOUR_BDE8),
        JOB_TEXT as u32, room as u32, 1, 0, u32::MAX]);
    for line in 0..CHAT_LINES as i32 {
        let at = chat_line(me as i32, line);
        let sender = read_i16(at + CHAT_LINE_SENDER);
        let colour = if sender == -1 {
            CHAT_COLOUR_BDE8
        } else if sender == me {
            G_GUI_COLOUR_YELLOW
        } else {
            G_GUI_COLOUR_PALE_GREY
        };
        let w = tel26_area_text([area, 0, line as u32, 1, black, 1, 2, tel35_colour(colour), at as u32]);
        if line == 0 {
            set_screen_param(0, w as i16 as isize as usize);
        }
    }
    tel26_back_next();
}

/// The text of a display item of the current human (+0x80); none (a null address) for -1.
unsafe fn chat_item_text(item: i16) -> usize {
    if item == -1 { 0 } else { display_item_at(current_human_display(), item) + display_item::TEXT }
}

/// Rule: on the manager chat screen a changed page is shown again, and "Send" (id 1) adds " <first> <second>: <text>"
/// to every human's page, scrolling a full page up by one line.
///
/// `manager_chat_screen_input(item)`: id 0x421 gives 0. With the human's page changed: each of the 19 line widgets
/// (from parameter 0) whose text differs from the page's line gets the line's text and colour (as the builder); redraw;
/// the page unchanged. Then the menu bar's command, then its object link (their answer when not 0). Id 1: for each of
/// the 16 pages, a full page (19 lines) moves its lines 1..18 up and writes the message on line 18, otherwise the
/// message goes on the next free line and the count goes up; the sender is the human; the page marked changed; -11.
/// Otherwise 0.
/// Original: `manager_chat_screen_input` (0x7657c0, Menubar.cpp, cdecl).
pub(crate) unsafe extern "C" fn manager_chat_screen_input(raw: u32) -> i32 {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let item = raw as i16;
    if tel26_item_id(item) == 0x421 {
        return 0;
    }
    let me = chat_human();
    let changed = (CHAT_CHANGED as isize + me as isize * 4) as usize;
    if read_u32(changed) != 0 {
        covered!("tel35_5 chat page refreshed");
        for line in 0..CHAT_LINES as i32 {
            let at = chat_line(me as i32, line);
            let w = (get_screen_param(0) as u32).wrapping_add(line as u32);
            let shown = chat_item_text(w as i16) as *const u8;
            let mut i = 0;
            let same = loop {
                let (p, q) = (*(at as *const u8).add(i), *shown.add(i));
                if p != q {
                    break false;
                }
                if p == 0 {
                    break true;
                }
                i += 1;
            };
            if !same {
                exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, (get_screen_param(0) as u32).wrapping_add(line as u32), at);
                let sender = read_i16(at + CHAT_LINE_SENDER);
                let colour = if sender == -1 {
                    CHAT_COLOUR_BDE8
                } else if sender == me {
                    G_GUI_COLOUR_YELLOW
                } else {
                    G_GUI_COLOUR_PALE_GREY
                };
                exe::screenmanager_set_widget_text_colour(G_SCREEN_MANAGER, (get_screen_param(0) as u32).wrapping_add(line as u32), tel35_colour(colour));
            }
        }
        exe::screenmanager_redraw_screen(G_SCREEN_MANAGER, 0, 0);
        write_u32(changed, 0);
    }
    let r = exe::menubar_handle_command(raw);
    if r != 0 {
        return r;
    }
    let r = exe::menubar_handle_object_link(raw);
    if r != 0 {
        return r;
    }
    if tel26_item_id(item) != 1 {
        return 0;
    }
    covered!("tel35_5 chat send");
    for human in 0..0x10i32 {
        let count = read_i16(CHAT_LINE_COUNTS + 2 * human as usize);
        let manager = tel26_slot_manager().addr();
        let (first, second) = (read_usize(manager + staff::FIRST_NAME), read_usize(manager + staff::SECOND_NAME));
        if count >= CHAT_LINES as i16 {
            for line in 0..CHAT_LINES as i32 - 1 {
                tel26_copy_text(chat_line(human, line) as *mut u8, chat_line(human, line + 1) as *const u8);
                write_u16(chat_line(human, line) + CHAT_LINE_SENDER, read_u16(chat_line(human, line + 1) + CHAT_LINE_SENDER));
            }
            let last = chat_line(human, CHAT_LINES as i32 - 1);
            format(last, 0xa5aaf4, first, second, chat_item_text(item));
            write_u16(last + CHAT_LINE_SENDER, me as u16);
        } else {
            let at = chat_line(human, count as i32);
            format(at, 0xa5aaf4, first, second, chat_item_text(item));
            write_u16(CHAT_LINE_COUNTS + 2 * human as usize, count.wrapping_add(1) as u16);
            write_u16(at + CHAT_LINE_SENDER, me as u16);
        }
        write_u32(CHAT_CHANGED + 4 * human as usize, 1);
    }
    -11
}

/// Rule: the continents are ordered by name.
///
/// `compare_continents_by_name(a, b)` (qsort of continent index bytes): a null a: 1; a null b: -1; otherwise the two
/// continents' names (+4 of their 0xc6-byte records) compared (`utils_string_compare`, 1).
/// Original: `compare_continents_by_name` (0x765b90, Menubar.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_continents_by_name(a: *const usize, b: *const usize) -> i32 {
    if a.is_null() {
        return 1;
    }
    if b.is_null() {
        return -1;
    }
    let table = read_usize(G_CONTINENT_TABLE) as isize;
    let name = |p: *const usize| (table + read_u8(p as usize) as i8 as isize * 0xc6 + 4) as *const u8;
    exe::utils_string_compare(name(a), name(b), 1, 0)
}

/// Comparer for `menubar_init_human_manager_records`.
pub(crate) unsafe extern "C" fn compare_menubar_init_human_manager_records() {
    // simplification: the start-up construction (and its atexit) runs once; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(%%I0%%)()
}

/// Comparer for `human_manager_record_human_manager_record`: side by side over the record (and the return).
pub(crate) unsafe extern "thiscall" fn compare_human_manager_record_human_manager_record(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I1%%);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&[(this, MENUBAR_RECORD_SIZE)], || o(this), || human_manager_record_human_manager_record(this));
    SCOREBOARD.record(equal && ro == rr, || format!("HUMAN_MANAGER_RECORD::HUMAN_MANAGER_RECORD({:#x}): differences in {:?}", this, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `menubar_destroy_human_manager_records`.
pub(crate) unsafe extern "C" fn compare_menubar_destroy_human_manager_records() {
    // simplification: the exit destruction runs once; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(%%I2%%)()
}

/// Comparer for `menubar_add_separator`: side by side over the display list.
pub(crate) unsafe extern "thiscall" fn compare_menubar_add_separator(sm: usize, area: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I3%%);
    let mut r = 0;
    compare_by_display_list_with("menubar_add_separator", &[], || r = o(sm, area), || { menubar_add_separator(sm, area); });
    r
}

/// Comparer for `menubar_add_change_player_menu`: side by side over the display list and the row.
pub(crate) unsafe extern "C" fn compare_menubar_add_change_player_menu(area: u32, row: *mut i32, waiting: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, *mut i32, u32) -> u32>(%%I4%%);
    let mut r = 0;
    compare_by_display_list_with("menubar_add_change_player_menu", &[(row as usize, 4)], || r = o(area, row, waiting),
        || { menubar_add_change_player_menu(area, row, waiting); });
    r
}

/// Comparer for `menubar_list_continents`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_menubar_list_continents(area: u32, id: u32, chosen: usize) {
    let o = original_fn::<unsafe extern "C" fn(u32, u32, usize)>(%%I5%%);
    compare_by_display_list_with("menubar_list_continents", &[], || o(area, id, chosen), || menubar_list_continents(area, id, chosen));
}

/// Comparer for `menubar_list_league_nations`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_menubar_list_league_nations(area: u32, id: u32, chosen: usize) {
    let o = original_fn::<unsafe extern "C" fn(u32, u32, usize)>(%%I6%%);
    compare_by_display_list_with("menubar_list_league_nations", &[], || o(area, id, chosen), || menubar_list_league_nations(area, id, chosen));
}

/// Comparer for `manager_chat_screen_build`: side by side over the display list, the chat set-up flag and the page's
/// changed flag (not the first time: the set-up writes every page).
pub(crate) unsafe extern "C" fn compare_manager_chat_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I7%%);
    if read_u32(CHAT_SET_UP) == 0 {
        // simplification: the chat set-up writes the whole log once; the first build only in the original
        return o();
    }
    compare_by_display_list_with("Manager chat screen (manager_chat_screen_build)", &[(CHAT_CHANGED, 0x40)], || o(), || manager_chat_screen_build());
}

/// Comparer for `manager_chat_screen_input`.
pub(crate) unsafe extern "C" fn compare_manager_chat_screen_input(item: u32) -> i32 {
    // simplification: sending, the redraw and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I8%%)(item)
}

compare_order!(compare_compare_continents_by_name, compare_continents_by_name, %%I9%%);
