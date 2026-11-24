// ---------- sx05b_8: building the AWOL state and loading it from a saved game (awol.cpp) ----------

/// The AWOL object's three record lists (`LLIST`, 0xe bytes each) and the size of their records: the absences (+0,
/// 12 bytes: staff id and the return date), the missed trainings (+4, 13 bytes) and the fitness incidents (+8, 17
/// bytes); the per-staff flag table is `awol_fields::PERSON_FLAGS`.
pub(crate) const AWOL_LISTS: [(usize, u32); 3] = [(0x0, 0xc), (0x4, 0xd), (0x8, 0x11)];
/// "E:\dev\CM3\cm3 00-01\cm3\code\awol.cpp" (the error boxes) and "awol.dat" (its file in the save).
pub(crate) const AWOL_CPP: usize = 0x9880f4;
pub(crate) const AWOL_DAT: usize = 0x9880e8;
/// `AWOL::load(save)` (0x418670, thiscall, ret 4: al nonzero when read), through its hook.
const FN_AWOL_LOAD: usize = 0x418670;

/// A new empty record list (`exe::new_llist` in 0xe bytes of `exe::crt_operator_new`), 0 when out of memory.
pub(crate) unsafe fn awol_new_list() -> usize {
    let p = exe::crt_operator_new(0xe);
    if p != 0 { exe::new_llist(p) } else { 0 }
}

/// Builds the AWOL state: everything 0; with a save, `AWOL::load`, and nothing more when it reads. Without a save, or
/// when the load fails (then its flag table and first list are freed first), a zeroed flag byte per staff member
/// (`exe::crt_calloc`(`G_STAFF_COUNT`, 1)) and three new empty lists. Returns this.
/// Defect of the original: a failed load frees only the first list; the second and third (when it built them) leak.
/// Original: `AWOL::AWOL` (0x418440, awol.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn awol_construct(this: usize, save: usize) -> usize {
    write_usize(this + awol_fields::PERSON_FLAGS, 0);
    for (off, _) in AWOL_LISTS {
        write_usize(this + off, 0);
    }
    if save != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_AWOL_LOAD);
        if load(this, save) != 0 {
            return this;
        }
        covered!("sx05b_8 awol load failed");
        let flags = read_usize(this + awol_fields::PERSON_FLAGS);
        if flags != 0 {
            exe::crt_free(flags);
            write_usize(this + awol_fields::PERSON_FLAGS, 0);
        }
        let first = read_usize(this);
        if first != 0 {
            exe::drop_llist(first);
            exe::crt_operator_delete(first);
            write_usize(this, 0);
        }
    }
    write_usize(this + awol_fields::PERSON_FLAGS, exe::crt_calloc(read_u32(G_STAFF_COUNT), 1));
    for (off, _) in AWOL_LISTS {
        write_usize(this + off, awol_new_list());
    }
    this
}

/// Reads one count or record from awol.dat; false on a short read.
unsafe fn awol_read(dest: usize, size: u32, z: usize) -> bool {
    exe::zread(dest, size, 1, z) == 1
}

/// Loads the AWOL state from a save's awol.dat (`exe::zopen`(save, "awol.dat", 0, 0x16, 4); none, 0): the staff count
/// (zero counts as a failure), the flag table (`exe::crt_malloc` of that many bytes, read in one go), the absences
/// (count, then each 12-byte record appended to a new list), the missed trainings (13 bytes) and, only in saves of a
/// version above 0x3e005 (`G_GAME_DATA_VERSION`, by its absolute value), the fitness incidents (17 bytes; older saves
/// get an empty third list). Any short read closes the file and gives 0; otherwise the file is closed and 1.
/// Original: `AWOL::load` (0x418670, awol.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn awol_load(this: usize, save: usize) -> u8 {
    if save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, AWOL_DAT, 0, 0x16, 4);
    if z == 0 {
        return 0;
    }
    let ok = awol_load_lists(this, z);
    exe::zclose(&mut z);
    ok as u8
}

/// The body of `awol_load` once the file is open.
unsafe fn awol_load_lists(this: usize, z: usize) -> bool {
    let mut n = 0u32;
    let np = &mut n as *mut u32 as usize;
    if !awol_read(np, 4, z) || n == 0 {
        return false;
    }
    let flags = exe::crt_malloc(n);
    write_usize(this + awol_fields::PERSON_FLAGS, flags);
    if flags == 0 || exe::zread(flags, 1, n, z) != n || !awol_read(np, 4, z) {
        return false;
    }
    let version = read_i32(G_GAME_DATA_VERSION).unsigned_abs();
    for (k, &(off, size)) in AWOL_LISTS.iter().enumerate() {
        if k == 2 {
            write_usize(this + off, awol_new_list());
            if version <= 0x3e005 {
                return true;
            }
            covered!("sx05b_8 fitness incidents read");
            if !awol_read(np, 4, z) {
                return false;
            }
        } else {
            write_usize(this + off, awol_new_list());
        }
        let mut record = [0u8; 0x11];
        for _ in 0..(n as i32).max(0) {
            if !awol_read(record.as_mut_ptr() as usize, size, z) {
                return false;
            }
            exe::llist_add_tail(read_usize(this + off), record.as_ptr() as usize, size);
        }
        if k == 0 && !awol_read(np, 4, z) {
            return false;
        }
    }
    true
}

/// Comparer for `awol_construct`: it allocates (and may read the save): only the original runs; then the flag table
/// and the three lists must exist.
pub(crate) unsafe extern "thiscall" fn compare_awol_construct(this: usize, save: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, save);
    let ok = r == this && read_usize(this + awol_fields::PERSON_FLAGS) != 0 && AWOL_LISTS.iter().all(|&(off, _)| read_usize(this + off) != 0);
    SCOREBOARD.record(ok, || format!("awol_construct(save {:#x}): flags {:#x} lists {:x?}", save, read_usize(this + awol_fields::PERSON_FLAGS),
        AWOL_LISTS.iter().map(|&(off, _)| read_usize(this + off)).collect::<Vec<_>>()));
    r
}

/// Comparer for `awol_load`: the save is read once; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_awol_load(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I1%%)(this, save)
}
