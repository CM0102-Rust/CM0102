# Mutacoes (sbE_10), com a Polonia ate 2002-08-01 (a tabela na criacao do jogo, a fase final em novembro de 2001):
# - as sementes 1 e 2 trocadas (outros clubes em cada grupo: a carreira);
# - os pontos levados inteiros para a fase final (outro campeao e outros rebaixados em maio de 2002: a carreira);
# - os grupos da fase final trocados (a ordem pelo cx13_5 poe o grupo de rebaixamento na frente: a carreira).
MUTACOES = [
    ('sbE_10_sementes_trocadas',
     'pub(crate) const POL_FIRST_SEEDS: [u8; 16] = [1, 4, 5, 8, 9, 12, 13, 16, 2, 3, 6, 7, 10, 11, 14, 15];',
     'pub(crate) const POL_FIRST_SEEDS: [u8; 16] = [2, 4, 5, 8, 9, 12, 13, 16, 1, 3, 6, 7, 10, 11, 14, 15]; // mutation',
     'sbE_10 polish first division table'),
    ('sbE_10_pontos_inteiros',
     'let half = |x: i16| ((x as i32 + 1) / 2) as i16;',
     'let half = |x: i16| x; // mutation: the full points',
     'sbE_10 polish final stage'),
    ('sbE_10_grupos_trocados',
     'comp_record_of(key)); // sbE_10: the final stage group',
     'comp_record_of(KEY_CLUB_COMP_IRISH_GROUP_A + KEY_CLUB_COMP_IRISH_GROUP_B - key)); // mutation: the groups swapped',
     'sbE_10 polish final stage'),
]
