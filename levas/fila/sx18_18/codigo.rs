// ---------- sx18_18: the Finnish league ruling body (constructor and card rules) and the Finnish cup body's constructor (ruling_body/rb_finland_league.cpp, rb_finland_cup.cpp) ----------

/// The Finnish league ruling body: vtable 0x96eb74, record slot 0, rule set 0xa, ban scale 7, Finland in Europe, league
/// rule 0xa, cup rule 0xb, the European rule 1.
pub(crate) const FINLAND_LEAGUE_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96eb74, record_slot: 0, rule_set: 0xa, scale: 7,
    nation: Some(KEY_NATION_FINLAND), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0xa, cup_rule: 0xb, continental_rule: 1 };

/// The Finnish cup ruling body: vtable 0x96eb54, record slot 1, rule set 0xb, ban scale 7, Finland in Europe, league
/// rule 0xa, cup rule 0xb, the European rule 1.
pub(crate) const FINLAND_CUP_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96eb54, record_slot: 1, rule_set: 0xb, scale: 7,
    nation: Some(KEY_NATION_FINLAND), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0xa, cup_rule: 0xb, continental_rule: 1 };

/// The Finnish card rules' verdict (league and cup, with their `rules`). A booking adds one to the entry's count and
/// every third bans for one match (reason 1, amount 3). A red card (exactly one) queues a points ban
/// (`pending_points_ban`) from a week after today and bans for one more match (reason 2, amount 1). The ban starts
/// today.
pub(crate) unsafe fn finland_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize, rules: [u8; 4]) -> CardVerdict {
    let (mut matches, mut reason, mut amount, mut pending) = (0u8, 0xffu8, 0i16, Vec::new());
    if row.yellow_cards() != 0 {
        let count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(1);
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
        if count % 3 == 0 { // sx18_18: every third Finnish booking
            covered!("sx18_18 finnish booking ban");
            matches = 1;
            reason = 1;
            amount = 3;
        }
    }
    if row.red_card() == 1 {
        covered!("sx18_18 finnish red card");
        pending.extend(pending_points_ban(this, row, fixture, rules, || ban_start_in_weeks(1)));
        matches += 1;
        reason = 2;
        amount = 1;
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules, start: ban_start_today() });
    CardVerdict { pending, ban }
}

/// The Finnish league rules, under the league rule set in the first slot.
unsafe fn finland_league_card_verdict(this: usize, row: usize, fixture: usize, e: usize) -> CardVerdict {
    finland_card_verdict(this, MatchRecordRow(row), fixture, e, [FINLAND_LEAGUE_RULING_BODY.rule_set, 0xff, 0xff, 0xff])
}

/// Builds the Finnish league ruling body (`ruling_body_class_construct` with `FINLAND_LEAGUE_RULING_BODY`). Returns this.
/// Original: `RbFinlandLeague::RbFinlandLeague` (0x7dd990, rb_finland_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_finland_league_construct(this: usize, file: usize) -> usize {
    covered!("sx18_18 finnish league body built");
    ruling_body_class_construct(this, file, &FINLAND_LEAGUE_RULING_BODY)
}

/// Applies a player's cards of one match under the Finnish league rules (`finland_card_verdict`).
/// Original: `RbFinlandLeague::process_match_cards` (0x7dd9e0, rb_finland_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_finland_league_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| finland_league_card_verdict(this, row, fixture, e));
}

/// Builds the Finnish cup ruling body (`ruling_body_class_construct` with `FINLAND_CUP_RULING_BODY`). Returns this.
/// Original: `RbFinlandCup::RbFinlandCup` (0x7dd660, rb_finland_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_finland_cup_construct(this: usize, file: usize) -> usize {
    covered!("sx18_18 finnish cup body built");
    ruling_body_class_construct(this, file, &FINLAND_CUP_RULING_BODY)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_finland_league_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_ruling_body_class_built(r, this, &FINLAND_LEAGUE_RULING_BODY, "rb_finland_league_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_finland_league_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| finland_league_card_verdict(this, row, fixture, e), "rb_finland_league_process_match_cards");
}

pub(crate) unsafe extern "thiscall" fn compare_rb_finland_cup_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%)(this, file);
    compare_ruling_body_class_built(r, this, &FINLAND_CUP_RULING_BODY, "rb_finland_cup_construct")
}
