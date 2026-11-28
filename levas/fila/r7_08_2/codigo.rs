// --- r7_08_2: a club's future transfer list for the club and finance screens (transfer_manager.cpp) ---

/// The parts of a future transfer row that `budget_list_rows` does not name (0x1b bytes; date +0, the other club +0xc,
/// the kind +0x18, the player's awareness +0x1a) [inference: from get_club_future_transfer_list's writes].
#[allow(dead_code)]
pub(crate) mod future_list_row {
    pub(crate) const DATE: usize = 0x0; // 8 bytes: the date the move happens (the future transfer's or the offer's due date)
    pub(crate) const OTHER_CLUB_ID: usize = 0xc; // 4 bytes: the other club's id (-1 when the seller has none)
    pub(crate) const KIND: usize = 0x18; // 1 byte: 0 cash, 1 with exchange players (or an exchange player's row), 4 pre-contract
    pub(crate) const PLAYER_AWARE: usize = 0x1a; // 1 byte: offer::PLAYER_AWARE (not written in an exchange player's row)
}

/// The stage of a future transfer row by the offer's status (the original's jump table): 1 on hold; 2 accepted, pre-contract
/// or awaiting the player; 3 awaiting the work permit; 4 the clubs' confirmations; 5 refused at the approval stage; 6
/// both confirmed or completing later; 0 the rest.
pub(crate) fn future_list_stage(status: u8) -> u8 {
    match status {
        1 => 1,
        5..=8 => 2,
        0xa => 3, // r7_08_2: awaiting the work permit
        0xb..=0xd => 4,
        0x11 | 0x12 => 5,
        0xe | 0x13 => 6,
        _ => 0,
    }
}

/// Grows the rows by 20 when full (`exe::crt_realloc`); false when the memory runs out (the box for `line` is shown).
unsafe fn future_list_room(rows: &mut usize, capacity: &mut u32, count: u32, line: u32) -> bool {
    if count != *capacity {
        return true;
    }
    *capacity = capacity.wrapping_add(20);
    *rows = exe::crt_realloc(*rows, capacity.wrapping_mul(0x1b));
    if *rows == 0 {
        show_transfer_memory_error(line, 0xa8ecfc); // "TRANSFER_MANAGER::get_club_future_transfer_list() : cannot allocate memory"
        return false;
    }
    true
}

/// Builds the list of a club's future transfers for the club and finance screens: one row per open offer the club made or
/// received (loans aside), and one per exchange player in it, sorted by stage and date; returns the number of rows and
/// the array (the caller frees it) through `out`.
///
/// `TRANSFER_MANAGER::get_club_future_transfer_list(club, out)`, from the club screen (0x455945) and the transfer budget
/// (0x5988d8, a human club with `pending`). A local LLIST (`exe::new_llist`) gets the index of every offer of the club as
/// buyer (`offer::BUYER_CLUB_ID`) still open: not rejected, withdrawn, lapsed, cancelled or completed (statuses 2, 3, 4,
/// 0x10, 0x14), and a collapsed one (0xf) only while `offer::RESPONSE_ROUND` is below 3; then the offers for the club's
/// players (TRANSFER_MANAGER::get_open_offers_for_club_players 0x8c6100). For each listed offer that is not a loan
/// (`offer::OFFER_TYPE` 1): the seller is the club of the target's contract (`exe::contract_manager_get_contract`, kept
/// in `G_CONTRACT_LOOKUP_RESULT`; none for a loan contract, type 5), the stage `future_list_stage`, and the date the
/// one of the future transfer of the offer's slot (`transfer_manager_fields::FUTURE_TRANSFERS`, `future_transfer::OFFER_SLOT`),
/// else the offer's `offer::DUE_DATE`. The row: date, stage, the target, `offer::PLAYER_AWARE`, the fee, and as buyer the
/// seller's id (−1 without) with outgoing 0, as seller the buyer's id with outgoing 1; kind 4 for a pre-contract
/// (`offer::PRE_CONTRACT`) without a fee completing at the contract end (timing 3 or 4), else 1 with exchange players,
/// else 0. Each exchange player (`offer::EXCHANGE_PLAYERS`) adds a row with the same date and stage, his id, the same
/// clubs with outgoing reversed, fee 0 and kind 1. The rows (0x1b bytes, grown 20 at a time by `exe::crt_realloc`) are
/// sorted with `exe::crt_qsort` and the comparer 0x8e55f0 (stage, then date). Out of memory: the "Memory Allocation
/// Error" box (lines 0x2b2c and 0x2b63), `out` = 0 and 0 rows.
///
/// Defect of the original: an exchange player's row never gets +0x1a (`future_list_row::PLAYER_AWARE`): it keeps what
/// the reallocated memory held. Kept (the copy does not write it either).
/// Original: `TRANSFER_MANAGER::get_club_future_transfer_list` (0x8ccb80, transfer_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_club_future_transfer_list(this: usize, club: usize, out: *mut usize) -> u32 {
    const GET_OPEN_OFFERS_FOR_CLUB_PLAYERS: usize = 0x8c6100; // TRANSFER_MANAGER::get_open_offers_for_club_players, thiscall, ret 8
    const COMPARE_FUTURE_ROWS: usize = 0x8e55f0; // the rows' qsort comparer (stage, then date)
    let tm = TransferManagerFields(this);
    let mut list_buf = [0u8; 0xe];
    let list = Llist(list_buf.as_mut_ptr() as usize);
    exe::new_llist(list);
    let (mut rows, mut capacity, mut n) = (0usize, 0u32, 0u32);
    let mut date = [0u32; 2];
    exe::date_date_54a110(date.as_mut_ptr() as usize);
    let club_id = read_u32(club);
    let offer_at = |k: usize| Offer(read_usize(read_usize(this).wrapping_add(k.wrapping_mul(4))));
    let mut k: i32 = 0;
    while k < tm.offer_capacity() as i32 {
        let o = offer_at(k as usize);
        if !o.is_null() {
            let open = match o.status() {
                2 | 3 | 4 | 0x10 | 0x14 => false,
                0xf => o.response_round() < 3, // r7_08_2: a collapsed offer still open
                _ => true,
            };
            if open && o.buyer_club_id() as u32 == club_id {
                exe::llist_add_tail(list, (&k) as *const i32 as usize, 4);
            }
        }
        k += 1;
    }
    let open_offers: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(GET_OPEN_OFFERS_FOR_CLUB_PLAYERS);
    open_offers(this, club, list.addr());
    list.set_cursor(list.head());
    let mut e = exe::llist_get_next(list);
    while e != 0 {
        let o = offer_at(read_usize(e));
        if o.offer_type() != 1 {
            let c = Contract(transfer_manager_get_contract_stored(staff_records_staff_by_id(o.target_staff_id()).addr()));
            let seller = if c.is_null() || c.contract_type() & 0x3f == 5 { Club(0) } else { transfer_manager_club_by_id(c.club()) };
            let stage = future_list_stage(o.status());
            let mut source = o.addr() + offer::DUE_DATE;
            for i in 0..tm.future_transfer_count() as i32 {
                let f = tm.future_transfers().wrapping_add((i as usize).wrapping_mul(0xc));
                if read_usize(f + future_transfer::OFFER_SLOT) == o.slot_index() {
                    source = f; // r7_08_2: the date of the future transfer
                    break;
                }
            }
            transfer_manager_assign_date(date.as_mut_ptr() as usize, source);
            if !future_list_room(&mut rows, &mut capacity, n, 0x2b2c) {
                return future_list_failed(list, out);
            }
            let buying = o.buyer_club_id() as u32 == club_id;
            let other = if buying { if seller.is_null() { u32::MAX } else { read_u32(seller.addr()) } } else { o.buyer_club_id() as u32 };
            let row = rows.wrapping_add((n as usize).wrapping_mul(0x1b));
            write_u8(row + budget_list_rows::FUTURE_STAGE, stage);
            transfer_manager_assign_date(row + future_list_row::DATE, date.as_ptr() as usize);
            write_usize(row + budget_list_rows::FUTURE_PERSON_ID, o.target_staff_id());
            write_u8(row + future_list_row::PLAYER_AWARE, o.player_aware());
            write_u32(row + future_list_row::OTHER_CLUB_ID, other);
            write_u32(row + budget_list_rows::FUTURE_OUTGOING, !buying as u32);
            write_usize(row + budget_list_rows::FUTURE_FEE, o.fee());
            let kind = if o.pre_contract() != 0 && o.fee() == 0 && matches!(o.completion_timing() & 0xf, 3 | 4) {
                4
            } else {
                !o.exchange_players().is_null() as u8
            };
            write_u8(row + future_list_row::KIND, kind);
            n += 1;
            let l = o.exchange_players();
            if !l.is_null() {
                l.set_cursor(l.head());
                let mut item = exe::llist_get_next(l);
                while item != 0 {
                    if !future_list_room(&mut rows, &mut capacity, n, 0x2b63) {
                        return future_list_failed(list, out);
                    }
                    covered!("r7_08_2 exchange player row");
                    let row = rows.wrapping_add((n as usize).wrapping_mul(0x1b));
                    write_u8(row + budget_list_rows::FUTURE_STAGE, stage);
                    transfer_manager_assign_date(row + future_list_row::DATE, date.as_ptr() as usize);
                    write_u32(row + budget_list_rows::FUTURE_PERSON_ID, read_u32(item));
                    write_u32(row + future_list_row::OTHER_CLUB_ID, other);
                    write_u32(row + budget_list_rows::FUTURE_OUTGOING, buying as u32); // r7_08_2: the exchange player moves the other way
                    write_u32(row + budget_list_rows::FUTURE_FEE, 0);
                    write_u8(row + future_list_row::KIND, 1);
                    n += 1;
                    item = exe::llist_get_next(l);
                }
            }
        }
        e = exe::llist_get_next(list);
    }
    exe::crt_qsort(rows, n, 0x1b, COMPARE_FUTURE_ROWS);
    *out = rows;
    exe::drop_llist(list);
    n
}

/// The out-of-memory exit of `get_club_future_transfer_list`: `out` = 0, the local list destroyed, 0 rows.
unsafe fn future_list_failed(list: Llist, out: *mut usize) -> u32 {
    *out = 0;
    exe::drop_llist(list);
    0
}

/// Comparer for `get_club_future_transfer_list`: the original and the copy side by side; the count and the rows must
/// match (+0x1a only in rows of kind 0 and 4: an exchange player's row leaves it unwritten), and the cached contract.
/// The copy's array is freed; the caller gets the original's.
pub(crate) unsafe extern "thiscall" fn compare_get_club_future_transfer_list(this: usize, club: usize, out: *mut usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut usize) -> u32>(%%I0%%);
    let mut mine = 0usize;
    let regions = [(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    let (n_orig, n_rust, same_cache, _, _) = crate::comparison::side_by_side_light(&regions, || o(this, club, out), || get_club_future_transfer_list(this, club, &mut mine));
    let theirs = *out;
    let row_same = |k: usize| {
        let (a, b) = (theirs + k * 0x1b, mine + k * 0x1b);
        let kind = read_u8(a + future_list_row::KIND);
        (0..0x1b).all(|i| i == future_list_row::PLAYER_AWARE && kind == 1 || read_u8(a + i) == read_u8(b + i))
    };
    let same = same_cache && n_orig == n_rust && (theirs == 0) == (mine == 0) && (theirs == 0 || (0..n_orig as usize).all(row_same));
    SCOREBOARD.record(same, || format!("get_club_future_transfer_list(club {:#x}) rows original={} rust={}", read_u32(club), n_orig, n_rust));
    if mine != 0 {
        exe::crt_free(mine);
    }
    n_orig
}
