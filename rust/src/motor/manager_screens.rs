use super::*;

/// Opens the Manager Stats screen for the current human manager's nation and sets its nine parameters.
///
/// `ScreenManager::open_screen` with the builder 0x6ae2a0, the handler 0x6b05e0 and the closer 0x6b09d0; when it
/// opens: parameter 0 = the nation id of the slot's manager (the nation of his club job, else of his national job,
/// else his own nation when it is selected (`league_selected()` bit 2), else -1; -1 without a manager), 1 and 2 = 0
/// (no list), 3 and 4 = 1 (page 1 of 1), 5 and 6 = 0, 7 = 1 (scroll to the human) and 8 = 1 (the first statistic).
/// Original: `open_manager_stats_screen` (0x6ae1a0, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_manager_stats_screen() {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x6ae2a0usize, 0x6b05e0usize, 0, 0x6b09d0usize, 0) == 0 {
        return;
    }
    covered!("tel26_3 open manager stats");
    let m = tel26_slot_manager();
    let nation = if m.is_null() {
        usize::MAX
    } else if !m.club_job().is_null() {
        tel26_nation_id(m.club_job().nation())
    } else if !m.national_job().is_null() {
        tel26_nation_id(m.national_job().nation())
    } else if m.nation().league_selected() & 2 != 0 {
        tel26_nation_id(m.nation())
    } else {
        usize::MAX
    };
    for (k, v) in [(0u32, nation), (1, 0), (2, 0), (3, 1), (4, 1), (5, 0), (6, 0), (7, 1), (8, 1)] {
        set_screen_param(k, v);
    }
}

/// The Manager Stats screen's answer to widget `item`.
///
/// With a list (parameter 1) and a widget, parameter 5 keeps the scroll row of the list area (parameter 6,
/// `ScreenManager::get_area_scroll_row`). Then by the widget's id (0 and -1: the default command): 1 = the nation
/// filter (parameter 0 = the widget's value) and 2 = the statistic (parameter 8 = the value), both freeing the list
/// (parameters 1 and 2 cleared) and going back to page 1 of 1 with the scroll to the human (3, 4, 7 = 1); 3 and 4 =
/// back 10 pages or 1 (not below 1), 5 and 6 = forward 1 or 10 (not past the page count, parameter 4); each of these
/// returns -11 (rebuild). Any other id goes to the menu bar (`menubar_handle_command`, then
/// `menubar_handle_object_link` when it gives 0).
/// Original: `manager_stats_screen_handle_action` (0x6b05e0, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn manager_stats_screen_handle_action(raw: u32) -> i32 {
    let item = raw as i16;
    let list = get_screen_param(1);
    let page = get_screen_param(3) as i16 as i32;
    let pages = get_screen_param(4) as i16 as i32;
    if list != 0 && item != -1 {
        let row = exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(6));
        set_screen_param(5, row);
    }
    let id = tel26_item_id(item);
    match id {
        1 | 2 => {
            covered!("tel26_3 manager stats filter");
            set_screen_param(if id == 1 { 0 } else { 8 }, tel26_item_value(item) as usize);
            tel26_free_list();
            for k in [3u32, 4, 7] {
                set_screen_param(k, 1);
            }
        }
        3 => set_screen_param(3, (page - 10).max(1) as usize),
        4 => set_screen_param(3, (page - 1).max(1) as usize),
        5 => set_screen_param(3, (page + 1).min(pages) as usize),
        6 => set_screen_param(3, (page + 10).min(pages) as usize),
        _ => {
            let r = exe::menubar_handle_command(raw);
            return if r != 0 { r } else { exe::menubar_handle_object_link(raw) };
        }
    }
    -11
}

/// Closing the Manager Stats screen frees its list (parameter 1) and clears parameters 1 and 2.
/// Original: `manager_stats_screen_close` (0x6b09d0, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn manager_stats_screen_close() {
    tel26_free_list();
}

/// Opens the Job Information screen and sets its parameters.
///
/// `ScreenManager::open_screen` with the builder 0x6b0b10, the handler 0x6b1790 and the closer 0x6b1980; when it
/// opens: parameter 0 = the nation of the slot manager's club job, else his own nation when it is selected (bit 2),
/// else the nation of his national job when selected, else -1 (also without a manager); 1, 2 = 0; 3, 4 = 1; 5, 6 =
/// 0; 7 = 1.
/// Original: `open_job_information_screen` (0x6b0a10, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_job_information_screen() {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x6b0b10usize, 0x6b1790usize, 0, 0x6b1980usize, 0) == 0 {
        return;
    }
    covered!("tel26_3 open job information");
    let m = tel26_slot_manager();
    let nation = if m.is_null() {
        usize::MAX
    } else if !m.club_job().is_null() {
        tel26_nation_id(m.club_job().nation())
    } else if m.nation().league_selected() & 2 != 0 {
        tel26_nation_id(m.nation())
    } else if !m.national_job().is_null() && m.national_job().nation().league_selected() & 2 != 0 {
        tel26_nation_id(m.national_job().nation())
    } else {
        usize::MAX
    };
    for (k, v) in [(0u32, nation), (1, 0), (2, 0), (3, 1), (4, 1), (5, 0), (6, 0), (7, 1)] {
        set_screen_param(k, v);
    }
}

/// Builds the Job Information screen: the clubs of a nation (or the national teams) with their manager, nation,
/// division and job security.
///
/// Parameters: 0 the nation id (-1 = international), 1 the list, 2 its count, 3 the page, 7 scroll to the human.
/// A random background, the title "Job Information" (colours [0xaebe04]/[0xae3184]), the menu bar (4, 0), the
/// subtitle (the nation's name, or "International"), the "View" button with its popup area (columns 0x130) holding
/// "International" (value -1), a separator and the selected nations (`league_selected()` & 3) in the order of
/// `key_nation_sorted_order(1)`, split in two parts with a header each by `NATION_MENU_SPLIT` (the chosen nation
/// marked 0x2002). Without a list, the list is built: every club that can be managed (`club_can_be_managed`), whose
/// nation has a ruling body (+0x71), either a club of the chosen nation in an active competition ([0xadadfc][comp
/// id]) or, for the international view, a national team whose nation's rating (+0x88) reaches 100 (1000 for the
/// senior team, `club_is_national_team`), and not a B team kept out (`club_get_linked_b_team` with its flag, which
/// once set stays set for the next clubs: a defect of the original); entries of 10 bytes (club, manager +0xcf, the
/// board's security, word +6 of the manager record 0x49 × club id of [[0xb67a44]], 0 without a manager; 0xfd2 and
/// above are left out), the list grown by 50 (`crt_realloc`; failing: the manager_screens.cpp 1049 box and the old
/// list lost), sorted by 0x6b3be0, kept in parameters 1 and 2. An empty list shows "There is no job information
/// available.". Otherwise 0x15d / font height 3 rows per page; the human's club (`human_manager_is_human_club`)
/// found; over 100 clubs, pages (`gui_add_page_buttons`), the page of the human's club the first time (parameter 7
/// cleared); the list area (5 columns weighted 10/6/5/2/5) and per row: the manager (named, coloured [0xae3184]
/// unless one of the 16 human slots) or "No Manager", the club (U21 suffix for the junior team), the nation and the
/// division (or the ruling body for a national team), and the security: Available, Very Insecure (≤ 3000), Insecure
/// (≤ 3500), Slightly Insecure (≤ 4275), Secure (< 8500), Very Secure. Then the scroll (to the human's row the first
/// time, else parameter 5), the area in parameter 6, and the Back/Next buttons.
/// Original: `job_information_screen_build` (0x6b0b10, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn job_information_screen_build() {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    covered!("tel26_3 job information");
    let weights: [u8; 5] = [10, 6, 5, 2, 5];
    let mut b_team_flag: u32 = 0;
    let mut group: u8 = 0xff;
    let nation = get_screen_param(0) as u32;
    let mut list = get_screen_param(1);
    let mut count = get_screen_param(2) as i32;
    let mut page = get_screen_param(3) as i32;
    let mut scroll_to_human = get_screen_param(7);
    exe::background_set_random_picture();
    text(JOB_TEXT, 0xa15424);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, job_colour(JOB_COLOUR_BE04), job_colour(JOB_COLOUR_3184), JOB_TEXT);
    exe::menubar_build(4, 0);
    if nation == u32::MAX {
        text(JOB_TEXT, 0xa153e8, 0xad9c64usize);
    } else {
        tel26_copy_text(JOB_TEXT as *mut u8, (read_usize(G_NATION_TABLE) + nation as usize * 0x122 + 0x38) as *const u8);
    }
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 0);
    text(JOB_TEXT, 0x988004);
    let d = current_human_display();
    let view = tel26_add_button([0x6e, 0x91, 0xeb, 0xa5, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0, 0]);
    let menu = tel26_add_area(d, [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), view]);
    text(JOB_TEXT, 0xa153e8, 0xad9c64usize);
    job_nation_line(d, nation == u32::MAX, u32::MAX, menu);
    tel26_add_item(d, [1, 0, 0, 0, 0, 0, 0, 0x1000010, job_colour(JOB_COLOUR_31AA), job_colour(JOB_COLOUR_BDEC), 1, 1,
        job_colour(JOB_COLOUR_BDEA), 0, 0, 0, 0, menu]);
    let order = exe::key_nation_sorted_order(1);
    let mut split = [0u8; 16];
    exe::new_nation_menu_split((split.as_mut_ptr()) as *mut u8 as usize, 0, order, 1);
    // the original's part area starts uninitialised; it is always set before use when the split has 1 or 2 parts
    let mut part_area: u32 = 0;
    for i in 0..0x23usize {
        let n = Nation(read_usize((G_LEAGUE_TABLE as isize + read_u8(order + i) as i8 as isize * 0x48) as usize));
        if n.is_null() || n.league_selected() & 3 == 0 {
            continue;
        }
        let mut header = false;
        if (exe::nation_menu_split_part_count((split.as_mut_ptr()) as *mut u8 as usize) as i8) > 1 && exe::nation_menu_split_in_second_part((split.as_mut_ptr()) as *mut u8 as usize, n) != group {
            header = true;
            group = exe::nation_menu_split_in_second_part((split.as_mut_ptr()) as *mut u8 as usize, n);
            exe::nation_menu_split_format_header((split.as_mut_ptr()) as *mut u8 as usize, JOB_TEXT, 0x7d0, group as u32);
            let item = tel26_add_item(d, [2, 0, 0, 0, 0, 0, 0, 0x10, job_colour(JOB_COLOUR_31AA), job_colour(JOB_COLOUR_BDEC), 1, 1,
                job_colour(JOB_COLOUR_BDEA), JOB_TEXT as u32, 0, 0, 0, menu]);
            part_area = tel26_add_area(d, [0, 0, 0, 0, 0, 0, 0, 0, 0x30, (item & 0xffff_0000) | job_colour(JOB_COLOUR_31AA), item]);
        }
        if !header && (exe::nation_menu_split_part_count((split.as_mut_ptr()) as *mut u8 as usize) as i8) == 1 {
            part_area = menu;
        }
        format(JOB_TEXT, 0x98ed14, n.addr() + nation::SHORT_NAME);
        let id = read_u32(n.addr());
        job_nation_line(d, id == nation, id, part_area);
    }
    if list == 0 {
        let clubs = read_i32(G_CLUB_COUNT);
        let mut offset = count.wrapping_mul(10);
        for j in 0..clubs.max(0) as usize {
            let club = club_at(j as isize).addr();
            if exe::club_can_be_managed(club) == 0 {
                continue;
            }
            let club_nation = read_usize(club + club::NATION);
            if club_nation == 0 || read_usize(club_nation + 0x71) == 0 {
                continue;
            }
            if exe::database_is_national_team(club) == 0 {
                let comp = read_usize(club + 0x57);
                if nation == u32::MAX || comp == 0
                    || read_u32((read_usize(0xadadfc) as isize + read_i32(comp) as isize * 4) as usize) == 0
                    || read_u32(read_usize(club + club::NATION)) != nation {
                    continue;
                }
            }
            if exe::database_is_national_team(club) != 0 {
                if nation as i32 > -1 {
                    continue;
                }
                let minimum = if exe::club_is_national_team(club) != 0 { 1000 } else { 100 };
                if read_i32(read_usize(club + club::NATION) + 0x88) < minimum {
                    continue;
                }
            }
            if exe::club_get_linked_b_team(club, &mut b_team_flag, 1) != 0 && b_team_flag == 0 {
                continue;
            }
            let manager = read_usize(club + 0xcf);
            let security = if manager == 0 || club == 0 {
                0
            } else {
                let s = read_i16((read_usize(read_usize(0xb67a44)) as isize + read_i32(club) as isize * 0x49 + 6) as usize);
                if s >= 0xfd2 {
                    continue;
                }
                s
            };
            if count % 50 == 0 {
                list = exe::crt_realloc(list, (offset + 500) as u32);
                if list == 0 {
                    tel26_error_box(MANAGER_SCREENS_CPP, 0x419);
                    return;
                }
            }
            write_usize(list + offset as usize + 4, manager);
            write_usize(list + offset as usize, club);
            count += 1;
            write_i16(list + offset as usize + 8, security);
            offset += 10;
        }
        if count != 0 {
            exe::crt_qsort(list, count as u32, 10, 0x6b3be0usize);
        }
        set_screen_param(1, list);
        set_screen_param(2, count as usize);
        if list == 0 {
            let rows = (0x15d / exe::font_get_height(3)) as u8;
            text(JOB_TEXT, 0xa15334);
            tel26_text_area([0x6e, 0xaa, 0x30c, 0x217, 2, 0, rows as u32, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0]);
            tel26_back_next();
            return;
        }
    }
    let rows = (0x15d / exe::font_get_height(3)) as u8;
    let mut human_row: i32 = -1;
    let mut k = 0;
    while k < count && human_row == -1 {
        if exe::human_manager_is_human_club(read_usize(list + k as usize * 10), 0, 1, 0, 0) != 0 {
            human_row = k;
        }
        k += 1;
    }
    let per = rows as i32;
    if count > 100 {
        if scroll_to_human != 0 {
            page = human_row.max(0) / per + 1;
            set_screen_param(3, page as i16 as i32 as usize);
            scroll_to_human = 0;
            set_screen_param(7, 0);
        }
        let pages = count / per + (count % per != 0) as i32;
        set_screen_param(4, pages as i16 as i32 as usize);
        exe::gui_add_page_buttons(0x30c, 0x91, page as u32, pages as u32, 0, 2, 3, 0);
    }
    // the original keeps the rows in the low byte of the nation parameter's local and pushes the whole dword
    let rows_dword = (nation & !0xff) | rows as u32;
    let area = tel26_add_area(d, [0x6e, 0xaa, 0x30c, 0x217, 5, weights.as_ptr() as u32, rows_dword, 0, 2, 0, u32::MAX]);
    let start = (page as i16 as i32 - 1).wrapping_mul(per);
    let mut row = 0;
    while (row < per || count < 100) && start + row < count {
        let entry = list + (start + row) as usize * 10;
        let manager = read_usize(entry + 4);
        let r = row as u32;
        if manager != 0 {
            let mut name = [0u8; 200];
            exe::database_format_staff_name(manager, name.as_mut_ptr() as usize, 200, 4);
            format(JOB_TEXT, 0x987ff0, name.as_ptr());
            let id = read_u32(manager);
            let colour = if (id as i32) < read_i32(G_STAFF_COUNT) - 0x10 { (id & 0xffff_0000) | job_colour(JOB_COLOUR_3184) } else { job_colour(JOB_COLOUR_BDEC) };
            tel26_area_button([area, 0, r, 1, 0, 1, 3, colour, JOB_TEXT as u32, 0x7d7, manager as u32]);
        } else {
            text(JOB_TEXT, 0xa153d8);
            tel26_area_text([area, 0, r, 1, 0, 1, 3, job_colour(JOB_COLOUR_317E), JOB_TEXT as u32]);
        }
        let club = read_usize(entry);
        if exe::club_is_national_team(club) != 0 {
            let c = read_u8(club + 0x52) as i8 as i32;
            text(JOB_TEXT, 0xa153c0, c, c, club + 0x38);
        } else {
            tel26_copy_text(JOB_TEXT as *mut u8, (club + 0x38) as *const u8);
        }
        tel26_area_button([area, 1, r, 1, 0, 1, 2, job_colour(JOB_COLOUR_BDE8), JOB_TEXT as u32, 0x7d5, club as u32]);
        if exe::database_is_national_team(club) == 0 {
            let team = exe::database_get_national_team(read_usize(club + club::NATION)) as u32;
            tel26_area_button([area, 2, r, 1, 0, 1, 2, job_colour(JOB_COLOUR_BDEC), (read_usize(club + club::NATION) + 0x38) as u32, 0x7d5, team]);
            let comp = read_usize(club + 0x57);
            tel26_area_button([area, 3, r, 1, 0, 1, 2, job_colour(JOB_COLOUR_BDD4), (comp + 0x53) as u32, 0x7d0, comp as u32]);
        } else {
            tel26_area_text([area, 2, r, 1, 0, 1, 2, job_colour(JOB_COLOUR_BDEC), (read_usize(read_usize(club + club::NATION) + 0x71) + 4) as u32]);
        }
        if manager == 0 {
            text(JOB_TEXT, 0xa153b4);
        } else {
            let s = read_i16(entry + 8);
            if s <= 0xbb8 {
                text(JOB_TEXT, 0xa153a4);
            } else if s <= 0xdac {
                text(JOB_TEXT, 0xa1537c, 0xad9c64usize);
            } else if s <= 0x10b3 {
                text(JOB_TEXT, 0xa15368);
            } else if s < 0x2134 {
                text(JOB_TEXT, 0x990444);
            } else {
                text(JOB_TEXT, 0xa1535c);
            }
        }
        tel26_area_text([area, 4, r, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);
        row += 1;
    }
    let area16 = area as i16;
    if scroll_to_human != 0 {
        let to = (human_row.max(0) - (per >> 1)).max(0);
        if area16 != -1 {
            exe::area_set_scroll_row(display_area_at(d, area16), to as usize);
        }
        set_screen_param(6, area16 as i32 as usize);
        set_screen_param(7, 0);
    } else {
        let to = get_screen_param(5);
        if area16 != -1 {
            exe::area_set_scroll_row(display_area_at(d, area16), to);
        }
        set_screen_param(6, area16 as i32 as usize);
    }
    tel26_back_next();
}

/// The Job Information screen's answer to widget `item`.
///
/// With a widget, parameter 5 keeps the list area's scroll row (parameter 6). Id 1 (the nation menu): parameter 0 =
/// the widget's value, the list freed (1, 2 cleared) and page 1 (3); id 2 = back a page (not below 1), id 3 =
/// forward a page (not past parameter 4); these return -11. Other ids go to the menu bar.
/// Original: `job_information_screen_handle_action` (0x6b1790, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn job_information_screen_handle_action(raw: u32) -> i32 {
    let item = raw as i16;
    let page = get_screen_param(3) as i16 as i32;
    let pages = get_screen_param(4) as i16 as i32;
    if item != -1 {
        let row = exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(6));
        set_screen_param(5, row);
    }
    match tel26_item_id(item) {
        1 => {
            covered!("tel26_3 job information nation");
            set_screen_param(0, tel26_item_value(item) as usize);
            if get_screen_param(1) != 0 {
                tel26_free_list();
                set_screen_param(3, 1);
            }
        }
        2 => set_screen_param(3, (page - 1).max(1) as usize),
        3 => set_screen_param(3, (page + 1).min(pages) as usize),
        _ => {
            let r = exe::menubar_handle_command(raw);
            return if r != 0 { r } else { exe::menubar_handle_object_link(raw) };
        }
    }
    -11
}

/// Closing the Job Information screen frees its list (1 and 2 cleared) and goes back to page 1.
/// Original: `job_information_screen_close` (0x6b1980, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn job_information_screen_close() {
    if get_screen_param(1) != 0 {
        tel26_free_list();
        set_screen_param(3, 1);
    }
}

/// Comparer for `open_manager_stats_screen`.
pub(crate) unsafe extern "C" fn compare_open_manager_stats_screen() {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(IDX_OPEN_MANAGER_STATS_SCREEN)()
}

/// Comparer for `manager_stats_screen_handle_action`.
pub(crate) unsafe extern "C" fn compare_manager_stats_screen_handle_action(item: u32) -> i32 {
    // simplification: freeing the list and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_MANAGER_STATS_SCREEN_HANDLE_ACTION)(item)
}

/// Comparer for `manager_stats_screen_close`.
pub(crate) unsafe extern "C" fn compare_manager_stats_screen_close() {
    // simplification: the free does not run twice
    original_fn::<unsafe extern "C" fn()>(IDX_MANAGER_STATS_SCREEN_CLOSE)()
}

/// Comparer for `open_job_information_screen`.
pub(crate) unsafe extern "C" fn compare_open_job_information_screen() {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(IDX_OPEN_JOB_INFORMATION_SCREEN)()
}

/// Comparer for `job_information_screen_build`: the Job Information screen side by side over the display list,
/// with the list (parameter 1); without a list (it is allocated here) only the original.
pub(crate) unsafe extern "C" fn compare_job_information_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_JOB_INFORMATION_SCREEN_BUILD);
    let list = get_screen_param(1);
    if list == 0 {
        // simplification: the list is allocated and sorted here; proven in Rust mode
        return o();
    }
    let n = get_screen_param(2) as i32;
    let extra = if n > 0 { vec![(list, n as usize * 10)] } else { Vec::new() };
    compare_by_display_list_with("Job Information screen (job_information_screen_build)", &extra, || o(), || job_information_screen_build());
}

/// Comparer for `job_information_screen_handle_action`.
pub(crate) unsafe extern "C" fn compare_job_information_screen_handle_action(item: u32) -> i32 {
    // simplification: freeing the list and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_JOB_INFORMATION_SCREEN_HANDLE_ACTION)(item)
}

/// Comparer for `job_information_screen_close`.
pub(crate) unsafe extern "C" fn compare_job_information_screen_close() {
    // simplification: the free does not run twice
    original_fn::<unsafe extern "C" fn()>(IDX_JOB_INFORMATION_SCREEN_CLOSE)()
}

/// Opens the confirm screen asking to apply for `club`'s job (builder 0x6b1a80, handler 0x6b1c30); a null club
/// shows the manager_screens.cpp 1319 box.
/// Original: `open_apply_for_job_screen` (0x6b19d0, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_apply_for_job_screen(club: usize) {
    covered!("tel26_4 open apply for job");
    manager_confirm_open(club, 0x6b1a80, 0x6b1c30, 1, 0x527);
}

/// Builds the "Apply for the <club> job ?" confirm screen (`manager_confirm_build` on parameter 0).
/// Original: `apply_for_job_screen_build` (0x6b1a80, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn apply_for_job_screen_build() {
    covered!("tel26_4 apply for job");
    manager_confirm_build(0, 0xa15434);
}

/// "No" (id 1) closes with -9; "Yes" (id 2) applies the slot's human manager for the club (parameter 0,
/// `MANAGER_MANAGER::apply_for_job` on [0xb67a44]) and closes with -9; other ids go to the menu bar.
/// Original: `apply_for_job_screen_handle_action` (0x6b1c30, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn apply_for_job_screen_handle_action(raw: u32) -> i32 {
    match tel26_item_id(raw as i16) {
        1 => -9,
        2 => {
            let manager = tel26_slot_manager();
            exe::manager_manager_apply_for_job(read_usize(0xb67a44), get_screen_param(0), manager);
            -9
        }
        _ => manager_menu_bar(raw),
    }
}

/// Opens the confirm screen asking to resign from `club` (builder 0x6b1dc0, handler 0x6b1f70); a null club shows
/// the manager_screens.cpp 1428 box.
/// Original: `open_resign_job_screen` (0x6b1d10, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_resign_job_screen(club: usize) {
    covered!("tel26_4 open resign");
    manager_confirm_open(club, 0x6b1dc0, 0x6b1f70, 1, 0x594);
}

/// Builds the "Resign as manager of <club> ?" confirm screen (`manager_confirm_build` on parameter 0).
/// Original: `resign_job_screen_build` (0x6b1dc0, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn resign_job_screen_build() {
    covered!("tel26_4 resign");
    manager_confirm_build(0, 0xa15458);
}

/// "No" (id 1) closes with -9; "Yes" (id 2) removes the manager of the club (parameter 0,
/// `MANAGER_MANAGER::remove_club_manager` with departure 2), refreshes the match day's human interest flags
/// ([0xb67a50], when there is one), opens the vacancy with the slot's manager as the previous one (`add_job_vacancy`)
/// and closes with -9; other ids go to the menu bar.
/// Original: `resign_job_screen_handle_action` (0x6b1f70, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn resign_job_screen_handle_action(raw: u32) -> i32 {
    let club = get_screen_param(0);
    match tel26_item_id(raw as i16) {
        1 => -9,
        2 => {
            exe::manager_manager_remove_club_manager(read_usize(0xb67a44), club, 2, 0, 0);
            let day = read_usize(0xb67a50);
            if day != 0 {
                exe::match_day_refresh_human_interest_flags(day);
            }
            exe::manager_manager_add_job_vacancy(read_usize(0xb67a44), club, tel26_slot_manager());
            -9
        }
        _ => manager_menu_bar(raw),
    }
}

/// Opens the Board Confidence screen of `club` (builder 0x6b2130, handler 0x6b2370); a null club shows the
/// manager_screens.cpp 1548 box.
/// Original: `open_board_confidence_screen` (0x6b2080, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_board_confidence_screen(club: usize) {
    covered!("tel26_4 open board confidence");
    manager_confirm_open(club, 0x6b2130, 0x6b2370, 0, 0x60c);
}

/// Builds the Board (or F.A., for a national team) Confidence screen of the club in parameter 0.
///
/// The club's background picture, the menu bar (4, 0), the title (the club's name in its kit colours), the
/// subtitle "F.A. Confidence" or "Board Confidence"; for a club, the "Board Request" (id 1) and "FA Request" (id
/// 0x145) buttons. The days in the job = 365 × (the year [0xae2c92] − the manager's start year +0x40, both
/// unsigned) − his start day (+0x3e) + the day [0xae2c90]; `MANAGER_MANAGER::build_board_confidence_text` (club,
/// days, 2000 bytes) and `build_board_squad_opinion_text` (into the same frame at +0x3f0) give the text: both
/// ("%s" twice, 0x991948) or only the first ("%s") in a text area; then the Back/Next buttons.
/// Original: `board_confidence_screen_build` (0x6b2130, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn board_confidence_screen_build() {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    covered!("tel26_4 board confidence");
    // the original's frame: the confidence text at +8 (2000 bytes) and the squad opinion at +0x3f0, overlapping
    let mut frame = [0u8; 0x7d8];
    let club = get_screen_param(0);
    let manager = tel26_slot_manager().addr();
    exe::background_set_club_picture(club, 0, 0);
    exe::menubar_build(4, 0);
    let (mut fore, mut back) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut fore, &mut back);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, club + 4);
    text(JOB_TEXT, if exe::database_is_national_team(club) != 0 { 0xa154b0 } else { 0xa1549c });
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    if exe::database_is_national_team(club) == 0 {
        text(JOB_TEXT, 0xa1548c);
        tel26_add_button([0x6e, 0x82, 0xeb, 0x96, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 1, 0]);
        text(JOB_TEXT, 0xa15480);
        tel26_add_button([0xf0, 0x82, 0x16d, 0x96, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0x145, 0]);
    }
    let years = (read_u16(0xae2c92) as i32).wrapping_sub(read_u16(manager + 0x40) as i32);
    let days = years.wrapping_mul(365).wrapping_sub(read_i16(manager + 0x3e) as i32).wrapping_add(read_i16(0xae2c90) as i32);
    let first = frame.as_mut_ptr().wrapping_add(8);
    let second = frame.as_mut_ptr().wrapping_add(0x3f0);
    exe::manager_manager_build_board_confidence_text(read_usize(0xb67a44), club, days, first, 0x7d0);
    if exe::manager_manager_build_board_squad_opinion_text(read_usize(0xb67a44), second, club, 0) != 0 {
        format(JOB_TEXT, 0x991948, first, second);
    } else {
        format(JOB_TEXT, 0x991944, first);
    }
    tel26_text_area([0x6e, 0x9b, 0x30c, 0x21c, 2, 0, 0x10, 4, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0]);
    tel26_back_next();
}

/// "Board Request" (id 1) opens the board request screen of the club (parameter 0, `board_request_screen_open`
/// 0x425390) and "FA Request" (0x145) the FA meeting (`fa_meeting_open_screen` 0x58acd0), both returning -4;
/// other ids go to the menu bar.
/// Original: `board_confidence_screen_handle_action` (0x6b2370, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn board_confidence_screen_handle_action(raw: u32) -> i32 {
    let target = match tel26_item_id(raw as i16) {
        1 => 0x425390usize,
        0x145 => 0x58acd0,
        _ => return manager_menu_bar(raw),
    };
    let open: unsafe extern "C" fn(usize) = core::mem::transmute(target);
    open(get_screen_param(0));
    -4
}

/// Opens the confirm screen of a job offer news item: the item `news_id` of `manager`'s inbox is fetched
/// (`News::find_item_by_id` on [0xb7431c] into a local `NewsItem`); a null manager or a missing item shows the
/// manager_screens.cpp 1678 / 1685 box. When the screen opens (builder 0x6b25e0, handler 0x6b2790, 1): parameter
/// 0 = the news id and 1 = the offering club (the club table entry of the item's slot 0).
/// Original: `open_job_offer_screen` (0x6b2450, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_job_offer_screen(manager: usize, news_id: u32) {
    covered!("tel26_4 open job offer");
    let mut item = [0u8; 0x100];
    exe::new_newsitem(item.as_mut_ptr() as usize, 0, 0);
    if manager == 0 {
        tel26_error_box(MANAGER_SCREENS_CPP, 0x68e);
        return;
    }
    if exe::news_find_item_by_id(read_usize(0xb7431c), manager, news_id, (item.as_mut_ptr()) as *mut u8 as usize) == 0 {
        tel26_error_box(MANAGER_SCREENS_CPP, 0x695);
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x6b25e0usize, 0x6b2790usize, 1, 0usize, 0) == 0 {
        return;
    }
    let slot = exe::newsitem_get_slot(item.as_mut_ptr() as usize, 0);
    let club = read_usize(G_CLUB_TABLE).wrapping_add(slot.wrapping_mul(0x245) as usize);
    set_screen_param(0, news_id as usize);
    set_screen_param(1, club);
}

/// Builds the "Accept the position as manager of <club> ?" confirm screen (`manager_confirm_build` on parameter 1).
/// Original: `job_offer_screen_build` (0x6b25e0, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn job_offer_screen_build() {
    covered!("tel26_4 job offer");
    manager_confirm_build(1, 0xa154c0);
}

/// The job offer's answer: "No" (id 1) marks the news item (parameter 0) answered with 2
/// (`News::set_item_slot(manager, id, 1, 2, 0)`) and puts the vacancy back (`add_job_vacancy` of the club in
/// parameter 1); "Yes" (id 2) marks it with 1 and appoints the slot's manager (`appoint_manager_to_club`); both
/// close with -9; other ids go to the menu bar.
/// Original: `job_offer_screen_handle_action` (0x6b2790, manager_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn job_offer_screen_handle_action(raw: u32) -> i32 {
    let news = get_screen_param(0);
    let club = get_screen_param(1);
    let manager = tel26_slot_manager().addr();
    match tel26_item_id(raw as i16) {
        1 => {
            exe::news_set_item_slot(read_usize(0xb7431c), manager, news, 1, 2, 0);
            exe::manager_manager_add_job_vacancy(read_usize(0xb67a44), club, manager);
            -9
        }
        2 => {
            exe::news_set_item_slot(read_usize(0xb7431c), manager, news, 1, 1, 0);
            exe::manager_manager_appoint_manager_to_club(read_usize(0xb67a44), club, manager);
            -9
        }
        _ => manager_menu_bar(raw),
    }
}

/// Comparer for `open_apply_for_job_screen`.
pub(crate) unsafe extern "C" fn compare_open_apply_for_job_screen(club: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize)>(IDX_OPEN_APPLY_FOR_JOB_SCREEN)(club)
}

/// Comparer for `apply_for_job_screen_build`: the confirm screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_apply_for_job_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_APPLY_FOR_JOB_SCREEN_BUILD);
    compare_by_display_list_with("Apply for job screen (apply_for_job_screen_build)", &[], || o(), || apply_for_job_screen_build());
}

/// Comparer for `apply_for_job_screen_handle_action`.
pub(crate) unsafe extern "C" fn compare_apply_for_job_screen_handle_action(item: u32) -> i32 {
    // simplification: the application and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_APPLY_FOR_JOB_SCREEN_HANDLE_ACTION)(item)
}

/// Comparer for `open_resign_job_screen`.
pub(crate) unsafe extern "C" fn compare_open_resign_job_screen(club: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize)>(IDX_OPEN_RESIGN_JOB_SCREEN)(club)
}

/// Comparer for `resign_job_screen_build`: the confirm screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_resign_job_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_RESIGN_JOB_SCREEN_BUILD);
    compare_by_display_list_with("Resign screen (resign_job_screen_build)", &[], || o(), || resign_job_screen_build());
}

/// Comparer for `resign_job_screen_handle_action`.
pub(crate) unsafe extern "C" fn compare_resign_job_screen_handle_action(item: u32) -> i32 {
    // simplification: the resignation and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_RESIGN_JOB_SCREEN_HANDLE_ACTION)(item)
}

/// Comparer for `open_board_confidence_screen`.
pub(crate) unsafe extern "C" fn compare_open_board_confidence_screen(club: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize)>(IDX_OPEN_BOARD_CONFIDENCE_SCREEN)(club)
}

/// Comparer for `board_confidence_screen_build`: the Board Confidence screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_board_confidence_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_BOARD_CONFIDENCE_SCREEN_BUILD);
    compare_by_display_list_with("Board Confidence screen (board_confidence_screen_build)", &[], || o(), || board_confidence_screen_build());
}

/// Comparer for `board_confidence_screen_handle_action`.
pub(crate) unsafe extern "C" fn compare_board_confidence_screen_handle_action(item: u32) -> i32 {
    // simplification: opening the next screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_BOARD_CONFIDENCE_SCREEN_HANDLE_ACTION)(item)
}

/// Comparer for `open_job_offer_screen`.
pub(crate) unsafe extern "C" fn compare_open_job_offer_screen(manager: usize, news_id: u32) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_OPEN_JOB_OFFER_SCREEN)(manager, news_id)
}

/// Comparer for `job_offer_screen_build`: the confirm screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_job_offer_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_JOB_OFFER_SCREEN_BUILD);
    compare_by_display_list_with("Job offer screen (job_offer_screen_build)", &[], || o(), || job_offer_screen_build());
}

/// Comparer for `job_offer_screen_handle_action`.
pub(crate) unsafe extern "C" fn compare_job_offer_screen_handle_action(item: u32) -> i32 {
    // simplification: answering the offer and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_JOB_OFFER_SCREEN_HANDLE_ACTION)(item)
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
            exe::gui_add_page_buttons((0x30c) as i32 as u32, (0x91) as i32 as u32, (page) as usize as u32, (pages) as usize as u32, 3, 4, 5, 6);
        } else {
            exe::gui_add_page_buttons((0x30c) as i32 as u32, (0x91) as i32 as u32, (page) as usize as u32, (pages) as usize as u32, 0, 4, 5, 0);
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

/// Comparer for `manager_stats_screen_build`.
///
/// Side by side over the display list (`compare_by_display_list_with`: the screen manager, the current `Display`,
/// the screen parameters, the work text) plus the list of parameter 1 (count x 210 bytes). Without a list the routine
/// builds it (allocation, the sort statics, parameters 1 and 2): only the original runs.
pub(crate) unsafe extern "C" fn compare_manager_stats_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_MANAGER_STATS_SCREEN_BUILD);
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
    let manager = tel26_slot_manager();
    manager_job_background(manager.addr());
    exe::menubar_build(4, 0);
    let (mut fore, mut back) = (0u32, 0u32);
    let team = if manager.club_job().addr() != 0 { manager.club_job().addr() } else { manager.national_job().addr() };
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
    let club = manager.club_job().addr();
    let nation = manager.national_job().addr();
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
            let manager = tel26_slot_manager();
            let club = manager.club_job().addr();
            if club != 0 {
                exe::manager_manager_remove_club_manager(read_usize(G_MANAGER_MANAGER), club, 3, 0, 0);
                if read_usize(G_MATCH_DAY) != 0 {
                    exe::match_day_refresh_human_interest_flags(read_usize(G_MATCH_DAY));
                }
                exe::manager_manager_add_job_vacancy(read_usize(G_MANAGER_MANAGER), club, manager);
                exe::transfer_manager_cancel_club_offers(read_usize(G_TRANSFER_MANAGER), club);
            }
            let nation = manager.national_job().addr();
            if nation != 0 {
                exe::manager_manager_remove_club_manager(read_usize(G_MANAGER_MANAGER), nation, 3, 0, 0);
                if read_usize(G_MATCH_DAY) != 0 {
                    exe::match_day_refresh_human_interest_flags(read_usize(G_MATCH_DAY));
                }
                exe::manager_manager_add_job_vacancy(read_usize(G_MANAGER_MANAGER), nation, manager);
            }
            exe::news_clear_inbox(read_usize(G_NEWS), manager);
            exe::human_manager_set_on_holiday(read_usize(G_HUMAN_MANAGER), 0, manager);
            tel35_this2(0x5fd590, read_usize(G_HUMAN_MANAGER), 0, manager.addr());
            tel35_this2(0x5fe140, read_usize(G_HUMAN_MANAGER), 0, manager.addr());
            exe::contract_manager_remove_pending_offers_for_staff(read_usize(G_CONTRACT_MANAGER), manager);
            exe::contract_manager_remove_contract(read_usize(G_CONTRACT_MANAGER), manager);
            tel35_this1(0x4d0bd0, read_usize(G_CONTRACT_MANAGER), manager.addr());
            tel35_this1(0x775ba0, read_usize(G_NATIONAL_TEAMS), manager.addr());
            tel35_this1(0x86fb80, read_usize(G_STAFF_RECORDS), manager.addr());
            tel35_this1(0x86fc70, read_usize(G_STAFF_RECORDS), manager.addr());
            tel35_this1(0x7fc310, read_usize(G_SCOUT_MANAGER), manager.addr());
            let first = manager.first_name();
            manager.set_classification(0);
            write_u8(first, 0);
            write_u8(manager.second_name(), 0);
            write_u8(manager.common_name(), 0);
            exe::date_set(manager.addr() + staff::DATE_OF_BIRTH, 1, 0, 0x7ae, -1);
            manager.set_year_of_birth(0x7ae);
            write_u32(manager.addr() + staff::NATIONAL_JOB, 0);
            write_u32(manager.addr() + staff::CLUB_JOB, 0);
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
    let pa = Staff(staff_table.wrapping_add((id_a as u32).wrapping_mul(staff::SIZE as u32) as usize));
    let sa = pa.non_player().addr();
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
            let ra = manager_stats_order_reputation(a, pa.addr(), id_a, read_i16(sa + at));
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
            if pa.club_job().addr() != 0 {
                d = d.wrapping_sub(manager_stats_days(G_CURRENT_DATE, pa.addr() + staff::DATE_JOINED_CLUB));
            }
            d
        }
        4 => read_u8(b + 0xb5) as i32 - read_u8(a + 0xb5) as i32 - read_u8(a + 0xb4) as i32 + read_u8(b + 0xb4) as i32,
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
        _ => pa.addr() as i32,
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
    original_fn::<unsafe extern "C" fn()>(IDX_OPEN_RETIRE_SCREEN)()
}

/// Comparer for `retire_screen_build`: the screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_retire_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_RETIRE_SCREEN_BUILD);
    compare_by_display_list_with("Retire screen (retire_screen_build)", &[], || o(), || retire_screen_build());
}

/// Comparer for `retire_screen_handle_action`.
pub(crate) unsafe extern "C" fn compare_retire_screen_handle_action(item: u32) -> i32 {
    // simplification: the retirement and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_RETIRE_SCREEN_HANDLE_ACTION)(item)
}

/// Comparer for `open_holiday_screen`.
pub(crate) unsafe extern "C" fn compare_open_holiday_screen() {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(IDX_OPEN_HOLIDAY_SCREEN)()
}

/// Comparer for `holiday_screen_build`: the screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_holiday_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_HOLIDAY_SCREEN_BUILD);
    compare_by_display_list_with("Holiday screen (holiday_screen_build)", &[], || o(), || holiday_screen_build());
}

/// Comparer for `holiday_screen_handle_action`.
pub(crate) unsafe extern "C" fn compare_holiday_screen_handle_action(item: u32) -> i32 {
    // simplification: the holiday settings and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_HOLIDAY_SCREEN_HANDLE_ACTION)(item)
}

/// Frees the list of screen parameter 1 when there is one and clears parameters 1 and 2.
pub(crate) unsafe fn tel26_free_list() {
    let list = get_screen_param(1);
    if list != 0 {
        exe::crt_free(list);
        set_screen_param(1, 0);
        set_screen_param(2, 0);
    }
}

/// The nation id of a nation (+0).
pub(crate) unsafe fn tel26_nation_id(n: Nation) -> usize {
    read_u32(n.addr()) as usize
}

/// One line of the nation menu: an item of type 2 (0x2002 when it is the chosen nation, value 0, not clickable as a
/// choice) in `area`.
pub(crate) unsafe fn job_nation_line(d: usize, chosen: bool, value: u32, area: u32) {
    let (kind, link, v) = if chosen { (0x2002, 0, 0) } else { (2, 1, value) };
    tel26_add_item(d, [kind, 0, 0, 0, 0, 0, 0, 0x10, job_colour(JOB_COLOUR_31AA), job_colour(JOB_COLOUR_BDEC), 1, 1,
        job_colour(JOB_COLOUR_BDEA), JOB_TEXT as u32, 0, link, v, area]);
}

/// Opens a "Please Confirm" screen for `club` (`open_screen` with `builder`, `handler`, 1, 0, 0; parameter 0 =
/// the club); a null club shows the manager_screens.cpp box at `line` instead.
pub(crate) unsafe fn manager_confirm_open(club: usize, builder: usize, handler: usize, single: u32, line: u32) {
    if club == 0 {
        tel26_error_box(MANAGER_SCREENS_CPP, line);
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, builder, handler, single, 0usize, 0) != 0 {
        set_screen_param(0, club);
    }
}

/// The "Please Confirm" screen of a club (parameter `club_param`): the club's background picture, the menu bar
/// (4, 0), the title (the human manager's name in the club's kit colours, `database_get_club_kit_colours`), the
/// subtitle "Please Confirm", the question (`question`, with the club's name and gender char +0x37) as a text area,
/// and a two-column area (weights 3/1) with "No" (id 1) and "Yes" (id 2).
pub(crate) unsafe fn manager_confirm_build(club_param: u32, question: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let weights: [u8; 2] = [3, 1];
    let club = get_screen_param(club_param);
    exe::background_set_club_picture(club, 0, 0);
    exe::menubar_build(4, 0);
    let (mut fore, mut back) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut fore, &mut back);
    exe::database_format_staff_name(tel26_slot_manager(), JOB_TEXT, 0x7d0, 1);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, JOB_TEXT);
    text(JOB_TEXT, 0x9909b8);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let gender = read_u8(club + 0x37) as i8 as i32;
    text(JOB_TEXT, question, gender, gender, club + 4);
    tel26_text_area([0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0]);
    let area = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    text(JOB_TEXT, 0x98e8cc);
    tel26_area_button([area, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 1, 0]);
    text(JOB_TEXT, 0x98e8d0);
    tel26_area_button([area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 2, 0]);
}

/// The menu bar's answer to a widget the screen does not handle: `menubar_handle_command`, then
/// `menubar_handle_object_link` when it gives 0.
pub(crate) unsafe fn manager_menu_bar(raw: u32) -> i32 {
    let r = exe::menubar_handle_command(raw);
    if r != 0 { r } else { exe::menubar_handle_object_link(raw) }
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
pub(crate) unsafe fn manager_stats_menu(nation: i32, stat: u8, header: &mut [u8; 200]) {
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
        let n = Nation(read_usize(G_NATION_TABLE) + nation as usize * nation::SIZE);
        let g = (n.short_gender_name() as i8) as i32;
        text(BUF, 0xa15300, g, g, n.addr() + nation::SHORT_NAME);
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
pub(crate) unsafe fn manager_stats_nation_menu(nation: i32, cols: &mut [u8; 8]) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let caixa: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let list: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    // key_nation.cpp: key_nation_sorted_order (cdecl), NATION_MENU_SPLIT ctor / part_count / format_header /
    // in_second_part (thiscall; ret 0xc, 0, 0xc, 4), checked against the disassembly
    let in_second_part: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_NATION_MENU_SPLIT_IN_SECOND_PART);
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
    let order = exe::key_nation_sorted_order(1);
    let mut split = [0u8; 16];
    let sp = split.as_mut_ptr() as usize;
    exe::new_nation_menu_split(sp, 0, order, 1);
    let mut part_area = 0usize; // ponytail: uninitialised in the original, always set before use
    for k in 0..0x23usize {
        let e = Nation(read_usize(G_LEAGUE_TABLE + (read_i8(order + k) as isize * 0x48) as usize));
        if e.is_null() || e.league_selected() & 3 == 0 {
            continue;
        }
        let mut new_part = false;
        if exe::nation_menu_split_part_count(sp) as i8 > 1 && in_second_part(sp, e.addr()) != cols[4] {
            covered!("tel26_5 manager stats nation menu part");
            cols[4] = in_second_part(sp, e.addr());
            exe::nation_menu_split_format_header(sp, BUF, 0x7d0, u32::from_le_bytes([cols[4], cols[5], cols[6], cols[7]]));
            let item = manager_stats_add_item(2, 0x10, BUF, 0, 0, 0, area);
            part_area = list(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, colour_word(MANAGER_STATS_MENU_COLOUR), item);
            new_part = true;
        }
        if !new_part && exe::nation_menu_split_part_count(sp) == 1 {
            part_area = area;
        }
        format(BUF, 0x98ed14, e.addr() + nation::SHORT_NAME);
        let id = read_i32(e.addr());
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
pub(crate) unsafe fn manager_stats_collect(nation: i32, stat: u8, mut count: i32) -> Option<(usize, i32)> {
    let is_manager = |j: u8| j == 5 || j == 12;
    let mut list = 0usize;
    let mut k = 0i32;
    while k < read_i32(G_STAFF_COUNT) {
        let s = Staff(read_usize(G_STAFF_TABLE) + k as usize * staff::SIZE);
        k += 1;
        if s.non_player().addr() == 0
            || !(is_manager(s.job_for_club()) || is_manager(s.job_for_nation())) {
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
pub(crate) unsafe fn manager_stats_row(stat: u8, nation: i32, area: usize, row: i32, idx: i32, rec: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let cell: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let text_of_cell: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    const BUF: usize = 0xde1f64;
    let sm = SCREEN_MANAGER;
    let row_ = row as usize;
    let s = Staff(read_usize(G_STAFF_TABLE) + read_i32(rec + manager_record::OWNER) as usize * staff::SIZE);
    let ord = exe::utils_format_ordinal(idx + 1, -1);
    text_of_cell(sm, area, 0, row_, 0x30, colour_word(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, colour_with_high_bits(ord, G_GUI_COLOUR_PALE_GREY), ord);
    let mut name = [0u8; 0x7d0];
    exe::database_format_staff_name(s, name.as_mut_ptr() as usize, 0x7d0, 1);
    format(BUF, 0x987ff0, name.as_ptr() as usize);
    let human_from = read_i32(G_STAFF_COUNT) - 0x10;
    let id = read_i32(s.addr());
    let colour = if id < human_from {
        colour_with_high_bits(human_from as usize, G_GUI_COLOUR_PALE_GREY)
    } else {
        covered!("tel26_5 manager stats human row");
        colour_with_high_bits(id as usize, G_GUI_COLOUR_YELLOW)
    };
    cell(sm, area, 1, row_, 1, 0, 1, 3, colour, BUF, 0x7d7, s.addr());
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
        let club = s.club_job().addr();
        let national = s.national_job().addr();
        let at_club = club != 0 && is_manager(s.job_for_club());
        let at_nation = national != 0 && is_manager(s.job_for_nation());
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
    manager_stats_stat_text(stat, nation, s.addr(), rec);
    text_of_cell(sm, area, 3, row_, 0x30, colour_word(MANAGER_STATS_CELL_COLOUR), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), BUF);
}

/// The back and forward buttons of the Manager Stats screen: forward asked first, then back, each enabled when the
/// screen manager says it is available.
pub(crate) unsafe fn manager_stats_back_next() {
    let forward = (exe::screenmanager_is_forward_unavailable(SCREEN_MANAGER) == 0) as u32;
    let back = (exe::screenmanager_is_back_unavailable(SCREEN_MANAGER, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(back, forward);
}

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
pub(crate) unsafe fn holiday_option(option: u32) -> u8 {
    exe::human_manager_get_holiday_option(read_usize(G_HUMAN_MANAGER), option, 0usize)
}

/// `HUMAN_MANAGER::set_holiday_option(option, value, 0)` (0x5feb60, thiscall, ret 0xc).
pub(crate) unsafe fn set_holiday_option(option: u32, value: i32) {
    exe::human_manager_set_holiday_option(read_usize(G_HUMAN_MANAGER), option, value, 0)
}

/// The background picture of the manager's screens: his club's, else his national team's, else a random one.
pub(crate) unsafe fn manager_job_background(manager: usize) {
    let manager = Staff(manager);
    let club = manager.club_job().addr();
    let nation = manager.national_job().addr();
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
pub(crate) unsafe fn manager_no_yes(weights: &[u8; 2]) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let area = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    text(JOB_TEXT, 0x98e8cc);
    tel26_area_button([area, 0, 0, 0x30, tel35_colour(G_GUI_COLOUR_GREY), 0xc, 3, tel35_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 1, 0]);
    text(JOB_TEXT, 0x98e8d0);
    tel26_area_button([area, 1, 0, 0x30, tel35_colour(G_GUI_COLOUR_GREY), 0xc, 3, tel35_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 2, 0]);
}

/// The reputation and age test of the manager stats order: a reputation below `MANAGER_STATS_UNKNOWN_BELOW` counts as
/// -1 for a person under 35 (one of the last 16 staff: with fewer than 50 games).
pub(crate) unsafe fn manager_stats_order_reputation(row: usize, person: usize, id: i32, reputation: i16) -> i16 {
    let person = Staff(person);
    if reputation >= MANAGER_STATS_UNKNOWN_BELOW {
        return reputation;
    }
    let limit = read_i32(G_STAFF_COUNT) - 0x10;
    let unknown = if id < limit { (person.age() as i8) < 0x23 } else { read_i16(row + 0x8e) < 0x32 };
    if unknown { -1 } else { reputation }
}

/// The days since a date of a manager record (`Date::days_since`, the date at `at` by value, from `this`).
pub(crate) unsafe fn manager_stats_days(this: usize, at: usize) -> i32 {
    exe::date_days_since(this, read_u32(at), read_u32(at + 4))
}

/// The win share of a manager record: 100 × wins (+0x90) / games (+0x8e) times the games (at most 100); -1 without
/// games.
pub(crate) unsafe fn manager_stats_win_score(row: usize) -> i32 {
    let games = read_i16(row + 0x8e);
    if games == 0 {
        return -1;
    }
    let share = (read_i16(row + 0x90) as i32 * 100) / games as i32;
    let weight = if games >= 0x64 { 0x64 } else { games as i32 };
    weight.wrapping_mul(share)
}

/// One item of the current human's `Display` with the menu colours (0xae31aa, yellow, black, all three short fields of
/// the widget, passed zero-extended).
pub(crate) unsafe fn manager_stats_add_item(kind: usize, style: usize, text: usize, a: usize, b: usize, c: usize, area: usize) -> usize {
    let row: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    row(current_human_display(), kind, 0, 0, 0, 0, 0, 0, style, colour_word(MANAGER_STATS_MENU_COLOUR),
        colour_word(G_GUI_COLOUR_YELLOW), 1, 1, colour_word(G_GUI_COLOUR_BLACK), text, a, b, c, area)
}

/// Copies the text after the first character of the work text (`0xde1f64` + 1) into `header`, as the original's
/// inline strcpy does, and returns the byte count it copied (the original leaves it in `eax`).
pub(crate) unsafe fn manager_stats_copy_header(header: &mut [u8; 200]) -> usize {
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
pub(crate) unsafe fn manager_stats_stat_text(stat: u8, nation: i32, s: usize, rec: usize) {
    let s = Staff(s);
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
            let np = s.non_player().addr();
            let rep = if nation == -1 { read_i16(np + non_player::WORLD_REPUTATION) } else { read_i16(np + non_player::CURRENT_REPUTATION) };
            if rep < MANAGER_STATS_UNKNOWN_BELOW
                && ((read_i32(s.addr()) >= human_from && read_i16(rec + manager_record::GAMES) < 0x32)
                    || (read_i32(rec + manager_record::OWNER) < human_from && (s.age() as i8) < 0x23)) {
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
        3 if s.club_job().addr() != 0 => days(G_CURRENT_DATE, s.addr() + staff::DATE_JOINED_CLUB),
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

/// The money statistics: the 32-bit fee through a local `Cash` (`exe::cash_get_display_amount` and
/// `exe::cash_format_money` with flags 5) into the work text.
pub(crate) unsafe fn manager_stats_money(fee: u32) {
    let cash = fee;
    let amount = exe::cash_get_display_amount(&cash, 5);
    exe::cash_format_money(0xde1f64usize, 0x7d0, amount, 5);
}

pub(crate) const MANAGER_SCREENS_CPP: usize = 0xa15138;

/// The colour word at 0xae31aa (no name yet), pushed as the first colour of every Manager Stats menu entry.
pub(crate) const MANAGER_STATS_MENU_COLOUR: usize = 0xae31aa;

/// The colour word at 0xae3192 (no name yet), the cell colour of the statistic column.
pub(crate) const MANAGER_STATS_CELL_COLOUR: usize = 0xae3192;

/// The size of one manager record copied into the screen's list (`manager_record`, 210 bytes).
pub(crate) const MANAGER_STATS_RECORD_SIZE: usize = 0xd2;

/// The qsort comparer of the list (0x6b36f0, still in the original) and the two statics it reads: the nation filter
/// (i32) and the chosen statistic (byte).
pub(crate) const MANAGER_STATS_SORT_COMPARER: usize = 0x6b36f0;

pub(crate) const MANAGER_STATS_SORT_NATION: usize = 0xa14f54;

pub(crate) const MANAGER_STATS_SORT_STAT: usize = 0xa14f58;

/// The menu entries of the statistic button, in order (statistic 1..16): the text (each starting with a space) and
/// whether a separator follows it.
pub(crate) const MANAGER_STATS_MENU: [(usize, bool); 16] = [
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
pub(crate) const MANAGER_STATS_UNKNOWN_BELOW: i16 = 0xea6;

pub(crate) const MANAGER_STATS_REPUTATION_BANDS: [(i16, usize); 7] = [
    (0x251c, 0xa150dc), // "World Class"
    (0x2134, 0xa150b4), // "Superb"
    (0x1d4c, 0xa15088), // "Very Good"
    (0x1964, 0xa15060), // "Good"
    (0x1482, 0xa15038), // "Fair"
    (0xea6, 0xa1500c),  // "Average"
    (0x4e2, 0xa14fe4),  // "Poor"
];

/// The holiday settings, in order: the text, the link value, the (option, value) whose match lights the line, the rows
/// to skip before it. A value of -1 lights the line when the option is not 0.
pub(crate) const HOLIDAY_SETTINGS: [(usize, u32, (u32, i32), i32); 7] = [
    (0xa156c8, 0, (0, 1), 2), // "Reject all offers for players"
    (0xa156a0, 1, (0, 2), 1), // "Reject all offers below asking price"
    (0xa15688, 2, (4, 1), 1), // "Do not buy any players"
    (0xa15668, 3, (4, 2), 1), // "Only buy players from shortlist"
    (0xa15630, 4, (1, -1), 2), // "Apply for manager position at higher reputation clubs"
    (0xa15614, 5, (2, -1), 1), // "Use current match tactics"
    (0xa155e8, 6, (3, -1), 1), // "Use current team selection when possible"
];
