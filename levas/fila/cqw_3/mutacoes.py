# Mutacoes (cqw_3). A fase de pontos corridos de cada calendario roda na criacao da carreira (janeiro de 2001) e na virada
# (contadores "cqw_3 <liga> liga"); as datas sao levadas ao dia da semana da rodada, entao mudar o dia da semana muda a data:
# - pernambucano: a 2a rodada da fase de pontos corridos com o dia da semana 5 no lugar de 6 (os jogos dela um dia antes);
# - carioca: a 2a rodada da fase de pontos corridos com o dia da semana 5 no lugar de 6 (os jogos dela um dia antes);
# - catarinense: a 2a rodada da fase de pontos corridos com o dia da semana 5 no lugar de 6 (os jogos dela um dia antes);
# - paulista: a 2a rodada da fase de pontos corridos com o dia da semana 5 no lugar de 6 (os jogos dela um dia antes);
MUTACOES = [
    ('cqw3_pernambucano_segunda_rodada_um_dia_antes',
     'const PERNAMBUCO_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 64] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 6, 2, 0)',
     'const PERNAMBUCO_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 64] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 5, 2, 0) /* mutation: the 2nd round one weekday earlier */',
     'cqw_3 pernambuco league'),
    ('cqw3_carioca_segunda_rodada_um_dia_antes',
     'const RIO_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 78] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 6, 2, 0)',
     'const RIO_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 78] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 5, 2, 0) /* mutation: the 2nd round one weekday earlier */',
     'cqw_3 rio league'),
    ('cqw3_catarinense_segunda_rodada_um_dia_antes',
     'const SANTA_CATARINA_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 64] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 6, 2, 0)',
     'const SANTA_CATARINA_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 64] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 5, 2, 0) /* mutation: the 2nd round one weekday earlier */',
     'cqw_3 santa catarina league'),
    ('cqw3_paulista_segunda_rodada_um_dia_antes',
     'const SAO_PAULO_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 56] = [\n    Cqp11D(0, 0x12, 1, 0, 6, 2, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x19, 1, 0, 6, 1, 0)',
     'const SAO_PAULO_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 56] = [\n    Cqp11D(0, 0x12, 1, 0, 6, 2, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x19, 1, 0, 5, 1, 0) /* mutation: the 2nd round one weekday earlier */',
     'cqw_3 sao paulo league'),
]
# Prova do agente (07/11, carreira Rust de 8 ligas ate 2002-06-01 com motor de lances; base com os 10 itens finais): as quatro ACUSADAS (2001-020, 2001-020, 2001-020, 2001-027).
