// ---------- batch %%LEVA%%: the shortlist manager loaded from the saved game (shortlist_manager.cpp) ----------

/// The save data versions that changed the shortlist manager's block (`G_GAME_DATA_VERSION`, by its absolute value):
/// below `SHORTLIST_OLD_RECORDS_GONE` the save still carries a block of old 21-byte records; up to it the last position
/// list does not exist and the clubs' slots are the old 36-byte ones; below `SHORTLIST_LAST_LIST_SAVED` the last
/// position list of each group is not saved and the position lists are rebuilt after the load.
pub(crate) const SHORTLIST_OLD_RECORDS_GONE: u32 = 0x3e016;
pub(crate) const SHORTLIST_LAST_LIST_SAVED: u32 = 0x3e017;

/// The old shortlist slot (saves up to `SHORTLIST_OLD_RECORDS_GONE`): 25 of them per club, 0x24 bytes each.
pub(crate) const SHORTLIST_OLD_SLOT_SIZE: usize = 0x24;
pub(crate) const SHORTLIST_OLD_SLOTS: usize = 25;

/// The person-keyed pair qsort comparer (cdecl), as used on `shortlist_manager_fields::PAIRS_BY_CLUB`.
pub(crate) const SHORTLIST_PAIR_COMPARE: usize = 0x844740;

/// Reads the shortlist manager from the saved game: the shortlist pairs, the 32 position lists and every club's 40
/// shortlist slots, converting the blocks of older saves.
///
/// `Shortlist_Manager::load(file)`, from the constructor with the open save stream (the TRANSFER_MANAGER's); the
/// version is |`G_GAME_DATA_VERSION`|:
/// - the pair count into both capacities and both counts (+8, `PAIRS_BY_CLUB_CAPACITY`, `PAIRS_BY_PERSON_COUNT`,
///   `PAIRS_BY_CLUB_COUNT`); both arrays (+0 and `PAIRS_BY_CLUB`, 8 bytes a pair, `malloc`); the pairs read into the
///   first, copied into the second and that one sorted by club (`qsort` with 0x844740);
/// - below `SHORTLIST_OLD_RECORDS_GONE`: a count and that many 21-byte old records read and thrown away;
/// - the 16 position list counts (`POSITION_LIST_COUNTS`; up to `SHORTLIST_OLD_RECORDS_GONE` the 16th is 0, not
///   read) and the list capacity (`POSITION_LIST_CAPACITY`);
/// - the two groups of 16 position lists (`POSITION_LISTS_BY_PERSON`, `POSITION_LISTS_BY_KEY`), each a `malloc` of
///   capacity × 12 bytes (none: the box, line 0x337, and the game stops) with its count of 12-byte entries read;
///   below `SHORTLIST_LAST_LIST_SAVED` the 16th list of each group is not read and the first dword of the person
///   group's 16th list becomes -1 (both times: see the defect);
/// - the club entries (`CLUB_SHORTLISTS`, 9 bytes each, `G_CLUB_COUNT` of them); each one whose slot pointer was
///   saved non-null gets its reputation copy raised to the club's reputation (`.reputation()`), a new block of 40
///   slots (`malloc(0x5c8)`) and the slots: read as they are above `SHORTLIST_OLD_RECORDS_GONE`, otherwise the 25 old
///   slots converted (`shortlist_convert_old_slots`) and the other 15 emptied;
/// - below `SHORTLIST_LAST_LIST_SAVED` the position lists are rebuilt (`Shortlist_Manager::build_position_lists(0)`).
/// A short read or a missing block: the shortlist_manager.cpp "Error" box (lines 0x2d3, 0x2e3, 0x2ea, 0x2fc, 0x305,
/// 0x30d, 0x31c, 0x329, 0x345, 0x34f, 0x362, 0x36c, 0x378) and the load stops there (the game goes on).
/// Defects of the original: below `SHORTLIST_LAST_LIST_SAVED` the -1 goes twice into the person group's 16th list and
/// the key group's 16th list keeps what `malloc` left; the old block's buffer is not freed when its read is short; the
/// slots' flag bits 6-7 keep what `malloc` left (as in the constructor).
/// Original: `Shortlist_Manager::load` (0x83a9d0, shortlist_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn shortlist_manager_load(this: usize, file: usize) {
    if let Err(row) = shortlist_manager_read(this, file) {
        show_error_box(SHORTLIST_MANAGER_SOURCE, row);
    }
}

/// The body of `shortlist_manager_load`: the line of the error box when a read is short or a block is missing.
pub(crate) unsafe fn shortlist_manager_read(this: usize, z: usize) -> Result<(), u32> {
    use shortlist_manager_fields as f;
    let version = read_i32(G_GAME_DATA_VERSION).unsigned_abs();
    shortlist_read_pairs(this, z)?;
    if version < SHORTLIST_OLD_RECORDS_GONE {
        covered!("rxf_5 shortlist old records");
        let mut old = 0u32;
        if exe::zread(&mut old as *mut u32 as usize, 4, 1, z) != 1 {
            return Err(0x2fc);
        }
        let buffer = exe::crt_malloc(old.wrapping_mul(21));
        if buffer == 0 && old != 0 {
            return Err(0x305);
        }
        if exe::zread(buffer, 0x15, old, z) != old {
            return Err(0x30d);
        }
        exe::crt_free(buffer);
    }
    for k in 0..16 {
        let count = this + f::POSITION_LIST_COUNTS + 4 * k;
        if version <= SHORTLIST_OLD_RECORDS_GONE && k >= 15 {
            write_u32(count, 0);
        } else if exe::zread(count, 4, 1, z) != 1 {
            return Err(0x31c);
        }
    }
    if exe::zread(this + f::POSITION_LIST_CAPACITY, 4, 1, z) != 1 {
        return Err(0x329);
    }
    for group in [f::POSITION_LISTS_BY_PERSON, f::POSITION_LISTS_BY_KEY] {
        for k in 0..16 {
            let list = exe::crt_malloc(read_u32(this + f::POSITION_LIST_CAPACITY).wrapping_mul(12));
            write_usize(this + group + 4 * k, list);
            if list == 0 {
                fatal_error_at(SHORTLIST_MANAGER_SOURCE, 0x337);
            }
            if version < SHORTLIST_LAST_LIST_SAVED && k == 15 {
                // defect of the original: always the person group's 16th list
                write_u32(read_usize(this + f::POSITION_LISTS_BY_PERSON + 4 * 15), u32::MAX);
                continue;
            }
            let count = read_u32(this + f::POSITION_LIST_COUNTS + 4 * k);
            if exe::zread(list, 0xc, count, z) != count {
                return Err(0x345);
            }
        }
    }
    shortlist_read_clubs(this, z, version)?;
    if version < SHORTLIST_LAST_LIST_SAVED {
        covered!("rxf_5 shortlist position lists rebuilt");
        let build: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(SHORTLIST_BUILD_POSITION_LISTS);
        build(this, 0);
    }
    Ok(())
}

/// The shortlist pairs of the save: the count into +8 (the person array's capacity), `PAIRS_BY_CLUB_CAPACITY` and both
/// counts; the person array (+0) and the club array (`PAIRS_BY_CLUB`) allocated (8 bytes a pair; either missing with a
/// non-zero count: line 0x2e3); the pairs read into the person array (short: 0x2ea), copied into the club array and
/// that one sorted (`qsort`, 0x844740).
pub(crate) unsafe fn shortlist_read_pairs(this: usize, z: usize) -> Result<(), u32> {
    use shortlist_manager_fields as f;
    if exe::zread(this + f::PAIRS_BY_PERSON_CAPACITY, 4, 1, z) != 1 {
        return Err(0x2d3);
    }
    let n = read_u32(this + f::PAIRS_BY_PERSON_CAPACITY);
    write_u32(this + f::PAIRS_BY_PERSON_COUNT, n);
    write_u32(this + f::PAIRS_BY_CLUB_COUNT, n);
    write_u32(this + f::PAIRS_BY_CLUB_CAPACITY, n);
    write_usize(this, exe::crt_malloc(n.wrapping_mul(8))); // +0: the person-keyed pair array
    write_usize(this + f::PAIRS_BY_CLUB, exe::crt_malloc(read_u32(this + f::PAIRS_BY_CLUB_CAPACITY).wrapping_mul(8)));
    if (read_usize(this) == 0 || read_usize(this + f::PAIRS_BY_CLUB) == 0) && read_u32(this + f::PAIRS_BY_PERSON_CAPACITY) != 0 {
        return Err(0x2e3);
    }
    let n = read_u32(this + f::PAIRS_BY_PERSON_COUNT);
    if exe::zread(read_usize(this), 8, n, z) != n {
        return Err(0x2ea);
    }
    covered!("rxf_5 shortlist pairs");
    core::ptr::copy_nonoverlapping(read_usize(this) as *const u8, read_usize(this + f::PAIRS_BY_CLUB) as *mut u8, n as usize * 8);
    exe::crt_qsort(read_usize(this + f::PAIRS_BY_CLUB), read_u32(this + f::PAIRS_BY_CLUB_COUNT), 8, SHORTLIST_PAIR_COMPARE); // rxf_5: the club-keyed pairs
    Ok(())
}

/// The club entries of the save (`CLUB_SHORTLISTS`, 9 bytes each, `G_CLUB_COUNT`; short: line 0x34f) and, for each
/// entry whose saved slot pointer is not null, its reputation copy (+5) raised to the club's `.reputation()`, a new
/// block of 40 slots (none: line 0x362) and the slots: 40 slots of 0x25 bytes read as they are above
/// `SHORTLIST_OLD_RECORDS_GONE` (short: 0x36c), otherwise the old slots converted (short: 0x378).
pub(crate) unsafe fn shortlist_read_clubs(this: usize, z: usize, version: u32) -> Result<(), u32> {
    use shortlist_manager_fields as f;
    let clubs = read_u32(G_CLUB_COUNT);
    if exe::zread(read_usize(this + f::CLUB_SHORTLISTS), SHORTLIST_CLUB_ENTRY_SIZE as u32, clubs, z) != read_u32(G_CLUB_COUNT) {
        return Err(0x34f);
    }
    let mut c = 0i32;
    while c < read_i32(G_CLUB_COUNT) {
        let entry = read_usize(this + f::CLUB_SHORTLISTS) + SHORTLIST_CLUB_ENTRY_SIZE * c as usize;
        if read_u32(entry) != 0 {
            let reputation = club_at(c as isize).reputation();
            if read_i16(entry + 5) < reputation { // +5: the club reputation copy
                write_i16(entry + 5, reputation);
            }
            write_usize(entry, exe::crt_malloc((SHORTLIST_SLOTS * SHORTLIST_SLOT_SIZE) as u32));
            if read_usize(entry) == 0 {
                return Err(0x362);
            }
            if version > SHORTLIST_OLD_RECORDS_GONE {
                if exe::zread(read_usize(entry), SHORTLIST_SLOT_SIZE as u32, SHORTLIST_SLOTS as u32, z) != SHORTLIST_SLOTS as u32 {
                    return Err(0x36c);
                }
            } else if !shortlist_convert_old_slots(read_usize(entry), z) {
                return Err(0x378);
            }
        }
        c += 1;
    }
    Ok(())
}

/// Reads a club's 25 old 36-byte slots (false when short) and fills its 40 new slots: each gets +0 = 0xb; the first 25
/// take the old one's position (+0), valuation (+2), price ceiling (+6), price (+0xa), target (+0xe), replacement
/// (+0x12), offer (+0x16), date (+0x1a, `Date::operator=`), index (byte +0x22) and flag bits 0-5 (byte +0x23); the
/// other 15 are emptied (`shortlist_slot_reset`).
pub(crate) unsafe fn shortlist_convert_old_slots(slots: usize, z: usize) -> bool {
    // ponytail: the original builds the 25 local records (Date::Date() at +0x1a) and destroys them with an empty
    // routine; the read fills them whole, and nothing is read from them when it is short
    let mut old = [0u8; SHORTLIST_OLD_SLOT_SIZE * SHORTLIST_OLD_SLOTS];
    if exe::zread(old.as_mut_ptr() as usize, SHORTLIST_OLD_SLOT_SIZE as u32, SHORTLIST_OLD_SLOTS as u32, z) != SHORTLIST_OLD_SLOTS as u32 {
        return false;
    }
    covered!("rxf_5 shortlist old slots converted");
    for j in 0..SHORTLIST_SLOTS {
        let slot = ShortlistSlot(slots + j * SHORTLIST_SLOT_SIZE);
        write_u8(slot.addr(), 0xb); // the slot's +0 byte
        if j >= SHORTLIST_OLD_SLOTS {
            shortlist_slot_reset(slot);
            continue;
        }
        let o = old.as_ptr() as usize + j * SHORTLIST_OLD_SLOT_SIZE;
        let old_flags = read_u8(o + 0x23);
        slot.set_position(read_i16(o));
        slot.set_valuation(read_i32(o + 2));
        slot.set_max_price(read_i32(o + 6));
        slot.set_price(read_i32(o + 0xa));
        slot.set_target_staff_id(read_usize(o + 0xe));
        slot.set_replacement_staff_id(read_u32(o + 0x12));
        slot.set_flags(slot.flags() & !0x1e | old_flags & 0x1e);
        slot.set_offer_ref(read_i32(o + 0x16));
        slot.set_flags(slot.flags() & !1 | old_flags & 1);
        slot.set_index(read_i8(o + 0x22));
        slot.set_flags(slot.flags() & !0x20 | old_flags & 0x20);
        let mut scratch = [0u8; 8];
        exe::date_operator_3d(slot.date_addr(), scratch.as_mut_ptr() as usize, o + 0x1a);
    }
    true
}

/// Comparer for `shortlist_manager_load`: it allocates and reads the save item, so it cannot run twice. Only the
/// original runs; the copy is proven by loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_shortlist_manager_load(this: usize, file: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, file)
}
