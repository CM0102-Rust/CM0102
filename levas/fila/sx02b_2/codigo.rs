/// How many past transfer records the manager has before `cutoff`: they are in date order, so the count of the leading
/// records dated before it (`transfer_manager_date_before`: year unsigned, then day signed).
unsafe fn count_past_transfers_before(this: TransferManagerFields, cutoff: usize) -> usize {
    (0..this.past_transfer_count())
        .take_while(|&i| transfer_manager_date_before(this.past_transfers() + i * PAST_TRANSFER_SIZE + past_transfer::DATE, cutoff))
        .count()
}

/// Moves the past transfers older than a year to the yearly files and shrinks the array.
///
/// - The cut-off is today (`G_CURRENT_DATE`) less 52 weeks (`exe::date_sub_weeks`). The records in front dated before
/// it (`count_past_transfers_before`) are appended (`exe::crt_fwrite` of 37 bytes) to `transfers_<year>.tmp` in
/// CM3_TEMP (the name sprintf'ed into 0xde1f64, `exe::utils_build_file_path`, `exe::crt_fopen` "ab"), one file open at
/// a time, closed when the year changes and at the end.
/// - A file that does not open: the "Error" box (line 0x1332); a write that fails: the "Error" box (line 0x133c).
/// - With records moved: a new array (malloc of 37 × (left + 100); none: the "Memory Allocation Error" box, line
/// 0x135d, and the array stays as it was) gets the records left, the old one is freed; count = left, capacity = left +
/// 100.
///
/// Defects of the original: after a file fails to open, the other records of that year count as moved without being
/// written anywhere (they are lost); a record whose write fails is not counted, so the array keeps as many of the
/// last written ones (they show twice later) and the failed one is lost; when the new array cannot be allocated, the
/// records already appended to the files also stay in memory (they show twice later).
/// Original: `TRANSFER_MANAGER::clear_old_past_transfers` (0x8c3eb0, transfer_manager.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn clear_old_past_transfers(this: usize) {
    let tm = TransferManagerFields(this);
    covered!("sx02b_2 clear old transfers");
    let mut cutoff = [0u8; 8];
    exe::date_sub_weeks(G_CURRENT_DATE, &mut cutoff, 0x34);
    let old = count_past_transfers_before(tm, cutoff.as_ptr() as usize);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let (mut file, mut year_open, mut moved) = (0usize, 0u16, 0usize);
    for i in 0..old {
        let r = tm.past_transfers() + i * PAST_TRANSFER_SIZE;
        let year = read_u16(r + past_transfer::DATE + cm_date::YEAR);
        if year != year_open {
            if file != 0 {
                exe::crt_fclose(file);
            }
            year_open = year;
            format(0xde1f64, 0xa8ea38, year as u32); // "transfers_%d.tmp"
            file = exe::crt_fopen(exe::utils_build_file_path(0x987da0, 0xde1f64), 0x9c79b0); // "CM3_TEMP", "ab"
            if file == 0 {
                show_error_box(0xa8d514, 0x1332);
                continue;
            }
            covered!("sx02b_2 yearly file");
        } else if file == 0 {
            moved += 1; // the original counts it without a file
            continue;
        }
        if exe::crt_fwrite(r, PAST_TRANSFER_SIZE as u32, 1, file) == 1 {
            moved += 1; // sx02b_2: a record moved to its year's file
        } else {
            show_error_box(0xa8d514, 0x133c);
        }
    }
    if file != 0 {
        exe::crt_fclose(file);
    }
    if moved == 0 {
        return;
    }
    let left = tm.past_transfer_count() - moved;
    let capacity = left + 100;
    let array = exe::crt_malloc((capacity * PAST_TRANSFER_SIZE) as u32);
    if array == 0 {
        show_out_of_memory_box(0x135d, 0xa8e9dc); // "TRANSFER_MANAGER::clear old_past_transfers() : could not malloc new past transfers array"
        return;
    }
    core::ptr::copy_nonoverlapping((tm.past_transfers() + moved * PAST_TRANSFER_SIZE) as *const u8, array as *mut u8, left * PAST_TRANSFER_SIZE);
    exe::crt_free(tm.past_transfers());
    write_usize(this + transfer_manager_fields::PAST_TRANSFERS, array);
    write_usize(this + transfer_manager_fields::PAST_TRANSFER_COUNT, left);
    write_usize(this + 0x866, capacity); // 0x866: past transfer capacity
}

/// Comparer for `clear_old_past_transfers`: only the original runs (it appends to the yearly files). The plan is made
/// beforehand (the records before the cut-off, `count_past_transfers_before`, and a copy of the array); afterwards the
/// count must have dropped by that much, the capacity be the count + 100 (when something moved) and the array hold the
/// records that were after them.
pub(crate) unsafe extern "thiscall" fn compare_clear_old_past_transfers(this: usize) {
    let tm = TransferManagerFields(this);
    let mut cutoff = [0u8; 8];
    exe::date_sub_weeks(G_CURRENT_DATE, &mut cutoff, 0x34);
    let old = count_past_transfers_before(tm, cutoff.as_ptr() as usize);
    let records = || {
        let n = tm.past_transfer_count();
        if n == 0 { Vec::new() } else { core::slice::from_raw_parts(tm.past_transfers() as *const u8, n * PAST_TRANSFER_SIZE).to_vec() }
    };
    let (n, before) = (tm.past_transfer_count(), records());
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let (m, after) = (tm.past_transfer_count(), records());
    let same = m + old == n && (old == 0 || read_usize(this + 0x866) == m + 100) && after[..] == before[old * PAST_TRANSFER_SIZE..];
    SCOREBOARD.record(same, || format!("clear_old_past_transfers: {} records, {} left (plan {} moved)", n, m, old));
}

/// Cancels offer `o` because the manager of `club` left: status `offer_status::CANCELLED`, its contract offers closed
/// (`exe::transfer_manager_close_offer_contract_offers`), the bid markers cleared (the buyer's on the target and the
/// seller's, the club of the target's contract through `club_of_staff_contract`, on each exchange player:
/// `exe::shortlist_manager_clear_bid_markers_for_player`), the target's contract offer from the buyer removed
/// (`exe::contract_manager_remove_contract_offer`), `terms_state()` 0, each exchange player's contract offer for the
/// offer's slot removed (`exe::contract_manager_remove_contract_offer_for_transfer`) and his "terms offered" byte
/// cleared; due again in 10 days (`exe::transfer_manager_schedule_offer`) and the collapse news with reason 0xb
/// (`exe::transfer_manager_post_offer_collapsed_news`).
unsafe fn cancel_offer_of_departed_manager(this: TransferManagerFields, o: Offer, club: usize) {
    o.set_status(offer_status::CANCELLED); // sx02b_2: an offer cancelled
    exe::transfer_manager_close_offer_contract_offers(this, o);
    let target = transfer_manager_staff_by_id(o.target_staff_id());
    let seller = club_of_staff_contract(target);
    let buyer = transfer_manager_club_by_id(o.buyer_club_id());
    exe::shortlist_manager_clear_bid_markers_for_player(this.shortlist_manager(), buyer, target);
    let exchange = o.exchange_players();
    if !exchange.is_null() {
        exchange.set_cursor(exchange.head());
        loop {
            let x = exe::llist_get_next(exchange);
            if x == 0 {
                break;
            }
            exe::shortlist_manager_clear_bid_markers_for_player(this.shortlist_manager(), seller, transfer_manager_staff_by_id(read_usize(x)));
        }
    }
    exe::contract_manager_remove_contract_offer(read_usize(G_CONTRACT_MANAGER), target, buyer);
    o.set_terms_state(0);
    let exchange = o.exchange_players();
    if !exchange.is_null() {
        exchange.set_cursor(exchange.head());
        loop {
            let x = exe::llist_get_next(exchange);
            if x == 0 {
                break;
            }
            exe::contract_manager_remove_contract_offer_for_transfer(read_usize(G_CONTRACT_MANAGER), transfer_manager_staff_by_id(read_usize(x)), o.slot_index());
            write_u8(x + 4, 0); // exchange item +4: terms offered
        }
    }
    exe::transfer_manager_schedule_offer(this, o, 10, 0);
    exe::transfer_manager_post_offer_collapsed_news(this, o, club, 0xb);
}

/// Cancels the offers of a club whose manager leaves, and those for its players.
///
/// - A national team (`exe::database_is_national_team`): nothing.
/// - Each open offer (not `offer_is_closed`) the club made (`buyer_club_id()` = the club's id), by slot up to
/// `offer_capacity()`: unless it is a future transfer (`offer_status::FUTURE_TRANSFER`), it is cancelled
/// (`cancel_offer_of_departed_manager`).
/// - Then each person in the club's squad (`exe::database_collect_club_squad`, 50 slots; empty slots and ids from
/// `G_STAFF_COUNT` up skipped): his open offers (`exe::transfer_manager_get_offers_for_player` with 0) are appended to
/// one list, and every offer in that list that is not a future transfer nor a pre-contract (`pre_contract()`) is
/// cancelled.
///
/// Defect of the original: the list of the squad's offers is never emptied between players, so a player with open
/// offers makes the offers of the players before him go through again: cancelled once more, scheduled again and their
/// collapse news posted again.
/// Original: `TRANSFER_MANAGER::cancel_club_offers` (0x8c4950, transfer_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cancel_club_offers(this: usize, club: usize) {
    let tm = TransferManagerFields(this);
    if exe::database_is_national_team(club) != 0 {
        return;
    }
    let mut list = [0u8; 0x10];
    let l = Llist(list.as_mut_ptr() as usize);
    exe::new_llist(l);
    let id = read_usize(club); // the club's id
    let mut found = 0;
    for k in 0..tm.offer_capacity() as i32 {
        let o = Offer(offer_at(this, k as usize));
        if !o.is_null() && !offer_is_closed(o) && o.buyer_club_id() == id {
            exe::llist_add_tail(l, &k as *const i32 as usize, 4);
            found += 1;
        }
    }
    if found != 0 {
        l.set_cursor(l.head());
        loop {
            let e = exe::llist_get_next(l);
            if e == 0 {
                break;
            }
            let o = Offer(offer_at(this, read_usize(e)));
            if o.status() != offer_status::FUTURE_TRANSFER {
                covered!("sx02b_2 the club's own offer");
                cancel_offer_of_departed_manager(tm, o, club); // sx02b_2: the club's own offer
            }
        }
    }
    exe::llist_clear(l);
    let mut squad = [0usize; 50];
    exe::database_collect_club_squad(club, squad.as_mut_ptr(), 0);
    for &person in squad.iter() {
        if person == 0 || read_i32(person) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        if exe::transfer_manager_get_offers_for_player(this, person, l, 0) == 0 {
            continue;
        }
        l.set_cursor(l.head());
        loop {
            let e = exe::llist_get_next(l);
            if e == 0 {
                break;
            }
            let o = Offer(offer_at(this, read_usize(e)));
            if o.status() != offer_status::FUTURE_TRANSFER && o.pre_contract() == 0 {
                covered!("sx02b_2 an offer for a squad player");
                cancel_offer_of_departed_manager(tm, o, club); // sx02b_2: an offer for a squad player
            }
        }
    }
    exe::drop_llist(l);
}

/// Comparer for `cancel_club_offers`: only the original runs (it schedules offers, removes contract offers and posts
/// news, none of which can run twice).
pub(crate) unsafe extern "thiscall" fn compare_cancel_club_offers(this: usize, club: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, club) // simplification: original only
}
