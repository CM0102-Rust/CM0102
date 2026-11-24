# A lista estatica dos registros de clube observado e o destrutor do gerente de olheiros (scout_manager.cpp; leitor
# sx19b, rodada 5, 24/11/2026): scout_manager_static_init_list_b74438 (0x7f6730..0x7f6745, ret) e
# SCOUT_MANAGER::~SCOUT_MANAGER (0x7f6aa0..0x7f6ae9, ret).
# Chamadas: o inicializador pelo _initterm do CRT (sem referencia direta: entra na tabela de inicializadores; roda
# depois do DllMain, com os desvios ja postos, como os do sx11_2); o destrutor pela limpeza do jogo (0x83030a, antes do
# operator delete do [0xb74448]).
# Fora da lista: 0x7f6750 scout_manager_static_destroy_list_b74438 (mov ecx, 0xb74438; jmp 0x68d040: um thunk de um
# jmp, sem desvio).
# Convencoes conferidas na desmontagem:
# - 0x7f6730 cdecl(): 0x68d020 LLIST::LLIST thiscall(0xb74438); 0x94627a atexit cdecl(0x7f6750);
# - 0x7f6aa0 thiscall(gerente): 0x9462bb vector_destructor_iterator(buscas, 0x2b0, [buscas-4], 0x7ae4f0), 0x944c94
#   operator delete(buscas-4), [+0] = 0; 0x9452ca free(+0x5c), [+0x5c] = 0.
# Simulacao (sem LIGAS): o inicializador roda na partida do programa; o destrutor so na limpeza do jogo. Sem mutacao
# (estruturais: a lista nasce vazia e o destrutor so libera).
SAVE = False
ROTINAS = [('scout_manager_static_init_list', 0x7f6730, [0xb9, 0x38, 0x44, 0xb7, 0x00]),
           ('scout_manager_destructor', 0x7f6aa0, [0x56, 0x8b, 0xf1, 0x8b, 0x06])]
DOCS = ('a lista estática dos clubes observados e o destrutor do gerente de olheiros',
        'the static scouted club list and the scout manager destructor',
        [('scout_manager_static_init_list', 'scout_manager_static_init_list_b74438', '0x7f6730', 'scout_manager.cpp', 'cdecl',
          'inicializador estático do exe (antes do WinMain): monta vazia a lista global dos registros de clube observado (0xb74438) e registra o destrutor dela no atexit',
          'static initialiser of the exe (before WinMain): builds the global list of scouted club records (0xb74438) empty and registers its destructor with atexit'),
         ('scout_manager_destructor', 'SCOUT_MANAGER::~SCOUT_MANAGER', '0x7f6aa0', 'scout_manager.cpp', 'thiscall',
          'desfaz o gerente de olheiros: as buscas de jogadores pelo destrutor delas, o bloco liberado e zerado, e o vetor de pedidos de observação liberado e zerado',
          'destroys the scout manager: the player searches through their destructor, the block deleted and zeroed, and the scout request array freed and zeroed')])
CAMPOS = []
INTERESSA = []
