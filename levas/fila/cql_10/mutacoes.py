# Mutacoes (cql_10), nas tabelas de rodadas das copas inglesas (a Inglaterra e uma das 8 ligas): o sorteio da 2a rodada
# da Copa da Liga uma semana depois (23/8 -> 30/8; o jogo e em 20/9), o sorteio da 2a rodada da Copa da Inglaterra uma
# semana depois (19/11 -> 26/11; o jogo e em 9/12) e o jogo da Supercopa uma semana antes (13/8 -> 6/8). Todas caem na
# janela (a carreira comeca em 2001-205 e vai ate 2002-06-01): o sorteio num outro dia muda a ordem dos sorteios do dia,
# o jogo noutro dia muda o resultado. Os contadores ficam depois da alocacao, colados na gravacao das rodadas.
MUTACOES = [
    ('cql10_copa_da_liga_sorteio_uma_semana_depois',
     'cql10r((0x17, 7, 0, 2), (0x14, 8, 0, 2, 2, 0), 0x14,',
     'cql10r((0x1e, 7, 0, 2), (0x14, 8, 0, 2, 2, 0), 0x14,',
     'cql_10 tabela da copa da liga'),
    ('cql10_copa_da_inglaterra_sorteio_uma_semana_depois',
     'cql10r((0x13, 0xa, 0, 6), (9, 0xb, 0, 5, 1, 0), 0x14,',
     'cql10r((0x1a, 0xa, 0, 6), (9, 0xb, 0, 5, 1, 0), 0x14,',
     'cql_10 tabela da copa da inglaterra'),
    ('cql10_supercopa_uma_semana_antes',
     'cql10r((5, 6, 0, 5), (0xd, 7, 0, 6, 1, 4), 0,',
     'cql10r((5, 6, 0, 5), (6, 7, 0, 6, 1, 4), 0,',
     'cql_10 tabela da supercopa inglesa'),
]
