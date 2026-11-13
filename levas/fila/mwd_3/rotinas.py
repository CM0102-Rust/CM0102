# The Croatian leagues: the tables (cro_a1.cpp, cro_a2a.cpp, cro_a2b.cpp). Reader mwd (12/11/2026), round 2 (2 calls
# each: career creation and the turnover).
# - 0x5147b0 CroA1::create_table, 0x515cb0 CroA2a::create_table, 0x516dd0 CroA2b::create_table: the template of
#   eng_prm_fill_league_table (checked instruction by instruction): 16 clubs; A1 30,000 a share and 100,000 TV money,
#   A2a/A2b 10,000 and 20,000.
# Conventions: thiscall, ret 0; 0x944e46 (cdecl), 0x687430 (push 0; push 0; push club; push row -> thiscall, ret 0x10;
# the row byte pushed in eax with the high bits of the previous value: the routine reads the byte), 0x682f70 (push quota
# -> thiscall, ret 4), 0x683010 (push 0; push value -> thiscall, ret 8).
# Comparer: eng_prm_compare_league_table (only the original; the planned clubs and the prize fund). Equal in
# cm0102-1080.exe.
# Depends on: eng_prm_fill_league_table, eng_prm_pay_tv_rights, eng_prm_compare_league_table (eng_prm.rs).
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('cro_a1_create_table', 0x5147b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cro_a2a_create_table', 0x515cb0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cro_a2b_create_table', 0x516dd0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as ligas croatas: as tabelas', 'the Croatian leagues: the tables',
        [('cro_a1_create_table', 'CroA1::create_table', '0x5147b0', 'cro_a1.cpp', 'thiscall, ret 0', 'regra: a Primeira Divisão croata tem 16 clubes, os da divisão, na ordem do banco; cheia, o fundo de prêmios (30 mil por cota) e a TV paga 100 mil a cada clube', 'rule: the Croatian First Division has 16 clubs, those of the division, in database order; when full, the prize fund (30,000 a share) and TV pays 100,000 to each club'),
         ('cro_a2a_create_table', 'CroA2a::create_table', '0x515cb0', 'cro_a2a.cpp', 'thiscall, ret 0', 'regra: a Segunda Divisão croata, grupo norte, tem 16 clubes; cheia, o fundo de prêmios (10 mil por cota) e a TV paga 20 mil a cada clube', 'rule: the Croatian Second Division north group has 16 clubs; when full, the prize fund (10,000 a share) and TV pays 20,000 to each club'),
         ('cro_a2b_create_table', 'CroA2b::create_table', '0x516dd0', 'cro_a2b.cpp', 'thiscall, ret 0', 'regra: a Segunda Divisão croata, grupo sul, como o grupo norte (16 clubes, 10 mil por cota, 20 mil da TV)', 'rule: the Croatian Second Division south group, as the north group (16 clubs, 10,000 a share, 20,000 TV money)')])
CAMPOS = [('liga +0x3e, +0xb1 tabela, +0x28 fundo de prêmios, +0xe1; clube +0x57 divisão', 'league +0x3e, +0xb1 table, +0x28 prize fund, +0xe1; club +0x57 division')]
