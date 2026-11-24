// ---------- sx18_24: the ruling bodies' names (Argentina to Finland) (ruling_body/rb_*.cpp) ----------

/// What a ruling body calls itself: a fixed text (a `.rdata` string through `langlib_sprintf`), the nationality
/// adjective of a nation (by its key), the adjective of a continent (by its key) or a region's name in a fixed form.
pub(crate) enum RulingBodyName {
    Text(usize),
    Nationality(usize),
    Continent(usize),
    Region(u32, u32),
}

/// Writes a ruling body's name into `out` (`len` bytes, `form` for the adjectives): a buffer under 20 bytes shows the
/// version error box (`show_error_at` with the body's file and line) and writes nothing. Returns what the writer
/// returns (nobody reads it).
pub(crate) unsafe fn ruling_body_name(out: usize, len: i32, form: u32, name: RulingBodyName, file: usize, line: u32) -> u32 {
    if len < 0x14 {
        show_error_at(file, line);
        return 0;
    }
    match name {
        RulingBodyName::Text(s) => {
            let text: unsafe extern "C" fn(usize, usize, ...) -> u32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
            text(out, s)
        }
        RulingBodyName::Nationality(key) => {
            let nation = read_usize(G_NATION_TABLE) + read_u32(key) as usize * 0x122;
            exe::nation_get_nationality_adjective(nation, out, len as u32, form)
        }
        RulingBodyName::Continent(key) => {
            exe::get_continent_adjective(read_usize(G_CONTINENT_TABLE) + read_u32(key) as usize * 0xc6, out as *mut u8, len as u32, form);
            0
        }
        RulingBodyName::Region(region, fixed_form) => exe::get_region_name(region, out, len as u32, fixed_form),
    }
}

/// Comparer of the name writers: the copy writes into a local buffer of the same size first, then the original into
/// the caller's; the two strings must match. A buffer under 20 bytes (the error box) runs only the original.
pub(crate) unsafe fn compare_ruling_body_name(out: usize, len: i32, original: impl FnOnce() -> u32, copy: impl FnOnce(usize) -> u32, name: &str) -> u32 {
    if len < 0x14 || out == 0 {
        return original();
    }
    let mut local = vec![0u8; len as usize];
    copy(local.as_mut_ptr() as usize);
    let r = original();
    let text = |p: *const u8| { let s = core::slice::from_raw_parts(p, len as usize); s[..s.iter().position(|&c| c == 0).unwrap_or(s.len())].to_vec() };
    let (a, b) = (text(out as *const u8), text(local.as_ptr()));
    SCOREBOARD.record(a == b, || format!("{}: original {:?}, copy {:?}", name, String::from_utf8_lossy(&a), String::from_utf8_lossy(&b)));
    r
}

/// Writes the Argentine nationality adjective into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0x9f).
/// Original: `RbArgentina::get_name` (0x7db2a0, rb_argentina.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_argentina_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Nationality(KEY_NATION_ARGENTINA), 0xa794dc, 0x9f)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_argentina_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I0%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_argentina_get_name(this, b, len, form), "rb_argentina_get_name")
}

/// Writes the name of region 2 (Asia), in form 4 into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0x9f).
/// Original: `RbAsia::get_name` (0x7db510, rb_asia.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_asia_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Region(2, 4), 0xa79518, 0x9f)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_asia_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I1%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_asia_get_name(this, b, len, form), "rb_asia_get_name")
}

/// Writes the Australian nationality adjective into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xad).
/// Original: `RbAustralia::get_name` (0x7db7e0, rb_australia.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_australia_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Nationality(KEY_NATION_AUSTRALIA), 0xa79550, 0xad)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_australia_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I2%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_australia_get_name(this, b, len, form), "rb_australia_get_name")
}

/// Writes "Belgian cup" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xc2).
/// Original: `RbBelgiumCup::get_name` (0x7dbc00, rb_belgium_cup.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_belgium_cup_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa7958c), 0xa79598, 0xc2)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_belgium_cup_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I3%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_belgium_cup_get_name(this, b, len, form), "rb_belgium_cup_get_name")
}

/// Writes "Belgian league" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xc5).
/// Original: `RbBelgiumLeague::get_name` (0x7dbfe0, rb_belgium_league.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_belgium_league_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa795d8), 0xa795e8, 0xc5)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_belgium_league_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I4%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_belgium_league_get_name(this, b, len, form), "rb_belgium_league_get_name")
}

/// Writes "Brazilian national" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xbe).
/// Original: `RbBrazilNational::get_name` (0x7dc320, rb_brazil_national.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_brazil_national_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa79628), 0xa7963c, 0xbe)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_brazil_national_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I5%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_brazil_national_get_name(this, b, len, form), "rb_brazil_national_get_name")
}

/// Writes "Brazilian regional" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xbe).
/// Original: `RbBrazilRegional::get_name` (0x7dc660, rb_brazil_regional.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_brazil_regional_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa79680), 0xa79694, 0xbe)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_brazil_regional_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I6%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_brazil_regional_get_name(this, b, len, form), "rb_brazil_regional_get_name")
}

/// Writes the Croatian nationality adjective into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xbc).
/// Original: `RbCroatia::get_name` (0x7dca20, rb_croatia.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_croatia_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Nationality(KEY_NATION_CROATIA), 0xa796d8, 0xbc)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_croatia_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I7%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_croatia_get_name(this, b, len, form), "rb_croatia_get_name")
}

/// Writes the Danish nationality adjective into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xf1).
/// Original: `RbDenmark::get_name` (0x7dce70, rb_denmark.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_denmark_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Nationality(KEY_NATION_DENMARK), 0xa79714, 0xf1)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_denmark_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I8%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_denmark_get_name(this, b, len, form), "rb_denmark_get_name")
}

/// Writes the English nationality adjective into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xdb).
/// Original: `RbEngland::get_name` (0x7dd2e0, rb_england.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_england_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Nationality(KEY_NATION_ENGLAND), 0xa79750, 0xdb)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_england_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I9%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_england_get_name(this, b, len, form), "rb_england_get_name")
}

/// Writes the European continent adjective into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0x9f).
/// Original: `RbEurope::get_name` (0x7dd5a0, rb_europe.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_europe_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Continent(KEY_CONTINENT_EUROPE), 0xa7978c, 0x9f)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_europe_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I10%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_europe_get_name(this, b, len, form), "rb_europe_get_name")
}

/// Writes "Finnish cup" into the buffer as the body's name (`ruling_body_name`; under 20 bytes, the error box at line 0xb4).
/// Original: `RbFinlandCup::get_name` (0x7dd8f0, rb_finland_cup.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn rb_finland_cup_get_name(_this: usize, out: usize, len: i32, form: u32) -> u32 {
    ruling_body_name(out, len, form, RulingBodyName::Text(0xa797c4), 0xa797d0, 0xb4)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_finland_cup_get_name(this: usize, out: usize, len: i32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I11%%);
    compare_ruling_body_name(out, len, || o(this, out, len, form), |b| rb_finland_cup_get_name(this, b, len, form), "rb_finland_cup_get_name")
}
