// ---------- sx18_8: the French card rules: the ten-match booking window (ruling_body/rb_france.cpp) ----------

/// The French card rules' verdict, on every player of a match (the entry is found first, cards or not).
///
/// The entry keeps a ten-match window of booked matches: bits 0..7 of its byte +0xa, bit 0 of +0xb (the 9th) and bit 0
/// of +0xc (the 10th, the match being played). Each match moves the window one match back (bit i takes bit i + 1 for i
/// up to 8; the 10th stays) and counts the booked matches among the nine before. A red card (exactly one) queues a
/// points ban (`pending_points_ban`) from the first weekday 3 after today and bans for one match on the spot (reason
/// 2, amount 1). A booking marks the 10th match and, when it makes three booked matches in the window, queues a
/// one-match ban (reason 1, amount 3, from today, the player's id at +0x1c) that becomes effective on the first weekday
/// 3 after today, and empties the window; with no booking the 10th match is cleared. The bans are under the French
/// rule set alone. The original leaves the booking node's flags (+0x13) and its byte +0x6f as stack garbage (or what a
/// red card's node left there in the same call); here they are 0.
pub(crate) unsafe fn france_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize) -> CardVerdict {
    let (old, ninth, tenth) = (read_u8(entry + 0xa), read_u8(entry + 0xb), read_u8(entry + 0xc)); // the booking window bytes
    let window = (old as u16 | ((ninth as u16 & 1) << 8) | ((tenth as u16 & 1) << 9)) >> 1 | ((tenth as u16 & 1) << 9);
    write_u8(entry + 0xa, window as u8);
    write_u8(entry + 0xb, (ninth & 0xfe) | ((window >> 8) as u8 & 1));
    let mut booked = (window & 0x1ff).count_ones() as u8;
    let rules = [FRANCE_RULING_BODY.rule_set, 0xff, 0xff, 0xff];
    let (mut matches, mut reason, mut amount, mut pending) = (0u8, 0xffu8, 0i16, Vec::new());
    if row.red_card() == 1 {
        covered!("sx18_8 french red card");
        pending.extend(pending_points_ban(this, row, fixture, rules, || first_weekday_after_today(3)));
        matches = 1;
        reason = 2;
        amount = 1;
    }
    if row.yellow_cards() != 0 {
        write_u8(entry + 0xc, read_u8(entry + 0xc) | 1);
        booked = booked.wrapping_add(1);
        if booked == 3 { // sx18_8: three booked matches in ten
            covered!("sx18_8 french booking ban");
            pending.push(france_booking_node(row, fixture, rules));
            write_u8(entry + 0xa, 0);
            write_u8(entry + 0xb, read_u8(entry + 0xb) & 0xfe);
            write_u8(entry + 0xc, read_u8(entry + 0xc) & 0xfe);
        }
    } else {
        write_u8(entry + 0xc, read_u8(entry + 0xc) & 0xfe);
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules, start: ban_start_today() });
    CardVerdict { pending, ban }
}

/// The pending one-match ban of three booked matches: one match, none served, no days, from today, the player's id
/// (`pending_ban::PERSON_ID`), the rules, reason 1 with amount 3; effective on the first weekday 3 after today (+0x14);
/// the fixture's first 0x4f bytes at +0x20.
unsafe fn france_booking_node(row: MatchRecordRow, fixture: usize, rules: [u8; 4]) -> [u8; 0x70] {
    let mut node = [0u8; 0x70];
    let p = node.as_mut_ptr() as usize;
    write_u8(p + ban_record::MATCHES, 1);
    write_u8(p + ban_record::MATCHES_SERVED, 0);
    write_u16(p + ban_record::DAYS, 0xffff);
    copy_record_date(p + ban_record::START_DATE, G_CURRENT_DATE);
    write_u32(p + pending_ban::PERSON_ID, row.staff_id() as u32);
    for k in 0..4 {
        write_u8(p + ban_record::RULES + k, rules[k]);
    }
    write_u8(p + ban_record::REASON, 1);
    write_u16(p + ban_record::REASON_AMOUNT, 3);
    let d = first_weekday_after_today(3);
    copy_record_date(p + 0x14, d.as_ptr() as usize); // the pending ban's start date
    core::ptr::copy_nonoverlapping(fixture as *const u8, (p + 0x20) as *mut u8, 0x4f); // the match, for the news
    node
}

/// Applies a player's cards of one match under the French rules (`france_card_verdict`), for every player of the match.
/// Original: `RbFrance::process_match_cards` (0x7ddd10, rb_france.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_france_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_entry_card_rules(this, row, fixture, |e| france_card_verdict(this, MatchRecordRow(row), fixture, e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_france_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_entry_card_rules(this, row, || o(this, row, fixture), |e| france_card_verdict(this, MatchRecordRow(row), fixture, e), "rb_france_process_match_cards");
}
