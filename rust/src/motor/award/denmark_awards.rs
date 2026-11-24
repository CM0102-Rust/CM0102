use super::*;

/// Builds Denmark's transfer rules: the base rules, Denmark's vtable and no transfer windows, so Danish clubs sign at
/// any time.
///
/// `DENMARK_RULES::DENMARK_RULES(a1..a5)`, once, when the transfer manager builds its rules (0x8d2892). The base
/// constructor (`exe::new_transfer_rules`), the vtable 0x969860, the window count (+8) and the window table (+4) at 0.
/// Returns the object.
/// Original: `DENMARK_RULES::DENMARK_RULES` (0x555460, denmark_awards.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_denmark_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("sx03_4 denmark rules");
    exe::new_transfer_rules(this, a1, a2, a3, a4, a5);
    write_u32(this, 0x969860);
    write_u8(this + 8, 0); // sx03_4: no windows
    write_u32(this + 4, 0);
    this
}

/// Comparer for `construct_denmark_rules` (`compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_denmark_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(IDX_CONSTRUCT_DENMARK_RULES);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_denmark_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_denmark_rules(s, a1, a2, a3, a4, a5);
    });
    r
}
