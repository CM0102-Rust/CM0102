# The Setup screens, part 8 (Setup.cpp, part 3; reader tel13, the screens round, 21/11/2026): the game start date, the
# competition object table, the network game and its two hooks, the region filter, the nationality comparison and the
# game title. Uses setup.rs (setup_warning) and the names of tel11_1 (SETUP_TEXT, G_SETUP_SCREEN_TITLE,
# G_NETWORK_SETUP_HOST_FLAG, FN_SETUP_ADD_HUMAN_MANAGER, setup_format_title, setup_open_screen, setup_confirm_message),
# tel11_2 (G_SCREEN_MANAGER_NETWORK) and tel11_3 (G_SCREEN_MANAGER_SLOT_RECORD_COUNTS): goes after them. Holds the key
# nation entry names (KEY_NATION_RECORD_*) that tel13_2..4 use: goes before them.
# The map's blocks hide functions: 0x830520 (33232 bytes) is setup_set_game_start_date plus the 35 create_*_comps
# (tel13_2..4); 0x8388e0 (896) is setup_create_or_join_network_game plus the hooks 0x838a80 and 0x838c00; 0x839340
# (560) is nation_matches_region_filter plus compare_nations_by_nationality 0x839530.
# Conventions checked in the disassembly and at the call sites (all cdecl, ret 0):
# - 0x830520 () -> 1 or 0 (create_game ignores it); 0x8386f0 () -> 1 or 0; 0x8388e0 (join, name) -> 0 or -4;
#   0x838a80 (short player) and 0x838c00 () hooks (the screen manager's player joined/left callbacks; both end in a
#   tail jump: to setup_add_human_manager () and to match_day_drop_viewer_and_finish_match (), which read no argument);
#   0x839340 (nation, text) -> 1 or 0; 0x839530 (nation **, nation **) -> int (qsort); 0x839570 () -> 0xdd70a8.
# - Date::operator= 0x417c10 thiscall(this, result, src) ret 8; Date::Date 0x549f70 thiscall(out, day, month, year,
#   weekday) ret 0x10; Friendly::register_comp_objects 0x5ca030 thiscall; ScreenManager::send_network_status_message
#   0x802f80 thiscall(sm, text, 0, 0, -1) ret 0x10; display_alloc_guio_snapshot_pool 0x55f1c0 stdcall(slots) ret 4 (the
#   original sets ecx = 0xca72f5, unused); Network::host_or_join_game 0x77a9f0 thiscall(network, join, name) ret 8;
#   open_club_squad_screen 0x453d60 cdecl(5); HUMAN_MANAGER::is_on_holiday 0x5fd210 thiscall(hm, 0) ret 4,
#   set_on_holiday 0x5fd2f0 thiscall(hm, 1, 0) ret 8; strstr 0x945a60; utils_string_compare 0x90c1d0 cdecl(a, b, 1, 0).
# Left out: 0x8387e0 setup_assign_restored_network_managers (block 0x8387e0 of the list): rust/src/simulation.rs
# already hooks its prologue for CM_FERIAS=1 (the save proof's holiday after -load); a second hook on it would make
# the CM_FERIAS hook fail. Porting it means moving the CM_FERIAS step into the Rust routine (a decision for the main
# session).
# Which routines run in the automatic career (auto_setup.cfg): setup_set_game_start_date and build_comp_object_table
# (once each, in create_game; build_comp_object_table also in every save load), nation_matches_region_filter (from
# create_game_from_setup_file 0x8274d1: 53 calls in the world run, docs/cobertura_mundo.txt; in the ENG chain only if
# the auto_setup.cfg names a region). The network routines, the hooks, the comparison and the title only run when
# someone plays (Network Play; the title from 0x5131b6).
# DEFECTS of the original kept: none found (setup_set_game_start_date can only bring a European 2001 start back to 1
# September, never forward).
MOTOR = True
TELAS = True
ROTINAS = [('setup_set_game_start_date', 0x830520, [0x8b, 0x15, 0x08, 0x2d, 0xae, 0x00]),
           ('build_comp_object_table', 0x8386f0, [0xa1, 0x78, 0x23, 0xae, 0x00]),
           ('setup_create_or_join_network_game', 0x8388e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('setup_on_network_player_joined', 0x838a80, [0xa1, 0x98, 0x3c, 0xb6, 0x00]),
           ('setup_on_network_player_left', 0x838c00, [0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00]),
           ('nation_matches_region_filter', 0x839340, [0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('compare_nations_by_nationality', 0x839530, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08]),
           ('get_game_title', 0x839570, [0x68, 0x88, 0x02, 0xa8, 0x00])]
DOCS = ('as telas de criar o jogo (8): a data de início, a tabela das competições, o jogo em rede e os filtros de nação',
        'the game creation screens (8): the start date, the competition table, the network game and the nation filters',
        [('setup_set_game_start_date', 'setup_set_game_start_date', '0x830520', 'Setup.cpp', 'cdecl',
          'a data de início do jogo novo: a START_DATE da entrada da nação principal (para uma nação europeia num início em 2001, depois de 1/set/2001 volta para 1/set); sem nação principal, a da primeira entrada cuja nação tem o bit 1 de LEAGUE_SELECTED; 0 se nenhuma serve',
          'the new game\'s start date: the START_DATE of the main nation\'s entry (for a European nation in a 2001 start, after 1 Sep 2001 it goes back to 1 Sep); without a main nation, the first entry whose nation has bit 1 of LEAGUE_SELECTED; 0 when none fits'),
         ('build_comp_object_table', 'build_comp_object_table', '0x8386f0', 'Setup.cpp', 'cdecl',
          'monta a tabela dos objetos de competição pelo id: uma vaga por registro de competição; cada objeto das entradas das nações-chave na vaga do id do seu registro, depois os amistosos; sem memória, a caixa de erro e 0',
          'builds the competition object table by id: one slot per competition record; every object of the key nation entries at its record\'s id, then the friendlies\'; without memory, the error box and 0'),
         ('setup_create_or_join_network_game', 'setup_create_or_join_network_game', '0x8388e0', 'Setup.cpp', 'cdecl',
          'cria (join 0) ou entra no jogo em rede: a mensagem de estado, a bandeira de anfitrião, o espaço das telas (16 ou 1); sem conseguir pela rede, libera o jogo em memória e abre a caixa "Unable To Create/Join"; 0 se deu certo, -4 se não',
          'creates (join 0) or joins the network game: the status message, the host flag, the screen snapshot pool (16 or 1); when the network fails, frees the game in memory and opens the "Unable To Create/Join" box; 0 when it worked, -4 when not'),
         ('setup_on_network_player_joined', 'setup_on_network_player_joined', '0x838a80', 'Setup.cpp', 'cdecl',
          'gancho de jogador que entrou na rede: com humanos já no jogo ou de férias abre Manager Status; com o jogo rodando abre o elenco e acrescenta um técnico; senão, na vaga de rede com um registro, abre Manager Status (o anfitrião para aí) e acrescenta um técnico',
          'hook of a player who joined the network: with humans already in or on holiday it opens Manager Status; in a running game it opens the squad and adds a manager; otherwise, on a network slot with one record, it opens Manager Status (the host stops there) and adds a manager'),
         ('setup_on_network_player_left', 'setup_on_network_player_left', '0x838c00', 'Setup.cpp', 'cdecl',
          'gancho de jogador que saiu da rede: o técnico da vaga com classificação 0 ou 5 volta a 0, senão sai de férias (se não estava); depois o espectador deixa a partida',
          'hook of a player who left the network: the slot\'s manager with classification 0 or 5 goes back to 0, otherwise goes on holiday (unless already); then the viewer leaves the match'),
         ('nation_matches_region_filter', 'nation_matches_region_filter', '0x839340', 'Setup.cpp', 'cdecl',
          'se a nação passa no filtro de região: ALL; EUROPE (as 14 europeias ocidentais); EASTERN_EUROPE (Rússia, Croácia, Polônia); SCANDINAVIA (Suécia, Noruega, Finlândia, Dinamarca); AMERICAS (Brasil, Argentina, EUA); ASIA (Austrália, Japão)',
          'whether the nation passes the region filter: ALL; EUROPE (the 14 western European ones); EASTERN_EUROPE (Russia, Croatia, Poland); SCANDINAVIA (Sweden, Norway, Finland, Denmark); AMERICAS (Brazil, Argentina, USA); ASIA (Australia, Japan)'),
         ('compare_nations_by_nationality', 'compare_nations_by_nationality', '0x839530', 'Setup.cpp', 'cdecl',
          'comparação do qsort de duas nações pela nacionalidade (nula vai para o fim)',
          'the qsort comparison of two nations by nationality (a null one goes last)'),
         ('get_game_title', 'get_game_title', '0x839570', 'Setup.cpp', 'cdecl',
          'o título do jogo ("Championship Manager 2001/02" traduzido) no buffer do título, que é devolvido',
          'the game title ("Championship Manager 2001/02" translated) into the title buffer, which is returned')])
