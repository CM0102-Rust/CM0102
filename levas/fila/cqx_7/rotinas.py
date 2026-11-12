# The Coppa Italia (ita_cup.cpp, in cups\): the predicted positions, the clubs' records and the news of a club's fate.
# Competitions front (reader cqx, 12/11/2026), 8-league career (Italy playable).
# - 0x640210 (ItaCup::set_predicted_positions, 1 call: the constructor at the start of the new game). The body shared
#   with the Serie C cup's (0x63e580, cqx_8), checked instruction by instruction: only the source file, the box lines,
#   the knockout stage (7 / 0xf) and the band width (8 / 16) change.
# - 0x63ff00 (ItaCup::update_staff_records_for_status, 78 calls): reuses cqq_13 (no PRONTO on 06/11: the weight
#   mutation escaped; the records go to the history, which nothing read before June depends on [inference]). Passenger:
#   no mutation, only the original in the comparer.
# - 0x63f130 (ItaCup::build_fate_news_text, 12 calls): the news text of a club's fate. Comparer: compare_text_writer (main 759c98f9d) on the text-only path; one mutation (the club link type) for it
#   (the text does not reach the impression: only the comparer catches it); the other paths only the original.
# Equal in cm0102-1080.exe (the three, whole).
# Conventions checked at the original's call sites: set_predicted_positions from the constructor (0x63e900: mov ecx,
# esi; call -> thiscall, ret 0); update_staff_records_for_status the virtual +0x44 (6 pushes; mov ecx, cup ->
# thiscall, ret 0x18); build_fate_news_text (10 pushes; mov ecx, cup -> thiscall, ret 0x28). Inside: 0x944e46, 0x9452ca
# (cdecl, 1 push), 0x945333 (qsort, 4 pushes; add esp, 0x10 -> cdecl), the virtual +0x3c (thiscall, ret 0x10), 0x4a2540
# (thiscall, ret 8), 0x4bf850 (4 pushes; add esp, 0x10 -> cdecl), 0x86bdd0/0x86c000 (5 pushes -> thiscall, ret 0x14),
# 0x86c1d0 (4 -> ret 0x10), 0x86a800/0x86b0b0 (3 -> ret 0xc), 0x4b4590 and 0x4b0b80 (6 pushes; add esp, 0x18 ->
# cdecl), 0x4b4420 (4 pushes; add esp, 0x10 -> cdecl), 0x66f4e0 (5 pushes; add esp, 0x14 -> cdecl), 0x4ae660 (thiscall,
# ret 4), 0x4ae8a0 (thiscall, ret 0xc).
# Comparers: set_predicted_positions side by side (the copy after the original over the entries it created: the same
# places); the records only the original; the news text side by side on the text-only path.
# Defect of the original: the predicted positions read the sorted copy with no bound check while giving the knockout
# places (more ties than entrants would read past the array); in the career the counts match.
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('ita_cup_set_predicted_positions', 0x640210, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00]),
           ('ita_cup_update_staff_records_for_status', 0x63ff00, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('ita_cup_build_fate_news_text', 0x63f130, [0x8a, 0x44, 0x24, 0x0c, 0x3c, 0x07])]
DOCS = ('a Copa da Itália: as posições previstas, os registros dos clubes e a notícia do destino de um clube',
        "the Coppa Italia: the predicted positions, the clubs' records and the news of a club's fate",
        [('ita_cup_set_predicted_positions', 'ItaCup::set_predicted_positions', '0x640210', 'ita_cup.cpp', 'thiscall, ret 0',
          'regra: as posições previstas da Copa da Itália: os 48 inscritos pela reputação; do mata-mata (fase 7) de trás para a frente, o campeão 1 e o vice 2, e os eliminados de cada rodada dividem o lugar (3 nas semifinais, 5 nas quartas...); os outros em faixas de 8',
          'rule: the Coppa Italia predicted positions: the 48 entrants by reputation; from the knockout (stage 7) backwards, the winner 1 and the runner-up 2, and the losers of each round share the place (3 in the semi-finals, 5 in the quarter-finals...); the rest in bands of 8'),
         ('ita_cup_update_staff_records_for_status', 'ItaCup::update_staff_records_for_status', '0x63ff00', 'ita_cup.cpp', 'thiscall, ret 0x18',
          'regra: na Copa da Itália, quem passa dos grupos ganha o registro de classificado (rodada 0xa, peso 0x28) e quem não passa o de não classificado; no mata-mata, a eliminação e a classificação valem pela rodada (peso de 50 a 100 nas 5 rodadas), e o campeão e o vice ganham os registros da final',
          'rule: in the Coppa Italia, those who get through the groups get the qualified record (round 0xa, weight 0x28) and those who do not the failed record; in the knockout, elimination and qualification weigh by round (weight 50 to 100 over the 5 rounds), and the winner and runner-up get the final records'),
         ('ita_cup_build_fate_news_text', 'ItaCup::build_fate_news_text', '0x63f130', 'ita_cup.cpp', 'thiscall, ret 0x28',
          'regra: a notícia do destino de um clube na Copa da Itália: nos grupos, a classificação e a eliminação pela notícia genérica de copa; o clube que passa sem objeto de notícia ganha o texto "through to first round" com o link do clube; no mata-mata, a notícia genérica de mata-mata',
          'rule: the news of a club\'s fate in the Coppa Italia: in the groups, qualification and elimination through the generic cup news; a club going through with no news object gets the text "through to first round" with a link to the club; in the knockout, the generic knockout news')])
CAMPOS = [('copa +0xee a lista dos inscritos, +0xf2 quantos (dword); +8 as estatísticas; ponteiro/valor; lê',
           'cup +0xee the entrant list, +0xf2 the count (dword); +8 the statistics; pointer/value; reads')]
INTERESSA = ['CMRTE']
