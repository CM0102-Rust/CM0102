# Mutacoes (cqz_1), nas regras montadas no construtor: a vitoria da Copa da Asia valendo 2 (muda o objeto da copa na memória do jogo, o banco;
# a copa de 2004 nao joga ate 2002-06-01); o sorteio da Copa da Oceania de 2002 em 5/3 em vez de 5/2 (o sorteio de 2002
# muda de data); o sorteio olimpico no dia 27 em vez de 20 (7 dias; muda a data gravada na copa).
MUTACOES = [
    ('cqz1_asia_vitoria_2',
     'let win = 3; // cqz1: the win in the Asian Cup groups',
     'let win = 2; // mutation: 2 points a win',
     'cqz_1 asian cup rules'),
    ('cqz1_oceania_sorteio_marco',
     'let month = 2; // cqz1: the month of the Oceania Nations Cup draw date',
     'let month = 3; // mutation: the draw a month later',
     'cqz_1 oceania nations cup rules'),
    ('cqz1_olimpiada_dia_27',
     'let day = 0x14; // cqz1: the day of the Olympic draw date',
     'let day = 0x1b; // mutation: 7 days later',
     'cqz_1 olympics rules'),
]
