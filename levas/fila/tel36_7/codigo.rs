// ---------- batch %%LEVA%%: the staff note screen (notes.cpp) ----------

/// The staff note (notes.cpp) fields read by the screen: the text at +4, the reminder date (+0x103 the day/year dword,
/// +0x107 the leap-year dword), the date it was written at +0x10b, the reason byte at +0x113.
const NOTE_TEXT: usize = 0x4;
const NOTE_REMINDER_DATE: usize = 0x103;
const NOTE_WRITTEN_DATE: usize = 0x10b;
const NOTE_REASON: usize = 0x113;

/// `Notes::<method>` on `G_NOTES` with one argument (thiscall, ret 4).
unsafe fn notes_call1(f: usize, a: usize) -> usize {
    let g: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(f);
    g(read_usize(G_NOTES), a)
}

/// `Notes::<method>` on `G_NOTES` with two arguments (thiscall, ret 8).
unsafe fn notes_call2(f: usize, a: usize, b: usize) -> usize {
    let g: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(f);
    g(read_usize(G_NOTES), a, b)
}

/// Rule: the staff note screen opens to create a note on a person, or to edit the human's note on him: the text, the
/// reason and, when the reminder is a week or four weeks after the writing, the matching reminder choice.
///
/// `notes_open_staff_note_screen(person)`: `open_screen` with the builder 0x796340, the handler 0x796b30, single
/// instance, the no-op leave routine 0x5a8f60 and 0; when it opens, the slot's human and `Notes::find_note(human,
/// person)` on `G_NOTES`. No note: `Notes::clear_edit_text(human)`, the reminder choice 0, the reason 0, not editing.
/// A note: editing, `Notes::set_edit_text(human, note text)`; the reminder choice is 0xff (a date) when
/// `Date::days_since` from the reminder date to the written date is 7 or 28, else 0; the reason is the note's byte.
/// Then the parameters: 0 the person's id, 1 today (the address of `G_CURRENT_DATE`), 2 the reminder choice, 3 the
/// reason, 4 editing (all sign-extended), 5 `Notes::get_edit_text(human)`, 6 and 7 the note's written and reminder
/// dates (0 without a note), 8 the reminder choice again.
/// Original: `notes_open_staff_note_screen` (0x7961b0, notes.cpp, cdecl).
pub(crate) unsafe extern "C" fn notes_open_staff_note_screen(person: usize) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x796340usize, 0x796b30usize, 1, 0x5a8f60usize, 0) == 0 {
        return;
    }
    covered!("tel36_7 open the staff note");
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    let human = read_usize((G_SCREEN_RECORDS_MANAGER as isize + slot * 0x300) as usize);
    let note = notes_call2(0x795750, human, person);
    let (choice, reason, editing) = if note == 0 {
        notes_call1(0x795a60, human);
        (0u8, 0u8, 0u8)
    } else {
        notes_call2(0x795a10, human, note + NOTE_TEXT);
        let days = exe::date_days_since(note + NOTE_WRITTEN_DATE, read_u32(note + NOTE_REMINDER_DATE), read_u32(note + NOTE_REMINDER_DATE + 4));
        let choice = if days == 7 || days == 0x1c { 0xff } else { 0 };
        (choice, read_u8(note + NOTE_REASON), 1)
    };
    let text = notes_call1(0x7959e0, human);
    set_screen_param(0, read_u32(person) as usize);
    set_screen_param(1, G_CURRENT_DATE);
    set_screen_param(2, choice as i8 as isize as usize);
    set_screen_param(3, reason as i8 as isize as usize);
    set_screen_param(4, editing as usize);
    set_screen_param(5, text);
    if note != 0 {
        set_screen_param(6, note + NOTE_WRITTEN_DATE);
        set_screen_param(7, note + NOTE_REMINDER_DATE);
    } else {
        set_screen_param(6, 0);
        set_screen_param(7, 0);
    }
    set_screen_param(8, choice as i8 as isize as usize);
}

/// Rule: the staff note screen shows the person in his club's colours ("Create" or "Edit Staff Note"), the sentence
/// with his link, the note's text entry, the three reminder choices and, editing, the reminder date; then the five
/// reasons (those not valid for the person greyed) and Cancel / Confirm (and Delete when editing).
///
/// `notes_draw_staff_note_screen()`: a local Text; the person (parameter 0), the reminder choice (2, a char), the
/// reason (3), editing (4, a char), the text (5), the written date (6) and the date choice (8, a char); the human of the
/// slot. The club: the person's `staff::CLUB_JOB`, else the club of `TRANSFER_RULES_DEFAULT_CLUB_KEY`. The person's
/// background picture; the title: his name (500 bytes, style 1) in the club's kit 1 colours; the subtitle "Edit Staff
/// Note" (editing) or "Create Staff Note"; rows = 0x195 / the height of font 3. Area 1 (0x6e, 0x8c)-(0x30c, 0x221),
/// seven columns (2, 20, 2, 10, 2, 4, 3), kind 2: the sentence 0xa69fec (0x64, the name) with the person's link
/// (0x7d7) drawn wrapped from row 0 (font 3, pale grey). Area 2, three columns (2, 5, 36), kind 1, from two rows below
/// (row R): "Note :" (column 1) and the text entry (flags 0x80018, row R, colour `JOB_COLOUR_BDE8`, the text, limit
/// 0xfe, id 8, in area 2). R + 2: "Reminder Required ?" in area 1. Area 3, two columns (2, 17), kind 1, from R + 4: the
/// three reminder labels (`notes_get_reminder_label` k, 2000 bytes) as buttons id 7 with data k, the chosen one
/// `JOB_COLOUR_BDEC` and the others pale grey; editing, with a written date after 1980, the long date string (1000
/// bytes) as a button id 7 with data -1 (`JOB_COLOUR_BDEC` when the choice is 0xff and the date choice is set). Two
/// rows below "Reason for note:" in area 1 and, from two rows further, the five reasons (`notes_get_reason_label` k,
/// 2000 bytes, 2) in area 3: not valid for the person (`notes_is_reason_valid_for_staff(human, person, k)`) grey with
/// id 0, otherwise id 3 with data k, the chosen one `JOB_COLOUR_BDEC`. The menu bar (4, 0) and the button area
/// (0x64, 0x22b)-(0x316, 0x24e) in the club's background colour: editing, columns 1, 4, 1 with "Delete" (id 6),
/// "Cancel" (4) and "Confirm" (5); otherwise columns 3, 1 with "Cancel" and "Confirm".
/// Original: `notes_draw_staff_note_screen` (0x796340, notes.cpp, cdecl).
pub(crate) unsafe extern "C" fn notes_draw_staff_note_screen() {
    let text = tel36_langlib();
    let columns: [u8; 7] = [2, 0x14, 2, 0xa, 2, 4, 3];
    let three: [u8; 3] = [1, 4, 1];
    let weights: [u8; 7] = [2, 0x11, 3, 1, 2, 5, 0x24];
    let mut tx = [0u32; 8];
    let t = tx.as_mut_ptr() as usize;
    exe::new_text(tx.as_mut_ptr() as *mut usize, 0usize);
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    let human = read_usize((G_SCREEN_RECORDS_MANAGER as isize + slot * 0x300) as usize);
    let person = tel36_staff_param(0);
    let choice = get_screen_param(2) as u8;
    let reason = get_screen_param(3) as u32;
    let editing = get_screen_param(4) as u8;
    let note_text = get_screen_param(5);
    let written = get_screen_param(6);
    let date_choice = get_screen_param(8) as u8;
    covered!("tel36_7 draw the staff note");
    let own = read_usize(person + staff::CLUB_JOB);
    let club = if own != 0 {
        own
    } else {
        read_usize(G_CLUB_TABLE).wrapping_add(read_u32(TRANSFER_RULES_DEFAULT_CLUB_KEY).wrapping_mul(club::SIZE as u32) as usize)
    };
    let background: unsafe extern "C" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x41c790usize);
    background(person, 0, 0);
    let (mut fore, mut back) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut fore, &mut back);
    let mut name = [0u8; 0x1f4];
    exe::database_format_staff_name(person, name.as_mut_ptr() as usize, 0x1f4, 1);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, name.as_ptr() as usize);
    text(JOB_TEXT, if editing != 0 { 0xa6a058 } else { 0xa6a044 });
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let rows = (0x195 / exe::font_get_height(3)) as u8 as u32;
    let pale = job_colour(JOB_COLOUR_3184);
    let first = tel26_add_area(current_human_display(), [0x6e, 0x8c, 0x30c, 0x221, 7, columns.as_ptr() as u32, rows, 0, 2, 0, u32::MAX]);
    text(JOB_TEXT, 0xa69fec, 0x64, name.as_ptr());
    exe::text_set_string(t, JOB_TEXT as *const u8);
    exe::text_add_link(t, name.as_ptr(), 0x7d7, person);
    let mut row = 0i32;
    tel35_text_draw_wrapped(t, first, 0x6e, 0x30c, &mut row, 3, pale, 0, 0);
    row += 2;
    let second = tel26_add_area(current_human_display(), [0x6e, 0x8c, 0x30c, 0x221, 3, weights.as_ptr().wrapping_add(4) as u32, rows, 0, 1, 0, u32::MAX]);
    text(JOB_TEXT, 0xa69fe4);
    tel26_area_text([second, 1, row as u32, 1, 0, 1, 3, pale, JOB_TEXT as u32]);
    tel26_add_item(current_human_display(), [0x80018, 0, 0, 0, 0, 2, row as u32, 1, 0, 0, 1, 3, job_colour(JOB_COLOUR_BDE8),
        note_text as u32, 0xfe, 8, 0, second]);
    row += 2;
    text(JOB_TEXT, 0xa69fd0);
    tel26_area_button([first, 1, row as u32, 1, 0, 1, 3, pale, JOB_TEXT as u32, 0, 0]);
    row += 2;
    let third = tel26_add_area(current_human_display(), [0x6e, 0x8c, 0x30c, 0x221, 2, weights.as_ptr() as u32, rows, 0, 1, 0, u32::MAX]);
    let reminder_label: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x795a90usize);
    for k in 0..3u8 {
        reminder_label(read_usize(G_NOTES), JOB_TEXT, 0x7d0, k as u32);
        let colour = if choice != k { pale } else { job_colour(JOB_COLOUR_BDEC) };
        tel26_area_button([third, 1, row as u32, 1, 0, 1, 3, colour, JOB_TEXT as u32, 7, k as u32]);
        row += 1;
    }
    if written != 0 && editing != 0 && read_u16(written + 2) > 0x7bc {
        covered!("tel36_7 staff note reminder date");
        exe::date_get_long_date_string(written, JOB_TEXT, 0x3e8, 0xff);
        let colour = if choice == 0xff && date_choice != 0 { job_colour(JOB_COLOUR_BDEC) } else { pale };
        tel26_area_button([third, 1, row as u32, 1, 0, 1, 3, colour, JOB_TEXT as u32, 7, u32::MAX]);
    }
    row += 2;
    text(JOB_TEXT, 0xa69fbc);
    tel26_area_button([first, 1, row as u32, 1, 0, 1, 3, pale, JOB_TEXT as u32, 0, 0]);
    row += 2;
    let reason_label: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x795b90usize);
    let valid: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u8 = core::mem::transmute(0x796100usize);
    for k in 0..5u8 {
        reason_label(read_usize(G_NOTES), JOB_TEXT, 0x7d0, k as u32, 2);
        if valid(read_usize(G_NOTES), human, person, k as u32) == 0 {
            tel26_area_button([third, 1, row as u32, 1, 0, 1, 3, job_colour(JOB_COLOUR_317E), JOB_TEXT as u32, 0, 0]);
        } else {
            let colour = if reason != k as u32 { pale } else { job_colour(JOB_COLOUR_BDEC) };
            tel26_area_button([third, 1, row as u32, 1, 0, 1, 3, colour, JOB_TEXT as u32, 3, k as u32]);
        }
        row += 1;
    }
    exe::menubar_build(4, 0);
    let grey = job_colour(JOB_COLOUR_317E);
    if editing != 0 {
        let bar = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 3, three.as_ptr() as u32, 1, 0, 1, back, u32::MAX]);
        text(JOB_TEXT, 0xa69fb4);
        tel26_area_button([bar, 0, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 6, 0]);
        text(JOB_TEXT, 0x98a1a0);
        tel26_area_button([bar, 1, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 4, 0]);
        text(JOB_TEXT, 0x98a198);
        tel26_area_button([bar, 2, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 5, 0]);
    } else {
        let bar = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, weights.as_ptr().wrapping_add(2) as u32, 1, 0, 1, back, u32::MAX]);
        text(JOB_TEXT, 0x98a1a0);
        tel26_area_button([bar, 0, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 4, 0]);
        text(JOB_TEXT, 0x98a198);
        tel26_area_button([bar, 1, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 5, 0]);
    }
    exe::drop_text(tx.as_mut_ptr() as *mut usize);
}

/// Rule: on the staff note screen a reminder or reason click chooses it (rebuilding the screen); Confirm saves the
/// note (a new one, or the edit, with the reminder dates when a date was chosen); Delete removes it; Cancel closes.
///
/// `notes_staff_note_screen_handler(item)`: the slot's human, the person (parameter 0), the choice (2), the reason
/// (3), editing (4), the text (5) and the two dates (6, 7). In a network game the pending command argument is cleared
/// and -9 comes back. By the widget's id: 7 (a reminder): parameter 2 = the data's low byte, sign-extended, -11; 3 (a
/// reason): parameter 3 the same, -11; 8 (the text entry): 0; 5 (Confirm): editing, with the choice 0xff and both dates,
/// `Notes::update_note_with_dates(human, person, text, &written, &reminder, reason)` (0x795710, copies of the dates);
/// editing otherwise `Notes::update_note(human, person, text, choice, reason)` (0x7956d0); not editing
/// `Notes::add_note(human, person, text, choice, reason)` (0x7953d0); -9. 6 (Delete): `Notes::delete_note(human,
/// person)` (0x795660), -9. 4 (Cancel): -9. Otherwise the menu bar's command, then its object link.
/// Original: `notes_staff_note_screen_handler` (0x796b30, notes.cpp, cdecl).
pub(crate) unsafe extern "C" fn notes_staff_note_screen_handler(raw: u32) -> i32 {
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    let human = read_usize((G_SCREEN_RECORDS_MANAGER as isize + slot * 0x300) as usize);
    let person = tel36_staff_param(0);
    let choice = get_screen_param(2) as u8 as u32;
    let reason = get_screen_param(3) as u8 as u32;
    let editing = get_screen_param(4) as u8;
    let note_text = get_screen_param(5);
    let written = get_screen_param(6);
    let reminder = get_screen_param(7);
    if read_u32(TEL35_NETWORK_FLAG) != 0 {
        write_u32(G_PENDING_COMMAND_ARG, 0);
        return -9;
    }
    let item = raw as i16;
    match tel26_item_id(item) {
        7 => {
            covered!("tel36_7 staff note reminder choice");
            set_screen_param(2, tel26_item_value(item) as u8 as i8 as isize as usize);
            -11
        }
        3 => {
            covered!("tel36_7 staff note reason");
            set_screen_param(3, tel26_item_value(item) as u8 as i8 as isize as usize);
            -11
        }
        8 => 0,
        5 => {
            covered!("tel36_7 staff note confirm");
            let notes = read_usize(G_NOTES);
            if editing != 0 {
                if choice == 0xff && written != 0 && reminder != 0 {
                    let first = [read_u32(written), read_u32(written + 4)];
                    let second = [read_u32(reminder), read_u32(reminder + 4)];
                    let save: unsafe extern "thiscall" fn(usize, usize, usize, usize, *const u32, *const u32, u32) = core::mem::transmute(0x795710usize);
                    save(notes, human, person, note_text, first.as_ptr(), second.as_ptr(), reason);
                } else {
                    let save: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32) = core::mem::transmute(0x7956d0usize);
                    save(notes, human, person, note_text, choice, reason);
                }
            } else {
                let add: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32) = core::mem::transmute(0x7953d0usize);
                add(notes, human, person, note_text, choice, reason);
            }
            -9
        }
        6 => {
            notes_call2(0x795660, human, person);
            -9
        }
        4 => -9,
        _ => manager_menu_bar(raw),
    }
}

/// Comparer for `notes_open_staff_note_screen`.
pub(crate) unsafe extern "C" fn compare_notes_open_staff_note_screen(person: usize) {
    // simplification: opening a screen (and setting the edit text) does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize)>(%%I0%%)(person)
}

/// Comparer for `notes_draw_staff_note_screen`: the screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_notes_draw_staff_note_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    compare_by_display_list_with("staff note screen (notes_draw_staff_note_screen)", &[], || o(), || notes_draw_staff_note_screen());
}

/// Comparer for `notes_staff_note_screen_handler`: the reminder and reason choices side by side over the screen
/// params; saving, deleting and the menu bar only the original.
pub(crate) unsafe extern "C" fn compare_notes_staff_note_screen_handler(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%);
    if read_u32(TEL35_NETWORK_FLAG) != 0 || !matches!(tel26_item_id(item as i16), 3 | 7 | 8) {
        // simplification: saving or deleting the note and the menu bar commands do not run twice; proven in Rust mode
        return o(item);
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&screen_param_regions(), || o(item), || notes_staff_note_screen_handler(item));
    SCOREBOARD.record(same && r == n, || format!("staff note screen choice: original={} rust={} differences {:?}", r, n, first_differences(&mo, &mr)));
    r
}
