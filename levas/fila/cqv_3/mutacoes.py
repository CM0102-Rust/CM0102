# Mutacoes (cqv_3). As tres rodam uma vez na carreira de 8 ligas ate 2002-06-01, na virada (contadores "cqv_3 ..."):
# - o acesso aos estaduais com uma vaga a menos em cada divisao (outros clubes nos estaduais de 2002);
# - a Serie C com 15 que entram no lugar de 16 (um clube a menos na divisao nacional);
# - a virada da J2 dois anos adiante (os jogos da J2 de 2002 saem para 2003 e nao acontecem antes de 2002-06-01).
MUTACOES = [
    ('cqv3_acesso_uma_vaga_a_menos',
     'if vagas[e] > 0 { // cqv3: as vagas de acesso do estadual',
     'if vagas[e] > 1 { // mutacao: uma vaga a menos',
     'cqv_3 acesso ao estadual'),
    ('cqv3_serie_c_quinze',
     'for k in 0..0x10 { // cqv3: os 16 que entram na Série C',
     'for k in 0..0xf { // mutacao: quinze',
     'cqv_3 serie c renovada'),
    ('cqv3_j2_dois_anos',
     'cqv3_virada_j2(this, 1) // cqv3: o ano seguinte da J2',
     'cqv3_virada_j2(this, 2) // mutacao: dois anos adiante',
     'cqv_3 virada da j2'),
]
