// ---------- sx18_6: the Italian league ruling body (constructor and card rules) and the Italian cup body's constructor (ruling_body/rb_italy_league.cpp, rb_italy_cup.cpp) ----------

/// The Italian league ruling body: vtable 0x96ec94, record slot 0, rule set 0x11, ban scale 8, Italy in Europe, league
/// rule 0x11, cup rule 0x12, the European rule 1.
pub(crate) const ITALY_LEAGUE_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ec94, record_slot: 0, rule_set: 0x11, scale: 8,
    nation: Some(KEY_NATION_ITALY), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0x11, cup_rule: 0x12, continental_rule: 1 };

/// The Italian cup ruling body: vtable 0x96ec74, record slot 1, rule set 0x12, ban scale 8, Italy in Europe, league
/// rule 0x11, cup rule 0x12, the European rule 1.
pub(crate) const ITALY_CUP_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ec74, record_slot: 1, rule_set: 0x12, scale: 8,
    nation: Some(KEY_NATION_ITALY), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0x11, cup_rule: 0x12, continental_rule: 1 };

/// Builds the Italian league ruling body (`ruling_body_class_construct` with `ITALY_LEAGUE_RULING_BODY`). Returns this.
/// Original: `RbItalyLeague::RbItalyLeague` (0x7dfd70, rb_italy_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_italy_league_construct(this: usize, file: usize) -> usize {
    covered!("sx18_6 italian league body built");
    ruling_body_class_construct(this, file, &ITALY_LEAGUE_RULING_BODY)
}

/// Builds the Italian cup ruling body (`ruling_body_class_construct` with `ITALY_CUP_RULING_BODY`). Returns this.
/// Original: `RbItalyCup::RbItalyCup` (0x7df960, rb_italy_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_italy_cup_construct(this: usize, file: usize) -> usize {
    covered!("sx18_6 italian cup body built");
    ruling_body_class_construct(this, file, &ITALY_CUP_RULING_BODY)
}

/// The Italian league card rules' verdict. A red card (exactly one) bans for one match (reason 2, amount 1) and, for an
/// offence code above 9 (`red_card_offence`), queues a points ban (`pending_points_ban`) from the first
/// weekday 3 after today. A booking adds one to the entry's count, which bans for one more match at 4 (amount 4), 7
/// (amount 3) and each odd count from 9 (amount 2), reason 1. The ban starts today, under the league rule set alone.
pub(crate) unsafe fn italy_league_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize) -> CardVerdict {
    let rules = [ITALY_LEAGUE_RULING_BODY.rule_set, 0xff, 0xff, 0xff];
    let (mut matches, mut reason, mut amount, mut pending) = (0u8, 0xffu8, 0i16, Vec::new());
    if row.red_card() == 1 {
        covered!("sx18_6 italian league red card");
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
        if count == 4 || count == 7 || (count > 8 && count % 2 == 1) { // sx18_6: the Italian league booking bans
            covered!("sx18_6 italian league booking ban");
            reason = 1;
            matches += 1;
            amount = match count {
                4 => 4,
                7 => 3,
                _ => 2,
            };
        }
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules, start: ban_start_today() });
    CardVerdict { pending, ban }
}

/// Applies a player's cards of one match under the Italian league rules (`italy_league_card_verdict`).
/// Original: `RbItalyLeague::process_match_cards` (0x7dfdc0, rb_italy_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_italy_league_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| italy_league_card_verdict(this, MatchRecordRow(row), fixture, e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_italy_league_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_ruling_body_class_built(r, this, &ITALY_LEAGUE_RULING_BODY, "rb_italy_league_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_italy_league_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| italy_league_card_verdict(this, MatchRecordRow(row), fixture, e), "rb_italy_league_process_match_cards");
}

pub(crate) unsafe extern "thiscall" fn compare_rb_italy_cup_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%)(this, file);
    compare_ruling_body_class_built(r, this, &ITALY_CUP_RULING_BODY, "rb_italy_cup_construct")
}
