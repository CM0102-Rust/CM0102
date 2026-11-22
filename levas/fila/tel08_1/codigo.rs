// ---------- batch %%LEVA%%: the staff profile's Contract tab (staff_screens.cpp, part 2) ----------

/// `langlib_sprintf` (0x66f4e0, cdecl, variadic): the translated format into `out`.
unsafe fn staff_tab_lang() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_LANGLIB_SPRINTF)
}

/// The CRT `sprintf` (0x944c9f, cdecl, variadic).
unsafe fn staff_tab_format() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_CRT_SPRINTF)
}

/// `ScreenManager::add_area_text` (0x804100) of the staff profile tabs: cell (`col`, `row`) of `area`, the four
/// style fields `a`..`d`, the colour word at `colour` (zero-extended: the original pushes the register with only its
/// low half loaded) and the text (0 = an empty cell).
unsafe fn staff_tab_cell(area: u32, col: u32, row: i32, a: u32, b: u32, c: u32, d: u32, colour: usize, text: usize) -> u32 {
    tel26_area_text([area, col, row as u32, a, b, c, d, job_colour(colour), text as u32])
}

/// `ScreenManager::add_area_button` (0x8040a0) of the staff profile tabs: like `staff_tab_cell` plus the command id
/// and its value.
unsafe fn staff_tab_button(area: u32, col: u32, row: i32, a: u32, b: u32, c: u32, d: u32, colour: usize, text: usize, id: u32, value: u32) -> u32 {
    tel26_area_button([area, col, row as u32, a, b, c, d, job_colour(colour), text as u32, id, value])
}

/// `Text::draw_wrapped` (0x4aeaf0, thiscall, ret 0x20): draws the Text into `area` from x1 to x2 starting at row
/// `*row` (which it moves on when the text wraps), with the font, the colour word at `colour`, the style and the flags.
unsafe fn staff_tab_draw_wrapped(text: *mut usize, area: u32, x1: u32, x2: u32, row: &mut i32, font: u32, colour: usize, style: u32, flags: u32) {
    let f: unsafe extern "thiscall" fn(*mut usize, u32, u32, u32, *mut i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x4aeaf0usize);
    f(text, area, x1, x2, row, font, job_colour(colour), style, flags);
}

/// `Text::clear` (0x4afa40, thiscall).
unsafe fn staff_tab_text_clear(text: *mut usize) {
    let f: unsafe extern "thiscall" fn(*mut usize) = core::mem::transmute(0x4afa40usize);
    f(text);
}

/// How many rows of font `font` fit between `top` and `bottom` (signed division, kept as a byte like the original).
unsafe fn staff_tab_rows(top: i32, bottom: i32, font: i32) -> u8 {
    ((bottom - top) / exe::font_get_height(font)) as u8
}

/// The "no information" text of a tab: the message `msg` (translated into the screens' buffer) as a text area from
/// x 0x6e to 0x30c between `top` and `bottom`, in font 3, as many lines as fit in that font.
unsafe fn staff_tab_no_information(top: i32, bottom: i32, msg: usize) {
    let lines = staff_tab_rows(top, bottom, 3);
    staff_tab_lang()(JOB_TEXT, msg);
    tel26_text_area([0x6e, top as u32, 0x30c, bottom as u32, 2, 0, lines as u32, 3, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 0]);
}

/// The CRT's inline strcpy of the original (the same bytes).
unsafe fn staff_tab_strcpy(dst: usize, src: usize) {
    let mut i = 0;
    loop {
        let c = read_u8(src + i);
        write_u8(dst + i, c);
        if c == 0 {
            break;
        }
        i += 1;
    }
}

/// The contract of person `p` at his club (`CONTRACT_MANAGER::get_contract`), also left in
/// `G_CONTRACT_LOOKUP_RESULT` as the original does each time it asks, and its club (null without a contract).
unsafe fn contract_tab_lookup(p: Staff) -> (Contract, usize) {
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p));
    write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
    let club = if c.is_null() { 0 } else { club_at(c.club() as isize).addr() };
    (c, club)
}

/// The contract type text of the Contract tab into the screens' buffer: the low 6 bits of `contract::CONTRACT_TYPE`
/// as a signed number (0 invalid, 1 monthly, 2 normal, 3 and 4 "N/A", 5 trial, 6 and 7 loan), "Part Time" when bit
/// 0x80 is set, "Full Time" otherwise; any other value: "Part Time Contract", or "Full Time" with the comment for a
/// full-time one. No contract: "N/A".
unsafe fn contract_tab_type_text(c: Contract) {
    const NOT_AVAILABLE: usize = 0xa86748; // "  N/A"
    let lang = staff_tab_lang();
    if c.is_null() {
        lang(JOB_TEXT, NOT_AVAILABLE);
        return;
    }
    let b = c.contract_type();
    let kind = ((b << 2) as i8 >> 2) as i32;
    let s = if b & 0x80 != 0 {
        match kind {
            0 => 0xa867ec, // "  Invalid Contract Type"
            1 => 0xa86854, // "  Part Time Monthly Contract"
            3 | 4 => NOT_AVAILABLE,
            5 => 0xa86838, // "  Part Time Trial Contract"
            6 | 7 => 0xa8681c, // "  Part Time Loan Contract"
            _ => 0xa86804, // "  Part Time Contract"
        }
    } else {
        match kind {
            0 => 0xa867ec, // "  Invalid Contract Type"
            1 => 0xa867cc, // "  Full Time Monthly Contract"
            2 => 0xa867b4, // "  Full Time Contract"
            3 | 4 => NOT_AVAILABLE,
            5 => 0xa86798, // "  Full Time Trial Contract"
            6 | 7 => 0xa8677c, // "  Full Time Loan Contract"
            _ => {
                lang(JOB_TEXT, 0xa86750usize, 0xad9c64usize); // "  Full Time<%s - COMMENT - contract type>"
                return;
            }
        }
    };
    lang(JOB_TEXT, s);
}

/// The work permit line of the Contract tab into the screens' buffer: the games played and missed under the permit
/// and the share played (played × 100 / (played + missed), integer, 0 with no games), with the singular texts for 1.
unsafe fn contract_tab_work_permit_text(permit: usize) {
    let lang = staff_tab_lang();
    let played = read_u16(permit + work_permit::MATCHES_PLAYED);
    let missed = read_u16(permit + work_permit::MATCHES_MISSED);
    let total = played as i32 + missed as i32;
    let share = if total != 0 { ((played as i32 * 100) / total) as u8 as i8 } else { 0 };
    if played != 1 && missed != 1 {
        lang(JOB_TEXT, 0xa86498usize, played as u32, missed as u32, share as i32); // "  %d games played, %d games missed (%d%%)"
    } else if played != 1 {
        lang(JOB_TEXT, 0xa863f0usize, played as u32, share as i32); // "  %d games played, 1 game missed (%d%%)"
    } else if missed != 1 {
        lang(JOB_TEXT, 0xa86444usize, missed as u32, share as i32); // "  1 game played, %d games missed (%d%%)"
    } else {
        lang(JOB_TEXT, 0xa863c8usize); // "  1 game played, 1 game missed (50%%)"
    }
}

/// Draws the Contract tab of the staff profile (the person of screen parameter 0) between screen rows `top` and
/// `bottom`.
///
/// Without contract information (a person whose `job_for_club` is 1 or 2, a national job 5 without a club job or
/// with a club job other than 11, or an id from the person count − 16 up), only "There is no contract information
/// available." in font 3. Otherwise a two-column table (weights 2 and 7, as many rows as fit in font 2), labels in
/// pale grey and values in yellow: Type (`contract_tab_type_text`); Wages (`STAFF_CONTRACT::format_wage_text`, "-"
/// without a contract); for a normal contract (type 2) of a player whose `job_for_club` is 11, 12, 13 or 15 while the
/// FIFA transfer rules are in force: Started (the start date when it is after the rule change date
/// `G_TRANSFER_RULE_CHANGE_DATE`, year then day; else "Signed before new transfer system came into force" in English
/// or "N/A") and Protected (`check_fifa_protected_period`: "Yes" or "No (At least %d years since start of contract)");
/// Expires (`format_end_date_text`); Squad Status (`shortlist_manager_squad_status_text` of the high nibble of
/// `contract::SQUAD_STATUS`, a button 5 when the current human controls the club) when the human controls the
/// contract's club, the job is a player job and the contract is not a trial (type 5); for a loan, Loan Details ("On
/// loan from <club> until <date>", the club linked 0x7d5, wrapped from x 0x102), "Cannot play in cup matches" when
/// loan byte +0x1c is 0 and "Cannot play in matches against <parent club>" (linked) when loan byte +0x1d is 0;
/// Bonuses (each of the three bonuses ≥ 0, `staff_contract_format_bonus_text`, or "None"); Clauses (each of the five
/// release clause bytes set, `staff_contract_format_release_clause_name` and the release fee as money, in red for a
/// clause byte 2, or "None"); Work Permit (`contract_tab_work_permit_text`) when the person has a permit, needs one
/// for the club, has a normal contract and is a player; Notes (the human's note about the person, at most 499
/// characters, as button 0x2c; "- " without one); then empty rows to the end. Each lookup of the contract also leaves
/// it in `G_CONTRACT_LOOKUP_RESULT`.
///
/// Defects of the original (kept): every clause shows the same amount (`contract::RELEASE_FEE`); the Loan Details and
/// Work Permit lines read the club contract without checking it (a loan without a club contract would crash).
/// Original: `staff_screens_draw_contract_tab` (0x87cd90, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_draw_contract_tab(top: i32, bottom: i32) {
    const LABEL: usize = G_GUI_COLOUR_PALE_GREY;
    const VALUE: usize = G_GUI_COLOUR_YELLOW;
    let lang = staff_tab_lang();
    let format = staff_tab_format();
    let weights: [u8; 2] = [2, 7];
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0);
    let p = Staff(get_screen_param(0));
    let job = p.job_for_club();
    let count = read_i32(G_STAFF_COUNT);
    if job == 1
        || (p.job_for_nation() == 5 && (p.club_job().is_null() || job != 0xb))
        || job == 2
        || read_i32(p.addr()) >= count - 0x10
        || (!p.is_null() && read_i32(p.addr()) >= count)
    {
        covered!("tel08_1 contract tab without information");
        staff_tab_no_information(top, bottom, 0xa8638c); // "There is no contract information available."
        exe::drop_text(t);
        return;
    }
    covered!("tel08_1 contract tab");
    let rows = staff_tab_rows(top, bottom, 2);
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 2, weights.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    let mut row: i32 = 0;
    let mut buf = [0u8; 0x200];
    let b = buf.as_mut_ptr() as usize;
    let contract = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p));
    let label = |row: i32, s: usize| {
        staff_tab_cell(area, 0, row, 2, 0, 1, 2, LABEL, s);
    };
    let value = |row: i32, s: usize| {
        staff_tab_cell(area, 1, row, 2, 0, 1, 2, VALUE, s);
    };

    lang(JOB_TEXT, 0xa86874usize, 0xad9c64usize); // "  Type<%s - COMMENT - contract type>"
    label(row, JOB_TEXT);
    contract_tab_type_text(contract);
    value(row, JOB_TEXT);
    row += 1;

    lang(JOB_TEXT, 0x98f9dcusize); // "  Wages"
    label(row, JOB_TEXT);
    if !contract.is_null() {
        let wage: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x85e750usize);
        wage(contract.addr(), b, 1, 2);
        format(JOB_TEXT, 0x987ff0usize, b); // "  %s"
    } else {
        lang(JOB_TEXT, 0x98e8c8usize); // "  -"
    }
    value(row, JOB_TEXT);
    row += 1;

    if !contract.is_null()
        && contract.contract_type() & 0x3f == 2
        && !p.player().is_null()
        && matches!(p.job_for_club(), 0xb | 0xd | 0xf | 0xc)
        && read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1
    {
        covered!("tel08_1 contract tab fifa rules");
        lang(JOB_TEXT, 0xa8671cusize, 0xad9c64usize); // "  Started<%s - COMMENT contract start date>"
        label(row, JOB_TEXT);
        let rule_day = read_i16(G_TRANSFER_RULE_CHANGE_DATE);
        let rule_year = read_u16(G_TRANSFER_RULE_CHANGE_DATE + 2);
        let year = contract.date_started_year();
        let after_rule = rule_year < year || (rule_year == year && rule_day < read_i16(contract.addr() + contract::DATE_STARTED));
        if after_rule {
            let short_date: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(0x54a650usize);
            short_date(contract.addr() + contract::DATE_STARTED, b, 0xc8, 0xff);
            format(JOB_TEXT, 0x987ff0usize, b); // "  %s"
        } else if exe::langlib_get_language() == 0 {
            format(JOB_TEXT, 0xa866e8usize); // "  Signed before new transfer system came into force"
        } else {
            lang(JOB_TEXT, 0xa86748usize); // "  N/A"
        }
        value(row, JOB_TEXT);
        row += 1;
        lang(JOB_TEXT, 0xa866b8usize, 0xad9c64usize); // "  Protected<%s - COMMENT contract protected>"
        label(row, JOB_TEXT);
        let mut years: i8 = 0;
        if exe::staff_contract_check_fifa_protected_period(contract, 0usize, 0usize, &mut years as *mut i8 as usize) != 0 {
            lang(JOB_TEXT, 0xa866b0usize); // "  Yes"
        } else {
            lang(JOB_TEXT, 0xa86668usize, years as i32); // "  No (At least %d years since start of contract)"
        }
        value(row, JOB_TEXT);
        row += 1;
    }

    lang(JOB_TEXT, 0xa8665cusize); // "  Expires"
    label(row, JOB_TEXT);
    if !contract.is_null() {
        let end_date: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x85e8f0usize);
        end_date(contract.addr(), b);
        format(JOB_TEXT, 0x987ff0usize, b); // "  %s"
    } else {
        staff_tab_strcpy(JOB_TEXT, 0x98e8c8); // "  -"
    }
    value(row, JOB_TEXT);
    row += 1;

    if !contract.is_null() {
        let (_, club) = contract_tab_lookup(p);
        if exe::human_manager_current_controls_club(club, 0, 1) != 0
            && exe::database_is_player_job(p.job_for_club() as i8) != 0
            && {
                let (c, _) = contract_tab_lookup(p);
                c.is_null() || c.contract_type() & 0x3f != 5
            }
        {
            covered!("tel08_1 contract tab squad status");
            lang(JOB_TEXT, 0xa8664cusize); // "  Squad Status"
            label(row, JOB_TEXT);
            // writes after the two spaces the label line left in the buffer
            let status: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(0x84b330usize);
            status(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), (contract.squad_status() >> 4) as u32, JOB_TEXT + 2, 2000);
            let (_, club) = contract_tab_lookup(p);
            if exe::human_manager_current_controls_club(club, 0, 1) != 0 {
                staff_tab_button(area, 1, row, 2, 0, 1, 2, VALUE, JOB_TEXT, 5, 0);
            } else {
                value(row, JOB_TEXT);
            }
            row += 1;
        }
    }

    let loan = exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p);
    if loan != 0 {
        covered!("tel08_1 contract tab loan");
        // the loan contract's two bytes: may play in cup matches, may play against the parent club
        const LOAN_CUP_MATCHES: usize = 0x1c;
        const LOAN_AGAINST_PARENT: usize = 0x1d;
        lang(JOB_TEXT, 0xa8663cusize); // "  Loan Details"
        label(row, JOB_TEXT);
        exe::date_get_long_date_string(loan + contract::CONTRACT_EXPIRES, b, 0xc8, 0xff);
        let club = club_at(contract.club() as isize);
        let gender = read_i8(club.addr() + club::SHORT_GENDER_NAME) as i32;
        // "  On loan from {}<%s - Club Name>{} until {}<%s - Date>"
        lang(JOB_TEXT, 0xa865dcusize, gender, gender, 100u32, club.addr() + club::SHORT_NAME, b);
        exe::text_set_string(t as usize, JOB_TEXT as *const u8);
        exe::text_add_link(t as usize, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
        value(row, 0);
        staff_tab_draw_wrapped(t, area, 0x102, 0x30c, &mut row, 2, VALUE, 2, 0);
        row += 1;
        if read_u8(loan + LOAN_CUP_MATCHES) == 0 {
            label(row, 0);
            lang(JOB_TEXT, 0xa865bcusize); // "  Cannot play in cup matches"
            value(row, JOB_TEXT);
            row += 1;
        }
        if read_u8(loan + LOAN_AGAINST_PARENT) == 0 {
            let (_, club) = contract_tab_lookup(p);
            label(row, 0);
            let gender = read_i8(club + club::SHORT_GENDER_NAME) as i32;
            lang(JOB_TEXT, 0xa86574usize, gender, gender, club + club::SHORT_NAME); // "  Cannot play in matches against {}<%s>{}"
            exe::text_set_string(t as usize, JOB_TEXT as *const u8);
            exe::text_add_link(t as usize, (club + club::SHORT_NAME) as *const u8, 0x7d5, club);
            value(row, 0);
            staff_tab_draw_wrapped(t, area, 0x102, 0x30c, &mut row, 2, VALUE, 2, 0);
            row += 1;
        }
    }

    lang(JOB_TEXT, 0x98f9d0usize); // "  Bonuses"
    label(row, JOB_TEXT);
    let first = row;
    if !contract.is_null() {
        let bonus: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x85f830usize);
        for i in 0..3u32 {
            if read_i32(contract.addr() + contract::GOAL_BONUS + 4 * i as usize) > -1 {
                if row > first {
                    label(row, 0);
                }
                bonus(contract.addr(), i, b, 100, 2);
                format(JOB_TEXT, 0x987ff0usize, b); // "  %s"
                value(row, JOB_TEXT);
                row += 1;
            }
        }
    }
    if row == first {
        lang(JOB_TEXT, 0xa86550usize, 0xad9c64usize); // "  None<%s - COMMENT - no bonuses>"
        value(row, JOB_TEXT);
        row += 1;
    }

    lang(JOB_TEXT, 0xa86544usize); // "  Clauses"
    label(row, JOB_TEXT);
    let first = row;
    if !contract.is_null() {
        let clause_name: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x85f940usize);
        let mut money = [0u8; 0x800];
        for i in 0..5u32 {
            let kind = read_u8(contract.addr() + contract::NON_PROMOTION_RC + i as usize);
            if kind != 0 {
                let fee = read_u32(contract.addr() + contract::RELEASE_FEE);
                let amount = exe::cash_get_display_amount(&fee, 6);
                exe::cash_format_money(money.as_mut_ptr() as usize, 2000, amount, 6);
                if row > first {
                    label(row, 0);
                }
                clause_name(contract.addr(), i, b, 100, 2);
                format(JOB_TEXT, 0xa86538usize, b, money.as_ptr() as usize); // "  %s (%s)"
                staff_tab_cell(area, 1, row, 2, 0, 1, 2, if kind == 2 { G_GUI_COLOUR_RED } else { VALUE }, JOB_TEXT);
                row += 1;
            }
        }
    }
    if row == first {
        lang(JOB_TEXT, 0xa86514usize, 0xad9c64usize); // "  None<%s - COMMENT - no clauses>"
        value(row, JOB_TEXT);
        row += 1;
    }

    let permit = exe::transfer_manager_find_work_permit(read_usize(G_TRANSFER_MANAGER), p);
    if permit != 0 {
        let (_, club) = contract_tab_lookup(p);
        if exe::transfer_manager_player_needs_work_permit(read_usize(G_TRANSFER_MANAGER), p, club, 1) != 0
            && contract.contract_type() & 0x3f == 2
            && !p.player().is_null()
        {
            covered!("tel08_1 contract tab work permit");
            lang(JOB_TEXT, 0xa86504usize); // "  Work Permit"
            label(row, JOB_TEXT);
            contract_tab_work_permit_text(permit);
            value(row, JOB_TEXT);
            row += 1;
        }
    }

    lang(JOB_TEXT, 0xa863c0usize); // "  Notes"
    label(row, JOB_TEXT);
    let manager = tel26_slot_manager().addr();
    let find_note: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x795750usize);
    if manager != 0 && find_note(read_usize(G_NOTES), manager, p.addr()) != 0 {
        let note_text: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> usize = core::mem::transmute(0x7957b0usize);
        let mut note = [0u8; 0x200];
        note_text(read_usize(G_NOTES), manager, p.addr(), note.as_mut_ptr() as usize);
        // strncpy(500) then the 500th byte cleared: at most 499 characters
        let mut copy = [0u8; 500];
        if let Some(n) = note.iter().take(500).position(|&c| c == 0) {
            copy[..n].copy_from_slice(&note[..n]);
        } else {
            copy.copy_from_slice(&note[..500]);
        }
        copy[499] = 0;
        format(JOB_TEXT, 0x987ff0usize, copy.as_ptr() as usize); // "  %s"
    } else {
        lang(JOB_TEXT, 0xa863b8usize); // "  - "
    }
    staff_tab_button(area, 1, row, 2, 0, 1, 2, VALUE, JOB_TEXT, 0x2c, 0); // tel08_1 the notes button
    row += 1;
    while row < rows as i32 {
        staff_tab_cell(area, 0, row, 2, 0, 0xc, 2, LABEL, 0);
        staff_tab_cell(area, 1, row, 2, 0, 0xc, 2, LABEL, 0);
        row += 1;
    }
    exe::drop_text(t);
}

/// Comparer for `staff_screens_draw_contract_tab`: both draw side by side and are compared by the display list
/// (`compare_match_screen_build`) plus the contract lookup result the tab leaves (`G_CONTRACT_LOOKUP_RESULT`).
pub(crate) unsafe extern "C" fn compare_staff_screens_draw_contract_tab(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(%%I0%%);
    let extra = [(G_CONTRACT_LOOKUP_RESULT, 4)];
    compare_match_screen_build("staff profile contract tab", &extra, || o(top, bottom), || staff_screens_draw_contract_tab(top, bottom));
}
