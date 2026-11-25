// ---------- rx13_7: clearing a contract's grievances and event flags (staff_contracts.cpp) ----------

/// rule: a person's grievances (the given bits) are cleared; then the transfer request is withdrawn if no grievance
/// still justifies it, and the opinion links tied to those grievances are removed
///
/// The contract's `grievances()` lose `bits` (the old value kept); the withdrawal check
/// (`exe::staff_contract_check_transfer_request_withdrawal`(contract, old grievances)); then the person of the contract
/// (`staff_at` of the id at +0, read again after the check) loses the opinion links of `bits`
/// (`exe::staff_remove_opinion_link`(person, bits), cdecl).
/// Original: `STAFF_CONTRACT::clear_grievances` (0x8674e0, staff_contracts.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn staff_contract_clear_grievances(this: usize, bits: u32) {
    covered!("rx13_7 grievances cleared");
    let contract = Contract(this);
    let old = contract.grievances();
    contract.set_grievances(old & !bits); // rx13_7: the bits cleared
    exe::staff_contract_check_transfer_request_withdrawal(contract, old);
    let person = staff_at(read_i32(this) as isize); // +0: the person's id
    exe::staff_remove_opinion_link(person, bits as i32);
}

/// rule: the given event flag bits of a contract are cleared
///
/// The contract's `flags()` lose `bits`.
/// Original: `STAFF_CONTRACT::clear_flags` (0x8676d0, staff_contracts.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn staff_contract_clear_flags(this: usize, bits: u32) {
    covered!("rx13_7 flags cleared");
    let contract = Contract(this);
    contract.set_flags(contract.flags() & !bits); // rx13_7: the flag bits cleared
}

/// Only the original: the withdrawal check and the opinion links may post news and change other people, so it cannot
/// run twice. In the comparison mode the post-match gate of 0x8674e0 (`POS_PROLOGUES`) is installed first and keeps
/// the address, so this comparer is not installed there.
pub(crate) unsafe extern "thiscall" fn compare_staff_contract_clear_grievances(this: usize, bits: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%)(this, bits)
}

/// Comparer for `staff_contract_clear_flags`: side by side over the flags dword. In the comparison mode the post-match
/// gate of 0x8676d0 is installed first and keeps the address (as for 0x8674e0).
pub(crate) unsafe extern "thiscall" fn compare_staff_contract_clear_flags(this: usize, bits: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I1%%);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(this + contract::FLAGS, 4)], || o(this, bits), || staff_contract_clear_flags(this, bits));
    SCOREBOARD.record(equal, || format!("staff_contract_clear_flags({:#x}) original {:x?} rust {:x?}", bits, mo, mr));
}
