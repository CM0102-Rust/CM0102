use super::*;

/// Builds the English ruling body: the RulingBody base with the file, an empty date at +0x23 (`Date::Date`; the English
/// object is the only one allocated with 0x2b bytes) and the class fields of `ENGLAND_RULING_BODY`. Returns this.
/// Original: `RbEngland::RbEngland` (0x7dcf90, rb_england.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_england_construct(this: usize, file: usize) -> usize {
    construct_ruling_body(this, file);
    exe::date_date_54a110(this + 0x23); // the English body's own date
    covered!("sx18_1 english ruling body built");
    ruling_body_class_fields(this, &ENGLAND_RULING_BODY)
}

/// Applies a player's cards of one match under the English rules (`england_card_verdict`): nothing without a booking or
/// a red card; the ban, when there is one, goes to the discipline with its news.
/// Original: `RbEngland::process_match_cards` (0x7dd010, rb_england.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_england_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| england_card_verdict(MatchRecordRow(row), Fixture(fixture), e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_england_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_RB_ENGLAND_CONSTRUCT)(this, file);
    compare_ruling_body_class_built(r, this, &ENGLAND_RULING_BODY, "rb_england_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_england_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_RB_ENGLAND_PROCESS_MATCH_CARDS);
    compare_card_rules(this, row, || o(this, row, fixture), |e| england_card_verdict(MatchRecordRow(row), Fixture(fixture), e), "rb_england_process_match_cards");
}

/// Sets the class fields on a built RulingBody base: the vtable, the record slot, the rule set, the ban scale, the
/// nation and continent ids, the area's league, cup and continental rule sets and the byte +0x22 cleared. Returns this.
pub(crate) unsafe fn ruling_body_class_fields(this: usize, class: &RulingBodyClass) -> usize {
    write_u32(this, class.vtable);
    write_u8(this + ruling_body::RECORD_SLOT, class.record_slot);
    write_u8(this + ruling_body::RULE_SET_INDEX, class.rule_set);
    write_u8(this + ruling_body::BAN_SCALE, class.scale);
    write_u32(this + ruling_body::NATION_ID, ruling_body_key(class.nation));
    write_u32(this + ruling_body::CONTINENT_ID, ruling_body_key(class.continent));
    write_u8(this + 0x1f, class.league_rule); // the area's league ruling body
    write_u8(this + 0x20, class.cup_rule); // the area's cup ruling body
    write_u8(this + 0x21, class.continental_rule); // the area's continental ruling body
    write_u8(this + 0x22, 0); // a flag byte, cleared
    this
}

/// Runs a body's card rules for one player (`run_entry_card_rules`), nothing without a booking or a red card.
pub(crate) unsafe fn run_card_rules(this: usize, row: usize, fixture: usize, verdict: impl FnOnce(usize) -> CardVerdict) {
    if row_has_cards(MatchRecordRow(row)) {
        run_entry_card_rules(this, row, fixture, verdict);
    }
}

/// Comparer of the card rules (`compare_entry_card_rules`); without a booking or a red card only the original runs.
pub(crate) unsafe fn compare_card_rules(this: usize, row: usize, original: impl FnOnce(), verdict: impl FnOnce(usize) -> CardVerdict, name: &str) {
    if row_has_cards(MatchRecordRow(row)) {
        compare_entry_card_rules(this, row, original, verdict, name);
    } else {
        original();
    }
}

/// The English card rules' verdict on one player's match (`entry`: his entry in the body).
///
/// Bookings: the entry's count grows by the match's bookings; the ban threshold is the next of 5, 10 and 15 above the
/// count before the match (from 15 on, (count mod 5 + 1) × 5). Reaching it bans for threshold / 5 matches with reason 1
/// and the new count as amount, except that the 5th booking only counts in a match played before 1 March of the
/// season's second year and the 10th only before the day after the second weekday-6 from 1 April of that year (the
/// match date is the fixture's day and year; the season year is the fixture's `.season_year()`). A red card adds one
/// match, two more for an offence code from 9 and three more for code 12 (the first card's offence with no booking,
/// the second's with one booking, none otherwise), plus one per red card already in the entry, whose count then grows;
/// reason 2, amount 1. The ban starts in 14 days, under the English rule set alone.
pub(crate) unsafe fn england_card_verdict(row: MatchRecordRow, fixture: Fixture, entry: usize) -> CardVerdict {
    let mut played = [0u8; 8];
    exe::new_date(played.as_mut_ptr() as usize, fixture.day() as u32, fixture.year() as u32);
    let next_year = fixture.season_year().wrapping_add(1) as u32;
    let mut april = [0u16; 4];
    exe::date_date_549f70(&mut april, 1, 3, next_year, u32::MAX);
    let mut weekday_6 = 0;
    loop {
        if exe::date_get_weekday(april.as_ptr() as usize) as u16 == 6 {
            weekday_6 += 1;
        }
        let mut t = [0u8; 8];
        let p = exe::date_add_days(april.as_ptr() as usize, t.as_mut_ptr() as usize, 1);
        copy_record_date(april.as_mut_ptr() as usize, p);
        if weekday_6 >= 2 {
            break;
        }
    }
    let mut march = [0u16; 4];
    exe::date_date_549f70(&mut march, 1, 2, next_year, u32::MAX);
    let (mut matches, mut reason, mut amount) = (0u8, 0xffu8, 0i16);
    let bookings = row.yellow_cards();
    if bookings != 0 {
        let before = read_u8(entry + ruling_body_entry::YELLOW_CARDS);
        let threshold: u8 = match before {
            0..=4 => 5, // sx18_1: the first booking ban
            5..=9 => 10,
            10..=14 => 15,
            _ => (before % 5 + 1).wrapping_mul(5),
        };
        let total = before.wrapping_add(bookings);
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, total);
        if total as i32 >= threshold as i8 as i32 {
            let due = match threshold {
                5 => cm_date_after(march.as_ptr() as usize, played.as_ptr() as usize),
                10 => cm_date_after(april.as_ptr() as usize, played.as_ptr() as usize),
                t => t as i8 > 10,
            };
            if due {
                covered!("sx18_1 english booking ban");
                reason = 1;
                matches = (threshold as i8 / 5) as u8;
                amount = total as i8 as i16;
            }
        }
    }
    if row.red_card() != 0 {
        covered!("sx18_1 english red card");
        let offence = red_card_offence(row);
        matches = matches.wrapping_add(1);
        if offence == 0xc {
            matches = matches.wrapping_add(3);
        } else if offence != -1 && offence >= 9 {
            matches = matches.wrapping_add(2);
        }
        let reds = read_u8(entry + ruling_body_entry::RED_CARDS);
        write_u8(entry + ruling_body_entry::RED_CARDS, reds.wrapping_add(1));
        reason = 2;
        amount = 1;
        matches = matches.wrapping_add(reds); // one more match per earlier red card
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules: [ENGLAND_RULING_BODY.rule_set, 0xff, 0xff, 0xff],
        start: ban_start_in(14) }); // sx18_1: the English ban starts in two weeks
    CardVerdict { pending: Vec::new(), ban }
}

/// Whether date `a` comes after date `b` (the year word unsigned, then the day word signed), as the card rules
/// compare their cut-off dates.
pub(crate) unsafe fn cm_date_after(a: usize, b: usize) -> bool {
    let (ya, yb) = (read_u16(a + 2), read_u16(b + 2));
    ya > yb || (ya == yb && read_i16(a) > read_i16(b))
}

/// The offence code of a player's red card: the first card's with no booking, the second card's with one booking, −1
/// otherwise.
pub(crate) unsafe fn red_card_offence(row: MatchRecordRow) -> i8 {
    match row.yellow_cards() {
        0 => row.first_card_offence(),
        1 => row.second_card_offence(),
        _ => -1,
    }
}

/// Whether the row has a booking or a red card (the card rules do nothing otherwise).
pub(crate) unsafe fn row_has_cards(row: MatchRecordRow) -> bool {
    row.yellow_cards() != 0 || row.red_card() != 0
}

/// Runs a body's card rules for one player: the player's entry in the body (`RulingBody::find_or_add_discipline_entry`,
/// created when missing), the verdict and its effects.
pub(crate) unsafe fn run_entry_card_rules(this: usize, row: usize, fixture: usize, verdict: impl FnOnce(usize) -> CardVerdict) {
    let row = MatchRecordRow(row);
    let e = exe::rulingbody_find_or_add_discipline_entry(this, row.staff_id() as u32, 1);
    let v = verdict(e);
    apply_card_verdict(this, row, fixture, &v);
}

/// Comparer of the card rules on the player's entry: the bans cannot be handed out twice, so the copy's verdict is taken
/// first on a copy of the entry (found or created as the original does first), then only the original runs. Checked
/// afterwards: the entry's bookings, red cards, points and history bytes (+4, +5, +8 to +0xd); the verdict's pending
/// bans more on the body's list; and, when the player had no ban in the body before, the new ban the entry
/// points to (matches, days, start, rules, reason, amount and flag bit 2).
pub(crate) unsafe fn compare_entry_card_rules(this: usize, row: usize, original: impl FnOnce(), verdict: impl FnOnce(usize) -> CardVerdict, name: &str) {
    let row = MatchRecordRow(row);
    let id = row.staff_id() as u32;
    let e = exe::rulingbody_find_or_add_discipline_entry(this, id, 1);
    let mut local = [0u8; 0xe];
    core::ptr::copy_nonoverlapping(e as *const u8, local.as_mut_ptr(), 0xe);
    let had_ban = read_i16(e + ruling_body_entry::BAN_INDEX) > -1;
    let list = read_usize(this + RULING_BODY_LIST);
    let queued = read_u16(list + llist::COUNT);
    let v = verdict(local.as_mut_ptr() as usize);
    original();
    let e = ruling_body_entry(this, id);
    let counts = |p: usize| [read_u16(p + ruling_body_entry::YELLOW_CARDS), read_u16(p + 8), read_u16(p + 0xa), read_u16(p + 0xc)];
    let (got, expected) = (if e != 0 { counts(e) } else { [0; 4] }, counts(local.as_ptr() as usize));
    let mut ok = e != 0 && got == expected && read_u16(list + llist::COUNT) == queued.wrapping_add(v.pending.len() as u16);
    if let (Some(b), false, true) = (&v.ban, had_ban, ok) {
        let s = read_i16(e + ruling_body_entry::BAN_INDEX);
        let r = immediate_ban_record(b, row);
        let p = r.as_ptr() as usize;
        let ban = read_usize(read_usize(G_DISCIPLINE) + discipline_fields::BANS).wrapping_add((s as i32 as usize).wrapping_mul(20));
        ok = s > -1 && read_u8(ban) == b.matches && read_u8(ban + ban_record::MATCHES_SERVED) == 0
            && read_i16(ban + ban_record::DAYS) == -1 && read_u32(ban + ban_record::START_DATE) == read_u32(p + ban_record::START_DATE)
            && read_u32(ban + ban_record::RULES) == read_u32(p + ban_record::RULES) && read_u8(ban + ban_record::REASON) == b.reason
            && read_u16(ban + ban_record::REASON_AMOUNT) == read_u16(p + ban_record::REASON_AMOUNT)
            && read_u8(ban + ban_record::FLAGS) & 4 == read_u8(p + ban_record::FLAGS) & 4;
    }
    SCOREBOARD.record(ok, || format!("{}(player {}): entry {:x?} (expected {:x?}), queued {} (before {}), ban expected {:?}",
        name, id, got, expected, read_u16(list + llist::COUNT), queued, v.ban.as_ref().map(|b| (b.matches, b.reason, b.amount))));
}

/// The 20-byte ban record of an immediate ban: matches, none served, no days, the start date, the rules, the reason
/// and its amount, and the flags with bit 2 = (the row's byte +0x48 == 0). The original leaves the other flag bits as
/// stack garbage (bit 0 is then set or cleared by `Discipline::add_ban`); here they are 0.
pub(crate) unsafe fn immediate_ban_record(b: &ImmediateBan, row: MatchRecordRow) -> [u8; 0x14] {
    let mut r = [0u8; 0x14];
    let p = r.as_mut_ptr() as usize;
    write_u8(p + ban_record::MATCHES, b.matches);
    write_u8(p + ban_record::MATCHES_SERVED, 0);
    write_u16(p + ban_record::DAYS, 0xffff);
    copy_record_date(p + ban_record::START_DATE, b.start.as_ptr() as usize);
    for k in 0..4 {
        write_u8(p + ban_record::RULES + k, b.rules[k]);
    }
    write_u8(p + ban_record::REASON, b.reason);
    write_u16(p + ban_record::REASON_AMOUNT, b.amount as u16);
    write_u8(p + ban_record::FLAGS, ((read_u8(row.addr() + 0x48) == 0) as u8) << 2); // row +0x48: a per-player byte of the match
    r
}

/// Carries out a verdict: the pending bans go to the end of the body's list (`LLIST::add_tail`, 0x70 bytes); the
/// immediate ban goes to the discipline (`Discipline::add_ban` with the player's id) and its news is posted
/// (`Discipline::post_ban_news` with the player, the ban and the fixture).
pub(crate) unsafe fn apply_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, v: &CardVerdict) {
    for node in &v.pending {
        exe::llist_add_tail(read_usize(this + RULING_BODY_LIST), node.as_ptr() as usize, 0x70);
    }
    if let Some(b) = &v.ban {
        let mut r = immediate_ban_record(b, row);
        add_ban(read_usize(G_DISCIPLINE), row.staff_id() as u32, r.as_mut_ptr() as usize);
        let post: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_POST_BAN_NEWS);
        post(read_usize(G_DISCIPLINE), staff_at(row.staff_id() as isize).addr(), r.as_mut_ptr() as usize, fixture);
    }
}

/// The English ruling body: vtable 0x96eb14, record slot 0, rule set 9, ban scale 5, England in Europe, league rule 9,
/// no cup rule, the European rule 1.
pub(crate) const ENGLAND_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96eb14, record_slot: 0, rule_set: 9, scale: 5,
    nation: Some(KEY_NATION_ENGLAND), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 9, cup_rule: 0xff, continental_rule: 1 };
