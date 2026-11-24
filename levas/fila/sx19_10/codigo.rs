// ---------- sx19_10: the Turkish cup ruling body (constructor and card rules) (ruling_body/rb_turkey_cup.cpp) ----------

/// The Turkish cup ruling body: vtable 0x96eed4, record slot 1, rule set 0x26, ban scale 10, Turkey in Europe, league
/// rule 0x25, cup rule 0x26, the European rule 1.
pub(crate) const TURKEY_CUP_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96eed4, record_slot: 1, rule_set: 0x26, scale: 0xa,
    nation: Some(KEY_NATION_TURKEY), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0x25, cup_rule: 0x26, continental_rule: 1 };

/// Builds the Turkish cup ruling body (`ruling_body_class_construct` with `TURKEY_CUP_RULING_BODY`). Returns this.
/// Original: `RbTurkeyCup::RbTurkeyCup` (0x7e3a10, rb_turkey_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_turkey_cup_construct(this: usize, file: usize) -> usize {
    covered!("sx19_10 turkish cup body built");
    ruling_body_class_construct(this, file, &TURKEY_CUP_RULING_BODY)
}

/// The Turkish cup card rules' verdict (`stepped_card_verdict`): a red card queues a points ban from tomorrow and bans
/// for one match; every fourth booking (one per match) bans for one more (amount 4); under the cup rule set.
pub(crate) unsafe fn turkey_cup_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize) -> CardVerdict {
    covered!("sx19_10 turkish cup cards");
    stepped_card_verdict(this, row, fixture, entry, &TURKEY_CUP_RULING_BODY, 4) // sx19_10: every fourth Turkish cup booking
}

/// Applies a player's cards of one match under the Turkish cup rules (`turkey_cup_card_verdict`).
/// Original: `RbTurkeyCup::process_match_cards` (0x7e3a60, rb_turkey_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_turkey_cup_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| turkey_cup_card_verdict(this, MatchRecordRow(row), fixture, e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_turkey_cup_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_ruling_body_class_built(r, this, &TURKEY_CUP_RULING_BODY, "rb_turkey_cup_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_turkey_cup_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| turkey_cup_card_verdict(this, MatchRecordRow(row), fixture, e), "rb_turkey_cup_process_match_cards");
}
