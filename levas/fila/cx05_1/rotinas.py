# Os destrutores das tres selecoes em torneio (Copa Africana, Copa da Asia, Eurocopa), da minicopa e da minliga dos
# amistosos e da competicao dos amistosos (african_nations.cpp, asia_nations.cpp, euro_champ.cpp, mini_cup.cpp,
# mini_league.cpp, friendly.cpp; leitor cx05, rodada 5, 24/11/2026). Item de TELAS: rodam quando o jogo e desmontado
# (carregar outro save com um jogo aberto, ou sair da carreira); a minicopa e a minliga tambem quando um torneio de
# pre-temporada e cancelado (Friendly::cancel_tournament, ja em Rust; 0 chamadas na carreira ENG ate 2002-06-01).
# Caminho no jogo: com uma carreira aberta (qualquer uma: as tres selecoes e os amistosos sempre existem; a minicopa e
# a minliga so com torneios de pre-temporada vivos, julho), carregar outro save, jogando comparando.
# Convencoes conferidas: todos thiscall, ret 0 (chamados pelos destrutores escalares: mov ecx -> call); dentro,
# League::free_table 0x687970 (push 0 -> thiscall, ret 4), o free do CRT 0x9452ca (cdecl), o destrutor escalar de cada
# fase (slot 0 com 1), Comp_Stats::~Comp_Stats 0x49f450 (mov ecx -> thiscall) e o operator delete 0x944c94 (cdecl),
# League::~League 0x682300, Cup::~Cup 0x518690 e Comp::~Comp 0x48c650 (mov ecx -> thiscall, ret 0), LLIST::~LLIST
# 0x68d040 (thiscall). O quadro SEH so cobre o desenrolar de excecao (nao portado). O Friendly::~Friendly termina com o
# vector destructor iterator do CRT sobre as 127 vagas com o destrutor 0x5a53c0, um corpo vazio: a copia nao o chama.
# Prologo push -1; push imm32 (7 bytes) em todos. Iguais no cm0102-1080.exe (as seis inteiras).
# A minliga solta o calendario (+0xba) sem zerar; as selecoes soltam e zeram.
# Depende do cx01_14 (drop_national_cup, o molde dos destrutores das copas): integrar depois dele.
# Comparadores: so a original (liberam memoria).
SAVE = False
TELAS = True
ROTINAS = [('drop_african_nations', 0x401220, [0x6a, 0xff, 0x68, 0xb8, 0x34, 0x95, 0x00]),
           ('drop_asia_nations', 0x40ea40, [0x6a, 0xff, 0x68, 0xc8, 0x38, 0x95, 0x00]),
           ('drop_euro_champ', 0x57aad0, [0x6a, 0xff, 0x68, 0xa8, 0x87, 0x95, 0x00]),
           ('drop_mini_cup', 0x765ea0, [0x6a, 0xff, 0x68, 0xa6, 0xda, 0x95, 0x00]),
           ('drop_mini_league', 0x7668f0, [0x6a, 0xff, 0x68, 0x56, 0xdb, 0x95, 0x00]),
           ('drop_friendly', 0x5c22d0, [0x6a, 0xff, 0x68, 0xae, 0x9d, 0x95, 0x00])]
DOCS = ('os destrutores das seleções em torneio, dos torneios de amistosos e dos amistosos',
        'the destructors of the national-team tournaments, the friendly tournaments and the friendlies',
        [('drop_african_nations', 'AfricanNations::~AfricanNations', '0x401220', 'african_nations.cpp', 'thiscall, ret 0',
          'desmonta a Copa Africana de Nações: libera a tabela, o calendário, cada fase e o vetor das fases, as estatísticas e roda o destrutor da liga',
          'destroys the African Nations Cup: frees the table, the round table, each stage and the stage vector, the statistics and runs the league destructor'),
         ('drop_asia_nations', 'AsiaNations::~AsiaNations', '0x40ea40', 'asia_nations.cpp', 'thiscall, ret 0',
          'desmonta a Copa da Ásia: libera a tabela, o calendário, cada fase e o vetor das fases, as estatísticas e roda o destrutor da liga',
          'destroys the Asian Cup: frees the table, the round table, each stage and the stage vector, the statistics and runs the league destructor'),
         ('drop_euro_champ', 'EuroChamp::~EuroChamp', '0x57aad0', 'euro_champ.cpp', 'thiscall, ret 0',
          'desmonta a Eurocopa: libera a tabela, o calendário, cada fase e o vetor das fases, as estatísticas e roda o destrutor da liga',
          'destroys the European Championship: frees the table, the round table, each stage and the stage vector, the statistics and runs the league destructor'),
         ('drop_mini_cup', 'MiniCup::~MiniCup', '0x765ea0', 'mini_cup.cpp', 'thiscall, ret 0',
          'desmonta uma minicopa de amistosos: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys a friendly mini cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_mini_league', 'MiniLeague::~MiniLeague', '0x7668f0', 'mini_league.cpp', 'thiscall, ret 0',
          'desmonta uma minliga de amistosos: libera a tabela, o calendário (sem zerar o ponteiro) e as estatísticas e roda o destrutor da liga',
          'destroys a friendly mini league: frees the table, the round table (the pointer is not cleared) and the statistics and runs the league destructor'),
         ('drop_friendly', 'Friendly::~Friendly', '0x5c22d0', 'friendly.cpp', 'thiscall, ret 0',
          'desmonta a competição dos amistosos: a competição de cada torneio com sede, os amistosos planejados, as quatro listas (amistosos, pedidos de datas, torneios pendentes, pedidos à espera de resposta), a tabela de datas internacionais, as seleções ordenadas, a lista de cada nação, os pares de nações proibidos e por fim a base da competição',
          'destroys the friendlies competition: each hosted tournament\'s competition, the planned friendlies, the four lists (friendlies, dates requests, pending tournaments, requests awaiting a reply), the international date table, the sorted nations, each nation\'s list, the banned nation pairs and finally the competition base')])
CAMPOS = []
INTERESSA = []
