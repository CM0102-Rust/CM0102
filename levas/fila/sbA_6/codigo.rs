// ---------- sbA_6: the linked lists kept in a file: forgetting the last node and reading a chain (file_llist.cpp) ----------

/// Marks a file list as having no last node: -1 into the last offset (the first is not read).
/// Original: `FILE_LLIST::reset_last_offset` (0x58f110, file_llist.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn file_llist_reset_last_offset(_this: usize, _first: *mut i32, last: *mut i32) {
    *last = -1;
}

/// Reads a chain of records from the file into a new in-memory list (`LLIST`, 0xe bytes, `operator new`): from the
/// first offset, each record (`LLIST::add_tail` with the record size) and then its next offset, until -1 or the record
/// at the last offset; a single-node chain (first = last) reads only the first. A first offset of -1 gives 0. A record
/// buffer that cannot be allocated: "Could not allocate memory" (0x21f, Memory Allocation Error) and 0; the file that
/// does not open ("rb"; 0x22a) or a record that cannot be read (0x236, 0x250): the Disk I/O box, the file closed, the
/// list destroyed and freed, and 0. The buffer is freed at the end.
/// Original: `FILE_LLIST::get_nodes` (0x58f120, file_llist.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn file_llist_get_nodes(this: usize, first: i32, last: i32) -> usize {
    if first == -1 {
        return 0;
    }
    let size = read_u32(this + 4);
    let buf = exe::crt_malloc(size);
    if buf == 0 {
        file_llist_fail(0x21f, 0x9c7a14, (0x9a8108, 0x9a7ff8), 0); // "Could not allocate memory"
        return 0;
    }
    let mem = exe::crt_operator_new(0xe);
    let list = if mem != 0 { exe::new_llist(mem) } else { 0 };
    let give_up = || {
        if list != 0 {
            exe::drop_llist(list);
            exe::crt_operator_delete(list);
        }
        exe::crt_free(buf);
        0
    };
    let f = exe::crt_fopen(read_usize(this), 0x98f928); // "rb"
    if f == 0 {
        file_llist_fail(0x22a, 0x9c79e4, FILE_LLIST_DISK_IO, 0); // "Could not open file"
        return give_up();
    }
    exe::crt_fseek(f, first, 0);
    if exe::crt_fread(buf, size, 1, f) != 1 {
        file_llist_fail(0x236, 0x9c79b4, FILE_LLIST_DISK_IO, f); // "Could not read data"
        return give_up();
    }
    // simplification: the original adds the size as a word with leftovers above it (add_tail reads the word)
    exe::llist_add_tail(list, buf, size & 0xffff);
    // simplification: a failed read of the offset leaves the original's local unset; the copy reads -1
    let mut next = -1i32;
    exe::crt_fread(&mut next as *mut i32 as usize, 4, 1, f);
    while next != -1 && first != last {
        exe::crt_fseek(f, next, 0);
        if exe::crt_fread(buf, size, 1, f) != 1 {
            file_llist_fail(0x250, 0x9c79b4, FILE_LLIST_DISK_IO, f); // "Could not read data"
            return give_up();
        }
        exe::llist_add_tail(list, buf, size & 0xffff);
        if next == last {
            break;
        }
        exe::crt_fread(&mut next as *mut i32 as usize, 4, 1, f);
    }
    exe::crt_fclose(f);
    exe::crt_free(buf);
    list
}

/// Comparer for `file_llist_reset_last_offset`: the original on the offset, the copy on a local copy of it.
pub(crate) unsafe extern "thiscall" fn compare_file_llist_reset_last_offset(this: usize, first: *mut i32, last: *mut i32) {
    let mut copy = *last;
    original_fn::<unsafe extern "thiscall" fn(usize, *mut i32, *mut i32)>(%%I0%%)(this, first, last);
    file_llist_reset_last_offset(this, first, &mut copy);
    SCOREBOARD.record(copy == *last, || format!("file_llist_reset_last_offset: original {} rust {}", *last, copy));
}

/// Comparer for `file_llist_get_nodes`: both read the file; the lists must hold the same records in the same order
/// (the copy's list is then destroyed and freed).
pub(crate) unsafe extern "thiscall" fn compare_file_llist_get_nodes(this: usize, first: i32, last: i32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32) -> usize>(%%I1%%)(this, first, last);
    let n = file_llist_get_nodes(this, first, last);
    let size = read_u32(this + 4) as usize;
    let records = |l: usize| {
        let mut v = Vec::new();
        if l != 0 {
            let cursor = read_u32(l + llist::CURSOR);
            write_u32(l + llist::CURSOR, read_u32(l + llist::HEAD));
            loop {
                let d = exe::llist_get_next(l);
                if d == 0 {
                    break;
                }
                v.push(core::slice::from_raw_parts(d as *const u8, size).to_vec());
            }
            write_u32(l + llist::CURSOR, cursor);
        }
        v
    };
    let (ro, rn) = (records(r), records(n));
    SCOREBOARD.record((r == 0) == (n == 0) && ro == rn,
        || format!("file_llist_get_nodes(first {}, last {}): original {} records, rust {}", first, last, ro.len(), rn.len()));
    if n != 0 {
        exe::drop_llist(n);
        exe::crt_operator_delete(n);
    }
    r
}
