// ---------- rx07_4: the contract manager written to the saved game and destroyed (contract_manager.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\contract_manager.cpp" (the error boxes).
pub(crate) const CONTRACT_MANAGER_CPP: usize = 0x9a8190;

/// "contract.dat", the contract manager's file in the saved game.
pub(crate) const CONTRACT_DAT: usize = 0x9af818;

/// The size of a contract talks record (`contract_manager_fields::TALKS_RECORDS`).
pub(crate) const TALKS_RECORD_SIZE: u32 = 0x15;

/// The size of a contract offer in the two offer lists.
pub(crate) const CONTRACT_OFFER_SIZE: u32 = 0x6f;

/// The version byte the contract manager writes at the end of contract.dat.
pub(crate) const CONTRACT_DAT_VERSION: u8 = 1;

/// The "Save Game Error" box of contract_manager.cpp: the line and its text (title 0x9af90c, body 0x9af828).
pub(crate) unsafe fn contract_save_error_box(line: u32, text: usize) {
    award_load_error_box(CONTRACT_MANAGER_CPP, line, text, (0x9af90c, 0x9af828));
}

/// Writes `n` records of `size` bytes from `data` to the open file `z`; true when all went.
pub(crate) unsafe fn contract_write(data: usize, size: u32, n: u32, z: usize) -> bool {
    exe::zwrite(data, size, n, z) == n
}

/// Writes an offer list to contract.dat: its count (the list's word; 0 without a list) in 2 bytes, then each offer
/// (`CONTRACT_OFFER_SIZE` bytes) walked from the head. The failed count gives `count_line`, a failed offer `offer_line`.
pub(crate) unsafe fn contract_write_offer_list(list: usize, z: usize, count_line: u32, offer_line: u32) -> Option<(u32, usize)> {
    let mut count = 0u32; // the original's dword holds the word with garbage above it; only 2 bytes go
    if list != 0 {
        count = read_u16(list + llist::COUNT) as u32;
    }
    if !contract_write(&mut count as *mut u32 as usize, 2, 1, z) {
        return Some((count_line, 0x9af98c)); // "cannot write offers count"
    }
    if list == 0 {
        return None;
    }
    write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
    let mut e = exe::llist_get_next(list);
    while e != 0 {
        if !contract_write(e, CONTRACT_OFFER_SIZE, 1, z) {
            return Some((offer_line, 0x9af954)); // "cannot write offer"
        }
        e = exe::llist_get_next(list);
    }
    None
}

/// The contents of contract.dat in the order `CONTRACT_MANAGER::load_from_disk` reads them; the first failed write
/// gives its line and text.
pub(crate) unsafe fn contract_write_file(this: usize, z: usize) -> Option<(u32, usize)> {
    let blocks: [(usize, usize, u32, (u32, usize), (u32, usize)); 3] = [
        (contract_manager_fields::TALKS_COUNT, contract_manager_fields::TALKS_RECORDS, TALKS_RECORD_SIZE, (0x3216, 0x9afb7c), (0x3221, 0x9afb34)),
        (contract_manager_fields::CONTRACT_COUNT, 0, 0x50, (0x3229, 0x9afae4), (0x3234, 0x9afaa0)),
        (contract_manager_fields::DECISION_COUNT, contract_manager_fields::DECISION_DATES, 0xe, (0x323c, 0x9afa54), (0x3247, 0x9afa14)),
    ];
    for (count, array, size, count_error, array_error) in blocks {
        if !contract_write(this + count, 4, 1, z) {
            return Some(count_error);
        }
        let n = read_u32(this + count);
        if n != 0 && !contract_write(read_usize(this + array), size, n, z) {
            return Some(array_error);
        }
    }
    if !contract_write(this + contract_manager_fields::DECISION_NEXT_INDEX, 4, 1, z) {
        return Some((0x324f, 0x9af9cc)); // "cannot write current decision index"
    }
    for (list, count_line, offer_line) in [(contract_manager_fields::CONTRACT_OFFERS, 0x325e, 0x326c), (contract_manager_fields::HELD_OFFERS, 0x327d, 0x328b)] {
        if let Some(error) = contract_write_offer_list(read_usize(this + list), z, count_line, offer_line) {
            return Some(error);
        }
    }
    let version = CONTRACT_DAT_VERSION;
    if !contract_write(&version as *const u8 as usize, 1, 1, z) {
        return Some((0x3294, 0x9af91c)); // "cannot write version"
    }
    None
}

/// Rule: the contract manager written to the saved game. Decision dates added since the last sort
/// (`contract_manager_fields::DECISIONS_ADDED` set) are sorted from the next one to process to the end
/// (`exe::crt_qsort` with `contract_day_globals::COMPARE_DECISIONS`) and the mark cleared. contract.dat
/// (`exe::zopen`(save, "contract.dat", 1, 0x16, 4)) gets the talks (count and 21-byte records), the contracts (count
/// and 80-byte records), the decision dates (count and 14-byte records), the next decision index, the two offer lists
/// (count in 2 bytes and 111-byte offers) and the version byte 1, then is closed. Each failure shows the "Save Game
/// Error" box with its line (0x320f the file, 0x3216 to 0x3294 the blocks) and stops.
///
/// Defect of the original: after a failed write the file is left open (no `zclose`); the copy does the same.
/// Original: `CONTRACT_MANAGER::save_to_disk` (0x4dc3a0, contract_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn contract_manager_save_to_disk(this: usize, save: usize) {
    if read_u32(this + contract_manager_fields::DECISIONS_ADDED) != 0 {
        covered!("rx07_4 decision dates sorted");
        let next = read_u32(this + contract_manager_fields::DECISION_NEXT_INDEX);
        let first = read_usize(this + contract_manager_fields::DECISION_DATES).wrapping_add((next as usize).wrapping_mul(0xe));
        exe::crt_qsort(first, read_u32(this + contract_manager_fields::DECISION_COUNT).wrapping_sub(next), 0xe, contract_day_globals::COMPARE_DECISIONS);
        write_u32(this + contract_manager_fields::DECISIONS_ADDED, 0);
    }
    let mut z = exe::zopen(save, CONTRACT_DAT, 1, 0x16, 4);
    if z == 0 {
        return contract_save_error_box(0x320f, 0x9afbd0); // "unable to open save game"
    }
    if let Some((line, text)) = contract_write_file(this, z) {
        return contract_save_error_box(line, text);
    }
    covered!("rx07_4 contract.dat saved");
    exe::zclose(&mut z);
}

/// Comparer for `contract_manager_save_to_disk`: it writes the file once; only the original runs (the save proof
/// compares the written game).
pub(crate) unsafe extern "thiscall" fn compare_contract_manager_save_to_disk(this: usize, save: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, save)
}

/// Rule: the contract manager destroyed with the game. The contract, decision date and talks arrays are freed
/// (`exe::crt_free`, when set) and each of the two offer lists destroyed (`exe::drop_llist`) and deleted
/// (`exe::crt_operator_delete`). The fields keep their values (the caller deletes the object).
/// Original: `CONTRACT_MANAGER::~CONTRACT_MANAGER` (0x4c74f0, contract_manager.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn contract_manager_destructor(this: usize) {
    for field in [0, contract_manager_fields::DECISION_DATES, contract_manager_fields::TALKS_RECORDS] {
        let p = read_usize(this + field);
        if p != 0 {
            exe::crt_free(p);
        }
    }
    for field in [contract_manager_fields::CONTRACT_OFFERS, contract_manager_fields::HELD_OFFERS] {
        let list = read_usize(this + field);
        if list != 0 {
            covered!("rx07_4 offer list destroyed");
            exe::drop_llist(list);
            exe::crt_operator_delete(list);
        }
    }
}

/// Comparer for `contract_manager_destructor`: it frees; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_contract_manager_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}
