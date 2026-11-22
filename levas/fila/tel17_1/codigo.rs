// ---------- batch %%LEVA%%: the transfer bid screen (transfer_screens.cpp, part 1: tel17_1) ----------

/// `TRANSFER_OFFER::TRANSFER_OFFER(player, club, 0, -1, 0)` (0x8e56a0, thiscall ret 0x14).
const TS_FN_NEW_TRANSFER_OFFER: usize = 0x8e56a0;
/// `TRANSFER_OFFER::has_same_terms(other)` (0x8e7f70, thiscall ret 4): nonzero when both offers have the same terms.
const TS_FN_HAS_SAME_TERMS: usize = 0x8e7f70;
/// `TRANSFER_MANAGER::get_negotiation_slot(index)` (0x8c6200, thiscall ret 4).
const TS_FN_GET_NEGOTIATION_SLOT: usize = 0x8c6200;
/// `HUMAN_MANAGER::get_bid_offer_slot(0)` (0x5fda30, thiscall on [G_HUMAN_MANAGER] ret 4): the human's bid being edited.
const TS_FN_BID_OFFER_SLOT: usize = 0x5fda30;
/// `Cash::set_from_amount_table(index, table, count)` (0x43f720, thiscall ret 0xc).
const TS_FN_CASH_FROM_TABLE: usize = 0x43f720;
/// `FOG_OF_WAR_MAP::get_knowledge_level(staff)` (0x5b02f0, thiscall ret 4).
const TS_FN_KNOWLEDGE_LEVEL: usize = 0x5b02f0;
/// The colour of the popup lists.
const TS_COLOUR_POPUP: usize = 0xae31aa;
/// "-", "+", "=".
const TS_MINUS: usize = 0x98ba0c;
const TS_PLUS: usize = 0x9afe7c;
const TS_EQUALS: usize = 0x9afe78;
/// A club's manager pointer (the fog of war map is per manager) [inference].
const TS_CLUB_MANAGER: usize = 0xcf;
/// The fee steps of the fee and clause popups.
const TS_FEE_STEPS: [u32; 10] = [0, 100000, 250000, 500000, 1000000, 2500000, 5000000, 10000000, 15000000, 20000000];

/// The display amount of a cash value (`Cash::get_display_amount(mode)`).
pub(crate) unsafe fn ts_display(value: u32, mode: u32) -> i64 {
    let v = value;
    exe::cash_get_display_amount(&v, mode)
}

/// The cash value formatted into `TS_TEXT` (2000 bytes) in `mode`.
pub(crate) unsafe fn ts_money_text(value: u32, mode: u32) {
    exe::cash_format_money(TS_TEXT, 0x7d0, ts_display(value, mode), mode);
}

/// `add_area_text(area, col, row, a, b, c, 3, colour, text)`.
unsafe fn ts_at(area: u32, col: u32, row: i32, a: u32, b: u32, c: u32, colour: usize, text: usize) -> u32 {
    tel26_area_text([area, col, row as u32, a, b, c, 3, ts_colour(colour), text as u32])
}

/// `add_area_button(area, col, row, a, b, c, 3, colour, text, id, value)`.
unsafe fn ts_ab(area: u32, col: u32, row: i32, a: u32, b: u32, c: u32, colour: usize, text: usize, id: u32, value: u32) -> u32 {
    tel26_area_button([area, col, row as u32, a, b, c, 3, ts_colour(colour), text as u32, id, value])
}

/// A popup area attached to the item `item` (an area of 0x130 style, the popup colour).
unsafe fn ts_popup_for(item: u32) -> u32 {
    tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, ts_colour(TS_COLOUR_POPUP), item as i16 as i32 as u32])
}

/// Whether `amount` is over the transfer budget `budget`, as display amounts (mode 4).
unsafe fn ts_over_budget(amount: u32, budget: u32) -> bool {
    ts_display(amount, 4) > ts_display(budget, 4)
}

/// The inline `TRANSFER_OFFER` copy of the bid screen: every field of `src` into `dst` (the completion timing and the
/// offer kind only by their low nibble), then `dst`'s first dword cleared and, when it has an exchange list, a new list
/// filled from `src`'s.
unsafe fn ts_copy_offer(dst: usize, src: usize) {
    for k in [0usize, 4, 8, 0xc] {
        write_u32(dst + k, read_u32(src + k));
    }
    write_u8(dst + 0x10, read_u8(src + 0x10));
    write_u8(dst + 0x11, read_u8(src + 0x11));
    let mut tmp = [0u32; 2];
    exe::date_operator_3d(dst + offer::DUE_DATE, tmp.as_mut_ptr() as usize, src + offer::DUE_DATE);
    write_u8(dst + 0x1a, read_u8(src + 0x1a));
    write_u32(dst + 0x1b, read_u32(src + 0x1b));
    write_u8(dst + 0x1f, read_u8(src + 0x1f));
    for k in [0x20usize, 0x24, 0x28] {
        write_u32(dst + k, read_u32(src + k));
    }
    for k in [0x2cusize, 0x2d, 0x2e, 0x2f] {
        write_u8(dst + k, read_u8(src + k));
    }
    let b = read_u8(dst + 0x30);
    write_u8(dst + 0x30, ((((read_u8(src + 0x30) << 4) as i8) >> 4) as u8 ^ b) & 0xf ^ b);
    let d = read_u32(dst + 0x31);
    write_u32(dst + 0x31, (read_u32(src + 0x31) ^ d) & 0xf ^ d);
    write_u32(dst + 0x35, read_u32(src + 0x35));
    write_u8(dst + 0x39, read_u8(src + 0x39));
    write_u32(dst + 0x3a, read_u32(src + 0x3a));
    write_u8(dst + 0x3e, read_u8(src + 0x3e));
    write_u8(dst + 0x3f, read_u8(src + 0x3f));
    write_u8(dst + 0x40, read_u8(src + 0x40));
}

/// After `ts_copy_offer`: the first dword cleared and, with an exchange list, a new LLIST copied from `src_list`.
unsafe fn ts_copy_offer_list(dst: usize, src_list: usize) {
    write_u32(dst, 0);
    if read_usize(dst + offer::EXCHANGE_PLAYERS) != 0 {
        let l = exe::crt_operator_new(0xe);
        let l = if l != 0 { exe::new_llist(l) } else { 0 };
        write_usize(dst + offer::EXCHANGE_PLAYERS, l);
        exe::llist_copy_from(l, src_list);
    }
}

/// Builds the transfer bid screen.
///
/// Params: 0 the player's id, 1 the slot of the offer being answered (or 0), 2 set to 1 here when the bid starts from a
/// live negotiation, 4 the exchange players are buttons (id 0x1f) and Remove is highlighted, 6 the human is bidding
/// (the budget lines and the Back/Cancel/Offer bar), 7 read only; the item params 0xf..0x16 are written here.
///
/// Without the player: the assert box (line 0x208). Without his contract, a contract of type 5, or its club: nothing.
/// Without the slot's manager: the assert box (0x21b); without his club, the assert box (0x222). The human's bid
/// (`HUMAN_MANAGER::get_bid_offer_slot(0)`) is created when empty (`operator new` 0x41, `TRANSFER_OFFER(player, club, 0,
/// -1, 0)`) and, with an offer answered, filled from it: from its negotiation (param 2 = 1) when it has one, else from
/// the offer itself (`ts_copy_offer`).
///
/// The header (an area 100,10..0x316,0x46 in the human club's kit colours) is the club's name with a link (0x7d5). When
/// the bid is still open (status 0), an offer is answered and the player plays for the human's club, the other open
/// bids for him (other buyers) go in a popup under a button (0x69,0xf..0x78,0x23): their buyer's name, id 0x27, the
/// offer's slot index. The title (font 6, yellow; 100,0x50..0x316,0x7d) is "Negotiate <buyer> bid for <player>" or
/// "<buyer> transfer bid for <player>" (bidding or read only). The table (0x6e,0x8c..0x30c,0x17c, 7 columns 2:0x14:2:
/// 0xa:2:2:2, as many rows as 0xf0 / font 3 height): the player's value line (0xa9e854, his contract club, links), when
/// bidding the transfer budget line (by the fog of war knowledge of the player when the masking is on), then the rows
/// Transfer Fee (item param 0xf; "-" 4, "+" 3, "=" 5, the value in red over budget; popup 6), Monthly Installments
/// (param 0x10; ids 7, 9, 8; at most 24), Percentage of Next Sale (0x11; 0xa, 0xc, 0xb; at most 25%), the league and
/// international appearance fees (0x12 and 0x13; the games a link 0xe / 0x13; "-" over 1000, "+" under 100000000;
/// popups 0x11 and 0x16) and, outside the read only view, "Make transfer bid public" (0x22; No / Yes, greyed once
/// public). Then the Exchange button (0x19, up to 5 players), Remove (0x1e), Reset (0x1a, item param 0x16; greyed when
/// the bid equals the offer answered) and Negotiated Bid (0x20, when the offer has a negotiation). The lower table
/// (0x6e,0x19a..0x30c,0x221, 3 columns 2:0xa:0x13): each exchange player and his value, and the Total Value (yellow,
/// item param 0x14). The menu bar, and the bar: bidding: Back 0x21, [Cancel 2,] Offer 1; read only: Back, Withdraw 2 /
/// Cancel 2; answering: Back, Reject 0x1b, [Delay 0x23,] Negotiate 0x1c (greyed Accept for an offer that can only be
/// accepted, Accept 0x1d when unchanged; item param 0x15). Last, the three popups get the ten fee steps
/// (`Cash::set_from_amount_table`), ids 6, 0x11 and 0x16.
///
/// Defect of the original (harmless): the row counts of the two tables are pushed as the low byte of a dword whose
/// high bytes are those of the bids popup's area variable; the copy pushes the same dword.
/// Original: `transfer_screens_build_bid_screen` (0x8e9760, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_build_bid_screen() {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: TsLang = core::mem::transmute(FN_CRT_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let text_new: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_TEXT_TEXT);
    let text_set: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(FN_TEXT_SET_STRING);
    let text_link: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> i32 = core::mem::transmute(FN_TEXT_ADD_LINK);
    let text_clear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(TS_FN_TEXT_CLEAR);
    let text_drop: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TEXT_DESTRUCTOR_TEXT);
    let draw: unsafe extern "thiscall" fn(usize, u32, i32, i32, *mut i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(TS_FN_TEXT_DRAW_WRAPPED);
    let new_offer: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> usize = core::mem::transmute(TS_FN_NEW_TRANSFER_OFFER);
    let same_terms: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(TS_FN_HAS_SAME_TERMS);
    let negotiation_slot: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(TS_FN_GET_NEGOTIATION_SLOT);
    let bid_slot: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(TS_FN_BID_OFFER_SLOT);
    let from_table: unsafe extern "thiscall" fn(*mut u32, u32, *const u32, u32) = core::mem::transmute(TS_FN_CASH_FROM_TABLE);
    let knowledge: unsafe extern "thiscall" fn(usize, usize) -> i8 = core::mem::transmute(TS_FN_KNOWLEDGE_LEVEL);
    let add_text_item: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize, u32) -> u32 =
        core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);

    let w7: [u8; 7] = [2, 0x14, 2, 0xa, 2, 2, 2];
    let w3: [u8; 3] = [2, 0xa, 0x13];
    let w2: [u8; 2] = [3, 1];
    let w4: [u8; 4] = [3, 1, 1, 1];
    let wb3: [u8; 3] = [2, 1, 1];
    let mut text = [0u32; 8];
    let t = text.as_mut_ptr() as usize;
    text_new(t, 0);
    // the bids popup's area; its dword also carries the row counts of the two tables (see the header)
    let mut bids_popup: u32 = u32::MAX;
    let mut budget: u32 = 0;
    let mut answered: usize = 0;
    let mut list = [0u8; 0x10];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    let mut known: u32 = 0;
    let player = staff_at(get_screen_param(0) as i32 as isize).addr();
    if get_screen_param(1) != 0 {
        answered = read_usize(get_screen_param(1));
    }
    let exchange_buttons = get_screen_param(4);
    let bidding = get_screen_param(6);
    let read_only = get_screen_param(7);
    let finish = |t: usize, l: usize| {
        llist_drop(l);
        text_drop(t);
    };
    if player == 0 {
        tel26_error_box(TS_CPP, 0x208);
        return finish(t, l);
    }
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    if c == 0 || read_u8(c + 0x35) & 0x3f == 5 || club_at(read_i32(c + 4) as isize).addr() == 0 {
        return finish(t, l);
    }
    if tel26_slot_manager().addr() == 0 {
        tel26_error_box(TS_CPP, 0x21b);
        return finish(t, l);
    }
    let human = ts_human_club();
    if human == 0 {
        tel26_error_box(TS_CPP, 0x222);
        return finish(t, l);
    }
    covered!("tel17_1 bid screen");
    let slot = bid_slot(read_usize(G_HUMAN_MANAGER), 0);
    if read_usize(slot) == 0 {
        covered!("tel17_1 bid screen creates the bid");
        let mem = exe::crt_operator_new(0x41);
        let o = if mem != 0 { new_offer(mem, player, human, 0, -1, 0) } else { 0 };
        write_usize(slot, o);
        if answered != 0 {
            if read_i32(answered + offer::NEGOTIATION_INDEX) != -1 {
                let ns = negotiation_slot(read_usize(G_TRANSFER_MANAGER), read_i32(answered + offer::NEGOTIATION_INDEX));
                ts_copy_offer(read_usize(slot), read_usize(ns));
                set_screen_param(2, 1);
                ts_copy_offer_list(read_usize(slot), read_usize(read_usize(ns) + offer::EXCHANGE_PLAYERS));
            } else {
                ts_copy_offer(o, answered);
                ts_copy_offer_list(read_usize(slot), read_usize(answered + offer::EXCHANGE_PLAYERS));
            }
        }
    }
    let bid = read_usize(slot);
    picture(player, 0, 0);
    let (mut kit_a, mut kit_b) = (0u32, 0u32);
    exe::database_get_club_kit_colours(human, 1, &mut kit_a, &mut kit_b);
    format(TS_TEXT, 0x991944, human + club::SHORT_NAME); // "%s"
    text_set(t, TS_TEXT);
    text_link(t, human + club::SHORT_NAME, 0x7d5, human);
    let header = tel26_add_area(current_human_display(), [100, 0xa, 0x316, 0x46, 1, 0, 1, 0, 0x30, kit_b, u32::MAX]);
    let mut row: i32 = 0;
    draw(t, header, 100, 0x316, &mut row, 7, kit_a, 1, 0);

    // the other open bids for a player of the human's club, in a popup
    if read_u8(bid + offer::STATUS) == 0 && answered != 0 {
        let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
        write_usize(G_CONTRACT_LOOKUP_RESULT, c);
        let own = if c == 0 || read_u8(c + 0x35) & 0x3f == 5 { 0 } else { club_at(read_i32(c + 4) as isize).addr() };
        if human == own {
            covered!("tel17_1 bid screen: the other bids");
            exe::transfer_manager_get_offers_for_player(read_usize(G_TRANSFER_MANAGER), player, l, 0);
            write_u32(l + llist::CURSOR, read_u32(l + llist::HEAD));
            loop {
                let node = exe::llist_get_next(l);
                if node == 0 {
                    break;
                }
                let other = read_usize(exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), read_i32(node)));
                if read_u8(other + offer::STATUS) != 0 || read_u32(other + offer::BUYER_CLUB_ID) == read_u32(bid + offer::BUYER_CLUB_ID) {
                    continue;
                }
                let buyer = club_at(read_i32(other + offer::BUYER_CLUB_ID) as isize).addr();
                if bids_popup as i16 == -1 {
                    let button = tel26_add_button([0x69, 0xf, 0x78, 0x23, 0x30, kit_b, 0xc, 1, kit_a, 0, 0, 0]);
                    bids_popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x30, ts_colour(TS_COLOUR_POPUP), button]);
                }
                tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, ts_colour(TS_COLOUR_POPUP), ts_colour(G_GUI_COLOUR_YELLOW), 1, 1,
                    ts_colour(G_GUI_COLOUR_BLACK), (buyer + club::NAME) as u32, 0, 0x27, read_u32(other + offer::SLOT_INDEX), bids_popup]);
            }
        }
    }

    // the title
    let mut name = [0u8; 400];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(player, n, 400, 5);
    let buyer = club_at(read_i32(bid + offer::BUYER_CLUB_ID) as isize).addr();
    let g = |c: usize| read_i8(c + club::SHORT_GENDER_NAME) as i32;
    if bidding == 0 && read_only == 0 {
        lang(TS_TEXT, 0xa9e930, g(buyer), g(buyer), 100, buyer + club::SHORT_NAME, n);
    } else {
        lang(TS_TEXT, 0xa9e8dc, g(buyer), g(buyer), buyer + club::SHORT_NAME, n);
    }
    text_clear(t);
    text_set(t, TS_TEXT);
    text_link(t, n, 0x7d7, player);
    text_link(t, buyer + club::SHORT_NAME, 0x7d5, buyer);
    let title = tel26_add_area(current_human_display(), [100, 0x50, 0x316, 0x7d, 1, 0, 1, 0, 2, 0, u32::MAX]);
    row = 0;
    draw(t, title, 100, 0x316, &mut row, 6, ts_colour(G_GUI_COLOUR_YELLOW), 1, 0);

    // the table
    let lines = (0xf0 / exe::font_get_height(3)) as u8;
    bids_popup = (bids_popup & !0xff) | lines as u32;
    let area = tel26_add_area(current_human_display(), [0x6e, 0x8c, 0x30c, 0x17c, 7, w7.as_ptr() as u32, bids_popup, 0, 2, 0, u32::MAX]);
    let mut value_text = [0u8; 0x20];
    let vt = value_text.as_mut_ptr() as usize;
    exe::cash_format_money(vt, 0x1e, ts_display(read_u32(player + 0x52), 6), 6);
    exe::database_format_staff_name(player, n, 400, 1);
    let cc = || contract_club_cached(Staff(player));
    let (c1, c2, c3) = (cc(), cc(), cc());
    lang(TS_TEXT, 0xa9e854, g(c3), g(c2), c1 + club::SHORT_NAME, n, vt);
    text_set(t, TS_TEXT);
    text_link(t, n, 0x7d7, player);
    let (c4, c5) = (cc(), cc());
    text_link(t, c5 + club::SHORT_NAME, 0x7d5, c4);
    if read_only == 0 {
        text_link(t, vt, 0x26, 0);
    }
    text_set(t, TS_TEXT);
    row = 0;
    draw(t, area, 0x76, 0x30c, &mut row, 3, ts_colour(G_GUI_COLOUR_PALE_GREY), 0, 0);
    if bidding != 0 {
        covered!("tel17_1 bid screen: the transfer budget");
        let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), human);
        let b = exe::finance_get_transfer_budget(fin, player as u32, 0, &mut known as *mut u32 as usize, 1, 0);
        budget = b as u32;
        exe::cash_format_money(vt, 0x1e, ts_display(budget, 6), 6);
        let mut with_value = 0xa9e510usize;
        let mut plain = 0usize;
        if read_u8(G_ATTRIBUTE_MASKING_ENABLED) != 0 && read_usize(G_FOG_OF_WAR) != 0 && known != 0 {
            let map = exe::fog_of_war_get_manager_map(read_usize(G_FOG_OF_WAR), read_usize(human + TS_CLUB_MANAGER));
            if map != 0 {
                match knowledge(map, player) {
                    1 => if b == 0 { plain = 0xa9e5e4 } else { with_value = 0xa9e560 },
                    2 => if b == 0 { plain = 0xa9e6d0 } else { with_value = 0xa9e67c },
                    3 => if b == 0 { plain = 0xa9e6d0 },
                    _ => if b == 0 { plain = 0xa9e7b8 } else { with_value = 0xa9e730 },
                }
            }
        }
        if plain != 0 {
            lang(TS_TEXT, plain);
        } else {
            lang(TS_TEXT, with_value, vt);
        }
        text_clear(t);
        text_set(t, TS_TEXT);
        text_link(t, vt, 0x25, 0);
        row += 1;
        draw(t, area, 0x76, 0x30c, &mut row, 3, ts_colour(G_GUI_COLOUR_PALE_GREY), 0, 0);
    }

    let fee = |o: usize| read_i32(o + offer::FEE);
    let value_colour = |amount: u32| if bidding != 0 && ts_over_budget(amount, budget) { G_GUI_COLOUR_RED } else { G_GUI_COLOUR_PALE_GREY };
    const PALE: usize = G_GUI_COLOUR_PALE_GREY;
    const GREY: usize = G_GUI_COLOUR_GREY;
    const DARK: usize = G_GUI_COLOUR_DARK_BLUE;
    const BLACK: usize = G_GUI_COLOUR_BLACK;

    // Transfer Fee
    row += 2;
    lang(TS_TEXT, 0xa9e500);
    let item = ts_ab(area, 1, row, 1, 0, 1, PALE, TS_TEXT, 0, 0);
    set_screen_param(0xf, item as i16 as i32 as usize);
    if fee(bid) > 0 && read_only == 0 {
        ts_ab(area, 2, row, 0x30, ts_colour_addr(DARK), 0xc, PALE, TS_MINUS, 4, 0);
    } else {
        ts_at(area, 2, row, 0x30, ts_colour_addr(DARK), 0xc, GREY, TS_MINUS);
    }
    ts_money_text(read_u32(bid + offer::FEE), 5);
    let colour = value_colour(read_u32(bid + offer::FEE));
    let item = ts_ab(area, 3, row, 1, ts_colour_addr(BLACK), 0xc, colour, TS_TEXT, 0, 0);
    let fee_popup = if read_only == 0 { ts_popup_for(item) } else { u32::MAX };
    if fee(bid) < 100000000 && read_only == 0 {
        ts_ab(area, 4, row, 0x30, ts_colour_addr(DARK), 0xc, PALE, TS_PLUS, 3, 0);
        ts_ab(area, 5, row, 0x30, ts_colour_addr(DARK), 0xc, PALE, TS_EQUALS, 5, 0);
    } else {
        ts_at(area, 4, row, 0x30, ts_colour_addr(DARK), 0xc, GREY, TS_PLUS);
        if read_only == 0 {
            ts_ab(area, 5, row, 0x30, ts_colour_addr(DARK), 0xc, PALE, TS_EQUALS, 5, 0);
        } else {
            ts_at(area, 5, row, 0x30, ts_colour_addr(DARK), 0xc, GREY, TS_EQUALS);
        }
    }

    // Monthly Installments and Percentage of Next Sale
    row += 1;
    lang(TS_TEXT, 0xa9e4e8);
    let inst = read_i8(bid + offer::INSTALMENTS);
    let item = ts_stepper_row(area, row, read_only != 0, inst, 6, 0x18, [7, 9, 8], ts_colour_addr(TS_COLOUR_POPUP), |v| {
        lang(TS_TEXT, 0xa9e4c8, v as i32);
    });
    set_screen_param(0x10, item as usize);
    row += 1;
    lang(TS_TEXT, 0xa9e4b0);
    let sell_on = read_i8(bid + offer::SELL_ON_PERCENT);
    let item = ts_stepper_row(area, row, read_only != 0, sell_on, 5, 0x19, [0xa, 0xc, 0xb], 0, |v| {
        format(TS_TEXT, 0x98e8d4, v as i32); // "%d%%"
    });
    set_screen_param(0x11, item as usize);

    // the appearance clauses
    row += 1;
    let games = read_i8(bid + offer::APPEARANCE_GAMES);
    if games == 1 {
        lang(TS_TEXT, 0xa9e498);
    } else {
        lang(TS_TEXT, 0xa9e468, games as i32);
    }
    let league_popup = ts_clause_row(t, area, &mut row, read_only != 0, bid + offer::APPEARANCE_FEE, games, [0xd, 0xe, 0x10, 0xf],
        ts_colour_addr(BLACK), 0x12, &value_colour);
    row += 1;
    let caps = read_i8(bid + offer::INTERNATIONAL_GAMES);
    if caps == 1 {
        lang(TS_TEXT, 0xa9e448);
    } else {
        lang(TS_TEXT, 0xa9e414, caps as i32);
    }
    let caps_popup = ts_clause_row(t, area, &mut row, read_only != 0, bid + offer::INTERNATIONAL_FEE, caps, [0x12, 0x13, 0x15, 0x14],
        0, 0x13, &value_colour);
    row += 1;

    // Make transfer bid public
    if read_only == 0 {
        lang(TS_TEXT, 0xa9e3f8);
        let aware = read_u8(bid + offer::PLAYER_AWARE);
        if aware != 1 {
            ts_ab(area, 1, row, 1, 0, 1, PALE, TS_TEXT, 0x22, 0);
        } else {
            ts_at(area, 1, row, 1, 0, 1, PALE, TS_TEXT);
        }
        match read_u8(bid + offer::PLAYER_AWARE) {
            0 => {
                lang(TS_TEXT, TS_NO);
                ts_ab(area, 3, row, 1, 0, 0xc, PALE, TS_TEXT, 0x22, 0);
            }
            2 => {
                lang(TS_TEXT, TS_YES);
                ts_ab(area, 3, row, 1, 0, 0xc, PALE, TS_TEXT, 0x22, 0);
            }
            1 => {
                lang(TS_TEXT, TS_YES);
                ts_at(area, 3, row, 1, 0, 0xc, GREY, TS_TEXT);
            }
            _ => {}
        }
    }

    // Exchange, Remove, Reset, Negotiated Bid
    lang(TS_TEXT, 0x991488); // "Exchange"
    let ex = read_usize(bid + offer::EXCHANGE_PLAYERS);
    if (ex == 0 || read_i16(ex) < 5) && read_only == 0 {
        tel26_add_button([0x6e, 0x181, 0xeb, 0x195, 0x30, ts_colour(GREY), 0xc, 1, ts_colour(PALE), TS_TEXT as u32, 0x19, 0]);
    } else {
        add_text_item(G_SCREEN_MANAGER, 0x6e, 0x181, 0xeb, 0x195, 0x30, ts_colour(GREY), 0x2c, 1, ts_colour(PALE), TS_TEXT, 1);
    }
    lang(TS_TEXT, 0xa8b354); // "Remove"
    if read_usize(bid + offer::EXCHANGE_PLAYERS) != 0 && read_only == 0 {
        if exchange_buttons != 0 {
            tel26_add_button([0xeb, 0x181, 0x168, 0x195, 0x70, ts_colour(GREY), 0xc, 1, ts_colour(PALE), TS_TEXT as u32, 0x1e, 0]);
        } else {
            tel26_add_button([0xeb, 0x181, 0x168, 0x195, 0x30, ts_colour(GREY), 0xc, 1, ts_colour(PALE), TS_TEXT as u32, 0x1e, 1]);
        }
    } else {
        add_text_item(G_SCREEN_MANAGER, 0xeb, 0x181, 0x168, 0x195, 0x30, ts_colour(GREY), 0x2c, 1, ts_colour(PALE), TS_TEXT, 1);
    }
    if answered != 0 && read_only == 0 {
        lang(TS_TEXT, 0xa7e768); // "Reset"
        let item = if same_terms(bid, answered) == 0 {
            tel26_add_button([0x28f, 0x181, 0x30c, 0x195, 0x30, ts_colour(GREY), 0xc, 1, ts_colour(PALE), TS_TEXT as u32, 0x1a, 0])
        } else {
            add_text_item(G_SCREEN_MANAGER, 0x28f, 0x181, 0x30c, 0x195, 0x30, ts_colour(GREY), 0x2c, 1, ts_colour(PALE), TS_TEXT, 1)
        };
        set_screen_param(0x16, item as i16 as i32 as usize);
        if read_i32(answered + offer::NEGOTIATION_INDEX) != -1 {
            let ns = negotiation_slot(read_usize(G_TRANSFER_MANAGER), read_i32(answered + offer::NEGOTIATION_INDEX));
            lang(TS_TEXT, 0xa9e3e8); // "Negotiated Bid"
            if same_terms(bid, read_usize(ns)) == 0 {
                tel26_add_button([0x212, 0x181, 0x28f, 0x195, 0x30, ts_colour(GREY), 0xc, 1, ts_colour(PALE), TS_TEXT as u32, 0x20, 0]);
            } else {
                add_text_item(G_SCREEN_MANAGER, 0x212, 0x181, 0x28f, 0x195, 0x30, ts_colour(GREY), 0x2c, 1, ts_colour(PALE), TS_TEXT, 1);
            }
        }
    }

    // the lower table: the exchange players and the total value
    let lines = (0x87 / exe::font_get_height(3)) as u8;
    bids_popup = (bids_popup & !0xff) | lines as u32;
    let lower = tel26_add_area(current_human_display(), [0x6e, 0x19a, 0x30c, 0x221, 3, w3.as_ptr() as u32, bids_popup, 0, 2, 0, u32::MAX]);
    row = 0;
    let mut total = ts_display(read_u32(bid + offer::FEE), 4);
    if read_i32(bid + offer::APPEARANCE_FEE) >= 0 {
        total = total.wrapping_add(ts_display(read_u32(bid + offer::APPEARANCE_FEE), 4));
    }
    if read_i32(bid + offer::INTERNATIONAL_FEE) >= 0 {
        total = total.wrapping_add(ts_display(read_u32(bid + offer::INTERNATIONAL_FEE), 4));
    }
    let ex = read_usize(bid + offer::EXCHANGE_PLAYERS);
    if ex != 0 {
        write_u32(ex + llist::CURSOR, read_u32(ex + llist::HEAD));
        loop {
            let node = exe::llist_get_next(read_usize(bid + offer::EXCHANGE_PLAYERS));
            if node == 0 {
                break;
            }
            let p = staff_at(read_i32(node) as isize).addr();
            format(TS_TEXT, 0xa9e3e4); // "+ "
            exe::database_format_staff_name(p, TS_TEXT + 2, 0x7d0, 2);
            if exchange_buttons != 0 {
                ts_ab(lower, 1, row, 1, 0, 1, PALE, TS_TEXT, 0x1f, node as u32);
            } else {
                ts_ab(lower, 1, row, 1, 0, 1, PALE, TS_TEXT, 0x7d7, p as u32);
            }
            let v = read_u32(p + 0x52);
            ts_money_text(v, 6);
            ts_at(lower, 2, row, 1, 0, 0xc, PALE, TS_TEXT);
            row += 1;
            total = total.wrapping_add(ts_display(v, 4));
        }
    }
    lang(TS_TEXT, 0xa9e3d8); // "Total Value"
    ts_at(lower, 1, row, 1, 0, 1, PALE, TS_TEXT);
    exe::cash_format_money(TS_TEXT, 0x7d0, total, 0x22);
    let item = ts_at(lower, 2, row, 1, 0, 0xc, G_GUI_COLOUR_YELLOW, TS_TEXT);
    set_screen_param(0x14, item as i16 as i32 as usize);
    exe::menubar_build(4, 0);

    // the button bar
    let bar_with = |columns: u32, weights: *const u8| {
        tel26_add_area(current_human_display(), [100, 0x22b, 0x316, 0x24e, columns, weights as u32, 1, 0, 1, kit_b, u32::MAX])
    };
    let button = |bar: u32, col: u32, label: usize, id: u32| -> u32 {
        lang(TS_TEXT, label);
        ts_ab(bar, col, 0, 0x30, ts_colour_addr(GREY), 0xc, PALE, TS_TEXT, id, 0)
    };
    if bidding != 0 {
        if read_only != 0 {
            let bar = bar_with(2, w2.as_ptr());
            button(bar, 0, TS_BACK, 0x21);
            button(bar, 1, 0xa9e3cc, 2); // "Withdraw"
        } else if answered != 0 {
            let bar = bar_with(3, w4.as_ptr());
            button(bar, 0, TS_BACK, 0x21);
            button(bar, 1, 0x98a1a0, 2); // "Cancel"
            button(bar, 2, 0x98e6e8, 1); // "Offer"
        } else {
            let bar = bar_with(2, w2.as_ptr());
            button(bar, 0, TS_BACK, 0x21);
            button(bar, 1, 0x98e6e8, 1); // "Offer"
        }
    } else if read_only != 0 {
        let bar = bar_with(2, w2.as_ptr());
        button(bar, 0, TS_BACK, 0x21);
        button(bar, 1, 0x98a1a0, 2); // "Cancel"
    } else {
        covered!("tel17_1 bid screen: answering a bid");
        let three = read_u8(bid + offer::RESPONSE_ROUND) >= 3;
        let bar = if three { bar_with(3, wb3.as_ptr()) } else { bar_with(4, w4.as_ptr()) };
        button(bar, 0, TS_BACK, 0x21);
        button(bar, 1, 0xa9e3c4, 0x1b); // "Reject"
        let last = if three { 2 } else {
            button(bar, 2, 0xa9e3b4, 0x23); // "Delay"
            3
        };
        let item = if same_terms(bid, answered) == 0 {
            if read_u8(answered + offer::OFFER_KIND) & 0xf == 0 {
                button(bar, last, 0xa6943c, 0x1c) // "Negotiate"
            } else {
                lang(TS_TEXT, 0xa9e3bc); // "Accept"
                ts_at(bar, last, 0, 0x30, ts_colour_addr(GREY), 0x2c, PALE, TS_TEXT)
            }
        } else {
            button(bar, last, 0xa9e3bc, 0x1d) // "Accept"
        };
        set_screen_param(0x15, item as i16 as i32 as usize);
    }

    // the fee steps of the three popups
    for (popup, id) in [(fee_popup, 6u32), (league_popup, 0x11), (caps_popup, 0x16)] {
        if popup as i16 == -1 {
            continue;
        }
        for k in 0..10u32 {
            let mut cash: u32 = 0;
            from_table(&mut cash, k, TS_FEE_STEPS.as_ptr(), 10);
            ts_money_text(cash, 5);
            tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, ts_colour(TS_COLOUR_POPUP), ts_colour(G_GUI_COLOUR_YELLOW), 1, 1,
                ts_colour(G_GUI_COLOUR_BLACK), TS_TEXT as u32, 0, id, cash, popup]);
        }
    }
    finish(t, l);
}

/// The colour word at `a` as the plain field value (the cell fields that are colours, not the text colour).
unsafe fn ts_colour_addr(a: usize) -> u32 {
    if a == 0 { 0 } else { ts_colour(a) }
}

/// One of the Monthly Installments / Percentage of Next Sale rows (the label already in `TS_TEXT`): the label (a
/// button `ids[0]` when editable; pale with a value, grey without), "-" (grey text at `floor` or read only; else a
/// button `ids[1]` with a value, `ids[0]` without), the value (`fill(value)` or "-") and "+" (grey text at `cap` or read
/// only; else `ids[2]` / `ids[0]`). Returns the label's item.
unsafe fn ts_stepper_row(area: u32, row: i32, read_only: bool, value: i8, floor: i8, cap: i8, ids: [u32; 3], value_field: u32, fill: impl Fn(i8)) -> i32 {
    let format: TsLang = core::mem::transmute(FN_CRT_SPRINTF);
    let set = value > 0;
    let label_colour = if set { G_GUI_COLOUR_PALE_GREY } else { G_GUI_COLOUR_GREY };
    let dark = ts_colour(G_GUI_COLOUR_DARK_BLUE);
    let item;
    if read_only {
        item = ts_at(area, 1, row, 1, 0, 1, label_colour, TS_TEXT) as i16 as i32;
        ts_at(area, 2, row, 0x30, dark, 0xc, G_GUI_COLOUR_GREY, TS_MINUS);
    } else {
        item = ts_ab(area, 1, row, 1, 0, 1, label_colour, TS_TEXT, ids[0], 0) as i16 as i32;
        if value == floor {
            ts_at(area, 2, row, 0x30, dark, 0xc, G_GUI_COLOUR_GREY, TS_MINUS);
        } else if set {
            ts_ab(area, 2, row, 0x30, dark, 0xc, G_GUI_COLOUR_PALE_GREY, TS_MINUS, ids[1], 0);
        } else {
            ts_ab(area, 2, row, 0x30, dark, 0xc, G_GUI_COLOUR_GREY, TS_MINUS, ids[0], 0);
        }
    }
    if set {
        fill(value);
    } else {
        format(TS_TEXT, TS_MINUS);
    }
    ts_at(area, 3, row, 1, value_field, 0xc, G_GUI_COLOUR_PALE_GREY, TS_TEXT);
    if read_only || value >= cap {
        ts_at(area, 4, row, 0x30, dark, 0xc, G_GUI_COLOUR_GREY, TS_PLUS);
    } else if set {
        ts_ab(area, 4, row, 0x30, dark, 0xc, G_GUI_COLOUR_PALE_GREY, TS_PLUS, ids[2], 0);
    } else {
        ts_ab(area, 4, row, 0x30, dark, 0xc, G_GUI_COLOUR_GREY, TS_PLUS, ids[0], 0);
    }
    item
}

/// One of the appearance clause rows (the label already in `TS_TEXT`; `fee_at` the clause fee, `games` its games).
/// `ids`: the zero id, the games link id, the "-" id over 1000 and the "+" id. Read only: grey/pale label text, "-" grey
/// text (item param `param`), the value (a black button for the league clause, `value_field` = black; plain text for
/// the international one) and "+" grey; no popup (-1). Editable: with a fee the label is a button without text and the
/// label plus the games (a link) are drawn wrapped over it; "-" is a button (zero id, grey) without a fee, grey text up
/// to 1000, a button over 1000; the value a button (red over the budget) with a popup; "+" a button (zero id) without a
/// fee, grey text from 100000000, a button otherwise. Returns the popup's area (or -1).
unsafe fn ts_clause_row(t: usize, area: u32, row: &mut i32, read_only: bool, fee_at: usize, games: i8, ids: [u32; 4], value_field: u32, param: u32,
    value_colour: &dyn Fn(u32) -> usize) -> u32 {
    let format: TsLang = core::mem::transmute(FN_CRT_SPRINTF);
    let text_set: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(FN_TEXT_SET_STRING);
    let text_link: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> i32 = core::mem::transmute(FN_TEXT_ADD_LINK);
    let text_clear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(TS_FN_TEXT_CLEAR);
    let draw: unsafe extern "thiscall" fn(usize, u32, i32, i32, *mut i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(TS_FN_TEXT_DRAW_WRAPPED);
    let dark = ts_colour(G_GUI_COLOUR_DARK_BLUE);
    let fee = read_i32(fee_at);
    let money = || {
        if read_i32(fee_at) > 0 {
            ts_money_text(read_u32(fee_at), 5);
        } else {
            format(TS_TEXT, TS_MINUS);
        }
    };
    if read_only {
        ts_at(area, 1, *row, 1, 0, 1, if fee > 0 { G_GUI_COLOUR_PALE_GREY } else { G_GUI_COLOUR_GREY }, TS_TEXT);
        let item = ts_at(area, 2, *row, 0x30, dark, 0xc, G_GUI_COLOUR_GREY, TS_MINUS);
        set_screen_param(param, item as i16 as i32 as usize);
        money();
        if value_field != 0 {
            ts_ab(area, 3, *row, 1, value_field, 0xc, G_GUI_COLOUR_PALE_GREY, TS_TEXT, 0, 0);
        } else {
            ts_at(area, 3, *row, 1, 0, 0xc, G_GUI_COLOUR_PALE_GREY, TS_TEXT);
        }
        ts_at(area, 4, *row, 0x30, dark, 0xc, G_GUI_COLOUR_GREY, TS_PLUS);
        return u32::MAX;
    }
    if fee > 0 {
        ts_ab(area, 1, *row, 1, 0, 1, G_GUI_COLOUR_PALE_GREY, 0, ids[0], 0);
        text_clear(t);
        text_set(t, TS_TEXT);
        format(TS_TEXT, 0x98806c, games as i32); // "%d"
        text_link(t, TS_TEXT, ids[1], 0);
        draw(t, area, 0x94, 0x1e3, row, 3, ts_colour(G_GUI_COLOUR_PALE_GREY), 0, 0);
    } else {
        ts_ab(area, 1, *row, 1, 0, 1, G_GUI_COLOUR_GREY, TS_TEXT, ids[0], 0);
    }
    let fee = read_i32(fee_at);
    let item = if fee <= 0 {
        ts_ab(area, 2, *row, 0x30, dark, 0xc, G_GUI_COLOUR_GREY, TS_MINUS, ids[0], 0)
    } else if fee <= 1000 {
        ts_at(area, 2, *row, 0x30, dark, 0xc, G_GUI_COLOUR_GREY, TS_MINUS)
    } else {
        ts_ab(area, 2, *row, 0x30, dark, 0xc, G_GUI_COLOUR_PALE_GREY, TS_MINUS, ids[2], 0)
    };
    set_screen_param(param, item as i16 as i32 as usize);
    money();
    let colour = value_colour(read_u32(fee_at));
    let item = ts_ab(area, 3, *row, 1, value_field, 0xc, colour, TS_TEXT, 0, 0);
    let popup = ts_popup_for(item);
    let fee = read_i32(fee_at);
    if fee <= 0 {
        ts_ab(area, 4, *row, 0x30, dark, 0xc, G_GUI_COLOUR_GREY, TS_PLUS, ids[0], 0);
    } else if fee >= 100000000 {
        ts_at(area, 4, *row, 0x30, dark, 0xc, G_GUI_COLOUR_GREY, TS_PLUS);
    } else {
        ts_ab(area, 4, *row, 0x30, dark, 0xc, G_GUI_COLOUR_PALE_GREY, TS_PLUS, ids[3], 0);
    }
    popup
}

/// `TRANSFER_OFFER::TRANSFER_OFFER()` (0x8e5680, thiscall, no arguments).
const TS_FN_NEW_EMPTY_TRANSFER_OFFER: usize = 0x8e5680;
/// `Cash::step_down(mode)` (0x43f580, thiscall ret 4) and `Cash::set_from_display(amount)` (0x43f4c0, thiscall, an
/// i64 by value).
const TS_FN_CASH_STEP_DOWN: usize = 0x43f580;
const TS_FN_CASH_SET_FROM_DISPLAY: usize = 0x43f4c0;
/// `transfer_manager_selling_club_must_accept(offer, bid, out reason)` (0x8cd240, thiscall on the transfer manager).
const TS_FN_SELLING_CLUB_MUST_ACCEPT: usize = 0x8cd240;
/// `transfer_manager_get_squad_status_reason(offer)` (0x8c54c0, thiscall on the transfer manager): the reason shown
/// first in the reject dialog.
const TS_FN_SQUAD_STATUS_REASON: usize = 0x8c54c0;
/// `open_add_part_exchange_player_screen(offer)` (0x47c0b0, cdecl).
const TS_FN_OPEN_ADD_EXCHANGE_PLAYER: usize = 0x47c0b0;
/// The leave function of the bid screen.
const TS_BID_SCREEN_LEAVE: usize = 0x93a710;

/// Opens the transfer bid screen for an offer (when `open` is set; otherwise only fills the params of the current
/// screen) and fills its params: 0 the target's id, 1 the offer slot, 2 0, 3 the offer's serial, 4..6 0, 8 whether the
/// player's contract club is in season (`key_nation_club_in_season`), 9 whether the buyer can sign him today
/// (`TRANSFER_MANAGER::can_sign_player`), 7 whether the offer is at status 5 or more (accepted), 0xf..0x16 -1 (the item
/// params the build writes). A new offer (`offer::PLAYER_AWARE` 0) becomes 2 (made public when sent).
/// Original: `transfer_screens_open_bid_screen_for_offer` (0x8e9550, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_bid_screen_for_offer(slot: usize, open: u32) {
    if open != 0 && exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8e9760usize, 0x8ec420usize, 1, TS_BID_SCREEN_LEAVE, 0) == 0 {
        return;
    }
    covered!("tel17_1 open bid screen for offer");
    set_screen_param(0, read_u32(read_usize(slot) + offer::TARGET_STAFF_ID) as usize);
    set_screen_param(1, slot);
    set_screen_param(2, 0);
    set_screen_param(3, read_u32(read_usize(slot) + offer::SERIAL) as usize);
    set_screen_param(4, 0);
    set_screen_param(5, 0);
    set_screen_param(6, 0);
    let o = read_usize(slot);
    let player = staff_at(read_i32(o + offer::TARGET_STAFF_ID) as isize).addr();
    let club = contract_club_cached(Staff(player));
    set_screen_param(8, exe::key_nation_club_in_season(club, 0usize, 0usize) as usize);
    if read_u8(o + offer::PLAYER_AWARE) == 0 {
        write_u8(o + offer::PLAYER_AWARE, 2);
    }
    let buyer = club_at(read_i32(read_usize(slot) + offer::BUYER_CLUB_ID) as isize).addr();
    let can = exe::transfer_manager_can_sign_player(read_usize(G_TRANSFER_MANAGER), buyer, G_CURRENT_DATE, 0usize, 0usize, 0, core::ptr::null_mut());
    set_screen_param(9, (can != 0) as usize);
    let accepted = slot != 0 && read_usize(slot) != 0 && read_i8(read_usize(slot) + offer::STATUS) >= 5;
    set_screen_param(7, accepted as usize);
    for k in 0xf..=0x16 {
        set_screen_param(k, usize::MAX);
    }
}

/// Comparer of `transfer_screens_open_bid_screen_for_offer`: only the original runs (it opens a screen and marks the
/// offer).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_bid_screen_for_offer(slot: usize, open: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32)>(%%I0%%)(slot, open)
}

/// Comparer of `transfer_screens_build_bid_screen`: side by side over the display list, the human's bid (0x41 bytes)
/// and the contract lookup global; when the bid is still to be created (empty slot) only the original runs (it
/// allocates it).
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_bid_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    let bid_slot: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(TS_FN_BID_OFFER_SLOT);
    if tel26_slot_manager().addr() == 0 || read_usize(G_HUMAN_MANAGER) == 0 {
        return o();
    }
    let slot = bid_slot(read_usize(G_HUMAN_MANAGER), 0);
    let bid = read_usize(slot);
    if bid == 0 {
        // simplification: creating the bid (operator new) does not run twice
        return o();
    }
    compare_by_display_list_with("Transfer bid screen", &[(bid, 0x41), (G_CONTRACT_LOOKUP_RESULT, 4)], || o(), || transfer_screens_build_bid_screen());
}

/// Steps a cash value (`Cash::step_up` / `step_down` in mode 4) and returns it.
unsafe fn ts_step(value: u32, up: bool) -> u32 {
    let down: unsafe extern "thiscall" fn(*mut u32, u32) -> i32 = core::mem::transmute(TS_FN_CASH_STEP_DOWN);
    let mut cash = value;
    if up {
        exe::cash_step_up(&mut cash as *mut u32 as usize, 4);
    } else {
        down(&mut cash, 4);
    }
    cash
}

/// The human's transfer budget for `staff` (`Finance::get_transfer_budget(staff, 0, 0, 1, 0)` of the slot manager's
/// club), the low dword.
unsafe fn ts_budget_of(club: usize, staff: usize) -> u32 {
    let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    exe::finance_get_transfer_budget(fin, staff as u32, 0, 0usize, 1, 0) as u32
}

/// A widget of the bid screen set to a state: flags, text colour and id.
unsafe fn ts_widget(item: u32, flags: u32, colour: usize, id: u32) {
    exe::screenmanager_set_widget_flags(G_SCREEN_MANAGER, item as usize, flags);
    exe::screenmanager_set_widget_text_colour(G_SCREEN_MANAGER, item, ts_colour(colour));
    exe::screenmanager_set_widget_id(G_SCREEN_MANAGER, item, id);
}

/// The value widget of a money row after a step: the formatted value (mode 5) and, when bidding, red over the budget.
unsafe fn ts_value_widget(item: u32, value: u32, bidding: bool, budget: u32) {
    ts_money_text(value, 5);
    exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, item, TS_TEXT);
    let colour = if bidding && ts_over_budget(value, budget) { G_GUI_COLOUR_RED } else { G_GUI_COLOUR_PALE_GREY };
    exe::screenmanager_set_widget_text_colour(G_SCREEN_MANAGER, item, ts_colour(colour));
}

/// The bid is replaced by a new copy of `src` (`ts_copy_offer`, the list copied from `src`'s) and param `param` set to 0.
unsafe fn ts_reset_bid(slot: usize, src: usize, param: u32) {
    let new_empty: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(TS_FN_NEW_EMPTY_TRANSFER_OFFER);
    let old = read_usize(slot);
    if old != 0 {
        let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
        destroy(old);
        exe::crt_operator_delete(old);
    }
    let mem = exe::crt_operator_new(0x41);
    let o = if mem != 0 { new_empty(mem) } else { 0 };
    write_usize(slot, o);
    ts_copy_offer(o, src);
    ts_copy_offer_list(read_usize(slot), read_usize(src + offer::EXCHANGE_PLAYERS));
    set_screen_param(param, 0);
}

/// Opens one of the bid dialogs: build/input `screen`, params 0 the bid slot, 1 `action`, 2 `extra`; -4 either way.
unsafe fn ts_open_bid_dialog(build: usize, input: usize, slot: usize, action: u32, extra: usize) -> i32 {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, build, input, 1, 0usize, 0) != 0 {
        set_screen_param(0, slot);
        set_screen_param(1, action as usize);
        set_screen_param(2, extra);
    }
    -4
}

/// The bid is dropped (deleted, slot cleared) and the offer changed screen opens for `live` with `serial`; -11.
unsafe fn ts_bid_overtaken(slot: usize, live: usize, serial: usize) -> i32 {
    ts_delete_offer(slot);
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8f24c0usize, 0x8f2800usize, 1, 0usize, 0) != 0 {
        set_screen_param(0, live);
        set_screen_param(1, serial);
    }
    -11
}

/// Answers the transfer bid screen.
///
/// The human's bid is `HUMAN_MANAGER::get_bid_offer_slot(0)`; param 1 is the slot of the offer answered and param 3 its
/// serial when the screen opened. Back (0x21), or a player without a contract (or of type 5, or without its club): the
/// bid is deleted, -9. A bid of the other bids popup (0x27): the bid is deleted and the bid screen reopens for that
/// offer (`transfer_screens_open_bid_screen_for_offer(slot, 0)`), -11. The actions (delay 0x23, cancel 2, offer 1,
/// reject 0x1b, negotiate 0x1c, accept 0x1d): when the offer answered changed (serial) or was rejected or withdrawn, the
/// bid is dropped and the offer changed screen opens (-11); otherwise: delay, reject (when not bidding, cancel too) ask
/// `transfer_manager_selling_club_must_accept` and open the must accept dialog (with its reason) or the confirm dialog
/// (the reject dialog for reject, with `transfer_manager_get_squad_status_reason`); offer opens the confirm dialog;
/// negotiate and accept check the offer again (status 0 or 1) and open the must accept or confirm dialog: -4.
/// The edits set params 2 and 5 (the bid changed) and answer -11 (redraw) or, with the widgets updated in place and
/// the total and buttons refreshed (`transfer_screens_refresh_bid_total_and_buttons`, `redraw_screen(0, 0)`), 0:
/// the fee popup (6), fee + (3) / - (4) (`Cash::step_up/step_down`; "-" greyed at 0, "+" greyed over 99999999, the value
/// red over the budget when bidding), "=" (5: the player's value less the exchange players', at least 0, at most the
/// budget when the human's club is the buyer), the value (0x26), the budget (0x25: when the human's club is the buyer;
/// otherwise the item argument itself); instalments on/off (7: negated, 6 when it was off), - (9) and + (8) by 6
/// (6..24); sell-on on/off (0xa: 10), - (0xc) and + (0xb) by 5 (5..25); the league clause popup (0x11), on/off (0xd:
/// a quarter of the player's value, at least 1000), the games (0xe: 1, then 10, 20 .. 50, then 1), - (0x10) and +
/// (0xf); the same for the international clause (0x16, 0x12, 0x13, 0x15, 0x14); public (0x22: 0 and 2 swap, 1 becomes
/// 0); the completion timing (0x24: to 2, or back to 0 / 1 by param 9); Exchange (0x19: the add part exchange player
/// screen, -4); Reset (0x1a) and Negotiated Bid (0x20): the bid is copied again from the offer answered or its
/// negotiation (or dropped when the offer changed); Remove (0x1e: param 4 = the item's value); an exchange player
/// (0x1f): his value goes back to the fee and he leaves the list (the list deleted when empty), param 4 = 0. The rest
/// goes to the menu bar.
/// Original: `transfer_screens_handle_bid_screen_action` (0x8ec420, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_handle_bid_screen_action(item: u32) -> i32 {
    let bid_slot: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(TS_FN_BID_OFFER_SLOT);
    let must_accept: unsafe extern "thiscall" fn(usize, usize, usize, *mut u32) -> u32 = core::mem::transmute(TS_FN_SELLING_CLUB_MUST_ACCEPT);
    let status_reason: unsafe extern "thiscall" fn(usize, usize) -> i16 = core::mem::transmute(TS_FN_SQUAD_STATUS_REASON);
    let negotiation_slot: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(TS_FN_GET_NEGOTIATION_SLOT);
    let set_from_display: unsafe extern "thiscall" fn(*mut u32, i64) = core::mem::transmute(TS_FN_CASH_SET_FROM_DISPLAY);
    let add_exchange: unsafe extern "C" fn(usize) = core::mem::transmute(TS_FN_OPEN_ADD_EXCHANGE_PLAYER);
    let format: TsLang = core::mem::transmute(FN_CRT_SPRINTF);
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let slot = bid_slot(read_usize(G_HUMAN_MANAGER), 0);
    let bid = read_usize(slot);
    let live_slot = get_screen_param(1);
    let serial = get_screen_param(3);
    let staff = staff_at(read_i32(bid + offer::TARGET_STAFF_ID) as isize).addr();
    let id = tel26_item_id(item as i16);
    let tm = read_usize(G_TRANSFER_MANAGER);
    let value_of = |p: usize| read_u32(p + 0x52);
    if id == 0x21 {
        ts_delete_offer(slot);
        return -9;
    }
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    if c == 0 || read_u8(c + 0x35) & 0x3f == 5 || club_at(read_i32(c + 4) as isize).addr() == 0 {
        ts_delete_offer(slot);
        return -9;
    }
    if id == 0x27 {
        covered!("tel17_1 bid screen: another bid");
        let open_bid: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x8e9550usize);
        ts_delete_offer(slot);
        open_bid(exe::transfer_manager_get_offer_slot(tm, tel26_item_value(item as i16) as i32), 0);
        return -11;
    }
    if matches!(id, 0x23 | 2 | 1 | 0x1b | 0x1c | 0x1d) && live_slot != 0 {
        let live = read_usize(live_slot);
        if read_usize(live + offer::SERIAL) != serial || matches!(read_u8(live + offer::STATUS), 2 | 3) {
            covered!("tel17_1 bid screen: the offer changed");
            return ts_bid_overtaken(slot, live, serial);
        }
    }
    let confirm = (0x8ef240usize, 0x8ef9f0usize);
    let must = (0x8f0bc0usize, 0x8f16c0usize);
    let mut why: u32 = item;
    match id {
        0x23 => {
            if must_accept(tm, read_usize(live_slot), 0, &mut why) != 0 {
                return ts_open_bid_dialog(must.0, must.1, slot, 0x23, why as i16 as i32 as usize);
            }
            return ts_open_bid_dialog(confirm.0, confirm.1, slot, 0x23, live_slot);
        }
        2 => {
            if get_screen_param(6) == 0 && must_accept(tm, read_usize(live_slot), 0, &mut why) != 0 {
                return ts_open_bid_dialog(must.0, must.1, slot, 0x1b, why as i16 as i32 as usize);
            }
            return ts_open_bid_dialog(confirm.0, confirm.1, slot, 2, live_slot);
        }
        1 => return ts_open_bid_dialog(confirm.0, confirm.1, slot, 1, 0),
        0x1b => {
            if must_accept(tm, read_usize(live_slot), 0, &mut why) != 0 {
                return ts_open_bid_dialog(must.0, must.1, slot, 0x1b, why as i16 as i32 as usize);
            }
            if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8efda0usize, 0x8f06f0usize, 1, 0usize, 0) != 0 {
                set_screen_param(0, slot);
                set_screen_param(1, live_slot);
                set_screen_param(2, status_reason(tm, read_usize(slot)) as i32 as usize);
            }
            return -4;
        }
        0x1c | 0x1d => {
            let live = read_usize(live_slot);
            if read_usize(live + offer::SERIAL) != get_screen_param(3) || !matches!(read_u8(live + offer::STATUS), 0 | 1) {
                return ts_bid_overtaken(slot, read_usize(live_slot), get_screen_param(3));
            }
            if id == 0x1c && must_accept(tm, live, read_usize(slot), &mut why) != 0 {
                return ts_open_bid_dialog(must.0, must.1, slot, 0x1c, why as i16 as i32 as usize);
            }
            return ts_open_bid_dialog(confirm.0, confirm.1, slot, id as u32, live_slot);
        }
        _ => {}
    }
    let changed = || {
        set_screen_param(2, 1);
        set_screen_param(5, 1);
    };
    let human = ts_human_club();
    let bidding = || get_screen_param(6) != 0;
    let refresh_and_redraw = |bid: usize| {
        transfer_screens_refresh_bid_total_and_buttons(bid, live_slot);
        exe::screenmanager_redraw_screen(G_SCREEN_MANAGER, 0, 0);
        0
    };
    match id {
        6 => {
            write_u32(bid + offer::FEE, tel26_item_value(item as i16));
            -11
        }
        3 | 4 => {
            covered!("tel17_1 bid screen: fee step");
            let budget = ts_budget_of(human, staff);
            let b = bidding();
            changed();
            write_u32(bid + offer::FEE, ts_step(read_u32(bid + offer::FEE), id == 3));
            let w = get_screen_param(0xf) as u32;
            let fee = read_i32(bid + offer::FEE);
            if id == 4 && fee <= 0 {
                ts_widget(w + 1, 6, G_GUI_COLOUR_GREY, 0);
            } else {
                ts_widget(w + 1, 2, G_GUI_COLOUR_PALE_GREY, 4);
            }
            ts_value_widget(w + 2, read_u32(bid + offer::FEE), b, budget);
            if id == 3 && read_i32(bid + offer::FEE) > 99999999 {
                ts_widget(w + 3, 6, G_GUI_COLOUR_GREY, 0);
            } else {
                ts_widget(w + 3, 2, G_GUI_COLOUR_PALE_GREY, 3);
            }
            refresh_and_redraw(bid)
        }
        5 => {
            changed();
            let mut v = ts_display(value_of(staff), 4);
            let ex = read_usize(bid + offer::EXCHANGE_PLAYERS);
            if ex != 0 {
                write_u32(ex + llist::CURSOR, read_u32(ex + llist::HEAD));
                loop {
                    let node = exe::llist_get_next(read_usize(bid + offer::EXCHANGE_PLAYERS));
                    if node == 0 {
                        break;
                    }
                    v = v.wrapping_sub(ts_display(value_of(staff_at(read_i32(node) as isize).addr()), 4));
                }
            }
            if v < 0 {
                v = 0;
            }
            let mut cash: u32 = 0;
            set_from_display(&mut cash, v);
            let buyer = club_at(read_i32(bid + offer::BUYER_CLUB_ID) as isize).addr();
            if human == buyer {
                let budget = ts_budget_of(buyer, staff);
                if cash as i32 > budget as i32 {
                    write_u32(bid + offer::FEE, budget);
                    return -11;
                }
            }
            write_u32(bid + offer::FEE, cash);
            -11
        }
        0x26 => {
            changed();
            write_u32(bid + offer::FEE, value_of(staff));
            -11
        }
        0x25 => {
            let buyer = club_at(read_i32(bid + offer::BUYER_CLUB_ID) as isize).addr();
            // defect of the original kept: with another buyer the fee becomes the item argument itself
            let fee = if human == buyer { ts_budget_of(buyer, staff) } else { item };
            changed();
            write_u32(bid + offer::FEE, fee);
            -11
        }
        7 | 0xa => {
            changed();
            let at = bid + if id == 7 { offer::INSTALMENTS } else { offer::SELL_ON_PERCENT };
            let v = (read_i8(at) as i8).wrapping_neg();
            write_i8(at, v);
            if v == 0 {
                write_u8(at, if id == 7 { 6 } else { 10 });
            }
            -11
        }
        9 | 8 | 0xc | 0xb => {
            covered!("tel17_1 bid screen: instalments or sell-on step");
            changed();
            let instalments = id == 9 || id == 8;
            let (at, step, floor, cap, ids, param) = if instalments {
                (bid + offer::INSTALMENTS, 6i8, 6i8, 0x18i8, (9u32, 8u32), 0x10u32)
            } else {
                (bid + offer::SELL_ON_PERCENT, 5, 5, 0x19, (0xc, 0xb), 0x11)
            };
            let down = id == 9 || id == 0xc;
            write_i8(at, read_i8(at).wrapping_add(if down { -step } else { step }));
            let w = get_screen_param(param) as u32;
            if down && read_i8(at) == floor {
                ts_widget(w + 1, 6, G_GUI_COLOUR_GREY, 0);
            } else {
                ts_widget(w + 1, 2, G_GUI_COLOUR_PALE_GREY, ids.0);
            }
            if instalments {
                lang(TS_TEXT, 0xa9e4c8, read_i8(at) as i32);
            } else {
                format(TS_TEXT, 0x98e8d4, read_i8(at) as i32);
            }
            exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, w + 2, TS_TEXT);
            if !down && read_i8(at) >= cap {
                ts_widget(w + 3, 6, G_GUI_COLOUR_GREY, 0);
            } else {
                ts_widget(w + 3, 2, G_GUI_COLOUR_PALE_GREY, ids.1);
            }
            refresh_and_redraw(bid)
        }
        0x11 | 0x16 => {
            let at = bid + if id == 0x11 { offer::APPEARANCE_FEE } else { offer::INTERNATIONAL_FEE };
            write_u32(at, tel26_item_value(item as i16));
            -11
        }
        0xd | 0x12 => {
            changed();
            let at = bid + if id == 0xd { offer::APPEARANCE_FEE } else { offer::INTERNATIONAL_FEE };
            let v = read_i32(at).wrapping_neg();
            write_i32(at, v);
            if v == 0 {
                let mut f = read_i32(staff_at(read_i32(bid + offer::TARGET_STAFF_ID) as isize).addr() + 0x52) as f64 * read_f64(rdata::F64_0_25);
                let floor = read_f64(rdata::F64_1000);
                if f < floor {
                    f = floor;
                }
                write_u32(at, f as i64 as u32);
            }
            -11
        }
        0xe | 0x13 => {
            changed();
            let at = bid + if id == 0xe { offer::APPEARANCE_GAMES } else { offer::INTERNATIONAL_GAMES };
            if read_i8(at) == 1 {
                write_u8(at, 10);
            } else {
                write_i8(at, read_i8(at).wrapping_add(10));
            }
            if read_i8(at) >= 0x33 {
                write_u8(at, 1);
            }
            -11
        }
        0x10 | 0xf | 0x15 | 0x14 => {
            covered!("tel17_1 bid screen: clause step");
            let league = id == 0x10 || id == 0xf;
            let (at, param, ids) = if league {
                (bid + offer::APPEARANCE_FEE, 0x12u32, (0x10u32, 0xfu32))
            } else {
                (bid + offer::INTERNATIONAL_FEE, 0x13, (0x15, 0x14))
            };
            let down = id == 0x10 || id == 0x15;
            let budget = ts_budget_of(human, staff);
            let b = bidding();
            changed();
            write_u32(at, ts_step(read_u32(at), !down));
            let w = get_screen_param(param) as u32;
            if down && read_i32(at) < 0x3e9 {
                ts_widget(w, 6, G_GUI_COLOUR_GREY, 0);
            } else {
                ts_widget(w, 2, G_GUI_COLOUR_PALE_GREY, ids.0);
            }
            ts_value_widget(w + 1, read_u32(at), b, budget);
            if !down && read_i32(at) >= 100000000 {
                ts_widget(w + 2, 6, G_GUI_COLOUR_GREY, 0);
            } else {
                ts_widget(w + 2, 2, G_GUI_COLOUR_PALE_GREY, ids.1);
            }
            refresh_and_redraw(bid)
        }
        0x22 => {
            write_u8(bid + offer::PLAYER_AWARE, ((read_u8(bid + offer::PLAYER_AWARE) != 0) as u8).wrapping_sub(1) & 2);
            -11
        }
        0x24 => {
            let b = read_u8(bid + offer::COMPLETION_TIMING);
            if b & 0xf != 2 {
                write_u8(bid + offer::COMPLETION_TIMING, b & 0xf2 | 2);
            } else if get_screen_param(9) != 0 {
                write_u8(bid + offer::COMPLETION_TIMING, b & 0xf0);
            } else {
                write_u8(bid + offer::COMPLETION_TIMING, b & 0xf1 | 1);
            }
            -11
        }
        0x19 => {
            changed();
            add_exchange(bid);
            -4
        }
        0x1a | 0x20 => {
            covered!("tel17_1 bid screen: reset or negotiated bid");
            if id == 0x1a {
                set_screen_param(5, 1);
            } else {
                set_screen_param(2, 1);
            }
            let live = read_usize(live_slot);
            if read_usize(live + offer::SERIAL) == get_screen_param(3) && matches!(read_u8(live + offer::STATUS), 0 | 1) {
                if id == 0x1a {
                    ts_reset_bid(slot, live, 2);
                } else {
                    let ns = negotiation_slot(tm, read_i32(live + offer::NEGOTIATION_INDEX));
                    ts_reset_bid(slot, read_usize(ns), 5);
                }
                return -11;
            }
            ts_bid_overtaken(slot, read_usize(live_slot), get_screen_param(3))
        }
        0x1e => {
            set_screen_param(4, tel26_item_value(item as i16) as usize);
            -11
        }
        0x1f => {
            covered!("tel17_1 bid screen: exchange player removed");
            let node = tel26_item_value(item as i16) as usize;
            changed();
            let back = value_of(staff_at(read_i32(node) as isize).addr());
            write_u32(bid + offer::FEE, read_u32(bid + offer::FEE).wrapping_add(back));
            exe::llist_remove(read_usize(bid + offer::EXCHANGE_PLAYERS), node);
            let l = read_usize(bid + offer::EXCHANGE_PLAYERS);
            if read_i16(l) == 0 {
                if l != 0 {
                    llist_drop(l);
                    exe::crt_operator_delete(l);
                }
                write_usize(bid + offer::EXCHANGE_PLAYERS, 0);
            }
            set_screen_param(4, 0);
            -11
        }
        _ => ts_menubar_answer(item),
    }
}

/// Comparer of `transfer_screens_handle_bid_screen_action`: the edits of the bid (steps, toggles, popups, the public
/// and timing marks, Remove) run side by side over the display list, the screen params and the human's bid (0x41
/// bytes); Back, the actions, Exchange, Reset, Negotiated Bid, an exchange player, another bid and the menu bar only
/// in the original (they delete or replace the bid, change its list or open screens).
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_bid_screen_action(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%);
    let bid_slot: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(TS_FN_BID_OFFER_SLOT);
    let id = tel26_item_id(item as i16);
    let edit = matches!(id, 3 | 4 | 5 | 6 | 7 | 8 | 9 | 0xa | 0xb | 0xc | 0xd | 0xe | 0xf | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16
        | 0x22 | 0x24 | 0x25 | 0x26 | 0x1e);
    if !edit || read_usize(G_HUMAN_MANAGER) == 0 {
        // simplification: deleting or replacing the bid and opening screens do not run twice
        return o(item);
    }
    let bid = read_usize(bid_slot(read_usize(G_HUMAN_MANAGER), 0));
    if bid == 0 {
        return o(item);
    }
    let staff = staff_at(read_i32(bid + offer::TARGET_STAFF_ID) as isize).addr();
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff);
    if c == 0 || read_u8(c + 0x35) & 0x3f == 5 {
        // the bid is deleted (-9)
        return o(item);
    }
    let mut reg = screen_build_regions();
    reg.extend(screen_param_regions());
    reg.push((TS_TEXT, 0x100));
    reg.push((0xae290c, 4));
    reg.push((bid, 0x41));
    reg.push((G_CONTRACT_LOOKUP_RESULT, 4));
    reg.extend_from_slice(&crate::rng::STATE_REGIONS);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&reg, || o(item), || crate::rng::without_counting(|| transfer_screens_handle_bid_screen_action(item)));
    SCOREBOARD.record(equal && ro == rr, || format!("Transfer bid screen answer to item {} (id {:#x}): original={} rust={} differences at {:?}",
        item as i16, id, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Recalculates the bid's Total Value widget (item param 0x14: the fee, the clauses that are on (not negative) and the
/// exchange players' values, as display amounts, formatted with mode 0x22) and, with an offer answered, the action
/// button (item param 0x15) and Reset (0x16): when the bid differs from the offer, "Negotiate" (id 0x1c) or, for an
/// offer that can only be accepted (`offer::OFFER_KIND` low nibble set), a greyed "Accept", and Reset enabled (id
/// 0x1a); when it is the same, "Accept" (id 0x1d) and Reset greyed.
/// Original: `transfer_screens_refresh_bid_total_and_buttons` (0x8eef60, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_refresh_bid_total_and_buttons(bid: usize, live_slot: usize) {
    let same_terms: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(TS_FN_HAS_SAME_TERMS);
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let sm = G_SCREEN_MANAGER;
    let total_item = get_screen_param(0x14) as u32;
    let mut total = ts_display(read_u32(bid + offer::FEE), 4);
    if read_i32(bid + offer::APPEARANCE_FEE) >= 0 {
        total = total.wrapping_add(ts_display(read_u32(bid + offer::APPEARANCE_FEE), 4));
    }
    if read_i32(bid + offer::INTERNATIONAL_FEE) >= 0 {
        total = total.wrapping_add(ts_display(read_u32(bid + offer::INTERNATIONAL_FEE), 4));
    }
    let ex = read_usize(bid + offer::EXCHANGE_PLAYERS);
    if ex != 0 {
        write_u32(ex + llist::CURSOR, read_u32(ex + llist::HEAD));
        loop {
            let node = exe::llist_get_next(read_usize(bid + offer::EXCHANGE_PLAYERS));
            if node == 0 {
                break;
            }
            total = total.wrapping_add(ts_display(read_u32(staff_at(read_i32(node) as isize).addr() + 0x52), 4));
        }
    }
    exe::cash_format_money(TS_TEXT, 0x7d0, total, 0x22);
    exe::screenmanager_set_widget_text(sm, total_item, TS_TEXT);
    if live_slot == 0 {
        return;
    }
    covered!("tel17_1 refresh bid buttons");
    if same_terms(bid, read_usize(live_slot)) == 0 {
        let w = get_screen_param(0x15) as u32;
        if read_u8(read_usize(live_slot) + offer::OFFER_KIND) & 0xf == 0 {
            lang(TS_TEXT, 0xa6943c); // "Negotiate"
            exe::screenmanager_set_widget_text(sm, w, TS_TEXT);
            exe::screenmanager_set_widget_id(sm, w, 0x1c);
            exe::screenmanager_set_widget_text_flags(sm, w, 0xc);
        } else {
            lang(TS_TEXT, 0xa9e3bc); // "Accept"
            exe::screenmanager_set_widget_text(sm, w, TS_TEXT);
            exe::screenmanager_set_widget_id(sm, w, 0);
            exe::screenmanager_set_widget_text_flags(sm, w, 0x2c);
        }
        let r = get_screen_param(0x16) as u32;
        exe::screenmanager_set_widget_flags(sm, r as usize, 2);
        exe::screenmanager_set_widget_text_flags(sm, r, 0xc);
        exe::screenmanager_set_widget_id(sm, r, 0x1a);
    } else {
        let w = get_screen_param(0x15) as u32;
        lang(TS_TEXT, 0xa9e3bc); // "Accept"
        exe::screenmanager_set_widget_text(sm, w, TS_TEXT);
        exe::screenmanager_set_widget_id(sm, w, 0x1d);
        exe::screenmanager_set_widget_text_flags(sm, w, 0xc);
        let r = get_screen_param(0x16) as u32;
        exe::screenmanager_set_widget_flags(sm, r as usize, 1);
        exe::screenmanager_set_widget_text_flags(sm, r, 0x2c);
        exe::screenmanager_set_widget_id(sm, r, 0);
    }
}

/// Comparer of `transfer_screens_refresh_bid_total_and_buttons`: side by side over the display list and the screen
/// params (the widgets are updated in place).
pub(crate) unsafe extern "C" fn compare_transfer_screens_refresh_bid_total_and_buttons(bid: usize, live_slot: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize)>(%%I3%%);
    compare_by_display_list_with("Transfer bid total and buttons", &[], || o(bid, live_slot), || transfer_screens_refresh_bid_total_and_buttons(bid, live_slot));
}
