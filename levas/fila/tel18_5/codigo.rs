// ---------- batch %%LEVA%%: the loan bid screen (transfer_screens.cpp, part 2: tel18_5) ----------

/// The orange colour word (the "(<wage>)" text of the wage contribution).
const TS_COLOUR_ORANGE: usize = 0xaebdd4;
/// `Date::get_short_month_date_string(out, size, style)` (0x54b2f0, thiscall, ret 0xc).
const TS_FN_SHORT_MONTH_DATE: usize = 0x54b2f0;
/// `STAFF_CONTRACT::format_wage_text(out, style, flags)` (0x85e750, thiscall, ret 0xc).
const TS_FN_WAGE_TEXT: usize = 0x85e750;
/// `confirm_push(title, heading, text, buttons, colour a, colour b, code)` (0x591840, cdecl).
const TS_FN_CONFIRM_PUSH: usize = 0x591840;

/// The wage share of a loan's terms: the offered wage over the contract's in percent (the quotient's byte), its
/// tenth, times 10 as a byte (`imul dl`), sign-extended. No rounding.
fn ts_loan_wage_share(offered: i32, wage: i32) -> i32 {
    let percent = offered.wrapping_mul(100) / wage;
    (((percent as i8 as i32) / 10) as u8).wrapping_mul(10) as i8 as i32
}

/// Five params from `base` set from a loan's terms (a contract offer or a negotiated loan contract): the wage share,
/// the month of the expiry date (`Date::get_month`, sign-extended), its year word, the cup matches byte and the
/// matches against the parent club byte (sign-extended).
unsafe fn ts_set_loan_terms(terms: usize, share: i32, base: u32) {
    set_screen_param(base, share as u32 as usize);
    set_screen_param(base + 1, exe::date_get_month(terms + TS_CONTRACT_OFFER_EXPIRY) as i8 as i32 as usize);
    set_screen_param(base + 2, read_u16(terms + TS_CONTRACT_OFFER_EXPIRY_YEAR) as usize);
    set_screen_param(base + 3, read_i8(terms + TS_CONTRACT_OFFER_CLAUSE_A) as i32 as usize);
    set_screen_param(base + 4, read_i8(terms + TS_CONTRACT_OFFER_CLAUSE_B) as i32 as usize);
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
    let o = read_usize(slot);
    let status = read_u8(o + offer::STATUS);
    if read_u32(o + offer::SERIAL) != serial || status == 3 || status == 2 {
        covered!("tel18_5 loan bid from news: offer changed");
        ts_open_offer_changed(o, serial);
        return;
    }
    let player = staff_at(read_i32(o + offer::TARGET_STAFF_ID) as isize).addr();
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
    let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
    let share = ts_loan_wage_share(read_i32(loan + TS_WAGE), read_i32(contract + contract::WAGE));
    ts_set_loan_terms(loan, share, 0xb);
    ts_set_loan_terms(loan, share, 0x15);
}

/// Comparer of `transfer_screens_open_loan_bid_from_news`: only the original runs (it opens a screen or withdraws the
/// offer).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_loan_bid_from_news(id_a: u32, id_b: u32) {
    original_fn::<unsafe extern "C" fn(u32, u32)>(%%I0%%)(id_a, id_b)
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
    let manager = tel26_slot_manager().addr();
    let club = || read_usize(manager + staff::CLUB_JOB);
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
    let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
    set_screen_param(1, read_u32(contract + contract::WAGE) as usize);
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
    let terms = exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), read_u32(player), read_u32(club()));
    if terms != 0 && read_u8(terms + contract::CONTRACT_TYPE) & 0x3f == 6 {
        covered!("tel18_5 open loan bid screen with a loan contract offer");
        let share = ts_loan_wage_share(read_i32(terms + TS_WAGE), read_i32(contract + contract::WAGE));
        ts_set_loan_terms(terms, share, 0xb);
        ts_set_loan_terms(terms, share, 0x10);
    }
    1
}

/// Comparer of `transfer_screens_open_loan_bid_screen`: only the original runs (it opens a screen or a box).
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_loan_bid_screen(player: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize) -> i32>(%%I1%%)(player)
}

/// A greyed button of the loan bid screen: a `Display::add_item` of flags 6 (clickable, disabled) in column `col` of
/// `area` with the text in `TS_TEXT`, grey on grey (style 0x2c), font `font`, id `id`. The original pushes the grey
/// word with whatever the stack slot's high half held; the callee reads the word.
unsafe fn ts_greyed_button(area: u32, col: u32, font: u32, id: u32) {
    let grey = ts_colour(G_GUI_COLOUR_GREY);
    tel26_add_item(current_human_display(), [6, 0, 0, 0, 0, col, 0, 0x30, grey, grey, 0x2c, font, ts_colour(G_GUI_COLOUR_PALE_GREY), TS_TEXT as u32, 0, id, 0, area]);
}

/// The loan bid screen's terms as read at the start of the build and the handler (params 0xb..0xf).
struct TsLoanTerms {
    share: i8,
    month: i8,
    year: u32,
    cup: u32,
    parent: u32,
}

impl TsLoanTerms {
    /// Whether the terms equal the five params from `base` (0x10 the contract offer's, 0x15 the negotiated ones): the
    /// share sign-extended, the month sign-extended against the param's low word, the year words, the two dwords.
    unsafe fn same_as(&self, base: u32) -> bool {
        self.share as i32 == get_screen_param(base) as i32
            && self.month as i32 == (get_screen_param(base + 1) & 0xffff) as i32
            && self.year as u16 == get_screen_param(base + 2) as u16
            && self.cup == get_screen_param(base + 3) as u32
            && self.parent == get_screen_param(base + 4) as u32
    }
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

/// The body of `transfer_screens_build_loan_bid_screen` with its Text `t` and local STAFF_CONTRACT `c`.
unsafe fn ts_build_loan_bid(t: usize, c: usize) {
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
    let contract_club = ts_contract_club_any(player);
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
    let offer = get_screen_param(7);
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
    let manager = tel26_slot_manager().addr();
    if manager == 0 {
        tel26_error_box(TS_CPP, 0x2406);
        return;
    }
    let club = read_usize(manager + staff::CLUB_JOB);
    if club == 0 {
        tel26_error_box(TS_CPP, 0x240d);
        return;
    }
    covered!("tel18_5 loan bid screen");
    picture(player, 0, 0);
    let (mut a, mut b) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut a, &mut b);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, b, a, club + club::SHORT_NAME);
    let mut name = [0u8; 0x190];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(player, n, 0x190, 5);
    if offer != 0 && read_i32(offer + offer::BUYER_CLUB_ID) != read_i32(club) {
        let buyer = club_at(read_i32(offer + offer::BUYER_CLUB_ID) as isize).addr();
        let g = read_i8(buyer + club::SHORT_GENDER_NAME) as i32;
        lang(TS_TEXT, 0xaa1364, g, g, buyer + club::SHORT_NAME, n); // "<club> loan bid for <player>"
    } else {
        lang(TS_TEXT, 0xaa1334, n); // "Loan bid for <player>"
    }
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    let d = current_human_display();
    let grey = ts_colour(G_GUI_COLOUR_GREY);
    let pale = ts_colour(G_GUI_COLOUR_PALE_GREY);
    let dark = ts_colour(G_GUI_COLOUR_DARK_BLUE);
    let mut y: u32 = 0x8c;
    if offer != 0 {
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
    write_i32(c + contract::WAGE, wage);
    wage_text(c, w, 2, 2);
    exe::database_format_staff_name(player, n, 0x190, 1);
    let g = read_i8(contract_club + club::SHORT_GENDER_NAME) as i32;
    let club_name = contract_club + club::SHORT_NAME;
    lang(TS_TEXT, 0xaa12a4, g, g, n, w, club_name); // "<player> is currently on a <wage> contract with <club>."
    exe::text_set_string(t, TS_TEXT as *const u8);
    exe::text_add_link(t, n as *const u8, 0x7d7, player);
    exe::text_add_link(t, w as *const u8, 7, wage as u32 as usize);
    exe::text_add_link(t, club_name as *const u8, 0x7d5, contract_club);
    if club != ts_contract_club_any(player) {
        covered!("tel18_5 loan bid screen: board wage limit");
        write_i32(c + contract::WAGE, max_wage);
        wage_text(c, w, 2, 2);
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
    write_i32(c + contract::WAGE, contribution);
    wage_text(c, w, 0, 1);
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
    let g = read_i8(contract_club + club::SHORT_GENDER_NAME) as i32;
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
    if offer != 0 {
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

/// Comparer of `transfer_screens_build_loan_bid_screen`: side by side over the display list, the params and the
/// contract lookup global; without the player, the manager or his club only the original runs (the assert box).
pub(crate) unsafe extern "C" fn compare_transfer_screens_build_loan_bid_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I2%%);
    let manager = tel26_slot_manager().addr();
    if get_screen_param(0) == 0 || manager == 0 || read_usize(manager + staff::CLUB_JOB) == 0 {
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
    let offer = get_screen_param(7);
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
            if read_u32(offer + offer::SERIAL) != serial || read_u8(offer + offer::STATUS) == 3 {
                covered!("tel18_5 loan bid: offer changed");
                ts_open_offer_changed(offer, serial);
                return -11;
            }
            if id == 0xa {
                covered!("tel18_5 loan bid Negotiate");
                transfer_screens_open_confirm_loan_offer(player, month as u32, year, share as u32, cup, parent, offer);
            } else {
                covered!("tel18_5 loan bid Accept or Reject");
                let slot = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), read_i32(offer + offer::SLOT_INDEX));
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
            let slot = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), read_i32(offer + offer::SLOT_INDEX));
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
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I3%%);
    ts_compare_answer("Loan bid screen", item, &[1, 2, 3, 4, 5, 7, 8, 9, 0xd, 0xe], o, transfer_screens_handle_loan_bid_screen)
}
