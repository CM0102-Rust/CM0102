// ---------- sx18_26: the last names and the ruling bodies' "one booking from a ban" questions (ruling_body/rb_*.cpp) ----------

/// The two outputs of the "one booking from a ban" question, when given: the ban's matches (1) and no days (0xff).
pub(crate) unsafe fn one_booking_outputs(matches: usize, days: usize) {
    if matches != 0 {
        write_u8(matches, 1);
    }
    if days != 0 {
        write_u8(days, 0xff);
    }
}

/// Comparer of the "one booking from a ban" questions: the copy answers into local outputs first, then the original
/// into the caller's; the answers and the outputs that were given must match.
pub(crate) unsafe fn compare_one_booking(matches: usize, days: usize, original: impl FnOnce() -> u32, copy: impl FnOnce(usize, usize) -> u32, name: &str) -> u32 {
    let (mut m, mut d) = (0u8, 0u8);
    if matches != 0 {
        m = read_u8(matches);
    }
    if days != 0 {
        d = read_u8(days);
    }
    let n = copy(if matches != 0 { &mut m as *mut u8 as usize } else { 0 }, if days != 0 { &mut d as *mut u8 as usize } else { 0 });
    let r = original();
    let ok = r == n && (matches == 0 || read_u8(matches) == m) && (days == 0 || read_u8(days) == d);
    SCOREBOARD.record(ok, || format!("{}: original {} copy {}, matches {:?} {}, days {:?} {}", name, r, n,
        (matches != 0).then(|| read_u8(matches)), m, (days != 0).then(|| read_u8(days)), d));
    r
}

/// Whether three more Danish points reach a threshold (15, 30, then every 14) that `points` is still under.
pub(crate) fn denmark_points_threshold_ahead(points: i32) -> bool {
    let next = points + 3;
    let mut threshold = 0xf;
    while threshold <= next {
        if points < threshold {
            return true;
        }
        threshold = if threshold == 0xf { 0x1e } else { threshold + 0xe };
    }
    false
}

/// The Northern Irish answer: within 4 points under 20, 32, 42 or 51, the ban of 1, 2, 4 or 6 matches (into `matches`
/// when given); `days` gets no days (0xff) in any case.
unsafe fn northern_ireland_one_booking(e: usize, matches: usize, days: usize) -> u32 {
    if days != 0 {
        write_u8(days, 0xff);
    }
    let points = read_u16(e + ruling_body_entry::DISCIPLINE_POINTS);
    for (low, ban) in [(0x10u16, 1u8), (0x1c, 2), (0x26, 4), (0x2f, 6)] { // sx18_26: the Northern Irish points bands
        if points >= low && points < low + 4 {
            if matches != 0 {
                write_u8(matches, ban);
            }
            return 1;
        }
    }
    0
}

/// Writes the South Korean nationality adjective into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xbb).
/// Original: `RbKorea::get_name` (0x7e09e0, rb_korea.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_korea_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Nationality(KEY_NATION_SOUTH_KOREA), 0xa79b68, 0xbb)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_korea_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I0%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_korea_get_name(this, b, len, form), "rb_korea_get_name")
}

/// Writes the Northern Irish nationality adjective into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xc3).
/// Original: `RbNorthernIreland::get_name` (0x7e0d60, rb_northern_ireland.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_northern_ireland_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Nationality(KEY_NATION_NORTHERN_IRELAND), 0xa79ba0, 0xc3)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_northern_ireland_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I1%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_northern_ireland_get_name(this, b, len, form), "rb_northern_ireland_get_name")
}

/// Writes "Norwegian cup" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xa4).
/// Original: `RbNorwayCup::get_name` (0x7e1080, rb_norway_cup.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_norway_cup_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa79be4), 0xa79bf4, 0xa4)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_norway_cup_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I2%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_norway_cup_get_name(this, b, len, form), "rb_norway_cup_get_name")
}

/// Whether one more booking bans the player under the body's rules (at 4 bookings or an even count from 6): the outputs, when given, get one match and
/// no days (`one_booking_outputs`).
/// Original: `RbAustralia::is_one_booking_from_ban` (0x7db8a0, rb_australia.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_australia_is_one_booking_from_ban(_this: usize, e: usize, matches: usize, days: usize, _competition: u32) -> u32 {
    one_booking_outputs(matches, days);
    let y = read_u8(e + ruling_body_entry::YELLOW_CARDS);
    (y == 4 || (y >= 6 && (y - 6) % 2 == 0)) as u32
}

pub(crate) unsafe extern "thiscall" fn compare_rb_australia_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I3%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_australia_is_one_booking_from_ban(this, e, m, d, competition), "rb_australia_is_one_booking_from_ban")
}

/// Whether one more booking bans the player under the body's rules (when 3 more points reach a threshold (15, 30, then every 14) the points are still under): the outputs, when given, get one match and
/// no days (`one_booking_outputs`).
/// Original: `RbDenmark::is_one_booking_from_ban` (0x7dcf30, rb_denmark.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_denmark_is_one_booking_from_ban(_this: usize, e: usize, matches: usize, days: usize, _competition: u32) -> u32 {
    one_booking_outputs(matches, days);
    (denmark_points_threshold_ahead(read_u16(e + ruling_body_entry::DISCIPLINE_POINTS) as i32)) as u32
}

pub(crate) unsafe extern "thiscall" fn compare_rb_denmark_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I4%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_denmark_is_one_booking_from_ban(this, e, m, d, competition), "rb_denmark_is_one_booking_from_ban")
}

/// Whether one more booking bans the player under the body's rules (at 4, 9, 14 or 19 bookings, except the 4th between days 101 and 170 of the year): the outputs, when given, get one match and
/// no days (`one_booking_outputs`).
/// Original: `RbEngland::is_one_booking_from_ban` (0x7dd3a0, rb_england.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_england_is_one_booking_from_ban(_this: usize, e: usize, matches: usize, days: usize, _competition: u32) -> u32 {
    one_booking_outputs(matches, days);
    let y = read_u8(e + ruling_body_entry::YELLOW_CARDS);
    let day = read_i16(G_CURRENT_DATE);
    (matches!(y, 4 | 9 | 14 | 19) && !(day > 0x64 && day < 0xab && y < 5)) as u32
}

pub(crate) unsafe extern "thiscall" fn compare_rb_england_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I5%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_england_is_one_booking_from_ban(this, e, m, d, competition), "rb_england_is_one_booking_from_ban")
}

/// Whether one more booking bans the player under the body's rules (when the booking window, without the oldest match, already holds 2 or more): the outputs, when given, get one match and
/// no days (`one_booking_outputs`).
/// Original: `RbFrance::is_one_booking_from_ban` (0x7de260, rb_france.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_france_is_one_booking_from_ban(_this: usize, e: usize, matches: usize, days: usize, _competition: u32) -> u32 {
    one_booking_outputs(matches, days);
    let window = ((read_u8(e + 0xa) >> 1) as u32).count_ones() + (read_u8(e + 0xb) & 1) as u32 + (read_u8(e + 0xc) & 1) as u32; // the French booking window
    (window as i8 >= 2) as u32
}

pub(crate) unsafe extern "thiscall" fn compare_rb_france_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I6%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_france_is_one_booking_from_ban(this, e, m, d, competition), "rb_france_is_one_booking_from_ban")
}

/// Whether one more booking bans the player under the body's rules (when the next booking is a multiple of 3): the outputs, when given, get one match and
/// no days (`one_booking_outputs`).
/// Original: `RbGreece::is_one_booking_from_ban` (0x7ded50, rb_greece.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_greece_is_one_booking_from_ban(_this: usize, e: usize, matches: usize, days: usize, _competition: u32) -> u32 {
    one_booking_outputs(matches, days);
    ((read_u8(e + ruling_body_entry::YELLOW_CARDS) as u32 + 1) % 3 == 0) as u32
}

pub(crate) unsafe extern "thiscall" fn compare_rb_greece_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I7%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_greece_is_one_booking_from_ban(this, e, m, d, competition), "rb_greece_is_one_booking_from_ban")
}

/// Whether one more booking bans the player under the body's rules (at 2 bookings): the outputs, when given, get one match and
/// no days (`one_booking_outputs`).
/// Original: `RbHolland::is_one_booking_from_ban` (0x7df150, rb_holland.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_holland_is_one_booking_from_ban(_this: usize, e: usize, matches: usize, days: usize, _competition: u32) -> u32 {
    one_booking_outputs(matches, days);
    (read_u8(e + ruling_body_entry::YELLOW_CARDS) == 2) as u32
}

pub(crate) unsafe extern "thiscall" fn compare_rb_holland_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I8%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_holland_is_one_booking_from_ban(this, e, m, d, competition), "rb_holland_is_one_booking_from_ban")
}

/// Whether one more booking bans the player under the body's rules (at 4, 7, 10 or every 3 from 13 bookings): the outputs, when given, get one match and
/// no days (`one_booking_outputs`).
/// Original: `RbIreland::is_one_booking_from_ban` (0x7df910, rb_ireland.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_ireland_is_one_booking_from_ban(_this: usize, e: usize, matches: usize, days: usize, _competition: u32) -> u32 {
    one_booking_outputs(matches, days);
    let y = read_u8(e + ruling_body_entry::YELLOW_CARDS);
    (matches!(y, 4 | 7 | 0xa) || (y >= 0xd && (y - 0xd) % 3 == 0)) as u32
}

pub(crate) unsafe extern "thiscall" fn compare_rb_ireland_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I9%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_ireland_is_one_booking_from_ban(this, e, m, d, competition), "rb_ireland_is_one_booking_from_ban")
}

/// Whether one more booking bans the player under the body's rules (at an odd count of bookings): the outputs, when given, get one match and
/// no days (`one_booking_outputs`).
/// Original: `RbItalyCup::is_one_booking_from_ban` (0x7dfd30, rb_italy_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_italy_cup_is_one_booking_from_ban(_this: usize, e: usize, matches: usize, days: usize, _competition: u32) -> u32 {
    one_booking_outputs(matches, days);
    (read_u8(e + ruling_body_entry::YELLOW_CARDS) % 2 == 1) as u32
}

pub(crate) unsafe extern "thiscall" fn compare_rb_italy_cup_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I10%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_italy_cup_is_one_booking_from_ban(this, e, m, d, competition), "rb_italy_cup_is_one_booking_from_ban")
}

/// Whether one more booking bans the player under the body's rules (at 3, 6 or an even count above 7): the outputs, when given, get one match and
/// no days (`one_booking_outputs`).
/// Original: `RbItalyLeague::is_one_booking_from_ban` (0x7e0160, rb_italy_league.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_italy_league_is_one_booking_from_ban(_this: usize, e: usize, matches: usize, days: usize, _competition: u32) -> u32 {
    one_booking_outputs(matches, days);
    let y = read_u8(e + ruling_body_entry::YELLOW_CARDS);
    (y == 3 || y == 6 || (y > 7 && y % 2 == 0)) as u32
}

pub(crate) unsafe extern "thiscall" fn compare_rb_italy_league_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I11%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_italy_league_is_one_booking_from_ban(this, e, m, d, competition), "rb_italy_league_is_one_booking_from_ban")
}
