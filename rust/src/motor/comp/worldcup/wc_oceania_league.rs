use super::*;

/// rule: in the Oceania qualifier groups, a team going through gets the qualification (weight 20) and one going out the
/// failed-to-qualify record and ends its campaign; in the Oceania final, the winner gets the qualification (weight 30)
/// and the loser the knock-out; in the play-off (against South America [inference]), the winner gets the World Cup
/// place (weight 100 record, the entry in its list and status 4) and the loser the knock-out (weight 100) and the end
/// of the campaign
/// Original: `WcOceaniaLeague::update_staff_records_for_status` (0x91e840, wc_oceania_league.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x91e840 } (thiscall, ret 0x18, wc_oceania_league.cpp; a virtual +0x44): `WcOceaniaLeague::
/// update_staff_records_for_status(seleção, desfecho, fase, saida, jogo, _)`; devolve 0. Fase (byte com sinal) 0 ou
/// menos, os grupos: 6, a classificação (fase 0, rodada 0x82, peso 0x14); 0xfe, a não classificação (0x41c, 0x14) e a
/// competição atual da seleção acaba. Fase 1 (a final da Oceania): 1, a classificação (0xa0, 0, 0x1e); 2 ou 0xff, a
/// eliminação (+0x32 e +0x30 do jogo, peso 0x1e) e a competição acaba. Fase 2 (a repescagem): 1, a classificação para
/// a Copa do Mundo (a competição [0xae23d0] + 0x6b × [0x9cf79c], 0, 0, 0x64), a vaga nela (0x92e740 no objeto
/// [[0xadadfc] + 4 × [0x9cf79c]]: push seleção -> thiscall, ret 4) e *saida = 4; 2, a eliminação (peso 0x64) e a
/// competição acaba.
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, _a6: u32) -> u32 {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_u32_as_usize(this + 4);
    let eliminar = |weight: u32| {
        // simplification: +0x30 and +0x32 go in registers with garbage in the high 16 bits; 0x86c000 only uses the words
        european_cup_add_knocked_out_record(club, comp, read_i16(game + 0x32) as u16 as u32, read_i16(game + 0x30) as u16 as u32, weight);
        clear_national_team_competition(club);
    };
    if f < 1 {
        if d == 6 {
            european_cup_add_qualified_record(club, comp, 0, 0x82, 0x14);
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x14);
            clear_national_team_competition(club);
        }
    } else if f == 1 {
        if d == 1 {
            european_cup_add_qualified_record(club, comp, 0xa0, 0, 0x1e);
        } else if d == 2 || d == 0xff {
            eliminar(0x1e);
        }
    } else if f == 2 {
        if d == 1 {
            crate::comparison::branch("cqj_10 oceania na copa do mundo");
            let id = read_u32_as_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP);
            european_cup_add_qualified_record(club, read_u32_as_usize(G_CLUB_COMP_TABLE).wrapping_add(id.wrapping_mul(0x6b)), 0, 0, 0x64);
            let slot: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_WORLDCUP_ADD_QUALIFIED_TEAM);
            slot(read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE).wrapping_add(id.wrapping_mul(4))), club);
            *(out as *mut u8) = 4;
        } else if d == 2 {
            eliminar(0x64);
        }
    }
    0
}

/// rule: the Oceania qualifier groups are single round robin, 3/1 points, tiebreak 1, 2, 3, one qualifier per group (to
/// the Oceania final), category 10, 7 substitutes and 3 changes, starting on 15 of month 0 on weekday 5
/// Original: `WcOceaniaLeague::set_up_rules` (0x91ecf0, wc_oceania_league.cpp, thiscall, ret 0).
///
/// [pt] { 0x91ecf0 } (thiscall, ret 0, wc_oceania_league.cpp; a virtual +0x8c; quadro SEH só da data local):
/// `WcOceaniaLeague::set_up_rules()`: os grupos da eliminatória da Oceania em turno único, 3/1 pontos, a categoria 0xa,
/// os desempates (1, 2, 3), um classificado por grupo, seleções, 7 no banco e 3 substituições, começo em 15 do mês 0
/// do ano da competição num dia da semana 5.
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_set_up_rules(this: usize) -> u32 {
    crate::comparison::branch("cqk_10 regras da oceania");
    goldcup_write_rule_fields(this, &OCEANIA_RULE_FIELDS, 3);
    let day = 0xf; // cqk10: the start day in Oceania
    let word = competition_date_word(this, day, 0, 0, 5);
    finish_rules_setup(this, word, 0)
}
