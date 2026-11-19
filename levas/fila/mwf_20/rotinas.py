# The Korean FA Cup (kor_fa_cup.cpp; with mwf_2's turnover the file is whole in Rust). Reader mwf (19/11/2026),
# whole-world round, continuation: the constructor runs once, at career creation; the round table and the entrants at
# creation and at the Korean turnover (docs/cobertura_mundo.txt).
# - 0x66a460 KorFaCup::KorFaCup: the WelCup constructor template (construct_british_cup, mwb_3), checked instruction by
#   instruction (no bench written; discipline rule 42, prize fund 200,000, vtable, entrants, file, box lines).
# - 0x66a720 KorFaCup::create_round_table: five rounds; the table generated from the disassembly by a symbolic run (all
#   16 fields written in every round, no branch).
# - 0x66aaf0 KorFaCup::collect_entrants: 32 clubs (22 non-K-League clubs of the nation sorted and shuffled, then the K
#   League); the counter of the local list is a signed byte (cmp bl, 0x64).
# Conventions: thiscall; ctor ret 8 (as mwb_3); round table ret 0x10 (0x521e60 and 0x521eb0 cdecl); entrants ret 0
# (0x944e46 cdecl; 0x4afbd0 3 pushes, add esp, 0xc -> cdecl).
# Comparers: the constructor only the original (then the plan, compare_british_cup_ctor); the rounds side by side
# (compare_cup_round_table); the entrants side by side with the random generator restored (compare_entrants_with_seeds,
# every seed compared).
# Depends on: BritishCupSpec / construct_british_cup / compare_british_cup_ctor (mwb_1), compare_entrants_with_seeds
# (mwd_8), FraSuperCupRound / cql12r / fra_super_create_single_stage_round_table (fra_super.rs), compare_cup_round_table
# (jap_j_cup.rs).
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_kor_fa_cup', 0x66a460, [0x6a, 0xff, 0x68, 0x89, 0xc6, 0x95, 0x00]),
           ('kor_fa_cup_create_round_table', 0x66a720, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('kor_fa_cup_collect_entrants', 0x66aaf0, [0x81, 0xec, 0x94, 0x03, 0x00, 0x00])]
DOCS = ('a Copa da Coreia', 'the Korean FA Cup',
        [('construct_kor_fa_cup', 'KorFaCup::KorFaCup', '0x66a460', 'kor_fa_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa da Coreia ao criar a carreira: a regra de disciplina 42 e, sem estado gravado, os 32 inscritos, o fundo de prêmios de 200 mil, as cinco rodadas, a árvore de jogos, as estatísticas e as posições previstas (não escreve o banco)',
          'rule: the Korean FA Cup at career creation: discipline rule 42 and, with no saved state, the 32 entrants, a 200,000 prize fund, the five rounds, the fixture tree, the statistics and the predicted positions (it does not write the bench)'),
         ('kor_fa_cup_create_round_table', 'KorFaCup::create_round_table', '0x66a720', 'kor_fa_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa da Coreia tem cinco rodadas em jogo único: a primeira (32 clubes) sorteada em 21/10 e jogada em 30/10; as oitavas 31/10 e 3/11; as quartas 4/11 e 7/11; as semifinais 8/11 e 17/11; a final 18/11 e 25/11; sem prêmio por rodada (uma parte do fundo)',
          'rule: the Korean FA Cup has five single-match rounds: the first (32 clubs) drawn 21/10 and played 30/10; the round of 16 31/10 and 3/11; the quarter-finals 4/11 and 7/11; the semi-finals 8/11 and 17/11; the final 18/11 and 25/11; no prize per round (a share of the fund)'),
         ('kor_fa_cup_collect_entrants', 'KorFaCup::collect_entrants', '0x66aaf0', 'kor_fa_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Coreia inscreve 32 clubes: 22 clubes da nação fora da K League (até 100 achados, ordenados pela reputação e embaralhados), depois os da K League; devolve 1 com a lista cheia; defeito do original: copia 22 mesmo que ache menos',
          'rule: the Korean FA Cup takes 32 clubs: 22 clubs of the nation outside the K League (up to 100 found, sorted by reputation and shuffled), then the K League clubs; returns 1 with the list full; defect of the original: it copies 22 even when it finds fewer')])
CAMPOS = [('copa +0x3e (inscritos), +0xa7 (6 bytes: clube, semente, situação); competição +0x5d (nação); clube +0x53 (nação), +0x57 (divisão)',
           'cup +0x3e (entrants), +0xa7 (6 bytes: club, seed, status); competition +0x5d (nation); club +0x53 (nation), +0x57 (division)')]
