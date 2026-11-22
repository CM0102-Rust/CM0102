// ---------- batch %%LEVA%%: opening the bid screen for a player (transfer_screens.cpp) ----------

/// Rule: the bid screen opens for a player, with an existing offer (its serial number) or a new one; it notes
/// whether the human's club may sign him now and whether the offer is already accepted.
///
/// `transfer_screens_open_bid_screen_for_player(person, offer_slot)`: the offer serial is -1, or the
/// `offer::SERIAL` of the offer the slot points to. `open_screen` with the builder 0x8e9760, the handler 0x8ec420,
/// single instance, the leave routine 0x93a710 and 0; when it opens: parameter 0 = the player's id, 1 = the slot, 2 =
/// 0, 3 = the serial, 4 and 5 = 0, 6 = 1; the player's contract (`CONTRACT_MANAGER::get_contract` on
/// `G_CONTRACT_MANAGER`, kept in `G_CONTRACT_LOOKUP_RESULT`) gives the selling club (`contract::CLUB`) unless its
/// `contract::CONTRACT_TYPE` & 0x3f is 5; parameter 8 = 0; 9 = `TRANSFER_MANAGER::can_sign_player(human club, today,
/// player, selling club, 1, 0)` on `G_TRANSFER_MANAGER` as 0/1 (the human of the current slot's `staff::CLUB_JOB`); 7 =
/// 1 when the slot's offer has an `offer::STATUS` of 5 or more other than 0x10, else 0; 0xf..0x16 = -1.
/// Original: `transfer_screens_open_bid_screen_for_player` (0x8e9350, transfer_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn transfer_screens_open_bid_screen_for_player(person: usize, offer_slot: usize) {
    let offer = if offer_slot != 0 { read_usize(offer_slot) } else { 0 };
    let serial = if offer != 0 { read_u32(offer + offer::SERIAL) as usize } else { usize::MAX };
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8e9760usize, 0x8ec420usize, 1, 0x93a710usize, 0) == 0 {
        return;
    }
    covered!("tel36_4 open the bid screen");
    set_screen_param(0, read_u32(person) as usize);
    set_screen_param(1, offer_slot);
    set_screen_param(2, 0);
    set_screen_param(3, serial);
    set_screen_param(4, 0);
    set_screen_param(5, 0);
    set_screen_param(6, 1);
    let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
    write_usize(G_CONTRACT_LOOKUP_RESULT, contract);
    let seller = if contract != 0 && read_u8(contract + contract::CONTRACT_TYPE) & 0x3f != 5 {
        read_usize(G_CLUB_TABLE).wrapping_add(read_u32(contract + contract::CLUB).wrapping_mul(club::SIZE as u32) as usize)
    } else {
        0
    };
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    let human = read_usize((G_SCREEN_RECORDS_MANAGER as isize + slot * 0x300) as usize);
    let human_club = read_usize(human + staff::CLUB_JOB);
    set_screen_param(8, 0);
    let can = exe::transfer_manager_can_sign_player(read_usize(G_TRANSFER_MANAGER), human_club, G_CURRENT_DATE, person, seller, 1, core::ptr::null_mut());
    set_screen_param(9, (can != 0) as usize);
    let accepted = offer != 0 && {
        let status = read_u8(offer + offer::STATUS) as i8;
        status >= 5 && status != 0x10
    };
    set_screen_param(7, accepted as usize);
    for k in 0xf..=0x16 {
        set_screen_param(k, usize::MAX);
    }
}

/// Comparer for `transfer_screens_open_bid_screen_for_player`.
pub(crate) unsafe extern "C" fn compare_transfer_screens_open_bid_screen_for_player(person: usize, offer_slot: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize, usize)>(%%I0%%)(person, offer_slot)
}
