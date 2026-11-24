// ---------- sx19_4: the Oceanian ruling body (constructor and card rules) (ruling_body/rb_oceania.cpp) ----------

/// The Oceanian ruling body: vtable 0x96ed74, record slot 2, rule set 0x23, ban scale 10, no nation, Oceania, no league
/// nor cup rule, itself as the continental rule.
pub(crate) const OCEANIA_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ed74, record_slot: 2, rule_set: 0x23, scale: 0xa,
    nation: None, continent: Some(KEY_CONTINENT_OCEANIA), league_rule: 0xff, cup_rule: 0xff, continental_rule: 0x23 };

/// Builds the Oceanian ruling body (`ruling_body_class_construct` with `OCEANIA_RULING_BODY`). Returns this.
/// Original: `RbOceania::RbOceania` (0x7e13d0, rb_oceania.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_oceania_construct(this: usize, file: usize) -> usize {
    covered!("sx19_4 oceanian body built");
    ruling_body_class_construct(this, file, &OCEANIA_RULING_BODY)
}

/// Oceania's card rules (`plain_card_verdict`): one match for a red card and one for every third booking (counted one
/// per match, amount 3), from today, under the Oceanian rule set in the confederation slot (rules ff ff 23 ff).
pub(crate) unsafe fn oceania_card_verdict(row: MatchRecordRow, entry: usize) -> CardVerdict {
    covered!("sx19_4 oceanian cards");
    plain_card_verdict(row, entry, false, 3, own_rules(&OCEANIA_RULING_BODY)) // sx19_4: every third Oceanian booking
}

/// Applies a player's cards of one match under the Oceanian rules (`oceania_card_verdict`): nothing without a booking
/// or a red card; the ban, when there is one, starts today and goes to the discipline with its news.
/// Original: `RbOceania::process_match_cards` (0x7e1420, rb_oceania.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_oceania_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| oceania_card_verdict(MatchRecordRow(row), e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_oceania_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_ruling_body_class_built(r, this, &OCEANIA_RULING_BODY, "rb_oceania_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_oceania_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| oceania_card_verdict(MatchRecordRow(row), e), "rb_oceania_process_match_cards");
}
