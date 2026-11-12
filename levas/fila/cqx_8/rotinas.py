# The Serie C cup (ita_c_cup.cpp, in cups\): the predicted positions and the clubs' records. Competitions front
# (reader cqx, 12/11/2026), 8-league career (Italy playable).
# Depends on cqx_7 (italian_cup_predicted_positions, compare_italian_cup_predicted, italian_cup_staff_records):
# integrate after it.
# - 0x63e580 (ItaCCup::set_predicted_positions, 1 call: the constructor at the start of the new game): the body of
#   0x640210 (checked instruction by instruction) with stage 0xf and bands of 16.
# - 0x63e270 (ItaCCup::update_staff_records_for_status, 142 calls): reuses cqq_13. Passenger: no mutation (the
#   records go to the history; nothing read before June depends on them [inference]), only the original in the
#   comparer.
# Equal in cm0102-1080.exe (both, whole).
# Conventions checked at the original's call sites: set_predicted_positions from the constructor (0x63cc40: mov ecx,
# esi; call -> thiscall, ret 0); update_staff_records_for_status the virtual +0x44 (6 pushes -> thiscall, ret 0x18).
# The calls inside as in cqx_7.
# Comparers: set_predicted_positions side by side; the records only the original.
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('ita_c_cup_set_predicted_positions', 0x63e580, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00]),
           ('ita_c_cup_update_staff_records_for_status', 0x63e270, [0x8a, 0x44, 0x24, 0x0c, 0x56])]
DOCS = ('a Copa da Série C: as posições previstas e os registros dos clubes',
        "the Serie C cup: the predicted positions and the clubs' records",
        [('ita_c_cup_set_predicted_positions', 'ItaCCup::set_predicted_positions', '0x63e580', 'ita_c_cup.cpp', 'thiscall, ret 0',
          'regra: as posições previstas da Copa da Série C: os 90 inscritos pela reputação; do mata-mata (fase 0xf) de trás para a frente, o campeão 1 e o vice 2, e os eliminados de cada rodada dividem o lugar; os outros em faixas de 16',
          'rule: the Serie C cup predicted positions: the 90 entrants by reputation; from the knockout (stage 0xf) backwards, the winner 1 and the runner-up 2, and the losers of each round share the place; the rest in bands of 16'),
         ('ita_c_cup_update_staff_records_for_status', 'ItaCCup::update_staff_records_for_status', '0x63e270', 'ita_c_cup.cpp', 'thiscall, ret 0x18',
          'regra: na Copa da Série C, como na Copa da Itália, com os grupos abaixo da fase 0xf e a classificação nos grupos na rodada 0x14 (peso 0x28)',
          'rule: in the Serie C cup, as in the Coppa Italia, with the groups below stage 0xf and the group qualification at round 0x14 (weight 0x28)')])
INTERESSA = ['CMRTE']
