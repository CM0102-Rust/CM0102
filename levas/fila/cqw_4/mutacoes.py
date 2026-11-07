# Mutacoes (cqw_4). A fase de pontos corridos de cada calendario roda na criacao da carreira (janeiro de 2001) e na virada
# (contadores "cqw_4 <liga> liga"); as datas sao levadas ao dia da semana da rodada, entao mudar o dia da semana muda a data:
# - serie_a: a 2a rodada da fase de pontos corridos com o dia da semana 1 no lugar de 2 (os jogos dela um dia antes);
# - serie_b: a 2a rodada da fase de pontos corridos com o dia da semana 1 no lugar de 2 (os jogos dela um dia antes);
# - j1: a 2a rodada da fase de pontos corridos com o dia da semana 4 no lugar de 5 (os jogos dela um dia antes);
# - nsl: a 2a rodada da fase de pontos corridos com o dia da semana 4 no lugar de 5 (os jogos dela um dia antes);
MUTACOES = [
    ('cqw4_serie_a_segunda_rodada_um_dia_antes',
     'const CQW4_SERIE_A_0: [Cqp11Op; 88] = [\n    Cqp11D(0, 5, 7, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 8, 7, 0, 2, 2, 0)',
     'const CQW4_SERIE_A_0: [Cqp11Op; 88] = [\n    Cqp11D(0, 5, 7, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 8, 7, 0, 1, 2, 0) /* mutacao: a 2a rodada num dia da semana antes */',
     'cqw_4 serie_a liga'),
    ('cqw4_serie_b_segunda_rodada_um_dia_antes',
     'const CQW4_SERIE_B_0: [Cqp11Op; 88] = [\n    Cqp11D(0, 5, 7, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 8, 7, 0, 2, 2, 0)',
     'const CQW4_SERIE_B_0: [Cqp11Op; 88] = [\n    Cqp11D(0, 5, 7, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 8, 7, 0, 1, 2, 0) /* mutacao: a 2a rodada num dia da semana antes */',
     'cqw_4 serie_b liga'),
    ('cqw4_j1_segunda_rodada_um_dia_antes',
     'const CQW4_J1_0: [Cqp11Op; 60] = [\n    Cqp11D(0, 0xb, 2, 0, 5, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x12, 2, 0, 5, 1, 0)',
     'const CQW4_J1_0: [Cqp11Op; 60] = [\n    Cqp11D(0, 0xb, 2, 0, 5, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x12, 2, 0, 4, 1, 0) /* mutacao: a 2a rodada num dia da semana antes */',
     'cqw_4 j1 liga'),
    ('cqw4_nsl_segunda_rodada_um_dia_antes',
     'const CQW4_NSL_1: [Cqp11Op; 48] = [\n    Cqp11D(0, 5, 9, 0, 5, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0xc, 9, 0, 5, 1, 0)',
     'const CQW4_NSL_1: [Cqp11Op; 48] = [\n    Cqp11D(0, 5, 9, 0, 5, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0xc, 9, 0, 4, 1, 0) /* mutacao: a 2a rodada num dia da semana antes */',
     'cqw_4 nsl liga'),
]
