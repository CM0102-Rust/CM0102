// ---------- batch %%LEVA%%: the competition history screen (comp_screens.cpp, part 1): opening it, building it (the tab bar and the Records tab) and the Winners tab ----------

/// The ids of the two competitions whose history columns change: the Olympics (Gold, Silver, Bronze) and the one whose
/// third column is "Minor Premiers" [inference: the Australian NSL] (dwords of the exe).
const COMP_HISTORY_OLYMPICS_ID: usize = 0x9cf7a0;
const COMP_HISTORY_MINOR_PREMIERS_ID: usize = 0x9cf89c;
/// The two flags the history builder writes outside the screen (1 on entry; [0xadae0c] 0 on entry and 1 once the
/// Records tab has its View popup) [inference: the screen's "built" marks, read by the menu bar].
const COMP_HISTORY_BUILT_FLAG: usize = 0x99c00c;
const COMP_HISTORY_POPUP_FLAG: usize = 0xadae0c;
/// The tabs of other items (tel23), called by address: Attendances, Results, Sequences, Teams, Players.
const COMP_HISTORY_TEL23_TABS: [usize; 5] = [0x499900, 0x499e10, 0x49a320, 0x49a830, 0x49ad40];

/// The 26 parameters 0..0x19 of the history screen as `comp_screens_open_history_screen` leaves them (`prev` = the
/// previous screen's 60 params, read before opening, used with `restore`).
unsafe fn comp_history_planned_params(comp: usize, restore: u32, prev: &[usize; 0x3c]) -> [usize; 0x1a] {
    let mut p = [0usize; 0x1a];
    p[0] = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(comp) as usize * 4);
    p[1] = 1;
    p[2] = usize::MAX;
    p[3] = usize::MAX;
    p[6] = 1;
    p[0xb] = 6;
    p[0xe] = 9;
    p[0x11] = 0xb;
    p[0x14] = 3;
    p[0x17] = 0xf;
    if restore != 0 {
        for k in [1usize, 6, 0xb, 0xe, 0x11, 0x14, 0x17] {
            p[k] = prev[k];
        }
    }
    p
}

/// Opens the history screen of a competition (builder `comp_screens_draw_history_screen`, handler 0x49b250, leave
/// 0x49b7e0). `comp` is the competition record: param 0 is its object (`G_COMP_OBJECT_TABLE[comp id]`). The params:
/// 1 the tab (1 Winners), 2 and 3 the first and last row of the Winners table in the club history table (−1: not
/// searched yet), 6 the Records view 1, 0xb, 0xe, 0x11, 0x14 and 0x17 the views of the Attendances, Results,
/// Sequences, Teams and Players tabs (6, 9, 0xb, 3, 0xf), the rest of 0..0x19 zero. With `restore`, params 1, 6, 0xb,
/// 0xe, 0x11, 0x14 and 0x17 come back from the previous screen (its 60 params read before opening).
/// Original: `comp_screens_open_history_screen` (0x498780, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_open_history_screen(comp: usize, restore: u32) {
    covered!("tel22_5 open comp history");
    let mut prev = [0usize; 0x3c];
    if restore != 0 {
        for (k, v) in prev.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
    }
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(comp) as usize * 4);
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x498a30, 0x49b250, 0, 0x49b7e0, 0) == 0 {
        return;
    }
    let p = comp_history_planned_params(comp, 0, &prev);
    set_screen_param(0, object);
    for k in 1..0x1a {
        set_screen_param(k as u32, p[k]);
    }
    if restore != 0 {
        for k in [1u32, 6, 0xb, 0xe, 0x11, 0x14, 0x17] {
            set_screen_param(k, prev[k as usize]);
        }
    }
}

/// The Records tab of the history screen (inline in the builder): the View button (period popup with the two periods
/// of `comp_stats_get_record_period_label`, the chosen one, param 6, ticked and copied without its leading space as
/// the heading), the heading, and one row per record type 1..0x16 that `Comp_Stats::is_record_type_shown`: its title
/// ("  %s", a button with id 4 and the type as value) and its text (`Comp_Stats::get_record_text` wrapped from x1 + 8
/// to 0x2ee in font 1, style 2); empty rows fill the visible ones; the scroll row from param 7, the area to param 8.
/// Kept from the original: with a view other than 1 and 2 the heading is stack garbage there (empty here).
/// Original: part of `comp_screens_draw_history_screen` (0x498a30, comp_screens.cpp).
unsafe fn comp_history_records_tab(object: usize, top: i32, bottom: i32) {
    let menu_item: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(0x415640usize);
    let ticked: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(0x456890usize);
    let period_label: unsafe extern "thiscall" fn(usize, u32, *mut u8, u32) = core::mem::transmute(0x4a5010usize);
    let title: unsafe extern "thiscall" fn(usize, u32, *mut u8, u32) = core::mem::transmute(0x4a5120usize);
    let shown: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a54c0usize);
    let record_text: unsafe extern "thiscall" fn(usize, u32, u32, *mut usize) -> u32 = core::mem::transmute(0x4a5530usize);
    let draw_wrapped: unsafe extern "thiscall" fn(*mut usize, u32, i32, i32, *mut i32, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x4aeaf0usize);
    let scroll: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x46ad90usize);
    let weights: [u8; 2] = [1, 2];
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0usize);
    let view = get_screen_param(6) as u8;
    comp_lang_format()(JOB_TEXT, 0x988004); // "View"
    let button = tel26_add_button([0x6e, top as u32, 0xeb, (top + 0x14) as u32, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1,
                                   job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 0, 0]);
    let popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), button]);
    write_u32(COMP_HISTORY_POPUP_FLAG, 1);
    let mut heading = [0u8; 0x7d0];
    let mut label = [0u8; 0xc8];
    let stats = read_usize(object + 8);
    for k in 1u32..=2 {
        period_label(stats, k, label.as_mut_ptr(), 0xc8);
        comp_crt_format()(JOB_TEXT, 0x98ed14, label.as_ptr()); // " %s"
        if k == view as i8 as i32 as u32 {
            ticked(G_SCREEN_MANAGER, popup, JOB_TEXT, 3, k);
            tel26_copy_text(heading.as_mut_ptr(), (JOB_TEXT + 1) as *const u8);
        } else {
            menu_item(G_SCREEN_MANAGER, popup, JOB_TEXT, 3, k);
        }
    }
    let top = top + 0x19;
    comp_text_item([0x6e, top as u32, 0x30c, (top + 0x23) as u32, 2, 0, 0xc, 3, job_colour(G_GUI_COLOUR_YELLOW), heading.as_ptr() as u32, 1]);
    let top = top + 0x28;
    let rows = ((bottom - top) / exe::font_get_height(2)) as u8;
    let area = comp_add_area([0x6e, top as u32, 0x30c, bottom as u32, 2, weights.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    let (w1, w2) = (weights[0] as i32, weights[1] as i32);
    let x = w1 * 0x288 / (w1 + w2) + 0x6e;
    let mut row: i32 = 0;
    for kind in 1u32..=0x16 {
        if shown(stats, kind) == 0 {
            continue;
        }
        title(stats, kind, heading.as_mut_ptr(), 0x7d0);
        comp_crt_format()(JOB_TEXT, 0x987ff0, heading.as_ptr()); // "  %s"
        tel26_area_button([area, 0, row as u32, 2, 0, 1, 2, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 4, kind]);
        record_text(stats, view as u32, kind, t);
        tel26_area_text([area, 1, row as u32, 2, 0, 1, 1, 0, 0]);
        draw_wrapped(t, area, x + 8, 0x2ee, &mut row, 1, job_colour(G_GUI_COLOUR_YELLOW), 2, 0);
        row += 1;
    }
    while row < rows as i32 {
        tel26_area_text([area, 0, row as u32, 2, 0, 1, 2, job_colour(G_GUI_COLOUR_PALE_GREY), 0]);
        tel26_area_text([area, 1, row as u32, 2, 0, 1, 1, job_colour(G_GUI_COLOUR_PALE_GREY), 0]);
        row += 1;
    }
    scroll(current_human_display(), area, get_screen_param(7));
    set_screen_param(8, area as u16 as i16 as isize as usize);
    exe::drop_text(t);
}

/// Builds the history screen of the competition object in param 0 (none: the comp_screens.cpp 5412 box): the section
/// picture 8, the title "<competition> History" in the competition's colours, the menu bar, the tab bar (Winners, when
/// the competition's history count, its virtual slot 0x50, is positive; Records, Attendances, Results, Sequences,
/// Teams, Players: kind 2 with the values 1..7) on param 1, then the open tab: 1 `comp_screens_draw_history_winners_tab`,
/// 2 the Records tab (`comp_history_records_tab`), 3..7 the tabs at 0x499900, 0x499e10, 0x49a320, 0x49a830 and
/// 0x49ad40 (called by address), each with the object and the tab bar's top and bottom; then the related
/// competitions menu (0x49d070, with the colours, 1, 0xa) and the Back/Next buttons. Writes 1 to [0x99c00c] and 0 to
/// [0xadae0c] first.
/// Original: `comp_screens_draw_history_screen` (0x498a30, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_history_screen() {
    // the 25 MenuItem slots of the tab bar (MenuItem::MenuItem: word +0 = −1, word +2 = 0; text at +4, kind +0xcc,
    // value +0xd0)
    let mut items = [[0u8; 0xd4]; 0x19];
    for it in items.iter_mut() {
        it[0] = 0xff;
        it[1] = 0xff;
    }
    write_u32(COMP_HISTORY_BUILT_FLAG, 1);
    write_u32(COMP_HISTORY_POPUP_FLAG, 0);
    let object = get_screen_param(0);
    if object == 0 {
        comp_screens_error_box(0x1524);
        return;
    }
    covered!("tel22_5 build comp history");
    let tab = get_screen_param(1) as u8 as i8;
    let section_picture: unsafe extern "C" fn(u32, u32) = core::mem::transmute(0x41c8e0usize);
    section_picture(8, 0);
    let comp = read_usize(object + 4);
    let (mut c28, mut c2c) = (0u32, 0u32);
    exe::database_get_comp_colours(comp, &mut c28, &mut c2c);
    let gender = read_u8(comp + 0x52) as i8 as i32;
    comp_lang_format()(JOB_TEXT, 0x99c764, gender, gender, comp + 0x38); // "{}<%s - competition>{} History"
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, c2c, c28, JOB_TEXT);
    exe::menubar_build(4, 0);
    let history_count: unsafe extern "thiscall" fn(usize) -> i8 = core::mem::transmute(read_usize(read_usize(object) + 0x50));
    let mut n = 0usize;
    let mut add = |items: &mut [[u8; 0xd4]; 0x19], fmt: usize, value: u32| {
        comp_lang_format()(items[n].as_mut_ptr().add(4) as usize, fmt);
        items[n][0xcc..0xd0].copy_from_slice(&2u32.to_le_bytes());
        items[n][0xd0..0xd4].copy_from_slice(&value.to_le_bytes());
        n += 1;
    };
    if history_count(object) > 0 {
        add(&mut items, 0x99b7d0, 1); // "Winners"
    }
    add(&mut items, 0x990ab4, 2); // "Records"
    add(&mut items, 0x990a9c, 3); // "Attendances"
    add(&mut items, 0x990a94, 4); // "Results"
    add(&mut items, 0x990a88, 5); // "Sequences"
    add(&mut items, 0x99c75c, 6); // "Teams"
    add(&mut items, 0x990a80, 7); // "Players"
    let (mut top, mut bottom) = (0x50i32, 0x221i32);
    exe::gui_add_tab_bar(items.as_mut_ptr() as usize, n as u32, tab as i32, &mut top, &mut bottom, 1);
    match tab {
        1 => comp_screens_draw_history_winners_tab(object, top, bottom),
        2 => comp_history_records_tab(object, top, bottom),
        3..=7 => {
            let f: unsafe extern "C" fn(usize, i32, i32) = core::mem::transmute(COMP_HISTORY_TEL23_TABS[tab as usize - 3]);
            f(object, top, bottom);
        }
        _ => {}
    }
    let related: unsafe extern "C" fn(usize, u32, u32, u32, u32) = core::mem::transmute(0x49d070usize);
    related(object, c2c, c28, 1, 0xa);
    tel26_back_next();
}

/// The Winners tab of the history screen: the competition's rows of the club history table (param 2 the first, 3 the
/// last; −1 at first: the table is sorted, `crt_qsort` by 0x540780, and searched for rows of this competition, +4 its
/// record, then both written). The columns: the year, the winner, the runner-up, a third column for competitions of
/// history kind 3 (the virtual slot 0x50; "Third Place", or "Minor Premiers" for the competition [0x9cf89c]) and a
/// Hosts column when the competition has hosts (slot 0x88): kind 2 with hosts 4 columns (weights 3,8,8,8, hosts in 3),
/// kind 2 without 3 (1,4,4), otherwise with hosts 5 (2,4,4,4,4, hosts in 4) or 4 (3,8,8,8); the Olympics ([0x9cf7a0])
/// head Gold, Silver, Bronze. Each club: "  name", yellow when it is a human's, a button (0x7d5, the club) when its
/// +0x53 record has +0x71, else text; the hosts in colour 0xaebdd4. The scroll row from param 4; the area to param 5.
/// Original: `comp_screens_draw_history_winners_tab` (0x4991f0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_history_winners_tab(object: usize, top: i32, bottom: i32) {
    let weights_3: [u8; 3] = [1, 4, 4];
    let weights_4: [u8; 4] = [3, 8, 8, 8];
    let weights_5: [u8; 5] = [2, 4, 4, 4, 4];
    let mut first = get_screen_param(2) as i32;
    let mut last = get_screen_param(3) as i32;
    if first == -1 {
        covered!("tel22_5 winners rows searched");
        exe::crt_qsort(read_usize(G_CLUB_COMP_HISTORY_TABLE), read_u32(G_CLUB_COMP_HISTORY_COUNT), 0x1a, 0x540780usize);
        let comp = read_u32(object + 4);
        let count = read_i32(G_CLUB_COMP_HISTORY_COUNT);
        for i in 0..count {
            if read_u32(read_usize(G_CLUB_COMP_HISTORY_TABLE) + i as usize * 0x1a + 4) == comp {
                if first == -1 {
                    first = i;
                }
                last = i;
            }
        }
        set_screen_param(2, first as isize as usize);
        set_screen_param(3, last as isize as usize);
    }
    let winner_rows = ((bottom - top - 0x10) / exe::font_get_height(3)) as u8;
    let vt = read_usize(object);
    let kind_of: unsafe extern "thiscall" fn(usize) -> i8 = core::mem::transmute(read_usize(vt + 0x50));
    let has_hosts: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(vt + 0x88));
    let kind = kind_of(object);
    let (columns, weights, style, hosts): (u8, usize, u32, i8) = if kind == 2 {
        if has_hosts(object) != 0 { (4, weights_4.as_ptr() as usize, 3, 3) } else { (3, weights_3.as_ptr() as usize, 3, -1) }
    } else if has_hosts(object) != 0 {
        (5, weights_5.as_ptr() as usize, 2, 4)
    } else {
        (4, weights_4.as_ptr() as usize, 3, -1)
    };
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, columns as u32, weights as u32,
                                                        winner_rows as u32, 0, 2, 0, u32::MAX]);
    let heading = |col: u32| tel26_area_text([area, col, 0, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32]);
    let comp_id = read_u32(read_usize(object + 4));
    if comp_id == read_u32(COMP_HISTORY_OLYMPICS_ID) {
        comp_lang_format()(JOB_TEXT, 0x99c80c, 0xad9c64); // "Gold"
        heading(1);
        comp_lang_format()(JOB_TEXT, 0x99c7dc, 0xad9c64); // "Silver"
        heading(2);
        comp_lang_format()(JOB_TEXT, 0x99c7ac, 0xad9c64); // "Bronze"
        heading(3);
    } else {
        comp_lang_format()(JOB_TEXT, 0x99b7d0); // "Winners"
        heading(1);
        comp_lang_format()(JOB_TEXT, 0x99c7a0); // "Runners-up"
        heading(2);
        if kind == 3 {
            let third = if comp_id == read_u32(COMP_HISTORY_MINOR_PREMIERS_ID) { 0x99c790 } else { 0x99c784 };
            comp_lang_format()(JOB_TEXT, third); // "Minor Premiers" / "Third Place"
            heading(3);
        }
    }
    if hosts != -1 {
        comp_lang_format()(JOB_TEXT, 0x990fc0); // "Hosts"
        heading(hosts as i32 as u32);
    }
    // one club cell: "  name", a button when the club's +0x53 record has +0x71 (id 0x7d5, the club), else text
    let club_cell = |club: usize, col: u32, row: u32, colour: u32| {
        comp_crt_format()(JOB_TEXT, 0x987ff0, club + 0x38); // "  %s"
        let rec = read_usize(club + 0x53);
        if rec != 0 && read_u32(rec + 0x71) != 0 {
            tel26_area_button([area, col, row, 1, 0, 1, style, colour, JOB_TEXT as u32, 0x7d5, club as u32]);
        } else {
            tel26_area_text([area, col, row, 1, 0, 1, style, colour, JOB_TEXT as u32]);
        }
    };
    let human_colour = |club: usize| {
        if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 { job_colour(G_GUI_COLOUR_YELLOW) } else { job_colour(G_GUI_COLOUR_PALE_GREY) }
    };
    if first > -1 && first <= last {
        let mut row = 1u32;
        for i in first..=last {
            let entry = read_usize(G_CLUB_COMP_HISTORY_TABLE) + i as usize * 0x1a;
            comp_crt_format()(JOB_TEXT, 0x98806c, read_i16(entry + 8) as i32); // "%d"
            tel26_area_text([area, 0, row, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32]);
            for (off, col) in [(0xa, 1u32), (0xe, 2)] {
                let club = read_usize(entry + off);
                if club != 0 {
                    club_cell(club, col, row, human_colour(club));
                }
            }
            if kind >= 3 {
                let club = read_usize(entry + 0x12);
                if club != 0 {
                    club_cell(club, 3, row, human_colour(club));
                }
            }
            if hosts != -1 {
                let club = read_usize(entry + 0x16);
                if club != 0 {
                    club_cell(club, hosts as i32 as u32, row, job_colour(JOB_COLOUR_BDD4));
                }
            }
            row += 1;
        }
    }
    let scroll = get_screen_param(4);
    if area as u16 != 0xffff {
        exe::area_set_scroll_row(current_human_display().wrapping_add(4).wrapping_add((area as i16 as isize as usize).wrapping_mul(0xbf1)), scroll);
    }
    set_screen_param(5, area as i16 as isize as usize);
}

/// Comparer for `comp_screens_open_history_screen`: opening only through the original (it opens the screen), the
/// params 0..0x19 checked against the plan (`comp_history_planned_params`).
pub(crate) unsafe extern "C" fn compare_comp_screens_open_history_screen(comp: usize, restore: u32) {
    let mut prev = [0usize; 0x3c];
    if restore != 0 {
        for (k, v) in prev.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
    }
    let plan = comp_history_planned_params(comp, restore, &prev);
    original_fn::<unsafe extern "C" fn(usize, u32)>(%%I0%%)(comp, restore);
    if comp != 0 && get_screen_param(0) == plan[0] && plan[0] != 0 {
        let mut got = [0usize; 0x1a];
        for (k, v) in got.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
        SCOREBOARD.record(got == plan, || format!("comp history screen (open {:#x}): params {:x?} x plan {:x?}", comp, got, plan));
    }
}

/// Comparer for `comp_screens_draw_history_screen`: side by side over the display list (`compare_match_screen_build`)
/// plus the two flags, for the Records tab and for the Winners tab once its rows are known (param 2 ≠ −1).
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_history_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    let tab = get_screen_param(1) as u8;
    // simplification: without the object (the error box), the first Winners build (it sorts the global club history
    // table and writes params 2 and 3) and the tel23 tabs (their lists are built and kept in the params on the first
    // build; they have their own comparers) only the original runs
    if get_screen_param(0) == 0 || !(tab == 2 || (tab == 1 && get_screen_param(2) as i32 != -1)) {
        return o();
    }
    compare_match_screen_build("comp history screen", &[(COMP_HISTORY_BUILT_FLAG, 4), (COMP_HISTORY_POPUP_FLAG, 4)], || o(),
        || comp_screens_draw_history_screen());
}

/// Comparer for `comp_screens_draw_history_winners_tab`: side by side over the display list once the rows are known.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_history_winners_tab(object: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(%%I2%%);
    // simplification: the first build sorts the global club history table (qsort, not stable: twice could reorder
    // equal rows) and writes params 2 and 3: only the original
    if object == 0 || get_screen_param(2) as i32 == -1 {
        return o(object, top, bottom);
    }
    compare_match_screen_build("comp history winners tab", &[], || o(object, top, bottom), || comp_screens_draw_history_winners_tab(object, top, bottom));
}
