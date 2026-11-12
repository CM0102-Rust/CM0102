# The UEFA Cup club list; the domestic cup semi-final losers in the UEFA seeding; the Mercosur records (uefa_cup.cpp
# and conmebol_merc.cpp in eurocomp\, uefa_seeding.cpp in comp\). Competitions front (reader cqx, 12/11/2026),
# 8-league career.
# - 0x903120 (UefaCup::collect_club_list, 3 calls): pure (it allocates its own list).
# - 0x905790 (UefaSeeding::record_cup_semi_final_losers, 8 calls: the European domestic cups' semi-finals): the 50
#   slots of 12 bytes at +0x440 (the slot of cqa_4's CAMPOS).
# - 0x4c5960 (ConmebolMerc::update_staff_records_for_status, 37 calls). Passenger: no mutation (the records go to the
#   history, which nothing read before June depends on [inference]; see cqq_13), only the original in the comparer.
# Equal in cm0102-1080.exe (the three, whole).
# Conventions checked at the original's call sites: collect_club_list is the virtual +0x58 (push kind; push &out; mov
# ecx, cup -> thiscall, ret 8); record_cup_semi_final_losers (push b; push a; push id; mov ecx, [0xde1f58]; no add esp
# -> thiscall, ret 0xc); the records, the virtual +0x44 (6 pushes -> thiscall, ret 0x18). Inside: 0x944e46 (push;
# add esp, 4 -> cdecl), the error box (show_error_at), 0x4bf850 (4 pushes; add esp, 0x10 -> cdecl), the staff records
# (thiscall: 0x86bdd0/0x86c000 ret 0x14, 0x86a800/0x86b0b0 ret 0xc).
# Comparers: collect_club_list side by side (the copy with its own out, its list freed); record_cup_semi_final_losers
# side by side over the same object (the copy after the original finds the cup's slot and rewrites it); the records
# only the original, with the plan of the return.
# Proof: neither mutation changes the career up to June; the comparers catch them only when the confirmation in
# comparison mode reaches the calls (the semi-finals are in spring 2002; the list's 3 calls, date to be seen in the
# counter "cqx_12 uefa cup round clubs").
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('uefa_cup_collect_club_list', 0x903120, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('uefa_seeding_record_cup_semi_final_losers', 0x905790, [0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('conmebol_merc_update_staff_records_for_status', 0x4c5960, [0x8a, 0x44, 0x24, 0x0c, 0x56])]
DOCS = ('a lista dos clubes da Copa da UEFA; os semifinalistas eliminados das copas nacionais na semeadura da UEFA; os registros da Mercosul',
        'the UEFA Cup club list; the domestic cup semi-final losers in the UEFA seeding; the Mercosur records',
        [('uefa_cup_collect_club_list', 'UefaCup::collect_club_list', '0x903120', 'uefa_cup.cpp', 'thiscall, ret 8',
          'regra: a lista dos clubes da Copa da UEFA: de uma rodada (tipo 1: o número de rodadas − 8, tipo 2: − 7, tipo 3: − 4) os dois clubes de cada confronto, na ordem dos confrontos; senão (ou sem essa rodada) todos os inscritos da tabela de times',
          'rule: the UEFA Cup club list: of one round (kind 1: the round count − 8, kind 2: − 7, kind 3: − 4) the two clubs of each tie, in tie order; otherwise (or with no such round) every entrant of the team table'),
         ('uefa_seeding_record_cup_semi_final_losers', 'UefaSeeding::record_cup_semi_final_losers', '0x905790', 'uefa_seeding.cpp', 'thiscall, ret 0xc',
          'regra: os dois semifinalistas eliminados da copa nacional de uma nação europeia ficam guardados na semeadura da UEFA (na vaga da copa, ou na primeira livre de 50), para as vagas da Copa da UEFA da temporada seguinte; copa de outro continente fica de fora',
          "rule: the two losing semi-finalists of a European nation's domestic cup are kept in the UEFA seeding (in the cup's slot, or the first free of 50), for the next season's UEFA Cup places; a cup of another continent is left out"),
         ('conmebol_merc_update_staff_records_for_status', 'ConmebolMerc::update_staff_records_for_status', '0x4c5960', 'conmebol_merc.cpp', 'thiscall, ret 0x18',
          'regra: os registros da Mercosul: nos grupos (fases abaixo de 4), o classificado ganha o registro da rodada 0x78 (peso 0x28); no mata-mata (fase 4), a eliminação e a classificação valem pela rodada (peso de 50 a 100 nas 2 rodadas), e o campeão e o vice ganham os registros da final',
          'rule: the Mercosur records: in the groups (stages below 4) the qualified club gets the round 0x78 record (weight 0x28); in the knockout (stage 4) elimination and qualification weigh by round (50 to 100 over the 2 rounds), and the winner and runner-up get the final records')])
CAMPOS = [('semeadura da UEFA [0xde1f58] +0x440: 50 vagas de 12 bytes (a competição, os dois clubes); grava',
           'UEFA seeding [0xde1f58] +0x440: 50 slots of 12 bytes (the competition, the two clubs); writes')]
INTERESSA = ['CMRTE']
