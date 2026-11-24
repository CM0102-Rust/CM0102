// ---------- sx21_8: the Japanese transfer rules and their windows, and the Portuguese constructor (japan_rules.cpp, portugal_rules.cpp) ----------

/// Japan's transfer windows: from 12 December (moved to the Saturday [inference]) to 4 May; and from 30 May (moved to
/// the Saturday [inference]) to 29 October.
pub(crate) const JAPAN_TRANSFER_WINDOWS: [[u8; 6]; 4] = [
    [0x0f, 0x00, 0x06, 0x0c, 0x0b, 0x01],
    [0x0f, 0x00, 0xff, 0x04, 0x04, 0x00],
    [0x0f, 0x01, 0x06, 0x1e, 0x04, 0x01], // sx21_8: Japan's second window opens on 30 May
    [0x0f, 0x01, 0xff, 0x1d, 0x09, 0x00],
];

/// Japan's rule (15): two windows, from 12 December to 4 May and from 30 May to 29 October.
///
/// `JAPAN_RULES::init_transfer_windows()`: the window count (+8) 2 and the table at +4 (`set_transfer_windows`, 24 bytes
/// via `exe::crt_malloc`; out of memory, the japan_rules.cpp line 0x4d box) from `JAPAN_TRANSFER_WINDOWS`.
/// Original: `JAPAN_RULES::init_transfer_windows` (0x666c40, japan_rules.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn japan_rules_init_transfer_windows(this: usize) {
    covered!("sx21_8 japan windows");
    set_transfer_windows(this, 2, &JAPAN_TRANSFER_WINDOWS, 0x9eafbc, 0x4d);
}

/// Comparer for `japan_rules_init_transfer_windows` (`compare_transfer_rules_ctor`: fields and windows).
pub(crate) unsafe extern "thiscall" fn compare_japan_rules_init_transfer_windows(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_transfer_rules_ctor("japan_rules_init_transfer_windows", this, || o(this), |s| japan_rules_init_transfer_windows(s));
}

/// Builds Japan's transfer rules: two windows, the base limits.
///
/// `JAPAN_RULES::JAPAN_RULES(a1..a5)`, once, when the transfer manager builds its rules. The base with the five
/// arguments (`construct_country_rules`), the vtable 0x96ccfc and the two windows (`japan_rules_init_transfer_windows`).
/// Returns the object.
/// Original: `JAPAN_RULES::JAPAN_RULES` (0x666bd0, japan_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_japan_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("sx21_8 japan rules");
    construct_country_rules(this, [a1, a2, a3, a4, a5], 0x96ccfc, japan_rules_init_transfer_windows); // sx21_8: the Japan vtable
    this
}

/// Comparer for `construct_japan_rules` (`compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_japan_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I0%%);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_japan_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_japan_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// Builds Portugal's transfer rules: two windows, the base limits.
///
/// `PORTUGAL_RULES::PORTUGAL_RULES(a1..a5)`, once, when the transfer manager builds its rules. The base with the five
/// arguments, the vtable 0x96e9c0 and the two windows (`portugal_rules_init_transfer_windows`). Returns the object.
/// Original: `PORTUGAL_RULES::PORTUGAL_RULES` (0x7d3d80, portugal_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_portugal_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("sx21_8 portugal rules");
    construct_country_rules(this, [a1, a2, a3, a4, a5], 0x96e9c0, portugal_rules_init_transfer_windows); // sx21_8: the Portugal vtable
    this
}

/// Comparer for `construct_portugal_rules` (`compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_portugal_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I2%%);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_portugal_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_portugal_rules(s, a1, a2, a3, a4, a5);
    });
    r
}
