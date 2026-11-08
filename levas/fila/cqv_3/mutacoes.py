# Mutacoes (cqv_3). As tres rodam uma vez na carreira de 8 ligas ate 2002-06-01, na virada (contadores "cqv_3 ..."):
# - o acesso aos estaduais com a lista de tras para frente (sobem os de menor injury_word_0c: outros clubes nos estaduais de
#   2002, com as mesmas contagens); as tentativas anteriores: uma vaga a menos em cada divisao derrubava o jogo em
#   2002-011 (um estadual mais curto) e sem abrir a vaga de tecnico escapou (os que sobem ja tem tecnico, ou a vaga de
#   clube de baixa injury_word_0c nao faz nada);
# - a Serie C sem abrir a vaga de tecnico dos que entram sem tecnico (a primeira tentativa, 15 no lugar de 16, derrubava
#   o jogo em 2002-011: a Serie C precisa dos 56 clubes);
# - a virada da J2 dois anos adiante (os jogos da J2 de 2002 saem para 2003 e nao acontecem antes de 2002-06-01).
MUTACOES = [
    ('cqv3_acesso_de_tras_para_frente',
     'for k in 0..n.max(0) as usize { // cqv3: a lista na ordem',
     'for k in (0..n.max(0) as usize).rev() { // mutacao: a lista de tras para frente (sobem os de menor injury_word_0c)',
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
# Prova do agente (07/11, carreira Rust de 8 ligas ate 2002-06-01 com motor de lances; base com os 10 itens finais): as tres ACUSADAS (acesso de tras para frente em 2002-012, Serie C sem a vaga de tecnico em 2002-012, J2 dois anos em 2002-067); as primeiras tentativas (uma vaga a menos, quinze na Serie C) derrubavam o jogo e a sem vaga de tecnico no acesso escapou.
