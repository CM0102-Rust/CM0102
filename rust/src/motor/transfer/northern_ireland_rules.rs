use super::*;

/// Constructs the Northern Ireland transfer rules (rule 16): one window, from 6 May (moved to the Saturday [inference])
/// to 20 March.
///
/// `NORTHERN_IRELAND_RULES::NORTHERN_IRELAND_RULES`; SEH frame only for the `new`, no destructor on the normal path.
/// The Northern Ireland transfer rules (rule 16): the base (`exe::new_transfer_rules`), the vtable 0x96df9c, 1 window from
/// `NORTHERN_IRELAND_TRANSFER_WINDOWS`. Returns the object.
/// Original: `NORTHERN_IRELAND_RULES::NORTHERN_IRELAND_RULES` (0x7936d0, northern_ireland_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_northern_ireland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("cqm_2 northern_ireland");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x96df9c, 1, &NORTHERN_IRELAND_TRANSFER_WINDOWS, 0xa697f4, 0x4e);
    this
}
