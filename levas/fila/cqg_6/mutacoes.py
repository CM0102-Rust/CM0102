# Mutacoes (cqg_6), no sorteio dos grupos da segunda fase da Liga dos Campeoes (0x585ab0, uma vez na carreira
# automatica ate 2002-06-01, em 6/11/2001, pela fase seguinte 0x5851d0):
# - o premio da segunda fase pela metade (600.000): o saldo dos 16 clubes muda;
# - um sorteio a mais em cada troca da ordem dos grupos: a contagem dos sorteios da carreira muda (e os grupos);
# - so o grupo da primeira fase impede o grupo (a nacao deixa de impedir): outros grupos e outros jogos de 2002.
# Os contadores ficam colados no efeito: o premio no laco dos clubes; a troca dentro do laco das 5 trocas; o grupo
# escolhido depois da escolha.
MUTACOES = [
    ('cqg6_premio_pela_metade',
     'let valor = 1_200_000i32; // cqg6: o prêmio da segunda fase',
     'let valor = 600_000i32; // mutacao: a metade do premio',
     'cqg_6 second stage prize'),
    ('cqg6_sorteio_a_mais_na_troca',
     'let b = sorteio(4) as i8 as usize; // cqg6: a troca da ordem dos grupos',
     'let b = { sorteio(4); sorteio(4) } as i8 as usize; // mutacao: um sorteio a mais',
     'cqg_6 order swap'),
    ('cqg6_nacao_nao_impede',
     'if cqg6_b(v + 5) == cqg6_b(p + 5) || cqg6_nacao(clube) == cqg6_nacao(cqg6_d(p)) { // cqg6: mesmo grupo ou nação',
     'if cqg6_b(v + 5) == cqg6_b(p + 5) { // mutacao: so o grupo da primeira fase',
     'cqg_6 group chosen'),
]
