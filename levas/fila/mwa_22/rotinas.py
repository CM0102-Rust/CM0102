# The Irish Super Cup (ire_super_cup.cpp, in cups\): the table, the daily update and the season turnover.
# Whole-world front, round 2 (reader mwa, 12/11/2026), career with ENG IRL NIR to 2002-08-01. Uses mwa_1's helpers
# (irish_club_division_id, irish_cup_club_count): it goes in the same batch or after it.
# - 0x63b0c0 (IreSuperCup::create_table, 2 calls), 0x63b300 (IreSuperCup::update_for_date, 2308 calls: every day; the
#   set-up runs once, when the UEFA seeding's season year reaches the cup's) and 0x63b550 (IreSuperCup::start_new_season,
#   1 call).
# Conventions checked at the call sites: create_table (mov ecx, esi; call -> thiscall, ret 0); update_for_date the
# league virtual (push flag; push date -> thiscall, ret 8); start_new_season the virtual +8 (thiscall, ret 0). Inside:
# malloc 0x944e46 (cdecl), League::init_table_row 0x687430 (thiscall, ret 0x10), League::create_fixtures 0x6835c0,
# League::sort_table 0x6827d0 (push 0), the virtuals +0x5c and +0x8c, League::update_for_date 0x6847c0 (thiscall, ret 8),
# League::free_table 0x687970, free 0x9452ca (cdecl), Comp_Stats::archive_season_and_reset 0x4a1c50, the error box.
# Comparers: the table only the original, with the plan (the clubs in the rows); the daily update only the original
# (when due, ACTIVE becomes 1 with a table; otherwise ACTIVE stays); the turnover only the original (the year up by one).
# Defect of the original: the Premier Division filler does not check the clubs already in (a Premier club in Europe
# enters twice when fewer than four Irish clubs are in Europe).
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('ire_super_cup_create_table', 0x63b0c0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ire_super_cup_update_for_date', 0x63b300, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ire_super_cup_start_new_season', 0x63b550, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Supercopa da Irlanda: a tabela, a atualização diária e a virada da temporada',
        'the Irish Super Cup: the table, the daily update and the season turnover',
        [('ire_super_cup_create_table', 'IreSuperCup::create_table', '0x63b0c0', 'ire_super_cup.cpp', 'thiscall, ret 0',
          'regra: a Supercopa da Irlanda tem quatro clubes: os irlandeses na Liga dos Campeões, depois os na Copa da UEFA, depois os na Intertoto, completados pelos clubes da Premier Division na ordem do banco de dados (sem conferir os que já entraram)',
          'rule: the Irish Super Cup has four clubs: the Irish clubs in the Champions League, then those in the UEFA Cup, then those in the Intertoto Cup, completed by the Premier Division clubs in database order (without checking those already in)'),
         ('ire_super_cup_update_for_date', 'IreSuperCup::update_for_date', '0x63b300', 'ire_super_cup.cpp', 'thiscall, ret 8',
          'regra: quando as vagas europeias do ano da Supercopa são conhecidas (a classificação da UEFA chegou a esse ano), a Supercopa da Irlanda pega os seus quatro clubes, sorteia os jogos e a ordem e as posições previstas, uma vez; depois o dia segue como em qualquer liga',
          'rule: once the European places of the Super Cup\'s year are known (the UEFA seeding has moved to that year), the Irish Super Cup takes its four clubs, draws its fixtures and order and its predicted positions, once; then the day goes on as in any league'),
         ('ire_super_cup_start_new_season', 'IreSuperCup::start_new_season', '0x63b550', 'ire_super_cup.cpp', 'thiscall, ret 0',
          'regra: na virada a Supercopa da Irlanda é limpa: a tabela e a tabela de rodadas liberadas, as estatísticas arquivadas, as fases destruídas, o ano avança e as regras (com as novas datas) são refeitas; ela espera inativa pelos novos clubes',
          'rule: at the season turn the Irish Super Cup is cleared: the table and the round table freed, the statistics archived, the stages destroyed, the year moved on and the rules (with the new dates) made again; it waits inactive for its new clubs')])
CAMPOS = [('liga +0x45 ativa (a Supercopa da Irlanda a usa como "já montada nesta temporada"); valor; lê e grava',
           'league +0x45 active (the Irish Super Cup uses it as "already set up this season"); value; reads and writes')]
INTERESSA = []
