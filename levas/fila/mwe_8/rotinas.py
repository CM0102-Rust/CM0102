# The Portuguese tables and the Liga de Honra turnover (reader mwe, whole-world round, 12/11/2026):
# - 0x7cf800 PorPrm::create_table (304 bytes): at the creation and at the turnover; the body of
#   eng_prm_fill_league_table (cqi_5), 18 clubs, share 100,000, TV money 1,000,000 to every club;
# - 0x7d1520 PorSecond::create_table (384 bytes): the body of spa_second_create_table (cqm_6), 18 clubs, the B team of a
#   Premier Division club with status 0xfc, share 50,000;
# - 0x7d1760 PorSecond::start_new_season (448 bytes): the Portuguese turnover in the summer of 2002 (the date the
#   career reaches it is the Portuguese season start, before 2002-08-01); the body of rebuild_season +
#   set_expectations_and_renew_officials (the Brazilian Series B, cqq_8).
# Equal in cm0102-1080.exe (whole).
# Conventions checked: the tables are called directly (mov ecx, esi; call) -> thiscall, ret 0; the turnover through
# vtable (PorPrm::start_new_season calls the other Portuguese leagues' virtual) -> thiscall, ret 0. Inside: malloc
# (cdecl), League::init_table_row (thiscall, ret 0x10), Club::get_linked_b_team 0x540a50 (push 1; push &flag; push
# club; add esp, 0xc -> cdecl; it writes the flag on both of its non-zero paths, checked at 0x5412a7 and 0x54187c),
# calc_prize_fund (push share -> thiscall, ret 4), pay_tv_rights (push 0; push value -> thiscall, ret 8); the turnover
# as rebuild_season (free_table push 0, free, archive_season_and_reset push 1, virtual 0 push 1, vtable +0x8c,
# create_fixtures, sort_table push 0, vtable +0x5c, set_club_season_expectations, renew_officials push record with
# ecx = [0xb74340]).
# Comparers: the tables, only the original (they allocate and pay) against the plan; the turnover, only the original,
# the year + 1 (compare_state_league_new_season).
# Determinism: the row is pushed as a stack dword with only the low byte set (garbage above); League::init_table_row
# reads only the byte. Nothing else reads stack, heap or an address as a value.
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('por_prm_create_table', 0x7cf800, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('por_second_create_table', 0x7d1520, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('por_second_start_new_season', 0x7d1760, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as tabelas das duas divisões portuguesas e a virada da Liga de Honra',
        'the tables of the two Portuguese divisions and the Liga de Honra turnover',
        [('por_prm_create_table', 'PorPrm::create_table', '0x7cf800', 'por_prm.cpp', 'thiscall, ret 0',
          'a primeira divisão portuguesa tem 18 clubes, os da divisão na ordem do banco de dados; o prêmio pela cota de 100.000 por posição e 1.000.000 de dinheiro da TV para cada clube',
          'the Portuguese Premier Division has 18 clubs, those of the division in database order; the prize fund by a share of 100,000 per place and 1,000,000 of TV money to every club'),
         ('por_second_create_table', 'PorSecond::create_table', '0x7d1520', 'por_second.cpp', 'thiscall, ret 0',
          'a Liga de Honra tem 18 clubes, na ordem do banco de dados; o time B de um clube da primeira divisão não pode subir (situação −4); o prêmio pela cota de 50.000 por posição; sem dinheiro da TV',
          'the Liga de Honra has 18 clubs, in database order; the B team of a Premier Division club cannot go up (status −4); the prize fund by a share of 50,000 per place; no TV money'),
         ('por_second_start_new_season', 'PorSecond::start_new_season', '0x7d1760', 'por_second.cpp', 'thiscall, ret 0',
          'a virada da Liga de Honra: arquiva a temporada nas estatísticas, limpa a tabela, as datas e as fases, passa ao ano seguinte e refaz as regras, a tabela, os jogos e a ordem; depois as posições previstas, as expectativas da diretoria e a renovação dos árbitros',
          'the Liga de Honra turnover: archives the season in the statistics, clears the table, the dates and the stages, moves to the next year and rebuilds the rules, the table, the fixtures and the order; then the predicted positions, the board expectations and the referee renewal')])
CAMPOS = [('linha da tabela +0x37 situação: 0xfc (não pode subir) para o time B; valor; grava (por_second_create_table)',
           'table row +0x37 status: 0xfc (cannot go up) for the B team; value; writes (por_second_create_table)')]
INTERESSA = []
