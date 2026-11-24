// --- sx20_4: clearing a contract's bonuses and release clauses (staff_contracts.cpp, 0x85f800) and the order of the
// shortlist position lists (shortlist_manager.cpp, 0x83b670) ---

/// Removes every bonus and release clause of a contract: the goal, assist and clean sheet bonuses become -1, the
/// non-promotion, minimum fee, 0x1e, relegation and manager job clause flags 0, and it is no longer leaving on a
/// Bosman nor has a transfer arranged (-1).
/// Original: `STAFF_CONTRACT::clear_bonuses_and_clauses` (0x85f800, staff_contracts.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn staff_contract_clear_bonuses_and_clauses(this: Contract) {
    covered!("sx20_4 contract bonuses cleared");
    this.set_leaving_on_bosman(0);
    this.set_transfer_arranged_for(usize::MAX);
    this.set_non_promotion_rc(0);
    this.set_minimum_fee_rc(0);
    write_u8(this.addr() + 0x1e, 0); // a release clause flag between minimum fee and relegation
    this.set_relegation_rc(0);
    this.set_manager_job_rc(0);
    this.set_goal_bonus(-1); // sx20_4: no goal bonus
    write_i32(this.addr() + contract::ASSIST_BONUS, -1);
    write_i32(this.addr() + contract::CLEAN_SHEET_BONUS, -1);
}

/// The qsort order of the shortlist position lists (12-byte entries): by the value at +8, lowest first, then by the
/// key at +4.
/// Original: `shortlist_compare_by_value_then_key` (0x83b670, shortlist_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn shortlist_compare_by_value_then_key(a: usize, b: usize) -> i32 {
    covered!("sx20_4 position list order");
    let by_value = read_i32(a + 8).wrapping_sub(read_i32(b + 8)); // sx20_4: lowest value first
    if by_value != 0 {
        return by_value;
    }
    read_i32(a + 4).wrapping_sub(read_i32(b + 4))
}

/// Comparer for `staff_contract_clear_bonuses_and_clauses`: side by side over the contract (+0x10..+0x4e).
pub(crate) unsafe extern "thiscall" fn compare_staff_contract_clear_bonuses_and_clauses(this: Contract) {
    let original = original_fn::<unsafe extern "thiscall" fn(Contract)>(%%I0%%);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(this.addr() + contract::GOAL_BONUS, 0x3e)],
        || original(this), || staff_contract_clear_bonuses_and_clauses(this));
    SCOREBOARD.record(same, || format!("staff_contract_clear_bonuses_and_clauses({:#x}) {:?}", this.addr(), first_differences(&mo, &mr)));
}

/// Comparer for `shortlist_compare_by_value_then_key`.
pub(crate) unsafe extern "C" fn compare_shortlist_compare_by_value_then_key(a: usize, b: usize) -> i32 {
    sx20_compare_pure("shortlist_compare_by_value_then_key", a, b,
        original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%), shortlist_compare_by_value_then_key)
}
