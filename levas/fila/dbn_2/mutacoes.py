# Mutacoes da carga do banco. A prova e o .qst da prova do save (a carga e a gravacao passam pela 0x5315a0; o vetor de
# clubes vai para o save) e a carga do FeriasTesteSave (CARGA). dbn2_cidade_sem_nacao: a cidade fica sem nacao (o .qst
# grava -1); se algo da criacao ler a nacao da cidade, derruba o jogo (tambem acusada).
MUTACOES = [
    ('dbn2_cidade_sem_nacao', '        a(e + 0x1f, 0xae23a8, 0x122); // a nação da cidade', '        wr(e + 0x1f, 0); // mutacao: a cidade sem nacao'),
    ('dbn2_ordenar', '    ordenar(rd(0xae2d1c), gi(0xae235c) as u32, 4, 0x540020); // todos os clubes pela reputação', '    ordenar(rd(0xae2d1c), (gi(0xae235c) - 1) as u32, 4, 0x540020); // mutacao: o ultimo clube fica fora da ordem'),
]
