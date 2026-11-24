// ---------- sx19_14: whether one more booking bans the player, per ruling body (ruling_body/rb_*.cpp, is_one_booking_from_ban) ----------

/// The entry's booking count (`ruling_body_entry::YELLOW_CARDS`).
unsafe fn entry_bookings(entry: usize) -> u8 {
    read_u8(entry + ruling_body_entry::YELLOW_CARDS)
}

/// Whether the next booking bans under the Norwegian league rules: the count is 2 or above 3 (one match, no days: `one_booking_outputs`).
/// Original: `RbNorwayLeague::is_one_booking_from_ban` (0x7e1390, rb_norway_league.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_norway_league_is_one_booking_from_ban(_this: usize, entry: usize, matches: usize, days: usize, _competition: u32) -> u32 {
    one_booking_outputs(matches, days);
    let n = entry_bookings(entry);
    (n == 2 || n > 3) as u32 // sx19_14: the Norwegian warning
}

/// Whether the next booking bans under the Polish rules: the count is 3, 6 or 9, or odd from 10 (one match, no days: `one_booking_outputs`).
/// Original: `RbPoland::is_one_booking_from_ban` (0x7e1a10, rb_poland.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_poland_is_one_booking_from_ban(_this: usize, entry: usize, matches: usize, days: usize, _competition: u32) -> u32 {
    one_booking_outputs(matches, days);
    let n = entry_bookings(entry);
    (n == 3 || n == 6 || n == 9 || (n >= 0xa && n % 2 == 1)) as u32
}

/// Whether the next booking bans under the Portuguese rules: the count plus one is a multiple of the step, 3 in the
/// competition whose record is the Portuguese Second Division B and 5 in the others (one match, no days: `one_booking_outputs`).
/// Original: `RbPortugal::is_one_booking_from_ban` (0x7e1db0, rb_portugal.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_portugal_is_one_booking_from_ban(_this: usize, entry: usize, matches: usize, days: usize, competition: u32) -> u32 {
    one_booking_outputs(matches, days);
    let step = if competition != 0 && read_u32(competition as usize) == read_u32(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B) { 3 } else { 5 }; // sx19_14: the Portuguese steps
    ((entry_bookings(entry) as u32 + 1) % step == 0) as u32
}

/// Whether the next booking completes the Russian cycle of four: the count is 3 (one-match warning; the cycle itself
/// never bans, see `russia_card_verdict`).
/// Original: `RbRussia::is_one_booking_from_ban` (0x7e2170, rb_russia.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_russia_is_one_booking_from_ban(_this: usize, entry: usize, matches: usize, days: usize, _competition: u32) -> u32 {
    one_booking_outputs(matches, days);
    (entry_bookings(entry) == 3) as u32
}

/// Whether the next booking bans under the Scottish cup rules: the player carries a booking (one match, no days: `one_booking_outputs`).
/// Original: `RbScotlandCup::is_one_booking_from_ban` (0x7e2440, rb_scotland_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_scotland_cup_is_one_booking_from_ban(_this: usize, entry: usize, matches: usize, days: usize, _competition: u32) -> u32 {
    one_booking_outputs(matches, days);
    (entry_bookings(entry) as u32 + 1 >= 2) as u32
}

/// Whether two more disciplinary points reach the Scottish 16-point ban, with the ban's length: this season's points
/// bans plus the two past seasons' (entry bytes +0xb and +0xa) plus one.
/// Original: `RbScotlandLeague::is_one_booking_from_ban` (0x7e2930, rb_scotland_league.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_scotland_league_is_one_booking_from_ban(_this: usize, entry: usize, matches: usize, days: usize, _competition: u32) -> u32 {
    let past = read_u8(entry + 0xa); // the points bans of two seasons ago (bits 0..2) and last season (bits 3..5)
    let season = read_u8(entry + 0xb) & 7; // this season's points bans
    if matches != 0 {
        write_u8(matches, ((past >> 3) & 7).wrapping_add(season).wrapping_add(past & 7).wrapping_add(1));
    }
    if days != 0 {
        write_u8(days, 0xff);
    }
    (read_u16(entry + ruling_body_entry::DISCIPLINE_POINTS) as u32 + 2 >= 0x10) as u32
}

/// Whether the next booking bans under the Swedish rules (league and cup): the count is even and not zero.
/// Original: `RbSwedenLeague::is_one_booking_from_ban` (0x7e39c0, rb_sweden_league.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_sweden_league_is_one_booking_from_ban(_this: usize, entry: usize, matches: usize, days: usize, _competition: u32) -> u32 {
    one_booking_outputs(matches, days);
    let n = entry_bookings(entry);
    (n % 2 == 0 && n != 0) as u32
}

/// Whether the next booking bans under the Turkish rules (league and cup): the count plus one is a multiple of four.
/// Original: `RbTurkeyLeague::is_one_booking_from_ban` (0x7e40b0, rb_turkey_league.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_turkey_league_is_one_booking_from_ban(_this: usize, entry: usize, matches: usize, days: usize, _competition: u32) -> u32 {
    one_booking_outputs(matches, days);
    ((entry_bookings(entry) as u32 + 1) % 4 == 0) as u32
}

/// Whether the next booking bans under the US rules: the disciplinary points stand at exactly 20.
/// Original: `RbUsa::is_one_booking_from_ban` (0x7e4390, rb_usa.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_usa_is_one_booking_from_ban(_this: usize, entry: usize, matches: usize, days: usize, _competition: u32) -> u32 {
    one_booking_outputs(matches, days);
    (read_u16(entry + ruling_body_entry::DISCIPLINE_POINTS) == 0x14) as u32
}

/// Whether the next booking is a fifth (Wales and the Spanish league): the count plus one is a multiple of five.
/// Original: `RbWales::is_one_booking_from_ban` (0x7e4710, rb_wales.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_wales_is_one_booking_from_ban(_this: usize, entry: usize, matches: usize, days: usize, _competition: u32) -> u32 {
    one_booking_outputs(matches, days);
    ((entry_bookings(entry) as u32 + 1) % 5 == 0) as u32
}


pub(crate) unsafe extern "thiscall" fn compare_rb_norway_league_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I0%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_norway_league_is_one_booking_from_ban(this, e, m, d, competition), "rb_norway_league_is_one_booking_from_ban")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_poland_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I1%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_poland_is_one_booking_from_ban(this, e, m, d, competition), "rb_poland_is_one_booking_from_ban")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_portugal_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I2%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_portugal_is_one_booking_from_ban(this, e, m, d, competition), "rb_portugal_is_one_booking_from_ban")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_russia_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I3%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_russia_is_one_booking_from_ban(this, e, m, d, competition), "rb_russia_is_one_booking_from_ban")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_scotland_cup_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I4%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_scotland_cup_is_one_booking_from_ban(this, e, m, d, competition), "rb_scotland_cup_is_one_booking_from_ban")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_scotland_league_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I5%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_scotland_league_is_one_booking_from_ban(this, e, m, d, competition), "rb_scotland_league_is_one_booking_from_ban")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_sweden_league_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I6%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_sweden_league_is_one_booking_from_ban(this, e, m, d, competition), "rb_sweden_league_is_one_booking_from_ban")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_turkey_league_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I7%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_turkey_league_is_one_booking_from_ban(this, e, m, d, competition), "rb_turkey_league_is_one_booking_from_ban")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_usa_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I8%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_usa_is_one_booking_from_ban(this, e, m, d, competition), "rb_usa_is_one_booking_from_ban")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_wales_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I9%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_wales_is_one_booking_from_ban(this, e, m, d, competition), "rb_wales_is_one_booking_from_ban")
}

/// Asks this body's own rule (vtable slot 0x14, `is_one_booking_from_ban`) whether one more booking bans the player,
/// on his entry (found, not added); without an entry, no.
/// Original: `RulingBody::check_one_booking_from_ban` (0x7e8d60, ruling_body.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn check_one_booking_from_ban(this: usize, id: u32, matches: usize, days: usize, competition: u32) -> u32 {
    let e = exe::rulingbody_find_or_add_discipline_entry(this, id, 0);
    if e == 0 {
        return 0;
    }
    let question: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this) + 0x14)); // slot 0x14
    question(this, e, matches, days, competition)
}

pub(crate) unsafe extern "thiscall" fn compare_check_one_booking_from_ban(this: usize, id: u32, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> u32>(%%I10%%);
    compare_one_booking(matches, days, || o(this, id, matches, days, competition), |m, d| check_one_booking_from_ban(this, id, m, d, competition), "check_one_booking_from_ban")
}
