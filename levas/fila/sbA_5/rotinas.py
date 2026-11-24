# As listas ligadas guardadas num arquivo: o construtor e os acrescimos (file_llist.cpp, sem assert que nomeie o
# arquivo; o mapa as poe no fifa_rankings.cpp; leitor sbA, rodada 5, 24/11/2026). Carreira ENG (sem LIGAS).
# - 0x58e280 FILE_LLIST::FILE_LLIST thiscall(caminho, tamanho do registro) ret 8 -> this: _strdup 0x952f5b para +0 e o
#   tamanho em +4; sem memoria, a caixa (file_llist.CPP 0x2c) e o fim do jogo. Roda na criacao (os historicos).
# - 0x58e340 FILE_LLIST::add_node thiscall(&primeiro, &ultimo, registro) ret 0xc: os historicos de pessoas e clubes
#   (disciplina, lesoes, premios...). fopen "ab"/"r+b", fseek, ftell, fwrite, fread, fclose da CRT.
# - 0x58e9b0 FILE_LLIST::add_node_list thiscall(&primeiro, &ultimo, LLIST) ret 0xc: a lista em memoria inteira de uma
#   vez (LLIST::get_next 0x68d1b0, LLIST::remove 0x68d1f0 e a recursao no caso de primeiro sem ultimo).
# Caixas: "v%s %s.%s %d (%s)" com o texto da falha; "Disk I/O Error" (0x9c7970/0x9c78b0) na abertura do "ab" do
# add_node e na leitura do proximo; "Disk Space Error" (0x9c7774/0x9c7680) nas outras (inclusive a abertura do "ab"
# do add_node_list); fecha o arquivo, menos na abertura que falhou e na busca do ultimo.
# Convencoes conferidas: CRT cdecl (fopen 0x945b00, fseek 0x946a22, ftell 0x946aae, fwrite 0x945ce6, fread 0x945bfe,
# fclose 0x945a09), LLIST thiscall. Prologos: mov eax, [esp+4]; sub esp, 0x200 (10 bytes); sub esp, 0x208 (6); sub esp,
# 0x208 (6). Sem relativas. Iguais no cm0102-1080.exe.
# Defeitos do original: a busca do ultimo no (add_node 0xa8, add_node_list 0x1a3) que falha deixa o arquivo aberto; o
# add_node que reaproveita o proximo no nao escreve -1 depois dele (o add_node_list escreve).
# Comparadores: o construtor so a original (copia o caminho), conferindo a copia e o tamanho; os acrescimos so a
# original (o arquivo muda, nao rodam duas vezes), conferindo pela releitura do arquivo que o registro dado (o ultimo da
# lista, no add_node_list) esta no novo ultimo deslocamento.
# Os auxiliares file_llist_fail, FILE_LLIST_DISK_IO, FILE_LLIST_DISK_SPACE e file_llist_read_back ficam aqui; o sbA_6
# os usa (integrar este antes).
SAVE = False
ROTINAS = [('new_file_llist', 0x58e280, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('file_llist_add_node', 0x58e340, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('file_llist_add_node_list', 0x58e9b0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('as listas ligadas guardadas num arquivo: o construtor e os acréscimos',
        'the linked lists kept in a file: building one and appending to it',
        [('new_file_llist', 'FILE_LLIST::FILE_LLIST', '0x58e280', 'file_llist.cpp', 'thiscall, ret 8',
          'monta uma lista guardada num arquivo: copia o caminho e guarda o tamanho do registro; sem memória, o jogo termina',
          'builds a list kept in a file: copies the path and keeps the record size; without memory the game ends'),
         ('file_llist_add_node', 'FILE_LLIST::add_node', '0x58e340', 'file_llist.cpp', 'thiscall, ret 0xc',
          'acrescenta um registro à lista do arquivo (cada registro seguido do deslocamento do próximo, −1 no fim): na lista vazia, no fim do arquivo; com primeiro e sem último, por cima do primeiro; senão, no próximo nó que já exista ou no fim do arquivo, ligado ao último; cada falha mostra a sua caixa',
          "appends a record to the file list (each record followed by the next one's offset, −1 at the end): to an empty list, at the end of the file; with a first and no last, over the first; otherwise into the next node already there or at the end of the file, linked from the last; each failure shows its box"),
         ('file_llist_add_node_list', 'FILE_LLIST::add_node_list', '0x58e9b0', 'file_llist.cpp', 'thiscall, ret 0xc',
          'acrescenta todos os registros de uma lista em memória à lista do arquivo, com o arquivo aberto uma vez só, pelas mesmas regras do acréscimo de um',
          'appends every record of an in-memory list to the file list, with the file open once, by the same rules as appending one')])
CAMPOS = [('FILE_LLIST (8 bytes): +0 o caminho do arquivo (copia), +4 o tamanho do registro; escreve (new_file_llist)',
           'FILE_LLIST (8 bytes): +0 the file path (copy), +4 the record size; writes (new_file_llist)')]
INTERESSA = []
