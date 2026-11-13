# Mutacoes (mwb_8): a Third com a disciplina das copas (0x19: outras suspensoes); a tabela com os clubes uma linha
# adiante (outra ordem, outros jogos); 2 pontos por vitoria (outra classificacao).
MUTACOES = [
    ('mwb8_third_disciplina_das_copas',
     'discipline: 0x18, // mwb8: the Third Division discipline rule',
     'discipline: 0x19, // mutation: the cups rule',
     'mwb_8 new scottish third'),
    ('mwb8_third_linhas_giradas',
     'None, None, 0, "mwb_8 scottish third table") // mwb8: the Third Division rows',
     'None, None, 1, "mwb_8 scottish third table") // mutation: rotated rows',
     'mwb_8 scottish third table'),
    ('mwb8_third_dois_pontos',
     'scottish_division_rules(this, 3, 2, 0, 0, KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION, 0) // mwb8: the Third Division rules',
     'scottish_division_rules(this, 2, 2, 0, 0, KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION, 0) // mutation: 2 points a win',
     'mwb_8 scottish third rules'),
]
