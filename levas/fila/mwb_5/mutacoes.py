# Mutacoes (mwb_5): a First escocesa com a disciplina das copas (0x19: outras suspensoes); a tabela com os clubes uma
# linha adiante (outra ordem, outros jogos); 2 pontos por vitoria (outra classificacao).
MUTACOES = [
    ('mwb5_first_disciplina_das_copas',
     'discipline: 0x18, // mwb5: the Scottish discipline rule',
     'discipline: 0x19, // mutation: the cups rule',
     'mwb_5 new scottish first'),
    ('mwb5_first_linhas_giradas',
     'Some(0x61a8), None, 0, "mwb_5 scottish first table") // mwb5: the First Division rows',
     'Some(0x61a8), None, 1, "mwb_5 scottish first table") // mutation: rotated rows',
     'mwb_5 scottish first table'),
    ('mwb5_first_dois_pontos',
     'scottish_division_rules(this, 3, 1, 2, 2, KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION, KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION) // mwb5: the First Division rules',
     'scottish_division_rules(this, 2, 1, 2, 2, KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION, KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION) // mutation: 2 points a win',
     'mwb_5 scottish first rules'),
]
