# The tables of the Russian First Division, the Polish Second Division and the Turkish Second Division (rus_first.cpp,
# pol_second.cpp, tur_second.cpp). Reader mwd (12/11/2026), round 2 (2 calls each: career creation and the turnover).
# - 0x7ea6c0 RusFirst::create_table: 18 clubs, at most 18 taken (the rest of the division is left out of the table),
#   sorted by reputation and shuffled (0x4afbd0: list, n, club_count; the words are read, the stack garbage above them
#   ignored), no prize fund nor TV. The branch that moves a surplus club to the south zone never runs (the list is
#   capped at 18).
# - 0x7cbdf0 PolSecond::create_table: 20 clubs in 2001, 18 afterwards; the template of eng_prm_fill_league_table with
#   no TV payment (20,000 a share).
# - 0x8ff150 TurSecond::create_table: 20 clubs of the second division category A (0x9cf8dc); no prize fund (+0x28 = 0,
#   +0xe1 = 2), 100,000 TV money.
# Conventions: thiscall, ret 0; 0x944e46, 0x4afbd0 (3 pushes; add esp, 0xc -> cdecl), 0x687430, 0x682f70, 0x683010.
# Comparers: only the original runs (RusFirst: the planned clubs in any order, the routine draws; PolSecond:
# eng_prm_compare_league_table; TurSecond: compare_division_table and the prize fields). Equal in cm0102-1080.exe.
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('rus_first_create_table', 0x7ea6c0, [0x81, 0xec, 0x48, 0x02, 0x00, 0x00]),
           ('pol_second_create_table', 0x7cbdf0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tur_second_create_table', 0x8ff150, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as tabelas da Primeira Divisão russa, da Segunda Divisão polonesa e da Segunda Divisão turca', 'the tables of the Russian First Division, the Polish Second Division and the Turkish Second Division',
        [('rus_first_create_table', 'RusFirst::create_table', '0x7ea6c0', 'rus_first.cpp', 'thiscall, ret 0',
          'regra: a Primeira Divisão russa tem 18 clubes: os 18 primeiros da divisão na ordem do banco (os demais ficam fora da tabela), ordenados por reputação e embaralhados, nessa ordem; com menos de 18, sem tabela; sem fundo de prêmios nem TV',
          'rule: the Russian First Division has 18 clubs: the first 18 of the division in database order (any more are left out of the table), sorted by reputation and shuffled, in that order; with fewer than 18, no table; no prize fund nor TV money'),
         ('pol_second_create_table', 'PolSecond::create_table', '0x7cbdf0', 'pol_second.cpp', 'thiscall, ret 0',
          'regra: a Segunda Divisão polonesa tem 20 clubes em 2001 e 18 depois, os da divisão na ordem do banco; cheia, o fundo de prêmios (20 mil por cota); sem TV',
          'rule: the Polish Second Division has 20 clubs in 2001 and 18 afterwards, those of the division in database order; when full, the prize fund (20,000 a share); no TV money'),
         ('tur_second_create_table', 'TurSecond::create_table', '0x8ff150', 'tur_second.cpp', 'thiscall, ret 0',
          'regra: a Segunda Divisão turca tem 20 clubes, os da segunda divisão categoria A na ordem do banco; cheia, sem fundo de prêmios (modo 2) e a TV paga 100 mil a cada clube',
          'rule: the Turkish Second Division has 20 clubs, those of the second division category A in database order; when full, no prize fund (mode 2) and TV pays 100,000 to each club')])
CAMPOS = [('liga +0x3e, +0xa7, +0xb1 tabela, +0x28 fundo de prêmios, +0xe1; clube +0x57 divisão', 'league +0x3e, +0xa7, +0xb1 table, +0x28 prize fund, +0xe1; club +0x57 division')]
