// ---------- sx18_23: the South Korean and the Asian card rules (ruling_body/rb_korea.cpp, rb_asia.cpp) ----------

/// The South Korean card rules' verdict. A red card (exactly one) queues a points ban (`pending_points_ban`) from
/// tomorrow and bans for one match (reason 2, amount 1). The match's bookings add to the entry's count; the 3rd and
/// every odd count after (5, 7, ...) ban for one more match (reason 1, amount 3 at the third, 2 after). The ban starts
/// today, under the Korean rule set alone.
pub(crate) unsafe fn korea_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize) -> CardVerdict {
    let rules = [KOREA_RULING_BODY.rule_set, 0xff, 0xff, 0xff];
    let (mut matches, mut reason, mut amount, mut pending) = (0u8, 0xffu8, 0i16, Vec::new());
    if row.red_card() == 1 {
        covered!("sx18_23 korean red card");
        pending.extend(pending_points_ban(this, row, fixture, rules, || ban_start_in(1)));
        matches = 1;
        reason = 2;
        amount = 1;
    }
    let bookings = row.yellow_cards();
    if bookings != 0 {
        let count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(bookings);
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
        if count >= 3 && (count - 3) % 2 == 0 { // sx18_23: the third and every second Korean booking after
            covered!("sx18_23 korean booking ban");
            reason = 1;
            matches += 1;
            amount = if count == 3 { 3 } else { 2 };
        }
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules, start: ban_start_today() });
    CardVerdict { pending, ban }
}

/// Applies a player's cards of one match under the South Korean rules (`korea_card_verdict`).
/// Original: `RbKorea::process_match_cards` (0x7e0780, rb_korea.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_korea_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| korea_card_verdict(this, MatchRecordRow(row), fixture, e));
}

/// The Asian card rules (`plain_card_verdict`): one match for a red card and one for every second booking (counted one
/// per match), under the Asian rule set in the confederation slot (rules ff ff 29 ff).
pub(crate) unsafe fn asia_card_verdict(row: MatchRecordRow, entry: usize) -> CardVerdict {
    covered!("sx18_23 asian cards");
    plain_card_verdict(row, entry, false, 2, [0xff, 0xff, ASIA_RULING_BODY.rule_set, 0xff]) // sx18_23: every second Asian booking
}

/// Applies a player's cards of one match under the Asian rules (`asia_card_verdict`).
/// Original: `RbAsia::process_match_cards` (0x7db3b0, rb_asia.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_asia_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| asia_card_verdict(MatchRecordRow(row), e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_korea_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| korea_card_verdict(this, MatchRecordRow(row), fixture, e), "rb_korea_process_match_cards");
}

pub(crate) unsafe extern "thiscall" fn compare_rb_asia_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| asia_card_verdict(MatchRecordRow(row), e), "rb_asia_process_match_cards");
}
