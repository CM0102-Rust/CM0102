// ---------- batch %%LEVA%%: the staff transfer status screen (transfer_screens.cpp, part 2: tel18_3) ----------

/// The network game flag (`[0xb77c84]`, as in match_screens.rs).
const TS_NETWORK_GAME: usize = 0xb77c84;
/// The asking price ceiling of the "+" button (30,000,000).
const TS_ASKING_PRICE_CAP: i32 = 0x1c9c380;
/// `shortlist_manager_squad_status_text(status, buffer, size)` (0x84b330, thiscall on [TM+0x856] ret 0xc).
const TS_FN_SQUAD_STATUS_TEXT: usize = 0x84b330;
/// `shortlist_manager_squad_status_allowed(staff, status)` (0x84b4d0, thiscall on [TM+0x856] ret 8).
const TS_FN_SQUAD_STATUS_ALLOWED: usize = 0x84b4d0;
/// The shortlist manager's list calls (thiscall on [TM+0x856]): `transfer_list_player(staff, 1)` (0x842320),
/// `remove_from_transfer_list(staff)` (0x844280), `add_to_loan_list(staff)` (0x8442b0), `remove_from_loan_list(staff)`
/// (0x8442d0).
const TS_FN_TRANSFER_LIST_PLAYER: usize = 0x842320;
const TS_FN_REMOVE_FROM_TRANSFER_LIST: usize = 0x844280;
const TS_FN_ADD_TO_LOAN_LIST: usize = 0x8442b0;
const TS_FN_REMOVE_FROM_LOAN_LIST: usize = 0x8442d0;
/// The contract's transfer status bits (`contract::TRANSFER_STATUS`): 2 cleared when listed by request, 4 automatic
/// asking price, 8 listed by request.
const TS_STATUS_BIT_2: u8 = 2;
const TS_STATUS_AUTO_PRICE: u8 = 4;
const TS_STATUS_BY_REQUEST: u8 = 8;

/// The shortlist manager of the transfer manager (`[TM + 0x856]`).
unsafe fn ts_shortlist_manager() -> usize {
    read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856)
}

/// Answers the Respond button of a transfer status news item: the news item `id_a`/`id_b` (built empty, then
/// `News::find_item_by_id`) gives the player in its slot 2, and his transfer status screen opens
/// (`transfer_screens_open_transfer_status_screen(player, id a, id b, 1)`).
/// Original: `transfer_screens_open_transfer_status_from_news` (0x8f66d0, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_transfer_status_from_news(id_a: u32, id_b: u32) {
    covered!("tel18_3 transfer status from news");
    let find: unsafe extern "thiscall" fn(usize, u32, u32, usize) -> usize = core::mem::transmute(0x786b70usize);
    let open: unsafe extern "C" fn(usize, u32, u32, u32) = core::mem::transmute(0x8f6770usize);
    let mut item = [0u8; 0xe0];
    let n = item.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0, 0);
    find(read_usize(G_NEWS), id_a, id_b, n);
    let player = staff_at(exe::newsitem_get_slot(n, 2) as i32 as isize).addr();
    open(player, id_a, id_b, 1);
}

/// Comparer of `transfer_screens_open_transfer_status_from_news`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_transfer_status_from_news(id_a: u32, id_b: u32) {
    original_fn::<unsafe extern "C" fn(u32, u32)>(%%I0%%)(id_a, id_b)
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
    covered!("tel18_3 open transfer status screen");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8f6a10usize, 0x8f7640usize, 1, 0usize, 0) == 0 {
        return;
    }
    let contract = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, read_usize(tel26_slot_manager().addr() + staff::CLUB_JOB));
    set_screen_param(0, read_u32(player) as usize);
    set_screen_param(1, read_u32(player + staff::VALUE) as usize);
    set_screen_param(2, 0);
    set_screen_param(7, a as usize);
    set_screen_param(8, b as usize);
    set_screen_param(9, c as usize);
    let club = read_usize(tel26_slot_manager().addr() + staff::CLUB_JOB);
    set_screen_param(0xb, club);
    let flagged = club != 0 && read_u8(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club) + 0x165) == 1;
    set_screen_param(0xc, flagged as usize);
    if contract != 0 {
        let status = read_u8(contract + contract::TRANSFER_STATUS);
        let by_request = ((status & TS_STATUS_BY_REQUEST) >> 3) as usize;
        let grievances = read_u32(contract + contract::GRIEVANCES);
        let listed = exe::staff_is_transfer_listed(player, 0usize);
        let loan = exe::staff_is_loan_listed(player, 0usize);
        set_screen_param(4, listed as usize);
        set_screen_param(5, loan as usize);
        set_screen_param(6, by_request);
        set_screen_param(0xd, by_request);
        set_screen_param(0xe, (grievances != 0) as usize);
        set_screen_param(3, (read_u8(contract + contract::SQUAD_STATUS) >> 4) as usize);
        set_screen_param(0xa, ((read_u8(contract + contract::TRANSFER_STATUS) & TS_STATUS_AUTO_PRICE) >> 2) as usize);
    } else {
        set_screen_param(3, 7);
        for k in [4u32, 5, 6, 0xd, 0xe, 0xa] {
            set_screen_param(k, 0);
        }
    }
    if read_usize(player + staff::PLAYER) != 0 {
        let club = read_usize(tel26_slot_manager().addr() + staff::CLUB_JOB);
        set_screen_param(0xf, exe::contract_manager_calc_player_value(read_usize(G_CONTRACT_MANAGER), player, club, 0, -1, 1) as u32 as usize);
    } else {
        set_screen_param(0xf, 0);
    }
}

/// Comparer of `transfer_screens_open_transfer_status_screen`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_transfer_status_screen(player: usize, a: u32, b: u32, c: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32, u32, u32)>(%%I1%%)(player, a, b, c)
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
    let text_new: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_TEXT_TEXT);
    let text_set: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(FN_TEXT_SET_STRING);
    let text_link: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> i32 = core::mem::transmute(FN_TEXT_ADD_LINK);
    let text_drop: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TEXT_DESTRUCTOR_TEXT);
    let draw: unsafe extern "thiscall" fn(usize, u32, i32, i32, *mut i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(TS_FN_TEXT_DRAW_WRAPPED);
    let status_text: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(TS_FN_SQUAD_STATUS_TEXT);
    let allowed: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(TS_FN_SQUAD_STATUS_ALLOWED);
    let from_table: unsafe extern "thiscall" fn(*mut u32, u32, *const u32, u32) = core::mem::transmute(TS_FN_CASH_FROM_TABLE);
    let weights7: [u8; 7] = [2, 0x14, 2, 0xa, 2, 4, 3];
    let weights4: [u8; 4] = [3, 1, 2, 0x11];
    let mut text = [0u32; 8];
    let t = text.as_mut_ptr() as usize;
    text_new(t, 0);
    let mut row: i32;
    let player = staff_at(get_screen_param(0) as i32 as isize).addr();
    let asking = get_screen_param(1) as i32;
    let squad = get_screen_param(3) as u8;
    let news_b = get_screen_param(8) as i32;
    let auto = get_screen_param(0xa);
    let club = get_screen_param(0xb);
    get_screen_param(0xc);
    let flagged = get_screen_param(0xc);
    let cap = get_screen_param(0xf) as i32;
    let contract = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, club);
    if contract == 0 {
        covered!("tel18_3 transfer status without a contract");
        tel26_error_box(TS_CPP, 0x1d15);
        text_drop(t);
        return;
    }
    covered!("tel18_3 transfer status screen");
    let kind = ((read_u8(contract + contract::CONTRACT_TYPE) << 2) as i8) >> 2;
    picture(player, 0, 0);
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
            exe::cash_format_money(m, 0x1e, ts_display(read_u32(player + staff::VALUE), 6), 6);
            lang(TS_TEXT, 0xaa0cec, n, m);
            text_link(t, m, 1, 0);
        }
        text_set(t, TS_TEXT);
        text_link(t, n, 0x7d7, player);
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
        if allowed(sm, player, k) == 0 {
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
        if read_u8(player + staff::JOB_FOR_CLUB) != 0xf {
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
    text_drop(t);
}

/// Comparer of `transfer_screens_build_transfer_status_screen`: side by side over the display list and the params
/// (the Text is the frame's own); without the contract only the original runs (the assert box).
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_transfer_status_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I2%%);
    let player = staff_at(get_screen_param(0) as i32 as isize).addr();
    if exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, get_screen_param(0xb)) == 0 {
        return o();
    }
    compare_by_display_list_with("Transfer status screen", &[], || o(), || transfer_screens_build_transfer_status_screen());
}

/// The three widgets after the "-" of the transfer status screen once the asking price changed (`price`): "-" (param
/// 2) enabled (id 2) above 0, else greyed (flags 6, no data); the price's text; "+" enabled (id 3) when `plus_on`, else
/// greyed (flags 6 and `plus_off`); the Auto widget back to style 0x30; the screen redrawn.
unsafe fn ts_asking_price_widgets(price: i32, plus_on: bool, plus_off: impl Fn(u32)) {
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
            let contract = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club);
            let old_squad = read_u8(contract + contract::SQUAD_STATUS) >> 4;
            let news_b = get_screen_param(8);
            let old_status = read_u8(contract + contract::TRANSFER_STATUS);
            let mut done: u32 = 0;
            let with_news = news_a != 0 && news_b as i32 > -1;
            if with_news {
                ts_news_slot(news_a, news_b, 6, 1);
                ts_news_slot(news_a, news_b, 7, squad as i8 as i32 as u32);
                ts_news_slot(news_a, news_b, 9, old_status as i8 as i32 as u32);
            }
            write_u8(contract + contract::SQUAD_STATUS, (read_u8(contract + contract::SQUAD_STATUS) & 0xf) | (old_squad << 4));
            let set: unsafe extern "thiscall" fn(usize, usize, usize, i32, *mut u32, u32, u32, u32) = core::mem::transmute(TS_FN_SET_PLAYER_SQUAD_STATUS);
            set(read_usize(G_TRANSFER_MANAGER), p, club, asking, &mut done, squad as u32, old_status as u32, 0);
            if with_news {
                ts_news_slot(news_a, news_b, 0xa, done);
            }
            if read_u8(contract + contract::CONTRACT_TYPE) & 0x3f == 6 {
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
            let at = contract + contract::TRANSFER_STATUS;
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
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I3%%);
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
