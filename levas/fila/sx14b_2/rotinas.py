# O objeto da diretoria e a carga do man_conf.dat (MANAGER_MANAGER::MANAGER_MANAGER 0x68dbf0, ~MANAGER_MANAGER
# 0x68e0e0, load_man_conf 0x68e180, manager_manager.cpp; leitor sx14b, rodada 5, 23-24/11/2026). As tres rotinas
# inteiras (0x68dbf0..0x68e0d3, ret 4; 0x68e0e0..0x68e17e, ret; 0x68e180..0x68e917, ret 4; o resto ate a proxima
# fronteira e enchimento).
# Chamadas: o construtor pela criacao do jogo (0x829be2, sem save; 1 vez na carreira) e pela carga (0x82c948, com o
# save); o destrutor pela limpeza do jogo (0x82fff4); a carga so pelo construtor (0x68de89).
# Convencoes conferidas na desmontagem:
# - 0x68dbf0 thiscall(this, save) ret 4, devolve this; a procura do CD (GetLogicalDriveStringsA, GetDriveTypeA,
#   sprintf 0x987e00, 0x4c0aa0 / 0x4c0c10, a caixa "Insert CD" 0x5e8290 com 1): no cm0102-1080 o jne 0x68dc41 virou
#   nop e o test 0x68dc85 virou xor (a copia le esses bytes, como as outras checagens portadas); as listas: 0x944cf1
#   operator new(0xe) e 0x68d020, na ordem +8, +4, +0xc, +0x10, +0x14; a tabela: 0x944e46 malloc([0xae235c] * 0x49),
#   falha = caixa fatal (0x944cff, 0x90d130, sprintf "v%s %s.%s %d", 0x5e8290, [0xb67a34] = 0, 0x5cc7d0, 0x5a8f60,
#   exit(-1)) nas linhas 0x69 e 0x9f; 0x68f130 init_club_board_records (Rust, chamada direta); 0x68e180 (Rust, direta).
# - 0x68e0e0 thiscall(this) ret; free(+0) e, para cada lista, 0x68d040 + 0x944c94 operator delete.
# - 0x68e180 thiscall(this, save) ret 4 -> eax; 0x9326e0 zopen(save, "man_conf.dat", 0, 0x16, 4); 0x932c30 zread;
#   0x932b00 zclose(&z) em todo caminho depois de aberto; 0x68d0d0 LLIST::add_tail thiscall(lista, item, tamanho);
#   versoes de [0xdd7110] (valor absoluto): > 0x3e040 registros de 0x49 bytes (senao de 0x41, convertidos: +3 so com
#   os 5 bits baixos, +0x41 = 23/06/1971 por 0x549f70 e 0x417c10); > 0x3e033 as listas +0x10 e +0x14; >= 0x3e035 os
#   itens de 0x19 bytes da +0x14 (senao 0x15, convertidos com 0xff nos 4 novos). As datas locais de 0x54a110 que o
#   original nunca le ficaram de fora.
# Prologos (fronteira de instrucao, sem relativo; iguais no 1080): 0x68dbf0 `mov eax, fs:[0]` (6); 0x68e0e0
# `push ebx; push esi; mov esi, ecx; xor ebx, ebx` (6); 0x68e180 `push -1; push 0x95cc33` (7).
# Defeito do original: na conversao dos registros antigos, os 3 bits altos do +3 ficam com o que o malloc deixou.
# Pilha particular: o construtor e chamado pela create_game (0x828110), que le residuo de pilha (ver sx08b_2): se a
# prova do save sair DIFERENTE sem mutacao, conferir primeiro esse residuo.
SAVE = True
CARGA = True  # a carga so roda ao carregar: a prova do save carrega o save de ferias com e sem a leva
ROTINAS = [('new_manager_manager', 0x68dbf0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('drop_manager_manager', 0x68e0e0, [0x53, 0x56, 0x8b, 0xf1, 0x33, 0xdb]),
           ('load_man_conf', 0x68e180, [0x6a, 0xff, 0x68, 0x33, 0xcc, 0x95, 0x00])]
DOCS = ('o objeto da diretoria e a carga do man_conf.dat',
        'the board object and the loading of man_conf.dat',
        [('new_manager_manager', 'MANAGER_MANAGER::MANAGER_MANAGER', '0x68dbf0', 'manager_manager.cpp', 'thiscall, ret 4',
          'monta o objeto da diretoria: espera o CD do jogo (a caixa "Insert CD" até achar o cm0102.exe num drive de CD), cria as cinco listas (vagas, saídas, +0xc, ultimatos, apelações) e, com um save, lê as listas e os registros da diretoria do man_conf.dat; sem save, ou se a leitura falhar (tudo o que ela montou é liberado e as listas são refeitas depois da tabela), a tabela de todos os clubes (sem memória, a caixa fatal nas linhas 0x69 ou 0x9f) preenchida do zero (init_club_board_records). Devolve o objeto',
          'builds the board object: waits for the game CD (the "Insert CD" box until cm0102.exe is found on a CD drive), creates the five lists (vacancies, departures, +0xc, ultimatums, appeals) and, with a save, reads the lists and the board records from man_conf.dat; without a save, or when the reading fails (all it built is freed and the lists are made again after the table), the table of every club (out of memory, the fatal box at lines 0x69 or 0x9f) filled fresh (init_club_board_records). Returns the object'),
         ('drop_manager_manager', 'MANAGER_MANAGER::~MANAGER_MANAGER', '0x68e0e0', 'manager_manager.cpp', 'thiscall',
          'libera a tabela dos registros da diretoria e as cinco listas (destrutor e operator delete), zerando cada ponteiro',
          'frees the board record table and the five lists (destructor and operator delete), zeroing each pointer'),
         ('load_man_conf', 'MANAGER_MANAGER::load_man_conf', '0x68e180', 'manager_manager.cpp', 'thiscall, ret 4',
          'lê o man_conf.dat do save: sem save ou sem o item, 0; a contagem de clubes (0 dá 0) e os registros da diretoria de 0x49 bytes (de 0x41 convertidos nas versões antigas: a data do último ultimato vira 23/06/1971), depois as listas (vagas de 0x26 bytes, saídas de 6, +0xc de 0xe e, nas versões novas, ultimatos de 4 e apelações de 0x19, ou de 0x15 convertidas com 0xff nos bytes novos). O item é fechado em todo caminho; 1 se leu tudo, 0 numa leitura curta ou falta de memória (sem caixa: o construtor monta a diretoria do zero). Defeito: na conversão, os 3 bits altos do +3 ficam com o lixo do malloc',
          'reads man_conf.dat from the save: without a save or the item, 0; the club count (0 gives 0) and the 0x49-byte board records (0x41-byte ones converted in old versions: the last ultimatum date becomes 23/06/1971), then the lists (vacancies of 0x26 bytes, departures of 6, +0xc of 0xe and, in new versions, ultimatums of 4 and appeals of 0x19, or 0x15 converted with 0xff in the new bytes). The item is closed on every path; 1 when all was read, 0 on a short read or out of memory (no box: the constructor builds the board fresh). Defect: in the conversion, the 3 high bits of +3 keep the malloc garbage')])
