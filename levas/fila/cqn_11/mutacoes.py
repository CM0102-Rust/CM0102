# Mutacoes (cqn_11), nas tabelas de rodadas das copas do Japao e dos EUA (ligas jogaveis na carreira de 8 ligas): o
# sorteio da 1a rodada da Copa da Liga japonesa uma semana depois (10/2 -> 17/2 de 2002; o jogo e em 12/4), o sorteio da
# 1a rodada da Copa do Imperador uma semana depois (1/11 -> 8/11 de 2001; o jogo e em 29/11) e o sorteio da semifinal da
# Copa dos EUA uma semana depois (10/8 -> 17/8 de 2001; o jogo e em 23/8). O sorteio noutro dia muda a ordem dos sorteios
# do dia. Os contadores ficam depois da alocacao, colados na gravacao das rodadas.
MUTACOES = [
    ('cqn11_copa_da_liga_japonesa_sorteio_uma_semana_depois',
     'cqn11r((0xa, 1, 0, 0xffff_ffff), (0xc, 3, 0, 2, 2, 0), 0xa,',
     'cqn11r((0x11, 1, 0, 0xffff_ffff), (0xc, 3, 0, 2, 2, 0), 0xa,',
     'cqn_11 tabela da copa da liga japonesa'),
    ('cqn11_copa_do_imperador_sorteio_uma_semana_depois',
     'cqn11r((1, 0xa, 0, 6), (0x1d, 0xa, 0, 6, 1, 0), 0xa,',
     'cqn11r((8, 0xa, 0, 6), (0x1d, 0xa, 0, 6, 1, 0), 0xa,',
     'cqn_11 tabela da copa do imperador'),
    ('cqn11_copa_dos_eua_sorteio_uma_semana_depois',
     'cqn11r((0xa, 7, 0, 3), (0x17, 7, 0, 2, 2, 0), 0x82,',
     'cqn11r((0x11, 7, 0, 3), (0x17, 7, 0, 2, 2, 0), 0x82,',
     'cqn_11 tabela da copa dos eua'),
]
