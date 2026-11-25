// ---------- rx07_5: the contract manager read from the saved game (contract_manager.cpp) ----------

/// How a block of contract.dat failed: a short read ("Load Game Error") or an allocation ("Memory Allocation Error"),
/// with the line and the text of the box.
#[derive(Clone, Copy)]
pub(crate) enum ContractLoadError {
    Read(u32, usize),
    Memory(u32, usize),
}

/// Shows the box of a failed load: "Load Game Error" (body 0x9af334, "The save game is corrupted...") or "Memory
/// Allocation Error" (body 0x9a7ff8), with the line and text of contract_manager.cpp.
pub(crate) unsafe fn contract_load_error_box(error: ContractLoadError) {
    match error {
        ContractLoadError::Read(line, text) => award_load_error_box(CONTRACT_MANAGER_CPP, line, text, (0x9af3e0, 0x9af334)),
        ContractLoadError::Memory(line, text) => award_load_error_box(CONTRACT_MANAGER_CPP, line, text, (0x9a8108, 0x9a7ff8)),
    }
}

/// Reads `n` records of `size` bytes from the open file `z` into `data`, else the error.
pub(crate) unsafe fn contract_read(data: usize, size: u32, n: u32, z: usize, error: ContractLoadError) -> Result<(), ContractLoadError> {
    if exe::zread(data, size, n, z) == n { Ok(()) } else { Err(error) }
}

/// A counted block of contract.dat: the count read into `capacity` (4 bytes) and copied to `used`; when not zero, an
/// array of `size`-byte records allocated (`exe::crt_malloc`) at `array` and, unless `read_records` is false, read in
/// one go. Returns the count.
pub(crate) unsafe fn contract_read_counted(this: usize, z: usize, (capacity, used, array, size): (usize, usize, usize, u32), read_records: bool,
                                           errors: [ContractLoadError; 3]) -> Result<u32, ContractLoadError> {
    contract_read(this + capacity, 4, 1, z, errors[0])?;
    let n = read_u32(this + capacity);
    write_u32(this + used, n);
    if n == 0 {
        return Ok(0);
    }
    let p = exe::crt_malloc(n.wrapping_mul(size));
    write_usize(this + array, p);
    if p == 0 {
        return Err(errors[1]);
    }
    if read_records {
        contract_read(p, size, n, z, errors[2])?;
    }
    Ok(n)
}

/// The contracts of a save up to 0x3e032 (76-byte records) moved to the 80-byte layout: the first 0x40 bytes as they
/// are (the dates through `exe::date_operator_3d`, +0x39 left alone), the flags (+0x40) and grievances (+0x45)
/// zeroed, and the last bytes of the old record (+0x40 to +0x4b) moved to the new places.
///
/// Defect of the original: the old records are read into the new array, and the conversion reads the old buffer,
/// which was never filled (`old` is fresh from `malloc`); the copy does the same.
pub(crate) unsafe fn convert_old_contracts(this: usize, old: usize) {
    let mut k = 0i32;
    while k < read_i32(this + contract_manager_fields::CONTRACT_CAPACITY) {
        let (n, o) = (read_usize(this).wrapping_add(k as usize * 0x50), old.wrapping_add(k as usize * 0x4c));
        for off in (0..0x1c).step_by(4) {
            write_u32(n + off, read_u32(o + off));
        }
        for off in 0x1c..0x21 {
            write_u8(n + off, read_u8(o + off));
        }
        write_u32(n + contract::RELEASE_FEE, read_u32(o + contract::RELEASE_FEE));
        let mut scratch = [0u32; 2];
        exe::date_operator_3d(n + contract::DATE_STARTED, scratch.as_mut_ptr() as usize, o + contract::DATE_STARTED);
        exe::date_operator_3d(n + contract::CONTRACT_EXPIRES, scratch.as_mut_ptr() as usize, o + contract::CONTRACT_EXPIRES);
        // the original merges the type byte in three bit-field steps (bits 0..5, 6, 7), all from the old byte
        write_u8(n + contract::CONTRACT_TYPE, read_u8(o + contract::CONTRACT_TYPE));
        write_u32(n + contract::PREVIOUS_WAGE, read_u32(o + contract::PREVIOUS_WAGE));
        write_u8(n + contract::SQUAD_NUMBER, read_u8(o + contract::SQUAD_NUMBER));
        write_u16(n + contract::CLUB_APPEARANCES, read_u16(o + contract::CLUB_APPEARANCES));
        write_u8(n + contract::HAPPINESS, read_u8(o + contract::HAPPINESS));
        write_u8(n + contract::UNHAPPY_FLAG, read_u8(o + contract::UNHAPPY_FLAG));
        write_u8(n + contract::MATCHES_LEFT_OUT, read_u8(o + contract::MATCHES_LEFT_OUT));
        write_u32(n + contract::FLAGS, 0);
        write_u8(n + contract::MORALE, read_u8(o + 0x40)); // the old morale
        write_u32(n + contract::GRIEVANCES, 0);
        write_u8(n + contract::LEAVING_ON_BOSMAN, read_u8(o + 0x45)); // the old Bosman flag
        write_u32(n + contract::TRANSFER_ARRANGED_FOR, read_u32(o + 0x46)); // the old transfer arranged for
        write_u8(n + contract::TRANSFER_STATUS, read_u8(o + 0x4a)); // the old transfer status
        // two nibble merges that end with the whole old byte
        write_u8(n + contract::SQUAD_STATUS, read_u8(o + 0x4b)); // the old squad status
        k += 1;
    }
}

/// The main contract (staff state record +0) of person `s`, as the original reads it inline: 0 for a null person, an
/// id past the count or no contract.
pub(crate) unsafe fn loaded_main_contract(this: usize, s: usize) -> usize {
    if s == 0 || read_i32(s) >= read_i32(G_STAFF_COUNT) {
        return 0;
    }
    let i = read_i32(contract_manager_staff_state_record(read_usize(s)));
    if i < 0 { 0 } else { (i as usize).wrapping_mul(80).wrapping_add(read_usize(this)) }
}

/// Each main contract names the person of the table slot that holds it (the slot index, written only when it differs).
pub(crate) unsafe fn relink_main_contracts(this: usize) {
    for k in 0..read_i32(G_STAFF_COUNT).max(0) as usize {
        let s = read_usize(G_STAFF_TABLE).wrapping_add(k * staff::SIZE);
        let c = loaded_main_contract(this, s);
        if c != 0 && read_usize(c) != k {
            covered!("rx07_5 contract relinked");
            write_usize(c, k);
        }
    }
}

/// An offer list of contract.dat: the count (2 bytes, signed); when not zero, a new list (`exe::crt_operator_new` of
/// 0xe and `exe::new_llist`) at `field` and the 111-byte offers appended (`exe::llist_add_tail`). Without memory, the
/// held list stops with `no_list` (the first list goes on with none).
pub(crate) unsafe fn contract_read_offer_list(this: usize, z: usize, field: usize, count_line: u32, offer_line: u32, no_list: Option<ContractLoadError>) -> Result<(), ContractLoadError> {
    let mut count = 0i16;
    contract_read(&mut count as *mut i16 as usize, 2, 1, z, ContractLoadError::Read(count_line, 0x9af480))?; // "cannot read offers count"
    if count == 0 {
        return Ok(());
    }
    let p = exe::crt_operator_new(0xe);
    let list = if p != 0 { exe::new_llist(p) } else { 0 };
    write_usize(this + field, list);
    if list == 0 {
        if let Some(error) = no_list {
            return Err(error);
        }
    }
    let mut offer = [0u8; CONTRACT_OFFER_SIZE as usize];
    let mut k = 0i16;
    while k < count {
        contract_read(offer.as_mut_ptr() as usize, CONTRACT_OFFER_SIZE, 1, z, ContractLoadError::Read(offer_line, 0x9af448))?; // "cannot read offer"
        exe::llist_add_tail(read_usize(this + field), offer.as_ptr() as usize, CONTRACT_OFFER_SIZE);
        k += 1;
    }
    Ok(())
}

/// Whether the C string at `s` is "Shevchenko" (0x9af3f0).
pub(crate) unsafe fn is_shevchenko(s: usize) -> bool {
    let mut k = 0;
    loop {
        let (a, b) = (read_u8(s + k), read_u8(0x9af3f0 + k));
        if a != b {
            return false;
        }
        if a == 0 {
            return true;
        }
        k += 1;
    }
}

/// Saves without the version byte: each current offer of a person whose second name is "Shevchenko" ends on 1 June
/// 2004; any other offer ending before 2000 ends on 1 June 2002.
pub(crate) unsafe fn fix_offer_ends_of_old_save(this: usize) {
    let list = read_usize(this + contract_manager_fields::CONTRACT_OFFERS);
    if list == 0 {
        return;
    }
    write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
    let mut e = exe::llist_get_next(list);
    while e != 0 {
        let name = read_usize(read_usize(G_STAFF_TABLE).wrapping_add(read_usize(e).wrapping_mul(staff::SIZE)) + staff::SECOND_NAME);
        let year = if is_shevchenko(name) { Some(0x7d4) } else if read_u16(e + 0x2f) < 2000 { Some(0x7d2) } else { None }; // the offer's end year
        if let Some(year) = year {
            covered!("rx07_5 old offer end fixed");
            let (mut date, mut scratch) = ([0u16; 4], [0u32; 2]);
            let d = exe::date_date_549f70(&mut date, 1, 5, year, u32::MAX);
            exe::date_operator_3d(e + 0x2d, scratch.as_mut_ptr() as usize, d); // the offer's contract end
        }
        e = exe::llist_get_next(list);
    }
}

/// After the load, for each person in the table: a loan contract (staff state record +4) next to a main one ends no
/// later than the main one (its end copied from the main contract when the main one ends first, years unsigned, days
/// signed); a loan contract without a main one is dropped: the person leaves every club squad (`club::SQUAD`, the clubs
/// before the national teams), the contract slot is freed (person -1) and the record's loan index cleared (-1).
pub(crate) unsafe fn check_loan_contracts(this: usize) {
    for k in 0..read_i32(G_STAFF_COUNT).max(0) as usize {
        let s = read_usize(G_STAFF_TABLE).wrapping_add(k * staff::SIZE);
        if s == 0 {
            continue;
        }
        let id = read_i32(s);
        if id >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        let state = contract_manager_staff_state_record(id as usize);
        let loan_index = read_i32(state + 4);
        if loan_index < 0 {
            continue;
        }
        let loan = (loan_index as usize).wrapping_mul(80).wrapping_add(read_usize(this));
        if loan == 0 {
            continue;
        }
        let main_index = read_i32(state);
        let main = if main_index < 0 { 0 } else { (main_index as usize).wrapping_mul(80).wrapping_add(read_usize(this)) };
        if main != 0 {
            let (my, ly) = (read_u16(main + contract::CONTRACT_EXPIRES_YEAR), read_u16(loan + contract::CONTRACT_EXPIRES_YEAR));
            if my < ly || (my == ly && read_i16(main + contract::CONTRACT_EXPIRES) < read_i16(loan + contract::CONTRACT_EXPIRES)) {
                covered!("rx07_5 loan end cut to the main contract");
                let mut scratch = [0u32; 2];
                exe::date_operator_3d(loan + contract::CONTRACT_EXPIRES, scratch.as_mut_ptr() as usize, main + contract::CONTRACT_EXPIRES);
            }
            continue;
        }
        covered!("rx07_5 loan without a main contract dropped");
        let clubs = read_i32(G_CLUB_COUNT).wrapping_sub(2 * read_i32(G_NATION_COUNT));
        for c in 0..clubs.max(0) as usize {
            for slot in 0..50 {
                let a = read_usize(G_CLUB_TABLE).wrapping_add(c * club::SIZE) + club::SQUAD + slot * 4;
                if read_usize(a) == s {
                    write_usize(a, 0);
                }
            }
        }
        let i = read_i32(state + 4);
        if i != -1 {
            write_i32((i as usize).wrapping_mul(80).wrapping_add(read_usize(this)) as usize, -1);
            write_i32(state + 4, -1);
        }
    }
}

/// The body of the load once contract.dat is open, in the file's order.
pub(crate) unsafe fn contract_load_file(this: usize, z: usize) -> Result<(), ContractLoadError> {
    use ContractLoadError::{Memory, Read};
    let version = read_i32(G_GAME_DATA_VERSION).wrapping_abs();
    if version > 0x3e000 {
        contract_read_counted(this, z, (0x30, contract_manager_fields::TALKS_COUNT, contract_manager_fields::TALKS_RECORDS, TALKS_RECORD_SIZE), true,
            [Read(0x30bf, 0x9af78c), Memory(0x30ca, 0x9af740), Read(0x30d2, 0x9af6f8)])?; // +0x30: the talks capacity
    }
    let new_layout = version > 0x3e032;
    let n = contract_read_counted(this, z, (contract_manager_fields::CONTRACT_CAPACITY, contract_manager_fields::CONTRACT_COUNT, 0, 0x50), new_layout,
        [Read(0x30db, 0x9af6b0), Memory(0x30e8, 0x9af668), Read(0x30f2, 0x9af624)])?;
    if n != 0 {
        if !new_layout {
            covered!("rx07_5 old contracts");
            let old = exe::crt_malloc(n.wrapping_mul(0x4c));
            if old == 0 {
                return Err(Memory(0x30fe, 0x9af5d8));
            }
            contract_read(read_usize(this), 0x4c, n, z, Read(0x3105, 0x9af624))?;
            convert_old_contracts(this, old);
            exe::crt_free(old);
        }
        relink_main_contracts(this);
    }
    contract_read_counted(this, z, (contract_manager_fields::DECISION_CAPACITY, contract_manager_fields::DECISION_COUNT, contract_manager_fields::DECISION_DATES, 0xe), true,
        [Read(0x3141, 0x9af58c), Memory(0x314e, 0x9af548), Read(0x3156, 0x9af508)])?;
    contract_read(this + contract_manager_fields::DECISION_NEXT_INDEX, 4, 1, z, Read(0x315e, 0x9af4c0))?;
    contract_read_offer_list(this, z, contract_manager_fields::CONTRACT_OFFERS, 0x3165, 0x3177, None)?;
    contract_read_offer_list(this, z, contract_manager_fields::HELD_OFFERS, 0x3183, 0x319a, Some(Memory(0x3190, 0x9af3fc)))?;
    let mut version_byte = 0u8;
    if exe::zread(&mut version_byte as *mut u8 as usize, 1, 1, z) == 0 {
        fix_offer_ends_of_old_save(this);
    }
    Ok(())
}

/// Rule: the contract manager read from the saved game's contract.dat (`exe::zopen`(save, "contract.dat", 0, 0x16,
/// 4)), in the order `contract_manager_save_to_disk` writes it: the talks (saves above 0x3e000, `G_GAME_DATA_VERSION`
/// by its absolute value), the contracts (saves up to 0x3e032 in the old 76-byte layout, `convert_old_contracts`;
/// then each main contract relinked to its person), the decision dates and the next index, the two offer lists and the
/// version byte (missing in old saves: `fix_offer_ends_of_old_save`). The file is closed, the loan contracts checked
/// against the main ones (`check_loan_contracts`) and 1 returned. Any failure shows its box ("Load Game Error" or
/// "Memory Allocation Error", lines 0x30b5 to 0x319a) and returns 0.
///
/// The original builds a STAFF_CONTRACT and a Date on its stack for each offer list (0x85e410, 0x54a110) that nothing
/// reads; the copy leaves them out. Defects of the original: after a failure the file is left open (no `zclose`); the
/// old contracts are converted from a buffer never read (`convert_old_contracts`); the copy does the same.
/// Original: `CONTRACT_MANAGER::load_from_disk` (0x4db5c0, contract_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn contract_manager_load_from_disk(this: usize, save: usize) -> u32 {
    let mut z = exe::zopen(save, CONTRACT_DAT, 0, 0x16, 4);
    if z == 0 {
        contract_load_error_box(ContractLoadError::Read(0x30b5, 0x9af7d8)); // "unable to open save game"
        return 0;
    }
    if let Err(error) = contract_load_file(this, z) {
        contract_load_error_box(error);
        return 0;
    }
    exe::zclose(&mut z);
    check_loan_contracts(this);
    covered!("rx07_5 contract.dat loaded");
    1
}

/// Comparer for `contract_manager_load_from_disk`: the save is read once; only the original runs; then the counts must
/// fit their capacities and every main contract must name its person.
pub(crate) unsafe extern "thiscall" fn compare_contract_manager_load_from_disk(this: usize, save: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, save);
    if r == 1 {
        let mut wrong = Vec::new();
        for k in 0..read_i32(G_STAFF_COUNT).max(0) as usize {
            let c = loaded_main_contract(this, read_usize(G_STAFF_TABLE).wrapping_add(k * staff::SIZE));
            if c != 0 && read_usize(c) != k {
                wrong.push(k);
            }
        }
        let fits = read_u32(this + contract_manager_fields::CONTRACT_COUNT) <= read_u32(this + contract_manager_fields::CONTRACT_CAPACITY)
            && read_u32(this + contract_manager_fields::DECISION_NEXT_INDEX) <= read_u32(this + contract_manager_fields::DECISION_COUNT);
        SCOREBOARD.record(fits && wrong.is_empty(), || format!("contract_manager_load_from_disk: counts fit {}, main contracts naming another person {:?}", fits, &wrong[..wrong.len().min(20)]));
    }
    r
}
