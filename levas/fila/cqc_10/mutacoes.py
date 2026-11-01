# Mutacoes (cqc_10), todas na 0x754a30 (321 chamadas ate 2002-06-01 na cobertura de 17/10, uma por dia, cada uma por todos
# os registros de interesse). Os boatos sorteiam, marcam os registros, chamam a reacao do elenco (desafeicoes, marcas
# do contrato) e a resposta a historia, que mudam o banco e a contagem de sorteios da carreira.
# - O interesse vale 45 dias em vez de 31: menos registros saem, mais sorteios de boato.
# - O sorteio do boato passa de 25 a 20: mais boatos.
# - O sorteio da reputacao para o pedido de transferencia passa de 12500 a 10000: mais pedidos (e reacoes do elenco).
MUTACOES = [
    ('cqc10_interesse_45_dias', '        let p = mais(r + 8, fim.as_mut_ptr(), 0x1f); // cqc10: o interesse vale 31 dias',
     '        let p = mais(r + 8, fim.as_mut_ptr(), 0x2d); // mutacao: o interesse vale 31 dias', 'cqc_10 interesse'),
    ('cqc10_boato_20', '            cqc10_sortear(0x19) == 0 // cqc10: o sorteio do boato',
     '            cqc10_sortear(0x14) == 0 // mutacao: o sorteio do boato', 'cqc_10 sorteio do boato'),
    ('cqc10_pedido_10000', '                let rr = cqc10_sortear(0x30d4); // cqc10: o sorteio da reputação para o pedido',
     '                let rr = cqc10_sortear(0x2710); // mutacao: o sorteio da reputação para o pedido', 'cqc_10 pedido de transferencia'),
]
