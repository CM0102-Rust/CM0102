// ---------- sbA_5: the linked lists kept in a file: building one and appending to it (file_llist.cpp) ----------

/// The source file named in FILE_LLIST's error boxes.
pub(crate) const FILE_LLIST_CPP: usize = 0x9c7540;
/// The "Disk I/O Error" box: (title, body).
pub(crate) const FILE_LLIST_DISK_IO: (usize, usize) = (0x9c7970, 0x9c78b0);
/// The "Disk Space Error" box: (title, body).
pub(crate) const FILE_LLIST_DISK_SPACE: (usize, usize) = (0x9c7774, 0x9c7680);

/// A FILE_LLIST error box, then the file is closed when given (0: left as it is). The text "v%s %s.%s %d (%s)"
/// (version, file_llist.CPP's name and extension, `row`, `text`) at 0xae24d0, the title translated at 0xae28bc, the
/// body (with the text) at 0xae26c4 and the box; the game continues.
pub(crate) unsafe fn file_llist_fail(row: u32, text: usize, (title, body): (usize, usize), file: usize) {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let translate: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(FILE_LLIST_CPP, 0, 0, name.as_mut_ptr(), ext.as_mut_ptr());
    let s = version(name.as_mut_ptr(), ext.as_mut_ptr(), row, text);
    format(0xae24d0, 0x9a8120, s, name.as_ptr(), ext.as_ptr(), row, text);
    translate(0xae28bc, title);
    translate(0xae26c4, body, 0xae24d0usize);
    exe::gui_show_message_box(0xae28bc, 0xae26c4, 0);
    if file != 0 {
        exe::crt_fclose(file);
    }
}

/// Writes a 32-bit offset at the file's position; whether it went.
unsafe fn file_llist_write_offset(file: usize, v: i32) -> bool {
    exe::crt_fwrite(&v as *const i32 as usize, 4, 1, file) == 1
}

/// Builds a list kept in a file: the path copied (`_strdup`) to +0 and the record size to +4. Without memory for the
/// path, the error box (file_llist.CPP 0x2c) and the game ends. Returns `this`.
/// Original: `FILE_LLIST::FILE_LLIST` (0x58e280, file_llist.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn new_file_llist(this: usize, path: usize, record_size: u32) -> usize {
    let copy = exe::at_952f5b(path as *const u8); // _strdup
    write_usize(this, copy); // +0: the file's path
    if copy == 0 {
        fatal_error_at(FILE_LLIST_CPP, 0x2c);
    }
    write_u32(this + 4, record_size); // +4: the record size
    this
}

/// Appends a record to the list kept in the file at +0 (records of the size at +4, each followed by the offset of the
/// next one, -1 at the end), given the offsets of the first and the last node.
///
/// An empty list (both -1): the file is opened to append ("ab"; failing: "Cannot append to file", 0x58, Disk I/O); the
/// record and -1 go at its end, which becomes the first and the last offset. A first node and no last: the record
/// overwrites the first node ("r+b") and the last becomes the first. Otherwise, after the last record ("r+b") the next
/// offset is read: a next node that is already there (not negative) gets the record and becomes the last; else the
/// record and -1 go at the end of the file, the last node's next offset points to it and it becomes the last. Each
/// failure shows its box (Disk Space; the read of the next offset Disk I/O) and closes the file, except a failed open.
/// Defect of the original: when the seek to the last node fails (0xa8) the file is left open.
/// Original: `FILE_LLIST::add_node` (0x58e340, file_llist.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn file_llist_add_node(this: usize, first: *mut i32, last: *mut i32, record: usize) {
    let (path, size) = (read_usize(this), read_u32(this + 4));
    let ds = FILE_LLIST_DISK_SPACE;
    if *first == -1 && *last == -1 {
        let f = exe::crt_fopen(path, 0x9c79b0); // "ab"
        if f == 0 {
            return file_llist_fail(0x58, 0x9c7980, FILE_LLIST_DISK_IO, 0); // "Cannot append to file"
        }
        exe::crt_fseek(f, 0, 2);
        let end = exe::crt_ftell(f);
        *last = end;
        *first = end;
        if exe::crt_fwrite(record, size, 1, f) != 1 {
            return file_llist_fail(0x65, 0x9c7880, ds, f); // "Cannot write record"
        }
        if !file_llist_write_offset(f, -1) {
            return file_llist_fail(0x6e, 0x9c784c, ds, f); // "Cannot write next offset"
        }
        exe::crt_fclose(f);
        return;
    }
    let f = exe::crt_fopen(path, 0x9c7848); // "r+b"
    if *last == -1 {
        if f == 0 {
            return file_llist_fail(0x7f, 0x9c781c, ds, 0); // "Cannot open file"
        }
        if exe::crt_fseek(f, *first, 0) != 0 {
            return file_llist_fail(0x86, 0x9c77e8, ds, f); // "Cannot seek to first node"
        }
        if exe::crt_fwrite(record, size, 1, f) != 1 {
            return file_llist_fail(0x8e, 0x9c77bc, ds, f); // "Cannot write data"
        }
        *last = *first;
        exe::crt_fclose(f);
        return;
    }
    if f == 0 {
        return file_llist_fail(0xa1, 0x9c781c, ds, 0); // "Cannot open file"
    }
    if exe::crt_fseek(f, *last, 0) != 0 {
        return file_llist_fail(0xa8, 0x9c7788, ds, 0); // "Cannot seek to last offset" (the defect: left open)
    }
    exe::crt_fseek(f, size as i32, 1);
    let mut next = -1i32;
    if exe::crt_fread(&mut next as *mut i32 as usize, 4, 1, f) != 1 {
        return file_llist_fail(0xb1, 0x9c764c, FILE_LLIST_DISK_IO, f); // "Could not read next offset"
    }
    if next > -1 {
        covered!("sbA_5 add_node reuses the next slot");
        if exe::crt_fseek(f, next, 0) != 0 {
            return file_llist_fail(0xbc, 0x9c7614, ds, f); // "Could not seek to next record"
        }
        if exe::crt_fwrite(record, size, 1, f) != 1 {
            return file_llist_fail(0xc4, 0x9c75e4, ds, f); // "Could not write data"
        }
        *last = next;
        exe::crt_fclose(f);
        return;
    }
    exe::crt_fseek(f, 0, 2);
    let end = exe::crt_ftell(f);
    if exe::crt_fwrite(record, size, 1, f) != 1 {
        return file_llist_fail(0xdb, 0x9c75e4, ds, f); // "Could not write data"
    }
    if !file_llist_write_offset(f, -1) {
        return file_llist_fail(0xe2, 0x9c75ac, ds, f); // "Could not write next offset"
    }
    exe::crt_fseek(f, (*last).wrapping_add(size as i32), 0);
    if !file_llist_write_offset(f, end) {
        return file_llist_fail(0xec, 0x9c756c, ds, f); // "Could not write offset of new record"
    }
    *last = end;
    exe::crt_fclose(f);
}

/// Appends every record of an in-memory list (`LLIST`, from its head) to the list kept in the file, like
/// `file_llist_add_node` one by one, but with the file open once.
///
/// An empty in-memory list does nothing. An empty file list: appended ("ab"; failing, 0x132), each record followed by
/// the offset of the next (its position + 4, -1 after the last), the first offset the old end and the last the last
/// record's. A first node and no last: the in-memory head overwrites the first node, the last becomes the first, the
/// head is removed from the in-memory list (`LLIST::remove`) and the rest goes in again. Otherwise, for each record:
/// after the last node the next offset is read; a next node that is there (not -1) gets the record, becomes the last
/// and its next offset is set to -1; else the record and -1 go at the end and the last node points to it. Failures as
/// in `file_llist_add_node` (Disk Space; the read of the next offset Disk I/O), closing the file except a failed open.
/// Defect of the original: when the seek to the last node fails (0x1a3) the file is left open.
/// Original: `FILE_LLIST::add_node_list` (0x58e9b0, file_llist.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn file_llist_add_node_list(this: usize, first: *mut i32, last: *mut i32, list: usize) {
    write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
    if exe::llist_get_next(list) == 0 {
        return;
    }
    write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
    let (path, size) = (read_usize(this), read_u32(this + 4));
    let ds = FILE_LLIST_DISK_SPACE;
    if *first == -1 && *last == -1 {
        let f = exe::crt_fopen(path, 0x9c79b0); // "ab"
        if f == 0 {
            return file_llist_fail(0x132, 0x9c7980, ds, 0); // "Cannot append to file"
        }
        exe::crt_fseek(f, 0, 2);
        *first = exe::crt_ftell(f);
        let mut node = exe::llist_get_next(list);
        while node != 0 {
            *last = exe::crt_ftell(f);
            if exe::crt_fwrite(node, size, 1, f) != 1 {
                return file_llist_fail(0x146, 0x9c7880, ds, f); // "Cannot write record"
            }
            let after = exe::llist_get_next(list);
            let next = if after != 0 { exe::crt_ftell(f) + 4 } else { -1 };
            if !file_llist_write_offset(f, next) {
                return file_llist_fail(0x15d, 0x9c784c, ds, f); // "Cannot write next offset"
            }
            node = after;
        }
        exe::crt_fclose(f);
        return;
    }
    if *last == -1 {
        let f = exe::crt_fopen(path, 0x9c7848); // "r+b"
        if f == 0 {
            return file_llist_fail(0x16f, 0x9c781c, ds, 0); // "Cannot open file"
        }
        if exe::crt_fseek(f, *first, 0) != 0 {
            return file_llist_fail(0x176, 0x9c77e8, ds, f); // "Cannot seek to first node"
        }
        let node = exe::llist_get_next(list);
        if exe::crt_fwrite(node, size, 1, f) != 1 {
            return file_llist_fail(0x181, 0x9c77bc, ds, f); // "Cannot write data"
        }
        *last = *first;
        exe::crt_fclose(f);
        exe::llist_remove(list, node);
        return file_llist_add_node_list(this, first, last, list);
    }
    let f = exe::crt_fopen(path, 0x9c7848); // "r+b"
    if f == 0 {
        return file_llist_fail(0x199, 0x9c781c, ds, 0); // "Cannot open file"
    }
    let mut node = exe::llist_get_next(list);
    while node != 0 {
        if exe::crt_fseek(f, *last, 0) != 0 {
            return file_llist_fail(0x1a3, 0x9c7788, ds, 0); // "Cannot seek to last offset" (the defect: left open)
        }
        exe::crt_fseek(f, size as i32, 1);
        let mut next = -1i32;
        if exe::crt_fread(&mut next as *mut i32 as usize, 4, 1, f) != 1 {
            return file_llist_fail(0x1ac, 0x9c764c, FILE_LLIST_DISK_IO, f); // "Could not read next offset"
        }
        if next != -1 {
            if exe::crt_fseek(f, next, 0) != 0 {
                return file_llist_fail(0x1b7, 0x9c7614, ds, f); // "Could not seek to next record"
            }
            if exe::crt_fwrite(node, size, 1, f) != 1 {
                return file_llist_fail(0x1bf, 0x9c75e4, ds, f); // "Could not write data"
            }
            *last = next;
            if !file_llist_write_offset(f, -1) {
                return file_llist_fail(0x1cb, 0x9c75ac, ds, f); // "Could not write next offset"
            }
        } else {
            exe::crt_fseek(f, 0, 2);
            let end = exe::crt_ftell(f);
            if exe::crt_fwrite(node, size, 1, f) != 1 {
                return file_llist_fail(0x1dc, 0x9c75e4, ds, f); // "Could not write data"
            }
            if !file_llist_write_offset(f, -1) {
                return file_llist_fail(0x1e3, 0x9c75ac, ds, f); // "Could not write next offset"
            }
            exe::crt_fseek(f, (*last).wrapping_add(size as i32), 0);
            if !file_llist_write_offset(f, end) {
                return file_llist_fail(0x1ed, 0x9c756c, ds, f); // "Could not write offset of new record"
            }
            *last = end;
        }
        node = exe::llist_get_next(list);
    }
    exe::crt_fclose(f);
}

/// The `size` bytes of the file list's record at `offset`, read back ("rb"); empty when it cannot be read.
pub(crate) unsafe fn file_llist_read_back(this: usize, offset: i32) -> Vec<u8> {
    let size = read_u32(this + 4) as usize;
    let f = exe::crt_fopen(read_usize(this), 0x98f928); // "rb"
    if f == 0 {
        return Vec::new();
    }
    let mut v = vec![0u8; size];
    let ok = exe::crt_fseek(f, offset, 0) == 0 && exe::crt_fread(v.as_mut_ptr() as usize, size as u32, 1, f) == 1;
    exe::crt_fclose(f);
    if ok { v } else { Vec::new() }
}

/// Comparer for `new_file_llist`: only the original runs (it copies the path); it checks the copy and the size.
pub(crate) unsafe extern "thiscall" fn compare_new_file_llist(this: usize, path: usize, record_size: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(%%I0%%)(this, path, record_size);
    let copy = read_usize(this);
    let same = copy != 0 && core::ffi::CStr::from_ptr(copy as *const core::ffi::c_char) == core::ffi::CStr::from_ptr(path as *const core::ffi::c_char);
    SCOREBOARD.record(r == this && same && read_u32(this + 4) == record_size,
        || format!("new_file_llist(size {}): path copied {}, size {}", record_size, same, read_u32(this + 4)));
    r
}

/// Comparer for `file_llist_add_node`: the file changes (it does not run twice), so only the original runs; it checks
/// that the record it was given is in the file at the new last offset.
pub(crate) unsafe extern "thiscall" fn compare_file_llist_add_node(this: usize, first: *mut i32, last: *mut i32, record: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, *mut i32, *mut i32, usize)>(%%I1%%)(this, first, last, record);
    let wanted = core::slice::from_raw_parts(record as *const u8, read_u32(this + 4) as usize);
    let found = file_llist_read_back(this, *last);
    SCOREBOARD.record(found == wanted, || format!("file_llist_add_node: the record at the last offset {} is not the one given", *last));
}

/// Comparer for `file_llist_add_node_list`: only the original runs (the file changes and the head may be removed from
/// the in-memory list); it checks that the in-memory list's last record is in the file at the new last offset.
pub(crate) unsafe extern "thiscall" fn compare_file_llist_add_node_list(this: usize, first: *mut i32, last: *mut i32, list: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut i32, *mut i32, usize)>(%%I2%%);
    let size = read_u32(this + 4) as usize;
    write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
    let mut tail = Vec::new();
    loop {
        let n = exe::llist_get_next(list);
        if n == 0 {
            break;
        }
        tail = core::slice::from_raw_parts(n as *const u8, size).to_vec();
    }
    o(this, first, last, list);
    if !tail.is_empty() {
        let found = file_llist_read_back(this, *last);
        SCOREBOARD.record(found == tail, || format!("file_llist_add_node_list: the last record at offset {} is not the list's last", *last));
    }
}
