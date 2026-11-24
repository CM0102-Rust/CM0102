// ---------- sx18_11: the Japanese cup card rules and the Australian ruling body (constructor and card rules) (ruling_body/rb_japan_cup.cpp, rb_australia.cpp) ----------

/// The Australian ruling body: vtable 0x96ea34, record slot 0, rule set 0x22, ban scale 10, Australia in Oceania,
/// league rule 0x22, no cup rule, the Oceanian rule 0x23.
pub(crate) const AUSTRALIA_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ea34, record_slot: 0, rule_set: 0x22, scale: 0xa,
    nation: Some(KEY_NATION_AUSTRALIA), continent: Some(KEY_CONTINENT_OCEANIA), league_rule: 0x22, cup_rule: 0xff, continental_rule: 0x23 };

/// The Japanese cup rules: every second booking, a red card with no offence code counts nothing, under the cup rule
/// set in the second slot.
unsafe fn japan_cup_card_verdict(row: usize, e: usize) -> CardVerdict {
    japan_card_verdict(MatchRecordRow(row), e, 2, 0, [0xff, JAPAN_CUP_RULING_BODY.rule_set, 0xff, 0xff]) // sx18_11: every second Japanese cup booking
}

/// Applies a player's cards of one match under the Japanese cup rules (`japan_card_verdict`, every second booking).
/// Original: `RbJapanCup::process_match_cards` (0x7e0200, rb_japan_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_japan_cup_process_match_cards(this: usize, row: usize, fixture: usize) {
    covered!("sx18_11 japanese cup cards");
    run_card_rules(this, row, fixture, |e| japan_cup_card_verdict(row, e));
}

/// Builds the Australian ruling body (`ruling_body_class_construct` with `AUSTRALIA_RULING_BODY`). Returns this.
/// Original: `RbAustralia::RbAustralia` (0x7db5b0, rb_australia.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_australia_construct(this: usize, file: usize) -> usize {
    covered!("sx18_11 australian body built");
    ruling_body_class_construct(this, file, &AUSTRALIA_RULING_BODY)
}

/// The Australian card rules' verdict. A match with exactly one booking adds one to the entry's count, which bans for
/// one match at 5 and at every second count from 7 (7, 9, ...; reason 1, the count as amount); other numbers of
/// bookings are not counted. A red card (exactly one) counts in the entry and bans for one more match, two from the
/// third red card on (reason 2, amount 1). The ban starts today, under the Australian rule set alone.
pub(crate) unsafe fn australia_card_verdict(row: MatchRecordRow, entry: usize) -> CardVerdict {
    let (mut matches, mut reason, mut amount) = (0u8, 0xffu8, 0i16);
    if row.yellow_cards() == 1 {
        let count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(1);
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
        if count == 5 || (count >= 7 && (count - 5) % 2 == 0) { // sx18_11: the Australian booking bans
            covered!("sx18_11 australian booking ban");
            matches = 1;
            reason = 1;
            amount = count as i8 as i16;
        }
    }
    if row.red_card() == 1 {
        covered!("sx18_11 australian red card");
        let reds = read_u8(entry + ruling_body_entry::RED_CARDS).wrapping_add(1);
        write_u8(entry + ruling_body_entry::RED_CARDS, reds);
        matches += if reds < 3 { 1 } else { 2 };
        reason = 2;
        amount = 1;
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules: [AUSTRALIA_RULING_BODY.rule_set, 0xff, 0xff, 0xff],
        start: ban_start_today() });
    CardVerdict { pending: Vec::new(), ban }
}

/// Applies a player's cards of one match under the Australian rules (`australia_card_verdict`).
/// Original: `RbAustralia::process_match_cards` (0x7db600, rb_australia.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_australia_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| australia_card_verdict(MatchRecordRow(row), e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_japan_cup_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| japan_cup_card_verdict(row, e), "rb_japan_cup_process_match_cards");
}

pub(crate) unsafe extern "thiscall" fn compare_rb_australia_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(this, file);
    compare_ruling_body_class_built(r, this, &AUSTRALIA_RULING_BODY, "rb_australia_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_australia_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| australia_card_verdict(MatchRecordRow(row), e), "rb_australia_process_match_cards");
}
