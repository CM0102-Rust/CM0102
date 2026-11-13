# Mutacoes (mwb_7): a Second com a disciplina das copas (0x19: outras suspensoes); a tabela com os clubes uma linha
# adiante (outra ordem, outros jogos); 2 pontos por vitoria (outra classificacao).
MUTACOES = [
    ('mwb7_second_disciplina_das_copas',
     'discipline: 0x18, // mwb7: the Second Division discipline rule',
     'discipline: 0x19, // mutation: the cups rule',
     'mwb_7 new scottish second'),
    ('mwb7_second_linhas_giradas',
     'None, None, 0, "mwb_7 scottish second table") // mwb7: the Second Division rows',
     'None, None, 1, "mwb_7 scottish second table") // mutation: rotated rows',
     'mwb_7 scottish second table'),
    ('mwb7_second_dois_pontos',
     'scottish_division_rules(this, 3, 2, 0, 2, KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION, KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION) // mwb7: the Second Division rules',
     'scottish_division_rules(this, 2, 2, 0, 2, KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION, KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION) // mutation: 2 points a win',
     'mwb_7 scottish second rules'),
]
