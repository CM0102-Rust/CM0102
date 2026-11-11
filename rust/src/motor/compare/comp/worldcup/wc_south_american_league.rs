use super::*;

// The comparers of the routines in ../comp/worldcup/wc_south_american_league.rs.

/// Comparer for `wc_south_american_league_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_set_up_rules(this: usize) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_SOUTH_AMERICAN_LEAGUE_SET_UP_RULES);
    let list = read_usize(G_COMP_OBJECT_TABLE);
    let mut ef = read_u8(this.addr() + wc_south_american_league_fields::OCEANIA_EXEMPT_COUNT);
    if list != 0 {
        let c = read_usize(list + read_usize(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION).wrapping_mul(4));
        if c != 0 {
            ef = read_u8(c + wc_oceania_league_fields::EXEMPT_COUNT);
        }
    }
    let slots = 4u8.wrapping_sub(ef).wrapping_sub(read_u8(this.addr() + wc_south_american_league_fields::EXEMPT_COUNT));
    let r = check_qualifier_rules("wc_south_american_league_set_up_rules", this.addr(), &SOUTH_AMERICA_QUALIFIER_RULE_FIELDS, (0x19, 0, 5), 0, || o(this.addr()));
    let (lido_ef, lido_be) = (read_u8(this.addr() + wc_south_american_league_fields::OCEANIA_EXEMPT_COUNT), this.promoted_count());
    SCOREBOARD.record(lido_ef == ef && lido_be == slots, || format!("wc_south_american_league_set_up_rules({:#x}) +0xef {} predicted {} slots {} predicted {}", this.addr(), lido_ef, ef, lido_be, slots));
    r
}
