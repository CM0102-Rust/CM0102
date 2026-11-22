// ---------- batch %%LEVA%%: the transfer list, release and trial dialogs (transfer_screens.cpp, part 2: tel18_4) ----------

/// The bar No (`no`) / Yes (`yes`) of a dialog, columns weighted 3:1.
pub(crate) unsafe fn ts_no_yes_bar(no: u32, yes: u32) {
    let weights: [u8; 2] = [3, 1];
    let bar = ts_button_bar(2, weights.as_ptr());
    ts_bar_button(bar, 0, TS_NO, no);
    ts_bar_button(bar, 1, TS_YES, yes);
}

/// The contract club of `player` (`club_at(contract +4)`, no contract: 0), stored in `G_CONTRACT_LOOKUP_RESULT` like
/// the original's inline lookup (a contract of type 5 counts).
pub(crate) unsafe fn ts_contract_club_any(player: usize) -> usize {
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    if c != 0 { club_at(read_i32(c + 4) as isize).addr() } else { 0 }
}

/// The title of a dialog about a player: the kit colours of his contract club (looked up), then the name of his
/// contract club looked up again (`+4` of a missing club as in the original).
pub(crate) unsafe fn ts_title_in_contract_club(player: usize) {
    let mut a = read_u16(G_GUI_COLOUR_BLUE) as u32;
    let mut b = read_u16(G_GUI_COLOUR_PALE_GREY) as u32;
    exe::database_get_club_kit_colours(ts_contract_club_any(player), 1, &mut a, &mut b);
    let club = ts_contract_club_any(player);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, b, a, club + club::NAME);
}

/// The text area of a dialog with the player's name as a link (0x7d7) and the given trailing link triples.
unsafe fn ts_dialog_text_with_player(name: usize, player: usize) {
    let text_area: TsTextAreaLinks = core::mem::transmute(0x5ed070usize);
    let lines = ts_dialog_lines();
    text_area(0x6e, 0x8c, 0x30c, 0x221, 2, 0, lines as i8 as i32 as u32, 3, ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT, name, 0x7d7u32, player, 0u32);
}

/// Opens the dialog that puts a player on, or takes him off, the transfer or loan list (build
/// `transfer_screens_build_transfer_list_dialog`, input `transfer_screens_handle_transfer_list_dialog`, flag 1): params
/// 0 the player, 1 the loan list.
/// Original: `transfer_screens_open_transfer_list_dialog` (0x8f8120, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_transfer_list_dialog(player: usize, loan: u32) {
    covered!("tel18_4 open transfer list dialog");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8f8170usize, 0x8f8450usize, 1, 0usize, 0) != 0 {
        set_screen_param(0, player);
        set_screen_param(1, loan as usize);
    }
}

/// Comparer of `transfer_screens_open_transfer_list_dialog`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_transfer_list_dialog(player: usize, loan: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32)>(%%I0%%)(player, loan)
}

/// Builds the transfer list dialog: whether the player (param 0) is already on the list (the loan list with param 1,
/// `staff_is_loan_listed`, else `staff_is_transfer_listed`); his picture, the title in his contract club's colours, the
/// subtitle (name style 5) "Remove <player> From List?", "Transfer List <player> For Loan?" or "Transfer List
/// <player>?" and the question (remove from or put on the loan or transfer list) with his name as a link; the menu bar
/// and the bar No (1) / Yes (2).
/// Original: `transfer_screens_build_transfer_list_dialog` (0x8f8170, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_build_transfer_list_dialog() {
    covered!("tel18_4 transfer list dialog");
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let player = get_screen_param(0);
    let loan = get_screen_param(1) != 0;
    let listed = if loan { exe::staff_is_loan_listed(player, 0usize) } else { exe::staff_is_transfer_listed(player, 0usize) } != 0;
    picture(player, 0, 0);
    ts_title_in_contract_club(player);
    let mut name = [0u8; 0x190];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(player, n, 0x190, 5);
    lang(TS_TEXT, if listed { 0xaa0fe8 } else if loan { 0xaa0fac } else { 0xaa0f78 }, n);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    exe::database_format_staff_name(player, n, 0x190, 1);
    let question = match (listed, loan) {
        (true, true) => 0xaa0f1c,
        (true, false) => 0xaa0ebc,
        (false, true) => 0xaa0e60,
        (false, false) => 0xaa0e08,
    };
    lang(TS_TEXT, question, n);
    ts_dialog_text_with_player(n, player);
    exe::menubar_build(4, 0);
    ts_no_yes_bar(1, 2);
}

/// Comparer of `transfer_screens_build_transfer_list_dialog`: side by side over the display list and the contract
/// lookup global.
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_transfer_list_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    compare_by_display_list_with("Transfer list dialog", &[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(), || transfer_screens_build_transfer_list_dialog());
}

/// Answers the transfer list dialog: No (id 1) -10. Yes (id 2): the loan list (param 1) toggles the player on or off
/// (`remove_from_loan_list`/`add_to_loan_list`); the transfer list puts him on (`transfer_list_player(player, 1)`) or
/// takes him off (`remove_from_transfer_list`), and then, when his contract with his club has no grievance 0x10000,
/// he wants to leave (`STAFF_CONTRACT::wants_to_leave`) and his loyalty is under 15, his morale drops by 50 (kept
/// within -100..100) and the grievance 0x10000 is raised (`apply_event_flags(0x10000, 0)`); -10. The rest goes to the
/// menu bar.
/// Original: `transfer_screens_handle_transfer_list_dialog` (0x8f8450, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_handle_transfer_list_dialog(item: u32) -> i32 {
    let id = tel26_item_id(item as i16);
    if id == 1 {
        return -10;
    }
    if id != 2 {
        return ts_menubar_answer(item);
    }
    let player = get_screen_param(0);
    let sm = || read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856);
    if get_screen_param(1) != 0 {
        covered!("tel18_4 loan list Yes");
        if exe::staff_is_loan_listed(player, 0usize) != 0 {
            let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x8442d0usize);
            f(sm(), player);
        } else {
            let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8442b0usize);
            f(sm(), player);
        }
        return -10;
    }
    if exe::staff_is_transfer_listed(player, 0usize) == 0 {
        covered!("tel18_4 transfer list Yes: listed");
        let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x842320usize);
        f(sm(), player, 1);
        return -10;
    }
    covered!("tel18_4 transfer list Yes: removed");
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x844280usize);
    f(sm(), player);
    let c = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, read_usize(player + staff::CLUB_JOB));
    if read_u32(c + contract::GRIEVANCES) & 0x10000 == 0 && exe::staff_contract_wants_to_leave(c) != 0 && read_i8(player + staff::LOYALTY) < 0xf {
        let morale = read_u8(c + contract::MORALE);
        let x = morale as i8 as i32 - 0x32;
        let v = if x > 100 { 0x64 } else if x < -100 { 0x9c } else { morale.wrapping_sub(0x32) };
        write_u8(c + contract::MORALE, v);
        exe::staff_contract_apply_event_flags(c, 0x10000, 0);
    }
    -10
}

/// Comparer of `transfer_screens_handle_transfer_list_dialog`: No side by side; Yes (the lists and the contract) and the
/// menu bar only in the original.
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_transfer_list_dialog(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%);
    ts_compare_answer("Transfer list dialog", item, &[1], o, transfer_screens_handle_transfer_list_dialog)
}

/// Opens the dialog that releases a player (build `transfer_screens_build_release_dialog`, input
/// `transfer_screens_handle_release_dialog`, flag 1): param 0 the player.
/// Original: `transfer_screens_open_release_dialog` (0x8f8630, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_release_dialog(player: usize) {
    covered!("tel18_4 open release dialog");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8f8670usize, 0x8f8b00usize, 1, 0usize, 0) != 0 {
        set_screen_param(0, player);
    }
}

/// Comparer of `transfer_screens_open_release_dialog`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_release_dialog(player: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I3%%)(player)
}

/// Builds the release dialog.
///
/// The player of param 0: his picture, the title in his contract club's colours, the subtitle "Release <player>?". The
/// pay-off (param 1): with a contract of type 2, his wage (`contract::WAGE`) times the days to its end
/// (`STAFF_CONTRACT::days_to_end_date`, none when not positive) / 7, a quarter of it when he wants to leave
/// (`wants_to_leave`); else 0. When its display amount (mode 4) is positive, "Are you sure you want to release <player>
/// ... compensation of <money>" with his contract club (looked up three times), else "... on a free transfer". The
/// text area with the player's and the club's links (the club looked up twice more), the menu bar and the bar No (1) /
/// Yes (2).
/// Original: `transfer_screens_build_release_dialog` (0x8f8670, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_build_release_dialog() {
    covered!("tel18_4 release dialog");
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let text_area: TsTextAreaLinks = core::mem::transmute(0x5ed070usize);
    let player = get_screen_param(0);
    picture(player, 0, 0);
    ts_title_in_contract_club(player);
    let mut name = [0u8; 0x190];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(player, n, 0x190, 1);
    lang(TS_TEXT, 0xaa1140, n);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    exe::database_format_staff_name(player, n, 0x190, 1);
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
    let mut pay: i32 = 0;
    if c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 2 {
        let days = exe::staff_contract_days_to_end_date(c);
        pay = if days > 0 { read_i32(c + contract::WAGE).wrapping_mul(days as i32) / 7 } else { 0 };
        if exe::staff_contract_wants_to_leave(c) != 0 {
            pay /= 4;
        }
    }
    set_screen_param(1, pay as usize);
    if ts_display(pay as u32, 4) > 0 {
        let mut money = [0u8; 0x64];
        let m = money.as_mut_ptr() as usize;
        exe::cash_format_money(m, 0x64, ts_display(pay as u32, 6), 6);
        let (c1, c2, c3) = (ts_contract_club_any(player), ts_contract_club_any(player), ts_contract_club_any(player));
        let g = |c: usize| read_i8(c + club::SHORT_GENDER_NAME) as i32;
        lang(TS_TEXT, 0xaa107c, g(c3), g(c2), n, c1 + club::SHORT_NAME, m);
    } else {
        lang(TS_TEXT, 0xaa1020, n);
    }
    let lines = ts_dialog_lines();
    let (c1, c2) = (ts_contract_club_any(player), ts_contract_club_any(player));
    text_area(0x6e, 0x8c, 0x30c, 0x221, 2, 0, lines as i8 as i32 as u32, 3, ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT,
        n, 0x7d7u32, player, c2 + club::SHORT_NAME, 0x7d5u32, c1, 0u32);
    exe::menubar_build(4, 0);
    ts_no_yes_bar(1, 2);
}

/// Comparer of `transfer_screens_build_release_dialog`: side by side over the display list, the params and the contract
/// lookup global.
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_release_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I4%%);
    compare_by_display_list_with("Release dialog", &[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(), || transfer_screens_build_release_dialog());
}

/// Answers the release dialog: No (id 1) -10; Yes (id 2) queues the player's free transfer with the pay-off of param 1
/// (`queue_free_transfer(player, 0, pay-off)`), -10; the rest goes to the menu bar.
/// Original: `transfer_screens_handle_release_dialog` (0x8f8b00, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_handle_release_dialog(item: u32) -> i32 {
    let id = tel26_item_id(item as i16);
    if id == 1 {
        return -10;
    }
    if id == 2 {
        covered!("tel18_4 release Yes");
        let player = get_screen_param(0);
        let pay = get_screen_param(1) as u32;
        exe::transfer_manager_queue_free_transfer(read_usize(G_TRANSFER_MANAGER), player, 0, pay);
        return -10;
    }
    ts_menubar_answer(item)
}

/// Comparer of `transfer_screens_handle_release_dialog`: No side by side; Yes (the free transfer) and the menu bar only
/// in the original.
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_release_dialog(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I5%%);
    ts_compare_answer("Release dialog", item, &[1], o, transfer_screens_handle_release_dialog)
}

/// Opens the dialog that offers a player a trial (build `transfer_screens_build_offer_trial_dialog`, input
/// `transfer_screens_handle_offer_trial_dialog`, flag 1): param 0 the player, param 1 two weeks.
/// Original: `transfer_screens_open_offer_trial_dialog` (0x8f8be0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_offer_trial_dialog(player: usize) {
    covered!("tel18_4 open offer trial dialog");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8f8c30usize, 0x8f9080usize, 1, 0usize, 0) != 0 {
        set_screen_param(0, player);
        set_screen_param(1, 2);
    }
}

/// Comparer of `transfer_screens_open_offer_trial_dialog`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_offer_trial_dialog(player: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I6%%)(player)
}

/// The weeks text of the trial dialog: "1 week" or "<n> weeks".
unsafe fn ts_trial_weeks_text(weeks: i8) {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if weeks == 1 {
        lang(TS_TEXT, 0xa7a0ec);
    } else {
        lang(TS_TEXT, 0xaa1170, weeks as i32);
    }
}

/// Builds the offer trial dialog.
///
/// The player (param 0) and the weeks (param 1). Without the player (line 0x21a6), the slot's manager (0x21b4) or his
/// club (0x21bb): the assert box. The player's picture, the title (the club's short name) in its kit colours, the
/// subtitle "Offer trial to <player>" (name style 5); the table (7 columns 2:0x14:2:0xa:2:2:2) with "Trial Period",
/// "-" (id 1, greyed at 1 week; param 2 gets its item), the weeks and "+" (id 2, greyed at 4 weeks). The menu bar and
/// the bar Back (id 3) / Offer (id 4), coloured with the kit's second colour.
/// Original: `transfer_screens_build_offer_trial_dialog` (0x8f8c30, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_build_offer_trial_dialog() {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let weights: [u8; 7] = [2, 0x14, 2, 0xa, 2, 2, 2];
    let player = get_screen_param(0);
    let weeks = get_screen_param(1) as u8 as i8;
    if player == 0 {
        tel26_error_box(TS_CPP, 0x21a6);
        return;
    }
    let manager = tel26_slot_manager().addr();
    if manager == 0 {
        tel26_error_box(TS_CPP, 0x21b4);
        return;
    }
    let club = read_usize(manager + staff::CLUB_JOB);
    if club == 0 {
        tel26_error_box(TS_CPP, 0x21bb);
        return;
    }
    covered!("tel18_4 offer trial dialog");
    picture(player, 0, 0);
    let (mut a, mut b) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut a, &mut b);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, b, a, club + club::SHORT_NAME);
    let mut name = [0u8; 0x190];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(player, n, 0x190, 5);
    lang(TS_TEXT, 0xaa119c, n);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    let lines = ts_dialog_lines();
    let d = current_human_display();
    let area = tel26_add_area(d, [0x6e, 0x8c, 0x30c, 0x221, 7, weights.as_ptr() as u32, lines as u32, 0, 2, 0, u32::MAX]);
    let pale = ts_colour(G_GUI_COLOUR_PALE_GREY);
    let grey = ts_colour(G_GUI_COLOUR_GREY);
    let dark = ts_colour(G_GUI_COLOUR_DARK_BLUE);
    lang(TS_TEXT, 0xaa118c); // "Trial Period"
    tel26_area_button([area, 1, 0, 1, 0, 1, 3, pale, TS_TEXT as u32, 0, 0]);
    let minus = if weeks == 1 {
        tel26_area_text([area, 2, 0, 0x30, dark, 0xc, 3, grey, TS_MINUS as u32])
    } else {
        tel26_area_button([area, 2, 0, 0x30, dark, 0xc, 3, pale, TS_MINUS as u32, 1, 0])
    };
    set_screen_param(2, minus as i16 as i32 as usize);
    ts_trial_weeks_text(weeks);
    tel26_area_text([area, 3, 0, 1, ts_colour(TS_COLOUR_POPUP), 0xc, 3, pale, TS_TEXT as u32]);
    if weeks == 4 {
        tel26_area_text([area, 4, 0, 0x30, dark, 0xc, 3, grey, TS_PLUS as u32]);
    } else {
        tel26_area_button([area, 4, 0, 0x30, dark, 0xc, 3, pale, TS_PLUS as u32, 2, 0]);
    }
    exe::menubar_build(4, 0);
    let bar_weights: [u8; 2] = [3, 1];
    let bar = tel26_add_area(d, [100, 0x22b, 0x316, 0x24e, 2, bar_weights.as_ptr() as u32, 1, 0, 1, b, u32::MAX]);
    ts_bar_button(bar, 0, TS_BACK, 3);
    ts_bar_button(bar, 1, 0x98e6e8, 4); // "Offer"
}

/// Comparer of `transfer_screens_build_offer_trial_dialog`: side by side over the display list and the params; without
/// the player, the manager or his club only the original runs (the assert box).
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_offer_trial_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I7%%);
    let manager = tel26_slot_manager().addr();
    if get_screen_param(0) == 0 || manager == 0 || read_usize(manager + staff::CLUB_JOB) == 0 {
        return o();
    }
    compare_by_display_list_with("Offer trial dialog", &[], || o(), || transfer_screens_build_offer_trial_dialog());
}

/// Answers the offer trial dialog: Back (id 3) -9. Offer (id 4): the confirm trial offer dialog for the player with
/// the weeks (`transfer_screens_open_confirm_trial_offer`; the weeks byte over the item argument's high bytes), -4.
/// "-" (1) and "+" (2): the weeks (param 1) one less or more, and the widgets in place: "-" (param 2) greyed with no id
/// at 1 week (else id 1), the weeks text, "+" greyed with no id at 4 weeks (else id 2); the screen redrawn, 0. The rest
/// goes to the menu bar.
/// Original: `transfer_screens_handle_offer_trial_dialog` (0x8f9080, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_handle_offer_trial_dialog(item: u32) -> i32 {
    let id = tel26_item_id(item as i16);
    if id == 3 {
        return -9;
    }
    if id == 4 {
        covered!("tel18_4 offer trial Offer");
        let confirm: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x8fb530usize);
        let player = get_screen_param(0);
        let weeks = get_screen_param(1) as u8;
        confirm(player, (item & !0xff) | weeks as u32);
        return -4;
    }
    if id == 1 || id == 2 {
        covered!("tel18_4 offer trial weeks");
        let weeks = (get_screen_param(1) as u8).wrapping_add(if id == 1 { 0xff } else { 1 }) as i8;
        set_screen_param(1, weeks as i32 as usize);
        let mut w = get_screen_param(2) as u32;
        let widget = |w: u32, on: bool, on_id: u32| {
            if on {
                exe::screenmanager_set_widget_flags(G_SCREEN_MANAGER, w as usize, 2);
                exe::screenmanager_set_widget_text_colour(G_SCREEN_MANAGER, w, ts_colour(G_GUI_COLOUR_PALE_GREY));
                exe::screenmanager_set_widget_id(G_SCREEN_MANAGER, w, on_id);
            } else {
                exe::screenmanager_set_widget_flags(G_SCREEN_MANAGER, w as usize, 6);
                exe::screenmanager_set_widget_text_colour(G_SCREEN_MANAGER, w, ts_colour(G_GUI_COLOUR_GREY));
                exe::screenmanager_set_widget_id(G_SCREEN_MANAGER, w, 0);
            }
        };
        widget(w, id == 2 || weeks != 1, 1);
        w += 1;
        ts_trial_weeks_text(weeks);
        exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, w, TS_TEXT);
        w += 1;
        widget(w, id == 1 || weeks != 4, 2);
        exe::screenmanager_redraw_screen(G_SCREEN_MANAGER, 0, 0);
        return 0;
    }
    ts_menubar_answer(item)
}

/// Comparer of `transfer_screens_handle_offer_trial_dialog`: Back side by side; Offer (opens a screen), "-"/"+"
/// (widgets and redraw) and the menu bar only in the original.
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_offer_trial_dialog(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I8%%);
    ts_compare_answer("Offer trial dialog", item, &[3], o, transfer_screens_handle_offer_trial_dialog)
}
