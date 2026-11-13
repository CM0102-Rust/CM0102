# The Croatian Cup (cro_cup.cpp). Reader mwd (12/11/2026), round 2 (the constructor 1 call at career creation; the round
# table and the entrants 2 calls: creation and the turnover).
# - 0x517480 CroCup::CroCup: the national cup constructor template (construct_east_nation_cup), shared with PolCup, PolLgeCup
#   (the substitutes written after the round table), RusCup (the stage vector), TurCup, GreSuper and PolSuper (no prize
#   fund, no rank_teams); checked instruction by instruction.
# - 0x517740 CroCup::create_round_table: six rounds (the table generated from the disassembly; all 16 fields written in
#   every round).
# - 0x517bb0 CroCup::collect_entrants: 48 clubs; the First Division clubs' seed byte (+4) is not written (heap content,
#   as the original): the comparer skips it.
# Conventions: thiscall; ctor ret 8 (0x518640 Cup::Cup thiscall ret 0; 0x51fc00 push 1 -> thiscall ret 4; 0x518790 and
# 0x5223a0 thiscall ret 0; 0x944cf1 cdecl; 0x49ee70 push this -> thiscall ret 4); round table ret 0x10 (0x521e60 and
# 0x521eb0 cdecl); entrants ret 0 (0x944e46 cdecl; 0x90cfc0 push 0x32; add esp, 4 -> cdecl).
# Comparers: the constructor only the original (fields checked); the rounds side by side (compare_cup_round_table); the
# entrants side by side with the random generator restored (compare_entrants_with_seeds). Equal in cm0102-1080.exe.
# Depends on: FraSuperCupRound / cql12r / fra_super_create_single_stage_round_table (fra_super.rs),
# compare_cup_round_table (jap_j_cup.rs). construct_east_nation_cup, compare_east_nation_cup_ctor, call_thiscall_no_args and
# compare_entrants_with_seeds are reused by the other mwd cup items.
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_cro_cup', 0x517480, [0x6a, 0xff, 0x68, 0xc9, 0x74, 0x95, 0x00]),
           ('cro_cup_create_round_table', 0x517740, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cro_cup_collect_entrants', 0x517bb0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('a Copa da Croácia', 'the Croatian Cup',
        [('construct_cro_cup', 'CroCup::CroCup', '0x517480', 'cro_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa da Croácia ao criar a carreira: a regra de disciplina 0x21, 7 reservas e 3 substituições e, sem estado gravado, os 48 inscritos, o fundo de prêmios de 100 mil, as seis rodadas, a árvore de jogos, as estatísticas e as posições previstas',
          'rule: the Croatian Cup at career creation: discipline rule 0x21, 7 substitutes and 3 substitutions and, with no saved state, the 48 entrants, a 100,000 prize fund, the six rounds, the fixture tree, the statistics and the predicted positions'),
         ('cro_cup_create_round_table', 'CroCup::create_round_table', '0x517740', 'cro_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa da Croácia tem seis rodadas: a preliminar (32 clubes, os da Segunda Divisão) sorteada em 16/7 e jogada em 15/8; a primeira fase (entram os 16 da Primeira) 20/8 e 22/9; as oitavas 24/9 e 27/10; as quartas em ida e volta com 14 dias, 29/10 e 7/11; as semifinais em ida e volta com 14 dias, 23/11 e 20/3 do ano seguinte; a final em ida e volta com 7 dias, 4/4 e 24/5; sem prêmio por rodada (uma parte do fundo)',
          'rule: the Croatian Cup has six rounds: the preliminary round (32 clubs, the Second Division ones) drawn 16/7, played 15/8; the first round (the 16 First Division clubs come in) 20/8 and 22/9; the round of 16 24/9 and 27/10; the quarter-finals over two legs 14 days apart, 29/10 and 7/11; the semi-finals over two legs 14 days apart, 23/11 and 20/3 of the next year; the final over two legs 7 days apart, 4/4 and 24/5; no prize per round (a share of the fund)'),
         ('cro_cup_collect_entrants', 'CroCup::collect_entrants', '0x517bb0', 'cro_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Croácia inscreve 48 clubes: primeiro os 32 dos dois grupos da Segunda Divisão, cada um com uma semente sorteada de 0 a 49, depois os 16 da Primeira Divisão, na ordem do banco; a semente dos clubes da Primeira não é escrita (fica o que a memória tinha, como o original)',
          'rule: the Croatian Cup takes 48 clubs: first the 32 of both Second Division groups, each with a random seed 0..49, then the 16 First Division clubs, in database order; the First Division clubs\' seed is not written (it keeps what the memory had, as the original)')])
CAMPOS = [('copa +4, +0x40, +0xab, +0x44, +0x30, +0x2c, +0x42, +0x49, +0x4a, +0x50, +0xb1, +0x28, +0xa3, +8, +0x3e, +0xa7 (6 bytes: clube, semente, situação)', 'cup +4, +0x40, +0xab, +0x44, +0x30, +0x2c, +0x42, +0x49, +0x4a, +0x50, +0xb1, +0x28, +0xa3, +8, +0x3e, +0xa7 (6 bytes: club, seed, status)')]
