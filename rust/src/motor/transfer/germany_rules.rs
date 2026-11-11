use super::*;

/// the German rule (10): two windows, from 1 July to 15 August and from 1 July to 15 January [inference: the second for
/// free agents]
/// Original: `GERMANY_RULES::init_transfer_windows` (0x5dffe0, germany_rules.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn germany_rules_init_transfer_windows(this: usize) {
    covered!("cqk_3 germany");
    set_transfer_windows(this, 2, &GERMANY_TRANSFER_WINDOWS, 0x9cd538, 0x4d);
}
