# Mutacoes da copa (cpb_3). A do premio paga ao perdedor o valor do vencedor: muda o saldo dos clubes eliminados (574
# chamadas), que pesa no orcamento e nas decisoes do computador ate 2002-06-01; a carreira deve acusar (pode escapar se
# o saldo nao mudar nenhuma decisao). A dos mandantes deixa de trocar os times (troca so os campos +0xa/+0xe): so muda
# algo se alguma copa da carreira inverter mandante (regra da rodada); PODE ESCAPAR; o comparando acusa na primeira
# inversao.
MUTACOES = [
    ('cpb_premio_perdedor', '        pagar(perdedor, ao_perdedor);', '        pagar(perdedor, ao_vencedor); // mutacao: o valor do vencedor'),
    ('cpb_mandantes_sem_times', '    for (a, b) in [(0usize, 2usize), (0xa, 0xc), (0xe, 0x10)] {', '    for (a, b) in [(0xausize, 0xcusize), (0xe, 0x10)] {'),
]
