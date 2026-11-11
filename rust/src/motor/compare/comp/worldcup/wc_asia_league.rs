use super::*;

// The comparers of the routines in ../comp/worldcup/wc_asia_league.rs.

/// Comparer for `wc_asia_league_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_ASIA_LEAGUE_SET_UP_RULES);
    check_qualifier_rules("wc_asia_league_set_up_rules", this, &ASIA_QUALIFIER_RULE_FIELDS, (0x19, 2, 5), 1, || o(this))
}
