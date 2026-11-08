# Mutacoes. A 0x7c0d70 roda 19.414 vezes na carreira automatica ate 2002-06-01 (cobertura de 17/10): todo jogo soma
# nos registros das categorias. O motor de partida le as categorias 2, 3 e 4 (jogos +0 e nota +0xe) pela get_stat
# 0x7c1e30: com a nota fora da soma, sem contar o jogo ou com as categorias da liga trocadas (2 <-> 3), as medias dos
# jogadores mudam e os jogos seguintes saem diferentes (a impressao das partidas pega). O ramo do jogo fica no
# primeiro jogador que conta da chamada; o da categoria, no tipo 1 da competicao (liga), colado na escolha.
MUTACOES = [
    ('cpx12_sem_nota',
     'add_u16_at(r + 0xe, nota as u16); // cpx12: a nota',
     '// mutacao: a nota fora da soma',
     'cpx_12 match recorded'),
    ('cpx12_sem_jogo',
     'add_u8_at(r, 1); // cpx12: o jogo',
     '// mutacao: o jogo fora da conta',
     'cpx_12 match recorded'),
    ('cpx12_categorias_da_liga_trocadas',
     'if conta(cpx12_d(p) as i32) != 0 { 2 } else { 3 } // cpx12: a categoria da liga',
     'if conta(cpx12_d(p) as i32) != 0 { 3 } else { 2 } // mutacao: as categorias da liga trocadas',
     'cpx_12 league category'),
]
