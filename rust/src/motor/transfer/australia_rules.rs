use super::*;

/// Constructs the Australian transfer rules (rule 2): one window, from 11 June (moved to the Saturday [inference]) to
/// 17 April; +0x13 = 6 [inference: the foreign squad limit].
///
/// `AUSTRALIA_RULES::Australia_rules`; SEH frame only for the `new`, no destructor on the normal path: the transfer
/// rules of Australia (rule 2): the base (`exe::new_transfer_rules`), the vtable 0x967830, 1 window from `AUSTRALIA_TRANSFER_WINDOWS`
/// and +0x13 = 6 [inference: the foreign player limit]. Returns the object.
/// Original: `AUSTRALIA_RULES::Australia_rules` (0x412c80, australia_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn australia_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("cqk_1 australia");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x967830, 1, &AUSTRALIA_TRANSFER_WINDOWS, 0x987cfc, 0x51);
    write_u8(this + 0x13, 6);
    this
}
