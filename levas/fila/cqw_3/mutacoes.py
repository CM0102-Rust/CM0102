# Mutacoes (cqw_3). A fase de pontos corridos de cada calendario roda na criacao da carreira (janeiro de 2001) e na virada
# (contadores "cqw_3 <liga> liga"); as datas sao levadas ao dia da semana da rodada, entao mudar o dia da semana muda a data:
# - pernambucano: a 2a rodada da fase de pontos corridos com o dia da semana 5 no lugar de 6 (os jogos dela um dia antes);
# - carioca: a 2a rodada da fase de pontos corridos com o dia da semana 5 no lugar de 6 (os jogos dela um dia antes);
# - catarinense: a 2a rodada da fase de pontos corridos com o dia da semana 5 no lugar de 6 (os jogos dela um dia antes);
# - paulista: a 2a rodada da fase de pontos corridos com o dia da semana 5 no lugar de 6 (os jogos dela um dia antes);
MUTACOES = [
    ('cqw3_pernambucano_segunda_rodada_um_dia_antes',
     'const CQW3_PERNAMBUCANO_0: [Cqp11Op; 64] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 6, 2, 0)',
     'const CQW3_PERNAMBUCANO_0: [Cqp11Op; 64] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 5, 2, 0) /* mutacao: a 2a rodada num dia da semana antes */',
     'cqw_3 pernambucano liga'),
    ('cqw3_carioca_segunda_rodada_um_dia_antes',
     'const CQW3_CARIOCA_0: [Cqp11Op; 78] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 6, 2, 0)',
     'const CQW3_CARIOCA_0: [Cqp11Op; 78] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 5, 2, 0) /* mutacao: a 2a rodada num dia da semana antes */',
     'cqw_3 carioca liga'),
    ('cqw3_catarinense_segunda_rodada_um_dia_antes',
     'const CQW3_CATARINENSE_0: [Cqp11Op; 64] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 6, 2, 0)',
     'const CQW3_CATARINENSE_0: [Cqp11Op; 64] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 5, 2, 0) /* mutacao: a 2a rodada num dia da semana antes */',
     'cqw_3 catarinense liga'),
    ('cqw3_paulista_segunda_rodada_um_dia_antes',
     'const CQW3_PAULISTA_0: [Cqp11Op; 56] = [\n    Cqp11D(0, 0x12, 1, 0, 6, 2, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x19, 1, 0, 6, 1, 0)',
     'const CQW3_PAULISTA_0: [Cqp11Op; 56] = [\n    Cqp11D(0, 0x12, 1, 0, 6, 2, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x19, 1, 0, 5, 1, 0) /* mutacao: a 2a rodada num dia da semana antes */',
     'cqw_3 paulista liga'),
]
