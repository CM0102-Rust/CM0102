// ---------- sx19_6: the Portuguese body's card rules and the Swedish league ruling body (constructor and card rules) (ruling_body/rb_portugal.cpp, rb_sweden_league.cpp) ----------

/// The Portuguese card rules' verdict (`stepped_card_verdict`): a red card queues a points ban from tomorrow and bans
/// for one match; every fifth booking (one per match) bans for one more (amount 5); under the Portuguese rule set.
pub(crate) unsafe fn portugal_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize) -> CardVerdict {
    covered!("sx19_6 portuguese cards");
    stepped_card_verdict(this, row, fixture, entry, &PORTUGAL_RULING_BODY, 5) // sx19_6: every fifth Portuguese booking
}

/// Applies a player's cards of one match under the Portuguese rules (`portugal_card_verdict`).
/// Original: `RbPortugal::process_match_cards` (0x7e1ab0, rb_portugal.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_portugal_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| portugal_card_verdict(this, MatchRecordRow(row), fixture, e));
}

/// The Swedish league ruling body: vtable 0x96eeb4, record slot 0, rule set 0x1c, ban scale 12, Sweden in Europe,
/// league rule 0x1c, cup rule 0x1d, the European rule 1.
pub(crate) const SWEDEN_LEAGUE_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96eeb4, record_slot: 0, rule_set: 0x1c, scale: 0xc,
    nation: Some(KEY_NATION_SWEDEN), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0x1c, cup_rule: 0x1d, continental_rule: 1 };

/// Builds the Swedish league ruling body (`ruling_body_class_construct` with `SWEDEN_LEAGUE_RULING_BODY`). Returns this.
/// Original: `RbSwedenLeague::RbSwedenLeague` (0x7e3650, rb_sweden_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_sweden_league_construct(this: usize, file: usize) -> usize {
    covered!("sx19_6 swedish league body built");
    ruling_body_class_construct(this, file, &SWEDEN_LEAGUE_RULING_BODY)
}

/// The Swedish card rules' verdict (league and cup). Exactly one red card queues a points ban
/// (`pending_points_ban`, from tomorrow); it also bans for one match (reason 2, amount 1) in the league, and in the cup
/// only when the player has exactly two bookings in the match (a second yellow). A booking adds one to the entry's count
/// (one per match): the third bans for one more match with amount 3, and from the fifth every odd count with amount 2
/// (reason 1). The ban starts today, under the body's own rule set.
pub(crate) unsafe fn sweden_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize, class: &RulingBodyClass, cup: bool) -> CardVerdict {
    let rules = own_rules(class);
    let (mut matches, mut reason, mut amount, mut pending) = (0u8, 0xffu8, -1i16, Vec::new());
    if row.red_card() == 1 {
        covered!("sx19_6 swedish red card");
        pending.extend(pending_points_ban(this, row, fixture, rules, || ban_start_in(1)));
        if !cup || row.yellow_cards() == 2 {
            matches = 1;
            reason = 2;
            amount = 1;
        }
    }
    if row.yellow_cards() != 0 {
        let count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(1);
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
        if count == 3 || (count > 4 && count % 2 == 1) { // sx19_6: the Swedish booking bans
            covered!("sx19_6 swedish booking ban");
            matches += 1;
            reason = 1;
            amount = if count == 3 { 3 } else { 2 };
        }
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules, start: ban_start_today() });
    CardVerdict { pending, ban }
}

/// Applies a player's cards of one match under the Swedish league rules (`sweden_card_verdict`): nothing without a
/// booking or a red card; the pending ban goes to the body's list and the ban, when there is one, starts today and goes
/// to the discipline with its news.
/// Original: `RbSwedenLeague::process_match_cards` (0x7e36a0, rb_sweden_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_sweden_league_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| sweden_card_verdict(this, MatchRecordRow(row), fixture, e, &SWEDEN_LEAGUE_RULING_BODY, false));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_portugal_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| portugal_card_verdict(this, MatchRecordRow(row), fixture, e), "rb_portugal_process_match_cards");
}

pub(crate) unsafe extern "thiscall" fn compare_rb_sweden_league_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(this, file);
    compare_ruling_body_class_built(r, this, &SWEDEN_LEAGUE_RULING_BODY, "rb_sweden_league_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_sweden_league_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%);
    compare_card_rules(this, row, || o(this, row, fixture),
        |e| sweden_card_verdict(this, MatchRecordRow(row), fixture, e, &SWEDEN_LEAGUE_RULING_BODY, false), "rb_sweden_league_process_match_cards");
}
