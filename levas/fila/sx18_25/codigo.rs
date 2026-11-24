// ---------- sx18_25: the ruling bodies' names (France to Norway) (ruling_body/rb_*.cpp) ----------

/// Writes "Finnish league" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xb4).
/// Original: `RbFinlandLeague::get_name` (0x7ddc20, rb_finland_league.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_finland_league_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa79810), 0xa79820, 0xb4)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_finland_league_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I0%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_finland_league_get_name(this, b, len, form), "rb_finland_league_get_name")
}

/// Writes the French nationality adjective into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xe6).
/// Original: `RbFrance::get_name` (0x7de1a0, rb_france.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_france_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Nationality(KEY_NATION_FRANCE), 0xa79860, 0xe6)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_france_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I1%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_france_get_name(this, b, len, form), "rb_france_get_name")
}

/// Writes "German cup" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xba).
/// Original: `RbGermanyCup::get_name` (0x7de580, rb_germany_cup.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_germany_cup_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa79898), 0xa798a4, 0xba)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_germany_cup_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I2%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_germany_cup_get_name(this, b, len, form), "rb_germany_cup_get_name")
}

/// Writes "German league" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xba).
/// Original: `RbGermanyLeague::get_name` (0x7de8c0, rb_germany_league.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_germany_league_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa798e4), 0xa798f4, 0xba)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_germany_league_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I3%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_germany_league_get_name(this, b, len, form), "rb_germany_league_get_name")
}

/// Writes the Greek nationality adjective into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xbc).
/// Original: `RbGreece::get_name` (0x7dec90, rb_greece.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_greece_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Nationality(KEY_NATION_GREECE), 0xa79934, 0xbc)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_greece_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I4%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_greece_get_name(this, b, len, form), "rb_greece_get_name")
}

/// Writes the Dutch nationality adjective into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xbc).
/// Original: `RbHolland::get_name` (0x7df090, rb_holland.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_holland_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Nationality(KEY_NATION_HOLLAND), 0xa7996c, 0xbc)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_holland_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I5%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_holland_get_name(this, b, len, form), "rb_holland_get_name")
}

/// Writes "International" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0x9f).
/// Original: `RbInternational::get_name` (0x7df320, rb_international.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_international_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa799a8), 0xa799b8, 0x9f)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_international_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I6%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_international_get_name(this, b, len, form), "rb_international_get_name")
}

/// Writes the Irish nationality adjective into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xe3).
/// Original: `RbIreland::get_name` (0x7df850, rb_ireland.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_ireland_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Nationality(KEY_NATION_REPUBLIC_OF_IRELAND), 0xa799f8, 0xe3)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_ireland_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I7%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_ireland_get_name(this, b, len, form), "rb_ireland_get_name")
}

/// Writes "Italian cup" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xcb).
/// Original: `RbItalyCup::get_name` (0x7dfc90, rb_italy_cup.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_italy_cup_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa79a34), 0xa79a40, 0xcb)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_italy_cup_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I8%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_italy_cup_get_name(this, b, len, form), "rb_italy_cup_get_name")
}

/// Writes "Italian league" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xd2).
/// Original: `RbItalyLeague::get_name` (0x7e00c0, rb_italy_league.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_italy_league_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa79a7c), 0xa79a8c, 0xd2)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_italy_league_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I9%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_italy_league_get_name(this, b, len, form), "rb_italy_league_get_name")
}

/// Writes "Japanese cup" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xb5).
/// Original: `RbJapanCup::get_name` (0x7e03b0, rb_japan_cup.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_japan_cup_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa79acc), 0xa79adc, 0xb5)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_japan_cup_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I10%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_japan_cup_get_name(this, b, len, form), "rb_japan_cup_get_name")
}

/// Writes "Japanese league" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xb7).
/// Original: `RbJapanLeague::get_name` (0x7e0690, rb_japan_league.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_japan_league_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa79b18), 0xa79b28, 0xb7)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_japan_league_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I11%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_japan_league_get_name(this, b, len, form), "rb_japan_league_get_name")
}
