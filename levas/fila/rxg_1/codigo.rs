// ---------- rxg_1: the transfer manager loaded from the saved game (transfer_manager.cpp) ----------

/// `TRANSFER_OFFER::load` (0x8e5790, thiscall(offer, stream, offers) ret 8; sx21b_5).
const FN_TRANSFER_OFFER_LOAD: usize = 0x8e5790;

/// How a failed step of `transfer_read_file` ends: the box it shows and the value the load returns.
enum TransferLoadFailure {
    /// "Load Game Error" with "The save game is corrupted...": the load returns 0.
    Corrupted(u32, usize),
    /// "Memory Allocation Error": the load returns 0.
    Memory(u32, usize),
    /// The plain "Error" box (`tel26_error_box`, line only): the load returns 0.
    Plain(u32),
    /// "Load Game Error", but the load still returns 1 (the original's own exit, see the routine's header).
    CorruptedButLoaded(u32, usize),
}

/// Reads one dword field of the manager; false when it did not come.
unsafe fn transfer_read(data: usize, size: u32, n: u32, z: usize) -> bool {
    exe::zread(data, size, n, z) == n
}

/// Reads a capacity field (`capacity`), copies it to the count field (`count`) and, when not 0, allocates the array
/// (`array`, `size` bytes a record, `exe::crt_malloc`) and reads it whole.
unsafe fn transfer_read_array(this: usize, z: usize, (capacity, count, array, size): (usize, usize, usize, u32), errors: [(u32, usize); 3]) -> Result<(), TransferLoadFailure> {
    use TransferLoadFailure::*;
    if !transfer_read(this + capacity, 4, 1, z) {
        return Err(Corrupted(errors[0].0, errors[0].1));
    }
    let n = read_u32(this + capacity);
    write_u32(this + count, n);
    if n == 0 {
        return Ok(());
    }
    let p = exe::crt_malloc(n.wrapping_mul(size));
    write_usize(this + array, p);
    if p == 0 {
        return Err(Memory(errors[1].0, errors[1].1));
    }
    if exe::zread(p, size, read_u32(this + count), z) != read_u32(this + count) {
        return Err(Corrupted(errors[2].0, errors[2].1));
    }
    Ok(())
}

/// Reads the offers or the negotiations: the capacity and the count, the pointer array (`exe::crt_calloc`) and each
/// record (a new `TRANSFER_OFFER` loaded with `TRANSFER_OFFER::load`). An offer goes to its own slot; a negotiation to
/// the slot its offer names (`offer::NEGOTIATION_INDEX`). A failed `new` shows the memory box and stops the records,
/// but the load goes on.
unsafe fn transfer_read_offers(this: usize, z: usize, negotiations: bool) -> Result<(), TransferLoadFailure> {
    use transfer_manager_fields as f;
    use TransferLoadFailure::*;
    let load: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRANSFER_OFFER_LOAD);
    let (array, capacity, count) = if negotiations { (f::NEGOTIATIONS, f::NEGOTIATION_CAPACITY, f::NEGOTIATION_COUNT) } else { (0, f::OFFER_CAPACITY, f::OFFER_COUNT) };
    let lines: [(u32, usize); 4] = if negotiations {
        [(0x198, 0xa8df24), (0x1a0, 0xa8ded4), (0x1ac, 0xa8de88), (0x1b9, 0xa8de34)]
    } else {
        [(0x16a, 0xa8e050), (0x172, 0xa8e008), (0x17e, 0xa8dfc4), (0x18b, 0xa8df70)]
    };
    if !transfer_read(this + capacity, 4, 1, z) {
        return Err(Corrupted(lines[0].0, lines[0].1)); // "cannot read transfer_offers_sz"
    }
    if !transfer_read(this + count, 4, 1, z) {
        return Err(Corrupted(lines[1].0, lines[1].1)); // "cannot read number_transfer_offers"
    }
    let n = read_u32(this + capacity);
    if n != 0 {
        let p = exe::crt_calloc(n, 4);
        write_usize(this + array, p);
        if p == 0 {
            return Err(Memory(lines[2].0, lines[2].1)); // "cannot malloc transfer offers"
        }
    }
    let mut k = 0;
    while k < read_i32(this + count) {
        let memory = exe::crt_operator_new(0x41);
        let o = if memory == 0 { 0 } else { exe::new_transfer_offer(memory) };
        if o == 0 {
            award_load_error_box(TRANSFER_MANAGER_CPP, lines[3].0, lines[3].1, MEMORY_ERROR); // and the load goes on
            break;
        }
        let offers = read_usize(this);
        if negotiations {
            load(o, z, 0);
            let slot = read_usize(offers + read_u32(o + offer::SLOT_INDEX) as usize * 4);
            write_usize(read_usize(this + f::NEGOTIATIONS) + read_u32(slot + offer::NEGOTIATION_INDEX) as usize * 4, o);
        } else {
            load(o, z, offers);
            write_usize(read_usize(this) + read_u32(o + offer::SLOT_INDEX) as usize * 4, o); // rxg_1: the offer's slot
        }
        k += 1;
    }
    Ok(())
}

/// Reads a word count into `count` and then that many `size`-byte records into `buffer`, each appended to `list`
/// (`exe::llist_add_tail`); a short record read gives `node_error`.
unsafe fn transfer_read_list(list: usize, z: usize, count: &mut i16, buffer: usize, size: u32, count_error: TransferLoadFailure, node_error: TransferLoadFailure) -> Result<(), TransferLoadFailure> {
    if !transfer_read(count as *mut i16 as usize, 2, 1, z) {
        return Err(count_error);
    }
    for _ in 0..*count as i32 {
        if !transfer_read(buffer, size, 1, z) {
            return Err(node_error);
        }
        exe::llist_add_tail(list, buffer, size);
    }
    Ok(())
}

/// The contents of transfer.dat in the order `TRANSFER_MANAGER::save_to_disk` writes them (`transfer_write_file`,
/// rxs1_11); the first failed step gives its box.
unsafe fn transfer_read_file(this: usize, z: usize) -> Result<(), TransferLoadFailure> {
    use transfer_manager_fields as f;
    use TransferLoadFailure::*;
    covered!("rxg_1 transfer manager loaded");
    transfer_read_offers(this, z, false)?;
    transfer_read_offers(this, z, true)?;
    // the 150 schedule lists (50 days × 3 slices): a word count and the 4-byte slots, the slot reads unchecked
    let mut count = 0i16;
    let mut slot = 0u32; // one local for all, as the original's (a failed read appends the last value again)
    for bucket in 0..0x96 {
        if !transfer_read(&mut count as *mut i16 as usize, 2, 1, z) {
            return Err(Corrupted(0x1d5, 0xa8dda8)); // "unable to load list count"
        }
        for _ in 0..count as i32 {
            exe::zread(&mut slot as *mut u32 as usize, 4, 1, z);
            exe::llist_add_tail(this + SCHEDULE_BUCKETS + bucket * 0xe, &mut slot as *mut u32 as usize, 4);
        }
    }
    if !transfer_read(this + f::DAY_CURSOR, 1, 1, z) {
        return Err(Corrupted(0x1e9, 0xa8dde8)); // "unable to read current decision index"
    }
    if !transfer_read(this + f::CURRENT_DATE, 8, 1, z) {
        return Err(Corrupted(0x1f1, 0xa8dd68)); // "unable to read current day"
    }
    if !transfer_read(this + f::CURRENT_SLICE, 1, 1, z) {
        return Err(Corrupted(0x1f9, 0xa8dd28)); // "unable to read current tod"
    }
    let memory = exe::crt_operator_new(0xe0);
    let shortlists = if memory == 0 {
        0
    } else {
        let construct: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_CONSTRUCTOR);
        construct(memory, z) // the loading constructor reads the shortlists from the same stream
    };
    write_usize(this + f::SHORTLIST_MANAGER, shortlists);
    if shortlists == 0 {
        return Err(Corrupted(0x202, 0xa8dce0)); // "cannot create shortlist manager"
    }
    if !transfer_read(this + f::NEXT_SERIAL, 4, 1, z) {
        return Err(Plain(0x20a));
    }
    // (capacity, count, array, record size): the capacities are read and copied to the counts
    transfer_read_array(this, z, (0x866, f::PAST_TRANSFER_COUNT, f::PAST_TRANSFERS, 0x25), [(0x212, 0xa8dc90), (0x21e, 0xa8dc48), (0x227, 0xa8dc04)])?; // 0x866: past transfer capacity
    transfer_read_array(this, z, (0x924, f::FUTURE_TRANSFER_COUNT, f::FUTURE_TRANSFERS, 0xc), [(0x230, 0xa8dbb4), (0x23c, 0xa8db6c), (0x245, 0xa8db24)])?; // 0x924: future transfer capacity
    let staff = read_u32(G_STAFF_COUNT);
    if exe::zread(read_usize(this + f::PERSON_FLAGS), 8, staff, z) != read_u32(G_STAFF_COUNT) {
        return Err(Corrupted(0x24e, 0xa8dae4)); // "unable to read staff info"
    }
    let mut instalment = [0u8; 0xd];
    transfer_read_list(this + f::INSTALMENTS, z, &mut count, instalment.as_mut_ptr() as usize, 0xd, Corrupted(0x256, 0xa8da94), Corrupted(0x261, 0xa8d9f8))?;
    transfer_read_array(this, z, (0x884, f::NEXT_SALE_PERCENTAGE_COUNT, f::NEXT_SALE_PERCENTAGES, 0xd), [(0x26d, 0xa8da40), (0x27a, 0xa8d9ac), (0x283, 0xa8d960)])?; // 0x884: capacity
    transfer_read_array(this, z, (0x890, f::APPEARANCE_MONEY_COUNT, f::APPEARANCE_MONEY, 0xd), [(0x28c, 0xa8d910), (0x299, 0xa8d8c8), (0x2a2, 0xa8d884)])?; // 0x890: capacity
    transfer_read_array(this, z, (0x89c, f::INTERNATIONAL_APPEARANCE_MONEY_COUNT, f::INTERNATIONAL_APPEARANCE_MONEY, 0xd), [(0x2ab, 0xa8d830), (0x2b8, 0xa8d7e0), (0x2c1, 0xa8d794)])?; // 0x89c: capacity
    let clubs = read_u32(G_CLUB_COUNT);
    if exe::zread(read_usize(this + f::SEASON_LOAN_COUNTS), 0x10, clubs, z) != read_u32(G_CLUB_COUNT) {
        return Err(CorruptedButLoaded(0x2ca, 0xa8d750)); // "unable to read club info list"
    }
    transfer_read_free_transfers(this, z, &mut count)?;
    transfer_read_array(this, z, (0x942, f::WORK_PERMIT_COUNT, f::WORK_PERMITS, 0xe), [(0x2f2, 0xa8d6b8), (0x2ff, 0xa8d620), (0x308, 0xa8d5dc)])?; // 0x942: work permit capacity
    if !transfer_read(&mut count as *mut i16 as usize, 2, 1, z) {
        return Err(Corrupted(0x311, 0xa8d588)); // "unable to read negotiated_loan_contracts count"
    }
    if count != 0 {
        covered!("rxg_1 loan negotiations loaded");
        let memory = exe::crt_operator_new(0xe);
        write_usize(this + f::LOAN_NEGOTIATIONS, if memory == 0 { 0 } else { exe::new_llist(memory) });
    }
    // the 0x50-byte records go through the original's local STAFF_CONTRACT (constructed once at the start and
    // overwritten whole by each read: the copy reads into plain bytes)
    let mut contract = [0u8; 0x50];
    for _ in 0..count as i32 {
        if !transfer_read(contract.as_mut_ptr() as usize, 0x50, 1, z) {
            return Err(Corrupted(0x320, 0xa8d548)); // "unable to read contract"
        }
        exe::llist_add_tail(read_usize(this + f::LOAN_NEGOTIATIONS), contract.as_mut_ptr() as usize, 0x50);
    }
    Ok(())
}

/// The free transfer queue: a word count and the 9-byte records (person, reason, compensation). Saves of data version
/// below 0x3e003 (`game_data_version`) kept 5 bytes, read unchecked, and the compensation becomes -1.
unsafe fn transfer_read_free_transfers(this: usize, z: usize, count: &mut i16) -> Result<(), TransferLoadFailure> {
    use TransferLoadFailure::*;
    if !transfer_read(count as *mut i16 as usize, 2, 1, z) {
        return Err(CorruptedButLoaded(0x2d2, 0xa8d704)); // "unable to read free transfers count"
    }
    let mut record = [0u8; 9];
    let r = record.as_mut_ptr() as usize;
    for _ in 0..*count as i32 {
        if game_data_version() < 0x3e003 {
            exe::zread(r, 4, 1, z);
            exe::zread(r + 4, 1, 1, z);
            write_i32(r + 5, -1);
        } else if !transfer_read(r, 9, 1, z) {
            return Err(CorruptedButLoaded(0x2e6, 0xa8d664)); // "unable to read free transfer staff structure"
        }
        exe::llist_add_tail(this + transfer_manager_fields::FREE_TRANSFER_QUEUE, r, 9);
    }
    Ok(())
}

/// Loads the transfer manager from transfer.dat in the saved game.
///
/// `TRANSFER_MANAGER::load_from_disk(save)`: with no save, 0. Opens transfer.dat (`exe::zopen` with 0, 0x16, 4;
/// failing, the "Load Game Error" box of line 0x163 and 0) and reads `transfer_read_file`, in the order the save writes
/// it: the offers (capacity, count, the pointer array by calloc and each `TRANSFER_OFFER` loaded into its slot) and
/// the negotiations (each into the slot its offer names); the 150 day schedule lists (a word count and the 4-byte
/// slots, the slot reads unchecked); the day cursor, the current date and slice; a new `Shortlist_Manager` built from
/// the same stream; the next serial; the past transfers (0x25 bytes each) and the future transfers (12), the capacity
/// read and copied to the count and the array allocated by malloc; the per-person records (8 × the staff count) into
/// the array already there; the monthly instalments (a list of 13-byte records); the sell-on percentages, the first
/// team and the international appearance money (13 each); the clubs' season loans (16 × the club count); the free
/// transfer queue (9-byte records; 5 bytes and compensation -1 below data version 0x3e003); the work permits (14);
/// and the loan negotiations (a new list when the count is not 0, of 0x50-byte records). Then the file is closed and
/// the load returns 1.
///
/// A failed step shows its box ("Load Game Error" with "The save game is corrupted...", or "Memory Allocation Error"
/// for a failed malloc or calloc), closes the file and returns 0; a failed read of the next serial shows the plain
/// "Error" box instead. Defects of the original: a failed `new` of an offer or a negotiation shows the memory box and
/// skips the rest of those records, but the load goes on and can return 1; the failed reads of the club season loans,
/// the free transfer count or a free transfer record show the box and close the file but return 1, leaving the work
/// permits and the loan negotiations unread; the schedule slot reads and the old 5-byte free transfer reads are not
/// checked. The copy does the same.
/// Original: `TRANSFER_MANAGER::load_from_disk` (0x8bc020, transfer_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn transfer_manager_load_from_disk(this: usize, save: usize) -> u32 {
    use TransferLoadFailure::*;
    if save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, TRANSFER_DAT, 0, 0x16, 4);
    if z == 0 {
        award_load_error_box(TRANSFER_MANAGER_CPP, 0x163, 0xa8e094, LOAD_ERROR); // "unable to open save game"
        return 0;
    }
    let loaded = match transfer_read_file(this, z) {
        Ok(()) => 1,
        Err(Corrupted(line, text)) => {
            award_load_error_box(TRANSFER_MANAGER_CPP, line, text, LOAD_ERROR);
            0
        }
        Err(Memory(line, text)) => {
            award_load_error_box(TRANSFER_MANAGER_CPP, line, text, MEMORY_ERROR);
            0
        }
        Err(Plain(line)) => {
            tel26_error_box(TRANSFER_MANAGER_CPP, line);
            0
        }
        Err(CorruptedButLoaded(line, text)) => {
            award_load_error_box(TRANSFER_MANAGER_CPP, line, text, LOAD_ERROR);
            1
        }
    };
    exe::zclose(&mut z);
    loaded
}

/// Comparer for `transfer_manager_load_from_disk`: the original only (it allocates and reads the stream). The proof is
/// the load of the saved game with and without the routine.
pub(crate) unsafe extern "thiscall" fn compare_transfer_manager_load_from_disk(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, save)
}
