# The Finnish Cup (fin_cup.cpp; with mwe_16's turnover the file is whole in Rust). Reader mwe (19/11/2026), whole-world
# round, continuation: the constructor runs once, at career creation; the round table and the entrants at creation and
# at the cup's turnover (docs/cobertura_mundo.txt).
# - 0x592250 FinCup::FinCup: the national cup constructor template (construct_east_nation_cup, the CroCup one of mwd_8),
#   checked instruction by instruction (only the discipline rule 11, the prize fund 200,000, the vtable, the entrants
#   routine, the file and the box lines change).
# - 0x592510 FinCup::create_round_table: seven rounds; the table generated from the disassembly by a symbolic run of the
#   body (all 16 fields written in every round, no branch).
# - 0x592a20 FinCup::collect_entrants: 76 clubs (44 lower clubs sorted and shuffled, the First Division, the Premier
#   Division below 5th, the Premier Division top 5).
# Conventions: thiscall; ctor ret 8 (0x518640 Cup::Cup thiscall ret 0; 0x51fc00 push 1 -> thiscall ret 4; 0x518790 and
# 0x5223a0 thiscall ret 0; 0x944cf1 cdecl; 0x49ee70 push this -> thiscall ret 4); round table ret 0x10 (0x521e60 and
# 0x521eb0 cdecl; 0x944e46 cdecl); entrants ret 0 (0x944e46 cdecl; 0x4afbd0 3 pushes, add esp, 0xc -> cdecl).
# Comparers: the constructor only the original (fields checked); the rounds side by side (compare_cup_round_table); the
# entrants side by side with the random generator restored (compare_entrants_with_seeds, every seed compared).
# Depends on: EastNationCup / construct_east_nation_cup / compare_east_nation_cup_ctor / compare_entrants_with_seeds
# (cro_cup, mwd_8, batch 301), FraSuperCupRound / cql12r / fra_super_create_single_stage_round_table (fra_super.rs),
# compare_cup_round_table (jap_j_cup.rs).
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_fin_cup', 0x592250, [0x6a, 0xff, 0x68, 0xb9, 0x8e, 0x95, 0x00]),
           ('fin_cup_create_round_table', 0x592510, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('fin_cup_collect_entrants', 0x592a20, [0x81, 0xec, 0x44, 0x03, 0x00, 0x00])]
DOCS = ('a Copa da Finlândia', 'the Finnish Cup',
        [('construct_fin_cup', 'FinCup::FinCup', '0x592250', 'fin_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa da Finlândia ao criar a carreira: a regra de disciplina 11, 7 reservas e 3 substituições e, sem estado gravado, os 76 inscritos, o fundo de prêmios de 200 mil, as sete rodadas, a árvore de jogos, as estatísticas e as posições previstas',
          'rule: the Finnish Cup at career creation: discipline rule 11, 7 substitutes and 3 substitutions and, with no saved state, the 76 entrants, a 200,000 prize fund, the seven rounds, the fixture tree, the statistics and the predicted positions'),
         ('fin_cup_create_round_table', 'FinCup::create_round_table', '0x592510', 'fin_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa da Finlândia tem sete rodadas em jogo único, decididas no dia: a primeira (64 clubes, os da Segunda Divisão e abaixo) sorteada em 15/4 e jogada em 29/4; a segunda 3/5 e 16/5; a terceira (entram os 8 da Primeira Divisão) 5/6 e 21/6; a quarta (entram os 4 primeiros da Premier) 22/6 e 25/7; as quartas 27/7 e 19/9; as semifinais 21/9 e 17/10; a final 18/10 e 10/11; sem prêmio por rodada (uma parte do fundo)',
          'rule: the Finnish Cup has seven single-leg rounds, settled on the day: the first (64 clubs, the Second Division and lower ones) drawn 15/4 and played 29/4; the second 3/5 and 16/5; the third (the 8 First Division clubs come in) 5/6 and 21/6; the fourth (the Premier Division top 4 come in) 22/6 and 25/7; the quarter-finals 27/7 and 19/9; the semi-finals 21/9 and 17/10; the final 18/10 and 10/11; no prize per round (a share of the fund)'),
         ('fin_cup_collect_entrants', 'FinCup::collect_entrants', '0x592a20', 'fin_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Finlândia inscreve 76 clubes: 44 da Segunda Divisão e abaixo (até 80 achados, ordenados pela reputação e embaralhados), depois os da Primeira Divisão, depois os da Premier que terminaram abaixo do 5º lugar e por fim os 5 primeiros; os da Premier com semente 1; devolve 1 com a lista cheia; defeito do original: copia 44 mesmo que ache menos',
          'rule: the Finnish Cup takes 76 clubs: 44 from the Second Division and below (up to 80 found, sorted by reputation and shuffled), then the First Division clubs, then the Premier Division clubs that finished below 5th and last the top 5; the Premier Division clubs with seed 1; returns 1 with the list full; defect of the original: it copies 44 even when it finds fewer')])
CAMPOS = [('copa +0x3e (inscritos), +0xa7 (6 bytes: clube, semente, situação); clube +0x57 (divisão), +0x5f (posição na última temporada)',
           'cup +0x3e (entrants), +0xa7 (6 bytes: club, seed, status); club +0x57 (division), +0x5f (last season position)')]
