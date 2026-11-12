# The Paulista's points for a result; the records of the Asian Cup Winners' Cup and of the Merconorte (bra_reg_sp.cpp
# in leagues\, asia_cup_winner.cpp and con_merc_cup.cpp in eurocomp\). Competitions front (reader cqx, 12/11/2026),
# 8-league career (Brazil and Japan playable).
# - 0x43c4a0 (BraRegSp::points_for_result, 32 bytes, 3,146 calls: every Paulista result): passes the query to League
#   (0x68aad0, already in Rust) with this in ecx untouched.
# - 0x40e260 (AsiaCupWinner::update_staff_records_for_status, 64 calls) and 0x4c2940 (ConMercCup::
#   update_staff_records_for_status, 22 calls). Passengers: no mutation (the records go to the history, which nothing
#   read before June depends on [inference]; see cqq_13), only the original in the comparer.
# Equal in cm0102-1080.exe (the three, whole).
# Conventions checked at the original's call sites: points_for_result (push fixture; push club; mov ecx, league; call
# [vtable] -> thiscall, ret 8); the records, the virtual +0x44 (6 pushes -> thiscall, ret 0x18). Inside: 0x68aad0 (2
# pushes; ecx = this -> thiscall, ret 8), 0x4bf850 (4 pushes; add esp, 0x10 -> cdecl), 0x86bdd0/0x86c000 (5 pushes ->
# thiscall, ret 0x14), 0x86c1d0 (4 -> ret 0x10), 0x86a800/0x86b0b0/0x86b710 (3 -> ret 0xc).
# Comparers: points_for_result side by side (only reads); the records only the original, with the plan of the return.
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('bra_reg_sp_points_for_result', 0x43c4a0, [0x8b, 0x44, 0x24, 0x08, 0x8b, 0x54, 0x24, 0x04]),
           ('asia_cup_winner_update_staff_records_for_status', 0x40e260, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('con_merc_cup_update_staff_records_for_status', 0x4c2940, [0x8a, 0x44, 0x24, 0x0c, 0x56])]
DOCS = ('os pontos de um resultado no Paulista; os registros da Recopa asiática e da Merconorte',
        "the Paulista's points for a result; the records of the Asian Cup Winners' Cup and of the Merconorte",
        [('bra_reg_sp_points_for_result', 'BraRegSp::points_for_result', '0x43c4a0', 'bra_reg_sp.cpp', 'thiscall, ret 8',
          'regra: o Paulista dá os pontos de liga de um resultado (os pontos da vitória e do empate das suas regras), passando a pergunta à base League',
          'rule: the São Paulo state league gives the league points for a result (the win and draw points of its rules), passing the query to the League base'),
         ('asia_cup_winner_update_staff_records_for_status', 'AsiaCupWinner::update_staff_records_for_status', '0x40e260', 'asia_cup_winner.cpp', 'thiscall, ret 0x18',
          'regra: os registros da Recopa asiática: nos mata-matas das zonas (fase 0), a eliminação e a classificação valem pela rodada (peso de 1 a 80 nas 3 rodadas) e o vencedor da zona ganha o registro da rodada 0x82; na final (fase 1), o campeão, o vice e o terceiro ganham os registros da final',
          "rule: the Asian Cup Winners' Cup records: in the zone knockouts (stage 0), elimination and qualification weigh by round (1 to 80 over the 3 rounds) and the zone winner gets the round 0x82 record; in the final (stage 1) the winner, the runner-up and the third get the final records"),
         ('con_merc_cup_update_staff_records_for_status', 'ConMercCup::update_staff_records_for_status', '0x4c2940', 'con_merc_cup.cpp', 'thiscall, ret 0x18',
          'regra: os registros da Merconorte: nos grupos (fases abaixo de 3), o classificado ganha o registro da rodada 0x82 e quem sai o de não classificado (peso 0x28); no mata-mata (fase 3), a eliminação e a classificação valem pela rodada (peso de 50 a 100 nas 2 rodadas), e o campeão e o vice ganham os registros da final',
          'rule: the Merconorte records: in the groups (stages below 3) the qualified club gets the round 0x82 record and the club going out the failed record (weight 0x28); in the knockout (stage 3) elimination and qualification weigh by round (50 to 100 over the 2 rounds), and the winner and runner-up get the final records')])
INTERESSA = ['CMRTE']
