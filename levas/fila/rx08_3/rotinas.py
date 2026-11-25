# O destrutor das estatisticas dos jogadores e a carga do player stats.dat (player_stats.cpp; leitor rx08, rodada 6,
# 25/11/2026). Inteiras (0x7c0d00..0x7c0d67, ret; 0x7c48b0..0x7c5594, ret 4; o resto ate a proxima fronteira e
# enchimento; sem tabela de saltos). O construtor 0x7bfd50 ja e Rust (construct_player_stats) e chama a carga pelo
# exe (exe::playerstats_load_stats): o desvio desta leva pega essa chamada.
# Chamadas: o destrutor pelo free_game_data (antes de uma carga e no fim); a carga so pelo construtor, com o save
# (load_game_data, LoadStep 0x11).
# Convencoes conferidas na desmontagem:
# - 0x7c0d00 thiscall(this) ret 0: FILE_LLIST::~FILE_LLIST 0x58e330 + operator delete 0x944c94 do +0 (zerado); free
#   0x9452ca de +4 (zera +4, +8, +0xc), +0x30 (+0x30, +0x34) e +0x3c (+0x3c, +0x40; a contagem +0x44 fica).
# - 0x7c48b0 thiscall(this, save) ret 4 -> 0/1 (quadro SEH so para os objetos da conversao): zopen 0x9326e0(save,
#   "player stats.dat", 0, 0x16, 4), zread 0x932c30 cdecl, zclose 0x932b00(&z); sem save e sem o item a caixa simples
#   (0xd1a, 0xd22) e 0; as leituras ate +0x38 e os malloc param o jogo (fatal_error_at); as das linguas so avisam
#   (0xda2, 0xda8, 0xdb3), fecham o item e devolvem 0. A versao: |[0xdd7110]| < 0x3e015 (cdq/xor/sub e jge com sinal).
#   A conversao do historico (so nos saves antigos): LLIST::LLIST 0x68d020 local; FILE_LLIST 0x58e280 thiscall(caminho,
#   0x1b) e ("new_history.tmp", 0x2b) ret 8; get_nodes 0x58f120 thiscall(inicio, fim) ret 8; LLIST::get_next 0x68d1b0;
#   LLIST::add_tail 0x68d0d0 thiscall(item, 0x2b) ret 8; FILE_LLIST::add_node_list 0x58e9b0 thiscall(&inicio, &fim,
#   lista) ret 0xc; LLIST::clear 0x68d080; LLIST::~LLIST 0x68d040; remove 0x94643a cdecl; a copia de arquivo 0x90cc50
#   cdecl(origem, destino).
# Prologos (fronteira de instrucao, sem relativo; iguais no cm0102-1080.exe): 0x7c0d00 `push ebx; push esi; mov esi,
# ecx; push edi; xor ebx, ebx` (7); 0x7c48b0 `mov eax, fs:[0]` (6).
# O no novo do historico (0x2b) a partir do antigo (0x1b): os bytes 0..0x16 iguais, 0x16..0x26 zero, 0x26..0x2b = os
# antigos 0x16..0x1b. O byte 0x2a sai das duas metades do antigo byte 0x1a (as operacoes xor/and 7 do original dao o
# byte inteiro): sem residuo de pilha.
# Defeitos do original [inferencia]: (1) o malloc das linguas nao e conferido: a rotina testa a capacidade lida em vez
# do bloco (capacidade 0 para o jogo; sem memoria segue com o ponteiro nulo e a leitura cai); (2) as linguas vao pela
# capacidade, nao pela contagem (como na gravacao); (3) o destrutor zera a capacidade das linguas e nao a contagem.
# Depende do rx08_1 (zread_or_stop) e do rx08_2 (player_stats_save_consts): integrar depois deles (INTEGRAR_ANTES.txt).
SAVE = True
CARGA = True  # a carga so roda ao carregar: a prova do save carrega o save de ferias com e sem a leva
ROTINAS = [('drop_player_stats', 0x7c0d00, [0x53, 0x56, 0x8b, 0xf1, 0x57, 0x33, 0xdb]),
           ('player_stats_load', 0x7c48b0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as estatísticas dos jogadores: destruir e carregar do save',
        'the player statistics: destroying and loading from the save',
        [('drop_player_stats', 'PlayerStats::~PlayerStats', '0x7c0d00', 'player_stats.cpp', 'thiscall',
          'destrói a lista do histórico de carreira e libera os registros por categoria, as entradas dos jogadores e as línguas, zerando os ponteiros e as contagens (a contagem das línguas fica)',
          "destroys the career history list and frees the per-category records, the player entries and the languages, zeroing the pointers and counts (the language count stays)"),
         ('player_stats_load', 'PlayerStats::load_stats', '0x7c48b0', 'player_stats.cpp', 'thiscall, ret 4',
          'lê do item "player stats.dat" do save os registros por categoria (nos saves antigos de 0x10 bytes, completados com zeros), as entradas dos jogadores, as quatro somas das notas, a contagem de registros livres e as línguas; num save antigo reescreve o arquivo do histórico de carreira com os nós novos de 0x2b bytes; leitura curta das línguas só avisa e devolve 0, as outras param o jogo',
          'reads from the save\'s "player stats.dat" item the per-category records (0x10 bytes in old saves, filled with zeros), the player entries, the four rating sums, the free record count and the languages; in an old save it rewrites the career history file with the new 0x2b-byte nodes; a short read of the languages only warns and answers 0, the others stop the game')])
CAMPOS = [('PlayerStats +0x34 número de entradas dos jogadores (0x27 bytes, PLAYER_ENTRIES); valor; grava (player_stats_load) e zera (drop_player_stats)',
           'PlayerStats +0x34 number of player entries (0x27 bytes, PLAYER_ENTRIES); value; writes (player_stats_load) and zeroes (drop_player_stats)')]
INTERESSA = ['SGE', 'CMRTE']
