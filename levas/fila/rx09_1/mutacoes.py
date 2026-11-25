# Mutacoes (rx09_1), com a Espanha e a Suecia ate 2002-09-01 e a base NORMAL:
# - a Segunda B rebaixa 3 em vez de 4 (o fim dos grupos em maio de 2002 e a virada mudam; o ensaio das regras acusa);
# - a primeira rodada dos grupos uma semana antes (15 de agosto): a temporada 2002/03 comeca antes; a tabela lado a lado
#   acusa na virada de 2002 e na criacao do jogo;
# - a final da repescagem do rebaixamento uma semana depois (11 de junho): a tabela lado a lado acusa em maio de 2002.
MUTACOES = [
    ('rx09_1_rebaixa_tres',
     '(league::RELEGATED_COUNT, 1, 4), // rx09_1: four go down',
     '(league::RELEGATED_COUNT, 1, 3), // mutation: three go down',
     'rx09_1 segunda b rules'),
    ('rx09_1_grupos_uma_semana_antes',
     '(22, 7, 0, 6, 1, &[(-1, -1, -1)]), // 0: 22/ago (rx09_1: the first group round)',
     '(15, 7, 0, 6, 1, &[(-1, -1, -1)]), // mutation: 15/ago',
     'rx09_1 segunda b group rounds'),
    ('rx09_1_final_uma_semana_depois',
     '(4, 5, 1, 6, 1, 0), 0, (0, 0, 3), 5, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)); // rx09_1: the final on 4 June',
     '(0xb, 5, 1, 6, 1, 0), 0, (0, 0, 3), 5, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)); // mutation: 11 June',
     'rx09_1 segunda b relegation final round'),
]
