use super::*;

/// The menu bar record of a human manager: the one of the current slot's manager when `manager` is null; null when
/// there is none or his id is not among the last 16 staff records.
///
/// `get_human_manager_record(manager)`: a null manager is replaced by the slot's manager ([G_SCREEN_RECORDS_MANAGER +
/// slot × 0x300]); null stays null. k = id − G_STAFF_COUNT + 16; 0 ≤ k < 16 gives record k, else null (`MENUBAR_RECORDS`, 0xf6 bytes each).
/// Original: `get_human_manager_record` (0x765480, Menubar.cpp, cdecl).
pub(crate) unsafe extern "C" fn get_human_manager_record(manager: usize) -> usize {
    let mut m = manager;
    if m == 0 {
        m = read_usize((G_SCREEN_RECORDS_MANAGER as isize + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize * 0x300) as usize);
        if m == 0 {
            return 0;
        }
    }
    covered!("tel39_3 human record");
    // tel39_3: the position among the last 16 staff records
    let k = read_i32(m).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(16);
    if !(0..16).contains(&k) {
        return 0;
    }
    MENUBAR_RECORDS + k as usize * MENUBAR_RECORD_SIZE
}

/// Comparer for `get_human_manager_record`: the record side by side (nothing is written).
pub(crate) unsafe extern "C" fn compare_get_human_manager_record(manager: usize) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_GET_HUMAN_MANAGER_RECORD)(manager);
    let n = get_human_manager_record(manager);
    SCOREBOARD.record(r == n, || format!("get_human_manager_record({:#x}) original={:#x} rust={:#x}", manager, r, n));
    r
}

/// Builds the game menu bar for the current human manager: the background, then the menu of `mode` (1 the game
/// menu, 2 the team menu, 3 the match menu) and the status item, and keeps the record of what was built.
///
/// It stores time() in `MENUBAR_BUILD_TIME` and the menu handler (0x823930, 0x423, 0x424). With the interactive flag
/// (`G_INTERACTIVE_MODE_FLAG`) clear, no manager in the slot or a manager outside the last 16 staff records, there is no
/// record and the game menu is built. Otherwise `menubar_record::HOLIDAY_PENDING` is zeroed and mode 4 (redraw) becomes
/// the record's mode, or 2 when the record has none or the manager can go neither back nor forward
/// (`exe::screenmanager_is_back_unavailable`(1) and `exe::screenmanager_is_forward_unavailable`).
///
/// The background area (0, 0, 0x59, 599: the height is read from the exe, 1079 in the 1080 copy) with the picture
/// "game.mbr" of CM3_DATA, and the bar's area (5, 10, 0x55, 0x24e).
///
/// Game menu (`menubar_build_game_menu`): the version, the arrows, "Add Manager", "Chat", "Restart Game" and "Exit
/// Game". Team menu (`menubar_build_team_menu`): the date (or the match, or the match day percentage), the arrows,
/// "Continue Game", the manager's menu, Competitions, Nations & Clubs, Find, the change player menu (0x7612c0) and
/// Game Options. Match menu (`menubar_build_match_menu`, the record's paused match): the minute, the arrows, "Paused",
/// both teams' tactics and the commentary speed. Then, with a record, the status item (`menubar_build_status`).
///
/// Finally, with a manager in the slot on holiday (`exe::human_manager_is_on_holiday`), the holiday stamp is set to
/// `exe::utils_get_time_ms`; and a record with no refresh time gets `exe::utils_get_time_ms` + 1000.
/// Original: menubar_build (0x75db00, Menubar.cpp, cdecl)
pub(crate) unsafe extern "C" fn menubar_build(mode: u32, context: i32) {
    let mut mode = mode as u8;
    let staff = read_usize(menubar_slot(MENUBAR_SLOT_MANAGER));
    exe::at_946ebb(MENUBAR_BUILD_TIME as *mut i32);
    write_u32(MENUBAR_HANDLER, 0x823930);
    write_u32(MENUBAR_HANDLER + 4, 0x423);
    write_u32(MENUBAR_HANDLER + 8, 0x424);
    let mut rec = 0usize;
    if read_u32(G_INTERACTIVE_MODE_FLAG) == 0 {
        covered!("tel26_6 menu bar outside the interactive mode");
        mode = 1;
    } else {
        let s = read_usize(menubar_slot(MENUBAR_SLOT_MANAGER));
        let k = if s == 0 { -1 } else { read_i32(s).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10) };
        if (0..0x10).contains(&k) {
            rec = MENUBAR_RECORDS + k as usize * MENUBAR_RECORD_SIZE;
        }
        if rec == 0 {
            covered!("tel26_6 menu bar without a record");
            mode = 1;
        } else {
            write_u32(rec + menubar_record::HOLIDAY_PENDING, 0);
            if mode == 4 {
                covered!("tel26_6 menu bar redraw");
                mode = if read_u8(rec + menubar_record::MODE) == 0
                    || (exe::screenmanager_is_back_unavailable(SCREEN_MANAGER, 1) != 0 && exe::screenmanager_is_forward_unavailable(SCREEN_MANAGER) != 0)
                {
                    2
                } else {
                    read_u8(rec + menubar_record::MODE)
                };
            }
        }
    }
    // 1080: the background's height is the one in the exe (0x257 in the original, 0x437 in the 1080 copy)
    let height = read_i32(0x75dc11);
    let background = menubar_area([0, 0, 0x59, height as usize, 1, 0, 1, 0, 1, 0, usize::MAX]);
    let picture = exe::utils_build_file_path(0x9b5348usize, 0xa5a7ccusize);
    menubar_item([0x400, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0xc, 1, 0, picture, 0, 0, 0, background]);
    let area = menubar_area([5, 10, 0x55, 0x24e, 1, 0, 0xd, 0, 1, 0, usize::MAX]);
    match mode {
        1 => menubar_build_game_menu(area, rec, context),
        2 => menubar_build_team_menu(area, rec, staff, context),
        3 => {
            let m = read_usize(rec + menubar_record::PAUSED_MATCH);
            if m != 0 {
                menubar_build_match_menu(area, rec, m, context);
            }
        }
        _ => covered!("tel26_6 menu bar with no menu"),
    }
    if rec != 0 {
        menubar_build_status(rec, mode);
    }
    if read_usize(menubar_slot(MENUBAR_SLOT_MANAGER)) != 0 && exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER), 0usize) != 0 {
        covered!("tel26_6 menu bar holiday stamp");
        let now = exe::utils_get_time_ms();
        exe::human_manager_set_holiday_stamp(read_usize(G_HUMAN_MANAGER), now, 0usize);
    }
    if rec != 0 && read_u32(rec + menubar_record::REFRESH_TIME) == 0 {
        write_u32(rec + menubar_record::REFRESH_TIME, exe::utils_get_time_ms().wrapping_add(1000));
    }
}

/// Comparer for `menubar_build`.
///
/// The bar is compared side by side over the display list (the screen manager, the current Display, the parameter
/// block, the text buffer, the generator) plus the human's menu bar record, without its refresh time
/// (`menubar_record::REFRESH_TIME`, `utils_get_time_ms` + 1000 in each run). The build time (time()) is not compared.
pub(crate) unsafe extern "C" fn compare_menubar_build(mode: u32, context: i32) {
    let o = original_fn::<unsafe extern "C" fn(u32, i32)>(IDX_MENUBAR_BUILD);
    if read_u32(MENUBAR_NETWORK_GAME) != 0 {
        // simplification: in a network game the countdown sends a packet (ScreenManager::send_clear_click) and the
        // times are compared across a second boundary; only the original runs
        return o(mode, context);
    }
    let mut extra = vec![(MENUBAR_TEXT, 0x800)];
    let s = read_usize(menubar_slot(MENUBAR_SLOT_MANAGER));
    if s != 0 {
        let k = read_i32(s).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10);
        if (0..0x10).contains(&k) {
            let rec = MENUBAR_RECORDS + k as usize * MENUBAR_RECORD_SIZE;
            extra.push((rec, menubar_record::REFRESH_TIME));
            extra.push((rec + menubar_record::HEADER_WIDGET, MENUBAR_RECORD_SIZE - menubar_record::HEADER_WIDGET));
        }
    }
    // simplification: a competition row with neither nation nor continent shows the error box in both runs
    compare_by_display_list_with("menubar_build", &extra, || o(mode, context), || menubar_build(mode, context));
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
    let mut keys = [0usize; 0x10];
    let mut count = 0i16;
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT);
    exe::screenmanager_get_channel_manager_keys(G_SCREEN_MANAGER, keys.as_mut_ptr(), &mut count, slot as i32);
    if count <= 1 || exe::is_current_manager_being_created() != 0 {
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
        let person = Staff(keys[k]);
        if person.is_null() {
            continue;
        }
        exe::database_format_staff_name(person, name.as_mut_ptr() as usize, 0xc8, 1);
        let club = person.club_job().addr();
        let team = if club != 0 { club } else { person.national_job().addr() };
        if team != 0 {
            exe::club_get_national_team_name(team, 0usize, team_name.as_mut_ptr() as usize, 0x64, 1, 1);
            text(JOB_TEXT, 0xa5a7d8, name.as_ptr(), team_name.as_ptr());
        } else {
            tel26_copy_text(JOB_TEXT as *mut u8, name.as_ptr());
        }
        if person == tel26_slot_manager() {
            tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, tel35_colour(MENU_ENTRY_COLOUR), tel35_colour(G_GUI_COLOUR_BLACK), 1, 1,
                tel35_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 0, 0x3fd, person.addr() as u32, popup]);
        } else if exe::screenmanager_is_manager_done(G_SCREEN_MANAGER, person.addr() as u32) != 0 && waiting == 0 {
            tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, tel35_colour(MENU_ENTRY_COLOUR), tel35_colour(G_GUI_COLOUR_YELLOW), 0x21, 1,
                tel35_colour(G_GUI_COLOUR_BLACK), JOB_TEXT as u32, 0, 0, 0, popup]);
        } else {
            menu_list_entry(2, JOB_TEXT as u32, 0x3fd, person.addr() as u32, popup);
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
    let header: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> usize = core::mem::transmute(FN_NATION_MENU_SPLIT_FORMAT_HEADER);
    exe::new_nation_menu_split(s, 0, order, 0);
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
        if exe::nation_menu_split_part_count(s) > 1 {
            if exe::nation_menu_split_in_second_part(s, league) != part {
                covered!("tel35_5 nations menu part header");
                part = exe::nation_menu_split_in_second_part(s, league);
                header(s, JOB_TEXT, 0x7d0, part as u32);
                let item = menu_list_entry(2, JOB_TEXT as u32, 0, 0, area);
                target = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x30, tel35_colour(MENU_ENTRY_COLOUR), item]);
            }
        } else if exe::nation_menu_split_part_count(s) == 1 {
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
    let manager = tel26_slot_manager();
    let room = 0x60 - tel35_c_string_len(manager.first_name() as *const u8) as i32
        - tel35_c_string_len(manager.second_name() as *const u8) as i32;
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
        let manager = tel26_slot_manager();
        let (first, second) = (manager.first_name(), manager.second_name());
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
    original_fn::<unsafe extern "C" fn()>(IDX_MENUBAR_INIT_HUMAN_MANAGER_RECORDS)()
}

/// Comparer for `human_manager_record_human_manager_record`: side by side over the record (and the return).
pub(crate) unsafe extern "thiscall" fn compare_human_manager_record_human_manager_record(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_HUMAN_MANAGER_RECORD_HUMAN_MANAGER_RECORD);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&[(this, MENUBAR_RECORD_SIZE)], || o(this), || human_manager_record_human_manager_record(this));
    SCOREBOARD.record(equal && ro == rr, || format!("HUMAN_MANAGER_RECORD::HUMAN_MANAGER_RECORD({:#x}): differences in {:?}", this, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `menubar_destroy_human_manager_records`.
pub(crate) unsafe extern "C" fn compare_menubar_destroy_human_manager_records() {
    // simplification: the exit destruction runs once; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(IDX_MENUBAR_DESTROY_HUMAN_MANAGER_RECORDS)()
}

/// Comparer for `menubar_add_separator`: side by side over the display list.
pub(crate) unsafe extern "thiscall" fn compare_menubar_add_separator(sm: usize, area: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_MENUBAR_ADD_SEPARATOR);
    let mut r = 0;
    compare_by_display_list_with("menubar_add_separator", &[], || r = o(sm, area), || { menubar_add_separator(sm, area); });
    r
}

/// Comparer for `menubar_add_change_player_menu`: side by side over the display list and the row.
pub(crate) unsafe extern "C" fn compare_menubar_add_change_player_menu(area: u32, row: *mut i32, waiting: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, *mut i32, u32) -> u32>(IDX_MENUBAR_ADD_CHANGE_PLAYER_MENU);
    let mut r = 0;
    compare_by_display_list_with("menubar_add_change_player_menu", &[(row as usize, 4)], || r = o(area, row, waiting),
        || { menubar_add_change_player_menu(area, row, waiting); });
    r
}

/// Comparer for `menubar_list_continents`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_menubar_list_continents(area: u32, id: u32, chosen: usize) {
    let o = original_fn::<unsafe extern "C" fn(u32, u32, usize)>(IDX_MENUBAR_LIST_CONTINENTS);
    compare_by_display_list_with("menubar_list_continents", &[], || o(area, id, chosen), || menubar_list_continents(area, id, chosen));
}

/// Comparer for `menubar_list_league_nations`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_menubar_list_league_nations(area: u32, id: u32, chosen: usize) {
    let o = original_fn::<unsafe extern "C" fn(u32, u32, usize)>(IDX_MENUBAR_LIST_LEAGUE_NATIONS);
    compare_by_display_list_with("menubar_list_league_nations", &[], || o(area, id, chosen), || menubar_list_league_nations(area, id, chosen));
}

/// Comparer for `manager_chat_screen_build`: side by side over the display list, the chat set-up flag and the page's
/// changed flag (not the first time: the set-up writes every page).
pub(crate) unsafe extern "C" fn compare_manager_chat_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_MANAGER_CHAT_SCREEN_BUILD);
    if read_u32(CHAT_SET_UP) == 0 {
        // simplification: the chat set-up writes the whole log once; the first build only in the original
        return o();
    }
    compare_by_display_list_with("Manager chat screen (manager_chat_screen_build)", &[(CHAT_CHANGED, 0x40)], || o(), || manager_chat_screen_build());
}

/// Comparer for `manager_chat_screen_input`.
pub(crate) unsafe extern "C" fn compare_manager_chat_screen_input(item: u32) -> i32 {
    // simplification: sending, the redraw and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_MANAGER_CHAT_SCREEN_INPUT)(item)
}

/// Rule: a linked word on any screen (a competition, a stage, a club, a match, an award, a person, a fixture to watch, a
/// search) opens its screen; "Print" prints the screen first.
///
/// `menubar_handle_object_link(item)`, by the widget's id (`tel26_item_id`) and data (`tel26_item_value`): 0x426 prints
/// the screen (`ScreenManager::print_screen`) and goes on. 0x7d0 (a competition record): when the competition object
/// exists (`G_COMP_OBJECT_TABLE` of its id) and it is neither a reserve or minor competition nor a friendly's own
/// record, its screen opens (stage and round -2). 0x7e5 (a stage link: competition << 22 | stage << 11 | round): a club
/// competition with an object, not reserve or minor and not a friendly's own: the stage (`Comp::get_stage` from -1 to
/// +0x30) whose id (+0x3a) is the link's; none: the competition's screen (-2, -2); a cup stage (+0x43 = 1) with rounds
/// (+0xab) whose round id (+7 of each 0x68-byte round at +0xa3) is the link round's base (`comp_util_get_base_round_id`):
/// that stage and round; otherwise the stage (-2). 0x7e6: the club history (club, 0, 1). 0x7d1: the tournament
/// invitations (club, 0). 0x7d3: a match (-1 gives 0) found in the match stats (`Match_Stats::find_record_fixture`,
/// none gives 0) opens its details. 0x7d5: the club's squad, except a national team with a division whose nation
/// (+0x71) is not set, or a club with no division. 0x7d6: the reserves' squad. 0x7d4: the award screen. 0x7d7, 0x7e7,
/// 0x7d8: the person's profile (1, 1, 0), (1, 1, extra 2), (2, 1, 0). Through the display's readers, the profiles of
/// `LINK_PROFILES`. Through the screen manager's: 0x7d2 switches the watched fixture (`HUMAN_MANAGER::is_watching_fixture`
/// / `set_watched_fixture` with the fixture or 0) and gives -11 when it took; 0x7e4 opens the player search on the
/// data. Each screen opened gives -4; nothing else, 0.
/// Original: `menubar_handle_object_link` (0x7643d0, Menubar.cpp, cdecl).
pub(crate) unsafe extern "C" fn menubar_handle_object_link(raw: u32) -> i32 {
    let item = raw as i16;
    if tel26_item_id(item) == 0x426 {
        exe::screenmanager_print_screen(G_SCREEN_MANAGER);
    }
    if tel26_item_id(item) == 0x7d0 {
        let record = tel26_item_value(item) as usize;
        if read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(record) as usize * 4) != 0
            && exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), record) == 0
            && exe::friendly_is_own_comp_record(read_usize(G_FRIENDLY), record) == 0 {
            covered!("tel35_6 link competition");
            link_open_comp(record, -2, -2);
            return -4;
        }
    }
    if tel26_item_id(item) == 0x7e5 {
        let link = tel26_item_value(item);
        let comp = (link >> 0x16) as i32;
        let stage_id = ((link as i32 >> 0xb) & 0x7ff) as u16;
        let round = link & 0x7ff;
        if comp >= 0 && comp < read_i32(G_CLUB_COMP_COUNT) {
            let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + comp as usize * 4);
            if object != 0 && exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), read_usize(object + 4)) == 0 {
                let offset = (comp as u32).wrapping_mul(LINK_CLUB_COMP_SIZE) as usize;
                if exe::friendly_is_own_comp_record(read_usize(G_FRIENDLY), read_usize(G_CLUB_COMP_TABLE) + offset) == 0 {
                    covered!("tel35_6 link stage");
                    let mut s = -1i32;
                    while s <= read_i32(object + 0x30) {
                        let stage = exe::comp_get_stage(object, s);
                        if read_u16(stage + 0x3a) == stage_id {
                            if read_u8(stage + 0x43) == 1 {
                                let mut j = 0i32;
                                while j < read_i8(stage + 0xab) as i32 {
                                    let base = exe::comp_util_get_base_round_id(round) as u16;
                                    if read_u16((read_usize(stage + 0xa3) as isize + 7 + j as isize * LINK_ROUND_SIZE) as usize) == base {
                                        link_open_comp(read_usize(G_CLUB_COMP_TABLE) + offset, s, j);
                                        return -4;
                                    }
                                    j += 1;
                                }
                            }
                            link_open_comp(read_usize(G_CLUB_COMP_TABLE) + offset, s, -2);
                            return -4;
                        }
                        s += 1;
                    }
                    link_open_comp(read_usize(G_CLUB_COMP_TABLE) + offset, -2, -2);
                    return -4;
                }
            }
        }
    }
    match tel26_item_id(item) {
        0x7e6 => {
            let f: unsafe extern "C" fn(u32, u32, u32) = core::mem::transmute(FN_OPEN_CLUB_HISTORY_SCREEN);
            f(tel26_item_value(item), 0, 1);
            return -4;
        }
        0x7d1 => {
            let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(FN_OPEN_TOURNAMENT_INVITATIONS_SCREEN);
            f(tel26_item_value(item), 0);
            return -4;
        }
        0x7d3 => {
            let fixture = tel26_item_value(item);
            if item != -1 && fixture == u32::MAX {
                return 0;
            }
            let mut record = [0u8; 0x50];
            if exe::match_stats_find_record_fixture(read_usize(G_MATCH_STATS), fixture as usize, record.as_mut_ptr() as usize) == 0 {
                return 0;
            }
            let f: unsafe extern "C" fn(u32) = core::mem::transmute(FN_OPEN_MATCH_DETAILS_SCREEN);
            f(fixture);
            return -4;
        }
        _ => {}
    }
    if tel26_item_id(item) == 0x7d5 {
        let club = tel26_item_value(item) as usize;
        let skip = (exe::database_is_national_team(club) != 0 && read_usize(club + 0x53) != 0 && read_usize(read_usize(club + 0x53) + 0x71) == 0)
            || (exe::database_is_national_team(club) == 0 && read_usize(club + 0x53) == 0);
        if !skip {
            link_open_squad(club as u32, 0);
            return -4;
        }
    }
    match tel26_item_id(item) {
        0x7d6 => {
            link_open_squad(tel26_item_value(item), 1);
            return -4;
        }
        0x7d4 => {
            tel35_cdecl1(0x414c20, tel26_item_value(item) as usize);
            return -4;
        }
        0x7d7 => {
            link_open_profile(tel26_item_value(item), 1, 1, 0);
            return -4;
        }
        0x7e7 => {
            link_open_profile(tel26_item_value(item), 1, 1, 2);
            return -4;
        }
        0x7d8 => {
            link_open_profile(tel26_item_value(item), 2, 1, 0);
            return -4;
        }
        _ => {}
    }
    for (id, tab, kind) in LINK_PROFILES {
        let mut w = link_widget(0x468b10, current_human_display(), raw) as u16;
        if w == 0 {
            w = read_u16(G_CURRENT_COMMAND_ID);
        }
        if w == id {
            covered!("tel35_6 link profile tab");
            link_open_profile(link_widget(0x4739a0, current_human_display(), raw), tab, kind, 0);
            return -4;
        }
    }
    if link_widget(0x468b40, G_SCREEN_MANAGER, raw) == 0x7d2 {
        let fixture = link_widget(0x468b90, G_SCREEN_MANAGER, raw) as usize;
        let watching = exe::human_manager_is_watching_fixture(read_usize(G_HUMAN_MANAGER), fixture, 0usize);
        if exe::human_manager_set_watched_fixture(read_usize(G_HUMAN_MANAGER), if watching != 0 { 0 } else { fixture }, 0) != 0 {
            covered!("tel35_6 link watch fixture");
            return -11;
        }
    }
    if link_widget(0x468b40, G_SCREEN_MANAGER, raw) == 0x7e4 {
        tel35_cdecl1(0x808540, link_widget(0x468b90, G_SCREEN_MANAGER, raw) as usize);
        return -4;
    }
    0
}

/// Comparer for `menubar_handle_object_link`.
pub(crate) unsafe extern "C" fn compare_menubar_handle_object_link(item: u32) -> i32 {
    // simplification: opening the linked screen (and printing) does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_MENUBAR_HANDLE_OBJECT_LINK)(item)
}

/// Rule: the menu bar's answer to a widget (item −1: none), on every frame of every screen.
///
/// 0x424 gives -11 at once. Then the periodic work (`menubar_command_periodic`: the auto save, the Continue button, the
/// flashing header, the status item), the idle continuation (`menubar_command_idle`), the match Continue
/// (`match_screens_handle_continue_command`, its nonzero answer returned), the boxes' answers when the screen manager
/// has one to dispatch (`menubar_command_answer`) and the command (`menubar_command_dispatch`). Each part can end the
/// routine; the answers -4 (a screen or a box opened), -5 (go on), -11, -12, -2 and -3 go to the screen manager; 0, not
/// handled (the object links come next).
/// Original: `menubar_handle_command` (0x7615d0, Menubar.cpp, cdecl).
pub(crate) unsafe extern "C" fn menubar_handle_command(raw: u32) -> i32 {
    let item = raw as i16;
    if tel26_item_id(item) == 0x424 {
        return -11;
    }
    let rec = menubar_command_record();
    let mut t = MenubarCommandTick { dry: false, rec, text: MENUBAR_TEXT, unsure: false, ms_marks: [0; 2], marks: 0, seconds: false };
    if let Some(r) = menubar_command_periodic(&mut t) {
        return r;
    }
    if let Some(r) = menubar_command_idle(&mut t, item) {
        return r;
    }
    let r = core::mem::transmute::<usize, unsafe extern "C" fn(u32) -> i32>(0x725ca0)(raw);
    if r != 0 {
        return r;
    }
    if read_u32(TEL35_NETWORK_FLAG) != 0 {
        if let Some(r) = menubar_command_answer(rec) {
            return r;
        }
    }
    menubar_command_dispatch(&mut t, item, rec)
}

/// Comparer for `menubar_handle_command`.
///
/// The routine runs on every frame of every screen (the automatic career included) and its commands open screens,
/// save and leave the game: only the original runs. Before it the copy's periodic work and idle continuation run dry
/// (`MenubarCommandTick`: on a copy of the record and a text buffer of their own, calling nothing that changes the game
/// or the screen); after it, with no command (id 0), the original's record (without the refresh time, read from the
/// clock) and its answer must be the predicted ones; with a command, the answer must be the command's fixed one
/// (`menubar_command_expected`) or the periodic work's. Not compared: a network game (the widget changes are sent), a box
/// answer to dispatch, the match Continue (0x3e8, `match_screens_handle_continue_command`), an outcome that needs
/// running (a save, a box, a screen), and a run in which the clock crossed a threshold the decisions read.
pub(crate) unsafe extern "C" fn compare_menubar_handle_command(raw: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_MENUBAR_HANDLE_COMMAND);
    if read_u32(MENUBAR_NETWORK_GAME) != 0 || read_u32(TEL35_NETWORK_FLAG) != 0 {
        // simplification: the widget changes are sent to the other players and the answers save, restart or leave the
        // game; only the original runs
        return o(raw);
    }
    let item = raw as i16;
    let id = tel26_item_id(item);
    let real = menubar_command_record();
    let mut copy = [0u8; MENUBAR_RECORD_SIZE];
    if real != 0 {
        core::ptr::copy_nonoverlapping(real as *const u8, copy.as_mut_ptr(), MENUBAR_RECORD_SIZE);
    }
    let before = copy;
    let mut text = [0u8; 0x200];
    let mut t = MenubarCommandTick {
        dry: true,
        rec: if real != 0 { copy.as_mut_ptr() as usize } else { 0 },
        text: text.as_mut_ptr() as usize,
        unsure: false,
        ms_marks: [0; 2],
        marks: 0,
        seconds: false,
    };
    let clock = || crate::simulation::clock_peek(0x7615d0).unwrap_or_else(|| exe::utils_get_time_ms() as i32);
    let (ms0, s0) = (clock(), exe::at_946ebb(core::ptr::null_mut()));
    let mut early = if id == 0x424 { Some(-11) } else { None };
    if early.is_none() {
        early = menubar_command_periodic(&mut t);
    }
    if early.is_none() {
        early = menubar_command_idle(&mut t, item);
    }
    let expected = match early {
        Some(v) => Some(v),
        None if id == 0x3e8 => None,
        None => menubar_command_expected(id),
    };
    let r = o(raw);
    let (ms1, s1) = (clock(), exe::at_946ebb(core::ptr::null_mut()));
    let crossed = t.ms_marks[..t.marks].iter().any(|&at| ms0.wrapping_sub(at) < 0 && ms1.wrapping_sub(at) >= 0);
    if t.unsure || crossed || (t.seconds && s0 != s1) {
        return r;
    }
    if let Some(e) = expected {
        SCOREBOARD.record(r == e, || format!("menubar_handle_command (menu bar, command {:#x}): answer original={} rust={}", id, r, e));
    }
    // the header flips on a clock threshold; in the simulation each calling routine has a clock of its own
    // (simulation.rs, `fixed_clock`), so the copy and the original read different clocks: a flip on one side only is
    // the clock, not the routine
    let refresh = |b: &[u8]| u32::from_le_bytes([b[6], b[7], b[8], b[9]]);
    let flipped_copy = refresh(&copy) != refresh(&before);
    let flipped_original = real != 0 && read_u32(real + menubar_record::REFRESH_TIME) != refresh(&before);
    if flipped_copy != flipped_original {
        return r;
    }
    if id == 0 && real != 0 {
        let skip = menubar_record::REFRESH_TIME..menubar_record::REFRESH_TIME + 4;
        let differ: Vec<usize> = (0..MENUBAR_RECORD_SIZE).filter(|k| !skip.contains(k) && read_u8(real + k) != copy[*k]).collect();
        SCOREBOARD.record(differ.is_empty(), || {
            format!("menubar_handle_command (menu bar record): bytes {:x?} differ, original {:x?} rust {:x?}", differ,
                differ.iter().map(|&k| read_u8(real + k)).collect::<Vec<_>>(), differ.iter().map(|&k| copy[k]).collect::<Vec<_>>())
        });
    }
    r
}

/// The address of a per-slot field of the screen manager for the current slot.
pub(crate) unsafe fn menubar_slot(offset: usize) -> usize {
    (SCREEN_MANAGER as isize + offset as isize + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize * 0x300) as usize
}

/// `Display::add_item` on the current human's Display.
pub(crate) unsafe fn menubar_item(a: [usize; 18]) -> usize {
    let f: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    f(current_human_display(), a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13], a[14], a[15], a[16], a[17])
}

/// `Display::add_area` on the current human's Display.
pub(crate) unsafe fn menubar_area(a: [usize; 11]) -> usize {
    let f: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    f(current_human_display(), a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10])
}

/// The game menu (mode 1): the version ("Version <%s>"), the arrows, "Add Manager" (a button 0x3fb with game data
/// loaded, `G_GAME_DATA_LOADED_FLAG`, and `menubar_can_add_manager`, else inactive text), "Chat" (0x421, with
/// `menubar_can_chat`), "Restart Game" (0x42f, outside a network slot, `screen_manager::NETWORK_SLOT`, and with no flagged
/// screen) and "Exit Game" (0x402, with no flagged screen); then, with a record, mode 1 and the context in it.
pub(crate) unsafe fn menubar_build_game_menu(area: usize, rec: usize, context: i32) {
    covered!("tel26_6 game menu");
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut row = 0i32;
    text(MENUBAR_TEXT, 0xa5a7b4, exe::utils_get_version_string());
    menubar_area_text(area, row, 0x1061, 0xc, 1, colour_word(G_GUI_COLOUR_YELLOW));
    row += 1;
    menubar_arrows(row);
    row += 1;
    menubar_text(0xa5a7a8);
    if read_u32(G_GAME_DATA_LOADED_FLAG) != 0 && menubar_can_add_manager() {
        menubar_area_button(area, 0, row, 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), MENUBAR_TEXT, 0x3fb);
    } else {
        menubar_area_text(area, row, 0x1021, 0x2c, 1, colour_word(G_GUI_COLOUR_PALE_GREY));
    }
    if menubar_can_chat() {
        row += 1;
        menubar_text(0xa5a7a0);
        menubar_area_button(area, 0, row, 0xc, 1, colour_word(MENUBAR_HEADING_COLOUR), MENUBAR_TEXT, 0x421);
    }
    row += 1;
    menubar_text(0xa5a790);
    if read_u16(menubar_slot(screen_manager::NETWORK_SLOT)) == 0 && menubar_no_flagged_screens() {
        menubar_area_button(area, 0, row, 0xc, 1, colour_word(G_GUI_COLOUR_YELLOW), MENUBAR_TEXT, 0x42f);
    } else {
        menubar_area_text(area, row, 0x1021, 0x2c, 1, colour_word(G_GUI_COLOUR_YELLOW));
    }
    row += 1;
    menubar_text(0xa5a784);
    if menubar_no_flagged_screens() {
        menubar_area_button(area, 0, row, 0xc, 1, colour_word(G_GUI_COLOUR_YELLOW), MENUBAR_TEXT, 0x402);
    } else {
        menubar_area_text(area, row, 0x1021, 0x2c, 1, colour_word(G_GUI_COLOUR_YELLOW));
    }
    if rec != 0 {
        write_u8(rec + menubar_record::MODE, 1);
        write_i32(rec + menubar_record::CONTEXT, context);
    }
}

/// The team menu (mode 2).
///
/// The header (kept in `menubar_record::HEADER_WIDGET` and copied to `menubar_record::HEADER_TEXT`): while the record is
/// updating and the match day (`G_MATCH_DAY`) is below 100 % (done +0x25 × 100 / total +0x21, a signed byte) with
/// `G_MATCH_DAY_PLAYING_FLAG` set, "Updating <%d>%"; otherwise, updating or with the config flag 0x10000 and a paused
/// match, "home v away"; otherwise the date (`Date::get_weekday_numeric_date_string` of `G_CURRENT_DATE` with
/// `G_CURRENT_DAY_SLICE`), in the font read from the exe (1, or 2 in the 1080 copy). Then the arrows.
///
/// The continue item (`menubar_record::CONTINUE_WIDGET` and `CONTINUE_STATE`): with a match day whose +0x34 stage is
/// neither 2 nor 8 and no paused match, "Latest Scores" (0x418, state 3); with a manager being created, a current
/// screen flagged without an exit function or a context, "Continue Game" inactive (state 2); a manager done
/// (`exe::screenmanager_is_manager_done`): "Waiting For Players" (0x3e8, state 1); otherwise "Continue Game" (0x3e8,
/// state 1), where a state-3 record restarts the countdown time and, in a network game with a countdown, no paused match
/// and the manager not on holiday, the seconds since it (`menubar_record::COUNTDOWN`) below 5 show the seconds left in
/// red, font 4; `menubar_record::HOLIDAY_PENDING` = 1 and `ScreenManager::send_clear_click`. Every state but the last
/// one clears the countdown time (−1).
///
/// Then the manager's menu (`menubar_manager_menu`), Competitions (`menubar_competitions_menu`), Nations & Clubs and
/// Find (`menubar_nations_and_find_menus`), the change player menu (0x7612c0, the others disabled with no paused
/// match), Game Options (`menubar_game_options_menu`) and mode 2 with the context in the record.
pub(crate) unsafe fn menubar_build_team_menu(area: usize, rec: usize, staff: usize, context: i32) {
    covered!("tel26_6 team menu");
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: MenubarSprintf = core::mem::transmute(FN_CRT_SPRINTF);
    let m = read_usize(rec + menubar_record::PAUSED_MATCH);
    let mut row = 0i32;
    let updating = read_u8(rec + menubar_record::UPDATING);
    let md = read_usize(G_MATCH_DAY);
    let percentage = |md: usize| {
        let total = read_i32(md + 0x21);
        if total == 0 { 0 } else { read_i32(md + 0x25).wrapping_mul(100).wrapping_div(total) as i8 }
    };
    let font;
    if updating == 1 && md != 0 && (read_i32(md + 0x21) == 0 || percentage(md) < 100) && read_u32(G_MATCH_DAY_PLAYING_FLAG) != 0 {
        covered!("tel26_6 team menu updating");
        text(MENUBAR_TEXT, 0xa5a764, percentage(md) as i32);
        font = 1;
    } else {
        if (updating == 1 || read_u32(read_usize(G_CONFIG) + 4) & 0x10000 != 0) && m != 0 {
            covered!("tel26_6 team menu match header");
            let away = Club(read_usize(m + 0x24));
            let home = Club(read_usize(m + 0x20));
            let a = (away.short_gender_name() as i8) as i32;
            let h = (home.short_gender_name() as i8) as i32;
            text(MENUBAR_TEXT, 0xa5a738, h, h, a, a, home.addr() + club::SHORT_NAME, away.addr() + club::SHORT_NAME);
        } else {
            let date: MenubarThis3 = core::mem::transmute(FN_DATE_GET_WEEKDAY_NUMERIC_DATE_STRING);
            date(G_CURRENT_DATE, MENUBAR_TEXT, 0x7d0, read_u8(G_CURRENT_DAY_SLICE) as usize);
        }
        // 1080: the header font is the one in the exe (1 in the original, 2 in the 1080 copy)
        font = read_u8(0x75e16c) as i8 as isize as usize;
    }
    let w = menubar_area_text(area, row, 0x1061, 0xc, font, colour_word(G_GUI_COLOUR_YELLOW));
    write_u16(rec + menubar_record::HEADER_WIDGET, w);
    let mut k = 0;
    loop {
        let c = read_u8(MENUBAR_TEXT + k);
        write_u8(rec + menubar_record::HEADER_TEXT + k, c);
        if c == 0 {
            break;
        }
        k += 1;
    }
    row += 1;
    menubar_arrows(row);
    row += 1;
    let md = read_usize(G_MATCH_DAY);
    let latest = md != 0 && {
        let stage = read_usize(md + 0x34);
        stage != 0 && read_u8(stage + 0x67) != 2 && read_u8(stage + 0x67) != 8
    } && m == 0;
    if latest {
        covered!("tel26_6 team menu latest scores");
        menubar_text(0xa5a728);
        let w = menubar_heading(2, row, 0x1021, 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), MENUBAR_TEXT, 0x418, area);
        write_u16(rec + menubar_record::CONTINUE_WIDGET, w as u16);
        write_u8(rec + menubar_record::CONTINUE_STATE, 3);
        write_i32(menubar_slot(MENUBAR_SLOT_CONTINUE_TIME), -1);
    } else if menubar_being_created()
        || (core::mem::transmute::<usize, MenubarThis0>(0x7fed30)(SCREEN_MANAGER) != 0 && core::mem::transmute::<usize, MenubarThis0>(0x7fedd0)(SCREEN_MANAGER) == 0)
        || context != 0
    {
        covered!("tel26_6 team menu continue disabled");
        menubar_text(0xa16d24);
        let w = menubar_heading(1, row, 0x1021, 0x2c, 1, colour_word(G_GUI_COLOUR_PALE_GREY), MENUBAR_TEXT, 0, area);
        write_u16(rec + menubar_record::CONTINUE_WIDGET, w as u16);
        write_u8(rec + menubar_record::CONTINUE_STATE, 2);
        write_i32(menubar_slot(MENUBAR_SLOT_CONTINUE_TIME), -1);
    } else if exe::screenmanager_is_manager_done(SCREEN_MANAGER, read_usize(menubar_slot(MENUBAR_SLOT_MANAGER)) as u32) != 0 {
        covered!("tel26_6 team menu waiting for players");
        menubar_text(0xa16d34);
        let w = menubar_heading(2, row, 0x1061, 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), MENUBAR_TEXT, 0x3e8, area);
        write_u16(rec + menubar_record::CONTINUE_WIDGET, w as u16);
        write_u8(rec + menubar_record::CONTINUE_STATE, 1);
        write_i32(menubar_slot(MENUBAR_SLOT_CONTINUE_TIME), -1);
    } else {
        covered!("tel26_6 team menu continue");
        let mut font = 1usize;
        let mut colour = colour_word(G_GUI_COLOUR_PALE_GREY);
        menubar_text(0xa16d24);
        if read_u8(rec + menubar_record::CONTINUE_STATE) == 3 {
            exe::at_946ebb(menubar_slot(MENUBAR_SLOT_CONTINUE_TIME) as *mut i32);
        }
        if read_u32(MENUBAR_NETWORK_GAME) != 0
            && read_i32(menubar_slot(MENUBAR_SLOT_CONTINUE_TIME)) != -1
            && m == 0
            && exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER), 0usize) == 0
        {
            covered!("tel26_6 team menu continue countdown");
            let mut now = 0i32;
            exe::at_946ebb(&mut now);
            let d = now.wrapping_sub(read_i32(menubar_slot(MENUBAR_SLOT_CONTINUE_TIME)));
            write_i32(rec + menubar_record::COUNTDOWN, d);
            if d < 5 {
                format(MENUBAR_TEXT, 0x98806c, 5i32.wrapping_sub(d));
                font = 4;
                colour = colour_word(G_GUI_COLOUR_RED);
            }
            write_u32(rec + menubar_record::HOLIDAY_PENDING, 1);
            core::mem::transmute::<usize, MenubarThis0>(0x800000)(SCREEN_MANAGER);
        }
        let w = menubar_heading(2, row, 0x1021, 0xc, font, colour, MENUBAR_TEXT, 0x3e8, area);
        write_u16(rec + menubar_record::CONTINUE_WIDGET, w as u16);
        write_u8(rec + menubar_record::CONTINUE_STATE, 1);
    }
    row += 1;
    menubar_manager_menu(area, row, staff, rec, m);
    row += 1;
    menubar_competitions_menu(area, row);
    menubar_nations_and_find_menus(area, &mut row);
    let change_player: MenubarCdecl3 = core::mem::transmute(FN_MENUBAR_ADD_CHANGE_PLAYER_MENU);
    change_player(area, &mut row as *mut i32 as usize, (m == 0) as usize);
    row += 1;
    menubar_game_options_menu(area, row);
    write_u8(rec + menubar_record::MODE, 2);
    write_i32(rec + menubar_record::CONTEXT, context);
}

/// The match menu (mode 3) of paused match `m`: the minute (`m` +0x5f, +0x4d) in font 5, the arrows, "Paused", "<team>
/// Tactics" of the home and away teams and "Commentary Speed" as inactive text; mode 3 and the context in the record.
pub(crate) unsafe fn menubar_build_match_menu(area: usize, rec: usize, m: usize, context: i32) {
    covered!("tel26_6 match menu");
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: MenubarSprintf = core::mem::transmute(FN_CRT_SPRINTF);
    let mut row = 0i32;
    format(MENUBAR_TEXT, 0x98806c, read_i16(read_usize(m + 0x5f) + 0x4d) as i32);
    menubar_area_text(area, row, 0x1061, 0xc, 5, colour_word(G_GUI_COLOUR_YELLOW));
    row += 1;
    menubar_arrows(row);
    row += 1;
    menubar_text(0xa16d54);
    menubar_heading(1, row, 0x1061, 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), MENUBAR_TEXT, 0, area);
    row += 1;
    for side in [0x20usize, 0x24] {
        let team = Club(read_usize(m + side));
        let g = (team.short_gender_name() as i8) as i32;
        text(MENUBAR_TEXT, 0xa16428, g, g, team.addr() + club::SHORT_NAME);
        menubar_area_text(area, row, 0x1021, 0x2c, 1, colour_word(G_GUI_COLOUR_PALE_GREY));
        row += 1;
    }
    menubar_text(0xa16414);
    menubar_area_text(area, row, 0x1021, 0x2c, 1, colour_word(G_GUI_COLOUR_PALE_GREY));
    write_u8(rec + menubar_record::MODE, 3);
    write_i32(rec + menubar_record::CONTEXT, context);
}

/// The status item at the bottom of the bar (`menubar_record::STATUS_WIDGET` and `STATUS_KIND`), outside the match menu:
/// in an unlocked slot with unread news (`exe::news_count_unread`), "You Have News" in red (0x3e9, kind 1); with a chat
/// message waiting for the manager (`MENUBAR_CHAT_PENDING`), "Chat Message" in red (0x421, kind 2); in an unlocked slot
/// with players waiting (`count_human_managers`(1)), "Player Waiting" (2 humans) or "<waiting> <total> Players" in
/// yellow (0x3fc, kind 3, the counts in `menubar_record::WAITING` and `TOTAL`); otherwise an empty item (kind 0). The
/// item kind is 0x802 with the config flag 0x20000, else 2. The counts are zeroed in the other cases.
pub(crate) unsafe fn menubar_build_status(rec: usize, mode: u8) {
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let kind = || if read_u32(read_usize(G_CONFIG) + 4) & 0x20000 == 0 { 2 } else { 0x802 };
    let status = |colour: usize, text: usize, id: usize| {
        menubar_item([kind(), 10, 0x226, 0x50, 0x24e, 0, 0, 1, colour_word(G_GUI_COLOUR_BLACK), colour_word(G_GUI_COLOUR_BLACK), 0xc, 1, colour,
            text, 0, id, 0, usize::MAX]) as u16
    };
    let mut kind_built = 0u8;
    let mut widget = None;
    if read_u16(menubar_slot(MENUBAR_SLOT_LOCKED)) == 0
        && read_usize(G_NEWS) != 0
        && exe::news_count_unread(read_usize(G_NEWS), read_usize(menubar_slot(MENUBAR_SLOT_MANAGER)), 0) > 0
        && mode != 3
    {
        covered!("tel26_6 status news");
        menubar_text(0xa5a3e0);
        widget = Some(status(colour_word(G_GUI_COLOUR_RED), MENUBAR_TEXT, 0x3e9));
        kind_built = 1;
    }
    if widget.is_none() {
        let s = read_usize(menubar_slot(MENUBAR_SLOT_MANAGER));
        let chat = s != 0 && {
            let k = read_u16(s).wrapping_sub(read_u16(G_STAFF_COUNT)).wrapping_add(0x10) as i16;
            read_u32((MENUBAR_CHAT_PENDING as isize + k as isize * 4) as usize) != 0
        } && mode != 3;
        if chat {
            covered!("tel26_6 status chat");
            menubar_text(0xa5a3d0);
            widget = Some(status(colour_word(G_GUI_COLOUR_RED), MENUBAR_TEXT, 0x421));
            kind_built = 2;
        } else if read_u16(menubar_slot(MENUBAR_SLOT_LOCKED)) == 0 {
            let waiting = menubar_count_humans(1);
            if waiting > 0 && mode != 3 {
                covered!("tel26_6 status players waiting");
                let total = menubar_count_humans(0);
                if total == 2 {
                    menubar_text(0xa5a3c0);
                } else {
                    text(MENUBAR_TEXT, 0xa5a394, waiting, total);
                }
                let w = status(colour_word(G_GUI_COLOUR_YELLOW), MENUBAR_TEXT, 0x3fc);
                write_u16(rec + menubar_record::STATUS_WIDGET, w);
                write_u8(rec + menubar_record::STATUS_KIND, 3);
                write_i32(rec + menubar_record::WAITING, waiting);
                write_i32(rec + menubar_record::TOTAL, total);
                return;
            }
        }
    }
    let w = match widget {
        Some(w) => w,
        None => status(colour_word(G_GUI_COLOUR_BLACK), MENUBAR_EMPTY, 0),
    };
    write_u16(rec + menubar_record::STATUS_WIDGET, w);
    write_u8(rec + menubar_record::STATUS_KIND, kind_built);
    write_i32(rec + menubar_record::WAITING, 0);
    write_i32(rec + menubar_record::TOTAL, 0);
}

/// A colour word zero-extended (the original pushes a register with only its low half loaded).
pub(crate) unsafe fn tel35_colour(a: usize) -> u32 {
    read_u16(a) as u32
}

/// A cdecl routine of the exe with one argument, returning eax.
pub(crate) unsafe fn tel35_cdecl1(f: usize, a: usize) -> usize {
    let g: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(f);
    g(a)
}

/// A chat line (`line` of the human `human`'s page).
pub(crate) fn chat_line(human: i32, line: i32) -> usize {
    (CHAT_PAGES as isize + human as isize * CHAT_PAGE_SIZE as isize + line as isize * CHAT_LINE_SIZE as isize) as usize
}

/// The human index of the slot manager (his staff id − the staff count + 16, as a word).
pub(crate) unsafe fn chat_human() -> i16 {
    (read_u16(tel26_slot_manager().addr()).wrapping_sub(read_u16(G_STAFF_COUNT))).wrapping_add(0x10) as i16
}

/// A menu entry item (`Display::add_item` kind `kind`, flags 0x10) in the menu entry colours, with its text, id, data
/// and area.
pub(crate) unsafe fn menu_list_entry(kind: u32, text: u32, id: u32, data: u32, area: u32) -> u32 {
    tel26_add_item(current_human_display(), [kind, 0, 0, 0, 0, 0, 0, 0x10, tel35_colour(MENU_ENTRY_COLOUR), tel35_colour(G_GUI_COLOUR_YELLOW), 1, 1,
        tel35_colour(G_GUI_COLOUR_BLACK), text, 0, id, data, area])
}

/// The text of a display item of the current human (+0x80); none (a null address) for -1.
pub(crate) unsafe fn chat_item_text(item: i16) -> usize {
    if item == -1 { 0 } else { display_item_at(current_human_display(), item) + display_item::TEXT }
}

/// `comp_screens_open_comp_screen(comp, stage, round, 0)` (0x48f4a0, cdecl).
pub(crate) unsafe fn link_open_comp(comp: usize, stage: i32, round: i32) {
    exe::comp_screens_open_comp_screen(comp, stage, round, 0);
}

/// `staff_screens_open_staff_profile(person, tab, kind, 0, extra)` (0x8702a0, cdecl).
pub(crate) unsafe fn link_open_profile(person: u32, table: u32, kind: u32, extra: u32) {
    exe::staff_screens_open_staff_profile(person, table, kind, 0, extra);
}

/// `open_club_squad_screen(club, reserves, 0, 0, 0)` (0x453d60, cdecl).
pub(crate) unsafe fn link_open_squad(club: u32, reserves: u32) {
    exe::open_club_squad_screen((club) as u32 as usize, (reserves) as u32 as usize, 0, 0, 0);
}

/// A thiscall reader of the current display or the screen manager taking the raw item (`Display::get_widget_id`
/// 0x468b10, `Display::get_widget_data` 0x4739a0, `ScreenManager::get_widget_action_id` 0x468b40,
/// `get_widget_action_data` 0x468b90).
pub(crate) unsafe fn link_widget(f: usize, this: usize, raw: u32) -> u32 {
    let g: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(f);
    g(this, raw)
}

/// The menu bar record of the slot's manager: 0 without a manager or when he is not among the last 16 staff records
/// (`[manager] − G_STAFF_COUNT + 16`, signed, outside 0..16).
pub(crate) unsafe fn menubar_command_record() -> usize {
    let manager = read_usize(menubar_slot(MENUBAR_SLOT_MANAGER));
    if manager == 0 {
        return 0;
    }
    let k = read_u32(manager).wrapping_sub(read_u32(G_STAFF_COUNT)).wrapping_add(0x10) as i32;
    if !(0..0x10).contains(&k) {
        return 0;
    }
    MENUBAR_RECORDS + k as usize * MENUBAR_RECORD_SIZE
}

/// Rule: the menu bar's periodic work, on every call.
///
/// With the slot's record and no box answer to dispatch (`TEL35_NETWORK_FLAG`, the screen manager's "a box answered"
/// flag): in the team menu (mode 2) the auto save (`menubar_command_autosave`), the Continue button
/// (`menubar_command_continue_button`) and the flashing header (`menubar_command_header`); in every menu the status
/// item (`menubar_command_status`). The auto save and the Continue button can end the routine.
pub(crate) unsafe fn menubar_command_periodic(t: &mut MenubarCommandTick) -> Option<i32> {
    if t.rec == 0 || read_u32(TEL35_NETWORK_FLAG) != 0 {
        return None;
    }
    if read_u8(t.rec + menubar_record::MODE) == 2 {
        if let Some(r) = menubar_command_autosave(t) {
            return Some(r);
        }
        if let Some(r) = menubar_command_continue_button(t) {
            return Some(r);
        }
        menubar_command_header(t);
    }
    menubar_command_status(t);
    None
}

/// Rule: with no widget (item −1) the bar continues the game by itself.
///
/// The holiday countdown (the record's `HOLIDAY_PENDING`): after 5 s (time() − the slot's `MENUBAR_SLOT_CONTINUE_TIME`)
/// it stops (the time −1, the flag 0) and continues (`menubar_command_continue`); before, the seconds left go to the
/// Continue item when they change (`COUNTDOWN`). Then, while the manager is not done: a manager on holiday
/// (`HUMAN_MANAGER::is_on_holiday`) goes on (`menubar_command_go_on`) 2 s after his holiday stamp; with no manager, in
/// interactive mode (`G_INTERACTIVE_MODE_FLAG`) outside a network game (or one with more than one screen record and a
/// local slot), 5 s after the menu bar was built (`MENUBAR_BUILD_TIME`, renewed while the mouse button is down). With a
/// widget, a manager on holiday gets his holiday stamp renewed.
pub(crate) unsafe fn menubar_command_idle(t: &mut MenubarCommandTick, item: i16) -> Option<i32> {
    let human_manager = read_usize(G_HUMAN_MANAGER);
    if item != -1 {
        if menubar_command_manager() != 0 && exe::human_manager_is_on_holiday(human_manager, 0usize) != 0 && !t.dry {
            exe::human_manager_set_holiday_stamp(read_usize(G_HUMAN_MANAGER), exe::utils_get_time_ms(), 0usize);
        }
        return None;
    }
    if t.rec != 0 && read_u32(t.rec + menubar_record::HOLIDAY_PENDING) != 0 {
        t.seconds = true;
        let mut now = 0i32;
        exe::at_946ebb(&mut now);
        let elapsed = now.wrapping_sub(read_i32(menubar_slot(MENUBAR_SLOT_CONTINUE_TIME)));
        if elapsed > 5 {
            if t.dry {
                return Some(t.give_up());
            }
            covered!("tel35_7 countdown over");
            write_i32(menubar_slot(MENUBAR_SLOT_CONTINUE_TIME), -1);
            write_u32(t.rec + menubar_record::HOLIDAY_PENDING, 0);
            return Some(menubar_command_continue(t));
        }
        if read_i32(t.rec + menubar_record::COUNTDOWN) != elapsed {
            covered!("tel35_7 countdown");
            let format: MenubarSprintf = core::mem::transmute(FN_CRT_SPRINTF);
            format(t.text, 0x98806c, 5 - elapsed);
            if !t.dry {
                exe::screenmanager_set_widget_text(SCREEN_MANAGER, read_u16(t.rec + menubar_record::CONTINUE_WIDGET) as u32, t.text);
            }
            menubar_command_redraw(t, 0);
            write_i32(t.rec + menubar_record::COUNTDOWN, elapsed);
        }
    }
    let manager = menubar_command_manager();
    if exe::screenmanager_is_manager_done(SCREEN_MANAGER, manager as u32) != 0 {
        return None;
    }
    if manager != 0 {
        if exe::human_manager_is_on_holiday(human_manager, 0usize) != 0 {
            let stamp = core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32) -> i32>(0x5fdc30)(read_usize(G_HUMAN_MANAGER), 0)
                .wrapping_add(2000);
            t.mark_ms(stamp);
            if t.now_ms() >= stamp {
                covered!("tel35_7 holiday goes on");
                return Some(menubar_command_go_on(t));
            }
        }
        return None;
    }
    if read_u32(G_INTERACTIVE_MODE_FLAG) != 0
        && (read_u32(MENUBAR_NETWORK_GAME) == 0
            || (core::mem::transmute::<usize, MenubarThis0>(0x8033c0)(SCREEN_MANAGER) as i16 > 1 && !menubar_command_remote()))
    {
        t.seconds = true;
        let mut now = 0i32;
        exe::at_946ebb(&mut now);
        let mut built = read_i32(MENUBAR_BUILD_TIME);
        if read_u32(G_MOUSE_LEFT_BUTTON_DOWN) != 0 {
            built = now;
            if !t.dry {
                write_i32(MENUBAR_BUILD_TIME, now);
            }
        }
        if now >= built.wrapping_add(5) {
            covered!("tel35_7 interactive goes on");
            return Some(menubar_command_go_on(t));
        }
    }
    None
}

/// Rule: the answers of the menu bar's boxes, when the screen manager has one to dispatch.
///
/// By the code (`G_PENDING_COMMAND_CODE`, cleared) and the answer (`G_PENDING_COMMAND_ARG`, cleared), in this order:
/// - `GO_ON_HOLIDAY` Yes: a remote slot with at most one active screen record asks "Exit the network game as you go on
///   holiday ?" (the manager's confirm, code `EXIT_NETWORK_ON_HOLIDAY`, -4); otherwise the holiday starts and the club's
///   shortlist is rebuilt (`Shortlist_Manager::rebuild_club_shortlist`).
/// - `EXIT_NETWORK_ON_HOLIDAY`: the holiday starts; Yes gives -12.
/// - `RETURN_FROM_HOLIDAY`: No keeps the holiday (it starts again); otherwise a club manager with the holiday option 3
///   gets his saved line-up back (`Tactics::clear_or_restore_selection`, then each of the 16 saved players still at
///   the club `Tactics::set_selected_player`).
/// - The saves (`SAVE_GAME`, `AUTO_SAVE`, `SAVE_BEFORE_RESTART`, `SAVE_BEFORE_EXIT`): an auto save answered with no name
///   sets the Config option 0x1000; the Config is applied and saved; with a typed path: a flagged screen gives the
///   "cannot be saved while a player is on a response screen" box (-4); otherwise the save name becomes the path's
///   (`_splitpath`), the game is saved (`save_game_data` 0x16; a failure gives its box, -4); the path is freed; then a
///   restart (`restart_game`) or an exit, and a redraw of the whole screen.
/// - `RESERVE_TEAM` and `NATIONAL_TEAMS` Yes: the control of the reserve team (of all the national teams) flips.
/// - `RESTART_GAME` Yes: a flagged screen gives the "cannot be restarted" box (-4); otherwise the save dialog "Save
///   Game Before Restart" (`menubar_command_save_first`, -4) or the restart.
/// - `EXIT_GAME` (also clearing the current command 0x402) Yes: a remote slot gives -12; a flagged screen the "cannot be
///   exited" box (-4); otherwise the save dialog "Save Game Before Exit" (-4) or the end of the program.
pub(crate) unsafe fn menubar_command_answer(rec: usize) -> Option<i32> {
    let code = || read_u32(G_PENDING_COMMAND_CODE);
    let answer = || read_u32(G_PENDING_COMMAND_ARG);
    let clear_answer = || write_u32(G_PENDING_COMMAND_ARG, 0);
    if code() == menubar_answer::GO_ON_HOLIDAY {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        if answer() == 2 {
            clear_answer();
            if menubar_command_remote() && core::mem::transmute::<usize, MenubarThis0>(0x7feb70)(SCREEN_MANAGER) as i16 <= 1 {
                covered!("tel35_7 exit network on holiday");
                menubar_command_person_confirm(menubar_command_manager(), 0xa5aa44, menubar_answer::EXIT_NETWORK_ON_HOLIDAY, || {});
                return Some(-4);
            }
            covered!("tel35_7 goes on holiday");
            menubar_command_start_holiday(1);
            let club = read_usize(menubar_command_manager() + staff::CLUB_JOB);
            if club != 0 {
                exe::shortlist_manager_rebuild_club_shortlist(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), club);
            }
        }
        clear_answer();
    }
    if code() == menubar_answer::EXIT_NETWORK_ON_HOLIDAY {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        menubar_command_start_holiday(1);
        if answer() == 2 {
            clear_answer();
            return Some(-12);
        }
        clear_answer();
    }
    if code() == menubar_answer::RETURN_FROM_HOLIDAY {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        let manager = Staff(menubar_command_manager());
        if answer() == 1 {
            clear_answer();
            menubar_command_start_holiday(1);
        } else if manager.club_job().addr() != 0 && exe::human_manager_get_holiday_option(read_usize(G_HUMAN_MANAGER), 3, manager) != 0 {
            covered!("tel35_7 saved line-up back");
            let lineup = exe::human_manager_get_saved_lineup(read_usize(G_HUMAN_MANAGER), manager) as usize;
            let club = manager.club_job().addr();
            exe::tactics_clear_or_restore_selection(read_usize(G_TACTICS), club, 0, 0, 0);
            for k in 0..0x10u32 {
                let player = read_i32(lineup + k as usize * 8);
                if player > -1 {
                    let person = Staff(staff_at(player as isize).addr());
                    if person.club_job() == manager.club_job() {
                        // the original pushes the whole local whose low byte is the slot; the routine reads the byte
                        exe::tactics_set_selected_player(read_usize(G_TACTICS), manager.club_job().addr(), k, person, 0);
                    }
                }
            }
        }
        clear_answer();
    }
    let c = code();
    if matches!(c, menubar_answer::SAVE_GAME | menubar_answer::AUTO_SAVE | menubar_answer::SAVE_BEFORE_RESTART | menubar_answer::SAVE_BEFORE_EXIT) {
        if c == menubar_answer::AUTO_SAVE && answer() == 0 {
            exe::config_set_option(read_usize(G_CONFIG), 0x1000);
        }
        let (restart, exit) = (code() == menubar_answer::SAVE_BEFORE_RESTART, code() == menubar_answer::SAVE_BEFORE_EXIT);
        write_u32(G_PENDING_COMMAND_CODE, 0);
        exe::config_apply_and_save(read_usize(G_CONFIG));
        let path = answer() as usize;
        if path != 0 {
            clear_answer();
            if !menubar_no_flagged_screens() {
                covered!("tel35_7 cannot save on a response screen");
                menubar_command_ok_box(0xa5aa00, None, 0x99081c);
                exe::crt_operator_delete(path);
                return Some(-4);
            }
            // _splitpath(path, 0, 0, the save name, 0): the name part becomes the save name
            exe::crt_strtok(path, 0usize, 0usize, (read_usize(G_CONFIG) + config::SAVE_NAME) as *mut u8, core::ptr::null_mut());
            if exe::save_game_data(path as *const u8, 0x16) == 0 {
                // the path is freed after the "could not be saved" box only; with the disk full or the no memory box
                // it stays allocated, as in the original
                if menubar_command_save_failed(path, None) {
                    exe::crt_operator_delete(path);
                }
                return Some(-4);
            }
            covered!("tel35_7 saved");
            exe::crt_operator_delete(path);
            if restart {
                core::mem::transmute::<usize, MenubarCdecl0>(0x68dbc0)();
            }
            if exit {
                menubar_command_exit_program();
            }
            exe::screenmanager_redraw_screen(SCREEN_MANAGER, 1, 0);
        }
    }
    if code() == menubar_answer::RESERVE_TEAM {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        if answer() == 2 {
            covered!("tel35_7 reserve team control");
            let on = exe::human_manager_controls_reserve_team(read_usize(G_HUMAN_MANAGER), 0usize) == 0;
            core::mem::transmute::<usize, MenubarThis2>(0x5fd590)(read_usize(G_HUMAN_MANAGER), on as usize, 0);
        }
        clear_answer();
    }
    if code() == menubar_answer::NATIONAL_TEAMS {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        if answer() == 2 {
            covered!("tel35_7 national teams control");
            let on = core::mem::transmute::<usize, MenubarThis2>(0x5fd6b0)(read_usize(G_HUMAN_MANAGER), 0, 0) == 0;
            core::mem::transmute::<usize, MenubarThis2>(0x5fd810)(read_usize(G_HUMAN_MANAGER), on as usize, 0);
        }
        clear_answer();
    }
    if code() == menubar_answer::RESTART_GAME {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        if answer() == 2 {
            clear_answer();
            if !menubar_no_flagged_screens() {
                covered!("tel35_7 cannot restart");
                menubar_command_ok_box(0xa5a9b8, None, 0xa5a9a4);
                return Some(-4);
            }
            if menubar_command_save_first(rec) {
                covered!("tel35_7 save before restart");
                menubar_command_file_dialog_copy(0xa5a988, menubar_answer::SAVE_BEFORE_RESTART, 0);
                return Some(-4);
            }
            core::mem::transmute::<usize, MenubarCdecl0>(0x68dbc0)();
        }
        clear_answer();
    }
    if code() == menubar_answer::EXIT_GAME {
        if read_u16(G_CURRENT_COMMAND_ID) == 0x402 {
            write_u16(G_CURRENT_COMMAND_ID, 0);
        }
        let yes = answer() == 2;
        write_u32(G_PENDING_COMMAND_CODE, 0);
        clear_answer();
        if yes {
            if menubar_command_remote() {
                return Some(-12);
            }
            if !menubar_no_flagged_screens() {
                covered!("tel35_7 cannot exit");
                menubar_command_ok_box(0xa5a944, None, 0xa5a934);
                return Some(-4);
            }
            if menubar_command_save_first(rec) {
                covered!("tel35_7 save before exit");
                menubar_command_file_dialog(0xa5a91c, read_usize(G_CONFIG) + config::SAVE_NAME, menubar_answer::SAVE_BEFORE_EXIT, 0);
                return Some(-4);
            }
            menubar_command_exit_program();
        }
    }
    None
}

/// Rule: the command of a widget (or of the current command with no widget), after the periodic work.
///
/// The Back and Forward arrows (−2, −3) give their id when they are available (`is_back_unavailable(0)`,
/// `is_forward_unavailable`); then the team bar and manager's menu (`menubar_command_manager_menu`), the Competitions and
/// Find menus (`menubar_command_find_menus`), the manager switch (`menubar_command_switch_manager`), 0x3fe "Save Game"
/// (a local slot, no manager being created, no flagged screen: clearing the current command 0x3fe, the save dialog
/// with the Config's name, code `SAVE_GAME`, last flag 1, -4) and the game menu (`menubar_command_game_menu`). The id
/// is read again after the switch (the slot changed) and after "Save Game".
pub(crate) unsafe fn menubar_command_dispatch(t: &mut MenubarCommandTick, item: i16, rec: usize) -> i32 {
    if tel26_item_id(item) == 0xfffe && exe::screenmanager_is_back_unavailable(SCREEN_MANAGER, 0) == 0 {
        return -2;
    }
    if tel26_item_id(item) == 0xfffd && exe::screenmanager_is_forward_unavailable(SCREEN_MANAGER) == 0 {
        return -3;
    }
    let id = tel26_item_id(item);
    if let Some(r) = menubar_command_manager_menu(t, id, item) {
        return r;
    }
    if let Some(r) = menubar_command_find_menus(id, item) {
        return r;
    }
    if id == 0x3fd {
        if let Some(r) = menubar_command_switch_manager(item, rec) {
            return r;
        }
    }
    if tel26_item_id(item) == 0x3fe && !menubar_command_remote() && !menubar_being_created() && menubar_no_flagged_screens() {
        covered!("tel35_7 save game");
        if read_u16(G_CURRENT_COMMAND_ID) == 0x3fe {
            write_u16(G_CURRENT_COMMAND_ID, 0);
        }
        menubar_command_file_dialog_copy(0xa5a4b4, menubar_answer::SAVE_GAME, 1);
        return -4;
    }
    menubar_command_game_menu(tel26_item_id(item))
}

/// The fixed answer of a command after the periodic work, for the comparer (none: it depends on what runs).
pub(crate) unsafe fn menubar_command_expected(id: u16) -> Option<i32> {
    match id {
        0 => Some(0),
        0xfffe => Some(if exe::screenmanager_is_back_unavailable(SCREEN_MANAGER, 0) == 0 { -2 } else { 0 }),
        0xfffd => Some(if exe::screenmanager_is_forward_unavailable(SCREEN_MANAGER) == 0 { -3 } else { 0 }),
        0x42d => Some(-11),
        0x3fb => Some(if menubar_command_manager() == 0 { -4 } else { -11 }),
        0x402 => Some(if menubar_no_flagged_screens() { -4 } else { 0 }),
        0x418 | 0x42e | 0x41e | 0x425 | 0x433 | 0x41d | 0x3f2 | 0x41c | 0x420 | 0x41f | 0x434 | 0x3e9 | 0x3eb | 0x415 | 0x3ec | 0x3ef
        | 0x3f0 | 0x414 | 0x3f3 | 0x40c | 0x3f4..=0x3fa | 0x429 | 0x3fc | 0x421 | 0x431 | 0x42a | 0x42b | 0x42c | 0x42f => Some(-4),
        _ => None,
    }
}

/// `langlib_sprintf` into the text buffer, with no arguments.
pub(crate) unsafe fn menubar_text(format: usize) {
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    text(MENUBAR_TEXT, format);
}

/// `ScreenManager::add_area_text` of the text buffer in column 0; returns the widget (AX).
pub(crate) unsafe fn menubar_area_text(area: usize, row: i32, style: usize, flags: usize, font: usize, colour: usize) -> u16 {
    let f: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    f(SCREEN_MANAGER, area, 0, row as usize, style, 0, flags, font, colour, MENUBAR_TEXT) as u16
}

/// `ScreenManager::add_area_button` with data 0.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn menubar_area_button(area: usize, column: usize, row: i32, flags: usize, font: usize, colour: usize, text: usize, id: usize) {
    let f: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    f(SCREEN_MANAGER, area, column, row as usize, 0x1021, 0, flags, font, colour, text, id, 0);
}

/// A menu heading or a top-level item of the bar (`kind` 0x82 opens a menu, 2 an item, 1 an inactive item).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn menubar_heading(kind: usize, row: i32, style: usize, flags: usize, font: usize, colour: usize, text: usize, id: usize, area: usize) -> usize {
    menubar_item([kind, 0, 0, 0, 0, 0, row as usize, style, 0, 0, flags, font, colour, text, 0, id, 0, area])
}

/// The `is_current_manager_being_created` (0x838de0) test.
pub(crate) unsafe fn menubar_being_created() -> bool {
    core::mem::transmute::<usize, MenubarCdecl0>(0x838de0)() != 0
}

/// `count_human_managers` (0x838d10).
pub(crate) unsafe fn menubar_count_humans(connected_only: usize) -> i32 {
    core::mem::transmute::<usize, MenubarCdecl1>(0x838d10)(connected_only) as i32
}

/// `ScreenManager::count_flagged_screens` (0x803d50) is zero (AX).
pub(crate) unsafe fn menubar_no_flagged_screens() -> bool {
    core::mem::transmute::<usize, MenubarThis0>(0x803d50)(SCREEN_MANAGER) as u16 == 0
}

/// "Add Manager" can be chosen: a free human manager row (0x838c60), no manager being created and the human
/// manager object's +8 clear.
pub(crate) unsafe fn menubar_can_add_manager() -> bool {
    core::mem::transmute::<usize, MenubarCdecl0>(0x838c60)() != 0 && !menubar_being_created() && read_u32(read_usize(G_HUMAN_MANAGER) + 8) == 0
}

/// "Chat" is offered: a network game and a manager in the slot with a first or second name [inference: in memory the
/// name fields hold pointers to the strings].
pub(crate) unsafe fn menubar_can_chat() -> bool {
    if read_u32(MENUBAR_NETWORK_GAME) == 0 {
        return false;
    }
    let s = Staff(read_usize(menubar_slot(MENUBAR_SLOT_MANAGER)));
    !s.is_null() && (read_u8(s.first_name()) != 0 || read_u8(s.second_name()) != 0)
}

/// The "<<<" and ">>>" (or the symbol font's 0x28 and 0x29, `G_FONT_MODE` ≠ −1) buttons in their own area, ids −2 and −3.
pub(crate) unsafe fn menubar_arrows(row: i32) {
    let format: MenubarSprintf = core::mem::transmute(FN_CRT_SPRINTF);
    let arrows = menubar_area([5, 10, 0x55, 0x24e, 2, 0, 0xd, 0, 1, 0, usize::MAX]);
    for (column, symbol, text, id) in [(0usize, 0x28usize, 0xa16444usize, -2isize), (1, 0x29, 0xa16440, -3)] {
        if read_i32(G_FONT_MODE) != -1 {
            format(MENUBAR_TEXT, 0x99ec1c, symbol);
            menubar_area_button(arrows, column, row, 0xc, 0, colour_word(G_GUI_COLOUR_YELLOW), MENUBAR_TEXT, id as usize);
        } else {
            menubar_area_button(arrows, column, row, 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), text, id as usize);
        }
    }
}

/// The manager's menu of the team menu.
///
/// With no human manager (`count_human_managers`(0)), no manager or a manager being created: "Manager Options" as
/// inactive text. Otherwise a menu headed by the manager's name (`exe::database_format_staff_name`, broken near the
/// middle): with a national job (`staff::NATIONAL_JOB`), unless it is a national team club
/// (`exe::club_is_national_team`), its squad (0x7d5) and, with a reserve squad, its 'B' squad (0x7d6); the U21 squad
/// (U23 when the nation's flag in `G_NATIONAL_TEAMS` + 0x36 is set; the team of `exe::database_get_national_u21_team`);
/// with the senior team only or not all teams available (`HUMAN_MANAGER::controls_all_national_teams`), "Control Senior
/// Team Only"/"Control All Teams" (0x433); "FA Confidence" (0x420), "Resign from Nation" (0x41d, greyed out in a locked
/// slot or with a paused match) and a separator. With a club job (`staff::CLUB_JOB`): its squad, its reserves (0x7d6)
/// and "Control Senior Team Only"/"Control Reserve Team" (0x425) when it has a reserve squad, the linked B team's squad
/// (`exe::club_get_linked_b_team`), "Board Confidence" (0x41f), "Resign from Club" (0x41c, same rule) and a separator.
/// Then News (0x3e9), Player & Staff Search (0x3eb), Compare two chosen players (0x434), Manager Stats (0x42e), Job
/// Information (0x41e), Transfers (0x415), History (0x3ec); not on holiday: "Go on Holiday" (0x3ef, same rule) and the
/// pending click on a hidden item 0x42d when the countdown started; on holiday: the pending click on a hidden item
/// 0x3f0 when the screen manager has at most one active record (else none); with more than one human manager, "Send
/// Abuse To" with each other human manager (staff classification 1 among the last 16 staff records, 0x414 with the
/// person) and "All" (0x414); a separator and "Retire" (0x3f2, same rule).
pub(crate) unsafe fn menubar_manager_menu(area: usize, row: i32, staff: usize, rec: usize, m: usize) {
    let staff = Staff(staff);
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if menubar_count_humans(0) == 0 || staff.is_null() || menubar_being_created() {
        covered!("tel26_6 manager menu inactive");
        menubar_text(0xa5a558);
        menubar_area_text(area, row, 0x1021, 0x2c, 1, colour_word(MENUBAR_HEADING_COLOUR));
        return;
    }
    covered!("tel26_6 manager menu");
    exe::database_format_staff_name(staff, MENUBAR_TEXT, 0x7d0, 1);
    exe::utils_break_line_near_middle(MENUBAR_TEXT as *mut u8, 1, 1);
    let heading = menubar_heading(0x82, row, 0x1021, 0xc, 1, colour_word(MENUBAR_HEADING_COLOUR), MENUBAR_TEXT, 0, area);
    let menu = menubar_submenu(heading);
    let hm = || read_usize(G_HUMAN_MANAGER);
    let free = || read_u16(menubar_slot(MENUBAR_SLOT_LOCKED)) == 0 && m == 0;
    let squad = |format: usize, team: usize| {
        let g = read_i8(team + club::SHORT_GENDER_NAME) as i32;
        text(MENUBAR_TEXT, format, g, g, team + club::SHORT_NAME);
    };
    let national = staff.national_job();
    if !national.is_null() {
        covered!("tel26_6 manager menu national job");
        if exe::club_is_national_team(national) == 0 {
            squad(0xa5a708, national.addr());
            menubar_entry(MENUBAR_TEXT, 1, 0x7d5, national.addr(), menu);
            if exe::database_club_has_reserve_squad(national) != 0 {
                squad(0x9b76e8, national.addr());
                menubar_entry(MENUBAR_TEXT, 1, 0x7d6, national.addr(), menu);
            }
        }
        let nation = national.nation().addr();
        let u23 = read_u8(read_usize(read_usize(G_NATIONAL_TEAMS) + 0x36) + read_usize(nation)) != 0;
        squad(if u23 { 0xa5a6e4 } else { 0xa5a6c0 }, national.addr());
        let u21 = exe::database_get_national_u21_team(national.nation().addr());
        menubar_entry(MENUBAR_TEXT, 1, 0x7d5, u21, menu);
        let all_teams: MenubarThis2 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_ALL_NATIONAL_TEAMS);
        let mut not_available = 0u32;
        if all_teams(hm(), 0, &mut not_available as *mut u32 as usize) != 0 || not_available == 0 {
            let only = all_teams(hm(), 0, 0) != 0;
            menubar_text(if only { 0xa5a6a4 } else { 0xa5a690 });
            menubar_entry(MENUBAR_TEXT, 1, 0x433, 0, menu);
        }
        menubar_text(0xa5a680);
        menubar_entry(MENUBAR_TEXT, 1, 0x420, 0, menu);
        menubar_text(0xa5a66c);
        menubar_entry_if(free(), 0x41d, menu);
        menubar_separator(menu);
    }
    let club = staff.club_job().addr();
    if club != 0 {
        covered!("tel26_6 manager menu club job");
        squad(0xa5a650, club);
        menubar_entry(MENUBAR_TEXT, 1, 0x7d5, club, menu);
        if exe::database_club_has_reserve_squad(club) != 0 {
            squad(0x990728, club);
            menubar_entry(MENUBAR_TEXT, 1, 0x7d6, club, menu);
            let only = exe::human_manager_controls_reserve_team(hm(), 0usize) != 0;
            menubar_text(if only { 0xa5a6a4 } else { 0xa5a638 });
            menubar_entry(MENUBAR_TEXT, 1, 0x425, 0, menu);
        }
        let b = exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1);
        if b != 0 {
            squad(0xa5a650, b);
            menubar_entry(MENUBAR_TEXT, 1, 0x7d5, b, menu);
        }
        menubar_text(0xa1549c);
        menubar_entry(MENUBAR_TEXT, 1, 0x41f, 0, menu);
        menubar_text(0xa5a624);
        menubar_entry_if(free(), 0x41c, menu);
        menubar_separator(menu);
    }
    for (format, id) in [(0xa5a61cusize, 0x3e9usize), (0xa5a604, 0x3eb), (0xa5a5e8, 0x434), (0xa15324, 0x42e), (0xa15424, 0x41e), (0x98e880, 0x415)] {
        menubar_text(format);
        menubar_entry(MENUBAR_TEXT, 1, id, 0, menu);
    }
    text(MENUBAR_TEXT, 0xa5a5b8, MENUBAR_EMPTY);
    menubar_entry(MENUBAR_TEXT, 1, 0x3ec, 0, menu);
    let hidden = |id: usize| {
        menubar_item([1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0x2c, 1, colour_word(G_GUI_COLOUR_PALE_GREY), MENUBAR_EMPTY, 0, id, 0, usize::MAX])
    };
    let pending = if exe::human_manager_is_on_holiday(hm(), 0usize) == 0 {
        menubar_text(0xa5a5a8);
        menubar_entry_if(free(), 0x3ef, menu);
        if read_u32(rec + menubar_record::HOLIDAY_PENDING) != 0 {
            covered!("tel26_6 manager menu holiday countdown click");
            hidden(0x42d)
        } else {
            usize::MAX
        }
    } else if core::mem::transmute::<usize, MenubarThis0>(0x7feb70)(SCREEN_MANAGER) as i16 > 1 {
        usize::MAX
    } else {
        covered!("tel26_6 manager menu on holiday click");
        hidden(0x3f0)
    };
    core::mem::transmute::<usize, MenubarThis1>(0x800160)(SCREEN_MANAGER, pending);
    if menubar_count_humans(0) > 1 {
        covered!("tel26_6 manager menu send abuse");
        menubar_text(0xa5a598);
        let item = menubar_entry(MENUBAR_TEXT, 1, 0, 0, menu);
        let abuse = menubar_submenu(item);
        for i in 0..0x10isize {
            let e = Staff(staff_at(((read_i32(G_STAFF_COUNT) as isize - i - 1)) as isize).addr());
            if e.classification() == 1 && e.addr() != read_usize(menubar_slot(MENUBAR_SLOT_MANAGER)) {
                let mut name = [0u8; 200];
                exe::database_format_staff_name(e, name.as_mut_ptr() as usize, 200, 1);
                menubar_entry(name.as_ptr() as usize, 1, 0x414, e.addr(), abuse);
            }
        }
        text(MENUBAR_TEXT, 0xa5a570, MENUBAR_EMPTY);
        menubar_entry(MENUBAR_TEXT, 1, 0x414, 0, abuse);
    }
    menubar_separator(menu);
    menubar_text(0xa5a568);
    menubar_entry_if(free(), 0x3f2, menu);
}

/// The Competitions menu: one submenu per entry of the nation order (`key_nation_sorted_order`(0), 35 entries) whose
/// `G_LEAGUE_TABLE` row has a competition list (+0x10).
///
/// Row 0: "World Cup" (a menu entry); 1: "International"; 2: "Worldwide Club"; a continent (+4): its adjective
/// (`exe::get_continent_adjective`); a nation (+0) with its +0x11c byte set: its name, under the part header
/// (`NATION_MENU_SPLIT::format_header`) when the list is split in two parts and the nation starts another part. A
/// row with neither shows the error box (Menubar.cpp line 0xd3f) and is skipped. Each submenu lists the competitions
/// (`+0xc` of them, `ScreenManager::add_menu_item` with link 0x7d0 and the competition) and, for a nation with awards
/// (`AWARD_MANAGER::find_nation_awards`), a separator and "Awards" (0x7d4). Rows 0 and 1 add a separator, their awards
/// and "FIFA Rankings" (0x3f3); row 3 a separator, its awards and "UEFA Coefficients" (0x40c).
pub(crate) unsafe fn menubar_competitions_menu(area: usize, row: i32) {
    covered!("tel26_6 competitions menu");
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: MenubarSprintf = core::mem::transmute(FN_CRT_SPRINTF);
    let add_menu_item: MenubarThis4 = core::mem::transmute(FN_SCREENMANAGER_ADD_MENU_ITEM);
    let add_menu_area: MenubarThis1 = core::mem::transmute(FN_SCREENMANAGER_ADD_MENU_AREA);
    let separator: MenubarThis1 = core::mem::transmute(FN_MENUBAR_ADD_SEPARATOR);
    let find_nation_awards: MenubarThis1 = core::mem::transmute(FN_AWARD_MANAGER_FIND_NATION_AWARDS);
    menubar_text(0x98ed18);
    let heading = menubar_heading(0x82, row, 0x1021, 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), MENUBAR_TEXT, 0, area);
    let comps = menubar_submenu(heading);
    let mut last = 0xffu8;
    let mut part = 0usize;
    let order = core::mem::transmute::<usize, MenubarCdecl1>(0x66a0c0)(0);
    let mut split = [0u32; 4];
    let s = split.as_mut_ptr() as usize;
    core::mem::transmute::<usize, MenubarThis3>(0x66a280)(s, 0, order, 0);
    let awards_of = |w: usize| read_u32(read_usize(read_usize(w + 4)) + 5) as usize;
    for i in 0..0x23usize {
        let c = read_i8(order + i);
        let entry = (G_LEAGUE_TABLE as isize + c as isize * 0x48) as usize;
        if read_usize(entry + 0x10) == 0 {
            continue;
        }
        let item = match c {
            0 => {
                menubar_text(0xa16284);
                menubar_entry(MENUBAR_TEXT, 1, 0, 0, comps)
            }
            1 => {
                text(MENUBAR_TEXT, 0xa16248, MENUBAR_EMPTY);
                add_menu_item(SCREEN_MANAGER, comps, MENUBAR_TEXT, 0, 0) as usize
            }
            2 => {
                text(MENUBAR_TEXT, 0xa16210, MENUBAR_EMPTY);
                add_menu_item(SCREEN_MANAGER, comps, MENUBAR_TEXT, 0, 0) as usize
            }
            _ if read_usize(entry + 4) != 0 => {
                let mut adjective = [0u8; 100];
                exe::get_continent_adjective(read_usize(entry + 4), adjective.as_mut_ptr(), 100, 0x14);
                add_menu_item(SCREEN_MANAGER, comps, adjective.as_ptr() as usize, 0, 0) as usize
            }
            _ => {
                let nation = read_usize(entry);
                if nation == 0 {
                    covered!("tel26_6 competitions menu error box");
                    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
                    exe::crt_strtok(0xa5a51cusize, 0usize, 0usize, name.as_mut_ptr(), ext.as_mut_ptr());
                    format(0xae24d0, 0x9870e8, exe::utils_get_version_string(), name.as_ptr(), ext.as_ptr(), 0xd3f);
                    exe::gui_show_message_box(0x9870e0usize, 0xae24d0usize, 0);
                    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
                    continue;
                }
                if read_u8(nation + 0x11c) == 0 {
                    continue;
                }
                if exe::nation_menu_split_part_count(s) as i8 > 1 && exe::nation_menu_split_in_second_part(s, nation) != last {
                    covered!("tel26_6 competitions menu split header");
                    last = exe::nation_menu_split_in_second_part(s, nation);
                    core::mem::transmute::<usize, MenubarThis3>(0x66a360)(s, MENUBAR_TEXT, 0x7d0, last as usize);
                    let header = add_menu_item(SCREEN_MANAGER, comps, MENUBAR_TEXT, 0, 0) as usize;
                    part = add_menu_area(SCREEN_MANAGER, header) as usize;
                } else if exe::nation_menu_split_part_count(s) == 1 {
                    part = comps;
                }
                add_menu_item(SCREEN_MANAGER, part, nation + 4, 0, 0) as usize
            }
        };
        let menu = add_menu_area(SCREEN_MANAGER, item) as usize;
        if read_usize(entry + 0x10) != 0 {
            let mut k = 0;
            while k < read_i32(entry + 0xc) {
                let e = read_usize(read_usize(entry + 0x10) + k as usize * 4);
                if e != 0 {
                    let comp = read_usize(e + 4);
                    add_menu_item(SCREEN_MANAGER, menu, comp + 0x38, 0x7d0, comp);
                }
                k += 1;
            }
            let nation = read_usize(entry);
            if nation != 0 {
                let awards = find_nation_awards(read_usize(G_AWARD_MANAGER), nation) as usize;
                if awards != 0 {
                    separator(SCREEN_MANAGER, menu);
                    menubar_text(0x99c0a8);
                    add_menu_item(SCREEN_MANAGER, menu, MENUBAR_TEXT, 0x7d4, awards_of(awards));
                }
            }
        }
        let c = read_u8(order + i);
        if c == 0 || c == 1 {
            separator(SCREEN_MANAGER, menu);
            let lists = read_usize(read_usize(G_AWARD_MANAGER) + 8);
            let w = if read_u8(order + i) == 0 { read_usize(lists) } else { read_usize(lists + 4) };
            menubar_text(0x99c0a8);
            add_menu_item(SCREEN_MANAGER, menu, MENUBAR_TEXT, 0x7d4, awards_of(w));
            menubar_text(0xa5a548);
            add_menu_item(SCREEN_MANAGER, menu, MENUBAR_TEXT, 0x3f3, 0);
        }
        if read_u8(order + i) == 3 {
            separator(SCREEN_MANAGER, menu);
            let w = read_usize(read_usize(read_usize(G_AWARD_MANAGER) + 8) + 8);
            menubar_text(0x99c0a8);
            add_menu_item(SCREEN_MANAGER, menu, MENUBAR_TEXT, 0x7d4, awards_of(w));
            menubar_text(0x99c924);
            add_menu_item(SCREEN_MANAGER, menu, MENUBAR_TEXT, 0x40c, 0);
        }
    }
}

/// The Nations & Clubs menu (Nations 0x3f4, Under-21s 0x3f5, Major Clubs 0x3f6 and Other 0x3f9 listed by continent,
/// 0x765010; League 0x3f7 and Non-League 0x3f8 by league nation, 0x7651e0) and the Find menu (Nation, Club,
/// Non-Player, Player: 0x3fa with 1..4; with the "cm3_history" folder, a config +8 of 0 or 1 and outside a network
/// slot, a separator and "Written History" 0x429). `row` moves to the Find heading.
pub(crate) unsafe fn menubar_nations_and_find_menus(area: usize, row: &mut i32) {
    covered!("tel26_6 nations and find menus");
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    *row += 1;
    menubar_text(0xa5a50c);
    let heading = menubar_heading(0x82, *row, 0x1021, 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), MENUBAR_TEXT, 0, area);
    let menu = menubar_submenu(heading);
    for (format, comment, id, list) in [
        (0x9c9f64usize, false, 0x3f4usize, 0x765010usize),
        (0x9c9f58, false, 0x3f5, 0x765010),
        (0x9c9f4c, false, 0x3f6, 0x765010),
        (0x990f4c, false, 0x3f7, 0x7651e0),
        (0x9c9f40, false, 0x3f8, 0x7651e0),
        (0x9c9f1c, true, 0x3f9, 0x765010),
    ] {
        if comment {
            text(MENUBAR_TEXT, format, MENUBAR_EMPTY);
        } else {
            menubar_text(format);
        }
        let item = menubar_entry(MENUBAR_TEXT, 1, 0, 0, menu);
        let sub = menubar_submenu(item);
        core::mem::transmute::<usize, MenubarCdecl3>(list)(sub, id, 0);
    }
    *row += 1;
    menubar_text(0xa5a504);
    let heading = menubar_heading(0x82, *row, 0x1021, 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), MENUBAR_TEXT, 0, area);
    let find = menubar_submenu(heading);
    for (format, data) in [(0x9ca010usize, 1usize), (0x9ca08c, 2), (0x9ca080, 3)] {
        menubar_text(format);
        menubar_entry(MENUBAR_TEXT, 1, 0x3fa, data, find);
    }
    text(MENUBAR_TEXT, 0xa5a4e0, MENUBAR_EMPTY);
    menubar_entry(MENUBAR_TEXT, 1, 0x3fa, 4, find);
    let folder = exe::utils_build_file_path(0x9cec18usize, MENUBAR_EMPTY);
    if core::mem::transmute::<usize, MenubarCdecl1>(0x90bf90)(folder) != 0
        && matches!(read_u32(read_usize(G_CONFIG) + 8), 0 | 1)
        && read_u16(menubar_slot(screen_manager::NETWORK_SLOT)) == 0
    {
        covered!("tel26_6 find menu written history");
        menubar_separator(find);
        menubar_text(0xa5a4d0);
        menubar_entry(MENUBAR_TEXT, 1, 0x429, 0, find);
    }
}

/// The Game Options menu: Save Game (0x3fe, outside a network slot, with no manager being created and no flagged
/// screen), Add Manager (0x3fb, `menubar_can_add_manager`), Manager Status (0x3fc), Chat (0x421, `menubar_can_chat`),
/// Game Settings (0x423), Selected Leagues (0x431), Hall of Fame (0x42a), Game Credits (0x42b), Web Sites (0x42c),
/// Restart Game (0x42f, outside a network slot and with no flagged screen) and, in a network slot, "Exit Network Game"
/// (0x402, greyed out in a locked slot), otherwise "Exit Game" (0x402, with no flagged screen).
pub(crate) unsafe fn menubar_game_options_menu(area: usize, row: i32) {
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    menubar_text(0xa5a4c0);
    let heading = menubar_heading(0x82, row, 0x1021, 0xc, 1, colour_word(G_GUI_COLOUR_YELLOW), MENUBAR_TEXT, 0, area);
    let menu = menubar_submenu(heading);
    let local = || read_u16(menubar_slot(screen_manager::NETWORK_SLOT)) == 0;
    menubar_text(0xa5a4b4);
    menubar_entry_if(local() && !menubar_being_created() && menubar_no_flagged_screens(), 0x3fe, menu);
    text(MENUBAR_TEXT, 0xa5a48c, MENUBAR_EMPTY);
    menubar_entry_if(menubar_can_add_manager(), 0x3fb, menu);
    menubar_text(0xa5a47c);
    menubar_entry(MENUBAR_TEXT, 1, 0x3fc, 0, menu);
    if menubar_can_chat() {
        menubar_text(0xa5a7a0);
        menubar_entry(MENUBAR_TEXT, 1, 0x421, 0, menu);
    }
    for (format, id) in [(0xa5a46cusize, 0x423usize), (0xa5a458, 0x431), (0x9ce514, 0x42a), (0x9b52fc, 0x42b), (0x9b53b4, 0x42c)] {
        menubar_text(format);
        menubar_entry(MENUBAR_TEXT, 1, id, 0, menu);
    }
    text(MENUBAR_TEXT, 0xa5a42c, MENUBAR_EMPTY);
    menubar_entry_if(local() && menubar_no_flagged_screens(), 0x42f, menu);
    if !local() {
        covered!("tel26_6 game options network exit");
        menubar_text(0xa5a418);
        menubar_entry_if(read_u16(menubar_slot(MENUBAR_SLOT_LOCKED)) == 0, 0x402, menu);
    } else {
        text(MENUBAR_TEXT, 0xa5a3f0, MENUBAR_EMPTY);
        menubar_entry_if(menubar_no_flagged_screens(), 0x402, menu);
    }
}

/// The slot's manager.
pub(crate) unsafe fn menubar_command_manager() -> usize {
    read_usize(menubar_slot(MENUBAR_SLOT_MANAGER))
}

/// The slot belongs to a remote network player (`MENUBAR_SLOT_REMOTE`).
pub(crate) unsafe fn menubar_command_remote() -> bool {
    read_u16(menubar_slot(MENUBAR_SLOT_REMOTE)) != 0
}

/// `ScreenManager::redraw_screen(all, 0)`, not in a dry run.
pub(crate) unsafe fn menubar_command_redraw(t: &MenubarCommandTick, all: u32) {
    if !t.dry {
        exe::screenmanager_redraw_screen(SCREEN_MANAGER, all, 0);
    }
}

/// An Ok box titled with the game's title: the heading `heading` over the message in the text buffer (formatted with
/// `message_format`, the path first when there is one), in the pale grey and the 0xaebdcc colours, code 0.
pub(crate) unsafe fn menubar_command_ok_box(message_format: usize, path: Option<usize>, heading: usize) {
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (grey, back) = (colour_word(G_GUI_COLOUR_PALE_GREY) as u32, colour_word(0xaebdcc) as u32);
    let m = match path {
        Some(path) => text(MENUBAR_TEXT, message_format, path, 0u32, grey, back, 0u32),
        None => text(MENUBAR_TEXT, message_format, 0u32, grey, back, 0u32),
    } as u32 as usize;
    let mut h = [0u8; 200];
    let h = text(h.as_mut_ptr() as usize, heading, m, 0u32, grey, back, 0u32) as u32 as usize;
    let title = menubar_command_game_title(h);
    menubar_command_box(title, h, m, 0, grey, back, 0);
}

/// A person's "Please Confirm" box: `person`'s name (`database_format_staff_name`, 200 bytes) as the title, the colours
/// starting as the heading colours (pale grey, red) and then his club's kit colours (or his nation's, when he has no
/// club), the question `question` (formatted with 1, the colours and the code), answer `code`. `before` runs between the
/// colours and the name (the original sets the holiday there).
pub(crate) unsafe fn menubar_command_person_confirm(person: usize, question: usize, code: u32, before: impl FnOnce()) {
    let person = Staff(person);
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut fore = menubar_command_colour(1);
    let mut back = menubar_command_colour(0);
    before();
    let mut name = [0u8; 200];
    exe::database_format_staff_name(person, name.as_mut_ptr() as usize, 200, 1);
    let club = if person.club_job().addr() != 0 { person.club_job().addr() } else { person.national_job().addr() };
    if club != 0 {
        menubar_command_kit_colours(club, &mut fore, &mut back);
    }
    let m = text(MENUBAR_TEXT, question, 1u32, fore, back, code) as u32 as usize;
    menubar_command_please_confirm(name.as_ptr() as usize, m, fore, back, code);
}

/// The save file dialog (`open_file_dialog` 0x58f500): the prompt "Enter File Name" for the name `name` in the save
/// folder with the ".sav" extension and the name check, the heading `heading`, in the pale grey and red colours, answer
/// `code` and the last flag `last` (1 for the plain "Save Game"). The prompt and the heading are formatted with the
/// dialog's arguments, as the original pushes them once for the three calls.
pub(crate) unsafe fn menubar_command_file_dialog(heading: usize, name: usize, code: u32, last: u32) {
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let folder = read_usize(G_CONFIG) + MENUBAR_CONFIG_SAVE_FOLDER;
    let back = menubar_command_colour(0);
    let fore = menubar_command_colour(1);
    let (mut p, mut h) = ([0u8; 200], [0u8; 200]);
    let p = text(p.as_mut_ptr() as usize, 0x9907f4, 1u32, folder, MENUBAR_SAVE_EXTENSION, name, MENUBAR_SAVE_NAME_CHECK, fore, back, code,
        0u32, 0u32, 0u32, last) as u32 as usize;
    let h = text(h.as_mut_ptr() as usize, heading, p, 1u32, folder, MENUBAR_SAVE_EXTENSION, name, MENUBAR_SAVE_NAME_CHECK, fore, back, code,
        0u32, 0u32, 0u32, last) as u32 as usize;
    #[allow(clippy::type_complexity)]
    let f: unsafe extern "C" fn(usize, usize, u32, usize, usize, usize, usize, u32, u32, u32, u32, u32, u32, u32) =
        core::mem::transmute(FN_OPEN_FILE_DIALOG);
    f(h, p, 1, folder, MENUBAR_SAVE_EXTENSION, name, MENUBAR_SAVE_NAME_CHECK, fore, back, code, 0, 0, 0, last);
}

/// The dialog with the configured save name copied into `MENUBAR_SAVE_NAME`.
pub(crate) unsafe fn menubar_command_file_dialog_copy(heading: usize, code: u32, last: u32) {
    menubar_command_copy_text(MENUBAR_SAVE_NAME, read_usize(G_CONFIG) + config::SAVE_NAME);
    menubar_command_file_dialog(heading, MENUBAR_SAVE_NAME, code, last);
}

/// A save failed (`save_game_data` gave 0): the last save date becomes today's (`Date::operator=` of `G_CURRENT_DATE`
/// into `G_LAST_AUTOSAVE_DATE`); then the disk full box (`G_GAME_IN_PROGRESS_FLAG`, set by the save) or the no memory
/// box (`G_GAME_SESSION_STARTED_FLAG`), both cleared, with code 2; otherwise the "could not be saved" box with the path
/// ("Unable To Save", titled `title_heading` formatted, or the game's title). Gives whether that last box was shown.
pub(crate) unsafe fn menubar_command_save_failed(path: usize, title_heading: Option<usize>) -> bool {
    let mut date = [0u32; 2];
    exe::date_operator_3d(G_LAST_AUTOSAVE_DATE, date.as_mut_ptr() as usize, G_CURRENT_DATE);
    if read_u32(G_GAME_IN_PROGRESS_FLAG) != 0 {
        write_u32(G_GAME_IN_PROGRESS_FLAG, 0);
        write_u32(G_GAME_SESSION_STARTED_FLAG, 0);
        tel35_cdecl1(0x826a60, 2);
        return false;
    }
    if read_u32(G_GAME_SESSION_STARTED_FLAG) != 0 {
        write_u32(G_GAME_IN_PROGRESS_FLAG, 0);
        write_u32(G_GAME_SESSION_STARTED_FLAG, 0);
        tel35_cdecl1(0x826ce0, 2);
        return false;
    }
    match title_heading {
        // the auto save: titled "Auto Save Game" over "Unable To Save"
        Some(title_format) => {
            let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
            let (grey, back) = (colour_word(G_GUI_COLOUR_PALE_GREY) as u32, colour_word(0xaebdcc) as u32);
            let m = text(MENUBAR_TEXT, 0xa5aa74, path, 0u32, grey, back, 0u32) as u32 as usize;
            let (mut h, mut tt) = ([0u8; 200], [0u8; 200]);
            let h = text(h.as_mut_ptr() as usize, 0x99081c, m, 0u32, grey, back, 0u32) as u32 as usize;
            let title = text(tt.as_mut_ptr() as usize, title_format, h, m, 0u32, grey, back, 0u32) as u32 as usize;
            menubar_command_box(title, h, m, 0, grey, back, 0);
        }
        None => menubar_command_ok_box(0xa5aa74, Some(path), 0x99081c),
    }
    true
}

/// Rule: the auto save of the team menu.
///
/// With the Config flag `MENUBAR_CONFIG_AUTOSAVE`, a local slot (`MENUBAR_SLOT_REMOTE` clear), no manager being created
/// and no flagged screen: with no save file yet (`G_GAME_HAS_SAVE_FILE_FLAG` clear) the file dialog asks for the name
/// ("Auto Save Game", answer `menubar_answer::AUTO_SAVE`) and the routine gives -4; otherwise, once the interval
/// (`MENUBAR_CONFIG_AUTOSAVE_DAYS`, i16) has passed since the last save (`Date::days_since` of `G_LAST_AUTOSAVE_DATE`),
/// it saves into the folder and name of the Config (`_makepath`, `save_game_data` mode 0x16) and redraws the whole screen;
/// a failure gives its box (`menubar_command_save_failed`) and -4.
pub(crate) unsafe fn menubar_command_autosave(t: &mut MenubarCommandTick) -> Option<i32> {
    let config = read_usize(G_CONFIG);
    if read_u32(config + MENUBAR_CONFIG_FLAGS) & MENUBAR_CONFIG_AUTOSAVE == 0
        || menubar_command_remote()
        || menubar_being_created()
        || !menubar_no_flagged_screens()
    {
        return None;
    }
    if read_u32(G_GAME_HAS_SAVE_FILE_FLAG) == 0 {
        if t.dry {
            return Some(t.give_up());
        }
        covered!("tel35_7 auto save asks the name");
        menubar_command_file_dialog_copy(0xa5aaa8, menubar_answer::AUTO_SAVE, 0);
        return Some(-4);
    }
    let interval = read_i16(config + MENUBAR_CONFIG_AUTOSAVE_DAYS) as i32;
    let days = exe::date_days_since(G_CURRENT_DATE, read_u32(G_LAST_AUTOSAVE_DATE), read_u32(G_LAST_AUTOSAVE_DATE + 4));
    if days < interval {
        return None;
    }
    if t.dry {
        return Some(t.give_up());
    }
    covered!("tel35_7 auto save");
    let mut path = [0u8; 260];
    let config = read_usize(G_CONFIG);
    exe::crt_makepath(path.as_mut_ptr(), 0usize, config + MENUBAR_CONFIG_SAVE_FOLDER, config + config::SAVE_NAME, MENUBAR_SAVE_EXTENSION);
    if exe::save_game_data(path.as_ptr(), 0x16) != 0 {
        exe::screenmanager_redraw_screen(SCREEN_MANAGER, 1, 0);
        return None;
    }
    menubar_command_save_failed(path.as_ptr() as usize, Some(0xa5aaa8));
    Some(-4)
}

/// Rule: the Continue button of the team menu follows the game.
///
/// During a match day with the human's match on (`G_MATCH_DAY`, its `match_day_fields::HUMAN_MATCH` with a
/// `day_match::STATE` other than 2 and 8 (finished)) and no paused match in the record: "Latest Scores" (id 0x418,
/// state 3). Otherwise, with no manager being created, a current screen without the flag or with an exit routine
/// (`get_current_screen_flag`, `current_screen_has_exit_fn`) and a team menu built with context 0: "Waiting For Players"
/// (style 0x1061) when the manager is done (`ScreenManager::is_manager_done`), else "Continue Game" (0x1021), both id
/// 0x3e8 and state 1; leaving the "Latest Scores" state that way, while not done, gives -11 instead. Otherwise the
/// greyed "Continue Game" with no id (state 2). Each change is made once (the record's `CONTINUE_STATE`).
pub(crate) unsafe fn menubar_command_continue_button(t: &mut MenubarCommandTick) -> Option<i32> {
    let state = read_u8(t.rec + menubar_record::CONTINUE_STATE);
    let day = read_usize(G_MATCH_DAY);
    let human_match = if day != 0 { read_usize(day + match_day_fields::HUMAN_MATCH) } else { 0 };
    let match_on = human_match != 0 && {
        let s = read_u8(human_match + day_match::STATE);
        s != 2 && s != 8
    };
    if match_on && read_u32(t.rec + menubar_record::PAUSED_MATCH) == 0 {
        if state != 3 {
            covered!("tel35_7 latest scores button");
            menubar_command_set_continue(t, 0xa5a728, None, 2, 0xc, 0x418, 3);
        }
        return None;
    }
    let screen_free = || {
        core::mem::transmute::<usize, MenubarThis0>(0x7fed30)(SCREEN_MANAGER) == 0
            || core::mem::transmute::<usize, MenubarThis0>(0x7fedd0)(SCREEN_MANAGER) != 0
    };
    if !menubar_being_created() && screen_free() && read_u32(t.rec + menubar_record::CONTEXT) == 0 {
        if state != 1 {
            if exe::screenmanager_is_manager_done(SCREEN_MANAGER, menubar_command_manager() as u32) != 0 {
                covered!("tel35_7 waiting for players button");
                menubar_command_set_continue(t, 0xa16d34, Some(0x1061), 2, 0xc, 0x3e8, 1);
            } else {
                if state == 3 {
                    return Some(-11);
                }
                covered!("tel35_7 continue button");
                menubar_command_set_continue(t, 0xa16d24, Some(0x1021), 2, 0xc, 0x3e8, 1);
            }
        }
    } else if state != 2 {
        covered!("tel35_7 continue button greyed");
        menubar_command_set_continue(t, 0xa16d24, None, 1, 0x2c, 0, 2);
    }
    None
}

/// Rule: the bar's header flashes between the date and the match line, once a second.
///
/// When the time (`utils_get_time_ms`, signed) reaches the record's `REFRESH_TIME`, the `UPDATING` phase flips and the
/// next time is one second later. Phase 1, during a match day being played (`G_MATCH_DAY_PLAYING_FLAG`) with matches
/// left (finished × 100 / matches, as a signed byte, below 100; no matches: 0): "Updating <percentage>%...". Otherwise,
/// in phase 1 or with the Config flag `MENUBAR_CONFIG_MATCH_HEADER`, the paused match "home v away" (short names with
/// their gender chars); otherwise (or with no paused match) the date (`Date::get_weekday_numeric_date_string` of
/// `G_CURRENT_DATE`, 0x7d0, the day slice). A text different from the record's `HEADER_TEXT` goes to the header widget,
/// with a redraw, and is kept.
pub(crate) unsafe fn menubar_command_header(t: &mut MenubarCommandTick) {
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let rec = t.rec;
    let at = read_i32(rec + menubar_record::REFRESH_TIME);
    t.mark_ms(at);
    if t.now_ms() < at {
        return;
    }
    covered!("tel35_7 header flips");
    let phase = 1u8.wrapping_sub(read_u8(rec + menubar_record::UPDATING));
    write_u8(rec + menubar_record::UPDATING, phase);
    write_u32(rec + menubar_record::REFRESH_TIME, (t.now_ms() as u32).wrapping_add(1000));
    let mut written = false;
    if phase == 1 {
        let day = read_usize(G_MATCH_DAY);
        if day != 0 {
            let matches = read_i32(day + match_day_fields::MATCH_COUNT);
            let percentage = || if matches == 0 { 0 } else { read_i32(day + match_day_fields::FINISHED_COUNT).wrapping_mul(100).wrapping_div(matches) as i8 };
            if (matches == 0 || percentage() < 100) && read_u32(G_MATCH_DAY_PLAYING_FLAG) != 0 {
                covered!("tel35_7 header updating");
                text(t.text, 0xa5a764, percentage() as i32);
                written = true;
            }
        }
    }
    if !written {
        let paused = read_usize(rec + menubar_record::PAUSED_MATCH);
        if (phase == 1 || read_u32(read_usize(G_CONFIG) + MENUBAR_CONFIG_FLAGS) & MENUBAR_CONFIG_MATCH_HEADER != 0) && paused != 0 {
            covered!("tel35_7 header match");
            let away = Club(read_usize(paused + day_match::AWAY_CLUB));
            let home = Club(read_usize(paused + day_match::HOME_CLUB));
            let a = (away.short_gender_name() as i8) as i32;
            let h = (home.short_gender_name() as i8) as i32;
            text(t.text, 0xa5a738, h, h, a, a, home.addr() + club::SHORT_NAME, away.addr() + club::SHORT_NAME);
        } else {
            let date: MenubarThis3 = core::mem::transmute(FN_DATE_GET_WEEKDAY_NUMERIC_DATE_STRING);
            date(G_CURRENT_DATE, t.text, 0x7d0, read_u8(G_CURRENT_DAY_SLICE) as usize);
        }
    }
    if !menubar_command_same_text(rec + menubar_record::HEADER_TEXT, t.text) {
        if !t.dry {
            exe::screenmanager_set_widget_text(SCREEN_MANAGER, read_u16(rec + menubar_record::HEADER_WIDGET) as u32, t.text);
        }
        menubar_command_redraw(t, 0);
        menubar_command_copy_text(rec + menubar_record::HEADER_TEXT, t.text);
    }
}

/// Rule: the status item at the bottom of the bar follows the news, the chat and the players waiting.
///
/// In an unlocked slot (`MENUBAR_SLOT_LOCKED`) with unread news (`News::count_unread`) outside the match menu: "You Have
/// News" in red (id 0x3e9, kind 1). Otherwise, with a chat message waiting for the manager (`MENUBAR_CHAT_PENDING`)
/// outside the match menu: "Chat Message" in red (0x421, kind 2). Otherwise, in an unlocked slot with players waiting
/// (`count_human_managers(1)`) outside the match menu: "Player Waiting" (two in the game) or "<waiting> <total>
/// Players" in yellow (0x3fc, kind 3, redone when a count changes). Otherwise the empty item (black, no id, kind 0).
/// Each change is made once (the record's `STATUS_KIND`); news and chat zero the counts.
pub(crate) unsafe fn menubar_command_status(t: &mut MenubarCommandTick) {
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let rec = t.rec;
    let kind = read_u8(rec + menubar_record::STATUS_KIND);
    let unlocked = || read_u16(menubar_slot(MENUBAR_SLOT_LOCKED)) == 0;
    if unlocked()
        && read_usize(G_NEWS) != 0
        && exe::news_count_unread(read_usize(G_NEWS), menubar_command_manager(), 0) > 0
        && read_u8(rec + menubar_record::MODE) != 3
    {
        if kind != 1 {
            covered!("tel35_7 status news");
            menubar_command_text(t, 0xa5a3e0);
            menubar_command_set_status(t, G_GUI_COLOUR_RED, t.text, 0x3e9, 1, 0, 0);
        }
        return;
    }
    let manager = menubar_command_manager();
    let chat = manager != 0 && {
        let k = read_u16(manager).wrapping_sub(read_u16(G_STAFF_COUNT)).wrapping_add(0x10) as i16;
        read_u32((MENUBAR_CHAT_PENDING as isize + k as isize * 4) as usize) != 0
    };
    if chat && read_u8(rec + menubar_record::MODE) != 3 {
        if kind != 2 {
            covered!("tel35_7 status chat");
            menubar_command_text(t, 0xa5a3d0);
            menubar_command_set_status(t, G_GUI_COLOUR_RED, t.text, 0x421, 2, 0, 0);
        }
        return;
    }
    if unlocked() {
        let waiting = menubar_count_humans(1);
        if waiting > 0 && read_u8(rec + menubar_record::MODE) != 3 {
            let total = menubar_count_humans(0);
            if kind == 3 && waiting == read_i32(rec + menubar_record::WAITING) && total == read_i32(rec + menubar_record::TOTAL) {
                return;
            }
            covered!("tel35_7 status players waiting");
            if total == 2 {
                menubar_command_text(t, 0xa5a3c0);
            } else {
                text(t.text, 0xa5a394, waiting, total);
            }
            menubar_command_set_status(t, G_GUI_COLOUR_YELLOW, t.text, 0x3fc, 3, waiting, total);
            return;
        }
    }
    if kind != 0 {
        covered!("tel35_7 status cleared");
        menubar_command_set_status(t, G_GUI_COLOUR_BLACK, MENUBAR_EMPTY, 0, 0, 0, 0);
    }
}

/// Rule: the game goes on (-5) unless space is short.
///
/// In a local slot with less than `MENUBAR_LOW_SPACE_MB` MB of free disk: the holiday ends (`set_on_holiday(0)`) and
/// the disk full box opens (code 3), -4. A remote slot goes on; a local one with enough free memory goes on; otherwise
/// the holiday ends and the no memory box opens (code 3), -4.
pub(crate) unsafe fn menubar_command_go_on(t: &mut MenubarCommandTick) -> i32 {
    if !menubar_command_remote() && exe::os_get_free_disk_space_mb(0) < MENUBAR_LOW_SPACE_MB {
        if t.dry {
            return t.give_up();
        }
        exe::human_manager_set_on_holiday(read_usize(G_HUMAN_MANAGER), 0, 0usize);
        tel35_cdecl1(0x826a60, 3);
        return -4;
    }
    if menubar_command_remote() || exe::os_get_free_memory_mb() >= MENUBAR_LOW_SPACE_MB {
        return -5;
    }
    if t.dry {
        return t.give_up();
    }
    exe::human_manager_set_on_holiday(read_usize(G_HUMAN_MANAGER), 0, 0usize);
    tel35_cdecl1(0x826ce0, 3);
    -4
}

/// Continue (the countdown's end and the 0x3e8 command): in an unlocked slot with urgent news
/// (`News::has_urgent_items(manager, 1)`) the news screen opens instead (-4); otherwise `menubar_command_go_on`.
pub(crate) unsafe fn menubar_command_continue(t: &mut MenubarCommandTick) -> i32 {
    if read_u16(menubar_slot(MENUBAR_SLOT_LOCKED)) == 0
        && core::mem::transmute::<usize, MenubarThis2>(0x787350)(read_usize(G_NEWS), menubar_command_manager(), 1) != 0
    {
        if t.dry {
            return t.give_up();
        }
        covered!("tel35_7 urgent news");
        exe::news_screens_open_news_screen(menubar_command_manager(), 0);
        return -4;
    }
    menubar_command_go_on(t)
}

/// The holiday starts: `set_on_holiday(1)`, the holiday stamp now, `ScreenManager::send_clear_click`.
pub(crate) unsafe fn menubar_command_start_holiday(on: u32) {
    exe::human_manager_set_on_holiday(read_usize(G_HUMAN_MANAGER), on, 0usize);
    exe::human_manager_set_holiday_stamp(read_usize(G_HUMAN_MANAGER), exe::utils_get_time_ms(), 0usize);
    core::mem::transmute::<usize, MenubarThis0>(0x800000)(SCREEN_MANAGER);
}

/// Before a restart or an exit the game is offered to be saved: a team menu record in a local slot, no manager being
/// created, no flagged screen, and the saved game not up to date.
pub(crate) unsafe fn menubar_command_save_first(rec: usize) -> bool {
    rec != 0
        && read_u8(rec + menubar_record::MODE) == 2
        && !menubar_command_remote()
        && !menubar_being_created()
        && menubar_no_flagged_screens()
        && !menubar_command_saved_up_to_date()
}

/// Ends the program (`game_fatal_cleanup`, the no-op, `exit(0)`).
pub(crate) unsafe fn menubar_command_exit_program() {
    exe::game_fatal_cleanup();
    exe::shared_noop_ret0_5a8f60();
    exe::crt_exit(0);
}

/// Rule: the commands of the team bar and of the manager's menu.
///
/// 0x3e8 Continue (`menubar_command_continue`); 0x418 the latest scores of the human's match competition
/// (`open_latest_scores_screen(manager, day_match::DAY_COMPETITION)`); 0x3e9 the news; 0x42d stops the countdown (the
/// slot's `MENUBAR_SLOT_CONTINUE_TIME` −1, -11); 0x42e the manager stats; 0x41e the job information; 0x425 and 0x433
/// the control questions of the reserve team (code `RESERVE_TEAM`) and of the national teams (`NATIONAL_TEAMS`);
/// 0x41d and 0x41c resign from the nation and the club (`open_resign_confirm_dialog` of the manager manager); 0x3f2
/// retire; 0x420 and 0x41f the board confidence of the nation and the club; 0x434 compare players
/// (`staff_screens_open_compare_players_screen(manager, 1, 1, 1, 0)` when the scout manager holds both players, else the
/// compare dialog); 0x3eb the player search (−1); 0x415 the transfers; 0x3ec the manager's profile (tab 5); 0x3ef go on
/// holiday; 0x3f0 return from holiday (the holiday ends; the manager's confirm, code `RETURN_FROM_HOLIDAY`); 0x414
/// send a message (the item's data). Each gives -4 unless said.
pub(crate) unsafe fn menubar_command_manager_menu(t: &mut MenubarCommandTick, id: u16, item: i16) -> Option<i32> {
    let manager = menubar_command_manager();
    let cdecl0 = |f: usize| core::mem::transmute::<usize, MenubarCdecl0>(f)();
    match id {
        0x3e8 => {
            covered!("tel35_7 continue");
            return Some(menubar_command_continue(t));
        }
        0x418 => {
            let day_match = read_usize(read_usize(G_MATCH_DAY) + match_day_fields::HUMAN_MATCH);
            exe::open_latest_scores_screen(manager, read_usize(day_match + day_match::DAY_COMPETITION));
        }
        0x42e => {
            cdecl0(0x6ae1a0);
        }
        0x41e => {
            cdecl0(0x6b0a10);
        }
        0x425 => {
            covered!("tel35_7 reserve team question");
            let controls = || exe::human_manager_controls_reserve_team(read_usize(G_HUMAN_MANAGER), 0usize) != 0;
            menubar_command_control_confirm(staff::CLUB_JOB, controls, 0xa5a8ec, 0xa5a8c0, menubar_answer::RESERVE_TEAM);
        }
        0x433 => {
            covered!("tel35_7 national teams question");
            let controls = || core::mem::transmute::<usize, MenubarThis2>(0x5fd6b0)(read_usize(G_HUMAN_MANAGER), 0, 0) != 0;
            menubar_command_control_confirm(staff::NATIONAL_JOB, controls, 0xa5a88c, 0xa5a864, menubar_answer::NATIONAL_TEAMS);
        }
        0x41d | 0x41c => {
            let job = read_usize(manager + if id == 0x41d { staff::NATIONAL_JOB } else { staff::CLUB_JOB });
            core::mem::transmute::<usize, MenubarThis1>(0x69b780)(read_usize(G_MANAGER_MANAGER), job);
        }
        0x3f2 => {
            cdecl0(0x6b28b0);
        }
        0x420 | 0x41f => {
            let job = read_usize(manager + if id == 0x420 { staff::NATIONAL_JOB } else { staff::CLUB_JOB });
            tel35_cdecl1(0x6b2080, job);
        }
        0x434 => {
            let scouts = read_usize(G_SCOUT_MANAGER);
            let first = core::mem::transmute::<usize, MenubarThis1>(0x7f7920)(scouts, 1);
            let second = core::mem::transmute::<usize, MenubarThis1>(0x7f7920)(read_usize(G_SCOUT_MANAGER), 2);
            if first != 0 && second != 0 {
                let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) = core::mem::transmute(FN_STAFF_SCREENS_OPEN_COMPARE_PLAYERS_SCREEN);
                f(manager, 1, 1, 1, 0);
            } else {
                tel35_cdecl1(0x88c440, manager);
            }
        }
        0x3e9 => {
            exe::news_screens_open_news_screen(manager, 0);
        }
        0x3eb => {
            tel35_cdecl1(0x808540, usize::MAX);
        }
        0x415 => {
            cdecl0(0x8f4fa0);
        }
        0x3ec => {
            let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) = core::mem::transmute(FN_STAFF_SCREENS_OPEN_STAFF_PROFILE);
            f(manager, 5, 1, 0, 0);
        }
        0x3ef => {
            cdecl0(0x6b2d60);
        }
        0x3f0 => {
            covered!("tel35_7 return from holiday question");
            menubar_command_person_confirm(manager, 0xa5a84c, menubar_answer::RETURN_FROM_HOLIDAY, || {
                exe::human_manager_set_on_holiday(read_usize(G_HUMAN_MANAGER), 0, 0usize);
            });
        }
        0x42d => {
            write_i32(menubar_slot(MENUBAR_SLOT_CONTINUE_TIME), -1);
            return Some(-11);
        }
        0x414 => {
            let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(FN_NEWS_SCREENS_OPEN_SEND_MESSAGE);
            f(tel26_item_value(item), 0);
        }
        _ => return None,
    }
    Some(-4)
}

/// Rule: the commands of the Competitions and Find menus.
///
/// 0x3f3 the FIFA rankings; 0x40c the UEFA coefficients; 0x3f4, 0x3f5, 0x3f6 and 0x3f9 the find screens of kinds 1, 2,
/// 3 and 6 on the item's data and the main nation (`key_nation_get_main_nation(0)`); 0x3f7 and 0x3f8 kinds 4 and 5 on
/// the main continent of the data (`key_nation_get_main_continent(data, 0)`) with the data; 0x3fa the find screen
/// (`open_find_screen`) of the data; 0x429 the written history. Each gives -4.
pub(crate) unsafe fn menubar_command_find_menus(id: u16, item: i16) -> Option<i32> {
    let cdecl0 = |f: usize| core::mem::transmute::<usize, MenubarCdecl0>(f)();
    let data = || tel26_item_value(item) as usize;
    match id {
        0x3f3 => {
            cdecl0(0x49c0b0);
        }
        0x40c => {
            cdecl0(0x49c700);
        }
        0x3f4 | 0x3f5 | 0x3f6 | 0x3f9 => {
            let kind = match id { 0x3f4 => 1, 0x3f5 => 2, 0x3f6 => 3, _ => 6 };
            let d = data();
            let nation = tel35_cdecl1(0x669600, 0);
            menubar_command_find(kind, d, nation);
        }
        0x3f7 | 0x3f8 => {
            let d = data();
            let f: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_KEY_NATION_GET_MAIN_CONTINENT);
            let continent = f(d, 0);
            menubar_command_find(if id == 0x3f7 { 4 } else { 5 }, continent, d);
        }
        0x3fa => {
            tel35_cdecl1(0x5a2c30, data());
        }
        0x429 => {
            tel35_cdecl1(0x5f2b80, 1);
        }
        _ => return None,
    }
    Some(-4)
}

/// Rule: 0x3fd switches to another human manager (the item's data).
///
/// A manager away or done (`ScreenManager::is_manager_away` / `is_manager_done`) is marked not done and gets his
/// screen: the news with no job, otherwise his club's screen (`open_manager_club_screen`). Then the switch
/// (`ScreenManager::switch_to_manager`, `send_clear_click`), the new slot's countdown time −1 and the old record's
/// `HOLIDAY_PENDING` 0. A human on holiday (`HUMAN_MANAGER::is_on_holiday`) is asked to return from it (the holiday
/// ends; the manager's confirm, code `RETURN_FROM_HOLIDAY`, -4); otherwise the remaining commands are tried.
pub(crate) unsafe fn menubar_command_switch_manager(item: i16, rec: usize) -> Option<i32> {
    let next = Staff(tel26_item_value(item) as usize);
    covered!("tel35_7 switch manager");
    if exe::screenmanager_is_manager_away(SCREEN_MANAGER, next) != 0 || exe::screenmanager_is_manager_done(SCREEN_MANAGER, next.addr() as u32) != 0 {
        core::mem::transmute::<usize, MenubarThis2>(0x7ffeb0)(SCREEN_MANAGER, next.addr(), 0);
        if next.club_job().addr() == 0 && next.national_job().addr() == 0 {
            exe::news_screens_open_news_screen(next, 0);
        } else {
            exe::open_manager_club_screen(next, 0, 0, 0, 0);
        }
    }
    core::mem::transmute::<usize, MenubarThis1>(0x7ffcd0)(SCREEN_MANAGER, next.addr());
    core::mem::transmute::<usize, MenubarThis0>(0x800000)(SCREEN_MANAGER);
    write_i32(menubar_slot(MENUBAR_SLOT_CONTINUE_TIME), -1);
    // a slot with no record writes at address 0xee, as the original
    write_u32(rec + menubar_record::HOLIDAY_PENDING, 0);
    if exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER), 0usize) == 0 {
        return None;
    }
    // ponytail: the original reads the two heading colours before the switch; they are constants, read here again
    menubar_command_person_confirm(next.addr(), 0xa5a84c, menubar_answer::RETURN_FROM_HOLIDAY, || {
        exe::human_manager_set_on_holiday(read_usize(G_HUMAN_MANAGER), 0, 0usize);
    });
    Some(-4)
}

/// Rule: the commands of the game menu.
///
/// 0x3fb adds a manager: in a slot with a manager a new record of the screen manager
/// (`ScreenManager::add_manager_record` with the network join and leave handlers, -11), else `setup_add_human_manager`;
/// 0x3fc the manager status; 0x421 the manager chat screen (its builder and handler, parameter 0 = 0); 0x431 a new game;
/// 0x42a the hall of fame; 0x42b the credits (clearing the current command 0x42b); 0x42c the web sites; 0x42f "Restart
/// the game ?" (the game's title, code `RESTART_GAME`); 0x402 with no flagged screen "Exit the game ?" ("Exit the
/// network game ?" in a remote slot; code `EXIT_GAME`). Each gives -4 unless said; any other id, 0.
pub(crate) unsafe fn menubar_command_game_menu(id: u16) -> i32 {
    let cdecl0 = |f: usize| core::mem::transmute::<usize, MenubarCdecl0>(f)();
    match id {
        0x3fb => {
            if menubar_command_manager() == 0 {
                cdecl0(0x820d40);
                return -4;
            }
            covered!("tel35_7 add manager record");
            core::mem::transmute::<usize, MenubarThis3>(0x7ff8f0)(SCREEN_MANAGER, 0x838a80, 0x838c00, 0);
            -11
        }
        0x3fc => {
            cdecl0(0x81fea0);
            -4
        }
        0x421 => {
            if exe::screenmanager_open_screen(SCREEN_MANAGER, 0x7654d0usize, 0x7657c0usize, 0, 0usize, 0) != 0 {
                exe::screenmanager_set_screen_param(SCREEN_MANAGER, 0, 0usize, 0);
            }
            -4
        }
        0x431 => {
            cdecl0(0x81ce00);
            -4
        }
        0x42a => {
            cdecl0(0x825cc0);
            -4
        }
        0x42b => {
            if read_u16(G_CURRENT_COMMAND_ID) == 0x42b {
                write_u16(G_CURRENT_COMMAND_ID, 0);
            }
            cdecl0(0x4e5920);
            -4
        }
        0x42c => {
            cdecl0(0x5134c0);
            -4
        }
        0x42f | 0x402 => {
            let code = if id == 0x42f {
                covered!("tel35_7 restart question");
                menubar_text(0xa5a838);
                menubar_answer::RESTART_GAME
            } else {
                if !menubar_no_flagged_screens() {
                    return 0;
                }
                covered!("tel35_7 exit question");
                menubar_text(if menubar_command_remote() { 0xa5a820 } else { 0xa5a810 });
                menubar_answer::EXIT_GAME
            };
            let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
            let back = menubar_command_colour(0);
            let fore = menubar_command_colour(1);
            let mut h = [0u8; 200];
            let h = text(h.as_mut_ptr() as usize, 0x9909b8, MENUBAR_TEXT, 1u32, fore, back, code) as u32 as usize;
            let title = menubar_command_game_title(h);
            menubar_command_box(title, h, MENUBAR_TEXT, 1, fore, back, code);
            -4
        }
        _ => 0,
    }
}

/// The menu area under a heading item.
pub(crate) unsafe fn menubar_submenu(item: usize) -> usize {
    menubar_area([0, 0, 0, 0, 0, 0, 0, 0, 0x30, colour_word(MENUBAR_MENU_COLOUR), item])
}

/// An entry of a menu: `flags` 1 enabled, 0x21 greyed out.
pub(crate) unsafe fn menubar_entry(text: usize, flags: usize, id: usize, data: usize, area: usize) -> usize {
    menubar_item([2, 0, 0, 0, 0, 0, 0, 0x10, colour_word(MENUBAR_MENU_COLOUR), colour_word(G_GUI_COLOUR_YELLOW), flags, 1,
        colour_word(G_GUI_COLOUR_BLACK), text, 0, id, data, area])
}

/// The entry of the text buffer: enabled with `id`, or greyed out with no id.
pub(crate) unsafe fn menubar_entry_if(enabled: bool, id: usize, area: usize) {
    if enabled {
        menubar_entry(MENUBAR_TEXT, 1, id, 0, area);
    } else {
        menubar_entry(MENUBAR_TEXT, 0x21, 0, 0, area);
    }
}

/// A separator line in a menu.
pub(crate) unsafe fn menubar_separator(area: usize) {
    menubar_item([1, 0, 0, 0, 0, 0, 0, 0x1000010, colour_word(MENUBAR_MENU_COLOUR), colour_word(G_GUI_COLOUR_YELLOW), 1, 1,
        colour_word(G_GUI_COLOUR_BLACK), 0, 0, 0, 0, area]);
}

/// `get_heading_colour(flag)` (0x839590): the pale grey (1) or the red (0) colour, zero-extended as the original's AX.
pub(crate) unsafe fn menubar_command_colour(flag: u32) -> u32 {
    exe::get_heading_colour(flag) as u32
}

/// `langlib_sprintf` with no arguments into the run's text buffer.
pub(crate) unsafe fn menubar_command_text(t: &MenubarCommandTick, format: usize) {
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    text(t.text, format);
}

/// The two strings are equal (the original's byte loop).
pub(crate) unsafe fn menubar_command_same_text(a: usize, b: usize) -> bool {
    let mut k = 0;
    loop {
        let c = read_u8(a + k);
        if c != read_u8(b + k) {
            return false;
        }
        if c == 0 {
            return true;
        }
        k += 1;
    }
}

/// `strcpy(dst, src)` (the original's rep movs of the length + 1).
pub(crate) unsafe fn menubar_command_copy_text(dst: usize, src: usize) {
    let mut k = 0;
    loop {
        let c = read_u8(src + k);
        write_u8(dst + k, c);
        if c == 0 {
            return;
        }
        k += 1;
    }
}

/// The Continue item (`menubar_record::CONTINUE_WIDGET`): the text `format`, the style (only for the active button),
/// the flags, the text flags and the id, a redraw, and the new state.
pub(crate) unsafe fn menubar_command_set_continue(t: &MenubarCommandTick, format: usize, style: Option<u32>, flags: u32, text_flags: u32, id: u32, state: u8) {
    menubar_command_text(t, format);
    if !t.dry {
        let w = || read_u16(t.rec + menubar_record::CONTINUE_WIDGET) as u32;
        if let Some(style) = style {
            exe::screenmanager_set_widget_style(SCREEN_MANAGER, w(), style);
        }
        exe::screenmanager_set_widget_flags(SCREEN_MANAGER, w() as usize, flags);
        exe::screenmanager_set_widget_text_flags(SCREEN_MANAGER, w(), text_flags);
        exe::screenmanager_set_widget_text(SCREEN_MANAGER, w(), t.text);
        exe::screenmanager_set_widget_id(SCREEN_MANAGER, w(), id);
    }
    menubar_command_redraw(t, 0);
    write_u8(t.rec + menubar_record::CONTINUE_STATE, state);
}

/// The status item at the bottom (`menubar_record::STATUS_WIDGET`): the colour, the text, the id, a redraw, and the kind
/// with the counts of players waiting and in the game.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn menubar_command_set_status(t: &MenubarCommandTick, colour: usize, text: usize, id: u32, kind: u8, waiting: i32, total: i32) {
    if !t.dry {
        let w = || read_u16(t.rec + menubar_record::STATUS_WIDGET) as u32;
        exe::screenmanager_set_widget_text_colour(SCREEN_MANAGER, w(), colour_word(colour) as u32);
        exe::screenmanager_set_widget_text(SCREEN_MANAGER, w(), text);
        exe::screenmanager_set_widget_id(SCREEN_MANAGER, w(), id);
    }
    menubar_command_redraw(t, 0);
    write_u8(t.rec + menubar_record::STATUS_KIND, kind);
    write_i32(t.rec + menubar_record::WAITING, waiting);
    write_i32(t.rec + menubar_record::TOTAL, total);
}

/// `confirm_push(title, heading, message, buttons, fore, back, code)` (0x591840, cdecl): the confirm box (`buttons` 1 =
/// No and Yes, 0 = Ok); its answer comes back with `code` in `G_PENDING_COMMAND_CODE`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn menubar_command_box(title: usize, heading: usize, message: usize, buttons: u32, fore: u32, back: u32, code: u32) {
    exe::confirm_push(title, heading, message, buttons, fore, back, code);
}

/// "Please Confirm" (No / Yes) over the question in the text buffer, titled `title`, in the colours `fore` and `back`,
/// answer `code`. The heading is formatted with the box's arguments, as the original pushes them once for both calls.
pub(crate) unsafe fn menubar_command_please_confirm(title: usize, message: usize, fore: u32, back: u32, code: u32) {
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut heading = [0u8; 200];
    let h = text(heading.as_mut_ptr() as usize, 0x9909b8, message, 1u32, fore, back, code) as u32 as usize;
    menubar_command_box(title, h, message, 1, fore, back, code);
}

/// The game's title (`get_game_title` 0x839570, "Championship Manager 2001/02"), the title of the boxes with no person.
pub(crate) unsafe fn menubar_command_game_title(heading: usize) -> usize {
    core::mem::transmute::<usize, MenubarCdecl1>(0x839570)(heading)
}

/// The kit colours of a club (or nation) record, `database_get_club_kit_colours(club, 1, ..)`: the routine writes the
/// two low words, the high ones stay.
pub(crate) unsafe fn menubar_command_kit_colours(club: usize, fore: &mut u32, back: &mut u32) {
    exe::database_get_club_kit_colours(club, 1, fore, back);
}

/// The control question of the club or the nation (0x425, 0x433): the manager's name as the title, the question
/// (`senior` when he already controls the second team, else `all`) with the team's name, the team's kit colours
/// (no defaults: the routine writes the low words), answer `code`.
pub(crate) unsafe fn menubar_command_control_confirm(team_field: usize, controls: impl FnOnce() -> bool, senior: usize, all: usize, code: u32) {
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let manager = menubar_command_manager();
    let mut name = [0u8; 200];
    exe::database_format_staff_name(manager, name.as_mut_ptr() as usize, 200, 1);
    let controls = controls();
    let team = Club(read_usize(manager + team_field));
    // a nation record has its short name and gender char at the club's offsets
    let g = (team.short_gender_name() as i8) as i32;
    text(MENUBAR_TEXT, if controls { senior } else { all }, g, g, team.addr() + club::SHORT_NAME);
    let (mut fore, mut back) = (0u32, 0u32);
    menubar_command_kit_colours(team.addr(), &mut fore, &mut back);
    menubar_command_please_confirm(name.as_ptr() as usize, MENUBAR_TEXT, fore, back, code);
}

/// The saved game is up to date: a save exists (`G_GAME_HAS_SAVE_FILE_FLAG`), its date (`G_LAST_AUTOSAVE_DATE`: the
/// year unsigned, then the day signed) is not before today and nothing changed since (`G_GAME_UNSAVED_CHANGES_FLAG`).
pub(crate) unsafe fn menubar_command_saved_up_to_date() -> bool {
    if read_u32(G_GAME_HAS_SAVE_FILE_FLAG) == 0 {
        return false;
    }
    let (year, today) = (read_u16(G_LAST_AUTOSAVE_DATE + 2), read_u16(G_CURRENT_DATE_YEAR));
    if year < today || (year == today && read_i16(G_LAST_AUTOSAVE_DATE) < read_i16(G_CURRENT_DATE)) {
        return false;
    }
    read_u32(G_GAME_UNSAVED_CHANGES_FLAG) == 0
}

/// `find_screens` openers of the Find menu (`open_choose_friendly_teams_screen` 0x5a0650, cdecl): the kind, the data
/// and the third argument, with the fourth slot 0 (left on the stack by the key nation call).
pub(crate) unsafe fn menubar_command_find(kind: u32, a: usize, b: usize) {
    exe::open_choose_friendly_teams_screen(kind, a, b, 0);
}

/// The empty string.
pub(crate) const MENUBAR_EMPTY: usize = 0xad9c64;

/// The menu entries' background colour (u16).
pub(crate) const MENUBAR_MENU_COLOUR: usize = 0xae31aa;

/// The highlighted heading colour (u16).
pub(crate) const MENUBAR_HEADING_COLOUR: usize = 0xaebde8;

/// time() of the last menu bar build.
pub(crate) const MENUBAR_BUILD_TIME: usize = 0xb74310;

/// The menu handler and its two command ids, written on every build [inference].
pub(crate) const MENUBAR_HANDLER: usize = 0xb77c3e;

/// Nonzero in a network game (screen manager + 0x3840) [inference].
pub(crate) const MENUBAR_NETWORK_GAME: usize = 0xb77c90;

/// One dword per human manager: a chat message is waiting [inference].
pub(crate) const MENUBAR_CHAT_PENDING: usize = 0xb6b7d0;

pub(crate) const MENUBAR_SLOT_LOCKED: usize = 0x7e6;

pub(crate) const MENUBAR_SLOT_CONTINUE_TIME: usize = 0x800;

pub(crate) type MenubarSprintf = unsafe extern "C" fn(usize, usize, ...) -> i32;

pub(crate) type MenubarThis0 = unsafe extern "thiscall" fn(usize) -> u32;

pub(crate) type MenubarThis1 = unsafe extern "thiscall" fn(usize, usize) -> u32;

pub(crate) type MenubarThis2 = unsafe extern "thiscall" fn(usize, usize, usize) -> u32;

pub(crate) type MenubarThis3 = unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32;

pub(crate) type MenubarThis4 = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize) -> u32;

pub(crate) type MenubarCdecl0 = unsafe extern "C" fn() -> usize;

pub(crate) type MenubarCdecl1 = unsafe extern "C" fn(usize) -> usize;

pub(crate) type MenubarCdecl3 = unsafe extern "C" fn(usize, usize, usize) -> usize;

/// The manager chat log: one 0x7b8-byte page per human (16), 19 lines of 0x68 bytes (the text, then at +0x66 the
/// sender's human index, -1 for none); the line counts (words), the "page changed" flags (dwords) and the "set up" flag.
pub(crate) const CHAT_PAGES: usize = 0xb6b810;

pub(crate) const CHAT_PAGE_SIZE: usize = 0x7b8;

pub(crate) const CHAT_LINE_SIZE: usize = 0x68;

pub(crate) const CHAT_LINE_SENDER: usize = 0x66;

pub(crate) const CHAT_LINES: usize = 0x13;

pub(crate) const CHAT_LINE_COUNTS: usize = 0xb742f0;

pub(crate) const CHAT_CHANGED: usize = 0xb6b7d0;

pub(crate) const CHAT_SET_UP: usize = 0xb74314;

/// The colour word at 0xaebde8 (no name yet): the empty chat lines and the text entry.
pub(crate) const CHAT_COLOUR_BDE8: usize = 0xaebde8;

/// The colour word at 0xae31aa (no name yet): the menu entries' first colour.
pub(crate) const MENU_ENTRY_COLOUR: usize = 0xae31aa;

/// The size of a club competition record (`G_CLUB_COMP_TABLE`) and of a cup round of a stage (+0xa3, the round id
/// word at +7).
pub(crate) const LINK_CLUB_COMP_SIZE: u32 = 0x6b;

pub(crate) const LINK_ROUND_SIZE: isize = 0x68;

/// The staff profile links read through the display's widget readers: (id, profile tab, kind).
pub(crate) const LINK_PROFILES: [(u16, u32, u32); 8] = [
    (0x7da, 3, 1), (0x7d9, 2, 1), (0x7db, 4, 1), (0x7dc, 1, 2), (0x7dd, 2, 2), (0x7df, 3, 2), (0x7de, 2, 2), (0x7e0, 4, 2),
];

/// The slot's u16 that is nonzero for a remote network player (screen manager +0x7d4 + 0x300 x slot) [inference: it
/// turns "Exit the game ?" into "Exit the network game ?", skips the auto save and the disk check, gives -5 and -12].
pub(crate) const MENUBAR_SLOT_REMOTE: usize = 0x7d4;

/// The Config fields read here: +4 the option flags, +0xd the auto save interval in days (i16), +0xf the save folder.
pub(crate) const MENUBAR_CONFIG_FLAGS: usize = 4;

pub(crate) const MENUBAR_CONFIG_SAVE_FOLDER: usize = 0xf;

/// Config flag: save automatically every `MENUBAR_CONFIG_AUTOSAVE_DAYS` days.
pub(crate) const MENUBAR_CONFIG_AUTOSAVE: u32 = 0x2000;

/// Config flag: the bar's header shows the paused match instead of the date on both phases of the flashing.
pub(crate) const MENUBAR_CONFIG_MATCH_HEADER: u32 = 0x10000;

/// The save name buffer the file dialog edits and the routine that checks the typed name (`check_saved_game_file`).
pub(crate) const MENUBAR_SAVE_NAME: usize = 0xdd6ed4;

pub(crate) const MENUBAR_SAVE_NAME_CHECK: usize = 0x838e10;

/// The ".sav" extension.
pub(crate) const MENUBAR_SAVE_EXTENSION: usize = 0xa5aab8;

/// Below this many free MB of disk (or memory) the game stops with the disk full (no memory) box.
pub(crate) const MENUBAR_LOW_SPACE_MB: i32 = 0x19;

/// One run of the menu bar's periodic work. The port runs it on the slot's real record and the text buffer
/// (`MENUBAR_TEXT`). The comparer runs it `dry` before the original: on a copy of the record and a text buffer of its
/// own, with no call that changes the game or the screen (the widgets, the redraw, the saves, the boxes); an outcome that
/// needs one of them sets `unsure`. It also notes the clock thresholds the decisions read (`ms_marks`, `seconds`): the
/// prediction holds only if the original's run did not cross them.
pub(crate) struct MenubarCommandTick {
    pub(crate) dry: bool,
    pub(crate) rec: usize,
    pub(crate) text: usize,
    pub(crate) unsure: bool,
    pub(crate) ms_marks: [i32; 2],
    pub(crate) marks: usize,
    pub(crate) seconds: bool,
}

impl MenubarCommandTick {
    /// `utils_get_time_ms`; dry, in the simulation, the menu bar's clock read without advancing it
    /// (`simulation::clock_peek`).
    pub(crate) unsafe fn now_ms(&self) -> i32 {
        if self.dry {
            if let Some(v) = crate::simulation::clock_peek(0x7615d0) {
                return v;
            }
        }
        exe::utils_get_time_ms() as i32
    }

    /// A threshold of `utils_get_time_ms` the run compared with.
    pub(crate) fn mark_ms(&mut self, at: i32) {
        if self.marks < self.ms_marks.len() {
            self.ms_marks[self.marks] = at;
            self.marks += 1;
        }
    }

    /// Dry: the outcome needs running; returns the value that stops the dry run.
    pub(crate) fn give_up(&mut self) -> i32 {
        self.unsure = true;
        0
    }
}

/// The text buffer every menu text is written into.
pub(crate) const MENUBAR_TEXT: usize = 0xde1f64;

pub(crate) const MENUBAR_CONFIG_AUTOSAVE_DAYS: usize = 0xd;
