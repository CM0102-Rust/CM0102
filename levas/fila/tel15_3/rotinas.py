# The tactics screen object (tactics_screens.cpp, 5; reader tel15, 21/11/2026): the slot coordinate and marking
# edits and the per-row texts and values. All thiscall: ret 8 the (slot/row, value) ones, ret 4
# is_formation_slot_customised, ret 0x14 get_slot_number_text, get_slot_number_and_name_text and
# get_slot_match_status_text, ret 0x18 get_slot_player_name_text, ret 0xc get_slot_pitch_position. The byte
# getters return al.
# Defects of the original: get_slot_match_status_text writes the text into the shared text buffer (0xde1f64), not
# into its output (the only caller passes that buffer).
# Conventions checked in the disassembly (every push against esp and the ret of each routine); the same bytes in the
# cm0102-1080.exe (0x8a2ca0..0x8ac600 identical).
# Needs tel26_1 (tel26_error_box, display_method, JOB_TEXT and the job colours), tel14_1 (tactics_screen::*,
# TACTICS_SCREENS_CPP, the destructor constants) and tel14_2 (TACTICS_PRESET_NAMES, TACTICS_COLOUR_3154); the
# tel15 items use the helpers of tel15_1 (tactics_screen_state, tactics_screen_calls, tactics_screen_fns,
# tactics_save_undo, compare_tactics_method...): tel15_1 goes first.
TELAS = True

ROTINAS = [('set_formation_slot_coordinate', 0x8a9e90, [0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('is_formation_slot_customised', 0x8a9fa0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('set_slot_marking_type', 0x8aa070, [0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('set_slot_marking_target', 0x8aa150, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('get_slot_number_text', 0x8aa200, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('get_slot_player_name_text', 0x8aa550, [0x81, 0xec, 0xd4, 0x02, 0x00, 0x00]),
           ('get_slot_number_and_name_text', 0x8aa830, [0x81, 0xec, 0x90, 0x03, 0x00, 0x00]),
           ('get_slot_pitch_position', 0x8aa9d0, [0x8a, 0x54, 0x24, 0x04, 0x56]),
           ('is_slot_on_bench', 0x8aaaa0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('get_slot_yellow_cards', 0x8aabb0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('get_slot_red_cards', 0x8aacb0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('get_slot_match_status_text', 0x8aadb0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o objeto da tela de tática: as edições de coordenada e marcação e os textos e valores de cada linha',
        'the tactics screen object: the coordinate and marking edits and the texts and values of each row',
        [('set_formation_slot_coordinate', 'Tactics_Screen::set_formation_slot_coordinate', '0x8a9e90', 'tactics_screens.cpp', 'thiscall',
          'grava a coordenada de um lugar na zona mostrada com a cópia de desfazer; fora da faixa, a caixa da linha 0x19e7',
          "sets a slot's coordinate in the zone shown with the undo copy; out of range shows the box of line 0x19e7"),
         ('is_formation_slot_customised', 'Tactics_Screen::is_formation_slot_customised', '0x8a9fa0', 'tactics_screens.cpp', 'thiscall',
          'se o lugar tem posição própria: na visão geral, qualquer coordenada própria; nas outras, na zona mostrada; fora da faixa, a caixa da linha 0x1a0d',
          'whether the slot has a custom position: in the overview, any custom coordinate; in the others, in the zone shown; out of range shows the box of line 0x1a0d'),
         ('set_slot_marking_type', 'Tactics_Screen::set_slot_marking_type', '0x8aa070', 'tactics_screens.cpp', 'thiscall',
          'grava o tipo de marcação de um lugar com a cópia de desfazer; fora da faixa, a caixa da linha 0x1a25',
          "sets a slot's marking type with the undo copy; out of range shows the box of line 0x1a25"),
         ('set_slot_marking_target', 'Tactics_Screen::set_slot_marking_target', '0x8aa150', 'tactics_screens.cpp', 'thiscall',
          'grava o alvo de marcação de um lugar (sem desfazer nem marca de mudança); fora da faixa, a caixa da linha 0x1a49',
          "sets a slot's marking target (no undo, no change mark); out of range shows the box of line 0x1a49"),
         ('get_slot_number_text', 'Tactics_Screen::get_slot_number_text', '0x8aa200', 'tactics_screens.cpp', 'thiscall',
          'o número de uma linha: a camisa do registro, o número do elenco no clube (partida ou jogador escolhido) ou o da linha; sem texto, a abreviação da posição (titulares) ou "SB<n>" (banco); caixa da linha 0x1a74',
          'a row\'s number: the record\'s shirt, the squad number at the club (match or selected player) or the row\'s; without a text, the position abbreviation (the eleven) or "SB<n>" (the bench); box of line 0x1a74'),
         ('get_slot_player_name_text', 'Tactics_Screen::get_slot_player_name_text', '0x8aa550', 'tactics_screens.cpp', 'thiscall',
          'o nome do jogador de uma linha num formato (do registro ou da pessoa), ou o nome da posição; no formato curto, se outro do elenco (ou do time B) tem o mesmo nome curto, o nome no formato 2; caixa da linha 0x1af7',
          "the name of a row's player in a format (from the record or the staff), or the position name; in the short format, when another of the squad (or the B team) has the same short name, the name in format 2; box of line 0x1af7"),
         ('get_slot_number_and_name_text', 'Tactics_Screen::get_slot_number_and_name_text', '0x8aa830', 'tactics_screens.cpp', 'thiscall',
          '"<número> <nome>" de uma linha, ou o que houver; caixa da linha 0x1b6c',
          '"<number> <name>" of a row, or whichever exists; box of line 0x1b6c'),
         ('get_slot_pitch_position', 'Tactics_Screen::get_slot_pitch_position', '0x8aa9d0', 'tactics_screens.cpp', 'thiscall',
          'se a linha está em campo e em que posição: na partida, a do jogador (a anterior se saiu lesionado, fora da disputa de pênaltis); fora dela, as 11 primeiras linhas',
          "whether the row is on the pitch and at which position: in a match, the player's (the previous one when off injured, outside a shoot-out); outside it, the first 11 rows"),
         ('is_slot_on_bench', 'Tactics_Screen::is_slot_on_bench', '0x8aaaa0', 'tactics_screens.cpp', 'thiscall',
          'se a linha é reserva não usado na partida ao vivo; caixa da linha 0x1bfe',
          'whether the row is an unused substitute of the live match; box of line 0x1bfe'),
         ('get_slot_yellow_cards', 'Tactics_Screen::get_slot_yellow_cards', '0x8aabb0', 'tactics_screens.cpp', 'thiscall',
          'os amarelos do jogador da linha na partida ao vivo; caixa da linha 0x1c31',
          "the yellow cards of the row's player in the live match; box of line 0x1c31"),
         ('get_slot_red_cards', 'Tactics_Screen::get_slot_red_cards', '0x8aacb0', 'tactics_screens.cpp', 'thiscall',
          'os vermelhos diretos do jogador da linha na partida ao vivo; caixa da linha 0x1c64',
          "the straight red cards of the row's player in the live match; box of line 0x1c64"),
         ('get_slot_match_status_text', 'Tactics_Screen::get_slot_match_status_text', '0x8aadb0', 'tactics_screens.cpp', 'thiscall',
          'a situação do jogador na partida e a cor: lesionado, expulso, substituído (vermelho) ou entrou (verde), com o minuto, no texto comum; caixa da linha 0x1c9a',
          "the player's match status and colour: injured, sent off, substituted (red) or brought on (green), with the minute, in the shared text; box of line 0x1c9a")])
