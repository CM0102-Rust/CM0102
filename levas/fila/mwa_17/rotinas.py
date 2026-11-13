# The Northern Irish League Cup (nir_lge_cup.cpp, in cups\): the constructor, the first group's table and the rules.
# Whole-world front, round 2 (reader mwa, 12/11/2026), career with ENG IRL NIR to 2002-08-01. Uses mwa_16's construct_irish_league_cup and its
# comparer: it goes in the same batch or after it.
# - 0x78c780 (NirLgeCup::NirLgeCup, 1 call): ItaCCup::ItaCCup's template (0x63cc40, same.py: only the vtable, the bench, the discipline
#   rule, the stage and group counts, the routines and the file; the box lines are the same), construct_irish_league_cup.
# - 0x78cfd0 (NirLgeCup::create_table, 2 calls): ItaCCup::init_first_group's template (create_first_group_table, same.py against
#   0x63d5e0: only the count, the file and the line).
# - 0x78d0d0 (NirLgeCup::set_up_rules, 2 calls): ItaCup::set_up_rules's template (set_cup_league_rules: only the tie-break order,
#   the places and the prize).
# Conventions checked at the call sites: the constructor thiscall, ret 8; create_table and set_up_rules (mov ecx, esi;
# call -> thiscall, ret 0). Inside: League::League 0x682200, League::load_from_temp_file 0x687b10 (push 1), malloc
# 0x944e46 (cdecl), operator new 0x944cf1 (cdecl), Comp_Stats::Comp_Stats 0x49ee70 (thiscall, ret 4), the cup's own
# routines (thiscall, ret 0; create_group_stage push g -> thiscall, ret 4), League::create_fixtures 0x6835c0,
# League::sort_table 0x6827d0 (push 0), League::init_table_row 0x687430 (thiscall, ret 0x10), League::send_group_draw_news
# 0x684230 (thiscall, ret 0), the virtual +0x3c (push 0; push &+0x3a; push &+0xa9; push -1 -> thiscall, ret 0x10), the
# error box.
# Comparers: the constructor only the original, with the plan (ita_cups_check_constructor_plan); the first group's
# table only the original, with the plan (compare_first_group_table); the rules only the original, checked after
# (check_cup_league_rules).
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_nir_lge_cup', 0x78c780, [0x6a, 0xff, 0x68, 0x19, 0xe4, 0x95, 0x00]),
           ('nir_lge_cup_create_table', 0x78cfd0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('nir_lge_cup_set_up_rules', 0x78d0d0, [0x53, 0x56, 0x8b, 0xf1, 0xb0, 0x04])]
DOCS = ('a Copa da Liga da Irlanda do Norte: o construtor, a tabela do primeiro grupo e as regras',
        "the Northern Irish League Cup: the constructor, the first group's table and the rules",
        [('construct_nir_lge_cup', 'NirLgeCup::NirLgeCup', '0x78c780', 'nir_lge_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa da Liga da Irlanda do Norte na criação da carreira: o ano e a competição do banco de dados, 5 reservas no banco e 3 substituições, a regra de disciplina 36 (Irlanda do Norte), importância 12; sem nada salvo, 4 fases (3 grupos e o mata-mata), os 20 inscritos, as estatísticas, as posições previstas, as regras, a tabela do primeiro grupo (a copa é a própria liga do primeiro grupo), os seus jogos e ordem e as fases dos 3 grupos',
          "rule: the Northern Irish League Cup at career creation: the year and the database competition, 5 substitutes on the bench and 3 substitutions, discipline rule 36 (Northern Ireland), importance 12; with nothing saved, 4 stages (3 groups and the knockout), the 20 entrants, the statistics, the predicted positions, the rules, the first group's table (the cup is itself the league of the first group), its fixtures and order and the stages of the 3 groups"),
         ('nir_lge_cup_create_table', 'NirLgeCup::create_table', '0x78cfd0', 'nir_lge_cup.cpp', 'thiscall, ret 0',
          'regra: o primeiro grupo da Copa da Liga da Irlanda do Norte tem 5 clubes, os 5 primeiros inscritos, que saem da lista de espera; depois a notícia do sorteio do grupo',
          'rule: the first group of the Northern Irish League Cup has 5 clubs, the first 5 entrants, which leave the waiting list; the group draw news follows'),
         ('nir_lge_cup_set_up_rules', 'NirLgeCup::set_up_rules', '0x78d0d0', 'nir_lge_cup.cpp', 'thiscall, ret 0',
          'regra: os grupos da Copa da Liga da Irlanda do Norte são turnos únicos: 3 pontos por vitória e 1 por empate, desempate por saldo, gols marcados e confronto direto [inferência]; os dois primeiros passam; categoria 4 (copa nacional); sem acesso nem rebaixamento; fundo de prêmios de 100.000',
          'rule: the Northern Irish League Cup groups are single round robins: 3 points for a win and 1 for a draw, ties split by goal difference, goals for and the head-to-head [inference]; the top two go through; category 4 (national cup); no promotion or relegation; a 100,000 prize fund')])
CAMPOS = []
INTERESSA = []
