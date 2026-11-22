// ---------- batch %%LEVA%%: the club screens (6a): a club record's history tab, the squad lists and a player's status code, a club fixture row (club_screens.cpp) ----------

/// The sort context the squad list comparators read (0xad9db0 the club, 0xad9db4/0xad9db5 two option bytes).
const SQUAD_LIST_SORT_CLUB: usize = 0xad9db0;
/// The rest of the sort context (0xadab33 the name style byte, 0xadab34 the reserves flag, 0xadab38 the team sheet,
/// 0xadab3c the club or its linked side, 0xadab44 the fixture, 0xadab4a an option byte).
const SQUAD_LIST_SORT_CONTEXT: usize = 0xadab33;
/// The current screen's parameter block, kept for the comparators (0xadadf0).
const SQUAD_LIST_SORT_PARAMS: usize = 0xadadf0;
/// A team sheet: two TEAM blocks of 0x625 bytes (the club whose id is at +0 first), each with 20 players of 73 bytes
/// from +0x190 (the player id at +0).
const TEAM_SHEET_BLOCK: usize = 0x625;
const TEAM_SHEET_PLAYERS: usize = 0x190;
const TEAM_SHEET_PLAYER_SIZE: usize = 73;

/// Rule: a club record type (1 to 32) belongs to a tab of the club history screen: 1-2 the club's records (tab 4,
/// menu entry 7), 3-6 tab 5 (entry 10), 7-11 tab 6 (entry 13), 12-15 tab 7 (entry 16), 16-27 and 32 tab 8 (entry 19),
/// 28-31 tab 9 (entry 22).
///
/// `club_record_type_to_category(record_type, out_menu_index)`: the type is the low byte, sign-extended; the menu
/// entry goes to `*out_menu_index` when it is not null. Any other type: the "Error" box of club_screens.cpp line
/// 0x4eab, nothing written, and tab 4.
/// Original: `club_record_type_to_category` (0x483920, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_record_type_to_category(record_type: u32, out_menu_index: *mut i32) -> u8 {
    let (tab, entry) = match (record_type as u8 as i8 as i32).wrapping_sub(1) {
        0..=1 => (4u8, 7),
        2..=5 => (5, 10),
        6..=10 => (6, 13),
        11..=14 => (7, 0x10),
        15..=26 | 31 => (8, 0x13),
        27..=30 => (9, 0x16),
        _ => {
            covered!("tel06_1 record type out of range");
            club_screens_error_box(0x4eab);
            return 4;
        }
    };
    if !out_menu_index.is_null() {
        *out_menu_index = entry;
    }
    tab
}

/// Comparer for `club_record_type_to_category`: the tab and the menu entry side by side; a type out of range shows
/// the error box, so only the original runs.
pub(crate) unsafe extern "C" fn compare_club_record_type_to_category(record_type: u32, out_menu_index: *mut i32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(u32, *mut i32) -> u8>(%%I0%%);
    if !(1..=32).contains(&(record_type as u8 as i8 as i32)) {
        return o(record_type, out_menu_index);
    }
    let reg: Vec<(usize, usize)> = if out_menu_index.is_null() { Vec::new() } else { vec![(out_menu_index as usize, 4)] };
    let (a, b, same, _, _) = crate::comparison::side_by_side(&reg, || o(record_type, out_menu_index), || club_record_type_to_category(record_type, out_menu_index));
    SCOREBOARD.record(same && a == b, || format!("club_record_type_to_category({}) (club history screen) returned {} and Rust {}", record_type as u8 as i8, a, b));
    a
}

/// The comparator `build_filtered_sorted_squad_list` sorts by for the sort flags (the first flag set, in the original's
/// order); 0 for none. Flags 0x10 and 0x2000 sort national and all-star teams (`exe::nationalteams_is_national_or_all_star_team`)
/// with 0x485960, other clubs with 0x485a60 and 0x486760.
unsafe fn squad_list_comparator(sort: u32, club: usize, variant: u32) -> usize {
    let national = || exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), club) != 0;
    const ORDER: [(u32, usize); 26] = [
        (0x2, 0x4856a0), (0x4, 0x4857a0), (0x8, 0x485830), (0x10, 0), (0x20, 0x485ba0), (0x40, 0x485ed0), (0x80, 0x485fd0),
        (0x100, 0x4860e0), (0x200, 0x4863c0), (0x8000000, 0x4861e0), (0x4000000, 0x4862c0), (0x400, 0x4864e0),
        (0x800, 0x486620), (0x1000, 0x486760), (0x2000, 0), (0x4000, 0x4868c0), (0x8000, 0x486970), (0x10000, 0x486a30),
        (0x20000, 0x486af0), (0x40000, 0x486c70), (0x20000000, 0x486e60), (0x80000, 0x486f80), (0x10000000, 0x487030),
        (0x100000, 0x487140), (0x200000, 0x487250), (0x400000, 0x487360),
    ];
    if sort & 1 != 0 {
        return if variant != 0 { 0x485630 } else { 0x485470 };
    }
    for (flag, cmp) in ORDER {
        if sort & flag == 0 {
            continue;
        }
        return match flag {
            0x10 => if national() { 0x485960 } else { 0x485a60 },
            0x2000 => if national() { 0x485960 } else { 0x486760 },
            _ => cmp,
        };
    }
    if sort & 0x800000 != 0 {
        0x487470
    } else if sort & 0x2000000 != 0 {
        0x485d20
    } else {
        0
    }
}

/// Whether the club's team sheet holds player `p`: the human's (`exe::tactics_find_selection_slot` with the reserves
/// flag, a place 0 or more) when he runs the club, otherwise the `team_sheet` given (20 players of the club's block).
unsafe fn squad_list_in_team_sheet(club: usize, p: usize, reserves: u32, team_sheet: usize) -> bool {
    if exe::human_manager_current_controls_club(club, 0, 1) != 0 {
        return exe::tactics_find_selection_slot(read_usize(G_TACTICS), club, p, reserves) as i8 > -1;
    }
    if team_sheet == 0 {
        return false;
    }
    let block = team_sheet + TEAM_SHEET_BLOCK * (read_u32(club) != read_u32(team_sheet)) as usize;
    (0..20).any(|k| read_u32(block + TEAM_SHEET_PLAYERS + TEAM_SHEET_PLAYER_SIZE * k) == read_u32(p))
}

/// Whether squad player `p` passes the list's filters (`filter`, see `build_filtered_sorted_squad_list`).
#[allow(clippy::too_many_arguments)]
unsafe fn squad_list_passes(club: usize, p: usize, reserves: u32, filter: u32, team_sheet: usize, fixture: usize, linked: usize) -> bool {
    let sheet = read_usize(p + staff::PLAYER);
    if exe::database_is_national_team(club) != 0 && exe::database_is_staff_foreign(p, read_usize(club + club::NATION), 1) != 0 {
        covered!("tel06_1 foreigner left a national squad");
        exe::database_remove_staff_from_club(club, p, 0usize, 1, 0);
    }
    if exe::database_is_national_team(club) == 0 && exe::human_manager_current_controls_club(club, 0, 1) == 0
        && read_i32(p) >= read_i32(G_STAFF_COUNT) && read_u8(p + staff::PLAYING_SQUAD) & 2 != 0 {
        return false;
    }
    if !(filter & 1 != 0 && filter & 0x20 != 0) {
        let mask = exe::formation_get_position_mask(sheet);
        let low = mask as u8;
        let groups: [(u32, bool); 7] = [(2, low & 1 != 0), (4, low & 6 != 0), (8, low & 0x38 != 0), (0x10, low & 0x40 != 0),
            (0x40, low & 0x80 != 0), (0x80, mask & 0x200 != 0), (0x100, mask & 0x800 != 0)];
        if groups.iter().any(|&(f, has)| filter & f != 0 && !has) {
            return false;
        }
    }
    if filter & 0x400 == 0 {
        let in_sheet = squad_list_in_team_sheet(club, p, reserves, team_sheet);
        if exe::database_is_national_team(club) != 0 {
            let flags = read_u8(p + staff::PLAYING_SQUAD);
            let needs = if reserves == 0 { flags & 8 != 0 } else { flags & 4 != 0 };
            if needs && !in_sheet {
                return false;
            }
        }
        if filter & 0x200 != 0 && exe::database_is_national_team(club) == 0 && exe::human_manager_current_controls_club(club, 0, 1) != 0 {
            let flags = get_squad_flags_for_club(read_usize(G_CONTRACT_MANAGER), p, club);
            let needs = if reserves == 0 { flags == 2 } else { flags == 1 };
            if needs && !in_sheet {
                return false;
            }
        }
    }
    if filter & 0x800 == 0 {
        let available = is_player_available_for_match(p, fixture, club, G_CURRENT_DATE) != 0;
        if (filter & 0x1000 != 0 && !available) || (filter & 0x2000 != 0 && available) {
            return false;
        }
    }
    if filter & 0x4000 == 0 && filter & 0x8000 != 0 && read_u8(p + staff::SQUAD_SELECTED_FOR) != 2 && read_i8(p + staff::AGE) > 0x13
        && (read_i32(club + club::CONTINENTAL_COMP_ID) != -1 || (linked != 0 && read_i32(linked + club::CONTINENTAL_COMP_ID) != -1)) {
        return false;
    }
    if filter & 0x10000 == 0 && filter & 0x20000 != 0 && read_i32(p) >= read_i32(G_STAFF_COUNT) {
        return false;
    }
    if filter & 0x40000 == 0 && filter & 0x80000 != 0 && read_u8(p + staff::SQUAD_SELECTED_FOR) == 8 {
        return false;
    }
    if filter & 0x100000 == 0 && filter & 0x200000 != 0 {
        let job = read_usize(p + staff::CLUB_JOB);
        if job != club && job != exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1) {
            return false;
        }
        let cm = read_usize(G_CONTRACT_MANAGER);
        if exe::contract_manager_get_contract(cm, p) == 0 {
            return false;
        }
        if read_u8(exe::contract_manager_get_contract(cm, p) + contract::CONTRACT_TYPE) & 0x3f == 5 {
            return false;
        }
    }
    true
}

/// Rule: a club's squad list keeps the players that pass the screen's filters (positions, team sheet, availability,
/// age rules, regens, the first team) and sorts them by the chosen column, optionally keeping only the best few.
///
/// `build_filtered_sorted_squad_list(club, reserves, sort_flags, name_style, opt4, opt5, opt6, filter_flags, list,
/// team_sheet, fixture, top_n, players, sort_variant)`. No club or no list: the "Error" box of club_screens.cpp line
/// 0x503f and 0. Otherwise the list is cleared (`exe::llist_clear`), the virtual players of the club are topped up
/// (`VIRTUAL_STAFF::fill_viewed_club_squad`, 0x90d4f0, on [G_VIRTUAL_STAFF]) and the 50 slots of `players` (the club's
/// `club::SQUAD` when null) are walked; a slot is kept when it holds a person with a player record and:
/// - (a national team removes a foreigner, `exe::database_is_staff_foreign` by `club::NATION`, from its squad first:
///   `exe::database_remove_staff_from_club`, and the player is still checked);
/// - not a hidden regen (a club that is not a national team and that the human does not run hides the people past
///   the database, `G_STAFF_COUNT`, with bit 2 of `staff::PLAYING_SQUAD`);
/// - his position mask (`exe::formation_get_position_mask`) has a bit of every group asked by filter bits 2, 4, 8,
///   0x10, 0x40, 0x80 and 0x100 (bits 1 and 0x20 together skip the groups);
/// - without filter bit 0x400, the team sheet (`exe::tactics_find_selection_slot` for the human's club, otherwise
///   `team_sheet`): a national team's player marked for the side (`staff::PLAYING_SQUAD` 8 for the first team, 4 for
///   the reserves) must be in it; with bit 0x200, at a club the human runs, so must one whose squad flags
///   (`CONTRACT_MANAGER::get_squad_flags_for_club`) are 2 (first team) or 1 (reserves);
/// - without bit 0x800, bit 0x1000 keeps those available for the fixture (`is_player_available_for_match`) and bit
///   0x2000 those not;
/// - without bit 0x4000, bit 0x8000 drops a player over 19 not selected for the side (`staff::SQUAD_SELECTED_FOR` 2)
///   when the club or its linked side (`exe::club_get_linked_b_team`) is in a continental competition;
/// - without bit 0x10000, bit 0x20000 drops the regens; without bit 0x40000, bit 0x80000 drops those selected for 8;
/// - without bit 0x100000, bit 0x200000 keeps only those employed by the club or its linked side, with a contract
///   (`exe::contract_manager_get_contract`) whose `contract::CONTRACT_TYPE` & 0x3f is not 5.
/// None kept: 1. Otherwise the sort context is written (the club; the club, or its linked side when it has one and
/// the out flag stays 0; the name style, the reserves flag, the team sheet, opt4..opt6, the fixture and the current
/// screen's parameter block), the players are sorted (`exe::crt_qsort`) by the comparator of the first sort flag
/// (`squad_list_comparator`), with `top_n` only the first `top_n` sorted again by 0x4857a0, and added to the list
/// (`exe::llist_add_tail`, 4 bytes each), backwards with sort flag 0x1000000. Returns 1.
/// Original: `build_filtered_sorted_squad_list` (0x4841d0, club_screens.cpp, cdecl).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn build_filtered_sorted_squad_list(club: usize, reserves: u32, sort_flags: u32, name_style: u32, opt4: u32,
    opt5: u32, opt6: u32, filter_flags: u32, list: usize, team_sheet: usize, fixture: usize, top_n: i32, players: usize, sort_variant: u32) -> u32 {
    if club == 0 || list == 0 {
        club_screens_error_box(0x503f);
        return 0;
    }
    covered!("tel06_1 squad list");
    let linked = exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1);
    exe::llist_clear(list);
    let fill: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x90d4f0usize);
    fill(read_usize(G_VIRTUAL_STAFF), club);
    let squad = if players != 0 { players } else { club + club::SQUAD };
    let mut kept = [0usize; 50];
    let mut count = 0usize;
    for k in 0..50 {
        let p = read_usize(squad + 4 * k);
        if p == 0 || read_usize(p + staff::PLAYER) == 0 {
            continue;
        }
        if squad_list_passes(club, p, reserves, filter_flags, team_sheet, fixture, linked) {
            kept[count] = p;
            count += 1;
        }
    }
    if count == 0 {
        return 1;
    }
    write_usize(SQUAD_LIST_SORT_CLUB, club);
    let mut no_link = 0u32;
    let partner = exe::club_get_linked_b_team(club, &mut no_link, 1);
    write_usize(0xadab3c, if no_link != 0 || partner == 0 { club } else { partner });
    write_u8(0xadab4a, opt4 as u8);
    write_u8(0xadab33, name_style as u8);
    write_u32(0xadab38, team_sheet as u32);
    write_u32(0xadab34, reserves);
    write_u8(0xad9db4, opt6 as u8);
    write_u8(0xad9db5, opt5 as u8);
    write_u32(0xadab44, fixture as u32);
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    write_usize(SQUAD_LIST_SORT_PARAMS, read_usize((G_SCREEN_MANAGER as isize + screen_manager::PARAM_BLOCK as isize + slot * 0x300) as usize));
    let arr = kept.as_mut_ptr() as usize;
    let cmp = squad_list_comparator(sort_flags, club, sort_variant);
    if cmp != 0 {
        exe::crt_qsort(arr, count as u32, 4, cmp);
    }
    if top_n != 0 {
        exe::crt_qsort(arr, (count as i32).min(top_n) as u32, 4, 0x4857a0usize);
    }
    if sort_flags & 0x1000000 != 0 {
        for k in (0..count).rev() {
            exe::llist_add_tail(list, arr + 4 * k, 4);
        }
    } else {
        for k in 0..count {
            exe::llist_add_tail(list, arr + 4 * k, 4);
        }
    }
    1
}

/// A squad list routine side by side: the sort context, the return and the list built (its items). The virtual
/// players topped up and a foreigner removed from a national squad by the first run are not undone (the second run
/// finds them done).
unsafe fn compare_squad_list(name: &str, list: usize, original: impl FnOnce() -> u32, rust: impl FnOnce() -> u32) -> u32 {
    let reg = [(SQUAD_LIST_SORT_CLUB, 8usize), (SQUAD_LIST_SORT_CONTEXT, 0x18), (SQUAD_LIST_SORT_PARAMS, 4)];
    let (a, b, same, _, _) = crate::comparison::side_by_side(&reg, || (original(), llist_items(list)), || (rust(), llist_items(list)));
    SCOREBOARD.record(same && a == b, || format!("{} (club squad list): returned {} with {:x?}, Rust {} with {:x?}", name, a.0, a.1, b.0, b.1));
    a.0
}

/// Comparer for `build_filtered_sorted_squad_list` (`compare_squad_list`); without a club or a list (the error box)
/// only the original runs.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn compare_build_filtered_sorted_squad_list(club: usize, reserves: u32, sort_flags: u32, name_style: u32, opt4: u32,
    opt5: u32, opt6: u32, filter_flags: u32, list: usize, team_sheet: usize, fixture: usize, top_n: i32, players: usize, sort_variant: u32) -> u32 {
    type F = unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, usize, usize, usize, i32, usize, u32) -> u32;
    let o = original_fn::<F>(%%I1%%);
    if club == 0 || list == 0 {
        return o(club, reserves, sort_flags, name_style, opt4, opt5, opt6, filter_flags, list, team_sheet, fixture, top_n, players, sort_variant);
    }
    compare_squad_list("build_filtered_sorted_squad_list", list,
        || o(club, reserves, sort_flags, name_style, opt4, opt5, opt6, filter_flags, list, team_sheet, fixture, top_n, players, sort_variant),
        || build_filtered_sorted_squad_list(club, reserves, sort_flags, name_style, opt4, opt5, opt6, filter_flags, list, team_sheet, fixture, top_n, players, sort_variant))
}

/// `build_filtered_sorted_squad_list` (0x4841d0) by address, as the original calls it.
#[allow(clippy::too_many_arguments)]
unsafe fn squad_list_call(club: usize, reserves: u32, sort_flags: u32, name_style: u32, opt4: u32, opt5: u32, opt6: u32, filter_flags: u32,
    list: usize, team_sheet: usize, fixture: usize, top_n: i32, players: usize, sort_variant: u32) -> u32 {
    let f: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, usize, usize, usize, i32, usize, u32) -> u32 = core::mem::transmute(0x4841d0usize);
    f(club, reserves, sort_flags, name_style, opt4, opt5, opt6, filter_flags, list, team_sheet, fixture, top_n, players, sort_variant)
}

/// Rule: the squad list of the staff and history screens: the first team sorted by sort flag 0x80000, name style 4,
/// filter 0x164c21, optionally the best `top_n`.
///
/// `build_squad_list_for_staff_history(club, list, top_n)` = `build_filtered_sorted_squad_list(club, 0, 0x80000, 4,
/// -1, -1, -1, 0x164c21, list, 0, 0, top_n, 0, 0)`; returns its result.
/// Original: `build_squad_list_for_staff_history` (0x484980, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn build_squad_list_for_staff_history(club: usize, list: usize, top_n: i32) -> u32 {
    squad_list_call(club, 0, 0x80000, 4, u32::MAX, u32::MAX, u32::MAX, 0x164c21, list, 0, 0, top_n, 0, 0)
}

/// Comparer for `build_squad_list_for_staff_history` (`compare_squad_list`).
pub(crate) unsafe extern "C" fn compare_build_squad_list_for_staff_history(club: usize, list: usize, top_n: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, i32) -> u32>(%%I2%%);
    if club == 0 || list == 0 {
        return o(club, list, top_n);
    }
    compare_squad_list("build_squad_list_for_staff_history", list, || o(club, list, top_n), || build_squad_list_for_staff_history(club, list, top_n))
}

/// Rule: the squad list of the club squad stats printout: the first team sorted by name (sort flag 4), name style 3,
/// filter 0x164c21.
///
/// `build_squad_list_by_name_for_printout(club, list)` = `build_filtered_sorted_squad_list(club, 0, 4, 3, -1, -1, -1,
/// 0x164c21, list, 0, 0, 0, 0, 0)`; returns its result.
/// Original: `build_squad_list_by_name_for_printout` (0x4849c0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn build_squad_list_by_name_for_printout(club: usize, list: usize) -> u32 {
    squad_list_call(club, 0, 4, 3, u32::MAX, u32::MAX, u32::MAX, 0x164c21, list, 0, 0, 0, 0, 0)
}

/// Comparer for `build_squad_list_by_name_for_printout` (`compare_squad_list`).
pub(crate) unsafe extern "C" fn compare_build_squad_list_by_name_for_printout(club: usize, list: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I3%%);
    if club == 0 || list == 0 {
        return o(club, list);
    }
    compare_squad_list("build_squad_list_by_name_for_printout", list, || o(club, list), || build_squad_list_by_name_for_printout(club, list))
}

/// `Injury::is_player_on_holiday` (0x631550, thiscall, ret 4).
unsafe fn status_on_holiday(injury: usize, p: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x631550usize);
    f(injury, p) != 0
}

/// `STAFF_CONTRACT::is_rolling` (0x85fa20, thiscall): the game reads al.
unsafe fn status_contract_rolling(contract: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x85fa20usize);
    f(contract) != 0
}

/// The contract checks that end both orders of `get_player_status_code`: leaving on a Bosman (0x12), grievances or
/// contract flags (0x13; `with_flags` asks `exe::staff_contract_get_flags`), a first-team squad status 2 with no
/// higher status bits for a player (0x14).
unsafe fn status_contract_code(p: usize, contract: usize, with_flags: bool) -> Option<u8> {
    if read_u8(contract + contract::LEAVING_ON_BOSMAN) != 0 {
        return Some(0x12);
    }
    if read_u32(contract + contract::GRIEVANCES) != 0 || (with_flags && exe::staff_contract_get_flags(contract) != 0) {
        return Some(0x13);
    }
    if read_u8(contract + contract::CONTRACT_TYPE) & 0x3f == 2 && read_u8(contract + contract::SQUAD_STATUS) & 0xf0 == 0 && read_usize(p + staff::PLAYER) != 0 {
        return Some(0x14);
    }
    None
}

/// The body of `get_player_status_code` once the interested clubs are in `interested` (see the routine).
unsafe fn status_code(p: usize, club: usize, fixture: usize, interested: usize) -> u8 {
    let injury = exe::injury_get_player_injury_status(read_usize(G_INJURY), p);
    let condition = exe::injury_get_condition(read_usize(G_INJURY), p, 1);
    let mut restrictions = [0u8; 0x20];
    let tm = read_usize(G_TRANSFER_MANAGER);
    if fixture != 0 {
        exe::transfer_manager_fill_squad_restrictions(tm, fixture, restrictions.as_mut_ptr(), 1);
    }
    exe::shortlist_manager_collect_interested_clubs(read_usize(tm + 0x856), p, interested, 1, 2, 0);
    let cm = read_usize(G_CONTRACT_MANAGER);
    let real = read_i32(p) < read_i32(G_STAFF_COUNT);
    let mut contract = 0usize;
    if real {
        contract = exe::contract_manager_get_contract_for_club(cm, p, club);
        if club != 0 && contract != 0 && read_u32(club) != read_u32(contract + contract::CLUB) && exe::contract_manager_get_loan_contract(cm, p) != 0
            && read_u32(exe::contract_manager_get_loan_contract(cm, p) + contract::CLUB) == read_u32(club) {
            contract = exe::contract_manager_get_loan_contract(cm, p);
        }
    }
    let listed = || read_u16(interested) != 0;
    let regen = || exe::player_regen_get_retirement_status(read_usize(G_PLAYER_REGEN), p) != 0;
    if club == 0 {
        covered!("tel06_1 status without a club");
        let order: [(u8, &dyn Fn() -> bool); 8] = [
            (0x18, &|| exe::transfer_manager_get_future_transfer(tm, p, 0) != 0),
            (0xb, &|| exe::transfer_manager_is_player_under_offer(tm, p, 1) != 0),
            (0xc, &listed),
            (0x17, &|| exe::scout_manager_has_scout_request(read_usize(G_SCOUT_MANAGER), p, 0usize, 0) != 0),
            (0xd, &|| exe::staff_is_listed_by_request(p, 0usize) != 0),
            (0xe, &|| exe::staff_is_transfer_listed(p, 0usize) != 0),
            (0xf, &|| exe::staff_is_loan_listed(p, 0usize) != 0),
            (0x10, &|| contract != 0 && status_contract_rolling(contract)),
        ];
        for (code, test) in order {
            if real && test() {
                return code;
            }
        }
        if real && regen() {
            return 0x11;
        }
        if contract != 0 {
            if let Some(code) = status_contract_code(p, contract, false) {
                return code;
            }
        }
        return match injury {
            2 => 6,
            1 => 8,
            _ => 0x19,
        };
    }
    let national = || exe::database_is_national_team(club) != 0;
    let nation = read_usize(club + club::NATION);
    if !national() && exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) != 0 {
        return 0x15;
    }
    if read_usize(G_INJURY) != 0 && status_on_holiday(read_usize(G_INJURY), p) {
        return 0x15;
    }
    if exe::transfer_manager_player_needs_work_permit(tm, p, club, 0) != 0 {
        return 1;
    }
    if fixture != 0 {
        if exe::transfer_manager_get_ineligibility_code(tm, p, fixture) == 1 || exe::transfer_manager_get_ineligibility_code(tm, p, fixture) == 0xa {
            return 2;
        }
        if exe::transfer_manager_get_ineligibility_code(tm, p, fixture) == 0xb || exe::transfer_manager_get_ineligibility_code(tm, p, fixture) != 0 {
            return 3;
        }
    }
    if exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), p, G_CURRENT_DATE, club, 0xde1f64u32, 0) != 0 {
        return 4;
    }
    if fixture != 0 && exe::discipline_is_player_banned(read_usize(G_DISCIPLINE), p, read_usize(fixture + fixture::COMPETITION), 1, 0) != 0 {
        return 5;
    }
    if injury == 2 {
        return 6;
    }
    if fixture != 0 && !national() && real && restrictions[0..3].iter().any(|&b| b != 0xff) && exe::database_is_staff_foreign(p, nation, 1) != 0 {
        return 7;
    }
    if injury == 1 {
        return 8;
    }
    if fixture != 0 && !national() && real && restrictions[3..6].iter().any(|&b| b != 0xff) && exe::database_is_staff_foreign(p, nation, 1) != 0
        && exe::database_is_staff_eu_national(p, nation) == 0 {
        return 9;
    }
    if condition < 0x41 {
        covered!("tel06_1 status unfit");
        return 0x16;
    }
    if real && exe::scout_manager_has_scout_request(read_usize(G_SCOUT_MANAGER), p, 0usize, 0) != 0 {
        return 0x17;
    }
    if fixture != 0 {
        let one_booking: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x55d360usize);
        if one_booking(read_usize(G_DISCIPLINE), p, read_usize(fixture + fixture::COMPETITION)) != 0 {
            return 0xa;
        }
    }
    let order: [(u8, &dyn Fn() -> bool); 7] = [
        (0x18, &|| exe::transfer_manager_get_future_transfer(tm, p, 0) != 0),
        (0xb, &|| exe::transfer_manager_is_player_under_offer(tm, p, 1) != 0),
        (0xc, &listed),
        (0xd, &|| exe::staff_is_listed_by_request(p, 0usize) != 0),
        (0xe, &|| exe::staff_is_transfer_listed(p, 0usize) != 0),
        (0xf, &|| exe::staff_is_loan_listed(p, 0usize) != 0),
        (0x10, &|| contract != 0 && status_contract_rolling(contract)),
    ];
    for (code, test) in order {
        if !national() && real && test() {
            return code;
        }
    }
    if real && regen() && read_i8(p + staff::AGE) > 0x1e {
        return 0x11;
    }
    if contract != 0 {
        if let Some(code) = status_contract_code(p, contract, true) {
            return code;
        }
    }
    0x19
}

/// Rule: the status code a squad list shows next to a player, the first that applies: for a club, away without leave
/// or on holiday (0x15), needs a work permit (1), ineligible for the fixture (2 or 3), on international duty (4),
/// banned in the competition (5), injured (6), over the foreigners limit (7), lightly injured (8), over the non-EU
/// limit (9), unfit (0x16), being scouted (0x17), one booking from a ban (0xa), a future transfer (0x18), under offer
/// (0xb), clubs interested (0xc), listed by request (0xd), transfer listed (0xe), loan listed (0xf), a rolling contract
/// (0x10), retiring (0x11), leaving on a Bosman (0x12), unhappy (0x13), a first-team player (0x14), otherwise 0x19.
///
/// `get_player_status_code(player, club, fixture)`. No player: the "Error" box of club_screens.cpp line 0x517f and 0.
/// The injury status and the condition as a percentage (`exe::injury_get_player_injury_status`,
/// `exe::injury_get_condition`), with a fixture the squad restrictions (`exe::transfer_manager_fill_squad_restrictions`,
/// six bytes: three foreigner and three non-EU limits, 0xff = none) and the clubs interested
/// (`exe::shortlist_manager_collect_interested_clubs` of the transfer manager's shortlists, +0x856, into a local list:
/// code 0xc when it has any). The contract: for a player of the database (`G_STAFF_COUNT`) the one for the club
/// (`exe::contract_manager_get_contract_for_club`), replaced by the loan contract when that one is with the club and
/// the first is not. With a club the order is the one above; a national team skips the away-without-leave check, the
/// foreigner limits and the transfer states (0x18 to 0x10), the database-only checks skip the regens, and the
/// retirement code needs an age over 30. Without a club: future transfer (0x18), under offer (0xb), interested clubs
/// (0xc), scouted (0x17), listed by request (0xd), transfer listed (0xe), loan listed (0xf), rolling contract (0x10),
/// retiring (0x11) for the database's players, then the contract codes (without the contract flags), injured (6),
/// lightly injured (8), otherwise 0x19.
/// Original: `get_player_status_code` (0x4849f0, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn get_player_status_code(player: usize, club: usize, fixture: usize) -> u8 {
    let mut interested = [0u8; 0x20];
    exe::new_llist(interested.as_mut_ptr() as usize);
    let code = if player == 0 {
        club_screens_error_box(0x517f);
        0
    } else {
        status_code(player, club, fixture, interested.as_mut_ptr() as usize)
    };
    exe::drop_llist(interested.as_mut_ptr() as usize);
    code
}

/// Comparer for `get_player_status_code`: the code and the international duty comment it may write (0xde1f64) side
/// by side; without a player (the error box) only the original runs.
pub(crate) unsafe extern "C" fn compare_get_player_status_code(player: usize, club: usize, fixture: usize) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u8>(%%I4%%);
    if player == 0 {
        return o(player, club, fixture);
    }
    let mut reg = vec![(0xde1f64usize, 0x7d0usize)];
    reg.extend_from_slice(&crate::rng::STATE_REGIONS);
    let (a, b, same, _, _) = crate::comparison::side_by_side(&reg, || o(player, club, fixture), || crate::rng::without_counting(|| get_player_status_code(player, club, fixture)));
    SCOREBOARD.record(same && a == b, || format!("get_player_status_code({:#x}, {:#x}, {:#x}) (club squad list) returned {:#x} and Rust {:#x}", player, club, fixture, a, b));
    a
}

/// The fixture's link data for the round label: the competition id, the round prefix and the round name packed 11
/// bits each (prefix and name masked to 11 bits).
unsafe fn fixture_row_round_link(fixture: usize) -> u32 {
    let id = read_u32(fixture + fixture::COMP_ID);
    (((id << 11) | (read_u16(fixture + fixture::ROUND_PREFIX) as u32 & 0x7ff)) << 11) | (read_u16(fixture + fixture::ROUND_NAME) as u32 & 0x7ff)
}

/// Whether the fixture's competition is a loaded one ([G_COMP_OBJECT_TABLE] by the fixture's competition id) that is
/// not a friendly of its own (`exe::friendly_is_own_comp_record`).
unsafe fn fixture_row_comp_loaded(fixture: usize, comp: usize) -> bool {
    read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_u32(fixture + fixture::COMP_ID) as usize) != 0
        && exe::friendly_is_own_comp_record(read_usize(G_FRIENDLY), comp) == 0
}

/// Whether the competition is current on the fixture's date (`Friendly::is_comp_current_on_date`, 0x5c29b0).
unsafe fn fixture_row_comp_current(fixture: usize, comp: usize) -> bool {
    let mut date = [0u8; 8];
    exe::new_date(date.as_mut_ptr() as usize, read_u16(fixture + fixture::DAY) as u32, read_u16(fixture + fixture::YEAR) as u32);
    let current: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x5c29b0usize);
    current(read_usize(G_FRIENDLY), comp, date.as_ptr() as usize) != 0
}

/// The result cell (column 5) once the result text is in the buffer: a link to the match details (0x7d3, the match
/// stats slot) when the match is still in the match stats (`exe::match_stats_fixture_entry_matches`), returning 1;
/// otherwise a plain text, returning `plain_returns`.
unsafe fn fixture_row_result(fixture: usize, area: u32, row: u32, plain_returns: u32) -> u32 {
    let found = exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), read_i32(fixture + fixture::MATCH_STATS_SLOT),
        read_i32(fixture + fixture::HOME_CLUB_ID), read_i32(fixture + fixture::AWAY_CLUB_ID), read_u16(fixture + fixture::DAY) as i32,
        read_u16(fixture + fixture::YEAR) as i32) != 0;
    let style = [area, 5, row, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32];
    if found {
        tel26_area_button([style[0], style[1], style[2], style[3], style[4], style[5], style[6], style[7], style[8], 0x7d3, read_u32(fixture + fixture::MATCH_STATS_SLOT)]);
        1
    } else {
        tel26_area_text(style);
        plain_returns
    }
}

/// Rule: a row of a club's fixtures list: the date (a link to the competition's fixtures when asked and the
/// competition is a current one), the result from the club's side (P:P postponed, A:A abandoned; a link to the match
/// details while they are kept, "view" for the match the human watches, otherwise a link to the fixture), the
/// opponent (yellow when human-run), its nation when foreign, the venue (H, A or N) and the round (a link to the
/// round when current).
///
/// `add_club_fixture_row(club, fixture, area, row, date_link_mode)`, cells of the screen manager's area `area` at
/// `row`, the texts written into the shared buffer (`JOB_TEXT`):
/// - column 0: the fixture's date without the year (`Date::get_short_weekday_date_string_no_year`, 0x54b550); a
///   button (`date_link_mode` as link, the fixture as data) when the mode is positive and the competition is a loaded
///   one that is neither a friendly of its own nor a reserve or minor competition (`exe::comp_is_reserve_or_minor_comp`),
///   otherwise a text; dark blue on pale grey;
/// - column 5: from the home side (the club is `fixture::HOME_CLUB`) the extra time score
///   (`fixture::HOME_GOALS_EXTRA_TIME` and away, when set) or the score (`fixture::HOME_GOALS`, -2 "P:P", -3 "A:A") as
///   home:away; from the away side away:home, where -3 prints as a number (a defect of the original) and the extra time
///   result as plain text returns 0; no result: "view" (`exe::human_manager_is_watching_fixture`) or the "no result"
///   text (0x98f858), a link to the fixture (0x7d2), returning 0. See `fixture_row_result`;
/// - column 1: "  " and the opponent's short name, a link to it (0x7d5), yellow when human-run
///   (`exe::human_manager_is_human_club`), otherwise pale grey;
/// - column 2: for a club that is not a national team, the opponent's nation's three letters when it differs, a link
///   (0x7d5) to its national team (`exe::database_get_national_team`), yellow;
/// - column 3: the venue (`exe::comp_util_get_fixture_venue_side`: 1 home "H", 2 away "A", otherwise "N");
/// - column 4: "  " and the round label (`exe::comp_util_format_fixture_round_label`, style 3); for a league division
///   (`exe::comp_util_is_league_division`) yellow, otherwise the friendly colour or orange
///   (`exe::friendly_is_friendly_competition`); a link to the round (0x7e5, `fixture_row_round_link`) when the
///   competition is loaded, not its own friendly (nor the reserve competition, `KEY_CLUB_COMP_RESERVE`, outside the
///   league divisions) and current on the date (`Friendly::is_comp_current_on_date`).
/// Returns 1 when the result cell is a details link or a plain score (0 for the cases above).
/// Original: `add_club_fixture_row` (0x483a90, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn add_club_fixture_row(club: usize, fixture: usize, area: u32, row: u32, date_link_mode: i32) -> u32 {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let short_date: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(0x54b550usize);
    let mut date = [0u8; 8];
    exe::new_date(date.as_mut_ptr() as usize, read_u16(fixture + fixture::DAY) as u32, read_u16(fixture + fixture::YEAR) as u32);
    short_date(date.as_ptr() as usize, JOB_TEXT, 0x7d0, 0xff);
    let comp = read_usize(fixture + fixture::COMPETITION);
    let date_style = [area, 0, row, 0x30, job_colour(JOB_COLOUR_BE04), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32];
    if date_link_mode > 0 && comp != 0 && fixture_row_comp_loaded(fixture, comp) && exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), comp) == 0 {
        covered!("tel06_1 fixture row date link");
        let s = date_style;
        tel26_area_button([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7], s[8], date_link_mode as u32, fixture as u32]);
    } else {
        tel26_area_text(date_style);
    }
    // the result cell, from the club's side
    let (et_home, et_away) = (read_i8(fixture + fixture::HOME_GOALS_EXTRA_TIME) as i32, read_i8(fixture + fixture::AWAY_GOALS_EXTRA_TIME) as i32);
    let (home, away) = (read_i8(fixture + fixture::HOME_GOALS) as i32, read_i8(fixture + fixture::AWAY_GOALS) as i32);
    const SCORE: usize = 0x990bb4; // "<%d - home score>:<%d - away score>"
    const POSTPONED: usize = 0x9918e0; // "P:P"
    const ABANDONED: usize = 0x9918dc; // "A:A"
    let home_side = club == read_usize(fixture + fixture::HOME_CLUB);
    let opponent = read_usize(fixture + if home_side { fixture::AWAY_CLUB } else { fixture::HOME_CLUB });
    let plain = [area, 5, row, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32];
    let mut ret = if et_home != -1 {
        if home_side {
            lang(JOB_TEXT, SCORE, et_home, et_away);
            fixture_row_result(fixture, area, row, 1)
        } else {
            lang(JOB_TEXT, SCORE, et_away, et_home);
            fixture_row_result(fixture, area, row, 0)
        }
    } else if home == -2 {
        lang(JOB_TEXT, POSTPONED);
        tel26_area_text(plain);
        1
    } else if home_side && home == -3 {
        lang(JOB_TEXT, ABANDONED);
        fixture_row_result(fixture, area, row, 1)
    } else if home != -1 {
        if home_side {
            lang(JOB_TEXT, SCORE, home, away);
        } else {
            covered!("tel06_1 fixture row away score");
            lang(JOB_TEXT, SCORE, away, home);
        }
        fixture_row_result(fixture, area, row, 1)
    } else {
        u32::MAX
    };
    if ret == u32::MAX {
        covered!("tel06_1 fixture row without a result");
        if exe::human_manager_is_watching_fixture(read_usize(G_HUMAN_MANAGER), fixture, 0usize) != 0 {
            lang(JOB_TEXT, 0x9918d4usize); // "view"
        } else {
            tel26_copy_text(JOB_TEXT as *mut u8, 0x98f858 as *const u8);
        }
        let s = plain;
        tel26_area_button([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7], s[8], 0x7d2, fixture as u32]);
        ret = 0;
    }
    // the opponent, its nation, the venue
    let human = exe::human_manager_is_human_club(opponent, 0, 1, 0, 0) != 0;
    sprintf(JOB_TEXT, 0x987ff0, opponent + club::SHORT_NAME); // "  %s"
    let colour = job_colour(if human { JOB_COLOUR_BDEC } else { JOB_COLOUR_3184 });
    tel26_area_button([area, 1, row, 1, 0, 1, 3, colour, JOB_TEXT as u32, 0x7d5, opponent as u32]);
    if exe::database_is_national_team(club) == 0 && read_usize(club + club::NATION) != read_usize(opponent + club::NATION) {
        let nation = read_usize(opponent + club::NATION);
        let team = exe::database_get_national_team(nation);
        tel26_area_button([area, 2, row, 1, 0, 1, 2, job_colour(JOB_COLOUR_BDEC), (nation + nation::THREE_LETTER_NAME) as u32, 0x7d5, team as u32]);
    }
    let venue = match exe::comp_util_get_fixture_venue_side(club, fixture) {
        1 => 0x98f880usize, // "H<%s - COMMENT - home>"
        2 => 0x98f868,      // "A<%s - COMMENT - away>"
        _ => 0x990b98,      // "N<%s - COMMENT - neutral>"
    };
    lang(JOB_TEXT, venue, 0xad9c64usize);
    tel26_area_text([area, 3, row, 1, 0, 0xc, 2, job_colour(JOB_COLOUR_BDE8), JOB_TEXT as u32]);
    // the round
    let mut label = [0u8; 0x7d0];
    exe::comp_util_format_fixture_round_label(fixture, 3, label.as_mut_ptr() as usize, 0x7d0);
    sprintf(JOB_TEXT, 0x987ff0, label.as_ptr());
    let (colour, linked) = if exe::comp_util_is_league_division(read_i32(fixture + fixture::COMP_ID)) != 0 {
        (job_colour(JOB_COLOUR_BDEC), comp != 0 && fixture_row_comp_loaded(fixture, comp))
    } else {
        let colour = job_colour(if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), comp) != 0 { 0xaebdde } else { JOB_COLOUR_BDD4 });
        let loaded = comp != 0 && read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_u32(fixture + fixture::COMP_ID) as usize) != 0
            && read_u32(comp) != read_u32(KEY_CLUB_COMP_RESERVE) && exe::friendly_is_own_comp_record(read_usize(G_FRIENDLY), comp) == 0;
        (colour, loaded)
    };
    if linked && fixture_row_comp_current(fixture, comp) {
        tel26_area_button([area, 4, row, 1, 0, 1, 2, colour, JOB_TEXT as u32, 0x7e5, fixture_row_round_link(fixture)]);
    } else {
        tel26_area_text([area, 4, row, 1, 0, 1, 2, colour, JOB_TEXT as u32]);
    }
    ret
}

/// Comparer for `add_club_fixture_row`: side by side over the display list (`compare_match_screen_build`) and the
/// return.
pub(crate) unsafe extern "C" fn compare_add_club_fixture_row(club: usize, fixture: usize, area: u32, row: u32, date_link_mode: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32, u32, i32) -> u32>(%%I5%%);
    let (mut a, mut b) = (0u32, 0u32);
    compare_match_screen_build("club fixture row", &[], || a = o(club, fixture, area, row, date_link_mode), || b = add_club_fixture_row(club, fixture, area, row, date_link_mode));
    SCOREBOARD.record(a == b, || format!("add_club_fixture_row (club fixtures tab) returned {} and Rust {}", a, b));
    a
}
