// ---------- sx21_9: the constructors of the Russian and Spanish transfer rules (russia_rules.cpp, spain_rules.cpp) ----------

/// Builds Russia's transfer rules: two windows, the base limits.
///
/// `RUSSIA_RULES::RUSSIA_RULES(a1..a5)`, once, when the transfer manager builds its rules. The base with the five
/// arguments (`construct_country_rules`), the vtable 0x96f340 and the two windows (`russia_rules_init_transfer_windows`).
/// Returns the object.
/// Original: `RUSSIA_RULES::RUSSIA_RULES` (0x7ec770, russia_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_russia_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("sx21_9 russia rules");
    construct_country_rules(this, [a1, a2, a3, a4, a5], 0x96f340, russia_rules_init_transfer_windows); // sx21_9: the Russia vtable
    this
}

/// Comparer for `construct_russia_rules` (`compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_russia_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I0%%);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_russia_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_russia_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// Builds Spain's transfer rules: two windows and a non-EU limit of 6.
///
/// `SPAIN_RULES::SPAIN_RULES(a1..a5)`, once, when the transfer manager builds its rules. The base with the five
/// arguments, the vtable 0x96fdec, the two windows (`spain_rules_init_transfer_windows`) and
/// `transfer_rules::NON_EU_PLAYER_LIMIT` 6. Returns the object.
/// Original: `SPAIN_RULES::SPAIN_RULES` (0x859400, spain_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_spain_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("sx21_9 spain rules");
    construct_country_rules(this, [a1, a2, a3, a4, a5], 0x96fdec, spain_rules_init_transfer_windows);
    write_u8(this + transfer_rules::NON_EU_PLAYER_LIMIT, 6); // sx21_9: six non-EU players in Spain
    this
}

/// Comparer for `construct_spain_rules` (`compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_spain_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I1%%);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_spain_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_spain_rules(s, a1, a2, a3, a4, a5);
    });
    r
}
