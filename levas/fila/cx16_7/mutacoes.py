# Mutacoes (cx16_7), ENG ate 2005-12-31 (os jogos das eliminatorias da campanha de 2004); acusadas pelos comparadores
# (lado a lado nos tres codigos da entrada):
# - as faixas dos grupos da CONCACAF com o terceiro 10 (quem fica em que faixa muda);
# - o segundo de um grupo europeu 19 no atual e no pior;
# - o segundo de um grupo da Oceania 2 no atual.
MUTACOES = [
    ('cx16_7_concacaf_terceiro_10',
     'return (band(current, 6), band(best, 1), band(worst, 6)); // cx16_7: the group bands',
     'return (band(current, 6).max(0xa), band(best, 1), band(worst, 6)); // mutation: the bands pushed down',
     'cx16_7 concacaf progress'),
    ('cx16_7_europa_segundo_19',
     'return (band(current, 0x10), band(best, 0xa), band(worst, 0x10)); // cx16_7: the European group bands',
     'return (band(current, 0x13), band(best, 0xa), band(worst, 0x13)); // mutation: runner-up 19',
     'cx16_7 european progress'),
    ('cx16_7_oceania_segundo_2',
     '2 => c = 3, // cx16_7: the Oceania group runner-up',
     '2 => c = 2, // mutation: runner-up kept',
     'cx16_7 oceania progress'),
]
