// ---------- sx19_5: the Norwegian league ruling body (constructor and card rules) and the Portuguese body's constructor (ruling_body/rb_norway_league.cpp, rb_portugal.cpp) ----------

/// The Norwegian league ruling body: vtable 0x96ed54, record slot 0, rule set 0x15, ban scale 10, Norway in Europe,
/// league rule 0x15, cup rule 0x16, the European rule 1.
pub(crate) const NORWAY_LEAGUE_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ed54, record_slot: 0, rule_set: 0x15, scale: 0xa,
    nation: Some(KEY_NATION_NORWAY), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0x15, cup_rule: 0x16, continental_rule: 1 };

/// The Portuguese ruling body: vtable 0x96edb4, record slot 0, rule set 0x17, ban scale 8, Portugal in Europe, league
/// rule 0x17, no cup rule, the European rule 1.
pub(crate) const PORTUGAL_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96edb4, record_slot: 0, rule_set: 0x17, scale: 8,
    nation: Some(KEY_NATION_PORTUGAL), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0x17, cup_rule: 0xff, continental_rule: 1 };

/// Builds the Norwegian league ruling body (`ruling_body_class_construct` with `NORWAY_LEAGUE_RULING_BODY`). Returns
/// this.
/// Original: `RbNorwayLeague::RbNorwayLeague` (0x7e1150, rb_norway_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_norway_league_construct(this: usize, file: usize) -> usize {
    covered!("sx19_5 norwegian league body built");
    ruling_body_class_construct(this, file, &NORWAY_LEAGUE_RULING_BODY)
}

/// Builds the Portuguese ruling body (`ruling_body_class_construct` with `PORTUGAL_RULING_BODY`). Returns this.
/// Original: `RbPortugal::RbPortugal` (0x7e1a60, rb_portugal.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_portugal_construct(this: usize, file: usize) -> usize {
    covered!("sx19_5 portuguese body built");
    ruling_body_class_construct(this, file, &PORTUGAL_RULING_BODY)
}

/// The Norwegian league card rules' verdict: always a one-match ban, from today, under the league rule set. Exactly one
/// red card bans (reason 2, amount 1) and the match's bookings are then not counted. Otherwise a booking adds one to
/// the entry's count (one per match): the third bans with amount 3, the fifth with amount 2 and each one after with
/// amount 1 (reason 1); the fourth does not ban.
pub(crate) unsafe fn norway_league_card_verdict(row: MatchRecordRow, entry: usize) -> CardVerdict {
    let none = CardVerdict { pending: Vec::new(), ban: None };
    let (reason, amount) = if row.red_card() == 1 {
        covered!("sx19_5 norwegian red card");
        (2, 1)
    } else {
        if row.yellow_cards() == 0 {
            return none;
        }
        let count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(1);
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
        match count {
            3 => (1, 3), // sx19_5: the third Norwegian booking
            0..=4 => return none,
            5 => (1, 2),
            _ => (1, 1),
        }
    };
    covered!("sx19_5 norwegian ban");
    let ban = ImmediateBan { matches: 1, reason, amount, rules: own_rules(&NORWAY_LEAGUE_RULING_BODY), start: ban_start_today() };
    CardVerdict { pending: Vec::new(), ban: Some(ban) }
}

/// Applies a player's cards of one match under the Norwegian league rules (`norway_league_card_verdict`): nothing
/// without a booking or a red card; the ban, when there is one, goes to the discipline with its news.
/// Original: `RbNorwayLeague::process_match_cards` (0x7e11a0, rb_norway_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_norway_league_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| norway_league_card_verdict(MatchRecordRow(row), e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_norway_league_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_ruling_body_class_built(r, this, &NORWAY_LEAGUE_RULING_BODY, "rb_norway_league_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_norway_league_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| norway_league_card_verdict(MatchRecordRow(row), e), "rb_norway_league_process_match_cards");
}

pub(crate) unsafe extern "thiscall" fn compare_rb_portugal_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%)(this, file);
    compare_ruling_body_class_built(r, this, &PORTUGAL_RULING_BODY, "rb_portugal_construct")
}
