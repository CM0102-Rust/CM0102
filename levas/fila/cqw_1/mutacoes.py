# Mutacoes (cqw_1). A fase de pontos corridos de cada calendario roda na criacao da carreira (janeiro de 2001) e na virada
# (contadores "cqw_1 <liga> league"); as datas sao levadas ao dia da semana da rodada, entao mudar o dia da semana muda a data:
# - baiano: a 2a rodada da fase de pontos corridos com o dia da semana 5 no lugar de 6 (os jogos dela um dia antes);
# - centro_oeste: a 2a rodada da fase de pontos corridos com o dia da semana 5 no lugar de 6 (os jogos dela um dia antes);
# - gaucho: a 2a rodada da fase de pontos corridos com o dia da semana 5 no lugar de 6 (os jogos dela um dia antes);
# - goiano: a 2a rodada da fase de pontos corridos com o dia da semana 5 no lugar de 6 (os jogos dela um dia antes);
MUTACOES = [
    ('cqw1_baiano_segunda_rodada_um_dia_antes',
     'const BAHIA_LEAGUE_STAGE_OPS: [AsiaClubChampScheduleOp; 64] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 6, 2, 0)',
     'const BAHIA_LEAGUE_STAGE_OPS: [AsiaClubChampScheduleOp; 64] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 5, 2, 0) /* mutacao: a 2a rodada num dia da semana antes */',
     'cqw_1 bahia league'),
    ('cqw1_centro_oeste_segunda_rodada_um_dia_antes',
     'const CENTRAL_LEAGUE_STAGE_OPS: [AsiaClubChampScheduleOp; 56] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 6, 2, 0)',
     'const CENTRAL_LEAGUE_STAGE_OPS: [AsiaClubChampScheduleOp; 56] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 5, 2, 0) /* mutacao: a 2a rodada num dia da semana antes */',
     'cqw_1 centre-west league'),
    ('cqw1_gaucho_segunda_rodada_um_dia_antes',
     'const GAUCHO_LEAGUE_STAGE_OPS: [AsiaClubChampScheduleOp; 62] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 6, 2, 0)',
     'const GAUCHO_LEAGUE_STAGE_OPS: [AsiaClubChampScheduleOp; 62] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 5, 2, 0) /* mutacao: a 2a rodada num dia da semana antes */',
     'cqw_1 gaucho league'),
    ('cqw1_goiano_segunda_rodada_um_dia_antes',
     'const GOIAS_LEAGUE_STAGE_OPS: [AsiaClubChampScheduleOp; 52] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 6, 2, 0)',
     'const GOIAS_LEAGUE_STAGE_OPS: [AsiaClubChampScheduleOp; 52] = [\n    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),\n    Cqp11D(1, 0x12, 1, 0, 5, 2, 0) /* mutacao: a 2a rodada num dia da semana antes */',
     'cqw_1 goias league'),
]
# Prova do agente (07/11, carreira Rust de 8 ligas ate 2002-06-01 com motor de lances; base com os 10 itens finais): as quatro ACUSADAS (2001-020 nas quatro).
