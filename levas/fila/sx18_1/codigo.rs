// ---------- sx18_1: the English ruling body (its constructor and its card rules) and the pieces every country's ruling body shares (ruling_body/rb_*.cpp) ----------

/// `Discipline::post_ban_news(player, ban, fixture)` (0x557280, discipline.cpp, thiscall, ret 0xc): the ban news.
const FN_POST_BAN_NEWS: usize = 0x557280;

/// One ruling body class as its constructor sets it on the RulingBody base: the vtable, the slot of the person's
/// discipline record it uses (`ruling_body::RECORD_SLOT`), its index in the discipline table
/// (`ruling_body::RULE_SET_INDEX`), the ban scale (`ruling_body::BAN_SCALE`), the nation and continent it covers (the
/// key, or None for −1) and the indexes of the league, cup and continental ruling bodies of its area (+0x1f, +0x20,
/// +0x21; 0xff none).
pub(crate) struct RulingBodyClass {
    pub(crate) vtable: u32,
    pub(crate) record_slot: u8,
    pub(crate) rule_set: u8,
    pub(crate) scale: u8,
    pub(crate) nation: Option<usize>,
    pub(crate) continent: Option<usize>,
    pub(crate) league_rule: u8,
    pub(crate) cup_rule: u8,
    pub(crate) continental_rule: u8,
}

/// The value of an optional key (−1 without one).
unsafe fn ruling_body_key(key: Option<usize>) -> u32 {
    key.map_or(u32::MAX, |k| read_u32(k))
}

/// Builds a ruling body of the class: the RulingBody base with the file (`construct_ruling_body`), then the class
/// fields (`ruling_body_class_fields`). Returns this.
pub(crate) unsafe fn ruling_body_class_construct(this: usize, file: usize, class: &RulingBodyClass) -> usize {
    construct_ruling_body(this, file);
    ruling_body_class_fields(this, class)
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

/// Comparer of the ruling body constructors: they are built once per game, so only the original runs and the fields
/// it leaves are checked against the class (the copy's constants).
pub(crate) unsafe fn compare_ruling_body_class_built(r: usize, this: usize, class: &RulingBodyClass, name: &str) -> usize {
    let ok = r == this && read_u32(this) == class.vtable
        && read_u8(this + ruling_body::RECORD_SLOT) == class.record_slot
        && read_u8(this + ruling_body::RULE_SET_INDEX) == class.rule_set
        && read_u8(this + ruling_body::BAN_SCALE) == class.scale
        && read_u32(this + ruling_body::NATION_ID) == ruling_body_key(class.nation)
        && read_u32(this + ruling_body::CONTINENT_ID) == ruling_body_key(class.continent)
        && read_u8(this + 0x1f) == class.league_rule && read_u8(this + 0x20) == class.cup_rule
        && read_u8(this + 0x21) == class.continental_rule && read_u8(this + 0x22) == 0;
    SCOREBOARD.record(ok, || format!("{}({:#x}): vtable {:#x} slot {} rule {} scale {} nation {} continent {} rules {:x} {:x} {:x}",
        name, this, read_u32(this), read_u8(this + ruling_body::RECORD_SLOT), read_u8(this + ruling_body::RULE_SET_INDEX),
        read_u8(this + ruling_body::BAN_SCALE), read_i32(this + ruling_body::NATION_ID), read_i32(this + ruling_body::CONTINENT_ID),
        read_u8(this + 0x1f), read_u8(this + 0x20), read_u8(this + 0x21)));
    r
}

/// Whether date `a` comes after date `b` (the year word unsigned, then the day word signed), as the card rules
/// compare their cut-off dates.
pub(crate) unsafe fn cm_date_after(a: usize, b: usize) -> bool {
    let (ya, yb) = (read_u16(a + 2), read_u16(b + 2));
    ya > yb || (ya == yb && read_i16(a) > read_i16(b))
}

/// The ban a ruling body hands out on the spot: the matches, the reason (1 bookings, 2 red card) and its amount, the
/// four rules it applies under and the start date.
pub(crate) struct ImmediateBan {
    pub(crate) matches: u8,
    pub(crate) reason: u8,
    pub(crate) amount: i8,
    pub(crate) rules: [u8; 4],
    pub(crate) start: [u8; 8],
}

/// What a ruling body's card rules decide for one player after a match: the 0x70-byte pending ban queued on the
/// body's list (a points ban that starts on a later date) and the ban handed out on the spot.
pub(crate) struct CardVerdict {
    pub(crate) pending: Option<[u8; 0x70]>,
    pub(crate) ban: Option<ImmediateBan>,
}

/// Today's date, as the start of a ban.
pub(crate) unsafe fn ban_start_today() -> [u8; 8] {
    let mut d = [0u8; 8];
    copy_record_date(d.as_mut_ptr() as usize, G_CURRENT_DATE);
    d
}

/// Today plus `days`, as the start of a ban (`Date::add_days`).
pub(crate) unsafe fn ban_start_in(days: u32) -> [u8; 8] {
    let (mut t, mut d) = ([0u8; 8], [0u8; 8]);
    let p = exe::date_add_days(G_CURRENT_DATE, t.as_mut_ptr() as usize, days);
    copy_record_date(d.as_mut_ptr() as usize, p);
    d
}

/// The 20-byte ban record of an immediate ban: matches, none served, no days, the start date, the rules, the reason
/// and its amount (sign-extended), and the flags with bit 2 = (the row's byte +0x48 == 0). The original leaves the other
/// flag bits as stack garbage (bit 0 is then set or cleared by `Discipline::add_ban`); here they are 0.
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
    write_u16(p + ban_record::REASON_AMOUNT, b.amount as i16 as u16);
    write_u8(p + ban_record::FLAGS, ((read_u8(row.addr() + 0x48) == 0) as u8) << 2); // row +0x48: a per-player byte of the match
    r
}

/// Carries out a verdict: the pending ban goes to the end of the body's list (`LLIST::add_tail`, 0x70 bytes); the
/// immediate ban goes to the discipline (`Discipline::add_ban` with the player's id) and its news is posted
/// (`Discipline::post_ban_news` with the player, the ban and the fixture).
pub(crate) unsafe fn apply_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, v: &CardVerdict) {
    if let Some(node) = &v.pending {
        exe::llist_add_tail(read_usize(this + RULING_BODY_LIST), node.as_ptr() as usize, 0x70);
    }
    if let Some(b) = &v.ban {
        let mut r = immediate_ban_record(b, row);
        add_ban(read_usize(G_DISCIPLINE), row.staff_id() as u32, r.as_mut_ptr() as usize);
        let post: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_POST_BAN_NEWS);
        post(read_usize(G_DISCIPLINE), staff_at(row.staff_id() as isize).addr(), r.as_mut_ptr() as usize, fixture);
    }
}

/// Runs a body's card rules for one player: nothing without a booking or a red card; otherwise the player's entry in
/// the body (`RulingBody::find_or_add_discipline_entry`, created when missing), the verdict and its effects.
pub(crate) unsafe fn run_card_rules(this: usize, row: usize, fixture: usize, verdict: impl FnOnce(usize) -> CardVerdict) {
    let row = MatchRecordRow(row);
    if row.yellow_cards() == 0 && row.red_card() == 0 {
        return;
    }
    let e = exe::rulingbody_find_or_add_discipline_entry(this, row.staff_id() as u32, 1);
    let v = verdict(e);
    apply_card_verdict(this, row, fixture, &v);
}

/// Comparer of the card rules: the bans cannot be handed out twice, so the copy's verdict is taken first on a copy of
/// the player's entry (the entry is found or created as the original does first), then only the original runs. Checked
/// afterwards: the entry's bookings, red cards and points; one more pending ban on the body's list when the verdict has
/// one; and, when the player had no ban in the body before, the new ban the entry points to (matches, days, start,
/// rules, reason, amount and flag bit 2).
pub(crate) unsafe fn compare_card_rules(this: usize, row: usize, original: impl FnOnce(), verdict: impl FnOnce(usize) -> CardVerdict, name: &str) {
    let row = MatchRecordRow(row);
    if row.yellow_cards() == 0 && row.red_card() == 0 {
        original();
        return;
    }
    let id = row.staff_id() as u32;
    let e = exe::rulingbody_find_or_add_discipline_entry(this, id, 1);
    let mut local = [0u8; 0xe];
    core::ptr::copy_nonoverlapping(e as *const u8, local.as_mut_ptr(), 0xe);
    let had_ban = read_i16(e + ruling_body_entry::BAN_INDEX) > -1;
    let list = read_usize(this + RULING_BODY_LIST);
    let queued = read_u16(list + llist::COUNT);
    let v = verdict(local.as_mut_ptr() as usize);
    original();
    let l = local.as_ptr() as usize;
    let e = ruling_body_entry(this, id);
    let mut ok = e != 0 && read_u8(e + ruling_body_entry::YELLOW_CARDS) == read_u8(l + ruling_body_entry::YELLOW_CARDS)
        && read_u8(e + ruling_body_entry::RED_CARDS) == read_u8(l + ruling_body_entry::RED_CARDS)
        && read_u16(e + ruling_body_entry::DISCIPLINE_POINTS) == read_u16(l + ruling_body_entry::DISCIPLINE_POINTS)
        && read_u16(list + llist::COUNT) == queued.wrapping_add(v.pending.is_some() as u16);
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
    SCOREBOARD.record(ok, || format!("{}(player {}): bookings {} red {} points {} (expected {} {} {}), queued {} (before {}), ban expected {:?}",
        name, id, read_u8(e + ruling_body_entry::YELLOW_CARDS), read_u8(e + ruling_body_entry::RED_CARDS), read_u16(e + ruling_body_entry::DISCIPLINE_POINTS),
        read_u8(l + ruling_body_entry::YELLOW_CARDS), read_u8(l + ruling_body_entry::RED_CARDS), read_u16(l + ruling_body_entry::DISCIPLINE_POINTS),
        read_u16(list + llist::COUNT), queued, v.ban.as_ref().map(|b| (b.matches, b.reason, b.amount))));
}

/// The English ruling body: vtable 0x96eb14, record slot 0, rule set 9, ban scale 5, England in Europe, league rule 9,
/// no cup rule, the European rule 1.
pub(crate) const ENGLAND_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96eb14, record_slot: 0, rule_set: 9, scale: 5,
    nation: Some(KEY_NATION_ENGLAND), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 9, cup_rule: 0xff, continental_rule: 1 };

/// Builds the English ruling body: the RulingBody base with the file, an empty date at +0x23 (`Date::Date`; the English
/// object is the only one allocated with 0x2b bytes) and the class fields of `ENGLAND_RULING_BODY`. Returns this.
/// Original: `RbEngland::RbEngland` (0x7dcf90, rb_england.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_england_construct(this: usize, file: usize) -> usize {
    construct_ruling_body(this, file);
    exe::date_date_54a110(this + 0x23); // the English body's own date
    covered!("sx18_1 english ruling body built");
    ruling_body_class_fields(this, &ENGLAND_RULING_BODY)
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
    let (mut matches, mut reason, mut amount) = (0u8, 0xffu8, 0i8);
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
                amount = total as i8;
            }
        }
    }
    if row.red_card() != 0 {
        covered!("sx18_1 english red card");
        let offence = match row.yellow_cards() {
            0 => row.first_card_offence(),
            1 => row.second_card_offence(),
            _ => -1,
        };
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
    CardVerdict { pending: None, ban }
}

/// Applies a player's cards of one match under the English rules (`england_card_verdict`): nothing without a booking or
/// a red card; the ban, when there is one, goes to the discipline with its news.
/// Original: `RbEngland::process_match_cards` (0x7dd010, rb_england.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_england_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| england_card_verdict(MatchRecordRow(row), Fixture(fixture), e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_england_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_ruling_body_class_built(r, this, &ENGLAND_RULING_BODY, "rb_england_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_england_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| england_card_verdict(MatchRecordRow(row), Fixture(fixture), e), "rb_england_process_match_cards");
}

/// The plain card rules several bodies share (Argentina, Asia, Europe, International): exactly one red card bans for one
/// match (reason 2, amount 1); a booking adds to the entry's count (all the match's bookings with `add_all`, otherwise
/// one) and every `every`-th booking bans for one more match (reason 1, amount `every`). The ban starts today, under
/// `rules`.
pub(crate) unsafe fn plain_card_verdict(row: MatchRecordRow, entry: usize, add_all: bool, every: u8, rules: [u8; 4]) -> CardVerdict {
    let (mut matches, mut reason, mut amount) = (0u8, 0xffu8, 0i8);
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
            amount = every as i8;
        }
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules, start: ban_start_today() });
    CardVerdict { pending: None, ban }
}

/// Europe's card rules (`plain_card_verdict`): one match for a red card and one for every third booking (counted one
/// per match), under the European rule set in the confederation slot (rules ff ff 01 ff).
pub(crate) unsafe fn europe_card_verdict(row: MatchRecordRow, entry: usize) -> CardVerdict {
    covered!("sx18_1 european cards");
    plain_card_verdict(row, entry, false, 3, [0xff, 0xff, 1, 0xff]) // sx18_1: every third European booking
}

/// Applies a player's cards of one match under the European rules (`europe_card_verdict`): nothing without a booking
/// or a red card; the ban, when there is one, starts today and goes to the discipline with its news.
/// Original: `RbEurope::process_match_cards` (0x7dd450, rb_europe.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_europe_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| europe_card_verdict(MatchRecordRow(row), e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_europe_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| europe_card_verdict(MatchRecordRow(row), e), "rb_europe_process_match_cards");
}
