# Mutacoes (cqe_4), na vontade de ir para um clube (0x846920, 186136 chamadas ate 2002-06-01):
# - a base para outro clube cai de 2,75 para 2,5: a nota de todo jogador com contrato sem pedida em aberto avaliando
#   outro clube muda; os pedidos de emprestimo (minimo 3,5 em 0x842740), os lances e as respostas aos contratos mudam;
# - o fator da injury_word_0c comparada (e0 = dv x 1/6200 + 1) dobra a inclinacao: quem ganha injury_word_0c quer mais, quem
#   perde quer menos, em todas as avaliacoes de outro clube;
# - a pedida pelo interesse (q = 3,5 / nota) sobe 50%: o salario pedido passa do teto mais vezes (nota 0 ou 0,1) e a
#   nota com salario muda nas ofertas de contrato (ramo do salario: 2048+ ate 2001-09-01).
# Os contadores ficam colados no efeito: a base depois das recusas; o fator antes da conta; o salario depois das
# condicoes (salario pedido, sem emprestimo, nota de 1,75).
MUTACOES = [
    ('cqe4_base_2_5',
     'f8 = 2.75; // cqe4: a base para outro clube',
     'f8 = 2.5; // mutacao: base 2,5',
     'cqe_4 youth from another club'),
    ('cqe4_fator_reputacao_dobrado',
     'let mut e0 = dv * k(0x96f960) + k(0x967890); // cqe4: o fator da reputação comparada',
     'let mut e0 = dv * k(0x96f960) * 2.0 + k(0x967890); // mutacao: o dobro da inclinacao',
     'cqe_4 injury_word_0c factor'),
    ('cqe4_pedida_maior',
     'let mut q = k(0x968e38) / base; // cqe4: a pedida pelo interesse',
     'let mut q = k(0x968e38) * 1.5 / base; // mutacao: pedida 50% maior',
     'cqe_4 wage'),
]
