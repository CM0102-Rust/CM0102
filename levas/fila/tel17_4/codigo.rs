// ---------- batch %%LEVA%%: the offer changed screen, the remove exchange player dialog and the confirm transfer button (transfer_screens.cpp, part 1: tel17_4) ----------

/// `gui_add_text_area` (0x5ed070) is variadic: after the text come (link text, link id, object) triples, ended by 0.
type TsTextAreaLinks = unsafe extern "C" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, usize, ...);
/// The final confirmation screen opener (`transfer_screens_open_confirm_transfer_screen`, 0x8f2e10, cdecl(slot,
/// id_a, id_b)).
const TS_FN_OPEN_CONFIRM_TRANSFER_SCREEN: usize = 0x8f2e10;

/// Opens the screen that tells the human a club has since changed, rejected, withdrawn or accepted the bid (build
/// `transfer_screens_build_offer_changed_screen`, input `transfer_screens_handle_offer_changed_action`, flag 1): params
/// 0 the offer, 1 the serial the news item knew.
/// Original: `transfer_screens_open_offer_changed_screen` (0x8f2470, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_offer_changed_screen(offer_: usize, serial: u32) {
    covered!("tel17_4 open offer changed screen");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8f24c0usize, 0x8f2800usize, 1, 0usize, 0) != 0 {
        set_screen_param(0, offer_);
        set_screen_param(1, serial as usize);
    }
}

/// Comparer of `transfer_screens_open_offer_changed_screen`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_offer_changed_screen(offer_: usize, serial: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32)>(%%I0%%)(offer_, serial)
}

/// Builds the offer changed screen.
///
/// Param 0 is the offer itself (not a slot), param 1 the serial the human last saw. The player's picture, the title in
/// the human's club colours; a Text "Transfer Bid for <player>" with a link to the player (0x7d7) drawn wrapped in
/// yellow, font 6, in an area 0x64,0x50..0x316,0x7d. The text: when the offer's serial moved on, 0xa9f94c (the buyer
/// changed the bid); otherwise by `offer::STATUS`: 2 (rejected) 0xa9f8e0 with the player's contract club (a contract of
/// type 5 counts as none), 5 or 6 (accepted) 0xa9f804 with the contract club, 3 (withdrawn) 0xa9f870 with the buyer;
/// another status leaves the text as it was. The text area carries the links of the contract club (0x7d5), the buyer
/// (0x7d5) and the player (0x7d7). Then the menu bar and a Done button (id 3).
/// Original: `transfer_screens_build_offer_changed_screen` (0x8f24c0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_build_offer_changed_screen() {
    covered!("tel17_4 offer changed screen");
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let text_new: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_TEXT_TEXT);
    let text_set: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(FN_TEXT_SET_STRING);
    let text_link: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> i32 = core::mem::transmute(FN_TEXT_ADD_LINK);
    let text_drop: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TEXT_DESTRUCTOR_TEXT);
    let draw: unsafe extern "thiscall" fn(usize, u32, i32, i32, *mut i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(TS_FN_TEXT_DRAW_WRAPPED);
    let text_area: TsTextAreaLinks = core::mem::transmute(0x5ed070usize);
    let mut text = [0u32; 8];
    let t = text.as_mut_ptr() as usize;
    text_new(t, 0);
    let mut row: i32 = 0;
    let o = get_screen_param(0);
    let serial = get_screen_param(1) as u32;
    let human = ts_human_club();
    let player = staff_at(read_i32(o + offer::TARGET_STAFF_ID) as isize).addr();
    picture(player, 0, 0);
    let buyer = club_at(read_i32(o + offer::BUYER_CLUB_ID) as isize).addr();
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    let seller = if c == 0 || read_u8(c + 0x35) & 0x3f == 5 { 0 } else { club_at(read_i32(c + 4) as isize).addr() };
    ts_title_in_club_colours(human);
    let mut name = [0u8; 300];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(player, n, 300, 1);
    lang(TS_TEXT, 0xa9f7c4, n); // "Transfer Bid for <player>"
    text_set(t, TS_TEXT);
    text_link(t, n, 0x7d7, player);
    let area = tel26_add_area(current_human_display(), [0x64, 0x50, 0x316, 0x7d, 1, 0, 1, 0, 2, 0, u32::MAX]);
    draw(t, area, 0x64, 0x316, &mut row, 6, ts_colour(G_GUI_COLOUR_YELLOW), 1, 0);
    let lines = ts_dialog_lines();
    let g = |c: usize| read_i8(c + club::SHORT_GENDER_NAME) as i32;
    if read_u32(o + offer::SERIAL) != serial {
        lang(TS_TEXT, 0xa9f94c, g(buyer), g(buyer), buyer + club::SHORT_NAME, n);
    } else {
        match read_u8(o + offer::STATUS) {
            2 => {
                lang(TS_TEXT, 0xa9f8e0, g(seller), g(seller), seller + club::SHORT_NAME, n);
            }
            5 | 6 => {
                lang(TS_TEXT, 0xa9f804, g(seller), g(seller), seller + club::SHORT_NAME, n);
            }
            3 => {
                lang(TS_TEXT, 0xa9f870, g(buyer), g(buyer), buyer + club::SHORT_NAME, n);
            }
            _ => {}
        }
    }
    text_area(0x6e, 0x8c, 0x30c, 0x221, 2, 0, lines as i8 as i32 as u32, 3, ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT,
        seller + club::SHORT_NAME, 0x7d5u32, seller, buyer + club::SHORT_NAME, 0x7d5u32, buyer, n, 0x7d7u32, player, 0u32);
    exe::menubar_build(4, 0);
    lang(TS_TEXT, 0xa9f7fc); // "Done"
    tel26_add_button([100, 0x22b, 0x316, 0x24e, 0x30, ts_colour(G_GUI_COLOUR_GREY), 0xc, 3, ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT as u32, 3, 0]);
    text_drop(t);
}

/// Comparer of `transfer_screens_build_offer_changed_screen`: side by side over the display list and the contract
/// lookup global (the Text is the frame's own).
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_offer_changed_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    compare_by_display_list_with("Offer changed screen", &[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(), || transfer_screens_build_offer_changed_screen());
}

/// Answers the offer changed screen: Done (id 3) -10, the rest to the menu bar.
/// Original: `transfer_screens_handle_offer_changed_action` (0x8f2800, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_handle_offer_changed_action(item: u32) -> i32 {
    if tel26_item_id(item as i16) == 3 {
        covered!("tel17_4 offer changed Done");
        return -10;
    }
    ts_menubar_answer(item)
}

/// Comparer of `transfer_screens_handle_offer_changed_action`: Done side by side, the menu bar only in the original.
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_offer_changed_action(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%);
    ts_compare_answer("Offer changed screen", item, &[3], o, transfer_screens_handle_offer_changed_action)
}

/// Opens the dialog that asks whether to remove a part exchange player from a bid (build
/// `transfer_screens_build_remove_exchange_player_dialog`, input `transfer_screens_handle_remove_exchange_player_action`,
/// flag 1): params 0 the offer slot, 1 the exchange player's id.
/// Original: `transfer_screens_open_remove_exchange_player_dialog` (0x8f2870, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_remove_exchange_player_dialog(slot: usize, player: u32) {
    covered!("tel17_4 open remove exchange player dialog");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8f28c0usize, 0x8f2b90usize, 1, 0usize, 0) != 0 {
        set_screen_param(0, slot);
        set_screen_param(1, player as usize);
    }
}

/// Comparer of `transfer_screens_open_remove_exchange_player_dialog`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_remove_exchange_player_dialog(slot: usize, player: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32)>(%%I3%%)(slot, player)
}

/// Builds the remove exchange player dialog.
///
/// The bid's target's picture, the title in the human's club colours, the subtitle "Remove <exchange player> from
/// transfer bid..." (name style 5), then the text 0xa9f9b8 with the exchange player, the target and the new cash part:
/// the bid's fee plus the exchange player's value (`staff +0x52`), formatted (`Cash::get_display_amount(6)`,
/// `cash_format_money(.., 6)`); the text area links both players (0x7d7). The menu bar and the bar No (id 1) / Yes
/// (id 2).
/// Original: `transfer_screens_build_remove_exchange_player_dialog` (0x8f28c0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_build_remove_exchange_player_dialog() {
    covered!("tel17_4 remove exchange player dialog");
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let text_area: TsTextAreaLinks = core::mem::transmute(0x5ed070usize);
    let weights: [u8; 2] = [3, 1];
    let o = read_usize(get_screen_param(0));
    let exchange = staff_at(get_screen_param(1) as i32 as isize).addr();
    let target = staff_at(read_i32(o + offer::TARGET_STAFF_ID) as isize).addr();
    let human = ts_human_club();
    picture(target, 0, 0);
    ts_title_in_club_colours(human);
    let mut name = [0u8; 0x190];
    let mut target_name = [0u8; 0x190];
    let mut money = [0u8; 200];
    let n = name.as_mut_ptr() as usize;
    let tn = target_name.as_mut_ptr() as usize;
    let m = money.as_mut_ptr() as usize;
    exe::database_format_staff_name(exchange, n, 0x190, 5);
    lang(TS_TEXT, 0xa9faa0, n);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    exe::database_format_staff_name(exchange, n, 0x190, 1);
    exe::database_format_staff_name(target, tn, 0x190, 1);
    let cash: u32 = read_u32(exchange + 0x52).wrapping_add(read_u32(o + offer::FEE));
    let amount = exe::cash_get_display_amount(&cash, 6);
    exe::cash_format_money(m, 200, amount, 6);
    lang(TS_TEXT, 0xa9f9b8, n, tn, m);
    let lines = ts_dialog_lines();
    text_area(0x6e, 0x8c, 0x30c, 0x221, 2, 0, lines as i8 as i32 as u32, 3, ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT,
        n, 0x7d7u32, exchange, tn, 0x7d7u32, target, 0u32);
    exe::menubar_build(4, 0);
    let bar = ts_button_bar(2, weights.as_ptr());
    ts_bar_button(bar, 0, TS_NO, 1);
    ts_bar_button(bar, 1, TS_YES, 2);
}

/// Comparer of `transfer_screens_build_remove_exchange_player_dialog`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_remove_exchange_player_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I4%%);
    compare_by_display_list_with("Remove exchange player dialog", &[], || o(), || transfer_screens_build_remove_exchange_player_dialog());
}

/// Answers the remove exchange player dialog. No (id 1) -9. Yes (id 2): the exchange player's value (`staff +0x52`)
/// is added to the bid's fee; when he is in the bid's exchange list (`offer::EXCHANGE_PLAYERS`, walked from its head),
/// he is removed (`TRANSFER_MANAGER::remove_exchange_player(player, offer, 1, 0)`) and, when the buyer is not a human's
/// club (`human_manager_is_human_club(buyer, 0, 1, 0, 0)`), the computer club reviews the bid
/// (`review_ai_club_bid(slot, 1)`); -10 either way. The rest goes to the menu bar.
/// Original: `transfer_screens_handle_remove_exchange_player_action` (0x8f2b90, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_handle_remove_exchange_player_action(item: u32) -> i32 {
    let id = tel26_item_id(item as i16);
    if id == 1 {
        return -9;
    }
    if id != 2 {
        return ts_menubar_answer(item);
    }
    covered!("tel17_4 remove exchange player Yes");
    let slot = get_screen_param(0);
    let o = read_usize(slot);
    let buyer = club_at(read_i32(o + offer::BUYER_CLUB_ID) as isize).addr();
    let id_ = get_screen_param(1) as u32;
    let player = staff_at(id_ as i32 as isize).addr();
    write_u32(o + offer::FEE, read_u32(o + offer::FEE).wrapping_add(read_u32(player + 0x52)));
    let list = read_usize(o + offer::EXCHANGE_PLAYERS);
    write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
    loop {
        let node = exe::llist_get_next(read_usize(o + offer::EXCHANGE_PLAYERS));
        if node == 0 {
            return -10;
        }
        if read_u32(node) == id_ {
            break;
        }
    }
    let tm = read_usize(G_TRANSFER_MANAGER);
    exe::transfer_manager_remove_exchange_player(tm, player, o, 1, 0);
    if exe::human_manager_is_human_club(buyer, 0, 1, 0, 0) == 0 {
        exe::transfer_manager_review_ai_club_bid(tm, slot, 1);
    }
    -10
}

/// Comparer of `transfer_screens_handle_remove_exchange_player_action`: No side by side; Yes (it changes the bid) and
/// the menu bar only in the original.
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_remove_exchange_player_action(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I5%%);
    ts_compare_answer("Remove exchange player dialog", item, &[1], o, transfer_screens_handle_remove_exchange_player_action)
}

/// Answers the Confirm button of an agreed transfer: slots 6 (the offer slot index) and 7 (its serial then). When the
/// offer is missing or still has the serial, and was not withdrawn, the final confirmation screen opens
/// (`transfer_screens_open_confirm_transfer_screen(slot, id_a, id_b)`); otherwise the offer changed screen. (A missing
/// offer is read as the original does: its status at address 0x2c.)
/// Original: `transfer_screens_news_confirm_transfer_action` (0x8f2d20, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_news_confirm_transfer_action(id_a: u32, id_b: u32) {
    covered!("tel17_4 news confirm transfer");
    let find: unsafe extern "thiscall" fn(usize, u32, u32, usize) -> usize = core::mem::transmute(0x786b70usize);
    let confirm: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_OPEN_CONFIRM_TRANSFER_SCREEN);
    let mut item = [0u8; 0xe0];
    let n = item.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0, 0);
    find(read_usize(G_NEWS), id_a, id_b, n);
    let index = exe::newsitem_get_slot(n, 6);
    let serial = exe::newsitem_get_slot(n, 7);
    let slot = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), index as i32);
    let o = read_usize(slot);
    let same = o == 0 || read_u32(o + offer::SERIAL) == serial;
    if same && read_u8(o + offer::STATUS) != 3 {
        confirm(slot, id_a, id_b);
    } else if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8f24c0usize, 0x8f2800usize, 1, 0usize, 0) != 0 {
        set_screen_param(0, o);
        set_screen_param(1, serial as usize);
    }
}

/// Comparer of `transfer_screens_news_confirm_transfer_action`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_news_confirm_transfer_action(id_a: u32, id_b: u32) {
    original_fn::<unsafe extern "C" fn(u32, u32)>(%%I6%%)(id_a, id_b)
}
