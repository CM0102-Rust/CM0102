# Mutacoes (cqg_10), no amistoso cancelado (0x5c6570, 3 chamadas ate 2002-06-01; o ramo do jogo de clube que sai do
# calendario rodou 2 vezes na carreira automatica de 02/11): o amistoso que fica no calendario e jogado (sorteios da
# partida). A vespera da 0x5cb9e0 e a vaga do amistoso nao rodaram nessa carreira: a terceira pode escapar.
MUTACOES = [
    ('cqg10_amistoso_fica',
     'tirar(cqg10_d(0xae2a58), jogo); // cqg10: o amistoso sai do calendário',
     'let _ = tirar; // mutacao: o amistoso fica no calendario',
     'cqg_10 amistoso fora do calendario'),
    ('cqg10_rodada_invertida',
     'if cqg10_b(jogo + 0x43) == 0xff { // cqg10: jogo sem rodada',
     'if cqg10_b(jogo + 0x43) != 0xff { // mutacao: a match_condition da rodada invertida',
     'cqg_10 amistoso fora do calendario'),
    ('cqg10_dois_dias_antes',
     'let v = menos(p0, p1, 1); // cqg10: a véspera',
     'let v = menos(p0, p1, 2); // mutacao: dois dias antes',
     'cqg_10 jogo da selecao na vespera'),
]
