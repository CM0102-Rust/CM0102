# The German League Cup entrants and the Belgian and Dutch super cup clubs (ger_lge_cup.cpp, bel_super.cpp,
# hol_super.cpp). Reader r3_mwcd (19/11/2026), round 2 (2 calls each: the constructor in 2001 and the turnover in
# summer 2002).
# - 0x5dc700 GerLgeCup::collect_entrants: 6 stack slots by last position 1..6 in the Bundesliga's record (pointer
#   compare with [0xae23d0] + id x 0x6b); short of 6 the box at 0x11d and German clubs placed below 6th to the first
#   empty slot; the list by calloc (0x945501, cdecl); entry 5 = the German Cup holder (0x4afd80, the record of
#   0x9cf714, place 0; none: box 0x146, 0); entries 4..0 the first slots that are set and not the holder; classes 0,
#   entries 4 and 5 class 1; returns 1.
# - 0x4212f0 BelSuper::select_teams and 0x5f8c10 HolSuper::select_teams: instruction by instruction the same mould
#   (only the lines, the keys, the classes and BelSuper's swap in 2001 change): champion, cup winner, the runner-up
#   for a double; no fallback (unlike GreSuper/PolSuper, mwd_26): a missing club is a box and 0.
# Conventions: thiscall, ret 0. Comparers: side by side (compare_entrants_with_seeds, mwd_8; compare_team_list, the
# Spanish Super Cup's).
# Mutations: the Belgian and Dutch Super Cups take the league runner-up instead of the champion (another club plays
# the Super Cup; the side-by-side comparer also catches it); the Ligapokal with no class-1 entry (the holder and the
# best placed after him lose their seeding: the first round changes).
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('ger_lge_cup_collect_entrants', 0x5dc700, [0x81, 0xec, 0x18, 0x02, 0x00, 0x00]),
           ('bel_super_select_teams', 0x4212f0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('hol_super_select_teams', 0x5f8c10, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os inscritos da Copa da Liga alemã e os clubes das Supercopas da Bélgica e da Holanda', 'the German League Cup entrants and the Belgian and Dutch super cup clubs',
        [('ger_lge_cup_collect_entrants', 'GerLgeCup::collect_entrants', '0x5dc700', 'ger_lge_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Liga alemã (Ligapokal) tem 6 clubes: os seis primeiros da Bundesliga na temporada passada sem o campeão da Copa da Alemanha, que entra como o sexto; as entradas 4 e 5 (o melhor colocado depois dele e o campeão da copa) têm a classe 1; faltando colocados, a caixa de erro e os alemães colocados abaixo do sexto',
          'rule: the German League Cup (Ligapokal) takes 6 clubs: the Bundesliga\'s top six of last season minus the German Cup holder, who enters as the sixth; entries 4 and 5 (the best-placed one after the holder, and the holder) are class 1; short of placed clubs, the error box and the German clubs placed below sixth'),
         ('bel_super_select_teams', 'BelSuper::select_teams', '0x4212f0', 'bel_super.cpp', 'thiscall, ret 0',
          'regra: a Supercopa da Bélgica é entre o campeão da Primeira Divisão (classe 2) e o campeão da Copa da Bélgica (classe 1); se o mesmo clube ganhou as duas, entra o vice da liga; faltando um, a caixa de erro; em 2001 os dois trocam de lugar',
          'rule: the Belgian Super Cup is between the First Division champion (class 2) and the Belgian Cup winner (class 1); if the same club won both, the league runner-up comes in; a missing club is an error box; in 2001 the two swap places'),
         ('hol_super_select_teams', 'HolSuper::select_teams', '0x5f8c10', 'hol_super.cpp', 'thiscall, ret 0',
          'regra: a Supercopa da Holanda é entre o campeão da Eredivisie e o campeão da Copa da Holanda (classe 0 os dois); se o mesmo clube ganhou as duas, entra o vice da liga; faltando um, a caixa de erro',
          'rule: the Dutch Super Cup is between the Eredivisie champion and the Dutch Cup winner (both class 0); if the same club won both, the league runner-up comes in; a missing club is an error box')])
CAMPOS = [('copa +0x3e número de times, +0x40 ano, +0xa7 lista de 6 bytes (+4 classe, +5 situação); clube +0x53 nação, +0x5b divisão anterior, +0x5f posição anterior', 'cup +0x3e team count, +0x40 year, +0xa7 6-byte list (+4 class, +5 status); club +0x53 nation, +0x5b last division, +0x5f last position')]
