use super::*;

/// rule: the Asian groups are home and away, 3/1 points, tiebreak 1, 2 and 3, each group winner goes on, category 10,
/// starts on 25 of month 2 on weekday 5, +0xb8 = 1 [inference: the stage starts the following year]
/// Original: `WcAsiaLeague::set_up_rules` (0x912eb0, wc_asia_league.cpp, thiscall, ret 0).
///
/// [pt] { 0x912eb0 } (thiscall, ret 0, wc_asia_league.cpp; a virtual +0x8c; quadro SEH só da data local):
/// `WcAsiaLeague::set_up_rules()`: os grupos asiáticos em dois turnos, 3/1 pontos, a categoria 0xa, os desempates (1, 2,
/// 3), um classificado por grupo, seleções, começo em 25 do mês 2 num dia da semana 5, +0xb8 = 1.
pub(crate) unsafe extern "thiscall" fn wc_asia_league_set_up_rules(this: usize) -> u32 {
    crate::comparison::branch("cqj_12 asia rules");
    wc_europe_league_write_rule_fields(this, &ASIA_QUALIFIER_RULE_FIELDS);
    let day = 0x19; // cqj12: the start day of the Asian qualifiers
    let word = comp_date_word(this, day, 2, 5);
    finish_rules_setup(this, word, 1)
}
