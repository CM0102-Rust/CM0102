// ---------- batch %%LEVA%%: the staff profile screen (staff_screens.cpp, part 2) ----------

/// The screen's shared text buffer (the one every builder formats into before handing it to the display list).
const PROFILE_BUF: usize = 0xde1f64;
/// A dword the profile builder clears on entry (staff_screens.cpp; read by the profile's handler) [inference].
const PROFILE_SCREEN_RESET: usize = 0xa84aa8;
/// One `MenuItem` of the profile's tab bar: 0xd4 bytes (25 of them, built and destroyed by the CRT vector iterators
/// with 0x4568f0 / 0x5a53c0): +0 the word −1, +2 the colour word, +4 the text, +0xcc the style (0x11 shown, 0 greyed),
/// +0xd0 the tab number.
const PROFILE_TAB_ITEM: usize = 0xd4;
const PROFILE_TAB_COUNT: usize = 25;

/// `Text::draw_wrapped` (0x4aeaf0, thiscall, ret 0x20): a Text into area `slot` from `left` to `right`, starting at row
/// `*line` (moved on), with the font, the colour, the style and one more flag.
type ProfileDrawFn = unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i32, usize, usize, usize, usize) -> usize;

/// Everything the profile builder keeps in its frame: the params of the screen (0..8 and 0x14), the viewer (the
/// screen record's human, a person among the 16 last staff records), the kit colours, the contract and the offer of
/// the viewer's club, the coordinates of the next block and the buffers.
struct ProfileScreen {
    staff: usize,
    tab: i8,
    view: u8,
    list: usize,
    mode: i32,
    flags: u32,
    history: usize,
    history_index: i32,
    history_row: usize,
    compare_columns: usize,
    compare1: usize,
    compare2: usize,
    viewer: usize,
    viewer_club: usize,
    viewer_nation_team: usize,
    contract: usize,
    offer: usize,
    fg: u32,
    bg: u32,
    y: i32,
    bottom: i32,
    text: [usize; 2],
    name: [u8; 0xc8],
    action: [u8; 0x100],
}

/// `Display::add_area` on the current human's Display.
unsafe fn profile_area(x1: usize, y1: usize, x2: usize, y2: usize, cols: usize, align: usize, rows: usize, a8: usize, a9: usize, colour: usize, parent: usize) -> usize {
    let f: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    f(current_human_display(), x1, y1, x2, y2, cols, align, rows, a8, a9, colour, parent)
}

/// `Display::add_item` on the current human's Display, in the shape the profile uses: kind (2 text item, 0x2002 the
/// chosen one, 1 the separator), the flags, the three colours, the text, the item code and its data, the parent.
unsafe fn profile_item(kind: usize, flags: usize, c1: usize, c2: usize, c3: usize, text: usize, code: usize, data: usize, parent: usize) -> usize {
    let f: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    f(current_human_display(), kind, 0, 0, 0, 0, 0, 0, flags, c1, c2, 1, 1, c3, text, 0, code, data, parent)
}

/// `ScreenManager::add_button_item` (0x804030, thiscall, ret 0x30).
unsafe fn profile_button(x1: usize, y1: usize, x2: usize, y2: usize, a5: usize, c1: usize, a7: usize, a8: usize, c2: usize, text: usize, code: usize, data: usize) -> usize {
    let f: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    f(SCREEN_MANAGER, x1, y1, x2, y2, a5, c1, a7, a8, c2, text, code, data)
}

/// `ScreenManager::add_area_text` (0x804100, thiscall, ret 0x24): cell (`col`, `row`) of table `area`.
unsafe fn profile_cell(area: usize, col: usize, row: usize, a4: usize, a5: usize, a6: usize, a7: usize, colour: usize, text: usize) -> usize {
    let f: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    f(SCREEN_MANAGER, area, col, row, a4, a5, a6, a7, colour, text)
}

/// `ScreenManager::add_area_button` (0x8040a0, thiscall, ret 0x2c): a clickable cell with an item code and its data.
unsafe fn profile_cell_button(area: usize, col: usize, row: usize, a4: usize, a5: usize, a6: usize, a7: usize, colour: usize, text: usize, code: usize, data: usize) -> usize {
    let f: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    f(SCREEN_MANAGER, area, col, row, a4, a5, a6, a7, colour, text, code, data)
}

/// The separator line of a menu (`profile_item` kind 1, flags 0x1000010, no text).
unsafe fn profile_separator(parent: usize) {
    profile_item(1, 0x1000010, colour_word(0xae31aa), colour_word(G_GUI_COLOUR_YELLOW), colour_word(G_GUI_COLOUR_BLACK), 0, 0, 0, parent);
}

/// One entry of a profile menu (text in the shared buffer): the first one creates the "Action" button (kit colours,
/// top right) and its drop-down area (0x130); the entry goes into it with `code` and `data`. `menu` is −1 (low word)
/// until then.
unsafe fn profile_add_action(menu: &mut usize, fg: u32, bg: u32, action: usize, code: usize, data: usize) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if *menu as u16 == 0xffff {
        lang(action, 0xa8560c); // "Action"
        let b = profile_button(0x294, 4, 0x311, 0x18, 0x30, fg as usize, 0xc, 1, bg as usize, action, 0, 0);
        *menu = profile_area(0, 0, 0, 0, 0, 0, 0, 0, 0x130, colour_word(0xae31aa), b);
    }
    profile_item(2, 0x10, colour_word(0xae31aa), colour_word(G_GUI_COLOUR_YELLOW), colour_word(G_GUI_COLOUR_BLACK), PROFILE_BUF, code, data, *menu);
}

/// `CONTRACT_MANAGER::get_contract` of `s` kept in `G_CONTRACT_LOOKUP_RESULT` (the original's inline lookup).
unsafe fn profile_contract_lookup(s: usize) -> usize {
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), s);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    c
}

/// The club of the contract of `s` (`profile_contract_lookup`), or 0 without a contract.
unsafe fn profile_contract_club(s: usize) -> usize {
    let c = profile_contract_lookup(s);
    if c == 0 { 0 } else { club_at(read_i32(c + contract::CLUB) as isize).addr() }
}

/// Whether the contract of `s` (`profile_contract_lookup`) exists and is of type `t` (`contract::CONTRACT_TYPE` & 0x3f).
unsafe fn profile_contract_is(s: usize, t: u8) -> bool {
    let c = profile_contract_lookup(s);
    c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f == t
}

/// `human_manager_current_controls_club(club, 0, 1)`.
unsafe fn profile_controls(club: usize) -> bool {
    exe::human_manager_current_controls_club(club, 0, 1) != 0
}

/// Byte `o` of the 79-byte staff state record of `s` (`G_STAFF_STATE_TABLE`).
unsafe fn profile_state(s: usize) -> usize {
    read_usize(G_STAFF_STATE_TABLE).wrapping_add((read_i32(s) as usize).wrapping_mul(staff_state::SIZE))
}

/// `staff_is_not_board_member` (0x4cb960, stdcall, ret 4; the callers load ecx with the contract manager).
unsafe fn profile_not_board_member(s: usize) -> bool {
    let f: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x4cb960usize);
    f(s) != 0
}

/// The CRT's inline strcpy of the original (the same bytes).
unsafe fn profile_strcpy(dst: usize, src: usize) {
    let mut i = 0;
    loop {
        let c = read_u8(src + i);
        write_u8(dst + i, c);
        if c == 0 {
            break;
        }
        i += 1;
    }
}

/// The CRT's inline strcat of the original.
unsafe fn profile_strcat(dst: usize, src: usize) {
    let mut n = 0;
    while read_u8(dst + n) != 0 {
        n += 1;
    }
    profile_strcpy(dst + n, src);
}

/// The params, the creations and the person's name: reads params 0..8 and 0x14 and the two compare players
/// (`SCOUT_MANAGER::get_compare_player` 1 and 2); with no offer list (param 3) creates one (`LLIST`, `operator new`
/// 0xe), stores it in param 3 and fills it (`TRANSFER_MANAGER::get_offers_for_player`); with no player sheet, param 5
/// loses bits 1..3 (1 when nothing is left) and a reserve flag (`staff::PLAYING_SQUAD` bit 2) becomes 1
/// (`staff_set_squad_flags`); with no manager record (`Staff_Records::get_manager_record`), param 5 loses bit 4 (1 when
/// nothing is left); on tab 5 with bit 2 of param 5, no career history (param 6) and a player sheet, builds it
/// (`PlayerStats::build_career_history`) into params 6 and 7. The background photo (`background_set_staff_picture`),
/// the viewer, his club and national team; the contract (`CONTRACT_MANAGER::get_contract`), and for someone without
/// one who has a non-player sheet, is in the database, has a club and a contract job (`is_contract_staff_job`), a new
/// `STAFF_CONTRACT` (`operator new` 0x50, the person's id, `generate_initial_contract`, type 1 keeping bits 0, 6 and 7)
/// signed with the contract manager (`sign_contract`). The name (`database_format_staff_name`); view 2 without a
/// national job becomes view 1.
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
unsafe fn profile_screen_setup(st: &mut ProfileScreen) {
    let compare_player: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x7f7920usize);
    let picture: unsafe extern "C" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x41c790usize);
    let contract_job: unsafe extern "stdcall" fn(u32) -> u32 = core::mem::transmute(0x4cbb20usize);
    write_u32(PROFILE_SCREEN_RESET, 0);
    st.staff = get_screen_param(0);
    st.tab = get_screen_param(1) as u8 as i8;
    st.view = get_screen_param(2) as u8;
    st.list = get_screen_param(3);
    st.mode = get_screen_param(4) as u8 as i8 as i32;
    st.flags = get_screen_param(5) as u32;
    st.history = get_screen_param(6);
    st.history_index = get_screen_param(7) as i32;
    st.history_row = get_screen_param(8);
    st.compare_columns = get_screen_param(0x14);
    st.compare1 = compare_player(read_usize(G_SCOUT_MANAGER), 1);
    st.compare2 = compare_player(read_usize(G_SCOUT_MANAGER), 2);
    if st.list == 0 {
        covered!("tel07_3 profile creates the offer list");
        let p = exe::crt_operator_new(0xe);
        st.list = if p != 0 { exe::new_llist(p) } else { 0 };
        set_screen_param(3, st.list);
        exe::transfer_manager_get_offers_for_player(read_usize(G_TRANSFER_MANAGER), st.staff, st.list, 0);
    }
    let s = st.staff;
    if read_usize(s + staff::PLAYER) == 0 {
        st.flags &= 0xfffffff1;
        if st.flags == 0 {
            st.flags = 1;
        }
        set_screen_param(5, st.flags as usize);
        if read_u8(s + staff::PLAYING_SQUAD) & 2 != 0 {
            exe::staff_set_squad_flags(s, 1);
        }
    }
    if exe::staff_records_get_manager_record(read_usize(G_STAFF_RECORDS), s, 0) == 0 {
        st.flags &= 0xffffffef;
        if st.flags == 0 {
            st.flags = 1;
        }
        set_screen_param(5, st.flags as usize);
    }
    if st.tab == 5 && st.flags & 2 != 0 && st.history == 0 && read_usize(s + staff::PLAYER) != 0 {
        covered!("tel07_3 profile builds the career history");
        st.history = exe::playerstats_build_career_history(read_usize(G_PLAYER_STATS), s, &mut st.history_index);
        set_screen_param(6, st.history);
        set_screen_param(7, st.history_index as usize);
    }
    picture(s, 0, 0);
    st.viewer = read_usize(G_SCREEN_RECORDS_MANAGER + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300);
    if st.viewer != 0 {
        st.viewer_club = read_usize(st.viewer + staff::CLUB_JOB);
        st.viewer_nation_team = read_usize(st.viewer + staff::NATIONAL_JOB);
    }
    st.contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), s);
    if st.contract == 0
        && read_usize(s + staff::NON_PLAYER) != 0
        && (s == 0 || read_i32(s) < read_i32(G_STAFF_COUNT))
        && read_usize(s + staff::CLUB_JOB) != 0
        && contract_job(read_u8(s + staff::JOB_FOR_CLUB) as u32) != 0
    {
        covered!("tel07_3 profile creates a staff contract");
        let p = exe::crt_operator_new(0x50);
        st.contract = if p != 0 { exe::new_staff_contract(p) } else { 0 };
        let c = st.contract;
        if c != 0 {
            write_u32(c, read_u32(s));
            exe::staff_contract_generate_initial_contract(c, s, 0);
            write_u8(c + contract::CONTRACT_TYPE, (read_u8(c + contract::CONTRACT_TYPE) & 0xc1) | 1);
            // simplification: the original pushes the job byte in a dword whose high bytes are register garbage
            // (the callee reads the byte); here zero-extended
            exe::contract_manager_sign_contract(read_usize(G_CONTRACT_MANAGER), c, read_u8(s + staff::JOB_FOR_CLUB) as u32);
        }
    }
    exe::database_format_staff_name(s, st.name.as_mut_ptr() as usize, 0xc8, 1);
    if st.view == 2 && read_usize(s + staff::NATIONAL_JOB) == 0 {
        st.view = 1;
    }
    st.y = 0x50;
    st.bottom = 0x221;
}

/// The title box of the profile: a red-bordered box (0x64, 0xa)-(0x316, 0x46) in the background colour, the Text
/// drawn in font 7, or 6 when the title is at least 0x2b2 wide in font 7, in the kit's foreground colour.
unsafe fn profile_screen_title_box(st: &mut ProfileScreen) {
    let draw: ProfileDrawFn = core::mem::transmute(0x4aeaf0usize);
    let bg = st.bg as usize;
    profile_area(0x64, 0xa, 0x316, 0x46, 1, 0, 1, 0, 0x30, bg, usize::MAX);
    let font = if exe::font_text_width(7, PROFILE_BUF as *const u8) < 0x2b2 { 7 } else { 6 };
    let mut row: i32 = 0;
    let a = profile_area(0x64, 0x19, 0x316, 0x37, 1, 0, 1, 0, 1, bg, usize::MAX);
    draw(st.text.as_mut_ptr() as usize, a, 0x64, 0x316, &mut row, font, st.fg as usize, 3, 0);
}

/// The heading of the profile, by the view (param 2): view 1 with a club, the club's kit and "<n>. <name> (<club>)"
/// with the squad number when the club uses squad numbers and the person has one, " Reserves" for a reserve
/// (`staff::PLAYING_SQUAD` bit 2) of a human club, the club linked (0x7d5); a national manager (national job 5, club
/// job not 5), "<name> (<nation>)" in the national team's kit; view 2 with a national job, the national team's kit and
/// "<name> (<club>)" (no club: the national team's name, with its address as the grammar args, as in the original);
/// otherwise the screen title (`ScreenManager::add_screen_title`) in his nation's kit with "<name> (Retired)" for the
/// retired; no nation: false (the caller shows the error box, line 0x308, and builds nothing more). The colours go to
/// params 0x11 and 0x12.
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
unsafe fn profile_screen_heading(st: &mut ProfileScreen) -> bool {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let s = st.staff;
    let name = st.name.as_ptr() as usize;
    let text = st.text.as_mut_ptr() as usize;
    let club = read_usize(s + staff::CLUB_JOB);
    let nation_team = read_usize(s + staff::NATIONAL_JOB);
    if st.view == 1 && club != 0 {
        covered!("tel07_3 profile heading of a club");
        exe::database_get_club_kit_colours(club, 1, &mut st.fg, &mut st.bg);
        let squads = read_usize(G_SQUAD_MANAGER);
        let g = read_i8(club + club::SHORT_GENDER_NAME) as i32;
        let reserves = || read_u8(s + staff::PLAYING_SQUAD) & 2 != 0 && exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0;
        if exe::squad_manager_club_uses_squad_numbers(squads, club, 0) != 0 && exe::squad_manager_get_squad_number(squads, s, club) != 0 {
            let fmt = if reserves() { 0xa856d8 } else { 0xa856a0 }; // "<n>. <player> ({}<club>{}[ Reserves])"
            let n = (exe::squad_manager_get_squad_number(squads, s, club) as i32).abs();
            lang(PROFILE_BUF, fmt, g, g, n, name, club + club::SHORT_NAME);
        } else {
            let fmt = if reserves() { 0xa85674 } else { 0xa85654 }; // "<player> ({}<club>{}[ Reserves])"
            lang(PROFILE_BUF, fmt, g, g, name, club + club::SHORT_NAME);
        }
        exe::text_set_string(text, PROFILE_BUF as *const u8);
        exe::text_add_link(text, (club + club::SHORT_NAME) as *const u8, 0x7d5, club);
        profile_screen_title_box(st);
    } else if nation_team != 0 && read_u8(s + staff::JOB_FOR_NATION) == 5 && read_u8(s + staff::JOB_FOR_CLUB) != 5 {
        covered!("tel07_3 profile heading of a national manager");
        exe::database_get_club_kit_colours(nation_team, 1, &mut st.fg, &mut st.bg);
        let g = read_i8(nation_team + club::SHORT_GENDER_NAME) as i32;
        lang(PROFILE_BUF, 0xa85654, g, g, name, nation_team + club::SHORT_NAME);
        exe::text_set_string(text, PROFILE_BUF as *const u8);
        exe::text_add_link(text, (nation_team + club::SHORT_NAME) as *const u8, 0x7d5, nation_team);
        profile_screen_title_box(st);
    } else if st.view == 2 && nation_team != 0 {
        covered!("tel07_3 profile heading of the national view");
        exe::database_get_club_kit_colours(nation_team, 1, &mut st.fg, &mut st.bg);
        if club != 0 {
            let g = read_i8(club + club::SHORT_GENDER_NAME) as i32;
            lang(PROFILE_BUF, 0xa85654, g, g, name, club + club::SHORT_NAME);
            exe::text_set_string(text, PROFILE_BUF as *const u8);
            exe::text_add_link(text, (club + club::SHORT_NAME) as *const u8, 0x7d5, club);
        } else {
            // DEFECT (original): with no club the grammar args are the national team name's address
            let n = nation_team + club::SHORT_NAME;
            lang(PROFILE_BUF, 0xa85654, n, n, name, n);
            exe::text_set_string(text, PROFILE_BUF as *const u8);
            exe::text_add_link(text, n as *const u8, 0x7d5, nation_team);
        }
        profile_screen_title_box(st);
    } else {
        let nation = read_usize(s + staff::NATION);
        if nation == 0 {
            covered!("tel07_3 profile without a nation");
            return false;
        }
        covered!("tel07_3 profile screen title");
        exe::database_get_club_kit_colours(exe::database_get_national_team(nation), 1, &mut st.fg, &mut st.bg);
        if exe::player_regen_get_retirement_status(read_usize(G_PLAYER_REGEN), s) == 2 {
            lang(PROFILE_BUF, 0xa8563c, name); // "<player> (Retired)"
        } else {
            profile_strcpy(PROFILE_BUF, name);
        }
        exe::screenmanager_add_screen_title(SCREEN_MANAGER, st.bg, st.fg, PROFILE_BUF);
    }
    set_screen_param(0x11, (st.fg & 0xffff) as usize);
    set_screen_param(0x12, (st.bg & 0xffff) as usize);
    true
}

/// The contract and transfer entries of the profile's Action menu (the viewer's own players and the others):
/// "Remove From / Add To Shortlist" (0xb) for a player the viewer does not control who is not on the board, not
/// retired and not seen by a viewer of classification 5; the squad moves (3) by `CONTRACT_MANAGER::get_squad_placement`
/// (1 "Promote To Seniors", 2 "Relegate To Reserves", 3 "Promote To 'A' Team", 4 "Relegate To 'B' Team"); for a
/// player of a club the viewer controls "Set Transfer Status" (5, jobs 0xb, 0xd, 0xf, not type 5); "Terminate Loan
/// Deal" (0x18) for one on loan at it; for a contract with a club the viewer controls (not type 5) "Offer New Contract"
/// (6, no future transfer, not queued for a free transfer, not leaving on a Bosman), "Terminate Contract" (0x1f, not a
/// player job), "Recall From Loan" (0x18, a loan not of type 7), "Discipline Player" (0x24, player jobs), "Give player
/// leave of absence" (0x2e, with a contract, not injured or AWOL, a grievance in 0x20060) and "Release On A Free" (9,
/// no loan, offers, exchange offers, future transfer or free transfer queue, jobs 0xb, 0xf, 0xd); with an offer of the
/// viewer's club, "Offer Contract" (0xc, accepted offer, no future transfer), "Withdraw Transfer Bid" (0xd) and
/// "Change Transfer Bid" (0xe, status 0 or 1, a contracted player, not a loan offer); "Offer Trial" (0x17) for a
/// clubless player; "Approach To Buy" (0x10) and "Approach To Loan" (0x16) for another club's player.
/// Returns whether the viewer may approach to sign (the contract's expiry band, or type 1, without the viewer's
/// control; or the FIFA rules with the protected period over).
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
unsafe fn profile_screen_contract_actions(st: &mut ProfileScreen, menu: &mut usize) -> bool {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let shortlisted: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_SCOUT_MANAGER_IS_PLAYER_SHORTLISTED);
    let is_b_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5448c0usize);
    let s = st.staff;
    let (fg, bg, act) = (st.fg, st.bg, st.action.as_mut_ptr() as usize);
    let cm = read_usize(G_CONTRACT_MANAGER);
    let tm = read_usize(G_TRANSFER_MANAGER);
    let add = |menu: &mut usize, text: usize, code: usize, data: usize| {
        lang(PROFILE_BUF, text);
        profile_add_action(menu, fg, bg, act, code, data);
    };
    // the offer of the viewer's club among the offers for the person
    let list = st.list;
    write_u32(list + 0xa, read_u32(list + 2));
    let mut node = exe::llist_get_next(list);
    while node != 0 {
        let slot = exe::transfer_manager_get_offer_slot(tm, read_i32(node));
        if st.viewer_club != 0 && read_i32(read_usize(slot) + offer::BUYER_CLUB_ID) == read_i32(st.viewer_club) {
            st.offer = slot;
        }
        node = exe::llist_get_next(list);
    }
    let own = profile_contract_club(s) != 0 && profile_controls(profile_contract_club(s));
    if !own
        && profile_not_board_member(s)
        && st.viewer != 0
        && read_u8(st.viewer + staff::CLASSIFICATION) != 5
        && read_usize(s + staff::PLAYER) != 0
        && exe::player_regen_get_retirement_status(read_usize(G_PLAYER_REGEN), s) != 2
    {
        let text = if shortlisted(read_usize(G_SCOUT_MANAGER), s, 0) != 0 { 0xa85614 } else { 0xa855f8 }; // Remove From / Add To Shortlist
        add(menu, text, 0xb, 0);
    }
    let placement = exe::contract_manager_get_squad_placement(cm, s, st.viewer_club);
    if (1..=4).contains(&placement) {
        let text = [0xa855e4, 0xa855cc, 0xa855b8, 0xa855a0][placement as usize - 1];
        add(menu, text, 3, 0);
    }
    let club = read_usize(s + staff::CLUB_JOB);
    let job = read_u8(s + staff::JOB_FOR_CLUB);
    if ((club != 0 && profile_controls(club)) || (profile_contract_club(s) != 0 && profile_controls(profile_contract_club(s))))
        && !profile_contract_is(s, 5)
        && matches!(job, 0xb | 0xd | 0xf)
    {
        add(menu, 0xa8558c, 5, 0); // "Set Transfer Status"
    }
    if read_usize(s + staff::CLUB_JOB) != 0 && exe::contract_manager_get_loan_contract(cm, s) != 0 && profile_controls(read_usize(s + staff::CLUB_JOB)) {
        add(menu, 0xa85578, 0x18, 0); // "Terminate Loan Deal"
    }
    if profile_contract_club(s) != 0 && profile_controls(profile_contract_club(s)) && !profile_contract_is(s, 5) {
        covered!("tel07_3 profile actions of an own contract");
        if exe::transfer_manager_get_future_transfer(tm, s, 0) == 0
            && exe::transfer_manager_is_queued_for_free_transfer(tm, s) == 0
            && (exe::contract_manager_get_contract(cm, s) == 0 || read_u8(exe::contract_manager_get_contract(cm, s) + contract::LEAVING_ON_BOSMAN) == 0)
        {
            add(menu, 0xa85564, 6, 0); // "Offer New Contract"
        }
        let job = read_u8(s + staff::JOB_FOR_CLUB);
        if !matches!(job, 0xf | 0xd | 0xc | 0xb) {
            add(menu, 0x9b10f4, 0x1f, 0); // "Terminate Contract"
        }
        if exe::contract_manager_get_loan_contract(cm, s) != 0 && read_u8(exe::contract_manager_get_loan_contract(cm, s) + contract::CONTRACT_TYPE) & 0x3f != 7 {
            add(menu, 0xa85550, 0x18, 0); // "Recall From Loan"
        }
        let job = read_u8(s + staff::JOB_FOR_CLUB);
        if matches!(job, 0xf | 0xd | 0xc | 0xb) {
            add(menu, 0x9cc0e8, 0x24, 0); // "Discipline Player"
        }
        let injury = exe::injury_get_player_injury(read_usize(G_INJURY), s);
        let c = st.contract;
        if c != 0
            && exe::injury_get_player_injury_status(read_usize(G_INJURY), s) != 2
            && exe::awol_is_player_awol(read_usize(G_AWOL), s, 0) == 0
            && injury == 0
            && read_u32(c + contract::GRIEVANCES) & 0x20060 != 0
        {
            add(menu, 0xa85530, 0x2e, 0); // "Give player leave of absence"
        }
        if exe::contract_manager_get_loan_contract(cm, s) == 0
            && exe::transfer_manager_get_offers_for_player(tm, s, 0, 0) == 0
            && exe::transfer_manager_get_exchange_offers_for_player(tm, s, 0) == 0
            && exe::transfer_manager_get_future_transfer(tm, s, 0) == 0
            && exe::transfer_manager_is_queued_for_free_transfer(tm, s) == 0
            && matches!(read_u8(s + staff::JOB_FOR_CLUB), 0xb | 0xf | 0xd)
        {
            add(menu, 0xa8551c, 9, 0); // "Release On A Free"
        }
    }
    let offer = st.offer;
    if offer != 0 {
        covered!("tel07_3 profile actions of an offer");
        if read_u8(read_usize(offer) + offer::STATUS) == 5 && exe::transfer_manager_get_future_transfer(tm, s, 0) == 0 {
            add(menu, 0x9aff10, 0xc, offer); // "Offer Contract"
        }
        add(menu, 0xa85504, 0xd, offer); // "Withdraw Transfer Bid"
        let status = read_u8(read_usize(offer) + offer::STATUS);
        if (status == 0 || status == 1) && profile_contract_club(s) != 0 && read_u8(read_usize(offer) + offer::OFFER_TYPE) != 1 {
            add(menu, 0xa854f0, 0xe, offer); // "Change Transfer Bid"
        }
    }
    if read_usize(s + staff::CLUB_JOB) == 0
        && st.viewer_club != 0
        && offer == 0
        && exe::transfer_manager_get_future_transfer(tm, s, 0) == 0
        && exe::contract_manager_find_contract_offer(cm, read_u32(s), read_u32(st.viewer_club)) == 0
        && profile_not_board_member(s)
        && read_usize(s + staff::PLAYER) != 0
        && exe::player_regen_get_retirement_status(read_usize(G_PLAYER_REGEN), s) != 2
        && st.viewer != 0
        && read_usize(st.viewer + staff::CLUB_JOB) != 0
    {
        add(menu, 0xa854e4, 0x17, 0); // "Offer Trial"
    }
    // approach to buy and to loan: another club's player (0x872620..0x872918)
    let c = st.contract;
    'buy: {
        if profile_contract_club(s) == 0 || profile_controls(profile_contract_club(s)) {
            break 'buy;
        }
        if exe::transfer_manager_get_future_transfer(tm, s, 0) != 0 || offer != 0 || st.viewer_club == 0 {
            break 'buy;
        }
        if c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 1 && exe::nation_allows_free_transfers(read_usize(club_at(read_i32(c + contract::CLUB) as isize).addr() + club::NATION)) != 0 {
            break 'buy;
        }
        if profile_contract_is(s, 5) || is_b_team(st.viewer_club) != 0 || read_usize(s + staff::PLAYER) == 0 || !profile_not_board_member(s) {
            break 'buy;
        }
        if read_u8(profile_state(s) + staff_state::STATUS_FLAGS) & 0x30 == 0x20 || st.viewer == 0 || read_usize(st.viewer + staff::CLUB_JOB) == 0 {
            break 'buy;
        }
        covered!("tel07_3 profile approach to buy");
        // DEFECT (original): this entry does not open the Action button when it is the first one (parent −1)
        lang(PROFILE_BUF, 0xa854d4); // "Approach To Buy"
        profile_item(2, 0x10, colour_word(0xae31aa), colour_word(G_GUI_COLOUR_YELLOW), colour_word(G_GUI_COLOUR_BLACK), PROFILE_BUF, 0x10, 0, *menu);
        if read_u8(s + staff::JOB_FOR_CLUB) != 0xc && exe::contract_manager_get_loan_contract(cm, s) == 0 && !profile_controls(read_usize(s + staff::CLUB_JOB)) {
            add(menu, 0xa854c0, 0x16, 0); // "Approach To Loan"
        }
    }
    let mut can_sign = false;
    if c != 0 && (exe::staff_contract_get_expiry_band(c) != 0 || read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 1) {
        can_sign = !profile_controls(profile_contract_club(s));
    }
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1
        && c != 0
        && exe::staff_contract_check_fifa_protected_period(c, st.viewer_club, 0, 0) == 0
        && !profile_controls(profile_contract_club(s))
    {
        can_sign = true;
    }
    can_sign
}

/// "Approach To Sign" (0xf): a viewer with a club; for a player of a club without a type-5 contract, only when he may
/// approach (`profile_screen_contract_actions`) and the club uses the Bosman rule; not for a contract with a club the
/// viewer controls (not type 5); only a person outside the 16 last records, not on the board, without an offer of the
/// viewer's club, not retired.
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
unsafe fn profile_screen_approach_to_sign(st: &mut ProfileScreen, menu: &mut usize, can_sign: bool) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let s = st.staff;
    if read_usize(st.viewer + staff::CLUB_JOB) == 0 {
        return;
    }
    if read_usize(s + staff::CLUB_JOB) != 0 && !profile_contract_is(s, 5) && read_usize(s + staff::PLAYER) != 0 {
        if !can_sign || exe::transfer_manager_player_club_uses_bosman(read_usize(G_TRANSFER_MANAGER), s) == 0 {
            return;
        }
    }
    if profile_contract_club(s) != 0 && !profile_contract_is(s, 5) && profile_controls(profile_contract_club(s)) {
        return;
    }
    if read_i32(s) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10) || !profile_not_board_member(s) || st.offer != 0 {
        return;
    }
    if exe::player_regen_get_retirement_status(read_usize(G_PLAYER_REGEN), s) == 2 {
        return;
    }
    lang(PROFILE_BUF, 0xa854ac); // "Approach To Sign"
    profile_add_action(menu, st.fg, st.bg, st.action.as_mut_ptr() as usize, 0xf, 0);
}

/// The national team entries of the Action menu, when the viewer may pick the person for his national team
/// (0x76d9b0 of `G_NATIONAL_TEAMS`): a separator (only when the menu exists), then by the squad the person is in
/// (0x76da80: 0 the squad, 1 the 'B' squad, 2 the under-21/23 squad, of the viewer's team's nation): "Remove From"
/// (0x1c) or "Move To / Call Up To" (0x1b), with the data 0 squad, 1 'B' squad, 2 youth squad; the 'B' squad only for a
/// team with one (`database_club_has_reserve_squad`) and a viewer controlling all national teams; the youth squad
/// only when its team has no manager and the viewer controls all national teams, under 23 (the nation's byte at
/// +0x36 of the national teams) for those under 24, else under 21 for those under 22. When the viewer's team is the
/// nation itself: only the youth squad (remove, or call up by age).
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
unsafe fn profile_screen_national_actions(st: &mut ProfileScreen, menu: &mut usize) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let can_pick: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x76d9b0usize);
    let in_squad: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x76da80usize);
    let controls_all_fn: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_ALL_NATIONAL_TEAMS);
    let s = st.staff;
    let viewer = st.viewer;
    let nts = read_usize(G_NATIONAL_TEAMS);
    if can_pick(nts, s, viewer) == 0 {
        return;
    }
    covered!("tel07_3 profile national team actions");
    if *menu as u16 != 0xffff {
        profile_separator(*menu);
    }
    let (fg, bg, act) = (st.fg, st.bg, st.action.as_mut_ptr() as usize);
    let team = st.viewer_nation_team;
    let nation = read_usize(team + club::NATION);
    let g = read_i8(team + club::SHORT_GENDER_NAME) as i32;
    let nm = team + club::SHORT_NAME;
    let mut add = |menu: &mut usize, text: usize, code: usize, data: usize| {
        lang(PROFILE_BUF, text, g, g, nm);
        profile_add_action(menu, fg, bg, act, code, data);
    };
    let controls_all = || controls_all_fn(read_usize(G_HUMAN_MANAGER), viewer, 0) != 0;
    let under23 = || read_u8(read_usize(read_usize(G_NATIONAL_TEAMS) + 0x36).wrapping_add(read_i32(nation) as usize)) != 0;
    let youth_free = || read_usize(exe::database_get_national_u21_team(nation) + club::MANAGER) == 0;
    let age = || read_i8(s + staff::AGE);
    // the youth squad entry of the first paths (0x872ec3, 0x8731f2, 0x873877): `texts` = (under 23, under 21)
    let youth = |menu: &mut usize, add: &mut dyn FnMut(&mut usize, usize, usize, usize), texts: (usize, usize)| {
        if !youth_free() || !controls_all() {
            return;
        }
        if under23() {
            if age() < 0x18 {
                add(menu, texts.0, 0x1b, 2);
            }
        } else if age() < 0x16 {
            add(menu, texts.1, 0x1b, 2);
        }
    };
    if exe::club_is_national_team(team) == 0 {
        if in_squad(nts, s, nation, 0) != 0 {
            add(menu, 0xa85474, 0x1c, 0); // Remove From Squad
            if exe::database_club_has_reserve_squad(team) != 0 && controls_all() {
                add(menu, 0xa8543c, 0x1b, 1); // Move To 'B' Squad
            }
            youth(menu, &mut add, (0xa85400, 0xa853c4)); // Move To Under 23 / 21
        } else if in_squad(nts, s, nation, 1) != 0 {
            add(menu, 0xa85390, 0x1b, 0); // Move To Squad
            if controls_all() {
                add(menu, 0xa85354, 0x1c, 1); // Remove From 'B' Squad
            }
            youth(menu, &mut add, (0xa85400, 0xa853c4));
        } else if in_squad(nts, s, nation, 2) != 0 {
            add(menu, 0xa85390, 0x1b, 0); // Move To Squad
            if !controls_all() {
                return;
            }
            if exe::database_club_has_reserve_squad(exe::database_get_national_team(nation)) != 0 {
                add(menu, 0xa8543c, 0x1b, 1); // Move To 'B' Squad
            }
            if youth_free() {
                add(menu, if under23() { 0xa85314 } else { 0xa852d4 }, 0x1c, 2); // Remove From Under 23 / 21
            }
        } else {
            add(menu, 0xa8529c, 0x1b, 0); // Call Up To Squad
            if exe::database_club_has_reserve_squad(team) != 0 && controls_all() {
                add(menu, 0xa85260, 0x1b, 1); // Call Up To 'B' Squad
            }
            youth(menu, &mut add, (0xa85220, 0xa851e0)); // Call Up To Under 23 / 21
        }
    } else {
        let squad = in_squad(nts, s, nation, 2) != 0;
        let u23 = under23();
        if squad {
            add(menu, if u23 { 0xa85314 } else { 0xa852d4 }, 0x1c, 2);
        } else if u23 {
            if age() < 0x18 {
                add(menu, 0xa85220, 0x1b, 2);
            }
        } else if age() < 0x16 {
            add(menu, 0xa851e0, 0x1b, 2);
        }
    }
}

/// The staff entries of the Action menu: "Scout Player" (0x29) for another club's player without a scout request;
/// "Edit Nickname" (0x25) when the viewer gave him one or he has a common name, else "Set Nickname" (0x26; always with
/// no viewer); for the viewer's own club (or its linked senior club for a B team): "Request Physio Report" (0x2b) with
/// physios, "Request Coach Report" (0x2a) with a coach other than him (the assistant or the first of the five
/// coaches), after a separator; "Send for Rehabilitation" (0x2d, after a separator) for one flagged for it
/// (`staff_state::EXTRA_FLAGS` bit 1), unless his injury is of category 0xc or 0x62ed30 answers 0xff, which clears the
/// flag instead; "Create / Edit Manager Note" (0x2c, after a separator) for a person of the database when the notes
/// exist. The separators here do not check that the menu exists.
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
unsafe fn profile_screen_staff_actions(st: &mut ProfileScreen, menu: &mut usize, viewer_known: bool) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let physios: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x53c9b0usize);
    let rehab_blocked: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62ed30usize);
    let has_note: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x795750usize);
    let s = st.staff;
    let viewer = st.viewer;
    let (fg, bg, act) = (st.fg, st.bg, st.action.as_mut_ptr() as usize);
    let add = |menu: &mut usize, text: usize, code: usize| {
        lang(PROFILE_BUF, text);
        profile_add_action(menu, fg, bg, act, code, 0);
    };
    if viewer_known {
        let vclub = read_usize(viewer + staff::CLUB_JOB);
        if vclub != 0
            && (read_usize(s + staff::CLUB_JOB) == 0 || read_usize(s + staff::CLUB_JOB) != vclub)
            && profile_contract_club(s) != vclub
            && read_usize(s + staff::PLAYER) != 0
            && exe::scout_manager_has_scout_request(read_usize(G_SCOUT_MANAGER), s, 0, 0) == 0
        {
            add(menu, 0xa851d0, 0x29); // "Scout Player"
        }
        let common = read_usize(s + staff::COMMON_NAME);
        if exe::human_manager_has_nickname(read_usize(G_HUMAN_MANAGER), s, viewer as u32) != 0 || (common != 0 && read_u8(common) != 0) {
            add(menu, 0xa851c0, 0x25); // "Edit Nickname"
        } else {
            add(menu, 0xa851b0, 0x26); // "Set Nickname"
        }
    } else {
        add(menu, 0xa851b0, 0x26); // "Set Nickname"
    }
    let mut own = read_usize(s + staff::CLUB_JOB);
    if own != 0 {
        let mut senior_unlinked: u32 = st.list as u32;
        let b = exe::club_get_linked_b_team(own, &mut senior_unlinked, 1);
        if b != 0 && senior_unlinked == 0 {
            own = b;
        }
    }
    if read_usize(s + staff::PLAYER) != 0 && own != 0 && viewer != 0 && read_usize(viewer + staff::CLUB_JOB) != 0 && own == read_usize(viewer + staff::CLUB_JOB) {
        covered!("tel07_3 profile reports of the own club");
        let mut coach = exe::database_get_club_assistant(own);
        if coach == s {
            coach = 0;
        }
        if coach == 0 {
            for k in 0..5 {
                let c = read_usize(own + club::COACHES + k * 4);
                if c != s && c != 0 {
                    coach = c;
                    break;
                }
            }
        }
        if coach != 0 || physios(read_usize(viewer + staff::CLUB_JOB)) != 0 {
            profile_separator(*menu);
        }
        if physios(read_usize(viewer + staff::CLUB_JOB)) != 0 {
            add(menu, 0xa6b2cc, 0x2b); // "Request Physio Report"
        }
        if coach != 0 {
            add(menu, 0x99b4d4, 0x2a); // "Request Coach Report"
        }
    }
    // the viewer is not checked here, as in the original
    let vclub = read_usize(viewer + staff::CLUB_JOB);
    if vclub != 0 && own == vclub && read_u8(profile_state(s) + staff_state::EXTRA_FLAGS) & 1 != 0 {
        let injury = exe::injury_get_player_injury(read_usize(G_INJURY), s);
        if (injury != 0 && exe::injury_get_type_category(read_usize(G_INJURY), read_i8(injury)) == 0xc) || rehab_blocked(read_usize(G_INJURY), s) == 0xff {
            covered!("tel07_3 profile clears the rehabilitation flag");
            let e = profile_state(s) + staff_state::EXTRA_FLAGS;
            write_u8(e, read_u8(e) & 0xfe);
        } else {
            profile_separator(*menu);
            add(menu, 0xa85198, 0x2d); // "Send for Rehabilitation"
        }
    }
    if (s == 0 || read_i32(s) < read_i32(G_STAFF_COUNT)) && read_usize(G_NOTES) != 0 {
        profile_separator(*menu);
        let text = if has_note(read_usize(G_NOTES), viewer, s) != 0 { 0xa85170 } else { 0xa85184 }; // Edit / Create Manager Note
        add(menu, text, 0x2c);
    }
}

/// The Action menu of the profile: with a pending award vote of the viewer for this person whose shortlist lets him
/// vote (`AWARD_MANAGER::get_pending_vote`, `AWARD_SHORTLIST::get_vote_status` −1), only the "Vote For Award" button
/// (code 2, the shortlist as data); for a person outside the database, nothing; otherwise the menu entries
/// (`profile_screen_contract_actions`, `profile_screen_approach_to_sign`, `profile_screen_national_actions`,
/// `profile_screen_staff_actions`; with no viewer straight to "Set Nickname"). Returns the menu area (−1 when none).
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
unsafe fn profile_screen_action_menu(st: &mut ProfileScreen) -> usize {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let pending_vote: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4149a0usize);
    let vote_status: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(0x418140usize);
    let s = st.staff;
    let mut menu = usize::MAX;
    if st.viewer != 0 {
        let e = pending_vote(read_usize(G_AWARD_MANAGER), st.viewer, s);
        if e != 0 {
            let am = read_usize(G_AWARD_MANAGER);
            let t = read_usize(read_usize(am + 8).wrapping_add((read_i32(e + 4) as usize).wrapping_mul(4)));
            let shortlist = read_usize(read_usize(t + 4).wrapping_add((read_i32(e + 8) as usize).wrapping_mul(4)));
            if vote_status(read_usize(shortlist + 0x1f), st.viewer) == 0xffff {
                covered!("tel07_3 profile vote for award");
                lang(PROFILE_BUF, 0xa8562c); // "Vote For Award"
                profile_button(0x294, 4, 0x311, 0x18, 0x30, st.fg as usize, 0xc, 1, st.bg as usize, PROFILE_BUF, 2, shortlist);
                return menu;
            }
        }
    }
    if s != 0 && read_i32(s) >= read_i32(G_STAFF_COUNT) {
        return menu;
    }
    covered!("tel07_3 profile action menu");
    let can_sign = profile_screen_contract_actions(st, &mut menu);
    if st.viewer == 0 {
        profile_screen_staff_actions(st, &mut menu, false);
        return menu;
    }
    profile_screen_approach_to_sign(st, &mut menu, can_sign);
    profile_screen_national_actions(st, &mut menu);
    profile_screen_staff_actions(st, &mut menu, true);
    menu
}

/// The "Compare Players" entry of the menu (code 0, under the Action menu, −1 when there is none) for a player of
/// the database who is not retired, with its sub-menu: "Set as 1st player in comparison" (0x2f, with the current one's
/// name), "Set as 2nd player in comparison" (0x30) and "Compare two chosen players" (0x31); when the sub-menu area is
/// −1 the entries open their own Action button.
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
unsafe fn profile_screen_compare_entries(st: &mut ProfileScreen, menu: usize) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let s = st.staff;
    if read_usize(s + staff::PLAYER) == 0 || read_i32(s) >= read_i32(G_STAFF_COUNT) || read_u8(profile_state(s) + staff_state::STATUS_FLAGS) & 0x30 == 0x20 {
        return;
    }
    covered!("tel07_3 profile compare entries");
    lang(PROFILE_BUF, 0xa85160); // "Compare Players"
    let item = profile_item(2, 0x10, colour_word(0xae31aa), colour_word(G_GUI_COLOUR_YELLOW), colour_word(G_GUI_COLOUR_BLACK), PROFILE_BUF, 0, 0, menu);
    let mut sub = profile_area(0, 0, 0, 0, 0, 0, 0, 0, 0x30, colour_word(0xae31aa), item);
    let (fg, bg, act) = (st.fg, st.bg, st.action.as_mut_ptr() as usize);
    let name = st.name.as_mut_ptr() as usize;
    for (k, (p, with, without)) in [(st.compare1, 0xa85114, 0xa850f4), (st.compare2, 0xa850a8, 0xa85088)].into_iter().enumerate() {
        if p != 0 {
            exe::database_format_staff_name(p, name, 0xc8, 1);
            lang(PROFILE_BUF, with, name);
        } else {
            lang(PROFILE_BUF, without);
        }
        profile_add_action(&mut sub, fg, bg, act, 0x2f + k, 0);
    }
    lang(PROFILE_BUF, 0xa5a5e8); // "Compare two chosen players"
    profile_add_action(&mut sub, fg, bg, act, 0x31, 0);
}

/// The menu bar and the tab bar of the profile (`gui_add_tab_bar` with the 25 MenuItems, moving the top and the
/// bottom of the screen's body): Profile (1; greyed and dropped for a manager or one of the 16 last records),
/// Injuries & Bans (2; greyed without a player sheet or outside the database), Contract (3; greyed for jobs 1 and 2,
/// for a national manager unless a club player, and for the 16 last records), Transfer (4; greyed for jobs 1 and 2
/// and the 16 last records) and History (5; greyed for jobs 1 and 2 and outside the database).
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
unsafe fn profile_screen_tabs(st: &mut ProfileScreen, items: usize) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let s = st.staff;
    exe::menubar_build(4, 0);
    let grey = read_u16(G_GUI_COLOUR_GREY);
    let item = |k: usize| items + k * PROFILE_TAB_ITEM;
    let n = read_i32(G_STAFF_COUNT);
    let id = read_i32(s);
    lang(item(0) + 4, 0xa85080); // "Profile"
    write_u32(item(0) + 0xcc, 0x11);
    let job = read_u8(s + staff::JOB_FOR_CLUB);
    if job == 2 || (id >= n.wrapping_sub(0x10) && (s == 0 || id < n)) {
        write_u16(item(0) + 2, grey);
        write_u32(item(0) + 0xcc, 0);
    }
    let mut k = 0;
    if job != 2 && (id < n.wrapping_sub(0x10) || (s != 0 && id >= n)) {
        k = 1;
        write_u32(item(0) + 0xd0, 1);
    }
    let grey_out = |k: usize| {
        write_u16(item(k) + 2, grey);
        write_u32(item(k) + 0xcc, 0);
    };
    lang(item(k) + 4, 0xa85070); // "Injuries & Bans"
    write_u32(item(k) + 0xcc, 0x11);
    if read_usize(s + staff::PLAYER) == 0 || (s != 0 && read_i32(s) >= read_i32(G_STAFF_COUNT)) {
        grey_out(k);
    }
    write_u32(item(k) + 0xd0, 2); // tel07_3 the Injuries & Bans tab
    k += 1;
    lang(item(k) + 4, 0x98e6d0); // "Contract"
    write_u32(item(k) + 0xcc, 0x11);
    let job = read_u8(s + staff::JOB_FOR_CLUB);
    let national_only = read_u8(s + staff::JOB_FOR_NATION) == 5 && (read_usize(s + staff::CLUB_JOB) == 0 || job != 0xb);
    if job == 1 || national_only || job == 2 || read_i32(s) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10) {
        grey_out(k);
    }
    write_u32(item(k) + 0xd0, 3);
    k += 1;
    lang(item(k) + 4, 0xa85064); // "Transfer"
    write_u32(item(k) + 0xcc, 0x11);
    let job = read_u8(s + staff::JOB_FOR_CLUB);
    if job == 1 || job == 2 || read_i32(s) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10) {
        grey_out(k);
    }
    write_u32(item(k) + 0xd0, 4);
    k += 1;
    lang(item(k) + 4, 0xa85034, 0xad9c64usize); // "History"
    write_u32(item(k) + 0xcc, 0x11);
    let job = read_u8(s + staff::JOB_FOR_CLUB);
    if job == 1 || job == 2 || (s != 0 && read_i32(s) >= read_i32(G_STAFF_COUNT)) {
        grey_out(k);
    }
    write_u32(item(k) + 0xd0, 5);
    k += 1;
    exe::gui_add_tab_bar(items, k as u32, st.tab as i32, &mut st.y, &mut st.bottom, 1);
}

/// The nationality of the profile into `out` (200 bytes): the adjective of his nation (`nation_get_nationality_adjective`
/// with 0xc, first letter upper-cased by the CRT `toupper` 0x9471cd) and, for a second nation that is not the first,
/// without international caps, without the "blocked by the second nation" flag (`staff_state::EXTRA_FLAGS` bit 2, for
/// a person of the database) and not Portugal (`KEY_NATION_PORTUGAL`), "/" and the second adjective (`extra` keeps it).
/// No nation: empty.
unsafe fn profile_screen_nationality(s: usize, out: &mut [u8; 0xc8], extra: &mut [u8; 0xc8]) {
    let toupper: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x9471cdusize);
    let nation = read_usize(s + staff::NATION);
    if nation == 0 {
        out[0] = 0;
        return;
    }
    let second = read_usize(s + staff::SECOND_NATION);
    let blocked = read_i32(s) < read_i32(G_STAFF_COUNT) && read_u8(profile_state(s) + staff_state::EXTRA_FLAGS) & 2 != 0;
    if second != nation && second != 0 && read_u8(s + staff::INT_APPS) == 0 && !blocked && read_i32(second) != read_i32(KEY_NATION_PORTUGAL) {
        exe::nation_get_nationality_adjective(nation, out.as_mut_ptr() as usize, 0xc8, 0xc);
        out[0] = toupper(out[0] as i8 as i32) as u8;
        exe::nation_get_nationality_adjective(second, extra.as_mut_ptr() as usize, 0xc8, 0xc);
        extra[0] = toupper(extra[0] as i8 as i32) as u8;
        profile_strcat(out.as_mut_ptr() as usize, 0x9b7630); // "/"
        profile_strcat(out.as_mut_ptr() as usize, extra.as_ptr() as usize);
    } else {
        exe::nation_get_nationality_adjective(nation, out.as_mut_ptr() as usize, 0xc8, 0xc);
        out[0] = toupper(out[0] as i8 as i32) as u8;
    }
}

/// The identity block of the profile (not on tab 5): "Born <date> (Age <n>). <Nationality> (<caps>)." with the
/// nations linked to their national teams (0x7d5; the second only without caps), the caps line by caps and goals, or
/// "(capped at B Team level)" / "(capped at U21 level)" from the staff state; on tab 1, for a person shown in full
/// with both sheets or a scout report, the view button (" Non-Playing" 1, " Playing" 0, " Scout Report" 2, the
/// current one marked 0x2002, item 0x12); then the positions (`database_format_player_positions` style 2; not on tab 1
/// in mode 1) or the job ("Unemployed") above the bottom.
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
unsafe fn profile_screen_identity(st: &mut ProfileScreen) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let draw: ProfileDrawFn = core::mem::transmute(0x4aeaf0usize);
    let clear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4afa40usize);
    let short_date: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> usize = core::mem::transmute(0x54a650usize);
    let scout_report: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x7f8610usize);
    let positions: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x53ee50usize);
    let text_item: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let s = st.staff;
    let text = st.text.as_mut_ptr() as usize;
    if st.tab != 5 {
        covered!("tel07_3 profile identity");
        let (mut date, mut born, mut nat, mut extra, mut caps) = ([0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8]);
        short_date(s + staff::DATE_OF_BIRTH, date.as_mut_ptr() as usize, 0xc8, 0xff);
        lang(born.as_mut_ptr() as usize, 0xa85010, 0x64usize, date.as_ptr() as usize, read_i8(s + staff::AGE) as i32); // "Born {}<date> (Age <n>)"
        profile_screen_nationality(s, &mut nat, &mut extra);
        let apps = read_u8(s + staff::INT_APPS) as u32;
        let goals = read_u8(s + staff::INT_GOALS) as u32;
        let cp = caps.as_mut_ptr() as usize;
        if apps > 1 {
            match goals {
                0 => drop(lang(cp, 0xa84fb8, apps)),        // "(<n> caps)"
                1 => drop(lang(cp, 0xa84fcc, apps)),        // "(<n> caps/1 goal)"
                _ => drop(lang(cp, 0xa84fe8, apps, goals)), // "(<n> caps/<g> goals)"
            }
        } else if apps == 1 {
            match goals {
                0 => drop(lang(cp, 0xa84f84)),         // "(1 cap)"
                1 => drop(lang(cp, 0xa84f8c)),         // "(1 cap/1 goal)"
                _ => drop(lang(cp, 0xa84f9c, goals)), // "(1 cap/<g> goals)"
            }
        } else if (s == 0 || read_i32(s) < read_i32(G_STAFF_COUNT)) && read_usize(s + staff::NATION) != 0 {
            let state = profile_state(s);
            if read_u8(state + staff_state::B_INTERNATIONAL_PLAYED) != 0 {
                lang(cp, 0xa84f5c); // "(capped at B Team level)"
            } else if read_u8(state + staff_state::EXTRA_FLAGS) & 2 != 0 {
                lang(cp, 0xa84f44); // "(capped at U21 level)"
            } else {
                caps[0] = 0;
            }
        } else {
            caps[0] = 0;
        }
        if caps[0] != 0 {
            format(PROFILE_BUF, 0xa84f78, born.as_ptr() as usize, nat.as_ptr() as usize, caps.as_ptr() as usize); // "%s. %s %s."
        } else {
            format(PROFILE_BUF, 0xa84f3c, born.as_ptr() as usize, nat.as_ptr() as usize); // "%s. %s."
        }
        clear(text);
        exe::text_set_string(text, PROFILE_BUF as *const u8);
        // the links carry the lower-case adjectives (the line has them capitalised), as in the original
        let nation = read_usize(s + staff::NATION);
        if nation != 0 {
            exe::nation_get_nationality_adjective(nation, nat.as_mut_ptr() as usize, 0xc8, 0xc);
            exe::text_add_link(text, nat.as_ptr(), 0x7d5, exe::database_get_national_team(nation));
        }
        let second = read_usize(s + staff::SECOND_NATION);
        if second != 0 && second != read_usize(s + staff::NATION) && read_u8(s + staff::INT_APPS) == 0 {
            exe::nation_get_nationality_adjective(second, extra.as_mut_ptr() as usize, 0xc8, 0xc);
            exe::text_add_link(text, extra.as_ptr(), 0x7d5, exe::database_get_national_team(second));
        }
        let y = st.y as usize;
        let a = profile_area(0x6e, y, 0x30c, y.wrapping_add(0x1e), 1, 0, 1, 0, 2, 0, usize::MAX);
        let mut row: i32 = 0;
        draw(text, a, 0x6e, 0x30c, &mut row, 3, colour_word(G_GUI_COLOUR_YELLOW), 3, 0);
        let report = scout_report(read_usize(G_SCOUT_MANAGER), s);
        let n = read_i32(G_STAFF_COUNT);
        let id = read_i32(s);
        let job = read_u8(s + staff::JOB_FOR_CLUB);
        let shown = job != 2 && (id < n.wrapping_sub(0x10) || (s != 0 && id >= n));
        let sheets = read_usize(s + staff::PLAYER) != 0 && read_usize(s + staff::NON_PLAYER) != 0;
        if shown && st.tab == 1 && (sheets || report != 0) {
            covered!("tel07_3 profile view button");
            let y = st.y as usize;
            let b = profile_button(0x73, y.wrapping_add(5), 0x82, y.wrapping_add(0x19), 0x30, colour_word(G_GUI_COLOUR_BLACK), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), 0, 0, 0);
            let sub = profile_area(0, 0, 0, 0, 0, 0, 0, 0, 0x30, colour_word(0xae31aa), b);
            let entry = |text: usize, view: i32| {
                let kind = if st.mode == view { 0x2002 } else { 2 };
                profile_item(kind, 0x10, colour_word(0xae31aa), colour_word(G_GUI_COLOUR_YELLOW), colour_word(G_GUI_COLOUR_BLACK), text, 0x12, view as usize, sub);
            };
            if read_usize(s + staff::NON_PLAYER) != 0 {
                lang(PROFILE_BUF, 0xa84f2c); // " Non-Playing"
                entry(PROFILE_BUF, 1);
            }
            if read_usize(s + staff::PLAYER) != 0 {
                lang(PROFILE_BUF, 0xa84f00, 0xad9c64usize); // " Playing"
                entry(PROFILE_BUF, 0);
            }
            if report != 0 {
                profile_separator(sub);
                lang(PROFILE_BUF, 0xa84ef0); // " Scout Report"
                entry(PROFILE_BUF, 2);
            }
        }
        st.y += 0x23;
    }
    let player = read_usize(s + staff::PLAYER);
    if player != 0 && !(st.tab == 1 && st.mode == 1) {
        positions(player, PROFILE_BUF, 0x7d0, 2);
    } else if read_usize(s + staff::CLUB_JOB) != 0 {
        profile_strcpy(PROFILE_BUF, exe::database_get_job_name(read_u8(s + staff::JOB_FOR_CLUB) as u32, 0, 1, core::ptr::null_mut()));
    } else if read_usize(s + staff::NATIONAL_JOB) != 0 {
        profile_strcpy(PROFILE_BUF, exe::database_get_job_name(read_u8(s + staff::JOB_FOR_NATION) as u32, 0, 1, core::ptr::null_mut()));
    } else {
        lang(PROFILE_BUF, 0x9b6b04); // "Unemployed"
    }
    let b = st.bottom as usize;
    text_item(SCREEN_MANAGER, 0x6e, b.wrapping_sub(0x1e), 0x30c, b, 2, 0, 0xc, 3, colour_word(0xaebde8), PROFILE_BUF, 1);
    st.bottom -= 0x23;
}

/// The statistics table of a player of the database (7 rows of font 2 above the bottom, 12 columns aligned by
/// `align`): the first row the season of the career history (tab 5 with bit 2 of param 5 and a history: the club
/// linked, 0x7d5) or empty, the page buttons "<<" (0x3c) and ">>" (0x3d) (font symbols 0x2a / 0x2b when the font mode
/// is set) and the column titles (the list by param 0x14 and the goalkeeper rating 20 of the player sheet); then six
/// rows, each with its title and the values (`PlayerStats` 0x7c2750 from the history entry, or 0x7c25b0 from the
/// player sheet), the yellow / grey stripes, column 0x11 in its own colour.
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
unsafe fn profile_screen_statistics(st: &mut ProfileScreen, align: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let season_text: unsafe extern "C" fn(u32, usize, usize, u32) -> i32 = core::mem::transmute(0x669630usize);
    let column_title: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> usize = core::mem::transmute(0x7c1b20usize);
    let row_title: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> usize = core::mem::transmute(0x7c14b0usize);
    let history_value: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x7c2750usize);
    let sheet_value: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x7c25b0usize);
    let s = st.staff;
    let h = exe::font_get_height(2);
    let height = h.wrapping_mul(7);
    let (b, top) = (st.bottom as usize, st.bottom.wrapping_sub(height) as usize);
    profile_area(0x6e, top, 0x30c, b, 1, 0, 1, 0, 2, 0, usize::MAX);
    let table = profile_area(0x6e, top, 0x30c, b, 0xc, align, 7, 0, 1, 0, usize::MAX);
    let mut row: usize = 0;
    let from_history = st.tab == 5 && st.flags & 2 != 0 && st.history != 0 && read_i16(st.history + 0xeb) != -1;
    let mut entry = st.history.wrapping_add(st.history_row.wrapping_mul(0x2b));
    let mut name = [0u8; 0xc8];
    let np = name.as_mut_ptr() as usize;
    if from_history {
        covered!("tel07_3 profile statistics of a history season");
        let index = read_i32(entry + 2);
        let club = if index > -1 { club_at(index as isize).addr() } else { 0 };
        if index > -1 && club != 0 {
            season_text(read_u16(entry) as u32, club, np, 0x7d0);
            format(PROFILE_BUF, 0xa6a1d0, np, club + club::SHORT_NAME); // "  %s %s"
            profile_cell_button(table, 0, row, 1, 0, 1, 2, colour_word(0xaebdde), PROFILE_BUF, 0x7d5, club);
        } else {
            season_text(read_u16(entry) as u32, 0, np, 0x7d0);
            format(PROFILE_BUF, 0x987ff0, np); // "  %s"
            profile_cell(table, 0, row, 1, 0, 1, 2, colour_word(0xaebdde), PROFILE_BUF);
        }
    }
    let symbols = read_i32(G_FONT_MODE) != -1;
    for (col, symbol, arrow, code) in [(1usize, 0x2a, 0x98e7c4usize, 0x3c), (2, 0x2b, 0x98e7a8, 0x3d)] {
        let text = if symbols {
            format(PROFILE_BUF, 0x99ec1c, symbol); // "%c"
            PROFILE_BUF
        } else {
            arrow // "<<" / ">>"
        };
        profile_cell_button(table, col, row, 0x30, colour_word(G_GUI_COLOUR_DARK_BLUE), 0xc, (!symbols) as usize, colour_word(G_GUI_COLOUR_PALE_GREY), text, code, 0);
    }
    let columns = |s: usize| -> usize {
        let p = read_usize(s + staff::PLAYER);
        let keeper = p != 0 && read_i8(p + player_sheet::GOALKEEPER) == 0x14;
        match (st.compare_columns != 0, keeper) {
            (true, true) => 0xa84a90,
            (true, false) => 0xa84a78,
            (false, true) => 0xa84a9c,
            (false, false) => 0xa84a84,
        }
    };
    let list = columns(s);
    let mut i = 0;
    while read_u8(list + i) != 0xff {
        column_title(read_usize(G_PLAYER_STATS), read_u8(list + i) as u32, PROFILE_BUF, 0x7d0);
        profile_cell(table, 3 + i, row, 0x30, colour_word(G_GUI_COLOUR_GREY), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), PROFILE_BUF);
        i += 1;
    }
    row += 1;
    entry = entry.wrapping_add(6);
    for k in 1..=6u32 {
        row_title(read_usize(G_PLAYER_STATS), k, np, 0xc8);
        format(PROFILE_BUF, 0x987ff0, np); // "  %s"
        profile_cell(table, 0, row, 1, 0, 1, 2, colour_word(G_GUI_COLOUR_PALE_GREY), PROFILE_BUF);
        let list = columns(st.staff);
        let mut i = 0;
        while read_u8(list + i) != 0xff {
            let column = read_u8(list + i);
            if st.tab == 5 && st.flags & 2 != 0 && st.history != 0 && read_i16(st.history + 0xeb) != -1 {
                history_value(read_usize(G_PLAYER_STATS), entry, column as u32, PROFILE_BUF, 0x7d0, 1, 0, 0);
            } else {
                sheet_value(read_usize(G_PLAYER_STATS), read_usize(st.staff + staff::PLAYER), k, column as u32, PROFILE_BUF, 0x7d0, 1, 0, 1);
            }
            if column == 0x11 {
                profile_cell(table, 3 + i, row, 0x30, colour_word(0xae3192), 0xc, 1, colour_word(0xaebde8), PROFILE_BUF);
            } else {
                let colour = if (k as i32 - 1) % 2 != 0 { colour_word(0xaebdd4) } else { colour_word(G_GUI_COLOUR_YELLOW) }; // tel07_3 the stripes
                profile_cell(table, 3 + i, row, 1, 0, 0xc, 1, colour, PROFILE_BUF);
            }
            i += 1;
        }
        row += 1;
        entry = entry.wrapping_add(0x2b);
    }
    st.bottom = st.bottom.wrapping_sub(5).wrapping_sub(height);
}

/// The Injuries & Bans tab (between `y` and `bottom`, two columns aligned 2 and 7): for a player of the database, the
/// injury (`Injury::get_player_injury_status`; the injury text by its type 0x4d..0x51: attending, surgery,
/// radiotherapy or the foreseeable future, leave, or "Out for <period> with <injury>"; without one the potential
/// injury of the category or "None"), its type (the cause in a wrapped Text, "Injured during training", or "  -"),
/// the condition (by condition, fatigue and the jadedness statistic: lacking match practice, in need of a rest,
/// exhausted, or the percentage; "  -" for status 2), the training (full, light or none, "(absent from training
/// recently)" for one who missed training), the bans (`Discipline::get_slot_text` of the four slots, or "None") and
/// empty rows down to the bottom; otherwise "There is no injury and ban information available.". A person without an
/// injury record despite the status: the tab stops after the injury row (the original jumps to the footer).
/// Original: part of the staff profile screen builder (0x870700, staff_screens.cpp).
unsafe fn profile_screen_injuries_tab(st: &mut ProfileScreen) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let draw: ProfileDrawFn = core::mem::transmute(0x4aeaf0usize);
    let manager_area: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize = core::mem::transmute(0x4567f0usize);
    let injury_name: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x62ee30usize);
    let injury_period: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> usize = core::mem::transmute(0x630af0usize);
    let potential: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62bf60usize);
    let category_name: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> i8 = core::mem::transmute(0x630c50usize);
    let injury_cause: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x631dc0usize);
    let missed_training: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x418ff0usize);
    let ban_text: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> usize = core::mem::transmute(0x55d3c0usize);
    let (y, bottom) = (st.y, st.bottom);
    let mut cause = [0usize; 2];
    let cp = cause.as_mut_ptr();
    exe::new_text(cp, 0);
    let align: [u8; 2] = [2, 7];
    let s = get_screen_param(0);
    let inj = read_usize(G_INJURY);
    let pale = colour_word(G_GUI_COLOUR_PALE_GREY);
    let yellow = colour_word(G_GUI_COLOUR_YELLOW);
    if read_usize(s + staff::PLAYER) == 0 || (s != 0 && read_i32(s) >= read_i32(G_STAFF_COUNT)) {
        covered!("tel07_3 profile without injury information");
        let rows = (bottom.wrapping_sub(y) / exe::font_get_height(3)) as u8;
        lang(PROFILE_BUF, 0xa84adc); // "There is no injury and ban information available."
        let text_area: unsafe extern "C" fn(i32, i32, i32, i32, u32, u32, u32, u32, usize, usize, u32) -> u32 = core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
        text_area(0x6e, y, 0x30c, bottom, 2, 0, rows as u32, 3, pale, PROFILE_BUF, 0);
        exe::drop_text(cp);
        return;
    }
    covered!("tel07_3 profile injuries tab");
    let rows = (bottom.wrapping_sub(y) / exe::font_get_height(2)) as u8;
    // simplification: the original pushes the row count as a dword whose high bytes are a stale local; here zero-extended
    let area = manager_area(SCREEN_MANAGER, 0x6e, y as usize, 0x30c, bottom as usize, 2, align.as_ptr() as usize, rows as usize, 0, 1, 0, usize::MAX);
    let mut row: i32 = 0;
    let status = exe::injury_get_player_injury_status(inj, s);
    lang(PROFILE_BUF, 0xa84ee4); // "  Injury"
    profile_cell(area, 0, row as usize, 2, 0, 1, 2, pale, PROFILE_BUF);
    if status != 0 {
        let (mut name, mut period) = ([0u8; 0x100], [0u8; 0x100]);
        let (np, pp) = (name.as_mut_ptr() as usize, period.as_mut_ptr() as usize);
        injury_name(inj, s, np, 0xc8, 1);
        injury_period(inj, s, pp, 0xc8, 1, 1);
        let injury = exe::injury_get_player_injury(inj, s);
        if injury == 0 {
            covered!("tel07_3 profile injury status without an injury");
            exe::drop_text(cp);
            return;
        }
        match read_i8(injury) as i32 - 0x4d {
            0 => drop(lang(PROFILE_BUF, 0xa84e94, pp)),
            1 => drop(lang(PROFILE_BUF, 0xa84e4c, pp)),
            2 => drop(lang(PROFILE_BUF, 0xa84d84, pp)), // "  Out for <period> due to surgery"
            3 => drop(if status == 1 { lang(PROFILE_BUF, 0xa84dfc) } else { lang(PROFILE_BUF, 0xa84dac) }),
            4 => drop(lang(PROFILE_BUF, 0xa84d64, pp)), // "  On leave for <period>"
            _ => drop(lang(PROFILE_BUF, 0xa84d38, pp, np)), // "  Out for <period> with <injury>"
        }
    } else {
        let p = potential(inj, s);
        if p != 0 {
            let mut name = [0u8; 0x100];
            let np = name.as_mut_ptr() as usize;
            let g = category_name(inj, read_u8(p + 4) as u32, np, 0xc8) as i32;
            lang(PROFILE_BUF, 0xa84d00, g, g, np); // "  Potential {}<category>{}"
        } else {
            lang(PROFILE_BUF, 0xa84cdc, 0xad9c64usize); // "  None"
        }
    }
    profile_cell(area, 1, row as usize, 2, 0, 1, 2, yellow, PROFILE_BUF);
    row += 1;
    lang(PROFILE_BUF, 0xa84cb8, 0xad9c64usize); // "  Type"
    profile_cell(area, 0, row as usize, 2, 0, 1, 2, pale, PROFILE_BUF);
    let mut type_text = 0x98e8c8usize; // "  -"
    let mut wrapped = false;
    if status != 0 {
        let injury = exe::injury_get_player_injury(inj, s);
        if read_u8(injury + 0x1a) == 0xff {
            if exe::injury_get_type_category(inj, read_i8(injury)) != 0 && exe::injury_get_type_category(inj, read_i8(injury)) != 0xc {
                lang(PROFILE_BUF, 0xa84c9c); // "  Injured during training"
                type_text = PROFILE_BUF;
            }
        } else {
            injury_cause(inj, injury, cp as usize);
            profile_cell(area, 1, row as usize, 2, 0, 1, 2, yellow, 0);
            draw(cp as usize, area, 0x102, 0x30c, &mut row, 2, yellow, 2, 0);
            wrapped = true;
        }
    }
    if !wrapped {
        profile_cell(area, 1, row as usize, 2, 0, 1, 2, yellow, type_text);
    }
    row += 1;
    lang(PROFILE_BUF, 0xa84c90); // "  Condition"
    profile_cell(area, 0, row as usize, 2, 0, 1, 2, pale, PROFILE_BUF);
    if status == 2 {
        profile_strcpy(PROFILE_BUF, 0x98e8c8); // "  -"
    } else {
        let id = read_u32(s);
        if exe::injury_get_condition(inj, s, 1) > 0x5f && exe::injury_get_fatigue(inj, id) >= -750 {
            format(PROFILE_BUF, 0xa84c08, exe::injury_get_condition(inj, s, 1) as i32); // "  %d%%"
        } else {
            let jaded = exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), read_usize(s + staff::PLAYER), 6, 1, 6.5) as i32 as i16;
            if exe::injury_get_fatigue(inj, id) < -750 && jaded < 0xa {
                lang(PROFILE_BUF, 0xa84c60, exe::injury_get_condition(inj, s, 1) as i32); // lacking match practice
            } else if exe::injury_get_fatigue(inj, id) > 0x44c && jaded > 0x1e {
                lang(PROFILE_BUF, 0xa84c34, exe::injury_get_condition(inj, s, 1) as i32); // in need of a rest
            } else if exe::injury_get_fatigue(inj, id) > 0x4c9 {
                lang(PROFILE_BUF, 0xa84c10, exe::injury_get_condition(inj, s, 1) as i32); // exhausted
            } else {
                format(PROFILE_BUF, 0xa84c08, exe::injury_get_condition(inj, s, 1) as i32); // "  %d%%"
            }
        }
    }
    profile_cell(area, 1, row as usize, 2, 0, 1, 2, yellow, PROFILE_BUF);
    row += 1;
    lang(PROFILE_BUF, 0xa84bfc); // "  Training"
    profile_cell(area, 0, row as usize, 2, 0, 1, 2, pale, PROFILE_BUF);
    let awol = read_usize(G_AWOL);
    let missed = awol != 0 && missed_training(awol, s) != 0;
    match (status, missed) {
        (0, true) => drop(lang(PROFILE_BUF, 0xa84bb4, 0xad9c64usize)), // "  Full (absent from training recently)"
        (1, true) => drop(lang(PROFILE_BUF, 0xa84b8c)),                // "  Light (absent from training recently)"
        (0, false) => drop(lang(PROFILE_BUF, 0xa84b64, 0xad9c64usize)), // "  Full"
        (1, false) => drop(lang(PROFILE_BUF, 0xa84b5c)),               // "  Light"
        _ => drop(lang(PROFILE_BUF, 0xa84b38, 0xad9c64usize)),          // "  None"
    }
    profile_cell(area, 1, row as usize, 2, 0, 1, 2, yellow, PROFILE_BUF);
    row += 1;
    lang(PROFILE_BUF, 0xa84b30); // "  Bans"
    profile_cell(area, 0, row as usize, 2, 0, 1, 2, pale, PROFILE_BUF);
    let first = row;
    let mut ban = [0u8; 0x100];
    let bp = ban.as_mut_ptr() as usize;
    for k in 0..4u32 {
        if ban_text(read_usize(G_DISCIPLINE), s, k, bp, 0xc8) != 0 {
            if row > first {
                profile_cell(area, 0, row as usize, 2, 0, 1, 2, pale, 0);
            }
            format(PROFILE_BUF, 0x987ff0, bp); // "  %s"
            profile_cell(area, 1, row as usize, 2, 0, 1, 2, yellow, PROFILE_BUF);
            row += 1;
        }
    }
    if row == first {
        lang(PROFILE_BUF, 0xa84b10, 0xad9c64usize); // "  None"
        profile_cell(area, 1, row as usize, 2, 0, 1, 2, yellow, PROFILE_BUF);
        row += 1;
    }
    while row < rows as i32 {
        profile_cell(area, 0, row as usize, 2, 0, 0xc, 2, pale, 0);
        profile_cell(area, 1, row as usize, 2, 0, 0xc, 2, pale, 0);
        row += 1;
    }
    exe::drop_text(cp);
}

/// Builds the staff profile screen of the person in param 0 on the tab of param 1: the setup and the creations
/// (`profile_screen_setup`), the heading (`profile_screen_heading`; no nation: the error box, line 0x308, and nothing
/// more), the Action menu (`profile_screen_action_menu`) and the compare entries, the tab bar (`profile_screen_tabs`),
/// the identity block (`profile_screen_identity`), the statistics table of a player of the database
/// (`profile_screen_statistics`), then the tab: 1 profile (0x87a510), 2 injuries and bans
/// (`profile_screen_injuries_tab`), 3 contract (0x87cd90), 4 transfer (0x87dd80), 5 history (0x885820), another tab
/// the error box (line 0x7de); the squad and history menu (0x88b910) and the Back/Next buttons.
///
/// Convention: cdecl, SEH frame (0x1d10 bytes of locals): a Text and 25 `MenuItem` for the tab bar. Split into named
/// pieces; the whole routine is hooked.
/// Original: `staff_screens_build_profile_screen` (0x870700, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_build_profile_screen() {
    let construct_vector: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(FN_CRT_VECTOR_CONSTRUCTOR_ITERATOR);
    let destroy_vector: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_VECTOR_DESTRUCTOR_ITERATOR);
    let tab_body: unsafe extern "C" fn(i32, i32) -> u32 = core::mem::transmute(0x87a510usize);
    let squad_menu: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x88b910usize);
    let mut st = ProfileScreen {
        staff: 0,
        tab: 0,
        view: 0,
        list: 0,
        mode: 0,
        flags: 0,
        history: 0,
        history_index: 0,
        history_row: 0,
        compare_columns: 0,
        compare1: 0,
        compare2: 0,
        viewer: 0,
        viewer_club: 0,
        viewer_nation_team: 0,
        contract: 0,
        offer: 0,
        fg: 0,
        bg: 0,
        y: 0,
        bottom: 0,
        text: [0; 2],
        name: [0; 0xc8],
        action: [0; 0x100],
    };
    exe::new_text(st.text.as_mut_ptr(), 0);
    let mut items = [0u8; PROFILE_TAB_COUNT * PROFILE_TAB_ITEM];
    let ip = items.as_mut_ptr() as usize;
    construct_vector(ip, PROFILE_TAB_ITEM as u32, PROFILE_TAB_COUNT as u32, 0x4568f0, 0x5a53c0);
    let align: [u8; 12] = [6, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2];
    covered!("tel07_3 build the staff profile");
    profile_screen_setup(&mut st);
    if !profile_screen_heading(&mut st) {
        show_error_at(0xa84aac, 0x308);
        destroy_vector(ip, PROFILE_TAB_ITEM as u32, PROFILE_TAB_COUNT as u32, 0x5a53c0);
        exe::drop_text(st.text.as_mut_ptr());
        return;
    }
    let menu = profile_screen_action_menu(&mut st);
    profile_screen_compare_entries(&mut st, menu);
    profile_screen_tabs(&mut st, ip);
    profile_screen_identity(&mut st);
    let s = st.staff;
    if read_usize(s + staff::PLAYER) != 0 && (s == 0 || read_i32(s) < read_i32(G_STAFF_COUNT)) {
        profile_screen_statistics(&mut st, align.as_ptr() as usize);
    }
    let (y, bottom) = (st.y, st.bottom);
    match st.tab as i32 {
        1 => drop(tab_body(y, bottom)),
        2 => profile_screen_injuries_tab(&mut st),
        3 => drop(core::mem::transmute::<usize, unsafe extern "C" fn(i32, i32) -> u32>(0x87cd90)(y, bottom)),
        4 => drop(core::mem::transmute::<usize, unsafe extern "C" fn(i32, i32) -> u32>(0x87dd80)(y, bottom)),
        5 => drop(core::mem::transmute::<usize, unsafe extern "C" fn(i32, i32) -> u32>(0x885820)(y, bottom)),
        _ => {
            covered!("tel07_3 profile on an unknown tab");
            show_error_at(0xa84aac, 0x7de);
        }
    }
    // simplification: the original pushes the view byte in a dword whose high bytes are stack garbage; here zero-extended
    squad_menu(s, st.view as u32, st.bg, st.fg, 1, 0x1e);
    let next = (exe::screenmanager_is_forward_unavailable(SCREEN_MANAGER) == 0) as u32;
    let back = (exe::screenmanager_is_back_unavailable(SCREEN_MANAGER, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(back, next);
    destroy_vector(ip, PROFILE_TAB_ITEM as u32, PROFILE_TAB_COUNT as u32, 0x5a53c0);
    exe::drop_text(st.text.as_mut_ptr());
}

/// What the profile builder will create or change in the game, read before it runs (the same rules as
/// `profile_screen_setup`, written apart to check the original afterwards): the offer list (param 3 null), a staff
/// contract (none, a non-player sheet, in the database, a club and a contract job), the career history (tab 5, bit 2
/// of param 5 after its masks, no param 6, a player sheet) and param 5 after its masks.
struct ProfilePlan {
    staff: usize,
    tab: u8,
    list: bool,
    contract: bool,
    history: bool,
    flags: u32,
}

unsafe fn profile_screen_plan() -> ProfilePlan {
    let contract_job: unsafe extern "stdcall" fn(u32) -> u32 = core::mem::transmute(0x4cbb20usize);
    let s = get_screen_param(0);
    let mut flags = get_screen_param(5) as u32;
    if read_usize(s + staff::PLAYER) == 0 {
        flags &= 0xfffffff1;
        if flags == 0 {
            flags = 1;
        }
    }
    if exe::staff_records_get_manager_record(read_usize(G_STAFF_RECORDS), s, 0) == 0 {
        flags &= 0xffffffef;
        if flags == 0 {
            flags = 1;
        }
    }
    let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), s) == 0
        && read_usize(s + staff::NON_PLAYER) != 0
        && (s == 0 || read_i32(s) < read_i32(G_STAFF_COUNT))
        && read_usize(s + staff::CLUB_JOB) != 0
        && contract_job(read_u8(s + staff::JOB_FOR_CLUB) as u32) != 0;
    let tab = get_screen_param(1) as u8;
    let history = tab == 5 && flags & 2 != 0 && get_screen_param(6) == 0 && read_usize(s + staff::PLAYER) != 0;
    ProfilePlan { staff: s, tab, list: get_screen_param(3) == 0, contract, history, flags }
}

/// Comparer for `staff_screens_build_profile_screen`: the builder creates game objects (the offer list, a staff
/// contract signed with the contract manager, the career history) and does not run twice, so only the original
/// builds for the game and the result is checked afterwards. Before it runs, the plan (`profile_screen_plan`) says what
/// it will create; afterwards the creations are checked against the plan (param 3 set, the person's contract of type 1
/// with his id, param 6 set, param 5 masked), and the copy builds its display list in a scratch: the screen (the
/// screen manager, the Display, the text buffer, the generator) goes back to before the original, the game keeps what
/// the original created (so the copy finds the list, the contract and the history instead of creating them), and the
/// copy's display list, params and writes are compared with the original's; then the original's screen comes back.
pub(crate) unsafe extern "C" fn compare_staff_screens_build_profile_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    let plan = profile_screen_plan();
    let s = plan.staff;
    let mut screen = screen_build_regions();
    screen.push((PROFILE_BUF, 0x100));
    screen.push((0xae290c, 4));
    screen.push((PROFILE_SCREEN_RESET, 4));
    screen.push((G_CONTRACT_LOOKUP_RESULT, 4));
    screen.extend_from_slice(&crate::rng::STATE_REGIONS);
    let before = crate::comparison::MemorySnapshot::read(&screen);
    let (p3_before, p6_before) = (get_screen_param(3), get_screen_param(6));
    o();
    let mut problems: Vec<String> = Vec::new();
    let p3 = get_screen_param(3);
    if (plan.list && p3 == 0) || (!plan.list && p3 != p3_before) {
        problems.push(format!("offer list: plan {} x param 3 {:#x} -> {:#x}", plan.list, p3_before, p3));
    }
    if plan.contract {
        let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), s);
        if c == 0 || read_i32(c) != read_i32(s) || read_u8(c + contract::CONTRACT_TYPE) & 0x3f != 1 {
            problems.push(format!("staff contract planned, the original signed {:#x}", c));
        }
    }
    let p6 = get_screen_param(6);
    if (plan.history && p6 == 0) || (!plan.history && p6 != p6_before) {
        problems.push(format!("career history: plan {} x param 6 {:#x} -> {:#x}", plan.history, p6_before, p6));
    }
    if get_screen_param(5) as u32 != plan.flags {
        problems.push(format!("param 5 {:#x} x plan {:#x}", get_screen_param(5), plan.flags));
    }
    let mut all = screen.clone();
    let slot = read_i16(SCREEN_MANAGER + screen_manager::CURRENT_SLOT) as isize;
    let block = core::ptr::read_unaligned((SCREEN_MANAGER as isize + (screen_manager::PARAM_BLOCK as isize) + slot * 0x300) as *const usize);
    if block != 0 {
        all.push((block, 0xb4));
        all.push((block + 0xb8, 0x200 - 0xb8));
    }
    if s != 0 {
        all.push((s + staff::PLAYING_SQUAD, 1));
        if read_i32(s) >= 0 && read_i32(s) < read_i32(G_STAFF_COUNT) {
            all.push((profile_state(s) + staff_state::EXTRA_FLAGS, 1));
        }
    }
    let after_orig = crate::comparison::MemorySnapshot::read(&all);
    before.write_back();
    crate::rng::pause_trace(|| crate::rng::without_counting(|| staff_screens_build_profile_screen()));
    let after_rust = crate::comparison::MemorySnapshot::read(&all);
    after_orig.write_back();
    let equal = after_orig == after_rust;
    SCOREBOARD.record(equal && problems.is_empty(), || {
        format!("staff profile screen ({:#x}, tab {}): plan {:?}; differences in {:?}", s, plan.tab, problems, first_differences(&after_orig, &after_rust))
    });
}
