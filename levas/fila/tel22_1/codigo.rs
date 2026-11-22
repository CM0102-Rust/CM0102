// ---------- batch %%LEVA%%: the competition screen (comp_screens.cpp, part 1): opening it on the fixtures, building it (the tab bar, the Results and Qualifiers tabs) and closing it ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\comp_screens.cpp", the path the comp_screens.cpp error boxes print.
pub(crate) const COMP_SCREENS_CPP: usize = 0x99c15c;

/// A variadic format routine of the exe (`langlib_sprintf` 0x66f4e0 translates the format first; `crt_sprintf`
/// 0x944c9f does not): destination, format, the arguments.
pub(crate) type CompFormatFn = unsafe extern "C" fn(usize, usize, ...) -> i32;

/// `langlib_sprintf` (0x66f4e0, cdecl): the format translated to the game's language, then printed.
pub(crate) unsafe fn comp_lang_format() -> CompFormatFn {
    core::mem::transmute(FN_LANGLIB_SPRINTF)
}

/// `crt_sprintf` (0x944c9f, cdecl).
pub(crate) unsafe fn comp_crt_format() -> CompFormatFn {
    core::mem::transmute(FN_CRT_SPRINTF)
}

/// The comp_screens.cpp "Error" box at a line ("v<version> comp_screens.cpp <line>"), the left button let go.
pub(crate) unsafe fn comp_screens_error_box(line: u32) {
    tel26_error_box(COMP_SCREENS_CPP, line);
}

/// `ScreenManager::add_text_item` (0x803fd0, thiscall, ret 0x2c): x1, y1, x2, y2, the font, the flags, the style,
/// the colour, the text, the id and the value (an item of kind 1 in the current slot's Display); returns eax.
pub(crate) unsafe fn comp_text_item(f: [u32; 11]) -> u32 {
    type TextItem = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: TextItem = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    g(G_SCREEN_MANAGER, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10])
}

/// `ScreenManager::add_area` (0x4567f0, thiscall, ret 0x2c) through the exe address (the hook routes it): x1, y1,
/// x2, y2, the columns, the column weights, the rows and four more fields; returns the area index (ax).
pub(crate) unsafe fn comp_add_area(f: [u32; 11]) -> u32 {
    type AddArea = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: AddArea = core::mem::transmute(0x4567f0usize);
    g(G_SCREEN_MANAGER, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10])
}

/// `ScreenManager::set_area_scroll_row` (0x4567a0, thiscall, ret 8) through the exe address.
pub(crate) unsafe fn comp_set_area_scroll_row(area: u32, row: u32) {
    let g: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x4567a0usize);
    g(G_SCREEN_MANAGER, area, row)
}

/// `ScreenManager::add_item` (0x45cb60, thiscall, ret 0x48) through the exe address: the 18 fields of an item.
pub(crate) unsafe fn comp_add_item(f: [u32; 18]) -> u32 {
    type AddItem = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: AddItem = core::mem::transmute(0x45cb60usize);
    g(G_SCREEN_MANAGER, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10], f[11], f[12], f[13], f[14], f[15], f[16], f[17])
}

/// Frees the LLIST in screen parameter `k` (its destructor and operator delete) when there is one, and clears the
/// parameter (as `comp_screens_close_comp_screen` does for 5, 0xe and 0x17).
pub(crate) unsafe fn comp_free_list_param(k: u32) {
    if get_screen_param(k) == 0 {
        return;
    }
    let list = get_screen_param(k);
    if list != 0 {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
    }
    set_screen_param(k, 0);
}

/// Frees the heap block in screen parameter `k` (crt free) when there is one and clears it and the parameter after it
/// (its count), as `comp_screens_close_comp_screen` does for 0x1a/0x1b, 0x24/0x25 and 0x1f/0x20.
pub(crate) unsafe fn comp_free_block_param(k: u32) {
    if get_screen_param(k) == 0 {
        return;
    }
    exe::crt_free(get_screen_param(k));
    set_screen_param(k, 0);
    set_screen_param(k + 1, 0);
}

/// The flag the competition screens raise on every build [0x99c00c] and the Results tab's date flag [0xadae0c] (0 at
/// the start of a build, 1 once the Results tab has its date heading) [inference].
const COMP_SCREEN_BUILT_FLAG: usize = 0x99c00c;
const COMP_SCREEN_DATE_BUTTONS_FLAG: usize = 0xadae0c;
/// The competition id the History tab is not offered for [0x9cf6f8] [inference].
const COMP_SCREEN_NO_HISTORY_ID: usize = 0x9cf6f8;
/// A stage byte: how many rounds a cup stage has (+0xab; a league stage counts as one table) [inference].
const COMP_SCREEN_STAGE_ROUNDS: usize = 0xab;
/// The competition's average points table (+0x18; the Average Points tab is offered when it is set) [inference].
const COMP_SCREEN_AVERAGE_POINTS: usize = 0x18;
/// A fixture: its round (+0x30), stage code (+0x32), stage index (+0x34) and stage byte (+0x42), as the Results tab
/// groups and links them [inference].
const COMP_SCREEN_FIXTURE_ROUND: usize = 0x30;
const COMP_SCREEN_FIXTURE_STAGE_CODE: usize = 0x32;
const COMP_SCREEN_FIXTURE_STAGE_INDEX: usize = 0x34;
const COMP_SCREEN_FIXTURE_STAGE_BYTE: usize = 0x42;
/// The column weights of the Results tab's fixture list (9 columns).
const COMP_SCREEN_RESULTS_WEIGHTS: [u8; 9] = [3, 2, 0x12, 1, 4, 1, 0x12, 2, 3];
/// A MenuItem of the tab bar (0xd4 bytes): word +0 = −1 and word +2 = 0 (MenuItem::MenuItem 0x4568f0), the text at
/// +4, the kind at +0xcc and the value at +0xd0.
const COMP_SCREEN_MENU_ITEM: usize = 0xd4;

/// Rule: opening the competition screen on its fixtures: the Results tab on a given date (`mode` ≠ 0) or the Fixtures
/// tab (`mode` 0) of the competition.
///
/// `comp_screens_open_comp_fixtures(comp, day, year, mode)`: the competition object is the entry of the competition
/// table [`G_COMP_OBJECT_TABLE`] at the record's id (`comp` +0). The screen opens with `open_screen` (builder
/// `comp_screens_draw_comp_screen`, input `comp_screens_handle_comp_screen_click`, flag 0, leave
/// `comp_screens_close_comp_screen`, no exit); if it does not open nothing else happens. Then the 54 parameters: 0 the
/// object; 1 the tab (2 Results with `mode`, else 3 Fixtures); 2 and 3 the stage −2 (current); 8 and 9 the Results
/// date (the day's and the year's low words with `mode`, else 0); 0x11 and 0x12 the Fixtures date (the same words
/// without `mode`, else 0); 0x1e 0x11; 0x23 3; 0x2a and 0x32 −1; 0x2d 1; every other one 0 (4..7, 0xa..0x10,
/// 0x13..0x1d, 0x1f..0x22, 0x24..0x29, 0x2b, 0x2c, 0x2e..0x31, 0x33..0x35), written in the original's order.
/// Original: `comp_screens_open_comp_fixtures` (0x48f8e0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_open_comp_fixtures(comp: usize, day: u32, year: u32, mode: u32) {
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(comp) as usize * 4);
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x48fcd0usize, 0x4965d0usize, 0, 0x4985e0usize, 0) == 0 {
        return;
    }
    covered!("tel22_1 open comp fixtures");
    let plan = comp_screen_planned_params(object, day, year, mode);
    for k in COMP_SCREEN_OPEN_ORDER {
        set_screen_param(k, plan[k as usize]);
    }
}

/// The order `comp_screens_open_comp_fixtures` writes its 54 parameters in.
const COMP_SCREEN_OPEN_ORDER: [u32; 54] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x10, 0x11, 0x12,
    0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x23, 0x24, 0x25, 0x26, 0x27, 0x1e, 0x1f, 0x20,
    0x21, 0x22, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35];

/// The 54 parameters of the competition screen as `comp_screens_open_comp_fixtures` leaves them.
fn comp_screen_planned_params(object: usize, day: u32, year: u32, mode: u32) -> [usize; 0x36] {
    let mut p = [0usize; 0x36];
    let (day, year) = ((day & 0xffff) as usize, (year & 0xffff) as usize);
    p[0] = object;
    p[1] = if mode != 0 { 2 } else { 3 };
    p[2] = -2isize as usize;
    p[3] = -2isize as usize;
    if mode != 0 {
        p[8] = day;
        p[9] = year;
    } else {
        p[0x11] = day;
        p[0x12] = year;
    }
    p[0x23] = 3;
    p[0x1e] = 0x11;
    p[0x2a] = usize::MAX;
    p[0x2d] = 1;
    p[0x32] = usize::MAX;
    p
}

/// Comparer for `comp_screens_open_comp_fixtures`: only the original runs (it opens a screen), and the parameters are
/// checked against the plan afterwards.
pub(crate) unsafe extern "C" fn compare_comp_screens_open_comp_fixtures(comp: usize, day: u32, year: u32, mode: u32) {
    let object = if comp != 0 { read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(comp) as usize * 4) } else { 0 };
    let plan = comp_screen_planned_params(object, day, year, mode);
    original_fn::<unsafe extern "C" fn(usize, u32, u32, u32)>(%%I0%%)(comp, day, year, mode);
    if object != 0 && get_screen_param(0) == object {
        let mut got = [0usize; 0x36];
        for (k, v) in got.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
        SCOREBOARD.record(got == plan, || format!("competition screen (open fixtures {:#x}): params {:x?} x plan {:x?}", comp, got, plan));
    }
}

/// `FIXTURE_MANAGER::scan_fixtures` (0x5acee0, thiscall, ret 0x48) as the Results tab asks it: from the date (`year`,
/// `day`, both moved to the match day found) and the mode byte, forward or not, the competition's matches (every club,
/// round and stage: −1), all of that day (−2), into `list`; returns how many (ax).
unsafe fn comp_screen_scan(year: &mut u32, day: &mut u32, mode: &mut u8, forward: u32, comp_id: u32, list: usize) -> u16 {
    type Scan = unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, u32, u32, i32, i32, i32, i32, i32, i32, i32,
                                            i32, usize, u32, i32, i32, u32) -> u32;
    let f: Scan = core::mem::transmute(FN_FIXTURE_MANAGER_SCAN_FIXTURES);
    f(read_usize(G_FIXTURE_MANAGER), year, day, mode, forward, comp_id, -1, -1, -1, -1, -1, -1, -1, -2, list, 1, -2, -1, 1) as u16
}

/// The match day before (`forward` 0, mode 2) or after (`forward` 1, mode 0) the date (`day`, `year`) for the Results
/// tab's arrows: `Date::Date`, one day back or ahead (`Date::sub_days`/`add_days` into `out`) and `Date::operator=`
/// (into `other`), then the first match day from there (`comp_screen_scan` into the local `list`). Returns the found
/// day and year dwords (the date's +0 and +2 dwords as the scan left them); none found leaves the year 1.
unsafe fn comp_screen_neighbour_day(day: u32, year: u32, forward: u32, comp_id: u32, list: usize, date: &mut [u8; 8], out: &mut [u8; 8], other: &mut [u8; 8]) -> (u32, u32) {
    exe::new_date(date.as_mut_ptr() as usize, day, year);
    let moved = if forward != 0 {
        exe::date_add_days(date.as_mut_ptr() as usize, out.as_mut_ptr() as usize, 1)
    } else {
        exe::date_sub_days(date.as_mut_ptr() as usize, out.as_mut_ptr() as usize, 1)
    };
    exe::date_operator_3d(date.as_mut_ptr() as usize, other.as_mut_ptr() as usize, moved);
    let mut found_day = u32::from_le_bytes([date[0], date[1], date[2], date[3]]);
    let mut found_year = u32::from_le_bytes([date[2], date[3], date[4], date[5]]);
    let mut mode: u8 = if forward != 0 { 0 } else { 2 };
    if comp_screen_scan(&mut found_year, &mut found_day, &mut mode, forward, comp_id, list) == 0 {
        found_year = 1;
    }
    (found_day, found_year)
}

/// The Results tab of the competition screen (tab 2): one match day of the competition's fixtures.
///
/// The day comes from parameters 8 and 9 (day, year) with the day's match list in parameter 0xe. Without the list,
/// unless the year is 1 (no matches): a year 0 takes today's date ([`G_CURRENT_DATE`]); a new LLIST (operator new 0xe)
/// goes to parameter 0xe and `scan_fixtures` (backward, mode 2) fills it with the match day found from the date (none:
/// year 1; else `comp_util_sort_fixture_list`), and the day and year go back to parameters 8 and 9. Year 1: "No matches
/// have been played." in a text area (pale grey, font 3). Otherwise the previous and next match days (parameters
/// 0xa/0xb and 0xc/0xd, looked up when their year is 0, `comp_screen_neighbour_day`, into a local list) give the
/// "<< Date" and "Date >>" cells (buttons 0xc and 0xd when the year is above 1, else grey text) in a small area; the
/// date (`Date::get_long_date_string`, without the year in the current year) in yellow font 3 under it ([0xadae0c] =
/// 1); then a 9-column area with the fixtures: each new round (stage code +0x32 / round +0x30, not both 0) gets an empty
/// row (but the first), its name (`comp_util_format_stage_round_name`) as a yellow item (a link, id 0xe and value
/// (stage byte + 1) × 1000 + stage index, when `Comp::are_round_fixtures_known`; up to x 0x2f1 by its width) and a
/// yellow underline (flags 0xc400); then each fixture's row (`comp_screens_add_fixture_row`, id 0xf). The scroll row
/// comes back from parameter 0xf and the area goes to parameter 0x10.
/// Original: part of `comp_screens_draw_comp_screen` (0x48fcd0, comp_screens.cpp).
unsafe fn comp_screen_results_tab(comp: usize, top: i32, bottom: i32) {
    let mut local_list = [0u8; 0xe];
    exe::new_llist(local_list.as_mut_ptr() as usize);
    let mut day = get_screen_param(8) as u32;
    let mut year = get_screen_param(9) as u32;
    let mut prev_year = get_screen_param(0xb) as u32;
    let mut next_year = get_screen_param(0xd) as u32;
    let mut list = get_screen_param(0xe);
    let comp_id = read_u32(read_usize(comp + league::COMPETITION));
    if list == 0 && year as u16 != 1 {
        if year as u16 == 0 {
            day = read_u32(G_CURRENT_DATE);
            year = (year & 0xffff_0000) | read_u16(G_CURRENT_DATE_YEAR) as u32;
        }
        let p = exe::crt_operator_new(0xe);
        list = if p != 0 { exe::new_llist(p) } else { 0 };
        set_screen_param(0xe, list);
        let mut mode: u8 = 2;
        if comp_screen_scan(&mut year, &mut day, &mut mode, 0, comp_id, list) == 0 {
            year = 1;
        } else {
            let sort: unsafe extern "C" fn(usize) = core::mem::transmute(0x4c0520usize);
            sort(list);
        }
        set_screen_param(8, (day & 0xffff) as usize);
        set_screen_param(9, (year & 0xffff) as usize);
    }
    if year as u16 == 1 {
        covered!("tel22_1 results no matches");
        let rows = ((bottom - top - 0x10) / exe::font_get_height(3)) as u8 as u32;
        comp_lang_format()(JOB_TEXT, 0x99c02c); // "No matches have been played."
        tel26_text_area([0x6e, top as u32, 0x30c, bottom as u32, 2, 0, rows, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0]);
        exe::drop_llist(local_list.as_mut_ptr() as usize);
        return;
    }
    let (mut date, mut out, mut other) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    if prev_year as u16 == 0 {
        let (d, y) = comp_screen_neighbour_day(day, year, 0, comp_id, local_list.as_mut_ptr() as usize, &mut date, &mut out, &mut other);
        prev_year = y;
        set_screen_param(0xa, (d & 0xffff) as usize);
        set_screen_param(0xb, (prev_year & 0xffff) as usize);
    }
    if next_year as u16 == 0 {
        // the next day's temporaries swap: the moved date in the third buffer, the assigned one in the second
        let (d, y) = comp_screen_neighbour_day(day, year, 1, comp_id, local_list.as_mut_ptr() as usize, &mut date, &mut other, &mut out);
        next_year = y;
        set_screen_param(0xc, (d & 0xffff) as usize);
        set_screen_param(0xd, (next_year & 0xffff) as usize);
    }
    let arrows = comp_add_area([0x6e, top as u32, 0x168, (top + 0x14) as u32, 2, 0, 1, 0, 1, 0, u32::MAX]);
    let mut word = [0u8; 0x200];
    let (grey, pale) = (job_colour(JOB_COLOUR_317E), job_colour(JOB_COLOUR_3184));
    for (column, fmt, other_year, id) in [(0u32, 0x988090usize, prev_year, 0xcu32), (1, 0x988088, next_year, 0xd)] {
        comp_lang_format()(word.as_mut_ptr() as usize, 0x988078); // "Date"
        comp_crt_format()(JOB_TEXT, fmt, word.as_ptr()); // "<< %s" / "%s >>"
        if other_year as u16 > 1 {
            tel26_area_button([arrows, column, 0, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, id, 0]);
        } else {
            tel26_area_text([arrows, column, 0, 0x30, grey, 0x2c, 1, pale, JOB_TEXT as u32]);
        }
    }
    let top = top + 0x19;
    write_u32(COMP_SCREEN_DATE_BUTTONS_FLAG, 1);
    if year as u16 == read_u16(G_CURRENT_DATE_YEAR) {
        exe::new_date(date.as_mut_ptr() as usize, day, year);
        exe::date_get_long_date_string_no_year(date.as_mut_ptr() as usize, JOB_TEXT, 0x7d0, 0xff);
    } else {
        exe::new_date(out.as_mut_ptr() as usize, day, year);
        exe::date_get_long_date_string(out.as_mut_ptr() as usize, JOB_TEXT, 0x7d0, 0xff);
    }
    comp_text_item([0x6e, top as u32, 0x30c, (top + 0x23) as u32, 2, 0, 0xc, 3, job_colour(G_GUI_COLOUR_YELLOW), JOB_TEXT as u32, 1]);
    let top = top + 0x28;
    // simplification: the row count goes as its byte; the original's dword carries stack garbage above it
    let rows = ((bottom - top - 0x10) / exe::font_get_height(3)) as u8 as u32;
    let area = comp_add_area([0x6e, top as u32, 0x30c, bottom as u32, 9, COMP_SCREEN_RESULTS_WEIGHTS.as_ptr() as u32, rows, 0, 2, 0, u32::MAX]);
    let known: unsafe extern "thiscall" fn(usize, i32, i32) -> u32 = core::mem::transmute(0x48d410usize);
    let mut row: i32 = 0;
    write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
    let mut prev = 0usize;
    let mut fx = exe::llist_get_next(list);
    while fx != 0 {
        let code = read_i16(fx + COMP_SCREEN_FIXTURE_STAGE_CODE);
        let round = read_i16(fx + COMP_SCREEN_FIXTURE_ROUND);
        let same = prev != 0 && code == read_i16(prev + COMP_SCREEN_FIXTURE_STAGE_CODE) && round == read_i16(prev + COMP_SCREEN_FIXTURE_ROUND);
        if !same && (code != 0 || round != 0) {
            covered!("tel22_1 results round heading");
            if prev != 0 {
                tel26_area_text([area, 0, row as u32, 1, 0, 1, 1, job_colour(JOB_COLOUR_3184), 0]);
                row += 1;
            }
            exe::comp_util_format_stage_round_name(code as i32, round as i32, JOB_TEXT, 0x7d0, 0);
            let x2 = (exe::font_text_width(3usize, JOB_TEXT as *const u8) + 0x73).min(0x2f1); // tel22_1 the round heading's width
            let stage_byte = read_u8(fx + COMP_SCREEN_FIXTURE_STAGE_BYTE) as i8 as i32;
            let stage_index = read_i16(fx + COMP_SCREEN_FIXTURE_STAGE_INDEX) as i32;
            let yellow = job_colour(G_GUI_COLOUR_YELLOW);
            if known(comp, stage_byte, stage_index) != 0 {
                let value = (stage_byte + 1) * 1000 + stage_index;
                comp_add_item([2, 0x73, 0, x2 as u32, 0, 0, row as u32, 1, 0, 0, 1, 3, yellow, JOB_TEXT as u32, 0, 0xe, value as u32, area]);
            } else {
                comp_add_item([1, 0x73, 0, x2 as u32, 0, 0, row as u32, 1, 0, 0, 1, 3, yellow, JOB_TEXT as u32, 0, 0, 0, area]);
            }
            row += 1;
            comp_add_item([1, 0x73, 0, x2 as u32, 0, 0, row as u32, 0xc400, yellow, 0, 0, 0, 0, 0, 0, 0, 0, area]);
            row += 1;
        }
        exe::comp_screens_add_fixture_row(fx, area as usize, &mut row, 0xf);
        prev = fx;
        fx = exe::llist_get_next(list);
    }
    comp_set_area_scroll_row(area, get_screen_param(0xf) as u32);
    set_screen_param(0x10, area as u16 as i16 as isize as usize);
    exe::drop_llist(local_list.as_mut_ptr() as usize);
}

/// The Qualifiers tab of the competition screen (tab 5): the clubs of the entrant book with a class (+4 ≠ 0), in two
/// halves side by side (columns 0-1 and 2-3) of as many rows as fit in font 3 (more: half of them, rounded up), each
/// "  short name" as a button (id 0x7d5, the club; yellow for a human's club, else pale grey) and its class ("Host
/// nation" for 1, "Previous winner" for 2, else empty) in yellow font 0xc; the cells past the last entrant stay empty
/// (pale grey).
/// Original: part of `comp_screens_draw_comp_screen` (0x48fcd0, comp_screens.cpp).
unsafe fn comp_screen_qualifiers_tab(comp: usize, top: i32, bottom: i32) {
    let entry_at = |k: u32| read_usize(comp + league::ENTRANT_BOOK) + (k as i16 as isize * 6) as usize;
    let count = read_i16(comp + league::ENTRANT_COUNT);
    let mut classed: i32 = 0;
    for k in 0..count.max(0) as u32 {
        if read_u8(entry_at(k) + 4) != 0 {
            classed += 1;
        }
    }
    let rows = ((bottom - top) / exe::font_get_height(3)) as i8;
    let half: i32 = if classed as i16 as i32 > rows as i32 * 2 { (classed + 1) / 2 } else { rows as i32 };
    let total = (half as u32).wrapping_shl(1);
    // simplification: the row count goes as its byte; the original's dword carries stack garbage above it
    let area = comp_add_area([0x6e, top as u32, 0x30c, bottom as u32, 4, 0, rows as u8 as u32, 0, 0, 0, u32::MAX]);
    if total as i16 <= 0 {
        return;
    }
    covered!("tel22_1 qualifiers rows");
    let (mut r, mut k, mut i): (u32, u32, i32) = (0, 0, 0);
    loop {
        let count = read_i16(comp + league::ENTRANT_COUNT);
        while (k as i16) < count && read_u8(entry_at(k) + 4) == 0 {
            k = k.wrapping_add(1);
        }
        let halfway = total as i16 as i32 / 2;
        let (column, row) = if i < halfway { (0i32, r as u8 as i8 as i32) } else { (2, (r as u8).wrapping_sub(halfway as u8) as i8 as i32) };
        if (k as i16) < count {
            let club = read_usize(entry_at(k));
            comp_crt_format()(JOB_TEXT, 0x987ff0, club + club::SHORT_NAME); // "  %s"
            let colour = if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 { job_colour(G_GUI_COLOUR_YELLOW) } else { job_colour(JOB_COLOUR_3184) };
            tel26_area_button([area, column as u32, row as u32, 2, 0, 1, 3, colour, JOB_TEXT as u32, 0x7d5, club as u32]);
            match read_u8(entry_at(k) + 4) as i8 {
                1 => {
                    comp_lang_format()(JOB_TEXT, 0x99c010); // "Host nation"
                }
                2 => {
                    comp_lang_format()(JOB_TEXT, 0x99c01c); // "Previous winner"
                }
                _ => write_u8(JOB_TEXT, 0),
            }
            tel26_area_text([area, (column + 1) as u32, row as u32, 2, 0, 0xc, 2, job_colour(G_GUI_COLOUR_YELLOW), JOB_TEXT as u32]);
        } else {
            tel26_area_text([area, column as u32, row as u32, 2, 0, 1, 2, job_colour(JOB_COLOUR_3184), 0]);
            tel26_area_text([area, (column + 1) as u32, row as u32, 2, 0, 1, 2, job_colour(JOB_COLOUR_3184), 0]);
        }
        r = r.wrapping_add(1);
        i += 1;
        k = k.wrapping_add(1);
        if (r as i16) >= (total as i16) {
            break;
        }
    }
}

/// Rule: building the competition screen: the title, the tab bar with the tabs the competition has, a Print button
/// with its popup, and the open tab.
///
/// [0x99c00c] = 1, [0xadae0c] = 0, parameter 0x34 = 0. The object is parameter 0; without it the comp_screens.cpp box
/// of line 0x1c6 and nothing else. The tab is parameter 1's low byte. The competition's picture
/// (`background_set_comp_picture`), its colours (`database_get_comp_colours`), the title (the competition's name) and
/// the menu bar (4). The stages from −1 to the current one (`league::CURRENT_STAGE`) are counted: tables (type 2) and
/// rounds (each cup stage's round byte +0xab, at least 1). The tabs, in order: the stages tab ("Stages" with tables
/// and rounds, "Tables"/"Table" with tables only, "Rounds"/"Draw" with rounds only) = 1, "Results" = 2, "Fixtures" =
/// 3, "Schedule" = 4, "Qualifiers" = 5 (when the entrant book holds the qualifiers), "Seedings" = 6 (when it does not
/// and there is a book), "Average Points" = 9 (+0x18 set), "Team Stats" = 7 and "Player Stats" = 8 (with
/// `league::COMP_STATS`), "Referee Stats" = 10 (`comp_util_is_league_division`), "Awards" (kind 3, the award of
/// `AWARD_MANAGER::find_award_by_club_comp`, its id +5) and "History" (kind 4; not for friendlies nor the competition
/// [0x9cf6f8]); a tab not offered while it is the open one sends the screen back to tab 1 (parameter 1 = 1). With
/// tables or statistics: the Print button (0x2c6, 0xf, 0x311, 0x23; black text, pale grey on a black competition
/// colour) and its popup area (flags 0x130) with the items "Table(s)" (data 1, only with tables), "Goals" 2,
/// "Assists" 4, "Average Ratings" 8, "Man of Match" 0x10 and "All" 0x1f (only with statistics; disabled 0x21
/// otherwise), id 0x19. The tab bar (`gui_add_tab_bar`, from 0x50 to 0x221, the narrow style below 8 tabs); then the
/// tab: 1 the current stage (parameter 2; −2 = `Comp::get_current_stage`'s index, written back) as a league table
/// (`comp_screens_draw_league_table_tab`) or a cup draw (`comp_screens_draw_cup_rounds_tab`); 2
/// `comp_screen_results_tab`; 3 Fixtures, 4 Schedule, 6 Seedings, 7 Team Stats, 8 Player Stats and 9 Average Points
/// (their tab routines, with the object); 5 `comp_screen_qualifiers_tab`; 10 the referee stats (0x79a530, with the
/// record). Then the related competitions menu (0x49d070, the colours, 1, 0x18) and the Back/Next buttons.
/// Original: `comp_screens_draw_comp_screen` (0x48fcd0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_comp_screen() {
    let mut items = [[0u8; COMP_SCREEN_MENU_ITEM]; 0x1a];
    for it in items.iter_mut() {
        it[0] = 0xff;
        it[1] = 0xff;
    }
    write_u32(COMP_SCREEN_BUILT_FLAG, 1);
    write_u32(COMP_SCREEN_DATE_BUTTONS_FLAG, 0);
    set_screen_param(0x34, 0);
    let comp = get_screen_param(0);
    if comp == 0 {
        comp_screens_error_box(0x1c6);
        return;
    }
    covered!("tel22_1 build comp screen");
    let mut tab = get_screen_param(1) as u8 as i8;
    let record = read_usize(comp + league::COMPETITION);
    exe::background_set_comp_picture(record, 0, 0);
    let (mut colour_44, mut colour_40) = (0u32, 0u32);
    exe::database_get_comp_colours(record, &mut colour_44, &mut colour_40);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, colour_40, colour_44, read_usize(comp + league::COMPETITION) + 4);
    exe::menubar_build(4, 0);
    let (mut tables, mut rounds) = (0i32, 0i32);
    let mut s: i32 = -1;
    while s <= read_i32(comp + league::CURRENT_STAGE) {
        let stage = exe::comp_get_stage(comp, s);
        if read_u8(stage + league::COMP_TYPE) == 2 {
            tables += 1;
        } else {
            let n = read_u8(stage + COMP_SCREEN_STAGE_ROUNDS) as i8;
            rounds += if n > 1 { n as i32 } else { 1 };
        }
        s += 1;
    }
    let heading = if tables != 0 {
        if rounds != 0 { 0x99c154 } else if tables > 1 { 0x99c14c } else { 0x99c144 } // "Stages" / "Tables" / "Table"
    } else if rounds > 1 {
        0x99c13c // "Rounds"
    } else {
        0x99c134 // "Draw"
    };
    comp_lang_format()(JOB_TEXT, heading);
    fn set_kind(items: &mut [[u8; COMP_SCREEN_MENU_ITEM]; 0x1a], k: usize, kind: u32, value: u32) {
        items[k][0xcc..0xd0].copy_from_slice(&kind.to_le_bytes());
        items[k][0xd0..0xd4].copy_from_slice(&value.to_le_bytes());
    }
    fn text(items: &mut [[u8; COMP_SCREEN_MENU_ITEM]; 0x1a], k: usize) -> usize {
        items[k][4..].as_mut_ptr() as usize
    }
    tel26_copy_text(text(&mut items, 0) as *mut u8, JOB_TEXT as *const u8);
    set_kind(&mut items, 0, 2, 1);
    comp_lang_format()(text(&mut items, 1), 0x990a94); // "Results"
    set_kind(&mut items, 1, 2, 2);
    comp_lang_format()(text(&mut items, 2), 0x98e868); // "Fixtures"
    set_kind(&mut items, 2, 2, 3);
    comp_lang_format()(text(&mut items, 3), 0x99c104, 0xad9c64usize); // "Schedule"
    set_kind(&mut items, 3, 2, 4);
    let mut n = 4usize;
    let back_to_tab_1 = |tab: &mut i8| {
        *tab = 1;
        set_screen_param(1, 1);
    };
    if read_u8(comp + league::ENTRANTS_ARE_QUALIFIERS) == 1 && read_usize(comp + league::ENTRANT_BOOK) != 0 {
        comp_lang_format()(text(&mut items, 4), 0x99c0f8); // "Qualifiers"
        set_kind(&mut items, 4, 2, 5);
        n = 5;
    } else if tab == 5 {
        back_to_tab_1(&mut tab);
    }
    if read_u8(comp + league::ENTRANTS_ARE_QUALIFIERS) == 0 && read_usize(comp + league::ENTRANT_BOOK) != 0 {
        comp_lang_format()(text(&mut items, n), 0x99c0ec); // "Seedings"
        set_kind(&mut items, n, 2, 6);
        n += 1;
    } else if tab == 6 {
        back_to_tab_1(&mut tab);
    }
    if read_usize(comp + COMP_SCREEN_AVERAGE_POINTS) != 0 {
        comp_lang_format()(text(&mut items, n), 0x99c0dc); // "Average Points"
        set_kind(&mut items, n, 2, 9);
        n += 1;
    } else if tab == 9 {
        back_to_tab_1(&mut tab);
    }
    if read_usize(comp + league::COMP_STATS) != 0 {
        comp_lang_format()(text(&mut items, n), 0x99c0d0); // "Team Stats"
        set_kind(&mut items, n, 2, 7);
        n += 1;
        comp_lang_format()(text(&mut items, n), 0x99c0c0); // "Player Stats"
        set_kind(&mut items, n, 2, 8);
        n += 1;
    } else if tab == 7 || tab == 8 {
        back_to_tab_1(&mut tab);
    }
    if exe::comp_util_is_league_division(read_i32(read_usize(comp + league::COMPETITION))) != 0 {
        comp_lang_format()(text(&mut items, n), 0x99c0b0); // "Referee Stats"
        set_kind(&mut items, n, 2, 10);
        n += 1;
    }
    let find_award: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x414ac0usize);
    let award = find_award(read_usize(G_AWARD_MANAGER), read_usize(comp + league::COMPETITION));
    if award != 0 {
        comp_lang_format()(text(&mut items, n), 0x99c0a8); // "Awards"
        set_kind(&mut items, n, 3, read_u32(award + 5));
        n += 1;
    }
    if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), read_usize(comp + league::COMPETITION)) == 0
        && read_u32(read_usize(comp + league::COMPETITION)) != read_u32(COMP_SCREEN_NO_HISTORY_ID)
    {
        comp_lang_format()(text(&mut items, n), 0x99c074, 0xad9c64usize); // "History"
        set_kind(&mut items, n, 4, 0);
        n += 1;
    }
    if tables != 0 || read_usize(comp + league::COMP_STATS) != 0 {
        covered!("tel22_1 print popup");
        comp_lang_format()(JOB_TEXT, 0x98e5f4); // "Print"
        let text_colour = if colour_40 as u16 == read_u16(G_GUI_COLOUR_BLACK) { job_colour(JOB_COLOUR_3184) } else { job_colour(G_GUI_COLOUR_BLACK) };
        let button = tel26_add_button([0x2c6, 0xf, 0x311, 0x23, 0x30, colour_40, 0xc, 1, text_colour, JOB_TEXT as u32, 0, 0]);
        let popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), button]);
        comp_lang_format()(JOB_TEXT, if tables > 1 { 0x99c14c } else { 0x99c144 }); // "Tables" / "Table"
        let popup_item = |enabled: bool, data: u32| {
            let (style, id, value) = if enabled { (1, 0x19, data) } else { (0x21, 0, 0) }; // tel22_1 a Print popup item
            tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, job_colour(JOB_COLOUR_31AA), job_colour(G_GUI_COLOUR_YELLOW), style, 1,
                job_colour(G_GUI_COLOUR_BLACK), JOB_TEXT as u32, 0, id, value, popup]);
        };
        popup_item(tables != 0, 1);
        for (fmt, data) in [(0x98e9ccusize, 2u32), (0x99c06c, 4), (0x99c05c, 8), (0x99c04c, 0x10)] {
            comp_lang_format()(JOB_TEXT, fmt); // "Goals", "Assists", "Average Ratings", "Man of Match"
            popup_item(read_usize(comp + league::COMP_STATS) != 0, data);
        }
        comp_lang_format()(JOB_TEXT, 0x98e5c8, 0xad9c64usize); // "All"
        popup_item(read_usize(comp + league::COMP_STATS) != 0, 0x1f);
    }
    let (mut top, mut bottom) = (0x50i32, 0x221i32);
    exe::gui_add_tab_bar(items.as_mut_ptr() as usize, n as u32, tab as i32, &mut top, &mut bottom, (n < 8) as u32);
    let tab_routine = |address: usize, object: usize| {
        let f: unsafe extern "C" fn(usize, i32, i32) = core::mem::transmute(address);
        f(object, top, bottom);
    };
    match tab {
        1 => {
            let mut s = get_screen_param(2) as i32;
            if s == -2 {
                let current: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x48cec0usize);
                s = read_u8(current(comp) + league::STAGE_INDEX) as i8 as i32;
                set_screen_param(2, s as isize as usize);
            }
            let stage = exe::comp_get_stage(comp, s);
            if read_u8(stage + league::COMP_TYPE) == 2 {
                tab_routine(0x491380, stage);
            } else {
                tab_routine(0x492980, stage);
            }
        }
        2 => comp_screen_results_tab(comp, top, bottom),
        3 => tab_routine(0x493ee0, comp),
        4 => tab_routine(0x4947a0, comp),
        5 => comp_screen_qualifiers_tab(comp, top, bottom),
        6 => tab_routine(0x494c60, comp),
        7 => tab_routine(0x4956e0, comp),
        8 => tab_routine(0x495bc0, comp),
        9 => tab_routine(0x495050, comp),
        10 => tab_routine(0x79a530, read_usize(comp + league::COMPETITION)),
        _ => {}
    }
    let related: unsafe extern "C" fn(usize, u32, u32, u32, u32) = core::mem::transmute(0x49d070usize);
    related(comp, colour_40, colour_44, 1, 0x18);
    tel26_back_next();
}

/// Comparer for `comp_screens_draw_comp_screen`: side by side over the display list (`compare_match_screen_build`)
/// plus the two flags, when the open tab keeps no list to build.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_comp_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    let comp = get_screen_param(0);
    // simplification: the tabs that build and keep a list in the params on their first build (1: a small league's
    // fixtures, param 5; 2: the day's matches, 0xe; 3: 0x17; 4: 0x1a; 7: 0x24; 8: 0x1f), the referee stats (10, not
    // in this file) and the error box (no object) run the original only; the tabs have their own comparers
    let ready = |tab: u8| match tab {
        1 => get_screen_param(5) != 0,
        2 => get_screen_param(0xe) != 0,
        3 => get_screen_param(0x17) != 0,
        4 => get_screen_param(0x1a) != 0,
        5 | 6 | 9 => true,
        7 => get_screen_param(0x24) != 0,
        8 => get_screen_param(0x1f) != 0,
        _ => false,
    };
    let tab = get_screen_param(1) as u8;
    if comp == 0 || !ready(tab) || ((5..=9).contains(&tab) && !ready(1)) {
        return o();
    }
    compare_match_screen_build("competition screen", &[(COMP_SCREEN_BUILT_FLAG, 4), (COMP_SCREEN_DATE_BUTTONS_FLAG, 4)], || o(),
        || comp_screens_draw_comp_screen());
}

/// Rule: leaving the competition screen frees what its tabs kept: the LLISTs of parameters 5, 0xe and 0x17 and the
/// blocks of parameters 0x1a, 0x24 and 0x1f (with their counts 0x1b, 0x25 and 0x20 cleared).
///
/// Each list: when the parameter is set, its destructor and operator delete, the parameter cleared; each block: when
/// set, crt free and the parameter and the next one cleared (`comp_free_list_param`, `comp_free_block_param`).
/// Original: `comp_screens_close_comp_screen` (0x4985e0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_close_comp_screen() {
    comp_free_list_param(5);
    comp_free_list_param(0xe);
    comp_free_list_param(0x17);
    comp_free_block_param(0x1a);
    comp_free_block_param(0x24);
    comp_free_block_param(0x1f);
}

/// The parameters `comp_screens_close_comp_screen` leaves: those it frees cleared (with the count after each block),
/// the others as they were.
unsafe fn comp_screen_close_plan() -> [usize; 0x36] {
    let mut p = [0usize; 0x36];
    for (k, v) in p.iter_mut().enumerate() {
        *v = get_screen_param(k as u32);
    }
    for k in [5usize, 0xe, 0x17] {
        p[k] = 0;
    }
    for k in [0x1ausize, 0x24, 0x1f] {
        if p[k] != 0 {
            p[k] = 0;
            p[k + 1] = 0;
        }
    }
    p
}

/// Comparer for `comp_screens_close_comp_screen`: only the original runs (it frees memory), the parameters checked
/// against the plan afterwards.
pub(crate) unsafe extern "C" fn compare_comp_screens_close_comp_screen() {
    let plan = comp_screen_close_plan();
    original_fn::<unsafe extern "C" fn()>(%%I2%%)();
    let mut got = [0usize; 0x36];
    for (k, v) in got.iter_mut().enumerate() {
        *v = get_screen_param(k as u32);
    }
    SCOREBOARD.record(got == plan, || format!("competition screen (close): params {:x?} x plan {:x?}", got, plan));
}
