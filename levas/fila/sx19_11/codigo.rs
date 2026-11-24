// ---------- sx19_11: the Scottish league ruling body: constructor, card rules (disciplinary points) and the points ban (ruling_body/rb_scotland_league.cpp) ----------

/// The Scottish league ruling body: vtable 0x96ee14, record slot 0, rule set 0x18, ban scale 5, Scotland in Europe,
/// league rule 0x18, cup rule 0x19, the European rule 1.
pub(crate) const SCOTLAND_LEAGUE_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ee14, record_slot: 0, rule_set: 0x18, scale: 5,
    nation: Some(KEY_NATION_SCOTLAND), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0x18, cup_rule: 0x19, continental_rule: 1 };

/// Byte +0xa of a Scottish entry: the points bans of two seasons ago (bits 0..2) and of last season (bits 3..5).
const SCOTLAND_PAST_BANS: usize = 0xa;
/// Byte +0xb of a Scottish entry: the points bans of this season (bits 0..2, up to 7).
const SCOTLAND_SEASON_BANS: usize = 0xb;

/// Builds the Scottish league ruling body (`ruling_body_class_construct` with `SCOTLAND_LEAGUE_RULING_BODY`). Returns
/// this.
/// Original: `RbScotlandLeague::RbScotlandLeague` (0x7e2470, rb_scotland_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_scotland_league_construct(this: usize, file: usize) -> usize {
    covered!("sx19_11 scottish league body built");
    ruling_body_class_construct(this, file, &SCOTLAND_LEAGUE_RULING_BODY)
}

/// The disciplinary points of an offence under the Scottish rules (signed code): up to 3 two points, up to 8 three, 9
/// five, 10 twelve, above that thirty-two.
pub(crate) fn scotland_offence_points(offence: i8) -> u16 {
    match offence {
        i8::MIN..=3 => 2,
        4..=8 => 3,
        9 => 5,
        10 => 0xc,
        _ => 0x20,
    }
}

/// The points of a Scottish red card (signed offence code, −1 none): none for −1, 5 for 9, 12 below 12, 32 from 12.
pub(crate) fn scotland_red_card_points(offence: i8) -> u16 {
    match offence {
        -1 => 0,
        9 => 5,
        i8::MIN..=0xb => 0xc,
        _ => 0x20,
    }
}

/// Turns each 16 disciplinary points of a Scottish entry into a points ban: for each, this season's ban count (up to 7)
/// goes up by one, the entry loses 16 points and the ban grows by this season's count plus the two past seasons' counts
/// (read once, before). Returns the matches added and the amount (16 per ban); (0, 0) under 16 points.
pub(crate) unsafe fn scotland_points_bans(entry: usize) -> (u8, u16) {
    let (mut matches, mut amount) = (0u8, 0u16);
    if read_u16(entry + ruling_body_entry::DISCIPLINE_POINTS) < 0x10 {
        return (0, 0);
    }
    let past = read_u8(entry + SCOTLAND_PAST_BANS);
    let (two_ago, last) = (past & 7, (past >> 3) & 7);
    while read_u16(entry + ruling_body_entry::DISCIPLINE_POINTS) >= 0x10 { // sx19_11: each 16 Scottish points
        covered!("sx19_11 scottish points ban");
        let season = read_u8(entry + SCOTLAND_SEASON_BANS);
        if season & 7 < 7 {
            write_u8(entry + SCOTLAND_SEASON_BANS, (season & !7) | ((season + 1) & 7));
        }
        let points = read_u16(entry + ruling_body_entry::DISCIPLINE_POINTS);
        write_u16(entry + ruling_body_entry::DISCIPLINE_POINTS, points.wrapping_sub(0x10));
        matches = matches.wrapping_add((read_u8(entry + SCOTLAND_SEASON_BANS) & 7).wrapping_add(two_ago).wrapping_add(last));
        amount += 0x10;
    }
    (matches, amount)
}

/// The Scottish league card rules' verdict. Exactly one red card bans for one match (reason 2, amount 1) and adds its
/// offence's points (`scotland_red_card_points`). Each booking adds its offence's points (`scotland_offence_points`;
/// the first booking's code, then the second's). Then each 16 points become a points ban (`scotland_points_bans`;
/// reason 0, the amount 16 per ban). The ban starts today after a red card and in 14 days otherwise, under the league
/// rule set.
pub(crate) unsafe fn scotland_league_card_verdict(row: MatchRecordRow, entry: usize) -> CardVerdict {
    let add_points = |p: u16| write_u16(entry + ruling_body_entry::DISCIPLINE_POINTS, read_u16(entry + ruling_body_entry::DISCIPLINE_POINTS).wrapping_add(p));
    let (mut matches, mut reason, mut amount) = (0u8, 0xffu8, 0i16);
    if row.red_card() == 1 {
        covered!("sx19_11 scottish red card");
        matches = 1;
        reason = 2;
        amount = 1;
        add_points(scotland_red_card_points(red_card_offence(row)));
    }
    for k in 0..row.yellow_cards() as i8 {
        covered!("sx19_11 scottish booking");
        let offence = if k == 0 { row.first_card_offence() } else { row.second_card_offence() };
        add_points(scotland_offence_points(offence)); // sx19_11: the points of a Scottish booking
    }
    let (bans, points_amount) = scotland_points_bans(entry);
    if points_amount != 0 {
        matches = matches.wrapping_add(bans);
        reason = 0;
        amount = points_amount as i16;
    }
    let start = if row.red_card() != 0 { ban_start_today() } else { ban_start_in(0xe) }; // sx19_11: two weeks without a red card
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules: own_rules(&SCOTLAND_LEAGUE_RULING_BODY), start });
    CardVerdict { pending: Vec::new(), ban }
}

/// Applies a player's cards of one match under the Scottish league rules (`scotland_league_card_verdict`): nothing
/// without a booking or a red card; the ban, when there is one, goes to the discipline with its news.
/// Original: `RbScotlandLeague::process_match_cards` (0x7e24c0, rb_scotland_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_scotland_league_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| scotland_league_card_verdict(MatchRecordRow(row), e));
}

/// The 20-byte record of a Scottish points ban: the matches, none served, no days, today's date, the league and cup
/// rule sets (18 19 ff ff), reason 0, the amount and the flags with bit 2 set. The original keeps the other flag bits
/// as stack garbage; here they are 0.
pub(crate) unsafe fn scotland_points_ban_record(matches: u8, amount: u16) -> [u8; 0x14] {
    let mut r = [0u8; 0x14];
    let p = r.as_mut_ptr() as usize;
    write_u8(p + ban_record::MATCHES, matches);
    write_u8(p + ban_record::MATCHES_SERVED, 0);
    write_u16(p + ban_record::DAYS, 0xffff);
    copy_record_date(p + ban_record::START_DATE, ban_start_today().as_ptr() as usize);
    let rules = [SCOTLAND_LEAGUE_RULING_BODY.league_rule, SCOTLAND_LEAGUE_RULING_BODY.cup_rule, 0xff, 0xff];
    for k in 0..4 {
        write_u8(p + ban_record::RULES + k, rules[k]);
    }
    write_u8(p + ban_record::REASON, 0);
    write_u16(p + ban_record::REASON_AMOUNT, amount);
    write_u8(p + ban_record::FLAGS, 4);
    r
}

/// Turns the player's Scottish disciplinary points into a ban starting today (`scotland_points_bans` on his entry,
/// created when missing): the ban goes to the discipline under the league and cup rule sets, with its news for the
/// fixture, and the routine returns 1; under 16 points, 0. Called by the Scottish cup's red card (vtable slot 0xc).
/// Original: `RbScotlandLeague::check_points_ban` (0x7e2720, rb_scotland_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_scotland_league_check_points_ban(this: usize, id: u32, fixture: usize) -> u32 {
    let entry = exe::rulingbody_find_or_add_discipline_entry(this, id, 1);
    let (matches, amount) = scotland_points_bans(entry);
    if matches == 0 {
        return 0;
    }
    let mut r = scotland_points_ban_record(matches, amount);
    add_ban(read_usize(G_DISCIPLINE), id, r.as_mut_ptr() as usize);
    let post: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_POST_BAN_NEWS);
    post(read_usize(G_DISCIPLINE), staff_at(id as isize).addr(), r.as_mut_ptr() as usize, fixture);
    1
}

pub(crate) unsafe extern "thiscall" fn compare_rb_scotland_league_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_ruling_body_class_built(r, this, &SCOTLAND_LEAGUE_RULING_BODY, "rb_scotland_league_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_scotland_league_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| scotland_league_card_verdict(MatchRecordRow(row), e), "rb_scotland_league_process_match_cards");
}

/// Comparer of the points ban: the ban cannot be handed out twice, so the copy works on a copy of the player's entry
/// (found or created as the original does first) and only the original runs. Checked afterwards: the return, the
/// entry's points and ban counts, and, when the player had no ban in the body before, the new ban's matches, rules,
/// reason, amount and flag bit 2.
pub(crate) unsafe extern "thiscall" fn compare_rb_scotland_league_check_points_ban(this: usize, id: u32, fixture: usize) -> u32 {
    let e = exe::rulingbody_find_or_add_discipline_entry(this, id, 1);
    let mut local = [0u8; 0xe];
    core::ptr::copy_nonoverlapping(e as *const u8, local.as_mut_ptr(), 0xe);
    let had_ban = read_i16(e + ruling_body_entry::BAN_INDEX) > -1;
    let l = local.as_mut_ptr() as usize;
    let (matches, amount) = scotland_points_bans(l);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> u32>(%%I2%%)(this, id, fixture);
    let e = ruling_body_entry(this, id);
    let state = |p: usize| (read_u16(p + ruling_body_entry::DISCIPLINE_POINTS), read_u8(p + SCOTLAND_PAST_BANS), read_u8(p + SCOTLAND_SEASON_BANS));
    let mut ok = e != 0 && r == (matches != 0) as u32 && state(e) == state(l);
    if ok && r == 1 && !had_ban {
        let s = read_i16(e + ruling_body_entry::BAN_INDEX);
        let b = read_usize(read_usize(G_DISCIPLINE) + discipline_fields::BANS).wrapping_add((s as i32 as usize).wrapping_mul(20));
        let x = scotland_points_ban_record(matches, amount);
        let p = x.as_ptr() as usize;
        ok = s > -1 && read_u8(b) == matches && read_u32(b + ban_record::RULES) == read_u32(p + ban_record::RULES)
            && read_u8(b + ban_record::REASON) == 0 && read_u16(b + ban_record::REASON_AMOUNT) == amount
            && read_u8(b + ban_record::FLAGS) & 4 == 4;
    }
    SCOREBOARD.record(ok, || format!("rb_scotland_league_check_points_ban(player {}): {} (expected {}, {} matches, amount {}), entry {:?} (expected {:?})",
        id, r, (matches != 0) as u32, matches, amount, if e != 0 { state(e) } else { (0, 0, 0) }, state(l)));
    r
}
