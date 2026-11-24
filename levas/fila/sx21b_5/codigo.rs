// ---------- sx21b_5: a transfer offer in the saved game: loading and saving it (transfer_offer.cpp) ----------

/// Reads a transfer offer from the saved game: the 0x41-byte record, its rival offer list and its exchange players.
///
/// `TRANSFER_OFFER::load(stream, offers)`. The record (`exe::zread`, 0x41 bytes) and a word count; then that many offer
/// slot indices: an index below this offer's own `offer::SLOT_INDEX` names an offer already loaded (`offers[index]`,
/// whose rival list this offer shares: +0 takes that list; a missing one shows the box, line 0x99); every index also
/// goes into a new list of its own (`exe::llist_add_tail`, 4 bytes), which becomes the rival list when none was shared
/// and is otherwise emptied and freed. A count with no `offers` shows the box (line 0x7a). Then another word count and
/// that many 7-byte exchange player nodes into a new `offer::EXCHANGE_PLAYERS` list (none: 0). A short read shows the
/// box (lines 0x69, 0x70, 0x8d, 0xbe, 0xd0) and stops there.
/// Original: `TRANSFER_OFFER::load` (0x8e5790, transfer_offer.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn transfer_offer_load(this: usize, stream: usize, offers: usize) {
    const FILE: usize = 0xa9e318; // transfer_offer.cpp
    if exe::zread(this, 0x41, 1, stream) != 1 {
        show_error_at(FILE, 0x69);
        return;
    }
    let mut count = 0i16;
    if exe::zread(&mut count as *mut i16 as usize, 2, 1, stream) != 1 {
        show_error_at(FILE, 0x70);
        return;
    }
    if count != 0 {
        if offers == 0 {
            show_error_at(FILE, 0x7a);
            return;
        }
        write_u32(this, 0); // the rival offer list
        let list = transfer_offer_new_list();
        let mut k = 0i16;
        while k < count {
            let mut index = 0i32;
            if exe::zread(&mut index as *mut i32 as usize, 4, 1, stream) != 1 {
                show_error_at(FILE, 0x8d);
                return;
            }
            if index < Offer(this).slot_index() as i32 { // sx21b_5: an offer loaded before shares its list
                let earlier = read_usize(offers + index as usize * 4);
                if earlier == 0 {
                    show_error_at(FILE, 0x99);
                } else {
                    covered!("sx21b_5 shared rival list");
                    write_u32(this, read_u32(earlier));
                }
            }
            exe::llist_add_tail(list, &mut index as *mut i32 as usize, 4);
            k += 1;
        }
        if read_u32(this) == 0 {
            write_usize(this, list);
        } else {
            exe::llist_clear(list);
            if list != 0 {
                exe::drop_llist(list);
                exe::crt_operator_delete(list);
            }
        }
    }
    if exe::zread(&mut count as *mut i16 as usize, 2, 1, stream) != 1 {
        show_error_at(FILE, 0xbe);
        return;
    }
    if count == 0 {
        write_u32(this + offer::EXCHANGE_PLAYERS, 0);
        return;
    }
    let list = transfer_offer_new_list();
    write_usize(this + offer::EXCHANGE_PLAYERS, list);
    let mut k = 0i16;
    while k < count {
        let mut node = [0u8; 8];
        if exe::zread(node.as_mut_ptr() as usize, 7, 1, stream) != 1 {
            show_error_at(FILE, 0xd0);
            return;
        }
        covered!("sx21b_5 exchange player loaded");
        exe::llist_add_tail(read_usize(this + offer::EXCHANGE_PLAYERS), node.as_mut_ptr() as usize, 7); // sx21b_5: 7-byte nodes
        k += 1;
    }
}

/// A new empty list (`operator new(0xe)` and `LLIST::LLIST`), or 0 when the allocation fails.
unsafe fn transfer_offer_new_list() -> usize {
    let p = exe::crt_operator_new(0xe);
    if p != 0 { exe::new_llist(p) } else { 0 }
}

/// Comparer for `transfer_offer_load`: only the original (it reads the saved game's stream and allocates); the copy
/// is proven by loading the holiday save with and without the batch.
pub(crate) unsafe extern "thiscall" fn compare_transfer_offer_load(this: usize, stream: usize, offers: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%)(this, stream, offers)
}

/// Writes a transfer offer to the saved game: the 0x41-byte record, its rival offer list and its exchange players.
///
/// `TRANSFER_OFFER::save(stream)`: the record (`exe::zwrite`, 0x41 bytes); the rival list's count (a word, 0 without
/// a list) and each of its 4-byte entries; the exchange list's count and each of its 7-byte nodes. A short write shows
/// the box (lines 0xee, 0x102, 0x110, 0x125, 0x133) and stops there.
/// Original: `TRANSFER_OFFER::save` (0x8e5b30, transfer_offer.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn transfer_offer_save(this: usize, stream: usize) {
    const FILE: usize = 0xa9e318; // transfer_offer.cpp
    if exe::zwrite(this, 0x41, 1, stream) != 1 {
        show_error_at(FILE, 0xee);
        return;
    }
    for (list, size, count_line, node_line) in [(read_usize(this), 4u32, 0x102, 0x110), (read_usize(this + offer::EXCHANGE_PLAYERS), 7, 0x125, 0x133)] {
        let mut count = if list != 0 { read_u16(list) } else { 0 };
        if exe::zwrite(&mut count as *mut u16 as usize, 2, 1, stream) != 1 {
            show_error_at(FILE, count_line);
            return;
        }
        if list == 0 {
            continue;
        }
        let l = Llist(list);
        l.set_cursor(l.head());
        loop {
            let d = exe::llist_get_next(l);
            if d == 0 {
                break;
            }
            covered!("sx21b_5 list entry saved");
            if exe::zwrite(d, size, 1, stream) != 1 { // sx21b_5: one entry
                show_error_at(FILE, node_line);
                return;
            }
        }
    }
}

/// Comparer for `transfer_offer_save`: only the original (it writes into the saved game's stream); the copy is proven
/// by the saved game written with and without the batch.
pub(crate) unsafe extern "thiscall" fn compare_transfer_offer_save(this: usize, stream: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, stream)
}
