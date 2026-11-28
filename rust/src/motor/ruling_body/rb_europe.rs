use super::*;

/// Applies a player's cards of one match under the European rules (`europe_card_verdict`): nothing without a booking
/// or a red card; the ban, when there is one, starts today and goes to the discipline with its news.
/// Original: `RbEurope::process_match_cards` (0x7dd450, rb_europe.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_europe_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| europe_card_verdict(MatchRecordRow(row), e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_europe_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_RB_EUROPE_PROCESS_MATCH_CARDS);
    compare_card_rules(this, row, || o(this, row, fixture), |e| europe_card_verdict(MatchRecordRow(row), e), "rb_europe_process_match_cards");
}

/// Builds the European ruling body (`ruling_body_class_construct` with `EUROPE_RULING_BODY`). Returns this.
/// Original: `RbEurope::RbEurope` (0x7dd400, rb_europe.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_europe_construct(this: usize, file: usize) -> usize {
    covered!("sx18_2 european ruling body built");
    ruling_body_class_construct(this, file, &EUROPE_RULING_BODY)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_europe_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_RB_EUROPE_CONSTRUCT)(this, file);
    compare_ruling_body_class_built(r, this, &EUROPE_RULING_BODY, "rb_europe_construct")
}

/// Europe's card rules (`plain_card_verdict`): one match for a red card and one for every third booking (counted one
/// per match), under the European rule set in the confederation slot (rules ff ff 01 ff).
pub(crate) unsafe fn europe_card_verdict(row: MatchRecordRow, entry: usize) -> CardVerdict {
    covered!("sx18_1 european cards");
    plain_card_verdict(row, entry, false, 3, [0xff, 0xff, 1, 0xff]) // sx18_1: every third European booking
}

/// The plain card rules several bodies share (Argentina, Asia, Europe, International): exactly one red card bans for one
/// match (reason 2, amount 1); a booking adds to the entry's count (all the match's bookings with `add_all`, otherwise
/// one) and every `every`-th booking bans for one more match (reason 1, amount `every`). The ban starts today, under
/// `rules`.
pub(crate) unsafe fn plain_card_verdict(row: MatchRecordRow, entry: usize, add_all: bool, every: u8, rules: [u8; 4]) -> CardVerdict {
    let (mut matches, mut reason, mut amount) = (0u8, 0xffu8, 0i16);
    if row.red_card() == 1 {
        matches = 1;
        reason = 2;
        amount = 1;
    }
    let bookings = row.yellow_cards();
    if bookings != 0 {
        let count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(if add_all { bookings } else { 1 });
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
        if count % every == 0 {
            matches += 1;
            reason = 1;
            amount = every as i16;
        }
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules, start: ban_start_today() });
    CardVerdict { pending: Vec::new(), ban }
}

/// The European ruling body: vtable 0x96eb34, record slot 2 (the confederations'), rule set 1, ban scale 10, no
/// nation, Europe, no league or cup rule, itself as the continental rule.
pub(crate) const EUROPE_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96eb34, record_slot: 2, rule_set: 1, scale: 0xa,
    nation: None, continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0xff, cup_rule: 0xff, continental_rule: 1 };
