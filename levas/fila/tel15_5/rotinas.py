# The tactics screen (tactics_screens.cpp, 4; reader tel15b, the screens round, 21/11/2026): the team instructions
# dialog, building it and its events.
# Conventions checked in the disassembly (every push against esp and the ret of each routine): both cdecl, ret 0, the
# builder without arguments (0x8a4830, the read-only builder the tactics handler opens, is a 5-byte jmp to 0x8a2db0:
# not hooked), the handler with the item; Display::add_area thiscall ret 0x2c, add_item ret 0x48 (tel26_1/tel14_2's
# wrappers), ScreenManager::add_text_item ret 0x2c, add_area_text ret 0x24, add_area_button ret 0x2c, add_button_item
# ret 0x30; draw_pitch cdecl with the float scale (2.0) as the 11th argument; Formation::set_team_instruction thiscall
# ret 4; Tactics_Screen::set_set_piece_taker ret 0xc and set_playmaker ret 8. The same bytes in the cm0102-1080.exe.
# The colours pushed as words are zero-extended (the original's high halves are register leftovers; the Guio keeps
# only the word, as tel14_2 checked).
# Asserts that cannot fail left out: the taker kind and choice and the playmaker index (error boxes of lines 0x1618
# and 0x165f; the static table 0x97039c only holds kinds 0..3 and choices 0..1).
# Defects of the original: the handler frees the copy on Cancel and Ok without clearing parameter 0; the set piece
# takers (and playmakers) are compared with the player record's address (get_slot_player) as dwords.
# Needs tel26_1, tel14_1, tel14_2 (tactics_item, tactics_area, tactics_text*, tactics_calls, TACTICS_EMPTY,
# TACTICS_COLOUR_3154) and tel15_1 (tactics_screen_state, tactics_screen_calls, tactics_screen_fns,
# tactics_save_undo/restore_undo, compare_tactics_method); tel15_6 uses this item's helpers: this one goes first.
TELAS = True

ROTINAS = [('tactics_build_team_instructions_dialog', 0x8a2db0, [0xa0, 0x10, 0x1d, 0xde, 0x00]),
           ('tactics_team_instructions_dialog_handle_event', 0x8a4840, [0x53, 0x55, 0x56, 0x57, 0x6a, 0x00])]
DOCS = ('a tela de tática (4): o diálogo de instruções do time',
        'the tactics screen (4): the team instructions dialog',
        [('tactics_build_team_instructions_dialog', 'tactics_build_team_instructions_dialog', '0x8a2db0', 'tactics_screens.cpp', 'cdecl',
          'monta o diálogo de instruções do time sobre a cópia da tela de tática: a caixa (menor só de leitura), o título, Cancelar e Ok (ou só Ok, só de leitura), um campo desenhado e as sete linhas de instruções (Mentalidade, Passe, Desarme, Pressão, Linha de Impedimento, Contra-Ataque, Homens Atrás da Bola), marcando a escolha atual pelos bits das instruções do time da formação; editável, os cobradores de falta (esquerda e direita), de escanteio (esquerda e direita) e os dois armadores, cada um com a lista dos jogadores em campo (menos o da outra escolha) e o escolhido embaixo (cinza se não está em campo); a segunda escolha só aparece com a primeira',
          "builds the team instructions dialog over the tactics screen copy: the box (smaller read-only), the title, Cancel and Ok (only Ok read-only), a pitch drawing and the seven instruction rows (Mentality, Passing, Tackling, Pressing, Offside Trap, Counter Attack, Men Behind Ball), ticking the current choice by the bits of the formation's team instructions; editable, the free kick takers (left and right), the corner takers (left and right) and the two playmakers, each with the list of the players on the pitch (but the one of the other choice) and the chosen one under it (greyed when not on the pitch); the second choice only shows once the first is set"),
         ('tactics_team_instructions_dialog_handle_event', 'tactics_team_instructions_dialog_handle_event', '0x8a4840', 'tactics_screens.cpp', 'cdecl',
          'trata os eventos do diálogo de instruções do time: Cancelar descarta a cópia; Ok copia a cópia sobre o objeto da tela (só se editável) e a descarta; uma instrução grava na formação da cópia com desfazer (recusada, volta a cópia de desfazer); cobradores e armadores gravam na cópia; o resto vai para a barra de menu',
          "handles the team instructions dialog events: Cancel throws the copy away; Ok copies it over the screen's object (editable only) and throws it away; an instruction is set in the copy's formation with undo (refused, the undo copy goes back); takers and playmakers are set in the copy; the rest goes to the menu bar")])
