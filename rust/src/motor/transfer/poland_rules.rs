use super::*;

/// Constructs the Poland transfer rules (rule 18): one window, from 1 July to 20 February;
/// `transfer_rules::FOREIGN_PLAYER_LIMIT` = 5 (base default 50) [inference: the foreign player limit];
/// `transfer_rules::NON_EU_PLAYER_LIMIT` = 5 (default 30). Ret 0x14, `POLAND_RULES::POLAND_RULES`; SEH frame only for
/// the `new`, no destructor on the normal path: the base (`exe::new_transfer_rules`), vtable 0x96e61c, 1 window from
/// `POLAND_TRANSFER_WINDOWS`; `transfer_rules::FOREIGN_PLAYER_LIMIT` = 5 (the base default is 50) [inference: the
/// foreign player limit]; `transfer_rules::NON_EU_PLAYER_LIMIT` = 5 (the default is 30). Returns the object.
/// Original: `POLAND_RULES::POLAND_RULES` (0x7cd830, poland_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_poland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("cqm_3 poland");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x96e61c, 1, &POLAND_TRANSFER_WINDOWS, 0xa786b8, 0x51);
    write_u8(this + transfer_rules::FOREIGN_PLAYER_LIMIT, 5);
    write_u8(this + transfer_rules::NON_EU_PLAYER_LIMIT, 5);
    this
}
