// ---------- batch %%LEVA%%: the club screens (1): dates, the screen manager's forwarders and a player's availability (club_screens.cpp) ----------

/// Rule: a date is later than another when its year is greater (the years compared as unsigned 16-bit words) or, in
/// the same year, when its day of the year is greater (signed).
///
/// `Date::operator>(other)`: `this` is the date, the other date comes by value in two dwords (`lo`: the day of the
/// year in the low word and the year in the high word; `hi`: the leap-year flag, not read). The other's year below
/// this year: 1; above: 0; the same year: 1 when the other's day is below this day (`setl`), otherwise 0.
/// Original: `Date::operator>` (0x456740, club_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn date_operator_greater(this: usize, lo: u32, _hi: u32) -> u32 {
    let (day, year) = (lo as u16 as i16, (lo >> 16) as u16);
    let this_year = read_u16(this + 2);
    if year < this_year {
        return 1;
    }
    if year > this_year {
        return 0;
    }
    (day < read_i16(this)) as u32
}

compare_value!(compare_date_operator_greater, date_operator_greater, %%I0%%, (lo: u32, hi: u32) -> u32);

/// Rule: a date is earlier than another when its year is smaller (unsigned 16-bit words) or, in the same year, when its
/// day of the year is smaller (signed).
///
/// `Date::operator<(other)`, the mirror of `date_operator_greater`: the other's year above this year: 1; below: 0; the
/// same year: 1 when the other's day is above this day (`setg`), otherwise 0. The leap-year dword is not read.
/// Original: `Date::operator<` (0x456770, club_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn date_operator_less(this: usize, lo: u32, _hi: u32) -> u32 {
    let (day, year) = (lo as u16 as i16, (lo >> 16) as u16);
    let this_year = read_u16(this + 2);
    if year > this_year {
        return 1;
    }
    if year < this_year {
        return 0;
    }
    (day > read_i16(this)) as u32
}

compare_value!(compare_date_operator_less, date_operator_less, %%I1%%, (lo: u32, hi: u32) -> u32);

/// The size of an `Area` in a `Display` (the areas start at +4: +4 + 0xbf1 × k).
const DISPLAY_AREA_SIZE: usize = 0xbf1;

/// The `Display` of the screen manager's current slot (`screen_manager::DISPLAY` of the slot
/// `screen_manager::CURRENT_SLOT`, 0x300 bytes per slot), read from the manager `sm` as the original does.
unsafe fn screens_current_display(sm: usize) -> usize {
    read_usize(sm + screen_manager::DISPLAY + read_i16(sm + screen_manager::CURRENT_SLOT) as isize as usize * 0x300)
}

/// Rule: an area of the current screen scrolls to a row; area −1 (none) does nothing.
///
/// `ScreenManager::set_area_scroll_row(area, row)`: the area (its low word, sign-extended) of the current slot's
/// `Display` (`Display` + 4 + 0xbf1 × area) gets the row through `exe::area_set_scroll_row` (thiscall, ret 4).
/// Original: `ScreenManager::set_area_scroll_row` (0x4567a0, club_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screenmanager_set_area_scroll_row(sm: usize, area: u32, row: u32) {
    if area as u16 == 0xffff {
        return;
    }
    covered!("tel01_1 area scroll row");
    let display = screens_current_display(sm);
    exe::area_set_scroll_row(display.wrapping_add(4).wrapping_add((area as i16 as isize as usize).wrapping_mul(DISPLAY_AREA_SIZE)), row as usize);
}

/// The screen-list writes a forwarder makes: the network packet it appends (the position [0xae290c] and up to 0x200
/// bytes from it in the buffer [0xae2908], never past the limit [0xae2910]) and, in the current `Display`, the counts
/// of areas and items (+0x12e99e, +0x12e9a0), the next area slot and the next item slot, and the two overflow flags.
unsafe fn screens_forwarder_regions(sm: usize) -> Vec<(usize, usize)> {
    let mut r = vec![(0xae290c, 4)];
    let (base, pos, limit) = (read_usize(0xae2908), read_usize(0xae290c), read_usize(0xae2910));
    if base != 0 && pos < limit {
        r.push((base + pos, (limit - pos).min(0x200)));
    }
    let display = screens_current_display(sm);
    if display != 0 {
        r.push((display + 0x12e99e, 4));
        let areas = read_i16(display + 0x12e99e);
        if (0..0xf9).contains(&areas) {
            r.push((display + 4 + areas as usize * DISPLAY_AREA_SIZE, DISPLAY_AREA_SIZE));
        }
        let items = read_i16(display + 0x12e9a0);
        if (0..0x4af).contains(&items) {
            r.push((display + 0xba95e + items as usize * 0x18c, 0x18c));
        }
        r.push((display + 0x12f549, 8));
    }
    r
}

/// The screen-list writes of a `ScreenManager` item helper: what `screens_forwarder_regions` covers, plus the
/// free item draw order (`display_list::ITEM_ORDER_COUNT` and its next slot) and, for an item that goes into an area
/// (`area` as i16 ≠ −1), that whole area (`Area::add_item` appends the index to its list).
unsafe fn scrman_item_regions(sm: usize, area: u32) -> Vec<(usize, usize)> {
    let mut r = screens_forwarder_regions(sm);
    let display = screens_current_display(sm);
    if display != 0 {
        r.push((display + display_list::ITEM_ORDER_COUNT, 2));
        let n = read_i16(display + display_list::ITEM_ORDER_COUNT);
        if (0..0x4b0).contains(&n) {
            r.push((display + display_list::ITEM_ORDER + n as usize * 2, 2));
        }
        let a = area as i16;
        if (0..0xf9).contains(&a) {
            r.push((display_area_at(display, a), display_list::AREA_SIZE));
        }
    }
    r
}

/// Comparer of a forwarder: side by side over `screens_forwarder_regions` and the return value.
unsafe fn compare_screens_forwarder(name: &str, sm: usize, original: impl FnOnce() -> u32, rust: impl FnOnce() -> u32) -> u32 {
    compare_screens_over(name, &screens_forwarder_regions(sm), original, rust)
}

/// Comparer of an item forwarder: side by side over `scrman_item_regions` (the item's area or the free item order
/// too, so the item the original appended is not appended again by the copy) and the return value.
unsafe fn compare_screens_item(name: &str, sm: usize, area: u32, original: impl FnOnce() -> u32, rust: impl FnOnce() -> u32) -> u32 {
    compare_screens_over(name, &scrman_item_regions(sm, area), original, rust)
}

/// Side by side over `regions` and the return value.
unsafe fn compare_screens_over(name: &str, regions: &[(usize, usize)], original: impl FnOnce() -> u32, rust: impl FnOnce() -> u32) -> u32 {
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(regions, original, rust);
    SCOREBOARD.record(equal && ro == rr, || format!("{}: return {:#x} x {:#x}, differences in {:?}", name, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// `Display::add_area` (0x55fba0, thiscall, ret 0x2c): the eleven dwords of an area.
type DisplayAddAreaFn = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
/// `Display::add_item` (0x55f990, thiscall, ret 0x48): the eighteen dwords of an item.
type DisplayAddItemFn = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;

/// Rule: adding an area to the screen being built goes to the current slot's display list.
///
/// `ScreenManager::add_area(..)`: the eleven arguments passed on unchanged to `Display::add_area` (`FN_DISPLAY_ADD_AREA`)
/// of the current slot's `Display`; its return (the area index) comes back whole.
/// Original: `ScreenManager::add_area` (0x4567f0, club_screens.cpp, thiscall, ret 0x2c).
pub(crate) unsafe extern "thiscall" fn screenmanager_add_area(sm: usize, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, a9: u32, a10: u32) -> u32 {
    let f: DisplayAddAreaFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    f(screens_current_display(sm), a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10)
}

/// Comparer for `screenmanager_add_area`.
pub(crate) unsafe extern "thiscall" fn compare_screenmanager_add_area(sm: usize, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, a9: u32, a10: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32>(%%I3%%);
    compare_screens_forwarder("screenmanager_add_area", sm,
        || o(sm, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10),
        || screenmanager_add_area(sm, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10))
}

/// Rule: a popup area (a drop-down list opened from a widget) is an empty area with flags 0x130 in the popup colour.
///
/// `ScreenManager::add_popup_area(owner)`: `Display::add_area` of the current slot's `Display` with an empty rectangle
/// and nothing else (eight zeros), the flags 0x130, the popup colour (the word at 0xae31aa; the original pushes it in a
/// dx whose high word is the caller's edx, here zero) and the owner widget (the whole dword); the area index comes back.
/// Original: `ScreenManager::add_popup_area` (0x456850, club_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screenmanager_add_popup_area(sm: usize, owner: u32) -> u32 {
    let f: DisplayAddAreaFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    f(screens_current_display(sm), 0, 0, 0, 0, 0, 0, 0, 0, 0x130, read_u16(0xae31aa) as u32, owner)
}

/// Comparer for `screenmanager_add_popup_area`.
pub(crate) unsafe extern "thiscall" fn compare_screenmanager_add_popup_area(sm: usize, owner: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I4%%);
    // simplification: the high word of the colour is the caller's edx in the original (garbage); Display::add_area
    // keeps the colour as a word
    compare_screens_forwarder("screenmanager_add_popup_area", sm, || o(sm, owner), || screenmanager_add_popup_area(sm, owner))
}

/// Rule: an item of a popup list is a ticked text item (flags 0x2002) with the popup colours, carrying its id and
/// data.
///
/// `ScreenManager::add_ticked_popup_item(area, text, id, data)`: `Display::add_item` of the current slot's `Display`
/// with 0x2002, six zeros, 0x10, the popup colour (0xae31aa), the text colour (0xaebdec), 1, 1, the highlight colour
/// (0xaebdea), the text, 0, the id, the data and the area. The colours go as the original pushes them: the word in the
/// low half and, in the high half, what the register held (the id's high word for 0xae31aa and 0xaebdea, the text
/// pointer's for 0xaebdec). The item index comes back.
/// Original: `ScreenManager::add_ticked_popup_item` (0x456890, club_screens.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn screenmanager_add_ticked_popup_item(sm: usize, area: u32, text: u32, id: u32, data: u32) -> u32 {
    let f: DisplayAddItemFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let high = |v: u32, colour: usize| (v & 0xffff_0000) | read_u16(colour) as u32;
    f(screens_current_display(sm), 0x2002, 0, 0, 0, 0, 0, 0, 0x10, high(id, 0xae31aa), high(text, 0xaebdec), 1, 1,
      high(id, 0xaebdea), text, 0, id, data, area)
}

/// Comparer for `screenmanager_add_ticked_popup_item`.
pub(crate) unsafe extern "thiscall" fn compare_screenmanager_add_ticked_popup_item(sm: usize, area: u32, text: u32, id: u32, data: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(%%I5%%);
    compare_screens_item("screenmanager_add_ticked_popup_item", sm, area, || o(sm, area, text, id, data),
        || screenmanager_add_ticked_popup_item(sm, area, text, id, data))
}

/// Rule: adding an item to the screen being built goes to the current slot's display list.
///
/// `ScreenManager::add_item(..)`: the eighteen arguments passed on unchanged to `Display::add_item` (`FN_DISPLAY_ADD_ITEM`)
/// of the current slot's `Display`; its return (the item index) comes back whole.
/// Original: `ScreenManager::add_item` (0x45cb60, club_screens.cpp, thiscall, ret 0x48).
pub(crate) unsafe extern "thiscall" fn screenmanager_add_item(sm: usize, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, a9: u32, a10: u32, a11: u32, a12: u32, a13: u32, a14: u32, a15: u32, a16: u32, a17: u32) -> u32 {
    let f: DisplayAddItemFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    f(screens_current_display(sm), a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16, a17)
}

/// Comparer for `screenmanager_add_item`.
pub(crate) unsafe extern "thiscall" fn compare_screenmanager_add_item(sm: usize, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, a9: u32, a10: u32, a11: u32, a12: u32, a13: u32, a14: u32, a15: u32, a16: u32, a17: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32>(%%I6%%);
    compare_screens_item("screenmanager_add_item", sm, a17,
        || o(sm, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16, a17),
        || screenmanager_add_item(sm, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16, a17))
}

/// Rule: a player can play a match unless he is injured, playing for this club on loan from it, ineligible for the
/// fixture, away on international duty, banned for the competition or in need of a work permit; with no fixture he is
/// always available.
///
/// `is_player_available_for_match(player, fixture, club, comp)`, in order: no player: 0; no fixture: 1; no club or no
/// competition: 0; injury status 2 (`exe::injury_get_player_injury_status`): 0; a loan contract
/// (`exe::contract_manager_get_loan_contract`) together with a contract (`exe::contract_manager_get_contract`, asked
/// twice as the original) whose `contract::CLUB` is the club's id: 0; an ineligibility code for the fixture
/// (`exe::transfer_manager_get_ineligibility_code`, its byte): 0; an international duty club
/// (`exe::nationalteams_get_international_duty_club` with the competition and the club, 0, 0): 0; banned
/// (`exe::discipline_is_player_banned` with the fixture's `fixture::COMPETITION`, 1, 0): 0; otherwise 1 when he does not
/// need a work permit for the club (`exe::transfer_manager_player_needs_work_permit`, 0).
/// Original: `is_player_available_for_match` (0x45cbe0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn is_player_available_for_match(player: usize, fixture: usize, club: usize, comp: usize) -> u32 {
    if player == 0 {
        return 0;
    }
    if fixture == 0 {
        return 1;
    }
    if club == 0 || comp == 0 {
        return 0;
    }
    if exe::injury_get_player_injury_status(read_usize(G_INJURY), player) == 2 {
        covered!("tel01_1 unavailable injured");
        return 0;
    }
    let contracts = read_usize(G_CONTRACT_MANAGER);
    if exe::contract_manager_get_loan_contract(contracts, player) != 0 && exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player) != 0 {
        let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
        if read_u32(contract + contract::CLUB) == read_u32(club) {
            covered!("tel01_1 unavailable loaned from the club");
            return 0;
        }
    }
    if exe::transfer_manager_get_ineligibility_code(read_usize(G_TRANSFER_MANAGER), player, fixture) != 0 {
        return 0;
    }
    if exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), player, comp, club, 0, 0) != 0 {
        return 0;
    }
    if exe::discipline_is_player_banned(read_usize(G_DISCIPLINE), player, read_usize(fixture + fixture::COMPETITION), 1, 0) != 0 {
        covered!("tel01_1 unavailable banned");
        return 0;
    }
    (exe::transfer_manager_player_needs_work_permit(read_usize(G_TRANSFER_MANAGER), player, club, 0) == 0) as u32
}

/// Comparer for `is_player_available_for_match`: the answer, side by side (the routine only reads).
pub(crate) unsafe extern "C" fn compare_is_player_available_for_match(player: usize, fixture: usize, club: usize, comp: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, usize, usize) -> u32>(%%I7%%)(player, fixture, club, comp);
    let n = is_player_available_for_match(player, fixture, club, comp);
    SCOREBOARD.record(r == n, || format!("is_player_available_for_match({:#x}, {:#x}, {:#x}, {:#x}) original={} rust={}", player, fixture, club, comp, r, n));
    r
}

/// Comparer for `screenmanager_set_area_scroll_row`: side by side over the area (its 0xbf1 bytes).
pub(crate) unsafe extern "thiscall" fn compare_screenmanager_set_area_scroll_row(sm: usize, area: u32, row: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I2%%);
    let display = screens_current_display(sm);
    if area as u16 == 0xffff || display == 0 {
        return o(sm, area, row);
    }
    let regions = [(display.wrapping_add(4).wrapping_add((area as i16 as isize as usize).wrapping_mul(DISPLAY_AREA_SIZE)), DISPLAY_AREA_SIZE)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(sm, area, row), || screenmanager_set_area_scroll_row(sm, area, row));
    SCOREBOARD.record(equal, || format!("screenmanager_set_area_scroll_row(area {}, row {}): differences in {:?}", area as i16, row, first_differences(&mo, &mr)));
}
