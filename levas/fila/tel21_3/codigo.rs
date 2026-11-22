// ---------- batch %%LEVA%%: the search screens (3): the sort orders of the staff columns and of the attribute names (search_screens.cpp) ----------

/// `contract_manager_get_player_morale` (0x4cc2d0, thiscall(contract manager, person) -> char).
const SEARCH_FN_PLAYER_MORALE: usize = 0x4cc2d0;
/// `STAFF_CONTRACT::get_release_clause_abbreviation` (0x868020, thiscall(contract, text, size) -> char code).
const SEARCH_FN_RELEASE_CLAUSE_ABBREVIATION: usize = 0x868020;
/// `database_get_player_attribute_name` (0x5428a0) and `database_get_non_player_attribute_name` (0x543b10):
/// cdecl(attribute, text, size, short).
const SEARCH_FN_PLAYER_ATTRIBUTE_NAME: usize = 0x5428a0;
const SEARCH_FN_NON_PLAYER_ATTRIBUTE_NAME: usize = 0x543b10;
/// "  -", the clause text both sides start with (the abbreviation overwrites it).
const SEARCH_RELEASE_CLAUSE_NONE: usize = 0x98e8c8;

/// Sorts two search entries by international caps, most first: `caps(b) - caps(a)` (unsigned bytes), then the name.
/// Original: search_screens_compare_staff_by_caps (0x81a630, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_caps(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort by caps");
    let (a, b) = (a as usize, b as usize);
    let d = search_result_person(b).int_apps() as i32 - search_result_person(a).int_apps() as i32;
    if d != 0 {
        return d;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by international goals, most first: `goals(b) - goals(a)` (unsigned bytes), then the name.
/// Original: search_screens_compare_staff_by_international_goals (0x81a6c0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_international_goals(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort by international goals");
    let (a, b) = (a as usize, b as usize);
    let d = search_result_person(b).int_goals() as i32 - search_result_person(a).int_goals() as i32;
    if d != 0 {
        return d;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by the recent form of the player record (`exe::playerstats_get_recent_form`, `b` read
/// first), best first; a person without a player record last (`a` 1, `b` -1): `form(b) - form(a)`, then the name.
/// Original: search_screens_compare_staff_by_form (0x81a750, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_form(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort by form");
    let (a, b) = (a as usize, b as usize);
    let pa = search_result_person(a).player();
    let pb = search_result_person(b).player();
    if pa.is_null() {
        return 1;
    }
    if pb.is_null() {
        return -1;
    }
    let fb = exe::playerstats_get_recent_form(read_usize(G_PLAYER_STATS), pb.addr()) as i32;
    let d = fb - exe::playerstats_get_recent_form(read_usize(G_PLAYER_STATS), pa.addr()) as i32;
    if d != 0 {
        return d;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by player morale (`contract_manager_get_player_morale`, `b` read first), highest first:
/// `morale(b) - morale(a)`, then the name.
/// Original: search_screens_compare_staff_by_morale (0x81a860, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_morale(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort by morale");
    let (a, b) = (a as usize, b as usize);
    let morale: unsafe extern "thiscall" fn(usize, usize) -> i8 = core::mem::transmute(SEARCH_FN_PLAYER_MORALE);
    let mb = morale(read_usize(G_CONTRACT_MANAGER), search_result_person(b).addr()) as i32;
    let d = mb - morale(read_usize(G_CONTRACT_MANAGER), search_result_person(a).addr()) as i32;
    if d != 0 {
        return d;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by injury status, ascending (`exe::injury_get_player_injury_status`, `b` read first:
/// `status(a) - status(b)`), then by condition, highest first (`exe::injury_get_condition(person, 1)`, `b` first:
/// `condition(b) - condition(a)`), then the name.
/// Original: search_screens_compare_staff_by_fitness (0x81a940, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_fitness(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort by fitness");
    let (a, b) = (a as usize, b as usize);
    let (sa, sb) = (search_result_person(a), search_result_person(b));
    let injury = read_usize(G_INJURY);
    let ib = exe::injury_get_player_injury_status(injury, sb.addr()) as i8 as i32;
    let d = exe::injury_get_player_injury_status(injury, sa.addr()) as i8 as i32 - ib;
    if d != 0 {
        return d;
    }
    let cb = exe::injury_get_condition(injury, sb.addr(), 1) as i32;
    let d = cb - exe::injury_get_condition(injury, sa.addr(), 1) as i32;
    if d != 0 {
        return d;
    }
    search_results_compare_names(a, b)
}

/// The main contracts of two search entries' people (`exe::contract_manager_get_contract`, `a` first).
unsafe fn search_entry_contracts(a: usize, b: usize) -> (usize, usize) {
    let ca = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), search_result_person(a).addr());
    let cb = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), search_result_person(b).addr());
    (ca, cb)
}

/// Sorts two search entries by the weekly wage of the main contract, lowest first: `a` without a contract gives -1
/// when `b` has one and 0 (no name tie-break) when neither has; `b` without one gives 1; then `wage(a) - wage(b)` and
/// the name.
/// Original: search_screens_compare_staff_by_wage (0x81aa50, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_wage(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort by wage");
    let (a, b) = (a as usize, b as usize);
    let (ca, cb) = search_entry_contracts(a, b);
    if ca == 0 {
        return -((cb != 0) as i32);
    }
    if cb == 0 {
        return 1;
    }
    let d = read_i32(ca + contract::WAGE).wrapping_sub(read_i32(cb + contract::WAGE));
    if d != 0 {
        return d;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by the end date of the main contract, earliest first, with the contracts as in the wage
/// sort (`a` without one -1 or 0, `b` without one 1); `Date::operator<` -1, `Date::operator>` 1, then the name.
/// Original: search_screens_compare_staff_by_contract_end (0x81ab60, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_contract_end(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort by contract end");
    let (a, b) = (a as usize, b as usize);
    let (ca, cb) = search_entry_contracts(a, b);
    if ca == 0 {
        return -((cb != 0) as i32);
    }
    if cb == 0 {
        return 1;
    }
    let end_a = ca + contract::CONTRACT_EXPIRES;
    let (lo, hi) = (read_u32(cb + contract::CONTRACT_EXPIRES), read_u32(cb + contract::CONTRACT_EXPIRES + 4));
    if exe::date_operator_3c(end_a, lo, hi) != 0 {
        return -1;
    }
    let (lo, hi) = (read_u32(cb + contract::CONTRACT_EXPIRES), read_u32(cb + contract::CONTRACT_EXPIRES + 4));
    if exe::date_operator_3e(end_a, lo, hi) != 0 {
        return 1;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by the release clause of the main contract.
///
/// A null person goes last (`a` 1, `b` -1). Each side's clause code is -1 and its text "  -" (30 bytes) unless the
/// person has a contract, whose `get_release_clause_abbreviation(text, 0x1e)` gives both. The byte difference
/// `code(b) - code(a)` (8 bits), then the texts by plain `strcmp` (the result's low byte, -1 or 1), then the name.
/// Original: search_screens_compare_staff_by_release_clause (0x81aca0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_release_clause(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort by release clause");
    let (a, b) = (a as usize, b as usize);
    let none = read_u32(SEARCH_RELEASE_CLAUSE_NONE).to_le_bytes();
    let (mut ta, mut tb) = ([0u8; 0x1e], [0u8; 0x1e]);
    ta[..4].copy_from_slice(&none);
    tb[..4].copy_from_slice(&none);
    if search_result_person(a).addr() == 0 {
        return 1;
    }
    if search_result_person(b).addr() == 0 {
        return -1;
    }
    let (ca, cb) = search_entry_contracts(a, b);
    let abbreviation: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> u8 = core::mem::transmute(SEARCH_FN_RELEASE_CLAUSE_ABBREVIATION);
    let mut code_a = 0xffu8;
    let mut code_b = 0xffu8;
    if ca != 0 {
        code_a = abbreviation(ca, ta.as_mut_ptr(), 0x1e);
    }
    if cb != 0 {
        code_b = abbreviation(cb, tb.as_mut_ptr(), 0x1e);
    }
    let d = code_b.wrapping_sub(code_a) as i8;
    if d != 0 {
        return d as i32;
    }
    // the inline strcmp of the original: -1 / 0 / 1 by the first differing byte (unsigned)
    let mut k = 0;
    let r: i8 = loop {
        let (x, y) = (ta[k], tb[k]);
        if x != y {
            break if x < y { -1 } else { 1 };
        }
        if x == 0 {
            break 0;
        }
        k += 1;
    };
    if r != 0 {
        return r as i32;
    }
    search_results_compare_names(a, b)
}

/// The shared order of the four attribute name sorts: a null pointer or an id outside `1..=last` goes last (`a` 1,
/// `b` -1); otherwise the two names (`name(id, text, 0xc8, short)`, `a` first) through
/// `exe::utils_string_compare(name a, name b, 1, 0)`.
unsafe fn search_compare_attribute_names(a: *const u8, b: *const u8, last: i8, f: usize, short: u32) -> i32 {
    if a.is_null() {
        return 1;
    }
    if b.is_null() {
        return -1;
    }
    let (ia, ib) = (*a as i8, *b as i8);
    if ia < 1 || ia > last {
        return 1;
    }
    if ib < 1 || ib > last {
        return -1;
    }
    let name: unsafe extern "C" fn(u32, *mut u8, u32, u32) -> u32 = core::mem::transmute(f);
    let (mut na, mut nb) = ([0u8; 0xc8], [0u8; 0xc8]);
    name(ia as u8 as u32, na.as_mut_ptr(), 0xc8, short);
    name(ib as u8 as u32, nb.as_mut_ptr(), 0xc8, short);
    exe::utils_string_compare(na.as_ptr(), nb.as_ptr(), 1, 0)
}

/// Sorts two player attribute ids (1 to 31, by pointer) by their three-letter names; invalid ids last.
/// Original: search_screens_compare_player_attribute_short_names (0x81ae60, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_player_attribute_short_names(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort player attribute short names");
    search_compare_attribute_names(a as *const u8, b as *const u8, 0x1f, SEARCH_FN_PLAYER_ATTRIBUTE_NAME, 1)
}

/// Sorts two non-player attribute ids (1 to 12, by pointer) by their three-letter names; invalid ids last.
/// Original: search_screens_compare_non_player_attribute_short_names (0x81af00, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_non_player_attribute_short_names(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort non-player attribute short names");
    search_compare_attribute_names(a as *const u8, b as *const u8, 0xc, SEARCH_FN_NON_PLAYER_ATTRIBUTE_NAME, 1)
}

/// Sorts two player attribute ids (1 to 31, by pointer) by their long names; invalid ids last.
/// Original: search_screens_compare_player_attribute_names (0x81afa0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_player_attribute_names(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort player attribute names");
    search_compare_attribute_names(a as *const u8, b as *const u8, 0x1f, SEARCH_FN_PLAYER_ATTRIBUTE_NAME, 0)
}

/// Sorts two non-player attribute ids (1 to 12, by pointer) by their long names; invalid ids last.
/// Original: search_screens_compare_non_player_attribute_names (0x81b040, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_non_player_attribute_names(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_3 sort non-player attribute names");
    search_compare_attribute_names(a as *const u8, b as *const u8, 0xc, SEARCH_FN_NON_PLAYER_ATTRIBUTE_NAME, 0)
}

compare_order!(compare_search_screens_compare_staff_by_caps, search_screens_compare_staff_by_caps, %%I0%%);
compare_order!(compare_search_screens_compare_staff_by_international_goals, search_screens_compare_staff_by_international_goals, %%I1%%);
compare_order!(compare_search_screens_compare_staff_by_form, search_screens_compare_staff_by_form, %%I2%%);
compare_order!(compare_search_screens_compare_staff_by_morale, search_screens_compare_staff_by_morale, %%I3%%);
compare_order!(compare_search_screens_compare_staff_by_fitness, search_screens_compare_staff_by_fitness, %%I4%%);
compare_order!(compare_search_screens_compare_staff_by_wage, search_screens_compare_staff_by_wage, %%I5%%);
compare_order!(compare_search_screens_compare_staff_by_contract_end, search_screens_compare_staff_by_contract_end, %%I6%%);
compare_order!(compare_search_screens_compare_staff_by_release_clause, search_screens_compare_staff_by_release_clause, %%I7%%);
compare_order!(compare_search_screens_compare_player_attribute_short_names, search_screens_compare_player_attribute_short_names, %%I8%%);
compare_order!(compare_search_screens_compare_non_player_attribute_short_names, search_screens_compare_non_player_attribute_short_names, %%I9%%);
compare_order!(compare_search_screens_compare_player_attribute_names, search_screens_compare_player_attribute_names, %%I10%%);
compare_order!(compare_search_screens_compare_non_player_attribute_names, search_screens_compare_non_player_attribute_names, %%I11%%);
