# The tables of the Greek First and Second Divisions and the Russian Premier League (gre_prm.cpp, gre_second.cpp,
# rus_prm.cpp). Reader mwd (12/11/2026), round 2 (2 calls each: career creation and the turnover).
# - 0x5ead60 GrePrm::create_table, 0x5ebcb0 GreSecond::create_table, 0x7eb610 RusPrm::create_table: the template of
#   eng_prm_fill_league_table (checked instruction by instruction against CroA1's 0x5147b0): Greece 14 clubs, 100,000 a
#   share and 500,000 TV money (both divisions); Russia 16 clubs, 50,000 a share and 300,000 TV money.
# Conventions: thiscall, ret 0 (the callees as in mwd_3).
# Comparer: eng_prm_compare_league_table (only the original). Equal in cm0102-1080.exe.
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('gre_prm_create_table', 0x5ead60, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('gre_second_create_table', 0x5ebcb0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rus_prm_create_table', 0x7eb610, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as tabelas das divisões gregas e da liga principal russa', 'the tables of the Greek divisions and the Russian Premier League',
        [('gre_prm_create_table', 'GrePrm::create_table', '0x5ead60', 'gre_prm.cpp', 'thiscall, ret 0', 'regra: a Primeira Divisão grega tem 14 clubes, os da divisão, na ordem do banco; cheia, o fundo de prêmios (100 mil por cota) e a TV paga 500 mil a cada clube', 'rule: the Greek First Division has 14 clubs, those of the division, in database order; when full, the prize fund (100,000 a share) and TV pays 500,000 to each club'),
         ('gre_second_create_table', 'GreSecond::create_table', '0x5ebcb0', 'gre_second.cpp', 'thiscall, ret 0', 'regra: a Segunda Divisão grega tem 14 clubes; cheia, o mesmo fundo (100 mil por cota) e a mesma TV (500 mil) da Primeira', 'rule: the Greek Second Division has 14 clubs; when full, the same prize fund (100,000 a share) and TV money (500,000) as the First Division'),
         ('rus_prm_create_table', 'RusPrm::create_table', '0x7eb610', 'rus_prm.cpp', 'thiscall, ret 0', 'regra: a liga principal russa tem 16 clubes; cheia, o fundo de prêmios (50 mil por cota) e a TV paga 300 mil a cada clube', 'rule: the Russian Premier League has 16 clubs; when full, the prize fund (50,000 a share) and TV pays 300,000 to each club')])
CAMPOS = [('liga +0x3e, +0xb1 tabela, +0x28 fundo de prêmios, +0xe1; clube +0x57 divisão', 'league +0x3e, +0xb1 table, +0x28 prize fund, +0xe1; club +0x57 division')]
