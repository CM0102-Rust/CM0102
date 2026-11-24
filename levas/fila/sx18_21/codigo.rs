// ---------- sx18_21: the Croatian card rules (ruling_body/rb_croatia.cpp) ----------

/// The Croatian card rules' verdict. Each of the match's bookings adds one to the entry's count and every third bans
/// for one more match; once there is a ban, each booking sets reason 1 with the count as amount. A red card (exactly
/// one) queues a points ban (`pending_points_ban`) from the first weekday 3 after today and bans for one more match
/// (reason 2, amount 1). The ban starts today, under the Croatian rule set alone.
pub(crate) unsafe fn croatia_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize) -> CardVerdict {
    let rules = [CROATIA_RULING_BODY.rule_set, 0xff, 0xff, 0xff];
    let (mut matches, mut reason, mut amount, mut pending) = (0u8, 0xffu8, 0i16, Vec::new());
    for _ in 0..row.yellow_cards() {
        let count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(1);
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
        if count % 3 == 0 { // sx18_21: every third Croatian booking
            covered!("sx18_21 croatian booking ban");
            matches += 1;
        }
        if matches != 0 {
            reason = 1;
            amount = count as i8 as i16;
        }
    }
    if row.red_card() == 1 {
        covered!("sx18_21 croatian red card");
        reason = 2;
        pending.extend(pending_points_ban(this, row, fixture, rules, || first_weekday_after_today(3)));
        matches += 1;
        amount = 1;
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules, start: ban_start_today() });
    CardVerdict { pending, ban }
}

/// Applies a player's cards of one match under the Croatian rules (`croatia_card_verdict`).
/// Original: `RbCroatia::process_match_cards` (0x7dc750, rb_croatia.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_croatia_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| croatia_card_verdict(this, MatchRecordRow(row), fixture, e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_croatia_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| croatia_card_verdict(this, MatchRecordRow(row), fixture, e), "rb_croatia_process_match_cards");
}
