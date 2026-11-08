# Mutacoes (cqw_2). A fase de pontos corridos de cada calendario roda na criacao da carreira (janeiro de 2001) e na virada
# (contadores "cqw_2 <liga> liga"); as datas sao levadas ao dia da semana da rodada, entao mudar o dia da semana muda a data:
# - mineiro: a 2a rodada da fase de pontos corridos com o dia da semana 5 no lugar de 6 (os jogos dela um dia antes);
# - norte: a 2a rodada da fase de pontos corridos com o dia da semana 5 no lugar de 6 (os jogos dela um dia antes);
# - nordeste: a 2a rodada da fase de pontos corridos com o dia da semana 5 no lugar de 6 (os jogos dela um dia antes);
# - paranaense: a 2a rodada da fase de pontos corridos com o dia da semana 5 no lugar de 6 (os jogos dela um dia antes);
MUTACOES = [
    ('cqw2_mineiro_segunda_rodada_um_dia_antes',
     'const CQW2_MINEIRO_0: [AsiaClubChampScheduleOp; 78] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 6, 2, 0)',
     'const CQW2_MINEIRO_0: [AsiaClubChampScheduleOp; 78] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 5, 2, 0) /* mutacao: a 2a rodada num dia da semana antes */',
     'cqw_2 mineiro liga'),
    ('cqw2_norte_segunda_rodada_um_dia_antes',
     'const CQW2_NORTE_0: [AsiaClubChampScheduleOp; 64] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 6, 2, 0)',
     'const CQW2_NORTE_0: [AsiaClubChampScheduleOp; 64] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 5, 2, 0) /* mutacao: a 2a rodada num dia da semana antes */',
     'cqw_2 norte liga'),
    ('cqw2_nordeste_segunda_rodada_um_dia_antes',
     'const CQW2_NORDESTE_0: [AsiaClubChampScheduleOp; 56] = [\n    Cqp11D(0, 0x12, 1, 0, 6, 2, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x19, 1, 0, 6, 1, 0)',
     'const CQW2_NORDESTE_0: [AsiaClubChampScheduleOp; 56] = [\n    Cqp11D(0, 0x12, 1, 0, 6, 2, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x19, 1, 0, 5, 1, 0) /* mutacao: a 2a rodada num dia da semana antes */',
     'cqw_2 nordeste liga'),
    ('cqw2_paranaense_segunda_rodada_um_dia_antes',
     'const CQW2_PARANAENSE_0: [AsiaClubChampScheduleOp; 78] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 6, 2, 0)',
     'const CQW2_PARANAENSE_0: [AsiaClubChampScheduleOp; 78] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 5, 2, 0) /* mutacao: a 2a rodada num dia da semana antes */',
     'cqw_2 paranaense liga'),
]
# Prova do agente (07/11, carreira Rust de 8 ligas ate 2002-06-01 com motor de lances; base com os 10 itens finais): as quatro ACUSADAS (2001-020, 2001-020, 2001-027, 2001-020).
