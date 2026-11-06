# Mutacoes (cqs_2). A virada da MLS roda uma vez na carreira de 8 ligas ate 2002-06-01 (a MLS de 2002) e as posicoes
# previstas duas vezes (na criacao da carreira e na virada):
# - a virada dois anos adiante: as regras e os jogos da MLS saem para 2003, e os jogos de 2002 nao acontecem antes de
#   2002-06-01 (outra contagem de partidas e de sorteios);
# - a virada sem a renovacao dos arbitros da MLS (ela sorteia arbitros novos: outra contagem de sorteios);
# - as posicoes previstas da primeira divisao de tras para frente (o favorito previsto em 10o): outras expectativas das
#   diretorias e outra reputacao prevista (na Italia, tirar as posicoes previstas foi acusado no primeiro dia, cqm_4).
MUTACOES = [
    ('cqs2_virada_dois_anos',
     'cqs2_virada(this, 1) // cqs2: o ano seguinte da MLS',
     'cqs2_virada(this, 2) // mutacao: dois anos adiante',
     'cqs_2 virada da mls'),
    ('cqs2_virada_sem_arbitros',
     'renovar(cqs2_d(0xb74340), cqs2_d(this + 4)) // cqs2: a renovação dos árbitros da MLS',
     '{ let _ = renovar; 1 } // mutacao: sem a renovacao dos arbitros',
     'cqs_2 virada da mls'),
    ('cqs2_previstas_invertidas',
     'let lugares: [u32; 12] = [1, 4, 7, 10, 1, 4, 7, 10, 1, 4, 7, 10]; // cqs2: as posições previstas das três divisões',
     'let lugares: [u32; 12] = [10, 7, 4, 1, 1, 4, 7, 10, 1, 4, 7, 10]; // mutacao: a primeira divisao invertida',
     'cqs_2 posicoes previstas'),
]
