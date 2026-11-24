// ---------- sx18_22: the Argentine ruling body (constructor and card rules) and the South Korean body's constructor (ruling_body/rb_argentina.cpp, rb_korea.cpp) ----------

/// The Argentine ruling body: vtable 0x96e9f4, record slot 0, rule set 3, ban scale 10, Argentina in South America,
/// league rule 3, no cup rule, the South American rule 2.
pub(crate) const ARGENTINA_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96e9f4, record_slot: 0, rule_set: 3, scale: 0xa,
    nation: Some(KEY_NATION_ARGENTINA), continent: Some(KEY_CONTINENT_SOUTH_AMERICA), league_rule: 3, cup_rule: 0xff, continental_rule: 2 };

/// The South Korean ruling body: vtable 0x96ecf4, record slot 0, rule set 0x2a, ban scale 8, South Korea in Asia,
/// league rule 0x2a, no cup rule, the Asian rule 0x29.
pub(crate) const KOREA_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ecf4, record_slot: 0, rule_set: 0x2a, scale: 8,
    nation: Some(KEY_NATION_SOUTH_KOREA), continent: Some(KEY_CONTINENT_ASIA), league_rule: 0x2a, cup_rule: 0xff, continental_rule: 0x29 };

/// Builds the Argentine ruling body (`ruling_body_class_construct` with `ARGENTINA_RULING_BODY`). Returns this.
/// Original: `RbArgentina::RbArgentina` (0x7db100, rb_argentina.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_argentina_construct(this: usize, file: usize) -> usize {
    covered!("sx18_22 argentine body built");
    ruling_body_class_construct(this, file, &ARGENTINA_RULING_BODY)
}

/// The Argentine card rules (`plain_card_verdict`): one match for a red card and one when the entry's count, which
/// grows by all the match's bookings, reaches a multiple of five (amount 5), under the Argentine rule set alone.
pub(crate) unsafe fn argentina_card_verdict(row: MatchRecordRow, entry: usize) -> CardVerdict {
    covered!("sx18_22 argentine cards");
    plain_card_verdict(row, entry, true, 5, [ARGENTINA_RULING_BODY.rule_set, 0xff, 0xff, 0xff]) // sx18_22: every fifth Argentine booking
}

/// Applies a player's cards of one match under the Argentine rules (`argentina_card_verdict`).
/// Original: `RbArgentina::process_match_cards` (0x7db150, rb_argentina.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_argentina_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| argentina_card_verdict(MatchRecordRow(row), e));
}

/// Builds the South Korean ruling body (`ruling_body_class_construct` with `KOREA_RULING_BODY`); with data older than
/// version 0x3e047 the discipline passes it no file. Returns this.
/// Original: `RbKorea::RbKorea` (0x7e0730, rb_korea.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_korea_construct(this: usize, file: usize) -> usize {
    covered!("sx18_22 korean body built");
    ruling_body_class_construct(this, file, &KOREA_RULING_BODY)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_argentina_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_ruling_body_class_built(r, this, &ARGENTINA_RULING_BODY, "rb_argentina_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_argentina_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| argentina_card_verdict(MatchRecordRow(row), e), "rb_argentina_process_match_cards");
}

pub(crate) unsafe extern "thiscall" fn compare_rb_korea_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%)(this, file);
    compare_ruling_body_class_built(r, this, &KOREA_RULING_BODY, "rb_korea_construct")
}
