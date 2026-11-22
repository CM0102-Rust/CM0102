# The club screen's input handler (club_screens.cpp, part 4; reader tel03b, the screens round, 21/11/2026).
# - 0x4671f0 club_screen_input cdecl(short widget) ret 0, SEH, 0x1130 bytes of locals (two TEAM blocks of 0x625 bytes
#   built by the CRT vector iterators with 0x453c30/0x453c10 and never used, 11 bytes of formation order, the text
#   buffers); from 0x4671f0 to 0x4689e3 plus its five jump tables to 0x468b10 (tab switch 0x4689e4; the squad view
#   0x4689f8/0x468a04, the squad columns 0x468a44/0x468a50, the transfers view 0x468a70/0x468a78, the info view
#   0x468af8/0x468b00); it holds the map's 0x467753 and 0x46835e entries (0x46835e is mid-routine) and the 0x4671f0
#   part of 0x4671e0. Ported whole, in named pieces per action group: club_screen_printout_answer (dialog code 0x38),
#   club_screen_common_actions (squad links, hall of fame, Print, tab change, training, Tactics, Match Stats, history),
#   club_screen_squad_tab_input (view/column flags, sort/filter, squad numbers, line-up drags, transfer status),
#   club_screen_transfers_tab_input, club_screen_info_tab_input, and the next match/fixtures tabs (0x468c10/0x4691d0,
#   item tel03_4) then the menu bar chain (0x482d10 handle_club_screen_action, 0x7615d0, 0x7643d0).
#   Calls (checked at the call sites): 0x7d4460 printout writer cdecl(club, reserves, kind, file, option) (printouts.cpp,
#   no map name), 0x591840 confirm_push cdecl(7), 0x58f500 open_file_dialog cdecl(14), 0x5fc5b0
#   HUMAN_MANAGER::get_folder_path thiscall(staff, kind) ret 8, 0x453d60 open_club_squad_screen cdecl(5), 0x5f11c0
#   history_screen_open, 0x545880 club_get_english_name, 0x8b5930 training_screens_open, 0x600c90
#   human_manager_current_controls_club, 0x899a50 tactics_screen_open cdecl(6), 0x729500
#   Match_Stats::fixture_entry_matches thiscall ret 0x14, 0x71b4a0 open_match_details, 0x53b900
#   database_is_national_team, 0x469e70 open_club_history_screen, 0x803320 get_area_scroll_row, 0x468b10/0x468b40/
#   0x468b90/0x468bd0 the widget readers (tel03_1), 0x896fe0/0x8968f0/0x896e90/0x8973e0/0x896c60/0x896a90 Tactics
#   (thiscall on [0xde1d08]), 0x601560/0x601070/0x6016e0/0x6013c0/0x601200 HUMAN_MANAGER squad numbers (thiscall on
#   [0xb63c98]), 0x5fbd00 get_active_tactic, 0x5b31b0 Formation::get_slots_in_position_order thiscall ret 8, 0x8f6770
#   transfer_screens_open_transfer_status_screen cdecl(person, 0, -1, 0).
# Depends on: tel03_2 (club_history_action, the 0x468b40 reader) and tel03_4 (club_tabs_action, club_tabs_value,
#   club_tabs_error_box, CLUB_TABS_TEXT, CLUB_TABS_DIALOG_ANSWERED).
# Kept from the original: the two unused TEAM blocks; the squad link with a value between -2000 and 2000 other than 0
#   answers 0 and does nothing. The place and number bytes passed to Tactics/HUMAN_MANAGER go in dword slots whose high
#   bytes are stack or register leftovers in the original (0 here: the callees read the byte); the date words passed to
#   fixture_entry_matches likewise (0 here: compared as words).
# Comparer: side by side (compare_resposta) only for the clicks that just write params (tab change; squad view,
#   columns, sort, filter; transfers view and page; info view); everything else original only, the answer checked
#   against the plan where it does not depend on the game (links, Print, Tactics, Match Stats, squad tab 0x25 and 0xf).
# How to reach it: any club's screen (a club name link, or the human's club from the menu): the tabs Squad,
#   Transfers, Next Match, Fixtures, Info; Squad, Tactics, Training, History, Hall of Fame, Match Stats, Print (with and
#   without the printer option); on the Squad tab the View and column choices, sorting, dragging players in the line-up
#   and onto squad numbers, Set Transfer Status; the reserves/first team link; the menu bar.
TELAS = True
ROTINAS = [('club_screen_input', 0x4671f0, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a resposta da tela do clube', 'the club screen\'s input',
        [('club_screen_input', 'club_screen_input', '0x4671f0', 'club_screens.cpp', 'cdecl, SEH',
          'Responde ao clique na tela do clube: primeiro a resposta da caixa Salvar Impressão (código 0x38: grava a impressão no arquivo escolhido, ou a caixa "Unable To Save"); depois os botões de todas as abas: elenco de outro clube ou o outro time do clube (reservas e principal), hall da fama, imprimir (direto na impressora com a opção, senão a caixa do nome do arquivo), trocar de aba (parâmetro 4), treino, táticas e estatísticas do último jogo (ou a caixa de que não há), histórico; depois a aba aberta: elenco (visão e colunas por grupos de bits exclusivos, ordem e filtro, números da camisa, arrastar jogadores nas vagas da tática e nos números, status de transferência), transferências (visão e página), próximo jogo e jogos (as rotinas deles), informações (visão); aba desconhecida mostra a caixa de erro; por fim as ações da tela do clube, a barra de menus e os links.',
          'Answers a click on the club screen: first the Save Printout box answer (code 0x38: writes the printout to the chosen file, or the "Unable To Save" box); then the buttons of every tab: another club\'s squad or the club\'s other side (reserves and first team), hall of fame, Print (straight to the printer with the option, else the file name box), tab change (param 4), training, the last match\'s tactics and stats (or the box saying there is none), history; then the open tab: squad (view and columns by exclusive bit groups, sort and filter, squad numbers, dragging players into the line-up places and onto numbers, transfer status), transfers (view and page), next match and fixtures (their routines), info (view); an unknown tab shows the error box; last the club screen actions, the menu bar and the links.')])
