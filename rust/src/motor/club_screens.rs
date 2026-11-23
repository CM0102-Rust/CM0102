use super::*;

/// The destructor of a match record's two team blocks: the C runtime's vector destructor over the two 0x625-byte
/// blocks at +0x11f, each with `match_team_block_destructor` (0x453c10: the vector destructor over its twenty
/// 0x49-byte entries at +0x71, whose destructor 0x5a53c0 is a bare `ret`), so nothing is actually written.
///
/// `match_team_pair_destructor(this)`: `exe::crt_vector_destructor_iterator(this + 0x11f, 0x625, 2, 0x453c10)`.
/// Original: `match_team_pair_destructor` (0x453bc0, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn match_team_pair_destructor(this: usize) {
    covered!("tel39_4 team pair destructor");
    exe::crt_vector_destructor_iterator(this + MATCH_TEAM_PAIR, MATCH_TEAM_BLOCK_SIZE as u32, 2, MATCH_TEAM_BLOCK_DESTRUCTOR);
}

/// The constructor of a record with a date at +0x14 (the average rating record of the club and competition
/// records): only the date is built.
///
/// `rating_record_construct(this)`: `Date::Date` (0x54a110) on `this + 0x14`; returns `this`.
/// Original: `rating_record_construct` (0x453c50, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn rating_record_construct(this: usize) -> usize {
    covered!("tel39_4 rating record constructor");
    // tel39_4: the date at +0x14
    exe::date_date_54a110(this + RATING_RECORD_DATE);
    this
}

/// The constructor of a human manager's squad screen view settings (15 bytes, in his HUMAN_RECORD): the default
/// view, sort and filter.
///
/// `squad_view_settings_construct(this)`: dword +0 = 0x41, dword +4 = 8, dword +8 = 0x154a21, byte +0xc = 6, bytes
/// +0xd and +0xe = 0xff [inference: the columns shown, the sort column and the filters]; returns `this`.
/// Original: `squad_view_settings_construct` (0x487810, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn squad_view_settings_construct(this: usize) -> usize {
    covered!("tel39_4 squad view settings");
    write_u32(this, 0x41);
    write_u32(this + 4, 8);
    write_u8(this + 0xc, 6);
    // tel39_4: the default view word
    write_u32(this + 8, 0x154a21);
    write_u8(this + 0xd, 0xff);
    write_u8(this + 0xe, 0xff);
    this
}

/// Comparer for `match_team_pair_destructor`: only the original runs (a destructor does not run twice; its element
/// destructors write nothing, so there is no outcome to check).
pub(crate) unsafe extern "thiscall" fn compare_match_team_pair_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_MATCH_TEAM_PAIR_DESTRUCTOR)(this)
}

/// Comparer for `rating_record_construct`: side by side over the date and the return.
pub(crate) unsafe extern "thiscall" fn compare_rating_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_RATING_RECORD_CONSTRUCT);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, RATING_RECORD_DATE + 8)], || o(this), || rating_record_construct(this));
    SCOREBOARD.record(same && r == n, || format!("rating_record_construct({:#x}) original={:#x} {:x?} rust={:#x} {:x?}", this, r, mo, n, mr));
    r
}

/// Comparer for `squad_view_settings_construct`: side by side over the 15 bytes and the return.
pub(crate) unsafe extern "thiscall" fn compare_squad_view_settings_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_SQUAD_VIEW_SETTINGS_CONSTRUCT);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, 15)], || o(this), || squad_view_settings_construct(this));
    SCOREBOARD.record(same && r == n, || format!("squad_view_settings_construct({:#x}) original {:x?} rust {:x?}", this, mo, mr));
    r
}

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32>(IDX_SCREENMANAGER_ADD_AREA);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_SCREENMANAGER_ADD_POPUP_AREA);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(IDX_SCREENMANAGER_ADD_TICKED_POPUP_ITEM);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32>(IDX_SCREENMANAGER_ADD_ITEM);
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
    let fixture = Fixture(fixture);
    if player == 0 {
        return 0;
    }
    if fixture.is_null() {
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
        let contract = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player));
        if (contract.club() as u32) == read_u32(club) {
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
    if exe::discipline_is_player_banned(read_usize(G_DISCIPLINE), player, fixture.competition().addr(), 1, 0) != 0 {
        covered!("tel01_1 unavailable banned");
        return 0;
    }
    (exe::transfer_manager_player_needs_work_permit(read_usize(G_TRANSFER_MANAGER), player, club, 0) == 0) as u32
}

/// Comparer for `is_player_available_for_match`: the answer, side by side (the routine only reads).
pub(crate) unsafe extern "C" fn compare_is_player_available_for_match(player: usize, fixture: usize, club: usize, comp: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, usize, usize) -> u32>(IDX_IS_PLAYER_AVAILABLE_FOR_MATCH)(player, fixture, club, comp);
    let n = is_player_available_for_match(player, fixture, club, comp);
    SCOREBOARD.record(r == n, || format!("is_player_available_for_match({:#x}, {:#x}, {:#x}, {:#x}) original={} rust={}", player, fixture, club, comp, r, n));
    r
}

/// Comparer for `screenmanager_set_area_scroll_row`: side by side over the area (its 0xbf1 bytes).
pub(crate) unsafe extern "thiscall" fn compare_screenmanager_set_area_scroll_row(sm: usize, area: u32, row: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_SCREENMANAGER_SET_AREA_SCROLL_ROW);
    let display = screens_current_display(sm);
    if area as u16 == 0xffff || display == 0 {
        return o(sm, area, row);
    }
    let regions = [(display.wrapping_add(4).wrapping_add((area as i16 as isize as usize).wrapping_mul(DISPLAY_AREA_SIZE)), DISPLAY_AREA_SIZE)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(sm, area, row), || screenmanager_set_area_scroll_row(sm, area, row));
    SCOREBOARD.record(equal, || format!("screenmanager_set_area_scroll_row(area {}, row {}): differences in {:?}", area as i16, row, first_differences(&mo, &mr)));
}

/// Rule: opening a human manager's club screen shows the club he is managing in a live match, otherwise his club, otherwise
/// his national team, on the squad tab (or the tactics tab), with the view he left the squad screen in, and marks his
/// three best known players.
///
/// `open_manager_club_screen(manager, squad_param, restore_params, open_tab_5, set_flag_0x800)`. No manager: the error
/// box (line 0x32b) and nothing else. The club: the manager's record (`exe::get_human_manager_record`) with a live
/// match (`human_manager_sheet::LIVE_MATCH`) gives `human_manager_sheet::CLUB`; otherwise `staff::CLUB_JOB`, otherwise
/// `staff::NATIONAL_JOB`. With `restore_params`, the 60 parameters of the current screen are read first. The view: the
/// human's squad view block (`HUMAN_MANAGER::get_squad_view_settings`, 0x5fe950) when he runs the club
/// (`exe::human_manager_current_controls_club`(club, 0, 1)), otherwise the default block (0x41, 8, 0x154a21, 6, -1,
/// -1). The screen opens for the manager (`exe::screenmanager_open_screen_for_each_record` with the build 0x454900,
/// the input 0x4671f0 and the leave 0x469da0; when it does not open, nothing else) and gets, for the manager
/// (`exe::screenmanager_set_screen_param_for_each_record`), its parameters 0..0x1f: 0 the club, 1 `squad_param`, 4 the
/// tab (5 with `open_tab_5`, otherwise 1), 7 and 8 the view's first two dwords, 9 its byte +0xc, 0xc and 0x17 the
/// current year, 0xf and 0x12 one, 0x18 the view's third dword, 0x19 and 0x1a its bytes +0xd and +0xe (sign-extended),
/// 0x1b 2 with `open_tab_5` (otherwise 1), all others 0; with `restore_params`, 4, 7, 8, 9, 0x10..0x12 and 0x18..0x1b
/// come back from the old screen. With `set_flag_0x800`, the current screen's parameter 7 gets the view's first dword
/// with bit 0x800 (`exe::screenmanager_set_screen_param`). Last, parameters 0x20..0x22: the ids of the three players
/// of the squad (`club::SQUAD`, those with a player record) with the best reputation, by strict comparison
/// (`player_sheet::WORLD_REPUTATION` for a national team, `exe::database_is_national_team`, otherwise
/// `player_sheet::CURRENT_REPUTATION`; signed words from -10000); -1 for the places not filled.
/// Original: `open_manager_club_screen` (0x4542f0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_manager_club_screen(manager: Staff, squad_param: u32, restore_params: u32, open_tab_5: u32, set_flag_0x800: u32) {
    if manager.is_null() {
        club_screens_error_box(0x32b);
        return;
    }
    covered!("tel01_2 open the club screen");
    let p = open_manager_club_screen_plan(manager, squad_param, restore_params, open_tab_5);
    let sm = G_SCREEN_MANAGER;
    // the view block is read again for the 0x800 flag (the plan has already read it)
    let club = p[0] as usize;
    let view_first = if club != 0 && exe::human_manager_current_controls_club(club, 0, 1) != 0 {
        read_u32(club_screens_squad_view_settings())
    } else {
        0x41
    };
    if exe::screenmanager_open_screen_for_each_record(sm, manager.addr() as u32, 0x454900, 0x4671f0, 0, 0x469da0, 0) == 0 {
        return;
    }
    let set = |k: u32| exe::screenmanager_set_screen_param_for_each_record(sm, manager.addr() as u32, k, p[k as usize] as usize, 0);
    for k in 0..0x20 {
        set(k);
    }
    if restore_params != 0 {
        for k in [4u32, 7, 8, 9, 0x10, 0x11, 0x12, 0x18, 0x19, 0x1a, 0x1b] {
            set(k);
        }
    }
    if set_flag_0x800 != 0 {
        exe::screenmanager_set_screen_param(sm, 7, (view_first | 0x800) as usize, 0);
    }
    for k in 0x20..0x23 {
        set(k);
    }
}

/// Comparer for `open_manager_club_screen`.
///
/// Opening cannot run twice: only the original runs, and then the parameters of the manager's new screen (his channel,
/// `exe::screenmanager_find_manager_channel`, parameter n at +0x14 + 8n of the block `screen_manager::PARAM_BLOCK`) are
/// checked against the plan worked out before (`open_manager_club_screen_plan`). Without a manager, or when the screen
/// did not open (parameter 0 is not the planned club), nothing is checked.
pub(crate) unsafe extern "C" fn compare_open_manager_club_screen(manager: Staff, squad_param: u32, restore_params: u32, open_tab_5: u32, set_flag_0x800: u32) {
    let o = original_fn::<unsafe extern "C" fn(Staff, u32, u32, u32, u32)>(IDX_OPEN_MANAGER_CLUB_SCREEN);
    if manager.is_null() {
        return o(manager, squad_param, restore_params, open_tab_5, set_flag_0x800);
    }
    let plan = crate::rng::without_counting(|| open_manager_club_screen_plan(manager, squad_param, restore_params, open_tab_5));
    o(manager, squad_param, restore_params, open_tab_5, set_flag_0x800);
    let slot = exe::screenmanager_find_manager_channel(G_SCREEN_MANAGER, manager) as i16;
    if !(0..64).contains(&slot) {
        return;
    }
    let block = read_usize(G_SCREEN_MANAGER + screen_manager::PARAM_BLOCK + slot as usize * 0x300);
    if block == 0 || read_u32(block + 0x14) != plan[0] {
        return;
    }
    let differ: Vec<usize> = (0..0x23).filter(|&k| k != 7 || set_flag_0x800 == 0).filter(|&k| read_u32(block + 0x14 + 8 * k) != plan[k]).collect();
    SCOREBOARD.record(differ.is_empty(), || format!("open_manager_club_screen (club screen): parameters {:x?} differ from the plan", differ));
}

/// Rule: the club screen shows the club's name (or its penalty takers), its tabs (Squad, Transfers, Next Match,
/// Fixtures, Finances & Info or General Info, Tactics, Training, Last Match, the division and History; the transfers
/// and training of a club the human does not run are greyed out), the chosen tab, the squad menu, the action button
/// or the print menu, and the back and next buttons; it remembers the last and the next fixture of the club.
///
/// `club_screen_build()`, from the screen parameters 0..7 (`get_screen_param`); [0x98ded8] = 1 first. No club (param 0):
/// the current human's (`screen_manager::DISPLAY` slot's human at +0x7d6: `staff::CLUB_JOB`, otherwise
/// `staff::NATIONAL_JOB`) written to param 0; still none: the error box (line 0x416) and nothing else. Param 1 (the
/// reserves) set when the human does not run the club and it is not a national team goes back to 0. Param 2 empty (the
/// last match): the club's fixtures from 8 weeks back (24 for a national team; `exe::date_sub_weeks`, the year of a first
/// result and the day of a second, as the original) to today (`exe::fixture_manager_find_fixtures` with the club's id,
/// the local list, -1, -2 and "no reserves") and, from the end of the list (`exe::llist_get_prev`), the first one with
/// the byte +0x43 ≥ 0 (played), copied (`fixture::SIZE` bytes) into param 2. Param 3 empty (the next match): the same
/// with the fixtures from today to 8 (24) weeks ahead (`exe::date_add_weeks`), appended to the same list, and from its
/// start (`exe::llist_get_next`) the first with +0x43 = -1. Params 5 and 6 both empty: the club's kit colours
/// (`exe::database_get_club_kit_colours`, kit 1) written there. The club's background picture
/// (`exe::background_set_club_picture`). Bit 0x800 of param 7 (the penalty takers) dropped and written back when the
/// human does not run the club. The title: the club's name (`exe::club_get_national_team_name`, style 3) or, with bit
/// 0x800 on the squad tab of a club he runs, "{}<%s>{} Penalty Takers" with the short name (style 1), in the colours of
/// params 6 and 5 (`exe::screenmanager_add_screen_title`). The menu bar (`exe::menubar_build` 4). The ten tabs
/// (text, kind, link): "Squad" 2/1; "Transfers" 2/2 (for a national team greyed 0/0, and the transfers tab becomes the
/// squad tab, param 4 = 1); "Next Match" 2/3; "Fixtures" 2/4; "Finances & Info" (a club he runs that is not a national
/// team) or "General Info" 2/5; "Tactics" 3/0; "Training" 4/0 (greyed 0/0 unless he runs the club and it is not a
/// national team); "Last Match" 5/0; the division: with the club's current league record
/// (`club_history_get_current_league_record`, 0x444190) "<pos> in {}<division>{}" (the ordinal of position + 1,
/// `exe::utils_format_ordinal`) when the record's byte +0xb is positive, otherwise the division's name (its short name
/// when the name has more than 20 characters), kind 0x7e5 with the link (division id << 11 | stage +0x3a & 0x7ff) << 11
/// when there is a stage, otherwise 0x7d0/0; without a record "Division" greyed 0/0; "History" 6/0. The tab bar
/// (`exe::gui_add_tab_bar`, 10 tabs, from 0x50 to 0x221, split), then the tab of param 4: 1 the squad (0x456900), 2 the
/// transfers (`club_screen_draw_transfers_tab`), 3 the next match (0x45ccf0), 4 the fixtures (0x45f380), 5 the info
/// (0x460590); another: the error box (line 0x505). Then the squad menu (`build_club_squad_navigation_menu`, 0x481ef0)
/// and the action button (`add_club_screen_action_button`, 0x482a40); when there is no action button, the "Print"
/// button (its colour 0xae3184 when param 6 is the colour 0xaebdea) with its popup of "Squad" 1, "Stats" 2,
/// "Transfers" 4 (a disabled item for a national team), "Fixtures" 8 and "All" 0xf. Last the back and next buttons
/// (`exe::gui_add_back_next_buttons`).
/// Original: `club_screen_build` (0x454900, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screen_build() {
    let sm = G_SCREEN_MANAGER;
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut list = [0u8; 0x20];
    club_screens_llist(&mut list, true);
    let mut tabs = [0u8; CLUB_TAB_SIZE * 0x19];
    for k in 0..0x19 {
        tabs[k * CLUB_TAB_SIZE..k * CLUB_TAB_SIZE + 2].copy_from_slice(&0xffffu16.to_le_bytes());
        tabs[k * CLUB_TAB_SIZE + 2..k * CLUB_TAB_SIZE + 4].copy_from_slice(&0u16.to_le_bytes());
    }
    let tab_at = |tabs: &mut [u8; CLUB_TAB_SIZE * 0x19], k: usize| tabs.as_mut_ptr() as usize + k * CLUB_TAB_SIZE;
    let mut stage = 0usize;
    write_u32(0x98ded8, 1);
    let mut club = get_screen_param(0);
    let mut reserves = get_screen_param(1);
    let last_match = get_screen_param(2);
    let next_match = get_screen_param(3);
    let mut tab = get_screen_param(4) as u8;
    let mut colour_text = get_screen_param(5) as u32;
    let mut colour = get_screen_param(6) as u32;
    let mut view = get_screen_param(7) as u32;
    if club == 0 {
        let human = Staff(read_usize(sm + 0x7d6 + read_i16(sm + screen_manager::CURRENT_SLOT) as isize as usize * 0x300));
        if !human.is_null() {
            club = human.club_job().addr();
            if club == 0 {
                club = human.national_job().addr();
            }
            set_screen_param(0, club);
        }
        if club == 0 {
            club_screens_error_box(0x416);
            club_screens_llist(&mut list, false);
            return;
        }
    }
    covered!("tel01_2 club screen");
    if reserves != 0 && exe::human_manager_current_controls_club(club, 0, 1) == 0 && exe::database_is_national_team(club) == 0 {
        set_screen_param(1, 0);
        reserves = 0;
    }
    let (mut d1, mut d2) = ([0u8; 8], [0u8; 8]);
    // simplification: the high halves of the date words pushed to find_fixtures are register garbage in the original
    // (find_fixtures keeps the words); here zero
    if last_match == 0 {
        let weeks = if exe::database_is_national_team(club) != 0 { 0x18 } else { 8 };
        let a = exe::date_sub_weeks(G_CURRENT_DATE, &mut d1, weeks);
        let year = read_u16(a + 2) as u32;
        let b = exe::date_sub_weeks(G_CURRENT_DATE, &mut d2, weeks);
        let day = read_u16(b) as u32;
        exe::fixture_manager_find_fixtures(read_usize(0xae2a58), day, year, read_u16(G_CURRENT_DATE) as u32, read_u16(G_CURRENT_DATE + 2) as u32,
            read_u32(club), list.as_mut_ptr() as usize, u32::MAX, (-2i32) as u32, (reserves == 0) as u32);
        let l = list.as_mut_ptr() as usize;
        if read_u16(l) != 0 {
            write_u32(l + 0xa, read_u32(l + 6));
            let mut f = exe::llist_get_prev(l);
            while f != 0 {
                if read_i8(f + 0x43) > -1 {
                    covered!("tel01_2 last match");
                    set_screen_param(2, club_screens_copy_fixture(f));
                    break;
                }
                f = exe::llist_get_prev(l);
            }
        }
    }
    if next_match == 0 {
        let weeks = if exe::database_is_national_team(club) != 0 { 0x18 } else { 8 };
        let a = exe::date_add_weeks(G_CURRENT_DATE, &mut d1, weeks);
        let year = read_u16(a + 2) as u32;
        let b = exe::date_add_weeks(G_CURRENT_DATE, &mut d2, weeks);
        let day = read_u16(b) as u32;
        exe::fixture_manager_find_fixtures(read_usize(0xae2a58), read_u16(G_CURRENT_DATE) as u32, read_u16(G_CURRENT_DATE + 2) as u32, day, year,
            read_u32(club), list.as_mut_ptr() as usize, u32::MAX, (-2i32) as u32, (reserves == 0) as u32);
        let l = list.as_mut_ptr() as usize;
        if read_u16(l) != 0 {
            write_u32(l + 0xa, read_u32(l + 2));
            let mut f = exe::llist_get_next(l);
            while f != 0 {
                if read_i8(f + 0x43) == -1 {
                    covered!("tel01_2 next match");
                    set_screen_param(3, club_screens_copy_fixture(f));
                    break;
                }
                f = exe::llist_get_next(l);
            }
        }
    }
    if colour_text as u16 == 0 && colour as u16 == 0 {
        exe::database_get_club_kit_colours(club, 1, &mut colour_text, &mut colour);
        set_screen_param(5, (colour_text & 0xffff) as usize);
        set_screen_param(6, (colour & 0xffff) as usize);
    }
    exe::background_set_club_picture(club, 0, 0);
    if view & 0x800 != 0 && exe::human_manager_current_controls_club(club, 0, 1) == 0 {
        view -= 0x800;
        set_screen_param(7, view as usize);
    }
    let (mut name, mut short_name) = ([0u8; 0x12c], [0u8; 0x12c]);
    exe::club_get_national_team_name(club, reserves, name.as_mut_ptr() as usize, 0x12c, 3, 1);
    exe::club_get_national_team_name(club, reserves, short_name.as_mut_ptr() as usize, 0x12c, 1, 1);
    if view & 0x800 != 0 && tab == 1 && exe::human_manager_current_controls_club(club, 0, 1) != 0 {
        let g = read_i8(club + club::SHORT_GENDER_NAME) as i32;
        text(0xde1f64, 0x98e894, g, g, short_name.as_ptr());
    } else {
        copy_text(0xde1f64 as *mut u8, name.as_ptr());
    }
    exe::screenmanager_add_screen_title(sm, colour, colour_text, 0xde1f64usize);
    exe::menubar_build(4, 0);
    let set_tab = |tabs: &mut [u8; CLUB_TAB_SIZE * 0x19], k: usize, kind: u32, link: u32| {
        write_u32(tab_at(tabs, k) + 0xcc, kind);
        write_u32(tab_at(tabs, k) + 0xd0, link);
    };
    let grey = |tabs: &mut [u8; CLUB_TAB_SIZE * 0x19], k: usize| write_u16(tab_at(tabs, k) + 2, read_u16(0xae317e));
    text(tab_at(&mut tabs, 0) + 4, 0x98e88c); // "Squad"
    set_tab(&mut tabs, 0, 2, 1);
    text(tab_at(&mut tabs, 1) + 4, 0x98e880); // "Transfers"
    if exe::database_is_national_team(club) == 0 {
        set_tab(&mut tabs, 1, 2, 2);
    } else {
        if tab == 2 {
            set_screen_param(4, 1);
            tab = 1;
        }
        grey(&mut tabs, 1);
        set_tab(&mut tabs, 1, 0, 0);
    }
    text(tab_at(&mut tabs, 2) + 4, 0x98e874); // "Next Match"
    set_tab(&mut tabs, 2, 2, 3);
    text(tab_at(&mut tabs, 3) + 4, 0x98e868); // "Fixtures"
    set_tab(&mut tabs, 3, 2, 4);
    if exe::human_manager_current_controls_club(club, 0, 1) != 0 && exe::database_is_national_team(club) == 0 {
        text(tab_at(&mut tabs, 4) + 4, 0x98e858); // "Finances & Info"
    } else {
        text(tab_at(&mut tabs, 4) + 4, 0x98e848); // "General Info"
    }
    set_tab(&mut tabs, 4, 2, 5);
    text(tab_at(&mut tabs, 5) + 4, 0x98e840); // "Tactics"
    set_tab(&mut tabs, 5, 3, 0);
    text(tab_at(&mut tabs, 6) + 4, 0x98e834); // "Training"
    if exe::human_manager_current_controls_club(club, 0, 1) != 0 && exe::database_is_national_team(club) == 0 {
        write_u32(tab_at(&mut tabs, 6) + 0xcc, 4);
    } else {
        grey(&mut tabs, 6);
        write_u32(tab_at(&mut tabs, 6) + 0xcc, 0);
    }
    write_u32(tab_at(&mut tabs, 6) + 0xd0, 0);
    text(tab_at(&mut tabs, 7) + 4, 0x98e828); // "Last Match"
    set_tab(&mut tabs, 7, 5, 0);
    let mut record = [0u8; 0x20];
    let mut complete = 0i32;
    if exe::club_history_get_current_league_record(club, record.as_mut_ptr(), &mut complete, &mut stage, 1) != 0 {
        let comp = read_usize(G_CLUB_COMP_TABLE) + read_u32(record.as_ptr() as usize) as usize * 0x6b;
        let division = tab_at(&mut tabs, 8) + 4;
        if (record[0xb] as i8) > 0 {
            let g = read_i8(comp + 0x52) as i32;
            let ordinal = exe::utils_format_ordinal(read_i8(record.as_ptr() as usize + 6) as i32 + 1, -1);
            text(division, 0x98e800, g, g, ordinal, comp + 0x53);
        } else {
            copy_text(division as *mut u8, (comp + 0x38) as *const u8);
            if club_screens_text_length(division as *const u8) > 0x14 {
                copy_text(division as *mut u8, (comp + 0x53) as *const u8);
            }
        }
        if stage != 0 {
            let link = ((read_u32(comp) << 0xb) | (read_u16(stage + 0x3a) as u32 & 0x7ff)) << 0xb;
            set_tab(&mut tabs, 8, 0x7e5, link);
        } else {
            set_tab(&mut tabs, 8, 0x7d0, 0);
        }
    } else {
        text(tab_at(&mut tabs, 8) + 4, 0x98e7f4); // "Division"
        grey(&mut tabs, 8);
        set_tab(&mut tabs, 8, 0, 0);
    }
    text(tab_at(&mut tabs, 9) + 4, 0x98e7c8, 0xad9c64usize); // "History<..>"
    set_tab(&mut tabs, 9, 6, 0);
    let (mut top, mut bottom) = (0x50i32, 0x221i32);
    exe::gui_add_tab_bar(tabs.as_mut_ptr() as usize, 0xa, tab as i8 as i32, &mut top, &mut bottom, 1);
    let draw = |f: usize, top: i32, bottom: i32| {
        let g: unsafe extern "C" fn(i32, i32) = core::mem::transmute(f);
        g(top, bottom)
    };
    match tab as i8 {
        1 => draw(0x456900, top, bottom),
        2 => club_screen_draw_transfers_tab(top, bottom),
        3 => draw(0x45ccf0, top, bottom),
        4 => draw(0x45f380, top, bottom),
        5 => draw(0x460590, top, bottom),
        _ => club_screens_error_box(0x505),
    }
    let menu: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_BUILD_CLUB_SQUAD_NAVIGATION_MENU);
    menu(club, colour, colour_text, 1, 0x21, 0x1f, 1, reserves as u32);
    if exe::add_club_screen_action_button(club, colour, colour_text) == 0 {
        covered!("tel01_2 print menu");
        let button: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
        let add_area: DisplayAddAreaFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
        let add_item: DisplayAddItemFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
        text(0xde1f64, 0x98e5f4); // "Print"
        let mut c = read_u16(0xaebdea);
        if colour as u16 == c {
            c = read_u16(0xae3184);
        }
        let b = button(sm, 0x2c6, 0xf, 0x311, 0x23, 0x30, colour as usize, 0xc, 1, c as usize, 0xde1f64, 0, 0);
        let area = add_area(screens_current_display(sm), 0, 0, 0, 0, 0, 0, 0, 0, 0x130, read_u16(0xae31aa) as u32, b as u32);
        let item = |label: usize, data: u32| {
            text(0xde1f64, label);
            add_item(screens_current_display(sm), 2, 0, 0, 0, 0, 0, 0, 0x10, read_u16(0xae31aa) as u32, read_u16(0xaebdec) as u32, 1, 1,
                read_u16(0xaebdea) as u32, 0xde1f64, 0, 0x22, data, area);
        };
        item(0x98e88c, 1); // "Squad"
        item(0x98e5ec, 2); // "Stats"
        if exe::database_is_national_team(club) == 0 {
            item(0x98e880, 4); // "Transfers"
        } else {
            text(0xde1f64, 0x98e880);
            add_item(screens_current_display(sm), 2, 0, 0, 0, 0, 0, 0, 0x10, read_u16(0xae31aa) as u32, read_u16(0xaebdec) as u32, 0x21, 1,
                read_u16(0xaebdea) as u32, 0xde1f64, 0, 0, 0, area);
        }
        item(0x98e868, 8); // "Fixtures"
        text(0xde1f64, 0x98e5c8, 0xad9c64usize); // "All<..>"
        add_item(screens_current_display(sm), 2, 0, 0, 0, 0, 0, 0, 0x10, read_u16(0xae31aa) as u32, read_u16(0xaebdec) as u32, 1, 1,
            read_u16(0xaebdea) as u32, 0xde1f64, 0, 0x22, 0xf, area);
    }
    let forward = (exe::screenmanager_is_forward_unavailable(sm) == 0) as u32;
    let back = (exe::screenmanager_is_back_unavailable(sm, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(back, forward);
    club_screens_llist(&mut list, false);
}

/// Comparer for `club_screen_build`.
///
/// Through the display list (`compare_by_display_list_with`: the screen manager, the current `Display`, the parameter
/// block, 0xde1f64, the network position, the generator), plus [0x98ded8] and the mouse flag. Only the original runs
/// when the build allocates or reallocates what it keeps: no club yet (param 0), no last or next match copy yet (params
/// 2 and 3), the fixtures tab (4, which allocates), the past transfers (tab 2 without the future view: the array of
/// params 0x15 and 0x16 grows) and an unknown tab (the error box).
pub(crate) unsafe extern "C" fn compare_club_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_CLUB_SCREEN_BUILD);
    let tab = get_screen_param(4) as u8;
    if get_screen_param(0) == 0 || get_screen_param(2) == 0 || get_screen_param(3) == 0 || !matches!(tab, 1 | 2 | 3 | 5)
        || (tab == 2 && get_screen_param(0x12) & 4 == 0) {
        // simplification: allocating and reallocating do not run twice; these cases are proven in Rust mode
        return o();
    }
    compare_by_display_list_with("club_screen_build (club screen)", &[(0x98ded8, 4), (G_MOUSE_LEFT_BUTTON_DOWN, 4)], || o(), || club_screen_build());
}

/// Rule: the club screen's squad tab lists the club's players in the chosen view (positions in two columns, contract,
/// selection, statistics, attributes or other information), sorted and filtered as the menus say, with each player's
/// squad number or selection slot, his status, his name (starred among the club's three most reputed players for a
/// club no human runs) and the view's columns.
///
/// `club_screen_draw_squad_tab(top, bottom)`, from the screen parameters 0 (club), 1 (reserves), 2 and 3 (the last and
/// next fixture), 7 (the view flags), 8 (the sort flags), 9 (the statistics category), 0x18 (the filter flags), 0x19,
/// 0x1a, 0x20..0x22 (the three best known players' ids) and 0xa (the scroll row). The pieces:
/// `club_screen_squad_tab_menus` (the View, Sort By, Competitions, Attributes and Filter menus, the clear button and the
/// title), `club_screen_squad_tab_selection` (the squad numbers or the team sheet row, and the list), the header and the
/// list area (`club_screen_squad_tab_header`) and the rows (`club_screen_squad_tab_rows`). Last, the list area's
/// scroll row from param 0xa (`screenmanager_set_area_scroll_row`) and the area written to param 0xb.
///
/// The cm0102-1080.exe changes this routine: the positions view's area gets 12 columns with the weights at 0x601eeb
/// and three players per row. The copy reads those bytes from the exe (0x459a22, 0x459a3b, 0x45c018), so it builds what
/// each exe builds.
/// Original: `club_screen_draw_squad_tab` (0x456900, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screen_draw_squad_tab(top: i32, bottom: i32) {
    covered!("tel01_3 squad tab");
    let sm = G_SCREEN_MANAGER;
    let mut t = SquadTab {
        sm,
        club: 0,
        reserves: 0,
        last_match: 0,
        next_match: 0,
        view: 0,
        sort: 0,
        category: 0,
        filter: 0,
        p19: 0,
        p1a: 0,
        best: [0; 3],
        human: 0,
        top,
        bottom,
        bar: 0,
        title: [0; 0xc8],
        list: [0; 0x20],
        from_record: false,
        attributes: 0,
        ebp: 0,
    };
    club_screens_llist(&mut t.list, true);
    let mut w = SquadWeights {
        two: [6, 5, 0x1e, 0x10, 6, 5, 0x1e, 0x10],
        contract: [5, 5, 0x18, 0xe, 0xe, 0xf, 0xb, 0, 0, 0, 0, 0, 0xa],
        stats: [5, 5, 0x18, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0xa],
        attributes: [5, 5, 0x18, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0xa],
        other: [5, 5, 0x18, 8, 6, 6, 6, 0xa, 0xc, 6, 0, 0, 0xa],
        selection: [5, 5, 0x18, 0xe, 0xa, 0xc, 6, 6, 6, 0, 0, 0, 0xa],
    };
    t.human = read_usize(sm + 0x7d6 + read_i16(sm + screen_manager::CURRENT_SLOT) as isize as usize * 0x300);
    t.club = get_screen_param(0);
    t.reserves = get_screen_param(1) as u32;
    t.last_match = get_screen_param(2);
    t.next_match = get_screen_param(3);
    t.view = get_screen_param(7) as u32;
    t.sort = get_screen_param(8) as u32;
    t.category = get_screen_param(9) as u8;
    t.filter = get_screen_param(0x18) as u32;
    t.p19 = get_screen_param(0x19) as u8;
    t.p1a = get_screen_param(0x1a) as u8;
    t.best = [get_screen_param(0x20) as u32, get_screen_param(0x21) as u32, get_screen_param(0x22) as u32];
    if exe::human_manager_current_controls_club(t.club, 0, 1) != 0 {
        let v = club_screens_squad_view_settings();
        write_u32(v, t.view);
        write_u32(v + 4, t.sort);
        write_u8(v + 0xc, t.category);
        write_u32(v + 8, t.filter);
        write_u8(v + 0xd, t.p19);
        write_u8(v + 0xe, t.p1a);
        exe::human_manager_select_tactic_slot(read_usize(0xb63c98), t.reserves as i32, t.club, 0usize);
    }
    club_screen_squad_tab_menus(&mut t);
    club_screen_squad_tab_selection(&mut t);
    let area = club_screen_squad_tab_header(&mut t, &mut w);
    club_screen_squad_tab_rows(&mut t, area);
    exe::screenmanager_set_area_scroll_row(sm, area, get_screen_param(0xa) as u32);
    set_screen_param(0xb, area as i16 as i32 as u32 as usize);
    club_screens_llist(&mut t.list, false);
}

/// Comparer for `club_screen_draw_squad_tab`.
///
/// Through the display list (`compare_by_display_list_with`), plus the attribute tables the attributes view sorts in
/// place (0x98de84..0x98ded8), the human's squad view block and the last match's team sheet record (0xad9db8, 0xd7a
/// bytes). What it also does outside the list runs twice and is idempotent: the tactic slot, the squad numbers pruned,
/// each player's training caught up to the week, the team sheet cleared of gone players.
pub(crate) unsafe extern "C" fn compare_club_screen_draw_squad_tab(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(IDX_CLUB_SCREEN_DRAW_SQUAD_TAB);
    let mut extra = vec![(0x98de84usize, 0x54usize), (0xad9db8, 0xd7a)];
    if exe::human_manager_current_controls_club(get_screen_param(0), 0, 1) != 0 {
        extra.push((club_screens_squad_view_settings(), 0xf));
    }
    compare_by_display_list_with("club_screen_draw_squad_tab (club screen, squad tab)", &extra, || o(top, bottom), || club_screen_draw_squad_tab(top, bottom));
}

/// Draws the next match tab of the club screen (screen params: 0 the club, 1 the reserves/'B' flag, 3 the next fixture)
/// between `top` and `bottom`.
///
/// Without a fixture: only the message "There are no forthcoming matches to be played." (reserves: "...reserve
/// matches...", or "...'B' international matches..." for a national team; `FN_GUI_ADD_TEXT_AREA`, as many lines of font
/// 3 as fit in `bottom - top - 0x10`).
///
/// With a fixture: the arrows row (x 0x6e..0x168): the previous fixtures of the club between the match date minus 24
/// weeks (national team) or 8 weeks and the day before (`FIXTURE_MANAGER::find_fixtures`, type −2, side 1 unless
/// reserves) give "<< Date" as button 0x26 when the last of them is still unplayed (home goals 0xff), the next ones from
/// the day after to the match date plus 24/8 weeks give "Date >>" as button 0x27 when there is any; otherwise grey text.
/// The fixtures from today to the match date are counted: the unplayed ones and, among them, those whose competition is
/// in the same group (`NEXT_MATCH_COMP_GROUP`) as the match's. Then (x 0x212..0x30c) "Progress" (button 0x1d) when the
/// fixture resolves to a league (`comp_fixture_resolve_league_clubs`) that is not a reserve or minor competition, and
/// "Past Meetings": a popup (button with a list of "Against <club>" items, id 0x1e) against the opponent and against the
/// human manager's national team (for a 'B' national team, when it is of another nation) or his club (when it is not
/// this club nor its linked B team); otherwise a plain button 0x1e. The title row: the opponent with "(Home)"/"(Away)"
/// (`comp_util_get_fixture_venue_side`; neutral: the name alone), button 0x7d5 to the opponent.
///
/// The table (two columns, 3:5) is laid out twice: the first pass only counts the rows, the second adds them, with the
/// right edge at 0x30a, or 0x2f6 (room for the scroll bar) when the first pass counted more rows than fit. Rows: the
/// round (`comp_util_format_fixture_round_label`, orange, a link 0x7e5 to the competition when it is a real current
/// competition: in the table, not the reserve key, not a friendly's own and current on the match date) and its underline;
/// Date (long date, with "(today)", "(tomorrow)" or "(N days)"); Venue (stadium, city); Aggregate (best of three: the
/// series; or the aggregate score when there is one); Match Rules (the foreign, non-EU or over-age squad limit of
/// `transfer_manager_get_selection_limit`, else "No player restrictions"); the tie rules of `fixture::TIE_RULES` and
/// `fixture::FLAGS` (away goals after 90 minutes, extra time or golden goal, away goals after extra time, penalties,
/// US-style shoot-out, replay, the seeded home or away team going through; "still level" once an earlier rule was
/// listed); the substitutes (named, used); Last Meeting (the latest played meeting of the two clubs before today,
/// `FIXTURE_MANAGER::scan_fixtures`: result, round and date with links, wrapped; or "-"); Weather or Weather Forecast
/// (known when the match is within 2 weeks); Pitch Condition (match day only); the referee with his strictness (Harsh 20,
/// Strict over 14, Lenient under 6; plain names with the database names language set). Outside the reserves, the team
/// news of both teams ("<team> News" header once per team), each squad player at most once, not on loan out, needing no
/// work permit and in the right squad (national team: not bit 8, else not bit 2 of `staff::PLAYING_SQUAD`), in this
/// order: away on international duty, ineligible, suspended (`Discipline::is_player_banned` with the same-group count),
/// injured beyond the match date, doubtful (back in light training but more than 3 days out), set for debut (joined
/// under 100 days ago, no appearances for the club, not a friendly, this is the next unplayed fixture, and a first team,
/// rotation or backup squad status).
///
/// Defect of the original (harmless): the first pass reads the selection limit before it is filled (stack leftover); the
/// rules block always counts exactly one row, so the layout does not change. The copy's frame starts zeroed.
/// Original: `club_screen_draw_next_match_tab` (0x45ccf0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screen_draw_next_match_tab(top: i32, bottom: i32) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let message: unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
        core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let add_item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let cell_button: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let cell_text: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let button_item: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    // thiscall helpers (checked at the call sites: ret = 4 x the arguments after this)
    let llist_drop: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let llist_clear: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_CLEAR);
    let text_clear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_TEXT_CLEAR); // Text::clear
    // Text::draw_wrapped(slot, left, right, &line, font, colour, style, flags), ret 0x20
    let short_date: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_GET_SHORT_DATE_STRING); // Date::get_short_date_string
    let find_fixtures: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
        core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURES);
    let scan_fixtures: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize, u32, u32, u32, u32) -> u32 =
        core::mem::transmute(FN_FIXTURE_MANAGER_SCAN_FIXTURES);
    let resolve_league: unsafe extern "C" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_COMP_FIXTURE_RESOLVE_LEAGUE_CLUBS); // comp_fixture_resolve_league_clubs
    // comp_is_reserve_or_minor_comp is stdcall(comp) ret 4; called with ecx = G_RESERVE_COMP (same stack as thiscall)
    let round_label: unsafe extern "C" fn(usize, u32, usize, u32) -> i32 = core::mem::transmute(FN_COMP_UTIL_FORMAT_FIXTURE_ROUND_LABEL);
    let comp_current: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_COMP_CURRENT_ON_DATE); // Friendly::is_comp_current_on_date
    // transfer_manager_get_selection_limit is stdcall(fixture, out) ret 8 (ecx unused)
    // weather_format_description and pitch_condition_text are stdcall(out, ..) ret 8, called with ecx = G_WEATHER
    let pitch_text: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_PITCH_CONDITION_TEXT); // pitch_condition_text
    let injury_text: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_INJURY_DESCRIBE_PLAYER_INJURY); // Injury::describe_player_injury
    // player_stats_get_apps_for_club is stdcall(staff, club) ret 8, called with ecx = G_PLAYER_STATS
    // database_format_name_from_strings: the caller pushes a 7th dword ([staff+0]); cdecl, so it is passed too

    let sm = SCREEN_MANAGER;
    const MAX: usize = usize::MAX;
    const BUF: usize = NEXT_MATCH_TEXT;
    // the original's frame ([esp+N] after the four registers): the objects passed by address in the same places
    // (+0x14 the row, +0x20 the Text, +0x30 the columns 3:5, +0x3c the fixture date, +0x4c the match date, +0x5c and
    // +0xb4 the fixture lists, +0x6c the selection limit, +0x84 the weather, +0xfc the shown players, +0x158 the name)
    let mut frame = [0u32; 0x400 / 4];
    let s = frame.as_mut_ptr() as usize;
    let at = |o: usize| s + o;
    let gender = |c: usize| read_i8(c + club::SHORT_GENDER_NAME) as i32;
    let pale = colour_word(G_GUI_COLOUR_PALE_GREY);
    let grey = colour_word(G_GUI_COLOUR_GREY);
    let yellow = colour_word(G_GUI_COLOUR_YELLOW);
    let orange = colour_word(NEXT_MATCH_COLOUR_ORANGE);

    write_u8(at(0x30), 3);
    write_u8(at(0x31), 5);
    write_u16(at(0x38), read_u16(G_GUI_COLOUR_YELLOW));
    exe::new_llist(at(0xb4));
    exe::new_llist(at(0x5c));
    exe::new_text((at(0x20)) as usize as *mut usize, 0);
    let club = exe::screenmanager_get_screen_param(sm, 0);
    let reserves = exe::screenmanager_get_screen_param(sm, 1);
    let fixture = Fixture(exe::screenmanager_get_screen_param(sm, 3));
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    let manager = Staff(read_usize((sm as isize + NEXT_MATCH_RECORD_KEY as isize + slot * 0x300) as usize));

    if fixture.is_null() {
        let lines = ((bottom - top - 0x10) / exe::font_get_height((3) as u32 as i32)) as u8;
        let text = if reserves != 0 {
            if exe::database_is_national_team(club) != 0 { 0x98f7f0 } else { 0x98f7b8 } // 'B' international / reserve
        } else {
            0x98f788
        };
        lang(BUF, text);
        message(0x6e, top as usize, 0x30c, bottom as usize, 2, 0, lines as usize, 3, pale, BUF, 0);
    } else {
        covered!("tel02_1 next match tab");
        let fm = read_usize(G_FIXTURE_MANAGER);
        let club_id = read_u32(club);
        let arrows = add_area(current_human_display(), 0x6e, top as usize, 0x168, (top + 0x14) as usize, 2, 0, 1, 0, 1, 0, MAX);
        exe::new_date(at(0x3c), fixture.day() as u32, fixture.year() as u32);
        let weeks = if exe::database_is_national_team(club) != 0 { 0x18 } else { 8 };
        let side = (reserves == 0) as u32;
        write_u8(at(0x88), side as u8);
        // the previous fixtures: from the match date minus the weeks to the day before
        let y1 = read_u16(exe::date_sub_days(at(0x3c), at(0x9c), 1) + 2) as u32;
        let d1 = read_u16(exe::date_sub_days(at(0x3c), at(0xa4), 1)) as u32;
        let yw = read_u16(exe::date_sub_weeks(at(0x3c), (at(0x94)) as usize as *mut [u8; 8], weeks) + 2) as u32;
        let dw = read_u16(exe::date_sub_weeks(at(0x3c), (at(0x8c)) as usize as *mut [u8; 8], weeks)) as u32;
        find_fixtures(fm, dw, yw, d1, y1, club_id, at(0x5c), u32::MAX, 0xffff_fffe, side);
        lang(at(0x1e4), 0x988078); // "Date"
        format(BUF, 0x988090, at(0x1e4)); // "<< %s"
        write_u32(at(0x5c + llist::CURSOR), read_u32(at(0x5c + llist::TAIL)));
        let last = Fixture(if read_u16(at(0x5c)) != 0 { exe::llist_get_prev(at(0x5c)) } else { 0 });
        if !last.is_null() && (last.home_goals() as u8) == 0xff {
            cell_button(sm, arrows, 0, 0, 0x30, grey, 0xc, 1, pale, BUF, 0x26, 0);
        } else {
            cell_text(sm, arrows, 0, 0, 0x30, grey, 0x2c, 1, pale, BUF);
        }
        llist_clear(at(0x5c));
        // the next fixtures: from the day after to the match date plus the weeks
        let yw = read_u16(exe::date_add_weeks(at(0x3c), (at(0x9c)) as usize as *mut [u8; 8], weeks) + 2) as u32;
        let dw = read_u16(exe::date_add_weeks(at(0x3c), (at(0xa4)) as usize as *mut [u8; 8], weeks)) as u32;
        let y1 = read_u16(exe::date_add_days(at(0x3c), at(0x94), 1) + 2) as u32;
        let d1 = read_u16(exe::date_add_days(at(0x3c), at(0x8c), 1)) as u32;
        find_fixtures(fm, d1, y1, dw, yw, club_id, at(0x5c), u32::MAX, 0xffff_fffe, side);
        lang(at(0x1e4), 0x988078); // "Date"
        format(BUF, 0x988088, at(0x1e4)); // "%s >>"
        if read_u16(at(0x5c)) != 0 {
            cell_button(sm, arrows, 1, 0, 0x30, grey, 0xc, 1, pale, BUF, 0x27, 0);
        } else {
            cell_text(sm, arrows, 1, 0, 0x30, grey, 0x2c, 1, pale, BUF);
        }
        llist_clear(at(0x5c));
        // the fixtures from today to the match date: the unplayed ones and those of the same competition group
        find_fixtures(fm, read_u16(G_CURRENT_DATE) as u32, read_u16(G_CURRENT_DATE_YEAR) as u32, read_u16(at(0x3c)) as u32,
            read_u16(at(0x3e)) as u32, club_id, at(0x5c), u32::MAX, 0xffff_fffe, side);
        write_u32(at(0x5c + llist::CURSOR), read_u32(at(0x5c + llist::HEAD)));
        let mut same_group = 0u32;
        let mut unplayed = 0u32;
        loop {
            let it = exe::llist_get_next(at(0x5c));
            if it == 0 {
                break;
            }
            if read_u8(it + fixture::HOME_GOALS) != 0xff {
                continue;
            }
            unplayed += 1;
            let a = read_i32(it + fixture::COMP_ID);
            let b = fixture.comp_id();
            if a >= 0 && b >= 0 {
                let table = read_usize(G_COMP_OBJECT_TABLE);
                let g = read_u8(read_usize(table + a as usize * 4) + NEXT_MATCH_COMP_GROUP);
                if g != 0xff && g == read_u8(read_usize(table + b as usize * 4) + NEXT_MATCH_COMP_GROUP) {
                    same_group += 1;
                }
            }
        }
        llist_clear(at(0x5c));

        let links = add_area(current_human_display(), 0x212, top as usize, 0x30c, (top + 0x14) as usize, 2, 0, 1, 0, 1, 0, MAX);
        if resolve_league(fixture.addr(), at(0xf8), at(0xf4), at(0xf0)) != 0
            && exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), fixture.competition().addr()) == 0
        {
            lang(BUF, 0x98f77c); // "Progress"
            cell_button(sm, links, 0, 0, 0x30, grey, 0xc, 1, pale, BUF, 0x1d, 0);
        }
        lang(BUF, 0x98f76c); // "Past Meetings"
        let home = fixture.home_club().addr();
        let opponent = if club == home { fixture.away_club().addr() } else { home };
        // the "Past Meetings" popup: against the opponent and against `other` when it is someone else
        let popup = |other: usize| {
            let b = cell_button(sm, links, 1, 0, 0x30, grey, 0xc, 1, pale, BUF, 0, 0);
            let list = add_area(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x130, colour_word(NEXT_MATCH_COLOUR_POPUP), b);
            lang(BUF, 0x98f74c, gender(opponent), gender(opponent), opponent + club::SHORT_NAME); // "Against {}<%s - Club Name>{}"
            add_item(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, colour_word(NEXT_MATCH_COLOUR_POPUP), yellow, 1, 1,
                colour_word(G_GUI_COLOUR_BLACK), BUF, 0, 0x1e, 0, list);
            if other != opponent {
                lang(BUF, 0x98f74c, gender(other), gender(other), other + club::SHORT_NAME);
                add_item(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, colour_word(NEXT_MATCH_COLOUR_POPUP), yellow, 1, 1,
                    colour_word(G_GUI_COLOUR_BLACK), BUF, 0, 0x1e, other, list);
            }
        };
        let mut shown = false;
        if exe::database_is_national_team(club) != 0 && exe::club_is_national_team(club) == 0 && !manager.is_null() {
            let national = manager.national_job().addr();
            if national != 0 && read_usize(national + club::NATION) != read_usize(club + club::NATION) {
                popup(national);
                shown = true;
            }
        }
        if !shown {
            let job = if !manager.is_null() { manager.club_job().addr() } else { 0 };
            if exe::database_is_national_team(club) == 0 && job != 0 && job != club && job != exe::club_get_linked_b_team(club, (0) as usize as *mut u32, 1) {
                popup(job);
            } else {
                cell_button(sm, links, 1, 0, 0x30, grey, 0xc, 1, pale, BUF, 0x1e, 0);
            }
        }

        // the title: the opponent and the venue
        let y = top + 0x19;
        exe::new_date(at(0x4c), fixture.day() as u32, fixture.year() as u32);
        match exe::comp_util_get_fixture_venue_side(club, fixture) {
            1 => { lang(BUF, 0x98f734, gender(opponent), gender(opponent), opponent + club::SHORT_NAME); } // "(Home)"
            2 => { lang(BUF, 0x98f71c, gender(opponent), gender(opponent), opponent + club::SHORT_NAME); } // "(Away)"
            _ => next_match_copy_text(BUF, opponent + club::SHORT_NAME),
        }
        button_item(sm, 0x6e, y as usize, 0x30c, (y + 0x23) as usize, 2, 0, 0xc, 3, yellow, BUF, 0x7d5, opponent);
        let y = y + 0x28;
        let lines = ((bottom - y - 0x10) / exe::font_get_height((3) as u32 as i32)) as u8;
        write_u8(at(0x48), lines);
        let table = add_area(current_human_display(), 0x6e, y as usize, 0x30c, bottom as usize, 2, at(0x30), read_u32(at(0x48)) as usize, 0, 2, 0, MAX);
        let visible = lines as i32;

        let row = || read_i32(at(0x14));
        let next_row = || write_i32(at(0x14), read_i32(at(0x14)) + 1);
        let label = |t: usize| { cell_text(sm, table, 0, row() as usize, 1, 0, 1, 3, pale, t); };
        let value = |t: usize| { cell_text(sm, table, 1, row() as usize, 1, 0, 1, 2, yellow, t); };
        // a link to a fixture's competition: in the table, not the reserve key, not a friendly's own, current on its date
        let real_comp = |f: usize, date: usize| {
            let comp = read_usize(f + fixture::COMPETITION);
            comp != 0
                && read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(f + fixture::COMP_ID) as usize * 4) != 0
                && read_u32(comp) != read_u32(KEY_CLUB_COMP_RESERVE)
                && exe::friendly_is_own_comp_record(read_usize(G_FRIENDLY), comp) == 0
                && {
                    exe::new_date(date, read_u16(f + fixture::DAY) as u32, read_u16(f + fixture::YEAR) as u32);
                    comp_current(read_usize(G_FRIENDLY), read_usize(f + fixture::COMPETITION), date) != 0
                }
        };
        let match_day = || read_u16(G_CURRENT_DATE_YEAR) == read_u16(at(0x4e)) && read_u16(G_CURRENT_DATE) == read_u16(at(0x4c));
        // a date is past the match date (years unsigned, days signed)
        let after_match = |d: usize| {
            let (day, year) = (read_u16(d), read_u16(d + 2));
            year > read_u16(at(0x4e)) || (year == read_u16(at(0x4e)) && (day as i16) > read_i16(at(0x4c)))
        };

        let mut rows_counted = 0i32;
        let mut round_right = 0i32;
        for pass in 0..2 {
            let draw = pass != 0;
            let width: i32 = if rows_counted <= visible { 0x30a } else { 0x2f6 };
            write_i32(at(0x14), 0);
            // the round and its underline
            if draw {
                round_label(fixture.addr(), 2, BUF, 0x7d0);
                round_right = exe::font_text_width((3) as u32 as usize, (BUF) as usize as *const u8) + 0x73;
                if real_comp(fixture.addr(), at(0x130)) {
                    let id = ((fixture.comp_id() as u32) << 11 | ((fixture.round_prefix() as u16) as u32 & 0x7ff)) << 11
                        | ((fixture.round_name() as u16) as u32 & 0x7ff);
                    add_item(current_human_display(), 2, 0x73, 0, round_right as usize, 0, 0, row() as usize, 1, 0, 0, 1, 3, orange, BUF, 0, 0x7e5, id as usize, table);
                } else {
                    add_item(current_human_display(), 1, 0x73, 0, round_right as usize, 0, 0, row() as usize, 1, 0, 0, 1, 3, orange, BUF, 0, 0, 0, table);
                }
            }
            next_row();
            if draw {
                add_item(current_human_display(), 1, 0x73, 0, round_right as usize, 0, 0, row() as usize, 0xc400, orange, 0, 0, 0, 0, 0, 0, 0, 0, table);
            }
            next_row();
            // Date
            if draw {
                lang(BUF, 0x98f714); // "  Date"
                label(BUF);
                exe::date_get_long_date_string_no_year(at(0x4c), at(0x1e4), 0xc8, 0xff);
                if match_day() {
                    lang(BUF, 0x98f6fc, 0x64, at(0x1e4)); // "(today)"
                } else if exe::date_days_since(at(0x4c), read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP)) == 1 {
                    lang(BUF, 0x98f6e0, 0x64, at(0x1e4)); // "(tomorrow)"
                } else {
                    let d = exe::date_days_since(at(0x4c), read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP));
                    lang(BUF, 0x98f6bc, 0x64, at(0x1e4), d); // "(<%d - days> days)"
                }
                value(BUF);
            }
            next_row();
            // Venue
            if draw {
                lang(BUF, 0x98f6b4); // "  Venue"
                label(BUF);
                let stadium = fixture.stadium();
                if stadium != 0 {
                    let city = read_usize(stadium + stadium::CITY);
                    if city != 0 {
                        format(BUF, 0x98f6ac, stadium + stadium::NAME, city + NEXT_MATCH_CITY_NAME); // "%s, %s"
                    } else {
                        next_match_copy_text(BUF, stadium + stadium::NAME);
                    }
                } else {
                    lang(BUF, 0x98f68c, NEXT_MATCH_EMPTY_TEXT); // "Unknown<%s - COMMENT - venue>"
                }
                value(BUF);
            }
            next_row();
            // Aggregate: the best of three series, or the aggregate score
            if (fixture.tie_rules() as u8) & 0x10 != 0 {
                if draw {
                    lang(BUF, 0x98f680); // "  Aggregate"
                    label(BUF);
                    let (h, a) = ((fixture.home_result_mark() as u8), (fixture.away_result_mark() as u8));
                    let leader = if h == 1 && a == 0 { fixture::HOME_CLUB } else if h == 0 && a == 1 { fixture::AWAY_CLUB } else { 0 };
                    if h == 1 && a == 1 {
                        lang(BUF, 0x98f664); // "1-1 in best of three games"
                    } else if leader != 0 {
                        let c = read_usize(fixture.addr() + leader);
                        lang(BUF, 0x98f634, gender(c), gender(c), c + club::SHORT_NAME); // "... lead 1-0 in best of three games"
                    } else {
                        lang(BUF, 0x98f610); // "Best of three games to be played"
                    }
                    value(BUF);
                }
                next_row();
            } else if fixture.home_aggregate() > -1 {
                if draw {
                    lang(BUF, 0x98f680); // "  Aggregate"
                    label(BUF);
                    let h = fixture.home_aggregate() as i32;
                    let a = fixture.away_aggregate() as i32;
                    if h == a {
                        lang(BUF, 0x98f5dc, h, a); // "<%d>-<%d> on aggregate"
                    } else if h > a {
                        let c = fixture.home_club().addr();
                        lang(BUF, 0x98f594, gender(c), gender(c), c + club::SHORT_NAME, h, a); // "... lead <%d>-<%d> on aggregate"
                    } else {
                        let c = fixture.away_club().addr();
                        lang(BUF, 0x98f594, gender(c), gender(c), c + club::SHORT_NAME, a, h);
                    }
                    value(BUF);
                }
                next_row();
            }
            // Match Rules: the squad limit
            if draw {
                lang(BUF, 0x98f584); // "  Match Rules"
                label(BUF);
                exe::transfer_manager_get_selection_limit(read_usize(G_TRANSFER_MANAGER), fixture, (at(0x6c)) as usize as *mut u8);
            }
            let first_rule = row();
            if read_u8(at(0x6c)) == 2 {
                if draw {
                    lang(BUF, 0x98f550, read_i8(at(0x71)) as i32); // "Max <%d> foreign players in the match squad"
                    value(BUF);
                }
                next_row();
            }
            if read_u8(at(0x6c)) == 1 {
                if draw {
                    if row() > first_rule {
                        cell_text(sm, table, 0, row() as usize, 1, 0, 1, 3, pale, 0);
                    }
                    lang(BUF, 0x98f51c, read_i8(at(0x71)) as i32); // "Max <%d> non-EU players in the match squad"
                    value(BUF);
                }
                next_row();
            }
            if read_u8(at(0x6c)) == 3 {
                if draw {
                    if row() > first_rule {
                        cell_text(sm, table, 0, row() as usize, 1, 0, 1, 3, pale, 0);
                    }
                    lang(BUF, 0x98f4dc, read_i8(at(0x71)) as i32, read_i8(at(0x6d)) as i32); // "Max <%d> players over <%d> yrs ..."
                    value(BUF);
                }
                next_row();
            }
            if row() == first_rule {
                if draw {
                    lang(BUF, 0x98f4c4); // "No player restrictions"
                    value(BUF);
                }
                next_row();
            }
            // the tie rules; once one is listed, the later ones say "still level"
            let rules = || fixture.tie_rules();
            let mut listed = 0u32;
            let rule = |text: usize| {
                if draw {
                    lang(BUF, text);
                    value(BUF);
                }
                next_row();
            };
            if exe::comp_util_get_round_variant(fixture.round_name() as i32) == 4 && rules() & 0x4004 == 0 {
                rule(0x98f488); // "Away goals used if scores are level after ninety minutes"
            }
            if rules() & 2 != 0 {
                rule(if rules() & 0x80 != 0 { 0x98f440 } else { 0x98f404 }); // golden goal / extra time
                listed = 1;
            }
            if exe::comp_util_get_round_variant(fixture.round_name() as i32) == 4 && rules() & 0x4000 != 0 && rules() & 2 != 0 {
                rule(0x98f3cc); // "Away goals used if scores are level after extra time"
            }
            if rules() & 1 != 0 {
                rule(if listed != 0 { 0x98f3a0 } else { 0x98f364 }); // penalty shoot-out
                listed += 1;
            }
            if rules() & 0x20 != 0 {
                rule(if listed != 0 { 0x98f334 } else { 0x98f2f8 }); // US-style shoot-out
                listed += 1;
            }
            if fixture.flags() & 4 != 0 {
                rule(if listed != 0 { 0x98f2cc } else { 0x98f294 }); // replay
                listed += 1;
            }
            for (bit, team) in [(0x800u16, fixture::HOME_CLUB), (0x1000, fixture::AWAY_CLUB)] {
                if rules() & bit != 0 {
                    if draw {
                        let c = read_usize(fixture.addr() + team);
                        let text = if listed != 0 { 0x98f24c } else { 0x98f1f8 }; // the seeded team wins
                        lang(BUF, text, gender(c), gender(c), c + club::SHORT_NAME);
                        value(BUF);
                    }
                    next_row();
                    if bit == 0x800 {
                        listed += 1; // tel02_1 the away seeding does not count as listed
                    }
                }
            }
            if draw {
                let w = fixture.squad_rules() as u32;
                lang(BUF, 0x98f1bc, w & 0xf, (w >> 4) & 0xf); // "<%d> subs named, maximum <%d> used"
                value(BUF);
            }
            next_row();
            // Last Meeting
            if draw {
                lang(BUF, 0x98f1ac); // "  Last Meeting"
                label(BUF);
                write_u32(at(0xd8), read_u32(G_CURRENT_DATE));
                write_u16(at(0x82), read_u16(G_CURRENT_DATE_YEAR));
                write_u8(at(0x57), 2);
                scan_fixtures(fm, at(0x82), at(0xd8), at(0x57), 0, u32::MAX, u32::MAX, u32::MAX, club_id, read_u32(opponent),
                    u32::MAX, u32::MAX, u32::MAX, 1, at(0xb4), 1, 0xffff_fffe, read_u32(at(0x88)), 1);
                let mut it = 0;
                if read_u16(at(0xb4)) != 0 {
                    text_clear(at(0x20));
                    write_u32(at(0xb4 + llist::CURSOR), read_u32(at(0xb4 + llist::HEAD)));
                    loop {
                        it = exe::llist_get_next(at(0xb4));
                        if it == 0 || read_u8(it + fixture::HOME_GOALS) != 0xfe {
                            break;
                        }
                    }
                }
                if it != 0 {
                    let (a, b) = if read_i8(it + fixture::HOME_GOALS_EXTRA_TIME) > -1 {
                        (read_i8(it + fixture::HOME_GOALS_EXTRA_TIME), read_i8(it + fixture::AWAY_GOALS_EXTRA_TIME))
                    } else {
                        (read_i8(it + fixture::HOME_GOALS), read_i8(it + fixture::AWAY_GOALS))
                    };
                    format(at(0xdc), 0x98c114, a as i32, b as i32); // "%d-%d"
                    round_label(it, 3, at(0x324), 0xc8);
                    exe::new_date(at(0x148), read_u16(it + fixture::DAY) as u32, read_u16(it + fixture::YEAR) as u32);
                    short_date(at(0x148), at(0x1d0), 0x14, 0xff);
                    let (h, w) = (read_usize(it + fixture::HOME_CLUB), read_usize(it + fixture::AWAY_CLUB));
                    lang(BUF, 0x98f154, gender(h), gender(h), gender(w), gender(w), 0x64, h + club::SHORT_NAME, at(0xdc),
                        w + club::SHORT_NAME, at(0x324), at(0x1d0)); // "<home> <result> <away>, <fixture> <date>"
                    (exe::text_set_string(at(0x20), (BUF) as usize as *const u8) as i32);
                    exe::text_add_link(at(0x20), (h + club::SHORT_NAME) as usize as *const u8, 0x7d5, h);
                    if (exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), (read_u32(it + fixture::MATCH_STATS_SLOT)) as u32 as i32, (read_u32(it + fixture::HOME_CLUB_ID)) as u32 as i32, (read_u32(it + fixture::AWAY_CLUB_ID)) as u32 as i32, (read_u16(it + fixture::DAY) as u32) as u32 as i32, (read_u16(it + fixture::YEAR) as u32) as u32 as i32) as u32) != 0
                    {
                        exe::text_add_link(at(0x20), (at(0xdc)) as usize as *const u8, 0x7d3, read_usize(it + fixture::MATCH_STATS_SLOT));
                    }
                    exe::text_add_link(at(0x20), (w + club::SHORT_NAME) as usize as *const u8, 0x7d5, w);
                    if real_comp(it, at(0x140)) {
                        exe::text_add_link(at(0x20), (at(0x324)) as usize as *const u8, 0x7d0, read_usize(it + fixture::COMPETITION));
                    }
                    exe::text_draw_wrapped(at(0x20), (table) as usize as u32, (width * 3 - 0x150) / 8 + 0x71, width, (at(0x14)) as usize as *mut i32, 2, (yellow) as usize as u32, 2, 0);
                } else {
                    value(0x98ba0c); // "-"
                }
            }
            next_row();
            // Weather or Weather Forecast: known within two weeks of the match
            if draw {
                lang(BUF, if match_day() { 0x98f148 } else { 0x98f134 });
                label(BUF);
                if after_match(exe::date_add_weeks(G_CURRENT_DATE, (at(0x150)) as usize as *mut [u8; 8], 2)) {
                    exe::weather_get_match_weather(read_usize(G_WEATHER), at(0x84), fixture);
                    exe::weather_format_description(read_usize(G_WEATHER), BUF, at(0x84));
                } else {
                    lang(BUF, 0x98f110, NEXT_MATCH_EMPTY_TEXT); // "Unknown<%s - COMMENT - conditions>"
                }
                value(BUF);
            }
            next_row();
            // Pitch Condition: on match day
            if match_day() {
                if draw {
                    lang(BUF, 0x98f0fc); // "  Pitch Condition"
                    label(BUF);
                    if after_match(exe::date_add_weeks(G_CURRENT_DATE, (at(0x138)) as usize as *mut [u8; 8], 6)) {
                        exe::weather_get_match_weather(read_usize(G_WEATHER), at(0x84), fixture);
                        write_u8(at(0xc8), exe::weather_get_pitch_condition(read_usize(G_WEATHER), at(0x84), fixture));
                        pitch_text(read_usize(G_WEATHER), BUF, read_u32(at(0xc8)));
                    } else {
                        lang(BUF, 0x98f110, NEXT_MATCH_EMPTY_TEXT);
                    }
                    value(BUF);
                }
                next_row();
            }
            // Referee
            if fixture.referee() != -1 {
                if draw {
                    lang(BUF, 0x98f0f0); // "  Referee"
                    label(BUF);
                    let r = (read_usize(G_OFFICIAL_TABLE) as isize + fixture.referee() as isize * NEXT_MATCH_OFFICIAL_SIZE as isize) as usize;
                    let text = if read_u32(G_DATABASE_NAMES_LANGUAGE) != 0 {
                        0x98f0b8 // "%s %s"
                    } else {
                        let d = read_i8(r + official::DISCIPLINE);
                        if d == 0x14 { 0x98f0e0 } else if d > 0xe { 0x98f0d0 } else if d >= 6 { 0x98f0b8 } else { 0x98f0c0 } // Harsh, Strict, -, Lenient
                    };
                    format(BUF, text, read_usize(r + official::FIRST_NAME), read_usize(r + official::SECOND_NAME));
                    value(BUF);
                }
                next_row();
            }
            // the team news of both teams
            if reserves == 0 {
                for side_k in 0..2 {
                    let team = read_usize(fixture.addr() + if side_k != 0 { fixture::AWAY_CLUB } else { fixture::HOME_CLUB });
                    let mut header = false;
                    core::ptr::write_bytes(at(0xfc) as *mut u8, 0, 0x32);
                    for case in 0..6 {
                        for k in 0..0x32usize {
                            let p = Staff(read_usize(team + club::SQUAD + k * 4));
                            if p.is_null() || read_u8(at(0xfc) + k) != 0 {
                                continue;
                            }
                            if exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) != 0 || exe::transfer_manager_player_needs_work_permit(read_usize(G_TRANSFER_MANAGER), p, team, 0) != 0 {
                                continue;
                            }
                            let squad = p.playing_squad();
                            let other_squad = if exe::database_is_national_team(club) != 0 { squad & 8 != 0 } else { squad & 2 != 0 };
                            if other_squad {
                                continue;
                            }
                            exe::database_format_name_from_strings(p.first_name(), p.second_name(), p.common_name(), at(0x158), 0x78, 1, (read_usize(p.addr())) as usize as i32);
                            let mut duty = 0usize;
                            let today = || exe::date_days_since(at(0x4c), read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP));
                            let injury = || exe::injury_get_player_injury(read_usize(G_INJURY), p);
                            let news = match case {
                                0 => team != p.national_job().addr() && {
                                    duty = exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), p, at(0x4c), team, (0) as usize as u32, (0) as usize as u32) as usize;
                                    duty != 0
                                },
                                1 => exe::transfer_manager_get_ineligibility_code(read_usize(G_TRANSFER_MANAGER), p, fixture) != 0,
                                2 => {
                                    let comp = fixture.competition().addr();
                                    comp != 0 && exe::discipline_is_player_banned(read_usize(G_DISCIPLINE), p, comp, same_group, (fixture.addr()) as usize as u32) != 0
                                }
                                3 => injury() != 0 && exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2
                                    && read_i16(injury() + injury_entry::TRAINING_RETURN_DAYS) as i32 > today(),
                                4 => injury() != 0 && read_i16(injury() + injury_entry::TRAINING_DAYS_LEFT) as i32 <= today()
                                    && read_i16(injury() + injury_entry::DAYS_LEFT) as i32 > today() + 3,
                                _ => exe::database_is_national_team(team) == 0
                                    && exe::date_days_since(G_CURRENT_DATE, p.date_joined_club() as u32, p.date_joined_club_leap_year() as u32) < 100
                                    && exe::player_stats_get_apps_for_club(read_usize(G_PLAYER_STATS), p, team) == 0
                                    && exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), fixture.competition().addr()) == 0
                                    && unplayed as u16 == 1
                                    && {
                                        let c = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, team);
                                        c != 0 && matches!(read_u8(c + contract::SQUAD_STATUS) & 0xf0, 0x10 | 0x20 | 0x30)
                                    },
                            };
                            if !news {
                                continue;
                            }
                            if draw {
                                if !header {
                                    lang(BUF, 0x98f090, gender(team), gender(team), team + club::SHORT_NAME); // "  <team> News"
                                    text_clear(at(0x20));
                                    (exe::text_set_string(at(0x20), (BUF) as usize as *const u8) as i32);
                                    exe::text_add_link(at(0x20), (team + club::SHORT_NAME) as usize as *const u8, 0x7d5, team);
                                    exe::text_draw_wrapped(at(0x20), (table) as usize as u32, 0x71, width, (at(0x14)) as usize as *mut i32, 3, (pale) as usize as u32, 2, 0);
                                    header = true;
                                }
                                if case == 0 {
                                    if exe::database_is_national_team(duty) == 0 && (exe::langlib_get_language() == 0 || exe::langlib_get_language() == 1) {
                                        format(BUF, 0x98f078, at(0x158), duty + club::SHORT_NAME); // "%s away playing with %s"
                                    } else {
                                        lang(BUF, 0x98f014, gender(duty), gender(duty), at(0x158), duty + club::SHORT_NAME); // "... on international duty with ..."
                                    }
                                } else {
                                    if case == 3 || case == 4 {
                                        injury_text(read_usize(G_INJURY), p.addr(), at(0x2ac), 0x78, 0);
                                    }
                                    let text = [0, 0x98efe4, 0x98efb8, 0x98ef6c, 0x98ef1c, 0x98eeec][case.min(5)]; // ineligible, suspended, out, doubtful, debut
                                    lang(BUF, text, at(0x158), at(0x2ac));
                                }
                                text_clear(at(0x20));
                                (exe::text_set_string(at(0x20), (BUF) as usize as *const u8) as i32);
                                exe::text_add_link(at(0x20), (at(0x158)) as usize as *const u8, 0x7d7, p);
                                if case == 0 {
                                    exe::text_add_link(at(0x20), (duty + club::SHORT_NAME) as usize as *const u8, 0x7d5, duty);
                                }
                                let (c0, c1) = (read_u8(at(0x30)) as i32, read_u8(at(0x31)) as i32);
                                exe::text_draw_wrapped(at(0x20), (table) as usize as u32, (width - 0x70) * c0 / (c0 + c1) + 0x71, width, (at(0x14)) as usize as *mut i32, 2, (yellow) as usize as u32, 2, 0);
                            }
                            next_row();
                            write_u8(at(0xfc) + k, 1);
                        }
                    }
                }
            }
            rows_counted = row();
        }
    }
    exe::drop_text((at(0x20)) as usize as *mut usize);
    llist_drop(at(0x5c));
    llist_drop(at(0xb4));
}

/// Comparer of `club_screen_draw_next_match_tab`: side by side over the display list (nothing is written outside it;
/// the fixture lists and the Text are the frame's own, built and freed inside).
pub(crate) unsafe extern "C" fn compare_club_screen_draw_next_match_tab(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(IDX_CLUB_SCREEN_DRAW_NEXT_MATCH_TAB);
    let extra: [(usize, usize); 0] = [];
    compare_by_display_list_with("Club screen next match tab", &extra, || o(top, bottom), || club_screen_draw_next_match_tab(top, bottom));
}

/// Draws the fixtures tab of the club screen: the season navigation, the friendly buttons and one row per day of the
/// season that has a fixture, friendly, tour or tournament.
///
/// Screen params: 0 the club, 1 nonzero for the read-only view [inference: no friendly buttons, no rows for friendly
/// dates], 0xc the season year shown, 0xd the fixture list (created here when 0), 0xe nonzero while a friendly is
/// being cancelled (Cancel highlighted, cancel marks on the rows), 0xf nonzero to scroll to the club's next fixture,
/// 0x10 the scroll row otherwise.
///
/// The header area (x 0x6e..0x168, `top`..`top`+0x14) holds "<< Season" (or "<< Year" for a national team) and
/// "Season >>": the first is a button (id 0x10) when the season year is after the club's first fixture year
/// (`get_club_first_fixture_year`) or after the current year, grey text otherwise; the second a button (id 0x11) while
/// the year is before the last one. The last year is `get_club_last_fixture_year`, at least the current year; for a
/// club whose nation's season spans two years (`key_nation_record::SPLIT_SEASON_FLAG` of its key nation entry), before
/// the nation's season end day (`SEASON_END_DAY`) the shown year drops by one (as a dword, + 0xffff, unless it is
/// 2001) and so does the last year; from that day on the last year drops by one unless it is the current year. For a
/// club the human controls (`human_manager_current_controls_club`), outside the read-only view, a second area
/// (0x212..0x30c) holds Cancel (id 0x12, style 0x70 while cancelling, 0x30 otherwise) and Arrange, a popup under
/// Arrange with Home Friendly (3), Away Friendly (4), Tour (0xb) and Tournament (0xa), all of id 0x13.
///
/// The dates run from the season start (the key nation's season end day of the shown year for a split season club,
/// day 0 otherwise; day 0 of 2001 when the year is 2001) up to the same day one year later. The caption "Season
/// <label>" (`key_nation_format_club_season`; the bare label for a national team) goes in a text item 0x23 high, then
/// the list area of 6 columns (5, 0xa, 2, 1, 0xc, 3) fills the rest, as many rows as font 3 fits in
/// `bottom` − y − 0x10 (the row count is the low byte of a dword whose upper bytes are those of the key nation entry
/// pointer, as the original pushes it). Without a list (param 0xd = 0) a new LLIST is created, stored in param 0xd and
/// filled by `FIXTURE_MANAGER::find_fixtures` (start, end, the club's id, the list, −1, −2, read-only view = 0).
///
/// Then for each day: the fixtures dated that day each get a row (`add_club_fixture_row`, link mode 0x14); a fixture
/// row whose helper returns nonzero is remembered as the row to scroll to; a fixture of the club's own friendly
/// competitions (`Friendly::is_own_comp_record`) still to be played (today before its date) gets the cancel mark (item
/// 0x15 over x 0x6e..0x2f6) while cancelling. Fixtures before the day are skipped. A day with no fixture, outside the
/// read-only view, asks `Friendly::get_club_date_status`: 5 (a friendly) gives the date (dark blue), the opponent
/// "  <short name> ?" as a link (yellow when a human manages it), its nation's three-letter name as a link to the
/// national team when it differs from the club's, H or A, "  Friendly", "---" and, while cancelling and not locked,
/// the cancel mark (0x16); 6 (a tour) the date, "  Tba", the nation toured, A, "  Tour Of <nation>", "---" and the
/// cancel mark (0x17) for the touring club the human controls (while cancelling, only when not locked); 3 (a
/// tournament) the date, "  Tba", H or A (host or not), the tournament name (a link 0x7d0 to its competition when it
/// exists), "---" and, with no competition yet, the cancel mark (0x18; while cancelling only when not under way).
///
/// At the end the list scrolls (`Area::set_scroll_row`) to the remembered row − 7 (at least 0) with param 0xf set,
/// which is then cleared, or to param 0x10; param 0x11 gets the list area. While cancelling, a full-screen button item
/// (0, 0, 0x31f, 0x257: these two are patched in the 1920x1080 exe and are read from the exe) with id 0x19 catches the
/// clicks.
/// Original: club_screen_draw_fixtures_tab (0x45f380, club_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn club_screen_draw_fixtures_tab(top: i32, bottom: i32) {
    covered!("tel02_1 fixtures tab");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let season_label: unsafe extern "C" fn(u32, usize, usize, i32) -> i32 = core::mem::transmute(FIXTURES_FN_FORMAT_CLUB_SEASON);
    let fixture_row: unsafe extern "C" fn(usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(FIXTURES_FN_ADD_CLUB_FIXTURE_ROW);
    let date_status: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32, u32, u32, u32) -> u8 =
        core::mem::transmute(FN_FRIENDLY_GET_CLUB_DATE_STATUS);
    let area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let button: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let label: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let text_item: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let button_item: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let sm = SCREEN_MANAGER;
    let c = |a: usize| colour_word(a);
    let mut d1 = [0u8; 8];
    let mut d2 = [0u8; 8];
    let mut out = [0u8; 8];
    let mut tmp = [0u8; 8];
    let d1p = d1.as_mut_ptr() as usize;
    let d2p = d2.as_mut_ptr() as usize;
    let outp = out.as_mut_ptr() as usize;
    let tmpp = tmp.as_mut_ptr() as usize;
    exe::date_date_54a110(d1p);
    exe::date_date_54a110(d2p);
    let mut columns: [u8; 6] = [5, 0xa, 2, 1, 0xc, 3];
    let club = Club(exe::screenmanager_get_screen_param(sm, 0));
    let read_only = exe::screenmanager_get_screen_param(sm, 1);
    let mut year = exe::screenmanager_get_screen_param(sm, 0xc) as u32;
    let mut list = exe::screenmanager_get_screen_param(sm, 0xd);
    let cancelling = exe::screenmanager_get_screen_param(sm, 0xe);
    let scroll_to_next = exe::screenmanager_get_screen_param(sm, 0xf);
    let header = area(current_human_display(), 0x6e, top as usize, 0x168, (top + 0x14) as usize, 2, 0, 1, 0, 1, 0, usize::MAX);
    let nation_entry = (G_LEAGUE_TABLE as isize + exe::key_nation_index_of_club(club) as i8 as isize * 0x48) as usize;
    let fm = read_usize(G_FIXTURE_MANAGER);
    let mut last = get_club_last_fixture_year(fm, club.addr());
    let current_year = read_u16(G_CURRENT_DATE_YEAR);
    if last < current_year {
        last = current_year;
    }
    let national = |x: usize| exe::database_is_national_team(x) != 0;
    if !national(club.addr()) && read_u32(nation_entry + key_nation_record::SPLIT_SEASON_FLAG) != 0 {
        let day = read_i16(G_CURRENT_DATE) as i32;
        if day < read_u16(nation_entry + key_nation_record::SEASON_END_DAY) as i32 && year as u16 != 0x7d1 {
            year = year.wrapping_add(0xffff);
        } else if last == current_year {
            last = last.wrapping_add(1);
        }
        last = last.wrapping_add(0xffff);
    }
    // the previous / next season buttons
    text(FIXTURES_TEXT, if national(club.addr()) { 0x98f914 } else { 0x98e7ac }, 0x98e7c4); // "<%s - ignore> Year|Season", "<<"
    let first = get_club_first_fixture_year(fm, club.addr());
    if year as u16 > first || year as u16 > current_year {
        button(sm, header, 0, 0, 0x30, c(G_GUI_COLOUR_GREY), 0xc, 1, c(G_GUI_COLOUR_PALE_GREY), FIXTURES_TEXT, 0x10, 0);
    } else {
        label(sm, header, 0, 0, 0x30, c(G_GUI_COLOUR_GREY), 0x2c, 1, c(G_GUI_COLOUR_PALE_GREY), FIXTURES_TEXT);
    }
    text(FIXTURES_TEXT, if national(club.addr()) { 0x98f900 } else { 0x98e790 }, 0x98e7a8); // "Year|Season <%s - ignore>", ">>"
    if (year as u16) < last {
        button(sm, header, 1, 0, 0x30, c(G_GUI_COLOUR_GREY), 0xc, 1, c(G_GUI_COLOUR_PALE_GREY), FIXTURES_TEXT, 0x11, 0);
    } else {
        label(sm, header, 1, 0, 0x30, c(G_GUI_COLOUR_GREY), 0x2c, 1, c(G_GUI_COLOUR_PALE_GREY), FIXTURES_TEXT);
    }
    // Cancel / Arrange and the friendly popup, for a club the human controls
    if !national(club.addr()) && exe::human_manager_current_controls_club(club, 0, 1) != 0 && read_only == 0 {
        let a = area(current_human_display(), 0x212, top as usize, 0x30c, (top + 0x14) as usize, 2, 0, 1, 0, 1, 0, usize::MAX);
        text(FIXTURES_TEXT, 0x98a1a0); // "Cancel"
        let style = if cancelling != 0 { 0x70 } else { 0x30 };
        button(sm, a, 0, 0, style, c(G_GUI_COLOUR_GREY), 0xc, 1, c(G_GUI_COLOUR_PALE_GREY), FIXTURES_TEXT, 0x12, 0);
        text(FIXTURES_TEXT, 0x98f8f8); // "Arrange"
        let arrange = button(sm, a, 1, 0, 0x30, c(G_GUI_COLOUR_GREY), 0xc, 1, c(G_GUI_COLOUR_PALE_GREY), FIXTURES_TEXT, 0, 0);
        let popup = area(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x130, c(FIXTURES_COLOUR_POPUP), arrange);
        for (caption, kind) in [(0x98f8e8usize, 3usize), (0x98f8d8, 4), (0x98f8d0, 0xb), (0x98f8c4, 0xa)] {
            text(FIXTURES_TEXT, caption); // "Home Friendly", "Away Friendly", "Tour", "Tournament"
            item(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, c(FIXTURES_COLOUR_POPUP), c(G_GUI_COLOUR_YELLOW), 1, 1,
                c(G_GUI_COLOUR_BLACK), FIXTURES_TEXT, 0, 0x13, kind, popup);
        }
    }
    let mut y = top + 0x19;
    // the season's dates: from the start day of the shown year to the same day a year later
    let start_day = if !national(club.addr()) && read_u32(nation_entry + key_nation_record::SPLIT_SEASON_FLAG) != 0 {
        read_u16(nation_entry + key_nation_record::SEASON_END_DAY) as u32
    } else {
        0
    };
    let t = exe::new_date(tmpp, start_day, year);
    exe::date_operator_3d(d1p, outp, t);
    let t = exe::new_date(tmpp, read_u32(d1p), read_u32(d1p + 2).wrapping_add(1));
    exe::date_operator_3d(d2p, outp, t);
    if year as u16 == 0x7d1 {
        let t = exe::new_date(tmpp, 0, 0x7d1);
        exe::date_operator_3d(d1p, outp, t);
    }
    let mut label_buffer = [0u8; 0xc8];
    let lb = label_buffer.as_mut_ptr() as usize;
    season_label(year, club.addr(), lb, 0xc8);
    if !national(club.addr()) {
        text(FIXTURES_TEXT, 0x98f8a0, lb); // "Season <%s - season e.g. 1998/9>"
    } else {
        fixtures_copy_text(FIXTURES_TEXT, lb);
    }
    text_item(sm, 0x6e, y as usize, 0x30c, (y + 0x23) as usize, 2, 0, 0xc, 3, c(G_GUI_COLOUR_YELLOW), FIXTURES_TEXT, 1);
    y += 0x28;
    let h = exe::font_get_height(3);
    let rows = ((bottom - y - 0x10) / h) as u8;
    let rows_dword = (nation_entry & !0xff) | rows as usize; // the original pushes the dword whose low byte it set
    let lst = area(current_human_display(), 0x6e, y as usize, 0x30c, bottom as usize, 6, columns.as_mut_ptr() as usize, rows_dword, 0, 2, 0, usize::MAX);
    let mut row = 0usize;
    if list == 0 {
        let p = exe::crt_operator_new(0xe);
        list = if p != 0 { exe::new_llist(p) } else { 0 };
        exe::screenmanager_set_screen_param(sm, 0xd, list, 0);
        exe::fixture_manager_find_fixtures(fm, read_u32(d1p), read_u32(d1p + 2), read_u32(d2p), read_u32(d2p + 2),
            read_u32(club.addr()), list, u32::MAX, 0xffff_fffe, (read_only == 0) as u32);
    }
    let mut next_row: i32 = -1;
    write_u32(list + 0xa, read_u32(list + 2)); // the list cursor back to the first node
    let mut fx = exe::llist_get_next(list);
    let friendly = read_usize(G_FRIENDLY);
    loop {
        let (y1, y2) = (read_u16(d1p + 2), read_u16(d2p + 2));
        let before_end = y2 > y1 || (y2 == y1 && read_i16(d2p) > read_i16(d1p));
        if !before_end {
            break;
        }
        let (day, yr) = (read_i16(d1p), read_u16(d1p + 2));
        if fx != 0 {
            let f = exe::new_date(tmpp, read_u16(fx + fixture::DAY) as u32, read_u16(fx + fixture::YEAR) as u32);
            if yr == read_u16(f + 2) && day == read_i16(f) {
                // a fixture on this day
                let r = fixture_row(club.addr(), fx, lst, row, 0x14);
                if exe::friendly_is_own_comp_record(friendly, read_usize(fx + fixture::COMPETITION)) == 0 {
                    if r != 0 {
                        next_row = row as i32;
                    }
                } else if r != 0 {
                    next_row = row as i32;
                } else {
                    let f = exe::new_date(tmpp, read_u16(fx + fixture::DAY) as u32, read_u16(fx + fixture::YEAR) as u32);
                    let (cd, cy) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
                    let fy = read_u16(f + 2);
                    let still_to_play = cy < fy || (cy == fy && cd < read_i16(f));
                    if still_to_play && cancelling != 0 {
                        covered!("tel02_1 fixtures cancel mark of an own friendly fixture");
                        item(current_human_display(), 2, 0x6e, 0, 0x2f6, 0, 0, row, 1, 0, 0, 0xc, 1, c(G_GUI_COLOUR_PALE_GREY), 0, 0, 0x15, fx, lst);
                    }
                }
                row += 1;
                fx = exe::llist_get_next(list);
                fixtures_next_day(d1p, outp, tmpp);
                continue;
            }
            // a fixture before this day: skip it
            let f = exe::new_date(tmpp, read_u16(fx + fixture::DAY) as u32, read_u16(fx + fixture::YEAR) as u32);
            let fy = read_u16(f + 2);
            if yr > fy || (yr == fy && day > read_i16(f)) {
                fx = exe::llist_get_next(list);
            }
        }
        if read_only == 0 {
            let (mut tournament, mut friendly_match, mut tour) = (0usize, 0usize, 0usize);
            let status = date_status(friendly, club.addr(), d1p, &mut tournament as *mut usize as usize,
                &mut friendly_match as *mut usize as usize, &mut tour as *mut usize as usize, 0, 0, 4, 1);
            let mut drawn = true;
            match status {
                5 => {
                    covered!("tel02_1 fixtures friendly row");
                    fixtures_date_cell(d1p, lst, row);
                    let fm_ = friendly_match;
                    let mut opponent = read_usize(fm_ + friendly_match::HOME_CLUB);
                    if club.addr() == opponent {
                        opponent = read_usize(fm_ + friendly_match::AWAY_CLUB);
                    }
                    format(FIXTURES_TEXT, 0x98f898, opponent + club::SHORT_NAME); // "  %s ?"
                    let tc = if exe::human_manager_is_human_club(opponent, 0, 1, 0, 0) != 0 { c(G_GUI_COLOUR_YELLOW) } else { c(G_GUI_COLOUR_PALE_GREY) };
                    button(sm, lst, 1, row, 1, 0, 1, 3, tc, FIXTURES_TEXT, 0x7d5, opponent);
                    if !national(opponent) && read_usize(opponent + club::NATION) != club.nation().addr() {
                        let team = exe::database_get_national_team(read_usize(opponent + club::NATION));
                        button(sm, lst, 2, row, 1, 0, 1, 2, c(G_GUI_COLOUR_YELLOW), read_usize(opponent + club::NATION) + nation::THREE_LETTER_NAME, 0x7d5, team);
                    }
                    let home = club.addr() == read_usize(fm_ + friendly_match::HOME_CLUB);
                    text(FIXTURES_TEXT, if home { 0x98f880 } else { 0x98f868 }, FIXTURES_EMPTY_ARG); // "H<..>" / "A<..>"
                    label(sm, lst, 3, row, 1, 0, 0xc, 2, c(FIXTURES_COLOUR_VENUE), FIXTURES_TEXT);
                    text(FIXTURES_TEXT, 0x98f85c); // "  Friendly"
                    label(sm, lst, 4, row, 1, 0, 1, 2, c(FIXTURES_COLOUR_COMP), FIXTURES_TEXT);
                    fixtures_no_result_cell(lst, row);
                    if cancelling != 0 && read_u32(fm_ + FIXTURES_FRIENDLY_LOCKED) == 0 {
                        item(current_human_display(), 2, 0x6e, 0, 0x2f6, 0, 0, row, 1, 0, 0, 0xc, 1, c(G_GUI_COLOUR_PALE_GREY), 0, 0, 0x16, fm_, lst);
                    }
                }
                6 => {
                    covered!("tel02_1 fixtures tour row");
                    fixtures_date_cell(d1p, lst, row);
                    text(FIXTURES_TEXT, 0x98f850); // "  Tba"
                    label(sm, lst, 1, row, 1, 0, 1, 3, c(G_GUI_COLOUR_PALE_GREY), FIXTURES_TEXT);
                    let n = read_usize(tour + FIXTURES_TOUR_NATION);
                    let team = exe::database_get_national_team(n);
                    button(sm, lst, 2, row, 1, 0, 1, 2, c(G_GUI_COLOUR_YELLOW), read_usize(tour + FIXTURES_TOUR_NATION) + nation::THREE_LETTER_NAME, 0x7d5, team);
                    text(FIXTURES_TEXT, 0x98f868, FIXTURES_EMPTY_ARG); // "A<%s - COMMENT - away>"
                    label(sm, lst, 3, row, 1, 0, 0xc, 2, c(FIXTURES_COLOUR_VENUE), FIXTURES_TEXT);
                    let n = read_usize(tour + FIXTURES_TOUR_NATION);
                    let gender = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
                    text(FIXTURES_TEXT, 0x98f834, gender, gender, n + nation::SHORT_NAME); // "  Tour Of {}<%s - nation>{}"
                    label(sm, lst, 4, row, 1, 0, 1, 2, c(FIXTURES_COLOUR_COMP), FIXTURES_TEXT);
                    fixtures_no_result_cell(lst, row);
                    if exe::human_manager_current_controls_club(club, 0, 1) != 0
                        && club.addr() == read_usize(tour + FIXTURES_TOUR_CLUB)
                        && (cancelling == 0 || read_u32(tour + FIXTURES_TOUR_LOCKED) == 0)
                    {
                        item(current_human_display(), 2, 0x6e, 0, 0x2f6, 0, 0, row, 1, 0, 0, 0xc, 1, c(G_GUI_COLOUR_PALE_GREY), 0, 0, 0x17, tour, lst);
                    }
                }
                3 => {
                    covered!("tel02_1 fixtures tournament row");
                    fixtures_date_cell(d1p, lst, row);
                    text(FIXTURES_TEXT, 0x98f850); // "  Tba"
                    label(sm, lst, 1, row, 1, 0, 1, 3, c(G_GUI_COLOUR_PALE_GREY), FIXTURES_TEXT);
                    let host = club.addr() == read_usize(tournament + tournament_slot::HOST);
                    text(FIXTURES_TEXT, if host { 0x98f880 } else { 0x98f868 }, FIXTURES_EMPTY_ARG); // "H<..>" / "A<..>"
                    label(sm, lst, 3, row, 1, 0, 0xc, 2, c(FIXTURES_COLOUR_VENUE), FIXTURES_TEXT);
                    format(FIXTURES_TEXT, 0x987ff0, tournament + tournament_slot::NAME); // "  %s"
                    let comp = read_usize(tournament + tournament_slot::COMPETITION);
                    if comp != 0 {
                        button(sm, lst, 4, row, 1, 0, 1, 2, c(FIXTURES_COLOUR_COMP), FIXTURES_TEXT, 0x7d0, read_usize(comp + FIXTURES_COMP_ID));
                    } else {
                        label(sm, lst, 4, row, 1, 0, 1, 2, c(FIXTURES_COLOUR_COMP), FIXTURES_TEXT);
                    }
                    fixtures_no_result_cell(lst, row);
                    if read_usize(tournament + tournament_slot::COMPETITION) == 0
                        && (cancelling == 0 || read_u32(tournament + tournament_slot::IN_USE) == 0)
                    {
                        item(current_human_display(), 2, 0x6e, 0, 0x2f6, 0, 0, row, 1, 0, 0, 0xc, 1, c(G_GUI_COLOUR_PALE_GREY), 0, 0, 0x18, tournament, lst);
                    }
                }
                _ => drawn = false,
            }
            if drawn {
                row += 1;
            }
        }
        fixtures_next_day(d1p, outp, tmpp);
    }
    // the scroll and the screen params
    let display = current_human_display();
    let lst16 = lst as i16;
    let area_of = |k: i16| (display as isize + k as isize * 0xbf1 + 4) as usize;
    if scroll_to_next != 0 {
        let r = next_row - 7;
        let r = if r <= 0 { 0 } else { r };
        if lst16 != -1 {
            exe::area_set_scroll_row(area_of(lst16), r as usize);
        }
        exe::screenmanager_set_screen_param(sm, 0x11, lst16 as isize as usize, 0);
        exe::screenmanager_set_screen_param(sm, 0xf, 0usize, 0);
    } else {
        let r = exe::screenmanager_get_screen_param(sm, 0x10);
        if lst16 != -1 {
            exe::area_set_scroll_row(area_of(lst16), r);
        }
        exe::screenmanager_set_screen_param(sm, 0x11, lst16 as isize as usize, 0);
    }
    if cancelling != 0 {
        // the 1920x1080 exe patches these two immediates (push 0x257 at 0x46041b, push 0x31f at 0x460420)
        let bottom_y = read_u32(0x46041c) as usize;
        let right_x = read_u32(0x460421) as usize;
        button_item(sm, 0, 0, right_x, bottom_y, 1, 0, 0xc, 1, c(G_GUI_COLOUR_PALE_GREY), 0, 0x19, 0);
    }
}

/// Comparer of `club_screen_draw_fixtures_tab`: side by side over the display list (and the screen params it sets,
/// 0xf and 0x11), with the fixture list's 0xe bytes (its cursor moves). With no list yet (param 0xd = 0) only the
/// original runs: it allocates the list and stores it in param 0xd.
pub(crate) unsafe extern "C" fn compare_club_screen_draw_fixtures_tab(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(IDX_CLUB_SCREEN_DRAW_FIXTURES_TAB);
    let list = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0xd);
    if list == 0 {
        // simplification: allocating the list (operator new, param 0xd) does not run twice
        return o(top, bottom);
    }
    let extra = [(list, 0xe)];
    compare_by_display_list_with("Club screen fixtures tab", &extra, || o(top, bottom), || club_screen_draw_fixtures_tab(top, bottom));
}

/// Finds the badge picture of a Spanish club and copies its path into `out`.
///
/// Only a club whose nation (`club::NATION`) is Spain (the id in `KEY_NATION_SPAIN`). The name looked up is the
/// English name (`exe::club_get_english_name`) of the club itself when `exe::club_get_linked_b_team` (asked with the
/// club in its out slot and 1) leaves that slot nonzero, otherwise of the club it returns. The table
/// `BADGE_PICTURE_TABLE` is walked in order until the first entry with an empty name; the first entry whose name is
/// equal (strcmp) and whose file builds a path (`exe::utils_build_file_path` with `CM3_PICTURES`) is the one: if that
/// file opens for reading ("rb", closed at once) its path is copied into `out` and the answer is 1. Otherwise 0 (a
/// path that does not open ends the search, it does not go on to the next entries).
/// Original: `find_club_badge_picture_path` (0x460450, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn find_club_badge_picture_path(club: usize, out: usize) -> i32 {
    let club = Club(club);
    if club.is_null() {
        return 0;
    }
    let nation = club.nation().addr();
    if nation == 0 || read_u32(nation) != read_u32(KEY_NATION_SPAIN) {
        return 0;
    }
    covered!("tel02_1 badge picture of a Spanish club");
    let mut slot = club.addr() as u32;
    let linked = exe::club_get_linked_b_team(club, &mut slot, 1);
    let name = exe::club_get_english_name(if slot != 0 { club.addr() } else { linked });
    if name == 0 {
        return 0;
    }
    let mut k: i16 = 0;
    loop {
        let entry = (BADGE_PICTURE_TABLE as isize + k as isize * 8) as usize;
        let entry_name = read_usize(entry);
        if read_u8(entry_name) == 0 {
            return 0;
        }
        if badge_names_equal(entry_name, name) {
            let path = exe::utils_build_file_path(BADGE_PICTURES_KEY, read_usize(entry + 4));
            if path != 0 {
                let f = exe::crt_fopen(path, BADGE_OPEN_MODE);
                if f == 0 {
                    return 0;
                }
                copy_c_string(out, path);
                exe::crt_fclose(f);
                return 1;
            }
        }
        k = k.wrapping_add(1);
    }
}

/// Comparer of `find_club_badge_picture_path`: side by side on the answer and the path written to `out` (the file is
/// only opened for reading).
pub(crate) unsafe extern "C" fn compare_find_club_badge_picture_path(club: usize, out: usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_FIND_CLUB_BADGE_PICTURE_PATH);
    if out == 0 {
        return o(club, out);
    }
    let (r, n, equal, _, _) = crate::comparison::side_by_side(&[(out, 0x104)], || o(club, out), || find_club_badge_picture_path(club, out));
    SCOREBOARD.record(equal && r == n, || format!("find_club_badge_picture_path(club={:#x}) original={} rust={}", club, r, n));
    r
}

/// Draws the club screen's info tab between `top` and `bottom`: the "View" menu, the title and one of five views.
///
/// The function-static green (`INFO_TAB_STATIC_GREEN`, `exe::graphics_rgb_to_pixel(0, 0xc0, 0, 0)`) is made on the
/// first call (bit 0 of `INFO_TAB_STATIC_GUARD`). The screen params: 0x1b the view flags (1 General Info, 0x10 Stats,
/// 2 Finances, 4 Income, 8 Expenditure), 0 the club, 1 a value the views read. Unless the view has bit 1 or 0x10, a
/// national team (`exe::database_is_national_team`) or a club the current human does not control
/// (`exe::human_manager_current_controls_club(club, 0, 1)`) falls back to General Info. The club's finance record comes
/// from `exe::financetable_get_club_finance`.
///
/// The "View" button (x 0x6e..0xeb, y `top`..`top + 0x14`, grey on pale grey) opens a menu (`Display::add_area` with
/// 0x130) of the views, each an item with id 0x20 and the view's bit as value, the selected one flagged 0x2002 and its
/// text kept as the title: General Info and Stats always, Finances, Income and Expenditure only for a club the human
/// controls that is not a national team. The title goes in yellow at y `top + 0x19`..`top + 0x3c` (font 3) and the
/// view starts at `top + 0x41`. With no view bit at all the title is the original's uninitialised stack buffer (here
/// empty).
///
/// General Info: a two-column table (0x6e..0x30c, 0x8e high, 6 rows) and, when `find_club_badge_picture_path` finds the
/// badge, the picture in an area at 0x27e. Rows (labels pale grey, values yellow): Continent (national team: the
/// nation's continent name, or "Unknown") or Nation (the Welsh clubs Swansea, Cardiff and Wrexham read "Wales";
/// otherwise the nation's name as a link 0x7d5 to its national team, or "Unknown"); Status ("-" for a national or
/// all-star team, `exe::nationalteams_is_national_or_all_star_team`; otherwise Professional, Semi-Professional, Amateur
/// by `club::PROFESSIONAL_STATUS` 1/2/3, or "Unknown"); Finances ("-" likewise; otherwise by
/// `exe::finance_get_wealth_band`: 2 Rich, 1 Secure, 0 Ok, −2 Bankrupt when the finance's +0x165 flag is 1, Insecure for
/// −1 or when that flag is not 1); Stadium (a club that does not own it, `club::OWN_STADIUM` 0: the first other club of
/// the table with the same stadium that owns it is named as the ground's owner, otherwise "(Rented Stadium)"; with the
/// stadium's city when it has one; no stadium: "Various" for a national team, else "Unknown"); Facilities (the capacity,
/// the seated capacity when it is below the capacity and not 0, "all seater" when the seated capacity reaches it, with
/// retractable roof and/or under-soil heating; "-" with no stadium or capacity 0); Training Ground (not for national
/// teams: above 15 "Top facilities plus youth academy", otherwise reputation / max(training, 1) up to 400 Good, 600
/// Adequate, 800 Need some investment, 1000 Need major investment, beyond Awful).
///
/// Then the "Non-Playing Staff" list (0x93 lower, down to `bottom`, two columns, as many rows as fit in font 3 after 16
/// pixels): an orange heading 0x73 pixels in, then the non-playing staff (`club_collect_non_playing_staff`, at most 0x15)
/// sorted (national or club comparer) after setting `INFO_TAB_STAFF_SORTED_FLAG`, a person repeated right after
/// himself shown once, from row 2: the name ("  %s") as a link (0x7dc national, 0x7d7 club) unless his club job
/// (`staff::JOB_FOR_CLUB`) is 1 and he has no regen mark (`staff_state::STATUS_FLAGS` 0x40), and his job name
/// (`staff::JOB_FOR_NATION` for a national team, `staff::JOB_FOR_CLUB` otherwise). No staff: "Unknown". The list is
/// scrolled to param 0x1c and its index goes to param 0x1d.
///
/// Stats (`info_tab_stats_view`) and Finances, Income and Expenditure (`info_tab_money_views`) are drawn from the same
/// frame.
/// Original: club_screen_draw_info_tab (0x460590, club_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn club_screen_draw_info_tab(top: i32, bottom: i32) {
    covered!("tel02_2 info tab");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let button: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let heading: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let sm = SCREEN_MANAGER;
    let c = colour_word;

    let guard = read_u8(INFO_TAB_STATIC_GUARD);
    if guard & 1 == 0 {
        write_u8(INFO_TAB_STATIC_GUARD, guard | 1);
        write_u16(INFO_TAB_STATIC_GREEN, exe::graphics_rgb_to_pixel(0, 0xc0, 0, 0) as u16);
    }
    let mut f = InfoTabFrame {
        club: 0,
        finance: 0,
        view: 0,
        param1: 0,
        y: top,
        bottom,
        menu_area: 0,
        columns: [2, 3, 2, 5, 2, 1, 1, 1, 1],
        general: 0,
        stats: 0,
        cash_a: [0, 0],
        title: [0; 0x100], // the original leaves it uninitialised (stack) until a view is selected
        path: [0; 0x104],
        staff: [0; 0x15],
    };
    f.view = exe::screenmanager_get_screen_param(sm, 0x1b) as u32;
    f.club = exe::screenmanager_get_screen_param(sm, 0);
    f.param1 = exe::screenmanager_get_screen_param(sm, 1);
    let club = f.club;
    if f.view & 0x11 == 0 {
        if exe::database_is_national_team(club) != 0 {
            f.view = 1;
        }
        if exe::human_manager_current_controls_club(club, 0, 1) == 0 {
            f.view = 1;
        }
    }
    f.finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);

    // the "View" button and its menu
    text(INFO_TAB_TEXT, 0x988004); // "View"
    let b = button(sm, 0x6e, top as usize, 0xeb, (top + 0x14) as usize, 0x30, c(G_GUI_COLOUR_GREY), 0xc, 1, c(G_GUI_COLOUR_PALE_GREY), INFO_TAB_TEXT, 0, 0);
    f.menu_area = area(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x130, c(INFO_TAB_COLOUR_MENU), b);
    let title = f.title.as_mut_ptr() as usize;
    let menu = f.menu_area;
    // one menu entry: id 0x20, value `bit`; the selected one flagged 0x2002 and copied as the title
    let entry = |bit: u32, selected: bool| {
        let mark = if selected { 0x2002 } else { 2 };
        item(current_human_display(), mark, 0, 0, 0, 0, 0, 0, 0x10, c(INFO_TAB_COLOUR_MENU), c(G_GUI_COLOUR_YELLOW), 1, 1, c(G_GUI_COLOUR_BLACK), INFO_TAB_TEXT, 0, 0x20, bit as usize, menu);
        if selected {
            copy_c_string(title, INFO_TAB_TEXT);
        }
    };
    text(INFO_TAB_TEXT, 0x990598); // " General Info"
    f.general = f.view & 1;
    entry(1, f.general != 0);
    text(INFO_TAB_TEXT, 0x98ee70); // " Stats"
    f.stats = f.view & 0x10;
    entry(0x10, f.stats != 0);
    if exe::human_manager_current_controls_club(club, 0, 1) != 0 && exe::database_is_national_team(club) == 0 {
        text(INFO_TAB_TEXT, 0x99058c); // " Finances"
        entry(2, f.view & 2 != 0);
        text(INFO_TAB_TEXT, 0x990584); // " Income"
        entry(4, f.view & 4 != 0);
        text(INFO_TAB_TEXT, 0x990574); // " Expenditure"
        entry(8, f.view & 8 != 0);
    }
    let y = top + 0x19;
    heading(sm, 0x6e, y as usize, 0x30c, (y + 0x23) as usize, 2, 0, 0xc, 3, c(G_GUI_COLOUR_YELLOW), title, 1);
    f.y = y + 0x28;

    if f.general != 0 {
        info_tab_general_view(&mut f);
    } else if f.stats != 0 {
        // 0x4619d3: esi = f.y, [E+0x2e0] = f.bottom, edi = f.club, [E+0x30] = f.param1
        info_tab_stats_view(f.y, f.bottom, f.club, f.param1);
    } else {
        // 0x462a80: esi = f.y, [E+0x2e0] = f.bottom, [E+0x14] = f.view, ebp = f.finance, [E+0x10] = f.cash_a
        info_tab_money_views(f.y, f.bottom, f.view as u8, Finance(f.finance), &mut f.cash_a[0]);
    }
    // 0x467166: the two Cash objects' destructors are the empty 0x5a53c0
}

/// Comparer of `club_screen_draw_info_tab`: side by side over the display list (with the screen params, where the
/// staff list's index goes), plus the function-static guard, the staff-sorted flag and the static green.
pub(crate) unsafe extern "C" fn compare_club_screen_draw_info_tab(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(IDX_CLUB_SCREEN_DRAW_INFO_TAB);
    let extra = [(INFO_TAB_STATIC_GUARD, 2), (INFO_TAB_STATIC_GREEN, 2)];
    compare_by_display_list_with("Club screen info tab", &extra, || o(top, bottom), || club_screen_draw_info_tab(top, bottom));
}

/// Stores a money amount: the dword at `this` gets `amount`.
/// Original: `Cash::set` (0x4671a0, club_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cash_set(this: usize, amount: i32) {
    covered!("tel03_1 cash set");
    write_i32(this, amount);
}

/// Returns the last month's block of a club's finance record (`this` + 0x20; a 4-byte routine, the hook covers the
/// first alignment nop).
/// Original: `Finance::get_last_month` (0x4671b0, club_screens.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn finance_get_last_month(this: usize) -> usize {
    this + FINANCE_LAST_MONTH_BLOCK
}

/// Returns the current month's block of a club's finance record (`this` + `finance::MONTH_PLAYERS_BOUGHT`, +0x70).
/// Original: `Finance::get_this_month` (0x4671c0, club_screens.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn finance_get_this_month(this: usize) -> usize {
    this + finance::MONTH_PLAYERS_BOUGHT // tel03_1 the current month block
}

/// Returns the last season's block of a club's finance record (`this` + `finance::LAST_SEASON_TOTALS`, +0xc0).
/// Original: `Finance::get_last_year` (0x4671d0, club_screens.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn finance_get_last_year(this: usize) -> usize {
    this + finance::LAST_SEASON_TOTALS
}

/// Returns the current season's block of a club's finance record (`this` + `finance::SEASON_PLAYERS_BOUGHT`, +0x110).
/// Original: `Finance::get_this_year` (0x4671e0, club_screens.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn finance_get_this_year(this: usize) -> usize {
    this + finance::SEASON_PLAYERS_BOUGHT
}

/// Returns the dword at the action id of item `widget` of this Display (`display::ITEM_ID`: the id in the low word,
/// the next field in the high word, as the original loads it), or 0 for widget −1.
/// Original: `Display::get_widget_id` (0x468b10, club_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn display_get_widget_id(this: usize, widget: u32) -> u32 {
    let widget = widget as i16;
    if widget == -1 {
        return 0;
    }
    read_u32(widget_field(this, widget, display::ITEM_ID))
}

/// Returns the action id (i16, sign-extended) of item `widget` of the current slot's Display; for widget −1, or an item
/// whose id is 0, the injected action id of the screen manager (+0x26278e).
/// Original: `ScreenManager::get_widget_action_id` (0x468b40, club_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screenmanager_get_widget_action_id(this: usize, widget: u32) -> i32 {
    const INJECTED_ACTION_ID: usize = 0x26278e;
    let widget = widget as i16;
    let id = if widget == -1 { 0 } else { // tel03_1 the widget's own id
        read_i16(widget_field(widget_current_display(this), widget, display::ITEM_ID)) };
    if id != 0 {
        id as i32
    } else {
        read_i16(this + INJECTED_ACTION_ID) as i32
    }
}

/// Returns the value attached to item `widget` of the current slot's Display (`display::ITEM_VALUE`), or 0 for widget
/// −1.
/// Original: `ScreenManager::get_widget_action_data` (0x468b90, club_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screenmanager_get_widget_action_data(this: usize, widget: u32) -> u32 {
    let widget = widget as i16;
    if widget == -1 {
        return 0;
    }
    read_u32(widget_field(widget_current_display(this), widget, display::ITEM_VALUE))
}

/// Returns the drop target (an item index, i16 at item +0x7e) that a drag left on item `widget` of the current slot's
/// Display, or −1 (the whole dword) for widget −1. The high word is the original's leftover of the address arithmetic
/// (the high half of `widget × 99`: 0 for a non-negative widget).
/// Original: `ScreenManager::get_widget_drop_target` (0x468bd0, club_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screenmanager_get_widget_drop_target(this: usize, widget: u32) -> u32 {
    const ITEM_DROP_TARGET: usize = 0xba9dc;
    let widget = widget as i16;
    if widget == -1 {
        return u32::MAX;
    }
    let high = ((widget as i32).wrapping_mul(99) as u32) & 0xffff_0000;
    high | read_u16(widget_field(widget_current_display(this), widget, ITEM_DROP_TARGET)) as u32
}

/// Sets the scroll row of area `area` of this Display (areas of 0xbf1 bytes from +4) through `Area::set_scroll_row`;
/// nothing for area −1.
/// Original: `Display::set_area_scroll_row` (0x46ad90, club_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn display_set_area_scroll_row(this: usize, area: u32, row: u32) {
    let area = area as i16;
    if area == -1 {
        return;
    }
    covered!("tel03_1 set area scroll row");
    exe::area_set_scroll_row((this as isize + area as isize * 0xbf1 + 4) as usize, row as usize);
}

/// Comparer for `cash_set`: side by side over the dword.
pub(crate) unsafe extern "thiscall" fn compare_cash_set(this: usize, amount: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_CASH_SET);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 4)], || o(this, amount), || cash_set(this, amount));
    SCOREBOARD.record(igual, || format!("cash_set({:#x}, {}): {:?} x {:?}", this, amount, mo, mr));
}

/// Comparer for `display_set_area_scroll_row`: side by side over the area's scroll row (`Area` +0x1fc).
pub(crate) unsafe extern "thiscall" fn compare_display_set_area_scroll_row(this: usize, area: u32, row: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_DISPLAY_SET_AREA_SCROLL_ROW);
    if area as i16 == -1 {
        return o(this, area, row);
    }
    let scroll = (this as isize + (area as i16) as isize * 0xbf1 + 4 + 0x1fc) as usize;
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(scroll, 4)], || o(this, area, row), || display_set_area_scroll_row(this, area, row));
    SCOREBOARD.record(igual, || format!("display_set_area_scroll_row(area {}, row {}): {:?} x {:?}", area as i16, row, mo, mr));
}

/// Opens the club history screen for `club` (builder `club_history_screen_build`, handler `club_history_screen_input`,
/// leave `club_history_screen_leave`). The params: 0 the club, 1 the tab (3 Records, or 8 Players with
/// `players_tab`), 0x19..0x1c the history lists (none yet), 0x1d the landmark period −1, 2 the records view 1, 7, 0xa,
/// 0xd, 0x10, 0x13 and 0x16 the views of the positions, attendances, results, sequences, players and transfers tabs (1,
/// 3, 7, 0xc, 0x10 or 0x20 with `players_tab`, 0x1c), 0x20 the landmarks page 1, 0x23 the page count 1, the rest 0.
/// With `restore`, params 1, 2, 7, 0xa, 0xd, 0x10, 0x13 and 0x16 come back from the previous screen (its 60 params read
/// before opening).
/// Original: `open_club_history_screen` (0x469e70, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_club_history_screen(club: usize, restore: u32, players_tab: u32) {
    covered!("tel03_2 open club history");
    let mut prev = [0usize; 0x3c];
    if restore != 0 {
        for (k, v) in prev.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x46a1e0, 0x46dd20, 0, 0x46e560, 0) == 0 {
        return;
    }
    let p = club_history_planned_params(club, 0, players_tab, &prev);
    // the original's order of writes
    for k in [0usize, 1, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x20, 0x21, 0x22, 0x23] {
        set_screen_param(k as u32, p[k]);
    }
    if restore != 0 {
        for k in [1u32, 2, 7, 0xa, 0xd, 0x10, 0x13, 0x16] {
            set_screen_param(k, prev[k as usize]);
        }
    }
}

/// Builds the club history screen of param 0 (with no club, the club_screens.cpp assert box, line 0x2436): the section
/// picture 8, the title "{club} History" in the club's kit colours (`database_get_club_kit_colours` 1), the menu bar,
/// the club's action button, the tab bar of 9 tabs (Competitions 1, Landmarks 2, Records 3, Positions 4, Attendances
/// 5, Results 6, Sequences 7, Players 8, Transfers 9; for a national team Positions and Transfers are greyed out with
/// no value, and an open one of them goes back to Competitions, param 1 = 1), then the open tab (param 1) from the top
/// and bottom the tab bar leaves: Competitions, Positions, Attendances, Results, Sequences, Players and Transfers by
/// their own routines, Landmarks and Records here; any other value an empty area. At the end the squad navigation menu
/// (`build_club_squad_navigation_menu` 1, 0xc) and the Back/Next buttons. Raises 0x98ded8.
///
/// Convention: cdecl, SEH frame (0x1ec0 bytes of locals: 25 menu items built and destroyed by the CRT vector
/// iterators). Split into named pieces (`club_history_landmarks_tab`, `club_history_records_tab`); the whole routine
/// is hooked.
/// Original: `club_history_screen_build` (0x46a1e0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_screen_build() {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let tab_panel = |f: usize, top: i32, bottom: i32| {
        let g: unsafe extern "C" fn(i32, i32) = core::mem::transmute(f);
        g(top, bottom)
    };
    let mut tabs: MatchTabs = [0; 0x14b4 / 4];
    construct_match_tabs(&mut tabs);
    write_u32(CLUB_HISTORY_BUILT_FLAG, 1);
    let club = Club(get_screen_param(0));
    if club.is_null() {
        covered!("tel03_2 club history with no club");
        club_history_error_box(0x2436);
        destruct_match_tabs(&mut tabs);
        return;
    }
    covered!("tel03_2 club history build");
    let mut tab = get_screen_param(1) as u8;
    exe::background_set_section_picture(8, 0);
    let (mut fore, mut back) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut fore, &mut back);
    let gender = (club.short_gender_name() as i8) as i32;
    lang(HISTORY_TEXT, 0x990ac8, gender, gender, club.addr() + club::SHORT_NAME); // "{}<%s - club name>{} History"
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, HISTORY_TEXT);
    exe::menubar_build(4, 0);
    exe::add_club_screen_action_button(club, back, fore);

    let titles: [usize; 9] = [0x98ed18, 0x990abc, 0x990ab4, 0x990aa8, 0x990a9c, 0x990a94, 0x990a88, 0x990a80, 0x98e880];
    for (k, &title) in titles.iter().enumerate() {
        lang(match_tab_title(&mut tabs, k), title);
        // the national teams have no positions and no transfers: greyed out (the word at +2), no mark and no value
        if (k == 3 || k == 8) && exe::database_is_national_team(club) != 0 {
            if tab == k as u8 + 1 {
                set_screen_param(1, 1);
                tab = 1;
            }
            let base = tabs.as_mut_ptr() as usize + k * 0xd4;
            write_u16(base + 2, read_u16(G_GUI_COLOUR_GREY));
            set_match_tab_flags(&mut tabs, k, 0, 0);
        } else {
            set_match_tab_flags(&mut tabs, k, 2, k as u32 + 1);
        }
    }
    let (mut top, mut bottom) = (0x50i32, 0x221i32);
    exe::gui_add_tab_bar(tabs.as_mut_ptr() as usize, 9, tab as i8 as i32, &mut top, &mut bottom, 1);
    match tab {
        1 => tab_panel(0x46adc0, top, bottom),
        2 => club_history_landmarks_tab(top, bottom),
        3 => club_history_records_tab(top, bottom),
        4 => tab_panel(0x46c030, top, bottom),
        5 => tab_panel(0x46c510, top, bottom),
        6 => tab_panel(0x46c9e0, top, bottom),
        7 => tab_panel(0x46ceb0, top, bottom),
        8 => tab_panel(0x46d370, top, bottom),
        9 => tab_panel(0x46d860, top, bottom),
        _ => {
            let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
            add_area(current_human_display(), 0x6e, top as usize, 0x30c, bottom as usize, 1, 0, 1, 0, 2, 0, usize::MAX);
        }
    }
    exe::build_club_squad_navigation_menu(club, back, fore, 1, 0xc, 0, 0, 0);
    let next = (exe::screenmanager_is_forward_unavailable(G_SCREEN_MANAGER) == 0) as u32;
    let back_possible = (exe::screenmanager_is_back_unavailable(G_SCREEN_MANAGER, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(back_possible, next);
    destruct_match_tabs(&mut tabs);
}

/// The View popup of the Sequences tab: action 9 sets the record type (param 0x10); scroll row in param 0x11 from the
/// area of param 0x12.
/// Original: `club_history_sequences_tab_input` (0x46e260, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_sequences_tab_input(widget: u32) -> i32 {
    club_history_view_input(widget, 0x11, 0x12, 9, 0x10)
}

/// The View popup of the Players tab: action 0xa sets the record type (param 0x13); scroll row in param 0x14 from the
/// area of param 0x15.
/// Original: `club_history_players_tab_input` (0x46e360, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_players_tab_input(widget: u32) -> i32 {
    club_history_view_input(widget, 0x14, 0x15, 0xa, 0x13)
}

/// The View popup of the Transfers tab: action 0xb sets the record type (param 0x16); scroll row in param 0x17 from
/// the area of param 0x18.
/// Original: `club_history_transfers_tab_input` (0x46e460, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_transfers_tab_input(widget: u32) -> i32 {
    club_history_view_input(widget, 0x17, 0x18, 0xb, 0x16)
}

/// Answers a click on the club history screen. First by the open tab (param 1), each saving its area's scroll row
/// before (for a real widget): Competitions (scroll 0x1e from area 0x1f) action 3 (the item id on the current human's
/// Display) picks the period (param 0x1d); Landmarks (0x22 from 0x21) actions 0xd and 0xe page back and forth (param
/// 0x20, 1 to the page count of param 0x23); Records (3 from 4) action 4 picks the period (param 2), action 5 a record
/// type: `club_record_type_to_category` gives the tab (param 1) and the param of its view, which gets the type, the
/// next param 0; Positions (8 from 9) action 6 → param 7; Attendances (0xb from 0xc) action 7 → param 0xa; Results
/// (0xe from 0xf) action 8 → param 0xd; Sequences, Players and Transfers by their View popup routines. Then for every
/// tab: action 1 opens another club's history (the widget's club, restoring the views, −4); action 0xc opens the hall
/// of fame of the club's English name (`history_screen_open`, −4); action 2 changes the tab (param 1, −11); anything
/// else goes to the club screen actions, the menu bar commands and the object links, in that order.
/// Original: `club_history_screen_input` (0x46dd20, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_screen_input(widget: u32) -> i32 {
    let w = widget as i16;
    let club = get_screen_param(0);
    let tab = get_screen_param(1) as u8 as i8;
    let scroll = |save: u32, area: u32| {
        if w != -1 {
            set_screen_param(save, exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(area)));
        }
    };
    let answer = match tab {
        1 => {
            scroll(0x1e, 0x1f);
            let mut id = exe::display_get_widget_id(current_human_display(), widget);
            if id == 0 {
                id = read_u16(G_SCREEN_MANAGER + 0x26278e);
            }
            if id == 3 {
                club_history_pick(widget, 3, 3, 0x1d)
            } else {
                None
            }
        }
        2 => {
            scroll(0x22, 0x21);
            let page = get_screen_param(0x20) as i16 as i32;
            let pages = get_screen_param(0x23) as i16 as i32;
            let action = club_history_action(widget);
            if action == 0xd {
                set_screen_param(0x20, (page - 1).max(1) as usize);
                Some(-11)
            } else if action == 0xe {
                set_screen_param(0x20, (page + 1).min(pages) as usize); // tel03_2 the next page
                Some(-11)
            } else {
                None
            }
        }
        3 => {
            scroll(3, 4);
            let picked = club_history_pick(widget, club_history_action(widget), 4, 2);
            if picked.is_some() {
                picked
            } else if club_history_action(widget) == 5 {
                covered!("tel03_2 club history record type");
                let kind = exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget) as u8;
                // the original passes its argument slot as the out param: it starts with the widget
                let mut param: i32 = widget as i32;
                let category = exe::club_record_type_to_category(kind as u32, &mut param);
                set_screen_param(1, category as i8 as isize as usize);
                set_screen_param(param as u32, kind as i8 as isize as usize);
                set_screen_param((param + 1) as u32, 0);
                Some(-11)
            } else {
                None
            }
        }
        4 => {
            scroll(8, 9);
            club_history_pick(widget, club_history_action(widget), 6, 7)
        }
        5 => {
            scroll(0xb, 0xc);
            club_history_pick(widget, club_history_action(widget), 7, 0xa)
        }
        6 => {
            scroll(0xe, 0xf);
            club_history_pick(widget, club_history_action(widget), 8, 0xd)
        }
        7 | 8 | 9 => {
            let f: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(match tab {
                7 => 0x46e260usize,
                8 => 0x46e360usize,
                _ => 0x46e460usize,
            });
            let r = f(widget);
            if r != 0 {
                Some(r)
            } else {
                None
            }
        }
        _ => None,
    };
    if let Some(r) = answer {
        return r;
    }
    let action = club_history_action(widget);
    if action == 1 {
        covered!("tel03_2 club history of another club");
        exe::open_club_history_screen(exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget), 1, 0);
        return -4;
    }
    if club_history_action(widget) == 0xc {
        exe::history_screen_open(exe::club_get_english_name(club));
        return -4;
    }
    if let Some(r) = club_history_pick(widget, club_history_action(widget), 2, 1) {
        return r;
    }
    let r = exe::handle_club_screen_action(widget);
    if r != 0 {
        return r;
    }
    let r = exe::menubar_handle_command(widget);
    if r != 0 {
        return r;
    }
    exe::menubar_handle_object_link(widget)
}

/// Frees what the club history screen holds when it closes: the history lists of params 0x19..0x1c (`LLIST::~LLIST`
/// and delete, each param back to 0) and the record array of param 5 (`free`; params 5 and 6 back to 0).
/// Original: `club_history_screen_leave` (0x46e560, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_screen_leave() {
    for k in 0x19..=0x1cu32 {
        if get_screen_param(k) != 0 {
            let list = get_screen_param(k);
            if list != 0 {
                exe::drop_llist(list);
                exe::crt_operator_delete(list);
            }
            set_screen_param(k, 0);
        }
    }
    if get_screen_param(5) != 0 {
        covered!("tel03_2 club history leave with records");
        exe::crt_free(get_screen_param(5));
        set_screen_param(5, 0);
        set_screen_param(6, 0);
    }
}

/// Comparer for `open_club_history_screen`: opening a screen does not run twice; only the original runs and the new
/// screen's 36 params are checked against the plan (`club_history_planned_params`), when it holds the club in param 0.
pub(crate) unsafe extern "C" fn compare_open_club_history_screen(club: usize, restore: u32, players_tab: u32) {
    let mut prev = [0usize; 0x3c];
    if restore != 0 {
        for (k, v) in prev.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
    }
    let plan = club_history_planned_params(club, restore, players_tab, &prev);
    original_fn::<unsafe extern "C" fn(usize, u32, u32)>(IDX_OPEN_CLUB_HISTORY_SCREEN)(club, restore, players_tab);
    if get_screen_param(0) == club && club != 0 {
        let mut got = [0usize; 0x24];
        for (k, v) in got.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
        SCOREBOARD.record(got == plan, || format!("club history screen (open {:#x}): params {:x?} x plan {:x?}", club, got, plan));
    }
}

/// Comparer for `club_history_screen_build`: both build side by side and are compared by the screen (the display list,
/// `compare_match_screen_build`) plus the built flag 0x98ded8. When the open tab still has to build the lists it keeps
/// in the params (Competitions: params 0x19..0x1c; the season tabs: the record array of param 5), only the original
/// runs: building twice would leave two lists and the params would differ by the pointer.
pub(crate) unsafe extern "C" fn compare_club_history_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_CLUB_HISTORY_SCREEN_BUILD);
    let club = get_screen_param(0);
    let mut tab = get_screen_param(1) as u8;
    if club != 0 && (tab == 4 || tab == 9) && exe::database_is_national_team(club) != 0 {
        tab = 1;
    }
    let missing = |ks: &[u32]| ks.iter().any(|&k| get_screen_param(k) == 0);
    if club == 0 || (tab == 1 && missing(&[0x19, 0x1a, 0x1b, 0x1c])) || ((4..=9).contains(&tab) && missing(&[5])) {
        return o();
    }
    compare_match_screen_build("club history screen", &[(CLUB_HISTORY_BUILT_FLAG, 4)], || o(), || club_history_screen_build());
}

/// Comparer for `club_history_screen_input`: the clicks that only write params run side by side (the params and the
/// answer, `compare_resposta`); the ones that open a screen or go to the menu bar run the original only.
pub(crate) unsafe extern "C" fn compare_club_history_screen_input(widget: u32) -> i32 {
    let stays = club_history_click_stays(widget);
    compare_resposta("club history screen input", widget, original_fn(IDX_CLUB_HISTORY_SCREEN_INPUT), club_history_screen_input, !stays)
}

/// Comparer for `club_history_sequences_tab_input`: side by side over the params and the answer.
pub(crate) unsafe extern "C" fn compare_club_history_sequences_tab_input(widget: u32) -> i32 {
    compare_resposta("club history sequences tab input", widget, original_fn(IDX_CLUB_HISTORY_SEQUENCES_TAB_INPUT), club_history_sequences_tab_input, false)
}

/// Comparer for `club_history_players_tab_input`: side by side over the params and the answer.
pub(crate) unsafe extern "C" fn compare_club_history_players_tab_input(widget: u32) -> i32 {
    compare_resposta("club history players tab input", widget, original_fn(IDX_CLUB_HISTORY_PLAYERS_TAB_INPUT), club_history_players_tab_input, false)
}

/// Comparer for `club_history_transfers_tab_input`: side by side over the params and the answer.
pub(crate) unsafe extern "C" fn compare_club_history_transfers_tab_input(widget: u32) -> i32 {
    compare_resposta("club history transfers tab input", widget, original_fn(IDX_CLUB_HISTORY_TRANSFERS_TAB_INPUT), club_history_transfers_tab_input, false)
}

/// Comparer for `club_history_screen_leave`: freeing does not run twice; only the original runs and the params it
/// clears are checked afterwards (0x19..0x1c, and 5 and 6 when there was a record array).
pub(crate) unsafe extern "C" fn compare_club_history_screen_leave() {
    let records = get_screen_param(5) != 0;
    original_fn::<unsafe extern "C" fn()>(IDX_CLUB_HISTORY_SCREEN_LEAVE)();
    let mut left = [0usize; 6];
    for (i, k) in [0x19u32, 0x1a, 0x1b, 0x1c, 5, 6].iter().enumerate() {
        left[i] = get_screen_param(*k);
    }
    let cleared = left[..4].iter().all(|&v| v == 0) && (!records || (left[4] == 0 && left[5] == 0));
    SCOREBOARD.record(cleared, || format!("club history screen leave: params 0x19..0x1c, 5, 6 left {:x?}", left));
}

/// Draws the Competitions tab of the club history screen from `top` to `bottom`. The View popup: Honours (link 3, −1),
/// Domestic Leagues (−2) and one entry per league and per cup the club played (the league and cup history lists,
/// params 0x19 and 0x1b, and their competition id lists, 0x1a and 0x1c, built by `Club_History` and kept in the params
/// on the first draw); the chosen view (param 0x1d) is the heading, a link to the competition (0x7d0) when it is a loaded
/// one. Then, in the rows that fit in font 2: Domestic Leagues or a league division, the league table
/// (`history_competitions_league_table`); a cup, the cup runs (`history_competitions_cup_runs`); Honours, the honours
/// (`history_competitions_honours`). The scroll row comes back from param 0x1e and the area goes to param 0x1f.
///
/// Convention: cdecl(top, bottom), SEH frame (0x818 bytes of locals: the view label and a Text). Split into named
/// pieces; the whole routine is hooked.
/// Original: `club_history_draw_competitions_tab` (0x46adc0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_draw_competitions_tab(top: i32, bottom: i32) {
    let button_item: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    covered!("tel03_3 competitions tab");
    let table_weights: [u8; 10] = [4, 2, 9, 3, 3, 3, 3, 3, 3, 3];
    let weights: [u8; 5] = [1, 8, 4, 3, 8];
    let club = get_screen_param(0);
    let mut leagues = get_screen_param(0x19);
    let mut league_list = get_screen_param(0x1a);
    let mut cups = get_screen_param(0x1b);
    let mut cup_list = get_screen_param(0x1c);
    let view = get_screen_param(0x1d) as i32;
    let popup = history_tab_view_button(top);
    // the heading stays as it was when no entry matches the view (stack garbage in the original, empty here)
    let mut heading = [0u8; 0x7d0];
    lang(HISTORY_TAB_TEXT, 0x990b90); // "Honours"
    history_tab_menu_entry(popup, 1, 3, usize::MAX);
    if view == -1 {
        history_tab_copy(&mut heading, HISTORY_TAB_TEXT);
    }
    let history = read_usize(G_CLUB_HISTORY);
    if leagues == 0 {
        leagues = exe::club_history_get_league_history(history, club);
        set_screen_param(0x19, leagues);
    }
    if league_list == 0 {
        league_list = exe::club_history_get_league_comp_ids(history, leagues);
        set_screen_param(0x1a, league_list);
    }
    if league_list != 0 {
        lang(HISTORY_TAB_TEXT, 0x990b7c); // "Domestic Leagues"
        history_tab_menu_entry(popup, 1, 3, (-2isize) as usize);
        if view == -2 {
            history_tab_copy(&mut heading, HISTORY_TAB_TEXT);
        }
        history_competitions_menu_list(league_list, popup, view, &mut heading);
    }
    if cups == 0 {
        cups = exe::club_history_get_cup_history(history, club);
        set_screen_param(0x1b, cups);
    }
    if cup_list == 0 {
        cup_list = exe::club_history_get_cup_comp_ids(history, cups);
        set_screen_param(0x1c, cup_list);
    }
    if cup_list != 0 {
        history_competitions_menu_list(cup_list, popup, view, &mut heading);
    }
    let mut y = top + 0x19;
    if view > -1 && history_tab_comp_loaded(view as usize) {
        button_item(G_SCREEN_MANAGER, 0x6e, y as usize, 0x30c, (y + 0x23) as usize, 2, 0, 0xc, 3, colour_word(G_GUI_COLOUR_YELLOW), heading.as_ptr() as usize, 0x7d0, history_tab_comp(view as usize));
    } else {
        history_tab_heading(y, heading.as_ptr() as usize);
    }
    y += 0x28;
    let rows = (((bottom - y) / exe::font_get_height(2)) as u32) & 0xff;
    let area = if view == -2 || (view > -1 && exe::comp_util_is_league_division(view) != 0) {
        history_competitions_league_table(club, view, y, bottom, rows, &table_weights, leagues)
    } else if view > -1 && exe::comp_util_is_league_division(view) == 0 {
        history_competitions_cup_runs(club, view, y, bottom, rows, &weights, cups)
    } else {
        history_competitions_honours(club, y, bottom, rows, &weights)
    };
    history_tab_scroll(area, 0x1e, 0x1f);
}

/// Draws the Positions tab of the club history screen: the season tab of category 4 (view param 7, scroll 8, area 9),
/// in the records tab's period (param 2).
/// Original: `club_history_draw_positions_tab` (0x46c030, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_draw_positions_tab(top: i32, bottom: i32) {
    covered!("tel03_3 positions tab");
    history_season_tab(top, bottom, 4, 7, 8, 9, HistoryPeriod::Param, HistoryDefault::Keep);
}

/// Draws the Attendances tab of the club history screen: the season tab of category 5 (view param 0xa, scroll 0xb,
/// area 0xc), period 1, type 3 when the stored one does not apply.
/// Original: `club_history_draw_attendances_tab` (0x46c510, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_draw_attendances_tab(top: i32, bottom: i32) {
    history_season_tab(top, bottom, 5, 0xa, 0xb, 0xc, HistoryPeriod::Fixed(1), HistoryDefault::Type(3)); // tel03_3 the attendances default type
}

/// Draws the Results tab of the club history screen: the season tab of category 6 (view param 0xd, scroll 0xe, area
/// 0xf), period 1, type 7 when the stored one does not apply.
/// Original: `club_history_draw_results_tab` (0x46c9e0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_draw_results_tab(top: i32, bottom: i32) {
    history_season_tab(top, bottom, 6, 0xd, 0xe, 0xf, HistoryPeriod::Fixed(1), HistoryDefault::Type(7));
}

/// Draws the Sequences tab of the club history screen: the season tab of category 7 (view param 0x10, scroll 0x11, area
/// 0x12), period 1.
/// Original: `club_history_draw_sequences_tab` (0x46ceb0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_draw_sequences_tab(top: i32, bottom: i32) {
    history_season_tab(top, bottom, 7, 0x10, 0x11, 0x12, HistoryPeriod::Fixed(1), HistoryDefault::Keep);
}

/// Draws the Players tab of the club history screen: the season tab of category 8 (view param 0x13, scroll 0x14, area
/// 0x15), period 2, the first type that applies when the stored one does not.
/// Original: `club_history_draw_players_tab` (0x46d370, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_draw_players_tab(top: i32, bottom: i32) {
    history_season_tab(top, bottom, 8, 0x13, 0x14, 0x15, HistoryPeriod::Fixed(2), HistoryDefault::FirstThatApplies);
}

/// Draws the Transfers tab of the club history screen: the season tab of category 9 (view param 0x16, scroll 0x17, area
/// 0x18), period 1.
/// Original: `club_history_draw_transfers_tab` (0x46d860, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_history_draw_transfers_tab(top: i32, bottom: i32) {
    history_season_tab(top, bottom, 9, 0x16, 0x17, 0x18, HistoryPeriod::Fixed(1), HistoryDefault::Keep);
}

/// Comparer for `club_history_draw_competitions_tab` (lists in params 0x19..0x1c).
pub(crate) unsafe extern "C" fn compare_club_history_draw_competitions_tab(top: i32, bottom: i32) {
    compare_history_tab("club history competitions tab", &[0x19, 0x1a, 0x1b, 0x1c], original_fn(IDX_CLUB_HISTORY_DRAW_COMPETITIONS_TAB), club_history_draw_competitions_tab, top, bottom)
}

/// Comparer for `club_history_draw_positions_tab` (record array in param 5).
pub(crate) unsafe extern "C" fn compare_club_history_draw_positions_tab(top: i32, bottom: i32) {
    compare_history_tab("club history positions tab", &[5], original_fn(IDX_CLUB_HISTORY_DRAW_POSITIONS_TAB), club_history_draw_positions_tab, top, bottom)
}

/// Comparer for `club_history_draw_attendances_tab` (record array in param 5).
pub(crate) unsafe extern "C" fn compare_club_history_draw_attendances_tab(top: i32, bottom: i32) {
    compare_history_tab("club history attendances tab", &[5], original_fn(IDX_CLUB_HISTORY_DRAW_ATTENDANCES_TAB), club_history_draw_attendances_tab, top, bottom)
}

/// Comparer for `club_history_draw_results_tab` (record array in param 5).
pub(crate) unsafe extern "C" fn compare_club_history_draw_results_tab(top: i32, bottom: i32) {
    compare_history_tab("club history results tab", &[5], original_fn(IDX_CLUB_HISTORY_DRAW_RESULTS_TAB), club_history_draw_results_tab, top, bottom)
}

/// Comparer for `club_history_draw_sequences_tab` (record array in param 5).
pub(crate) unsafe extern "C" fn compare_club_history_draw_sequences_tab(top: i32, bottom: i32) {
    compare_history_tab("club history sequences tab", &[5], original_fn(IDX_CLUB_HISTORY_DRAW_SEQUENCES_TAB), club_history_draw_sequences_tab, top, bottom)
}

/// Comparer for `club_history_draw_players_tab` (record array in param 5).
pub(crate) unsafe extern "C" fn compare_club_history_draw_players_tab(top: i32, bottom: i32) {
    compare_history_tab("club history players tab", &[5], original_fn(IDX_CLUB_HISTORY_DRAW_PLAYERS_TAB), club_history_draw_players_tab, top, bottom)
}

/// Comparer for `club_history_draw_transfers_tab` (record array in param 5).
pub(crate) unsafe extern "C" fn compare_club_history_draw_transfers_tab(top: i32, bottom: i32) {
    compare_history_tab("club history transfers tab", &[5], original_fn(IDX_CLUB_HISTORY_DRAW_TRANSFERS_TAB), club_history_draw_transfers_tab, top, bottom)
}

/// Answers a click on the club screen's next match tab (param 0 the club, 1 the reserves flag, 3 the shown fixture, a
/// copy the screen owns): action 0x1d opens the league progress of the fixture (−4); 0x1e the past meetings with the
/// widget's club, or with the fixture's other club (−4); 0x26 and 0x27 move to the previous and the next fixture of the
/// club (`club_next_match_neighbour`): a new 0x4f-byte copy goes to param 3 (the old one is not freed, as in the
/// original) and the answer is −11, or 0 when there is none. Anything else: 0.
/// Original: `club_screen_next_match_tab_input` (0x468c10, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screen_next_match_tab_input(widget: u32) -> i32 {
    let w = widget as i16;
    let club = get_screen_param(0);
    let reserves = get_screen_param(1);
    let fixture = Fixture(get_screen_param(3));
    let action = club_tabs_action(w);
    if action == 0x1d {
        exe::comp_open_league_progress_for_fixture(fixture);
        return -4;
    }
    if action == 0x1e {
        let other = club_tabs_value(w);
        let other = if other != 0 {
            other
        } else if club != fixture.home_club().addr() {
            fixture.home_club().addr()
        } else {
            fixture.away_club().addr()
        };
        exe::open_past_meetings_screen(club, other, reserves);
        return -4;
    }
    if action == 0x26 || action == 0x27 {
        covered!("tel03_4 next match neighbour");
        return match club_next_match_neighbour(club, reserves, fixture.addr(), action == 0x27) {
            None => 0,
            Some(c) => {
                let copy = exe::crt_operator_new(fixture::SIZE as u32);
                for (k, b) in c.iter().enumerate() {
                    write_u8(copy + k, *b);
                }
                set_screen_param(3, copy);
                -11
            }
        };
    }
    0
}

/// Answers a click on the club screen's fixtures tab (param 0 the club, 0xc the season page, 0xd the fixtures list, 0xe
/// the cancel mode, 0x1e what a cancellation box is about). First the answer of a cancellation box: 0x10/0x11 open the
/// tournament invitations of the answer (−4); 0x12 (a friendly), 0x13 (a proposed friendly), 0x14 (a tour) and 0x15 (a
/// tournament) cancel it on yes (`Friendly::on_friendly_fixture_cancelled`, `remove_friendly_match`,
/// `cancel_friendly_dates_request`, `cancel_tournament`), the friendly's copy is deleted, and param 0x1e goes back to 0.
/// Then the click (a real widget saves the scroll row of param 0x11 into param 0x10): 0x10/0x11 the season page down or
/// up (param 0xc; the list of param 0xd freed, −11). In cancel mode: 0x15 a friendly, 0x16 a proposed friendly, 0x17 a
/// tour and 0x18 a tournament the club controls and hosts open the Please Confirm box (codes 0x12..0x15) after leaving
/// the mode (−4); any other widget only leaves the mode (−11). Otherwise: 0x12 switches the cancel mode (−11); 0x13
/// arranges a friendly (a tour, kind 0xb, opens the tour nation screen) (−4); 0x14 opens the competition at the
/// fixture's date (−4); 0x17 the tour's clubs screen (the assert box without a tour) (−4); 0x18 the tournament
/// invitations (−4). Anything else: 0.
/// Original: `club_screen_fixtures_tab_input` (0x4691d0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screen_fixtures_tab_input(widget: u32) -> i32 {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let invitations: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_OPEN_TOURNAMENT_INVITATIONS_SCREEN);
    let w = widget as i16;
    let club = get_screen_param(0);
    let page = get_screen_param(0xc);
    let list = get_screen_param(0xd);
    let cancel_mode = get_screen_param(0xe);
    if read_u32(CLUB_TABS_DIALOG_ANSWERED) != 0 {
        let code = read_u32(G_PENDING_COMMAND_CODE);
        if code == 0x11 || code == 0x10 {
            covered!("tel03_4 fixtures tournament invitations answer");
            let answer = read_usize(G_PENDING_COMMAND_ARG);
            write_u32(G_PENDING_COMMAND_CODE, 0);
            write_u32(G_PENDING_COMMAND_ARG, 0);
            invitations(answer, 1);
            return -4;
        }
        let friendly = read_usize(G_FRIENDLY);
        let yes = || read_u32(G_PENDING_COMMAND_ARG) == 2;
        match code {
            0x12 => {
                write_u32(G_PENDING_COMMAND_CODE, 0);
                let copy = get_screen_param(0x1e);
                if yes() {
                    exe::friendly_on_friendly_fixture_cancelled(friendly, copy, (club) as usize as u32);
                }
                exe::crt_operator_delete(copy);
                set_screen_param(0x1e, 0);
                write_u32(G_PENDING_COMMAND_ARG, 0);
            }
            0x13 => {
                write_u32(G_PENDING_COMMAND_CODE, 0);
                let p = get_screen_param(0x1e);
                if yes() {
                    exe::friendly_remove_friendly_match(friendly, p, 1);
                }
                set_screen_param(0x1e, 0);
                write_u32(G_PENDING_COMMAND_ARG, 0);
            }
            0x14 => {
                write_u32(G_PENDING_COMMAND_CODE, 0);
                let p = get_screen_param(0x1e);
                if yes() {
                    exe::friendly_cancel_friendly_dates_request(friendly, p);
                }
                set_screen_param(0x1e, 0);
                write_u32(G_PENDING_COMMAND_ARG, 0);
            }
            0x15 => {
                write_u32(G_PENDING_COMMAND_CODE, 0);
                let p = get_screen_param(0x1e);
                if yes() {
                    exe::friendly_cancel_tournament(friendly, p, 1);
                }
                set_screen_param(0x1e, 0);
                write_u32(G_PENDING_COMMAND_ARG, 0);
            }
            _ => {}
        }
    }
    if w != -1 {
        set_screen_param(0x10, exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(0x11)));
    }
    let action = club_tabs_action(w);
    if action == 0x10 || action == 0x11 {
        let page = (page & 0xffff) as i32 + if action == 0x10 { -1 } else { 1 }; // tel03_4 the season page step
        set_screen_param(0xc, page as usize);
        if list != 0 {
            exe::drop_llist(list);
            exe::crt_operator_delete(list);
            set_screen_param(0xd, 0);
        }
        return -11;
    }
    let mut colours = (0u32, 0u32);
    let mut date = [0u8; 0xc8];
    if cancel_mode != 0 {
        match action {
            0x15 => {
                covered!("tel03_4 fixtures cancel friendly");
                set_screen_param(0xe, 0);
                let copy = exe::crt_operator_new(fixture::SIZE as u32);
                let source = club_tabs_value(w);
                for k in 0..fixture::SIZE {
                    write_u8(copy + k, read_u8(source + k));
                }
                set_screen_param(0x1e, copy);
                let home = read_usize(copy + fixture::HOME_CLUB);
                let other = if club == home { read_usize(copy + fixture::AWAY_CLUB) } else { home };
                let mut d = [0u8; 8];
                let at = exe::new_date((&mut d) as *mut [u8; 8] as usize, read_u16(copy + fixture::DAY) as u32, read_u16(copy + fixture::YEAR) as u32);
                club_fixtures_long_date(at, &mut date);
                exe::database_get_club_kit_colours(club, 1, &mut colours.0, &mut colours.1);
                let gender = read_i8(other + club::SHORT_GENDER_NAME) as i32;
                // "Cancel the friendly match against {}<%s - club>{} on {}<%s - date> ?" (and the other text for a
                // friendly with a stage byte at +0x42)
                let text = if read_u8(copy + 0x42) == 0xff { 0x990a38 } else { 0x9909c8 };
                lang(CLUB_TABS_TEXT, text, gender, gender, 0x64, other + club::SHORT_NAME, date.as_ptr() as usize);
                club_fixtures_confirm(0x9909a8, CLUB_TABS_TEXT, colours, 0x12); // "Cancel Friendly"
                return -4;
            }
            0x16 => {
                set_screen_param(0xe, 0);
                let p = club_tabs_value(w);
                set_screen_param(0x1e, p);
                let mut other = read_usize(p + 0x10);
                if club == other {
                    other = read_usize(p + 0x14);
                }
                club_fixtures_long_date(p, &mut date);
                exe::database_get_club_kit_colours(club, 1, &mut colours.0, &mut colours.1);
                let gender = read_i8(other + club::SHORT_GENDER_NAME) as i32;
                // "Cancel the proposed friendly match against {}<%s - club>{} on {}<%s - date> ?"
                let message = lang(CLUB_TABS_TEXT, 0x990958, gender, gender, 0x64, other + club::SHORT_NAME, date.as_ptr() as usize);
                club_fixtures_confirm(0x9909a8, message, colours, 0x13); // "Cancel Friendly"
                return -4;
            }
            0x17 => {
                set_screen_param(0xe, 0);
                let p = club_tabs_value(w);
                set_screen_param(0x1e, p);
                let start = p + 9;
                club_fixtures_long_date(start, &mut date);
                exe::database_get_club_kit_colours(club, 1, &mut colours.0, &mut colours.1);
                let today = read_u32(G_CURRENT_DATE);
                let (today_day, today_year) = (today as u16 as i16, (today >> 16) as u16);
                let start_year = read_u16(start + 2);
                let started = today_year > start_year || (today_year == start_year && today_day > read_i16(start));
                let nation = Nation(read_usize(p + 4));
                let gender = (nation.short_gender_name() as i8) as i32;
                // "Cancel the tour of {}<%s - nation>{} which started on / starts on {}<%s - date> ?"
                lang(CLUB_TABS_TEXT, if started { 0x990910 } else { 0x9908c8 }, gender, gender, 0x64, nation.addr() + nation::SHORT_NAME, date.as_ptr() as usize);
                club_fixtures_confirm(0x9908bc, CLUB_TABS_TEXT, colours, 0x14); // "Cancel Tour"
                return -4;
            }
            0x18 => {
                set_screen_param(0xe, 0);
                let p = club_tabs_value(w);
                if exe::human_manager_current_controls_club(club, 0, 1) != 0 && club == read_usize(p) {
                    set_screen_param(0x1e, p);
                    club_fixtures_long_date(p + 4, &mut date);
                    exe::database_get_club_kit_colours(club, 1, &mut colours.0, &mut colours.1);
                    // "Cancel the <%s - competition> tournament which starts on {}<%s - date> ?"
                    lang(CLUB_TABS_TEXT, 0x990870, 0x64, p + 0x31, date.as_ptr() as usize);
                    club_fixtures_confirm(0x99085c, CLUB_TABS_TEXT, colours, 0x15); // "Cancel Tournament"
                    return -4;
                }
                // not a tournament the club hosts: on to the ordinary answers
            }
            _ => {
                if w != -1 {
                    set_screen_param(0xe, 0);
                    return -11;
                }
            }
        }
    }
    let action = club_tabs_action(w);
    match action {
        0x12 => {
            set_screen_param(0xe, (cancel_mode == 0) as usize); // tel03_4 the cancel mode switch
            -11
        }
        0x13 => {
            let value = club_tabs_value(w);
            if w != -1 && value as u8 == 0xb {
                let tour: unsafe extern "C" fn(usize) = core::mem::transmute(FN_OPEN_FRIENDLY_TOUR_NATION_SCREEN);
                tour(club);
            } else {
                exe::open_arrange_friendly_screen(club, value, 0);
            }
            -4
        }
        0x14 => {
            let f = Fixture(club_tabs_value(w));
            let by_stage = ((f.home_goals() as u8) != 0xff) as u32;
            exe::comp_screens_open_comp_fixtures(f.competition().addr(), f.day() as u32, f.year() as u32, by_stage);
            -4
        }
        0x17 => {
            let tour = club_tabs_value(w);
            if tour == 0 {
                covered!("tel03_4 fixtures tour assert");
                club_tabs_error_box(0x33a3);
                return -4;
            }
            if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x472380, 0x472850, 1, 0, 0) != 0 {
                set_screen_param(0, tour);
                set_screen_param(1, 0);
                set_screen_param(2, 0);
                write_u32(tour + 0x103, 1);
            }
            -4
        }
        0x18 => {
            invitations(club_tabs_value(w), 1);
            -4
        }
        _ => 0,
    }
}

/// Frees what the club screen holds when it closes: the buffers of params 2 and 3 (delete; 3 is the next match tab's
/// fixture copy), the fixtures list of param 0xd (`LLIST::~LLIST` and delete) and the array of param 0x15 (`free`, with
/// param 0x16), each param back to 0. The four params are read first.
/// Original: `club_screen_leave` (0x469da0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screen_leave() {
    let a = get_screen_param(2);
    let b = get_screen_param(3);
    let list = get_screen_param(0xd);
    let array = get_screen_param(0x15);
    if a != 0 {
        exe::crt_operator_delete(a);
        set_screen_param(2, 0);
    }
    if b != 0 {
        exe::crt_operator_delete(b);
        set_screen_param(3, 0);
    }
    if list != 0 {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
        set_screen_param(0xd, 0);
    }
    if array != 0 {
        exe::crt_free(array);
        set_screen_param(0x15, 0);
        set_screen_param(0x16, 0);
    }
}

/// Comparer for `club_screen_next_match_tab_input`: answers that open a screen run the original only; Previous and
/// Next (0x26/0x27) also run the original only (they allocate the new copy), and the fixture it leaves in param 3 is
/// checked against the one the copy picks (`club_next_match_neighbour`), or "none" against an answer of 0.
pub(crate) unsafe extern "C" fn compare_club_screen_next_match_tab_input(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_CLUB_SCREEN_NEXT_MATCH_TAB_INPUT);
    let action = club_tabs_action(widget as i16);
    if action != 0x26 && action != 0x27 {
        return o(widget);
    }
    let old = get_screen_param(3);
    let plan = club_next_match_neighbour(get_screen_param(0), get_screen_param(1), old, action == 0x27);
    let r = o(widget);
    let now = get_screen_param(3);
    let ok = match plan {
        None => r == 0 && now == old,
        Some(c) => r == -11 && now != 0 && c.iter().enumerate().all(|(k, b)| read_u8(now + k) == *b),
    };
    SCOREBOARD.record(ok, || format!("club screen next match (action {:#x}): answer {} param 3 {:#x} (was {:#x}), plan {}", action, r, now, old, if plan.is_some() { "a fixture" } else { "none" }));
    r
}

/// Comparer for `club_screen_fixtures_tab_input`: the clicks that only write params (switching or leaving the cancel
/// mode) run side by side (`compare_resposta`); the season page runs the original only (it frees the list) and the
/// page and the list param are checked afterwards; a pending box answer and everything that opens a screen or a box
/// run the original only.
pub(crate) unsafe extern "C" fn compare_club_screen_fixtures_tab_input(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_CLUB_SCREEN_FIXTURES_TAB_INPUT);
    let w = widget as i16;
    let pending = read_u32(CLUB_TABS_DIALOG_ANSWERED) != 0 && (0x10..=0x15).contains(&read_u32(G_PENDING_COMMAND_CODE));
    if pending || w == -1 {
        return o(widget);
    }
    let action = club_tabs_action(w);
    if action == 0x10 || action == 0x11 {
        let page = get_screen_param(0xc);
        let r = o(widget);
        let want = ((page & 0xffff) as i32 + if action == 0x10 { -1 } else { 1 }) as usize;
        let ok = r == -11 && get_screen_param(0xc) == want && get_screen_param(0xd) == 0;
        SCOREBOARD.record(ok, || format!("club screen fixtures page (action {:#x}): answer {} page {:#x} (want {:#x}) list {:#x}", action, r, get_screen_param(0xc), want, get_screen_param(0xd)));
        return r;
    }
    let cancel_mode = get_screen_param(0xe) != 0;
    let stays = if cancel_mode { !(0x15..=0x18).contains(&action) } else { action == 0x12 };
    compare_resposta("club screen fixtures tab input", widget, o, club_screen_fixtures_tab_input, !stays)
}

/// Comparer for `club_screen_leave`: freeing does not run twice; only the original runs and the params it clears are
/// checked afterwards.
pub(crate) unsafe extern "C" fn compare_club_screen_leave() {
    let had = [get_screen_param(2), get_screen_param(3), get_screen_param(0xd), get_screen_param(0x15)];
    original_fn::<unsafe extern "C" fn()>(IDX_CLUB_SCREEN_LEAVE)();
    let left = [get_screen_param(2), get_screen_param(3), get_screen_param(0xd), get_screen_param(0x15), get_screen_param(0x16)];
    let ok = (0..4).all(|k| had[k] == 0 || left[k] == 0) && (had[3] == 0 || left[4] == 0);
    SCOREBOARD.record(ok, || format!("club screen leave: params 2, 3, 0xd, 0x15, 0x16 left {:x?}", left));
}

/// Opens the past meetings screen of `club` against `opposition` (builder `past_meetings_screen_build`, handler
/// `past_meetings_screen_input`); either club missing shows the club_screens.cpp 11728 box instead. Params: 0 the club,
/// 1 the opposition, 2 the filter 0x221 (all time, anywhere, ignore friendlies), 3 the slot's past meetings cache, 4
/// and 5 the scroll row and area (0), 6 the reserves flag.
/// Original: `open_past_meetings_screen` (0x46e6b0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_past_meetings_screen(club: usize, opposition: usize, reserves: u32) {
    if club == 0 || opposition == 0 {
        club_pages_error_box(0x2dd0);
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x46e820usize, 0x46ffd0usize, 0, 0usize, 0) == 0 {
        return;
    }
    covered!("tel04_1 open past meetings");
    let cache = past_meetings_cache_of_slot();
    for (k, v) in [(0u32, club), (1, opposition), (2, 0x221), (3, cache), (4, 0), (5, 0), (6, reserves as usize)] {
        set_screen_param(k, v);
    }
}

/// Builds the past meetings screen of club (param 0) against the opposition (param 1).
///
/// The club's background picture, its name (with the reserves flag, param 6) in its kit colours as the title, "Past
/// Meetings With <opposition>" as the subtitle and the menu bar. The View popup: All Time (bit 1), Last 3, 5, 10 and 25
/// Years (2, 4, 8, 0x10); the Filter popup: Anywhere (0x20), At <club's stadium or club> (0x40), At <opposition's>
/// (0x80), On Neutral Ground (0x100), a separator and Ignore Friendlies (0x400, its value 0x200 when ticked); the ticked
/// choices are the bits of the filter (param 2). When the cache (param 3) was made for another day, clubs, reserves
/// flag or filter, the search runs again (`past_meetings_search`). Then the head-to-head table (Pld, Won, Drn, Lst,
/// For, Ag, Pts; a row per club of the sorted table: place, linked name in yellow for a human's club, the figures) and
/// one row per meeting: the date (linked to the fixture when the competition is linked), the home club, the score (the
/// final one when there is one; linked to the match when it has stats), the away club, H/A/N for the club and the
/// round (linked to the competition); empty rows fill the area, whose scroll row comes from param 4 and goes to param 5;
/// the Back and Next buttons.
/// Original: `past_meetings_screen_build` (0x46e820, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn past_meetings_screen_build() {
    let lang = club_pages_lang();
    let format = club_pages_format();
    let table_weights: [u8; 11] = [4, 2, 0x10, 0, 4, 4, 4, 4, 4, 4, 4];
    let list_weights: [u8; 7] = [3, 7, 2, 7, 1, 10, 3];
    let club = get_screen_param(0);
    let opposition = Club(get_screen_param(1));
    let filter = get_screen_param(2) as u32;
    let cache = get_screen_param(3);
    let reserves = get_screen_param(6);
    covered!("tel04_1 past meetings build");
    exe::background_set_club_picture(club, 0, 0);
    exe::club_get_national_team_name(club, reserves, JOB_TEXT, 0x7d0, 3, 1);
    club_pages_title(club, JOB_TEXT);
    let c = (opposition.short_gender_name() as i8) as i32;
    lang(JOB_TEXT, 0x990c84, c, c, opposition.addr() + club::SHORT_NAME); // "Past Meetings With {}<%s - opposition club>{}"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    exe::menubar_build(4, 0);
    let (grey, pale) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY));
    let blue = job_colour(0xae31aa);

    lang(JOB_TEXT, 0x988004); // "View"
    let button = tel26_add_button([0x6e, 0x91, 0xeb, 0xa5, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0, 0]);
    let view = club_pages_area([0, 0, 0, 0, 0, 0, 0, 0, 0x130, blue, button]);
    lang(JOB_TEXT, 0x990c78); // " All Time"
    past_meetings_choice(filter & 1 != 0, (filter & 1 == 0) as u32, (filter & 1 == 0) as u32, view);
    for (bit, name) in [(2u32, 0x990c68usize), (4, 0x990c58), (8, 0x990c48), (0x10, 0x990c38)] {
        lang(JOB_TEXT, name); // " Last 3/5/10/25 Years"
        let on = filter & bit != 0;
        past_meetings_choice(on, !on as u32, if on { 0 } else { bit }, view);
    }
    lang(JOB_TEXT, 0x98ebf8); // "Filter"
    let button = tel26_add_button([0x28f, 0x91, 0x30c, 0xa5, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0, 0]);
    let venue = club_pages_area([0, 0, 0, 0, 0, 0, 0, 0, 0x130, blue, button]);
    lang(JOB_TEXT, 0x990c2c); // " Anywhere"
    let on = filter & 0x20 != 0;
    past_meetings_choice(on, !on as u32, if on { 0 } else { 0x20 }, venue);
    for (bit, who) in [(0x40u32, club), (0x80, opposition.addr())] {
        past_meetings_at(who);
        let on = filter & bit != 0;
        past_meetings_choice(on, !on as u32, if on { 0 } else { bit }, venue);
    }
    lang(JOB_TEXT, 0x990bec); // " On Neutral Ground"
    let on = filter & 0x100 != 0;
    past_meetings_choice(on, !on as u32, if on { 0 } else { 0x100 }, venue);
    club_pages_item([1, 0, 0, 0, 0, 0, 0, 0x1000010, blue, job_colour(G_GUI_COLOUR_YELLOW), 1, 1, job_colour(G_GUI_COLOUR_BLACK), 0, 0, 0, 0, venue]);
    lang(JOB_TEXT, 0x990bd8); // " Ignore Friendlies"
    let on = filter & 0x400 != 0;
    past_meetings_choice(on, 1, if on { 0x200 } else { 0x400 }, venue);

    let today = read_u32(G_CURRENT_DATE);
    if read_u16(cache + 2) != (today >> 16) as u16 || read_u16(cache) != today as u16 || read_usize(cache + past_meetings_cache::CLUB) != club
        || read_usize(cache + past_meetings_cache::OPPOSITION) != opposition.addr() || read_usize(cache + past_meetings_cache::RESERVES) != reserves
        || read_u32(cache + past_meetings_cache::FILTER) != filter {
        covered!("tel04_1 past meetings search");
        past_meetings_search(cache, club, opposition.addr(), reserves, filter);
    }

    let h3 = exe::font_get_height(3);
    let table_area = club_pages_area([0x6e, 0xaa, 0x30c, (h3 * 3 + 0xba) as u32, 0xb, table_weights.as_ptr() as u32, 3, 0, 2, 0, u32::MAX]);
    for (col, title, arg) in [(4u32, 0x990b3cusize, 0usize), (5, 0x990b1c, 0xad9c64), (6, 0x990b14, 0), (7, 0x990af4, 0xad9c64), (8, 0x990af0, 0), (9, 0x990aec, 0), (10, 0x990ae8, 0)] {
        if arg != 0 {
            lang(JOB_TEXT, title, arg); // "Won" / "Lst"
        } else {
            lang(JOB_TEXT, title); // "Pld", "Drn", "For", "Ag", "Pts"
        }
        tel26_area_text([table_area, col, 0, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32]);
    }
    let table = read_usize(cache + past_meetings_cache::TABLE);
    if table != 0 {
        write_u32(table + 0xad, 0);
        exe::league_sort_table(table, 0usize);
        let mut k: i8 = 0;
        let mut row = 0u32;
        while (k as i16) < read_i16(read_usize(cache + past_meetings_cache::TABLE) + 0x3e) {
            let r = exe::league_get_table_row(read_usize(cache + past_meetings_cache::TABLE), k as i32);
            row += 1;
            let place = exe::utils_format_ordinal(k as i32 + 1, -1);
            tel26_area_text([table_area, 0, row, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, pale, place as u32]);
            let c = read_usize(r);
            let colour = if exe::human_manager_is_human_club(c, 0, 1, 0, 0) != 0 { job_colour(G_GUI_COLOUR_YELLOW) } else { pale };
            tel26_area_button([table_area, 2, row, 1, 0, 1, 3, colour, (c + club::SHORT_NAME) as u32, 0x7d5, c as u32]);
            let figures = [
                read_i8(r + 0x11) as i32,
                read_i8(r + 0x12) as i32,
                read_i8(r + 0x13) as i32,
                read_i8(r + 0x14) as i32,
                read_i16(r + 0x15) as i32,
                read_i16(r + 0x17) as i32,
            ];
            for (n, v) in figures.iter().enumerate() {
                format(JOB_TEXT, 0x98806c, *v); // "%d"
                tel26_area_text([table_area, 4 + n as u32, row, 1, 0, 0xc, 2, pale, JOB_TEXT as u32]);
            }
            format(JOB_TEXT, 0x98806c, read_i16(r + 0x19) as i32);
            tel26_area_text([table_area, 10, row, 0x30, job_colour(0xae3192), 0xc, 1, pale, JOB_TEXT as u32]);
            k = k.wrapping_add(1);
        }
    }

    let top = h3 * 3 + 0xbf;
    let rows = ((0x217 - top) / exe::font_get_height(2)) as u8 as u32;
    let area = club_pages_area([0x6e, top as u32, 0x30c, 0x217, 7, list_weights.as_ptr() as u32, rows, 0, 1, 0, u32::MAX]);
    let list = cache + past_meetings_cache::LIST;
    write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
    let mut row = 0u32;
    let (yellow, dark_blue) = (job_colour(G_GUI_COLOUR_YELLOW), job_colour(G_GUI_COLOUR_DARK_BLUE));
    loop {
        let f = exe::llist_get_next(list);
        if f == 0 {
            break;
        }
        let mut date = [0u32; 2];
        let d = club_pages_date(date.as_mut_ptr(), read_u16(f + screen_fixture::DAY), read_u16(f + screen_fixture::YEAR));
        club_pages_short_date(d, JOB_TEXT, 0x7d0, 0xff);
        if past_meetings_comp_linked(f) {
            tel26_area_button([area, 0, row, 0x30, dark_blue, 0xc, 1, pale, JOB_TEXT as u32, 2, f as u32]);
        } else {
            tel26_area_text([area, 0, row, 0x30, dark_blue, 0xc, 1, pale, JOB_TEXT as u32]);
        }
        for (col, side) in [(1u32, screen_fixture::HOME), (3, screen_fixture::AWAY)] {
            let c = read_usize(f + side);
            format(JOB_TEXT, 0x987ff0, c + club::SHORT_NAME); // "  %s"
            let colour = if exe::human_manager_is_human_club(c, 0, 1, 0, 0) != 0 { yellow } else { pale };
            tel26_area_button([area, col, row, 2, 0, 1, 2, colour, JOB_TEXT as u32, 0x7d5, c as u32]);
            if col == 1 {
                let (h, a) = if read_i8(f + screen_fixture::FINAL_HOME_GOALS) > -1 {
                    (read_i8(f + screen_fixture::FINAL_HOME_GOALS), read_i8(f + screen_fixture::FINAL_AWAY_GOALS))
                } else {
                    (read_i8(f + screen_fixture::HOME_GOALS), read_i8(f + screen_fixture::AWAY_GOALS))
                };
                lang(JOB_TEXT, 0x990bb4, h as i32, a as i32); // "<%d - home score>:<%d - away score>"
                let stats = exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), read_i32(f + screen_fixture::MATCH_ID), read_i32(f + 0xc), read_i32(f + 0x10),
                    read_u16(f + screen_fixture::DAY) as i32, read_u16(f + screen_fixture::YEAR) as i32);
                if stats != 0 {
                    tel26_area_button([area, 2, row, 2, 0, 0xc, 2, yellow, JOB_TEXT as u32, 0x7d3, read_u32(f + screen_fixture::MATCH_ID)]);
                } else {
                    tel26_area_text([area, 2, row, 2, 0, 0xc, 2, yellow, JOB_TEXT as u32]);
                }
            }
        }
        let venue = match exe::comp_util_get_fixture_venue_side(club, f) {
            1 => 0x98f880usize, // "H<%s - COMMENT - home>"
            2 => 0x98f868,      // "A<%s - COMMENT - away>"
            _ => 0x990b98,      // "N<%s - COMMENT - neutral>"
        };
        lang(JOB_TEXT, venue, 0xad9c64usize);
        tel26_area_text([area, 4, row, 2, 0, 0xc, 2, job_colour(0xaebde8), JOB_TEXT as u32]);
        let mut round = [0u8; 0xc8];
        exe::comp_util_format_fixture_round_label(f, 3, round.as_mut_ptr() as usize, 0xc8);
        format(JOB_TEXT, 0x987ff0, round.as_ptr());
        if past_meetings_comp_linked(f) {
            tel26_area_button([area, 5, row, 2, 0, 0xc, 2, job_colour(0xaebdd4), JOB_TEXT as u32, 0x7d0, read_u32(f + screen_fixture::COMP)]);
        } else {
            tel26_area_text([area, 5, row, 2, 0, 0xc, 2, job_colour(0xaebdd4), JOB_TEXT as u32]);
        }
        write_u8(JOB_TEXT, 0);
        tel26_area_text([area, 6, row, 0x30, job_colour(0xae3192), 0xc, 1, pale, JOB_TEXT as u32]);
        row += 1;
    }
    while row < rows {
        for col in 0..7u32 {
            tel26_area_text([area, col, row, 2, 0, 1, 2, pale, 0]);
        }
        row += 1;
    }
    club_pages_restore_scroll(area, 4, 5);
    tel26_back_next();
}

/// Answers a click on the past meetings screen. A real widget first saves the list's scroll row (param 4 from the area
/// of param 5). Id 1 (a View or Filter choice): its value goes into the filter (param 2): 1, 2, 4, 8 and 0x10 replace
/// the period bits (0x1f), 0x20, 0x40, 0x80 and 0x100 the venue bits (0x1e0), 0x200 and 0x400 the friendlies bits
/// (0x600); any value makes the cache stale (its filter 0) and the screen is rebuilt (−11). Id 2 (a meeting's date):
/// the competition's fixtures on that date (mode 1, −4). Anything else goes to the menu bar.
/// Original: `past_meetings_screen_input` (0x46ffd0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn past_meetings_screen_input(widget: u32) -> i32 {
    let filter = get_screen_param(2) as u32;
    let cache = get_screen_param(3);
    club_pages_save_scroll(widget, 4, 5);
    match club_pages_id(widget) {
        1 => {
            let value = tel26_item_value(widget as i16);
            let filter = match value {
                1 | 2 | 4 | 8 | 0x10 => Some(filter & !0x1f | value),
                0x20 | 0x40 | 0x80 | 0x100 => Some(filter & !0x1e0 | value), // tel04_1 the venue bits
                0x200 | 0x400 => Some(filter & !0x600 | value),
                _ => None,
            };
            if let Some(f) = filter {
                set_screen_param(2, f as usize);
            }
            write_u32(cache + past_meetings_cache::FILTER, 0);
            -11
        }
        2 => {
            let f = tel26_item_value(widget as i16) as usize;
            club_pages_open_comp_fixtures(read_usize(f + screen_fixture::COMP), read_u16(f + screen_fixture::DAY), read_u16(f + screen_fixture::YEAR), 1);
            -4
        }
        _ => club_pages_menu_bar(widget),
    }
}

/// Opens the arrange friendly dates screen for `club` (builder `arrange_friendly_dates_screen_build`, handler
/// `arrange_friendly_dates_screen_input`, leave `arrange_friendly_dates_screen_leave`): kind 3 a home friendly, 4 away,
/// 0xb a tour (it needs the tour record, and the screen is not a new history entry), 10 a CM Cup tournament (new
/// settings, `friendly_new_cup_settings`, in params 9 and 10). No club, or a tour without its record: the
/// club_screens.cpp 12257 box. Params: 0 the club, 1 the page 1, 2 and 3 the chosen dates and their count (none yet),
/// 4 the kind, 5 the rows per page and 6 the fixture list (0), 7 the page count 20, 8 the tour record.
/// Original: `open_arrange_friendly_screen` (0x4701f0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_arrange_friendly_screen(club: usize, kind: u32, tour: usize) {
    let kind = kind as u8;
    if club == 0 || (kind == 0xb && tour == 0) {
        club_pages_error_box(0x2fe1);
        return;
    }
    let mut settings = 0usize;
    if kind == 0xb {
        if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x470400usize, 0x470fe0usize, 0, 0x4718f0usize, 0) == 0 {
            return;
        }
    } else {
        if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x470400usize, 0x470fe0usize, 1, 0x4718f0usize, 0) == 0 {
            return;
        }
        if kind == 10 {
            covered!("tel04_1 open the CM Cup dates");
            settings = exe::crt_operator_new(0xd1);
            friendly_new_cup_settings(settings);
        }
    }
    covered!("tel04_1 open the friendly dates");
    for (k, v) in [(0u32, club), (1, 1), (2, 0), (3, 0), (4, kind as i8 as isize as usize), (5, 0), (6, 0), (7, 0x14), (8, tour), (9, settings), (10, settings)] {
        set_screen_param(k, v);
    }
}

/// Builds the arrange friendly dates screen: a calendar of the days from today, a page at a time.
///
/// The chosen dates (param 2, a vector of 10 Dates) and the fixture list (param 6) are made on the first build. The
/// club's background picture; the title (kind 3 "Arrange Friendly At Home", 4 "Away", 0xb "Arrange Tour Of <nation>",
/// else "Arrange Friendly League" or "Cup" by the tournament's kind) in the club's kit colours, "Select Date" (3, 4)
/// or "Select Dates" as the subtitle, the page buttons (page param 1 of param 7) and the menu bar; a tournament has a
/// Configure button (id 5). As many rows as fit in font 3 (param 5) from (page − 1) × rows days after today: a day
/// with a fixture of the club shows it (`add_club_fixture_row`, mode 6), the others their status
/// (`friendly_dates_status_row`). At the bottom Cancel (id 9; Back, −2, for a tour) and Next (id 10, only with a
/// chosen date). With param 10 set, a screen-wide area holds the Configure screen that opens on the first click.
/// Original: `arrange_friendly_dates_screen_build` (0x470400, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn arrange_friendly_dates_screen_build() {
    let lang = club_pages_lang();
    let mut date = [0u32; 2];
    exe::date_date_54a110((date.as_mut_ptr()) as *mut u32 as usize);
    let row_weights: [u8; 6] = [5, 10, 2, 1, 0xc, 3];
    let button_weights: [u8; 2] = [3, 1];
    let club = get_screen_param(0);
    let page = get_screen_param(1) as u8;
    let mut chosen = get_screen_param(2);
    let count = get_screen_param(3) as u8;
    let kind = get_screen_param(4) as u8;
    let mut list = get_screen_param(6);
    let pages = get_screen_param(7) as u8;
    let tour = Nation(get_screen_param(8));
    let settings = get_screen_param(9);
    if chosen == 0 {
        covered!("tel04_1 friendly dates vector");
        let room = exe::crt_operator_new(0x54);
        chosen = if room != 0 {
            write_u32(room, 10);
            exe::crt_vector_constructor_iterator(room + 4, 8, 10, 0x54a110usize, 0x5a53c0usize);
            room + 4
        } else {
            0
        };
        set_screen_param(2, chosen);
    }
    if list == 0 {
        let room = exe::crt_operator_new(0xe);
        list = if room != 0 {
            exe::new_llist(room)
        } else {
            0
        };
        set_screen_param(6, list);
    }
    covered!("tel04_1 friendly dates build");
    exe::background_set_club_picture(club, 0, 0);
    match kind {
        3 => {
            lang(JOB_TEXT, 0x990d88); // "Arrange Friendly At Home"
        }
        4 => {
            lang(JOB_TEXT, 0x990d70); // "Arrange Friendly Away"
        }
        0xb => {
            let c = (tour.short_gender_name() as i8) as i32;
            lang(JOB_TEXT, 0x990d4c, c, c, tour.addr() + nation::SHORT_NAME); // "Arrange Tour Of {}<%s - nation>{}"
        }
        _ => {
            lang(JOB_TEXT, if read_u8(settings + 0xc8) == 2 { 0x990d34usize } else { 0x990d1c }); // League / Cup
        }
    }
    club_pages_title(club, JOB_TEXT);
    lang(JOB_TEXT, if kind == 3 || kind == 4 { 0x990d00usize } else { 0x990d0c }); // "Select Date" / "Select Dates"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    exe::gui_add_page_buttons(0x311, 0x55, page as i8 as i16 as u16 as u32, pages as i8 as i16 as u16 as u32, 1, 2, 3, 4);
    exe::menubar_build(4, 0);
    let (grey, pale) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY));
    let mut top = 0x91;
    if kind == 10 {
        lang(JOB_TEXT, 0x990cf4); // "Configure"
        tel26_add_button([0x28f, 0x91, 0x30c, 0xa5, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 5, 0]);
        top = 0xaa;
    }
    let rows = ((0x207 - top) / exe::font_get_height(3)) as u8;
    set_screen_param(5, rows as usize);
    let area = club_pages_area([0x6e, top as u32, 0x30c, 0x217, 6, row_weights.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    exe::llist_clear(list);
    let first = (page as i8 as i16).wrapping_sub(1).wrapping_mul(rows as i16);
    for row in 0..rows as u32 {
        let mut later = [0u32; 2];
        let d = club_pages_add_days(G_CURRENT_DATE, later.as_mut_ptr(), first.wrapping_add(row as i16));
        club_pages_copy_date(date.as_mut_ptr() as usize, d);
        let before = read_i16(list + llist::COUNT);
        let (lo, hi) = (date[0], read_u32(date.as_ptr() as usize + 2));
        exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), lo, hi, lo, hi, read_u32(club), list, u32::MAX, 0xffff_fffe, u32::MAX);
        if before < read_i16(list + llist::COUNT) {
            write_u32(list + llist::CURSOR, read_u32(list + llist::TAIL));
            let fixture = exe::llist_get_current(list);
            exe::add_club_fixture_row(club, fixture, area, row, 6);
        } else {
            friendly_dates_status_row(area, row, date.as_mut_ptr(), club, chosen, count, kind, settings);
        }
    }
    let buttons = club_pages_area([100, 0x22b, 0x316, 0x24e, 2, button_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    if kind == 0xb {
        lang(JOB_TEXT, 0x9880a8); // "Back"
        tel26_area_button([buttons, 0, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 0xffff_fffe, 0]);
    } else {
        lang(JOB_TEXT, 0x98a1a0); // "Cancel"
        tel26_area_button([buttons, 0, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 9, 0]);
    }
    lang(JOB_TEXT, 0x9880a0); // "Next"
    if count == 0 {
        tel26_area_text([buttons, 1, 0, 0x30, grey, 0x2c, 3, pale, JOB_TEXT as u32]);
    } else {
        tel26_area_button([buttons, 1, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 10, 0]);
    }
    if get_screen_param(10) != 0 {
        let (w, h) = (exe_immediate(0x470fb3 as *const i32), exe_immediate(0x470fae as *const i32));
        club_pages_area([0, 0, w as u32, h as u32, 1, 0, 1, 0, 2, 0, u32::MAX]);
    }
}

/// Answers a click on the arrange friendly dates screen.
///
/// With param 10 set (the first click of a new CM Cup) it is cleared and the Configure screen opens over the
/// settings (param 9, as a new tournament; −4). Ids 1 and 2 go back 10 pages and 1 (not below 1), 3 and 4 forward 1
/// and 10 (not past the page count, param 7) (−11). Id 5 opens Configure (−4); id 6 the competition's fixtures on the
/// day (mode 0, −4). Id 7 drops a date (the value is its row: today + (page − 1) × rows + row): a tour takes it out of
/// the chosen dates (the last one moves into its place), keeps them sorted and lowers the count; the others clear the
/// count. Id 8 chooses a date: home and away friendlies have one date, a tour adds it (up to 10, sorted), a tournament
/// takes as many days from it as its kind needs (`friendly_get_rest_days_for_kind`). Id 9 cancels (a friendly's open
/// invitation closed, else the tournament settings freed; −9). Id 10 goes on: a home or away friendly is added
/// (`Friendly::add_friendly_match`) and the opponent screen opens (by nation for a club, by continent for a national
/// team; −4); a tour asks the nation for dates (`add_friendly_dates_request`, −10); a tournament is added
/// (`Friendly::add_tournament`, the settings freed and command 0x11 queued with it; −10); a failure shows the
/// club_screens.cpp 12841, 12863 or 12882 box (0). Anything else goes to the menu bar.
/// Original: `arrange_friendly_dates_screen_input` (0x470fe0, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn arrange_friendly_dates_screen_input(widget: u32) -> i32 {
    let mut date = [0u32; 2];
    exe::date_date_54a110((date.as_mut_ptr()) as *mut u32 as usize);
    let d = date.as_mut_ptr() as usize;
    let club = get_screen_param(0);
    let page = get_screen_param(1) as u8 as i8;
    let chosen = get_screen_param(2);
    let count = get_screen_param(3) as u8 as i8;
    let kind = get_screen_param(4) as u8;
    let rows = get_screen_param(5) as u8;
    let pages = get_screen_param(7) as u8 as i8;
    let tour = get_screen_param(8);
    let settings = get_screen_param(9);
    if get_screen_param(10) != 0 {
        set_screen_param(10, 0);
        open_configure_tournament_screen_by_address(settings, 1);
        return -4;
    }
    let item_date = |date: usize| {
        let row = tel26_item_value(widget as i16);
        let days = (page as i16).wrapping_sub(1).wrapping_mul(rows as i16).wrapping_add(row as i16);
        let mut later = [0u32; 2];
        let r = club_pages_add_days(G_CURRENT_DATE, later.as_mut_ptr(), days);
        club_pages_copy_date(date, r);
    };
    let qsort = |n: i32| exe::crt_qsort(chosen, n as u32, 8, 0x4877c0usize);
    match club_pages_id(widget) {
        1 => {
            set_screen_param(1, (page as i32 - 10).max(1) as usize);
            -11
        }
        2 => {
            set_screen_param(1, (page as i32 - 1).max(1) as usize);
            -11
        }
        3 => {
            set_screen_param(1, (page as i32 + 1).min(pages as i32) as usize);
            -11
        }
        4 => {
            set_screen_param(1, (page as i32 + 10).min(pages as i32) as usize);
            -11
        }
        5 => {
            open_configure_tournament_screen_by_address(settings, 0);
            -4
        }
        6 => {
            let f = tel26_item_value(widget as i16) as usize;
            club_pages_open_comp_fixtures(read_usize(f + screen_fixture::COMP), read_u16(f + screen_fixture::DAY), read_u16(f + screen_fixture::YEAR), 0);
            -4
        }
        7 => {
            item_date(d);
            if kind != 0xb {
                set_screen_param(3, 0);
                return -11;
            }
            covered!("tel04_1 drop a tour date");
            for k in 0..count.max(0) as i32 {
                let slot = friendly_dates_slot(chosen, k);
                if read_u16(d + 2) == read_u16(slot + 2) && read_u16(d) == read_u16(slot) {
                    club_pages_copy_date(slot, friendly_dates_slot(chosen, count as i32 - 1));
                }
            }
            let left = count.wrapping_sub(1);
            set_screen_param(3, left as isize as usize);
            if left != 0 {
                qsort(left as i32);
            }
            -11
        }
        8 => {
            item_date(d);
            if kind == 3 || kind == 4 {
                club_pages_copy_date(chosen, d);
                set_screen_param(3, 1);
                return -11;
            }
            if kind == 0xb {
                if count >= 10 {
                    return -11;
                }
                club_pages_copy_date(friendly_dates_slot(chosen, count as i32), d);
                let n = count + 1;
                set_screen_param(3, n as isize as usize);
                qsort(n as i32);
                return -11;
            }
            let days: unsafe extern "thiscall" fn(usize, u32) -> i8 = core::mem::transmute(FN_FRIENDLY_GET_REST_DAYS_FOR_KIND);
            let mut k: i8 = 0;
            while k < days(read_usize(G_FRIENDLY), read_u8(settings + 0xc8) as u32) {
                let mut later = [0u32; 2];
                let r = club_pages_add_days(d, later.as_mut_ptr(), k as i16);
                club_pages_copy_date(friendly_dates_slot(chosen, k as i32), r);
                k += 1;
            }
            set_screen_param(3, k as isize as usize);
            -11
        }
        9 => {
            if kind == 3 || kind == 4 {
                let invitation = exe::friendly_find_human_club_open_invitation(read_usize(G_FRIENDLY));
                if invitation != 0 {
                    write_u32(invitation + 8, 0);
                }
            } else {
                exe::crt_operator_delete(settings);
            }
            -9
        }
        10 => friendly_dates_next(club, chosen, count, kind, tour, settings),
        _ => club_pages_menu_bar(widget),
    }
}

/// Leaves the arrange friendly dates screen: the chosen dates vector (param 2, the count before it) is destroyed and
/// freed and params 2 and 3 cleared; the fixture list (param 6) is destroyed and freed and param 6 cleared.
/// Original: `arrange_friendly_dates_screen_leave` (0x4718f0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn arrange_friendly_dates_screen_leave() {
    if get_screen_param(2) != 0 {
        let chosen = get_screen_param(2);
        if chosen != 0 {
            exe::crt_vector_destructor_iterator(chosen, 8, read_u32(chosen - 4), 0x5a53c0usize);
            exe::crt_operator_delete(chosen - 4);
        }
        set_screen_param(2, 0);
        set_screen_param(3, 0);
    }
    if get_screen_param(6) != 0 {
        let list = get_screen_param(6);
        if list != 0 {
            exe::drop_llist(list);
            exe::crt_operator_delete(list);
        }
        set_screen_param(6, 0);
    }
}

/// Comparer for `open_past_meetings_screen`: opening does not run twice; only the original runs and the new screen's
/// params are checked against the plan.
pub(crate) unsafe extern "C" fn compare_open_past_meetings_screen(club: usize, opposition: usize, reserves: u32) {
    original_fn::<unsafe extern "C" fn(usize, usize, u32)>(IDX_OPEN_PAST_MEETINGS_SCREEN)(club, opposition, reserves);
    if club != 0 && opposition != 0 && get_screen_param(0) == club && get_screen_param(1) == opposition {
        let plan = vec![club, opposition, 0x221, past_meetings_cache_of_slot(), 0, 0, reserves as usize];
        let got = club_pages_params(7);
        SCOREBOARD.record(got == plan, || format!("past meetings screen (open): params {:x?} x plan {:x?}", got, plan));
    }
}

/// Comparer for `past_meetings_screen_build`: side by side by the display list plus the cache and its table; when the
/// cache is stale (the search makes a new list and table) only the original runs.
pub(crate) unsafe extern "C" fn compare_past_meetings_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_PAST_MEETINGS_SCREEN_BUILD);
    let (club, opposition, cache) = (get_screen_param(0), get_screen_param(1), get_screen_param(3));
    let today = read_u32(G_CURRENT_DATE);
    if club == 0 || opposition == 0 || cache == 0 || read_u16(cache + 2) != (today >> 16) as u16 || read_u16(cache) != today as u16
        || read_usize(cache + past_meetings_cache::CLUB) != club || read_usize(cache + past_meetings_cache::OPPOSITION) != opposition
        || read_usize(cache + past_meetings_cache::RESERVES) != get_screen_param(6) || read_u32(cache + past_meetings_cache::FILTER) != get_screen_param(2) as u32 {
        // simplification: the search allocates the list and the table; proven in Rust mode
        return o();
    }
    let mut extra = vec![(cache, past_meetings_cache::SIZE)];
    let table = read_usize(cache + past_meetings_cache::TABLE);
    if table != 0 {
        extra.push((table, 0xee));
    }
    compare_match_screen_build("past meetings screen", &extra, || o(), || past_meetings_screen_build());
}

/// Comparer for `past_meetings_screen_input`: the View and Filter choices (id 1) side by side over the params and the
/// cache's filter; the rest only the original.
pub(crate) unsafe extern "C" fn compare_past_meetings_screen_input(widget: u32) -> i32 {
    let cache = get_screen_param(3);
    let both = cache != 0 && club_pages_id(widget) == 1;
    club_pages_compare_input("past meetings screen input", widget, original_fn(IDX_PAST_MEETINGS_SCREEN_INPUT), past_meetings_screen_input, both, &[(cache + past_meetings_cache::FILTER, 4)])
}

/// Comparer for `open_arrange_friendly_screen`: only the original runs (it opens a screen and may allocate the CM Cup
/// settings); the params and the new settings are checked against the plan.
pub(crate) unsafe extern "C" fn compare_open_arrange_friendly_screen(club: usize, kind: u32, tour: usize) {
    original_fn::<unsafe extern "C" fn(usize, u32, usize)>(IDX_OPEN_ARRANGE_FRIENDLY_SCREEN)(club, kind, tour);
    let k = kind as u8;
    if club == 0 || (k == 0xb && tour == 0) || get_screen_param(0) != club || get_screen_param(8) != tour {
        return;
    }
    let settings = get_screen_param(9);
    let mut plan = vec![club, 1, 0, 0, k as i8 as isize as usize, 0, 0, 0x14, tour, settings, settings];
    let mut ok = true;
    if k == 10 {
        let mut expected = [0u8; 0xd1];
        friendly_new_cup_settings(expected.as_mut_ptr() as usize);
        ok = settings != 0 && core::slice::from_raw_parts(settings as *const u8, 0xd1)[..7] == expected[..7]
            && core::slice::from_raw_parts((settings + 0xc8) as *const u8, 9) == &expected[0xc8..];
    } else {
        plan[9] = 0;
        plan[10] = 0;
    }
    let got = club_pages_params(11);
    SCOREBOARD.record(ok && got == plan, || format!("arrange friendly dates screen (open kind {}): params {:x?} x plan {:x?} settings ok {}", k, got, plan, ok));
}

/// Comparer for `arrange_friendly_dates_screen_build`: side by side by the display list plus the chosen dates, when
/// the vector and the list exist and no day of the page has a fixture of the club (the fixture rows carry the
/// pointers of the list's fresh copies); otherwise only the original.
pub(crate) unsafe extern "C" fn compare_arrange_friendly_dates_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_ARRANGE_FRIENDLY_DATES_SCREEN_BUILD);
    let (chosen, list) = (get_screen_param(2), get_screen_param(6));
    if chosen == 0 || list == 0 {
        // simplification: the first build allocates the vector and the list; proven in Rust mode
        return o();
    }
    club_pages_compare_build_when("arrange friendly dates screen", &[(chosen, 0x50)], || o(), || arrange_friendly_dates_screen_build(),
        || read_i16(list + llist::COUNT) == 0);
}

/// Comparer for `arrange_friendly_dates_screen_input`: the page buttons and choosing or dropping a date (ids 1..4, 7,
/// 8) side by side over the params and the chosen dates; the rest (other screens, the friendlies' records, freeing)
/// only the original.
pub(crate) unsafe extern "C" fn compare_arrange_friendly_dates_screen_input(widget: u32) -> i32 {
    let chosen = get_screen_param(2);
    let id = club_pages_id(widget);
    let both = chosen != 0 && get_screen_param(10) == 0 && matches!(id, 1..=4 | 7 | 8);
    club_pages_compare_input("arrange friendly dates screen input", widget, original_fn(IDX_ARRANGE_FRIENDLY_DATES_SCREEN_INPUT), arrange_friendly_dates_screen_input, both, &[(chosen, 0x50)])
}

/// Comparer for `arrange_friendly_dates_screen_leave`: freeing does not run twice; only the original runs and the
/// cleared params are checked afterwards.
pub(crate) unsafe extern "C" fn compare_arrange_friendly_dates_screen_leave() {
    original_fn::<unsafe extern "C" fn()>(IDX_ARRANGE_FRIENDLY_DATES_SCREEN_LEAVE)();
    let left = [get_screen_param(2), get_screen_param(3), get_screen_param(6)];
    SCOREBOARD.record(left == [0, 0, 0], || format!("arrange friendly dates screen leave: params 2, 3, 6 left {:x?}", left));
}

/// Opens the arrange friendly tour screen of `club`, the nation choice (builder `friendly_tour_nation_screen_build`,
/// handler `friendly_tour_nation_screen_handle_event`); no club shows the club_screens.cpp 12948 box. Params: 0 the
/// club, 1 the chosen nation (none), 2 and 3 the scroll row and the area (0), 4 "available only" on.
/// Original: `open_friendly_tour_nation_screen` (0x4719a0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_friendly_tour_nation_screen(club: usize) {
    if club == 0 {
        club_pages_error_box(0x3294);
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x471a90usize, 0x472180usize, 1, 0usize, 0) != 0 {
        covered!("tel04_2 open the tour nations");
        for (k, v) in [(0u32, club), (1, 0), (2, 0), (3, 0), (4, 1)] {
            set_screen_param(k, v);
        }
    }
}

/// Builds the arrange friendly tour screen, the nation choice of the club (param 0).
///
/// The club's background picture, "Arrange Friendly Tour" in its kit colours, "Select Nation" and the menu bar; the
/// Filter popup (Available Only, ticked with param 4, and All Nations). The nations that have a continent, a game
/// importance up to 2 and clubs, other than the club's own, sorted by short name (`database_compare_clubs_by_short_name`
/// over a malloc'd list; a failed malloc shows the club_screens.cpp 13048 box and ends the build), two per row: the
/// name (a button, id 2, highlighted 0x801 when it is the chosen one, param 1, or plain grey text when the tour rules
/// close it, `friendly_tour_nation_open`; with Available Only the closed ones are left out) and its continent. The
/// scroll row of param 2, the area to param 3; Cancel (id 3) and Next (id 4, only with a chosen nation).
/// Original: `friendly_tour_nation_screen_build` (0x471a90, club_screens.cpp, cdecl; the map's 0x472070 fragment is its
/// tail).
pub(crate) unsafe extern "C" fn friendly_tour_nation_screen_build() {
    let lang = club_pages_lang();
    let format = club_pages_format();
    let row_weights: [u8; 4] = [6, 1, 6, 1];
    let button_weights: [u8; 2] = [3, 1];
    let club = Club(get_screen_param(0));
    let chosen = get_screen_param(1);
    let available_only = get_screen_param(4);
    covered!("tel04_2 tour nations build");
    exe::background_set_club_picture(club, 0, 0);
    lang(JOB_TEXT, 0x990dc4); // "Arrange Friendly Tour"
    club_pages_title(club.addr(), JOB_TEXT);
    lang(JOB_TEXT, 0x990db4); // "Select Nation"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    exe::menubar_build(4, 0);
    let (grey, pale, yellow) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY), job_colour(G_GUI_COLOUR_YELLOW));
    let (blue, black) = (job_colour(0xae31aa), job_colour(G_GUI_COLOUR_BLACK));
    lang(JOB_TEXT, 0x98ebf8); // "Filter"
    let button = tel26_add_button([0x28f, 0x91, 0x30c, 0xa5, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0, 0]);
    let filter = club_pages_area([0, 0, 0, 0, 0, 0, 0, 0, 0x130, blue, button]);
    lang(JOB_TEXT, 0x98eb6c); // " Available Only"
    let on = available_only != 0;
    club_pages_item([if on { 0x2002 } else { 2 }, 0, 0, 0, 0, 0, 0, 0x10, blue, yellow, 1, 1, black, JOB_TEXT as u32, 0, !on as u32, !on as u32, filter]);
    lang(JOB_TEXT, 0x990da4); // " All Nations"
    let on = available_only == 0;
    club_pages_item([if on { 0x2002 } else { 2 }, 0, 0, 0, 0, 0, 0, 0x10, blue, yellow, 1, 1, black, JOB_TEXT as u32, 0, !on as u32, 0, filter]);
    let rows = (0x15d / exe::font_get_height(3)) as u8 as u32;
    let area = club_pages_area([0x6e, 0xaa, 0x30c, 0x217, 4, row_weights.as_ptr() as u32, rows, 0, 2, 0, u32::MAX]);
    let (mut col, mut row) = (0u32, 0u32);
    let list = exe::crt_malloc((read_i32(G_NATION_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        club_pages_error_box(0x32f8);
        return;
    }
    let mut count = 0usize;
    for k in 0..read_i32(G_NATION_COUNT).max(0) as usize {
        let n = read_usize(G_NATION_TABLE) + k * nation::SIZE;
        if read_usize(n + nation::CONTINENT) != 0 && read_i8(n + nation::GAME_IMPORTANCE) <= 2 && n != club.nation().addr() && read_i16(n + nation::NUMBER_CLUBS) != 0 {
            write_usize(list + count * 4, n);
            count += 1;
        }
    }
    exe::crt_qsort(list, count as u32, 4, 0x540080usize);
    for k in 0..count {
        let n = read_usize(list + k * 4);
        let open = friendly_tour_nation_open(club.addr(), n);
        if available_only != 0 && !open {
            continue;
        }
        format(JOB_TEXT, 0x987ff0, n + 4); // "  %s"
        if open {
            let style = if n == chosen { 0x801 } else { 1 };
            tel26_area_button([area, col, row, style, 0, 1, 3, pale, JOB_TEXT as u32, 2, n as u32]);
        } else {
            tel26_area_text([area, col, row, 1, 0, 1, 3, grey, JOB_TEXT as u32]);
        }
        tel26_area_text([area, col + 1, row, 1, 0, 0xc, 2, yellow, (read_usize(n + nation::CONTINENT) + 0x1f) as u32]);
        col += 2;
        if col > 3 {
            col = 0;
            row += 1;
        }
    }
    exe::crt_free(list);
    club_pages_restore_scroll(area, 2, 3);
    let buttons = club_pages_area([100, 0x22b, 0x316, 0x24e, 2, button_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    lang(JOB_TEXT, 0x98a1a0); // "Cancel"
    tel26_area_button([buttons, 0, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 3, 0]);
    lang(JOB_TEXT, 0x9880a0); // "Next"
    if chosen == 0 {
        tel26_area_text([buttons, 1, 0, 0x30, grey, 0x2c, 3, pale, JOB_TEXT as u32]);
    } else {
        tel26_area_button([buttons, 1, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 4, 0]);
    }
}

/// Answers a click on the tour nation choice. A real widget saves the scroll row (param 2 from the area of param 3).
/// Id 1 toggles Available Only (param 4, −11); id 2 chooses the nation (param 1, −11); id 3 cancels (−9); id 4 opens
/// the tour dates (`open_arrange_friendly_screen`, kind 0xb, with the nation; −4). Anything else goes to the menu bar.
/// Original: `friendly_tour_nation_screen_handle_event` (0x472180, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn friendly_tour_nation_screen_handle_event(widget: u32) -> i32 {
    club_pages_save_scroll(widget, 2, 3);
    let club = get_screen_param(0);
    let chosen = get_screen_param(1);
    let available_only = get_screen_param(4);
    match club_pages_id(widget) {
        1 => {
            set_screen_param(4, (available_only == 0) as usize);
            -11
        }
        2 => {
            set_screen_param(1, tel26_item_value(widget as i16) as usize);
            -11
        }
        3 => -9,
        4 => {
            exe::open_arrange_friendly_screen(club, (0xb) as u32 as usize, chosen);
            -4
        }
        _ => club_pages_menu_bar(widget),
    }
}

/// Builds the tour club choice of a tour (param 0: the club, then the nation).
///
/// The club's background picture, "Arrange Tour Of <nation>" in its kit colours, "Select Club" and the menu bar; Cancel
/// (id 1) and Next (−3, plain text when there is no next screen). The nation's clubs (not national teams), sorted by
/// short name over a malloc'd list (a failed malloc: the club_screens.cpp 13318 box), two per row of three columns: the
/// name (yellow for a human's), the national team (linked) and the division (linked when it has a competition object).
/// The scroll row of param 1, the area to param 2.
/// Original: `friendly_tour_club_screen_build` (0x472380, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn friendly_tour_club_screen_build() {
    let lang = club_pages_lang();
    let format = club_pages_format();
    let row_weights: [u8; 6] = [5, 1, 1, 5, 1, 1];
    let button_weights: [u8; 2] = [3, 1];
    let tour = get_screen_param(0);
    let club = read_usize(tour);
    let tour_nation = Nation(read_usize(tour + 4));
    covered!("tel04_2 tour clubs build");
    exe::background_set_club_picture(club, 0, 0);
    let c = (tour_nation.short_gender_name() as i8) as i32;
    lang(JOB_TEXT, 0x990d4c, c, c, tour_nation.addr() + nation::SHORT_NAME); // "Arrange Tour Of {}<%s - nation>{}"
    club_pages_title(club, JOB_TEXT);
    lang(JOB_TEXT, 0x990ddc); // "Select Club"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    exe::menubar_build(4, 0);
    let (grey, pale, yellow) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY), job_colour(G_GUI_COLOUR_YELLOW));
    let buttons = club_pages_area([100, 0x22b, 0x316, 0x24e, 2, button_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    lang(JOB_TEXT, 0x98a1a0); // "Cancel"
    tel26_area_button([buttons, 0, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 1, 0]);
    lang(JOB_TEXT, 0x9880a0); // "Next"
    if exe::screenmanager_is_forward_unavailable(G_SCREEN_MANAGER) == 0 {
        tel26_area_button([buttons, 1, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 0xffff_fffd, 0]);
    } else {
        tel26_area_text([buttons, 1, 0, 0x30, grey, 0x2c, 3, pale, JOB_TEXT as u32]);
    }
    let rows = (0x176 / exe::font_get_height(3)) as u8 as u32;
    let area = club_pages_area([0x6e, 0x91, 0x30c, 0x217, 6, row_weights.as_ptr() as u32, rows, 0, 2, 0, u32::MAX]);
    let (mut col, mut row) = (0u32, 0u32);
    let list = exe::crt_malloc((read_i32(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        club_pages_error_box(0x3406);
        return;
    }
    let mut count = 0usize;
    for k in 0..read_i32(G_CLUB_COUNT).max(0) as usize {
        let c = read_usize(G_CLUB_TABLE) + k * club::SIZE;
        if exe::database_is_national_team(c) == 0 && read_usize(c + club::NATION) == tour_nation.addr() {
            write_usize(list + count * 4, c);
            count += 1;
        }
    }
    exe::crt_qsort(list, count as u32, 4, 0x540080usize);
    for k in 0..count {
        let c = read_usize(list + k * 4);
        format(JOB_TEXT, 0x987ff0, c + club::SHORT_NAME); // "  %s"
        let colour = if exe::human_manager_is_human_club(c, 0, 1, 0, 0) != 0 { yellow } else { pale };
        tel26_area_button([area, col, row, 1, 0, 1, 3, colour, JOB_TEXT as u32, 0x7d5, c as u32]);
        let n = read_usize(c + club::NATION);
        let team = exe::database_get_national_team(n);
        tel26_area_button([area, col + 1, row, 1, 0, 0xc, 2, yellow, (n + 0x53) as u32, 0x7d5, team as u32]);
        let division = read_usize(c + club::DIVISION);
        if division != 0 {
            if read_u32(read_usize(G_COMP_OBJECT_TABLE) + read_u32(division) as usize * 4) == 0 {
                tel26_area_text([area, col + 2, row, 1, 0, 0xc, 2, job_colour(0xaebdd4), (division + 0x53) as u32]);
            } else {
                tel26_area_button([area, col + 2, row, 1, 0, 0xc, 2, job_colour(0xaebdd4), (division + 0x53) as u32, 0x7d0, division as u32]);
            }
        }
        col += 3;
        if col > 3 {
            col = 0;
            row += 1;
        }
    }
    exe::crt_free(list);
    club_pages_restore_scroll(area, 1, 2);
}

/// Answers a click on the tour club choice: a real widget saves the scroll row (param 1 from the area of param 2); id 1
/// (Cancel) clears the tour record's +0x103 and leaves (−9); anything else goes to the menu bar.
/// Original: `friendly_tour_club_screen_handle_event` (0x472850, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn friendly_tour_club_screen_handle_event(widget: u32) -> i32 {
    let tour = get_screen_param(0);
    club_pages_save_scroll(widget, 1, 2);
    if club_pages_id(widget) == 1 {
        write_u32(tour + 0x103, 0);
        return -9;
    }
    club_pages_menu_bar(widget)
}

/// Opens the Configure Tournament screen over the tournament settings `settings` (builder
/// `configure_tournament_screen_build`, handler `configure_tournament_screen_handle_event`); no settings shows the
/// club_screens.cpp 13437 box. Params: 0 the settings, 1 `arg` (set: the screen was opened by a new tournament and its
/// settings are freed on Cancel), 2 a new working copy of the 0xd1 bytes.
/// Original: `open_configure_tournament_screen` (0x472900, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_configure_tournament_screen(settings: usize, arg: u32) {
    if settings == 0 {
        club_pages_error_box(0x347d);
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x472a00usize, 0x4735f0usize, 1, 0usize, 0) != 0 {
        covered!("tel04_2 open the tournament settings");
        let copy = exe::crt_operator_new(0xd1);
        core::ptr::copy_nonoverlapping(settings as *const u8, copy as *mut u8, 0xd1);
        set_screen_param(0, settings);
        set_screen_param(1, arg as usize);
        set_screen_param(2, copy);
    }
}

/// Builds the Configure Tournament screen over the working copy of the settings (param 2).
///
/// The green of the choices ([0xad9db6]) is made once (`graphics_rgb_to_pixel(0, 0xc0, 0, 0)`). A cup (kind 1) gets a
/// decider (+0xcd 0 → 1) and 4 teams (+0xcf 3 → 4). No background picture; without param 1 a screen-wide area blocks
/// the screen below; a dark green box, the title "Configure Tournament", Cancel (id 1) and Ok (id 2), the pitch drawing
/// and the table: the name (an edit field over the copy's name, 15 characters), the type (League 2 / Cup 1, id 3),
/// points for a win (3 / 2, id 4), the tie-break (goal difference 1, goals for 2, games won 3, id 5), extra time (none
/// 0, 30 minutes 2, golden goal 0x80, id 6), the decider (none 0 / penalties 1, id 7) and the teams (4 / 3, id 8); a
/// cup has no points, tie-break, "none" decider nor 3 teams.
/// Original: `configure_tournament_screen_build` (0x472a00, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn configure_tournament_screen_build() {
    let lang = club_pages_lang();
    let button_weights: [u8; 2] = [3, 1];
    let name_weights: [u8; 4] = [3, 6, 0, 0];
    let table_weights: [u8; 4] = [3, 2, 2, 2];
    if read_u8(0xadab40) & 1 == 0 {
        write_u8(0xadab40, read_u8(0xadab40) | 1);
        write_u16(0xad9db6, exe::graphics_rgb_to_pixel(0, 0xc0, 0, 0) as u16);
    }
    let arg = get_screen_param(1);
    let s = get_screen_param(2);
    covered!("tel04_2 tournament settings build");
    let kind = read_u8(s + 0xc8);
    if kind == 1 {
        if read_u16(s + 0xcd) == 0 {
            write_u16(s + 0xcd, 1);
        }
        if read_u16(s + 0xcf) == 3 {
            write_u16(s + 0xcf, 4); // tel04_2 a cup's teams
        }
    }
    exe::background_set_picture(0usize);
    if arg == 0 {
        let (w, h) = (exe_immediate(0x472adc as *const i32), exe_immediate(0x472ad7 as *const i32));
        club_pages_area([0, 0, w as u32, h as u32, 1, 0, 1, 0, 2, 0, u32::MAX]);
    }
    let dark_green = exe::graphics_rgb_to_pixel(0, 0x38, 0, 0);
    club_pages_area([0xaa, 0x89, 0x2cf, 0x1cd, 1, 0, 1, 0, 0x30, dark_green, u32::MAX]);
    let (grey, pale, label) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY), job_colour(SCREEN_COLOUR_3154));
    lang(JOB_TEXT, 0x990f80); // "Configure Tournament"
    exe::screenmanager_add_text_item(G_SCREEN_MANAGER, 0xb4, 0x93, 0x2c5, 0xc0, 0x30, job_colour(G_GUI_COLOUR_BLACK), 0xc, 4, pale, JOB_TEXT as u32, 1);
    let buttons = club_pages_area([0xb4, 0x1a5, 0x2c5, 0x1c3, 2, button_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    lang(JOB_TEXT, 0x98a1a0); // "Cancel"
    tel26_area_button([buttons, 0, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 1, 0]);
    lang(JOB_TEXT, 0x990f7c); // "Ok"
    tel26_area_button([buttons, 1, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 2, 0]);
    let mut pitch = [0i32; 4];
    let p = pitch.as_mut_ptr();
    let draw_pitch: unsafe extern "C" fn(u32, u32, u32, u32, *mut i32, *mut i32, *mut i32, *mut i32, u32, u32, u32) -> u32 = core::mem::transmute(FN_DRAW_PITCH);
    draw_pitch(0xbe, 0xd4, 0x2bb, 0x191, p.add(3), p.add(2), p.add(1), p, 1, buttons, 0x4080_0000);
    let names = club_pages_area([0xc3, 0xd9, 0x2b6, 0x18c, 4, name_weights.as_ptr() as u32, 7, 0, 1, 0, u32::MAX]);
    lang(JOB_TEXT, 0x98ee04); // " Name"
    tel26_area_text([names, 0, 0, 1, 0, 1, 3, label, JOB_TEXT as u32]);
    club_pages_item([0x80018, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 2, job_colour(0xad9db6), s as u32, 0xf, 0, 0, names]);
    let t = club_pages_area([0xc3, 0xd9, 0x2b6, 0x18c, 4, table_weights.as_ptr() as u32, 7, 0, 1, 0, u32::MAX]);
    let (c9, ca, cb, cd, cf) = (read_u8(s + 0xc9), read_u8(s + 0xca), read_u16(s + 0xcb), read_u16(s + 0xcd), read_u16(s + 0xcf));
    let cup = kind == 1;
    let title = |row: u32, text: usize, arg: usize| {
        if arg != 0 {
            lang(JOB_TEXT, text, arg);
        } else {
            lang(JOB_TEXT, text);
        }
        tel26_area_text([t, 0, row, 1, 0, 1, 3, label, JOB_TEXT as u32]);
    };
    let choice = |col: u32, row: u32, text: usize, arg: usize, disabled: bool, current: bool, id: u32, value: u32| {
        if arg != 0 {
            lang(JOB_TEXT, text, arg);
        } else {
            lang(JOB_TEXT, text);
        }
        tournament_setting(t, col, row, disabled, current, id, value);
    };
    title(1, 0x990f54, 0xad9c64); // " Type"
    choice(1, 1, 0x990f4c, 0, false, kind == 2, 3, 2); // "League"
    choice(2, 1, 0x990f48, 0, false, kind == 1, 3, 1); // "Cup"
    title(2, 0x990f38, 0); // " Points For Win"
    choice(1, 2, 0x990f10, 0xad9c64, cup, c9 == 3, 4, 3); // "Three"
    choice(2, 2, 0x990ee8, 0xad9c64, cup, c9 == 2, 4, 2); // "Two"
    title(3, 0x990edc, 0); // " Separator"
    choice(1, 3, 0x990ed0, 0, cup, ca == 1, 5, 1); // "Goal Diff."
    choice(2, 3, 0x990ec4, 0, cup, ca == 2, 5, 2); // "Goals For"
    choice(3, 3, 0x990eb8, 0, cup, ca == 3, 5, 3); // "Games Won"
    title(4, 0x990eac, 0); // " Extra Time"
    choice(1, 4, 0x990e88, 0xad9c64, false, cb == 0, 6, 0); // "None"
    choice(2, 4, 0x990e80, 0, false, cb == 2, 6, 2); // "30 Mins"
    choice(3, 4, 0x990e74, 0, false, cb == 0x80, 6, 0x80); // "Golden Goal"
    title(5, 0x990e68, 0); // " Decider"
    choice(1, 5, 0x990e48, 0xad9c64, cup, cd == 0, 7, 0); // "None"
    choice(2, 5, 0x990e40, 0, false, cd == 1, 7, 1); // "Penalties"
    title(6, 0x990e38, 0); // " Teams"
    choice(1, 6, 0x990e10, 0xad9c64, false, cf == 4, 8, 4); // "Four"
    choice(2, 6, 0x990de8, 0xad9c64, cup, cf == 3, 8, 3); // "Three"
}

/// Answers a click on Configure Tournament (params 0 the settings, 1 the new-tournament flag, 2 the working copy).
/// Id 1 (Cancel) frees the copy and, for a new tournament, the settings too (−10; else −9). Id 2 (Ok) names an unnamed
/// copy "CM Cup", copies it over the settings and frees it (−9). Ids 3, 4 and 5 store the value as the kind, the points
/// (+0xc9) and the tie-break (+0xca), 6, 7 and 8 as the extra time (+0xcb), the decider (+0xcd) and the teams (+0xcf)
/// (−11). Anything else: 0.
/// Original: `configure_tournament_screen_handle_event` (0x4735f0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn configure_tournament_screen_handle_event(widget: u32) -> i32 {
    let settings = get_screen_param(0);
    let arg = get_screen_param(1);
    let copy = get_screen_param(2);
    let value = tel26_item_value(widget as i16);
    match club_pages_id(widget) {
        1 => {
            exe::crt_operator_delete(copy);
            if arg != 0 {
                exe::crt_operator_delete(settings);
                return -10;
            }
            -9
        }
        2 => {
            if read_u8(copy) == 0 {
                club_pages_lang()(copy, 0x990cb4); // "CM Cup"
            }
            core::ptr::copy_nonoverlapping(copy as *const u8, settings as *mut u8, 0xd1);
            exe::crt_operator_delete(copy);
            -9
        }
        3 => {
            write_u8(copy + 0xc8, value as u8);
            -11
        }
        4 => {
            write_u8(copy + 0xc9, value as u8);
            -11
        }
        5 => {
            write_u8(copy + 0xca, value as u8);
            -11
        }
        6 => {
            write_u16(copy + 0xcb, value as u16);
            -11
        }
        7 => {
            write_u16(copy + 0xcd, value as u16); // tel04_2 the decider
            -11
        }
        8 => {
            write_u16(copy + 0xcf, value as u16);
            -11
        }
        _ => 0,
    }
}

/// Returns the value of widget `widget` of the Display `this`, 0 for −1.
/// Original: `Display::get_widget_data` (0x4739a0, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn display_get_widget_data(this: usize, widget: u32) -> u32 {
    let w = widget as i16;
    if w == -1 {
        return 0;
    }
    read_u32((this as isize + w as isize * display_list::ITEM_SIZE as isize + display::ITEM_VALUE as isize) as usize)
}

/// Comparer for `open_friendly_tour_nation_screen`: only the original runs; the params checked against the plan.
pub(crate) unsafe extern "C" fn compare_open_friendly_tour_nation_screen(club: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(IDX_OPEN_FRIENDLY_TOUR_NATION_SCREEN)(club);
    if club != 0 && get_screen_param(0) == club {
        let got = club_pages_params(5);
        SCOREBOARD.record(got == [club, 0, 0, 0, 1], || format!("friendly tour nation screen (open): params {:x?}", got));
    }
}

/// Comparer for `friendly_tour_nation_screen_build`: side by side by the display list.
pub(crate) unsafe extern "C" fn compare_friendly_tour_nation_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_FRIENDLY_TOUR_NATION_SCREEN_BUILD);
    compare_match_screen_build("friendly tour nation screen", &[], || o(), || friendly_tour_nation_screen_build());
}

/// Comparer for `friendly_tour_nation_screen_handle_event`: ids 1..3 side by side over the params; opening the tour
/// dates and the menu bar only the original.
pub(crate) unsafe extern "C" fn compare_friendly_tour_nation_screen_handle_event(widget: u32) -> i32 {
    let both = matches!(club_pages_id(widget), 1..=3);
    club_pages_compare_input("friendly tour nation screen input", widget, original_fn(IDX_FRIENDLY_TOUR_NATION_SCREEN_HANDLE_EVENT), friendly_tour_nation_screen_handle_event, both, &[])
}

/// Comparer for `friendly_tour_club_screen_build`: side by side by the display list.
pub(crate) unsafe extern "C" fn compare_friendly_tour_club_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_FRIENDLY_TOUR_CLUB_SCREEN_BUILD);
    compare_match_screen_build("friendly tour club screen", &[], || o(), || friendly_tour_club_screen_build());
}

/// Comparer for `friendly_tour_club_screen_handle_event`: Cancel side by side over the params and the tour record's
/// +0x103; the menu bar only the original.
pub(crate) unsafe extern "C" fn compare_friendly_tour_club_screen_handle_event(widget: u32) -> i32 {
    let tour = get_screen_param(0);
    let both = tour != 0 && club_pages_id(widget) == 1;
    club_pages_compare_input("friendly tour club screen input", widget, original_fn(IDX_FRIENDLY_TOUR_CLUB_SCREEN_HANDLE_EVENT), friendly_tour_club_screen_handle_event, both, &[(tour + 0x103, 4)])
}

/// Comparer for `open_configure_tournament_screen`: only the original runs (it allocates the copy); the params and
/// the copy checked against the settings.
pub(crate) unsafe extern "C" fn compare_open_configure_tournament_screen(settings: usize, arg: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_OPEN_CONFIGURE_TOURNAMENT_SCREEN)(settings, arg);
    if settings != 0 && get_screen_param(0) == settings {
        let copy = get_screen_param(2);
        let same = copy != 0 && core::slice::from_raw_parts(copy as *const u8, 0xd1) == core::slice::from_raw_parts(settings as *const u8, 0xd1);
        SCOREBOARD.record(same && get_screen_param(1) == arg as usize, || format!("configure tournament screen (open): copy {:#x} same {} arg {}", copy, same, get_screen_param(1)));
    }
}

/// Comparer for `configure_tournament_screen_build`: side by side by the display list plus the working copy and the
/// green colour made once.
pub(crate) unsafe extern "C" fn compare_configure_tournament_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_CONFIGURE_TOURNAMENT_SCREEN_BUILD);
    let copy = get_screen_param(2);
    if copy == 0 {
        return o();
    }
    compare_match_screen_build("configure tournament screen", &[(copy, 0xd1), (0xadab40, 1), (0xad9db6, 2)], || o(), || configure_tournament_screen_build());
}

/// Comparer for `configure_tournament_screen_handle_event`: the settings choices side by side over the params and the
/// working copy; Cancel and Ok (they free) only the original.
pub(crate) unsafe extern "C" fn compare_configure_tournament_screen_handle_event(widget: u32) -> i32 {
    let copy = get_screen_param(2);
    let both = copy != 0 && !matches!(club_pages_id(widget), 1 | 2);
    club_pages_compare_input("configure tournament screen input", widget, original_fn(IDX_CONFIGURE_TOURNAMENT_SCREEN_HANDLE_EVENT), configure_tournament_screen_handle_event, both, &[(copy, 0xd1)])
}

/// Comparer for `display_get_widget_data`: both read, the values side by side.
pub(crate) unsafe extern "thiscall" fn compare_display_get_widget_data(this: usize, widget: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_DISPLAY_GET_WIDGET_DATA)(this, widget);
    let r = display_get_widget_data(this, widget);
    SCOREBOARD.record(o == r, || format!("Display::get_widget_data({:#x}, {}) original={:#x} rust={:#x}", this, widget as i16, o, r));
    o
}

/// Opens the invited clubs screen of tournament `t` (builder `tournament_invitations_screen_build`, handler
/// `tournament_invitations_screen_handle_event`; a new history entry only for the host); no tournament shows the
/// club_screens.cpp 13860 box. The host is the current human when he manages the tournament's club. Params: 0 the
/// tournament, 1 the host flag; for the host the tournament marks the invite session (+0x29 = 1, +0x2d = `arg`).
/// Original: `open_tournament_invitations_screen` (0x4739d0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_tournament_invitations_screen(t: usize, arg: u32) {
    if t == 0 {
        club_pages_error_box(0x3624);
        return;
    }
    let host = tournament_hosted_here(t);
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x473ad0usize, 0x473fb0usize, host as u32, 0usize, 0) == 0 {
        return;
    }
    covered!("tel04_3 open the invitations");
    set_screen_param(0, t);
    set_screen_param(1, host as usize);
    if host {
        write_u32(t + 0x29, 1);
        write_u32(t + 0x2d, arg);
    }
}

/// Builds the invited clubs screen of a tournament (param 0; param 1 the host flag).
///
/// The host club's background picture, the tournament's name in its kit colours, "Invited Clubs" (or "Invited
/// Nations" for a national team) and the menu bar; the host gets Invite Another (id 1, plain text when every slot is
/// taken). One row per taken slot: the club (yellow for a human's) with, for a club, its national team and its division
/// (linked when it has a competition object), for a national team its continent; then "Hosts" for the first slot,
/// "Accepted Invitation" (status 3) or "Waiting For Reply". The host has Back (id 2) and Next (−3, text without a next
/// screen); the others the Back and Next buttons.
/// Original: `tournament_invitations_screen_build` (0x473ad0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn tournament_invitations_screen_build() {
    let lang = club_pages_lang();
    let format = club_pages_format();
    let row_weights: [u8; 4] = [5, 3, 3, 3];
    let button_weights: [u8; 2] = [3, 1];
    let t = get_screen_param(0);
    let host = get_screen_param(1);
    let club = read_usize(t);
    covered!("tel04_3 invitations build");
    exe::background_set_club_picture(club, 0, 0);
    club_pages_title(club, t + 0x31);
    lang(JOB_TEXT, if exe::database_is_national_team(club) != 0 { 0x990fe8usize } else { 0x990fd8 }); // Invited Nations / Clubs
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    exe::menubar_build(4, 0);
    let (grey, pale, yellow) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY), job_colour(G_GUI_COLOUR_YELLOW));
    let slots = |t: usize| read_i16(t + 0x13);
    let slot = |t: usize, k: i8| read_usize(t + 0x15 + k as isize as usize * 4);
    let mut taken: i8 = 0;
    let mut k: i8 = 0;
    while (k as i16) < slots(t) {
        if slot(t, k) != 0 {
            taken = taken.wrapping_add(1);
        }
        k = k.wrapping_add(1);
    }
    let mut top = 0x91;
    if host != 0 {
        lang(JOB_TEXT, 0x990fc8); // "Invite Another"
        if (taken as i16) < slots(t) {
            tel26_add_button([0x28f, 0x91, 0x30c, 0xa5, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 1, 0]);
        } else {
            exe::screenmanager_add_text_item(G_SCREEN_MANAGER, 0x28f, 0x91, 0x30c, 0xa5, 0x30, grey, 0x2c, 1, pale, JOB_TEXT as u32, 1);
        }
        top = 0xaa;
    }
    let rows = ((0x207 - top) / exe::font_get_height(3)) as u8 as u32;
    let area = club_pages_area([0x6e, top as u32, 0x30c, 0x217, 4, row_weights.as_ptr() as u32, rows, 0, 2, 0, u32::MAX]);
    let mut row = 0u32;
    let mut k: i8 = 0;
    while (k as i16) < slots(t) {
        let c = Club(slot(t, k));
        if !c.is_null() {
            format(JOB_TEXT, 0x987ff0, c.addr() + club::SHORT_NAME); // "  %s"
            let colour = if exe::human_manager_is_human_club(c, 0, 1, 0, 0) != 0 { yellow } else { pale };
            tel26_area_button([area, 0, row, 1, 0, 1, 3, colour, JOB_TEXT as u32, 0x7d5, c.addr() as u32]);
            let n = c.nation();
            if exe::database_is_national_team(c) != 0 {
                tel26_area_text([area, 1, row, 1, 0, 1, 2, yellow, (n.continent() + 4) as u32]);
            } else {
                let team = exe::database_get_national_team(n);
                tel26_area_button([area, 1, row, 1, 0, 1, 2, yellow, (n.addr() + nation::SHORT_NAME) as u32, 0x7d5, team as u32]);
                let division = c.division().addr();
                if division != 0 {
                    if read_u32(read_usize(G_COMP_OBJECT_TABLE) + read_u32(division) as usize * 4) != 0 {
                        tel26_area_button([area, 2, row, 1, 0, 1, 2, job_colour(0xaebdd4), (division + 0x38) as u32, 0x7d0, division as u32]);
                    } else {
                        tel26_area_text([area, 2, row, 1, 0, 1, 2, job_colour(0xaebdd4), (division + 0x38) as u32]);
                    }
                }
            }
            let status = if k == 0 {
                0x990fc0usize // "Hosts"
            } else if read_u8(t + 0x25 + k as isize as usize) == 3 {
                0x990fac // "Accepted Invitation"
            } else {
                0x990f98 // "Waiting For Reply"
            };
            lang(JOB_TEXT, status);
            tel26_area_text([area, 3, row, 1, 0, 1, 2, grey, JOB_TEXT as u32]); // tel04_3 the reply column
            row += 1;
        }
        k = k.wrapping_add(1);
    }
    if host == 0 {
        tel26_back_next();
        return;
    }
    let buttons = club_pages_area([100, 0x22b, 0x316, 0x24e, 2, button_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    lang(JOB_TEXT, 0x9880a8); // "Back"
    tel26_area_button([buttons, 0, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 2, 0]);
    lang(JOB_TEXT, 0x9880a0); // "Next"
    if exe::screenmanager_is_forward_unavailable(G_SCREEN_MANAGER) != 0 {
        tel26_area_text([buttons, 1, 0, 0x30, grey, 0x2c, 3, pale, JOB_TEXT as u32]);
    } else {
        tel26_area_button([buttons, 1, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 0xffff_fffd, 0]);
    }
}

/// Answers a click on the invited clubs screen: id 2 (Back) ends the invite session (+0x29 and +0x2d cleared, −9);
/// id 1 (Invite Another) opens the opponent choice (by continent for a national team, by nation for a club; −4);
/// anything else goes to the menu bar.
/// Original: `tournament_invitations_screen_handle_event` (0x473fb0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn tournament_invitations_screen_handle_event(widget: u32) -> i32 {
    let t = get_screen_param(0);
    let club = Club(read_usize(t));
    match club_pages_id(widget) {
        2 => {
            write_u32(t + 0x29, 0); // tel04_3 the invite session
            write_u32(t + 0x2d, 0);
            -9
        }
        1 => {
            let national = exe::database_is_national_team(club);
            let n = club.nation();
            if national != 0 {
                exe::open_choose_friendly_teams_screen(1, n.continent(), n, 2);
            } else {
                exe::open_choose_friendly_teams_screen(3, n.continent(), n, 1);
            }
            -4
        }
        _ => club_pages_menu_bar(widget),
    }
}

/// Opens the friendly invitation of a news item (`manager`, `id`; not found: the club_screens.cpp 14121 box): builder
/// `friendly_invitation_screen_build`, handler `friendly_invitation_screen_handle_event`. Params: 0 and 1 the news item,
/// 2 the friendly match (`friendly_invitation_match`).
/// Original: `open_friendly_invitation_screen` (0x4740c0, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn open_friendly_invitation_screen(manager: usize, id: usize) {
    let Some(fixture) = friendly_invitation_match(manager, id) else {
        club_pages_error_box(0x3729);
        return;
    };
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x474280usize, 0x4744b0usize, 1, 0usize, 0) != 0 {
        covered!("tel04_3 open the friendly invitation");
        set_screen_param(0, manager);
        set_screen_param(1, id);
        set_screen_param(2, fixture);
    }
}

/// Builds the friendly invitation (param 2, the friendly match): the background picture and kit colours of the invited
/// club (the one that is not the inviter, +0xc), "Friendly Match" and "Invitation", the menu bar, the question "Accept
/// the invitation to a friendly match with <inviter> on <date> ?" (the inviter linked) and Back, No and Yes.
/// Original: `friendly_invitation_screen_build` (0x474280, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn friendly_invitation_screen_build() {
    let lang = club_pages_lang();
    let fixture = get_screen_param(2);
    let inviter = Club(read_usize(fixture + 0xc));
    let mut club = read_usize(fixture + 0x10);
    if inviter.addr() == club {
        club = read_usize(fixture + 0x14);
    }
    covered!("tel04_3 friendly invitation build");
    exe::background_set_club_picture(club, 0, 0);
    lang(JOB_TEXT, 0x991058); // "Friendly Match"
    club_pages_title(club, JOB_TEXT);
    lang(JOB_TEXT, 0x99104c); // "Invitation"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    exe::menubar_build(4, 0);
    let mut date = [0u8; 0xc8];
    exe::date_get_long_date_string(fixture, date.as_mut_ptr() as usize, 0xc8, 0xff);
    let c = (inviter.short_gender_name() as i8) as i32;
    lang(JOB_TEXT, 0x990ff8, c, c, 100, inviter.addr() + club::SHORT_NAME, date.as_ptr()); // "Accept the invitation to a friendly match with ..."
    let area: unsafe extern "C" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) = core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    area(0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, (inviter.addr() + club::SHORT_NAME) as u32, 0x7d5, inviter.addr() as u32, 0);
    invitation_answer_buttons();
}

/// Answers the friendly invitation (params 0 and 1 the news item, 2 the friendly match). Back: −9. No or Yes: the
/// match's reply (+0x18) is 4 or 3; the news item gets slot 5 the reply and slot 6 cleared; a reply news item (type 7:
/// the home and away clubs' ids, the inviter's, the day, the year and the reply) goes to the inviter's manager and the
/// friendlies process the reply (−9). Anything else goes to the menu bar.
/// Original: `friendly_invitation_screen_handle_event` (0x4744b0, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn friendly_invitation_screen_handle_event(widget: u32) -> i32 {
    let manager = get_screen_param(0);
    let id = get_screen_param(1);
    let fixture = get_screen_param(2);
    let answer = club_pages_id(widget);
    match answer {
        1 => -9,
        2 | 3 => {
            write_u8(fixture + 0x18, if answer == 3 { 3 } else { 4 }); // tel04_3 the friendly reply
            let reply = read_i8(fixture + 0x18) as i32 as u32;
            club_pages_set_news_slot(manager, id, 5, reply);
            club_pages_set_news_slot(manager, id, 6, 0);
            let mut item = [0u32; 0x40];
            let it = club_pages_news_item(&mut item, 7, 0);
            exe::newsitem_set_slot(it, 0, read_u32(read_usize(fixture + 0x10)));
            exe::newsitem_set_slot(it, 1, read_u32(read_usize(fixture + 0x14)));
            exe::newsitem_set_slot(it, 2, read_u32(read_usize(fixture + 0xc)));
            exe::newsitem_set_slot(it, 3, read_i16(fixture) as i32 as u32);
            exe::newsitem_set_slot(it, 4, read_u16(fixture + 2) as u32);
            exe::newsitem_set_slot(it, 5, read_i8(fixture + 0x18) as i32 as u32);
            exe::news_send_to_club_manager(read_usize(G_NEWS), it, read_usize(fixture + 0xc));
            exe::friendly_process_friendly_reply(read_usize(G_FRIENDLY), fixture, 0);
            -9
        }
        _ => club_pages_menu_bar(widget),
    }
}

/// Opens the tournament invitation of a news item (`manager`, `id`; not found: the club_screens.cpp 14287 box): builder
/// `tournament_invitation_screen_build`, handler `tournament_invitation_screen_handle_event`. Params: 0 and 1 the news
/// item, 2 the tournament, 3 the invited club (`tournament_invitation_target`).
/// Original: `open_tournament_invitation_screen` (0x474720, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn open_tournament_invitation_screen(manager: usize, id: usize) {
    let Some((t, club)) = tournament_invitation_target(manager, id) else {
        club_pages_error_box(0x37cf);
        return;
    };
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x4748a0usize, 0x474ac0usize, 1, 0usize, 0) != 0 {
        covered!("tel04_3 open the tournament invitation");
        for (k, v) in [(0u32, manager), (1, id), (2, t), (3, club)] {
            set_screen_param(k, v);
        }
    }
}

/// Builds the tournament invitation (param 2 the tournament, 3 the invited club). A tournament without a host only
/// clears the background picture. Otherwise the club's background picture, the tournament's name in its kit colours,
/// "Invitation", the menu bar, the question "Accept the invitation to the <tournament> tournament starting on <date> ?"
/// and Back, No and Yes.
/// Original: `tournament_invitation_screen_build` (0x4748a0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn tournament_invitation_screen_build() {
    let lang = club_pages_lang();
    let t = get_screen_param(2);
    let club = get_screen_param(3);
    if read_usize(t) == 0 {
        exe::background_set_picture(0usize);
        return;
    }
    covered!("tel04_3 tournament invitation build");
    exe::background_set_club_picture(club, 0, 0);
    club_pages_title(club, t + 0x31);
    lang(JOB_TEXT, 0x99104c); // "Invitation"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    exe::menubar_build(4, 0);
    let mut date = [0u8; 0xc8];
    exe::date_get_long_date_string(t + 4, date.as_mut_ptr() as usize, 0xc8, 0xff);
    lang(JOB_TEXT, 0x991068, 100, t + 0x31, date.as_ptr()); // "Accept the invitation to the <tournament> tournament ..."
    tel26_text_area([0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 0]);
    invitation_answer_buttons();
}

/// Answers the tournament invitation (params 0 and 1 the news item, 2 the tournament, 3 the invited club). A tournament
/// without a host, or Back: −9. No or Yes: the club's invite slot gets status 3 (Yes) or is freed (No: status 0, slot
/// cleared); the news item gets slot 3 the reply (3 or 4); a reply news item (type 10: the host's id, the club's, the
/// start day and year, the reply and the tournament's +0x64) goes to the host's manager and the friendlies process the
/// replies (−9). Anything else goes to the menu bar.
/// Original: `tournament_invitation_screen_handle_event` (0x474ac0, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn tournament_invitation_screen_handle_event(widget: u32) -> i32 {
    let manager = get_screen_param(0);
    let id = get_screen_param(1);
    let t = get_screen_param(2);
    let club = get_screen_param(3);
    if read_usize(t) == 0 {
        return -9;
    }
    let answer = club_pages_id(widget);
    match answer {
        1 => -9,
        2 | 3 => {
            let mut k: i8 = 0;
            while (k as i16) < read_i16(t + 0x13) {
                let slot = t + 0x15 + k as isize as usize * 4;
                if read_usize(slot) == club {
                    if answer == 3 {
                        write_u8(t + 0x25 + k as isize as usize, 3);
                    } else {
                        write_u8(t + 0x25 + k as isize as usize, 0);
                        write_u32(slot, 0);
                    }
                    break;
                }
                k = k.wrapping_add(1);
            }
            let reply = if answer == 3 { 3 } else { 4 };
            club_pages_set_news_slot(manager, id, 3, reply);
            let mut item = [0u32; 0x40];
            let it = club_pages_news_item(&mut item, 10, 0);
            exe::newsitem_set_slot(it, 0, read_u32(read_usize(t)));
            exe::newsitem_set_slot(it, 1, read_u32(club));
            exe::newsitem_set_slot(it, 2, read_i16(t + 4) as i32 as u32);
            exe::newsitem_set_slot(it, 3, read_u16(t + 6) as u32);
            exe::newsitem_set_slot(it, 4, reply); // tel04_3 the tournament reply
            exe::newsitem_set_slot(it, 5, read_i8(t + 0x64) as i32 as u32);
            exe::news_send_to_club_manager(read_usize(G_NEWS), it, read_usize(t));
            exe::friendly_process_tournament_replies(read_usize(G_FRIENDLY), t, 0, 0);
            -9
        }
        _ => club_pages_menu_bar(widget),
    }
}

/// Opens the squad numbers screen before a match, when it is needed: the club of the match screen context (+0xe6) in
/// its fixture (*+0xe2) has the squad flag set (+0x40 at home, +0x41 away) and uses squad numbers
/// (`SQUAD_MANAGER::club_uses_squad_numbers`). Each selected player (slots 0 to 10 + the fixture's substitutes, +0x4d &
/// 0xf) who is in the database and still belongs to the club (for a national team his national team, +0x24; for a club
/// his club, +0x39, unless there are no national teams or the club is a national or all-star team) and has a squad
/// number (not negative) opens the screen (params: the slot's manager, −1, 1, 1, ..., the fixture) and returns 1; a
/// player who no longer belongs is taken out of the selection. Otherwise 0.
/// Original: `open_squad_numbers_screen_for_match` (0x474e00, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_squad_numbers_screen_for_match(context: usize) -> u32 {
    let club = read_usize(context + 0xe6);
    let fixture = read_usize(read_usize(context + 0xe2));
    let flag = if club == read_usize(fixture + screen_fixture::HOME) { read_u8(fixture + 0x40) } else { read_u8(fixture + 0x41) };
    if flag == 0 || exe::squad_manager_club_uses_squad_numbers(read_usize(G_SQUAD_MANAGER), club, 0) == 0 {
        return 0;
    }
    let mut k: i8 = 0;
    while (k as i32) < ((read_u8(fixture + 0x4d) & 0xf) as i32 + 0xb) {
        // the slot goes in the high bytes of the original's argument slot (the context pointer): the same dword here
        let slot = (context as u32 & 0xffff_ff00) | k as u8 as u32;
        let p = exe::tactics_get_selected_player(read_usize(G_TACTICS), club, slot, 0);
        if p != 0 && read_i32(p) < read_i32(G_STAFF_COUNT) {
            let belongs_to = if exe::database_is_national_team(club) != 0 {
                Some(read_usize(p + 0x24))
            } else if read_usize(G_NATIONAL_TEAMS) != 0 && exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), club) == 0 {
                Some(read_usize(p + 0x39))
            } else {
                None
            };
            if belongs_to.is_some_and(|c| c != club) {
                exe::tactics_clear_selection_slot(read_usize(G_TACTICS), club, slot, 0);
            } else if exe::squad_manager_get_squad_number(read_usize(G_SQUAD_MANAGER), p, club) >= 0 {
                covered!("tel04_3 squad numbers before the match");
                return open_squad_numbers_with(|| {
                    let manager = read_usize((G_SCREEN_RECORDS_MANAGER as isize + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize * 0x300) as usize);
                    squad_numbers_plan(manager, usize::MAX, 1, 1, [0, 0, 0], fixture)
                }) as u32;
            }
        }
        k = k.wrapping_add(1);
    }
    0
}

/// Opens the squad numbers screen of a news item (`manager`, `id`; not found: the club_screens.cpp 14580 box): kind 1,
/// flag set when the item's type is 0x18.
/// Original: `open_squad_numbers_screen` (0x475070, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn open_squad_numbers_screen(manager: usize, id: usize) {
    let mut item = [0u32; 0x40];
    let it = club_pages_news_item(&mut item, 0, 0);
    if club_pages_find_news(manager, id, it) == 0 {
        club_pages_error_box(0x38f4);
        return;
    }
    covered!("tel04_3 squad numbers from the news");
    let flag = (item[0] == 0x18) as usize;
    open_squad_numbers_with(|| squad_numbers_plan(manager, id, 1, flag, [0, 0, 0], 0));
}

/// Opens the European squad screen of a news item (`manager`, `id`; not found: the club_screens.cpp 14633 box): kind 2,
/// the item's slots 2, 4 and 5 (bytes) in params 0xb..0xd.
/// Original: `open_european_squad_screen` (0x4752b0, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn open_european_squad_screen(manager: usize, id: usize) {
    let mut item = [0u32; 0x40];
    let it = club_pages_news_item(&mut item, 0, 0);
    if club_pages_find_news(manager, id, it) == 0 {
        club_pages_error_box(0x3929);
        return;
    }
    covered!("tel04_3 european squad");
    let bytes = [2u32, 4, 5].map(|s| exe::newsitem_get_slot(it, s) as u8 as i8 as isize as usize);
    open_squad_numbers_with(|| squad_numbers_plan(manager, id, 2, 0, bytes, 0));
}

/// Comparer for `open_tournament_invitations_screen`: only the original runs; the params and the invite session marks
/// checked against the plan.
pub(crate) unsafe extern "C" fn compare_open_tournament_invitations_screen(t: usize, arg: u32) {
    let host = t != 0 && tournament_hosted_here(t);
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_OPEN_TOURNAMENT_INVITATIONS_SCREEN)(t, arg);
    if t != 0 && get_screen_param(0) == t {
        let ok = get_screen_param(1) == host as usize && (!host || (read_u32(t + 0x29) == 1 && read_u32(t + 0x2d) == arg));
        SCOREBOARD.record(ok, || format!("tournament invitations screen (open {:#x}): host {} params {:x?}", t, host, club_pages_params(2)));
    }
}

/// Comparer for `tournament_invitations_screen_build`: side by side by the display list.
pub(crate) unsafe extern "C" fn compare_tournament_invitations_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TOURNAMENT_INVITATIONS_SCREEN_BUILD);
    compare_match_screen_build("tournament invitations screen", &[], || o(), || tournament_invitations_screen_build());
}

/// Comparer for `tournament_invitations_screen_handle_event`: Back side by side over the params and the invite session
/// marks; the opponent choice and the menu bar only the original.
pub(crate) unsafe extern "C" fn compare_tournament_invitations_screen_handle_event(widget: u32) -> i32 {
    let t = get_screen_param(0);
    let both = t != 0 && club_pages_id(widget) == 2;
    club_pages_compare_input("tournament invitations screen input", widget, original_fn(IDX_TOURNAMENT_INVITATIONS_SCREEN_HANDLE_EVENT), tournament_invitations_screen_handle_event, both, &[(t + 0x29, 8)])
}

/// Comparer for `open_friendly_invitation_screen`: only the original runs; the params checked against the plan.
pub(crate) unsafe extern "C" fn compare_open_friendly_invitation_screen(manager: usize, id: usize) {
    let plan = friendly_invitation_match(manager, id);
    original_fn::<unsafe extern "C" fn(usize, usize)>(IDX_OPEN_FRIENDLY_INVITATION_SCREEN)(manager, id);
    if let Some(fixture) = plan {
        if get_screen_param(0) == manager && get_screen_param(1) == id {
            let got = club_pages_params(3);
            SCOREBOARD.record(got == [manager, id, fixture], || format!("friendly invitation screen (open): params {:x?} x match {:#x}", got, fixture));
        }
    }
}

/// Comparer for `friendly_invitation_screen_build`: side by side by the display list.
pub(crate) unsafe extern "C" fn compare_friendly_invitation_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_FRIENDLY_INVITATION_SCREEN_BUILD);
    compare_match_screen_build("friendly invitation screen", &[], || o(), || friendly_invitation_screen_build());
}

/// Comparer for `friendly_invitation_screen_handle_event`: Back side by side; the reply (news and the friendlies'
/// records) and the menu bar only the original.
pub(crate) unsafe extern "C" fn compare_friendly_invitation_screen_handle_event(widget: u32) -> i32 {
    let both = club_pages_id(widget) == 1;
    club_pages_compare_input("friendly invitation screen input", widget, original_fn(IDX_FRIENDLY_INVITATION_SCREEN_HANDLE_EVENT), friendly_invitation_screen_handle_event, both, &[])
}

/// Comparer for `open_tournament_invitation_screen`: only the original runs; the params checked against the plan.
pub(crate) unsafe extern "C" fn compare_open_tournament_invitation_screen(manager: usize, id: usize) {
    let plan = tournament_invitation_target(manager, id);
    original_fn::<unsafe extern "C" fn(usize, usize)>(IDX_OPEN_TOURNAMENT_INVITATION_SCREEN)(manager, id);
    if let Some((t, club)) = plan {
        if get_screen_param(0) == manager && get_screen_param(1) == id {
            let got = club_pages_params(4);
            SCOREBOARD.record(got == [manager, id, t, club], || format!("tournament invitation screen (open): params {:x?} x {:#x} {:#x}", got, t, club));
        }
    }
}

/// Comparer for `tournament_invitation_screen_build`: side by side by the display list.
pub(crate) unsafe extern "C" fn compare_tournament_invitation_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TOURNAMENT_INVITATION_SCREEN_BUILD);
    compare_match_screen_build("tournament invitation screen", &[], || o(), || tournament_invitation_screen_build());
}

/// Comparer for `tournament_invitation_screen_handle_event`: Back (and a tournament without a host) side by side; the
/// reply and the menu bar only the original.
pub(crate) unsafe extern "C" fn compare_tournament_invitation_screen_handle_event(widget: u32) -> i32 {
    let t = get_screen_param(2);
    let both = t != 0 && (read_usize(t) == 0 || club_pages_id(widget) == 1);
    club_pages_compare_input("tournament invitation screen input", widget, original_fn(IDX_TOURNAMENT_INVITATION_SCREEN_HANDLE_EVENT), tournament_invitation_screen_handle_event, both, &[])
}

/// Comparer for `open_squad_numbers_screen_for_match`: only the original runs (it opens a screen and edits the
/// selection); when it opened the screen, its params are checked against the plan.
pub(crate) unsafe extern "C" fn compare_open_squad_numbers_screen_for_match(context: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_OPEN_SQUAD_NUMBERS_SCREEN_FOR_MATCH)(context);
    if r == 1 {
        let manager = read_usize((G_SCREEN_RECORDS_MANAGER as isize + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize * 0x300) as usize);
        let plan = squad_numbers_plan(manager, usize::MAX, 1, 1, [0, 0, 0], read_usize(read_usize(context + 0xe2)));
        let got = club_pages_params(0x13);
        SCOREBOARD.record(got == plan, || format!("squad numbers screen (open for the match): params {:x?} x plan {:x?}", got, plan));
    }
    r
}

/// Comparer for `open_squad_numbers_screen`: only the original runs; the params checked against the plan.
pub(crate) unsafe extern "C" fn compare_open_squad_numbers_screen(manager: usize, id: usize) {
    let mut item = [0u32; 0x40];
    let it = club_pages_news_item(&mut item, 0, 0);
    let found = club_pages_find_news(manager, id, it) != 0;
    original_fn::<unsafe extern "C" fn(usize, usize)>(IDX_OPEN_SQUAD_NUMBERS_SCREEN)(manager, id);
    if found && get_screen_param(0) == manager && get_screen_param(1) == id {
        let plan = squad_numbers_plan(manager, id, 1, (item[0] == 0x18) as usize, [0, 0, 0], 0);
        let got = club_pages_params(0x13);
        SCOREBOARD.record(got == plan, || format!("squad numbers screen (open): params {:x?} x plan {:x?}", got, plan));
    }
}

/// Comparer for `open_european_squad_screen`: only the original runs; the params checked against the plan.
pub(crate) unsafe extern "C" fn compare_open_european_squad_screen(manager: usize, id: usize) {
    let mut item = [0u32; 0x40];
    let it = club_pages_news_item(&mut item, 0, 0);
    let found = club_pages_find_news(manager, id, it) != 0;
    let bytes = [2u32, 4, 5].map(|s| if found { exe::newsitem_get_slot(it, s) as u8 as i8 as isize as usize } else { 0 });
    original_fn::<unsafe extern "C" fn(usize, usize)>(IDX_OPEN_EUROPEAN_SQUAD_SCREEN)(manager, id);
    if found && get_screen_param(0) == manager && get_screen_param(1) == id {
        let plan = squad_numbers_plan(manager, id, 2, 0, bytes, 0);
        let got = club_pages_params(0x13);
        SCOREBOARD.record(got == plan, || format!("european squad screen (open): params {:x?} x plan {:x?}", got, plan));
    }
}

/// Rule: the squad numbers screen (and its European squad mode) answers its buttons: the views, sorts and filters are
/// written back to the screen parameters; the numbers are given, swapped, freed or worked out automatically; the
/// European registrations are ticked or worked out automatically; the submission asks for confirmation, or says what
/// is missing; the answers of the confirmation boxes apply the squad numbers or the European squad and acknowledge the
/// news item.
///
/// `squad_numbers_screen_handle_event(widget)`. Parameters: 0 the human manager, 1 the news item id, 2 the mode (1 the
/// squad numbers, otherwise the European squad), 3 the next season flag, 4 the view, 5 the sort, 6 the statistics
/// category, 9 and 0xa the attribute and the other column chosen for the sort, 0xb, 0xd the European registration
/// rule bytes, 0xe and 0xf the counts of registered and reserve players, 0x10 the squad size asked, 0x11 the squad copy
/// (50 player slots), 0x12 the fixture (squad numbers for a match). A local news item (`NewsItem::NewsItem(0, 0)`,
/// never destroyed, as in the original). The club is the manager's `staff::CLUB_JOB`.
///
/// On a network screen ([0xb77c84]) the confirmation answers come first: pending command 0x16 (both pending words
/// cleared) answered 2 (yes): without a fixture, the news item (`News::find_item_by_id`, the "Error" box at line 0x4057
/// and 0 when missing) gets slot 1 (type 0x18) or slot 7 (type 0xfa8) set to 1 (`News::set_item_slot`); then, for
/// every database player of the club's squad (a 200-byte copy, the box at line 0x4066 when it cannot be allocated) with a
/// positive provisional number, the number is made final (negated); with a fixture, pending command 0x43; -9. Pending
/// 0x17 (cleared) answered 2: slot 3 set to 1 and slot 6 to the registered count (param 0xe), then the European
/// registrations made final (1 -> 2, 4 -> 0; the box at line 0x409a); -9. Another answer clears the pending argument.
///
/// Then, for a widget other than -1, the list's scroll row (`ScreenManager::get_area_scroll_row` of the area in
/// param 8) is kept in param 7. By the widget's id (0 and -1: the default command id [0xdd6bde]): 1 a view (1, 2, 4,
/// 8, 0x10, 0x20 and 0x1000 replace the view and keep the attribute group, 0x40..0x400 replace the attribute group;
/// one already shown falls through to the menu bar); 2 a statistics category (param 6) other than the current one; 3
/// the automatic button: squad numbers with a fixture, every player selected for the match (`Tactics::find_selection_slot`
/// on the club's side, inside the eleven plus the fixture's substitutes, byte +0x4d & 0xf) without a negative number
/// gets one (`SQUAD_MANAGER::allocate_squad_number`); without a fixture, the whole club numbered
/// (`SQUAD_MANAGER::assign_club_squad_numbers`, next season) or one free number allocated (no player); European squad,
/// the registrations cleared (`staff::SQUAD_SELECTED_FOR` 1 -> 0) and chosen again
/// (`SQUAD_MANAGER::select_euro_registered_players` with param 0xb), and the counts of 1s and 4s written to params 0xe and
/// 0xf (once per player found, as the original); 4 the clear button: the numbers freed (a positive number back to 0) or
/// the registrations undone (1 -> 0, 4 -> 2, the counts to 0); 6 a sort (param 5, bit 0x1000000 reversing an already
/// chosen sort); 7 and 8 the attribute and the other column sorts (bits 0x400 and 0x800, params 9 and 0xa); 5 a number
/// dragged: onto another number, the two holders swap (the same number: its holder freed); onto a player (id 0x7d7 or
/// 9; the scan walks the club's own squad slots, `club::SQUAD`), the player takes the number and its old holder is
/// freed, unless either holds a provisional (negative) number; 9 a player given the first free number from 1 (up to
/// 50); 0xa a player's European registration cycled (none -> 1 registered, 1 -> none, 2 -> 4 reserve, 4 -> 2, with the
/// counts); 0xb leaving the European squad mode 2 undoes the registrations, -9; 0xc the submission: squad numbers, with
/// a fixture each selected player (`Tactics::get_selected_player` up to the eleven plus substitutes) needs a number,
/// otherwise "Incomplete" (code 0); otherwise "Please Confirm" with the right question (new numbers for a match, the
/// forthcoming season, the current season) and code 0x16; European squad, fewer registered (0xe - 0xf + 0xd) than the
/// squad size asked (at most 20) with rule byte 0xb at 1 or byte 0xd at 0: "Incomplete" (not all registered, or fewer
/// than 20), else "Please Confirm" with code 0x17; -4; 0xd the transfer status screen of the item's person, -4; any other
/// id goes to the menu bar (`exe::menubar_handle_command`, then `exe::menubar_handle_object_link`). The redraws return
/// -0xb.
/// Original: `squad_numbers_screen_handle_event` (0x47a9e0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn squad_numbers_screen_handle_event(widget: u32) -> i32 {
    covered!("tel05_1 squad numbers events");
    let mut news_item = [0u32; 0x38]; // the local NewsItem (0xe0 bytes; the original never destroys it)
    let item = news_item.as_mut_ptr() as usize;
    exe::new_newsitem(item, 0, 0);
    let manager = Staff(get_screen_param(0));
    let club = manager.club_job();
    let news_id = get_screen_param(1) as u32;
    let mode = get_screen_param(2) as u8;
    let next_season = get_screen_param(3) as u32;
    let view = get_screen_param(4) as u32;
    let sort = get_screen_param(5) as u32;
    let category = get_screen_param(6) as u8;
    let attribute_sort = get_screen_param(9) as u8;
    let other_sort = get_screen_param(0xa) as u8;
    let rule_b = get_screen_param(0xb) as u8;
    let rule_d = get_screen_param(0xd) as u8;
    let registered = get_screen_param(0xe) as u8 as i8;
    let reserves = get_screen_param(0xf) as u8 as i8;
    let squad_size = get_screen_param(0x10) as u8 as i8;
    let squad = get_screen_param(0x11);
    let fixture = Fixture(get_screen_param(0x12));
    let news = || read_usize(G_NEWS);
    if read_u32(G_SCREEN_MANAGER + 0x3834) != 0 {
        // the confirmation boxes' answers (pending command and argument)
        let code = read_u32(G_PENDING_COMMAND_CODE);
        if code == 0x16 {
            let answer = read_u32(G_PENDING_COMMAND_ARG);
            write_u32(G_PENDING_COMMAND_CODE, 0);
            write_u32(G_PENDING_COMMAND_ARG, 0);
            if answer == 2 {
                covered!("tel05_1 squad numbers confirmed");
                if fixture.is_null() {
                    if exe::news_find_item_by_id(news(), get_screen_param(0), news_id, item) == 0 {
                        show_error_at(SQUAD_NUMBERS_SOURCE_FILE, 0x4057);
                        return 0;
                    }
                    let kind = read_u32(item);
                    if kind == 0x18 || kind == 0xfa8 {
                        exe::news_set_item_slot(news(), get_screen_param(0), (news_id) as u32 as usize, if kind == 0x18 { 1 } else { 7 }, 1, (0) as usize as u32);
                    }
                }
                let block = squad_numbers_collect(club.addr(), 0x4066);
                if block == 0 {
                    return -9;
                }
                for k in 0..SQUAD_NUMBERS_SLOTS {
                    let p = read_usize(block + 4 * k);
                    if squad_numbers_in_database(p) {
                        let n = squad_numbers_number(p, club.addr());
                        if n > 0 {
                            squad_numbers_set(p, club.addr(), n.wrapping_neg());
                        }
                    }
                }
                exe::crt_free(block);
                if !fixture.is_null() {
                    write_u32(G_PENDING_COMMAND_CODE, 0x43);
                }
                return -9;
            }
        } else if code == 0x17 {
            write_u32(G_PENDING_COMMAND_CODE, 0);
            if read_u32(G_PENDING_COMMAND_ARG) == 2 {
                covered!("tel05_1 european squad confirmed");
                write_u32(G_PENDING_COMMAND_ARG, 0);
                exe::news_set_item_slot(news(), get_screen_param(0), (news_id) as u32 as usize, 3, 1, (0) as usize as u32);
                exe::news_set_item_slot(news(), get_screen_param(0), (news_id) as u32 as usize, 6, registered as i32 as u32, (0) as usize as u32);
                let block = squad_numbers_collect(club.addr(), 0x409a);
                if block == 0 {
                    return -9;
                }
                for k in 0..SQUAD_NUMBERS_SLOTS {
                    let p = read_usize(block + 4 * k);
                    if squad_numbers_in_database(p) {
                        match read_u8(p + staff::SQUAD_SELECTED_FOR) {
                            1 => write_u8(p + staff::SQUAD_SELECTED_FOR, 2),
                            4 => write_u8(p + staff::SQUAD_SELECTED_FOR, 0),
                            _ => {}
                        }
                    }
                }
                exe::crt_free(block);
                return -9;
            }
            write_u32(G_PENDING_COMMAND_ARG, 0);
        }
    }
    let w = widget as u16 as i16;
    if w != -1 {
        let row = exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(8));
        set_screen_param(7, row);
    }
    let id = item_id_or_default(w);
    let value = tel26_item_value(w);
    if id == 1 && view & value == 0 {
        let mask = match value {
            1 | 2 | 4 | 8 | 0x10 | 0x20 | 0x1000 => 0xffff_efc0,
            0x40 | 0x80 | 0x100 | 0x200 | 0x400 => 0xffff_f83f,
            _ => return -0xb,
        };
        set_screen_param(4, ((view & mask) | value) as usize);
        return -0xb;
    }
    if id == 2 && value as u8 != category {
        set_screen_param(6, value as usize);
        return -0xb;
    }
    if id == 3 {
        covered!("tel05_1 automatic numbers");
        if mode == 1 {
            if !fixture.is_null() {
                let away = club != fixture.home_club();
                let side = if away { fixture::AWAY_SIDE_MARK } else { fixture::HOME_SIDE_MARK };
                let flag = (read_u8(fixture.addr() + side) == 0) as u32;
                for k in 0..SQUAD_NUMBERS_SLOTS {
                    let p = read_usize(squad + 4 * k);
                    if !squad_numbers_in_database(p) {
                        continue;
                    }
                    let slot = exe::tactics_find_selection_slot(read_usize(G_TACTICS), club, p, flag) as i8;
                    // the fixture's substitutes: byte +0x4d & 0xf
                    if slot >= 0 && (slot as i32) < ((fixture.squad_rules() as u8) & 0xf) as i32 + 0xb && squad_numbers_number(p, club.addr()) >= 0 {
                        exe::squad_manager_allocate_squad_number(read_usize(G_SQUAD_MANAGER), club, p);
                    }
                }
            } else if next_season != 0 {
                exe::squad_manager_assign_club_squad_numbers(read_usize(G_SQUAD_MANAGER), club, 1);
            } else {
                exe::squad_manager_allocate_squad_number(read_usize(G_SQUAD_MANAGER), club, 0usize);
            }
            return -0xb;
        }
        for k in 0..SQUAD_NUMBERS_SLOTS {
            let p = read_usize(squad + 4 * k);
            if p != 0 && read_u8(p + staff::SQUAD_SELECTED_FOR) == 1 {
                write_u8(p + staff::SQUAD_SELECTED_FOR, 0);
            }
        }
        // simplification: param 0xb is a byte in a stack slot whose upper bytes are garbage in the original (a char)
        exe::squad_manager_select_euro_registered_players(read_usize(G_SQUAD_MANAGER), club, rule_b as u32, 1);
        for wanted in [(1u8, 0xeu32), (4, 0xf)] {
            let mut count = 0i8;
            for k in 0..SQUAD_NUMBERS_SLOTS {
                let p = read_usize(squad + 4 * k);
                if squad_numbers_in_database(p) && read_u8(p + staff::SQUAD_SELECTED_FOR) == wanted.0 {
                    count = count.wrapping_add(1);
                    set_screen_param(wanted.1, count as i32 as usize);
                }
            }
        }
        return -0xb;
    }
    if id == 4 {
        if mode == 1 {
            for k in 0..SQUAD_NUMBERS_SLOTS {
                let p = read_usize(squad + 4 * k);
                if squad_numbers_in_database(p) && squad_numbers_number(p, club.addr()) > 0 {
                    squad_numbers_set(p, club.addr(), 0);
                }
            }
            return -0xb;
        }
        for k in 0..SQUAD_NUMBERS_SLOTS {
            let p = read_usize(squad + 4 * k);
            if squad_numbers_in_database(p) {
                match read_u8(p + staff::SQUAD_SELECTED_FOR) {
                    1 => write_u8(p + staff::SQUAD_SELECTED_FOR, 0),
                    4 => write_u8(p + staff::SQUAD_SELECTED_FOR, 2),
                    _ => {}
                }
            }
        }
        set_screen_param(0xe, 0);
        set_screen_param(0xf, 0);
        return -0xb;
    }
    if id == 6 {
        let reversed = if sort & value != 0 && sort & 0x100_0000 == 0 { sort | 0x100_0000 } else { sort & 0xfeff_ffff };
        set_screen_param(5, (value | (reversed & 0xc100_0000)) as usize);
        return -0xb;
    }
    if id == 7 || id == 8 {
        let (bit, chosen, param) = if id == 7 { (0x400u32, attribute_sort, 9u32) } else { (0x800, other_sort, 0xa) };
        let reversed = if sort & bit != 0 && value as u8 == chosen && sort & 0x100_0000 == 0 { sort | 0x100_0000 } else { sort & 0xfeff_ffff };
        set_screen_param(5, ((reversed & (0xc100_0000 | bit)) | bit) as usize);
        set_screen_param(param, value as usize);
        return -0xb;
    }
    let d = current_human_display();
    if id == 5 {
        let target = if w == -1 { -1 } else { read_i16(display_item_at(d, w) + display_item::DROP_TARGET) };
        if target != -1 {
            let target_id = item_id_or_default(target);
            let target_value = tel26_item_value(target);
            if target_id == 5 {
                covered!("tel05_1 number dragged onto a number");
                if value == target_value {
                    let holder = squad_numbers_holder(squad, club.addr(), value as u8 as i8 as i32);
                    if holder != 0 {
                        squad_numbers_set(holder, club.addr(), 0);
                    }
                    return -0xb;
                }
                let first = squad_numbers_holder(squad, club.addr(), value as u8 as i8 as i32);
                let second = squad_numbers_holder(squad, club.addr(), target_value as u8 as i8 as i32);
                if first != 0 {
                    squad_numbers_set(first, club.addr(), target_value as u8 as i8);
                }
                if second != 0 {
                    squad_numbers_set(second, club.addr(), value as u8 as i8);
                }
                return -0xb;
            }
            if target_id == 0x7d7 || target_id == 9 {
                covered!("tel05_1 number dragged onto a player");
                let n = value as u8 as i8;
                let player = target_value as usize;
                let mut holder = 0usize;
                for k in 0..SQUAD_NUMBERS_SLOTS {
                    if (club.squad(k) as u32) == 0 {
                        continue;
                    }
                    let p = read_usize(squad + 4 * k);
                    // a missing player is asked too (number of a null person) and, matching, keeps the scan going, as
                    // the original
                    if (p == 0 || read_i32(p) < read_i32(G_STAFF_COUNT)) && (squad_numbers_number(p, club.addr()) as i32).wrapping_abs() == n as i32 {
                        holder = p;
                        if holder != 0 {
                            break;
                        }
                    }
                }
                if player != 0 && squad_numbers_number(player, club.addr()) < 0 {
                    return -0xb;
                }
                if holder != 0 && squad_numbers_number(holder, club.addr()) < 0 {
                    return -0xb;
                }
                if player != 0 {
                    squad_numbers_set(player, club.addr(), n);
                }
                if holder != 0 {
                    squad_numbers_set(holder, club.addr(), 0);
                }
                return -0xb;
            }
        }
    }
    if id == 9 {
        covered!("tel05_1 first free number");
        let mut n = 0i8;
        loop {
            if squad_numbers_holder(squad, club.addr(), n as i32 + 1) == 0 {
                squad_numbers_set(value as usize, club.addr(), n.wrapping_add(1));
                return -0xb;
            }
            n += 1;
            if n >= 0x32 {
                return -0xb;
            }
        }
    }
    if id == 0xa {
        let p = value as usize;
        match read_u8(p + staff::SQUAD_SELECTED_FOR) {
            1 => {
                write_u8(p + staff::SQUAD_SELECTED_FOR, 0);
                set_screen_param(0xe, (registered as i32 - 1) as usize);
            }
            2 => {
                write_u8(p + staff::SQUAD_SELECTED_FOR, 4);
                set_screen_param(0xf, (reserves as i32 + 1) as usize);
            }
            4 => {
                write_u8(p + staff::SQUAD_SELECTED_FOR, 2);
                set_screen_param(0xf, (reserves as i32 - 1) as usize);
            }
            _ => {
                write_u8(p + staff::SQUAD_SELECTED_FOR, 1);
                set_screen_param(0xe, (registered as i32 + 1) as usize);
            }
        }
        return -0xb;
    }
    if id == 0xb {
        if mode == 2 {
            for k in 0..SQUAD_NUMBERS_SLOTS {
                let p = read_usize(squad + 4 * k);
                if squad_numbers_in_database(p) {
                    match read_u8(p + staff::SQUAD_SELECTED_FOR) {
                        1 => write_u8(p + staff::SQUAD_SELECTED_FOR, 0),
                        4 => write_u8(p + staff::SQUAD_SELECTED_FOR, 2),
                        _ => {}
                    }
                }
            }
        }
        return -9;
    }
    if id == 0xc {
        covered!("tel05_1 submission");
        let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
        if mode == 1 {
            if !fixture.is_null() {
                let away = club != fixture.home_club();
                let side = if away { fixture::AWAY_SIDE_MARK } else { fixture::HOME_SIDE_MARK };
                let flag = (read_u8(fixture.addr() + side) == 0) as i32;
                let mut k = 0i8;
                while (k as i32) < ((fixture.squad_rules() as u8) & 0xf) as i32 + 0xb {
                    // simplification: the slot is a byte in a stack slot whose upper bytes are param 0xb's (a char)
                    let p = exe::tactics_get_selected_player(read_usize(G_TACTICS), club, k as u8 as u32, flag);
                    if p != 0 && read_usize(p + staff::PLAYER) != 0 && squad_numbers_number(p, club.addr()) == 0 {
                        let colours = squad_numbers_kit_colours(club.addr());
                        let text = lang(0xde1f64, 0x9912d4, 0, colours.0, colours.1, 0); // "Not all players have been assigned a squad number."
                        squad_numbers_confirm(0x991308, 0x9912c8, text, 0, colours, 0); // "Submit Squad Numbers", "Incomplete"
                        return -4;
                    }
                    k += 1;
                }
            }
            let colours = squad_numbers_kit_colours(club.addr());
            let question = if !fixture.is_null() {
                0x9913a4 // "Submit new squad number changes ?"
            } else if next_season != 0 {
                0x99135c // "Submit the current set of squad numbers for the forthcoming season ?"
            } else {
                0x991320 // "Submit the squad number changes for the current season ?"
            };
            lang(0xde1f64, question);
            squad_numbers_confirm(0x991308, 0x9909b8, 0xde1f64, 1, colours, 0x16); // "Submit Squad Numbers", "Please Confirm"
            return -4;
        }
        let asked = if squad_size < 0x14 { squad_size as i32 } else { 0x14 };
        let have = registered as i32 - reserves as i32 + rule_d as i8 as i32;
        if have < asked && (rule_b == 1 || rule_d == 0) {
            let colours = squad_numbers_kit_colours(club.addr());
            // "Not all players have been registered." / "Not enough players have been registered - you must register at least 20."
            lang(0xde1f64, if squad_size < 0x14 { 0x9912a0 } else { 0x991254 });
            squad_numbers_confirm(0x99123c, 0x9912c8, 0xde1f64, 0, colours, 0); // "Submit European Squad", "Incomplete"
            return -4;
        }
        let colours = squad_numbers_kit_colours(club.addr());
        let text = lang(0xde1f64, 0x991218, 1, colours.0, colours.1, 0x17); // "Submit the current European squad ?"
        squad_numbers_confirm(0x99123c, 0x9909b8, text, 1, colours, 0x17);
        return -4;
    }
    if id == 0xd {
        exe::transfer_screens_open_transfer_status_screen((value) as u32 as usize, 0, -1, 0);
        return -4;
    }
    let r = exe::menubar_handle_command(widget);
    if r != 0 {
        return r;
    }
    exe::menubar_handle_object_link(widget)
}

/// Comparer for `squad_numbers_screen_handle_event`.
///
/// Only the original runs: the handler writes the squad numbers and the European registrations, acknowledges news items,
/// opens the confirmation and transfer status screens and answers the menu bar (none of it runs twice). The copy is
/// proven in Rust mode by its branches.
pub(crate) unsafe extern "C" fn compare_squad_numbers_screen_handle_event(widget: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_SQUAD_NUMBERS_SCREEN_HANDLE_EVENT)(widget)
}

/// Rule: leaving the squad numbers screen frees its squad copy.
///
/// `squad_numbers_screen_leave()`: the block of parameter 0x11, when there is one, freed (`free`) and the parameter
/// cleared.
/// Original: `squad_numbers_screen_leave` (0x47c070, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn squad_numbers_screen_leave() {
    if get_screen_param(0x11) != 0 {
        covered!("tel05_1 squad copy freed");
        exe::crt_free(get_screen_param(0x11));
        set_screen_param(0x11, 0);
    }
}

/// Comparer for `squad_numbers_screen_leave`: only the original runs (the block is freed once); afterwards parameter
/// 0x11 must be empty.
pub(crate) unsafe extern "C" fn compare_squad_numbers_screen_leave() {
    original_fn::<unsafe extern "C" fn()>(IDX_SQUAD_NUMBERS_SCREEN_LEAVE)();
    let left = get_screen_param(0x11);
    SCOREBOARD.record(left == 0, || format!("squad_numbers_screen_leave (squad numbers screen): parameter 0x11 still {:#x}", left));
}

/// Rule: choosing a player to add to a transfer bid opens the part exchange screen for the bid, with the default view,
/// sort and filter.
///
/// `open_add_part_exchange_player_screen(bid)`: `exe::screenmanager_open_screen` with the builder 0x47c180 and the
/// handler 0x4814e0 (1, 0, 0); when it opens, param 0 the bid and params 1..9 `PART_EXCHANGE_OPEN_PARAMS`.
/// Original: `open_add_part_exchange_player_screen` (0x47c0b0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_add_part_exchange_player_screen(bid: usize) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x47c180usize, 0x4814e0usize, 1, 0usize, 0) == 0 {
        return;
    }
    covered!("tel05_1 part exchange screen opened");
    set_screen_param(0, bid);
    for (k, v) in PART_EXCHANGE_OPEN_PARAMS.iter().enumerate() {
        set_screen_param(k as u32 + 1, *v as usize);
    }
}

/// Comparer for `open_add_part_exchange_player_screen`: opening a screen does not run twice, so only the original runs;
/// when the new screen holds the bid in param 0, params 1..9 are checked against `PART_EXCHANGE_OPEN_PARAMS`.
pub(crate) unsafe extern "C" fn compare_open_add_part_exchange_player_screen(bid: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(IDX_OPEN_ADD_PART_EXCHANGE_PLAYER_SCREEN)(bid);
    if get_screen_param(0) != bid {
        return;
    }
    let differ: Vec<u32> = (1..=9u32).filter(|&k| get_screen_param(k) as u32 != PART_EXCHANGE_OPEN_PARAMS[k as usize - 1]).collect();
    SCOREBOARD.record(differ.is_empty(), || format!("open_add_part_exchange_player_screen (part exchange screen): parameters {:x?} differ from the plan", differ));
}

/// Rule: the part exchange screen (Add Player To Transfer Bid) lists the players of the bidding club the selling club
/// would take in a part exchange, in the chosen view, sorted and filtered as the menus say; when none can be offered, it
/// says whether the selling club is not interested or the players' wage demands are too high.
///
/// `part_exchange_screen_build()`, from the screen parameters 0 (the bid), 3, 4 (the view), 5 (the sort), 6..8 and 9 (the
/// filter), 1 (the scroll row). The pieces: `part_exchange_header` (the title, the bid text and the menus),
/// `part_exchange_candidates` (the club's filtered squad and the players that can be offered), `part_exchange_list_area`
/// (the list's area and the column headings), `part_exchange_rows` (one row per candidate) and `part_exchange_footer`
/// (the message when nobody can be offered, the scroll row, the area in param 2 and the Back and Next buttons). Local
/// objects as the original: the bid text (`Text::Text(0)`, destroyed at the end), four lists (`LLIST`) and a contract
/// (`STAFF_CONTRACT::STAFF_CONTRACT`, no destructor).
/// Original: `part_exchange_screen_build` (0x47c180, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn part_exchange_screen_build() {
    covered!("tel05_2 part exchange screen");
    let mut text = [0usize; 4]; // the bid text (Text, 8 bytes)
    exe::new_text(text.as_mut_ptr(), 0usize);
    let mut s = PartExchange {
        bid: 0,
        choosing: 0,
        view: 0,
        view_flags: 0,
        sort: 0,
        category: 0,
        option7: 0,
        option8: 0,
        filter: 0,
        club: 0,
        player: 0,
        seller: 0,
        human: 0,
        kit_back: part_exchange_colour(0xae3184),
        kit_text: part_exchange_colour(0xaebdf0),
        title: [0; 0xc8],
        name: [0; 0x17c],
        bar: 0,
        not_interested: 0,
        too_dear: 0,
        shown: 0,
        squad_head: 0,
        offers: [0; 0x20],
        negotiations: [0; 0x20],
        squad: [0; 0x20],
        candidates: [0; 0x20],
        contract: [0; 0x14],
    };
    part_exchange_llist(&mut s.offers, true);
    part_exchange_llist(&mut s.negotiations, true);
    exe::new_staff_contract(s.contract.as_mut_ptr() as usize);
    part_exchange_llist(&mut s.squad, true);
    let mut w = PartExchangeWeights {
        two: [6, 5, 0x1e, 0x10, 6, 5, 0x1e, 0x10],
        contract: [5, 5, 0x18, 0xe, 0xe, 0xf, 0xb, 0, 0, 0, 0, 0, 0xa],
        stats: [5, 5, 0x18, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0xa],
        attributes: [5, 5, 0x18, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0xa],
        other: [5, 5, 0x18, 8, 6, 6, 6, 0xa, 0xc, 6, 0, 0, 0xa],
        selection: [5, 5, 0x18, 0xe, 0xa, 0xc, 6, 6, 6, 0, 0, 0, 0xa],
    };
    part_exchange_llist(&mut s.candidates, true);
    part_exchange_header(&mut s, text.as_mut_ptr() as usize);
    part_exchange_candidates(&mut s);
    let area = part_exchange_list_area(&mut s, &mut w);
    part_exchange_rows(&mut s, area, &w);
    part_exchange_footer(&mut s, area);
    part_exchange_llist(&mut s.candidates, false);
    part_exchange_llist(&mut s.squad, false);
    part_exchange_llist(&mut s.negotiations, false);
    part_exchange_llist(&mut s.offers, false);
    exe::drop_text(text.as_mut_ptr());
}

/// Comparer for `part_exchange_screen_build`.
///
/// Through the display list (`compare_by_display_list_with`), plus [0xae1080] (the player's contract, written), the
/// attribute tables the attributes view sorts in place (0x98de84..0x98ded8) and the bid's offered list (its walk). What
/// it also does outside the list runs twice and is idempotent: the background picture, each candidate's training caught
/// up to the week, the local lists and contract.
pub(crate) unsafe extern "C" fn compare_part_exchange_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_PART_EXCHANGE_SCREEN_BUILD);
    let mut extra = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize), (0x98de84usize, 0x54usize)];
    let offered = read_usize(get_screen_param(0) + 4);
    if offered != 0 {
        extra.push((offered, 0x10));
    }
    compare_by_display_list_with("part_exchange_screen_build (part exchange screen)", &extra, || o(), || part_exchange_screen_build());
}

/// Rule: the squad numbers screen (and its European squad mode) shows the club's players in the chosen view with their
/// squad numbers (the 50 numbers on two rows above and below the list, who holds each) or their European registration,
/// sorted and filtered as the menus say; for a match only the selected players are listed.
///
/// `squad_numbers_screen_build()`, from the screen parameters 0 (the human manager: his club, `staff::CLUB_JOB`), 2 (the
/// mode), 4 (view), 5 (sort), 6, 9, 0xa (list options), 0xb..0xf (the European rule bytes and counts), 0x11 (the squad
/// copy), 0x12 (the fixture), 7 (the scroll row). The pieces: `squad_numbers_build_header` (the squad copy, the title, the
/// menus), `squad_numbers_build_top` (the numbers grid or the selected count, the club's list), `squad_numbers_build_area`
/// (the list's area and headings), `squad_numbers_build_rows` and the footer (the scroll row from param 7, the area
/// written to param 8, the Back (link 0xb) and Submit (link 0xc) buttons). A local list (`LLIST`), destroyed at the end.
/// Original: `squad_numbers_screen_build` (0x475520, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn squad_numbers_screen_build() {
    covered!("tel05_3 squad numbers screen");
    let mut s = SquadNumbersScreen {
        club: 0,
        mode: 0,
        view: 0,
        sort: 0,
        category: 0,
        option9: 0,
        slot_a: 0,
        rule_b: 0,
        squad_size: 0,
        rule_d: 0,
        registered: 0,
        reserves: 0,
        squad: 0,
        fixture: 0,
        side: 0,
        title: [0; 0xc8],
        list: [0; 0x20],
        last_area: 0,
        top: 0,
        bottom: 0,
    };
    part_exchange_llist(&mut s.list, true);
    let mut w = PartExchangeWeights {
        two: [6, 5, 0x1e, 0x10, 6, 5, 0x1e, 0x10],
        contract: [5, 5, 0x18, 0xe, 0xe, 0xf, 0xb, 0, 0, 0, 0, 0, 0xa],
        stats: [5, 5, 0x18, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0xa],
        attributes: [5, 5, 0x18, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0xa],
        other: [5, 5, 0x18, 8, 6, 6, 6, 0xa, 0xc, 6, 0, 0, 0xa],
        selection: [5, 5, 0x18, 0xe, 0xa, 0xc, 6, 6, 6, 0, 0, 0, 0xa],
    };
    let bottom_weights = [3u8, 1];
    if squad_numbers_build_header(&mut s) {
        squad_numbers_build_top(&mut s);
        let area = squad_numbers_build_area(&mut s, &mut w);
        squad_numbers_build_rows(&mut s, area);
        let scroll = get_screen_param(7);
        if area as u16 != 0xffff {
            let d = screens_current_display(G_SCREEN_MANAGER);
            exe::area_set_scroll_row(d.wrapping_add(4).wrapping_add((area as i16 as isize as usize).wrapping_mul(0xbf1)), scroll);
        }
        set_screen_param(8, area as i16 as i32 as usize);
        let bar = tel26_add_area(screens_current_display(G_SCREEN_MANAGER), [100, 0x22b, 0x316, 0x24e, 2, bottom_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
        let (c317e, c3184) = (part_exchange_colour(0xae317e), part_exchange_colour(0xae3184));
        part_exchange_lang()(0xde1f64, 0x9880a8); // "Back"
        tel26_area_button([bar, 0, 0, 0x30, c317e, 0xc, 3, c3184, 0xde1f64, 0xb, 0]);
        part_exchange_lang()(0xde1f64, 0x9910c0); // "Submit"
        tel26_area_button([bar, 1, 0, 0x30, c317e, 0xc, 3, c3184, 0xde1f64, 0xc, 0]);
    }
    part_exchange_llist(&mut s.list, false);
}

/// Comparer for `squad_numbers_screen_build`.
///
/// Through the display list (`compare_by_display_list_with`), plus [0xae1080], the attribute tables sorted in place and
/// the squad copy (param 0x11, 200 bytes). Without a copy the original allocates one and writes it to param 0x11: only
/// the original runs, and the copy is proven in Rust mode. What it also does outside the list runs twice and is
/// idempotent: the background picture, each player's training caught up to the week.
pub(crate) unsafe extern "C" fn compare_squad_numbers_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_SQUAD_NUMBERS_SCREEN_BUILD);
    let squad = get_screen_param(0x11);
    if squad == 0 {
        return o();
    }
    let extra = [(G_CONTRACT_LOOKUP_RESULT, 4usize), (0x98de84usize, 0x54usize), (squad, 200)];
    compare_by_display_list_with("squad_numbers_screen_build (squad numbers screen)", &extra, || o(), || squad_numbers_screen_build());
}

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
    let o = original_fn::<unsafe extern "C" fn(u32, *mut i32) -> u8>(IDX_CLUB_RECORD_TYPE_TO_CATEGORY);
    if !(1..=32).contains(&(record_type as u8 as i8 as i32)) {
        return o(record_type, out_menu_index);
    }
    let reg: Vec<(usize, usize)> = if out_menu_index.is_null() { Vec::new() } else { vec![(out_menu_index as usize, 4)] };
    let (a, b, same, _, _) = crate::comparison::side_by_side(&reg, || o(record_type, out_menu_index), || club_record_type_to_category(record_type, out_menu_index));
    SCOREBOARD.record(same && a == b, || format!("club_record_type_to_category({}) (club history screen) returned {} and Rust {}", record_type as u8 as i8, a, b));
    a
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
    let club = Club(club);
    if club.is_null() || list == 0 {
        club_screens_error_box(0x503f);
        return 0;
    }
    covered!("tel06_1 squad list");
    let linked = exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1);
    exe::llist_clear(list);
    exe::virtual_staff_fill_viewed_club_squad(read_usize(G_VIRTUAL_STAFF), club);
    let squad = if players != 0 { players } else { club.addr() + club::SQUAD };
    let mut kept = [0usize; 50];
    let mut count = 0usize;
    for k in 0..50 {
        let p = Staff(read_usize(squad + 4 * k));
        if p.is_null() || p.player().addr() == 0 {
            continue;
        }
        if squad_list_passes(club.addr(), p.addr(), reserves, filter_flags, team_sheet, fixture, linked) {
            kept[count] = p.addr();
            count += 1;
        }
    }
    if count == 0 {
        return 1;
    }
    write_usize(SQUAD_LIST_SORT_CLUB, club.addr());
    let mut no_link = 0u32;
    let partner = exe::club_get_linked_b_team(club, &mut no_link, 1);
    write_usize(0xadab3c, if no_link != 0 || partner == 0 { club.addr() } else { partner });
    write_u8(0xadab4a, opt4 as u8);
    write_u8(0xadab33, name_style as u8);
    write_u32(0xadab38, team_sheet as u32);
    write_u32(0xadab34, reserves);
    write_u8(0xad9db4, opt6 as u8);
    write_u8(0xad9db5, opt5 as u8);
    write_u32(0xadab44, fixture as u32);
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    // the slot's manager, [0xb74c26 + slot * 0x300] (0x484618): screen manager +0x7d6, not the parameter block
    write_usize(SQUAD_LIST_SORT_MANAGER, read_usize((G_SCREEN_MANAGER as isize + MENUBAR_SLOT_MANAGER as isize + slot * 0x300) as usize));
    let arr = kept.as_mut_ptr() as usize;
    let cmp = squad_list_comparator(sort_flags, club.addr(), sort_variant);
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

/// Comparer for `build_filtered_sorted_squad_list` (`compare_squad_list`); without a club or a list (the error box)
/// only the original runs.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn compare_build_filtered_sorted_squad_list(club: usize, reserves: u32, sort_flags: u32, name_style: u32, opt4: u32,
    opt5: u32, opt6: u32, filter_flags: u32, list: usize, team_sheet: usize, fixture: usize, top_n: i32, players: usize, sort_variant: u32) -> u32 {
    type F = unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, usize, usize, usize, i32, usize, u32) -> u32;
    let o = original_fn::<F>(IDX_BUILD_FILTERED_SORTED_SQUAD_LIST);
    if club == 0 || list == 0 {
        return o(club, reserves, sort_flags, name_style, opt4, opt5, opt6, filter_flags, list, team_sheet, fixture, top_n, players, sort_variant);
    }
    compare_squad_list("build_filtered_sorted_squad_list", list,
        || o(club, reserves, sort_flags, name_style, opt4, opt5, opt6, filter_flags, list, team_sheet, fixture, top_n, players, sort_variant),
        || build_filtered_sorted_squad_list(club, reserves, sort_flags, name_style, opt4, opt5, opt6, filter_flags, list, team_sheet, fixture, top_n, players, sort_variant))
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
    let o = original_fn::<unsafe extern "C" fn(usize, usize, i32) -> u32>(IDX_BUILD_SQUAD_LIST_FOR_STAFF_HISTORY);
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
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_BUILD_SQUAD_LIST_BY_NAME_FOR_PRINTOUT);
    if club == 0 || list == 0 {
        return o(club, list);
    }
    compare_squad_list("build_squad_list_by_name_for_printout", list, || o(club, list), || build_squad_list_by_name_for_printout(club, list))
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
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u8>(IDX_GET_PLAYER_STATUS_CODE);
    if player == 0 {
        return o(player, club, fixture);
    }
    let mut reg = vec![(0xde1f64usize, 0x7d0usize)];
    reg.extend_from_slice(&crate::rng::STATE_REGIONS);
    let (a, b, same, _, _) = crate::comparison::side_by_side(&reg, || o(player, club, fixture), || crate::rng::without_counting(|| get_player_status_code(player, club, fixture)));
    SCOREBOARD.record(same && a == b, || format!("get_player_status_code({:#x}, {:#x}, {:#x}) (club squad list) returned {:#x} and Rust {:#x}", player, club, fixture, a, b));
    a
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
    let fixture = Fixture(fixture);
    let club = Club(club);
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let mut date = [0u8; 8];
    exe::new_date(date.as_mut_ptr() as usize, fixture.day() as u32, fixture.year() as u32);
    exe::date_get_short_weekday_date_string_no_year((date.as_ptr() as usize) as usize as *mut u32, JOB_TEXT, 0x7d0, 0xff);
    let comp = fixture.competition().addr();
    let date_style = [area, 0, row, 0x30, job_colour(JOB_COLOUR_BE04), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32];
    if date_link_mode > 0 && comp != 0 && fixture_row_comp_loaded(fixture.addr(), comp) && exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), comp) == 0 {
        covered!("tel06_1 fixture row date link");
        let s = date_style;
        tel26_area_button([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7], s[8], date_link_mode as u32, fixture.addr() as u32]);
    } else {
        tel26_area_text(date_style);
    }
    // the result cell, from the club's side
    let (et_home, et_away) = (fixture.home_goals_extra_time() as i32, fixture.away_goals_extra_time() as i32);
    let (home, away) = (fixture.home_goals() as i32, fixture.away_goals() as i32);
    const SCORE: usize = 0x990bb4; // "<%d - home score>:<%d - away score>"
    const POSTPONED: usize = 0x9918e0; // "P:P"
    const ABANDONED: usize = 0x9918dc; // "A:A"
    let home_side = club == fixture.home_club();
    let opponent = Club(read_usize(fixture.addr() + if home_side { fixture::AWAY_CLUB } else { fixture::HOME_CLUB }));
    let plain = [area, 5, row, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32];
    let mut ret = if et_home != -1 {
        if home_side {
            lang(JOB_TEXT, SCORE, et_home, et_away);
            fixture_row_result(fixture.addr(), area, row, 1)
        } else {
            lang(JOB_TEXT, SCORE, et_away, et_home);
            fixture_row_result(fixture.addr(), area, row, 0)
        }
    } else if home == -2 {
        lang(JOB_TEXT, POSTPONED);
        tel26_area_text(plain);
        1
    } else if home_side && home == -3 {
        lang(JOB_TEXT, ABANDONED);
        fixture_row_result(fixture.addr(), area, row, 1)
    } else if home != -1 {
        if home_side {
            lang(JOB_TEXT, SCORE, home, away);
        } else {
            covered!("tel06_1 fixture row away score");
            lang(JOB_TEXT, SCORE, away, home);
        }
        fixture_row_result(fixture.addr(), area, row, 1)
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
        tel26_area_button([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7], s[8], 0x7d2, fixture.addr() as u32]);
        ret = 0;
    }
    // the opponent, its nation, the venue
    let human = exe::human_manager_is_human_club(opponent, 0, 1, 0, 0) != 0;
    sprintf(JOB_TEXT, 0x987ff0, opponent.addr() + club::SHORT_NAME); // "  %s"
    let colour = job_colour(if human { JOB_COLOUR_BDEC } else { JOB_COLOUR_3184 });
    tel26_area_button([area, 1, row, 1, 0, 1, 3, colour, JOB_TEXT as u32, 0x7d5, opponent.addr() as u32]);
    if exe::database_is_national_team(club) == 0 && club.nation() != opponent.nation() {
        let nation = opponent.nation();
        let team = exe::database_get_national_team(nation);
        tel26_area_button([area, 2, row, 1, 0, 1, 2, job_colour(JOB_COLOUR_BDEC), (nation.addr() + nation::THREE_LETTER_NAME) as u32, 0x7d5, team as u32]);
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
    let (colour, linked) = if exe::comp_util_is_league_division(fixture.comp_id()) != 0 {
        (job_colour(JOB_COLOUR_BDEC), comp != 0 && fixture_row_comp_loaded(fixture.addr(), comp))
    } else {
        let colour = job_colour(if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), comp) != 0 { 0xaebdde } else { JOB_COLOUR_BDD4 });
        let loaded = comp != 0 && read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * (fixture.comp_id() as u32) as usize) != 0
            && read_u32(comp) != read_u32(KEY_CLUB_COMP_RESERVE) && exe::friendly_is_own_comp_record(read_usize(G_FRIENDLY), comp) == 0;
        (colour, loaded)
    };
    if linked && fixture_row_comp_current(fixture.addr(), comp) {
        tel26_area_button([area, 4, row, 1, 0, 1, 2, colour, JOB_TEXT as u32, 0x7e5, fixture_row_round_link(fixture.addr())]);
    } else {
        tel26_area_text([area, 4, row, 1, 0, 1, 2, colour, JOB_TEXT as u32]);
    }
    ret
}

/// Comparer for `add_club_fixture_row`: side by side over the display list (`compare_match_screen_build`) and the
/// return.
pub(crate) unsafe extern "C" fn compare_add_club_fixture_row(club: usize, fixture: usize, area: u32, row: u32, date_link_mode: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32, u32, i32) -> u32>(IDX_ADD_CLUB_FIXTURE_ROW);
    let (mut a, mut b) = (0u32, 0u32);
    compare_match_screen_build("club fixture row", &[], || a = o(club, fixture, area, row, date_link_mode), || b = add_club_fixture_row(club, fixture, area, row, date_link_mode));
    SCOREBOARD.record(a == b, || format!("add_club_fixture_row (club fixtures table) returned {} and Rust {}", a, b));
    a
}

/// Rule: the club screen's squad menu lists the club's own sides (first team, 'B' or reserve side, the national
/// Under-21 or Under-23 side), the related clubs (the continent's national teams, or the clubs of the same division),
/// the club's national team and the written history.
///
/// `build_club_squad_navigation_menu(club, colour, colour_text, squad_handler, history_handler, alt_handler,
/// with_squads, senior_mode)`; the popup and its sections are made by `squad_menu_section`, each entry by
/// `squad_menu_entry`. With `with_squads`:
/// - a national team (`exe::database_is_national_team`): a second side (`exe::club_is_national_team` false) lists
///   "Main Squad" (with `senior_mode`) or "'B' Squad" (when it has one, `exe::database_club_has_reserve_squad`) with
///   `alt_handler`, then "Under 23 Squad" (the nation's flag in [G_NATIONAL_TEAMS]+0x36 by nation id) or "Under 21
///   Squad" with the nation's Under-21 side (`exe::database_get_national_u21_team`) and `squad_handler`; a first side
///   lists "Main Squad" with the national team (`exe::database_get_national_team`) and `squad_handler`, and "'B'
///   Squad" with `alt_handler` when the national team has one;
/// - a club: its linked side (`exe::club_get_linked_b_team`) by name with `squad_handler`, or, when the human runs it
///   and it has a reserve side, "Senior Squad" (`senior_mode`) or "Reserve Squad" with `alt_handler`.
/// Then the related clubs (`squad_menu_related_clubs`; a separator counts as an entry), sorted by 0x53ffe0 (reputation)
/// and cut to 28 entries minus those already listed, minus 2 for a club's national team and minus 2 for the history,
/// sorted again by 0x540080 (name), each by name with `alt_handler` (`senior_mode`) or `squad_handler`; the club's
/// national team for a club (by name, `squad_handler`); "History" with `history_handler` when the club's history file
/// exists (`history_his_file_exists`, 0x5f2500, of `exe::club_get_english_name`). Returns 1.
/// Original: `build_club_squad_navigation_menu` (0x481ef0, club_screens.cpp, cdecl).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn build_club_squad_navigation_menu(club: usize, colour: u32, colour_text: u32, squad_handler: u32, history_handler: u32,
    alt_handler: u32, with_squads: u32, senior_mode: u32) -> u32 {
    let club = Club(club);
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    const MAIN_SQUAD: usize = 0x991550;
    const B_SQUAD: usize = 0x991544;
    let mut area = u32::MAX;
    let mut count = 0i32;
    let nation = club.nation().addr();
    if with_squads != 0 {
        if exe::database_is_national_team(club) != 0 {
            squad_menu_section(&mut area, colour, colour_text);
            if exe::club_is_national_team(club) == 0 {
                covered!("tel06_2 squad menu of a second national side");
                if senior_mode != 0 {
                    lang(JOB_TEXT, MAIN_SQUAD);
                    squad_menu_entry(area, JOB_TEXT, alt_handler, 0);
                    count = 1;
                } else if exe::database_club_has_reserve_squad(club) != 0 {
                    lang(JOB_TEXT, B_SQUAD);
                    squad_menu_entry(area, JOB_TEXT, alt_handler, 0);
                    count = 1;
                }
                let under_23 = read_u8(read_usize(read_usize(G_NATIONAL_TEAMS) + 0x36) + read_u32(nation) as usize) != 0;
                lang(JOB_TEXT, if under_23 { 0x991534usize } else { 0x991524 });
                let side = exe::database_get_national_u21_team(nation);
                squad_menu_entry(area, JOB_TEXT, squad_handler, side);
                count += 1;
            } else {
                lang(JOB_TEXT, MAIN_SQUAD);
                squad_menu_entry(area, JOB_TEXT, squad_handler, exe::database_get_national_team(nation));
                count = 1;
                if exe::database_club_has_reserve_squad(exe::database_get_national_team(nation)) != 0 {
                    lang(JOB_TEXT, B_SQUAD);
                    squad_menu_entry(area, JOB_TEXT, alt_handler, exe::database_get_national_team(nation));
                    count = 2;
                }
            }
        } else {
            let mut no_link = 0u32;
            let linked = Club(exe::club_get_linked_b_team(club, &mut no_link, 1));
            if !linked.is_null() {
                squad_menu_section(&mut area, colour, colour_text);
                squad_menu_entry(area, linked.addr() + club::SHORT_NAME, squad_handler, linked.addr());
                count = 1;
            } else if exe::human_manager_current_controls_club(club, 0, 1) != 0 && exe::database_club_has_reserve_squad(club) != 0 {
                squad_menu_section(&mut area, colour, colour_text);
                lang(JOB_TEXT, if senior_mode != 0 { 0x991514usize } else { 0x991504 }); // "Senior Squad" / "Reserve Squad"
                squad_menu_entry(area, JOB_TEXT, alt_handler, 0);
                count = 1;
            }
        }
    }
    let mut related = squad_menu_related_clubs(club.addr());
    let history = exe::history_his_file_exists(exe::club_get_english_name(club)) != 0;
    if !related.is_empty() {
        covered!("tel06_2 squad menu related clubs");
        if squad_menu_section(&mut area, colour, colour_text) {
            count += 1;
        }
        let arr = related.as_mut_ptr() as usize;
        exe::crt_qsort(arr, related.len() as u32, 4, 0x53ffe0usize);
        let mut limit = 0x1c - count;
        if exe::database_is_national_team(club) == 0 && nation != 0 && exe::database_get_national_team(nation) != 0 {
            limit -= 2;
        }
        if history {
            limit -= 2;
        }
        let n = (related.len() as i32).min(limit);
        exe::crt_qsort(arr, n as u32, 4, 0x540080usize);
        for &c in related.iter().take(n.max(0) as usize) {
            squad_menu_entry(area, c + club::SHORT_NAME, if senior_mode != 0 { alt_handler } else { squad_handler }, c);
        }
    }
    if exe::database_is_national_team(club) == 0 && nation != 0 {
        let team = Club(exe::database_get_national_team(nation));
        if !team.is_null() {
            squad_menu_section(&mut area, colour, colour_text);
            squad_menu_entry(area, team.addr() + club::SHORT_NAME, squad_handler, team.addr());
        }
    }
    if history {
        squad_menu_section(&mut area, colour, colour_text);
        lang(JOB_TEXT, 0x9914d4usize, 0xad9c64usize); // "History<%s - COMMENT - written history screen>"
        squad_menu_entry(area, JOB_TEXT, history_handler, 0);
    }
    1
}

/// Comparer for `build_club_squad_navigation_menu`: side by side over the display list (`compare_match_screen_build`)
/// and the return.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn compare_build_club_squad_navigation_menu(club: usize, colour: u32, colour_text: u32, squad_handler: u32,
    history_handler: u32, alt_handler: u32, with_squads: u32, senior_mode: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32) -> u32>(IDX_BUILD_CLUB_SQUAD_NAVIGATION_MENU);
    let (mut a, mut b) = (0u32, 0u32);
    compare_match_screen_build("club squad menu", &[],
        || a = o(club, colour, colour_text, squad_handler, history_handler, alt_handler, with_squads, senior_mode),
        || b = build_club_squad_navigation_menu(club, colour, colour_text, squad_handler, history_handler, alt_handler, with_squads, senior_mode));
    SCOREBOARD.record(a == b, || format!("build_club_squad_navigation_menu (club screen) returned {} and Rust {}", a, b));
    a
}

/// Rule: the club screen's action buttons and the answers to their dialogs: take control, assign a scout, apply for
/// the job, invite for a friendly (or offer one), answer a dates request, invite to or uninvite from the human's
/// tournament.
///
/// `handle_club_screen_action(widget)`. First, when a dialog has answered (`CLUB_TABS_DIALOG_ANSWERED`), by its code
/// (`G_PENDING_COMMAND_CODE`, cleared) and a yes (`G_PENDING_COMMAND_ARG` 2, cleared; codes 0xc..0xf clear the answer
/// also on a no):
/// - 0x41: the scouts' current search goes to the screen's club (`SCOUT_MANAGER::scope_current_search_to_club`,
///   0x7f82f0, param 0): -13;
/// - 0xc: the open invitation (0x5c68b0) gets the club as opponent (+0x10, or +0x14 when +0x10 equals the human club
///   +0xc), is sent (`exe::friendly_send_friendly_request`) and +8 cleared: -10;
/// - 0xd: the open dates request (0x5c6920) gets the club (`Friendly::schedule_assign_opponent`, 0x5c57d0); each of
///   its dates (count +8, entries of 25 bytes from +9: the date, +0x10 the opponent, +0x14 the club, +0x18 the state)
///   with an opponent and state 1 goes to state 2 and becomes a friendly (`exe::friendly_add_friendly_match`, sent when
///   made); +0x103 cleared: -10;
/// - 0xe / 0xf: the tournament (0x5c6830) drops / adds the club (`Friendly::set_tournament_invite`, 0x5c6990), +0x29
///   cleared; a pending +0x2d becomes dialog 0x10 with the tournament (and is cleared): -10.
/// Then the button (`club_tabs_action`), its value (`club_tabs_value`) and the screen's club (param 0, `c` below);
/// the boxes use `confirm_push` with the kit colours of the other club (`exe::database_get_club_kit_colours`):
/// - 0x64 Take Control: `open_take_control_confirm` (0x822a60) of the club: -4;
/// - 0x6a Scout Club: "Club Already Being Watched" when the club is in the session's searches
///   (`SCOUT_MANAGER::is_club_in_session_searches`, 0x7f8500), otherwise "Send scout to watch all ... matches ?"
///   (yes/no, code 0x41): -4;
/// - 0x65 Apply for Job: with a human on the screen slot (`G_SCREEN_RECORDS_MANAGER`), `open_apply_for_job_screen`
///   (0x6b19d0) of the club and -4; otherwise 0;
/// - 0x66 the open invitation (the value): when `c` is free on its date (`exe::friendly_is_club_free_on_date` and
///   `Friendly::get_club_date_status` 1, kind 0, stadium checked) "Invite For Friendly" (with an opponent at +0x10) or
///   "Offer Friendly" (yes/no, code 0xc); otherwise its date is moved to a week from today when its year is before
///   2001 and the "unavailable" box; -4;
/// - 0x67 the dates request (the value): the date of its first entry without an opponent; free: "Invite For Friendly"
///   (code 0xd), otherwise the "unavailable" box; -4;
/// - 0x68 the tournament (the value): "Uninvite To Tournament" (code 0xe); -4;
/// - 0x69 the tournament: free on its date (+4, kind +0xc, stadium not checked): "Invite To Tournament" (code 0xf),
///   otherwise the "unavailable for the tournament" box; -4.
/// Any other button: 0.
/// Original: `handle_club_screen_action` (0x482d10, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn handle_club_screen_action(widget: u32) -> i32 {
    if read_u32(CLUB_TABS_DIALOG_ANSWERED) != 0 {
        if let Some(r) = club_action_dialog_answer() {
            return r;
        }
    }
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let w = widget as i16;
    let value = club_tabs_value(w);
    match club_tabs_action(w) {
        0x64 => {
            exe::open_take_control_confirm(get_screen_param(0), 0);
            -4
        }
        0x6a => {
            covered!("tel06_2 scout club");
            let c = get_screen_param(0);
            let (fg, bg) = club_action_kit(c);
            let (mut title, mut heading) = ([0u8; 0xc8], [0u8; 0xc8]);
            let gender = read_i8(c + club::SHORT_GENDER_NAME) as i32;
            if exe::scout_manager_is_club_in_session_searches(read_usize(G_SCOUT_MANAGER), c) != 0 {
                lang(title.as_mut_ptr() as usize, 0x9918c4usize); // "Assign Scout"
                let text = lang(JOB_TEXT, 0x991880usize, gender, gender, c + club::SHORT_NAME);
                let h = lang(heading.as_mut_ptr() as usize, 0x991864usize); // "Club Already Being Watched"
                club_action_confirm(title.as_ptr() as usize, h, text, 0, fg, bg, 0);
            } else {
                lang(title.as_mut_ptr() as usize, 0x991854usize); // "Assign Scout ?"
                lang(JOB_TEXT, 0x991820usize, gender, gender, c + club::SHORT_NAME);
                let h = lang(heading.as_mut_ptr() as usize, 0x9909b8usize);
                club_action_confirm(title.as_ptr() as usize, h, JOB_TEXT, 1, fg, bg, 0x41);
            }
            -4
        }
        0x65 => {
            let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
            if read_u32((G_SCREEN_RECORDS_MANAGER as isize + slot * 0x300) as usize) == 0 {
                return 0;
            }
            exe::open_apply_for_job_screen(get_screen_param(0));
            -4
        }
        0x66 => {
            covered!("tel06_2 friendly invitation");
            let c = get_screen_param(0);
            if club_action_free(c, value, 0, 1) {
                club_action_friendly_box(c, value, read_usize(value + 0xc), read_u32(value + 0x10) != 0, 0xc, true);
            } else {
                if read_u16(value + 2) < 0x7d1 {
                    let (mut later, mut result) = ([0u8; 8], [0u8; 8]);
                    let p = exe::date_add_days(G_CURRENT_DATE, later.as_mut_ptr() as usize, 7);
                    exe::date_operator_3d(value, result.as_mut_ptr() as usize, p);
                }
                club_action_friendly_box(c, value, read_usize(value + 0xc), true, 0, true);
            }
            -4
        }
        0x67 => {
            let mut date = [0u8; 8];
            exe::date_date_54a110(date.as_mut_ptr() as usize);
            let last = read_i8(value + 8).wrapping_sub(1);
            if last >= 0 {
                for k in (0..=last as isize).rev() {
                    let e = (value as isize + 25 * k + 9) as usize;
                    if read_u32(e + 0x10) == 0 {
                        let mut result = [0u8; 8];
                        exe::date_operator_3d(date.as_mut_ptr() as usize, result.as_mut_ptr() as usize, e);
                    }
                }
            }
            let c = get_screen_param(0);
            let d = date.as_mut_ptr() as usize;
            let code = if club_action_free(c, d, 0, 1) { 0xd } else { 0 };
            club_action_friendly_box(c, d, read_usize(value), true, code, false);
            -4
        }
        0x68 => {
            let c = get_screen_param(0);
            club_action_tournament_box(c, value, 0x991690, 0x9909b8, 0x991678, 1, 0xe); // "Uninvite To Tournament"
            -4
        }
        0x69 => {
            covered!("tel06_2 tournament invitation");
            let c = get_screen_param(0);
            let kind = read_u8(value + 0xc) as u32;
            if club_action_free(c, value + 4, kind, 0) {
                club_action_tournament_box(c, value, 0x991620, 0x9909b8, 0x991608, 1, 0xf); // "Invite To Tournament"
            } else {
                club_action_tournament_box(c, value, 0x9915a4, 0x9916f8, 0x991608, 0, 0);
            }
            -4
        }
        _ => 0,
    }
}

/// Comparer for `handle_club_screen_action`: the answers write to the friendlies and the scouts and open dialogs and
/// screens, so only the original runs; its answer is checked against the plan (`club_action_planned_answer`).
pub(crate) unsafe extern "C" fn compare_handle_club_screen_action(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_HANDLE_CLUB_SCREEN_ACTION);
    let plan = club_action_planned_answer(widget);
    let r = o(widget);
    SCOREBOARD.record(r == plan, || format!("handle_club_screen_action({}) (club screen actions) returned {} and the plan {}", widget as i16, r, plan));
    r
}

/// Rule: the part exchange screen's input: the view and the columns (exclusive groups of bits), the sort column (a
/// second click reverses it), the scroll, adding the picked player to the offer (his value comes off the cash part)
/// and the transfer status screen.
///
/// `part_exchange_screen_handle_event(widget)`, from the screen's parameters 4 (view), 5 (sort), 6 (the sort column),
/// 9 (columns), 7 and 8 (the two other sort columns). A widget other than -1 first keeps the scroll of the list (param
/// 2's area, `exe::screenmanager_get_area_scroll_row`) in param 1. Then by the widget's action (`club_tabs_action`)
/// and value (`club_tabs_value`), each answering -11 unless said:
/// - 4: a view bit not already set replaces its group in param 4 (`PART_EXCHANGE_VIEW_GROUPS`; another bit: nothing);
/// - 9: a column bit not already set replaces its group in param 9 (`PART_EXCHANGE_COLUMN_GROUPS`);
/// - 5: a sort column (low byte) different from param 6 goes to param 6;
/// - 6: param 5 becomes the value plus the reverse flag (`part_exchange_sort` with the value's bits);
/// - 7 / 8: param 5 sorts by column 0x400 / 0x800 (reversed when it already did with the same column in param 7 / 8)
///   and the value goes to param 7 / 8;
/// - 3: -9; 1: the value to param 3;
/// - 2: the player (the value) joins the offer (param 0): its list (+4, made on the first one, `LLIST` of 0xe bytes)
///   gets a 7-byte record (his id, 0, 2, and a byte the original leaves uninitialised: 0 here), and his
///   `staff::VALUE` comes off the cash +0x1b (not below 0): -9;
/// - 0xa: the transfer status screen of the player (0x8f6770, (player, 0, -1, 0)): -4;
/// - otherwise (also 4, 9 and 5 when nothing changes) the menu bar (`exe::menubar_handle_command`, then
///   `exe::menubar_handle_object_link` when that answers 0) answers.
/// Original: `part_exchange_screen_handle_event` (0x4814e0, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn part_exchange_screen_handle_event(widget: u32) -> i32 {
    let p4 = get_screen_param(4) as u32;
    let p5 = get_screen_param(5) as u32;
    let p6 = get_screen_param(6) as u8;
    let p9 = get_screen_param(9) as u32;
    let p7 = get_screen_param(7) as u8;
    let p8 = get_screen_param(8) as u8;
    let w = widget as i16;
    if w != -1 {
        set_screen_param(1, exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(2)));
    }
    let id = club_tabs_action(w);
    let v = club_tabs_value(w) as u32;
    match id {
        4 if p4 & v == 0 => {
            if let Some(n) = part_exchange_pick(p4, v, &PART_EXCHANGE_VIEW_GROUPS) {
                set_screen_param(4, n as usize);
            }
            return -11;
        }
        9 if p9 & v == 0 => {
            if let Some(n) = part_exchange_pick(p9, v, &PART_EXCHANGE_COLUMN_GROUPS) {
                set_screen_param(9, n as usize);
            }
            return -11;
        }
        5 if v as u8 != p6 => {
            set_screen_param(6, v as usize);
            return -11;
        }
        6 => {
            let mut s = if p5 & v != 0 && p5 & 0x1000000 == 0 { p5 | 0x1000000 } else { p5 & !0x1000000 };
            s &= 0xc1000000;
            set_screen_param(5, (v | s) as usize);
            return -11;
        }
        7 => {
            covered!("tel06_2 part exchange sort 7");
            set_screen_param(5, (part_exchange_sort(p5, 0x400, v as u8 == p7) | 0x400) as usize);
            set_screen_param(7, v as usize);
            return -11;
        }
        8 => {
            set_screen_param(5, (part_exchange_sort(p5, 0x800, v as u8 == p8) | 0x800) as usize);
            set_screen_param(8, v as usize);
            return -11;
        }
        3 => return -9,
        1 => {
            set_screen_param(3, v as usize);
            return -11;
        }
        2 => {
            covered!("tel06_2 part exchange player added");
            let offer = get_screen_param(0);
            let player = Staff(v as usize);
            let mut record = [0u8; 7];
            record[0..4].copy_from_slice(&read_u32(player.addr()).to_le_bytes());
            record[5] = 2;
            if read_usize(offer + 4) == 0 {
                let l = exe::crt_operator_new(0xe);
                write_usize(offer + 4, if l != 0 { exe::new_llist(l) } else { 0 });
            }
            exe::llist_add_tail(read_usize(offer + 4), record.as_ptr() as usize, 7);
            let cash = read_i32(offer + 0x1b).wrapping_sub(player.value() as i32);
            write_i32(offer + 0x1b, if cash < 0 { 0 } else { cash });
            return -9;
        }
        0xa => {
            exe::transfer_screens_open_transfer_status_screen(v as usize, 0, -1, 0);
            return -4;
        }
        _ => {}
    }
    let r = exe::menubar_handle_command(widget);
    if r != 0 {
        r
    } else {
        exe::menubar_handle_object_link(widget)
    }
}

/// Comparer for `part_exchange_screen_handle_event`: the clicks that only write screen parameters side by side over
/// the display list and the parameters (`compare_by_display_list_with`) and the answer; adding a player (2) and the
/// transfer status screen (0xa) run only in the original, the answer checked (-9, -4); the menu bar, only the original.
pub(crate) unsafe extern "C" fn compare_part_exchange_screen_handle_event(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_PART_EXCHANGE_SCREEN_HANDLE_EVENT);
    if part_exchange_input_params_only(widget) {
        let (mut a, mut b) = (0i32, 0i32);
        compare_by_display_list_with("part exchange screen input", &[], || a = o(widget), || b = part_exchange_screen_handle_event(widget));
        SCOREBOARD.record(a == b, || format!("part_exchange_screen_handle_event({}) (part exchange screen) returned {} and Rust {}", widget as i16, a, b));
        return a;
    }
    let plan = match club_tabs_action(widget as i16) {
        2 => Some(-9),
        0xa => Some(-4),
        _ => None,
    };
    let r = o(widget);
    if let Some(p) = plan {
        SCOREBOARD.record(r == p, || format!("part_exchange_screen_handle_event({}) (part exchange screen) returned {} and the plan {}", widget as i16, r, p));
    }
    r
}

/// Rule: the club screen offers one action, the first that applies: "Take Control" while the human is creating his
/// manager, "Scout Club", inviting the club to the human's friendly, dates request or tournament (or "Uninvite Club"),
/// "Apply for Job".
///
/// `add_club_screen_action_button(club, colour, colour_text)`, the button made by `club_action_button`:
/// - while a manager is being created (`is_current_manager_being_created`, 0x838de0): nothing for a human club
///   (`exe::human_manager_is_human_club`(club, 1, 1, 0, 0)) or one that cannot be managed (`exe::club_can_be_managed`),
///   otherwise "Take Control" (0x64);
/// - "Scout Club" (0x6a) when the scouts' current search has no club (`SCOUT_MANAGER::current_search_has_no_club`,
///   0x7f8420), the club is not in the session's searches (0x7f8500) and is not a national team;
/// - for a club the human does not run, from the Friendly object ([G_FRIENDLY]): the open invitation (0x5c68b0) whose
///   club +0xc is a national team as much as this club: "Invite Club" (0x66, the invitation); the open dates request
///   (0x5c6920) of the club's nation (+4) whose club +0 matches likewise: "Invite Club" (0x67); the tournament
///   (0x5c6830) whose club +0 matches likewise: "Uninvite Club" (0x68) when the club is among its invited (+0x13
///   count, +0x15 the clubs; the index a signed byte as the original), otherwise "Invite Club" (0x69);
/// - "Apply for Job" (0x65) for a club without a manager (`exe::database_get_club_manager`) that can be managed.
/// Returns 1 when a button was added, otherwise 0.
/// Original: `add_club_screen_action_button` (0x482a40, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn add_club_screen_action_button(club: usize, colour: u32, colour_text: u32) -> u32 {
    let club = Club(club);
    if exe::is_current_manager_being_created() != 0 {
        if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 || exe::club_can_be_managed(club) == 0 {
            return 0;
        }
        club_action_button(colour, colour_text, 0x991594, 0x64, 0); // "Take Control"
        return 1;
    }
    let scouts = read_usize(G_SCOUT_MANAGER);
    if exe::scout_manager_current_search_has_no_club(scouts) != 0 && exe::scout_manager_is_club_in_session_searches(scouts, club) == 0 && exe::database_is_national_team(club) == 0 {
        club_action_button(colour, colour_text, 0x991588, 0x6a, 0); // "Scout Club"
        return 1;
    }
    if exe::human_manager_current_controls_club(club, 0, 1) == 0 {
        let find = |f: usize| {
            let g: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(f);
            g(read_usize(G_FRIENDLY))
        };
        let national = |c: usize| exe::database_is_national_team(c) != 0;
        const INVITE: usize = 0x99157c; // "Invite Club"
        let m = find(0x5c68b0);
        if m != 0 && national(read_usize(m + 0xc)) == national(club.addr()) {
            club_action_button(colour, colour_text, INVITE, 0x66, m);
            return 1;
        }
        let r = find(0x5c6920);
        if r != 0 && (club.nation().addr() as u32) == read_u32(r + 4) && national(club.addr()) == national(read_usize(r)) {
            club_action_button(colour, colour_text, INVITE, 0x67, r);
            return 1;
        }
        let t = find(0x5c6830);
        if t != 0 && national(club.addr()) == national(read_usize(t)) {
            covered!("tel06_3 tournament button");
            let n = read_i16(t + 0x13);
            let mut invited = false;
            let mut k = 0i8;
            while n > 0 && !invited {
                if read_usize((t as isize + 0x15 + 4 * k as isize) as usize) == club.addr() {
                    invited = true;
                }
                k = k.wrapping_add(1);
                if k as i16 >= n {
                    break;
                }
            }
            if invited {
                club_action_button(colour, colour_text, 0x99156c, 0x68, t); // "Uninvite Club"
            } else {
                club_action_button(colour, colour_text, INVITE, 0x69, t);
            }
            return 1;
        }
    }
    if exe::database_get_club_manager(club) == 0 && exe::club_can_be_managed(club) != 0 {
        club_action_button(colour, colour_text, 0x99155c, 0x65, 0); // "Apply for Job"
        return 1;
    }
    0
}

/// Comparer for `add_club_screen_action_button`: side by side over the display list (`compare_match_screen_build`)
/// and the return.
pub(crate) unsafe extern "C" fn compare_add_club_screen_action_button(club: usize, colour: u32, colour_text: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32) -> u32>(IDX_ADD_CLUB_SCREEN_ACTION_BUTTON);
    let (mut a, mut b) = (0u32, 0u32);
    compare_match_screen_build("club screen action button", &[], || a = o(club, colour, colour_text), || b = add_club_screen_action_button(club, colour, colour_text));
    SCOREBOARD.record(a == b, || format!("add_club_screen_action_button (club screen) returned {} and Rust {}", a, b));
    a
}

/// Squad list order "selection": the players by their slot in the team selection, the unselected last.
///
/// A null first entry sorts last (1), a null second first (-1). When the human controls the list's club
/// (`human_manager_current_controls_club(club, 0, 1)`), each player's slot is `Tactics::find_selection_slot(club,
/// player, PLAYER_SORT_RESERVES)`; otherwise, with no team sheet (`PLAYER_SORT_TEAM_SHEET` 0) the result is 1, and
/// with one the slot is the last of its 20 entries (0x49 bytes apart, from +0x190 of the sheet's first or second half:
/// the second, 0x625 bytes on, when the club's id matches the sheet's first dword) holding the player's id. An
/// unselected first player (0xff) sorts last (1), then an unselected second one first (-1); different slots give
/// the signed byte difference first - second; equal slots go to the names (`player_sort_by_name`).
/// Original: `compare_players_by_selection_slot_then_name` (0x485470, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_selection_slot_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let club = read_usize(PLAYER_SORT_CLUB);
    let (mut sa, mut sb) = (0xffu8, 0xffu8);
    if exe::human_manager_current_controls_club(club, 0, 1) != 0 {
        covered!("tel30_5 selection slot of the human's club");
        let reserves = read_u32(PLAYER_SORT_RESERVES);
        sa = exe::tactics_find_selection_slot(read_usize(G_TACTICS), club, a, reserves);
        sb = exe::tactics_find_selection_slot(read_usize(G_TACTICS), club, b, reserves);
    } else {
        let sheet = read_usize(PLAYER_SORT_TEAM_SHEET);
        if sheet == 0 {
            return 1;
        }
        let (ida, idb) = (read_u32(a), read_u32(b));
        let half = (read_u32(club) == read_u32(sheet)) as usize;
        let mut entry = sheet + half * 0x625 + 0x190;
        for k in 0u8..0x14 {
            let id = read_u32(entry);
            if id == ida {
                sa = k;
            }
            if id == idb {
                sb = k;
            }
            entry += 0x49;
        }
    }
    if sa == 0xff {
        return 1;
    }
    if sb == 0xff {
        return -1;
    }
    let d = sa.wrapping_sub(sb) as i8;
    if d != 0 {
        return d as i32;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "squad slot": the players by the human manager's squad number index
/// (`HUMAN_MANAGER::get_player_squad_number(player, 0)`), the unnumbered (0xff) last, no name tie-break.
///
/// A null first entry sorts last (1), a null second first (-1). An unnumbered first player gives 1 when the second
/// has a number and 0 when neither has; an unnumbered second one gives -1; otherwise first - second (signed bytes).
/// Original: `compare_players_by_human_squad_slot` (0x485630, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_human_squad_slot(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let na = exe::human_manager_get_player_squad_number(read_usize(G_HUMAN_MANAGER), a, 0);
    let nb = exe::human_manager_get_player_squad_number(read_usize(G_HUMAN_MANAGER), b, 0);
    if na == 0xff {
        return (nb != 0xff) as i32;
    }
    if nb == 0xff {
        return -1;
    }
    na as i8 as i32 - nb as i8 as i32
}

/// Squad list order "status": the players by their status code (`get_player_status_code(player, club, fixture)`,
/// 0x4849f0), then by name.
///
/// A null first entry sorts last (1), a null second first (-1); then the signed byte difference of the codes
/// first - second; equal codes go to the names.
/// Original: `compare_players_by_status_code_then_name` (0x4856a0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_status_code_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let (club, fixture) = (read_usize(PLAYER_SORT_CLUB), read_usize(PLAYER_SORT_FIXTURE));
    let sb = exe::get_player_status_code(b, club, fixture);
    let sa = exe::get_player_status_code(a, club, fixture);
    let d = sa.wrapping_sub(sb) as i8;
    if d != 0 {
        return d as i32;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "name": the players by name (`player_sort_by_name`).
/// Original: `compare_players_by_name` (0x4857a0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "position": the players by their positions text, then by name.
///
/// A null first entry (or one with no player record at `staff::PLAYER`) sorts last (1), a null second one first
/// (-1); the positions are written by `database_format_player_positions(player, buffer, 200, 4)` and compared with
/// `utils_string_compare(first, second, 1, 0)`; equal texts go to the names.
/// Original: `compare_players_by_positions_then_name` (0x485830, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_positions_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let (ra, rb) = (read_usize(a + staff::PLAYER), read_usize(b + staff::PLAYER));
    if ra == 0 {
        return 1;
    }
    if rb == 0 {
        return -1;
    }
    let (mut x, mut y) = ([0u8; 200], [0u8; 200]);
    exe::database_format_player_positions(ra, (x.as_mut_ptr()) as *mut u8 as usize, 200, 4);
    exe::database_format_player_positions(rb, (y.as_mut_ptr()) as *mut u8 as usize, 200, 4);
    let r = exe::utils_string_compare(x.as_ptr(), y.as_ptr(), 1, 0);
    if r != 0 {
        return r;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "club": the players by their club's name (the text at +0x38 of the `staff::CLUB_JOB` club),
/// then by name.
///
/// A null first entry (or one without a club) sorts last (1), a null second one first (-1); the club names are
/// compared with `utils_string_compare(first, second, 1, 0)`; equal names go to the players' names.
/// Original: `compare_players_by_club_name_then_name` (0x485960, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_club_name_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let (ca, cb) = (read_usize(a + staff::CLUB_JOB), read_usize(b + staff::CLUB_JOB));
    if ca == 0 {
        return 1;
    }
    if cb == 0 {
        return -1;
    }
    let r = exe::utils_string_compare((ca + 0x38) as *const u8, (cb + 0x38) as *const u8, 1, 0);
    if r != 0 {
        return r;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "squad status": the players by the squad status of their contract with the list's side, then
/// by name.
///
/// A null first entry sorts last (1), a null second one first (-1). Each player's contract with
/// `PLAYER_SORT_SIDE` comes from `CONTRACT_MANAGER::get_contract_for_club`; when the human controls the side
/// (`human_manager_current_controls_club(side, 0, 1)`) the status is the high nibble of the contract's byte +0x4f
/// (100 without a contract), otherwise both are 100. The signed difference first - second decides; equal statuses
/// go to the names.
/// Original: `compare_players_by_squad_status_then_name` (0x485a60, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_squad_status_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let side = read_usize(PLAYER_SORT_SIDE);
    let ca = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), a, side);
    let cb = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), b, side);
    let (mut sa, mut sb) = (100u8, 100u8);
    if exe::human_manager_current_controls_club(side, 0, 1) != 0 {
        if ca != 0 {
            sa = read_u8(ca + 0x4f) >> 4;
        }
        if cb != 0 {
            sb = read_u8(cb + 0x4f) >> 4;
        }
    }
    let d = sa as i8 as i32 - sb as i8 as i32;
    if d != 0 {
        return d;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "squad number": the players by the absolute value of their squad number at the list's side,
/// then by name.
///
/// A null first entry (or one with no player record) sorts last (1), a null second one first (-1); then a first
/// player with squad number 0 (`SQUAD_MANAGER::get_squad_number(player, PLAYER_SORT_SIDE)`) sorts last (1) and a
/// second one first (-1); otherwise the byte difference |first| - |second| (signed) decides, and equal numbers go
/// to the names.
/// Original: `compare_players_by_squad_number_then_name` (0x485ba0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_squad_number_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    if read_usize(a + staff::PLAYER) == 0 {
        return 1;
    }
    if read_usize(b + staff::PLAYER) == 0 {
        return -1;
    }
    let number = |p: usize| exe::squad_manager_get_squad_number(read_usize(PLAYER_SORT_SQUAD_MANAGER), p, read_usize(PLAYER_SORT_SIDE));
    if number(a) == 0 {
        return 1;
    }
    if number(b) == 0 {
        return -1;
    }
    let nb = (number(b) as i32).abs();
    let na = (number(a) as i32).abs();
    let d = (na as u8).wrapping_sub(nb as u8) as i8;
    if d != 0 {
        return d as i32;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "release clause": the players by the code of their release clause, then by its abbreviation,
/// then by name.
///
/// A null first entry sorts last (1), a null second one first (-1). Each player's buffer starts as "  -" (the four
/// bytes at 0x98e8c8) and 26 zero bytes, and the code as -1; a contract (`CONTRACT_MANAGER::get_contract`) replaces
/// both with `STAFF_CONTRACT::get_release_clause_abbreviation(buffer, 30)`. The signed byte difference
/// second - first decides; then the two texts byte by byte (1 when the first is greater, -1 when smaller, the low
/// byte of the result); equal texts go to the names.
/// Original: `compare_players_by_release_clause_then_name` (0x485d20, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_release_clause_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    let (mut ta, mut tb) = ([0u8; 30], [0u8; 30]);
    let dash = read_u32(0x98e8c8).to_le_bytes();
    ta[..4].copy_from_slice(&dash);
    tb[..4].copy_from_slice(&dash);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let ca = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), a);
    let cb = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), b);
    let (mut ka, mut kb) = (0xffu8, 0xffu8);
    if ca != 0 {
        ka = exe::staff_contract_get_release_clause_abbreviation(ca, (ta.as_mut_ptr()) as *mut u8 as usize, 0x1e);
    }
    if cb != 0 {
        kb = exe::staff_contract_get_release_clause_abbreviation(cb, (tb.as_mut_ptr()) as *mut u8 as usize, 0x1e);
    }
    let d = kb.wrapping_sub(ka) as i8;
    if d != 0 {
        return d as i32;
    }
    let mut k = 0;
    let order = loop {
        let (x, y) = (ta[k], tb[k]);
        if x != y {
            break if x < y { -1i8 } else { 1 };
        }
        if x == 0 || k + 1 >= ta.len() {
            break 0;
        }
        k += 1;
    };
    if order != 0 {
        return order as i32;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "European registration": the players by the rank of their registration byte
/// (`staff::SQUAD_SELECTED_FOR`: 1, 2, 4, 0, then the others), then by name.
///
/// A null first entry sorts last (1), a null second one first (-1); then the signed byte difference of the ranks
/// second - first (1 → 4, 2 → 3, 4 → 2, 0 → 1, others 0); equal ranks go to the names.
/// Original: `compare_players_by_euro_registration_then_name` (0x485ed0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_euro_registration_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let rb = euro_registration_rank(read_u8(b + staff::SQUAD_SELECTED_FOR));
    let ra = euro_registration_rank(read_u8(a + staff::SQUAD_SELECTED_FOR));
    let d = rb.wrapping_sub(ra) as i8;
    if d != 0 {
        return d as i32;
    }
    player_sort_by_name(a, b)
}

/// Squad list order "form": the players by their recent form (`PlayerStats::get_recent_form`), best first, then by
/// name.
///
/// A null first entry, or one without a player record or with an id not below `G_STAFF_COUNT`, sorts last (1); the
/// same for the second gives -1. Then the signed difference second - first of the forms; equal forms go to the names.
/// Original: `compare_players_by_form_then_name` (0x485fd0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_form_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let (ra, rb) = (read_usize(a + staff::PLAYER), read_usize(b + staff::PLAYER));
    let count = read_i32(G_STAFF_COUNT);
    if ra == 0 || read_i32(a) >= count {
        return 1;
    }
    if rb == 0 || read_i32(b) >= count {
        return -1;
    }
    let fb = exe::playerstats_get_recent_form(read_usize(G_PLAYER_STATS), rb) as i32;
    let fa = exe::playerstats_get_recent_form(read_usize(G_PLAYER_STATS), ra) as i32;
    if fb - fa != 0 {
        return fb - fa;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Comparer for `compare_players_by_selection_slot_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_selection_slot_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("selection", original_fn(IDX_COMPARE_PLAYERS_BY_SELECTION_SLOT_THEN_NAME), compare_players_by_selection_slot_then_name, a, b)
}

/// Comparer for `compare_players_by_human_squad_slot`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_human_squad_slot(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("human squad slot", original_fn(IDX_COMPARE_PLAYERS_BY_HUMAN_SQUAD_SLOT), compare_players_by_human_squad_slot, a, b)
}

/// Comparer for `compare_players_by_status_code_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_status_code_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("status", original_fn(IDX_COMPARE_PLAYERS_BY_STATUS_CODE_THEN_NAME), compare_players_by_status_code_then_name, a, b)
}

/// Comparer for `compare_players_by_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("name", original_fn(IDX_COMPARE_PLAYERS_BY_NAME), compare_players_by_name, a, b)
}

/// Comparer for `compare_players_by_positions_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_positions_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("positions", original_fn(IDX_COMPARE_PLAYERS_BY_POSITIONS_THEN_NAME), compare_players_by_positions_then_name, a, b)
}

/// Comparer for `compare_players_by_club_name_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_club_name_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("club", original_fn(IDX_COMPARE_PLAYERS_BY_CLUB_NAME_THEN_NAME), compare_players_by_club_name_then_name, a, b)
}

/// Comparer for `compare_players_by_squad_status_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_squad_status_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("squad status", original_fn(IDX_COMPARE_PLAYERS_BY_SQUAD_STATUS_THEN_NAME), compare_players_by_squad_status_then_name, a, b)
}

/// Comparer for `compare_players_by_squad_number_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_squad_number_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("squad number", original_fn(IDX_COMPARE_PLAYERS_BY_SQUAD_NUMBER_THEN_NAME), compare_players_by_squad_number_then_name, a, b)
}

/// Comparer for `compare_players_by_release_clause_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_release_clause_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("release clause", original_fn(IDX_COMPARE_PLAYERS_BY_RELEASE_CLAUSE_THEN_NAME), compare_players_by_release_clause_then_name, a, b)
}

/// Comparer for `compare_players_by_euro_registration_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_euro_registration_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("European registration", original_fn(IDX_COMPARE_PLAYERS_BY_EURO_REGISTRATION_THEN_NAME), compare_players_by_euro_registration_then_name, a, b)
}

/// Comparer for `compare_players_by_form_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_form_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("form", original_fn(IDX_COMPARE_PLAYERS_BY_FORM_THEN_NAME), compare_players_by_form_then_name, a, b)
}

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
    let morale: unsafe extern "thiscall" fn(usize, usize) -> i8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_PLAYER_MORALE);
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
    player_sort_compare("morale", original_fn(IDX_COMPARE_PLAYERS_BY_MORALE_THEN_NAME), compare_players_by_morale_then_name, a, b)
}

/// Comparer for `compare_players_by_staff_state_byte_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_staff_state_byte_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("state", original_fn(IDX_COMPARE_PLAYERS_BY_STAFF_STATE_BYTE_THEN_NAME), compare_players_by_staff_state_byte_then_name, a, b)
}

/// Comparer for `compare_players_by_fatigue_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_fatigue_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("fatigue", original_fn(IDX_COMPARE_PLAYERS_BY_FATIGUE_THEN_NAME), compare_players_by_fatigue_then_name, a, b)
}

/// Comparer for `compare_players_by_injury_recovery_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_injury_recovery_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("injury", original_fn(IDX_COMPARE_PLAYERS_BY_INJURY_RECOVERY_THEN_NAME), compare_players_by_injury_recovery_then_name, a, b)
}

/// Comparer for `compare_players_by_selected_stat_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_selected_stat_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("statistic", original_fn(IDX_COMPARE_PLAYERS_BY_SELECTED_STAT_THEN_NAME), compare_players_by_selected_stat_then_name, a, b)
}

/// Comparer for `compare_players_by_selected_attribute_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_selected_attribute_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("attribute", original_fn(IDX_COMPARE_PLAYERS_BY_SELECTED_ATTRIBUTE_THEN_NAME), compare_players_by_selected_attribute_then_name, a, b)
}

/// Comparer for `compare_players_by_nationality_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_nationality_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("nationality", original_fn(IDX_COMPARE_PLAYERS_BY_NATIONALITY_THEN_NAME), compare_players_by_nationality_then_name, a, b)
}

/// Comparer for `compare_players_by_age_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_age_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("age", original_fn(IDX_COMPARE_PLAYERS_BY_AGE_THEN_NAME), compare_players_by_age_then_name, a, b)
}

/// Comparer for `compare_players_by_caps_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_caps_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("caps", original_fn(IDX_COMPARE_PLAYERS_BY_CAPS_THEN_NAME), compare_players_by_caps_then_name, a, b)
}

/// Comparer for `compare_players_by_international_goals_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_international_goals_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("international goals", original_fn(IDX_COMPARE_PLAYERS_BY_INTERNATIONAL_GOALS_THEN_NAME), compare_players_by_international_goals_then_name, a, b)
}

/// Comparer for `compare_players_by_wage_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_wage_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("wage", original_fn(IDX_COMPARE_PLAYERS_BY_WAGE_THEN_NAME), compare_players_by_wage_then_name, a, b)
}

/// Squad list order "contract expiry": the players by the contract the list shows (`player_sort_list_contract`):
/// expired, then rolling, then by the days to its end, then by name.
///
/// A null first entry (or one without that contract) sorts last (1), a null second one first (-1). An expired
/// second contract (`STAFF_CONTRACT::is_expired`) gives 1, an expired first -1; then a rolling second
/// (`STAFF_CONTRACT::is_rolling`) 1, a rolling first -1; then the days to the end (`days_to_end_date`, signed words)
/// first - second; equal days go to the names (`player_sort_by_name`).
/// Original: `compare_players_by_contract_expiry_then_name` (0x486c70, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_contract_expiry_then_name(pa: *const usize, pb: *const usize) -> i32 {
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
    if exe::staff_contract_is_expired(cb) != 0 {
        return 1;
    }
    if exe::staff_contract_is_expired(ca) != 0 {
        return -1;
    }
    if exe::staff_contract_is_rolling(cb) != 0 {
        return 1;
    }
    if exe::staff_contract_is_rolling(ca) != 0 {
        return -1;
    }
    let db = exe::staff_contract_days_to_end_date(cb) as i32;
    let da = exe::staff_contract_days_to_end_date(ca) as i32;
    if da - db != 0 {
        return da - db;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "FIFA protected period": the players by the byte their contract returns from
/// `STAFF_CONTRACT::check_fifa_protected_period(0, 0, 0)` (0 without a contract), then by name.
///
/// A null first entry sorts last (1), a null second one first (-1); then first - second (signed bytes); equal
/// values go to the names.
/// Original: `compare_players_by_fifa_protected_period_then_name` (0x486e60, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_fifa_protected_period_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let period = |p: usize| {
        let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
        if c != 0 { exe::staff_contract_check_fifa_protected_period(c, 0usize, 0usize, 0usize) as i8 as i32 } else { 0 }
    };
    let va = period(a);
    let d = va - period(b);
    if d != 0 {
        return d;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "value": the players by `staff::VALUE`, highest first, then by name.
///
/// A null first entry sorts last (1), a null second one first (-1); then second - first; equal values go to the
/// names.
/// Original: `compare_players_by_value_then_name` (0x486f80, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_value_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let d = read_i32(b + staff::VALUE).wrapping_sub(read_i32(a + staff::VALUE));
    if d != 0 {
        return d;
    }
    player_sort_by_name(a, b)
}

/// Squad list order "appearances": the players by the season statistic 1 (`player_sort_by_season_stat`), most
/// first, then by name; a null first entry sorts last (1), a null second one first (-1).
/// Original: `compare_players_by_appearances_then_name` (0x487030, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_appearances_then_name(pa: *const usize, pb: *const usize) -> i32 {
    player_sort_by_season_stat(pa, pb, 1)
}

/// Squad list order "goals": the players by the season statistic 2, most first, then by name.
/// Original: `compare_players_by_goals_then_name` (0x487140, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_goals_then_name(pa: *const usize, pb: *const usize) -> i32 {
    player_sort_by_season_stat(pa, pb, 2)
}

/// Squad list order "goals conceded": the players by the season statistic 3, most first, then by name.
/// Original: `compare_players_by_goals_conceded_then_name` (0x487250, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_goals_conceded_then_name(pa: *const usize, pb: *const usize) -> i32 {
    player_sort_by_season_stat(pa, pb, 3)
}

/// Squad list order "assists": the players by the season statistic 5, most first, then by name.
/// Original: `compare_players_by_assists_then_name` (0x487360, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_assists_then_name(pa: *const usize, pb: *const usize) -> i32 {
    player_sort_by_season_stat(pa, pb, 5)
}

/// Squad list order "average rating": the players by `PlayerStats::get_stat(player record, 6, 0x11, 6.5)`, highest
/// first (`player_sort_by_float_stat`: the first rounded to a float, a greater second gives 1, a smaller -1), then
/// by name; a null first entry sorts last (1), a null second one first (-1).
/// Original: `compare_players_by_average_rating_then_name` (0x487470, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_average_rating_then_name(pa: *const usize, pb: *const usize) -> i32 {
    player_sort_by_float_stat(pa, pb, 6, 0x11)
}

/// Staff list order "club job": the staff by `staff::JOB_FOR_CLUB` (12 counting as 5), first - second, then by name;
/// a null first entry sorts last (1), a null second one first (-1).
/// Original: `compare_staff_by_club_job_then_name` (0x4875a0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_staff_by_club_job_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let d = staff_sort_job(read_i8(a + staff::JOB_FOR_CLUB)) - staff_sort_job(read_i8(b + staff::JOB_FOR_CLUB));
    if d != 0 {
        return d;
    }
    player_sort_by_name(a, b)
}

/// Staff list order "nation job": the staff by `staff::JOB_FOR_NATION` (12 counting as 5), first - second, then by
/// name; a null first entry sorts last (1), a null second one first (-1).
/// Original: `compare_staff_by_nation_job_then_name` (0x487660, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_staff_by_nation_job_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let d = staff_sort_job(read_i8(a + staff::JOB_FOR_NATION)) - staff_sort_job(read_i8(b + staff::JOB_FOR_NATION));
    if d != 0 {
        return d;
    }
    player_sort_by_name(a, b)
}

/// Attribute list order: two attribute codes (bytes) by their long names.
///
/// A null first pointer sorts last (1), a null second one first (-1); a first code outside 1..=31 gives 1, then a
/// second one outside it -1; otherwise both names (`database_get_player_attribute_name(code, buffer, 200, 1)`) are
/// compared with `utils_string_compare(first, second, 1, 0)`.
/// Original: `compare_attributes_by_name` (0x487720, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_attributes_by_name(pa: *const usize, pb: *const usize) -> i32 {
    let (pa, pb) = (pa as usize, pb as usize);
    if pa == 0 {
        return 1;
    }
    if pb == 0 {
        return -1;
    }
    let (ka, kb) = (read_i8(pa), read_i8(pb));
    if !(1..=0x1f).contains(&ka) {
        return 1;
    }
    if !(1..=0x1f).contains(&kb) {
        return -1;
    }
    let (mut x, mut y) = ([0u8; 200], [0u8; 200]);
    exe::database_get_player_attribute_name(ka as u8 as u32, (x.as_mut_ptr()) as *mut u8 as usize, 200, 1);
    exe::database_get_player_attribute_name(kb as u8 as u32, (y.as_mut_ptr()) as *mut u8 as usize, 200, 1);
    exe::utils_string_compare(x.as_ptr(), y.as_ptr(), 1, 0)
}

/// Date list order: ascending. A null first pointer sorts last (1), a null second one first (-1); then -1 when the
/// first date is before the second (`Date::operator<`), 1 when after (`Date::operator>`), 0 when equal.
/// Original: `compare_dates_ascending` (0x4877c0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_dates_ascending(pa: *const usize, pb: *const usize) -> i32 {
    let (pa, pb) = (pa as usize, pb as usize);
    if pa == 0 {
        return 1;
    }
    if pb == 0 {
        return -1;
    }
    if exe::date_operator_3c(pa, read_u32(pb), read_u32(pb + 4)) != 0 {
        return -1;
    }
    (exe::date_operator_3e(pa, read_u32(pb), read_u32(pb + 4)) != 0) as i32
}

/// Comparer for `compare_players_by_contract_expiry_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_contract_expiry_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("contract expiry", original_fn(IDX_COMPARE_PLAYERS_BY_CONTRACT_EXPIRY_THEN_NAME), compare_players_by_contract_expiry_then_name, a, b)
}

/// Comparer for `compare_players_by_fifa_protected_period_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_fifa_protected_period_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("FIFA protected period", original_fn(IDX_COMPARE_PLAYERS_BY_FIFA_PROTECTED_PERIOD_THEN_NAME), compare_players_by_fifa_protected_period_then_name, a, b)
}

/// Comparer for `compare_players_by_value_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_value_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("value", original_fn(IDX_COMPARE_PLAYERS_BY_VALUE_THEN_NAME), compare_players_by_value_then_name, a, b)
}

/// Comparer for `compare_players_by_appearances_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_appearances_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("appearances", original_fn(IDX_COMPARE_PLAYERS_BY_APPEARANCES_THEN_NAME), compare_players_by_appearances_then_name, a, b)
}

/// Comparer for `compare_players_by_goals_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_goals_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("goals", original_fn(IDX_COMPARE_PLAYERS_BY_GOALS_THEN_NAME), compare_players_by_goals_then_name, a, b)
}

/// Comparer for `compare_players_by_goals_conceded_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_goals_conceded_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("goals conceded", original_fn(IDX_COMPARE_PLAYERS_BY_GOALS_CONCEDED_THEN_NAME), compare_players_by_goals_conceded_then_name, a, b)
}

/// Comparer for `compare_players_by_assists_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_assists_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("assists", original_fn(IDX_COMPARE_PLAYERS_BY_ASSISTS_THEN_NAME), compare_players_by_assists_then_name, a, b)
}

/// Comparer for `compare_players_by_average_rating_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_average_rating_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("average rating", original_fn(IDX_COMPARE_PLAYERS_BY_AVERAGE_RATING_THEN_NAME), compare_players_by_average_rating_then_name, a, b)
}

/// Comparer for `compare_staff_by_club_job_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_staff_by_club_job_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("staff club job", original_fn(IDX_COMPARE_STAFF_BY_CLUB_JOB_THEN_NAME), compare_staff_by_club_job_then_name, a, b)
}

/// Comparer for `compare_staff_by_nation_job_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_staff_by_nation_job_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("staff nation job", original_fn(IDX_COMPARE_STAFF_BY_NATION_JOB_THEN_NAME), compare_staff_by_nation_job_then_name, a, b)
}

/// Comparer for `compare_attributes_by_name`: the order side by side (the attribute codes, not staff pointers).
pub(crate) unsafe extern "C" fn compare_compare_attributes_by_name(a: *const usize, b: *const usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(*const usize, *const usize) -> i32>(IDX_COMPARE_ATTRIBUTES_BY_NAME);
    let r = o(a, b);
    let v = compare_attributes_by_name(a, b);
    SCOREBOARD.record(r == v, || format!("attribute list sort by name: original={} rust={}", r, v));
    r
}

/// Comparer for `compare_dates_ascending`: the order side by side (dates, not staff pointers).
pub(crate) unsafe extern "C" fn compare_compare_dates_ascending(a: *const usize, b: *const usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(*const usize, *const usize) -> i32>(IDX_COMPARE_DATES_ASCENDING);
    let r = o(a, b);
    let v = compare_dates_ascending(a, b);
    SCOREBOARD.record(r == v, || format!("date list sort ascending: original={} rust={}", r, v));
    r
}

/// Builds the last match's team sheet record (`match_team_pair_construct`) and registers its destructor
/// (`club_screens_exit_match_record`) with `atexit`.
/// Original: `club_screens_init_match_record` (0x453b90, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screens_init_match_record() {
    match_team_pair_construct(CLUB_SCREENS_MATCH_RECORD);
    exe::crt_atexit(0x453bb0usize);
}

/// Destroys the last match's team sheet record (`match_team_pair_destructor`).
/// Original: `club_screens_exit_match_record` (0x453bb0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screens_exit_match_record() {
    exe::match_team_pair_destructor(CLUB_SCREENS_MATCH_RECORD);
}

/// Builds a team sheet record: its two 0x625-byte team blocks from +0x11f (`eh vector constructor iterator` with
/// `match_team_block_construct` and `match_team_block_destruct`); returns the record.
/// Original: `match_team_pair_construct` (0x453be0, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn match_team_pair_construct(this: usize) -> usize {
    exe::crt_vector_constructor_iterator(this + 0x11f, 0x625, 2, 0x453c30usize, 0x453c10usize);
    this
}

/// Destroys a team block: its 20 rating records of 0x49 bytes from +0x71 (`eh vector destructor iterator` with the
/// empty 0x5a53c0).
/// Original: `match_team_block_destruct` (0x453c10, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn match_team_block_destruct(this: usize) {
    exe::crt_vector_destructor_iterator(this + 0x71, 0x49, 0x14, 0x5a53c0usize);
}

/// Builds a team block: its 20 rating records of 0x49 bytes from +0x71 (`rating_record_construct` 0x453c50, the
/// empty 0x5a53c0 as destructor); returns the block.
/// Original: `match_team_block_construct` (0x453c30, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn match_team_block_construct(this: usize) -> usize {
    exe::crt_vector_constructor_iterator(this + 0x71, 0x49, 0x14, 0x453c50usize, 0x5a53c0usize);
    this
}

/// Builds the 16 club history states (0x2a bytes each, `club_history_state_construct` and
/// `club_history_state_destruct`) and registers their destruction (`club_screens_exit_history_states`) with `atexit`.
/// Original: `club_screens_init_history_states` (0x453c60, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screens_init_history_states() {
    exe::crt_vector_constructor_iterator(CLUB_SCREENS_HISTORY_STATES, 0x2a, 0x10, 0x453c90usize, 0x453cf0usize);
    exe::crt_atexit(0x453d40usize);
}

/// Builds a club history state: its date (`Date::Date`) and its list at +0x18 (`LLIST::LLIST`), then all its 0x2a
/// bytes cleared; returns the state.
/// Original: `club_history_state_construct` (0x453c90, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn club_history_state_construct(this: usize) -> usize {
    exe::date_date_54a110(this);
    exe::new_llist(this + 0x18);
    core::ptr::write_bytes(this as *mut u8, 0, 0x2a);
    this
}

/// Destroys a club history state: the object at +0x26, when there is one, through its scalar deleting destructor
/// (slot 0 of its vtable, with 1), then the list at +0x18 (`LLIST::~LLIST`).
/// Original: `club_history_state_destruct` (0x453cf0, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn club_history_state_destruct(this: usize) {
    let object = read_usize(this + 0x26);
    if object != 0 {
        let destroy: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(read_usize(read_usize(object)));
        destroy(object, 1);
    }
    exe::drop_llist(this + 0x18);
}

/// Destroys the 16 club history states (`eh vector destructor iterator` with `club_history_state_destruct`).
/// Original: `club_screens_exit_history_states` (0x453d40, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screens_exit_history_states() {
    exe::crt_vector_destructor_iterator(CLUB_SCREENS_HISTORY_STATES, 0x2a, 0x10, 0x453cf0usize);
}

/// Opens the club screen at its squad tab, with the squad view and the club's three best players.
///
/// A club without a reserve squad (`database_club_has_reserve_squad`) has `reserve` cleared. With `restore`, the 60
/// parameters of the current screen are saved first. The view settings are the human's
/// (`HUMAN_MANAGER::get_squad_view_settings(0)`) when he controls the club, otherwise the defaults. The screen opens
/// (`ScreenManager::open_screen` with the club screen's build, input and leave functions, not single instance); when
/// it does: 0 = club, 1 = reserve, 2 = 3 = 0, 4 = 5 with `tab5` (otherwise 1), 5 = 6 = 0, 7 / 8 = the view's first
/// two dwords, 9 = its byte +0xc (signed), 0xa = 0xb = 0, 0xc = the year, 0xd = 0xe = 0, 0xf = 1, 0x10 = 0x11 = 0,
/// 0x12 = 1, 0x13..0x16 = 0, 0x17 = the year, 0x18 = the view's third dword, 0x19 / 0x1a = its bytes +0xd / +0xe
/// (signed), 0x1b = 2 with `tab5` (otherwise 1), 0x1c..0x1f = 0; with `restore` the saved 4, 7, 8, 9, 0x10, 0x11,
/// 0x12, 0x18, 0x19, 0x1a and 0x1b come back; with `highlight` parameter 7 gets bit 0x800. Parameters 0x20, 0x21 and
/// 0x22 are the ids of the club's three best players (`club_squad_stars`), -1 for each missing.
/// Original: `open_club_squad_screen` (0x453d60, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_club_squad_screen(club: usize, reserve: u32, restore: u32, tab5: u32, highlight: u32) {
    let mut reserve = reserve;
    if club != 0 && exe::database_club_has_reserve_squad(club) == 0 {
        reserve = 0;
    }
    let mut saved = [0usize; 0x3c];
    if restore != 0 {
        for (k, v) in saved.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
    }
    let default_view = CLUB_SQUAD_DEFAULT_VIEW;
    let view = if club != 0 && exe::human_manager_current_controls_club(club, 0, 1) != 0 {
        exe::human_manager_get_squad_view_settings(read_usize(G_HUMAN_MANAGER), 0)
    } else {
        default_view.as_ptr() as usize
    };
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, FN_CLUB_SCREEN_BUILD, FN_CLUB_SCREEN_INPUT, 0, FN_CLUB_SCREEN_LEAVE, 0) == 0 {
        return;
    }
    covered!("tel30_8 club squad screen opened");
    let year = read_u16(G_CURRENT_DATE_YEAR) as usize;
    let tab = |a: usize, b: usize| if tab5 != 0 { a } else { b };
    let params: [(u32, usize); 32] = [
        (0, club), (1, reserve as usize), (2, 0), (3, 0), (4, tab(5, 1)), (5, 0), (6, 0), (7, read_usize(view)),
        (8, read_usize(view + 4)), (9, read_i8(view + 0xc) as i32 as usize), (0xa, 0), (0xb, 0), (0xc, year), (0xd, 0),
        (0xe, 0), (0xf, 1), (0x10, 0), (0x11, 0), (0x12, 1), (0x13, 0), (0x14, 0), (0x15, 0), (0x16, 0), (0x17, year),
        (0x18, read_usize(view + 8)), (0x19, read_i8(view + 0xd) as i32 as usize), (0x1a, read_i8(view + 0xe) as i32 as usize),
        (0x1b, tab(2, 1)), (0x1c, 0), (0x1d, 0), (0x1e, 0), (0x1f, 0),
    ];
    for (k, v) in params {
        set_screen_param(k, v);
    }
    if restore != 0 {
        for k in [4u32, 7, 8, 9, 0x10, 0x11, 0x12, 0x18, 0x19, 0x1a, 0x1b] {
            set_screen_param(k, saved[k as usize]);
        }
    }
    if highlight != 0 {
        set_screen_param(7, read_usize(view) | 0x800);
    }
    let (count, best) = if club != 0 { club_squad_stars(club) } else { (0, [0; 3]) };
    let id = |k: usize, need: i8| if count >= need { read_usize(best[k]) } else { usize::MAX };
    set_screen_param(0x20, id(0, 1));
    set_screen_param(0x21, id(1, 2));
    set_screen_param(0x22, id(2, 3));
}

/// A menu item: the id -1 and the value 0 (two words); returns the item.
/// Original: `MenuItem::MenuItem` (0x4568f0, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn menu_item_menu_item(this: usize) -> usize {
    write_u16(this, 0xffff);
    write_u16(this + 2, 0);
    this
}

/// Gathers the squad statistics of a club's screen into `out` (`club_squad_stats`).
///
/// The side is the club's linked senior side when `club_get_linked_b_team` reports it as a B team, else the club; the
/// squad is `database_collect_club_squad(club, list, 1)`. Every field is cleared but `SELECTED_AVERAGE_WAGE`. Each
/// of the first 50 players with an id below `G_STAFF_COUNT` (for a national team, without the squad flag 8, or 4 with
/// `reserve`): counted; injured (status 2) counted; his age (`staff::AGE`, signed) summed in 16 bits, and also for
/// the selected (`Tactics::find_selection_slot(side, player, 0)` ≠ 0xff, counted); his contract with the side
/// (`get_contract_for_club`): the wage summed (also for the selected, asked again), the best paid (strictly more) and,
/// among contracts of a type other than 6, the worst paid (strictly less); the oldest (earliest date of birth: year,
/// then day; the first on ties) and the youngest (latest); the caps summed, the most capped (more than the most so
/// far, a signed byte) and the fewest (fewer); the most valuable (strictly more `staff::VALUE`); the counts of a
/// non-national team job with the squad flag 4, of a national team job, of foreigners (`database_is_staff_foreign(..,
/// club nation, 1)`), of non-EU players (`database_is_staff_eu_national`) and of the club's nationals. At the end, with
/// players, `AVERAGE_WAGE` = wages / count truncated and `AVERAGE_AGE` = the age sum / count as a float; with
/// selected players, `SELECTED_AVERAGE_WAGE` = their wages / their count truncated and `SELECTED_AGE` their age average.
/// Original: `club_screen_collect_squad_stats` (0x487840, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screen_collect_squad_stats(club: usize, reserve: u32, out: usize) {
    let club = Club(club);
    use club_squad_stats as s;
    let (mut oldest, mut youngest, mut scratch) = ([0u32; 2], [0u32; 2], [0u32; 2]);
    exe::date_date_54a110(oldest.as_mut_ptr() as usize);
    exe::date_date_54a110(youngest.as_mut_ptr() as usize);
    let (mut selected, mut most_caps, mut fewest_caps) = (0u8, 0u8, 0u8);
    let (mut age_sum, mut selected_age_sum) = (0u16, 0u16);
    let mut senior = 0u32;
    let mut side = club.addr();
    if exe::club_get_linked_b_team(club, &mut senior, 1) != 0 && senior == 0 {
        side = exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1);
    }
    let mut squad = [0usize; 100];
    exe::database_collect_club_squad(club, squad.as_mut_ptr(), 1);
    for at in [s::SELECTED_AGE, s::AVERAGE_AGE, s::WAGES, s::SELECTED_WAGES, s::AVERAGE_WAGE, s::OLDEST, s::YOUNGEST, s::BEST_PAID,
        s::BEST_WAGE, s::WORST_PAID, s::WORST_WAGE, s::MOST_CAPPED, s::LEAST_CAPPED, s::MOST_VALUABLE] {
        write_u32(out + at, 0);
    }
    for at in [s::COUNT, s::INJURED, s::CLUB_JOB_ELSEWHERE, s::INTERNATIONALS, s::FOREIGNERS, s::NON_EU, s::NATIONALS] {
        write_u8(out + at, 0);
    }
    write_u16(out + s::CAPS, 0);
    let bump = |at: usize| write_u8(out + at, read_u8(out + at).wrapping_add(1));
    let club_nation = club.nation().addr();
    for &p in squad.iter().take(0x32) {
        if p == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        if exe::database_is_national_team(club) != 0 {
            let flag = if reserve != 0 { 4 } else { 8 };
            if read_u8(p + staff::PLAYING_SQUAD) & flag != 0 {
                continue;
            }
        }
        bump(s::COUNT);
        if exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2 {
            bump(s::INJURED);
        }
        let age = read_i8(p + staff::AGE) as i16 as u16;
        age_sum = age_sum.wrapping_add(age);
        if exe::tactics_find_selection_slot(read_usize(G_TACTICS), side, p, 0) != 0xff {
            selected_age_sum = selected_age_sum.wrapping_add(age);
            selected = selected.wrapping_add(1);
        }
        let contract = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, side);
        if contract != 0 {
            let wage = read_i32(contract + 0xc);
            write_i32(out + s::WAGES, read_i32(out + s::WAGES).wrapping_add(wage));
            if exe::tactics_find_selection_slot(read_usize(G_TACTICS), side, p, 0) != 0xff {
                write_i32(out + s::SELECTED_WAGES, read_i32(out + s::SELECTED_WAGES).wrapping_add(read_i32(contract + 0xc)));
            }
            if read_usize(out + s::BEST_PAID) == 0 || read_i32(contract + 0xc) > read_i32(out + s::BEST_WAGE) {
                write_usize(out + s::BEST_PAID, p);
                write_i32(out + s::BEST_WAGE, read_i32(contract + 0xc));
            }
            if read_u8(contract + 0x35) & 0x3f != 6
                && (read_usize(out + s::WORST_PAID) == 0 || read_i32(contract + 0xc) < read_i32(out + s::WORST_WAGE)) {
                write_usize(out + s::WORST_PAID, p);
                write_i32(out + s::WORST_WAGE, read_i32(contract + 0xc));
            }
        }
        let birth = p + staff::DATE_OF_BIRTH;
        let (day, year) = (read_i16(birth), read_u16(birth + 2));
        let (oday, oyear) = (oldest[0] as u16 as i16, (oldest[0] >> 16) as u16);
        if read_usize(out + s::OLDEST) == 0 || oyear > year || (oyear == year && oday > day) {
            write_usize(out + s::OLDEST, p);
            exe::date_operator_3d(oldest.as_mut_ptr() as usize, scratch.as_mut_ptr() as usize, birth);
        }
        let (yday, yyear) = (youngest[0] as u16 as i16, (youngest[0] >> 16) as u16);
        if read_usize(out + s::YOUNGEST) == 0 || yyear < year || (yyear == year && yday < day) {
            write_usize(out + s::YOUNGEST, p);
            exe::date_operator_3d(youngest.as_mut_ptr() as usize, scratch.as_mut_ptr() as usize, birth);
        }
        let caps = read_u8(p + staff::INT_APPS);
        write_u16(out + s::CAPS, read_u16(out + s::CAPS).wrapping_add(caps as u16));
        if caps as i32 > most_caps as i8 as i32 || read_usize(out + s::MOST_CAPPED) == 0 {
            write_usize(out + s::MOST_CAPPED, p);
            most_caps = read_u8(p + staff::INT_APPS);
        }
        if (caps as i32) < fewest_caps as i8 as i32 || read_usize(out + s::LEAST_CAPPED) == 0 {
            write_usize(out + s::LEAST_CAPPED, p);
            fewest_caps = read_u8(p + staff::INT_APPS);
        }
        let valuable = Staff(read_usize(out + s::MOST_VALUABLE));
        if valuable.is_null() || read_i32(p + staff::VALUE) > (valuable.value() as i32) {
            write_usize(out + s::MOST_VALUABLE, p);
        }
        let job = read_usize(p + staff::NATIONAL_JOB);
        if job != 0 && exe::club_is_national_team(job) == 0 && read_u8(p + staff::PLAYING_SQUAD) & 4 != 0 {
            bump(s::CLUB_JOB_ELSEWHERE);
        }
        let job = read_usize(p + staff::NATIONAL_JOB);
        if job != 0 && exe::club_is_national_team(job) != 0 {
            bump(s::INTERNATIONALS);
        }
        if exe::database_is_staff_foreign(p, club.nation().addr(), 1) != 0 {
            bump(s::FOREIGNERS);
        }
        if exe::database_is_staff_eu_national(p, club.nation().addr()) == 0 {
            bump(s::NON_EU);
        }
        if read_usize(p + staff::NATION) == club_nation {
            bump(s::NATIONALS);
        }
    }
    let count = read_i8(out + s::COUNT);
    if count != 0 {
        write_i32(out + s::AVERAGE_WAGE, ftol(read_i32(out + s::WAGES) as f64 / count as f64));
    }
    if selected != 0 {
        let n = selected as i8 as f64;
        write_i32(out + s::SELECTED_AVERAGE_WAGE, ftol(read_i32(out + s::SELECTED_WAGES) as f64 / n));
        write_f32(out + s::SELECTED_AGE, (selected_age_sum as i16 as f64 / n) as f32);
    }
    if count != 0 {
        write_f32(out + s::AVERAGE_AGE, (age_sum as i16 as f64 / count as f64) as f32);
    }
}

/// Comparer for `club_screens_init_match_record`.
pub(crate) unsafe extern "C" fn compare_club_screens_init_match_record() {
    // simplification: a static constructor (atexit) runs once at the game start; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(IDX_CLUB_SCREENS_INIT_MATCH_RECORD)()
}

/// Comparer for `club_screens_exit_match_record`.
pub(crate) unsafe extern "C" fn compare_club_screens_exit_match_record() {
    // simplification: a static destructor runs once at the game exit; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(IDX_CLUB_SCREENS_EXIT_MATCH_RECORD)()
}

/// Comparer for `match_team_pair_construct`.
pub(crate) unsafe extern "thiscall" fn compare_match_team_pair_construct(this: usize) -> usize {
    // simplification: a constructor (the vector iterator registers its unwinding) runs once; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_MATCH_TEAM_PAIR_CONSTRUCT)(this)
}

/// Comparer for `match_team_block_destruct`.
pub(crate) unsafe extern "thiscall" fn compare_match_team_block_destruct(this: usize) {
    // simplification: a destructor runs once; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_MATCH_TEAM_BLOCK_DESTRUCT)(this)
}

/// Comparer for `match_team_block_construct`.
pub(crate) unsafe extern "thiscall" fn compare_match_team_block_construct(this: usize) -> usize {
    // simplification: a constructor runs once; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_MATCH_TEAM_BLOCK_CONSTRUCT)(this)
}

/// Comparer for `club_screens_init_history_states`.
pub(crate) unsafe extern "C" fn compare_club_screens_init_history_states() {
    // simplification: a static constructor (atexit) runs once at the game start; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(IDX_CLUB_SCREENS_INIT_HISTORY_STATES)()
}

/// Comparer for `club_history_state_construct`: the original only (it builds a list), then the plan: the 0x2a bytes
/// cleared and the state returned.
pub(crate) unsafe extern "thiscall" fn compare_club_history_state_construct(this: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_CLUB_HISTORY_STATE_CONSTRUCT)(this);
    let clear = (0..0x2a).all(|k| read_u8(this + k) == 0);
    SCOREBOARD.record(clear && r == this, || format!("club_history_state_construct({:#x}): cleared {} returned {:#x}", this, clear, r));
    r
}

/// Comparer for `club_history_state_destruct`.
pub(crate) unsafe extern "thiscall" fn compare_club_history_state_destruct(this: usize) {
    // simplification: a destructor runs once; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CLUB_HISTORY_STATE_DESTRUCT)(this)
}

/// Comparer for `club_screens_exit_history_states`.
pub(crate) unsafe extern "C" fn compare_club_screens_exit_history_states() {
    // simplification: a static destructor runs once at the game exit; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(IDX_CLUB_SCREENS_EXIT_HISTORY_STATES)()
}

/// Comparer for `open_club_squad_screen`.
pub(crate) unsafe extern "C" fn compare_open_club_squad_screen(club: usize, reserve: u32, restore: u32, tab5: u32, highlight: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32)>(IDX_OPEN_CLUB_SQUAD_SCREEN)(club, reserve, restore, tab5, highlight);
    let node = current_screen_node(G_SCREEN_MANAGER);
    if node == 0 || read_usize(node) != FN_CLUB_SCREEN_BUILD || get_screen_param(0) != club {
        return;
    }
    let (count, best) = if club != 0 { club_squad_stars(club) } else { (0, [0; 3]) };
    let id = |k: usize, need: i8| if count >= need { read_usize(best[k]) } else { usize::MAX };
    let want = [id(0, 1), id(1, 2), id(2, 3)];
    let got = [get_screen_param(0x20), get_screen_param(0x21), get_screen_param(0x22)];
    SCOREBOARD.record(want == got, || format!("open_club_squad_screen (club screen, squad): best players original={:?} rust={:?}", got, want));
}

/// Comparer for `menu_item_menu_item`: the two words side by side.
pub(crate) unsafe extern "thiscall" fn compare_menu_item_menu_item(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_MENU_ITEM_MENU_ITEM);
    let (r, n, same, _, _) = crate::comparison::side_by_side_light(&[(this, 4)], || o(this), || menu_item_menu_item(this));
    SCOREBOARD.record(same && r == n, || format!("MenuItem::MenuItem({:#x}) differs", this));
    r
}

/// Comparer for `club_screen_collect_squad_stats`: the 0x45-byte statistics side by side.
pub(crate) unsafe extern "C" fn compare_club_screen_collect_squad_stats(club: usize, reserve: u32, out: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize)>(IDX_CLUB_SCREEN_COLLECT_SQUAD_STATS);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(out, club_squad_stats::SIZE)], || o(club, reserve, out),
        || club_screen_collect_squad_stats(club, reserve, out));
    SCOREBOARD.record(same, || format!("club_screen_collect_squad_stats (club screen, squad statistics): differences {:?}", first_differences(&mo, &mr)));
}

/// The `Display` of the screen manager's current slot (`screen_manager::DISPLAY` of the slot
/// `screen_manager::CURRENT_SLOT`, 0x300 bytes per slot), read from the manager `sm` as the original does.
pub(crate) unsafe fn screens_current_display(sm: usize) -> usize {
    read_usize(sm + screen_manager::DISPLAY + read_i16(sm + screen_manager::CURRENT_SLOT) as isize as usize * 0x300)
}

/// Comparer of a forwarder: side by side over `screens_forwarder_regions` and the return value.
pub(crate) unsafe fn compare_screens_forwarder(name: &str, sm: usize, original: impl FnOnce() -> u32, rust: impl FnOnce() -> u32) -> u32 {
    compare_screens_over(name, &screens_forwarder_regions(sm), original, rust)
}

/// Comparer of an item forwarder: side by side over `scrman_item_regions` (the item's area or the free item order
/// too, so the item the original appended is not appended again by the copy) and the return value.
pub(crate) unsafe fn compare_screens_item(name: &str, sm: usize, area: u32, original: impl FnOnce() -> u32, rust: impl FnOnce() -> u32) -> u32 {
    compare_screens_over(name, &scrman_item_regions(sm, area), original, rust)
}

/// The "Error" box of club_screens.cpp at line `row` ("v%s %s.%s %d": the version, the file's name and extension
/// split by `exe::crt_strtok`, which is the C runtime's _splitpath, and the line), then the mouse button released
/// (`G_MOUSE_LEFT_BUTTON_DOWN` = 0).
pub(crate) unsafe fn club_screens_error_box(row: u32) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(CLUB_SCREENS_CPP_PATH, 0, 0, name.as_mut_ptr(), ext.as_mut_ptr());
    let v = version(name.as_mut_ptr(), ext.as_mut_ptr(), row);
    format(0xae24d0, 0x9870e8, v, name.as_ptr(), ext.as_ptr(), row);
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0);
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
}

/// `HUMAN_MANAGER::get_squad_view_settings(human)` (0x5fe950, thiscall, ret 4): the 15-byte block where the human
/// keeps the squad screen's view (0 = the current human).
pub(crate) unsafe fn club_screens_squad_view_settings() -> usize {
    exe::human_manager_get_squad_view_settings(read_usize(0xb63c98), 0)
}

/// The club screen's parameters (0..0x22, 0 = not written) that `open_manager_club_screen` gives the new screen: the
/// plan, worked out before the screen opens (it reads the current screen's parameters to restore them).
pub(crate) unsafe fn open_manager_club_screen_plan(manager: Staff, squad_param: u32, restore_params: u32, open_tab_5: u32) -> [u32; 0x23] {
    let rec = exe::get_human_manager_record(manager);
    let club = Club(if rec != 0 && read_u32(rec + human_manager_sheet::LIVE_MATCH) != 0 {
        read_usize(rec + human_manager_sheet::CLUB)
    } else if manager.club_job().addr() != 0 {
        manager.club_job().addr()
    } else {
        manager.national_job().addr()
    });
    let mut saved = [0u32; 60];
    if restore_params != 0 {
        for (k, s) in saved.iter_mut().enumerate() {
            *s = get_screen_param(k as u32) as u32;
        }
    }
    // the default view when the human does not run the club (the original's local block)
    let mut default_view = [0u8; 16];
    default_view[0..4].copy_from_slice(&0x41u32.to_le_bytes());
    default_view[4..8].copy_from_slice(&8u32.to_le_bytes());
    default_view[8..12].copy_from_slice(&0x154a21u32.to_le_bytes());
    default_view[12] = 6;
    default_view[13] = 0xff;
    default_view[14] = 0xff;
    let view = if !club.is_null() && exe::human_manager_current_controls_club(club, 0, 1) != 0 {
        club_screens_squad_view_settings()
    } else {
        default_view.as_ptr() as usize
    };
    let year = read_u16(G_CURRENT_DATE + 2) as u32;
    let mut p = [0u32; 0x23];
    p[0] = club.addr() as u32;
    p[1] = squad_param;
    p[4] = if open_tab_5 != 0 { 5 } else { 1 };
    p[7] = read_u32(view);
    p[8] = read_u32(view + 4);
    p[9] = read_i8(view + 0xc) as i32 as u32;
    p[0xc] = year;
    p[0xf] = 1;
    p[0x12] = 1;
    p[0x17] = year;
    p[0x18] = read_u32(view + 8);
    p[0x19] = read_i8(view + 0xd) as i32 as u32;
    p[0x1a] = read_i8(view + 0xe) as i32 as u32;
    p[0x1b] = if open_tab_5 != 0 { 2 } else { 1 };
    if restore_params != 0 {
        for k in [4usize, 7, 8, 9, 0x10, 0x11, 0x12, 0x18, 0x19, 0x1a, 0x1b] {
            p[k] = saved[k];
        }
    }
    // the three players of the squad with the best reputation (world for a national team, otherwise current), by
    // strict comparison of the signed words, starting from -10000
    let (mut best, mut second, mut third) = (0usize, 0usize, 0usize);
    let (mut s1, mut s2, mut s3) = (-10000i16, -10000i16, -10000i16);
    let mut count = 0u8;
    if !club.is_null() {
        for i in 0..50 {
            let pl = Staff(club.squad(i));
            if pl.is_null() || pl.player().addr() == 0 {
                continue;
            }
            let sheet = pl.player();
            let score = if exe::database_is_national_team(club) != 0 {
                sheet.world_reputation()
            } else {
                sheet.current_reputation()
            };
            if count < 3 {
                count += 1;
            }
            if score > s1 {
                third = second;
                second = best;
                s3 = s2;
                s2 = s1;
                s1 = score;
                best = pl.addr();
            } else if score > s2 {
                s3 = s2;
                third = second;
                s2 = score;
                second = pl.addr();
            } else if score > s3 {
                s3 = score;
                third = pl.addr();
            }
        }
    }
    p[0x20] = if count > 0 { read_u32(best) } else { u32::MAX };
    p[0x21] = if count > 1 { read_u32(second) } else { u32::MAX };
    p[0x22] = if count > 2 { read_u32(third) } else { u32::MAX };
    p
}

/// `LLIST::LLIST` (0x68d020) and `LLIST::~LLIST` (0x68d040), thiscall: the screen's local fixture list.
pub(crate) unsafe fn club_screens_llist(l: &mut [u8; 0x20], construct: bool) {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(if construct { FN_LLIST_LLIST } else { FN_LLIST_DESTRUCTOR_LLIST });
    f(l.as_mut_ptr() as usize);
}

/// A copy (operator new, `fixture::SIZE` bytes) of the fixture `f`, for the screen parameters 2 and 3.
pub(crate) unsafe fn club_screens_copy_fixture(f: usize) -> usize {
    let c = exe::crt_operator_new(fixture::SIZE as u32);
    core::ptr::copy_nonoverlapping(f as *const u8, c as *mut u8, fixture::SIZE);
    c
}

/// The length of a zero-ended text (strlen).
pub(crate) unsafe fn club_screens_text_length(p: *const u8) -> usize {
    core::ffi::CStr::from_ptr(p as *const core::ffi::c_char).to_bytes().len()
}

/// Rule: the club's transfers tab lists, for the season shown, the players in and out, the future transfers, the loans
/// or the staff moves (as the View menu picks), each with its date, the person, the other club and the fee, and the
/// balance of the fees at the bottom.
///
/// The transfers tab of `club_screen_build` (inline in the original, 0x455273..0x45633d), between `top` and `bottom`.
/// The parameters: 0 the club, 0x12 the view flags (1 players in, 2 players out, 4 future transfers, 0x10 loans in,
/// 0x20 loans out, 0x40 staff in, 0x80 staff out), 0x15 and 0x16 the past transfers array and its count (kept between
/// builds), 0x17 the season's year, 0x13 the scroll row. With any view but the future transfers (flags & 0xf3), the season
/// arrows ("<<" button 0x1b when the year is above [0xdd6eca], ">>" button 0x1c when below the current year; the current
/// year one less when the club's key nation, `exe::key_nation_index_of_club`, has its season start +0x15 ahead of today
/// and +0x17 set; otherwise plain texts). The "View" button with its popup (`screenmanager_add_popup_area`) of the eight
/// views, the chosen ones ticked (`screenmanager_add_ticked_popup_item`, the text of the last ticked one kept as the
/// title) and the others plain (`ScreenManager::add_menu_item`, 0x415640). The season: from the key nation's start day
/// (+0x15, or day 0 without +0x17) of the year to the day before it a year later; in 2001 from day 1. The title "<view>
/// - Season <season>" (`key_nation_format_club_season`, 0x669630) when a view is chosen, else the view text; the rows that
/// fit in font 3. Future transfers (`exe::transfer_manager_get_club_future_transfer_list`, 0x1b bytes each, freed at the
/// end): those of a club the human runs or with the other club run by a human (or stage +0x19 = 6, or +0x1a set); the
/// stage (Offer, Negotiation, Contract, Work Permit, Confirmation, Delayed, else the date), the player, "to"/"from" the
/// other club (or "from Free Transfer"), the fee (with an exchange "<cash> p/x"; without fee "Exch", "Bos" or "Free"), the
/// balance (an outgoing fee is added once per player, an incoming one subtracted; a negative balance with "-"). Past
/// transfers (`TRANSFER_MANAGER::get_past_transfers`, 0x8cc030, by the months and years of the season's ends, into params 0x15
/// and 0x16): those of the season's dates
/// passing the view's filters (0x51: the club is +0x10; 0xa2: +0x14; 0x33: kind +0x1c not 5; 0xc0: kind 5; 0x30: kind 2;
/// 3: kind not 2), sorted (qsort with 0x8e5540), each with its date, the name (a button when the person still matches the
/// names, `exe::database_staff_matches_names`), the other club (+0x14 with 0x51, otherwise +0x10; "Free Transfer"
/// without), the fee or the kind word ("Loan" too), and the sum of the fees. The list area's scroll row from param 0x13
/// (`screenmanager_set_area_scroll_row`) and the area written to param 0x14.
pub(crate) unsafe fn club_screen_draw_transfers_tab(top: i32, bottom: i32) {
    let sm = G_SCREEN_MANAGER;
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let area_button: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let area_text: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let button: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let manager_text: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let add_area: DisplayAddAreaFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let w = |a: usize| read_u16(a) as usize;
    let weights = [5u8, 0xd, 0xa, 5];
    let (mut start, mut end) = ([0u8; 8], [0u8; 8]);
    exe::date_date_54a110(start.as_mut_ptr() as usize);
    exe::date_date_54a110(end.as_mut_ptr() as usize);
    let mut future: usize = 0;
    let mut total: i32 = 0;
    let club = get_screen_param(0);
    let flags = get_screen_param(0x12) as u32;
    let mut past = get_screen_param(0x15);
    let mut past_count = get_screen_param(0x16) as i32;
    let mut season_year = get_screen_param(0x17) as u32;
    let knr = 0xb63d60 + exe::key_nation_index_of_club(club) as i8 as isize as usize * 0x48;
    let mut title = [0u8; 0xc8];
    let mut current_year: u32;
    let mut ebp = top;
    if flags & 0xf3 != 0 {
        covered!("tel01_2 transfers season arrows");
        let arrows = add_area(screens_current_display(sm), 0x6e, top as u32, 0x168, (top + 0x14) as u32, 2, 0, 1, 0, 1, 0, u32::MAX);
        current_year = read_u16(G_CURRENT_DATE + 2) as u32;
        if read_u32(knr + 0x17) != 0 && (read_i16(G_CURRENT_DATE) as i32) < read_u16(knr + 0x15) as i32 {
            season_year = season_year.wrapping_add(0xffff);
            current_year = current_year.wrapping_add(0xffff);
        }
        text(0xde1f64, 0x98e7ac, 0x98e7c4usize); // "<<"
        if season_year as u16 > read_u16(0xdd6eca) {
            area_button(sm, arrows as usize, 0, 0, 0x30, w(0xae317e), 0xc, 1, w(0xae3184), 0xde1f64, 0x1b, 0);
        } else {
            area_text(sm, arrows as usize, 0, 0, 0x30, w(0xae317e), 0x2c, 1, w(0xae3184), 0xde1f64);
        }
        text(0xde1f64, 0x98e790, 0x98e7a8usize); // ">>"
        if (season_year as u16) < current_year as u16 {
            area_button(sm, arrows as usize, 1, 0, 0x30, w(0xae317e), 0xc, 1, w(0xae3184), 0xde1f64, 0x1c, 0);
        } else {
            area_text(sm, arrows as usize, 1, 0, 0x30, w(0xae317e), 0x2c, 1, w(0xae3184), 0xde1f64);
        }
    }
    text(0xde1f64, 0x988004); // "View"
    let b = button(sm, 0x28f, top as usize, 0x30c, (top + 0x14) as usize, 0x30, w(0xae317e), 0xc, 1, w(0xae3184), 0xde1f64, 0, 0);
    let popup: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_SCREENMANAGER_ADD_POPUP_AREA);
    let pa = popup(sm, b as u32);
    let ticked: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_SCREENMANAGER_ADD_TICKED_POPUP_ITEM);
    let plain: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_SCREENMANAGER_ADD_MENU_ITEM);
    for (label, bit) in [(0x98e784usize, 1u32), (0x98e774, 2), (0x98e760, 4), (0x98e754, 0x10), (0x98e748, 0x20), (0x98e73c, 0x40), (0x98e730, 0x80)] {
        text(0xde1f64, label);
        if flags & bit != 0 {
            ticked(sm, pa, 0xde1f64, 0x1a, bit);
            copy_text(title.as_mut_ptr(), 0xde1f64 as *const u8);
        } else {
            plain(sm, pa, 0xde1f64, 0x1a, bit);
        }
    }
    let mut tmp = [0u8; 8];
    if read_u32(knr + 0x17) != 0 {
        club_screens_date(&mut tmp, read_u16(knr + 0x15) as u32, season_year);
    } else {
        club_screens_date(&mut tmp, 0, season_year);
    }
    start = tmp;
    let (lo, hi) = (u32::from_le_bytes([start[0], start[1], start[2], start[3]]), u32::from_le_bytes([start[2], start[3], start[4], start[5]]));
    let a = club_screens_date(&mut tmp, lo, hi.wrapping_add(1));
    exe::date_sub_days(a, end.as_mut_ptr() as usize, 1);
    // the dword that later carries the row count holds the first dword of the season's last date (in 2001, of day 1)
    let mut rows_word = u32::from_le_bytes([end[0], end[1], end[2], end[3]]);
    if season_year as u16 == 0x7d1 {
        club_screens_date(&mut tmp, 1, 0x7d1);
        start = tmp;
        rows_word = u32::from_le_bytes([start[0], start[1], start[2], start[3]]);
    }
    let mut season = [0u8; 0xc8];
    exe::key_nation_format_club_season(season_year, club, (season.as_mut_ptr()) as *mut u8 as usize, 0xc8);
    if flags & 0xf3 != 0 {
        text(0xde1f64, 0x98e6f0, title.as_ptr(), season.as_ptr());
    } else {
        copy_text(0xde1f64 as *mut u8, title.as_ptr());
    }
    ebp += 0x19;
    manager_text(sm, 0x6e, ebp as usize, 0x30c, (ebp + 0x23) as usize, 2, 0, 0xc, 3, w(0xaebdec), 0xde1f64, 1);
    ebp += 0x28;
    // the row count is a byte written over the low byte of that dword, as the original
    let rows = (rows_word & 0xffff_ff00) | ((bottom - ebp - 0x10) / exe::font_get_height(3)) as u8 as u32;
    let area;
    if flags & 4 != 0 {
        covered!("tel01_2 future transfers");
        let list: unsafe extern "thiscall" fn(usize, usize, *mut usize) -> i32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_CLUB_FUTURE_TRANSFER_LIST);
        let count = list(read_usize(G_TRANSFER_MANAGER), club, &mut future);
        let add: DisplayAddAreaFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA);
        area = add(sm, 0x6e, ebp as u32, 0x30c, bottom as u32, 4, weights.as_ptr() as u32, rows, 0, 2, 0, u32::MAX);
        let mut row = 0usize;
        let mut k = 0i32;
        while k < count {
            let rec = future + k as usize * 0x1b;
            let mut shown = read_u8(rec + 0x1a) != 0 || read_u8(rec + 0x19) == 6 || exe::human_manager_current_controls_club(club, 0, 1) != 0;
            if !shown {
                let other = read_i32(rec + 0xc);
                shown = other != -1 && exe::human_manager_current_controls_club(club_screens_club(other), 0, 1) != 0;
            }
            if !shown {
                k += 1;
                continue;
            }
            match read_u8(rec + 0x19) {
                0 => { text(0xde1f64, 0x98e6e8); } // "Offer"
                1 => { text(0xde1f64, 0x98e6dc); } // "Negotiation"
                2 => { text(0xde1f64, 0x98e6d0); } // "Contract"
                3 => { text(0xde1f64, 0x98e6c4); } // "Work Permit"
                4 => { text(0xde1f64, 0x98e6b4); } // "Confirmation"
                5 => { text(0xde1f64, 0x98e6ac); } // "Delayed"
                _ => club_screens_short_date(rec, 0xde1f64, 0x7d0, 0xff),
            }
            area_text(sm, area as usize, 0, row, 0x30, w(0xaebe04), 0xc, 1, w(0xae3184), 0xde1f64);
            let person = club_screens_staff(read_i32(rec + 8));
            exe::database_format_staff_name(person, title.as_mut_ptr() as usize, 0xc8, 1);
            format(0xde1f64, 0x987ff0, title.as_ptr());
            area_button(sm, area as usize, 1, row, 1, 0, 1, 3, w(0xae3184), 0xde1f64, 0x7db, person);
            let other = read_i32(rec + 0xc);
            if read_u32(rec + 0x10) != 0 {
                let c = club_screens_club(other);
                let g = read_i8(c + club::SHORT_GENDER_NAME) as i32;
                text(0xde1f64, 0x98e684, g, g, c + club::SHORT_NAME);
                // the fee counts once per player (an earlier line with the same person does not add it again)
                let seen = (0..k).any(|j| read_i32(future + j as usize * 0x1b + 8) == read_i32(rec + 8));
                if !seen {
                    total = total.wrapping_add(read_i32(rec + 0x14));
                }
            } else {
                if other == -1 {
                    text(0xde1f64, 0x98e670); // "from Free Transfer"
                } else {
                    let c = club_screens_club(other);
                    let g = read_i8(c + club::SHORT_GENDER_NAME) as i32;
                    text(0xde1f64, 0x98e644, g, g, c + club::SHORT_NAME);
                }
                total = total.wrapping_sub(read_i32(rec + 0x14));
            }
            if other == -1 {
                area_text(sm, area as usize, 2, row, 1, 0, 1, 2, w(0xaebdec), 0xde1f64);
            } else {
                area_button(sm, area as usize, 2, row, 1, 0, 1, 2, w(0xaebdec), 0xde1f64, 0x7d5, club_screens_club(other));
            }
            let fee = read_i32(rec + 0x14);
            if fee != 0 && read_u8(rec + 0x18) == 1 {
                let amount = fee;
                let v = exe::cash_get_display_amount(&amount as *const i32 as *const u32, 5);
                exe::cash_format_money(title.as_mut_ptr() as usize, 0xc8, v, 5);
                text(0xde1f64, 0x98e628, title.as_ptr());
            } else if fee != 0 {
                club_screens_format_cash(fee, 0xde1f64, 0x7d0, 5);
            } else {
                text(0xde1f64, club_screens_free_kind(read_u8(rec + 0x18), false));
            }
            area_text(sm, area as usize, 3, row, 0x30, w(0xae3192), 0xc, 1, w(0xae3184), 0xde1f64);
            row += 1;
            k += 1;
        }
        if total < 0 {
            format(0xde1f64, 0x98ba0c); // "-"
            club_screens_format_cash(total.wrapping_neg(), 0xde1f65, 0x7d0, 5);
        } else {
            club_screens_format_cash(total, 0xde1f64, 0x7d0, 5);
        }
        area_text(sm, area as usize, 3, row, 0x30, w(0xae3192), 0xc, 1, w(0xae3152), 0xde1f64);
        if future != 0 {
            exe::crt_free(future);
        }
    } else {
        covered!("tel01_2 past transfers");
        let end_year = u32::from_le_bytes([end[2], end[3], end[4], end[5]]);
        let start_year = u32::from_le_bytes([start[2], start[3], start[4], start[5]]);
        // the month index (0..11, or -1 when out of the table) of each end of the season, sign-extended as the original's eax
        let e = exe::date_get_month(end.as_ptr() as usize) as i8 as i32 as u32;
        let s = exe::date_get_month(start.as_ptr() as usize) as i8 as i32 as u32;
        exe::transfer_manager_get_past_transfers(read_usize(G_TRANSFER_MANAGER), s, start_year, e, end_year, &mut past, &mut past_count);
        set_screen_param(0x15, past);
        set_screen_param(0x16, past_count as u32 as usize);
        let add: DisplayAddAreaFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA);
        area = add(sm, 0x6e, ebp as u32, 0x30c, bottom as u32, 4, weights.as_ptr() as u32, rows, 0, 2, 0, u32::MAX);
        let mut picked: usize = 0;
        if past_count != 0 {
            picked = exe::crt_malloc((past_count as u32).wrapping_mul(4));
        }
        let mut n = 0u32;
        let id = read_u32(club);
        let start_lo = u32::from_le_bytes([start[0], start[1], start[2], start[3]]);
        let start_hi = u32::from_le_bytes([start[4], start[5], start[6], start[7]]);
        let end_lo = u32::from_le_bytes([end[0], end[1], end[2], end[3]]);
        let end_hi = u32::from_le_bytes([end[4], end[5], end[6], end[7]]);
        let mut i = 0i32;
        while i < past_count {
            let rec = past + i as usize * 0x25;
            i += 1;
            let kind = read_u8(rec + 0x1c);
            if flags & 0x51 != 0 && read_u32(rec + 0x10) != id {
                continue;
            }
            if flags & 0xa2 != 0 && read_u32(rec + 0x14) != id {
                continue;
            }
            if (flags & 0x33 != 0 && kind == 5) || (flags & 0xc0 != 0 && kind != 5) || (flags & 0x30 != 0 && kind != 2) || (flags & 3 != 0 && kind == 2) {
                continue;
            }
            if exe::date_operator_3c(rec + 0x1d, start_lo, start_hi) != 0 || exe::date_operator_3e(rec + 0x1d, end_lo, end_hi) != 0 {
                continue;
            }
            write_usize(picked + n as usize * 4, rec);
            n += 1;
        }
        exe::crt_qsort(picked, n, 4, 0x8e5540usize);
        let mut row = 0usize;
        for k in 0..n as usize {
            let rec = read_usize(picked + k * 4);
            club_screens_short_date(rec + 0x1d, 0xde1f64, 0x7d0, 0xff);
            area_text(sm, area as usize, 0, row, 0x30, w(0xaebe04), 0xc, 1, w(0xae3184), 0xde1f64);
            exe::database_format_name_from_ids(read_u32(rec + 4), read_u32(rec + 8), read_u32(rec + 0xc), title.as_mut_ptr(), 0xc8, 1, read_u32(rec));
            format(0xde1f64, 0x987ff0, title.as_ptr());
            let person = club_screens_staff(read_i32(rec));
            if exe::database_staff_matches_names(person, read_u32(rec + 4), read_u32(rec + 8), read_u32(rec + 0xc), 0usize) != 0 {
                area_button(sm, area as usize, 1, row, 1, 0, 1, 3, w(0xae3184), 0xde1f64, 0x7d7, person);
            } else {
                area_text(sm, area as usize, 1, row, 1, 0, 1, 3, w(0xae3184), 0xde1f64);
            }
            let other = if flags & 0x51 != 0 { read_i32(rec + 0x14) } else { read_i32(rec + 0x10) };
            total = total.wrapping_add(read_i32(rec + 0x18));
            if other > -1 {
                let c = club_screens_club(other);
                copy_text(0xde1f64 as *mut u8, (c + club::SHORT_NAME) as *const u8);
                area_button(sm, area as usize, 2, row, 1, 0, 1, 2, w(0xaebdec), 0xde1f64, 0x7d5, c);
            } else {
                text(0xde1f64, 0x98e604); // "Free Transfer"
                area_text(sm, area as usize, 2, row, 1, 0, 1, 2, w(0xaebdec), 0xde1f64);
            }
            let fee = read_i32(rec + 0x18);
            if fee != 0 && read_u8(rec + 0x1c) == 1 {
                club_screens_format_cash(fee, title.as_mut_ptr() as usize, 0xc8, 5);
                text(0xde1f64, 0x98e628, title.as_ptr());
            } else if fee != 0 {
                club_screens_format_cash(fee, 0xde1f64, 0x7d0, 5);
            } else {
                text(0xde1f64, club_screens_free_kind(read_u8(rec + 0x1c), true));
            }
            area_text(sm, area as usize, 3, row, 0x30, w(0xae3192), 0xc, 1, w(0xae3184), 0xde1f64);
            row += 1;
        }
        club_screens_format_cash(total, 0xde1f64, 0x7d0, 5);
        area_text(sm, area as usize, 3, row, 0x30, w(0xae3192), 0xc, 1, w(0xae3152), 0xde1f64);
        if picked != 0 {
            exe::crt_free(picked);
        }
    }
    exe::screenmanager_set_area_scroll_row(sm, area, get_screen_param(0x13) as u32);
    set_screen_param(0x14, area as i16 as i32 as u32 as usize);
}

/// The menus of the squad tab, in the original's order.
///
/// The view bar area (0x6e..0x168 by 0x14) with the "View" button and its popup (link 7, the item of a set bit ticked):
/// Traditional 1 (title "Position(s)"), Contract 2 ("Contract Info"), Selection 0x1000 ("Selection Info"), Stats 4, More
/// Stats 8, Attributes 0x10, Other 0x20 ("Other Info"), and for a club the human runs a separator and Penalty Takers
/// 0x800. In the positions view, "Sort By" (link 0xc, the ticked one's text, without the first space, as the title):
/// Name 4, Position(s) 8, Squad Number 0x20, Form 0x80, Morale 0x100, Condition 0x200, Nationality (Club for a national
/// or all-star team) 0x2000, Age 0x4000, Int. Caps 0x8000, Int. Goals 0x10000, Basic Wage 0x20000, Contract Expiry
/// 0x40000, Contract Protected 0x20000000 (only with [0xae233c] = 1), Value 0x80000, Goals 0x100000, Conceded 0x200000,
/// Assists 0x400000, Av. Rating 0x800000. With Stats or More Stats, "Competitions" (link 9): the six categories
/// (`player_stats_get_category_name`), the one of param 9 ticked with "<category> Stats" as the title. With
/// Attributes, "Attributes" (link 7): Physical 0x40, Mental 0x80, Goalkeeping 0x100, Defensive 0x200, Attacking 0x400,
/// each ticked with its "... Attributes" title. Then the filter area (0x212..0x30c): for a club the human runs, "Undo
/// Clear" (when there is an undo copy of the selection, `Tactics::has_selection_undo`, and no penalty takers view, data
/// 1) or "Clear Squad" (data 0), link 0x25; "Filter" (link 8) with All Positions 1, Goalkeepers 2, Defenders 4,
/// Midfielders 8, Attackers 0x10, a separator, All Sides 0x20, Left Sided 0x40, Central 0x80, Right Sided 0x100, a
/// separator, Available Only (bit 0x1000: ticked with data 0x800, plain with 0x1000), Unavailable (0x2000: 0x800 or
/// 0x2000); for a club (not a national team) of a European nation (nation +0x71 continent id = [0x9cfa10]) with a
/// European competition (+0x1db, its own or its B team's, `club_get_linked_b_team`) a separator and European Squad
/// (0x8000: 0x4000 or 0x8000); with a reserve squad (`exe::database_club_has_reserve_squad`): for a club the human runs
/// (not a national team) a separator and Include Reserves (Include Seniors on the reserves; 0x400: 0x200 or 0x400),
/// otherwise bit 0x400 dropped from the filter. The title (font 3) under the bars; the top moves down 0x41. For a
/// national team the human runs, "<n> players selected (maximum <m>)" (`exe::database_count_club_squad` with 4, or 8
/// for the B squad; `NationalTeams::get_max_squad_size`) at the bottom, which moves up 0x19.
pub(crate) unsafe fn club_screen_squad_tab_menus(t: &mut SquadTab) {
    let sm = t.sm;
    t.bar = t.add_area([0x6e, t.top as u32, 0x168, (t.top + 0x14) as u32, 2, 0, 1, 0, 1, 0, u32::MAX]);
    t.title[0] = 0;
    let view_menu = t.popup_button(t.bar, 0, 0x988004); // "View"
    let v = t.view;
    for (label, bit, title) in [(0x98eedcusize, 1u32, 0x98eed0usize), (0x98eec4, 2, 0x98eeb4), (0x98ee88, 0x1000, 0x98ee78), (0x98ee70, 4, 0),
                                (0x98ee64, 8, 0), (0x98ee58, 0x10, 0), (0x98ee30, 0x20, 0x98ee24)] {
        squad_text(0xde1f64, label);
        let ticked = v & bit != 0;
        if ticked && title != 0 {
            squad_text(t.title.as_mut_ptr() as usize, title);
        }
        t.menu_item(view_menu, ticked, 7, bit);
    }
    if exe::human_manager_current_controls_club(t.club, 0, 1) != 0 {
        t.separator(view_menu);
        squad_text(0xde1f64, 0x98ee14); // " Penalty Takers"
        t.menu_item(view_menu, v & 0x800 != 0, 7, 0x800);
    }
    if v & 1 != 0 {
        covered!("tel01_3 sort menu");
        let sort_menu = t.popup_button(t.bar, 1, 0x98ee0c); // "Sort By"
        let club_label = if exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), t.club) == 0 { 0x98edb8 } else { 0x98edb0 };
        let mut items: [(usize, u32); 18] = [(0x98ee04, 4), (0x98edf4, 8), (0x98ede4, 0x20), (0x98eddc, 0x80), (0x98edd4, 0x100), (0x98edc8, 0x200),
            (club_label, 0x2000), (0x98eda8, 0x4000), (0x98ed9c, 0x8000), (0x98ed90, 0x10000), (0x98ed84, 0x20000), (0x98ed70, 0x40000),
            (0x98ed5c, 0x20000000), (0x98ed54, 0x80000), (0x98ed4c, 0x100000), (0x98ed40, 0x200000), (0x98ed34, 0x400000), (0x98ed28, 0x800000)];
        if read_u8(0xae233c) != 1 {
            items[12].0 = 0; // without the protection rule, no "Contract Protected"
        }
        for (label, bit) in items {
            if label == 0 {
                continue;
            }
            squad_text(0xde1f64, label);
            let ticked = t.sort & bit != 0;
            t.menu_item(sort_menu, ticked, 0xc, bit);
            if ticked {
                t.ticked_title();
            }
        }
    }
    if v & 4 != 0 || v & 8 != 0 {
        let comp_menu = t.popup_button(t.bar, 1, 0x98ed18); // "Competitions"
        let mut buf = [0u8; 0xc8];
        for cat in 1u8..=6 {
            exe::player_stats_get_category_name(read_usize(0xb74350), cat as u32, buf.as_mut_ptr(), 0xc8);
            squad_string_text(0x98ed14, buf.as_ptr() as usize); // " %s"
            if cat == t.category {
                let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
                f(t.title.as_mut_ptr() as usize, 0x98ecf4, buf.as_ptr());
                t.menu_item(comp_menu, true, 9, cat as u32);
            } else {
                t.menu_item(comp_menu, false, 9, cat as u32);
            }
        }
    }
    if v & 0x10 != 0 {
        let attr_menu = t.popup_button(t.bar, 1, 0x98ece8); // "Attributes"
        for (label, bit, title) in [(0x98ecdcusize, 0x40u32, 0x98ecc8usize), (0x98ecc0, 0x80, 0x98ecac), (0x98eca0, 0x100, 0x98ec88),
                                    (0x98ec7c, 0x200, 0x98ec64), (0x98ec58, 0x400, 0x98ec40)] {
            squad_text(0xde1f64, label);
            let ticked = v & bit != 0;
            if ticked {
                squad_text(t.title.as_mut_ptr() as usize, title);
            }
            t.menu_item(attr_menu, ticked, 7, bit);
        }
    }
    let filter_area = t.add_area([0x212, t.top as u32, 0x30c, (t.top + 0x14) as u32, 2, 0, 1, 0, 1, 0, u32::MAX]);
    t.ebp = filter_area;
    if exe::human_manager_current_controls_club(t.club, 0, 1) != 0 {
        if v & 0x800 == 0 && exe::tactics_has_selection_undo(read_usize(0xde1d08), t.club, t.reserves) != 0 {
            squad_text(0xde1f64, 0x98ec0c); // "Undo Clear"
            t.button(filter_area, 0, 0, 0x30, squad_colour(0xae317e), 0xc, 1, squad_colour(0xae3184), 0xde1f64, 0x25, 1);
        } else {
            squad_text(0xde1f64, 0x98ec00); // "Clear Squad"
            t.button(filter_area, 0, 0, 0x30, squad_colour(0xae317e), 0xc, 1, squad_colour(0xae3184), 0xde1f64, 0x25, 0);
        }
    }
    let filter_menu = t.popup_button(filter_area, 1, 0x98ebf8); // "Filter"
    let mut f = t.filter;
    for (label, bit) in [(0x98ebe8usize, 1u32), (0x98ebd8, 2), (0x98ebcc, 4), (0x98ebbc, 8), (0x98ebb0, 0x10)] {
        squad_text(0xde1f64, label);
        t.menu_item(filter_menu, f & bit != 0, 8, bit);
    }
    t.separator(filter_menu);
    for (label, bit) in [(0x98eba4usize, 0x20u32), (0x98eb98, 0x40), (0x98eb8c, 0x80), (0x98eb7c, 0x100)] {
        squad_text(0xde1f64, label);
        t.menu_item(filter_menu, f & bit != 0, 8, bit);
    }
    t.separator(filter_menu);
    // the toggles: ticked they carry 0x800 (or 0x4000, 0x200), plain their own bit
    squad_text(0xde1f64, 0x98eb6c); // " Available Only"
    t.menu_item(filter_menu, f & 0x1000 != 0, 8, if f & 0x1000 != 0 { 0x800 } else { 0x1000 });
    squad_text(0xde1f64, 0x98eb40); // " Unavailable"
    t.menu_item(filter_menu, f & 0x2000 != 0, 8, if f & 0x2000 != 0 { 0x800 } else { 0x2000 });
    let club = t.club;
    if exe::database_is_national_team(club) == 0 {
        let nation = read_usize(club + 0x53);
        if nation != 0 && read_usize(nation + 0x71) != 0 && read_u32(read_usize(nation + 0x71)) == read_u32(0x9cfa10) {
            let european = if read_i32(club + 0x1db) != -1 {
                true
            } else {
                let b = exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1);
                b != 0 && read_i32(b + 0x1db) != -1
            };
            if european {
                t.separator(filter_menu);
                squad_text(0xde1f64, 0x98eb30); // " European Squad"
                t.menu_item(filter_menu, f & 0x8000 != 0, 8, if f & 0x8000 != 0 { 0x4000 } else { 0x8000 });
            }
        }
    }
    if exe::database_club_has_reserve_squad(club) != 0 {
        if exe::database_is_national_team(club) != 0 {
            if f & 0x400 != 0 {
                f -= 0x400;
                t.filter = f;
            }
        } else if exe::human_manager_current_controls_club(club, 0, 1) != 0 {
            t.separator(filter_menu);
            squad_text(0xde1f64, if t.reserves != 0 { 0x98eb1c } else { 0x98eb08 });
            t.menu_item(filter_menu, f & 0x400 != 0, 8, if f & 0x400 != 0 { 0x200 } else { 0x400 });
        } else if f & 0x400 != 0 {
            f -= 0x400;
            t.filter = f;
        }
    }
    let manager_text: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    t.top += 0x19;
    manager_text(sm, 0x6e, t.top as usize, 0x30c, (t.top + 0x23) as usize, 2, 0, 0xc, 3, squad_colour(0xaebdec) as usize, t.title.as_ptr() as usize, 1);
    t.top += 0x28;
    if exe::human_manager_current_controls_club(club, 0, 1) != 0 && exe::database_is_national_team(club) != 0 {
        let count = exe::database_count_club_squad(club, if t.reserves == 0 { 4 } else { 8 }, 0) as i8 as i32;
        let m = exe::nationalteams_get_max_squad_size(read_usize(G_NATIONAL_TEAMS), club, t.reserves) as i8 as i32;
        let fmt: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        if count == 1 {
            fmt(0xde1f64, 0x98ead4, m);
        } else {
            fmt(0xde1f64, 0x98ea88, count, m);
        }
        manager_text(sm, 0x6e, (t.bottom - 0x14) as usize, 0x30c, t.bottom as usize, 2, 0, 0xc, 2, squad_colour(0xaebdec) as usize, 0xde1f64, 1);
        t.bottom -= 0x19;
    }
}

/// The squad numbers or the team sheet row, then the player list.
///
/// For a club the human runs: his squad numbers pruned (`HUMAN_MANAGER::prune_squad_numbers`); in the penalty takers view
/// (0x800) two rows of 25 numbers (1..25 at the top, 26..50 at the bottom; the top moves down and the bottom up 0x19),
/// each white (0x70) when a player holds it (`exe::human_manager_get_squad_number_player`), link 0x23 with the number;
/// otherwise the team sheet row: the active tactic's eleven positions in order (`Formation::get_slots_in_position_order`)
/// and the substitutes "SB<n>" up to 20 slots, the ones past the fixture's allowed substitutes (+0x4d & 0xf, 9 without a
/// next fixture) greyed; each slot's selected player (`exe::tactics_get_selected_player`) marks it white, available
/// (`is_player_available_for_match` with the current date as the competition, as the original) in the text colour,
/// otherwise 0xae31a8; link 0xa with the slot; the top moves down 0x19. For a club he does not run, the penalty takers
/// view is dropped and, with the last fixture's match record (`exe::match_stats_read_match_record` into 0xad9db8), its
/// team sheet is used. Then the list (`build_filtered_sorted_squad_list`, 0x4841d0) with the club, reserves, sort, style
/// 3, params 9, 0x19, 0x1a, the filter, the record's team sheet or none, the next fixture and the penalty takers bit.
pub(crate) unsafe fn club_screen_squad_tab_selection(t: &mut SquadTab) {
    t.from_record = false;
    let club = t.club;
    let human_manager = read_usize(0xb63c98);
    if exe::human_manager_current_controls_club(club, 0, 1) != 0 {
        exe::human_manager_prune_squad_numbers(human_manager);
        if t.view & 0x800 != 0 {
            covered!("tel01_3 squad numbers");
            let grid = |t: &SquadTab, area: u32, k: u32, col: u32| {
                squad_number_text(0x98806c, k as i32 + 1);
                let held = exe::human_manager_get_squad_number_player(read_usize(0xb63c98), k as i32, 0usize) != 0;
                let (flags, colour) = if held { (0x70, squad_colour(0xaebdec)) } else { (0x30, squad_colour(0xae3184)) };
                t.add_item([0x46, 0, 0, 0, 0, col, 0, flags, squad_colour(0xae3192), squad_colour(0xae3192), 0xc, 1, colour, 0xde1f64, 0, 0x23, k, area]);
            };
            let a = t.add_area([0x6e, t.top as u32, 0x30c, (t.top + 0x14) as u32, 0x19, 0, 1, 0, 1, 0, u32::MAX]);
            for k in 0..0x19 {
                grid(t, a, k, k);
            }
            t.top += 0x19;
            let b = t.add_area([0x6e, (t.bottom - 0x14) as u32, 0x30c, t.bottom as u32, 0x19, 0, 1, 0, 1, 0, u32::MAX]);
            for k in 0x19..0x32 {
                grid(t, b, k, k - 0x19);
            }
            t.bottom -= 0x19;
            t.ebp = b;
        } else {
            covered!("tel01_3 team sheet row");
            let a = t.add_area([0x6e, t.top as u32, 0x30c, (t.top + 0x14) as u32, 0x14, 0, 1, 0, 1, 0, u32::MAX]);
            t.ebp = a;
            let tactic = exe::human_manager_get_active_tactic(human_manager, 0usize);
            let mut slots = [0u8; 0xc];
            let order: unsafe extern "thiscall" fn(usize, *mut u8, u32) = core::mem::transmute(FN_FORMATION_GET_SLOTS_IN_POSITION_ORDER);
            order(tactic, slots.as_mut_ptr(), 0xb);
            let shown = if t.next_match != 0 { (read_u8(t.next_match + 0x4d) & 0xf).wrapping_add(0xb) } else { 0x14 } as i8;
            for k in 0u8..0x14 {
                if k < 0xb {
                    let pos = exe::formation_get_slot_position(tactic, slots[k as usize] as u32, 0);
                    squad_position_text(pos as u32);
                } else {
                    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
                    f(0xde1f64, 0x98ea6c, k as i32 - 0xa);
                }
                let slot = if k < 0xb { slots[k as usize] } else { k };
                let player = exe::tactics_get_selected_player(read_usize(0xde1d08), club, slot as u32, t.reserves as i32);
                let style = if (k as i8) >= shown { 0x2c } else { 0xc };
                if player != 0 {
                    let colour = if exe::is_player_available_for_match(player, t.next_match, club, G_CURRENT_DATE) != 0 { squad_colour(0xaebdec) } else { squad_colour(0xae31a8) };
                    t.add_item([0x46, 0, 0, 0, 0, k as u32, 0, 0x70, squad_colour(0xae3150), squad_colour(0xae3150), style, 1, colour, 0xde1f64, 0, 0xa,
                        slot as i8 as i32 as u32, a]);
                } else {
                    t.add_item([0x46, 0, 0, 0, 0, k as u32, 0, 0x30, squad_colour(0xae3150), squad_colour(0xae3150), style, 1, squad_colour(0xae3184),
                        0xde1f64, 0, 0xa, slot as i8 as i32 as u32, a]);
                }
            }
            t.top += 0x19;
        }
    } else {
        if t.view & 0x800 != 0 {
            t.view -= 0x800;
        }
        if t.last_match != 0 && exe::match_stats_read_match_record(read_usize(0xb6b7c0), read_u32(t.last_match + 8), 0xad9db8usize) != 0 {
            covered!("tel01_3 team sheet of the last match");
            t.from_record = true;
        }
    }
    let build: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, usize, usize, usize, u32, u32, u32) -> u32 =
        core::mem::transmute(FN_BUILD_FILTERED_SORTED_SQUAD_LIST);
    // simplification: params 9, 0x19 and 0x1a are bytes in stack slots whose upper bytes are garbage in the original
    // (the list takes them as chars)
    build(t.club, t.reserves, t.sort, 3, t.category as u32, t.p19 as u32, t.p1a as u32, t.filter, t.list.as_mut_ptr() as usize,
        if t.from_record { 0xad9db8 } else { 0 }, t.next_match, 0, 0, t.view & 0x800);
}

/// The list's area: the positions view's own, or the background, the column headings and the list of the other views.
///
/// Positions view (1): one area from the top to the bottom with 8 columns (two players per row; the 1080 exe 12, three
/// per row) and the rows that fit in font 3. Other views: the background area, then the rows that fit in font 2 and
/// their height (the height over the rows, rounded, `ftol`); the headings in an area from the top + 8, one row high, as
/// wide as the list (0x2f6 when the list has more players than rows, to leave the scroll bar), with the view's weights:
/// "Pkd" 1, "Inf" 2, "   Name" 4 (all link 0xc, the sort data) and the view's columns: contract (Squad Status or Club
/// 0x10, Basic Wage 0x20000, Contract Expiry 0x40000, Releases 0x2000000 when not a national team; a national team's
/// weights 0x12, 0x12, 0x12, 0); selection (Position 8, Form 0x80, Morale 0x100, Cond. 0x200, the fields 1 and 0x11
/// headings 0x10000000 and 0x800000); statistics (the fields of [0x98de84] or, More Stats, [0x98de90], link 0xd);
/// attributes (the table of the chosen attribute group, [0x98de9c] when none, sorted in place by qsort with 0x487720,
/// link 0xe; none chosen sets bit 0x40 in the local view); other (Nat. 0x1000, Age 0x4000, Caps 0x8000, Goals 0x10000,
/// Form, Morale, Cond.); "Value" 0x80000 in column 0xc. Then the list area below, to the bottom - 8.
pub(crate) unsafe fn club_screen_squad_tab_header(t: &mut SquadTab, w: &mut SquadWeights) -> u32 {
    let v = t.view;
    if v & 1 != 0 {
        covered!("tel01_3 positions view");
        let rows = ((t.bottom - t.top - 0x10) / exe::font_get_height(3)) as u8 as u32;
        // the 1080 exe puts its own weights (mov edx, imm32) and column count in this routine
        let weights = if read_u8(0x459a22) == 0xba { read_u32(0x459a23) } else { w.two.as_ptr() as u32 };
        let cols = read_u8(0x459a3b) as u32;
        // simplification: the row count is a byte in a stack slot whose upper bytes are garbage in the original
        return t.add_area([0x6e, t.top as u32, 0x30c, t.bottom as u32, cols, weights, rows, 0, 2, 0, u32::MAX]);
    }
    let (top, bottom) = (t.top, t.bottom);
    t.add_area([0x6e, top as u32, 0x30c, bottom as u32, 1, 0, 1, 0, 2, 0, u32::MAX]);
    let height = bottom - top - 0x10;
    let rows = (height / exe::font_get_height(2)) as u8;
    let row_h = ftol(height as f64 / rows as u32 as f64 + read_f64(rdata::F64_0_5));
    let count = read_i16(t.list.as_ptr() as usize) as i32;
    let x2 = if count > rows as i32 - 1 { 0x2f6 } else { 0x30c };
    let weights: usize;
    let mut area;
    let head = |t: &SquadTab, weights: usize| t.add_area([0x6e, (top + 8) as u32, x2, (row_h + top + 7) as u32, 0xd, weights as u32, 1, 0, 1, 0, u32::MAX]);
    if v & 2 != 0 {
        covered!("tel01_3 contract view");
        if exe::database_is_national_team(t.club) != 0 {
            w.contract[3] = 0x12;
            w.contract[4] = 0x12;
            w.contract[5] = 0x12;
            w.contract[6] = 0;
        }
        weights = w.contract.as_ptr() as usize;
        area = head(t, weights);
        t.heading(area, 0, 0xc, 0x98ea68, 0xc, 1);
        t.heading(area, 1, 0xc, 0x98ea64, 0xc, 2);
        t.heading(area, 2, 1, 0x98ea5c, 0xc, 4);
        let label = if exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), t.club) == 0 { 0x98ea4c } else { 0x98ea44 };
        t.heading(area, 3, 1, label, 0xc, 0x10);
        t.heading(area, 4, 1, 0x98ea34, 0xc, 0x20000);
        t.heading(area, 5, 1, 0x98ea20, 0xc, 0x40000);
        if exe::database_is_national_team(t.club) == 0 {
            t.heading(area, 6, 1, 0x98ea14, 0xc, 0x2000000);
        }
        t.heading(area, 0xc, 0xc, 0x98ea0c, 0xc, 0x80000);
    } else if v & 0x1000 != 0 {
        covered!("tel01_3 selection view");
        weights = w.selection.as_ptr() as usize;
        area = head(t, weights);
        t.heading(area, 0, 0xc, 0x98ea68, 0xc, 1);
        t.heading(area, 1, 0xc, 0x98ea64, 0xc, 2);
        t.heading(area, 2, 1, 0x98ea5c, 0xc, 4);
        t.heading(area, 3, 1, 0x98ea00, 0xc, 8);
        t.heading(area, 4, 0xc, 0x98e9f8, 0xc, 0x80);
        t.heading(area, 5, 0xc, 0x98e9f0, 0xc, 0x100);
        t.heading(area, 6, 0xc, 0x98e9e8, 0xc, 0x200);
        let label: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(FN_PLAYER_STATS_GET_FIELD_LABEL);
        label(read_usize(0xb74350), 1, 0xde1f64, 0x7d0);
        t.heading(area, 7, 0xc, 0, 0xc, 0x10000000);
        label(read_usize(0xb74350), 0x11, 0xde1f64, 0x7d0);
        t.heading(area, 8, 0xc, 0, 0xc, 0x800000);
        t.heading(area, 0xc, 0xc, 0x98ea0c, 0xc, 0x80000);
    } else if v & 4 != 0 || v & 8 != 0 {
        covered!("tel01_3 stats view");
        weights = w.stats.as_ptr() as usize;
        area = head(t, weights);
        t.heading(area, 0, 0xc, 0x98ea68, 0xc, 1);
        t.heading(area, 1, 0xc, 0x98ea64, 0xc, 2);
        t.heading(area, 2, 1, 0x98ea5c, 0xc, 4);
        let table = if v & 4 != 0 { 0x98de84 } else { 0x98de90 };
        let label: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(FN_PLAYER_STATS_GET_FIELD_LABEL);
        let mut k = 0usize;
        while read_u8(table + k) != 0xff {
            let field = read_u8(table + k);
            label(read_usize(0xb74350), field as u32, 0xde1f64, 0x7d0);
            t.heading(area, k as i32 + 3, 0xc, 0, 0xd, field as i8 as i32 as u32);
            k += 1;
        }
        t.heading(area, 0xc, 0xc, 0x98ea0c, 0xc, 0x80000);
    } else if v & 0x10 != 0 {
        covered!("tel01_3 attributes view");
        weights = w.attributes.as_ptr() as usize;
        area = head(t, weights);
        t.heading(area, 0, 0xc, 0x98ea68, 0xc, 1);
        t.heading(area, 1, 0xc, 0x98ea64, 0xc, 2);
        t.heading(area, 2, 1, 0x98ea5c, 0xc, 4);
        t.attributes = if t.view & 0x40 != 0 {
            0x98de9c
        } else if t.view & 0x80 != 0 {
            0x98dea8
        } else if t.view & 0x100 != 0 {
            0x98deb4
        } else if t.view & 0x200 != 0 {
            0x98dec0
        } else if t.view & 0x400 != 0 {
            0x98decc
        } else {
            t.view |= 0x40;
            0x98de9c
        };
        exe::crt_qsort(t.attributes, 9, 1, 0x487720usize);
        let mut k = 0usize;
        while read_u8(t.attributes + k) != 0xff {
            let attr = read_u8(t.attributes + k);
            exe::database_get_player_attribute_name(attr as u32, 0xde1f64, 0x7d0, 1);
            t.heading(area, k as i32 + 3, 0xc, 0, 0xe, attr as i8 as i32 as u32);
            k += 1;
        }
        t.heading(area, 0xc, 0xc, 0x98ea0c, 0xc, 0x80000);
    } else if v & 0x20 != 0 {
        covered!("tel01_3 other view");
        weights = w.other.as_ptr() as usize;
        area = head(t, weights);
        t.heading(area, 0, 0xc, 0x98ea68, 0xc, 1);
        t.heading(area, 1, 0xc, 0x98ea64, 0xc, 2);
        t.heading(area, 2, 1, 0x98ea5c, 0xc, 4);
        for (col, label, data) in [(3, 0x98e9e0usize, 0x1000u32), (4, 0x98e9dc, 0x4000), (5, 0x98e9d4, 0x8000), (6, 0x98e9cc, 0x10000),
                                   (7, 0x98e9f8, 0x80), (8, 0x98e9f0, 0x100), (9, 0x98e9e8, 0x200)] {
            t.heading(area, col, 0xc, label, 0xc, data);
        }
        t.heading(area, 0xc, 0xc, 0x98ea0c, 0xc, 0x80000);
    } else {
        // no view of its own (the penalty takers alone): no list area; the rows go to the last area made (ebp)
        return t.ebp;
    }
    let _ = area;
    // the row count dword keeps the filter flags' upper bytes (ebx held them; only bl was the count)
    let rows_word = (t.filter & 0xffff_ff00) | rows.wrapping_sub(1) as u32;
    area = t.add_area([0x6e, (row_h + top + 8) as u32, 0x30c, (bottom - 8) as u32, 0xd, weights as u32, rows_word, 0, 1, 0, u32::MAX]);
    area
}

/// The squad tab's rows, one per player of the list with a player record (`staff::PLAYER`), in the area `area`.
///
/// Each player's training is caught up first (`exe::training_manager_update_player_training`). Column 0 (the
/// number): for a club the human runs, in the penalty takers view his squad number
/// (`HUMAN_MANAGER::get_player_squad_number`, link 0x23) or an empty item (link 0x24 with the player); otherwise his
/// selection slot (`exe::tactics_find_selection_slot`): the position of the active tactic's slot
/// (`formation_get_position_abbreviation`) or "SB<n>", link 0xa with the slot, or an empty item (link 0xb with the
/// player); for a club he does not run, the slot in the last match's team sheet (0xad9f48, the club's side by
/// [0xad9db8]: its position word at 0xad9f0c or "SB<n>") as a plain text. Column 1, his status
/// (`get_player_status_code`, 0x4849f0) as an item (flags 0x202, link and data by the status, for a national team other
/// links): Wpm, Cup, Ine (0x7e0 or 0x7db), international duty (with the duty club, link 0x7d5; without, an item 0x201
/// with no link; the text is whatever 0xde1f64 holds, as the original), Sus (0x7de or 0x7d9), Inj (0x7dd or 0x7d8),
/// Abs, Fgn, Tir, Yel (the colour of `exe::graphics_rgb_to_pixel`(0xb4, 0xb4, 0, 0)), Bid, Trn, Wnt, Sct, Req, Lst,
/// Loa, Ctr (0x7da), Ret, Bos, Unh, Fut (this last through `screenmanager_add_item`). Column 2, his name
/// (`exe::database_format_staff_name`, style 4 in the positions view, else 1; with "*" when he is one of the three best
/// known players of a club no human controls, `exe::human_manager_is_human_club`) as a button to him (link 0x7dc for a
/// national team, else 0x7d7), its colour: his club or nation run by the human (white), a person made in the game (the
/// greyed colour), on loan (0xae3152 at the club, else 0xaebdde), starred (0xae31a2), else 0xae3184. Then by the view:
/// in the positions view column 3 by the sort (squad number, form, morale, condition, nation or club, age, caps, goals,
/// basic wage, contract expiry, protection, value, goals, conceded, assists, rating, else the positions), two players
/// per row (three in the 1080 exe); in the others the view's columns and the value in column 0xc (a button to him for
/// a player of a club the human runs), one per row.
pub(crate) unsafe fn club_screen_squad_tab_rows(t: &mut SquadTab, area: u32) {
    let l = t.list.as_mut_ptr() as usize;
    write_u32(l + 0xa, read_u32(l + 2));
    let (mut row, mut col) = (0i32, 0i32);
    let club = t.club;
    let positions_view = t.view & 1 != 0;
    let last_base = read_u8(0x45c018) as i8 as i32; // 4, or 8 in the 1080 exe (three players per row)
    let mut buf = [0u8; 0x7d0];
    let b = buf.as_mut_ptr() as usize;
    if exe::llist_get_current(l) == 0 {
        return;
    }
    loop {
        let player = Staff(read_usize(exe::llist_get_next(l)));
        let pdata = player.player().addr();
        if pdata != 0 {
            exe::training_manager_update_player_training(read_usize(0xde1f44), player, 0);
            club_screen_squad_tab_number(t, area, player.addr(), col, row);
            club_screen_squad_tab_status(t, area, player.addr(), col, row);
            let contract_club = if squad_is_new_person(player.addr()) {
                0
            } else {
                exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, club)
            };
            let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
            let style = if positions_view { 3 } else { 2 };
            let mut starred = false;
            let name_style = if positions_view { 4 } else { 1 };
            if exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 && t.best.contains(&read_u32(player.addr())) {
                starred = true;
                exe::database_format_staff_name(player, b, 0x7d0, name_style);
                squad_string_text(0x98e8dc, b); // "%s*"
            } else {
                exe::database_format_staff_name(player, 0xde1f64usize, 0x7d0, name_style);
            }
            let national = exe::database_is_national_team(club) != 0;
            let club_job = player.club_job().addr();
            let national_job = player.national_job().addr();
            let (colour, link) = if national && club_job != 0 && exe::human_manager_current_controls_club(club_job, 0, 1) != 0 {
                (squad_colour(0xaebdec), 0x7dc)
            } else if !national && national_job != 0 && exe::human_manager_current_controls_club(national_job, 0, 1) != 0 {
                (squad_colour(0xaebdec), 0x7d7)
            } else if national && squad_is_new_person(player.addr()) {
                (squad_colour(0xae317e), 0x7dc)
            } else if !national && squad_is_new_person(player.addr()) {
                (squad_colour(0xae317e), 0x7d7)
            } else if national {
                (if starred { squad_colour(0xae31a2) } else { squad_colour(0xae3184) }, 0x7dc)
            } else if (contract_club != 0 && read_u8(contract_club + 0x35) & 0x3f == 5)
                || exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), player) != 0 {
                (if club_job == club { squad_colour(0xae3152) } else { squad_colour(0xaebdde) }, 0x7d7)
            } else {
                (if starred { squad_colour(0xae31a2) } else { squad_colour(0xae3184) }, 0x7d7)
            };
            t.button(area, col + 2, row, 1, 0, 1, style, colour, 0xde1f64, link, player.addr());
            let injury = exe::injury_get_player_injury_status(read_usize(G_INJURY), player);
            if positions_view {
                club_screen_squad_tab_sort_cell(t, area, player.addr(), col, row, injury, contract_club, contract, &mut buf);
                col += 4;
                if col > last_base {
                    row += 1;
                    col = 0;
                }
            } else {
                club_screen_squad_tab_view_cells(t, area, player.addr(), col, row, injury, contract_club, contract, &mut buf);
                squad_value_text(player.addr());
                if !squad_is_new_person(player.addr()) && club_job != 0 && exe::human_manager_current_controls_club(club_job, 0, 1) != 0 {
                    t.button(area, col + 0xc, row, 0x30, squad_colour(0xae3192), 0xc, 1, squad_colour(0xae3184), 0xde1f64, 0xf, player.addr());
                } else {
                    t.text(area, col + 0xc, row, 0x30, squad_colour(0xae3192), 0xc, 1, squad_colour(0xae3184), 0xde1f64);
                }
                row += 1;
            }
        }
        if exe::llist_get_current(l) == 0 {
            break;
        }
    }
}

/// strcpy of the original (rep movs): `src` with its terminator into `dst`.
pub(crate) unsafe fn next_match_copy_text(dst: usize, src: usize) {
    let mut k = 0;
    loop {
        let c = read_u8(src + k);
        write_u8(dst + k, c);
        if c == 0 {
            break;
        }
        k += 1;
    }
}

/// strcpy(dst, src) the way the original inlines it (repne scasb + rep movs, the terminator included).
pub(crate) unsafe fn fixtures_copy_text(dst: usize, src: usize) {
    let mut k = 0usize;
    loop {
        let b = read_u8(src + k);
        write_u8(dst + k, b);
        if b == 0 {
            break;
        }
        k += 1;
    }
}

/// D1 = D1 + 1 day (`Date::add_days` into a temporary, then `Date::operator=`).
pub(crate) unsafe fn fixtures_next_day(d1: usize, out: usize, tmp: usize) {
    let t = exe::date_add_days(d1, tmp, 1);
    exe::date_operator_3d(d1, out, t);
}

/// Column 0 of a friendly/tour/tournament row: the long date (`Date::get_long_date_string`, 0x7d0, 0xff), dark blue
/// background.
pub(crate) unsafe fn fixtures_date_cell(d1: usize, lst: usize, row: usize) {
    let label: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    exe::date_get_long_date_string(d1, FIXTURES_TEXT, 0x7d0, 0xff);
    label(SCREEN_MANAGER, lst, 0, row, 0x30, colour_word(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), FIXTURES_TEXT);
}

/// Column 5 of a friendly/tour/tournament row: "---".
pub(crate) unsafe fn fixtures_no_result_cell(lst: usize, row: usize) {
    let label: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    fixtures_copy_text(FIXTURES_TEXT, FIXTURES_NO_RESULT_TEXT);
    label(SCREEN_MANAGER, lst, 5, row, 0x30, colour_word(FIXTURES_COLOUR_RESULT_BG), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), FIXTURES_TEXT);
}

/// The inlined strcmp of the original, as equality (byte by byte up to the first difference or the end).
pub(crate) unsafe fn badge_names_equal(a: usize, b: usize) -> bool {
    let mut k = 0;
    loop {
        let c = read_u8(a + k);
        if c != read_u8(b + k) {
            return false;
        }
        if c == 0 {
            return true;
        }
        k += 1;
    }
}

/// The General Info view of `club_screen_draw_info_tab` (0x460c02..0x4619ce): see its header.
pub(crate) unsafe fn info_tab_general_view(f: &mut InfoTabFrame) {
    covered!("tel02_2 info tab general");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let label: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let link: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let collect: unsafe extern "C" fn(usize, usize, i32) -> u8 = core::mem::transmute(INFO_TAB_FN_COLLECT_NON_PLAYING_STAFF);
    let sm = SCREEN_MANAGER;
    let c = colour_word;
    let club = Club(f.club);
    let y = f.y;
    let t = INFO_TAB_TEXT;

    let table = area(current_human_display(), 0x6e, y as usize, 0x30c, (y + 0x8e) as usize, 2, f.columns.as_mut_ptr().add(2) as usize, 6, 0, 2, 0, usize::MAX);
    if find_club_badge_picture_path(club.addr(), f.path.as_mut_ptr() as usize) != 0 {
        let pic = area(current_human_display(), 0x27e, y as usize, 0x30c, (y + 0x8e) as usize, 1, 0, 1, 0, 1, 0, usize::MAX);
        item(current_human_display(), 0x400, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0xc, 1, 0, f.path.as_ptr() as usize, 0, 0, 0, pic);
    }
    // a row's label (pale grey) and value (yellow) in the shared text
    let key = |row: usize| label(sm, table, 0, row, 1, c(G_GUI_COLOUR_BLACK), 1, 3, c(G_GUI_COLOUR_PALE_GREY), t);
    let value = |row: usize| label(sm, table, 1, row, 1, c(G_GUI_COLOUR_BLACK), 1, 2, c(G_GUI_COLOUR_YELLOW), t);
    let national = || exe::database_is_national_team(club) != 0;
    let all_star = || exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), club) != 0;

    // row 0: Continent or Nation
    let nation = club.nation();
    if national() {
        text(t, 0x990568); // "  Continent"
        key(0);
        let continent = nation.continent();
        if continent != 0 {
            label(sm, table, 1, 0, 1, c(G_GUI_COLOUR_BLACK), 1, 2, c(G_GUI_COLOUR_YELLOW), continent + 4);
        } else {
            text(t, 0x990544, INFO_TAB_NO_COMMENT); // "Unknown<%s - COMMENT - continent>"
            value(0);
        }
    } else {
        text(t, 0x990538); // "  Nation"
        key(0);
        let id = read_u32(club.addr());
        if id == read_u32(INFO_TAB_KEY_SWANSEA_CITY) || id == read_u32(INFO_TAB_KEY_CARDIFF_CITY) || id == read_u32(INFO_TAB_KEY_WREXHAM) {
            text(t, 0x9904f4, INFO_TAB_NO_COMMENT); // "Wales<%s - COMMENT - Country Name>"
            value(0);
        } else if !nation.is_null() {
            let team = exe::database_get_national_team(nation);
            link(sm, table, 1, 0, 1, c(G_GUI_COLOUR_BLACK), 1, 2, c(G_GUI_COLOUR_YELLOW), nation.addr() + 4, 0x7d5, team);
        } else {
            text(t, 0x990518, INFO_TAB_NO_COMMENT); // "Unknown<%s - COMMENT - nation>"
            value(0);
        }
    }

    // row 1: Status
    text(t, 0x9904c0, INFO_TAB_NO_COMMENT); // "  Status<%s - COMMENT - club professional status>"
    key(1);
    if all_star() {
        copy_c_string(t, 0x98ba0c); // "-"
    } else {
        match club.professional_status() {
            1 => { text(t, 0x9904b0); } // "Professional"
            2 => { text(t, 0x99049c); } // "Semi-Professional"
            3 => { text(t, 0x990494); } // "Amateur"
            _ => { text(t, 0x990460, INFO_TAB_NO_COMMENT); } // "Unknown<%s - COMMENT - club professional status>"
        }
    }
    value(1);

    // row 2: Finances (each band test asks the finance again, as the original)
    text(t, 0x990454); // "  Finances"
    key(2);
    let fin = f.finance;
    let band = || exe::finance_get_wealth_band(fin) as i8;
    if all_star() {
        copy_c_string(t, 0x98ba0c); // "-"
    } else if band() == 2 {
        text(t, 0x99044c); // "Rich"
    } else if band() == 1 {
        text(t, 0x990444); // "Secure"
    } else if band() == 0 {
        text(t, 0x990428, INFO_TAB_NO_COMMENT); // "Ok<%s - COMMENT - finances>"
    } else if band() != -1 && read_u8(fin + 0x165) == 1 {
        if band() == -2 {
            text(t, 0x99041c); // "Bankrupt"
        }
        // tel02_2: any other band keeps the text as it is ("  Finances")
    } else {
        text(t, 0x9903f8, INFO_TAB_NO_COMMENT); // "Insecure<%s - COMMENT - finances>"
    }
    value(2);

    // row 3: Stadium
    text(t, 0x9903ec); // "  Stadium"
    key(3);
    let stadium = club.stadium().addr();
    let sname = stadium + stadium::NAME;
    let sg = || read_i8(stadium + stadium::GENDER_NAME) as i32;
    let unknown_stadium = || {
        if national() {
            text(t, 0x99029c); // "Various"
        } else {
            text(t, 0x99027c, INFO_TAB_NO_COMMENT); // "Unknown<%s - COMMENT - stadium>"
        }
    };
    if club.own_stadium() == 0 {
        let mut owner = 0usize;
        let clubs = read_usize(G_CLUB_TABLE);
        for k in 0..read_i32(G_CLUB_COUNT).max(0) as usize {
            let other = Club(clubs + k * club::SIZE);
            if other != club && other.stadium().addr() == stadium && other.own_stadium() != 0 {
                owner = other.addr();
                break;
            }
        }
        if stadium == 0 {
            unknown_stadium();
        } else if owner != 0 {
            let city = read_usize(stadium + stadium::CITY);
            let og = read_i8(owner + club::SHORT_GENDER_NAME) as i32;
            if city != 0 {
                let cg = read_i8(city + city::GENDER_NAME) as i32;
                text(t, 0x990348, sg(), sg(), cg, cg, og, og, sname, city + 4, owner + club::SHORT_NAME); // ground owned by, with the city
            } else {
                text(t, 0x9902ec, sg(), sg(), og, og, sname, owner + club::SHORT_NAME); // "{}<%s - stadium>{} (Ground owned by {}<..>{})"
            }
        } else {
            let city = read_usize(stadium + stadium::CITY);
            if city != 0 {
                let cg = read_i8(city + city::GENDER_NAME) as i32;
                text(t, 0x9903b4, sg(), sg(), cg, cg, sname, city + 4); // "{}<%s - stadium>{} (Rented Stadium), {}<%s - city>{}"
            } else {
                text(t, 0x9902c8, sg(), sg(), sname); // "{}<%s - stadium>{} (Rented Stadium)"
            }
        }
    } else if stadium == 0 {
        unknown_stadium();
    } else {
        let city = read_usize(stadium + stadium::CITY);
        if city != 0 {
            let cg = read_i8(city + city::GENDER_NAME) as i32;
            text(t, 0x9902a4, sg(), sg(), cg, cg, sname, city + 4); // "{}<%s - stadium>{}, {}<%s - city>{}"
        } else {
            copy_c_string(t, sname);
        }
    }
    value(3);

    // row 4: Facilities
    text(t, 0x99026c); // "  Facilities"
    key(4);
    let cap = if stadium != 0 { read_i32(stadium + stadium::CAPACITY) } else { 0 };
    if cap == 0 {
        copy_c_string(t, 0x98ba0c); // "-"
    } else {
        let seats = read_i32(stadium + stadium::SEATING_CAPACITY);
        let roof = read_u8(stadium + stadium::COVERED) != 0;
        let heating = read_u8(stadium + stadium::UNDER_SOIL_HEATING) != 0;
        // (roof and heating, roof, heating, neither)
        let pick = |fs: [usize; 4]| if roof && heating { fs[0] } else if roof { fs[1] } else if heating { fs[2] } else { fs[3] };
        if seats >= cap {
            text(t, pick([0x990220, 0x9901ec, 0x9901b8, 0x990198]), cap); // "... all seater ..."
        } else if seats != 0 {
            text(t, pick([0x990138, 0x9900f0, 0x9900a4, 0x990070]), cap, seats); // "... (<%ld> seated) ..."
        } else {
            text(t, pick([0x990030, 0x990008, 0x98ffdc, 0x98ffc8]), cap); // "<%ld - capacity> ..."
        }
    }
    value(4);

    // row 5: Training Ground
    if !national() {
        text(t, 0x98ffb4); // "  Training Ground"
        key(5);
        let training = club.training() as i32;
        if training > 0xf {
            text(t, 0x98ff90); // "Top facilities plus youth academy"
        } else {
            let ratio = club.reputation() as i32 / training.max(1);
            let fmt = if ratio <= 400 {
                0x98ff80 // "Good facilities"
            } else if ratio <= 600 {
                0x98ff6c // "Adequate facilities"
            } else if ratio <= 800 {
                0x98ff54 // "Need some investment"
            } else if ratio <= 1000 {
                0x98ff3c // "Need major investment"
            } else {
                0x98ff28 // "Awful facilities"
            };
            text(t, fmt);
        }
        value(5);
    }

    // the non-playing staff list
    let y2 = f.y + 0x93;
    let rows = ((f.bottom - y2 - 0x10) / exe::font_get_height(3)) as u8;
    f.general = (f.general & !0xff) | rows as u32; // [E+0x24]: the row count byte written over view & 1
    let list = area(current_human_display(), 0x6e, y2 as usize, 0x30c, f.bottom as usize, 2, f.columns.as_mut_ptr() as usize, f.general as usize, 0, 2, 0, usize::MAX);
    text(t, 0x98ff14); // "Non-Playing Staff"
    let w = exe::font_text_width(3usize, t as *const u8) + 0x73;
    let orange = c(INFO_TAB_COLOUR_ORANGE);
    item(current_human_display(), 1, 0x73, 0, w as usize, 0, 0, 0, 1, 0, 0, 1, 3, orange, t, 0, 0, 0, list);
    item(current_human_display(), 1, 0x73, 0, w as usize, 0, 0, 1, 0xc400, orange, 0, 0, 0, 0, 0, 0, 0, 0, list);
    let count = collect(club.addr(), f.staff.as_mut_ptr() as usize, 0x15) as i8 as i32;
    if count == 0 {
        text(t, 0x98fee8, INFO_TAB_NO_COMMENT); // "  Unknown<%s - COMMENT - non-playing staff>"
        label(sm, list, 0, 2, 1, c(G_GUI_COLOUR_BLACK), 1, 3, c(G_GUI_COLOUR_PALE_GREY), t);
    } else {
        write_u8(INFO_TAB_STAFF_SORTED_FLAG, 1);
        let sorter = if national() { INFO_TAB_FN_SORT_NATIONAL_STAFF } else { INFO_TAB_FN_SORT_CLUB_STAFF };
        exe::crt_qsort(f.staff.as_mut_ptr() as usize, count as u32, 4, sorter);
        let mut row = 1usize;
        let mut previous = 0usize;
        for k in 0..count.max(0) as usize {
            let p = Staff(f.staff[k]);
            if p.addr() == previous {
                continue; // tel02_2 a person repeated right after himself is shown once
            }
            previous = p.addr();
            row += 1;
            let name = f.path.as_mut_ptr() as usize;
            exe::database_format_staff_name(p, name, 0xc8, 1);
            format(t, 0x987ff0, name); // "  %s"
            let job = p.job_for_club();
            let regen = read_u8(read_usize(G_STAFF_STATE_TABLE) + read_i32(p.addr()) as usize * staff_state::SIZE + staff_state::STATUS_FLAGS) & 0x40 != 0;
            let nat = national();
            if job == INFO_TAB_JOB_PLAIN && !regen {
                label(sm, list, 0, row, 1, c(G_GUI_COLOUR_BLACK), 1, 3, c(G_GUI_COLOUR_PALE_GREY), t);
            } else {
                link(sm, list, 0, row, 1, c(G_GUI_COLOUR_BLACK), 1, 3, c(G_GUI_COLOUR_PALE_GREY), t, if nat { 0x7dc } else { 0x7d7 }, p.addr());
            }
            let job_byte = if nat { p.job_for_nation() } else { p.job_for_club() };
            let job_name = exe::database_get_job_name(job_byte as i8 as u32, 0, 1, core::ptr::null_mut());
            label(sm, list, 1, row, 1, c(G_GUI_COLOUR_BLACK), 1, 2, c(G_GUI_COLOUR_YELLOW), job_name);
        }
    }
    let scroll = exe::screenmanager_get_screen_param(sm, 0x1c);
    if list as i16 != -1 {
        exe::area_set_scroll_row(current_human_display() + 4 + (list as i16 as isize * 0xbf1) as usize, scroll);
    }
    exe::screenmanager_set_screen_param(sm, 0x1d, list as i16 as isize as usize, 0);
}

/// Draws the Stats view of the club info tab into a two-column grid from `top` to `bottom`.
///
/// The grid (`Display::add_area` on the current human's display: x 0x6e..0x30c, `top`..`bottom`, 2 columns, as many
/// rows as font 2 fits, `(bottom − top) / font_get_height(2)` kept as a byte) gets one label (pale grey, column 0) and
/// one value (yellow, column 1) per row; the numbers come from `club_screen_collect_squad_stats(club, reserves, ..)`.
/// Rows: 0 Number Of Players, 1 Number Of Players Injured (both "%d"), 2 Average Age - First Team and 3 Average Age -
/// Squad ("%4.2f", "-" when exactly 0.0). Unless the club is a national team (`database_is_national_team`): 4 Total
/// Wage Bill - First Team, 5 Total Wage Bill - Squad, 6 Average Wage, 7 Highest Wage and 8 Lowest Wage, each in the
/// Config's wage period (per week when +4 has 0x200000, per month = ×4 when 0x400000, otherwise per year = ×52; the
/// label says p/w, p/m or p/a) formatted as money (`Cash::get_display_amount` with option 8 per week/month and 4 per
/// year, `cash_format_money`); the highest and lowest wage rows also append " - " and the player's name
/// (`database_format_staff_name`) as a name button (id 0x7d7, the player as value), or "-" without a player; their
/// option is the one the Average Wage row used. Then Oldest Player and Youngest Player ("%d - " with the age read as a
/// signed byte, and the name button, or "-"). For a national team: Total Number Of Caps ("%d"), Highest Capped Player
/// and Lowest Capped Player ("%d - " with the caps as an unsigned byte, and the name button, or "-"), and the view ends.
/// For a club: Highest Valued Player (the value as money with option 6, " - " and the name button, or "-"), Number Of
/// Current International Players and Number Of Current Under 21 Players (the label translated when the database
/// names language `G_DATABASE_NAMES_LANGUAGE` is set, otherwise the untranslated "  No. Of Current ..." copied as is),
/// Number Of Foreign Players, and Number Of Non-EU Players only when the club's nation is an EEC member
/// (`database_club_nation_is_eec`). Labels go through `langlib_sprintf`, numbers through `sprintf`.
///
/// Frame of the original at the entry 0x4619df (E = esp there): `top` = esi (the y below the view buttons, also in
/// the top argument slot [E+0x2dc]); `bottom` = the bottom argument [E+0x2e0]; `club` = edi and [E+0x1c] (screen
/// param 0; the original passes edi to the collector and [E+0x1c] to the other calls: the same value when the view is
/// entered from the header, i.e. when the General Info view bit is not also set); `reserves` = [E+0x30] (screen param
/// 1). The row count byte overwrites the low byte of [E+0x24] (the General Info flag, `view & 1`, zero here) and the
/// grid gets that dword; the money goes through the Cash object at [E+0x2c] and the option byte through the low byte of
/// [E+0x14] (the copy keeps both in locals: nothing reads them after the view, which leaves through 0x467166).
/// Original: a piece of `club_screen_draw_info_tab` (0x460590, club_screens.cpp, cdecl(top, bottom)), 0x4619df..0x462a7f.
pub(crate) unsafe fn info_tab_stats_view(top: i32, bottom: i32, club: usize, reserves: usize) {
    covered!("tel02_2 stats view");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let collect: unsafe extern "C" fn(usize, usize, usize) = core::mem::transmute(STATS_VIEW_FN_COLLECT_SQUAD_STATS);
    let is_eec: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(STATS_VIEW_FN_CLUB_NATION_IS_EEC);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let button: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let label: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let sm = SCREEN_MANAGER;

    let h = exe::font_get_height(2);
    let rows = ((bottom - top) / h) as u8;
    let area = add_area(current_human_display(), 0x6e, top as usize, 0x30c, bottom as usize, 2, 0, rows as usize, 0, 2, 0, usize::MAX);
    let mut stats = [0u8; squad_stats::SIZE];
    let s = stats.as_mut_ptr() as usize;
    collect(club, reserves, s);

    let black = || colour_word(G_GUI_COLOUR_BLACK);
    // the label of row `row` (already in the shared text): column 0, pale grey
    let put_label = |row: usize| {
        label(sm, area, 0, row, 1, black(), 1, 2, colour_word(G_GUI_COLOUR_PALE_GREY), STATS_VIEW_TEXT);
    };
    // a value (text at `t`) in column 1, yellow
    let put_value = |row: usize, t: usize| {
        label(sm, area, 1, row, 1, black(), 1, 2, colour_word(G_GUI_COLOUR_YELLOW), t);
    };
    // the shared text in column 1 as the name button of `person`
    let put_person = |row: usize, person: usize| {
        button(sm, area, 1, row, 1, black(), 1, 2, colour_word(G_GUI_COLOUR_YELLOW), STATS_VIEW_TEXT, STATS_VIEW_PERSON_BUTTON, person);
    };
    let money = |amount: u32, option: u32| {
        let cash = amount;
        let shown = exe::cash_get_display_amount(&cash, option);
        exe::cash_format_money(STATS_VIEW_TEXT, 0x7d0, shown, option);
    };
    let name_after = |person: usize| {
        exe::database_format_staff_name(person, stats_view_text_end(), 0x7d0, 1);
    };
    // "%d" of a small count in column 1
    let count_row = |row: usize, caption: usize, n: i32| {
        text(STATS_VIEW_TEXT, caption);
        put_label(row);
        format(STATS_VIEW_TEXT, 0x98806c, n); // "%d"
        put_value(row, STATS_VIEW_TEXT);
    };
    // "%4.2f", or "-" when exactly 0.0 (fcomp + C3: NaN also gives "-")
    let age_row = |row: usize, caption: usize, age: f32| {
        text(STATS_VIEW_TEXT, caption);
        put_label(row);
        let zero = read_f32(rdata::F32_0);
        if !(age == zero || age.is_nan()) {
            format(STATS_VIEW_TEXT, 0x98fe94, age as f64); // "%4.2f"
        } else {
            format(STATS_VIEW_TEXT, STATS_VIEW_DASH);
        }
        put_value(row, STATS_VIEW_TEXT);
    };
    // a wage row in the Config's period: (amount, option); the captions are (p/w, p/m, p/a)
    let period = |captions: (usize, usize, usize), weekly: i32| -> (u32, u32) {
        let flags = read_u32(read_usize(G_CONFIG) + STATS_VIEW_CONFIG_FLAGS);
        if flags & STATS_VIEW_WAGES_PER_WEEK != 0 {
            text(STATS_VIEW_TEXT, captions.0);
            (weekly as u32, 8)
        } else if flags & STATS_VIEW_WAGES_PER_MONTH != 0 {
            text(STATS_VIEW_TEXT, captions.1);
            (weekly.wrapping_mul(4) as u32, 8)
        } else {
            text(STATS_VIEW_TEXT, captions.2);
            (weekly.wrapping_mul(52) as u32, 4)
        }
    };
    // a person row: "%d - " with `number`, the name button, or "-" without a person
    let person_row = |row: usize, caption: usize, person: usize, number: i32| {
        text(STATS_VIEW_TEXT, caption);
        put_label(row);
        if person != 0 {
            format(STATS_VIEW_TEXT, 0x98fcbc, number); // "%d - "
            name_after(person);
            put_person(row, person);
        } else {
            put_value(row, STATS_VIEW_DASH);
        }
    };

    count_row(0, 0x98fed4, read_i8(s + squad_stats::PLAYERS) as i32); // "  Number Of Players"
    count_row(1, 0x98feb8, read_i8(s + squad_stats::INJURED) as i32); // "  Number Of Players Injured"
    age_row(2, 0x98fe9c, read_f32(s + squad_stats::AVERAGE_AGE_FIRST_TEAM)); // "  Average Age - First Team"
    age_row(3, 0x98fe7c, read_f32(s + squad_stats::AVERAGE_AGE_SQUAD)); // "  Average Age - Squad"

    let national = exe::database_is_national_team(club) != 0;
    let mut row = 4usize;
    if !national {
        // "  Total Wage Bill - First Team (p/w)", "(p/m)", "(p/a)"
        let (amount, option) = period((0x98fe54, 0x98fe2c, 0x98fe04), read_i32(s + squad_stats::WAGE_BILL_FIRST_TEAM));
        put_label(4);
        money(amount, option);
        put_value(4, STATS_VIEW_TEXT);
        // "  Total Wage Bill - Squad (p/w)", "(p/m)", "(p/a)"
        let (amount, option) = period((0x98fde4, 0x98fdc4, 0x98fda4), read_i32(s + squad_stats::WAGE_BILL_SQUAD));
        put_label(5);
        money(amount, option);
        put_value(5, STATS_VIEW_TEXT);
        // "  Average Wage (p/w)", "(p/m)", "(p/a)"; its option also serves the highest and lowest wage rows
        let (amount, option) = period((0x98fd8c, 0x98fd74, 0x98fd5c), read_i32(s + squad_stats::AVERAGE_WAGE));
        put_label(6);
        money(amount, option);
        put_value(6, STATS_VIEW_TEXT);
        // "  Highest Wage (p/w)", "(p/m)", "(p/a)" and "  Lowest Wage ..."
        for (r, captions, wage, player) in [
            (7usize, (0x98fd44, 0x98fd2c, 0x98fd14), squad_stats::HIGHEST_WAGE, squad_stats::HIGHEST_WAGE_PLAYER),
            (8, (0x98fcfc, 0x98fce8, 0x98fcd4), squad_stats::LOWEST_WAGE, squad_stats::LOWEST_WAGE_PLAYER),
        ] {
            let (amount, _) = period(captions, read_i32(s + wage));
            put_label(r);
            let person = read_usize(s + player);
            if person != 0 {
                money(amount, option);
                text(stats_view_text_end(), 0x98fd10); // " - "
                name_after(person);
                put_person(r, person);
            } else {
                put_value(r, STATS_VIEW_DASH);
            }
        }
        row = 9;
    }
    let oldest = Staff(read_usize(s + squad_stats::OLDEST));
    person_row(row, 0x98fcc4, oldest.0, if oldest.0 != 0 { oldest.age() as i8 as i32 } else { 0 }); // "  Oldest Player"
    row += 1;
    let youngest = Staff(read_usize(s + squad_stats::YOUNGEST));
    person_row(row, 0x98fca8, youngest.0, if youngest.0 != 0 { youngest.age() as i8 as i32 } else { 0 }); // "  Youngest Player"
    row += 1;
    if exe::database_is_national_team(club) != 0 {
        count_row(row, 0x98fc90, read_i16(s + squad_stats::TOTAL_CAPS) as i32); // "  Total Number Of Caps"
        row += 1;
        let most = Staff(read_usize(s + squad_stats::HIGHEST_CAPPED));
        person_row(row, 0x98fc78, most.0, if most.0 != 0 { most.int_apps() as i32 } else { 0 }); // "  Highest Capped Player"
        row += 1;
        let least = Staff(read_usize(s + squad_stats::LOWEST_CAPPED));
        person_row(row, 0x98fc60, least.0, if least.0 != 0 { least.int_apps() as i32 } else { 0 }); // "  Lowest Capped Player"
        row += 1;
    }
    if exe::database_is_national_team(club) != 0 {
        return;
    }
    text(STATS_VIEW_TEXT, 0x98fc48); // "  Highest Valued Player"
    put_label(row);
    let valued = Staff(read_usize(s + squad_stats::HIGHEST_VALUED));
    if valued.0 != 0 {
        money(valued.value() as u32, 6);
        // the original's sprintf passes the value after " - " (unused by the format)
        format(stats_view_text_end(), 0x98fd10, valued.value() as u32); // " - "
        name_after(valued.0);
        put_person(row, valued.0);
    } else {
        put_value(row, STATS_VIEW_DASH);
    }
    row += 1;
    // the untranslated caption copied as is when the database names language is off
    let caption = |translated: usize, plain: usize| {
        if read_u32(G_DATABASE_NAMES_LANGUAGE) != 0 {
            text(STATS_VIEW_TEXT, translated);
        } else {
            core::ptr::copy_nonoverlapping(plain as *const u8, STATS_VIEW_TEXT as *mut u8, stats_view_c_len(plain) + 1);
        }
    };
    caption(0x98fbf4, 0x98fc20); // "  Number Of Current International Players" / "  No. Of Current International Players"
    put_label(row);
    format(STATS_VIEW_TEXT, 0x98806c, read_i8(s + squad_stats::INTERNATIONALS) as i32);
    put_value(row, STATS_VIEW_TEXT);
    row += 1;
    caption(0x98fba8, 0x98fbd0); // "  Number Of Current Under 21 Players" / "  No. Of Current Under 21 Players"
    put_label(row);
    format(STATS_VIEW_TEXT, 0x98806c, read_i8(s + squad_stats::UNDER_21S) as i32);
    put_value(row, STATS_VIEW_TEXT);
    row += 1;
    count_row(row, 0x98fb8c, read_i8(s + squad_stats::FOREIGN) as i32); // "  Number Of Foreign Players"
    row += 1;
    if is_eec(club) != 0 {
        count_row(row, 0x98fb70, read_i8(s + squad_stats::NON_EU) as i32); // "  Number Of Non-EU Players"
    }
}

/// The money views of the club screen's info tab (Finances, Income, Expenditure): the five-column table and the rows
/// of the view in `view`.
///
/// The table runs from `top` to `bottom` on the left of the screen (x 0x6e): as many rows as fit in font 3 below a
/// 0x10 margin, `(bottom − top − 0x10) / font_get_height(3)` (truncated to a byte), and the columns 2, 1, 1, 1, 1. Two
/// areas are added over the same rows: one to x 0x30c (mode 2, its index unused) and the grid to x 0x302 (mode 1) that
/// holds every cell. Row 0 holds the titles in orange: "  Item" (left), "This Month", "Last Month", "This Year" and
/// "Last Year"; the amounts are right aligned (1) when the currency symbol is not empty, centred (0x40) otherwise. An
/// orange line (`Display::add_item`, kind 1, 0x7d to 0x2fd, flags 0xc400) closes the titles.
///
/// Finances (bit 2 of `view`): the rows "  Income" (2), "  Expenditure" (3), "  Profit/(Loss)" (5), "  Balance" (7) and
/// "  Transfer Funds" (8). The totals of each column come from the club's `Finance` blocks (this month, last month,
/// this season, last season): the income adds the low dword of the players sold and the eight income dwords, the
/// expenditure the low dword of the players bought and the eight cost dwords, as 32-bit sums put in a `Cash` and
/// shown in yellow (flags 0x22, or 0x21 when the text has more than 13 characters). Profit/(Loss) is the difference
/// of the two display amounts (flags 0x20) of the column, set back into the `Cash`: green when not negative, red in
/// parentheses with its magnitude when negative. The balance (`finance::BALANCE`) is shown the same way from its low
/// dword only (negated when the i64 is negative). The transfer funds (`Finance::get_transfer_budget`(0, 0, 0, 1, 0))
/// show "None available" in red when negative, otherwise the low dword of a second call, flags 6 (0x21 past 13
/// characters), in green.
///
/// Income (bit 4) and Expenditure (otherwise) are the rows of `info_tab_income_rows` and `info_tab_expenditure_rows`.
///
/// Inputs, as the original holds them at 0x462a80: `top` = esi (the tab's top + 0x41, also frame +0x2dc), `bottom` =
/// the second argument (frame +0x2e0, edi from 0x462ab3), `view` = the low byte of frame +0x14 (param 0x1b, forced to
/// 1 by the header for a national team or a club the human does not control), `fin` = ebp (the club's record from
/// `FinanceTable::get_club_finance`), `cash` = the `Cash` at frame +0x10 (0 from the prologue).
/// Defects of the original: the income and expenditure totals take only the low dword of the players sold and bought
/// (i64) and add in 32 bits; the balance shows only its low dword (a balance past ±2^31 in cash units, or a negative
/// one past −2^32, comes out wrong); the negative text's second format says size 0x7d0 for a 0x11c-byte buffer.
/// Original: piece of `club_screen_draw_info_tab` (0x462a80..0x463bd8 and the tail 0x46715b, club_screens.cpp).
pub(crate) unsafe fn info_tab_money_views(top: i32, bottom: i32, view: u8, fin: Finance, cash: &mut u32) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let add_line: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let set_cash: unsafe extern "thiscall" fn(*mut u32, i64) = core::mem::transmute(MONEY_VIEW_FN_CASH_SET_DISPLAY);
    // 0x43f440 returns the symbol's length: right aligned with a symbol, centred without
    let align: usize = if read_u8(MONEY_VIEW_CURRENCY_SYMBOL) != 0 { 1 } else { 0x40 };
    let rows = ((bottom - top - 0x10) / exe::font_get_height(3)) as u8 as usize;
    let mut widths: [u8; 5] = [2, 1, 1, 1, 1];
    let w = widths.as_mut_ptr() as usize;
    add_area(current_human_display(), 0x6e, top as usize, 0x30c, bottom as usize, 5, w, rows, 0, 2, 0, usize::MAX);
    let area = add_area(current_human_display(), 0x6e, top as usize, 0x302, bottom as usize, 5, w, rows, 0, 1, 0, usize::MAX);
    let cell: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    text(MONEY_VIEW_TEXT, 0x98fb68); // "  Item"
    cell(SCREEN_MANAGER, area, 0, 0, 1, colour_word(G_GUI_COLOUR_BLACK), 1, 3, colour_word(MONEY_VIEW_ORANGE), MONEY_VIEW_TEXT);
    for (col, title) in [(1usize, 0x98fb5cusize), (2, 0x98fb50), (3, 0x98fb44), (4, 0x98fb38)] {
        text(MONEY_VIEW_TEXT, title); // "This Month", "Last Month", "This Year", "Last Year"
        money_view_cell(area, col, 0, align, MONEY_VIEW_ORANGE);
    }
    add_line(current_human_display(), 1, 0x7d, 0, 0x2fd, 0, 0, 1, 0xc400, colour_word(MONEY_VIEW_ORANGE), 0, 0, 0, 0, 0, 0, 0, 0, area);
    if view & 2 == 0 {
        // live here (0x463bd9 / 0x4657a4): esi = `area`, ebx = `align`, ebp = `fin`, edi = `bottom`; frame +0x10 =
        // `cash`, +0x14 = `view`, +0x1c = the club, +0x24 = `rows` (dword), +0x30 = 0, +0x5c = `widths`, +0x2dc = `top`
        if view & 4 != 0 {
            covered!("tel02_2 info tab income view");
            info_tab_income_rows(fin.0, area, align);
        } else {
            covered!("tel02_2 info tab expenditure view");
            info_tab_expenditure_rows(fin.0, area, align);
        }
        return;
    }
    covered!("tel02_2 info tab finances view");
    money_view_label(area, 2, 0x98fb2c, 0); // "  Income"
    let mut income = [0i64; 4];
    for (k, &b) in MONEY_VIEW_BLOCKS.iter().enumerate() {
        income[k] = money_view_total_cell(cash, area, k + 1, 2, align, money_view_income_total(fin, b));
    }
    money_view_label(area, 3, 0x98fb1c, 0); // "  Expenditure"
    let mut spent = [0i64; 4];
    for (k, &b) in MONEY_VIEW_BLOCKS.iter().enumerate() {
        spent[k] = money_view_total_cell(cash, area, k + 1, 3, align, money_view_expenditure_total(fin, b));
    }
    money_view_label(area, 5, 0x98fb0c, 0); // "  Profit/(Loss)"
    for k in 0..4 {
        let diff = income[k].wrapping_sub(spent[k]); // tel02_2 profit is income minus expenditure
        let negative = diff < 0;
        set_cash(cash, if negative { diff.wrapping_neg() } else { diff });
        let colour = money_view_signed_text(cash, negative);
        money_view_cell(area, k + 1, 5, align, colour);
    }
    money_view_label(area, 7, 0x98fadc, MONEY_VIEW_EMPTY_TEXT); // "  Balance<%s - COMMENT - bank balance>"
    let balance = fin.balance();
    *cash = if balance < 0 { (balance as u32).wrapping_neg() } else { balance as u32 };
    let colour = money_view_signed_text(cash, balance < 0);
    money_view_cell(area, 1, 7, align, colour);
    money_view_label(area, 8, 0x98fac8, 0); // "  Transfer Funds"
    let colour = if exe::finance_get_transfer_budget(fin, 0, 0, 0usize, 1, 0) < 0 {
        text(MONEY_VIEW_TEXT, 0x98fab8); // "None available"
        G_GUI_COLOUR_RED
    } else {
        *cash = exe::finance_get_transfer_budget(fin, 0, 0, 0usize, 1, 0) as u32;
        money_view_format(cash, MONEY_VIEW_TEXT, MONEY_VIEW_TEXT_SIZE, 6);
        if money_view_len(MONEY_VIEW_TEXT) > 0xd {
            money_view_format(cash, MONEY_VIEW_TEXT, MONEY_VIEW_TEXT_SIZE, 0x21);
        }
        MONEY_VIEW_GREEN
    };
    money_view_cell(area, 1, 8, align, colour); // the shared tail 0x46715b
}

/// The Display of the screen manager's current slot (`screen_manager::DISPLAY` of slot `screen_manager::CURRENT_SLOT`).
pub(crate) unsafe fn widget_current_display(manager: usize) -> usize {
    let slot = read_i16(manager + screen_manager::CURRENT_SLOT) as isize;
    read_usize((manager as isize + slot * 0x300 + screen_manager::DISPLAY as isize) as usize)
}

/// The address of item `widget` (0x18c bytes each) plus `field` in a Display.
pub(crate) fn widget_field(display: usize, widget: i16, field: usize) -> usize {
    (display as isize + widget as isize * 0x18c + field as isize) as usize
}

/// Shows the club_screens.cpp "Error" box at the given line (the game continues) and clears the left mouse flag: the
/// original's `_splitpath` of the source path, `utils_get_version_string`, "v%s %s.%s %d" and the message box.
pub(crate) unsafe fn club_history_error_box(line: u32) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let join: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(CLUB_SCREENS_SOURCE, 0, 0, name.as_mut_ptr(), ext.as_mut_ptr());
    let s = join(name.as_mut_ptr(), ext.as_mut_ptr(), line);
    format(0xae24d0, 0x9870e8, s); // "v%s %s.%s %d"
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0); // "Error"
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
}

/// The 36 parameters of the club history screen as `open_club_history_screen` leaves them (`prev` = the previous
/// screen's 60 params, read before opening, used with `restore`).
pub(crate) unsafe fn club_history_planned_params(club: usize, restore: u32, players_tab: u32, prev: &[usize; 0x3c]) -> [usize; 0x24] {
    let mut p = [0usize; 0x24];
    p[0] = club;
    p[1] = if players_tab != 0 { 8 } else { 3 };
    p[0x1d] = usize::MAX;
    p[2] = 1;
    p[7] = 1;
    p[0xa] = 3;
    p[0xd] = 7;
    p[0x10] = 0xc;
    p[0x13] = if players_tab != 0 { 0x20 } else { 0x10 };
    p[0x16] = 0x1c;
    p[0x20] = 1;
    p[0x23] = 1;
    if restore != 0 {
        for k in [1usize, 2, 7, 0xa, 0xd, 0x10, 0x13, 0x16] {
            p[k] = prev[k];
        }
    }
    p
}

/// The Landmarks tab of the club history screen: the club's history (`Staff_Records::get_club_history`, mask 0), one
/// row per entry from the newest (the list walked from its tail): the date (`Date::get_short_date_string`) and the
/// entry's text (`Staff_Records::format_club_history_entry`, wrapped in the second column). More than 100 entries go in
/// pages of as many rows as fit in font 2 (param 0x23 the page count, the page buttons at the top, ids 0xd and 0xe;
/// param 0x20 the page); up to 100 all rows go in, scrolled, and the text column is narrower (0x2f6) when they do not
/// fit. Empty rows fill the rest; the scroll row comes back from param 0x21 and the area goes to param 0x22.
/// Original: part of `club_history_screen_build` (0x46a1e0, club_screens.cpp).
pub(crate) unsafe fn club_history_landmarks_tab(top: i32, bottom: i32) {
    let history: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_CLUB_HISTORY);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let weights: [u8; 2] = [3, 0x14];
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0);
    let club = get_screen_param(0);
    let page = get_screen_param(0x20);
    let list = history(read_usize(G_STAFF_RECORDS), club, 0);
    let rows = ((bottom - top - 0x19) / exe::font_get_height(2)) & 0xff;
    let mut y1 = top;
    if list != 0 && read_i16(list + llist::COUNT) > 100 {
        covered!("tel03_2 landmarks pages");
        let count = read_i16(list + llist::COUNT) as i32;
        let pages = count / rows + (count % rows != 0) as i32;
        set_screen_param(0x23, pages as i16 as isize as usize);
        exe::gui_add_page_buttons(0x30c, (top) as i32 as u32, (page) as usize as u32, (pages) as i32 as u32, 0, 0xd, 0xe, 0);
        y1 = top + 0x19;
    }
    let rows = ((bottom - y1) / exe::font_get_height(2)) & 0xff;
    let area = add_area(current_human_display(), 0x6e, y1 as usize, 0x30c, bottom as usize, 2, weights.as_ptr() as usize, rows as usize, 0, 1, 0, usize::MAX);
    let mut r: i32 = 0;
    let dark_blue = colour_word(G_GUI_COLOUR_DARK_BLUE);
    let pale = colour_word(G_GUI_COLOUR_PALE_GREY);
    let yellow = colour_word(G_GUI_COLOUR_YELLOW);
    if list != 0 {
        let count = read_i16(list + llist::COUNT) as i32;
        let x2 = if count > rows && count <= 100 { 0x2f6 } else { 0x30c }; // tel03_2 the landmark text column
        let x1 = (x2 * 3 - 0x14a) / 23 + 0x6e;
        let mut k = (page as i16 as i32 - 1) * rows;
        write_u32(list + llist::CURSOR, read_u32(list + llist::TAIL));
        for _ in 0..k.max(0) {
            exe::llist_get_prev(list);
        }
        while (r < rows || read_i16(list + llist::COUNT) < 100) && k < read_i16(list + llist::COUNT) as i32 {
            let e = exe::llist_get_prev(list);
            (exe::date_get_short_date_string(e, HISTORY_TEXT, 0x7d0, 0xff) as usize);
            club_history_cell(area, 0, r, 0x30, dark_blue, 0xc, 1, pale, HISTORY_TEXT);
            exe::staff_records_format_club_history_entry(read_usize(G_STAFF_RECORDS), e, t);
            club_history_cell(area, 1, r, 2, 0, 1, 2, yellow, 0);
            club_history_draw(t, area, x1, x2, &mut r, 2, yellow, 2);
            r += 1;
            k += 1;
        }
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
    }
    while r < rows {
        for col in 0..3 {
            club_history_cell(area, col, r, 2, 0, 0xc, 2, pale, 0);
        }
        r += 1;
    }
    club_history_restore_scroll(area, get_screen_param(0x21));
    set_screen_param(0x22, area as i16 as isize as usize);
    exe::drop_text(t);
}

/// The Records tab of the club history screen: the View button (a popup of the two periods,
/// `club_records_get_period_label` 1 and 2, the chosen one, param 2, shown as the heading) and one row per record type
/// 1..0x20 that applies to the club in that period (`club_records_record_type_applies`): the type's label as a button
/// (link 5 with the type) and the record's text (`club_records_get_record_text`, wrapped). Empty rows fill the rest; the
/// scroll row comes back from param 3 (`Display::set_area_scroll_row`) and the area goes to param 4.
/// Original: part of `club_history_screen_build` (0x46a1e0, club_screens.cpp).
pub(crate) unsafe fn club_history_records_tab(top: i32, bottom: i32) {
    let button_item: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let text_item: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let area_button: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let add_area: DisplayListFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA); // ScreenManager::add_area
    let applies: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32 = core::mem::transmute(FN_CLUB_RECORDS_RECORD_TYPE_APPLIES);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let weights: [u8; 2] = [1, 2];
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0);
    let club = get_screen_param(0);
    // the view is a char (the callees read the low byte)
    let view = get_screen_param(2) as u8;
    lang(HISTORY_TEXT, 0x988004); // "View"
    let pale = colour_word(G_GUI_COLOUR_PALE_GREY);
    let grey = colour_word(G_GUI_COLOUR_GREY);
    let yellow = colour_word(G_GUI_COLOUR_YELLOW);
    let b = button_item(G_SCREEN_MANAGER, 0x6e, top as usize, 0xeb, (top + 0x14) as usize, 0x30, grey, 0xc, 1, pale, HISTORY_TEXT, 0, 0);
    let menu = exe::screenmanager_add_popup_area(G_SCREEN_MANAGER, (b) as usize as u32) as usize;
    let mut heading = [0u8; 0x7d0];
    let records = read_usize(G_CLUB_RECORDS);
    for k in 1..=2u32 {
        exe::club_records_get_period_label(records, k, HISTORY_TEXT, 0x7d0);
        exe::screenmanager_add_menu_item(G_SCREEN_MANAGER, (menu) as usize as u32, (HISTORY_TEXT) as usize as u32, 4, k);
        if k == view as i8 as i32 as u32 {
            let mut i = 0;
            loop {
                let c = read_u8(HISTORY_TEXT + i);
                heading[i] = c;
                if c == 0 {
                    break;
                }
                i += 1;
            }
        }
    }
    let y = top + 0x19;
    text_item(G_SCREEN_MANAGER, 0x6e, y as usize, 0x30c, (y + 0x23) as usize, 2, 0, 0xc, 3, yellow, heading.as_ptr() as usize, 1);
    let y = y + 0x28;
    let rows = ((bottom - y) / exe::font_get_height(2)) as u8 as usize;
    let area = add_area(G_SCREEN_MANAGER, 0x6e, y as usize, 0x30c, bottom as usize, 2, weights.as_ptr() as usize, rows, 0, 1, 0, usize::MAX);
    let x1 = (weights[0] as i32 * 648) / (weights[0] as i32 + weights[1] as i32) + 0x6e;
    let mut r: i32 = 0;
    let mut label = [0u8; 0x7d0];
    for kind in 1..=0x20u32 {
        if applies(records, kind, club, view as u32) == 0 {
            continue;
        }
        exe::club_records_get_record_type_label(records, kind, label.as_mut_ptr() as usize, 0x7d0);
        format(HISTORY_TEXT, 0x987ff0, label.as_ptr() as usize); // "  %s"
        area_button(G_SCREEN_MANAGER, area, 0, r as usize, 2, 0, 1, 2, pale, HISTORY_TEXT, 5, kind as usize);
        exe::club_records_get_record_text(records, club, view as u32, kind, t);
        club_history_cell(area, 1, r, 2, 0, 1, 1, 0, 0);
        club_history_draw(t, area, x1 + 8, 0x2ee, &mut r, 1, yellow, 2);
        r += 1;
    }
    while r < rows as i32 {
        club_history_cell(area, 0, r, 2, 0, 1, 2, pale, 0);
        club_history_cell(area, 1, r, 2, 0, 1, 1, pale, 0);
        r += 1;
    }
    exe::display_set_area_scroll_row(current_human_display(), area as u32, get_screen_param(3));
    set_screen_param(4, area as i16 as isize as usize);
    exe::drop_text(t);
}

/// One View popup of a club history tab (the sequences, players and transfers tabs): for a real widget, the area's
/// scroll row (param `area_param`) is saved into param `scroll_param` first; then, when the widget's action id (on the
/// current human's Display, or the injected one when it has none) is `id` and its value differs from param
/// `view_param`, the value goes there and the answer is −11 (rebuild); otherwise 0.
pub(crate) unsafe fn club_history_view_input(widget: u32, scroll_param: u32, area_param: u32, id: u16, view_param: u32) -> i32 {
    let w = widget as i16;
    let mut action = 0u16;
    if w != -1 {
        set_screen_param(scroll_param, exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(area_param)));
        action = read_u16((current_human_display() as isize + w as isize * 0x18c + display::ITEM_ID as isize) as usize);
    }
    if action == 0 {
        action = read_u16(G_SCREEN_MANAGER + 0x26278e);
    }
    if action != id {
        return 0;
    }
    let value = |w: i16| if w == -1 { 0 } else { read_usize((current_human_display() as isize + w as isize * 0x18c + display::ITEM_VALUE as isize) as usize) };
    if value(w) == get_screen_param(view_param) {
        return 0;
    }
    set_screen_param(view_param, value(w));
    -11
}

/// One "pick a value" answer of the club history screen: when `action` is `id` and the widget's value differs from
/// param `param`, the value goes there (read twice, as in the original) and the answer is −11; otherwise None.
pub(crate) unsafe fn club_history_pick(widget: u32, action: i32, id: i32, param: u32) -> Option<i32> {
    if action != id || exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget) == get_screen_param(param) {
        return None;
    }
    set_screen_param(param, exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget));
    Some(-11)
}

/// `ScreenManager::get_widget_action_id` of the club history screen's widget.
pub(crate) unsafe fn club_history_action(widget: u32) -> i32 {
    exe::screenmanager_get_widget_action_id(G_SCREEN_MANAGER, widget)
}

/// Whether a click on the club history screen only writes params (the tab answers that end in −11 without calling
/// another screen): only then both run side by side.
pub(crate) unsafe fn club_history_click_stays(widget: u32) -> bool {
    let w = widget as i16;
    if w == -1 {
        return false;
    }
    let action = club_history_action(widget);
    let data = exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget);
    let differs = |p: u32| data != get_screen_param(p);
    let by_tab = match get_screen_param(1) as u8 {
        1 => {
            let mut id = exe::display_get_widget_id(current_human_display(), widget);
            if id == 0 {
                id = read_u16(G_SCREEN_MANAGER + 0x26278e);
            }
            id == 3 && differs(0x1d)
        }
        2 => action == 0xd || action == 0xe,
        3 => (action == 4 && differs(2)) || action == 5,
        4 => action == 6 && differs(7),
        5 => action == 7 && differs(0xa),
        6 => action == 8 && differs(0xd),
        7 => action == 9 && differs(0x10),
        8 => action == 0xa && differs(0x13),
        9 => action == 0xb && differs(0x16),
        _ => false,
    };
    by_tab || (action == 2 && differs(1))
}

/// `Display::add_item` (0x55f990, thiscall, ret 0x48) of one entry of a View popup (`popup`): the text in `HISTORY_TAB_TEXT`, the
/// flags `flags` (1 = can be picked, 0x21 = greyed out), the link `link` with `data`.
pub(crate) unsafe fn history_tab_menu_entry(popup: usize, flags: u32, link: u32, data: usize) -> usize {
    let menu = colour_word(0xae31aa);
    exe::display_add_item(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, menu, colour_word(G_GUI_COLOUR_YELLOW), flags as usize, 1, colour_word(G_GUI_COLOUR_BLACK), HISTORY_TAB_TEXT, 0, link as usize, data, popup) as usize
}

/// The View button of a club history tab (from `top`) and its popup area (`Display::add_area` 0x130 with the menu colour
/// 0xae31aa, under the button); returns the popup.
pub(crate) unsafe fn history_tab_view_button(top: i32) -> usize {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let button_item: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    lang(HISTORY_TAB_TEXT, 0x988004); // "View"
    let b = button_item(G_SCREEN_MANAGER, 0x6e, top as usize, 0xeb, (top + 0x14) as usize, 0x30, colour_word(G_GUI_COLOUR_GREY), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), HISTORY_TAB_TEXT, 0, 0);
    add_area(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x130, colour_word(0xae31aa), b)
}

/// The CRT's inline strcpy of the original into a local buffer.
pub(crate) unsafe fn history_tab_copy(dst: &mut [u8], src: usize) {
    let mut i = 0;
    loop {
        let c = read_u8(src + i);
        dst[i] = c;
        if c == 0 {
            break;
        }
        i += 1;
    }
}

/// `ScreenManager::add_text_item` of a tab's heading (the chosen view) from `y`.
pub(crate) unsafe fn history_tab_heading(y: i32, heading: usize) {
    let text_item: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    text_item(G_SCREEN_MANAGER, 0x6e, y as usize, 0x30c, (y + 0x23) as usize, 2, 0, 0xc, 3, colour_word(G_GUI_COLOUR_YELLOW), heading, 1);
}

/// The inline `Display::set_area_scroll_row` of the tabs: the scroll row of param `scroll_param` (read first) goes to
/// the area (nothing for −1), and the area goes to param `area_param`.
pub(crate) unsafe fn history_tab_scroll(area: usize, scroll_param: u32, area_param: u32) {
    let row = get_screen_param(scroll_param);
    if area as i16 != -1 {
        exe::area_set_scroll_row((current_human_display() as isize + (area as i16) as isize * 0xbf1 + 4) as usize, row);
    }
    set_screen_param(area_param, area as i16 as isize as usize);
}

/// The record of competition `id` (0x6b bytes in `G_CLUB_COMP_TABLE`).
pub(crate) unsafe fn history_tab_comp(id: usize) -> usize {
    read_usize(G_CLUB_COMP_TABLE) + id.wrapping_mul(0x6b)
}

/// Whether competition `id` has its object loaded (`G_COMP_OBJECT_TABLE`).
pub(crate) unsafe fn history_tab_comp_loaded(id: usize) -> bool {
    read_usize(read_usize(G_COMP_OBJECT_TABLE) + id.wrapping_mul(4)) != 0
}

/// The competitions of the View popup of the Competitions tab from a list of competition ids (`list`, an LLIST of ids,
/// walked from its head): one entry per competition (its short name, link 3 with its id); the one equal to `view` gives
/// the heading.
pub(crate) unsafe fn history_competitions_menu_list(list: usize, popup: usize, view: i32, heading: &mut [u8]) {
    write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
    let mut node = exe::llist_get_next(list);
    while node != 0 {
        let comp = history_tab_comp(read_usize(node));
        history_tab_copy(core::slice::from_raw_parts_mut(HISTORY_TAB_TEXT as *mut u8, 0x100), comp + competition::SHORT_NAME);
        history_tab_menu_entry(popup, 1, 3, read_usize(comp));
        if view == read_i32(comp) {
            history_tab_copy(heading, HISTORY_TAB_TEXT);
        }
        node = exe::llist_get_next(list);
    }
}

/// The cup runs view of the Competitions tab (one cup, `view`): one row per run of the club in that cup (the cup history
/// list, param 0x1b): the season (`key_nation_format_club_season`, two years on for the World Cup qualifying sections
/// and the European Championship qualifying) and the round reached, as the competition object writes it (its virtual
/// method +0x38 with the round, the stage and the flag of the run), "-" when it writes nothing. Empty rows fill the
/// rest. Returns the area.
/// Original: part of `club_history_draw_competitions_tab` (0x46adc0, club_screens.cpp).
pub(crate) unsafe fn history_competitions_cup_runs(club: usize, view: i32, top: i32, bottom: i32, rows: u32, weights: &[u8; 5], cups: usize) -> usize {
    let season: unsafe extern "C" fn(u32, usize, usize, u32) -> i32 = core::mem::transmute(FN_KEY_NATION_FORMAT_CLUB_SEASON);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let area = add_area(current_human_display(), 0x6e, top as usize, 0x30c, bottom as usize, 2, weights.as_ptr() as usize, rows as usize, 0, 1, 0, usize::MAX);
    let pale = colour_word(G_GUI_COLOUR_PALE_GREY);
    let mut r: i32 = 0;
    let mut round = [0u8; 0x7d0];
    if cups != 0 {
        write_u32(cups + llist::CURSOR, read_u32(cups + llist::HEAD));
        let mut node = exe::llist_get_next(cups);
        while node != 0 {
            if read_i32(node) == view {
                let mut year = read_u16(node + 4);
                let qualifying = [
                    KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION,
                    KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION,
                    KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION,
                    KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT,
                    KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION,
                    KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION,
                    KEY_CLUB_COMP_EUROPEAN_CHAMPIONSHIP_QUALIFYING,
                ];
                if qualifying.iter().any(|&k| view == read_i32(k)) {
                    year = year.wrapping_add(2);
                }
                season(year as u32, club, HISTORY_TAB_TEXT, 0x7d0);
                history_tab_cell(area, 0, r, 0x30, colour_word(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, pale, HISTORY_TAB_TEXT);
                let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_usize(node) * 4);
                let method: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, usize) = core::mem::transmute(read_usize(read_usize(object) + 0x38));
                // the original pushes the byte and the words in the club pointer's register: its high bytes go along
                let high = club as u32 & 0xffff_0000;
                let flag = (club as u32 & 0xffff_ff00) | read_u8(node + 6) as u32;
                method(object, round.as_mut_ptr() as usize, 0x7d0, high | read_u16(node + 7) as u32, high | read_u16(node + 9) as u32, flag, club);
                let shown = if round[0] != 0 { round.as_ptr() as usize } else { 0x98ba0c }; // "-"
                format(HISTORY_TAB_TEXT, 0x987ff0, shown); // "  %s"
                history_tab_cell(area, 1, r, 2, 0, 1, 2, colour_word(G_GUI_COLOUR_YELLOW), HISTORY_TAB_TEXT);
                r += 1;
            }
            node = exe::llist_get_next(cups);
        }
    }
    while r < rows as i32 {
        history_tab_cell(area, 0, r, 2, 0, 1, 2, pale, 0);
        history_tab_cell(area, 1, r, 2, 0, 1, 2, pale, 0);
        r += 1;
    }
    area
}

/// The honours view of the Competitions tab: every first, second or third place of the club (and, for Germany's
/// national team, of West Germany's: `comp_util_collect_club_honours`), in three columns: the competition (linked 0x7d0
/// when its object is loaded; only on its first row), the place ("Winners", "Runners Up", "Third Placed") and the
/// years, up to 8 a row (", %d"), the same competition and place running on in new rows. Empty rows fill the rest.
/// Returns the area.
/// Original: part of `club_history_draw_competitions_tab` (0x46adc0, club_screens.cpp).
pub(crate) unsafe fn history_competitions_honours(club: usize, top: i32, bottom: i32, rows: u32, weights: &[u8; 5]) -> usize {
    let club = Club(club);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0);
    let mut previous: usize = 0;
    let mut partner: usize = 0;
    if exe::database_is_national_team(club) != 0 {
        let nation = club.nation().addr();
        if nation != 0 && read_i32(nation) == read_i32(KEY_NATION_GERMANY) && exe::club_is_national_team(club) == 0 {
            covered!("tel03_3 honours with west germany");
            partner = exe::database_get_national_team(read_usize(G_NATION_TABLE) + read_usize(KEY_NATION_WEST_GERMANY) * 0x122);
        }
    }
    let mut rows_of: usize = 0;
    let count = exe::comp_util_collect_club_honours(club, partner, &mut rows_of);
    let area = add_area(current_human_display(), 0x6e, top as usize, 0x30c, bottom as usize, 3, weights.as_ptr() as usize + 2, rows as usize, 0, 1, 0, usize::MAX);
    let pale = colour_word(G_GUI_COLOUR_PALE_GREY);
    let yellow = colour_word(G_GUI_COLOUR_YELLOW);
    let mut r: i32 = 0;
    let mut i: i32 = 0;
    let mut row = rows_of;
    let mut more = [0u8; 0x7d0];
    while i < count {
        exe::text_clear(t);
        let comp = read_usize(row + 4);
        let first = row;
        if previous != comp {
            format(HISTORY_TAB_TEXT, 0x987ff0, comp + competition::SHORT_NAME); // "  %s"
            if history_tab_comp_loaded(read_usize(comp)) {
                history_tab_link_cell(area, 0, r, pale, HISTORY_TAB_TEXT, 0x7d0, comp);
            } else {
                history_tab_cell(area, 0, r, 2, 0, 1, 2, pale, HISTORY_TAB_TEXT);
            }
        } else {
            history_tab_cell(area, 0, r, 2, 0, 1, 2, pale, 0);
        }
        previous = comp;
        let place = history_honours_place(row, club.addr(), partner);
        lang(HISTORY_TAB_TEXT, [0x990b50usize, 0x990b5c, 0x990b6c][place as usize]); // "  Winners", "  Runners Up", "  Third Placed"
        history_tab_cell(area, 1, r, 2, 0, 1, 2, pale, HISTORY_TAB_TEXT);
        format(HISTORY_TAB_TEXT, 0x990b48, read_i16(row + 8) as i32); // "  %d"
        exe::text_set_string(t as usize, HISTORY_TAB_TEXT as *const u8);
        i += 1;
        row += 0x1a;
        let mut years: i16 = 1;
        while i < count {
            if read_usize(row + 4) != read_usize(first + 4) || history_honours_place(row, club.addr(), partner) != place {
                break;
            }
            if years < 8 { // tel03_3 up to 8 years a row
                format(more.as_mut_ptr() as usize, 0x990b40, read_i16(row + 8) as i32); // ", %d"
                let mut n = 0;
                while read_u8(HISTORY_TAB_TEXT + n) != 0 {
                    n += 1;
                }
                let mut k = 0;
                loop {
                    write_u8(HISTORY_TAB_TEXT + n + k, more[k]);
                    if more[k] == 0 {
                        break;
                    }
                    k += 1;
                }
                years += 1;
            } else {
                covered!("tel03_3 honours years on a new row");
                history_tab_cell(area, 2, r, 2, 0, 1, 2, yellow, HISTORY_TAB_TEXT);
                format(HISTORY_TAB_TEXT, 0x990b48, read_i16(row + 8) as i32); // "  %d"
                r += 1;
                years = 1;
                history_tab_cell(area, 0, r, 2, 0, 1, 2, yellow, 0);
                history_tab_cell(area, 1, r, 2, 0, 1, 2, yellow, 0);
            }
            i += 1;
            row += 0x1a;
        }
        history_tab_cell(area, 2, r, 2, 0, 1, 2, yellow, HISTORY_TAB_TEXT);
        r += 1;
    }
    if rows_of != 0 {
        exe::crt_free(rows_of);
    }
    while r < rows as i32 {
        for col in 0..3 {
            history_tab_cell(area, col, r, 2, 0, 1, 2, pale, 0);
        }
        r += 1;
    }
    exe::drop_text(t);
    area
}

/// The league table view of the Competitions tab (all the domestic leagues, view −2, or one league, a division of
/// `comp_util_is_league_division`): a heading row (Pld, Won, Drn or SOW when the first league's object has flag 0x20 at
/// +0xab, Lst, For, Ag, Pts) and one row per league season of the club (the league history list, param 0x19) in that
/// view: the season, the position (`utils_format_ordinal`), the league (linked 0x7d0 when its object is loaded) and the
/// seven numbers of the record. Empty rows fill the rest. Returns the area.
/// Original: part of `club_history_draw_competitions_tab` (0x46adc0, club_screens.cpp).
pub(crate) unsafe fn history_competitions_league_table(club: usize, view: i32, top: i32, bottom: i32, rows: u32, weights: &[u8; 10], leagues: usize) -> usize {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let area = add_area(current_human_display(), 0x6e, top as usize, 0x30c, bottom as usize, 0xa, weights.as_ptr() as usize, rows as usize, 0, 1, 0, usize::MAX);
    let pale = colour_word(G_GUI_COLOUR_PALE_GREY);
    let grey = colour_word(G_GUI_COLOUR_GREY);
    let mut r: i32 = 0;
    if leagues != 0 {
        for col in 0..3 {
            history_tab_cell(area, col, 0, 2, 0, 1, 2, pale, 0);
        }
        let heading = |col: u32| {
            history_tab_cell(area, col, 0, 0x30, grey, 0xc, 1, pale, HISTORY_TAB_TEXT);
        };
        lang(HISTORY_TAB_TEXT, 0x990b3c); // "Pld"
        heading(3);
        lang(HISTORY_TAB_TEXT, 0x990b1c, 0xad9c64); // "Won<%s - COMMENT - games won>"
        heading(4);
        write_u32(leagues + llist::CURSOR, read_u32(leagues + llist::HEAD));
        let first = read_usize(exe::llist_get_current(leagues));
        if read_u8(read_usize(read_usize(G_COMP_OBJECT_TABLE) + first * 4) + 0xab) & 0x20 != 0 {
            lang(HISTORY_TAB_TEXT, 0x990b18); // "SOW"
        } else {
            lang(HISTORY_TAB_TEXT, 0x990b14); // "Drn"
        }
        heading(5);
        lang(HISTORY_TAB_TEXT, 0x990af4, 0xad9c64); // "Lst<%s - COMMENT - games lost>"
        heading(6);
        lang(HISTORY_TAB_TEXT, 0x990af0); // "For"
        heading(7);
        lang(HISTORY_TAB_TEXT, 0x990aec); // "Ag"
        heading(8);
        lang(HISTORY_TAB_TEXT, 0x990ae8); // "Pts"
        heading(9);
        write_u32(leagues + llist::CURSOR, read_u32(leagues + llist::HEAD));
        r = 1;
        let mut node = exe::llist_get_next(leagues);
        while node != 0 {
            if view == -2 || read_i32(node) == view {
                let comp = history_tab_comp(read_usize(node));
                exe::key_nation_format_club_season(read_u16(node + 4) as u32, club, HISTORY_TAB_TEXT, 0x7d0);
                history_tab_cell(area, 0, r, 0x30, colour_word(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, pale, HISTORY_TAB_TEXT);
                history_tab_copy(core::slice::from_raw_parts_mut(HISTORY_TAB_TEXT as *mut u8, 0x100), exe::utils_format_ordinal(read_i8(node + 6) as i32 + 1, -1));
                history_tab_cell(area, 1, r, 2, 0, 0xc, 2, pale, HISTORY_TAB_TEXT);
                format(HISTORY_TAB_TEXT, 0x987ff0, comp + competition::SHORT_NAME); // "  %s"
                if history_tab_comp_loaded(read_usize(comp)) {
                    history_tab_link_cell(area, 2, r, colour_word(G_GUI_COLOUR_YELLOW), HISTORY_TAB_TEXT, 0x7d0, comp);
                } else {
                    history_tab_cell(area, 2, r, 2, 0, 1, 2, colour_word(G_GUI_COLOUR_YELLOW), HISTORY_TAB_TEXT);
                }
                let numbers = [
                    read_i8(node + 0xb) as i32,
                    read_i8(node + 0xc) as i32,
                    read_i8(node + 0xd) as i32,
                    read_i8(node + 0xe) as i32,
                    read_i16(node + 0xf) as i32,
                    read_i16(node + 0x11) as i32,
                ];
                for (k, &n) in numbers.iter().enumerate() {
                    format(HISTORY_TAB_TEXT, 0x98806c, n); // "%d"
                    history_tab_cell(area, 3 + k as u32, r, 2, 0, 0xc, 2, pale, HISTORY_TAB_TEXT);
                }
                format(HISTORY_TAB_TEXT, 0x98806c, read_i16(node + 0x13) as i32); // "%d"
                history_tab_cell(area, 9, r, 0x30, colour_word(0xae3192), 0xc, 1, pale, HISTORY_TAB_TEXT);
                r += 1;
            }
            node = exe::llist_get_next(leagues);
        }
    }
    while r < rows as i32 {
        for col in 0..10 {
            history_tab_cell(area, col, r, 2, 0, 1, 2, pale, 0);
        }
        r += 1;
    }
    area
}

/// One season tab of the club history screen (positions, attendances, results, sequences, players, transfers): the
/// record types of `category` (`club_record_type_to_category`) in the View popup (link `category` + 2 with the type,
/// greyed out when the type does not apply to the club in the period: `club_records_record_type_applies`), the chosen
/// type (param `view_param`, or the default) as the heading, and one row per season of the club's records (the array
/// of `Club_Records::build_history_list`, built once and kept in params 5 and 6; 0x38d bytes a season, year in the first
/// word, 0 = none): the season (`key_nation_format_club_season`) and the record of that type
/// (`club_records_format_record_text`, wrapped). Empty rows fill the rest; the scroll row comes back from param
/// `scroll_param` and the area goes to param `area_param`.
pub(crate) unsafe fn history_season_tab(top: i32, bottom: i32, category: u8, view_param: u32, scroll_param: u32, area_param: u32, period: HistoryPeriod, default: HistoryDefault) {
    let applies: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32 = core::mem::transmute(FN_CLUB_RECORDS_RECORD_TYPE_APPLIES);
    let season: unsafe extern "C" fn(u32, usize, usize, u32) -> i32 = core::mem::transmute(FN_KEY_NATION_FORMAT_CLUB_SEASON);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let weights: [u8; 2] = [1, 8];
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0);
    let club = get_screen_param(0);
    let mut seasons = get_screen_param(5);
    let mut count = get_screen_param(6) as i32;
    let mut view = get_screen_param(view_param) as u8;
    let period = match period {
        HistoryPeriod::Param => get_screen_param(2) as u8 as u32,
        HistoryPeriod::Fixed(p) => p as u32,
    };
    let popup = history_tab_view_button(top);
    let records = read_usize(G_CLUB_RECORDS);
    match default {
        HistoryDefault::Keep => {}
        HistoryDefault::Type(k) => {
            if applies(records, view as u32, club, period) == 0 {
                view = k;
            }
        }
        HistoryDefault::FirstThatApplies => {
            if applies(records, view as u32, club, period) == 0 {
                view = 0xff;
            }
        }
    }
    // the heading stays as it was when no type matches the view (stack garbage in the original, empty here)
    let mut heading = [0u8; 0x7d0];
    for kind in 1..=0x20u32 {
        if exe::club_record_type_to_category(kind, (0) as usize as *mut i32) != category {
            continue;
        }
        if applies(records, kind, club, period) != 0 {
            if let HistoryDefault::FirstThatApplies = default {
                if view == 0xff {
                    view = kind as u8;
                }
            }
            exe::club_records_get_record_type_label(records, kind, HISTORY_TAB_TEXT, 0x7d0);
            history_tab_menu_entry(popup, 1, category as u32 + 2, kind as usize);
            if kind as u8 == view {
                history_tab_copy(&mut heading, HISTORY_TAB_TEXT);
            }
        } else {
            exe::club_records_get_record_type_label(records, kind, HISTORY_TAB_TEXT, 0x7d0);
            history_tab_menu_entry(popup, 0x21, 0, 0);
        }
    }
    history_tab_heading(top + 0x19, heading.as_ptr() as usize);
    let y = top + 0x41;
    let rows = ((bottom - y) / exe::font_get_height(2)) as u8;
    let area = add_area(current_human_display(), 0x6e, y as usize, 0x30c, bottom as usize, 2, weights.as_ptr() as usize, rows as usize, 0, 1, 0, usize::MAX);
    if seasons == 0 {
        covered!("tel03_3 season tab builds the records");
        seasons = exe::club_records_build_history_list(records, club, &mut count);
        set_screen_param(5, seasons);
        set_screen_param(6, count as usize);
    }
    let pale = colour_word(G_GUI_COLOUR_PALE_GREY);
    let yellow = colour_word(G_GUI_COLOUR_YELLOW);
    let mut r: i32 = 0;
    let mut entry = seasons;
    let mut i = 0;
    while i < count {
        let year = read_u16(entry);
        if year != 0 {
            season(year as u32, club, HISTORY_TAB_TEXT, 0x7d0);
            history_tab_cell(area, 0, r, 0x30, colour_word(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, pale, HISTORY_TAB_TEXT);
            exe::club_records_format_record_text(records, club, entry + 2, view as u32, t);
            history_tab_cell(area, 1, r, 2, 0, 1, 2, 0, 0);
            (exe::text_draw_wrapped((t) as *mut usize as usize, (area) as usize as u32, (0xbe) as usize as i32, (0x2ee) as usize as i32, &mut r, (2) as usize as u32, (yellow) as usize as u32, (2) as usize as u32, (0) as usize as u32) as usize);
            r += 1;
        }
        i += 1;
        entry += 0x38d;
    }
    while r < rows as i32 {
        history_tab_cell(area, 0, r, 2, 0, 1, 2, yellow, 0);
        history_tab_cell(area, 1, r, 2, 0, 1, 2, yellow, 0);
        r += 1;
    }
    history_tab_scroll(area, scroll_param, area_param);
    exe::drop_text(t);
}

/// The side-by-side comparer of a club history tab: both draw and are compared by the display list
/// (`compare_match_screen_build`); when a list or the record array the tab keeps in the params is still to be built
/// (`cache` holds one of those params at 0), only the original runs (building twice would leave two lists and the
/// params would differ by the pointer).
pub(crate) unsafe fn compare_history_tab(name: &str, cache: &[u32], o: unsafe extern "C" fn(i32, i32), r: unsafe extern "C" fn(i32, i32), top: i32, bottom: i32) {
    if cache.iter().any(|&k| get_screen_param(k) == 0) {
        return o(top, bottom);
    }
    compare_match_screen_build(name, &[], || o(top, bottom), || r(top, bottom));
}

/// The action id of widget `w` on the current human's Display (the item id read inline), or the injected id
/// (`G_CURRENT_COMMAND_ID`) when it has none or for widget −1.
pub(crate) unsafe fn club_tabs_action(w: i16) -> u16 {
    let id = if w == -1 { 0 } else { read_u16((current_human_display() as isize + w as isize * 0x18c + display::ITEM_ID as isize) as usize) };
    if id == 0 {
        read_u16(G_CURRENT_COMMAND_ID)
    } else {
        id
    }
}

/// The value attached to widget `w` on the current human's Display (read inline), or 0 for widget −1.
pub(crate) unsafe fn club_tabs_value(w: i16) -> usize {
    if w == -1 {
        0
    } else {
        read_usize((current_human_display() as isize + w as isize * 0x18c + display::ITEM_VALUE as isize) as usize)
    }
}

/// The fixture the Previous (`forward` false) or Next (`forward` true) button of the next match tab moves to, from the
/// shown fixture `fixture`: `FIXTURE_MANAGER::find_fixtures` of the club (any competition, type −2, the side "not
/// reserves") from the day after the fixture to 24 weeks on (8 for a club that is not a national team), or from 24 (8)
/// weeks before to the day before; Next takes the first found, Previous the last (the list walked from its tail).
/// Returns a copy of its 0x4f bytes, or None.
pub(crate) unsafe fn club_next_match_neighbour(club: usize, reserves: usize, fixture: usize, forward: bool) -> Option<[u8; 0x4f]> {
    let fixture = Fixture(fixture);
    type DateStep = unsafe extern "thiscall" fn(*const [u8; 8], *mut [u8; 8], u32) -> *const [u8; 8];
    let sub_days: DateStep = core::mem::transmute(FN_DATE_SUB_DAYS);
    let sub_weeks: DateStep = core::mem::transmute(FN_DATE_SUB_WEEKS);
    let add_weeks: DateStep = core::mem::transmute(FN_DATE_ADD_WEEKS);
    let add_days: DateStep = core::mem::transmute(FN_DATE_ADD_DAYS);
    let mut list = [0u8; 0xe];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    let mut d = [0u8; 8];
    exe::new_date((&mut d) as *mut [u8; 8] as usize, fixture.day() as u32, fixture.year() as u32);
    let weeks = if exe::database_is_national_team(club) != 0 { 0x18 } else { 8 }; // tel03_4 the search window
    // each date is worked out twice, once for its year and once for its day, as in the original
    let step = |f: DateStep, n: u32| -> (u32, u32) {
        let (mut a, mut b) = ([0u8; 8], [0u8; 8]);
        let year = read_u16(f(&d, &mut a, n) as usize + 2) as u32;
        let day = read_u16(f(&d, &mut b, n) as usize) as u32;
        (day, year)
    };
    let ((start_day, start_year), (end_day, end_year)) = if forward {
        (step(add_days, 1), step(add_weeks, weeks))
    } else {
        (step(sub_weeks, weeks), step(sub_days, 1))
    };
    exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), start_day, start_year, end_day, end_year, read_u32(club), l, u32::MAX, (-2i32) as u32, (reserves == 0) as u32);
    let found = if forward {
        write_u32(l + llist::CURSOR, read_u32(l + llist::HEAD));
        exe::llist_get_next(l)
    } else {
        write_u32(l + llist::CURSOR, read_u32(l + llist::TAIL));
        exe::llist_get_prev(l)
    };
    let copy = if found == 0 {
        None
    } else {
        let mut c = [0u8; 0x4f];
        for (k, b) in c.iter_mut().enumerate() {
            *b = read_u8(found + k);
        }
        Some(c)
    };
    exe::drop_llist(l);
    copy
}

/// The Please Confirm box of a cancellation on the fixtures tab: the title "Cancel ..." (`caption`), the heading
/// "Please Confirm", the message (already in the text buffer: `message` is what the original passes, the buffer or the
/// formatter's answer), the yes/no buttons, the club's kit colours and the dialog code `code`.
pub(crate) unsafe fn club_fixtures_confirm(caption: usize, message: usize, colours: (u32, u32), code: u32) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (mut heading, mut title) = ([0u8; 0x100], [0u8; 0x100]);
    let h = lang(heading.as_mut_ptr() as usize, 0x9909b8); // "Please Confirm"
    let t = lang(title.as_mut_ptr() as usize, caption);
    exe::confirm_push(t, h, message, 1, colours.0, colours.1, code);
}

/// The long date of the cancellation boxes (`Date::get_long_date_string`, 200 bytes, no time of day).
pub(crate) unsafe fn club_fixtures_long_date(date: usize, out: &mut [u8; 0xc8]) {
    exe::date_get_long_date_string(date, out.as_mut_ptr() as usize, 0xc8, 0xff);
}

/// Shows the club_screens.cpp "Error" box at the given line (the game continues) and clears the left mouse flag.
pub(crate) unsafe fn club_tabs_error_box(line: u32) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let join: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(0x98e598, 0, 0, name.as_mut_ptr(), ext.as_mut_ptr());
    let s = join(name.as_mut_ptr(), ext.as_mut_ptr(), line);
    format(0xae24d0, 0x9870e8, s); // "v%s %s.%s %d"
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0); // "Error"
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
}

/// The club_screens.cpp "Error" box at `line` (the game goes on).
pub(crate) unsafe fn club_pages_error_box(line: u32) {
    tel26_error_box(CLUB_SCREENS_CPP_FILE, line);
}

/// `langlib_sprintf` (cdecl, variadic).
pub(crate) unsafe fn club_pages_lang() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_LANGLIB_SPRINTF)
}

/// The CRT `sprintf` (cdecl, variadic).
pub(crate) unsafe fn club_pages_format() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_CRT_SPRINTF)
}

/// `Display::add_item` on the current human's Display.
pub(crate) unsafe fn club_pages_item(f: DisplayItemFields) -> u32 {
    tel26_add_item(current_human_display(), f)
}

/// `Display::add_area` on the current human's Display (returns eax, the area in ax).
pub(crate) unsafe fn club_pages_area(f: [u32; 11]) -> u32 {
    tel26_add_area(current_human_display(), f)
}

/// The id of the clicked widget on the current human's Display (0 and −1 become the default command id) as the
/// handlers compare it.
pub(crate) unsafe fn club_pages_id(widget: u32) -> i16 {
    tel26_item_id(widget as i16) as i16
}

/// The scroll row of the area in screen param `area_param` saved in param `save` (when the widget is real).
pub(crate) unsafe fn club_pages_save_scroll(widget: u32, save: u32, area_param: u32) {
    if widget as i16 != -1 {
        set_screen_param(save, exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(area_param)));
    }
}

/// The scroll row of screen param `row_param` put back on area `area` (when there is one) and the area written to
/// param `area_param`.
pub(crate) unsafe fn club_pages_restore_scroll(area: u32, row_param: u32, area_param: u32) {
    let row = get_screen_param(row_param);
    if area as i16 != -1 {
        exe::area_set_scroll_row(display_area_at(current_human_display(), area as i16), row);
    }
    set_screen_param(area_param, area as i16 as isize as usize);
}

/// The menu bar's answer to a click no screen handler took: its commands, then the object links.
pub(crate) unsafe fn club_pages_menu_bar(widget: u32) -> i32 {
    let r = exe::menubar_handle_command(widget);
    if r != 0 {
        return r;
    }
    exe::menubar_handle_object_link(widget)
}

/// The kit colours of `club` (`database_get_club_kit_colours`, mode 1) and the screen title with them.
pub(crate) unsafe fn club_pages_title(club: usize, text: usize) {
    let (mut first, mut second) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut first, &mut second);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, second, first, text);
}

/// `comp_screens_open_comp_fixtures(comp, day, year, mode)` (0x48f8e0, cdecl).
pub(crate) unsafe fn club_pages_open_comp_fixtures(comp: usize, day: u16, year: u16, mode: u32) {
    exe::comp_screens_open_comp_fixtures(comp, day as u32, year as u32, mode);
}

/// `Date::Date(this, day, year)` (0x549ef0, thiscall): returns `this`.
pub(crate) unsafe fn club_pages_date(date: *mut u32, day: u16, year: u16) -> usize {
    exe::new_date((date) as *mut u32 as usize, day as u32, year as u32)
}

/// `Date::get_short_date_string(this, out, size, time)` (0x54a650, thiscall).
pub(crate) unsafe fn club_pages_short_date(date: usize, out: usize, size: u32, time: u32) {
    exe::date_get_short_date_string(date, out, size, time);
}

/// `Date::add_days(this, out, days)` (0x54c5b0, thiscall): returns `out`.
pub(crate) unsafe fn club_pages_add_days(date: usize, out: *mut u32, days: i16) -> usize {
    exe::date_add_days(date, (out) as *mut u32 as usize, (days as i32) as i32 as u32)
}

/// A Date copied like `Date::operator=` (0x417c10) does it (the 8 bytes; the hidden result is a scratch copy).
pub(crate) unsafe fn club_pages_copy_date(dst: usize, src: usize) {
    let mut scratch = [0u32; 2];
    exe::date_operator_3d(dst, scratch.as_mut_ptr() as usize, src);
}

/// The past meetings cache of the current screen slot: the record of its human manager (by staff id minus the staff
/// count), or the default one without a manager.
pub(crate) unsafe fn past_meetings_cache_of_slot() -> usize {
    let manager = read_usize((G_SCREEN_RECORDS_MANAGER as isize + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize * 0x300) as usize);
    if manager == 0 {
        return PAST_MEETINGS_DEFAULT_CACHE;
    }
    (read_u32(manager).wrapping_sub(read_u32(G_STAFF_COUNT)).wrapping_mul(past_meetings_cache::SIZE as u32) as usize).wrapping_add(PAST_MEETINGS_CACHES)
}

/// One choice of the past meetings popups: ticked (0x2002, no id) when its filter bit is on, else a plain item with
/// the id and value that pick it.
pub(crate) unsafe fn past_meetings_choice(on: bool, id: u32, value: u32, area: u32) {
    let (blue, yellow, black) = (job_colour(0xae31aa), job_colour(G_GUI_COLOUR_YELLOW), job_colour(G_GUI_COLOUR_BLACK));
    let kind = if on { 0x2002 } else { 2 };
    club_pages_item([kind, 0, 0, 0, 0, 0, 0, 0x10, blue, yellow, 1, 1, black, JOB_TEXT as u32, 0, id, value, area]);
}

/// "At <club>" or, with a stadium, "At <stadium>" into the text buffer.
pub(crate) unsafe fn past_meetings_at(club: usize) {
    let club = Club(club);
    let lang = club_pages_lang();
    let stadium = club.stadium().addr();
    if stadium == 0 {
        let c = (club.short_gender_name() as i8) as i32;
        lang(JOB_TEXT, 0x990c00, c, c, club.addr() + club::SHORT_NAME); // "At {}<%s - club>{}"
    } else {
        let c = read_i8(stadium + 0x37) as i32;
        lang(JOB_TEXT, 0x990c14, c, c, stadium + 4); // "At {}<%s - stadium>{}"
    }
}

/// Whether a meeting's competition gets a link: a competition with an object (0xadadfc by the fixture's first field),
/// not a reserve or minor one, not a friendly's own record, and current on the meeting's date.
pub(crate) unsafe fn past_meetings_comp_linked(f: usize) -> bool {
    let comp = read_usize(f + screen_fixture::COMP);
    if comp == 0 || read_u32(read_usize(G_COMP_OBJECT_TABLE) + read_u32(f) as usize * 4) == 0 {
        return false;
    }
    if exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), comp) != 0 || exe::friendly_is_own_comp_record(read_usize(G_FRIENDLY), comp) != 0 {
        return false;
    }
    let mut date = [0u32; 2];
    let d = club_pages_date(date.as_mut_ptr(), read_u16(f + screen_fixture::DAY), read_u16(f + screen_fixture::YEAR));
    exe::friendly_is_comp_current_on_date(read_usize(G_FRIENDLY), comp, d) != 0
}

/// The past meetings search: the date, the clubs, the reserves flag and the filter are stored in the cache, its list
/// is emptied and the fixtures of the club from the period's start (game start, or today minus 3, 5, 10 or 25 years)
/// to today are found (`FIXTURE_MANAGER::find_fixtures`, reserves when the flag is 0); the played ones against the
/// opposition that pass the venue filters (0x40 home, 0x80 away, 0x100 neutral) and, with 0x400, are not friendlies
/// go into the cache's list; then the head-to-head table (a SubLeague of the two clubs over that list) replaces the
/// old one.
pub(crate) unsafe fn past_meetings_search(cache: usize, club: usize, opposition: usize, reserves: usize, filter: u32) {
    let mut found = [0u8; 0xe];
    let list = found.as_mut_ptr();
    exe::new_llist((list) as *mut u8 as usize);
    club_pages_copy_date(cache + past_meetings_cache::DATE, G_CURRENT_DATE);
    write_usize(cache + past_meetings_cache::CLUB, club);
    write_usize(cache + past_meetings_cache::OPPOSITION, opposition);
    write_usize(cache + past_meetings_cache::RESERVES, reserves);
    write_u32(cache + past_meetings_cache::FILTER, filter);
    exe::llist_clear(cache + past_meetings_cache::LIST);
    let today = read_u32(G_CURRENT_DATE);
    let (day, year) = (today & 0xffff, today >> 16);
    let id = read_u32(club);
    let first_team = (reserves == 0) as u32;
    // simplification: the original passes the start and end dates as dwords whose high halves are register
    // garbage (the club id, a stack address); find_fixtures reads their words
    let back = [(2u32, 3u32), (4, 5), (8, 10), (0x10, 25)].iter().find(|(bit, _)| filter & bit != 0).map(|&(_, years)| years);
    let (from_day, from_year) = match back {
        Some(years) => (today, year.wrapping_sub(years) & 0xffff), // tel04_1 the period start
        None => (read_u16(G_GAME_START_DATE) as u32, (id & 0xffff_0000) | read_u16(G_GAME_START_DATE + 2) as u32),
    };
    exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), from_day, from_year, (id & 0xffff_0000) | day, year, id, list as usize, u32::MAX, 0xffff_fffe, first_team);
    if read_i16(list as usize + llist::COUNT) != 0 {
        write_u32(list as usize + llist::CURSOR, read_u32(list as usize + llist::HEAD));
        loop {
            let f = exe::llist_get_next(list as usize);
            if f == 0 {
                break;
            }
            if read_i8(f + screen_fixture::HOME_GOALS) <= -1 || (read_usize(f + screen_fixture::HOME) != opposition && read_usize(f + screen_fixture::AWAY) != opposition) {
                continue;
            }
            let side = exe::comp_util_get_fixture_venue_side(club, f);
            if (filter & 0x40 != 0 && side != 1) || (filter & 0x80 != 0 && side != 2) || (filter & 0x100 != 0 && side != 3) {
                continue;
            }
            if filter & 0x400 != 0 && exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), read_usize(f + screen_fixture::COMP)) != 0 {
                continue;
            }
            exe::llist_add_tail(cache + past_meetings_cache::LIST, f, 0x4f);
        }
    }
    let ids = [read_u32(club), read_u32(opposition)];
    let rules: [u8; 3] = [1, 2, 3];
    let old = read_usize(cache + past_meetings_cache::TABLE);
    if old != 0 {
        let destroy: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(read_usize(read_usize(old)));
        destroy(old, 1);
    }
    let room = exe::crt_operator_new(0xee);
    let table = if room != 0 {
        type SubLeague = unsafe extern "thiscall" fn(usize, u32, *const u32, i32, i32, u32, u32, i32, u32, u32, *const u8, i32, usize) -> usize;
        let make: SubLeague = core::mem::transmute(FN_SUBLEAGUE_SUBLEAGUE);
        make(room, 2, ids.as_ptr(), -1, -1, 0, 0, -1, 3, 1, rules.as_ptr(), -1, cache + past_meetings_cache::LIST)
    } else {
        0
    };
    write_usize(cache + past_meetings_cache::TABLE, table);
    exe::drop_llist((list) as *mut u8 as usize);
}

/// The tournament settings (0xd1 bytes) of a new CM Cup: the name "CM Cup", kind 2 (league) at +0xc8, 3 points for
/// a win (+0xc9), goal difference (+0xca), no extra time (+0xcb), no decider (+0xcd) and 4 teams (+0xcf).
pub(crate) unsafe fn friendly_new_cup_settings(s: usize) {
    club_pages_lang()(s, 0x990cb4); // "CM Cup"
    write_u8(s + 0xc8, 2);
    write_u8(s + 0xc9, 3);
    write_u8(s + 0xca, 1);
    write_u16(s + 0xcb, 0);
    write_u16(s + 0xcd, 0);
    write_u16(s + 0xcf, 4);
}

/// One row of the friendly dates calendar: the date, then by the date's status for the club
/// (`Friendly::get_club_date_status`): 5 a friendly match (the opponent "?" linked, his national team when he is a club
/// of another nation, H/A, "Friendly"), 6 a tour request (Tba, the nation's national team, A, "Tour Of <nation>"), 3 a
/// tournament (Tba, H/A for its host, its name linked to its competition), 2 unavailable, 4 chosen (click to drop, id
/// 7), 1 available (click to choose, id 8); other statuses leave the row empty.
pub(crate) unsafe fn friendly_dates_status_row(area: u32, row: u32, date: *mut u32, club: usize, chosen: usize, count: u8, kind: u8, settings: usize) {
    let club = Club(club);
    let lang = club_pages_lang();
    let format = club_pages_format();
    let (pale, yellow, dark_blue) = (job_colour(G_GUI_COLOUR_PALE_GREY), job_colour(G_GUI_COLOUR_YELLOW), job_colour(G_GUI_COLOUR_DARK_BLUE));
    let code = match kind {
        10 => read_u8(settings + 0xc8) as u32,
        3 | 4 => 0,
        _ => kind as u32,
    };
    let (mut tournament, mut fixture, mut request) = (0usize, 0usize, 0usize);
    type Status = unsafe extern "thiscall" fn(usize, usize, *mut u32, *mut usize, *mut usize, *mut usize, usize, i32, u32, u32) -> u8;
    let status: Status = core::mem::transmute(FN_FRIENDLY_GET_CLUB_DATE_STATUS);
    let s = status(read_usize(G_FRIENDLY), club.addr(), date, &mut tournament, &mut fixture, &mut request, chosen, count as i8 as i32, code, 1);
    if !matches!(s, 1..=6) {
        return;
    }
    exe::date_get_short_weekday_date_string_no_year(date, JOB_TEXT, 0x7d0, 0xff);
    tel26_area_text([area, 0, row, 0x30, dark_blue, 0xc, 1, pale, JOB_TEXT as u32]);
    match s {
        5 => {
            let mut opponent = read_usize(fixture + 0x10);
            if club.addr() == opponent {
                opponent = read_usize(fixture + 0x14);
            }
            format(JOB_TEXT, 0x98f898, opponent + club::SHORT_NAME); // "  %s ?"
            let colour = if exe::human_manager_is_human_club(opponent, 0, 1, 0, 0) != 0 { yellow } else { pale };
            tel26_area_button([area, 1, row, 1, 0, 1, 3, colour, JOB_TEXT as u32, 0x7d5, opponent as u32]);
            let nation = read_usize(opponent + club::NATION);
            if exe::database_is_national_team(opponent) == 0 && nation != club.nation().addr() {
                let team = exe::database_get_national_team(nation);
                tel26_area_button([area, 2, row, 1, 0, 1, 2, yellow, (nation + 0x53) as u32, 0x7d5, team as u32]);
            }
            lang(JOB_TEXT, if club.addr() == read_usize(fixture + 0x10) { 0x98f880usize } else { 0x98f868 }, 0xad9c64usize); // H / A
            tel26_area_text([area, 3, row, 1, 0, 0xc, 2, job_colour(0xaebde8), JOB_TEXT as u32]);
            lang(JOB_TEXT, 0x98f85c); // "  Friendly"
            tel26_area_text([area, 4, row, 1, 0, 1, 2, job_colour(SCREEN_COLOUR_BDDE), JOB_TEXT as u32]);
        }
        6 => {
            lang(JOB_TEXT, 0x98f850); // "  Tba"
            tel26_area_text([area, 1, row, 1, 0, 1, 3, pale, JOB_TEXT as u32]);
            let nation = read_usize(request + 4);
            let team = exe::database_get_national_team(nation);
            tel26_area_button([area, 2, row, 1, 0, 1, 2, yellow, (nation + 0x53) as u32, 0x7d5, team as u32]);
            lang(JOB_TEXT, 0x98f868, 0xad9c64usize); // "A"
            tel26_area_text([area, 3, row, 1, 0, 0xc, 2, job_colour(0xaebde8), JOB_TEXT as u32]);
            let c = read_i8(nation + nation::SHORT_GENDER_NAME) as i32;
            lang(JOB_TEXT, 0x98f834, c, c, nation + nation::SHORT_NAME); // "  Tour Of {}<%s - nation>{}"
            tel26_area_text([area, 4, row, 1, 0, 1, 2, job_colour(SCREEN_COLOUR_BDDE), JOB_TEXT as u32]);
        }
        3 => {
            lang(JOB_TEXT, 0x98f850); // "  Tba"
            tel26_area_text([area, 1, row, 1, 0, 1, 3, pale, JOB_TEXT as u32]);
            lang(JOB_TEXT, if club.addr() == read_usize(tournament) { 0x98f880usize } else { 0x98f868 }, 0xad9c64usize);
            tel26_area_text([area, 3, row, 1, 0, 0xc, 2, job_colour(0xaebde8), JOB_TEXT as u32]);
            format(JOB_TEXT, 0x987ff0, tournament + 0x31); // "  %s"
            let comp = read_usize(tournament + 0x65);
            if comp != 0 {
                tel26_area_button([area, 4, row, 1, 0, 1, 2, job_colour(SCREEN_COLOUR_BDDE), JOB_TEXT as u32, 0x7d0, read_u32(comp + 4)]);
            } else {
                tel26_area_text([area, 4, row, 1, 0, 1, 2, job_colour(SCREEN_COLOUR_BDDE), JOB_TEXT as u32]);
            }
        }
        2 => {
            lang(JOB_TEXT, 0x990ce0); // "  Date Unavailable"
            tel26_area_text([area, 1, row, 1, 0, 1, 3, job_colour(G_GUI_COLOUR_GREY), JOB_TEXT as u32]);
        }
        4 => {
            lang(JOB_TEXT, 0x990cd0); // "  Date Selected"
            tel26_area_button([area, 1, row, 1, 0, 1, 3, yellow, JOB_TEXT as u32, 7, row]);
        }
        _ => {
            lang(JOB_TEXT, 0x990cbc); // "  Date Available"
            tel26_area_button([area, 1, row, 1, 0, 1, 3, pale, JOB_TEXT as u32, 8, row]); // tel04_1 the available date
        }
    }
    tel26_copy_text(JOB_TEXT as *mut u8, FRIENDLY_DATES_ROW_END as *const u8);
    tel26_area_text([area, 5, row, 0x30, job_colour(0xae3192), 0xc, 1, pale, JOB_TEXT as u32]);
}

/// The 8-byte Date `k` of the chosen dates vector.
pub(crate) unsafe fn friendly_dates_slot(chosen: usize, k: i32) -> usize {
    (chosen as isize + k as isize * 8) as usize
}

/// `open_configure_tournament_screen` (0x472900) by address.
pub(crate) unsafe fn open_configure_tournament_screen_by_address(settings: usize, arg: u32) {
    exe::open_configure_tournament_screen(settings, arg);
}

/// The Next button of the friendly dates (id 10): see `arrange_friendly_dates_screen_input`.
pub(crate) unsafe fn friendly_dates_next(club: usize, chosen: usize, count: i8, kind: u8, tour: usize, settings: usize) -> i32 {
    let club = Club(club);
    if kind == 3 || kind == 4 {
        let made = exe::friendly_add_friendly_match(read_usize(G_FRIENDLY), club, chosen, 0usize, (kind == 3) as u32);
        if made == 0 {
            club_pages_error_box(0x3229);
            return 0;
        }
        write_u32(made + 8, 1);
        let national = exe::database_is_national_team(club);
        let nation = club.nation();
        if national != 0 {
            exe::open_choose_friendly_teams_screen(1, nation.continent(), nation, 2);
        } else {
            exe::open_choose_friendly_teams_screen(4, nation.continent(), nation, 1);
        }
        return -4;
    }
    if kind == 0xb {
        if exe::friendly_add_friendly_dates_request(read_usize(G_FRIENDLY), club, tour, count as u8 as u32, chosen) != 0 {
            return -10;
        }
        club_pages_error_box(0x323f);
        return 0;
    }
    let invites: [usize; 4] = [club.addr(), 0, 0, 0];
    type AddTournament = unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32, u32, u32, *const usize, usize) -> usize;
    let add: AddTournament = core::mem::transmute(FN_FRIENDLY_ADD_TOURNAMENT);
    let made = add(read_usize(G_FRIENDLY), club.addr(), read_u8(settings + 0xc8) as u32, chosen, read_u8(settings + 0xc9) as u32, read_u8(settings + 0xca) as u32,
        read_u16(settings + 0xcb) as u32, read_u16(settings + 0xcd) as u32, read_u16(settings + 0xcf) as u32, invites.as_ptr(), settings);
    if made == 0 {
        club_pages_error_box(0x3252);
        return 0;
    }
    exe::crt_operator_delete(settings);
    write_u32(G_PENDING_COMMAND_CODE, 0x11);
    write_usize(G_PENDING_COMMAND_ARG, made);
    -10
}

/// A club screen build side by side, but the copy runs only when `go_on` (asked after the original) says so: the
/// original's result stays otherwise.
pub(crate) unsafe fn club_pages_compare_build_when(name: &str, extra: &[(usize, usize)], original: impl FnOnce(), rust: impl FnOnce(), go_on: impl FnOnce() -> bool) {
    let reg = club_pages_build_regions(extra);
    let before = crate::comparison::MemorySnapshot::read(&reg);
    original();
    if !go_on() {
        // simplification: this case does not run twice; proven in Rust mode
        return;
    }
    let after_original = crate::comparison::MemorySnapshot::read(&reg);
    before.write_back();
    crate::rng::pause_trace(|| crate::rng::without_counting(rust));
    let after_rust = crate::comparison::MemorySnapshot::read(&reg);
    after_original.write_back();
    let same = after_original == after_rust;
    SCOREBOARD.record(same, || format!("{}: differences {:?}", name, first_differences(&after_original, &after_rust)));
}

/// A club screen input side by side over the screen params plus `extra` (the answer too), or only the original.
pub(crate) unsafe fn club_pages_compare_input(name: &str, widget: u32, o: unsafe extern "C" fn(u32) -> i32, r: unsafe extern "C" fn(u32) -> i32, both: bool, extra: &[(usize, usize)]) -> i32 {
    if widget as u16 == 0xffff || !both {
        // simplification: opening another screen, the menu bar and the game's records do not run twice
        return o(widget);
    }
    let mut reg = screen_param_regions();
    reg.extend_from_slice(extra);
    let (ro, rr, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(widget), || r(widget));
    SCOREBOARD.record(same && ro == rr, || format!("{} (widget {:#x}): original={} rust={} differences {:?}", name, widget, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// The params 0..`n` of the current screen.
pub(crate) unsafe fn club_pages_params(n: u32) -> Vec<usize> {
    (0..n).map(|k| get_screen_param(k)).collect()
}

/// Whether a nation is open to a tour of `club`: a club of reputation tier (`friendly_club_reputation_tier`) below 4
/// only tours its own region, below 5 its own continent.
pub(crate) unsafe fn friendly_tour_nation_open(club: usize, n: usize) -> bool {
    let n = Nation(n);
    let club = Club(club);
    let tier: unsafe extern "thiscall" fn(usize, usize) -> i8 = core::mem::transmute(FN_FRIENDLY_CLUB_REPUTATION_TIER);
    let home = club.nation();
    if tier(read_usize(G_FRIENDLY), club.addr()) < 4 && (home.actual_region() as i8) != (n.actual_region() as i8) {
        return false;
    }
    !(tier(read_usize(G_FRIENDLY), club.addr()) < 5 && home.continent() != n.continent()) // tel04_2 the continent rule
}

/// One choice of the Configure Tournament table: plain grey text when it does not apply (a cup's points, tie-break and
/// decider), the yellow text 0x221 when it is the setting, else a button with its id and value.
pub(crate) unsafe fn tournament_setting(area: u32, col: u32, row: u32, disabled: bool, current: bool, id: u32, value: u32) {
    let green = job_colour(0xad9db6);
    if disabled {
        tel26_area_text([area, col, row, 1, 0, 0x2c, 2, green, JOB_TEXT as u32]);
    } else if current {
        let yellow = job_colour(G_GUI_COLOUR_YELLOW);
        tel26_area_text([area, col, row, 0x221, yellow, 0xc, 2, yellow, JOB_TEXT as u32]);
    } else {
        tel26_area_button([area, col, row, 1, 0, 0xc, 2, green, JOB_TEXT as u32, id, value]);
    }
}

/// A NewsItem (0xe0 bytes; room to spare) made by its constructor (0x77bbe0, thiscall(type, important flag)).
pub(crate) unsafe fn club_pages_news_item(item: &mut [u32; 0x40], kind: u32, important: u32) -> usize {
    exe::new_newsitem((item.as_mut_ptr()) as *mut u32 as usize, kind, important);
    item.as_mut_ptr() as usize
}

/// `News::find_item_by_id(manager, id, out)` (0x786b70, thiscall on [0xb7431c]): nonzero when found.
pub(crate) unsafe fn club_pages_find_news(manager: usize, id: usize, item: usize) -> u32 {
    exe::news_find_item_by_id(read_usize(G_NEWS), manager, (id) as usize as u32, item)
}

/// `News::set_item_slot(manager, id, slot, value, copy)` (0x786d30, thiscall on [0xb7431c]).
pub(crate) unsafe fn club_pages_set_news_slot(manager: usize, id: usize, slot: u32, value: u32) {
    exe::news_set_item_slot(read_usize(G_NEWS), manager, id, slot, value, 0);
}

/// The Back (id 1), No (id 2) and Yes (id 3) buttons of an invitation.
pub(crate) unsafe fn invitation_answer_buttons() {
    let lang = club_pages_lang();
    let weights: [u8; 3] = [2, 1, 1];
    let (grey, pale) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY));
    let area = club_pages_area([100, 0x22b, 0x316, 0x24e, 3, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    for (col, text) in [(0u32, 0x9880a8usize), (1, 0x98e8cc), (2, 0x98e8d0)] {
        lang(JOB_TEXT, text); // "Back", "No", "Yes"
        tel26_area_button([area, col, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, col + 1, 0]);
    }
}

/// Whether the current slot's human manages `club`.
pub(crate) unsafe fn tournament_hosted_here(t: usize) -> bool {
    read_u32(read_usize(t) + club::MANAGER) == read_u32((G_SCREEN_RECORDS_MANAGER as isize + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize * 0x300) as usize)
}

/// The friendly match a friendly invitation news item points at: slot 2 the club index, slots 3 and 4 the day and year
/// (`Friendly::find_friendly_match`); `None` when the item is not found.
pub(crate) unsafe fn friendly_invitation_match(manager: usize, id: usize) -> Option<usize> {
    let mut item = [0u32; 0x40];
    let it = club_pages_news_item(&mut item, 0, 0);
    if club_pages_find_news(manager, id, it) == 0 {
        return None;
    }
    let c = exe::newsitem_get_slot(it, 2) as i32;
    let day = exe::newsitem_get_slot(it, 3);
    let year = exe::newsitem_get_slot(it, 4);
    let club = club_pages_club_at(c);
    let mut date = [0u32; 2];
    let d = club_pages_date(date.as_mut_ptr(), day as u16, year as u16);
    Some(exe::friendly_find_friendly_match(read_usize(G_FRIENDLY), d, club))
}

/// The tournament and the invited club of a tournament invitation news item (slot 2 the tournament index,
/// `Friendly::get_tournament` without a date; slot 1 the club index); `None` when the item is not found.
pub(crate) unsafe fn tournament_invitation_target(manager: usize, id: usize) -> Option<(usize, usize)> {
    let mut item = [0u32; 0x40];
    let it = club_pages_news_item(&mut item, 0, 0);
    if club_pages_find_news(manager, id, it) == 0 {
        return None;
    }
    let c = exe::newsitem_get_slot(it, 1) as i32;
    let index = exe::newsitem_get_slot(it, 2) as u8;
    let t = exe::friendly_get_tournament(read_usize(G_FRIENDLY), index as u32, 0);
    Some((t, club_pages_club_at(c)))
}

/// The 19 params of the squad numbers screen as its openers write them: 0 and 1 the given pair, 2 the screen kind, 3
/// its flag, 4 the view 0x41, 5 the sort 8, 6 the second sort 6, 7 and 8 0, 9 and 10 −1, 0xb..0xd the three bytes,
/// 0xe..0x11 0, 0x12 the match fixture.
pub(crate) fn squad_numbers_plan(p0: usize, p1: usize, kind: usize, flag: usize, bytes: [usize; 3], fixture: usize) -> [usize; 0x13] {
    [p0, p1, kind, flag, 0x41, 8, 6, 0, 0, usize::MAX, usize::MAX, bytes[0], bytes[1], bytes[2], 0, 0, 0, 0, fixture]
}

/// Opens the squad numbers screen (builder 0x475520, handler 0x47a9e0, leave 0x47c070) with its 19 params (made
/// once the screen is open).
pub(crate) unsafe fn open_squad_numbers_with(plan: impl FnOnce() -> [usize; 0x13]) -> bool {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x475520usize, 0x47a9e0usize, 1, 0x47c070usize, 0) == 0 {
        return false;
    }
    for (k, v) in plan().iter().enumerate() {
        set_screen_param(k as u32, *v);
    }
    true
}

/// A player of the squad copy: present and made by the database (his id below `G_STAFF_COUNT`, not a regen).
pub(crate) unsafe fn squad_numbers_in_database(p: usize) -> bool {
    p != 0 && read_i32(p) < read_i32(G_STAFF_COUNT)
}

/// The squad number `p` holds at `club` (`exe::squad_manager_get_squad_number`, signed: negative = provisional).
pub(crate) unsafe fn squad_numbers_number(p: usize, club: usize) -> i8 {
    exe::squad_manager_get_squad_number(read_usize(G_SQUAD_MANAGER), p, club)
}

/// Gives `p` the squad number `n` at `club` (`exe::squad_manager_set_squad_number`; the callee reads the byte only).
pub(crate) unsafe fn squad_numbers_set(p: usize, club: usize, n: i8) {
    exe::squad_manager_set_squad_number(read_usize(G_SQUAD_MANAGER), p, club, n as u8 as u32);
}

/// The first player of the squad copy (`squad`, 50 slots, only database players) whose number, as an absolute value,
/// is `n`; 0 when none.
pub(crate) unsafe fn squad_numbers_holder(squad: usize, club: usize, n: i32) -> usize {
    for k in 0..SQUAD_NUMBERS_SLOTS {
        let p = read_usize(squad + 4 * k);
        if squad_numbers_in_database(p) && (squad_numbers_number(p, club) as i32).wrapping_abs() == n {
            return p;
        }
    }
    0
}

/// The club's squad copied into a 200-byte block (`malloc`, `exe::database_collect_club_squad`); 0 and the error box
/// of club_screens.cpp at `line` when the allocation fails.
pub(crate) unsafe fn squad_numbers_collect(club: usize, line: u32) -> usize {
    let block = exe::crt_malloc(200);
    if block == 0 {
        show_error_at(SQUAD_NUMBERS_SOURCE_FILE, line);
        return 0;
    }
    exe::database_collect_club_squad(club, block as *mut usize, 0);
    block
}

/// The "Please Confirm" / "Incomplete" box of the squad numbers screen: the heading `heading` and the title `title`
/// formatted (langlib) into two 200-byte buffers with the original's arguments (the text, the buttons, the club's kit
/// colours and the result code), then `confirm_push` (0x591840) with the formatter's answers.
pub(crate) unsafe fn squad_numbers_confirm(title: usize, heading: usize, text: usize, buttons: u32, colours: (u32, u32), code: u32) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (mut heading_buf, mut title_buf) = ([0u8; 0xc8], [0u8; 0xc8]);
    let h = lang(heading_buf.as_mut_ptr() as usize, heading, text, buttons, colours.0, colours.1, code);
    let t = lang(title_buf.as_mut_ptr() as usize, title, h, text, buttons, colours.0, colours.1, code);
    exe::confirm_push(t, h, text, buttons, colours.0, colours.1, code);
}

/// The club's first kit colours (`exe::database_get_club_kit_colours`, kit 1) into two locals (the original's are
/// uninitialised stack slots; here zero).
pub(crate) unsafe fn squad_numbers_kit_colours(club: usize) -> (u32, u32) {
    let (mut a, mut b) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut a, &mut b);
    (a, b)
}

/// A colour word of the interface palette, zero-extended (the original pushes it in a register whose high half is
/// whatever the register held: garbage that `Display` keeps out, since it stores the colours as words).
pub(crate) unsafe fn part_exchange_colour(a: usize) -> u32 {
    read_u16(a) as u32
}

/// The game's text formatter (`FN_LANGLIB_SPRINTF`), variadic.
pub(crate) unsafe fn part_exchange_lang() -> unsafe extern "C" fn(usize, usize, ...) -> usize {
    core::mem::transmute(FN_LANGLIB_SPRINTF)
}

/// `LLIST::LLIST` (0x68d020) and `LLIST::~LLIST` (0x68d040), thiscall, on a local list.
pub(crate) unsafe fn part_exchange_llist(l: &mut [u8; 0x20], construct: bool) {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(if construct { FN_LLIST_LLIST } else { FN_LLIST_DESTRUCTOR_LLIST });
    f(l.as_mut_ptr() as usize);
}

/// The parameters, the title and the bid text, and the menus of the part exchange screen, in the original's order.
///
/// The club is the club table's bid +8, the player the person table's bid +0xc; his contract
/// (`exe::contract_manager_get_contract`) is written to [0xae1080] (`G_CONTRACT_LOOKUP_RESULT`, as the original) and its
/// club (+4) is the seller. The player's value × 1.2 less each offered player's value is worked out and never used, as
/// the original. The player's background picture (`background_set_staff_picture`, 0x41c790). The club's kit colours
/// (kit 1) as the title colours (`exe::screenmanager_add_screen_title` with the club's short name). The text "Add Player
/// To <player> Transfer Bid" (his name in style 5, a link 0x7d7 to him) wrapped (`Text::draw_wrapped`, 0x4aeaf0, font 6)
/// in an area 0x64..0x316 by 0x50..0x7d. The menu bar (`exe::menubar_build` 4). The view bar (0x6e..0x168 by 0x91..0xa5)
/// with "View" (link 4): Traditional 1 (title "Position(s)"), Contract 2 ("Contract Info"), Selection 0x1000 ("Selection
/// Info"), Stats 4, More Stats 8, Attributes 0x10, Other 0x20 ("Other Info"); in the Traditional view "Sort By" (link 6,
/// the ticked one's text without the first space as the title): Name 4, Position(s) 8, Squad Number 0x20, Form 0x80,
/// Morale 0x100, Condition 0x200, Nationality 0x2000, Age 0x4000, Int. Caps 0x8000, Int. Goals 0x10000, Basic Wage
/// 0x20000, Contract Expiry 0x40000, Contract Protected 0x20000000, Value 0x80000, a separator, Goals 0x100000, Conceded
/// 0x200000, Assists 0x400000, Av. Rating 0x800000; with Stats or More Stats "Competitions" (link 5): the six categories
/// (`player_stats_get_category_name`, 0x7c14b0), the one of param 6 ticked with "<category> Stats" as the title; with
/// Attributes "Attributes" (link 4): Physical 0x40, Mental 0x80, Goalkeeping 0x100, Defensive 0x200, Attacking 0x400,
/// each ticked with its "... Attributes" title. The "Exchange" button (0x294..0x311 by 4..0x18, the kit colours,
/// flags 0x70 and data 0 while choosing, else 0x30 and 1). The "Filter" button (0x28f..0x30c by 0x91..0xa5, link 9):
/// All Positions 1, Goalkeepers 2, Defenders 4, Midfielders 8, Attackers 0x10, a separator, All Sides 0x20, Left Sided
/// 0x40, Central 0x80, Right Sided 0x100, a separator. The title (font 3) at 0x6e..0x30c by 0xaa..0xcd.
pub(crate) unsafe fn part_exchange_header(s: &mut PartExchange, text: usize) {
    let sm = G_SCREEN_MANAGER;
    s.bid = get_screen_param(0);
    s.choosing = get_screen_param(3) as u32;
    s.view = get_screen_param(4) as u32;
    s.view_flags = s.view;
    s.sort = get_screen_param(5) as u32;
    s.category = get_screen_param(6) as u8;
    s.option7 = get_screen_param(7) as u8;
    s.option8 = get_screen_param(8) as u8;
    s.filter = get_screen_param(9) as u32;
    s.club = club_screens_club(read_i32(s.bid + 8));
    s.player = club_screens_staff(read_i32(s.bid + 0xc));
    let contract = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), s.player));
    write_usize(G_CONTRACT_LOOKUP_RESULT, contract.addr());
    s.seller = if !contract.is_null() { club_screens_club(contract.club() as i32) } else { 0 };
    s.human = tel26_slot_manager().addr();
    // the bid's budget (value × 1.2, less the offered players' values later): worked out and never read, as the original
    let _budget = read_i32(s.player + staff::VALUE) as f64 * read_f64(rdata::F64_1_2);
    let picture: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_BACKGROUND_SET_STAFF_PICTURE);
    picture(s.player, 0, 0);
    exe::database_get_club_kit_colours(s.club, 1, &mut s.kit_text, &mut s.kit_back);
    exe::screenmanager_add_screen_title(sm, s.kit_back, s.kit_text, s.club + club::SHORT_NAME);
    exe::database_format_staff_name(s.player, s.name.as_mut_ptr() as usize, 0x12c, 5);
    part_exchange_lang()(0xde1f64, 0x991494, 100u32, s.name.as_ptr()); // "Add Player To <%s ...>{} Transfer Bid"
    exe::text_set_string(text, 0xde1f64 as *const u8);
    exe::text_add_link(text, s.name.as_ptr(), 0x7d7, s.player);
    let mut line = 0i32;
    let a = s.add_area([0x64, 0x50, 0x316, 0x7d, 1, 0, 1, 0, 2, 0, u32::MAX]);
    let wrapped: unsafe extern "thiscall" fn(usize, u32, u32, u32, *mut i32, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_TEXT_DRAW_WRAPPED);
    wrapped(text, a, 0x64, 0x316, &mut line, 6, part_exchange_colour(0xaebdec), 1, 0);
    exe::menubar_build(4, 0);
    s.title[0] = 0;
    s.bar = s.add_area([0x6e, 0x91, 0x168, 0xa5, 2, 0, 1, 0, 1, 0, u32::MAX]);
    squad_list_menus(s.bar, s.view, s.sort, s.category, s.title.as_mut_ptr(), [4, 6, 5, 4], 0x98edb8);
    let lang = part_exchange_lang();
    lang(0xde1f64, 0x991488); // "Exchange"
    let flags = if s.choosing != 0 { 0x70 } else { 0x30 };
    tel26_add_button([0x294, 4, 0x311, 0x18, flags, s.kit_text, 0xc, 1, s.kit_back, 0xde1f64, 1, (s.choosing == 0) as u32]);
    lang(0xde1f64, 0x98ebf8); // "Filter"
    let filter_button = tel26_add_button([0x28f, 0x91, 0x30c, 0xa5, 0x30, part_exchange_colour(0xae317e), 0xc, 1, part_exchange_colour(0xae3184), 0xde1f64, 0, 0]);
    let filter_menu = s.add_area([0, 0, 0, 0, 0, 0, 0, 0, 0x130, part_exchange_colour(0xae31aa), filter_button]);
    for (label, bit) in [(0x98ebe8usize, 1u32), (0x98ebd8, 2), (0x98ebcc, 4), (0x98ebbc, 8), (0x98ebb0, 0x10)] {
        lang(0xde1f64, label);
        squad_list_menu_item(filter_menu, s.filter & bit != 0, 9, bit);
    }
    squad_list_separator(filter_menu);
    for (label, bit) in [(0x98eba4usize, 0x20u32), (0x98eb98, 0x40), (0x98eb8c, 0x80), (0x98eb7c, 0x100)] {
        lang(0xde1f64, label);
        squad_list_menu_item(filter_menu, s.filter & bit != 0, 9, bit);
    }
    squad_list_separator(filter_menu);
    let manager_text: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    manager_text(sm, 0x6e, 0xaa, 0x30c, 0xcd, 2, 0, 0xc, 3, part_exchange_colour(0xaebdec) as usize, s.title.as_ptr() as usize, 1);
}

/// The players that can be offered: the bidding club's squad filtered and sorted, less the ones already offered.
///
/// The club's list (`build_filtered_sorted_squad_list`, 0x4841d0: the club, no reserves, the sort, style 3, params 6..8,
/// the filter). For each person already offered (the bid's list +4, staff ids), the bid's budget loses his value and his
/// entry in the club's list is emptied. Then each player of the list made by the database, with a player record, without
/// a loan contract nor a contract of type 5, a player by job (`staff::JOB_FOR_CLUB` 0xb), with no transfer offer
/// (`TRANSFER_MANAGER::get_offers_for_player`) and no exchange offer or negotiation, or with exactly one exchange offer
/// (or else exactly one exchange negotiation) that is empty or belongs to this bid (+0xc, the bid's player): when the
/// club may bid for him at the seller (`TRANSFER_MANAGER::can_club_bid_for_player`, 1), for the human's own club with a
/// computer seller that does not want him (`SHORTLIST_MANAGER::club_wants_player` of the transfer manager's +0x856) he
/// counts as "not interested"; otherwise his desired wage at the seller (`STAFF_CONTRACT::set_to_desired_contract`, -1,
/// status 2, 0xb) above 1.5 × the seller's maximum wage (`Finance::get_max_wage`, 2, of
/// `FinanceTable::get_club_finance`) counts as "too dear", else he becomes a candidate.
pub(crate) unsafe fn part_exchange_candidates(s: &mut PartExchange) {
    let build: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, usize, usize, usize, u32, u32, u32) -> u32 =
        core::mem::transmute(FN_BUILD_FILTERED_SORTED_SQUAD_LIST);
    let squad = s.squad.as_mut_ptr() as usize;
    // simplification: params 6..8 are bytes in stack slots whose upper bytes are garbage in the original (chars)
    build(s.club, 0, s.sort, 3, s.category as u32, s.option7 as u32, s.option8 as u32, s.filter, squad, 0, 0, 0, 0, 0);
    s.squad_head = read_u32(squad);
    let offered = read_usize(s.bid + 4);
    if offered != 0 {
        part_exchange_rewind(offered);
        loop {
            let e = exe::llist_get_next(offered);
            if e == 0 {
                break;
            }
            let id = read_u32(e);
            part_exchange_rewind(squad);
            loop {
                let f = exe::llist_get_next(squad);
                if f == 0 {
                    break;
                }
                let p = read_usize(f);
                if p != 0 && read_u32(p) == id {
                    write_usize(f, 0);
                    break;
                }
            }
        }
    }
    let tm = || read_usize(G_TRANSFER_MANAGER);
    let (offers, negotiations) = (s.offers.as_mut_ptr() as usize, s.negotiations.as_mut_ptr() as usize);
    let contract = s.contract.as_mut_ptr() as usize;
    let seller = s.seller;
    part_exchange_rewind(squad);
    loop {
        let f = exe::llist_get_next(squad);
        if f == 0 {
            break;
        }
        let p = read_usize(f);
        if p == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) || read_usize(p + staff::PLAYER) == 0 {
            continue;
        }
        if exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) != 0 {
            continue;
        }
        if exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p) != 0
            && read_u8(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p) + contract::CONTRACT_TYPE) & 0x3f == 5 {
            continue;
        }
        if read_u8(p + staff::JOB_FOR_CLUB) != 0xb {
            continue;
        }
        exe::llist_clear(offers);
        exe::llist_clear(negotiations);
        let free = exe::transfer_manager_get_offers_for_player(tm(), p, 0usize, 0) == 0
            && exe::transfer_manager_get_exchange_offers_for_player(tm(), p, offers) == 0
            && exe::transfer_manager_get_exchange_negotiations_for_player(tm(), p, negotiations) == 0;
        if !free {
            // one exchange offer (or else one negotiation) that is empty or this bid's
            let (list, slot_of) = if read_u16(offers) == 1 {
                (offers, 0x8c61e0usize)
            } else if read_u16(negotiations) == 1 {
                (negotiations, 0x8c6200usize)
            } else {
                continue;
            };
            part_exchange_rewind(list);
            let e = exe::llist_get_next(list);
            if e == 0 {
                continue;
            }
            let slot_fn: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(slot_of);
            let slot = slot_fn(tm(), read_u32(e));
            if slot != 0 && read_usize(slot) != 0 && read_u32(s.bid + 0xc) != read_u32(read_usize(slot) + 0xc) {
                continue;
            }
        }
        if exe::transfer_manager_can_club_bid_for_player(tm(), p, seller, core::ptr::null_mut(), 1) == 0 {
            continue;
        }
        if read_usize(s.human + staff::CLUB_JOB) == s.club && exe::human_manager_is_human_club(seller, 0, 1, 0, 0) == 0
            && exe::shortlist_manager_club_wants_player(read_usize(tm() + 0x856), p, seller, 0, 0) == 0 {
            covered!("tel05_2 not interested");
            s.not_interested = s.not_interested.wrapping_add(1);
            continue;
        }
        let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), seller);
        let max_wage = exe::finance_get_max_wage(finance, p, 2);
        let limit = ftol(max_wage as f64 * read_f64(rdata::F64_1_5));
        if exe::staff_contract_set_to_desired_contract(contract, p, seller, -1, 2, 0xb, 0, 0) > limit {
            covered!("tel05_2 too dear");
            s.too_dear = s.too_dear.wrapping_add(1);
        } else {
            let cell = p;
            exe::llist_add_tail(s.candidates.as_mut_ptr() as usize, &cell as *const usize as usize, 4);
        }
    }
}

/// The list's area and the column headings.
///
/// Traditional view: one area 0x6e..0x30c by 0xd2..0x217 with 8 columns (two players per row) and the rows that fit in
/// font 3 (0x135 / the font's height; the row count dword keeps the filter's upper bytes, as the original). Other views:
/// the background area, the rows that fit in font 2, their height (309 / rows + 0.5, `ftol`); the headings in an area
/// from 0xda, one row high, 0x2f6 wide when there are more candidates than rows (the scroll bar), else 0x30c: "Inf" 2,
/// "   Name" 4 (link 6) and the view's columns: contract (Squad Status 0x10, Basic Wage 0x20000, Contract Expiry 0x40000,
/// Releases 0x2000000); selection (Position 8, Form 0x80, Morale 0x100, Cond. 0x200, the stat fields 1 and 0x11 as
/// 0x10000000 and 0x800000); statistics (the fields of [0x98de84] or, More Stats, [0x98de90], link 7); attributes (the
/// table of the chosen group, sorted in place by qsort with 0x487720, link 8; with no group chosen the view flags
/// themselves are taken as the table, as the original); other (Nat. 0x1000, Age 0x4000, Caps 0x8000, Goals 0x10000, Form,
/// Morale, Cond.); "Value" 0x80000 in column 0xc. Then the list area below to 0x20f, its row count the rows less one in
/// the low byte of the last heading's return (as the original). No view: the view bar.
pub(crate) unsafe fn part_exchange_list_area(s: &mut PartExchange, w: &mut PartExchangeWeights) -> u32 {
    let lang = part_exchange_lang();
    let v = s.view;
    if v & 1 != 0 {
        covered!("tel05_2 traditional view");
        let rows = (0x135 / exe::font_get_height(3)) as u8;
        s.filter = (s.filter & 0xffff_ff00) | rows as u32;
        return s.add_area([0x6e, 0xd2, 0x30c, 0x217, 8, w.two.as_ptr() as u32, s.filter, 0, 2, 0, u32::MAX]);
    }
    s.add_area([0x6e, 0xd2, 0x30c, 0x217, 1, 0, 1, 0, 2, 0, u32::MAX]);
    let rows = (0x135 / exe::font_get_height(2)) as u8;
    s.filter = (s.filter & 0xffff_ff00) | rows as u32;
    let row_h = ftol(read_f32(rdata::F32_309) as f64 / rows as u32 as f64 + read_f64(rdata::F64_0_5)); // 309.0 / rows + 0.5
    let count = read_i16(s.candidates.as_ptr() as usize) as i32;
    let x2 = if count > rows as i32 - 1 { 0x2f6 } else { 0x30c };
    let head = |s: &PartExchange, weights: *const u8| s.add_area([0x6e, 0xda, x2, (row_h + 0xd9) as u32, 0xd, weights as u32, 1, 0, 1, 0, u32::MAX]);
    let first_two = |s: &PartExchange, area: u32| {
        lang(0xde1f64, 0x98ea64); // "Inf"
        s.heading(area, 1, 0xc, 6, 2);
        lang(0xde1f64, 0x98ea5c); // "   Name"
        s.heading(area, 2, 1, 6, 4);
    };
    let value_heading = |s: &PartExchange, area: u32| -> u32 {
        lang(0xde1f64, 0x98ea0c); // "Value"
        s.heading(area, 0xc, 0xc, 6, 0x80000)
    };
    let (weights, last): (*const u8, u32);
    if v & 2 != 0 {
        covered!("tel05_2 contract view");
        weights = w.contract.as_ptr();
        let area = head(s, weights);
        first_two(s, area);
        for (col, label, data) in [(3u32, 0x98ea4cusize, 0x10u32), (4, 0x98ea34, 0x20000), (5, 0x98ea20, 0x40000), (6, 0x98ea14, 0x2000000)] {
            lang(0xde1f64, label);
            s.heading(area, col, 1, 6, data);
        }
        last = value_heading(s, area);
    } else if v & 0x1000 != 0 {
        covered!("tel05_2 selection view");
        weights = w.selection.as_ptr();
        let area = head(s, weights);
        first_two(s, area);
        lang(0xde1f64, 0x98ea00); // "Position"
        s.heading(area, 3, 1, 6, 8);
        for (col, label, data) in [(4u32, 0x98e9f8usize, 0x80u32), (5, 0x98e9f0, 0x100), (6, 0x98e9e8, 0x200)] {
            lang(0xde1f64, label);
            s.heading(area, col, 0xc, 6, data);
        }
        exe::player_stats_get_field_label(read_usize(G_PLAYER_STATS), 1, 0xde1f64, 0x7d0);
        s.heading(area, 7, 0xc, 6, 0x10000000);
        exe::player_stats_get_field_label(read_usize(G_PLAYER_STATS), 0x11, 0xde1f64, 0x7d0);
        s.heading(area, 8, 0xc, 6, 0x800000);
        last = value_heading(s, area);
    } else if v & 4 != 0 || v & 8 != 0 {
        covered!("tel05_2 stats view");
        weights = w.stats.as_ptr();
        let area = head(s, weights);
        first_two(s, area);
        let table = if v & 4 != 0 { 0x98de84 } else { 0x98de90 };
        let mut k = 0usize;
        while read_u8(table + k) != 0xff {
            let field = read_u8(table + k);
            // simplification: the field goes in a register whose upper bytes are garbage in the original (a char)
            exe::player_stats_get_field_label(read_usize(G_PLAYER_STATS), field as u32, 0xde1f64, 0x7d0);
            s.heading(area, k as u32 + 3, 0xc, 7, field as i8 as i32 as u32);
            k += 1;
        }
        last = value_heading(s, area);
    } else if v & 0x10 != 0 {
        covered!("tel05_2 attributes view");
        weights = w.attributes.as_ptr();
        let area = head(s, weights);
        first_two(s, area);
        // the slot of the view flags takes the table of the chosen group (none: the flags stay, used as the table)
        let mut table = s.view as usize;
        for (bit, t) in [(0x40u32, 0x98de9cusize), (0x80, 0x98dea8), (0x100, 0x98deb4), (0x200, 0x98dec0), (0x400, 0x98decc)] {
            if v & bit != 0 {
                table = t;
            }
        }
        s.view = table as u32;
        exe::crt_qsort(table, 9, 1, 0x487720usize);
        let mut k = 0usize;
        while read_u8(table + k) != 0xff {
            let attr = read_u8(table + k);
            exe::database_get_player_attribute_name(attr as u32, 0xde1f64, 0x7d0, 1);
            s.heading(area, k as u32 + 3, 0xc, 8, attr as i8 as i32 as u32);
            k += 1;
        }
        last = value_heading(s, area);
    } else if v & 0x20 != 0 {
        covered!("tel05_2 other view");
        weights = w.other.as_ptr();
        let area = head(s, weights);
        first_two(s, area);
        for (col, label, data) in [(3u32, 0x98e9e0usize, 0x1000u32), (4, 0x98e9dc, 0x4000), (5, 0x98e9d4, 0x8000), (6, 0x98e9cc, 0x10000),
                                   (7, 0x98e9f8, 0x80), (8, 0x98e9f0, 0x100), (9, 0x98e9e8, 0x200)] {
            lang(0xde1f64, label);
            s.heading(area, col, 0xc, 6, data);
        }
        last = value_heading(s, area);
    } else {
        // no view of its own: the rows go to the view bar (ebx in the original)
        return s.bar;
    }
    s.add_area([0x6e, (row_h + 0xda) as u32, 0x30c, 0x20f, 0xd, weights as u32, (last & 0xffff_ff00) | rows.wrapping_sub(1) as u32, 0, 1, 0, u32::MAX])
}

/// The rows, one per candidate, in the area `area`.
///
/// Each player's training caught up (`exe::training_manager_update_player_training`); column 0 an empty text; column 1
/// his status (`get_player_status_code`, 0x4849f0, with no fixture) as an item (flags 0x202, the colours, link and data
/// by the status): Wpm, Cup, Ine, Fgn and Sct link 0x7db; international duty with the duty club (link 0x7d5), or an item
/// 0x201 with no link and whatever text 0xde1f64 holds (as the original); Sus 0x7d9; Inj 0x7d8; Inj (8) and Tir 0x7d8 in
/// the tired colour; Yel 0x7d9 in the colour of `exe::graphics_rgb_to_pixel`(0xb4, 0xb4, 0, 0); Ctr 0x7da; Trn, Bid, Wnt,
/// Req, Lst, Loa, Ret, Bos, Unh, Fut 0x7db. Column 2 his name (style 4 and font 3 in the Traditional view, else 1 and 2)
/// as a button (link 2 while choosing, else 0x7d7 to him). The Traditional view: column 3 by the sort (`squad_list_sort_cell`),
/// two players per row (columns 0 and 4); the others: the view's columns (`squad_list_view_cells`) and the value in
/// column 0xc (a button, link 0xa, for a database player when the human runs the club), one player per row.
pub(crate) unsafe fn part_exchange_rows(s: &mut PartExchange, area: u32, _w: &PartExchangeWeights) {
    let list = s.candidates.as_mut_ptr() as usize;
    let (mut col, mut row) = (0u32, 0u32);
    let traditional = s.view_flags & 1 != 0;
    let mut buf = [0u8; 0x17c]; // the name buffer of the original (0x17c bytes at [esp+0x1fc])
    let rules = SquadListRules { club: s.club, sort: s.sort, view_flags: s.view_flags, category: s.category, table: s.view as usize, squad_screen: false };
    part_exchange_rewind(list);
    loop {
        let e = exe::llist_get_next(list);
        if e == 0 {
            break;
        }
        s.shown = s.shown.wrapping_add(1);
        let player = read_usize(e);
        exe::training_manager_update_player_training(read_usize(G_TRAINING_MANAGER), player, 0);
        tel26_area_text([area, col, row, 0x30, part_exchange_colour(0xaebe04), 0xc, 1, part_exchange_colour(0xaebdec), 0]);
        squad_list_status(&rules, area, player, col, row);
        let contract_club = if player == 0 || read_i32(player) < read_i32(G_STAFF_COUNT) {
            exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, s.club)
        } else {
            0
        };
        let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
        let (style, font) = if traditional { (4, 3) } else { (1, 2) };
        exe::database_format_staff_name(player, 0xde1f64usize, 0x7d0, style);
        tel26_area_button([area, col + 2, row, 1, 0, 1, font, part_exchange_colour(0xae3184), 0xde1f64, if s.choosing != 0 { 2 } else { 0x7d7 },
            player as u32]);
        let injury = exe::injury_get_player_injury_status(read_usize(G_INJURY), player);
        let b = buf.as_mut_ptr() as usize;
        if traditional {
            squad_list_sort_cell(&rules, area, player, col, row, injury, contract_club, contract, b);
            col += 4;
            if col > 4 {
                row += 1;
                col = 0;
            }
        } else {
            squad_list_view_cells(&rules, area, player, col, row, injury, contract_club, contract, b);
            part_exchange_value(player);
            let (c3192, c3184) = (part_exchange_colour(0xae3192), part_exchange_colour(0xae3184));
            if (player == 0 || read_i32(player) < read_i32(G_STAFF_COUNT)) && exe::human_manager_current_controls_club(s.club, 0, 1) != 0 {
                tel26_area_button([area, col + 0xc, row, 0x30, c3192, 0xc, 1, c3184, 0xde1f64, 0xa, player as u32]);
            } else {
                tel26_area_text([area, col + 0xc, row, 0x30, c3192, 0xc, 1, c3184, 0xde1f64]);
            }
            row += 1;
        }
    }
}

/// The end of the screen: when no candidate was shown and the bid offers nobody yet while the club's list was not empty,
/// the reason as a wrapped text in the list's place ("<seller> are not interested in a part-exchange deal." or the
/// wage demands one, with the clubs' names linked, both to the seller, as the original; `gui_add_text_area_from_text`,
/// font 3), otherwise the text buffer emptied. The list area's scroll row from param 1 (`Area::set_scroll_row`), the area
/// written to param 2, and the bottom bar (0x64..0x316 by 0x22b..0x24e, weights 3 and 1): "Back" (link 3) and "Next"
/// (link -3, or a greyed text when there is no next screen).
pub(crate) unsafe fn part_exchange_footer(s: &mut PartExchange, area: u32) {
    let lang = part_exchange_lang();
    let mut said = false;
    if s.shown == 0 && read_usize(s.bid + 4) == 0 && s.squad_head as u16 != 0 {
        let seller = Club(s.seller);
        let gender = |c: usize| read_i8(c + club::SHORT_GENDER_NAME) as i32;
        if s.not_interested != 0 {
            lang(0xde1f64, 0x991438, gender(seller.addr()), gender(seller.addr()), seller.addr() + club::SHORT_NAME);
            said = true;
        } else if s.too_dear != 0 {
            lang(0xde1f64, 0x9913c8, gender(s.club), gender(s.club), gender(seller.addr()), gender(seller.addr()), s.club + club::SHORT_NAME, seller.addr() + club::SHORT_NAME);
            said = true;
        }
        if said && read_u8(0xde1f64) != 0 {
            covered!("tel05_2 nobody to offer");
            let mut text = [0usize; 4];
            exe::new_text(text.as_mut_ptr(), 0xde1f64usize);
            let t = text.as_mut_ptr() as usize;
            exe::text_add_link(t, (seller.addr() + club::SHORT_NAME) as *const u8, 0x7d5, seller);
            exe::text_add_link(t, (s.club + club::SHORT_NAME) as *const u8, 0x7d5, seller);
            let lines = 0x145 / exe::font_get_height(3);
            exe::gui_add_text_area_from_text(0x6e, 0xd2, 0x30c, 0x217, 1, 0, lines, 3, part_exchange_colour(0xae3184), t);
            exe::drop_text(text.as_mut_ptr());
        }
    }
    if !said {
        write_u8(0xde1f64, 0);
    }
    let scroll = get_screen_param(1);
    if area as u16 != 0xffff {
        let d = s.display();
        exe::area_set_scroll_row(d.wrapping_add(4).wrapping_add((area as i16 as isize as usize).wrapping_mul(0xbf1)), scroll);
    }
    set_screen_param(2, area as i16 as i32 as usize);
    let weights = [3u8, 1];
    let bottom = s.add_area([0x64, 0x22b, 0x316, 0x24e, 2, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    let (c317e, c3184) = (part_exchange_colour(0xae317e), part_exchange_colour(0xae3184));
    lang(0xde1f64, 0x9880a8); // "Back"
    tel26_area_button([bottom, 0, 0, 0x30, c317e, 0xc, 3, c3184, 0xde1f64, 3, 0]);
    lang(0xde1f64, 0x9880a0); // "Next"
    if exe::screenmanager_is_forward_unavailable(G_SCREEN_MANAGER) == 0 {
        tel26_area_button([bottom, 1, 0, 0x30, c317e, 0xc, 3, c3184, 0xde1f64, 0xffff_fffd, 0]);
    } else {
        tel26_area_text([bottom, 1, 0, 0x30, c317e, 0x2c, 3, c3184, 0xde1f64]);
    }
}

/// The parameters, the squad copy, the title and the menus of the squad numbers screen.
///
/// Without a squad copy (param 0x11) the club's squad is copied into a 200-byte block (`database_collect_club_squad`;
/// the "Error" box at line 0x398a and nothing else when it cannot be allocated, returns false) written to param 0x11; in
/// the European mode (2) a player on loan (`CONTRACT_MANAGER::get_loan_contract`) or whose contract
/// (`CONTRACT_MANAGER::get_contract`, kept in [0xae1080] as the original) is of type 5 leaves the copy. The club's
/// background picture (`background_set_club_picture`). The title "<club> Squad Numbers" or "<club> European Squad" in the
/// club's kit colours (locals the original leaves uninitialised; here zero) and the subtitle "Please Select"
/// (`ScreenManager::add_screen_subtitle`, 0x803f80). The menu bar (`exe::menubar_build` 4), the view bar (0x6e..0x168 by
/// 0x91..0xa5) and its menus (`squad_list_menus`, links 1, 6, 2 and 1; "Club" instead of "Nationality" for a national or
/// all-star team), the Auto (link 3) and Clear (link 4) buttons (0x212..0x30c) and the title (font 3, 0x6e..0x30c by
/// 0xaa..0xcd).
pub(crate) unsafe fn squad_numbers_build_header(s: &mut SquadNumbersScreen) -> bool {
    let sm = G_SCREEN_MANAGER;
    s.club = read_usize(get_screen_param(0) + staff::CLUB_JOB);
    s.mode = get_screen_param(2) as u8;
    s.view = get_screen_param(4) as u32;
    s.sort = get_screen_param(5) as u32;
    s.category = get_screen_param(6) as u8;
    s.option9 = get_screen_param(9) as u8;
    s.slot_a = get_screen_param(0xa) as u8 as usize;
    s.rule_b = get_screen_param(0xb) as u8;
    s.squad_size = get_screen_param(0xc) as u8;
    s.rule_d = get_screen_param(0xd) as u8;
    s.registered = get_screen_param(0xe) as u8;
    s.reserves = get_screen_param(0xf) as u8;
    s.squad = get_screen_param(0x11);
    s.fixture = get_screen_param(0x12);
    let club = Club(s.club);
    if s.fixture != 0 {
        let side = if club.addr() == read_usize(s.fixture + fixture::HOME_CLUB) { fixture::HOME_SIDE_MARK } else { fixture::AWAY_SIDE_MARK };
        s.side = (read_u8(s.fixture + side) == 0) as u32;
    }
    if s.squad == 0 {
        let block = exe::crt_malloc(200);
        s.squad = block;
        if block == 0 {
            show_error_at(SQUAD_NUMBERS_SOURCE_FILE, 0x398a);
            return false;
        }
        exe::database_collect_club_squad(club, block as *mut usize, 0);
        set_screen_param(0x11, block);
        if s.mode == 2 {
            covered!("tel05_3 european squad copy");
            for k in 0..SQUAD_NUMBERS_SLOTS {
                let p = read_usize(block + 4 * k);
                if p == 0 {
                    continue;
                }
                if exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) != 0 {
                    write_usize(block + 4 * k, 0);
                    continue;
                }
                let contract = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p));
                write_usize(G_CONTRACT_LOOKUP_RESULT, contract.addr());
                if !contract.is_null() && contract.contract_type() & 0x3f == 5 {
                    write_usize(block + 4 * k, 0);
                }
            }
        }
    }
    exe::background_set_club_picture(club, 0, 0);
    let lang = part_exchange_lang();
    let gender = (club.short_gender_name() as i8) as i32;
    // "{}<%s - club>{} Squad Numbers" / "{}<%s - club>{} European Squad"
    lang(0xde1f64, if s.mode == 1 { 0x9911f8 } else { 0x9911d8 }, gender, gender, club.addr() + club::SHORT_NAME);
    let (mut kit_a, mut kit_b) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut kit_a, &mut kit_b);
    exe::screenmanager_add_screen_title(sm, kit_b, kit_a, 0xde1f64usize);
    lang(0xde1f64, 0x9911c8); // "Please Select"
    exe::screenmanager_add_screen_subtitle(sm, 0xde1f64, 1);
    exe::menubar_build(4, 0);
    s.title[0] = 0;
    let d = screens_current_display(sm);
    let bar = tel26_add_area(d, [0x6e, 0x91, 0x168, 0xa5, 2, 0, 1, 0, 1, 0, u32::MAX]);
    let club_label = if s.view & 1 != 0 && exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), club) != 0 {
        0x98edb0 // " Club"
    } else {
        0x98edb8 // " Nationality"
    };
    squad_list_menus(bar, s.view, s.sort, s.category, s.title.as_mut_ptr(), [1, 6, 2, 1], club_label);
    let buttons = tel26_add_area(screens_current_display(sm), [0x212, 0x91, 0x30c, 0xa5, 2, 0, 1, 0, 1, 0, u32::MAX]);
    let (c317e, c3184) = (part_exchange_colour(0xae317e), part_exchange_colour(0xae3184));
    lang(0xde1f64, 0x9911a0, 0xad9c64usize); // "Auto"
    tel26_area_button([buttons, 0, 0, 0x30, c317e, 0xc, 1, c3184, 0xde1f64, 3, 0]);
    lang(0xde1f64, 0x991174, 0xad9c64usize); // "Clear"
    tel26_area_button([buttons, 1, 0, 0x30, c317e, 0xc, 1, c3184, 0xde1f64, 4, 0]);
    s.last_area = buttons;
    let manager_text: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    manager_text(sm, 0x6e, 0xaa, 0x30c, 0xcd, 2, 0, 0xc, 3, part_exchange_colour(0xaebdec) as usize, s.title.as_ptr() as usize, 1);
    true
}

/// The squad numbers grid (squad numbers mode) or the selected count (European mode), then the club's list.
///
/// Squad numbers: two areas of 25 columns (0x6e..0x30c by 0xd2..0xe6 and 0x203..0x217), the numbers 1..25 and 26..50:
/// each one free (flags 0x46, link 5, data the number, the pale colour), held for good (0x46, 0x70, white) or held
/// provisionally (an item 1 with font 0x2c and no link); the list between 0xeb and 0x1fe, filter 0x164c21. European: "No
/// Players Selected", "1 Player Selected" or "<n> Players Selected" (registered - reserves + rule byte 0xd) under the
/// list (0x6e..0x30c by 500..0x217); the list between 0xd2 and 0x1ef, filter 0x224c21. The list
/// (`build_filtered_sorted_squad_list`, 0x4841d0: the club, the sort, style 3, params 6, 9 and 0xa, the squad copy) and
/// its count written to param 0x10.
pub(crate) unsafe fn squad_numbers_build_top(s: &mut SquadNumbersScreen) {
    let sm = G_SCREEN_MANAGER;
    let filter;
    if s.mode == 1 {
        covered!("tel05_3 numbers grid");
        let d = screens_current_display(sm);
        let first = tel26_add_area(d, [0x6e, 0xd2, 0x30c, 0xe6, 0x19, 0, 1, 0, 1, 0, u32::MAX]);
        let second = tel26_add_area(screens_current_display(sm), [0x6e, 0x203, 0x30c, 0x217, 0x19, 0, 1, 0, 1, 0, u32::MAX]);
        let mut area = first;
        let mut col = 0u32;
        let (c3150, c3184, cbdec) = (part_exchange_colour(0xae3150), part_exchange_colour(0xae3184), part_exchange_colour(0xaebdec));
        for n in 1u32..=0x32 {
            let holder = squad_numbers_holder(s.squad, s.club, n as i32);
            part_exchange_sprintf()(0xde1f64, 0x98806c, n); // "%d"
            let f = if holder == 0 {
                [0x46, 0, 0, 0, 0, col, 0, 0x30, c3150, c3150, 0xc, 1, c3184, 0xde1f64, 0, 5, n, area]
            } else if squad_numbers_number(holder, s.club) > 0 {
                [0x46, 0, 0, 0, 0, col, 0, 0x70, c3150, c3150, 0xc, 1, cbdec, 0xde1f64, 0, 5, n, area]
            } else {
                [1, 0, 0, 0, 0, col, 0, 0x70, c3150, c3150, 0x2c, 1, cbdec, 0xde1f64, 0, 0, 0, area]
            };
            tel26_add_item(screens_current_display(sm), f);
            col += 1;
            if col >= 0x19 {
                area = second;
                col = 0;
            }
        }
        s.last_area = area;
        filter = 0x164c21;
        s.top = 0xeb;
        s.bottom = 0x1fe;
    } else {
        let count = s.registered as i8 as i32 - s.reserves as i8 as i32 + s.rule_d as i8 as i32;
        let lang = part_exchange_lang();
        if count == 0 {
            lang(0xde1f64, 0x991160); // "No Players Selected"
        } else if count == 1 {
            lang(0xde1f64, 0x99114c); // "1 Player Selected"
        } else {
            lang(0xde1f64, 0x99112c, count); // "<%d - number> Players Selected"
        }
        let manager_text: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
        manager_text(sm, 0x6e, 500, 0x30c, 0x217, 2, 0, 0xc, 3, part_exchange_colour(0xaebdec) as usize, 0xde1f64, 1);
        filter = 0x224c21;
        s.top = 0xd2;
        s.bottom = 0x1ef;
    }
    let build: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, usize, usize, usize, u32, usize, u32) -> u32 =
        core::mem::transmute(FN_BUILD_FILTERED_SORTED_SQUAD_LIST);
    // simplification: params 6, 9 and 0xa are bytes in stack slots whose upper bytes are garbage in the original (chars)
    build(s.club, 0, s.sort, 3, s.category as u32, s.option9 as u32, s.slot_a as u32, filter, s.list.as_mut_ptr() as usize, 0, 0, 0, s.squad, 0);
    set_screen_param(0x10, read_i16(s.list.as_ptr() as usize) as i32 as usize);
}

/// The list's area and the column headings (as `part_exchange_list_area`, between the top and the bottom of
/// `squad_numbers_build_top`).
///
/// Traditional view: 8 columns (two players per row), the rows that fit in font 3 in the low byte of the squad copy's
/// slot (as the original). Other views: the background area, the rows that fit in font 2 (into the slot of param 0xa)
/// and their height (the height over the rows + 0.5, `ftol`); the headings one row high from top + 8, 0x2f6 wide when the
/// list has more players than rows: "No." 0x20 (squad numbers) or "Reg" 0x40 (European), "Inf" 2, "   Name" 4 (link 6),
/// then the view's: contract ("Club" for a national or all-star team, else Squad Status 0x10, Basic Wage 0x20000,
/// Contract Expiry 0x40000, Releases 0x2000000 when not a national team, whose weights are 0x12, 0x12, 0x12, 0);
/// selection; statistics (link 7); attributes (link 8, the table of the chosen group sorted in place by qsort with
/// 0x487720; with none chosen the row count is taken as the table, as the original); other; "Value" 0x80000. The list area
/// from row height + top + 8 to bottom - 8. No view: the last area made.
pub(crate) unsafe fn squad_numbers_build_area(s: &mut SquadNumbersScreen, w: &mut PartExchangeWeights) -> u32 {
    let sm = G_SCREEN_MANAGER;
    let lang = part_exchange_lang();
    let v = s.view;
    let (top, bottom) = (s.top, s.bottom);
    let d = || screens_current_display(sm);
    if v & 1 != 0 {
        covered!("tel05_3 traditional view");
        let rows = ((bottom - top - 0x10) / exe::font_get_height(3)) as u8;
        s.squad = (s.squad & 0xffff_ff00) | rows as usize;
        return tel26_add_area(d(), [0x6e, top as u32, 0x30c, bottom as u32, 8, w.two.as_ptr() as u32, s.squad as u32, 0, 2, 0, u32::MAX]);
    }
    tel26_add_area(d(), [0x6e, top as u32, 0x30c, bottom as u32, 1, 0, 1, 0, 2, 0, u32::MAX]);
    let height = bottom - top - 0x10;
    let rows = (height / exe::font_get_height(2)) as u8;
    s.slot_a = rows as usize;
    let row_h = ftol(height as f64 / rows as u32 as f64 + read_f64(rdata::F64_0_5));
    let club = s.club;
    let heading = |area: u32, col: u32, font: u32, link: u32, data: u32| -> u32 {
        tel26_area_button([area, col, 0, 0x30, part_exchange_colour(0xae317e), font, 1, part_exchange_colour(0xae3184), 0xde1f64, link, data])
    };
    let count = read_i16(s.list.as_ptr() as usize) as i32;
    let x2 = if count > rows as i32 - 1 { 0x2f6 } else { 0x30c };
    let head = |weights: *const u8| tel26_add_area(d(), [0x6e, (top + 8) as u32, x2, (row_h + top + 7) as u32, 0xd, weights as u32, 1, 0, 1, 0, u32::MAX]);
    let mode = s.mode;
    let first_three = |area: u32| {
        if mode == 1 {
            lang(0xde1f64, 0x991128);
            heading(area, 0, 0xc, 6, 0x20);
        } else {
            lang(0xde1f64, 0x991124);
            heading(area, 0, 0xc, 6, 0x40);
        }
        lang(0xde1f64, 0x98ea64); // "Inf"
        heading(area, 1, 0xc, 6, 2);
        lang(0xde1f64, 0x98ea5c); // "   Name"
        heading(area, 2, 1, 6, 4);
    };
    let value_heading = |area: u32| {
        lang(0xde1f64, 0x98ea0c); // "Value"
        heading(area, 0xc, 0xc, 6, 0x80000);
    };
    let weights: *const u8;
    if v & 2 != 0 {
        covered!("tel05_3 contract view");
        if exe::database_is_national_team(club) != 0 {
            w.contract[3] = 0x12;
            w.contract[4] = 0x12;
            w.contract[5] = 0x12;
            w.contract[6] = 0;
        }
        weights = w.contract.as_ptr();
        let area = head(weights);
        first_three(area);
        let label = if exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), club) == 0 { 0x98ea4c } else { 0x98ea44 };
        lang(0xde1f64, label); // "   Squad Status" / "   Club"
        heading(area, 3, 1, 6, 0x10);
        lang(0xde1f64, 0x98ea34); // "   Basic Wage"
        heading(area, 4, 1, 6, 0x20000);
        lang(0xde1f64, 0x98ea20); // "   Contract Expiry"
        heading(area, 5, 1, 6, 0x40000);
        if exe::database_is_national_team(club) == 0 {
            lang(0xde1f64, 0x98ea14); // "   Releases"
            heading(area, 6, 1, 6, 0x2000000);
        }
        value_heading(area);
    } else if v & 0x1000 != 0 {
        weights = w.selection.as_ptr();
        let area = head(weights);
        first_three(area);
        lang(0xde1f64, 0x98ea00); // "Position"
        heading(area, 3, 1, 6, 8);
        for (col, label, data) in [(4u32, 0x98e9f8usize, 0x80u32), (5, 0x98e9f0, 0x100), (6, 0x98e9e8, 0x200)] {
            lang(0xde1f64, label);
            heading(area, col, 0xc, 6, data);
        }
        exe::player_stats_get_field_label(read_usize(G_PLAYER_STATS), 1, 0xde1f64, 0x7d0);
        heading(area, 7, 0xc, 6, 0x10000000);
        exe::player_stats_get_field_label(read_usize(G_PLAYER_STATS), 0x11, 0xde1f64, 0x7d0);
        heading(area, 8, 0xc, 6, 0x800000);
        value_heading(area);
    } else if v & 4 != 0 || v & 8 != 0 {
        weights = w.stats.as_ptr();
        let area = head(weights);
        first_three(area);
        let table = if v & 4 != 0 { 0x98de84 } else { 0x98de90 };
        let mut k = 0usize;
        while read_u8(table + k) != 0xff {
            let field = read_u8(table + k);
            exe::player_stats_get_field_label(read_usize(G_PLAYER_STATS), field as u32, 0xde1f64, 0x7d0);
            heading(area, k as u32 + 3, 0xc, 7, field as i8 as i32 as u32);
            k += 1;
        }
        value_heading(area);
    } else if v & 0x10 != 0 {
        covered!("tel05_3 attributes view");
        weights = w.attributes.as_ptr();
        let area = head(weights);
        first_three(area);
        for (bit, t) in [(0x40u32, 0x98de9cusize), (0x80, 0x98dea8), (0x100, 0x98deb4), (0x200, 0x98dec0), (0x400, 0x98decc)] {
            if v & bit != 0 {
                s.slot_a = t;
            }
        }
        let table = s.slot_a;
        exe::crt_qsort(table, 9, 1, 0x487720usize);
        let mut k = 0usize;
        while read_u8(table + k) != 0xff {
            let attr = read_u8(table + k);
            exe::database_get_player_attribute_name(attr as u32, 0xde1f64, 0x7d0, 1);
            heading(area, k as u32 + 3, 0xc, 8, attr as i8 as i32 as u32);
            k += 1;
        }
        value_heading(area);
    } else if v & 0x20 != 0 {
        weights = w.other.as_ptr();
        let area = head(weights);
        first_three(area);
        for (col, label, data) in [(3u32, 0x98e9e0usize, 0x1000u32), (4, 0x98e9dc, 0x4000), (5, 0x98e9d4, 0x8000), (6, 0x98e9cc, 0x10000),
                                   (7, 0x98e9f8, 0x80), (8, 0x98e9f0, 0x100), (9, 0x98e9e8, 0x200)] {
            lang(0xde1f64, label);
            heading(area, col, 0xc, 6, data);
        }
        value_heading(area);
    } else {
        // no view of its own: the rows go to the last area made (ebp in the original)
        return s.last_area;
    }
    tel26_add_area(d(), [0x6e, (row_h + top + 8) as u32, 0x30c, (bottom - 8) as u32, 0xd, weights as u32, rows.wrapping_sub(1) as u32, 0, 1, 0, u32::MAX])
}

/// The rows, one per player of the list (for a match, only the ones selected inside the eleven plus the substitutes
/// with no provisional number; only players with a player record), in the area `area`.
///
/// Each player's training caught up; column 0: squad numbers mode, his number (none: an empty item 6, link 9 to him; held
/// for good: 0x46, link 5 with the number; provisional: an item 1 in the greyed colour); European mode, his registration
/// "In" (1: a button, link 0xa; 2: a button when rule byte 0xb is 3 and fewer than 2 reserves, else a text), "Out" (4: a
/// button while registered - reserves + 0xd is below the squad size asked, else a text), "Ine" (8 or 0x10, a text), else
/// an empty button under the same condition or an empty text. Column 1 the status (`squad_list_status`), column 2 the name
/// (white for a player of a club the human runs: link 0x7dc on a national team's screen, else 0x7d7; a person made in the
/// game in the greyed colour; on loan or with a contract of type 5 at the club, 0xae3152; else 0xae3184). Then the
/// Traditional view's sort cell (two per row) or the view's cells and the value in column 0xc (a button, link 0xd, for a
/// database player whose club the human runs).
pub(crate) unsafe fn squad_numbers_build_rows(s: &mut SquadNumbersScreen, area: u32) {
    let list = s.list.as_mut_ptr() as usize;
    let club = s.club;
    let traditional = s.view & 1 != 0;
    let rules = SquadListRules { club, sort: s.sort, view_flags: s.view, category: s.category, table: s.slot_a, squad_screen: true };
    let (mut col, mut row) = (0u32, 0u32);
    let mut buf = [0u8; 0xc8];
    let b = buf.as_mut_ptr() as usize;
    let (cbe04, c3184, c317e, cbdec, cbdcc) = (part_exchange_colour(0xaebe04), part_exchange_colour(0xae3184), part_exchange_colour(0xae317e),
        part_exchange_colour(0xaebdec), part_exchange_colour(0xaebdcc));
    let lang = part_exchange_lang();
    part_exchange_rewind(list);
    while exe::llist_get_current(list) != 0 {
        let p = Staff(read_usize(exe::llist_get_next(list)));
        if s.mode == 1 && s.fixture != 0 {
            let slot = exe::tactics_find_selection_slot(read_usize(G_TACTICS), club, p, s.side) as i8;
            if slot < 0 || slot as i32 >= (read_u8(s.fixture + 0x4d) & 0xf) as i32 + 0xb || squad_numbers_number(p.addr(), club) < 0 {
                continue;
            }
        }
        if p.player().addr() == 0 {
            continue;
        }
        exe::training_manager_update_player_training(read_usize(G_TRAINING_MANAGER), p, 0);
        let item = |f: [u32; 18]| {
            tel26_add_item(screens_current_display(G_SCREEN_MANAGER), f);
        };
        if s.mode == 1 {
            let n = squad_numbers_number(p.addr(), club);
            if n == 0 {
                item([6, 0, 0, 0, 0, col, row, 0x30, cbe04, cbe04, 0xc, 1, c3184, 0xad9c64, 0, 9, p.addr() as u32, area]);
            } else {
                part_exchange_sprintf()(0xde1f64, 0x98806c, (n as i32).wrapping_abs()); // "%d"
                if n > 0 {
                    item([0x46, 0, 0, 0, 0, col, row, 0x30, cbe04, cbe04, 0xc, 1, c3184, 0xde1f64, 0, 5, n as i32 as u32, area]);
                } else {
                    item([1, 0, 0, 0, 0, col, row, 0x30, cbe04, cbe04, 0xc, 1, c317e, 0xde1f64, 0, 0, 0, area]);
                }
            }
        } else {
            let open = (s.registered as i8 as i32 - s.reserves as i8 as i32 + s.rule_d as i8 as i32) < s.squad_size as i8 as i32;
            let button = |colour: u32, text: u32| {
                tel26_area_button([area, col, row, 0x30, cbe04, 0xc, 1, colour, text, 0xa, p.addr() as u32]);
            };
            let text = |a5: u32, colour: u32, text: u32| {
                tel26_area_text([area, col, row, 0x30, a5, 0xc, 1, colour, text]);
            };
            match p.squad_selected_for() {
                1 => {
                    lang(0xde1f64, 0x9910f8, 0xad9c64usize); // "In"
                    button(cbdec, 0xde1f64);
                }
                2 => {
                    lang(0xde1f64, 0x9910f8, 0xad9c64usize); // "In"
                    if s.rule_b == 3 && (s.reserves as i8) < 2 {
                        button(c3184, 0xde1f64);
                    } else {
                        text(cbe04, c3184, 0xde1f64);
                    }
                }
                4 => {
                    lang(0xde1f64, 0x9910c8, 0xad9c64usize); // "Out"
                    if open {
                        button(c317e, 0xde1f64);
                    } else {
                        text(cbe04, c317e, 0xde1f64);
                    }
                }
                8 | 0x10 => {
                    lang(0xde1f64, 0x98e994); // "Ine"
                    text(cbdcc, c3184, 0xde1f64);
                }
                _ => {
                    if open {
                        button(c3184, 0);
                    } else {
                        text(cbe04, c3184, 0);
                    }
                }
            }
        }
        squad_list_status(&rules, area, p.addr(), col, row);
        let new_person = !p.is_null() && read_i32(p.addr()) >= read_i32(G_STAFF_COUNT);
        let contract_club = Contract(if new_person { 0 } else { exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club) });
        let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
        exe::database_format_staff_name(p, 0xde1f64usize, 0x7d0, if traditional { 4 } else { 1 });
        let national = || exe::database_is_national_team(club) != 0;
        let controls = |c: usize| c != 0 && exe::human_manager_current_controls_club(c, 0, 1) != 0;
        let (colour, link) = if national() && controls(p.club_job().addr()) {
            (cbdec, 0x7dc)
        } else if !national() && controls(p.national_job().addr()) {
            (cbdec, 0x7d7)
        } else if national() && new_person {
            (c317e, 0x7dc)
        } else if !national() && new_person {
            (c317e, 0x7d7)
        } else if national() {
            (c3184, 0x7dc)
        } else if (!contract_club.is_null() && contract_club.contract_type() & 0x3f == 5)
            || exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) != 0 {
            (part_exchange_colour(0xae3152), 0x7d7)
        } else {
            (c3184, 0x7d7)
        };
        tel26_area_button([area, col + 2, row, 1, 0, 1, if traditional { 3 } else { 2 }, colour, 0xde1f64, link, p.addr() as u32]);
        let injury = exe::injury_get_player_injury_status(read_usize(G_INJURY), p);
        if traditional {
            squad_list_sort_cell(&rules, area, p.addr(), col, row, injury, contract_club.addr(), contract, b);
            col += 4;
            if col > 4 {
                row += 1;
                col = 0;
            }
        } else {
            squad_list_view_cells(&rules, area, p.addr(), col, row, injury, contract_club.addr(), contract, b);
            part_exchange_value(p.addr());
            let (c3192, club_job) = (part_exchange_colour(0xae3192), p.club_job().addr());
            if !new_person && club_job != 0 && exe::human_manager_current_controls_club(club_job, 0, 1) != 0 {
                tel26_area_button([area, col + 0xc, row, 0x30, c3192, 0xc, 1, c3184, 0xde1f64, 0xd, p.addr() as u32]);
            } else {
                tel26_area_text([area, col + 0xc, row, 0x30, c3192, 0xc, 1, c3184, 0xde1f64]);
            }
            row += 1;
        }
    }
}

/// The comparator `build_filtered_sorted_squad_list` sorts by for the sort flags (the first flag set, in the original's
/// order); 0 for none. Flags 0x10 and 0x2000 sort national and all-star teams (`exe::nationalteams_is_national_or_all_star_team`)
/// with 0x485960, other clubs with 0x485a60 and 0x486760.
pub(crate) unsafe fn squad_list_comparator(sort: u32, club: usize, variant: u32) -> usize {
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

/// Whether squad player `p` passes the list's filters (`filter`, see `build_filtered_sorted_squad_list`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn squad_list_passes(club: usize, p: usize, reserves: u32, filter: u32, team_sheet: usize, fixture: usize, linked: usize) -> bool {
    let p = Staff(p);
    let linked = Club(linked);
    let club = Club(club);
    let sheet = p.player().addr();
    if exe::database_is_national_team(club) != 0 && exe::database_is_staff_foreign(p, club.nation().addr(), 1) != 0 {
        covered!("tel06_1 foreigner left a national squad");
        exe::database_remove_staff_from_club(club, p, 0usize, 1, 0);
    }
    if exe::database_is_national_team(club) == 0 && exe::human_manager_current_controls_club(club, 0, 1) == 0
        && read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) && p.playing_squad() & 2 != 0 {
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
        let in_sheet = squad_list_in_team_sheet(club.addr(), p.addr(), reserves, team_sheet);
        if exe::database_is_national_team(club) != 0 {
            let flags = p.playing_squad();
            let needs = if reserves == 0 { flags & 8 != 0 } else { flags & 4 != 0 };
            if needs && !in_sheet {
                return false;
            }
        }
        if filter & 0x200 != 0 && exe::database_is_national_team(club) == 0 && exe::human_manager_current_controls_club(club, 0, 1) != 0 {
            let flags = get_squad_flags_for_club(read_usize(G_CONTRACT_MANAGER), p.addr(), club.addr());
            let needs = if reserves == 0 { flags == 2 } else { flags == 1 };
            if needs && !in_sheet {
                return false;
            }
        }
    }
    if filter & 0x800 == 0 {
        let available = is_player_available_for_match(p.addr(), fixture, club.addr(), G_CURRENT_DATE) != 0;
        if (filter & 0x1000 != 0 && !available) || (filter & 0x2000 != 0 && available) {
            return false;
        }
    }
    if filter & 0x4000 == 0 && filter & 0x8000 != 0 && p.squad_selected_for() != 2 && (p.age() as i8) > 0x13
        && ((club.continental_comp_id() as i32) != -1 || (!linked.is_null() && (linked.continental_comp_id() as i32) != -1)) {
        return false;
    }
    if filter & 0x10000 == 0 && filter & 0x20000 != 0 && read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) {
        return false;
    }
    if filter & 0x40000 == 0 && filter & 0x80000 != 0 && p.squad_selected_for() == 8 {
        return false;
    }
    if filter & 0x100000 == 0 && filter & 0x200000 != 0 {
        let job = p.club_job().addr();
        if job != club.addr() && job != exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1) {
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

/// A squad list routine side by side: the sort context, the return and the list built (its items). The virtual
/// players topped up and a foreigner removed from a national squad by the first run are not undone (the second run
/// finds them done).
pub(crate) unsafe fn compare_squad_list(name: &str, list: usize, original: impl FnOnce() -> u32, rust: impl FnOnce() -> u32) -> u32 {
    let reg = [(SQUAD_LIST_SORT_CLUB, 8usize), (SQUAD_LIST_SORT_CONTEXT, 0x18), (SQUAD_LIST_SORT_MANAGER, 4)];
    // the sort's comparators read the players' status, which can roll the day's fitness or the week's training in the
    // original's run (`training_rolled_by`): the copy then sorts other values; not compared
    let mut rolled = false;
    let (a, b, same, _, _) = crate::comparison::side_by_side(&reg, || {
        let mut r = (0, Vec::new());
        rolled = training_rolled_by(|| r = (original(), squad_list_people(list)));
        r
    }, || (rust(), squad_list_people(list)));
    SCOREBOARD.record((same && a == b) || rolled, || format!("{} (club squad list): returned {} with {:x?}, Rust {} with {:x?}", name, a.0, a.1, b.0, b.1));
    a.0
}

/// `build_filtered_sorted_squad_list` (0x4841d0) by address, as the original calls it.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn squad_list_call(club: usize, reserves: u32, sort_flags: u32, name_style: u32, opt4: u32, opt5: u32, opt6: u32, filter_flags: u32,
    list: usize, team_sheet: usize, fixture: usize, top_n: i32, players: usize, sort_variant: u32) -> u32 {
    exe::build_filtered_sorted_squad_list(club, reserves, sort_flags, name_style, opt4, opt5, opt6, filter_flags, list, team_sheet, fixture, top_n, players, sort_variant)
}

/// The body of `get_player_status_code` once the interested clubs are in `interested` (see the routine).
pub(crate) unsafe fn status_code(p: usize, club: usize, fixture: usize, interested: usize) -> u8 {
    let fixture = Fixture(fixture);
    let p = Staff(p);
    let club = Club(club);
    let injury = exe::injury_get_player_injury_status(read_usize(G_INJURY), p);
    let condition = exe::injury_get_condition(read_usize(G_INJURY), p, 1);
    let mut restrictions = [0u8; 0x20];
    let tm = read_usize(G_TRANSFER_MANAGER);
    if !fixture.is_null() {
        exe::transfer_manager_fill_squad_restrictions(tm, fixture, restrictions.as_mut_ptr(), 1);
    }
    exe::shortlist_manager_collect_interested_clubs(read_usize(tm + 0x856), p, interested, 1, 2, 0);
    let cm = read_usize(G_CONTRACT_MANAGER);
    let real = read_i32(p.addr()) < read_i32(G_STAFF_COUNT);
    let mut contract = 0usize;
    if real {
        contract = exe::contract_manager_get_contract_for_club(cm, p, club);
        if !club.is_null() && contract != 0 && read_u32(club.addr()) != read_u32(contract + contract::CLUB) && exe::contract_manager_get_loan_contract(cm, p) != 0
            && read_u32(exe::contract_manager_get_loan_contract(cm, p) + contract::CLUB) == read_u32(club.addr()) {
            contract = exe::contract_manager_get_loan_contract(cm, p);
        }
    }
    let listed = || read_u16(interested) != 0;
    let regen = || exe::player_regen_get_retirement_status(read_usize(G_PLAYER_REGEN), p) != 0;
    if club.is_null() {
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
            if let Some(code) = status_contract_code(p.addr(), contract, false) {
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
    let nation = club.nation().addr();
    if !national() && exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) != 0 {
        return 0x15;
    }
    if read_usize(G_INJURY) != 0 && status_on_holiday(read_usize(G_INJURY), p.addr()) {
        return 0x15;
    }
    if exe::transfer_manager_player_needs_work_permit(tm, p, club, 0) != 0 {
        return 1;
    }
    if !fixture.is_null() {
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
    if !fixture.is_null() && exe::discipline_is_player_banned(read_usize(G_DISCIPLINE), p, fixture.competition().addr(), 1, 0) != 0 {
        return 5;
    }
    if injury == 2 {
        return 6;
    }
    if !fixture.is_null() && !national() && real && restrictions[0..3].iter().any(|&b| b != 0xff) && exe::database_is_staff_foreign(p, nation, 1) != 0 {
        return 7;
    }
    if injury == 1 {
        return 8;
    }
    if !fixture.is_null() && !national() && real && restrictions[3..6].iter().any(|&b| b != 0xff) && exe::database_is_staff_foreign(p, nation, 1) != 0
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
    if !fixture.is_null() {
        if exe::discipline_is_one_booking_from_ban(read_usize(G_DISCIPLINE), p, fixture.competition().addr()) != 0 {
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
    if real && regen() && (p.age() as i8) > 0x1e {
        return 0x11;
    }
    if contract != 0 {
        if let Some(code) = status_contract_code(p.addr(), contract, true) {
            return code;
        }
    }
    0x19
}

/// The fixture's link data for the round label: the competition id, the round prefix and the round name packed 11
/// bits each (prefix and name masked to 11 bits).
pub(crate) unsafe fn fixture_row_round_link(fixture: usize) -> u32 {
    let fixture = Fixture(fixture);
    let id = fixture.comp_id() as u32;
    (((id << 11) | ((fixture.round_prefix() as u16) as u32 & 0x7ff)) << 11) | ((fixture.round_name() as u16) as u32 & 0x7ff)
}

/// Whether the fixture's competition is a loaded one ([G_COMP_OBJECT_TABLE] by the fixture's competition id) that is
/// not a friendly of its own (`exe::friendly_is_own_comp_record`).
pub(crate) unsafe fn fixture_row_comp_loaded(fixture: usize, comp: usize) -> bool {
    let fixture = Fixture(fixture);
    read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * (fixture.comp_id() as u32) as usize) != 0
        && exe::friendly_is_own_comp_record(read_usize(G_FRIENDLY), comp) == 0
}

/// Whether the competition is current on the fixture's date (`Friendly::is_comp_current_on_date`, 0x5c29b0).
pub(crate) unsafe fn fixture_row_comp_current(fixture: usize, comp: usize) -> bool {
    let fixture = Fixture(fixture);
    let mut date = [0u8; 8];
    exe::new_date(date.as_mut_ptr() as usize, fixture.day() as u32, fixture.year() as u32);
    exe::friendly_is_comp_current_on_date(read_usize(G_FRIENDLY), comp, date.as_ptr() as usize) != 0
}

/// The result cell (column 5) once the result text is in the buffer: a link to the match details (0x7d3, the match
/// stats slot) when the match is still in the match stats (`exe::match_stats_fixture_entry_matches`), returning 1;
/// otherwise a plain text, returning `plain_returns`.
pub(crate) unsafe fn fixture_row_result(fixture: usize, area: u32, row: u32, plain_returns: u32) -> u32 {
    let fixture = Fixture(fixture);
    let found = exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), fixture.match_stats_slot() as i32,
        fixture.home_club_id() as i32, fixture.away_club_id() as i32, fixture.day() as i32,
        fixture.year() as i32) != 0;
    let style = [area, 5, row, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32];
    if found {
        tel26_area_button([style[0], style[1], style[2], style[3], style[4], style[5], style[6], style[7], style[8], 0x7d3, (fixture.match_stats_slot() as u32)]);
        1
    } else {
        tel26_area_text(style);
        plain_returns
    }
}

/// One entry of the club squad navigation menu (`Display::add_item`, kind 2): the text, the handler and the data, in
/// the popup list colours.
pub(crate) unsafe fn squad_menu_entry(area: u32, text: usize, handler: u32, data: usize) {
    tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, job_colour(JOB_COLOUR_31AA), job_colour(JOB_COLOUR_BDEC), 1, 1,
        job_colour(JOB_COLOUR_BDEA), text as u32, 0, handler, data as u32, area]);
}

/// The menu's popup area, made on the first entry: the menu button (`ScreenManager::add_button_item` at 0x69, 0xf,
/// 0x78, 0x23 in the screen's colours) and the popup area it opens (`Display::add_area`, flags 0x30, popup colour);
/// later sections get a separator line instead (kind 1, flags 0x1000010). Returns true when a separator was added.
pub(crate) unsafe fn squad_menu_section(area: &mut u32, colour: u32, colour_text: u32) -> bool {
    if *area as u16 == 0xffff {
        let button = tel26_add_button([0x69, 0xf, 0x78, 0x23, 0x30, colour, 0xc, 1, colour_text, 0, 0, 0]);
        *area = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x30, job_colour(JOB_COLOUR_31AA), button]);
        false
    } else {
        tel26_add_item(current_human_display(), [1, 0, 0, 0, 0, 0, 0, 0x1000010, job_colour(JOB_COLOUR_31AA), job_colour(JOB_COLOUR_BDEC), 1, 1,
            job_colour(JOB_COLOUR_BDEA), 0, 0, 0, 0, *area]);
        true
    }
}

/// The clubs the navigation menu lists as related (`build_club_squad_navigation_menu`), in table order.
pub(crate) unsafe fn squad_menu_related_clubs(club: usize) -> Vec<usize> {
    let club = Club(club);
    let mut v = Vec::new();
    let nation = club.nation();
    let table = read_usize(G_CLUB_TABLE);
    let at = |k: i32| table + k as usize * club::SIZE;
    let same_continent = |c: usize| read_usize(c + club::NATION) != 0 && read_u32(read_usize(c + club::NATION) + nation::CONTINENT) == (nation.continent() as u32);
    if exe::club_is_national_team(club) != 0 && !nation.is_null() && (nation.continent() as u32) != 0 {
        // the national teams of the continent (the last G_NATION_COUNT entries of the club table)
        let mut k = read_i32(G_CLUB_COUNT) - read_i32(G_NATION_COUNT);
        while k < read_i32(G_CLUB_COUNT) {
            if same_continent(at(k)) {
                v.push(at(k));
            }
            k += 1;
        }
    } else if exe::database_is_national_team(club) != 0 && !nation.is_null() && (nation.continent() as u32) != 0 {
        // the second sides of the continent's national teams (the G_NATION_COUNT entries before them)
        let m = read_i32(G_NATION_COUNT);
        let mut k = read_i32(G_CLUB_COUNT) - 2 * m;
        while k < read_i32(G_CLUB_COUNT) - read_i32(G_NATION_COUNT) {
            if same_continent(at(k)) {
                v.push(at(k));
            }
            k += 1;
        }
    } else if club.division().addr() != 0 && !nation.is_null() {
        // the clubs of the same nation and division (the club itself included)
        let mut k = 0;
        while k < read_i32(G_CLUB_COUNT) - read_i32(G_NATION_COUNT) {
            let c = at(k);
            if read_usize(c + club::NATION) == nation.addr() && read_usize(c + club::DIVISION) == club.division().addr() {
                v.push(c);
            }
            k += 1;
        }
    }
    v
}

/// Whether `club` is free on `date` (`exe::friendly_is_club_free_on_date`) and its date status is 1.
pub(crate) unsafe fn club_action_free(club: usize, date: usize, kind: u32, check_stadium: u32) -> bool {
    exe::friendly_is_club_free_on_date(read_usize(G_FRIENDLY), date as *mut [u8; 8], club, kind) != 0 && club_action_date_status(club, date, kind, check_stadium) == 1
}

/// The club's kit colours (`exe::database_get_club_kit_colours`, kit 1): the foreground and background words in
/// dwords.
pub(crate) unsafe fn club_action_kit(club: usize) -> (u32, u32) {
    let (mut fg, mut bg) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut fg, &mut bg);
    (fg, bg)
}

/// `confirm_push` (0x591840, cdecl): the box with its title, heading, text, buttons (1 = yes/no), the two colours and
/// the dialog code its answer comes back with.
pub(crate) unsafe fn club_action_confirm(title: usize, heading: usize, text: usize, buttons: u32, text_colour: u32, colour: u32, code: u32) {
    exe::confirm_push(title, heading, text, buttons, text_colour, colour, code);
}

/// The answer of a club screen dialog (see `handle_club_screen_action`); None when it falls through to the buttons.
pub(crate) unsafe fn club_action_dialog_answer() -> Option<i32> {
    let code = read_u32(G_PENDING_COMMAND_CODE);
    let yes = read_u32(G_PENDING_COMMAND_ARG) == 2;
    let f = read_usize(G_FRIENDLY);
    let tournament_answer = |add: u32| {
        let t = club_action_friendly_find(0x5c6830);
        exe::friendly_set_tournament_invite(f, get_screen_param(0), add);
        write_u32(t + 0x29, 0);
        if read_u32(t + 0x2d) != 0 {
            write_u32(G_PENDING_COMMAND_CODE, 0x10);
            write_usize(G_PENDING_COMMAND_ARG, t);
            write_u32(t + 0x2d, 0);
        }
        -10
    };
    match code {
        0x41 => {
            write_u32(G_PENDING_COMMAND_CODE, 0);
            if yes {
                covered!("tel06_2 scout assigned");
                write_u32(G_PENDING_COMMAND_ARG, 0);
                exe::scout_manager_scope_current_search_to_club(read_usize(G_SCOUT_MANAGER), get_screen_param(0));
                return Some(-13);
            }
        }
        0xc => {
            write_u32(G_PENDING_COMMAND_CODE, 0);
            if yes {
                covered!("tel06_2 friendly invitation sent");
                write_u32(G_PENDING_COMMAND_ARG, 0);
                let m = club_action_friendly_find(0x5c68b0);
                let club = get_screen_param(0);
                if read_u32(m + 0x10) == read_u32(m + 0xc) {
                    write_usize(m + 0x14, club);
                } else {
                    write_usize(m + 0x10, club);
                }
                exe::friendly_send_friendly_request(f, m);
                write_u32(m + 8, 0);
                return Some(-10);
            }
            write_u32(G_PENDING_COMMAND_ARG, 0);
        }
        0xd => {
            write_u32(G_PENDING_COMMAND_CODE, 0);
            if yes {
                covered!("tel06_2 friendly dates request answered");
                write_u32(G_PENDING_COMMAND_ARG, 0);
                let r = club_action_friendly_find(0x5c6920);
                exe::friendly_schedule_assign_opponent(f, r, get_screen_param(0));
                let mut k = 0i8;
                while read_i8(r + 8) > 0 && k < read_i8(r + 8) {
                    let e = (r as isize + 25 * k as isize) as usize;
                    if read_u32(e + 0x19) != 0 && read_u8(e + 0x21) == 1 {
                        write_u8(e + 0x21, 2);
                        let m = exe::friendly_add_friendly_match(f, read_usize(e + 0x1d), e + 9, read_usize(e + 0x19), 0);
                        if m != 0 {
                            exe::friendly_send_friendly_request(f, m);
                        }
                    }
                    k = k.wrapping_add(1);
                }
                write_u32(r + 0x103, 0);
                return Some(-10);
            }
            write_u32(G_PENDING_COMMAND_ARG, 0);
        }
        0xe => {
            write_u32(G_PENDING_COMMAND_CODE, 0);
            if yes {
                write_u32(G_PENDING_COMMAND_ARG, 0);
                return Some(tournament_answer(0));
            }
            write_u32(G_PENDING_COMMAND_ARG, 0);
        }
        0xf => {
            write_u32(G_PENDING_COMMAND_CODE, 0);
            write_u32(G_PENDING_COMMAND_ARG, 0);
            if yes {
                return Some(tournament_answer(1));
            }
        }
        _ => {}
    }
    None
}

/// The friendly dialogs (buttons 0x66 and 0x67): "Invite For Friendly"/"Offer Friendly" when the club is free on the
/// date, otherwise the "unavailable" box (see `handle_club_screen_action`).
pub(crate) unsafe fn club_action_friendly_box(club: usize, date: usize, kit_club: usize, invite: bool, code: u32, swapped: bool) {
    let club = Club(club);
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (mut title, mut heading, mut when) = ([0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8]);
    let (t, h, d) = (title.as_mut_ptr() as usize, heading.as_mut_ptr() as usize, when.as_mut_ptr() as usize);
    let gender = (club.short_gender_name() as i8) as i32;
    exe::date_get_long_date_string(date, d, 0xc8, 0xff);
    let (fg, bg) = club_action_kit(kit_club);
    let (text_colour, colour) = if swapped { (bg, fg) } else { (fg, bg) };
    if code != 0 {
        if swapped {
            // button 0x66: the title first, the text, the heading
            lang(t, if invite { 0x99180cusize } else { 0x9917bc });
            lang(JOB_TEXT, if invite { 0x9917ccusize } else { 0x991778 }, gender, gender, 0x64, club.addr() + club::SHORT_NAME, d);
            let hd = lang(h, 0x9909b8usize); // "Please Confirm"
            club_action_confirm(t, hd, JOB_TEXT, 1, text_colour, colour, code);
        } else {
            let text = lang(JOB_TEXT, 0x9917ccusize, gender, gender, 0x64, club.addr() + club::SHORT_NAME, d);
            let hd = lang(h, 0x9909b8usize);
            let tt = lang(t, 0x99180cusize); // "Invite For Friendly"
            club_action_confirm(tt, hd, text, 1, text_colour, colour, code);
        }
    } else {
        // "{}<%s - club>{} are unavailable for a friendly match on {}<%s - date>."
        let text = lang(JOB_TEXT, 0x991730usize, gender, gender, 0x64, club.addr() + club::SHORT_NAME, d);
        let hd = lang(h, 0x9916f8usize, 0xad9c64usize);
        let tt = lang(t, 0x99180cusize);
        club_action_confirm(tt, hd, text, 0, text_colour, colour, 0);
    }
}

/// The tournament dialogs (buttons 0x68 and 0x69; see `handle_club_screen_action`).
pub(crate) unsafe fn club_action_tournament_box(club: usize, t: usize, format: usize, heading_format: usize, title_format: usize, buttons: u32, code: u32) {
    let club = Club(club);
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (mut title, mut heading, mut when) = ([0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8]);
    let (tb, h, d) = (title.as_mut_ptr() as usize, heading.as_mut_ptr() as usize, when.as_mut_ptr() as usize);
    let gender = (club.short_gender_name() as i8) as i32;
    exe::date_get_long_date_string(t + 4, d, 0xc8, 0xff);
    let (fg, bg) = club_action_kit(read_usize(t));
    let text = lang(JOB_TEXT, format, gender, gender, 0x64, club.addr() + club::SHORT_NAME, t + 0x31, d);
    let hd = if heading_format == 0x9916f8 { lang(h, heading_format, 0xad9c64usize) } else { lang(h, heading_format) };
    let tt = lang(tb, title_format);
    club_action_confirm(tt, hd, text, buttons, fg, bg, code);
}

/// What `handle_club_screen_action` answers, worked out without running it.
pub(crate) unsafe fn club_action_planned_answer(widget: u32) -> i32 {
    if read_u32(CLUB_TABS_DIALOG_ANSWERED) != 0 && read_u32(G_PENDING_COMMAND_ARG) == 2 {
        match read_u32(G_PENDING_COMMAND_CODE) {
            0x41 => return -13,
            0xc..=0xf => return -10,
            _ => {}
        }
    }
    match club_tabs_action(widget as i16) {
        0x64 | 0x6a | 0x66..=0x69 => -4,
        0x65 => {
            let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
            if read_u32((G_SCREEN_RECORDS_MANAGER as isize + slot * 0x300) as usize) == 0 { 0 } else { -4 }
        }
        _ => 0,
    }
}

/// The new value of a part exchange view parameter for the picked bit `v`: the bit replaces the others of its group
/// (`groups`: the masks that clear a group, with its members); None for a bit of no group.
pub(crate) fn part_exchange_pick(param: u32, v: u32, groups: &[(u32, &[u32])]) -> Option<u32> {
    groups.iter().find(|(_, members)| members.contains(&v)).map(|&(mask, _)| (param & mask) | v)
}

/// The sort parameter 5 after picking sort column `bit` with value `v` against `current` (parameter 7 or 8; for bit
/// 0 the column is `v` itself): picking the column already sorted by flips the reverse flag 0x1000000, a new column
/// clears it; the bits other than 0xc1000000 and the column's are cleared.
pub(crate) fn part_exchange_sort(p5: u32, bit: u32, same: bool) -> u32 {
    let reverse = if p5 & bit != 0 && same && p5 & 0x1000000 == 0 { p5 | 0x1000000 } else { p5 & !0x1000000 };
    reverse & (0xc1000000 | bit)
}

/// Whether `part_exchange_screen_handle_event` only writes screen parameters for this widget (and can run twice).
pub(crate) unsafe fn part_exchange_input_params_only(widget: u32) -> bool {
    let w = widget as i16;
    let v = club_tabs_value(w) as u32;
    match club_tabs_action(w) {
        4 => get_screen_param(4) as u32 & v == 0,
        9 => get_screen_param(9) as u32 & v == 0,
        5 => v as u8 != get_screen_param(6) as u8,
        1 | 3 | 6 | 7 | 8 => true,
        _ => false,
    }
}

/// The action button of the club screen heading (`ScreenManager::add_button_item` at 0x294, 4, 0x311, 0x18): the
/// text (`FN_LANGLIB_SPRINTF` of `format` into `JOB_TEXT`), the action id and its data.
pub(crate) unsafe fn club_action_button(colour: u32, colour_text: u32, format: usize, id: u32, data: usize) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    lang(JOB_TEXT, format);
    tel26_add_button([0x294, 4, 0x311, 0x18, 0x30, colour_text, 0xc, 1, colour, JOB_TEXT as u32, id, data as u32]);
}

/// The tie-break of every squad list comparator: a null first entry sorts last (1), a null second one first (-1);
/// otherwise the two names, written in the list's name style (`PLAYER_SORT_NAME_STYLE`) into 200-byte buffers by
/// `database_format_staff_name`, compared with `utils_string_compare(first, second, 1, 0)`.
pub(crate) unsafe fn player_sort_by_name(a: usize, b: usize) -> i32 {
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let style = read_u8(PLAYER_SORT_NAME_STYLE) as u32;
    let (mut x, mut y) = ([0u8; 200], [0u8; 200]);
    exe::database_format_staff_name(a, x.as_mut_ptr() as usize, 200, style);
    exe::database_format_staff_name(b, y.as_mut_ptr() as usize, 200, style);
    exe::utils_string_compare(x.as_ptr(), y.as_ptr(), 1, 0)
}

/// The two entries of a qsort call (pointers to staff pointers).
pub(crate) unsafe fn player_sort_pair(a: *const usize, b: *const usize) -> (usize, usize) {
    (*a, *b)
}

/// The rank of a European registration byte (`staff::SQUAD_SELECTED_FOR`): 1 → 4, 2 → 3, 4 → 2, 0 → 1, others 0.
pub(crate) fn euro_registration_rank(v: u8) -> u8 {
    match v {
        1 => 4,
        2 => 3,
        4 => 2,
        0 => 1,
        _ => 0,
    }
}

/// The original of one of this item's comparators side by side with the copy, by return value.
pub(crate) unsafe fn player_sort_compare(name: &str, original: unsafe extern "C" fn(*const usize, *const usize) -> i32,
    copy: unsafe extern "C" fn(*const usize, *const usize) -> i32, a: *const usize, b: *const usize) -> i32 {
    let r = original(a, b);
    let v = copy(a, b);
    SCOREBOARD.record(r == v, || format!("squad list sort {} ({:#x}, {:#x}): original={} rust={}", name, *a, *b, r, v));
    r
}

/// The "statistic" and "average rating" orders: the statistic of the first player is stored as a float, the
/// second's kept as returned; a greater second gives 1, a smaller -1, otherwise (equal or unordered) the names.
pub(crate) unsafe fn player_sort_by_float_stat(pa: *const usize, pb: *const usize, category: u32, field: u32) -> i32 {
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

/// The season statistic orders: `PlayerStats::get_stat(player record, 6, field, 6.5)` of the second player stored
/// as a float, minus the first's, truncated (`_ftol`) and narrowed to a signed word; zero goes to the names.
pub(crate) unsafe fn player_sort_by_season_stat(pa: *const usize, pb: *const usize, field: u32) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let stats = read_usize(G_PLAYER_STATS);
    let sb = exe::playerstats_get_stat(stats, read_usize(b + staff::PLAYER), 6, field, 6.5) as f32 as f64;
    let sa = exe::playerstats_get_stat(stats, read_usize(a + staff::PLAYER), 6, field, 6.5);
    let d = ftol(sb - sa) as i16;
    if d != 0 {
        return d as i32;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// A job byte for the staff orders: 12 counts as 5.
pub(crate) fn staff_sort_job(job: i8) -> i32 {
    if job == 0xc { 5 } else { job as i32 }
}

/// The three best players of a club's squad (its 50 pointers at +0xd7) by `CURRENT_REPUTATION` (`WORLD_REPUTATION`
/// for a national team) + 10 × `CURRENT_ABILITY`, in 16-bit arithmetic; ties keep the earlier; returns the count (at
/// most 3) and the three players.
pub(crate) unsafe fn club_squad_stars(club: usize) -> (i8, [usize; 3]) {
    let (mut count, mut best, mut score) = (0i8, [0usize; 3], [-10000i16; 3]);
    for k in 0..0x32 {
        let p = Staff(read_usize(club + 0xd7 + k * 4));
        if p.is_null() || p.player().addr() == 0 {
            continue;
        }
        let national = exe::database_is_national_team(club) != 0;
        let sheet = p.player();
        let reputation = read_i16(sheet.addr() + if national { player_sheet::WORLD_REPUTATION } else { player_sheet::CURRENT_REPUTATION });
        let s = reputation.wrapping_add(sheet.current_ability().wrapping_mul(10));
        if count < 3 {
            count += 1;
        }
        if s > score[0] {
            best = [p.addr(), best[0], best[1]];
            score = [s, score[0], score[1]];
        } else if s > score[1] {
            best = [best[0], p.addr(), best[1]];
            score = [score[0], s, score[1]];
        } else if s > score[2] {
            best[2] = p.addr();
            score[2] = s;
        }
    }
    (count, best)
}

/// The screen-list writes a forwarder makes: the network packet it appends (the position [0xae290c] and up to 0x200
/// bytes from it in the buffer [0xae2908], never past the limit [0xae2910]) and, in the current `Display`, the counts
/// of areas and items (+0x12e99e, +0x12e9a0), the next area slot and the next item slot, and the two overflow flags.
pub(crate) unsafe fn screens_forwarder_regions(sm: usize) -> Vec<(usize, usize)> {
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
pub(crate) unsafe fn scrman_item_regions(sm: usize, area: u32) -> Vec<(usize, usize)> {
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

/// Side by side over `regions` and the return value.
pub(crate) unsafe fn compare_screens_over(name: &str, regions: &[(usize, usize)], original: impl FnOnce() -> u32, rust: impl FnOnce() -> u32) -> u32 {
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(regions, original, rust);
    SCOREBOARD.record(equal && ro == rr, || format!("{}: return {:#x} x {:#x}, differences in {:?}", name, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// `Date::Date(this, day, year)` (0x549ef0, thiscall, ret 8): a date from the day of the year and the year.
pub(crate) unsafe fn club_screens_date(out: &mut [u8; 8], day: u32, year: u32) -> usize {
    exe::new_date(out.as_mut_ptr() as usize, day, year)
}

/// The club record of id `id` in the club table ([0xae23b4] + `club::SIZE` × id).
pub(crate) unsafe fn club_screens_club(id: i32) -> usize {
    read_usize(0xae23b4).wrapping_add((id as usize).wrapping_mul(club::SIZE))
}

/// The person record of id `id` in the person table ([0xae23bc] + `staff::SIZE` × id).
pub(crate) unsafe fn club_screens_staff(id: i32) -> usize {
    read_usize(0xae23bc).wrapping_add((id as usize).wrapping_mul(staff::SIZE))
}

/// `Cash::format_display_amount(this, out, size, flags)` (0x450aa0, thiscall, ret 0xc) on a local amount.
pub(crate) unsafe fn club_screens_format_cash(amount: i32, out: usize, size: u32, flags: u32) {
    exe::cash_format_display_amount(&amount, out, size, flags);
}

/// `Date::get_short_date_string(this, out, size, time_of_day)` (0x54a650, thiscall, ret 0xc).
pub(crate) unsafe fn club_screens_short_date(date: usize, out: usize, size: u32, time_of_day: u32) {
    (exe::date_get_short_date_string(date, out, size, time_of_day) as u32);
}

/// A transfer line's kind word when it has no fee: "Exch" (1), "Loan" (2, only `with_loan`), "Bos" (4), else "Free".
pub(crate) unsafe fn club_screens_free_kind(kind: u8, with_loan: bool) -> usize {
    match kind {
        1 => 0x98e620,
        2 if with_loan => 0x98e5fc,
        4 => 0x98e61c,
        _ => 0x98e614,
    }
}

/// A colour word of the interface palette, zero-extended (the original pushes it in a register whose high half is
/// whatever the register held: garbage that `Display` keeps out, since it stores the colours as words).
pub(crate) unsafe fn squad_colour(a: usize) -> u32 {
    read_u16(a) as u32
}

/// The game's text formatter (`FN_LANGLIB_SPRINTF`) with one string or a few arguments.
pub(crate) unsafe fn squad_text(out: usize, format: usize) -> i32 {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    f(out, format, 0xad9c64usize)
}

/// sprintf (`FN_CRT_SPRINTF`) of one integer into 0xde1f64.
pub(crate) unsafe fn squad_number_text(format: usize, v: i32) {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    f(0xde1f64, format, v);
}

/// sprintf (`FN_CRT_SPRINTF`) of one text into 0xde1f64.
pub(crate) unsafe fn squad_string_text(format: usize, s: usize) {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    f(0xde1f64, format, s);
}

/// `formation_get_position_abbreviation(position, 0xde1f64, 0x7d0, 0)` (0x5b7810, cdecl).
pub(crate) unsafe fn squad_position_text(position: u32) {
    exe::formation_get_position_abbreviation(position, 0xde1f64, 0x7d0, 0);
}

/// A person whose id is at or past the database's person count ([0xae2364]: one made in the game, a regen).
pub(crate) unsafe fn squad_is_new_person(player: usize) -> bool {
    player != 0 && read_i32(player) >= read_i32(0xae2364)
}

/// The player's value as money (`Cash::format_display_amount`, flags 5) when he has a club and a value (+0x52), else
/// "Free".
pub(crate) unsafe fn squad_value_text(player: usize) {
    let player = Staff(player);
    let club_job = player.club_job().addr();
    if club_job != 0 && (player.value() as i32) != 0 {
        club_screens_format_cash(player.value() as i32, 0xde1f64, 0x7d0, 5);
    } else {
        squad_text(0xde1f64, 0x98e614); // "Free"
    }
}

/// Column 0 of a row: the squad number, the selection slot or the last match's slot (see `club_screen_squad_tab_rows`).
pub(crate) unsafe fn club_screen_squad_tab_number(t: &mut SquadTab, area: u32, player: usize, col: i32, row: i32) {
    let club = t.club;
    let sb: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (cbe04, c3184) = (squad_colour(0xaebe04), squad_colour(0xae3184));
    if exe::human_manager_current_controls_club(club, 0, 1) != 0 {
        if t.view & 0x800 != 0 {
            let n = exe::human_manager_get_player_squad_number(read_usize(0xb63c98), player, 0) as i8;
            if n > -1 {
                squad_number_text(0x98806c, n as i32 + 1);
                t.add_item([0x46, 0, 0, 0, 0, col as u32, row as u32, 0x30, cbe04, cbe04, 0xc, 1, c3184, 0xde1f64, 0, 0x23, n as i32 as u32, area]);
            } else {
                t.add_item([6, 0, 0, 0, 0, col as u32, row as u32, 0x30, cbe04, cbe04, 0xc, 1, c3184, 0xad9c64, 0, 0x24, player as u32, area]);
            }
        } else {
            let slot = exe::tactics_find_selection_slot(read_usize(0xde1d08), club, player, t.reserves) as i8;
            if slot > -1 {
                if slot < 0xb {
                    let tactic = exe::human_manager_get_active_tactic(read_usize(0xb63c98), 0usize);
                    // the slot goes in a stack slot whose upper bytes are the filter's (as the original)
                    let pos = exe::formation_get_slot_position(tactic, (t.filter & 0xffff_ff00) | slot as u8 as u32, 0);
                    squad_position_text(pos as u32);
                } else {
                    sb(0xde1f64, 0x98ea6c, slot as i32 - 0xa);
                }
                t.add_item([0x46, 0, 0, 0, 0, col as u32, row as u32, 0x30, cbe04, cbe04, 0xc, 1, c3184, 0xde1f64, 0, 0xa, slot as i32 as u32, area]);
            } else {
                t.add_item([6, 0, 0, 0, 0, col as u32, row as u32, 0x30, cbe04, cbe04, 0xc, 1, c3184, 0xad9c64, 0, 0xb, player as u32, area]);
            }
        }
        return;
    }
    let mut text = 0xad9c64usize;
    if t.from_record {
        let side = if read_u32(club) != read_u32(0xad9db8) { 0x625 } else { 0 };
        let mut slot = -1i8;
        for k in 0..0x14u8 {
            if read_u32(0xad9f48 + side + k as usize * 0x49) == read_u32(player) {
                slot = k as i8;
            }
        }
        if slot > -1 {
            if slot < 0xb {
                squad_position_text(read_u16(0xad9f0c + side + slot as usize * 2) as u32);
            } else {
                sb(0xde1f64, 0x98ea6c, slot as i32 - 0xa);
            }
            text = 0xde1f64;
        }
    }
    t.text(area, col, row, 0x30, cbe04, 0xc, 1, c3184, text);
}

/// Column 1 of a row: the status item (see `club_screen_squad_tab_rows`).
pub(crate) unsafe fn club_screen_squad_tab_status(t: &mut SquadTab, area: u32, player: usize, col: i32, row: i32) {
    let club = t.club;
    let status = exe::get_player_status_code(player, club, t.next_match);
    // the item: flags, the colour twice (a9, a10), font 0xc, the text colour c13, link and data
    let item = |t: &SquadTab, flags: u32, a9: u32, a10: u32, c13: u32, link: u32, data: usize| {
        t.add_item([flags, 0, 0, 0, 0, (col + 1) as u32, row as u32, 0x30, a9, a10, 0xc, 1, c13, 0xde1f64, 0, link, data as u32, area]);
    };
    let say = |label: usize| {
        squad_text(0xde1f64, label);
    };
    let national = || exe::database_is_national_team(club) != 0;
    let (c3184, cbe04, cbdcc, c3190, c317c, cbdec) = (squad_colour(0xae3184), squad_colour(0xaebe04), squad_colour(0xaebdcc), squad_colour(0xae3190),
        squad_colour(0xae317c), squad_colour(0xaebdec));
    match status {
        1 | 2 | 3 => {
            say(match status { 1 => 0x98e9c8, 2 => 0x98e998, _ => 0x98e994 }); // "Wpm", "Cup", "Ine"
            let link = if national() { 0x7e0 } else { 0x7db };
            item(t, 0x202, cbe04, cbe04, c3184, link, player);
        }
        4 => {
            // international duty: no text of its own (0xde1f64 keeps what the row wrote last, as the original)
            let duty = exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), player, G_CURRENT_DATE, club, 0, 0) as usize;
            if duty != 0 {
                item(t, 0x202, cbe04, cbe04, c3184, 0x7d5, duty);
            } else {
                item(t, 0x201, cbe04, cbe04, c3184, 0, 0);
            }
        }
        5 => {
            say(0x98e990); // "Sus"
            let link = if national() { 0x7de } else { 0x7d9 };
            item(t, 0x202, cbdcc, cbdcc, c3184, link, player);
        }
        6 => {
            say(0x98e98c); // "Inj"
            let link = if national() { 0x7dd } else { 0x7d8 };
            item(t, 0x202, cbdcc, cbdcc, c3184, link, player);
        }
        0x15 => {
            say(0x98e988); // "Abs"
            item(t, 0x202, cbdcc, cbdcc, c3184, 0x7db, player);
        }
        7 | 9 => {
            say(0x98e984); // "Fgn"
            item(t, 0x202, cbe04, cbe04, c3184, 0x7db, player);
        }
        8 => {
            say(0x98e98c); // "Inj"
            let link = if national() { 0x7dd } else { 0x7d8 };
            item(t, 0x202, c3190, c3190, c3184, link, player);
        }
        0x16 => {
            say(0x98e97c); // "Tir "
            item(t, 0x202, c3190, c3190, c3184, 0x7d8, player);
        }
        0xa => {
            say(0x98e978); // "Yel"
            let link = if national() { 0x7de } else { 0x7d9 };
            let colour = squad_colour(0xaebdea);
            let first = exe::graphics_rgb_to_pixel(0xb4, 0xb4, 0, 0);
            let second = exe::graphics_rgb_to_pixel(0xb4, 0xb4, 0, 0);
            item(t, 0x202, second, first, colour, link, player);
        }
        0xb | 0xc | 0xd | 0xf | 0x12 | 0x18 | 0xe | 0x13 => {
            // Bid, Wnt, Req, Loa, Bos, Trn, Lst, Unh
            say(match status { 0xb => 0x98e974, 0xc => 0x98e96c, 0xd => 0x98e934, 0xf => 0x98e904, 0x12 => 0x98e8f0, 0x18 => 0x98e970, 0xe => 0x98e908,
                _ => 0x98e8e8 });
            item(t, 0x202, c317c, c317c, cbdec, 0x7db, player);
        }
        0x17 | 0x11 => {
            say(if status == 0x17 { 0x98e938 } else { 0x98e8f8 }); // "Sct", "Ret "
            let link = if national() { 0x7e0 } else { 0x7db };
            item(t, 0x202, c317c, c317c, cbdec, link, player);
        }
        0x10 => {
            say(0x98e900); // "Ctr"
            item(t, 0x202, c317c, c317c, cbdec, 0x7da, player);
        }
        0x14 => {
            say(0x98e8e0); // "Fut "
            let add: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute(FN_SCREENMANAGER_ADD_ITEM);
            add(t.sm, 0x202, 0, 0, 0, 0, (col + 1) as u32, row as u32, 0x30, c317c, c317c, 0xc, 1, cbdec, 0xde1f64, 0, 0x7db, player as u32, area);
        }
        _ => {}
    }
}

/// Column 3 of the positions view, by the sort (see `club_screen_squad_tab_rows`).
pub(crate) unsafe fn club_screen_squad_tab_sort_cell(t: &mut SquadTab, area: u32, player: usize, col: i32, row: i32, injury: u8, contract_club: usize, contract: usize,
                                          buf: &mut [u8; 0x7d0]) {
    let player = Staff(player);
    let club = t.club;
    let sort = t.sort;
    let pdata = player.player().addr();
    let cbdec = squad_colour(0xaebdec);
    let c = col + 3;
    let b = buf.as_mut_ptr() as usize;
    let cell = |t: &SquadTab, a6: u32, a7: u32| t.text(area, c, row, 1, 0, a6, a7, cbdec, 0xde1f64);
    if sort & 0x20 != 0 {
        let squad = read_usize(0xdd7ef0);
        if exe::squad_manager_club_uses_squad_numbers(squad, club, 0) != 0 && exe::squad_manager_get_squad_number(squad, player, club) != 0 {
            let n = exe::squad_manager_get_squad_number(read_usize(0xdd7ef0), player, club) as i32;
            squad_number_text(0x98806c, n.wrapping_abs());
        } else {
            squad_number_text(0x98ba0c, 0); // "-"
        }
        cell(t, 0xc, 2);
    } else if sort & 0x80 != 0 {
        if squad_is_new_person(player.addr()) {
            copy_text(0xde1f64 as *mut u8, 0x98ba0c as *const u8);
        } else {
            squad_recent_ratings(pdata);
        }
        cell(t, 0xc, 1);
    } else if sort & 0x100 != 0 {
        squad_morale(player.addr());
        cell(t, 0xc, 2);
    } else if sort & 0x200 != 0 {
        if injury == 2 {
            squad_text(0xde1f64, 0x98e98c); // "Inj"
        } else {
            squad_number_text(0x98e8d4, exe::injury_get_condition(read_usize(G_INJURY), player, 1) as i32);
        }
        cell(t, 0xc, 2);
    } else if sort & 0x2000 != 0 {
        if exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), club) == 0 {
            let nation = player.nation().addr();
            if nation != 0 {
                let team = exe::database_get_national_team(nation);
                t.button(area, c, row, 1, 0, 0xc, 2, cbdec, nation + 0x53, 0x7d5, team);
            }
        } else {
            let club_job = player.club_job().addr();
            if club_job != 0 {
                t.button(area, c, row, 1, 0, 1, 2, cbdec, club_job + club::SHORT_NAME, 0x7d5, club_job);
            }
        }
    } else if sort & 0x4000 != 0 {
        squad_number_text(0x98806c, read_i8(player.addr() + 0x18) as i32);
        cell(t, 0xc, 2);
    } else if sort & 0x8000 != 0 {
        squad_number_text(0x98806c, player.int_apps() as i32);
        cell(t, 0xc, 2);
    } else if sort & 0x10000 != 0 {
        squad_number_text(0x98806c, player.int_goals() as i32);
        cell(t, 0xc, 2);
    } else if sort & 0x20000 != 0 {
        if contract_club != 0 || contract != 0 {
            squad_wage_text(if contract_club != 0 { contract_club } else { contract }, buf);
            squad_string_text(0x987ff0, b); // "  %s"
        } else {
            write_u8(0xde1f64, 0);
        }
        cell(t, 1, 2);
    } else if sort & 0x40000 != 0 {
        if contract_club != 0 || contract != 0 {
            squad_end_date(if contract_club != 0 { contract_club } else { contract }, b);
            squad_string_text(0x987ff0, b);
        } else {
            write_u8(0xde1f64, 0);
        }
        cell(t, 1, 2);
    } else if sort & 0x20000000 != 0 {
        let yes = contract != 0 && exe::staff_contract_check_fifa_protected_period(contract, 0usize, 0usize, 0usize) != 0;
        squad_text(0xde1f64, if yes { 0x98e8d0 } else { 0x98e8cc }); // "Yes", "No"
        cell(t, 1, 2);
    } else if sort & 0x80000 != 0 {
        squad_value_text(player.addr());
        let club_job = player.club_job().addr();
        if !squad_is_new_person(player.addr()) && club_job != 0 && exe::human_manager_current_controls_club(club_job, 0, 1) != 0 {
            t.button(area, c, row, 1, 0, 0xc, 2, cbdec, 0xde1f64, 0xf, player.addr());
        } else {
            cell(t, 0xc, 2);
        }
    } else if sort & 0xf00000 != 0 {
        let field = if sort & 0x100000 != 0 { 2 } else if sort & 0x200000 != 0 { 3 } else if sort & 0x400000 != 0 { 5 } else { 0x11 };
        squad_stat(pdata, 6, field, 0xde1f64);
        cell(t, 0xc, 2);
    } else {
        squad_positions(pdata, 0xde1f64, 0x7d0);
        cell(t, 1, 2);
    }
}

/// The columns 3.. of the views other than positions (see `club_screen_squad_tab_rows`).
pub(crate) unsafe fn club_screen_squad_tab_view_cells(t: &mut SquadTab, area: u32, player: usize, col: i32, row: i32, injury: u8, contract_club: usize, contract: usize,
                                           buf: &mut [u8; 0x7d0]) {
    let player = Staff(player);
    let club = t.club;
    let view = t.view;
    let pdata = player.player().addr();
    let b = buf.as_mut_ptr() as usize;
    let (cbdec, cbde8, cbdd4, c31a8) = (squad_colour(0xaebdec), squad_colour(0xaebde8), squad_colour(0xaebdd4), squad_colour(0xae31a8));
    let cell = |t: &SquadTab, c: i32, a6: u32, a7: u32, colour: u32, text: usize| t.text(area, col + c, row, 1, 0, a6, a7, colour, text);
    let form = |t: &SquadTab, c: i32| {
        if squad_is_new_person(player.addr()) {
            copy_text(0xde1f64 as *mut u8, 0x98ba0c as *const u8);
        } else {
            squad_recent_ratings(pdata);
        }
        cell(t, c, 0xc, 1, cbdec, 0xde1f64);
    };
    let condition = |t: &SquadTab, c: i32| {
        if injury == 2 {
            squad_text(0xde1f64, 0x98e98c); // "Inj"
        } else {
            squad_number_text(0x98e8d4, exe::injury_get_condition(read_usize(G_INJURY), player, 1) as i32);
        }
        cell(t, c, 0xc, 2, cbdd4, 0xde1f64);
    };
    if view & 2 != 0 {
        if exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), club) == 0 {
            if exe::human_manager_current_controls_club(club, 0, 1) != 0 && contract_club != 0 {
                exe::staff_contract_get_squad_status_text(contract_club, b, 0x7d0);
                squad_string_text(0x987ff0, b);
                t.button(area, col + 3, row, 1, 0, 1, 2, cbde8, 0xde1f64, 0xf, player.addr());
            } else {
                cell(t, 3, 1, 2, cbde8, 0x98e8c8); // "  -"
            }
        } else {
            let club_job = player.club_job();
            if !club_job.is_null() {
                squad_string_text(0x987ff0, club_job.addr() + club::SHORT_NAME);
                t.button(area, col + 3, row, 1, 0, 1, 2, cbde8, 0xde1f64, 0x7d5, club_job.addr());
            }
        }
        if contract_club != 0 || contract != 0 {
            squad_wage_text(if contract_club != 0 { contract_club } else { contract }, buf);
            squad_string_text(0x987ff0, b);
        } else {
            squad_string_text(0x98e8c8, 0); // "  -"
        }
        cell(t, 4, 1, 2, cbdec, 0xde1f64);
        if contract_club != 0 || contract != 0 {
            squad_end_date(if contract_club != 0 { contract_club } else { contract }, b);
            squad_string_text(0x987ff0, b);
        } else {
            squad_string_text(0x98e8c8, 0);
        }
        cell(t, 5, 1, 2, cbdec, 0xde1f64);
        if exe::database_is_national_team(club) == 0 {
            if contract != 0 {
                let r = exe::staff_contract_get_release_clause_abbreviation(contract, b, 0x7d0);
                squad_string_text(0x987ff0, b);
                if r == 2 {
                    t.button(area, col + 6, row, 1, 0, 1, 2, c31a8, 0xde1f64, 0x7da, player.addr());
                } else if r == 1 {
                    t.button(area, col + 6, row, 1, 0, 1, 2, cbdd4, 0xde1f64, 0x7da, player.addr());
                } else {
                    cell(t, 6, 1, 2, cbdd4, 0xde1f64);
                }
            } else {
                cell(t, 6, 1, 2, cbdd4, 0x98e8c8);
            }
        }
    } else if view & 0x1000 != 0 {
        squad_positions(pdata, b, 0xc8);
        squad_string_text(0x987ff0, b);
        cell(t, 3, 1, 2, cbde8, 0xde1f64);
        form(t, 4);
        squad_morale(player.addr());
        cell(t, 5, 0xc, 2, cbdec, 0xde1f64);
        condition(t, 6);
        squad_stat(pdata, 6, 1, 0xde1f64);
        cell(t, 7, 0xc, 2, cbde8, 0xde1f64);
        squad_stat(pdata, 6, 0x11, 0xde1f64);
        cell(t, 8, 0xc, 2, cbde8, 0xde1f64);
    } else if view & 4 != 0 || view & 8 != 0 {
        let table = if view & 4 != 0 { 0x98de84 } else { 0x98de90 };
        let mut k = 0usize;
        while read_u8(table + k) != 0xff {
            let field = read_u8(table + k);
            // simplification: param 9 is a byte in a stack slot whose upper bytes are garbage in the original (a char)
            squad_stat(pdata, t.category as u32, field as u32, 0xde1f64);
            cell(t, k as i32 + 3, 0xc, 2, if field == 0x11 { cbdec } else { cbde8 }, 0xde1f64);
            k += 1;
        }
    } else if view & 0x10 != 0 {
        let table = t.attributes;
        let mut k = 0usize;
        while read_u8(table + k) != 0xff {
            let attr = read_u8(table + k);
            if t.human != 0 && exe::staff_is_player_attribute_visible(t.human, player, attr as u32) != 0 {
                squad_number_text(0x98806c, exe::staff_get_player_attribute(player, attr as u32) as i32);
                cell(t, k as i32 + 3, 0xc, 2, cbde8, 0xde1f64);
            } else {
                cell(t, k as i32 + 3, 0xc, 2, cbde8, 0x98ba0c); // "-"
            }
            k += 1;
        }
    } else if view & 0x20 != 0 {
        let nation = player.nation().addr();
        if nation != 0 {
            let team = exe::database_get_national_team(nation);
            t.button(area, col + 3, row, 1, 0, 0xc, 2, cbde8, nation + 0x53, 0x7d5, team);
        }
        squad_number_text(0x98806c, read_i8(player.addr() + 0x18) as i32);
        cell(t, 4, 0xc, 2, cbde8, 0xde1f64);
        squad_number_text(0x98806c, player.int_apps() as i32);
        cell(t, 5, 0xc, 2, cbde8, 0xde1f64);
        squad_number_text(0x98806c, player.int_goals() as i32);
        cell(t, 6, 0xc, 2, cbde8, 0xde1f64);
        form(t, 7);
        squad_morale(player.addr());
        cell(t, 8, 0xc, 2, cbdec, 0xde1f64);
        condition(t, 9);
    }
}

/// The end of the shared text (where the original appends with strlen).
pub(crate) unsafe fn stats_view_text_end() -> usize {
    let mut p = STATS_VIEW_TEXT;
    while read_u8(p) != 0 {
        p += 1;
    }
    p
}

/// strlen of the C string at `p`.
pub(crate) unsafe fn stats_view_c_len(p: usize) -> usize {
    let mut n = 0;
    while read_u8(p + n) != 0 {
        n += 1;
    }
    n
}

/// Length of the NUL-terminated text at `a`.
pub(crate) unsafe fn money_view_len(a: usize) -> usize {
    let mut n = 0;
    while read_u8(a + n) != 0 {
        n += 1;
    }
    n
}

/// `cash` formatted (`exe::cash_format_money` of `exe::cash_get_display_amount` with the same `flags`) into `buf`.
pub(crate) unsafe fn money_view_format(cash: &u32, buf: usize, size: u32, flags: u32) {
    exe::cash_format_money(buf, size, exe::cash_get_display_amount(cash as *const u32, flags), flags);
}

/// A finance block's income: the low dword of the players sold (+8, an i64) plus the eight dwords +0x30..+0x4c (gate
/// receipts, +0x34, prize money, season tickets, +0x40, other income, TV, merchandising), wrapping 32-bit.
pub(crate) unsafe fn money_view_income_total(fin: Finance, block: usize) -> u32 {
    let b = fin.addr() + block;
    (0..8).fold(read_u32(b + 8), |s, k| s.wrapping_add(read_u32(b + 0x30 + 4 * k)))
}

/// A finance block's expenditure: the low dword of the players bought (+0, an i64) plus the eight dwords +0x10..+0x2c
/// (other costs, +0x14, fines, ground, wages, dividends, bonuses, signing-on fees), wrapping 32-bit.
pub(crate) unsafe fn money_view_expenditure_total(fin: Finance, block: usize) -> u32 {
    let b = fin.addr() + block;
    (0..8).fold(read_u32(b), |s, k| s.wrapping_add(read_u32(b + 0x10 + 4 * k)))
}

/// A row title in column 0: the translated `fmt` (with `arg`) in pale grey, left aligned (1), font 3.
pub(crate) unsafe fn money_view_label(area: usize, row: usize, fmt: usize, arg: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let cell: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    text(MONEY_VIEW_TEXT, fmt, arg);
    cell(SCREEN_MANAGER, area, 0, row, 1, colour_word(G_GUI_COLOUR_BLACK), 1, 3, colour_word(G_GUI_COLOUR_PALE_GREY), MONEY_VIEW_TEXT);
}

/// A money cell of the table: the shared text in `colour`, font 2, aligned `align`.
pub(crate) unsafe fn money_view_cell(area: usize, col: usize, row: usize, align: usize, colour: usize) {
    let cell: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    cell(SCREEN_MANAGER, area, col, row, 1, colour_word(G_GUI_COLOUR_BLACK), align, 2, colour_word(colour), MONEY_VIEW_TEXT);
}

/// A total cell (Income, Expenditure): `cash` = `value`, formatted with flags 0x22 (0x21 when longer than 13
/// characters) in yellow; returns its display amount with flags 0x20 (read before the cell is added).
pub(crate) unsafe fn money_view_total_cell(cash: &mut u32, area: usize, col: usize, row: usize, align: usize, value: u32) -> i64 {
    *cash = value;
    money_view_format(cash, MONEY_VIEW_TEXT, MONEY_VIEW_TEXT_SIZE, 0x22);
    if money_view_len(MONEY_VIEW_TEXT) > 0xd {
        money_view_format(cash, MONEY_VIEW_TEXT, MONEY_VIEW_TEXT_SIZE, 0x21);
    }
    let shown = exe::cash_get_display_amount(cash as *const u32, 0x20);
    money_view_cell(area, col, row, align, G_GUI_COLOUR_YELLOW);
    shown
}

/// The text of a signed amount already in `cash` (its magnitude): positive with flags 0x22 (0x21 when longer than 13)
/// in the shared text, green; negative formatted into a local buffer with flags 0x22 and size 0xc8 (0x21 and size 0x7d0
/// when longer than 12), then "(%s)" into the shared text, red. Returns the colour.
pub(crate) unsafe fn money_view_signed_text(cash: &u32, negative: bool) -> usize {
    if !negative {
        money_view_format(cash, MONEY_VIEW_TEXT, MONEY_VIEW_TEXT_SIZE, 0x22);
        if money_view_len(MONEY_VIEW_TEXT) > 0xd {
            money_view_format(cash, MONEY_VIEW_TEXT, MONEY_VIEW_TEXT_SIZE, 0x21);
        }
        return MONEY_VIEW_GREEN;
    }
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    // the original's buffer (frame +0xac) has 0x11c bytes but the second format says 0x7d0: room for it here
    let mut buf = [0u8; 0x7d0];
    let b = buf.as_mut_ptr() as usize;
    money_view_format(cash, b, 0xc8, 0x22);
    if money_view_len(b) > 0xc {
        money_view_format(cash, b, 0x7d0, 0x21);
    }
    sprintf(MONEY_VIEW_TEXT, 0x98fb04, b); // "(%s)"
    G_GUI_COLOUR_RED
}

/// The Income view of the club info tab's money table: Gate Receipts, Season Tickets, TV Revenue, Merchandising,
/// Players Sold, Prize Money, Investments, Interest and Other, then the Total (see `ledger_rows`).
///
/// `fin` = the club's Finance record (ebp, `FinanceTable::get_club_finance` at 0x460686), `area` = the money table area
/// (esi, the second `Display::add_area` at 0x462b32), `align` = the value cells' alignment (ebx: 1 when the currency
/// text 0xad9ca8 is not empty, 0x40 otherwise, from 0x462a94). The original falls into the common exit at 0x467166.
/// Original: part of `club_screen_draw_info_tab` (0x463bd9..0x4657a3, club_screens.cpp).
pub(crate) unsafe fn info_tab_income_rows(fin: usize, area: usize, align: usize) {
    covered!("tel02_2 income view");
    ledger_rows(fin, area, align, &LEDGER_INCOME_ROWS, 0x98f9e4); // "  Total<%s - COMMENT - income>"
}

/// The Expenditure view of the club info tab's money table: Wages, Bonuses, Signing On Fees, Dividends, Players Bought,
/// Ground Maintenance, League Fines, Interest and Other, then the Total (see `ledger_rows`). Same parameters as
/// `info_tab_income_rows`.
/// Original: part of `club_screen_draw_info_tab` (0x4657a4..0x467165, club_screens.cpp).
pub(crate) unsafe fn info_tab_expenditure_rows(fin: usize, area: usize, align: usize) {
    covered!("tel02_2 expenditure view");
    ledger_rows(fin, area, align, &LEDGER_EXPENDITURE_ROWS, 0x98f92c); // "  Total<%s - COMMENT - expenditure>"
}

/// `ScreenManager::add_area_text` (0x804100, thiscall, ret 0x24) of the club history screen: cell (`col`, `row`) of
/// area `area` with the two layout words `a4`/`a5`, the font `font`, the style `style`, the colour and the text (0 =
/// an empty cell).
pub(crate) unsafe fn club_history_cell(area: usize, col: u32, row: i32, a4: u32, a5: usize, font: u32, style: u32, colour: usize, text: usize) -> usize {
    let f: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    f(G_SCREEN_MANAGER, area, col as usize, row as usize, a4 as usize, a5, font as usize, style as usize, colour, text)
}

/// `Text::draw_wrapped` (0x4aeaf0, thiscall, ret 0x20): draws a Text into area `area` from x1 to x2, starting at row
/// `*row` (which it moves on), with the font, the colour, the style and the last flag.
pub(crate) unsafe fn club_history_draw(text: *mut usize, area: usize, x1: i32, x2: i32, row: *mut i32, font: u32, colour: usize, style: u32) {
    (exe::text_draw_wrapped((text) as *mut usize as usize, (area) as usize as u32, (x1 as usize) as usize as i32, (x2 as usize) as usize as i32, row, (font as usize) as usize as u32, (colour) as usize as u32, (style as usize) as usize as u32, (0) as usize as u32) as usize);
}

/// `ScreenManager::set_area_scroll_row` (0x4567a0, thiscall, ret 8).
pub(crate) unsafe fn club_history_restore_scroll(area: usize, row: usize) {
    exe::screenmanager_set_area_scroll_row(G_SCREEN_MANAGER, (area) as usize as u32, (row) as usize as u32);
}

/// `ScreenManager::add_area_text` (0x804100, thiscall, ret 0x24) of the club history tabs: cell (`col`, `row`) of area
/// `area` with the two layout words, the font, the style, the colour and the text (0 = an empty cell).
pub(crate) unsafe fn history_tab_cell(area: usize, col: u32, row: i32, a4: u32, a5: usize, font: u32, style: u32, colour: usize, text: usize) -> usize {
    let f: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    f(G_SCREEN_MANAGER, area, col as usize, row as usize, a4 as usize, a5, font as usize, style as usize, colour, text)
}

/// `ScreenManager::add_area_button` (0x8040a0, thiscall, ret 0x2c): a cell that links (`link`, `data`).
pub(crate) unsafe fn history_tab_link_cell(area: usize, col: u32, row: i32, colour: usize, text: usize, link: u32, data: usize) -> usize {
    let f: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    f(G_SCREEN_MANAGER, area, col as usize, row as usize, 2, 0, 1, 2, colour, text, link as usize, data)
}

/// Where a club finished in an honours row (0x1a-byte competition history record: +0xa the winner, +0xe the runner-up):
/// 0 winner, 1 runner-up, 2 third, counting the partner club too (West Germany's national team for Germany's).
pub(crate) unsafe fn history_honours_place(row: usize, club: usize, partner: usize) -> u8 {
    let is = |c: usize| c == club || (partner != 0 && c == partner);
    if is(read_usize(row + 0xa)) {
        0
    } else if is(read_usize(row + 0xe)) {
        1
    } else {
        2
    }
}

/// The regions a club screen build is compared over (as `compare_match_screen_build`): the screen manager, the
/// Display, the screen params, the text buffer, 0xae290c, `extra` and the generator.
pub(crate) unsafe fn club_pages_build_regions(extra: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut reg = screen_build_regions();
    reg.push((G_MATCH_SCREEN_PANEL_REFRESHED_FLAG, 12));
    let slot = read_i16(SCREEN_MANAGER + screen_manager::CURRENT_SLOT) as isize;
    let block = read_usize((SCREEN_MANAGER as isize + screen_manager::PARAM_BLOCK as isize + slot * 0x300) as usize);
    if block != 0 {
        reg.push((block, 0xb4));
        reg.push((block + 0xb8, 0x200 - 0xb8));
    }
    reg.push((JOB_TEXT, 0x100));
    reg.push((0xae290c, 4));
    reg.extend_from_slice(extra);
    reg.extend_from_slice(&crate::rng::STATE_REGIONS);
    reg
}

/// Club `k` of the club table (0 for a negative index).
pub(crate) unsafe fn club_pages_club_at(k: i32) -> usize {
    if k < 0 { 0 } else { read_usize(G_CLUB_TABLE) + k as usize * club::SIZE }
}

/// The C runtime's sprintf (`FN_CRT_SPRINTF`), variadic.
pub(crate) unsafe fn part_exchange_sprintf() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_CRT_SPRINTF)
}

/// Puts a local list's walk back at its head (the iterator +0xa takes the head +2).
pub(crate) unsafe fn part_exchange_rewind(l: usize) {
    write_u32(l + 0xa, read_u32(l + 2));
}

/// The player's value as money into 0xde1f64 (`Cash::get_display_amount` 5 on a local amount, then
/// `cash_format_money`, 5) when he has a club and a value, else "Free".
pub(crate) unsafe fn part_exchange_value(player: usize) {
    let player = Staff(player);
    if player.club_job().addr() != 0 && (player.value() as i32) != 0 {
        let cash = player.value() as u32;
        let amount = exe::cash_get_display_amount(&cash, 5);
        exe::cash_format_money(0xde1f64usize, 0x7d0, amount, 5);
    } else {
        part_exchange_lang()(0xde1f64, 0x98e614); // "Free"
    }
}

/// A popup menu's item (ticked 0x2002 or plain 2) with the text at 0xde1f64.
pub(crate) unsafe fn squad_list_menu_item(area: u32, ticked: bool, link: u32, data: u32) {
    tel26_add_item(screens_current_display(G_SCREEN_MANAGER), [if ticked { 0x2002 } else { 2 }, 0, 0, 0, 0, 0, 0, 0x10, part_exchange_colour(0xae31aa),
        part_exchange_colour(0xaebdec), 1, 1, part_exchange_colour(0xaebdea), 0xde1f64, 0, link, data, area]);
}

/// A popup menu's separator line (flags 1, 0x1000010).
pub(crate) unsafe fn squad_list_separator(area: u32) {
    tel26_add_item(screens_current_display(G_SCREEN_MANAGER), [1, 0, 0, 0, 0, 0, 0, 0x1000010, part_exchange_colour(0xae31aa),
        part_exchange_colour(0xaebdec), 1, 1, part_exchange_colour(0xaebdea), 0, 0, 0, 0, area]);
}

/// The menus of a squad list's view bar `bar`, in the original's order (the links of the View, Sort By, Competitions
/// and Attributes menus in `links`): "View": Traditional 1 (title "Position(s)"), Contract 2 ("Contract Info"),
/// Selection 0x1000 ("Selection Info"), Stats 4, More Stats 8, Attributes 0x10, Other 0x20 ("Other Info"); in the
/// Traditional view "Sort By" (the ticked one's text without the first space as the title): Name 4, Position(s) 8, Squad
/// Number 0x20, Form 0x80, Morale 0x100, Condition 0x200, Nationality (or `club_label`) 0x2000, Age 0x4000, Int. Caps
/// 0x8000, Int. Goals 0x10000, Basic Wage 0x20000, Contract Expiry 0x40000, Contract Protected 0x20000000, Value 0x80000,
/// a separator, Goals 0x100000, Conceded 0x200000, Assists 0x400000, Av. Rating 0x800000; with Stats or More Stats
/// "Competitions": the six categories (`player_stats_get_category_name`, 0x7c14b0), the `category` one ticked with
/// "<category> Stats" as the title; with Attributes "Attributes": Physical 0x40, Mental 0x80, Goalkeeping 0x100,
/// Defensive 0x200, Attacking 0x400, each ticked with its "... Attributes" title.
pub(crate) unsafe fn squad_list_menus(bar: u32, v: u32, sort: u32, category: u8, title: *mut u8, links: [u32; 4], club_label: usize) {
    let lang = part_exchange_lang();
    lang(0xde1f64, 0x988004); // "View"
    let view_menu = squad_list_popup(bar, 0);
    for (label, comment, bit, t) in [(0x98eedcusize, false, 1u32, 0x98eed0usize), (0x98eec4, false, 2, 0x98eeb4), (0x98ee88, true, 0x1000, 0x98ee78),
                                     (0x98ee70, false, 4, 0), (0x98ee64, false, 8, 0), (0x98ee58, false, 0x10, 0), (0x98ee30, true, 0x20, 0x98ee24)] {
        if comment {
            lang(0xde1f64, label, 0xad9c64usize);
        } else {
            lang(0xde1f64, label);
        }
        let ticked = v & bit != 0;
        if ticked && t != 0 {
            lang(title as usize, t);
        }
        squad_list_menu_item(view_menu, ticked, links[0], bit);
    }
    if v & 1 != 0 {
        covered!("tel05_2 sort menu");
        lang(0xde1f64, 0x98ee0c); // "Sort By"
        let sort_menu = squad_list_popup(bar, 1);
        let items: [(usize, u32); 18] = [(0x98ee04, 4), (0x98edf4, 8), (0x98ede4, 0x20), (0x98eddc, 0x80), (0x98edd4, 0x100), (0x98edc8, 0x200),
            (club_label, 0x2000), (0x98eda8, 0x4000), (0x98ed9c, 0x8000), (0x98ed90, 0x10000), (0x98ed84, 0x20000), (0x98ed70, 0x40000),
            (0x98ed5c, 0x20000000), (0x98ed54, 0x80000), (0x98ed4c, 0x100000), (0x98ed40, 0x200000), (0x98ed34, 0x400000), (0x98ed28, 0x800000)];
        for (label, bit) in items {
            if bit == 0x100000 {
                squad_list_separator(sort_menu);
            }
            lang(0xde1f64, label);
            let ticked = sort & bit != 0;
            squad_list_menu_item(sort_menu, ticked, links[1], bit);
            if ticked {
                copy_text(title, 0xde1f65 as *const u8);
            }
        }
    }
    if v & 4 != 0 || v & 8 != 0 {
        lang(0xde1f64, 0x98ed18); // "Competitions"
        let comp_menu = squad_list_popup(bar, 1);
        let mut buf = [0u8; 0xc8];
        for cat in 1i32..=6 {
            exe::player_stats_get_category_name(read_usize(G_PLAYER_STATS), cat as u32, buf.as_mut_ptr(), 0xc8);
            part_exchange_sprintf()(0xde1f64, 0x98ed14, buf.as_ptr()); // " %s"
            let ticked = cat == category as i8 as i32;
            if ticked {
                lang(title as usize, 0x98ecf4, buf.as_ptr()); // "<%s> Stats"
            }
            squad_list_menu_item(comp_menu, ticked, links[2], cat as u32);
        }
    }
    if v & 0x10 != 0 {
        lang(0xde1f64, 0x98ece8); // "Attributes"
        let attr_menu = squad_list_popup(bar, 1);
        for (label, bit, t) in [(0x98ecdcusize, 0x40u32, 0x98ecc8usize), (0x98ecc0, 0x80, 0x98ecac), (0x98eca0, 0x100, 0x98ec88),
                                (0x98ec7c, 0x200, 0x98ec64), (0x98ec58, 0x400, 0x98ec40)] {
            lang(0xde1f64, label);
            let ticked = v & bit != 0;
            if ticked {
                lang(title as usize, t);
            }
            squad_list_menu_item(attr_menu, ticked, links[3], bit);
        }
    }
}

/// Column 1 of a row: the status item (see `part_exchange_rows`); on the squad numbers screen a national team's links
/// (`exe::database_is_national_team`, asked in each case as the original) are 0x7e0 (Wpm, Cup, Ine, Sct, Ret), 0x7de (Sus,
/// Yel) and 0x7dd (Inj).
pub(crate) unsafe fn squad_list_status(r: &SquadListRules, area: u32, player: usize, col: u32, row: u32) {
    let club = r.club;
    let status = exe::get_player_status_code(player, club, 0);
    let lang = part_exchange_lang();
    let national = || r.squad_screen && exe::database_is_national_team(club) != 0;
    // the item: flags, the colour twice (a9, a10), font 0xc, the text colour c13, link and data
    let item = |flags: u32, a9: u32, a10: u32, c13: u32, link: u32, data: usize| {
        tel26_add_item(screens_current_display(G_SCREEN_MANAGER), [flags, 0, 0, 0, 0, col + 1, row, 0x30, a9, a10, 0xc, 1, c13, 0xde1f64, 0, link,
            data as u32, area]);
    };
    let (c3184, cbe04, cbdcc, c3190, c317c, cbdec) = (part_exchange_colour(0xae3184), part_exchange_colour(0xaebe04), part_exchange_colour(0xaebdcc),
        part_exchange_colour(0xae3190), part_exchange_colour(0xae317c), part_exchange_colour(0xaebdec));
    match status {
        1 | 2 | 3 | 0x17 => {
            match status {
                1 => lang(0xde1f64, 0x98e9c8),                // "Wpm"
                2 => lang(0xde1f64, 0x98e998, 0xad9c64usize), // "Cup"
                3 => lang(0xde1f64, 0x98e994),                // "Ine"
                _ => lang(0xde1f64, 0x98e938, 0xad9c64usize), // "Sct"
            };
            item(0x202, cbe04, cbe04, c3184, if national() { 0x7e0 } else { 0x7db }, player);
        }
        7 | 9 => {
            lang(0xde1f64, 0x98e984); // "Fgn"
            item(0x202, cbe04, cbe04, c3184, 0x7db, player);
        }
        4 => {
            // international duty: no text of its own (0xde1f64 keeps what the previous row wrote, as the original)
            let duty = exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), player, G_CURRENT_DATE, club, 0, 0) as usize;
            if duty != 0 {
                item(0x202, cbe04, cbe04, c3184, 0x7d5, duty);
            } else {
                item(0x201, cbe04, cbe04, c3184, 0, 0);
            }
        }
        5 => {
            lang(0xde1f64, 0x98e990); // "Sus"
            item(0x202, cbdcc, cbdcc, c3184, if national() { 0x7de } else { 0x7d9 }, player);
        }
        6 => {
            lang(0xde1f64, 0x98e98c); // "Inj"
            item(0x202, cbdcc, cbdcc, c3184, if national() { 0x7dd } else { 0x7d8 }, player);
        }
        8 => {
            lang(0xde1f64, 0x98e98c); // "Inj"
            item(0x202, c3190, c3190, c3184, if national() { 0x7dd } else { 0x7d8 }, player);
        }
        0x16 => {
            lang(0xde1f64, 0x98e97c); // "Tir "
            item(0x202, c3190, c3190, c3184, 0x7d8, player);
        }
        0xa => {
            lang(0xde1f64, 0x98e978); // "Yel"
            let link = if national() { 0x7de } else { 0x7d9 };
            let colour = part_exchange_colour(0xaebdea);
            let first = exe::graphics_rgb_to_pixel(0xb4, 0xb4, 0, 0);
            let second = exe::graphics_rgb_to_pixel(0xb4, 0xb4, 0, 0);
            item(0x202, second, first, colour, link, player);
        }
        0x10 => {
            lang(0xde1f64, 0x98e900); // "Ctr"
            item(0x202, c317c, c317c, cbdec, 0x7da, player);
        }
        0x11 => {
            lang(0xde1f64, 0x98e8f8); // "Ret "
            item(0x202, c317c, c317c, cbdec, if national() { 0x7e0 } else { 0x7db }, player);
        }
        0xb..=0xf | 0x12..=0x14 | 0x18 => {
            match status {
                0xe => lang(0xde1f64, 0x98e908, 0xad9c64usize), // "Lst"
                _ => lang(0xde1f64, match status {
                    0x18 => 0x98e970, // "Trn"
                    0xb => 0x98e974,  // "Bid"
                    0xc => 0x98e96c,  // "Wnt"
                    0xd => 0x98e934,  // "Req"
                    0xf => 0x98e904,  // "Loa"
                    0x12 => 0x98e8f0, // "Bos "
                    0x13 => 0x98e8e8, // "Unh "
                    _ => 0x98e8e0,    // "Fut "
                }),
            };
            item(0x202, c317c, c317c, cbdec, 0x7db, player);
        }
        _ => {}
    }
}

/// Column 3 of the Traditional view, by the sort (one of them, in this order): squad number (`SQUAD_MANAGER`: when the club
/// uses numbers and he holds one, its absolute value, else empty), form, morale, condition, nationality (a button to his
/// nation's team, link 0x7d5; on the squad numbers screen for a national or all-star team, a button to his club), age,
/// caps, international goals, basic wage and contract expiry (the club's contract, else his contract, else empty),
/// protection ("Yes" when his contract is in the FIFA protected period, else "No"), value (a button for a database player
/// whose club the human runs: link 0xa, 0xd on the squad numbers screen), goals, conceded, assists, rating (the season's
/// statistics), else his positions.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn squad_list_sort_cell(r: &SquadListRules, area: u32, player: usize, col: u32, row: u32, injury: u8, contract_club: usize, contract: usize, buf: usize) {
    let player = Staff(player);
    let sort = r.sort;
    let pdata = player.player().addr();
    let cbdec = part_exchange_colour(0xaebdec);
    let c = col + 3;
    let cell = |a6: u32, a7: u32| {
        tel26_area_text([area, c, row, 1, 0, a6, a7, cbdec, 0xde1f64]);
    };
    let number = |n: u32| {
        part_exchange_sprintf()(0xde1f64, 0x98806c, n); // "%d"
    };
    if sort & 0x20 != 0 {
        let squad = read_usize(G_SQUAD_MANAGER);
        if exe::squad_manager_club_uses_squad_numbers(squad, r.club, 0) != 0 && exe::squad_manager_get_squad_number(squad, player, r.club) != 0 {
            let n = exe::squad_manager_get_squad_number(read_usize(G_SQUAD_MANAGER), player, r.club) as i32;
            number(n.wrapping_abs() as u32);
        } else {
            write_u8(0xde1f64, 0);
        }
        cell(0xc, 2);
    } else if sort & 0x80 != 0 {
        part_exchange_form(player.addr());
        cell(0xc, 1);
    } else if sort & 0x100 != 0 {
        part_exchange_morale(player.addr());
        cell(0xc, 2);
    } else if sort & 0x200 != 0 {
        part_exchange_condition(player.addr(), injury);
        cell(0xc, 2);
    } else if sort & 0x2000 != 0 {
        if !r.squad_screen || exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), r.club) == 0 {
            let nation = player.nation().addr();
            if nation != 0 {
                let team = exe::database_get_national_team(nation);
                tel26_area_button([area, c, row, 1, 0, 0xc, 2, cbdec, (nation + 0x53) as u32, 0x7d5, team as u32]);
            }
        } else {
            let club_job = player.club_job().addr();
            if club_job != 0 {
                tel26_area_button([area, c, row, 1, 0, 1, 2, cbdec, (club_job + club::SHORT_NAME) as u32, 0x7d5, club_job as u32]);
            }
        }
    } else if sort & 0x4000 != 0 {
        number((player.age() as i8) as i32 as u32);
        cell(0xc, 2);
    } else if sort & 0x8000 != 0 {
        number(player.int_apps() as u32);
        cell(0xc, 2);
    } else if sort & 0x10000 != 0 {
        number(player.int_goals() as u32);
        cell(0xc, 2);
    } else if sort & 0x20000 != 0 {
        if contract_club != 0 || contract != 0 {
            part_exchange_wage(if contract_club != 0 { contract_club } else { contract }, buf);
        } else {
            write_u8(0xde1f64, 0);
        }
        cell(1, 2);
    } else if sort & 0x40000 != 0 {
        if contract_club != 0 || contract != 0 {
            part_exchange_end_date(if contract_club != 0 { contract_club } else { contract }, buf);
        } else {
            write_u8(0xde1f64, 0);
        }
        cell(1, 2);
    } else if sort & 0x20000000 != 0 {
        let yes = contract != 0 && exe::staff_contract_check_fifa_protected_period(contract, 0usize, 0usize, 0usize) != 0;
        part_exchange_lang()(0xde1f64, if yes { 0x98e8d0 } else { 0x98e8cc }); // "Yes", "No"
        cell(1, 2);
    } else if sort & 0x80000 != 0 {
        part_exchange_value(player.addr());
        let club_job = player.club_job().addr();
        if (player.is_null() || read_i32(player.addr()) < read_i32(G_STAFF_COUNT)) && club_job != 0 && exe::human_manager_current_controls_club(club_job, 0, 1) != 0 {
            tel26_area_button([area, c, row, 1, 0, 0xc, 2, cbdec, 0xde1f64, if r.squad_screen { 0xd } else { 0xa }, player.addr() as u32]);
        } else {
            cell(0xc, 2);
        }
    } else if sort & 0xf00000 != 0 {
        let field = if sort & 0x100000 != 0 { 2 } else if sort & 0x200000 != 0 { 3 } else if sort & 0x400000 != 0 { 5 } else { 0x11 };
        part_exchange_stat(pdata, 6, field);
        cell(0xc, 2);
    } else {
        part_exchange_positions(pdata, 0xde1f64, 0x7d0);
        cell(1, 2);
    }
}

/// The columns 3.. of the views other than Traditional: contract (the squad status of the club's contract as a button,
/// link 0xa (0xd on the squad numbers screen), when the human runs the club, else "  -"; on the squad numbers screen for
/// a national or all-star team his club as a button, link 0x7d5; the wage and the end date of the club's contract or
/// "  -"; the release clause of his contract (not for a national team on the squad numbers screen): a button, link 0x7da,
/// in the red colour (2) or the cell colour (1), else a text, or "  -"); selection (positions, form, morale, condition,
/// the stat fields 1 and 0x11 of category 6); statistics (the table's fields in the chosen category, the field 0x11 in the
/// text colour); attributes (each attribute of the table, `exe::staff_get_player_attribute`); other (the nation as a
/// button to its team, age, caps, international goals, form, morale, condition).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn squad_list_view_cells(r: &SquadListRules, area: u32, player: usize, col: u32, row: u32, injury: u8, contract_club: usize, contract: usize, buf: usize) {
    let player = Staff(player);
    let pdata = player.player().addr();
    let (cbdec, cbde8, cbdd4, c31a8) = (part_exchange_colour(0xaebdec), part_exchange_colour(0xaebde8), part_exchange_colour(0xaebdd4), part_exchange_colour(0xae31a8));
    let cell = |c: u32, a6: u32, a7: u32, colour: u32, text: usize| {
        tel26_area_text([area, col + c, row, 1, 0, a6, a7, colour, text as u32]);
    };
    let sprintf = part_exchange_sprintf();
    let number = |n: u32| {
        sprintf(0xde1f64, 0x98806c, n); // "%d"
    };
    let flags = r.view_flags;
    if flags & 2 != 0 {
        if !r.squad_screen || exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), r.club) == 0 {
            if exe::human_manager_current_controls_club(r.club, 0, 1) != 0 && contract_club != 0 {
                exe::staff_contract_get_squad_status_text(contract_club, buf, 0x7d0);
                sprintf(0xde1f64, 0x987ff0, buf); // "  %s"
                tel26_area_button([area, col + 3, row, 1, 0, 1, 2, cbde8, 0xde1f64, if r.squad_screen { 0xd } else { 0xa }, player.addr() as u32]);
            } else {
                cell(3, 1, 2, cbde8, 0x98e8c8); // "  -"
            }
        } else {
            let club_job = player.club_job();
            if !club_job.is_null() {
                sprintf(0xde1f64, 0x987ff0, club_job.addr() + club::SHORT_NAME);
                tel26_area_button([area, col + 3, row, 1, 0, 1, 2, cbde8, 0xde1f64, 0x7d5, club_job.addr() as u32]);
            }
        }
        if contract_club != 0 {
            part_exchange_wage(contract_club, buf);
        } else {
            sprintf(0xde1f64, 0x98e8c8);
        }
        cell(4, 1, 2, cbdec, 0xde1f64);
        if contract_club != 0 {
            part_exchange_end_date(contract_club, buf);
        } else {
            sprintf(0xde1f64, 0x98e8c8);
        }
        cell(5, 1, 2, cbdec, 0xde1f64);
        if r.squad_screen && exe::database_is_national_team(r.club) != 0 {
            return;
        }
        if contract != 0 {
            let k = exe::staff_contract_get_release_clause_abbreviation(contract, buf, 0x7d0);
            sprintf(0xde1f64, 0x987ff0, buf);
            if k == 2 {
                tel26_area_button([area, col + 6, row, 1, 0, 1, 2, c31a8, 0xde1f64, 0x7da, player.addr() as u32]);
            } else if k == 1 {
                tel26_area_button([area, col + 6, row, 1, 0, 1, 2, cbdd4, 0xde1f64, 0x7da, player.addr() as u32]);
            } else {
                cell(6, 1, 2, cbdd4, 0xde1f64);
            }
        } else {
            cell(6, 1, 2, cbdd4, 0x98e8c8);
        }
    } else if flags & 0x1000 != 0 {
        part_exchange_positions(pdata, buf, 0xc8);
        sprintf(0xde1f64, 0x987ff0, buf);
        cell(3, 1, 2, cbde8, 0xde1f64);
        part_exchange_form(player.addr());
        cell(4, 0xc, 1, cbdec, 0xde1f64);
        part_exchange_morale(player.addr());
        cell(5, 0xc, 2, cbdec, 0xde1f64);
        part_exchange_condition(player.addr(), injury);
        cell(6, 0xc, 2, cbdd4, 0xde1f64);
        part_exchange_stat(pdata, 6, 1);
        cell(7, 0xc, 2, cbde8, 0xde1f64);
        part_exchange_stat(pdata, 6, 0x11);
        cell(8, 0xc, 2, cbde8, 0xde1f64);
    } else if flags & 4 != 0 || flags & 8 != 0 {
        let table = if flags & 4 != 0 { 0x98de84 } else { 0x98de90 };
        let mut k = 0usize;
        while read_u8(table + k) != 0xff {
            let field = read_u8(table + k);
            // simplification: param 6 and the field are bytes in a stack slot and a register whose upper bytes are
            // garbage in the original (chars)
            part_exchange_stat(pdata, r.category as u32, field as u32);
            cell(k as u32 + 3, 0xc, 2, if field == 0x11 { cbdec } else { cbde8 }, 0xde1f64);
            k += 1;
        }
    } else if flags & 0x10 != 0 {
        let table = r.table;
        let mut k = 0usize;
        while read_u8(table + k) != 0xff {
            let attr = read_u8(table + k);
            number(exe::staff_get_player_attribute(player, attr as u32) as i32 as u32);
            cell(k as u32 + 3, 0xc, 2, cbde8, 0xde1f64);
            k += 1;
        }
    } else if flags & 0x20 != 0 {
        let nation = player.nation().addr();
        if nation != 0 {
            let team = exe::database_get_national_team(nation);
            tel26_area_button([area, col + 3, row, 1, 0, 0xc, 2, cbde8, (nation + 0x53) as u32, 0x7d5, team as u32]);
        }
        number((player.age() as i8) as i32 as u32);
        cell(4, 0xc, 2, cbde8, 0xde1f64);
        number(player.int_apps() as u32);
        cell(5, 0xc, 2, cbde8, 0xde1f64);
        number(player.int_goals() as u32);
        cell(6, 0xc, 2, cbde8, 0xde1f64);
        part_exchange_form(player.addr());
        cell(7, 0xc, 1, cbdec, 0xde1f64);
        part_exchange_morale(player.addr());
        cell(8, 0xc, 2, cbdec, 0xde1f64);
        part_exchange_condition(player.addr(), injury);
        cell(9, 0xc, 2, cbdd4, 0xde1f64);
    }
}

/// Whether the club's team sheet holds player `p`: the human's (`exe::tactics_find_selection_slot` with the reserves
/// flag, a place 0 or more) when he runs the club, otherwise the `team_sheet` given (20 players of the club's block).
pub(crate) unsafe fn squad_list_in_team_sheet(club: usize, p: usize, reserves: u32, team_sheet: usize) -> bool {
    if exe::human_manager_current_controls_club(club, 0, 1) != 0 {
        return exe::tactics_find_selection_slot(read_usize(G_TACTICS), club, p, reserves) as i8 > -1;
    }
    if team_sheet == 0 {
        return false;
    }
    let block = team_sheet + TEAM_SHEET_BLOCK * (read_u32(club) != read_u32(team_sheet)) as usize;
    (0..20).any(|k| read_u32(block + TEAM_SHEET_PLAYERS + TEAM_SHEET_PLAYER_SIZE * k) == read_u32(p))
}

/// The people of a squad list, in order. Each node holds a 4-byte block that `llist_add_tail` copied onto the heap,
/// so the node data is that block's address (another one in each run) and the person is the dword inside it.
pub(crate) unsafe fn squad_list_people(list: usize) -> Vec<usize> {
    llist_items(list).into_iter().map(|block| if block == 0 { 0 } else { read_usize(block) }).collect()
}

/// `Injury::is_player_on_holiday` (0x631550, thiscall, ret 4).
pub(crate) unsafe fn status_on_holiday(injury: usize, p: usize) -> bool {
    exe::injury_is_player_on_holiday(injury, p) != 0
}

/// `STAFF_CONTRACT::is_rolling` (0x85fa20, thiscall): the game reads al.
pub(crate) unsafe fn status_contract_rolling(contract: usize) -> bool {
    exe::staff_contract_is_rolling(contract) != 0
}

/// The contract checks that end both orders of `get_player_status_code`: leaving on a Bosman (0x12), grievances or
/// contract flags (0x13; `with_flags` asks `exe::staff_contract_get_flags`), a first-team squad status 2 with no
/// higher status bits for a player (0x14).
pub(crate) unsafe fn status_contract_code(p: usize, contract: usize, with_flags: bool) -> Option<u8> {
    let contract = Contract(contract);
    let p = Staff(p);
    if contract.leaving_on_bosman() != 0 {
        return Some(0x12);
    }
    if contract.grievances() != 0 || (with_flags && exe::staff_contract_get_flags(contract) != 0) {
        return Some(0x13);
    }
    if contract.contract_type() & 0x3f == 2 && contract.squad_status() & 0xf0 == 0 && p.player().addr() != 0 {
        return Some(0x14);
    }
    None
}

/// A method of the Friendly object ([G_FRIENDLY]) without arguments that returns a pointer (0x5c68b0 the human club's
/// open invitation, 0x5c6920 its open dates request, 0x5c6830 its tournament).
pub(crate) unsafe fn club_action_friendly_find(f: usize) -> usize {
    let g: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(f);
    g(read_usize(G_FRIENDLY))
}

/// `Friendly::get_club_date_status` (0x5c75a0, thiscall, ret 0x24): the date status of `club` on `date` (1 = free
/// for the match), with no chosen dates.
pub(crate) unsafe fn club_action_date_status(club: usize, date: usize, kind: u32, check_stadium: u32) -> u8 {
    type F = unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut usize, *mut usize, usize, u32, u32, u32) -> u8;
    let f: F = core::mem::transmute(FN_FRIENDLY_GET_CLUB_DATE_STATUS);
    let (mut tournament, mut fixture, mut request) = (0usize, 0usize, 0usize);
    f(read_usize(G_FRIENDLY), club, date, &mut tournament, &mut fixture, &mut request, 0, 0, kind, check_stadium)
}

/// The PlayerStats object ([0xb74350]) and its methods used by the squad tab.
pub(crate) unsafe fn squad_stat(pdata: usize, category: u32, field: u32, out: usize) {
    exe::playerstats_format_player_stat(read_usize(0xb74350), pdata, category, field, out, 0x7d0, 1, 0, 1);
}

/// `PlayerStats::format_recent_ratings` (0x7c1660, thiscall, ret 0xc) into 0xde1f64.
pub(crate) unsafe fn squad_recent_ratings(pdata: usize) {
    exe::playerstats_format_recent_ratings(read_usize(0xb74350), pdata, 0xde1f64, 0x7d0);
}

/// `contract_manager_write_morale_text` (0x4cc2e0, ret 0xc) into 0xde1f64 (called with ecx = [0xae1050]).
pub(crate) unsafe fn squad_morale(player: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_CONTRACT_MANAGER_WRITE_MORALE_TEXT);
    f(read_usize(G_CONTRACT_MANAGER), player, 0xde1f64, 0x7d0);
}

/// `database_format_player_positions(pdata, out, size, style)` (0x53ee50, cdecl).
pub(crate) unsafe fn squad_positions(pdata: usize, out: usize, size: u32) {
    exe::database_format_player_positions(pdata, out, size, 1);
}

/// `STAFF_CONTRACT::format_end_date_text(out)` (0x85e8f0, thiscall, ret 4).
pub(crate) unsafe fn squad_end_date(contract: usize, out: usize) {
    exe::staff_contract_format_end_date_text(contract, out);
}

/// A contract's wage text into `buf` (style 0, flags 2; again with flags 1 when longer than 8 characters).
pub(crate) unsafe fn squad_wage_text(contract: usize, buf: &mut [u8; 0x7d0]) {
    squad_wage(contract, buf.as_mut_ptr() as usize, 2);
    if club_screens_text_length(buf.as_ptr()) > 8 {
        squad_wage(contract, buf.as_mut_ptr() as usize, 1);
    }
}

/// The rows of one money view and its Total row (row 0xc; row 0xb stays empty).
///
/// Each row (rows 2 to 0xa): the translated label (`langlib_sprintf`, with the empty text as the `%s` argument where the
/// label has one) in column 0, pale grey; then the four columns (this month, last month, this season, last season) from
/// the club's Finance record `fin`: the dword of the row's field in that column's 0x50-byte block (the low dword for the
/// i64 transfer fields) becomes the cash value, formatted as money (`ledger_format`) in yellow, and its unrounded
/// display amount (`Cash::get_display_amount` with 0x20) is added to the column's 64-bit total. The Total row: the label
/// and each column's total turned back into a cash value (`Cash::set_from_amount`) and formatted, all in the total
/// colour.
pub(crate) unsafe fn ledger_rows(fin: usize, area: usize, align: usize, rows: &[(usize, usize, bool); 9], total_label: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let set_from_amount: unsafe extern "thiscall" fn(*mut u32, i64) = core::mem::transmute(LEDGER_FN_CASH_SET_FROM_AMOUNT);
    let mut totals = [0i64; 4];
    let mut cash = 0u32;
    for (k, &(label, field, comment)) in rows.iter().enumerate() {
        let row = 2 + k;
        if comment {
            text(LEDGER_TEXT, label, LEDGER_EMPTY_TEXT);
        } else {
            text(LEDGER_TEXT, label);
        }
        ledger_cell(area, 0, row, 1, 3, G_GUI_COLOUR_PALE_GREY);
        for (c, block) in LEDGER_BLOCKS.iter().enumerate() {
            cash = read_u32(fin + block + field - finance::MONTH_PLAYERS_BOUGHT);
            ledger_format(&cash);
            totals[c] = totals[c].wrapping_add(exe::cash_get_display_amount(&cash, 0x20));
            ledger_cell(area, c + 1, row, align, 2, G_GUI_COLOUR_YELLOW);
        }
    }
    text(LEDGER_TEXT, total_label, LEDGER_EMPTY_TEXT);
    ledger_cell(area, 0, 0xc, 1, 3, LEDGER_COLOUR_TOTAL);
    for (c, total) in totals.iter().enumerate() {
        set_from_amount(&mut cash, *total);
        ledger_format(&cash);
        ledger_cell(area, c + 1, 0xc, align, 2, LEDGER_COLOUR_TOTAL);
    }
}

/// `PlayerStats::format_player_stat(pdata, category, field, 0xde1f64, 0x7d0, 1, 0, 1)` (0x7c25b0, thiscall on
/// [0xb74350], ret 0x20).
pub(crate) unsafe fn part_exchange_stat(pdata: usize, category: u32, field: u32) {
    exe::playerstats_format_player_stat(read_usize(G_PLAYER_STATS), pdata, category, field, 0xde1f64, 0x7d0, 1, 0, 1);
}

/// The player's form: "-" for a person made in the game, else `PlayerStats::format_recent_ratings` (0x7c1660) into
/// 0xde1f64.
pub(crate) unsafe fn part_exchange_form(player: usize) {
    let player = Staff(player);
    if !player.is_null() && read_i32(player.addr()) >= read_i32(G_STAFF_COUNT) {
        copy_text(0xde1f64 as *mut u8, 0x98ba0c as *const u8);
    } else {
        exe::playerstats_format_recent_ratings(read_usize(G_PLAYER_STATS), player.player().addr(), 0xde1f64, 0x7d0);
    }
}

/// The player's morale text (`contract_manager_write_morale_text`, 0x4cc2e0, with ecx = [0xae1050]) into 0xde1f64.
pub(crate) unsafe fn part_exchange_morale(player: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_CONTRACT_MANAGER_WRITE_MORALE_TEXT);
    f(read_usize(G_CONTRACT_MANAGER), player, 0xde1f64, 0x7d0);
}

/// The player's condition: "Inj" when injured (status 2), else "<condition>%" (`exe::injury_get_condition`, 1).
pub(crate) unsafe fn part_exchange_condition(player: usize, injury: u8) {
    if injury == 2 {
        part_exchange_lang()(0xde1f64, 0x98e98c); // "Inj"
    } else {
        part_exchange_sprintf()(0xde1f64, 0x98e8d4, exe::injury_get_condition(read_usize(G_INJURY), player, 1) as i32); // "%d%%"
    }
}

/// `database_format_player_positions(pdata, out, size, 1)` (0x53ee50, cdecl).
pub(crate) unsafe fn part_exchange_positions(pdata: usize, out: usize, size: u32) {
    exe::database_format_player_positions(pdata, out, size, 1);
}

/// A contract's wage text into `buf` (`STAFF_CONTRACT::format_wage_text`, 0x85e750, style 0, cash flags 2; again with
/// flags 1 when longer than 8 characters), then "  %s" into 0xde1f64.
pub(crate) unsafe fn part_exchange_wage(contract: usize, buf: usize) {
    exe::staff_contract_format_wage_text(contract, buf, 0, 2);
    if core::ffi::CStr::from_ptr(buf as *const core::ffi::c_char).to_bytes().len() > 8 {
        exe::staff_contract_format_wage_text(contract, buf, 0, 1);
    }
    part_exchange_sprintf()(0xde1f64, 0x987ff0, buf); // "  %s"
}

/// A contract's end date (`STAFF_CONTRACT::format_end_date_text`, 0x85e8f0) into `buf`, then "  %s" into 0xde1f64.
pub(crate) unsafe fn part_exchange_end_date(contract: usize, buf: usize) {
    exe::staff_contract_format_end_date_text(contract, buf);
    part_exchange_sprintf()(0xde1f64, 0x987ff0, buf);
}

/// A menu button in column `col` of the bar `bar` and its popup area (the label already at 0xde1f64).
pub(crate) unsafe fn squad_list_popup(bar: u32, col: u32) -> u32 {
    let b = tel26_area_button([bar, col, 0, 0x30, part_exchange_colour(0xae317e), 0xc, 1, part_exchange_colour(0xae3184), 0xde1f64, 0, 0]);
    tel26_add_area(screens_current_display(G_SCREEN_MANAGER), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, part_exchange_colour(0xae31aa), b])
}

/// `STAFF_CONTRACT::format_wage_text(out, style, flags)` (0x85e750, thiscall, ret 0xc).
pub(crate) unsafe fn squad_wage(contract: usize, out: usize, flags: u32) {
    exe::staff_contract_format_wage_text(contract, out, 0, flags);
}

/// Formats the cash value `cash` into the shared text as money (`cash_format_money` of `Cash::get_display_amount` with
/// flags 0x22, 2000 bytes); when that is longer than 13 characters, again with flags 0x21 (the short form).
pub(crate) unsafe fn ledger_format(cash: &u32) {
    exe::cash_format_money(LEDGER_TEXT, 0x7d0, exe::cash_get_display_amount(cash, 0x22), 0x22);
    let mut n = 0usize;
    while read_u8(LEDGER_TEXT + n) != 0 {
        n += 1;
    }
    if n > 0xd {
        exe::cash_format_money(LEDGER_TEXT, 0x7d0, exe::cash_get_display_amount(cash, 0x21), 0x21);
    }
}

/// One cell of the money table `area`: `ScreenManager::add_area_text`(area, col, row, 1, black, align, 2, fg, text);
/// the labels go in column 0 with (1, 3) instead of (align, 2).
pub(crate) unsafe fn ledger_cell(area: usize, col: usize, row: usize, a5: usize, a6: usize, fg: usize) {
    let cell: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    cell(SCREEN_MANAGER, area, col, row, 1, colour_word(G_GUI_COLOUR_BLACK), a5, a6, colour_word(fg), LEDGER_TEXT);
}

/// Where a match record keeps its two team blocks, the size of one and the block's destructor.
pub(crate) const MATCH_TEAM_PAIR: usize = 0x11f;

pub(crate) const MATCH_TEAM_BLOCK_SIZE: usize = 0x625;

pub(crate) const MATCH_TEAM_BLOCK_DESTRUCTOR: usize = 0x453c10;

/// The date field of a dated record (the average rating record and its kin).
pub(crate) const RATING_RECORD_DATE: usize = 0x14;

/// The size of an `Area` in a `Display` (the areas start at +4: +4 + 0xbf1 × k).
pub(crate) const DISPLAY_AREA_SIZE: usize = 0xbf1;

/// club_screens.cpp's source path (0x98e598 "E:\dev\CM3\cm3 00-01\cm3\code\club_screens.cpp"), for the error boxes.
pub(crate) const CLUB_SCREENS_CPP_PATH: usize = 0x98e598;

/// A tab of the club screen's tab bar (`gui_add_tab_bar`, 0xd4 bytes each: +2 the colour, +4 the text, +0xcc the kind,
/// +0xd0 the link).
pub(crate) const CLUB_TAB_SIZE: usize = 0xd4;

/// The squad tab's state: what the original keeps in registers and stack slots across its pieces.
pub(crate) struct SquadTab {
    pub(crate) sm: usize,
    pub(crate) club: usize,
    pub(crate) reserves: u32,
    pub(crate) last_match: usize,
    pub(crate) next_match: usize,
    pub(crate) view: u32,
    pub(crate) sort: u32,
    pub(crate) category: u8,
    pub(crate) filter: u32,
    pub(crate) p19: u8,
    pub(crate) p1a: u8,
    pub(crate) best: [u32; 3],
    pub(crate) human: usize,
    pub(crate) top: i32,
    pub(crate) bottom: i32,
    pub(crate) bar: u32,
    pub(crate) title: [u8; 0xc8],
    pub(crate) list: [u8; 0x20],
    pub(crate) from_record: bool,
    pub(crate) attributes: usize,
    /// the area the original keeps in ebp (the rows go there when the view has none of its own)
    pub(crate) ebp: u32,
}

impl SquadTab {
    pub(crate) unsafe fn display(&self) -> usize {
        screens_current_display(self.sm)
    }
    pub(crate) unsafe fn add_area(&self, a: [u32; 11]) -> u32 {
        let f: DisplayAddAreaFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
        f(self.display(), a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10])
    }
    pub(crate) unsafe fn add_item(&self, a: [u32; 18]) -> u32 {
        let f: DisplayAddItemFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
        f(self.display(), a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13], a[14], a[15], a[16], a[17])
    }
    pub(crate) unsafe fn text(&self, area: u32, col: i32, row: i32, a4: u32, a5: u32, a6: u32, a7: u32, colour: u32, text: usize) {
        let f: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
        f(self.sm, area as usize, col as usize, row as usize, a4 as usize, a5 as usize, a6 as usize, a7 as usize, colour as usize, text);
    }
    pub(crate) unsafe fn button(&self, area: u32, col: i32, row: i32, a4: u32, a5: u32, a6: u32, a7: u32, colour: u32, text: usize, link: u32, data: usize) -> u32 {
        let f: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
        f(self.sm, area as usize, col as usize, row as usize, a4 as usize, a5 as usize, a6 as usize, a7 as usize, colour as usize, text, link as usize, data) as u32
    }
    /// A popup menu's item (ticked 0x2002 or plain 2) with the text at 0xde1f64.
    pub(crate) unsafe fn menu_item(&self, area: u32, ticked: bool, link: u32, data: u32) {
        self.add_item([if ticked { 0x2002 } else { 2 }, 0, 0, 0, 0, 0, 0, 0x10, squad_colour(0xae31aa), squad_colour(0xaebdec), 1, 1,
            squad_colour(0xaebdea), 0xde1f64, 0, link, data, area]);
    }
    /// A popup menu's separator line (flags 1, 0x1000010).
    pub(crate) unsafe fn separator(&self, area: u32) {
        self.add_item([1, 0, 0, 0, 0, 0, 0, 0x1000010, squad_colour(0xae31aa), squad_colour(0xaebdec), 1, 1, squad_colour(0xaebdea), 0, 0, 0, 0, area]);
    }
    /// A menu button in a cell of `area` and its popup area.
    pub(crate) unsafe fn popup_button(&self, area: u32, col: i32, label: usize) -> u32 {
        squad_text(0xde1f64, label);
        let b = self.button(area, col, 0, 0x30, squad_colour(0xae317e), 0xc, 1, squad_colour(0xae3184), 0xde1f64, 0, 0);
        self.add_area([0, 0, 0, 0, 0, 0, 0, 0, 0x130, squad_colour(0xae31aa), b])
    }
    /// A column heading button (font `style`, link `link`, data `data`) in row 0 of `area`.
    pub(crate) unsafe fn heading(&self, area: u32, col: i32, style: u32, label: usize, link: u32, data: u32) {
        if label != 0 {
            squad_text(0xde1f64, label);
        }
        self.button(area, col, 0, 0x30, squad_colour(0xae317e), style, 1, squad_colour(0xae3184), 0xde1f64, link, data as usize);
    }
    /// The title text: the ticked item's text without its first space.
    pub(crate) unsafe fn ticked_title(&mut self) {
        copy_text(self.title.as_mut_ptr(), 0xde1f65 as *const u8);
    }
}

/// The view's column weights (13 columns) and the positions view's (8, or what the 1080 exe puts there).
pub(crate) struct SquadWeights {
    pub(crate) two: [u8; 8],
    pub(crate) contract: [u8; 13],
    pub(crate) stats: [u8; 13],
    pub(crate) attributes: [u8; 13],
    pub(crate) other: [u8; 13],
    pub(crate) selection: [u8; 13],
}

pub(crate) const NEXT_MATCH_RECORD_KEY: usize = 0x7d6; // the screen record's manager (staff) pointer, +0x300 per slot (scrman's record KEY)

pub(crate) const NEXT_MATCH_COMP_GROUP: usize = 0x50; // byte of the competition object: its group of related competitions (0xff none) [inference]

pub(crate) const NEXT_MATCH_CITY_NAME: usize = 0x4; // the city record's name (text)

pub(crate) const NEXT_MATCH_OFFICIAL_SIZE: usize = 0x2b; // one record of G_OFFICIAL_TABLE

pub(crate) const NEXT_MATCH_COLOUR_POPUP: usize = 0xae31aa; // the popup list background colour word

pub(crate) const NEXT_MATCH_COLOUR_ORANGE: usize = 0xaebdd4; // g_gui_colour_orange

pub(crate) const NEXT_MATCH_EMPTY_TEXT: usize = 0xad9c64; // "" (the langlib comment argument)

pub(crate) const NEXT_MATCH_TEXT: usize = 0xde1f64; // the shared text buffer

/// `key_nation_format_club_season` (0x669630): cdecl(year, club, buffer, size) -> int, ret 0.
pub(crate) const FIXTURES_FN_FORMAT_CLUB_SEASON: usize = 0x669630;

/// `add_club_fixture_row` (0x483a90): cdecl(club, fixture, area, row, date_link_mode), ret 0; nonzero when the row is
/// the club's next fixture [inference: the screen keeps that row to scroll to].
pub(crate) const FIXTURES_FN_ADD_CLUB_FIXTURE_ROW: usize = 0x483a90;

/// The colours this screen pushes that have no name in globals.rs (the words at these addresses).
pub(crate) const FIXTURES_COLOUR_POPUP: usize = 0xae31aa;

pub(crate) const FIXTURES_COLOUR_VENUE: usize = 0xaebde8;

pub(crate) const FIXTURES_COLOUR_COMP: usize = 0xaebdde;

pub(crate) const FIXTURES_COLOUR_RESULT_BG: usize = 0xae3192;

/// "---": the empty result column of a friendly, tour or tournament row.
pub(crate) const FIXTURES_NO_RESULT_TEXT: usize = 0x98f858;

/// The empty langlib argument (`""`).
pub(crate) const FIXTURES_EMPTY_ARG: usize = 0xad9c64;

/// A friendly match record (`friendly_match`): nonzero once it can no longer be cancelled [inference: with it set the
/// cancel mark (item 0x16) is not offered].
pub(crate) const FIXTURES_FRIENDLY_LOCKED: usize = 0x8;

/// A tour record (0x5c75a0 status 6): the touring club, the nation toured and a flag that hides the cancel mark
/// [inference].
pub(crate) const FIXTURES_TOUR_CLUB: usize = 0x0;

pub(crate) const FIXTURES_TOUR_NATION: usize = 0x4;

pub(crate) const FIXTURES_TOUR_LOCKED: usize = 0x103;

/// The competition's id inside the tournament competition object [inference: the value of the 0x7d0 link].
pub(crate) const FIXTURES_COMP_ID: usize = 0x4;

/// The shared text buffer.
pub(crate) const FIXTURES_TEXT: usize = 0xde1f64;

/// The Spanish clubs' badge pictures: 8-byte entries (the club's English name, the picture file under `CM3_PICTURES`),
/// ended by an entry with an empty name.
pub(crate) const BADGE_PICTURE_TABLE: usize = 0x98dedc;

pub(crate) const BADGE_PICTURES_KEY: usize = 0x9899e0; // "CM3_PICTURES"

pub(crate) const BADGE_OPEN_MODE: usize = 0x98f928; // "rb"

/// The shared text buffer the screen builders format into.
pub(crate) const INFO_TAB_TEXT: usize = 0xde1f64;

/// The empty argument the "<%s - COMMENT - ...>" texts take.
pub(crate) const INFO_TAB_NO_COMMENT: usize = 0xad9c64;

/// The menu background colour word (unnamed in globals.rs; match_screens uses the literal).
pub(crate) const INFO_TAB_COLOUR_MENU: usize = 0xae31aa;

/// g_gui_colour_orange (the "Non-Playing Staff" heading).
pub(crate) const INFO_TAB_COLOUR_ORANGE: usize = 0xaebdd4;

/// The function-static green: bit 0 of the guard byte, then the pixel word (`graphics_rgb_to_pixel(0, 0xc0, 0, 0)`).
pub(crate) const INFO_TAB_STATIC_GUARD: usize = 0xadab32;

pub(crate) const INFO_TAB_STATIC_GREEN: usize = 0xadab48;

/// A byte the General Info view sets to 1 before sorting the non-playing staff [inference: read by the sort
/// comparers 0x487660/0x4875a0].
pub(crate) const INFO_TAB_STAFF_SORTED_FLAG: usize = 0xadab33;

/// The Welsh clubs of the English pyramid (g_key_club_swansea_city, g_key_club_cardiff_city, g_key_club_wrexham): their
/// nation reads "Wales".
pub(crate) const INFO_TAB_KEY_SWANSEA_CITY: usize = 0x9d0430;

pub(crate) const INFO_TAB_KEY_CARDIFF_CITY: usize = 0x9d0434;

pub(crate) const INFO_TAB_KEY_WREXHAM: usize = 0x9d043c;

/// club_collect_non_playing_staff (0x544c70, cdecl(club, out, max) -> count in AL).
pub(crate) const INFO_TAB_FN_COLLECT_NON_PLAYING_STAFF: usize = 0x544c70;

/// The qsort comparers of the non-playing staff: national team and club.
pub(crate) const INFO_TAB_FN_SORT_NATIONAL_STAFF: usize = 0x487660;

pub(crate) const INFO_TAB_FN_SORT_CLUB_STAFF: usize = 0x4875a0;

/// The job code whose (non-regen) holder is listed as plain text, not a link [inference: the chairman].
pub(crate) const INFO_TAB_JOB_PLAIN: u8 = 1;

/// What the info tab's frame holds when it hands over to the Stats view (0x4619d3) or to the money views (0x462a80).
/// Both entries see the same frame (esp = the body's esp, E): the fields say which register or slot they mirror.
pub(crate) struct InfoTabFrame {
    /// [E+0x1c] and edi: the club (screen param 0).
    pub(crate) club: usize,
    /// ebp: the club's finance record (`FinanceTable::get_club_finance`).
    pub(crate) finance: usize,
    /// [E+0x14]: the view flags (screen param 0x1b, forced to 1 = General Info as described in the header).
    pub(crate) view: u32,
    /// [E+0x30]: screen param 1.
    pub(crate) param1: usize,
    /// esi and [E+0x2dc] (the `top` argument's slot, rewritten): the y below the title, `top + 0x41`.
    pub(crate) y: i32,
    /// [E+0x2e0]: the `bottom` argument.
    pub(crate) bottom: i32,
    /// ebx: the view menu area (`Display::add_area` attached to the "View" button).
    pub(crate) menu_area: usize,
    /// [E+0x58..E+0x60]: the column widths 2, 3, 2, 5, 2, 1, 1, 1, 1 (General Info uses +2 and +0).
    pub(crate) columns: [u8; 9],
    /// [E+0x24]: view & 1 (0 at both hand-overs); [E+0x38]: view & 0x10.
    pub(crate) general: u32,
    pub(crate) stats: u32,
    /// [E+0x10]: the Cash object (int64 amount, low dword zeroed at entry) the money views use; the frame's second
    /// Cash at [E+0x2c] is not read by any view here. Their destructor (0x5a53c0) is the empty function, so leaving
    /// needs nothing.
    pub(crate) cash_a: [u32; 2],
    /// [E+0xac]: the title (the selected view's menu text); [E+0x1c8]: the badge path / staff name buffer;
    /// [E+0x174]: the 0x15 non-playing staff pointers.
    pub(crate) title: [u8; 0x100],
    pub(crate) path: [u8; 0x104],
    pub(crate) staff: [usize; 0x15],
}

/// The shared text buffer the club screens format into.
pub(crate) const STATS_VIEW_TEXT: usize = 0xde1f64;

/// "-" (shown when there is no such player).
pub(crate) const STATS_VIEW_DASH: usize = 0x98ba0c;

/// The button id of a person's name (opens the profile; the person goes as the button value).
pub(crate) const STATS_VIEW_PERSON_BUTTON: usize = 0x7d7;

/// The display-currency options in the Config's +4 flags: wages per week, per month (neither: per year).
pub(crate) const STATS_VIEW_CONFIG_FLAGS: usize = 4;

pub(crate) const STATS_VIEW_WAGES_PER_WEEK: u32 = 0x200000;

pub(crate) const STATS_VIEW_WAGES_PER_MONTH: u32 = 0x400000;

/// club_screen_collect_squad_stats (0x487840, club_screens.cpp): cdecl(club, reserves, out) ret 0.
pub(crate) const STATS_VIEW_FN_COLLECT_SQUAD_STATS: usize = 0x487840;

/// database_club_nation_is_eec (0x53d7d0, Database.cpp): cdecl(club) ret 0, 1 when the club's nation is an EEC member.
pub(crate) const STATS_VIEW_FN_CLUB_NATION_IS_EEC: usize = 0x53d7d0;

/// The shared text buffer and its size as the money cells pass it.
pub(crate) const MONEY_VIEW_TEXT: usize = 0xde1f64;

pub(crate) const MONEY_VIEW_TEXT_SIZE: u32 = 0x7d0;

/// The currency symbol text (0x43f440, a Cash method that ignores `this`, returns its length) [inference: the symbol
/// `cash_format_money` prints].
pub(crate) const MONEY_VIEW_CURRENCY_SYMBOL: usize = 0xad9ca8;

/// The green the info tab computes on its first call (`graphics_rgb_to_pixel(0, 0xc0, 0, 0)`, guard bit 0 of 0xadab32).
pub(crate) const MONEY_VIEW_GREEN: usize = 0xadab48;

/// `g_gui_colour_orange` (the column titles and the line under them).
pub(crate) const MONEY_VIEW_ORANGE: usize = 0xaebdd4;

/// The empty text passed to the "Balance" label's `%s`.
pub(crate) const MONEY_VIEW_EMPTY_TEXT: usize = 0xad9c64;

/// Cash::set from a display amount (0x43f4c0): thiscall(Cash*, i64) ret 8; the cash becomes ftol(amount / the display
/// factor at 0xad9cb8), the inverse of `Cash::get_display_amount`.
pub(crate) const MONEY_VIEW_FN_CASH_SET_DISPLAY: usize = 0x43f4c0;

/// The four 0x50-byte blocks of the club's `Finance` record, in column order: this month, last month (+0x20, no name
/// yet), this season, last season.
pub(crate) const MONEY_VIEW_BLOCKS: [usize; 4] = [finance::MONTH_PLAYERS_BOUGHT, 0x20, finance::SEASON_PLAYERS_BOUGHT, finance::LAST_SEASON_TOTALS];

/// Cash::set_from_amount (0x43f4c0, thiscall(Cash*, i64 amount) ret 8): the cash value becomes amount / the display
/// factor (fild/fdiv/_ftol).
pub(crate) const LEDGER_FN_CASH_SET_FROM_AMOUNT: usize = 0x43f4c0;

/// The colour word of the Total rows (label and the four values) [inference: a bright text colour].
pub(crate) const LEDGER_COLOUR_TOTAL: usize = 0xaebde8;

/// The empty string, the `%s` argument of the Interest, Other and Total labels.
pub(crate) const LEDGER_EMPTY_TEXT: usize = 0xad9c64;

pub(crate) const LEDGER_TEXT: usize = 0xde1f64;

/// The four money blocks of the club Finance record, in column order: this month, last month (Finance::get_last_month
/// 0x4671b0), this season (0x4671e0) and last season (0x4671d0); each 0x50 bytes with the same layout as the month block.
pub(crate) const LEDGER_BLOCKS: [usize; 4] = [finance::MONTH_PLAYERS_BOUGHT, 0x20, finance::SEASON_PLAYERS_BOUGHT, finance::LAST_SEASON_TOTALS];

/// The Income rows: (label text, month field, label has the `%s` comment argument).
pub(crate) const LEDGER_INCOME_ROWS: [(usize, usize, bool); 9] = [
    (0x98faa8, finance::MONTH_GATE_RECEIPTS, false),   // "  Gate Receipts"
    (0x98fa94, finance::MONTH_SEASON_TICKETS, false),  // "  Season Tickets"
    (0x98fa84, finance::MONTH_TV_REVENUE, false),      // "  TV Revenue"
    (0x98fa74, finance::MONTH_MERCHANDISING, false),   // "  Merchandising"
    (0x98fa64, finance::MONTH_PLAYERS_SOLD, false),    // "  Players Sold" (low dword of the i64)
    (0x98fa54, finance::MONTH_PRIZE_MONEY, false),     // "  Prize Money"
    (0x98fa44, finance::MONTH_OTHER_INCOME, false),    // "  Investments"
    (0x98fa24, LEDGER_MONTH_INTEREST_RECEIVED, true),  // "  Interest<%s - COMMENT - cash>"
    (0x98fa04, LEDGER_MONTH_OTHER_INCOME_ROW, true),   // "  Other<%s - COMMENT - income>"
];

/// The Expenditure rows, same layout.
pub(crate) const LEDGER_EXPENDITURE_ROWS: [(usize, usize, bool); 9] = [
    (0x98f9dc, finance::MONTH_WAGES, false),           // "  Wages"
    (0x98f9d0, finance::MONTH_BONUSES, false),         // "  Bonuses"
    (0x98f9bc, finance::MONTH_SIGN_ON_FEES, false),    // "  Signing On Fees"
    (0x98f9b0, finance::MONTH_DIVIDENDS, false),       // "  Dividends"
    (0x98f99c, finance::MONTH_PLAYERS_BOUGHT, false),  // "  Players Bought" (low dword of the i64)
    (0x98f984, finance::MONTH_GROUND_COSTS, false),    // "  Ground Maintenance"
    (0x98f974, finance::MONTH_FINES, false),           // "  League Fines"
    (0x98fa24, LEDGER_MONTH_INTEREST_PAID, true),      // "  Interest<%s - COMMENT - cash>"
    (0x98f950, finance::MONTH_OTHER_COSTS, true),      // "  Other<%s - COMMENT - expenditure>"
];

/// The last month's block of a club's finance record (0x50 bytes at +0x20, before the current month's at
/// `finance::MONTH_PLAYERS_BOUGHT` +0x70).
pub(crate) const FINANCE_LAST_MONTH_BLOCK: usize = 0x20;

/// The club history screen's flag the builder raises on every build (0x98ded8; read elsewhere) [inference].
pub(crate) const CLUB_HISTORY_BUILT_FLAG: usize = 0x98ded8;

/// The club_screens.cpp source path the assert boxes print.
pub(crate) const CLUB_SCREENS_SOURCE: usize = 0x98e598;

/// The shared text buffer the screens format into (0xde1f64).
pub(crate) const HISTORY_TEXT: usize = 0xde1f64;

/// The shared text buffer the screens format into (0xde1f64).
pub(crate) const HISTORY_TAB_TEXT: usize = 0xde1f64;

/// How a season tab picks the period of the records it lists.
#[derive(Clone, Copy)]
pub(crate) enum HistoryPeriod {
    /// the records tab's period (param 2)
    Param,
    /// a fixed period
    Fixed(u8),
}

/// What a season tab does when the stored record type does not apply to the club in the period.
#[derive(Clone, Copy)]
pub(crate) enum HistoryDefault {
    /// nothing
    Keep,
    /// this type
    Type(u8),
    /// the first type that applies (0xff until then)
    FirstThatApplies,
}

/// The club_screens.cpp source path the error boxes print.
pub(crate) const CLUB_SCREENS_CPP_FILE: usize = 0x98e598;

/// The past meetings caches: one 42-byte record per human manager (by staff id minus the staff count) and the one
/// used when the slot has no manager [inference].
pub(crate) const PAST_MEETINGS_CACHES: usize = 0xadadf0;

pub(crate) const PAST_MEETINGS_DEFAULT_CACHE: usize = 0xadab50;

/// Colour words of the screens that have no name in `globals.rs`.
pub(crate) const SCREEN_COLOUR_BDDE: usize = 0xaebdde;

pub(crate) const SCREEN_COLOUR_3154: usize = 0xae3154;

/// The text the friendly dates rows end with (0x98f858).
pub(crate) const FRIENDLY_DATES_ROW_END: usize = 0x98f858;

/// club_screens.cpp's source path (0x98e598), for the handler's error boxes (`show_error_at`).
pub(crate) const SQUAD_NUMBERS_SOURCE_FILE: usize = 0x98e598;

/// The 50 player slots of the squad copy kept in parameter 0x11 of the squad numbers screen.
pub(crate) const SQUAD_NUMBERS_SLOTS: usize = 0x32;

/// The parameters 1..9 the part exchange screen opens with: 1, 2 and 3 empty, the view 0x41 (Traditional, physical
/// attributes), the sort 8 (positions), the statistics category 6, the two sort columns -1 and the filter 0x154e21.
pub(crate) const PART_EXCHANGE_OPEN_PARAMS: [u32; 9] = [0, 0, 0, 0x41, 8, 6, u32::MAX, u32::MAX, 0x154e21];

/// The part exchange screen's state: what the original keeps in registers and stack slots across its pieces.
pub(crate) struct PartExchange {
    /// param 0: the transfer bid (+8 the bidding club's id, +0xc the player's id, +4 the list of the players already
    /// offered in exchange)
    pub(crate) bid: usize,
    /// param 3: nonzero while choosing (the names link to 2 instead of the person) [inference]
    pub(crate) choosing: u32,
    /// param 4: the view flags; in the attributes view the slot then holds the attribute table, as the original
    pub(crate) view: u32,
    /// param 4 as read (the tests of the rows; the original keeps each one in its own stack slot)
    pub(crate) view_flags: u32,
    /// param 5: the sort flags
    pub(crate) sort: u32,
    /// params 6, 7 and 8 (bytes): the statistics category and two list options
    pub(crate) category: u8,
    pub(crate) option7: u8,
    pub(crate) option8: u8,
    /// param 9: the filter flags; its low byte later holds the row count (the original reuses the slot)
    pub(crate) filter: u32,
    /// the bidding club (its players are listed) and the bid's player and his club (the seller)
    pub(crate) club: usize,
    pub(crate) player: usize,
    pub(crate) seller: usize,
    /// the slot's human manager
    pub(crate) human: usize,
    /// the bidding club's kit colours (the original's words in dword slots)
    pub(crate) kit_back: u32,
    pub(crate) kit_text: u32,
    pub(crate) title: [u8; 0xc8],
    pub(crate) name: [u8; 0x17c],
    /// the view bar (the rows go there when there is no view of its own)
    pub(crate) bar: u32,
    /// players of the club the selling club does not want / whose wage demand is too high / rows shown
    pub(crate) not_interested: u8,
    pub(crate) too_dear: u8,
    pub(crate) shown: u8,
    /// the first dword of the club's filtered squad list (its count in the low word)
    pub(crate) squad_head: u32,
    /// the offer and negotiation lists of a player, the club's filtered squad, the candidates
    pub(crate) offers: [u8; 0x20],
    pub(crate) negotiations: [u8; 0x20],
    pub(crate) squad: [u8; 0x20],
    pub(crate) candidates: [u8; 0x20],
    /// the local contract used to ask for the wage (STAFF_CONTRACT, 0x50 bytes)
    pub(crate) contract: [u32; 0x14],
}

/// The column weights of the views (13 columns; the positions view's 8).
pub(crate) struct PartExchangeWeights {
    pub(crate) two: [u8; 8],
    pub(crate) contract: [u8; 13],
    pub(crate) stats: [u8; 13],
    pub(crate) attributes: [u8; 13],
    pub(crate) other: [u8; 13],
    pub(crate) selection: [u8; 13],
}

impl PartExchange {
    pub(crate) unsafe fn display(&self) -> usize {
        screens_current_display(G_SCREEN_MANAGER)
    }
    pub(crate) unsafe fn add_area(&self, f: [u32; 11]) -> u32 {
        tel26_add_area(self.display(), f)
    }
    /// A column heading button in row 0 of `area` (the label already at 0xde1f64); returns eax.
    pub(crate) unsafe fn heading(&self, area: u32, col: u32, font: u32, link: u32, data: u32) -> u32 {
        tel26_area_button([area, col, 0, 0x30, part_exchange_colour(0xae317e), font, 1, part_exchange_colour(0xae3184), 0xde1f64, link, data])
    }
}

/// What the row helpers need, and the differences between the part exchange screen and the squad numbers screen (the
/// latter, `squad_screen`, has national team links and its own value and contract links).
#[derive(Clone, Copy)]
pub(crate) struct SquadListRules {
    pub(crate) club: usize,
    pub(crate) sort: u32,
    pub(crate) view_flags: u32,
    pub(crate) category: u8,
    /// the attribute table of the attributes view (the reused view or row-count slot, as the original)
    pub(crate) table: usize,
    pub(crate) squad_screen: bool,
}

/// The squad numbers screen's state: what the original keeps in registers and stack slots across its pieces.
pub(crate) struct SquadNumbersScreen {
    pub(crate) club: usize,
    /// param 2: 1 the squad numbers, otherwise the European squad
    pub(crate) mode: u8,
    pub(crate) view: u32,
    pub(crate) sort: u32,
    pub(crate) category: u8,
    pub(crate) option9: u8,
    /// param 0xa (byte), then the row count, then the attribute table (the original reuses the slot)
    pub(crate) slot_a: usize,
    /// params 0xb..0xf (bytes): the European rule byte, the squad size asked, and the registered counts
    pub(crate) rule_b: u8,
    pub(crate) squad_size: u8,
    pub(crate) rule_d: u8,
    pub(crate) registered: u8,
    pub(crate) reserves: u8,
    /// param 0x11: the squad copy (50 player slots); its slot later holds the Traditional view's row count
    pub(crate) squad: usize,
    pub(crate) fixture: usize,
    /// the club's side of the fixture (1 when its side byte is 0)
    pub(crate) side: u32,
    pub(crate) title: [u8; 0xc8],
    pub(crate) list: [u8; 0x20],
    /// the last area made (ebp in the original: the rows go there when there is no view of its own)
    pub(crate) last_area: u32,
    pub(crate) top: i32,
    pub(crate) bottom: i32,
}

/// The sort context the squad list comparators read (0xad9db0 the club, 0xad9db4/0xad9db5 two option bytes).
pub(crate) const SQUAD_LIST_SORT_CLUB: usize = 0xad9db0;

/// The rest of the sort context (0xadab33 the name style byte, 0xadab34 the reserves flag, 0xadab38 the team sheet,
/// 0xadab3c the club or its linked side, 0xadab44 the fixture, 0xadab4a an option byte).
pub(crate) const SQUAD_LIST_SORT_CONTEXT: usize = 0xadab33;

/// The manager of the current screen slot (screen manager +0x7d6 of the slot), kept for the comparators (0xadadf0).
pub(crate) const SQUAD_LIST_SORT_MANAGER: usize = 0xadadf0;

/// A team sheet: two TEAM blocks of 0x625 bytes (the club whose id is at +0 first), each with 20 players of 73 bytes
/// from +0x190 (the player id at +0).
pub(crate) const TEAM_SHEET_BLOCK: usize = 0x625;

pub(crate) const TEAM_SHEET_PLAYERS: usize = 0x190;

pub(crate) const TEAM_SHEET_PLAYER_SIZE: usize = 73;

/// The groups of the part exchange screen's parameter 4 (the view) and parameter 9 (the columns).
pub(crate) const PART_EXCHANGE_VIEW_GROUPS: [(u32, &[u32]); 2] = [(0xffffefc0, &[1, 2, 4, 8, 0x10, 0x20, 0x1000]), (0xfffff83f, &[0x40, 0x80, 0x100, 0x200, 0x400])];

pub(crate) const PART_EXCHANGE_COLUMN_GROUPS: [(u32, &[u32]); 8] = [
    (0xffffffe0, &[1, 2, 4, 8, 0x10]), (0xfffffe1f, &[0x20, 0x40, 0x80, 0x100]), (0xfffff9ff, &[0x200, 0x400]),
    (0xffffc7ff, &[0x800, 0x1000, 0x2000]), (0xffff3fff, &[0x4000, 0x8000]), (0xfffcffff, &[0x10000, 0x20000]),
    (0xfff3ffff, &[0x40000, 0x80000]), (0xffcfffff, &[0x100000, 0x200000]),
];

/// The sort context of the squad list comparators, written by `build_filtered_sorted_squad_list` (0x4841d0) before
/// its qsort: the club whose list it is.
pub(crate) const PLAYER_SORT_CLUB: usize = 0xad9db0;

/// u8: the attribute of the "attribute" column.
pub(crate) const PLAYER_SORT_ATTRIBUTE: usize = 0xad9db4;

/// u8: the field of the "statistic" column (`PlayerStats::get_stat`).
pub(crate) const PLAYER_SORT_STAT_FIELD: usize = 0xad9db5;

/// u8: the name style passed to `database_format_staff_name`.
pub(crate) const PLAYER_SORT_NAME_STYLE: usize = 0xadab33;

/// u32: the team index passed to `Tactics::find_selection_slot` (1 = reserves).
pub(crate) const PLAYER_SORT_RESERVES: usize = 0xadab34;

/// usize: the team sheet of a club the human does not control (0 = none).
pub(crate) const PLAYER_SORT_TEAM_SHEET: usize = 0xadab38;

/// usize: the club or its linked side, whose contracts and squad numbers the columns show.
pub(crate) const PLAYER_SORT_SIDE: usize = 0xadab3c;

/// usize: the fixture of the status column.
pub(crate) const PLAYER_SORT_FIXTURE: usize = 0xadab44;

/// u8: the category of the "statistic" column (`PlayerStats::get_stat`).
pub(crate) const PLAYER_SORT_STAT_CATEGORY: usize = 0xadab4a;

/// usize: the viewer whose knowledge hides attributes (`staff_is_player_attribute_visible`).
pub(crate) const PLAYER_SORT_VIEWER: usize = 0xadadf0;

/// `G_SQUAD_MANAGER` (0xdd7ef0), the squad number owner.
pub(crate) const PLAYER_SORT_SQUAD_MANAGER: usize = 0xdd7ef0;

/// The last match's team sheet record (two 0x625-byte team blocks from +0x11f) and the club history states (16 of
/// 0x2a bytes), globals of club_screens.cpp built at the game start.
pub(crate) const CLUB_SCREENS_MATCH_RECORD: usize = 0xad9db8;

pub(crate) const CLUB_SCREENS_HISTORY_STATES: usize = 0xadab50;

/// The club screen's build, input and leave functions (passed to `ScreenManager::open_screen`).
pub(crate) const FN_CLUB_SCREEN_BUILD: usize = 0x454900;

pub(crate) const FN_CLUB_SCREEN_INPUT: usize = 0x4671f0;

pub(crate) const FN_CLUB_SCREEN_LEAVE: usize = 0x469da0;

/// The default squad view settings of a club the human does not control: the dwords 0x41 (the filter, parameter 7),
/// 8 (parameter 8) and 0x154a21 (parameter 0x18), then the bytes 6 (parameter 9), -1 (0x19) and -1 (0x1a).
pub(crate) const CLUB_SQUAD_DEFAULT_VIEW: [u8; 15] = [0x41, 0, 0, 0, 8, 0, 0, 0, 0x21, 0x4a, 0x15, 0, 6, 0xff, 0xff];

/// Month-block fields with no name in fields.rs yet, named from the screen's labels [inference].
pub(crate) const LEDGER_MONTH_INTEREST_PAID: usize = 0x84; // "Interest" (expenditure)

pub(crate) const LEDGER_MONTH_OTHER_INCOME_ROW: usize = 0xa4; // "Other" (income)

pub(crate) const LEDGER_MONTH_INTEREST_RECEIVED: usize = 0xb0; // "Interest" (income)

/// Answers a click on the club screen (param 0 the club, 1 the side: reserves when set, 2 the club's last match, 4 the
/// tab, 5 and 6 the club's colours). First the answer of the Save Printout box (`club_screen_printout_answer`); then
/// the buttons of every tab (`club_screen_common_actions`); then the open tab: 1 squad
/// (`club_screen_squad_tab_input`), 2 transfers (`club_screen_transfers_tab_input`), 3 next match
/// (`club_screen_next_match_tab_input`) and 4 fixtures (`club_screen_fixtures_tab_input`) when they answer nonzero, 5
/// info (`club_screen_info_tab_input`), any other tab the club_screens.cpp "Error" box (line 8053); last, the club
/// screen actions (`handle_club_screen_action`), the menu bar commands and the object links, in that order. The
/// original builds two TEAM blocks (0x625 bytes each) on its stack on entry and destroys them on every exit without
/// using them; kept here.
/// Original: `club_screen_input` (0x4671f0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screen_input(widget: u32) -> i32 {
    let mut sides = [0u8; 2 * 0x625];
    let s = sides.as_mut_ptr() as usize;
    exe::crt_vector_constructor_iterator(s, 0x625, 2, 0x453c30usize, 0x453c10usize);
    let r = club_screen_answer(widget);
    exe::crt_vector_destructor_iterator(s, 0x625, 2, 0x453c10usize);
    r
}

/// Comparer for `club_screen_input`: the clicks that only write params (`club_screen_click_stays`) run side by side
/// (`compare_resposta`, the params and the answer); everything else (screens and boxes opened, printing, the line-up
/// and squad number drags, the next match and fixtures tabs, the menu bar) runs the original only, and its answer is
/// checked against the planned one where it does not depend on the game (`club_screen_planned_answer`).
pub(crate) unsafe extern "C" fn compare_club_screen_input(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_CLUB_SCREEN_INPUT);
    if club_screen_click_stays(widget) {
        return compare_resposta("club screen input", widget, o, club_screen_input, false);
    }
    let action = club_tabs_action(widget as i16);
    let plan = club_screen_planned_answer(widget);
    let r = o(widget);
    if let Some(p) = plan {
        SCOREBOARD.record(r == p, || format!("club screen input (widget {:#x}, action {:#x}): answer {} planned {}", widget, action, r, p));
    }
    r
}
