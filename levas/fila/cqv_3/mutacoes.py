# Mutacoes (cqv_3). As tres rodam uma vez na carreira de 8 ligas ate 2002-06-01, na virada (contadores "cqv_3 ..."):
# - o acesso aos estaduais sem abrir a vaga de tecnico do clube que sobe sem tecnico (outros tecnicos contratados; a
#   primeira tentativa, uma vaga a menos em cada divisao, derrubava o jogo em 2002-011 com um estadual mais curto);
# - a Serie C sem abrir a vaga de tecnico dos que entram sem tecnico (a primeira tentativa, 15 no lugar de 16, derrubava
#   o jogo em 2002-011: a Serie C precisa dos 56 clubes);
# - a virada da J2 dois anos adiante (os jogos da J2 de 2002 saem para 2003 e nao acontecem antes de 2002-06-01).
MUTACOES = [
    ('cqv3_acesso_sem_vaga_de_tecnico',
     'if cqv3_d(c + 0x57) != 0 && cqv3_d(c + 0xcf) == 0 {',
     'if false && cqv3_d(c + 0x57) != 0 && cqv3_d(c + 0xcf) == 0 { // mutacao: o que sobe sem tecnico nao abre a vaga',
     'cqv_3 acesso ao estadual'),
    ('cqv3_serie_c_sem_vaga_de_tecnico',
     'if cqv3_d(c + 0x60) != 0 && cqv3_d(c + 0xcf) == 0 {',
     'if false && cqv3_d(c + 0x60) != 0 && cqv3_d(c + 0xcf) == 0 { // mutacao: o que entra sem tecnico nao abre a vaga',
     'cqv_3 serie c renovada'),
    ('cqv3_j2_dois_anos',
     'cqv3_virada_j2(this, 1) // cqv3: o ano seguinte da J2',
     'cqv3_virada_j2(this, 2) // mutacao: dois anos adiante',
     'cqv_3 virada da j2'),
]
