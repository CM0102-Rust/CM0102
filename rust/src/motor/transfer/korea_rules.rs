use super::*;

/// Constructs the South Korea transfer rules (rule 27): one window, from 25 November (moved to the Sunday [inference])
/// to 25 July; +0x13 = 7 (base default 50) [inference: the foreign limit].
///
/// `KOREA_RULES::KOREA_RULES`; SEH frame only for the `new`, no destructor on the normal path: the transfer rules of
/// South Korea (rule 27): the base (`exe::new_transfer_rules`), the vtable 0x96cfec, 1 window from `SOUTH_KOREA_TRANSFER_WINDOWS`; +0x13
/// = 7 (the base default is 50) [inference: the foreign player limit]. Returns the object.
/// Original: `KOREA_RULES::KOREA_RULES` (0x66e630, korea_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_korea_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("cqn_1 korea");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x96cfec, 1, &SOUTH_KOREA_TRANSFER_WINDOWS, 0x9eb20c, 0x51);
    write_u8(this + 0x13, 7);
    this
}
