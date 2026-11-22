# The tactics screen (tactics_screens.cpp, 5; reader tel15b, the screens round, 21/11/2026): the player instructions
# dialog, building it and its events.
# Conventions checked in the disassembly (every push against esp and the ret of each routine): both cdecl, ret 0, the
# builder without arguments, the handler with the item; the display wrappers as in tel15_5; draw_pitch with scale 4.0;
# Formation::get_player_instructions thiscall ret 4 (64 bits in edx:eax), get_marking_type and get_marking_target ret 4,
# set_player_instruction ret 0xc; Tactics_Screen::get_slot_number_and_name_text ret 0x14, get_slot_marking_target_row
# ret 8, set_player_instruction ret 0xc, set_slot_marking_type and set_slot_marking_target ret 8. The same bytes in the
# cm0102-1080.exe.
# The rows and slots the original pushes as whole dwords whose upper bytes are stack leftovers go with them zeroed
# (the callees read the low byte), except the slot of Formation::set_player_instruction in the handler, which keeps
# the item's upper bytes as the original's reused argument slot does.
# Assert that cannot fail left out: the opponent row index (error box of line 0x1b8e).
# Defects of the original: the handler's low-dword instruction checks READ_ONLY but the high-dword, marking type and
# marked opponent events do not; the dialog has no read-only case; the copy is freed without clearing parameter 2.
# Needs tel15_5 (tactics_dialog_*, tactics_screen_assign) and what it needs: tel15_5 goes first.
TELAS = True

ROTINAS = [('tactics_build_player_instructions_dialog', 0x8a5330, [0xa0, 0x28, 0x1e, 0xde, 0x00]),
           ('tactics_player_instructions_dialog_handle_event', 0x8a6f80, [0x83, 0xec, 0x08, 0x53, 0x55])]
DOCS = ('a tela de tática (5): o diálogo de instruções do jogador',
        'the tactics screen (5): the player instructions dialog',
        [('tactics_build_player_instructions_dialog', 'tactics_build_player_instructions_dialog', '0x8a5330', 'tactics_screens.cpp', 'cdecl',
          'monta o diálogo de instruções do jogador de uma linha: o título com número e nome, a lista para trocar de jogador (os outros em campo), Cancelar e Ok, um campo desenhado, as 13 linhas de instruções (Passe, Desarme, Pressão, Passar Para, Bolas Paradas no ataque e na defesa, Liberdade, Avanços, Conduzir a Bola, Segurar a Bola, Chutes de Longe, Passes em Profundidade, Cruzamentos) marcando a escolha pelos 64 bits de instruções da posição, e a marcação (Zona, Nenhuma, Individual); na individual, a lista dos adversários, o marcado e o texto Adversário',
          "builds the player instructions dialog of a row: the title with number and name, the list to switch player (the others on the pitch), Cancel and Ok, a pitch drawing, the 13 instruction rows (Passing, Tackling, Pressing, Pass To, Set Pieces attacking and defending, Free Role, Forward Runs, Run With Ball, Hold Up Ball, Long Shots, Try Through Balls, Cross Ball) ticking the choice by the position's 64 instruction bits, and the marking (Zonal, None, Man); with man marking, the list of the opponents, the marked one and the Opposition text"),
         ('tactics_player_instructions_dialog_handle_event', 'tactics_player_instructions_dialog_handle_event', '0x8a6f80', 'tactics_screens.cpp', 'cdecl',
          'trata os eventos do diálogo de instruções do jogador: outro jogador troca a linha; Cancelar devolve a cópia guardada sobre o objeto da tela e Ok só limpa o desfazer, os dois descartam a cópia; as instruções (com desfazer), o tipo de marcação e o adversário marcado gravam no objeto da tela; o resto vai para a barra de menu',
          "handles the player instructions dialog events: another player switches the row; Cancel puts the kept copy back over the screen's object and Ok only clears the undo, both throw the copy away; the instructions (with undo), the marking type and the marked opponent are set in the screen's object; the rest goes to the menu bar")])
