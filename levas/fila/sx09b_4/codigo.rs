// --- sx09b_4: the ban texts of the person screens (discipline.cpp) ---

/// Returns the person's served bans from the ban history file, or 0 for a person outside the table.
///
/// `get_ban_history(id)`: the person `staff_at(id)`; when the record's address is not null and the id stored in it is
/// at or above `G_STAFF_COUNT`, 0. Otherwise the nodes between the person's `discipline_record::HISTORY_START` and
/// `discipline_record::HISTORY_LAST` in the history file (`exe::file_llist_get_nodes` on
/// `discipline_fields::BAN_HISTORY`).
/// Original: `Discipline::get_ban_history` (0x55cbc0, discipline.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_ban_history(this: usize, id: u32) -> usize {
    let person = staff_at(id as i32 as isize);
    if !person.is_null() && read_i32(person.addr()) >= read_i32(G_STAFF_COUNT) {
        covered!("sx09b_4 ban history of a person outside the table");
        return 0;
    }
    let r = read_usize(this + discipline_fields::PERSON_RECORDS).wrapping_add((id as usize).wrapping_mul(0x1d)); // the 29-byte person record
    exe::file_llist_get_nodes(read_usize(this + discipline_fields::BAN_HISTORY), read_u32(r + discipline_record::HISTORY_START), read_u32(r + discipline_record::HISTORY_LAST))
}

/// Describes a ban in one line: "N match <scope> ban", or the length and the scope, with the start date when it is
/// still to come.
///
/// `describe_ban(ban, out, size)`: a `size` below 50 gives the "Error" box (line 0x1403). The scope by
/// `get_scope_text` (through its address: the ban's rules, a 100-byte buffer, bit 1 of `.flags()` and form 4), which
/// also gives the form `g`. When the start date is after today (`exe::date_operator_3e` with `G_CURRENT_DATE`), the
/// date as text (`exe::date_get_short_date_string`, 50 bytes, 0xff) and: one match "1 match <scope> ban ( starts
/// <date> )" (0x9c3400), more matches 0x9c3398, a ban by days (`.matches()` ≤ 0) the length
/// (`exe::format_duration_text` of `.days()`, 30 bytes) in 0x9c332c. Otherwise one match "1 match <scope> ban"
/// (0x9c3300), more matches "<n> match <scope> ban" (0x9c32c0), by days "<length> <scope> ban" (0x9c327c). The
/// arguments go in the original's order, with the 100 it pushes between them.
/// Original: `Discipline::describe_ban` (0x55cc20, discipline.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn describe_ban(this: usize, ban: usize, out: usize, size: i32) {
    if size < 0x32 {
        show_error_at(0x9b8f18, 0x1403);
        return;
    }
    let ban = BanRecord(ban);
    let mut scope = [0u8; 0x64];
    let sp = scope.as_mut_ptr() as usize;
    let scope_text: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, u32) -> u8 = core::mem::transmute(FN_DISCIPLINE_GET_SCOPE_TEXT);
    let g = scope_text(this, ban.addr() + ban_record::RULES, sp, 0x64, (ban.flags() as u32 >> 1) & 1, 4) as i8 as i32; // sx09b_4: the form
    let matches = ban.matches() as i8 as i32;
    let mut length = [0u8; 0x1e];
    let lp = length.as_mut_ptr() as usize;
    let start = ban.addr() + ban_record::START_DATE;
    if exe::date_operator_3e(start, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP)) != 0 {
        let mut date = [0u8; 0x32];
        let dp = date.as_mut_ptr() as usize;
        exe::date_get_short_date_string(start, dp, 0x32, 0xff);
        if matches == 1 {
            covered!("sx09b_4 one match ban to come");
            story_lang!(out, 0x9c3400, g, g, 0x64, sp, dp); // "1 match <scope> ban ( starts <date> )"
        } else if matches > 0 {
            story_lang!(out, 0x9c3398, g, g, 0x64, matches, sp, dp);
        } else {
            covered!("sx09b_4 ban by days to come");
            exe::format_duration_text(ban.days() as i32, lp, 0x1e, 0);
            story_lang!(out, 0x9c332c, 0x64, g, g, lp, sp, dp);
        }
    } else if matches == 1 {
        covered!("sx09b_4 one match ban");
        story_lang!(out, 0x9c3300, g, g, sp); // "1 match <scope> ban"
    } else if matches > 0 {
        covered!("sx09b_4 ban of matches");
        story_lang!(out, 0x9c32c0, matches, g, g, matches, sp); // sx09b_4: "<n> match <scope> ban"
    } else {
        covered!("sx09b_4 ban by days");
        exe::format_duration_text(ban.days() as i32, lp, 0x1e, 0);
        story_lang!(out, 0x9c327c, 0x64, g, g, lp, sp); // "<length> <scope> ban"
    }
}

/// Writes the reason of a ban: "Use of an illegal substance" (reason 5), "Invalid passport" (6), "<n> points" (0),
/// "Yellow card" or "<n> yellow cards" (1), "<n> successive yellow cards" (4), "Red card" or "<n> red cards" (2),
/// "Misconduct" (3), "Unprofessional Behaviour" (any other); n is `.reason_amount()`.
///
/// `get_reason_text(ban, out, size)`: a `size` below 15 gives the "Error" box (line 0x150e).
/// Original: `Discipline::get_reason_text` (0x55d0a0, discipline.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn get_reason_text(_this: usize, ban: usize, out: usize, size: i32) {
    if size < 0xf {
        show_error_at(0x9b8f18, 0x150e);
        return;
    }
    let ban = BanRecord(ban);
    let n = ban.reason_amount() as i32;
    match ban.reason() {
        5 => story_lang!(out, 0x9c3568), // "Use of an illegal substance"
        6 => story_lang!(out, 0x9c3554), // "Invalid passport"
        0 => story_lang!(out, 0x9c3534, n), // "<n> points"
        1 if n == 1 => story_lang!(out, 0x9c3528), // sx09b_4: "Yellow card"
        1 => story_lang!(out, 0x9c3504, n), // "<n> yellow cards"
        4 => story_lang!(out, 0x9c34d4, n), // "<n> successive yellow cards"
        2 if n == 1 => story_lang!(out, 0x9c34c8), // "Red card"
        2 => story_lang!(out, 0x9c34a8, n), // "<n> red cards"
        3 => story_lang!(out, 0x9c349c), // "Misconduct"
        _ => story_lang!(out, 0x9c3480), // "Unprofessional Behaviour"
    }
}

/// Comparer for `get_ban_history`: the nodes come from the history file (a new list each time), so only the original
/// runs; the copy's plan (0 for a person outside the table) is checked against its result.
pub(crate) unsafe extern "thiscall" fn compare_get_ban_history(this: usize, id: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I0%%)(this, id);
    let person = staff_at(id as i32 as isize);
    let outside = !person.is_null() && read_i32(person.addr()) >= read_i32(G_STAFF_COUNT);
    SCOREBOARD.record(!outside || r == 0, || format!("ban history screen: get_ban_history(id {}) outside the table, original={:#x}", id, r));
    r
}

/// Comparer for `describe_ban`: side by side over the output buffer. A short buffer (the "Error" box) runs the
/// original only.
pub(crate) unsafe extern "thiscall" fn compare_describe_ban(this: usize, ban: usize, out: usize, size: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32)>(%%I1%%);
    if size < 0x32 {
        return o(this, ban, out, size);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(out, size as usize)], || o(this, ban, out, size), || describe_ban(this, ban, out, size));
    SCOREBOARD.record(same, || format!("ban history screen: describe_ban(ban {:#x}) {:?}", ban, first_differences(&mo, &mr)));
}

/// Comparer for `get_reason_text`: side by side over the output buffer. A short buffer (the "Error" box) runs the
/// original only.
pub(crate) unsafe extern "thiscall" fn compare_get_reason_text(this: usize, ban: usize, out: usize, size: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32)>(%%I2%%);
    if size < 0xf {
        return o(this, ban, out, size);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(out, size as usize)], || o(this, ban, out, size), || get_reason_text(this, ban, out, size));
    SCOREBOARD.record(same, || format!("person screen: get_reason_text(ban {:#x}, reason {}) {:?}", ban, read_u8(ban + ban_record::REASON), first_differences(&mo, &mr)));
}
