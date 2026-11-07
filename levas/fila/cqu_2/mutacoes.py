# Mutacoes (cqu_2). Os tres construtores rodam uma vez na criacao da carreira de 8 ligas (contadores "cqu_2 <liga> nova"):
# - a Serie C um ano adiante (os grupos e os jogos da Serie C de 2001 saem para 2002);
# - a J2 com outra regra de disciplina (0 no lugar de 0x13: outras suspensoes nos jogos da J2);
# - a MLS com 2 pontos por vitoria no lugar de 3 (outra tabela da MLS de 2001: outros classificados ao mata-mata).
MUTACOES = [
    ('cqu2_serie_c_um_ano_adiante',
     'cqu2_construir(this, ano, registro, &CQU2_SERIE_C,',
     'cqu2_construir(this, ano + 1, registro, &CQU2_SERIE_C, /* mutacao: um ano adiante */',
     'cqu_2 serie_c nova'),
    ('cqu2_j2_outra_disciplina',
     '&CQU2_J2, 0x13, "cqu_2 j2 nova")',
     '&CQU2_J2, 0, "cqu_2 j2 nova") /* mutacao: outra disciplina */',
     'cqu_2 j2 nova'),
    ('cqu2_mls_dois_pontos',
     '(0xc2, 1, 3), // cqu2: os pontos por vitória na MLS',
     '(0xc2, 1, 2), // mutacao: dois pontos por vitoria',
     'cqu_2 mls nova'),
]
# Prova do agente (07/11, carreira Rust de 8 ligas ate 2002-06-01 com motor de lances; base com os 12 itens cqu/cqv/cqw, impressao f29c85195c2a534f): as tres ACUSADAS (2001-215, 2001-084, 2001-120).
