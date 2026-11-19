# The Belgian, Danish and German cup entrants (bel_fa_cup.cpp, den_cup.cpp, ger_cup.cpp). Reader r3_mwcd (19/11/2026),
# round 2 (2 calls each: the constructor in 2001 and the turnover in summer 2002).
# - 0x41d8d0 BelFaCup::collect_entrants: a stack list of 150 (Fourth Division A-D, nation record by pointer:
#   [0xae23a8] + id x 0x122), 0x4afbd0 with 44 and 44 copied (class 3) whatever the count (fewer: stack garbage in the
#   original, zeros in the copy); then three passes (Third/A/B class 2, Second class 1, First class 0) that stop on
#   count == team_count, checked only after adding.
# - 0x551240 DenCup::collect_entrants: four passes, all class 0: lowest division + Denmark (offset < 0x120: 48), Second
#   + Denmark (while below team_count, no stop), First and Superliga (while below, stop on equal).
# - 0x5d9370 GerCup::collect_entrants: the amateur clubs (Regionalliga division with +0x60 set, or Germany's lowest
#   division; B teams out by 0x540a50) into direct entries (last division the 2. Bundesliga, offset < 0x18: 4) or a
#   malloc'd list of 700; 0x4afbd0 with 24; two copy passes up to offset 0xa8 (28), the first zeroing each slot (the
#   second copies zeros when there are too few: defect of the original, kept); a dead loop at 0x5d95ab (only ecx/edi,
#   overwritten) is not reproduced; then the 2. Bundesliga (class 2 when promoted from below) and the Bundesliga.
# Conventions: thiscall, ret 0; 0x944e46, 0x9452ca, 0x4afbd0, 0x540a50 cdecl.
# Comparers: side by side with the random generator restored (compare_entrants_with_seeds, mwd_8), every entry's club,
# class and status. The 2001 call (constructor) shapes the 2001/02 cups.
# Mutations: the Belgian draw among the best 66 instead of 132 (other Fourth Division clubs enter); in Germany up to 2
# instead of 4 relegated amateur clubs go straight in (other amateur clubs enter).
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('bel_fa_cup_collect_entrants', 0x41d8d0, [0x81, 0xec, 0x58, 0x04, 0x00, 0x00]),
           ('den_cup_collect_entrants', 0x551240, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('ger_cup_collect_entrants', 0x5d9370, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00])]
DOCS = ('os inscritos das copas da Bélgica, da Dinamarca e da Alemanha', 'the Belgian, Danish and German cup entrants',
        [('bel_fa_cup_collect_entrants', 'BelFaCup::collect_entrants', '0x41d8d0', 'bel_fa_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Bélgica tem 112 clubes: 44 belgas dos quatro grupos da Quarta Divisão (ordenados por força, os 132 melhores embaralhados) com a classe 3, depois os da Terceira Divisão (a nacional e os grupos A e B) com a classe 2, os da Segunda com a 1 e os da Primeira com a 0, cada grupo na ordem do banco até a lista encher',
          'rule: the Belgian Cup takes 112 clubs: 44 Belgian clubs of the four Fourth Division groups (sorted by strength, the best 132 shuffled) with class 3, then the Third Division clubs (the national one and groups A and B) with class 2, the Second Division\'s with class 1 and the First Division\'s with class 0, each group in database order until the list is full'),
         ('den_cup_collect_entrants', 'DenCup::collect_entrants', '0x551240', 'den_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Dinamarca tem 92 clubes, todos da classe 0: até 48 dinamarqueses da divisão mais baixa, depois os da Segunda Divisão, os da Primeira e os da Superliga, na ordem do banco até a lista encher',
          "rule: the Danish Cup takes 92 clubs, all class 0: up to 48 Danish clubs of the lowest division, then the Second Division's, the First Division's and the Superliga's, in database order until the list is full"),
         ('ger_cup_collect_entrants', 'GerCup::collect_entrants', '0x5d9370', 'ger_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Alemanha tem 64 clubes: até 4 amadores rebaixados da 2. Bundesliga na temporada passada (classe 2), depois os outros amadores (ordenados por força, os 72 melhores embaralhados) até serem 28 (classe 2), depois os da 2. Bundesliga (classe 2 os que subiram de baixo, 1 os outros) e os da Bundesliga (classe 1); amadores são os da divisão da Regionalliga com divisão reserva e os alemães da divisão mais baixa, sem os times B; com poucos amadores, o original completa os 28 com clubes nulos (defeito)',
          "rule: the German Cup takes 64 clubs: up to 4 amateur clubs relegated from the 2. Bundesliga last season (class 2), then the other amateur clubs (sorted by strength, the best 72 shuffled) until 28 are in (class 2), then the 2. Bundesliga's (class 2 for those promoted from below, 1 for the others) and the Bundesliga's (class 1); the amateur clubs are those of the Regionalliga division with a reserve division and the German clubs of the lowest division, without B teams; with too few amateur clubs the original fills the 28 with null clubs (a defect)")])
CAMPOS = [('copa +0x3e número de times, +0xa7 lista de 6 bytes (+4 classe, +5 situação); clube +0x53 nação, +0x57 divisão, +0x5b divisão anterior, +0x60 divisão reserva', 'cup +0x3e team count, +0xa7 6-byte list (+4 class, +5 status); club +0x53 nation, +0x57 division, +0x5b last division, +0x60 reserve division')]
