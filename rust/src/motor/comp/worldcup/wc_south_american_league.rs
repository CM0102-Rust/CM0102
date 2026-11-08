use super::*;

/// rule: the South American qualifier is a single home and away league, 3/1 points, tiebreak 1, 2 and 3; the direct
/// places are four minus the exempt nations (the world champion and South American hosts, counted in +0xee and +0xef
/// [inference]); the fifth goes to the play-off (+0xbf = 1); category 10, 7 substitutes and 3 changes; starts on 25 of
/// month 0 on weekday 5
/// Original: `WcSouthAmericanLeague::set_up_rules` (0x921040, wc_south_american_league.cpp, thiscall, ret 0).
///
/// [pt] { 0x921040 } (thiscall, ret 0, wc_south_american_league.cpp; a virtual +0x8c; quadro SEH só da data local):
/// `WcSouthAmericanLeague::set_up_rules()`: com o objeto da competição [0x9cf76c], +0xef recebe o +0xee dele [inferência:
/// as nações isentas por serem a campeã]; a liga única sul-americana em dois turnos, 3/1 pontos, a categoria 0xa, os
/// desempates (1, 2, 3), vagas diretas 4 − +0xef − +0xee (as isentas saem das quatro), uma de repescagem, seleções, 7 no
/// banco e 3 substituições, +0xa7 = −1, +0xab = +0xdb = 0, começo em 25 do mês 0 num dia da semana 5.
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_set_up_rules(this: usize) -> u32 {
    let list = core::ptr::read_unaligned(G_COMP_OBJECT_TABLE as *const usize);
    if list != 0 {
        let o = core::ptr::read_unaligned((list + core::ptr::read_unaligned(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION as *const usize).wrapping_mul(4)) as *const usize);
        if o != 0 {
            *((this + 0xef) as *mut u8) = *((o + 0xee) as *const u8);
        }
    }
    crate::comparison::branch("cqj_12 south america rules");
    wc_europe_league_write_rule_fields(this, &SOUTH_AMERICA_QUALIFIER_RULE_FIELDS);
    *((this + league::WIN_POINTS) as *mut u8) = 3; // cqj12: the win in the South American qualifiers
    *((this + league::PROMOTED_COUNT) as *mut u8) = 4u8.wrapping_sub(*((this + 0xef) as *const u8)).wrapping_sub(*((this + 0xee) as *const u8)); // cqj12: the direct places of South America
    let day = 0x19; // cqj12: the start day of the South American qualifier
    let word = comp_date_word(this, day, 0, 5);
    finish_rules_setup(this, word, 0)
}
