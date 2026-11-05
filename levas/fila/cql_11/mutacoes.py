# Mutacoes (cql_11), nas tabelas de rodadas das copas da Franca e da Espanha (ligas jogaveis na carreira de 8 ligas):
# o sorteio da 7a rodada da Copa da Franca uma semana depois (12/11 -> 19/11; o jogo e em 27/11), o sorteio da 1a
# rodada da Copa da Liga francesa uma semana depois (15/10 -> 22/10; o jogo e em 16/11) e o sorteio da 2a rodada da
# Copa do Rei uma semana depois (15/12 -> 22/12; o jogo e em 3/1). Todos na janela (2001-205 a 2002-06-01): o sorteio
# noutro dia muda a ordem dos sorteios. Os contadores ficam depois da alocacao, colados na gravacao das rodadas.
MUTACOES = [
    ('cql11_copa_da_franca_sorteio_uma_semana_depois',
     'cql11r((0xc, 0xa, 0, 4), (0x1b, 0xa, 0, 5, 2, 0), 0x46,',
     'cql11r((0x13, 0xa, 0, 4), (0x1b, 0xa, 0, 5, 2, 0), 0x46,',
     'cql_11 tabela da copa da franca'),
    ('cql11_copa_da_liga_francesa_sorteio_uma_semana_depois',
     'cql11r((0xf, 9, 0, 4), (0x10, 0xa, 0, 1, 2, 0), 0xa,',
     'cql11r((0x16, 9, 0, 4), (0x10, 0xa, 0, 1, 2, 0), 0xa,',
     'cql_11 tabela da copa da liga francesa'),
    ('cql11_copa_do_rei_sorteio_uma_semana_depois',
     'cql11r((0xf, 0xb, 0, 4), (3, 0, 1, 2, 2, 0), 0x14,',
     'cql11r((0x16, 0xb, 0, 4), (3, 0, 1, 2, 2, 0), 0x14,',
     'cql_11 tabela da copa do rei'),
]
