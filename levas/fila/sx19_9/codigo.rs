// ---------- sx19_9: the Russian body's card rules and the Turkish league ruling body (constructor and card rules) (ruling_body/rb_russia.cpp, rb_turkey_league.cpp) ----------

/// The Russian card rules' verdict. All the match's bookings add to the entry's count, which drops by four when it
/// reaches four: the cycle of four bookings never bans. Exactly one red card queues a points ban
/// (`pending_points_ban`, from the first weekday 3 after today) and bans for one match (reason 2, amount 1), from
/// today, under the Russian rule set.
pub(crate) unsafe fn russia_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize) -> CardVerdict {
    let rules = own_rules(&RUSSIA_RULING_BODY);
    let mut count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(row.yellow_cards());
    if count >= 4 { // sx19_9: the Russian cycle of four bookings
        covered!("sx19_9 russian booking cycle");
        count -= 4;
    }
    write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
    if row.red_card() != 1 {
        return CardVerdict { pending: Vec::new(), ban: None };
    }
    covered!("sx19_9 russian red card");
    let pending = pending_points_ban(this, row, fixture, rules, || first_weekday_after_today(3)).into_iter().collect();
    let ban = ImmediateBan { matches: 1, reason: 2, amount: 1, rules, start: ban_start_today() };
    CardVerdict { pending, ban: Some(ban) }
}

/// Applies a player's cards of one match under the Russian rules (`russia_card_verdict`): nothing without a booking or
/// a red card.
/// Original: `RbRussia::process_match_cards` (0x7e1e50, rb_russia.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_russia_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| russia_card_verdict(this, MatchRecordRow(row), fixture, e));
}

/// The Turkish league ruling body: vtable 0x96eef4, record slot 0, rule set 0x25, ban scale 7, Turkey in Europe, league
/// rule 0x25, cup rule 0x26, the European rule 1.
pub(crate) const TURKEY_LEAGUE_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96eef4, record_slot: 0, rule_set: 0x25, scale: 7,
    nation: Some(KEY_NATION_TURKEY), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0x25, cup_rule: 0x26, continental_rule: 1 };

/// Builds the Turkish league ruling body (`ruling_body_class_construct` with `TURKEY_LEAGUE_RULING_BODY`). Returns this.
/// Original: `RbTurkeyLeague::RbTurkeyLeague` (0x7e3d60, rb_turkey_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_turkey_league_construct(this: usize, file: usize) -> usize {
    covered!("sx19_9 turkish league body built");
    ruling_body_class_construct(this, file, &TURKEY_LEAGUE_RULING_BODY)
}

/// The Turkish league card rules' verdict (`stepped_card_verdict`): a red card queues a points ban from tomorrow and
/// bans for one match; every fourth booking (one per match) bans for one more (amount 4); under the league rule set.
pub(crate) unsafe fn turkey_league_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize) -> CardVerdict {
    covered!("sx19_9 turkish league cards");
    stepped_card_verdict(this, row, fixture, entry, &TURKEY_LEAGUE_RULING_BODY, 4) // sx19_9: every fourth Turkish league booking
}

/// Applies a player's cards of one match under the Turkish league rules (`turkey_league_card_verdict`).
/// Original: `RbTurkeyLeague::process_match_cards` (0x7e3db0, rb_turkey_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_turkey_league_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| turkey_league_card_verdict(this, MatchRecordRow(row), fixture, e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_russia_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| russia_card_verdict(this, MatchRecordRow(row), fixture, e), "rb_russia_process_match_cards");
}

pub(crate) unsafe extern "thiscall" fn compare_rb_turkey_league_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(this, file);
    compare_ruling_body_class_built(r, this, &TURKEY_LEAGUE_RULING_BODY, "rb_turkey_league_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_turkey_league_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| turkey_league_card_verdict(this, MatchRecordRow(row), fixture, e), "rb_turkey_league_process_match_cards");
}
