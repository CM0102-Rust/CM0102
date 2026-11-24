// ---------- sx19_13: the Welsh ruling body (constructor and card rules) (ruling_body/rb_wales.cpp) ----------

/// The Welsh ruling body: vtable 0x96ef34, record slot 0, rule set 0x28, ban scale 8, Wales in Europe, league rule
/// 0x28, no cup rule, the European rule 1.
pub(crate) const WALES_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ef34, record_slot: 0, rule_set: 0x28, scale: 8,
    nation: Some(KEY_NATION_WALES), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0x28, cup_rule: 0xff, continental_rule: 1 };

/// Builds the Welsh ruling body (`ruling_body_class_construct` with `WALES_RULING_BODY`). Returns this.
/// Original: `RbWales::RbWales` (0x7e43c0, rb_wales.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_wales_construct(this: usize, file: usize) -> usize {
    covered!("sx19_13 welsh body built");
    ruling_body_class_construct(this, file, &WALES_RULING_BODY)
}

/// The Welsh card rules' verdict (`stepped_card_verdict`): a red card queues a points ban from tomorrow and bans for one
/// match; every fifth booking (one per match) bans for one more (amount 5); under the Welsh rule set.
pub(crate) unsafe fn wales_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize) -> CardVerdict {
    covered!("sx19_13 welsh cards");
    stepped_card_verdict(this, row, fixture, entry, &WALES_RULING_BODY, 5) // sx19_13: every fifth Welsh booking
}

/// Applies a player's cards of one match under the Welsh rules (`wales_card_verdict`).
/// Original: `RbWales::process_match_cards` (0x7e4410, rb_wales.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_wales_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| wales_card_verdict(this, MatchRecordRow(row), fixture, e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_wales_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_ruling_body_class_built(r, this, &WALES_RULING_BODY, "rb_wales_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_wales_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| wales_card_verdict(this, MatchRecordRow(row), fixture, e), "rb_wales_process_match_cards");
}
