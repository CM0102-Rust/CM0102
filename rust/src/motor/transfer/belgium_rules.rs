use super::*;

/// the Belgium rule (3): three windows, from 1 June to 30 June; from 1 June to 31 December; from 1 June to 31 March
/// Original: `BELGIUM_RULES::init_transfer_windows` (0x424c00, belgium_rules.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn belgium_rules_init_transfer_windows(this: usize) {
    covered!("cqn_2 belgium");
    set_transfer_windows(this, 3, &BELGIUM_TRANSFER_WINDOWS, 0x989e34, 0x53);
}
