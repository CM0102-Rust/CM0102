// ---------- sx18_9: the Brazilian card rules, national and regional, and the regional body's constructor (ruling_body/rb_brazil_national.cpp, rb_brazil_regional.cpp) ----------

/// The Brazilian regional ruling body: vtable 0x96eab4, record slot 1, rule set 7, ban scale 5, Brazil in South
/// America, league rule 6, cup rule 7, the South American rule 2.
pub(crate) const BRAZIL_REGIONAL_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96eab4, record_slot: 1, rule_set: 7, scale: 5,
    nation: Some(KEY_NATION_BRAZIL), continent: Some(KEY_CONTINENT_SOUTH_AMERICA), league_rule: 6, cup_rule: 7, continental_rule: 2 };

/// The Brazilian card rules' verdict (national and regional, with their `rules`). A red card (exactly one) counts in
/// the entry; from the second one it also queues a points ban (`pending_points_ban`) from a week after today; it bans
/// for one match on the spot (reason 2, amount 1) and the match's bookings are not counted. Otherwise the match's
/// bookings add to the entry's count and every third one bans for one match (reason 1, amount 3), taking three off the
/// count. The ban starts today.
pub(crate) unsafe fn brazil_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize, rules: [u8; 4]) -> CardVerdict {
    let mut pending = Vec::new();
    let (reason, amount) = if row.red_card() == 1 {
        covered!("sx18_9 brazilian red card");
        let reds = read_u8(entry + ruling_body_entry::RED_CARDS).wrapping_add(1);
        write_u8(entry + ruling_body_entry::RED_CARDS, reds);
        if reds > 1 { // sx18_9: the points ban from the second red card
            pending.extend(pending_points_ban(this, row, fixture, rules, || ban_start_in(7)));
        }
        (2, 1)
    } else {
        let bookings = row.yellow_cards();
        if bookings == 0 {
            return CardVerdict { pending, ban: None };
        }
        let count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(bookings);
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
        if count % 3 != 0 {
            return CardVerdict { pending, ban: None };
        }
        covered!("sx18_9 brazilian booking ban");
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count.wrapping_sub(3));
        (1, 3)
    };
    CardVerdict { pending, ban: Some(ImmediateBan { matches: 1, reason, amount, rules, start: ban_start_today() }) }
}

/// The Brazilian national rules (`brazil_card_verdict` under the national rule set in the first slot).
unsafe fn brazil_national_card_verdict(this: usize, row: usize, fixture: usize, e: usize) -> CardVerdict {
    brazil_card_verdict(this, MatchRecordRow(row), fixture, e, [BRAZIL_NATIONAL_RULING_BODY.rule_set, 0xff, 0xff, 0xff])
}

/// The Brazilian regional rules (`brazil_card_verdict` under the regional rule set in the second slot).
unsafe fn brazil_regional_card_verdict(this: usize, row: usize, fixture: usize, e: usize) -> CardVerdict {
    brazil_card_verdict(this, MatchRecordRow(row), fixture, e, [0xff, BRAZIL_REGIONAL_RULING_BODY.rule_set, 0xff, 0xff])
}

/// Applies a player's cards of one match under the Brazilian national rules (`brazil_card_verdict`).
/// Original: `RbBrazilNational::process_match_cards` (0x7dc0d0, rb_brazil_national.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_brazil_national_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| brazil_national_card_verdict(this, row, fixture, e));
}

/// Builds the Brazilian regional ruling body (`ruling_body_class_construct` with `BRAZIL_REGIONAL_RULING_BODY`).
/// Returns this.
/// Original: `RbBrazilRegional::RbBrazilRegional` (0x7dc3c0, rb_brazil_regional.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_brazil_regional_construct(this: usize, file: usize) -> usize {
    covered!("sx18_9 brazilian regional body built");
    ruling_body_class_construct(this, file, &BRAZIL_REGIONAL_RULING_BODY)
}

/// Applies a player's cards of one match under the Brazilian regional rules (`brazil_card_verdict`).
/// Original: `RbBrazilRegional::process_match_cards` (0x7dc410, rb_brazil_regional.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_brazil_regional_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| brazil_regional_card_verdict(this, row, fixture, e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_brazil_national_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| brazil_national_card_verdict(this, row, fixture, e), "rb_brazil_national_process_match_cards");
}

pub(crate) unsafe extern "thiscall" fn compare_rb_brazil_regional_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(this, file);
    compare_ruling_body_class_built(r, this, &BRAZIL_REGIONAL_RULING_BODY, "rb_brazil_regional_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_brazil_regional_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| brazil_regional_card_verdict(this, row, fixture, e), "rb_brazil_regional_process_match_cards");
}
