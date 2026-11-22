// ---------- batch %%LEVA%%: the contract screens (2): the offer contract screen, its answers and the bonuses tab (contract_screens.cpp, tel24_2) ----------

/// The size of the contract terms the offer screen edits (`STAFF_CONTRACT`, 0x50 bytes).
const CS_TERMS_SIZE: usize = 0x50;
/// `STAFF_CONTRACT`'s bonus amounts (goal, assist, clean sheet: three dwords from +0x10) and its release clause flags
/// (five bytes from +0x1c; index 1 is the minimum fee clause, 4 the manager job clause).
const CS_TERMS_BONUSES: usize = 0x10;
const CS_TERMS_CLAUSES: usize = 0x1c;
/// The amount of the terms capped to the transfer budget of param 0x15 (`STAFF_CONTRACT` +8) [inference: the fee
/// part of the terms].
const CS_TERMS_FEE: usize = 0x8;
/// The job a contract offer is for (byte +0x61 of a CONTRACT_MANAGER offer).
const CS_OFFER_JOB: usize = 0x61;
/// The fees of the release fee popup (9 steps) and the base of the bonus popups.
const CS_RELEASE_FEE_STEPS: [u32; 9] = [0, 100_000, 250_000, 500_000, 1_000_000, 2_500_000, 5_000_000, 10_000_000, 15_000_000];
/// `CONTRACT_MANAGER::will_negotiate_offer(terms)` (0x4d4660, thiscall ret 4): whether the person asked for terms.
const CS_FN_WILL_NEGOTIATE: usize = 0x4d4660;
/// `CONTRACT_MANAGER::prefill_offer_from_pending(terms)` (0x4d4830, thiscall ret 4).
const CS_FN_PREFILL_OFFER: usize = 0x4d4830;
/// `staff_contract_is_clause_allowed(staff, club, clause)` (0x867ad0, cdecl).
const CS_FN_CLAUSE_ALLOWED: usize = 0x867ad0;
/// `staff_contract_get_release_clause_name(clause, buffer)` (0x867a00, cdecl).
const CS_FN_CLAUSE_NAME: usize = 0x867a00;
/// `staff_contract_is_bonus_type_allowed(staff, kind)` (0x867aa0, cdecl).
const CS_FN_BONUS_ALLOWED: usize = 0x867aa0;
/// `staff_contract_get_bonus_type_name(kind, buffer)` (0x8679a0, cdecl).
const CS_FN_BONUS_NAME: usize = 0x8679a0;
/// `Cash::format_display_amount(buffer, size, mode)` (0x450aa0, thiscall on a cash dword, ret 0xc).
const CS_FN_CASH_FORMAT_DISPLAY: usize = 0x450aa0;
/// `Cash::set_from_amount_table(index, table, count)` (0x43f720, thiscall on a cash dword, ret 0xc).
const CS_FN_CASH_FROM_TABLE: usize = 0x43f720;
/// `ScreenManager::add_menu_item(area, text, id, value)` (0x415640, thiscall, ret 0x10).
const CS_FN_ADD_MENU_ITEM: usize = 0x415640;
/// The three tab answers of the offer screen (cdecl(item) -> int), the tab builders (cdecl) and the other screens it
/// opens, by address.
const CS_FN_BASIC_TERMS_ACTION: usize = 0x4e2280;
const CS_FN_BONUSES_ACTION: usize = 0x4e39c0;
const CS_FN_CLAUSES_ACTION: usize = 0x4e4160;
const CS_FN_BUILD_BASIC_TERMS_TAB: usize = 0x4e06e0;
const CS_FN_BUILD_BONUSES_TAB: usize = 0x4e30f0;
const CS_FN_OPEN_STAFF_OFFER: usize = 0x4e4730;
const CS_FN_RULES_SCREEN: usize = 0x77b700;
const CS_FN_REMOVE_EXCHANGE_PLAYER: usize = 0x8f2870;
const CS_FN_BID_CONFIRM: usize = 0x8ef1e0;
/// The qsort comparers that order a club's squad for the squad popup.
const CS_SORT_BY_APPEARANCES: usize = 0x4e5850;
const CS_SORT_BY_SURNAME: usize = 0x53ff60;

/// The human manager's contract terms being edited (`HUMAN_MANAGER::get_contract_offer(0)`).
pub(crate) unsafe fn cs_human_terms() -> usize {
    let f: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(CS_FN_HUMAN_CONTRACT_OFFER);
    f(read_usize(G_HUMAN_MANAGER), 0)
}

/// `CONTRACT_MANAGER::will_negotiate_offer(terms)`.
unsafe fn cs_will_negotiate(terms: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(CS_FN_WILL_NEGOTIATE);
    f(read_usize(G_CONTRACT_MANAGER), terms)
}

/// Copies contract terms: the original copies them field by field (the two dates through `Date::operator=`, the
/// type and squad status bytes through bit merges whose result is the source byte), which is the whole 0x50 bytes.
// ponytail: one byte copy stands for the field-by-field copy; same bytes written
unsafe fn cs_copy_terms(dst: usize, src: usize) {
    for k in 0..CS_TERMS_SIZE {
        write_u8(dst + k, read_u8(src + k));
    }
}

/// A money value formatted into `dst` (2000 bytes) in `mode`, again in `short` when it takes more than 8 characters
/// (`Cash::get_display_amount` + `cash_format_money`).
pub(crate) unsafe fn cs_money_text(dst: usize, value: u32, mode: u32, short: u32) {
    let cash = value;
    exe::cash_format_money(dst, 0x7d0, exe::cash_get_display_amount(&cash, mode), mode);
    if c_string_len(dst) > 8 {
        exe::cash_format_money(dst, 0x7d0, exe::cash_get_display_amount(&cash, short), short);
    }
}

/// `Cash::format_display_amount` of a cash dword into `TS_TEXT` (2000 bytes).
pub(crate) unsafe fn cs_cash_display(cash: *mut u32, mode: u32) {
    let f: unsafe extern "thiscall" fn(*mut u32, usize, u32, u32) = core::mem::transmute(CS_FN_CASH_FORMAT_DISPLAY);
    f(cash, TS_TEXT, 0x7d0, mode);
}

/// `Cash::set_from_amount_table(index, table, count)` on a cash dword.
pub(crate) unsafe fn cs_cash_from_table(cash: *mut u32, index: u32, table: *const u32, count: u32) {
    let f: unsafe extern "thiscall" fn(*mut u32, u32, *const u32, u32) = core::mem::transmute(CS_FN_CASH_FROM_TABLE);
    f(cash, index, table, count);
}

/// The text of the current human's display wrapped into `area` (x 0x78..0x302) from `row`, font 3.
pub(crate) unsafe fn cs_wrapped(t: *mut usize, area: u32, row: &mut i32, colour: u32) {
    let draw: unsafe extern "thiscall" fn(usize, u32, i32, i32, *mut i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(TS_FN_TEXT_DRAW_WRAPPED);
    exe::text_set_string(t as usize, TS_TEXT as *const u8);
    draw(t as usize, area, 0x78, 0x302, row, 3, colour, 0, 0);
}

/// Builds the offer contract screen: the terms of the contract the human offers a person, on three tabs (basic
/// terms, bonuses, clauses).
///
/// Params read: 0 the person's id, 2 the edited terms, 3 the transfer offer slot, 8 the job, 1 the tab. A person
/// outside the table: the contract_screens.cpp assert box (line 0x56a). On the first build (no edited terms, param 2)
/// new terms are made (`STAFF_CONTRACT::STAFF_CONTRACT`, `set_to_desired_contract(person, club, slot index, role (param
/// 0xd), job, &x, &y)`; param 0x19 the first out value and 0x1a the second (a float) × 1000) and the human's terms
/// filled: from the club's current offer to him (`find_contract_offer`, not of types 5 or 6; its job goes to param 8)
/// or, when the human's terms are for someone else, from the new terms (then `prefill_offer_from_pending`); the wage is
/// cut to the budget of param 0x15. Params 9/0xa the start month and year (his contract's expiry when he is at the
/// terms' club, else today less 32 days), 2 the edited terms, 0xb/0xc the edited terms' expiry month and year.
/// Then the picture; when he plays for the human's club in a player job, a squad button with a popup of the club's
/// first twenty players (the club's squad sorted in place by appearances then by surname; id 3, the player as value)
/// in the club's kit colours; the title (name and club, the club a link 0x7d5) in the human's club colours; "Offer
/// Contract"; the transfer rules button (id 0x31) when the FIFA rules are in force and param 0x1d is set; the tab bar
/// Basic (1) / Bonuses (2) / Clauses (3) for a player job; the tab: `contract_screens_build_basic_terms_tab`,
/// `contract_screens_build_bonuses_tab` or the clauses (below); any other tab the assert box (line 0x662). The
/// clauses: two areas (labels 2:1:0x24, values 2:1:0xe:2:8:2:2:2:6), "Offer player the following clauses:-", the
/// release clauses 0..3 he may have (`is_clause_allowed`; a "*" when the edited terms hold it and he asked for terms),
/// a button each (id 0x13 pale when offered, 0x12 grey when not); the release fee row: without a release clause grey
/// text and "-" / fee / "+" texts; with one, "-" (id 0x28) when the fee is above the minimum (param 0x17 with the
/// minimum fee clause, else 5; the item index in param 0xe), the fee with a popup of the steps from 0 to 15 million
/// (below the minimum disabled, else id 0x26 with the fee), "+" (id 0x27) below 50 million and the last button (id
/// 0x29); "Non Release Fee Clauses:-" and clause 4; "* Requested by the player" on the last row when he asked. Last the
/// menu bar and the bar: without a transfer slot and no open offer (state 4 or 1 or none), Cancel (0x1b) / Offer (1);
/// otherwise Back (0x1b) / Cancel (2) / Offer (1).
/// Original: `contract_screens_build_offer_contract_screen` (0x4deac0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_build_offer_contract_screen() {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let draw: unsafe extern "thiscall" fn(usize, u32, i32, i32, *mut i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(TS_FN_TEXT_DRAW_WRAPPED);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    // simplification: the three MenuItem objects of the tab bar start zeroed apart from what the constructor and the
    // routine write (the original leaves the rest of the frame as it was)
    let mut tabs = [0u8; 0xd4 * 3];
    let mut text = [0usize; 8];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0usize);
    let id = get_screen_param(0) as i32;
    let staff = staff_at(id as isize).addr();
    let slot = get_screen_param(3);
    let mut slot_index: i32 = -1;
    if slot != 0 && read_usize(slot) != 0 {
        slot_index = read_i32(read_usize(slot) + offer::SLOT_INDEX);
    }
    let mut job = get_screen_param(8) as u8 as i8;
    if staff == 0 {
        tel26_error_box(CS_CPP, 0x56a);
        exe::drop_text(t);
        return;
    }
    let mut edited = get_screen_param(2);
    let club = cs_human_club();
    let terms = cs_human_terms();
    if edited == 0 {
        covered!("tel24_2 offer screen first build");
        let role = get_screen_param(0xd) as u8;
        let mem = exe::crt_operator_new(CS_TERMS_SIZE as u32);
        edited = if mem != 0 { exe::new_staff_contract(mem) } else { 0 };
        let mut first: u32 = 0;
        let mut second: f32 = 0.0;
        exe::staff_contract_set_to_desired_contract(edited, staff, club, slot_index, role as u32, job as u8 as u32,
            &mut first as *mut u32 as u32, &mut second as *mut f32 as u32);
        set_screen_param(0x19, first as usize);
        set_screen_param(0x1a, ftol(second as f64 * read_f64(rdata::F64_1000)) as usize);
        let current = exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), read_u32(staff), read_u32(club));
        if current == 0 || matches!(cs_contract_type(current), 5 | 6) {
            if read_u32(terms) != read_u32(edited) || read_u32(terms + 4) != read_u32(edited + 4) {
                cs_copy_terms(terms, edited);
                let prefill: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(CS_FN_PREFILL_OFFER);
                prefill(read_usize(G_CONTRACT_MANAGER), edited);
                let budget = get_screen_param(0x15) as i32;
                if read_i32(terms + CS_TERMS_FEE) > budget {
                    write_i32(terms + CS_TERMS_FEE, budget);
                }
            }
        } else {
            cs_copy_terms(terms, current);
            let budget = get_screen_param(0x15) as i32;
            if read_i32(terms + CS_TERMS_FEE) > budget {
                write_i32(terms + CS_TERMS_FEE, budget);
            }
            set_screen_param(8, read_i8(current + CS_OFFER_JOB) as i32 as usize);
            job = read_i8(current + CS_OFFER_JOB);
        }
        let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff);
        if c != 0 && read_u32(c + contract::CLUB) == read_u32(terms + 4) {
            set_screen_param(9, exe::date_get_month(c + contract::CONTRACT_EXPIRES) as i8 as i32 as usize);
            set_screen_param(0xa, read_u16(c + contract::CONTRACT_EXPIRES_YEAR) as usize);
        } else {
            let mut before = [0u32; 2];
            let d = exe::date_sub_days(G_CURRENT_DATE, before.as_mut_ptr() as usize, 0x20);
            set_screen_param(9, exe::date_get_month(d) as i8 as i32 as usize);
            let d = exe::date_sub_days(G_CURRENT_DATE, before.as_mut_ptr() as usize, 0x20);
            set_screen_param(0xa, read_u16(d + 2) as usize);
        }
        set_screen_param(2, edited);
        set_screen_param(0xb, exe::date_get_month(edited + contract::CONTRACT_EXPIRES) as i8 as i32 as usize);
        set_screen_param(0xc, read_u16(edited + contract::CONTRACT_EXPIRES_YEAR) as usize);
    }
    let tab = get_screen_param(1) as u8 as i8;
    picture(staff, 0, 0);
    let mut a = read_u16(G_GUI_COLOUR_BLUE) as u32;
    let mut b = read_u16(G_GUI_COLOUR_PALE_GREY) as u32;
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    let own = if c == 0 || cs_contract_type(c) == 5 { 0 } else { club_at(read_i32(c + contract::CLUB) as isize).addr() };
    if own != 0 && own == club && exe::database_is_player_job(read_i8(staff + staff::JOB_FOR_CLUB)) != 0 {
        covered!("tel24_2 offer screen squad popup");
        exe::database_get_club_kit_colours(own, 1, &mut a, &mut b);
        let button = tel26_add_button([0x69, 0xf, 0x78, 0x23, 0x30, b, 0xc, 1, a, 0, 0, 0]);
        let popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x30, job_colour(JOB_COLOUR_31AA), button]);
        let squad = own + club::SQUAD;
        exe::crt_qsort(squad, 0x32, 4, CS_SORT_BY_APPEARANCES);
        exe::crt_qsort(squad, 0x14, 4, CS_SORT_BY_SURNAME);
        for k in 0..20 {
            let p = read_usize(squad + k * 4);
            if p == 0 || p == staff {
                continue;
            }
            exe::database_format_staff_name(p, TS_TEXT, 0x7d0, 4);
            tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, job_colour(JOB_COLOUR_31AA), ts_colour(G_GUI_COLOUR_YELLOW), 1, 1,
                ts_colour(G_GUI_COLOUR_BLACK), TS_TEXT as u32, 0, 3, read_u32(p), popup]);
        }
    } else {
        a = read_u16(G_GUI_COLOUR_BLUE) as u32;
        b = read_u16(G_GUI_COLOUR_PALE_GREY) as u32;
    }
    if club != 0 {
        exe::database_get_club_kit_colours(club, 1, &mut a, &mut b);
    }
    let mut name = [0u8; 2000];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(staff, n, 0x7d0, 1);
    let own_club = read_usize(staff + staff::CLUB_JOB);
    if own_club != 0 {
        sprintf(TS_TEXT, 0x9a3d8c, n, own_club + club::SHORT_NAME); // "%s (%s)"
        exe::text_set_string(t as usize, TS_TEXT as *const u8);
        let own_club = read_usize(staff + staff::CLUB_JOB);
        exe::text_add_link(t as usize, (own_club + club::SHORT_NAME) as *const u8, 0x7d5, own_club);
    } else {
        tel26_copy_text(TS_TEXT as *mut u8, n as *const u8);
        exe::text_set_string(t as usize, TS_TEXT as *const u8);
    }
    let title = tel26_add_area(current_human_display(), [100, 10, 0x316, 0x46, 1, 0, 1, 0, 0x30, b, u32::MAX]);
    let mut row: i32 = 0;
    draw(t as usize, title, 100, 0x316, &mut row, 7, a, 1, 0);
    lang(TS_TEXT, 0x9aff10); // "Offer Contract"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    let pale = ts_colour(G_GUI_COLOUR_PALE_GREY);
    let grey = ts_colour(G_GUI_COLOUR_GREY);
    let dark = ts_colour(G_GUI_COLOUR_DARK_BLUE);
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 && get_screen_param(0x1d) != 0 {
        lang(TS_TEXT, 0x9afed4, STR_EMPTY); // "Rules<...>"
        tel26_add_button([0x2da, 0x55, 0x311, 0x69, 0x30, dark, 0xc, 1, pale, TS_TEXT as u32, 0x31, 0]);
    }
    let mut top: i32 = 0x8c;
    let mut bottom: i32 = 0x221;
    if exe::database_is_player_job(job) != 0 {
        for (k, (label, id)) in [(0x9afeccusize, 1u32), (0x9afec4, 2), (0x9afebc, 3)].into_iter().enumerate() {
            // a MenuItem: the word 0xffff its constructor writes, the text, the font and the id
            let item = tabs.as_mut_ptr() as usize + k * 0xd4;
            write_u16(item, 0xffff);
            lang(item + 4, label); // "Basic", "Bonuses", "Clauses"
            write_u32(item + 0xcc, 0xc);
            write_u32(item + 0xd0, id);
        }
        exe::gui_add_tab_bar(tabs.as_mut_ptr() as usize, 3, tab as i32, &mut top, &mut bottom, 1);
    }
    match tab {
        1 => {
            let f: unsafe extern "C" fn(usize, usize, usize, i32, i32) = core::mem::transmute(CS_FN_BUILD_BASIC_TERMS_TAB);
            f(terms, edited, staff, top, bottom);
        }
        2 => {
            let f: unsafe extern "C" fn(usize, usize, usize, i32, i32) = core::mem::transmute(CS_FN_BUILD_BONUSES_TAB);
            f(terms, edited, staff, top, bottom);
        }
        3 => cs_build_clauses_tab(terms, edited, staff, top, bottom),
        _ => tel26_error_box(CS_CPP, 0x662),
    }
    exe::menubar_build(4, 0);
    let w2: [u8; 2] = [3, 1];
    let w3: [u8; 3] = [2, 1, 1];
    let three = |bar: u32| {
        lang(TS_TEXT, TS_BACK);
        tel26_area_button([bar, 0, 0, 0x30, grey, 0xc, 3, pale, TS_TEXT as u32, 0x1b, 0]);
        lang(TS_TEXT, 0x98a1a0); // "Cancel"
        tel26_area_button([bar, 1, 0, 0x30, grey, 0xc, 3, pale, TS_TEXT as u32, 2, 0]);
        lang(TS_TEXT, 0x98e6e8); // "Offer"
        2u32
    };
    let (bar, col) = if slot == 0 {
        let open = exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), read_u32(staff), read_u32(club));
        if open == 0 || matches!(read_u8(open + contract_offer::STATE), 4 | 1) {
            let bar = tel26_add_area(current_human_display(), [100, 0x22b, 0x316, 0x24e, 2, w2.as_ptr() as u32, 1, 0, 1, b, u32::MAX]);
            lang(TS_TEXT, 0x98a1a0); // "Cancel"
            tel26_area_button([bar, 0, 0, 0x30, grey, 0xc, 3, pale, TS_TEXT as u32, 0x1b, 0]);
            lang(TS_TEXT, 0x98e6e8); // "Offer"
            (bar, 1)
        } else {
            let bar = tel26_add_area(current_human_display(), [100, 0x22b, 0x316, 0x24e, 3, w3.as_ptr() as u32, 1, 0, 1, b, u32::MAX]);
            (bar, three(bar))
        }
    } else {
        let bar = tel26_add_area(current_human_display(), [100, 0x22b, 0x316, 0x24e, 3, w3.as_ptr() as u32, 1, 0, 1, b, u32::MAX]);
        (bar, three(bar))
    };
    tel26_area_button([bar, col, 0, 0x30, grey, 0xc, 3, pale, TS_TEXT as u32, 1, 0]);
    exe::drop_text(t);
}

/// The clauses tab of the offer contract screen (inline in `contract_screens_build_offer_contract_screen`).
unsafe fn cs_build_clauses_tab(terms: usize, edited: usize, staff: usize, top: i32, bottom: i32) {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let allowed: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(CS_FN_CLAUSE_ALLOWED);
    let clause_name: unsafe extern "C" fn(u32, usize) = core::mem::transmute(CS_FN_CLAUSE_NAME);
    let add_menu_item: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(CS_FN_ADD_MENU_ITEM);
    let pale = ts_colour(G_GUI_COLOUR_PALE_GREY);
    let grey = ts_colour(G_GUI_COLOUR_GREY);
    let dark = ts_colour(G_GUI_COLOUR_DARK_BLUE);
    let marker = ts_colour(0xaebdd4);
    let w_labels: [u8; 3] = [2, 1, 0x24];
    let w_values: [u8; 9] = [2, 1, 0xe, 2, 8, 2, 2, 2, 6];
    let mut row: i32 = 0;
    let mut popup: u32 = u32::MAX;
    let mut text = [0usize; 8];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0usize);
    let mut any = false;
    let mut cash: u32 = 0;
    let steps = CS_RELEASE_FEE_STEPS;
    let asked = cs_will_negotiate(edited);
    let rows = ((bottom - top) / exe::font_get_height(3)) as u8;
    let labels = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 3, w_labels.as_ptr() as u32, rows as u32, 0, 2, pale, u32::MAX]);
    let values = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 9, w_values.as_ptr() as u32, rows as u32, 0, 1, pale, u32::MAX]);
    lang(TS_TEXT, 0x9afe94); // "Offer player the following clauses:-"
    cs_wrapped(t, labels, &mut row, pale);
    row += 2;
    let terms_club = || club_at(read_i32(terms + 4) as isize).addr();
    let clause_row = |k: u32, row: &mut i32, any: &mut bool| {
        if read_i32(terms + 4) == -1 || allowed(staff, terms_club(), k) == 0 {
            return;
        }
        if read_u8(edited + CS_TERMS_CLAUSES + k as usize) != 0 && asked != 0 {
            tel26_area_text([labels, 1, *row as u32, 1, 0, 0xc, 3, marker, 0x989ad8]); // "*"
        }
        clause_name(k, TS_TEXT);
        if read_u8(terms + CS_TERMS_CLAUSES + k as usize) == 1 {
            tel26_area_button([labels, 2, *row as u32, 1, 0, 1, 3, pale, TS_TEXT as u32, 0x13, k]);
            *any = true;
        } else {
            tel26_area_button([labels, 2, *row as u32, 1, 0, 1, 3, grey, TS_TEXT as u32, 0x12, k]);
        }
        *row += 1;
    };
    for k in 0..4u32 {
        clause_row(k, &mut row, &mut any);
    }
    row += 1;
    let r = row as u32;
    let minimum: i32;
    if !any {
        covered!("tel24_2 clauses without a release clause");
        lang(TS_TEXT, 0x9afe80); // "Clause Release Fee"
        tel26_area_text([labels, 2, r, 1, 0, 1, 3, grey, TS_TEXT as u32]);
        tel26_area_text([values, 3, r, 0x30, dark, 0x2c, 3, pale, 0x98ba0c]); // "-"
        let mut fee = read_u32(terms + contract::RELEASE_FEE);
        cs_cash_display(&mut fee, 6);
        if c_string_len(TS_TEXT) > 8 {
            cs_cash_display(&mut fee, 5);
        }
        tel26_area_text([values, 4, r, 1, 0, 0x2c, 3, pale, TS_TEXT as u32]);
        tel26_area_text([values, 5, r, 0x30, dark, 0x2c, 3, pale, 0x9afe7c]); // "+"
        tel26_area_text([values, 6, r, 0x30, dark, 0x2c, 3, pale, 0x9afe78]);
        minimum = fee as i32;
    } else {
        covered!("tel24_2 clauses with a release clause");
        let fee = read_i32(terms + contract::RELEASE_FEE);
        lang(TS_TEXT, 0x9afe80); // "Clause Release Fee"
        tel26_area_text([labels, 2, r, 1, 0, 1, 3, pale, TS_TEXT as u32]);
        minimum = if read_u8(terms + contract::MINIMUM_FEE_RC) == 1 { get_screen_param(0x17) as i32 } else { 5 };
        let minus = if minimum < fee {
            tel26_area_button([values, 3, r, 0x30, dark, 0xc, 3, pale, 0x98ba0c, 0x28, 0])
        } else {
            tel26_area_text([values, 3, r, 0x30, dark, 0x2c, 3, pale, 0x98ba0c])
        };
        set_screen_param(0xe, minus as i16 as i32 as usize);
        cs_money_text(TS_TEXT, fee as u32, 6, 5);
        let item = tel26_area_button([values, 4, r, 1, 0, 0xc, 3, pale, TS_TEXT as u32, 0, 0]);
        let add_popup: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x456850usize);
        popup = add_popup(G_SCREEN_MANAGER, item);
        if fee < 50_000_000 {
            tel26_area_button([values, 5, r, 0x30, dark, 0xc, 3, pale, 0x9afe7c, 0x27, 0]);
        } else {
            tel26_area_text([values, 5, r, 0x30, dark, 0x2c, 3, pale, 0x9afe7c]);
        }
        tel26_area_button([values, 6, r, 0x30, dark, 0xc, 3, pale, 0x9afe78, 0x29, 0]);
    }
    row += 2;
    lang(TS_TEXT, 0x9afe5c); // "Non Release Fee Clauses:-"
    cs_wrapped(t, labels, &mut row, pale);
    row += 2;
    clause_row(4, &mut row, &mut any);
    if asked != 0 {
        row = rows as i8 as i32 - 1;
        lang(TS_TEXT, 0x9afe40); // "* Requested by the player"
        cs_wrapped(t, labels, &mut row, marker);
    }
    if popup as u16 != 0xffff {
        for k in 0..9u32 {
            cs_cash_from_table(&mut cash, k, steps.as_ptr(), 9);
            cs_cash_display(&mut cash, 5);
            if (cash as i32) < minimum {
                contract_screens_add_disabled_menu_item_by_address(popup, TS_TEXT as u32);
            } else {
                add_menu_item(G_SCREEN_MANAGER, popup, TS_TEXT, 0x26, cash);
            }
        }
    }
    exe::drop_text(t);
}

/// `contract_screens_add_disabled_menu_item` by address (it is hooked).
unsafe fn contract_screens_add_disabled_menu_item_by_address(area: u32, text: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x4e0180usize);
    f(G_SCREEN_MANAGER, area, text);
}

/// The regions an offer screen build writes beyond the display list: the human's terms, the edited terms and the
/// contract lookup global.
unsafe fn cs_offer_regions() -> Vec<(usize, usize)> {
    let mut r = vec![(G_CONTRACT_LOOKUP_RESULT, 4)];
    let terms = cs_human_terms();
    if terms != 0 {
        r.push((terms, CS_TERMS_SIZE));
    }
    let edited = get_screen_param(2);
    if edited != 0 {
        r.push((edited, CS_TERMS_SIZE));
    }
    r
}

/// Comparer of `contract_screens_build_offer_contract_screen`: side by side over the display list, the human's and
/// the edited terms, the contract lookup global and the squad the popup sorts; on the first build (it makes the
/// edited terms) and without the person only the original runs.
pub(crate) unsafe extern "C" fn compare_contract_screens_build_offer_contract_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    let staff = staff_at(get_screen_param(0) as i32 as isize).addr();
    if get_screen_param(2) == 0 || staff == 0 {
        return o();
    }
    let mut extra = cs_offer_regions();
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff);
    if c != 0 {
        let own = club_at(read_i32(c + contract::CLUB) as isize).addr();
        if own != 0 {
            extra.push((own + club::SQUAD, 0xc8));
        }
    }
    compare_by_display_list_with("Offer contract screen", &extra, || o(), || contract_screens_build_offer_contract_screen());
}

/// Answers the offer contract screen.
///
/// A tab (id 0xc) goes to param 1 (-11); a player of the squad popup (id 3) becomes the person (param 0) and the edited
/// terms are dropped from param 2 without freeing them (-11); the rules button (id 0x31) opens the transfer rules
/// screen (-4). Cancel (id 2): for someone else's transfer offer (param 6), when the offer changed (its serial is not
/// param 5's, or it was rejected or withdrawn) the offer is deleted, the slot cleared and the offer changed screen
/// opened with the deleted offer (-11), else the remove exchange player dialog (-4); with a transfer offer the bid
/// confirm dialog (cancel, 2); otherwise the withdraw dialog (`open_staff_offer_contract_screen(0, 0, 0xb, 1, 0, 3)`).
/// Back (id 0x1b) clears the human's terms (id -1) and answers -10. Offer (id 1): the human's terms expire on the day of
/// their start (at most the 28th) of the month and year of params 0xb/0xc; when the transfer offer changed, the offer
/// changed screen (-11), else the offer dialog (`open_staff_offer_contract_screen(slot, aware, job, 0, param 0x1d,
/// param 0x1c)`; aware is the offer's or param 7) (-4). Anything else goes to the basic terms, bonuses and clauses
/// answers, then the menu bar.
/// Original: `contract_screens_handle_offer_contract_screen` (0x4e01e0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_handle_offer_contract_screen(item: u32) -> i32 {
    let item16 = item as i16;
    let open_staff: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32) = core::mem::transmute(CS_FN_OPEN_STAFF_OFFER);
    let changed: unsafe extern "C" fn(usize, u32) = core::mem::transmute(CS_FN_OPEN_OFFER_CHANGED);
    let id = tel26_item_id(item16);
    if id == 0xc {
        set_screen_param(1, tel26_item_value(item16) as usize);
        return -11;
    }
    if id == 3 {
        set_screen_param(0, tel26_item_value(item16) as usize);
        set_screen_param(2, 0);
        return -11;
    }
    if id == 0x31 {
        let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(CS_FN_RULES_SCREEN);
        f(0, 0);
        return -4;
    }
    let offer_changed = |o: usize, serial: u32| read_u32(o + offer::SERIAL) != serial || matches!(read_u8(o + offer::STATUS), 3 | 2);
    if id == 2 {
        covered!("tel24_2 offer screen Cancel");
        let slot = get_screen_param(3);
        let o = if slot != 0 { read_usize(slot) } else { 0 };
        if get_screen_param(6) != 0 {
            let serial = get_screen_param(5) as u32;
            if o != 0 && offer_changed(o, serial) {
                let live = read_usize(slot);
                if live != 0 {
                    exe::drop_transfer_offer(live);
                    exe::crt_operator_delete(live);
                }
                write_usize(slot, 0);
                // the original passes the offer it has just deleted (a dangling pointer) to the screen
                changed(o, serial);
                return -11;
            }
            let f: unsafe extern "C" fn(usize, u32) = core::mem::transmute(CS_FN_REMOVE_EXCHANGE_PLAYER);
            f(slot, read_u32(cs_human_terms()));
            return -4;
        }
        if slot != 0 {
            let f: unsafe extern "C" fn(usize, u32, usize) = core::mem::transmute(CS_FN_BID_CONFIRM);
            f(slot, 2, slot);
            return -4;
        }
        open_staff(0, 0, 0xb, 1, 0, 3);
        return -4;
    }
    if id == 0x1b {
        write_i32(cs_human_terms(), -1);
        return -10;
    }
    if id != 1 {
        for f in [CS_FN_BASIC_TERMS_ACTION, CS_FN_BONUSES_ACTION, CS_FN_CLAUSES_ACTION] {
            let f: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(f);
            let r = f(item);
            if r != 0 {
                return r;
            }
        }
        return ts_menubar_answer(item);
    }
    covered!("tel24_2 offer screen Offer");
    let terms = cs_human_terms();
    let slot = get_screen_param(3);
    let mut o = 0;
    let aware = if slot == 0 {
        get_screen_param(7) as u8
    } else {
        o = read_usize(slot);
        read_u8(o + offer::PLAYER_AWARE)
    };
    let job = get_screen_param(8) as u8;
    let window_state = get_screen_param(0x1c) as u8;
    let window = get_screen_param(0x1d);
    let month = get_screen_param(0xb);
    let year = get_screen_param(0xc);
    let start = terms + contract::DATE_STARTED;
    let day = if exe::date_get_day_of_month(start) as i16 > 0x1c { 0x1c } else { exe::date_get_day_of_month(start) as i16 as i32 };
    let mut date = [0u16; 4];
    let d = exe::date_date_549f70(&mut date, day as u32, month as u32, year as u32, u32::MAX);
    let mut out = [0u32; 2];
    exe::date_operator_3d(terms + contract::CONTRACT_EXPIRES, out.as_mut_ptr() as usize, d);
    let serial = get_screen_param(5) as u32;
    if o != 0 && offer_changed(o, serial) {
        changed(o, serial);
        return -11;
    }
    open_staff(slot, aware as u32, job as u32, 0, window as u32, window_state as u32);
    -4
}

/// Comparer of `contract_screens_handle_offer_contract_screen`: the tab and squad player answers side by side over
/// the screen params; the rest only in the original (they open screens, change the terms or the offer).
pub(crate) unsafe extern "C" fn compare_contract_screens_handle_offer_contract_screen(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I1%%);
    ts_compare_answer("Offer contract screen", item, &[0xc, 3], o, contract_screens_handle_offer_contract_screen)
}

/// Builds the bonuses tab of the offer contract screen.
///
/// One area (columns 2:1:0xe:2:8:2:2:2:6, as many rows as fit between `top` and `bottom`) with "Offer player the
/// following bonuses:-". The most a bonus can be is a tenth of the club's maximum wage (`get_max_wage(person, -1)` of
/// the terms' club). For each bonus type he can have (`is_bonus_type_allowed`; goal, assist, clean sheet): the human's
/// bonus is cut to that maximum (a write to the terms); when he asked for terms and the edited bonus is set and within
/// the maximum, a "*" and a note of what he asked (the amount in mode 10, 9 when long). Without a bonus (negative), a
/// grey button (id 0xd) and texts (the amount shown is a tenth of the wage, or his request); with one, a pale button
/// (0xe), "-" (id 0x10, the item index in param 0x12 + type) from 6 up, the amount with a popup of five steps (a fifth
/// of the maximum each; id 0x2a + type, the amount as value), "+" (id 0xf) below the maximum and the last button (id
/// 0x11), and the request note. "* Requested by the player" on the last row when he asked.
/// Original: `contract_screens_build_bonuses_tab` (0x4e30f0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_build_bonuses_tab(terms: usize, edited: usize, staff: usize, top: i32, bottom: i32) {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let allowed: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(CS_FN_BONUS_ALLOWED);
    let bonus_name: unsafe extern "C" fn(u32, usize) = core::mem::transmute(CS_FN_BONUS_NAME);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let pale = ts_colour(G_GUI_COLOUR_PALE_GREY);
    let grey = ts_colour(G_GUI_COLOUR_GREY);
    let dark = ts_colour(G_GUI_COLOUR_DARK_BLUE);
    let marker = ts_colour(0xaebdd4);
    let w: [u8; 9] = [2, 1, 0xe, 2, 8, 2, 2, 2, 6];
    let mut row: i32 = 0;
    let mut text = [0usize; 8];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0usize);
    let mut cash: u32 = 0;
    let mut popups: [i16; 3] = [-1; 3];
    let asked = cs_will_negotiate(edited);
    let rows = ((bottom - top) / exe::font_get_height(3)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 9, w.as_ptr() as u32, rows as u32, 0, 2, pale, u32::MAX]);
    lang(TS_TEXT, 0x9b0eb4); // "Offer player the following bonuses:-"
    cs_wrapped(t, area, &mut row, pale);
    row += 2;
    let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club_at(read_i32(edited + contract::CLUB) as isize).addr());
    let most = exe::finance_get_max_wage(fin, staff, u32::MAX) / 10;
    let mut request = [0u8; 0x20];
    let req = request.as_mut_ptr() as usize;
    for k in 0..3u32 {
        if allowed(staff, k) == 0 {
            continue;
        }
        let bonus = terms + CS_TERMS_BONUSES + k as usize * 4;
        if most < read_i32(bonus) {
            write_i32(bonus, most);
        }
        let wanted = read_i32(edited + CS_TERMS_BONUSES + k as usize * 4);
        let r = row as u32;
        if wanted < 0 || asked == 0 || most < wanted {
            request[0] = 0;
        } else {
            tel26_area_text([area, 1, r, 1, 0, 0xc, 3, marker, 0x989ad8]); // "*"
            cs_money_text(TS_TEXT, wanted as u32, 10, 9);
            sprintf(req, 0x98fb04, TS_TEXT);
        }
        let v = read_i32(bonus);
        if v < 0 {
            bonus_name(k, TS_TEXT);
            tel26_area_button([area, 2, r, 1, 0, 1, 3, grey, TS_TEXT as u32, 0xd, k]);
            tel26_area_button([area, 3, r, 0x30, dark, 0x2c, 3, pale, 0x98ba0c, 0xd, k]); // "-"
            if request[0] == 0 {
                cs_money_text(TS_TEXT, (read_i32(terms + contract::WAGE) / 10) as u32, 10, 9);
            } else {
                tel26_copy_text(TS_TEXT as *mut u8, req as *const u8);
            }
            tel26_area_button([area, 4, r, 1, 0, 0x2c, 3, pale, TS_TEXT as u32, 0xd, k]);
            tel26_area_button([area, 5, r, 0x30, dark, 0x2c, 3, pale, 0x9afe7c, 0xd, k]); // "+"
            tel26_area_button([area, 6, r, 0x30, dark, 0x2c, 3, pale, 0x9afe78, 0xd, k]);
            if request[0] != 0 {
                tel26_area_button([area, 8, r, 1, 0, 0x21, 3, marker, req as u32, 0xd, k]);
            }
        } else {
            covered!("tel24_2 bonuses tab bonus set");
            bonus_name(k, TS_TEXT);
            tel26_area_button([area, 2, r, 1, 0, 1, 3, pale, TS_TEXT as u32, 0xe, k]);
            let minus = if v < 6 {
                tel26_area_text([area, 3, r, 0x30, dark, 0x2c, 3, pale, 0x98ba0c])
            } else {
                tel26_area_button([area, 3, r, 0x30, dark, 0xc, 3, pale, 0x98ba0c, 0x10, k])
            };
            set_screen_param(k + 0x12, minus as i16 as i32 as usize);
            cs_money_text(TS_TEXT, v as u32, 10, 9);
            let item = tel26_area_button([area, 4, r, 1, 0, 0xc, 3, pale, TS_TEXT as u32, 0, 0]);
            popups[k as usize] = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), item]) as i16;
            if v < most {
                tel26_area_button([area, 5, r, 0x30, dark, 0xc, 3, pale, 0x9afe7c, 0xf, k]);
            } else {
                tel26_area_text([area, 5, r, 0x30, dark, 0x2c, 3, pale, 0x9afe7c]);
            }
            tel26_area_button([area, 6, r, 0x30, dark, 0xc, 3, pale, 0x9afe78, 0x11, k]);
            if request[0] != 0 {
                tel26_area_button([area, 8, r, 1, 0, 1, 3, marker, req as u32, 0x11, k]);
            }
        }
        row += 1;
    }
    if asked != 0 {
        row = rows as i8 as i32 - 1;
        lang(TS_TEXT, 0x9afe40); // "* Requested by the player"
        cs_wrapped(t, area, &mut row, marker);
    }
    let mut steps = [0i32; 5];
    let mut acc = most;
    for s in steps.iter_mut() {
        *s = acc / 5;
        acc = acc.wrapping_add(most);
    }
    for (k, &popup) in popups.iter().enumerate() {
        if popup == -1 {
            continue;
        }
        for i in 0..5u32 {
            cs_cash_from_table(&mut cash, i, steps.as_ptr() as *const u32, 5);
            exe::cash_format_money(TS_TEXT, 0x7d0, exe::cash_get_display_amount(&cash, 10), 10);
            tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, job_colour(JOB_COLOUR_31AA), ts_colour(G_GUI_COLOUR_YELLOW), 1, 1,
                ts_colour(G_GUI_COLOUR_BLACK), TS_TEXT as u32, 0, 0x2a + k as u32, cash, popup as u16 as u32]);
        }
    }
    exe::drop_text(t);
}

/// Comparer of `contract_screens_build_bonuses_tab`: side by side over the display list, the human's and the edited
/// terms (the bonuses are capped in the terms) and the screen params.
pub(crate) unsafe extern "C" fn compare_contract_screens_build_bonuses_tab(terms: usize, edited: usize, staff: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, i32, i32)>(%%I2%%);
    let extra = [(terms, CS_TERMS_SIZE), (edited, CS_TERMS_SIZE)];
    compare_by_display_list_with("Offer contract bonuses tab", &extra, || o(terms, edited, staff, top, bottom), || contract_screens_build_bonuses_tab(terms, edited, staff, top, bottom));
}
