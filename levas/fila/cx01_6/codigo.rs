// ---------- cx01_6: the file search of the game (comp_util.cpp) ----------

/// C runtime `_findfirst(pattern, data)` (cdecl): the search handle, −1 when nothing matches.
const FN_CRT_FINDFIRST: usize = 0x9467ce;
/// C runtime `_findnext(handle, data)` (cdecl): 0, −1 when there is no further entry.
const FN_CRT_FINDNEXT: usize = 0x946898;
/// C runtime `_findclose(handle)` (cdecl).
const FN_CRT_FINDCLOSE: usize = 0x94695d;

/// The C runtime's `_finddata_t` (0x118 bytes): the attributes +0, the size +0x10 and the name +0x14 (260 bytes).
const FINDDATA_SIZE: usize = 0x118;
const FINDDATA_ATTRIB: usize = 0x0;
const FINDDATA_FILE_SIZE: usize = 0x10;
const FINDDATA_NAME: usize = 0x14;

/// The game's search state (0x114 bytes): the handle +0, the size +4, the attributes +8, the name
/// (`file_find_state::FILE_NAME`, 255 characters and the terminator at +0x10f) and the kind wanted +0x110.
const FIND_HANDLE: usize = 0x0;
const FIND_SIZE: usize = 0x4;
const FIND_ATTRIB: usize = 0x8;
const FIND_KIND: usize = 0x110;

/// Whether a found entry is of the kind wanted: kind 0x10 takes directories (attribute bit 0x10), kind 0 takes plain
/// files; any other kind takes nothing.
fn find_kind_matches(kind: u32, attrib: u32) -> bool {
    (kind == 0x10 && attrib & 0x10 != 0) || (kind == 0 && attrib & 0x10 == 0)
}

/// Steps the open search `state` with `_findnext` until an entry of the kind wanted (the one already in `data`
/// first) and stores its size, attributes and name (`exe::at_9466d0`, the C runtime strncpy, 255 characters and a
/// terminator). Returns 0, or −1 when the search runs out (the handle stays open).
unsafe fn find_take_matching(state: usize, data: usize) -> i32 {
    let next: unsafe extern "C" fn(i32, usize) -> i32 = core::mem::transmute(FN_CRT_FINDNEXT);
    while !find_kind_matches(read_u32(state + FIND_KIND), read_u32(data + FINDDATA_ATTRIB)) {
        if next(read_i32(state + FIND_HANDLE), data) == -1 {
            return -1;
        }
    }
    write_u32(state + FIND_ATTRIB, read_u32(data + FINDDATA_ATTRIB));
    write_u32(state + FIND_SIZE, read_u32(data + FINDDATA_FILE_SIZE));
    exe::at_9466d0(state + file_find_state::FILE_NAME, (data + FINDDATA_NAME) as *const u8, 0xff);
    write_u8(state + file_find_state::FILE_NAME + 0xff, 0);
    0
}

/// Starts a file search for `pattern` that wants `kind` (0x10 directories, 0 plain files) in the search state `state`:
/// the kind is stored, the handle of `_findfirst` too; −1 when nothing matches, otherwise the first entry of the kind
/// wanted (`find_take_matching`), 0, or −1 when there is none.
/// Original: `file_find_first` (0x4c0aa0, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn file_find_first(pattern: usize, state: usize, kind: u32) -> i32 {
    let first: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(FN_CRT_FINDFIRST);
    let mut data = [0u8; FINDDATA_SIZE];
    write_u32(state + FIND_KIND, kind);
    let handle = first(pattern, data.as_mut_ptr() as usize);
    write_i32(state + FIND_HANDLE, handle);
    if handle == -1 {
        covered!("cx01_6 find first: nothing");
        return -1;
    }
    covered!("cx01_6 find first");
    find_take_matching(state, data.as_mut_ptr() as usize)
}

/// Steps the search in `state` to the next entry of the kind wanted: −1 for a closed search (handle −1); otherwise
/// `_findnext` (into a record first filled with the stored size, attributes and 256 bytes from the name, which the
/// call overwrites), −1 when there is no further entry, else the first entry of the kind wanted (`find_take_matching`).
/// Original: `file_find_next` (0x4c0b50, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn file_find_next(state: usize) -> i32 {
    if read_i32(state + FIND_HANDLE) == -1 {
        return -1;
    }
    let next: unsafe extern "C" fn(i32, usize) -> i32 = core::mem::transmute(FN_CRT_FINDNEXT);
    let mut data = [0u8; FINDDATA_SIZE];
    let d = data.as_mut_ptr() as usize;
    write_u32(d + FINDDATA_FILE_SIZE, read_u32(state + FIND_SIZE));
    write_u32(d + FINDDATA_ATTRIB, read_u32(state + FIND_ATTRIB));
    exe::at_9466d0(d + FINDDATA_NAME, (state + file_find_state::FILE_NAME) as *const u8, 0x100);
    if next(read_i32(state + FIND_HANDLE), d) == -1 {
        covered!("cx01_6 find next: no more");
        return -1;
    }
    covered!("cx01_6 find next");
    find_take_matching(state, d)
}

/// Closes the search in `state`: −1 for one already closed (handle −1); otherwise `_findclose`, the handle −1 and 0.
/// Original: `file_find_close` (0x4c0c10, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn file_find_close(state: usize) -> i32 {
    let handle = read_i32(state + FIND_HANDLE);
    if handle == -1 {
        return -1;
    }
    covered!("cx01_6 find close");
    let close: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_CRT_FINDCLOSE);
    close(handle);
    write_i32(state + FIND_HANDLE, -1);
    0
}

/// Comparer for `file_find_first`: the original into the caller's state and the copy into a local one (closed
/// afterwards); the return, the size, the attributes, the name and the kind must match (the handles differ).
pub(crate) unsafe extern "C" fn compare_file_find_first(pattern: usize, state: usize, kind: u32) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> i32>(%%I0%%)(pattern, state, kind);
    let mut mine = [0u8; 0x114];
    let m = mine.as_mut_ptr() as usize;
    let n = file_find_first(pattern, m, kind);
    let name = |p: usize| { let s = core::slice::from_raw_parts(p as *const u8, 0x100); s[..s.iter().position(|&c| c == 0).unwrap_or(0x100)].to_vec() };
    let found = |s: usize| (read_u32(s + FIND_SIZE), read_u32(s + FIND_ATTRIB), name(s + file_find_state::FILE_NAME), read_u32(s + FIND_KIND));
    let (a, b) = (found(state), found(m));
    // the fields after a failed search are whatever was there before: only the return counts
    SCOREBOARD.record(r == n && (r != 0 || a == b), || format!("file_find_first({:?}, kind {:#x}): original {} {:x?} rust {} {:x?}", String::from_utf8_lossy(&c_string_bytes(pattern)), kind, r, a, n, b));
    if read_i32(m + FIND_HANDLE) != -1 {
        let close: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_CRT_FINDCLOSE);
        close(read_i32(m + FIND_HANDLE));
    }
    r
}

/// Comparer for `file_find_next`: it advances the open search (cannot run twice); only the original runs.
pub(crate) unsafe extern "C" fn compare_file_find_next(state: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize) -> i32>(%%I1%%)(state)
}

/// Comparer for `file_find_close`: it closes the search (cannot run twice); only the original runs.
pub(crate) unsafe extern "C" fn compare_file_find_close(state: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize) -> i32>(%%I2%%)(state)
}
