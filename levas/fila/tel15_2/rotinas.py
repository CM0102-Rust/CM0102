# The tactics screen object (tactics_screens.cpp, 4; reader tel15, 21/11/2026): the set piece takers, the playmakers,
# the penalty order, the slot coordinate, loading, saving and presets, the player instructions, pasting and swapping
# coordinates. All thiscall; ret 0xc set_set_piece_taker and set_player_instruction (the 64-bit flag as two dwords),
# ret 8 the two-argument ones, ret 4 set_preset_formation, ret 0 set_default_penalty_takers and paste_positions.
# The Tactics_Screen::set_set_piece_taker routine is screen_set_piece_taker in Rust (Tactics::set_set_piece_taker
# already has the name).
# Defects of the original: swap_slot_coordinates returns 0 even when it swaps.
# Conventions checked in the disassembly (every push against esp and the ret of each routine); the same bytes in the
# cm0102-1080.exe (0x8a2ca0..0x8ac600 identical).
# Needs tel26_1 (tel26_error_box, display_method, JOB_TEXT and the job colours), tel14_1 (tactics_screen::*,
# TACTICS_SCREENS_CPP, the destructor constants) and tel14_2 (TACTICS_PRESET_NAMES, TACTICS_COLOUR_3154); the
# tel15 items use the helpers of tel15_1 (tactics_screen_state, tactics_screen_calls, tactics_screen_fns,
# tactics_save_undo, compare_tactics_method...): tel15_1 goes first.
TELAS = True

ROTINAS = [('screen_set_piece_taker', 0x8a8ed0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('set_playmaker', 0x8a9020, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('set_penalty_taker', 0x8a9120, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('swap_penalty_takers', 0x8a9270, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('set_default_penalty_takers', 0x8a9370, [0x83, 0xec, 0x64, 0x53, 0x55]),
           ('get_slot_coordinate', 0x8a94d0, [0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('load_formation', 0x8a95d0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('save_formation', 0x8a9700, [0x81, 0xec, 0x38, 0x02, 0x00, 0x00]),
           ('set_preset_formation', 0x8a9950, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('set_player_instruction', 0x8a9c20, [0x53, 0x8b, 0xd9, 0x55, 0x56]),
           ('paste_positions', 0x8a9cb0, [0x55, 0x8b, 0xe9, 0x8b, 0x85, 0x40, 0x0e, 0x00, 0x00]),
           ('swap_slot_coordinates', 0x8a9d70, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o objeto da tela de tática: cobradores, armadores, ordem dos pênaltis, carregar, salvar e as edições da formação',
        'the tactics screen object: takers, playmakers, penalty order, loading, saving and the formation edits',
        [('screen_set_piece_taker', 'Tactics_Screen::set_set_piece_taker', '0x8a8ed0', 'tactics_screens.cpp', 'thiscall',
          'grava o cobrador de bola parada (1ª ou 2ª opção de um tipo 0..4) com a cópia de desfazer; limpar a 1ª opção sobe a 2ª; fora da faixa, a caixa da linha 0x1630; só de leitura, 0',
          'stores a set piece taker (first or second choice of a kind 0..4) with the undo copy; clearing the first choice moves the second up; out of range shows the box of line 0x1630; read-only, 0'),
         ('set_playmaker', 'Tactics_Screen::set_playmaker', '0x8a9020', 'tactics_screens.cpp', 'thiscall',
          'grava o armador (1ª ou 2ª opção) com a cópia de desfazer; limpar a 1ª sobe a 2ª; fora da faixa, a caixa da linha 0x1674',
          'stores the playmaker (first or second choice) with the undo copy; clearing the first moves the second up; out of range shows the box of line 0x1674'),
         ('set_penalty_taker', 'Tactics_Screen::set_penalty_taker', '0x8a9120', 'tactics_screens.cpp', 'thiscall',
          'põe uma linha do elenco num lugar da ordem dos pênaltis, trocando com o lugar antigo dela; lugar fora da faixa, caixa 0x16d0; linha fora, caixa 0x16d7',
          'puts a squad row at a place of the penalty order, swapping with its old place; a place out of range shows box 0x16d0, a row box 0x16d7'),
         ('swap_penalty_takers', 'Tactics_Screen::swap_penalty_takers', '0x8a9270', 'tactics_screens.cpp', 'thiscall',
          'troca dois lugares da ordem dos pênaltis; fora da faixa, a caixa da linha 0x1704',
          'swaps two places of the penalty order; out of range shows the box of line 0x1704'),
         ('set_default_penalty_takers', 'Tactics_Screen::set_default_penalty_takers', '0x8a9370', 'tactics_screens.cpp', 'thiscall',
          'preenche os 11 cobradores de pênalti do clube na tabela de táticas com os jogadores em campo: o menor número de camisa (sem número conta 51), o maior atributo de pênalti no empate, a primeira linha no empate total',
          "fills the club's 11 penalty takers in the Tactics table with the players on the pitch: the lowest squad number (none counts 51), the higher penalties attribute on a tie, the first row on a full tie"),
         ('get_slot_coordinate', 'Tactics_Screen::get_slot_coordinate', '0x8a94d0', 'tactics_screens.cpp', 'thiscall',
          'a coordenada de um lugar da formação na zona anterior (fase da visão anterior; a da visão quando ela é 1); fora da faixa, a caixa da linha 0x1784',
          "a formation slot's coordinate in the previous zone (the previous view's phase; the view's own when it is 1); out of range shows the box of line 0x1784"),
         ('load_formation', 'Tactics_Screen::load_formation', '0x8a95d0', 'tactics_screens.cpp', 'thiscall',
          'carrega um arquivo de formação com a cópia de desfazer (desfeita se falha); arquivo do humano (não pronto) entra nos recentes; caminho nulo, a caixa da linha 0x17b9',
          "loads a formation file with the undo copy (put back if it fails); a human's file (not a preset) enters the recent files; a null path shows the box of line 0x17b9"),
         ('save_formation', 'Tactics_Screen::save_formation', '0x8a9700', 'tactics_screens.cpp', 'thiscall',
          'salva a formação assinada pelo técnico do lugar de rede atual; com técnico de trava, sob a senha dele; sem, numa partida ao vivo do clube do humano também substitui a tática ativa de mesmo nome; o caminho entra nos recentes; caminho nulo, a caixa da linha 0x17f1',
          "saves the formation signed by the current network slot's manager; with a locking manager, under his password; without, in a live match of the human's club it also replaces the active tactic of the same name; the path enters the recent files; a null path shows the box of line 0x17f1"),
         ('set_preset_formation', 'Tactics_Screen::set_preset_formation', '0x8a9950', 'tactics_screens.cpp', 'thiscall',
          'copia uma tática pronta para a formação com a cópia de desfazer; as 8 formações padrão ("4-4-2 Defensive"...) ganham o nome traduzido; índice fora, a caixa da linha 0x1845',
          'copies a preset tactic into the formation with the undo copy; the 8 standard formations ("4-4-2 Defensive"...) get the translated name; an index out of range shows the box of line 0x1845'),
         ('set_player_instruction', 'Tactics_Screen::set_player_instruction', '0x8a9c20', 'tactics_screens.cpp', 'thiscall',
          'grava uma instrução de jogador num lugar da formação com a cópia de desfazer (desfeita se a formação recusa)',
          'sets a player instruction of a formation slot with the undo copy (put back if the formation refuses)'),
         ('paste_positions', 'Tactics_Screen::paste_positions', '0x8a9cb0', 'tactics_screens.cpp', 'thiscall',
          'cola as posições copiadas na visão mostrada: a fase inteira ou a zona mostrada; nada na visão geral ou sem cópia',
          'pastes the copied positions into the view shown: the whole phase or the zone shown; nothing in the overview or with nothing copied'),
         ('swap_slot_coordinates', 'Tactics_Screen::swap_slot_coordinates', '0x8a9d70', 'tactics_screens.cpp', 'thiscall',
          'troca as coordenadas de dois lugares na zona mostrada com a cópia de desfazer; devolve 0 sempre; fora da faixa, a caixa da linha 0x19c0',
          "swaps two slots' coordinates in the zone shown with the undo copy; always returns 0; out of range shows the box of line 0x19c0")])
