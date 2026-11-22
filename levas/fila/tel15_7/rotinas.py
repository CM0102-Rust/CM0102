# The tactics screen (tactics_screens.cpp, 6; reader tel15b, the screens round, 21/11/2026): the leave-the-match
# confirmation text, the pitch drawing and the point of a position on it (with the tel16 group: draw_pitch and
# get_pitch_point_for_position_flags).
# Conventions checked in the disassembly (every push against esp and the ret of each routine): build_return_to_match_text
# thiscall(this, Text*) ret 4 (0x3a74 bytes of locals, _chkstk); draw_pitch cdecl(left, top, right, bottom, out left,
# out top, out right, out bottom, across, area, float scale) ret 0, returns 1; get_pitch_point_for_position_flags
# cdecl(flags, left, top, right, bottom, out x, out y, wide) ret 0, returns 1 or 0. Text::clear thiscall ret 0,
# Text::add_link ret 0xc, Text::set_string ret 4; langlib_sprintf and the CRT's sprintf cdecl; match_man_get_captain_score
# cdecl(0, match player, club). The same bytes in the cm0102-1080.exe (0x8a2db0..0x8adbb0 identical).
# draw_pitch on the x87: the coordinates are integers plus multiples of the float scale (constants 1, 6, 10, 12, 18 and
# 22 of the .rdata), truncated by the game's _ftol; the copy does the same in f64 (the game runs the x87 in double
# precision, as rng.rs already assumes). The colours pushed as words are zero-extended (register leftovers above).
# Assert that cannot fail left out in build_return_to_match_text: the row index of the takers' loop (line 0x1b8e).
# Defects of the original: build_return_to_match_text compares the set piece takers with the player record's
# address; with a non-empty message but no FIXTURE it leaves the links in the Text and returns 0.
# Needs tel26_1, tel14_1, tel14_2 (tactics_item, tactics_calls) and tel15_1 (tactics_screen_state,
# tactics_row_match_player, compare_tactics_method).
TELAS = True

ROTINAS = [('build_return_to_match_text', 0x8a8630, [0xb8, 0x74, 0x3a, 0x00, 0x00]),
           ('draw_pitch', 0x8ac600, [0xa0, 0x20, 0x1e, 0xde, 0x00]),
           ('get_pitch_point_for_position_flags', 0x8ad780, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a tela de tática (6): o texto de voltar à partida e o desenho do campo',
        'the tactics screen (6): the return to the match text and the pitch drawing',
        [('build_return_to_match_text', 'Tactics_Screen::build_return_to_match_text', '0x8a8630', 'tactics_screens.cpp', 'thiscall',
          'monta o texto de confirmação de voltar à partida ao vivo: as substituições feitas (mais de 3, só a quantidade por extenso; senão cada uma com camisa e nome), o capitão escolhido automaticamente quando o atual não está em campo (a maior nota de capitão, o primeiro no empate) e a lista dos cobradores de bola parada que não estão mais em campo, cada jogador com link; com texto e o jogo marcado, acrescenta "Voltar à partida?", grava no Text e devolve 1; senão 0',
          'builds the confirmation text for returning to the live match: the substitutions made (more than 3, only the count in words; else each one with shirt and name), the captain picked automatically when the current one is not on the pitch (the highest captain score, the first on a tie) and the list of the set piece takers no longer on the pitch, each player linked; with a text and the fixture set, adds "Return to the match ?", sets the Text and returns 1; else 0'),
         ('draw_pitch', 'draw_pitch', '0x8ac600', 'tactics_screens.cpp', 'cdecl',
          'desenha um campo de futebol num retângulo da tela (itens da lista de exibição): o gramado com três faixas, as linhas laterais a 2 vezes a escala para dentro (devolvidas nas quatro saídas), a linha do meio, o círculo central, e em cada ponta a meia-lua, a marca do pênalti, a grande e a pequena área, em unidades da escala; deitado (gols à esquerda e à direita) ou em pé; devolve 1',
          'draws a football pitch in a rectangle of the screen (display list items): the grass with three stripes, the touchlines 2 x the scale inside (returned in the four outputs), the halfway line, the centre circle, and at each end the penalty arc, the penalty spot, the penalty area and the goal area, in units of the scale; laid across (goals left and right) or upright; returns 1'),
         ('get_pitch_point_for_position_flags', 'get_pitch_point_for_position_flags', '0x8ad780', 'tactics_screens.cpp', 'cdecl',
          'o ponto de uma posição no campo desenhado: o goleiro no meio, 10 acima do fundo; as outras pela linha (bits 2 a 0x40, de baixo para cima) e pela coluna (bits 0x80 a 0x800: décimos com largura larga, senão oitavos); sem linha ou sem coluna, a caixa de erro e 0',
          'the point of a position on the drawn pitch: the goalkeeper in the middle, 10 above the bottom; the others by the row (bits 2 to 0x40, bottom up) and the column (bits 0x80 to 0x800: tenths when wide, else eighths); without a row or a column, the error box and 0')])
