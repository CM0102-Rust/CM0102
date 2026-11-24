# As repescagens de acesso e de rebaixamento da Terceira Divisao belga (bel_third.cpp; leitor cx09, rodada 5,
# 24/11/2026). Simulacao com a Belgica ('ENG BEL HOL GER DEN', ate 2002-08-01): as duas rodam uma vez, quando a
# Segunda termina (advance_stage na fase 7, primavera de 2002).
# - 0x423880 BelThird::create_promotion_playoff_stage thiscall() ret 0: em cada grupo (A: a liga; B: a fase 0) conta os
#   status 1 e, enquanto < 3, cada linha 0xff vira 1 com a virtual 0x40 da liga (clube, 1, +0x44, 0, 0) e a 0x68
#   (clube, +0x44, 2, 2, 7); a lista de 7: os tres primeiros status 1 do A nas vagas 0, 2, 4, os do B nas 5, 3, 1, e o
#   16o da divisao de cima ([[0xadadfc] + 4 x +0x1c], linha +0x375) na 6; virtual 0x3c (8, &rodadas, &extra, 0);
#   operator new 0xb2 e Cup_Stage 0x522e00 (this, 7, lista, rodadas, competicao, tabela, ano, 8, 1, extra, 0x14, 1, 0,
#   0, 0) na vaga 8; sem fase a caixa (linha 0x611, sem liberar a tabela); senao Cup::set_up_round 0x51c800 (0), free,
#   +0x30 = 8. Vagas da lista sem clube ficam com o lixo da pilha no original (aqui 0). Prologo mov eax, fs:[0] (6
#   bytes, como a 0x62be00).
# - 0x423ba0 BelThird::create_relegation_playoff_stage thiscall() ret 0: os clubes ([0xae23b4], 0x245 bytes, indice
#   palavra ate [0xae235c] - 2 x [0xae2350]) com nacao e divisao de id [0x9cf7d4]/[0x9cf7d8]/[0x9cf7dc]/[0x9cf7e0],
#   ate 100; menos de 16: a caixa (0x64a). comp_util_sort_and_shuffle_top_clubs 0x4afbd0 (lista, n, 16) cdecl; os ids
#   dos 4 primeiros em +0xee..+0xfa; os 12 seguintes e o ultimo status 2 do grupo A e do B (as duas tabelas no mesmo
#   laco, pela contagem da liga); sem um deles a caixa (0x66a). Virtual 0x3c (9, ...), Cup_Stage (this, 14, lista,
#   rodadas, competicao, tabela, ano, 9, 1, extra, 0x14, 0, 0, 0, sementes {2 x 12, 1, 1}) na vaga 9; sem fase a caixa
#   (0x678); senao set_up_round (0), free, +0x30 = 9. Prologo push -1; push imm32.
# Comparadores: so a original (constroem fases, a segunda sorteia), conferidas depois (a vaga e a fase atual).
# Iguais no cm0102-1080.exe. Depende do cx09_8 (BEL_THIRD_SOURCE).
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('bel_third_create_promotion_playoff_stage', 0x423880, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('bel_third_create_relegation_playoff_stage', 0x423ba0, [0x6a, 0xff, 0x68, 0xbe, 0x41, 0x95, 0x00])]
DOCS = ('as repescagens de acesso e de rebaixamento da Terceira Divisão belga', 'the Belgian Third Division promotion and relegation play-offs',
        [('bel_third_create_promotion_playoff_stage', 'BelThird::create_promotion_playoff_stage', '0x423880', 'bel_third.cpp', 'thiscall, ret 0',
          'a repescagem de acesso: cada grupo completa até três classificados (os campeões dos períodos e, faltando, os melhores ainda indefinidos), e os seis jogam com o 16º da Segunda Divisão (fase 8)',
          "the promotion play-off: each group is topped up to three qualified clubs (the period winners and, if short, the best still undecided), and the six play with the Second Division's 16th club (stage 8)"),
         ('bel_third_create_relegation_playoff_stage', 'BelThird::create_relegation_playoff_stage', '0x423ba0', 'bel_third.cpp', 'thiscall, ret 0',
          'a repescagem de rebaixamento: os clubes das quatro Quartas Divisões são ordenados e sorteados; os quatro primeiros sobem direto e os doze seguintes jogam com o penúltimo de cada grupo da Terceira (14 clubes, fase 9)',
          'the relegation play-off: the clubs of the four Fourth Divisions are ranked and shuffled; the first four go up directly and the next twelve play with each Third Division group\'s relegation play-off club (14 clubs, stage 9)')])
CAMPOS = [('terceira divisão belga +0xee..+0xfd: os ids dos quatro clubes da Quarta Divisão que sobem direto', 'Belgian Third Division +0xee..+0xfd: the ids of the four Fourth Division clubs promoted directly')]
INTERESSA = []
