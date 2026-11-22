// ---------- batch %%LEVA%%: the final transfer confirmation, the shortlist and comparison prompts and the withdraw bid dialog (transfer_screens.cpp, part 2: tel18_1) ----------

/// `HUMAN_MANAGER::get_confirm_offer_slot(0)` (0x5fdb30, thiscall on [G_HUMAN_MANAGER] ret 4): the slot of the copy of
/// the offer being confirmed.
pub(crate) const TS_FN_CONFIRM_OFFER_SLOT: usize = 0x5fdb30;
/// `SCOUT_MANAGER::set_compare_player(staff, 0, which)` (0x7f7780, thiscall on [G_SCOUT_MANAGER] ret 0xc).
const TS_FN_SET_COMPARE_PLAYER: usize = 0x7f7780;
/// `SCOUT_MANAGER::remove_from_shortlist(staff, 0)` (0x7f7a20, thiscall on [G_SCOUT_MANAGER] ret 8).
const TS_FN_REMOVE_FROM_SHORTLIST: usize = 0x7f7a20;
/// "Cancel" and "Delay".
pub(crate) const TS_CANCEL: usize = 0x98a1a0;
pub(crate) const TS_DELAY: usize = 0xa9e3b4;

/// `News::set_item_slot(id a, id b, slot, value, 0)` (0x786d30, thiscall on [G_NEWS] ret 0x14), keeping eax.
pub(crate) unsafe fn ts_news_slot(id_a: usize, id_b: usize, slot: u32, value: u32) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_NEWS_SET_ITEM_SLOT);
    f(read_usize(G_NEWS), id_a, id_b, slot, value, 0)
}

/// The dword the original pushes for a sign-extended byte moved into the low word of a register (`movsx r16, byte`):
/// the byte as a word over the register's high half.
pub(crate) fn ts_word_over(high: u32, b: u8) -> u32 {
    (high & 0xffff_0000) | (b as i8 as i16 as u16) as u32
}

/// Opens the must accept dialog for the live offer `live` (`transfer_screens_build_offer_must_accept_dialog`, input
/// `transfer_screens_handle_back_dialog_action`, flag 1): params 0 the slot, 1 the action tried, 2 the reason
/// (sign-extended word). Answers -4 whether it opened or not.
pub(crate) unsafe fn ts_open_must_accept(live: usize, action: u32, why: u32) -> i32 {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8f0bc0usize, 0x8f16c0usize, 1, 0usize, 0) != 0 {
        set_screen_param(0, live);
        set_screen_param(1, action as usize);
        set_screen_param(2, why as i16 as i32 as usize);
    }
    -4
}

/// Opens the final confirmation screen of a transfer (build `transfer_screens_build_confirm_transfer_screen`, input
/// `transfer_screens_handle_confirm_transfer_action`, flag 1) and, when it opens, sets its params: 0 the live offer
/// slot, 1 and 2 the news item's ids, 3 0, 4 the offer's serial (`offer::SERIAL`), 5 the player's id
/// (`offer::TARGET_STAFF_ID`), 6 0.
/// Original: `transfer_screens_open_confirm_transfer_screen` (0x8f2e10, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_confirm_transfer_screen(slot: usize, id_a: u32, id_b: u32) {
    covered!("tel18_1 open confirm transfer screen");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8f2ec0usize, 0x8f3c70usize, 1, 0usize, 0) != 0 {
        set_screen_param(0, slot);
        set_screen_param(1, id_a as usize);
        set_screen_param(2, id_b as usize);
        set_screen_param(3, 0);
        set_screen_param(4, read_u32(read_usize(slot) + offer::SERIAL) as usize);
        set_screen_param(5, read_u32(read_usize(slot) + offer::TARGET_STAFF_ID) as usize);
        set_screen_param(6, 0);
    }
}

/// Comparer of `transfer_screens_open_confirm_transfer_screen`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_confirm_transfer_screen(slot: usize, id_a: u32, id_b: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32, u32)>(%%I0%%)(slot, id_a, id_b)
}

/// Builds the final confirmation screen of a transfer.
///
/// The human's confirm copy (`HUMAN_MANAGER::get_confirm_offer_slot(0)`) is created when empty (`operator new` 0x41,
/// `TRANSFER_OFFER(player of param 5, the human's club, 0, -1, 0)`) and filled from the live offer of param 0
/// (`ts_copy_offer`, first dword cleared, exchange list copied). The buyer is the copy's `offer::BUYER_CLUB_ID`, the
/// seller the player's contract club (none for a contract of type 5). The player's picture, the title in the human's
/// club colours; `TRANSFER_MANAGER::check_offer_restrictions(copy, the human's club, &reason, 1)` gives the state (param
/// 6) and the reason (param 3); the subtitle is "Confirm Transfer Offer" (state 0), "Delay Transfer Offer" (1) or
/// "Transfer Cancelled".
///
/// State 0: the exchange players (at most 5 named; a sixth shows the assert box, line 0x15e1) are joined into one text
/// ("<a>", "<a> and <b>", ... up to five). The money shown is the fee plus both appearance fees plus the sell-on
/// percentage of the player's value (`staff::VALUE`, truncated division by 100) when that total is above the fee,
/// else the fee (cash display mode 6). The text: with a positive total over the fee "Buy/Sell <player> for <money>
/// [plus <exchange>]" from the seller (the human is the buyer) or to the buyer; with the fee alone, the same wording
/// when the fee is positive, otherwise "Sign <player> from <seller>?" ("Sign <player>?" without a seller), "Sell
/// <player> to <buyer>?" or, with exchange players, the exchange wordings.
/// Other states: the text by the reason: 3 (with the player), 4, 0x22, 0x23 the buyer, 5 the seller, and 0x17, 0x18,
/// 0xe, 0x14, 0x15, 0xc, 0xd, 0xf, 0x10, 0x11 the human's club (the transfer restrictions); another reason keeps the
/// text.
/// The text area, the menu bar and the bar: state 2, Back (id 2) alone; state 1, Back (2) / Cancel (1) / Delay (4);
/// state 0, Back (2) / No (1) / Delay (4) / Yes (3).
/// Original: `transfer_screens_build_confirm_transfer_screen` (0x8f2ec0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_build_confirm_transfer_screen() {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let confirm_slot: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(TS_FN_CONFIRM_OFFER_SLOT);
    let new_offer: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> usize = core::mem::transmute(TS_FN_NEW_TRANSFER_OFFER);
    let live_slot = get_screen_param(0);
    let slot = confirm_slot(read_usize(G_HUMAN_MANAGER), 0);
    let human = ts_human_club();
    let player = staff_at(get_screen_param(5) as i32 as isize).addr();
    if read_usize(slot) == 0 {
        covered!("tel18_1 confirm transfer: new confirm copy");
        let mem = exe::crt_operator_new(0x41);
        let o = if mem != 0 { new_offer(mem, player, human, 0, -1, 0) } else { 0 };
        write_usize(slot, o);
        let src = read_usize(live_slot);
        ts_copy_offer(o, src);
        ts_copy_offer_list(o, read_usize(read_usize(live_slot) + offer::EXCHANGE_PLAYERS));
    }
    let o = read_usize(slot);
    let buyer = club_at(read_i32(o + offer::BUYER_CLUB_ID) as isize).addr();
    let seller = ts_contract_club_not_type_5(player);
    picture(player, 0, 0);
    ts_title_in_club_colours(human);
    let mut reason: u32 = 0;
    let state = exe::transfer_manager_check_offer_restrictions(read_usize(G_TRANSFER_MANAGER), o, human, &mut reason, 1);
    let reason = reason as u16;
    set_screen_param(3, reason as i16 as i32 as usize);
    set_screen_param(6, state as i8 as i32 as usize);
    lang(TS_TEXT, match state { 0 => 0xaa097c, 1 => 0xaa0964, _ => 0xaa0950 });
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    let mut name = [0u8; 0x190];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(staff_at(read_i32(o + offer::TARGET_STAFF_ID) as isize).addr(), n, 0x190, 1);
    let g = |c: usize| read_i8(c + club::SHORT_GENDER_NAME) as i32;
    let lg = |c: usize| read_i8(c + club::GENDER_NAME) as i32;
    if state == 0 {
        covered!("tel18_1 confirm transfer screen");
        let mut names = [[0u8; 0x190]; 5];
        let mut exchange = [0u8; 0x190];
        let x = exchange.as_mut_ptr() as usize;
        let mut count: i8 = 0;
        let list = read_usize(o + offer::EXCHANGE_PLAYERS);
        if list != 0 {
            write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
            loop {
                let node = exe::llist_get_next(list);
                if node == 0 {
                    break;
                }
                if count < 5 {
                    let s = staff_at(read_i32(node) as isize).addr();
                    exe::database_format_staff_name(s, names[count as usize].as_mut_ptr() as usize, 0x190, 1);
                    count += 1;
                } else {
                    tel26_error_box(TS_CPP, 0x15e1);
                }
            }
            let a = |k: usize| names[k].as_ptr() as usize;
            match count {
                1 => copy_c_string(x, a(0)),
                2 => {
                    lang(x, 0xaa0904, a(0), a(1));
                }
                3 => {
                    lang(x, 0xaa0894, a(0), a(1), a(2));
                }
                4 => {
                    lang(x, 0xaa0804, a(0), a(1), a(2), a(3));
                }
                5 => {
                    lang(x, 0xaa0750, a(0), a(1), a(2), a(3), a(4));
                }
                _ => {}
            }
        }
        let fee = read_i32(o + offer::FEE);
        let sell_on = (read_i8(o + offer::SELL_ON_PERCENT) as i32).wrapping_mul(read_i32(player + staff::VALUE)) / 100;
        let total = sell_on.wrapping_add(read_i32(o + offer::INTERNATIONAL_FEE)).wrapping_add(read_i32(o + offer::APPEARANCE_FEE)).wrapping_add(fee);
        let mut money = [0u8; 0x32];
        let m = money.as_mut_ptr() as usize;
        let we_buy = read_u32(human) == read_u32(o + offer::BUYER_CLUB_ID);
        let other = if we_buy { seller } else { buyer };
        if total > fee {
            exe::cash_format_money(m, 0x32, ts_display(total as u32, 6), 6);
            if count == 0 {
                lang(TS_TEXT, if we_buy { 0xaa06dc } else { 0xaa0668 }, g(other), g(other), n, other + club::SHORT_NAME, m);
            } else {
                lang(TS_TEXT, if we_buy { 0xaa05a8 } else { 0xaa04ec }, g(other), g(other), n, other + club::SHORT_NAME, m, x);
            }
        } else {
            exe::cash_format_money(m, 0x32, ts_display(fee as u32, 6), 6);
            if count == 0 {
                if fee > 0 {
                    lang(TS_TEXT, if we_buy { 0xaa047c } else { 0xaa040c }, g(other), g(other), n, other + club::SHORT_NAME, m);
                } else if !we_buy {
                    lang(TS_TEXT, 0xaa0340, g(buyer), g(buyer), n, buyer + club::SHORT_NAME);
                } else if seller != 0 {
                    lang(TS_TEXT, 0xaa03bc, g(seller), g(seller), n, seller + club::SHORT_NAME);
                } else {
                    lang(TS_TEXT, 0xaa0390, n);
                }
            } else if fee > 0 {
                lang(TS_TEXT, if we_buy { 0xaa0294 } else { 0xaa01e8 }, g(other), g(other), n, other + club::SHORT_NAME, m, x);
            } else {
                lang(TS_TEXT, if we_buy { 0xaa014c } else { 0xaa00b4 }, g(other), g(other), n, other + club::SHORT_NAME, x);
            }
        }
    } else {
        covered!("tel18_1 confirm transfer screen: restriction");
        match reason {
            3 => {
                lang(TS_TEXT, 0xaa000c, lg(buyer), lg(buyer), 100, buyer + club::NAME, n);
            }
            4 => {
                lang(TS_TEXT, 0xa9ff74, lg(buyer), lg(buyer), buyer + club::NAME);
            }
            0x22 | 0x23 => {
                lang(TS_TEXT, 0xa9fedc, lg(buyer), lg(buyer), buyer + club::NAME);
            }
            5 => {
                lang(TS_TEXT, 0xa9ff74, lg(seller), lg(seller), seller + club::NAME);
            }
            0x17 | 0x18 | 0xe | 0x14 | 0x15 | 0xc | 0xd | 0xf | 0x10 | 0x11 => {
                let fmt = match reason {
                    0x17 => 0xa9fe60,
                    0x18 => 0xa9fdd8,
                    0xe => 0xa9fd7c,
                    0x14 => 0xa9fd24,
                    0x15 => 0xa9fccc,
                    0xc => 0xa9fc68,
                    0xd => 0xa9fc00,
                    0xf => 0xa9fb94,
                    0x10 => 0xa9fb3c,
                    _ => 0xa9fae0,
                };
                lang(TS_TEXT, fmt, lg(human), lg(human), human + club::NAME);
            }
            _ => {}
        }
    }
    ts_dialog_text(ts_dialog_lines());
    exe::menubar_build(4, 0);
    if state == 2 {
        let bar = ts_button_bar(1, core::ptr::null());
        ts_bar_button(bar, 0, TS_BACK, 2);
    } else if state == 1 {
        let weights: [u8; 3] = [2, 1, 1];
        let bar = ts_button_bar(3, weights.as_ptr());
        ts_bar_button(bar, 0, TS_BACK, 2);
        ts_bar_button(bar, 1, TS_CANCEL, 1);
        ts_bar_button(bar, 2, TS_DELAY, 4);
    } else {
        let weights: [u8; 4] = [3, 1, 1, 1];
        let bar = ts_button_bar(4, weights.as_ptr());
        ts_bar_button(bar, 0, TS_BACK, 2);
        ts_bar_button(bar, 1, TS_NO, 1);
        ts_bar_button(bar, 2, TS_DELAY, 4);
        ts_bar_button(bar, 3, TS_YES, 3);
    }
}

/// Comparer of `transfer_screens_build_confirm_transfer_screen`: side by side over the display list, the screen params
/// and the contract lookup global; only the original while the confirm copy is still to be created (operator new).
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_confirm_transfer_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    let confirm_slot: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(TS_FN_CONFIRM_OFFER_SLOT);
    if read_usize(confirm_slot(read_usize(G_HUMAN_MANAGER), 0)) == 0 {
        // simplification: the allocation of the confirm copy does not run twice
        return o();
    }
    compare_by_display_list_with("Confirm transfer screen", &[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(), || transfer_screens_build_confirm_transfer_screen());
}

/// Answers the final confirmation screen of a transfer.
///
/// Params: 0 the live offer slot, 1 and 2 the news item's ids, 3 the restriction reason, 4 the serial, 6 the state.
/// Back (id 2): with state 2 the offer is collapsed (`collapse_offer(offer, the human's club, reason, 1)`); the confirm
/// copy is deleted, -9. No (1), Delay (4) and Yes (3) first check the offer: when its serial changed or it was rejected
/// (status 2) or withdrawn (3), the confirm copy is deleted and the offer changed screen opens (offer, serial), -11.
/// No: the human's club withdraws its own bid (`withdraw_transfer_offer(slot, 0, reason)`); else, when the selling club
/// must accept (`transfer_manager_selling_club_must_accept`, the reason starting at 0), the must accept dialog
/// (action 0x1b, -4), otherwise the offer is rejected with that reason. Delay: the human's own bid marks news slot 9 =
/// 0x12 and the club rejects it (`club_rejects_offer(offer, club, reason)`); else, with the selling club bound to
/// accept (the reason starting at the item argument), the must accept dialog (0x23, -4), otherwise news slot 9 = 0x11
/// and the club rejects it. Yes: `confirm_offer_by_club(offer, the human's club)`. All three then delete the confirm
/// copy, -9. The rest goes to the menu bar.
/// Original: `transfer_screens_handle_confirm_transfer_action` (0x8f3c70, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_handle_confirm_transfer_action(item: u32) -> i32 {
    let confirm_slot: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(TS_FN_CONFIRM_OFFER_SLOT);
    let must_accept: unsafe extern "thiscall" fn(usize, usize, usize, *mut u32) -> u32 = core::mem::transmute(TS_FN_SELLING_CLUB_MUST_ACCEPT);
    let slot = confirm_slot(read_usize(G_HUMAN_MANAGER), 0);
    let club = ts_human_club();
    let live = get_screen_param(0);
    let reason = get_screen_param(3) as u8;
    let serial = get_screen_param(4) as u32;
    let state = get_screen_param(6) as u32;
    let tm = read_usize(G_TRANSFER_MANAGER);
    let id = tel26_item_id(item as i16);
    let drop_and_back = || {
        ts_delete_offer(slot);
        -9
    };
    if id == 2 {
        if state as u8 == 2 {
            covered!("tel18_1 confirm transfer: collapse");
            exe::transfer_manager_collapse_offer(tm, read_usize(live), club, ts_word_over(state, reason), 1);
        }
        return drop_and_back();
    }
    if matches!(id, 1 | 4 | 3) {
        let o = read_usize(live);
        if read_u32(o + offer::SERIAL) != serial || matches!(read_u8(o + offer::STATUS), 2 | 3) {
            covered!("tel18_1 confirm transfer: offer changed");
            ts_delete_offer(slot);
            let o = read_usize(live);
            if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8f24c0usize, 0x8f2800usize, 1, 0usize, 0) != 0 {
                set_screen_param(0, o);
                set_screen_param(1, serial as usize);
            }
            return -11;
        }
    }
    if id == 1 {
        let o = read_usize(live);
        if club_at(read_i32(o + offer::BUYER_CLUB_ID) as isize).addr() == club {
            covered!("tel18_1 confirm transfer No: withdraw");
            exe::transfer_manager_withdraw_transfer_offer(tm, live as *mut usize, 0, ts_word_over(o as u32, reason));
            return drop_and_back();
        }
        let mut why: u32 = 0;
        if must_accept(tm, o, 0, &mut why) != 0 {
            return ts_open_must_accept(live, 0x1b, why);
        }
        covered!("tel18_1 confirm transfer No: reject");
        exe::transfer_manager_reject_transfer_offer(tm, live as *mut usize, 0, why);
        return drop_and_back();
    }
    if id == 4 {
        let id_a = get_screen_param(1);
        let id_b = get_screen_param(2);
        let o = read_usize(live);
        let eax = if read_u32(club) == read_u32(o + offer::BUYER_CLUB_ID) {
            ts_news_slot(id_a, id_b, 9, 0x12)
        } else {
            // the original passes its own argument slot as the reason: it starts as the item argument
            let mut why: u32 = item;
            if must_accept(tm, o, 0, &mut why) != 0 {
                return ts_open_must_accept(live, 0x23, why);
            }
            ts_news_slot(id_a, id_b, 9, 0x11)
        };
        covered!("tel18_1 confirm transfer Delay");
        exe::transfer_manager_club_rejects_offer(tm, read_usize(live), club, ts_word_over(eax, reason));
        return drop_and_back();
    }
    if id == 3 {
        covered!("tel18_1 confirm transfer Yes");
        exe::transfer_manager_confirm_offer_by_club(tm, read_usize(live), club);
        return drop_and_back();
    }
    ts_menubar_answer(item)
}

/// Comparer of `transfer_screens_handle_confirm_transfer_action`: only the original runs (every button changes the
/// offer, deletes the confirm copy or opens a screen; the menu bar opens screens).
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_confirm_transfer_action(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%);
    ts_compare_answer("Confirm transfer screen", item, &[], o, transfer_screens_handle_confirm_transfer_action)
}

/// The Yes/No dialogs about one player (shortlist, comparison): param 0 the player's id, param 1 the choice. The
/// player's picture, the title (the slot manager's name, in the colours of his club job, else his national job, else
/// blue on pale grey), the subtitle `subtitle`, the text `text` with the player's name as a link (0x7d7), the menu bar
/// and the bar No (`no_id`) / Yes (`yes_id`).
unsafe fn ts_player_yes_no_dialog(subtitle: usize, text: usize, no_id: u32, yes_id: u32) {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let text_area: TsTextAreaLinks = core::mem::transmute(0x5ed070usize);
    let mut pale = read_u16(G_GUI_COLOUR_PALE_GREY) as u32;
    let mut blue = read_u16(G_GUI_COLOUR_BLUE) as u32;
    let player = staff_at(get_screen_param(0) as i32 as isize).addr();
    let manager = tel26_slot_manager().addr();
    let mut club = read_usize(manager + staff::CLUB_JOB);
    if club == 0 {
        club = read_usize(manager + staff::NATIONAL_JOB);
    }
    picture(player, 0, 0);
    if club != 0 {
        exe::database_get_club_kit_colours(club, 1, &mut blue, &mut pale);
    }
    exe::database_format_staff_name(manager, TS_TEXT, 0x7d0, 1);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, pale, blue, TS_TEXT);
    lang(TS_TEXT, subtitle);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    let mut name = [0u8; 0x190];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(player, n, 0x190, 1);
    lang(TS_TEXT, text, n);
    let lines = ts_dialog_lines();
    text_area(0x6e, 0x8c, 0x30c, 0x221, 2, 0, lines as i8 as i32 as u32, 3, ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT, n, 0x7d7u32, player, 0u32);
    exe::menubar_build(4, 0);
    let weights: [u8; 2] = [3, 1];
    let bar = ts_button_bar(2, weights.as_ptr());
    ts_bar_button(bar, 0, TS_NO, no_id);
    ts_bar_button(bar, 1, TS_YES, yes_id);
}

/// Opens the dialog that adds a player to, or removes him from, the shortlist (build
/// `transfer_screens_build_shortlist_dialog`, input `transfer_screens_handle_shortlist_dialog`, flag 1): param 0 the
/// player's id (the dword at `player_id`), param 1 `add`.
/// Original: `transfer_screens_open_shortlist_dialog` (0x8f4160, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_shortlist_dialog(player_id: usize, add: u32) {
    covered!("tel18_1 open shortlist dialog");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8f4650usize, 0x8f48b0usize, 1, 0usize, 0) != 0 {
        set_screen_param(0, read_u32(player_id) as usize);
        set_screen_param(1, add as usize);
    }
}

/// Comparer of `transfer_screens_open_shortlist_dialog`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_shortlist_dialog(player_id: usize, add: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32)>(%%I3%%)(player_id, add)
}

/// Opens the dialog that sets a player as the first or second in the comparison (build
/// `transfer_screens_build_comparison_prompt`, input `transfer_screens_handle_comparison_prompt_action`, flag 1): param
/// 0 the player's id (the dword at `player_id`), param 1 whether `which` (a char) is 1 (first player).
/// Original: `transfer_screens_open_comparison_prompt` (0x8f41b0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_comparison_prompt(player_id: usize, unused: u32, which: u32) {
    covered!("tel18_1 open comparison prompt");
    let _ = unused;
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8f4210usize, 0x8f44d0usize, 1, 0usize, 0) != 0 {
        let first = (which as u8 == 1) as usize;
        set_screen_param(0, read_u32(player_id) as usize);
        set_screen_param(1, first);
    }
}

/// Comparer of `transfer_screens_open_comparison_prompt`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_comparison_prompt(player_id: usize, unused: u32, which: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32, u32)>(%%I4%%)(player_id, unused, which)
}

/// Builds the comparison prompt: "Set as 1st/2nd player in comparison?" and "Set <player> as 1st/2nd player in
/// comparison?" (param 1), the bar No / Yes with ids 2 / 1 for the first player and 0xb / 0xa for the second
/// (`ts_player_yes_no_dialog`).
/// Original: `transfer_screens_build_comparison_prompt` (0x8f4210, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_build_comparison_prompt() {
    covered!("tel18_1 comparison prompt");
    if get_screen_param(1) != 0 {
        ts_player_yes_no_dialog(0xaa0a48, 0xaa09dc, 2, 1);
    } else {
        ts_player_yes_no_dialog(0xaa0a24, 0xaa0994, 0xb, 0xa);
    }
}

/// Comparer of `transfer_screens_build_comparison_prompt`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_comparison_prompt() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I5%%);
    compare_by_display_list_with("Comparison prompt", &[], || o(), || transfer_screens_build_comparison_prompt());
}

/// Answers the comparison prompt: No (id 2 or 0xb) -9; Yes sets the player of param 0 as the first (id 1) or second
/// (id 0xa) player of the comparison (`SCOUT_MANAGER::set_compare_player(staff, 0, 1 or 2)`), -9; the rest goes to the
/// menu bar.
/// Original: `transfer_screens_handle_comparison_prompt_action` (0x8f44d0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_handle_comparison_prompt_action(item: u32) -> i32 {
    let set_compare: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(TS_FN_SET_COMPARE_PLAYER);
    let id = tel26_item_id(item as i16);
    if id == 2 || id == 0xb {
        return -9;
    }
    if id == 1 || id == 0xa {
        covered!("tel18_1 comparison prompt Yes");
        let player = staff_at(get_screen_param(0) as i32 as isize).addr();
        set_compare(read_usize(G_SCOUT_MANAGER), player, 0, if id == 1 { 1 } else { 2 });
        return -9;
    }
    ts_menubar_answer(item)
}

/// Comparer of `transfer_screens_handle_comparison_prompt_action`: No side by side; Yes (the scout manager's compare
/// player) and the menu bar only in the original.
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_comparison_prompt_action(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I6%%);
    ts_compare_answer("Comparison prompt", item, &[2, 0xb], o, transfer_screens_handle_comparison_prompt_action)
}

/// Builds the shortlist dialog: "Add Player To Shortlist" / "Add <player> to shortlist?" when param 1 is set, else
/// "Remove Player From Shortlist" / "Remove <player> from shortlist?"; the bar No (2) / Yes (1)
/// (`ts_player_yes_no_dialog`).
/// Original: `transfer_screens_build_shortlist_dialog` (0x8f4650, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_build_shortlist_dialog() {
    covered!("tel18_1 shortlist dialog");
    if get_screen_param(1) != 0 {
        ts_player_yes_no_dialog(0xaa0b00, 0xaa0aa8, 2, 1);
    } else {
        ts_player_yes_no_dialog(0xaa0ae0, 0xaa0a6c, 2, 1);
    }
}

/// Comparer of `transfer_screens_build_shortlist_dialog`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_shortlist_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I7%%);
    compare_by_display_list_with("Shortlist dialog", &[], || o(), || transfer_screens_build_shortlist_dialog());
}

/// Answers the shortlist dialog.
///
/// No (id 2) -9. Yes (id 1) with param 1 clear removes the player of param 0 from the shortlist
/// (`SCOUT_MANAGER::remove_from_shortlist`), -9. With param 1 set the player is added (`add_to_shortlist`) and the press
/// may hear of it: the slot manager's club (club job, else national job; none: done), its finances (none, or a
/// national team: done); 1 in (37 - the player's current reputation / 300) (`utils_random`); unless his reputation is
/// above 5000 + random(2000), only a player of a human club; his interest in joining (`staff_interest_in_joining_club`)
/// at least 2.75. Then two draws (random(20000), random(40000)) fill the deal record (the first is -10000 when the
/// club's transfer budget or its balance is not positive), `is_compensation_deal`, and the asking price: with a seller
/// (his contract club, not of type 5) and no compensation deal, the human asking price for a human seller or the
/// computed one; else 0. When the transfer budget, or else the balance, reaches 0.75 of the price,
/// `Media::add_transfer_interest(player, club, price, 0.0)`. -9 in every case. The rest goes to the menu bar.
/// Original: `transfer_screens_handle_shortlist_dialog` (0x8f48b0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_handle_shortlist_dialog(item: u32) -> i32 {
    let id = tel26_item_id(item as i16);
    if id == 2 {
        return -9;
    }
    if id != 1 {
        return ts_menubar_answer(item);
    }
    let player = staff_at(get_screen_param(0) as i32 as isize).addr();
    let scout = read_usize(G_SCOUT_MANAGER);
    if get_screen_param(1) == 0 {
        covered!("tel18_1 shortlist remove");
        let remove: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(TS_FN_REMOVE_FROM_SHORTLIST);
        remove(scout, player, 0);
        return -9;
    }
    covered!("tel18_1 shortlist add");
    exe::scout_manager_add_to_shortlist(scout, player, 0);
    let manager = tel26_slot_manager().addr();
    let mut club = read_usize(manager + staff::CLUB_JOB);
    let mut today = [0u32; 2];
    exe::date_date_54a110(today.as_mut_ptr() as usize);
    if club == 0 {
        club = read_usize(manager + staff::NATIONAL_JOB);
        if club == 0 {
            return -9;
        }
    }
    let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    if fin == 0 || exe::database_is_national_team(club) != 0 {
        return -9;
    }
    let reputation = || read_i16(read_usize(player + staff::PLAYER) + player_sheet::CURRENT_REPUTATION) as i32;
    if exe::utils_random(0x25 - reputation() / 300) != 0 {
        return -9;
    }
    let r = exe::utils_random(0x7d0) + 0x1388;
    if reputation() <= r {
        let own = read_usize(player + staff::CLUB_JOB);
        if own == 0 || exe::human_manager_is_human_club(own, 0, 1, 0, 0) == 0 {
            return -9;
        }
    }
    let sm = read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856);
    if !(exe::shortlist_manager_staff_interest_in_joining_club(sm, player, club, 0, 0, 1, 0) >= read_f64(rdata::F64_2_75)) {
        return -9;
    }
    covered!("tel18_1 shortlist add: press interest");
    let seller = ts_contract_club_not_type_5(player);
    let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
    // the deal record of the frame: the second draw at +3, the first at +0xb (the other bytes are left as they come)
    let mut deal = [0u8; 0x10];
    let d = deal.as_mut_ptr() as usize;
    let first = exe::utils_random(0x4e20) as i16 as i32;
    write_i32(d + 0xb, first);
    let second = exe::utils_random(0x9c40) as i16 as i32;
    write_i32(d + 3, second);
    let budget = exe::finance_get_transfer_budget(fin, player as u32, 1, 0usize, 1, 0);
    if budget <= 0 || read_i64(fin) <= 0 {
        write_i32(d + 0xb, -10000);
    }
    let mut out: i32 = 0;
    let compensation = exe::shortlist_manager_is_compensation_deal(sm, d, contract, club, &mut out);
    let price = if seller != 0 && compensation == 0 {
        if exe::human_manager_is_human_club(seller, 0, 1, 0, 0) != 0 {
            exe::contract_manager_get_human_asking_price(read_usize(G_CONTRACT_MANAGER), seller, club, player)
        } else {
            exe::contract_manager_calc_asking_price(read_usize(G_CONTRACT_MANAGER), seller, club, player, 0, 1, 1)
        }
    } else {
        0
    };
    let needed = price as f64 * read_f64(rdata::F64_0_75);
    let budget = exe::finance_get_transfer_budget(fin, player as u32, 1, 0usize, 1, 0);
    if (budget as f64) < needed && (read_i64(fin) as f64) < needed {
        return -9;
    }
    exe::media_add_transfer_interest(read_usize(G_MEDIA), player, club, price, 0.0);
    -9
}

/// Comparer of `transfer_screens_handle_shortlist_dialog`: No side by side; Yes (the shortlist, the draws and the press)
/// and the menu bar only in the original.
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_shortlist_dialog(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I8%%);
    ts_compare_answer("Shortlist dialog", item, &[2], o, transfer_screens_handle_shortlist_dialog)
}

/// Opens the dialog that withdraws the human's transfer bid (build `transfer_screens_build_withdraw_bid_dialog`, input
/// `transfer_screens_handle_withdraw_bid_dialog`, flag 1): param 0 the offer slot.
/// Original: `transfer_screens_open_withdraw_bid_dialog` (0x8f4c70, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_withdraw_bid_dialog(slot: usize) {
    covered!("tel18_1 open withdraw bid dialog");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8f4cb0usize, 0x8f4ed0usize, 1, 0usize, 0) != 0 {
        set_screen_param(0, slot);
    }
}

/// Comparer of `transfer_screens_open_withdraw_bid_dialog`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_withdraw_bid_dialog(slot: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I9%%)(slot)
}

/// Builds the withdraw bid dialog: the player's picture (the offer of param 0), the title in the buyer's colours, the
/// subtitle "Withdraw <player> transfer bid?" (name style 5), the text "Are you sure you want to withdraw your transfer
/// bid for <player>?" with his name as a link (0x7d7), the menu bar and the bar No (1) / Yes (2).
/// Original: `transfer_screens_build_withdraw_bid_dialog` (0x8f4cb0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_build_withdraw_bid_dialog() {
    covered!("tel18_1 withdraw bid dialog");
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let text_area: TsTextAreaLinks = core::mem::transmute(0x5ed070usize);
    let o = read_usize(get_screen_param(0));
    let player = staff_at(read_i32(o + offer::TARGET_STAFF_ID) as isize).addr();
    let buyer = club_at(read_i32(o + offer::BUYER_CLUB_ID) as isize).addr();
    picture(player, 0, 0);
    ts_title_in_club_colours(buyer);
    let mut name = [0u8; 0x190];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(player, n, 0x190, 5);
    lang(TS_TEXT, 0xaa0b78, n);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    exe::database_format_staff_name(player, n, 0x190, 1);
    lang(TS_TEXT, 0xaa0b18, n);
    let lines = ts_dialog_lines();
    text_area(0x6e, 0x8c, 0x30c, 0x221, 2, 0, lines as i8 as i32 as u32, 3, ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT, n, 0x7d7u32, player, 0u32);
    exe::menubar_build(4, 0);
    let weights: [u8; 2] = [3, 1];
    let bar = ts_button_bar(2, weights.as_ptr());
    let (no, yes) = (1, 2);
    ts_bar_button(bar, 0, TS_NO, no);
    ts_bar_button(bar, 1, TS_YES, yes);
}

/// Comparer of `transfer_screens_build_withdraw_bid_dialog`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_withdraw_bid_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I10%%);
    compare_by_display_list_with("Withdraw bid dialog", &[], || o(), || transfer_screens_build_withdraw_bid_dialog());
}

/// Answers the withdraw bid dialog: No (id 1) -10; Yes (id 2) withdraws the offer of param 0
/// (`withdraw_transfer_offer(slot, 0, 0)`), -10; the rest goes to the menu bar.
/// Original: `transfer_screens_handle_withdraw_bid_dialog` (0x8f4ed0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_handle_withdraw_bid_dialog(item: u32) -> i32 {
    let id = tel26_item_id(item as i16);
    if id == 1 {
        return -10;
    }
    if id == 2 {
        covered!("tel18_1 withdraw bid Yes");
        exe::transfer_manager_withdraw_transfer_offer(read_usize(G_TRANSFER_MANAGER), get_screen_param(0) as *mut usize, 0, 0);
        return -10;
    }
    ts_menubar_answer(item)
}

/// Comparer of `transfer_screens_handle_withdraw_bid_dialog`: No side by side; Yes (withdraws the bid) and the menu bar
/// only in the original.
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_withdraw_bid_dialog(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I11%%);
    ts_compare_answer("Withdraw bid dialog", item, &[1], o, transfer_screens_handle_withdraw_bid_dialog)
}
