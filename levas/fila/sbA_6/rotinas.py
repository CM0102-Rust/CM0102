# As listas ligadas guardadas num arquivo: esquecer o ultimo no e ler uma cadeia (file_llist.cpp, sem assert que
# nomeie o arquivo; o mapa as poe no fifa_rankings.cpp; leitor sbA, rodada 5, 24/11/2026). Carreira ENG (sem LIGAS).
# - 0x58f110 FILE_LLIST::reset_last_offset thiscall(&primeiro, &ultimo) ret 8: -1 no ultimo. Corpo de 13 bytes (o
#   prologo mov eax, [esp+8]; mov dword [eax], -1 tem 10 bytes, sem relativa).
# - 0x58f120 FILE_LLIST::get_nodes thiscall(primeiro, ultimo) ret 8 -> LLIST*: le os historicos (telas, premios,
#   disciplina). malloc 0x944e46 do registro, operator new(0xe) + LLIST::LLIST 0x68d020, fopen "rb", fseek, fread,
#   LLIST::add_tail 0x68d0d0 (thiscall(dado, tamanho em palavra)), fclose, free 0x9452ca; nas falhas LLIST::~LLIST
#   0x68d040 e operator delete 0x944c94. Caixas: "Memory Allocation Error" (0x9a8108/0x9a7ff8, 0x21f) e "Disk I/O
#   Error" (0x22a, 0x236, 0x250).
# Prologos: 0x58f110 10 bytes; 0x58f120 mov eax, fs:[0] (6 bytes). Sem relativas. Iguais no cm0102-1080.exe.
# Defeito do original: a leitura do deslocamento do proximo nao e conferida; falhando, o laco segue com o lixo do local
# (a copia le -1 e para).
# Comparadores: reset_last_offset lado a lado (a copia num local); get_nodes as duas leem o arquivo e as listas tem de
# ter os mesmos registros na mesma ordem (a da copia e destruida depois).
# Depende do sbA_5 (file_llist_fail e FILE_LLIST_DISK_IO): integrar o sbA_5 antes.
SAVE = False
ROTINAS = [('file_llist_reset_last_offset', 0x58f110, [0x8b, 0x44, 0x24, 0x08, 0xc7, 0x00, 0xff, 0xff, 0xff, 0xff]),
           ('file_llist_get_nodes', 0x58f120, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as listas ligadas guardadas num arquivo: esquecer o último nó e ler uma cadeia',
        'the linked lists kept in a file: forgetting the last node and reading a chain',
        [('file_llist_reset_last_offset', 'FILE_LLIST::reset_last_offset', '0x58f110', 'file_llist.cpp', 'thiscall, ret 8',
          'marca a lista do arquivo como sem último nó (−1 no deslocamento do último)',
          'marks the file list as having no last node (−1 in the last offset)'),
         ('file_llist_get_nodes', 'FILE_LLIST::get_nodes', '0x58f120', 'file_llist.cpp', 'thiscall, ret 8',
          'lê do arquivo, a partir do primeiro deslocamento, os registros encadeados até o −1 ou até o registro do último deslocamento, numa lista nova em memória; nas falhas, a caixa e nenhuma lista',
          'reads from the file, from the first offset, the chained records until −1 or the record at the last offset, into a new in-memory list; on failure, the box and no list')])
CAMPOS = []
INTERESSA = []
