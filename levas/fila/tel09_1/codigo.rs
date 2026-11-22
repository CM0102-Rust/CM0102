// ---------- batch %%LEVA%%: the contract and board accessors of the staff profile's Transfer tab (staff_screens.cpp) ----------

/// The contract's morale (`contract::MORALE`, signed byte).
///
/// An out-of-line accessor of staff_screens.cpp, called by the profile's Transfer tab (0x87dd80) only.
/// Original: `staff_screens_contract_morale` (0x8857b0, staff_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn staff_screens_contract_morale(this: usize) -> u8 {
    Contract(this).morale() as u8
}

/// The matches the contract's person was left out (`contract::MATCHES_LEFT_OUT`).
///
/// An out-of-line accessor of staff_screens.cpp, called by the profile's Transfer tab (0x87dd80) only.
/// Original: `staff_screens_contract_matches_left_out` (0x8857c0, staff_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn staff_screens_contract_matches_left_out(this: usize) -> u8 {
    Contract(this).matches_left_out()
}

/// The contract's squad status: the high nibble of `contract::SQUAD_STATUS`.
///
/// An out-of-line accessor of staff_screens.cpp, called by the profile's Transfer tab (0x87dd80) only.
/// Original: `staff_screens_contract_squad_status` (0x8857d0, staff_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn staff_screens_contract_squad_status(this: usize) -> u8 {
    Contract(this).squad_status() >> 4
}

/// Whether the contract's person leaves on a Bosman (`contract::LEAVING_ON_BOSMAN`).
///
/// An out-of-line accessor of staff_screens.cpp, called by the profile's Transfer tab (0x87dd80) only.
/// Original: `staff_screens_contract_leaving_on_bosman` (0x8857e0, staff_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn staff_screens_contract_leaving_on_bosman(this: usize) -> u8 {
    Contract(this).leaving_on_bosman()
}

/// The transfer request bit of `contract::TRANSFER_STATUS` (`transfer_status::TRANSFER_REQUESTED`, 8 or 0, in the
/// whole eax).
///
/// An out-of-line accessor of staff_screens.cpp, called by the profile's Transfer tab (0x87dd80) only.
/// Original: `staff_screens_contract_transfer_requested` (0x8857f0, staff_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn staff_screens_contract_transfer_requested(this: usize) -> u32 {
    (Contract(this).transfer_status() & transfer_status::TRANSFER_REQUESTED) as u32
}

/// The board's confidence in the manager of `club` (`board_record::CONFIDENCE` of the club's 0x49-byte record in the
/// MANAGER_MANAGER board table); 0 without a club.
///
/// An out-of-line accessor of staff_screens.cpp, called by the profile's Transfer tab (0x87dd80) only.
/// Original: `staff_screens_board_confidence` (0x885800, staff_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn staff_screens_board_confidence(this: usize, club: usize) -> u16 {
    if club == 0 {
        covered!("tel09_1 board confidence without club");
        return 0;
    }
    read_u16(read_usize(this).wrapping_add((read_i32(club) as usize).wrapping_mul(0x49)) + board_record::CONFIDENCE)
}

compare_value!(compare_staff_screens_contract_morale, staff_screens_contract_morale, %%I0%%, () -> u8);
compare_value!(compare_staff_screens_contract_matches_left_out, staff_screens_contract_matches_left_out, %%I1%%, () -> u8);
compare_value!(compare_staff_screens_contract_squad_status, staff_screens_contract_squad_status, %%I2%%, () -> u8);
compare_value!(compare_staff_screens_contract_leaving_on_bosman, staff_screens_contract_leaving_on_bosman, %%I3%%, () -> u8);
compare_value!(compare_staff_screens_contract_transfer_requested, staff_screens_contract_transfer_requested, %%I4%%, () -> u32);
compare_value!(compare_staff_screens_board_confidence, staff_screens_board_confidence, %%I5%%, (club: usize) -> u16);
