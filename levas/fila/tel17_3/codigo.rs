// ---------- batch %%LEVA%%: the transfer news buttons and the delay request dialog (transfer_screens.cpp, part 1: tel17_3) ----------

/// `News::find_item_by_id(id_a, id_b, out)` (0x786b70, thiscall on [G_NEWS], ret 0xc): copies the news item into `out`.
const TS_FN_NEWS_FIND_ITEM: usize = 0x786b70;
/// `transfer_screens_open_bid_screen_for_player(player, offer slot or 0)` (0x8e9350, cdecl).
const TS_FN_OPEN_BID_SCREEN_FOR_PLAYER: usize = 0x8e9350;
/// `contract_screens_offer_contract_to_staff(staff, offer slot, 1, 0)` (0x4dd0f0, cdecl).
const TS_FN_OFFER_CONTRACT_TO_STAFF: usize = 0x4dd0f0;
/// The bid screen (`transfer_screens_open_bid_screen_for_offer`, 0x8e9550, cdecl(slot, open)).
const TS_FN_OPEN_BID_SCREEN_FOR_OFFER: usize = 0x8e9550;
/// The screens a news button opens: the offer changed screen and the delay request dialog (build, input), the loan bid
/// screen (build, input).
const TS_OFFER_CHANGED_BUILD: usize = 0x8f24c0;
const TS_OFFER_CHANGED_INPUT: usize = 0x8f2800;
const TS_DELAY_REQUEST_BUILD: usize = 0x8f2010;
const TS_DELAY_REQUEST_INPUT: usize = 0x8f22c0;
const TS_LOAN_BID_BUILD: usize = 0x8f9a20;
const TS_LOAN_BID_INPUT: usize = 0x8fac30;
/// A contract offer's (`contract_manager_find_contract_offer`) fields read by the loan bid button: the wage offered
/// (+0xc, like the contract's), the expiry date (+0x2d, its year word at +0x2f) and the two cup clause bytes (+0x1c,
/// +0x1d) [inference: the loan bid screen's params 0xc..0x14].
const TS_CONTRACT_OFFER_EXPIRY: usize = 0x2d;
const TS_CONTRACT_OFFER_EXPIRY_YEAR: usize = 0x2f;
const TS_CONTRACT_OFFER_CLAUSE_A: usize = 0x1c;
const TS_CONTRACT_OFFER_CLAUSE_B: usize = 0x1d;
const TS_WAGE: usize = 0xc;

/// The news item `id_a`/`id_b` copied into `item` (a NewsItem built empty, then `News::find_item_by_id`).
unsafe fn ts_find_news_item(item: &mut [u8; 0xe0], id_a: u32, id_b: u32) {
    let find: unsafe extern "thiscall" fn(usize, u32, u32, usize) -> usize = core::mem::transmute(TS_FN_NEWS_FIND_ITEM);
    let n = item.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0, 0);
    find(read_usize(G_NEWS), id_a, id_b, n);
}

/// Opens the offer changed screen for `offer` (params 0 the offer, 1 the serial the news item knew).
unsafe fn ts_open_offer_changed(offer: usize, serial: u32) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, TS_OFFER_CHANGED_BUILD, TS_OFFER_CHANGED_INPUT, 1, 0usize, 0) != 0 {
        set_screen_param(0, offer);
        set_screen_param(1, serial as usize);
    }
}

/// Answers the Respond button of a received bid: the news item's slots 6 (the offer slot index) and 7 (the offer's
/// serial then); when the offer still has that serial and was not withdrawn (status 3), the bid screen opens for it
/// (`transfer_screens_open_bid_screen_for_offer(slot, 1)`); otherwise the offer changed screen.
/// Original: `transfer_screens_news_respond_bid_action` (0x8f1730, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_news_respond_bid_action(id_a: u32, id_b: u32) {
    covered!("tel17_3 news respond bid");
    let open_bid: unsafe extern "C" fn(usize, u32) = core::mem::transmute(TS_FN_OPEN_BID_SCREEN_FOR_OFFER);
    let mut item = [0u8; 0xe0];
    ts_find_news_item(&mut item, id_a, id_b);
    let n = item.as_mut_ptr() as usize;
    let index = exe::newsitem_get_slot(n, 6);
    let serial = exe::newsitem_get_slot(n, 7);
    let slot = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), index as i32);
    let o = read_usize(slot);
    if read_u32(o + offer::SERIAL) == serial && read_u8(o + offer::STATUS) != 3 {
        open_bid(slot, 1);
    } else {
        ts_open_offer_changed(o, serial);
    }
}

/// Comparer of `transfer_screens_news_respond_bid_action`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_news_respond_bid_action(id_a: u32, id_b: u32) {
    original_fn::<unsafe extern "C" fn(u32, u32)>(%%I0%%)(id_a, id_b)
}

/// Answers the Respond button of a counter offer: slots 6 and 7 as above; when the offer still has the serial and is
/// neither withdrawn (3) nor rejected (2), the bid screen opens for its target with the offer
/// (`transfer_screens_open_bid_screen_for_player(player, slot)`); otherwise the offer changed screen.
/// Original: `transfer_screens_news_respond_counter_offer_action` (0x8f1830, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_news_respond_counter_offer_action(id_a: u32, id_b: u32) {
    covered!("tel17_3 news respond counter offer");
    let open_bid: unsafe extern "C" fn(usize, usize) = core::mem::transmute(TS_FN_OPEN_BID_SCREEN_FOR_PLAYER);
    let mut item = [0u8; 0xe0];
    ts_find_news_item(&mut item, id_a, id_b);
    let n = item.as_mut_ptr() as usize;
    let index = exe::newsitem_get_slot(n, 6);
    let serial = exe::newsitem_get_slot(n, 7);
    let slot = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), index as i32);
    let o = read_usize(slot);
    let status = read_u8(o + offer::STATUS);
    if read_u32(o + offer::SERIAL) == serial && status != 3 && status != 2 {
        open_bid(staff_at(read_i32(o + offer::TARGET_STAFF_ID) as isize).addr(), slot);
    } else {
        ts_open_offer_changed(o, serial);
    }
}

/// Comparer of `transfer_screens_news_respond_counter_offer_action`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_news_respond_counter_offer_action(id_a: u32, id_b: u32) {
    original_fn::<unsafe extern "C" fn(u32, u32)>(%%I1%%)(id_a, id_b)
}

/// Answers the Offer Contract button of an accepted bid: slots 7 (the offer slot index), 8 (its serial then) and 0
/// (the player id); when the offer still has the serial and was not withdrawn, contract talks open with the player
/// (`contract_screens_offer_contract_to_staff(player, slot, 1, 0)`); otherwise the offer changed screen.
/// Original: `transfer_screens_news_offer_contract_action` (0x8f1950, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_news_offer_contract_action(id_a: u32, id_b: u32) {
    covered!("tel17_3 news offer contract");
    let talks: unsafe extern "C" fn(usize, usize, u32, u32) = core::mem::transmute(TS_FN_OFFER_CONTRACT_TO_STAFF);
    let mut item = [0u8; 0xe0];
    ts_find_news_item(&mut item, id_a, id_b);
    let n = item.as_mut_ptr() as usize;
    let index = exe::newsitem_get_slot(n, 7);
    let serial = exe::newsitem_get_slot(n, 8);
    let player = staff_at(exe::newsitem_get_slot(n, 0) as i32 as isize).addr();
    let slot = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), index as i32);
    let o = read_usize(slot);
    if read_u32(o + offer::SERIAL) == serial && read_u8(o + offer::STATUS) != 3 {
        talks(player, slot, 1, 0);
    } else {
        ts_open_offer_changed(o, serial);
    }
}

/// Comparer of `transfer_screens_news_offer_contract_action`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_news_offer_contract_action(id_a: u32, id_b: u32) {
    original_fn::<unsafe extern "C" fn(u32, u32)>(%%I2%%)(id_a, id_b)
}

/// Answers the Respond button of a transfer delay request: the offer slot of the news item's slot 9; the delay request
/// dialog opens with params 0 and 1 the news item ids and 2 the offer slot.
/// Original: `transfer_screens_news_respond_delay_request_action` (0x8f1a60, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_news_respond_delay_request_action(id_a: u32, id_b: u32) {
    covered!("tel17_3 news respond delay request");
    let mut item = [0u8; 0xe0];
    ts_find_news_item(&mut item, id_a, id_b);
    let index = exe::newsitem_get_slot(item.as_mut_ptr() as usize, 9);
    let slot = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), index as i32);
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, TS_DELAY_REQUEST_BUILD, TS_DELAY_REQUEST_INPUT, 1, 0usize, 0) != 0 {
        set_screen_param(0, id_a as usize);
        set_screen_param(1, id_b as usize);
        set_screen_param(2, slot);
    }
}

/// Comparer of `transfer_screens_news_respond_delay_request_action`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_news_respond_delay_request_action(id_a: u32, id_b: u32) {
    original_fn::<unsafe extern "C" fn(u32, u32)>(%%I3%%)(id_a, id_b)
}

/// Answers the Re-Negotiate button of a bid: slots 0xb (the offer slot index) and 0xc (its serial then); when the offer
/// still has the serial and is neither withdrawn nor rejected, the bid screen opens for its target with the live offer,
/// otherwise with a fresh bid (`transfer_screens_open_bid_screen_for_player(player, slot or 0)`).
/// Original: `transfer_screens_news_renegotiate_bid_action` (0x8f1b40, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_news_renegotiate_bid_action(id_a: u32, id_b: u32) {
    covered!("tel17_3 news renegotiate bid");
    let open_bid: unsafe extern "C" fn(usize, usize) = core::mem::transmute(TS_FN_OPEN_BID_SCREEN_FOR_PLAYER);
    let mut item = [0u8; 0xe0];
    ts_find_news_item(&mut item, id_a, id_b);
    let n = item.as_mut_ptr() as usize;
    let index = exe::newsitem_get_slot(n, 0xb);
    let serial = exe::newsitem_get_slot(n, 0xc);
    let slot = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), index as i32);
    let o = read_usize(slot);
    let player = staff_at(read_i32(o + offer::TARGET_STAFF_ID) as isize).addr();
    let status = read_u8(o + offer::STATUS);
    if read_u32(o + offer::SERIAL) == serial && status != 3 && status != 2 {
        open_bid(player, slot);
    } else {
        open_bid(player, 0);
    }
}

/// Comparer of `transfer_screens_news_renegotiate_bid_action`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_news_renegotiate_bid_action(id_a: u32, id_b: u32) {
    original_fn::<unsafe extern "C" fn(u32, u32)>(%%I4%%)(id_a, id_b)
}

/// Answers the Respond button of a loan bid.
///
/// Slots 6 (the offer slot index) and 7 (its serial then). When the offer changed or was withdrawn: the offer changed
/// screen. Otherwise the loan bid screen opens (build 0x8f9a20, input 0x8fac30, flag 1) and its params are filled: 0
/// the player; the player's contract (`contract_manager_get_contract`, not stored in the lookup global) and the
/// contract offer of the buyer (`contract_manager_find_contract_offer(player id, buyer id)`); without that offer the
/// transfer_screens.cpp assert box, line 0x237f. 1 and 2 the contract's wage; the loan period of the buyer, the
/// player's contract club and the player (`TRANSFER_MANAGER::get_loan_period`): 3 the first byte, 4 the first word, 5
/// the second byte, 6 the second word; 7 the offer, 8 the contract offer, 9 the offer's serial, 0xa 0, 0xb and 0x10 the
/// wage share (the offered wage over the contract's, in percent, rounded to the nearest 10), 0xc and 0x11 the month of
/// the contract offer's expiry date, 0xd and 0x12 its year word, 0xe/0x13 and 0xf/0x14 its two clause bytes.
/// Original: `transfer_screens_news_respond_loan_bid_action` (0x8f1c30, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_news_respond_loan_bid_action(id_a: u32, id_b: u32) {
    let mut item = [0u8; 0xe0];
    ts_find_news_item(&mut item, id_a, id_b);
    let n = item.as_mut_ptr() as usize;
    let index = exe::newsitem_get_slot(n, 6);
    let serial = exe::newsitem_get_slot(n, 7);
    let slot = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), index as i32);
    let o = read_usize(slot);
    if read_u32(o + offer::SERIAL) != serial || read_u8(o + offer::STATUS) == 3 {
        ts_open_offer_changed(o, serial);
        return;
    }
    let player = staff_at(read_i32(o + offer::TARGET_STAFF_ID) as isize).addr();
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, TS_LOAN_BID_BUILD, TS_LOAN_BID_INPUT, 1, 0usize, 0) == 0 {
        return;
    }
    set_screen_param(0, player);
    let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
    let terms = exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), read_u32(player), read_u32(o + offer::BUYER_CLUB_ID));
    if terms == 0 {
        covered!("tel17_3 news respond loan bid without contract offer");
        tel26_error_box(TS_CPP, 0x237f);
        return;
    }
    covered!("tel17_3 news respond loan bid");
    let share = ((read_i32(terms + TS_WAGE).wrapping_mul(100) / read_i32(contract + TS_WAGE)).wrapping_add(5) / 10).wrapping_mul(10);
    set_screen_param(1, read_u32(contract + TS_WAGE) as usize);
    set_screen_param(2, read_u32(contract + TS_WAGE) as usize);
    let contract_club = contract_club_cached(Staff(player));
    let (mut first_byte, mut first_word, mut second_byte, mut second_word) = (0i8, 0u16, 0i8, 0u16);
    exe::transfer_manager_get_loan_period(read_usize(G_TRANSFER_MANAGER), club_at(read_i32(o + offer::BUYER_CLUB_ID) as isize).addr(), contract_club,
        player, &mut first_byte, &mut first_word, &mut second_byte, &mut second_word);
    set_screen_param(3, first_byte as i32 as usize);
    set_screen_param(4, first_word as usize);
    set_screen_param(5, second_byte as i32 as usize);
    set_screen_param(6, second_word as usize);
    set_screen_param(7, o);
    set_screen_param(8, terms);
    set_screen_param(9, read_u32(o + offer::SERIAL) as usize);
    set_screen_param(0xa, 0);
    set_screen_param(0xb, share as usize);
    let expiry = terms + TS_CONTRACT_OFFER_EXPIRY;
    set_screen_param(0xc, exe::date_get_month(expiry) as i8 as i32 as usize);
    set_screen_param(0xd, read_u16(terms + TS_CONTRACT_OFFER_EXPIRY_YEAR) as usize);
    set_screen_param(0xe, read_i8(terms + TS_CONTRACT_OFFER_CLAUSE_A) as i32 as usize);
    set_screen_param(0xf, read_i8(terms + TS_CONTRACT_OFFER_CLAUSE_B) as i32 as usize);
    set_screen_param(0x10, share as usize);
    set_screen_param(0x11, exe::date_get_month(expiry) as i8 as i32 as usize);
    set_screen_param(0x12, read_u16(terms + TS_CONTRACT_OFFER_EXPIRY_YEAR) as usize);
    set_screen_param(0x13, read_i8(terms + TS_CONTRACT_OFFER_CLAUSE_A) as i32 as usize);
    set_screen_param(0x14, read_i8(terms + TS_CONTRACT_OFFER_CLAUSE_B) as i32 as usize);
}

/// Comparer of `transfer_screens_news_respond_loan_bid_action`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_news_respond_loan_bid_action(id_a: u32, id_b: u32) {
    original_fn::<unsafe extern "C" fn(u32, u32)>(%%I5%%)(id_a, id_b)
}

/// Builds the dialog that asks whether to accept a one week delay in a transfer.
///
/// Param 2 is the offer slot; the player is its target, the selling club his contract club (a contract of type 5 counts
/// as none), the buyer `offer::BUYER_CLUB_ID`'s. The player's picture, the title in the human's club colours, the
/// subtitle "Transfer Bid for <player>", the text (0xa9f724: the selling club and the buyer by their full names), the
/// menu bar and the bar Back (id 1), Reject (id 2), Accept (id 3) (weights 2:1:1).
/// Original: `transfer_screens_build_delay_request_dialog` (0x8f2010, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_build_delay_request_dialog() {
    covered!("tel17_3 delay request dialog");
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let human = tel26_slot_manager().addr();
    let weights: [u8; 3] = [2, 1, 1];
    let o = read_usize(get_screen_param(2));
    let player = staff_at(read_i32(o + offer::TARGET_STAFF_ID) as isize).addr();
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    let seller = if c == 0 || read_u8(c + 0x35) & 0x3f == 5 { 0 } else { club_at(read_i32(c + 4) as isize).addr() };
    let buyer = club_at(read_i32(o + offer::BUYER_CLUB_ID) as isize).addr();
    picture(player, 0, 0);
    ts_title_in_club_colours(read_usize(human + staff::CLUB_JOB));
    let mut name = [0u8; 300];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(player, n, 300, 1);
    lang(TS_TEXT, 0xa9f7c4, n); // "Transfer Bid for <player>"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    let lines = ts_dialog_lines();
    let gs = read_i8(seller + club::SHORT_GENDER_NAME) as i32;
    let gb = read_i8(buyer + club::SHORT_GENDER_NAME) as i32;
    lang(TS_TEXT, 0xa9f724, 100, gs, gs, gb, gb, n, seller + club::NAME, buyer + club::NAME);
    ts_dialog_text(lines);
    exe::menubar_build(4, 0);
    let bar = ts_button_bar(3, weights.as_ptr());
    ts_bar_button(bar, 0, TS_BACK, 1);
    ts_bar_button(bar, 1, 0xa9e3c4, 2); // "Reject"
    ts_bar_button(bar, 2, 0xa9e3bc, 3); // "Accept"
}

/// Comparer of `transfer_screens_build_delay_request_dialog`: side by side over the display list and the contract
/// lookup global.
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_delay_request_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I6%%);
    compare_by_display_list_with("Delay request dialog", &[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(), || transfer_screens_build_delay_request_dialog());
}

/// Answers the delay request dialog (params 0 and 1 the news item ids, 2 the offer slot). Accept (id 3): the news
/// item's slots 7 and 8 are set to 1 (`News::set_item_slot`), -9. Reject (id 2): slot 7 = 1 and slot 8 = 0, then the
/// offer is withdrawn (status 0x11) or rejected, with reason 6, -9. Back (id 1) -9; the rest to the menu bar.
/// Original: `transfer_screens_handle_delay_request_action` (0x8f22c0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_handle_delay_request_action(item: u32) -> i32 {
    let set_slot: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_NEWS_SET_ITEM_SLOT);
    let id_a = get_screen_param(0);
    let id_b = get_screen_param(1);
    let id = tel26_item_id(item as i16);
    if id == 3 {
        covered!("tel17_3 delay request Accept");
        set_slot(read_usize(G_NEWS), id_a, id_b, 7, 1, 0);
        set_slot(read_usize(G_NEWS), id_a, id_b, 8, 1, 0);
        return -9;
    }
    if id == 2 {
        covered!("tel17_3 delay request Reject");
        set_slot(read_usize(G_NEWS), id_a, id_b, 7, 1, 0);
        set_slot(read_usize(G_NEWS), id_a, id_b, 8, 0, 0);
        let slot = get_screen_param(2);
        let tm = read_usize(G_TRANSFER_MANAGER);
        if read_u8(read_usize(slot) + offer::STATUS) == 0x11 {
            exe::transfer_manager_withdraw_transfer_offer(tm, slot as *mut usize, 0, 6);
        } else {
            exe::transfer_manager_reject_transfer_offer(tm, slot as *mut usize, 0, 6);
        }
        return -9;
    }
    if id == 1 {
        return -9;
    }
    ts_menubar_answer(item)
}

/// Comparer of `transfer_screens_handle_delay_request_action`: Back side by side; Accept, Reject (news item and offer)
/// and the menu bar only in the original.
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_delay_request_action(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I7%%);
    ts_compare_answer("Delay request dialog", item, &[1], o, transfer_screens_handle_delay_request_action)
}
