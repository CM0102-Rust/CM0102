// ---------- sx19_2: the Spanish league ruling body (constructor and card rules) and the Spanish cup body's constructor (ruling_body/rb_spain_league.cpp, rb_spain_cup.cpp) ----------

/// The Spanish league ruling body: vtable 0x96ee74, record slot 0, rule set 0x1a, ban scale 10, Spain in Europe, league
/// rule 0x1a, cup rule 0x1b, the European rule 1.
pub(crate) const SPAIN_LEAGUE_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ee74, record_slot: 0, rule_set: 0x1a, scale: 0xa,
    nation: Some(KEY_NATION_SPAIN), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0x1a, cup_rule: 0x1b, continental_rule: 1 };

/// The Spanish cup ruling body: vtable 0x96ee54, record slot 1, rule set 0x1b, ban scale 10, Spain in Europe, league
/// rule 0x1a, cup rule 0x1b, the European rule 1.
pub(crate) const SPAIN_CUP_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ee54, record_slot: 1, rule_set: 0x1b, scale: 0xa,
    nation: Some(KEY_NATION_SPAIN), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0x1a, cup_rule: 0x1b, continental_rule: 1 };

/// Builds the Spanish league ruling body (`ruling_body_class_construct` with `SPAIN_LEAGUE_RULING_BODY`). Returns this.
/// Original: `RbSpainLeague::RbSpainLeague` (0x7e2f80, rb_spain_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_spain_league_construct(this: usize, file: usize) -> usize {
    covered!("sx19_2 spanish league body built");
    ruling_body_class_construct(this, file, &SPAIN_LEAGUE_RULING_BODY)
}

/// Builds the Spanish cup ruling body (`ruling_body_class_construct` with `SPAIN_CUP_RULING_BODY`). Returns this.
/// Original: `RbSpainCup::RbSpainCup` (0x7e2c30, rb_spain_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_spain_cup_construct(this: usize, file: usize) -> usize {
    covered!("sx19_2 spanish cup body built");
    ruling_body_class_construct(this, file, &SPAIN_CUP_RULING_BODY)
}

/// The Spanish league card rules' verdict (`stepped_card_verdict`): a red card queues a points ban from tomorrow and
/// bans for one match; every fifth booking (one per match) bans for one more (amount 5); under the league rule set.
pub(crate) unsafe fn spain_league_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize) -> CardVerdict {
    covered!("sx19_2 spanish league cards");
    stepped_card_verdict(this, row, fixture, entry, &SPAIN_LEAGUE_RULING_BODY, 5) // sx19_2: every fifth Spanish league booking
}

/// Applies a player's cards of one match under the Spanish league rules (`spain_league_card_verdict`): nothing without
/// a booking or a red card; the pending ban goes to the body's list and the ban, when there is one, starts today and
/// goes to the discipline with its news.
/// Original: `RbSpainLeague::process_match_cards` (0x7e2fd0, rb_spain_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_spain_league_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| spain_league_card_verdict(this, MatchRecordRow(row), fixture, e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_spain_league_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_ruling_body_class_built(r, this, &SPAIN_LEAGUE_RULING_BODY, "rb_spain_league_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_spain_league_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| spain_league_card_verdict(this, MatchRecordRow(row), fixture, e), "rb_spain_league_process_match_cards");
}

pub(crate) unsafe extern "thiscall" fn compare_rb_spain_cup_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%)(this, file);
    compare_ruling_body_class_built(r, this, &SPAIN_CUP_RULING_BODY, "rb_spain_cup_construct")
}
