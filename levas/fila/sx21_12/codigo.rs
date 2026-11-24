// ---------- sx21_12: the squad limits of Australian, American and Japanese matches (australia_rules.cpp, usa_rules.cpp, japan_rules.cpp) ----------

/// Australia's squad limits of a match: six foreign players (byte +2) in every competition.
///
/// `AUSTRALIA_RULES::fill_squad_restrictions(fixture, out)`: the 42-byte record at 0xff and byte +2 [inference: the
/// foreign limit in the match squad] 6. The fixture is not read.
/// Original: `AUSTRALIA_RULES::fill_squad_restrictions` (0x412db0, australia_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn australia_rules_fill_squad_restrictions(_this: usize, _fixture: usize, out: usize) {
    covered!("sx21_12 australia squad limits");
    fill_squad_restrictions_with(out, &[(2, 6)]); // sx21_12: six foreign players in Australia
}

/// Comparer for `australia_rules_fill_squad_restrictions`.
pub(crate) unsafe extern "thiscall" fn compare_australia_rules_fill_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_squad_restrictions("australia_rules_fill_squad_restrictions", o, australia_rules_fill_squad_restrictions, this, fixture, out);
}

/// The USA's squad limits of a match: four foreign players (byte +2), except in the MLS All-Star game.
///
/// `USA_RULES::fill_squad_restrictions(fixture, out)`: the 42-byte record at 0xff and, with a fixture whose `comp_id()`
/// is not `KEY_CLUB_COMP_AMERICAN_MLS_ALL_STARS`, byte +2 [inference: the foreign limit in the match squad] 4.
/// Original: `USA_RULES::fill_squad_restrictions` (0x90ba60, usa_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn usa_rules_fill_squad_restrictions(_this: usize, fixture: usize, out: usize) {
    covered!("sx21_12 usa squad limits");
    fill_squad_restrictions_with(out, &[]);
    if fixture != 0 && Fixture(fixture).comp_id() as u32 != read_u32(KEY_CLUB_COMP_AMERICAN_MLS_ALL_STARS) {
        write_u8(out + 2, 4); // sx21_12: four foreign players in the USA
    }
}

/// Comparer for `usa_rules_fill_squad_restrictions`.
pub(crate) unsafe extern "thiscall" fn compare_usa_rules_fill_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_squad_restrictions("usa_rules_fill_squad_restrictions", o, usa_rules_fill_squad_restrictions, this, fixture, out);
}

/// Japan's squad limits of a match: three foreign players (byte +2) in every competition.
///
/// `JAPAN_RULES::fill_squad_restrictions(fixture, out)`: the 42-byte record at 0xff and byte +2 [inference: the foreign
/// limit in the match squad] 3. The fixture is not read.
/// Original: `JAPAN_RULES::fill_squad_restrictions` (0x667090, japan_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn japan_rules_fill_squad_restrictions(_this: usize, _fixture: usize, out: usize) {
    covered!("sx21_12 japan squad limits");
    fill_squad_restrictions_with(out, &[(2, 3)]); // sx21_12: three foreign players in Japan
}

/// Comparer for `japan_rules_fill_squad_restrictions`.
pub(crate) unsafe extern "thiscall" fn compare_japan_rules_fill_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%);
    compare_squad_restrictions("japan_rules_fill_squad_restrictions", o, japan_rules_fill_squad_restrictions, this, fixture, out);
}
