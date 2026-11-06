# Mutacoes (cqr_1). Os turnos do Carioca, do Mineiro e da J1 rodam na carreira de 8 ligas ate 2002-06-01 (os de 2001
# logo no comeco da carreira): cada mutacao da a vitoria, na fase do turno, os pontos do empate (+0xc3 no lugar de
# +0xc2). A tabela do turno muda de ordem e o campeao do turno (o primeiro dela) pode ser outro: outra final, ou final
# onde nao havia (outros jogos e outra contagem de sorteios). O contador fica depois da fase gravada, com o nome de
# cada liga.
MUTACOES = [
    ('cqr1_carioca_vitoria_vale_empate',
     '0xc2, "cqr_1 turno do carioca") // cqr1: os pontos por vitória no Carioca',
     '0xc3, "cqr_1 turno do carioca") // mutacao: a vitoria vale o empate',
     'cqr_1 turno do carioca'),
    ('cqr1_mineiro_vitoria_vale_empate',
     '0xc2, "cqr_1 turno do mineiro") // cqr1: os pontos por vitória no Mineiro',
     '0xc3, "cqr_1 turno do mineiro") // mutacao: a vitoria vale o empate',
     'cqr_1 turno do mineiro'),
    ('cqr1_j1_vitoria_vale_empate',
     '0xc2, "cqr_1 turno da j1") // cqr1: os pontos por vitória na J1',
     '0xc3, "cqr_1 turno da j1") // mutacao: a vitoria vale o empate',
     'cqr_1 turno da j1'),
]
# Prova do agente (06/11, carreira Rust de 8 ligas ate 2002-06-01; base com os 12 itens cqr/cqs/cqt_1..4, impressao b690295afd222f6a): as tres ACUSADAS (Carioca e Mineiro na impressao final; J1 divergiu em 2001-136).
