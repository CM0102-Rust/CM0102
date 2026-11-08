use super::*;

// The comparers of the routines in ../comp/worldcup/wc_europe_league.rs.
// [pt] Os comparadores das rotinas de ../comp/worldcup/wc_europe_league.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `wc_europe_league_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_EUROPE_LEAGUE_SET_UP_RULES);
    check_qualifier_rules("wc_europe_league_set_up_rules", this, &EUROPE_RULE_FIELDS, (0x19, 2, 5), 0, || o(this))
}

