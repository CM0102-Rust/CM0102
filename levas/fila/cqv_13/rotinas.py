# As posições previstas e o sorteio dos grupos do Campeonato de Clubes da Oceania; as posições previstas da Copa
# Africana de Nações e da eliminatória da Oceania (oceania_club_champ.cpp, african_nations.cpp, wc_oceania_league.cpp).
# Frente das competições (Agente E, 07/11/2026, madrugada), carreira de 8 ligas (a Austrália jogável: o clube
# australiano joga os grupos de janeiro de 2002).
# - 0x798640 (OceaniaClubChamp::set_predicted_positions): as previstas e o embaralhamento que faz os grupos. Três
#   mutações.
# - 0x4027d0 (AfricanNations::set_predicted_positions) e 0x91ff30 (WcOceaniaLeague::set_predicted_positions): as
#   previstas. Passageiras.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são a virtual +0x5c, thiscall, ret 0. Dentro: 0x945333 (4 pushes ->
# cdecl), 0x4a2540 (2 pushes; mov ecx, [+8] -> thiscall, ret 8), 0x4afb40 (push n (word); push lista; add esp, 8 ->
# cdecl), 0x944e46 e 0x9452ca (push; add esp, 4 -> cdecl).
# Comparadores: só a original, com o plano: as posições gravadas pela reputação (empates podem trocar) e, no sorteio
# da Oceania, a lista depois uma permutação da de antes.
# Determinismo: nada lê pilha, heap ou endereço como valor.
# Provas: em curso.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqv13_oceania_clubes_previstas', 0x798640, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqv13_africana_previstas', 0x4027d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqv13_eliminatoria_oceania_previstas', 0x91ff30, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as posições previstas e o sorteio dos grupos do Campeonato de Clubes da Oceania; as previstas da Copa Africana e da eliminatória da Oceania',
        'the Oceania Club Championship predicted positions and group draw; the African Nations Cup and Oceania qualifying predicted positions',
        [('cqv13_oceania_clubes_previstas', 'OceaniaClubChamp::set_predicted_positions', '0x798640', 'oceania_club_champ.cpp', 'thiscall, ret 0', 'regra: as posições previstas do Campeonato de Clubes da Oceania pela reputação: 1, 2, 3, 3, 5, 5, 7 e 7; depois os oito são embaralhados, e essa ordem faz os grupos (os quatro primeiros no A, os outros no B)', 'rule: the Oceania Club Championship predicted positions by reputation: 1, 2, 3, 3, 5, 5, 7 and 7; then the eight are shuffled, and that order makes the groups (the first four in A, the rest in B)'),
         ('cqv13_africana_previstas', 'AfricanNations::set_predicted_positions', '0x4027d0', 'african_nations.cpp', 'thiscall, ret 0', 'regra: as posições previstas da Copa Africana de Nações pela reputação: 1, 2, 3, 4, quatro 5, quatro 9 e quatro 13', 'rule: the African Nations Cup predicted positions by reputation: 1, 2, 3, 4, four 5ths, four 9ths and four 13ths'),
         ('cqv13_eliminatoria_oceania_previstas', 'WcOceaniaLeague::set_predicted_positions', '0x91ff30', 'wc_oceania_league.cpp', 'thiscall, ret 0', 'regra: as posições previstas da eliminatória da Oceania pela reputação: 1, 2, 3, 3, 5, 5, 7, 7, 9 e, com 10 seleções, 10', 'rule: the Oceania qualifying predicted positions by reputation: 1, 2, 3, 3, 5, 5, 7, 7, 9 and, with 10 nations, 10')])
CAMPOS = [('copa +0xee/+0xef a lista dos clubes ou seleções, +0xf2/+0xf3 a conta; ponteiro/valor; ordena e embaralha (Oceania) ou copia (as outras)', 'cup +0xee/+0xef the club or nation list, +0xf2/+0xf3 its count; pointer/value; sorts and shuffles (Oceania) or copies (the others)'),
          ('registro de estatística +0x72: a posição prevista; valor; grava', 'stats record +0x72: the predicted position; value; writes')]
INTERESSA = ['CMRTE', 'GPF2']
