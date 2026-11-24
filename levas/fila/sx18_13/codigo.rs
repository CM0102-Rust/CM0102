// ---------- sx18_13: the Belgian cup card rules and the Dutch ruling body (constructor and card rules) (ruling_body/rb_belgium_cup.cpp, rb_holland.cpp) ----------

/// The Dutch ruling body: vtable 0x96ec14, record slot 0, rule set 0xf, ban scale 0x11, Holland in Europe, league rule
/// 0xf, no cup rule, the European rule 1.
pub(crate) const HOLLAND_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ec14, record_slot: 0, rule_set: 0xf, scale: 0x11,
    nation: Some(KEY_NATION_HOLLAND), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0xf, cup_rule: 0xff, continental_rule: 1 };

/// The Belgian cup card rules' verdict. A red card (exactly one) queues a points ban (`pending_points_ban`) from the
/// 15th of this month (of the next when today is past it) and bans for one match on the spot (reason 2, amount 1). A
/// booking adds one to the entry's count and every third one bans for one more match (reason 1, amount 3). The ban
/// starts today, under the cup rule set in the second slot.
pub(crate) unsafe fn belgium_cup_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize) -> CardVerdict {
    let rules = [0xff, BELGIUM_CUP_RULING_BODY.rule_set, 0xff, 0xff];
    let (mut matches, mut reason, mut amount, mut pending) = (0u8, 0xffu8, 0i16, Vec::new());
    if row.red_card() == 1 {
        covered!("sx18_13 belgian cup red card");
        pending.extend(pending_points_ban(this, row, fixture, rules, || day_of_this_or_next_month(0xf))); // sx18_13: from the 15th
        matches = 1;
        reason = 2;
        amount = 1;
    }
    if row.yellow_cards() != 0 {
        let count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(1);
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
        if count % 3 == 0 {
            matches += 1;
            reason = 1;
            amount = 3;
        }
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules, start: ban_start_today() });
    CardVerdict { pending, ban }
}

/// Applies a player's cards of one match under the Belgian cup rules (`belgium_cup_card_verdict`).
/// Original: `RbBelgiumCup::process_match_cards` (0x7db940, rb_belgium_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_belgium_cup_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| belgium_cup_card_verdict(this, MatchRecordRow(row), fixture, e));
}

/// Builds the Dutch ruling body (`ruling_body_class_construct` with `HOLLAND_RULING_BODY`). Returns this.
/// Original: `RbHolland::RbHolland` (0x7ded90, rb_holland.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_holland_construct(this: usize, file: usize) -> usize {
    covered!("sx18_13 dutch body built");
    ruling_body_class_construct(this, file, &HOLLAND_RULING_BODY)
}

/// The Dutch card rules' verdict. Each of the match's bookings adds one to the entry's count; the third and every one
/// from the fifth ban for one more match each (reason 1, the count as amount). A red card (exactly one) queues a points
/// ban (`pending_points_ban`) from the first weekday 3 after today and bans for one more match (reason 2, amount 1). The
/// ban starts today, under the Dutch rule set alone.
pub(crate) unsafe fn holland_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize) -> CardVerdict {
    let rules = [HOLLAND_RULING_BODY.rule_set, 0xff, 0xff, 0xff];
    let (mut matches, mut reason, mut amount, mut pending) = (0u8, 0xffu8, 0i16, Vec::new());
    for _ in 0..row.yellow_cards() {
        let count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(1);
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
        if count == 3 || count > 4 { // sx18_13: the third and from the fifth Dutch booking
            covered!("sx18_13 dutch booking ban");
            matches += 1;
            amount = count as i8 as i16;
            reason = 1;
        }
    }
    if row.red_card() == 1 {
        covered!("sx18_13 dutch red card");
        reason = 2;
        pending.extend(pending_points_ban(this, row, fixture, rules, || first_weekday_after_today(3)));
        matches += 1;
        amount = 1;
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules, start: ban_start_today() });
    CardVerdict { pending, ban }
}

/// Applies a player's cards of one match under the Dutch rules (`holland_card_verdict`).
/// Original: `RbHolland::process_match_cards` (0x7dede0, rb_holland.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_holland_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| holland_card_verdict(this, MatchRecordRow(row), fixture, e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_belgium_cup_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| belgium_cup_card_verdict(this, MatchRecordRow(row), fixture, e), "rb_belgium_cup_process_match_cards");
}

pub(crate) unsafe extern "thiscall" fn compare_rb_holland_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(this, file);
    compare_ruling_body_class_built(r, this, &HOLLAND_RULING_BODY, "rb_holland_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_holland_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| holland_card_verdict(this, MatchRecordRow(row), fixture, e), "rb_holland_process_match_cards");
}
