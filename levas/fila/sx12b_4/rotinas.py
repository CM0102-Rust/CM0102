# A carga das lesoes e dos tecnicos humanos do jogo salvo (injury.cpp, human_manager.cpp; leitor sx12b, rodada 5,
# 23/11/2026). Rodam na carga de um jogo salvo, pelos construtores (Injury::Injury 0x62a680 e
# HUMAN_MANAGER::HUMAN_MANAGER 0x5fb430 com o stream), chamados por load_game_data 0x82a9c0.
# Convencoes conferidas na desmontagem:
# - 0x62ab00: thiscall(this, save), ret 4, eax 1/0; zopen 0x9326e0 cdecl(save, "injury.dat", 0, 0x16, 4); zread
#   0x932c30 cdecl(destino, tamanho, quantidade, z); zclose 0x932b00 cdecl(&z); malloc 0x944e46; nas versoes antigas
#   (|[0xdd7110]| < 0x3e024) Date::add_days 0x54c5b0 thiscall(data, saida, dias) ret 8 (le so a palavra dos dias: o
#   original empilha ecx com a metade alta do ponteiro das lesoes) e Date::days_since 0x54cdb0 thiscall(data,
#   [0xae2c90], [0xae2c94]) ret 8, a palavra do resultado; LLIST: ~LLIST 0x68d040, operator new(0xe), LLIST 0x68d020,
#   add_tail 0x68d0d0 thiscall(lista, item, 6) ret 8. Sem caixa nem zclose: 0x15f (sem save), 0x167 (zopen); os outros
#   erros: caixa, zclose, 0.
# - 0x5fee50: thiscall(this, save), ret 4, eax 1/0; zopen(save, "human_manager.dat", 0, 0x16, 4); operator new(n x
#   0xa6ef + 4) e o iterador 0x9463af stdcall(p+4, 0xa6ef, n, 0x5fb690, 0x5fb700); versoes antigas (< 0x3e021) pelo
#   buffer estatico 0xb5e138 (3 x 0x1e75), Formation::set_player_instruction 0x5b5290 thiscall(slot, j, 0, 8|2) ret 0xc
#   e set_team_instruction 0x5b50d0 thiscall(slot, 0x20000) ret 4; < 0x3e005 a escalacao salva em 16 dwords; < 0x3e001
#   o byte +0x59 das 16 pessoas humanas = 0x14. Os erros dos apelidos (0x892, 0x8a2, 0x8aa) saem sem zclose.
# Comparadores: so a original (alocam e leem o item do save). Prova: CARGA (o FeriasTesteSave carregado com e sem as
# rotinas); as conversoes de versoes antigas nao rodam com saves atuais (so pela leitura da desmontagem).
SAVE = True
CARGA = True
ROTINAS = [('injury_load', 0x62ab00, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('human_manager_load', 0x5fee50, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a carga das lesões e dos técnicos humanos',
        'loading the injuries and the human managers',
        [('injury_load', 'Injury::load', '0x62ab00', 'injury.cpp', 'thiscall, ret 4',
          'lê o injury.dat do jogo salvo: os registros físicos de cada pessoa (31 bytes), as lesões (0x30 bytes; saves antigos guardavam 0x2c e os dias restantes são recalculados pela data da lesão e a data atual), a fila das notícias de recuperação (22 bytes), os surtos de doença (17 bytes) e a lista das pancadas das partidas do dia (6 bytes); cada erro mostra a caixa, fecha o item e devolve 0',
          'reads injury.dat from the saved game: each person\'s physical record (31 bytes), the injuries (0x30 bytes; old saves kept 0x2c and the days left are worked out again from the injury date and today), the recovery news queue (22 bytes), the illness outbreaks (17 bytes) and the list of the day\'s match knocks (6 bytes); each error shows the box, closes the item and returns 0'),
         ('human_manager_load', 'HUMAN_MANAGER::load', '0x5fee50', 'human_manager.cpp', 'thiscall, ret 4',
          'lê o human_manager.dat do jogo salvo: a contagem e os registros dos técnicos humanos (as duas primeiras táticas, as opções, o contrato de rascunho, a senha, a partida observada, as pastas e os arquivos recentes, a visão do elenco, as opções de férias, a escalação guardada e os apelidos), depois as contagens e as três últimas táticas; converte as táticas, a escalação e os técnicos de saves antigos; refaz os ponteiros da partida observada pelos índices',
          'reads human_manager.dat from the saved game: the count and the human manager records (the first two tactics, the options, the draft contract, the password, the watched fixture, the folders and recent files, the squad view, the holiday options, the saved lineup and the nicknames), then the counts and the last three tactics; converts the tactics, the lineup and the managers of old saves; rebuilds the watched fixture\'s pointers from the indices')])
