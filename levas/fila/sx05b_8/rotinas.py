# O estado do AWOL: a construcao e a carga do awol.dat (awol.cpp; leitor sx05b, rodada 5, 23/11/2026). Define
# AWOL_LISTS, AWOL_CPP, AWOL_DAT e awol_new_list, que o sx05b_9 usa.
# Rodam ao carregar um jogo salvo (o construtor com o save, pela carga do jogo 0x82c1d7) e na criacao (0x82919f, sem
# save). Item de carga: SAVE e CARGA (a criacao do jogo tambem passa pelo construtor, sem save).
# Convencoes conferidas na desmontagem:
# - 0x418440 thiscall(save) ret 4: zera +0xc, +0, +4, +8; com save, 0x418670 (al); falhando, free 0x9452ca da tabela
#   e LLIST::~LLIST 0x68d040 + operator delete 0x944c94 so da primeira lista (as outras duas vazam: defeito); calloc
#   0x945501 cdecl([0xae2364], 1); tres listas por operator new 0x944cf1(0xe) e LLIST::LLIST 0x68d020;
# - 0x418670 thiscall(save) ret 4, devolve al: zopen 0x9326e0 cdecl(save, "awol.dat", 0, 0x16, 4); zread 0x932c30
#   cdecl(destino, tamanho, n, arquivo); a conta de pessoas (zero falha), a tabela (malloc 0x944e46, lida como n
#   bytes), as listas de 12 e 13 bytes (LLIST::add_tail 0x68d0d0 thiscall(registro, tamanho) ret 8) e, com |[0xdd7110]|
#   > 0x3e005, a de 17; toda leitura curta fecha (zclose 0x932b00 cdecl(&arquivo)) e devolve 0.
# Comparadores: so a original (aloca e le o save) e o objeto conferido.
SAVE = True
CARGA = True  # a prova do save carrega o FeriasTesteSave com e sem as rotinas (prova_save.py)
ROTINAS = [('awol_construct', 0x418440, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('awol_load', 0x418670, [0x6a, 0xff, 0x68, 0x89, 0x3e, 0x95, 0x00])]
DOCS = ('o estado do AWOL: construção e carga',
        'the AWOL state: building and loading',
        [('awol_construct', 'AWOL::AWOL', '0x418440', 'awol.cpp', 'thiscall, ret 4',
          'monta o estado do AWOL: tudo zerado; com save, carrega o awol.dat e para se ler; sem save ou com a carga falhando (a tabela e a primeira lista liberadas antes), um byte de marcas zerado por pessoa e três listas vazias. Defeito do original: a carga que falha libera só a primeira lista; a segunda e a terceira vazam',
          'builds the AWOL state: everything zeroed; with a save, loads awol.dat and stops when it reads; without a save or when the load fails (the table and the first list freed first), a zeroed flag byte per person and three empty lists. Defect of the original: a failed load frees only the first list; the second and third leak'),
         ('awol_load', 'AWOL::load', '0x418670', 'awol.cpp', 'thiscall, ret 4',
          'lê o awol.dat do save: a conta de pessoas (zero é falha), a tabela de marcas, as ausências (registros de 12 bytes), os treinos perdidos (13) e, em saves de versão acima de 0x3e005, os incidentes de forma (17; nos antigos, a terceira lista vazia); leitura curta fecha o arquivo e devolve 0',
          'reads awol.dat from the save: the staff count (zero is a failure), the flag table, the absences (12-byte records), the missed trainings (13) and, in saves of a version above 0x3e005, the fitness incidents (17; older saves get an empty third list); a short read closes the file and returns 0')])
CAMPOS = []
INTERESSA = ['SGE', 'CMRTE']
