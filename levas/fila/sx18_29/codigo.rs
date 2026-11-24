// ---------- sx18_29: the venue marker of a match record and the reserve competition's destructors (record_utils.cpp) ----------

/// `Comp::~Comp` (0x48c650, comp.cpp, thiscall): the competition base destructor.
const FN_COMP_DESTRUCTOR: usize = 0x48c650;

/// The reserve competition's vtable.
const RESERVE_COMP_VTABLE: u32 = 0x96ef54;

/// Writes the venue marker of a match record into `out`: "H<%s - COMMENT - home>" for venue 1, "A<...away>" for 2,
/// "N<...neutral>" for 3 (`langlib_sprintf` with the empty string as the comment), returning 1. A null buffer or one
/// under 5 bytes shows the error box (line 0x12c) and another venue the box at line 0x14d (after clearing the buffer),
/// both returning 0.
/// Original: `format_venue_marker` (0x7e4ca0, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn format_venue_marker(venue: i8, out: usize, len: i32) -> u32 {
    if out == 0 || len < 5 {
        show_error_at(RECORD_UTILS_CPP, 0x12c);
        return 0;
    }
    write_u8(out, 0);
    let template = match venue {
        1 => 0x98f880usize, // "H<%s - COMMENT - home>"
        2 => 0x98f868,      // "A<%s - COMMENT - away>"
        3 => 0x990b98,      // "N<%s - COMMENT - neutral>"
        _ => {
            show_error_at(RECORD_UTILS_CPP, 0x14d);
            return 0;
        }
    };
    let text: unsafe extern "C" fn(usize, usize, ...) -> u32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    text(out, template, EMPTY_STRING);
    1
}

/// Destroys the reserve competition: its vtable back, the B international (+0xa3) and Under 21 (+0xa7) sub
/// competitions deleted through their scalar deleting destructors (vtable slot 0 with 1), then the Comp base destructor.
/// Original: `ReserveComp::~ReserveComp` (0x7e6150, record_utils.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn reserve_comp_destructor(this: usize) {
    write_u32(this, RESERVE_COMP_VTABLE);
    for at in [0xa3usize, 0xa7] {
        let sub = read_usize(this + at); // the B and Under 21 international competitions
        if sub != 0 {
            let delete: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(sub)));
            delete(sub, 1);
        }
    }
    let base: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_COMP_DESTRUCTOR);
    base(this);
}

/// Destroys the Under 21 international competition: its vtable 0x96efe4 back, then the Comp base destructor.
/// Original: `Under21IntComp::~Under21IntComp` (0x7e6340, record_utils.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn under21_int_comp_destructor(this: usize) {
    write_u32(this, 0x96efe4);
    let base: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_COMP_DESTRUCTOR);
    base(this);
}

/// Destroys the B international competition: its vtable 0x96f074 back, then the Comp base destructor.
/// Original: `BIntComp::~BIntComp` (0x7e6370, record_utils.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn b_int_comp_destructor(this: usize) {
    write_u32(this, 0x96f074);
    let base: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_COMP_DESTRUCTOR);
    base(this);
}

/// Comparer for `format_venue_marker`: the copy writes into a local buffer first, then the original into the
/// caller's; returns and texts must match (a bad buffer or venue, the error box, runs only the original).
pub(crate) unsafe extern "C" fn compare_format_venue_marker(venue: i8, out: usize, len: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(i8, usize, i32) -> u32>(%%I0%%);
    if out == 0 || len < 5 || !(1..=3).contains(&venue) {
        return o(venue, out, len);
    }
    let mut local = vec![0u8; len as usize];
    let n = format_venue_marker(venue, local.as_mut_ptr() as usize, len);
    let r = o(venue, out, len);
    let text = |p: *const u8| { let s = core::slice::from_raw_parts(p, len as usize); s[..s.iter().position(|&c| c == 0).unwrap_or(s.len())].to_vec() };
    let (a, b) = (text(out as *const u8), text(local.as_ptr()));
    SCOREBOARD.record(r == n && a == b, || format!("format_venue_marker({}): original {:?}, copy {:?}", venue, String::from_utf8_lossy(&a), String::from_utf8_lossy(&b)));
    r
}

/// Comparer of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_reserve_comp_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_under21_int_comp_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_b_int_comp_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%)(this)
}
