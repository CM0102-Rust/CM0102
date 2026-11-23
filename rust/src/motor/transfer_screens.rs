use super::*;

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
    let text_clear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(TS_FN_TEXT_CLEAR);
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
    exe::new_text((t) as usize as *mut usize, 0);
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
        exe::drop_text((t) as usize as *mut usize);
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
    let human = Club(ts_human_club());
    if human.is_null() {
        tel26_error_box(TS_CPP, 0x222);
        return finish(t, l);
    }
    covered!("tel17_1 bid screen");
    let slot = bid_slot(read_usize(G_HUMAN_MANAGER), 0);
    if read_usize(slot) == 0 {
        covered!("tel17_1 bid screen creates the bid");
        let mem = exe::crt_operator_new(0x41);
        let o = if mem != 0 { new_offer(mem, player, human.addr(), 0, -1, 0) } else { 0 };
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
    let bid = Offer(read_usize(slot));
    picture(player, 0, 0);
    let (mut kit_a, mut kit_b) = (0u32, 0u32);
    exe::database_get_club_kit_colours(human, 1, &mut kit_a, &mut kit_b);
    format(TS_TEXT, 0x991944, human.addr() + club::SHORT_NAME); // "%s"
    (exe::text_set_string(t, (TS_TEXT) as usize as *const u8) as i32);
    exe::text_add_link(t, (human.addr() + club::SHORT_NAME) as usize as *const u8, 0x7d5, human);
    let header = tel26_add_area(current_human_display(), [100, 0xa, 0x316, 0x46, 1, 0, 1, 0, 0x30, kit_b, u32::MAX]);
    let mut row: i32 = 0;
    draw(t, header, 100, 0x316, &mut row, 7, kit_a, 1, 0);

    // the other open bids for a player of the human's club, in a popup
    if bid.status() == 0 && answered != 0 {
        let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
        write_usize(G_CONTRACT_LOOKUP_RESULT, c);
        let own = if c == 0 || read_u8(c + 0x35) & 0x3f == 5 { 0 } else { club_at(read_i32(c + 4) as isize).addr() };
        if human.addr() == own {
            covered!("tel17_1 bid screen: the other bids");
            exe::transfer_manager_get_offers_for_player(read_usize(G_TRANSFER_MANAGER), player, l, 0);
            write_u32(l + llist::CURSOR, read_u32(l + llist::HEAD));
            loop {
                let node = exe::llist_get_next(l);
                if node == 0 {
                    break;
                }
                let other = Offer(read_usize(exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), read_i32(node))));
                if other.status() != 0 || (other.buyer_club_id() as u32) == (bid.buyer_club_id() as u32) {
                    continue;
                }
                let buyer = club_at((other.buyer_club_id() as i32) as isize).addr();
                if bids_popup as i16 == -1 {
                    let button = tel26_add_button([0x69, 0xf, 0x78, 0x23, 0x30, kit_b, 0xc, 1, kit_a, 0, 0, 0]);
                    bids_popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x30, ts_colour(TS_COLOUR_POPUP), button]);
                }
                tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, ts_colour(TS_COLOUR_POPUP), ts_colour(G_GUI_COLOUR_YELLOW), 1, 1,
                    ts_colour(G_GUI_COLOUR_BLACK), (buyer + club::NAME) as u32, 0, 0x27, (other.slot_index() as u32), bids_popup]);
            }
        }
    }

    // the title
    let mut name = [0u8; 400];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(player, n, 400, 5);
    let buyer = club_at((bid.buyer_club_id() as i32) as isize).addr();
    let g = |c: usize| read_i8(c + club::SHORT_GENDER_NAME) as i32;
    if bidding == 0 && read_only == 0 {
        lang(TS_TEXT, 0xa9e930, g(buyer), g(buyer), 100, buyer + club::SHORT_NAME, n);
    } else {
        lang(TS_TEXT, 0xa9e8dc, g(buyer), g(buyer), buyer + club::SHORT_NAME, n);
    }
    text_clear(t);
    (exe::text_set_string(t, (TS_TEXT) as usize as *const u8) as i32);
    exe::text_add_link(t, (n) as usize as *const u8, 0x7d7, player);
    exe::text_add_link(t, (buyer + club::SHORT_NAME) as usize as *const u8, 0x7d5, buyer);
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
    (exe::text_set_string(t, (TS_TEXT) as usize as *const u8) as i32);
    exe::text_add_link(t, (n) as usize as *const u8, 0x7d7, player);
    let (c4, c5) = (cc(), cc());
    exe::text_add_link(t, (c5 + club::SHORT_NAME) as usize as *const u8, 0x7d5, c4);
    if read_only == 0 {
        exe::text_add_link(t, (vt) as usize as *const u8, 0x26, 0);
    }
    (exe::text_set_string(t, (TS_TEXT) as usize as *const u8) as i32);
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
            let map = exe::fog_of_war_get_manager_map(read_usize(G_FOG_OF_WAR), read_usize(human.addr() + TS_CLUB_MANAGER));
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
        (exe::text_set_string(t, (TS_TEXT) as usize as *const u8) as i32);
        exe::text_add_link(t, (vt) as usize as *const u8, 0x25, 0);
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
    if fee(bid.addr()) > 0 && read_only == 0 {
        ts_ab(area, 2, row, 0x30, ts_colour_addr(DARK), 0xc, PALE, TS_MINUS, 4, 0);
    } else {
        ts_at(area, 2, row, 0x30, ts_colour_addr(DARK), 0xc, GREY, TS_MINUS);
    }
    ts_money_text(bid.fee() as u32, 5);
    let colour = value_colour(bid.fee() as u32);
    let item = ts_ab(area, 3, row, 1, ts_colour_addr(BLACK), 0xc, colour, TS_TEXT, 0, 0);
    let fee_popup = if read_only == 0 { ts_popup_for(item) } else { u32::MAX };
    if fee(bid.addr()) < 100000000 && read_only == 0 {
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
    let inst = bid.instalments() as i8;
    let item = ts_stepper_row(area, row, read_only != 0, inst, 6, 0x18, [7, 9, 8], ts_colour_addr(TS_COLOUR_POPUP), |v| {
        lang(TS_TEXT, 0xa9e4c8, v as i32);
    });
    set_screen_param(0x10, item as usize);
    row += 1;
    lang(TS_TEXT, 0xa9e4b0);
    let sell_on = bid.sell_on_percent() as i8;
    let item = ts_stepper_row(area, row, read_only != 0, sell_on, 5, 0x19, [0xa, 0xc, 0xb], 0, |v| {
        format(TS_TEXT, 0x98e8d4, v as i32); // "%d%%"
    });
    set_screen_param(0x11, item as usize);

    // the appearance clauses
    row += 1;
    let games = bid.appearance_games() as i8;
    if games == 1 {
        lang(TS_TEXT, 0xa9e498);
    } else {
        lang(TS_TEXT, 0xa9e468, games as i32);
    }
    let league_popup = ts_clause_row(t, area, &mut row, read_only != 0, bid.addr() + offer::APPEARANCE_FEE, games, [0xd, 0xe, 0x10, 0xf],
        ts_colour_addr(BLACK), 0x12, &value_colour);
    row += 1;
    let caps = bid.international_games() as i8;
    if caps == 1 {
        lang(TS_TEXT, 0xa9e448);
    } else {
        lang(TS_TEXT, 0xa9e414, caps as i32);
    }
    let caps_popup = ts_clause_row(t, area, &mut row, read_only != 0, bid.addr() + offer::INTERNATIONAL_FEE, caps, [0x12, 0x13, 0x15, 0x14],
        0, 0x13, &value_colour);
    row += 1;

    // Make transfer bid public
    if read_only == 0 {
        lang(TS_TEXT, 0xa9e3f8);
        let aware = bid.player_aware();
        if aware != 1 {
            ts_ab(area, 1, row, 1, 0, 1, PALE, TS_TEXT, 0x22, 0);
        } else {
            ts_at(area, 1, row, 1, 0, 1, PALE, TS_TEXT);
        }
        match bid.player_aware() {
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
    let ex = bid.exchange_players().addr();
    if (ex == 0 || read_i16(ex) < 5) && read_only == 0 {
        tel26_add_button([0x6e, 0x181, 0xeb, 0x195, 0x30, ts_colour(GREY), 0xc, 1, ts_colour(PALE), TS_TEXT as u32, 0x19, 0]);
    } else {
        add_text_item(G_SCREEN_MANAGER, 0x6e, 0x181, 0xeb, 0x195, 0x30, ts_colour(GREY), 0x2c, 1, ts_colour(PALE), TS_TEXT, 1);
    }
    lang(TS_TEXT, 0xa8b354); // "Remove"
    if bid.exchange_players().addr() != 0 && read_only == 0 {
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
        let item = if same_terms(bid.addr(), answered) == 0 {
            tel26_add_button([0x28f, 0x181, 0x30c, 0x195, 0x30, ts_colour(GREY), 0xc, 1, ts_colour(PALE), TS_TEXT as u32, 0x1a, 0])
        } else {
            add_text_item(G_SCREEN_MANAGER, 0x28f, 0x181, 0x30c, 0x195, 0x30, ts_colour(GREY), 0x2c, 1, ts_colour(PALE), TS_TEXT, 1)
        };
        set_screen_param(0x16, item as i16 as i32 as usize);
        if read_i32(answered + offer::NEGOTIATION_INDEX) != -1 {
            let ns = negotiation_slot(read_usize(G_TRANSFER_MANAGER), read_i32(answered + offer::NEGOTIATION_INDEX));
            lang(TS_TEXT, 0xa9e3e8); // "Negotiated Bid"
            if same_terms(bid.addr(), read_usize(ns)) == 0 {
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
    let mut total = ts_display(bid.fee() as u32, 4);
    if (bid.appearance_fee() as i32) >= 0 {
        total = total.wrapping_add(ts_display(bid.appearance_fee() as u32, 4));
    }
    if (bid.international_fee() as i32) >= 0 {
        total = total.wrapping_add(ts_display(bid.international_fee() as u32, 4));
    }
    let ex = bid.exchange_players().addr();
    if ex != 0 {
        write_u32(ex + llist::CURSOR, read_u32(ex + llist::HEAD));
        loop {
            let node = exe::llist_get_next(bid.exchange_players().addr());
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
        let three = bid.response_round() >= 3;
        let bar = if three { bar_with(3, wb3.as_ptr()) } else { bar_with(4, w4.as_ptr()) };
        button(bar, 0, TS_BACK, 0x21);
        button(bar, 1, 0xa9e3c4, 0x1b); // "Reject"
        let last = if three { 2 } else {
            button(bar, 2, 0xa9e3b4, 0x23); // "Delay"
            3
        };
        let item = if same_terms(bid.addr(), answered) == 0 {
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
    let o = Offer(read_usize(slot));
    let player = staff_at((o.target_staff_id() as i32) as isize).addr();
    let club = contract_club_cached(Staff(player));
    set_screen_param(8, exe::key_nation_club_in_season(club, 0usize, 0usize) as usize);
    if o.player_aware() == 0 {
        o.set_player_aware(2);
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
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_TRANSFER_SCREENS_OPEN_BID_SCREEN_FOR_OFFER)(slot, open)
}

/// Comparer of `transfer_screens_build_bid_screen`: side by side over the display list, the human's bid (0x41 bytes)
/// and the contract lookup global; when the bid is still to be created (empty slot) only the original runs (it
/// allocates it).
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_bid_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_SCREENS_BUILD_BID_SCREEN);
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
        ts_delete_offer(slot);
        exe::transfer_screens_open_bid_screen_for_offer((exe::transfer_manager_get_offer_slot(tm, tel26_item_value(item as i16) as i32)) as usize as u32, 0);
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
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRANSFER_SCREENS_HANDLE_BID_SCREEN_ACTION);
    let bid_slot: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(TS_FN_BID_OFFER_SLOT);
    let id = tel26_item_id(item as i16);
    let edit = matches!(id, 3 | 4 | 5 | 6 | 7 | 8 | 9 | 0xa | 0xb | 0xc | 0xd | 0xe | 0xf | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16
        | 0x22 | 0x24 | 0x25 | 0x26 | 0x1e);
    if !edit || read_usize(G_HUMAN_MANAGER) == 0 {
        // simplification: deleting or replacing the bid and opening screens do not run twice
        return o(item);
    }
    let bid = Offer(read_usize(bid_slot(read_usize(G_HUMAN_MANAGER), 0)));
    if bid.is_null() {
        return o(item);
    }
    let staff = staff_at((bid.target_staff_id() as i32) as isize).addr();
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff);
    if c == 0 || read_u8(c + 0x35) & 0x3f == 5 {
        // the bid is deleted (-9)
        return o(item);
    }
    let mut reg = screen_build_regions();
    reg.extend(screen_param_regions());
    reg.push((TS_TEXT, 0x100));
    reg.push((0xae290c, 4));
    reg.push((bid.addr(), 0x41));
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
    let bid = Offer(bid);
    let same_terms: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(TS_FN_HAS_SAME_TERMS);
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let sm = G_SCREEN_MANAGER;
    let total_item = get_screen_param(0x14) as u32;
    let mut total = ts_display(bid.fee() as u32, 4);
    if (bid.appearance_fee() as i32) >= 0 {
        total = total.wrapping_add(ts_display(bid.appearance_fee() as u32, 4));
    }
    if (bid.international_fee() as i32) >= 0 {
        total = total.wrapping_add(ts_display(bid.international_fee() as u32, 4));
    }
    let ex = bid.exchange_players().addr();
    if ex != 0 {
        write_u32(ex + llist::CURSOR, read_u32(ex + llist::HEAD));
        loop {
            let node = exe::llist_get_next(bid.exchange_players().addr());
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
    if same_terms(bid.addr(), read_usize(live_slot)) == 0 {
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
    let o = original_fn::<unsafe extern "C" fn(usize, usize)>(IDX_TRANSFER_SCREENS_REFRESH_BID_TOTAL_AND_BUTTONS);
    compare_by_display_list_with("Transfer bid total and buttons", &[], || o(bid, live_slot), || transfer_screens_refresh_bid_total_and_buttons(bid, live_slot));
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
    original_fn::<unsafe extern "C" fn(usize, u32, usize)>(IDX_TRANSFER_SCREENS_OPEN_BID_CONFIRM_DIALOG)(edited, action, live)
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
/// Then the text area (as many lines as fit), the menu bar (4, 0) and the bar No (id 2) / Yes (id 3).
/// Original: `transfer_screens_build_bid_confirm_dialog` (0x8ef240, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_build_bid_confirm_dialog() {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let mut slot = get_screen_param(0);
    if slot == 0 {
        slot = get_screen_param(2);
    }
    let o = Offer(read_usize(slot));
    let player = staff_at((o.target_staff_id() as i32) as isize).addr();
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
    lang(TS_TEXT, if o.offer_type() == 0 { 0xa9ee00 } else { 0xa9edcc }, n);
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
        match o.completion_timing() & 0xf {
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
        if o.offer_type() == 1 {
            let buyer = club_at((o.buyer_club_id() as i32) as isize).addr();
            lang(TS_TEXT, 0xa9ea60, g(buyer), g(buyer), buyer + club::SHORT_NAME, n);
        } else {
            lang(TS_TEXT, 0xa9ea20, n);
        }
    } else if action == 0x1b {
        let buyer = club_at((o.buyer_club_id() as i32) as isize).addr();
        lang(TS_TEXT, 0xa9e9c0, g(buyer), g(buyer), buyer + club::SHORT_NAME, n);
    }
    ts_dialog_text(lines);
    exe::menubar_build(4, 0);
    let weights: [u8; 2] = [3, 1];
    let bar = ts_button_bar(2, weights.as_ptr());
    // No is 2 and Yes is 3 (0x8ef90c, 0x8ef94b)
    ts_bar_button(bar, 0, TS_NO, 2);
    ts_bar_button(bar, 1, TS_YES, 3);
}

/// Comparer of `transfer_screens_build_bid_confirm_dialog`: side by side over the display list and the contract
/// lookup global; without the slot's manager only the original runs (the assert box does not run twice).
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_bid_confirm_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_SCREENS_BUILD_BID_CONFIRM_DIALOG);
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
        let e = Offer(if edited != 0 { read_usize(edited) } else { read_usize(live) });
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
                exe::transfer_manager_reject_loan_offer(tm, live, aware(e.addr()), 0);
                true
            }
            0x1c => {
                let eo = Offer(read_usize(edited));
                let player = staff_at((eo.target_staff_id() as i32) as isize).addr();
                exe::transfer_manager_add_transfer_negotiation(tm, read_usize(live), eo, aware(eo.addr()));
                if read_u32(read_usize(edited) + offer::FEE) != read_u32(read_usize(live) + offer::FEE) {
                    ts_squad_status_after_bid(player, edited, live, u32::MAX, 0);
                }
                true
            }
            0x1d => {
                if e.offer_type() == 1 {
                    let accept_loan: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(TS_FN_ACCEPT_LOAN_OFFER);
                    accept_loan(tm, live, aware(e.addr()));
                } else {
                    exe::transfer_manager_accept_transfer_bid(tm, live, aware(e.addr()));
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
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRANSFER_SCREENS_HANDLE_BID_CONFIRM_ACTION);
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
    let o = Offer(read_usize(slot));
    let player = staff_at((o.target_staff_id() as i32) as isize).addr();
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
    exe::new_text((t) as usize as *mut usize, TS_TEXT);
    let pale = ts_colour(G_GUI_COLOUR_PALE_GREY);
    draw(t, area, 0x7d, 0x307, &mut row, 3, pale, 0, 0);
    row += 2;
    lang(TS_TEXT, 0xa9ee48); // "Reason:-"
    text_clear(t);
    (exe::text_set_string(t, (TS_TEXT) as usize as *const u8) as i32);
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
        let valid = reason_valid(tm, reason, o.addr()) != 0;
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
        let bid = Offer(read_usize(s));
        if bid.status() == 0 {
            open_bids += 1;
            if reason_valid(tm, chosen as u32, bid.addr()) == 0 {
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
    exe::drop_text((t) as usize as *mut usize);
    llist_drop(l);
}

/// Comparer of `transfer_screens_build_reject_bid_dialog`: side by side over the display list (the bid list and the
/// Text are the frame's own); without the slot's manager only the original runs.
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_reject_bid_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_SCREENS_BUILD_REJECT_BID_DIALOG);
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
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRANSFER_SCREENS_HANDLE_REJECT_BID_ACTION);
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
    let o = Offer(read_usize(slot));
    let player = staff_at((o.target_staff_id() as i32) as isize).addr();
    let buyer = Club(club_at((o.buyer_club_id() as i32) as isize).addr());
    let human = Club(ts_human_club());
    picture(player, 0, 0);
    ts_title_in_club_colours(human.addr());
    let mut name = [0u8; 300];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(player, n, 300, 5);
    let g = (buyer.short_gender_name() as i8) as i32;
    lang(TS_TEXT, 0xa9f17c, g, g, buyer.addr() + club::SHORT_NAME, n);
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
            let hg = human.gender_name() as i32;
            lang(TS_TEXT, board, hg, hg, human.addr() + club::NAME);
        }
    }
    ts_dialog_text(lines);
    exe::menubar_build(4, 0);
    ts_back_button();
}

/// Comparer of `transfer_screens_build_offer_must_accept_dialog`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_offer_must_accept_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_SCREENS_BUILD_OFFER_MUST_ACCEPT_DIALOG);
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
            let bid = Offer(read_usize(exe::transfer_manager_get_offer_slot(tm, read_i32(node))));
            if matches!(bid.status(), 0xb | 0xc | 0xd | 0xe | 0x13) {
                set_screen_param(3, (bid.buyer_club_id() as u32) as usize);
            }
        }
        llist_drop(l);
    }
}

/// Comparer of `transfer_screens_open_cannot_sign_dialog`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_cannot_sign_dialog(player: usize, club: usize, reason: i16, mode: i8) {
    original_fn::<unsafe extern "C" fn(usize, usize, i16, i8)>(IDX_TRANSFER_SCREENS_OPEN_CANNOT_SIGN_DIALOG)(player, club, reason, mode)
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
    let club = Club(get_screen_param(1));
    let reason = get_screen_param(2) as i16;
    let mode = get_screen_param(4) as i8;
    picture(player, 0, 0);
    ts_title_in_club_colours(club.addr());
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
            club.nation().addr()
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
            let deal = Club(club_at(get_screen_param(3) as i32 as isize).addr());
            lang(TS_TEXT, 0xa9f1cc, g(deal.addr()), g(deal.addr()), n, deal.addr() + club::SHORT_NAME);
        }
        0x29 => {
            lang(TS_TEXT, 0xa9f514, g(club.addr()), g(club.addr()), club.addr() + club::SHORT_NAME);
        }
        0x19 | 0x28 => {
            let fmt = match mode {
                0 => 0xa9f308,
                1 => 0xa8f30c,
                2 => 0xa9f29c,
                _ => 0xa9f230,
            };
            exe::nation_get_nationality_adjective(nation_of(reason), adj, 100, 0xd);
            lang(TS_TEXT, fmt, g(club.addr()), g(club.addr()), 100, 100, club.addr() + club::SHORT_NAME, adj);
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
            lang(TS_TEXT, fmt, g(club.addr()), g(club.addr()), 100, 100, club.addr() + club::SHORT_NAME, adj);
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
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_SCREENS_BUILD_CANNOT_SIGN_DIALOG);
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
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRANSFER_SCREENS_HANDLE_BACK_DIALOG_ACTION);
    ts_compare_answer("Back dialog", item, &[1], o, transfer_screens_handle_back_dialog_action)
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
    let o = Offer(read_usize(slot));
    if (o.serial() as u32) == serial && o.status() != 3 {
        open_bid(slot, 1);
    } else {
        ts_open_offer_changed(o.addr(), serial);
    }
}

/// Comparer of `transfer_screens_news_respond_bid_action`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_news_respond_bid_action(id_a: u32, id_b: u32) {
    original_fn::<unsafe extern "C" fn(u32, u32)>(IDX_TRANSFER_SCREENS_NEWS_RESPOND_BID_ACTION)(id_a, id_b)
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
    let o = Offer(read_usize(slot));
    let status = o.status();
    if (o.serial() as u32) == serial && status != 3 && status != 2 {
        open_bid(staff_at((o.target_staff_id() as i32) as isize).addr(), slot);
    } else {
        ts_open_offer_changed(o.addr(), serial);
    }
}

/// Comparer of `transfer_screens_news_respond_counter_offer_action`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_news_respond_counter_offer_action(id_a: u32, id_b: u32) {
    original_fn::<unsafe extern "C" fn(u32, u32)>(IDX_TRANSFER_SCREENS_NEWS_RESPOND_COUNTER_OFFER_ACTION)(id_a, id_b)
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
    let o = Offer(read_usize(slot));
    if (o.serial() as u32) == serial && o.status() != 3 {
        talks(player, slot, 1, 0);
    } else {
        ts_open_offer_changed(o.addr(), serial);
    }
}

/// Comparer of `transfer_screens_news_offer_contract_action`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_news_offer_contract_action(id_a: u32, id_b: u32) {
    original_fn::<unsafe extern "C" fn(u32, u32)>(IDX_TRANSFER_SCREENS_NEWS_OFFER_CONTRACT_ACTION)(id_a, id_b)
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
    original_fn::<unsafe extern "C" fn(u32, u32)>(IDX_TRANSFER_SCREENS_NEWS_RESPOND_DELAY_REQUEST_ACTION)(id_a, id_b)
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
    let o = Offer(read_usize(slot));
    let player = staff_at((o.target_staff_id() as i32) as isize).addr();
    let status = o.status();
    if (o.serial() as u32) == serial && status != 3 && status != 2 {
        open_bid(player, slot);
    } else {
        open_bid(player, 0);
    }
}

/// Comparer of `transfer_screens_news_renegotiate_bid_action`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_news_renegotiate_bid_action(id_a: u32, id_b: u32) {
    original_fn::<unsafe extern "C" fn(u32, u32)>(IDX_TRANSFER_SCREENS_NEWS_RENEGOTIATE_BID_ACTION)(id_a, id_b)
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
    let o = Offer(read_usize(slot));
    if (o.serial() as u32) != serial || o.status() == 3 {
        ts_open_offer_changed(o.addr(), serial);
        return;
    }
    let player = staff_at((o.target_staff_id() as i32) as isize).addr();
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, TS_LOAN_BID_BUILD, TS_LOAN_BID_INPUT, 1, 0usize, 0) == 0 {
        return;
    }
    set_screen_param(0, player);
    let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
    let terms = exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), read_u32(player), o.buyer_club_id() as u32);
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
    exe::transfer_manager_get_loan_period(read_usize(G_TRANSFER_MANAGER), club_at((o.buyer_club_id() as i32) as isize).addr(), contract_club,
        player, &mut first_byte, &mut first_word, &mut second_byte, &mut second_word);
    set_screen_param(3, first_byte as i32 as usize);
    set_screen_param(4, first_word as usize);
    set_screen_param(5, second_byte as i32 as usize);
    set_screen_param(6, second_word as usize);
    set_screen_param(7, o.addr());
    set_screen_param(8, terms);
    set_screen_param(9, (o.serial() as u32) as usize);
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
    original_fn::<unsafe extern "C" fn(u32, u32)>(IDX_TRANSFER_SCREENS_NEWS_RESPOND_LOAN_BID_ACTION)(id_a, id_b)
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
    let human = tel26_slot_manager();
    let weights: [u8; 3] = [2, 1, 1];
    let o = Offer(read_usize(get_screen_param(2)));
    let player = staff_at((o.target_staff_id() as i32) as isize).addr();
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    let seller = Club(if c == 0 || read_u8(c + 0x35) & 0x3f == 5 { 0 } else { club_at(read_i32(c + 4) as isize).addr() });
    let buyer = Club(club_at((o.buyer_club_id() as i32) as isize).addr());
    picture(player, 0, 0);
    ts_title_in_club_colours(human.club_job().addr());
    let mut name = [0u8; 300];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(player, n, 300, 1);
    lang(TS_TEXT, 0xa9f7c4, n); // "Transfer Bid for <player>"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    let lines = ts_dialog_lines();
    let gs = (seller.short_gender_name() as i8) as i32;
    let gb = (buyer.short_gender_name() as i8) as i32;
    lang(TS_TEXT, 0xa9f724, 100, gs, gs, gb, gb, n, seller.addr() + club::NAME, buyer.addr() + club::NAME);
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
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_SCREENS_BUILD_DELAY_REQUEST_DIALOG);
    compare_by_display_list_with("Delay request dialog", &[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(), || transfer_screens_build_delay_request_dialog());
}

/// Answers the delay request dialog (params 0 and 1 the news item ids, 2 the offer slot). Accept (id 3): the news
/// item's slots 7 and 8 are set to 1 (`News::set_item_slot`), -9. Reject (id 2): slot 7 = 1 and slot 8 = 0, then the
/// offer is withdrawn (status 0x11) or rejected, with reason 6, -9. Back (id 1) -9; the rest to the menu bar.
/// Original: `transfer_screens_handle_delay_request_action` (0x8f22c0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_handle_delay_request_action(item: u32) -> i32 {
    let id_a = get_screen_param(0);
    let id_b = get_screen_param(1);
    let id = tel26_item_id(item as i16);
    if id == 3 {
        covered!("tel17_3 delay request Accept");
        exe::news_set_item_slot(read_usize(G_NEWS), id_a, id_b, 7, 1, 0);
        exe::news_set_item_slot(read_usize(G_NEWS), id_a, id_b, 8, 1, 0);
        return -9;
    }
    if id == 2 {
        covered!("tel17_3 delay request Reject");
        exe::news_set_item_slot(read_usize(G_NEWS), id_a, id_b, 7, 1, 0);
        exe::news_set_item_slot(read_usize(G_NEWS), id_a, id_b, 8, 0, 0);
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
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRANSFER_SCREENS_HANDLE_DELAY_REQUEST_ACTION);
    ts_compare_answer("Delay request dialog", item, &[1], o, transfer_screens_handle_delay_request_action)
}

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
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_TRANSFER_SCREENS_OPEN_OFFER_CHANGED_SCREEN)(offer_, serial)
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
    let draw: unsafe extern "thiscall" fn(usize, u32, i32, i32, *mut i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(TS_FN_TEXT_DRAW_WRAPPED);
    let text_area: TsTextAreaLinks = core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    let mut text = [0u32; 8];
    let t = text.as_mut_ptr() as usize;
    exe::new_text((t) as usize as *mut usize, 0);
    let mut row: i32 = 0;
    let o = Offer(get_screen_param(0));
    let serial = get_screen_param(1) as u32;
    let human = ts_human_club();
    let player = staff_at((o.target_staff_id() as i32) as isize).addr();
    picture(player, 0, 0);
    let buyer = Club(club_at((o.buyer_club_id() as i32) as isize).addr());
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    let seller = Club(if c == 0 || read_u8(c + 0x35) & 0x3f == 5 { 0 } else { club_at(read_i32(c + 4) as isize).addr() });
    ts_title_in_club_colours(human);
    let mut name = [0u8; 300];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(player, n, 300, 1);
    lang(TS_TEXT, 0xa9f7c4, n); // "Transfer Bid for <player>"
    (exe::text_set_string(t, (TS_TEXT) as usize as *const u8) as i32);
    exe::text_add_link(t, (n) as usize as *const u8, 0x7d7, player);
    let area = tel26_add_area(current_human_display(), [0x64, 0x50, 0x316, 0x7d, 1, 0, 1, 0, 2, 0, u32::MAX]);
    draw(t, area, 0x64, 0x316, &mut row, 6, ts_colour(G_GUI_COLOUR_YELLOW), 1, 0);
    let lines = ts_dialog_lines();
    let g = |c: usize| read_i8(c + club::SHORT_GENDER_NAME) as i32;
    if (o.serial() as u32) != serial {
        lang(TS_TEXT, 0xa9f94c, g(buyer.addr()), g(buyer.addr()), buyer.addr() + club::SHORT_NAME, n);
    } else {
        match o.status() {
            2 => {
                lang(TS_TEXT, 0xa9f8e0, g(seller.addr()), g(seller.addr()), seller.addr() + club::SHORT_NAME, n);
            }
            5 | 6 => {
                lang(TS_TEXT, 0xa9f804, g(seller.addr()), g(seller.addr()), seller.addr() + club::SHORT_NAME, n);
            }
            3 => {
                lang(TS_TEXT, 0xa9f870, g(buyer.addr()), g(buyer.addr()), buyer.addr() + club::SHORT_NAME, n);
            }
            _ => {}
        }
    }
    text_area(0x6e, 0x8c, 0x30c, 0x221, 2, 0, lines as i8 as i32 as u32, 3, ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT,
        seller.addr() + club::SHORT_NAME, 0x7d5u32, seller.addr(), buyer.addr() + club::SHORT_NAME, 0x7d5u32, buyer.addr(), n, 0x7d7u32, player, 0u32);
    exe::menubar_build(4, 0);
    lang(TS_TEXT, 0xa9f7fc); // "Done"
    tel26_add_button([100, 0x22b, 0x316, 0x24e, 0x30, ts_colour(G_GUI_COLOUR_GREY), 0xc, 3, ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT as u32, 3, 0]);
    exe::drop_text((t) as usize as *mut usize);
}

/// Comparer of `transfer_screens_build_offer_changed_screen`: side by side over the display list and the contract
/// lookup global (the Text is the frame's own).
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_offer_changed_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_SCREENS_BUILD_OFFER_CHANGED_SCREEN);
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
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRANSFER_SCREENS_HANDLE_OFFER_CHANGED_ACTION);
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
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_TRANSFER_SCREENS_OPEN_REMOVE_EXCHANGE_PLAYER_DIALOG)(slot, player)
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
    let text_area: TsTextAreaLinks = core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    let weights: [u8; 2] = [3, 1];
    let o = Offer(read_usize(get_screen_param(0)));
    let exchange = staff_at(get_screen_param(1) as i32 as isize).addr();
    let target = staff_at((o.target_staff_id() as i32) as isize).addr();
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
    let cash: u32 = read_u32(exchange + 0x52).wrapping_add(o.fee() as u32);
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
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_SCREENS_BUILD_REMOVE_EXCHANGE_PLAYER_DIALOG);
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
    let o = Offer(read_usize(slot));
    let buyer = club_at((o.buyer_club_id() as i32) as isize).addr();
    let id_ = get_screen_param(1) as u32;
    let player = staff_at(id_ as i32 as isize).addr();
    o.set_fee(((o.fee() as u32).wrapping_add(read_u32(player + 0x52))) as usize);
    let list = o.exchange_players().addr();
    write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
    loop {
        let node = exe::llist_get_next(o.exchange_players().addr());
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
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRANSFER_SCREENS_HANDLE_REMOVE_EXCHANGE_PLAYER_ACTION);
    ts_compare_answer("Remove exchange player dialog", item, &[1], o, transfer_screens_handle_remove_exchange_player_action)
}

/// Answers the Confirm button of an agreed transfer: slots 6 (the offer slot index) and 7 (its serial then). When the
/// offer is missing or still has the serial, and was not withdrawn, the final confirmation screen opens
/// (`transfer_screens_open_confirm_transfer_screen(slot, id_a, id_b)`); otherwise the offer changed screen. (A missing
/// offer is read as the original does: its status at address 0x2c.)
/// Original: `transfer_screens_news_confirm_transfer_action` (0x8f2d20, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_news_confirm_transfer_action(id_a: u32, id_b: u32) {
    covered!("tel17_4 news confirm transfer");
    let confirm: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_OPEN_CONFIRM_TRANSFER_SCREEN);
    let mut item = [0u8; 0xe0];
    let n = item.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0, 0);
    (exe::news_find_item_by_id(read_usize(G_NEWS), (id_a) as u32 as usize, id_b, n) as usize);
    let index = exe::newsitem_get_slot(n, 6);
    let serial = exe::newsitem_get_slot(n, 7);
    let slot = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), index as i32);
    let o = Offer(read_usize(slot));
    let same = o.is_null() || (o.serial() as u32) == serial;
    if same && o.status() != 3 {
        confirm(slot, id_a, id_b);
    } else if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8f24c0usize, 0x8f2800usize, 1, 0usize, 0) != 0 {
        set_screen_param(0, o.addr());
        set_screen_param(1, serial as usize);
    }
}

/// Comparer of `transfer_screens_news_confirm_transfer_action`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_news_confirm_transfer_action(id_a: u32, id_b: u32) {
    original_fn::<unsafe extern "C" fn(u32, u32)>(IDX_TRANSFER_SCREENS_NEWS_CONFIRM_TRANSFER_ACTION)(id_a, id_b)
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
    original_fn::<unsafe extern "C" fn(usize, u32, u32)>(IDX_TRANSFER_SCREENS_OPEN_CONFIRM_TRANSFER_SCREEN)(slot, id_a, id_b)
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
    let human = Club(ts_human_club());
    let player = Staff(staff_at(get_screen_param(5) as i32 as isize).addr());
    if read_usize(slot) == 0 {
        covered!("tel18_1 confirm transfer: new confirm copy");
        let mem = exe::crt_operator_new(0x41);
        let o = if mem != 0 { new_offer(mem, player.addr(), human.addr(), 0, -1, 0) } else { 0 };
        write_usize(slot, o);
        let src = read_usize(live_slot);
        ts_copy_offer(o, src);
        ts_copy_offer_list(o, read_usize(read_usize(live_slot) + offer::EXCHANGE_PLAYERS));
    }
    let o = read_usize(slot);
    let buyer = Club(club_at(read_i32(o + offer::BUYER_CLUB_ID) as isize).addr());
    let seller = Club(ts_contract_club_not_type_5(player.addr()));
    picture(player.addr(), 0, 0);
    ts_title_in_club_colours(human.addr());
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
        let sell_on = (read_i8(o + offer::SELL_ON_PERCENT) as i32).wrapping_mul(player.value() as i32) / 100;
        let total = sell_on.wrapping_add(read_i32(o + offer::INTERNATIONAL_FEE)).wrapping_add(read_i32(o + offer::APPEARANCE_FEE)).wrapping_add(fee);
        let mut money = [0u8; 0x32];
        let m = money.as_mut_ptr() as usize;
        let we_buy = read_u32(human.addr()) == read_u32(o + offer::BUYER_CLUB_ID);
        let other = Club(if we_buy { seller.addr() } else { buyer.addr() });
        if total > fee {
            exe::cash_format_money(m, 0x32, ts_display(total as u32, 6), 6);
            if count == 0 {
                lang(TS_TEXT, if we_buy { 0xaa06dc } else { 0xaa0668 }, g(other.addr()), g(other.addr()), n, other.addr() + club::SHORT_NAME, m);
            } else {
                lang(TS_TEXT, if we_buy { 0xaa05a8 } else { 0xaa04ec }, g(other.addr()), g(other.addr()), n, other.addr() + club::SHORT_NAME, m, x);
            }
        } else {
            exe::cash_format_money(m, 0x32, ts_display(fee as u32, 6), 6);
            if count == 0 {
                if fee > 0 {
                    lang(TS_TEXT, if we_buy { 0xaa047c } else { 0xaa040c }, g(other.addr()), g(other.addr()), n, other.addr() + club::SHORT_NAME, m);
                } else if !we_buy {
                    lang(TS_TEXT, 0xaa0340, g(buyer.addr()), g(buyer.addr()), n, buyer.addr() + club::SHORT_NAME);
                } else if !seller.is_null() {
                    lang(TS_TEXT, 0xaa03bc, g(seller.addr()), g(seller.addr()), n, seller.addr() + club::SHORT_NAME);
                } else {
                    lang(TS_TEXT, 0xaa0390, n);
                }
            } else if fee > 0 {
                lang(TS_TEXT, if we_buy { 0xaa0294 } else { 0xaa01e8 }, g(other.addr()), g(other.addr()), n, other.addr() + club::SHORT_NAME, m, x);
            } else {
                lang(TS_TEXT, if we_buy { 0xaa014c } else { 0xaa00b4 }, g(other.addr()), g(other.addr()), n, other.addr() + club::SHORT_NAME, x);
            }
        }
    } else {
        covered!("tel18_1 confirm transfer screen: restriction");
        match reason {
            3 => {
                lang(TS_TEXT, 0xaa000c, lg(buyer.addr()), lg(buyer.addr()), 100, buyer.addr() + club::NAME, n);
            }
            4 => {
                lang(TS_TEXT, 0xa9ff74, lg(buyer.addr()), lg(buyer.addr()), buyer.addr() + club::NAME);
            }
            0x22 | 0x23 => {
                lang(TS_TEXT, 0xa9fedc, lg(buyer.addr()), lg(buyer.addr()), buyer.addr() + club::NAME);
            }
            5 => {
                lang(TS_TEXT, 0xa9ff74, lg(seller.addr()), lg(seller.addr()), seller.addr() + club::NAME);
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
                lang(TS_TEXT, fmt, lg(human.addr()), lg(human.addr()), human.addr() + club::NAME);
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
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_SCREENS_BUILD_CONFIRM_TRANSFER_SCREEN);
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
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRANSFER_SCREENS_HANDLE_CONFIRM_TRANSFER_ACTION);
    ts_compare_answer("Confirm transfer screen", item, &[], o, transfer_screens_handle_confirm_transfer_action)
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
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_TRANSFER_SCREENS_OPEN_SHORTLIST_DIALOG)(player_id, add)
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
    original_fn::<unsafe extern "C" fn(usize, u32, u32)>(IDX_TRANSFER_SCREENS_OPEN_COMPARISON_PROMPT)(player_id, unused, which)
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
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_SCREENS_BUILD_COMPARISON_PROMPT);
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
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRANSFER_SCREENS_HANDLE_COMPARISON_PROMPT_ACTION);
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
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_SCREENS_BUILD_SHORTLIST_DIALOG);
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
    let player = Staff(staff_at(get_screen_param(0) as i32 as isize).addr());
    let scout = read_usize(G_SCOUT_MANAGER);
    if get_screen_param(1) == 0 {
        covered!("tel18_1 shortlist remove");
        let remove: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(TS_FN_REMOVE_FROM_SHORTLIST);
        remove(scout, player.addr(), 0);
        return -9;
    }
    covered!("tel18_1 shortlist add");
    exe::scout_manager_add_to_shortlist(scout, player, 0);
    let manager = tel26_slot_manager();
    let mut club = manager.club_job().addr();
    let mut today = [0u32; 2];
    exe::date_date_54a110(today.as_mut_ptr() as usize);
    if club == 0 {
        club = manager.national_job().addr();
        if club == 0 {
            return -9;
        }
    }
    let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    if fin == 0 || exe::database_is_national_team(club) != 0 {
        return -9;
    }
    let reputation = || read_i16(player.player().addr() + player_sheet::CURRENT_REPUTATION) as i32;
    if exe::utils_random(0x25 - reputation() / 300) != 0 {
        return -9;
    }
    let r = exe::utils_random(0x7d0) + 0x1388;
    if reputation() <= r {
        let own = player.club_job().addr();
        if own == 0 || exe::human_manager_is_human_club(own, 0, 1, 0, 0) == 0 {
            return -9;
        }
    }
    let sm = read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856);
    if !(exe::shortlist_manager_staff_interest_in_joining_club(sm, player, club, 0, 0, 1, 0) >= read_f64(rdata::F64_2_75)) {
        return -9;
    }
    covered!("tel18_1 shortlist add: press interest");
    let seller = ts_contract_club_not_type_5(player.addr());
    let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
    // the deal record of the frame: the second draw at +3, the first at +0xb (the other bytes are left as they come)
    let mut deal = [0u8; 0x10];
    let d = deal.as_mut_ptr() as usize;
    let first = exe::utils_random(0x4e20) as i16 as i32;
    write_i32(d + 0xb, first);
    let second = exe::utils_random(0x9c40) as i16 as i32;
    write_i32(d + 3, second);
    let budget = exe::finance_get_transfer_budget(fin, player.addr() as u32, 1, 0usize, 1, 0);
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
    let budget = exe::finance_get_transfer_budget(fin, player.addr() as u32, 1, 0usize, 1, 0);
    if (budget as f64) < needed && (read_i64(fin) as f64) < needed {
        return -9;
    }
    exe::media_add_transfer_interest(read_usize(G_MEDIA), player, club, price, 0.0);
    -9
}

/// Comparer of `transfer_screens_handle_shortlist_dialog`: No side by side; Yes (the shortlist, the draws and the press)
/// and the menu bar only in the original.
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_shortlist_dialog(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRANSFER_SCREENS_HANDLE_SHORTLIST_DIALOG);
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
    original_fn::<unsafe extern "C" fn(usize)>(IDX_TRANSFER_SCREENS_OPEN_WITHDRAW_BID_DIALOG)(slot)
}

/// Builds the withdraw bid dialog: the player's picture (the offer of param 0), the title in the buyer's colours, the
/// subtitle "Withdraw <player> transfer bid?" (name style 5), the text "Are you sure you want to withdraw your transfer
/// bid for <player>?" with his name as a link (0x7d7), the menu bar and the bar No (1) / Yes (2).
/// Original: `transfer_screens_build_withdraw_bid_dialog` (0x8f4cb0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_build_withdraw_bid_dialog() {
    covered!("tel18_1 withdraw bid dialog");
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let text_area: TsTextAreaLinks = core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    let o = Offer(read_usize(get_screen_param(0)));
    let player = staff_at((o.target_staff_id() as i32) as isize).addr();
    let buyer = club_at((o.buyer_club_id() as i32) as isize).addr();
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
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_SCREENS_BUILD_WITHDRAW_BID_DIALOG);
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
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRANSFER_SCREENS_HANDLE_WITHDRAW_BID_DIALOG);
    ts_compare_answer("Withdraw bid dialog", item, &[1], o, transfer_screens_handle_withdraw_bid_dialog)
}

/// Opens the transfers screen (build `transfer_screens_build_transfers_screen`, input
/// `transfer_screens_handle_transfers_screen`, leave `transfer_screens_close_transfers_screen`) with its params: 0 all
/// nations (-1), 1 this month, 2 this year, 3 1 (by month), 4 and 5 no list, 6 page 1, 7 1, 8 and 9 0.
/// Original: `transfer_screens_open_transfers_screen` (0x8f4fa0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_transfers_screen() {
    covered!("tel18_2 open transfers screen");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8f5080usize, 0x8f5f40usize, 0, 0x8f6690usize, 0) == 0 {
        return;
    }
    set_screen_param(0, usize::MAX);
    set_screen_param(1, exe::date_get_month(G_CURRENT_DATE) as i8 as i32 as usize);
    set_screen_param(2, read_u16(G_CURRENT_DATE_YEAR) as usize);
    set_screen_param(3, 1);
    set_screen_param(4, 0);
    set_screen_param(5, 0);
    set_screen_param(6, 1);
    set_screen_param(7, 1);
    set_screen_param(8, 0);
    set_screen_param(9, 0);
}

/// Comparer of `transfer_screens_open_transfers_screen`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_transfers_screen() {
    original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_SCREENS_OPEN_TRANSFERS_SCREEN)()
}

/// Builds the transfers screen: the transfers of a month or a year, all nations or one.
///
/// Params: 0 the nation (-1 all), 1 the month, 2 the year, 3 by month, 4 and 5 the list and its count (built here when
/// missing), 6 the page, 7 the page count (written), 8 the table's scroll row, 9 the table's area (written). The
/// background is the nation's picture (or a random one); the title "Transfers" (dark blue on pale grey), the menu bar,
/// the subtitle "<month> <year>" or "<year>". The period popup (a button 0x69,0x55..0x78,0x69) holds " Month" (id 1,
/// value 1) and " Year" (id 1, value 0), the current one selected; the "<" (2) and ">" (3) buttons (the '*' and '+'
/// glyphs of the symbol font when `G_FONT_MODE` is not -1). The View popup (0x6e,0x91..0xeb,0xa5) holds " All
/// Transfers" (id 4, value -1), a separator and the 35 key nations with a league (`key_nation_sorted_order(1)`,
/// `nation +0x11c`), split in two parts with a header each by `NATION_MENU_SPLIT` (id 4, the nation's id).
///
/// Without a list: `TRANSFER_MANAGER::get_past_transfers` for the month (or the months 0..11 of the year); with a
/// nation, the transfers whose buyer and seller are both of other nations (`club +0x53`) are dropped (the last record
/// moved over them); the rest sorted by fee (qsort); params 4 and 5. With transfers: the page count (rows of 0x15d /
/// font 2 height) in param 7, the page buttons (0x30c,0x91; ids 5..8) and the table 0x6e,0xaa..0x30c,0x217 (5 columns
/// 5:0xa:7:7:4): the date, the player (a link when the record still names him; yellow when either club is human), the
/// seller ("Free Transfer" without one), "to <buyer>" ("Released" without one) and the fee ("<money> + exchange" for an
/// exchange with money, else "Exch", "Loan", "Bos" or "Free"); the area's scroll row from param 8, the area in param 9.
/// Without transfers: "No transfers have taken place.". Last the Back/Next buttons.
/// Original: `transfer_screens_build_transfers_screen` (0x8f5080, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_build_transfers_screen() {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let month_name: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(TS_FN_MONTH_NAME);
    let nation_picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_NATION_PICTURE);
    let past: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, *mut usize, *mut i32) = core::mem::transmute(TS_FN_GET_PAST_TRANSFERS);
    let short_date: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(TS_FN_SHORT_DATE);
    let page_buttons: unsafe extern "C" fn(u32, u32, u32, u32, u32, u32, u32, u32) = core::mem::transmute(TS_FN_PAGE_BUTTONS);
    covered!("tel18_2 transfers screen");
    let weights: [u8; 5] = [5, 0xa, 7, 7, 4];
    let mut group: u8 = 0xff;
    let nation = get_screen_param(0) as i32;
    let month = get_screen_param(1) as u8;
    let year = get_screen_param(2) as u32;
    let by_month = get_screen_param(3) as u32;
    let mut list = get_screen_param(4);
    let mut count = get_screen_param(5) as i32;
    let page = get_screen_param(6) as u32;
    if nation > -1 {
        nation_picture((read_usize(G_NATION_TABLE) as isize + nation as isize * 0x122) as usize, 0, 0);
    } else {
        exe::background_set_random_picture();
    }
    lang(TS_TEXT, 0x98e880); // "Transfers"
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, ts_colour(G_GUI_COLOUR_DARK_BLUE), ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT);
    exe::menubar_build(4, 0);
    if by_month != 0 {
        lang(TS_TEXT, 0x99c890, month_name(month as u32), year & 0xffff);
    } else {
        lang(TS_TEXT, 0xaa0c24, year & 0xffff);
    }
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    let d = current_human_display();
    let period = tel26_add_button([0x69, 0x55, 0x78, 0x69, 0x30, ts_colour(G_GUI_COLOUR_BLACK), 0xc, 1, ts_colour(G_GUI_COLOUR_PALE_GREY), 0, 0, 0]);
    let period_menu = tel26_add_area(d, [0, 0, 0, 0, 0, 0, 0, 0, 0x30, ts_colour(TS_COLOUR_POPUP), period]);
    lang(TS_TEXT, 0xaa0c1c); // " Month"
    ts_menu_line(by_month != 0, 1, 1, period_menu);
    lang(TS_TEXT, 0xaa0c14); // " Year"
    ts_menu_line(by_month == 0, 1, 0, period_menu);
    let symbols = read_i32(G_FONT_MODE) != -1;
    for (glyph, arrow, x1, x2, id) in [(0x2au32, 0x9cdc2cusize, 0x2e7u32, 0x2fbu32, 2u32), (0x2b, 0x9cdc54, 0x2fd, 0x311, 3)] {
        let (font, text) = if symbols {
            format(TS_TEXT, 0x99ec1c, glyph);
            (0, TS_TEXT)
        } else {
            (1, arrow)
        };
        tel26_add_button([x1, 0x55, x2, 0x69, 0x30, ts_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, font, ts_colour(G_GUI_COLOUR_PALE_GREY), text as u32, id, 0]);
    }
    lang(TS_TEXT, 0x988004); // "View"
    let view = tel26_add_button([0x6e, 0x91, 0xeb, 0xa5, 0x30, ts_colour(G_GUI_COLOUR_GREY), 0xc, 1, ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT as u32, 0, 0]);
    let menu = tel26_add_area(d, [0, 0, 0, 0, 0, 0, 0, 0, 0x130, ts_colour(TS_COLOUR_POPUP), view]);
    lang(TS_TEXT, 0xaa0c04); // " All Transfers"
    ts_menu_line(nation == -1, 4, u32::MAX, menu);
    tel26_add_item(d, [1, 0, 0, 0, 0, 0, 0, 0x1000010, ts_colour(TS_COLOUR_POPUP), ts_colour(G_GUI_COLOUR_YELLOW), 1, 1,
        ts_colour(G_GUI_COLOUR_BLACK), 0, 0, 0, 0, menu]);
    let order = exe::key_nation_sorted_order(1);
    let mut split = [0u8; 16];
    exe::new_nation_menu_split((split.as_mut_ptr()) as *mut u8 as usize, 0, order, 1);
    // the original's part area starts as param 3 (the variable's slot is reused); it is set before use when the split
    // has one or two parts
    let mut part_area = by_month;
    for i in 0..0x23usize {
        let n = read_usize((G_LEAGUE_TABLE as isize + read_i8(order + i) as isize * 0x48) as usize);
        if n == 0 || read_u8(n + 0x11c) == 0 {
            continue;
        }
        if (exe::nation_menu_split_part_count((split.as_mut_ptr()) as *mut u8 as usize) as i8) > 1 && exe::nation_menu_split_in_second_part((split.as_mut_ptr()) as *mut u8 as usize, n) != group {
            group = exe::nation_menu_split_in_second_part((split.as_mut_ptr()) as *mut u8 as usize, n);
            exe::nation_menu_split_format_header((split.as_mut_ptr()) as *mut u8 as usize, TS_TEXT, 0x7d0, group as u32);
            let item = ts_menu_line(false, 0, 0, menu);
            part_area = tel26_add_area(d, [0, 0, 0, 0, 0, 0, 0, 0, 0x30, ts_colour(TS_COLOUR_POPUP), item]);
        } else if (exe::nation_menu_split_part_count((split.as_mut_ptr()) as *mut u8 as usize) as i8) == 1 {
            part_area = menu;
        }
        format(TS_TEXT, 0x98ed14, n + 0x38);
        let id = read_u32(n);
        ts_menu_line(id as i32 == nation, 4, id, part_area);
    }
    if list == 0 {
        if by_month != 0 {
            past(read_usize(G_TRANSFER_MANAGER), month as u32, year, month as u32, year, &mut list, &mut count);
        } else {
            past(read_usize(G_TRANSFER_MANAGER), 0, year, 0xb, year, &mut list, &mut count);
        }
        let mut k: i32 = 0;
        while k < count {
            let r = list + k as usize * TS_PAST_TRANSFER_SIZE;
            let club_of = |at: usize| {
                let id = read_i32(r + at);
                if id > -1 { club_at(id as isize).addr() } else { 0 }
            };
            let (buyer, seller) = (club_of(past_transfer::TO_CLUB_ID), club_of(past_transfer::FROM_CLUB_ID));
            let of_nation = |c: usize| c != 0 && read_usize(c + 0x53) != 0 && read_i32(read_usize(c + 0x53)) == nation;
            if nation > -1 && !of_nation(buyer) && !of_nation(seller) {
                count -= 1;
                let last = list + count as usize * TS_PAST_TRANSFER_SIZE;
                for o in (0..0x1c).step_by(4) {
                    write_u32(r + o, read_u32(last + o));
                }
                write_u8(r + 0x1c, read_u8(last + 0x1c));
                let mut tmp = [0u32; 2];
                exe::date_operator_3d(r + past_transfer::DATE, tmp.as_mut_ptr() as usize, last + past_transfer::DATE);
                k -= 1;
            }
            k += 1;
        }
        exe::crt_qsort(list, count as u32, TS_PAST_TRANSFER_SIZE as u32, TS_PAST_TRANSFERS_BY_FEE);
        set_screen_param(4, list);
        set_screen_param(5, count as usize);
    }
    if count > 0 {
        let rows = (0x15d / exe::font_get_height(2)) as u8 as i32;
        let pages = count / rows + (count % rows != 0) as i32;
        set_screen_param(7, pages as i16 as i32 as usize);
        page_buttons(0x30c, 0x91, page, pages as u32, 5, 6, 7, 8);
        let area = tel26_add_area(d, [0x6e, 0xaa, 0x30c, 0x217, 5, weights.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
        let start = (page as i16 as i32 - 1) * rows;
        let mut row: i32 = 0;
        while row < rows && start + row < count {
            let r = list + (start + row) as usize * TS_PAST_TRANSFER_SIZE;
            short_date(r + past_transfer::DATE, TS_TEXT, 0x7d0, 0xff);
            tel26_area_text([area, 0, row as u32, 0x30, ts_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT as u32]);
            let club_of = |at: usize| {
                let id = read_i32(r + at);
                if id > -1 { club_at(id as isize).addr() } else { 0 }
            };
            let buyer = club_of(past_transfer::TO_CLUB_ID);
            let seller = club_of(past_transfer::FROM_CLUB_ID);
            let human = |c: usize| c != 0 && exe::human_manager_is_human_club(c, 0, 1, 0, 0) != 0;
            let colour = if human(buyer) || human(seller) { G_GUI_COLOUR_YELLOW } else { G_GUI_COLOUR_PALE_GREY };
            let colour = ts_colour(colour);
            let mut name = [0u8; 0xc8];
            let (n1, n2, n3) = (read_u32(r + past_transfer::FIRST_NAME_ID), read_u32(r + past_transfer::SECOND_NAME_ID), read_u32(r + past_transfer::COMMON_NAME_ID));
            exe::database_format_name_from_ids(n1, n2, n3, name.as_mut_ptr(), 0xc8, 1, read_u32(r));
            format(TS_TEXT, 0x987ff0, name.as_ptr() as usize);
            let player = staff_at(read_i32(r) as isize).addr();
            if exe::database_staff_matches_names(player, n1, n2, n3, 0usize) != 0 {
                tel26_area_button([area, 1, row as u32, 1, 0, 1, 2, colour, TS_TEXT as u32, 0x7d7, player as u32]);
            } else {
                tel26_area_text([area, 1, row as u32, 1, 0, 1, 2, colour, TS_TEXT as u32]);
            }
            let g = |c: usize| read_i8(c + club::SHORT_GENDER_NAME) as i32;
            if seller != 0 {
                lang(TS_TEXT, 0xaa0bf4, g(seller), g(seller), seller + club::SHORT_NAME);
                tel26_area_button([area, 2, row as u32, 1, 0, 1, 2, ts_colour(G_GUI_COLOUR_YELLOW), TS_TEXT as u32, 0x7d5, seller as u32]);
            } else {
                lang(TS_TEXT, 0x98e604); // "Free Transfer"
                tel26_area_text([area, 2, row as u32, 1, 0, 1, 2, ts_colour(G_GUI_COLOUR_YELLOW), TS_TEXT as u32]);
            }
            if buyer != 0 {
                lang(TS_TEXT, 0xaa0be0, g(buyer), g(buyer), buyer + club::SHORT_NAME);
                tel26_area_button([area, 3, row as u32, 1, 0, 1, 2, ts_colour(JOB_COLOUR_BDD4), TS_TEXT as u32, 0x7d5, buyer as u32]);
            } else {
                lang(TS_TEXT, 0xaa0bd4); // "Released"
                tel26_area_text([area, 3, row as u32, 1, 0, 1, 2, ts_colour(JOB_COLOUR_BDD4), TS_TEXT as u32]);
            }
            let fee = read_u32(r + past_transfer::FEE);
            let kind = read_u8(r + past_transfer::MOVE_TYPE);
            if fee != 0 && kind == 1 {
                let mut money = [0u8; 0xc8];
                exe::cash_format_money(money.as_mut_ptr() as usize, 0xc8, ts_display(fee, 5), 5);
                lang(TS_TEXT, 0x98e628, money.as_ptr() as usize);
            } else if fee != 0 {
                exe::cash_format_money(TS_TEXT, 0x7d0, ts_display(fee, 5), 5);
            } else {
                lang(TS_TEXT, match kind { 1 => 0x98e620, 2 => 0x98e5fc, 4 => 0x98e61c, _ => 0x98e614 });
            }
            tel26_area_text([area, 4, row as u32, 0x30, ts_colour(JOB_COLOUR_3192), 0xc, 1, ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT as u32]);
            row += 1;
        }
        let scroll = get_screen_param(8);
        if area as i16 != -1 {
            exe::area_set_scroll_row(display_area_at(d, area as i16), scroll);
        }
        set_screen_param(9, area as i16 as i32 as usize);
    } else {
        let lines = (0x15d / exe::font_get_height(3)) as u8;
        lang(TS_TEXT, 0xaa0bb4); // "No transfers have taken place."
        tel26_text_area([0x6e, 0xaa, 0x30c, 0x217, 2, 0, lines as u32, 3, ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT as u32, 0]);
    }
    tel26_back_next();
}

/// Comparer of `transfer_screens_build_transfers_screen`: side by side over the display list and the params; only the
/// original while the list is still to be built (it is malloc'd and kept in param 4).
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_transfers_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_SCREENS_BUILD_TRANSFERS_SCREEN);
    if get_screen_param(4) == 0 {
        // simplification: the list allocation does not run twice
        return o();
    }
    compare_by_display_list_with("Transfers screen", &[], || o(), || transfer_screens_build_transfers_screen());
}

/// Answers the transfers screen.
///
/// With transfers listed, any item first keeps the table's scroll row (param 8, from the area of param 9). Month/Year
/// (id 1): param 3 takes the item's value; by month, the month is kept between the game's start and today's month in
/// their years; param 1 the month, the list freed, page 1, -11. "<" (2): by month the previous month (December of the
/// year before after January) unless at the game's start; by year the year before unless it is the start year; ">" (3):
/// the next month or year, up to today's; both free the list, page 1, -11. A nation (id 4, other than the shown one):
/// param 0, the list freed, page 1, -11. The page buttons 5..8: page -10, -1 (at least 1), +1, +10 (at most param 7),
/// -11. Anything else (and the steps that cannot move) goes to the menu bar.
/// Original: `transfer_screens_handle_transfers_screen` (0x8f5f40, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_handle_transfers_screen(item: u32) -> i32 {
    let nation = get_screen_param(0) as u32;
    let mut month = get_screen_param(1) as u8 as i8;
    let mut year = get_screen_param(2) as u32;
    let by_month = get_screen_param(3);
    let count = get_screen_param(5) as i32;
    let page = get_screen_param(6) as i16 as i32;
    let pages = get_screen_param(7) as i16 as i32;
    let it = item as i16;
    if count > 0 && it != -1 {
        let row = exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(9));
        set_screen_param(8, row);
    }
    let id = tel26_item_id(it);
    let start_month = || exe::date_get_month(G_GAME_START_DATE) as i8;
    let today_month = || exe::date_get_month(G_CURRENT_DATE) as i8;
    let start_year = read_u16(G_GAME_START_DATE_YEAR);
    let today_year = read_u16(G_CURRENT_DATE_YEAR);
    let refresh = || {
        transfer_screens_close_transfers_screen();
        set_screen_param(6, 1);
        -11
    };
    if id == 1 {
        covered!("tel18_2 transfers: month or year");
        let value = tel26_item_value(it);
        set_screen_param(3, value as usize);
        if value != 0 {
            if year as u16 == start_year && month <= start_month() {
                month = start_month();
            }
            if year as u16 == today_year && month >= today_month() {
                month = today_month();
            }
        }
        set_screen_param(1, month as i32 as usize);
        return refresh();
    }
    if id == 2 {
        if by_month != 0 {
            if year as u16 > start_year || (year as u16 == start_year && month > start_month()) {
                covered!("tel18_2 transfers: previous month");
                if month == 0 {
                    month = 0xb;
                    year = year.wrapping_add(0xffff);
                } else {
                    month = month.wrapping_sub(1);
                }
                set_screen_param(1, month as i32 as usize);
                set_screen_param(2, (year & 0xffff) as usize);
                return refresh();
            }
        } else if year as u16 > start_year {
            covered!("tel18_2 transfers: previous year");
            set_screen_param(2, ((year & 0xffff) - 1) as usize);
            return refresh();
        }
    }
    if id == 3 {
        if by_month != 0 {
            if (year as u16) < today_year || (year as u16 == today_year && month < today_month()) {
                covered!("tel18_2 transfers: next month");
                if month == 0xb {
                    month = 0;
                    year = year.wrapping_add(1);
                } else {
                    month = month.wrapping_add(1);
                }
                set_screen_param(1, month as i32 as usize);
                set_screen_param(2, (year & 0xffff) as usize);
                return refresh();
            }
        } else if (year as u16) < today_year {
            covered!("tel18_2 transfers: next year");
            set_screen_param(2, ((year & 0xffff) + 1) as usize);
            return refresh();
        }
    }
    if id == 4 && tel26_item_value(it) != nation {
        covered!("tel18_2 transfers: nation");
        set_screen_param(0, tel26_item_value(it) as usize);
        return refresh();
    }
    let new_page = match id {
        5 => Some((page - 10).max(1)),
        6 => Some((page - 1).max(1)),
        7 => Some((page + 1).min(pages)),
        8 => Some((page + 10).min(pages)),
        _ => None,
    };
    if let Some(p) = new_page {
        set_screen_param(6, p as usize);
        return -11;
    }
    ts_menubar_answer(item)
}

/// Comparer of `transfer_screens_handle_transfers_screen`: side by side over the params for the period, nation and page
/// buttons when there is no list to free (else only the original: the list is freed); the menu bar only in the
/// original.
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_transfers_screen(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRANSFER_SCREENS_HANDLE_TRANSFERS_SCREEN);
    let id = tel26_item_id(item as i16);
    if (5..=8).contains(&id) || (matches!(id, 1..=4) && get_screen_param(4) == 0) {
        let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&screen_param_regions(), || o(item), || transfer_screens_handle_transfers_screen(item));
        SCOREBOARD.record(equal && ro == rr, || format!("Transfers screen answer to item {}: original={} rust={} differences at {:?}", item as i16, ro, rr, first_differences(&mo, &mr)));
        return ro;
    }
    o(item)
}

/// Leaves the transfers screen: frees the list of param 4 (crt free) and clears params 4 and 5.
/// Original: `transfer_screens_close_transfers_screen` (0x8f6690, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_close_transfers_screen() {
    let list = get_screen_param(4);
    if list != 0 {
        exe::crt_free(list);
        set_screen_param(4, 0);
        set_screen_param(5, 0);
    }
}

/// Comparer of `transfer_screens_close_transfers_screen`: only the original runs (it frees the list).
pub(crate) unsafe extern "C" fn compare_transfer_screens_close_transfers_screen() {
    original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_SCREENS_CLOSE_TRANSFERS_SCREEN)()
}

/// Answers the Respond button of a transfer status news item: the news item `id_a`/`id_b` (built empty, then
/// `News::find_item_by_id`) gives the player in its slot 2, and his transfer status screen opens
/// (`transfer_screens_open_transfer_status_screen(player, id a, id b, 1)`).
/// Original: `transfer_screens_open_transfer_status_from_news` (0x8f66d0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_transfer_status_from_news(id_a: u32, id_b: u32) {
    covered!("tel18_3 transfer status from news");
    let mut item = [0u8; 0xe0];
    let n = item.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0, 0);
    (exe::news_find_item_by_id(read_usize(G_NEWS), (id_a) as u32 as usize, id_b, n) as usize);
    let player = staff_at(exe::newsitem_get_slot(n, 2) as i32 as isize).addr();
    exe::transfer_screens_open_transfer_status_screen(player, id_a, (id_b) as u32 as i32, 1);
}

/// Comparer of `transfer_screens_open_transfer_status_from_news`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_transfer_status_from_news(id_a: u32, id_b: u32) {
    original_fn::<unsafe extern "C" fn(u32, u32)>(IDX_TRANSFER_SCREENS_OPEN_TRANSFER_STATUS_FROM_NEWS)(id_a, id_b)
}

/// Opens the transfer status screen of a player of the human's club (build
/// `transfer_screens_build_transfer_status_screen`, input `transfer_screens_handle_transfer_status_screen`, flag 1).
///
/// Params: 0 the player's id, 1 his value (`staff::VALUE`) as the asking price, 2 0, 7/8/9 the caller's three values
/// (a news item's ids and a flag), 0xb the slot manager's club, 0xc 1 when that club's finances have the flag +0x165
/// set. From his contract with the club (`get_contract_for_club`): 3 the squad status (high nibble of
/// `contract::SQUAD_STATUS`), 4 transfer listed, 5 loan listed (`staff_is_transfer_listed`/`staff_is_loan_listed`),
/// 6 and 0xd listed by request (bit 3 of `contract::TRANSFER_STATUS`), 0xe grievances (`contract::GRIEVANCES` set),
/// 0xa automatic asking price (bit 2); without a contract 3 = 7 and the rest 0. 0xf the player's value for the club
/// (`calc_player_value(player, club, 0, -1, 1)`) when he is a player, else 0.
/// Original: `transfer_screens_open_transfer_status_screen` (0x8f6770, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_transfer_status_screen(player: usize, a: u32, b: u32, c: u32) {
    let player = Staff(player);
    covered!("tel18_3 open transfer status screen");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8f6a10usize, 0x8f7640usize, 1, 0usize, 0) == 0 {
        return;
    }
    let contract = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, read_usize(tel26_slot_manager().addr() + staff::CLUB_JOB)));
    set_screen_param(0, read_u32(player.addr()) as usize);
    set_screen_param(1, (player.value() as u32) as usize);
    set_screen_param(2, 0);
    set_screen_param(7, a as usize);
    set_screen_param(8, b as usize);
    set_screen_param(9, c as usize);
    let club = read_usize(tel26_slot_manager().addr() + staff::CLUB_JOB);
    set_screen_param(0xb, club);
    let flagged = club != 0 && read_u8(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club) + 0x165) == 1;
    set_screen_param(0xc, flagged as usize);
    if !contract.is_null() {
        let status = contract.transfer_status();
        let by_request = ((status & TS_STATUS_BY_REQUEST) >> 3) as usize;
        let grievances = contract.grievances();
        let listed = exe::staff_is_transfer_listed(player, 0usize);
        let loan = exe::staff_is_loan_listed(player, 0usize);
        set_screen_param(4, listed as usize);
        set_screen_param(5, loan as usize);
        set_screen_param(6, by_request);
        set_screen_param(0xd, by_request);
        set_screen_param(0xe, (grievances != 0) as usize);
        set_screen_param(3, (contract.squad_status() >> 4) as usize);
        set_screen_param(0xa, ((contract.transfer_status() & TS_STATUS_AUTO_PRICE) >> 2) as usize);
    } else {
        set_screen_param(3, 7);
        for k in [4u32, 5, 6, 0xd, 0xe, 0xa] {
            set_screen_param(k, 0);
        }
    }
    if player.player().addr() != 0 {
        let club = read_usize(tel26_slot_manager().addr() + staff::CLUB_JOB);
        set_screen_param(0xf, exe::contract_manager_calc_player_value(read_usize(G_CONTRACT_MANAGER), player, club, 0, -1, 1) as u32 as usize);
    } else {
        set_screen_param(0xf, 0);
    }
}

/// Comparer of `transfer_screens_open_transfer_status_screen`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_transfer_status_screen(player: usize, a: u32, b: u32, c: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32, u32, u32)>(IDX_TRANSFER_SCREENS_OPEN_TRANSFER_STATUS_SCREEN)(player, a, b, c)
}

/// Builds the transfer status screen ("Set Staff Transfer Status").
///
/// Params as written by the opener (1 the asking price, 3 the squad status, 8 the news item's second id, 0xa the
/// automatic price, 0xb the club, 0xc the finance flag, 0xf the value cap); param 2 gets the "-" item. Without the
/// player's contract with the club: the assert box (line 0x1d15). The contract type (low 6 bits of
/// `contract::CONTRACT_TYPE`, signed) picks the layout. The title is the player's name in the club's kit colours.
///
/// The table (0x6e,0x8c..0x30c,0x221, 7 columns 2:0x14:2:0xa:2:4:3): unless the type is 6, the wrapped text (out of
/// contract for type 3, month to month for type 1, else "<player> is valued at <money>" with the money as link 1) with
/// the player's name as a link, then "Asking Price" with "-" (id 2), the price (a popup of the ten fee steps, id 0xb,
/// unless the finance flag) and "+" (id 3, up to 30,000,000 and, with the flag, below the value cap), greyed for the
/// types 3 and 1 or at the bounds, and "Auto" (id 5, value 1 when not automatic). Then "Squad Status" and, in a second
/// table of 2 columns 2:0x11, the seven squad statuses (`squad_status_text`; greyed when not allowed for him; the
/// current one yellow; id 7, the status as value). Unless the type is 6: "Transfer Status" and the rows "This player is
/// transfer listed" (id 9), "This player is available for loan" (id 8; not for the job 0xf) and "This player is
/// transfer listed by request" (id 0xa), yellow when set and greyed when he asked to leave and has grievances. The menu
/// bar and the bar Back (id 4) / Set (id 6, greyed without a squad status while a news item is attached); the price
/// popup's ten items.
/// Original: `transfer_screens_build_transfer_status_screen` (0x8f6a10, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_build_transfer_status_screen() {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let draw: unsafe extern "thiscall" fn(usize, u32, i32, i32, *mut i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(TS_FN_TEXT_DRAW_WRAPPED);
    let status_text: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(TS_FN_SQUAD_STATUS_TEXT);
    let allowed: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(TS_FN_SQUAD_STATUS_ALLOWED);
    let from_table: unsafe extern "thiscall" fn(*mut u32, u32, *const u32, u32) = core::mem::transmute(TS_FN_CASH_FROM_TABLE);
    let weights7: [u8; 7] = [2, 0x14, 2, 0xa, 2, 4, 3];
    let weights4: [u8; 4] = [3, 1, 2, 0x11];
    let mut text = [0u32; 8];
    let t = text.as_mut_ptr() as usize;
    exe::new_text((t) as usize as *mut usize, 0);
    let mut row: i32;
    let player = Staff(staff_at(get_screen_param(0) as i32 as isize).addr());
    let asking = get_screen_param(1) as i32;
    let squad = get_screen_param(3) as u8;
    let news_b = get_screen_param(8) as i32;
    let auto = get_screen_param(0xa);
    let club = get_screen_param(0xb);
    get_screen_param(0xc);
    let flagged = get_screen_param(0xc);
    let cap = get_screen_param(0xf) as i32;
    let contract = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, club));
    if contract.is_null() {
        covered!("tel18_3 transfer status without a contract");
        tel26_error_box(TS_CPP, 0x1d15);
        exe::drop_text((t) as usize as *mut usize);
        return;
    }
    covered!("tel18_3 transfer status screen");
    let kind = ((contract.contract_type() << 2) as i8) >> 2;
    picture(player.addr(), 0, 0);
    let (mut a, mut b) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut a, &mut b);
    let mut name = [0u8; 0x1f4];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(player, n, 0x1f4, 1);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, b, a, n);
    lang(TS_TEXT, 0xaa0dec); // "Set Staff Transfer Status"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    let lines = (0x195 / exe::font_get_height(3)) as u8;
    let d = current_human_display();
    let area = tel26_add_area(d, [0x6e, 0x8c, 0x30c, 0x221, 7, weights7.as_ptr() as u32, lines as u32, 0, 2, 0, u32::MAX]);
    let pale = ts_colour(G_GUI_COLOUR_PALE_GREY);
    let grey = ts_colour(G_GUI_COLOUR_GREY);
    let dark = ts_colour(G_GUI_COLOUR_DARK_BLUE);
    let black = ts_colour(G_GUI_COLOUR_BLACK);
    let yellow = ts_colour(G_GUI_COLOUR_YELLOW);
    let mut popup: u32 = u32::MAX;
    if kind != 6 {
        let fixed = kind == 3 || kind == 1;
        if kind == 3 {
            lang(TS_TEXT, 0xaa0da4, n);
        } else if kind == 1 {
            lang(TS_TEXT, 0xaa0d50, n);
        } else {
            let mut money = [0u8; 0x1e];
            let m = money.as_mut_ptr() as usize;
            exe::cash_format_money(m, 0x1e, ts_display(player.value() as u32, 6), 6);
            lang(TS_TEXT, 0xaa0cec, n, m);
            exe::text_add_link(t, (m) as usize as *const u8, 1, 0);
        }
        (exe::text_set_string(t, (TS_TEXT) as usize as *const u8) as i32);
        exe::text_add_link(t, (n) as usize as *const u8, 0x7d7, player);
        row = 0;
        draw(t, area, 0x6e, 0x30c, &mut row, 3, pale, 0, 0);
        row += 2;
        lang(TS_TEXT, 0xaa0cdc); // "Asking Price"
        tel26_area_text([area, 1, row as u32, 1, 0, 1, 3, pale, TS_TEXT as u32]);
        let minus = if asking > 0 && !fixed {
            tel26_area_button([area, 2, row as u32, 0x30, dark, 0xc, 3, pale, TS_MINUS as u32, 2, 0])
        } else {
            tel26_area_text([area, 2, row as u32, 0x30, dark, 0xc, 3, grey, TS_MINUS as u32])
        };
        set_screen_param(2, minus as i16 as i32 as usize);
        if !fixed {
            exe::cash_format_money(TS_TEXT, 0x7d0, ts_display(asking as u32, 5), 5);
            let item = tel26_area_button([area, 3, row as u32, 1, black, 0xc, 3, pale, TS_TEXT as u32, 0, 0]);
            if flagged == 0 {
                popup = tel26_add_area(d, [0, 0, 0, 0, 0, 0, 0, 0, 0x130, ts_colour(TS_COLOUR_POPUP), item as i16 as i32 as u32]);
            }
        } else {
            tel26_area_text([area, 3, row as u32, 1, black, 0xc, 3, grey, 0x99cc54]); // "0"
        }
        if asking < TS_ASKING_PRICE_CAP && !fixed && (flagged == 0 || asking < cap) {
            tel26_area_button([area, 4, row as u32, 0x30, dark, 0xc, 3, pale, TS_PLUS as u32, 3, 0]);
        } else {
            tel26_area_text([area, 4, row as u32, 0x30, dark, 0xc, 3, grey, TS_PLUS as u32]);
        }
        lang(TS_TEXT, 0xaa0cb0, 0xad9c64usize); // "Auto"
        if auto != 0 {
            tel26_area_button([area, 5, row as u32, 0x70, dark, 0xc, 2, pale, TS_TEXT as u32, 5, 0]);
        } else {
            tel26_area_button([area, 5, row as u32, 0x30, dark, 0xc, 2, pale, TS_TEXT as u32, 5, 1]);
        }
        row += 2;
    } else {
        row = 0;
    }
    lang(TS_TEXT, 0x9afe1c); // "Squad Status"
    tel26_area_button([area, 1, row as u32, 1, 0, 1, 3, pale, TS_TEXT as u32, 0, 0]);
    row += 2;
    let statuses = tel26_add_area(d, [0x6e, 0x8c, 0x30c, 0x221, 2, weights4.as_ptr() as u32 + 2, lines as u32, 0, 1, 0, u32::MAX]);
    let sm = ts_shortlist_manager();
    for k in 1..8u32 {
        status_text(ts_shortlist_manager(), k, TS_TEXT, 0x7d0);
        if allowed(sm, player.addr(), k) == 0 {
            tel26_area_text([statuses, 1, row as u32, 1, 0, 1, 3, grey, TS_TEXT as u32]);
        } else {
            let colour = if squad as i8 as i16 == k as i16 { yellow } else { pale };
            tel26_area_button([statuses, 1, row as u32, 1, 0, 1, 3, colour, TS_TEXT as u32, 7, k]);
        }
        row += 1;
    }
    row += 1;
    if kind != 6 {
        let by_request = get_screen_param(6);
        let grievances = get_screen_param(0xe);
        let listed = get_screen_param(4);
        let loan = get_screen_param(5);
        let request = get_screen_param(0xd);
        lang(TS_TEXT, 0xaa0ca0); // "Transfer Status"
        tel26_area_button([area, 1, row as u32, 1, 0, 1, 3, pale, TS_TEXT as u32, 0, 0]);
        row += 1;
        let locked = by_request != 0 && grievances != 0;
        lang(TS_TEXT, 0xaa0c80); // "This player is transfer listed"
        if locked {
            tel26_area_text([statuses, 1, row as u32, 1, 0, 1, 3, grey, TS_TEXT as u32]);
        } else {
            tel26_area_button([statuses, 1, row as u32, 1, 0, 1, 3, if listed != 0 { yellow } else { pale }, TS_TEXT as u32, 9, 0]);
        }
        row += 1;
        if player.job_for_club() != 0xf {
            lang(TS_TEXT, 0xaa0c5c); // "This player is available for loan"
            if locked {
                tel26_area_text([statuses, 1, row as u32, 1, 0, 1, 3, grey, TS_TEXT as u32]);
            } else {
                tel26_area_button([statuses, 1, row as u32, 1, 0, 1, 3, if loan != 0 { yellow } else { pale }, TS_TEXT as u32, 8, 0]);
            }
            row += 1;
        }
        lang(TS_TEXT, 0xaa0c30); // "This player is transfer listed by request"
        if request == 0 {
            if by_request != 0 {
                tel26_area_button([statuses, 1, row as u32, 1, 0, 1, 3, pale, TS_TEXT as u32, 0xa, 0]);
            } else {
                tel26_area_text([statuses, 1, row as u32, 1, 0, 1, 3, grey, TS_TEXT as u32]);
            }
        } else if grievances == 0 {
            tel26_area_button([statuses, 1, row as u32, 1, 0, 1, 3, yellow, TS_TEXT as u32, 0xa, 0]);
        } else {
            tel26_area_text([statuses, 1, row as u32, 1, 0, 1, 3, yellow, TS_TEXT as u32]);
        }
    }
    exe::menubar_build(4, 0);
    let bar = tel26_add_area(d, [100, 0x22b, 0x316, 0x24e, 2, weights4.as_ptr() as u32, 1, 0, 1, b, u32::MAX]);
    lang(TS_TEXT, TS_BACK);
    tel26_area_button([bar, 0, 0, 0x30, grey, 0xc, 3, pale, TS_TEXT as u32, 4, 0]);
    lang(TS_TEXT, 0x9afcc0, 0xad9c64usize); // "Set"
    if squad == 0 && news_b != -1 {
        tel26_area_text([bar, 1, 0, 0x30, grey, 0x2c, 3, pale, TS_TEXT as u32]);
    } else {
        tel26_area_button([bar, 1, 0, 0x30, grey, 0xc, 3, pale, TS_TEXT as u32, 6, 0]);
    }
    if popup as i16 != -1 {
        let mut cash: u32 = 0;
        for k in 0..10u32 {
            from_table(&mut cash, k, TS_FEE_STEPS.as_ptr(), 10);
            exe::cash_format_money(TS_TEXT, 0x7d0, ts_display(cash, 5), 5);
            tel26_add_item(d, [2, 0, 0, 0, 0, 0, 0, 0x10, ts_colour(TS_COLOUR_POPUP), yellow, 1, 1, black, TS_TEXT as u32, 0, 0xb, cash, popup]);
        }
    }
    exe::drop_text((t) as usize as *mut usize);
}

/// Comparer of `transfer_screens_build_transfer_status_screen`: side by side over the display list and the params
/// (the Text is the frame's own); without the contract only the original runs (the assert box).
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_transfer_status_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_SCREENS_BUILD_TRANSFER_STATUS_SCREEN);
    let player = staff_at(get_screen_param(0) as i32 as isize).addr();
    if exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, get_screen_param(0xb)) == 0 {
        return o();
    }
    compare_by_display_list_with("Transfer status screen", &[], || o(), || transfer_screens_build_transfer_status_screen());
}

/// Answers the transfer status screen.
///
/// On a network game with the pending command argument 3: it is cleared, -9. Auto (id 5): param 0xa flips and, when
/// it becomes set, the asking price (param 1) is the player's value for his contract club; -11. A price from the popup
/// (id 0xb): not automatic, param 1 the item's value, -11. A squad status (id 7): param 3, -11. Transfer listed (id 9):
/// param 4 flips, 5 and 0xd cleared; loan (id 8): param 5 flips, 4 and 0xd cleared; by request (id 0xa): param 0xd
/// flips, 4 and 5 cleared; -11 each. "-" (2) and "+" (3): the price steps down or up (`Cash::step_down`/`step_up`, 4),
/// not automatic, and the widgets change in place (`ts_asking_price_widgets`; "+" up to 30,000,000 and, stepping up
/// with the finance flag, below the value cap), 0. The asking price (id 1) back to the player's value, -11. Back (4)
/// -9. Set (6): with a news item (params 7 and 8 >= 0) its slots 6 = 1, 7 = the squad status, 9 = the old transfer
/// status; the contract's squad status kept (the original rewrites it unchanged); `set_player_squad_status(player,
/// club, price, &done, squad status, old transfer status, 0)`, news slot 0xa = done; unless the contract type is 6, the
/// transfer and loan lists follow params 4 and 5 (`transfer_list_player`, `remove_from_transfer_list`,
/// `add_to_loan_list`, `remove_from_loan_list`), and the contract's bits: by request (8, clearing bit 2) from param 0xd
/// and automatic price (4) from param 0xa; -9. The rest goes to the menu bar.
/// Original: `transfer_screens_handle_transfer_status_screen` (0x8f7640, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_handle_transfer_status_screen(item: u32) -> i32 {
    let down: unsafe extern "thiscall" fn(*mut u32, u32) -> i32 = core::mem::transmute(TS_FN_CASH_STEP_DOWN);
    let asking = get_screen_param(1) as i32;
    let squad = get_screen_param(3) as u8;
    let auto = get_screen_param(0xa);
    let flagged = get_screen_param(0xc);
    let cap = get_screen_param(0xf) as i32;
    if read_u32(TS_NETWORK_GAME) != 0 && read_u32(G_PENDING_COMMAND_ARG) == 3 {
        write_u32(G_PENDING_COMMAND_ARG, 0);
        return -9;
    }
    let it = item as i16;
    let id = tel26_item_id(it);
    let player = || staff_at(get_screen_param(0) as i32 as isize).addr();
    match id {
        5 => {
            covered!("tel18_3 transfer status Auto");
            let p = player();
            let on = (auto == 0) as usize;
            set_screen_param(0xa, on);
            if on != 0 {
                let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
                write_usize(G_CONTRACT_LOOKUP_RESULT, c);
                let club = if c != 0 { club_at(read_i32(c + 4) as isize).addr() } else { 0 };
                set_screen_param(1, exe::contract_manager_calc_player_value(read_usize(G_CONTRACT_MANAGER), p, club, 0, -1, 1) as u32 as usize);
            }
            -11
        }
        0xb => {
            set_screen_param(0xa, 0);
            set_screen_param(1, tel26_item_value(it) as usize);
            -11
        }
        7 => {
            set_screen_param(3, tel26_item_value(it) as usize);
            -11
        }
        9 => {
            set_screen_param(4, (get_screen_param(4) == 0) as usize);
            set_screen_param(5, 0);
            set_screen_param(0xd, 0);
            -11
        }
        8 => {
            let on = (get_screen_param(5) == 0) as usize;
            set_screen_param(4, 0);
            set_screen_param(5, on);
            set_screen_param(0xd, 0);
            -11
        }
        0xa => {
            let on = (get_screen_param(0xd) == 0) as usize;
            set_screen_param(4, 0);
            set_screen_param(5, 0);
            set_screen_param(0xd, on);
            -11
        }
        2 => {
            covered!("tel18_3 transfer status minus");
            let mut cash = asking as u32;
            down(&mut cash, 4);
            let price = cash as i32;
            set_screen_param(1, price as usize);
            set_screen_param(0xa, 0);
            ts_asking_price_widgets(price, price < TS_ASKING_PRICE_CAP, |w| exe::screenmanager_set_widget_data(G_SCREEN_MANAGER, w, 0usize));
            0
        }
        3 => {
            covered!("tel18_3 transfer status plus");
            let mut cash = asking as u32;
            exe::cash_step_up(&mut cash as *mut u32 as usize, 4);
            let price = cash as i32;
            set_screen_param(1, price as usize);
            set_screen_param(0xa, 0);
            let on = price < TS_ASKING_PRICE_CAP && (flagged == 0 || price < cap);
            ts_asking_price_widgets(price, on, |w| exe::screenmanager_set_widget_id(G_SCREEN_MANAGER, w, 0));
            0
        }
        1 => {
            set_screen_param(1, read_u32(player() + staff::VALUE) as usize);
            -11
        }
        4 => -9,
        6 => {
            covered!("tel18_3 transfer status Set");
            let p = player();
            let news_a = get_screen_param(7);
            let club = get_screen_param(0xb);
            let contract = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club));
            let old_squad = contract.squad_status() >> 4;
            let news_b = get_screen_param(8);
            let old_status = contract.transfer_status();
            let mut done: u32 = 0;
            let with_news = news_a != 0 && news_b as i32 > -1;
            if with_news {
                ts_news_slot(news_a, news_b, 6, 1);
                ts_news_slot(news_a, news_b, 7, squad as i8 as i32 as u32);
                ts_news_slot(news_a, news_b, 9, old_status as i8 as i32 as u32);
            }
            contract.set_squad_status((contract.squad_status() & 0xf) | (old_squad << 4));
            let set: unsafe extern "thiscall" fn(usize, usize, usize, i32, *mut u32, u32, u32, u32) = core::mem::transmute(TS_FN_SET_PLAYER_SQUAD_STATUS);
            set(read_usize(G_TRANSFER_MANAGER), p, club, asking, &mut done, squad as u32, old_status as u32, 0);
            if with_news {
                ts_news_slot(news_a, news_b, 0xa, done);
            }
            if contract.contract_type() & 0x3f == 6 {
                return -9;
            }
            let listed = get_screen_param(4);
            let loan = get_screen_param(5);
            let request = get_screen_param(0xd);
            let sm = ts_shortlist_manager();
            if listed != 0 {
                let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(TS_FN_TRANSFER_LIST_PLAYER);
                f(sm, p, 1);
            } else if exe::staff_is_transfer_listed(p, 0usize) != 0 {
                let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(TS_FN_REMOVE_FROM_TRANSFER_LIST);
                f(ts_shortlist_manager(), p);
            }
            if loan != 0 {
                let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(TS_FN_ADD_TO_LOAN_LIST);
                f(ts_shortlist_manager(), p);
            } else if exe::staff_is_loan_listed(p, 0usize) != 0 {
                let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(TS_FN_REMOVE_FROM_LOAN_LIST);
                f(ts_shortlist_manager(), p);
            }
            let at = contract.addr() + contract::TRANSFER_STATUS;
            if request != 0 {
                if read_u8(at) & TS_STATUS_BIT_2 != 0 {
                    write_u8(at, read_u8(at) & !TS_STATUS_BIT_2);
                }
                write_u8(at, read_u8(at) | TS_STATUS_BY_REQUEST);
            } else {
                write_u8(at, read_u8(at) & !TS_STATUS_BY_REQUEST);
            }
            if auto != 0 {
                write_u8(at, read_u8(at) | TS_STATUS_AUTO_PRICE);
            } else {
                write_u8(at, read_u8(at) & !TS_STATUS_AUTO_PRICE);
            }
            -9
        }
        _ => ts_menubar_answer(item),
    }
}

/// Comparer of `transfer_screens_handle_transfer_status_screen`: the param edits (Auto, the popup price, a squad
/// status, the three list toggles, the value) and Back side by side over the params and the contract lookup global;
/// "-"/"+" (they change the widgets and redraw), Set (lists, contract, news) and the menu bar only in the original.
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_transfer_status_screen(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRANSFER_SCREENS_HANDLE_TRANSFER_STATUS_SCREEN);
    let id = tel26_item_id(item as i16);
    let network = read_u32(TS_NETWORK_GAME) != 0 && read_u32(G_PENDING_COMMAND_ARG) == 3;
    if network || !matches!(id, 1 | 4 | 5 | 7 | 8 | 9 | 0xa | 0xb) {
        return o(item);
    }
    let mut reg = screen_param_regions();
    reg.push((G_CONTRACT_LOOKUP_RESULT, 4));
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&reg, || o(item), || transfer_screens_handle_transfer_status_screen(item));
    SCOREBOARD.record(equal && ro == rr, || format!("Transfer status screen answer to item {}: original={} rust={} differences at {:?}", item as i16, ro, rr, first_differences(&mo, &mr)));
    ro
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
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_TRANSFER_SCREENS_OPEN_TRANSFER_LIST_DIALOG)(player, loan)
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
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_SCREENS_BUILD_TRANSFER_LIST_DIALOG);
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
    let player = Staff(get_screen_param(0));
    let sm = || read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856);
    if get_screen_param(1) != 0 {
        covered!("tel18_4 loan list Yes");
        if exe::staff_is_loan_listed(player, 0usize) != 0 {
            let f: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_REMOVE_FROM_LOAN_LIST);
            f(sm(), player.addr());
        } else {
            let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_ADD_TO_LOAN_LIST);
            f(sm(), player.addr());
        }
        return -10;
    }
    if exe::staff_is_transfer_listed(player, 0usize) == 0 {
        covered!("tel18_4 transfer list Yes: listed");
        let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_TRANSFER_LIST_PLAYER);
        f(sm(), player.addr(), 1);
        return -10;
    }
    covered!("tel18_4 transfer list Yes: removed");
    let f: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_REMOVE_FROM_TRANSFER_LIST);
    f(sm(), player.addr());
    let c = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, player.club_job().addr()));
    if c.grievances() & 0x10000 == 0 && exe::staff_contract_wants_to_leave(c) != 0 && player.loyalty() < 0xf {
        let morale = c.morale() as u8;
        let x = morale as i8 as i32 - 0x32;
        let v = if x > 100 { 0x64 } else if x < -100 { 0x9c } else { morale.wrapping_sub(0x32) };
        c.set_morale((v) as i8);
        exe::staff_contract_apply_event_flags(c, 0x10000, 0);
    }
    -10
}

/// Comparer of `transfer_screens_handle_transfer_list_dialog`: No side by side; Yes (the lists and the contract) and the
/// menu bar only in the original.
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_transfer_list_dialog(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRANSFER_SCREENS_HANDLE_TRANSFER_LIST_DIALOG);
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
    original_fn::<unsafe extern "C" fn(usize)>(IDX_TRANSFER_SCREENS_OPEN_RELEASE_DIALOG)(player)
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
    let text_area: TsTextAreaLinks = core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
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
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_SCREENS_BUILD_RELEASE_DIALOG);
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
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRANSFER_SCREENS_HANDLE_RELEASE_DIALOG);
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
    original_fn::<unsafe extern "C" fn(usize)>(IDX_TRANSFER_SCREENS_OPEN_OFFER_TRIAL_DIALOG)(player)
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
    let manager = tel26_slot_manager();
    if manager.is_null() {
        tel26_error_box(TS_CPP, 0x21b4);
        return;
    }
    let club = manager.club_job();
    if club.is_null() {
        tel26_error_box(TS_CPP, 0x21bb);
        return;
    }
    covered!("tel18_4 offer trial dialog");
    picture(player, 0, 0);
    let (mut a, mut b) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut a, &mut b);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, b, a, club.addr() + club::SHORT_NAME);
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
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_SCREENS_BUILD_OFFER_TRIAL_DIALOG);
    let manager = tel26_slot_manager();
    if get_screen_param(0) == 0 || manager.is_null() || manager.club_job().addr() == 0 {
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
        let player = get_screen_param(0);
        let weeks = get_screen_param(1) as u8;
        exe::transfer_screens_open_confirm_trial_offer(player, (item & !0xff) | weeks as u32);
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
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRANSFER_SCREENS_HANDLE_OFFER_TRIAL_DIALOG);
    ts_compare_answer("Offer trial dialog", item, &[3], o, transfer_screens_handle_offer_trial_dialog)
}

/// Answers the Respond button of a loan offer negotiated in the news: the news item's slots 6 (the offer slot index)
/// and 7 (the offer's serial then). When the offer still has that serial and is neither withdrawn (3) nor rejected
/// (2), the loan bid screen opens for its target (`transfer_screens_open_loan_bid_screen`); refused, the offer is
/// withdrawn (`withdraw_transfer_offer(slot, 0, 0)`). Opened, the negotiated loan contract of the target with the buyer
/// (`TRANSFER_MANAGER::find_negotiated_loan_contract`; none: the transfer_screens.cpp assert box, line 0x22d5) fills
/// the params: 7 the offer, 0xa the loan contract, 0xb..0xf and again 0x15..0x19 its terms (the wage share over the
/// player's contract, `contract_manager_get_contract`, not stored in the lookup global). Otherwise the offer changed
/// screen (params 0 the offer, 1 the serial).
/// Original: `transfer_screens_open_loan_bid_from_news` (0x8f93b0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_loan_bid_from_news(id_a: u32, id_b: u32) {
    let mut item = [0u8; 0xe0];
    ts_find_news_item(&mut item, id_a, id_b);
    let n = item.as_mut_ptr() as usize;
    let index = exe::newsitem_get_slot(n, 6);
    let serial = exe::newsitem_get_slot(n, 7);
    let slot = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), index as i32);
    let o = Offer(read_usize(slot));
    let status = o.status();
    if (o.serial() as u32) != serial || status == 3 || status == 2 {
        covered!("tel18_5 loan bid from news: offer changed");
        ts_open_offer_changed(o.addr(), serial);
        return;
    }
    let player = staff_at((o.target_staff_id() as i32) as isize).addr();
    if transfer_screens_open_loan_bid_screen(player) == 0 {
        covered!("tel18_5 loan bid from news: refused");
        exe::transfer_manager_withdraw_transfer_offer(read_usize(G_TRANSFER_MANAGER), slot as *mut usize, 0, 0);
        return;
    }
    let buyer = club_at(read_i32(read_usize(slot) + offer::BUYER_CLUB_ID) as isize).addr();
    let loan = find_negotiated_loan_contract(read_usize(G_TRANSFER_MANAGER), player, buyer);
    if loan == 0 {
        covered!("tel18_5 loan bid from news without loan contract");
        tel26_error_box(TS_CPP, 0x22d5);
        return;
    }
    covered!("tel18_5 loan bid from news");
    set_screen_param(7, read_usize(slot));
    set_screen_param(0xa, loan);
    let contract = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player));
    let share = ts_loan_wage_share(read_i32(loan + TS_WAGE), contract.wage());
    ts_set_loan_terms(loan, share, 0xb);
    ts_set_loan_terms(loan, share, 0x15);
}

/// Comparer of `transfer_screens_open_loan_bid_from_news`: only the original runs (it opens a screen or withdraws the
/// offer).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_loan_bid_from_news(id_a: u32, id_b: u32) {
    original_fn::<unsafe extern "C" fn(u32, u32)>(IDX_TRANSFER_SCREENS_OPEN_LOAN_BID_FROM_NEWS)(id_a, id_b)
}

/// Opens the loan bid screen for `player` on behalf of the club of the slot's manager.
///
/// When `TRANSFER_MANAGER::can_loan_player(club, player, TS_TEXT)` refuses: the box "Loan offer for <player>" (name
/// style 5) with the reason in `TS_TEXT`, in the club's kit colours (`confirm_push(club name, title, TS_TEXT, 0, b, a,
/// 0)`), 0. Otherwise the screen (build `transfer_screens_build_loan_bid_screen`, input
/// `transfer_screens_handle_loan_bid_screen`, flag 1; not opened: 0) and its params: 0 the player, 1 his contract's
/// wage (`contract_manager_get_contract`, not stored in the lookup global), 2 the club's maximum wage for him
/// (`Finance::get_max_wage(player, -1)`), the loan period of the club with his contract club (looked up inline, stored
/// in the lookup global) (`get_loan_period`): 3 the first month, 4 the first year, 5 the last month, 6 the last year;
/// 7 and 8 0; 0xb 0, 0xc and 0xd the first month and year, 0xe and 0xf 1. A contract offer of the club to him
/// (`find_contract_offer(player id, club id)`) of type 6 (the low 6 bits) gives its terms to 0xb..0xf and 0x10..0x14.
/// Returns 1.
/// Original: `transfer_screens_open_loan_bid_screen` (0x8f9690, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_loan_bid_screen(player: usize) -> i32 {
    let manager = tel26_slot_manager();
    let club = || manager.club_job().addr();
    if exe::transfer_manager_can_loan_player(read_usize(G_TRANSFER_MANAGER), club(), player, TS_TEXT) == 0 {
        covered!("tel18_5 open loan bid screen: refused");
        let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
        let confirm: unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(TS_FN_CONFIRM_PUSH);
        let mut name = [0u8; 0x190];
        let n = name.as_mut_ptr() as usize;
        exe::database_format_staff_name(player, n, 0x190, 5);
        let mut title = [0u8; 0x190];
        let t = title.as_mut_ptr() as usize;
        lang(t, 0xa9edcc, n); // "Loan offer for %s"
        let (mut a, mut b) = (0u32, 0u32);
        exe::database_get_club_kit_colours(club(), 1, &mut a, &mut b);
        confirm(club() + club::NAME, t, TS_TEXT, 0, b, a, 0);
        return 0;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, TS_LOAN_BID_BUILD, TS_LOAN_BID_INPUT, 1, 0usize, 0) == 0 {
        return 0;
    }
    covered!("tel18_5 open loan bid screen");
    set_screen_param(0, player);
    let contract = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player));
    set_screen_param(1, (contract.wage() as u32) as usize);
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club());
    set_screen_param(2, exe::finance_get_max_wage(finance, player, u32::MAX) as u32 as usize);
    let contract_club = ts_contract_club_any(player);
    let (mut first_month, mut first_year, mut last_month, mut last_year) = (0i8, 0u16, 0i8, 0u16);
    exe::transfer_manager_get_loan_period(read_usize(G_TRANSFER_MANAGER), club(), contract_club, player, &mut first_month, &mut first_year,
        &mut last_month, &mut last_year);
    set_screen_param(3, first_month as i32 as usize);
    set_screen_param(4, first_year as usize);
    set_screen_param(5, last_month as i32 as usize);
    set_screen_param(6, last_year as usize);
    set_screen_param(7, 0);
    set_screen_param(8, 0);
    set_screen_param(0xb, 0);
    set_screen_param(0xc, first_month as i32 as usize);
    set_screen_param(0xd, first_year as usize);
    set_screen_param(0xe, 1);
    set_screen_param(0xf, 1);
    let terms = Contract(exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), read_u32(player), read_u32(club())));
    if !terms.is_null() && terms.contract_type() & 0x3f == 6 {
        covered!("tel18_5 open loan bid screen with a loan contract offer");
        let share = ts_loan_wage_share(read_i32(terms.addr() + TS_WAGE), contract.wage());
        ts_set_loan_terms(terms.addr(), share, 0xb);
        ts_set_loan_terms(terms.addr(), share, 0x10);
    }
    1
}

/// Comparer of `transfer_screens_open_loan_bid_screen`: only the original runs (it opens a screen or a box).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_loan_bid_screen(player: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize) -> i32>(IDX_TRANSFER_SCREENS_OPEN_LOAN_BID_SCREEN)(player)
}

/// Builds the loan bid screen.
///
/// The params: 0 the player, 1 his wage, 2 the club's maximum wage, 3/4 the first month and year of the loan period,
/// 5/6 the last, 7 the offer being answered (or 0), 0xa its negotiated loan contract, 0xb the wage share (a negative
/// byte becomes 0, written back), 0xc/0xd the loan's expiry month and year, 0xe/0xf the cup matches and the matches
/// against the parent club. His contract club is looked up inline (the lookup global) at the start. Without the player
/// (line 0x23f8), the slot's manager (0x2406) or his club (0x240d): the assert box.
///
/// The player's picture, the title (the club's short name) in its kit colours, the subtitle (name style 5) "<buyer>
/// loan bid for <player>" when the offer's buyer is another club, else "Loan bid for <player>". With an offer, a small
/// area (0x212,0x8c..0x30c,0xa0, 2 columns): Reset (id 0xe, font 1; greyed when the terms equal params 0x10..0x14)
/// and, with a loan contract, Negotiated Bid (id 0xd; greyed when equal to 0x15..0x19); the table then starts at 0xa5.
/// Without an offer param 0x1e is -1 and the table starts at 0x8c.
///
/// The table (7 columns 2:0x14:2:0xa:2:2:2, as many rows of font 3 as fit): "<player> is currently on a <wage>
/// contract with <club>." (the wage by `STAFF_CONTRACT::format_wage_text(2, 2)` of a local contract, links to the
/// player, the wage (7) and the club), plus " Your board will allow wages up to <max wage>." (a link 7) when the
/// contract club (looked up again) is not the human's; a blank row; Loan Expiry Date with "-" (id 1; grey at the first
/// year and up to its month), the date (day = today's, at most 28; pale when `can_end_loan(contract club looked up
/// again, player, 0, date, 1)`, else grey) and "+" (id 2; grey at the last year from its month); Wage Contribution
/// with "-" (id 3; grey at 0), "<share>%" (red when share * wage / 100 is over the maximum wage) and "+" (id 4; grey at
/// 100); "(<contribution>)" in orange (`format_wage_text(0, 1)`); two rows down "Player can play in cup matches" (a
/// link 8) and "Player can play in matches against <contract club>" (a link 9), each pale when its param (re-read) is
/// set. The menu bar, then the bar in the kit's second colour: with an offer (2:1:1) Back (5) and, without a loan
/// contract, Reject (0xc) and Accept (0xb, terms equal to 0x10..0x14) or Negotiate (0xa), each greyed when the loan
/// cannot end; with a loan contract Cancel (0xf) and Offer (6); without an offer (3:1) Back (5) and Offer (6, plain
/// grey text when not). Offer is greyed unless the loan can end and the contribution is within the maximum wage.
/// Original: `transfer_screens_build_loan_bid_screen` (0x8f9a20, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_build_loan_bid_screen() {
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0usize);
    let mut contract = [0u8; 0x50];
    let c = contract.as_mut_ptr() as usize;
    exe::new_staff_contract(c);
    ts_build_loan_bid(t as usize, c);
    exe::drop_text(t);
}

/// Comparer of `transfer_screens_build_loan_bid_screen`: side by side over the display list, the params and the
/// contract lookup global; without the player, the manager or his club only the original runs (the assert box).
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_loan_bid_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_SCREENS_BUILD_LOAN_BID_SCREEN);
    let manager = tel26_slot_manager();
    if get_screen_param(0) == 0 || manager.is_null() || manager.club_job().addr() == 0 {
        return o();
    }
    compare_by_display_list_with("Loan bid screen", &[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(), || transfer_screens_build_loan_bid_screen());
}

/// Answers the loan bid screen (the params as in the build; param 0xa is read and left unused).
///
/// Back (id 5) -9. Offer (6): the confirm loan offer dialog with the player, the month, the year, the share, the two
/// match bytes and no offer (`transfer_screens_open_confirm_loan_offer`), -4. "-"/"+" of the expiry date (1/2): the
/// month one less or more, wrapping to 11 with the year word one less, or to 0 with the year one more (params 0xc, 0xd;
/// the period params 3..6 and the player read and left unused). "-"/"+" of the share (3/4): 10 less or more (param 0xb,
/// a byte). The share's value popup (7): the item's value capped at the smaller of the wage and the maximum wage, in
/// percent of the wage (param 0xb, the quotient's byte). The cup (8) and parent club (9) links toggle params 0xe / 0xf.
/// Reset (0xe) / Negotiated Bid (0xd): params 0xb..0xf from 0x10..0x14 / 0x15..0x19. These answer -11. Negotiate
/// (0xa), Accept (0xb) and Reject (0xc): when the offer's serial is no longer param 9 or it was withdrawn, the offer
/// changed screen (-11); else Negotiate opens the confirm loan offer dialog with the offer (-4), Accept and Reject the
/// bid confirmation of the offer's slot (`get_offer_slot(offer::SLOT_INDEX)`) with action 0x1d or 0x1b (-4). Cancel
/// (0xf) withdraws the offer (`withdraw_transfer_offer(slot, 0, 0)`), -9. The rest goes to the menu bar.
/// Original: `transfer_screens_handle_loan_bid_screen` (0x8fac30, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_handle_loan_bid_screen(item: u32) -> i32 {
    let player = get_screen_param(0);
    let offer = Offer(get_screen_param(7));
    get_screen_param(0xa);
    let share = get_screen_param(0xb) as u8;
    let mut month = get_screen_param(0xc) as u8;
    let mut year = get_screen_param(0xd) as u32;
    let cup = get_screen_param(0xe) as u32;
    let parent = get_screen_param(0xf) as u32;
    let id = tel26_item_id(item as i16);
    let copy_terms = |base: u32| {
        for k in 0..5 {
            set_screen_param(0xb + k, get_screen_param(base + k));
        }
    };
    match id {
        5 => -9,
        6 => {
            covered!("tel18_5 loan bid Offer");
            transfer_screens_open_confirm_loan_offer(player, month as u32, year, share as u32, cup, parent, 0);
            -4
        }
        1 | 2 => {
            covered!("tel18_5 loan bid expiry step");
            let (a, b) = if id == 1 { (3, 4) } else { (5, 6) };
            get_screen_param(a);
            get_screen_param(b);
            get_screen_param(0);
            if id == 1 {
                month = month.wrapping_sub(1);
                if (month as i8) < 0 {
                    month = 0xb;
                    year = year.wrapping_add(0xffff);
                }
            } else {
                month = month.wrapping_add(1);
                if month as i8 > 0xb {
                    month = 0;
                    year = year.wrapping_add(1);
                }
            }
            set_screen_param(0xc, month as i8 as i32 as usize);
            set_screen_param(0xd, (year & 0xffff) as usize);
            -11
        }
        3 | 4 => {
            covered!("tel18_5 loan bid share step");
            let v = if id == 3 { share.wrapping_sub(10) } else { share.wrapping_add(10) };
            set_screen_param(0xb, v as i8 as i32 as usize);
            -11
        }
        7 => {
            covered!("tel18_5 loan bid share popup");
            let mut v = tel26_item_value(item as i16) as i32;
            let wage = get_screen_param(1) as i32;
            let max = get_screen_param(2) as i32;
            let cap = if wage < max { wage } else { max };
            if v >= cap {
                v = cap;
            }
            set_screen_param(0xb, (v.wrapping_mul(100) / wage) as i8 as i32 as usize);
            -11
        }
        8 => {
            set_screen_param(0xe, (cup == 0) as usize);
            -11
        }
        9 => {
            set_screen_param(0xf, (parent == 0) as usize);
            -11
        }
        0xa..=0xc => {
            let serial = get_screen_param(9) as u32;
            if (offer.serial() as u32) != serial || offer.status() == 3 {
                covered!("tel18_5 loan bid: offer changed");
                ts_open_offer_changed(offer.addr(), serial);
                return -11;
            }
            if id == 0xa {
                covered!("tel18_5 loan bid Negotiate");
                transfer_screens_open_confirm_loan_offer(player, month as u32, year, share as u32, cup, parent, offer.addr());
            } else {
                covered!("tel18_5 loan bid Accept or Reject");
                let slot = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), offer.slot_index() as i32);
                transfer_screens_open_bid_confirm_dialog(0, if id == 0xb { 0x1d } else { 0x1b }, slot);
            }
            -4
        }
        0xe => {
            copy_terms(0x10);
            -11
        }
        0xd => {
            copy_terms(0x15);
            -11
        }
        0xf => {
            covered!("tel18_5 loan bid Cancel");
            let slot = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), offer.slot_index() as i32);
            exe::transfer_manager_withdraw_transfer_offer(read_usize(G_TRANSFER_MANAGER), slot as *mut usize, 0, 0);
            -9
        }
        _ => ts_menubar_answer(item),
    }
}

/// Comparer of `transfer_screens_handle_loan_bid_screen`: the answers that only edit params (the steps, the popup, the
/// links, Reset, Negotiated Bid) and Back side by side over the params; Offer, Negotiate, Accept, Reject (screens),
/// Cancel (the offer) and the menu bar only in the original.
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_loan_bid_screen(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRANSFER_SCREENS_HANDLE_LOAN_BID_SCREEN);
    ts_compare_answer("Loan bid screen", item, &[1, 2, 3, 4, 5, 7, 8, 9, 0xd, 0xe], o, transfer_screens_handle_loan_bid_screen)
}

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
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_TRANSFER_SCREENS_OPEN_CONFIRM_TRIAL_OFFER)(player, weeks)
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
    original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, usize)>(IDX_TRANSFER_SCREENS_OPEN_CONFIRM_LOAN_OFFER)(player, month, year, a4, a5, a6, offer)
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
    let text_area: TsTextAreaLinks = core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    let player = get_screen_param(0);
    let weeks = get_screen_param(1) as u8;
    let offer = Offer(get_screen_param(7));
    picture(player, 0, 0);
    let manager = tel26_slot_manager();
    if manager.is_null() {
        tel26_error_box(TS_CPP, 0x26f4);
        return;
    }
    covered!("tel18_6 confirm offer dialog");
    ts_title_in_club_colours(manager.club_job().addr());
    let mut name = [0u8; 0x12c];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(player, n, 0x12c, 5);
    lang(TS_TEXT, if weeks != 0xff { 0xaa1568 } else if !offer.is_null() { 0xaa1528 } else { 0xaa14ec }, n);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    let lines = ts_dialog_lines();
    if weeks != 0xff {
        exe::database_format_staff_name(player, n, 0x12c, 1);
        if weeks == 1 {
            lang(TS_TEXT, 0xaa14ac, n); // "Offer a 1 week trial to <player> ?"
        } else {
            lang(TS_TEXT, 0xaa1458, weeks as i8 as i32, n);
        }
    } else if !offer.is_null() {
        let club = Club(club_at((offer.buyer_club_id() as i32) as isize).addr());
        let g = (club.short_gender_name() as i8) as i32;
        lang(TS_TEXT, 0xaa13f4, g, g, club.addr() + club::SHORT_NAME, n);
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
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_SCREENS_BUILD_CONFIRM_OFFER_DIALOG);
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
    let manager = tel26_slot_manager();
    if weeks != 0xff {
        covered!("tel18_6 confirm trial Yes");
        let mut contract = [0u8; 0x50];
        let c = contract.as_mut_ptr();
        exe::new_staff_contract(c as usize);
        exe::staff_contract_init_trial_offer(c as usize, player, manager.club_job().addr(), weeks as u32);
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
        exe::transfer_manager_add_loan_negotiation(read_usize(G_TRANSFER_MANAGER), offer, d, a4, a5, a6);
    } else {
        covered!("tel18_6 confirm loan Yes: new offer");
        exe::transfer_manager_make_loan_offer(read_usize(G_TRANSFER_MANAGER), player, manager.club_job().addr(), d, a4, a5, a6);
    }
    -10
}

/// Comparer of `transfer_screens_handle_confirm_offer_dialog`: No side by side; Yes (the offers) and the menu bar only
/// in the original.
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_confirm_offer_dialog(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRANSFER_SCREENS_HANDLE_CONFIRM_OFFER_DIALOG);
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
    let player = Staff(player);
    let manager = tel26_slot_manager();
    if exe::transfer_manager_can_end_loan(read_usize(G_TRANSFER_MANAGER), manager.club_job().addr(), player, TS_TEXT as u32, G_CURRENT_DATE, 0) == 0 {
        covered!("tel18_6 cannot end loan");
        let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
        let (mut name, mut heading) = ([0u8; 0x190], [0u8; 0x190]);
        let (n, h) = (name.as_mut_ptr() as usize, heading.as_mut_ptr() as usize);
        exe::database_format_staff_name(player, n, 0x190, 5);
        if manager.club_job() == player.club_job() {
            lang(h, 0xaa15d4, 0x64, n);
        } else {
            lang(h, 0xaa15a0, n);
        }
        let club = manager.club_job();
        let (mut a, mut b) = (0u32, 0u32);
        exe::database_get_club_kit_colours(club, 1, &mut a, &mut b);
        exe::confirm_push(club.addr() + club::NAME, h, TS_TEXT, 0, b, a, 0);
        return;
    }
    covered!("tel18_6 open end loan dialog");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8fbdf0usize, 0x8fc0e0usize, 1, 0usize, 0) != 0 {
        set_screen_param(0, player.addr());
        set_screen_param(1, player.club_job().addr());
    }
}

/// Comparer of `transfer_screens_open_end_loan_dialog`: only the original runs (it opens a box or a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_end_loan_dialog(player: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(IDX_TRANSFER_SCREENS_OPEN_END_LOAN_DIALOG)(player)
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
    let text_area: TsTextAreaLinks = core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    let player = Staff(get_screen_param(0));
    let club = Club(get_screen_param(1));
    picture(player.addr(), 0, 0);
    let manager = tel26_slot_manager();
    if manager.is_null() {
        tel26_error_box(TS_CPP, 0x27f0);
        return;
    }
    covered!("tel18_6 end loan dialog");
    let own = manager.club_job().addr();
    ts_title_in_club_colours(own);
    let mut name = [0u8; 0x12c];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(player, n, 0x12c, 5);
    if own == player.club_job().addr() {
        lang(TS_TEXT, 0xaa170c, 0, n);
    } else {
        lang(TS_TEXT, 0xaa16d4, n);
    }
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    let lines = ts_dialog_lines();
    exe::database_format_staff_name(player, n, 0x12c, 1);
    let g = (club.short_gender_name() as i8) as i32;
    if own == player.club_job().addr() {
        lang(TS_TEXT, 0xaa166c, 0, g, g, n, club.addr() + club::SHORT_NAME); // "Terminate <player>'s loan contract at <club> ?"
    } else {
        lang(TS_TEXT, 0xaa1610, g, g, n, club.addr() + club::SHORT_NAME);
    }
    text_area(0x6e, 0x8c, 0x30c, 0x221, 2, 0, lines as i8 as i32 as u32, 3, ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT,
        n, 0x7d7u32, player.addr(), club.addr() + club::SHORT_NAME, 0x7d5u32, club.addr(), 0u32);
    exe::menubar_build(4, 0);
    ts_no_yes_bar(1, 2);
}

/// Comparer of `transfer_screens_build_end_loan_dialog`: side by side over the display list; without the slot's
/// manager only the original runs (the assert box).
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_end_loan_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRANSFER_SCREENS_BUILD_END_LOAN_DIALOG);
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
    let player = Staff(get_screen_param(0));
    let contract_club = Club(ts_contract_club_any(player.addr()));
    let own = Club(read_usize(tel26_slot_manager().addr() + staff::CLUB_JOB));
    exe::contract_manager_schedule_loan_end(read_usize(G_CONTRACT_MANAGER), player, own);
    let (mut a, mut b) = (0u32, 0u32);
    exe::database_get_club_kit_colours(own, 1, &mut a, &mut b);
    let (mut name, mut heading) = ([0u8; 0x190], [0u8; 0x258]);
    let (n, h) = (name.as_mut_ptr() as usize, heading.as_mut_ptr() as usize);
    exe::database_format_staff_name(player, n, 0x190, 5);
    if own == player.club_job() {
        lang(h, 0xaa17d8, 0, n);
    } else {
        lang(h, 0xaa17a4, n);
    }
    exe::database_format_staff_name(player, n, 0x190, 1);
    let g = (contract_club.short_gender_name() as i8) as i32;
    lang(TS_TEXT, 0xaa1744, g, g, n, contract_club.addr() + club::SHORT_NAME);
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, TS_TEXT);
    exe::text_add_link(t as usize, n as *const u8, 0x7d7, player);
    exe::text_add_link(t as usize, (own.addr() + club::SHORT_NAME) as *const u8, 0x7d5, own);
    exe::confirm_push_text_object(own.addr() + club::NAME, h, t as usize, 0, a, b, 3);
    exe::drop_text(t);
    -4
}

/// Comparer of `transfer_screens_handle_end_loan_dialog`: No side by side; the network command, Yes (the loan end and
/// the box) and the menu bar only in the original.
pub(crate) unsafe extern "C" fn compare_transfer_screens_handle_end_loan_dialog(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRANSFER_SCREENS_HANDLE_END_LOAN_DIALOG);
    if read_u32(TS_NETWORK_GAME) != 0 && read_u32(G_PENDING_COMMAND_CODE) == 3 {
        return o(item);
    }
    ts_compare_answer("End loan dialog", item, &[1], o, transfer_screens_handle_end_loan_dialog)
}

/// Rule: the bid screen opens for a player, with an existing offer (its serial number) or a new one; it notes
/// whether the human's club may sign him now and whether the offer is already accepted.
///
/// `transfer_screens_open_bid_screen_for_player(person, offer_slot)`: the offer serial is -1, or the
/// `offer::SERIAL` of the offer the slot points to. `open_screen` with the builder 0x8e9760, the handler 0x8ec420,
/// single instance, the leave routine 0x93a710 and 0; when it opens: parameter 0 = the player's id, 1 = the slot, 2 =
/// 0, 3 = the serial, 4 and 5 = 0, 6 = 1; the player's contract (`CONTRACT_MANAGER::get_contract` on
/// `G_CONTRACT_MANAGER`, kept in `G_CONTRACT_LOOKUP_RESULT`) gives the selling club (`contract::CLUB`) unless its
/// `contract::CONTRACT_TYPE` & 0x3f is 5; parameter 8 = 0; 9 = `TRANSFER_MANAGER::can_sign_player(human club, today,
/// player, selling club, 1, 0)` on `G_TRANSFER_MANAGER` as 0/1 (the human of the current slot's `staff::CLUB_JOB`); 7 =
/// 1 when the slot's offer has an `offer::STATUS` of 5 or more other than 0x10, else 0; 0xf..0x16 = -1.
/// Original: `transfer_screens_open_bid_screen_for_player` (0x8e9350, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_bid_screen_for_player(person: usize, offer_slot: usize) {
    let offer = Offer(if offer_slot != 0 { read_usize(offer_slot) } else { 0 });
    let serial = if !offer.is_null() { (offer.serial() as u32) as usize } else { usize::MAX };
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8e9760usize, 0x8ec420usize, 1, 0x93a710usize, 0) == 0 {
        return;
    }
    covered!("tel36_4 open the bid screen");
    set_screen_param(0, read_u32(person) as usize);
    set_screen_param(1, offer_slot);
    set_screen_param(2, 0);
    set_screen_param(3, serial);
    set_screen_param(4, 0);
    set_screen_param(5, 0);
    set_screen_param(6, 1);
    let contract = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    write_usize(G_CONTRACT_LOOKUP_RESULT, contract.addr());
    let seller = if !contract.is_null() && contract.contract_type() & 0x3f != 5 {
        read_usize(G_CLUB_TABLE).wrapping_add((contract.club() as u32).wrapping_mul(club::SIZE as u32) as usize)
    } else {
        0
    };
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    let human = Staff(read_usize((G_SCREEN_RECORDS_MANAGER as isize + slot * 0x300) as usize));
    let human_club = human.club_job().addr();
    set_screen_param(8, 0);
    let can = exe::transfer_manager_can_sign_player(read_usize(G_TRANSFER_MANAGER), human_club, G_CURRENT_DATE, person, seller, 1, core::ptr::null_mut());
    set_screen_param(9, (can != 0) as usize);
    let accepted = !offer.is_null() && {
        let status = offer.status() as i8;
        status >= 5 && status != 0x10
    };
    set_screen_param(7, accepted as usize);
    for k in 0xf..=0x16 {
        set_screen_param(k, usize::MAX);
    }
}

/// Comparer for `transfer_screens_open_bid_screen_for_player`.
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_bid_screen_for_player(person: usize, offer_slot: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize, usize)>(IDX_TRANSFER_SCREENS_OPEN_BID_SCREEN_FOR_PLAYER)(person, offer_slot)
}

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
pub(crate) unsafe fn ts_at(area: u32, col: u32, row: i32, a: u32, b: u32, c: u32, colour: usize, text: usize) -> u32 {
    tel26_area_text([area, col, row as u32, a, b, c, 3, ts_colour(colour), text as u32])
}

/// `add_area_button(area, col, row, a, b, c, 3, colour, text, id, value)`.
pub(crate) unsafe fn ts_ab(area: u32, col: u32, row: i32, a: u32, b: u32, c: u32, colour: usize, text: usize, id: u32, value: u32) -> u32 {
    tel26_area_button([area, col, row as u32, a, b, c, 3, ts_colour(colour), text as u32, id, value])
}

/// A popup area attached to the item `item` (an area of 0x130 style, the popup colour).
pub(crate) unsafe fn ts_popup_for(item: u32) -> u32 {
    tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, ts_colour(TS_COLOUR_POPUP), item as i16 as i32 as u32])
}

/// Whether `amount` is over the transfer budget `budget`, as display amounts (mode 4).
pub(crate) unsafe fn ts_over_budget(amount: u32, budget: u32) -> bool {
    ts_display(amount, 4) > ts_display(budget, 4)
}

/// The inline `TRANSFER_OFFER` copy of the bid screen: every field of `src` into `dst` (the completion timing and the
/// offer kind only by their low nibble), then `dst`'s first dword cleared and, when it has an exchange list, a new list
/// filled from `src`'s.
pub(crate) unsafe fn ts_copy_offer(dst: usize, src: usize) {
    let src = Offer(src);
    let dst = Offer(dst);
    for k in [0usize, 4, 8, 0xc] {
        write_u32(dst.addr() + k, read_u32(src.addr() + k));
    }
    dst.set_terms_state(src.terms_state());
    dst.set_job_type(src.job_type());
    let mut tmp = [0u32; 2];
    exe::date_operator_3d(dst.addr() + offer::DUE_DATE, tmp.as_mut_ptr() as usize, src.addr() + offer::DUE_DATE);
    dst.set_due_slice(src.due_slice());
    dst.set_fee(((src.fee() as u32)) as usize);
    dst.set_offer_type(src.offer_type());
    for k in [0x20usize, 0x24, 0x28] {
        write_u32(dst.addr() + k, read_u32(src.addr() + k));
    }
    for k in [0x2cusize, 0x2d, 0x2e, 0x2f] {
        write_u8(dst.addr() + k, read_u8(src.addr() + k));
    }
    let b = dst.completion_timing();
    dst.set_completion_timing(((((src.completion_timing() << 4) as i8) >> 4) as u8 ^ b) & 0xf ^ b);
    let d = dst.offer_kind() as u32;
    dst.set_offer_kind((((src.offer_kind() as u32) ^ d) & 0xf ^ d) as usize);
    dst.set_appearance_fee(((src.appearance_fee() as u32)) as usize);
    dst.set_appearance_games(src.appearance_games());
    dst.set_international_fee(((src.international_fee() as u32)) as usize);
    dst.set_international_games(src.international_games());
    dst.set_sell_on_percent(src.sell_on_percent());
    dst.set_instalments(src.instalments());
}

/// After `ts_copy_offer`: the first dword cleared and, with an exchange list, a new LLIST copied from `src_list`.
pub(crate) unsafe fn ts_copy_offer_list(dst: usize, src_list: usize) {
    let dst = Offer(dst);
    write_u32(dst.addr(), 0);
    if dst.exchange_players().addr() != 0 {
        let l = exe::crt_operator_new(0xe);
        let l = if l != 0 { exe::new_llist(l) } else { 0 };
        write_usize(dst.addr() + offer::EXCHANGE_PLAYERS, l);
        exe::llist_copy_from(l, src_list);
    }
}

/// The colour word at `a` as the plain field value (the cell fields that are colours, not the text colour).
pub(crate) unsafe fn ts_colour_addr(a: usize) -> u32 {
    if a == 0 { 0 } else { ts_colour(a) }
}

/// One of the Monthly Installments / Percentage of Next Sale rows (the label already in `TS_TEXT`): the label (a
/// button `ids[0]` when editable; pale with a value, grey without), "-" (grey text at `floor` or read only; else a
/// button `ids[1]` with a value, `ids[0]` without), the value (`fill(value)` or "-") and "+" (grey text at `cap` or read
/// only; else `ids[2]` / `ids[0]`). Returns the label's item.
pub(crate) unsafe fn ts_stepper_row(area: u32, row: i32, read_only: bool, value: i8, floor: i8, cap: i8, ids: [u32; 3], value_field: u32, fill: impl Fn(i8)) -> i32 {
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
pub(crate) unsafe fn ts_clause_row(t: usize, area: u32, row: &mut i32, read_only: bool, fee_at: usize, games: i8, ids: [u32; 4], value_field: u32, param: u32,
    value_colour: &dyn Fn(u32) -> usize) -> u32 {
    let format: TsLang = core::mem::transmute(FN_CRT_SPRINTF);
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
        (exe::text_set_string(t, (TS_TEXT) as usize as *const u8) as i32);
        format(TS_TEXT, 0x98806c, games as i32); // "%d"
        exe::text_add_link(t, (TS_TEXT) as usize as *const u8, ids[1], 0);
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

/// Steps a cash value (`Cash::step_up` / `step_down` in mode 4) and returns it.
pub(crate) unsafe fn ts_step(value: u32, up: bool) -> u32 {
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
pub(crate) unsafe fn ts_budget_of(club: usize, staff: usize) -> u32 {
    let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    exe::finance_get_transfer_budget(fin, staff as u32, 0, 0usize, 1, 0) as u32
}

/// A widget of the bid screen set to a state: flags, text colour and id.
pub(crate) unsafe fn ts_widget(item: u32, flags: u32, colour: usize, id: u32) {
    exe::screenmanager_set_widget_flags(G_SCREEN_MANAGER, item as usize, flags);
    exe::screenmanager_set_widget_text_colour(G_SCREEN_MANAGER, item, ts_colour(colour));
    exe::screenmanager_set_widget_id(G_SCREEN_MANAGER, item, id);
}

/// The value widget of a money row after a step: the formatted value (mode 5) and, when bidding, red over the budget.
pub(crate) unsafe fn ts_value_widget(item: u32, value: u32, bidding: bool, budget: u32) {
    ts_money_text(value, 5);
    exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, item, TS_TEXT);
    let colour = if bidding && ts_over_budget(value, budget) { G_GUI_COLOUR_RED } else { G_GUI_COLOUR_PALE_GREY };
    exe::screenmanager_set_widget_text_colour(G_SCREEN_MANAGER, item, ts_colour(colour));
}

/// The bid is replaced by a new copy of `src` (`ts_copy_offer`, the list copied from `src`'s) and param `param` set to 0.
pub(crate) unsafe fn ts_reset_bid(slot: usize, src: usize, param: u32) {
    let src = Offer(src);
    let new_empty: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(TS_FN_NEW_EMPTY_TRANSFER_OFFER);
    let old = read_usize(slot);
    if old != 0 {
        exe::drop_transfer_offer(old);
        exe::crt_operator_delete(old);
    }
    let mem = exe::crt_operator_new(0x41);
    let o = if mem != 0 { new_empty(mem) } else { 0 };
    write_usize(slot, o);
    ts_copy_offer(o, src.addr());
    ts_copy_offer_list(read_usize(slot), src.exchange_players().addr());
    set_screen_param(param, 0);
}

/// Opens one of the bid dialogs: build/input `screen`, params 0 the bid slot, 1 `action`, 2 `extra`; -4 either way.
pub(crate) unsafe fn ts_open_bid_dialog(build: usize, input: usize, slot: usize, action: u32, extra: usize) -> i32 {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, build, input, 1, 0usize, 0) != 0 {
        set_screen_param(0, slot);
        set_screen_param(1, action as usize);
        set_screen_param(2, extra);
    }
    -4
}

/// The bid is dropped (deleted, slot cleared) and the offer changed screen opens for `live` with `serial`; -11.
pub(crate) unsafe fn ts_bid_overtaken(slot: usize, live: usize, serial: usize) -> i32 {
    ts_delete_offer(slot);
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8f24c0usize, 0x8f2800usize, 1, 0usize, 0) != 0 {
        set_screen_param(0, live);
        set_screen_param(1, serial);
    }
    -11
}

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
    let club = Club(club);
    let mut a = read_u16(G_GUI_COLOUR_BLUE) as u32;
    let mut b = read_u16(G_GUI_COLOUR_PALE_GREY) as u32;
    exe::database_get_club_kit_colours(club, 1, &mut a, &mut b);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, b, a, club.addr() + club::NAME);
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
    let o = read_usize(slot);
    if o != 0 {
        exe::drop_transfer_offer(o);
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

/// `LLIST::~LLIST` on a list of the frame.
pub(crate) unsafe fn llist_drop(l: usize) {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    f(l);
}

/// The contract club of `player` when the contract is not of type 5 (`contract +0x35` low 6 bits), else 0, looked up
/// and stored in `G_CONTRACT_LOOKUP_RESULT` like the original's inline macro.
pub(crate) unsafe fn ts_contract_club_not_type_5(player: usize) -> usize {
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    if c == 0 || read_u8(c + 0x35) & 0x3f == 5 {
        0
    } else {
        club_at(read_i32(c + 4) as isize).addr()
    }
}

/// The news item `id_a`/`id_b` copied into `item` (a NewsItem built empty, then `News::find_item_by_id`).
pub(crate) unsafe fn ts_find_news_item(item: &mut [u8; 0xe0], id_a: u32, id_b: u32) {
    let find: unsafe extern "thiscall" fn(usize, u32, u32, usize) -> usize = core::mem::transmute(TS_FN_NEWS_FIND_ITEM);
    let n = item.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0, 0);
    find(read_usize(G_NEWS), id_a, id_b, n);
}

/// Opens the offer changed screen for `offer` (params 0 the offer, 1 the serial the news item knew).
pub(crate) unsafe fn ts_open_offer_changed(offer: usize, serial: u32) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, TS_OFFER_CHANGED_BUILD, TS_OFFER_CHANGED_INPUT, 1, 0usize, 0) != 0 {
        set_screen_param(0, offer);
        set_screen_param(1, serial as usize);
    }
}

/// `News::set_item_slot(id a, id b, slot, value, 0)` (0x786d30, thiscall on [G_NEWS] ret 0x14), keeping eax.
pub(crate) unsafe fn ts_news_slot(id_a: usize, id_b: usize, slot: u32, value: u32) -> u32 {
    exe::news_set_item_slot(read_usize(G_NEWS), id_a, id_b, slot, value, 0)
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

/// The Yes/No dialogs about one player (shortlist, comparison): param 0 the player's id, param 1 the choice. The
/// player's picture, the title (the slot manager's name, in the colours of his club job, else his national job, else
/// blue on pale grey), the subtitle `subtitle`, the text `text` with the player's name as a link (0x7d7), the menu bar
/// and the bar No (`no_id`) / Yes (`yes_id`).
pub(crate) unsafe fn ts_player_yes_no_dialog(subtitle: usize, text: usize, in_the_id: u32, yes_id: u32) {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let text_area: TsTextAreaLinks = core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    let mut pale = read_u16(G_GUI_COLOUR_PALE_GREY) as u32;
    let mut blue = read_u16(G_GUI_COLOUR_BLUE) as u32;
    let player = staff_at(get_screen_param(0) as i32 as isize).addr();
    let manager = tel26_slot_manager();
    let mut club = manager.club_job().addr();
    if club == 0 {
        club = manager.national_job().addr();
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
    ts_bar_button(bar, 0, TS_NO, in_the_id);
    ts_bar_button(bar, 1, TS_YES, yes_id);
}

/// A popup item line of the transfers screen's menus: `selected` gives the 0x2002 flags and no id, else flags 2 with
/// `id` and `value`; the text is `TS_TEXT`, the area `area`.
pub(crate) unsafe fn ts_menu_line(selected: bool, id: u32, value: u32, area: u32) -> u32 {
    let d = current_human_display();
    let (flags, id, value) = if selected { (0x2002, 0, 0) } else { (2, id, value) };
    tel26_add_item(d, [flags, 0, 0, 0, 0, 0, 0, 0x10, ts_colour(TS_COLOUR_POPUP), ts_colour(G_GUI_COLOUR_YELLOW), 1, 1,
        ts_colour(G_GUI_COLOUR_BLACK), TS_TEXT as u32, 0, id, value, area])
}

/// The shortlist manager of the transfer manager (`[TM + 0x856]`).
pub(crate) unsafe fn ts_shortlist_manager() -> usize {
    read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856)
}

/// The three widgets after the "-" of the transfer status screen once the asking price changed (`price`): "-" (param
/// 2) enabled (id 2) above 0, else greyed (flags 6, no data); the price's text; "+" enabled (id 3) when `plus_on`, else
/// greyed (flags 6 and `plus_off`); the Auto widget back to style 0x30; the screen redrawn.
pub(crate) unsafe fn ts_asking_price_widgets(price: i32, plus_on: bool, plus_off: impl Fn(u32)) {
    let mut w = get_screen_param(2) as u32;
    if price <= 0 {
        exe::screenmanager_set_widget_flags(G_SCREEN_MANAGER, w as usize, 6);
        exe::screenmanager_set_widget_text_colour(G_SCREEN_MANAGER, w, ts_colour(G_GUI_COLOUR_GREY));
        exe::screenmanager_set_widget_data(G_SCREEN_MANAGER, w, 0usize);
    } else {
        exe::screenmanager_set_widget_flags(G_SCREEN_MANAGER, w as usize, 2);
        exe::screenmanager_set_widget_text_colour(G_SCREEN_MANAGER, w, ts_colour(G_GUI_COLOUR_PALE_GREY));
        exe::screenmanager_set_widget_id(G_SCREEN_MANAGER, w, 2);
    }
    w += 1;
    exe::cash_format_money(TS_TEXT, 0x7d0, ts_display(price as u32, 5), 5);
    exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, w, TS_TEXT);
    w += 1;
    if plus_on {
        exe::screenmanager_set_widget_flags(G_SCREEN_MANAGER, w as usize, 2);
        exe::screenmanager_set_widget_text_colour(G_SCREEN_MANAGER, w, ts_colour(G_GUI_COLOUR_PALE_GREY));
        exe::screenmanager_set_widget_id(G_SCREEN_MANAGER, w, 3);
    } else {
        exe::screenmanager_set_widget_flags(G_SCREEN_MANAGER, w as usize, 6);
        exe::screenmanager_set_widget_text_colour(G_SCREEN_MANAGER, w, ts_colour(G_GUI_COLOUR_GREY));
        plus_off(w);
    }
    w += 1;
    exe::screenmanager_set_widget_style(G_SCREEN_MANAGER, w, 0x30);
    exe::screenmanager_redraw_screen(G_SCREEN_MANAGER, 0, 0);
}

/// The bar No (`no`) / Yes (`yes`) of a dialog, columns weighted 3:1.
pub(crate) unsafe fn ts_no_yes_bar(in_the: u32, yes: u32) {
    let weights: [u8; 2] = [3, 1];
    let bar = ts_button_bar(2, weights.as_ptr());
    ts_bar_button(bar, 0, TS_NO, in_the);
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
    let club = Club(ts_contract_club_any(player));
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, b, a, club.addr() + club::NAME);
}

/// The text area of a dialog with the player's name as a link (0x7d7) and the given trailing link triples.
pub(crate) unsafe fn ts_dialog_text_with_player(name: usize, player: usize) {
    let text_area: TsTextAreaLinks = core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    let lines = ts_dialog_lines();
    text_area(0x6e, 0x8c, 0x30c, 0x221, 2, 0, lines as i8 as i32 as u32, 3, ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT, name, 0x7d7u32, player, 0u32);
}

/// The weeks text of the trial dialog: "1 week" or "<n> weeks".
pub(crate) unsafe fn ts_trial_weeks_text(weeks: i8) {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if weeks == 1 {
        lang(TS_TEXT, 0xa7a0ec);
    } else {
        lang(TS_TEXT, 0xaa1170, weeks as i32);
    }
}

/// The wage share of a loan's terms: the offered wage over the contract's in percent (the quotient's byte), its
/// tenth, times 10 as a byte (`imul dl`), sign-extended. No rounding.
pub(crate) fn ts_loan_wage_share(offered: i32, wage: i32) -> i32 {
    let percent = offered.wrapping_mul(100) / wage;
    (((percent as i8 as i32) / 10) as u8).wrapping_mul(10) as i8 as i32
}

/// Five params from `base` set from a loan's terms (a contract offer or a negotiated loan contract): the wage share,
/// the month of the expiry date (`Date::get_month`, sign-extended), its year word, the cup matches byte and the
/// matches against the parent club byte (sign-extended).
pub(crate) unsafe fn ts_set_loan_terms(terms: usize, share: i32, base: u32) {
    set_screen_param(base, share as u32 as usize);
    set_screen_param(base + 1, exe::date_get_month(terms + TS_CONTRACT_OFFER_EXPIRY) as i8 as i32 as usize);
    set_screen_param(base + 2, read_u16(terms + TS_CONTRACT_OFFER_EXPIRY_YEAR) as usize);
    set_screen_param(base + 3, read_i8(terms + TS_CONTRACT_OFFER_CLAUSE_A) as i32 as usize);
    set_screen_param(base + 4, read_i8(terms + TS_CONTRACT_OFFER_CLAUSE_B) as i32 as usize);
}

/// The body of `transfer_screens_build_loan_bid_screen` with its Text `t` and local STAFF_CONTRACT `c`.
pub(crate) unsafe fn ts_build_loan_bid(t: usize, c: usize) {
    let c = Contract(c);
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: TsLang = core::mem::transmute(FN_CRT_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let wage_text: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(TS_FN_WAGE_TEXT);
    let short_date: unsafe extern "thiscall" fn(*mut [u16; 4], usize, u32, u32) -> usize = core::mem::transmute(TS_FN_SHORT_MONTH_DATE);
    let text_clear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(TS_FN_TEXT_CLEAR);
    let draw: unsafe extern "thiscall" fn(usize, u32, i32, i32, *mut i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(TS_FN_TEXT_DRAW_WRAPPED);
    let weights: [u8; 7] = [2, 0x14, 2, 0xa, 2, 2, 2];
    let player = get_screen_param(0);
    let wage = get_screen_param(1) as i32;
    let contract_club = Club(ts_contract_club_any(player));
    let mut share = get_screen_param(0xb) as u8 as i8;
    if share < 0 {
        set_screen_param(0xb, 0);
        share = 0;
    }
    let max_wage = get_screen_param(2) as i32;
    let first_month = get_screen_param(3) as u8 as i8;
    let first_year = get_screen_param(4) as u16;
    let last_month = get_screen_param(5) as u8 as i8;
    let last_year = get_screen_param(6) as u16;
    let offer = Offer(get_screen_param(7));
    let loan = get_screen_param(0xa);
    let month = get_screen_param(0xc) as u8 as i8;
    let year = get_screen_param(0xd) as u32;
    let cup = get_screen_param(0xe) as u32;
    let parent = get_screen_param(0xf) as u32;
    let terms = TsLoanTerms { share, month, year, cup, parent };
    if player == 0 {
        tel26_error_box(TS_CPP, 0x23f8);
        return;
    }
    let manager = tel26_slot_manager();
    if manager.is_null() {
        tel26_error_box(TS_CPP, 0x2406);
        return;
    }
    let club = manager.club_job();
    if club.is_null() {
        tel26_error_box(TS_CPP, 0x240d);
        return;
    }
    covered!("tel18_5 loan bid screen");
    picture(player, 0, 0);
    let (mut a, mut b) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut a, &mut b);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, b, a, club.addr() + club::SHORT_NAME);
    let mut name = [0u8; 0x190];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(player, n, 0x190, 5);
    if !offer.is_null() && (offer.buyer_club_id() as i32) != read_i32(club.addr()) {
        let buyer = Club(club_at((offer.buyer_club_id() as i32) as isize).addr());
        let g = (buyer.short_gender_name() as i8) as i32;
        lang(TS_TEXT, 0xaa1364, g, g, buyer.addr() + club::SHORT_NAME, n); // "<club> loan bid for <player>"
    } else {
        lang(TS_TEXT, 0xaa1334, n); // "Loan bid for <player>"
    }
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    let d = current_human_display();
    let grey = ts_colour(G_GUI_COLOUR_GREY);
    let pale = ts_colour(G_GUI_COLOUR_PALE_GREY);
    let dark = ts_colour(G_GUI_COLOUR_DARK_BLUE);
    let mut y: u32 = 0x8c;
    if !offer.is_null() {
        covered!("tel18_5 loan bid screen with an offer");
        let small = tel26_add_area(d, [0x212, 0x8c, 0x30c, 0xa0, 2, 0, 1, 0, 1, 0, u32::MAX]);
        lang(TS_TEXT, 0xa7e768); // "Reset"
        if terms.same_as(0x10) {
            ts_greyed_button(small, 1, 1, 0xe);
        } else {
            tel26_area_button([small, 1, 0, 0x30, grey, 0xc, 1, pale, TS_TEXT as u32, 0xe, 0]);
        }
        if loan != 0 {
            lang(TS_TEXT, 0xa9e3e8); // "Negotiated Bid"
            if terms.same_as(0x15) {
                ts_greyed_button(small, 0, 1, 0xd);
            } else {
                tel26_area_button([small, 0, 0, 0x30, grey, 0xc, 1, pale, TS_TEXT as u32, 0xd, 0]);
            }
        }
        y = 0xa5;
    } else {
        set_screen_param(0x1e, usize::MAX);
    }
    // the original pushes the rows byte with the stack slot's other three bytes
    let lines = ((0x221 - y as i32) / exe::font_get_height(3)) as u8;
    let area = tel26_add_area(d, [0x6e, y, 0x30c, 0x221, 7, weights.as_ptr() as u32, lines as u32, 0, 2, 0, u32::MAX]);
    let mut wage_buf = [0u8; 0x164];
    let w = wage_buf.as_mut_ptr() as usize;
    c.set_wage(wage);
    wage_text(c.addr(), w, 2, 2);
    exe::database_format_staff_name(player, n, 0x190, 1);
    let g = (contract_club.short_gender_name() as i8) as i32;
    let club_name = contract_club.addr() + club::SHORT_NAME;
    lang(TS_TEXT, 0xaa12a4, g, g, n, w, club_name); // "<player> is currently on a <wage> contract with <club>."
    exe::text_set_string(t, TS_TEXT as *const u8);
    exe::text_add_link(t, n as *const u8, 0x7d7, player);
    exe::text_add_link(t, w as *const u8, 7, wage as u32 as usize);
    exe::text_add_link(t, club_name as *const u8, 0x7d5, contract_club);
    if club.addr() != ts_contract_club_any(player) {
        covered!("tel18_5 loan bid screen: board wage limit");
        c.set_wage(max_wage);
        wage_text(c.addr(), w, 2, 2);
        lang(TS_TEXT, 0xaa125c, w); // " Your board will allow wages up to <wage>."
        exe::text_append_string(t, TS_TEXT);
        exe::text_add_link(t, w as *const u8, 7, max_wage as u32 as usize);
    }
    let mut row: i32 = 0;
    draw(t, area, 0x78, 0x302, &mut row, 3, pale, 0, 0);
    text_clear(t);
    row += 1;
    ts_at(area, 0, row, 1, 0, 0xc, G_GUI_COLOUR_PALE_GREY, 0);
    row += 1;
    lang(TS_TEXT, 0xaa1248); // "Loan Expiry Date"
    ts_ab(area, 1, row, 1, 0, 1, G_GUI_COLOUR_PALE_GREY, TS_TEXT, 0, 0);
    if year as u16 == first_year && month <= first_month {
        ts_at(area, 2, row, 0x30, dark, 0xc, G_GUI_COLOUR_GREY, TS_MINUS);
    } else {
        ts_ab(area, 2, row, 0x30, dark, 0xc, G_GUI_COLOUR_PALE_GREY, TS_MINUS, 1, 0);
    }
    let today = exe::date_get_day_of_month(G_CURRENT_DATE) as i16;
    let day = if today > 0x1c { 0x1c } else { exe::date_get_day_of_month(G_CURRENT_DATE) as i16 as i32 as u32 };
    let mut date = [0u16; 4];
    // the original pushes the month byte with the stack slot's other three bytes; Date::Date reads the word
    exe::date_date_549f70(&mut date, day, month as u8 as u32, year, u32::MAX);
    short_date(&mut date, TS_TEXT, 0x7d0, 0xff);
    let can_end = exe::transfer_manager_can_end_loan(read_usize(G_TRANSFER_MANAGER), ts_contract_club_any(player), player, 0,
        date.as_mut_ptr() as usize, 1) != 0;
    ts_at(area, 3, row, 1, 0, 0xc, if can_end { G_GUI_COLOUR_PALE_GREY } else { G_GUI_COLOUR_GREY }, TS_TEXT);
    if year as u16 == last_year && month >= last_month {
        ts_at(area, 4, row, 0x30, dark, 0xc, G_GUI_COLOUR_GREY, TS_PLUS);
    } else {
        ts_ab(area, 4, row, 0x30, dark, 0xc, G_GUI_COLOUR_PALE_GREY, TS_PLUS, 2, 0);
    }
    row += 1;
    lang(TS_TEXT, 0xaa1234); // "Wage Contribution"
    ts_ab(area, 1, row, 1, 0, 1, G_GUI_COLOUR_PALE_GREY, TS_TEXT, 0, 0);
    if share == 0 {
        ts_at(area, 2, row, 0x30, dark, 0xc, G_GUI_COLOUR_GREY, TS_MINUS);
    } else {
        ts_ab(area, 2, row, 0x30, dark, 0xc, G_GUI_COLOUR_PALE_GREY, TS_MINUS, 3, 0);
    }
    let contribution = (share as i32).wrapping_mul(wage) / 100;
    format(TS_TEXT, 0x98e8d4, share as i32); // "%d%%"
    let wage_ok = contribution <= max_wage;
    ts_at(area, 3, row, 1, ts_colour(TS_COLOUR_POPUP), 0xc, if wage_ok { G_GUI_COLOUR_PALE_GREY } else { G_GUI_COLOUR_RED }, TS_TEXT);
    if share == 100 {
        ts_at(area, 4, row, 0x30, dark, 0xc, G_GUI_COLOUR_GREY, TS_PLUS);
    } else {
        ts_ab(area, 4, row, 0x30, dark, 0xc, G_GUI_COLOUR_PALE_GREY, TS_PLUS, 4, 0);
    }
    row += 1;
    c.set_wage(contribution);
    wage_text(c.addr(), w, 0, 1);
    format(TS_TEXT, 0x98fb04, w); // "(%s)"
    ts_at(area, 3, row, 1, 0, 0xc, TS_COLOUR_ORANGE, TS_TEXT);
    row += 2;
    lang(TS_TEXT, 0xaa1214); // "Player can play in cup matches"
    exe::text_set_string(t, TS_TEXT as *const u8);
    exe::text_add_link(t, TS_TEXT as *const u8, 8, 0usize);
    let colour = if get_screen_param(0xe) != 0 { pale } else { grey };
    draw(t, area, 0x90, 0x2bc, &mut row, 3, colour, 2, 0);
    text_clear(t);
    row += 1;
    let g = (contract_club.short_gender_name() as i8) as i32;
    lang(TS_TEXT, 0xaa11cc, g, g, club_name); // "Player can play in matches against <club>"
    exe::text_set_string(t, TS_TEXT as *const u8);
    exe::text_add_link(t, TS_TEXT as *const u8, 9, 0usize);
    let colour = if get_screen_param(0xf) != 0 { pale } else { grey };
    draw(t, area, 0x90, 0x2bc, &mut row, 3, colour, 2, 0);
    text_clear(t);
    exe::menubar_build(4, 0);
    // a bar button: enabled with `ts_bar_button`, else the label greyed (font 3)
    let button = |bar: u32, col: u32, label: usize, id: u32, enabled: bool| {
        if enabled {
            ts_bar_button(bar, col, label, id);
        } else {
            lang(TS_TEXT, label);
            ts_greyed_button(bar, col, 3, id);
        }
    };
    if !offer.is_null() {
        let bar_weights: [u8; 3] = [2, 1, 1];
        let bar = tel26_add_area(d, [100, 0x22b, 0x316, 0x24e, 3, bar_weights.as_ptr() as u32, 1, 0, 1, b, u32::MAX]);
        ts_bar_button(bar, 0, TS_BACK, 5);
        if loan == 0 {
            ts_bar_button(bar, 1, 0xa9e3c4, 0xc); // "Reject"
            if terms.same_as(0x10) {
                button(bar, 2, 0xa9e3bc, 0xb, can_end); // "Accept"
            } else {
                button(bar, 2, 0xa6943c, 0xa, can_end); // "Negotiate"
            }
        } else {
            ts_bar_button(bar, 1, TS_CANCEL, 0xf);
            button(bar, 2, 0x98e6e8, 6, can_end && wage_ok); // "Offer"
        }
    } else {
        let bar_weights: [u8; 2] = [3, 1];
        let bar = tel26_add_area(d, [100, 0x22b, 0x316, 0x24e, 2, bar_weights.as_ptr() as u32, 1, 0, 1, b, u32::MAX]);
        ts_bar_button(bar, 0, TS_BACK, 5);
        if can_end && wage_ok {
            ts_bar_button(bar, 1, 0x98e6e8, 6); // "Offer"
        } else {
            lang(TS_TEXT, 0x98e6e8);
            ts_at(bar, 1, 0, 0x30, grey, 0x2c, G_GUI_COLOUR_PALE_GREY, TS_TEXT);
        }
    }
}

/// `ScreenManager::add_button_item` (0x804030) on the screen manager: 12 fields; returns eax (the item).
pub(crate) unsafe fn tel26_add_button(f: [u32; 12]) -> u32 {
    type AddButton = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: AddButton = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    g(G_SCREEN_MANAGER, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10], f[11])
}

/// The human manager of the current screen slot.
pub(crate) unsafe fn tel26_slot_manager() -> Staff {
    Staff(read_usize((G_SCREEN_RECORDS_MANAGER as isize + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize * 0x300) as usize))
}

/// A greyed button of the loan bid screen: a `Display::add_item` of flags 6 (clickable, disabled) in column `col` of
/// `area` with the text in `TS_TEXT`, grey on grey (style 0x2c), font `font`, id `id`. The original pushes the grey
/// word with whatever the stack slot's high half held; the callee reads the word.
pub(crate) unsafe fn ts_greyed_button(area: u32, col: u32, font: u32, id: u32) {
    let grey = ts_colour(G_GUI_COLOUR_GREY);
    tel26_add_item(current_human_display(), [6, 0, 0, 0, 0, col, 0, 0x30, grey, grey, 0x2c, font, ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT as u32, 0, id, 0, area]);
}

/// `TRANSFER_OFFER::TRANSFER_OFFER(player, club, 0, -1, 0)` (0x8e56a0, thiscall ret 0x14).
pub(crate) const TS_FN_NEW_TRANSFER_OFFER: usize = 0x8e56a0;

/// `TRANSFER_OFFER::has_same_terms(other)` (0x8e7f70, thiscall ret 4): nonzero when both offers have the same terms.
pub(crate) const TS_FN_HAS_SAME_TERMS: usize = 0x8e7f70;

/// `TRANSFER_MANAGER::get_negotiation_slot(index)` (0x8c6200, thiscall ret 4).
pub(crate) const TS_FN_GET_NEGOTIATION_SLOT: usize = 0x8c6200;

/// `HUMAN_MANAGER::get_bid_offer_slot(0)` (0x5fda30, thiscall on [G_HUMAN_MANAGER] ret 4): the human's bid being edited.
pub(crate) const TS_FN_BID_OFFER_SLOT: usize = 0x5fda30;

/// `Cash::set_from_amount_table(index, table, count)` (0x43f720, thiscall ret 0xc).
pub(crate) const TS_FN_CASH_FROM_TABLE: usize = 0x43f720;

/// `FOG_OF_WAR_MAP::get_knowledge_level(staff)` (0x5b02f0, thiscall ret 4).
pub(crate) const TS_FN_KNOWLEDGE_LEVEL: usize = 0x5b02f0;

/// The colour of the popup lists.
pub(crate) const TS_COLOUR_POPUP: usize = 0xae31aa;

/// "-", "+", "=".
pub(crate) const TS_MINUS: usize = 0x98ba0c;

pub(crate) const TS_PLUS: usize = 0x9afe7c;

pub(crate) const TS_EQUALS: usize = 0x9afe78;

/// A club's manager pointer (the fog of war map is per manager) [inference].
pub(crate) const TS_CLUB_MANAGER: usize = 0xcf;

/// The fee steps of the fee and clause popups.
pub(crate) const TS_FEE_STEPS: [u32; 10] = [0, 100000, 250000, 500000, 1000000, 2500000, 5000000, 10000000, 15000000, 20000000];

/// `TRANSFER_OFFER::TRANSFER_OFFER()` (0x8e5680, thiscall, no arguments).
pub(crate) const TS_FN_NEW_EMPTY_TRANSFER_OFFER: usize = 0x8e5680;

/// `Cash::step_down(mode)` (0x43f580, thiscall ret 4) and `Cash::set_from_display(amount)` (0x43f4c0, thiscall, an
/// i64 by value).
pub(crate) const TS_FN_CASH_STEP_DOWN: usize = 0x43f580;

pub(crate) const TS_FN_CASH_SET_FROM_DISPLAY: usize = 0x43f4c0;

/// `transfer_manager_selling_club_must_accept(offer, bid, out reason)` (0x8cd240, thiscall on the transfer manager).
pub(crate) const TS_FN_SELLING_CLUB_MUST_ACCEPT: usize = 0x8cd240;

/// `transfer_manager_get_squad_status_reason(offer)` (0x8c54c0, thiscall on the transfer manager): the reason shown
/// first in the reject dialog.
pub(crate) const TS_FN_SQUAD_STATUS_REASON: usize = 0x8c54c0;

/// `open_add_part_exchange_player_screen(offer)` (0x47c0b0, cdecl).
pub(crate) const TS_FN_OPEN_ADD_EXCHANGE_PLAYER: usize = 0x47c0b0;

/// The leave function of the bid screen.
pub(crate) const TS_BID_SCREEN_LEAVE: usize = 0x93a710;

/// transfer_screens.cpp's path, for the assert box.
pub(crate) const TS_CPP: usize = 0xa9e98c;

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

/// `News::find_item_by_id(id_a, id_b, out)` (0x786b70, thiscall on [G_NEWS], ret 0xc): copies the news item into `out`.
pub(crate) const TS_FN_NEWS_FIND_ITEM: usize = 0x786b70;

/// `transfer_screens_open_bid_screen_for_player(player, offer slot or 0)` (0x8e9350, cdecl).
pub(crate) const TS_FN_OPEN_BID_SCREEN_FOR_PLAYER: usize = 0x8e9350;

/// `contract_screens_offer_contract_to_staff(staff, offer slot, 1, 0)` (0x4dd0f0, cdecl).
pub(crate) const TS_FN_OFFER_CONTRACT_TO_STAFF: usize = 0x4dd0f0;

/// The bid screen (`transfer_screens_open_bid_screen_for_offer`, 0x8e9550, cdecl(slot, open)).
pub(crate) const TS_FN_OPEN_BID_SCREEN_FOR_OFFER: usize = 0x8e9550;

/// The screens a news button opens: the offer changed screen and the delay request dialog (build, input), the loan bid
/// screen (build, input).
pub(crate) const TS_OFFER_CHANGED_BUILD: usize = 0x8f24c0;

pub(crate) const TS_OFFER_CHANGED_INPUT: usize = 0x8f2800;

pub(crate) const TS_DELAY_REQUEST_BUILD: usize = 0x8f2010;

pub(crate) const TS_DELAY_REQUEST_INPUT: usize = 0x8f22c0;

pub(crate) const TS_LOAN_BID_BUILD: usize = 0x8f9a20;

pub(crate) const TS_LOAN_BID_INPUT: usize = 0x8fac30;

/// A contract offer's (`contract_manager_find_contract_offer`) fields read by the loan bid button: the wage offered
/// (+0xc, like the contract's), the expiry date (+0x2d, its year word at +0x2f) and the two cup clause bytes (+0x1c,
/// +0x1d) [inference: the loan bid screen's params 0xc..0x14].
pub(crate) const TS_CONTRACT_OFFER_EXPIRY: usize = 0x2d;

pub(crate) const TS_CONTRACT_OFFER_EXPIRY_YEAR: usize = 0x2f;

pub(crate) const TS_CONTRACT_OFFER_CLAUSE_A: usize = 0x1c;

pub(crate) const TS_CONTRACT_OFFER_CLAUSE_B: usize = 0x1d;

pub(crate) const TS_WAGE: usize = 0xc;

/// `gui_add_text_area` (0x5ed070) is variadic: after the text come (link text, link id, object) triples, ended by 0.
pub(crate) type TsTextAreaLinks = unsafe extern "C" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, usize, ...);

/// The final confirmation screen opener (`transfer_screens_open_confirm_transfer_screen`, 0x8f2e10, cdecl(slot,
/// id_a, id_b)).
pub(crate) const TS_FN_OPEN_CONFIRM_TRANSFER_SCREEN: usize = 0x8f2e10;

/// `HUMAN_MANAGER::get_confirm_offer_slot(0)` (0x5fdb30, thiscall on [G_HUMAN_MANAGER] ret 4): the slot of the copy of
/// the offer being confirmed.
pub(crate) const TS_FN_CONFIRM_OFFER_SLOT: usize = 0x5fdb30;

/// `SCOUT_MANAGER::set_compare_player(staff, 0, which)` (0x7f7780, thiscall on [G_SCOUT_MANAGER] ret 0xc).
pub(crate) const TS_FN_SET_COMPARE_PLAYER: usize = 0x7f7780;

/// `SCOUT_MANAGER::remove_from_shortlist(staff, 0)` (0x7f7a20, thiscall on [G_SCOUT_MANAGER] ret 8).
pub(crate) const TS_FN_REMOVE_FROM_SHORTLIST: usize = 0x7f7a20;

/// "Cancel" and "Delay".
pub(crate) const TS_CANCEL: usize = 0x98a1a0;

pub(crate) const TS_DELAY: usize = 0xa9e3b4;

/// `TRANSFER_MANAGER::get_past_transfers(month from, year from, month to, year to, &list, &count)` (0x8cc030, thiscall
/// ret 0x18): a malloc'd array of 0x25-byte records.
pub(crate) const TS_FN_GET_PAST_TRANSFERS: usize = 0x8cc030;

/// `transfer_manager_compare_past_transfers_by_fee` (0x8e54f0), the qsort comparer.
pub(crate) const TS_PAST_TRANSFERS_BY_FEE: usize = 0x8e54f0;

/// `date_get_month_name(month)` (0x54d3b0, cdecl): the month's name.
pub(crate) const TS_FN_MONTH_NAME: usize = 0x54d3b0;

/// `background_set_nation_picture(nation, 0, 0)` (0x41c100, cdecl).
pub(crate) const TS_FN_NATION_PICTURE: usize = 0x41c100;

/// `Date::get_short_weekday_date_string_no_year(buffer, size, 0xff)` (0x54b550, thiscall ret 0xc).
pub(crate) const TS_FN_SHORT_DATE: usize = 0x54b550;

/// `gui_add_page_buttons(x, y, page, pages, id first, id previous, id next, id last)` (0x5ede20, cdecl).
pub(crate) const TS_FN_PAGE_BUTTONS: usize = 0x5ede20;

/// The size of a past transfer record (`past_transfer`): the date is its last field.
pub(crate) const TS_PAST_TRANSFER_SIZE: usize = 0x25;

/// The network game flag (`[0xb77c84]`, as in match_screens.rs).
pub(crate) const TS_NETWORK_GAME: usize = 0xb77c84;

/// The asking price ceiling of the "+" button (30,000,000).
pub(crate) const TS_ASKING_PRICE_CAP: i32 = 0x1c9c380;

/// `shortlist_manager_squad_status_text(status, buffer, size)` (0x84b330, thiscall on [TM+0x856] ret 0xc).
pub(crate) const TS_FN_SQUAD_STATUS_TEXT: usize = 0x84b330;

/// `shortlist_manager_squad_status_allowed(staff, status)` (0x84b4d0, thiscall on [TM+0x856] ret 8).
pub(crate) const TS_FN_SQUAD_STATUS_ALLOWED: usize = 0x84b4d0;

/// The shortlist manager's list calls (thiscall on [TM+0x856]): `transfer_list_player(staff, 1)` (0x842320),
/// `remove_from_transfer_list(staff)` (0x844280), `add_to_loan_list(staff)` (0x8442b0), `remove_from_loan_list(staff)`
/// (0x8442d0).
pub(crate) const TS_FN_TRANSFER_LIST_PLAYER: usize = 0x842320;

pub(crate) const TS_FN_REMOVE_FROM_TRANSFER_LIST: usize = 0x844280;

pub(crate) const TS_FN_ADD_TO_LOAN_LIST: usize = 0x8442b0;

pub(crate) const TS_FN_REMOVE_FROM_LOAN_LIST: usize = 0x8442d0;

/// The contract's transfer status bits (`contract::TRANSFER_STATUS`): 2 cleared when listed by request, 4 automatic
/// asking price, 8 listed by request.
pub(crate) const TS_STATUS_BIT_2: u8 = 2;

pub(crate) const TS_STATUS_AUTO_PRICE: u8 = 4;

pub(crate) const TS_STATUS_BY_REQUEST: u8 = 8;

/// The orange colour word (the "(<wage>)" text of the wage contribution).
pub(crate) const TS_COLOUR_ORANGE: usize = 0xaebdd4;

/// `Date::get_short_month_date_string(out, size, style)` (0x54b2f0, thiscall, ret 0xc).
pub(crate) const TS_FN_SHORT_MONTH_DATE: usize = 0x54b2f0;

/// `STAFF_CONTRACT::format_wage_text(out, style, flags)` (0x85e750, thiscall, ret 0xc).
pub(crate) const TS_FN_WAGE_TEXT: usize = 0x85e750;

/// `confirm_push(title, heading, text, buttons, colour a, colour b, code)` (0x591840, cdecl).
pub(crate) const TS_FN_CONFIRM_PUSH: usize = 0x591840;

/// The loan bid screen's terms as read at the start of the build and the handler (params 0xb..0xf).
pub(crate) struct TsLoanTerms {
    pub(crate) share: i8,
    pub(crate) month: i8,
    pub(crate) year: u32,
    pub(crate) cup: u32,
    pub(crate) parent: u32,
}

impl TsLoanTerms {
    /// Whether the terms equal the five params from `base` (0x10 the contract offer's, 0x15 the negotiated ones): the
    /// share sign-extended, the month sign-extended against the param's low word, the year words, the two dwords.
    pub(crate) unsafe fn same_as(&self, base: u32) -> bool {
        self.share as i32 == get_screen_param(base) as i32
            && self.month as i32 == (get_screen_param(base + 1) & 0xffff) as i32
            && self.year as u16 == get_screen_param(base + 2) as u16
            && self.cup == get_screen_param(base + 3) as u32
            && self.parent == get_screen_param(base + 4) as u32
    }
}
