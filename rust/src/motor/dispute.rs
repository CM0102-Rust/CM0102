use super::*;

/// Clears the person's dispute flag and removes her entry from the dispute list.
///
/// `DISPUTE::remove_dispute(person)`: without a person or without the flag table (+4), or with the person's flag at
/// zero: nothing. Otherwise it zeroes the flag and, with the list (+0), moves the cursor back to the start (+0xa = +2)
/// and searches, among the first `count` (+0, word), the entry with the person's id (0x68d1b0): if found, it removes it
/// (0x68d1f0).
/// Original: `DISPUTE::remove_dispute` (0x56a090, dispute.cpp, thiscall).
///
/// [pt] { 0x56a090 } (thiscall, ret 4, dispute.cpp): `DISPUTE::remove_dispute(pessoa)`: sem pessoa ou sem a tabela de marcas
/// (+4), ou com a marca da pessoa zerada: nada. Senão zera a marca e, com a lista (+0), volta o cursor ao começo (+0xa =
/// +2) e procura, nas `conta` (+0, palavra) primeiras, a entrada do id da pessoa (0x68d1b0): achando, tira-a (0x68d1f0).
pub(crate) unsafe extern "thiscall" fn remove_dispute(this: usize, p: usize) {
    if p == 0 || read_u32_as_usize(this + 4) == 0 {
        return;
    }
    let m = (read_u32_as_usize(this + 4) as isize + read_i32(p) as isize) as *mut u8;
    if *m == 0 {
        return;
    }
    *m = 0;
    let list = read_u32_as_usize(this);
    if list == 0 {
        return;
    }
    core::ptr::write_unaligned((list + 0xa) as *mut u32, read_u32_as_usize(list + 2) as u32);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut k = 0i32;
    while (k as i16 as i32) < read_i16(read_u32_as_usize(this)) as i32 {
        let no = next(read_u32_as_usize(this));
        if no == 0 {
            return;
        }
        if read_i32(no) == read_i32(p) {
            crate::comparison::branch("cqg_7 saiu da contest");
            let remove: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_REMOVE);
            remove(read_u32_as_usize(this), no);
            return;
        }
        k += 1;
    }
}
