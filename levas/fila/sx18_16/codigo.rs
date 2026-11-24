// ---------- sx18_16: the Irish ruling body (constructor and card rules) and the Northern Irish body's constructor (ruling_body/rb_ireland.cpp, rb_northern_ireland.cpp) ----------

/// The Irish ruling body: vtable 0x96ec54, record slot 0, rule set 0x10, ban scale 10, the Republic of Ireland in
/// Europe, league rule 0x10, no cup rule, the European rule 1.
pub(crate) const IRELAND_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ec54, record_slot: 0, rule_set: 0x10, scale: 0xa,
    nation: Some(KEY_NATION_REPUBLIC_OF_IRELAND), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0x10, cup_rule: 0xff, continental_rule: 1 };

/// The Northern Irish ruling body: vtable 0x96ed14, record slot 0, rule set 0x24, ban scale 5, Northern Ireland in
/// Europe, league rule 0x24, no cup rule, the European rule 1.
pub(crate) const NORTHERN_IRELAND_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ed14, record_slot: 0, rule_set: 0x24, scale: 5,
    nation: Some(KEY_NATION_NORTHERN_IRELAND), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0x24, cup_rule: 0xff, continental_rule: 1 };

/// Builds the Irish ruling body (`ruling_body_class_construct` with `IRELAND_RULING_BODY`). Returns this.
/// Original: `RbIreland::RbIreland` (0x7df480, rb_ireland.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_ireland_construct(this: usize, file: usize) -> usize {
    covered!("sx18_16 irish body built");
    ruling_body_class_construct(this, file, &IRELAND_RULING_BODY)
}

/// The Irish card rules' verdict, on every player of a match (the entry is found first, cards or not).
///
/// The entry's byte +0xa keeps a five-match card history in its bits 0..4: each match moves bits 1..4 to 0..3 and sets
/// bit 4 when the player had a booking or a red card (bits 5..7 stay). A match without cards after five clean ones takes
/// a booking off the count. Each of the match's bookings adds one to the count: the 5th and the 8th ban for one more
/// match, the 11th for two and every third from the 14th (14, 17, ...) for three (reason 1, the count as amount). A
/// red card (exactly one) queues a points ban (`pending_points_ban`) from the first weekday 3 after today and bans for
/// one more match (reason 2, amount 1). The ban starts in 14 days, under the Irish rule set alone.
pub(crate) unsafe fn ireland_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize) -> CardVerdict {
    let cards = row_has_cards(row);
    let history = read_u8(entry + 0xa); // the five-match card history
    let history = (history & 0xe0) | ((history >> 1) & 0xf) | ((cards as u8) << 4);
    write_u8(entry + 0xa, history);
    if !cards && history & 0x1f == 0 && read_u8(entry + ruling_body_entry::YELLOW_CARDS) != 0 {
        covered!("sx18_16 irish clean record");
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, read_u8(entry + ruling_body_entry::YELLOW_CARDS) - 1);
    }
    let rules = [IRELAND_RULING_BODY.rule_set, 0xff, 0xff, 0xff];
    let (mut matches, mut reason, mut amount, mut pending) = (0u8, 0xffu8, 0i16, Vec::new());
    for _ in 0..row.yellow_cards() {
        let count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(1);
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
        let more = match count {
            5 | 8 => 1, // sx18_16: the 5th and 8th Irish bookings
            11 => 2,
            c if c >= 0xe && (c - 0xe) % 3 == 0 => 3,
            _ => 0,
        };
        if more != 0 {
            covered!("sx18_16 irish booking ban");
            matches += more;
            reason = 1;
            amount = count as i8 as i16;
        }
    }
    if row.red_card() == 1 {
        covered!("sx18_16 irish red card");
        pending.extend(pending_points_ban(this, row, fixture, rules, || first_weekday_after_today(3)));
        matches += 1;
        reason = 2;
        amount = 1;
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules, start: ban_start_in(14) });
    CardVerdict { pending, ban }
}

/// Applies a player's cards of one match under the Irish rules (`ireland_card_verdict`), for every player of the match.
/// Original: `RbIreland::process_match_cards` (0x7df4d0, rb_ireland.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_ireland_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_entry_card_rules(this, row, fixture, |e| ireland_card_verdict(this, MatchRecordRow(row), fixture, e));
}

/// Builds the Northern Irish ruling body (`ruling_body_class_construct` with `NORTHERN_IRELAND_RULING_BODY`). Returns
/// this.
/// Original: `RbNorthernIreland::RbNorthernIreland` (0x7e0af0, rb_northern_ireland.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_northern_ireland_construct(this: usize, file: usize) -> usize {
    covered!("sx18_16 northern irish body built");
    ruling_body_class_construct(this, file, &NORTHERN_IRELAND_RULING_BODY)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_ireland_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_ruling_body_class_built(r, this, &IRELAND_RULING_BODY, "rb_ireland_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_ireland_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_entry_card_rules(this, row, || o(this, row, fixture), |e| ireland_card_verdict(this, MatchRecordRow(row), fixture, e), "rb_ireland_process_match_cards");
}

pub(crate) unsafe extern "thiscall" fn compare_rb_northern_ireland_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%)(this, file);
    compare_ruling_body_class_built(r, this, &NORTHERN_IRELAND_RULING_BODY, "rb_northern_ireland_construct")
}
