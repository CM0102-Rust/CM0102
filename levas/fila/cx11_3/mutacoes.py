# Mutacoes (cx11_3), com a Italia e a base NORMAL ate 2002-09-30:
# - a vitoria vale 2 pontos nas C2 (o auxiliar serve aos dois grupos): a tabela muda de ordem ao longo da temporada,
#   e com ela os da repescagem, do play-out e o campeao (acusada tambem pelo comparador das regras, lado a lado).
# A tabela fica sem mutacao propria: o molde (ita_ser_a_fill_league_table) ja e provado na C1 (cqj_4), a C2 nao tem
# fundo nem TV (as mutacoes da C1 eram nas cotas), e as linhas trocadas ou contadas de outro jeito derrubam o jogo (o
# confronto direto da 0x687430 le a linha seguinte ainda vazia; provado no cqj_4).
MUTACOES = [
    ('cx11_3_vitoria_vale_2',
     '(league::WIN_POINTS, 1, 3)]); // cx11_3: the C2 gives 3 points per win',
     '(league::WIN_POINTS, 1, 2)]); // mutation: 2 points',
     'cx11_3 serie c2 rules'),
]
