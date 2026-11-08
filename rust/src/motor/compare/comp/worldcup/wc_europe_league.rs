use super::*;

// The comparers of the routines in ../comp/worldcup/wc_europe_league.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/worldcup/wc_europe_league.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `wc_europe_league_set_up_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_wc_europe_league_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_EUROPE_LEAGUE_SET_UP_RULES);
    check_qualifier_rules("wc_europe_league_set_up_rules", this, &EUROPE_RULE_FIELDS, (0x19, 2, 5), 0, || o(this))
}

/// Comparer for `wc_europe_league_set_up_rules_921040` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_wc_europe_league_set_up_rules_921040(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_EUROPE_LEAGUE_SET_UP_RULES_921040);
    let lista = core::ptr::read_unaligned(G_COMP_OBJECT_TABLE as *const usize);
    let mut ef = *((this + 0xef) as *const u8);
    if lista != 0 {
        let c = core::ptr::read_unaligned((lista + core::ptr::read_unaligned(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION as *const usize).wrapping_mul(4)) as *const usize);
        if c != 0 {
            ef = *((c + 0xee) as *const u8);
        }
    }
    let vagas = 4u8.wrapping_sub(ef).wrapping_sub(*((this + 0xee) as *const u8));
    let r = check_qualifier_rules("wc_europe_league_set_up_rules_921040", this, &SOUTH_AMERICA_QUALIFIER_RULE_FIELDS, (0x19, 0, 5), 0, || o(this));
    let (lido_ef, lido_be) = (*((this + 0xef) as *const u8), *((this + 0xbe) as *const u8));
    PLACAR.registrar(lido_ef == ef && lido_be == vagas, || format!("wc_europe_league_set_up_rules_921040({:#x}) +0xef {} previsto {} vagas {} previstas {}", this, lido_ef, ef, lido_be, vagas));
    r
}

/// Comparer for `wc_europe_league_set_up_rules_912eb0` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_wc_europe_league_set_up_rules_912eb0(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_EUROPE_LEAGUE_SET_UP_RULES_912EB0);
    check_qualifier_rules("wc_europe_league_set_up_rules_912eb0", this, &ASIA_QUALIFIER_RULE_FIELDS, (0x19, 2, 5), 1, || o(this))
}
