// ---------- sx18_7: the Italian cup card rules, the French and the Brazilian national ruling bodies' constructors (ruling_body/rb_italy_cup.cpp, rb_france.cpp, rb_brazil_national.cpp) ----------

/// The French ruling body: vtable 0x96eb94, record slot 0, rule set 0xc, ban scale 5, France in Europe, league rule
/// 0xc, no cup rule, the European rule 1.
pub(crate) const FRANCE_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96eb94, record_slot: 0, rule_set: 0xc, scale: 5,
    nation: Some(KEY_NATION_FRANCE), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0xc, cup_rule: 0xff, continental_rule: 1 };

/// The Brazilian national ruling body: vtable 0x96ea94, record slot 0, rule set 6, ban scale 5, Brazil in South
/// America, league rule 6, cup rule 7, the South American rule 2.
pub(crate) const BRAZIL_NATIONAL_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ea94, record_slot: 0, rule_set: 6, scale: 5,
    nation: Some(KEY_NATION_BRAZIL), continent: Some(KEY_CONTINENT_SOUTH_AMERICA), league_rule: 6, cup_rule: 7, continental_rule: 2 };

/// The Italian cup card rules' verdict. A red card (exactly one) bans for one match (reason 2, amount 1) and, for an
/// offence code above 9 (`red_card_offence`), queues a points ban from the first weekday 3 after today. A booking adds
/// one to the entry's count and every second one bans for one more match (reason 1, amount 2). The ban starts today,
/// under the cup rule set in the second slot.
pub(crate) unsafe fn italy_cup_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize) -> CardVerdict {
    let rules = [0xff, ITALY_CUP_RULING_BODY.rule_set, 0xff, 0xff];
    let (mut matches, mut reason, mut amount, mut pending) = (0u8, 0xffu8, 0i16, Vec::new());
    if row.red_card() == 1 {
        covered!("sx18_7 italian cup red card");
        if red_card_offence(row) > 9 {
            pending.extend(pending_points_ban(this, row, fixture, rules, || first_weekday_after_today(3)));
        }
        matches = 1;
        reason = 2;
        amount = 1;
    }
    if row.yellow_cards() != 0 {
        let count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(1);
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
        if count % 2 == 0 { // sx18_7: every second Italian cup booking
            covered!("sx18_7 italian cup booking ban");
            matches += 1;
            reason = 1;
            amount = 2;
        }
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules, start: ban_start_today() });
    CardVerdict { pending, ban }
}

/// Applies a player's cards of one match under the Italian cup rules (`italy_cup_card_verdict`).
/// Original: `RbItalyCup::process_match_cards` (0x7df9b0, rb_italy_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_italy_cup_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| italy_cup_card_verdict(this, MatchRecordRow(row), fixture, e));
}

/// Builds the French ruling body (`ruling_body_class_construct` with `FRANCE_RULING_BODY`). Returns this.
/// Original: `RbFrance::RbFrance` (0x7ddcc0, rb_france.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_france_construct(this: usize, file: usize) -> usize {
    covered!("sx18_7 french body built");
    ruling_body_class_construct(this, file, &FRANCE_RULING_BODY)
}

/// Builds the Brazilian national ruling body (`ruling_body_class_construct` with `BRAZIL_NATIONAL_RULING_BODY`).
/// Returns this.
/// Original: `RbBrazilNational::RbBrazilNational` (0x7dc080, rb_brazil_national.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_brazil_national_construct(this: usize, file: usize) -> usize {
    covered!("sx18_7 brazilian national body built");
    ruling_body_class_construct(this, file, &BRAZIL_NATIONAL_RULING_BODY)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_italy_cup_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| italy_cup_card_verdict(this, MatchRecordRow(row), fixture, e), "rb_italy_cup_process_match_cards");
}

pub(crate) unsafe extern "thiscall" fn compare_rb_france_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(this, file);
    compare_ruling_body_class_built(r, this, &FRANCE_RULING_BODY, "rb_france_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_brazil_national_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%)(this, file);
    compare_ruling_body_class_built(r, this, &BRAZIL_NATIONAL_RULING_BODY, "rb_brazil_national_construct")
}
