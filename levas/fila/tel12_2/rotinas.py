# The Setup screens, part 6 (Setup.cpp, part 2; reader tel12, the screens round, 21/11/2026): Hall of Fame and the disk
# full and no memory boxes. Uses the Setup helpers of tel11_1 and setup_box_question of tel11_4: goes after them.
# Conventions checked in the disassembly and at the call sites (all cdecl, ret 0):
# - 0x825cc0, 0x825d10 (SEH frame, two local Dates built by 0x54a110), 0x826aa0, 0x826d20 (); 0x826690, 0x826c20,
#   0x826e60 input(short item) -> int; 0x826a60, 0x826ce0 open(int reason).
# - HallOfFame::get_entry 0x5ef130 thiscall(hof, char table, char rank, entry *) ret 0xc (the table goes as a stack
#   dword with leftover above the byte; the callee reads the char); export_to_csv 0x5f0430 ret 4; import_file 0x5efd50
#   ret 8; save 0x5f00c0; hall_of_fame_get_table_name 0x5f07c0 and hall_of_fame_sort_league_order 0x5f10c0 cdecl;
#   ScreenManager::get_area_scroll_row 0x803320 thiscall ret 4; Area::set_scroll_row 0x403320 thiscall ret 4 on the
#   area at display + 4 + 0xbf1 x index; _getdrive 0x9473a7, localtime 0x946c40 cdecl; get_current_screen_flag
#   0x7fed30 thiscall.
# Which routines run in the automatic career: none (the coverage counts 0 for the three blocks; the boxes only open when
# a game cannot be created or loaded).
# DEFECTS of the original kept (candidates for docs/DEFEITOS_DO_ORIGINAL.md):
# - Hall of Fame, Save As Text and Merge In: when the export or the import fails, the dialog's path is freed twice
#   (once before the "Error" box, once after).
MOTOR = True
TELAS = True
ROTINAS = [('open_hall_of_fame_screen', 0x825cc0, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x00]),
           ('hall_of_fame_screen_build', 0x825d10, [0x6a, 0xff, 0x68, 0x96, 0x13, 0x96, 0x00]),
           ('hall_of_fame_screen_handle_input', 0x826690, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('open_disk_full_box', 0x826a60, [0xa1, 0x18, 0x71, 0xdd, 0x00]),
           ('disk_full_box_build', 0x826aa0, [0x51, 0x56, 0x6a, 0x00, 0xb9, 0x50, 0x44, 0xb7, 0x00]),
           ('disk_full_box_handle_input', 0x826c20, [0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00]),
           ('open_no_memory_box', 0x826ce0, [0xa1, 0x18, 0x71, 0xdd, 0x00]),
           ('no_memory_box_build', 0x826d20, [0x56, 0x6a, 0x00, 0xb9, 0x50, 0x44, 0xb7, 0x00]),
           ('no_memory_box_handle_input', 0x826e60, [0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00])]
DOCS = ('as telas de criar o jogo (6): Hall of Fame e as caixas de disco cheio e de falta de memória',
        'the game creation screens (6): Hall of Fame and the disk full and no memory boxes',
        [('open_hall_of_fame_screen', 'open_hall_of_fame_screen', '0x825cc0', 'Setup.cpp', 'cdecl',
          'abre a tela Hall of Fame com os parâmetros 0, 1 e 2 zerados',
          'opens the Hall of Fame screen with params 0, 1 and 2 zeroed'),
         ('hall_of_fame_screen_build', 'hall_of_fame_screen_build', '0x825d10', 'Setup.cpp', 'cdecl',
          'monta Hall of Fame: o menu View com as 28 tabelas (a primeira, Nations 1 e Nations 2), o menu File (Save As Text; Merge In só fora do jogo) e a lista dos 100 melhores (posição, técnico, clube, data, pontos), parando na primeira vaga vazia depois da 20ª linha',
          'builds Hall of Fame: the View menu with the 28 tables (the first, Nations 1 and Nations 2), the File menu (Save As Text; Merge In only outside a game) and the list of the best 100 (rank, manager, club, date, points), stopping at the first empty rank after row 20'),
         ('hall_of_fame_screen_handle_input', 'hall_of_fame_screen_handle_input', '0x826690', 'Setup.cpp', 'cdecl',
          'responde a Hall of Fame: os arquivos escolhidos nos diálogos (exportar a tabela em texto; importar e gravar), a linha da lista, trocar de tabela, abrir os diálogos de gravar e de juntar',
          'answers Hall of Fame: the files chosen in the dialogs (export the table as text; import and save), the list row, switching tables, opening the save and merge dialogs'),
         ('open_disk_full_box', 'open_disk_full_box', '0x826a60', 'Setup.cpp', 'cdecl',
          'abre a caixa de disco cheio com o motivo no parâmetro 0',
          'opens the disk full box with the reason in param 0'),
         ('disk_full_box_build', 'disk_full_box_build', '0x826aa0', 'Setup.cpp', 'cdecl',
          'monta a caixa de disco cheio: uma das quatro mensagens pelo motivo, com a letra da unidade atual, e Ok',
          'builds the disk full box: one of four messages by the reason, with the current drive letter, and Ok'),
         ('disk_full_box_handle_input', 'disk_full_box_handle_input', '0x826c20', 'Setup.cpp', 'cdecl',
          'responde à caixa de disco cheio: Ok fecha (−9 ou −5 pela bandeira da tela), voltar dá 0, o resto vai para a barra de menu',
          'answers the disk full box: Ok closes it (−9 or −5 by the screen flag), back gives 0, the rest goes to the menu bar'),
         ('open_no_memory_box', 'open_no_memory_box', '0x826ce0', 'Setup.cpp', 'cdecl',
          'abre a caixa de falta de memória com o motivo no parâmetro 0',
          'opens the no memory box with the reason in param 0'),
         ('no_memory_box_build', 'no_memory_box_build', '0x826d20', 'Setup.cpp', 'cdecl',
          'monta a caixa de falta de memória: uma das quatro mensagens pelo motivo e Ok',
          'builds the no memory box: one of four messages by the reason and Ok'),
         ('no_memory_box_handle_input', 'no_memory_box_handle_input', '0x826e60', 'Setup.cpp', 'cdecl',
          'responde à caixa de falta de memória: Ok fecha (−9 ou −5), voltar dá 0, o resto vai para a barra de menu',
          'answers the no memory box: Ok closes it (−9 or −5), back gives 0, the rest goes to the menu bar')])
