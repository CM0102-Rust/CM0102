# Mutacoes (cqb_6), na parte dos clubes anteriores (0x8e4f20), que a copia Rust da transferencia efetivada (0x8c9390)
# chama em toda transferencia com valor de jogador (a base da 261a tem ~256 vendas pelo contador "cpq_4 venda"):
# - so acima de 30 anos: quase nenhum jogador vendido paga a parte; o comprador deixa de receber e o saldo dele muda
#   (o saldo entra na verba de transferencias e nas decisoes da diretoria);
# - a parte sem dividir pela soma dos pesos: o comprador recebe varias vezes mais;
# - a parte sai pelas compras em vez de entrar nas vendas: o saldo do comprador cai em vez de subir.
# Os contadores ficam colados no efeito: o filtro da idade e o pagamento de cada vaga.
MUTACOES = [
    ('cqb6_so_acima_de_30',
     'if (cqb6_b(p + 0x18) as i8) < 0x17 { // cqb6: só acima de 22 anos',
     'if (cqb6_b(p + 0x18) as i8) < 0x1f { // mutacao: so acima de 30 anos',
     'cqb_6 parte dos clubes anteriores'),
    ('cqb6_parte_sem_dividir',
     'let valor = vinte.wrapping_div(divisor).wrapping_mul(peso); // cqb6: a parte da vaga',
     'let valor = vinte.wrapping_mul(peso); // mutacao: sem dividir pela soma dos pesos',
     'cqb_6 parte paga'),
    ('cqb6_parte_sai_do_comprador',
     'post_finance_amount(0x599800, fin, valor); // cqb6: entra nas vendas do comprador',
     'post_finance_amount(0x599890, fin, valor); // mutacao: sai pelas compras do comprador',
     'cqb_6 parte paga'),
]
