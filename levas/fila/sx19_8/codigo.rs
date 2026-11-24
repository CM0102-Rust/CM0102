// ---------- sx19_8: the Polish ruling body (constructor and card rules) and the Russian body's constructor (ruling_body/rb_poland.cpp, rb_russia.cpp) ----------

/// The Polish ruling body: vtable 0x96ed94, record slot 0, rule set 0x1f, ban scale 10, Poland in Europe, league rule
/// 0x1f, no cup rule, the European rule 1.
pub(crate) const POLAND_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ed94, record_slot: 0, rule_set: 0x1f, scale: 0xa,
    nation: Some(KEY_NATION_POLAND), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0x1f, cup_rule: 0xff, continental_rule: 1 };

/// The Russian ruling body: vtable 0x96edd4, record slot 0, rule set 0x27, ban scale 8, Russia in Europe, league rule
/// 0x27, no cup rule, the European rule 1.
pub(crate) const RUSSIA_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96edd4, record_slot: 0, rule_set: 0x27, scale: 8,
    nation: Some(KEY_NATION_RUSSIA), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0x27, cup_rule: 0xff, continental_rule: 1 };

/// Builds the Polish ruling body (`ruling_body_class_construct` with `POLAND_RULING_BODY`). Returns this.
/// Original: `RbPoland::RbPoland` (0x7e1610, rb_poland.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_poland_construct(this: usize, file: usize) -> usize {
    covered!("sx19_8 polish body built");
    ruling_body_class_construct(this, file, &POLAND_RULING_BODY)
}

/// Builds the Russian ruling body (`ruling_body_class_construct` with `RUSSIA_RULING_BODY`). Returns this.
/// Original: `RbRussia::RbRussia` (0x7e1e00, rb_russia.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_russia_construct(this: usize, file: usize) -> usize {
    covered!("sx19_8 russian body built");
    ruling_body_class_construct(this, file, &RUSSIA_RULING_BODY)
}

/// The Polish card rules' verdict. Each of the match's bookings adds one to the entry's count: the 4th and the 7th ban
/// for one match each, and from the 10th every even count for two; once there is a ban, each later booking of the match
/// sets reason 1 and the count as the amount. Exactly one red card then queues a points ban (`pending_points_ban`, from
/// the first weekday 3 after today) and adds one match (reason 2, amount 1). The ban starts today, under the Polish
/// rule set.
pub(crate) unsafe fn poland_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize) -> CardVerdict {
    let rules = own_rules(&POLAND_RULING_BODY);
    let (mut matches, mut reason, mut amount, mut pending) = (0u8, 0xffu8, -1i16, Vec::new());
    for _ in 0..row.yellow_cards() as i8 {
        let count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(1);
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
        if count == 4 || count == 7 { // sx19_8: the Polish booking bans
            matches = matches.wrapping_add(1);
        } else if count >= 0xa && count % 2 == 0 {
            matches = matches.wrapping_add(2);
        }
        if matches != 0 {
            covered!("sx19_8 polish booking ban");
            reason = 1;
            amount = count as i8 as i16;
        }
    }
    if row.red_card() == 1 {
        covered!("sx19_8 polish red card");
        pending.extend(pending_points_ban(this, row, fixture, rules, || first_weekday_after_today(3)));
        reason = 2;
        matches = matches.wrapping_add(1);
        amount = 1;
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules, start: ban_start_today() });
    CardVerdict { pending, ban }
}

/// Applies a player's cards of one match under the Polish rules (`poland_card_verdict`): nothing without a booking or a
/// red card; the pending ban goes to the body's list and the ban, when there is one, starts today and goes to the
/// discipline with its news.
/// Original: `RbPoland::process_match_cards` (0x7e1660, rb_poland.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_poland_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| poland_card_verdict(this, MatchRecordRow(row), fixture, e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_poland_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_ruling_body_class_built(r, this, &POLAND_RULING_BODY, "rb_poland_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_poland_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| poland_card_verdict(this, MatchRecordRow(row), fixture, e), "rb_poland_process_match_cards");
}

pub(crate) unsafe extern "thiscall" fn compare_rb_russia_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%)(this, file);
    compare_ruling_body_class_built(r, this, &RUSSIA_RULING_BODY, "rb_russia_construct")
}
