# Os grupos 2 a 4, os grupos do acesso e a repescagem do rebaixamento da Segunda Divisao B espanhola (spa_second_b.cpp;
# leitor rx09, rodada 6, 25/11/2026). So existe com a base NORMAL. Simulacao com a Espanha e a Suecia ate 2002-09-01:
# os grupos na criacao e na virada de 2002; o acesso e a repescagem no fim dos grupos (maio de 2002).
# - 0x857210 SpaSecondB::create_group_stage thiscall(grupo) ret 4: a virtual +0x3c(grupo, &rodadas, &extra, 0) com o
#   dword do argumento; a chave (0 -> B2 0x9cf630, 1 -> B3, outro -> B4); malloc 0x50 (caixa 0x2a1); os clubes como no
#   load_group_clubs; o 21o em diante: caixa 0x2b2 e a divisao zerada (a varredura segue); exatamente 19: n = 19 e os
#   lugares do local {0, 4, 1, 3}; senao n = 20 e os lugares da liga (+0xbe); menos: caixa 0x2c7. League_Stage 0x68b100
#   (os 22 argumentos conferidos: importancia +0x51, bits +0xd9); sem a fase, caixa 0x2d2; senao free da lista e da
#   tabela e +0x30 = grupo (com sinal).
# - 0x857570 SpaSecondB::create_promotion_playoff_groups thiscall() ret 0: a tabela local de 3 x 4 x 4 bytes conferida
#   byte a byte; utils_random(3) 0x90cfc0 (movsx); para g = 0..3 os quatro clubes (a linha k do grupo da tabela, 0 a
#   propria liga, 1..3 as fases 0..2), a fase g + 3 com 4 clubes, lugares {1, 0, 0, 0}, importancia 0x14 e bits 0x22;
#   sem a fase, caixa 0x315 e fim; senao free da tabela e +0x30 = g + 3.
# - 0x857860 SpaSecondB::create_relegation_playoff thiscall() ret 0: a linha 15 (+0x375) de cada um dos quatro grupos;
#   a virtual +0x3c(7, ...); Cup_Stage 0x522e00 (new 0xb2; this, 4, lista, rodadas, registro, tabela, ano, 7, 1,
#   extra, 0x14, 0, 0, 0, 0; ret 0x3c) em +0xc[7]; sem ela, caixa 0x33d; senao free da tabela, Cup::draw_round(0)
#   0x51c410 e +0x30 = 7.
# Prologos push -1; push imm32 (7 bytes); iguais no cm0102-1080.exe.
# Comparadores: so a original nas tres (alocam e sorteiam); o plano conferido depois (os clubes de cada fase como
# conjunto e a fase atual; o acesso contra as tres rotacoes).
# Depende do rx09_2 (segunda_b_group_clubs, SEGUNDA_B_SOURCE) e do cx14_6 (stage_round_table, category_b_league_stage,
# category_b_stage_clubs, compare_category_b_stage).
SAVE = False
LIGAS = 'ENG ESP SWE'
ATE = '2002-09-01'
BASE = 'NORMAL'
ROTINAS = [('spa_second_b_create_group_stage', 0x857210, [0x6a, 0xff, 0x68, 0xee, 0x27, 0x96, 0x00]),
           ('spa_second_b_create_promotion_playoff_groups', 0x857570, [0x6a, 0xff, 0x68, 0x0e, 0x28, 0x96, 0x00]),
           ('spa_second_b_create_relegation_playoff', 0x857860, [0x6a, 0xff, 0x68, 0x2e, 0x28, 0x96, 0x00])]
DOCS = ('os grupos 2 a 4, os grupos do acesso e a repescagem do rebaixamento da Segunda Divisão B espanhola',
        'the groups 2 to 4, the promotion play-off groups and the relegation play-off of the Spanish Second Division B',
        [('spa_second_b_create_group_stage', 'SpaSecondB::create_group_stage', '0x857210', 'spa_second_b.cpp', 'thiscall, ret 4',
          'regra: cada um dos grupos 2 a 4 tem 20 clubes, os da Segunda B daquele grupo na ordem do banco; um grupo de exatamente 19 rebaixa 3 em vez de 4; do 21º em diante os clubes perdem a divisão (defeito do original)',
          'rule: each of groups 2 to 4 has 20 clubs, the Second Division B ones of that group in database order; a group of exactly 19 relegates 3 instead of 4; from the 21st on the clubs lose their division (defect of the original)'),
         ('spa_second_b_create_promotion_playoff_groups', 'SpaSecondB::create_promotion_playoff_groups', '0x857570', 'spa_second_b.cpp', 'thiscall, ret 0',
          'regra: os quatro primeiros de cada grupo vão a quatro grupos do acesso de quatro clubes, um de cada grupo e de cada posição, pela rotação sorteada entre três; o primeiro de cada grupo do acesso sobe',
          'rule: the top four of each group go into four promotion play-off groups of four clubs, one from each group and each place, by a rotation drawn among three; the winner of each play-off group goes up'),
         ('spa_second_b_create_relegation_playoff', 'SpaSecondB::create_relegation_playoff', '0x857860', 'spa_second_b.cpp', 'thiscall, ret 0',
          'regra: os quatro 16º colocados jogam a repescagem do rebaixamento em mata-mata de ida e volta, sorteado',
          'rule: the four sixteenth-placed clubs play the relegation play-off as drawn two-leg ties')])
CAMPOS = []
INTERESSA = []
