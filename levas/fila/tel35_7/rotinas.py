# The menu bar's commands and periodic work (Menubar.cpp; reader tel35b, the screens round, 22/11/2026). Uses the
# helpers of tel26_1 (tel26_item_id, tel26_item_value), tel26_6 (menubar_record, menubar_slot, MENUBAR_*, the Menubar*
# call types, menubar_being_created, menubar_count_humans, menubar_no_flagged_screens, menubar_text), tel35_1
# (TEL35_NETWORK_FLAG = 0xb77c84, the "a box answered" flag) and tel35_2 (tel35_cdecl1): it enters after them.
# - 0x7615d0 menubar_handle_command (11771 bytes, 0x7615d0..0x7643cb): cdecl(short item), no SEH, no jump table (a
#   chain of compares). Split in named pieces (periodic: auto save, Continue button, flashing header, status item; the
#   idle continuation; the box answers; the commands by menu), all inside the one hooked routine.
# Conventions checked in the disassembly (the Ghidra pseudo-C hides most arguments):
#   langlib_sprintf returns its buffer, and the original chains the calls: the arguments pushed for the first call stay
#   on the stack for the next ones and for the final call, so the heading and the prompt are formatted with all of them
#   and confirm_push 0x591840 gets (title, heading, message, buttons, fore, back, code) (7, cdecl) and open_file_dialog
#   0x58f500 (title, prompt, 1, folder, ".sav", name, check 0x838e10, fore, back, code, 0, 0, 0, last) (14, cdecl;
#   last 1 only for "Save Game"); get_heading_colour 0x839590 cdecl(flag) AX (high half from the 0/1 argument: 0);
#   get_game_title 0x839570 ignores its argument; open_choose_friendly_teams_screen 0x5a0650 cdecl(kind, a, b, 0): kinds
#   1, 2, 3, 6 (data, key_nation_get_main_nation(0)), kinds 4, 5 (key_nation_get_main_continent(data, 0), data) — the
#   pseudo-C shows only two arguments; database_get_club_kit_colours writes the two low words only; _splitpath
#   0x944cff (path, 0, 0, Config +0x113, 0) (its fifth argument is a push made before the branch); Tactics::
#   set_selected_player reads the slot as a byte (the original pushes a whole dirty local); News::has_urgent_items
#   0x787350 thiscall(news, manager, 1) ret 8; HUMAN_MANAGER (on [0xb63c98]) get_holiday_stamp 0x5fdc30 (1),
#   set_controls_reserve_team 0x5fd590 (2), controls_all_national_teams 0x5fd6b0 (2, eax tested whole),
#   set_controls_all_national_teams 0x5fd810 (2); ScreenManager count_all_records 0x8033c0 and count_active_records
#   0x7feb70 (AX signed), set_manager_done 0x7ffeb0 (2), switch_to_manager 0x7ffcd0 (1), add_manager_record 0x7ff8f0
#   (3), send_clear_click 0x800000; SCOUT_MANAGER::get_compare_player 0x7f7920 thiscall([0xb74448], k);
#   open_resign_confirm_dialog 0x69b780 thiscall([0xb67a44], job); the openers by address (cdecl): 0x6ae1a0,
#   0x6b0a10, 0x6b28b0, 0x6b2080 (1), 0x870650 (5), 0x88c440 (1), 0x808540 (-1), 0x8f4fa0, 0x8702a0 (5), 0x6b2d60,
#   0x7897f0 (2), 0x49c0b0, 0x49c700, 0x5a2c30 (1), 0x5f2b80 (1), 0x820d40, 0x81fea0, 0x81ce00, 0x825cc0, 0x4e5920,
#   0x5134c0, 0x826a60 / 0x826ce0 (1), restart_game 0x68dbc0, match_screens_handle_continue_command 0x725ca0 (1).
# - The current command id [0xdd6bde] is read once after the arrows and again after the manager switch (0x3fd, the slot
#   changes) and after "Save Game" (0x3fe): the copy reads it at the same points.
# - Equal in cm0102-1080.exe (no byte of 0x7615d0..0x7643cb differs).
# Defects of the original (kept): after a failed save from the save dialog with the disk full or the no memory box,
# the typed path is never freed (a leak); 0x3fd with no menu bar record writes 0 at address 0xee (a crash; the record
# exists for every human manager); a failed auto save sets the last save date to today (the next try waits the
# whole interval; by design: no retry on every frame).
# Comparer: only the original runs (the commands open screens, save, restart or leave the game); the periodic work and
# the idle continuation run dry before it, and with no command the record and the answer are checked afterwards; a
# command with a fixed answer checks the answer. Not compared in a network game nor with a box answer to dispatch.
MOTOR = True
TELAS = True
ROTINAS = [('menubar_handle_command', 0x7615d0, [0x81, 0xec, 0xa4, 0x02, 0x00, 0x00])]
DOCS = ('a barra de menus (4): os comandos e o trabalho periódico',
        'the menu bar (4): the commands and the periodic work',
        [('menubar_handle_command', 'menubar_handle_command', '0x7615d0', 'Menubar.cpp', 'cdecl',
          'A resposta da barra de menus a um controle (item -1: nenhum), a cada quadro de toda tela: 0x424 dá -11 logo. O trabalho periódico (com o registro da barra e sem resposta de caixa a despachar): no menu do time, o salvamento automático (opção 0x2000 da configuração, vaga local, sem técnico sendo criado nem tela marcada: sem arquivo ainda pede o nome, "Auto Save Game"; passado o intervalo de dias desde o último, salva na pasta e no nome da configuração; a falha abre a caixa de disco cheio, de memória ou "não pôde ser salvo"), o botão Continue ("Latest Scores" durante a partida do humano no dia de jogos; "Waiting For Players" com o técnico pronto ou "Continue Game"; cinza quando não se pode continuar; ao sair de "Latest Scores" sem estar pronto, -11) e o cabeçalho que pisca a cada segundo ("Updating <n>%..." no dia de jogos, a partida pausada ou a data); em todo menu, o aviso de baixo (notícias, mensagem de chat, jogadores esperando ou vazio). Sem controle: a contagem regressiva das férias (5 s), o técnico de férias 2 s depois do carimbo e o modo interativo sem técnico 5 s depois da montagem seguem o jogo (-5), salvo pouco espaço em disco ou memória (a caixa e o fim das férias). Depois, o Continue da partida, as respostas das caixas (férias, sair do jogo em rede, voltar das férias com a escalação salva, os salvamentos, o reserva e as seleções, reiniciar, sair, com o diálogo "salvar antes" quando o jogo salvo não está em dia) e os comandos dos menus: voltar e avançar, Continue, últimos resultados, notícias, estatísticas de técnicos, informação de emprego, controle do reserva e das seleções, demitir-se, aposentar, confiança da diretoria, comparar jogadores, buscas, transferências, perfil, férias, mensagem, ranking da FIFA, coeficientes da UEFA, as telas de busca, o histórico escrito, trocar de técnico, salvar, adicionar técnico, situação, chat, novo jogo, galeria da fama, créditos, sites, reiniciar e sair. -4 quando abre tela ou caixa; 0 quando não trata',
          'The menu bar\'s answer to a widget (item -1: none), on every frame of every screen: 0x424 gives -11 at once. The periodic work (with the bar\'s record and no box answer to dispatch): in the team menu, the auto save (Config option 0x2000, a local slot, no manager being created nor flagged screen: with no file yet it asks the name, "Auto Save Game"; once the interval of days since the last one has passed it saves into the Config\'s folder and name; a failure opens the disk full, no memory or "could not be saved" box), the Continue button ("Latest Scores" during the human\'s match on a match day; "Waiting For Players" when the manager is done or "Continue Game"; greyed when the game cannot go on; leaving "Latest Scores" while not done, -11) and the header flashing every second ("Updating <n>%..." on a match day, the paused match or the date); in every menu, the bottom status item (news, chat message, players waiting or empty). With no widget: the holiday countdown (5 s), the manager on holiday 2 s after his stamp and the interactive mode with no manager 5 s after the build go on (-5), unless the disk or the memory is short (the box and the end of the holiday). Then the match Continue, the boxes\' answers (holiday, leaving the network game, coming back from holiday with the saved line-up, the saves, the reserve and national teams, restart, exit, with the "save first" dialog when the saved game is not up to date) and the menu commands: back and forward, Continue, latest scores, news, manager stats, job information, the reserve and national teams control, resign, retire, board confidence, compare players, searches, transfers, profile, holiday, message, FIFA rankings, UEFA coefficients, the find screens, the written history, change manager, save, add manager, manager status, chat, new game, hall of fame, credits, web sites, restart and exit. -4 when a screen or a box opens; 0 when not handled')])
