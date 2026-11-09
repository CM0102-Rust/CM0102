use super::*;

// The comparers of the routines in ../comp/worldcup/wc_south_american_league.rs.
// [pt] Os comparadores das rotinas de ../comp/worldcup/wc_south_american_league.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `wc_south_american_league_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_set_up_rules(this: usize) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_SOUTH_AMERICAN_LEAGUE_SET_UP_RULES);
    let list = core::ptr::read_unaligned(G_COMP_OBJECT_TABLE as *const usize);
    let mut ef = *((this.addr() + 0xef) as *const u8);
    if list != 0 {
        let c = core::ptr::read_unaligned((list + core::ptr::read_unaligned(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION as *const usize).wrapping_mul(4)) as *const usize);
        if c != 0 {
            ef = *((c + 0xee) as *const u8);
        }
    }
    let slots = 4u8.wrapping_sub(ef).wrapping_sub(*((this.addr() + 0xee) as *const u8));
    let r = check_qualifier_rules("wc_south_american_league_set_up_rules", this.addr(), &SOUTH_AMERICA_QUALIFIER_RULE_FIELDS, (0x19, 0, 5), 0, || o(this.addr()));
    let (lido_ef, lido_be) = (*((this.addr() + 0xef) as *const u8), this.promoted_count());
    SCOREBOARD.record(lido_ef == ef && lido_be == slots, || format!("wc_south_american_league_set_up_rules({:#x}) +0xef {} previsto {} vagas {} previstas {}", this.addr(), lido_ef, ef, lido_be, slots));
    r
}
