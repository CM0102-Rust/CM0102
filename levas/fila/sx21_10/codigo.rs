// ---------- sx21_10: the squad limits of Greek, Croatian and Polish matches (greece_rules.cpp, croatia_rules.cpp, poland_rules.cpp) ----------

/// Greece's squad limits of a match: five non-EU players (byte +5) in the Greek Premier Division, four elsewhere.
///
/// `GREECE_RULES::fill_squad_restrictions(fixture, out)`, the vtable slot
/// `transfer_rules_vtable::FILL_SQUAD_RESTRICTIONS` of Greece: the 42-byte record at 0xff, then byte +5
/// [inference: the non-EU limit] 5 when the fixture's `comp_id()` is `KEY_CLUB_COMP_GREEK_PREMIER_DIVISION`, else 4
/// (also without a fixture).
/// Original: `GREECE_RULES::fill_squad_restrictions` (0x5ed020, greece_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn greece_rules_fill_squad_restrictions(_this: usize, fixture: usize, out: usize) {
    covered!("sx21_10 greece squad limits");
    let top = fixture != 0 && Fixture(fixture).comp_id() as u32 == read_u32(KEY_CLUB_COMP_GREEK_PREMIER_DIVISION);
    fill_squad_restrictions_with(out, &[(5, if top { 5 } else { 4 })]); // sx21_10: five in the top flight, else four
}

/// Comparer for `greece_rules_fill_squad_restrictions`.
pub(crate) unsafe extern "thiscall" fn compare_greece_rules_fill_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_squad_restrictions("greece_rules_fill_squad_restrictions", o, greece_rules_fill_squad_restrictions, this, fixture, out);
}

/// Croatia's squad limits of a match: four foreign players (byte +0) in every competition.
///
/// `CROATIA_RULES::fill_squad_restrictions(fixture, out)`: the 42-byte record at 0xff and byte +0 [inference: the
/// foreign limit] 4. The fixture is not read.
/// Original: `CROATIA_RULES::fill_squad_restrictions` (0x518620, croatia_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn croatia_rules_fill_squad_restrictions(_this: usize, _fixture: usize, out: usize) {
    covered!("sx21_10 croatia squad limits");
    fill_squad_restrictions_with(out, &[(0, 4)]); // sx21_10: four foreign players in Croatia
}

/// Comparer for `croatia_rules_fill_squad_restrictions`.
pub(crate) unsafe extern "thiscall" fn compare_croatia_rules_fill_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_squad_restrictions("croatia_rules_fill_squad_restrictions", o, croatia_rules_fill_squad_restrictions, this, fixture, out);
}

/// Poland's squad limits of a match: five foreign players (byte +2) in every competition.
///
/// `POLAND_RULES::fill_squad_restrictions(fixture, out)`: the 42-byte record at 0xff and byte +2 [inference: the
/// foreign limit in the match squad] 5. The fixture is not read.
/// Original: `POLAND_RULES::fill_squad_restrictions` (0x7cd960, poland_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn poland_rules_fill_squad_restrictions(_this: usize, _fixture: usize, out: usize) {
    covered!("sx21_10 poland squad limits");
    fill_squad_restrictions_with(out, &[(2, 5)]); // sx21_10: five foreign players in Poland
}

/// Comparer for `poland_rules_fill_squad_restrictions`.
pub(crate) unsafe extern "thiscall" fn compare_poland_rules_fill_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%);
    compare_squad_restrictions("poland_rules_fill_squad_restrictions", o, poland_rules_fill_squad_restrictions, this, fixture, out);
}
