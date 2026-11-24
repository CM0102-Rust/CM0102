// ---------- sx19_3: the Spanish cup body's card rules and the United States ruling body (constructor and card rules) (ruling_body/rb_spain_cup.cpp, rb_usa.cpp) ----------

/// The Spanish cup card rules' verdict (`stepped_card_verdict`): a red card queues a points ban from tomorrow and bans
/// for one match; every second booking (one per match) bans for one more (amount 2); under the cup rule set.
pub(crate) unsafe fn spain_cup_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize) -> CardVerdict {
    covered!("sx19_3 spanish cup cards");
    stepped_card_verdict(this, row, fixture, entry, &SPAIN_CUP_RULING_BODY, 2) // sx19_3: every second Spanish cup booking
}

/// Applies a player's cards of one match under the Spanish cup rules (`spain_cup_card_verdict`).
/// Original: `RbSpainCup::process_match_cards` (0x7e2c80, rb_spain_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_spain_cup_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| spain_cup_card_verdict(this, MatchRecordRow(row), fixture, e));
}

/// The United States ruling body: vtable 0x96ef14, record slot 0, rule set 0x1e, ban scale 10, the United States in
/// North America, league rule 0x1e, no cup nor continental rule.
pub(crate) const USA_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ef14, record_slot: 0, rule_set: 0x1e, scale: 0xa,
    nation: Some(KEY_NATION_UNITED_STATES), continent: Some(KEY_CONTINENT_NORTH_AMERICA), league_rule: 0x1e, cup_rule: 0xff, continental_rule: 0xff };

/// Builds the United States ruling body (`ruling_body_class_construct` with `USA_RULING_BODY`). Returns this.
/// Original: `RbUsa::RbUsa` (0x7e40f0, rb_usa.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_usa_construct(this: usize, file: usize) -> usize {
    covered!("sx19_3 us body built");
    ruling_body_class_construct(this, file, &USA_RULING_BODY)
}

/// The disciplinary points of one booking under the US rules, by its offence code (signed): up to 3 one point, up to 6
/// two, above that four.
pub(crate) fn usa_booking_points(offence: i8) -> u16 {
    match offence {
        i8::MIN..=3 => 1,
        4..=6 => 2,
        _ => 4,
    }
}

/// The US card rules' verdict. Exactly one red card bans for one match (reason 2, amount 1). Each booking adds its
/// points (`usa_booking_points`; the first booking's offence code, then the second's) to the entry's disciplinary
/// points. At 32 points or more the player is banned for one more match and loses 11 points; from 21 to 31 he is banned
/// for one more match and keeps them, so each later match with a card bans him again. A points ban alone has reason 0
/// and amount 11 (from 32) or 21. The ban starts today, under the US rule set alone.
pub(crate) unsafe fn usa_card_verdict(row: MatchRecordRow, entry: usize) -> CardVerdict {
    let (mut matches, mut reason, mut amount) = (0u8, 0xffu8, 0i16);
    if row.red_card() == 1 {
        covered!("sx19_3 us red card");
        matches = 1;
        reason = 2;
        amount = 1;
    }
    for k in 0..row.yellow_cards() as i8 {
        let offence = if k == 0 { row.first_card_offence() } else { row.second_card_offence() };
        let points = read_u16(entry + ruling_body_entry::DISCIPLINE_POINTS);
        write_u16(entry + ruling_body_entry::DISCIPLINE_POINTS, points.wrapping_add(usa_booking_points(offence)));
    }
    let points = read_u16(entry + ruling_body_entry::DISCIPLINE_POINTS);
    if points >= 0x20 { // sx19_3: the US 32-point ban
        covered!("sx19_3 us points ban");
        matches += 1;
        if matches == 1 {
            reason = 0;
            amount = 0xb;
        }
        write_u16(entry + ruling_body_entry::DISCIPLINE_POINTS, points.wrapping_sub(0xb));
    } else if points >= 0x15 {
        covered!("sx19_3 us 21 points");
        matches += 1;
        if matches == 1 {
            reason = 0;
            amount = 0x15;
        }
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules: own_rules(&USA_RULING_BODY), start: ban_start_today() });
    CardVerdict { pending: Vec::new(), ban }
}

/// Applies a player's cards of one match under the US rules (`usa_card_verdict`): nothing without a booking or a red
/// card; the ban, when there is one, starts today and goes to the discipline with its news.
/// Original: `RbUsa::process_match_cards` (0x7e4140, rb_usa.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_usa_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| usa_card_verdict(MatchRecordRow(row), e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_spain_cup_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| spain_cup_card_verdict(this, MatchRecordRow(row), fixture, e), "rb_spain_cup_process_match_cards");
}

pub(crate) unsafe extern "thiscall" fn compare_rb_usa_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(this, file);
    compare_ruling_body_class_built(r, this, &USA_RULING_BODY, "rb_usa_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_usa_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| usa_card_verdict(MatchRecordRow(row), e), "rb_usa_process_match_cards");
}
