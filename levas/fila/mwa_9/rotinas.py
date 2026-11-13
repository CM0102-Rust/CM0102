# The Northern Irish Premier Division (nir_prm.cpp, in leagues\): the constructor, the rules and the table.
# Whole-world front, round 2 (reader mwa, 12/11/2026), career with ENG IRL NIR to 2002-08-01.
# - 0x78e070 (NirPrm::NirPrm, 1 call at the career's creation): JapJ2::JapJ2's template (0x6644c0, same.py: only the
#   vtable, the discipline rule, the rules and table routines, the file and the lines), in one body with the
#   differences as data (construct_irish_league, also used by mwa_10..12).
# - 0x78ed80 (NirPrm::set_up_rules, 2 calls: the constructor and the turnover); the fields as data
#   (set_up_irish_league_rules, also used by mwa_10..12).
# - 0x78ec60 (NirPrm::create_table, 2 calls): ItaSerA::create_table's fill (ita_ser_a_fill_league_table, checked
#   instruction by instruction against 0x641510) and the prize fund, without the TV money.
# Conventions checked at the call sites: the constructor thiscall, ret 8 (push record; push year; mov ecx, new);
# set_up_rules and create_table (mov ecx, esi; call -> thiscall, ret 0). Inside: League::League 0x682200,
# League::load_from_temp_file 0x687b10 (push 1 -> thiscall, ret 4), League::create_fixtures 0x6835c0, League::sort_table
# 0x6827d0 (push 0 -> thiscall, ret 4), operator new 0x944cf1 (cdecl), Comp_Stats::Comp_Stats 0x49ee70 (thiscall, ret
# 4), League::set_predicted_positions 0x68a850, the virtual +0x3c (push 0; push &+0x3a; push &+0xa9; push -1 ->
# thiscall, ret 0x10), League::init_table_row 0x687430 (thiscall, ret 0x10), League::calc_prize_fund 0x682f70
# (thiscall, ret 4), malloc 0x944e46 (cdecl), the error box.
# Comparers: the constructor only the original, with the plan; the rules side by side outside the round table
# (league_regions_without_schedule); the table only the original, with the plan (ita_ser_a_compare_league_table).
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_nir_prm', 0x78e070, [0x6a, 0xff, 0x68, 0xa9, 0xe4, 0x95, 0x00]),
           ('nir_prm_set_up_rules', 0x78ed80, [0x53, 0x56, 0x8b, 0xf1, 0xbb, 0x01, 0x00, 0x00, 0x00]),
           ('nir_prm_create_table', 0x78ec60, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Premier da Irlanda do Norte: o construtor, as regras e a tabela',
        'the Northern Irish Premier Division: the constructor, the rules and the table',
        [('construct_nir_prm', 'NirPrm::NirPrm', '0x78e070', 'nir_prm.cpp', 'thiscall, ret 8',
          'regra: a Premier da Irlanda do Norte na criação da carreira: o ano e a competição do banco de dados, a regra de disciplina 36 (Irlanda do Norte) e, sem nada salvo, uma liga simples (sem fase extra): as regras, a tabela, a tabela de jogos, a ordem, as estatísticas e as posições previstas',
          'rule: the Northern Irish Premier Division at career creation: the year and the database competition, discipline rule 36 (Northern Ireland) and, with nothing saved, a plain league (no extra stage): the rules, the table, the fixtures, the order, the statistics and the predicted positions'),
         ('nir_prm_set_up_rules', 'NirPrm::set_up_rules', '0x78ed80', 'nir_prm.cpp', 'thiscall, ret 0',
          'regra: a Premier da Irlanda do Norte: dez clubes que se enfrentam quatro vezes, 3 pontos por vitória e 1 por empate, desempate por saldo e gols marcados; o último cai para a First Division e o penúltimo joga a repescagem; 3 reservas no banco e 3 substituições; sem liga acima',
          'rule: the Northern Irish Premier Division: ten clubs meeting four times, 3 points for a win and 1 for a draw, ties split by goal difference and goals for; the last goes down to the First Division and the second last plays the play-off; 3 substitutes on the bench and 3 substitutions; no division above'),
         ('nir_prm_create_table', 'NirPrm::create_table', '0x78ec60', 'nir_prm.cpp', 'thiscall, ret 0',
          'regra: a tabela da Premier da Irlanda do Norte tem dez clubes, os da divisão na ordem do banco de dados; cheia, o fundo de prêmios por uma cota de 500 por posição, pago pela classificação no fim da temporada',
          'rule: the Northern Irish Premier Division table has ten clubs, the division\'s in database order; when full, the prize fund for a 500 share per place, paid by table position at the season\'s end')])
CAMPOS = []
INTERESSA = []
