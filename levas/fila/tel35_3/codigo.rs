// ---------- batch %%LEVA%%: the retire and holiday screens and two list comparers (manager_screens.cpp) ----------

/// A thiscall routine of the exe with one argument.
pub(crate) unsafe fn tel35_this1(f: usize, this: usize, a: usize) -> usize {
    let g: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(f);
    g(this, a)
}

/// A thiscall routine of the exe with two arguments.
pub(crate) unsafe fn tel35_this2(f: usize, this: usize, a: usize, b: usize) -> usize {
    let g: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(f);
    g(this, a, b)
}

/// `HUMAN_MANAGER::get_holiday_option(option, 0)` (0x5fea50) of the human manager table: the option's byte.
unsafe fn holiday_option(option: u32) -> u8 {
    exe::human_manager_get_holiday_option(read_usize(G_HUMAN_MANAGER), option, 0usize)
}

/// `HUMAN_MANAGER::set_holiday_option(option, value, 0)` (0x5feb60, thiscall, ret 0xc).
unsafe fn set_holiday_option(option: u32, value: i32) {
    let g: unsafe extern "thiscall" fn(usize, u32, i32, u32) = core::mem::transmute(0x5feb60usize);
    g(read_usize(G_HUMAN_MANAGER), option, value, 0)
}

/// The background picture of the manager's screens: his club's, else his national team's, else a random one.
unsafe fn manager_job_background(manager: usize) {
    let club = read_usize(manager + staff::CLUB_JOB);
    let nation = read_usize(manager + staff::NATIONAL_JOB);
    if club != 0 {
        exe::background_set_club_picture(club, 0, 0);
    } else if nation != 0 {
        exe::background_set_club_picture(nation, 0, 0);
    } else {
        exe::background_set_random_picture();
    }
}

/// The No (id 1) and Yes (id 2) buttons of a manager confirm screen: the area (0x64, 0x22b)-(0x316, 0x24e), two columns
/// with the weights `weights` (3, 1).
unsafe fn manager_no_yes(weights: &[u8; 2]) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let area = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    text(JOB_TEXT, 0x98e8cc);
    tel26_area_button([area, 0, 0, 0x30, tel35_colour(G_GUI_COLOUR_GREY), 0xc, 3, tel35_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 1, 0]);
    text(JOB_TEXT, 0x98e8d0);
    tel26_area_button([area, 1, 0, 0x30, tel35_colour(G_GUI_COLOUR_GREY), 0xc, 3, tel35_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 2, 0]);
}

/// Rule: the retire screen opens for the current human manager.
///
/// `open_retire_screen()`: `open_screen` with the builder 0x6b28d0, the handler 0x6b2b40, single instance, no leave
/// routine.
/// Original: `open_retire_screen` (0x6b28b0, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_retire_screen() {
    covered!("tel35_3 open retire");
    exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x6b28d0usize, 0x6b2b40usize, 1, 0usize, 0);
}

/// Rule: the retire screen asks the human manager to confirm resigning his club and national jobs and retiring.
///
/// `retire_screen_build()`: the slot's manager; the background of his club, national team or a random one; the menu bar
/// (4, 0); the title (his name, `database_format_staff_name` 2000 bytes, style 1) in the kit colours (kit 1) of his
/// club (or national team), or in the heading colours 0 and 1 with neither; the subtitle "Please Confirm"; the question
/// with both jobs ("{}<club>{} and {}<nation>{}", their names +0x38 and gender chars +0x52), only the club, only the
/// nation (name +4, gender char +0x37) or "Retire from football ?", as a text area (0x6e, 0x91)-(0x30c, 0x217); No and
/// Yes.
/// Original: `retire_screen_build` (0x6b28d0, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn retire_screen_build() {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    covered!("tel35_3 retire");
    let weights: [u8; 2] = [3, 1];
    let manager = tel26_slot_manager().addr();
    manager_job_background(manager);
    exe::menubar_build(4, 0);
    let (mut fore, mut back) = (0u32, 0u32);
    let team = if read_usize(manager + staff::CLUB_JOB) != 0 { read_usize(manager + staff::CLUB_JOB) } else { read_usize(manager + staff::NATIONAL_JOB) };
    if team != 0 {
        exe::database_get_club_kit_colours(team, 1, &mut fore, &mut back);
    } else {
        fore = exe::get_heading_colour(1) as u32;
        back = exe::get_heading_colour(0) as u32;
    }
    exe::database_format_staff_name(manager, JOB_TEXT, 0x7d0, 1);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back & 0xffff, fore & 0xffff, JOB_TEXT);
    text(JOB_TEXT, 0x9909b8);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let club = read_usize(manager + staff::CLUB_JOB);
    let nation = read_usize(manager + staff::NATIONAL_JOB);
    let gender = |t: usize, at: usize| read_u8(t + at) as i8 as i32;
    if club != 0 && nation != 0 {
        text(JOB_TEXT, 0xa15590, gender(club, 0x52), gender(club, 0x52), gender(nation, 0x52), gender(nation, 0x52), club + 0x38, nation + 0x38);
    } else if club != 0 {
        text(JOB_TEXT, 0xa15550, gender(club, 0x37), gender(club, 0x37), club + 4);
    } else if nation != 0 {
        text(JOB_TEXT, 0xa1550c, gender(nation, 0x37), gender(nation, 0x37), nation + 4);
    } else {
        text(JOB_TEXT, 0xa154f4);
    }
    tel26_text_area([0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, tel35_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 0]);
    manager_no_yes(&weights);
}

/// Rule: confirming the retirement takes the manager out of his jobs and out of the game.
///
/// `retire_screen_handle_action(item)`: id 1 (No) -9. Id 2 (Yes), the slot's manager: his club (when he has one) loses
/// its manager (`MANAGER_MANAGER::remove_club_manager` departure 3), the match day's human interest refreshed, the
/// vacancy opened with him as the previous manager and the club's transfer offers cancelled; the same for his national
/// team (without the offers); his inbox cleared, holiday, reserve control and computer control off, his pending
/// offers, contract and loan contract removed, he leaves the national squads, his staff history rewound and his human
/// manager record cleared, his scouting lists cleared; his three names emptied, +0x5f = 0, the date at +0x10 set to
/// day 1 of 1966, +0x18 = 1966, no national (+0x24) or club (+0x39) job, and his screen record removed; -11. Otherwise
/// the menu bar.
/// Original: `retire_screen_handle_action` (0x6b2b40, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn retire_screen_handle_action(raw: u32) -> i32 {
    match tel26_item_id(raw as i16) {
        1 => -9,
        2 => {
            covered!("tel35_3 retire confirmed");
            let manager = tel26_slot_manager().addr();
            let club = read_usize(manager + staff::CLUB_JOB);
            if club != 0 {
                exe::manager_manager_remove_club_manager(read_usize(G_MANAGER_MANAGER), club, 3, 0, 0);
                if read_usize(G_MATCH_DAY) != 0 {
                    exe::match_day_refresh_human_interest_flags(read_usize(G_MATCH_DAY));
                }
                exe::manager_manager_add_job_vacancy(read_usize(G_MANAGER_MANAGER), club, manager);
                exe::transfer_manager_cancel_club_offers(read_usize(G_TRANSFER_MANAGER), club);
            }
            let nation = read_usize(manager + staff::NATIONAL_JOB);
            if nation != 0 {
                exe::manager_manager_remove_club_manager(read_usize(G_MANAGER_MANAGER), nation, 3, 0, 0);
                if read_usize(G_MATCH_DAY) != 0 {
                    exe::match_day_refresh_human_interest_flags(read_usize(G_MATCH_DAY));
                }
                exe::manager_manager_add_job_vacancy(read_usize(G_MANAGER_MANAGER), nation, manager);
            }
            exe::news_clear_inbox(read_usize(G_NEWS), manager);
            exe::human_manager_set_on_holiday(read_usize(G_HUMAN_MANAGER), 0, manager);
            tel35_this2(0x5fd590, read_usize(G_HUMAN_MANAGER), 0, manager);
            tel35_this2(0x5fe140, read_usize(G_HUMAN_MANAGER), 0, manager);
            exe::contract_manager_remove_pending_offers_for_staff(read_usize(G_CONTRACT_MANAGER), manager);
            exe::contract_manager_remove_contract(read_usize(G_CONTRACT_MANAGER), manager);
            tel35_this1(0x4d0bd0, read_usize(G_CONTRACT_MANAGER), manager);
            tel35_this1(0x775ba0, read_usize(G_NATIONAL_TEAMS), manager);
            tel35_this1(0x86fb80, read_usize(G_STAFF_RECORDS), manager);
            tel35_this1(0x86fc70, read_usize(G_STAFF_RECORDS), manager);
            tel35_this1(0x7fc310, read_usize(G_SCOUT_MANAGER), manager);
            let first = read_usize(manager + staff::FIRST_NAME);
            write_u8(manager + staff::CLASSIFICATION, 0);
            write_u8(first, 0);
            write_u8(read_usize(manager + staff::SECOND_NAME), 0);
            write_u8(read_usize(manager + staff::COMMON_NAME), 0);
            exe::date_set(manager + staff::DATE_OF_BIRTH, 1, 0, 0x7ae, -1);
            write_u16(manager + staff::YEAR_OF_BIRTH, 0x7ae);
            write_u32(manager + staff::NATIONAL_JOB, 0);
            write_u32(manager + staff::CLUB_JOB, 0);
            exe::screenmanager_remove_manager_record(G_SCREEN_MANAGER, manager, -1);
            -11
        }
        _ => manager_menu_bar(raw),
    }
}

/// Rule: the holiday screen opens for the current human manager.
///
/// `open_holiday_screen()`: `open_screen` with the builder 0x6b2d80, the handler 0x6b3440, single instance, no leave
/// routine.
/// Original: `open_holiday_screen` (0x6b2d60, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_holiday_screen() {
    covered!("tel35_3 open holiday");
    exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x6b2d80usize, 0x6b3440usize, 1, 0usize, 0);
}

/// The holiday settings, in order: the text, the link value, the (option, value) whose match lights the line, the rows
/// to skip before it. A value of -1 lights the line when the option is not 0.
const HOLIDAY_SETTINGS: [(usize, u32, (u32, i32), i32); 7] = [
    (0xa156c8, 0, (0, 1), 2), // "Reject all offers for players"
    (0xa156a0, 1, (0, 2), 1), // "Reject all offers below asking price"
    (0xa15688, 2, (4, 1), 1), // "Do not buy any players"
    (0xa15668, 3, (4, 2), 1), // "Only buy players from shortlist"
    (0xa15630, 4, (1, -1), 2), // "Apply for manager position at higher reputation clubs"
    (0xa15614, 5, (2, -1), 1), // "Use current match tactics"
    (0xa155e8, 6, (3, -1), 1), // "Use current team selection when possible"
];

/// Rule: the holiday screen asks to confirm going on holiday and shows the holiday settings, the ones on in pale grey.
///
/// `holiday_screen_build()`: a local Text; the slot's manager; the background of his club, national team or a random
/// one; the menu bar (4, 0); the title (his name) in the heading colours 0 and 1; the subtitle "Please Confirm" (0);
/// the area (0x6e, 0x91)-(0x30c, 0x217) (kind 2); "Go on holiday ?" (font 4); the settings area (0x82, 0xfa)-(0x2f8,
/// 0x217), one column, 0x122 / the height of font 3 rows, kind 1, with "Settings" at row 0 and each setting
/// (`HOLIDAY_SETTINGS`: a Text with the link 3 of its value) drawn wrapped from 0x96 to 0x2f8, pale grey when on
/// (`HUMAN_MANAGER::get_holiday_option`) and grey when off, rows skipped as the table says; No and Yes.
/// Original: `holiday_screen_build` (0x6b2d80, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn holiday_screen_build() {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    covered!("tel35_3 holiday");
    let weights: [u8; 2] = [3, 1];
    let mut tx = [0u32; 8];
    let t = tx.as_mut_ptr() as usize;
    exe::new_text(tx.as_mut_ptr() as *mut usize, 0usize);
    let manager = tel26_slot_manager().addr();
    manager_job_background(manager);
    exe::menubar_build(4, 0);
    exe::database_format_staff_name(manager, JOB_TEXT, 0x7d0, 1);
    let fore = exe::get_heading_colour(1) as u32;
    let back = exe::get_heading_colour(0) as u32;
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, JOB_TEXT);
    text(JOB_TEXT, 0x9909b8);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 0);
    tel26_add_area(current_human_display(), [0x6e, 0x91, 0x30c, 0x217, 1, 0, 1, 0, 2, 0, u32::MAX]);
    text(JOB_TEXT, 0xa156f4);
    let pale = tel35_colour(G_GUI_COLOUR_PALE_GREY);
    tel35_text_item([0x6e, 0xa5, 0x30c, 0xc8, 1, 0, 0xc, 4, pale, JOB_TEXT as u32, 1]);
    let rows = (0x122 / exe::font_get_height(3)) as u8 as u32;
    let area = tel26_add_area(current_human_display(), [0x82, 0xfa, 0x2f8, 0x217, 1, 0, rows, 0, 1, 0, u32::MAX]);
    let mut line = 0i32;
    text(JOB_TEXT, 0xa156e8);
    tel26_area_text([area, 0, line as u32, 1, 0, 1, 3, pale, JOB_TEXT as u32]);
    for (label, link, (option, value), skip) in HOLIDAY_SETTINGS {
        line += skip;
        text(JOB_TEXT, label);
        exe::text_set_string(t, JOB_TEXT as *const u8);
        exe::text_add_link(t, JOB_TEXT as *const u8, 3, link as usize);
        let current = holiday_option(option);
        let on = if value == -1 { current != 0 } else { current as i32 == value };
        let colour = if on { pale } else { tel35_colour(G_GUI_COLOUR_GREY) };
        tel35_text_draw_wrapped(t, area, 0x96, 0x2f8, &mut line, 3, colour, 2, 0);
        tel35_this1(0x4afa40, t, 0);
    }
    manager_no_yes(&weights);
    exe::drop_text(tx.as_mut_ptr() as *mut usize);
}

/// Rule: on the holiday screen No and Yes leave with the holiday answer pending, and a setting click switches it.
///
/// `holiday_screen_handle_action(item)`: id 1 (No): the pending command 8 with 1, -9. Id 2 (Yes): the pending command 8
/// with 2; options 2 and 3 written back with their own values (`set_holiday_option`); -9. Id 3 (a setting), its value
/// k (the low byte of the widget's data): 0 sets option 0 to 1, or to 0 when it was 1; 1 sets option 0 to 2, or 0 when
/// it was 2; 2 and 3 the same with option 4; 4, 5 and 6 switch options 1, 2 and 3 (0 becomes 1, anything else 0); any
/// other value changes nothing; -11. Otherwise the menu bar.
/// Original: `holiday_screen_handle_action` (0x6b3440, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn holiday_screen_handle_action(raw: u32) -> i32 {
    let item = raw as i16;
    match tel26_item_id(item) {
        1 => {
            write_u32(G_PENDING_COMMAND_CODE, 8);
            write_u32(G_PENDING_COMMAND_ARG, 1);
            -9
        }
        2 => {
            covered!("tel35_3 holiday confirmed");
            write_u32(G_PENDING_COMMAND_CODE, 8);
            write_u32(G_PENDING_COMMAND_ARG, 2);
            set_holiday_option(2, holiday_option(2) as i8 as i32);
            set_holiday_option(3, holiday_option(3) as i8 as i32);
            -9
        }
        3 => {
            covered!("tel35_3 holiday setting");
            let k = tel26_item_value(item) as u8 as i8 as i32;
            let pick = |option: u32, value: i32| set_holiday_option(option, if holiday_option(option) as i32 == value { 0 } else { value });
            let switch = |option: u32| set_holiday_option(option, (holiday_option(option) == 0) as i32);
            match k {
                0 => pick(0, 1),
                1 => pick(0, 2),
                2 => pick(4, 1),
                3 => pick(4, 2),
                4 => switch(1),
                5 => switch(2),
                6 => switch(3),
                _ => {}
            }
            -11
        }
        _ => manager_menu_bar(raw),
    }
}

/// The reputation and age test of the manager stats order: a reputation below `MANAGER_STATS_UNKNOWN_BELOW` counts as
/// -1 for a person under 35 (one of the last 16 staff: with fewer than 50 games).
unsafe fn manager_stats_order_reputation(row: usize, person: usize, id: i32, reputation: i16) -> i16 {
    if reputation >= MANAGER_STATS_UNKNOWN_BELOW {
        return reputation;
    }
    let limit = read_i32(G_STAFF_COUNT) - 0x10;
    let unknown = if id < limit { read_i8(person + staff::AGE) < 0x23 } else { read_i16(row + 0x8e) < 0x32 };
    if unknown { -1 } else { reputation }
}

/// The days since a date of a manager record (`Date::days_since`, the date at `at` by value, from `this`).
unsafe fn manager_stats_days(this: usize, at: usize) -> i32 {
    exe::date_days_since(this, read_u32(at), read_u32(at + 4))
}

/// The win share of a manager record: 100 × wins (+0x90) / games (+0x8e) times the games (at most 100); -1 without
/// games.
unsafe fn manager_stats_win_score(row: usize) -> i32 {
    let games = read_i16(row + 0x8e);
    if games == 0 {
        return -1;
    }
    let share = (read_i16(row + 0x90) as i32 * 100) / games as i32;
    let weight = if games >= 0x64 { 0x64 } else { games as i32 };
    weight.wrapping_mul(share)
}

/// Rule: the Manager Stats list is ordered by the chosen statistic, the best first, and then by name.
///
/// `compare_manager_stats_rows(a, b)` (qsort, 210-byte manager records; +0x86 the staff id): a null a: 1; a null b: -1;
/// a person without his stats block (+0x69 of the staff record): 1 for a, -1 for b. By the statistic
/// (`MANAGER_STATS_SORT_STAT`), b minus a: 1 the reputation (+0xc of the stats block with no nation filter, +0xa with
/// one; `manager_stats_order_reputation`); 2 the money (+0xb6, or with a nation filter the two job amounts +0xba/+0xbe
/// whose nations +0xc2/+0xc6 are the filter); 3 the days in the club job (`staff::CLUB_JOB` and the date +0x3e,
/// `Date::days_since` from today); 4 the bytes +0xb5 and +0xb4; 5 the days since +0x6a's date (when +0x5e is set);
/// 6 the same with +0x7e/+0x72, a minus b; 7, 8, 9, 11, 12 the words +0x8a, +0x8c, +0x9e, +0xa0, +0xa2; 10 the win score
/// (`manager_stats_win_score`); 13, 14 the dwords +0x18, +0x3d; 15, 16 the 64-bit amounts +0xa4, +0xac (1 or -1); any
/// other statistic returns the address of a's staff record (a defect kept). A tie: the names (style 3, 200 bytes)
/// compared (`utils_string_compare`, 1).
/// Original: `compare_manager_stats_rows` (0x6b36f0, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_manager_stats_rows(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let staff_table = read_usize(G_STAFF_TABLE);
    let id_a = read_i32(a + 0x86);
    let pa = staff_table.wrapping_add((id_a as u32).wrapping_mul(staff::SIZE as u32) as usize);
    let sa = read_usize(pa + staff::NON_PLAYER);
    if sa == 0 {
        return 1;
    }
    let id_b = read_i32(b + 0x86);
    let pb = staff_table.wrapping_add((id_b as u32).wrapping_mul(staff::SIZE as u32) as usize);
    let sb = read_usize(pb + staff::NON_PLAYER);
    if sb == 0 {
        return -1;
    }
    let nation = read_i32(MANAGER_STATS_SORT_NATION);
    let wide = |at: usize| ((read_u32(b + at + 4) as i32 as i64) << 32 | read_u32(b + at) as i64)
        .cmp(&((read_u32(a + at + 4) as i32 as i64) << 32 | read_u32(a + at) as i64));
    let d = match read_u8(MANAGER_STATS_SORT_STAT) {
        1 => {
            let at = if nation == -1 { 0xc } else { 0xa };
            let ra = manager_stats_order_reputation(a, pa, id_a, read_i16(sa + at));
            let rb = manager_stats_order_reputation(b, pb, id_b, read_i16(sb + at));
            rb as i32 - ra as i32
        }
        2 => {
            if nation == -1 {
                read_i32(b + 0xb6).wrapping_sub(read_i32(a + 0xb6))
            } else {
                let mut d = 0i32;
                if read_i32(b + 0xc2) == nation {
                    d = read_i32(b + 0xba);
                }
                if read_i32(b + 0xc6) == nation {
                    d = d.wrapping_add(read_i32(b + 0xbe));
                }
                if read_i32(a + 0xc2) == nation {
                    d = d.wrapping_sub(read_i32(a + 0xba));
                }
                if read_i32(a + 0xc6) == nation {
                    d = d.wrapping_sub(read_i32(a + 0xbe));
                }
                d
            }
        }
        3 => {
            let mut d = 0i32;
            if read_usize(pb + staff::CLUB_JOB) != 0 {
                d = manager_stats_days(G_CURRENT_DATE, pb + 0x3e);
            }
            if read_usize(pa + staff::CLUB_JOB) != 0 {
                d = d.wrapping_sub(manager_stats_days(G_CURRENT_DATE, pa + 0x3e));
            }
            d
        }
        4 => (read_u8(b + 0xb5) as i32 - read_u8(a + 0xb5) as i32 - read_u8(a + 0xb4) as i32 + read_u8(b + 0xb4) as i32),
        5 => {
            let mut d = 0i32;
            if read_i32(b + 0x5e) > -1 {
                d = manager_stats_days(b + 0x6a, b + 0x62);
            }
            if read_i32(a + 0x5e) > -1 {
                d = d.wrapping_sub(manager_stats_days(a + 0x6a, a + 0x62));
            }
            d
        }
        6 => {
            let mut d = 0i32;
            if read_i32(b + 0x72) > -1 {
                d = manager_stats_days(b + 0x7e, b + 0x76).wrapping_neg();
            }
            if read_i32(a + 0x72) > -1 {
                d = d.wrapping_add(manager_stats_days(a + 0x7e, a + 0x76));
            }
            d
        }
        7 => read_i16(b + 0x8a) as i32 - read_i16(a + 0x8a) as i32,
        8 => read_i16(b + 0x8c) as i32 - read_i16(a + 0x8c) as i32,
        9 => read_i16(b + 0x9e) as i32 - read_i16(a + 0x9e) as i32,
        0xa => manager_stats_win_score(b).wrapping_sub(manager_stats_win_score(a)),
        0xb => read_i16(b + 0xa0) as i32 - read_i16(a + 0xa0) as i32,
        0xc => read_i16(b + 0xa2) as i32 - read_i16(a + 0xa2) as i32,
        0xd => read_i32(b + 0x18).wrapping_sub(read_i32(a + 0x18)),
        0xe => read_i32(b + 0x3d).wrapping_sub(read_i32(a + 0x3d)),
        0xf => wide(0xa4) as i32,
        0x10 => wide(0xac) as i32,
        _ => pa as i32,
    };
    if d != 0 {
        return d;
    }
    covered!("tel35_3 manager stats name order");
    let (mut name_a, mut name_b) = ([0u8; 200], [0u8; 200]);
    exe::database_format_staff_name(pa, name_a.as_mut_ptr() as usize, 0xc8, 3);
    let pb = read_usize(G_STAFF_TABLE).wrapping_add((read_u32(b + 0x86)).wrapping_mul(staff::SIZE as u32) as usize);
    exe::database_format_staff_name(pb, name_b.as_mut_ptr() as usize, 0xc8, 3);
    exe::utils_string_compare(name_a.as_ptr(), name_b.as_ptr(), 1, 0)
}

/// Rule: the job information list is ordered by the word +8 of its 10-byte rows.
///
/// `compare_job_information_rows(a, b)` (qsort): a null a: 1; a null b: -1; otherwise the word +8 of a minus that of
/// b, as a signed word.
/// Original: `compare_job_information_rows` (0x6b3be0, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_job_information_rows(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    read_u16(a + 8).wrapping_sub(read_u16(b + 8)) as i16 as i32
}

/// Comparer for `open_retire_screen`.
pub(crate) unsafe extern "C" fn compare_open_retire_screen() {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(%%I0%%)()
}

/// Comparer for `retire_screen_build`: the screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_retire_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    compare_by_display_list_with("Retire screen (retire_screen_build)", &[], || o(), || retire_screen_build());
}

/// Comparer for `retire_screen_handle_action`.
pub(crate) unsafe extern "C" fn compare_retire_screen_handle_action(item: u32) -> i32 {
    // simplification: the retirement and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%)(item)
}

/// Comparer for `open_holiday_screen`.
pub(crate) unsafe extern "C" fn compare_open_holiday_screen() {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(%%I3%%)()
}

/// Comparer for `holiday_screen_build`: the screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_holiday_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I4%%);
    compare_by_display_list_with("Holiday screen (holiday_screen_build)", &[], || o(), || holiday_screen_build());
}

/// Comparer for `holiday_screen_handle_action`.
pub(crate) unsafe extern "C" fn compare_holiday_screen_handle_action(item: u32) -> i32 {
    // simplification: the holiday settings and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I5%%)(item)
}

compare_order!(compare_compare_manager_stats_rows, compare_manager_stats_rows, %%I6%%);
compare_order!(compare_compare_job_information_rows, compare_job_information_rows, %%I7%%);
