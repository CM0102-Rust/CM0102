# As tabelas dos periodos da Terceira Divisao belga: um resultado, a fase do periodo e a ordem das fases (bel_third.cpp;
# leitor cx09, rodada 5, 24/11/2026). Simulacao com a Belgica (as ligas do cx02_21: 'ENG BEL HOL GER DEN', ate
# 2002-08-01); na cobertura do mundo a 0x422c50 roda 505 vezes e a 0x422cf0 662 ate 2002-08-01.
# - 0x422c50 BelThird::add_result thiscall(jogo, x) ret 8 -> 1: League::add_result 0x685d30 (chamada direta, thiscall
#   ret 8); com o +0x42 do jogo 0xff ou 0 (grupo A ou B): periodo pela rodada (+0x34 < 10: 1, < 20: 2, senao 3; o
#   original empilha o dword do 1o argumento com o byte trocado: so o byte e lido), a fase por 0x422cf0 (periodo, +0x32
#   do jogo, 1), +0x42 = a fase, +8 da liga zerado durante a virtual +4 da fase (jogo, x), e de volta. Prologo mov eax,
#   [esp+8]; push ebx; push esi (6 bytes).
# - 0x422cf0 BelThird::find_or_create_period_stage thiscall(periodo, prefixo, criar) ret 0xc -> al: rotulo 0x45d..0x45f
#   (prefixo 0x41f) ou 0x460..0x462; procura nas fases 1..+0x30 pelo +0x3a; sem ela e sem criar: 0xfe; malloc(+0x3e x 4)
#   (caixa 0x378 e 0xfe), os clubes da tabela do grupo (A: a liga; B: a fase 0; contagem da fonte), +0x30 + 1,
#   operator new 0xee e League_Stage 0x68b100 (22 argumentos conferidos, lugares 0,1,0,0 do local), free, qsort
#   0x945333 (fases+4, +0x30, 4, 0x424380), +0x44 de cada fase = o indice, e a busca de novo sem criar. Prologo push -1;
#   push imm32 (7 bytes).
# - 0x424380 bel_third_compare_period_stages cdecl(a, b) -> int: grupo (0x41f/0x420) e periodo pelo rotulo; rotulo de
#   fora: o dword do 2o argumento (o ponteiro b) como grupo e o byte dele como periodo (defeito, inofensivo). Diferencas
#   com sinal (palavra, depois byte). Prologo mov eax, [esp+8]; mov edx, [esp+4] (8 bytes).
# Comparadores: add_result e a fase so a original (somam resultados e criam fases), conferidas depois (o retorno, o
# +0x42 e o +8 de volta; a fase achada pela busca da copia); a ordem lado a lado (compare_order!).
# Iguais no cm0102-1080.exe. Sem dependencias (LeagueStageCtorFn publicado no bra_nat_third.rs).
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('bel_third_add_result', 0x422c50, [0x8b, 0x44, 0x24, 0x08, 0x53, 0x56]),
           ('bel_third_find_or_create_period_stage', 0x422cf0, [0x6a, 0xff, 0x68, 0x5e, 0x41, 0x95, 0x00]),
           ('bel_third_compare_period_stages', 0x424380, [0x8b, 0x44, 0x24, 0x08, 0x8b, 0x54, 0x24, 0x04])]
DOCS = ('as tabelas dos períodos da Terceira Divisão belga', 'the Belgian Third Division period tables',
        [('bel_third_add_result', 'BelThird::add_result', '0x422c50', 'bel_third.cpp', 'thiscall, ret 8',
          'cada jogo de grupo da Terceira Divisão belga conta também na tabela do período do seu grupo: rodadas 1 a 9 o primeiro período, 10 a 19 o segundo, dali em diante o terceiro; a tabela do período não grava estatísticas',
          'each Belgian Third Division group game also counts in the table of its group period: rounds 1 to 9 the first period, 10 to 19 the second, from then on the third; the period table writes no statistics'),
         ('bel_third_find_or_create_period_stage', 'BelThird::find_or_create_period_stage', '0x422cf0', 'bel_third.cpp', 'thiscall, ret 0xc',
          'a tabela de um período de um grupo nasce no primeiro jogo dele, com os clubes do grupo, um turno, sem estatísticas; as fases dos períodos ficam em ordem de grupo e de período',
          'a group period table is created with its first game, with the group clubs, one leg, no statistics; the period stages are kept in group and period order'),
         ('bel_third_compare_period_stages', 'bel_third_compare_period_stages', '0x424380', 'bel_third.cpp', 'cdecl',
          'a ordem das fases dos períodos: grupo A antes do B, e dentro do grupo pelo período',
          'the order of the period stages: group A before group B, and within a group by period')])
CAMPOS = []
INTERESSA = []
