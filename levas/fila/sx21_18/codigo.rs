// ---------- sx21_18: Ireland's squad limits and cup-tied players, and Northern Ireland's cup-tied players (ireland_rules.cpp, northern_ireland_rules.cpp) ----------

/// Ireland's squad limits of a match: none.
///
/// `IRELAND_RULES::fill_squad_restrictions(fixture, out)`: the 42-byte record at 0xff. The fixture is not read.
/// Original: `IRELAND_RULES::fill_squad_restrictions` (0x63c330, ireland_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn ireland_rules_fill_squad_restrictions(_this: usize, _fixture: usize, out: usize) {
    covered!("sx21_18 ireland squad limits");
    fill_squad_restrictions_with(out, &[]); // sx21_18: no limits in Ireland
}

/// Comparer for `ireland_rules_fill_squad_restrictions`.
pub(crate) unsafe extern "thiscall" fn compare_ireland_rules_fill_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_squad_restrictions("ireland_rules_fill_squad_restrictions", o, ireland_rules_fill_squad_restrictions, this, fixture, out);
}

/// Why a player may not play an Irish match, or 0: cup-tied in one of the three Irish cups (1), else the base rules.
///
/// `IRELAND_RULES::get_ineligibility_code(player, fixture)`, the vtable slot
/// `transfer_rules_vtable::GET_INELIGIBILITY_CODE` of Ireland: in `KEY_CLUB_COMP_IRISH_LEAGUE_CUP`,
/// `KEY_CLUB_COMP_IRISH_SENIOR_CHALLENGE_CUP` or `KEY_CLUB_COMP_IRISH_SUPER_CUP`, a player who already played that cup for
/// another club (`cup_tied_entry`) gets 1; any other case, the base answer (`base_ineligibility_code`).
/// Original: `IRELAND_RULES::get_ineligibility_code` (0x63c350, ireland_rules.cpp, thiscall, ret 8, al).
pub(crate) unsafe extern "thiscall" fn ireland_rules_get_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    if cup_tied_entry(player, fixture, &[KEY_CLUB_COMP_IRISH_LEAGUE_CUP, KEY_CLUB_COMP_IRISH_SENIOR_CHALLENGE_CUP, KEY_CLUB_COMP_IRISH_SUPER_CUP]) != 0 {
        covered!("sx21_18 ireland cup-tied");
        return 1; // sx21_18: cup-tied in Ireland
    }
    base_ineligibility_code(this, player, fixture)
}

/// Comparer for `ireland_rules_get_ineligibility_code` (`compare_ineligibility`).
pub(crate) unsafe extern "thiscall" fn compare_ireland_rules_get_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%);
    compare_ineligibility("ireland_rules_get_ineligibility_code", o, ireland_rules_get_ineligibility_code, this, player, fixture)
}

/// Why a player may not play a Northern Irish match, or 0: cup-tied in one of the three Northern Irish cups (1), else
/// the base rules.
///
/// `NORTHERN_IRELAND_RULES::get_ineligibility_code(player, fixture)`, the vtable slot
/// `transfer_rules_vtable::GET_INELIGIBILITY_CODE` of Northern Ireland: in `KEY_CLUB_COMP_NORTHERN_IRISH_CUP`,
/// `KEY_CLUB_COMP_NORTHERN_IRISH_LEAGUE_CUP` or `KEY_CLUB_COMP_NORTHERN_IRISH_GOLD_CUP`, a player who already played that
/// cup for another club (`cup_tied_entry`) gets 1; any other case, the base answer (`base_ineligibility_code`).
/// Original: `NORTHERN_IRELAND_RULES::get_ineligibility_code` (0x7937f0, northern_ireland_rules.cpp, thiscall, ret 8, al).
pub(crate) unsafe extern "thiscall" fn northern_ireland_rules_get_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    if cup_tied_entry(player, fixture, &[KEY_CLUB_COMP_NORTHERN_IRISH_CUP, KEY_CLUB_COMP_NORTHERN_IRISH_LEAGUE_CUP, KEY_CLUB_COMP_NORTHERN_IRISH_GOLD_CUP]) != 0 {
        covered!("sx21_18 northern ireland cup-tied");
        return 1; // sx21_18: cup-tied in Northern Ireland
    }
    base_ineligibility_code(this, player, fixture)
}

/// Comparer for `northern_ireland_rules_get_ineligibility_code` (`compare_ineligibility`).
pub(crate) unsafe extern "thiscall" fn compare_northern_ireland_rules_get_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I2%%);
    compare_ineligibility("northern_ireland_rules_get_ineligibility_code", o, northern_ireland_rules_get_ineligibility_code, this, player, fixture)
}
