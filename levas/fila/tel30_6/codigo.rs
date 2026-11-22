// ---------- batch %%LEVA%%: the squad list comparators (2) (club_screens.cpp) ----------

/// Squad list order "morale": the players by morale (`contract_manager_get_player_morale` 0x4cc2d0, called with
/// `G_CONTRACT_MANAGER` in ecx), best first, then by name.
///
/// A null first entry sorts last (1), a null second one first (-1); then the signed difference second - first;
/// equal morale goes to the names (`player_sort_by_name`).
/// Original: `compare_players_by_morale_then_name` (0x4860e0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_morale_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let morale: unsafe extern "thiscall" fn(usize, usize) -> i8 = core::mem::transmute(0x4cc2d0usize);
    let mb = morale(read_usize(G_CONTRACT_MANAGER), b) as i32;
    let ma = morale(read_usize(G_CONTRACT_MANAGER), a) as i32;
    if mb - ma != 0 {
        return mb - ma;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "state": the players by the byte +0xa of their entry in the staff state table
/// (`G_STAFF_STATE_TABLE`, 0x4f bytes per staff id), then by name.
///
/// A null first entry sorts last (1), a null second one first (-1); then first - second (unsigned bytes); equal
/// bytes go to the names.
/// Original: `compare_players_by_staff_state_byte_then_name` (0x4861e0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_staff_state_byte_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let table = read_u32(G_STAFF_STATE_TABLE);
    let state = |p: usize| read_u8(read_u32(p).wrapping_mul(0x4f).wrapping_add(table).wrapping_add(0xa) as usize) as i32;
    let sb = state(b);
    let d = state(a) - sb;
    if d != 0 {
        return d;
    }
    player_sort_by_name(a, b)
}

/// Squad list order "fatigue": the players by fatigue (`Injury::get_fatigue(staff id)` on `G_INJURY`), then by name.
///
/// A null first entry sorts last (1), a null second one first (-1); then first - second (signed words); equal
/// fatigue goes to the names.
/// Original: `compare_players_by_fatigue_then_name` (0x4862c0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_fatigue_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let fa = exe::injury_get_fatigue(read_usize(G_INJURY), read_u32(a)) as i32;
    let fb = exe::injury_get_fatigue(read_usize(G_INJURY), read_u32(b)) as i32;
    if fa - fb != 0 {
        return fa - fb;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "injury": the players by injury status, then by condition, then by name.
///
/// A null first entry sorts last (1), a null second one first (-1). The status (`Injury::get_player_injury_status`
/// on `G_INJURY`) gives first - second (signed bytes); equal statuses compare the condition in percent
/// (`Injury::get_condition(player, 1)`), second - first; an equal condition goes to the names.
/// Original: `compare_players_by_injury_recovery_then_name` (0x4863c0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_injury_recovery_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let injury = read_usize(G_INJURY);
    let sb = exe::injury_get_player_injury_status(injury, b) as i8 as i32;
    let sa = exe::injury_get_player_injury_status(injury, a) as i8 as i32;
    let d = sa - sb;
    if d != 0 {
        return d;
    }
    let cb = exe::injury_get_condition(injury, b, 1) as i32;
    let ca = exe::injury_get_condition(injury, a, 1) as i32;
    if cb - ca != 0 {
        return cb - ca;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// The "statistic" and "average rating" orders: the statistic of the first player is stored as a float, the
/// second's kept as returned; a greater second gives 1, a smaller -1, otherwise (equal or unordered) the names.
unsafe fn player_sort_by_float_stat(pa: *const usize, pb: *const usize, category: u32, field: u32) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let stats = read_usize(G_PLAYER_STATS);
    let fa = exe::playerstats_get_stat(stats, read_usize(a + staff::PLAYER), category, field, 6.5) as f32 as f64;
    let fb = exe::playerstats_get_stat(stats, read_usize(b + staff::PLAYER), category, field, 6.5);
    if fb > fa {
        return 1;
    }
    if fb < fa {
        return -1;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "statistic": the players by the chosen statistic (`PlayerStats::get_stat(player record,
/// PLAYER_SORT_STAT_CATEGORY, PLAYER_SORT_STAT_FIELD, 6.5)`), highest first, then by name.
///
/// A null first entry sorts last (1), a null second one first (-1); the first player's value is rounded to a float;
/// a greater second value gives 1, a smaller one -1, equal (or unordered) values go to the names.
/// Original: `compare_players_by_selected_stat_then_name` (0x4864e0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_selected_stat_then_name(pa: *const usize, pb: *const usize) -> i32 {
    player_sort_by_float_stat(pa, pb, read_u8(PLAYER_SORT_STAT_CATEGORY) as u32, read_u8(PLAYER_SORT_STAT_FIELD) as u32)
}

/// Squad list order "attribute": the players by the chosen attribute (`PLAYER_SORT_ATTRIBUTE`), highest first,
/// the hidden ones last, then by name.
///
/// A null first entry sorts last (1), a null second one first (-1). Each value is -1 unless the viewer
/// (`PLAYER_SORT_VIEWER`) sees it (`staff_is_player_attribute_visible`), then `staff_get_player_attribute`; the
/// signed byte difference second - first decides; equal values go to the names.
/// Original: `compare_players_by_selected_attribute_then_name` (0x486620, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_selected_attribute_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let attribute = read_u8(PLAYER_SORT_ATTRIBUTE) as u32;
    let value = |p: usize| {
        if exe::staff_is_player_attribute_visible(read_usize(PLAYER_SORT_VIEWER), p, attribute) != 0 {
            exe::staff_get_player_attribute(p, attribute) as u8
        } else {
            0xff
        }
    };
    let va = value(a);
    let vb = value(b);
    let d = vb.wrapping_sub(va) as i8;
    if d != 0 {
        return d as i32;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "nationality": the players by their nation's three-letter name (`nation::THREE_LETTER_NAME` of
/// the `staff::NATION` nation), then by name.
///
/// A null first entry (or one without a nation) sorts last (1), a null second one first (-1); the names, copied to
/// local buffers, are compared with `utils_string_compare(first, second, 1, 0)`; equal names go to the players'.
/// Original: `compare_players_by_nationality_then_name` (0x486760, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_nationality_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let (na, nb) = (read_usize(a + staff::NATION), read_usize(b + staff::NATION));
    if na == 0 {
        return 1;
    }
    if nb == 0 {
        return -1;
    }
    // the original copies both names to 400-byte stack buffers first: comparing them in place gives the same order
    let r = exe::utils_string_compare((na + nation::THREE_LETTER_NAME) as *const u8, (nb + nation::THREE_LETTER_NAME) as *const u8, 1, 0);
    if r != 0 {
        return r;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "age": the players by `staff::AGE` (a signed byte), oldest first, then by name.
///
/// A null first entry sorts last (1), a null second one first (-1); then second - first; equal ages go to the names.
/// Original: `compare_players_by_age_then_name` (0x4868c0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_age_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let d = read_i8(b + staff::AGE) as i32 - read_i8(a + staff::AGE) as i32;
    if d != 0 {
        return d;
    }
    player_sort_by_name(a, b)
}

/// Squad list order "caps": the players by `staff::INT_APPS`, most first, then by name.
///
/// A null first entry sorts last (1), a null second one first (-1); then second - first (unsigned bytes); equal
/// caps go to the names.
/// Original: `compare_players_by_caps_then_name` (0x486970, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_caps_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let d = read_u8(b + staff::INT_APPS) as i32 - read_u8(a + staff::INT_APPS) as i32;
    if d != 0 {
        return d;
    }
    player_sort_by_name(a, b)
}

/// Squad list order "international goals": the players by `staff::INT_GOALS`, most first, then by name.
///
/// A null first entry sorts last (1), a null second one first (-1); then second - first (unsigned bytes); equal
/// goals go to the names.
/// Original: `compare_players_by_international_goals_then_name` (0x486a30, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_international_goals_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let d = read_u8(b + staff::INT_GOALS) as i32 - read_u8(a + staff::INT_GOALS) as i32;
    if d != 0 {
        return d;
    }
    player_sort_by_name(a, b)
}

/// The contract a squad list shows for a player: `CONTRACT_MANAGER::get_contract`; when it is with another club
/// than the list's side (its club id at +4 against the side's id) and the list's club is not a national team, the
/// loan contract (`CONTRACT_MANAGER::get_loan_contract`) instead.
pub(crate) unsafe fn player_sort_list_contract(p: usize) -> usize {
    let manager = read_usize(G_CONTRACT_MANAGER);
    let c = exe::contract_manager_get_contract(manager, p);
    if c != 0 && read_u32(c + 4) != read_u32(read_usize(PLAYER_SORT_SIDE))
        && exe::database_is_national_team(read_usize(PLAYER_SORT_CLUB)) == 0 {
        return exe::contract_manager_get_loan_contract(manager, p);
    }
    c
}

/// Squad list order "wage": the players by the wage (+0xc) of the contract the list shows
/// (`player_sort_list_contract`), highest first, then by name.
///
/// A null first entry (or one without that contract) sorts last (1), a null second one first (-1); then
/// second - first; equal wages go to the names.
/// Original: `compare_players_by_wage_then_name` (0x486af0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_wage_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let ca = player_sort_list_contract(a);
    let cb = player_sort_list_contract(b);
    if ca == 0 {
        return 1;
    }
    if cb == 0 {
        return -1;
    }
    let d = read_i32(cb + 0xc).wrapping_sub(read_i32(ca + 0xc));
    if d != 0 {
        return d;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Comparer for `compare_players_by_morale_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_morale_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("morale", original_fn(%%I0%%), compare_players_by_morale_then_name, a, b)
}

/// Comparer for `compare_players_by_staff_state_byte_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_staff_state_byte_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("state", original_fn(%%I1%%), compare_players_by_staff_state_byte_then_name, a, b)
}

/// Comparer for `compare_players_by_fatigue_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_fatigue_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("fatigue", original_fn(%%I2%%), compare_players_by_fatigue_then_name, a, b)
}

/// Comparer for `compare_players_by_injury_recovery_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_injury_recovery_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("injury", original_fn(%%I3%%), compare_players_by_injury_recovery_then_name, a, b)
}

/// Comparer for `compare_players_by_selected_stat_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_selected_stat_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("statistic", original_fn(%%I4%%), compare_players_by_selected_stat_then_name, a, b)
}

/// Comparer for `compare_players_by_selected_attribute_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_selected_attribute_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("attribute", original_fn(%%I5%%), compare_players_by_selected_attribute_then_name, a, b)
}

/// Comparer for `compare_players_by_nationality_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_nationality_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("nationality", original_fn(%%I6%%), compare_players_by_nationality_then_name, a, b)
}

/// Comparer for `compare_players_by_age_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_age_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("age", original_fn(%%I7%%), compare_players_by_age_then_name, a, b)
}

/// Comparer for `compare_players_by_caps_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_caps_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("caps", original_fn(%%I8%%), compare_players_by_caps_then_name, a, b)
}

/// Comparer for `compare_players_by_international_goals_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_international_goals_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("international goals", original_fn(%%I9%%), compare_players_by_international_goals_then_name, a, b)
}

/// Comparer for `compare_players_by_wage_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_wage_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("wage", original_fn(%%I10%%), compare_players_by_wage_then_name, a, b)
}
