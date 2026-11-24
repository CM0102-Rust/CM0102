// ---------- batch %%LEVA%%: saving the injuries, the human managers and the key nation competitions (injury.cpp, human_manager.cpp, key_nation.cpp; sx12_5) ----------

/// Writes the injury state into the `injury.dat` item of a saved game.
///
/// `Injury::save(save)`: no container: the "Error" box (injury.cpp line 0x227). Opens `injury.dat` for writing
/// (`exe::zopen`; failing: the box, line 0x22f). Then, each write checked against its count, in order: the 31-byte
/// physical records (`injury_fields::PHYSICAL_RECORDS`, `G_STAFF_COUNT` of them; line 0x236), the injury slot count
/// (`injury_fields::INJURY_ENTRY_COUNT`; 0x23e) and the 0x30-byte injuries (`injury_fields::INJURY_ENTRIES`; 0x246),
/// the recovery news count (`injury_fields::RECOVERY_NEWS_COUNT`; 0x24e) and, when not zero, its 22-byte entries
/// (`injury_fields::RECOVERY_NEWS_QUEUE`; 0x259), the outbreak count (`injury_fields::OUTBREAK_COUNT`; 0x262) and,
/// when not zero, the 17-byte outbreaks (`injury_fields::OUTBREAKS`; 0x26e), the word count of the match knock list
/// (`injury_fields::MATCH_HIT_LIST`, an LLIST; 0 without a list; 0x27d) and, when not zero, each of its 6-byte
/// items from the first (`exe::llist_get_next`; 0x28f). A failed write shows the box with its line and closes the item
/// (`exe::zclose`); otherwise the item is closed at the end. Defect of the original: a failed write of a knock item
/// (line 0x28f) leaves the item open.
/// Original: `Injury::save` (0x62b2d0, injury.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn injury_save(this: usize, save: usize) {
    const FILE: usize = 0x9e7f28; // "...\injury.cpp"
    if save == 0 {
        show_error_at(FILE, 0x227);
        return;
    }
    let mut z = exe::zopen(save, 0x9e7f68usize, 1, 0x16, 4); // "injury.dat"
    if z == 0 {
        show_error_at(FILE, 0x22f);
        return;
    }
    covered!("sx12_5 injury save");
    let failed = |z: &mut usize, row: u32| {
        show_error_at(FILE, row);
        exe::zclose(z);
    };
    let staff = read_u32(G_STAFF_COUNT);
    if exe::zwrite(read_usize(this + injury_fields::PHYSICAL_RECORDS), 0x1f, staff, z) != read_u32(G_STAFF_COUNT) { // sx12_5: the physical records
        return failed(&mut z, 0x236);
    }
    if exe::zwrite(this + injury_fields::INJURY_ENTRY_COUNT, 4, 1, z) != 1 {
        return failed(&mut z, 0x23e);
    }
    let slots = read_u32(this + injury_fields::INJURY_ENTRY_COUNT);
    if exe::zwrite(read_usize(this + injury_fields::INJURY_ENTRIES), 0x30, slots, z) != read_u32(this + injury_fields::INJURY_ENTRY_COUNT) {
        return failed(&mut z, 0x246);
    }
    if exe::zwrite(this + injury_fields::RECOVERY_NEWS_COUNT, 4, 1, z) != 1 {
        return failed(&mut z, 0x24e);
    }
    let news = read_u32(this + injury_fields::RECOVERY_NEWS_COUNT);
    if news != 0 && exe::zwrite(read_usize(this + injury_fields::RECOVERY_NEWS_QUEUE), 0x16, news, z) != read_u32(this + injury_fields::RECOVERY_NEWS_COUNT) {
        return failed(&mut z, 0x259);
    }
    if exe::zwrite(this + injury_fields::OUTBREAK_COUNT, 4, 1, z) != 1 {
        return failed(&mut z, 0x262);
    }
    let outbreaks = read_u32(this + injury_fields::OUTBREAK_COUNT);
    if outbreaks != 0 && exe::zwrite(read_usize(this + injury_fields::OUTBREAKS), 0x11, outbreaks, z) != read_u32(this + injury_fields::OUTBREAK_COUNT) {
        return failed(&mut z, 0x26e); // sx12_5: the outbreaks
    }
    let list = read_usize(this + injury_fields::MATCH_HIT_LIST);
    let count: u16 = if list != 0 { read_u16(list + llist::COUNT) } else { 0 };
    if exe::zwrite(&count as *const u16 as usize, 2, 1, z) != 1 {
        return failed(&mut z, 0x27d);
    }
    if count != 0 {
        awol_llist_rewind(read_usize(this + injury_fields::MATCH_HIT_LIST));
        let mut item = exe::llist_get_next(read_usize(this + injury_fields::MATCH_HIT_LIST));
        while item != 0 {
            if exe::zwrite(item, 6, 1, z) != 1 {
                show_error_at(FILE, 0x28f); // the original leaves the item open here
                return;
            }
            item = exe::llist_get_next(read_usize(this + injury_fields::MATCH_HIT_LIST));
        }
    }
    exe::zclose(&mut z);
}

/// Writes every human manager record and the counts into the `human_manager.dat` item of a saved game; returns 1, or 0
/// after an error.
///
/// `HUMAN_MANAGER::save(save)`: no container: the "Error" box (human_manager.cpp line 0x916) and 0. Opens
/// `human_manager.dat` for writing (`exe::zopen`; failing: the box, line 0x91e, and 0). Writes the record count (+4;
/// failing: line 0x925, the item closed, 0). For each record (0xa6ef bytes from [this]), unchecked: the first two
/// tactic slots (2 × 0x1ecd), `human_manager_record::ON_HOLIDAY` (4), the dword at +0x9a05 (the two option bytes
/// `CONTROLS_RESERVE_TEAM`, `CONTROLS_ALL_NATIONAL_TEAMS` and two more), the draft contract (+0x9a09, 0x50), the two
/// offer pointers (+0x9a59, +0x9a5d), the dword +0x9a61, `HOLIDAY_STAMP`, `ACTIVE_TACTIC_SLOT` (1), the password
/// (+0x9a6a, 0x32) and its typed entry (+0x9a9c, 0x32), `COMPUTER_CONTROLLED`, the commentary speed (+0x9ad2, 2), the
/// watched fixture (`WATCHED_FIXTURE`, 0x4f), three folder paths, the first two with their four recent files
/// (+0x9ba8, 0x104; +0x9cac, 0x410; +0xa0bc, 0x104; +0xa1c0, 0x410; +0xa5d0, 0x104), the squad view (+0xa6d4, 0xf), the
/// five `HOLIDAY_OPTIONS`, the 16 `SAVED_LINEUP` pairs (8 bytes each), `NICKNAME_COUNT` and the 30-byte `NICKNAMES`.
/// Then the dwords +8 (line 0x94d) and +0xc (line 0x955), each failing with the item closed and 0; at the end, for each
/// record, the last three tactic slots (3 × 0x1ecd from slot 2, +0x3d9a), the item closed, 1.
/// Original: `HUMAN_MANAGER::save` (0x5ff9e0, human_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn human_manager_save(this: usize, save: usize) -> u32 {
    const FILE: usize = 0x9cf11c; // "...\human_manager.cpp"
    if save == 0 {
        show_error_at(FILE, 0x916);
        return 0;
    }
    let mut z = exe::zopen(save, 0x9cf164usize, 1, 0x16, 4); // "human_manager.dat"
    if z == 0 {
        show_error_at(FILE, 0x91e);
        return 0;
    }
    let failed = |z: &mut usize, row: u32| {
        show_error_at(FILE, row);
        exe::zclose(z);
        0
    };
    if exe::zwrite(this + 4, 4, 1, z) != 1 {
        return failed(&mut z, 0x925);
    }
    covered!("sx12_5 human manager save");
    // (offset in the record, item size, item count), in the original's order
    const FIELDS: [(usize, u32, u32); 23] = [
        (0, 0x1ecd, 2), // the first two tactic slots
        (human_manager_record::ON_HOLIDAY, 4, 1),
        (human_manager_record::CONTROLS_RESERVE_TEAM, 4, 1), // the option bytes
        (0x9a09, 0x50, 1), // the draft contract
        (0x9a59, 4, 1), // the bid offer
        (0x9a5d, 4, 1), // the offer to confirm
        (0x9a61, 4, 1),
        (human_manager_record::HOLIDAY_STAMP, 4, 1),
        (human_manager_record::ACTIVE_TACTIC_SLOT, 1, 1),
        (0x9a6a, 1, 0x32), // the password
        (0x9a9c, 1, 0x32), // the typed password
        (human_manager_record::COMPUTER_CONTROLLED, 4, 1),
        (0x9ad2, 2, 1), // the commentary speed
        (human_manager_record::WATCHED_FIXTURE, 0x4f, 1),
        (0x9ba8, 1, 0x104), // a folder path
        (0x9cac, 1, 0x410), // its four recent files
        (0xa0bc, 1, 0x104), // a folder path
        (0xa1c0, 1, 0x410), // its four recent files
        (0xa5d0, 1, 0x104), // a folder path
        (0xa6d4, 0xf, 1), // the squad view
        (human_manager_record::HOLIDAY_OPTIONS, 1, 5),
        (human_manager_record::SAVED_LINEUP, 8, 0x10),
        (human_manager_record::NICKNAME_COUNT, 4, 1),
    ];
    for k in 0..read_i32(this + 4).max(0) as usize {
        let record = human_manager_record_at(this, k);
        for &(at, size, n) in FIELDS.iter() {
            exe::zwrite(record + at, size, n, z); // sx12_5: the record's fields
        }
        exe::zwrite(read_usize(record + human_manager_record::NICKNAMES), 0x1e, read_u32(record + human_manager_record::NICKNAME_COUNT), z);
    }
    if exe::zwrite(this + 8, 4, 1, z) != 1 {
        return failed(&mut z, 0x94d);
    }
    if exe::zwrite(this + 0xc, 4, 1, z) != 1 {
        return failed(&mut z, 0x955);
    }
    for k in 0..read_i32(this + 4).max(0) as usize {
        exe::zwrite(human_manager_record_at(this, k) + 2 * 0x1ecd, 0x1ecd, 3, z); // the last three tactic slots
    }
    exe::zclose(&mut z);
    1
}

/// Tells every competition of each key nation row to save itself to its temporary file; returns 1.
///
/// `key_nation_save_all_comps()`: for each of the 35 rows of `G_LEAGUE_TABLE` with a competition array (+0x10, count at
/// +0xc), each competition that is not null gets its virtual `comp_vtable::SAVE_TO_TEMP_FILE` with 1. Called by the
/// game save (step 0x19 of 0x82e430).
/// Original: `key_nation_save_all_comps` (0x6692f0, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_save_all_comps() -> u32 {
    for k in 0..0x23usize {
        let row = G_LEAGUE_TABLE + 72 * k;
        if read_usize(row + 0x10) == 0 {
            continue; // no competition array
        }
        let mut i = 0;
        while i < read_i32(row + 0xc) {
            let comp = read_usize(read_usize(row + 0x10) + 4 * i as usize);
            if comp != 0 {
                covered!("sx12_5 save competition");
                let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(read_usize(read_usize(comp) + comp_vtable::SAVE_TO_TEMP_FILE));
                f(comp, 1); // sx12_5: the competition's temporary file
            }
            i += 1;
        }
    }
    1
}

/// Comparer for `injury_save`: writes into the save container (cannot run twice); only the original runs, and the
/// copy is proven by the save proof (the `.qst` written with and without the batch, item by item).
pub(crate) unsafe extern "thiscall" fn compare_injury_save(this: usize, save: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, save)
}

/// Comparer for `human_manager_save`: only the original runs (the save container); proven by the save proof.
pub(crate) unsafe extern "thiscall" fn compare_human_manager_save(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, save)
}

/// Comparer for `key_nation_save_all_comps`: only the original runs (each competition writes its temporary file);
/// proven by the save proof.
pub(crate) unsafe extern "C" fn compare_key_nation_save_all_comps() -> u32 {
    original_fn::<unsafe extern "C" fn() -> u32>(%%I2%%)()
}
