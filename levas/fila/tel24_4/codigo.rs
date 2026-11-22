// ---------- batch %%LEVA%%: the contract screens (4): the basic terms tab and its answers (contract_screens.cpp, tel24_4) ----------

/// `STAFF_CONTRACT::format_wage_text(buffer, mode, style)` (0x85e750, thiscall on the terms, ret 0xc).
const CS_FN_FORMAT_WAGE_TEXT: usize = 0x85e750;
/// `STAFF_CONTRACT::format_expiry_text(buffer, style)` (0x85fa30, thiscall on the terms, ret 8).
const CS_FN_FORMAT_EXPIRY_TEXT: usize = 0x85fa30;
/// `STAFF_CONTRACT::increase_wage_step()` and `decrease_wage_step()` (0x867b70, 0x867c50, thiscall on the terms, ret 0).
const CS_FN_INCREASE_WAGE_STEP: usize = 0x867b70;
const CS_FN_DECREASE_WAGE_STEP: usize = 0x867c50;
/// `transfer_manager_build_refusal_reasons_text(club, his club, manager, his manager, x, interest, buffer)` (0x8e4ae0,
/// thiscall on the transfer manager, ret 0x20; the interest a double): whether there are reasons, written to `buffer`.
const CS_FN_REFUSAL_REASONS: usize = 0x8e4ae0;
/// `Date::get_short_date_string(buffer, size, flags)` (0x54a650, thiscall on the date, ret 0xc).
const CS_FN_SHORT_DATE: usize = 0x54a650;
/// `date_get_short_month_name(month)` (0x54d210, cdecl; only the low byte of the month is read): "Jan".."Dec".
const CS_FN_SHORT_MONTH_NAME: usize = 0x54d210;
/// The doubles 2500.0 and the float 0.001 of the basic terms tab.
const CS_F64_2500: usize = 0x9690b0;
const CS_F32_0_001: usize = 0x9690c0;
/// The answers of the basic terms tab that only change the human's terms or the screen params (the wage and fee
/// back to the request, a popup step, the expiry back to the request, the public offer, the job, part time, the
/// transfer date): side by side in the comparer.
const CS_BASIC_PARAM_ANSWERS: [u16; 10] = [6, 7, 8, 0xb, 0x16, 0x19, 0x1a, 0x1c, 0x1e, 0x30];

/// `STAFF_CONTRACT::format_wage_text(buffer, mode, style)` of contract terms.
unsafe fn cs_format_wage(terms: usize, dst: usize, mode: u32, style: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(CS_FN_FORMAT_WAGE_TEXT);
    f(terms, dst, mode, style);
}

/// `STAFF_CONTRACT::format_expiry_text(buffer, style)` of contract terms.
unsafe fn cs_format_expiry(terms: usize, dst: usize, style: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(CS_FN_FORMAT_EXPIRY_TEXT);
    f(terms, dst, style);
}

/// The short name of a month ("Aug"), by its low byte.
unsafe fn cs_month_name(month: i8) -> usize {
    let f: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(CS_FN_SHORT_MONTH_NAME);
    f(month as u8 as u32)
}

/// `Cash::set_from_amount_table(index, table, count)` on a cash dword, returning the amount it stores.
unsafe fn cs_cash_step_from(cash: *mut u32, index: u32, table: *const i32, count: u32) -> u32 {
    let f: unsafe extern "thiscall" fn(*mut u32, u32, *const i32, u32) -> u32 = core::mem::transmute(CS_FN_CASH_FROM_TABLE);
    f(cash, index, table, count)
}

/// A money value formatted into `TS_TEXT` with the size 200 in `mode`, again in `short` when it takes more than 8
/// characters (the answers of the tab pass 0xc8 where the builders pass 2000).
unsafe fn cs_money_text_200(value: u32, mode: u32, short: u32) {
    let cash = value;
    exe::cash_format_money(TS_TEXT, 0xc8, exe::cash_get_display_amount(&cash, mode), mode);
    if c_string_len(TS_TEXT) > 8 {
        exe::cash_format_money(TS_TEXT, 0xc8, exe::cash_get_display_amount(&cash, short), short);
    }
}

/// A widget of the tab set to a state: flags, text flags and id (`set_widget_flags`, `set_widget_text_flags`,
/// `set_widget_id`).
unsafe fn cs_widget_state(item: u32, flags: u32, text_flags: u32, id: u32) {
    exe::screenmanager_set_widget_flags(G_SCREEN_MANAGER, item as usize, flags);
    exe::screenmanager_set_widget_text_flags(G_SCREEN_MANAGER, item, text_flags);
    exe::screenmanager_set_widget_id(G_SCREEN_MANAGER, item, id);
}

/// The same three steps as the original writes them for the wage and fee "+"/"-" neighbours: `set_widget_id` where
/// `set_widget_flags` was meant (the id is then set again), so the widget's flags stay as they were.
unsafe fn cs_widget_state_by_id(item: u32, flags: u32, text_flags: u32, id: u32) {
    exe::screenmanager_set_widget_id(G_SCREEN_MANAGER, item, flags);
    exe::screenmanager_set_widget_text_flags(G_SCREEN_MANAGER, item, text_flags);
    exe::screenmanager_set_widget_id(G_SCREEN_MANAGER, item, id);
}

/// The wage below which a young player (under 24) of the edited terms' club still expects terms: the club's
/// reputation turned into a wage by the squad status of the edited terms (0 below the bands).
///
/// Status 1: above 7250 reputation × 1.25 + 10000, above 6250 reputation + 7500, above 4750 reputation × 0.75 + 2500.
/// Status 2: above 7250 reputation + 7500, above 6250 × 0.75 + 5000, above 4750 × 0.5. Status 3: above 7250 × 0.75 +
/// 5000, above 6250 × 0.5 + 2500, above 4750 × 0.25. Status 5: above 7250 × 0.5, above 6250 × 0.25. Status 4 and the
/// rest: 0. The result is truncated (`_ftol`) and kept as a word.
unsafe fn cs_young_wage_threshold(club: usize, status: u8) -> i16 {
    let r = read_i16(club + club::REPUTATION);
    let rep = r as f64;
    let d = |a: usize| read_f64(a);
    let v = match status {
        1 if r > 0x1c52 => rep * d(rdata::F64_1_25) + d(rdata::F64_10000),
        1 if r > 0x186a => rep + d(rdata::F64_7500),
        1 if r > 0x128e => rep * d(rdata::F64_0_75) + d(CS_F64_2500),
        2 if r > 0x1c52 => rep + d(rdata::F64_7500),
        2 if r > 0x186a => rep * d(rdata::F64_0_75) + d(rdata::F64_5000),
        2 if r > 0x128e => rep * d(rdata::F64_0_5),
        3 if r > 0x1c52 => rep * d(rdata::F64_0_75) + d(rdata::F64_5000),
        3 if r > 0x186a => rep * d(rdata::F64_0_5) + d(CS_F64_2500),
        3 if r > 0x128e => rep * d(rdata::F64_0_25),
        5 if r > 0x1c52 => rep * d(rdata::F64_0_5),
        5 if r > 0x186a => rep * d(rdata::F64_0_25),
        _ => return 0,
    };
    ftol(v) as i16
}

/// The manager id of a club (`club::MANAGER`), or -1 without one.
unsafe fn cs_club_manager_id(club: usize) -> u32 {
    let m = read_usize(club + club::MANAGER);
    if m != 0 { read_u32(m) } else { u32::MAX }
}

/// Builds the basic terms tab of the offer contract screen: what the person says about the terms, the board's wage
/// limit, and the rows of the wage, signing on fee, dates, contract type, public offer and transfer date.
///
/// Params read: 0x1c the transfer window state, 0x15 the transfer budget, 3 the transfer offer slot, 8 the job, 0xd
/// the role, 0x1d the window mark, 0x18 the wage limit, 0x19 and 0x1a the two out values of the desired contract (the
/// second × 0.001 is the interest, a float), 0x1b the talk flags. The edited terms' club counts only for an id from 1
/// to the club count. One area (columns 2:0xe:2:0xe:2:2:2, as many rows as fit between `top` and `bottom`).
///
/// The person's words, wrapped: he expects terms when he asked for terms (`will_negotiate_offer`), the limit × 1.5 is
/// above the edited wage and either the interest is above 2.75, or his current club (the contract's, not of type 5)
/// is the edited terms' club with the talk flag 2, or the job is 8, 9 or 10 with an edited wage below 3200, or he is
/// under 24 with an edited wage below `cs_young_wage_threshold` + 3250. Then the edited fee (mode 6), and for a
/// month-to-month contract "expects ... weekly wage" (the part time text with the bit 0x80); otherwise the expiry text
/// and, part time, "expects a part time contract"; no fee, "expects a wage of"; an odd person id, "wants"; else
/// "is requesting" (professionalism and loyalty above 10) or "is demanding", all with the fee; the expiry is a link
/// (0xb). Otherwise, at his own club (the current club is the edited one): no contract "does not wish to negotiate",
/// unhappy (grievance 2) with an edited wage below 25000 and a current wage above 2100 "doesn't believe the club can
/// meet his demands", grievances 0x6214 "doesn't want to negotiate ... while he is unhappy", grievance 2 the same
/// doubt for role 1 or the 0x9b09c8 text, else "does not wish". Elsewhere the refusal reasons are built
/// (`build_refusal_reasons_text` with both clubs and managers, -1 for a missing current club); with the screen param
/// 0x16 above the limit × 1.5: interest from 3.15, edited wage below 175000 and not month to month, the expiry text
/// ("expects a wage of", or part time); reasons, the reasons text; interest below 0.75, the club's name text; else
/// "has no interest". Otherwise by the edited squad status 1 to 6: with reasons and interest above 1.25 the status'
/// reasons text, else its "no interest" text; any other status, the staff text for a person with a non-player sheet
/// (else the text buffer is left as it was). The name is a link (0x7da).
///
/// Then the board's limit: a local contract with the limit as wage; "will allow wages up to", or for a player under
/// the club's reputation + 750 his chairman favours "feel he will be a key signing" ("an important member" when he is
/// already at the club; the contract is looked up again into `G_CONTRACT_LOOKUP_RESULT`); the human's wage is cut to
/// the limit; the limit is a link (7). Rows: the wage label by the money option (weekly, monthly, yearly), "-" (id 5;
/// a text up to 100; its index in param 0xf), the wage (popup owner), "+" (id 4) while one step up stays within the
/// limit, "=" (id 6); the signing on fee: "-" (id 0x18, its index in param 0x10) when set, the fee (mode 6, 5 when
/// long), "+" (id 0x17) while one step up stays within the budget, "=" (id 0x16); the start date when it is after
/// today; the expiry (month and year of params 0xb/0xc, or "Month to Month"): "-" (id 0x1d one month after the start,
/// else 0xa; its index in param 0x11) unless the expiry already reaches the request of params 9/0xa or the contract is
/// month to month, "+" (id 0x1d month to month, else 9) unless a fixed contract reaches the maximum length, "=" (id
/// 0xb); the type (part or full time, id 0x1e); "Make offer public" No/Yes (id 0x1a) without a transfer offer and when
/// he is not at the edited terms' club. Popups: eight wage steps (k × limit / 8) when the first is above 100 (id 8,
/// the wage as value), eight fee steps (k × min(budget, 3 million) / 7, id 0x19) when they are more than 1000 apart.
/// With the window mark: a contract with expiry band 0, the next transfer window text (the current one ends / the
/// next opens on its long date); with another band, the move text and the "Transfer date" row (End of contract or
/// Immediate, id 0x30).
/// Original: `contract_screens_build_basic_terms_tab` (0x4e06e0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_build_basic_terms_tab(terms: usize, edited: usize, staff: usize, top: i32, bottom: i32) {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let draw: unsafe extern "thiscall" fn(usize, u32, i32, i32, *mut i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(TS_FN_TEXT_DRAW_WRAPPED);
    let text_clear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(TS_FN_TEXT_CLEAR);
    let pale = ts_colour(G_GUI_COLOUR_PALE_GREY);
    let green = job_colour(JOB_COLOUR_31AA);
    let dark = ts_colour(G_GUI_COLOUR_DARK_BLUE);
    let black = ts_colour(G_GUI_COLOUR_BLACK);
    let yellow = ts_colour(G_GUI_COLOUR_YELLOW);
    let mut row: i32 = 0;
    let widths: [u8; 7] = [2, 0xe, 2, 0xe, 2, 2, 2];
    let mut text = [0usize; 8];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0usize);
    let mut local_terms = [0u8; CS_TERMS_SIZE];
    let local = local_terms.as_mut_ptr() as usize;
    exe::new_staff_contract(local);
    let mut reasons = [0u8; 400];
    let window_state = get_screen_param(0x1c) as u8;
    let budget = get_screen_param(0x15) as i32;
    let slot = get_screen_param(3);
    let job = get_screen_param(8) as u8 as i8;
    let role = get_screen_param(0xd) as u8 as i8;
    let window = get_screen_param(0x1d);
    let limit = get_screen_param(0x18) as i32 as f64;
    let first = get_screen_param(0x19) as u32;
    let interest = (get_screen_param(0x1a) as i32 as f64 * read_f32(CS_F32_0_001) as f64) as f32;
    let flags = get_screen_param(0x1b) as u32;
    let club_id = read_i32(edited + contract::CLUB);
    let club = if club_id > 0 && club_id < read_i32(G_CLUB_COUNT) { club_at(club_id as isize).addr() } else { 0 };
    let rows = ((bottom - top) / exe::font_get_height(3)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 7, widths.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    let current = if c != 0 && cs_contract_type(c) != 5 { club_at(read_i32(c + contract::CLUB) as isize).addr() } else { 0 };
    let threshold = if club != 0 { cs_young_wage_threshold(club, cs_squad_status(edited)) } else { 0 };
    let mut name = [0u8; 2000];
    let n = name.as_mut_ptr() as usize;
    let mut wage_text = [0u8; 2000];
    let w = wage_text.as_mut_ptr() as usize;
    let mut expiry = [0u8; 200];
    let e = expiry.as_mut_ptr() as usize;
    let wage = read_i32(edited + contract::WAGE);
    let expects = cs_will_negotiate(edited) != 0 && limit * read_f64(rdata::F64_1_5) > wage as f64
        && ((interest as f64) > read_f64(rdata::F64_2_75)
            || (current == club && flags & 2 != 0)
            || (matches!(job, 8 | 10 | 9) && wage < 0xc80)
            || (read_i8(staff + staff::AGE) < 0x18 && wage < threshold as i32 + 0xcb2));
    exe::database_format_staff_name(staff, n, 0x7d0, 1);
    cs_format_wage(edited, w, 2, 2);
    if expects {
        covered!("tel24_4 basic terms he expects terms");
        let mut fee_text = [0u8; 2000];
        let ft = fee_text.as_mut_ptr() as usize;
        let cash = read_u32(edited + CS_TERMS_FEE);
        exe::cash_format_money(ft, 0x7d0, exe::cash_get_display_amount(&cash, 6), 6);
        let kind = read_u8(edited + contract::CONTRACT_TYPE);
        if kind & 0x3f == 1 {
            // "<name> expects <wage> weekly wage" (a part time variant with the bit 0x80)
            lang(TS_TEXT, if kind & 0x80 != 0 { 0x9b0e4c } else { 0x9b0de4 }, n, w);
        } else {
            cs_format_expiry(edited, e, 1);
            if kind & 0x80 != 0 {
                lang(TS_TEXT, 0x9b0d58, 100u32, n, w, e); // "expects a part time contract"
            } else if read_i32(edited + CS_TERMS_FEE) == 0 {
                lang(TS_TEXT, 0x9b0ae8, 100u32, n, w, e); // "expects a wage of"
            } else if read_i32(staff) % 2 != 0 {
                lang(TS_TEXT, 0x9b0cb8, 100u32, n, w, e, ft); // "wants"
            } else if read_i8(staff + staff::PROFESSIONALISM) > 10 && read_i8(staff + staff::LOYALTY) > 10 {
                lang(TS_TEXT, 0x9b0c08, 100u32, n, w, e, ft); // "is requesting"
            } else {
                lang(TS_TEXT, 0x9b0b58, 100u32, n, w, e, ft); // "is demanding"
            }
            exe::text_add_link(t as usize, e as *const u8, 0xb, 0usize);
        }
        exe::text_set_string(t as usize, TS_TEXT as *const u8);
        exe::text_add_link(t as usize, n as *const u8, 0x7da, staff);
        exe::text_add_link(t as usize, w as *const u8, 6, 0usize);
    } else {
        if current != 0 && current == club {
            covered!("tel24_4 basic terms at his own club");
            let own = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff);
            let fmt = if own == 0 {
                0x9b0964 // "does not wish to negotiate a new contract at the current time"
            } else {
                let g = read_u32(own + contract::GRIEVANCES);
                if g & 2 != 0 && wage < 0x61a8 && read_i32(own + contract::WAGE) > 0x834 {
                    0x9b0a94 // "doesn't believe the club can meet his demands"
                } else if g & 0x6214 != 0 {
                    0x9b0a30 // "doesn't want to negotiate a new contract while he is unhappy"
                } else if g & 2 != 0 {
                    if role == 1 { 0x9b0a94 } else { 0x9b09c8 }
                } else {
                    0x9b0964
                }
            };
            lang(TS_TEXT, fmt, n);
        } else {
            covered!("tel24_4 basic terms refusal reasons");
            let refusal: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, f64, usize) -> u8 = core::mem::transmute(CS_FN_REFUSAL_REASONS);
            let (their_club, their_manager) = if current != 0 { (read_u32(current), cs_club_manager_id(current)) } else { (u32::MAX, u32::MAX) };
            let refused = refusal(read_usize(G_TRANSFER_MANAGER), their_club, read_u32(club), their_manager, cs_club_manager_id(club), first,
                interest as f64, reasons.as_mut_ptr() as usize) != 0;
            let r = reasons.as_ptr() as usize;
            let asked_for = get_screen_param(0x16) as i32 as f64;
            if limit * read_f64(rdata::F64_1_5) < asked_for {
                let kind = read_u8(edited + contract::CONTRACT_TYPE);
                if !((interest as f64) < read_f64(rdata::F64_3_15)) && wage < 0x2ab98 && kind & 0x3f != 1 {
                    cs_format_expiry(edited, e, 1);
                    lang(TS_TEXT, if kind & 0x80 != 0 { 0x9b0d58 } else { 0x9b0ae8 }, 100u32, n, w, e);
                    exe::text_add_link(t as usize, e as *const u8, 0xb, 0usize);
                } else if refused {
                    lang(TS_TEXT, 0x9b0844, n, r);
                } else if (interest as f64) < read_f64(rdata::F64_0_75) {
                    let gender = read_i8(club + club::GENDER_NAME) as i32;
                    lang(TS_TEXT, 0x9b08f8, gender, gender, n, club + club::NAME);
                } else {
                    lang(TS_TEXT, 0x9b08ac, n); // "has no interest in negotiating terms"
                }
            } else {
                // the squad status' texts: (with reasons, without)
                const BY_STATUS: [(usize, usize); 6] = [(0x9b0844, 0x9b08ac), (0x9b0844, 0x9b08ac), (0x9b07c4, 0x9b075c),
                    (0x9b06e4, 0x9b0684), (0x9b05fc, 0x9b0590), (0x9b0514, 0x9b04b0)];
                let status = cs_squad_status(edited);
                if (1..=6).contains(&status) {
                    let (with, without) = BY_STATUS[status as usize - 1];
                    if refused && (interest as f64) > read_f64(rdata::F64_1_25) {
                        lang(TS_TEXT, with, n, r);
                    } else {
                        lang(TS_TEXT, without, n);
                    }
                } else if read_usize(staff + staff::NON_PLAYER) != 0 {
                    lang(TS_TEXT, 0x9b0450, n); // "has no interest in negotiating terms for this position"
                }
            }
        }
        exe::text_set_string(t as usize, TS_TEXT as *const u8);
        exe::text_add_link(t as usize, n as *const u8, 0x7da, staff);
    }
    draw(t as usize, area, 0x78, 0x302, &mut row, 3, pale, 0, 0);
    text_clear(t as usize);
    row += 2;
    // the board's limit
    let limit_int = ftol(limit);
    write_i32(local + contract::WAGE, limit_int);
    let mut limit_text = [0u8; 200];
    let lt = limit_text.as_mut_ptr() as usize;
    cs_format_wage(local, lt, 2, 2);
    let player = if staff != 0 { read_usize(staff + staff::PLAYER) } else { 0 };
    let fmt = if staff == 0 || club == 0 || player == 0
        || read_i16(club + club::REPUTATION) as i32 >= read_i16(player + player_sheet::CURRENT_REPUTATION) as i32 + 0x2ee
        || exe::staff_is_favoured_by(staff, read_u32(club + club::CHAIRMAN) as usize) == 0 {
        0x9b0324 // "Your board will allow wages up to"
    } else {
        let again = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff);
        write_usize(G_CONTRACT_LOOKUP_RESULT, again);
        let own = if again == 0 || cs_contract_type(again) == 5 { 0 } else { club_at(read_i32(again + contract::CLUB) as isize).addr() };
        if own == club { 0x9b03d8 } else { 0x9b036c } // "an important member" / "a key signing"
    };
    lang(TS_TEXT, fmt, lt);
    if read_i32(terms + contract::WAGE) > limit_int {
        write_i32(terms + contract::WAGE, limit_int);
    }
    exe::text_set_string(t as usize, TS_TEXT as *const u8);
    exe::text_add_link(t as usize, lt as *const u8, 7, limit_int as u32 as usize);
    draw(t as usize, area, 0x78, 0x302, &mut row, 3, pale, 0, 0);
    row += 1;
    tel26_area_text([area, 0, row as u32, 1, 0, 0xc, 3, pale, 0]);
    row += 1;
    // the wage row
    let options = read_u32(read_usize(G_CONFIG) + 4);
    lang(TS_TEXT, if options & 0x200000 != 0 { 0x9b0318 } else if options & 0x400000 != 0 { 0x9b0308 } else { 0x9b02fc });
    tel26_area_button([area, 1, row as u32, 1, green, 1, 3, pale, TS_TEXT as u32, 0, 0]);
    let r = row as u32;
    let minus = if read_i32(terms + contract::WAGE) <= 100 {
        tel26_area_text([area, 2, r, 0x30, dark, 0x2c, 3, pale, 0x98ba0c])
    } else {
        tel26_area_button([area, 2, r, 0x30, dark, 0xc, 3, pale, 0x98ba0c, 5, 0])
    };
    set_screen_param(0xf, minus as i16 as i32 as usize);
    cs_format_wage(terms, w, 0, 2);
    let wage_item = tel26_area_button([area, 3, r, 1, black, 0xc, 3, pale, w as u32, 0, 0]);
    let mut up = read_u32(terms + contract::WAGE);
    exe::cash_step_up(&mut up as *mut u32 as usize, 8);
    let top_cash = limit_int as u32;
    if exe::cash_get_display_amount(&up, 8) > exe::cash_get_display_amount(&top_cash, 8) {
        tel26_area_text([area, 4, r, 0x30, dark, 0x2c, 3, pale, 0x9afe7c]); // "+"
    } else {
        tel26_area_button([area, 4, r, 0x30, dark, 0xc, 3, pale, 0x9afe7c, 4, 0]);
    }
    tel26_area_button([area, 5, r, 0x30, dark, 0xc, 3, pale, 0x9afe78, 6, 0]); // "="
    row += 1;
    // the signing on fee row
    let r = row as u32;
    lang(TS_TEXT, 0x9b02ec); // "Signing On Fee"
    tel26_area_button([area, 1, r, 1, green, 1, 3, pale, TS_TEXT as u32, 0, 0]);
    let fee_minus = if read_i32(terms + CS_TERMS_FEE) == 0 {
        tel26_area_text([area, 2, r, 0x30, dark, 0x2c, 3, pale, 0x98ba0c])
    } else {
        tel26_area_button([area, 2, r, 0x30, dark, 0xc, 3, pale, 0x98ba0c, 0x18, 0])
    };
    set_screen_param(0x10, fee_minus as i16 as i32 as usize);
    let mut cash = read_u32(terms + CS_TERMS_FEE);
    cs_money_text(TS_TEXT, cash, 6, 5);
    let fee_item = tel26_area_button([area, 3, r, 1, black, 0xc, 3, pale, TS_TEXT as u32, 0, 0]);
    let mut fee_up = read_u32(terms + CS_TERMS_FEE);
    if exe::cash_step_up(&mut fee_up as *mut u32 as usize, 4) > budget {
        tel26_area_text([area, 4, r, 0x30, dark, 0x2c, 3, pale, 0x9afe7c]);
    } else {
        tel26_area_button([area, 4, r, 0x30, dark, 0xc, 3, pale, 0x9afe7c, 0x17, 0]);
    }
    tel26_area_button([area, 5, r, 0x30, dark, 0xc, 3, pale, 0x9afe78, 0x16, 0]);
    row += 1;
    // the start date, when it is after today
    let start = terms + contract::DATE_STARTED;
    let today_year = read_u16(G_CURRENT_DATE + 2);
    let start_year = read_u16(terms + contract::DATE_STARTED_YEAR);
    if today_year < start_year || (today_year == start_year && read_i16(G_CURRENT_DATE) < read_i16(start)) {
        let short_date: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(CS_FN_SHORT_DATE);
        lang(TS_TEXT, 0x9b02d8); // "Contract Start Date"
        tel26_area_button([area, 1, row as u32, 1, green, 1, 3, pale, TS_TEXT as u32, 0, 0]);
        short_date(start, TS_TEXT, 0x7d0, 0xff);
        tel26_area_button([area, 3, row as u32, 1, 0, 0xc, 3, pale, TS_TEXT as u32, 0, 0]);
        row += 1;
    }
    // the expiry row
    let r = row as u32;
    lang(TS_TEXT, 0x9b02c4); // "Contract Expires"
    tel26_area_button([area, 1, r, 1, green, 1, 3, pale, TS_TEXT as u32, 0, 0]);
    let asked_month = get_screen_param(9) as u8 as i8;
    let asked_year = get_screen_param(0xa) as u16;
    let month = get_screen_param(0xb) as u8 as i8;
    let year_param = get_screen_param(0xc) as u32;
    let year = year_param & 0xffff;
    let start_year = start_year as u32;
    let start_month = || exe::date_get_month(start) as i8;
    // the expiry one step earlier: a year back while it stays past the first year, else a month back
    let (earlier_month, earlier_year) = if (year == start_year + 1 && month > start_month()) || year > start_year + 1 {
        (month, year_param.wrapping_sub(1) as u16)
    } else if month.wrapping_sub(1) < 0 {
        (11, year_param.wrapping_sub(1) as u16)
    } else {
        (month.wrapping_sub(1), year_param as u16)
    };
    let at_request = if earlier_year == asked_year { earlier_month <= asked_month } else { earlier_year < asked_year };
    let kind = read_u8(terms + contract::CONTRACT_TYPE) & 0x3f;
    let expiry_minus = if at_request || kind == 1 {
        tel26_area_text([area, 2, r, 0x30, dark, 0x2c, 3, pale, 0x98ba0c])
    } else if (year_param as u16 == start_year as u16 && month as i32 == start_month() as i32 + 1)
        || (year == start_year + 1 && month == 0 && start_month() == 11) {
        tel26_area_button([area, 2, r, 0x30, dark, 0xc, 3, pale, 0x98ba0c, 0x1d, 0])
    } else {
        tel26_area_button([area, 2, r, 0x30, dark, 0xc, 3, pale, 0x98ba0c, 0xa, 0])
    };
    set_screen_param(0x11, expiry_minus as i16 as i32 as usize);
    if read_u8(terms + contract::CONTRACT_TYPE) & 0x3f == 1 {
        lang(TS_TEXT, 0x9b02b4); // "Month to Month"
    } else {
        lang(TS_TEXT, 0x9b0288, cs_month_name(month), year); // "<month> <year>"
    }
    tel26_area_button([area, 3, r, 1, 0, 0xc, 3, pale, TS_TEXT as u32, 0, 0]);
    let kind = read_u8(terms + contract::CONTRACT_TYPE) & 0x3f;
    if kind == 2 && year as i32 - read_i8(G_MAX_CONTRACT_LENGTH_YEARS) as i32 >= read_u16(G_CURRENT_DATE_YEAR) as i32 {
        tel26_area_text([area, 4, r, 0x30, dark, 0x2c, 3, pale, 0x9afe7c]);
    } else {
        tel26_area_button([area, 4, r, 0x30, dark, 0xc, 3, pale, 0x9afe7c, if kind == 1 { 0x1d } else { 9 }, 0]);
    }
    tel26_area_button([area, 5, r, 0x30, dark, 0xc, 3, pale, 0x9afe78, 0xb, 0]);
    row += 1;
    // the contract type row
    let r = row as u32;
    lang(TS_TEXT, 0x9b0278); // "Contract Type"
    tel26_area_text([area, 1, r, 1, 0, 1, 3, pale, TS_TEXT as u32]);
    if read_u8(terms + contract::CONTRACT_TYPE) & 0x80 != 0 {
        lang(TS_TEXT, 0x9b026c); // "Part Time"
    } else {
        lang(TS_TEXT, 0x9b0244, STR_EMPTY); // "Full Time"
    }
    tel26_area_button([area, 3, r, 1, green, 0xc, 3, pale, TS_TEXT as u32, 0x1e, 0]);
    row += 1;
    // the public offer row
    let at_club = || {
        let job_club = read_usize(staff + staff::CLUB_JOB);
        job_club != 0 && read_i32(job_club) == club_id
    };
    if !at_club() && slot == 0 {
        let r = row as u32;
        lang(TS_TEXT, 0x9b0230); // "Make offer public"
        tel26_area_button([area, 1, r, 1, 0, 1, 3, pale, TS_TEXT as u32, 0x1a, 0]);
        if get_screen_param(7) as u8 == 0 {
            lang(TS_TEXT, TS_NO);
            tel26_area_button([area, 3, r, 1, 0, 0xc, 3, pale, TS_TEXT as u32, 0x1a, 1]);
        } else {
            lang(TS_TEXT, TS_YES);
            tel26_area_button([area, 3, r, 1, 0, 0xc, 3, pale, TS_TEXT as u32, 0x1a, 0]);
        }
    }
    // the wage popup: eight steps of an eighth of the limit
    let mut wage_steps = [0i32; 8];
    for k in 1..=8usize {
        wage_steps[k - 1] = ftol(k as f64 * limit * read_f64(rdata::F64_0_125));
    }
    if wage_steps[0] > 100 {
        covered!("tel24_4 basic terms wage popup");
        let popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, green, wage_item]);
        for i in 0..8u32 {
            let v = cs_cash_step_from(&mut cash, i, wage_steps.as_ptr(), 8);
            write_u32(local + contract::WAGE, v);
            cs_format_wage(local, TS_TEXT, 0, 2);
            tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, green, yellow, 1, 1, black, TS_TEXT as u32, 0, 8,
                read_u32(local + contract::WAGE), popup]);
        }
    }
    // the fee popup: eight steps of a seventh of the budget (at most 3 million)
    if fee_item as i16 != -1 {
        let cap = if budget < 3_000_000 { budget } else { 3_000_000 };
        let mut fee_steps = [0i32; 8];
        for (k, s) in fee_steps.iter_mut().enumerate() {
            *s = (k as i32).wrapping_mul(cap) / 7;
        }
        if fee_steps[1].wrapping_sub(fee_steps[0]) > 1000 {
            let popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, green, fee_item]);
            if popup as i16 != -1 {
                covered!("tel24_4 basic terms fee popup");
                for i in 0..8u32 {
                    cs_cash_step_from(&mut cash, i, fee_steps.as_ptr(), 8);
                    exe::cash_format_money(TS_TEXT, 0x7d0, exe::cash_get_display_amount(&cash, 5), 5);
                    tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, green, yellow, 1, 1, black, TS_TEXT as u32, 0, 0x19,
                        cash, popup]);
                }
            }
        }
    }
    row += 2;
    if window != 0 {
        let contract_of = || exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff);
        if contract_of() != 0 && exe::staff_contract_get_expiry_band(contract_of()) == 0 {
            covered!("tel24_4 basic terms next window");
            let mut opens = [0u32; 2];
            let mut ends = [0u32; 2];
            exe::date_date_54a110(opens.as_mut_ptr() as usize);
            exe::date_date_54a110(ends.as_mut_ptr() as usize);
            lang(TS_TEXT, if at_club() { 0x9b01c4 } else { 0x9b00f4 });
            exe::text_set_string(t as usize, TS_TEXT as *const u8);
            exe::transfer_manager_get_next_window_dates(&mut opens, ends.as_mut_ptr() as usize);
            let mut long_date = [0u8; 200];
            let ld = long_date.as_mut_ptr() as usize;
            if read_u16(G_CURRENT_DATE + 2) == (opens[0] >> 16) as u16 && read_u16(G_CURRENT_DATE) == opens[0] as u16 {
                exe::date_get_long_date_string(ends.as_ptr() as usize, ld, 0xc8, 0xff);
                lang(TS_TEXT, 0x9b00c0, ld); // "The current transfer window ends on"
            } else {
                exe::date_get_long_date_string(opens.as_ptr() as usize, ld, 0xc8, 0xff);
                lang(TS_TEXT, 0x9b0090, ld); // "The next transfer window opens on"
            }
            exe::text_append_string(t as usize, TS_TEXT);
            draw(t as usize, area, 0x78, 0x302, &mut row, 3, pale, 0, 0);
        } else if contract_of() != 0 && exe::staff_contract_get_expiry_band(contract_of()) != 0 {
            covered!("tel24_4 basic terms transfer date");
            lang(TS_TEXT, if at_club() { 0x9b0028 } else { 0x9aff4c });
            exe::text_set_string(t as usize, TS_TEXT as *const u8);
            draw(t as usize, area, 0x78, 0x302, &mut row, 3, pale, 0, 0);
            row += 2;
            lang(TS_TEXT, 0x9aff3c); // "Transfer date"
            tel26_area_text([area, 1, row as u32, 1, 0, 1, 3, pale, TS_TEXT as u32]);
            if window_state == 3 {
                lang(TS_TEXT, 0x9aff2c); // "End of contract"
                tel26_area_button([area, 3, row as u32, 1, 0, 0xc, 3, pale, TS_TEXT as u32, 0x30, 0]);
            } else {
                lang(TS_TEXT, 0x9aff20); // "Immediate"
                tel26_area_button([area, 3, row as u32, 1, 0, 0xc, 3, pale, TS_TEXT as u32, 0x30, 3]);
            }
            row += 1;
        }
    }
    let _ = row;
    exe::drop_text(t);
}

/// Comparer of `contract_screens_build_basic_terms_tab`: side by side over the display list, the human's terms (the
/// wage is cut to the limit), the edited terms and the contract lookup global.
pub(crate) unsafe extern "C" fn compare_contract_screens_build_basic_terms_tab(terms: usize, edited: usize, staff: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, i32, i32)>(%%I0%%);
    let extra = [(terms, CS_TERMS_SIZE), (edited, CS_TERMS_SIZE), (G_CONTRACT_LOOKUP_RESULT, 4)];
    compare_by_display_list_with("Offer contract basic terms tab", &extra, || o(terms, edited, staff, top, bottom),
        || contract_screens_build_basic_terms_tab(terms, edited, staff, top, bottom));
}

/// Answers the basic terms tab of the offer contract screen (0 when the item is not one of its own).
///
/// Without the human's terms: the fatal box (line 0xb34). The wage: "+" (id 4) reads param 8 (unused) and steps the
/// wage up (`increase_wage_step`), "-" (5) steps it down (`decrease_wage_step`); both reset "-" (param 0xf: enabled
/// id 5, or after "-" disabled up to a wage of 100), write the wage text and set "+" (enabled id 4 while one more step
/// stays within the limit of param 0x18, else disabled), then redraw and answer 0; "=" (6) goes back to the edited
/// wage and a popup step (7, 8) becomes the wage (-11). The signing on fee: "+" (0x17) and "-" (0x18) step it (mode 4)
/// and redraw: "-" (param 0x10) enabled while the fee is set, the fee text (size 200, mode 6, 5 when long), "+"
/// enabled while one more step stays within the budget of param 0x15; "=" (0x16) and a popup step (0x19) set it, cut
/// to the budget (-11). The "+"/"-" neighbours of the wage and the fee get `set_widget_id` where `set_widget_flags`
/// was meant (`cs_widget_state_by_id`). The expiry: "+" (9) makes the contract fixed (type 2) and, unless it was month
/// to month, moves params 0xb/0xc a year on past the first year (from the requested month of param 9) or a month on;
/// "-" (0xa) moves them a year back past next year (or a month back); "-" of the first month (0x1d) turns a month to
/// month contract into a fixed one or a fixed one into month to month; each rewrites the expiry widgets (param 0x11)
/// like the builder and redraws; "=" (0xb) takes the edited expiry and type (-11). The public offer (0x1a) and the
/// transfer date (0x30) go to params 7 and 0x1c (-11). The job (0x1c): a player swaps 11 and 15; anyone else takes the
/// next job up to 10 he can take, and param 0x18 gets its wage limit (the club's maximum wage for the player jobs,
/// else `cs_staff_wage_limit`) (-11). Part time (0x1e) flips the bit 0x80 of the type (-11). The squad status (0x1f)
/// moves down (2→1, 3→2, 4→3, 5→4; 1→5 under 24, else 4), the edited terms are made again
/// (`set_to_desired_contract` with job 11) and params 0x19/0x1a get its two out values (the second × 1000) (-11).
/// Original: `contract_screens_handle_basic_terms_action` (0x4e2280, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_handle_basic_terms_action(item: u32) -> i32 {
    let item16 = item as i16;
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let terms = cs_human_terms();
    if terms == 0 {
        cs_fatal(0xb34);
    }
    let staff = staff_at(get_screen_param(0) as isize).addr();
    let club = cs_human_club();
    let edited = get_screen_param(2);
    let budget = get_screen_param(0x15) as i32;
    let mut fee = read_i32(terms + CS_TERMS_FEE);
    let slot = get_screen_param(3);
    let mut slot_index: i32 = -1;
    if slot != 0 && read_usize(slot) != 0 {
        slot_index = read_i32(read_usize(slot) + offer::SLOT_INDEX);
    }
    let asked_month = get_screen_param(9) as u8 as i8;
    let asked_year = get_screen_param(0xa) as u16;
    let mut month = get_screen_param(0xb) as u8 as i8;
    let mut year = get_screen_param(0xc) as u32;
    let redraw = || {
        exe::screenmanager_redraw_screen(G_SCREEN_MANAGER, 0, 0);
        0
    };
    let expiry_text = |month: i8, year: u32| {
        lang(TS_TEXT, 0x9b0288, cs_month_name(month), year & 0xffff);
    };
    let fixed_at_maximum = |year: u32| (year & 0xffff) as i32 - read_i8(G_MAX_CONTRACT_LENGTH_YEARS) as i32 >= read_u16(G_CURRENT_DATE_YEAR) as i32;
    match tel26_item_id(item16) {
        4 => {
            covered!("tel24_4 basic terms wage up");
            get_screen_param(8);
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(CS_FN_INCREASE_WAGE_STEP);
            f(terms);
            let limit = get_screen_param(0x18) as u32;
            let minus = get_screen_param(0xf) as u32;
            cs_widget_state(minus, 2, 0xc, 5);
            let mut up = read_u32(terms + contract::WAGE);
            cs_format_wage(terms, TS_TEXT, 0, 2);
            exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, minus + 1, TS_TEXT);
            exe::cash_step_up(&mut up as *mut u32 as usize, 8);
            if exe::cash_get_display_amount(&up, 8) > exe::cash_get_display_amount(&limit, 8) {
                cs_widget_state_by_id(minus + 2, 6, 0x2c, 0);
            } else {
                cs_widget_state_by_id(minus + 2, 2, 0xc, 4);
            }
            redraw()
        }
        5 => {
            covered!("tel24_4 basic terms wage down");
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(CS_FN_DECREASE_WAGE_STEP);
            f(terms);
            let minus = get_screen_param(0xf) as u32;
            if read_i32(terms + contract::WAGE) <= 100 {
                cs_widget_state_by_id(minus, 6, 0x2c, 0);
            } else {
                cs_widget_state_by_id(minus, 2, 0xc, 5);
            }
            cs_format_wage(terms, TS_TEXT, 0, 2);
            exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, minus + 1, TS_TEXT);
            cs_widget_state(minus + 2, 2, 0xc, 4);
            redraw()
        }
        6 => {
            write_i32(terms + contract::WAGE, read_i32(edited + contract::WAGE));
            -11
        }
        7 | 8 => {
            write_u32(terms + contract::WAGE, tel26_item_value(item16));
            -11
        }
        9 => {
            covered!("tel24_4 basic terms expiry up");
            let kind = read_u8(terms + contract::CONTRACT_TYPE);
            write_u8(terms + contract::CONTRACT_TYPE, kind & 0xc2 | 2);
            if kind & 0x3f != 1 {
                let next_year = read_u16(G_CURRENT_DATE_YEAR) as u32 + 1;
                let y = year & 0xffff;
                if (y == next_year && month >= asked_month) || y > next_year {
                    year = year.wrapping_add(1);
                } else {
                    month = month.wrapping_add(1);
                    if month > 11 {
                        year = year.wrapping_add(1);
                        month = 0;
                    }
                }
                set_screen_param(0xb, month as i32 as usize);
                set_screen_param(0xc, (year & 0xffff) as usize);
            }
            let minus = get_screen_param(0x11) as u32;
            cs_widget_state(minus, 2, 0xc, 0xa);
            expiry_text(month, year);
            exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, minus + 1, TS_TEXT);
            if fixed_at_maximum(year) {
                cs_widget_state(minus + 2, 6, 0x2c, 0);
            } else {
                cs_widget_state(minus + 2, 2, 0xc, 9);
            }
            redraw()
        }
        0xa => {
            covered!("tel24_4 basic terms expiry down");
            let next_year = read_u16(G_CURRENT_DATE_YEAR) as u32 + 1;
            let y = year & 0xffff;
            if (y == next_year && month > exe::date_get_month(G_CURRENT_DATE) as i8) || y > next_year {
                year = year.wrapping_add(0xffff);
            } else {
                month = month.wrapping_sub(1);
                if month < 0 {
                    year = year.wrapping_add(0xffff);
                    month = 11;
                }
            }
            set_screen_param(0xb, month as i32 as usize);
            let shown_year = year & 0xffff;
            set_screen_param(0xc, shown_year as usize);
            let minus = get_screen_param(0x11) as u32;
            let start = terms + contract::DATE_STARTED;
            let start_year = read_u16(terms + contract::DATE_STARTED_YEAR) as u32;
            let start_month = || exe::date_get_month(start) as i8;
            let (earlier_month, earlier_year) = if (shown_year == start_year + 1 && month > start_month()) || shown_year > start_year + 1 {
                (month, year.wrapping_sub(1) as u16)
            } else if month.wrapping_sub(1) < 0 {
                (11, year.wrapping_sub(1) as u16)
            } else {
                (month.wrapping_sub(1), year as u16)
            };
            let at_request = if earlier_year == asked_year { earlier_month <= asked_month } else { earlier_year < asked_year };
            if at_request {
                cs_widget_state(minus, 6, 0x2c, 0);
            } else if (year as u16 == start_year as u16 && month as i32 == start_month() as i32 + 1)
                || (shown_year == start_year + 1 && month == 0 && start_month() == 11) {
                cs_widget_state(minus, 2, 0xc, 0x1d);
            } else if read_u8(terms + contract::CONTRACT_TYPE) & 0x3f == 1 {
                cs_widget_state(minus, 6, 0x2c, 0);
            } else {
                cs_widget_state(minus, 2, 0xc, 0xa);
            }
            expiry_text(month, shown_year);
            exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, minus + 1, TS_TEXT);
            cs_widget_state(minus + 2, 2, 0xc, 9);
            redraw()
        }
        0xb => {
            set_screen_param(0xb, exe::date_get_month(edited + contract::CONTRACT_EXPIRES) as i8 as i32 as usize);
            set_screen_param(0xc, read_u16(edited + contract::CONTRACT_EXPIRES_YEAR) as usize);
            let kind = read_u8(terms + contract::CONTRACT_TYPE);
            write_u8(terms + contract::CONTRACT_TYPE, kind & 0xc0 | read_u8(edited + contract::CONTRACT_TYPE) & 0x3f);
            -11
        }
        0x16 => {
            let wanted = read_i32(edited + CS_TERMS_FEE);
            write_i32(terms + CS_TERMS_FEE, wanted);
            if wanted > budget {
                write_i32(terms + CS_TERMS_FEE, budget);
            }
            -11
        }
        0x17 => {
            covered!("tel24_4 basic terms fee up");
            exe::cash_step_up(&mut fee as *mut i32 as usize, 4);
            write_i32(terms + CS_TERMS_FEE, fee);
            let minus = get_screen_param(0x10) as u32;
            cs_widget_state(minus, 2, 0xc, 0x18);
            let mut cash = read_u32(terms + CS_TERMS_FEE);
            cs_money_text_200(cash, 6, 5);
            exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, minus + 1, TS_TEXT);
            if exe::cash_step_up(&mut cash as *mut u32 as usize, 4) > budget {
                cs_widget_state_by_id(minus + 2, 6, 0x2c, 0);
            } else {
                cs_widget_state_by_id(minus + 2, 2, 0xc, 0x17);
            }
            redraw()
        }
        0x18 => {
            covered!("tel24_4 basic terms fee down");
            let f: unsafe extern "thiscall" fn(*mut i32, u32) -> u32 = core::mem::transmute(CS_FN_CASH_STEP_DOWN);
            f(&mut fee, 4);
            write_i32(terms + CS_TERMS_FEE, fee);
            let minus = get_screen_param(0x10) as u32;
            if read_i32(terms + CS_TERMS_FEE) == 0 {
                cs_widget_state_by_id(minus, 6, 0x2c, 0);
            } else {
                cs_widget_state_by_id(minus, 2, 0xc, 0x18);
            }
            cs_money_text_200(read_u32(terms + CS_TERMS_FEE), 6, 5);
            exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, minus + 1, TS_TEXT);
            cs_widget_state(minus + 2, 2, 0xc, 0x17);
            redraw()
        }
        0x19 => {
            let v = tel26_item_value(item16) as i32;
            write_i32(terms + CS_TERMS_FEE, v);
            if v > budget {
                write_i32(terms + CS_TERMS_FEE, budget);
            }
            -11
        }
        0x1a => {
            set_screen_param(7, tel26_item_value(item16) as usize);
            -11
        }
        0x1c => {
            covered!("tel24_4 basic terms job");
            let mut job = get_screen_param(8) as u8;
            if read_usize(staff + staff::PLAYER) != 0 {
                job = if job != 0xb { 0xb } else { 0xf };
            } else {
                loop {
                    job = job.wrapping_add(1);
                    if job == 0xb {
                        job = 0;
                    }
                    if cs_can_take_job(job as u32, staff) {
                        break;
                    }
                }
                let limit = if cs_player_wage_job(job as i8) {
                    let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
                    exe::finance_get_max_wage(fin, staff, u32::MAX) as f64
                } else {
                    cs_staff_wage_limit(club, staff, job as u32)
                };
                set_screen_param(0x18, ftol(limit) as usize);
            }
            set_screen_param(8, job as i8 as i32 as usize);
            -11
        }
        0x1d => {
            covered!("tel24_4 basic terms first month");
            let minus = get_screen_param(0x11) as u32;
            let kind = read_u8(terms + contract::CONTRACT_TYPE);
            if kind & 0x3f == 1 {
                write_u8(terms + contract::CONTRACT_TYPE, kind & 0xc2 | 2);
                cs_widget_state(minus, 2, 0xc, 0xa);
                expiry_text(month, year);
                exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, minus + 1, TS_TEXT);
                if fixed_at_maximum(year) {
                    cs_widget_state(minus + 2, 6, 0x2c, 0);
                    return redraw();
                }
            } else {
                write_u8(terms + contract::CONTRACT_TYPE, kind & 0xc1 | 1);
                cs_widget_state(minus, 3, 0x2c, 0);
                lang(TS_TEXT, 0x9b02b4); // "Month to Month"
                exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, minus + 1, TS_TEXT);
            }
            cs_widget_state(minus + 2, 2, 0xc, 9);
            redraw()
        }
        0x1e => {
            let kind = read_u8(terms + contract::CONTRACT_TYPE);
            write_u8(terms + contract::CONTRACT_TYPE, (if kind & 0x80 == 0 { 0x80 } else { 0 }) | kind & 0x7f);
            -11
        }
        0x1f => {
            covered!("tel24_4 basic terms squad status");
            let b = read_u8(terms + contract::SQUAD_STATUS);
            let status = b >> 4;
            let next = match status {
                1 => Some(if read_i8(staff + staff::AGE) >= 0x18 { 4 } else { 5 }),
                2 => Some(1),
                3 => Some(2),
                4 => Some(3),
                5 => Some(4),
                _ => None,
            };
            if let Some(s) = next {
                write_u8(terms + contract::SQUAD_STATUS, b & 0xf | s << 4);
            }
            // the status argument: the new status in the low byte over what the register held (the person for
            // status 1, status - 1 otherwise); the callee reads the low byte
            let high = (if status == 1 { staff as u32 } else { (status as i32 - 1) as u32 }) & 0xffff_ff00;
            let mut first: u32 = 0;
            let mut second: f32 = 0.0;
            exe::staff_contract_set_to_desired_contract(edited, staff, club, slot_index, high | (read_u8(terms + contract::SQUAD_STATUS) >> 4) as u32,
                0xb, &mut first as *mut u32 as u32, &mut second as *mut f32 as u32);
            set_screen_param(0x19, first as usize);
            set_screen_param(0x1a, ftol(second as f64 * read_f64(rdata::F64_1000)) as usize);
            -11
        }
        0x30 => {
            set_screen_param(0x1c, tel26_item_value(item16) as usize);
            -11
        }
        _ => 0,
    }
}

/// Comparer of `contract_screens_handle_basic_terms_action`: the answers that only change the human's terms or the
/// screen params (`CS_BASIC_PARAM_ANSWERS`) side by side over the screen params, the human's and the edited terms and
/// the contract lookup global; the others (they change widgets and redraw, or remake the edited terms with a draw)
/// and the missing terms (the fatal box) only in the original.
pub(crate) unsafe extern "C" fn compare_contract_screens_handle_basic_terms_action(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I1%%);
    let terms = cs_human_terms();
    if terms == 0 || !CS_BASIC_PARAM_ANSWERS.contains(&tel26_item_id(item as i16)) {
        // simplification: the widget changes, the redraw and the desired contract's draw do not run twice
        return o(item);
    }
    let mut regions = screen_param_regions();
    regions.push((terms, CS_TERMS_SIZE));
    regions.push((G_CONTRACT_LOOKUP_RESULT, 4));
    let edited = get_screen_param(2);
    if edited != 0 {
        regions.push((edited, CS_TERMS_SIZE));
    }
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(item), || contract_screens_handle_basic_terms_action(item));
    SCOREBOARD.record(equal && ro == rr, || format!("Offer contract basic terms tab answer to item {}: original={} rust={} differences at {:?}", item as i16, ro, rr, first_differences(&mo, &mr)));
    ro
}
