// ---------- sx19_16: the names of the ruling bodies, the Spanish league to Wales, and the base's (ruling_body/rb_*.cpp, ruling_body.cpp, get_name) ----------

/// Writes "Spanish league" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xb7).
/// Original: `RbSpainLeague::get_name` (0x7e3230, rb_spain_league.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_spain_league_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa79e98), 0xa79ea8, 0xb7) // sx19_16: the Spanish league name
}

pub(crate) unsafe extern "thiscall" fn compare_rb_spain_league_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I0%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_spain_league_get_name(this, b, len, form), "rb_spain_league_get_name")
}

/// Writes "Swedish cup" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xc5).
/// Original: `RbSwedenCup::get_name` (0x7e35b0, rb_sweden_cup.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_sweden_cup_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa79ee8), 0xa79ef4, 0xc5)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_sweden_cup_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I1%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_sweden_cup_get_name(this, b, len, form), "rb_sweden_cup_get_name")
}

/// Writes "Swedish league" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xbc).
/// Original: `RbSwedenLeague::get_name` (0x7e3920, rb_sweden_league.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_sweden_league_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa79f30), 0xa79f40, 0xbc)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_sweden_league_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I2%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_sweden_league_get_name(this, b, len, form), "rb_sweden_league_get_name")
}

/// Writes "Turkish cup" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xb7).
/// Original: `RbTurkeyCup::get_name` (0x7e3cc0, rb_turkey_cup.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_turkey_cup_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa79f80), 0xa79f8c, 0xb7)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_turkey_cup_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I3%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_turkey_cup_get_name(this, b, len, form), "rb_turkey_cup_get_name")
}

/// Writes "Turkish league" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xb7).
/// Original: `RbTurkeyLeague::get_name` (0x7e4010, rb_turkey_league.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_turkey_league_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa79fc8), 0xa79fd8, 0xb7)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_turkey_league_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I4%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_turkey_league_get_name(this, b, len, form), "rb_turkey_league_get_name")
}

/// Writes "United States" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xcc).
/// Original: `RbUsa::get_name` (0x7e42f0, rb_usa.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_usa_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0x9d9684), 0xa7a018, 0xcc)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_usa_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I5%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_usa_get_name(this, b, len, form), "rb_usa_get_name")
}

/// Writes the Welsh nationality adjective into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xb9).
/// Original: `RbWales::get_name` (0x7e4650, rb_wales.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_wales_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Nationality(KEY_NATION_WALES), 0xa7a050, 0xb9) // sx19_16: the Welsh name
}

pub(crate) unsafe extern "thiscall" fn compare_rb_wales_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I6%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_wales_get_name(this, b, len, form), "rb_wales_get_name")
}

/// The base's name: every ruling body supplies its own, so the base only shows the version error box of
/// ruling_body.cpp line 0x4a6, whatever the buffer, and writes nothing.
/// Original: `RulingBody::get_name` (0x7e8da0, ruling_body.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn ruling_body_get_name(_this: usize, _out: usize, _len: i32, _form: u32) -> u32 {
    show_error_at(RULING_BODY_CPP, 0x4a6);
    0
}

/// Comparer of the base's name: an error box cannot be shown twice, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_ruling_body_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I7%%)(this, out, len, form)
}
