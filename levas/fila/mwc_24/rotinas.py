# The Belgian Second Division and Eerste Divisie period results and stages (bel_second.cpp, hol_first.cpp). Reader
# r3_mwcd (19/11/2026), round 2 (add_result on every result of the two leagues in 2001/02; advance_stage at the end of
# each Belgian phase, spring 2002).
# - 0x420110 BelSecond::add_result and 0x5f65b0 HolFirst::add_result: instruction by instruction the same mould (only
#   the period bounds and the create_period_stage address change): League::add_result (0x685d30, thiscall, ret 8);
#   with fixture +0x42 = 0xff, the period by the round word +0x34 (Belgium: <10, <22, else; Holland: <8, <16, <26,
#   else); create_period_stage(byte +0x30 + 1) (0x4204d0 / 0x5f6ad0, thiscall, ret 4: push edx with dl the byte) while
#   +0x30 < the period; the stage's virtual +4 (fixture, arg) with +8 (the statistics) set aside; +0x42 = 0xff.
#   Returns 1. (BelThird::add_result, 0x422c50, is another shape: find_or_create_period_stage; left out.)
# - 0x4201a0 BelSecond::advance_stage: stage 3 builds the play-off group (0x420230, thiscall, ret 0); line 0x26e.
# Conventions: thiscall; add_result ret 8, advance_stage ret 0.
# Comparers: add_result only the original (it creates stages), the current stage reaching the round's period and +0x42
# back to 0xff; advance_stage side by side on +0x30 when nothing is built.
# Mutations: the Belgian second period starts at round 12 instead of 10 and the Dutch one at round 10 instead of 8 (the
# period tables, their winners and the play-off places change; the comparer also catches it).
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('bel_second_add_result', 0x420110, [0x53, 0x8b, 0x5c, 0x24, 0x0c, 0x56]),
           ('bel_second_advance_stage', 0x4201a0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('hol_first_add_result', 0x5f65b0, [0x53, 0x8b, 0x5c, 0x24, 0x0c, 0x56])]
DOCS = ('os períodos da Segunda Divisão belga e da Eerste Divisie', 'the Belgian Second Division and Eerste Divisie periods',
        [('bel_second_add_result', 'BelSecond::add_result', '0x420110', 'bel_second.cpp', 'thiscall, ret 8',
          'regra: um resultado da Segunda Divisão belga conta também no seu período: rodadas 1 a 9 o primeiro, 10 a 21 o segundo, 22 em diante o terceiro (a fase de cada período criada quando a temporada chega a ela)',
          'rule: a Belgian Second Division result also counts in its period: rounds 1 to 9 the first, 10 to 21 the second, 22 on the third (each period stage created when the season reaches it)'),
         ('bel_second_advance_stage', 'BelSecond::advance_stage', '0x4201a0', 'bel_second.cpp', 'thiscall, ret 0',
          'regra: a fase seguinte da Segunda Divisão belga: depois da última, a caixa de erro; senão a fase atual sobe uma; ao chegar à fase 3 (depois dos três períodos) é montado o grupo do mata-mata',
          'rule: the next stage of the Belgian Second Division: past the last stage, the error box; otherwise the current stage goes up by one; on reaching stage 3 (after the three periods) the play-off group is built'),
         ('hol_first_add_result', 'HolFirst::add_result', '0x5f65b0', 'hol_first.cpp', 'thiscall, ret 8',
          'regra: um resultado da Eerste Divisie conta também no seu período: rodadas 1 a 7 o primeiro, 8 a 15 o segundo, 16 a 25 o terceiro, 26 em diante o quarto',
          'rule: an Eerste Divisie result also counts in its period: rounds 1 to 7 the first, 8 to 15 the second, 16 to 25 the third, 26 on the fourth')])
CAMPOS = [('liga +8 estatísticas, +0xc fases, +0x2c número de fases, +0x30 fase atual; jogo +0x34 rodada, +0x42 fase', 'league +8 statistics, +0xc stages, +0x2c stage count, +0x30 current stage; fixture +0x34 round, +0x42 stage')]
