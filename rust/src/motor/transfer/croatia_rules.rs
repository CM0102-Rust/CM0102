use super::*;

/// Constructs the Croatia transfer rules (rule 5): one window, from 1 June to 30 March.
///
/// `CROATIA_RULES::CROATIA_RULES`; SEH frame only for the `new`, no destructor on the normal path: the transfer rules
/// of Croatia (rule 5): the base (`exe::new_transfer_rules`), the vtable 0x969394, 1 window from `CROATIA_TRANSFER_WINDOWS`. Returns the
/// object.
/// Original: `CROATIA_RULES::CROATIA_RULES` (0x518500, croatia_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_croatia_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("cqm_1 croatia");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x969394, 1, &CROATIA_TRANSFER_WINDOWS, 0x9b5570, 0x4e);
    this
}
