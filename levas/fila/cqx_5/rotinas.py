# The predicted positions of the state leagues: Northeast league, Paraná state league, Pernambuco state league (the set_predicted_positions slot of bra_reg_northeast.cpp, bra_reg_parana.cpp, bra_reg_pern.cpp).
# Competitions front (reader cqx, 12/11/2026). Each runs twice in the 8-league career to 2002-06-01
# (docs/cobertura_competicoes.txt): at creation (from the constructor) and at the season turnover. The same mould in
# the twelve (checked instruction by instruction: only the source, the box lines and the fixed knockout places change:
# 1, 2, 3, 3 in the semi-final ones; 1, 2, 3, 3, 5, 5, 5, 5 in the quarter-final ones; 1, 2 in the final ones); items
# cqx_3 to cqx_6 cover the twelve.
# Reuses cqu_4/cqv_1/cqv_2 (out on 07/11: the predicted position +0x72 is only read by the end-of-season average
# attendance, after June: every mutation escaped the career). Now the comparer is side by side (the copy runs after the
# original over the entries it created and must rewrite the same places), so the mutations are caught through the
# confirmation in comparison mode (mutacoes.py --confirmar 2001-08-20: the creation runs on the first day).
# Depends on cqx_3 (the shared mould state_league_predicted_positions and its comparer): integrate after it.
# Conventions checked at the original's call sites: the routines, from the constructor (mov ecx, esi; direct call) and
# the turnover (call [vtable + 0x5c]) -> thiscall, ret 0; 0x944e46 (malloc), 0x9452ca (free): push; add esp, 4 ->
# cdecl; 0x4c08f0 (comp_util_sort_clubs_by_strength): push n; push array; add esp, 8 -> cdecl; 0x4a2540
# (Comp_Stats::set_club_final_position): push place; push club; mov ecx, [esi + 8] -> thiscall, ret 8; the error box
# (show_error_at).
# Determinism: the order comes from the clubs' strength (0x4c0720); nothing reads stack, heap or an address as a value.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('bra_reg_northeast_set_predicted_positions', 0x437290, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('bra_reg_parana_set_predicted_positions', 0x438590, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('bra_reg_pern_set_predicted_positions', 0x4398d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as posições previstas dos estaduais: Nordeste, Paranaense, Pernambucano',
        'the predicted positions of the state leagues: Northeast league, Paraná state league, Pernambuco state league',
        [('bra_reg_northeast_set_predicted_positions', 'BraRegNortheast::set_predicted_positions', '0x437290', 'bra_reg_northeast.cpp', 'thiscall, ret 0', 'regra: as posições previstas do Nordeste (na criação e na virada): os clubes do estadual ordenados pela força; os primeiros ganham os lugares do mata-mata, 1, 2, 3, 3 (as semifinais), e os outros a posição na ordem; é a expectativa da diretoria e da imprensa para cada clube [inferência]', 'rule: the predicted positions of the Northeast league (at creation and at the season turnover): the clubs sorted by strength; the first ones get the knockout places, 1, 2, 3, 3 (the semi-finals), and the others their place in the order; it is the board and press expectation for each club [inference]'),
         ('bra_reg_parana_set_predicted_positions', 'BraRegParana::set_predicted_positions', '0x438590', 'bra_reg_parana.cpp', 'thiscall, ret 0', 'regra: as posições previstas do Paranaense (na criação e na virada): os clubes do estadual ordenados pela força; os primeiros ganham os lugares do mata-mata, 1, 2 (a final entre os dois primeiros), e os outros a posição na ordem; é a expectativa da diretoria e da imprensa para cada clube [inferência]', 'rule: the predicted positions of the Paraná state league (at creation and at the season turnover): the clubs sorted by strength; the first ones get the knockout places, 1, 2 (the final between the top two), and the others their place in the order; it is the board and press expectation for each club [inference]'),
         ('bra_reg_pern_set_predicted_positions', 'BraRegPern::set_predicted_positions', '0x4398d0', 'bra_reg_pern.cpp', 'thiscall, ret 0', 'regra: as posições previstas do Pernambucano (na criação e na virada): os clubes do estadual ordenados pela força; os primeiros ganham os lugares do mata-mata, 1, 2, 3, 3 (as semifinais), e os outros a posição na ordem; é a expectativa da diretoria e da imprensa para cada clube [inferência]', 'rule: the predicted positions of the Pernambuco state league (at creation and at the season turnover): the clubs sorted by strength; the first ones get the knockout places, 1, 2, 3, 3 (the semi-finals), and the others their place in the order; it is the board and press expectation for each club [inference]')])
INTERESSA = ['CMRTE']
