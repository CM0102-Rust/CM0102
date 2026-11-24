// ---------- sx18_15: the German cup card rules and the Danish ruling body (constructor and points rules) (ruling_body/rb_germany_cup.cpp, rb_denmark.cpp) ----------

/// The Danish ruling body: vtable 0x96eaf4, record slot 0, rule set 8, ban scale 1, Denmark in Europe, league rule 8,
/// no cup rule, the European rule 1.
pub(crate) const DENMARK_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96eaf4, record_slot: 0, rule_set: 8, scale: 1,
    nation: Some(KEY_NATION_DENMARK), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 8, cup_rule: 0xff, continental_rule: 1 };

/// The German cup rules: the pending ban under the cup rule set; the ban under the cup rule set, and the league's too
/// for a straight red card.
unsafe fn germany_cup_card_verdict(this: usize, row: usize, fixture: usize, e: usize) -> CardVerdict {
    let (league, cup) = (GERMANY_LEAGUE_RULING_BODY.rule_set, GERMANY_CUP_RULING_BODY.rule_set);
    germany_card_verdict(this, MatchRecordRow(row), fixture, e, [0xff, cup, 0xff, 0xff],
        |straight| [if straight { league } else { 0xff }, cup, 0xff, 0xff]) // sx18_15: a straight red card counts in the league too
}

/// Applies a player's cards of one match under the German cup rules (`germany_card_verdict`).
/// Original: `RbGermanyCup::process_match_cards` (0x7de330, rb_germany_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_germany_cup_process_match_cards(this: usize, row: usize, fixture: usize) {
    covered!("sx18_15 german cup cards");
    run_card_rules(this, row, fixture, |e| germany_cup_card_verdict(this, row, fixture, e));
}

/// Builds the Danish ruling body (`ruling_body_class_construct` with `DENMARK_RULING_BODY`). Returns this.
/// Original: `RbDenmark::RbDenmark` (0x7dcae0, rb_denmark.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_denmark_construct(this: usize, file: usize) -> usize {
    covered!("sx18_15 danish body built");
    ruling_body_class_construct(this, file, &DENMARK_RULING_BODY)
}

/// The Danish points rules' verdict. A red card (exactly one) queues a points ban (`pending_points_ban`) from the 24th
/// of this month (of the next when today is past it) and adds 9 points for a second booking's red (two bookings in the
/// match), 24 otherwise. Each of the match's bookings adds 3 points for an offence code up to 3, 4 up to 6, 6 above
/// (the first booking's offence, then the second's). With 15 points or more, each threshold (15, 30, then every 14:
/// 44, 58, ...) up to the new total that the points before the match were under bans for one match (reason 0, the new
/// total as amount). The ban starts today, under the Danish rule set alone.
pub(crate) unsafe fn denmark_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize) -> CardVerdict {
    let rules = [DENMARK_RULING_BODY.rule_set, 0xff, 0xff, 0xff];
    let mut pending = Vec::new();
    let points = |e: usize| read_u16(e + ruling_body_entry::DISCIPLINE_POINTS);
    let add = |n: u16| write_u16(entry + ruling_body_entry::DISCIPLINE_POINTS, points(entry).wrapping_add(n));
    let before = points(entry);
    if row.red_card() == 1 {
        covered!("sx18_15 danish red card");
        pending.extend(pending_points_ban(this, row, fixture, rules, || day_of_this_or_next_month(0x18)));
        add(if row.yellow_cards() == 2 { 9 } else { 0x18 });
    }
    for k in 0..row.yellow_cards() {
        let offence = if k == 0 { row.first_card_offence() } else { row.second_card_offence() };
        add(match offence {
            i8::MIN..=3 => 3,
            4..=6 => 4,
            _ => 6,
        });
    }
    let total = points(entry);
    let (mut matches, mut amount) = (0u8, before as i16);
    if total >= 0xf {
        let mut threshold = 0xfi32;
        loop {
            if (before as i32) < threshold {
                matches += 1;
                amount = total as i16;
            }
            threshold = if threshold == 0xf { 0x1e } else { threshold + 0xe }; // sx18_15: the Danish points thresholds
            if threshold > total as i32 {
                break;
            }
        }
    }
    if matches != 0 {
        covered!("sx18_15 danish points ban");
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason: 0, amount, rules, start: ban_start_today() });
    CardVerdict { pending, ban }
}

/// Applies a player's cards of one match under the Danish points rules (`denmark_card_verdict`).
/// Original: `RbDenmark::process_match_cards` (0x7dcb30, rb_denmark.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_denmark_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| denmark_card_verdict(this, MatchRecordRow(row), fixture, e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_germany_cup_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| germany_cup_card_verdict(this, row, fixture, e), "rb_germany_cup_process_match_cards");
}

pub(crate) unsafe extern "thiscall" fn compare_rb_denmark_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(this, file);
    compare_ruling_body_class_built(r, this, &DENMARK_RULING_BODY, "rb_denmark_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_denmark_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| denmark_card_verdict(this, MatchRecordRow(row), fixture, e), "rb_denmark_process_match_cards");
}
