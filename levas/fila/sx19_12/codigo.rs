// ---------- sx19_12: the Scottish league body's new season and the Scottish cup ruling body (constructor and card rules) (ruling_body/rb_scotland_league.cpp, rb_scotland_cup.cpp) ----------

/// Rolls one Scottish entry into a new season: last season's points bans become the two-seasons-ago count, this
/// season's become last season's (the top two bits of +0xa are kept), this season's count and the points go to zero.
pub(crate) unsafe fn scotland_entry_new_season(e: usize) {
    let (past, season) = (read_u8(e + SCOTLAND_PAST_BANS), read_u8(e + SCOTLAND_SEASON_BANS));
    write_u8(e + SCOTLAND_PAST_BANS, ((past >> 3) & 7) | ((season & 7) << 3) | (past & 0xc0)); // sx19_12: the bans move back a season
    write_u8(e + SCOTLAND_SEASON_BANS, season & 0xf8);
    write_u16(e + ruling_body_entry::DISCIPLINE_POINTS, 0);
}

/// At the new season, rolls every entry of the Scottish league body into it (`scotland_entry_new_season`): unlike the
/// other bodies, no entry is freed and the bookings and red cards stay.
/// Original: `RbScotlandLeague::start_new_season` (0x7e2980, rb_scotland_league.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn rb_scotland_league_start_new_season(this: usize) {
    covered!("sx19_12 scottish new season");
    let entries = read_usize(this + ruling_body::ENTRIES);
    for k in 0..read_i32(this + ruling_body::ENTRY_COUNT).max(0) as usize {
        scotland_entry_new_season(entries + k * 0xe);
    }
}

/// The Scottish cup ruling body: vtable 0x96edf4, record slot 1, rule set 0x19, ban scale 5, Scotland in Europe, league
/// rule 0x18, cup rule 0x19, the European rule 1.
pub(crate) const SCOTLAND_CUP_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96edf4, record_slot: 1, rule_set: 0x19, scale: 5,
    nation: Some(KEY_NATION_SCOTLAND), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0x18, cup_rule: 0x19, continental_rule: 1 };

/// Builds the Scottish cup ruling body (`ruling_body_class_construct` with `SCOTLAND_CUP_RULING_BODY`). Returns this.
/// Original: `RbScotlandCup::RbScotlandCup` (0x7e21a0, rb_scotland_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_scotland_cup_construct(this: usize, file: usize) -> usize {
    covered!("sx19_12 scottish cup body built");
    ruling_body_class_construct(this, file, &SCOTLAND_CUP_RULING_BODY)
}

/// Whether a Scottish cup red card goes to the league instead: exactly one red card with fewer than two bookings in the
/// match (a straight red).
pub(crate) unsafe fn scotland_cup_red_to_league(row: MatchRecordRow) -> bool {
    row.red_card() == 1 && row.yellow_cards() < 2
}

/// The league's part of a straight red in the Scottish cup: 12 disciplinary points on the player's Scottish league
/// entry (created when missing) and the league body's points ban check (vtable slot 0xc, `check_points_ban`, with the
/// fixture). Returns whether the league handed out a ban.
pub(crate) unsafe fn scotland_cup_red_league_ban(row: MatchRecordRow, fixture: usize) -> bool {
    let league = ruling_body_of_rule(read_usize(G_DISCIPLINE), SCOTLAND_LEAGUE_RULING_BODY.rule_set);
    let e = exe::rulingbody_find_or_add_discipline_entry(league, row.staff_id() as u32, 1);
    write_u16(e + ruling_body_entry::DISCIPLINE_POINTS, read_u16(e + ruling_body_entry::DISCIPLINE_POINTS).wrapping_add(0xc)); // sx19_12: 12 league points
    let check: unsafe extern "thiscall" fn(usize, u32, usize) -> u32 = core::mem::transmute(read_usize(read_usize(league) + 0xc)); // slot 0xc: check_points_ban
    check(league, row.staff_id() as u32, fixture) != 0
}

/// The Scottish cup card rules' verdict. Any red card bans for one match (reason 2, amount 1). All the match's
/// bookings add to the entry's count; from two, the player is banned for one match per pair (reason 1, the count as the
/// amount) and the count keeps the odd one. A straight red (`scotland_cup_red_to_league`) also goes to the league
/// (`league_ban`); when the league hands out a ban, the cup hands out none. The ban starts today, under the cup rule set.
pub(crate) unsafe fn scotland_cup_card_verdict(row: MatchRecordRow, entry: usize, league_ban: impl FnOnce() -> bool) -> CardVerdict {
    let (mut matches, mut reason, mut amount) = (0u8, 0xffu8, 0i16);
    if row.red_card() != 0 {
        covered!("sx19_12 scottish cup red card");
        matches = 1;
        reason = 2;
        amount = 1;
    }
    let count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(row.yellow_cards());
    write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
    if count > 1 { // sx19_12: a Scottish cup ban per two bookings
        covered!("sx19_12 scottish cup booking ban");
        matches = matches.wrapping_add(count / 2);
        reason = 1;
        amount = count as i16;
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count % 2);
    }
    let by_league = scotland_cup_red_to_league(row) && league_ban();
    let ban = (matches != 0 && !by_league).then(|| ImmediateBan { matches, reason, amount, rules: own_rules(&SCOTLAND_CUP_RULING_BODY), start: ban_start_today() });
    CardVerdict { pending: Vec::new(), ban }
}

/// Applies a player's cards of one match under the Scottish cup rules (`scotland_cup_card_verdict`, a straight red
/// through `scotland_cup_red_league_ban`): nothing without a booking or a red card; the ban, when there is one, goes to
/// the discipline with its news.
/// Original: `RbScotlandCup::process_match_cards` (0x7e21f0, rb_scotland_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_scotland_cup_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| scotland_cup_card_verdict(MatchRecordRow(row), e, || scotland_cup_red_league_ban(MatchRecordRow(row), fixture)));
}

/// Comparer of the Scottish new season: the copy rolls a copy of the entries, the original runs, the bytes are
/// compared.
pub(crate) unsafe extern "thiscall" fn compare_rb_scotland_league_start_new_season(this: usize) {
    let entries = read_usize(this + ruling_body::ENTRIES);
    let n = read_i32(this + ruling_body::ENTRY_COUNT).max(0) as usize;
    let mut copy = core::slice::from_raw_parts(entries as *const u8, n * 0xe).to_vec();
    for k in 0..n {
        scotland_entry_new_season(copy.as_mut_ptr() as usize + k * 0xe);
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let got = core::slice::from_raw_parts(entries as *const u8, n * 0xe);
    let wrong = (0..n).find(|&k| got[k * 0xe..k * 0xe + 0xe] != copy[k * 0xe..k * 0xe + 0xe]);
    SCOREBOARD.record(wrong.is_none(), || format!("rb_scotland_league_start_new_season: entry {:?} of {}", wrong, n));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_scotland_cup_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(this, file);
    compare_ruling_body_class_built(r, this, &SCOTLAND_CUP_RULING_BODY, "rb_scotland_cup_construct")
}

/// Comparer of the Scottish cup card rules (`compare_card_rules`). A straight red also touches the league body (its
/// points and maybe a ban), which cannot happen twice: then only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_rb_scotland_cup_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%);
    if scotland_cup_red_to_league(MatchRecordRow(row)) {
        return o(this, row, fixture);
    }
    compare_card_rules(this, row, || o(this, row, fixture), |e| scotland_cup_card_verdict(MatchRecordRow(row), e, || false), "rb_scotland_cup_process_match_cards");
}
