// ---------- sx18_20: the Greek ruling body (constructor and card rules) and the Croatian body's constructor (ruling_body/rb_greece.cpp, rb_croatia.cpp) ----------

/// The Greek ruling body: vtable 0x96ebf4, record slot 0, rule set 0x20, ban scale 10, Greece in Europe, league rule
/// 0x20, no cup rule, the European rule 1.
pub(crate) const GREECE_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ebf4, record_slot: 0, rule_set: 0x20, scale: 0xa,
    nation: Some(KEY_NATION_GREECE), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0x20, cup_rule: 0xff, continental_rule: 1 };

/// The Croatian ruling body: vtable 0x96ead4, record slot 0, rule set 0x21, ban scale 10, Croatia in Europe, league
/// rule 0x21, no cup rule, the European rule 1.
pub(crate) const CROATIA_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ead4, record_slot: 0, rule_set: 0x21, scale: 0xa,
    nation: Some(KEY_NATION_CROATIA), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0x21, cup_rule: 0xff, continental_rule: 1 };

/// Builds the Greek ruling body (`ruling_body_class_construct` with `GREECE_RULING_BODY`). Returns this.
/// Original: `RbGreece::RbGreece` (0x7de960, rb_greece.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_greece_construct(this: usize, file: usize) -> usize {
    covered!("sx18_20 greek body built");
    ruling_body_class_construct(this, file, &GREECE_RULING_BODY)
}

/// The Greek card rules' verdict. Each of the match's bookings adds one to the entry's count; at every third the ban
/// becomes count / 3 matches (3 → 1, 6 → 2, ...), and from then on each booking sets reason 1 with the count as
/// amount. A red card (exactly one) queues a points ban (`pending_points_ban`) from the first weekday 3 after today and
/// bans for one more match (reason 2, amount 1). The ban starts today, under the Greek rule set alone.
pub(crate) unsafe fn greece_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize) -> CardVerdict {
    let rules = [GREECE_RULING_BODY.rule_set, 0xff, 0xff, 0xff];
    let (mut matches, mut reason, mut amount, mut pending) = (0u8, 0xffu8, 0i16, Vec::new());
    for _ in 0..row.yellow_cards() {
        let count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(1);
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
        if count % 3 == 0 {
            covered!("sx18_20 greek booking ban");
            matches = count / 3; // sx18_20: the Greek ban grows with the bookings
        }
        if matches != 0 {
            reason = 1;
            amount = count as i8 as i16;
        }
    }
    if row.red_card() == 1 {
        covered!("sx18_20 greek red card");
        pending.extend(pending_points_ban(this, row, fixture, rules, || first_weekday_after_today(3)));
        matches = matches.wrapping_add(1);
        reason = 2;
        amount = 1;
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules, start: ban_start_today() });
    CardVerdict { pending, ban }
}

/// Applies a player's cards of one match under the Greek rules (`greece_card_verdict`).
/// Original: `RbGreece::process_match_cards` (0x7de9b0, rb_greece.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_greece_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| greece_card_verdict(this, MatchRecordRow(row), fixture, e));
}

/// Builds the Croatian ruling body (`ruling_body_class_construct` with `CROATIA_RULING_BODY`). Returns this.
/// Original: `RbCroatia::RbCroatia` (0x7dc700, rb_croatia.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_croatia_construct(this: usize, file: usize) -> usize {
    covered!("sx18_20 croatian body built");
    ruling_body_class_construct(this, file, &CROATIA_RULING_BODY)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_greece_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_ruling_body_class_built(r, this, &GREECE_RULING_BODY, "rb_greece_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_greece_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| greece_card_verdict(this, MatchRecordRow(row), fixture, e), "rb_greece_process_match_cards");
}

pub(crate) unsafe extern "thiscall" fn compare_rb_croatia_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%)(this, file);
    compare_ruling_body_class_built(r, this, &CROATIA_RULING_BODY, "rb_croatia_construct")
}
