use super::*;

// The comparers of the routines in ../dispute.rs.
// [pt] Os comparadores das rotinas de ../dispute.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `remove_dispute`.
///
/// [pt] Comparador: sem a marca (nada a fazer), lado a lado na marca e no cursor da lista. Com a marca, só a original com o
/// plano: a marca zerada e a conta da lista uma a menos quando a pessoa está nela (a procura feita antes, com o cursor
/// devolvido).
pub(crate) unsafe extern "thiscall" fn compare_remove_dispute(this: usize, p: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REMOVE_DISPUTE);
    if p == 0 || read_u32_as_usize(this + 4) == 0 {
        return o(this, p);
    }
    let m = read_u32_as_usize(this + 4).wrapping_add(read_i32(p) as isize as usize);
    let list = read_u32_as_usize(this);
    if *(m as *const u8) == 0 {
        let mut reg = vec![(m, 1)];
        if list != 0 {
            reg.push((list, 0xe));
        }
        let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, p), || remove_dispute(this, p));
        SCOREBOARD.record(igual, || format!("remove_dispute(pessoa {:x}) {:x?} x {:x?}", p, mo.0, mr.0));
        return;
    }
    let mut found = 0i16;
    let count_before = if list != 0 { read_i16(list) } else { 0 };
    if list != 0 {
        let cursor = read_u32_as_usize(list + llist::CURSOR);
        core::ptr::write_unaligned((list + llist::CURSOR) as *mut u32, read_u32_as_usize(list + llist::HEAD) as u32);
        let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
        for _ in 0..count_before.max(0) {
            let no = next(list);
            if no == 0 {
                break;
            }
            if read_i32(no) == read_i32(p) {
                found = 1;
                break;
            }
        }
        core::ptr::write_unaligned((list + llist::CURSOR) as *mut u32, cursor as u32);
    }
    o(this, p);
    let count_after = if list != 0 { read_i16(list) } else { 0 };
    SCOREBOARD.record(*(m as *const u8) == 0 && count_after == count_before - found,
        || format!("remove_dispute(pessoa {:x}) conta {} -> {} (achou {})", p, count_before, count_after, found));
}
