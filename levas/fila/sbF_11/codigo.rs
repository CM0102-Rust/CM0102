// ---------- sbF_11: the AI friendly tournament (friendly.cpp) ----------

/// The days a friendly tournament of `kind` takes (the inline switch of friendly.cpp, the same table as
/// `friendly_get_rest_days_for_kind`): 2 for kinds 1 and 10, 3 for kind 2, 1 for kinds 0, 3, 4 and 11; any other kind
/// the box at line 0xf18 of friendly.cpp and 1.
pub(crate) unsafe fn friendly_tournament_days(kind: i8) -> u8 {
    match kind {
        1 | 10 => 2,
        2 => 3,
        0 | 3 | 4 | 11 => 1,
        _ => {
            show_error_at(0x9cd0a0, 0xf18);
            1
        }
    }
}

/// Rule: an AI preseason tournament for a group of clubs: it starts on the last day of the clubs' common friendly
/// window that still leaves room for the tournament (a friendly cup takes 2 days, a friendly league 3), moved back to a
/// Saturday [inference: weekday 5] when the window is wider than a week; no room: no tournament. The host is the first
/// club; the tournament is named "Friendly Cup" (kind 1) or "Friendly League" (the others), the invitations go out, and
/// every club gets its tournament days in its planned dates and 2 (cup) or 3 (league) more friendlies.
///
/// `Friendly::create_ai_tournament(group, n, kind)`: over the `n` candidates of `group` (signed byte count), the latest
/// `preseason_candidate::WINDOW_START` and the earliest `preseason_candidate::WINDOW_END` (from 0x16d) of those with a
/// window (start ≠ −1), 16-bit signed; the end goes back by the tournament days (`friendly_tournament_days`, the kind a
/// signed byte); start ≥ end: 0. Otherwise the start date = `friendly::PRESEASON_BASE_DATE` + end days
/// (`exe::date_add_days`; the original pushes the day count with the high 16 bits of a pointer, Date::add_days reads
/// the word); when end − start > 6, one day back (`exe::date_sub_days`, `exe::date_operator_3d`) until
/// `exe::date_get_weekday` = 5. The host and the invited clubs are the clubs of the candidates
/// (`preseason_candidate::CLUB_ID`, `club_at`). The name into the text buffer (`FN_LANGLIB_SPRINTF`, cdecl) with byte
/// +0x18 zeroed; `add_tournament`(host, 1, start, 0, 1, 0, 1, n, invited, name) for a cup, (host, kind, start, 3, 1, 0,
/// 0, n, invited, name) for a league; Friendly::send_tournament_invitations of the slot it returns (thiscall, ret 4,
/// even when 0). Then each candidate gets a date list when it has none (`exe::crt_operator_new`(0xe) and LLIST::LLIST;
/// 0 without memory) and its count +4 up by 2 or 3; and, from the start up to the start + the tournament days
/// (exclusive), every day is appended to each candidate's list (`exe::llist_add_tail`(list, day, 8)). Returns 1.
/// Original: `Friendly::create_ai_tournament` (0x5c3be0, friendly.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn friendly_create_ai_tournament(this: usize, group: usize, n: u32, kind: u32) -> u32 {
    let n = n as u8 as i8;
    let kind = kind as u8 as i8;
    let candidate = |k: usize| read_usize(group + 4 * k);
    let (mut start, mut end) = (0i16, 0x16di16);
    for k in 0..n.max(0) as usize {
        let c = candidate(k);
        let s = read_i16(c + preseason_candidate::WINDOW_START);
        if s != -1 {
            start = start.max(s);
            end = end.min(read_i16(c + preseason_candidate::WINDOW_END));
        }
    }
    let end = end.wrapping_sub(friendly_tournament_days(kind) as i16); // sbF11: room for the tournament
    if start >= end {
        return 0;
    }
    covered!("sbF_11 ai tournament");
    let (mut first, mut t0, mut t1) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let (pd, p0, p1) = (first.as_mut_ptr() as usize, t0.as_mut_ptr() as usize, t1.as_mut_ptr() as usize);
    exe::date_add_days(this + friendly::PRESEASON_BASE_DATE, pd, end as u16 as u32);
    if end as i32 - start as i32 > 6 {
        while exe::date_get_weekday(pd) as i16 != 5 { // sbF11: back to the weekday 5
            let r = exe::date_sub_days(pd, p0, 1);
            exe::date_operator_3d(pd, p1, r);
        }
    }
    let host = club_at(read_i32(candidate(0) + preseason_candidate::CLUB_ID) as isize).addr();
    // simplification: the original's list of four is the stack, written only for the n candidates; add_tournament
    // copies four (the callers pass n = 4)
    let mut invited = [0usize; 4];
    for (k, slot) in invited.iter_mut().enumerate().take(n.max(0) as usize) {
        *slot = club_at(read_i32(candidate(k) + preseason_candidate::CLUB_ID) as isize).addr();
    }
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let cup = kind == 1;
    format(TS_TEXT, if cup { 0x9cd0e0 } else { 0x9cd0d0 }); // "Friendly Cup" / "Friendly League"
    write_u8(TS_TEXT + 0x18, 0);
    let count = n as i16 as u16 as u32;
    let slot = if cup {
        add_tournament(this, host, 1, pd, 0, 1, 0, 1, count, invited.as_ptr() as usize, TS_TEXT)
    } else {
        add_tournament(this, host, kind as u32, pd, 3, 1, 0, 0, count, invited.as_ptr() as usize, TS_TEXT)
    };
    let invite: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5c5310usize); // Friendly::send_tournament_invitations
    invite(this, slot);
    for k in 0..n.max(0) as usize {
        let c = candidate(k);
        if read_usize(c + preseason_candidate::PLANNED_DATES) == 0 {
            let p = exe::crt_operator_new(0xe);
            let list = if p != 0 { exe::new_llist(p) } else { 0 };
            write_usize(c + preseason_candidate::PLANNED_DATES, list);
        }
        write_u8(c + 4, read_u8(c + 4).wrapping_add(if cup { 2 } else { 3 })); // +4: the candidate's friendly count
    }
    let (mut last, mut day) = ([0u8; 8], first);
    let (pe, pc) = (last.as_mut_ptr() as usize, day.as_mut_ptr() as usize);
    exe::date_add_days(pd, pe, friendly_tournament_days(kind) as i16 as u16 as u32);
    while friendly_date_after(pe, pc) {
        for k in 0..n.max(0) as usize {
            exe::llist_add_tail(read_usize(candidate(k) + preseason_candidate::PLANNED_DATES), pc, 8);
        }
        let r = exe::date_add_days(pc, p1, 1);
        exe::date_operator_3d(pc, p0, r);
    }
    1
}

/// Comparer for `friendly_create_ai_tournament`: only the original (it fills a tournament slot, sends news and grows the
/// lists); the plan: the return (1 when the copy's window leaves room, else 0), and on 1 every candidate with a date
/// list and its count up by 2 (cup) or 3.
pub(crate) unsafe extern "thiscall" fn compare_friendly_create_ai_tournament(this: usize, group: usize, n: u32, kind: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(%%I0%%);
    let count = (n as u8 as i8).max(0) as usize;
    let candidates: Vec<usize> = (0..count).map(|k| read_usize(group + 4 * k)).collect();
    let windows: Vec<(i16, i16)> = candidates.iter().map(|&c| (read_i16(c + preseason_candidate::WINDOW_START), read_i16(c + preseason_candidate::WINDOW_END))).collect();
    let (start, end) = windows.iter().filter(|w| w.0 != -1).fold((0i16, 0x16di16), |(s, e), w| (s.max(w.0), e.min(w.1)));
    let days = match kind as u8 as i8 { 1 | 10 => 2, 2 => 3, _ => 1 };
    let expected = (start < end.wrapping_sub(days)) as u32;
    let before: Vec<u8> = candidates.iter().map(|&c| read_u8(c + 4)).collect();
    let r = o(this, group, n, kind);
    let step = if kind as u8 == 1 { 2 } else { 3 };
    let ok = r == expected && (r == 0 || candidates.iter().zip(&before).all(|(&c, &b)| read_u8(c + 4) == b.wrapping_add(step) && read_usize(c + preseason_candidate::PLANNED_DATES) != 0));
    SCOREBOARD.record(ok, || format!("friendly_create_ai_tournament({:#x}, {}, kind {}) returned {} (plan {}), window {}..{}", this, count, kind as u8 as i8, r, expected, start, end));
    r
}
