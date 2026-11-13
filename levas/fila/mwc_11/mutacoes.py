# Mutacoes (mwc_11).
MUTACOES = [
    ('mwc11_disciplina_inglesa',
     'discipline: discipline_rule::GERMANY_LEAGUE, // mwc11: the Bundesliga discipline rule',
     'discipline: discipline_rule::ENGLAND, // mutation: the English discipline rule',
     'mwc_1 new division'),
    ('mwc11_vitoria_vale_2',
     'v.push((0xc2, 1, 3)); // mwc11: the Bundesliga points for a win',
     'v.push((0xc2, 1, 2)); // mutation: 2 points for a win',
     'mwc_11 ger first rules'),
    ('mwc11_gerfirst_rodada_5_uma_semana_depois',
     '    (9, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 5: 9 Sep (mwc11 ger_first)',
     '    (16, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 5: 9 Sep mutation: a week later',
     'mwc_11 ger first schedule'),
]
