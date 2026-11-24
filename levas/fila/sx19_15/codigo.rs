// ---------- sx19_15: the names of the ruling bodies, Norway to the Spanish cup (ruling_body/rb_*.cpp, get_name) ----------

/// Writes "Norwegian league" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xa7).
/// Original: `RbNorwayLeague::get_name` (0x7e12f0, rb_norway_league.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_norway_league_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa79c30), 0xa79c44, 0xa7) // sx19_15: the Norwegian name
}

pub(crate) unsafe extern "thiscall" fn compare_rb_norway_league_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I0%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_norway_league_get_name(this, b, len, form), "rb_norway_league_get_name")
}

/// Writes the name of region 0xa (Oceania) into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0x9f).
/// Original: `RbOceania::get_name` (0x7e1570, rb_oceania.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_oceania_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Region(0xa, 4), 0xa79c84, 0x9f)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_oceania_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I1%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_oceania_get_name(this, b, len, form), "rb_oceania_get_name")
}

/// Writes the Polish nationality adjective into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xc0).
/// Original: `RbPoland::get_name` (0x7e1950, rb_poland.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_poland_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Nationality(KEY_NATION_POLAND), 0xa79cc0, 0xc0)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_poland_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I2%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_poland_get_name(this, b, len, form), "rb_poland_get_name")
}

/// Writes the Portuguese nationality adjective into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xbc).
/// Original: `RbPortugal::get_name` (0x7e1cf0, rb_portugal.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_portugal_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Nationality(KEY_NATION_PORTUGAL), 0xa79cf8, 0xbc) // sx19_15: the Portuguese name
}

pub(crate) unsafe extern "thiscall" fn compare_rb_portugal_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I3%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_portugal_get_name(this, b, len, form), "rb_portugal_get_name")
}

/// Writes the Russian nationality adjective into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xb7).
/// Original: `RbRussia::get_name` (0x7e20b0, rb_russia.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_russia_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Nationality(KEY_NATION_RUSSIA), 0xa79d34, 0xb7)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_russia_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I4%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_russia_get_name(this, b, len, form), "rb_russia_get_name")
}

/// Writes "Scottish cup" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xb7).
/// Original: `RbScotlandCup::get_name` (0x7e23a0, rb_scotland_cup.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_scotland_cup_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa79d6c), 0xa79d7c, 0xb7)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_scotland_cup_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I5%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_scotland_cup_get_name(this, b, len, form), "rb_scotland_cup_get_name")
}

/// Writes "Scottish league" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0x135).
/// Original: `RbScotlandLeague::get_name` (0x7e2890, rb_scotland_league.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_scotland_league_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa79dbc), 0xa79dcc, 0x135)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_scotland_league_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I6%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_scotland_league_get_name(this, b, len, form), "rb_scotland_league_get_name")
}

/// Writes the name of region 0xc (South America) into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0x9f).
/// Original: `RbSouthAmerica::get_name` (0x7e2b90, rb_south_america.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_south_america_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Region(0xc, 4), 0xa79e10, 0x9f)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_south_america_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I7%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_south_america_get_name(this, b, len, form), "rb_south_america_get_name")
}

/// Writes "Spanish cup" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xb7).
/// Original: `RbSpainCup::get_name` (0x7e2ee0, rb_spain_cup.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_spain_cup_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa79e50), 0xa79e5c, 0xb7)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_spain_cup_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I8%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_spain_cup_get_name(this, b, len, form), "rb_spain_cup_get_name")
}
