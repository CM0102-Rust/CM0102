use super::*;

/// Clears the person's dispute flag and removes her entry from the dispute list.
///
/// `DISPUTE::remove_dispute(person)`: without a person or without the flag table (+4), or with the person's flag at
/// zero: nothing. Otherwise it zeroes the flag and, with the list (+0), moves the cursor back to the start (+0xa = +2)
/// and searches, among the first `count` (+0, word), the entry with the person's id (`next_list_item`): if found, it removes it
/// (`manager_manager_remove`).
/// Original: `DISPUTE::remove_dispute` (0x56a090, dispute.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn remove_dispute(this: usize, p: usize) {
    if p == 0 || read_usize(this + 4) == 0 {
        return;
    }
    let m = (read_usize(this + 4) as isize + read_i32(p) as isize) as *mut u8;
    if *m == 0 {
        return;
    }
    *m = 0;
    let list = read_usize(this);
    if list == 0 {
        return;
    }
    write_u32(list + llist::CURSOR, read_usize(list + 2) as u32);
    let mut k = 0i32;
    while (k as i16 as i32) < read_i16(read_usize(this)) as i32 {
        let in_the = exe::llist_get_next(read_usize(this));
        if in_the == 0 {
            return;
        }
        if read_i32(in_the) == read_i32(p) {
            covered!("cqg_7 left the contest");
            exe::llist_remove(read_usize(this), in_the);
            return;
        }
        k += 1;
    }
}
