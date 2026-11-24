// --- sx16_2: the staff notes of the human managers (notes.cpp: add, remove, replace, find, format, edit text, labels) ---

/// Exe routines of the staff notes without an `exe::` wrapper.
pub(crate) mod notes_fns {
    pub(crate) const ADD_NOTE: usize = 0x7953d0; // Notes::add_note, thiscall(this, manager, staff, text, reminder, reason), ret 0x14
    pub(crate) const ADD_NOTE_DATED: usize = 0x795550; // Notes::add_note_dated, thiscall(this, manager, staff, text, reminder date, written date, reason), ret 0x18
    pub(crate) const REMOVE_NOTE: usize = 0x795660; // Notes::remove_note, thiscall(this, manager, staff), ret 8
    pub(crate) const NUMERIC_DATE_STRING: usize = 0x54a470; // Date::get_numeric_date_string, thiscall(date, out, size, time of day), ret 0xc
}

/// The source path of notes.cpp in the "Error" box ("E:\dev\CM3\cm3 00-01\cm3\code\notes.cpp").
const NOTES_CPP_PATH: usize = 0xa69a18;
/// The size of a staff note record (the LLIST item): staff id, text, written date, reminder date, reason.
const NOTE_SIZE: usize = 0x114;
/// The note's written date (+0x103). `NOTE_REMINDER_DATE` (notes.rs) names the same offset, and `NOTE_WRITTEN_DATE`
/// the reminder date at +0x10b (`NOTE_DUE_DATE`): the screen reads them crossed, as the original screen does.
const NOTE_WRITTEN_ON: usize = 0x103;
/// The note edit text buffers: 255 bytes per manager slot from +0x80 of the notes object.
const NOTES_EDIT_TEXT: usize = 0x80;
const NOTES_EDIT_TEXT_SIZE: usize = 0xff;

/// The human manager slot (0..15) of a manager: the human rows are the last 16 of the staff table, so the slot is
/// `G_STAFF_COUNT` − id − 1; anything else is no slot.
unsafe fn notes_slot(manager: usize) -> Option<usize> {
    let k = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(manager)).wrapping_sub(1);
    (0..0x10).contains(&k).then_some(k as usize)
}

/// The note list of a manager slot (the first LLIST of the slot's 8 bytes).
unsafe fn notes_list(this: usize, slot: usize) -> usize {
    read_usize(this + slot * 8)
}

/// Walks a slot's note list as the original does: the cursor back to the head, then up to the list's count (read again
/// on each pass) `LLIST::get_next`, stopping at an empty answer; the note whose staff id matches, or 0.
unsafe fn notes_walk(list: usize, staff_id: u32) -> usize {
    write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
    let mut k = 0i32;
    while k < read_i16(list + llist::COUNT) as i32 {
        let note = exe::llist_get_next(list);
        if note == 0 {
            return 0;
        }
        if read_u32(note) == staff_id {
            return note;
        }
        k += 1;
    }
    0
}

/// Builds a note record: the staff id, the text (`strncpy` of 255 bytes, the last byte +0x102 zeroed), the written
/// date, the reminder date (both by `Date::operator=`) and the reason byte.
unsafe fn notes_build_note(rec: &mut [u8; NOTE_SIZE], staff_id: u32, text: usize, written: usize, reminder: usize, reason: u8) {
    let r = rec.as_mut_ptr() as usize;
    write_u32(r, staff_id);
    exe::at_9466d0(r + NOTE_TEXT, text as *const u8, 0xff);
    write_u8(r + 0x102, 0); // the text's last byte
    let mut scratch = [0u8; 8];
    exe::date_operator_3d(r + NOTE_WRITTEN_ON, scratch.as_mut_ptr() as usize, written);
    write_u8(r + NOTE_REASON, reason);
    exe::date_operator_3d(r + NOTE_DUE_DATE, scratch.as_mut_ptr() as usize, reminder);
}

/// The note record `notes_add_note` appends: written today; the reminder in one week (choice 1), four weeks (2,
/// `Date::add_weeks` of today) or, for anything else, the "no reminder" date 23 July 1971 (`Date::Date(23, 7, 1971,
/// -1)`).
unsafe fn notes_note_for_today(rec: &mut [u8; NOTE_SIZE], staff_id: u32, text: usize, reminder: i8, reason: u8) {
    let mut later = [0u8; 8];
    let mut none = [0u16; 4];
    let due = match reminder {
        1 => exe::date_add_weeks(G_CURRENT_DATE, &mut later, 1),
        2 => exe::date_add_weeks(G_CURRENT_DATE, &mut later, 4),
        _ => exe::date_date_549f70(&mut none, 0x17, 7, 0x7b3, u32::MAX),
    };
    notes_build_note(rec, staff_id, text, G_CURRENT_DATE, due, reason);
}

/// Appends a staff note for a human manager, written today, with the reminder by the choice (one week, four weeks or
/// none, `notes_note_for_today`) and the reason; a manager outside the 16 human slots gets nothing. The note is
/// copied into the slot's list (`LLIST::add_tail`, 0x114 bytes).
/// Original: `Notes::add_note` (0x7953d0, notes.cpp, thiscall, ret 0x14; SEH frame for the local dates, no
/// destructor that does anything).
pub(crate) unsafe extern "thiscall" fn notes_add_note(this: usize, manager: usize, staff: usize, text: usize, reminder: u32, reason: u32) {
    let Some(slot) = notes_slot(manager) else { return };
    covered!("sx16_2 add a note");
    let mut rec = [0u8; NOTE_SIZE];
    notes_note_for_today(&mut rec, read_u32(staff), text, reminder as i8, reason as u8);
    exe::llist_add_tail(notes_list(this, slot), rec.as_ptr() as usize, NOTE_SIZE as u32);
}

/// Compares the note the original appended (the list's tail) with the record the copy builds.
unsafe fn notes_compare_appended(this: usize, manager: usize, count_before: Option<i16>, expected: &[u8; NOTE_SIZE], what: &str) {
    let Some(slot) = notes_slot(manager) else { return };
    let list = notes_list(this, slot);
    let tail = read_usize(list + llist::TAIL);
    let got = if tail != 0 { core::slice::from_raw_parts(read_usize(tail + llist_node::DATA) as *const u8, NOTE_SIZE) } else { &[] };
    let grew = count_before.map(|c| read_i16(list + llist::COUNT) == c + 1).unwrap_or(false);
    SCOREBOARD.record(grew && got == &expected[..], || format!("{}: slot {} appended {:x?} expected {:x?}", what, slot, &got[..got.len().min(8)], &expected[..8]));
}

/// Comparer for `notes_add_note`: the original appends; the copy builds the same record (`notes_note_for_today`), which
/// must equal the list's new tail.
pub(crate) unsafe extern "thiscall" fn compare_notes_add_note(this: usize, manager: usize, staff: usize, text: usize, reminder: u32, reason: u32) {
    let before = notes_slot(manager).map(|k| read_i16(notes_list(this, k) + llist::COUNT));
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32)>(%%I0%%)(this, manager, staff, text, reminder, reason);
    if before.is_some() {
        let mut rec = [0u8; NOTE_SIZE];
        notes_note_for_today(&mut rec, read_u32(staff), text, reminder as i8, reason as u8);
        notes_compare_appended(this, manager, before, &rec, "Notes::add_note");
    }
}

/// Appends a staff note for a human manager with the given reminder and written dates (an edited note keeps its own
/// dates) and the reason; nothing outside the 16 human slots.
/// Original: `Notes::add_note_dated` (0x795550, notes.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn notes_add_note_dated(this: usize, manager: usize, staff: usize, text: usize, reminder: usize, written: usize, reason: u32) {
    let Some(slot) = notes_slot(manager) else { return };
    covered!("sx16_2 add a dated note");
    let mut rec = [0u8; NOTE_SIZE];
    notes_build_note(&mut rec, read_u32(staff), text, written, reminder, reason as u8);
    exe::llist_add_tail(notes_list(this, slot), rec.as_ptr() as usize, NOTE_SIZE as u32);
}

/// Comparer for `notes_add_note_dated`: as `compare_notes_add_note`.
pub(crate) unsafe extern "thiscall" fn compare_notes_add_note_dated(this: usize, manager: usize, staff: usize, text: usize, reminder: usize, written: usize, reason: u32) {
    let before = notes_slot(manager).map(|k| read_i16(notes_list(this, k) + llist::COUNT));
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32)>(%%I1%%)(this, manager, staff, text, reminder, written, reason);
    if before.is_some() {
        let mut rec = [0u8; NOTE_SIZE];
        notes_build_note(&mut rec, read_u32(staff), text, written, reminder, reason as u8);
        notes_compare_appended(this, manager, before, &rec, "Notes::add_note_dated");
    }
}

/// Removes a human manager's note about a staff member: the first note of the slot's list with his id
/// (`notes_walk`) goes (`LLIST::remove`); nothing outside the 16 slots or without such a note. Runs on every finished
/// transfer (`finalise_transfer`) for the buying manager.
/// Original: `Notes::remove_note` (0x795660, notes.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn notes_remove_note(this: usize, manager: usize, staff: usize) {
    let Some(slot) = notes_slot(manager) else { return };
    let list = notes_list(this, slot);
    let note = notes_walk(list, read_u32(staff));
    if note != 0 {
        covered!("sx16_2 remove a note");
        exe::llist_remove(list, note);
    }
}

/// Comparer for `notes_remove_note`: it frees the note, so only the original runs; the copy's walk, run before it,
/// says whether a note goes, and the list's count must drop by exactly that.
pub(crate) unsafe extern "thiscall" fn compare_notes_remove_note(this: usize, manager: usize, staff: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%);
    let Some(slot) = notes_slot(manager) else { return o(this, manager, staff) };
    let list = notes_list(this, slot);
    let before = read_i16(list + llist::COUNT);
    let found = (notes_walk(list, read_u32(staff)) != 0) as i16;
    o(this, manager, staff);
    let after = read_i16(list + llist::COUNT);
    SCOREBOARD.record(after == before - found, || format!("Notes::remove_note(slot {}, staff {}): count {} -> {}, the copy expected {} removed", slot, read_u32(staff), before, after, found));
}

/// Replaces a manager's note about a staff member: the old note removed (`Notes::remove_note`) and a new one added
/// dated today (`Notes::add_note` with the text, the reminder choice and the reason), both through the exe.
/// Original: `Notes::replace_note` (0x7956d0, notes.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn notes_replace_note(this: usize, manager: usize, staff: usize, text: usize, reminder: u32, reason: u32) {
    let remove: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(notes_fns::REMOVE_NOTE);
    let add: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32) = core::mem::transmute(notes_fns::ADD_NOTE);
    remove(this, manager, staff);
    add(this, manager, staff, text, reminder, reason);
}

/// Comparer for `notes_replace_note`: only the original (the remove and the add compare themselves).
pub(crate) unsafe extern "thiscall" fn compare_notes_replace_note(this: usize, manager: usize, staff: usize, text: usize, reminder: u32, reason: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32)>(%%I3%%)(this, manager, staff, text, reminder, reason)
}

/// Replaces a manager's note about a staff member keeping given dates: `Notes::remove_note`, then
/// `Notes::add_note_dated` with the text, the reminder and written dates and the reason.
/// Original: `Notes::replace_note_dated` (0x795710, notes.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn notes_replace_note_dated(this: usize, manager: usize, staff: usize, text: usize, reminder: usize, written: usize, reason: u32) {
    let remove: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(notes_fns::REMOVE_NOTE);
    let add: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32) = core::mem::transmute(notes_fns::ADD_NOTE_DATED);
    remove(this, manager, staff);
    add(this, manager, staff, text, reminder, written, reason);
}

/// Comparer for `notes_replace_note_dated`: only the original.
pub(crate) unsafe extern "thiscall" fn compare_notes_replace_note_dated(this: usize, manager: usize, staff: usize, text: usize, reminder: usize, written: usize, reason: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32)>(%%I4%%)(this, manager, staff, text, reminder, written, reason)
}

/// A human manager's note about a staff member (`notes_walk` of the slot's list), or 0 (also outside the 16 slots).
/// Original: `Notes::find_note` (0x795750, notes.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn notes_find_note(this: usize, manager: usize, staff: usize) -> usize {
    match notes_slot(manager) {
        Some(slot) => notes_walk(notes_list(this, slot), read_u32(staff)),
        None => 0,
    }
}

/// The 14-byte LLIST header of a manager's note list (the cursor moves), or nothing outside the slots.
unsafe fn notes_list_region(this: usize, manager: usize) -> Vec<(usize, usize)> {
    notes_slot(manager).map(|k| vec![(notes_list(this, k), 0xe)]).unwrap_or_default()
}

/// Comparer for `notes_find_note`: side by side over the list header; the same note.
pub(crate) unsafe extern "thiscall" fn compare_notes_find_note(this: usize, manager: usize, staff: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(%%I5%%);
    let (r, n, same, _, _) = crate::comparison::side_by_side(&notes_list_region(this, manager), || o(this, manager, staff), || notes_find_note(this, manager, staff));
    SCOREBOARD.record(same && r == n, || format!("Notes::find_note(staff {}): original={:#x} rust={:#x}", read_u32(staff), r, n));
    r
}

/// Writes a manager's note about a staff member into `out`, with the reason label and the reminder date when set.
///
/// Nothing (0) without `out` or outside the 16 slots; without the note `out` is emptied and 0 returned. The reason
/// label (style 2, `Notes::get_reason_label` through the exe, 100 bytes) when the reason is set; the reminder text
/// "Reminder: <date>" (`langlib_sprintf` 0xa69a64 of `Date::get_numeric_date_string`, 100 bytes, no time) unless the
/// reminder date is the "no reminder" 23 July 1971 (day and year compared). Then: no label and no reminder, the text
/// (`strncpy` of 255 bytes, byte 0xfe zeroed); a label only, "(label) text"; a reminder only, "text (reminder)"; both,
/// "(label) text (reminder)" (`sprintf`). Returns `out`.
/// Original: `Notes::format_note_text` (0x7957b0, notes.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn notes_format_note_text(this: usize, manager: usize, staff: usize, out: usize) -> usize {
    if out == 0 {
        return 0;
    }
    let Some(slot) = notes_slot(manager) else { return 0 };
    let note = notes_walk(notes_list(this, slot), read_u32(staff));
    if note == 0 {
        write_u8(out, 0);
        return 0;
    }
    covered!("sx16_2 format a note");
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let date_string: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(notes_fns::NUMERIC_DATE_STRING);
    let mut label = [0u8; 0xc8];
    let reason = read_u8(note + NOTE_REASON);
    if reason != 0 {
        exe::notes_get_reason_label(this, label.as_mut_ptr() as usize, 0x64, reason as u32, 2);
    }
    let mut none = [0u16; 4];
    let d = exe::date_date_549f70(&mut none, 0x17, 7, 0x7b3, u32::MAX);
    let due = note + NOTE_DUE_DATE;
    let mut reminder = [0u8; 0xc0];
    if read_u16(d + 2) != read_u16(due + 2) || read_u16(d) != read_u16(due) {
        let mut date = [0u8; 0x64];
        date_string(due, date.as_mut_ptr() as usize, 0x64, 0xff);
        format(reminder.as_mut_ptr() as usize, 0xa69a64, date.as_ptr()); // "Reminder: <date>"
    }
    let text = note + NOTE_TEXT;
    match (label[0] != 0, reminder[0] != 0) {
        (false, false) => {
            exe::at_9466d0(out, text as *const u8, 0xff);
            write_u8(out + 0xfe, 0);
        }
        (true, false) => {
            sprintf(out, 0xa69a5c, label.as_ptr(), text); // "(%s) %s"
        }
        (false, true) => {
            sprintf(out, 0x9a3d8c, text, reminder.as_ptr()); // "%s (%s)"
        }
        (true, true) => {
            sprintf(out, 0xa69a4c, label.as_ptr(), text, reminder.as_ptr()); // "(%s) %s (%s)"
        }
    }
    out
}

/// Comparer for `notes_format_note_text`: side by side over the output (512 bytes) and the list header.
pub(crate) unsafe extern "thiscall" fn compare_notes_format_note_text(this: usize, manager: usize, staff: usize, out: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> usize>(%%I6%%);
    if out == 0 {
        return o(this, manager, staff, out);
    }
    let mut regions = notes_list_region(this, manager);
    regions.push((out, 0x200));
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, manager, staff, out), || notes_format_note_text(this, manager, staff, out));
    SCOREBOARD.record(same && r == n, || format!("Notes::format_note_text(staff {}): differences {:?}", read_u32(staff), first_differences(&mo, &mr)));
    r
}

/// The 255-byte note edit buffer of a human manager's slot (+0x80 + slot × 255), or 0 outside the 16 slots.
/// Original: `Notes::get_edit_text` (0x7959e0, notes.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn notes_get_edit_text(this: usize, manager: usize) -> usize {
    notes_slot(manager).map(|k| this + NOTES_EDIT_TEXT + k * NOTES_EDIT_TEXT_SIZE).unwrap_or(0)
}

/// Comparer for `notes_get_edit_text`: the same buffer.
pub(crate) unsafe extern "thiscall" fn compare_notes_get_edit_text(this: usize, manager: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I7%%)(this, manager);
    let n = notes_get_edit_text(this, manager);
    SCOREBOARD.record(r == n, || format!("Notes::get_edit_text: original={:#x} rust={:#x}", r, n));
    r
}

/// Copies an existing note's text into the slot's edit buffer (`strncpy` of 255 bytes, the last byte zeroed);
/// nothing outside the 16 slots.
/// Original: `Notes::set_edit_text` (0x795a10, notes.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn notes_set_edit_text(this: usize, manager: usize, text: usize) {
    let buf = notes_get_edit_text(this, manager);
    if buf != 0 {
        exe::at_9466d0(buf, text as *const u8, 0xff);
        write_u8(buf + 0xfe, 0);
    }
}

/// Comparer for `notes_set_edit_text`: side by side over the 16 edit buffers.
pub(crate) unsafe extern "thiscall" fn compare_notes_set_edit_text(this: usize, manager: usize, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I8%%);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(this + NOTES_EDIT_TEXT, 0x10 * NOTES_EDIT_TEXT_SIZE)], || o(this, manager, text), || notes_set_edit_text(this, manager, text));
    SCOREBOARD.record(same, || format!("Notes::set_edit_text: differences {:?}", first_differences(&mo, &mr)));
}

/// Empties the slot's edit buffer (its first byte) before a new note is written; nothing outside the 16 slots.
/// Original: `Notes::clear_edit_text` (0x795a60, notes.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn notes_clear_edit_text(this: usize, manager: usize) {
    let buf = notes_get_edit_text(this, manager);
    if buf != 0 {
        write_u8(buf, 0);
    }
}

/// Comparer for `notes_clear_edit_text`: side by side over the 16 edit buffers.
pub(crate) unsafe extern "thiscall" fn compare_notes_clear_edit_text(this: usize, manager: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I9%%);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(this + NOTES_EDIT_TEXT, 0x10 * NOTES_EDIT_TEXT_SIZE)], || o(this, manager), || notes_clear_edit_text(this, manager));
    SCOREBOARD.record(same, || format!("Notes::clear_edit_text: differences {:?}", first_differences(&mo, &mr)));
}

/// The notes.cpp "Error" box of a label routine without a buffer (`row_missing`) or with one under 50 bytes
/// (`row_short`); true when the box was shown.
unsafe fn notes_label_buffer_error(out: usize, size: i32, row_missing: u32, row_short: u32) -> bool {
    if out == 0 {
        show_error_box(NOTES_CPP_PATH, row_missing);
        return true;
    }
    if size < 0x32 {
        show_error_box(NOTES_CPP_PATH, row_short);
        return true;
    }
    false
}

/// Writes the label of a note reminder choice: 0 "No Reminder", 1 "One Week", 2 "One Month" (`langlib_sprintf`);
/// any other choice writes nothing. Returns `out`; without a buffer (box line 0x2db) or with one under 50 bytes (line
/// 0x2e1), the notes.cpp "Error" box and 0.
/// Original: `notes_get_reminder_label` (0x795a90, notes.cpp, thiscall, ret 0xc; `this` unused).
pub(crate) unsafe extern "thiscall" fn notes_get_reminder_label(_this: usize, out: usize, size: i32, choice: u32) -> usize {
    if notes_label_buffer_error(out, size, 0x2db, 0x2e1) {
        return 0;
    }
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let text = match choice as i8 {
        0 => 0xa69a8c, // "No Reminder"
        1 => 0xa69a98, // "One Week"
        2 => 0xa69aa4, // "One Month"
        _ => return out,
    };
    format(out, text);
    out
}

/// Comparer for `notes_get_reminder_label`: side by side over the buffer (the size given, at most 512 bytes); the
/// error boxes only through the original.
pub(crate) unsafe extern "thiscall" fn compare_notes_get_reminder_label(this: usize, out: usize, size: i32, choice: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> usize>(%%I10%%);
    if out == 0 || size < 0x32 {
        return o(this, out, size, choice);
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(out, (size as usize).min(0x200))], || o(this, out, size, choice), || notes_get_reminder_label(this, out, size, choice));
    SCOREBOARD.record(same && r == n, || format!("notes_get_reminder_label({}): differences {:?}", choice as i8, first_differences(&mo, &mr)));
    r
}

/// The reason labels by style, reasons 1..4 (promising youngster, transfer prospect, contract renewal, squad member),
/// then the text for any other reason; the short tags and "None" take the empty argument (`EMPTY_STRING`).
const NOTE_REASON_PLAIN: [usize; 5] = [0xa69c98, 0xa69c84, 0xa69c70, 0xa69cac, 0xa69c38];
const NOTE_REASON_REMINDER_TITLE: [usize; 5] = [0xa69bf8, 0xa69bd4, 0xa69bb4, 0xa69c1c, 0xa69b9c];
const NOTE_REASON_TAG: [usize; 5] = [0xa69b44, 0xa69b10, 0xa69adc, 0xa69b74, 0xa69ab0];

/// Writes the label of a note reason by style (`langlib_sprintf`): style 2 the plain name ("Promising Youngster",
/// "Transfer Prospect", "Contract Renewal", "Squad Member", else "None"); style 0 the reminder news title ("Reminder
/// about ..." or "Reminder about Player"); any other style the short tag ("Young", "Prosp", "Cont.", "Squad", else
/// "None"). "None" and the tags take the empty argument. Returns `out`; the error boxes as the reminder label (lines
/// 0x302 and 0x308).
/// Original: `notes_get_reason_label` (0x795b90, notes.cpp, thiscall, ret 0x10; `this` unused).
pub(crate) unsafe extern "thiscall" fn notes_get_reason_label(_this: usize, out: usize, size: i32, reason: u32, style: u32) -> usize {
    if notes_label_buffer_error(out, size, 0x302, 0x308) {
        return 0;
    }
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let k = (reason as i8 as i32 - 1) as u32;
    let pick = |texts: &[usize; 5]| texts[k.min(4) as usize];
    match style as u8 {
        2 => format(out, pick(&NOTE_REASON_PLAIN), EMPTY_STRING),
        0 => format(out, pick(&NOTE_REASON_REMINDER_TITLE)),
        _ => format(out, pick(&NOTE_REASON_TAG), EMPTY_STRING),
    };
    out
}

/// Comparer for `notes_get_reason_label`: as the reminder label's.
pub(crate) unsafe extern "thiscall" fn compare_notes_get_reason_label(this: usize, out: usize, size: i32, reason: u32, style: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32, u32) -> usize>(%%I11%%);
    if out == 0 || size < 0x32 {
        return o(this, out, size, reason, style);
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(out, (size as usize).min(0x200))], || o(this, out, size, reason, style), || notes_get_reason_label(this, out, size, reason, style));
    SCOREBOARD.record(same && r == n, || format!("notes_get_reason_label({}, style {}): differences {:?}", reason as i8, style as u8, first_differences(&mo, &mr)));
    r
}
