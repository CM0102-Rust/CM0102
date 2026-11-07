# Mutacoes (cqu_1). Os quatro construtores rodam uma vez na criacao da carreira de 8 ligas (contadores "cqu_1 <liga> nova"):
# - a Serie A um ano adiante (os jogos da Serie A de 2001 saem para 2002: outros jogos e outra contagem de sorteios);
# - a Serie B com outra regra de disciplina (0 no lugar de 6: outras suspensoes nos jogos da Serie B);
# - a J1 um ano adiante (os jogos da J1 saem para o ano seguinte);
# - a NSL com outra regra de disciplina (0 no lugar de 0x22).
MUTACOES = [
    ('cqu1_serie_a_um_ano_adiante',
     'cqu1_construir(this, ano, registro, &CQU1_SERIE_A,',
     'cqu1_construir(this, ano + 1, registro, &CQU1_SERIE_A, /* mutacao: um ano adiante */',
     'cqu_1 serie_a nova'),
    ('cqu1_serie_b_outra_disciplina',
     '&CQU1_SERIE_B, 6, "cqu_1 serie_b nova")',
     '&CQU1_SERIE_B, 0, "cqu_1 serie_b nova") /* mutacao: outra disciplina */',
     'cqu_1 serie_b nova'),
    ('cqu1_j1_um_ano_adiante',
     'cqu1_construir(this, ano, registro, &CQU1_J1,',
     'cqu1_construir(this, ano + 1, registro, &CQU1_J1, /* mutacao: um ano adiante */',
     'cqu_1 j1 nova'),
    ('cqu1_nsl_outra_disciplina',
     '&CQU1_NSL, 0x22, "cqu_1 nsl nova")',
     '&CQU1_NSL, 0, "cqu_1 nsl nova") /* mutacao: outra disciplina */',
     'cqu_1 nsl nova'),
]
# Prova do agente (07/11, carreira Rust de 8 ligas ate 2002-06-01 com motor de lances; base com os 12 itens cqu/cqv/cqw, impressao f29c85195c2a534f): as quatro ACUSADAS (2001-189, 2001-216, 2001-057, 2001-304).
