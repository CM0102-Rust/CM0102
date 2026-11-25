# O objeto dos recordes dos clubes: construtor, destrutor e a carga do club records.dat (club_records.cpp; leitor rx08,
# rodada 6, 25/11/2026). As tres inteiras (0x444520..0x444892, ret 4; 0x4448a0..0x444923, ret; 0x4515a0..0x451fec, ret 4;
# o resto ate a proxima fronteira e enchimento; sem tabela de saltos).
# Chamadas: o construtor pela criacao do jogo (create_game, CreateStep 0xc com 0; 1 vez na carreira, a cobertura viu o
# corpo 0x44454e) e pela carga (load_game_data, LoadStep 0x10 com o save); o destrutor pelo free_game_data (antes de uma
# carga e no fim); a carga so pelo construtor (0x4446f7).
# Convencoes conferidas na desmontagem:
# - 0x444520 thiscall(this, save) ret 4 -> this (quadro SEH so para o operator new): a procura do CD igual a do
#   find_game_cd (GetLogicalDriveStringsA, GetDriveTypeA, sprintf 0x987e00, 0x4c0aa0/0x4c0c10, a caixa "Insert CD"
#   0x5e8290 com 1); o cm0102-1080 (e o portatil) trocam o jne 0x444577 por nop nop e o test 0x4445b9 por xor: sao os
#   unicos bytes diferentes do 1080 (assinatura diferente entre os exes; a copia le esses bytes). operator new(8) +
#   FILE_LLIST::FILE_LLIST 0x58e280 thiscall(caminho, 0x38d) ret 8, o caminho por utils_build_file_path 0x90bde0
#   cdecl("CM3_TEMP", "club records history.tmp") (= club_history_new_file_list, tel42_1); falha: caixa fatal linha 0x50
#   (0x944cff, 0x90d130, sprintf "v%s %s.%s %d", 0x5e8290, [0xb67a34] = 0, 0x5cc7d0, 0x5a8f60, exit(-1) =
#   fatal_error_at); malloc 0x944e46 (contagem x 0x6a), fatal 0x77; o laco das pessoas: database_get_national_team
#   0x53b3d0 cdecl(nacao) (o push 1 antes dele fica para a 0x452710), get_all_time_record 0x452710 thiscall(selecao, 1)
#   ret 8, fill_staff_name_ref 0x7e4920 cdecl(pessoa, registro + 0x2fb); comparacao de palavra com sinal.
# - 0x4448a0 thiscall(this) ret 0: FILE_LLIST::~FILE_LLIST 0x58e330 + operator delete 0x944c94 do +0; free 0x9452ca
#   de +4 (zera +4, +8, +0xc), +0x10 (+0x10, +0x14, +0x18), +0x1c (+0x1c, +0x20) e +0x24 (+0x24, +0x28, +0x2c).
# - 0x4515a0 thiscall(this, save) ret 4 -> 0/1: zopen 0x9326e0(save, "club records.dat", 0, 0x16, 4), zread 0x932c30
#   cdecl(dados, tamanho, n, z), zclose 0x932b00(&z); sem save e sem o item a caixa simples (0x1515, 0x151d) e 0; o
#   resto fatal. A entrada invalida: so a primeira mostra a caixa 0x15a2 (sem sair).
# Prologos (fronteira de instrucao, sem relativo; iguais no 1080 fora os 3 bytes do CD): 0x444520 `mov eax, fs:[0]` (6);
# 0x4448a0 `push ebx; push esi; mov esi, ecx; push edi; xor edi, edi` (7); 0x4515a0 `sub esp, 0x20c` (6).
# Defeitos do original [inferencia]: a conferencia das entradas carregadas compara os indices com as capacidades dos
# vetores, nao com as contagens: um indice entre a contagem e a capacidade passa e aponta para um registro que o save
# nao gravou (o malloc deixa lixo). O registro de 0x38d bytes do arquivo de historico nao e o de 0x38b do vetor.
# Usa find_game_cd (transfer_manager.rs), club_history_new_file_list e club_history_consts (club_history.rs) e
# fatal_error_at/show_error_at, todos ja no HEAD.
SAVE = True
CARGA = True  # a carga so roda ao carregar: a prova do save carrega o save de ferias com e sem a leva
ROTINAS = [('new_club_records', 0x444520, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('drop_club_records', 0x4448a0, [0x53, 0x56, 0x8b, 0xf1, 0x57, 0x33, 0xff]),
           ('club_records_load', 0x4515a0, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00])]
DOCS = ('os recordes dos clubes: montar, destruir e carregar do save',
        "the club records: building, destroying and loading from the save",
        [('new_club_records', 'Club_Records::Club_Records', '0x444520', 'club_records.cpp', 'thiscall, ret 4',
          'monta o objeto dos recordes: espera o CD do jogo (a caixa "Insert CD" até achar; o exe sem CD remenda a busca e a cópia lê os bytes), abre o arquivo temporário do histórico (CM3_TEMP\\club records history.tmp) e, com save, carrega os recordes; sem save, uma entrada vazia por clube e, para cada seleção, o jogador com mais partidas por ela vira o recorde de partidas internacionais (no empate fica o primeiro da tabela de pessoas)',
          'builds the records object: waits for the game CD (the "Insert CD" box until found; a no-CD exe patches the search and the copy reads the bytes), opens the history temp file (CM3_TEMP\\club records history.tmp) and, with a save, loads the records; without one, an empty entry per club and, for each national team, the player with most caps for it becomes the international caps record (on a tie the first in the person table keeps it)'),
         ('drop_club_records', 'Club_Records::~Club_Records', '0x4448a0', 'club_records.cpp', 'thiscall',
          'destrói a lista do histórico e libera os recordes de sempre e da temporada, as entradas dos clubes e a fila das notícias de recorde, zerando ponteiros, capacidades e contagens',
          "destroys the history list and frees the all-time and season records, the clubs' entries and the record news queue, zeroing pointers, capacities and counts"),
         ('club_records_load', 'Club_Records::load', '0x4515a0', 'club_records.cpp', 'thiscall, ret 4',
          'lê do item "club records.dat" do save os recordes de sempre e os da temporada (capacidade, contagem e registros de 0x38b bytes), as entradas dos clubes (0x6a bytes cada; uma entrada com índice fora do lugar avisa uma vez e recomeça vazia) e a fila das notícias de recorde; leitura curta ou falta de memória param o jogo',
          'reads from the save\'s "club records.dat" item the all-time and the season records (capacity, count and 0x38b-byte records), the clubs\' entries (0x6a bytes each; an entry with an index out of place warns once and starts empty) and the record news queue; a short read or no memory stops the game')])
CAMPOS = [('Club_Records +0 lista em arquivo do histórico (FILE_LLIST, registros de 0x38d) e +0x20 o número de entradas dos clubes (a contagem de clubes); ponteiro e valor; grava (new_club_records, club_records_load) e libera (drop_club_records)',
           'Club_Records +0 history file list (FILE_LLIST, 0x38d-byte records) and +0x20 the number of club entries (the club count); pointer and value; writes (new_club_records, club_records_load) and frees (drop_club_records)')]
INTERESSA = ['SGE', 'CMRTE']
