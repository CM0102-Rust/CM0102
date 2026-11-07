# Mutacoes (cqu_4). A fase de pontos corridos de cada calendario roda na criacao da carreira (janeiro de 2001) e na virada
# (contadores "cqu_4 <liga> liga"); as datas sao levadas ao dia da semana da rodada, entao mudar o dia da semana muda a data:
# - mls: o primeiro jogo do play-off em 9/8 no lugar de 16/8 (uma semana antes; a troca do dia da semana da 2a rodada da
#   fase de pontos corridos escapou: os jogos da MLS saem da 0x9086e0 e das conferencias, nao dessa tabela);
# - j2: a 2a rodada da fase de pontos corridos com o dia da semana 5 no lugar de 6 (os jogos dela um dia antes);
MUTACOES = [
    ('cqu4_mls_playoff_uma_semana_antes',
     'const CQU4_MLS_3: [Cqp11Op; 38] = [\n    Cqp11S(0, 7, 8, 0, 6), Cqp11J(0, 0x10, 8, 0, 5, 1, 0)',
     'const CQU4_MLS_3: [Cqp11Op; 38] = [\n    Cqp11S(0, 7, 8, 0, 6), Cqp11J(0, 9, 8, 0, 5, 1, 0) /* mutacao: o play-off uma semana antes */',
     'cqu_4 mls fase 2'),
    ('cqu4_j2_segunda_rodada_um_dia_antes',
     'const CQU4_J2_0: [Cqp11Op; 88] = [\n    Cqp11D(0, 0xc, 2, 0, 6, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x13, 2, 0, 6, 1, 0)',
     'const CQU4_J2_0: [Cqp11Op; 88] = [\n    Cqp11D(0, 0xc, 2, 0, 6, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x13, 2, 0, 5, 1, 0) /* mutacao: a 2a rodada num dia da semana antes */',
     'cqu_4 j2 liga'),
]
