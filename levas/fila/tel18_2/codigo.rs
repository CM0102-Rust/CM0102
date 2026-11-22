// ---------- batch %%LEVA%%: the transfers screen (transfer_screens.cpp, part 2: tel18_2) ----------

/// `TRANSFER_MANAGER::get_past_transfers(month from, year from, month to, year to, &list, &count)` (0x8cc030, thiscall
/// ret 0x18): a malloc'd array of 0x25-byte records.
const TS_FN_GET_PAST_TRANSFERS: usize = 0x8cc030;
/// `transfer_manager_compare_past_transfers_by_fee` (0x8e54f0), the qsort comparer.
const TS_PAST_TRANSFERS_BY_FEE: usize = 0x8e54f0;
/// `date_get_month_name(month)` (0x54d3b0, cdecl): the month's name.
const TS_FN_MONTH_NAME: usize = 0x54d3b0;
/// `background_set_nation_picture(nation, 0, 0)` (0x41c100, cdecl).
const TS_FN_NATION_PICTURE: usize = 0x41c100;
/// `Date::get_short_weekday_date_string_no_year(buffer, size, 0xff)` (0x54b550, thiscall ret 0xc).
pub(crate) const TS_FN_SHORT_DATE: usize = 0x54b550;
/// `gui_add_page_buttons(x, y, page, pages, id first, id previous, id next, id last)` (0x5ede20, cdecl).
pub(crate) const TS_FN_PAGE_BUTTONS: usize = 0x5ede20;
/// The size of a past transfer record (`past_transfer`): the date is its last field.
const TS_PAST_TRANSFER_SIZE: usize = 0x25;

/// A popup item line of the transfers screen's menus: `selected` gives the 0x2002 flags and no id, else flags 2 with
/// `id` and `value`; the text is `TS_TEXT`, the area `area`.
unsafe fn ts_menu_line(selected: bool, id: u32, value: u32, area: u32) -> u32 {
    let d = current_human_display();
    let (flags, id, value) = if selected { (0x2002, 0, 0) } else { (2, id, value) };
    tel26_add_item(d, [flags, 0, 0, 0, 0, 0, 0, 0x10, ts_colour(TS_COLOUR_POPUP), ts_colour(G_GUI_COLOUR_YELLOW), 1, 1,
        ts_colour(G_GUI_COLOUR_BLACK), TS_TEXT as u32, 0, id, value, area])
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
    original_fn::<unsafe extern "C" fn()>(%%I0%%)()
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
    let sorted_nations: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x66a0c0usize);
    let split_new: unsafe extern "thiscall" fn(*mut u8, u32, usize, u32) = core::mem::transmute(0x66a280usize);
    let split_parts: unsafe extern "thiscall" fn(*mut u8) -> i8 = core::mem::transmute(0x66a350usize);
    let split_second: unsafe extern "thiscall" fn(*mut u8, usize) -> u8 = core::mem::transmute(0x66a430usize);
    let split_header: unsafe extern "thiscall" fn(*mut u8, usize, u32, u32) = core::mem::transmute(0x66a360usize);
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
    let order = sorted_nations(1);
    let mut split = [0u8; 16];
    split_new(split.as_mut_ptr(), 0, order, 1);
    // the original's part area starts as param 3 (the variable's slot is reused); it is set before use when the split
    // has one or two parts
    let mut part_area = by_month;
    for i in 0..0x23usize {
        let n = read_usize((G_LEAGUE_TABLE as isize + read_i8(order + i) as isize * 0x48) as usize);
        if n == 0 || read_u8(n + 0x11c) == 0 {
            continue;
        }
        if split_parts(split.as_mut_ptr()) > 1 && split_second(split.as_mut_ptr(), n) != group {
            group = split_second(split.as_mut_ptr(), n);
            split_header(split.as_mut_ptr(), TS_TEXT, 0x7d0, group as u32);
            let item = ts_menu_line(false, 0, 0, menu);
            part_area = tel26_add_area(d, [0, 0, 0, 0, 0, 0, 0, 0, 0x30, ts_colour(TS_COLOUR_POPUP), item]);
        } else if split_parts(split.as_mut_ptr()) == 1 {
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
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
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
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%);
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
    original_fn::<unsafe extern "C" fn()>(%%I3%%)()
}
