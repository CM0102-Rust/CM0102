# Os grupos da primeira e da segunda fase e a fase seguinte das eliminatorias asiaticas da Copa de 2006
# (wc_asia_league.cpp; leitor rx09, rodada 6, 25/11/2026). Simulacao com a Inglaterra ate 2005-12-31: os grupos 1 a 9 no
# dia do sorteio (marco de 2005, pelo update_for_date); a segunda fase no fim da primeira (agosto de 2005); a fase
# seguinte no fim de cada fase.
# - 0x913540 WcAsiaLeague::create_first_round_group thiscall(k) ret 4 -> 0/1: n = get_first_round_group_size(k) (palavra;
#   o dword dela empurrado como n); malloc 4n (caixa 0x49a e 0); as vagas do grupo k + 1 do sorteio (+0xf7 + 16(k + 1));
#   a virtual +0x3c(k, ...); League_Stage 0x68b100 com os lugares da liga (+0xbe), importancia +0x51, o 17o argumento 4 e
#   os bits +0xd9; +0xc[k] (o dword do argumento); free da lista e da tabela antes de conferir; sem a fase, caixa 0x4b0 e
#   0; senao +0x30 = k, League::send_group_draw_news 0x684230 na fase e 1.
# - 0x913490 WcAsiaLeague::advance_stage thiscall() ret 0: na ultima fase a caixa 0x471; senao +0x30 + 1; em 9 a segunda
#   fase (0x913740), em 11 a terceira (0x913b60), em 12 a repescagem (0x913d50).
# - 0x913740 WcAsiaLeague::create_second_round_groups thiscall() ret 0 -> 0/1: os lugares num local {1, 1, 0, 0} ({2, ...}
#   sem isentos); 5 vencedores sorteados (utils_random(10), sem repetir; League::get_table_row 0x6826d0(grupo, 0)) e os
#   outros 5 na ordem; as fases 9 e 10 com 5 selecoes, importancia 0xf, bits 4 (caixas 0x4d9, 0x502, 0x510, 0x534).
# Prologos push -1; push imm32 (7 bytes) e mov edx, [ecx+0x2c]; mov eax, [ecx+0x30] (6 bytes). Iguais no
# cm0102-1080.exe. Comparadores: os grupos so a original e o plano (as selecoes como conjunto e a fase atual); a fase
# seguinte lado a lado quando nao constroi.
# Depende do rx09_16 (WC_ASIA_DRAW, wc_asia_group_size) e do cx14_6 (stage_round_table, category_b_stage_clubs,
# compare_category_b_stage).
SAVE = False
LIGAS = 'ENG'
ATE = '2005-12-31'
ROTINAS = [('wc_asia_league_create_first_round_group', 0x913540, [0x6a, 0xff, 0x68, 0x8e, 0x57, 0x96, 0x00]),
           ('wc_asia_league_advance_stage', 0x913490, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('wc_asia_league_create_second_round_groups', 0x913740, [0x6a, 0xff, 0x68, 0xbc, 0x57, 0x96, 0x00])]
DOCS = ('os grupos da primeira e da segunda fase e a fase seguinte das eliminatórias asiáticas',
        'the first and second round groups and the next stage of the Asian World Cup qualifying',
        [('wc_asia_league_create_first_round_group', 'WcAsiaLeague::create_first_round_group', '0x913540', 'wc_asia_league.cpp', 'thiscall, ret 4',
          'regra: cada um dos grupos 1 a 9 da primeira fase é montado com as seleções sorteadas para ele, em ida e volta, e a notícia do sorteio é dada',
          'rule: each of the first round groups 1 to 9 is built with the national teams drawn into it, home and away, and the draw news is sent'),
         ('wc_asia_league_advance_stage', 'WcAsiaLeague::advance_stage', '0x913490', 'wc_asia_league.cpp', 'thiscall, ret 0',
          'regra: depois dos dez grupos da primeira fase vêm os dois grupos da segunda; depois deles, o confronto da terceira fase e então a repescagem',
          'rule: after the ten first round groups come the two second round groups; after them, the third round tie and then the play-off'),
         ('wc_asia_league_create_second_round_groups', 'WcAsiaLeague::create_second_round_groups', '0x913740', 'wc_asia_league.cpp', 'thiscall, ret 0',
          'regra: os dez vencedores da primeira fase são divididos ao acaso em dois grupos de cinco; em cada grupo o primeiro se classifica para a Copa (os dois primeiros quando não há isentos) e o seguinte vai à terceira fase',
          'rule: the ten first round winners are split at random into two groups of five; in each group the winner qualifies for the World Cup (the top two when there are no exempt nations) and the next one goes to the third round')])
CAMPOS = []
INTERESSA = []
