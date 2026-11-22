// ---------- batch %%LEVA%%: the club screens (5a): the squad numbers and European squad screen's events, leaving it, and opening the part exchange screen (club_screens.cpp) ----------

/// club_screens.cpp's source path (0x98e598), for the handler's error boxes (`show_error_at`).
const SQUAD_NUMBERS_SOURCE_FILE: usize = 0x98e598;

/// The 50 player slots of the squad copy kept in parameter 0x11 of the squad numbers screen.
const SQUAD_NUMBERS_SLOTS: usize = 0x32;

/// A player of the squad copy: present and made by the database (his id below `G_STAFF_COUNT`, not a regen).
unsafe fn squad_numbers_in_database(p: usize) -> bool {
    p != 0 && read_i32(p) < read_i32(G_STAFF_COUNT)
}

/// The squad number `p` holds at `club` (`exe::squad_manager_get_squad_number`, signed: negative = provisional).
unsafe fn squad_numbers_number(p: usize, club: usize) -> i8 {
    exe::squad_manager_get_squad_number(read_usize(G_SQUAD_MANAGER), p, club)
}

/// Gives `p` the squad number `n` at `club` (`exe::squad_manager_set_squad_number`; the callee reads the byte only).
unsafe fn squad_numbers_set(p: usize, club: usize, n: i8) {
    exe::squad_manager_set_squad_number(read_usize(G_SQUAD_MANAGER), p, club, n as u8 as u32);
}

/// The first player of the squad copy (`squad`, 50 slots, only database players) whose number, as an absolute value,
/// is `n`; 0 when none.
unsafe fn squad_numbers_holder(squad: usize, club: usize, n: i32) -> usize {
    for k in 0..SQUAD_NUMBERS_SLOTS {
        let p = read_usize(squad + 4 * k);
        if squad_numbers_in_database(p) && (squad_numbers_number(p, club) as i32).wrapping_abs() == n {
            return p;
        }
    }
    0
}

/// The club's squad copied into a 200-byte block (`malloc`, `exe::database_collect_club_squad`); 0 and the error box
/// of club_screens.cpp at `line` when the allocation fails.
unsafe fn squad_numbers_collect(club: usize, line: u32) -> usize {
    let block = exe::crt_malloc(200);
    if block == 0 {
        show_error_at(SQUAD_NUMBERS_SOURCE_FILE, line);
        return 0;
    }
    exe::database_collect_club_squad(club, block as *mut usize, 0);
    block
}

/// The "Please Confirm" / "Incomplete" box of the squad numbers screen: the heading `heading` and the title `title`
/// formatted (langlib) into two 200-byte buffers with the original's arguments (the text, the buttons, the club's kit
/// colours and the result code), then `confirm_push` (0x591840) with the formatter's answers.
unsafe fn squad_numbers_confirm(title: usize, heading: usize, text: usize, buttons: u32, colours: (u32, u32), code: u32) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let confirm: unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x591840usize);
    let (mut heading_buf, mut title_buf) = ([0u8; 0xc8], [0u8; 0xc8]);
    let h = lang(heading_buf.as_mut_ptr() as usize, heading, text, buttons, colours.0, colours.1, code);
    let t = lang(title_buf.as_mut_ptr() as usize, title, h, text, buttons, colours.0, colours.1, code);
    confirm(t, h, text, buttons, colours.0, colours.1, code);
}

/// The club's first kit colours (`exe::database_get_club_kit_colours`, kit 1) into two locals (the original's are
/// uninitialised stack slots; here zero).
unsafe fn squad_numbers_kit_colours(club: usize) -> (u32, u32) {
    let (mut a, mut b) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut a, &mut b);
    (a, b)
}

/// Rule: the squad numbers screen (and its European squad mode) answers its buttons: the views, sorts and filters are
/// written back to the screen parameters; the numbers are given, swapped, freed or worked out automatically; the
/// European registrations are ticked or worked out automatically; the submission asks for confirmation, or says what
/// is missing; the answers of the confirmation boxes apply the squad numbers or the European squad and acknowledge the
/// news item.
///
/// `squad_numbers_screen_handle_event(widget)`. Parameters: 0 the human manager, 1 the news item id, 2 the mode (1 the
/// squad numbers, otherwise the European squad), 3 the next season flag, 4 the view, 5 the sort, 6 the statistics
/// category, 9 and 0xa the attribute and the other column chosen for the sort, 0xb, 0xd the European registration
/// rule bytes, 0xe and 0xf the counts of registered and reserve players, 0x10 the squad size asked, 0x11 the squad copy
/// (50 player slots), 0x12 the fixture (squad numbers for a match). A local news item (`NewsItem::NewsItem(0, 0)`,
/// never destroyed, as in the original). The club is the manager's `staff::CLUB_JOB`.
///
/// On a network screen ([0xb77c84]) the confirmation answers come first: pending command 0x16 (both pending words
/// cleared) answered 2 (yes): without a fixture, the news item (`News::find_item_by_id`, the "Error" box at line 0x4057
/// and 0 when missing) gets slot 1 (type 0x18) or slot 7 (type 0xfa8) set to 1 (`News::set_item_slot`); then, for
/// every database player of the club's squad (a 200-byte copy, the box at line 0x4066 when it cannot be allocated) with a
/// positive provisional number, the number is made final (negated); with a fixture, pending command 0x43; -9. Pending
/// 0x17 (cleared) answered 2: slot 3 set to 1 and slot 6 to the registered count (param 0xe), then the European
/// registrations made final (1 -> 2, 4 -> 0; the box at line 0x409a); -9. Another answer clears the pending argument.
///
/// Then, for a widget other than -1, the list's scroll row (`ScreenManager::get_area_scroll_row` of the area in
/// param 8) is kept in param 7. By the widget's id (0 and -1: the default command id [0xdd6bde]): 1 a view (1, 2, 4,
/// 8, 0x10, 0x20 and 0x1000 replace the view and keep the attribute group, 0x40..0x400 replace the attribute group;
/// one already shown falls through to the menu bar); 2 a statistics category (param 6) other than the current one; 3
/// the automatic button: squad numbers with a fixture, every player selected for the match (`Tactics::find_selection_slot`
/// on the club's side, inside the eleven plus the fixture's substitutes, byte +0x4d & 0xf) without a negative number
/// gets one (`SQUAD_MANAGER::allocate_squad_number`); without a fixture, the whole club numbered
/// (`SQUAD_MANAGER::assign_club_squad_numbers`, next season) or one free number allocated (no player); European squad,
/// the registrations cleared (`staff::SQUAD_SELECTED_FOR` 1 -> 0) and chosen again
/// (`SQUAD_MANAGER::select_euro_registered_players` with param 0xb), and the counts of 1s and 4s written to params 0xe and
/// 0xf (once per player found, as the original); 4 the clear button: the numbers freed (a positive number back to 0) or
/// the registrations undone (1 -> 0, 4 -> 2, the counts to 0); 6 a sort (param 5, bit 0x1000000 reversing an already
/// chosen sort); 7 and 8 the attribute and the other column sorts (bits 0x400 and 0x800, params 9 and 0xa); 5 a number
/// dragged: onto another number, the two holders swap (the same number: its holder freed); onto a player (id 0x7d7 or
/// 9; the scan walks the club's own squad slots, `club::SQUAD`), the player takes the number and its old holder is
/// freed, unless either holds a provisional (negative) number; 9 a player given the first free number from 1 (up to
/// 50); 0xa a player's European registration cycled (none -> 1 registered, 1 -> none, 2 -> 4 reserve, 4 -> 2, with the
/// counts); 0xb leaving the European squad mode 2 undoes the registrations, -9; 0xc the submission: squad numbers, with
/// a fixture each selected player (`Tactics::get_selected_player` up to the eleven plus substitutes) needs a number,
/// otherwise "Incomplete" (code 0); otherwise "Please Confirm" with the right question (new numbers for a match, the
/// forthcoming season, the current season) and code 0x16; European squad, fewer registered (0xe - 0xf + 0xd) than the
/// squad size asked (at most 20) with rule byte 0xb at 1 or byte 0xd at 0: "Incomplete" (not all registered, or fewer
/// than 20), else "Please Confirm" with code 0x17; -4; 0xd the transfer status screen of the item's person, -4; any other
/// id goes to the menu bar (`exe::menubar_handle_command`, then `exe::menubar_handle_object_link`). The redraws return
/// -0xb.
/// Original: `squad_numbers_screen_handle_event` (0x47a9e0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn squad_numbers_screen_handle_event(widget: u32) -> i32 {
    covered!("tel05_1 squad numbers events");
    let mut news_item = [0u32; 0x38]; // the local NewsItem (0xe0 bytes; the original never destroys it)
    let item = news_item.as_mut_ptr() as usize;
    exe::new_newsitem(item, 0, 0);
    let manager = get_screen_param(0);
    let club = read_usize(manager + staff::CLUB_JOB);
    let news_id = get_screen_param(1) as u32;
    let mode = get_screen_param(2) as u8;
    let next_season = get_screen_param(3) as u32;
    let view = get_screen_param(4) as u32;
    let sort = get_screen_param(5) as u32;
    let category = get_screen_param(6) as u8;
    let attribute_sort = get_screen_param(9) as u8;
    let other_sort = get_screen_param(0xa) as u8;
    let rule_b = get_screen_param(0xb) as u8;
    let rule_d = get_screen_param(0xd) as u8;
    let registered = get_screen_param(0xe) as u8 as i8;
    let reserves = get_screen_param(0xf) as u8 as i8;
    let squad_size = get_screen_param(0x10) as u8 as i8;
    let squad = get_screen_param(0x11);
    let fixture = get_screen_param(0x12);
    let news = || read_usize(G_NEWS);
    let find_item: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(0x786b70usize);
    let set_slot: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize) = core::mem::transmute(FN_NEWS_SET_ITEM_SLOT);
    if read_u32(G_SCREEN_MANAGER + 0x3834) != 0 {
        // the confirmation boxes' answers (pending command and argument)
        let code = read_u32(G_PENDING_COMMAND_CODE);
        if code == 0x16 {
            let answer = read_u32(G_PENDING_COMMAND_ARG);
            write_u32(G_PENDING_COMMAND_CODE, 0);
            write_u32(G_PENDING_COMMAND_ARG, 0);
            if answer == 2 {
                covered!("tel05_1 squad numbers confirmed");
                if fixture == 0 {
                    if find_item(news(), get_screen_param(0), news_id, item) == 0 {
                        show_error_at(SQUAD_NUMBERS_SOURCE_FILE, 0x4057);
                        return 0;
                    }
                    let kind = read_u32(item);
                    if kind == 0x18 || kind == 0xfa8 {
                        set_slot(news(), get_screen_param(0), news_id, if kind == 0x18 { 1 } else { 7 }, 1, 0);
                    }
                }
                let block = squad_numbers_collect(club, 0x4066);
                if block == 0 {
                    return -9;
                }
                for k in 0..SQUAD_NUMBERS_SLOTS {
                    let p = read_usize(block + 4 * k);
                    if squad_numbers_in_database(p) {
                        let n = squad_numbers_number(p, club);
                        if n > 0 {
                            squad_numbers_set(p, club, n.wrapping_neg());
                        }
                    }
                }
                exe::crt_free(block);
                if fixture != 0 {
                    write_u32(G_PENDING_COMMAND_CODE, 0x43);
                }
                return -9;
            }
        } else if code == 0x17 {
            write_u32(G_PENDING_COMMAND_CODE, 0);
            if read_u32(G_PENDING_COMMAND_ARG) == 2 {
                covered!("tel05_1 european squad confirmed");
                write_u32(G_PENDING_COMMAND_ARG, 0);
                set_slot(news(), get_screen_param(0), news_id, 3, 1, 0);
                set_slot(news(), get_screen_param(0), news_id, 6, registered as i32 as u32, 0);
                let block = squad_numbers_collect(club, 0x409a);
                if block == 0 {
                    return -9;
                }
                for k in 0..SQUAD_NUMBERS_SLOTS {
                    let p = read_usize(block + 4 * k);
                    if squad_numbers_in_database(p) {
                        match read_u8(p + staff::SQUAD_SELECTED_FOR) {
                            1 => write_u8(p + staff::SQUAD_SELECTED_FOR, 2),
                            4 => write_u8(p + staff::SQUAD_SELECTED_FOR, 0),
                            _ => {}
                        }
                    }
                }
                exe::crt_free(block);
                return -9;
            }
            write_u32(G_PENDING_COMMAND_ARG, 0);
        }
    }
    let w = widget as u16 as i16;
    if w != -1 {
        let row = exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(8));
        set_screen_param(7, row);
    }
    let id = item_id_or_default(w);
    let value = tel26_item_value(w);
    if id == 1 && view & value == 0 {
        let mask = match value {
            1 | 2 | 4 | 8 | 0x10 | 0x20 | 0x1000 => 0xffff_efc0,
            0x40 | 0x80 | 0x100 | 0x200 | 0x400 => 0xffff_f83f,
            _ => return -0xb,
        };
        set_screen_param(4, ((view & mask) | value) as usize);
        return -0xb;
    }
    if id == 2 && value as u8 != category {
        set_screen_param(6, value as usize);
        return -0xb;
    }
    if id == 3 {
        covered!("tel05_1 automatic numbers");
        if mode == 1 {
            if fixture != 0 {
                let away = club != read_usize(fixture + fixture::HOME_CLUB);
                let side = if away { fixture::AWAY_SIDE_MARK } else { fixture::HOME_SIDE_MARK };
                let flag = (read_u8(fixture + side) == 0) as u32;
                for k in 0..SQUAD_NUMBERS_SLOTS {
                    let p = read_usize(squad + 4 * k);
                    if !squad_numbers_in_database(p) {
                        continue;
                    }
                    let slot = exe::tactics_find_selection_slot(read_usize(G_TACTICS), club, p, flag) as i8;
                    // the fixture's substitutes: byte +0x4d & 0xf
                    if slot >= 0 && (slot as i32) < (read_u8(fixture + 0x4d) & 0xf) as i32 + 0xb && squad_numbers_number(p, club) >= 0 {
                        exe::squad_manager_allocate_squad_number(read_usize(G_SQUAD_MANAGER), club, p);
                    }
                }
            } else if next_season != 0 {
                exe::squad_manager_assign_club_squad_numbers(read_usize(G_SQUAD_MANAGER), club, 1);
            } else {
                exe::squad_manager_allocate_squad_number(read_usize(G_SQUAD_MANAGER), club, 0usize);
            }
            return -0xb;
        }
        for k in 0..SQUAD_NUMBERS_SLOTS {
            let p = read_usize(squad + 4 * k);
            if p != 0 && read_u8(p + staff::SQUAD_SELECTED_FOR) == 1 {
                write_u8(p + staff::SQUAD_SELECTED_FOR, 0);
            }
        }
        // simplification: param 0xb is a byte in a stack slot whose upper bytes are garbage in the original (a char)
        exe::squad_manager_select_euro_registered_players(read_usize(G_SQUAD_MANAGER), club, rule_b as u32, 1);
        for wanted in [(1u8, 0xeu32), (4, 0xf)] {
            let mut count = 0i8;
            for k in 0..SQUAD_NUMBERS_SLOTS {
                let p = read_usize(squad + 4 * k);
                if squad_numbers_in_database(p) && read_u8(p + staff::SQUAD_SELECTED_FOR) == wanted.0 {
                    count = count.wrapping_add(1);
                    set_screen_param(wanted.1, count as i32 as usize);
                }
            }
        }
        return -0xb;
    }
    if id == 4 {
        if mode == 1 {
            for k in 0..SQUAD_NUMBERS_SLOTS {
                let p = read_usize(squad + 4 * k);
                if squad_numbers_in_database(p) && squad_numbers_number(p, club) > 0 {
                    squad_numbers_set(p, club, 0);
                }
            }
            return -0xb;
        }
        for k in 0..SQUAD_NUMBERS_SLOTS {
            let p = read_usize(squad + 4 * k);
            if squad_numbers_in_database(p) {
                match read_u8(p + staff::SQUAD_SELECTED_FOR) {
                    1 => write_u8(p + staff::SQUAD_SELECTED_FOR, 0),
                    4 => write_u8(p + staff::SQUAD_SELECTED_FOR, 2),
                    _ => {}
                }
            }
        }
        set_screen_param(0xe, 0);
        set_screen_param(0xf, 0);
        return -0xb;
    }
    if id == 6 {
        let reversed = if sort & value != 0 && sort & 0x100_0000 == 0 { sort | 0x100_0000 } else { sort & 0xfeff_ffff };
        set_screen_param(5, (value | (reversed & 0xc100_0000)) as usize);
        return -0xb;
    }
    if id == 7 || id == 8 {
        let (bit, chosen, param) = if id == 7 { (0x400u32, attribute_sort, 9u32) } else { (0x800, other_sort, 0xa) };
        let reversed = if sort & bit != 0 && value as u8 == chosen && sort & 0x100_0000 == 0 { sort | 0x100_0000 } else { sort & 0xfeff_ffff };
        set_screen_param(5, ((reversed & (0xc100_0000 | bit)) | bit) as usize);
        set_screen_param(param, value as usize);
        return -0xb;
    }
    let d = current_human_display();
    if id == 5 {
        let target = if w == -1 { -1 } else { read_i16(display_item_at(d, w) + display_item::DROP_TARGET) };
        if target != -1 {
            let target_id = item_id_or_default(target);
            let target_value = tel26_item_value(target);
            if target_id == 5 {
                covered!("tel05_1 number dragged onto a number");
                if value == target_value {
                    let holder = squad_numbers_holder(squad, club, value as u8 as i8 as i32);
                    if holder != 0 {
                        squad_numbers_set(holder, club, 0);
                    }
                    return -0xb;
                }
                let first = squad_numbers_holder(squad, club, value as u8 as i8 as i32);
                let second = squad_numbers_holder(squad, club, target_value as u8 as i8 as i32);
                if first != 0 {
                    squad_numbers_set(first, club, target_value as u8 as i8);
                }
                if second != 0 {
                    squad_numbers_set(second, club, value as u8 as i8);
                }
                return -0xb;
            }
            if target_id == 0x7d7 || target_id == 9 {
                covered!("tel05_1 number dragged onto a player");
                let n = value as u8 as i8;
                let player = target_value as usize;
                let mut holder = 0usize;
                for k in 0..SQUAD_NUMBERS_SLOTS {
                    if read_u32(club + club::SQUAD + 4 * k) == 0 {
                        continue;
                    }
                    let p = read_usize(squad + 4 * k);
                    // a missing player is asked too (number of a null person) and, matching, keeps the scan going, as
                    // the original
                    if (p == 0 || read_i32(p) < read_i32(G_STAFF_COUNT)) && (squad_numbers_number(p, club) as i32).wrapping_abs() == n as i32 {
                        holder = p;
                        if holder != 0 {
                            break;
                        }
                    }
                }
                if player != 0 && squad_numbers_number(player, club) < 0 {
                    return -0xb;
                }
                if holder != 0 && squad_numbers_number(holder, club) < 0 {
                    return -0xb;
                }
                if player != 0 {
                    squad_numbers_set(player, club, n);
                }
                if holder != 0 {
                    squad_numbers_set(holder, club, 0);
                }
                return -0xb;
            }
        }
    }
    if id == 9 {
        covered!("tel05_1 first free number");
        let mut n = 0i8;
        loop {
            if squad_numbers_holder(squad, club, n as i32 + 1) == 0 {
                squad_numbers_set(value as usize, club, n.wrapping_add(1));
                return -0xb;
            }
            n += 1;
            if n >= 0x32 {
                return -0xb;
            }
        }
    }
    if id == 0xa {
        let p = value as usize;
        match read_u8(p + staff::SQUAD_SELECTED_FOR) {
            1 => {
                write_u8(p + staff::SQUAD_SELECTED_FOR, 0);
                set_screen_param(0xe, (registered as i32 - 1) as usize);
            }
            2 => {
                write_u8(p + staff::SQUAD_SELECTED_FOR, 4);
                set_screen_param(0xf, (reserves as i32 + 1) as usize);
            }
            4 => {
                write_u8(p + staff::SQUAD_SELECTED_FOR, 2);
                set_screen_param(0xf, (reserves as i32 - 1) as usize);
            }
            _ => {
                write_u8(p + staff::SQUAD_SELECTED_FOR, 1);
                set_screen_param(0xe, (registered as i32 + 1) as usize);
            }
        }
        return -0xb;
    }
    if id == 0xb {
        if mode == 2 {
            for k in 0..SQUAD_NUMBERS_SLOTS {
                let p = read_usize(squad + 4 * k);
                if squad_numbers_in_database(p) {
                    match read_u8(p + staff::SQUAD_SELECTED_FOR) {
                        1 => write_u8(p + staff::SQUAD_SELECTED_FOR, 0),
                        4 => write_u8(p + staff::SQUAD_SELECTED_FOR, 2),
                        _ => {}
                    }
                }
            }
        }
        return -9;
    }
    if id == 0xc {
        covered!("tel05_1 submission");
        let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
        if mode == 1 {
            if fixture != 0 {
                let away = club != read_usize(fixture + fixture::HOME_CLUB);
                let side = if away { fixture::AWAY_SIDE_MARK } else { fixture::HOME_SIDE_MARK };
                let flag = (read_u8(fixture + side) == 0) as i32;
                let mut k = 0i8;
                while (k as i32) < (read_u8(fixture + 0x4d) & 0xf) as i32 + 0xb {
                    // simplification: the slot is a byte in a stack slot whose upper bytes are param 0xb's (a char)
                    let p = exe::tactics_get_selected_player(read_usize(G_TACTICS), club, k as u8 as u32, flag);
                    if p != 0 && read_usize(p + staff::PLAYER) != 0 && squad_numbers_number(p, club) == 0 {
                        let colours = squad_numbers_kit_colours(club);
                        let text = lang(0xde1f64, 0x9912d4, 0, colours.0, colours.1, 0); // "Not all players have been assigned a squad number."
                        squad_numbers_confirm(0x991308, 0x9912c8, text, 0, colours, 0); // "Submit Squad Numbers", "Incomplete"
                        return -4;
                    }
                    k += 1;
                }
            }
            let colours = squad_numbers_kit_colours(club);
            let question = if fixture != 0 {
                0x9913a4 // "Submit new squad number changes ?"
            } else if next_season != 0 {
                0x99135c // "Submit the current set of squad numbers for the forthcoming season ?"
            } else {
                0x991320 // "Submit the squad number changes for the current season ?"
            };
            lang(0xde1f64, question);
            squad_numbers_confirm(0x991308, 0x9909b8, 0xde1f64, 1, colours, 0x16); // "Submit Squad Numbers", "Please Confirm"
            return -4;
        }
        let asked = if squad_size < 0x14 { squad_size as i32 } else { 0x14 };
        let have = registered as i32 - reserves as i32 + rule_d as i8 as i32;
        if have < asked && (rule_b == 1 || rule_d == 0) {
            let colours = squad_numbers_kit_colours(club);
            // "Not all players have been registered." / "Not enough players have been registered - you must register at least 20."
            lang(0xde1f64, if squad_size < 0x14 { 0x9912a0 } else { 0x991254 });
            squad_numbers_confirm(0x99123c, 0x9912c8, 0xde1f64, 0, colours, 0); // "Submit European Squad", "Incomplete"
            return -4;
        }
        let colours = squad_numbers_kit_colours(club);
        let text = lang(0xde1f64, 0x991218, 1, colours.0, colours.1, 0x17); // "Submit the current European squad ?"
        squad_numbers_confirm(0x99123c, 0x9909b8, text, 1, colours, 0x17);
        return -4;
    }
    if id == 0xd {
        let open: unsafe extern "C" fn(u32, u32, i32, u32) = core::mem::transmute(0x8f6770usize);
        open(value, 0, -1, 0);
        return -4;
    }
    let r = exe::menubar_handle_command(widget);
    if r != 0 {
        return r;
    }
    exe::menubar_handle_object_link(widget)
}

/// Comparer for `squad_numbers_screen_handle_event`.
///
/// Only the original runs: the handler writes the squad numbers and the European registrations, acknowledges news items,
/// opens the confirmation and transfer status screens and answers the menu bar (none of it runs twice). The copy is
/// proven in Rust mode by its branches.
pub(crate) unsafe extern "C" fn compare_squad_numbers_screen_handle_event(widget: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I0%%)(widget)
}

/// Rule: leaving the squad numbers screen frees its squad copy.
///
/// `squad_numbers_screen_leave()`: the block of parameter 0x11, when there is one, freed (`free`) and the parameter
/// cleared.
/// Original: `squad_numbers_screen_leave` (0x47c070, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn squad_numbers_screen_leave() {
    if get_screen_param(0x11) != 0 {
        covered!("tel05_1 squad copy freed");
        exe::crt_free(get_screen_param(0x11));
        set_screen_param(0x11, 0);
    }
}

/// Comparer for `squad_numbers_screen_leave`: only the original runs (the block is freed once); afterwards parameter
/// 0x11 must be empty.
pub(crate) unsafe extern "C" fn compare_squad_numbers_screen_leave() {
    original_fn::<unsafe extern "C" fn()>(%%I1%%)();
    let left = get_screen_param(0x11);
    SCOREBOARD.record(left == 0, || format!("squad_numbers_screen_leave (squad numbers screen): parameter 0x11 still {:#x}", left));
}

/// The parameters 1..9 the part exchange screen opens with: 1, 2 and 3 empty, the view 0x41 (Traditional, physical
/// attributes), the sort 8 (positions), the statistics category 6, the two sort columns -1 and the filter 0x154e21.
const PART_EXCHANGE_OPEN_PARAMS: [u32; 9] = [0, 0, 0, 0x41, 8, 6, u32::MAX, u32::MAX, 0x154e21];

/// Rule: choosing a player to add to a transfer bid opens the part exchange screen for the bid, with the default view,
/// sort and filter.
///
/// `open_add_part_exchange_player_screen(bid)`: `exe::screenmanager_open_screen` with the builder 0x47c180 and the
/// handler 0x4814e0 (1, 0, 0); when it opens, param 0 the bid and params 1..9 `PART_EXCHANGE_OPEN_PARAMS`.
/// Original: `open_add_part_exchange_player_screen` (0x47c0b0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_add_part_exchange_player_screen(bid: usize) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x47c180usize, 0x4814e0usize, 1, 0usize, 0) == 0 {
        return;
    }
    covered!("tel05_1 part exchange screen opened");
    set_screen_param(0, bid);
    for (k, v) in PART_EXCHANGE_OPEN_PARAMS.iter().enumerate() {
        set_screen_param(k as u32 + 1, *v as usize);
    }
}

/// Comparer for `open_add_part_exchange_player_screen`: opening a screen does not run twice, so only the original runs;
/// when the new screen holds the bid in param 0, params 1..9 are checked against `PART_EXCHANGE_OPEN_PARAMS`.
pub(crate) unsafe extern "C" fn compare_open_add_part_exchange_player_screen(bid: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I2%%)(bid);
    if get_screen_param(0) != bid {
        return;
    }
    let differ: Vec<u32> = (1..=9u32).filter(|&k| get_screen_param(k) as u32 != PART_EXCHANGE_OPEN_PARAMS[k as usize - 1]).collect();
    SCOREBOARD.record(differ.is_empty(), || format!("open_add_part_exchange_player_screen (part exchange screen): parameters {:x?} differ from the plan", differ));
}
