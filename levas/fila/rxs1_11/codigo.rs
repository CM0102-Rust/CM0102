/// "E:\dev\CM3\cm3 00-01\cm3\code\transfer_manager.cpp" (the error boxes).
pub(crate) const TRANSFER_MANAGER_CPP: usize = 0xa8d514;

/// "transfer.dat", the transfer manager's file in the saved game.
pub(crate) const TRANSFER_DAT: usize = 0xa8e0d4;

/// `TRANSFER_OFFER::save_to_disk` (0x8e5b30, thiscall(offer, file) ret 4).
pub(crate) const FN_TRANSFER_OFFER_SAVE_TO_DISK: usize = 0x8e5b30;

/// `SHORTLIST_MANAGER::save_to_disk` (0x83b370, thiscall(manager, file) ret 4).
pub(crate) const FN_SHORTLIST_MANAGER_SAVE_TO_DISK: usize = 0x83b370;

/// `SHORTLIST_MANAGER::~SHORTLIST_MANAGER` (0x839d90, thiscall(manager)).
pub(crate) const FN_SHORTLIST_MANAGER_DESTRUCTOR: usize = 0x839d90;

/// `TRANSFER_RULE::~TRANSFER_RULE` (0x8e8540, thiscall(rule)), the 28 per-nation rule objects.
pub(crate) const FN_TRANSFER_RULE_DESTRUCTOR: usize = 0x8e8540;

/// The day schedule of the transfer manager: 150 inline lists of 14 bytes from +0x18 (50 days × 3 slices) of 4-byte
/// offer slots.
pub(crate) const SCHEDULE_BUCKETS: usize = 0x18;

/// The "Save Game Error" box of transfer_manager.cpp: the line and its text (title 0x9af90c, body 0x9af828).
pub(crate) unsafe fn transfer_save_error_box(line: u32, text: usize) {
    award_load_error_box(TRANSFER_MANAGER_CPP, line, text, (0x9af90c, 0x9af828));
}

/// Writes `n` records of `size` bytes from `data` to the open file `z`; true when all went.
pub(crate) unsafe fn transfer_write(data: usize, size: u32, n: u32, z: usize) -> bool {
    exe::zwrite(data, size, n, z) == n
}

/// Writes the count of an inline list (its word, in 2 bytes) and then each of its nodes' data (`size` bytes, walked from
/// the head); unchecked node writes when `node_error` is `None`.
unsafe fn transfer_write_list(list: usize, size: u32, z: usize, count_error: (u32, usize), node_error: Option<(u32, usize)>) -> Option<(u32, usize)> {
    let count = read_u16(list + llist::COUNT) as u32; // the original's dword holds the word with garbage above; 2 bytes go
    if !transfer_write(&count as *const u32 as usize, 2, 1, z) {
        return Some(count_error);
    }
    write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
    let mut e = exe::llist_get_next(list);
    while e != 0 {
        let ok = transfer_write(e, size, 1, z);
        if let (false, Some(error)) = (ok, node_error) {
            return Some(error);
        }
        e = exe::llist_get_next(list);
    }
    None
}

/// Trims an array's capacity to its last used place + 1 (the field `capacity` of the manager; unchanged when no place
/// is used) and returns it.
unsafe fn transfer_trim_capacity(this: usize, array: usize, capacity: usize) -> i32 {
    let places = read_usize(this + array);
    let mut k = read_i32(this + capacity) - 1;
    while k >= 0 {
        if read_usize(places + k as usize * 4) != 0 {
            write_i32(this + capacity, k + 1);
            break;
        }
        k -= 1;
    }
    read_i32(this + capacity)
}

/// The contents of transfer.dat in the order `TRANSFER_MANAGER::load_from_disk` reads them; the first failed write gives
/// its line and text.
pub(crate) unsafe fn transfer_write_file(this: usize, z: usize) -> Option<(u32, usize)> {
    use transfer_manager_fields as f;
    let save_offer: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_OFFER_SAVE_TO_DISK);
    covered!("rxs1_11 transfer manager saved");
    let n = transfer_trim_capacity(this, 0, f::OFFER_CAPACITY);
    if !transfer_write(this + f::OFFER_CAPACITY, 4, 1, z) {
        return Some((0x35a, 0xa8e8c0)); // "cannot write transfer_offers_sz"
    }
    let offers = read_usize(this);
    let live = (0..n.max(0) as usize).filter(|&k| read_usize(offers + k * 4) != 0).count();
    write_u32(this + f::OFFER_COUNT, live as u32);
    if !transfer_write(this + f::OFFER_COUNT, 4, 1, z) {
        return Some((0x36a, 0xa8e878)); // "cannot write number_transfer_offers"
    }
    for k in 0..read_i32(this + f::OFFER_CAPACITY).max(0) as usize {
        let o = read_usize(read_usize(this) + k * 4);
        if o != 0 {
            save_offer(o, z);
        }
    }
    transfer_trim_capacity(this, f::NEGOTIATIONS, f::NEGOTIATION_CAPACITY);
    if !transfer_write(this + f::NEGOTIATION_CAPACITY, 4, 1, z) {
        return Some((0x38a, 0xa8e82c)); // "cannot write transfer_negotiations_sz"
    }
    if !transfer_write(this + f::NEGOTIATION_COUNT, 4, 1, z) {
        return Some((0x392, 0xa8e7dc)); // "cannot write number_transfer_negotiations"
    }
    for k in 0..read_i32(this + f::NEGOTIATION_CAPACITY).max(0) as usize {
        let o = read_usize(read_usize(this + f::NEGOTIATIONS) + k * 4);
        if o != 0 {
            save_offer(o, z);
        }
    }
    for bucket in 0..0x96 {
        if let Some(error) = transfer_write_list(this + SCHEDULE_BUCKETS + bucket * 0xe, 4, z, (0x3b3, 0xa8e750), None) {
            return Some(error); // "unable to save list count"
        }
    }
    if !transfer_write(this + f::DAY_CURSOR, 1, 1, z) {
        return Some((0x3c4, 0xa8e790)); // "unable to save current decision index"
    }
    if !transfer_write(this + f::CURRENT_DATE, 8, 1, z) {
        return Some((0x3cc, 0xa8e710)); // "unable to save current day"
    }
    if !transfer_write(this + f::CURRENT_SLICE, 1, 1, z) {
        return Some((0x3d4, 0xa8e6d0)); // "unable to save current tod"
    }
    let save_shortlists: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_SAVE_TO_DISK);
    save_shortlists(read_usize(this + f::SHORTLIST_MANAGER), z);
    if !transfer_write(this + f::NEXT_SERIAL, 4, 1, z) {
        return Some((0x3df, 0xa8e688)); // "unable to save current transfer id"
    }
    // the counted arrays: (count, array, record size, count error, array error)
    let arrays: [(usize, usize, u32, (u32, usize), (u32, usize)); 2] = [
        (f::PAST_TRANSFER_COUNT, f::PAST_TRANSFERS, 0x25, (0x3e7, 0xa8e63c), (0x3f0, 0xa8e5f8)), // rxs1_11: past transfers
        (f::FUTURE_TRANSFER_COUNT, f::FUTURE_TRANSFERS, 0xc, (0x3f8, 0xa8e5a8), (0x401, 0xa8e564)),
    ];
    for (count, array, size, count_error, array_error) in arrays {
        if !transfer_write(this + count, 4, 1, z) {
            return Some(count_error);
        }
        if !transfer_write(read_usize(this + array), size, read_u32(this + count), z) {
            return Some(array_error);
        }
    }
    if !transfer_write(read_usize(this + f::PERSON_FLAGS), 8, read_u32(G_STAFF_COUNT), z) {
        return Some((0x409, 0xa8e524)); // "unable to save staff info"
    }
    if let Some(error) = transfer_write_list(this + f::INSTALMENTS, 0xd, z, (0x412, 0xa8e4d0), Some((0x41f, 0xa8e434))) {
        return Some(error); // the monthly instalments
    }
    let money: [(usize, usize, (u32, usize), (u32, usize)); 3] = [
        (f::NEXT_SALE_PERCENTAGE_COUNT, f::NEXT_SALE_PERCENTAGES, (0x428, 0xa8e47c), (0x431, 0xa8e3ec)),
        (f::APPEARANCE_MONEY_COUNT, f::APPEARANCE_MONEY, (0x439, 0xa8e3a0), (0x442, 0xa8e35c)),
        (f::INTERNATIONAL_APPEARANCE_MONEY_COUNT, f::INTERNATIONAL_APPEARANCE_MONEY, (0x44a, 0xa8e308), (0x453, 0xa8e2bc)),
    ];
    for (count, array, count_error, array_error) in money {
        if !transfer_write(this + count, 4, 1, z) {
            return Some(count_error);
        }
        if !transfer_write(read_usize(this + array), 0xd, read_u32(this + count), z) {
            return Some(array_error);
        }
    }
    if !transfer_write(read_usize(this + f::SEASON_LOAN_COUNTS), 0x10, read_u32(G_CLUB_COUNT), z) {
        return Some((0x45b, 0xa8e278)); // "unable to write club info list"
    }
    if let Some(error) = transfer_write_list(this + f::FREE_TRANSFER_QUEUE, 9, z, (0x464, 0xa8e230), Some((0x470, 0xa8e198))) {
        return Some(error); // the free transfers
    }
    if !transfer_write(this + f::WORK_PERMIT_COUNT, 4, 1, z) {
        return Some((0x479, 0xa8e1e4)); // "unable to save number of work permits"
    }
    if !transfer_write(read_usize(this + f::WORK_PERMITS), 0xe, read_u32(this + f::WORK_PERMIT_COUNT), z) {
        return Some((0x482, 0xa8e158)); // "unable to save work permits"
    }
    let loans = read_usize(this + f::LOAN_NEGOTIATIONS);
    if loans == 0 {
        let none = 0u32;
        if !transfer_write(&none as *const u32 as usize, 2, 1, z) {
            return Some((0x48b, 0xa8e120)); // "unable to save count"
        }
        return None;
    }
    covered!("rxs1_11 loan negotiations saved");
    transfer_write_list(loans, 0x50, z, (0x48b, 0xa8e120), Some((0x49a, 0xa8e0e4))) // rxs1_11: the loan negotiations
}

/// Writes the transfer manager to transfer.dat in the saved game.
///
/// `TRANSFER_MANAGER::save_to_disk(save)`: opens transfer.dat (`exe::zopen` with 1, 0x16, 4; failing, the "Save Game
/// Error" box of line 0x346 and nothing else) and writes `transfer_write_file`: the offer array's capacity (first
/// trimmed to the last used place + 1), the count of live offers (recounted and stored), each offer
/// (`TRANSFER_OFFER::save_to_disk`), the same for the negotiations (the capacity trimmed, the count as kept), the 150
/// day schedule lists (count in 2 bytes and the 4-byte slots, unchecked), the day cursor, the current date and slice,
/// the shortlists (`SHORTLIST_MANAGER::save_to_disk`), the next serial, the past transfers (0x25 bytes each), the
/// future transfers (12), the per-person records (8 × the staff count), the monthly instalments (a list of 13-byte
/// records), the sell-on percentages, the first team and the international appearance money (13 each), the clubs'
/// season loans (16 × the club count), the free transfer queue (a list of 9-byte records), the work permits (14) and
/// the loan negotiations (a list of 0x50-byte records; count 0 without the list). The first failed write shows the box
/// with its line and closes the file (`exe::zclose`); otherwise the file is closed at the end.
/// Original: `TRANSFER_MANAGER::save_to_disk` (0x8bd4b0, transfer_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn transfer_manager_save_to_disk(this: usize, save: usize) {
    let mut z = exe::zopen(save, TRANSFER_DAT, 1, 0x16, 4);
    if z == 0 {
        transfer_save_error_box(0x346, 0xa8e904); // "unable to open save game"
        return;
    }
    if let Some((line, text)) = transfer_write_file(this, z) {
        transfer_save_error_box(line, text);
    }
    exe::zclose(&mut z);
}

/// Comparer for `transfer_manager_save_to_disk`: the original only (it writes the item to the saved game). The proof
/// is the saved game written with and without the routine.
pub(crate) unsafe extern "thiscall" fn compare_transfer_manager_save_to_disk(this: usize, save: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, save) // simplification: original only
}

/// Destroys the transfer manager with the game.
///
/// `TRANSFER_MANAGER::~TRANSFER_MANAGER()`: the offers and the negotiations (each `exe::drop_transfer_offer` and
/// `exe::crt_operator_delete`) and their arrays (`exe::crt_free`); the past transfers; the shortlist manager
/// (`SHORTLIST_MANAGER::~SHORTLIST_MANAGER` and delete); the per-person records, the clubs' season loans, the three
/// money arrays and the future transfers; the 28 transfer rules (`TRANSFER_RULE::~TRANSFER_RULE` and delete); the
/// transfer windows and the work permits; the loan negotiations list (`exe::drop_llist` and delete); then the free
/// transfer queue and the instalments lists and the 150 schedule lists (`exe::crt_vector_destructor_iterator` with
/// `LLIST::~LLIST`).
/// Original: `TRANSFER_MANAGER::~TRANSFER_MANAGER` (0x8bbe20, transfer_manager.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn transfer_manager_destructor(this: usize) {
    use transfer_manager_fields as f;
    let drop_object = |p: usize, destructor: usize| {
        if p != 0 {
            let d: unsafe extern "thiscall" fn(usize) = core::mem::transmute(destructor);
            d(p);
            exe::crt_operator_delete(p);
        }
    };
    let free = |field: usize| {
        let p = read_usize(this + field);
        if p != 0 {
            exe::crt_free(p);
        }
    };
    for (array, capacity) in [(0, f::OFFER_CAPACITY), (f::NEGOTIATIONS, f::NEGOTIATION_CAPACITY)] {
        if read_usize(this + array) == 0 {
            continue;
        }
        for k in 0..read_i32(this + capacity).max(0) as usize {
            drop_object(read_usize(read_usize(this + array) + k * 4), FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
        }
        exe::crt_free(read_usize(this + array));
    }
    free(f::PAST_TRANSFERS);
    drop_object(read_usize(this + f::SHORTLIST_MANAGER), FN_SHORTLIST_MANAGER_DESTRUCTOR);
    for field in [f::PERSON_FLAGS, f::SEASON_LOAN_COUNTS, f::NEXT_SALE_PERCENTAGES, f::APPEARANCE_MONEY, f::INTERNATIONAL_APPEARANCE_MONEY, f::FUTURE_TRANSFERS] {
        free(field);
    }
    for k in 0..0x1c {
        drop_object(read_usize(this + f::TRANSFER_RULES + k * 4), FN_TRANSFER_RULE_DESTRUCTOR); // rxs1_11: the 28 rules
    }
    free(f::TRANSFER_WINDOWS);
    free(f::WORK_PERMITS);
    drop_object(read_usize(this + f::LOAN_NEGOTIATIONS), FN_LLIST_DESTRUCTOR_LLIST);
    exe::drop_llist(this + f::FREE_TRANSFER_QUEUE);
    exe::drop_llist(this + f::INSTALMENTS);
    covered!("rxs1_11 transfer manager destroyed");
    exe::crt_vector_destructor_iterator(this + SCHEDULE_BUCKETS, 0xe, 0x96, FN_LLIST_DESTRUCTOR_LLIST);
}

/// Comparer for `transfer_manager_destructor`: the original only (it frees).
pub(crate) unsafe extern "thiscall" fn compare_transfer_manager_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this) // simplification: original only
}
