# The season turnover of the Danish First and Second Divisions and the 2. Bundesliga (den_first.cpp, den_second.cpp,
# ger_second.cpp). Reader r3_mwcd (19/11/2026), round 2 (1 call each, the turnover of summer 2002, called by the top
# division's).
# - 0x552450 DenFirst::start_new_season: instruction by instruction the template of TurSecond::start_new_season
#   (0x8ff330, mwd_24): rebuild_turkish_league + finish_croatian_turnover; only the table (0x552290), the file
#   (0x9b8e30) and the lines change.
# - 0x554c00 DenSecond::start_new_season and 0x5df490 GerSecond::start_new_season: instruction by instruction the
#   template of FraSecond::start_new_season (0x5bf160, mwf_7): rebuild_season(.., 1) + set_expectations_and_renew_officials.
# Conventions: thiscall, ret 0. Comparers: only the original (the year +1), as the sisters.
# Mutations: none of their own (the guide: a turnover that only passes year, addresses and lines to the common
# template; what it builds is used by the 2002/03 season, after the chain's date). The templates are proven by mwd_24
# and mwf_7.
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('den_first_start_new_season', 0x552450, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('den_second_start_new_season', 0x554c00, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ger_second_start_new_season', 0x5df490, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a virada das divisões de baixo da Dinamarca e da Alemanha', "the Danish and German lower divisions' season turnover",
        [('den_first_start_new_season', 'DenFirst::start_new_season', '0x552450', 'den_first.cpp', 'thiscall, ret 0',
          'regra: a virada da Primeira Divisão dinamarquesa (chamada pela da Superliga): a temporada reaberta, a tabela, as datas e as estatísticas refeitas para o ano seguinte (as fases ficam), as regras, a tabela, os jogos e a ordem; as posições previstas, as expectativas da diretoria e os árbitros',
          "rule: the Danish First Division turnover (called by the Superliga's): the season reopened, the table, the dates and the statistics rebuilt for the next year (the stages are kept), the rules, the table, the fixtures and the order; the predicted positions, the board expectations and the officials"),
         ('den_second_start_new_season', 'DenSecond::start_new_season', '0x554c00', 'den_second.cpp', 'thiscall, ret 0',
          'regra: a virada da Segunda Divisão dinamarquesa (chamada pela da Superliga): a temporada reaberta, as fases da temporada passada destruídas, o ano seguinte, as regras, a tabela, os jogos e a ordem refeitos; as posições previstas, as expectativas da diretoria e os árbitros',
          "rule: the Danish Second Division turnover (called by the Superliga's): the season reopened, last season's stages destroyed, the next year, the rules, the table, the fixtures and the order rebuilt; the predicted positions, the board expectations and the officials"),
         ('ger_second_start_new_season', 'GerSecond::start_new_season', '0x5df490', 'ger_second.cpp', 'thiscall, ret 0',
          'regra: a virada da 2. Bundesliga (chamada pela da Bundesliga): o mesmo molde da Segunda Divisão dinamarquesa',
          "rule: the 2. Bundesliga turnover (called by the Bundesliga's): the same template as the Danish Second Division")])
CAMPOS = [('liga +0x4c, +0xba, +8, +0xc, +0x30, +0x40', 'league +0x4c, +0xba, +8, +0xc, +0x30, +0x40')]
