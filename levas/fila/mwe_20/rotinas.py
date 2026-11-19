# The Portuguese Cup (por_cup.cpp; with mwe_16's turnover the file is whole in Rust). Reader mwe (19/11/2026),
# whole-world round, continuation: the constructor runs once, at career creation; the round table and the entrants at
# creation and at the cup's turnover in the summer of 2002 (docs/cobertura_mundo.txt).
# - 0x7cd980 PorCup::PorCup: the WelCup constructor template (construct_british_cup, mwb_3), checked instruction by
#   instruction (no bench written; discipline rule 23, prize fund 800,000, vtable, entrants, file, box lines).
# - 0x7cdc40 PorCup::create_round_table: eight rounds; the table generated from the disassembly by a symbolic run (all
#   16 fields written in every round, no branch).
# - 0x7ce220 PorCup::collect_entrants: 164 clubs (up to 128 lower clubs without the unlinked B teams, the Second
#   Division, the Premier Division) and the swap of the last club with one drawn up to 18 places before it.
# Conventions: thiscall; ctor ret 8 (as mwb_3); round table ret 0x10 (0x521e60 and 0x521eb0 cdecl); entrants ret 0
# (0x944e46 cdecl; 0x540a50 3 pushes, add esp, 0xc -> cdecl; 0x90cfc0 push 0x12, add esp, 4 -> cdecl).
# Comparers: the constructor only the original (then the plan, compare_british_cup_ctor); the rounds side by side
# (compare_cup_round_table); the entrants side by side with the random generator restored (compare_entrants_with_seeds,
# every seed compared).
# Depends on: BritishCupSpec / construct_british_cup / compare_british_cup_ctor (mwb_1), compare_entrants_with_seeds
# (mwd_8), FraSuperCupRound / cql12r / fra_super_create_single_stage_round_table (fra_super.rs), compare_cup_round_table
# (jap_j_cup.rs).
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_por_cup', 0x7cd980, [0x6a, 0xff, 0x68, 0xd9, 0xf9, 0x95, 0x00]),
           ('por_cup_create_round_table', 0x7cdc40, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('por_cup_collect_entrants', 0x7ce220, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('a Taça de Portugal', 'the Portuguese Cup',
        [('construct_por_cup', 'PorCup::PorCup', '0x7cd980', 'por_cup.cpp', 'thiscall, ret 8',
          'regra: a Taça de Portugal ao criar a carreira: a regra de disciplina 23 e, sem estado gravado, os 164 inscritos, o fundo de prêmios de 800 mil, as oito rodadas, a árvore de jogos, as estatísticas e as posições previstas (não escreve o banco)',
          'rule: the Portuguese Cup at career creation: discipline rule 23 and, with no saved state, the 164 entrants, an 800,000 prize fund, the eight rounds, the fixture tree, the statistics and the predicted positions (it does not write the bench)'),
         ('por_cup_create_round_table', 'PorCup::create_round_table', '0x7cdc40', 'por_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Taça de Portugal tem oito rodadas em jogo único, com jogo extra no empate até as semifinais: a primeira (128 clubes das divisões de baixo) sorteada em 10/9 e jogada em 19/9; a segunda (entram os 18 da Segunda Divisão) 23/9 e 5/10; a terceira (entram 17 da Primeira) 18/10 e 14/11; a quarta (entra o último) 29/12 e 9/1; a quinta (15 clubes em 7 jogos, como o original) 10/1 e 26/1; as quartas 3/2 e 9/2; as semifinais 20/3 e 12/4; a final 13/4 e 21/5; sem prêmio por rodada (uma parte do fundo)',
          'rule: the Portuguese Cup has eight single-match rounds, with a replay on a draw up to the semi-finals: the first (128 lower clubs) drawn 10/9 and played 19/9; the second (the 18 Second Division clubs come in) 23/9 and 5/10; the third (17 Premier Division clubs come in) 18/10 and 14/11; the fourth (the last one comes in) 29/12 and 9/1; the fifth (15 clubs in 7 ties, as the original) 10/1 and 26/1; the quarter-finals 3/2 and 9/2; the semi-finals 20/3 and 12/4; the final 13/4 and 21/5; no prize per round (a share of the fund)'),
         ('por_cup_collect_entrants', 'PorCup::collect_entrants', '0x7ce220', 'por_cup.cpp', 'thiscall, ret 0',
          'regra: a Taça de Portugal inscreve 164 clubes: até 128 das divisões de baixo (os grupos da Segunda Divisão B, depois a Terceira Divisão e os portugueses da divisão inferior), sem os times B que não se ligam a clube nenhum; depois os da Segunda Divisão e os da Primeira; por fim troca o último inscrito com um sorteado até 18 lugares antes; devolve 1 com a lista cheia',
          'rule: the Portuguese Cup takes 164 clubs: up to 128 from the lower divisions (the Second Division B groups, then the Third Division and the Portuguese clubs of the lower division), without the B teams linked to no club; then the Second Division and the Premier Division clubs; last it swaps the last entrant with one drawn up to 18 places before; returns 1 with the list full')])
CAMPOS = [('copa +0x3e (inscritos), +0xa7 (6 bytes: clube, semente, situação); clube +0x53 (nação), +0x57 (divisão)',
           'cup +0x3e (entrants), +0xa7 (6 bytes: club, seed, status); club +0x53 (nation), +0x57 (division)')]
