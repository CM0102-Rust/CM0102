# Mutacoes (cqg_4), no sorteio dos grupos da primeira fase da Liga dos Campeoes (0x585290, uma vez na carreira
# automatica ate 2002-06-01, no fim de agosto de 2001, pela fase seguinte 0x5851d0):
# - o clube nao guarda o parceiro da mesma nacao (so o parceiro guarda o dele): as metades dos clubes da mesma nacao
#   deixam de ser opostas e os grupos (e os jogos de setembro em diante) mudam;
# - um sorteio a mais antes de cada clube sorteado do pote: a contagem dos sorteios da carreira muda (e os grupos);
# - o premio da fase de grupos pela metade (900.000): o saldo dos 32 clubes muda.
# Os contadores ficam colados no efeito: o parceiro depois da condicao da nacao; o clube sorteado depois do laco da repeticao; o premio no laco dos clubes de cada grupo.
MUTACOES = [
    ('cqg4_sem_parceiro_de_um_lado',
     'cqg4_pd(e + 4, cqg4_d(o)); // cqg4: o parceiro da mesma nação',
     'cqg4_pd(e + 4, 0); // mutacao: sem o parceiro deste lado',
     'cqg_4 parceiro da mesma nação'),
    ('cqg4_sorteio_a_mais_no_pote',
     'let mut s = sorteio(8).wrapping_add(base); // cqg4: o clube sorteado do pote',
     'let mut s = { sorteio(8); sorteio(8) }.wrapping_add(base); // mutacao: um sorteio a mais',
     'cqg_4 clube sorteado'),
    ('cqg4_premio_pela_metade',
     'let valor = 1_800_000i32; // cqg4: o prêmio da fase de grupos',
     'let valor = 900_000i32; // mutacao: a metade do premio',
     'cqg_4 prêmio da fase de grupos'),
]
