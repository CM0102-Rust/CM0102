// ---------- sbE_13: the Portuguese B team limits at the turnover (por_prm.cpp) ----------

/// The Portuguese B team rule: the Liga de Honra, the Second Division B in its North, Central and South zones, por_prm.cpp.
pub(crate) const PORTUGUESE_B_TEAM_LIMITS: BTeamLimits = BTeamLimits {
    second_key: KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION,
    groups_key: KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B,
    group_keys: &[KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_NORTH, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_CENTRAL, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_SOUTH],
    file: 0xa78724,
    rows: [0x390, 0x3c9, 0x3f7],
};

/// Rule: at the Portuguese turnover with the Second Division B, no B team may end in the same division as its senior
/// club: a Liga de Honra B team whose senior club goes down from the Premier Division is relegated with it (with the
/// news), one that would go up is held back and its place goes to the next undecided club; a Second Division B team in
/// the same case is held back (or relegated with the news when its senior club comes down to it) and its zone's place
/// goes to the next undecided club; each B team relegated from the Liga de Honra saves one relegated club.
///
/// `resolve_b_team_clashes` (sbE_12) with `PORTUGUESE_B_TEAM_LIMITS` (the zones: the Second Division B itself, its stages
/// 0 and 1; the por_prm.cpp (0xa78724) boxes at lines 0x390, 0x3c9 and 0x3f7).
/// Original: `PorPrm::apply_b_team_promotion_limits` (0x7d0320, por_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_prm_apply_b_team_promotion_limits(this: usize) {
    covered!("sbE_13 portuguese b team limits");
    resolve_b_team_clashes(League(this), &PORTUGUESE_B_TEAM_LIMITS);
}

/// Comparer for `por_prm_apply_b_team_promotion_limits` (`compare_b_team_clashes`).
pub(crate) unsafe extern "thiscall" fn compare_por_prm_apply_b_team_promotion_limits(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    compare_b_team_clashes("por_prm_apply_b_team_promotion_limits", &PORTUGUESE_B_TEAM_LIMITS, || o(this));
}
