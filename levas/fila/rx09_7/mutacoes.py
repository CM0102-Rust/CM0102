# Mutacoes (rx09_7), com a Espanha e a Suecia ate 2002-09-01 e a base NORMAL:
# - a Segunda sueca rebaixa 1 em vez de 2 (o fim das secoes em novembro de 2001 e a virada mudam; o ensaio das regras
#   acusa);
# - a regra de disciplina 0x1a (a espanhola) em vez de 0x1c: as suspensoes mudam (o comparador do construtor acusa na
#   criacao do jogo);
# - a rodada de 19 de julho uma semana depois: a tabela lado a lado acusa na criacao e os jogos de julho de 2001 mudam;
# - o acesso uma semana depois (22 de novembro): a tabela lado a lado acusa em novembro de 2001.
MUTACOES = [
    ('rx09_7_rebaixa_um',
     '(league::RELEGATED_COUNT, 1, 2), // rx09_7: two go down',
     '(league::RELEGATED_COUNT, 1, 1), // mutation: one goes down',
     'rx09_7 swedish second division rules'),
    ('rx09_7_disciplina_espanhola',
     'division: LeagueDivision { vtabela: 0x9702a0, discipline: 0x1c,',
     'division: LeagueDivision { vtabela: 0x9702a0, discipline: 0x1a,',
     'rx09_7 new swedish second division'),
    ('rx09_7_julho_uma_semana_depois',
     '(19, 6, 0, 6, 1, &[(-1, -1, -1)]), // 8: 19/jul',
     '(26, 6, 0, 6, 1, &[(-1, -1, -1)]), // mutation: 26/jul',
     'rx09_7 swedish second section rounds'),
    ('rx09_7_acesso_uma_semana_depois',
     'cql10r((9, 10, 0, 0), (0xf, 10, 0, 6, 1, 0), 0xa0,',
     'cql10r((9, 10, 0, 0), (0x16, 10, 0, 6, 1, 0), 0xa0,',
     'rx09_7 swedish second promotion play-off round'),
]
