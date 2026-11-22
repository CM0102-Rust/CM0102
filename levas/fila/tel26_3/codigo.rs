// ---------- batch %%LEVA%%: the manager stats and job information screens (manager_screens.cpp) ----------

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

/// One line of the nation menu: an item of type 2 (0x2002 when it is the chosen nation, value 0, not clickable as a
/// choice) in `area`.
unsafe fn job_nation_line(d: usize, chosen: bool, value: u32, area: u32) {
    let (kind, link, v) = if chosen { (0x2002, 0, 0) } else { (2, 1, value) };
    tel26_add_item(d, [kind, 0, 0, 0, 0, 0, 0, 0x10, job_colour(JOB_COLOUR_31AA), job_colour(JOB_COLOUR_BDEC), 1, 1,
        job_colour(JOB_COLOUR_BDEA), JOB_TEXT as u32, 0, link, v, area]);
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
    let sorted_nations: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x66a0c0usize);
    let split_new: unsafe extern "thiscall" fn(*mut u8, u32, usize, u32) = core::mem::transmute(0x66a280usize);
    let split_parts: unsafe extern "thiscall" fn(*mut u8) -> i8 = core::mem::transmute(0x66a350usize);
    let split_second: unsafe extern "thiscall" fn(*mut u8, usize) -> u8 = core::mem::transmute(0x66a430usize);
    let split_header: unsafe extern "thiscall" fn(*mut u8, usize, u32, u32) = core::mem::transmute(0x66a360usize);
    let page_buttons: unsafe extern "C" fn(u32, u32, u32, u32, u32, u32, u32, u32) = core::mem::transmute(0x5ede20usize);
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
    let order = sorted_nations(1);
    let mut split = [0u8; 16];
    split_new(split.as_mut_ptr(), 0, order, 1);
    // the original's part area starts uninitialised; it is always set before use when the split has 1 or 2 parts
    let mut part_area: u32 = 0;
    for i in 0..0x23usize {
        let n = read_usize((G_LEAGUE_TABLE as isize + read_u8(order + i) as i8 as isize * 0x48) as usize);
        if n == 0 || read_u8(n + nation::LEAGUE_SELECTED) & 3 == 0 {
            continue;
        }
        let mut header = false;
        if split_parts(split.as_mut_ptr()) > 1 && split_second(split.as_mut_ptr(), n) != group {
            header = true;
            group = split_second(split.as_mut_ptr(), n);
            split_header(split.as_mut_ptr(), JOB_TEXT, 0x7d0, group as u32);
            let item = tel26_add_item(d, [2, 0, 0, 0, 0, 0, 0, 0x10, job_colour(JOB_COLOUR_31AA), job_colour(JOB_COLOUR_BDEC), 1, 1,
                job_colour(JOB_COLOUR_BDEA), JOB_TEXT as u32, 0, 0, 0, menu]);
            part_area = tel26_add_area(d, [0, 0, 0, 0, 0, 0, 0, 0, 0x30, (item & 0xffff_0000) | job_colour(JOB_COLOUR_31AA), item]);
        }
        if !header && split_parts(split.as_mut_ptr()) == 1 {
            part_area = menu;
        }
        format(JOB_TEXT, 0x98ed14, n + 0x38);
        let id = read_u32(n);
        job_nation_line(d, id == nation, id, part_area);
    }
    if list == 0 {
        let clubs = read_i32(G_CLUB_COUNT);
        let mut offset = count.wrapping_mul(10);
        for j in 0..clubs.max(0) as usize {
            let club = read_usize(G_CLUB_TABLE) + j * 0x245;
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
        page_buttons(0x30c, 0x91, page as u32, pages as u32, 0, 2, 3, 0);
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
    original_fn::<unsafe extern "C" fn()>(%%I0%%)()
}

/// Comparer for `manager_stats_screen_handle_action`.
pub(crate) unsafe extern "C" fn compare_manager_stats_screen_handle_action(item: u32) -> i32 {
    // simplification: freeing the list and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I1%%)(item)
}

/// Comparer for `manager_stats_screen_close`.
pub(crate) unsafe extern "C" fn compare_manager_stats_screen_close() {
    // simplification: the free does not run twice
    original_fn::<unsafe extern "C" fn()>(%%I2%%)()
}

/// Comparer for `open_job_information_screen`.
pub(crate) unsafe extern "C" fn compare_open_job_information_screen() {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(%%I3%%)()
}

/// Comparer for `job_information_screen_build`: the Job Information screen side by side over the display list,
/// with the list (parameter 1); without a list (it is allocated here) only the original.
pub(crate) unsafe extern "C" fn compare_job_information_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I4%%);
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
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I5%%)(item)
}

/// Comparer for `job_information_screen_close`.
pub(crate) unsafe extern "C" fn compare_job_information_screen_close() {
    // simplification: the free does not run twice
    original_fn::<unsafe extern "C" fn()>(%%I6%%)()
}
