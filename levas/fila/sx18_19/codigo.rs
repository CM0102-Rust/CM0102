// ---------- sx18_19: the Finnish cup card rules and the Norwegian cup ruling body (constructor and card rules) (ruling_body/rb_finland_cup.cpp, rb_norway_cup.cpp) ----------

/// The Norwegian cup ruling body: vtable 0x96ed34, record slot 1, rule set 0x16, ban scale 10, Norway in Europe, league
/// rule 0x15, cup rule 0x16, the European rule 1.
pub(crate) const NORWAY_CUP_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ed34, record_slot: 1, rule_set: 0x16, scale: 0xa,
    nation: Some(KEY_NATION_NORWAY), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0x15, cup_rule: 0x16, continental_rule: 1 };

/// The Finnish cup rules, under the cup rule set in the second slot.
unsafe fn finland_cup_card_verdict(this: usize, row: usize, fixture: usize, e: usize) -> CardVerdict {
    finland_card_verdict(this, MatchRecordRow(row), fixture, e, [0xff, FINLAND_CUP_RULING_BODY.rule_set, 0xff, 0xff]) // sx18_19: the Finnish cup rules
}

/// Applies a player's cards of one match under the Finnish cup rules (`finland_card_verdict`).
/// Original: `RbFinlandCup::process_match_cards` (0x7dd6b0, rb_finland_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_finland_cup_process_match_cards(this: usize, row: usize, fixture: usize) {
    covered!("sx18_19 finnish cup cards");
    run_card_rules(this, row, fixture, |e| finland_cup_card_verdict(this, row, fixture, e));
}

/// Builds the Norwegian cup ruling body (`ruling_body_class_construct` with `NORWAY_CUP_RULING_BODY`). Returns this.
/// Original: `RbNorwayCup::RbNorwayCup` (0x7e0ef0, rb_norway_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_norway_cup_construct(this: usize, file: usize) -> usize {
    covered!("sx18_19 norwegian cup body built");
    ruling_body_class_construct(this, file, &NORWAY_CUP_RULING_BODY)
}

/// The Norwegian cup card rules' verdict: always a one-match ban, from today, under the cup rule set in the second slot.
/// A red card (exactly one) gives it (reason 2, amount 1) and the match's bookings are not counted. Otherwise a booking
/// adds one to the entry's count and every booking from the second gives it (reason 1, amount 2 at the second, then 1).
pub(crate) unsafe fn norway_cup_card_verdict(row: MatchRecordRow, entry: usize) -> CardVerdict {
    let (reason, amount) = if row.red_card() == 1 {
        covered!("sx18_19 norwegian cup red card");
        (2, 1)
    } else {
        if row.yellow_cards() == 0 {
            return CardVerdict { pending: Vec::new(), ban: None };
        }
        let count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(1);
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
        if count <= 1 { // sx18_19: the first Norwegian cup booking is free
            return CardVerdict { pending: Vec::new(), ban: None };
        }
        covered!("sx18_19 norwegian cup booking ban");
        (1, if count == 2 { 2 } else { 1 })
    };
    let rules = [0xff, NORWAY_CUP_RULING_BODY.rule_set, 0xff, 0xff];
    CardVerdict { pending: Vec::new(), ban: Some(ImmediateBan { matches: 1, reason, amount, rules, start: ban_start_today() }) }
}

/// Applies a player's cards of one match under the Norwegian cup rules (`norway_cup_card_verdict`).
/// Original: `RbNorwayCup::process_match_cards` (0x7e0f40, rb_norway_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_norway_cup_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| norway_cup_card_verdict(MatchRecordRow(row), e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_finland_cup_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| finland_cup_card_verdict(this, row, fixture, e), "rb_finland_cup_process_match_cards");
}

pub(crate) unsafe extern "thiscall" fn compare_rb_norway_cup_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(this, file);
    compare_ruling_body_class_built(r, this, &NORWAY_CUP_RULING_BODY, "rb_norway_cup_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_norway_cup_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| norway_cup_card_verdict(MatchRecordRow(row), e), "rb_norway_cup_process_match_cards");
}
