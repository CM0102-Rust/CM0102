# As pastas, os arquivos recentes e as opcoes do tecnico humano (human_manager.cpp; leitor sx12b, rodada 5, 23/11/2026).
# DEPENDE do sx12_4 (human_record_or_error, human_record_found): integrar junto ou depois dele.
# Quem chama: get_folder_path pelos dialogos de arquivo (financas, pesquisa, taticas, treino, impressao: 37 chamadas);
# get_recent_file pela pesquisa de jogadores, taticas e treino; add_recent_file ao carregar/gravar formacao, pesquisa e
# agenda de treino; set_controls_reserve_team pela tela de aposentadoria, pela barra de menus e pela carga (0x82a9c0);
# set_controls_all_national_teams pela barra de menus; set_computer_controlled pela aposentadoria, pela confirmacao de
# "Assumir o controle", pela carga e pela rede; set_watched_fixture pelo link de partida da barra de menus;
# human_manager_set_holiday_option pela tela de ferias.
# Caminho no jogo para o Lucas: Taticas -> Carregar/Salvar (as pastas e os recentes); Pesquisa de jogadores ->
# carregar/salvar pesquisa; Treino -> carregar/salvar agenda; barra de menus -> Opcoes do tecnico (Control Reserve Team,
# selecoes); Ferias (opcao "manter o time" = 3); um link de partida -> "assistir".
# Convencoes conferidas na desmontagem (todas thiscall; o tecnico nulo e o humano atual [0xb74c26 + 0x300 x
# [0xb77c7a]]; indice = id - [0xae2364] + 16 contra [this+4] - 1):
# - 0x5fc5b0 (gerente, tipo) ret 8 -> eax; caixas 0x249/0x255. Tipo 1: slot ativo +0x18f1; 3: +0xa0bc; 2: +0x9ba8;
#   outro: +0xa5d0.
# - 0x5fc760 (indice, saida, tamanho, tirar_pasta, gerente, tipo) ret 0x18 -> eax; caixas 0x286 (saida nula ou tamanho
#   < 0x104), 0x291, 0x29c, 0x2a5 (tipo 4), 0x2b0; _splitpath 0x944cff, _makepath 0x945b13 (0xad9c64 = ""), stricmp
#   0x951c50; a pasta do tipo para comparar (1: slot +0x18f1; 3: +0xa0bc; outro: +0x9ba8, diferente do get_folder_path).
# - 0x5fcb60 (caminho, gerente, tipo) ret 0xc -> eax 1/0; caixas 0x328, 0x333, 0x33c, 0x347; strncpy 0x9466d0 de 0x104
#   e o byte 0x103 zerado.
# - 0x5fd590 / 0x5fd810 (valor, gerente) ret 8; byte +0x9a05 / +0x9a06; MATCH_DAY::refresh_human_interest_flags
#   0x6b5040 thiscall([0xb67a50]) se houver dia de jogo; caixas 0x488/0x494 e 0x4e4/0x4f0.
# - 0x5fe140 (marca, gerente) ret 8; a contagem em +8; caixas 0x642/0x64e.
# - 0x5fe500 (partida, gerente) ret 8 -> eax; sem tecnico: 0 sem caixa; 0x6d9; can_watch_fixture 0x5fe700 thiscall(this,
#   partida, gerente) ret 8; o dia da partida (+0x2a, sem sinal) contra [0xae2c90] (com sinal), +0x28 contra
#   [0xae2c92], +0x3f contra [0xae2c88], [0xb67a50] e [0xb67a48] == 0: recusa; copia ou zera 0x4f bytes.
# - 0x5feb60 (opcao, valor, gerente) ret 0xc; caixas 0x7bc/0x7c8; Tactics::get_club_tactic 0x896660 thiscall([0xde1d08],
#   clube, 0), get_selected_player 0x8968f0 thiscall([0xde1d08], clube, k, 0), Formation::get_slot_position 0x5b2ae0
#   thiscall(tatica, k, 0) -> ax com sinal (o original empilha k como dword com residuo; as rotinas leem o byte).
# Comparadores lado a lado (o caminho escrito, o registro de 0xa6ef bytes, as contagens +8/+0xc); a atualizacao do dia
# de jogo roda nas duas (so refaz as marcas a partir dos registros); caixa: so a original.
# Defeito do original: get_recent_file com a pasta vazia testa o byte antes do buffer como a barra final.
TELAS = True
SAVE = False
ROTINAS = [('get_folder_path', 0x5fc5b0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('get_recent_file', 0x5fc760, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x0c, 0x07, 0x00, 0x00]),
           ('add_recent_file', 0x5fcb60, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('set_controls_reserve_team', 0x5fd590, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('set_controls_all_national_teams', 0x5fd810, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('set_computer_controlled', 0x5fe140, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('set_watched_fixture', 0x5fe500, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('human_manager_set_holiday_option', 0x5feb60, [0x8b, 0x54, 0x24, 0x0c, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('as pastas, os arquivos recentes e as opções do técnico humano',
        'the human manager\'s folders, recent files and options',
        [('get_folder_path', 'HUMAN_MANAGER::get_folder_path', '0x5fc5b0', 'human_manager.cpp', 'thiscall, ret 8',
          'devolve a pasta guardada do técnico para um tipo de arquivo: táticas (a do bloco de tática em uso), pesquisa, treino ou, qualquer outro, impressão',
          'returns the manager\'s stored folder for a kind of file: tactics (the one of the tactic block in use), search, training or, any other, print'),
         ('get_recent_file', 'HUMAN_MANAGER::get_recent_file', '0x5fc760', 'human_manager.cpp', 'thiscall, ret 0x18',
          'copia o arquivo recente 0 a 3 de um tipo para a saída; com a opção, se o arquivo está na pasta do próprio tipo, fica só o nome e a extensão',
          'copies recent file 0 to 3 of a kind into the output; with the option, a file in the kind\'s own folder keeps only its name and extension'),
         ('add_recent_file', 'HUMAN_MANAGER::add_recent_file', '0x5fcb60', 'human_manager.cpp', 'thiscall, ret 0xc',
          'põe um caminho no topo dos quatro arquivos recentes de um tipo: se já estava na lista sobe de lá, senão o último sai; os de cima descem uma posição',
          'puts a path at the top of the four recent files of a kind: when already in the list it moves up from there, otherwise the last one drops out; those above move down one place'),
         ('set_controls_reserve_team', 'HUMAN_MANAGER::set_controls_reserve_team', '0x5fd590', 'human_manager.cpp', 'thiscall, ret 8',
          'liga ou desliga a opção de controlar o time reserva e, com um dia de jogo em curso, atualiza as marcas de interesse dos humanos',
          'turns the Control Reserve Team option on or off and, with a match day running, refreshes the human interest flags'),
         ('set_controls_all_national_teams', 'HUMAN_MANAGER::set_controls_all_national_teams', '0x5fd810', 'human_manager.cpp', 'thiscall, ret 8',
          'liga ou desliga a opção de controlar todas as seleções da nação e, com um dia de jogo em curso, atualiza as marcas',
          'turns the control all national teams option on or off and, with a match day running, refreshes the flags'),
         ('set_computer_controlled', 'HUMAN_MANAGER::set_computer_controlled', '0x5fe140', 'human_manager.cpp', 'thiscall, ret 8',
          'liga ou desliga o controle pelo computador do técnico, mantendo a contagem dos controlados, e atualiza as marcas do dia de jogo',
          'turns the manager\'s computer control on or off, keeping the count of controlled managers, and refreshes the match day flags'),
         ('set_watched_fixture', 'HUMAN_MANAGER::set_watched_fixture', '0x5fe500', 'human_manager.cpp', 'thiscall, ret 8',
          'guarda (ou apaga) a partida que o técnico vai assistir; recusa uma que ele não pode assistir e a troca de uma partida do dia de jogo em curso',
          'stores (or clears) the fixture the manager will watch; refuses one he cannot watch and a change away from a fixture of the live match day'),
         ('human_manager_set_holiday_option', 'HUMAN_MANAGER::set_holiday_option', '0x5feb60', 'human_manager.cpp', 'thiscall, ret 0xc',
          'muda uma opção de férias do técnico; a opção 3 ligada guarda a escalação atual do clube: os 16 jogadores escolhidos e, para os 11 titulares, a posição',
          'sets one of the manager\'s holiday options; option 3 turned on saves the club\'s current selection: the 16 chosen players and, for the eleven starters, their position')])
