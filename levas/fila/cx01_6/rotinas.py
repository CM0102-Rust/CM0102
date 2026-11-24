# A busca de arquivos do jogo (comp_util.cpp; leitor cx01, rodada 5, 24/11/2026). Item do SAVE (CARGA): as tres
# cascas da CRT (_findfirst 0x9467ce, _findnext 0x946898, _findclose 0x94695d, strncpy 0x9466d0, todas cdecl) usadas
# pela gravacao do save (zadd_matching_files junta os .tmp no .qst), pela limpeza da pasta temporaria
# (utils_clear_temp_folder), pela carga do banco (load_database) e pelas verificacoes do CD (43 chamadores).
# - 0x4c0aa0 file_find_first cdecl(padrao, estado, tipo) -> 0/-1: o tipo em +0x110, o identificador em +0; pula ate uma
#   entrada do tipo (0x10 pastas, 0 arquivos; outro tipo nunca acha) e guarda tamanho +4, atributos +8 e nome +0x10.
# - 0x4c0b50 file_find_next cdecl(estado) -> 0/-1: -1 com a busca fechada; antes do _findnext preenche o registro com o
#   que esta guardado (o _findnext o sobrescreve).
# - 0x4c0c10 file_find_close cdecl(estado) -> 0/-1: _findclose e o identificador -1.
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: a primeira lado a lado (a copia num estado local, fechado depois; os identificadores diferem); a
# seguinte e o fechamento so a original (andam e fecham a busca aberta).
SAVE = True
CARGA = True
ROTINAS = [('file_find_first', 0x4c0aa0, [0x8b, 0x54, 0x24, 0x04, 0x8b, 0x44, 0x24, 0x0c]),
           ('file_find_next', 0x4c0b50, [0x81, 0xec, 0x18, 0x01, 0x00, 0x00]),
           ('file_find_close', 0x4c0c10, [0x56, 0x8b, 0x74, 0x24, 0x08])]
DOCS = ('a busca de arquivos do jogo',
        "the game's file search",
        [('file_find_first', 'file_find_first', '0x4c0aa0', 'comp_util.cpp', 'cdecl',
          'começa a busca do padrão querendo pastas (0x10) ou arquivos (0): guarda o tipo e o identificador e pula até a primeira entrada do tipo, guardando o tamanho, os atributos e o nome (255 caracteres); 0, ou −1 sem nenhuma',
          'starts the search for the pattern wanting directories (0x10) or files (0): stores the kind and the handle and skips to the first entry of that kind, storing the size, the attributes and the name (255 characters); 0, or −1 with none'),
         ('file_find_next', 'file_find_next', '0x4c0b50', 'comp_util.cpp', 'cdecl',
          'anda a busca aberta até a próxima entrada do tipo, guardando-a; −1 com a busca fechada ou sem mais entradas',
          'steps the open search to the next entry of the kind, storing it; −1 with the search closed or no more entries'),
         ('file_find_close', 'file_find_close', '0x4c0c10', 'comp_util.cpp', 'cdecl',
          'fecha a busca aberta (identificador −1) e devolve 0; já fechada, −1',
          'closes the open search (handle −1) and returns 0; already closed, −1')])
CAMPOS = [('estado da busca: +0 identificador, +4 tamanho, +8 atributos, +0x110 tipo querido', 'search state: +0 handle, +4 size, +8 attributes, +0x110 kind wanted')]
INTERESSA = []
