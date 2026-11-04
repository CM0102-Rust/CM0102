# Mutações (cqi_3), na Copa da UEFA e na virada da classificação no começo da carreira (uma vez; com as 8 ligas até
# 2002-06-01 decidem quem joga a Copa da UEFA e a Intertoto de 2001/02):
# - as 6 primeiras nações com 2 vagas na primeira fase da Copa da UEFA em vez de 3 (contador "cqi_3 tres na primeira
#   fase");
# - sem a vaga do fair play: as três nações sorteadas não ganham a vaga na preliminar (contador "cqi_3 fair play");
# - a virada sem as vagas da Intertoto: nenhum clube na Intertoto de 2001 (contador "cqi_3 virada da classificacao").
MUTACOES = [
    ('cqi3_duas_na_primeira_fase',
     '(3, 0) // cqi3: 1ª a 6ª nação',
     '(2, 0) // mutacao: 2 vagas',
     'cqi_3 tres na primeira fase'),
    ('cqi3_sem_fair_play',
     'vagas[2] = vagas[2].wrapping_add(1); // cqi3: a vaga do fair play',
     '// mutacao: sem o fair play',
     'cqi_3 fair play'),
    ('cqi3_sem_intertoto',
     'intertoto(this, t, n); // cqi3: as vagas da Intertoto',
     'let _ = intertoto; // mutacao: sem a Intertoto',
     'cqi_3 virada da classificacao'),
]
