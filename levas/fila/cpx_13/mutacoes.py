# Mutacoes. A 0x76bab0 roda 1.363 vezes na carreira automatica ate 2002-06-01 (cobertura de 17/10) e pede o elenco
# (0x76c120) 1.007 vezes: os convocados deixam o clube nos dias do jogo de selecao (e a 0x776fc0 nao marca jogo de
# clube com convocados perto da data). Sem pedir elenco, com o elenco valendo por 60 dias em vez de 14 ou com a janela
# de uma semana em vez de duas, os convocados e as datas mudam e a carreira segue outra (escalacoes dos clubes,
# cansaco, sorteios do motor). O ramo fica colado no pedido, depois de todas as condicoes; a janela e o limit usam o
# mesmo ramo (o pedido so acontece dentro da janela e acima do limit).
MUTACOES = [
    ('cpx13_sem_elenco',
     'escolher(this, cpx13_d(clube + 0x53), tipo, 0xae2c90, p, 0); // cpx13: o elenco',
     'let _ = escolher; // mutacao: sem pedir elenco',
     'cpx_13 squad requested'),
    ('cpx13_elenco_de_60_dias',
     'if d <= 0xe && cpx13_b(base + 0x11) != 0 && extra() {',
     'if d <= 60 && cpx13_b(base + 0x11) != 0 && extra() { // mutacao: o elenco vale 60 dias',
     'cpx_13 squad requested'),
    ('cpx13_janela_de_uma_semana',
     'let q = semanas_antes(jogo.as_mut_ptr(), r.as_mut_ptr(), 2); // cpx13: a janela',
     'let q = semanas_antes(jogo.as_mut_ptr(), r.as_mut_ptr(), 1); // mutacao: a janela de uma semana',
     'cpx_13 squad requested'),
]
