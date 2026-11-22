// ---------- batch %%LEVA%%: the confirm trial and loan offer dialogs and the end loan dialog (transfer_screens.cpp, part 2: tel18_6) ----------

/// Opens the dialog that confirms a trial offer (build `transfer_screens_build_confirm_offer_dialog`, input
/// `transfer_screens_handle_confirm_offer_dialog`, flag 1): param 0 the player, 1 the weeks (the low byte of `weeks`,
/// sign-extended), 2 to 6 -1, 7 0 (no loan offer).
/// Original: `transfer_screens_open_confirm_trial_offer` (0x8fb530, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_confirm_trial_offer(player: usize, weeks: u32) {
    covered!("tel18_6 open confirm trial offer");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8fb6a0usize, 0x8fb9e0usize, 1, 0usize, 0) != 0 {
        set_screen_param(0, player);
        set_screen_param(1, weeks as u8 as i8 as i32 as usize);
        for k in 2..7 {
            set_screen_param(k, usize::MAX);
        }
        set_screen_param(7, 0);
    }
}

/// Comparer of `transfer_screens_open_confirm_trial_offer`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_confirm_trial_offer(player: usize, weeks: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32)>(%%I0%%)(player, weeks)
}

/// Opens the dialog that confirms a loan offer (the same build and input as the trial, flag 1): param 0 the player,
/// 1 -1 (no weeks: a loan), 2 the loan's end month (low byte of `month`, sign-extended), 3 its year (low word), 4 the
/// low byte of `a4` (sign-extended), 5 `a5`, 6 `a6`, 7 the loan offer under negotiation (0 for a new offer).
/// Original: `transfer_screens_open_confirm_loan_offer` (0x8fb5e0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_confirm_loan_offer(player: usize, month: u32, year: u32, a4: u32, a5: u32, a6: u32, offer: usize) {
    covered!("tel18_6 open confirm loan offer");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8fb6a0usize, 0x8fb9e0usize, 1, 0usize, 0) != 0 {
        set_screen_param(0, player);
        set_screen_param(1, usize::MAX);
        set_screen_param(2, month as u8 as i8 as i32 as usize);
        set_screen_param(3, (year & 0xffff) as usize);
        set_screen_param(4, a4 as u8 as i8 as i32 as usize);
        set_screen_param(5, a5 as usize);
        set_screen_param(6, a6 as usize);
        set_screen_param(7, offer);
    }
}

/// Comparer of `transfer_screens_open_confirm_loan_offer`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_confirm_loan_offer(player: usize, month: u32, year: u32, a4: u32, a5: u32, a6: u32, offer: usize) {
    original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, usize)>(%%I1%%)(player, month, year, a4, a5, a6, offer)
}

/// Builds the confirm offer dialog (trial or loan).
///
/// Param 0 the player, 1 the weeks (byte; 0xff = a loan), 7 the loan offer under negotiation. The player's picture;
/// without the slot's manager the assert box (line 0x26f4). The title in the kit colours of the manager's club; the
/// subtitle (name style 5) "Confirm offer to <player>" (trial), "Negotiate loan offer for <player>" (loan with an
/// offer) or "Confirm loan offer for <player>". The question: a trial (name style 1) "Offer a 1 week trial to
/// <player> ?" or "Offer a <n> week trial to <player> ?"; a loan with an offer "Negotiate <buyer club>'s loan offer
/// for <player> ?" (the club of `offer::BUYER_CLUB_ID`), else "Submit loan offer for <player> ?", the loan texts with
/// the name still in style 5. The text area with the player's link, the menu bar and the bar No (1) / Yes (2).
/// Original: `transfer_screens_build_confirm_offer_dialog` (0x8fb6a0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_build_confirm_offer_dialog() {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let text_area: TsTextAreaLinks = core::mem::transmute(0x5ed070usize);
    let player = get_screen_param(0);
    let weeks = get_screen_param(1) as u8;
    let offer = get_screen_param(7);
    picture(player, 0, 0);
    let manager = tel26_slot_manager().addr();
    if manager == 0 {
        tel26_error_box(TS_CPP, 0x26f4);
        return;
    }
    covered!("tel18_6 confirm offer dialog");
    ts_title_in_club_colours(read_usize(manager + staff::CLUB_JOB));
    let mut name = [0u8; 0x12c];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(player, n, 0x12c, 5);
    lang(TS_TEXT, if weeks != 0xff { 0xaa1568 } else if offer != 0 { 0xaa1528 } else { 0xaa14ec }, n);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    let lines = ts_dialog_lines();
    if weeks != 0xff {
        exe::database_format_staff_name(player, n, 0x12c, 1);
        if weeks == 1 {
            lang(TS_TEXT, 0xaa14ac, n); // "Offer a 1 week trial to <player> ?"
        } else {
            lang(TS_TEXT, 0xaa1458, weeks as i8 as i32, n);
        }
    } else if offer != 0 {
        let club = club_at(read_i32(offer + offer::BUYER_CLUB_ID) as isize).addr();
        let g = read_i8(club + club::SHORT_GENDER_NAME) as i32;
        lang(TS_TEXT, 0xaa13f4, g, g, club + club::SHORT_NAME, n);
    } else {
        lang(TS_TEXT, 0xaa13b8, n);
    }
    text_area(0x6e, 0x8c, 0x30c, 0x221, 2, 0, lines as i8 as i32 as u32, 3, ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT, n, 0x7d7u32, player, 0u32);
    exe::menubar_build(4, 0);
    ts_no_yes_bar(1, 2);
}

/// Comparer of `transfer_screens_build_confirm_offer_dialog`: side by side over the display list; without the slot's
/// manager only the original runs (the assert box).
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_confirm_offer_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I2%%);
    if tel26_slot_manager().addr() == 0 {
        return o();
    }
    compare_by_display_list_with("Confirm offer dialog", &[], || o(), || transfer_screens_build_confirm_offer_dialog());
}

/// Answers the confirm offer dialog: No (id 1) -9. Yes (id 2), -10 after the offer is made: a trial (weeks of param 1
/// not 0xff) builds a contract on the stack (`STAFF_CONTRACT::STAFF_CONTRACT`, then `init_trial_offer(player, the
/// manager's club, weeks)`) and adds it as a contract offer (`CONTRACT_MANAGER::add_contract_offer(&contract, -1, -1,
/// 0xb, 0, 1, 0)`); a loan dates its end on day min(today's day of the month, 28), month param 2 (the byte over the
/// item argument's high bytes) and year param 3 (`Date::Date(day, month, year, -1)`), then the offer of param 7 goes to
/// `TRANSFER_MANAGER::add_loan_negotiation(offer, &date, param 4, param 5, param 6)`, or without one
/// `make_loan_offer(player, the manager's club, &date, param 4, param 5, param 6)`. The rest goes to the menu bar.
/// Original: `transfer_screens_handle_confirm_offer_dialog` (0x8fb9e0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_handle_confirm_offer_dialog(item: u32) -> i32 {
    let id = tel26_item_id(item as i16);
    if id == 1 {
        return -9;
    }
    if id != 2 {
        return ts_menubar_answer(item);
    }
    let player = get_screen_param(0);
    let weeks = get_screen_param(1) as u8;
    let month = get_screen_param(2) as u8;
    let year = get_screen_param(3) as u32;
    let a4 = get_screen_param(4) as u8 as u32;
    let a5 = get_screen_param(5) as u32;
    let a6 = get_screen_param(6) as u32;
    let offer = get_screen_param(7);
    let manager = tel26_slot_manager().addr();
    if weeks != 0xff {
        covered!("tel18_6 confirm trial Yes");
        let init: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x85e480usize);
        let mut contract = [0u8; 0x50];
        let c = contract.as_mut_ptr();
        exe::new_staff_contract(c as usize);
        init(c as usize, player, read_usize(manager + staff::CLUB_JOB), weeks as u32);
        exe::contract_manager_add_contract_offer(read_usize(G_CONTRACT_MANAGER), c, usize::MAX, usize::MAX, 0xb, 0, 1, 0);
        return -10;
    }
    let today = exe::date_get_day_of_month(G_CURRENT_DATE) as i16;
    let day = if today > 0x1c { 0x1c } else { exe::date_get_day_of_month(G_CURRENT_DATE) as i16 as i32 as u32 };
    let mut date = [0u16; 4];
    exe::date_date_549f70(&mut date, day, (item & !0xff) | month as u32, year, u32::MAX);
    let d = date.as_mut_ptr() as usize;
    if offer != 0 {
        covered!("tel18_6 confirm loan Yes: negotiation");
        let negotiate: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x8bee50usize);
        negotiate(read_usize(G_TRANSFER_MANAGER), offer, d, a4, a5, a6);
    } else {
        covered!("tel18_6 confirm loan Yes: new offer");
        exe::transfer_manager_make_loan_offer(read_usize(G_TRANSFER_MANAGER), player, read_usize(manager + staff::CLUB_JOB), d, a4, a5, a6);
    }
    -10
}

/// Comparer of `transfer_screens_handle_confirm_offer_dialog`: No side by side; Yes (the offers) and the menu bar only
/// in the original.
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_confirm_offer_dialog(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I3%%);
    ts_compare_answer("Confirm offer dialog", item, &[1], o, transfer_screens_handle_confirm_offer_dialog)
}

/// Opens the dialog that ends a player's loan (recalls him, or terminates the loan of a player loaned to the human).
///
/// When `TRANSFER_MANAGER::can_end_loan(the manager's club, player, TS_TEXT, today, 0)` refuses (the reason in
/// `TS_TEXT`): a box (`confirm_push`, kind 0, code 0) titled with the manager's club name in its kit colours, headed
/// "Cannot terminate <player>'s loan" when he plays for the manager's club, else "Cannot recall <player>" (name
/// style 5). Otherwise the dialog (build `transfer_screens_build_end_loan_dialog`, input
/// `transfer_screens_handle_end_loan_dialog`, flag 1): param 0 the player, 1 the club he plays for.
/// Original: `transfer_screens_open_end_loan_dialog` (0x8fbcc0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_end_loan_dialog(player: usize) {
    let manager = tel26_slot_manager().addr();
    if exe::transfer_manager_can_end_loan(read_usize(G_TRANSFER_MANAGER), read_usize(manager + staff::CLUB_JOB), player, TS_TEXT as u32, G_CURRENT_DATE, 0) == 0 {
        covered!("tel18_6 cannot end loan");
        let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
        let confirm: unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x591840usize);
        let (mut name, mut heading) = ([0u8; 0x190], [0u8; 0x190]);
        let (n, h) = (name.as_mut_ptr() as usize, heading.as_mut_ptr() as usize);
        exe::database_format_staff_name(player, n, 0x190, 5);
        if read_usize(manager + staff::CLUB_JOB) == read_usize(player + staff::CLUB_JOB) {
            lang(h, 0xaa15d4, 0x64, n);
        } else {
            lang(h, 0xaa15a0, n);
        }
        let club = read_usize(manager + staff::CLUB_JOB);
        let (mut a, mut b) = (0u32, 0u32);
        exe::database_get_club_kit_colours(club, 1, &mut a, &mut b);
        confirm(club + club::NAME, h, TS_TEXT, 0, b, a, 0);
        return;
    }
    covered!("tel18_6 open end loan dialog");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8fbdf0usize, 0x8fc0e0usize, 1, 0usize, 0) != 0 {
        set_screen_param(0, player);
        set_screen_param(1, read_usize(player + staff::CLUB_JOB));
    }
}

/// Comparer of `transfer_screens_open_end_loan_dialog`: only the original runs (it opens a box or a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_end_loan_dialog(player: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I4%%)(player)
}

/// Builds the end loan dialog.
///
/// Param 0 the player, 1 the club he plays for. The player's picture; without the slot's manager the assert box (line
/// 0x27f0). The title in the kit colours of the manager's club. When the player plays for the manager's club, the
/// subtitle "Terminate <player>'s loan" and "Terminate <player>'s loan contract at <club> ?", else "Recall <player>
/// from loan" and "Recall <player> from loan at <club> ?" (subtitle name style 5, text style 1). The text area with
/// the player's and the club's links, the menu bar and the bar No (1) / Yes (2).
/// Original: `transfer_screens_build_end_loan_dialog` (0x8fbdf0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_build_end_loan_dialog() {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let text_area: TsTextAreaLinks = core::mem::transmute(0x5ed070usize);
    let player = get_screen_param(0);
    let club = get_screen_param(1);
    picture(player, 0, 0);
    let manager = tel26_slot_manager().addr();
    if manager == 0 {
        tel26_error_box(TS_CPP, 0x27f0);
        return;
    }
    covered!("tel18_6 end loan dialog");
    let own = read_usize(manager + staff::CLUB_JOB);
    ts_title_in_club_colours(own);
    let mut name = [0u8; 0x12c];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(player, n, 0x12c, 5);
    if own == read_usize(player + staff::CLUB_JOB) {
        lang(TS_TEXT, 0xaa170c, 0, n);
    } else {
        lang(TS_TEXT, 0xaa16d4, n);
    }
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    let lines = ts_dialog_lines();
    exe::database_format_staff_name(player, n, 0x12c, 1);
    let g = read_i8(club + club::SHORT_GENDER_NAME) as i32;
    if own == read_usize(player + staff::CLUB_JOB) {
        lang(TS_TEXT, 0xaa166c, 0, g, g, n, club + club::SHORT_NAME); // "Terminate <player>'s loan contract at <club> ?"
    } else {
        lang(TS_TEXT, 0xaa1610, g, g, n, club + club::SHORT_NAME);
    }
    text_area(0x6e, 0x8c, 0x30c, 0x221, 2, 0, lines as i8 as i32 as u32, 3, ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT,
        n, 0x7d7u32, player, club + club::SHORT_NAME, 0x7d5u32, club, 0u32);
    exe::menubar_build(4, 0);
    ts_no_yes_bar(1, 2);
}

/// Comparer of `transfer_screens_build_end_loan_dialog`: side by side over the display list; without the slot's
/// manager only the original runs (the assert box).
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_end_loan_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I5%%);
    if tel26_slot_manager().addr() == 0 {
        return o();
    }
    compare_by_display_list_with("End loan dialog", &[], || o(), || transfer_screens_build_end_loan_dialog());
}

/// Answers the end loan dialog.
///
/// On a network game (`[0xb77c84]`) with the pending command 3: the pending command and its argument cleared, -9. No
/// (id 1) -9. Yes (id 2), -4: the player's contract club looked up (written to `G_CONTRACT_LOOKUP_RESULT`; no contract
/// is not checked), `CONTRACT_MANAGER::schedule_loan_end(player, the manager's club)`, and a box
/// (`confirm_push_text_object`, kind 0, code 3) titled with the manager's club name in its kit colours, headed
/// "<player>'s loan terminated" when he plays for the manager's club, else "<player> recalled from loan" (name style
/// 5), with the text "<player> will return to <contract club> tomorrow." (style 1) linking the player (0x7d7) and the
/// manager's club's short name (0x7d5). The rest goes to the menu bar.
/// Original: `transfer_screens_handle_end_loan_dialog` (0x8fc0e0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_handle_end_loan_dialog(item: u32) -> i32 {
    if read_u32(TS_NETWORK_GAME) != 0 && read_u32(G_PENDING_COMMAND_CODE) == 3 {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        write_u32(G_PENDING_COMMAND_ARG, 0);
        return -9;
    }
    let id = tel26_item_id(item as i16);
    if id == 1 {
        return -9;
    }
    if id != 2 {
        return ts_menubar_answer(item);
    }
    covered!("tel18_6 end loan Yes");
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let schedule: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4ccca0usize);
    let confirm: unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x5919b0usize);
    let player = get_screen_param(0);
    let contract_club = ts_contract_club_any(player);
    let own = read_usize(tel26_slot_manager().addr() + staff::CLUB_JOB);
    schedule(read_usize(G_CONTRACT_MANAGER), player, own);
    let (mut a, mut b) = (0u32, 0u32);
    exe::database_get_club_kit_colours(own, 1, &mut a, &mut b);
    let (mut name, mut heading) = ([0u8; 0x190], [0u8; 0x258]);
    let (n, h) = (name.as_mut_ptr() as usize, heading.as_mut_ptr() as usize);
    exe::database_format_staff_name(player, n, 0x190, 5);
    if own == read_usize(player + staff::CLUB_JOB) {
        lang(h, 0xaa17d8, 0, n);
    } else {
        lang(h, 0xaa17a4, n);
    }
    exe::database_format_staff_name(player, n, 0x190, 1);
    let g = read_i8(contract_club + club::SHORT_GENDER_NAME) as i32;
    lang(TS_TEXT, 0xaa1744, g, g, n, contract_club + club::SHORT_NAME);
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, TS_TEXT);
    exe::text_add_link(t as usize, n as *const u8, 0x7d7, player);
    exe::text_add_link(t as usize, (own + club::SHORT_NAME) as *const u8, 0x7d5, own);
    confirm(own + club::NAME, h, t as usize, 0, a, b, 3);
    exe::drop_text(t);
    -4
}

/// Comparer of `transfer_screens_handle_end_loan_dialog`: No side by side; the network command, Yes (the loan end and
/// the box) and the menu bar only in the original.
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_end_loan_dialog(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I6%%);
    if read_u32(TS_NETWORK_GAME) != 0 && read_u32(G_PENDING_COMMAND_CODE) == 3 {
        return o(item);
    }
    ts_compare_answer("End loan dialog", item, &[1], o, transfer_screens_handle_end_loan_dialog)
}
