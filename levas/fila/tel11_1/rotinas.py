# The Setup screens, part 1 (Setup.cpp; reader tel11, the screens round, 21/11/2026): the front end, Setup Game,
# Select League(s) and Select Start Season. Holds the helpers the other tel11 items use (setup_clicked_id,
# setup_colour, setup_screen_header, setup_fatal_exit, setup_open_screen, setup_confirm_message, setup_clicked_value).
# Conventions checked in the disassembly and at the call sites (all cdecl, ret 0):
# - 0x81b890 setup_run_front_end() -> 1, called by 0x5cd0bd; 0x81bab0/0x81e8d0 build (SEH frame, the CRT vector
#   iterators 0x9463af/0x9462bb with 0x4568f0/0x5a53c0 over 25/35 menu items of 0xd4 bytes); 0x81bd70, 0x81dc30,
#   0x81eb00 input(short item) -> int screen result; 0x81ce00, 0x81d010 () -> void.
# - the screen manager calls are thiscall on 0xb74450: open_screen 0x7fe600 (build, input, flag, 0, 0) ret 0x14,
#   open_screen_for_each_record 0x7fe4a0, set_screen_param 0x7ff270 (index, value, copy) ret 0xc, get_screen_param
#   0x7ff010, set_default_screen 0x7feb20 ret 0x1c, start_network_session 0x7ff500 (0, 0) ret 8;
#   Display::add_area 0x55fba0 thiscall(display, x1, y1, x2, y2, ncols, cols, nrows, rows, 2|1, 0, -1) ret 0x2c;
#   add_area_button 0x8040a0 (11 args) ret 0x2c, add_area_text 0x804100 (9 args) ret 0x24: the colours go in registers
#   whose high word is leftover (the callee keeps the word); here zero-extended.
# - cdecl helpers: gui_add_menu_buttons 0x5ed300 (11 args), open_file_dialog 0x58f500 (14), confirm_push 0x591840 (7),
#   0x945bd2 _mkdir, 0x946fd8 sscanf, 0x947120 strncmp, 0x944cff _splitpath, 0x945b13 _makepath.
# Which routines run in the automatic career (auto_setup.cfg, the chain exercises them): setup_run_front_end (once,
# at the start; the whole Setup then runs inside it, on the private stack in the simulation),
# setup_build_setup_game_screen and setup_handle_setup_game_input (the "auto_setup.cfg" branch:
# create_game_from_setup_file("auto_setup.cfg", 0) and -5). The others only run when someone plays: Start New Game
# -> Select League(s) -> (more than one league) Select Start Season.
# DEFECTS of the original kept (candidates for docs/DEFEITOS_DO_ORIGINAL.md):
# - Select League(s), read-only view (game running): the Attribute Masking "No" goes in column 7, the one-pixel
#   spacer, instead of column 8 (the clickable one is in column 8);
# - Select Start Season: the second year of a split season is (year + 1) % 10 printed with %02d, so from 2009/10 on
#   it shows "09/00" ("10/01" for 2010/11...);
# - Select Start Season: the bold mask shifts by an uninitialised local when the main nation is not among the items
#   (the copy uses 0, the first item);
# - Setup Game: the "-load" argument is copied into a 256-byte stack buffer without a limit.
MOTOR = True
TELAS = True
ROTINAS = [('setup_run_front_end', 0x81b890, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('setup_build_setup_game_screen', 0x81bab0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('setup_handle_setup_game_input', 0x81bd70, [0x81, 0xec, 0x08, 0x0f, 0x00, 0x00]),
           ('setup_start_new_game', 0x81ce00, [0xa1, 0xa8, 0x23, 0xae, 0x00]),
           ('setup_build_select_leagues_screen', 0x81d010, [0x83, 0xec, 0x2c, 0x53, 0x56]),
           ('setup_handle_select_leagues_input', 0x81dc30, [0xa1, 0xa8, 0x23, 0xae, 0x00]),
           ('setup_build_select_start_season_screen', 0x81e8d0, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('setup_handle_select_start_season_input', 0x81eb00, [0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00])]
DOCS = ('as telas de criar o jogo (1): a abertura, Setup Game, Select League(s) e Select Start Season',
        'the game creation screens (1): the front end, Setup Game, Select League(s) and Select Start Season',
        [('setup_run_front_end', 'setup_run_front_end', '0x81b890', 'Setup.cpp', 'cdecl',
          'a abertura do jogo: confere a pasta dos inícios rápidos, data 1/1/2001, semente (-seed) e rede (-nonetwork), lê o player_setup.cfg e roda a tela Setup Game até haver um jogo (um jogo feito pela metade é liberado e a tela volta); depois Manager Status vira a tela padrão',
          'the game front end: checks the quick start folder, date 1/1/2001, seed (-seed) and network (-nonetwork), loads player_setup.cfg and runs the Setup Game screen until a game exists (a half-made game is freed and the screen comes back); then Manager Status becomes the default screen'),
         ('setup_build_setup_game_screen', 'setup_build_setup_game_screen', '0x81bab0', 'Setup.cpp', 'cdecl',
          'monta a tela Setup Game: título, os nove botões do menu (Network Play sem id e cinza com a rede desligada) e o botão de voltar; com o parâmetro 0 (as definições vão abrir), uma área vazia cobre a tela',
          'builds the Setup Game screen: title, the nine menu buttons (Network Play with no id and grey when the network is off) and the back button; with param 0 (the settings box is about to open), an empty area covers the screen'),
         ('setup_handle_setup_game_input', 'setup_handle_setup_game_input', '0x81bd70', 'Setup.cpp', 'cdecl',
          'responde à tela Setup Game: as definições na primeira vez, o "-load" da linha de comando, a produção dos inícios rápidos (quick_start.cfg), a carreira automática (auto_setup.cfg), o arquivo escolhido no diálogo (carregar ou apagar) e os nove botões',
          'answers the Setup Game screen: the settings the first time, the command line "-load", the quick start production (quick_start.cfg), the automatic career (auto_setup.cfg), the file chosen in the dialog (load or delete) and the nine buttons'),
         ('setup_start_new_game', 'setup_start_new_game', '0x81ce00', 'Setup.cpp', 'cdecl',
          'começa um jogo novo: sem banco de dados, abaixo de 50 MB livres abre a caixa de memória; senão carrega o banco (CM3_DATA), escolhe a liga padrão pelo idioma e monta as nações-chave; abre Select League(s)',
          'starts a new game: without a database, below 50 MB free it opens the no memory box; otherwise it loads the database (CM3_DATA), picks the default league by language and builds the key nations; opens Select League(s)'),
         ('setup_build_select_leagues_screen', 'setup_build_select_leagues_screen', '0x81d010', 'Setup.cpp', 'cdecl',
          'monta Select League(s): as opções jogadores reais e máscara de atributos, Select All (com menos de 15 ligas e memória para 16) e De-Select All, e a lista das nações-chave com SELECTED, BACKGROUND e a divisão extra; recontagem das ligas escolhidas; com o jogo rodando, só leitura',
          'builds Select League(s): the real players and attribute masking options, Select All (with fewer than 15 leagues and memory for 16) and De-Select All, and the key nation list with SELECTED, BACKGROUND and the extra division; recounts the selected leagues; read-only while the game runs'),
         ('setup_handle_select_leagues_input', 'setup_handle_select_leagues_input', '0x81dc30', 'Setup.cpp', 'cdecl',
          'responde a Select League(s): escolher todas ou nenhuma, as opções, escolher, rebaixar para o fundo e tirar ligas e a divisão extra (com a conferência de memória: 1 liga por 7 MB livres), a nação principal; Next abre Select Start Season (mais de uma liga) ou cria o jogo',
          'answers Select League(s): select all or none, the options, select, move to the background and drop leagues and the extra division (with the memory check: 1 league per 7 MB free), the main nation; Next opens Select Start Season (more than one league) or creates the game'),
         ('setup_build_select_start_season_screen', 'setup_build_select_start_season_screen', '0x81e8d0', 'Setup.cpp', 'cdecl',
          'monta Select Start Season: um botão por liga escolhida com a temporada ("%s %02d/%02d" ou "%s %d"), a da nação principal em negrito; 3 colunas acima de 12 ligas',
          'builds Select Start Season: one button per selected league with its season ("%s %02d/%02d" or "%s %d"), the main nation\'s in bold; 3 columns above 12 leagues'),
         ('setup_handle_select_start_season_input', 'setup_handle_select_start_season_input', '0x81eb00', 'Setup.cpp', 'cdecl',
          'responde a Select Start Season: a liga clicada vira a nação principal; Next cria o jogo e abre Manager Status com um técnico novo',
          'answers Select Start Season: the clicked league becomes the main nation; Next creates the game and opens Manager Status with a new manager')])
