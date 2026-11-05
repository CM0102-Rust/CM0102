# Mutacoes (cql_12), nas tabelas das supercopas (Franca, Espanha e Japao, ligas jogaveis na carreira de 8 ligas): o jogo
# da Supercopa da Franca uma semana depois (24/7 -> 31/7, ja dentro da janela, que comeca em 2001-205), a ida da Supercopa
# da Espanha uma semana antes (20/8 -> 13/8) e o jogo da Supercopa do Japao uma semana depois (4/3 -> 11/3 de 2002). O
# jogo noutro dia muda o resultado e os sorteios do dia. Os contadores ficam depois da alocacao, colados na gravacao.
MUTACOES = [
    ('cql12_supercopa_da_franca_uma_semana_depois',
     'cql12r((5, 6, 0, 5), (0x18, 6, 0, 5, 2, 0x400), 0,',
     'cql12r((5, 6, 0, 5), (0x1f, 6, 0, 5, 2, 0x400), 0,',
     'cql_12 tabela da supercopa da franca'),
    ('cql12_supercopa_da_espanha_uma_semana_antes',
     'cql12r((5, 6, 0, 5), (0x14, 7, 0, 6, 1, 0), 0,',
     'cql12r((5, 6, 0, 5), (0xd, 7, 0, 6, 1, 0), 0,',
     'cql_12 tabela da supercopa da espanha'),
    ('cql12_supercopa_do_japao_uma_semana_depois',
     'cql12r((1, 1, 0, 5), (4, 2, 0, 5, 1, 4), 0,',
     'cql12r((1, 1, 0, 5), (0xb, 2, 0, 5, 1, 4), 0,',
     'cql_12 tabela da supercopa do japao'),
]
