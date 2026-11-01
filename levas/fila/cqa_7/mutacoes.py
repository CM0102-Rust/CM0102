# Mutacoes (cqa_7), no valor do jogador para o clube (0x840d80), que as listas de observacao e as ofertas do computador
# chamam para cada jogador avaliado (a nota pela vaga e o preco que o clube pagaria): o preco e a nota mudam quem entra
# nas listas, os lances e os valores das transferencias, que a carreira le (saldo, elencos).
# - o valor base da vaga comum com 0,30 no lugar de 0,32 (o preco de quase todos os jogadores avaliados cai);
# - o listado para transferencia de outro clube com a nota x 1,10 no lugar de 1,05 (muda a ordem das listas);
# - o fator da idade acima do divisor com 0,9 no lugar de 0,8 (o preco dos mais velhos cai).
# Os contadores ficam colados no efeito (no ramo do valor base comum, depois da condicao do listado e depois das
# condicoes da idade).
MUTACOES = [
    ('cqa7_valor_base_030',
     'cqa7_k(0x96f918)) as u32 // cqa7: o valor base',
     '0.30) as u32 // mutacao: o valor base com 0,30',
     'cqa_7 valor base'),
    ('cqa7_listado_110',
     '*saida = ftol(*saida as f64 * cqa7_k(0x9689b8)); // cqa7: o listado de outro clube × 1,05',
     '*saida = ftol(*saida as f64 * 1.10); // mutacao: o listado x 1,10',
     'cqa_7 listado para transferencia'),
    ('cqa7_idade_com_09',
     '(dd * cqa7_k(0x968e80) + cqa7_k(0x9689b0)); // cqa7: a idade acima do divisor',
     '(dd * cqa7_k(0x968e80) + 0.9); // mutacao: a idade com 0,9',
     'cqa_7 idade'),
]
