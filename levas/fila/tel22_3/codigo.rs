// ---------- batch %%LEVA%%: the competition screen's tabs (comp_screens.cpp, part 3): Fixtures, Schedule, Seedings, Average Points, Team Stats and Player Stats ----------

/// The weights of the nine columns of the Fixtures tab's list (3, 2, 18, 1, 4, 1, 18, 2, 3).
const FIXTURES_TAB_WEIGHTS: [u8; 9] = [3, 2, 0x12, 1, 4, 1, 0x12, 2, 3];
/// Two colour words of the competition tabs with no name yet (the average points columns and the unknown rounds).
const COMP_TABS_COLOUR_3152: usize = 0xae3152;
const COMP_TABS_COLOUR_3154: usize = 0xae3154;
/// The ids of the European competitions the Seedings tab links (Intertoto for kind 4, European Cup for kind 5)
/// [inference: the labels next to them].
const SEEDINGS_TAB_COMP_KIND_4: usize = 0x9cf6f8;
const SEEDINGS_TAB_COMP_KIND_5: usize = 0x9cf6e8;
/// Size of a competition record (`competition::REPUTATION` + 2).
const SEEDINGS_TAB_COMPETITION_SIZE: usize = 0x6b;

/// Restores the scroll row of a tab's list area from screen parameter `scroll_param` (when the area exists) and
/// keeps the area index (sign-extended) in `area_param`, as every list tab of the competition screen ends.
pub(crate) unsafe fn comp_tabs_restore_scroll(area: u32, scroll_param: u32, area_param: u32) {
    let scroll = get_screen_param(scroll_param);
    let display = current_human_display();
    if area as i16 != -1 {
        exe::area_set_scroll_row(display.wrapping_add(4).wrapping_add((area as i16 as isize * 0xbf1) as usize), scroll);
    }
    set_screen_param(area_param, area as i16 as isize as usize);
}

/// `Comp::are_round_fixtures_known(stage, round)` (0x48d410, thiscall, ret 8) through the exe address.
unsafe fn fixtures_tab_round_known(comp: usize, stage: i32, round: i32) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, i32, i32) -> u32 = core::mem::transmute(0x48d410usize);
    f(comp, stage, round)
}

/// `FIXTURE_MANAGER::scan_fixtures` (0x5acee0, thiscall, ret 0x48) as the Fixtures tab asks it: from the date
/// (`year`, `day`, both moved by the scan) and the slot, forward or not, the competition's matches (every club, round
/// and stage: −1), at most `count` (−2 all, −1 one), into `list`; returns how many were found (ax).
unsafe fn fixtures_tab_scan(year: &mut u32, day: &mut u32, slot: &mut u8, forward: u32, comp_id: u32, count: i32, list: usize) -> u16 {
    type Scan = unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, u32, u32, i32, i32, i32, i32, i32, i32, i32,
                                            i32, usize, u32, i32, i32, u32) -> u32;
    let f: Scan = core::mem::transmute(FN_FIXTURE_MANAGER_SCAN_FIXTURES);
    f(read_usize(G_FIXTURE_MANAGER), year, day, slot, forward, comp_id, -1, -1, -1, -1, -1, -1, -1, count, list, 0, -2, -1, 1) as u16
}

/// The neighbouring match date of the Fixtures tab: from (`day`, `year`) one day back (`forward` 0, slot 2) or ahead
/// (`forward` 1, slot 0) (`Date::sub_days`/`add_days` and `Date::operator=`), then the first match from there
/// (`fixtures_tab_scan`, one, into the local list `scratch`); none found leaves the year 1 (no button). The day and
/// year go to parameters `param` and `param + 1` (their words).
unsafe fn fixtures_tab_neighbour_date(day: u32, year: u32, forward: u32, comp_id: u32, scratch: usize, param: u32) -> u32 {
    let (mut d, mut t, mut t2) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let dp = d.as_mut_ptr() as usize;
    exe::new_date(dp, day, year);
    let r = if forward != 0 {
        exe::date_add_days(dp, t.as_mut_ptr() as usize, 1)
    } else {
        exe::date_sub_days(dp, t.as_mut_ptr() as usize, 1)
    };
    exe::date_operator_3d(dp, t2.as_mut_ptr() as usize, r);
    let (mut n_day, mut n_year) = (read_u32(dp), read_u32(dp + 2));
    let mut slot: u8 = if forward != 0 { 0 } else { 2 };
    if fixtures_tab_scan(&mut n_year, &mut n_day, &mut slot, forward, comp_id, -1, scratch) == 0 {
        n_year = 1;
    }
    set_screen_param(param, (n_day & 0xffff) as usize);
    set_screen_param(param + 1, (n_year & 0xffff) as usize);
    n_year
}

/// Builds the Fixtures tab of the competition screen: one match date, its matches round by round, and the buttons to
/// the previous and next dates.
///
/// No competition: the comp_screens.cpp error box (line 0xaf5). The date comes from parameters 0x11 (day) and 0x12
/// (year), the neighbours from 0x13/0x14 and 0x15/0x16, the match list from 0x17. With no list yet: the current date
/// when the year is 0; a new LLIST (operator new 0xe) in parameter 0x17, filled with every match of the competition from
/// that date (`fixtures_tab_scan` forward from slot 0, all), sorted (`comp_util_sort_fixture_list`) or, with none, the
/// year 1; the day and year back to 0x11/0x12. Year 1: "No fixtures have been scheduled." in a text area. A neighbour
/// year 0 is looked for (`fixtures_tab_neighbour_date`: the previous date first, then the next). Then the two-column
/// area of the "<< Date" and "Date >>" cells: a button (ids 0x10 and 0x11) when that neighbour year is above 1, plain
/// text otherwise; the long date (without the year in the current year) as the heading; and the nine-column list
/// (`FIXTURES_TAB_WEIGHTS`, as many rows as fit in font 3): at each new stage and round (stage or round not 0, and
/// different from the previous match's) an empty row (except the first), the round's name
/// (`comp_util_format_stage_round_name`, up to 0x2f1 wide), a link (id 0x12, value round + 1000 × (stage + 1)) when the
/// round's fixtures are known or plain text, and an underline row (flags 0xc400); every match a row
/// (`comp_screens_add_fixture_row`, mode 0x13). The scroll row comes back from parameter 0x18 and the area goes to 0x19.
/// Original: `comp_screens_draw_fixtures_tab` (0x493ee0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_fixtures_tab(comp: usize, top: i32, bottom: i32) {
    let mut scratch = [0u8; 0x10];
    let lp = scratch.as_mut_ptr() as usize;
    exe::new_llist(lp);
    let weights = FIXTURES_TAB_WEIGHTS;
    if comp == 0 {
        comp_screens_error_box(0xaf5);
        exe::drop_llist(lp);
        return;
    }
    let mut day = get_screen_param(0x11) as u32;
    let mut year = get_screen_param(0x12) as u32;
    let _prev_day = get_screen_param(0x13);
    let mut prev_year = get_screen_param(0x14) as u32;
    let _next_day = get_screen_param(0x15);
    let mut next_year = get_screen_param(0x16) as u32;
    let mut list = get_screen_param(0x17);
    let pale = job_colour(G_GUI_COLOUR_PALE_GREY);
    let grey = job_colour(G_GUI_COLOUR_GREY);
    let yellow = job_colour(G_GUI_COLOUR_YELLOW);
    if list == 0 {
        covered!("tel22_3 fixtures list built");
        if year as u16 == 0 {
            day = read_u32(G_CURRENT_DATE);
            year = (year & 0xffff_0000) | read_u16(G_CURRENT_DATE_YEAR) as u32;
        }
        let block = exe::crt_operator_new(0xe);
        list = if block != 0 { exe::new_llist(block) } else { 0 };
        set_screen_param(0x17, list);
        let mut slot: u8 = 0;
        let comp_id = read_u32(read_usize(comp + 4));
        if fixtures_tab_scan(&mut year, &mut day, &mut slot, 1, comp_id, -2, list) == 0 {
            year = 1;
        } else {
            let sort: unsafe extern "C" fn(usize) = core::mem::transmute(0x4c0520usize);
            sort(list);
        }
        set_screen_param(0x11, (day & 0xffff) as usize);
        set_screen_param(0x12, (year & 0xffff) as usize);
    }
    if year as u16 == 1 {
        covered!("tel22_3 fixtures none");
        let rows = ((bottom - top - 0x10) / exe::font_get_height(3)) as u8;
        comp_lang_format()(JOB_TEXT, 0x99c3dc); // "No fixtures have been scheduled."
        tel26_text_area([0x6e, top as u32, 0x30c, bottom as u32, 2, 0, rows as u32, 3, pale, JOB_TEXT as u32, 0]);
        exe::drop_llist(lp);
        return;
    }
    if prev_year as u16 == 0 {
        prev_year = fixtures_tab_neighbour_date(day, year, 0, read_u32(read_usize(comp + 4)), lp, 0x13);
    }
    if next_year as u16 == 0 {
        next_year = fixtures_tab_neighbour_date(day, year, 1, read_u32(read_usize(comp + 4)), lp, 0x15);
    }
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x168, (top + 0x14) as u32, 2, 0, 1, 0, 1, 0, u32::MAX]);
    let mut label = [0u8; 0x100];
    let lb = label.as_mut_ptr() as usize;
    comp_lang_format()(lb, 0x988078); // "Date"
    comp_crt_format()(JOB_TEXT, 0x988090, lb); // "<< %s"
    if prev_year as u16 > 1 {
        tel26_area_button([area, 0, 0, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0x10, 0]);
    } else {
        tel26_area_text([area, 0, 0, 0x30, grey, 0x2c, 1, pale, JOB_TEXT as u32]);
    }
    comp_lang_format()(lb, 0x988078);
    comp_crt_format()(JOB_TEXT, 0x988088, lb); // "%s >>"
    if next_year as u16 > 1 {
        tel26_area_button([area, 1, 0, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0x11, 0]);
    } else {
        tel26_area_text([area, 1, 0, 0x30, grey, 0x2c, 1, pale, JOB_TEXT as u32]);
    }
    let y = top + 0x19;
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    exe::new_date(dp, day, year);
    if year as u16 == read_u16(G_CURRENT_DATE_YEAR) {
        exe::date_get_long_date_string_no_year(dp, JOB_TEXT, 0x7d0, 0xff);
    } else {
        exe::date_get_long_date_string(dp, JOB_TEXT, 0x7d0, 0xff);
    }
    comp_text_item([0x6e, y as u32, 0x30c, (y + 0x23) as u32, 2, 0, 0xc, 3, yellow, JOB_TEXT as u32, 1]);
    let y = y + 0x28;
    let rows = ((bottom - y - 0x10) / exe::font_get_height(3)) as u8;
    let area = tel26_add_area(current_human_display(),
        [0x6e, y as u32, 0x30c, bottom as u32, 9, weights.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    let mut row: i32 = 0;
    let mut prev: usize = 0;
    write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
    let mut f = exe::llist_get_next(list);
    while f != 0 {
        let same = prev != 0 && read_i16(f + 0x32) == read_i16(prev + 0x32) && read_i16(f + 0x30) == read_i16(prev + 0x30);
        if !same && !(read_i16(f + 0x32) == 0 && read_i16(f + 0x30) == 0) {
            if prev != 0 {
                tel26_area_text([area, 0, row as u32, 1, 0, 1, 1, pale, 0]);
                row += 1;
            }
            exe::comp_util_format_stage_round_name(read_i16(f + 0x32) as i32, read_i16(f + 0x30) as i32, JOB_TEXT, 0x7d0, 0);
            let x2 = (exe::font_text_width(3usize, JOB_TEXT as *const u8) + 0x73).min(0x2f1);
            if fixtures_tab_round_known(comp, read_i8(f + 0x42) as i32, read_i16(f + 0x34) as i32) != 0 {
                covered!("tel22_3 fixtures round link");
                let value = (read_i16(f + 0x34) as i32).wrapping_add((read_i8(f + 0x42) as i32 + 1).wrapping_mul(1000));
                tel26_add_item(current_human_display(), [2, 0x73, 0, x2 as u32, 0, 0, row as u32, 1, 0, 0, 1, 3, yellow,
                    JOB_TEXT as u32, 0, 0x12, value as u32, area]);
            } else {
                tel26_add_item(current_human_display(), [1, 0x73, 0, x2 as u32, 0, 0, row as u32, 1, 0, 0, 1, 3, yellow,
                    JOB_TEXT as u32, 0, 0, 0, area]);
            }
            row += 1;
            tel26_add_item(current_human_display(), [1, 0x73, 0, x2 as u32, 0, 0, row as u32, 0xc400, yellow, 0, 0, 0, 0,
                0, 0, 0, 0, area]);
            row += 1;
        }
        exe::comp_screens_add_fixture_row(f, area as usize, &mut row, 0x13);
        prev = f;
        f = exe::llist_get_next(list);
    }
    comp_tabs_restore_scroll(area, 0x18, 0x19);
    exe::drop_llist(lp);
}

/// Builds the Schedule tab of the competition screen: one row per match date of the competition, with the date (a link)
/// and the stage and round played on it.
///
/// No competition: the error box (line 0xc24). A competition not yet scheduled (`comp + 0x45` = 0): "The competition
/// has not been scheduled yet." in a text area. The date list comes from parameters 0x1a (the 0x14-byte entries) and
/// 0x1b (their count); with none yet it is built (`Comp::build_match_date_list`) and kept there. A two-column area
/// (weights 1 and 4, as many rows as fit in font 3); an entry repeating the previous one's date, stage (the parent stage
/// when there is one, `comp_util_get_parent_stage_id`) and round is skipped. The name: stage 0 and round 0 give "Game n"
/// (n = the entry's round + 1) in a league division, or the competition's short name; otherwise
/// `comp_util_format_stage_round_name`. Column 0 is the short weekday date (a link, id 0x14, value the entry); column 1
/// the name indented, a link (id 0x15, value round + 1000 × (first stage of the group + 1)) when the round's fixtures are
/// known, text in the 0xae3154 colour otherwise. Empty rows fill the area; the scroll row comes back from parameter
/// 0x1c and the area goes to 0x1d.
/// Original: `comp_screens_draw_schedule_tab` (0x4947a0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_schedule_tab(comp: usize, top: i32, bottom: i32) {
    let weights: [u8; 2] = [1, 4];
    if comp == 0 {
        comp_screens_error_box(0xc24);
        return;
    }
    let mut list = get_screen_param(0x1a);
    let mut count = get_screen_param(0x1b) as i32;
    let pale = job_colour(G_GUI_COLOUR_PALE_GREY);
    if read_u32(comp + 0x45) == 0 {
        covered!("tel22_3 schedule not yet");
        let rows = ((bottom - top - 0x10) / exe::font_get_height(3)) as u8;
        comp_lang_format()(JOB_TEXT, 0x99c2bc); // "The competition has not been scheduled yet."
        tel26_text_area([0x6e, top as u32, 0x30c, bottom as u32, 2, 0, rows as u32, 3, pale, JOB_TEXT as u32, 0]);
        return;
    }
    if list == 0 {
        covered!("tel22_3 schedule list built");
        let build: unsafe extern "thiscall" fn(usize, *mut i32) -> usize = core::mem::transmute(0x48d500usize);
        list = build(comp, &mut count);
        set_screen_param(0x1a, list);
        set_screen_param(0x1b, count as usize);
    }
    let rows = ((bottom - top - 0x10) / exe::font_get_height(3)) as u8;
    let area = tel26_add_area(current_human_display(),
        [0x6e, top as u32, 0x30c, bottom as u32, 2, weights.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    let (mut prev_stage, mut prev_round, mut prev_entry) = (-1i32, -1i32, 0usize);
    let mut row: i32 = 0;
    let mut name = [0u8; 0x800];
    let nb = name.as_mut_ptr() as usize;
    let mut k: i32 = 0;
    let mut e = list;
    while k < count {
        let mut stage = read_i16(e + 0x10) as i32;
        let round = read_i16(e + 0x12) as i32;
        let parent = exe::comp_util_get_parent_stage_id(stage);
        if parent > -1 {
            stage = parent;
        }
        let same = prev_entry != 0 && read_i16(e + 0xc) == read_i16(prev_entry + 0xc) && read_i16(e + 0xe) == read_i16(prev_entry + 0xe)
            && stage == prev_stage && round == prev_round;
        if !same {
            if stage == 0 && round == 0 {
                let record = read_usize(comp + 4);
                if exe::comp_util_is_league_division(read_i32(record)) != 0 {
                    comp_lang_format()(nb, 0x99c400, read_i32(e + 4) + 1); // "Game %d"
                } else {
                    copy_c_string(nb, record + competition::SHORT_NAME);
                }
            } else {
                exe::comp_util_format_stage_round_name(stage, round, nb, 0x7d0, 0);
            }
            let mut d = [0u8; 8];
            let dp = d.as_mut_ptr();
            exe::new_date(dp as usize, read_u16(e + 0xc) as u32, read_u16(e + 0xe) as u32);
            exe::date_get_short_weekday_date_string(dp, JOB_TEXT as *mut u8, 0x7d0, 0xff);
            tel26_area_button([area, 0, row as u32, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, pale, JOB_TEXT as u32, 0x14, e as u32]);
            let known = fixtures_tab_round_known(comp, read_i32(e), read_i32(e + 4));
            comp_crt_format()(JOB_TEXT, 0x987ff0, nb); // "  %s"
            if known != 0 {
                let first: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x48d2b0usize);
                let g = first(comp, read_i32(e));
                let value = read_i32(e + 4).wrapping_add((g + 1).wrapping_mul(1000));
                tel26_area_button([area, 1, row as u32, 2, 0, 1, 2, pale, JOB_TEXT as u32, 0x15, value as u32]);
            } else {
                tel26_area_text([area, 1, row as u32, 2, 0, 1, 2, job_colour(COMP_TABS_COLOUR_3154), JOB_TEXT as u32]);
            }
            prev_stage = stage;
            prev_round = round;
            prev_entry = e;
            row += 1;
        }
        k += 1;
        e += 0x14;
    }
    while row < rows as i32 {
        tel26_area_text([area, 0, row as u32, 2, 0, 1, 2, pale, 0]);
        tel26_area_text([area, 1, row as u32, 2, 0, 1, 2, pale, 0]);
        row += 1;
    }
    comp_tabs_restore_scroll(area, 0x1c, 0x1d);
}

/// Builds the Seedings tab of the competition screen: the seeded clubs in two halves side by side, each with its
/// seeding.
///
/// The seedings are the competition's 6-byte entries (`comp + 0x14`, count `comp + 0x36`: the club, the kind byte at
/// +4; kind 0 is not seeded). Rows: as many as fit in font 3 (the whole height); the list takes twice that when the
/// seeded clubs fit, otherwise their count rounded up to even. A four-column area; the first half goes in columns 0
/// and 1, the second in 2 and 3 (rows from the top again). Each slot takes the next seeded entry: the club indented, a
/// link (id 0x7d5, value the club) in yellow for a human's club, pale grey otherwise; its kind in column + 1: 3
/// "Seeded", 4 "Intertoto" and 5 "European Cup" (both links, id 0x7d0, to the competition records of [0x9cf6f8] and
/// [0x9cf6e8]), 7 "First seeds", 8 "Second seeds", 10 to 12 "Pool 1" to "Pool 3", nothing else. Slots past the last
/// entry are empty.
/// Original: `comp_screens_draw_seedings_tab` (0x494c60, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_seedings_tab(comp: usize, top: i32, bottom: i32) {
    let count = read_i16(comp + 0x36);
    let mut seeded: i16 = 0;
    if count > 0 {
        let mut p = read_usize(comp + 0x14) + 4;
        for _ in 0..count {
            if read_u8(p) != 0 {
                seeded = seeded.wrapping_add(1);
            }
            p += 6;
        }
    }
    let rows = ((bottom - top) / exe::font_get_height(3)) as i8;
    let total: i16 = if seeded as i32 <= rows as i32 * 2 {
        (rows as i16) << 1
    } else {
        covered!("tel22_3 seedings longer than the area");
        (((seeded as i32 + 1) / 2) * 2) as i16
    };
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 4, 0, rows as u8 as u32, 0, 0, 0, u32::MAX]);
    let pale = job_colour(G_GUI_COLOUR_PALE_GREY);
    let yellow = job_colour(G_GUI_COLOUR_YELLOW);
    let mut entry: i16 = 0;
    let mut counter: i16 = 0;
    let mut i: i32 = 0;
    if total <= 0 {
        return;
    }
    loop {
        let n = read_i16(comp + 0x36);
        if entry < n {
            while read_u8(read_usize(comp + 0x14) + entry as isize as usize * 6 + 4) == 0 {
                entry += 1;
                if entry >= n {
                    break;
                }
            }
        }
        let half = total as i32 / 2;
        let (col, row) = if i < half {
            (0u32, counter as u8 as i8 as i32)
        } else {
            (2u32, (counter as u8).wrapping_sub(half as u8) as i8 as i32)
        };
        if entry < n {
            let e = read_usize(comp + 0x14) + entry as isize as usize * 6;
            let club = read_usize(e);
            comp_crt_format()(JOB_TEXT, 0x987ff0, club + club::SHORT_NAME); // "  %s"
            let colour = if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 { yellow } else { pale };
            tel26_area_button([area, col, row as u32, 2, 0, 1, 3, colour, JOB_TEXT as u32, 0x7d5, read_u32(e)]);
            let label = match read_i8(e + 4) as i32 {
                3 => 0x99c468,  // "Seeded"
                4 => 0x99c440,  // "Intertoto"
                5 => 0x99c430,  // "European Cup"
                7 => 0x99c45c,  // "First seeds"
                8 => 0x99c44c,  // "Second seeds"
                10 => 0x99c428, // "Pool 1"
                11 => 0x99c420, // "Pool 2"
                12 => 0x99c418, // "Pool 3"
                _ => 0usize,
            };
            if label != 0 {
                comp_lang_format()(JOB_TEXT, label);
            } else {
                write_u8(JOB_TEXT, 0);
            }
            let kind = read_u8(e + 4);
            if kind == 4 || kind == 5 {
                covered!("tel22_3 seedings european link");
                let id = read_u32(if kind == 4 { SEEDINGS_TAB_COMP_KIND_4 } else { SEEDINGS_TAB_COMP_KIND_5 }) as usize;
                let record = read_usize(G_CLUB_COMP_TABLE).wrapping_add(id.wrapping_mul(SEEDINGS_TAB_COMPETITION_SIZE));
                tel26_area_button([area, col + 1, row as u32, 2, 0, 0xc, 2, yellow, JOB_TEXT as u32, 0x7d0, record as u32]);
            } else {
                tel26_area_text([area, col + 1, row as u32, 2, 0, 0xc, 2, yellow, JOB_TEXT as u32]);
            }
        } else {
            tel26_area_text([area, col, row as u32, 2, 0, 1, 2, pale, 0]);
            tel26_area_text([area, col + 1, row as u32, 2, 0, 1, 2, pale, 0]);
        }
        counter = counter.wrapping_add(1);
        i += 1;
        entry = entry.wrapping_add(1);
        if counter >= total {
            break;
        }
    }
}

/// Builds the Average Points tab of the competition screen (the relegation averages): each club's points in the last
/// two or three seasons, the total, the games and the average.
///
/// The stage comes from parameter 0x2a (−1 the competition itself; a stage past the last or without an averages table,
/// `+ 0x18`, also gives −1). The competition's virtual slot 0x34 runs with 0 first (`update_relegation_averages`, a
/// no-op except in the Argentine second division). When any stage or the competition has an averages table: a "Stage"
/// menu entry for each (`comp_screens_add_stage_menu_item`, at x 0x28f), the chosen one's name as the heading
/// (`comp_util_get_stage_name`) and the list 0x41 lower. Three seasons (8 columns, weights 2 10 4 4 4 4 4 4) unless the
/// first entry's third season games are 0xff (then 2 and 7 columns); the headings are the seasons
/// ("year/year+1" from the competition's table year back), "Total", "Played" and "Average". Each club entry (0x13 bytes;
/// club −1 skipped): its position, its name (a link, yellow for a human's club), the points of each season, their total,
/// the games of the seasons and the average ("%5.3f").
/// Original: `comp_screens_draw_average_points_tab` (0x495050, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_average_points_tab(comp: usize, top: i32, bottom: i32) {
    let weights8: [u8; 8] = [2, 0xa, 4, 4, 4, 4, 4, 4];
    let weights7: [u8; 7] = [2, 0xa, 4, 4, 4, 4, 4];
    let mut menu_state: i32 = -1;
    let stage_at = |i: i32| read_usize(read_usize(comp + 0xc) + (i as isize * 4) as usize);
    let mut chosen = get_screen_param(0x2a) as u8 as i8;
    if chosen as i32 > read_i32(comp + 0x30) {
        chosen = -1;
    } else if chosen > -1 && read_usize(stage_at(chosen as i32) + 0x18) == 0 {
        chosen = -1;
    }
    // comp_vtable slot 0x34: update_relegation_averages(0) (ArgSecond); the others pop the argument and return
    let update: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(read_usize(read_usize(comp) + 0x34));
    update(comp, 0);
    let stages = read_i32(comp + 0x30);
    let mut has_table = false;
    if stages >= 0 {
        let mut i: i8 = 0;
        loop {
            if read_usize(stage_at(i as i32) + 0x18) != 0 {
                has_table = true;
                break;
            }
            i = i.wrapping_add(1);
            if i as i32 > stages {
                break;
            }
        }
    }
    let yellow = job_colour(G_GUI_COLOUR_YELLOW);
    let pale = job_colour(G_GUI_COLOUR_PALE_GREY);
    let grey = job_colour(G_GUI_COLOUR_GREY);
    let mut obj = comp;
    let mut y = top;
    if has_table {
        covered!("tel22_3 average points stage menu");
        type MenuItem = unsafe extern "C" fn(*mut i32, u32, i32, u32, i32, u32, i32, u32, i32, u32);
        let menu_item: MenuItem = core::mem::transmute(0x49d930usize);
        if stages >= -1 {
            let mut i: i8 = -1;
            loop {
                let o = if i == -1 { comp } else { stage_at(i as i32) };
                if read_usize(o + 0x18) != 0 {
                    menu_item(&mut menu_state, 0x28f, top, 0x30c, top + 0x14, 0x99c518, read_i16(o + 0x3a) as i32, 0, i as i32, 0); // "Stage"
                }
                i = i.wrapping_add(1);
                if i as i32 > read_i32(comp + 0x30) {
                    break;
                }
            }
        }
        y = top + 0x19;
        write_u32(0xadae0c, 1);
        obj = if chosen == -1 { comp } else { stage_at(chosen as i32) };
        exe::comp_util_get_stage_name(JOB_TEXT as *mut u8, 0x7d0, read_u16(obj + 0x3a) as u32, 0);
        comp_text_item([0x6e, y as u32, 0x30c, (y + 0x23) as u32, 2, 0, 0xc, 3, yellow, JOB_TEXT as u32, 1]);
        y += 0x28;
    }
    let rows = ((bottom - y) / exe::font_get_height(3)) as u8;
    let seasons: u8;
    let area = if read_u8(read_usize(obj + 0x18) + 0x12) == 0xff {
        seasons = 2;
        tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, bottom as u32, 7, weights7.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX])
    } else {
        seasons = 3;
        tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, bottom as u32, 8, weights8.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX])
    };
    let mut col: u32 = 2;
    let mut back = seasons as i32 - 1;
    while back >= 0 {
        let year = read_u16(read_usize(comp + 0x18) + 8) as i32 - back;
        comp_lang_format()(JOB_TEXT, 0x99c4e8, year, year + 1); // "%d/%d"
        tel26_area_text([area, col, 0, 0x30, grey, 0xc, 2, pale, JOB_TEXT as u32]);
        col += 1;
        back -= 1;
    }
    comp_lang_format()(JOB_TEXT, 0x99c4c4, 0xad9c64usize); // "Total"
    tel26_area_text([area, col, 0, 0x30, grey, 0xc, 2, pale, JOB_TEXT as u32]);
    col += 1;
    comp_lang_format()(JOB_TEXT, 0x99c4a0, 0xad9c64usize); // "Played"
    tel26_area_text([area, col, 0, 0x30, grey, 0xc, 2, pale, JOB_TEXT as u32]);
    comp_lang_format()(JOB_TEXT, 0x99c478, 0xad9c64usize); // "Average"
    tel26_area_text([area, col + 1, 0, 0x30, grey, 0xc, 2, pale, JOB_TEXT as u32]);
    let numbers = job_colour(COMP_TABS_COLOUR_3152);
    let mut row: u32 = 1;
    let mut k: i16 = 0;
    while k < read_i16(obj + 0x34) {
        let e = read_usize(obj + 0x18) + (k as i32 * 0x13) as usize;
        let id = read_i32(e);
        if id != -1 {
            comp_crt_format()(JOB_TEXT, 0x98806c, row); // "%d"
            tel26_area_text([area, 0, row, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, pale, JOB_TEXT as u32]);
            let club = read_usize(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(club::SIZE));
            let human = exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0;
            comp_crt_format()(JOB_TEXT, 0x987ff0, club + club::SHORT_NAME);
            tel26_area_button([area, 1, row, 1, 0, 1, 3, if human { yellow } else { pale }, JOB_TEXT as u32, 0x7d5, club as u32]);
            let points = |o: usize| read_i16(e + o) as i32;
            comp_crt_format()(JOB_TEXT, 0x98806c, points(0xa));
            tel26_area_text([area, 2, row, 1, 0, 0xc, 2, numbers, JOB_TEXT as u32]);
            comp_crt_format()(JOB_TEXT, 0x98806c, points(0xd));
            tel26_area_text([area, 3, row, 1, 0, 0xc, 2, numbers, JOB_TEXT as u32]);
            let mut c: u32 = 4;
            let total = if seasons == 3 {
                comp_crt_format()(JOB_TEXT, 0x98806c, points(0x10));
                tel26_area_text([area, 4, row, 1, 0, 0xc, 2, numbers, JOB_TEXT as u32]);
                c = 5;
                points(0xd) + points(0x10) + points(0xa)
            } else {
                points(0xd) + points(0xa)
            };
            comp_crt_format()(JOB_TEXT, 0x98806c, total);
            tel26_area_text([area, c, row, 1, 0, 0xc, 2, numbers, JOB_TEXT as u32]);
            c += 1;
            let games = |o: usize| read_i8(e + o) as i32;
            let played = if seasons == 3 { games(0xf) + games(0x12) + games(0xc) } else { games(0xf) + games(0xc) };
            comp_crt_format()(JOB_TEXT, 0x98806c, played);
            tel26_area_text([area, c, row, 1, 0, 0xc, 2, yellow, JOB_TEXT as u32]);
            comp_crt_format()(JOB_TEXT, 0x99c470, read_f32(e + 4) as f64); // "%5.3f"
            tel26_area_text([area, c + 1, row, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 2, yellow, JOB_TEXT as u32]);
            row += 1;
        }
        k = k.wrapping_add(1);
    }
}

/// A popup area for a View or Filter button (empty, flags 0x130, the popup colour, owned by the button).
unsafe fn team_stats_tab_popup_area(owner: u32) -> u32 {
    tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), owner])
}

/// An item of a stats popup list with the text in `JOB_TEXT`: `kind` 0x2002 (ticked) or 2, the popup colours, the id
/// and the value.
unsafe fn team_stats_tab_menu_item(kind: u32, id: u32, value: u32, area: u32) {
    tel26_add_item(current_human_display(), [kind, 0, 0, 0, 0, 0, 0, 0x10, job_colour(JOB_COLOUR_31AA), job_colour(G_GUI_COLOUR_YELLOW), 1, 1,
        job_colour(G_GUI_COLOUR_BLACK), JOB_TEXT as u32, 0, id, value, area]);
}

/// Builds the Team Stats tab of the competition screen: the clubs ranked by one statistic, chosen in the View popup.
///
/// The statistic comes from parameter 0x23, the sorted table from 0x24 and its count from 0x25. The View button
/// (0x6e..0xeb) opens a popup with the 17 statistics (`comp_stats_get_team_stat_label`, items with id 0x16 and the
/// statistic as value; the chosen one ticked, and its label, without the leading space, is the heading). A four-column
/// list (weights 2 12 10 3) below the heading. With no table yet: `comp_stats_get_sorted_team_table` (at most 0x4b) into
/// parameters 0x24/0x25; none: no rows. Each entry (0x76 bytes; club −1 skipped): the position (`utils_format_ordinal`),
/// the club (a link, yellow for a human's club), in a national team competition (`comp + 0x42` = 2) the club's nation (a
/// link to its national team), and the value (`comp_stats_format_team_stat`). The scroll row comes back from parameter
/// 0x26 and the area goes to 0x27.
/// Original: `comp_screens_draw_team_stats_tab` (0x4956e0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_team_stats_tab(comp: usize, top: i32, bottom: i32) {
    let weights: [u8; 4] = [2, 0xc, 0xa, 3];
    let stat = get_screen_param(0x23) as u8;
    let mut table = get_screen_param(0x24);
    let mut count = get_screen_param(0x25) as i32;
    let pale = job_colour(G_GUI_COLOUR_PALE_GREY);
    let yellow = job_colour(G_GUI_COLOUR_YELLOW);
    comp_lang_format()(JOB_TEXT, 0x988004); // "View"
    let button = tel26_add_button([0x6e, top as u32, 0xeb, (top + 0x14) as u32, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, pale, JOB_TEXT as u32, 0, 0]);
    let popup = team_stats_tab_popup_area(button);
    write_u32(0xadae0c, 1);
    let chosen = stat as i8 as u32;
    let label_fn: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32 = core::mem::transmute(0x4a2810usize);
    let mut label = [0u8; 0xc8];
    // simplification: with no statistic ticked the original's heading is stack garbage; empty here
    let mut heading = [0u8; 0x100];
    for k in 1..=0x11u32 {
        label_fn(read_usize(comp + 8), k, label.as_mut_ptr() as usize, 0xc8);
        comp_crt_format()(JOB_TEXT, 0x98ed14, label.as_ptr() as usize); // " %s"
        if k == chosen {
            team_stats_tab_menu_item(0x2002, 0x16, k, popup);
            copy_c_string(heading.as_mut_ptr() as usize, JOB_TEXT + 1);
        } else {
            team_stats_tab_menu_item(2, 0x16, k, popup);
        }
    }
    let y = top + 0x19;
    comp_text_item([0x6e, y as u32, 0x30c, (y + 0x23) as u32, 2, 0, 0xc, 3, yellow, heading.as_ptr() as u32, 1]);
    let y = y + 0x28;
    let rows = ((bottom - y - 0x10) / exe::font_get_height(3)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, bottom as u32, 4, weights.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    let mut row: i32 = 0;
    if table == 0 {
        covered!("tel22_3 team stats table built");
        // simplification: the original passes the statistic's dword slot (its other bytes stack residue); the callee reads the byte
        table = exe::comp_stats_get_sorted_team_table(read_usize(comp + 8), stat as u32, &mut count, 0x4b);
        set_screen_param(0x24, table);
        set_screen_param(0x25, count as usize);
        if table == 0 {
            comp_tabs_restore_scroll(area, 0x26, 0x27);
            return;
        }
    }
    let format: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(0x4a39c0usize);
    let mut k: i32 = 0;
    let mut e = table;
    while k < count {
        let id = read_i32(e);
        if id != -1 {
            let club = read_usize(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(club::SIZE));
            let place = exe::utils_format_ordinal(row + 1, -1);
            tel26_area_text([area, 0, row as u32, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, pale, place as u32]);
            let human = exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0;
            comp_crt_format()(JOB_TEXT, 0x987ff0, club + club::NAME); // "  %s"
            tel26_area_button([area, 1, row as u32, 1, 0, 9, 3, if human { yellow } else { pale }, JOB_TEXT as u32, 0x7d5, club as u32]);
            if read_u8(comp + 0x42) == 2 && read_usize(club + club::NATION) != 0 {
                let team = exe::database_get_national_team(read_usize(club + club::NATION));
                tel26_area_button([area, 2, row as u32, 1, 0, 1, 2, yellow, (read_usize(club + club::NATION) + 0x38) as u32, 0x7d5, team as u32]);
            }
            format(read_usize(comp + 8), e, stat as u32, JOB_TEXT, 0x7d0, 1);
            tel26_area_text([area, 3, row as u32, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, pale, JOB_TEXT as u32]);
            row += 1;
        }
        k += 1;
        e += 0x76;
    }
    comp_tabs_restore_scroll(area, 0x26, 0x27);
}

/// Builds the Player Stats tab of the competition screen: the players ranked by one statistic (View popup), filtered
/// by position (Filter popup).
///
/// The statistic comes from parameter 0x1e, the filter from 0x36, the sorted table from 0x1f and its count from 0x20.
/// The View popup has the 17 statistics (`comp_stats_get_player_stat_label`, id 0x17; the chosen one ticked and its
/// label the heading); the Filter button (0x28f..0x30c) opens Goalkeepers (1), Defenders (4), Midfielders (0x10),
/// Attackers (0x40) and All Players (−1) (id 0x1b), the chosen one ticked and named in the heading ("... (Goalkeepers)").
/// With no table yet: `Comp_Stats::get_sorted_player_table` (at most 0x4b, with the filter) into parameters 0x1f/0x20;
/// none: no rows. Each entry (0x2d bytes; player −1 skipped): the position, the player's name (a link, id 0x7d7, yellow
/// when he plays for a human's club, or a human's national team in a national team competition, `comp + 0x42` = 4), his
/// club or nation (a link), and the value (`Comp_Stats::format_player_stat`). The scroll row comes back from parameter
/// 0x21 and the area goes to 0x22.
/// Original: `comp_screens_draw_player_stats_tab` (0x495bc0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_player_stats_tab(comp: usize, top: i32, bottom: i32) {
    let weights: [u8; 4] = [2, 0xc, 0xa, 3];
    let stat = get_screen_param(0x1e) as u8;
    let filter = get_screen_param(0x36) as u8;
    let mut table = get_screen_param(0x1f);
    let mut count = get_screen_param(0x20) as i32;
    let pale = job_colour(G_GUI_COLOUR_PALE_GREY);
    let grey = job_colour(G_GUI_COLOUR_GREY);
    let yellow = job_colour(G_GUI_COLOUR_YELLOW);
    comp_lang_format()(JOB_TEXT, 0x988004); // "View"
    let button = tel26_add_button([0x6e, top as u32, 0xeb, (top + 0x14) as u32, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0, 0]);
    let popup = team_stats_tab_popup_area(button);
    write_u32(0xadae0c, 1);
    let chosen = stat as i8 as u32;
    let label_fn: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32 = core::mem::transmute(0x4a2b10usize);
    let mut label = [0u8; 0xc8];
    // simplification: with no statistic ticked the original's heading is stack garbage; empty here
    let mut heading = [0u8; 0xc8];
    let mut line = [0u8; 0x800];
    let lb = line.as_mut_ptr() as usize;
    for k in 1..=0x11u32 {
        label_fn(read_usize(comp + 8), k, label.as_mut_ptr() as usize, 0xc8);
        comp_crt_format()(JOB_TEXT, 0x98ed14, label.as_ptr() as usize); // " %s"
        if k == chosen {
            team_stats_tab_menu_item(0x2002, 0x17, k, popup);
            copy_c_string(heading.as_mut_ptr() as usize, JOB_TEXT + 1);
        } else {
            team_stats_tab_menu_item(2, 0x17, k, popup);
        }
    }
    comp_lang_format()(JOB_TEXT, 0x98ebf8); // "Filter"
    let button = tel26_add_button([0x28f, top as u32, 0x30c, (top + 0x14) as u32, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0, 0]);
    let popup = team_stats_tab_popup_area(button);
    copy_c_string(lb, heading.as_ptr() as usize);
    // the filters: value, the item text, the heading "<stat> (<position>)"
    for (value, text, titled) in [(1u8, 0x99c710usize, 0x99c6e0usize), (4, 0x99c694, 0x99c664), (0x10, 0x99c618, 0x99c5e8),
                                  (0x40, 0x99c59c, 0x99c56c), (0xff, 0x99c520, 0)] {
        comp_lang_format()(JOB_TEXT, text, 0xad9c64usize);
        let on = filter == value;
        team_stats_tab_menu_item(if on { 0x2002 } else { 2 }, 0x1b, value as i8 as u32, popup);
        if on && titled != 0 {
            comp_lang_format()(lb, titled, heading.as_ptr() as usize);
        }
    }
    let y = top + 0x19;
    comp_text_item([0x6e, y as u32, 0x30c, (y + 0x23) as u32, 2, 0, 0xc, 3, yellow, lb as u32, 1]);
    let y = y + 0x28;
    let rows = ((bottom - y - 0x10) / exe::font_get_height(3)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, bottom as u32, 4, weights.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    let mut row: i32 = 0;
    if table == 0 {
        covered!("tel22_3 player stats table built");
        let sorted: unsafe extern "thiscall" fn(usize, u32, *mut i32, u32, u32) -> usize = core::mem::transmute(0x4a4dc0usize);
        // simplification: the statistic goes as its dword slot and the filter as a sign-extended word in the original
        // (stack residue and register garbage above them); the callee reads the low parts
        table = sorted(read_usize(comp + 8), stat as u32, &mut count, 0x4b, filter as i8 as i16 as u16 as u32);
        set_screen_param(0x1f, table);
        set_screen_param(0x20, count as usize);
        if table == 0 {
            comp_tabs_restore_scroll(area, 0x21, 0x22);
            return;
        }
    }
    let format: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(0x4a3ee0usize);
    let mut k: i32 = 0;
    let mut p = table + 4;
    while k < count {
        let id = read_i32(p);
        if id != -1 {
            let staff = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
            let place = exe::utils_format_ordinal(row + 1, -1);
            tel26_area_text([area, 0, row as u32, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, pale, place as u32]);
            if read_u8(comp + 0x42) == 4 {
                covered!("tel22_3 player stats national");
                let nation = read_usize(staff + staff::NATION);
                let human = nation != 0 && exe::human_manager_is_human_club(exe::database_get_national_team(nation), 0, 1, 0, 0) != 0;
                exe::database_format_staff_name(staff, lb, 0x7d0, 1);
                comp_crt_format()(JOB_TEXT, 0x987ff0, lb); // "  %s"
                tel26_area_button([area, 1, row as u32, 1, 0, 9, 3, if human { yellow } else { pale }, JOB_TEXT as u32, 0x7d7, staff as u32]);
                let nation = read_usize(staff + staff::NATION);
                if nation != 0 {
                    let team = exe::database_get_national_team(nation);
                    tel26_area_button([area, 2, row as u32, 1, 0, 1, 2, yellow, (read_usize(staff + staff::NATION) + 0x38) as u32, 0x7d5, team as u32]);
                }
            } else {
                let club = read_usize(staff + staff::CLUB_JOB);
                let human = club != 0 && exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0;
                exe::database_format_staff_name(staff, lb, 0x7d0, 1);
                comp_crt_format()(JOB_TEXT, 0x987ff0, lb);
                tel26_area_button([area, 1, row as u32, 1, 0, 9, 3, if human { yellow } else { pale }, JOB_TEXT as u32, 0x7d7, staff as u32]);
                let club = read_usize(staff + staff::CLUB_JOB);
                if club != 0 {
                    tel26_area_button([area, 2, row as u32, 1, 0, 1, 2, yellow, (club + 0x38) as u32, 0x7d5, club as u32]);
                }
            }
            format(read_usize(comp + 8), p - 4, stat as u32, JOB_TEXT, 0x7d0, 1);
            tel26_area_text([area, 3, row as u32, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, pale, JOB_TEXT as u32]);
            row += 1;
        }
        k += 1;
        p += 0x2d;
    }
    comp_tabs_restore_scroll(area, 0x21, 0x22);
}

/// Comparer for `comp_screens_draw_fixtures_tab`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_fixtures_tab(comp: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(%%I0%%);
    // simplification: the first build allocates the match list (parameter 0x17) and the error box shows a message:
    // original only (proven in Rust mode)
    if comp == 0 || get_screen_param(0x17) == 0 {
        return o(comp, top, bottom);
    }
    compare_match_screen_build("competition screen, Fixtures tab", &[], || o(comp, top, bottom), || comp_screens_draw_fixtures_tab(comp, top, bottom));
}

/// Comparer for `comp_screens_draw_schedule_tab`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_schedule_tab(comp: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(%%I1%%);
    // simplification: building the date list (parameter 0x1a) allocates, and the error box: original only
    if comp == 0 || (read_u32(comp + 0x45) != 0 && get_screen_param(0x1a) == 0) {
        return o(comp, top, bottom);
    }
    compare_match_screen_build("competition screen, Schedule tab", &[], || o(comp, top, bottom), || comp_screens_draw_schedule_tab(comp, top, bottom));
}

/// Comparer for `comp_screens_draw_seedings_tab`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_seedings_tab(comp: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(%%I2%%);
    compare_match_screen_build("competition screen, Seedings tab", &[], || o(comp, top, bottom), || comp_screens_draw_seedings_tab(comp, top, bottom));
}

/// Comparer for `comp_screens_draw_average_points_tab`: side by side over the display list (the averages update runs
/// in both, recomputing the same values from the season points).
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_average_points_tab(comp: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(%%I3%%);
    compare_match_screen_build("competition screen, Average Points tab", &[], || o(comp, top, bottom), || comp_screens_draw_average_points_tab(comp, top, bottom));
}

/// Comparer for `comp_screens_draw_team_stats_tab`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_team_stats_tab(comp: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(%%I4%%);
    // simplification: sorting the table (parameter 0x24) allocates: original only
    if get_screen_param(0x24) == 0 {
        return o(comp, top, bottom);
    }
    compare_match_screen_build("competition screen, Team Stats tab", &[], || o(comp, top, bottom), || comp_screens_draw_team_stats_tab(comp, top, bottom));
}

/// Comparer for `comp_screens_draw_player_stats_tab`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_player_stats_tab(comp: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(%%I5%%);
    // simplification: sorting the table (parameter 0x1f) allocates: original only
    if get_screen_param(0x1f) == 0 {
        return o(comp, top, bottom);
    }
    compare_match_screen_build("competition screen, Player Stats tab", &[], || o(comp, top, bottom), || comp_screens_draw_player_stats_tab(comp, top, bottom));
}
