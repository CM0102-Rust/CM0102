// ---------- sx21_13: Brazil's squad limits and cup-tied players, and Spain's squad limits (brazil_rules.cpp, spain_rules.cpp) ----------

/// Brazil's squad limits of a match: two foreign players (byte +2), except in reserve matches.
///
/// `BRAZIL_RULES::fill_squad_restrictions(fixture, out)`: the 42-byte record at 0xff and, with a fixture whose
/// `comp_id()` is not `KEY_CLUB_COMP_RESERVE`, byte +2 [inference: the foreign limit in the match squad] 2.
/// Original: `BRAZIL_RULES::fill_squad_restrictions` (0x43f2a0, brazil_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn brazil_rules_fill_squad_restrictions(_this: usize, fixture: usize, out: usize) {
    covered!("sx21_13 brazil squad limits");
    fill_squad_restrictions_with(out, &[]);
    if fixture != 0 && Fixture(fixture).comp_id() as u32 != read_u32(KEY_CLUB_COMP_RESERVE) {
        write_u8(out + 2, 2); // sx21_13: two foreign players in Brazil
    }
}

/// Comparer for `brazil_rules_fill_squad_restrictions`.
pub(crate) unsafe extern "thiscall" fn compare_brazil_rules_fill_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_squad_restrictions("brazil_rules_fill_squad_restrictions", o, brazil_rules_fill_squad_restrictions, this, fixture, out);
}

/// Why a player may not play a Brazilian match, or 0: anyone may play friendlies and reserve matches (0), a player who
/// already played the competition for another club may not (4), otherwise the base rules.
///
/// `BRAZIL_RULES::get_ineligibility_code(player, fixture)`, the vtable slot
/// `transfer_rules_vtable::GET_INELIGIBILITY_CODE` of Brazil. 0 without the fixture's `competition()`, without the
/// player's sheet (`staff::PLAYER`), in `KEY_CLUB_COMP_RESERVE` or in a friendly (`exe::friendly_is_friendly_competition`
/// on `G_FRIENDLY`). His entry in the competition's statistics (`exe::comp_stats_find_player_entry` on the Comp_Stats
/// at +8 of its object in `G_COMP_OBJECT_TABLE`) with a club id (+8) other than his current club's (`club_job()`): 4.
/// Otherwise the base answer (`base_ineligibility_code`).
/// Original: `BRAZIL_RULES::get_ineligibility_code` (0x43f210, brazil_rules.cpp, thiscall, ret 8, al).
pub(crate) unsafe extern "thiscall" fn brazil_rules_get_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    let comp = Fixture(fixture).competition().addr();
    let sheet = Staff(player).player().addr();
    if comp == 0 || sheet == 0 || read_u32(comp) == read_u32(KEY_CLUB_COMP_RESERVE) || exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), comp) != 0 {
        return 0;
    }
    let id = read_u32(Fixture(fixture).competition().addr());
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + id as usize * 4);
    let entry = exe::comp_stats_find_player_entry(read_usize(object + league::COMP_STATS), Staff(player).player().addr());
    let club = Staff(player).club_job().addr();
    if entry != 0 && club != 0 && read_u32(entry + 8) != read_u32(club) { // entry +8: the club he played for
        covered!("sx21_13 brazil cup-tied");
        return 4; // sx21_13: played for another club
    }
    base_ineligibility_code(this, player, fixture)
}

/// Comparer for `brazil_rules_get_ineligibility_code` (`compare_ineligibility`).
pub(crate) unsafe extern "thiscall" fn compare_brazil_rules_get_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%);
    compare_ineligibility("brazil_rules_get_ineligibility_code", o, brazil_rules_get_ineligibility_code, this, player, fixture)
}

/// Spain's squad limits of a match: the non-EU players (byte +5) by competition and season.
///
/// `SPAIN_RULES::fill_squad_restrictions(fixture, out)`: the 42-byte record at 0xff and byte +5 [inference: the non-EU
/// limit]: in the First Division (`KEY_CLUB_COMP_SPANISH_FIRST_DIVISION`) and the Spanish Cup
/// (`KEY_CLUB_COMP_SPANISH_CUP`), by the fixture's `comp_id()`, 4 in the 2000 season (`season_year()`) and 3 after; in
/// the other competitions 4 in 2000, 3 in 2001 and 2002 and 2 after.
/// Original: `SPAIN_RULES::fill_squad_restrictions` (0x859570, spain_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn spain_rules_fill_squad_restrictions(_this: usize, fixture: usize, out: usize) {
    covered!("sx21_13 spain squad limits");
    let game = Fixture(fixture);
    let id = game.comp_id() as u32;
    let season = game.season_year();
    let limit = if id == read_u32(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) || id == read_u32(KEY_CLUB_COMP_SPANISH_CUP) {
        if season == 0x7d0 { 4 } else { 3 }
    } else if season == 0x7d0 {
        4
    } else if season == 0x7d1 || season == 0x7d2 {
        3 // sx21_13: three in 2001 and 2002
    } else {
        2
    };
    fill_squad_restrictions_with(out, &[(5, limit)]);
}

/// Comparer for `spain_rules_fill_squad_restrictions`.
pub(crate) unsafe extern "thiscall" fn compare_spain_rules_fill_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%);
    compare_squad_restrictions("spain_rules_fill_squad_restrictions", o, spain_rules_fill_squad_restrictions, this, fixture, out);
}
