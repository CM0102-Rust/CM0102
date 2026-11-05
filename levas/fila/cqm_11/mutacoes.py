# Mutacoes (cqm_11), nos inscritos das copas francesas (a Franca e uma das 8 ligas): na Copa da Franca e na Copa da Liga
# francesa, a Primeira inscrita antes da Segunda (a ordem da lista decide em que rodada cada clube entra: a Primeira
# passaria a entrar no lugar da Segunda); na Supercopa, o vice da Primeira no lugar do campeao. As tres rodam uma vez no
# comeco da carreira (os construtores). Os contadores ficam colados no efeito.
MUTACOES = [
    ('cqm11_copa_da_franca_primeira_antes',
     '    cqm11_divisao(this, &mut n, 0x9cf5d0); // cqm11: a Segunda francesa na Copa da França\n    cqm11_divisao(this, &mut n, 0x9cf5cc); // cqm11: a Primeira francesa na Copa da França',
     '    cqm11_divisao(this, &mut n, 0x9cf5cc); // mutacao: a Primeira antes\n    cqm11_divisao(this, &mut n, 0x9cf5d0); // mutacao: a Segunda depois',
     'cqm_11 inscritos da copa da franca'),
    ('cqm11_copa_da_liga_primeira_antes',
     '    cqm11_divisao(this, &mut n, 0x9cf5d0); // cqm11: a Segunda francesa na Copa da Liga\n    cqm11_divisao(this, &mut n, 0x9cf5cc); // cqm11: a Primeira francesa na Copa da Liga',
     '    cqm11_divisao(this, &mut n, 0x9cf5cc); // mutacao: a Primeira antes\n    cqm11_divisao(this, &mut n, 0x9cf5d0); // mutacao: a Segunda depois',
     'cqm_11 inscritos da copa da liga francesa'),
    ('cqm11_supercopa_com_o_vice',
     'let a = cqm11_colocado(0x9cf5cc, 0); // cqm11: o campeão francês',
     'let a = cqm11_colocado(0x9cf5cc, 1); // mutacao: o vice',
     'cqm_11 clubes da supercopa francesa'),
]
