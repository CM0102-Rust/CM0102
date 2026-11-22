# The tactics screen (tactics_screens.cpp, 2; reader tel14, the screens round, 21/11/2026): building it.
# tactics_screen_build (0x899ea0..0x89f6f0, 22.6 KB: the whole routine hooked, written in named pieces: the menus,
# the header and column widths, a list row and its view column, the overview with the run arrows and the zone views).
# Conventions checked in the disassembly: cdecl, ret 0 (the reserve thunk 0x89f700 jumps here); Display::add_item
# thiscall ret 0x48, add_area ret 0x2c, add_area_button ret 0x2c, add_area_text ret 0x24, add_text_item ret 0x2c,
# add_button_item ret 0x30 (tel26_1's wrappers); the Tactics_Screen and Formation methods thiscall (ret = 4 x the
# arguments); draw_pitch cdecl with an 11th argument (the float 4.0); get_pitch_point_for_position_flags and
# mask64_to_bit_index cdecl.
# The cm0102-1080.exe changes three bytes of it: the font of the current view tab (push 1 -> push 3 at 0x89c625,
# 0x89c69d and 0x89c73b): the copy reads them from the running exe.
# The arrowheads of the runs are computed on the x87 (inline asm: fpatan, fsin, fcos and the game's _ftol), the
# same instructions as the original.
# Unreachable asserts left out (the row, slot and penalty order indices are always in range: error boxes of lines
# 0x1b8e, 0x179f and 0x16b9).
# Needs tel14_1 (tactics_screen::*, tactics_screen_construct) and tel26_1 (the display wrappers, job colours,
# tel26_copy_text, tel26_slot_manager, tel26_back_next).
# Defects of the original: "Load Preset" appears twice in the Tactics menu (after Load and after Save As) when the
# presets are on.
TELAS = True
ROTINAS = [('tactics_screen_build', 0x899ea0, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8])]
DOCS = ('a tela de tática (2): a montagem',
        'the tactics screen (2): building it',
        [('tactics_screen_build', 'tactics_screen_build', '0x899ea0', 'tactics_screens.cpp', 'cdecl',
          'monta a tela de tática: as cores fixas na primeira vez; o objeto da tela (construído se falta); o fundo (o estádio do registro da partida, senão a competição, ou o clube); o título e a barra de menu. Só de leitura, só o botão Instruções do Time; editável, os menus Tática (instruções, cobradores de pênalti, padrão com as táticas prontas, carregar, salvar, salvar como, salvar para outro técnico humano, apagar, travar ou destravar e os quatro arquivos recentes), Ver (as colunas: posições, condição, gols, forma, média, nota da partida e as instruções de jogador, marcação e marcador), Última Partida e Editar (desfazer, copiar, colar, espelhar, mostrar ou esconder as corridas). Depois o nome da tática (com a marca de não salva), TRAVADA, as abas Geral / Com Bola / Sem Bola e as larguras das colunas conforme a coluna escolhida; a lista dos 20 jogadores (número clicável em campo ou no banco com substituições sobrando, os cartões, o pênalti ou a braçadeira de capitão, a coluna escolhida e o nome); o campo; na visão geral as setas das corridas (do lugar de defesa ao de ataque) e os círculos com número e nome; nas visões com e sem bola as 12 zonas, as corridas até a zona mostrada, a zona do mouse e a mostrada e os círculos; e Cancelar / Ok (editável) ou Voltar e Avançar',
          'builds the tactics screen: the fixed colours the first time; the screen object (built when missing); the background (the stadium of the match record, else the competition, or the club); the title and the menu bar. Read-only, only the Team Instructions button; editable, the Tactics menu (team instructions, penalty takers, set to default with the preset formations, load, save, save as, save for another human manager, delete, lock or unlock and the four recent files), View (the columns: positions, condition, goals, form, average and match rating and the player instructions, marking and marker), Last Match and Edit (undo, copy, paste, reflect, show or hide the runs). Then the formation name (with the unsaved mark), LOCKED, the Overview / With Ball / Without Ball tabs and the column widths for the chosen column; the list of the 20 players (number clickable on the pitch or on the bench with substitutions left, the cards, the penalty order or the captain armband, the chosen column and the name); the pitch; in the overview the run arrows (from the defending to the attacking position) and the circles with number and name; in the with and without the ball views the 12 zones, the runs to the zone shown, the zone under the mouse and the zone shown and the circles; and Cancel / Ok (editable) or Back and Next')])
