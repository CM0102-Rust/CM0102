// ---------- batch %%LEVA%%: the human manager's screen state: tactics, commentary speed, offers, password and squad view (human_manager.cpp; sx12_4) ----------

/// The record of a human manager (null: the current human, `manager_or_current_human`), or `None` after the original's
/// "Error" box: `missing` (human_manager.cpp line) when there is no manager at all, `out_of_range` when his index
/// (`human_manager_human_manager_index`) falls outside the table.
pub(crate) unsafe fn human_record_or_error(this: usize, manager: usize, missing: u32, out_of_range: u32) -> Option<usize> {
    let m = manager_or_current_human(manager);
    if m == 0 {
        show_error_at(0x9cf11c, missing); // "...\human_manager.cpp"
        return None;
    }
    match human_manager_human_manager_index(this, m) {
        Some(k) => Some(human_manager_record_at(this, k)),
        None => {
            show_error_at(0x9cf11c, out_of_range);
            None
        }
    }
}

/// Whether `human_record_or_error` would find the record (no "Error" box): the comparers run only the original
/// otherwise.
pub(crate) unsafe fn human_record_found(this: usize, manager: usize) -> bool {
    let m = manager_or_current_human(manager);
    m != 0 && human_manager_human_manager_index(this, m).is_some()
}

/// The address of a byte of the human manager's active tactic slot (0x1ecd bytes each, chosen by
/// `human_manager_record::ACTIVE_TACTIC_SLOT`, signed), at `offset` in the slot; 0 after the "Error" box.
unsafe fn active_tactic_slot_byte(this: usize, manager: usize, offset: usize, rows: (u32, u32)) -> usize {
    let Some(record) = human_record_or_error(this, manager, rows.0, rows.1) else {
        return 0;
    };
    let slot = read_i8(record + human_manager_record::ACTIVE_TACTIC_SLOT) as isize;
    record.wrapping_add((slot * 0x1ecd) as usize) + offset
}

/// Returns the address of the stored zone selection of the active tactic slot (+0x18e3 of the slot), which the tactics
/// screen saves and restores; no manager or out of range: the "Error" box (lines 0x18a, 0x196) and 0.
/// Original: `HUMAN_MANAGER::get_tactic_zone_selection_ptr` (0x5fc010, human_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_tactic_zone_selection_ptr(this: usize, manager: usize) -> usize {
    covered!("sx12_4 zone selection");
    active_tactic_slot_byte(this, manager, 0x18e3, (0x18a, 0x196)) // sx12_4: the zone selection
}

/// Returns the address of the previous zone selection of the active tactic slot (+0x18e7 of the slot); no manager or
/// out of range: the "Error" box (lines 0x1b0, 0x1bc) and 0.
/// Original: `HUMAN_MANAGER::get_tactic_prev_zone_selection_ptr` (0x5fc130, human_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_tactic_prev_zone_selection_ptr(this: usize, manager: usize) -> usize {
    active_tactic_slot_byte(this, manager, 0x18e7, (0x1b0, 0x1bc))
}

/// Returns the address of the tactics screen view mode byte of the active slot (+0x18eb: Overview, With Ball or Without
/// Ball); no manager or out of range: the "Error" box (lines 0x1d6, 0x1e2) and 0.
/// Original: `HUMAN_MANAGER::get_tactic_view_mode_ptr` (0x5fc250, human_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_tactic_view_mode_ptr(this: usize, manager: usize) -> usize {
    covered!("sx12_4 view mode");
    active_tactic_slot_byte(this, manager, 0x18eb, (0x1d6, 0x1e2)) // sx12_4: the view mode
}

/// Returns the address of the previous view mode byte of the active slot (+0x18ec); no manager or out of range: the
/// "Error" box (lines 0x1fc, 0x208) and 0.
/// Original: `HUMAN_MANAGER::get_tactic_prev_view_mode_ptr` (0x5fc370, human_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_tactic_prev_view_mode_ptr(this: usize, manager: usize) -> usize {
    active_tactic_slot_byte(this, manager, 0x18ec, (0x1fc, 0x208))
}

/// Returns the address of the Show Runs flag of the active tactic slot (+0x18ed); no manager or out of range: the
/// "Error" box (lines 0x222, 0x22e) and 0.
/// Original: `HUMAN_MANAGER::get_tactic_show_runs_ptr` (0x5fc490, human_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_tactic_show_runs_ptr(this: usize, manager: usize) -> usize {
    active_tactic_slot_byte(this, manager, 0x18ed, (0x222, 0x22e))
}

/// Returns the human manager's match commentary speed (the word at +0x9ad2 of his record; only ax is the return
/// value); no manager or out of range: the "Error" box (lines 0x39b, 0x3a7) and 0.
/// Original: `HUMAN_MANAGER::get_commentary_speed` (0x5fcff0, human_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_commentary_speed(this: usize, manager: usize) -> u16 {
    let Some(record) = human_record_or_error(this, manager, 0x39b, 0x3a7) else {
        return 0;
    };
    covered!("sx12_4 commentary speed");
    read_u16(record + 0x9ad2) // sx12_4: the commentary speed word
}

/// Stores the human manager's match commentary speed (the word at +0x9ad2 of his record); no manager or out of range:
/// the "Error" box (lines 0x3c1, 0x3cd) and nothing.
/// Original: `HUMAN_MANAGER::set_commentary_speed` (0x5fd100, human_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn set_commentary_speed(this: usize, speed: u16, manager: usize) {
    if let Some(record) = human_record_or_error(this, manager, 0x3c1, 0x3cd) {
        covered!("sx12_4 set commentary speed");
        write_u16(record + 0x9ad2, speed); // sx12_4: the commentary speed word
    }
}

/// Returns the address of the manager's draft contract offer block (+0x9a09, a STAFF_CONTRACT) used by the offer contract screens; no manager or out of range: the "Error" box (lines 0x50d, 0x519) and 0.
/// Original: `HUMAN_MANAGER::get_contract_offer` (0x5fd930, human_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_contract_offer(this: usize, manager: usize) -> usize {
    let Some(record) = human_record_or_error(this, manager, 0x50d, 0x519) else {
        return 0;
    };
    covered!("sx12_4 contract offer");
    record + 0x9a09 // sx12_4: the contract offer
}

/// Returns the address of the manager's pending transfer bid offer pointer (+0x9a59) used by the bid screen; no manager or out of range: the "Error" box (lines 0x533, 0x53f) and 0.
/// Original: `HUMAN_MANAGER::get_bid_offer_slot` (0x5fda30, human_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_bid_offer_slot(this: usize, manager: usize) -> usize {
    let Some(record) = human_record_or_error(this, manager, 0x533, 0x53f) else {
        return 0;
    };
    record + 0x9a59
}

/// Returns the address of the manager's transfer offer pointer (+0x9a5d) used by the confirm transfer screen; no manager or out of range: the "Error" box (lines 0x559, 0x565) and 0.
/// Original: `HUMAN_MANAGER::get_confirm_offer_slot` (0x5fdb30, human_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_confirm_offer_slot(this: usize, manager: usize) -> usize {
    let Some(record) = human_record_or_error(this, manager, 0x559, 0x565) else {
        return 0;
    };
    record + 0x9a5d
}

/// Returns the address of the manager's password string (+0x9a6a), which the setup name screens and the file password checks use; no manager or out of range: the "Error" box (lines 0x5cf, 0x5db) and 0.
/// Original: `HUMAN_MANAGER::get_password` (0x5fde40, human_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_password(this: usize, manager: usize) -> usize {
    let Some(record) = human_record_or_error(this, manager, 0x5cf, 0x5db) else {
        return 0;
    };
    covered!("sx12_4 password");
    record + 0x9a6a // sx12_4: the password
}

/// Returns the address of the manager's 50-byte typed password buffer (+0x9a9c), which the password boxes compare with the stored password; no manager or out of range: the "Error" box (lines 0x5f5, 0x601) and 0.
/// Original: `HUMAN_MANAGER::get_password_entry` (0x5fdf40, human_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_password_entry(this: usize, manager: usize) -> usize {
    let Some(record) = human_record_or_error(this, manager, 0x5f5, 0x601) else {
        return 0;
    };
    record + 0x9a9c
}

/// Returns the address of the 15-byte block (+0xa6d4) where the manager keeps the squad screen view settings between visits; no manager or out of range: the "Error" box (lines 0x76a, 0x776) and 0.
/// Original: `HUMAN_MANAGER::get_squad_view_settings` (0x5fe950, human_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_squad_view_settings(this: usize, manager: usize) -> usize {
    let Some(record) = human_record_or_error(this, manager, 0x76a, 0x776) else {
        return 0;
    };
    covered!("sx12_4 squad view");
    record + 0xa6d4 // sx12_4: the squad view
}

/// Comparer for the human manager getters: when the record is found, the return side by side; otherwise (the "Error"
/// box) only the original runs.
pub(crate) unsafe fn compare_human_getter<R: PartialEq + core::fmt::LowerHex + Copy>(name: &str, this: usize, manager: usize,
    original: impl FnOnce() -> R, copy: impl FnOnce() -> R) -> R {
    if !human_record_found(this, manager) {
        return original();
    }
    let r = original();
    let n = copy();
    SCOREBOARD.record(r == n, || format!("HUMAN_MANAGER::{}(manager {:#x}) original {:#x} rust {:#x}", name, manager, r, n));
    r
}

/// Comparer for `get_tactic_zone_selection_ptr`.
pub(crate) unsafe extern "thiscall" fn compare_get_tactic_zone_selection_ptr(this: usize, manager: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%);
    compare_human_getter("get_tactic_zone_selection_ptr", this, manager, || o(this, manager), || get_tactic_zone_selection_ptr(this, manager))
}

/// Comparer for `get_tactic_prev_zone_selection_ptr`.
pub(crate) unsafe extern "thiscall" fn compare_get_tactic_prev_zone_selection_ptr(this: usize, manager: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%);
    compare_human_getter("get_tactic_prev_zone_selection_ptr", this, manager, || o(this, manager), || get_tactic_prev_zone_selection_ptr(this, manager))
}

/// Comparer for `get_tactic_view_mode_ptr`.
pub(crate) unsafe extern "thiscall" fn compare_get_tactic_view_mode_ptr(this: usize, manager: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%);
    compare_human_getter("get_tactic_view_mode_ptr", this, manager, || o(this, manager), || get_tactic_view_mode_ptr(this, manager))
}

/// Comparer for `get_tactic_prev_view_mode_ptr`.
pub(crate) unsafe extern "thiscall" fn compare_get_tactic_prev_view_mode_ptr(this: usize, manager: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I3%%);
    compare_human_getter("get_tactic_prev_view_mode_ptr", this, manager, || o(this, manager), || get_tactic_prev_view_mode_ptr(this, manager))
}

/// Comparer for `get_tactic_show_runs_ptr`.
pub(crate) unsafe extern "thiscall" fn compare_get_tactic_show_runs_ptr(this: usize, manager: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I4%%);
    compare_human_getter("get_tactic_show_runs_ptr", this, manager, || o(this, manager), || get_tactic_show_runs_ptr(this, manager))
}

/// Comparer for `get_commentary_speed`: ax.
pub(crate) unsafe extern "thiscall" fn compare_get_commentary_speed(this: usize, manager: usize) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u16>(%%I5%%);
    compare_human_getter("get_commentary_speed", this, manager, || o(this, manager), || get_commentary_speed(this, manager))
}

/// Comparer for `set_commentary_speed`: the word written, side by side; the "Error" box: only the original.
pub(crate) unsafe extern "thiscall" fn compare_set_commentary_speed(this: usize, speed: u16, manager: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u16, usize)>(%%I6%%);
    if !human_record_found(this, manager) {
        return o(this, speed, manager);
    }
    let k = human_manager_human_manager_index(this, manager_or_current_human(manager)).unwrap_or(0);
    let at = human_manager_record_at(this, k) + 0x9ad2;
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(at, 2)], || o(this, speed, manager), || set_commentary_speed(this, speed, manager));
    SCOREBOARD.record(equal, || format!("HUMAN_MANAGER::set_commentary_speed({}) {:x?} x {:x?}", speed, mo, mr));
}

/// Comparer for `get_contract_offer`.
pub(crate) unsafe extern "thiscall" fn compare_get_contract_offer(this: usize, manager: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I7%%);
    compare_human_getter("get_contract_offer", this, manager, || o(this, manager), || get_contract_offer(this, manager))
}

/// Comparer for `get_bid_offer_slot`.
pub(crate) unsafe extern "thiscall" fn compare_get_bid_offer_slot(this: usize, manager: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I8%%);
    compare_human_getter("get_bid_offer_slot", this, manager, || o(this, manager), || get_bid_offer_slot(this, manager))
}

/// Comparer for `get_confirm_offer_slot`.
pub(crate) unsafe extern "thiscall" fn compare_get_confirm_offer_slot(this: usize, manager: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I9%%);
    compare_human_getter("get_confirm_offer_slot", this, manager, || o(this, manager), || get_confirm_offer_slot(this, manager))
}

/// Comparer for `get_password`.
pub(crate) unsafe extern "thiscall" fn compare_get_password(this: usize, manager: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I10%%);
    compare_human_getter("get_password", this, manager, || o(this, manager), || get_password(this, manager))
}

/// Comparer for `get_password_entry`.
pub(crate) unsafe extern "thiscall" fn compare_get_password_entry(this: usize, manager: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I11%%);
    compare_human_getter("get_password_entry", this, manager, || o(this, manager), || get_password_entry(this, manager))
}

/// Comparer for `get_squad_view_settings`.
pub(crate) unsafe extern "thiscall" fn compare_get_squad_view_settings(this: usize, manager: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I12%%);
    compare_human_getter("get_squad_view_settings", this, manager, || o(this, manager), || get_squad_view_settings(this, manager))
}
