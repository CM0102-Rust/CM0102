# The Setup screens, part 2 (Setup.cpp; reader tel11, the screens round, 21/11/2026): Network Play, Enter Network
# Game Name, Select Network Game, Manager Status and adding a human manager. Uses the helpers of tel11_1 (the two go
# together: this item does not compile without tel11_1).
# Conventions checked in the disassembly and at the call sites (all cdecl, ret 0): the builders () -> void, the input
# handlers (short item) -> int screen result; setup_open_manager_status_screen and setup_add_human_manager () -> void,
# called by setup_handle_setup_game_input, the select leagues/season handlers and the network name handler (also by
# 0x5cd28f, 0x5ce029, 0x76409b and 0x764025 outside Setup.cpp).
# - thiscall on the screen manager 0xb74450: start_network_session 0x7ff500 (join 0x838a80, leave 0x838c00) ret 8,
#   get_local_addresses 0x7fc7c0 (out) ret 4 -> count of 20-byte texts, is_player_done 0x7fff20 (short) ret 4,
#   count_idle_channels 0x7fff70 -> ax, run_network_client 0x7fdc00, disconnect_network 0x7fc690,
#   find_manager_channel 0x7fffa0 -> ax, is_manager_done 0x7ffe40 (key) ret 4; add_button_item 0x804030 (12 args)
#   ret 0x30; add_text_item 0x803fd0 (11 args) ret 0x2c; Display::add_item 0x55f990 (18 args) ret 0x48.
# - TcpIp::send_datagram 0x8aec90 thiscall(net, data, length, port 0x1a0b, 0, 0, 0) ret 0x18 on [0xb77c90];
#   HUMAN_MANAGER::get_password 0x5fde40 / get_password_entry 0x5fdf40 thiscall(HM, staff) ret 4 -> char *;
#   setup_create_or_join_network_game 0x8388e0 cdecl(join, name) -> 0 on success; gui_add_text_area 0x5ed070 cdecl
#   variadic (10 args and the 0 that ends the links).
# Which routines run in the automatic career: none of these for sure (the coverage counts 0 for 0x81fea0 and
# 0x820d40); Manager Status becomes the default screen at the end of the front end, so its builder/handler
# (0x81ff10/0x820950) might be built by the screen manager during the holiday career: not proven either way (the
# coverage files only count the map's block starts).
# How to reach them in the game: Manager Status after Start New Game -> (leagues) -> Next (with Add Manager: Enter
# Name, item tel11_3); Network Play from the Setup Game menu (Start/Quick Start/Restore -> Enter Network Game Name; Join
# -> Select Network Game, with Refresh).
# DEFECTS of the original kept (candidates for docs/DEFEITOS_DO_ORIGINAL.md):
# - Select Network Game without a network object (the session did not start): the count of games is the word of the
#   two column weights (0x301 = 769), so 769 buttons read names past the 500-byte list copy;
# - setup_add_human_manager with no free record among the last 16 staff (or without staff) writes through a null
#   person (the original crashes there; so does the copy);
# - Select Network Game: a click on a game with no item (-1) copies from a null text.
MOTOR = True
TELAS = True
ROTINAS = [('setup_build_network_play_screen', 0x81edd0, [0x6a, 0xff, 0x68, 0x49, 0x13, 0x96, 0x00]),
           ('setup_handle_network_play_input', 0x81ef70, [0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00]),
           ('setup_build_enter_network_game_name_screen', 0x81f1a0, [0xa1, 0x90, 0x7c, 0xb7, 0x00]),
           ('setup_handle_enter_network_game_name_input', 0x81f2c0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('setup_build_select_network_game_screen', 0x81f8d0, [0x51, 0xa1, 0x90, 0x7c, 0xb7, 0x00]),
           ('setup_handle_select_network_game_input', 0x81fbd0, [0x55, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('setup_open_manager_status_screen', 0x81fea0, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x00]),
           ('setup_build_manager_status_screen', 0x81ff10, [0x81, 0xec, 0xf4, 0x05, 0x00, 0x00]),
           ('setup_handle_manager_status_input', 0x820950, [0x8b, 0x0d, 0xbc, 0x23, 0xae, 0x00]),
           ('setup_add_human_manager', 0x820d40, [0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00])]
DOCS = ('as telas de criar o jogo (2): o jogo em rede, Manager Status e o técnico novo',
        'the game creation screens (2): network play, Manager Status and the new manager',
        [('setup_build_network_play_screen', 'setup_build_network_play_screen', '0x81edd0', 'Setup.cpp', 'cdecl',
          'monta Network Play: os quatro botões (começar, início rápido, restaurar e entrar num jogo em rede) e o de voltar',
          'builds Network Play: the four buttons (start, quick start, restore and join a network game) and the back one'),
         ('setup_handle_network_play_input', 'setup_handle_network_play_input', '0x81ef70', 'Setup.cpp', 'cdecl',
          'responde a Network Play: começar, início rápido e restaurar abrem a tela do nome do jogo em rede (parâmetros 0/0, 1/1, 1/0); entrar abre a lista dos jogos; o nome é apagado',
          'answers Network Play: start, quick start and restore open the network game name screen (params 0/0, 1/1, 1/0); join opens the game list; the name is emptied'),
         ('setup_build_enter_network_game_name_screen', 'setup_build_enter_network_game_name_screen', '0x81f1a0', 'Setup.cpp', 'cdecl',
          'monta Enter Network Game Name: abre a sessão de rede se não houver, o campo do nome (50 letras) e os botões; Next só com nome',
          'builds Enter Network Game Name: starts the network session if there is none, the name entry (50 characters) and the buttons; Next only with a name'),
         ('setup_handle_enter_network_game_name_input', 'setup_handle_enter_network_game_name_input', '0x81f2c0', 'Setup.cpp', 'cdecl',
          'responde a Enter Network Game Name: o arquivo escolhido no diálogo é carregado e o jogo em rede criado (Manager Status, com um técnico novo se não houver); Next começa o jogo novo ou abre o diálogo do início rápido ou do jogo salvo; voltar encerra a sessão',
          'answers Enter Network Game Name: the file chosen in the dialog is loaded and the network game created (Manager Status, with a new manager if there is none); Next starts the new game or opens the quick start or saved game dialog; back ends the session'),
         ('setup_build_select_network_game_screen', 'setup_build_select_network_game_screen', '0x81f8d0', 'Setup.cpp', 'cdecl',
          'monta Select Network Game: abre a sessão e pede a lista dos jogos ("CM3 LIST SERVER REQ", porta 0x1a0b), Refresh, o nome a entrar e um botão por jogo encontrado',
          'builds Select Network Game: starts the session and asks for the game list ("CM3 LIST SERVER REQ", port 0x1a0b), Refresh, the name to join and one button per game found'),
         ('setup_handle_select_network_game_input', 'setup_handle_select_network_game_input', '0x81fbd0', 'Setup.cpp', 'cdecl',
          'responde a Select Network Game: Refresh pede a lista de novo; a tela se refaz quando o número de jogos muda; o jogo clicado vira o nome; Next entra no jogo e roda o cliente de rede; voltar encerra a sessão',
          'answers Select Network Game: Refresh asks for the list again; the screen is rebuilt when the game count changes; the clicked game becomes the name; Next joins the game and runs the network client; back ends the session'),
         ('setup_open_manager_status_screen', 'setup_open_manager_status_screen', '0x81fea0', 'Setup.cpp', 'cdecl',
          'abre Manager Status com os cinco parâmetros zerados',
          'opens Manager Status with its five params zeroed'),
         ('setup_build_manager_status_screen', 'setup_build_manager_status_screen', '0x81ff10', 'Setup.cpp', 'cdecl',
          'monta Manager Status: os endereços do servidor, um técnico humano por linha com o cargo e a situação (de férias, sem jogar, jogando, esperando), Finish (com senha de quem está de férias não) e os parâmetros que a resposta vigia',
          'builds Manager Status: the server addresses, one human manager per row with his job and state (on holiday, not playing, playing, waiting), Finish (not with a password of one on holiday) and the params the answer watches'),
         ('setup_handle_manager_status_input', 'setup_handle_manager_status_input', '0x820950', 'Setup.cpp', 'cdecl',
          'responde a Manager Status: refaz a tela quando algo mudou; a linha de um técnico abre a senha ou a confirmação de retomar o controle; Finish (antes do jogo) devolve -5',
          'answers Manager Status: rebuilds the screen when something changed; a manager\'s row opens the password or the resume control confirmation; Finish (before the game) returns -5'),
         ('setup_add_human_manager', 'setup_add_human_manager', '0x820d40', 'Setup.cpp', 'cdecl',
          'acrescenta um técnico humano: o primeiro registro livre entre os 16 últimos do staff, nascido em 1/1/1966, 35 anos, a nação do idioma; abre Enter Name',
          'adds a human manager: the first free record among the last 16 staff, born 1/1/1966, aged 35, the nation of the language; opens Enter Name')])
