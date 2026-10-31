# Mutacoes (com o ramo de cada uma). Todas no pedido de transferencia do clube do computador (as outras duas rotinas
# so rodam na tela da comissao). Com o fator 1,2 no lugar de 1,5, menos jogadores entram na lista a pedido e mais caem
# no sorteio da queixa (a lista e os sorteios mudam a carreira). A moral cai 6 e nao 12 (a moral pesa nas escolhas de
# escalacao e de contrato). O sorteio da queixa contra 25 em vez de 50 da menos queixas 0x10000 (que podem pedir a
# transferencia de novo e sortear mais). Os ramos dizem se a carreira passou por eles.
MUTACOES = [
    ('cps1_fator_do_valor', 'if (v as f64) * 1.5 > a as f64 { // cps1: o valor passa do preço',
     'if (v as f64) * 1.2 > a as f64 { // mutacao: fator 1,2',
     'cps_1 listado pelo clube'),
    ('cps1_moral_6', 'moral(pessoa, 0xffff_fff4, 0); // cps1: a moral cai 12',
     'moral(pessoa, 0xffff_fffa, 0); // mutacao: cai 6',
     'cps_1 pedido no clube do computador'),
    ('cps1_queixa_25', 'if r < sorteio(0x32) { // cps1: o sorteio da queixa',
     'if r < sorteio(0x19) { // mutacao: contra 25',
     'cps_1 pedido no clube do computador'),
]
