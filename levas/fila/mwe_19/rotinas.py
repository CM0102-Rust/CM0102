# The Norwegian Cup (nor_cup.cpp; with mwe_16's turnover the file is whole in Rust). Reader mwe (19/11/2026),
# whole-world round, continuation: the constructor runs once, at career creation; the round table and the entrants at
# creation and at the cup's turnover (docs/cobertura_mundo.txt).
# - 0x78f020 NorCup::NorCup: the national cup constructor template (construct_east_nation_cup, the CroCup one of mwd_8),
#   checked instruction by instruction (discipline rule 22, prize fund 800,000, vtable, entrants, file, box lines).
# - 0x78f2e0 NorCup::create_round_table: seven rounds; the table generated from the disassembly by a symbolic run (all
#   16 fields written in every round); the final's day by two Date::Date (0x549f70) and their comparison (24 or 31
#   October). The original keeps an SEH frame (for the Date locals): no effect on the result.
# - 0x78f890 NorCup::collect_entrants: 128 clubs without "2" in the short name (strstr 0x945a60), the league clubs, the
#   Third Division, then the Norwegian clubs without a division; clears a club's division that points at the cup itself.
# Conventions: thiscall; ctor ret 8 (as mwd_8); round table ret 0x10 (0x521e60 and 0x521eb0 cdecl; 0x549f70 lea ecx,
# local; 4 pushes -> thiscall ret 0x10); entrants ret 0 (0x944e46 cdecl; 0x945a60 2 pushes, add esp, 8 -> cdecl).
# Comparers: the constructor only the original (fields checked); the rounds side by side (compare_cup_round_table); the
# entrants side by side (compare_entrants_with_seeds, every seed compared; the division clearing has the same end state
# in both runs).
# Depends on: EastNationCup / construct_east_nation_cup / compare_east_nation_cup_ctor / compare_entrants_with_seeds
# (mwd_8, batch 301), FraSuperCupRound / cql12r / fra_super_create_single_stage_round_table (fra_super.rs),
# compare_cup_round_table (jap_j_cup.rs).
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_nor_cup', 0x78f020, [0x6a, 0xff, 0x68, 0xf9, 0xe4, 0x95, 0x00]),
           ('nor_cup_create_round_table', 0x78f2e0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('nor_cup_collect_entrants', 0x78f890, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('a Copa da Noruega', 'the Norwegian Cup',
        [('construct_nor_cup', 'NorCup::NorCup', '0x78f020', 'nor_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa da Noruega ao criar a carreira: a regra de disciplina 22, 7 reservas e 3 substituições e, sem estado gravado, os 128 inscritos, o fundo de prêmios de 800 mil, as sete rodadas, a árvore de jogos, as estatísticas e as posições previstas',
          'rule: the Norwegian Cup at career creation: discipline rule 22, 7 substitutes and 3 substitutions and, with no saved state, the 128 entrants, an 800,000 prize fund, the seven rounds, the fixture tree, the statistics and the predicted positions'),
         ('nor_cup_create_round_table', 'NorCup::create_round_table', '0x78f2e0', 'nor_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa da Noruega tem sete rodadas: a primeira (128 clubes) sorteada em 7/4 e jogada em 5/5; a segunda 6/5 e 26/5; a terceira 27/5 e 2/6; a quarta (com jogo extra no empate) 3/6 e 30/6; as quartas 1/7 e 4/8; as semifinais 5/8 e 25/9; a final sorteada em 29/9 e jogada em 24 ou 31 de outubro (31/10 levado ao dia da semana; se isso passar de 1/11, o dia 24), com jogo extra 3 dias depois; sem prêmio por rodada (uma parte do fundo)',
          'rule: the Norwegian Cup has seven rounds: the first (128 clubs) drawn 7/4 and played 5/5; the second 6/5 and 26/5; the third 27/5 and 2/6; the fourth (a replay on a draw) 3/6 and 30/6; the quarter-finals 1/7 and 4/8; the semi-finals 5/8 and 25/9; the final drawn 29/9 and played on 24 or 31 October (31/10 moved to the weekday; if that passes 1/11, the 24th), with a replay 3 days later; no prize per round (a share of the fund)'),
         ('nor_cup_collect_entrants', 'NorCup::collect_entrants', '0x78f890', 'nor_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Noruega inscreve 128 clubes, nenhum com "2" no nome curto (os times reservas): primeiro os da Premier (semente 1), da Primeira Divisão e dos oito grupos da Segunda; depois os da Terceira Divisão; por fim os clubes noruegueses sem divisão; o clube cuja divisão aponta para a própria copa tem a divisão apagada; devolve 1 com a lista cheia',
          'rule: the Norwegian Cup takes 128 clubs, none with "2" in the short name (the reserve sides): first the Premier Division (seed 1), First Division and the eight Second Division groups clubs; then the Third Division; last the Norwegian clubs without a division; a club whose division points at the cup itself has its division cleared; returns 1 with the list full')])
CAMPOS = [('copa +0x3e (inscritos), +0xa7 (6 bytes: clube, semente, situação); clube +0x38 (nome curto), +0x53 (nação), +0x57 (divisão)',
           'cup +0x3e (entrants), +0xa7 (6 bytes: club, seed, status); club +0x38 (short name), +0x53 (nation), +0x57 (division)')]
