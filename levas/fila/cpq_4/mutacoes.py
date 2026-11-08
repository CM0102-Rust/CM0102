# Mutacoes dos registros de carreira (cpq_4). Todas na transferencia (milhares na carreira ate 2002-06-01): a maior
# compra sem o valor, as compras contadas de 2 em 2 e as vendas sem a receita. Os registros dos tecnicos ficam fora do
# banco e a carreira so os le no tecnico do mes e do ano: a carreira PODE NAO PEGAR; o comparando acusa na primeira
# transferencia (lado a lado pelos dois registros). A eliminacao e a nao classificacao so gravam o historico em arquivo
# (fora da impressao) e acabam convocacoes so nas competicoes guardadas: sem mutacao util.
MUTACOES = [
    ('cpq4_maior_compra_sem_valor', '                cpq4_pi(r + 0x18, valor);',
     '                // mutacao: a maior compra sem o valor',
     'cpq_4 biggest purchase'),
    ('cpq4_compras_de_2_em_2', '            cpq4_pw(r + 0xa0, cpq4_w(r + 0xa0).wrapping_add(1));',
     '            cpq4_pw(r + 0xa0, cpq4_w(r + 0xa0).wrapping_add(2)); // mutacao: de 2 em 2',
     'cpq_4 purchase'),
    ('cpq4_venda_sem_receita', '            staff_records_add_i64(r + 0xac, valor);\n            if cpq4_d(t + 4) != 0 {\n                cpq4_pw(r + 0xa0,',
     '            // mutacao: sem a receita\n            if cpq4_d(t + 4) != 0 {\n                cpq4_pw(r + 0xa0,',
     'cpq_4 sale'),
]
