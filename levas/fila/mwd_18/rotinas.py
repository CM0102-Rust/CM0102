# The Polish Cup and the Polish League Cup constructors (pol_cup.cpp, pol_lge_cup.cpp). Reader mwd (12/11/2026), round 2
# (the constructors 1 call each at career creation; the round table 2 calls: creation and the turnover).
# - 0x7c7d80 PolCup::PolCup, 0x7ca4e0 PolLgeCup::PolLgeCup: the national cup constructor template (construct_east_nation_cup,
#   mwd_8), checked instruction by instruction against CroCup's 0x517480 (PolLgeCup writes +0x49/+0x4a after the round
#   table instead of before the load).
# - 0x7c8040 PolCup::create_round_table: seven rounds (generated from the disassembly; all 16 fields in every round).
# Conventions: as in mwd_8. Comparers: the constructors only the original; the rounds side by side. Equal in
# cm0102-1080.exe.
# Depends on: construct_east_nation_cup / compare_east_nation_cup_ctor (mwd_8).
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_pol_cup', 0x7c7d80, [0x6a, 0xff, 0x68, 0xe9, 0xf6, 0x95, 0x00]),
           ('pol_cup_create_round_table', 0x7c8040, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('construct_pol_lge_cup', 0x7ca4e0, [0x6a, 0xff, 0x68, 0xe9, 0xf7, 0x95, 0x00])]
DOCS = ('a Copa da Polônia e a Copa da Liga polonesa', 'the Polish Cup and the Polish League Cup',
        [('construct_pol_cup', 'PolCup::PolCup', '0x7c7d80', 'pol_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa da Polônia ao criar a carreira: a regra de disciplina 0x1f (a polonesa), 7 reservas e 3 substituições e, sem estado gravado, os 52 inscritos, o fundo de prêmios de 600 mil, as sete rodadas, a árvore de jogos, as estatísticas e as posições previstas',
          'rule: the Polish Cup at career creation: discipline rule 0x1f (the Polish one), 7 substitutes and 3 substitutions and, with no saved state, the 52 entrants, a 600,000 prize fund, the seven rounds, the fixture tree, the statistics and the predicted positions'),
         ('pol_cup_create_round_table', 'PolCup::create_round_table', '0x7c8040', 'pol_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa da Polônia tem sete rodadas: a preliminar (8 times) sorteada em 6/8 e jogada em 9/8; a primeira fase (32 times, 28 novos) 11/8 e 13/9; a segunda (16 novos) 15/9 e 23/9; as oitavas em ida e volta com 14 dias, 29/9 e 8/11; as quartas em ida e volta, 3/3 e 14/3 do ano seguinte; as semifinais em ida e volta, 1/4 e 11/4; a final em ida e volta com 7 dias, 19/4 e 16/5, 500 mil ao campeão',
          'rule: the Polish Cup has seven rounds: the preliminary round (8 teams) drawn 6/8, played 9/8; the first round (32 teams, 28 new) 11/8 and 13/9; the second round (16 new) 15/9 and 23/9; the round of 16 over two legs 14 days apart, 29/9 and 8/11; the quarter-finals over two legs, 3/3 and 14/3 of the next year; the semi-finals over two legs, 1/4 and 11/4; the final over two legs 7 days apart, 19/4 and 16/5, 500,000 to the winner'),
         ('construct_pol_lge_cup', 'PolLgeCup::PolLgeCup', '0x7ca4e0', 'pol_lge_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa da Liga polonesa ao criar a carreira: a regra de disciplina 0x1f e, sem estado gravado, os inscritos, o fundo de prêmios de 400 mil, as rodadas (7 reservas e 3 substituições gravadas depois delas), a árvore de jogos, as estatísticas e as posições previstas',
          'rule: the Polish League Cup at career creation: discipline rule 0x1f and, with no saved state, its entrants, a 400,000 prize fund, the rounds (7 substitutes and 3 substitutions written after them), the fixture tree, the statistics and the predicted positions')])
CAMPOS = [('copa +4, +0x40, +0xab, +0x44, +0x30, +0x2c, +0x42, +0x49, +0x4a, +0x50, +0xb1, +0x28, +0xa3, +8; a rodada de copa (0x68 bytes)', 'cup +4, +0x40, +0xab, +0x44, +0x30, +0x2c, +0x42, +0x49, +0x4a, +0x50, +0xb1, +0x28, +0xa3, +8; the cup round (0x68 bytes)')]
