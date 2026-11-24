# Mutacoes (cx12_13), com a Noruega ate 2002-06-01 (a temporada norueguesa de 2002 comeca em abril):
# - a escolha do fim de abril invertida (as duas ligas): as rodadas 26/4, 30/4 e 3/5 de 2002 em outras datas, e com
#   elas outros jogos e resultados ate junho; acusada tambem pelo comparador (lado a lado na tabela);
# - a Premier de 2002 (ano par) com as datas dos anos impares: outras rodadas em abril e maio de 2002;
# - a repescagem de 2001 da Primeira uma semana depois (1/11 no lugar de 25/10): os jogos de outubro de 2001 mudam de
#   dia, e com eles o calendario dos dois clubes.
MUTACOES = [
    ('cx12_13_fim_de_abril_invertido',
     '&NORWAY_LATE_APRIL_NEAR, &NORWAY_LATE_APRIL_FAR); // cx12_13: 30 April or 3 May',
     '&NORWAY_LATE_APRIL_FAR, &NORWAY_LATE_APRIL_NEAR); // mutation: the choice swapped',
     'cx12_13 norwegian premier schedule'),
    ('cx12_13_premier_par_com_datas_impares',
     '(&NORWEGIAN_PREMIER_EVEN_START, &NORWEGIAN_PREMIER_EVEN_END) // cx12_13: the even years',
     '(&NORWEGIAN_PREMIER_ODD_START, &NORWEGIAN_PREMIER_ODD_END) // mutation: the odd years dates',
     'cx12_13 norwegian premier schedule'),
    ('cx12_13_repescagem_uma_semana_depois',
     'cql10r((0x13, 9, 0, 0), (0x19, 9, 0, 6, 1, 0), 0xa0,',
     'cql10r((0x13, 9, 0, 0), (1, 10, 0, 6, 1, 0), 0xa0,',
     'cx12_13 norwegian play-off round'),
]
