use super::*;

/// Builds the South American ruling body (`ruling_body_class_construct` with `SOUTH_AMERICA_RULING_BODY`). Returns this.
/// Original: `RbSouthAmerica::RbSouthAmerica` (0x7e29e0, rb_south_america.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_south_america_construct(this: usize, file: usize) -> usize {
    covered!("sx19_1 south american body built");
    ruling_body_class_construct(this, file, &SOUTH_AMERICA_RULING_BODY)
}

/// Applies a player's cards of one match under the South American rules (`south_america_card_verdict`): nothing
/// without a booking or a red card; the ban, when there is one, starts today and goes to the discipline with its news.
/// Original: `RbSouthAmerica::process_match_cards` (0x7e2a30, rb_south_america.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_south_america_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| south_america_card_verdict(MatchRecordRow(row), e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_south_america_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_RB_SOUTH_AMERICA_CONSTRUCT)(this, file);
    compare_ruling_body_class_built(r, this, &SOUTH_AMERICA_RULING_BODY, "rb_south_america_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_south_america_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_RB_SOUTH_AMERICA_PROCESS_MATCH_CARDS);
    compare_card_rules(this, row, || o(this, row, fixture), |e| south_america_card_verdict(MatchRecordRow(row), e), "rb_south_america_process_match_cards");
}

/// South America's card rules (`plain_card_verdict`): one match for a red card and one for every second booking
/// (counted one per match, amount 2), from today, under the South American rule set in the confederation slot (rules
/// ff ff 02 ff).
pub(crate) unsafe fn south_america_card_verdict(row: MatchRecordRow, entry: usize) -> CardVerdict {
    covered!("sx19_1 south american cards");
    plain_card_verdict(row, entry, false, 2, own_rules(&SOUTH_AMERICA_RULING_BODY)) // sx19_1: every second South American booking
}

/// The South American ruling body: vtable 0x96ee34, record slot 2, rule set 2, ban scale 10, no nation, South America,
/// no league nor cup rule, itself as the continental rule.
pub(crate) const SOUTH_AMERICA_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ee34, record_slot: 2, rule_set: 2, scale: 0xa,
    nation: None, continent: Some(KEY_CONTINENT_SOUTH_AMERICA), league_rule: 0xff, cup_rule: 0xff, continental_rule: 2 };
