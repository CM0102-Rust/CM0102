# Mutacoes (cqs_4). A final da Serie C de 2001 e a virada para 2002 rodam uma vez cada na carreira de 8 ligas ate
# 2002-06-01; as posicoes previstas, duas (na criacao da carreira e na virada):
# - a final com os campeoes trocados (o do grupo B primeiro na lista): outro mandante no primeiro jogo [inferencia];
# - a virada sem a renovacao dos arbitros da Serie C (ela sorteia arbitros novos: outra contagem de sorteios);
# - as posicoes previstas sem ordenar a lista dos 56 pela reputacao (as posicoes seguem a ordem do banco): outras
#   previsoes (na Italia, tirar as posicoes previstas foi acusado no primeiro dia, cqm_4).
MUTACOES = [
    ('cqs4_final_trocada',
     '    lista[0] = cqs4_primeiro(this, 8); // cqs4: o campeão do grupo A\n    lista[1] = cqs4_primeiro(this, 9);\n',
     '    lista[0] = cqs4_primeiro(this, 9); // mutacao: o campeao do grupo B primeiro\n    lista[1] = cqs4_primeiro(this, 8);\n',
     'cqs_4 final'),
    ('cqs4_virada_sem_arbitros',
     'renovar(cqs4_d(0xb74340), cqs4_d(this + 4)); // cqs4: a renovação dos árbitros da Série C',
     'let _ = renovar; // mutacao: sem a renovacao dos arbitros',
     'cqs_4 virada da serie c'),
    ('cqs4_previstas_sem_ordenar',
     'ordenar(cqs4_d(this + 0xee), 0x38, 4, 0x4b07d0); // cqs4: a lista dos 56 pela reputação',
     'let _ = ordenar; // mutacao: sem ordenar pela reputacao',
     'cqs_4 posicoes previstas'),
]
# Prova do agente (06/11, carreira Rust de 8 ligas ate 2002-06-01; base com os 12 itens cqr/cqs/cqt_1..4, impressao b690295afd222f6a): as tres ACUSADAS (a final em 2001-343, a virada em 2002-012, as previstas em 2001-001).
