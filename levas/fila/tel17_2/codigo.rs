// ---------- batch %%LEVA%%: the transfer bid dialogs: confirm, reject, must accept and cannot sign (transfer_screens.cpp, part 1: tel17_2) ----------

/// The shared text buffer of the screens.
pub(crate) const TS_TEXT: usize = 0xde1f64;
/// transfer_screens.cpp's path, for the assert box.
pub(crate) const TS_CPP: usize = 0xa9e98c;
/// "Back", "No" and "Yes".
pub(crate) const TS_BACK: usize = 0x9880a8;
pub(crate) const TS_NO: usize = 0x98e8cc;
pub(crate) const TS_YES: usize = 0x98e8d0;
/// The game's langlib_sprintf (variadic, cdecl).
pub(crate) type TsLang = unsafe extern "C" fn(usize, usize, ...) -> i32;
/// `TRANSFER_MANAGER::set_player_squad_status(staff, club, value, out, reason, flag_a, flag_b)` (0x8ce0e0, thiscall
/// ret 0x1c). `out` is a scratch slot of the caller's frame.
pub(crate) const TS_FN_SET_PLAYER_SQUAD_STATUS: usize = 0x8ce0e0;
/// `transfer_manager_get_squad_status_for_reason(staff, reason)` (0x8c5800, thiscall on the transfer manager, ret 8):
/// the squad status a rejection reason gives the player (a byte).
pub(crate) const TS_FN_SQUAD_STATUS_FOR_REASON: usize = 0x8c5800;
/// `transfer_manager_get_rejection_reason_text(reason, buffer, size)` (0x8c5690, thiscall on the transfer manager).
pub(crate) const TS_FN_REJECTION_REASON_TEXT: usize = 0x8c5690;
/// `transfer_manager_is_rejection_reason_valid(reason, offer)` (0x8c5560, thiscall on the transfer manager).
pub(crate) const TS_FN_REJECTION_REASON_VALID: usize = 0x8c5560;
/// `TRANSFER_MANAGER::accept_loan_offer(slot, mark)` (0x8c0660, thiscall).
pub(crate) const TS_FN_ACCEPT_LOAN_OFFER: usize = 0x8c0660;
/// `background_set_staff_picture(staff, 0, 0)` (0x41c790, cdecl).
pub(crate) const TS_FN_STAFF_PICTURE: usize = 0x41c790;
/// `Text::draw_wrapped(area, left, right, &row, font, colour, style, flags)` (0x4aeaf0, thiscall ret 0x20) and
/// `Text::clear` (0x4afa40).
pub(crate) const TS_FN_TEXT_DRAW_WRAPPED: usize = 0x4aeaf0;
pub(crate) const TS_FN_TEXT_CLEAR: usize = 0x4afa40;

/// The colour word at `a`, zero-extended (the original pushes it with whatever the register's high half held; the
/// callees only read the word).
pub(crate) unsafe fn ts_colour(a: usize) -> u32 {
    read_u16(a) as u32
}

/// The club of the human manager of the current screen slot (`staff::CLUB_JOB` of the slot's manager).
pub(crate) unsafe fn ts_human_club() -> usize {
    read_usize(tel26_slot_manager().addr() + staff::CLUB_JOB)
}

/// The screen title in a club's kit colours: `database_get_club_kit_colours(club, 1, &a, &b)` (a starts as the blue
/// word, b as the pale grey one) and `add_screen_title(b, a, club name)`.
pub(crate) unsafe fn ts_title_in_club_colours(club: usize) {
    let mut a = read_u16(G_GUI_COLOUR_BLUE) as u32;
    let mut b = read_u16(G_GUI_COLOUR_PALE_GREY) as u32;
    exe::database_get_club_kit_colours(club, 1, &mut a, &mut b);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, b, a, club + club::NAME);
}

/// How many lines of font 3 fit in the dialog's text area (0x195 / height), as the byte the original keeps.
pub(crate) unsafe fn ts_dialog_lines() -> u8 {
    (0x195 / exe::font_get_height(3)) as u8
}

/// The dialog's text (`TS_TEXT`) in the area 0x6e,0x8c..0x30c,0x221, pale grey, font 3.
pub(crate) unsafe fn ts_dialog_text(lines: u8) {
    tel26_text_area([0x6e, 0x8c, 0x30c, 0x221, 2, 0, lines as i8 as i32 as u32, 3, ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT as u32, 0]);
}

/// The single Back button of a dialog (`add_button_item` 100,0x22b..0x316,0x24e, id 1).
pub(crate) unsafe fn ts_back_button() {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    lang(TS_TEXT, TS_BACK);
    tel26_add_button([100, 0x22b, 0x316, 0x24e, 0x30, ts_colour(G_GUI_COLOUR_GREY), 0xc, 3, ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT as u32, 1, 0]);
}

/// The button bar of a dialog: an area 100,0x22b..0x316,0x24e of `columns` columns (weights at `weights`, one row).
pub(crate) unsafe fn ts_button_bar(columns: u32, weights: *const u8) -> u32 {
    tel26_add_area(current_human_display(), [100, 0x22b, 0x316, 0x24e, columns, weights as u32, 1, 0, 1, 0, u32::MAX])
}

/// A button of the dialog's button bar: column `col`, label `label` (langlib), id `id`.
pub(crate) unsafe fn ts_bar_button(area: u32, col: u32, label: usize, id: u32) {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    lang(TS_TEXT, label);
    tel26_area_button([area, col, 0, 0x30, ts_colour(G_GUI_COLOUR_GREY), 0xc, 3, ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT as u32, id, 0]);
}

/// The menu bar answer of a screen: `menubar_handle_command`, else `menubar_handle_object_link`.
pub(crate) unsafe fn ts_menubar_answer(item: u32) -> i32 {
    let r = exe::menubar_handle_command(item);
    if r != 0 { r } else { exe::menubar_handle_object_link(item) }
}

/// Deletes the offer held in `slot` (`TRANSFER_OFFER::~TRANSFER_OFFER` + operator delete) and clears the slot.
pub(crate) unsafe fn ts_delete_offer(slot: usize) {
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
    let o = read_usize(slot);
    if o != 0 {
        destroy(o);
        exe::crt_operator_delete(o);
    }
    write_usize(slot, 0);
}

/// The edited copy of a bid (`edited`) is thrown away once the action is done, unless there is none or it is the live
/// offer's own slot.
pub(crate) unsafe fn ts_drop_edited(edited: usize, live: usize) {
    if edited != 0 && edited != live {
        ts_delete_offer(edited);
    }
}

/// `TRANSFER_MANAGER::set_player_squad_status` after a bid: with the fee changed from the live offer, the edited bid's
/// total value (`get_total_value(1, 0, 0, 1)`) and flags (1, 1); with the same fee, the player's own value
/// (`staff +0x52`, a dword) and flags (0, 1). `reason_dword` is the dword the original pushes (the reason's squad
/// status byte over the high bytes of the item argument).
pub(crate) unsafe fn ts_squad_status_after_bid(staff: usize, edited: usize, live: usize, reason_dword: u32, fee_changed_flag: u32) {
    let set: unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u32, u32, u32, u32) = core::mem::transmute(TS_FN_SET_PLAYER_SQUAD_STATUS);
    let mut scratch = [0u32; 2];
    let club = contract_club_cached(Staff(staff));
    if read_u32(read_usize(edited) + offer::FEE) != read_u32(read_usize(live) + offer::FEE) {
        let value = exe::transfer_offer_get_total_value(read_usize(edited), 1, 0, 0, 1) as u32;
        set(read_usize(G_TRANSFER_MANAGER), staff, club, value, scratch.as_mut_ptr(), reason_dword, fee_changed_flag, 1);
    } else {
        set(read_usize(G_TRANSFER_MANAGER), staff, club, read_u32(staff + 0x52), scratch.as_mut_ptr(), reason_dword, 0, 1);
    }
}

/// Side by side over the screen params when the clicked id is one of `ids` (answers that only return a code or set a
/// param); otherwise only the original runs (it carries out the transfer action or opens a screen).
pub(crate) unsafe fn ts_compare_answer(name: &str, item: u32, ids: &[u16], o: unsafe extern "C" fn(u32) -> i32, r: unsafe extern "C" fn(u32) -> i32) -> i32 {
    let id = tel26_item_id(item as i16);
    if !ids.contains(&id) {
        // simplification: the answer writes the offer or opens a screen and does not run twice
        return o(item);
    }
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&screen_param_regions(), || o(item), || r(item));
    SCOREBOARD.record(equal && ro == rr, || format!("{} answer to item {}: original={} rust={} differences at {:?}", name, item as i16, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Opens the bid confirmation dialog (build `transfer_screens_build_bid_confirm_dialog`, input
/// `transfer_screens_handle_bid_confirm_action`, flag 1) and, when it opens, sets its params: 0 the edited offer slot,
/// 1 the action (1 submit, 2 cancel, 0x1b reject, 0x1c negotiate, 0x1d accept, 0x23 delay), 2 the live offer slot.
/// Original: `transfer_screens_open_bid_confirm_dialog` (0x8ef1e0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_bid_confirm_dialog(edited: usize, action: u32, live: usize) {
    covered!("tel17_2 open bid confirm dialog");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8ef240usize, 0x8ef9f0usize, 1, 0usize, 0) != 0 {
        set_screen_param(0, edited);
        set_screen_param(1, action as usize);
        set_screen_param(2, live);
    }
}

/// Comparer of `transfer_screens_open_bid_confirm_dialog`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_bid_confirm_dialog(edited: usize, action: u32, live: usize) {
    original_fn::<unsafe extern "C" fn(usize, u32, usize)>(%%I0%%)(edited, action, live)
}

/// Builds the Yes/No dialog that confirms a bid action.
///
/// The offer is the one in param 0 (the edited copy) or, without it, in param 2 (the live one); the player is its
/// target (`offer::TARGET_STAFF_ID`), whose picture is the background. Without the current slot's human manager: the
/// transfer_screens.cpp assert box, line 0xd42. The title is in the human's club colours; the subtitle "Transfer offer
/// for <player>" (or "Loan offer for <player>" when `offer::OFFER_TYPE` is set). The text, by the action in param 1:
/// - 1 (submit): with the completion date (`transfer_manager_get_offer_completion_date`, long date string). By the
///   completion timing (`offer::COMPLETION_TIMING` low nibble): 1, "Submit transfer bid ... at the end of his contract
///   at <club> on <date>"; 2, the same with the season end and the club named twice; otherwise "... from <club>". Every
///   club name is the player's contract club, looked up again each time (`contract_club_cached`).
/// - 0x23 (delay): "Delay decision over transfer bid ..." with the contract club; 2 (cancel), 0x1c (negotiate): the
///   player alone; 0x1d (accept): a loan, "Accept <buyer> loan offer for <player>", else "Accept transfer bid for
///   <player>"; 0x1b (reject): "Reject <buyer> loan offer for <player>". Any other action leaves the text as it was.
/// Then the text area (as many lines as fit), the menu bar (4, 0) and the bar No (id 3) / Yes (id 2).
/// Original: `transfer_screens_build_bid_confirm_dialog` (0x8ef240, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_build_bid_confirm_dialog() {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let mut slot = get_screen_param(0);
    if slot == 0 {
        slot = get_screen_param(2);
    }
    let o = read_usize(slot);
    let player = staff_at(read_i32(o + offer::TARGET_STAFF_ID) as isize).addr();
    let action = get_screen_param(1) as u32;
    picture(player, 0, 0);
    if tel26_slot_manager().addr() == 0 {
        covered!("tel17_2 bid confirm dialog without a manager");
        tel26_error_box(TS_CPP, 0xd42);
        return;
    }
    covered!("tel17_2 bid confirm dialog");
    ts_title_in_club_colours(ts_human_club());
    let mut name = [0u8; 300];
    exe::database_format_staff_name(player, name.as_mut_ptr() as usize, 300, 1);
    let n = name.as_ptr() as usize;
    lang(TS_TEXT, if read_u8(o + offer::OFFER_TYPE) == 0 { 0xa9ee00 } else { 0xa9edcc }, n);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    let lines = ts_dialog_lines();
    let g = |c: usize| read_i8(c + club::SHORT_GENDER_NAME) as i32;
    let cc = || contract_club_cached(Staff(player));
    if action == 1 {
        let mut date = [0u32; 2];
        let mut when = [0u8; 200];
        exe::date_date_54a110(date.as_mut_ptr() as usize);
        exe::transfer_manager_get_offer_completion_date(read_usize(G_TRANSFER_MANAGER), o, date.as_mut_ptr() as usize);
        exe::date_get_long_date_string(date.as_mut_ptr() as usize, when.as_mut_ptr() as usize, 200, 0xff);
        let w = when.as_ptr() as usize;
        match read_u8(o + offer::COMPLETION_TIMING) & 0xf {
            1 => {
                let (c1, c2, c3) = (cc(), cc(), cc());
                lang(TS_TEXT, 0xa9ed14, g(c3), g(c2), 100, n, c1 + club::SHORT_NAME, w);
            }
            2 => {
                let (c1, c2, c3, c4, c5, c6) = (cc(), cc(), cc(), cc(), cc(), cc());
                lang(TS_TEXT, 0xa9ec1c, g(c6), g(c5), 100, g(c4), g(c3), n, c2 + club::SHORT_NAME, w, c1 + club::SHORT_NAME);
            }
            _ => {
                let (c1, c2, c3) = (cc(), cc(), cc());
                lang(TS_TEXT, 0xa9ebb8, g(c3), g(c2), n, c1 + club::SHORT_NAME);
            }
        }
    } else if action == 0x23 {
        let (c1, c2, c3) = (cc(), cc(), cc());
        lang(TS_TEXT, 0xa9eb44, g(c3), g(c2), n, c1 + club::SHORT_NAME);
    } else if action == 2 {
        lang(TS_TEXT, 0xa9eb04, n);
    } else if action == 0x1c {
        lang(TS_TEXT, 0xa9eac0, n);
    } else if action == 0x1d {
        if read_u8(o + offer::OFFER_TYPE) == 1 {
            let buyer = club_at(read_i32(o + offer::BUYER_CLUB_ID) as isize).addr();
            lang(TS_TEXT, 0xa9ea60, g(buyer), g(buyer), buyer + club::SHORT_NAME, n);
        } else {
            lang(TS_TEXT, 0xa9ea20, n);
        }
    } else if action == 0x1b {
        let buyer = club_at(read_i32(o + offer::BUYER_CLUB_ID) as isize).addr();
        lang(TS_TEXT, 0xa9e9c0, g(buyer), g(buyer), buyer + club::SHORT_NAME, n);
    }
    ts_dialog_text(lines);
    exe::menubar_build(4, 0);
    let weights: [u8; 2] = [3, 1];
    let bar = ts_button_bar(2, weights.as_ptr());
    ts_bar_button(bar, 0, TS_NO, 3);
    ts_bar_button(bar, 1, TS_YES, 2);
}

/// Comparer of `transfer_screens_build_bid_confirm_dialog`: side by side over the display list and the contract
/// lookup global; without the slot's manager only the original runs (the assert box does not run twice).
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_bid_confirm_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    if tel26_slot_manager().addr() == 0 {
        return o();
    }
    compare_by_display_list_with("Bid confirm dialog", &[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(), || transfer_screens_build_bid_confirm_dialog());
}

/// Answers the bid confirmation dialog.
///
/// Yes (id 3) carries out the action of param 1 with the edited offer slot `edited` (param 0, or the live one when it
/// is empty) and the live slot `live` (param 2), then deletes the edited copy (unless there is none or it is the live
/// slot) and answers -10:
/// - 0x23 (delay): `club_rejects_offer(live offer, the human's club, 8)`;
/// - 2 (cancel): the live offer is withdrawn (`withdraw_transfer_offer`) when the human's club is its buyer, else
///   rejected (`reject_transfer_offer`), with "player aware = 2" of the edited copy;
/// - 1 (submit): `add_transfer_offer(edited offer, 0)`;
/// - 0x1b (reject): `reject_loan_offer(live, aware)`;
/// - 0x1c (negotiate): `add_transfer_negotiation(live offer, edited offer, aware)`; with the fee changed, the player's
///   squad status follows the bid (`ts_squad_status_after_bid`);
/// - 0x1d (accept): `accept_loan_offer` for a loan, else `accept_transfer_bid` (live, aware).
/// Any other action, and any other item: No (id 2) answers -9 and the rest go to the menu bar.
/// Original: `transfer_screens_handle_bid_confirm_action` (0x8ef9f0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_handle_bid_confirm_action(item: u32) -> i32 {
    let action = get_screen_param(1) as u32;
    if tel26_item_id(item as i16) == 3 {
        let club = ts_human_club();
        let edited = get_screen_param(0);
        let live = get_screen_param(2);
        let e = if edited != 0 { read_usize(edited) } else { read_usize(live) };
        let tm = read_usize(G_TRANSFER_MANAGER);
        let aware = |o: usize| (read_u8(o + offer::PLAYER_AWARE) == 2) as u32;
        let done = match action {
            0x23 => {
                exe::transfer_manager_club_rejects_offer(tm, read_usize(live), club, 8);
                true
            }
            2 => {
                let a = aware(read_usize(edited));
                if live != 0 {
                    if club != 0 && read_u32(club) == read_u32(read_usize(live) + offer::BUYER_CLUB_ID) {
                        exe::transfer_manager_withdraw_transfer_offer(tm, live as *mut usize, a, 0);
                    } else {
                        exe::transfer_manager_reject_transfer_offer(tm, live as *mut usize, a, 0);
                    }
                }
                true
            }
            1 => {
                exe::transfer_manager_add_transfer_offer(tm, read_usize(edited) as *mut u8, core::ptr::null_mut());
                true
            }
            0x1b => {
                exe::transfer_manager_reject_loan_offer(tm, live, aware(e), 0);
                true
            }
            0x1c => {
                let eo = read_usize(edited);
                let player = staff_at(read_i32(eo + offer::TARGET_STAFF_ID) as isize).addr();
                exe::transfer_manager_add_transfer_negotiation(tm, read_usize(live), eo, aware(eo));
                if read_u32(read_usize(edited) + offer::FEE) != read_u32(read_usize(live) + offer::FEE) {
                    ts_squad_status_after_bid(player, edited, live, u32::MAX, 0);
                }
                true
            }
            0x1d => {
                if read_u8(e + offer::OFFER_TYPE) == 1 {
                    let accept_loan: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(TS_FN_ACCEPT_LOAN_OFFER);
                    accept_loan(tm, live, aware(e));
                } else {
                    exe::transfer_manager_accept_transfer_bid(tm, live, aware(e));
                }
                true
            }
            _ => false,
        };
        if done {
            covered!("tel17_2 bid confirm Yes");
            ts_drop_edited(edited, live);
            return -10;
        }
    }
    if tel26_item_id(item as i16) == 2 {
        return -9;
    }
    ts_menubar_answer(item)
}

/// Comparer of `transfer_screens_handle_bid_confirm_action`: No (id 2) side by side; Yes and the menu bar only in the
/// original (they carry out the transfer action or open screens).
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_bid_confirm_action(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%);
    ts_compare_answer("Bid confirm dialog", item, &[2], o, transfer_screens_handle_bid_confirm_action)
}

/// Builds the dialog that rejects a transfer bid.
///
/// The offer is param 0's (or, without it, param 1's); param 2 holds the chosen reason. Without the slot's manager:
/// the assert box, line 0xeed. Title in the human's club colours, subtitle "Transfer offer for <player>", then a list
/// area (0x6e,0x8c..0x30c,0x221, columns 1:0x14, as many rows as fit) with, wrapped (`Text::draw_wrapped`, x 0x7d..0x307),
/// "Reject transfer bid for <player>", a blank row, "Reason:-", a blank row, and the reasons one per row in column 1
/// (`transfer_manager_get_rejection_reason_text`): 0, 0x200, 0x400 always offered; 0x2000, 0x1000, 0xa and 0x800 only
/// when `transfer_manager_is_rejection_reason_valid` (else grey text). The chosen reason is yellow text, the others
/// buttons (id 1, the reason as value). Then the menu bar and the bids for the player (`get_offers_for_player`): when
/// more than one is still open (status 0), the bar No (3) / Reject All Bids (4, grey text unless the chosen reason is
/// valid for every open bid) / Yes (2); otherwise No (3) / Yes (2).
/// Original: `transfer_screens_build_reject_bid_dialog` (0x8efda0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_build_reject_bid_dialog() {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let reason_text: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(TS_FN_REJECTION_REASON_TEXT);
    let reason_valid: unsafe extern "thiscall" fn(usize, u32, usize) -> u32 = core::mem::transmute(TS_FN_REJECTION_REASON_VALID);
    let draw: unsafe extern "thiscall" fn(usize, u32, i32, i32, *mut i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(TS_FN_TEXT_DRAW_WRAPPED);
    let text_clear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(TS_FN_TEXT_CLEAR);
    let text_new: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_TEXT_TEXT);
    let text_set: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(FN_TEXT_SET_STRING);
    let text_drop: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TEXT_DESTRUCTOR_TEXT);
    let mut list = [0u8; 0x10];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    let mut row: i32 = 0;
    let mut open_bids = 0i32;
    let mut all_valid = true;
    let mut slot = get_screen_param(0);
    let chosen = get_screen_param(2);
    if slot == 0 {
        slot = get_screen_param(1);
    }
    let o = read_usize(slot);
    let player = staff_at(read_i32(o + offer::TARGET_STAFF_ID) as isize).addr();
    picture(player, 0, 0);
    if tel26_slot_manager().addr() == 0 {
        covered!("tel17_2 reject bid dialog without a manager");
        tel26_error_box(TS_CPP, 0xeed);
        llist_drop(l);
        return;
    }
    covered!("tel17_2 reject bid dialog");
    ts_title_in_club_colours(ts_human_club());
    let mut name = [0u8; 300];
    exe::database_format_staff_name(player, name.as_mut_ptr() as usize, 300, 1);
    let n = name.as_ptr() as usize;
    lang(TS_TEXT, 0xa9ee00, n);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    let lines = ts_dialog_lines();
    let columns: [u8; 2] = [1, 0x14];
    let area = tel26_add_area(current_human_display(), [0x6e, 0x8c, 0x30c, 0x221, 2, columns.as_ptr() as u32, lines as u32, 0, 2, 0, u32::MAX]);
    lang(TS_TEXT, 0xa9ee54, n);
    let mut text = [0u32; 8];
    let t = text.as_mut_ptr() as usize;
    text_new(t, TS_TEXT);
    let pale = ts_colour(G_GUI_COLOUR_PALE_GREY);
    draw(t, area, 0x7d, 0x307, &mut row, 3, pale, 0, 0);
    row += 2;
    lang(TS_TEXT, 0xa9ee48); // "Reason:-"
    text_clear(t);
    text_set(t, TS_TEXT);
    draw(t, area, 0x7d, 0x307, &mut row, 3, pale, 0, 0);
    row += 2;
    let tm = read_usize(G_TRANSFER_MANAGER);
    let chosen16 = chosen as u16;
    let mut reason_row = |reason: u32, offered: bool| {
        reason_text(tm, reason, TS_TEXT, 2000);
        let r = row as u32;
        row += 1;
        if !offered {
            tel26_area_text([area, 1, r, 1, 0, 1, 3, ts_colour(G_GUI_COLOUR_GREY), TS_TEXT as u32]);
        } else if chosen16 == reason as u16 {
            tel26_area_text([area, 1, r, 1, 0, 1, 3, ts_colour(G_GUI_COLOUR_YELLOW), TS_TEXT as u32]);
        } else {
            tel26_area_button([area, 1, r, 1, 0, 1, 3, pale, TS_TEXT as u32, 1, reason]);
        }
    };
    reason_row(0, true);
    reason_row(0x200, true);
    reason_row(0x400, true);
    for reason in [0x2000u32, 0x1000, 0xa, 0x800] {
        let valid = reason_valid(tm, reason, o) != 0;
        reason_row(reason, valid);
    }
    exe::menubar_build(4, 0);
    exe::transfer_manager_get_offers_for_player(tm, player, l, 0);
    write_u32(l + llist::CURSOR, read_u32(l + llist::HEAD));
    loop {
        let node = exe::llist_get_next(l);
        if node == 0 {
            break;
        }
        let s = exe::transfer_manager_get_offer_slot(tm, read_i32(node));
        let bid = read_usize(s);
        if read_u8(bid + offer::STATUS) == 0 {
            open_bids += 1;
            if reason_valid(tm, chosen as u32, bid) == 0 {
                all_valid = false;
            }
        }
    }
    if open_bids > 1 {
        let weights: [u8; 3] = [2, 1, 1];
        let bar = ts_button_bar(3, weights.as_ptr());
        ts_bar_button(bar, 0, TS_NO, 3);
        lang(TS_TEXT, 0xa9ee38); // "Reject All Bids"
        if all_valid {
            tel26_area_button([bar, 1, 0, 0x30, ts_colour(G_GUI_COLOUR_GREY), 0xc, 3, pale, TS_TEXT as u32, 4, 0]);
        } else {
            tel26_area_text([bar, 1, 0, 0x30, ts_colour(G_GUI_COLOUR_GREY), 0x2c, 3, pale, TS_TEXT as u32]);
        }
        ts_bar_button(bar, 2, TS_YES, 2);
    } else {
        let weights: [u8; 2] = [3, 1];
        let bar = ts_button_bar(2, weights.as_ptr());
        ts_bar_button(bar, 0, TS_NO, 3);
        ts_bar_button(bar, 1, TS_YES, 2);
    }
    text_drop(t);
    llist_drop(l);
}

/// `LLIST::~LLIST` on a list of the frame.
pub(crate) unsafe fn llist_drop(l: usize) {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    f(l);
}

/// Comparer of `transfer_screens_build_reject_bid_dialog`: side by side over the display list (the bid list and the
/// Text are the frame's own); without the slot's manager only the original runs.
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_reject_bid_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I3%%);
    if tel26_slot_manager().addr() == 0 {
        return o();
    }
    compare_by_display_list_with("Reject bid dialog", &[], || o(), || transfer_screens_build_reject_bid_dialog());
}

/// Answers the reject bid dialog.
///
/// A reason (id 1) goes to param 2 (the item's value) and the dialog is redrawn (-11). Reject All Bids (id 4): the
/// reason's squad status (`transfer_manager_get_squad_status_for_reason`) is set on the player
/// (`ts_squad_status_after_bid`, flags 1, 1 with the fee changed) and every open bid (status 0) for him is rejected
/// with the reason (`reject_transfer_offer(slot, 0, reason)`), the edited copy deleted when it is not the live slot,
/// -10. Yes (id 2): the live bid (param 1) is rejected with the reason ("aware = 2" of the edited copy) and the squad
/// status set the same way (flags 1, 0 with the fee changed), the edited copy deleted, -10. No (id 3) answers -9; the
/// rest goes to the menu bar.
/// Original: `transfer_screens_handle_reject_bid_action` (0x8f06f0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_handle_reject_bid_action(item: u32) -> i32 {
    let status_for_reason: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(TS_FN_SQUAD_STATUS_FOR_REASON);
    let id = tel26_item_id(item as i16);
    let tm = read_usize(G_TRANSFER_MANAGER);
    if id == 1 {
        set_screen_param(2, tel26_item_value(item as i16) as usize);
        return -11;
    }
    if id == 4 {
        covered!("tel17_2 reject all bids");
        let mut list = [0u8; 0x10];
        let l = list.as_mut_ptr() as usize;
        exe::new_llist(l);
        let reason = get_screen_param(2);
        let edited = get_screen_param(0);
        let live = get_screen_param(1);
        let player = staff_at(read_i32(read_usize(edited) + offer::TARGET_STAFF_ID) as isize).addr();
        let r = status_for_reason(tm, player, reason);
        // the original keeps the byte over the low byte of the item argument and pushes the whole dword
        let r_dword = (item & !0xff) | r as u32;
        ts_squad_status_after_bid(player, edited, live, r_dword, 1);
        exe::transfer_manager_get_offers_for_player(tm, player, l, 0);
        write_u32(l + llist::CURSOR, read_u32(l + llist::HEAD));
        loop {
            let node = exe::llist_get_next(l);
            if node == 0 {
                break;
            }
            let s = exe::transfer_manager_get_offer_slot(tm, read_i32(node));
            if read_u8(read_usize(s) + offer::STATUS) == 0 {
                exe::transfer_manager_reject_transfer_offer(tm, s as *mut usize, 0, reason as u32);
            }
        }
        if edited != live {
            ts_delete_offer(edited);
        }
        llist_drop(l);
        return -10;
    }
    if id == 2 {
        covered!("tel17_2 reject bid Yes");
        let edited = get_screen_param(0);
        let live = get_screen_param(1);
        let reason = get_screen_param(2);
        let player = staff_at(read_i32(read_usize(edited) + offer::TARGET_STAFF_ID) as isize).addr();
        let r = status_for_reason(tm, player, reason);
        let r_dword = (item & !0xff) | r as u32;
        let aware = (read_u8(read_usize(edited) + offer::PLAYER_AWARE) == 2) as u32;
        exe::transfer_manager_reject_transfer_offer(tm, live as *mut usize, aware, reason as u32);
        ts_squad_status_after_bid(player, edited, live, r_dword, 0);
        ts_drop_edited(edited, live);
        return -10;
    }
    if id == 3 {
        return -9;
    }
    ts_menubar_answer(item)
}

/// Comparer of `transfer_screens_handle_reject_bid_action`: a reason (id 1, param 2) and No (id 3) side by side; the
/// rejections and the menu bar only in the original.
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_reject_bid_action(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I4%%);
    ts_compare_answer("Reject bid dialog", item, &[1, 3], o, transfer_screens_handle_reject_bid_action)
}

/// Builds the dialog that says a bid must be accepted.
///
/// Params: 0 the offer slot, 1 the action tried (0x1b reject, 0x1c negotiate, 0x23 delay), 2 why (0x1b: a release
/// clause). The player's picture, the title in the human's club colours, the subtitle "<buyer> bid for <player>"
/// (name style 5), then (name style 1) by the action: with the release clause "This offer may not be
/// rejected/negotiated/delayed as ..." with the player, otherwise "The <club> board ..." with the human's club
/// (`club::GENDER_NAME`, `club::NAME`); another action leaves the text as it was. The text area, the menu bar and a
/// single Back button (id 1).
/// Original: `transfer_screens_build_offer_must_accept_dialog` (0x8f0bc0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_build_offer_must_accept_dialog() {
    covered!("tel17_2 offer must accept dialog");
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let slot = get_screen_param(0);
    let action = get_screen_param(1) as u32;
    let why = get_screen_param(2) as u16;
    let o = read_usize(slot);
    let player = staff_at(read_i32(o + offer::TARGET_STAFF_ID) as isize).addr();
    let buyer = club_at(read_i32(o + offer::BUYER_CLUB_ID) as isize).addr();
    let human = ts_human_club();
    picture(player, 0, 0);
    ts_title_in_club_colours(human);
    let mut name = [0u8; 300];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(player, n, 300, 5);
    let g = read_i8(buyer + club::SHORT_GENDER_NAME) as i32;
    lang(TS_TEXT, 0xa9f17c, g, g, buyer + club::SHORT_NAME, n);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    let lines = ts_dialog_lines();
    exe::database_format_staff_name(player, n, 300, 1);
    let texts = match action {
        0x1b => Some((0xa9f114usize, 0xa9f084usize)),
        0x1c => Some((0xa9f018, 0xa9ef88)),
        0x23 => Some((0xa9ef20, 0xa9ee94)),
        _ => None,
    };
    if let Some((clause, board)) = texts {
        if why == 0x1b {
            lang(TS_TEXT, clause, 100, n);
        } else {
            let hg = read_i8(human + club::GENDER_NAME) as i32;
            lang(TS_TEXT, board, hg, hg, human + club::NAME);
        }
    }
    ts_dialog_text(lines);
    exe::menubar_build(4, 0);
    ts_back_button();
}

/// Comparer of `transfer_screens_build_offer_must_accept_dialog`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_offer_must_accept_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I5%%);
    compare_by_display_list_with("Offer must accept dialog", &[], || o(), || transfer_screens_build_offer_must_accept_dialog());
}

/// Opens the dialog on why a club cannot sign, loan or trial a player (build
/// `transfer_screens_build_cannot_sign_dialog`, input `transfer_screens_handle_back_dialog_action`, flag 1) and sets
/// its params: 0 the player, 1 the club, 2 the reason (sign-extended), 4 the mode (0 bid, 1 loan, 2 trial, else sign;
/// sign-extended). For the reasons 0x12 and 0x20, param 3 gets the buyer club id of every bid for the player whose
/// status is 0xb, 0xc, 0xd, 0xe or 0x13 (the last one wins).
/// Original: `transfer_screens_open_cannot_sign_dialog` (0x8f0e40, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_cannot_sign_dialog(player: usize, club: usize, reason: i16, mode: i8) {
    covered!("tel17_2 open cannot sign dialog");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8f0f90usize, 0x8f16c0usize, 1, 0usize, 0) == 0 {
        return;
    }
    set_screen_param(0, player);
    set_screen_param(1, club);
    set_screen_param(2, reason as i32 as usize);
    set_screen_param(4, mode as i32 as usize);
    if reason == 0x12 || reason == 0x20 {
        let mut list = [0u8; 0x10];
        let l = list.as_mut_ptr() as usize;
        exe::new_llist(l);
        let tm = read_usize(G_TRANSFER_MANAGER);
        exe::transfer_manager_get_offers_for_player(tm, player, l, 0);
        write_u32(l + llist::CURSOR, read_u32(l + llist::HEAD));
        loop {
            let node = exe::llist_get_next(l);
            if node == 0 {
                break;
            }
            let bid = read_usize(exe::transfer_manager_get_offer_slot(tm, read_i32(node)));
            if matches!(read_u8(bid + offer::STATUS), 0xb | 0xc | 0xd | 0xe | 0x13) {
                set_screen_param(3, read_u32(bid + offer::BUYER_CLUB_ID) as usize);
            }
        }
        llist_drop(l);
    }
}

/// Comparer of `transfer_screens_open_cannot_sign_dialog`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_cannot_sign_dialog(player: usize, club: usize, reason: i16, mode: i8) {
    original_fn::<unsafe extern "C" fn(usize, usize, i16, i8)>(%%I6%%)(player, club, reason, mode)
}

/// The contract club of `player` when the contract is not of type 5 (`contract +0x35` low 6 bits), else 0, looked up
/// and stored in `G_CONTRACT_LOOKUP_RESULT` like the original's inline macro.
unsafe fn ts_contract_club_not_type_5(player: usize) -> usize {
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    if c == 0 || read_u8(c + 0x35) & 0x3f == 5 {
        0
    } else {
        club_at(read_i32(c + 4) as isize).addr()
    }
}

/// Builds the dialog that explains why a club cannot sign, loan or trial a player.
///
/// Params: 0 the player, 1 the club, 2 the reason, 3 the club of the agreed deal (reasons 0x12, 0x20), 4 the mode. The
/// player's picture, the title in the club's colours, the subtitle by the mode ("Bid for", "Cannot loan", "Cannot
/// trial", "Cannot sign" <player>). The text by the reason: 7 the player alone; 0x27 "Players from Saudi Arabia are
/// not..."; 0x21 the club he agreed a contract with (`get_agreed_contract_club`; when its contract offer
/// (`find_contract_offer`) links a transfer offer, that offer's `offer::PLAYER_AWARE` is set to 1); 1 his contract club
/// (a contract of type 5 counts as none: a text with the player alone), named three times; 0x12/0x20 the club of param
/// 3; 0x29 the club; 0x19/0x28 the club and a nation adjective (0x19: the nation of `KEY_NATION_PAYS_BASQUE`, 0x28: the
/// club's nation), worded by the mode; 0x1a the club and the nation of `KEY_NATION_SPAIN`, worded by the mode; any other reason
/// leaves the text as it was. The text area, the menu bar and a Back button (id 1).
/// Original: `transfer_screens_build_cannot_sign_dialog` (0x8f0f90, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_build_cannot_sign_dialog() {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let player = get_screen_param(0);
    let club = get_screen_param(1);
    let reason = get_screen_param(2) as i16;
    let mode = get_screen_param(4) as i8;
    picture(player, 0, 0);
    ts_title_in_club_colours(club);
    let mut name = [0u8; 300];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(player, n, 300, 1);
    let subtitle = match mode {
        0 => 0xa9f6ec,
        1 => 0xa9f6bc,
        2 => 0xa9f688,
        _ => 0xa9f658,
    };
    lang(TS_TEXT, subtitle, n);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    let lines = ts_dialog_lines();
    let g = |c: usize| read_i8(c + club::SHORT_GENDER_NAME) as i32;
    let mut adjective = [0u8; 100];
    let adj = adjective.as_mut_ptr() as usize;
    let nation_of = |r: i16| -> usize {
        if r == 0x19 {
            (read_usize(G_NATION_TABLE) as isize + read_i32(KEY_NATION_PAYS_BASQUE) as isize * 0x122) as usize
        } else {
            read_usize(club + club::NATION)
        }
    };
    match reason {
        7 => {
            lang(TS_TEXT, 0xa8f3c4, n);
        }
        0x27 => {
            lang(TS_TEXT, 0xa9f610);
        }
        0x21 => {
            covered!("tel17_2 cannot sign: contract agreed elsewhere");
            let mut agreed: usize = 0;
            exe::transfer_manager_get_agreed_contract_club(read_usize(G_TRANSFER_MANAGER), player, &mut agreed as *mut usize as usize);
            let offer_ = exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), read_u32(player), read_u32(agreed));
            if offer_ != 0 && read_i32(offer_ + 0x59) != -1 {
                let s = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), read_i32(offer_ + 0x59));
                if s != 0 && read_usize(s) != 0 {
                    write_u8(read_usize(s) + offer::PLAYER_AWARE, 1);
                }
            }
            lang(TS_TEXT, 0xa8f468, g(agreed), g(agreed), n, agreed + club::SHORT_NAME);
        }
        1 => {
            let first = ts_contract_club_not_type_5(player);
            if first == 0 {
                lang(TS_TEXT, 0xa9f560, n);
            } else {
                let (c1, c2, c3) = (ts_contract_club_not_type_5(player), ts_contract_club_not_type_5(player), ts_contract_club_not_type_5(player));
                lang(TS_TEXT, 0xa9f5a8, g(c3), g(c2), n, c1 + club::SHORT_NAME);
            }
        }
        0x12 | 0x20 => {
            let deal = club_at(get_screen_param(3) as i32 as isize).addr();
            lang(TS_TEXT, 0xa9f1cc, g(deal), g(deal), n, deal + club::SHORT_NAME);
        }
        0x29 => {
            lang(TS_TEXT, 0xa9f514, g(club), g(club), club + club::SHORT_NAME);
        }
        0x19 | 0x28 => {
            let fmt = match mode {
                0 => 0xa9f308,
                1 => 0xa8f30c,
                2 => 0xa9f29c,
                _ => 0xa9f230,
            };
            exe::nation_get_nationality_adjective(nation_of(reason), adj, 100, 0xd);
            lang(TS_TEXT, fmt, g(club), g(club), 100, 100, club + club::SHORT_NAME, adj);
        }
        0x1a => {
            let fmt = match mode {
                0 => 0xa9f4ac,
                1 => 0xa9f444,
                2 => 0xa9f3dc,
                _ => 0xa9f374,
            };
            let nation = (read_usize(G_NATION_TABLE) as isize + read_i32(KEY_NATION_SPAIN) as isize * 0x122) as usize;
            exe::nation_get_nationality_adjective(nation, adj, 100, 0xd);
            lang(TS_TEXT, fmt, g(club), g(club), 100, 100, club + club::SHORT_NAME, adj);
        }
        _ => {}
    }
    covered!("tel17_2 cannot sign dialog");
    ts_dialog_text(lines);
    exe::menubar_build(4, 0);
    ts_back_button();
}

/// Comparer of `transfer_screens_build_cannot_sign_dialog`: side by side over the display list and the contract
/// lookup global; for reason 0x21 only the original runs (it marks the linked offer).
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_cannot_sign_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I7%%);
    if get_screen_param(2) as i16 == 0x21 {
        // simplification: the mark on the offer (+0x2d) does not run twice
        return o();
    }
    compare_by_display_list_with("Cannot sign dialog", &[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(), || transfer_screens_build_cannot_sign_dialog());
}

/// Answers the single Back button of the cannot sign and must accept dialogs: Back (id 1) -9, the rest to the menu
/// bar.
/// Original: `transfer_screens_handle_back_dialog_action` (0x8f16c0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_handle_back_dialog_action(item: u32) -> i32 {
    if tel26_item_id(item as i16) == 1 {
        covered!("tel17_2 dialog Back");
        return -9;
    }
    ts_menubar_answer(item)
}

/// Comparer of `transfer_screens_handle_back_dialog_action`: Back side by side, the menu bar only in the original.
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_back_dialog_action(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I8%%);
    ts_compare_answer("Back dialog", item, &[1], o, transfer_screens_handle_back_dialog_action)
}
