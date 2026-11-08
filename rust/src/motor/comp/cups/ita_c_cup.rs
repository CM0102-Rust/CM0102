use super::*;

/// rule: the Serie C Cup starts with five-round groups in August (12, 19, 22, 26 and 29; id 0x420 + stage); the
/// knockout (stage 0xf) over two legs: 1st round drawn on 30/8 and played on 11/10 (32 teams), 2nd 26/10 and 1/11,
/// quarter-finals 9/11 and 15/11, semi-final 30/11 and 11 April, final draw 26/4 and match on 9 May; 14 days between
/// legs, 7 in the 2nd round and the final; category 4 up to the 2nd round and 6 after
/// Original: `ItaCCup::create_schedule` (0x63cf90, ita_c_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x63cf90 } (thiscall, ret 0x10, ita_c_cup.cpp; a virtual +0x3c da Copa da Série C): `ItaCCup::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: grupos abaixo da fase 0xf (extra = fase + 0x420; a caixa da linha 0xbc), o mata-mata na
/// fase 0xf (a caixa da linha 0xe3).
pub(crate) unsafe extern "thiscall" fn ita_c_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_schedule(this, stage, rounds, extra, league, 0xf, 0x420, &SERIE_C_CUP_GROUP_ROUNDS, &SERIE_C_CUP_KNOCKOUT_ROUNDS, 0x9ea204, 0xbc, 0xe3,
        "cqo_10 grupos da copa da serie c", "cqo_10 mata-mata da copa da serie c")
}

/// rule: the Serie C Cup takes the 90 Italian clubs of C1 and C2 (the two C1 groups and the three C2 ones). Those that
/// came down from Serie B and those that came from C1 in 4th, 5th or 6th go to the last places (from the 81st); the
/// first 80 are shuffled, sorted by the city's longitude and each of their 8 tens shuffled (the last 10 places stay in
/// order): the groups are regional [inference]
/// Original: `ItaCCup::select_entrants` (0x63dfe0, ita_c_cup.cpp, thiscall, ret 0).
///
/// [pt] regra: entram na Copa da Série C os 90 clubes italianos da C1 e da C2 (as duas chaves da C1 e as três da C2). Os que
/// vieram da Série B (rebaixados) e os que vieram da C1 em 4º, 5º ou 6º vão para as últimas vagas (a partir da 81ª); os
/// 80 primeiros são embaralhados, ordenados pela longitude da cidade e cada uma das suas 8 dezenas embaralhada (as 10
/// últimas vagas ficam na ordem): os grupos são regionais [inferência]
pub(crate) unsafe extern "thiscall" fn ita_c_cup_select_entrants(this: usize) -> i32 {
    select_serie_c_cup_entrants(this).0
}

/// rule: the Serie C Italian Cup groups follow the Italian Cup rules (single round robin, 3/1, tiebreak 4, 1, 2, one
/// qualifier per group); prize fund of 300,000
/// Original: `ItaCCup::set_up_rules` (0x63d6e0, ita_c_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x63d6e0 } (thiscall, ret 0, ita_c_cup.cpp; a virtual +0x8c): `ItaCCup::set_up_rules()`: os grupos da Copa da
/// Itália da Série C, com as mesmas regras da Copa da Itália e o fundo de prêmios de 300.000.
pub(crate) unsafe extern "thiscall" fn ita_c_cup_set_up_rules(this: usize) -> u32 {
    crate::comparison::branch("cqi_12 regras da copa da serie c");
    let turnos = 1i16; // cqi12: o turno único dos grupos da Série C
    set_cup_league_rules(this, &SERIE_C_CUP_RULES, turnos)
}
