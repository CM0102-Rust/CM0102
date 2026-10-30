# Mutacoes (com o ramo de cada uma). A compensacao do jogador abaixo de 24 anos entre clubes britanicos vira um quarto do
# valor (o preco pedido 0x4d1600 e os lances da IA por ele mudam). O vendedor nao recebe a primeira parcela (o saldo
# do clube vendedor fica menor: verbas, salarios e transferencias seguintes mudam). Toda multa conta como ativa (sem
# clausula, o +0x21 do contrato e o piso: o vendedor passa a ter de aceitar ofertas que recusava). Os ramos dizem se
# a carreira passou por eles.
MUTACOES = [
    ('cpq6_britanica', 'valor(gerente(), pessoa, vendedor, 1, 0xffff_ffff, 0) / 2 // menor de 24 anos entre clubes britânicos',
     'valor(gerente(), pessoa, vendedor, 1, 0xffff_ffff, 0) / 4 // mutacao: um quarto',
     'cpq_6 compensacao britanica'),
    ('cpq6_vendedor_recebe', 'receber(fin_v, primeira); // o vendedor recebe',
     'let _ = receber; // mutacao: o vendedor nao recebe',
     'cpq_6 pagar transferencia'),
    ('cpq6_multa', 'if ativa(c) != 0 && cpq6_d(oferta + 4) == 0',
     'if (ativa(c) | 1) != 0 && cpq6_d(oferta + 4) == 0',
     'cpq_6 clube deve aceitar'),
]
