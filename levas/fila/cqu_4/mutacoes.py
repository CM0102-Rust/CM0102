# Mutacoes (cqu_4). A fase de pontos corridos de cada calendario roda na criacao da carreira (janeiro de 2001) e na virada
# (contadores "cqu_4 <liga> liga"); as datas sao levadas ao dia da semana da rodada, entao mudar o dia da semana muda a data:
# - mls: a 2a rodada da fase de pontos corridos com o dia da semana 4 no lugar de 5 (os jogos dela um dia antes);
# - j2: a 2a rodada da fase de pontos corridos com o dia da semana 5 no lugar de 6 (os jogos dela um dia antes);
MUTACOES = [
    ('cqu4_mls_segunda_rodada_um_dia_antes',
     'const CQU4_MLS_2: [Cqp11Op; 64] = [\n    Cqp11D(0, 0x16, 2, 0, 5, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x1d, 2, 0, 5, 1, 0)',
     'const CQU4_MLS_2: [Cqp11Op; 64] = [\n    Cqp11D(0, 0x16, 2, 0, 5, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x1d, 2, 0, 4, 1, 0) /* mutacao: a 2a rodada num dia da semana antes */',
     'cqu_4 mls liga'),
    ('cqu4_j2_segunda_rodada_um_dia_antes',
     'const CQU4_J2_0: [Cqp11Op; 88] = [\n    Cqp11D(0, 0xc, 2, 0, 6, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x13, 2, 0, 6, 1, 0)',
     'const CQU4_J2_0: [Cqp11Op; 88] = [\n    Cqp11D(0, 0xc, 2, 0, 6, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x13, 2, 0, 5, 1, 0) /* mutacao: a 2a rodada num dia da semana antes */',
     'cqu_4 j2 liga'),
]
