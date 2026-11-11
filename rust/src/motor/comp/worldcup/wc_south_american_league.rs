use super::*;

/// rule: the South American qualifier is a single home and away league, 3/1 points, tiebreak 1, 2 and 3; the direct
/// places are four minus the exempt nations (the world champion and South American hosts, counted in +0xee and +0xef
/// [inference]); the fifth goes to the play-off (+0xbf = 1); category 10, 7 substitutes and 3 changes; starts on 25 of
/// month 0 on weekday 5
/// Original: `WcSouthAmericanLeague::set_up_rules` (0x921040, wc_south_american_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_set_up_rules(this: usize) -> u32 {
    let this = League(this);
    let list = read_usize(G_COMP_OBJECT_TABLE);
    if list != 0 {
        let o = read_usize(list + read_usize(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION).wrapping_mul(4));
        if o != 0 {
            write_u8(this.addr() + wc_south_american_league_fields::OCEANIA_EXEMPT_COUNT, read_u8(o + wc_oceania_league_fields::EXEMPT_COUNT));
        }
    }
    covered!("cqj_12 south america rules");
    wc_europe_league_write_rule_fields(this, &SOUTH_AMERICA_QUALIFIER_RULE_FIELDS);
    this.set_win_points(3); // cqj12: the win in the South American qualifiers
    this.set_promoted_count(4u8.wrapping_sub(read_u8(this.addr() + wc_south_american_league_fields::OCEANIA_EXEMPT_COUNT)).wrapping_sub(read_u8(this.addr() + wc_south_american_league_fields::EXEMPT_COUNT))); // cqj12: the direct places of South America
    let day = 0x19; // cqj12: the start day of the South American qualifier
    let word = comp_date_word(this, day, 0, 5);
    finish_rules_setup(this, word, 0)
}
