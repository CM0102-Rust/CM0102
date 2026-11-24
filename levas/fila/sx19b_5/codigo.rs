// ---------- sx19b_5: the ban line of a person under a ruling body (ruling_body/ruling_body.cpp) ----------

/// Ban texts of `RulingBody::get_ban_text` (langlib formats in .rdata).
const BAN_TEXT_ONE_MATCH_FROM: usize = 0xa7abdc; // "1 match ... ban from <date> (<reason>)"
const BAN_TEXT_MATCHES_FROM: usize = 0xa7ab4c; // "<n> match ... ban from <date> (<reason>)"
const BAN_TEXT_DAYS_FROM: usize = 0xa7aab8; // "<time length> ... ban from <date> (<reason>)"
const BAN_TEXT_ONE_MATCH: usize = 0xa7aa64; // "1 match <scope> ban (<reason>)"
const BAN_TEXT_MATCHES: usize = 0xa7a9fc; // "<n> match <scope> ban (<reason>)"
const BAN_TEXT_DAYS: usize = 0xa7a990; // "<time length> <scope> ban (<reason>)"
const BAN_TEXT_YELLOW_ONE_MATCH: usize = 0xa7a948; // "One yellow card away from <scope> 1 match ban"
const BAN_TEXT_YELLOW_MATCHES: usize = 0xa7a8ec; // "One yellow card away from <scope> <n> match ban"
const BAN_TEXT_YELLOW_DAYS: usize = 0xa7a894; // "One yellow card away from <scope> <n> day ban"
const BAN_SCOPE_GLOBAL: usize = 0x9c3478; // "global"

/// The ruling body's virtual slot +0x10 `get_name(text, size, form)` (thiscall, ret 0xc): its scope text.
unsafe fn ruling_body_scope_name(this: usize, text: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(read_usize(read_usize(this) + 0x10));
    f(this, text, 100, 4);
}

/// The days still to serve of a ban by days: its days (+2) less the difference between today and its start, as the
/// original computes it (`Date::get_difference` of `G_CURRENT_DATE` into the copy of today, then years × 365 + days).
unsafe fn ban_days_left(ban: usize) -> i32 {
    let mut diff = [0u8; 8];
    core::ptr::copy_nonoverlapping(G_CURRENT_DATE as *const u8, diff.as_mut_ptr(), 8);
    exe::date_get_difference(G_CURRENT_DATE, diff.as_mut_ptr(), read_u32(ban + 4), read_u32(ban + 8));
    let (day, year) = (i16::from_le_bytes([diff[0], diff[1]]) as i32, u16::from_le_bytes([diff[2], diff[3]]) as i32);
    read_i16(ban + 2) as i32 - year * 365 - day
}

/// Writes a text line about the person's ban from this body, or a "one yellow card away from a ban" warning.
///
/// `RulingBody::get_ban_text(person, text, size)`, from `Discipline::get_slot_text` (the person's discipline screen).
/// A size below 100: the ruling_body.cpp "Error" box (line 0x3c7) and 0. The person's entry in this body
/// (`table_penalty` with `ruling_body::RECORD_SLOT` and `RULE_SET_INDEX`; none: 0) has its bans moved on
/// (`Discipline::update_ban_progress` with 0 matches). Without a ban (`ruling_body_entry::BAN_INDEX` = −1), the body's
/// "one booking away" slot (vtable +0x14, with the entry, the matches, the days and the person's club's +0x57) decides:
/// 0 writes nothing; otherwise "One yellow card away from <scope> ..." with 1 match, n matches or n days (negative
/// matches). With a ban (`Discipline` ban n, 20 bytes): its four rule bytes (+0xc) must be −1 or this body's rule set
/// up to the first that is (another: 0). The scope: "global" for a global ban (flag 2 at +0x13) of rule set 0,
/// otherwise the body's name (vtable +0x10); the reason (`Discipline::get_reason_text`, 30 bytes). A ban that starts
/// after today ("from <date>", `Date::get_short_date_string`): 1 match, n matches (matches − served) or the time left
/// (`format_duration_text` of `ban_days_left`); one already running: the same without the date. Returns 1 when it
/// wrote.
/// Original: `RulingBody::get_ban_text` (0x7e88d0, ruling_body.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn ruling_body_get_ban_text(this: usize, person: usize, text: usize, size: i32) -> u32 {
    if size < 100 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32) -> u32>(%%I0%%)(this, person, text, size); // the box
    }
    let job = Staff(person).club_job();
    let club_field = if job.is_null() { 0 } else { read_u32(job.addr() + 0x57) }; // the club's +0x57
    let d = read_usize(G_DISCIPLINE);
    let id = read_i32(person);
    let k = table_penalty(d, id, read_i8(this + ruling_body::RECORD_SLOT), read_i8(this + ruling_body::RULE_SET_INDEX));
    if k == -1 {
        return 0;
    }
    let entry = read_usize(this + ruling_body::ENTRIES).wrapping_add((k as usize).wrapping_mul(14));
    exe::discipline_update_ban_progress(d, id as u32, read_u8(this + ruling_body::RULE_SET_INDEX) as u32, 0);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut scope = [0u8; 0x64];
    let b = read_i16(entry + ruling_body_entry::BAN_INDEX);
    if b == -1 {
        let (mut matches, mut days) = (0i8, 0i8);
        let away: unsafe extern "thiscall" fn(usize, usize, *mut i8, *mut i8, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this) + 0x14));
        if away(this, entry, &mut matches, &mut days, club_field) == 0 {
            return 0;
        }
        covered!("sx19b_5 one booking away");
        ruling_body_scope_name(this, scope.as_mut_ptr() as usize);
        let s = scope.as_ptr() as usize;
        if matches < 0 {
            sprintf(text, BAN_TEXT_YELLOW_DAYS, 100u32, 100u32, 100u32, s, days as i32);
        } else if matches == 1 {
            sprintf(text, BAN_TEXT_YELLOW_ONE_MATCH, 100u32, 100u32, s);
        } else {
            sprintf(text, BAN_TEXT_YELLOW_MATCHES, 100u32, 100u32, 100u32, s, matches as i32);
        }
        return 1;
    }
    let ban = read_usize(d + discipline_fields::BANS).wrapping_add((b as isize as usize).wrapping_mul(20));
    let rule = read_i8(this + ruling_body::RULE_SET_INDEX);
    for i in 0..4 {
        let r = read_i8(ban + 0xc + i);
        if r != -1 && r != rule {
            return 0;
        }
        if r == rule {
            break;
        }
    }
    let s = scope.as_mut_ptr() as usize;
    if read_u8(ban + ban_record::FLAGS) & 2 != 0 && rule == 0 {
        sprintf(s, BAN_SCOPE_GLOBAL);
    } else {
        ruling_body_scope_name(this, s);
    }
    let mut reason = [0u8; 0x20];
    let r = reason.as_mut_ptr() as usize;
    exe::discipline_get_reason_text(d, ban, r, 0x1e);
    let (today_day, today_year) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE + 2));
    let (start_day, start_year) = (read_i16(ban + 4), read_u16(ban + 6));
    let matches = read_i8(ban) as i32 - read_i8(ban + 1) as i32;
    let mut duration = [0u8; 0x20];
    let t = duration.as_mut_ptr() as usize;
    if today_year < start_year || (today_year == start_year && today_day < start_day) {
        covered!("sx19b_5 ban from a date");
        let mut date = [0u8; 0x34];
        let dt = date.as_mut_ptr() as usize;
        exe::date_get_short_date_string(ban + 4, dt, 0x32, 0xff);
        if read_i8(ban) > 0 {
            if matches == 1 {
                sprintf(text, BAN_TEXT_ONE_MATCH_FROM, 100u32, 100u32, 100u32, s, dt, r);
            } else {
                sprintf(text, BAN_TEXT_MATCHES_FROM, 100u32, 100u32, 100u32, matches, s, dt, r);
            }
        } else {
            exe::format_duration_text(ban_days_left(ban), t, 0x1e, 0);
            sprintf(text, BAN_TEXT_DAYS_FROM, 100u32, 100u32, 100u32, t, s, dt, r);
        }
    } else if read_i8(ban) > 0 {
        if matches == 1 {
            sprintf(text, BAN_TEXT_ONE_MATCH, 100u32, 100u32, s, r);
        } else {
            sprintf(text, BAN_TEXT_MATCHES, 100u32, 100u32, matches, s, r); // sx19b_5: the matches left
        }
    } else {
        exe::format_duration_text(ban_days_left(ban), t, 0x1e, 0);
        sprintf(text, BAN_TEXT_DAYS, 100u32, 100u32, 100u32, t, s, r);
    }
    1
}

/// Comparer for `ruling_body_get_ban_text`: side by side, the original into the screen's text and the copy into a
/// local one (the ban progress with 0 matches is already moved on when the copy runs); the texts and the answers.
pub(crate) unsafe extern "thiscall" fn compare_ruling_body_get_ban_text(this: usize, person: usize, text: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32) -> u32>(%%I0%%);
    if size < 100 || text == 0 {
        return o(this, person, text, size);
    }
    let ro = o(this, person, text, size);
    let mut local = [0u8; 0x400];
    let rr = ruling_body_get_ban_text(this, person, local.as_mut_ptr() as usize, size);
    let read_text = |a: usize| -> Vec<u8> {
        let b = core::slice::from_raw_parts(a as *const u8, size.min(0x400) as usize);
        b[..b.iter().position(|&c| c == 0).unwrap_or(b.len())].to_vec()
    };
    let (to, tr) = (if ro != 0 { read_text(text) } else { Vec::new() }, if rr != 0 { read_text(local.as_ptr() as usize) } else { Vec::new() });
    SCOREBOARD.record(ro == rr && to == tr, || format!("RulingBody::get_ban_text (person discipline screen, person {}): original {} {:?}, rust {} {:?}",
        read_i32(person), ro, String::from_utf8_lossy(&to), rr, String::from_utf8_lossy(&tr)));
    ro
}
