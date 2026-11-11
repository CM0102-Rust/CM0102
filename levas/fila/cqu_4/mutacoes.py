# Mutacoes (cqu_4). A fase de pontos corridos de cada calendario roda na criacao da carreira (janeiro de 2001) e na virada
# (contadores "cqu_4 <liga> liga"); as datas sao levadas ao dia da semana da rodada, entao mudar o dia da semana muda a data:
# - j2: a 2a rodada da fase de pontos corridos com o dia da semana 5 no lugar de 6 (os jogos dela um dia antes);
MUTACOES = [
    ('cqu4_j2_segunda_rodada_um_dia_antes',
     'pub(crate) const J2_LEAGUE_ROUNDS: [AsiaClubChampScheduleOp; 88] = [\n    Cqp11D(0, 0xc, 2, 0, 6, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x13, 2, 0, 6, 1, 0)',
     'pub(crate) const J2_LEAGUE_ROUNDS: [AsiaClubChampScheduleOp; 88] = [\n    Cqp11D(0, 0xc, 2, 0, 6, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x13, 2, 0, 5, 1, 0) /* mutation: round 2 one weekday earlier */',
     'cqu_4 j2 league'),
]
# Prova do agente (07/11, carreira Rust de 8 ligas ate 2002-06-01 com motor de lances; base com os 10 itens finais): ACUSADA (2001-074).
