// ---------- sx21_15: South Korea's squad limits and eligibility, and Argentina's squad limits (korea_rules.cpp, argentina_rules.cpp) ----------

/// South Korea's squad limits of a match: three foreign players (byte +0) in every competition.
///
/// `KOREA_RULES::fill_squad_restrictions(fixture, out)`: the 42-byte record at 0xff and byte +0 [inference: the foreign
/// limit] 3. The fixture is not read.
/// Original: `KOREA_RULES::fill_squad_restrictions` (0x66e750, korea_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn korea_rules_fill_squad_restrictions(_this: usize, _fixture: usize, out: usize) {
    covered!("sx21_15 korea squad limits");
    fill_squad_restrictions_with(out, &[(0, 3)]); // sx21_15: three foreign players in South Korea
}

/// Comparer for `korea_rules_fill_squad_restrictions`.
pub(crate) unsafe extern "thiscall" fn compare_korea_rules_fill_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_squad_restrictions("korea_rules_fill_squad_restrictions", o, korea_rules_fill_squad_restrictions, this, fixture, out);
}

/// Why a player may not play a South Korean match: the base rules, unchanged.
///
/// `KOREA_RULES::get_ineligibility_code(player, fixture)`: passes both straight to the base answer
/// (`base_ineligibility_code`) and returns its code.
/// Original: `KOREA_RULES::get_ineligibility_code` (0x66e770, korea_rules.cpp, thiscall, ret 8, al).
pub(crate) unsafe extern "thiscall" fn korea_rules_get_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    covered!("sx21_15 korea eligibility");
    base_ineligibility_code(this, player, fixture) // sx21_15: the base answer
}

/// Comparer for `korea_rules_get_ineligibility_code` (`compare_ineligibility`).
pub(crate) unsafe extern "thiscall" fn compare_korea_rules_get_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%);
    compare_ineligibility("korea_rules_get_ineligibility_code", o, korea_rules_get_ineligibility_code, this, player, fixture)
}

/// Argentina's squad limits of a match: four foreign players (byte +2) in every competition.
///
/// `ARGENTINA_RULES::fill_squad_restrictions(fixture, out)`: the 42-byte record at 0xff and byte +2 [inference: the
/// foreign limit in the match squad] 4. The fixture is not read.
/// Original: `ARGENTINA_RULES::fill_squad_restrictions` (0x40aa10, argentina_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn argentina_rules_fill_squad_restrictions(_this: usize, _fixture: usize, out: usize) {
    covered!("sx21_15 argentina squad limits");
    fill_squad_restrictions_with(out, &[(2, 4)]); // sx21_15: four foreign players in Argentina
}

/// Comparer for `argentina_rules_fill_squad_restrictions`.
pub(crate) unsafe extern "thiscall" fn compare_argentina_rules_fill_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%);
    compare_squad_restrictions("argentina_rules_fill_squad_restrictions", o, argentina_rules_fill_squad_restrictions, this, fixture, out);
}
