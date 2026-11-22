// ---------- batch %%LEVA%%: the staff profile's History tab (staff_screens.cpp, part 3) ----------

/// Colour word 0xae3152 of the screen items (the season fee and the career totals).
const HISTORY_TAB_COLOUR_3152: usize = 0xae3152;

/// `Display::add_item` (0x55f990) of the History tab's View and Filter popups: a menu line with the text in the
/// screens' buffer, flags 0x2002 when it is the line in use (2 otherwise), command `id` with `value`, in popup area
/// `parent` (the whole eax the area call returned, as the original pushes it).
unsafe fn history_tab_menu_line(in_use: bool, id: u32, value: u32, parent: u32) {
    tel26_add_item(current_human_display(), [if in_use { 0x2002 } else { 2 }, 0, 0, 0, 0, 0, 0, 0x10, job_colour(JOB_COLOUR_31AA),
        job_colour(G_GUI_COLOUR_YELLOW), 1, 1, job_colour(G_GUI_COLOUR_BLACK), JOB_TEXT as u32, 0, id, value, parent]);
}

/// A View popup line (command 0x14, `bit` of screen parameter 5): the line in use also becomes the tab's title.
unsafe fn history_tab_view_line(view: u32, bit: u32, parent: u32, title: &mut [u8; 200]) {
    let in_use = view & bit != 0;
    history_tab_menu_line(in_use, 0x14, bit, parent);
    if in_use {
        staff_tab_strcpy(title.as_mut_ptr() as usize, JOB_TEXT);
    }
}

/// `Date::get_short_date_string` (0x54a650, thiscall, ret 0xc): the date at `date` into `out`.
unsafe fn history_tab_date(date: usize, out: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(0x54a650usize);
    f(date, out, 2000, 0xff);
}

/// The 0xff-ended list of statistic codes that are the columns of the Playing Career view: with screen parameter
/// 0x15 (the second statistics view) the 0xa84a90 list for a goalkeeper (`goalkeeper` 20) and 0xa84a78 otherwise;
/// without it 0xa84a9c for a goalkeeper and 0xa84a84 otherwise.
unsafe fn history_tab_columns(p: Staff, second_view: usize) -> usize {
    let keeper = !p.player().is_null() && p.player().goalkeeper() == 0x14;
    match (second_view != 0, keeper) {
        (true, true) => 0xa84a90,
        (true, false) => 0xa84a78,
        (false, true) => 0xa84a9c,
        (false, false) => 0xa84a84,
    }
}

/// The empty cells of a table row from column 0 to `columns` - 1 (style 2, 0, 0xc, 2, pale grey, no text).
unsafe fn history_tab_blank_row(area: u32, row: i32, columns: u32) {
    for col in 0..columns {
        staff_tab_cell(area, col, row, 2, 0, 0xc, 2, G_GUI_COLOUR_PALE_GREY, 0);
    }
}

/// The scroll row of the table area: `Area::set_scroll_row` with screen parameter 9 when there is an area, and the
/// area index (as a signed word) in screen parameter 10.
unsafe fn history_tab_keep_scroll(area: u32) {
    let row = get_screen_param(9);
    if area as i16 != -1 {
        exe::area_set_scroll_row(display_area_at(current_human_display(), area as i16), row);
    }
    set_screen_param(10, area as i16 as i32 as usize);
}

/// The fee text of a career season into `out` (200 bytes): a positive fee as money (`Cash::get_display_amount` and
/// `cash_format_money` in mode 5), with "<%s - cash> p/x" around it for a type 2 (exchange) move; no fee: "Exch" for
/// type 2, "Bos" for type 3 and "Free" otherwise; a negative fee: "Loan" for type 1, nothing otherwise. The type is
/// the low 3 bits of the season's byte +0x28.
unsafe fn history_tab_fee_text(e: usize, out: &mut [u8; 200]) {
    let lang = staff_tab_lang();
    out[0] = 0;
    let fee = read_i32(e + 0x24);
    let kind = read_u8(e + 0x28) & 7;
    if fee > 0 {
        let mut money = [0u8; 200];
        let amount = exe::cash_get_display_amount(&(fee as u32), 5);
        exe::cash_format_money(money.as_mut_ptr() as usize, 200, amount, 5);
        if kind == 2 {
            lang(out.as_mut_ptr() as usize, 0xa8b0a0usize, money.as_ptr() as usize); // "<%s - cash> p/x"
        } else {
            staff_tab_strcpy(out.as_mut_ptr() as usize, money.as_ptr() as usize);
        }
    } else if fee == 0 {
        let s = match kind {
            2 => 0x98e620, // "Exch"
            3 => 0x98e61c, // "Bos"
            _ => 0x98e614, // "Free"
        };
        lang(out.as_mut_ptr() as usize, s);
    } else if kind == 1 {
        lang(out.as_mut_ptr() as usize, 0x98e5fcusize); // "Loan"
    }
}

/// `PlayerStats` text of statistic `code` (0x7c1b20 the column title; 0x7c2750 a season's value; 0x7c3060 the career
/// total), thiscall on `G_PLAYER_STATS`, into the screens' buffer.
unsafe fn history_tab_stat_title(code: u8) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(0x7c1b20usize);
    f(read_usize(G_PLAYER_STATS), code as u32, JOB_TEXT, 2000);
}

unsafe fn history_tab_stat_season(entry: usize, code: u8, full: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32, u32) = core::mem::transmute(0x7c2750usize);
    f(read_usize(G_PLAYER_STATS), entry, code as u32, JOB_TEXT, 2000, 1, 0, full);
}

unsafe fn history_tab_stat_total(totals: usize, code: u8) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32, u32) = core::mem::transmute(0x7c3060usize);
    f(read_usize(G_PLAYER_STATS), totals, code as u32, JOB_TEXT, 2000, 0, 0, 0);
}

/// A statistic cell of the Playing Career table: code 0x11 in style 0x30 with the 0xae3192 colour word, the others in
/// style 2, 0, 0xc, 2; text colour `colour`.
unsafe fn history_tab_stat_cell(area: u32, col: u32, row: i32, code: u8, colour: usize) {
    if code == 0x11 {
        staff_tab_cell(area, col, row, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, colour, JOB_TEXT);
    } else {
        staff_tab_cell(area, col, row, 2, 0, 0xc, 2, colour, JOB_TEXT);
    }
}

/// The Playing Career view: the seasons of screen parameter 6 (0x102-byte rows from +0xd9; screen parameter 7 is
/// six times their number, 8 the season in use), the career totals and empty rows to the end.
unsafe fn history_tab_playing_career(p: Staff, top: i32, bottom: i32, kind: u32, second_view: usize) {
    let weights_head: [u8; 12] = [0x33, 5, 5, 8, 8, 8, 8, 8, 8, 8, 8, 8];
    let weights_body: [u8; 14] = [0xd, 0x1a, 0xc, 5, 5, 8, 8, 8, 8, 8, 8, 8, 8, 8];
    let career = get_screen_param(6);
    let size = get_screen_param(7) as i32;
    let in_use = get_screen_param(8) as i32;
    let mut totals = [0i32; 19];
    let t = totals.as_mut_ptr() as usize;
    let add = |i: usize, v: i32| write_i32(t + 4 * i, read_i32(t + 4 * i).wrapping_add(v));
    let mut loaned_apps: i32 = 0;
    let span = bottom - top;
    let full = (span / exe::font_get_height(2)) as u8;
    let row_height = span / full as i32;
    let rows = full.wrapping_sub(1);
    let rows_dword = (kind & !0xff) | rows as u32;
    let d = current_human_display();
    let x2 = if size / 6 > rows as i32 { 0x2f8 } else { 0x30c };
    let head = tel26_add_area(d, [0x6e, top as u32, x2, (top + row_height) as u32, 0xc, weights_head.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    let body = tel26_add_area(d, [0x6e, (top + row_height) as u32, 0x30c, bottom as u32, 0xe, weights_body.as_ptr() as u32, rows_dword, 0, 1, 0, u32::MAX]);
    let mut row: i32 = 0;
    staff_tab_cell(head, 0, 0, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, 0);
    // the previous and next statistics views: the symbol font's arrows or "<<" and ">>"
    for (col, ch, plain, id) in [(1u32, 0x2au32, 0x98e7c4usize, 0x3eu32), (2, 0x2b, 0x98e7a8, 0x3f)] {
        if read_i32(G_FONT_MODE) != -1 {
            staff_tab_format()(JOB_TEXT, 0x99ec1cusize, ch); // "%c"
            staff_tab_button(head, col, row, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 0, G_GUI_COLOUR_PALE_GREY, JOB_TEXT, id, 0);
        } else {
            staff_tab_button(head, col, row, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, G_GUI_COLOUR_PALE_GREY, plain, id, 0);
        }
    }
    let codes = history_tab_columns(p, second_view);
    let mut k = 0;
    while read_u8(codes + k) != 0xff {
        history_tab_stat_title(read_u8(codes + k));
        staff_tab_cell(head, k as u32 + 3, row, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
        k += 1;
    }
    let mut index: i32 = 0;
    if size > 0 {
        let mut e = career + 0xd9;
        loop {
            staff_tab_cell(body, 3, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, 0);
            staff_tab_cell(body, 4, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, 0);
            let id = read_i32(e);
            let club = if id > -1 {
                let club = club_at(id as isize).addr();
                staff_tab_format()(JOB_TEXT, 0x987ff0usize, club + 0x38); // "  %s"
                staff_tab_button(body, 1, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, JOB_TEXT, 0x7d5, club as u32);
                club
            } else if id == -2 {
                staff_tab_lang()(JOB_TEXT, 0xa8b0b0usize); // "  Unknown Club"
                staff_tab_cell(body, 1, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
                0
            } else {
                staff_tab_cell(body, 1, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, 0);
                0
            };
            add(0, read_u8(e + 4) as i32);
            add(2, read_u8(e + 6) as i32);
            // the season text (key_nation 0x669630): the year word pushed in a register whose high half still holds
            // the second total
            let season: unsafe extern "C" fn(u32, usize, usize, u32) = core::mem::transmute(0x669630usize);
            season((read_u32(t + 8) & 0xffff_0000) | read_u16(e - 2) as u32, club, JOB_TEXT, 2000);
            if read_i16(e + 0x12) >= 0 {
                if index != in_use {
                    staff_tab_button(body, 0, row, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT, 0x15, index as u32);
                } else {
                    staff_tab_cell(body, 0, row, 0x30, job_colour(G_GUI_COLOUR_PALE_GREY), 0xc, 1, G_GUI_COLOUR_DARK_BLUE, JOB_TEXT);
                }
                // the season's byte statistics +5..+0x11 into the totals 1..13, the signed word +0x12 into 14, the
                // words +0x14..+0x1e into the words of 15..17 and the dword +0x20 into 18
                for b in 1..14usize {
                    if b != 2 {
                        add(b, read_u8(e + 4 + b) as i32);
                    }
                }
                add(14, read_i16(e + 0x12) as i32);
                for w in 0..6usize {
                    let a = t + 0x3c + 2 * w;
                    write_i16(a, read_i16(a).wrapping_add(read_i16(e + 0x14 + 2 * w)));
                }
                add(18, read_i32(e + 0x20));
            } else {
                covered!("tel09_2 history tab loan season");
                staff_tab_cell(body, 0, row, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
                loaned_apps += read_u8(e + 4) as i32;
            }
            let mut fee = [0u8; 200];
            history_tab_fee_text(e, &mut fee);
            staff_tab_cell(body, 2, row, 2, 0, 0xc, 1, HISTORY_TAB_COLOUR_3152, fee.as_ptr() as usize);
            let codes = history_tab_columns(p, second_view);
            let mut k = 0;
            while read_u8(codes + k) != 0xff {
                let code = read_u8(codes + k);
                if read_i16(e + 0x12) < 0 {
                    if k >= 2 {
                        write_u8(JOB_TEXT, 0);
                    } else {
                        history_tab_stat_season(e + 4, code, 0);
                    }
                } else {
                    history_tab_stat_season(e + 4, code, 1);
                }
                history_tab_stat_cell(body, k as u32 + 5, row, code, G_GUI_COLOUR_YELLOW);
                k += 1;
            }
            index += 6;
            row += 1;
            e += 0x102;
            if index >= size {
                break;
            }
        }
    }
    staff_tab_cell(body, 0, row, 2, 0, 0xc, 2, G_GUI_COLOUR_PALE_GREY, 0);
    staff_tab_lang()(JOB_TEXT, 0xa8b098usize); // "  Total"
    staff_tab_cell(body, 1, row, 2, 0, 1, 2, HISTORY_TAB_COLOUR_3152, JOB_TEXT);
    for col in 2..5 {
        staff_tab_cell(body, col, row, 2, 0, 1, 2, HISTORY_TAB_COLOUR_3152, 0);
    }
    let codes = history_tab_columns(p, second_view);
    let mut k = 0;
    while read_u8(codes + k) != 0xff {
        let code = read_u8(codes + k);
        if code == 0x11 {
            add(0, loaned_apps.wrapping_neg());
        }
        if read_i32(t + 0x38) < 0 && k >= 2 {
            write_u8(JOB_TEXT, 0);
        } else {
            history_tab_stat_total(t, code);
        }
        history_tab_stat_cell(body, k as u32 + 5, row, code, HISTORY_TAB_COLOUR_3152);
        k += 1;
    }
    row += 1;
    while row < rows as i32 {
        history_tab_blank_row(body, row, 14);
        row += 1;
    }
    history_tab_keep_scroll(body);
}

/// The Injuries view: the person's injury history (`Injury` 0x62ec20 on `G_INJURY`), one row per injury.
unsafe fn history_tab_injuries(p: Staff, top: i32, bottom: i32, kind: u32) {
    let weights: [u8; 4] = [3, 0xb, 6, 6];
    let rows = ((bottom - top) / exe::font_get_height(2)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 4, weights.as_ptr() as u32, (kind & !0xff) | rows as u32, 0, 1, 0, u32::MAX]);
    let mut row: i32 = 0;
    let history: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ec20usize);
    let list = history(read_usize(G_INJURY), p.addr());
    if list != 0 {
        write_u32(list + 0xa, read_u32(list + 6));
        let mut e = exe::llist_get_prev(list);
        while e != 0 {
            if row == 0 {
                staff_tab_cell(area, 0, 0, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, 0);
                staff_tab_cell(area, 1, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, 0);
                staff_tab_lang()(JOB_TEXT, 0xa8b074usize, 0xad9c64usize); // "Type<%s - COMMENT - injury type>"
                staff_tab_cell(area, 2, row, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
                staff_tab_lang()(JOB_TEXT, 0xa8b068usize); // "Period Out"
                staff_tab_cell(area, 3, row, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
                row += 1;
            }
            history_tab_date(e + 0x20, JOB_TEXT);
            staff_tab_cell(area, 0, row, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
            let mut name = [0u8; 200];
            let type_name: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x62ef30usize);
            type_name(read_usize(G_INJURY), read_u8(e) as u32, name.as_mut_ptr() as usize, 200, 0);
            history_tab_capitalised(&name);
            staff_tab_cell(area, 1, row, 2, 0, 1, 2, G_GUI_COLOUR_YELLOW, JOB_TEXT);
            if read_u8(e + 0x1a) != 0xff {
                staff_tab_lang()(JOB_TEXT, 0xa8b03cusize, 0xad9c64usize); // "Match<%s - COMMENT - match injury>"
            } else {
                staff_tab_lang()(JOB_TEXT, 0x98e834usize); // "Training"
            }
            staff_tab_cell(area, 2, row, 2, 0, 0xc, 2, JOB_COLOUR_BDD4, JOB_TEXT);
            let days: unsafe extern "C" fn(i32, usize, u32, u32) = core::mem::transmute(0x7e5780usize);
            days(read_i16(e + 0x28) as i32, JOB_TEXT, 2000, 0);
            staff_tab_cell(area, 3, row, 2, 0, 0xc, 2, JOB_COLOUR_BDD4, JOB_TEXT);
            row += 1;
            e = exe::llist_get_prev(list);
        }
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
    }
    while row < rows as i32 {
        history_tab_blank_row(area, row, 4);
        row += 1;
    }
    history_tab_keep_scroll(area);
}

/// "  %c%s" of a name with its first letter in capitals (`utils_char_to_upper`) into the screens' buffer.
unsafe fn history_tab_capitalised(name: &[u8; 200]) {
    let first = exe::utils_char_to_upper(u32::from_le_bytes([name[0], name[1], name[2], name[3]]) as i32) as u32 & 0xff;
    staff_tab_format()(JOB_TEXT, 0xa8b060usize, first, name.as_ptr().add(1) as usize); // "  %c%s"
}

/// The Bans view: the person's ban history (`Discipline` 0x55cbc0 on `G_DISCIPLINE`, by the person's id), one row
/// per ban.
unsafe fn history_tab_bans(p: Staff, top: i32, bottom: i32, kind: u32) {
    let weights: [u8; 3] = [3, 0x11, 6];
    let rows = ((bottom - top) / exe::font_get_height(2)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 3, weights.as_ptr() as u32, (kind & !0xff) | rows as u32, 0, 1, 0, u32::MAX]);
    let mut row: i32 = 0;
    let history: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x55cbc0usize);
    let list = history(read_usize(G_DISCIPLINE), read_u32(p.addr()));
    if list != 0 {
        write_u32(list + 0xa, read_u32(list + 6));
        let mut e = exe::llist_get_prev(list);
        while e != 0 {
            if row == 0 {
                staff_tab_cell(area, 0, 0, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, 0);
                staff_tab_cell(area, 1, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, 0);
                staff_tab_lang()(JOB_TEXT, 0xa7e1b4usize); // "Reason"
                staff_tab_cell(area, 2, row, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
                row += 1;
            }
            history_tab_date(e + 4, JOB_TEXT);
            staff_tab_cell(area, 0, row, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
            let mut name = [0u8; 200];
            let competition: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x55cc20usize);
            competition(read_usize(G_DISCIPLINE), e, name.as_mut_ptr() as usize, 200);
            history_tab_capitalised(&name);
            staff_tab_cell(area, 1, row, 2, 0, 1, 2, G_GUI_COLOUR_YELLOW, JOB_TEXT);
            let reason: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x55d0a0usize);
            reason(read_usize(G_DISCIPLINE), e, JOB_TEXT, 2000);
            staff_tab_cell(area, 2, row, 2, 0, 0xc, 2, JOB_COLOUR_BDD4, JOB_TEXT);
            row += 1;
            e = exe::llist_get_prev(list);
        }
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
    }
    while row < rows as i32 {
        history_tab_blank_row(area, row, 3);
        row += 1;
    }
    history_tab_keep_scroll(area);
}

/// A label and value row of the Manager Stats view: the label in pale grey, the value (in the screens' buffer, written
/// by `value`) in yellow.
unsafe fn history_tab_stat_row(area: u32, row: &mut i32, label: usize, value: impl FnOnce()) {
    staff_tab_lang()(JOB_TEXT, label);
    staff_tab_cell(area, 0, *row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
    value();
    staff_tab_cell(area, 1, *row, 2, 0, 1, 2, G_GUI_COLOUR_YELLOW, JOB_TEXT);
    *row += 1;
}

/// "  %d" of a number into the screens' buffer.
unsafe fn history_tab_number(v: i32) {
    staff_tab_format()(JOB_TEXT, 0x990b48usize, v); // "  %d"
}

/// The players bought or sold line: "  %d" with none, else the count and the total value as money (mode 6).
unsafe fn history_tab_deals(count: i16, value: u32, fmt: usize) {
    if count == 0 {
        history_tab_number(0);
    } else {
        let mut money = [0u8; 200];
        let amount = exe::cash_get_display_amount(&value, 6);
        exe::cash_format_money(money.as_mut_ptr() as usize, 200, amount, 6);
        staff_tab_lang()(JOB_TEXT, fmt, count as i32, money.as_ptr() as usize);
    }
}

/// The Manager Stats view: the manager record (`Staff_Records::get_manager_record` with 1) as label and value rows.
unsafe fn history_tab_manager_stats(p: Staff, top: i32, bottom: i32, kind: u32) {
    let weights: [u8; 2] = [1, 3];
    let rows = ((bottom - top) / exe::font_get_height(2)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 2, weights.as_ptr() as u32, (kind & !0xff) | rows as u32, 0, 1, 0, u32::MAX]);
    let mut row: i32 = 0;
    let r = exe::staff_records_get_manager_record(read_usize(G_STAFF_RECORDS), p, 1);
    if r != 0 {
        covered!("tel09_2 history tab manager stats");
        history_tab_stat_row(area, &mut row, 0xa8b02c, || history_tab_number(read_i16(r + 0x8c) as i32)); // "  Leagues won"
        history_tab_stat_row(area, &mut row, 0xa8b020, || history_tab_number(read_i16(r + 0x8a) as i32)); // "  Cups won"
        history_tab_stat_row(area, &mut row, 0xa8b010, || history_tab_number(read_i16(r + 0x9e) as i32)); // "  Awards won"
        history_tab_stat_row(area, &mut row, 0xa8affc, || history_tab_number(read_i32(r + 0xba))); // "  Manager points"
        history_tab_stat_row(area, &mut row, 0xa8afec, || history_tab_number(read_i16(r + 0x8e) as i32)); // "  Games played"
        history_tab_stat_row(area, &mut row, 0xa8afe0, || history_tab_number(read_i16(r + 0x90) as i32)); // "  Games won"
        history_tab_stat_row(area, &mut row, 0xa8afd0, || history_tab_number(read_i16(r + 0x92) as i32)); // "  Games drawn"
        history_tab_stat_row(area, &mut row, 0xa8afc0, || history_tab_number(read_i16(r + 0x94) as i32)); // "  Games lost"
        history_tab_stat_row(area, &mut row, 0xa8afb0, || history_tab_number(read_i32(r + 0x96))); // "  Goals scored"
        history_tab_stat_row(area, &mut row, 0xa8af9c, || history_tab_number(read_i32(r + 0x9a))); // "  Goals conceded"
        // "  Players bought" / "  <%d - players bought> (total value <%s - total value>)"
        history_tab_stat_row(area, &mut row, 0xa8af88, || history_tab_deals(read_i16(r + 0xa0), read_u32(r + 0xa4), 0xa8af4c));
        // "  Players sold" / "  <%d - players sold> (total value <%s - total value>)"
        history_tab_stat_row(area, &mut row, 0xa8af3c, || history_tab_deals(read_i16(r + 0xa2), read_u32(r + 0xac), 0xa8af04));
    }
    while row < rows as i32 {
        history_tab_blank_row(area, row, 2);
        row += 1;
    }
    history_tab_keep_scroll(area);
}

/// The Filter view (the history records): the records of the kind in use (`Staff_Records` 0x86de70), a page of them
/// as date, club and the wrapped record text, and empty rows to the end.
unsafe fn history_tab_records(top: i32, bottom: i32, kind: u32, list: usize, page: usize, t: *mut usize) {
    let weights: [u8; 3] = [3, 5, 0xf];
    let rows = ((bottom - top) / exe::font_get_height(2)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 3, weights.as_ptr() as u32, (kind & !0xff) | rows as u32, 0, 1, 0, u32::MAX]);
    let mut row: i32 = 0;
    if list != 0 {
        covered!("tel09_2 history tab records");
        let n = read_i16(list);
        let width: i32 = if n > rows as i16 && n <= 100 { 0x2f6 } else { 0x30c };
        let x1 = (width * 8 - 0x370) / 0x17 + 0x6e;
        let mut index = (page as i16 as i32 - 1) * rows as i32;
        write_u32(list + 0xa, read_u32(list + 6));
        for _ in 0..index.max(0) {
            exe::llist_get_prev(list);
        }
        while (row < rows as i32 || read_i16(list) < 100) && index < read_i16(list) as i32 {
            let node = exe::llist_get_prev(list);
            history_tab_date(node, JOB_TEXT);
            staff_tab_cell(area, 0, row, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
            let id = read_i32(node + 8);
            let club = if id > -1 { club_at(id as isize).addr() } else { 0 };
            if club != 0 {
                staff_tab_format()(JOB_TEXT, 0x987ff0usize, club + 0x38); // "  %s"
                staff_tab_button(area, 1, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, JOB_TEXT, 0x7d5, club as u32);
            } else {
                staff_tab_cell(area, 1, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, 0);
            }
            let record_text: unsafe extern "thiscall" fn(usize, usize, *mut usize) = core::mem::transmute(0x86e0b0usize);
            record_text(read_usize(G_STAFF_RECORDS), node, t);
            staff_tab_cell(area, 2, row, 2, 0, 1, 2, G_GUI_COLOUR_YELLOW, 0);
            staff_tab_draw_wrapped(t, area, x1 as u32, width as u32, &mut row, 2, G_GUI_COLOUR_YELLOW, 2, 0);
            row += 1;
            index += 1;
        }
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
    }
    while row < rows as i32 {
        history_tab_blank_row(area, row, 3);
        row += 1;
    }
    history_tab_keep_scroll(area);
}

/// Draws the History tab of the staff profile (the person of screen parameter 0) between screen rows `top` and
/// `bottom`.
///
/// Without history (`job_for_club` 1 or 2, or an id from the person count up), only "There is no history information
/// available." in font 3. Otherwise a View button (command 0) with its popup: Achievements (bit 1 of screen parameter
/// 5), and for a player Playing Career (2), Injuries (4) and Bans (8), and Manager Stats (0x10) when the person has a
/// manager record; each line is command 0x14 with its bit, flags 0x2002 for the one in use, which is also the tab's
/// title. With Achievements (the history records) a Filter button with its popup (command 0x20): All Records (0),
/// Competitions (1), Awards (2), and for a club or national job 5 or 0xc Transfers (4) and Jobs (8), the kind in use
/// being screen parameter 0xe; the records of that kind (`Staff_Records` 0x86de70) and, when there are more than 100,
/// the page count in screen parameter 0x10 and the page buttons (`gui_add_page_buttons` with page parameter 0xf,
/// commands 0x21 and 0x22). The title (font 3, yellow) goes from row top + 0x19, and the table from top + 0x3c: the
/// records (`history_tab_records`), the playing career (`history_tab_playing_career`), the injuries, the bans or the
/// manager stats; the table area's scroll row comes from screen parameter 9 and its index goes to parameter 10 (no
/// table, nothing).
///
/// Defects of the original (kept): the seasons' club ids are only checked against -1 and -2, not against a null club.
/// Original: `staff_screens_draw_history_tab` (0x885820, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_draw_history_tab(top: i32, bottom: i32) {
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0);
    let p = Staff(get_screen_param(0));
    let view = get_screen_param(5) as u32;
    // parameters 6, 7, 8 (the playing career) are read again by its view; 0x14 is read and unused
    for k in [6u32, 7, 8] {
        get_screen_param(k);
    }
    let mut kind = get_screen_param(0xe) as u32;
    let page = get_screen_param(0xf);
    get_screen_param(0x14);
    let second_view = get_screen_param(0x15);
    let job = p.job_for_club();
    if job == 1 || job == 2 || (!p.is_null() && read_i32(p.addr()) >= read_i32(G_STAFF_COUNT)) {
        covered!("tel09_2 history tab without information");
        staff_tab_no_information(top, bottom, 0xa8aed8); // "There is no history information available."
        exe::drop_text(t);
        return;
    }
    covered!("tel09_2 history tab");
    let mut title = [0u8; 200];
    let view_area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x168, (top + 0x14) as u32, 2, 0, 1, 0, 1, 0, u32::MAX]);
    staff_tab_lang()(JOB_TEXT, 0x988004usize); // "View"
    let button = staff_tab_button(view_area, 0, 0, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT, 0, 0);
    let menu = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), button]);
    staff_tab_lang()(JOB_TEXT, 0xa8b124usize); // " Achievements"
    let filter = view & 1;
    history_tab_view_line(view, 1, menu, &mut title);
    if !p.player().is_null() {
        staff_tab_lang()(JOB_TEXT, 0xa8b114usize); // " Playing Career"
        history_tab_view_line(view, 2, menu, &mut title);
        staff_tab_lang()(JOB_TEXT, 0xa7fde4usize); // " Injuries"
        history_tab_view_line(view, 4, menu, &mut title);
        staff_tab_lang()(JOB_TEXT, 0xa8b10cusize); // " Bans"
        history_tab_view_line(view, 8, menu, &mut title);
    }
    if exe::staff_records_get_manager_record(read_usize(G_STAFF_RECORDS), p, 0) != 0 {
        staff_tab_lang()(JOB_TEXT, 0xa8b0fcusize); // " Manager Stats"
        history_tab_view_line(view, 0x10, menu, &mut title);
    }
    let mut list = 0usize;
    if filter != 0 {
        covered!("tel09_2 history tab filter");
        staff_tab_lang()(JOB_TEXT, 0x98ebf8usize); // "Filter"
        let button = staff_tab_button(view_area, 1, 0, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT, 0, 0);
        let menu = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), button]);
        let line = |s: usize, value: u32| {
            staff_tab_lang()(JOB_TEXT, s);
            history_tab_menu_line(kind == value, 0x20, value, menu);
        };
        line(0xa8b0ec, 0); // " All Records"
        line(0xa8b0dc, 1); // " Competitions"
        line(0xa8b0d4, 2); // " Awards"
        if matches!(p.job_for_club(), 5 | 0xc) || matches!(p.job_for_nation(), 5 | 0xc) {
            line(0xa8b0c8, 4); // " Transfers"
            line(0xa8b0c0, 8); // " Jobs"
        }
        let records: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86de70usize);
        list = records(read_usize(G_STAFF_RECORDS), p.addr(), kind);
        let rows = ((bottom - top - 0x3c) / exe::font_get_height(2)) as u8;
        kind = (kind & !0xff) | rows as u32;
        if list != 0 && read_i16(list) > 100 {
            covered!("tel09_2 history tab pages");
            let n = read_i16(list) as i32;
            let pages = (n % rows as i32 != 0) as i32 + n / rows as i32;
            set_screen_param(0x10, pages as i16 as i32 as usize);
            let page_buttons: unsafe extern "C" fn(u32, i32, usize, i32, u32, u32, u32, u32) = core::mem::transmute(0x5ede20usize);
            page_buttons(0x30c, top, page, pages, 0, 0x21, 0x22, 0);
        }
    }
    let add_text: unsafe extern "thiscall" fn(usize, u32, i32, u32, i32, u32, u32, u32, u32, u32, usize, u32) -> u32 =
        core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    add_text(G_SCREEN_MANAGER, 0x6e, top + 0x19, 0x30c, top + 0x37, 2, 0, 0xc, 3, job_colour(G_GUI_COLOUR_YELLOW), title.as_ptr() as usize, 1);
    let top = top + 0x3c;
    if filter != 0 {
        history_tab_records(top, bottom, kind, list, page, t);
    } else if view & 2 != 0 {
        covered!("tel09_2 history tab playing career");
        history_tab_playing_career(p, top, bottom, kind, second_view);
    } else if view & 4 != 0 {
        covered!("tel09_2 history tab injuries");
        history_tab_injuries(p, top, bottom, kind);
    } else if view & 8 != 0 {
        covered!("tel09_2 history tab bans");
        history_tab_bans(p, top, bottom, kind);
    } else if view & 0x10 != 0 {
        history_tab_manager_stats(p, top, bottom, kind);
    }
    exe::drop_text(t);
}

/// Comparer for `staff_screens_draw_history_tab`: both draw side by side and are compared by the display list
/// (`compare_match_screen_build`, which also holds the screen parameters the tab writes: the page count 0x10 and the
/// table area 10).
pub(crate) unsafe extern "C" fn compare_staff_screens_draw_history_tab(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(%%I0%%);
    compare_match_screen_build("staff profile history tab", &[], || o(top, bottom), || staff_screens_draw_history_tab(top, bottom));
}
