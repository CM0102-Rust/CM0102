use super::*;

// The comparers of the routines in ../comp/leagues/jap_j2.rs.
// [pt] Os comparadores das rotinas de ../comp/leagues/jap_j2.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparador: como o do Goiano.
pub(crate) unsafe extern "thiscall" fn compare_jap_j2_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_JAP_J2_SET_UP_RULES)(this), || {
        j2_rules(this);
        1
    });
    SCOREBOARD.record(igual && r == n, || format!("jap_j2_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// Comparer for `jap_j2_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_jap_j2_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_JAP_J2_CREATE_TABLE);
    compare_japanese_table("jap_j2_create_table", League(this), 0xc, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2, true, 0, || o(this))
}
