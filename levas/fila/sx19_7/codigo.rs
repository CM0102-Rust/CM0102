// ---------- sx19_7: the Swedish cup ruling body (constructor and card rules) (ruling_body/rb_sweden_cup.cpp) ----------

/// The Swedish cup ruling body: vtable 0x96ee94, record slot 1, rule set 0x1d, ban scale 12, Sweden in Europe, league
/// rule 0x1c, cup rule 0x1d, the European rule 1.
pub(crate) const SWEDEN_CUP_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ee94, record_slot: 1, rule_set: 0x1d, scale: 0xc,
    nation: Some(KEY_NATION_SWEDEN), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0x1c, cup_rule: 0x1d, continental_rule: 1 };

/// Builds the Swedish cup ruling body (`ruling_body_class_construct` with `SWEDEN_CUP_RULING_BODY`). Returns this.
/// Original: `RbSwedenCup::RbSwedenCup` (0x7e32d0, rb_sweden_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_sweden_cup_construct(this: usize, file: usize) -> usize {
    covered!("sx19_7 swedish cup body built");
    ruling_body_class_construct(this, file, &SWEDEN_CUP_RULING_BODY)
}

/// The Swedish cup card rules' verdict (`sweden_card_verdict` with `cup`: a red card bans on the spot only after a
/// second yellow).
pub(crate) unsafe fn sweden_cup_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize) -> CardVerdict {
    covered!("sx19_7 swedish cup cards");
    sweden_card_verdict(this, row, fixture, entry, &SWEDEN_CUP_RULING_BODY, true) // sx19_7: the cup's red card rule
}

/// Applies a player's cards of one match under the Swedish cup rules (`sweden_cup_card_verdict`).
/// Original: `RbSwedenCup::process_match_cards` (0x7e3320, rb_sweden_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_sweden_cup_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| sweden_cup_card_verdict(this, MatchRecordRow(row), fixture, e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_sweden_cup_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_ruling_body_class_built(r, this, &SWEDEN_CUP_RULING_BODY, "rb_sweden_cup_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_sweden_cup_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| sweden_cup_card_verdict(this, MatchRecordRow(row), fixture, e), "rb_sweden_cup_process_match_cards");
}
