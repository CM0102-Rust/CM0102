# Mutacoes (cx15_9), carreira das 8 ligas ate 2002-06-01. Nenhuma abre caixa. Os jogos da MLS sao montados a mao
# (usa_mls_create_season_fixtures) e a tabela de datas desta rotina pode nao mudar a carreira (a tentativa da cqu_4, em
# 06/11/2026, viu duas mutacoes de data escaparem da carreira); o comparador lado a lado da rotina acusa as duas na
# comparacao de 8 ligas (os bytes gravados da tabela diferem).
# - a 2a rodada da temporada regular (32 rodadas) uma semana depois (29/2 -> 7/3);
# - os play-offs sorteados uma semana depois (7/8 -> 14/8).
MUTACOES = [
    ('cx15_9_segunda_rodada_semana_depois',
     'Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x1d, 2, 0, 5, 1, 0),\n    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 5, 3, 0, 5, 1, 0),',
     'Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 7, 3, 0, 5, 1, 0), // mutation\n    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 5, 3, 0, 5, 1, 0),',
     'cx15_9 mls 32 rounds'),
    ('cx15_9_playoffs_semana_depois',
     '    Cqp11S(0, 7, 8, 0, 6), Cqp11J(0, 0x10, 8, 0, 5, 1, 0),',
     '    Cqp11S(0, 0xe, 8, 0, 6) /* mutation */, Cqp11J(0, 0x10, 8, 0, 5, 1, 0),',
     'cx15_9 mls play-offs'),
]
