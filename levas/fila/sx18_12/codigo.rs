// ---------- sx18_12: the Belgian league ruling body (constructor and card rules) and the Belgian cup body's constructor (ruling_body/rb_belgium_league.cpp, rb_belgium_cup.cpp) ----------

/// The Belgian league ruling body: vtable 0x96ea74, record slot 0, rule set 4, ban scale 5, Belgium in Europe, league
/// rule 4, cup rule 5, the European rule 1.
pub(crate) const BELGIUM_LEAGUE_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ea74, record_slot: 0, rule_set: 4, scale: 5,
    nation: Some(KEY_NATION_BELGIUM), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 4, cup_rule: 5, continental_rule: 1 };

/// The Belgian cup ruling body: vtable 0x96ea54, record slot 1, rule set 5, ban scale 5, Belgium in Europe, league rule
/// 4, cup rule 5, the European rule 1.
pub(crate) const BELGIUM_CUP_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ea54, record_slot: 1, rule_set: 5, scale: 5,
    nation: Some(KEY_NATION_BELGIUM), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 4, cup_rule: 5, continental_rule: 1 };

/// Builds the Belgian league ruling body (`ruling_body_class_construct` with `BELGIUM_LEAGUE_RULING_BODY`). Returns this.
/// Original: `RbBelgiumLeague::RbBelgiumLeague` (0x7dbca0, rb_belgium_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_belgium_league_construct(this: usize, file: usize) -> usize {
    covered!("sx18_12 belgian league body built");
    ruling_body_class_construct(this, file, &BELGIUM_LEAGUE_RULING_BODY)
}

/// The Belgian league card rules' verdict. A red card (exactly one) queues a points ban (`pending_points_ban`) from
/// the 15th of this month (of the next when today is past it); only a red after a booking in the match bans on the
/// spot, for one match (reason 2, amount 1). A booking adds one to the entry's count and every third one bans for one
/// more match (reason 1, amount 3). The ban starts today, under the league rule set alone.
pub(crate) unsafe fn belgium_league_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize) -> CardVerdict {
    let rules = [BELGIUM_LEAGUE_RULING_BODY.rule_set, 0xff, 0xff, 0xff];
    let (mut matches, mut reason, mut amount, mut pending) = (0u8, 0xffu8, 0i16, Vec::new());
    if row.red_card() == 1 {
        covered!("sx18_12 belgian league red card");
        pending.extend(pending_points_ban(this, row, fixture, rules, || day_of_this_or_next_month(0xf)));
        if row.yellow_cards() != 0 { // sx18_12: only the second booking's red bans on the spot
            matches = 1;
            reason = 2;
            amount = 1;
        }
    }
    if row.yellow_cards() != 0 {
        let count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(1);
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
        if count % 3 == 0 {
            covered!("sx18_12 belgian league booking ban");
            reason = 1;
            matches += 1;
            amount = 3;
        }
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules, start: ban_start_today() });
    CardVerdict { pending, ban }
}

/// Applies a player's cards of one match under the Belgian league rules (`belgium_league_card_verdict`).
/// Original: `RbBelgiumLeague::process_match_cards` (0x7dbcf0, rb_belgium_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_belgium_league_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| belgium_league_card_verdict(this, MatchRecordRow(row), fixture, e));
}

/// Builds the Belgian cup ruling body (`ruling_body_class_construct` with `BELGIUM_CUP_RULING_BODY`). Returns this.
/// Original: `RbBelgiumCup::RbBelgiumCup` (0x7db8f0, rb_belgium_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_belgium_cup_construct(this: usize, file: usize) -> usize {
    covered!("sx18_12 belgian cup body built");
    ruling_body_class_construct(this, file, &BELGIUM_CUP_RULING_BODY)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_belgium_league_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_ruling_body_class_built(r, this, &BELGIUM_LEAGUE_RULING_BODY, "rb_belgium_league_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_belgium_league_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| belgium_league_card_verdict(this, MatchRecordRow(row), fixture, e), "rb_belgium_league_process_match_cards");
}

pub(crate) unsafe extern "thiscall" fn compare_rb_belgium_cup_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%)(this, file);
    compare_ruling_body_class_built(r, this, &BELGIUM_CUP_RULING_BODY, "rb_belgium_cup_construct")
}
