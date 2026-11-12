# The Intertoto match (the UEFA coefficient points); the records of the LDV Vans Trophy and of the Oceania Club
# Championship (intertoto_cup.cpp and oceania_club_champ.cpp in eurocomp\, eng_auto_cup.cpp in cups\). Competitions
# front (reader cqx, 12/11/2026), 8-league career.
# - 0x633290 (IntertotoCup::process_fixture, 401 calls): the coefficient part of EuropeanCup::process_fixture (0x583b10,
#   in Rust) without the prizes, then Cup::process_fixture.
# - 0x56bee0 (EngAutoCup::update_staff_records_for_status, 110 calls; reuses cqq_13) and 0x797a90
#   (OceaniaClubChamp::update_staff_records_for_status, 16 calls). Passengers: no mutation (the records go to the
#   history, which nothing read before June depends on [inference]; see cqq_13), only the original in the comparer.
# Equal in cm0102-1080.exe (the three, whole).
# Conventions checked at the original's call sites: process_fixture is the virtual (push a2; push fixture; mov ecx,
# cup -> thiscall, ret 8); the records, the virtual +0x44 (6 pushes -> thiscall, ret 0x18). Inside: 0x9058b0 (push
# points; push nation; mov ecx, [0xde1f58] -> thiscall, ret 8), 0x51a150 (push a2; push fixture; mov ecx, this ->
# thiscall, ret 8), 0x4bf850 (4 pushes; add esp, 0x10 -> cdecl), the staff records (thiscall: 0x86bdd0/0x86c000 ret
# 0x14, 0x86c1d0 ret 0x10, 0x86a800/0x86b0b0 ret 0xc).
# Comparers: process_fixture in the Champions League mould (the copy first in a dry run, the coefficient words read and
# put back, then the original); the records only the original.
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('intertoto_cup_process_fixture', 0x633290, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('eng_auto_cup_update_staff_records_for_status', 0x56bee0, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('oceania_club_champ_update_staff_records_for_status', 0x797a90, [0x8a, 0x44, 0x24, 0x0c, 0x3c, 0x01])]
DOCS = ('o jogo da Intertoto (os pontos de coeficiente da UEFA); os registros do LDV Vans Trophy e do Campeonato de Clubes da Oceania',
        'the Intertoto match (the UEFA coefficient points); the records of the LDV Vans Trophy and of the Oceania Club Championship',
        [('intertoto_cup_process_fixture', 'IntertotoCup::process_fixture', '0x633290', 'intertoto_cup.cpp', 'thiscall, ret 8',
          'regra: cada jogo da Intertoto dá pontos de coeficiente da UEFA às nações dos clubes: 2 à nação do vencedor, 1 a cada uma no empate (pelo placar da prorrogação, quando houve); depois o jogo segue como em qualquer copa',
          "rule: each Intertoto match gives UEFA coefficient points to the clubs' nations: 2 to the winner's nation, 1 to each on a draw (by the score after extra time, when there was extra time); then the match goes on as in any cup"),
         ('eng_auto_cup_update_staff_records_for_status', 'EngAutoCup::update_staff_records_for_status', '0x56bee0', 'eng_auto_cup.cpp', 'thiscall, ret 0x18',
          'regra: no LDV Vans Trophy, nas chaves a eliminação e a classificação valem pela rodada (peso de 1 a 80 nas 5 rodadas) e o vencedor da chave ganha o registro de classificado para a final (rodada 0x96, peso 0x5a); na final, o campeão e o vice ganham os registros da final',
          'rule: in the LDV Vans Trophy, in the sections elimination and qualification weigh by round (weight 1 to 80 over the 5 rounds) and the section winner gets the record of reaching the final (round 0x96, weight 0x5a); in the final, the winner and runner-up get the final records'),
         ('oceania_club_champ_update_staff_records_for_status', 'OceaniaClubChamp::update_staff_records_for_status', '0x797a90', 'oceania_club_champ.cpp', 'thiscall, ret 0x18',
          'regra: os registros do Campeonato de Clubes da Oceania: nos grupos, o classificado ganha o registro da rodada 0x82 e quem sai o de não classificado (peso 0x19); na final, o campeão e o vice ganham os registros da final',
          'rule: the Oceania Club Championship records: in the groups the qualified club gets the round 0x82 record and the club going out the failed record (weight 0x19); in the final the winner and runner-up get the final records')])
INTERESSA = ['CMRTE']
