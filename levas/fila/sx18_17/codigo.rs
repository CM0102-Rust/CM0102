// ---------- sx18_17: the Northern Irish points rules and new season (ruling_body/rb_northern_ireland.cpp) ----------

/// The Northern Irish points rules' verdict. A red card (exactly one) adds 12 points when it came from a second booking
/// (one booking in the match) and bans for one match (reason 2, amount 1). Each of the match's bookings adds 4 points.
/// The first threshold of 51, 42, 32 and 20 points that the new total reaches and the points before the match were
/// under bans for six, four, two or one more match (reason 0, the threshold as amount); reaching 51 also clears the
/// points. The ban starts on the first day from today whose weekday is 0, under the Northern Irish rule set alone.
pub(crate) unsafe fn northern_ireland_card_verdict(row: MatchRecordRow, entry: usize) -> CardVerdict {
    let points = |e: usize| read_u16(e + ruling_body_entry::DISCIPLINE_POINTS);
    let before = points(entry);
    let (mut matches, mut reason, mut amount) = (0u8, 0xffu8, before as i16);
    if row.red_card() == 1 {
        covered!("sx18_17 northern irish red card");
        if row.yellow_cards() == 1 {
            write_u16(entry + ruling_body_entry::DISCIPLINE_POINTS, before.wrapping_add(0xc));
        }
        matches = 1;
        reason = 2;
        amount = 1;
    }
    for _ in 0..row.yellow_cards() {
        write_u16(entry + ruling_body_entry::DISCIPLINE_POINTS, points(entry).wrapping_add(4)); // sx18_17: 4 points a booking
    }
    let total = points(entry);
    for (threshold, more) in [(0x33u16, 6u8), (0x2a, 4), (0x20, 2), (0x14, 1)] {
        if total >= threshold && (before as i16) < threshold as i16 {
            covered!("sx18_17 northern irish points ban");
            matches += more;
            reason = 0;
            amount = threshold as i16;
            if threshold == 0x33 {
                write_u16(entry + ruling_body_entry::DISCIPLINE_POINTS, 0);
            }
            break;
        }
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules: [NORTHERN_IRELAND_RULING_BODY.rule_set, 0xff, 0xff, 0xff],
        start: date_on_weekday(ban_start_today(), 0) });
    CardVerdict { pending: Vec::new(), ban }
}

/// Applies a player's cards of one match under the Northern Irish points rules (`northern_ireland_card_verdict`).
/// Original: `RbNorthernIreland::process_match_cards` (0x7e0b40, rb_northern_ireland.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_northern_ireland_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| northern_ireland_card_verdict(MatchRecordRow(row), e));
}

/// The Northern Irish new season: clears the points of every entry (`ruling_body_entry::DISCIPLINE_POINTS` of the
/// `ruling_body::ENTRY_COUNT` 14-byte entries), free or not; nothing else (no ban released, no entry freed).
/// Original: `RbNorthernIreland::start_new_season` (0x7e0ec0, rb_northern_ireland.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn rb_northern_ireland_start_new_season(this: usize) {
    covered!("sx18_17 northern irish new season");
    let e = read_usize(this + ruling_body::ENTRIES);
    let mut i = 0i32;
    while i < read_i32(this + ruling_body::ENTRY_COUNT) {
        write_u16(e + i as usize * 0xe + ruling_body_entry::DISCIPLINE_POINTS, 0);
        i += 1;
    }
}

pub(crate) unsafe extern "thiscall" fn compare_rb_northern_ireland_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| northern_ireland_card_verdict(MatchRecordRow(row), e), "rb_northern_ireland_process_match_cards");
}

/// Comparer for `rb_northern_ireland_start_new_season`: the copy clears a copy of the entries first, then the original
/// runs on the table; the two must match byte for byte.
pub(crate) unsafe extern "thiscall" fn compare_rb_northern_ireland_start_new_season(this: usize) {
    let n = read_i32(this + ruling_body::ENTRY_COUNT).max(0) as usize;
    let e = read_usize(this + ruling_body::ENTRIES);
    let mut copy = vec![0u8; n * 0xe];
    if n != 0 {
        core::ptr::copy_nonoverlapping(e as *const u8, copy.as_mut_ptr(), n * 0xe);
    }
    let mut local = [0u8; 0x23];
    core::ptr::copy_nonoverlapping(this as *const u8, local.as_mut_ptr(), 0x23);
    write_usize(local.as_mut_ptr() as usize + ruling_body::ENTRIES, copy.as_mut_ptr() as usize);
    rb_northern_ireland_start_new_season(local.as_mut_ptr() as usize);
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let same = n == 0 || core::slice::from_raw_parts(e as *const u8, n * 0xe) == &copy[..];
    SCOREBOARD.record(same, || format!("rb_northern_ireland_start_new_season: {} entries differ after the reset", n));
}
