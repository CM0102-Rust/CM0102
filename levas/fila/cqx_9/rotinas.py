# The season turnover of the Asian Super Cup and of the CONCACAF Champions' Cup; the Asian Club Championship records
# (asia_super_cup.cpp, con_champ.cpp, asia_club_champ.cpp, in eurocomp\). Competitions front (reader cqx, 12/11/2026),
# 8-league career (Japan and the USA playable).
# - 0x410c50 (AsiaSuperCup::start_new_season, 1 call to 2002-06-01): only in June (Date::get_month, 0-based, = 5);
# - 0x4c1440 (ConChamp::start_new_season, 1 call);
# - 0x40c5b0 (AsiaClubChamp::update_staff_records_for_status, 64 calls). Passenger: no mutation (the records go to the
#   history, which nothing read before June depends on [inference]; see cqq_13), only the original in the comparer.
# The free-season block (cup_turnover_free_season) is the same in the two turnovers (checked instruction by
# instruction); unlike the Brazilian/Japanese cups' (start_new_cup_season), no entrant routine in ConChamp and the first
# round set up in AsiaSuperCup.
# Equal in cm0102-1080.exe (the three, whole).
# Conventions checked at the original's call sites: start_new_season is the virtual +8 (mov ecx; call [eax + 8] ->
# thiscall, ret 0); update_staff_records_for_status the virtual +0x44 (6 pushes -> thiscall, ret 0x18). Inside:
# 0x54cfb0 (mov ecx, 0xae2c90 -> thiscall, ret 0), 0x9452ca (push; add esp, 4 -> cdecl), 0x4a1c50 (push 1 -> thiscall,
# ret 4), 0x945501 (push 6; push n; add esp, 8 -> cdecl), 0x410a50 (mov ecx -> thiscall, ret 0), the virtuals +0x8c,
# +0x94, +0x5c (thiscall, ret 0), 0x51c800 (push 0 -> thiscall, ret 4), 0x4bf850 (4 pushes; add esp, 0x10 -> cdecl),
# the staff records (thiscall: 0x86bdd0/0x86c000 ret 0x14, 0x86a800/0x86b0b0/0x86b710 ret 0xc).
# Comparers: only the original (the turnovers free and archive; the records go to the history), with the plan.
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('asia_super_cup_start_new_season', 0x410c50, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('con_champ_start_new_season', 0x4c1440, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('asia_club_champ_update_staff_records_for_status', 0x40c5b0, [0x8a, 0x44, 0x24, 0x0c, 0x56])]
DOCS = ('a virada da Supercopa da Ásia e da Copa dos Campeões da CONCACAF; os registros da Liga dos Campeões asiática',
        "the season turnover of the Asian Super Cup and of the CONCACAF Champions' Cup; the Asian Club Championship records",
        [('asia_super_cup_start_new_season', 'AsiaSuperCup::start_new_season', '0x410c50', 'asia_super_cup.cpp', 'thiscall, ret 0',
          'regra: a Supercopa da Ásia só vira em junho: a copa terminada é limpa, as estatísticas arquivadas e o ano avança; depois os dois participantes, as regras, a árvore dos confrontos, a primeira rodada e as posições previstas de novo',
          'rule: the Asian Super Cup turns over only in June: the finished cup is cleared, its statistics archived and the year moves on; then the two entrants, the rules, the tie tree, the first round and the predicted positions again'),
         ('con_champ_start_new_season', 'ConChamp::start_new_season', '0x4c1440', 'con_champ.cpp', 'thiscall, ret 0',
          'regra: a virada da Copa dos Campeões da CONCACAF: a copa terminada é limpa, as estatísticas arquivadas e o ano avança; uma tabela de times vazia para os participantes (escolhidos depois), as regras e a árvore dos confrontos de novo',
          "rule: the CONCACAF Champions' Cup turnover: the finished cup is cleared, its statistics archived and the year moves on; an empty team table for the entrants (chosen later), the rules and the tie tree again"),
         ('asia_club_champ_update_staff_records_for_status', 'AsiaClubChamp::update_staff_records_for_status', '0x40c5b0', 'asia_club_champ.cpp', 'thiscall, ret 0x18',
          'regra: os registros da Liga dos Campeões asiática: nos mata-matas das zonas (fase 0), a eliminação e a classificação valem pela rodada (peso de 1 a 80 nas 2 rodadas) e o vencedor da zona ganha o registro de chegar às finais (rodada 0x78, peso 0x5a); nas fases 1 e 2 o classificado ganha o registro da rodada 0x82; nas finais (fase 3), o campeão, o vice e o terceiro ganham os registros da final',
          'rule: the Asian Club Championship records: in the zone knockouts (stage 0), elimination and qualification weigh by round (1 to 80 over the 2 rounds) and the zone winner gets the record of reaching the finals (round 0x78, weight 0x5a); in stages 1 and 2 the qualified club gets the round 0x82 record; in the finals (stage 3) the winner, the runner-up and the third get the final records')])
CAMPOS = [('copa +0x4c, +0xa3, +0xa7, +0xad (as tabelas da temporada), +0x40 o ano, +0xab, +0xb1; valor/ponteiro; grava',
           'cup +0x4c, +0xa3, +0xa7, +0xad (the season tables), +0x40 the year, +0xab, +0xb1; value/pointer; writes')]
INTERESSA = ['CMRTE']
