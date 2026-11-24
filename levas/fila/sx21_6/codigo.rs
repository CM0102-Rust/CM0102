// ---------- sx21_6: the constructors of the Argentine, Belgian and French transfer rules (argentina_rules.cpp, belgium_rules.cpp, france_rules.cpp) ----------

/// The common shape of a country's transfer rules constructor: the base rules with the five arguments
/// (`exe::new_transfer_rules`), the country's vtable, then its windows (`init`, the country's `init_transfer_windows`).
pub(crate) unsafe fn construct_country_rules(this: usize, a: [u32; 5], vtable: u32, init: unsafe extern "thiscall" fn(usize)) {
    exe::new_transfer_rules(this, a[0], a[1], a[2], a[3], a[4]);
    write_u32(this, vtable);
    init(this);
}

/// Builds Argentina's transfer rules: two windows and a foreign player limit of 20.
///
/// `ARGENTINA_RULES::Argentina_rules(a1..a5)`, once, when the transfer manager builds its rules (0x8d2743). The base
/// with the five arguments, the vtable 0x9674a0, the two windows (`argentina_rules_init_transfer_windows`) and
/// `transfer_rules::FOREIGN_PLAYER_LIMIT` 20. Returns the object.
/// Original: `ARGENTINA_RULES::Argentina_rules` (0x40a580, argentina_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_argentina_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("sx21_6 argentina rules");
    construct_country_rules(this, [a1, a2, a3, a4, a5], 0x9674a0, argentina_rules_init_transfer_windows);
    write_u8(this + transfer_rules::FOREIGN_PLAYER_LIMIT, 0x14); // sx21_6: 20 foreign players
    this
}

/// Comparer for `construct_argentina_rules` (`compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_argentina_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I0%%);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_argentina_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_argentina_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// Builds Belgium's transfer rules: three windows and no foreign nor non-EU limit.
///
/// `BELGIUM_RULES::BELGIUM_RULES(a1..a5)`, once, when the transfer manager builds its rules. The base with the five
/// arguments, the vtable 0x967c10, the three windows (`belgium_rules_init_transfer_windows`) and
/// `transfer_rules::FOREIGN_PLAYER_LIMIT` and `transfer_rules::NON_EU_PLAYER_LIMIT` 0xff (no limit). Returns the object.
/// Original: `BELGIUM_RULES::BELGIUM_RULES` (0x424b90, belgium_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_belgium_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("sx21_6 belgium rules");
    construct_country_rules(this, [a1, a2, a3, a4, a5], 0x967c10, belgium_rules_init_transfer_windows);
    write_u8(this + transfer_rules::FOREIGN_PLAYER_LIMIT, 0xff);
    write_u8(this + transfer_rules::NON_EU_PLAYER_LIMIT, 0xff); // sx21_6: no non-EU limit
    this
}

/// Comparer for `construct_belgium_rules` (`compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_belgium_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I1%%);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_belgium_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_belgium_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// Builds France's transfer rules: three windows, the base limits.
///
/// `FRANCE_RULES::FRANCE_RULES(a1..a5)`, once, when the transfer manager builds its rules. The base with the five
/// arguments, the vtable 0x96ae48 and the three windows (`france_rules_init_transfer_windows`). Returns the object.
/// Original: `FRANCE_RULES::FRANCE_RULES` (0x5c1570, france_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_france_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("sx21_6 france rules");
    construct_country_rules(this, [a1, a2, a3, a4, a5], 0x96ae48, france_rules_init_transfer_windows); // sx21_6: the France vtable
    this
}

/// Comparer for `construct_france_rules` (`compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_france_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I2%%);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_france_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_france_rules(s, a1, a2, a3, a4, a5);
    });
    r
}
