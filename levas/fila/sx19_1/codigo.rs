// ---------- sx19_1: the South American ruling body (its constructor and its card rules) and the stepped card rules several countries share (ruling_body/rb_south_america.cpp) ----------

/// The South American ruling body: vtable 0x96ee34, record slot 2, rule set 2, ban scale 10, no nation, South America,
/// no league nor cup rule, itself as the continental rule.
pub(crate) const SOUTH_AMERICA_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ee34, record_slot: 2, rule_set: 2, scale: 0xa,
    nation: None, continent: Some(KEY_CONTINENT_SOUTH_AMERICA), league_rule: 0xff, cup_rule: 0xff, continental_rule: 2 };

/// Builds the South American ruling body (`ruling_body_class_construct` with `SOUTH_AMERICA_RULING_BODY`). Returns this.
/// Original: `RbSouthAmerica::RbSouthAmerica` (0x7e29e0, rb_south_america.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_south_america_construct(this: usize, file: usize) -> usize {
    covered!("sx19_1 south american body built");
    ruling_body_class_construct(this, file, &SOUTH_AMERICA_RULING_BODY)
}

/// South America's card rules (`plain_card_verdict`): one match for a red card and one for every second booking
/// (counted one per match, amount 2), from today, under the South American rule set in the confederation slot (rules
/// ff ff 02 ff).
pub(crate) unsafe fn south_america_card_verdict(row: MatchRecordRow, entry: usize) -> CardVerdict {
    covered!("sx19_1 south american cards");
    plain_card_verdict(row, entry, false, 2, own_rules(&SOUTH_AMERICA_RULING_BODY)) // sx19_1: every second South American booking
}

/// Applies a player's cards of one match under the South American rules (`south_america_card_verdict`): nothing
/// without a booking or a red card; the ban, when there is one, starts today and goes to the discipline with its news.
/// Original: `RbSouthAmerica::process_match_cards` (0x7e2a30, rb_south_america.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_south_america_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| south_america_card_verdict(MatchRecordRow(row), e));
}

/// The rules (league, cup, continental, nation slots) of a country's ban under its own rule set: the rule set in the
/// body's record slot (0 league, 1 cup), 0xff in the others.
pub(crate) fn own_rules(class: &RulingBodyClass) -> [u8; 4] {
    let mut r = [0xff; 4];
    r[class.record_slot as usize] = class.rule_set;
    r
}

/// The stepped card rules several countries share (Portugal, Spain, Turkey, Wales). Exactly one red card queues a
/// points ban (`pending_points_ban`, from tomorrow) and bans for one match (reason 2, amount 1). A booking adds one to
/// the entry's count (one per match), and every `every`-th booking bans for one more match (reason 1, amount `every`).
/// The ban starts today; the pending and the immediate ban apply under the body's own rule set (`own_rules`).
pub(crate) unsafe fn stepped_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize, class: &RulingBodyClass, every: u8) -> CardVerdict {
    let rules = own_rules(class);
    let (mut matches, mut reason, mut amount, mut pending) = (0u8, 0xffu8, 0i16, Vec::new());
    if row.red_card() == 1 {
        covered!("sx19_1 stepped red card");
        pending.extend(pending_points_ban(this, row, fixture, rules, || ban_start_in(1))); // sx19_1: the points ban from tomorrow
        matches = 1;
        reason = 2;
        amount = 1;
    }
    if row.yellow_cards() != 0 {
        let count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(1);
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
        if count % every == 0 {
            covered!("sx19_1 stepped booking ban");
            matches += 1;
            reason = 1;
            amount = every as i16;
        }
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules, start: ban_start_today() });
    CardVerdict { pending, ban }
}

pub(crate) unsafe extern "thiscall" fn compare_rb_south_america_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_ruling_body_class_built(r, this, &SOUTH_AMERICA_RULING_BODY, "rb_south_america_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_south_america_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| south_america_card_verdict(MatchRecordRow(row), e), "rb_south_america_process_match_cards");
}
