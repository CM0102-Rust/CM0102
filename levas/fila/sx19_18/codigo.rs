// ---------- sx19_18: whether a ruling body's ban covers a match, clearing a club's bookings and dropping a player's pending bans (ruling_body/ruling_body.cpp) ----------

/// Whether date `a` (day i16 +0, year u16 +2) comes before date `b`: the year unsigned, then the day signed.
unsafe fn ban_date_before(a: usize, b: usize) -> bool {
    let (ya, yb) = (read_u16(a + 2), read_u16(b + 2));
    ya < yb || (ya == yb && read_i16(a) < read_i16(b))
}

/// A ban that has not started yet against a match: the player's club in the fixture (home when the fixture's home club
/// is his club or his national team, otherwise away) and that side's mark pick the club's fixtures from the ban's start
/// to the match (`FIXTURE_MANAGER::find_fixtures` into a local list, any competition and kind, the side filter = the mark
/// is set); the unplayed ones (home goals 0xff) of a competition under the same discipline rule set as the fixture's are
/// counted. Covered when the ban's matches are at least that count; a ban starting after the match covers nothing.
unsafe fn ban_covers_later_match(ban: usize, person: Staff, fixture: Fixture) -> bool {
    let mut l = [0u8; 14];
    let list = Llist(l.as_mut_ptr() as usize);
    exe::new_llist(list);
    let mut md = [0u8; 8];
    exe::new_date(md.as_mut_ptr() as usize, fixture.day() as u32, fixture.year() as u32);
    let m = md.as_ptr() as usize;
    if ban_date_before(m, ban + ban_record::START_DATE) {
        exe::drop_llist(list);
        return false;
    }
    let home = fixture.home_club().addr();
    let (club, mark) = if read_usize(person.addr() + staff::CLUB_JOB) == home || read_usize(person.addr() + staff::NATIONAL_JOB) == home {
        (home, fixture.home_side_mark())
    } else {
        (fixture.away_club().addr(), fixture.away_side_mark() as u8)
    };
    exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), read_u16(ban + ban_record::START_DATE) as u32,
        read_u16(ban + ban_record::START_DATE + 2) as u32, read_u16(m) as u32, read_u16(m + 2) as u32, read_u32(club), list, u32::MAX,
        (-2i32) as u32, (mark != 0) as u32);
    let rule_of = |comp: i32| League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((comp as usize).wrapping_mul(4)))).discipline_rule();
    list.set_cursor(list.head());
    let mut count = 0i16;
    let mut f = exe::llist_get_next(list);
    while f != 0 {
        let other = Fixture(f);
        if other.home_goals() as u8 == 0xff && other.comp_id() >= 0 && fixture.comp_id() >= 0 {
            let r = rule_of(other.comp_id());
            if r != 0xff && r == rule_of(fixture.comp_id()) { // sx19_18: a match of the same rule set still to play
                count += 1;
            }
        }
        f = exe::llist_get_next(list);
    }
    exe::drop_llist(list);
    read_i8(ban + ban_record::MATCHES) as i16 >= count
}

/// Whether this ruling body's active ban on the player covers a match.
///
/// No entry in the body (`ruling_body_entry_index`): no. A body without entries is the error box of line 0x330 and no.
/// First the player's bans move on (`Discipline::update_ban_progress` with the body's rule set and 0 matches); then
/// the entry's ban (none: no). With `flagged_only`, a ban without flag bit 1 does not count. A ban that has not started
/// yet (today before its start) only counts against a later match (`ban_covers_later_match`; no fixture: no). A
/// started ban by days covers any match without a fixture, and a fixture's match when the days from the ban's start
/// to the match are fewer than the ban's; a ban by matches covers when the matches served plus `extra` are at most its
/// matches.
/// Original: `RulingBody::is_player_banned` (0x7e85b0, ruling_body.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn is_player_banned(this: usize, person: usize, flagged_only: u32, extra: i16, fixture: usize) -> u32 {
    let d = read_usize(G_DISCIPLINE);
    let id = read_u32(person);
    let k = ruling_body_entry_index(this, id);
    if k == -1 {
        return 0;
    }
    if read_usize(this + ruling_body::ENTRIES) == 0 {
        show_error_at(RULING_BODY_CPP, 0x330);
        return 0;
    }
    exe::discipline_update_ban_progress(d, id, read_u8(this + ruling_body::RULE_SET_INDEX) as u32, 0);
    let b = read_i16(ruling_body_entry_at(this, k) + ruling_body_entry::BAN_INDEX);
    if b == -1 {
        return 0;
    }
    let ban = read_usize(d + discipline_fields::BANS).wrapping_add((b as isize * 20) as usize);
    if flagged_only != 0 && read_u8(ban + ban_record::FLAGS) & 2 == 0 {
        return 0;
    }
    if ban_date_before(G_CURRENT_DATE, ban + ban_record::START_DATE) {
        covered!("sx19_18 ban not started");
        return (fixture != 0 && ban_covers_later_match(ban, Staff(person), Fixture(fixture))) as u32;
    }
    if read_i16(ban + ban_record::DAYS) != -1 {
        if fixture == 0 {
            return 1;
        }
        covered!("sx19_18 ban by days");
        let mut md = [0u8; 8];
        exe::new_date(md.as_mut_ptr() as usize, Fixture(fixture).day() as u32, Fixture(fixture).year() as u32);
        let days = exe::date_days_since(md.as_ptr() as usize, read_u32(ban + ban_record::START_DATE), read_u32(ban + ban_record::START_DATE + 4));
        return (days < read_i16(ban + ban_record::DAYS) as i32) as u32;
    }
    covered!("sx19_18 ban by matches");
    (read_i8(ban + ban_record::MATCHES_SERVED) as i32 + extra as i32 <= read_i8(ban + ban_record::MATCHES) as i32) as u32 // sx19_18: matches still to serve
}

/// Clears the bookings of every player of the club in this body (all players with a null club); entries without a
/// player (−1) are skipped.
/// Original: `RulingBody::reset_bookings_for_club` (0x7e9180, ruling_body.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn reset_bookings_for_club(this: usize, club: usize) {
    let mut k = 0;
    while k < read_i32(this + ruling_body::ENTRY_COUNT) {
        let e = ruling_body_entry_at(this, k);
        let id = read_i32(e + ruling_body_entry::PERSON_ID);
        if id != -1 && (club == 0 || read_usize(staff_at(id as isize).addr() + staff::CLUB_JOB) == club) {
            covered!("sx19_18 bookings cleared");
            write_u8(e + ruling_body_entry::YELLOW_CARDS, 0); // sx19_18: the club's bookings cleared
        }
        k += 1;
    }
}

/// Removes every pending ban of the player from this body's list (`LLIST::remove` on the nodes whose person is his).
/// Original: `RulingBody::remove_pending_bans` (0x7e91d0, ruling_body.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn remove_pending_bans(this: usize, id: u32) {
    let list = Llist(read_usize(this + RULING_BODY_LIST));
    if list.is_null() {
        return;
    }
    list.set_cursor(list.head());
    let mut node = exe::llist_get_next(list);
    while node != 0 {
        if read_u32(node + pending_ban::PERSON_ID) == id { // sx19_18: the player's pending ban
            covered!("sx19_18 pending ban removed");
            exe::llist_remove(list, node);
        }
        node = exe::llist_get_next(list);
    }
}

/// Comparer of `is_player_banned`: the original runs first (it moves the player's bans on); the copy then asks the same
/// question (moving them on again changes nothing) and the answers are compared.
pub(crate) unsafe extern "thiscall" fn compare_is_player_banned(this: usize, person: usize, flagged_only: u32, extra: i16, fixture: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, i16, usize) -> u32>(%%I0%%)(this, person, flagged_only, extra, fixture);
    let n = is_player_banned(this, person, flagged_only, extra, fixture);
    SCOREBOARD.record(r == n, || format!("is_player_banned(player {}, {}, {}, fixture {:#x}): original {} rust {}", read_u32(person), flagged_only, extra, fixture, r, n));
    r
}

/// Comparer of `reset_bookings_for_club`: the copy clears a copy of the entries, the original runs, the bytes are
/// compared.
pub(crate) unsafe extern "thiscall" fn compare_reset_bookings_for_club(this: usize, club: usize) {
    let entries = read_usize(this + ruling_body::ENTRIES);
    let n = read_i32(this + ruling_body::ENTRY_COUNT).max(0) as usize;
    let mut copy = core::slice::from_raw_parts(entries as *const u8, n * RULING_BODY_ENTRY_SIZE).to_vec();
    for k in 0..n {
        let e = copy.as_mut_ptr() as usize + k * RULING_BODY_ENTRY_SIZE;
        let id = read_i32(e + ruling_body_entry::PERSON_ID);
        if id != -1 && (club == 0 || read_usize(staff_at(id as isize).addr() + staff::CLUB_JOB) == club) {
            write_u8(e + ruling_body_entry::YELLOW_CARDS, 0);
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, club);
    let got = core::slice::from_raw_parts(entries as *const u8, n * RULING_BODY_ENTRY_SIZE);
    SCOREBOARD.record(got == &copy[..], || format!("reset_bookings_for_club({:#x}, club {:#x}): {} entries differ", this, club, n));
}

/// Comparer of `remove_pending_bans`: only the original runs; afterwards no node of the body's list is the player's.
pub(crate) unsafe extern "thiscall" fn compare_remove_pending_bans(this: usize, id: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I2%%)(this, id);
    let list = Llist(read_usize(this + RULING_BODY_LIST));
    if list.is_null() {
        return;
    }
    let mut left = 0;
    list.set_cursor(list.head());
    let mut node = exe::llist_get_next(list);
    while node != 0 {
        if read_u32(node + pending_ban::PERSON_ID) == id {
            left += 1;
        }
        node = exe::llist_get_next(list);
    }
    SCOREBOARD.record(left == 0, || format!("remove_pending_bans(player {}): {} nodes left", id, left));
}
