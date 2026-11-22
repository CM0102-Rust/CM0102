// ---------- batch %%LEVA%%: the small accessors of club_screens.cpp (cash, finance blocks, widgets, area scroll) ----------

/// The last month's block of a club's finance record (0x50 bytes at +0x20, before the current month's at
/// `finance::MONTH_PLAYERS_BOUGHT` +0x70).
const FINANCE_LAST_MONTH_BLOCK: usize = 0x20;

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

/// The Display of the screen manager's current slot (`screen_manager::DISPLAY` of slot `screen_manager::CURRENT_SLOT`).
unsafe fn widget_current_display(manager: usize) -> usize {
    let slot = read_i16(manager + screen_manager::CURRENT_SLOT) as isize;
    read_usize((manager as isize + slot * 0x300 + screen_manager::DISPLAY as isize) as usize)
}

/// The address of item `widget` (0x18c bytes each) plus `field` in a Display.
fn widget_field(display: usize, widget: i16, field: usize) -> usize {
    (display as isize + widget as isize * 0x18c + field as isize) as usize
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(%%I0%%);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 4)], || o(this, amount), || cash_set(this, amount));
    SCOREBOARD.record(igual, || format!("cash_set({:#x}, {}): {:?} x {:?}", this, amount, mo, mr));
}

compare_value!(compare_finance_get_last_month, finance_get_last_month, %%I1%%, () -> usize);
compare_value!(compare_finance_get_this_month, finance_get_this_month, %%I2%%, () -> usize);
compare_value!(compare_finance_get_last_year, finance_get_last_year, %%I3%%, () -> usize);
compare_value!(compare_finance_get_this_year, finance_get_this_year, %%I4%%, () -> usize);
compare_value!(compare_display_get_widget_id, display_get_widget_id, %%I5%%, (widget: u32) -> u32);
compare_value!(compare_screenmanager_get_widget_action_id, screenmanager_get_widget_action_id, %%I6%%, (widget: u32) -> i32);
compare_value!(compare_screenmanager_get_widget_action_data, screenmanager_get_widget_action_data, %%I7%%, (widget: u32) -> u32);
compare_value!(compare_screenmanager_get_widget_drop_target, screenmanager_get_widget_drop_target, %%I8%%, (widget: u32) -> u32);

/// Comparer for `display_set_area_scroll_row`: side by side over the area's scroll row (`Area` +0x1fc).
pub(crate) unsafe extern "thiscall" fn compare_display_set_area_scroll_row(this: usize, area: u32, row: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I9%%);
    if area as i16 == -1 {
        return o(this, area, row);
    }
    let scroll = (this as isize + (area as i16) as isize * 0xbf1 + 4 + 0x1fc) as usize;
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(scroll, 4)], || o(this, area, row), || display_set_area_scroll_row(this, area, row));
    SCOREBOARD.record(igual, || format!("display_set_area_scroll_row(area {}, row {}): {:?} x {:?}", area as i16, row, mo, mr));
}
