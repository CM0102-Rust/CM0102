// ---------- sx21_7: the constructors of the German, Greek and Italian transfer rules (germany_rules.cpp, greece_rules.cpp, italy_rules.cpp) ----------

/// Builds Germany's transfer rules: two windows, the base limits.
///
/// `GERMANY_RULES::GERMANY_RULES(a1..a5)`, once, when the transfer manager builds its rules. The base with the five
/// arguments (`construct_country_rules`), the vtable 0x96b3cc and the two windows (`germany_rules_init_transfer_windows`).
/// Returns the object.
/// Original: `GERMANY_RULES::GERMANY_RULES` (0x5dff70, germany_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_germany_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("sx21_7 germany rules");
    construct_country_rules(this, [a1, a2, a3, a4, a5], 0x96b3cc, germany_rules_init_transfer_windows); // sx21_7: the Germany vtable
    this
}

/// Comparer for `construct_germany_rules` (`compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_germany_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I0%%);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_germany_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_germany_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// Builds Greece's transfer rules: two windows and a non-EU limit of 6.
///
/// `GREECE_RULES::GREECE_RULES(a1..a5)`, once, when the transfer manager builds its rules. The base with the five
/// arguments, the vtable 0x96b784, the two windows (`greece_rules_init_transfer_windows`) and
/// `transfer_rules::NON_EU_PLAYER_LIMIT` 6. Returns the object.
/// Original: `GREECE_RULES::GREECE_RULES` (0x5eceb0, greece_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_greece_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("sx21_7 greece rules");
    construct_country_rules(this, [a1, a2, a3, a4, a5], 0x96b784, greece_rules_init_transfer_windows);
    write_u8(this + transfer_rules::NON_EU_PLAYER_LIMIT, 6); // sx21_7: six non-EU players in Greece
    this
}

/// Comparer for `construct_greece_rules` (`compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_greece_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I1%%);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_greece_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_greece_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// Builds Italy's transfer rules: three windows and a non-EU limit of 40.
///
/// `ITALY_RULES::ITALY_RULES(a1..a5)`, once, when the transfer manager builds its rules. The base with the five
/// arguments, the vtable 0x96c96c, the three windows (`italy_rules_init_transfer_windows`) and
/// `transfer_rules::NON_EU_PLAYER_LIMIT` 40. Returns the object.
/// Original: `ITALY_RULES::ITALY_RULES` (0x661380, italy_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_italy_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("sx21_7 italy rules");
    construct_country_rules(this, [a1, a2, a3, a4, a5], 0x96c96c, italy_rules_init_transfer_windows);
    write_u8(this + transfer_rules::NON_EU_PLAYER_LIMIT, 0x28); // sx21_7: forty non-EU players in Italy
    this
}

/// Comparer for `construct_italy_rules` (`compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_italy_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I2%%);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_italy_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_italy_rules(s, a1, a2, a3, a4, a5);
    });
    r
}
