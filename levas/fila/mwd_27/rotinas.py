# The Turkish and Russian cup entrants (tur_cup.cpp, rus_cup.cpp). Reader mwd (12/11/2026), round 2 (2 calls each: the
# constructor and the turnover).
# - 0x8fcb90 TurCup::collect_entrants: local lists of 30 (category B) and 20 (category A); category B sorted by qsort
#   (0x945333 with 0x4c0a40, compare_clubs_by_last_division) and its entries 25..0 copied (26 whatever the count: with
#   fewer the original reads stack garbage, the copy zeros); category A reversed; the top flight from the end.
# - 0x7e99a0 RusCup::select_entrants: the holder (0x4afd80 on the cup's own record, place 0); zone clubs (by record
#   pointer) sorted and shuffled (0x4afbd0 with 14); First Division from entry 31 backwards (class 2, the holder 1),
#   then up to 14 zone clubs while fewer than 32 (with fewer than 14 zone clubs the original reads stack garbage, the
#   copy zeros), then the Premier League from entry 32 on (class 1). Returns 1 when the list is full.
# Conventions: thiscall, ret 0; 0x944e46, 0x945333 (4 pushes; add esp, 0x10 -> cdecl), 0x4afd80, 0x4afbd0 (cdecl).
# Comparers: side by side with the random generator restored (compare_entrants_with_seeds, mwd_8), every entry's club,
# class and status. Equal in cm0102-1080.exe.
# Depends on: compare_entrants_with_seeds (mwd_8), RUSSIAN_ZONE_PAIRS (mwd_23).
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('tur_cup_collect_entrants', 0x8fcb90, [0x81, 0xec, 0x48, 0x03, 0x00, 0x00]),
           ('rus_cup_select_entrants', 0x7e99a0, [0x81, 0xec, 0xb0, 0x03, 0x00, 0x00])]
DOCS = ('os inscritos das copas da Turquia e da Rússia', 'the Turkish and Russian cup entrants',
        [('tur_cup_collect_entrants', 'TurCup::collect_entrants', '0x8fcb90', 'tur_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Turquia inscreve 64 clubes, todos da classe 1: 26 da segunda divisão categoria B (os 30 primeiros na ordem do banco, ordenados pela última divisão, tomados do 26º para trás), depois os 20 primeiros da categoria A em ordem inversa, e os da elite do fim da lista para trás',
          'rule: the Turkish Cup takes 64 clubs, all class 1: 26 of the second division category B (the first 30 in database order, sorted by their last division, taken from the 26th backwards), then the first 20 of category A in reverse order, and the top-flight clubs from the end of the list backwards'),
         ('rus_cup_select_entrants', 'RusCup::select_entrants', '0x7e99a0', 'rus_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Rússia inscreve 48 clubes: os da Primeira Divisão (classe 2; o atual campeão da copa entre eles, classe 1) da 32ª vaga para trás, depois os das zonas da segunda divisão (ordenados por força, os 42 melhores embaralhados) até completar 32, classe 2; depois os da liga principal (classe 1) da 33ª vaga em diante; dá certo quando a lista fica cheia',
          'rule: the Russian Cup takes 48 clubs: the First Division clubs (class 2; the cup holder among them class 1) from entry 31 backwards, then the second division zone clubs (sorted by strength, the best 42 shuffled) until 32 are in, class 2; then the Premier League clubs (class 1) from entry 32 on; it succeeds when the list is full')])
CAMPOS = [('copa +4, +0x3e, +0xa7 (6 bytes: clube, classe, situação); clube +0x57 divisão', 'cup +4, +0x3e, +0xa7 (6 bytes: club, class, status); club +0x57 division')]
