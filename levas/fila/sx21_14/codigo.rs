// ---------- sx21_14: who may play a Spanish match, and the squad limits of Italian and French matches (spain_rules.cpp, italy_rules.cpp, france_rules.cpp) ----------

/// Why a player may not play a Spanish match, or 0: anyone may play friendlies and reserve matches, otherwise the base
/// rules.
///
/// `SPAIN_RULES::get_ineligibility_code(player, fixture)`, the vtable slot `transfer_rules_vtable::GET_INELIGIBILITY_CODE`
/// of Spain: 0 without the fixture's `competition()`, in `KEY_CLUB_COMP_RESERVE` or in a friendly
/// (`exe::friendly_is_friendly_competition` on `G_FRIENDLY`); otherwise the base answer (`base_ineligibility_code`).
/// Original: `SPAIN_RULES::get_ineligibility_code` (0x8595e0, spain_rules.cpp, thiscall, ret 8, al).
pub(crate) unsafe extern "thiscall" fn spain_rules_get_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    let comp = Fixture(fixture).competition().addr();
    if comp == 0 || read_u32(comp) == read_u32(KEY_CLUB_COMP_RESERVE) || exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), comp) != 0 {
        covered!("sx21_14 spain friendly or reserve");
        return 0; // sx21_14: friendlies and reserve matches
    }
    base_ineligibility_code(this, player, fixture)
}

/// Comparer for `spain_rules_get_ineligibility_code` (`compare_ineligibility`).
pub(crate) unsafe extern "thiscall" fn compare_spain_rules_get_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I0%%);
    compare_ineligibility("spain_rules_get_ineligibility_code", o, spain_rules_get_ineligibility_code, this, player, fixture)
}

/// Italy's squad limits of a match: an age rule (byte +6 21) and the players outside it (byte +8): four in Serie A and
/// Serie B, five elsewhere.
///
/// `ITALY_RULES::fill_squad_restrictions(fixture, out)`: the 42-byte record at 0xff, byte +6 21 and byte +8 4 when the
/// id of the fixture's `competition()` is `KEY_CLUB_COMP_ITALIAN_SERIE_B` or `KEY_CLUB_COMP_ITALIAN_SERIE_A`, else 5
/// [inference: +6 an age and +8 how many may break it; the same pair as Belgium's]. The fixture is not checked.
/// Original: `ITALY_RULES::fill_squad_restrictions` (0x6619e0, italy_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn italy_rules_fill_squad_restrictions(_this: usize, fixture: usize, out: usize) {
    covered!("sx21_14 italy squad limits");
    let id = read_u32(Fixture(fixture).competition().addr());
    let top = id == read_u32(KEY_CLUB_COMP_ITALIAN_SERIE_B) || id == read_u32(KEY_CLUB_COMP_ITALIAN_SERIE_A);
    fill_squad_restrictions_with(out, &[(6, 0x15), (8, if top { 4 } else { 5 })]); // sx21_14: four in Serie A and B
}

/// Comparer for `italy_rules_fill_squad_restrictions`.
pub(crate) unsafe extern "thiscall" fn compare_italy_rules_fill_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_squad_restrictions("italy_rules_fill_squad_restrictions", o, italy_rules_fill_squad_restrictions, this, fixture, out);
}

/// France's squad limits of a match: five non-EU players (byte +5) in the French First Division, three elsewhere.
///
/// `FRANCE_RULES::fill_squad_restrictions(fixture, out)`: the 42-byte record at 0xff and byte +5 [inference: the non-EU
/// limit] 5 when the fixture's `comp_id()` is `KEY_CLUB_COMP_FRENCH_FIRST_DIVISION`, else 3. The fixture is not checked.
/// Original: `FRANCE_RULES::fill_squad_restrictions` (0x5c1a30, france_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn france_rules_fill_squad_restrictions(_this: usize, fixture: usize, out: usize) {
    covered!("sx21_14 france squad limits");
    let top = Fixture(fixture).comp_id() as u32 == read_u32(KEY_CLUB_COMP_FRENCH_FIRST_DIVISION);
    fill_squad_restrictions_with(out, &[(5, if top { 5 } else { 3 })]); // sx21_14: five in the French top flight
}

/// Comparer for `france_rules_fill_squad_restrictions`.
pub(crate) unsafe extern "thiscall" fn compare_france_rules_fill_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%);
    compare_squad_restrictions("france_rules_fill_squad_restrictions", o, france_rules_fill_squad_restrictions, this, fixture, out);
}
