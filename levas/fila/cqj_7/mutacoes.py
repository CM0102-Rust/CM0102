# Mutacoes (cqj_7): as regras do Goiano e da J-League, gravadas na montagem da carreira e [inferencia] na virada da
# temporada de ano civil. A J1 e a J2 jogam ate novembro/dezembro de 2001 e o Goiano no primeiro semestre de 2002, as
# tres antes de 2002-06-01: a tabela muda (pontos, posicoes, destinos) e a memoria do jogo (impressao diaria) com ela.
# - Goiano: o primeiro desempate passa do confronto direto (3) para o saldo (1).
# - J1: o empate vale 0.
# - J2: a vitoria vale 2.
MUTACOES = [
    ('cqj7_desempate_goiano', 'cqj7_b(o, 0xc5, 3); // cqj7: o primeiro desempate do Goiano',
     'cqj7_b(o, 0xc5, 1); // mutacao: o primeiro desempate do Goiano', 'cqj_7 goiano rules'),
    ('cqj7_empate_j1', 'cqj7_b(o, 0xc3, 1); // cqj7: os pontos por empate da J1',
     'cqj7_b(o, 0xc3, 0); // mutacao: os pontos por empate da J1', 'cqj_7 j1 rules'),
    ('cqj7_vitoria_j2', 'cqj7_b(o, 0xc2, 3); // cqj7: os pontos por vitória da J2',
     'cqj7_b(o, 0xc2, 2); // mutacao: os pontos por vitória da J2', 'cqj_7 j2 rules'),
]
