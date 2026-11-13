# Mutations (mwe_5): the Second Division section clubs filed under the lower division instead of the Second Division
# when the game is created (the Swedish Cup pool, built from the four divisions, changes); the Veikkausliiga
# relegating 2 instead of 1 and the Ykkonen promoting 1 instead of 2 (who goes up and down at the end of 2001, and the
# table status bytes in the autumn).
MUTACOES = [
    ('mwe5_secoes_na_divisao_de_baixo',
     'return Some((comp_record_of(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION), Some(comp))); // mwe5: a section club goes into the Second Division',
     'return Some((comp_record_of(KEY_CLUB_COMP_SWEDISH_LOWER_DIVISION), Some(comp))); // mutation: into the lower division',
     'mwe_5 new allsvenskan'),
    ('mwe5_veikkausliiga_rebaixa_2',
     'v.push((league::RELEGATED_COUNT, 1, 1)); // mwe5: the Veikkausliiga relegates 1',
     'v.push((league::RELEGATED_COUNT, 1, 2)); // mutation: 2 relegated',
     'mwe_5 veikkausliiga rules'),
    ('mwe5_ykkonen_sobe_1',
     'v.push((league::PROMOTED_COUNT, 1, 2)); // mwe5: the Ykkonen promotes 2',
     'v.push((league::PROMOTED_COUNT, 1, 1)); // mutation: 1 promoted',
     'mwe_5 ykkonen rules'),
]
