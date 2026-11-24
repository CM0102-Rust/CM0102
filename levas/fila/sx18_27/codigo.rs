// ---------- sx18_27: the last "one booking from a ban" questions of the ruling bodies (ruling_body/rb_*.cpp) ----------

/// Whether one more booking bans the player under the body's rules (at an odd count of bookings): the outputs, when given, get one match and
/// no days (`one_booking_outputs`).
/// Original: `RbJapanCup::is_one_booking_from_ban` (0x7e0450, rb_japan_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_japan_cup_is_one_booking_from_ban(_this: usize, e: usize, matches: usize, days: usize, _competition: u32) -> u32 {
    one_booking_outputs(matches, days);
    ((read_u8(e + ruling_body_entry::YELLOW_CARDS) as u32 + 1) % 2 == 0) as u32
}

pub(crate) unsafe extern "thiscall" fn compare_rb_japan_cup_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I0%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_japan_cup_is_one_booking_from_ban(this, e, m, d, competition), "rb_japan_cup_is_one_booking_from_ban")
}

/// Whether one more booking bans the player under the body's rules (at 2, 5, 8... bookings (every three, which is not the Korean rule: a defect of the original)): the outputs, when given, get one match and
/// no days (`one_booking_outputs`).
/// Original: `RbKorea::is_one_booking_from_ban` (0x7e0aa0, rb_korea.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_korea_is_one_booking_from_ban(_this: usize, e: usize, matches: usize, days: usize, _competition: u32) -> u32 {
    one_booking_outputs(matches, days);
    let y = read_u8(e + ruling_body_entry::YELLOW_CARDS) as i32;
    (y + 1 >= 3 && (y - 2) % 3 == 0) as u32
}

pub(crate) unsafe extern "thiscall" fn compare_rb_korea_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I1%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_korea_is_one_booking_from_ban(this, e, m, d, competition), "rb_korea_is_one_booking_from_ban")
}

/// Whether one more booking takes the player over a Northern Irish points threshold (`northern_ireland_one_booking`):
/// within 4 points under 20, 32, 42 or 51, writing the ban of 1, 2, 4 or 6 matches; the days output gets 0xff.
/// Original: `RbNorthernIreland::is_one_booking_from_ban` (0x7e0e20, rb_northern_ireland.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_northern_ireland_is_one_booking_from_ban(_this: usize, e: usize, matches: usize, days: usize, _competition: u32) -> u32 {
    northern_ireland_one_booking(e, matches, days)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_northern_ireland_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I2%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_northern_ireland_is_one_booking_from_ban(this, e, m, d, competition), "rb_northern_ireland_is_one_booking_from_ban")
}

/// Whether one more booking bans the player under the body's rules (at 1 booking): the outputs, when given, get one match and
/// no days (`one_booking_outputs`).
/// Original: `RbNorwayCup::is_one_booking_from_ban` (0x7e1120, rb_norway_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rb_norway_cup_is_one_booking_from_ban(_this: usize, e: usize, matches: usize, days: usize, _competition: u32) -> u32 {
    one_booking_outputs(matches, days);
    (read_u8(e + ruling_body_entry::YELLOW_CARDS) == 1) as u32
}

pub(crate) unsafe extern "thiscall" fn compare_rb_norway_cup_is_one_booking_from_ban(this: usize, e: usize, matches: usize, days: usize, competition: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I3%%);
    compare_one_booking(matches, days, || o(this, e, matches, days, competition), |m, d| rb_norway_cup_is_one_booking_from_ban(this, e, m, d, competition), "rb_norway_cup_is_one_booking_from_ban")
}
