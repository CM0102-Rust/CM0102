# The Polish and Greek cup entrants (pol_cup.cpp, pol_lge_cup.cpp, gre_cup.cpp). Reader r3_mwcd (19/11/2026), round 2
# (2 calls each: the constructor in 2001 and the turnover in summer 2002).
# - 0x7c8550 PolCup::collect_entrants: a stack list of 70 (last division the lowest: the promoted first, then those still
#   there; the limit G_CLUB_COUNT - 2 x G_NATION_COUNT read once for both); 0x4afbd0 on the second part only (list +
#   first, n - first, 36 - first); Second Division by last position 13..20 then <= 12 (class 2); the list while below 36
#   (class 3); last season's First Division while the offset is below 0x138 (class 1, the count not kept). Returns 1.
# - 0x7cad30 PolLgeCup::collect_entrants: team_count 36 in 2001 (year +0x40), else 34; four passes, all class 0.
# - 0x5e9a80 GreCup::collect_entrants: not a Cup list: +0xee = calloc(4, 44) of club pointers, +0xf2 = 44 (the League
#   group cup's entrant vector); 16 Third Division clubs (0x4afbd0 with 16; 16 copied whatever the count: zeros in the
#   copy), then the Second and First Divisions while below +0xf2; short: the box at 0x322; returns 1 (0 out of memory).
# Conventions: thiscall, ret 0; 0x944e46, 0x945501, 0x4afbd0 cdecl.
# Comparers: side by side with the random generator restored (compare_entrants_with_seeds, mwd_8; GreCup's own: +0xf2,
# the return and the 44 pointers).
# Mutations: the Greek draw among the best 24 instead of 48 third division clubs; the Polish lowest-division clubs with
# class 2 instead of 3.
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('pol_cup_collect_entrants', 0x7c8550, [0x81, 0xec, 0x24, 0x03, 0x00, 0x00]),
           ('pol_lge_cup_collect_entrants', 0x7cad30, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('gre_cup_collect_entrants', 0x5e9a80, [0x81, 0xec, 0x90, 0x03, 0x00, 0x00])]
DOCS = ('os inscritos das copas da Polônia e da Grécia', 'the Polish and Greek cup entrants',
        [('pol_cup_collect_entrants', 'PolCup::collect_entrants', '0x7c8550', 'pol_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Polônia tem 52 clubes: os da Segunda Divisão que ficaram entre o 13º e o 20º na temporada passada, depois os até o 12º (classe 2); depois os da divisão mais baixa (os que subiram primeiro, depois os outros, ordenados por força e os melhores embaralhados) até serem 36 (classe 3); depois os da Primeira Divisão da temporada passada (classe 1)',
          "rule: the Polish Cup takes 52 clubs: the Second Division clubs placed 13th to 20th last season, then those up to 12th (class 2); then the lowest division clubs (the promoted first, then the others, sorted by strength with the best ones shuffled) until 36 are in (class 3); then last season's First Division clubs (class 1)"),
         ('pol_lge_cup_collect_entrants', 'PolLgeCup::collect_entrants', '0x7cad30', 'pol_lge_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Liga polonesa tem 36 clubes em 2001 e 34 depois, todos da classe 0: os da Segunda Divisão que subiram de baixo, os outros da Segunda, os da Primeira que ficaram abaixo do 7º e os até o 7º, na ordem do banco até a lista encher',
          'rule: the Polish League Cup takes 36 clubs in 2001 and 34 afterwards, all class 0: the Second Division clubs that came up from below, the other Second Division clubs, the First Division clubs placed below 7th and those up to 7th, in database order until the list is full'),
         ('gre_cup_collect_entrants', 'GreCup::collect_entrants', '0x5e9a80', 'gre_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Grécia tem 44 clubes: 16 da Terceira Divisão (ordenados por força, os 48 melhores embaralhados), depois os da Segunda e os da Primeira Divisão na ordem do banco até a lista encher; faltando clubes, a caixa de erro',
          'rule: the Greek Cup takes 44 clubs: 16 Third Division clubs (sorted by strength, the best 48 shuffled), then the Second and First Division clubs in database order until the list is full; short of clubs, the error box')])
CAMPOS = [('copa +0x3e número de times, +0x40 ano, +0xa7 lista de 6 bytes; copa grega +0xee lista de clubes, +0xf2 tamanho; clube +0x57 divisão, +0x5b divisão anterior, +0x5f posição anterior', 'cup +0x3e team count, +0x40 year, +0xa7 6-byte list; Greek cup +0xee club list, +0xf2 its size; club +0x57 division, +0x5b last division, +0x5f last position')]
