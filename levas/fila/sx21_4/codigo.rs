// ---------- sx21_4: the base transfer rules: the default answers of slots 0x14, 0x20 and 0x24 (argentina_rules.cpp) ----------

/// The base answer of vtable slot 0x14: always 1 (no country of 3.9.68 overrides it).
///
/// `TRANSFER_RULES::rule_slot_5(_a1, _a2, _a3)`: returns 1 without reading its arguments.
/// Original: `TRANSFER_RULES::rule_slot_5` (0x40a5f0, argentina_rules.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn transfer_rules_rule_slot_5(_this: usize, _a1: u32, _a2: u32, _a3: u32) -> u32 {
    covered!("sx21_4 slot 5");
    1 // sx21_4: slot 5 allows
}

/// Comparer for `transfer_rules_rule_slot_5`: the value of both.
pub(crate) unsafe extern "thiscall" fn compare_transfer_rules_rule_slot_5(this: usize, a1: u32, a2: u32, a3: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(%%I0%%)(this, a1, a2, a3);
    let c = transfer_rules_rule_slot_5(this, a1, a2, a3);
    SCOREBOARD.record(r == c, || format!("transfer_rules_rule_slot_5 original={} rust={}", r, c));
    r
}

/// The base loan check: every loan is allowed.
///
/// `TRANSFER_RULES::can_loan_player(_club, _player, _msg, _reason)`, the vtable slot
/// `transfer_rules_vtable::CAN_LOAN_PLAYER` of the countries without a loan rule: returns 1, writing no message nor
/// reason.
/// Original: `TRANSFER_RULES::can_loan_player` (0x40a600, argentina_rules.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn transfer_rules_can_loan_player(_this: usize, _club: usize, _player: usize, _msg: usize, _reason: *mut u16) -> u32 {
    covered!("sx21_4 base loan check");
    1 // sx21_4: the loan is allowed
}

/// Comparer for `transfer_rules_can_loan_player`: the value of both.
pub(crate) unsafe extern "thiscall" fn compare_transfer_rules_can_loan_player(this: usize, club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(%%I1%%)(this, club, player, msg, reason);
    let c = transfer_rules_can_loan_player(this, club, player, msg, reason);
    SCOREBOARD.record(r == c, || format!("transfer_rules_can_loan_player(club {:#x}, player {:#x}) original={} rust={}", club, player, r, c));
    r
}

/// The base check of a loan's end: a loaned player may be recalled or sent back at any time.
///
/// `TRANSFER_RULES::can_end_loan(_club, _player, _msg, _date, _flag)`, the vtable slot
/// `transfer_rules_vtable::CAN_END_LOAN` of the countries without a loan deadline (England among them): returns 1.
/// Original: `TRANSFER_RULES::can_end_loan` (0x40a610, argentina_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn transfer_rules_can_end_loan(_this: usize, _club: usize, _player: usize, _msg: usize, _date: usize, _flag: u32) -> u32 {
    covered!("sx21_4 base loan end");
    1 // sx21_4: the loan may end
}

/// Comparer for `transfer_rules_can_end_loan` (`rules_compare_can_end_loan`: the return and the message).
pub(crate) unsafe extern "thiscall" fn compare_transfer_rules_can_end_loan(this: usize, club: usize, player: usize, msg: usize, date: usize, flag: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32>(%%I2%%);
    rules_compare_can_end_loan(o, transfer_rules_can_end_loan, "transfer_rules_can_end_loan", this, club, player, msg, date, flag)
}
