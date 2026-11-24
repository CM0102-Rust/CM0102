// ---------- sx18_14: the German league ruling body (constructor and card rules) and the German cup body's constructor (ruling_body/rb_germany_league.cpp, rb_germany_cup.cpp) ----------

/// The German league ruling body: vtable 0x96ebd4, record slot 0, rule set 0xd, ban scale 7, Germany in Europe, league
/// rule 0xd, cup rule 0xe, the European rule 1.
pub(crate) const GERMANY_LEAGUE_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ebd4, record_slot: 0, rule_set: 0xd, scale: 7,
    nation: Some(KEY_NATION_GERMANY), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0xd, cup_rule: 0xe, continental_rule: 1 };

/// The German cup ruling body: vtable 0x96ebb4, record slot 1, rule set 0xe, ban scale 10, Germany in Europe, league
/// rule 0xd, cup rule 0xe, the European rule 1.
pub(crate) const GERMANY_CUP_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ebb4, record_slot: 1, rule_set: 0xe, scale: 0xa,
    nation: Some(KEY_NATION_GERMANY), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0xd, cup_rule: 0xe, continental_rule: 1 };

/// The German card rules' verdict (league and cup). A booking adds one to the entry's count and every fifth bans for
/// one match (reason 1, amount 5). A red card (exactly one) queues a points ban (`pending_points_ban`, under
/// `pending_rules`) from a week after today and bans for one more match (reason 2, amount 1). The ban starts today,
/// under `rules(straight)`: whether the red card came with no booking (a straight red card also counts in the other
/// competition).
pub(crate) unsafe fn germany_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize, pending_rules: [u8; 4], rules: impl FnOnce(bool) -> [u8; 4]) -> CardVerdict {
    let (mut matches, mut reason, mut amount, mut pending) = (0u8, 0xffu8, 0i16, Vec::new());
    if row.yellow_cards() != 0 {
        let count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(1);
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
        if count % 5 == 0 { // sx18_14: every fifth German booking
            covered!("sx18_14 german booking ban");
            matches = 1;
            reason = 1;
            amount = 5;
        }
    }
    if row.red_card() == 1 {
        covered!("sx18_14 german red card");
        reason = 2;
        pending.extend(pending_points_ban(this, row, fixture, pending_rules, || ban_start_in_weeks(1)));
        matches += 1;
        amount = 1;
    }
    let straight = row.red_card() != 0 && row.yellow_cards() == 0;
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules: rules(straight), start: ban_start_today() });
    CardVerdict { pending, ban }
}

/// The German league rules: the pending ban under the league rule set; the ban under the league rule set, and the cup's
/// too for a straight red card.
unsafe fn germany_league_card_verdict(this: usize, row: usize, fixture: usize, e: usize) -> CardVerdict {
    let (league, cup) = (GERMANY_LEAGUE_RULING_BODY.rule_set, GERMANY_CUP_RULING_BODY.rule_set);
    germany_card_verdict(this, MatchRecordRow(row), fixture, e, [league, 0xff, 0xff, 0xff],
        |straight| [league, if straight { cup } else { 0xff }, 0xff, 0xff])
}

/// Builds the German league ruling body (`ruling_body_class_construct` with `GERMANY_LEAGUE_RULING_BODY`). Returns this.
/// Original: `RbGermanyLeague::RbGermanyLeague` (0x7de620, rb_germany_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_germany_league_construct(this: usize, file: usize) -> usize {
    covered!("sx18_14 german league body built");
    ruling_body_class_construct(this, file, &GERMANY_LEAGUE_RULING_BODY)
}

/// Applies a player's cards of one match under the German league rules (`germany_card_verdict`).
/// Original: `RbGermanyLeague::process_match_cards` (0x7de670, rb_germany_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_germany_league_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| germany_league_card_verdict(this, row, fixture, e));
}

/// Builds the German cup ruling body (`ruling_body_class_construct` with `GERMANY_CUP_RULING_BODY`). Returns this.
/// Original: `RbGermanyCup::RbGermanyCup` (0x7de2e0, rb_germany_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_germany_cup_construct(this: usize, file: usize) -> usize {
    covered!("sx18_14 german cup body built");
    ruling_body_class_construct(this, file, &GERMANY_CUP_RULING_BODY)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_germany_league_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_ruling_body_class_built(r, this, &GERMANY_LEAGUE_RULING_BODY, "rb_germany_league_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_germany_league_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| germany_league_card_verdict(this, row, fixture, e), "rb_germany_league_process_match_cards");
}

pub(crate) unsafe extern "thiscall" fn compare_rb_germany_cup_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%)(this, file);
    compare_ruling_body_class_built(r, this, &GERMANY_CUP_RULING_BODY, "rb_germany_cup_construct")
}
