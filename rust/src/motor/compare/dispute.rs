use super::*;

// The comparers of the routines in ../dispute.rs.

/// Comparer for `remove_dispute`.
pub(crate) unsafe extern "thiscall" fn compare_remove_dispute(this: usize, p: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REMOVE_DISPUTE);
    if p == 0 || read_usize(this + 4) == 0 {
        return o(this, p);
    }
    let m = read_usize(this + 4).wrapping_add(read_i32(p) as isize as usize);
    let list = Llist(read_usize(this));
    if read_u8(m) == 0 {
        let mut reg = vec![(m, 1)];
        if !list.is_null() {
            reg.push((list.addr(), 0xe));
        }
        let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, p), || remove_dispute(this, p));
        SCOREBOARD.record(igual, || format!("remove_dispute(person {:x}) {:x?} x {:x?}", p, mo.0, mr.0));
        return;
    }
    let mut found = 0i16;
    let count_before = if !list.is_null() { read_i16(list.addr()) } else { 0 };
    if !list.is_null() {
        let cursor = list.cursor();
        list.set_cursor((list.head() as u32) as usize);
        for _ in 0..count_before.max(0) {
            let in_the = exe::llist_get_next(list);
            if in_the == 0 {
                break;
            }
            if read_i32(in_the) == read_i32(p) {
                found = 1;
                break;
            }
        }
        list.set_cursor((cursor as u32) as usize);
    }
    o(this, p);
    let count_after = if !list.is_null() { read_i16(list.addr()) } else { 0 };
    SCOREBOARD.record(read_u8(m) == 0 && count_after == count_before - found,
        || format!("remove_dispute(person {:x}) count {} -> {} (found {})", p, count_before, count_after, found));
}
