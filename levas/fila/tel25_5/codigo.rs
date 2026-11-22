// ---------- batch %%LEVA%%: the training screens (4): the training screen's builder (training_screens.cpp) ----------

/// The edit session's fields the training screen reads: the current schedule (i8, 0..9), the schedules from +0xfa
/// (0x9b bytes each), the club, the squad list (an LLIST of 5-byte entries: +0 the person, +4 the schedule).
const TRAINING_SESSION_CURRENT: usize = 0x7d9;
const TRAINING_SESSION_SCHEDULES: usize = 0xfa;
const TRAINING_SESSION_CLUB: usize = 0x806;
/// A schedule's fields: +9 its name, +0x15 its kind (0 and −1 nothing to edit, 7 a preset that cannot be renamed),
/// +0x18 the squad view (0x7d bytes, see `TRAINING_SQUAD_VIEW`), +0x2e the file it was saved to, +0x93 saved since the
/// last change, +0x97 it has a file [inference: from the menu's Save and Load].
const TRAINING_SCHEDULE_NAME: usize = 9;
const TRAINING_SCHEDULE_KIND: usize = 0x15;
const TRAINING_SCHEDULE_FILE: usize = 0x2e;
const TRAINING_SCHEDULE_SAVED: usize = 0x93;
const TRAINING_SCHEDULE_HAS_FILE: usize = 0x97;
/// The squad view's fields: +0 the flags (bit 1 General, 4 Attributes, 0x10..0x200 the attribute group, 0x800..
/// 0x2000000 the sort column), +4 the sort attribute, +5 reversed, +9 the list's scroll row, +0xd the filter (1 all
/// positions, 2 goalkeepers, 4 defenders, 8 midfielders, 0x10 attackers, 0x20 all sides, 0x40 left, 0x80 central,
/// 0x100 right).
const TRAINING_VIEW_REVERSED: usize = 5;
const TRAINING_VIEW_SCROLL_ROW: usize = 9;
const TRAINING_VIEW_FILTER: usize = 0xd;
/// The tab of each schedule handed to `gui_add_tab_bar` (0xd4 bytes, `0x4568f0` builds them): +2 the colour, +4 the
/// text, +0xcc 1, +0xd0 the schedule.
const TRAINING_TAB_SIZE: usize = 0xd4;
/// "" (0xad9c64).
const TRAINING_EMPTY_TEXT: usize = 0xad9c64;
/// Colours the screen reads besides the gui colours: 0xae31aa (the popups), 0xae3152, 0xae317c, 0xae3190, 0xaebdcc.
const TRAINING_COLOUR_3152: usize = 0xae3152;
const TRAINING_COLOUR_317C: usize = 0xae317c;
const TRAINING_COLOUR_3190: usize = 0xae3190;
const TRAINING_COLOUR_BDCC: usize = 0xaebdcc;
/// The attribute groups of the Attributes view (nine attribute bytes and −1, sorted in place by the builder):
/// physical, mental, goalkeeping, defensive, attacking, and 0xa8d0c8 (all −1) for the view flag 0x200.
const TRAINING_ATTRIBUTE_GROUPS: usize = 0xa8d08c;

/// The edit session's schedule number `k` (a signed byte).
fn training_schedule(session: usize, k: i8) -> usize {
    (session as isize + TRAINING_SESSION_SCHEDULES as isize + k as isize * TRAINING_SCHEDULE_SIZE as isize) as usize
}

/// The session's current schedule.
unsafe fn training_current_schedule(session: usize) -> usize {
    training_schedule(session, read_i8(session + TRAINING_SESSION_CURRENT))
}

/// An entry of the training screen's popup menus (an item of the current human's Display in area `popup`): kind 2
/// (0x2002 ticked, 1 a separator), flags, state (1, 0x21 greyed), text, link and value, in the popup colours.
unsafe fn training_menu_item(popup: u32, kind: u32, flags: u32, state: u32, text: u32, link: u32, value: u32) -> u32 {
    tel26_add_item(current_human_display(), [kind, 0, 0, 0, 0, 0, 0, flags, job_colour(JOB_COLOUR_31AA), job_colour(JOB_COLOUR_BDEC), state, 1,
        job_colour(JOB_COLOUR_BDEA), text, 0, link, value, popup])
}

/// A ticked/plain popup entry with the text already in the buffer: link and value, ticked when `on`.
unsafe fn training_menu_choice(popup: u32, on: bool, link: u32, value: u32) {
    training_menu_item(popup, if on { 0x2002 } else { 2 }, 0x10, 1, TRAINING_TEXT_BUFFER as u32, link, value);
}

/// A popup owned by item `owner` (an empty area of style 0x130 in the popup colour).
unsafe fn training_popup(owner: u32) -> u32 {
    tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), owner])
}

/// A header button of the squad list: column `col` of `area`, the sort flag `sort` as value of link 3, centred
/// (style 0xc) or left (1).
unsafe fn training_list_header(area: u32, col: u32, style: u32, text: usize, sort: u32) {
    comp_lang_format()(TRAINING_TEXT_BUFFER, text, TRAINING_EMPTY_TEXT);
    tel26_area_button([area, col, 0, 0x30, job_colour(G_GUI_COLOUR_GREY), style, 1, job_colour(G_GUI_COLOUR_PALE_GREY), TRAINING_TEXT_BUFFER as u32, 3, sort]);
}

/// The schedule tabs: the default name of each of the ten schedules is set, and each tab shows the name, or for a
/// schedule of kind other than −1 and 7 "name (n)" with n `0x8b0d10` (how many of the squad follow it); a preset (kind
/// 7) is grey. Returns the tab bar's top and bottom (`gui_add_tab_bar`, from 0x50 and 0x221).
unsafe fn training_schedule_tabs(session: usize, view_index: i32, tabs: usize) -> (i32, i32) {
    let format: CompFormatFn = comp_crt_format();
    let count: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(0x8b0d10usize);
    for k in 0..10i8 {
        let schedule = training_schedule(session, k);
        exe::training_schedule_set_default_name(schedule);
        let tab = tabs + k as usize * TRAINING_TAB_SIZE;
        let kind = read_u8(schedule + TRAINING_SCHEDULE_KIND);
        if kind == 0xff || kind == 7 {
            tel26_copy_text((tab + 4) as *mut u8, (schedule + TRAINING_SCHEDULE_NAME) as *const u8);
        } else {
            let n = count(session, k as u8 as u32) as i8;
            format(tab + 4, 0xa8d164, schedule + TRAINING_SCHEDULE_NAME, n as i32); // "%s (%d)"
        }
        if read_u8(schedule + TRAINING_SCHEDULE_KIND) == 7 {
            write_u16(tab + 2, read_u16(G_GUI_COLOUR_GREY));
        }
        write_u32(tab + 0xcc, 1);
        write_i32(tab + 0xd0, k as i32);
    }
    let (mut top, mut bottom) = (0x50i32, 0x221i32);
    exe::gui_add_tab_bar(tabs, 10, view_index, &mut top, &mut bottom, 1);
    (top, bottom)
}

/// The Schedule menu (column 0 of the top area): greyed when the current schedule's kind is 0 or −1; otherwise a
/// popup with Load (link 2), Save (0xd; only when not saved and with a file), Save As (0xe; not for a preset), Delete
/// (0xf) and the last four schedule files (`0x5fc760` of the human manager): each, when it names a file, "n. name"
/// (link 0x10, value n − 1; greyed when `0x8b5820` cannot read it), the first one after a separator.
unsafe fn training_schedule_menu(session: usize, area: u32) {
    let format: CompFormatFn = comp_crt_format();
    type RecentFile = unsafe extern "thiscall" fn(usize, u32, *mut u8, u32, u32, u32, u32) -> u32;
    let recent: RecentFile = core::mem::transmute(0x5fc760usize);
    let readable: unsafe extern "C" fn(*const u8, *mut u8) -> u32 = core::mem::transmute(0x8b5820usize);
    let (grey, pale) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY));
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa8d138, TRAINING_EMPTY_TEXT); // "Schedule<..>"
    let schedule = training_current_schedule(session);
    let kind = read_u8(schedule + TRAINING_SCHEDULE_KIND);
    if kind == 0 || kind == 0xff {
        tel26_area_text([area, 0, 0, 0x30, grey, 0x2c, 1, pale, TRAINING_TEXT_BUFFER as u32]);
        return;
    }
    let owner = tel26_area_button([area, 0, 0, 0x30, grey, 0xc, 1, pale, TRAINING_TEXT_BUFFER as u32, 0, 0]);
    let popup = training_popup(owner);
    let text = TRAINING_TEXT_BUFFER as u32;
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa7e7f4); // "Load"
    training_menu_item(popup, 2, 0x10, 1, text, 2, 0);
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa7e7dc); // "Save"
    let schedule = training_current_schedule(session);
    if read_u32(schedule + TRAINING_SCHEDULE_SAVED) == 0 && read_u32(schedule + TRAINING_SCHEDULE_HAS_FILE) != 0 {
        training_menu_item(popup, 2, 0x10, 1, text, 0xd, 0);
    } else {
        training_menu_item(popup, 2, 0x10, 0x21, text, 0, 0);
    }
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa7e7c4); // "Save As"
    if read_u8(training_current_schedule(session) + TRAINING_SCHEDULE_KIND) != 7 {
        training_menu_item(popup, 2, 0x10, 1, text, 0xe, 0);
    } else {
        training_menu_item(popup, 2, 0x10, 0x21, text, 0, 0);
    }
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa69fb4); // "Delete"
    training_menu_item(popup, 2, 0x10, 1, text, 0xf, 0);
    let mut separator = u32::MAX;
    let (mut file, mut read_back) = ([0u8; 0x104], [0u8; 0x6c]);
    for k in 0..4u32 {
        let manager = read_usize(G_HUMAN_MANAGER);
        recent(manager, k, file.as_mut_ptr(), 0x104, 0, 0, 3);
        if file[0] == 0 {
            continue;
        }
        covered!("tel25_5 a recent schedule file");
        let ok = readable(file.as_ptr(), read_back.as_mut_ptr());
        recent(read_usize(G_HUMAN_MANAGER), k, file.as_mut_ptr(), 0x104, 1, 0, 3);
        if separator as i16 == -1 {
            separator = training_menu_item(popup, 1, 0x100_0010, 1, 0, 0, 0);
        }
        format(TRAINING_TEXT_BUFFER, 0xa7e728, k + 1, file.as_ptr()); // "%d. %s"
        if ok != 0 {
            training_menu_item(popup, 2, 0x10, 1, text, 0x10, k);
        } else {
            training_menu_item(popup, 2, 0x10, 0x21, text, 0, 0);
        }
    }
}

/// The View menu (column 1 of the top area; greyed when the squad list is empty): General (link 3, value 1) and
/// Attributes (value 4), ticked by the view flags; in the Attributes view, the Attributes menu (column 2): Physical,
/// Mental, Goalkeeping, Defensive and Attacking (link 3, values 0x10 to 0x100).
unsafe fn training_view_menu(area: u32, view: usize, squad_empty: bool) {
    let (grey, pale) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY));
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0x988004); // "View"
    if squad_empty {
        tel26_area_text([area, 1, 0, 0x30, grey, 0x2c, 1, pale, TRAINING_TEXT_BUFFER as u32]);
        return;
    }
    let owner = tel26_area_button([area, 1, 0, 0x30, grey, 0xc, 1, pale, TRAINING_TEXT_BUFFER as u32, 0, 0]);
    let popup = training_popup(owner);
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa8d10c, TRAINING_EMPTY_TEXT); // "  General<..>"
    training_menu_choice(popup, read_u8(view) & 1 != 0, 3, 1);
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa7e6e8); // "  Attributes"
    training_menu_choice(popup, read_u8(view) & 4 != 0, 3, 4);
    if read_u8(view) & 4 == 0 {
        return;
    }
    covered!("tel25_5 the attributes view menu");
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0x98ece8); // "Attributes"
    let owner = tel26_area_button([area, 2, 0, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), TRAINING_TEXT_BUFFER as u32, 0, 0]);
    let popup = training_popup(owner);
    // "  Physical", "  Mental", "  Goalkeeping", "  Defensive", "  Attacking"
    for (text, bit) in [(0xa7e6a4usize, 0x10u32), (0xa7e698, 0x20), (0xa7e688, 0x40), (0xa7e67c, 0x80), (0xa7e670, 0x100)] {
        comp_lang_format()(TRAINING_TEXT_BUFFER, text);
        training_menu_choice(popup, read_u32(view) & bit != 0, 3, bit);
    }
}

/// The Edit (or Coaches) button and the Filter menu, on the tab bar's line: a schedule of kind −1 shows Coaches (link
/// 4), kind 0 a grey Edit, any other Edit (link 5); the Filter popup lists All Positions, Goalkeepers, Defenders,
/// Midfielders, Attackers, a separator, All Sides, Left Sided, Central and Right Sided (link 0x11, the filter bit as
/// value), ticked by the view's filter.
unsafe fn training_edit_and_filter(session: usize, view: usize, top: i32) {
    let (grey, pale) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY));
    let area = tel26_add_area(current_human_display(), [0x212, top as u32, 0x30c, (top + 0x14) as u32, 2, 0, 1, 0, 1, 0, u32::MAX]);
    let kind = read_u8(training_current_schedule(session) + TRAINING_SCHEDULE_KIND);
    if kind == 0xff {
        comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa8d104); // "Coaches"
        tel26_area_button([area, 0, 0, 0x30, grey, 0xc, 1, pale, TRAINING_TEXT_BUFFER as u32, 4, 0]);
    } else {
        comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa8be68); // "Edit"
        if read_u8(training_current_schedule(session) + TRAINING_SCHEDULE_KIND) == 0 {
            tel26_area_text([area, 0, 0, 0x30, grey, 0x2c, 1, pale, TRAINING_TEXT_BUFFER as u32]);
        } else {
            tel26_area_button([area, 0, 0, 0x30, grey, 0xc, 1, pale, TRAINING_TEXT_BUFFER as u32, 5, 0]);
        }
    }
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0x98ebf8); // "Filter"
    let owner = tel26_add_button([0x28f, top as u32, 0x30c, (top + 0x14) as u32, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1,
        job_colour(G_GUI_COLOUR_PALE_GREY), TRAINING_TEXT_BUFFER as u32, 0, 0]);
    let popup = training_popup(owner);
    let filter = |bit: u32| read_u32(view + TRAINING_VIEW_FILTER) & bit != 0;
    // " All Positions", " Goalkeepers", " Defenders", " Midfielders", " Attackers"
    for (text, bit) in [(0x98ebe8usize, 1u32), (0x98ebd8, 2), (0x98ebcc, 4), (0x98ebbc, 8), (0x98ebb0, 0x10)] {
        comp_lang_format()(TRAINING_TEXT_BUFFER, text);
        training_menu_choice(popup, filter(bit), 0x11, bit);
    }
    training_menu_item(popup, 1, 0x100_0010, 1, 0, 0, 0);
    // " All Sides", " Left Sided", " Central", " Right Sided"
    for (text, bit) in [(0x98eba4usize, 0x20u32), (0x98eb98, 0x40), (0x98eb8c, 0x80), (0x98eb7c, 0x100)] {
        comp_lang_format()(TRAINING_TEXT_BUFFER, text);
        training_menu_choice(popup, filter(bit), 0x11, bit);
    }
}

/// Does player `person` pass the view's filter? All when both "all" bits (1 and 0x20) are on; otherwise each position
/// bit asks the position mask (`exe::formation_get_position_mask` of the player sheet) for goalkeeper (1), defender
/// (6), midfielder (0x38) or attacker (0x40), and each side bit for left (0x80), central (0x200) or right (0x800).
unsafe fn training_passes_filter(view: usize, person: usize) -> bool {
    let f = read_u32(view + TRAINING_VIEW_FILTER);
    if f & 1 != 0 && f & 0x20 != 0 {
        return true;
    }
    let m = exe::formation_get_position_mask(read_usize(person + staff::PLAYER));
    let f = read_u32(view + TRAINING_VIEW_FILTER);
    [(2u32, 1u16), (4, 6), (8, 0x38), (0x10, 0x40), (0x40, 0x80), (0x80, 0x200), (0x100, 0x800)]
        .iter().all(|&(bit, mask)| f & bit == 0 || m & mask != 0)
}

/// The status cell of a squad row (column 1): a short code and a link by `get_player_status_code` (0x4849f0): 1 Wpm, 2
/// Cup, 3 Ine, 9 Fgn (link 0x7e0 at a national team, else 0x7db), 4 on international duty (link 0x7d5 with the duty
/// club, else a plain text item with whatever the buffer holds), 5 Sus (0x7de / 0x7d9), 6 and 8 Inj (0x7dd / 0x7d8),
/// 0x15 Abs (kind 0x202), 7 Fgn, 0xa Yel (0x7de / 0x7d9, yellow from `graphics_rgb_to_pixel(0xb4, 0xb4, 0, 0)`), 0x10 Ctr
/// (0x7da), 0x11 Ret (0x7e0 / 0x7db), 0xb..0xf, 0x12..0x14 and 0x18 the transfer codes (0x7db); others nothing.
unsafe fn training_status_cell(area: u32, row: u32, person: usize, club: usize, status: u8) {
    let text = TRAINING_TEXT_BUFFER as u32;
    let lang = comp_lang_format();
    // kind 2 item: colours f8/f9 `c`, f12 `c12`, link and value
    let item = |kind: u32, c8: u32, c9: u32, c12: u32, link: u32, value: u32| {
        tel26_add_item(current_human_display(), [kind, 0, 0, 0, 0, 1, row, 0x30, c8, c9, 0xc, 1, c12, text, 0, link, value, area]);
    };
    let (blue, pale, red_ink) = (job_colour(G_GUI_COLOUR_DARK_BLUE), job_colour(G_GUI_COLOUR_PALE_GREY), job_colour(TRAINING_COLOUR_BDCC));
    let national = |code: u32, other: u32| if exe::database_is_national_team(club) != 0 { code } else { other };
    match status {
        1 | 2 | 3 => {
            match status {
                1 => lang(TRAINING_TEXT_BUFFER, 0x98e9c8), // "Wpm"
                2 => lang(TRAINING_TEXT_BUFFER, 0x98e998, TRAINING_EMPTY_TEXT), // "Cup<..>"
                _ => lang(TRAINING_TEXT_BUFFER, 0x98e994), // "Ine"
            };
            let link = national(0x7e0, 0x7db);
            item(2, blue, blue, pale, link, person as u32);
        }
        4 => {
            let duty = exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), person, G_CURRENT_DATE, club, 0, 0);
            if duty != 0 {
                item(2, blue, blue, pale, 0x7d5, duty);
            } else {
                tel26_add_item(current_human_display(), [1, 0, 0, 0, 0, 1, row, 0x30, blue, blue, 0xc, 1, pale, text, 0, 0, 0, area]);
            }
        }
        5 => {
            lang(TRAINING_TEXT_BUFFER, 0x98e990); // "Sus"
            let link = national(0x7de, 0x7d9);
            item(2, red_ink, red_ink, pale, link, person as u32);
        }
        6 => {
            lang(TRAINING_TEXT_BUFFER, 0x98e98c); // "Inj"
            let link = national(0x7dd, 0x7d8);
            item(2, red_ink, red_ink, pale, link, person as u32);
        }
        0x15 => {
            lang(TRAINING_TEXT_BUFFER, 0x98e988); // "Abs"
            item(0x202, red_ink, red_ink, pale, 0x7db, person as u32);
        }
        7 | 9 => {
            lang(TRAINING_TEXT_BUFFER, 0x98e984); // "Fgn"
            item(2, blue, blue, pale, 0x7db, person as u32);
        }
        8 => {
            lang(TRAINING_TEXT_BUFFER, 0x98e98c); // "Inj"
            let link = national(0x7dd, 0x7d8);
            let c = job_colour(TRAINING_COLOUR_3190);
            item(2, c, c, pale, link, person as u32);
        }
        0xa => {
            covered!("tel25_5 a yellow card status");
            lang(TRAINING_TEXT_BUFFER, 0x98e978); // "Yel"
            let link = national(0x7de, 0x7d9);
            let black = job_colour(G_GUI_COLOUR_BLACK);
            let first = exe::graphics_rgb_to_pixel(0xb4, 0xb4, 0, 0) as u16 as u32;
            let second = exe::graphics_rgb_to_pixel(0xb4, 0xb4, 0, 0) as u16 as u32;
            item(2, second, first, black, link, person as u32);
        }
        0xb..=0x14 | 0x18 => {
            match status {
                0x18 => lang(TRAINING_TEXT_BUFFER, 0x98e970), // "Trn"
                0xb => lang(TRAINING_TEXT_BUFFER, 0x98e974),  // "Bid"
                0xc => lang(TRAINING_TEXT_BUFFER, 0x98e96c),  // "Wnt"
                0xd => lang(TRAINING_TEXT_BUFFER, 0x98e934),  // "Req"
                0xe => lang(TRAINING_TEXT_BUFFER, 0x98e908, TRAINING_EMPTY_TEXT), // "Lst<..>"
                0xf => lang(TRAINING_TEXT_BUFFER, 0x98e904),  // "Loa"
                0x10 => lang(TRAINING_TEXT_BUFFER, 0x98e900), // "Ctr"
                0x11 => lang(TRAINING_TEXT_BUFFER, 0x98e8f8), // "Ret "
                0x12 => lang(TRAINING_TEXT_BUFFER, 0x98e8f0), // "Bos "
                0x13 => lang(TRAINING_TEXT_BUFFER, 0x98e8e8), // "Unh "
                _ => lang(TRAINING_TEXT_BUFFER, 0x98e8e0),    // "Fut "
            };
            let link = match status {
                0x10 => 0x7da,
                0x11 => national(0x7e0, 0x7db),
                _ => 0x7db,
            };
            let c = job_colour(TRAINING_COLOUR_317C);
            item(2, c, c, job_colour(G_GUI_COLOUR_YELLOW), link, person as u32);
        }
        _ => {}
    }
}

/// The colours of the attribute cells, read once at the start: risen, fallen and unchanged (the colourblind setting
/// swaps the last two).
struct TrainingAttributeColours {
    up: u32,
    down: u32,
    same: u32,
}

/// One row of the squad list: the player's schedule name (when its number is 2 or more), the status cell, the name (a
/// kind 0x46 item with link 0xc and the person; grey when injured), then by view: General: positions, age, form,
/// morale and condition ("Inj" when injured, else n%); Attributes: each attribute of the group in a colour by the
/// training change; last, the value (`Cash` 5) or "Free".
unsafe fn training_squad_row(session: usize, area: u32, row: u32, entry: usize, club: usize, table: usize, colours: &TrainingAttributeColours) {
    let format: CompFormatFn = comp_crt_format();
    let lang = comp_lang_format();
    let text = TRAINING_TEXT_BUFFER as u32;
    let person = read_usize(entry);
    exe::training_manager_update_player_training(read_usize(G_TRAINING_MANAGER), person, 0);
    let number = read_i8(entry + 4);
    let name = if number > 1 { training_schedule(session, number) + TRAINING_SCHEDULE_NAME } else { 0 };
    tel26_area_text([area, 0, row, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), name as u32]);
    let status_code: unsafe extern "C" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x4849f0usize);
    let status = status_code(person, club, 0);
    training_status_cell(area, row, person, club, status);
    exe::database_format_staff_name(person, TRAINING_TEXT_BUFFER, 0x7d0, 3);
    let ink = if status == 6 { job_colour(G_GUI_COLOUR_GREY) } else { job_colour(G_GUI_COLOUR_PALE_GREY) };
    tel26_add_item(current_human_display(), [0x46, 0, 0, 0, 0, 2, row, 1, 0, 0, 1, 2, ink, text, 0, 0xc, person as u32, area]);
    let view = read_usize(TRAINING_SQUAD_VIEW);
    let mut words = [0u8; 200];
    if read_u32(view) & 1 != 0 {
        let positions: unsafe extern "C" fn(usize, *mut u8, u32, u32) -> u32 = core::mem::transmute(0x53ee50usize);
        positions(read_usize(person + staff::PLAYER), words.as_mut_ptr(), 0xc8, 1);
        format(TRAINING_TEXT_BUFFER, 0x987ff0, words.as_ptr()); // "  %s"
        tel26_area_text([area, 3, row, 1, 0, 1, 2, job_colour(JOB_COLOUR_BDE8), text]);
        format(TRAINING_TEXT_BUFFER, 0x98806c, read_i8(person + staff::AGE) as i32); // "%d"
        tel26_area_text([area, 4, row, 1, 0, 0xc, 2, job_colour(JOB_COLOUR_BDE8), text]);
        let form: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x7c1660usize);
        form(read_usize(G_PLAYER_STATS), read_usize(person + staff::PLAYER), TRAINING_TEXT_BUFFER, 0x7d0);
        tel26_area_text([area, 5, row, 1, 0, 0xc, 1, job_colour(G_GUI_COLOUR_YELLOW), text]);
        // the original loads ecx with the contract manager; the routine is stdcall and does not read it
        let morale: unsafe extern "stdcall" fn(usize, *mut u8, u32) = core::mem::transmute(0x4cc2e0usize);
        morale(person, words.as_mut_ptr(), 0xc8);
        format(TRAINING_TEXT_BUFFER, 0x991944, words.as_ptr()); // "%s"
        tel26_area_text([area, 6, row, 1, 0, 0xc, 2, job_colour(G_GUI_COLOUR_YELLOW), text]);
        if exe::injury_get_player_injury_status(read_usize(G_INJURY), person) == 2 {
            lang(TRAINING_TEXT_BUFFER, 0x98e98c); // "Inj"
        } else {
            let condition = exe::injury_get_condition(read_usize(G_INJURY), person, 1);
            format(TRAINING_TEXT_BUFFER, 0x98e8d4, condition as i32); // "%d%%"
        }
        tel26_area_text([area, 7, row, 1, 0, 0xc, 2, job_colour(JOB_COLOUR_BDD4), text]);
    } else if read_u32(view) & 4 != 0 && read_u8(table) != 0xff {
        let change: unsafe extern "thiscall" fn(usize, usize, u32) -> i8 = core::mem::transmute(0x8b37c0usize);
        let mut k = 0u8;
        while read_u8(table + k as usize) != 0xff {
            let attribute = read_u8(table + k as usize) as u32;
            format(TRAINING_TEXT_BUFFER, 0x98806c, exe::staff_get_player_attribute(person, attribute) as i32); // "%d"
            let delta = change(read_usize(G_TRAINING_MANAGER), person, read_u8(table + k as usize) as u32);
            let ink = if delta > 0 { colours.up } else if delta < 0 { colours.down } else { colours.same };
            tel26_area_text([area, k as u32 + 3, row, 1, 0, 0xc, 2, ink, text]);
            k = k.wrapping_add(1);
        }
    }
    let value = read_u32(person + staff::VALUE);
    if club != 0 && value != 0 {
        let cash = value;
        let amount = exe::cash_get_display_amount(&cash, 5);
        exe::cash_format_money(TRAINING_TEXT_BUFFER, 0x7d0, amount, 5);
    } else {
        lang(TRAINING_TEXT_BUFFER, 0x98e614); // "Free"
    }
    tel26_area_text([area, 0xc, row, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), text]);
}

/// The squad list under the tab bar: the schedule's description (when it has a file name) and two areas, then, when
/// the squad is not empty, the players that pass the filter in a new list, the headers of the view and one row each
/// (from the end when the view is reversed). Returns the area whose scroll row is restored.
unsafe fn training_squad_list(session: usize, view: usize, squad: usize, club: usize, mut y: i32, bottom: i32, top_area: u32,
    colours: &TrainingAttributeColours) -> u32 {
    y += 0x19;
    let file = training_current_schedule(session) + TRAINING_SCHEDULE_FILE;
    if read_u8(file) != 0 {
        let copy: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(0x9466d0usize);
        copy(TRAINING_TEXT_BUFFER, file, 0x7d0);
        write_u8(TRAINING_TEXT_BUFFER + 0x7cf, 0);
        comp_text_item([0x6e, y as u32, 0x30c, (y + 0x23) as u32, 2, 0, 0xc, 3, job_colour(G_GUI_COLOUR_YELLOW), TRAINING_TEXT_BUFFER as u32, 1]);
        y += 0x28;
    }
    let mut area = tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, (y + 0x14) as u32, 9, 0, 1, 0, 1, 0, u32::MAX]);
    tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, bottom as u32, 1, 0, 1, 0, 2, 0, u32::MAX]);
    if read_u16(squad) == 0 {
        return area;
    }
    covered!("tel25_5 the squad list");
    let block = exe::crt_operator_new(0xe);
    let shown = if block != 0 {
        let construct: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
        construct(block)
    } else {
        0
    };
    write_u32(squad + 0xa, read_u32(squad + 2));
    loop {
        let node = exe::llist_get_next(squad);
        if node == 0 {
            break;
        }
        if training_passes_filter(view, read_usize(node)) {
            exe::llist_add_tail(shown, node, 5);
        }
    }
    let height = bottom - y - 0x10;
    let rows = (height / exe::font_get_height(2)) as u8;
    let row_height = ftol(height as f64 / rows as f64 + 0.5);
    let right = if (read_i16(shown) as i32) < rows as i32 { 0x30c } else { 0x2f6 };
    let general_weights: [u8; 13] = [0xa, 5, 0x1a, 0xc, 5, 0xa, 0xa, 6, 0, 0, 0, 0, 0xa];
    let attribute_weights: [u8; 13] = [0xa, 5, 0x15, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0xa];
    // the attribute group; without a group bit the original keeps the stack dword whose low byte holds the rows
    let mut table = (top_area & 0xffff_ff00) as usize | rows as usize;
    let flags = read_u32(view);
    if flags & 1 != 0 {
        let header = tel26_add_area(current_human_display(), [0x6e, (y + 8) as u32, right, (row_height + y + 7) as u32, 0xd,
            general_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
        // "Inf", "   Name", "   Position<..>", "Age", "Form", "Morale", "Cond.", "Value"
        training_list_header(header, 1, 0xc, 0x98ea64, 0x800);
        training_list_header(header, 2, 1, 0x98ea5c, 0x1000);
        training_list_header(header, 3, 1, 0xa8d0d4, 0x8000);
        training_list_header(header, 4, 0xc, 0x98e9dc, 0x100000);
        training_list_header(header, 5, 0xc, 0x98e9f8, 0x800000);
        training_list_header(header, 6, 0xc, 0x98e9f0, 0x1000000);
        training_list_header(header, 7, 0xc, 0x98e9e8, 0x2000000);
        training_list_header(header, 0xc, 0xc, 0x98ea0c, 0x10000);
        area = tel26_add_area(current_human_display(), [0x6e, (y + row_height + 8) as u32, 0x30c, (bottom - 8) as u32, 0xd,
            general_weights.as_ptr() as u32, rows.wrapping_sub(1) as u32, 0, 1, 0, u32::MAX]);
    } else if flags & 4 != 0 {
        covered!("tel25_5 the attributes view");
        let header = tel26_add_area(current_human_display(), [0x6e, (y + 8) as u32, right, (row_height + y + 7) as u32, 0xd,
            attribute_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
        training_list_header(header, 1, 0xc, 0x98ea64, 0x800);
        training_list_header(header, 2, 1, 0x98ea5c, 0x1000);
        let flags = read_u32(view);
        for (bit, group) in [(0x10u32, 0usize), (0x20, 0x30), (0x40, 0xc), (0x80, 0x18), (0x100, 0x24), (0x200, 0x3c)] {
            if flags & bit != 0 {
                table = TRAINING_ATTRIBUTE_GROUPS + group;
            }
        }
        exe::crt_qsort(table, 9, 1, 0x81ae60usize);
        let name: unsafe extern "C" fn(u32, usize, u32, u32) -> u32 = core::mem::transmute(0x5428a0usize);
        let mut k = 0u8;
        while read_u8(table + k as usize) != 0xff {
            name(read_u8(table + k as usize) as u32, TRAINING_TEXT_BUFFER, 0x7d0, 1);
            tel26_area_button([header, k as u32 + 3, 0, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY),
                TRAINING_TEXT_BUFFER as u32, 8, read_i8(table + k as usize) as i32 as u32]);
            k = k.wrapping_add(1);
        }
        while (k as i8) < 9 {
            tel26_area_text([header, k as u32 + 3, 0, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), 0x98ba0c]); // "-"
            k += 1;
        }
        training_list_header(header, 0xc, 0xc, 0x98ea0c, 0x10000);
        area = tel26_add_area(current_human_display(), [0x6e, (y + row_height + 8) as u32, 0x30c, (bottom - 8) as u32, 0xd,
            attribute_weights.as_ptr() as u32, rows.wrapping_sub(1) as u32, 0, 1, 0, u32::MAX]);
    }
    let reversed = read_u32(view + TRAINING_VIEW_REVERSED) != 0;
    write_u32(shown + 0xa, read_u32(shown + if reversed { 6 } else { 2 }));
    let current: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d2e0usize);
    let mut row = 0u32;
    loop {
        let entry = current(shown);
        if entry == 0 {
            break;
        }
        if read_u32(read_usize(TRAINING_SQUAD_VIEW) + TRAINING_VIEW_REVERSED) == 0 {
            exe::llist_get_next(shown);
        } else {
            exe::llist_get_prev(shown);
        }
        training_squad_row(session, area, row, entry, club, table, colours);
        row += 1;
    }
    if shown != 0 {
        let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
        destroy(shown);
        exe::crt_operator_delete(shown);
    }
    area
}

/// Rule: the training screen of a club: the ten schedule tabs, the Schedule, View (and Attributes), Edit or Coaches
/// and Filter menus, the current schedule's description and the squad list in the chosen view, filtered and sorted,
/// with Cancel and Ok.
///
/// Parameter 0 is the edit session; 1 and 2 the title colours (the club's kit colours,
/// `database_get_club_kit_colours(club, 1)`, when both are 0). `TRAINING_SQUAD_VIEW` points at the current schedule's
/// view. The session's squad list is rebuilt in the view's order (`0x8b0d60` with the comparer of the sort flag:
/// 0x800 status, 0x1000 name, 0x8000 position, 0x10000 value, 0x40000 attribute, 0x100000 age, 0x800000 form,
/// 0x1000000 morale, 0x2000000 injury, else unsorted). Then `0x41c8e0(6, 0)`, the title "<club> Training", the menu
/// bar (4), the tabs (`training_schedule_tabs`), a three-column area on the tab bar's line (0x6e..0x1e5) with the
/// Schedule and View menus, the Edit/Filter line (`training_edit_and_filter`) and the list
/// (`training_squad_list`, `training_squad_row`). The list area gets the view's scroll row back
/// (`Area::set_scroll_row`) and becomes parameter 5; last, Cancel (link 9) and Ok (link 0xa) in a two-column area
/// (weights 3, 1). The attribute colours: risen 0xae31aa; fallen red 0xae31a8 (yellow when colourblind); unchanged
/// yellow (0xae3152 when colourblind).
/// Original: `training_screens_draw` (0x8b5a10, training_screens.cpp, cdecl, SEH)
pub(crate) unsafe extern "C" fn training_screens_draw() {
    let mut empty_list = [0u8; 0xe];
    let construct: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    construct(empty_list.as_mut_ptr() as usize);
    let mut tabs = [0u8; 25 * TRAINING_TAB_SIZE];
    exe::crt_vector_constructor_iterator(tabs.as_mut_ptr() as usize, TRAINING_TAB_SIZE as u32, 25, 0x4568f0usize, 0x5a53c0usize);
    let colourblind = read_u8(G_PLAYER_SETUP_COLOURBLIND_FLAG) != 0;
    let colours = TrainingAttributeColours {
        up: job_colour(JOB_COLOUR_31AA),
        down: if colourblind { job_colour(G_GUI_COLOUR_YELLOW) } else { job_colour(G_GUI_COLOUR_RED) },
        same: if colourblind { job_colour(TRAINING_COLOUR_3152) } else { job_colour(G_GUI_COLOUR_YELLOW) },
    };
    let session = get_screen_param(0);
    let mut first = get_screen_param(1) as u32;
    let mut second = get_screen_param(2) as u32;
    let club = read_usize(session + TRAINING_SESSION_CLUB);
    let view_index = read_i8(session + TRAINING_SESSION_CURRENT) as i32;
    write_usize(TRAINING_SQUAD_VIEW, training_current_schedule(session) + 0x18);
    if first as u16 == 0 && second as u16 == 0 {
        covered!("tel25_5 the title colours from the kit");
        exe::database_get_club_kit_colours(club, 1, &mut first, &mut second);
        set_screen_param(1, (first & 0xffff) as usize);
        set_screen_param(2, (second & 0xffff) as usize);
    }
    let flags = read_u32(read_usize(TRAINING_SQUAD_VIEW));
    let order = [(0x800u32, 0x8bb190usize), (0x1000, 0x8bb2a0), (0x8000, 0x8bb340), (0x10000, 0x8bb460), (0x40000, 0x8bb510),
        (0x100000, 0x8bb620), (0x800000, 0x8bb6d0), (0x1000000, 0x8bb7d0), (0x2000000, 0x8bb8d0)]
        .iter().find(|&&(bit, _)| flags & bit != 0).map_or(0, |&(_, f)| f);
    let mut squad = 0usize;
    let sort: unsafe extern "thiscall" fn(usize, *mut usize, usize) = core::mem::transmute(0x8b0d60usize);
    sort(session, &mut squad, order);
    let background: unsafe extern "C" fn(u32, u32) = core::mem::transmute(0x41c8e0usize);
    background(6, 0);
    let gender = read_i8(club + club::SHORT_GENDER_NAME) as i32;
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa8d16c, gender, gender, club + club::SHORT_NAME); // "<club> Training"
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, second, first, TRAINING_TEXT_BUFFER);
    exe::menubar_build(4, 0);
    let (top, bottom) = training_schedule_tabs(session, view_index, tabs.as_mut_ptr() as usize);
    let top_area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x1e5, (top + 0x14) as u32, 3, 0, 1, 0, 1, 0, u32::MAX]);
    training_schedule_menu(session, top_area);
    let view = read_usize(TRAINING_SQUAD_VIEW);
    training_view_menu(top_area, view, read_u16(squad) == 0);
    training_edit_and_filter(session, view, top);
    let area = training_squad_list(session, view, squad, club, top, bottom, top_area, &colours);
    let display = current_human_display();
    if area as i16 != -1 {
        exe::area_set_scroll_row(display_area_at(display, area as i16), read_usize(read_usize(TRAINING_SQUAD_VIEW) + TRAINING_VIEW_SCROLL_ROW));
    }
    set_screen_param(5, area as i16 as isize as usize);
    let buttons_weights: [u8; 2] = [3, 1];
    let buttons = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, buttons_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    let (grey, pale) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY));
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0x98a1a0); // "Cancel"
    tel26_area_button([buttons, 0, 0, 0x30, grey, 0xc, 3, pale, TRAINING_TEXT_BUFFER as u32, 9, 0]);
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0x990f7c); // "Ok"
    tel26_area_button([buttons, 1, 0, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 3, job_colour(G_GUI_COLOUR_PALE_GREY), TRAINING_TEXT_BUFFER as u32, 0xa, 0]);
    exe::crt_vector_destructor_iterator(tabs.as_mut_ptr() as usize, TRAINING_TAB_SIZE as u32, 25, 0x5a53c0usize);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    destroy(empty_list.as_mut_ptr() as usize);
}

/// Comparer for `training_screens_draw`: side by side over the display list (`compare_match_screen_build`) plus what
/// the build writes or rolls outside it: the session's 50 squad entries (sorted in place), the squad view pointer,
/// the attribute groups (sorted in place), and what `update_player_training` touches for each listed player (his
/// person and player sheet, the training entries and the position training count, the contract lookup scratch).
/// The session's squad list itself is not restored: it is cleared and rebuilt (freed and new nodes) on each run.
pub(crate) unsafe extern "C" fn compare_training_screens_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    let session = get_screen_param(0);
    if session == 0 {
        return o();
    }
    let mut extra = vec![(session, 0xfa), (session + 0x818, 1), (TRAINING_SQUAD_VIEW, 4), (TRAINING_ATTRIBUTE_GROUPS, 0x48),
        (G_CONTRACT_LOOKUP_RESULT, 4)];
    let manager = read_usize(G_TRAINING_MANAGER);
    if manager != 0 {
        let entries = read_usize(manager + training_manager_fields::PLAYER_ENTRIES);
        if entries != 0 {
            extra.push((entries, read_u32(G_PLAYER_DATA_COUNT) as usize * 6));
        }
        extra.push((manager + training_manager_fields::POSITION_TRAINING_COUNT, 4));
        let items = read_usize(manager + training_manager_fields::POSITION_TRAINING_ITEMS);
        if items != 0 {
            extra.push((items, read_u32(manager + training_manager_fields::POSITION_TRAINING_COUNT) as usize * 8));
        }
    }
    for k in 0..50 {
        let person = read_usize(session + k * 5);
        if person != 0 {
            extra.push((person, staff::SIZE));
            let sheet = read_usize(person + staff::PLAYER);
            if sheet != 0 {
                extra.push((sheet, 0x46));
            }
        }
    }
    compare_match_screen_build("training screen", &extra, || o(), || training_screens_draw());
}
