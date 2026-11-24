// ---------- sx18_10: the Japanese league ruling body (constructor and card rules) and the Japanese cup body's constructor (ruling_body/rb_japan_league.cpp, rb_japan_cup.cpp) ----------

/// The Japanese league ruling body: vtable 0x96ecd4, record slot 0, rule set 0x13, ban scale 10, Japan in Asia, league
/// rule 0x13, cup rule 0x14, no continental rule.
pub(crate) const JAPAN_LEAGUE_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ecd4, record_slot: 0, rule_set: 0x13, scale: 0xa,
    nation: Some(KEY_NATION_JAPAN), continent: Some(KEY_CONTINENT_ASIA), league_rule: 0x13, cup_rule: 0x14, continental_rule: 0xff };

/// The Japanese cup ruling body: vtable 0x96ecb4, record slot 1, rule set 0x14, ban scale 10, Japan in Asia, league
/// rule 0x13, cup rule 0x14, no continental rule.
pub(crate) const JAPAN_CUP_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ecb4, record_slot: 1, rule_set: 0x14, scale: 0xa,
    nation: Some(KEY_NATION_JAPAN), continent: Some(KEY_CONTINENT_ASIA), league_rule: 0x13, cup_rule: 0x14, continental_rule: 0xff };

/// The Japanese card rules' verdict (league every third booking, cup every second, with their `rules`). The match's
/// bookings add to the entry's count, and reaching a multiple of `every` bans for one match (reason 1, amount `every`).
/// A red card (exactly one) adds matches by its offence (`red_card_offence`): one for code 9, two for 10, three for
/// 11, four for 12, `no_offence` (the league 1, the cup 0) with no offence code (−1), nothing for another code; reason
/// 2, amount 1. The ban starts today.
pub(crate) unsafe fn japan_card_verdict(row: MatchRecordRow, entry: usize, every: u8, no_offence: u8, rules: [u8; 4]) -> CardVerdict {
    let (mut matches, mut reason, mut amount) = (0u8, 0xffu8, 0i16);
    let bookings = row.yellow_cards();
    if bookings != 0 {
        let count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(bookings);
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
        if count % every == 0 {
            covered!("sx18_10 japanese booking ban");
            matches = 1;
            reason = 1;
            amount = every as i16;
        }
    }
    if row.red_card() == 1 {
        covered!("sx18_10 japanese red card");
        matches += match red_card_offence(row) {
            -1 => no_offence,
            9 => 1,
            10 => 2, // sx18_10: two matches for offence 10
            11 => 3,
            12 => 4,
            _ => 0,
        };
        reason = 2;
        amount = 1;
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules, start: ban_start_today() });
    CardVerdict { pending: Vec::new(), ban }
}

/// Builds the Japanese league ruling body (`ruling_body_class_construct` with `JAPAN_LEAGUE_RULING_BODY`). Returns this.
/// Original: `RbJapanLeague::RbJapanLeague` (0x7e0490, rb_japan_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_japan_league_construct(this: usize, file: usize) -> usize {
    covered!("sx18_10 japanese league body built");
    ruling_body_class_construct(this, file, &JAPAN_LEAGUE_RULING_BODY)
}

/// The Japanese league rules: every third booking, a red card with no offence code counts one match, under the league
/// rule set in the first slot.
unsafe fn japan_league_card_verdict(row: usize, e: usize) -> CardVerdict {
    japan_card_verdict(MatchRecordRow(row), e, 3, 1, [JAPAN_LEAGUE_RULING_BODY.rule_set, 0xff, 0xff, 0xff])
}

/// Applies a player's cards of one match under the Japanese league rules (`japan_card_verdict`, every third booking).
/// Original: `RbJapanLeague::process_match_cards` (0x7e04e0, rb_japan_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_japan_league_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| japan_league_card_verdict(row, e));
}

/// Builds the Japanese cup ruling body (`ruling_body_class_construct` with `JAPAN_CUP_RULING_BODY`). Returns this.
/// Original: `RbJapanCup::RbJapanCup` (0x7e01b0, rb_japan_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_japan_cup_construct(this: usize, file: usize) -> usize {
    covered!("sx18_10 japanese cup body built");
    ruling_body_class_construct(this, file, &JAPAN_CUP_RULING_BODY)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_japan_league_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_ruling_body_class_built(r, this, &JAPAN_LEAGUE_RULING_BODY, "rb_japan_league_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_japan_league_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| japan_league_card_verdict(row, e), "rb_japan_league_process_match_cards");
}

pub(crate) unsafe extern "thiscall" fn compare_rb_japan_cup_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%)(this, file);
    compare_ruling_body_class_built(r, this, &JAPAN_CUP_RULING_BODY, "rb_japan_cup_construct")
}
