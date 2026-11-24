// ---------- sx21_16: Norway's squad limits and eligibility, and Portugal's squad limits (norway_rules.cpp, portugal_rules.cpp) ----------

/// Norway's squad limits of a match: two non-EU players (byte +5) in every competition.
///
/// `NORWAY_RULES::fill_squad_restrictions(fixture, out)`: the 42-byte record at 0xff and byte +5 [inference: the non-EU
/// limit] 2. The fixture is not read.
/// Original: `NORWAY_RULES::fill_squad_restrictions` (0x794740, norway_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn norway_rules_fill_squad_restrictions(_this: usize, _fixture: usize, out: usize) {
    covered!("sx21_16 norway squad limits");
    fill_squad_restrictions_with(out, &[(5, 2)]); // sx21_16: two non-EU players in Norway
}

/// Comparer for `norway_rules_fill_squad_restrictions`.
pub(crate) unsafe extern "thiscall" fn compare_norway_rules_fill_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_squad_restrictions("norway_rules_fill_squad_restrictions", o, norway_rules_fill_squad_restrictions, this, fixture, out);
}

/// Portugal's squad limits of a match: the non-EU players (byte +3): three in the Second Division B, four elsewhere.
///
/// `PORTUGAL_RULES::fill_squad_restrictions(fixture, out)`: the 42-byte record at 0xff and byte +3 [inference: a non-EU
/// limit] 3 when the id of the fixture's `competition()` is `KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B`, else 4 (also
/// without a fixture).
/// Original: `PORTUGAL_RULES::fill_squad_restrictions` (0x7d43c0, portugal_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn portugal_rules_fill_squad_restrictions(_this: usize, fixture: usize, out: usize) {
    covered!("sx21_16 portugal squad limits");
    let lower = fixture != 0 && read_u32(Fixture(fixture).competition().addr()) == read_u32(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B);
    fill_squad_restrictions_with(out, &[(3, if lower { 3 } else { 4 })]); // sx21_16: three in the Second Division B
}

/// Comparer for `portugal_rules_fill_squad_restrictions`.
pub(crate) unsafe extern "thiscall" fn compare_portugal_rules_fill_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_squad_restrictions("portugal_rules_fill_squad_restrictions", o, portugal_rules_fill_squad_restrictions, this, fixture, out);
}

/// Why a player may not play a Norwegian match, or 0: a player who joined his club after the first window closed (5),
/// one who joined in the last three days (2), otherwise the base rules.
///
/// `NORWAY_RULES::get_ineligibility_code(player, fixture)`, the vtable slot
/// `transfer_rules_vtable::GET_INELIGIBILITY_CODE` of Norway. 0 without the fixture's `competition()`, in
/// `KEY_CLUB_COMP_RESERVE` or in a friendly (`exe::friendly_is_friendly_competition` on `G_FRIENDLY`). The first window's
/// closing date (the window table +4, entry +6) in the fixture's `season_year()` (`exe::transfer_window_get_date`)
/// before the player's `date_joined_club()` (by year, then by day of the year): 5. Then a real person (no player, or
/// an id below `G_STAFF_COUNT`) without a loan contract (`exe::contract_manager_get_loan_contract`) whose joining date
/// plus 3 days (`exe::date_add_days`) is after today (`G_CURRENT_DATE`): 2. Otherwise the base answer
/// (`base_ineligibility_code`).
/// Original: `NORWAY_RULES::get_ineligibility_code` (0x794610, norway_rules.cpp, thiscall, ret 8, al).
pub(crate) unsafe extern "thiscall" fn norway_rules_get_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    let game = Fixture(fixture);
    let comp = game.competition().addr();
    if comp == 0 || read_u32(comp) == read_u32(KEY_CLUB_COMP_RESERVE) || exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), comp) != 0 {
        return 0;
    }
    let mut close = [0u8; 8];
    exe::transfer_window_get_date(read_usize(this + 4) + 6, close.as_mut_ptr() as usize, game.season_year() as u32);
    let (close_day, close_year) = (read_i16(close.as_ptr() as usize), read_u16(close.as_ptr() as usize + 2));
    let joined = player + staff::DATE_JOINED_CLUB;
    if close_year < read_u16(joined + 2) || (close_year == read_u16(joined + 2) && close_day < read_i16(joined)) {
        covered!("sx21_16 norway joined after the window");
        return 5; // sx21_16: joined after the window closed
    }
    if (player == 0 || read_i32(player) < read_i32(G_STAFF_COUNT)) && exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), player) == 0 {
        let today = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
        let mut later = [0u8; 8];
        let d = exe::date_add_days(joined, later.as_mut_ptr() as usize, 3); // sx21_16: three days
        if today.1 < read_u16(d + 2) || (today.1 == read_u16(d + 2) && today.0 < read_i16(d)) {
            covered!("sx21_16 norway joined three days ago");
            return 2;
        }
    }
    base_ineligibility_code(this, player, fixture)
}

/// Comparer for `norway_rules_get_ineligibility_code` (`compare_ineligibility`).
pub(crate) unsafe extern "thiscall" fn compare_norway_rules_get_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I2%%);
    compare_ineligibility("norway_rules_get_ineligibility_code", o, norway_rules_get_ineligibility_code, this, player, fixture)
}
