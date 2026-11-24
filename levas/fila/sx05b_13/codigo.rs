// ---------- sx05b_13: the Swedish teams of the week and the Swedish transfer rules (award/sweden_awards.cpp) ----------

/// Builds the team of the week of a Swedish team award: only for the Premier Division and First Division teams of the
/// week (awards 7 and 9 of the table), `NationAwards::update_team_of_the_week` with the award's club competition id.
/// Original: `SwedenAwards::update_team_of_the_week` (0x893d60, sweden_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn sweden_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &[7, 9]);
}

/// Builds the Swedish transfer rules: the base (`exe::new_transfer_rules` with the five arguments), the vtable
/// 0x970368, no transfer windows (+4 = 0, count +8 = 0) and `transfer_rules::SKIP_CONTRACT_TYPE_5_INELIGIBILITY` = 1.
/// Returns this.
/// Original: `SWEDEN_RULES::SWEDEN_RULES` (0x893d80, sweden_awards.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_sweden_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    exe::new_transfer_rules(this, a1, a2, a3, a4, a5);
    write_u32(this, 0x970368);
    write_u8(this + 8, 0);
    write_usize(this + 4, 0);
    write_u32(this + transfer_rules::SKIP_CONTRACT_TYPE_5_INELIGIBILITY, 1);
    covered!("sx05b_13 swedish rules");
    this
}

/// Comparer for `sweden_awards_team_of_the_week`: the team of the week cannot be built twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_sweden_awards_team_of_the_week(this: usize, award: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, award)
}

/// Comparer for `construct_sweden_rules`: the original builds the rules, the copy a scratch copy, field by field
/// (`compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_sweden_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I1%%);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_sweden_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_sweden_rules(s, a1, a2, a3, a4, a5);
    });
    r
}
