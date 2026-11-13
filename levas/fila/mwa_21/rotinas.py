# The Irish Super Cup (ire_super_cup.cpp, in cups\): the constructor, the rules and the schedule.
# Whole-world front, round 2 (reader mwa, 12/11/2026), career with ENG IRL NIR to 2002-08-01. A League (not a Cup): a
# four-club round robin at the end of June, set up once the UEFA seeding reaches its year (mwa_22).
# - 0x63acd0 (IreSuperCup::IreSuperCup, 1 call): the rules written inline (the same fields as set_up_rules, then the
#   round table), no table yet (club count 0, ACTIVE 0), the statistics.
# - 0x63b4c0 (IreSuperCup::set_up_rules, 1 call: the turnover): ItaCup::set_up_rules's template (set_cup_league_rules).
# - 0x63af60 (IreSuperCup::create_schedule, 2 calls): FraThird::create_schedule's template (create_league_schedule; the
#   three dates read by the path-following reader).
# Conventions checked at the call sites: the constructor thiscall, ret 8; set_up_rules (thiscall, ret 0); the schedule
# the virtual +0x3c (thiscall, ret 0x10). Inside: League::League 0x682200, League::load_from_temp_file 0x687b10,
# operator new 0x944cf1 (cdecl), Comp_Stats::Comp_Stats 0x49ee70 (thiscall, ret 4), malloc 0x944e46 (cdecl), 0x68a160 and
# 0x68a1c0 (cdecl), the error box.
# Comparers: the constructor only the original, with the plan; the rules only the original, checked after; the schedule
# side by side (compare_cfa_schedule).
# When the 2002 edition is played (the end of June 2002, if the seeding has moved by then) is not known without running.
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_ire_super_cup', 0x63acd0, [0x6a, 0xff, 0x68, 0x29, 0xb6, 0x95, 0x00]),
           ('ire_super_cup_set_up_rules', 0x63b4c0, [0x53, 0x56, 0x8b, 0xf1, 0xb0, 0x02]),
           ('ire_super_cup_create_schedule', 0x63af60, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Supercopa da Irlanda: o construtor, as regras e a tabela de jogos',
        'the Irish Super Cup: the constructor, the rules and the schedule',
        [('construct_ire_super_cup', 'IreSuperCup::IreSuperCup', '0x63acd0', 'ire_super_cup.cpp', 'thiscall, ret 8',
          'regra: a Supercopa da Irlanda na criação da carreira: o ano e a competição do banco de dados, 7 reservas no banco e 5 substituições, a regra de disciplina 16 (Irlanda); sem nada salvo, as regras (um turno entre quatro clubes) e as três datas, sem clubes ainda (vêm quando as vagas europeias são conhecidas), e as estatísticas',
          'rule: the Irish Super Cup at career creation: the year and the database competition, 7 substitutes on the bench and 5 substitutions, discipline rule 16 (Ireland); with nothing saved, its rules (a four-club round robin) and its three dates, with no clubs yet (they come once the European places are known), and the statistics'),
         ('ire_super_cup_set_up_rules', 'IreSuperCup::set_up_rules', '0x63b4c0', 'ire_super_cup.cpp', 'thiscall, ret 0',
          'regra: a Supercopa da Irlanda é um turno único entre quatro clubes: 3 pontos por vitória e 1 por empate, desempate por confronto direto [inferência], saldo e gols marcados; categoria 2; sem acesso nem rebaixamento',
          'rule: the Irish Super Cup is a single round robin of four clubs: 3 points for a win and 1 for a draw, ties split by the head-to-head [inference], goal difference and goals for; category 2; no promotion or relegation'),
         ('ire_super_cup_create_schedule', 'IreSuperCup::create_schedule', '0x63af60', 'ire_super_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Supercopa da Irlanda é jogada em três rodadas: 27 de junho, 29 de junho e 1º de julho do seu ano, cada data levada ao seu dia da semana',
          'rule: the Irish Super Cup is played over three matchdays: 27 June, 29 June and 1 July of its year, each moved to its weekday')])
CAMPOS = []
INTERESSA = []
