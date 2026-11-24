// ---------- sx05b_22: stepping a money amount up or down on the display currency's ladder (cash.cpp) ----------

/// A display currency amount back to a stored cash value: divided by the exchange rate (`CURRENCY_RATE`), truncated to
/// 64 bits (`_ftol`) and kept in its low dword.
pub(crate) unsafe fn cash_from_display(amount: i64) -> u32 {
    (amount as f64 / read_f64(CURRENCY_RATE)) as i64 as u32
}

/// Raises a cash value one step: its display amount (`cash_get_display_amount` with `flags`: rounded to the nearest
/// step unless flag 0x20) plus the rounding step of the next amount (`cash_get_rounding_step`(amount + 1, flags)), back
/// in pounds (`cash_from_display`); stored and returned.
/// Original: `Cash::step_up` (0x43f4e0, cash.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cash_step_up(cash: usize, flags: u32) -> u32 {
    let amount = cash_get_display_amount(cash, flags);
    let v = cash_from_display(amount.wrapping_add(cash_get_rounding_step(amount.wrapping_add(1), flags)));
    covered!("sx05b_22 cash stepped up");
    write_u32(cash, v);
    v
}

/// Lowers a cash value one step: its display amount (as `cash_step_up`) minus the rounding step of that amount, back
/// in pounds; stored and returned.
/// Original: `Cash::step_down` (0x43f580, cash.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cash_step_down(cash: usize, flags: u32) -> u32 {
    let amount = cash_get_display_amount(cash, flags);
    let v = cash_from_display(amount.wrapping_sub(cash_get_rounding_step(amount, flags)));
    covered!("sx05b_22 cash stepped down");
    write_u32(cash, v);
    v
}

/// Comparer for `cash_step_up`: side by side over the cash value and the result.
pub(crate) unsafe extern "thiscall" fn compare_cash_step_up(cash: usize, flags: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I0%%);
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&[(cash, 4)], || o(cash, flags), || cash_step_up(cash, flags));
    SCOREBOARD.record(equal && r == v, || format!("cash_step_up(flags {:#x}) original={} rust={} {:?}", flags, r, v, first_differences(&a, &b)));
    r
}

/// Comparer for `cash_step_down`: side by side over the cash value and the result.
pub(crate) unsafe extern "thiscall" fn compare_cash_step_down(cash: usize, flags: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I1%%);
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&[(cash, 4)], || o(cash, flags), || cash_step_down(cash, flags));
    SCOREBOARD.record(equal && r == v, || format!("cash_step_down(flags {:#x}) original={} rust={} {:?}", flags, r, v, first_differences(&a, &b)));
    r
}
