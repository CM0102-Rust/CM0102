# Os inicializadores e destrutores estaticos e os destrutores dos tecnicos humanos, das lesoes e das nacoes-chave
# (hall_of_fame.cpp, human_manager.cpp, injury.cpp, key_nation.cpp; leitor sx12b, rodada 5, 23/11/2026).
# Quando rodam:
# - 0x6670b0 e 0x5efd00: no _initterm do exe, a cada inicio do programa (os desvios ja estao: a DLL instala no DllMain);
#   0x6670e0 e 0x5efd30: no atexit, ao sair;
# - 0x62a8a0, 0x5fb730 e 0x669290: no free_game_data 0x82ff40 (ja Rust, chama pelo endereco), quando ha um jogo
#   carregado: carregar um jogo salvo com outro aberto, voltar ao menu principal; 0x5fb700 pelo iterador de vetor do
#   0x5fb730 (e no desenrolar de excecao do construtor e da carga).
# Caminho no jogo para o Lucas: abrir o jogo (os estaticos), carregar um jogo salvo, depois carregar outro pelo menu
# Arquivo (free_game_data destroi o primeiro), sair do jogo.
# Convencoes conferidas na desmontagem:
# - 0x6670b0/0x5efd00: cdecl(void); o iterador de construcao 0x9463af stdcall(tabela, tamanho, quantidade, construtor,
#   destrutor 0x5a53c0 = ret) e atexit 0x94627a cdecl; 0x6670e0/0x5efd30: o iterador de destruicao 0x9462bb
#   stdcall(tabela, tamanho, quantidade, 0x5a53c0). Prologo push imm32 (5 bytes).
# - 0x62a8a0: thiscall, ret 0; free 0x9452ca; FILE_LLIST::~FILE_LLIST 0x58e330 e LLIST::~LLIST 0x68d040 thiscall,
#   operator delete 0x944c94. Nao zera nada (o free_game_data apaga o objeto e zera o global).
# - 0x5fb730: thiscall, ret 0; TRANSFER_OFFER::~TRANSFER_OFFER 0x8e5710 thiscall; o iterador 0x9462bb(registros, 0xa6ef,
#   [registros-4], 0x5fb700) e delete(registros-4); zera [this] e +4 so quando havia tabela.
# - 0x5fb700: thiscall, ret 0; o iterador(this, 0x1ecd, 5, 0x5a53c0).
# - 0x669290: cdecl(void); para cada linha (0xb63d6c + 0x48 k: a contagem +0xc, o vetor +0x10), o destrutor virtual
#   (vtable[0] thiscall(comp, 1)) de cada competicao e free do vetor.
# Comparadores: so a original em todos.
TELAS = True
SAVE = False
ROTINAS = [('key_nation_static_init', 0x6670b0, [0x68, 0xc0, 0x53, 0x5a, 0x00]),
           ('key_nation_static_destroy', 0x6670e0, [0x68, 0xc0, 0x53, 0x5a, 0x00]),
           ('hall_of_fame_import_buffer_init', 0x5efd00, [0x68, 0xc0, 0x53, 0x5a, 0x00]),
           ('hall_of_fame_import_buffer_destroy', 0x5efd30, [0x68, 0xc0, 0x53, 0x5a, 0x00]),
           ('injury_destructor', 0x62a8a0, [0x56, 0x8b, 0xf1, 0x8b, 0x06]),
           ('human_manager_destructor', 0x5fb730, [0x56, 0x8b, 0xf1, 0x83, 0x3e, 0x00]),
           ('human_record_destructor', 0x5fb700, [0x68, 0xc0, 0x53, 0x5a, 0x00]),
           ('key_nation_delete_all_comps', 0x669290, [0x56, 0x57, 0xbe, 0x6c, 0x3d, 0xb6, 0x00])]
DOCS = ('as tabelas estáticas e os destrutores dos técnicos humanos, das lesões e das nações-chave',
        'the static tables and the destructors of the human managers, injuries and key nations',
        [('key_nation_static_init', 'key_nation_static_init', '0x6670b0', 'key_nation.cpp', 'cdecl',
          'no início do programa, monta as 35 linhas estáticas das nações-chave e registra o destrutor delas para a saída',
          'at program start, builds the 35 static key nation rows and registers their destructor for the exit'),
         ('key_nation_static_destroy', 'key_nation_static_destroy', '0x6670e0', 'key_nation.cpp', 'cdecl',
          'na saída do programa, destrói as 35 linhas das nações-chave (destrutor vazio)',
          'at program exit, destroys the 35 key nation rows (empty destructor)'),
         ('hall_of_fame_import_buffer_init', 'hall_of_fame_import_buffer_init', '0x5efd00', 'hall_of_fame.cpp', 'cdecl',
          'no início do programa, monta as 2800 entradas do buffer de importação do hall da fama e registra o destrutor delas para a saída',
          'at program start, builds the 2800 entries of the hall of fame import buffer and registers their destructor for the exit'),
         ('hall_of_fame_import_buffer_destroy', 'hall_of_fame_import_buffer_destroy', '0x5efd30', 'hall_of_fame.cpp', 'cdecl',
          'na saída do programa, destrói as 2800 entradas do buffer de importação (destrutor vazio)',
          'at program exit, destroys the 2800 entries of the import buffer (empty destructor)'),
         ('injury_destructor', 'Injury::~Injury', '0x62a8a0', 'injury.cpp', 'thiscall',
          'libera os registros físicos, as lesões, a lista do histórico, a fila de notícias de recuperação, a lista das pancadas do dia e os surtos',
          'frees the physical records, the injuries, the history list, the recovery news queue, the list of the day\'s knocks and the outbreaks'),
         ('human_manager_destructor', 'HUMAN_MANAGER::~HUMAN_MANAGER', '0x5fb730', 'human_manager.cpp', 'thiscall',
          'com a tabela, apaga as duas propostas pendentes e libera os apelidos de cada técnico humano, destrói os registros e zera o ponteiro e a contagem',
          'with the table, deletes each human manager\'s two pending offers and frees his nicknames, destroys the records and zeroes the pointer and the count'),
         ('human_record_destructor', 'HUMAN_RECORD::~HUMAN_RECORD', '0x5fb700', 'human_manager.cpp', 'thiscall',
          'destrói os cinco blocos de tática de um registro (destrutor vazio)',
          'destroys the five tactic blocks of a record (empty destructor)'),
         ('key_nation_delete_all_comps', 'key_nation_delete_all_comps', '0x669290', 'key_nation.cpp', 'cdecl',
          'apaga as competições de cada linha das nações-chave (destrutor virtual), libera o vetor e zera o vetor e a contagem',
          'deletes the competitions of each key nation row (virtual destructor), frees the array and zeroes the array and the count')])
