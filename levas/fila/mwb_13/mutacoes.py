# Mutacoes (mwb_13): a Premier com a disciplina das copas (0x19: outras suspensoes); a tabela com os clubes uma linha
# adiante (outra ordem, outros jogos); 2 pontos por vitoria (outra classificacao, outra divisao em dois grupos).
MUTACOES = [
    ('mwb13_premier_disciplina_das_copas',
     'discipline: 0x18, // mwb13: the Premier discipline rule',
     'discipline: 0x19, // mutation: the cups rule',
     'mwb_13 new scottish premier'),
    ('mwb13_premier_linhas_giradas',
     'Some(0xc350), Some(0x7a120), 0, "mwb_13 scottish premier table") // mwb13: the Premier rows',
     'Some(0xc350), Some(0x7a120), 1, "mwb_13 scottish premier table") // mutation: rotated rows',
     'mwb_13 scottish premier table'),
    ('mwb13_premier_dois_pontos',
     '(league::WIN_POINTS, 1, 3), // mwb13: the Premier points for a win',
     '(league::WIN_POINTS, 1, 2), // mutation: 2 points a win',
     'mwb_13 scottish premier rules'),
]
