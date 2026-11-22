// ---------- batch %%LEVA%%: the manager stats screen (manager_screens.cpp) ----------

/// The colour word at 0xae31aa (no name yet), pushed as the first colour of every Manager Stats menu entry.
const MANAGER_STATS_MENU_COLOUR: usize = 0xae31aa;
/// The colour word at 0xae3192 (no name yet), the cell colour of the statistic column.
const MANAGER_STATS_CELL_COLOUR: usize = 0xae3192;
/// The size of one manager record copied into the screen's list (`manager_record`, 210 bytes).
const MANAGER_STATS_RECORD_SIZE: usize = 0xd2;
/// The qsort comparer of the list (0x6b36f0, still in the original) and the two statics it reads: the nation filter
/// (i32) and the chosen statistic (byte).
const MANAGER_STATS_SORT_COMPARER: usize = 0x6b36f0;
const MANAGER_STATS_SORT_NATION: usize = 0xa14f54;
const MANAGER_STATS_SORT_STAT: usize = 0xa14f58;
/// The menu entries of the statistic button, in order (statistic 1..16): the text (each starting with a space) and
/// whether a separator follows it.
const MANAGER_STATS_MENU: [(usize, bool); 16] = [
    (0, false), // 1: " World Reputation" or " Reputation in <nation>", written before the loop
    (0xa152dc, true), // " Manager Points"
    (0xa152c4, false), // " Period at Current Club"
    (0xa152ac, false), // " Number of Manager Jobs"
    (0xa15290, false), // " Longest Period at a Club"
    (0xa15274, true), // " Shortest Period at a Club"
    (0xa15260, false), // " Number of Cups Won"
    (0xa15248, false), // " Number of Leagues Won"
    (0xa15230, false), // " Number of Awards Won"
    (0xa15218, true), // " Overall Match Record"
    (0xa151fc, false), // " Number of Players Bought"
    (0xa151e4, false), // " Number of Players Sold"
    (0xa151c8, false), // " Highest Transfer Fee Paid"
    (0xa151a8, false), // " Highest Transfer Fee Received"
    (0xa1518c, false), // " Total Transfer Fees Paid"
    (0xa1516c, false), // " Total Transfer Fees Received"
];
/// The reputation words below which the reputation column reads "Unknown", and the rating bands (the lowest bound of
/// each text, from the top).
const MANAGER_STATS_UNKNOWN_BELOW: i16 = 0xea6;
const MANAGER_STATS_REPUTATION_BANDS: [(i16, usize); 7] = [
    (0x251c, 0xa150dc), // "World Class"
    (0x2134, 0xa150b4), // "Superb"
    (0x1d4c, 0xa15088), // "Very Good"
    (0x1964, 0xa15060), // "Good"
    (0x1482, 0xa15038), // "Fair"
    (0xea6, 0xa1500c),  // "Average"
    (0x4e2, 0xa14fe4),  // "Poor"
];

/// One item of the current human's `Display` with the menu colours (0xae31aa, yellow, black, all three short fields of
/// the widget, passed zero-extended).
unsafe fn manager_stats_add_item(kind: usize, style: usize, text: usize, a: usize, b: usize, c: usize, area: usize) -> usize {
    let row: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    row(current_human_display(), kind, 0, 0, 0, 0, 0, 0, style, colour_word(MANAGER_STATS_MENU_COLOUR),
        colour_word(G_GUI_COLOUR_YELLOW), 1, 1, colour_word(G_GUI_COLOUR_BLACK), text, a, b, c, area)
}

/// Copies the text after the first character of the work text (`0xde1f64` + 1) into `header`, as the original's
/// inline strcpy does, and returns the byte count it copied (the original leaves it in `eax`).
unsafe fn manager_stats_copy_header(header: &mut [u8; 200]) -> usize {
    let mut i = 0;
    loop {
        let b = read_u8(0xde1f65 + i);
        header[i] = b;
        i += 1;
        if b == 0 {
            return i;
        }
    }
}

/// The top of the Manager Stats screen: the background, the title, the menu bar, the heading box, the statistic
/// button with its 16-entry menu and the heading that names the chosen statistic.
///
/// The background (`exe::background_set_random_picture`), the title "Manager Stats" (`FN_LANGLIB_SPRINTF` into
/// `0xde1f64`, `exe::screenmanager_add_screen_title` in dark blue and pale grey), the menu bar (`exe::menubar_build`
/// with 4), an empty heading box (`FN_SCREENMANAGER_ADD_TEXT_ITEM` at 100,0x50..0x316,0x7d), the statistic button
/// (`FN_SCREENMANAGER_ADD_BUTTON_ITEM` at 0x69,0x55..0x78,0x69) and its menu area (`FN_DISPLAY_ADD_AREA`). The menu
/// holds the 16 statistics: the first is " World Reputation" without a nation filter (`nation` < 0) or
/// " Reputation in <nation>" (`nation::SHORT_NAME` with its gender byte `nation::SHORT_GENDER_NAME` twice); a
/// separator follows statistics 2, 6 and 10. The chosen one (`stat`) is the selected entry (kind 0x2002, no id) and its
/// text without the leading space becomes the heading; the others are kind 2 with id 2 and the statistic number. The
/// heading box is drawn again with that text in font 1. The colours that ride in a register whose high word is known
/// (the return of the previous call) are rebuilt with it (`colour_with_high_bits`); the others are zero-extended.
unsafe fn manager_stats_menu(nation: i32, stat: u8, header: &mut [u8; 200]) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let txt: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let caixa: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let list: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    const BUF: usize = 0xde1f64;
    let sm = SCREEN_MANAGER;
    exe::background_set_random_picture();
    let r = text(BUF, 0xa15324) as usize;
    exe::screenmanager_add_screen_title(sm, colour_word(G_GUI_COLOUR_DARK_BLUE) as u32, colour_with_high_bits(r, G_GUI_COLOUR_PALE_GREY) as u32, BUF);
    exe::menubar_build(4, 0);
    let r = txt(sm, 100, 0x50, 0x316, 0x7d, 2, 0, 0xc, 6, colour_word(G_GUI_COLOUR_YELLOW), EMPTY_TEXT, 1);
    let r = caixa(sm, 0x69, 0x55, 0x78, 0x69, 0x30, colour_word(G_GUI_COLOUR_BLACK), 0xc, 1, colour_with_high_bits(r, G_GUI_COLOUR_PALE_GREY), 0, 0, 0);
    let area = list(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, colour_word(MANAGER_STATS_MENU_COLOUR), r);
    header[0] = 0;
    if nation > -1 {
        let n = read_usize(G_NATION_TABLE) + nation as usize * nation::SIZE;
        let g = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
        text(BUF, 0xa15300, g, g, n + nation::SHORT_NAME);
    } else {
        text(BUF, 0xa152ec);
    }
    // the high word of the last register before the heading box: the strcpy count or the item index
    let mut last = 0usize;
    for (k, &(label, separator)) in MANAGER_STATS_MENU.iter().enumerate() {
        let k = k as u8 + 1;
        if label != 0 {
            text(BUF, label);
        }
        if stat == k {
            covered!("tel26_5 manager stats selected statistic");
            manager_stats_add_item(0x2002, 0x10, BUF, 0, 0, 0, area);
            last = manager_stats_copy_header(header);
        } else {
            last = manager_stats_add_item(2, 0x10, BUF, 0, 2, k as usize, area);
        }
        if separator {
            last = manager_stats_add_item(1, 0x1000010, 0, 0, 0, 0, area);
        }
    }
    txt(sm, 0x64, 0x50, 0x316, 0x7d, 1, 0, 0xc, 6, colour_with_high_bits(last, G_GUI_COLOUR_YELLOW), header.as_ptr() as usize, 1);
}

/// The nation filter of the Manager Stats screen: the button and its menu, "All Nations" and the selected key nations.
///
/// The button "Nation" (`FN_SCREENMANAGER_ADD_BUTTON_ITEM` at 0x6e,0x91..0xeb,0xa5, grey and pale grey) and its menu
/// area; " All Nations" is selected without a filter (`nation` = -1), or else kind 2 with id 1 and value -1; a
/// separator. Then the 35 key nations in screen order (`key_nation_sorted_order` 0x66a0c0 with 1) whose
/// `nation::LEAGUE_SELECTED` has bit 0 or 1: with more than 16 of them (`NATION_MENU_SPLIT::part_count` 0x66a350 =
/// 2) the menu splits in two parts, each opened by its header ("Nations A-x", "Nations y-Z",
/// `NATION_MENU_SPLIT::format_header` 0x66a360) and its own area; each nation is " <short name>" (crt sprintf),
/// selected when it is the filter, else kind 2 with id 1 and the nation id.
///
/// The last part seen (0xff at first) is the byte right after the four column weights of the list (`cols[4]`): the
/// original keeps both in the same stack bytes (the weights are passed with their count, so the byte is not read as a
/// weight). The part area starts as an uninitialised local in the original (`edi`); it is always set before use, here
/// 0.
unsafe fn manager_stats_nation_menu(nation: i32, cols: &mut [u8; 8]) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let caixa: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let list: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    // key_nation.cpp: key_nation_sorted_order (cdecl), NATION_MENU_SPLIT ctor / part_count / format_header /
    // in_second_part (thiscall; ret 0xc, 0, 0xc, 4), checked against the disassembly
    let sorted_order: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x66a0c0usize);
    let split_new: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> usize = core::mem::transmute(0x66a280usize);
    let part_count: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x66a350usize);
    let format_header: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x66a360usize);
    let in_second_part: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x66a430usize);
    const BUF: usize = 0xde1f64;
    let sm = SCREEN_MANAGER;
    text(BUF, 0x9ca010);
    let r = caixa(sm, 0x6e, 0x91, 0xeb, 0xa5, 0x30, colour_word(G_GUI_COLOUR_GREY), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), BUF, 0, 0);
    let area = list(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x130, colour_with_high_bits(r, MANAGER_STATS_MENU_COLOUR), r);
    text(BUF, 0x990da4);
    if nation == -1 {
        manager_stats_add_item(0x2002, 0x10, BUF, 0, 0, 0, area);
    } else {
        manager_stats_add_item(2, 0x10, BUF, 0, 1, usize::MAX, area);
    }
    manager_stats_add_item(1, 0x1000010, 0, 0, 0, 0, area);
    let order = sorted_order(1);
    let mut split = [0u8; 16];
    let sp = split.as_mut_ptr() as usize;
    split_new(sp, 0, order, 1);
    let mut part_area = 0usize; // ponytail: uninitialised in the original, always set before use
    for k in 0..0x23usize {
        let e = read_usize(G_LEAGUE_TABLE + (read_i8(order + k) as isize * 0x48) as usize);
        if e == 0 || read_u8(e + nation::LEAGUE_SELECTED) & 3 == 0 {
            continue;
        }
        let mut new_part = false;
        if part_count(sp) as u8 as i8 > 1 && in_second_part(sp, e) as u8 != cols[4] {
            covered!("tel26_5 manager stats nation menu part");
            cols[4] = in_second_part(sp, e) as u8;
            format_header(sp, BUF, 0x7d0, u32::from_le_bytes([cols[4], cols[5], cols[6], cols[7]]));
            let item = manager_stats_add_item(2, 0x10, BUF, 0, 0, 0, area);
            part_area = list(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, colour_word(MANAGER_STATS_MENU_COLOUR), item);
            new_part = true;
        }
        if !new_part && part_count(sp) as u8 == 1 {
            part_area = area;
        }
        format(BUF, 0x98ed14, e + nation::SHORT_NAME);
        let id = read_i32(e);
        if id == nation {
            manager_stats_add_item(0x2002, 0x10, BUF, 0, 0, 0, part_area);
        } else {
            manager_stats_add_item(2, 0x10, BUF, 0, 1, id as usize, part_area);
        }
    }
}

/// Builds the list of the Manager Stats screen: the manager record of every club or national team manager that
/// passes the nation filter, sorted by the chosen statistic. Returns the list and its count, or `None` when the
/// allocation failed (the error box was shown).
///
/// For every person (`G_STAFF_TABLE`, `G_STAFF_COUNT`) with a non-player sheet (`staff::NON_PLAYER`) who is a
/// manager (job 5 or 12) at his club (`staff::JOB_FOR_CLUB`) or national team (`staff::JOB_FOR_NATION`):
/// `exe::staff_records_get_manager_record` (creating it); without a filter (`nation` <= -1) every record, else only
/// the ones whose `manager_record::SEASON_NATION` or `NATIONAL_NATION` is the filter. The list grows by 50 records
/// (`exe::crt_realloc`); the record is copied whole (the original copies it field by field, with `Date::operator=`
/// and `run_record_assign` for the dates and runs: the same 210 bytes). With records, the statics of the comparer get
/// the filter and the statistic and the list is sorted (`exe::crt_qsort`, comparer 0x6b36f0). The count starts from
/// the screen's (parameter 2).
///
/// Defect of the original: when `crt_realloc` fails the old block is lost (the pointer is overwritten with 0 before
/// the check), and the screen returns after the error box without its parameters.
unsafe fn manager_stats_collect(nation: i32, stat: u8, mut count: i32) -> Option<(usize, i32)> {
    let is_manager = |j: u8| j == 5 || j == 12;
    let mut list = 0usize;
    let mut k = 0i32;
    while k < read_i32(G_STAFF_COUNT) {
        let s = read_usize(G_STAFF_TABLE) + k as usize * staff::SIZE;
        k += 1;
        if read_usize(s + staff::NON_PLAYER) == 0
            || !(is_manager(read_u8(s + staff::JOB_FOR_CLUB)) || is_manager(read_u8(s + staff::JOB_FOR_NATION))) {
            continue;
        }
        let rec = exe::staff_records_get_manager_record(read_usize(G_STAFF_RECORDS), s, 1);
        if rec == 0 {
            continue;
        }
        if nation > -1 && read_i32(rec + manager_record::SEASON_NATION) != nation && read_i32(rec + manager_record::NATIONAL_NATION) != nation {
            continue;
        }
        if count % 50 == 0 {
            list = exe::crt_realloc(list, (count as usize * MANAGER_STATS_RECORD_SIZE + 0x2904) as u32);
            if list == 0 {
                covered!("tel26_5 manager stats allocation error");
                show_error_at(0xa15138, 0x1ab);
                return None;
            }
        }
        core::ptr::copy_nonoverlapping(rec as *const u8, (list + count as usize * MANAGER_STATS_RECORD_SIZE) as *mut u8, MANAGER_STATS_RECORD_SIZE);
        count += 1;
    }
    if count != 0 {
        write_i32(MANAGER_STATS_SORT_NATION, nation);
        write_u8(MANAGER_STATS_SORT_STAT, stat);
        exe::crt_qsort(list, count as u32, MANAGER_STATS_RECORD_SIZE as u32, MANAGER_STATS_SORT_COMPARER);
    }
    Some((list, count))
}

/// The money statistics: the 32-bit fee through a local `Cash` (`exe::cash_get_display_amount` and
/// `exe::cash_format_money` with flags 5) into the work text.
unsafe fn manager_stats_money(fee: u32) {
    let cash = fee;
    let amount = exe::cash_get_display_amount(&cash, 5);
    exe::cash_format_money(0xde1f64usize, 0x7d0, amount, 5);
}

/// Writes the statistic column of one row into the work text (`0xde1f64`).
///
/// 1 reputation: the non-player sheet's world reputation without a filter, else the current one; "Unknown" below
/// 3750 when the manager is a human (one of the last 16 people) with fewer than 50 games, or a computer manager
/// younger than 35 (`staff::AGE`); else the band (World Class 9500, Superb 8500, Very Good 7500, Good 6500, Fair
/// 5250, Average 3750, Poor 1250, else Very Poor). 2 points: `YEAR_POINTS` without a filter, else the season points
/// of the club (`SEASON_POINTS`) and of the national team (`NATIONAL_SEASON_POINTS`) whose nation is the filter.
/// 3 the days since `staff::DATE_JOINED_CLUB` (`exe::date_days_since` from `G_CURRENT_DATE`; "-" without a club).
/// 4 club plus national jobs. 5 and 6 the days of the longest and shortest job ("-" without one). 7 cups, 8 leagues,
/// 9 awards, 11 bought, 12 sold. 10 "P<games>/W<wins>" ("-" with no game). 13 and 14 the biggest purchase and sale fee,
/// 15 and 16 the purchases and sales totals ("-" when zero; the totals are 64-bit, but only their low dword is
/// shown: defect of the original). Any other statistic: "???".
unsafe fn manager_stats_stat_text(stat: u8, nation: i32, s: usize, rec: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    const BUF: usize = 0xde1f64;
    const DASH: usize = 0x98ba0c;
    let number = |v: i32| {
        format(BUF, 0x98c11c, v);
    };
    let days = |end: usize, start: usize| {
        let d = exe::date_days_since(end, read_u32(start), read_u32(start + 4));
        text(BUF, 0xa14fa4, d);
    };
    let human_from = read_i32(G_STAFF_COUNT) - 0x10;
    let i16_ = |o: usize| read_i16(rec + o) as i32;
    match stat {
        1 => {
            let np = read_usize(s + staff::NON_PLAYER);
            let rep = if nation == -1 { read_i16(np + non_player::WORLD_REPUTATION) } else { read_i16(np + non_player::CURRENT_REPUTATION) };
            if rep < MANAGER_STATS_UNKNOWN_BELOW
                && ((read_i32(s) >= human_from && read_i16(rec + manager_record::GAMES) < 0x32)
                    || (read_i32(rec + manager_record::OWNER) < human_from && read_i8(s + staff::AGE) < 0x23)) {
                covered!("tel26_5 manager stats unknown reputation");
                text(BUF, 0xa1510c, EMPTY_TEXT);
            } else {
                let label = MANAGER_STATS_REPUTATION_BANDS.iter().find(|b| rep >= b.0).map_or(0xa14fb8, |b| b.1);
                text(BUF, label, EMPTY_TEXT);
            }
        }
        2 => {
            if nation == -1 {
                number(read_i32(rec + manager_record::YEAR_POINTS));
            } else {
                let mut v = 0i32;
                if read_i32(rec + manager_record::SEASON_NATION) == nation {
                    v = read_i32(rec + manager_record::SEASON_POINTS);
                }
                if read_i32(rec + manager_record::NATIONAL_NATION) == nation {
                    v = v.wrapping_add(read_i32(rec + manager_record::NATIONAL_SEASON_POINTS));
                }
                number(v);
            }
        }
        3 if read_usize(s + staff::CLUB_JOB) != 0 => days(G_CURRENT_DATE, s + staff::DATE_JOINED_CLUB),
        4 => number(read_u8(rec + manager_record::NATIONAL_JOBS) as i32 + read_u8(rec + manager_record::CLUB_JOBS) as i32),
        5 if read_i32(rec + manager_record::LONGEST_CLUB) > -1 => days(rec + manager_record::LONGEST_END, rec + manager_record::LONGEST_START),
        6 if read_i32(rec + manager_record::SHORTEST_CLUB) > -1 => days(rec + manager_record::SHORTEST_END, rec + manager_record::SHORTEST_START),
        7 => number(i16_(manager_record::CUPS_WON)),
        8 => number(i16_(manager_record::LEAGUES_WON)),
        9 => number(i16_(manager_record::AWARDS)),
        10 if read_i16(rec + manager_record::GAMES) != 0 => {
            text(BUF, 0xa14f88, i16_(manager_record::GAMES), i16_(manager_record::WINS));
        }
        11 => number(i16_(manager_record::PLAYERS_BOUGHT)),
        12 => number(i16_(manager_record::PLAYERS_SOLD)),
        13 if read_u32(rec + manager_record::BIGGEST_PURCHASE_FEE) != 0 => manager_stats_money(read_u32(rec + manager_record::BIGGEST_PURCHASE_FEE)),
        14 if read_u32(rec + manager_record::BIGGEST_SALE_FEE) != 0 => manager_stats_money(read_u32(rec + manager_record::BIGGEST_SALE_FEE)),
        15 if read_u64(rec + manager_record::PURCHASES_TOTAL) != 0 => manager_stats_money(read_u32(rec + manager_record::PURCHASES_TOTAL)),
        16 if read_u64(rec + manager_record::SALES_TOTAL) != 0 => manager_stats_money(read_u32(rec + manager_record::SALES_TOTAL)),
        3 | 5 | 6 | 10 | 13..=16 => copy_to_work_text(DASH),
        _ => copy_to_work_text(0xa14f84),
    }
}

/// One row of the Manager Stats list: the place, the name, the club or national team and the statistic.
///
/// The place (`exe::utils_format_ordinal` of `idx` + 1) in column 0; the name (`exe::database_format_staff_name`
/// into a local, "  <name>") as a link 0x7d7 to the person in column 1, yellow for a human manager (one of the last 16
/// people) and pale grey otherwise. Column 2: for statistics 5 and 6 with a longest or shortest job, that club
/// (`exe::club_get_national_team_name`, link 0x7d5); otherwise the club and the national team ("club/nation") when
/// he manages both, the one he manages, or "No Club/Nation" as plain text. Column 3: the statistic
/// (`manager_stats_stat_text`). Colours whose high word the original takes from a known register are rebuilt with
/// it (the ordinal's pointer, the person id, the club index x 0x91, the text routine's return).
///
/// Defect of the original: the name goes into a 200-byte stack buffer with a size of 2000 (here a 2000-byte buffer, the
/// same text for any real name).
unsafe fn manager_stats_row(stat: u8, nation: i32, area: usize, row: i32, idx: i32, rec: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let cell: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let text_of_cell: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    const BUF: usize = 0xde1f64;
    let sm = SCREEN_MANAGER;
    let row_ = row as usize;
    let s = read_usize(G_STAFF_TABLE) + read_i32(rec + manager_record::OWNER) as usize * staff::SIZE;
    let ord = exe::utils_format_ordinal(idx + 1, -1);
    text_of_cell(sm, area, 0, row_, 0x30, colour_word(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, colour_with_high_bits(ord, G_GUI_COLOUR_PALE_GREY), ord);
    let mut name = [0u8; 0x7d0];
    exe::database_format_staff_name(s, name.as_mut_ptr() as usize, 0x7d0, 1);
    format(BUF, 0x987ff0, name.as_ptr() as usize);
    let human_from = read_i32(G_STAFF_COUNT) - 0x10;
    let id = read_i32(s);
    let colour = if id < human_from {
        colour_with_high_bits(human_from as usize, G_GUI_COLOUR_PALE_GREY)
    } else {
        covered!("tel26_5 manager stats human row");
        colour_with_high_bits(id as usize, G_GUI_COLOUR_YELLOW)
    };
    cell(sm, area, 1, row_, 1, 0, 1, 3, colour, BUF, 0x7d7, s);
    // column 2
    let job_club = match stat {
        5 => read_i32(rec + manager_record::LONGEST_CLUB),
        6 => read_i32(rec + manager_record::SHORTEST_CLUB),
        _ => -1,
    };
    if job_club > -1 {
        let club = club_at(job_club as isize).addr();
        exe::club_get_national_team_name(club, 0usize, BUF, 0x7d0, 1, 1);
        let hi = (job_club as usize).wrapping_mul(0x91);
        cell(sm, area, 2, row_, 1, 0, 1, 2, colour_with_high_bits(hi, G_GUI_COLOUR_YELLOW), BUF, 0x7d5, club);
    } else {
        let is_manager = |j: u8| j == 5 || j == 12;
        let club = read_usize(s + staff::CLUB_JOB);
        let national = read_usize(s + staff::NATIONAL_JOB);
        let at_club = club != 0 && is_manager(read_u8(s + staff::JOB_FOR_CLUB));
        let at_nation = national != 0 && is_manager(read_u8(s + staff::JOB_FOR_NATION));
        let data = if at_club && at_nation {
            covered!("tel26_5 manager stats club and nation");
            let (mut a, mut b) = ([0u8; 200], [0u8; 200]);
            exe::club_get_national_team_name(club, 0usize, a.as_mut_ptr() as usize, 0xc8, 1, 1);
            exe::club_get_national_team_name(national, 0usize, b.as_mut_ptr() as usize, 0xc8, 1, 1);
            format(BUF, 0x9ce448, a.as_ptr() as usize, b.as_ptr() as usize);
            club
        } else if at_club {
            exe::club_get_national_team_name(club, 0usize, BUF, 0x7d0, 1, 1);
            club
        } else if at_nation {
            exe::club_get_national_team_name(national, 0usize, BUF, 0x7d0, 1, 1);
            national
        } else {
            0
        };
        if data != 0 {
            cell(sm, area, 2, row_, 1, 0, 1, 2, colour_word(G_GUI_COLOUR_YELLOW), BUF, 0x7d5, data);
        } else {
            let r = text(BUF, 0x9ce438) as usize;
            text_of_cell(sm, area, 2, row_, 1, 0, 1, 2, colour_with_high_bits(r, G_GUI_COLOUR_YELLOW), BUF);
        }
    }
    manager_stats_stat_text(stat, nation, s, rec);
    text_of_cell(sm, area, 3, row_, 0x30, colour_word(MANAGER_STATS_CELL_COLOUR), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), BUF);
}

/// Builds the Manager Stats screen, which lists managers ranked by a chosen statistic with a nation filter.
///
/// Parameters of the screen: 0 the nation filter (an id, or -1), 1 the list (0 = build it), 2 its count, 3 the page,
/// 5 the remembered scroll row, 7 "scroll to the human" (1 when opened), 8 the statistic (1..16). The top
/// (`manager_stats_menu`) and the nation filter (`manager_stats_nation_menu`), then, without a list, the list is built
/// (`manager_stats_collect`) and written to parameters 1 and 2; an empty build shows "There are no manager stats
/// available." (`FN_GUI_ADD_TEXT_AREA`, as many lines as 0x15d / `exe::font_get_height(3)`). Otherwise the rows per page
/// are 0x15d / font height (a byte); the first human is the first record whose `manager_record::OWNER` is one of the last 16
/// people. Up to 100 records: one page (parameters 4 = 1 and 3 = 1 when above it). Above 100: with parameter 7 the
/// page of the first human goes to parameter 3 and 7 is cleared; the page count (rounded up) to parameter 4, the page
/// clamped to it, and the page buttons (`gui_add_page_buttons` 0x5ede20 at 0x30c,0x91, first and last buttons only
/// above 10 pages). The list area (`FN_DISPLAY_ADD_AREA` at 0x6e,0xaa..0x30c,0x217, column weights 2, 12, 10, 3, the
/// rows per page) and the rows from the page's first record (`manager_stats_row`) while there is room or the list
/// has fewer than 100 records. Then, with parameter 7 still set, the list scrolls to put the first human half a page
/// down (`exe::area_set_scroll_row`) and 7 is cleared; else it scrolls to parameter 5; the area goes to parameter 6.
/// Last, the back and forward buttons (`exe::gui_add_back_next_buttons`, disabled when the manager has nowhere to go).
///
/// Defect of the original: with exactly 100 records the screen has one page but shows only the rows that fit on it
/// (the one-page test is "at most 100", the row loop's "fewer than 100").
/// Original: manager_stats_screen_build (0x6ae2a0, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn manager_stats_screen_build() {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let list_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let page_buttons: unsafe extern "C" fn(i32, i32, usize, usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x5ede20usize);
    let message: unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
        core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    let sx16 = |v: usize| v as i16 as i32 as usize;
    // the four column weights and, in the original's next stack bytes, the last nation menu part / the first human
    let mut cols: [u8; 8] = [2, 0xc, 0xa, 3, 0xff, 0, 0, 0];
    let nation = get_screen_param(0) as i32;
    let mut list = get_screen_param(1);
    let mut count = get_screen_param(2) as i32;
    let mut page = get_screen_param(3);
    let mut scroll_to_human = get_screen_param(7);
    let stat = get_screen_param(8) as u8;
    let mut header = [0u8; 200];
    manager_stats_menu(nation, stat, &mut header);
    manager_stats_nation_menu(nation, &mut cols);
    if list == 0 {
        match manager_stats_collect(nation, stat, count) {
            None => return,
            Some((l, n)) => {
                list = l;
                count = n;
            }
        }
        set_screen_param(1, list);
        set_screen_param(2, count as usize);
        if list == 0 {
            covered!("tel26_5 manager stats empty");
            let rows = (0x15d / exe::font_get_height(3)) as u8;
            text(0xde1f64, 0xa14f5c);
            message(0x6e, 0xaa, 0x30c, 0x217, 2, 0, rows as usize, 3, colour_word(G_GUI_COLOUR_PALE_GREY), 0xde1f64, 0);
            manager_stats_back_next();
            return;
        }
    }
    let rpp = (0x15d / exe::font_get_height(3)) as u8;
    let rows = rpp as i32;
    let mut first = -1i32;
    let human_from = read_i32(G_STAFF_COUNT) - 0x10;
    let mut i = 0;
    while i < count && first == -1 {
        if read_i32(list + i as usize * MANAGER_STATS_RECORD_SIZE + manager_record::OWNER) >= human_from {
            first = i;
        }
        i += 1;
    }
    cols[4..8].copy_from_slice(&first.to_le_bytes());
    if count > 100 {
        covered!("tel26_5 manager stats pages");
        if scroll_to_human != 0 {
            page = (first.max(0) / rows + 1) as usize;
            set_screen_param(3, sx16(page));
            scroll_to_human = 0;
            set_screen_param(7, 0);
        }
        let pages = (count / rows + (count % rows != 0) as i32) as usize;
        set_screen_param(4, sx16(pages));
        if page as i16 > pages as i16 {
            set_screen_param(3, sx16(pages));
            page = pages;
        }
        if pages as i16 > 10 {
            page_buttons(0x30c, 0x91, page, pages, 3, 4, 5, 6);
        } else {
            page_buttons(0x30c, 0x91, page, pages, 0, 4, 5, 0);
        }
    } else {
        set_screen_param(4, 1);
        if page as i16 > 1 {
            set_screen_param(3, 1);
            page = 1;
        }
    }
    let area = list_area(current_human_display(), 0x6e, 0xaa, 0x30c, 0x217, 4, cols.as_ptr() as usize, rpp as usize, 0, 2, 0, usize::MAX);
    let mut row = 0i32;
    let mut idx = (page as i16 as i32 - 1) * rows;
    while (row < rows || count < 100) && idx < count {
        manager_stats_row(stat, nation, area, row, idx, list + idx as usize * MANAGER_STATS_RECORD_SIZE);
        idx += 1;
        row += 1;
    }
    let area_obj = || current_human_display().wrapping_add((area as i16 as isize * 0xbf1) as usize) + 4;
    if scroll_to_human != 0 {
        covered!("tel26_5 manager stats scroll to the human");
        let scroll = (first.max(0) - (rows >> 1)).max(0);
        if area as i16 != -1 {
            exe::area_set_scroll_row(area_obj(), scroll);
        }
        set_screen_param(6, sx16(area));
        set_screen_param(7, 0);
    } else {
        let scroll = get_screen_param(5);
        if area as i16 != -1 {
            exe::area_set_scroll_row(area_obj(), scroll);
        }
        set_screen_param(6, sx16(area));
    }
    manager_stats_back_next();
}

/// The back and forward buttons of the Manager Stats screen: forward asked first, then back, each enabled when the
/// screen manager says it is available.
unsafe fn manager_stats_back_next() {
    let forward = (exe::screenmanager_is_forward_unavailable(SCREEN_MANAGER) == 0) as u32;
    let back = (exe::screenmanager_is_back_unavailable(SCREEN_MANAGER, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(back, forward);
}

/// Comparer for `manager_stats_screen_build`.
///
/// Side by side over the display list (`compare_by_display_list_with`: the screen manager, the current `Display`,
/// the screen parameters, the work text) plus the list of parameter 1 (count x 210 bytes). Without a list the routine
/// builds it (allocation, the sort statics, parameters 1 and 2): only the original runs.
pub(crate) unsafe extern "C" fn compare_manager_stats_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    let list = get_screen_param(1);
    if list == 0 {
        // simplification: building the list allocates it (crt_realloc) and writes parameters 1 and 2: does not run
        // twice; the build is proven in Rust mode
        return o();
    }
    let n = get_screen_param(2) as i32;
    let mut extra = Vec::new();
    if n > 0 {
        extra.push((list, n as usize * MANAGER_STATS_RECORD_SIZE));
    }
    compare_by_display_list_with("Manager Stats screen (manager_stats_screen_build)", &extra, || o(), || manager_stats_screen_build());
}
