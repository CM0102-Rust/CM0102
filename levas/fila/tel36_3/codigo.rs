// ---------- batch %%LEVA%%: the physio report screen (physio.cpp) ----------

/// Rule: the physio report screen opens for a player and a physio.
///
/// `physio_report_screen_open(person, physio)`: nothing when either is missing. `open_screen` with the builder
/// 0x7a2ab0, the handler 0x7a2f10, single instance, the no-op leave routine 0x5a8f60 and 0; when it opens, parameter 0
/// = the player's id and 1 = the physio's id.
/// Original: `physio_report_screen_open` (0x7a2a50, physio.cpp, cdecl).
pub(crate) unsafe extern "C" fn physio_report_screen_open(person: usize, physio: usize) {
    if person == 0 || physio == 0 {
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x7a2ab0usize, 0x7a2f10usize, 1, 0x5a8f60usize, 0) == 0 {
        return;
    }
    covered!("tel36_3 open the physio report");
    set_screen_param(0, read_u32(person) as usize);
    set_screen_param(1, read_u32(physio) as usize);
}

/// Rule: the physio report screen names the player in his club's colours and, with one physio at the club, asks to
/// confirm the report on the player by that physio; with several, it lists them (the chosen one highlighted) to
/// pick one. Cancel / Confirm below.
///
/// `physio_report_screen_build()`: a local Text; the player of parameter 0 and the physio of parameter 1; with no club
/// (`staff::CLUB_JOB`) nothing is built. The club's physios: the three `club::PHYSIOS` slots, those with a
/// `staff::NON_PLAYER` record, in order. The player's background picture (`background_set_staff_picture`, 0, 0); the
/// title: his name (500 bytes, style 1) in the club's kit 1 colours; the subtitle "Request Physio Report"; rows =
/// 0x195 / the height of font 3. With exactly one physio: the text 0xa6b264 with the physio's name (parameter 1, not the
/// list) and the player's (150 bytes each) as a text area (0x6e, 0x8c)-(0x30c, 0x221) (font 3, pale grey). Otherwise:
/// area (0x6e, 0x8c)-(0x30c, 0x221), seven columns (2, 20, 2, 10, 2, 4, 3), kind 2: the sentence 0xa6b20c with the
/// player's name and his link (0x7d7) drawn wrapped from row 0 (font 3, pale grey); a second area on the same
/// rectangle, three columns (2, 17, 10), kind 1: from two rows below, one button per physio in column 1 (id 1, the
/// physio's id as data, his name in 150 bytes; yellow-ish `JOB_COLOUR_BDEC` for the physio of parameter 1, grey
/// otherwise). Then the menu bar (4, 0) and the button area (0x64, 0x22b)-(0x316, 0x24e) (columns 3, 1; the club's
/// background colour) with "Cancel" (id 2) and "Confirm" (id 3).
/// Original: `physio_report_screen_build` (0x7a2ab0, physio.cpp, cdecl).
pub(crate) unsafe extern "C" fn physio_report_screen_build() {
    let text = tel36_langlib();
    let columns: [u8; 7] = [2, 0x14, 2, 0xa, 2, 4, 3];
    let buttons: [u8; 5] = [3, 1, 2, 0x11, 0xa];
    let mut tx = [0u32; 8];
    let t = tx.as_mut_ptr() as usize;
    exe::new_text(tx.as_mut_ptr() as *mut usize, 0usize);
    let person = tel36_staff_param(0);
    let chosen = tel36_staff_param(1);
    let club = read_usize(person + staff::CLUB_JOB);
    if club == 0 {
        exe::drop_text(tx.as_mut_ptr() as *mut usize);
        return;
    }
    covered!("tel36_3 build the physio report");
    let mut physios = [0usize; 3];
    let mut n = 0usize;
    for k in 0..3 {
        let p = read_usize(club + club::PHYSIOS + 4 * k);
        if p != 0 && read_usize(p + staff::NON_PLAYER) != 0 {
            physios[n] = p;
            n += 1;
        }
    }
    let background: unsafe extern "C" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x41c790usize);
    background(person, 0, 0);
    let (mut fore, mut back) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut fore, &mut back);
    let mut name = [0u8; 0x1f4];
    exe::database_format_staff_name(person, name.as_mut_ptr() as usize, 0x1f4, 1);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, name.as_ptr() as usize);
    text(JOB_TEXT, 0xa6b2cc);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let rows = (0x195 / exe::font_get_height(3)) as u8;
    let pale = job_colour(JOB_COLOUR_3184);
    if n == 1 {
        covered!("tel36_3 one physio");
        let mut physio_name = [0u8; 0x96];
        exe::database_format_staff_name(chosen, physio_name.as_mut_ptr() as usize, 0x96, 1);
        exe::database_format_staff_name(person, name.as_mut_ptr() as usize, 0x96, 1);
        text(JOB_TEXT, 0xa6b264, physio_name.as_ptr(), name.as_ptr());
        tel26_text_area([0x6e, 0x8c, 0x30c, 0x221, 2, 0, rows as i8 as i32 as u32, 3, pale, JOB_TEXT as u32, 0]);
    } else {
        let d = current_human_display();
        let area = tel26_add_area(d, [0x6e, 0x8c, 0x30c, 0x221, 7, columns.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
        text(JOB_TEXT, 0xa6b20c, name.as_ptr());
        exe::text_set_string(t, JOB_TEXT as *const u8);
        exe::text_add_link(t, name.as_ptr(), 0x7d7, person);
        let mut line = 0i32;
        tel35_text_draw_wrapped(t, area, 0x6e, 0x30c, &mut line, 3, pale, 0, 0);
        let mut row = line + 2;
        let d = current_human_display();
        let list = tel26_add_area(d, [0x6e, 0x8c, 0x30c, 0x221, 3, buttons.as_ptr().wrapping_add(2) as u32, rows as u32, 0, 1, 0, u32::MAX]);
        for &p in &physios[..n] {
            exe::database_format_staff_name(p, name.as_mut_ptr() as usize, 0x96, 1);
            let colour = if p == chosen { job_colour(JOB_COLOUR_BDEC) } else { job_colour(JOB_COLOUR_317E) };
            tel26_area_button([list, 1, row as u32, 1, 0, 1, 3, colour, name.as_ptr() as u32, 1, read_u32(p)]);
            row += 1;
        }
    }
    exe::menubar_build(4, 0);
    let d = current_human_display();
    let bar = tel26_add_area(d, [0x64, 0x22b, 0x316, 0x24e, 2, buttons.as_ptr() as u32, 1, 0, 1, back, u32::MAX]);
    text(JOB_TEXT, 0x98a1a0);
    tel26_area_button([bar, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, pale, JOB_TEXT as u32, 2, 0]);
    text(JOB_TEXT, 0x98a198);
    tel26_area_button([bar, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, pale, JOB_TEXT as u32, 3, 0]);
    exe::drop_text(tx.as_mut_ptr() as *mut usize);
}

/// Rule: on the physio report screen a physio click chooses him (and the screen is rebuilt), Confirm files the report
/// news by the chosen physio on the player and closes, Cancel closes.
///
/// `physio_report_screen_handle_click(item)`: in a network game (`TEL35_NETWORK_FLAG`) the pending command argument is
/// cleared and -9 comes back. The widget's id (`tel26_item_id`): 1 (a physio): parameter 1 = the widget's data
/// (`tel26_item_value`), -11. 3 (Confirm): `physio_file_report_news(physio, player)` (0x7a1a90, on 0xb74344), -9. 2
/// (Cancel): -9. Otherwise the menu bar's command, then its object link.
/// Original: `physio_report_screen_handle_click` (0x7a2f10, physio.cpp, cdecl).
pub(crate) unsafe extern "C" fn physio_report_screen_handle_click(raw: u32) -> i32 {
    let person = tel36_staff_param(0);
    let physio = tel36_staff_param(1);
    if read_u32(TEL35_NETWORK_FLAG) != 0 {
        write_u32(G_PENDING_COMMAND_ARG, 0);
        return -9;
    }
    let item = raw as i16;
    match tel26_item_id(item) {
        1 => {
            covered!("tel36_3 choose a physio");
            set_screen_param(1, tel26_item_value(item) as usize);
            -11
        }
        3 => {
            covered!("tel36_3 file the physio report");
            let report: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x7a1a90usize);
            report(0xb74344, physio, person);
            -9
        }
        2 => -9,
        _ => manager_menu_bar(raw),
    }
}

/// Comparer for `physio_report_screen_open`.
pub(crate) unsafe extern "C" fn compare_physio_report_screen_open(person: usize, physio: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize, usize)>(%%I0%%)(person, physio)
}

/// Comparer for `physio_report_screen_build`: the screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_physio_report_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    compare_by_display_list_with("physio report screen (physio_report_screen_build)", &[], || o(), || physio_report_screen_build());
}

/// Comparer for `physio_report_screen_handle_click`: the physio choice side by side over the screen params; the
/// report and the menu bar only the original.
pub(crate) unsafe extern "C" fn compare_physio_report_screen_handle_click(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%);
    if read_u32(TEL35_NETWORK_FLAG) != 0 || tel26_item_id(item as i16) != 1 {
        // simplification: the report news and the menu bar commands do not run twice; proven in Rust mode
        return o(item);
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&screen_param_regions(), || o(item), || physio_report_screen_handle_click(item));
    SCOREBOARD.record(same && r == n, || format!("physio report screen choice: original={} rust={} differences {:?}", r, n, first_differences(&mo, &mr)));
    r
}
